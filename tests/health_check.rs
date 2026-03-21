use std::net::SocketAddr;
use tempfile::NamedTempFile;

#[tokio::test]
async fn health_check_returns_ok() {
    // Start test server
    let addr = spawn_app().await;
    
    // Make HTTP request
    let client = reqwest::Client::new();
    let response = client
        .get(format!("http://{}/health", addr))
        .send()
        .await
        .expect("Failed to execute request");
    
    // Assert
    assert!(response.status().is_success());
    let body = response.text().await.expect("Failed to read body");
    assert_eq!(body, "OK");
}

async fn spawn_app() -> SocketAddr {
    // Create a temporary database file
    let temp_file = NamedTempFile::new().expect("Failed to create temp file");
    let db_url = format!("sqlite:{}", temp_file.path().to_str().unwrap());
    
    // Use port 0 to let OS assign a random port
    let listener = tokio::net::TcpListener::bind("127.0.0.1:0")
        .await
        .expect("Failed to bind to random port");
    let addr = listener.local_addr().unwrap();
    
    // Create database pool
    let db_pool = rezepte::db::create_pool(&db_url)
        .await
        .expect("Failed to create database pool");
    
    // Create router
    let app = rezepte::routes::create_router(db_pool);
    
    // Spawn server
    tokio::spawn(async move {
        axum::serve(listener, app)
            .await
            .expect("Server failed");
    });
    
    addr
}