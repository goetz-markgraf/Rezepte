use crate::error::AppError;
use crate::models::recipe::validate_rating;
use crate::models::{
    create_recipe, create_saved_filter, delete_recipe, delete_saved_filter,
    filter_recipes_by_categories, filter_recipes_next_seven_days, filter_recipes_not_made_recently,
    find_similar_recipes, get_all_saved_filters, get_recipe_by_id, update_recipe,
    update_recipe_rating, CreateRecipe, CreateSavedFilter, Recipe, UpdateRecipe, VALID_CATEGORIES,
};
use crate::templates::{
    CategoryFilterItem, ConfirmDeleteTemplate, DuplicateHintTemplate, IndexTemplate,
    InlineRatingTemplate, NotFoundTemplate, RecipeDetailTemplate, RecipeFormTemplate,
    RecipeListItem, SavedFilterItem,
};
use askama::Template;
use axum::{
    extract::{Path, Query, RawQuery, State},
    http::{HeaderMap, StatusCode},
    response::{Html, IntoResponse, Redirect},
};
use serde::Deserialize;
use sqlx::SqlitePool;
use std::sync::Arc;

const GERMAN_MONTHS_LONG: &[&str] = &[
    "Januar",
    "Februar",
    "März",
    "April",
    "Mai",
    "Juni",
    "Juli",
    "August",
    "September",
    "Oktober",
    "November",
    "Dezember",
];

/// Formatiert ein `time::Date` in das lange deutsche Format: "5. März 2025".
fn format_planned_date_long(date: Option<time::Date>) -> Option<String> {
    date.map(|d| {
        let month_name = GERMAN_MONTHS_LONG[(d.month() as u8 - 1) as usize];
        format!("{}. {} {}", d.day(), month_name, d.year())
    })
}

/// Formatiert ein `time::Date` in das kompakte Format: "05.03.2025".
fn format_planned_date_short(date: Option<time::Date>) -> Option<String> {
    date.map(|d| format!("{:02}.{:02}.{}", d.day(), d.month() as u8, d.year()))
}

const GERMAN_WEEKDAYS: &[&str] = &["Mo", "Di", "Mi", "Do", "Fr", "Sa", "So"];

/// Formatiert ein `time::Date` mit Wochentag: "Mo, 31.03.2026".
fn format_planned_date_with_weekday(date: Option<time::Date>) -> Option<String> {
    date.map(|d| {
        let weekday_idx = d.weekday().number_days_from_monday() as usize;
        let weekday = GERMAN_WEEKDAYS[weekday_idx];
        format!(
            "{}, {:02}.{:02}.{}",
            weekday,
            d.day(),
            d.month() as u8,
            d.year()
        )
    })
}

/// Formatiert ein `time::Date` für die Texteingabe im deutschen Format: "5.3.2025".
fn format_planned_date_input(date: Option<time::Date>) -> String {
    date.map(|d| format!("{}.{}.{}", d.day(), d.month() as u8, d.year()))
        .unwrap_or_default()
}

#[derive(Deserialize)]
pub struct RecipeDetailQuery {
    pub success: Option<String>,
}

#[derive(Deserialize)]
pub struct IndexQuery {
    pub deleted: Option<String>,
    pub q: Option<String>,
    pub bewertung: Option<String>,
    pub save_error: Option<String>,
    pub save_name: Option<String>,
}

/// Extrahiert alle `kategorie`-Parameter aus dem Raw-Query-String.
/// Unterstützt Mehrfachwerte: `?kategorie=Brot&kategorie=Kuchen`.
fn extract_kategorie_params(raw_query: &str) -> Vec<String> {
    raw_query
        .split('&')
        .filter_map(|pair| {
            let (key, value) = pair.split_once('=')?;
            if key == "kategorie" {
                urlencoding::decode(value).ok().map(|v| v.into_owned())
            } else {
                None
            }
        })
        .collect()
}

/// Extrahiert den ersten `filter`-Parameter aus dem Raw-Query-String.
/// Bei mehrfach gesetztem `filter` (z.B. DeepLink-Konflikt) gewinnt der erste Wert.
fn extract_filter_param(raw_query: &str) -> Option<String> {
    raw_query.split('&').find_map(|pair| {
        let (key, value) = pair.split_once('=')?;
        if key == "filter" {
            urlencoding::decode(value).ok().map(|v| v.into_owned())
        } else {
            None
        }
    })
}

