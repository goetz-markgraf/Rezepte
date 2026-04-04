use crate::error::AppError;
use crate::models::get_recipes_current_week;
use crate::templates::{Wochentag, WochentagesEintragItem, WochenvorschauTemplate};
use askama::Template;
use axum::{
    extract::{Query, State},
    response::Html,
};
use serde::Deserialize;
use sqlx::SqlitePool;
use std::sync::Arc;

/// Query-Parameter für die Wochenvorschau (jetzt ohne week-Parameter)
#[derive(Deserialize)]
pub struct WeekQuery {
    #[allow(dead_code)]
    pub week: Option<String>, // Für Rückwärtskompatibilität, wird ignoriert
}

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

#[allow(dead_code)]
const GERMAN_WEEKDAYS_SHORT: &[&str] = &[
    "Mo", "Di", "Mi", "Do", "Fr", "Sa", "So",
];

/// Gibt den kurzen deutschen Wochentag-Namen zurück: "Mo" bis "So".
fn german_weekday_short(weekday: time::Weekday) -> &'static str {
    GERMAN_WEEKDAYS_SHORT[weekday.number_days_from_monday() as usize]
}

/// Formatiert ein Datum als "Fr, 04.04.2026" (kurzer Wochentag + Datum).
fn format_date_with_short_weekday(date: time::Date) -> String {
    let weekday = german_weekday_short(date.weekday());
    format!(
        "{}, {:02}.{:02}.{}",
        weekday,
        date.day(),
        date.month() as u8,
        date.year()
    )
}

/// Formatiert ein Datum als "T. Monatsname", z.B. "30. März" oder "5. April".
fn format_date_kurz(date: time::Date) -> String {
    let month_name = GERMAN_MONTHS_LONG[(date.month() as u8 - 1) as usize];
    format!("{}. {}", date.day(), month_name)
}

/// Handler für GET /wochenvorschau — zeigt alle Rezepte der nächsten 15 Tage.
/// Die Navigation mit week-Parameter wurde entfernt (Story 38).
pub async fn wochenvorschau_handler(
    Query(_query): Query<WeekQuery>,
    State(pool): State<Arc<SqlitePool>>,
) -> Result<Html<String>, AppError> {
    let today = time::OffsetDateTime::now_utc().date();

    // Zeige immer die nächsten 15 Tage ab heute
    let start_datum = today;
    let end_datum = today + time::Duration::days(14); // +14 = insgesamt 15 Tage

    let recipes = get_recipes_current_week(&pool, start_datum, end_datum).await?;

    // Zeitraum-Anzeige: "04.04.2026 – 18.04.2026"
    let zeitraum_anzeige = format_zeitraum_header(start_datum, end_datum);

    let tage: Vec<Wochentag> = (0..15)
        .map(|i| start_datum + time::Duration::days(i))
        .map(|datum| {
            let rezepte = recipes
                .iter()
                .filter(|r| r.planned_date == Some(datum))
                .map(|r| WochentagesEintragItem {
                    id: r.id,
                    title: r.title.clone(),
                })
                .collect();
            Wochentag {
                wochentag_name: format_date_with_short_weekday(datum),
                datum_kurz: format_date_kurz(datum),
                ist_heute: datum == today,
                ist_vergangen: datum < today,
                rezepte,
            }
        })
        .collect();

    let hat_rezepte = tage.iter().any(|t| !t.rezepte.is_empty());

    let template = WochenvorschauTemplate {
        tage,
        zeitraum_anzeige,
        hat_rezepte,
    };

    let html = template
        .render()
        .map_err(|e| AppError::BadRequest(format!("Template-Fehler: {}", e)))?;
    Ok(Html(html))
}

/// Formatiert den Zeitraum-Header: "04.04.2026 – 18.04.2026".
fn format_zeitraum_header(start: time::Date, end: time::Date) -> String {
    format!(
        "{:02}.{:02}.{} – {:02}.{:02}.{}",
        start.day(),
        start.month() as u8,
        start.year(),
        end.day(),
        end.month() as u8,
        end.year()
    )
}

