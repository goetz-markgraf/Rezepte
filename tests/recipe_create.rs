use axum::body::Body;
use axum::http::{Request, StatusCode};
use rezepte::{create_pool, create_router};
use tower::ServiceExt;

async fn setup_test_app() -> (axum::Router, tempfile::NamedTempFile) {
    let temp_file = tempfile::NamedTempFile::new().unwrap();
    let db_url = format!("sqlite:{}", temp_file.path().to_str().unwrap());
    let pool = create_pool(&db_url).await.unwrap();
    let app = create_router(pool);
    (app, temp_file)
}

#[tokio::test]
async fn should_show_recipe_form() {
    let (app, _temp) = setup_test_app().await;

    let response = app
        .oneshot(
            Request::builder()
                .uri("/recipes/new")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::OK);
}

#[tokio::test]
async fn should_create_recipe_successfully() {
    let (app, _temp) = setup_test_app().await;

    let form_data = "title=Test%20Rezept&categories=Mittagessen";

    let request = Request::builder()
        .method("POST")
        .uri("/recipes")
        .header("Content-Type", "application/x-www-form-urlencoded")
        .body(Body::from(form_data))
        .unwrap();

    let response = app.oneshot(request).await.unwrap();

    assert_eq!(response.status(), StatusCode::SEE_OTHER);
    let location = response.headers().get("location").unwrap();
    assert!(location.to_str().unwrap().starts_with("/recipes/"));
}

#[tokio::test]
async fn should_validate_required_fields() {
    let (app, _temp) = setup_test_app().await;

    let form_data = "title=&categories=";

    let request = Request::builder()
        .method("POST")
        .uri("/recipes")
        .header("Content-Type", "application/x-www-form-urlencoded")
        .body(Body::from(form_data))
        .unwrap();

    let response = app.oneshot(request).await.unwrap();

    assert_eq!(response.status(), StatusCode::BAD_REQUEST);
}

#[tokio::test]
async fn should_show_recipe_detail() {
    let (app, _temp) = setup_test_app().await;

    // Create a recipe first
    let form_data = "title=Test%20Rezept&categories=Mittagessen&ingredients=Test%20Zutaten&instructions=Test%20Anleitung";

    let create_request = Request::builder()
        .method("POST")
        .uri("/recipes")
        .header("Content-Type", "application/x-www-form-urlencoded")
        .body(Body::from(form_data))
        .unwrap();

    let create_response = app.clone().oneshot(create_request).await.unwrap();
    let location = create_response
        .headers()
        .get("location")
        .unwrap()
        .to_str()
        .unwrap();
    let id: i64 = location.split('/').next_back().unwrap().parse().unwrap();

    // Now get the detail page
    let response = app
        .oneshot(
            Request::builder()
                .uri(format!("/recipes/{}", id))
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::OK);
}

#[tokio::test]
async fn should_show_index_page() {
    let (app, _temp) = setup_test_app().await;

    let response = app
        .oneshot(Request::builder().uri("/").body(Body::empty()).unwrap())
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::OK);
}
