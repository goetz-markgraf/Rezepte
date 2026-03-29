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

/// Berechnet ein relatives Datum als deutschen Datumsstring (T.M.JJJJ).
fn date_in_days(n: i64) -> String {
    let d = time::OffsetDateTime::now_utc().date() + time::Duration::days(n);
    format!("{}.{}.{}", d.day(), d.month() as u8, d.year())
}

#[tokio::test]
async fn next_seven_days_filter_returns_200_with_recipes_in_window() {
    // Given: Rezept mit Datum morgen
    let (app, _temp) = setup_test_app().await;
    let tomorrow = date_in_days(1);
    create_recipe_with_date(&app, "Morgen-Rezept", &["Mittagessen"], Some(&tomorrow)).await;

    // When: GET /?filter=naechste-7-tage
    let (status, body) = get_body(app, "/?filter=naechste-7-tage").await;

    // Then: HTTP 200 und Rezept im Body
    assert_eq!(status, StatusCode::OK);
    assert!(
        body.contains("Morgen-Rezept"),
        "Morgen-Rezept sollte sichtbar sein"
    );
}

#[tokio::test]
async fn next_seven_days_filter_excludes_past_dates() {
    // Given: Rezept mit Vergangenheitsdatum, Rezept mit Zukunftsdatum (morgen)
    let (app, _temp) = setup_test_app().await;
    let yesterday = date_in_days(-1);
    let tomorrow = date_in_days(1);
    create_recipe_with_date(&app, "Vergangenes", &["Mittagessen"], Some(&yesterday)).await;
    create_recipe_with_date(&app, "Morgen-Gericht", &["Mittagessen"], Some(&tomorrow)).await;

    // When: GET /?filter=naechste-7-tage
    let (_status, body) = get_body(app, "/?filter=naechste-7-tage").await;

    // Then: Morgen-Gericht sichtbar, Vergangenes nicht
    assert!(
        body.contains("Morgen-Gericht"),
        "Morgen-Gericht sollte sichtbar sein"
    );
    assert!(
        !body.contains("Vergangenes"),
        "Vergangenes (Vergangenheitsdatum) sollte nicht sichtbar sein"
    );
}

#[tokio::test]
async fn next_seven_days_filter_excludes_null_dates() {
    // Given: Rezept ohne Datum, Rezept mit Datum morgen
    let (app, _temp) = setup_test_app().await;
    let tomorrow = date_in_days(1);
    create_recipe_with_date(&app, "Ohne-Datum", &["Mittagessen"], None).await;
    create_recipe_with_date(&app, "Mit-Datum", &["Mittagessen"], Some(&tomorrow)).await;

    // When: GET /?filter=naechste-7-tage
    let (_status, body) = get_body(app, "/?filter=naechste-7-tage").await;

    // Then: Mit-Datum sichtbar, Ohne-Datum nicht
    assert!(body.contains("Mit-Datum"), "Mit-Datum sollte sichtbar sein");
    assert!(
        !body.contains("Ohne-Datum"),
        "Ohne-Datum (kein Datum) sollte nicht sichtbar sein"
    );
}

#[tokio::test]
async fn next_seven_days_filter_excludes_dates_beyond_seven_days() {
    // Given: Rezept in 8 Tagen, Rezept morgen
    let (app, _temp) = setup_test_app().await;
    let day_eight = date_in_days(8);
    let tomorrow = date_in_days(1);
    create_recipe_with_date(&app, "Tag-8-Rezept", &["Mittagessen"], Some(&day_eight)).await;
    create_recipe_with_date(&app, "Morgen-Rezept", &["Mittagessen"], Some(&tomorrow)).await;

    // When: GET /?filter=naechste-7-tage
    let (_status, body) = get_body(app, "/?filter=naechste-7-tage").await;

    // Then: Morgen-Rezept sichtbar, Tag-8-Rezept nicht
    assert!(
        body.contains("Morgen-Rezept"),
        "Morgen-Rezept sollte sichtbar sein"
    );
    assert!(
        !body.contains("Tag-8-Rezept"),
        "Tag-8-Rezept (außerhalb des Fensters) sollte nicht sichtbar sein"
    );
}

