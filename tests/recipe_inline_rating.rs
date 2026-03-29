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

async fn create_test_recipe(app: &axum::Router, form_data: &str) -> i64 {
    let request = Request::builder()
        .method("POST")
        .uri("/recipes")
        .header("Content-Type", "application/x-www-form-urlencoded")
        .body(Body::from(form_data.to_string()))
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

async fn post_rating(app: axum::Router, id: i64, rating_body: &str) -> (StatusCode, String) {
    let request = Request::builder()
        .method("POST")
        .uri(format!("/recipes/{}/rating", id))
        .header("Content-Type", "application/x-www-form-urlencoded")
        .body(Body::from(rating_body.to_string()))
        .unwrap();
    let response = app.oneshot(request).await.unwrap();
    let status = response.status();
    let bytes = response.into_body().collect().await.unwrap().to_bytes();
    let body = String::from_utf8(bytes.to_vec()).unwrap();
    (status, body)
}

async fn get_body(app: axum::Router, uri: &str) -> (StatusCode, String) {
    let response = app
        .oneshot(Request::builder().uri(uri).body(Body::empty()).unwrap())
        .await
        .unwrap();
    let status = response.status();
    let bytes = response.into_body().collect().await.unwrap().to_bytes();
    let body = String::from_utf8(bytes.to_vec()).unwrap();
    (status, body)
}

#[tokio::test]
async fn update_rating_stores_new_value() {
    // Given: Ein Rezept ohne Bewertung
    let (app, _temp) = setup_test_app().await;
    let id = create_test_recipe(&app, "title=Inline%20Rating%20Test&categories=Mittagessen").await;

    // When: POST auf /recipes/:id/rating mit rating=4
    let (status, _body) = post_rating(app.clone(), id, "rating=4").await;

    // Then: HTTP 200 OK
    assert_eq!(status, StatusCode::OK);

    // And: GET der Detailseite zeigt 4 von 5 Sternen
    let (_, detail_body) = get_body(app, &format!("/recipes/{}", id)).await;
    assert!(
        detail_body.contains("4 von 5 Sternen"),
        "Detailseite sollte '4 von 5 Sternen' zeigen, got: {}",
        &detail_body[..500.min(detail_body.len())]
    );
}

#[tokio::test]
async fn update_rating_resets_to_none() {
    // Given: Ein Rezept mit Rating 3
    let (app, _temp) = setup_test_app().await;
    let id = create_test_recipe(&app, "title=Reset%20Test&categories=Mittagessen&rating=3").await;

    // When: POST mit leerem rating=
    let (status, _body) = post_rating(app.clone(), id, "rating=").await;
    assert_eq!(status, StatusCode::OK);

    // Then: GET der Detailseite zeigt keinen Sterne-Block (kein recipe-stars)
    let (_, detail_body) = get_body(app, &format!("/recipes/{}", id)).await;
    assert!(
        !detail_body.contains("recipe-stars"),
        "Detailseite sollte keinen Sterne-Block zeigen nach Reset"
    );
}

#[tokio::test]
async fn update_rating_returns_html_fragment() {
    // Given: Ein Rezept
    let (app, _temp) = setup_test_app().await;
    let id = create_test_recipe(&app, "title=Fragment%20Test&categories=Mittagessen").await;

    // When: POST auf /recipes/:id/rating
    let (status, body) = post_rating(app, id, "rating=3").await;

    // Then: Response enthält den inline-rating Container
    assert_eq!(status, StatusCode::OK);
    assert!(
        body.contains("inline-rating"),
        "Response-Body sollte 'inline-rating' enthalten, got: {}",
        &body[..500.min(body.len())]
    );
}

#[tokio::test]
async fn update_rating_rejects_invalid_value() {
    // Given: Ein Rezept
    let (app, _temp) = setup_test_app().await;
    let id = create_test_recipe(&app, "title=Invalid%20Rating&categories=Mittagessen").await;

    // When: POST mit rating=6 (ungültig)
    let (status, _body) = post_rating(app, id, "rating=6").await;

    // Then: HTTP 400
    assert_eq!(status, StatusCode::BAD_REQUEST);
}

#[tokio::test]
async fn update_rating_returns_404_for_unknown_id() {
    // Given: Leere Datenbank
    let (app, _temp) = setup_test_app().await;

    // When: POST auf nicht existierende ID
    let (status, _body) = post_rating(app, 99999, "rating=4").await;

    // Then: HTTP 404
    assert_eq!(status, StatusCode::NOT_FOUND);
}