/// Formatiert einen SQLite-Timestamp (z.B. "2026-03-27 10:45:00") in ein deutsches Datumsformat ("27.03.2026").
/// Bei ungültiger Eingabe wird der ursprüngliche String zurückgegeben.
pub fn format_date(date_str: &str) -> String {
    let date_part = date_str.split_whitespace().next().unwrap_or(date_str);
    let parts: Vec<&str> = date_part.split('-').collect();
    if parts.len() == 3 {
        if let (Ok(_), Ok(_), Ok(_)) = (
            parts[0].parse::<u32>(),
            parts[1].parse::<u32>(),
            parts[2].parse::<u32>(),
        ) {
            return format!("{}.{}.{}", parts[2], parts[1], parts[0]);
        }
    }
    date_str.to_string()
}

/// Parst einen rohen Rating-String aus dem Formular. Leer oder fehlend → None.
fn parse_rating(raw: Option<&str>) -> Option<i32> {
    raw.filter(|s| !s.trim().is_empty())
        .and_then(|s| s.trim().parse::<i32>().ok())
}

fn render_template<T: askama::Template>(template: T) -> Result<String, AppError> {
    template
        .render()
        .map_err(|e: askama::Error| AppError::BadRequest(e.to_string()))
}

fn decode_form_value(value: &str) -> String {
    // Replace + with space first (URL form encoding), then decode %XX sequences
    let with_spaces = value.replace('+', " ");
    urlencoding::decode(&with_spaces)
        .unwrap_or_default()
        .to_string()
}

fn parse_form_data(body: &[u8]) -> std::collections::HashMap<String, Vec<String>> {
    let form_data = String::from_utf8_lossy(body);
    let mut params: std::collections::HashMap<String, Vec<String>> =
        std::collections::HashMap::new();

    for pair in form_data.split('&') {
        if let Some((key, value)) = pair.split_once('=') {
            let key = decode_form_value(key);
            let value = decode_form_value(value);
            params.entry(key).or_default().push(value);
        }
    }

    params
}

/// Baut die Toggle-URL für eine Kategorie: aktiv→entfernen, inaktiv→hinzufügen.
/// Bestehender Suchbegriff, `not_made_filter_active`, `next_seven_days_filter_active` und `bewertung` bleiben erhalten.
fn build_category_toggle_url(
    category: &str,
    is_active: bool,
    active_categories: &[String],
    search_query: &str,
    not_made_filter_active: bool,
    next_seven_days_filter_active: bool,
    bewertung: Option<&str>,
) -> String {
    let mut params: Vec<String> = Vec::new();

    if !search_query.is_empty() {
        params.push(format!("q={}", urlencoding::encode(search_query)));
    }

    for cat in active_categories {
        if is_active && cat == category {
            continue; // aktive Kategorie beim Klick entfernen
        }
        params.push(format!("kategorie={}", urlencoding::encode(cat)));
    }

    if !is_active {
        params.push(format!("kategorie={}", urlencoding::encode(category)));
    }

    if not_made_filter_active {
        params.push("filter=laenger-nicht-gemacht".to_string());
    } else if next_seven_days_filter_active {
        params.push("filter=naechste-7-tage".to_string());
    }

    if let Some(b) = bewertung {
        params.push(format!("bewertung={}", urlencoding::encode(b)));
    }

    if params.is_empty() {
        "/".to_string()
    } else {
        format!("/?{}", params.join("&"))
    }
}

/// Erstellt alle CategoryFilterItems für alle gültigen Kategorien.
fn build_category_filters(
    active_categories: &[String],
    search_query: &str,
    not_made_filter_active: bool,
    next_seven_days_filter_active: bool,
    bewertung: Option<&str>,
) -> Vec<CategoryFilterItem> {
    VALID_CATEGORIES
        .iter()
        .map(|&cat| {
            let is_active = active_categories.iter().any(|a| a == cat);
            let toggle_url = build_category_toggle_url(
                cat,
                is_active,
                active_categories,
                search_query,
                not_made_filter_active,
                next_seven_days_filter_active,
                bewertung,
            );
            CategoryFilterItem {
                name: cat.to_string(),
                is_active,
                toggle_url,
            }
        })
        .collect()
}

