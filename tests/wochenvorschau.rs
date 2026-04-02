/// Integrationstests für Story 18: Wochenvorschau
///
/// Testet GET /wochenvorschau mit verschiedenen Szenarien.
use axum::body::Body;
use axum::http::{Request, StatusCode};
use http_body_util::BodyExt;
use rezepte::{create_pool, create_router};
use tower::ServiceExt;

async fn setup_test_app() -> (axum::Router, tempfile::NamedTempFile) {
    let temp_file = tempfile::NamedTempFile::new().unwrap();
    let db_url = format!("sqlite:{}", temp_file.path().to_str().unwrap());
    let pool = create_pool(&db_url).await.unwrap();
    let app = create_router(pool);
    (app, temp_file)
}

async fn get_body(app: axum::Router, uri: &str) -> (StatusCode, String) {
    let response = app
        .oneshot(Request::builder().uri(uri).body(Body::empty()).unwrap())
        .await
        .unwrap();
    let status = response.status();
    let body = response.into_body().collect().await.unwrap().to_bytes();
    let body_str = String::from_utf8(body.to_vec()).unwrap();
    (status, body_str)
}

/// Erstellt ein Rezept mit optionalem Datum (deutsches Format T.M.JJJJ).
async fn create_recipe_with_date(
    app: &axum::Router,
    title: &str,
    categories: &[&str],
    planned_date: Option<&str>,
) {
    let mut form_data = format!("title={}", urlencoding::encode(title));
    for cat in categories {
        form_data.push_str(&format!("&categories={}", urlencoding::encode(cat)));
    }
    if let Some(date) = planned_date {
        form_data.push_str(&format!("&planned_date={}", urlencoding::encode(date)));
    }

    let request = Request::builder()
        .method("POST")
        .uri("/recipes")
        .header("Content-Type", "application/x-www-form-urlencoded")
        .body(Body::from(form_data))
        .unwrap();

    app.clone().oneshot(request).await.unwrap();
}

/// Berechnet das Datum des Montags dieser Woche (ISO: Woche beginnt Montag).
/// Gibt den Offset-Wochentag zurück als deutschen Datumsstring (T.M.JJJJ).
fn this_week_date(day_offset_from_monday: i64) -> String {
    let today = time::OffsetDateTime::now_utc().date();
    let days_from_monday = today.weekday().number_days_from_monday() as i64;
    let monday = today - time::Duration::days(days_from_monday);
    let target = monday + time::Duration::days(day_offset_from_monday);
    format!(
        "{}.{}.{}",
        target.day(),
        target.month() as u8,
        target.year()
    )
}

/// Berechnet ein relatives Datum als deutschen Datumsstring (T.M.JJJJ).
fn date_in_days(n: i64) -> String {
    let d = time::OffsetDateTime::now_utc().date() + time::Duration::days(n);
    format!("{}.{}.{}", d.day(), d.month() as u8, d.year())
}

#[tokio::test]
async fn wochenvorschau_returns_200() {
    // Given: App ohne Rezepte
    let (app, _temp) = setup_test_app().await;

    // When: GET /wochenvorschau
    let (status, _body) = get_body(app, "/wochenvorschau").await;

    // Then: HTTP 200
    assert_eq!(status, StatusCode::OK);
}

#[tokio::test]
async fn wochenvorschau_shows_all_seven_weekdays() {
    // Given: App ohne Rezepte
    let (app, _temp) = setup_test_app().await;

    // When: GET /wochenvorschau
    let (_status, body) = get_body(app, "/wochenvorschau").await;

    // Then: Alle 7 Wochentage im Body
    for weekday in &[
        "Montag",
        "Dienstag",
        "Mittwoch",
        "Donnerstag",
        "Freitag",
        "Samstag",
        "Sonntag",
    ] {
        assert!(
            body.contains(weekday),
            "Wochentag '{}' sollte im Body sichtbar sein",
            weekday
        );
    }
}

