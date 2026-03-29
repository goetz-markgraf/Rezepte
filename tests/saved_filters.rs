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

async fn post_form(app: &axum::Router, uri: &str, body: &str) -> (StatusCode, Option<String>) {
    let request = Request::builder()
        .method("POST")
        .uri(uri)
        .header("Content-Type", "application/x-www-form-urlencoded")
        .body(Body::from(body.to_string()))
        .unwrap();

    let response = app.clone().oneshot(request).await.unwrap();
    let status = response.status();
    let location = response
        .headers()
        .get("Location")
        .and_then(|v| v.to_str().ok())
        .map(|s| s.to_string());
    (status, location)
}

async fn post_form_htmx(app: &axum::Router, uri: &str, body: &str) -> StatusCode {
    let request = Request::builder()
        .method("POST")
        .uri(uri)
        .header("Content-Type", "application/x-www-form-urlencoded")
        .header("HX-Request", "true")
        .body(Body::from(body.to_string()))
        .unwrap();

    let response = app.clone().oneshot(request).await.unwrap();
    response.status()
}

#[tokio::test]
async fn filter_speichern_happy_path() {
    // Gegeben: App mit leerem Zustand
    let (app, _temp) = setup_test_app().await;

    // Wenn: POST /saved-filters mit name="Brot-Ideen" und query_string="kategorie=Brot"
    let (status, location) = post_form(
        &app,
        "/saved-filters",
        "name=Brot-Ideen&query_string=kategorie%3DBrot",
    )
    .await;

    // Then: Redirect (303) zurück zur Startseite
    assert_eq!(status, StatusCode::SEE_OTHER);
    assert!(
        location.as_deref().unwrap_or("").contains("kategorie"),
        "Redirect sollte kategorie-Parameter enthalten"
    );

    // Und: GET / enthält "Brot-Ideen" in der HTML-Antwort
    let (_status, body) = get_body(app, "/").await;
    assert!(
        body.contains("Brot-Ideen"),
        "Gespeicherter Filter 'Brot-Ideen' sollte in der HTML-Antwort sichtbar sein"
    );
}

#[tokio::test]
async fn filter_laden_zeigt_gespeicherten_filter() {
    // Gegeben: Ein gespeicherter Filter "Brot-Ideen" wird angelegt
    let (app, _temp) = setup_test_app().await;
    post_form(
        &app,
        "/saved-filters",
        "name=Brot-Ideen&query_string=kategorie%3DBrot",
    )
    .await;

    // Wenn: GET /
    let (_status, body) = get_body(app, "/").await;

    // Dann: Body enthält Link mit href="/?kategorie=Brot"
    assert!(
        body.contains("/?kategorie=Brot"),
        "Body sollte Link mit href='/?kategorie=Brot' enthalten"
    );
}

#[tokio::test]
async fn filter_loeschen_htmx_request() {
    // Gegeben: Gespeicherter Filter "Brot-Ideen"
    let (app, _temp) = setup_test_app().await;
    post_form(
        &app,
        "/saved-filters",
        "name=Brot-Ideen&query_string=kategorie%3DBrot",
    )
    .await;

    // Und: ID des gespeicherten Filters aus dem HTML auslesen
    let (_status, body) = get_body(app.clone(), "/").await;
    // ID finden: nach "saved-filters/" im href
    let id_start = body
        .find("/saved-filters/")
        .expect("saved-filters link sollte vorhanden sein");
    let id_part = &body[id_start + "/saved-filters/".len()..];
    let id_end = id_part.find('/').unwrap_or(id_part.len());
    let id: i64 = id_part[..id_end].parse().expect("ID sollte eine Zahl sein");

    // Wenn: POST /saved-filters/:id/delete mit HX-Request: true Header
    let status = post_form_htmx(&app, &format!("/saved-filters/{}/delete", id), "").await;

    // Dann: 200-Antwort
    assert_eq!(status, StatusCode::OK);

    // Und: GET / enthält "Brot-Ideen" nicht mehr
    let (_status, body) = get_body(app, "/").await;
    assert!(
        !body.contains("Brot-Ideen"),
        "Gelöschter Filter 'Brot-Ideen' sollte nicht mehr in der HTML-Antwort sichtbar sein"
    );
}