/// Erstellt die URL zum Zurücksetzen aller Kategorie-Filter (Suchbegriff bleibt erhalten).
/// Wenn `not_made_filter_active`, `next_seven_days_filter_active` oder `bewertung` gesetzt ist, bleiben diese erhalten.
fn build_reset_url(
    search_query: &str,
    not_made_filter_active: bool,
    next_seven_days_filter_active: bool,
    bewertung: Option<&str>,
) -> String {
    let mut params: Vec<String> = Vec::new();
    if !search_query.is_empty() {
        params.push(format!("q={}", urlencoding::encode(search_query)));
    }
    if not_made_filter_active {
        params.push("filter=laenger-nicht-gemacht".to_string());
    } else if next_seven_days_filter_active {
        params.push("filter=naechste-7-tage".to_string());
    }
    if let Some(b) = bewertung {
        params.push(format!("bewertung={}", urlencoding::encode(b)));
    }
    if params.is_empty() {
        "/".to_string()
    } else {
        format!("/?{}", params.join("&"))
    }
}

/// Baut die Toggle-URL für den "Länger nicht gemacht"-Filter.
/// Aktiv → URL ohne `filter`-Parameter (Kategorie + Suche + bewertung bleiben erhalten).
/// Inaktiv → URL mit `filter=laenger-nicht-gemacht` (Kategorie + Suche + bewertung bleiben erhalten).
fn build_not_made_toggle_url(
    is_active: bool,
    active_categories: &[String],
    search_query: &str,
    bewertung: Option<&str>,
) -> String {
    let mut params: Vec<String> = Vec::new();

    if !search_query.is_empty() {
        params.push(format!("q={}", urlencoding::encode(search_query)));
    }

    for cat in active_categories {
        params.push(format!("kategorie={}", urlencoding::encode(cat)));
    }

    if !is_active {
        params.push("filter=laenger-nicht-gemacht".to_string());
    }

    if let Some(b) = bewertung {
        params.push(format!("bewertung={}", urlencoding::encode(b)));
    }

    if params.is_empty() {
        "/".to_string()
    } else {
        format!("/?{}", params.join("&"))
    }
}

/// Baut die Toggle-URL für den "Nächste 7 Tage"-Filter.
/// Aktiv → URL ohne `filter`-Parameter (Kategorie + Suche + bewertung bleiben erhalten).
/// Inaktiv → URL mit `filter=naechste-7-tage` (Kategorie + Suche + bewertung bleiben erhalten).
fn build_next_seven_days_toggle_url(
    is_active: bool,
    active_categories: &[String],
    search_query: &str,
    bewertung: Option<&str>,
) -> String {
    let mut params: Vec<String> = Vec::new();

    if !search_query.is_empty() {
        params.push(format!("q={}", urlencoding::encode(search_query)));
    }

    for cat in active_categories {
        params.push(format!("kategorie={}", urlencoding::encode(cat)));
    }

    if !is_active {
        params.push("filter=naechste-7-tage".to_string());
    }

    if let Some(b) = bewertung {
        params.push(format!("bewertung={}", urlencoding::encode(b)));
    }

    if params.is_empty() {
        "/".to_string()
    } else {
        format!("/?{}", params.join("&"))
    }
}

/// Baut die Toggle-URL für den Bewertungsfilter.
/// Wenn der aktuelle Wert dem Zielwert entspricht (Toggle: deaktivieren), wird `bewertung` aus der URL entfernt.
/// Andernfalls wird `bewertung={value}` gesetzt (ggf. ersetzt).
/// Alle anderen Parameter (q, kategorie, filter) bleiben erhalten.
fn build_bewertung_toggle_url(
    value: &str,
    current: Option<&str>,
    active_categories: &[String],
    search_query: &str,
    not_made_filter_active: bool,
    next_seven_days_filter_active: bool,
) -> String {
    let mut params: Vec<String> = Vec::new();

    if !search_query.is_empty() {
        params.push(format!("q={}", urlencoding::encode(search_query)));
    }

    for cat in active_categories {
        params.push(format!("kategorie={}", urlencoding::encode(cat)));
    }

    if not_made_filter_active {
        params.push("filter=laenger-nicht-gemacht".to_string());
    } else if next_seven_days_filter_active {
        params.push("filter=naechste-7-tage".to_string());
    }

    // Toggle: gleiches Value → deaktivieren (kein bewertung-Parameter)
    if current != Some(value) {
        params.push(format!("bewertung={}", urlencoding::encode(value)));
    }

    if params.is_empty() {
        "/".to_string()
    } else {
        format!("/?{}", params.join("&"))
    }
}

