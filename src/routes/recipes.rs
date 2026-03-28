use crate::error::AppError;
use crate::models::{
    create_recipe, delete_recipe, filter_recipes_by_categories, get_recipe_by_id, update_recipe,
    CreateRecipe, Recipe, UpdateRecipe, VALID_CATEGORIES,
};
use crate::templates::{
    CategoryFilterItem, ConfirmDeleteTemplate, IndexTemplate, NotFoundTemplate,
    RecipeDetailTemplate, RecipeFormTemplate, RecipeListItem,
};
use axum::{
    extract::{Path, Query, RawQuery, State},
    http::StatusCode,
    response::{Html, IntoResponse, Redirect},
};
use serde::Deserialize;
use sqlx::SqlitePool;
use std::sync::Arc;

#[derive(Deserialize)]
pub struct RecipeDetailQuery {
    pub success: Option<String>,
}

#[derive(Deserialize)]
pub struct IndexQuery {
    pub deleted: Option<String>,
    pub q: Option<String>,
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
/// Bestehender Suchbegriff bleibt erhalten.
fn build_category_toggle_url(
    category: &str,
    is_active: bool,
    active_categories: &[String],
    search_query: &str,
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
) -> Vec<CategoryFilterItem> {
    VALID_CATEGORIES
        .iter()
        .map(|&cat| {
            let is_active = active_categories.iter().any(|a| a == cat);
            let toggle_url =
                build_category_toggle_url(cat, is_active, active_categories, search_query);
            CategoryFilterItem {
                name: cat.to_string(),
                is_active,
                toggle_url,
            }
        })
        .collect()
}

/// Erstellt die URL zum Zurücksetzen aller Kategorie-Filter (Suchbegriff bleibt erhalten).
fn build_reset_url(search_query: &str) -> String {
    if search_query.is_empty() {
        "/".to_string()
    } else {
        format!("/?q={}", urlencoding::encode(search_query))
    }
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
pub async fn index(
    State(pool): State<Arc<SqlitePool>>,
    Query(query): Query<IndexQuery>,
    RawQuery(raw_query): RawQuery,
) -> Result<impl IntoResponse, AppError> {
    let search_query = query.q.unwrap_or_default();
    let raw = raw_query.unwrap_or_default();
    let active_categories = normalize_categories(extract_kategorie_params(&raw));

    let recipes: Vec<Recipe> =
        filter_recipes_by_categories(&pool, &active_categories, &search_query).await?;

    let recipe_items: Vec<RecipeListItem> = recipes
        .into_iter()
        .map(|r| RecipeListItem {
            id: r.id,
            title: r.title.clone(),
            categories: r.categories_vec(),
        })
        .collect();

    let category_filters = build_category_filters(&active_categories, &search_query);
    let reset_categories_url = build_reset_url(&search_query);

    let template = IndexTemplate {
        recipes: recipe_items,
        deleted_title: query.deleted,
        search_query,
        active_categories,
        category_filters,
        reset_categories_url,
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

    let recipe = CreateRecipe {
        title: title.clone(),
        categories: categories.clone(),
        ingredients: ingredients.clone(),
        instructions: instructions.clone(),
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

    let template = RecipeDetailTemplate {
        id: recipe.id,
        title: recipe.title.clone(),
        categories: recipe.categories_vec(),
        ingredients: recipe.ingredients,
        instructions: recipe.instructions,
        created_at: format_date(&recipe.created_at),
        updated_at: format_date(&recipe.updated_at),
        success: query.success.as_deref() == Some("1"),
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

    let template = RecipeFormTemplate {
        categories: VALID_CATEGORIES.iter().map(|&s| s.to_string()).collect(),
        errors: Vec::new(),
        title: recipe.title.clone(),
        selected_categories: recipe.categories_vec(),
        ingredients: recipe.ingredients.unwrap_or_default(),
        instructions: recipe.instructions.unwrap_or_default(),
        recipe_id: Some(id),
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

    let recipe = UpdateRecipe {
        title: title.clone(),
        categories: categories.clone(),
        ingredients: ingredients.clone(),
        instructions: instructions.clone(),
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