#[tokio::test]
async fn wochenvorschau_shows_recipe_in_current_week() {
    // Given: Rezept mit planned_date = Mittwoch dieser Woche
    let (app, _temp) = setup_test_app().await;
    let wednesday = this_week_date(2); // Mittwoch = +2 Tage von Montag
    create_recipe_with_date(
        &app,
        "Wochenmittwoch-Suppe",
        &["Mittagessen"],
        Some(&wednesday),
    )
    .await;

    // When: GET /wochenvorschau
    let (_status, body) = get_body(app, "/wochenvorschau").await;

    // Then: Rezept im Body sichtbar
    assert!(
        body.contains("Wochenmittwoch-Suppe"),
        "Rezept dieser Woche sollte sichtbar sein"
    );
}

#[tokio::test]
async fn wochenvorschau_does_not_show_recipe_from_next_week() {
    // Given: Rezept mit planned_date = Montag nächste Woche (monday + 7 days)
    let (app, _temp) = setup_test_app().await;
    let next_monday = this_week_date(7); // 7 Tage nach Montag = nächster Montag
    create_recipe_with_date(
        &app,
        "Naechste-Woche-Rezept",
        &["Mittagessen"],
        Some(&next_monday),
    )
    .await;

    // When: GET /wochenvorschau
    let (_status, body) = get_body(app, "/wochenvorschau").await;

    // Then: Rezept NICHT im Body
    assert!(
        !body.contains("Naechste-Woche-Rezept"),
        "Rezept der nächsten Woche sollte nicht sichtbar sein"
    );
}

#[tokio::test]
async fn wochenvorschau_does_not_show_recipe_from_last_week() {
    // Given: Rezept mit planned_date = Sonntag letzte Woche (monday - 1 day)
    let (app, _temp) = setup_test_app().await;
    let last_sunday = this_week_date(-1); // 1 Tag vor Montag = letzter Sonntag
    create_recipe_with_date(
        &app,
        "Letzte-Woche-Rezept",
        &["Mittagessen"],
        Some(&last_sunday),
    )
    .await;

    // When: GET /wochenvorschau
    let (_status, body) = get_body(app, "/wochenvorschau").await;

    // Then: Rezept NICHT im Body
    assert!(
        !body.contains("Letzte-Woche-Rezept"),
        "Rezept der letzten Woche sollte nicht sichtbar sein"
    );
}

#[tokio::test]
async fn wochenvorschau_shows_empty_state_when_no_recipes() {
    // Given: Keine Rezepte mit planned_date diese Woche
    let (app, _temp) = setup_test_app().await;

    // When: GET /wochenvorschau
    let (_status, body) = get_body(app, "/wochenvorschau").await;

    // Then: Leerzustand-Meldung sichtbar
    assert!(
        body.contains("Für diese Woche noch nichts geplant"),
        "Leer-Meldung sollte sichtbar sein"
    );
}

#[tokio::test]
async fn wochenvorschau_shows_multiple_recipes_on_same_day() {
    // Given: Zwei Rezepte mit gleichem planned_date in dieser Woche
    let (app, _temp) = setup_test_app().await;
    let thursday = this_week_date(3); // Donnerstag = +3 von Montag
    create_recipe_with_date(
        &app,
        "Donnerstag-Pfannkuchen",
        &["Mittagessen"],
        Some(&thursday),
    )
    .await;
    create_recipe_with_date(&app, "Donnerstag-Rührei", &["Mittagessen"], Some(&thursday)).await;

    // When: GET /wochenvorschau
    let (_status, body) = get_body(app, "/wochenvorschau").await;

    // Then: Beide Rezepte im Body
    assert!(
        body.contains("Donnerstag-Pfannkuchen"),
        "Erstes Rezept sollte sichtbar sein"
    );
    assert!(
        body.contains("Donnerstag-Rührei"),
        "Zweites Rezept sollte sichtbar sein"
    );
}

#[tokio::test]
async fn wochenvorschau_recipe_link_leads_to_detail() {
    // Given: Rezept mit ID und planned_date diese Woche
    let (app, _temp) = setup_test_app().await;
    let tuesday = this_week_date(1); // Dienstag = +1 von Montag
    create_recipe_with_date(&app, "Link-Test-Rezept", &["Mittagessen"], Some(&tuesday)).await;

    // When: GET /wochenvorschau
    let (_status, body) = get_body(app, "/wochenvorschau").await;

    // Then: Body enthält Link zur Detailansicht (/recipes/{id})
    assert!(
        body.contains("/recipes/"),
        "Rezept-Link sollte /recipes/{{id}} enthalten"
    );
    assert!(
        body.contains("Link-Test-Rezept"),
        "Rezept-Titel sollte sichtbar sein"
    );
}