/// Baut den Query-String aus den aktiven Filterparametern (ohne führendes `?`).
/// Ergebnis wird für das Speichern-Formular verwendet.
fn build_current_query_string(
    active_categories: &[String],
    search_query: &str,
    not_made_filter_active: bool,
    next_seven_days_filter_active: bool,
    bewertung: Option<&str>,
) -> String {
    let mut params: Vec<String> = Vec::new();

    if !search_query.is_empty() {
        params.push(format!("q={}", urlencoding::encode(search_query)));
    }

    for cat in active_categories {
        params.push(format!("kategorie={}", urlencoding::encode(cat)));
    }

    if not_made_filter_active {
        params.push("filter=laenger-nicht-gemacht".to_string());
    } else if next_seven_days_filter_active {
        params.push("filter=naechste-7-tage".to_string());
    }

    if let Some(b) = bewertung {
        params.push(format!("bewertung={}", urlencoding::encode(b)));
    }

    params.join("&")
}

/// Normalisiert Kategorienamen aus URL-Parametern auf die kanonische Schreibweise aus VALID_CATEGORIES.
/// Ungültige Kategorien werden stillschweigend ignoriert.
fn normalize_categories(raw: Vec<String>) -> Vec<String> {
    raw.into_iter()
        .filter_map(|input| {
            VALID_CATEGORIES
                .iter()
                .find(|&&valid| valid.to_lowercase() == input.to_lowercase())
                .map(|&valid| valid.to_string())
        })
        .collect()
}

