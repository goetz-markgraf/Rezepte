use anyhow::Result;
use tracing_subscriber;

mod config;
mod db;
mod error;
mod routes;

#[tokio::main]
async fn main() -> Result<()> {
    // Initialize tracing
    tracing_subscriber::fmt::init();
    
    // Load configuration
    let config = config::Config::from_env();
    
    // Create database pool
    let db_pool = db::create_pool(&config.database_url).await?;
    
    // Create router
    let app = routes::create_router(db_pool);
    
    // Start server
    let listener = tokio::net::TcpListener::bind(format!("0.0.0.0:{}", config.port)).await?;
    tracing::info!("Server running on http://0.0.0.0:{}", config.port);
    
    axum::serve(listener, app).await?;
    
    Ok(())
}