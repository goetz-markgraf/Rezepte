use crate::error::AppError;
use crate::markdown::render_and_sanitize;
use crate::models::{
    create_recipe, create_saved_filter, delete_recipe, delete_saved_filter, determine_merge_target,
    filter_recipes_by_categories, filter_recipes_next_seven_days, filter_recipes_not_made_recently,
    find_all_duplicate_pairs, find_similar_recipes, get_all_saved_filters, get_recipe_by_id,
    get_recipes_by_date_range, merge_recipes, update_recipe, CreateRecipe, CreateSavedFilter,
    Recipe, UpdateRecipe, VALID_CATEGORIES,
};
use crate::templates::{
    CategoryFilterItem, ConfirmDeleteTemplate, DublettenPaarItem, DublettenUebersichtTemplate,
    DuplicateHintTemplate, IndexTemplate, MergeRezeptInfo, MergeTemplate, NotFoundTemplate,
    RecipeDetailTemplate, RecipeFormTemplate, RecipeListItem, SavedFilterItem,
    WeekdayPickerRecipeInfo,
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
    #[allow(dead_code)]
    pub bewertung: Option<String>,
    pub save_error: Option<String>,
    pub save_name: Option<String>,
    pub filter_collapsed: Option<String>,
}

/// Baut die Toggle-URL für das Ein-/Ausklappen des Filterbereichs.
/// Eingeklappt (`true`) → gibt URL mit `filter_collapsed=0` zurück (Aufklappen).
/// Ausgeklappt (`false`) → gibt URL ohne `filter_collapsed`-Parameter zurück (Einklappen).
/// Alle anderen aktiven Parameter (q, kategorie, filter) bleiben erhalten.
pub fn build_filter_collapsed_toggle_url(
    is_collapsed: bool,
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


    // Zustand umkehren: war eingeklappt → jetzt ausklappen (filter_collapsed=0 hinzufügen)
    // war ausgeklappt → jetzt einklappen (kein Parameter = Default eingeklappt, Story 40)
    if is_collapsed {
        params.push("filter_collapsed=0".to_string());
    }

    if params.is_empty() {
        "/".to_string()
    } else {
        format!("/?{}", params.join("&"))
    }
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
/// Bestehender Suchbegriff, `not_made_filter_active`, `next_seven_days_filter_active` bleiben erhalten.
fn build_category_toggle_url(
    category: &str,
    is_active: bool,
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
/// Wenn `not_made_filter_active` oder `next_seven_days_filter_active` gesetzt ist, bleiben diese erhalten.
fn build_reset_url(
    search_query: &str,
    not_made_filter_active: bool,
    next_seven_days_filter_active: bool,
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
    if params.is_empty() {
        "/".to_string()
    } else {
        format!("/?{}", params.join("&"))
    }
}

/// Baut die Toggle-URL für den "Länger nicht gemacht"-Filter.
/// Aktiv → URL ohne `filter`-Parameter (Kategorie + Suche bleiben erhalten).
/// Inaktiv → URL mit `filter=laenger-nicht-gemacht` (Kategorie + Suche bleiben erhalten).
fn build_not_made_toggle_url(
    is_active: bool,
    active_categories: &[String],
    search_query: &str,
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


    if params.is_empty() {
        "/".to_string()
    } else {
        format!("/?{}", params.join("&"))
    }
}

/// Baut die Toggle-URL für den "Nächste 7 Tage"-Filter.
/// Aktiv → URL ohne `filter`-Parameter (Kategorie + Suche bleiben erhalten).
/// Inaktiv → URL mit `filter=naechste-7-tage` (Kategorie + Suche bleiben erhalten).
fn build_next_seven_days_toggle_url(
    is_active: bool,
    active_categories: &[String],
    search_query: &str,
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

    // filter_collapsed: "0" → ausgeklappt, alles andere → eingeklappt (Story 40)
    let filter_collapsed = query.filter_collapsed.as_deref() != Some("0");

    let recipes: Vec<Recipe> = if not_made_filter_active {
        filter_recipes_not_made_recently(&pool, &active_categories, &search_query)
        .await?
    } else if next_seven_days_filter_active {
        filter_recipes_next_seven_days(&pool, &active_categories, &search_query)
        .await?
    } else {
        filter_recipes_by_categories(&pool, &active_categories, &search_query)
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
        })
        .collect();

    let category_filters = build_category_filters(&active_categories, &search_query, not_made_filter_active, next_seven_days_filter_active);
    let reset_categories_url = build_reset_url(&search_query, not_made_filter_active, next_seven_days_filter_active);
    let not_made_filter_toggle_url = build_not_made_toggle_url(not_made_filter_active, &active_categories, &search_query);
    let next_seven_days_filter_toggle_url = build_next_seven_days_toggle_url(next_seven_days_filter_active, &active_categories, &search_query);

    let any_filter_active = !active_categories.is_empty()
        || !search_query.is_empty()
        || not_made_filter_active
        || next_seven_days_filter_active;

    let current_query_string = build_current_query_string(&active_categories, &search_query, not_made_filter_active, next_seven_days_filter_active);

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

    let filter_collapsed_toggle_url = build_filter_collapsed_toggle_url(filter_collapsed, &active_categories, &search_query, not_made_filter_active, next_seven_days_filter_active);

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
        any_filter_active,
        saved_filters: saved_filter_items,
        current_query_string,
        save_error: query.save_error,
        save_name: query.save_name,
        filter_collapsed,
        filter_collapsed_toggle_url,
    };
    Ok(Html(render_template(template)?))
}

