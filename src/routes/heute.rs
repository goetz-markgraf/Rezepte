use crate::error::AppError;
use crate::models::recipe::validate_rating;
use crate::models::{get_recipe_by_id, get_recipes_drei_tage, update_recipe_rating};
use crate::templates::{
    HeuteRezeptItem, HeuteTagesabschnitt, HeuteTemplate, InlineRatingHeuteTemplate,
};
use askama::Template;
use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::{Html, IntoResponse},
};
use sqlx::SqlitePool;
use std::sync::Arc;

const GERMAN_MONTHS_LONG: &[&str] = &[
    "Januar",
    "Februar",
    "März",
    "April",
    "Mai",
    "Juni",
    "Juli",
    "August",
    "September",
    "Oktober",
    "November",
    "Dezember",
];

const GERMAN_WEEKDAYS_LONG: &[&str] = &[
    "Montag",
    "Dienstag",
    "Mittwoch",
    "Donnerstag",
    "Freitag",
    "Samstag",
    "Sonntag",
];

/// Gibt den langen deutschen Wochentag-Namen zurück: "Montag" bis "Sonntag".
fn german_weekday_long(weekday: time::Weekday) -> &'static str {
    GERMAN_WEEKDAYS_LONG[weekday.number_days_from_monday() as usize]
}

/// Gibt den deutschen Wochentag-Namen als owned String zurück.
pub fn format_weekday_name(date: time::Date) -> String {
    german_weekday_long(date.weekday()).to_string()
}

/// Formatiert ein Datum als "T. Monatsname", z.B. "30. März" oder "5. April".
pub fn format_date_kurz(date: time::Date) -> String {
    let month_name = GERMAN_MONTHS_LONG[(date.month() as u8 - 1) as usize];
    format!("{}. {}", date.day(), month_name)
}

/// Formatiert das heutige Datum als vollständige Anzeige: "Donnerstag, 2. April 2026".
pub fn format_heute_anzeige(today: time::Date) -> String {
    let weekday = german_weekday_long(today.weekday());
    let month_name = GERMAN_MONTHS_LONG[(today.month() as u8 - 1) as usize];
    format!(
        "{}, {}. {} {}",
        weekday,
        today.day(),
        month_name,
        today.year()
    )
}

fn render_template<T: Template>(template: T) -> Result<String, AppError> {
    template
        .render()
        .map_err(|e: askama::Error| AppError::BadRequest(e.to_string()))
}

fn parse_rating(raw: Option<&str>) -> Option<i32> {
    raw.filter(|s| !s.trim().is_empty())
        .and_then(|s| s.trim().parse::<i32>().ok())
}

fn parse_form_data(body: &[u8]) -> std::collections::HashMap<String, Vec<String>> {
    let form_data = String::from_utf8_lossy(body);
    let mut params: std::collections::HashMap<String, Vec<String>> =
        std::collections::HashMap::new();

    for pair in form_data.split('&') {
        if let Some((key, value)) = pair.split_once('=') {
            let key = decode_form_value(key);
            let value = decode_form_value(value);
            params.entry(key).or_default().push(value);
        }
    }

    params
}

fn decode_form_value(value: &str) -> String {
    let with_spaces = value.replace('+', " ");
    urlencoding::decode(&with_spaces)
        .unwrap_or_default()
        .to_string()
}