/// Zeigt die Liste aller Rezepte. Unterstützt `?deleted=Titel` für Erfolgsmeldungen nach dem Löschen.
/// Unterstützt `?q=suchbegriff` für die Volltextsuche in Titel, Zutaten und Anleitung.
/// Unterstützt `?kategorie=Brot&kategorie=Kuchen` für den Kategorie-Filter (ODER-Logik).
/// Unterstützt `?filter=laenger-nicht-gemacht` für den "Länger nicht gemacht"-Filter.
/// Unterstützt `?filter=naechste-7-tage` für den "Nächste 7 Tage"-Filter.
/// Unterstützt `?bewertung=gut` (rating >= 3) und `?bewertung=favoriten` (rating = 5) für den Bewertungsfilter.
pub async fn index(
    State(pool): State<Arc<SqlitePool>>,
    Query(query): Query<IndexQuery>,
    RawQuery(raw_query): RawQuery,
) -> Result<impl IntoResponse, AppError> {
    let search_query = query.q.unwrap_or_default();
    let raw = raw_query.unwrap_or_default();
    let active_categories = normalize_categories(extract_kategorie_params(&raw));
    // filter-Parameter aus Raw-Query extrahieren, damit mehrfache filter=-Werte (DeepLink-Konflikt)
    // graceful behandelt werden: erster Wert gewinnt, kein 400-Fehler.
    let filter_param = extract_filter_param(&raw);
    let not_made_filter_active = filter_param.as_deref() == Some("laenger-nicht-gemacht");
    let next_seven_days_filter_active = filter_param.as_deref() == Some("naechste-7-tage");

    // Bewertungsfilter: nur "gut" und "favoriten" akzeptieren, Rest ignorieren
    let bewertung: Option<String> = query.bewertung.and_then(|b| {
        if b == "gut" || b == "favoriten" {
            Some(b)
        } else {
            None
        }
    });

    let recipes: Vec<Recipe> = if not_made_filter_active {
        filter_recipes_not_made_recently(
            &pool,
            &active_categories,
            &search_query,
            bewertung.as_deref(),
        )
        .await?
    } else if next_seven_days_filter_active {
        filter_recipes_next_seven_days(
            &pool,
            &active_categories,
            &search_query,
            bewertung.as_deref(),
        )
        .await?
    } else {
        filter_recipes_by_categories(
            &pool,
            &active_categories,
            &search_query,
            bewertung.as_deref(),
        )
        .await?
    };

    let recipe_items: Vec<RecipeListItem> = recipes
        .into_iter()
        .map(|r| RecipeListItem {
            id: r.id,
            title: r.title.clone(),
            categories: r.categories_vec(),
            planned_date: format_planned_date_short(r.planned_date),
            planned_date_weekday: if next_seven_days_filter_active {
                format_planned_date_with_weekday(r.planned_date)
            } else {
                None
            },
            rating: r.rating,
        })
        .collect();

    let category_filters = build_category_filters(
        &active_categories,
        &search_query,
        not_made_filter_active,
        next_seven_days_filter_active,
        bewertung.as_deref(),
    );
    let reset_categories_url = build_reset_url(
        &search_query,
        not_made_filter_active,
        next_seven_days_filter_active,
        bewertung.as_deref(),
    );
    let not_made_filter_toggle_url = build_not_made_toggle_url(
        not_made_filter_active,
        &active_categories,
        &search_query,
        bewertung.as_deref(),
    );
    let next_seven_days_filter_toggle_url = build_next_seven_days_toggle_url(
        next_seven_days_filter_active,
        &active_categories,
        &search_query,
        bewertung.as_deref(),
    );
    let bewertung_gut_toggle_url = build_bewertung_toggle_url(
        "gut",
        bewertung.as_deref(),
        &active_categories,
        &search_query,
        not_made_filter_active,
        next_seven_days_filter_active,
    );
    let bewertung_favoriten_toggle_url = build_bewertung_toggle_url(
        "favoriten",
        bewertung.as_deref(),
        &active_categories,
        &search_query,
        not_made_filter_active,
        next_seven_days_filter_active,
    );

    let any_filter_active = !active_categories.is_empty()
        || !search_query.is_empty()
        || not_made_filter_active
        || next_seven_days_filter_active
        || bewertung.is_some();

    let current_query_string = build_current_query_string(
        &active_categories,
        &search_query,
        not_made_filter_active,
        next_seven_days_filter_active,
        bewertung.as_deref(),
    );

    let saved_filters = get_all_saved_filters(&pool).await?;
    let saved_filter_items: Vec<SavedFilterItem> = saved_filters
        .into_iter()
        .map(|sf| SavedFilterItem {
            id: sf.id,
            name: sf.name.clone(),
            url: format!("/?{}", sf.query_string),
            delete_aria_label: format!("Filter '{}' löschen", sf.name),
        })
        .collect();

    let template = IndexTemplate {
        recipes: recipe_items,
        deleted_title: query.deleted,
        search_query,
        active_categories,
        category_filters,
        reset_categories_url,
        not_made_filter_active,
        not_made_filter_toggle_url,
        next_seven_days_filter_active,
        next_seven_days_filter_toggle_url,
        bewertung_filter: bewertung,
        bewertung_gut_toggle_url,
        bewertung_favoriten_toggle_url,
        any_filter_active,
        saved_filters: saved_filter_items,
        current_query_string,
        save_error: query.save_error,
        save_name: query.save_name,
    };
    Ok(Html(render_template(template)?))
}

/// Zeigt das Formular zum Erstellen eines neuen Rezepts.
pub async fn new_recipe_form() -> Result<impl IntoResponse, AppError> {
    let template = RecipeFormTemplate::new();
    Ok(Html(render_template(template)?))
}

