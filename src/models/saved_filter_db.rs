use crate::models::saved_filter::{CreateSavedFilter, SavedFilter};
use sqlx::SqlitePool;

/// Gibt alle gespeicherten Filter zurück, sortiert nach Erstellungsdatum (älteste zuerst).
pub async fn get_all_saved_filters(pool: &SqlitePool) -> Result<Vec<SavedFilter>, sqlx::Error> {
    sqlx::query_as::<_, SavedFilter>(
        r#"
        SELECT id, name, query_string, created_at
        FROM saved_filters
        ORDER BY created_at ASC
        "#,
    )
    .fetch_all(pool)
    .await
}

/// Erstellt einen neuen gespeicherten Filter und gibt die ID zurück.
/// Gibt `sqlx::Error::Database` zurück, wenn ein Filter mit demselben Namen bereits existiert.
pub async fn create_saved_filter(
    pool: &SqlitePool,
    filter: &CreateSavedFilter,
) -> Result<i64, sqlx::Error> {
    let result = sqlx::query(
        r#"
        INSERT INTO saved_filters (name, query_string)
        VALUES (?1, ?2)
        "#,
    )
    .bind(&filter.name)
    .bind(&filter.query_string)
    .execute(pool)
    .await?;

    Ok(result.last_insert_rowid())
}

/// Löscht einen gespeicherten Filter anhand seiner ID.
/// Gibt `sqlx::Error::RowNotFound` zurück, wenn die ID nicht existiert.
pub async fn delete_saved_filter(pool: &SqlitePool, id: i64) -> Result<(), sqlx::Error> {
    let rows_affected = sqlx::query(
        r#"
        DELETE FROM saved_filters WHERE id = ?1
        "#,
    )
    .bind(id)
    .execute(pool)
    .await?
    .rows_affected();

    if rows_affected == 0 {
        return Err(sqlx::Error::RowNotFound);
    }

    Ok(())
}

/// Benennt einen gespeicherten Filter um.
/// Gibt `sqlx::Error::RowNotFound` zurück, wenn die ID nicht existiert.
#[allow(dead_code)]
pub async fn update_saved_filter_name(
    pool: &SqlitePool,
    id: i64,
    new_name: &str,
) -> Result<(), sqlx::Error> {
    let rows_affected = sqlx::query(
        r#"
        UPDATE saved_filters SET name = ?1 WHERE id = ?2
        "#,
    )
    .bind(new_name)
    .bind(id)
    .execute(pool)
    .await?
    .rows_affected();

    if rows_affected == 0 {
        return Err(sqlx::Error::RowNotFound);
    }

    Ok(())
}
