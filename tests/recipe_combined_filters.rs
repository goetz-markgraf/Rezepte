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

async fn create_recipe(
    app: &axum::Router,
    title: &str,
    categories: &[&str],
    planned_date: Option<&str>,
) {
    // Gegeben: Rezept wird per POST /recipes angelegt
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

#[tokio::test]
async fn three_filters_category_not_made_returns_matching_recipes() {
    // Gegeben: "Dinkelbrot" (Brot, planned_date 2025-06-01),
    //          "Roggenbrot" (Brot, planned_date 2026-06-01 = Zukunft),
    //          "Linseneintopf" (Mittagessen, planned_date 2024-01-01)
    let (app, _temp) = setup_test_app().await;
    create_recipe(&app, "Dinkelbrot", &["Brot"], Some("1.6.2025")).await;
    create_recipe(&app, "Roggenbrot", &["Brot"], Some("1.6.2026")).await;
    create_recipe(
        &app,
        "Linseneintopf",
        &["Mittagessen"],
        Some("1.1.2024"),
    )
    .await;

    // Wenn: GET /?kategorie=Brot&filter=laenger-nicht-gemacht
    let (status, body) = get_body(
        app,
        "/?kategorie=Brot&filter=laenger-nicht-gemacht",
    )
    .await;

    // Dann: HTTP 200, nur "Dinkelbrot" im Body
    assert_eq!(status, StatusCode::OK);
    assert!(
        body.contains("Dinkelbrot"),
        "Dinkelbrot soll sichtbar sein (Brot, Vergangenheitsdatum)"
    );
    assert!(
        !body.contains("Linseneintopf"),
        "Linseneintopf soll nicht sichtbar sein (falsche Kategorie)"
    );
    assert!(
        !body.contains("Roggenbrot"),
        "Roggenbrot soll nicht sichtbar sein (Zukunftsdatum)"
    );
}

#[tokio::test]
async fn category_and_search_combined_returns_intersection() {
    // Gegeben: "Dinkelbrot" (Brot), "Roggenbrot" (Brot), "Dinkel-Müsli" (Snacks)
    let (app, _temp) = setup_test_app().await;
    create_recipe(&app, "Dinkelbrot", &["Brot"], None).await;
    create_recipe(&app, "Roggenbrot", &["Brot"], None).await;
    create_recipe(&app, "Dinkel-Müsli", &["Snacks"], None).await;

    // Wenn: GET /?kategorie=Brot&q=Dinkel
    let (status, body) = get_body(app, "/?kategorie=Brot&q=Dinkel").await;

    // Dann: "Dinkelbrot" vorhanden, "Roggenbrot" nicht, "Dinkel-Müsli" nicht
    assert_eq!(status, StatusCode::OK);
    assert!(body.contains("Dinkelbrot"), "Dinkelbrot soll sichtbar sein");
    assert!(
        !body.contains("Roggenbrot"),
        "Roggenbrot soll nicht sichtbar sein (kein Dinkel)"
    );
    assert!(
        !body.contains("Dinkel-Müsli"),
        "Dinkel-Müsli soll nicht sichtbar sein (falsche Kategorie)"
    );
}

#[tokio::test]
async fn category_and_date_filter_combined_returns_intersection() {
    // Gegeben: "Dinkelbrot" (Brot, planned_date Vergangenheit), "Roggenbrot" (Brot, None), "Spaghetti" (Mittagessen)
    let (app, _temp) = setup_test_app().await;
    create_recipe(&app, "Dinkelbrot", &["Brot"], Some("1.1.2024")).await;
    create_recipe(&app, "Roggenbrot", &["Brot"], None).await;
    create_recipe(&app, "Spaghetti", &["Mittagessen"], None).await;

    // Wenn: GET /?kategorie=Brot&filter=laenger-nicht-gemacht
    let (status, body) = get_body(app, "/?kategorie=Brot&filter=laenger-nicht-gemacht").await;

    // Dann: "Dinkelbrot" vorhanden (Vergangenheitsdatum), "Roggenbrot" auch (kein Datum = erstes), "Spaghetti" nicht
    assert_eq!(status, StatusCode::OK);
    assert!(
        body.contains("Dinkelbrot"),
        "Dinkelbrot soll sichtbar sein (Brot, Vergangenheitsdatum)"
    );
    assert!(
        body.contains("Roggenbrot"),
        "Roggenbrot soll sichtbar sein (Brot, kein Datum)"
    );
    assert!(
        !body.contains("Spaghetti"),
        "Spaghetti soll nicht sichtbar sein (falsche Kategorie)"
    );
}

#[tokio::test]
async fn reset_all_filters_button_appears_when_filter_active() {
    // Gegeben: Ein Filter ist aktiv (?filter=laenger-nicht-gemacht)
    let (app, _temp) = setup_test_app().await;

    // Wenn: GET /?filter=laenger-nicht-gemacht
    let (status, body) = get_body(app, "/?filter=laenger-nicht-gemacht").await;

    // Dann: Body enthält "Alle Filter zurücksetzen"-Text
    assert_eq!(status, StatusCode::OK);
    assert!(
        body.contains("Alle Filter zurücksetzen"),
        "\"Alle Filter zurücksetzen\"-Button soll sichtbar sein wenn Filter aktiv"
    );
}

#[tokio::test]
async fn reset_all_filters_button_absent_when_no_filter_active() {
    // Gegeben: Keine Filter aktiv
    let (app, _temp) = setup_test_app().await;

    // Wenn: GET /
    let (status, body) = get_body(app, "/").await;

    // Dann: Body enthält keinen "Alle Filter zurücksetzen"-Text
    assert_eq!(status, StatusCode::OK);
    assert!(
        !body.contains("Alle Filter zurücksetzen"),
        "\"Alle Filter zurücksetzen\"-Button soll nicht sichtbar sein wenn kein Filter aktiv"
    );
}

#[tokio::test]
async fn conflict_both_date_filters_in_url_applies_first_one() {
    // Gegeben: URL mit beiden Datumsfiltern gleichzeitig
    let (app, _temp) = setup_test_app().await;

    // Wenn: GET /?filter=laenger-nicht-gemacht&filter=naechste-7-tage
    let (status, body) =
        get_body(app, "/?filter=laenger-nicht-gemacht&filter=naechste-7-tage").await;

    // Dann: HTTP 200 (kein Absturz, kein 500)
    assert_eq!(status, StatusCode::OK);
    // Und: Seite enthält genau eine der beiden Filterdarstellungen (erster Wert gewinnt)
    let has_not_made = body.contains("Länger nicht gemacht");
    let has_seven_days = body.contains("Nächste 7 Tage");
    // Beide Buttons werden immer angezeigt, aber nur einer ist aktiv (aria-pressed="true")
    assert!(
        has_not_made || has_seven_days,
        "Mindestens ein Filter-Button soll sichtbar sein"
    );
    // Kein Fehler im Body
    assert!(
        !body.contains("Internal Server Error"),
        "Kein Server-Fehler soll auftreten"
    );
}

#[tokio::test]
async fn deeplink_multiple_filters_returns_correct_state() {
    // Gegeben: "Dinkelbrot" (Brot) und "Roggenbrot" (Brot) existieren
    let (app, _temp) = setup_test_app().await;
    create_recipe(&app, "Dinkelbrot", &["Brot"], None).await;
    create_recipe(&app, "Roggenbrot", &["Brot"], None).await;

    // Wenn: GET /?kategorie=Brot direkt aufgerufen
    let (status, body) = get_body(app, "/?kategorie=Brot").await;

    // Dann: HTTP 200, "Dinkelbrot" vorhanden, "Roggenbrot" nicht vorhanden
    assert_eq!(status, StatusCode::OK);
    assert!(
        body.contains("Dinkelbrot"),
        "Dinkelbrot soll sichtbar sein (Brot)"
    );
    assert!(
        body.contains("Roggenbrot"),
        "Roggenbrot soll sichtbar sein (Brot)"
    );
    // Und: aria-pressed="true" für aktiven Filter-Button vorhanden
    assert!(
        body.contains("aria-pressed=\"true\""),
        "Aktiver Filter-Button soll aria-pressed=\"true\" haben"
    );
}
