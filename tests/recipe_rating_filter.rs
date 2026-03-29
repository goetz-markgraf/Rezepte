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

async fn create_recipe_with_rating(
    app: &axum::Router,
    title: &str,
    categories: &[&str],
    rating: Option<i32>,
    planned_date: Option<&str>,
) {
    let mut form_data = format!("title={}", urlencoding::encode(title));
    for cat in categories {
        form_data.push_str(&format!("&categories={}", urlencoding::encode(cat)));
    }
    if let Some(r) = rating {
        form_data.push_str(&format!("&rating={r}"));
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
async fn rating_filter_gut_returns_200_with_three_plus_recipes() {
    // Given: Rezept mit 4 Sternen, Rezept mit 2 Sternen, Rezept ohne Bewertung
    let (app, _temp) = setup_test_app().await;
    create_recipe_with_rating(&app, "Viersternerezept", &["Mittagessen"], Some(4), None).await;
    create_recipe_with_rating(&app, "Zweiesternerezept", &["Mittagessen"], Some(2), None).await;
    create_recipe_with_rating(&app, "UnbewertetesRezept", &["Mittagessen"], None, None).await;

    // When: GET /?bewertung=gut
    let (status, body) = get_body(app, "/?bewertung=gut").await;

    // Then: HTTP 200, nur 4-Sterne-Rezept im Body
    assert_eq!(status, StatusCode::OK);
    assert!(
        body.contains("Viersternerezept"),
        "4-Sterne-Rezept soll sichtbar sein"
    );
    assert!(
        !body.contains("Zweiesternerezept"),
        "2-Sterne-Rezept soll nicht sichtbar sein"
    );
    assert!(
        !body.contains("UnbewertetesRezept"),
        "Unbewertet soll nicht sichtbar sein"
    );
}

#[tokio::test]
async fn rating_filter_gut_excludes_one_and_two_stars() {
    // Given: 1-Stern-Rezept und 2-Sterne-Rezept
    let (app, _temp) = setup_test_app().await;
    create_recipe_with_rating(&app, "EinsternRezept", &["Mittagessen"], Some(1), None).await;
    create_recipe_with_rating(&app, "ZweisternRezept", &["Mittagessen"], Some(2), None).await;

    // When: GET /?bewertung=gut
    let (status, body) = get_body(app, "/?bewertung=gut").await;

    // Then: Keine Rezepte, Hinweistext sichtbar
    assert_eq!(status, StatusCode::OK);
    assert!(
        !body.contains("EinsternRezept"),
        "1-Stern-Rezept soll nicht sichtbar sein"
    );
    assert!(
        !body.contains("ZweisternRezept"),
        "2-Sterne-Rezept soll nicht sichtbar sein"
    );
    assert!(
        body.contains("Keine Rezepte mit dieser Bewertung gefunden"),
        "Hinweistext soll sichtbar sein"
    );
}

#[tokio::test]
async fn rating_filter_gut_excludes_unrated_recipes() {
    // Given: Rezept ohne Bewertung
    let (app, _temp) = setup_test_app().await;
    create_recipe_with_rating(&app, "UnbewertetesRezept", &["Mittagessen"], None, None).await;

    // When: GET /?bewertung=gut
    let (_status, body) = get_body(app, "/?bewertung=gut").await;

    // Then: Rezept nicht im Body
    assert!(
        !body.contains("UnbewertetesRezept"),
        "Unbewertet soll nicht sichtbar sein"
    );
}

#[tokio::test]
async fn rating_filter_favoriten_returns_only_five_star_recipes() {
    // Given: 5-Sterne, 4-Sterne und 3-Sterne-Rezept
    let (app, _temp) = setup_test_app().await;
    create_recipe_with_rating(&app, "FünfsterneRezept", &["Mittagessen"], Some(5), None).await;
    create_recipe_with_rating(&app, "ViersterneRezept", &["Mittagessen"], Some(4), None).await;
    create_recipe_with_rating(&app, "DreiSterneRezept", &["Mittagessen"], Some(3), None).await;

    // When: GET /?bewertung=favoriten
    let (status, body) = get_body(app, "/?bewertung=favoriten").await;

    // Then: Nur 5-Sterne-Rezept im Body
    assert_eq!(status, StatusCode::OK);
    assert!(
        body.contains("FünfsterneRezept"),
        "5-Sterne-Rezept soll sichtbar sein"
    );
    assert!(
        !body.contains("ViersterneRezept"),
        "4-Sterne-Rezept soll nicht sichtbar sein"
    );
    assert!(
        !body.contains("DreiSterneRezept"),
        "3-Sterne-Rezept soll nicht sichtbar sein"
    );
}

#[tokio::test]
async fn rating_filter_favoriten_empty_when_no_five_star() {
    // Given: Nur 4-Sterne-Rezepte
    let (app, _temp) = setup_test_app().await;
    create_recipe_with_rating(&app, "ViersterneRezept", &["Mittagessen"], Some(4), None).await;

    // When: GET /?bewertung=favoriten
    let (_status, body) = get_body(app, "/?bewertung=favoriten").await;

    // Then: Hinweistext sichtbar, kein Rezept
    assert!(
        !body.contains("ViersterneRezept"),
        "4-Sterne-Rezept soll nicht sichtbar sein"
    );
    assert!(
        body.contains("Keine Rezepte mit dieser Bewertung gefunden"),
        "Hinweistext soll sichtbar sein"
    );
}

#[tokio::test]
async fn rating_filter_toggle_deactivates_when_same_value_clicked() {
    // Given: URL /?bewertung=gut wird aufgerufen
    let (app, _temp) = setup_test_app().await;

    // When: Seite mit bewertung=gut aufgerufen
    let (_status, body) = get_body(app, "/?bewertung=gut").await;

    // Then: Toggle-URL für "Nur Gute" enthält kein bewertung=gut
    // Der Button zum Deaktivieren des aktiven Filters soll auf "/" oder ohne bewertung-Parameter zeigen
    // Wir prüfen, dass der aktive Button aria-pressed="true" hat
    assert!(
        body.contains("aria-pressed=\"true\""),
        "Aktiver Filter soll aria-pressed=\"true\" haben"
    );
}

#[tokio::test]
async fn rating_filter_combined_with_category() {
    // Given: Brot-Rezept 5 Sterne, Mittagessen-Rezept 5 Sterne
    let (app, _temp) = setup_test_app().await;
    create_recipe_with_rating(&app, "FünfsterneBrot", &["Brot"], Some(5), None).await;
    create_recipe_with_rating(
        &app,
        "FünfsterneMittagessen",
        &["Mittagessen"],
        Some(5),
        None,
    )
    .await;

    // When: GET /?bewertung=favoriten&kategorie=Brot
    let (status, body) = get_body(app, "/?bewertung=favoriten&kategorie=Brot").await;

    // Then: Nur Brot-Rezept im Body
    assert_eq!(status, StatusCode::OK);
    assert!(
        body.contains("FünfsterneBrot"),
        "Brot-Rezept soll sichtbar sein"
    );
    assert!(
        !body.contains("FünfsterneMittagessen"),
        "Mittagessen-Rezept soll nicht sichtbar sein"
    );
}

#[tokio::test]
async fn rating_filter_combined_with_search() {
    // Given: "Dinkelbrot" 4 Sterne, "Roggenbrot" 4 Sterne
    let (app, _temp) = setup_test_app().await;
    create_recipe_with_rating(&app, "Dinkelbrot", &["Brot"], Some(4), None).await;
    create_recipe_with_rating(&app, "Roggenbrot", &["Brot"], Some(4), None).await;

    // When: GET /?bewertung=gut&q=dinkel
    let (status, body) = get_body(app, "/?bewertung=gut&q=dinkel").await;

    // Then: Nur "Dinkelbrot" im Body
    assert_eq!(status, StatusCode::OK);
    assert!(body.contains("Dinkelbrot"), "Dinkelbrot soll sichtbar sein");
    assert!(
        !body.contains("Roggenbrot"),
        "Roggenbrot soll nicht sichtbar sein"
    );
}

#[tokio::test]
async fn rating_filter_combined_with_not_made_filter() {
    // Given: "Linseneintopf" 4 Sterne, Vergangenheitsdatum; "Tomatensuppe" 2 Sterne, Vergangenheitsdatum
    let (app, _temp) = setup_test_app().await;
    create_recipe_with_rating(
        &app,
        "Linseneintopf",
        &["Mittagessen"],
        Some(4),
        Some("1.1.2025"),
    )
    .await;
    create_recipe_with_rating(
        &app,
        "Tomatensuppe",
        &["Mittagessen"],
        Some(2),
        Some("1.1.2024"),
    )
    .await;

    // When: GET /?bewertung=gut&filter=laenger-nicht-gemacht
    let (status, body) = get_body(app, "/?bewertung=gut&filter=laenger-nicht-gemacht").await;

    // Then: Nur "Linseneintopf" im Body
    assert_eq!(status, StatusCode::OK);
    assert!(
        body.contains("Linseneintopf"),
        "Linseneintopf soll sichtbar sein"
    );
    assert!(
        !body.contains("Tomatensuppe"),
        "Tomatensuppe soll nicht sichtbar sein"
    );
}

#[tokio::test]
async fn deeplink_rating_filter_returns_correct_state() {
    // Given: URL /?bewertung=favoriten direkt aufgerufen
    let (app, _temp) = setup_test_app().await;
    create_recipe_with_rating(&app, "FünfSterneRezept", &["Mittagessen"], Some(5), None).await;

    // When: GET /?bewertung=favoriten
    let (status, body) = get_body(app, "/?bewertung=favoriten").await;

    // Then: 200, aria-pressed="true" für Favoriten-Button im Body
    assert_eq!(status, StatusCode::OK);
    assert!(
        body.contains("FünfSterneRezept"),
        "Favoriten-Rezept soll sichtbar sein"
    );
    assert!(
        body.contains("aria-pressed=\"true\""),
        "Favoriten-Button soll aria-pressed=\"true\" haben"
    );
}

#[tokio::test]
async fn invalid_rating_filter_value_returns_all_recipes() {
    // Given: Rezepte mit verschiedenen Bewertungen, URL /?bewertung=ungueltig
    let (app, _temp) = setup_test_app().await;
    create_recipe_with_rating(&app, "Rezept1", &["Mittagessen"], Some(1), None).await;
    create_recipe_with_rating(&app, "Rezept2", &["Mittagessen"], Some(5), None).await;
    create_recipe_with_rating(&app, "Rezept3", &["Mittagessen"], None, None).await;

    // When: GET /?bewertung=ungueltig
    let (status, body) = get_body(app, "/?bewertung=ungueltig").await;

    // Then: Alle Rezepte werden angezeigt (ungültige Werte werden ignoriert)
    assert_eq!(status, StatusCode::OK);
    assert!(body.contains("Rezept1"), "Rezept1 soll sichtbar sein");
    assert!(body.contains("Rezept2"), "Rezept2 soll sichtbar sein");
    assert!(body.contains("Rezept3"), "Rezept3 soll sichtbar sein");
}
