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

async fn create_recipe(app: &axum::Router, title: &str, categories: &[&str]) {
    let mut form_data = format!("title={}", urlencoding::encode(title));
    for cat in categories {
        form_data.push_str(&format!("&categories={}", urlencoding::encode(cat)));
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
async fn filter_by_single_category_shows_matching_recipe() {
    // Given: "Vollkornbrot" (Brot) und "Spaghetti" (Mittagessen)
    let (app, _temp) = setup_test_app().await;
    create_recipe(&app, "Vollkornbrot", &["Brot"]).await;
    create_recipe(&app, "Spaghetti Bolognese", &["Mittagessen"]).await;

    // When: GET /?kategorie=Brot
    let (_status, body) = get_body(app, "/?kategorie=Brot").await;

    // Then: "Vollkornbrot" ist im Body, "Spaghetti" nicht
    assert!(
        body.contains("Vollkornbrot"),
        "Vollkornbrot sollte bei Brot-Filter sichtbar sein"
    );
    assert!(
        !body.contains("Spaghetti Bolognese"),
        "Spaghetti sollte nicht bei Brot-Filter sichtbar sein"
    );
}

#[tokio::test]
async fn filter_by_single_category_hides_non_matching_recipe() {
    // Given: Nur Mittagessen-Rezepte vorhanden
    let (app, _temp) = setup_test_app().await;
    create_recipe(&app, "Lasagne", &["Mittagessen"]).await;

    // When: GET /?kategorie=Brot (kein Brot vorhanden)
    let (_status, body) = get_body(app, "/?kategorie=Brot").await;

    // Then: "Lasagne" nicht sichtbar
    assert!(
        !body.contains("Lasagne"),
        "Lasagne sollte nicht bei Brot-Filter sichtbar sein"
    );
}

#[tokio::test]
async fn filter_by_multiple_categories_shows_all_matching() {
    // Given: Käsekuchen (Kuchen), Partybrot (Brot+Party), Spaghetti (Mittagessen)
    let (app, _temp) = setup_test_app().await;
    create_recipe(&app, "Käsekuchen", &["Kuchen"]).await;
    create_recipe(&app, "Partybrot", &["Brot", "Party"]).await;
    create_recipe(&app, "Spaghetti", &["Mittagessen"]).await;

    // When: GET /?kategorie=Kuchen&kategorie=Brot
    let (_status, body) = get_body(app, "/?kategorie=Kuchen&kategorie=Brot").await;

    // Then: Käsekuchen und Partybrot sichtbar, Spaghetti nicht
    assert!(
        body.contains("Käsekuchen"),
        "Käsekuchen sollte bei Kuchen+Brot-Filter sichtbar sein"
    );
    assert!(
        body.contains("Partybrot"),
        "Partybrot sollte bei Kuchen+Brot-Filter sichtbar sein (hat Brot)"
    );
    assert!(
        !body.contains("Spaghetti"),
        "Spaghetti sollte nicht bei Kuchen+Brot-Filter sichtbar sein"
    );
}

#[tokio::test]
async fn filter_returns_empty_state_message_for_category_without_recipes() {
    // Given: Nur Brot-Rezepte, keine Snacks
    let (app, _temp) = setup_test_app().await;
    create_recipe(&app, "Roggenbrot", &["Brot"]).await;

    // When: GET /?kategorie=Snacks
    let (_status, body) = get_body(app, "/?kategorie=Snacks").await;

    // Then: Leer-Meldung sichtbar, Roggenbrot nicht
    assert!(
        body.contains("Keine Rezepte in dieser Kategorie"),
        "Leer-Meldung sollte bei Kategorie ohne Rezepte erscheinen"
    );
    assert!(
        !body.contains("Roggenbrot"),
        "Roggenbrot sollte nicht sichtbar sein"
    );
}

#[tokio::test]
async fn filter_combined_with_search_applies_and_logic() {
    // Given: Dinkelbrot (Brot) und Roggenbrot (Brot)
    let (app, _temp) = setup_test_app().await;
    create_recipe(&app, "Dinkelbrot", &["Brot"]).await;
    create_recipe(&app, "Roggenbrot", &["Brot"]).await;

    // When: GET /?kategorie=Brot&q=dinkel
    let (_status, body) = get_body(app, "/?kategorie=Brot&q=dinkel").await;

    // Then: Dinkelbrot sichtbar, Roggenbrot nicht
    assert!(
        body.contains("Dinkelbrot"),
        "Dinkelbrot sollte bei Brot+dinkel-Filter sichtbar sein"
    );
    assert!(
        !body.contains("Roggenbrot"),
        "Roggenbrot sollte bei Brot+dinkel-Filter nicht sichtbar sein"
    );
}

#[tokio::test]
async fn filter_resets_when_no_kategorie_param() {
    // Given: Brot und Mittagessen-Rezepte
    let (app, _temp) = setup_test_app().await;
    create_recipe(&app, "Vollkornbrot", &["Brot"]).await;
    create_recipe(&app, "Spaghetti", &["Mittagessen"]).await;

    // When: GET / (kein kategorie-Parameter)
    let (_status, body) = get_body(app, "/").await;

    // Then: Alle Rezepte sichtbar
    assert!(
        body.contains("Vollkornbrot"),
        "Vollkornbrot sollte ohne Filter sichtbar sein"
    );
    assert!(
        body.contains("Spaghetti"),
        "Spaghetti sollte ohne Filter sichtbar sein"
    );
}

#[tokio::test]
async fn deeplink_with_kategorie_param_returns_200() {
    // Given: App mit Rezept
    let (app, _temp) = setup_test_app().await;
    create_recipe(&app, "Partykuchen", &["Party"]).await;

    // When: GET /?kategorie=Party (DeepLink)
    let (status, _body) = get_body(app, "/?kategorie=Party").await;

    // Then: HTTP 200
    assert_eq!(status, StatusCode::OK);
}

#[tokio::test]
async fn invalid_kategorie_param_is_silently_ignored() {
    // Given: App mit Rezept
    let (app, _temp) = setup_test_app().await;
    create_recipe(&app, "Apfelkuchen", &["Kuchen"]).await;

    // When: GET /?kategorie=Ungültig (ungültige Kategorie)
    let (status, body) = get_body(app, "/?kategorie=Ung%C3%BCltig").await;

    // Then: HTTP 200, alle Rezepte sichtbar (ungültige Kategorie ignoriert → keine Filterung)
    assert_eq!(status, StatusCode::OK);
    assert!(
        body.contains("Apfelkuchen"),
        "Apfelkuchen sollte bei ungültigem Filter sichtbar sein"
    );
}

#[tokio::test]
async fn category_filter_buttons_are_rendered_in_html() {
    // Given: App (keine speziellen Rezepte nötig)
    let (app, _temp) = setup_test_app().await;

    // When: GET /
    let (_status, body) = get_body(app, "/").await;

    // Then: Alle 5 Kategorie-Buttons sichtbar
    for cat in &["Mittagessen", "Brot", "Party", "Kuchen", "Snacks"] {
        assert!(
            body.contains(cat),
            "Kategorie-Button '{cat}' sollte auf der Seite sichtbar sein"
        );
    }
}

#[tokio::test]
async fn active_category_button_has_aria_pressed_true() {
    // Given: App mit Rezept
    let (app, _temp) = setup_test_app().await;

    // When: GET /?kategorie=Brot
    let (_status, body) = get_body(app, "/?kategorie=Brot").await;

    // Then: Der Brot-Button hat aria-pressed="true"
    // Wir prüfen, dass aria-pressed="true" im HTML vorkommt (beim aktiven Button)
    assert!(
        body.contains("aria-pressed=\"true\""),
        "Aktiver Kategorie-Button sollte aria-pressed=\"true\" haben"
    );
}