/// Hilfsfunktion: Lädt geplante Rezepte für den Wochenpicker (nächste 10 Tage ab morgen).
/// Gibt einen Vektor mit 10 Optionen zurück (Index 0-9 = morgen bis +10 Tage).
async fn load_planned_recipes_for_weekday_picker(
    pool: &sqlx::SqlitePool,
) -> Result<Vec<Option<WeekdayPickerRecipeInfo>>, AppError> {
    let today = time::OffsetDateTime::now_utc().date();
    let tomorrow = today + time::Duration::days(1);
    let day_ten = today + time::Duration::days(10);

    // Alle Rezepte im Bereich [morgen, +10 Tage] laden
    let recipes = get_recipes_by_date_range(pool, tomorrow, day_ten).await?;

    // Gruppiere Rezepte nach Datum (nur erstes Rezept pro Tag)
    let mut planned_recipes: Vec<Option<WeekdayPickerRecipeInfo>> = vec![None; 10];

    for recipe in recipes {
        if let Some(date) = recipe.planned_date {
            // Berechne Offset vom morgigen Tag (0-9)
            let offset =
                (i64::from(date.to_julian_day()) - i64::from(tomorrow.to_julian_day())) as usize;
            if offset < 10 && planned_recipes[offset].is_none() {
                // Nur das erste Rezept pro Tag speichern
                planned_recipes[offset] = Some(WeekdayPickerRecipeInfo {
                    id: recipe.id,
                    title: recipe.title,
                });
            }
        }
    }

    Ok(planned_recipes)
}

