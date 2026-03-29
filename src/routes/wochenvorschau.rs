use crate::error::AppError;
use crate::models::get_recipes_current_week;
use crate::templates::{Wochentag, WochentagesEintragItem, WochenvorschauTemplate};
use askama::Template;
use axum::{extract::State, response::Html};
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

/// Gibt den deutschen Wochentag-Namen als owned String zurück: "Montag" bis "Sonntag".
fn format_weekday_name(date: time::Date) -> String {
    german_weekday_long(date.weekday()).to_string()
}

/// Formatiert ein Datum als "T. Monatsname", z.B. "30. März" oder "5. April".
fn format_date_kurz(date: time::Date) -> String {
    let month_name = GERMAN_MONTHS_LONG[(date.month() as u8 - 1) as usize];
    format!("{}. {}", date.day(), month_name)
}

/// Berechnet die ISO-Kalenderwoche für ein Datum (ISO 8601).
/// Woche beginnt am Montag. KW 1 ist die Woche mit dem ersten Donnerstag.
fn iso_week_number(date: time::Date) -> u8 {
    // Donnerstag der gleichen Woche (0=Mo, 1=Di, 2=Mi, 3=Do, 4=Fr, 5=Sa, 6=So)
    let days_to_thursday = 3i64 - date.weekday().number_days_from_monday() as i64;
    let thursday = date + time::Duration::days(days_to_thursday);

    // 4. Januar ist immer in KW 1 → seinen Montag finden = erster Montag von KW 1
    let jan4 = time::Date::from_calendar_date(thursday.year(), time::Month::January, 4).unwrap();
    // Donnerstag von KW 1 (Donnerstag, der in der Woche des 4. Jan liegt)
    let jan4_days_to_thursday = 3i64 - jan4.weekday().number_days_from_monday() as i64;
    let first_kw1_thursday = jan4 + time::Duration::days(jan4_days_to_thursday);

    // Differenz in vollen Wochen + 1 ergibt die KW
    let diff_days = (thursday - first_kw1_thursday).whole_days();
    (diff_days / 7 + 1) as u8
}

/// Berechnet die KW-Anzeige: "KW 14 · 30. März – 5. April 2026".
fn format_kw_header(monday: time::Date, sunday: time::Date) -> String {
    let kw = iso_week_number(monday);
    let monday_month = GERMAN_MONTHS_LONG[(monday.month() as u8 - 1) as usize];
    let sunday_month = GERMAN_MONTHS_LONG[(sunday.month() as u8 - 1) as usize];

    if monday.month() == sunday.month() {
        // Gleicher Monat: "KW 14 · 30. – 5. März 2026"
        format!(
            "KW {} · {}. – {}. {} {}",
            kw,
            monday.day(),
            sunday.day(),
            sunday_month,
            sunday.year()
        )
    } else {
        // Monatswechsel: "KW 14 · 30. März – 5. April 2026"
        format!(
            "KW {} · {}. {} – {}. {} {}",
            kw,
            monday.day(),
            monday_month,
            sunday.day(),
            sunday_month,
            sunday.year()
        )
    }
}

/// Handler für GET /wochenvorschau — zeigt alle Rezepte der laufenden Woche.
pub async fn wochenvorschau_handler(
    State(pool): State<Arc<SqlitePool>>,
) -> Result<Html<String>, AppError> {
    let today = time::OffsetDateTime::now_utc().date();
    let days_from_monday = today.weekday().number_days_from_monday() as i64;
    let monday = today - time::Duration::days(days_from_monday);
    let sunday = monday + time::Duration::days(6);

    let recipes = get_recipes_current_week(&pool, monday, sunday).await?;

    let kw_anzeige = format_kw_header(monday, sunday);

    let tage: Vec<Wochentag> = (0..7)
        .map(|i| monday + time::Duration::days(i))
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
                wochentag_name: format_weekday_name(datum),
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
        kw_anzeige,
        hat_rezepte,
    };

    let html = template
        .render()
        .map_err(|e| AppError::BadRequest(format!("Template-Fehler: {}", e)))?;
    Ok(Html(html))
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
    fn format_weekday_name_returns_correct_name() {
        assert_eq!(format_weekday_name(make_date(2026, 3, 30)), "Montag");
        assert_eq!(format_weekday_name(make_date(2026, 3, 31)), "Dienstag");
        assert_eq!(format_weekday_name(make_date(2026, 4, 1)), "Mittwoch");
        assert_eq!(format_weekday_name(make_date(2026, 4, 2)), "Donnerstag");
        assert_eq!(format_weekday_name(make_date(2026, 4, 3)), "Freitag");
        assert_eq!(format_weekday_name(make_date(2026, 4, 4)), "Samstag");
        assert_eq!(format_weekday_name(make_date(2026, 4, 5)), "Sonntag");
    }

    #[test]
    fn wochentag_felder_sind_korrekt_befuellt() {
        use crate::templates::{Wochentag, WochentagesEintragItem};
        let tag = Wochentag {
            wochentag_name: "Mittwoch".to_string(),
            datum_kurz: "1. April".to_string(),
            ist_heute: true,
            ist_vergangen: false,
            rezepte: vec![WochentagesEintragItem {
                id: 1,
                title: "Test-Rezept".to_string(),
            }],
        };
        assert_eq!(tag.wochentag_name, "Mittwoch");
        assert_eq!(tag.datum_kurz, "1. April");
        assert!(tag.ist_heute);
        assert!(!tag.ist_vergangen);
        assert_eq!(tag.rezepte.len(), 1);
    }

    #[test]
    fn format_kw_header_same_month() {
        // Woche im gleichen Monat: KW 14, 30. März – 5. April → unterschiedliche Monate
        // Gleicher Monat: z.B. 6.–12. April 2026
        let monday = make_date(2026, 4, 6);
        let sunday = make_date(2026, 4, 12);
        let result = format_kw_header(monday, sunday);
        assert!(result.contains("KW"));
        assert!(result.contains("April"));
        assert!(result.contains("6"));
        assert!(result.contains("12"));
    }

    #[test]
    fn format_kw_header_different_months() {
        // Monatswechsel: 30. März – 5. April
        let monday = make_date(2026, 3, 30);
        let sunday = make_date(2026, 4, 5);
        let result = format_kw_header(monday, sunday);
        assert!(result.contains("März"));
        assert!(result.contains("April"));
        assert!(result.contains("30"));
        assert!(result.contains("5"));
    }

    #[test]
    fn german_weekday_long_returns_correct_names() {
        assert_eq!(german_weekday_long(time::Weekday::Monday), "Montag");
        assert_eq!(german_weekday_long(time::Weekday::Tuesday), "Dienstag");
        assert_eq!(german_weekday_long(time::Weekday::Wednesday), "Mittwoch");
        assert_eq!(german_weekday_long(time::Weekday::Thursday), "Donnerstag");
        assert_eq!(german_weekday_long(time::Weekday::Friday), "Freitag");
        assert_eq!(german_weekday_long(time::Weekday::Saturday), "Samstag");
        assert_eq!(german_weekday_long(time::Weekday::Sunday), "Sonntag");
    }

    #[test]
    fn iso_week_number_kw14_2026() {
        // KW 14 2026 beginnt am 30. März
        let monday = make_date(2026, 3, 30);
        assert_eq!(iso_week_number(monday), 14);
    }
}
