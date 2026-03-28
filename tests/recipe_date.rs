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

async fn body_text(body: axum::body::Body) -> String {
    let bytes = body.collect().await.unwrap().to_bytes();
    String::from_utf8_lossy(&bytes).to_string()
}

#[tokio::test]
async fn should_create_recipe_with_valid_date_and_redirect() {
    // Given: Eine leere Datenbank
    let (app, _temp) = setup_test_app().await;

    // When: Rezept mit gültigem Datum "5.3.2025" erstellt wird
    let form_data = "title=Datums-Rezept&categories=Mittagessen&planned_date=5.3.2025";

    let request = Request::builder()
        .method("POST")
        .uri("/recipes")
        .header("Content-Type", "application/x-www-form-urlencoded")
        .body(Body::from(form_data))
        .unwrap();

    let response = app.oneshot(request).await.unwrap();

    // Then: Redirect auf die Detailseite (HTTP 303)
    assert_eq!(response.status(), StatusCode::SEE_OTHER);
    let location = response
        .headers()
        .get("location")
        .unwrap()
        .to_str()
        .unwrap();
    assert!(location.starts_with("/recipes/"));
}

#[tokio::test]
async fn should_show_date_in_detail_view() {
    // Given: Ein Rezept mit Datum "5.3.2025" wurde erstellt
    let (app, _temp) = setup_test_app().await;

    let form_data = "title=Datums-Rezept&categories=Mittagessen&planned_date=5.3.2025";
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
        .unwrap()
        .to_string();

    // When: Die Detailseite aufgerufen wird
    let response = app
        .oneshot(
            Request::builder()
                .uri(&location)
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    // Then: Das Datum "März" ist in der Antwort sichtbar
    assert_eq!(response.status(), StatusCode::OK);
    let body = body_text(response.into_body()).await;
    assert!(
        body.contains("März"),
        "Detailseite sollte den Monatsnamen März enthalten"
    );
    assert!(
        body.contains("2025"),
        "Detailseite sollte das Jahr 2025 enthalten"
    );
}

#[tokio::test]
async fn should_reject_invalid_date_with_400() {
    // Given: Eine leere Datenbank
    let (app, _temp) = setup_test_app().await;

    // When: Rezept mit ungültigem Datum "morgen" gesendet wird
    let form_data = "title=Test%20Rezept&categories=Mittagessen&planned_date=morgen";

    let request = Request::builder()
        .method("POST")
        .uri("/recipes")
        .header("Content-Type", "application/x-www-form-urlencoded")
        .body(Body::from(form_data))
        .unwrap();

    let response = app.oneshot(request).await.unwrap();

    // Then: HTTP 400 mit Fehlermeldung
    assert_eq!(response.status(), StatusCode::BAD_REQUEST);
    let body = body_text(response.into_body()).await;
    assert!(
        body.contains("Kein gültiges Datum"),
        "Fehlermeldung sollte 'Kein gültiges Datum' enthalten"
    );
}

#[tokio::test]
async fn should_retain_form_values_on_invalid_date() {
    // Given: Eine leere Datenbank
    let (app, _temp) = setup_test_app().await;

    // When: Formular mit ungültigem Datum und einem Titel gesendet wird
    let form_data =
        "title=Mein%20Rezept&categories=Mittagessen&planned_date=kein%20datum&ingredients=Test";

    let request = Request::builder()
        .method("POST")
        .uri("/recipes")
        .header("Content-Type", "application/x-www-form-urlencoded")
        .body(Body::from(form_data))
        .unwrap();

    let response = app.oneshot(request).await.unwrap();

    // Then: HTTP 400, Titel und Zutaten bleiben im Formular erhalten
    assert_eq!(response.status(), StatusCode::BAD_REQUEST);
    let body = body_text(response.into_body()).await;
    assert!(
        body.contains("Mein Rezept"),
        "Titel sollte im Fehlerformular erhalten bleiben"
    );
    assert!(
        body.contains("Test"),
        "Zutaten sollten im Fehlerformular erhalten bleiben"
    );
}

#[tokio::test]
async fn should_create_recipe_without_date() {
    // Given: Eine leere Datenbank
    let (app, _temp) = setup_test_app().await;

    // When: Rezept ohne Datum erstellt wird (Feld leer)
    let form_data = "title=Ohne%20Datum&categories=Mittagessen&planned_date=";

    let request = Request::builder()
        .method("POST")
        .uri("/recipes")
        .header("Content-Type", "application/x-www-form-urlencoded")
        .body(Body::from(form_data))
        .unwrap();

    let response = app.oneshot(request).await.unwrap();

    // Then: Rezept wird erfolgreich erstellt (HTTP 303)
    assert_eq!(response.status(), StatusCode::SEE_OTHER);
}

#[tokio::test]
async fn should_clear_date_on_update() {
    // Given: Ein Rezept mit Datum "1.1.2025" existiert
    let (app, _temp) = setup_test_app().await;

    let form_data = "title=Rezept%20mit%20Datum&categories=Mittagessen&planned_date=1.1.2025";
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
        .unwrap()
        .to_string();
    let id: i64 = location.split('/').next_back().unwrap().parse().unwrap();

    // When: Das Datum wird im Update-Formular geleert
    let update_data = "title=Rezept%20mit%20Datum&categories=Mittagessen&planned_date=";
    let update_request = Request::builder()
        .method("POST")
        .uri(format!("/recipes/{}", id))
        .header("Content-Type", "application/x-www-form-urlencoded")
        .body(Body::from(update_data))
        .unwrap();

    let update_response = app.clone().oneshot(update_request).await.unwrap();
    assert_eq!(update_response.status(), StatusCode::SEE_OTHER);

    // Then: Die Detailseite zeigt kein Datum mehr an
    let detail_response = app
        .oneshot(
            Request::builder()
                .uri(format!("/recipes/{}", id))
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    let body = body_text(detail_response.into_body()).await;
    assert!(
        !body.contains("Januar"),
        "Kein Monat sollte nach dem Löschen des Datums erscheinen"
    );
}
