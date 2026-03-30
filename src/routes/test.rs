use axum::{extract::State, response::IntoResponse, Json};
use serde::Deserialize;
use sqlx::SqlitePool;
use std::sync::Arc;

/// Request-Body für das Seeding eines Test-Rezepts
#[derive(Debug, Deserialize)]
pub struct SeedRecipeRequest {
    pub title: String,
    pub categories: Vec<String>,
    pub planned_date: Option<String>, // YYYY-MM-DD Format
}

/// Löscht alle Rezepte aus der Datenbank (Test-Endpunkt)
pub async fn clear_recipes(State(pool): State<Arc<SqlitePool>>) -> impl IntoResponse {
    match sqlx::query("DELETE FROM recipes").execute(&*pool).await {
        Ok(_) => "OK",
        Err(_) => "Error",
    }
}

/// Erstellt ein Test-Rezept und gibt die ID zurück
pub async fn seed_recipe(
    State(pool): State<Arc<SqlitePool>>,
    Json(request): Json<SeedRecipeRequest>,
) -> impl IntoResponse {
    let categories_json =
        serde_json::to_string(&request.categories).unwrap_or_else(|_| "[]".to_string());

    // Parse das Datum im Format YYYY-MM-DD
    let planned_date = request.planned_date.and_then(|date_str| {
        time::Date::parse(
            &date_str,
            &time::format_description::parse("[year]-[month]-[day]").ok()?,
        )
        .ok()
    });

    match sqlx::query(
        r#"
        INSERT INTO recipes (title, categories, ingredients, instructions, planned_date, rating)
        VALUES (?1, ?2, ?3, ?4, ?5, ?6)
        "#,
    )
    .bind(&request.title)
    .bind(&categories_json)
    .bind("") // ingredients
    .bind("") // instructions
    .bind(planned_date)
    .bind(None::<i32>) // rating
    .execute(&*pool)
    .await
    {
        Ok(result) => result.last_insert_rowid().to_string(),
        Err(_) => "Error".to_string(),
    }
}
