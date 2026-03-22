use anyhow::Result;
use sqlx::SqlitePool;
use std::path::Path;

pub async fn create_pool(database_url: &str) -> Result<SqlitePool> {
    // Extract path from URL and create parent directory if needed
    // Handle both "sqlite:" prefix and bare paths
    let path_str = database_url
        .strip_prefix("sqlite:")
        .or_else(|| database_url.strip_prefix("file:"))
        .unwrap_or(database_url);

    let db_path = Path::new(path_str);

    if let Some(parent) = db_path.parent() {
        if !parent.as_os_str().is_empty() {
            tracing::info!("Creating database directory: {:?}", parent);
            std::fs::create_dir_all(parent)?;
        }
    }

    // Create empty database file if it doesn't exist
    // SQLite needs the file to exist before connecting
    if !db_path.exists() {
        tracing::info!("Creating empty database file: {:?}", db_path);
        std::fs::File::create(db_path)?;
    }

    tracing::info!("Connecting to database: {}", database_url);

    // Try simple connection first
    let pool = SqlitePool::connect(database_url).await?;

    tracing::info!("Connected successfully, running migrations...");

    // Run migrations
    sqlx::migrate!("./migrations").run(&pool).await?;

    Ok(pool)
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::NamedTempFile;

    #[tokio::test]
    async fn database_connection_can_be_established() {
        // Create a temporary database file
        let temp_file = NamedTempFile::new().expect("Failed to create temp file");
        let db_url = format!("sqlite:{}", temp_file.path().to_str().unwrap());

        let pool = create_pool(&db_url).await;

        assert!(pool.is_ok());
    }
}
