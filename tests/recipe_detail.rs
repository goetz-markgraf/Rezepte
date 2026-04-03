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
    // Given: Gültige SQLite-Datumsstrings
    // When: format_date aufgerufen wird
    // Then: Datum wird im deutschen Format (TT.MM.JJJJ) zurückgegeben
    use rezepte::routes::recipes::format_date;
    assert_eq!(format_date("2026-03-27 10:45:00"), "27.03.2026");
    assert_eq!(format_date("2026-01-01"), "01.01.2026");
    assert_eq!(format_date("2026-12-31 23:59:59"), "31.12.2026");
}

#[test]
fn format_date_handles_invalid_input() {
    // Given: Ungültige oder leere Datumsstrings
    // When: format_date aufgerufen wird
    // Then: Der Eingabewert wird unverändert zurückgegeben
    use rezepte::routes::recipes::format_date;
    let input = "ungültiges-datum";
    assert_eq!(format_date(input), input);
    let input2 = "";
    assert_eq!(format_date(input2), input2);
}

// Handler-Tests
#[tokio::test]
async fn show_recipe_displays_title() {
    // Given: Ein Rezept mit dem Titel "Schnitzel mit Pommes" wurde erstellt
    let (app, _temp) = setup_test_app().await;
    let id = create_test_recipe(&app, "title=Schnitzel+mit+Pommes&categories=Mittagessen").await;

    // When: GET /recipes/{id} aufgerufen wird
    let (status, body) = get_body(app, &format!("/recipes/{}", id)).await;

    // Then: HTTP 200, Titel im Body enthalten
    assert_eq!(status, StatusCode::OK);
    assert!(body.contains("Schnitzel mit Pommes"));
}

#[tokio::test]
async fn show_recipe_displays_ingredients_section() {
    // Given: Ein Rezept mit Zutaten wurde erstellt
    let (app, _temp) = setup_test_app().await;
    let id = create_test_recipe(
        &app,
        "title=Testrezept&categories=Mittagessen&ingredients=Mehl%2C+Eier",
    )
    .await;

    // When: GET /recipes/{id} aufgerufen wird
    let (status, body) = get_body(app, &format!("/recipes/{}", id)).await;

    // Then: HTTP 200, Zutaten-Sektion und Zutat im Body enthalten
    assert_eq!(status, StatusCode::OK);
    assert!(body.contains("Zutaten"));
    assert!(body.contains("Mehl"));
}

