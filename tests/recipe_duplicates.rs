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

/// Erstellt ein Rezept über den POST-Endpunkt und gibt die ID zurück.
async fn create_recipe(app: axum::Router, title: &str) -> i64 {
    let form_data = format!(
        "title={}&categories=Mittagessen",
        urlencoding::encode(title)
    );
    let request = Request::builder()
        .method("POST")
        .uri("/recipes")
        .header("Content-Type", "application/x-www-form-urlencoded")
        .body(Body::from(form_data))
        .unwrap();
    let response = app.oneshot(request).await.unwrap();
    let location = response
        .headers()
        .get("location")
        .unwrap()
        .to_str()
        .unwrap()
        .to_string();
    // Location: /recipes/42
    location.split('/').last().unwrap().parse::<i64>().unwrap()
}

/// Hilfsfunktion: GET /recipes/duplicates und Body als String zurückgeben.
async fn get_duplicates_page(app: axum::Router) -> (StatusCode, String) {
    let response = app
        .oneshot(
            Request::builder()
                .uri("/recipes/duplicates")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();
    let status = response.status();
    let body_bytes = response.into_body().collect().await.unwrap().to_bytes();
    let body = std::str::from_utf8(&body_bytes).unwrap().to_string();
    (status, body)
}

#[tokio::test]
async fn duplicates_page_returns_200() {
    // Given: App mit leerer Datenbank
    let (app, _temp) = setup_test_app().await;

    // When: GET /recipes/duplicates
    let (status, _body) = get_duplicates_page(app).await;

    // Then: HTTP 200
    assert_eq!(status, StatusCode::OK);
}

#[tokio::test]
async fn duplicates_page_shows_similar_pair() {
    // Given: Zwei Rezepte, wobei Titel A ein Substring von Titel B ist
    // "Dinkel" ist Substring von "Dinkelbrot" → LIKE %Dinkel% matcht "Dinkelbrot"
    let (app, _temp) = setup_test_app().await;
    let id_a = create_recipe(app.clone(), "Dinkel").await;
    let id_b = create_recipe(app.clone(), "Dinkelbrot").await;

    // When: GET /recipes/duplicates
    let (_status, body) = get_duplicates_page(app).await;

    // Then: Beide Rezept-Titel erscheinen im HTML
    assert!(
        body.contains("Dinkel"),
        "Erwarteter Titel 'Dinkel' nicht im Body (id_a={})",
        id_a
    );
    assert!(
        body.contains("Dinkelbrot"),
        "Erwarteter Titel 'Dinkelbrot' nicht im Body (id_b={})",
        id_b
    );
}

#[tokio::test]
async fn duplicates_page_shows_empty_message_when_no_duplicates() {
    // Given: Rezepte ohne ähnliche Titel existieren
    let (app, _temp) = setup_test_app().await;
    create_recipe(app.clone(), "Spaghetti Bolognese").await;
    create_recipe(app.clone(), "Apfelkuchen").await;

    // When: GET /recipes/duplicates
    let (_status, body) = get_duplicates_page(app).await;

    // Then: Leerzustand-Meldung erscheint
    assert!(
        body.contains("sauber"),
        "Leerzustand-Meldung 'sauber' nicht im Body"
    );
    // And: Kein Paar-Element vorhanden
    assert!(
        !body.contains("duplicate-pair"),
        "Keine Paar-Elemente erwartet, aber 'duplicate-pair' im Body gefunden"
    );
}

#[tokio::test]
async fn duplicates_page_links_to_recipe_detail() {
    // Given: Zwei ähnliche Rezepte, wobei "Dinkel" ein Substring von "Dinkelbrot" ist
    let (app, _temp) = setup_test_app().await;
    let id_a = create_recipe(app.clone(), "Dinkel").await;
    let id_b = create_recipe(app.clone(), "Dinkelbrot").await;

    // When: GET /recipes/duplicates
    let (_status, body) = get_duplicates_page(app).await;

    // Then: Links zu beiden Rezepten vorhanden
    let link_a = format!("href=\"/recipes/{}\"", id_a);
    let link_b = format!("href=\"/recipes/{}\"", id_b);
    assert!(
        body.contains(&link_a),
        "Link zu Rezept A (id={}) nicht im Body gefunden",
        id_a
    );
    assert!(
        body.contains(&link_b),
        "Link zu Rezept B (id={}) nicht im Body gefunden",
        id_b
    );
}

#[tokio::test]
async fn duplicates_page_pair_appears_only_once() {
    // Given: Zwei ähnliche Rezepte, wobei Titel A ein Substring von Titel B ist
    // "Brot" ist Substring von "Brotkorb" → LIKE %Brot% matcht "Brotkorb"
    // Die Deduplizierung stellt sicher, dass das Paar nur einmal erscheint (nicht A→B und B→A)
    let (app, _temp) = setup_test_app().await;
    create_recipe(app.clone(), "Brot").await;
    create_recipe(app.clone(), "Brotkorb").await;

    // When: GET /recipes/duplicates
    let (_status, body) = get_duplicates_page(app).await;

    // Then: Genau ein <li class="duplicate-pair"> im HTML (nicht zwei Paare)
    let pair_count = body.matches(r#"class="duplicate-pair""#).count();
    assert_eq!(
        pair_count, 1,
        "Exakt ein Paar-Element erwartet (Deduplizierung), aber {} gefunden",
        pair_count
    );
}

#[tokio::test]
async fn duplicates_page_recipe_not_paired_with_itself() {
    // Given: Ein einzelnes Rezept existiert
    let (app, _temp) = setup_test_app().await;
    create_recipe(app.clone(), "Einziges Rezept").await;

    // When: GET /recipes/duplicates
    let (_status, body) = get_duplicates_page(app).await;

    // Then: Kein Paar-Element (Rezept darf nicht mit sich selbst gepaart werden)
    assert!(
        !body.contains("duplicate-pair"),
        "Rezept sollte nicht mit sich selbst gepaart werden"
    );
    // And: Leerzustand-Meldung erscheint
    assert!(
        body.contains("sauber"),
        "Leerzustand-Meldung 'sauber' sollte erscheinen"
    );
}
