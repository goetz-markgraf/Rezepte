use crate::models::recipe::{CreateRecipe, Recipe, UpdateRecipe};
use sqlx::SqlitePool;

/// Erstellt ein neues Rezept in der Datenbank und gibt die ID zurück.
pub async fn create_recipe(pool: &SqlitePool, recipe: &CreateRecipe) -> Result<i64, sqlx::Error> {
    let categories_json = recipe.categories_json();

    let result = sqlx::query(
        r#"
        INSERT INTO recipes (title, categories, ingredients, instructions)
        VALUES (?1, ?2, ?3, ?4)
        "#,
    )
    .bind(&recipe.title)
    .bind(&categories_json)
    .bind(&recipe.ingredients)
    .bind(&recipe.instructions)
    .execute(pool)
    .await?;

    Ok(result.last_insert_rowid())
}

/// Gibt ein Rezept anhand seiner ID zurück, oder `None` wenn es nicht gefunden wird.
pub async fn get_recipe_by_id(pool: &SqlitePool, id: i64) -> Result<Option<Recipe>, sqlx::Error> {
    let recipe = sqlx::query_as::<_, Recipe>(
        r#"
        SELECT id, title, categories, ingredients, instructions, created_at, updated_at
        FROM recipes
        WHERE id = ?1
        "#,
    )
    .bind(id)
    .fetch_optional(pool)
    .await?;

    Ok(recipe)
}

/// Normalisiert einen Titel für die deutsche alphabetische Sortierung.
/// Umlaute werden auf ihre Basisvokale reduziert (ä→a, ö→o, ü→u, ß→ss),
/// und das Ergebnis wird in Kleinbuchstaben umgewandelt.
fn normalize_for_sort(title: &str) -> String {
    title
        .chars()
        .flat_map(|c| match c {
            'ä' | 'Ä' => vec!['a'],
            'ö' | 'Ö' => vec!['o'],
            'ü' | 'Ü' => vec!['u'],
            'ß' => vec!['s', 's'],
            other => vec![other.to_lowercase().next().unwrap_or(other)],
        })
        .collect()
}

/// Gibt alle Rezepte zurück, alphabetisch sortiert nach Titel (deutsche Sortierung).
/// Umlaute (ä, ö, ü) werden wie ihre Basisvokale (a, o, u) behandelt.
pub async fn get_all_recipes(pool: &SqlitePool) -> Result<Vec<Recipe>, sqlx::Error> {
    let mut recipes = sqlx::query_as::<_, Recipe>(
        r#"
        SELECT id, title, categories, ingredients, instructions, created_at, updated_at
        FROM recipes
        "#,
    )
    .fetch_all(pool)
    .await?;

    recipes.sort_by(|a, b| normalize_for_sort(&a.title).cmp(&normalize_for_sort(&b.title)));

    Ok(recipes)
}

/// Aktualisiert ein bestehendes Rezept. Gibt `RowNotFound` zurück, wenn die ID nicht existiert.
pub async fn update_recipe(
    pool: &SqlitePool,
    id: i64,
    recipe: &UpdateRecipe,
) -> Result<(), sqlx::Error> {
    let categories_json = recipe.categories_json();

    let rows_affected = sqlx::query(
        r#"
        UPDATE recipes 
        SET title = ?1, 
            categories = ?2, 
            ingredients = ?3, 
            instructions = ?4,
            updated_at = CURRENT_TIMESTAMP
        WHERE id = ?5
        "#,
    )
    .bind(&recipe.title)
    .bind(&categories_json)
    .bind(&recipe.ingredients)
    .bind(&recipe.instructions)
    .bind(id)
    .execute(pool)
    .await?
    .rows_affected();

    if rows_affected == 0 {
        return Err(sqlx::Error::RowNotFound);
    }

    Ok(())
}

