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
    category: &str,
    ingredients: Option<&str>,
    instructions: Option<&str>,
) {
    let mut form_data = format!(
        "title={}&categories={}",
        urlencoding::encode(title),
        urlencoding::encode(category)
    );
    if let Some(ing) = ingredients {
        form_data.push_str(&format!("&ingredients={}", urlencoding::encode(ing)));
    }
    if let Some(inst) = instructions {
        form_data.push_str(&format!("&instructions={}", urlencoding::encode(inst)));
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
async fn search_returns_200_for_query() {
    // Given: App mit Rezepten
    let (app, _temp) = setup_test_app().await;
    create_recipe(
        &app,
        "Spaghetti Bolognese",
        "Mittagessen",
        Some("Hackfleisch, Tomaten"),
        Some("Sauce kochen"),
    )
    .await;

    // When: GET /?q=bolognese
    let (status, _body) = get_body(app, "/?q=bolognese").await;

    // Then: HTTP 200
    assert_eq!(status, StatusCode::OK);
}

#[tokio::test]
async fn search_finds_recipe_by_title() {
    // Given: App mit "Spaghetti Bolognese" und "Pfannkuchen"
    let (app, _temp) = setup_test_app().await;
    create_recipe(
        &app,
        "Spaghetti Bolognese",
        "Mittagessen",
        Some("Hackfleisch, Tomaten"),
        Some("Sauce kochen"),
    )
    .await;
    create_recipe(&app, "Pfannkuchen", "Snacks", None, None).await;

    // When: GET /?q=bolognese
    let (_status, body) = get_body(app, "/?q=bolognese").await;

    // Then: "Spaghetti Bolognese" ist im Body, "Pfannkuchen" nicht
    assert!(
        body.contains("Spaghetti Bolognese"),
        "Spaghetti Bolognese sollte im Suchergebnis sein"
    );
    assert!(
        !body.contains("Pfannkuchen"),
        "Pfannkuchen sollte nicht im Suchergebnis sein"
    );
}

#[tokio::test]
async fn search_finds_recipe_by_ingredients() {
    // Given: Rezept mit "Dinkelvollkornmehl" in Zutaten
    let (app, _temp) = setup_test_app().await;
    create_recipe(
        &app,
        "Pfannkuchen",
        "Snacks",
        Some("Dinkelvollkornmehl, Eier, Milch"),
        Some("Teig mischen"),
    )
    .await;
    create_recipe(
        &app,
        "Brot",
        "Brot",
        Some("Weizenmehl, Hefe"),
        Some("Backen"),
    )
    .await;

    // When: GET /?q=dinkel
    let (_status, body) = get_body(app, "/?q=dinkel").await;

    // Then: Pfannkuchen (mit Dinkel) sichtbar; Brot-Rezept nicht als Rezept-Titel in Ergebnisliste.
    // Hinweis: "Brot" erscheint als Kategorie-Filter-Button — deshalb prüfen wir
    // den Rezept-Titel als H2-Element, der nur in der Ergebnisliste vorkommt.
    assert!(
        body.contains("Pfannkuchen"),
        "Pfannkuchen sollte gefunden werden (Dinkel in Zutaten)"
    );
    assert!(
        !body.contains("<h2>Brot</h2>"),
        "Brot-Rezept sollte nicht als Ergebnis erscheinen"
    );
}

#[tokio::test]
async fn search_finds_recipe_by_instructions() {
    // Given: Rezept mit "im Ofen backen" in Anleitung
    let (app, _temp) = setup_test_app().await;
    create_recipe(
        &app,
        "Brot im Ofen",
        "Brot",
        Some("Mehl, Hefe, Wasser"),
        Some("Teig kneten und im Ofen backen"),
    )
    .await;
    create_recipe(
        &app,
        "Nudeln",
        "Mittagessen",
        Some("Nudeln, Salz"),
        Some("In kochendem Wasser garen"),
    )
    .await;

    // When: GET /?q=ofen
    let (_status, body) = get_body(app, "/?q=ofen").await;

    // Then: "Brot im Ofen" gefunden, "Nudeln" nicht
    assert!(
        body.contains("Brot im Ofen"),
        "Brot im Ofen sollte gefunden werden (Ofen in Anleitung)"
    );
    assert!(
        !body.contains("Nudeln"),
        "Nudeln sollte nicht gefunden werden"
    );
}

#[tokio::test]
async fn search_is_case_insensitive() {
    // Given: Rezept "Spaghetti Bolognese"
    let (app, _temp) = setup_test_app().await;
    create_recipe(&app, "Spaghetti Bolognese", "Mittagessen", None, None).await;

    // When: GET /?q=BOLOGNESE (Großbuchstaben)
    let (_status, body) = get_body(app, "/?q=BOLOGNESE").await;

    // Then: "Spaghetti Bolognese" trotzdem gefunden
    assert!(
        body.contains("Spaghetti Bolognese"),
        "Suche sollte case-insensitiv sein"
    );
}

#[tokio::test]
async fn search_shows_no_results_message() {
    // Given: App mit Rezepten
    let (app, _temp) = setup_test_app().await;
    create_recipe(&app, "Apfelkuchen", "Kuchen", None, None).await;

    // When: GET /?q=xyzxyz (kein Treffer)
    let (_status, body) = get_body(app, "/?q=xyzxyz").await;

    // Then: "Keine Rezepte" Meldung im Body
    assert!(
        body.contains("Keine Rezepte"),
        "Keine-Treffer-Meldung sollte angezeigt werden"
    );
    assert!(
        body.contains("xyzxyz"),
        "Suchbegriff sollte in der Meldung erscheinen"
    );
}

#[tokio::test]
async fn search_with_empty_query_shows_all_recipes() {
    // Given: App mit mehreren Rezepten
    let (app, _temp) = setup_test_app().await;
    create_recipe(&app, "Apfelkuchen", "Kuchen", None, None).await;
    create_recipe(&app, "Bolognese", "Mittagessen", None, None).await;

    // When: GET /?q= (leere Suche)
    let (_status, body) = get_body(app, "/?q=").await;

    // Then: Alle Rezepte sind sichtbar
    assert!(
        body.contains("Apfelkuchen"),
        "Apfelkuchen sollte bei leerer Suche sichtbar sein"
    );
    assert!(
        body.contains("Bolognese"),
        "Bolognese sollte bei leerer Suche sichtbar sein"
    );
}

#[tokio::test]
async fn index_without_query_shows_all_recipes() {
    // Given: App mit mehreren Rezepten
    let (app, _temp) = setup_test_app().await;
    create_recipe(&app, "Apfelkuchen", "Kuchen", None, None).await;
    create_recipe(&app, "Zupfbrot", "Brot", None, None).await;

    // When: GET / (kein q-Parameter, Rückwärtskompatibilität)
    let (_status, body) = get_body(app, "/").await;

    // Then: Alle Rezepte sichtbar
    assert!(
        body.contains("Apfelkuchen"),
        "Apfelkuchen sollte ohne q-Parameter sichtbar sein"
    );
    assert!(
        body.contains("Zupfbrot"),
        "Zupfbrot sollte ohne q-Parameter sichtbar sein"
    );
}
