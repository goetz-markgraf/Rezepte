use anyhow::Result;

mod config;
mod db;
mod emoji;
mod error;
mod markdown;
mod models;
mod routes;
mod templates;
mod vision;

#[tokio::main]
async fn main() -> Result<()> {
    // Initialize tracing
    tracing_subscriber::fmt::init();

    // Load configuration
    let config = config::Config::from_env();
    let port = config.port;

    // Create database pool
    let db_pool = db::create_pool(&config.database_url).await?;

    // Create router
    let app = routes::create_router(db_pool, config);

    // Start server
    let listener = tokio::net::TcpListener::bind(format!("0.0.0.0:{}", port)).await?;
    tracing::info!("Server running on http://0.0.0.0:{}", port);

    axum::serve(listener, app).await?;

    Ok(())
}
