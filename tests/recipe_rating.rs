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

async fn create_recipe_with_form(
    app: axum::Router,
    form_data: &str,
) -> (axum::Router, StatusCode, String) {
    let request = Request::builder()
        .method("POST")
        .uri("/recipes")
        .header("Content-Type", "application/x-www-form-urlencoded")
        .body(Body::from(form_data.to_string()))
        .unwrap();
    let response = app.clone().oneshot(request).await.unwrap();
    let status = response.status();
    let location = response
        .headers()
        .get("location")
        .map(|v| v.to_str().unwrap().to_string())
        .unwrap_or_default();
    (app, status, location)
}

async fn get_response_body(app: axum::Router, uri: &str) -> String {
    let response = app
        .oneshot(Request::builder().uri(uri).body(Body::empty()).unwrap())
        .await
        .unwrap();
    let bytes = response.into_body().collect().await.unwrap().to_bytes();
    String::from_utf8(bytes.to_vec()).unwrap()
}

#[tokio::test]
async fn create_recipe_with_rating_stores_it() {
    // Given: Eine leere Datenbank
    let (app, _temp) = setup_test_app().await;

    // When: POST mit rating=4 und einem Rezept gesendet wird
    let form_data = "title=Test%20Rezept&categories=Mittagessen&rating=4";
    let (app, status, location) = create_recipe_with_form(app, form_data).await;

    // Then: Redirect auf die Detailseite
    assert_eq!(status, StatusCode::SEE_OTHER);

    // And: Detail-Seite zeigt 4 Sterne
    let body = get_response_body(app, &location).await;
    assert!(body.contains("4 von 5 Sternen"), "Sterne-Anzeige erwartet");
}

#[tokio::test]
async fn create_recipe_without_rating_stores_null() {
    // Given: Eine leere Datenbank
    let (app, _temp) = setup_test_app().await;

    // When: POST ohne rating-Feld gesendet wird
    let form_data = "title=Ohne%20Bewertung&categories=Mittagessen";
    let (app, status, location) = create_recipe_with_form(app, form_data).await;

    // Then: Redirect auf die Detailseite
    assert_eq!(status, StatusCode::SEE_OTHER);

    // And: Detail-Seite zeigt keinen Sterne-Block
    let body = get_response_body(app, &location).await;
    assert!(!body.contains("recipe-stars"), "Kein Sterne-Block erwartet");
}

#[tokio::test]
async fn update_recipe_changes_rating() {
    // Given: Ein Rezept mit Rating 3 existiert
    let (app, _temp) = setup_test_app().await;
    let form_data = "title=Update%20Test&categories=Mittagessen&rating=3";
    let (app, _, location) = create_recipe_with_form(app, form_data).await;
    let id: i64 = location.split('/').next_back().unwrap().parse().unwrap();

    // When: Update auf Rating 5 durchgeführt wird
    let update_data = "title=Update%20Test&categories=Mittagessen&rating=5";
    let update_request = Request::builder()
        .method("POST")
        .uri(format!("/recipes/{}", id))
        .header("Content-Type", "application/x-www-form-urlencoded")
        .body(Body::from(update_data))
        .unwrap();
    let update_response = app.clone().oneshot(update_request).await.unwrap();
    assert_eq!(update_response.status(), StatusCode::SEE_OTHER);

    // Then: Detail-Seite zeigt 5 Sterne
    let body = get_response_body(app, &format!("/recipes/{}", id)).await;
    assert!(
        body.contains("5 von 5 Sternen"),
        "5-Sterne-Anzeige erwartet"
    );
}

#[tokio::test]
async fn update_recipe_removes_rating() {
    // Given: Ein Rezept mit Rating 5 existiert
    let (app, _temp) = setup_test_app().await;
    let form_data = "title=Bewertung%20Entfernen&categories=Mittagessen&rating=5";
    let (app, _, location) = create_recipe_with_form(app, form_data).await;
    let id: i64 = location.split('/').next_back().unwrap().parse().unwrap();

    // When: Update mit leerem Rating durchgeführt wird
    let update_data = "title=Bewertung%20Entfernen&categories=Mittagessen&rating=";
    let update_request = Request::builder()
        .method("POST")
        .uri(format!("/recipes/{}", id))
        .header("Content-Type", "application/x-www-form-urlencoded")
        .body(Body::from(update_data))
        .unwrap();
    let update_response = app.clone().oneshot(update_request).await.unwrap();
    assert_eq!(update_response.status(), StatusCode::SEE_OTHER);

    // Then: Detail-Seite zeigt keinen Sterne-Block mehr
    let body = get_response_body(app, &format!("/recipes/{}", id)).await;
    assert!(
        !body.contains("recipe-stars"),
        "Kein Sterne-Block nach Entfernen erwartet"
    );
}