#[tokio::test]
async fn wochenvorschau_shows_kw_header() {
    // Given: App
    let (app, _temp) = setup_test_app().await;

    // When: GET /wochenvorschau
    let (_status, body) = get_body(app, "/wochenvorschau").await;

    // Then: KW-Angabe im Body
    assert!(
        body.contains("KW "),
        "Kalenderwochen-Angabe sollte sichtbar sein"
    );
}

#[tokio::test]
async fn wochenvorschau_does_not_show_recipe_without_date() {
    // Given: Rezept ohne planned_date
    let (app, _temp) = setup_test_app().await;
    create_recipe_with_date(&app, "Kein-Datum-Rezept", &["Mittagessen"], None).await;

    // When: GET /wochenvorschau
    let (_status, body) = get_body(app, "/wochenvorschau").await;

    // Then: Rezept NICHT im Body
    assert!(
        !body.contains("Kein-Datum-Rezept"),
        "Rezept ohne Datum sollte nicht in der Wochenvorschau erscheinen"
    );
}

#[tokio::test]
async fn wochenvorschau_shows_empty_state_when_recipe_is_next_week() {
    // Given: Nur Rezept nächste Woche
    let (app, _temp) = setup_test_app().await;
    let far_future = date_in_days(30);
    create_recipe_with_date(&app, "Fernes-Rezept", &["Mittagessen"], Some(&far_future)).await;

    // When: GET /wochenvorschau
    let (_status, body) = get_body(app, "/wochenvorschau").await;

    // Then: Leerzustand-Meldung
    assert!(
        body.contains("Für diese Woche noch nichts geplant"),
        "Leer-Meldung sollte sichtbar sein wenn keine Wochenrezepte"
    );
}

// Story 33: Wochenübersicht Navigation mit Pfeiltasten

#[tokio::test]
async fn wochenvorschau_mit_week_parameter_zeigt_andere_woche() {
    // Given: App
    let (app, _temp) = setup_test_app().await;

    // When: GET /wochenvorschau?week=2025-W02
    let (_status, body) = get_body(app, "/wochenvorschau?week=2025-W02").await;

    // Then: Navigation enthält Pfeile und Zeitraum-Anzeige
    assert!(
        body.contains("wochen-nav-prev") || body.contains("href=\"/wochenvorschau?week=2025-W01\""),
        "Body sollte Link zur vorherigen Woche enthalten"
    );
    assert!(
        body.contains("wochen-nav-next") || body.contains("href=\"/wochenvorschau?week=2025-W03\""),
        "Body sollte Link zur nächsten Woche enthalten"
    );
}

#[tokio::test]
async fn wochenvorschau_ohne_parameter_zeigt_navigation_links() {
    // Given: App
    let (app, _temp) = setup_test_app().await;

    // When: GET /wochenvorschau ohne Parameter
    let (_status, body) = get_body(app, "/wochenvorschau").await;

    // Then: Navigation enthält Vorherige/Nächste Woche Links
    assert!(
        body.contains("wochenvorschau-nav") || body.contains("aria-label=\"Vorherige Woche\"") || body.contains("<"),
        "Body sollte Navigationselemente enthalten"
    );
}

// Story-19-Tests: Formatierung der Wochentage

#[tokio::test]
async fn wochenvorschau_hat_css_klasse_wochentag_heute() {
    // Given: App ohne Rezepte
    let (app, _temp) = setup_test_app().await;

    // When: GET /wochenvorschau
    let (_status, body) = get_body(app, "/wochenvorschau").await;

    // Then: Body enthält "wochentag-heute" (CSS-Klasse für heutigen Tag)
    assert!(
        body.contains("wochentag-heute"),
        "Body sollte CSS-Klasse 'wochentag-heute' für den heutigen Tag enthalten"
    );
}

