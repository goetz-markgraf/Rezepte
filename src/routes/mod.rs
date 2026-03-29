use axum::{
    routing::{get, post},
    Router,
};

use sqlx::SqlitePool;
use std::sync::Arc;
use tower_http::services::ServeDir;

pub mod heute;
pub mod recipes;
pub mod wochenvorschau;

pub fn create_router(pool: SqlitePool) -> Router {
    let pool = Arc::new(pool);

    Router::new()
        .route("/", get(recipes::index))
        .route("/health", get(health_check))
        .route(
            "/wochenvorschau",
            get(wochenvorschau::wochenvorschau_handler),
        )
        .route("/heute", get(heute::heute_handler))
        .route(
            "/heute/recipes/:id/rating",
            post(heute::heute_rating_handler),
        )
        .route("/recipes", post(recipes::create_recipe_handler))
        .route("/recipes/new", get(recipes::new_recipe_form))
        .route("/recipes/check-duplicate", get(recipes::check_duplicate))
        .route(
            "/recipes/:id",
            get(recipes::show_recipe).post(recipes::update_recipe_handler),
        )
        .route("/recipes/:id/edit", get(recipes::edit_recipe_form))
        .route("/recipes/:id/confirm-delete", get(recipes::confirm_delete))
        .route("/recipes/:id/delete", post(recipes::delete_recipe_handler))
        .route(
            "/recipes/:id/rating",
            post(recipes::update_recipe_rating_handler),
        )
        .route("/saved-filters", post(recipes::create_saved_filter_handler))
        .route(
            "/saved-filters/:id/delete",
            post(recipes::delete_saved_filter_handler),
        )
        .nest_service("/static", ServeDir::new("src/static"))
        .with_state(pool)
}

async fn health_check() -> &'static str {
    "OK"
}
