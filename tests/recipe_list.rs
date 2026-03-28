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

async fn create_recipe_with_title(app: &axum::Router, title: &str, category: &str) -> i64 {
    let form_data = format!(
        "title={}&categories={}",
        urlencoding::encode(title),
        urlencoding::encode(category)
    );
    let request = Request::builder()
        .method("POST")
        .uri("/recipes")
        .header("Content-Type", "application/x-www-form-urlencoded")
        .body(Body::from(form_data))
        .unwrap();

    let response = app.clone().oneshot(request).await.unwrap();
    let location = response
        .headers()
        .get("location")
        .unwrap()
        .to_str()
        .unwrap()
        .to_string();
    location
        .split('/')
        .next_back()
        .unwrap()
        .parse::<i64>()
        .unwrap()
}

#[tokio::test]
async fn index_returns_200() {
    // Given: Eine leere Datenbank
    let (app, _temp) = setup_test_app().await;

    // When: GET / aufgerufen wird
    let (status, _body) = get_body(app, "/").await;

    // Then: HTTP 200
    assert_eq!(status, StatusCode::OK);
}

#[tokio::test]
async fn index_shows_h1_rezepte() {
    // Given: Eine leere Datenbank
    let (app, _temp) = setup_test_app().await;

    // When: GET / aufgerufen wird
    let (_status, body) = get_body(app, "/").await;

    // Then: H1 enthält genau "Rezepte"
    assert!(
        body.contains("<h1>Rezepte</h1>"),
        "H1 sollte genau 'Rezepte' enthalten, nicht 'Rezepte Übersicht'"
    );
}

#[tokio::test]
async fn index_shows_all_recipes() {
    // Given: Drei Rezepte wurden erstellt
    let (app, _temp) = setup_test_app().await;
    create_recipe_with_title(&app, "Apfelkuchen", "Kuchen").await;
    create_recipe_with_title(&app, "Bolognese", "Mittagessen").await;
    create_recipe_with_title(&app, "Zupfbrot", "Brot").await;

    // When: GET / aufgerufen wird
    let (_status, body) = get_body(app, "/").await;

    // Then: Alle drei Rezepte sind im Body enthalten
    assert!(
        body.contains("Apfelkuchen"),
        "Apfelkuchen sollte in der Liste sein"
    );
    assert!(
        body.contains("Bolognese"),
        "Bolognese sollte in der Liste sein"
    );
    assert!(
        body.contains("Zupfbrot"),
        "Zupfbrot sollte in der Liste sein"
    );
}

#[tokio::test]
async fn index_shows_recipes_in_alphabetical_order() {
    // Given: Zwei Rezepte in umgekehrter alphabetischer Reihenfolge erstellt
    let (app, _temp) = setup_test_app().await;
    create_recipe_with_title(&app, "Zupfbrot", "Brot").await;
    create_recipe_with_title(&app, "Apfelkuchen", "Kuchen").await;

    // When: GET / aufgerufen wird
    let (_status, body) = get_body(app, "/").await;

    // Then: Apfelkuchen (A) erscheint vor Zupfbrot (Z)
    let pos_apfel = body.find("Apfelkuchen").unwrap();
    let pos_zupf = body.find("Zupfbrot").unwrap();
    assert!(
        pos_apfel < pos_zupf,
        "Apfelkuchen (A) sollte vor Zupfbrot (Z) erscheinen"
    );
}

#[tokio::test]
async fn index_shows_empty_state_message() {
    // Given: Eine leere Datenbank
    let (app, _temp) = setup_test_app().await;

    // When: GET / aufgerufen wird
    let (_status, body) = get_body(app, "/").await;

    // Then: Leerzustand-Meldung "Noch keine Rezepte" wird angezeigt
    assert!(
        body.contains("Noch keine Rezepte"),
        "Leerzustand sollte 'Noch keine Rezepte' anzeigen"
    );
}

#[tokio::test]
async fn index_shows_create_link_in_empty_state() {
    // Given: Eine leere Datenbank
    let (app, _temp) = setup_test_app().await;

    // When: GET / aufgerufen wird
    let (_status, body) = get_body(app, "/").await;

    // Then: Link zu /recipes/new ist im Leerzustand enthalten
    assert!(
        body.contains("/recipes/new"),
        "Leerzustand sollte Link zu /recipes/new enthalten"
    );
}

#[tokio::test]
async fn index_shows_new_recipe_button() {
    // Given: Eine leere Datenbank
    let (app, _temp) = setup_test_app().await;

    // When: GET / aufgerufen wird
    let (_status, body) = get_body(app, "/").await;

    // Then: "Neues Rezept"-Button mit Link zu /recipes/new ist vorhanden
    assert!(
        body.contains("/recipes/new"),
        "'Neues Rezept'-Button sollte auf der Seite vorhanden sein"
    );
    assert!(
        body.contains("Neues Rezept"),
        "Seite sollte den Text 'Neues Rezept' enthalten"
    );
}

#[tokio::test]
async fn index_shows_category_for_recipe() {
    // Given: Ein Rezept mit Kategorie "Mittagessen" wurde erstellt
    let (app, _temp) = setup_test_app().await;
    create_recipe_with_title(&app, "Gulasch", "Mittagessen").await;

    // When: GET / aufgerufen wird
    let (_status, body) = get_body(app, "/").await;

    // Then: Kategorie "Mittagessen" ist im Listeneintrag sichtbar
    assert!(
        body.contains("Mittagessen"),
        "Kategorie 'Mittagessen' sollte im Listeneintrag sichtbar sein"
    );
}

#[tokio::test]
async fn index_recipe_links_to_detail() {
    // Given: Ein Rezept wurde erstellt
    let (app, _temp) = setup_test_app().await;
    let id = create_recipe_with_title(&app, "Pfannkuchen", "Snacks").await;

    // When: GET / aufgerufen wird
    let (_status, body) = get_body(app, "/").await;

    // Then: Listeneintrag enthält Link zur Detailseite /recipes/{id}
    let expected_link = format!("/recipes/{}", id);
    assert!(
        body.contains(&expected_link),
        "Listeneintrag sollte Link zu /recipes/{{id}} enthalten"
    );
}