/// Zeigt das Formular zum Erstellen eines neuen Rezepts.
pub async fn new_recipe_form(
    State(pool): State<Arc<SqlitePool>>,
) -> Result<impl IntoResponse, AppError> {
    let planned_recipes = load_planned_recipes_for_weekday_picker(&pool).await?;
    let template = RecipeFormTemplate {
        planned_recipes,
        ..RecipeFormTemplate::new()
    };
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

    let recipe = CreateRecipe {
        title: title.clone(),
        categories: categories.clone(),
        ingredients: ingredients.clone(),
        instructions: instructions.clone(),
        planned_date_input: planned_date_raw.clone(),
    };

    if let Err(errors) = recipe.validate() {
        let planned_recipes = load_planned_recipes_for_weekday_picker(&pool).await?;
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
            planned_recipes,
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
    let ingredients = render_and_sanitize(recipe.ingredients.as_deref());
    let instructions = render_and_sanitize(recipe.instructions.as_deref());
    let template = RecipeDetailTemplate {
        id: recipe.id,
        title: recipe.title.clone(),
        categories: recipe.categories_vec(),
        ingredients,
        instructions,
        created_at: format_date(&recipe.created_at),
        updated_at: format_date(&recipe.updated_at),
        success: query.success.as_deref() == Some("1"),
        planned_date,
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
    let planned_recipes = load_planned_recipes_for_weekday_picker(&pool).await?;
    let template = RecipeFormTemplate {
        categories: VALID_CATEGORIES.iter().map(|&s| s.to_string()).collect(),
        errors: Vec::new(),
        title: recipe.title.clone(),
        selected_categories: recipe.categories_vec(),
        ingredients: recipe.ingredients.unwrap_or_default(),
        instructions: recipe.instructions.unwrap_or_default(),
        recipe_id: Some(id),
        planned_date,
        planned_recipes,
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

    let recipe = UpdateRecipe {
        title: title.clone(),
        categories: categories.clone(),
        ingredients: ingredients.clone(),
        instructions: instructions.clone(),
        planned_date_input: planned_date_raw.clone(),
    };

    if let Err(errors) = recipe.validate() {
        let planned_recipes = load_planned_recipes_for_weekday_picker(&pool).await?;
        let template = RecipeFormTemplate {
            categories: VALID_CATEGORIES.iter().map(|&s| s.to_string()).collect(),
            errors,
            title,
            selected_categories: categories,
            ingredients: ingredients.unwrap_or_default(),
            instructions: instructions.unwrap_or_default(),
            recipe_id: Some(id),
            planned_date: planned_date_raw.unwrap_or_default(),
            planned_recipes,
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

/// Zeigt die Dubletten-Übersichtsseite an.
/// Findet alle potentiellen Dubletten-Paare und stellt sie als vollständige HTML-Seite dar.
pub async fn duplicates_handler(
    State(pool): State<Arc<SqlitePool>>,
) -> Result<impl IntoResponse, AppError> {
    let paare = find_all_duplicate_pairs(&pool).await?;
    let paar_items: Vec<DublettenPaarItem> = paare
        .into_iter()
        .map(|p| DublettenPaarItem {
            id_a: p.rezept_a.id,
            titel_a: p.rezept_a.title,
            id_b: p.rezept_b.id,
            titel_b: p.rezept_b.title,
        })
        .collect();
    let template = DublettenUebersichtTemplate { paare: paar_items };
    Ok(Html(render_template(template)?))
}

/// Query-Parameter für die Merge-Seite.
#[derive(Deserialize)]
pub struct MergeQuery {
    pub source: Option<i64>,
    pub target: Option<i64>,
}

/// Konvertiert ein `Recipe` in ein `MergeRezeptInfo` für das Template.
fn recipe_to_merge_info(recipe: &Recipe) -> MergeRezeptInfo {
    let planned_date = format_planned_date_long(recipe.planned_date);
    MergeRezeptInfo {
        title: recipe.title.clone(),
        categories: recipe.categories_vec(),
        ingredients: recipe.ingredients.clone(),
        instructions: recipe.instructions.clone(),
        planned_date,
        created_at: format_date(&recipe.created_at),
        updated_at: format_date(&recipe.updated_at),
    }
}

/// Zeigt die Merge-Seite mit beiden Rezepten nebeneinander an.
/// GET /recipes/merge?source=ID&target=ID
pub async fn merge_form_handler(
    State(pool): State<Arc<SqlitePool>>,
    Query(query): Query<MergeQuery>,
) -> Result<impl IntoResponse, AppError> {
    let source_id = query.source.ok_or_else(|| {
        AppError::BadRequest("source und target Parameter erforderlich".to_string())
    })?;
    let target_id = query.target.ok_or_else(|| {
        AppError::BadRequest("source und target Parameter erforderlich".to_string())
    })?;

    let source: Recipe = get_recipe_by_id(&pool, source_id)
        .await?
        .ok_or_else(|| AppError::NotFound(format!("Rezept mit ID {} nicht gefunden", source_id)))?;

    let target: Recipe = get_recipe_by_id(&pool, target_id)
        .await?
        .ok_or_else(|| AppError::NotFound(format!("Rezept mit ID {} nicht gefunden", target_id)))?;

    // determine_merge_target bestimmt, welches Rezept als Ziel empfohlen wird
    // aber wir verwenden die URL-Parameter direkt (der Nutzer hat die Rollen schon festgelegt)
    let _ = determine_merge_target(&source, &target); // nur für zukünftige Erweiterungen

    let template = MergeTemplate {
        rezept_a: recipe_to_merge_info(&source),
        rezept_b: recipe_to_merge_info(&target),
        source_id,
        target_id,
        fehler: Vec::new(),
    };

    Ok(Html(render_template(template)?))
}

/// Verarbeitet das Merge-Formular und führt die Rezepte zusammen.
/// POST /recipes/merge
pub async fn merge_handler(
    State(pool): State<Arc<SqlitePool>>,
    axum::extract::RawForm(body): axum::extract::RawForm,
) -> Result<impl IntoResponse, AppError> {
    let params = parse_form_data(&body);

    let source_id: i64 = params
        .get("source_id")
        .and_then(|v| v.first())
        .and_then(|s| s.parse().ok())
        .ok_or_else(|| AppError::BadRequest("source_id fehlt oder ungültig".to_string()))?;

    let target_id: i64 = params
        .get("target_id")
        .and_then(|v| v.first())
        .and_then(|s| s.parse().ok())
        .ok_or_else(|| AppError::BadRequest("target_id fehlt oder ungültig".to_string()))?;

    // Beide Rezepte laden (für Fehlerfall und Feldwerte)
    let source: Recipe = get_recipe_by_id(&pool, source_id)
        .await?
        .ok_or_else(|| AppError::NotFound(format!("Rezept mit ID {} nicht gefunden", source_id)))?;

    let target: Recipe = get_recipe_by_id(&pool, target_id)
        .await?
        .ok_or_else(|| AppError::NotFound(format!("Rezept mit ID {} nicht gefunden", target_id)))?;

    // Feldauswahlen lesen: "a" = source, "b" = target
    let title_from = params
        .get("title_from")
        .and_then(|v| v.first())
        .map(|s| s.as_str())
        .unwrap_or("a");
    let categories_from = params
        .get("categories_from")
        .and_then(|v| v.first())
        .map(|s| s.as_str())
        .unwrap_or("a");
    let ingredients_from = params
        .get("ingredients_from")
        .and_then(|v| v.first())
        .map(|s| s.as_str())
        .unwrap_or("a");
    let instructions_from = params
        .get("instructions_from")
        .and_then(|v| v.first())
        .map(|s| s.as_str())
        .unwrap_or("a");
    let planned_date_from = params
        .get("planned_date_from")
        .and_then(|v| v.first())
        .map(|s| s.as_str())
        .unwrap_or("a");

    let title = if title_from == "b" {
        target.title.clone()
    } else {
        source.title.clone()
    };

    let categories = if categories_from == "b" {
        target.categories_vec()
    } else {
        source.categories_vec()
    };

    let ingredients = if ingredients_from == "b" {
        target.ingredients.clone()
    } else {
        source.ingredients.clone()
    };

    let instructions = if instructions_from == "b" {
        target.instructions.clone()
    } else {
        source.instructions.clone()
    };

    // Datum: Wir speichern im deutschen Format für den Input
    let planned_date_input = if planned_date_from == "b" {
        target
            .planned_date
            .map(|d| format!("{}.{}.{}", d.day(), d.month() as u8, d.year()))
    } else {
        source
            .planned_date
            .map(|d| format!("{}.{}.{}", d.day(), d.month() as u8, d.year()))
    };

    let recipe = UpdateRecipe {
        title: title.clone(),
        categories: categories.clone(),
        ingredients: ingredients.clone(),
        instructions: instructions.clone(),
        planned_date_input: planned_date_input.clone(),
    };

    if let Err(errors) = recipe.validate() {
        let template = MergeTemplate {
            rezept_a: recipe_to_merge_info(&source),
            rezept_b: recipe_to_merge_info(&target),
            source_id,
            target_id,
            fehler: errors,
        };
        return Ok((StatusCode::BAD_REQUEST, Html(render_template(template)?)).into_response());
    }

    merge_recipes(&pool, source_id, target_id, &recipe)
        .await
        .map_err(|e| match e {
            sqlx::Error::RowNotFound => {
                AppError::NotFound("Ein oder beide Rezepte wurden nicht gefunden".to_string())
            }
            other => AppError::Database(other),
        })?;

    Ok(Redirect::to(&format!("/recipes/{}?success=1", target_id)).into_response())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn toggle_url_ausgeklappt_zu_eingeklappt() {
        // Gegeben: Filter ausgeklappt, kein aktiver Filter
        let url = build_filter_collapsed_toggle_url(false, &[], "", false, false);
        // Dann: URL enthält keinen filter_collapsed Parameter (Story 40: Default eingeklappt)
        assert_eq!(url, "/");
    }

    #[test]
    fn toggle_url_eingeklappt_zu_ausgeklappt() {
        // Gegeben: Filter eingeklappt, kein aktiver Filter
        let url = build_filter_collapsed_toggle_url(true, &[], "", false, false);
        // Dann: URL enthält filter_collapsed=0 (Story 40: explizit ausklappen)
        assert_eq!(url, "/?filter_collapsed=0");
    }

    #[test]
    fn toggle_url_behaelt_suchbegriff() {
        // Gegeben: Suchbegriff "pasta" aktiv, Filter ausgeklappt (Story 40: kein Parameter = eingeklappt)
        let url = build_filter_collapsed_toggle_url(false, &[], "pasta", false, false);
        // Dann: URL enthält q=pasta und keinen filter_collapsed Parameter
        assert!(url.contains("q=pasta"), "URL sollte q=pasta enthalten");
        assert!(
            !url.contains("filter_collapsed"),
            "URL sollte keinen filter_collapsed Parameter enthalten"
        );
    }

    #[test]
    fn toggle_url_behaelt_kategorie() {
        // Gegeben: Kategorie "Brot" aktiv, Filter ausgeklappt (Story 40: kein Parameter = eingeklappt)
        let url =
            build_filter_collapsed_toggle_url(false, &["Brot".to_string()], "", false, false);
        // Dann: URL enthält kategorie=Brot und keinen filter_collapsed Parameter
        assert!(
            url.contains("kategorie=Brot"),
            "URL sollte kategorie=Brot enthalten"
        );
        assert!(
            !url.contains("filter_collapsed"),
            "URL sollte keinen filter_collapsed Parameter enthalten"
        );
    }

    #[test]
    fn toggle_url_behaelt_nicht_gemacht_filter() {
        // Gegeben: "Länger nicht gemacht"-Filter aktiv, Filter ausgeklappt (Story 40: kein Parameter = eingeklappt)
        let url = build_filter_collapsed_toggle_url(false, &[], "", true, false);
        // Dann: URL enthält filter=laenger-nicht-gemacht und keinen filter_collapsed Parameter
        assert!(
            url.contains("filter=laenger-nicht-gemacht"),
            "URL sollte filter=laenger-nicht-gemacht enthalten"
        );
        assert!(
            !url.contains("filter_collapsed"),
            "URL sollte keinen filter_collapsed Parameter enthalten"
        );
    }

    #[test]
    fn toggle_url_eingeklappt_behaelt_alle_parameter() {
        // Gegeben: Filter eingeklappt + Kategorie + Suche aktiv (Story 40)
        let url = build_filter_collapsed_toggle_url(
            true,
            &["Brot".to_string()],
            "pasta",
            false,
            false,
        );
        // Dann: URL enthält alle Parameter INKLUSIVE filter_collapsed=0 (ausklappen)
        assert!(url.contains("kategorie=Brot"), "kategorie=Brot fehlt");
        assert!(url.contains("q=pasta"), "q=pasta fehlt");
        assert!(
            url.contains("filter_collapsed=0"),
            "filter_collapsed=0 sollte enthalten sein wenn eingeklappt (um auszuklappen)"
        );
    }

    #[test]
    fn toggle_url_ausgeklappt_mit_parameter_behaelt_alle_parameter() {
        // Gegeben: Filter ausgeklappt (filter_collapsed=0) + Kategorie + Suche aktiv (Story 40)
        let url = build_filter_collapsed_toggle_url(
            false,
            &["Brot".to_string()],
            "pasta",
            false,
            false,
        );
        // Dann: URL enthält alle Parameter AUSSER filter_collapsed (einklappen = Default)
        assert!(url.contains("kategorie=Brot"), "kategorie=Brot fehlt");
        assert!(url.contains("q=pasta"), "q=pasta fehlt");
        assert!(
            !url.contains("filter_collapsed"),
            "filter_collapsed sollte fehlen wenn ausgeklappt (einklappen = Default)"
        );
    }
}
