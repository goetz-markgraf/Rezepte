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

async fn create_recipe_with_date(
    app: &axum::Router,
    title: &str,
    categories: &[&str],
    planned_date: Option<&str>,
) {
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

    app.clone().oneshot(request).await.unwrap();
}

#[tokio::test]
async fn not_made_filter_returns_200_with_correct_recipes() {
    // Given: Rezept mit Vergangenheitsdatum
    let (app, _temp) = setup_test_app().await;
    create_recipe_with_date(&app, "Linseneintopf", &["Mittagessen"], Some("1.1.2020")).await;

    // When: GET /?filter=laenger-nicht-gemacht
    let (status, body) = get_body(app, "/?filter=laenger-nicht-gemacht").await;

    // Then: HTTP 200 und Rezept im Body
    assert_eq!(status, StatusCode::OK);
    assert!(
        body.contains("Linseneintopf"),
        "Linseneintopf sollte sichtbar sein"
    );
}

#[tokio::test]
async fn not_made_filter_excludes_future_dated_recipes() {
    // Given: Rezept mit Zukunftsdatum und Rezept mit Vergangenheitsdatum
    let (app, _temp) = setup_test_app().await;
    create_recipe_with_date(&app, "Sonntagsbraten", &["Mittagessen"], Some("1.1.2099")).await;
    create_recipe_with_date(&app, "Linseneintopf", &["Mittagessen"], Some("1.1.2020")).await;

    // When: GET /?filter=laenger-nicht-gemacht
    let (_status, body) = get_body(app, "/?filter=laenger-nicht-gemacht").await;

    // Then: Linseneintopf sichtbar, Sonntagsbraten nicht
    assert!(
        body.contains("Linseneintopf"),
        "Linseneintopf sollte sichtbar sein"
    );
    assert!(
        !body.contains("Sonntagsbraten"),
        "Sonntagsbraten (Zukunftsdatum) sollte nicht sichtbar sein"
    );
}

#[tokio::test]
async fn not_made_filter_shows_null_date_recipes_first() {
    // Given: Rezept ohne Datum, Rezept mit Vergangenheitsdatum
    let (app, _temp) = setup_test_app().await;
    create_recipe_with_date(&app, "Mit Datum", &["Mittagessen"], Some("1.1.2020")).await;
    create_recipe_with_date(&app, "Ohne Datum", &["Mittagessen"], None).await;

    // When: GET /?filter=laenger-nicht-gemacht
    let (_status, body) = get_body(app, "/?filter=laenger-nicht-gemacht").await;

    // Then: "Ohne Datum" erscheint vor "Mit Datum" im HTML
    let pos_ohne = body
        .find("Ohne Datum")
        .expect("Ohne Datum sollte im Body sein");
    let pos_mit = body
        .find("Mit Datum")
        .expect("Mit Datum sollte im Body sein");
    assert!(
        pos_ohne < pos_mit,
        "Rezept ohne Datum sollte vor Rezept mit Datum erscheinen"
    );
}

#[tokio::test]
async fn not_made_filter_shows_empty_state_message() {
    // Given: Alle Rezepte haben Zukunftsdatum
    let (app, _temp) = setup_test_app().await;
    create_recipe_with_date(
        &app,
        "Geplantes Gericht",
        &["Mittagessen"],
        Some("1.1.2099"),
    )
    .await;

    // When: GET /?filter=laenger-nicht-gemacht
    let (_status, body) = get_body(app, "/?filter=laenger-nicht-gemacht").await;

    // Then: Hinweistext erscheint, Rezept nicht sichtbar
    assert!(
        body.contains("Keine Rezepte ohne zukünftiges Datum gefunden"),
        "Hinweistext sollte erscheinen"
    );
    assert!(
        !body.contains("Geplantes Gericht"),
        "Geplantes Gericht sollte nicht sichtbar sein"
    );
}