/// Verarbeitet das Formular zum Erstellen eines neuen Rezepts und speichert es in der Datenbank.
pub async fn create_recipe_handler(
    State(pool): State<Arc<SqlitePool>>,
    axum::extract::RawForm(body): axum::extract::RawForm,
) -> Result<impl IntoResponse, AppError> {
    let params = parse_form_data(&body);

    let title = params
        .get("title")
        .and_then(|v| v.first())
        .cloned()
        .unwrap_or_default();
    let categories: Vec<String> = params.get("categories").cloned().unwrap_or_default();
    let ingredients = params
        .get("ingredients")
        .and_then(|v| v.first())
        .filter(|s| !s.is_empty())
        .cloned();
    let instructions = params
        .get("instructions")
        .and_then(|v| v.first())
        .filter(|s| !s.is_empty())
        .cloned();
    let planned_date_raw = params.get("planned_date").and_then(|v| v.first()).cloned();
    let rating = parse_rating(
        params
            .get("rating")
            .and_then(|v| v.first())
            .map(|s| s.as_str()),
    );

    let recipe = CreateRecipe {
        title: title.clone(),
        categories: categories.clone(),
        ingredients: ingredients.clone(),
        instructions: instructions.clone(),
        planned_date_input: planned_date_raw.clone(),
        rating,
    };

    if let Err(errors) = recipe.validate() {
        let template = RecipeFormTemplate {
            categories: VALID_CATEGORIES
                .iter()
                .map(|&s: &&str| s.to_string())
                .collect(),
            errors,
            title,
            selected_categories: categories,
            ingredients: ingredients.unwrap_or_default(),
            instructions: instructions.unwrap_or_default(),
            recipe_id: None,
            planned_date: planned_date_raw.unwrap_or_default(),
            rating,
        };
        return Ok((StatusCode::BAD_REQUEST, Html(render_template(template)?)).into_response());
    }

    let id = create_recipe(&pool, &recipe).await?;
    Ok(Redirect::to(&format!("/recipes/{}", id)).into_response())
}

/// Zeigt die Detailansicht eines Rezepts. Unterstützt `?success=1` für Erfolgsmeldungen.
pub async fn show_recipe(
    State(pool): State<Arc<SqlitePool>>,
    Path(id): Path<i64>,
    Query(query): Query<RecipeDetailQuery>,
) -> Result<impl IntoResponse, AppError> {
    let recipe_option: Option<Recipe> = get_recipe_by_id(&pool, id).await?;

    let Some(recipe) = recipe_option else {
        let template = NotFoundTemplate {
            message: format!("Rezept mit ID {} wurde nicht gefunden.", id),
        };
        let html = render_template(template)?;
        return Ok((StatusCode::NOT_FOUND, Html(html)).into_response());
    };

    let planned_date = format_planned_date_long(recipe.planned_date);
    let template = RecipeDetailTemplate {
        id: recipe.id,
        title: recipe.title.clone(),
        categories: recipe.categories_vec(),
        ingredients: recipe.ingredients,
        instructions: recipe.instructions,
        created_at: format_date(&recipe.created_at),
        updated_at: format_date(&recipe.updated_at),
        success: query.success.as_deref() == Some("1"),
        planned_date,
        rating: recipe.rating,
    };

    Ok(Html(render_template(template)?).into_response())
}

/// Zeigt das Formular zum Bearbeiten eines bestehenden Rezepts, vorausgefüllt mit den aktuellen Daten.
pub async fn edit_recipe_form(
    State(pool): State<Arc<SqlitePool>>,
    Path(id): Path<i64>,
) -> Result<impl IntoResponse, AppError> {
    let recipe: Recipe = get_recipe_by_id(&pool, id)
        .await?
        .ok_or_else(|| AppError::NotFound(format!("Rezept mit ID {} nicht gefunden", id)))?;

    let planned_date = format_planned_date_input(recipe.planned_date);
    let template = RecipeFormTemplate {
        categories: VALID_CATEGORIES.iter().map(|&s| s.to_string()).collect(),
        errors: Vec::new(),
        title: recipe.title.clone(),
        selected_categories: recipe.categories_vec(),
        ingredients: recipe.ingredients.unwrap_or_default(),
        instructions: recipe.instructions.unwrap_or_default(),
        recipe_id: Some(id),
        planned_date,
        rating: recipe.rating,
    };

    Ok(Html(render_template(template)?))
}

