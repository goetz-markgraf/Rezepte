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

async fn create_test_recipe(app: &axum::Router) -> i64 {
    let form_data = "title=Testrezept&categories=Mittagessen";
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
async fn confirm_delete_shows_recipe_title() {
    let (app, _temp) = setup_test_app().await;
    let id = create_test_recipe(&app).await;

    let response = app
        .clone()
        .oneshot(
            Request::builder()
                .uri(format!("/recipes/{}/confirm-delete", id))
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::OK);

    let body = response.into_body().collect().await.unwrap().to_bytes();
    let body_str = String::from_utf8(body.to_vec()).unwrap();
    assert!(body_str.contains("Testrezept"));
    assert!(body_str.contains("Wirklich löschen"));
    assert!(body_str.contains("Abbrechen"));
}

#[tokio::test]
async fn confirm_delete_returns_404_for_nonexistent() {
    let (app, _temp) = setup_test_app().await;

    let response = app
        .oneshot(
            Request::builder()
                .uri("/recipes/99999/confirm-delete")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::NOT_FOUND);
}

#[tokio::test]
async fn delete_recipe_removes_from_db() {
    let (app, _temp) = setup_test_app().await;
    let id = create_test_recipe(&app).await;

    let response = app
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri(format!("/recipes/{}/delete", id))
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::SEE_OTHER);
    let location = response
        .headers()
        .get("location")
        .unwrap()
        .to_str()
        .unwrap();
    assert!(location.starts_with("/?deleted="));

    // Verify recipe is gone
    let detail_response = app
        .oneshot(
            Request::builder()
                .uri(format!("/recipes/{}", id))
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();
    assert_eq!(detail_response.status(), StatusCode::NOT_FOUND);
}

#[tokio::test]
async fn delete_recipe_returns_404_for_nonexistent() {
    let (app, _temp) = setup_test_app().await;

    let response = app
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/recipes/99999/delete")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::NOT_FOUND);
}