/// Löscht ein Rezept anhand seiner ID. Gibt `RowNotFound` zurück, wenn die ID nicht existiert.
pub async fn delete_recipe(pool: &SqlitePool, id: i64) -> Result<(), sqlx::Error> {
    let rows_affected = sqlx::query("DELETE FROM recipes WHERE id = ?1")
        .bind(id)
        .execute(pool)
        .await?
        .rows_affected();

    if rows_affected == 0 {
        return Err(sqlx::Error::RowNotFound);
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::db::create_pool;
    use tempfile::NamedTempFile;

    #[tokio::test]
    async fn get_all_recipes_returns_alphabetically_sorted() {
        let temp_file = NamedTempFile::new().unwrap();
        let db_url = format!("sqlite:{}", temp_file.path().to_str().unwrap());
        let pool = create_pool(&db_url).await.unwrap();

        for title in ["Zupfbrot", "Apfelkuchen", "Bolognese"] {
            create_recipe(
                &pool,
                &CreateRecipe {
                    title: title.to_string(),
                    categories: vec!["Mittagessen".to_string()],
                    ingredients: None,
                    instructions: None,
                },
            )
            .await
            .unwrap();
        }

        let recipes = get_all_recipes(&pool).await.unwrap();
        let titles: Vec<&str> = recipes.iter().map(|r| r.title.as_str()).collect();
        assert_eq!(titles, vec!["Apfelkuchen", "Bolognese", "Zupfbrot"]);
    }

    #[tokio::test]
    async fn get_all_recipes_sorts_case_insensitively() {
        let temp_file = NamedTempFile::new().unwrap();
        let db_url = format!("sqlite:{}", temp_file.path().to_str().unwrap());
        let pool = create_pool(&db_url).await.unwrap();

        for title in ["zitronenkuchen", "Apfelkuchen"] {
            create_recipe(
                &pool,
                &CreateRecipe {
                    title: title.to_string(),
                    categories: vec!["Kuchen".to_string()],
                    ingredients: None,
                    instructions: None,
                },
            )
            .await
            .unwrap();
        }

        let recipes = get_all_recipes(&pool).await.unwrap();
        let titles: Vec<&str> = recipes.iter().map(|r| r.title.as_str()).collect();
        assert_eq!(titles, vec!["Apfelkuchen", "zitronenkuchen"]);
    }

    #[tokio::test]
    async fn get_all_recipes_sorts_umlauts_correctly() {
        let temp_file = NamedTempFile::new().unwrap();
        let db_url = format!("sqlite:{}", temp_file.path().to_str().unwrap());
        let pool = create_pool(&db_url).await.unwrap();

        // ä → wie a: "Ährenbrot" sollte vor "Apfelkuchen" oder direkt danach kommen
        // ü → wie u: "Überbackene Nudeln" nach "U..."
        for title in ["Überbackene Nudeln", "Apfelkuchen", "Ährenbrot"] {
            create_recipe(
                &pool,
                &CreateRecipe {
                    title: title.to_string(),
                    categories: vec!["Mittagessen".to_string()],
                    ingredients: None,
                    instructions: None,
                },
            )
            .await
            .unwrap();
        }

        let recipes = get_all_recipes(&pool).await.unwrap();
        let titles: Vec<&str> = recipes.iter().map(|r| r.title.as_str()).collect();
        // Ä wie A → Ährenbrot kommt vor Apfelkuchen (Äh < Ap bei deutschem Sortieren)
        // Ü wie U → Überbackene Nudeln kommt nach U... aber es gibt kein anderes U-Wort hier
        // Erwartete Reihenfolge: Ährenbrot, Apfelkuchen, Überbackene Nudeln
        assert_eq!(
            titles,
            vec!["Ährenbrot", "Apfelkuchen", "Überbackene Nudeln"]
        );
    }

    #[tokio::test]
    async fn get_all_recipes_returns_empty_for_empty_db() {
        let temp_file = NamedTempFile::new().unwrap();
        let db_url = format!("sqlite:{}", temp_file.path().to_str().unwrap());
        let pool = create_pool(&db_url).await.unwrap();

        let recipes = get_all_recipes(&pool).await.unwrap();
        assert!(recipes.is_empty());
    }

    #[tokio::test]
    async fn can_create_and_retrieve_recipe() {
        let temp_file = NamedTempFile::new().unwrap();
        let db_url = format!("sqlite:{}", temp_file.path().to_str().unwrap());
        let pool = create_pool(&db_url).await.unwrap();

        let recipe = CreateRecipe {
            title: "Test Rezept".to_string(),
            categories: vec!["Mittagessen".to_string()],
            ingredients: Some("Zutat 1, Zutat 2".to_string()),
            instructions: Some("Anleitung".to_string()),
        };

        let id = create_recipe(&pool, &recipe).await.unwrap();
        assert!(id > 0);

        let retrieved = get_recipe_by_id(&pool, id).await.unwrap().unwrap();
        assert_eq!(retrieved.title, "Test Rezept");
        assert_eq!(retrieved.categories_vec(), vec!["Mittagessen"]);
    }

    #[tokio::test]
    async fn get_recipe_by_id_returns_none_for_nonexistent() {
        let temp_file = NamedTempFile::new().unwrap();
        let db_url = format!("sqlite:{}", temp_file.path().to_str().unwrap());
        let pool = create_pool(&db_url).await.unwrap();

        let result = get_recipe_by_id(&pool, 9999).await.unwrap();
        assert!(result.is_none());
    }

    #[tokio::test]
    async fn can_get_all_recipes() {
        let temp_file = NamedTempFile::new().unwrap();
        let db_url = format!("sqlite:{}", temp_file.path().to_str().unwrap());
        let pool = create_pool(&db_url).await.unwrap();

        let recipe1 = CreateRecipe {
            title: "Rezept 1".to_string(),
            categories: vec!["Party".to_string()],
            ingredients: None,
            instructions: None,
        };

        let recipe2 = CreateRecipe {
            title: "Rezept 2".to_string(),
            categories: vec!["Kuchen".to_string()],
            ingredients: None,
            instructions: None,
        };

        create_recipe(&pool, &recipe1).await.unwrap();
        create_recipe(&pool, &recipe2).await.unwrap();

        let recipes = get_all_recipes(&pool).await.unwrap();
        assert_eq!(recipes.len(), 2);
    }

    #[tokio::test]
    async fn can_update_recipe() {
        let temp_file = NamedTempFile::new().unwrap();
        let db_url = format!("sqlite:{}", temp_file.path().to_str().unwrap());
        let pool = create_pool(&db_url).await.unwrap();

        // Erst ein Rezept erstellen
        let recipe = CreateRecipe {
            title: "Original Titel".to_string(),
            categories: vec!["Mittagessen".to_string()],
            ingredients: Some("Original Zutaten".to_string()),
            instructions: Some("Original Anleitung".to_string()),
        };
        let id = create_recipe(&pool, &recipe).await.unwrap();

        // Dann aktualisieren
        let updated = UpdateRecipe {
            title: "Neuer Titel".to_string(),
            categories: vec!["Party".to_string()],
            ingredients: Some("Neue Zutaten".to_string()),
            instructions: Some("Neue Anleitung".to_string()),
        };

        update_recipe(&pool, id, &updated).await.unwrap();

        // Prüfen
        let retrieved = get_recipe_by_id(&pool, id).await.unwrap().unwrap();
        assert_eq!(retrieved.title, "Neuer Titel");
        assert_eq!(retrieved.categories_vec(), vec!["Party"]);
        assert_eq!(retrieved.ingredients, Some("Neue Zutaten".to_string()));
    }

    #[tokio::test]
    async fn update_recipe_returns_error_for_nonexistent() {
        let temp_file = NamedTempFile::new().unwrap();
        let db_url = format!("sqlite:{}", temp_file.path().to_str().unwrap());
        let pool = create_pool(&db_url).await.unwrap();

        let updated = UpdateRecipe {
            title: "Neuer Titel".to_string(),
            categories: vec!["Party".to_string()],
            ingredients: None,
            instructions: None,
        };

        let result = update_recipe(&pool, 9999, &updated).await;
        assert!(matches!(result, Err(sqlx::Error::RowNotFound)));
    }

    #[tokio::test]
    async fn update_recipe_updates_timestamp() {
        let temp_file = NamedTempFile::new().unwrap();
        let db_url = format!("sqlite:{}", temp_file.path().to_str().unwrap());
        let pool = create_pool(&db_url).await.unwrap();

        // Rezept erstellen
        let recipe = CreateRecipe {
            title: "Original".to_string(),
            categories: vec!["Mittagessen".to_string()],
            ingredients: None,
            instructions: None,
        };
        let id = create_recipe(&pool, &recipe).await.unwrap();

        let original = get_recipe_by_id(&pool, id).await.unwrap().unwrap();

        // Länger warten und aktualisieren (SQLite Zeitstempel haben Sekunden-Präzision)
        tokio::time::sleep(tokio::time::Duration::from_secs(2)).await;

        let updated = UpdateRecipe {
            title: "Geändert".to_string(),
            categories: vec!["Mittagessen".to_string()],
            ingredients: None,
            instructions: None,
        };
        update_recipe(&pool, id, &updated).await.unwrap();

        let retrieved = get_recipe_by_id(&pool, id).await.unwrap().unwrap();
        assert_ne!(original.updated_at, retrieved.updated_at);
    }

    #[tokio::test]
    async fn can_delete_recipe() {
        let temp_file = NamedTempFile::new().unwrap();
        let db_url = format!("sqlite:{}", temp_file.path().to_str().unwrap());
        let pool = create_pool(&db_url).await.unwrap();

        let recipe = CreateRecipe {
            title: "Zum Löschen".to_string(),
            categories: vec!["Mittagessen".to_string()],
            ingredients: None,
            instructions: None,
        };
        let id = create_recipe(&pool, &recipe).await.unwrap();

        delete_recipe(&pool, id).await.unwrap();

        let result = get_recipe_by_id(&pool, id).await.unwrap();
        assert!(result.is_none());
    }

    #[tokio::test]
    async fn delete_recipe_returns_error_for_nonexistent() {
        let temp_file = NamedTempFile::new().unwrap();
        let db_url = format!("sqlite:{}", temp_file.path().to_str().unwrap());
        let pool = create_pool(&db_url).await.unwrap();

        let result = delete_recipe(&pool, 9999).await;
        assert!(matches!(result, Err(sqlx::Error::RowNotFound)));
    }

    #[tokio::test]
    async fn delete_recipe_is_idempotent() {
        let temp_file = NamedTempFile::new().unwrap();
        let db_url = format!("sqlite:{}", temp_file.path().to_str().unwrap());
        let pool = create_pool(&db_url).await.unwrap();

        let recipe = CreateRecipe {
            title: "Doppelt löschen".to_string(),
            categories: vec!["Party".to_string()],
            ingredients: None,
            instructions: None,
        };
        let id = create_recipe(&pool, &recipe).await.unwrap();

        delete_recipe(&pool, id).await.unwrap();
        let second_result = delete_recipe(&pool, id).await;
        assert!(matches!(second_result, Err(sqlx::Error::RowNotFound)));
    }
}
