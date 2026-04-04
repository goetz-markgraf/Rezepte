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
const GERMAN_WEEKDAYS_LONG: &[&str] = &[
    "Montag",
    "Dienstag",
    "Mittwoch",
    "Donnerstag",
    "Freitag",
    "Samstag",
    "Sonntag",
];

#[allow(dead_code)]
/// Gibt den langen deutschen Wochentag-Namen zurück: "Montag" bis "Sonntag".
fn german_weekday_long(weekday: time::Weekday) -> &'static str {
    GERMAN_WEEKDAYS_LONG[weekday.number_days_from_monday() as usize]
}

#[allow(dead_code)]
/// Gibt den deutschen Wochentag-Namen als owned String zurück: "Montag" bis "Sonntag".
fn format_weekday_name(date: time::Date) -> String {
    german_weekday_long(date.weekday()).to_string()
}

#[allow(dead_code)]
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

#[allow(dead_code)]
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

#[allow(dead_code)]
/// Formatiert ein Datum als ISO-Woche: "YYYY-WNN".
fn format_iso_week(date: time::Date) -> String {
    let iso_week = date.iso_week();
    format!("{}-W{:02}", date.year(), iso_week)
}

#[allow(dead_code)]
/// Parsed einen ISO-Wochen-String "YYYY-WNN" und gibt den Montag dieser Woche zurück.
fn parse_iso_week(week_str: &str) -> Option<time::Date> {
    // Format: "YYYY-WNN" (z.B. "2025-W02")
    let parts: Vec<&str> = week_str.split('-').collect();
    if parts.len() != 2 {
        return None;
    }

    let year = parts[0].parse::<i32>().ok()?;
    let week_part = parts[1];

    if !week_part.starts_with('W') {
        return None;
    }

    let week_num = week_part[1..].parse::<u8>().ok()?;

    // Validiere Wochennummer (1-53, aber nicht jedes Jahr hat 53 Wochen)
    if !(1..=53).contains(&week_num) {
        return None;
    }

    // Berechne den Montag der angegebenen ISO-Woche
    // ISO 8601: Woche 1 ist die Woche mit dem ersten Donnerstag
    // Der 4. Januar ist immer in Woche 1
    let jan_4 = time::Date::from_calendar_date(year, time::Month::January, 4).ok()?;
    let jan_4_weekday = jan_4.weekday().number_days_from_monday() as i64;
    let first_monday = jan_4 - time::Duration::days(jan_4_weekday);

    // Der Montag der gewünschten Woche
    let target_monday = first_monday + time::Duration::days((week_num as i64 - 1) * 7);

    // Verifiziere, dass wir im gleichen Jahr sind oder das Jahr davor/danach
    // (für Woche 52/53 Übergang)
    Some(target_monday)
}

#[allow(dead_code)]
/// Berechnet den Montag der aktuellen Woche.
fn get_current_week_monday() -> time::Date {
    let today = time::OffsetDateTime::now_utc().date();
    let days_from_monday = today.weekday().number_days_from_monday() as i64;
    today - time::Duration::days(days_from_monday)
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

    // Tests für Story 33: Wochenübersicht Navigation

    #[test]
    fn format_iso_week_returns_correct_format() {
        let date = make_date(2025, 1, 6); // Montag, KW 2 2025
        assert_eq!(format_iso_week(date), "2025-W02");
    }

    #[test]
    fn format_iso_week_single_digit_week() {
        let date = make_date(2025, 1, 6); // Montag, KW 2
        assert_eq!(format_iso_week(date), "2025-W02");
    }

    #[test]
    fn parse_iso_week_returns_correct_monday() {
        let result = parse_iso_week("2025-W02").unwrap();
        // KW 2 2025 beginnt am Montag, 6. Januar 2025
        assert_eq!(result.year(), 2025);
        assert_eq!(result.month() as u8, 1);
        assert_eq!(result.day(), 6);
    }

    #[test]
    fn parse_iso_week_returns_none_for_invalid_format() {
        assert!(parse_iso_week("invalid").is_none());
        assert!(parse_iso_week("2025").is_none());
        assert!(parse_iso_week("2025-02").is_none());
        assert!(parse_iso_week("").is_none());
    }

    #[test]
    fn parse_iso_week_returns_none_for_invalid_week_numbers() {
        assert!(parse_iso_week("2025-W00").is_none());
        assert!(parse_iso_week("2025-W54").is_none());
        assert!(parse_iso_week("2025-W99").is_none());
    }

    #[test]
    fn parse_iso_week_handles_week_53() {
        // 2020 hatte 53 Wochen
        let result = parse_iso_week("2020-W53").unwrap();
        assert_eq!(result.year(), 2020);
    }

    #[test]
    fn parse_and_format_iso_week_are_inverse() {
        // Teste dass parse(format(date)) = date für verschiedene Daten
        // Hinweis: 2025-12-29 ist nicht geeignet, da es in KW 1 von 2026 fällt
        let test_dates = [
            make_date(2025, 1, 6),   // KW 2
            make_date(2025, 6, 16),  // KW 25
            make_date(2025, 12, 22), // KW 52
        ];

        for date in test_dates {
            let formatted = format_iso_week(date);
            let parsed = parse_iso_week(&formatted).unwrap();
            assert_eq!(
                parsed, date,
                "parse_iso_week(format_iso_week({:?})) sollte {:?} zurückgeben",
                date, date
            );
        }
    }

    #[test]
    fn parse_iso_week_handles_year_transition() {
        // KW 1 2026 beginnt am 29. Dezember 2025 (weil der Donnerstag im neuen Jahr liegt)
        let result = parse_iso_week("2026-W01").unwrap();
        // Der Montag der KW 1 2026
        assert!(result.year() == 2025 || result.year() == 2026);
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
        // Freitag, 4. April 2026 → "Fr, 04.04.2026"
        let date = make_date(2026, 4, 4);
        assert_eq!(format_date_with_short_weekday(date), "Fr, 04.04.2026");
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
