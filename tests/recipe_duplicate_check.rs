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

#[tokio::test]
async fn check_duplicate_returns_200() {
    // Given: App mit einer leeren Datenbank
    let (app, _temp) = setup_test_app().await;

    // When: GET /recipes/check-duplicate?title=Test
    let response = app
        .oneshot(
            Request::builder()
                .uri("/recipes/check-duplicate?title=Test")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    // Then: HTTP 200 (auch ohne Treffer)
    assert_eq!(response.status(), StatusCode::OK);
}

#[tokio::test]
async fn check_duplicate_returns_empty_for_short_title() {
    // Given: App mit Rezept "Dinkelbrot"
    let (app, _temp) = setup_test_app().await;
    create_recipe(app.clone(), "Dinkelbrot").await;

    // When: GET /recipes/check-duplicate?title=Di (nur 2 Zeichen)
    let response = app
        .oneshot(
            Request::builder()
                .uri("/recipes/check-duplicate?title=Di")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    // Then: HTTP 200, Body enthält KEINEN Hinweis-Block
    assert_eq!(response.status(), StatusCode::OK);
    let body_bytes = response.into_body().collect().await.unwrap().to_bytes();
    let body = std::str::from_utf8(&body_bytes).unwrap();
    assert!(
        !body.contains("duplicate-hint-info"),
        "Kein Hinweis bei zu kurzem Titel erwartet"
    );
}

#[tokio::test]
async fn check_duplicate_finds_similar_recipe() {
    // Given: App mit Rezept "Dinkelbrot"
    let (app, _temp) = setup_test_app().await;
    create_recipe(app.clone(), "Dinkelbrot").await;

    // When: GET /recipes/check-duplicate?title=Dinkel
    let response = app
        .oneshot(
            Request::builder()
                .uri("/recipes/check-duplicate?title=Dinkel")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    // Then: HTTP 200, Body enthält "Dinkelbrot"
    assert_eq!(response.status(), StatusCode::OK);
    let body_bytes = response.into_body().collect().await.unwrap().to_bytes();
    let body = std::str::from_utf8(&body_bytes).unwrap();
    assert!(
        body.contains("Dinkelbrot"),
        "Erwarteter Treffer 'Dinkelbrot' nicht im Body"
    );
}

#[tokio::test]
async fn check_duplicate_is_case_insensitive() {
    // Given: App mit Rezept "Dinkelbrot"
    let (app, _temp) = setup_test_app().await;
    create_recipe(app.clone(), "Dinkelbrot").await;

    // When: GET /recipes/check-duplicate?title=DINKEL
    let response = app
        .oneshot(
            Request::builder()
                .uri("/recipes/check-duplicate?title=DINKEL")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    // Then: Body enthält "Dinkelbrot" (case-insensitiv)
    let body_bytes = response.into_body().collect().await.unwrap().to_bytes();
    let body = std::str::from_utf8(&body_bytes).unwrap();
    assert!(
        body.contains("Dinkelbrot"),
        "Case-insensitiver Treffer 'Dinkelbrot' nicht im Body"
    );
}

#[tokio::test]
async fn check_duplicate_excludes_self() {
    // Given: Rezept "Dinkelbrot" mit bekannter ID
    let (app, _temp) = setup_test_app().await;
    let id = create_recipe(app.clone(), "Dinkelbrot").await;

    // When: GET /recipes/check-duplicate?title=Dinkelbrot&exclude_id=<id>
    let uri = format!(
        "/recipes/check-duplicate?title=Dinkelbrot&exclude_id={}",
        id
    );
    let response = app
        .oneshot(Request::builder().uri(uri).body(Body::empty()).unwrap())
        .await
        .unwrap();

    // Then: Body enthält "Dinkelbrot" NICHT (eigenes Rezept ausgeschlossen)
    let body_bytes = response.into_body().collect().await.unwrap().to_bytes();
    let body = std::str::from_utf8(&body_bytes).unwrap();
    assert!(
        !body.contains("Dinkelbrot"),
        "Eigenes Rezept sollte nicht als Duplikat erscheinen"
    );
}

#[tokio::test]
async fn check_duplicate_returns_empty_when_no_match() {
    // Given: App mit Rezept "Spaghetti"
    let (app, _temp) = setup_test_app().await;
    create_recipe(app.clone(), "Spaghetti").await;

    // When: GET /recipes/check-duplicate?title=Dinkel
    let response = app
        .oneshot(
            Request::builder()
                .uri("/recipes/check-duplicate?title=Dinkel")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    // Then: Body enthält KEINEN Hinweis-Block
    assert_eq!(response.status(), StatusCode::OK);
    let body_bytes = response.into_body().collect().await.unwrap().to_bytes();
    let body = std::str::from_utf8(&body_bytes).unwrap();
    assert!(
        !body.contains("duplicate-hint-info"),
        "Kein Hinweis bei fehlendem Treffer erwartet"
    );
}