#[tokio::test]
async fn next_seven_days_filter_includes_today() {
    // Given: Rezept mit heutigem Datum
    let (app, _temp) = setup_test_app().await;
    let today = date_in_days(0);
    create_recipe_with_date(&app, "Heute-Rezept", &["Mittagessen"], Some(&today)).await;

    // When: GET /?filter=naechste-7-tage
    let (_status, body) = get_body(app, "/?filter=naechste-7-tage").await;

    // Then: Heute-Rezept sichtbar (inklusive Grenze)
    assert!(
        body.contains("Heute-Rezept"),
        "Heute-Rezept sollte sichtbar sein (heutiges Datum ist inklusive)"
    );
}

#[tokio::test]
async fn next_seven_days_filter_shows_empty_state_message() {
    // Given: Alle Rezepte außerhalb des 7-Tage-Fensters
    let (app, _temp) = setup_test_app().await;
    create_recipe_with_date(&app, "Weit-Weg-Rezept", &["Mittagessen"], Some("1.1.2099")).await;

    // When: GET /?filter=naechste-7-tage
    let (_status, body) = get_body(app, "/?filter=naechste-7-tage").await;

    // Then: Hinweistext erscheint, Rezept nicht sichtbar
    assert!(
        body.contains("nächsten 7 Tage"),
        "Hinweistext für nächste 7 Tage sollte erscheinen"
    );
    assert!(
        !body.contains("Weit-Weg-Rezept"),
        "Weit-Weg-Rezept sollte nicht sichtbar sein"
    );
}

#[tokio::test]
async fn next_seven_days_filter_combined_with_category() {
    // Given: Brot-Rezept und Mittagessen-Rezept, beide im Fenster
    let (app, _temp) = setup_test_app().await;
    let day_two = date_in_days(2);
    let day_three = date_in_days(3);
    create_recipe_with_date(&app, "Dinkelbrot", &["Brot"], Some(&day_two)).await;
    create_recipe_with_date(&app, "Spaghetti", &["Mittagessen"], Some(&day_three)).await;

    // When: GET /?filter=naechste-7-tage&kategorie=Brot
    let (_status, body) = get_body(app, "/?filter=naechste-7-tage&kategorie=Brot").await;

    // Then: Nur Dinkelbrot sichtbar, Spaghetti nicht
    assert!(
        body.contains("Dinkelbrot"),
        "Dinkelbrot sollte sichtbar sein"
    );
    assert!(
        !body.contains("Spaghetti"),
        "Spaghetti (Mittagessen) sollte nicht sichtbar sein"
    );
}

#[tokio::test]
async fn next_seven_days_filter_combined_with_search() {
    // Given: Zwei Rezepte im Fenster
    let (app, _temp) = setup_test_app().await;
    let day_two = date_in_days(2);
    let day_three = date_in_days(3);
    create_recipe_with_date(&app, "Dinkelbrot", &["Brot"], Some(&day_two)).await;
    create_recipe_with_date(&app, "Roggenbrot", &["Brot"], Some(&day_three)).await;

    // When: GET /?filter=naechste-7-tage&q=dinkel
    let (_status, body) = get_body(app, "/?filter=naechste-7-tage&q=dinkel").await;

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
async fn deeplink_next_seven_days_filter_returns_correct_state() {
    // Given: Rezept mit Datum in 2 Tagen vorhanden
    let (app, _temp) = setup_test_app().await;
    let day_two = date_in_days(2);
    create_recipe_with_date(&app, "Planungs-Rezept", &["Snacks"], Some(&day_two)).await;

    // When: URL direkt mit ?filter=naechste-7-tage aufgerufen
    let (status, body) = get_body(app, "/?filter=naechste-7-tage").await;

    // Then: HTTP 200, Filter-Button als aktiv markiert (aria-pressed="true"), Rezept sichtbar
    assert_eq!(status, StatusCode::OK);
    assert!(
        body.contains("aria-pressed=\"true\""),
        "Filter-Button sollte aria-pressed='true' haben"
    );
    assert!(
        body.contains("Planungs-Rezept"),
        "Planungs-Rezept sollte sichtbar sein"
    );
}

#[tokio::test]
async fn no_filter_param_returns_alphabetical_list() {
    // Given: Zwei Rezepte mit zukünftigen Daten
    let (app, _temp) = setup_test_app().await;
    let day_two = date_in_days(2);
    let day_three = date_in_days(3);
    create_recipe_with_date(&app, "Zupfbrot", &["Brot"], Some(&day_two)).await;
    create_recipe_with_date(&app, "Apfelkuchen", &["Kuchen"], Some(&day_three)).await;

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
