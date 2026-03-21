use axum::{
    routing::get,
    Router,
};
use sqlx::SqlitePool;

pub mod recipes;

pub fn create_router(pool: SqlitePool) -> Router {
    Router::new()
        .route("/health", get(health_check))
        .with_state(pool)
}

async fn health_check() -> &'static str {
    "OK"
}