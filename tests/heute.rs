/// Integrationstests für Story 20: "Heute gekocht"-Ansicht
///
/// Testet GET /heute und POST /heute/recipes/:id/rating mit verschiedenen Szenarien.
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

/// Erstellt ein Rezept mit optionalem Datum (deutsches Format T.M.JJJJ)
/// und gibt die ID des erstellten Rezepts zurück.
async fn create_recipe_with_date(
    app: &axum::Router,
    title: &str,
    categories: &[&str],
    planned_date: Option<&str>,
) -> i64 {
    let mut form_data = format!("title={}", urlencoding::encode(title));
    for cat in categories {
        form_data.push_str(&format!("&categories={}", urlencoding::encode(cat)));
    }
    if let Some(date) = planned_date {
        form_data.push_str(&format!("&planned_date={}", urlencoding::encode(date)));
    }

    let request = Request::builder()
        .method("POST")
        .uri("/recipes")
        .header("Content-Type", "application/x-www-form-urlencoded")
        .body(Body::from(form_data))
        .unwrap();

    let response = app.clone().oneshot(request).await.unwrap();
    // Die Redirect-URL enthält die ID: /recipes/42
    let location = response
        .headers()
        .get("location")
        .and_then(|v| v.to_str().ok())
        .unwrap_or("/recipes/0");
    let id: i64 = location
        .trim_start_matches("/recipes/")
        .split('?')
        .next()
        .unwrap_or("0")
        .parse()
        .unwrap_or(0);
    id
}

/// Berechnet ein relatives Datum als deutschen Datumsstring (T.M.JJJJ).
fn date_in_days(n: i64) -> String {
    let d = time::OffsetDateTime::now_utc().date() + time::Duration::days(n);
    format!("{}.{}.{}", d.day(), d.month() as u8, d.year())
}

async fn post_rating(app: axum::Router, id: i64, rating: &str) -> (StatusCode, String) {
    let form_data = format!("rating={}", rating);
    let request = Request::builder()
        .method("POST")
        .uri(format!("/heute/recipes/{}/rating", id))
        .header("Content-Type", "application/x-www-form-urlencoded")
        .body(Body::from(form_data))
        .unwrap();
    let response = app.oneshot(request).await.unwrap();
    let status = response.status();
    let body = response.into_body().collect().await.unwrap().to_bytes();
    let body_str = String::from_utf8(body.to_vec()).unwrap();
    (status, body_str)
}

#[tokio::test]
async fn heute_returns_200() {
    // Given: App ohne Rezepte
    let (app, _temp) = setup_test_app().await;

    // When: GET /heute
    let (status, _body) = get_body(app, "/heute").await;

    // Then: HTTP 200
    assert_eq!(status, StatusCode::OK);
}

#[tokio::test]
async fn heute_zeigt_mehrere_rezepte_fuer_heute() {
    // Given: Zwei Rezepte mit planned_date = heute
    let (app, _temp) = setup_test_app().await;
    let today_str = date_in_days(0);
    create_recipe_with_date(&app, "Spaghetti", &["Mittagessen"], Some(&today_str)).await;
    create_recipe_with_date(&app, "Salat", &["Snacks"], Some(&today_str)).await;

    // When: GET /heute
    let (status, body) = get_body(app, "/heute").await;

    // Then: Beide Rezepte im Body sichtbar
    assert_eq!(status, StatusCode::OK);
    assert!(
        body.contains("Spaghetti"),
        "Body sollte 'Spaghetti' enthalten"
    );
    assert!(body.contains("Salat"), "Body sollte 'Salat' enthalten");
}

#[tokio::test]
async fn heute_hat_link_zur_detailansicht() {
    // Given: Rezept mit planned_date = heute
    let (app, _temp) = setup_test_app().await;
    let today_str = date_in_days(0);
    let id =
        create_recipe_with_date(&app, "Link-Test-Rezept", &["Mittagessen"], Some(&today_str)).await;

    // When: GET /heute
    let (status, body) = get_body(app, "/heute").await;

    // Then: Body enthält /recipes/{id} Link
    assert_eq!(status, StatusCode::OK);
    assert!(
        body.contains(&format!("/recipes/{}", id)),
        "Body sollte Link /recipes/{} enthalten",
        id
    );
}

#[tokio::test]
async fn heute_hat_css_klasse_tagesabschnitt_heute() {
    // Given: App
    let (app, _temp) = setup_test_app().await;

    // When: GET /heute
    let (status, body) = get_body(app, "/heute").await;

    // Then: Body enthält CSS-Klasse "tagesabschnitt-heute"
    assert_eq!(status, StatusCode::OK);
    assert!(
        body.contains("tagesabschnitt-heute"),
        "Body sollte CSS-Klasse 'tagesabschnitt-heute' enthalten"
    );
}
