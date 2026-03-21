use axum::{
    routing::get,
    routing::post,
    Router,
};
use sqlx::SqlitePool;
use std::sync::Arc;
use tower_http::services::ServeDir;

pub mod recipes;

pub fn create_router(pool: SqlitePool) -> Router {
    let pool = Arc::new(pool);
    
    Router::new()
        .route("/", get(recipes::index))
        .route("/health", get(health_check))
        .route("/recipes", post(recipes::create_recipe_handler))
        .route("/recipes/new", get(recipes::new_recipe_form))
        .route("/recipes/:id", get(recipes::show_recipe))
        .nest_service("/static", ServeDir::new("src/static"))
        .with_state(pool)
}

async fn health_check() -> &'static str {
    "OK"
}