#[tokio::test]
async fn filter_loeschen_fallback_ohne_htmx() {
    // Gegeben: Gespeicherter Filter "Brot-Ideen"
    let (app, _temp) = setup_test_app().await;
    post_form(
        &app,
        "/saved-filters",
        "name=Brot-Ideen&query_string=kategorie%3DBrot",
    )
    .await;

    // Und: ID des gespeicherten Filters aus dem HTML auslesen
    let (_status, body) = get_body(app.clone(), "/").await;
    let id_start = body
        .find("/saved-filters/")
        .expect("saved-filters link sollte vorhanden sein");
    let id_part = &body[id_start + "/saved-filters/".len()..];
    let id_end = id_part.find('/').unwrap_or(id_part.len());
    let id: i64 = id_part[..id_end].parse().expect("ID sollte eine Zahl sein");

    // Wenn: POST /saved-filters/:id/delete ohne HX-Request Header
    let (status, location) = post_form(&app, &format!("/saved-filters/{}/delete", id), "").await;

    // Dann: 303-Redirect zu /
    assert_eq!(status, StatusCode::SEE_OTHER);
    assert_eq!(location.as_deref(), Some("/"));
}

#[tokio::test]
async fn duplikater_name_gibt_fehler_redirect() {
    // Gegeben: Filter "Brot-Ideen" bereits gespeichert
    let (app, _temp) = setup_test_app().await;
    post_form(
        &app,
        "/saved-filters",
        "name=Brot-Ideen&query_string=kategorie%3DBrot",
    )
    .await;

    // Wenn: POST /saved-filters mit name="Brot-Ideen" und query_string="bewertung=gut"
    let (status, location) = post_form(
        &app,
        "/saved-filters",
        "name=Brot-Ideen&query_string=bewertung%3Dgut",
    )
    .await;

    // Dann: Redirect-URL enthält save_error=duplikat
    assert_eq!(status, StatusCode::SEE_OTHER);
    let loc = location.unwrap_or_default();
    assert!(
        loc.contains("save_error=duplikat"),
        "Redirect-URL sollte save_error=duplikat enthalten, war: {}",
        loc
    );
}

#[tokio::test]
async fn leerer_name_wird_abgelehnt() {
    // Gegeben: App im Basiszustand
    let (app, _temp) = setup_test_app().await;

    // Wenn: POST /saved-filters mit leerem name
    let (status, location) = post_form(
        &app,
        "/saved-filters",
        "name=&query_string=kategorie%3DBrot",
    )
    .await;

    // Dann: Redirect mit save_error oder anderer Fehlerbehandlung
    assert_eq!(status, StatusCode::SEE_OTHER);
    let loc = location.unwrap_or_default();
    assert!(
        loc.contains("save_error"),
        "Redirect-URL sollte save_error enthalten, war: {}",
        loc
    );
}

#[tokio::test]
async fn leerer_query_string_wird_abgelehnt() {
    // Gegeben: App im Basiszustand
    let (app, _temp) = setup_test_app().await;

    // Wenn: POST /saved-filters mit name="Test" und leerem query_string
    let (status, location) = post_form(&app, "/saved-filters", "name=Test&query_string=").await;

    // Dann: Redirect mit Fehler
    assert_eq!(status, StatusCode::SEE_OTHER);
    let loc = location.unwrap_or_default();
    assert!(
        loc.contains("save_error"),
        "Redirect-URL sollte save_error enthalten, war: {}",
        loc
    );
}

#[tokio::test]
async fn nicht_vorhandener_filter_loeschen_gibt_404() {
    // Gegeben: App im Basiszustand, kein Filter mit ID 999
    let (app, _temp) = setup_test_app().await;

    // Wenn: POST /saved-filters/999/delete
    let (status, _) = post_form(&app, "/saved-filters/999/delete", "").await;

    // Dann: 404-Antwort
    assert_eq!(status, StatusCode::NOT_FOUND);
}