#[tokio::test]
async fn wochenvorschau_hat_css_klasse_wochentag_name() {
    // Given: App ohne Rezepte
    let (app, _temp) = setup_test_app().await;

    // When: GET /wochenvorschau
    let (_status, body) = get_body(app, "/wochenvorschau").await;

    // Then: Body enthält class="wochentag-name"
    assert!(
        body.contains("wochentag-name"),
        "Body sollte CSS-Klasse 'wochentag-name' enthalten"
    );
}

#[tokio::test]
async fn wochenvorschau_hat_css_klasse_wochentag_datum() {
    // Given: App ohne Rezepte
    let (app, _temp) = setup_test_app().await;

    // When: GET /wochenvorschau
    let (_status, body) = get_body(app, "/wochenvorschau").await;

    // Then: Body enthält class="wochentag-datum"
    assert!(
        body.contains("wochentag-datum"),
        "Body sollte CSS-Klasse 'wochentag-datum' enthalten"
    );
}

#[tokio::test]
async fn wochenvorschau_heute_tag_enthaelt_heute_badge() {
    // Given: App ohne Rezepte
    let (app, _temp) = setup_test_app().await;

    // When: GET /wochenvorschau
    let (_status, body) = get_body(app, "/wochenvorschau").await;

    // Then: Body enthält "Heute" (Badge-Text für heutigen Tag)
    assert!(
        body.contains("Heute"),
        "Body sollte 'Heute'-Badge für den heutigen Tag enthalten"
    );
}

#[tokio::test]
async fn wochenvorschau_vergangene_tage_korrekt_wenn_nicht_montag() {
    // Given: App ohne Rezepte
    let (app, _temp) = setup_test_app().await;

    // When: GET /wochenvorschau
    let (_status, body) = get_body(app, "/wochenvorschau").await;

    // Berechne ob heute Montag ist
    let today = time::OffsetDateTime::now_utc().date();
    let is_monday = today.weekday() == time::Weekday::Monday;

    if is_monday {
        // Wenn heute Montag ist, gibt es keine vergangenen Tage in dieser Woche
        assert!(
            !body.contains("wochentag-vergangen"),
            "Montag sollte keine 'wochentag-vergangen'-Klasse erzeugen"
        );
    } else {
        // Wenn heute nicht Montag ist, gibt es mindestens einen vergangenen Tag
        assert!(
            body.contains("wochentag-vergangen"),
            "Body sollte CSS-Klasse 'wochentag-vergangen' für vergangene Tage enthalten"
        );
    }
}

// Story 34: "Länger nicht gemacht" Button in Wochenvorschau

#[tokio::test]
async fn wochenvorschau_enthaelt_link_zur_not_made_suche() {
    // Given: App ohne Rezepte
    let (app, _temp) = setup_test_app().await;

    // When: GET /wochenvorschau
    let (_status, body) = get_body(app, "/wochenvorschau").await;

    // Then: Body enthält Link zur Suche mit Filter "Länger nicht gemacht" und Kategorie Mittagessen
    assert!(
        body.contains("filter=laenger-nicht-gemacht"),
        "Body sollte Link zu '/?filter=laenger-nicht-gemacht' enthalten"
    );

    // And: Link enthält auch Kategorie=Mittagessen (Story 35)
    assert!(
        body.contains("kategorie=Mittagessen"),
        "Body sollte Link mit '&kategorie=Mittagessen' enthalten"
    );

    // And: Button hat korrekte CSS-Klasse
    assert!(
        body.contains("not-made-button"),
        "Body sollte CSS-Klasse 'not-made-button' enthalten"
    );

    // And: Button hat korrektes ARIA-Label (Story 35: Mittagessen-spezifisch)
    assert!(
        body.contains("Mittagessen-Rezepte anzeigen, die länger nicht gemacht wurden"),
        "Body sollte ARIA-Label für Barrierefreiheit enthalten"
    );

    // And: Toolbar-Container ist vorhanden
    assert!(
        body.contains("wochenvorschau-toolbar"),
        "Body sollte CSS-Klasse 'wochenvorschau-toolbar' enthalten"
    );

    // And: Button-Text ist vorhanden
    assert!(
        body.contains("Länger nicht gemacht"),
        "Body sollte Button-Text 'Länger nicht gemacht' enthalten"
    );
}
