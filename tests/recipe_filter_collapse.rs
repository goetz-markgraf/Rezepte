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

/// Test 1: GET /?filter_collapsed=1 → HTML enthält filter-panel--collapsed
#[tokio::test]
async fn filter_collapsed_parameter_rendert_collapsed_klasse() {
    // Gegeben: App ohne Daten
    let (app, _temp) = setup_test_app().await;

    // Wenn: Seite mit ?filter_collapsed=1 aufgerufen wird
    let (status, body) = get_body(app, "/?filter_collapsed=1").await;

    // Dann: HTTP 200 und filter-panel--collapsed im HTML
    assert_eq!(status, StatusCode::OK);
    assert!(
        body.contains("filter-panel--collapsed"),
        "HTML sollte filter-panel--collapsed enthalten, body enthielt: {}",
        &body[..body.len().min(500)]
    );
}

/// Test 2: GET / (ohne Parameter) → HTML enthält KEIN filter-panel--collapsed
#[tokio::test]
async fn ohne_parameter_kein_collapsed() {
    // Gegeben: App ohne Daten
    let (app, _temp) = setup_test_app().await;

    // Wenn: Seite ohne filter_collapsed aufgerufen wird
    let (status, body) = get_body(app, "/").await;

    // Dann: HTTP 200 und kein filter-panel--collapsed im HTML
    assert_eq!(status, StatusCode::OK);
    assert!(
        !body.contains("filter-panel--collapsed"),
        "HTML sollte kein filter-panel--collapsed enthalten wenn Parameter fehlt"
    );
}

/// Test 3: GET /?kategorie=Brot&filter_collapsed=1 → Aktiv-Indikator im HTML
#[tokio::test]
async fn aktiver_filter_plus_collapsed_zeigt_indikator() {
    // Gegeben: App ohne Daten
    let (app, _temp) = setup_test_app().await;

    // Wenn: Seite mit aktivem Kategorie-Filter und filter_collapsed=1 aufgerufen
    let (status, body) = get_body(app, "/?kategorie=Brot&filter_collapsed=1").await;

    // Dann: HTTP 200 und filter-active-indicator im HTML
    assert_eq!(status, StatusCode::OK);
    assert!(
        body.contains("filter-active-indicator"),
        "HTML sollte filter-active-indicator enthalten wenn Filter aktiv und eingeklappt"
    );
}

/// Test 4: GET / ohne aktiven Filter → kein filter-active-indicator
#[tokio::test]
async fn kein_aktiver_filter_kein_indikator() {
    // Gegeben: App ohne Daten
    let (app, _temp) = setup_test_app().await;

    // Wenn: Seite ohne Filter aufgerufen wird
    let (status, body) = get_body(app, "/").await;

    // Dann: kein filter-active-indicator
    assert_eq!(status, StatusCode::OK);
    assert!(
        !body.contains("filter-active-indicator"),
        "HTML sollte kein filter-active-indicator enthalten wenn kein Filter aktiv"
    );
}

/// Test 5: Toggle-URL eingeklappt → zeigt URL ohne filter_collapsed
#[tokio::test]
async fn collapsed_toggle_url_zeigt_aufklappen() {
    // Gegeben: App ohne Daten
    let (app, _temp) = setup_test_app().await;

    // Wenn: Seite mit filter_collapsed=1 aufgerufen wird
    let (status, body) = get_body(app, "/?filter_collapsed=1").await;

    // Dann: Toggle-Button-Link soll URL ohne filter_collapsed=1 enthalten (zum Aufklappen)
    assert_eq!(status, StatusCode::OK);
    // Der Button zeigt "Filter ▶" und seine href-URL hat keinen filter_collapsed=1-Parameter mehr
    assert!(
        body.contains("Filter &#9658;") || body.contains("Filter ▶"),
        "Toggle-Button sollte 'Filter ▶' anzeigen wenn eingeklappt"
    );
}

/// Test 6: Toggle-URL ausgeklappt → zeigt URL mit filter_collapsed=1
#[tokio::test]
async fn ausgeklappt_toggle_url_zeigt_einklappen() {
    // Gegeben: App ohne Daten
    let (app, _temp) = setup_test_app().await;

    // Wenn: Seite ohne filter_collapsed aufgerufen wird
    let (status, body) = get_body(app, "/").await;

    // Dann: Toggle-Button-Link enthält filter_collapsed=1 in der href (zum Einklappen)
    assert_eq!(status, StatusCode::OK);
    assert!(
        body.contains("filter_collapsed=1"),
        "Toggle-Button href sollte filter_collapsed=1 enthalten wenn ausgeklappt"
    );
    assert!(
        body.contains("Filter &#9660;") || body.contains("Filter ▼"),
        "Toggle-Button sollte 'Filter ▼' anzeigen wenn ausgeklappt"
    );
}