#[tokio::test]
async fn show_recipe_hides_ingredients_when_empty() {
    // Given: Ein Rezept ohne Zutaten wurde erstellt
    let (app, _temp) = setup_test_app().await;
    let id = create_test_recipe(&app, "title=Ohnerezept&categories=Brot").await;

    // When: GET /recipes/{id} aufgerufen wird
    let (status, body) = get_body(app, &format!("/recipes/{}", id)).await;

    // Then: HTTP 200, Zutaten-Sektion wird nicht gerendert
    assert_eq!(status, StatusCode::OK);
    assert!(!body.contains(r#"class="ingredients""#));
}

#[tokio::test]
async fn show_recipe_hides_instructions_when_empty() {
    // Given: Ein Rezept ohne Anleitung wurde erstellt
    let (app, _temp) = setup_test_app().await;
    let id = create_test_recipe(&app, "title=Ohnerezept&categories=Brot").await;

    // When: GET /recipes/{id} aufgerufen wird
    let (status, body) = get_body(app, &format!("/recipes/{}", id)).await;

    // Then: HTTP 200, Anleitungs-Sektion wird nicht gerendert
    assert_eq!(status, StatusCode::OK);
    assert!(!body.contains(r#"class="instructions""#));
}

#[tokio::test]
async fn show_recipe_returns_404_for_missing_id() {
    // Given: ID 99999 existiert nicht
    let (app, _temp) = setup_test_app().await;

    // When: GET /recipes/99999 aufgerufen wird
    let (status, _body) = get_body(app, "/recipes/99999").await;

    // Then: HTTP 404
    assert_eq!(status, StatusCode::NOT_FOUND);
}

#[tokio::test]
async fn show_recipe_404_contains_back_link() {
    // Given: ID 99999 existiert nicht
    let (app, _temp) = setup_test_app().await;

    // When: GET /recipes/99999 aufgerufen wird
    let (status, body) = get_body(app, "/recipes/99999").await;

    // Then: HTTP 404, Seite enthält Link zurück zur Übersicht
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
    // Given: Ein Rezept wurde erstellt
    let (app, _temp) = setup_test_app().await;
    let id = create_test_recipe(&app, "title=Testrezept&categories=Mittagessen").await;

    // When: GET /recipes/{id}?success=1 aufgerufen wird
    let (status, body) = get_body(app, &format!("/recipes/{}?success=1", id)).await;

    // Then: HTTP 200, Erfolgsmeldung "Rezept erfolgreich aktualisiert" sichtbar
    assert_eq!(status, StatusCode::OK);
    assert!(
        body.contains("success"),
        "Seite sollte die .success-Klasse enthalten"
    );
    assert!(body.contains("Rezept erfolgreich aktualisiert"));
}

#[tokio::test]
async fn show_recipe_no_success_flash_without_param() {
    // Given: Ein Rezept wurde erstellt
    let (app, _temp) = setup_test_app().await;
    let id = create_test_recipe(&app, "title=Testrezept&categories=Mittagessen").await;

    // When: GET /recipes/{id} ohne ?success=1 aufgerufen wird
    let (_status, body) = get_body(app, &format!("/recipes/{}", id)).await;

    // Then: Kein Erfolgs-Flash-Banner in der Seite
    assert!(
        !body.contains("Rezept erfolgreich aktualisiert"),
        "Seite sollte ohne ?success=1 keinen Flash-Banner zeigen"
    );
}

#[tokio::test]
async fn show_recipe_displays_edit_link() {
    // Given: Ein Rezept wurde erstellt
    let (app, _temp) = setup_test_app().await;
    let id = create_test_recipe(&app, "title=Testrezept&categories=Mittagessen").await;

    // When: GET /recipes/{id} aufgerufen wird
    let (_status, body) = get_body(app, &format!("/recipes/{}", id)).await;

    // Then: Seite enthält den Link zur Bearbeitungsseite
    assert!(
        body.contains(&format!("/recipes/{}/edit", id)),
        "Seite sollte einen Link zum Bearbeiten enthalten"
    );
}

#[tokio::test]
async fn show_recipe_displays_delete_link() {
    // Given: Ein Rezept wurde erstellt
    let (app, _temp) = setup_test_app().await;
    let id = create_test_recipe(&app, "title=Testrezept&categories=Mittagessen").await;

    // When: GET /recipes/{id} aufgerufen wird
    let (_status, body) = get_body(app, &format!("/recipes/{}", id)).await;

    // Then: Seite enthält den Link zur Lösch-Bestätigungsseite
    assert!(
        body.contains(&format!("/recipes/{}/confirm-delete", id)),
        "Seite sollte einen Link zum Löschen enthalten"
    );
}

#[tokio::test]
async fn show_recipe_displays_formatted_date() {
    // Given: Ein Rezept wurde erstellt (Datum wird automatisch gesetzt)
    let (app, _temp) = setup_test_app().await;
    let id = create_test_recipe(&app, "title=Datumstest&categories=Mittagessen").await;

    // When: GET /recipes/{id} aufgerufen wird
    let (_status, body) = get_body(app, &format!("/recipes/{}", id)).await;

    // Then: Datum wird im deutschen Format (TT.MM.JJJJ) angezeigt, nicht im SQLite-Rohformat
    let has_german_date = body
        .split_whitespace()
        .any(|word| word.chars().filter(|&c| c == '.').count() == 2);
    assert!(
        has_german_date,
        "Datum sollte im deutschen Format (TT.MM.JJJJ) angezeigt werden"
    );
}

// === Story 36: Markdown-Rendering ===

#[tokio::test]
async fn show_recipe_renders_ingredient_list_as_ul() {
    // Given: Ein Rezept mit Zutaten als Aufzählungsliste wurde erstellt
    let (app, _temp) = setup_test_app().await;
    let id = create_test_recipe(
        &app,
        "title=Markdown-Test&categories=Mittagessen&ingredients=-%20500g%20Mehl%0A-%201%20Ei%0A-%20250ml%20Milch",
    )
    .await;

    // When: GET /recipes/{id} aufgerufen wird
    let (status, body) = get_body(app, &format!("/recipes/{}", id)).await;

    // Then: HTTP 200, HTML enthält <ul> statt rohem "- " Text
    assert_eq!(status, StatusCode::OK);
    assert!(
        body.contains("<ul>"),
        "Zutaten-Liste sollte als <ul> gerendert werden"
    );
    assert!(
        body.contains("<li>"),
        "Zutaten-Punkte sollten als <li> gerendert werden"
    );
    assert!(
        !body.contains("- 500g Mehl"),
        "Markdown-Syntax '- ' sollte nicht als Rohtext sichtbar sein"
    );
}

#[tokio::test]
async fn show_recipe_renders_numbered_instructions_as_ol() {
    // Given: Ein Rezept mit nummerierter Zubereitungsliste
    let (app, _temp) = setup_test_app().await;
    let id = create_test_recipe(
        &app,
        "title=Nummeriert&categories=Mittagessen&instructions=1.%20Ofen%20vorheizen%0A2.%20Teig%20kneten",
    )
    .await;

    // When: GET /recipes/{id} aufgerufen wird
    let (status, body) = get_body(app, &format!("/recipes/{}", id)).await;

    // Then: HTTP 200, HTML enthält <ol>
    assert_eq!(status, StatusCode::OK);
    assert!(
        body.contains("<ol>"),
        "Nummerierte Liste sollte als <ol> gerendert werden"
    );
}

#[tokio::test]
async fn show_recipe_renders_bold_text_as_strong() {
    // Given: Ein Rezept mit Fettschrift in der Zubereitung
    let (app, _temp) = setup_test_app().await;
    let id = create_test_recipe(
        &app,
        "title=Fettschrift-Test&categories=Mittagessen&instructions=**Wichtig%3A**%20Ofen%20vorheizen",
    )
    .await;

    // When: GET /recipes/{id} aufgerufen wird
    let (status, body) = get_body(app, &format!("/recipes/{}", id)).await;

    // Then: HTTP 200, HTML enthält <strong>
    assert_eq!(status, StatusCode::OK);
    assert!(
        body.contains("<strong>"),
        "Fettschrift sollte als <strong> gerendert werden"
    );
    assert!(
        !body.contains("**Wichtig"),
        "Markdown-Syntax '**' sollte nicht als Rohtext sichtbar sein"
    );
}

#[tokio::test]
async fn show_recipe_xss_script_tag_in_ingredients_is_sanitized() {
    // Given: Ein Rezept mit Script-Tag in den Zutaten (XSS-Versuch)
    let (app, _temp) = setup_test_app().await;
    let id = create_test_recipe(
        &app,
        "title=XSS-Test&categories=Mittagessen&ingredients=%3Cscript%3Ealert(1)%3C%2Fscript%3E",
    )
    .await;

    // When: GET /recipes/{id} aufgerufen wird
    let (status, body) = get_body(app, &format!("/recipes/{}", id)).await;

    // Then: HTTP 200, <script>-Tag ist nicht im HTML-Output
    assert_eq!(status, StatusCode::OK);
    assert!(
        !body.contains("<script>"),
        "XSS: <script>-Tag darf nicht im gerenderten HTML sein"
    );
}

#[tokio::test]
async fn show_recipe_whitespace_only_ingredients_hides_section() {
    // Given: Ein Rezept mit nur Leerzeichen in den Zutaten
    let (app, _temp) = setup_test_app().await;
    let id = create_test_recipe(&app, "title=Leertest&categories=Brot&ingredients=%20%20%20").await;

    // When: GET /recipes/{id} aufgerufen wird
    let (status, body) = get_body(app, &format!("/recipes/{}", id)).await;

    // Then: HTTP 200, Zutaten-Sektion wird nicht angezeigt (da nur Whitespace)
    assert_eq!(status, StatusCode::OK);
    assert!(
        !body.contains(r#"class="ingredients""#),
        "Zutaten-Sektion sollte bei reinem Whitespace ausgeblendet werden"
    );
}