/// Verarbeitet das Formular zum Bearbeiten eines Rezepts und aktualisiert es in der Datenbank.
pub async fn update_recipe_handler(
    State(pool): State<Arc<SqlitePool>>,
    Path(id): Path<i64>,
    axum::extract::RawForm(body): axum::extract::RawForm,
) -> Result<impl IntoResponse, AppError> {
    let params = parse_form_data(&body);

    let title = params
        .get("title")
        .and_then(|v| v.first())
        .cloned()
        .unwrap_or_default();
    let categories: Vec<String> = params.get("categories").cloned().unwrap_or_default();
    let ingredients = params
        .get("ingredients")
        .and_then(|v| v.first())
        .filter(|s| !s.is_empty())
        .cloned();
    let instructions = params
        .get("instructions")
        .and_then(|v| v.first())
        .filter(|s| !s.is_empty())
        .cloned();
    let planned_date_raw = params.get("planned_date").and_then(|v| v.first()).cloned();
    let rating = parse_rating(
        params
            .get("rating")
            .and_then(|v| v.first())
            .map(|s| s.as_str()),
    );

    let recipe = UpdateRecipe {
        title: title.clone(),
        categories: categories.clone(),
        ingredients: ingredients.clone(),
        instructions: instructions.clone(),
        planned_date_input: planned_date_raw.clone(),
        rating,
    };

    if let Err(errors) = recipe.validate() {
        let template = RecipeFormTemplate {
            categories: VALID_CATEGORIES.iter().map(|&s| s.to_string()).collect(),
            errors,
            title,
            selected_categories: categories,
            ingredients: ingredients.unwrap_or_default(),
            instructions: instructions.unwrap_or_default(),
            recipe_id: Some(id),
            planned_date: planned_date_raw.unwrap_or_default(),
            rating,
        };
        return Ok((StatusCode::BAD_REQUEST, Html(render_template(template)?)).into_response());
    }

    update_recipe(&pool, id, &recipe)
        .await
        .map_err(|e| match e {
            sqlx::Error::RowNotFound => {
                AppError::NotFound(format!("Rezept mit ID {} nicht gefunden", id))
            }
            other => AppError::Database(other),
        })?;

    Ok(Redirect::to(&format!("/recipes/{}?success=1", id)).into_response())
}

/// Zeigt die Bestätigungsseite zum Löschen eines Rezepts.
pub async fn confirm_delete(
    State(pool): State<Arc<SqlitePool>>,
    Path(id): Path<i64>,
) -> Result<impl IntoResponse, AppError> {
    let recipe: Recipe = get_recipe_by_id(&pool, id)
        .await?
        .ok_or_else(|| AppError::NotFound(format!("Rezept mit ID {} nicht gefunden", id)))?;

    let template = ConfirmDeleteTemplate {
        id: recipe.id,
        title: recipe.title,
    };
    Ok(Html(render_template(template)?))
}

/// Aktualisiert nur die Bewertung eines Rezepts und gibt das aktualisierte HTML-Fragment zurück.
/// Antwortet mit dem Inline-Rating-Fragment für HTMX-Swap.
pub async fn update_recipe_rating_handler(
    State(pool): State<Arc<SqlitePool>>,
    Path(id): Path<i64>,
    axum::extract::RawForm(body): axum::extract::RawForm,
) -> Result<impl IntoResponse, AppError> {
    let params = parse_form_data(&body);
    let rating_raw = params
        .get("rating")
        .and_then(|v| v.first())
        .map(|s| s.as_str());
    let rating = parse_rating(rating_raw);

    // Validierung: Wert vorhanden und außerhalb 1-5 → 400
    if let Some(err) = validate_rating(rating) {
        return Err(AppError::BadRequest(err));
    }

    update_recipe_rating(&pool, id, rating)
        .await
        .map_err(|e| match e {
            sqlx::Error::RowNotFound => {
                AppError::NotFound(format!("Rezept mit ID {} nicht gefunden", id))
            }
            other => AppError::Database(other),
        })?;

    let template = InlineRatingTemplate { id, rating };
    Ok(Html(render_template(template)?))
}

/// Löscht ein Rezept und leitet zur Übersichtsseite mit Erfolgsmeldung weiter.
pub async fn delete_recipe_handler(
    State(pool): State<Arc<SqlitePool>>,
    Path(id): Path<i64>,
) -> Result<impl IntoResponse, AppError> {
    let recipe: Recipe = get_recipe_by_id(&pool, id)
        .await?
        .ok_or_else(|| AppError::NotFound(format!("Rezept mit ID {} nicht gefunden", id)))?;

    let title = recipe.title.clone();

    delete_recipe(&pool, id).await.map_err(|e| match e {
        sqlx::Error::RowNotFound => {
            AppError::NotFound(format!("Rezept mit ID {} nicht gefunden", id))
        }
        other => AppError::Database(other),
    })?;

    let encoded_title = urlencoding::encode(&title);
    Ok(Redirect::to(&format!("/?deleted={encoded_title}")).into_response())
}