#[tokio::test]
async fn not_made_filter_combined_with_category() {
    // Given: Brot-Rezept mit Vergangenheitsdatum, Mittagessen-Rezept mit Vergangenheitsdatum
    let (app, _temp) = setup_test_app().await;
    create_recipe_with_date(&app, "Dinkelbrot", &["Brot"], Some("1.1.2025")).await;
    create_recipe_with_date(&app, "Spaghetti", &["Mittagessen"], Some("1.1.2020")).await;

    // When: GET /?filter=laenger-nicht-gemacht&kategorie=Brot
    let (_status, body) = get_body(app, "/?filter=laenger-nicht-gemacht&kategorie=Brot").await;

    // Then: Nur Dinkelbrot sichtbar, Spaghetti nicht
    assert!(
        body.contains("Dinkelbrot"),
        "Dinkelbrot sollte sichtbar sein"
    );
    assert!(
        !body.contains("Spaghetti"),
        "Spaghetti sollte nicht sichtbar sein (falsche Kategorie)"
    );
}

#[tokio::test]
async fn not_made_filter_combined_with_search() {
    // Given: Zwei Brot-Rezepte mit Vergangenheitsdaten
    let (app, _temp) = setup_test_app().await;
    create_recipe_with_date(&app, "Dinkelbrot", &["Brot"], Some("1.1.2020")).await;
    create_recipe_with_date(&app, "Roggenbrot", &["Brot"], Some("1.6.2022")).await;

    // When: GET /?filter=laenger-nicht-gemacht&q=dinkel
    let (_status, body) = get_body(app, "/?filter=laenger-nicht-gemacht&q=dinkel").await;

    // Then: Nur Dinkelbrot sichtbar, Roggenbrot nicht
    assert!(
        body.contains("Dinkelbrot"),
        "Dinkelbrot sollte sichtbar sein"
    );
    assert!(
        !body.contains("Roggenbrot"),
        "Roggenbrot sollte nicht sichtbar sein"
    );
}

#[tokio::test]
async fn deeplink_not_made_filter_returns_correct_state() {
    // Given: Rezept mit Vergangenheitsdatum
    let (app, _temp) = setup_test_app().await;
    create_recipe_with_date(&app, "Pfannkuchen", &["Snacks"], Some("1.1.2020")).await;

    // When: URL direkt mit ?filter=laenger-nicht-gemacht aufgerufen
    let (_status, body) = get_body(app, "/?filter=laenger-nicht-gemacht").await;

    // Then: Filter-Button ist als aktiv markiert (aria-pressed="true") und Rezept sichtbar
    assert!(
        body.contains("aria-pressed=\"true\""),
        "Filter-Button sollte aria-pressed='true' haben"
    );
    assert!(
        body.contains("Pfannkuchen"),
        "Pfannkuchen sollte sichtbar sein"
    );
}

#[tokio::test]
async fn no_filter_param_returns_alphabetical_list() {
    // Given: Zwei Rezepte mit verschiedenen Daten
    let (app, _temp) = setup_test_app().await;
    create_recipe_with_date(&app, "Zupfbrot", &["Brot"], Some("1.1.2020")).await;
    create_recipe_with_date(&app, "Apfelkuchen", &["Kuchen"], Some("1.6.2022")).await;

    // When: GET / (ohne filter-Parameter)
    let (_status, body) = get_body(app, "/").await;

    // Then: Beide Rezepte sichtbar, Apfelkuchen vor Zupfbrot (alphabetisch)
    assert!(body.contains("Zupfbrot"), "Zupfbrot sollte sichtbar sein");
    assert!(
        body.contains("Apfelkuchen"),
        "Apfelkuchen sollte sichtbar sein"
    );

    // Alphabetische Reihenfolge: Apfelkuchen zuerst
    let pos_apfel = body
        .find("Apfelkuchen")
        .expect("Apfelkuchen sollte im Body sein");
    let pos_zupf = body.find("Zupfbrot").expect("Zupfbrot sollte im Body sein");
    assert!(
        pos_apfel < pos_zupf,
        "Apfelkuchen sollte vor Zupfbrot erscheinen (alphabetisch)"
    );
}