/// Handler für GET /heute — zeigt die Rezepte von gestern, heute und morgen.
pub async fn heute_handler(State(pool): State<Arc<SqlitePool>>) -> Result<Html<String>, AppError> {
    let today = time::OffsetDateTime::now_utc().date();
    let gestern = today - time::Duration::days(1);
    let morgen = today + time::Duration::days(1);

    let recipes = get_recipes_drei_tage(&pool, gestern, morgen).await?;

    let abschnitte = vec![
        HeuteTagesabschnitt {
            label: "Gestern".to_string(),
            wochentag_name: format_weekday_name(gestern),
            datum_kurz: format_date_kurz(gestern),
            ist_heute: false,
            rezepte: recipes
                .iter()
                .filter(|r| r.planned_date == Some(gestern))
                .map(|r| HeuteRezeptItem {
                    id: r.id,
                    title: r.title.clone(),
                    rating: r.rating,
                })
                .collect(),
        },
        HeuteTagesabschnitt {
            label: "Heute".to_string(),
            wochentag_name: format_weekday_name(today),
            datum_kurz: format_date_kurz(today),
            ist_heute: true,
            rezepte: recipes
                .iter()
                .filter(|r| r.planned_date == Some(today))
                .map(|r| HeuteRezeptItem {
                    id: r.id,
                    title: r.title.clone(),
                    rating: r.rating,
                })
                .collect(),
        },
        HeuteTagesabschnitt {
            label: "Morgen".to_string(),
            wochentag_name: format_weekday_name(morgen),
            datum_kurz: format_date_kurz(morgen),
            ist_heute: false,
            rezepte: recipes
                .iter()
                .filter(|r| r.planned_date == Some(morgen))
                .map(|r| HeuteRezeptItem {
                    id: r.id,
                    title: r.title.clone(),
                    rating: r.rating,
                })
                .collect(),
        },
    ];

    let template = HeuteTemplate {
        abschnitte,
        heute_anzeige: format_heute_anzeige(today),
    };

    Ok(Html(render_template(template)?))
}

/// Handler für POST /heute/recipes/:id/rating — speichert Inline-Bewertung und
/// gibt das Rating-Fragment mit kontextueller ID zurück.
pub async fn heute_rating_handler(
    State(pool): State<Arc<SqlitePool>>,
    Path(id): Path<i64>,
    axum::extract::RawForm(body): axum::extract::RawForm,
) -> Result<impl IntoResponse, AppError> {
    let params = parse_form_data(&body);
    let rating_raw = params
        .get("rating")
        .and_then(|v| v.first())
        .map(|s| s.as_str());
    let rating = parse_rating(rating_raw);

    // Validierung: Wert vorhanden und außerhalb 1-5 → 400
    if let Some(err) = validate_rating(rating) {
        return Err(AppError::BadRequest(err));
    }

    // Prüfen, ob Rezept existiert
    get_recipe_by_id(&pool, id)
        .await?
        .ok_or_else(|| AppError::NotFound(format!("Rezept mit ID {} nicht gefunden", id)))?;

    update_recipe_rating(&pool, id, rating)
        .await
        .map_err(|e| match e {
            sqlx::Error::RowNotFound => {
                AppError::NotFound(format!("Rezept mit ID {} nicht gefunden", id))
            }
            other => AppError::Database(other),
        })?;

    let template = InlineRatingHeuteTemplate { id, rating };
    Ok((StatusCode::OK, Html(render_template(template)?)))
}

#[cfg(test)]
mod tests {
    use super::*;
    use time::Month;

    fn make_date(year: i32, month: u8, day: u8) -> time::Date {
        time::Date::from_calendar_date(year, Month::try_from(month).unwrap(), day).unwrap()
    }

    #[test]
    fn format_heute_anzeige_formats_correctly() {
        // Donnerstag, 2. April 2026
        let date = make_date(2026, 4, 2);
        assert_eq!(format_heute_anzeige(date), "Donnerstag, 2. April 2026");
    }

    #[test]
    fn format_heute_anzeige_sunday() {
        // Sonntag, 5. April 2026
        let date = make_date(2026, 4, 5);
        assert_eq!(format_heute_anzeige(date), "Sonntag, 5. April 2026");
    }

    #[test]
    fn format_heute_anzeige_january() {
        // Montag, 1. Januar 2026
        let date = make_date(2026, 1, 5);
        assert_eq!(format_heute_anzeige(date), "Montag, 5. Januar 2026");
    }

    #[test]
    fn format_date_kurz_formats_correctly() {
        let date = make_date(2026, 3, 30);
        assert_eq!(format_date_kurz(date), "30. März");
    }

    #[test]
    fn format_weekday_name_returns_correct_name() {
        assert_eq!(format_weekday_name(make_date(2026, 3, 30)), "Montag");
        assert_eq!(format_weekday_name(make_date(2026, 4, 2)), "Donnerstag");
        assert_eq!(format_weekday_name(make_date(2026, 4, 5)), "Sonntag");
    }
}