#[tokio::test]
async fn create_recipe_rejects_rating_zero() {
    // Given: Eine leere Datenbank
    let (app, _temp) = setup_test_app().await;

    // When: POST mit rating=0 (ungültig) gesendet wird
    let form_data = "title=Test%20Rezept&categories=Mittagessen&rating=0";
    let (_, status, _) = create_recipe_with_form(app, form_data).await;

    // Then: HTTP 400 Bad Request
    assert_eq!(status, StatusCode::BAD_REQUEST);
}

#[tokio::test]
async fn create_recipe_rejects_rating_six() {
    // Given: Eine leere Datenbank
    let (app, _temp) = setup_test_app().await;

    // When: POST mit rating=6 (ungültig) gesendet wird
    let form_data = "title=Test%20Rezept&categories=Mittagessen&rating=6";
    let (_, status, _) = create_recipe_with_form(app, form_data).await;

    // Then: HTTP 400 Bad Request
    assert_eq!(status, StatusCode::BAD_REQUEST);
}

#[tokio::test]
async fn create_recipe_accepts_rating_one() {
    // Given: Eine leere Datenbank
    let (app, _temp) = setup_test_app().await;

    // When: POST mit rating=1 (Minimum, gültig) gesendet wird
    let form_data = "title=Test%20Rezept&categories=Mittagessen&rating=1";
    let (_, status, _) = create_recipe_with_form(app, form_data).await;

    // Then: Redirect (Erfolg)
    assert_eq!(status, StatusCode::SEE_OTHER);
}

#[tokio::test]
async fn create_recipe_accepts_rating_five() {
    // Given: Eine leere Datenbank
    let (app, _temp) = setup_test_app().await;

    // When: POST mit rating=5 (Maximum, gültig) gesendet wird
    let form_data = "title=Test%20Rezept&categories=Mittagessen&rating=5";
    let (_, status, _) = create_recipe_with_form(app, form_data).await;

    // Then: Redirect (Erfolg)
    assert_eq!(status, StatusCode::SEE_OTHER);
}

#[tokio::test]
async fn recipe_list_shows_rating() {
    // Given: Ein Rezept mit Rating 5 wurde erstellt
    let (app, _temp) = setup_test_app().await;
    let form_data = "title=Sterne%20Rezept&categories=Mittagessen&rating=5";
    let (app, _, _) = create_recipe_with_form(app, form_data).await;

    // When: Liste abgerufen wird
    let body = get_response_body(app, "/").await;

    // Then: Sterne sind in der Liste sichtbar
    assert!(body.contains("recipe-stars"), "Sterne in Liste erwartet");
    assert!(
        body.contains("5 von 5 Sternen"),
        "5-Sterne aria-label erwartet"
    );
}

#[tokio::test]
async fn recipe_list_hides_rating_when_none() {
    // Given: Ein Rezept ohne Rating wurde erstellt
    let (app, _temp) = setup_test_app().await;
    let form_data = "title=Ohne%20Bewertung&categories=Mittagessen";
    let (app, _, _) = create_recipe_with_form(app, form_data).await;

    // When: Liste abgerufen wird
    let body = get_response_body(app, "/").await;

    // Then: Keine Sterne in der Liste
    assert!(
        !body.contains("recipe-stars"),
        "Keine Sterne in Liste erwartet"
    );
}

#[tokio::test]
async fn form_prefills_rating_when_editing() {
    // Given: Ein Rezept mit Rating 3 existiert
    let (app, _temp) = setup_test_app().await;
    let form_data = "title=Edit%20Test&categories=Mittagessen&rating=3";
    let (app, _, location) = create_recipe_with_form(app, form_data).await;
    let id: i64 = location.split('/').next_back().unwrap().parse().unwrap();

    // When: Edit-Formular für das Rezept geladen wird
    let body = get_response_body(app, &format!("/recipes/{}/edit", id)).await;

    // Then: Radio-Button für 3 Sterne ist ausgewählt (value="3" ... checked)
    assert!(
        body.contains(r#"value="3" checked"#),
        "Radio-Button für 3 Sterne sollte checked sein"
    );
}