/// Speichert den aktuellen Filterzustand unter einem Namen.
/// Bei Duplikat-Namen → Redirect mit Fehler-Query-Parameter.
pub async fn create_saved_filter_handler(
    State(pool): State<Arc<SqlitePool>>,
    axum::extract::RawForm(body): axum::extract::RawForm,
) -> Result<impl IntoResponse, AppError> {
    let params = parse_form_data(&body);

    let name = params
        .get("name")
        .and_then(|v| v.first())
        .cloned()
        .unwrap_or_default();
    let query_string = params
        .get("query_string")
        .and_then(|v| v.first())
        .cloned()
        .unwrap_or_default();

    let filter = CreateSavedFilter {
        name: name.clone(),
        query_string: query_string.clone(),
    };

    if !filter.is_valid() {
        let encoded_name = urlencoding::encode(&name);
        let redirect_url = if query_string.is_empty() {
            format!("/?save_error=ungueltig&save_name={encoded_name}")
        } else {
            format!(
                "/?{}&save_error=ungueltig&save_name={encoded_name}",
                query_string
            )
        };
        return Ok(Redirect::to(&redirect_url).into_response());
    }

    match create_saved_filter(&pool, &filter).await {
        Ok(_) => Ok(Redirect::to(&format!("/?{}", query_string)).into_response()),
        Err(sqlx::Error::Database(db_err)) if db_err.is_unique_violation() => {
            let encoded_name = urlencoding::encode(&name);
            let redirect_url = if query_string.is_empty() {
                format!("/?save_error=duplikat&save_name={encoded_name}")
            } else {
                format!(
                    "/?{}&save_error=duplikat&save_name={encoded_name}",
                    query_string
                )
            };
            Ok(Redirect::to(&redirect_url).into_response())
        }
        Err(e) => Err(AppError::Database(e)),
    }
}

/// Löscht einen gespeicherten Filter anhand seiner ID.
/// Bei HTMX-Request: leere 200-Antwort (HTMX entfernt das Element per `hx-swap="delete"`).
/// Ohne HTMX: Redirect zu `/`.
pub async fn delete_saved_filter_handler(
    State(pool): State<Arc<SqlitePool>>,
    Path(id): Path<i64>,
    headers: HeaderMap,
) -> Result<impl IntoResponse, AppError> {
    delete_saved_filter(&pool, id).await.map_err(|e| match e {
        sqlx::Error::RowNotFound => {
            AppError::NotFound(format!("Gespeicherter Filter mit ID {} nicht gefunden", id))
        }
        other => AppError::Database(other),
    })?;

    let is_htmx = headers
        .get("HX-Request")
        .and_then(|v| v.to_str().ok())
        .map(|v| v == "true")
        .unwrap_or(false);

    if is_htmx {
        Ok(StatusCode::OK.into_response())
    } else {
        Ok(Redirect::to("/").into_response())
    }
}

/// Query-Parameter für den Duplikat-Check-Endpunkt.
#[derive(Deserialize)]
pub struct CheckDuplicateQuery {
    pub title: Option<String>,
    pub exclude_id: Option<i64>,
}

/// Prüft, ob ähnliche Rezepte mit dem angegebenen Titel existieren.
/// Antwortet mit einem HTML-Fragment (Askama-Partial) — immer HTTP 200.
/// Bei DB-Fehler oder kurzem Titel wird ein leeres Fragment zurückgegeben (Graceful Degradation).
pub async fn check_duplicate(
    State(pool): State<Arc<SqlitePool>>,
    Query(params): Query<CheckDuplicateQuery>,
) -> impl IntoResponse {
    let title = params.title.unwrap_or_default();
    let candidates = find_similar_recipes(&pool, &title, params.exclude_id)
        .await
        .unwrap_or_default();

    let template = DuplicateHintTemplate { candidates };
    let html = template.render().unwrap_or_default();

    Html(html)
}
