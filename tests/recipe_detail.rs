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

// Tests für format_date
#[test]
fn format_date_formats_correctly() {
    use rezepte::routes::recipes::format_date;
    assert_eq!(format_date("2026-03-27 10:45:00"), "27.03.2026");
    assert_eq!(format_date("2026-01-01"), "01.01.2026");
    assert_eq!(format_date("2026-12-31 23:59:59"), "31.12.2026");
}

#[test]
fn format_date_handles_invalid_input() {
    use rezepte::routes::recipes::format_date;
    let input = "ungültiges-datum";
    assert_eq!(format_date(input), input);
    let input2 = "";
    assert_eq!(format_date(input2), input2);
}

// Handler-Tests
#[tokio::test]
async fn show_recipe_displays_title() {
    let (app, _temp) = setup_test_app().await;
    let id = create_test_recipe(&app, "title=Schnitzel+mit+Pommes&categories=Mittagessen").await;

    let (status, body) = get_body(app, &format!("/recipes/{}", id)).await;

    assert_eq!(status, StatusCode::OK);
    assert!(body.contains("Schnitzel mit Pommes"));
}

#[tokio::test]
async fn show_recipe_displays_ingredients_section() {
    let (app, _temp) = setup_test_app().await;
    let id = create_test_recipe(
        &app,
        "title=Testrezept&categories=Mittagessen&ingredients=Mehl%2C+Eier",
    )
    .await;

    let (status, body) = get_body(app, &format!("/recipes/{}", id)).await;

    assert_eq!(status, StatusCode::OK);
    assert!(body.contains("Zutaten"));
    assert!(body.contains("Mehl"));
}

#[tokio::test]
async fn show_recipe_hides_ingredients_when_empty() {
    let (app, _temp) = setup_test_app().await;
    let id = create_test_recipe(&app, "title=Ohnerezept&categories=Brot").await;

    let (status, body) = get_body(app, &format!("/recipes/{}", id)).await;

    assert_eq!(status, StatusCode::OK);
    assert!(!body.contains(r#"class="ingredients""#));
}

#[tokio::test]
async fn show_recipe_hides_instructions_when_empty() {
    let (app, _temp) = setup_test_app().await;
    let id = create_test_recipe(&app, "title=Ohnerezept&categories=Brot").await;

    let (status, body) = get_body(app, &format!("/recipes/{}", id)).await;

    assert_eq!(status, StatusCode::OK);
    assert!(!body.contains(r#"class="instructions""#));
}

#[tokio::test]
async fn show_recipe_returns_404_for_missing_id() {
    let (app, _temp) = setup_test_app().await;

    let (status, _body) = get_body(app, "/recipes/99999").await;

    assert_eq!(status, StatusCode::NOT_FOUND);
}

#[tokio::test]
async fn show_recipe_404_contains_back_link() {
    let (app, _temp) = setup_test_app().await;

    let (status, body) = get_body(app, "/recipes/99999").await;

    assert_eq!(status, StatusCode::NOT_FOUND);
    assert!(
        body.contains("Zurück zur Übersicht"),
        "404-Seite sollte einen Link zurück zur Übersicht enthalten"
    );
    assert!(
        body.contains("href=\"/\""),
        "404-Seite sollte einen Link zu / enthalten"
    );
}

#[tokio::test]
async fn show_recipe_displays_success_flash() {
    let (app, _temp) = setup_test_app().await;
    let id = create_test_recipe(&app, "title=Testrezept&categories=Mittagessen").await;

    let (status, body) = get_body(app, &format!("/recipes/{}?success=1", id)).await;

    assert_eq!(status, StatusCode::OK);
    assert!(
        body.contains("success"),
        "Seite sollte die .success-Klasse enthalten"
    );
    assert!(body.contains("Rezept erfolgreich aktualisiert"));
}

#[tokio::test]
async fn show_recipe_no_success_flash_without_param() {
    let (app, _temp) = setup_test_app().await;
    let id = create_test_recipe(&app, "title=Testrezept&categories=Mittagessen").await;

    let (_status, body) = get_body(app, &format!("/recipes/{}", id)).await;

    assert!(
        !body.contains("Rezept erfolgreich aktualisiert"),
        "Seite sollte ohne ?success=1 keinen Flash-Banner zeigen"
    );
}

#[tokio::test]
async fn show_recipe_displays_edit_link() {
    let (app, _temp) = setup_test_app().await;
    let id = create_test_recipe(&app, "title=Testrezept&categories=Mittagessen").await;

    let (_status, body) = get_body(app, &format!("/recipes/{}", id)).await;

    assert!(
        body.contains(&format!("/recipes/{}/edit", id)),
        "Seite sollte einen Link zum Bearbeiten enthalten"
    );
}

#[tokio::test]
async fn show_recipe_displays_delete_link() {
    let (app, _temp) = setup_test_app().await;
    let id = create_test_recipe(&app, "title=Testrezept&categories=Mittagessen").await;

    let (_status, body) = get_body(app, &format!("/recipes/{}", id)).await;

    assert!(
        body.contains(&format!("/recipes/{}/confirm-delete", id)),
        "Seite sollte einen Link zum Löschen enthalten"
    );
}

#[tokio::test]
async fn show_recipe_displays_formatted_date() {
    let (app, _temp) = setup_test_app().await;
    let id = create_test_recipe(&app, "title=Datumstest&categories=Mittagessen").await;

    let (_status, body) = get_body(app, &format!("/recipes/{}", id)).await;

    // The date should be in German format (DD.MM.YYYY), not raw SQLite format (YYYY-MM-DD ...)
    // Check that the page contains a date in DD.MM.YYYY format
    let has_german_date = body
        .split_whitespace()
        .any(|word| word.chars().filter(|&c| c == '.').count() == 2);
    assert!(
        has_german_date,
        "Datum sollte im deutschen Format (TT.MM.JJJJ) angezeigt werden"
    );
}
