use crate::error::AppError;
use crate::models::{CreateRecipe, Recipe, create_recipe, get_recipe_by_id, get_all_recipes, VALID_CATEGORIES};
use crate::templates::{IndexTemplate, RecipeDetailTemplate, RecipeFormTemplate, RecipeListItem};
use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::{Html, IntoResponse, Redirect},
};
use sqlx::SqlitePool;
use std::sync::Arc;

fn render_template<T: askama::Template>(template: T) -> Result<String, AppError> {
    template.render().map_err(|e: askama::Error| AppError::BadRequest(e.to_string()))
}

fn decode_form_value(value: &str) -> String {
    // Replace + with space first (URL form encoding), then decode %XX sequences
    let with_spaces = value.replace('+', " ");
    urlencoding::decode(&with_spaces).unwrap_or_default().to_string()
}

pub async fn index(State(pool): State<Arc<SqlitePool>>) -> Result<impl IntoResponse, AppError> {
    let recipes: Vec<Recipe> = get_all_recipes(&pool).await?;
    
    let recipe_items: Vec<RecipeListItem> = recipes
        .into_iter()
        .map(|r| RecipeListItem {
            id: r.id,
            title: r.title.clone(),
            categories: r.categories_vec(),
        })
        .collect();

    let template = IndexTemplate { recipes: recipe_items };
    Ok(Html(render_template(template)?))
}

pub async fn new_recipe_form() -> Result<impl IntoResponse, AppError> {
    let template = RecipeFormTemplate::new();
    Ok(Html(render_template(template)?))
}

pub async fn create_recipe_handler(
    State(pool): State<Arc<SqlitePool>>,
    axum::extract::RawForm(body): axum::extract::RawForm,
) -> Result<impl IntoResponse, AppError> {
    // Parse form data manually
    let form_data = String::from_utf8_lossy(&body);
    let mut params = std::collections::HashMap::new();
    
    for pair in form_data.split('&') {
        if let Some((key, value)) = pair.split_once('=') {
            let key = decode_form_value(key);
            let value = decode_form_value(value);
            params.entry(key).or_insert_with(Vec::new).push(value);
        }
    }
    
    let title = params.get("title").and_then(|v| v.first()).cloned().unwrap_or_default();
    let categories: Vec<String> = params.get("categories").cloned().unwrap_or_default();
    let ingredients = params.get("ingredients").and_then(|v| v.first()).cloned();
    let instructions = params.get("instructions").and_then(|v| v.first()).cloned();
    
    let recipe = CreateRecipe {
        title: title.clone(),
        categories: categories.clone(),
        ingredients: ingredients.clone(),
        instructions: instructions.clone(),
    };

    if let Err(errors) = recipe.validate() {
        let template = RecipeFormTemplate {
            categories: VALID_CATEGORIES.iter().map(|&s: &&str| s.to_string()).collect(),
            errors,
            title,
            selected_categories: categories,
            ingredients: ingredients.unwrap_or_default(),
            instructions: instructions.unwrap_or_default(),
        };
        return Ok((
            StatusCode::BAD_REQUEST,
            Html(render_template(template)?),
        ).into_response());
    }

    let id = create_recipe(&pool, &recipe).await?;
    Ok(Redirect::to(&format!("/recipes/{}", id)).into_response())
}

pub async fn show_recipe(
    State(pool): State<Arc<SqlitePool>>,
    Path(id): Path<i64>,
) -> Result<impl IntoResponse, AppError> {
    let recipe: Recipe = get_recipe_by_id(&pool, id)
        .await?
        .ok_or_else(|| AppError::NotFound(format!("Rezept mit ID {} nicht gefunden", id)))?;

    let template = RecipeDetailTemplate {
        id: recipe.id,
        title: recipe.title.clone(),
        categories: recipe.categories_vec(),
        ingredients: recipe.ingredients,
        instructions: recipe.instructions,
        created_at: recipe.created_at,
    };

    Ok(Html(render_template(template)?))
}