#[cfg(test)]
mod tests {
    use super::*;
    use time::Month;

    fn make_date(year: i32, month: u8, day: u8) -> time::Date {
        time::Date::from_calendar_date(year, Month::try_from(month).unwrap(), day).unwrap()
    }

    #[test]
    fn format_date_kurz_formats_correctly() {
        // Montag, 30. März 2026 → "30. März"
        let date = make_date(2026, 3, 30);
        assert_eq!(format_date_kurz(date), "30. März");
    }

    #[test]
    fn format_date_kurz_single_digit_day() {
        // Sonntag, 5. April 2026 → "5. April"
        let date = make_date(2026, 4, 5);
        assert_eq!(format_date_kurz(date), "5. April");
    }

    #[test]
    fn format_date_kurz_january() {
        // 1. Januar 2026 → "1. Januar"
        let date = make_date(2026, 1, 1);
        assert_eq!(format_date_kurz(date), "1. Januar");
    }

    #[test]
    fn wochentag_felder_sind_korrekt_befuellt() {
        use crate::templates::{Wochentag, WochentagesEintragItem};
        let tag = Wochentag {
            wochentag_name: "Mi, 01.04.2026".to_string(),
            datum_kurz: "1. April".to_string(),
            ist_heute: true,
            ist_vergangen: false,
            rezepte: vec![WochentagesEintragItem {
                id: 1,
                title: "Test-Rezept".to_string(),
            }],
        };
        assert_eq!(tag.wochentag_name, "Mi, 01.04.2026");
        assert_eq!(tag.datum_kurz, "1. April");
        assert!(tag.ist_heute);
        assert!(!tag.ist_vergangen);
        assert_eq!(tag.rezepte.len(), 1);
    }

    // Tests für Story 38: 15-Tage-Liste

    #[test]
    fn german_weekday_short_returns_correct_names() {
        assert_eq!(german_weekday_short(time::Weekday::Monday), "Mo");
        assert_eq!(german_weekday_short(time::Weekday::Tuesday), "Di");
        assert_eq!(german_weekday_short(time::Weekday::Wednesday), "Mi");
        assert_eq!(german_weekday_short(time::Weekday::Thursday), "Do");
        assert_eq!(german_weekday_short(time::Weekday::Friday), "Fr");
        assert_eq!(german_weekday_short(time::Weekday::Saturday), "Sa");
        assert_eq!(german_weekday_short(time::Weekday::Sunday), "So");
    }

    #[test]
    fn format_date_with_short_weekday_formats_correctly() {
        // Samstag, 4. April 2026 → "Sa, 04.04.2026"
        let date = make_date(2026, 4, 4);
        assert_eq!(format_date_with_short_weekday(date), "Sa, 04.04.2026");
    }

    #[test]
    fn format_date_with_short_weekday_single_digit_month() {
        // Montag, 5. Januar 2026 → "Mo, 05.01.2026"
        let date = make_date(2026, 1, 5);
        assert_eq!(format_date_with_short_weekday(date), "Mo, 05.01.2026");
    }

    #[test]
    fn format_zeitraum_header_formats_correctly() {
        // 4. April 2026 bis 18. April 2026
        let start = make_date(2026, 4, 4);
        let end = make_date(2026, 4, 18);
        assert_eq!(
            format_zeitraum_header(start, end),
            "04.04.2026 – 18.04.2026"
        );
    }

    #[test]
    fn format_zeitraum_header_months_transition() {
        // 30. März 2026 bis 13. April 2026
        let start = make_date(2026, 3, 30);
        let end = make_date(2026, 4, 13);
        assert_eq!(
            format_zeitraum_header(start, end),
            "30.03.2026 – 13.04.2026"
        );
    }
}
