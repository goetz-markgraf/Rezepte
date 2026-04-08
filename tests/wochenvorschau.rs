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
async fn wochenvorschau_shows_all_fifteen_days() {
    // Given: App ohne Rezepte
    let (app, _temp) = setup_test_app().await;

    // When: GET /wochenvorschau
    let (_status, body) = get_body(app, "/wochenvorschau").await;

    // Then: Alle 15 Tage im Body (heute + 14 weitere Tage)
    // Wir prüfen, dass der Zeitraum-Anzeige existiert
    assert!(
        body.contains("–"),
        "Zeitraum-Anzeige sollte im Body sichtbar sein"
    );
}

#[tokio::test]
async fn wochenvorschau_shows_recipe_today() {
    // Given: Rezept mit planned_date = heute
    let (app, _temp) = setup_test_app().await;
    let today = date_in_days(0); // Heute
    create_recipe_with_date(
        &app,
        "Heute-Rezept",
        &["Mittagessen"],
        Some(&today),
    )
    .await;

    // When: GET /wochenvorschau
    let (_status, body) = get_body(app, "/wochenvorschau").await;

    // Then: Rezept im Body sichtbar
    assert!(
        body.contains("Heute-Rezept"),
        "Rezept von heute sollte sichtbar sein"
    );
}

#[tokio::test]
async fn wochenvorschau_shows_recipe_in_15_day_range() {
    // Given: Rezept mit planned_date in 14 Tagen (Tag 15, index 14)
    let (app, _temp) = setup_test_app().await;
    let day_14 = date_in_days(14);
    create_recipe_with_date(
        &app,
        "Tag-14-Rezept",
        &["Mittagessen"],
        Some(&day_14),
    )
    .await;

    // When: GET /wochenvorschau
    let (_status, body) = get_body(app, "/wochenvorschau").await;

    // Then: Rezept im Body sichtbar
    assert!(
        body.contains("Tag-14-Rezept"),
        "Rezept von Tag 14 sollte sichtbar sein"
    );
}

#[tokio::test]
async fn wochenvorschau_does_not_show_recipe_after_15_days() {
    // Given: Rezept mit planned_date = Tag 15 (außerhalb des 15-Tage-Fensters)
    let (app, _temp) = setup_test_app().await;
    let day_15 = date_in_days(15);
    create_recipe_with_date(
        &app,
        "Tag-15-Rezept",
        &["Mittagessen"],
        Some(&day_15),
    )
    .await;

    // When: GET /wochenvorschau
    let (_status, body) = get_body(app, "/wochenvorschau").await;

    // Then: Rezept NICHT im Body
    assert!(
        !body.contains("Tag-15-Rezept"),
        "Rezept außerhalb der 15 Tage sollte nicht sichtbar sein"
    );
}

#[tokio::test]
async fn wochenvorschau_shows_empty_state_when_no_recipes() {
    // Given: Keine Rezepte mit planned_date im 15-Tage-Bereich
    let (app, _temp) = setup_test_app().await;

    // When: GET /wochenvorschau
    let (_status, body) = get_body(app, "/wochenvorschau").await;

    // Then: Leerzustand-Meldung sichtbar
    assert!(
        body.contains("Für die nächsten 15 Tage noch nichts geplant"),
        "Leer-Meldung sollte sichtbar sein"
    );
}

#[tokio::test]
async fn wochenvorschau_shows_multiple_recipes_on_same_day() {
    // Given: Zwei Rezepte mit gleichem planned_date = heute
    let (app, _temp) = setup_test_app().await;
    let today = date_in_days(0); // Heute
    create_recipe_with_date(
        &app,
        "Heute-Pfannkuchen",
        &["Mittagessen"],
        Some(&today),
    )
    .await;
    create_recipe_with_date(&app, "Heute-Rührei", &["Mittagessen"], Some(&today)).await;

    // When: GET /wochenvorschau
    let (_status, body) = get_body(app, "/wochenvorschau").await;

    // Then: Beide Rezepte im Body
    assert!(
        body.contains("Heute-Pfannkuchen"),
        "Erstes Rezept sollte sichtbar sein"
    );
    assert!(
        body.contains("Heute-Rührei"),
        "Zweites Rezept sollte sichtbar sein"
    );
}

#[tokio::test]
async fn wochenvorschau_recipe_link_leads_to_detail() {
    // Given: Rezept mit ID und planned_date = heute
    let (app, _temp) = setup_test_app().await;
    let today = date_in_days(0); // Heute
    create_recipe_with_date(&app, "Link-Test-Rezept", &["Mittagessen"], Some(&today)).await;

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
async fn wochenvorschau_shows_zeitraum_header() {
    // Given: App
    let (app, _temp) = setup_test_app().await;

    // When: GET /wochenvorschau
    let (_status, body) = get_body(app, "/wochenvorschau").await;

    // Then: Zeitraum-Anzeige mit "–" im Body
    assert!(
        body.contains("–"),
        "Zeitraum-Anzeige sollte sichtbar sein"
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
async fn wochenvorschau_shows_empty_state_when_no_recipes_in_range() {
    // Given: Nur Rezept außerhalb des 15-Tage-Bereichs
    let (app, _temp) = setup_test_app().await;
    let far_future = date_in_days(30);
    create_recipe_with_date(&app, "Fernes-Rezept", &["Mittagessen"], Some(&far_future)).await;

    // When: GET /wochenvorschau
    let (_status, body) = get_body(app, "/wochenvorschau").await;

    // Then: Leerzustand-Meldung
    assert!(
        body.contains("Für die nächsten 15 Tage noch nichts geplant"),
        "Leer-Meldung sollte sichtbar sein wenn keine Rezepte im Bereich"
    );
}

// Story 33: Navigation entfällt (Story 38)

#[tokio::test]
async fn wochenvorschau_hat_keine_navigation() {
    // Given: App
    let (app, _temp) = setup_test_app().await;

    // When: GET /wochenvorschau
    let (_status, body) = get_body(app, "/wochenvorschau").await;

    // Then: Keine Navigationselemente vorhanden
    assert!(
        !body.contains("wochen-nav-prev") && !body.contains("wochen-nav-next"),
        "Body sollte keine Navigation enthalten"
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
async fn wochenvorschau_keine_vergangenen_tage() {
    // Given: App ohne Rezepte
    let (app, _temp) = setup_test_app().await;

    // When: GET /wochenvorschau
    let (_status, body) = get_body(app, "/wochenvorschau").await;

    // Then: Es sollte keine vergangenen Tage geben (15-Tage-Liste beginnt ab heute)
    assert!(
        !body.contains("wochentag-vergangen"),
        "15-Tage-Liste sollte keine vergangenen Tage enthalten"
    );
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
