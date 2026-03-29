use crate::models::recipe::{CreateRecipe, Recipe, UpdateRecipe};
use sqlx::SqlitePool;

/// Erstellt ein neues Rezept in der Datenbank und gibt die ID zurück.
pub async fn create_recipe(pool: &SqlitePool, recipe: &CreateRecipe) -> Result<i64, sqlx::Error> {
    let categories_json = recipe.categories_json();
    let planned_date = recipe.parsed_date();

    let result = sqlx::query(
        r#"
        INSERT INTO recipes (title, categories, ingredients, instructions, planned_date, rating)
        VALUES (?1, ?2, ?3, ?4, ?5, ?6)
        "#,
    )
    .bind(&recipe.title)
    .bind(&categories_json)
    .bind(&recipe.ingredients)
    .bind(&recipe.instructions)
    .bind(planned_date)
    .bind(recipe.rating)
    .execute(pool)
    .await?;

    Ok(result.last_insert_rowid())
}

/// Gibt ein Rezept anhand seiner ID zurück, oder `None` wenn es nicht gefunden wird.
pub async fn get_recipe_by_id(pool: &SqlitePool, id: i64) -> Result<Option<Recipe>, sqlx::Error> {
    let recipe = sqlx::query_as::<_, Recipe>(
        r#"
        SELECT id, title, categories, ingredients, instructions, planned_date, created_at, updated_at, rating
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
        SELECT id, title, categories, ingredients, instructions, planned_date, created_at, updated_at, rating
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
    let planned_date = recipe.parsed_date();

    let rows_affected = sqlx::query(
        r#"
        UPDATE recipes
        SET title = ?1,
            categories = ?2,
            ingredients = ?3,
            instructions = ?4,
            planned_date = ?5,
            rating = ?6,
            updated_at = CURRENT_TIMESTAMP
        WHERE id = ?7
        "#,
    )
    .bind(&recipe.title)
    .bind(&categories_json)
    .bind(&recipe.ingredients)
    .bind(&recipe.instructions)
    .bind(planned_date)
    .bind(recipe.rating)
    .bind(id)
    .execute(pool)
    .await?
    .rows_affected();

    if rows_affected == 0 {
        return Err(sqlx::Error::RowNotFound);
    }

    Ok(())
}

/// Aktualisiert nur das Rating-Feld eines Rezepts. Gibt `RowNotFound` zurück, wenn die ID nicht existiert.
pub async fn update_recipe_rating(
    pool: &SqlitePool,
    id: i64,
    rating: Option<i32>,
) -> Result<(), sqlx::Error> {
    let rows_affected = sqlx::query(
        r#"
        UPDATE recipes
        SET rating = ?1, updated_at = CURRENT_TIMESTAMP
        WHERE id = ?2
        "#,
    )
    .bind(rating)
    .bind(id)
    .execute(pool)
    .await?
    .rows_affected();

    if rows_affected == 0 {
        return Err(sqlx::Error::RowNotFound);
    }

    Ok(())
}

/// Durchsucht alle Rezepte nach einem Suchbegriff in Titel, Zutaten und Anleitung.
/// Die Suche ist case-insensitiv. Bei leerem Suchbegriff werden alle Rezepte zurückgegeben.
/// Ergebnisse sind alphabetisch sortiert (gleiche Logik wie `get_all_recipes`).
pub async fn search_recipes(pool: &SqlitePool, query: &str) -> Result<Vec<Recipe>, sqlx::Error> {
    if query.trim().is_empty() {
        return get_all_recipes(pool).await;
    }

    let term = format!("%{}%", query.to_lowercase());

    let mut recipes = sqlx::query_as::<_, Recipe>(
        r#"
        SELECT id, title, categories, ingredients, instructions, planned_date, created_at, updated_at, rating
        FROM recipes
        WHERE LOWER(title) LIKE ?1
           OR LOWER(ingredients) LIKE ?1
           OR LOWER(instructions) LIKE ?1
        "#,
    )
    .bind(&term)
    .fetch_all(pool)
    .await?;

    recipes.sort_by(|a, b| normalize_for_sort(&a.title).cmp(&normalize_for_sort(&b.title)));

    Ok(recipes)
}

/// Gibt eine statische SQL-Klausel für den Bewertungsfilter zurück.
///
/// - `Some("gut")` → `AND rating >= 3`
/// - `Some("favoriten")` → `AND rating = 5`
/// - Alles andere → leerer String (kein Filter)
///
/// Kein SQL-Injection-Risiko: Die Klausel vergleicht nur gegen konstante Integer-Literale.
fn rating_sql_clause(bewertung: Option<&str>) -> &'static str {
    match bewertung {
        Some("gut") => "AND rating >= 3",
        Some("favoriten") => "AND rating = 5",
        _ => "",
    }
}

/// Filtert Rezepte nach Kategorien, Suchbegriff und/oder Bewertung.
///
/// - Alle leer → alle Rezepte
/// - Nur `search_query` → Volltextsuche (wie `search_recipes`)
/// - Nur `categories` → ODER-Logik: Rezept erscheint, wenn mindestens eine Kategorie passt
/// - Beides gesetzt → UND-Verknüpfung: Kategorie-Filter UND Suchbegriff
/// - `bewertung` → zusätzliche SQL-Bedingung (AND-Logik): `gut` = rating >= 3, `favoriten` = rating = 5
///
/// Ergebnisse sind alphabetisch sortiert (deutsche Sortierung mit Umlauten).
pub async fn filter_recipes_by_categories(
    pool: &SqlitePool,
    categories: &[String],
    search_query: &str,
    bewertung: Option<&str>,
) -> Result<Vec<Recipe>, sqlx::Error> {
    let rating_clause = rating_sql_clause(bewertung);

    if categories.is_empty() && rating_clause.is_empty() {
        return search_recipes(pool, search_query).await;
    }

    if categories.is_empty() {
        // Nur Bewertungsfilter (und ggf. Suche)
        let sql = if search_query.trim().is_empty() {
            format!(
                "SELECT id, title, categories, ingredients, instructions, planned_date, created_at, updated_at, rating \
                 FROM recipes WHERE 1=1 {rating_clause}"
            )
        } else {
            format!(
                "SELECT id, title, categories, ingredients, instructions, planned_date, created_at, updated_at, rating \
                 FROM recipes WHERE (LOWER(title) LIKE ? OR LOWER(ingredients) LIKE ? OR LOWER(instructions) LIKE ?) \
                 {rating_clause}"
            )
        };
        let mut query = sqlx::query_as::<_, Recipe>(&sql);
        if !search_query.trim().is_empty() {
            let search_term = format!("%{}%", search_query.to_lowercase());
            query = query
                .bind(search_term.clone())
                .bind(search_term.clone())
                .bind(search_term);
        }
        let mut recipes = query.fetch_all(pool).await?;
        recipes.sort_by(|a, b| normalize_for_sort(&a.title).cmp(&normalize_for_sort(&b.title)));
        return Ok(recipes);
    }

    let category_conditions: Vec<String> = categories
        .iter()
        .map(|_| "LOWER(categories) LIKE ?".to_string())
        .collect();
    let category_clause = category_conditions.join(" OR ");

    let sql = if search_query.trim().is_empty() {
        format!(
            "SELECT id, title, categories, ingredients, instructions, planned_date, created_at, updated_at, rating \
             FROM recipes WHERE ({category_clause}) {rating_clause}"
        )
    } else {
        format!(
            "SELECT id, title, categories, ingredients, instructions, planned_date, created_at, updated_at, rating \
             FROM recipes WHERE ({category_clause}) \
             AND (LOWER(title) LIKE ? OR LOWER(ingredients) LIKE ? OR LOWER(instructions) LIKE ?) \
             {rating_clause}"
        )
    };

    let mut query = sqlx::query_as::<_, Recipe>(&sql);

    for category in categories {
        let pattern = format!("%\"{}\"%", category.to_lowercase());
        query = query.bind(pattern);
    }

    if !search_query.trim().is_empty() {
        let search_term = format!("%{}%", search_query.to_lowercase());
        query = query
            .bind(search_term.clone())
            .bind(search_term.clone())
            .bind(search_term);
    }

    let mut recipes = query.fetch_all(pool).await?;
    recipes.sort_by(|a, b| normalize_for_sort(&a.title).cmp(&normalize_for_sort(&b.title)));

    Ok(recipes)
}

/// Filtert Rezepte nach dem Prinzip "Länger nicht gemacht":
/// - Rezepte mit Zukunftsdatum werden ausgeschlossen
/// - Rezepte ohne Datum (`planned_date IS NULL`) erscheinen zuerst
/// - Dann aufsteigend nach Datum (ältestes zuerst)
/// - Innerhalb gleichen Datums alphabetisch nach Titel (deutsche Sortierung)
///
/// Optional kombinierbar mit Kategorie-Filter (ODER-Logik), Volltextsuche (UND-Logik)
/// und Bewertungsfilter (`bewertung`: `"gut"` = rating >= 3, `"favoriten"` = rating = 5).
pub async fn filter_recipes_not_made_recently(
    pool: &SqlitePool,
    categories: &[String],
    search_query: &str,
    bewertung: Option<&str>,
) -> Result<Vec<Recipe>, sqlx::Error> {
    let category_clause = if categories.is_empty() {
        String::new()
    } else {
        let conditions: Vec<String> = categories
            .iter()
            .map(|_| "LOWER(categories) LIKE ?".to_string())
            .collect();
        format!("AND ({})", conditions.join(" OR "))
    };

    let search_clause = if search_query.trim().is_empty() {
        String::new()
    } else {
        "AND (LOWER(title) LIKE ? OR LOWER(ingredients) LIKE ? OR LOWER(instructions) LIKE ?)"
            .to_string()
    };

    let rating_clause = rating_sql_clause(bewertung);

    let sql = format!(
        "SELECT id, title, categories, ingredients, instructions, planned_date, created_at, updated_at, rating \
         FROM recipes \
         WHERE (planned_date IS NULL OR planned_date <= DATE('now')) \
         {category_clause} {search_clause} {rating_clause} \
         ORDER BY CASE WHEN planned_date IS NULL THEN 0 ELSE 1 END ASC, planned_date ASC"
    );

    let mut query = sqlx::query_as::<_, Recipe>(&sql);

    for category in categories {
        let pattern = format!("%\"{}\"%", category.to_lowercase());
        query = query.bind(pattern);
    }

    if !search_query.trim().is_empty() {
        let term = format!("%{}%", search_query.to_lowercase());
        query = query.bind(term.clone()).bind(term.clone()).bind(term);
    }

    let mut recipes = query.fetch_all(pool).await?;

    // Sekundärsortierung innerhalb gleichen Datums: alphabetisch (deutsche Sortierung)
    recipes.sort_by(|a, b| {
        let date_cmp = a.planned_date.cmp(&b.planned_date);
        if date_cmp == std::cmp::Ordering::Equal {
            normalize_for_sort(&a.title).cmp(&normalize_for_sort(&b.title))
        } else {
            date_cmp
        }
    });

    Ok(recipes)
}

/// Filtert Rezepte nach dem Prinzip "Nächste 7 Tage":
/// - Nur Rezepte mit `planned_date` zwischen heute (inklusive) und heute + 7 Tage (inklusive)
/// - Rezepte ohne Datum, mit Vergangenheitsdatum oder > heute+7 werden ausgeschlossen
/// - Sortierung: chronologisch aufsteigend nach Datum, bei gleichem Datum alphabetisch (deutsche Sortierung)
///
/// Optional kombinierbar mit Kategorie-Filter (ODER-Logik), Volltextsuche (UND-Logik)
/// und Bewertungsfilter (`bewertung`: `"gut"` = rating >= 3, `"favoriten"` = rating = 5).
pub async fn filter_recipes_next_seven_days(
    pool: &SqlitePool,
    categories: &[String],
    search_query: &str,
    bewertung: Option<&str>,
) -> Result<Vec<Recipe>, sqlx::Error> {
    let category_clause = if categories.is_empty() {
        String::new()
    } else {
        let conditions: Vec<String> = categories
            .iter()
            .map(|_| "LOWER(categories) LIKE ?".to_string())
            .collect();
        format!("AND ({})", conditions.join(" OR "))
    };

    let search_clause = if search_query.trim().is_empty() {
        String::new()
    } else {
        "AND (LOWER(title) LIKE ? OR LOWER(ingredients) LIKE ? OR LOWER(instructions) LIKE ?)"
            .to_string()
    };

    let rating_clause = rating_sql_clause(bewertung);

    let sql = format!(
        "SELECT id, title, categories, ingredients, instructions, planned_date, created_at, updated_at, rating \
         FROM recipes \
         WHERE planned_date >= DATE('now') \
           AND planned_date <= DATE('now', '+7 days') \
         {category_clause} {search_clause} {rating_clause} \
         ORDER BY planned_date ASC"
    );

    let mut query = sqlx::query_as::<_, Recipe>(&sql);

    for category in categories {
        let pattern = format!("%\"{}\"%", category.to_lowercase());
        query = query.bind(pattern);
    }

    if !search_query.trim().is_empty() {
        let term = format!("%{}%", search_query.to_lowercase());
        query = query.bind(term.clone()).bind(term.clone()).bind(term);
    }

    let mut recipes = query.fetch_all(pool).await?;

    // Sekundärsortierung innerhalb gleichen Datums: alphabetisch (deutsche Sortierung)
    recipes.sort_by(|a, b| {
        let date_cmp = a.planned_date.cmp(&b.planned_date);
        if date_cmp == std::cmp::Ordering::Equal {
            normalize_for_sort(&a.title).cmp(&normalize_for_sort(&b.title))
        } else {
            date_cmp
        }
    });

    Ok(recipes)
}

/// Gibt alle Rezepte der laufenden Kalenderwoche zurück (Montag bis Sonntag).
/// Sortierung: aufsteigend nach Datum, dann alphabetisch nach Titel.
pub async fn get_recipes_current_week(
    pool: &SqlitePool,
    monday: time::Date,
    sunday: time::Date,
) -> Result<Vec<Recipe>, sqlx::Error> {
    sqlx::query_as::<_, Recipe>(
        r#"
        SELECT id, title, categories, ingredients, instructions, planned_date, created_at, updated_at, rating
        FROM recipes
        WHERE planned_date >= ?1
          AND planned_date <= ?2
        ORDER BY planned_date ASC, title ASC
        "#,
    )
    .bind(monday)
    .bind(sunday)
    .fetch_all(pool)
    .await
}

/// Gibt alle Rezepte im Datumsbereich [gestern, morgen] zurück (für "Heute gekocht"-Ansicht).
/// Sortierung: aufsteigend nach Datum, dann alphabetisch nach Titel.
pub async fn get_recipes_drei_tage(
    pool: &SqlitePool,
    gestern: time::Date,
    morgen: time::Date,
) -> Result<Vec<Recipe>, sqlx::Error> {
    sqlx::query_as::<_, Recipe>(
        r#"
        SELECT id, title, categories, ingredients, instructions, planned_date, created_at, updated_at, rating
        FROM recipes
        WHERE planned_date >= ?1
          AND planned_date <= ?2
        ORDER BY planned_date ASC, title ASC
        "#,
    )
    .bind(gestern)
    .bind(morgen)
    .fetch_all(pool)
    .await
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

/// Leichtgewichtiger Struct für die Duplikaterkennung.
/// Enthält nur die für den Hinweis benötigten Felder.
pub struct SimilarRecipe {
    pub id: i64,
    pub title: String,
    pub rating: Option<i32>,
}

/// Ein Paar potentiell doppelter Rezepte.
pub struct DublettenPaar {
    pub rezept_a: SimilarRecipe,
    pub rezept_b: SimilarRecipe,
}

/// Findet alle potentiellen Dubletten-Paare in der Datenbank.
///
/// Algorithmus:
/// 1. Alle Rezepte alphabetisch laden
/// 2. Für jedes Rezept `find_similar_recipes` aufrufen
/// 3. Paare als (min(id_a, id_b), max(id_a, id_b)) deduplizieren
/// 4. Reihenfolge: erstes Auftreten (alphabetisch nach Titel von Rezept A)
pub async fn find_all_duplicate_pairs(
    pool: &SqlitePool,
) -> Result<Vec<DublettenPaar>, sqlx::Error> {
    let all_recipes = get_all_recipes(pool).await?;
    let mut seen: std::collections::HashSet<(i64, i64)> = std::collections::HashSet::new();
    let mut result: Vec<DublettenPaar> = Vec::new();

    for recipe in &all_recipes {
        let candidates = find_similar_recipes(pool, &recipe.title, Some(recipe.id)).await?;
        for kandidat in candidates {
            let id_a = recipe.id.min(kandidat.id);
            let id_b = recipe.id.max(kandidat.id);
            if seen.insert((id_a, id_b)) {
                // Rezept A = das alphabetisch frühere (kleinere ID nach Sortierung),
                // aber wir wollen konsistente Darstellung: A = das aktuelle Rezept
                let (rezept_a, rezept_b) = if recipe.id < kandidat.id {
                    (
                        SimilarRecipe {
                            id: recipe.id,
                            title: recipe.title.clone(),
                            rating: recipe.rating,
                        },
                        kandidat,
                    )
                } else {
                    (
                        kandidat,
                        SimilarRecipe {
                            id: recipe.id,
                            title: recipe.title.clone(),
                            rating: recipe.rating,
                        },
                    )
                };
                result.push(DublettenPaar { rezept_a, rezept_b });
            }
        }
    }

    Ok(result)
}

/// Sucht bis zu 3 Rezepte, deren Titel den eingegebenen Begriff enthält
/// (case-insensitive LIKE-Suche). Optional kann eine ID ausgeschlossen werden
/// (für die Bearbeitung eines bestehenden Rezepts).
///
/// Gibt eine leere Liste zurück, wenn der Titel kürzer als 3 Zeichen ist.
pub async fn find_similar_recipes(
    pool: &SqlitePool,
    title: &str,
    exclude_id: Option<i64>,
) -> Result<Vec<SimilarRecipe>, sqlx::Error> {
    if title.trim().len() < 3 {
        return Ok(Vec::new());
    }

    let pattern = format!("%{}%", title.trim().to_lowercase());

    let rows = if let Some(id) = exclude_id {
        sqlx::query_as::<_, (i64, String, Option<i32>)>(
            r#"
            SELECT id, title, rating
            FROM recipes
            WHERE LOWER(title) LIKE ?1
              AND id != ?2
            ORDER BY title ASC
            LIMIT 3
            "#,
        )
        .bind(&pattern)
        .bind(id)
        .fetch_all(pool)
        .await?
    } else {
        sqlx::query_as::<_, (i64, String, Option<i32>)>(
            r#"
            SELECT id, title, rating
            FROM recipes
            WHERE LOWER(title) LIKE ?1
            ORDER BY title ASC
            LIMIT 3
            "#,
        )
        .bind(&pattern)
        .fetch_all(pool)
        .await?
    };

    Ok(rows
        .into_iter()
        .map(|(id, title, rating)| SimilarRecipe { id, title, rating })
        .collect())
}

/// Führt zwei Rezepte zusammen: Aktualisiert das Ziel-Rezept und löscht das Quell-Rezept.
/// Beide Operationen erfolgen in einer atomaren SQLite-Transaktion.
/// Gibt `RowNotFound` zurück, wenn eine der beiden IDs nicht existiert.
pub async fn merge_recipes(
    pool: &SqlitePool,
    source_id: i64,
    target_id: i64,
    merged_data: &UpdateRecipe,
) -> Result<(), sqlx::Error> {
    let mut tx = pool.begin().await?;

    // Beide IDs validieren
    let count: i64 = sqlx::query_scalar("SELECT COUNT(*) FROM recipes WHERE id IN (?1, ?2)")
        .bind(source_id)
        .bind(target_id)
        .fetch_one(&mut *tx)
        .await?;

    if count < 2 {
        return Err(sqlx::Error::RowNotFound);
    }

    // Ziel-Rezept aktualisieren
    let categories_json = merged_data.categories_json();
    let planned_date = merged_data.parsed_date();

    let rows_affected = sqlx::query(
        r#"
        UPDATE recipes
        SET title = ?1,
            categories = ?2,
            ingredients = ?3,
            instructions = ?4,
            planned_date = ?5,
            rating = ?6,
            updated_at = CURRENT_TIMESTAMP
        WHERE id = ?7
        "#,
    )
    .bind(&merged_data.title)
    .bind(&categories_json)
    .bind(&merged_data.ingredients)
    .bind(&merged_data.instructions)
    .bind(planned_date)
    .bind(merged_data.rating)
    .bind(target_id)
    .execute(&mut *tx)
    .await?
    .rows_affected();

    if rows_affected == 0 {
        return Err(sqlx::Error::RowNotFound);
    }

    // Quell-Rezept löschen
    sqlx::query("DELETE FROM recipes WHERE id = ?1")
        .bind(source_id)
        .execute(&mut *tx)
        .await?;

    tx.commit().await?;

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::db::create_pool;
    use tempfile::NamedTempFile;

    fn make_recipe(title: &str, category: &str) -> CreateRecipe {
        CreateRecipe {
            title: title.to_string(),
            categories: vec![category.to_string()],
            ingredients: None,
            instructions: None,
            planned_date_input: None,
            rating: None,
        }
    }

    #[tokio::test]
    async fn get_all_recipes_returns_alphabetically_sorted() {
        let temp_file = NamedTempFile::new().unwrap();
        let db_url = format!("sqlite:{}", temp_file.path().to_str().unwrap());
        let pool = create_pool(&db_url).await.unwrap();

        for title in ["Zupfbrot", "Apfelkuchen", "Bolognese"] {
            create_recipe(&pool, &make_recipe(title, "Mittagessen"))
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
            create_recipe(&pool, &make_recipe(title, "Kuchen"))
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
            create_recipe(&pool, &make_recipe(title, "Mittagessen"))
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
            planned_date_input: None,
            rating: None,
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

        let recipe1 = make_recipe("Rezept 1", "Party");
        let recipe2 = make_recipe("Rezept 2", "Kuchen");

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
            planned_date_input: None,
            rating: None,
        };
        let id = create_recipe(&pool, &recipe).await.unwrap();

        // Dann aktualisieren
        let updated = UpdateRecipe {
            title: "Neuer Titel".to_string(),
            categories: vec!["Party".to_string()],
            ingredients: Some("Neue Zutaten".to_string()),
            instructions: Some("Neue Anleitung".to_string()),
            planned_date_input: None,
            rating: None,
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
            planned_date_input: None,
            rating: None,
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
        let recipe = make_recipe("Original", "Mittagessen");
        let id = create_recipe(&pool, &recipe).await.unwrap();

        let original = get_recipe_by_id(&pool, id).await.unwrap().unwrap();

        // Länger warten und aktualisieren (SQLite Zeitstempel haben Sekunden-Präzision)
        tokio::time::sleep(tokio::time::Duration::from_secs(2)).await;

        let updated = UpdateRecipe {
            title: "Geändert".to_string(),
            categories: vec!["Mittagessen".to_string()],
            ingredients: None,
            instructions: None,
            planned_date_input: None,
            rating: None,
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

        let recipe = make_recipe("Zum Löschen", "Mittagessen");
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
    async fn search_recipes_finds_match_in_title() {
        let temp_file = NamedTempFile::new().unwrap();
        let db_url = format!("sqlite:{}", temp_file.path().to_str().unwrap());
        let pool = create_pool(&db_url).await.unwrap();

        create_recipe(
            &pool,
            &CreateRecipe {
                title: "Spaghetti Bolognese".to_string(),
                categories: vec!["Mittagessen".to_string()],
                ingredients: Some("Hackfleisch, Tomaten".to_string()),
                instructions: Some("Sauce kochen".to_string()),
                planned_date_input: None,
                rating: None,
            },
        )
        .await
        .unwrap();

        let results = search_recipes(&pool, "bolognese").await.unwrap();
        assert_eq!(results.len(), 1);
        assert_eq!(results[0].title, "Spaghetti Bolognese");
    }

    #[tokio::test]
    async fn search_recipes_finds_match_in_ingredients() {
        let temp_file = NamedTempFile::new().unwrap();
        let db_url = format!("sqlite:{}", temp_file.path().to_str().unwrap());
        let pool = create_pool(&db_url).await.unwrap();

        create_recipe(
            &pool,
            &CreateRecipe {
                title: "Pfannkuchen".to_string(),
                categories: vec!["Snacks".to_string()],
                ingredients: Some("Dinkelvollkornmehl, Eier, Milch".to_string()),
                instructions: Some("Teig mischen".to_string()),
                planned_date_input: None,
                rating: None,
            },
        )
        .await
        .unwrap();

        let results = search_recipes(&pool, "dinkel").await.unwrap();
        assert_eq!(results.len(), 1);
        assert_eq!(results[0].title, "Pfannkuchen");
    }

    #[tokio::test]
    async fn search_recipes_finds_match_in_instructions() {
        let temp_file = NamedTempFile::new().unwrap();
        let db_url = format!("sqlite:{}", temp_file.path().to_str().unwrap());
        let pool = create_pool(&db_url).await.unwrap();

        create_recipe(
            &pool,
            &CreateRecipe {
                title: "Brot im Ofen".to_string(),
                categories: vec!["Brot".to_string()],
                ingredients: Some("Mehl, Hefe, Wasser".to_string()),
                instructions: Some("Teig kneten und im Ofen backen".to_string()),
                planned_date_input: None,
                rating: None,
            },
        )
        .await
        .unwrap();

        let results = search_recipes(&pool, "ofen").await.unwrap();
        assert_eq!(results.len(), 1);
        assert_eq!(results[0].title, "Brot im Ofen");
    }

    #[tokio::test]
    async fn search_recipes_is_case_insensitive() {
        let temp_file = NamedTempFile::new().unwrap();
        let db_url = format!("sqlite:{}", temp_file.path().to_str().unwrap());
        let pool = create_pool(&db_url).await.unwrap();

        create_recipe(&pool, &make_recipe("Spaghetti Bolognese", "Mittagessen"))
            .await
            .unwrap();

        let results = search_recipes(&pool, "BOLOGNESE").await.unwrap();
        assert_eq!(results.len(), 1);
        assert_eq!(results[0].title, "Spaghetti Bolognese");
    }

    #[tokio::test]
    async fn search_recipes_returns_empty_for_no_match() {
        let temp_file = NamedTempFile::new().unwrap();
        let db_url = format!("sqlite:{}", temp_file.path().to_str().unwrap());
        let pool = create_pool(&db_url).await.unwrap();

        create_recipe(&pool, &make_recipe("Spaghetti Bolognese", "Mittagessen"))
            .await
            .unwrap();

        let results = search_recipes(&pool, "xyzxyzxyz").await.unwrap();
        assert!(results.is_empty());
    }

    #[tokio::test]
    async fn search_recipes_returns_all_for_empty_query() {
        let temp_file = NamedTempFile::new().unwrap();
        let db_url = format!("sqlite:{}", temp_file.path().to_str().unwrap());
        let pool = create_pool(&db_url).await.unwrap();

        for title in ["Apfelkuchen", "Bolognese", "Zupfbrot"] {
            create_recipe(&pool, &make_recipe(title, "Mittagessen"))
                .await
                .unwrap();
        }

        let results = search_recipes(&pool, "").await.unwrap();
        assert_eq!(results.len(), 3);
    }

    #[tokio::test]
    async fn search_recipes_returns_recipe_only_once_even_if_match_in_multiple_fields() {
        let temp_file = NamedTempFile::new().unwrap();
        let db_url = format!("sqlite:{}", temp_file.path().to_str().unwrap());
        let pool = create_pool(&db_url).await.unwrap();

        create_recipe(
            &pool,
            &CreateRecipe {
                title: "Bolognese Rezept".to_string(),
                categories: vec!["Mittagessen".to_string()],
                ingredients: Some("Bolognese-Soße, Hackfleisch".to_string()),
                instructions: Some("Bolognese zubereiten".to_string()),
                planned_date_input: None,
                rating: None,
            },
        )
        .await
        .unwrap();

        let results = search_recipes(&pool, "bolognese").await.unwrap();
        assert_eq!(results.len(), 1, "Rezept sollte nur einmal erscheinen");
    }

    #[tokio::test]
    async fn filter_by_single_category_returns_matching_recipes() {
        let temp_file = NamedTempFile::new().unwrap();
        let db_url = format!("sqlite:{}", temp_file.path().to_str().unwrap());
        let pool = create_pool(&db_url).await.unwrap();

        create_recipe(&pool, &make_recipe("Vollkornbrot", "Brot"))
            .await
            .unwrap();

        create_recipe(&pool, &make_recipe("Spaghetti", "Mittagessen"))
            .await
            .unwrap();

        let results = filter_recipes_by_categories(&pool, &["Brot".to_string()], "", None)
            .await
            .unwrap();
        assert_eq!(results.len(), 1);
        assert_eq!(results[0].title, "Vollkornbrot");
    }

    #[tokio::test]
    async fn filter_by_multiple_categories_uses_or_logic() {
        let temp_file = NamedTempFile::new().unwrap();
        let db_url = format!("sqlite:{}", temp_file.path().to_str().unwrap());
        let pool = create_pool(&db_url).await.unwrap();

        create_recipe(&pool, &make_recipe("Käsekuchen", "Kuchen"))
            .await
            .unwrap();

        create_recipe(
            &pool,
            &CreateRecipe {
                title: "Partybrot".to_string(),
                categories: vec!["Brot".to_string(), "Party".to_string()],
                ingredients: None,
                instructions: None,
                planned_date_input: None,
                rating: None,
            },
        )
        .await
        .unwrap();

        create_recipe(&pool, &make_recipe("Spaghetti", "Mittagessen"))
            .await
            .unwrap();

        let results = filter_recipes_by_categories(
            &pool,
            &["Kuchen".to_string(), "Brot".to_string()],
            "",
            None,
        )
        .await
        .unwrap();

        let titles: Vec<&str> = results.iter().map(|r| r.title.as_str()).collect();
        assert!(
            titles.contains(&"Käsekuchen"),
            "Käsekuchen sollte enthalten sein"
        );
        assert!(
            titles.contains(&"Partybrot"),
            "Partybrot sollte enthalten sein"
        );
        assert!(
            !titles.contains(&"Spaghetti"),
            "Spaghetti sollte nicht enthalten sein"
        );
    }

    #[tokio::test]
    async fn filter_returns_empty_for_category_without_recipes() {
        let temp_file = NamedTempFile::new().unwrap();
        let db_url = format!("sqlite:{}", temp_file.path().to_str().unwrap());
        let pool = create_pool(&db_url).await.unwrap();

        create_recipe(&pool, &make_recipe("Brot", "Brot"))
            .await
            .unwrap();

        let results = filter_recipes_by_categories(&pool, &["Snacks".to_string()], "", None)
            .await
            .unwrap();
        assert!(results.is_empty());
    }

    #[tokio::test]
    async fn filter_combined_with_search_uses_and_logic() {
        let temp_file = NamedTempFile::new().unwrap();
        let db_url = format!("sqlite:{}", temp_file.path().to_str().unwrap());
        let pool = create_pool(&db_url).await.unwrap();

        create_recipe(&pool, &make_recipe("Dinkelbrot", "Brot"))
            .await
            .unwrap();

        create_recipe(&pool, &make_recipe("Roggenbrot", "Brot"))
            .await
            .unwrap();

        let results = filter_recipes_by_categories(&pool, &["Brot".to_string()], "dinkel", None)
            .await
            .unwrap();

        assert_eq!(results.len(), 1);
        assert_eq!(results[0].title, "Dinkelbrot");
    }

    #[tokio::test]
    async fn filter_with_no_categories_returns_all_recipes() {
        let temp_file = NamedTempFile::new().unwrap();
        let db_url = format!("sqlite:{}", temp_file.path().to_str().unwrap());
        let pool = create_pool(&db_url).await.unwrap();

        for title in ["Apfelkuchen", "Bolognese", "Zupfbrot"] {
            create_recipe(&pool, &make_recipe(title, "Mittagessen"))
                .await
                .unwrap();
        }

        let results = filter_recipes_by_categories(&pool, &[], "", None)
            .await
            .unwrap();
        assert_eq!(results.len(), 3);
    }

    #[tokio::test]
    async fn filter_result_is_alphabetically_sorted() {
        let temp_file = NamedTempFile::new().unwrap();
        let db_url = format!("sqlite:{}", temp_file.path().to_str().unwrap());
        let pool = create_pool(&db_url).await.unwrap();

        for title in ["Zupfbrot", "Apfelbrot", "Mischbrot"] {
            create_recipe(&pool, &make_recipe(title, "Brot"))
                .await
                .unwrap();
        }

        let results = filter_recipes_by_categories(&pool, &["Brot".to_string()], "", None)
            .await
            .unwrap();

        let titles: Vec<&str> = results.iter().map(|r| r.title.as_str()).collect();
        assert_eq!(titles, vec!["Apfelbrot", "Mischbrot", "Zupfbrot"]);
    }

    #[tokio::test]
    async fn delete_recipe_is_idempotent() {
        let temp_file = NamedTempFile::new().unwrap();
        let db_url = format!("sqlite:{}", temp_file.path().to_str().unwrap());
        let pool = create_pool(&db_url).await.unwrap();

        let recipe = make_recipe("Doppelt löschen", "Party");
        let id = create_recipe(&pool, &recipe).await.unwrap();

        delete_recipe(&pool, id).await.unwrap();
        let second_result = delete_recipe(&pool, id).await;
        assert!(matches!(second_result, Err(sqlx::Error::RowNotFound)));
    }

    #[tokio::test]
    async fn create_recipe_with_date_stores_and_retrieves_date() {
        // Given: Ein Rezept mit einem gültigen Datum
        let temp_file = NamedTempFile::new().unwrap();
        let db_url = format!("sqlite:{}", temp_file.path().to_str().unwrap());
        let pool = create_pool(&db_url).await.unwrap();

        let recipe = CreateRecipe {
            title: "Datums-Rezept".to_string(),
            categories: vec!["Mittagessen".to_string()],
            ingredients: None,
            instructions: None,
            planned_date_input: Some("5.3.2025".to_string()),
            rating: None,
        };

        // When: Rezept erstellt und zurückgelesen wird
        let id = create_recipe(&pool, &recipe).await.unwrap();
        let retrieved = get_recipe_by_id(&pool, id).await.unwrap().unwrap();

        // Then: Das gespeicherte Datum stimmt überein
        let date = retrieved.planned_date.unwrap();
        assert_eq!(date.year(), 2025);
        assert_eq!(date.month(), time::Month::March);
        assert_eq!(date.day(), 5);
    }

    #[tokio::test]
    async fn create_recipe_without_date_has_none() {
        // Given: Ein Rezept ohne Datum
        let temp_file = NamedTempFile::new().unwrap();
        let db_url = format!("sqlite:{}", temp_file.path().to_str().unwrap());
        let pool = create_pool(&db_url).await.unwrap();

        // When: Rezept ohne Datum erstellt und zurückgelesen
        let id = create_recipe(&pool, &make_recipe("Ohne Datum", "Mittagessen"))
            .await
            .unwrap();
        let retrieved = get_recipe_by_id(&pool, id).await.unwrap().unwrap();

        // Then: planned_date ist None
        assert!(retrieved.planned_date.is_none());
    }

    #[tokio::test]
    async fn update_recipe_changes_date() {
        // Given: Ein Rezept mit Datum wird erstellt
        let temp_file = NamedTempFile::new().unwrap();
        let db_url = format!("sqlite:{}", temp_file.path().to_str().unwrap());
        let pool = create_pool(&db_url).await.unwrap();

        let recipe = CreateRecipe {
            title: "Rezept mit Datum".to_string(),
            categories: vec!["Mittagessen".to_string()],
            ingredients: None,
            instructions: None,
            planned_date_input: Some("1.1.2025".to_string()),
            rating: None,
        };
        let id = create_recipe(&pool, &recipe).await.unwrap();

        // When: Das Datum wird über Update geändert
        let updated = UpdateRecipe {
            title: "Rezept mit Datum".to_string(),
            categories: vec!["Mittagessen".to_string()],
            ingredients: None,
            instructions: None,
            planned_date_input: Some("15.4.2026".to_string()),
            rating: None,
        };
        update_recipe(&pool, id, &updated).await.unwrap();

        // Then: Das neue Datum wird zurückgegeben
        let retrieved = get_recipe_by_id(&pool, id).await.unwrap().unwrap();
        let date = retrieved.planned_date.unwrap();
        assert_eq!(date.year(), 2026);
        assert_eq!(date.month(), time::Month::April);
        assert_eq!(date.day(), 15);
    }

    #[tokio::test]
    async fn update_recipe_clears_date() {
        // Given: Ein Rezept mit Datum
        let temp_file = NamedTempFile::new().unwrap();
        let db_url = format!("sqlite:{}", temp_file.path().to_str().unwrap());
        let pool = create_pool(&db_url).await.unwrap();

        let recipe = CreateRecipe {
            title: "Rezept mit Datum".to_string(),
            categories: vec!["Mittagessen".to_string()],
            ingredients: None,
            instructions: None,
            planned_date_input: Some("1.1.2025".to_string()),
            rating: None,
        };
        let id = create_recipe(&pool, &recipe).await.unwrap();

        // When: Das Datum wird gelöscht (leere Eingabe)
        let updated = UpdateRecipe {
            title: "Rezept mit Datum".to_string(),
            categories: vec!["Mittagessen".to_string()],
            ingredients: None,
            instructions: None,
            planned_date_input: Some("".to_string()),
            rating: None,
        };
        update_recipe(&pool, id, &updated).await.unwrap();

        // Then: planned_date ist None
        let retrieved = get_recipe_by_id(&pool, id).await.unwrap().unwrap();
        assert!(retrieved.planned_date.is_none());
    }

    fn make_recipe_with_date(title: &str, category: &str, date: Option<&str>) -> CreateRecipe {
        CreateRecipe {
            title: title.to_string(),
            categories: vec![category.to_string()],
            ingredients: None,
            instructions: None,
            planned_date_input: date.map(|d| d.to_string()),
            rating: None,
        }
    }

    #[tokio::test]
    async fn not_made_recently_null_dates_appear_first() {
        // Given: Rezept mit Datum und Rezept ohne Datum
        let temp_file = NamedTempFile::new().unwrap();
        let db_url = format!("sqlite:{}", temp_file.path().to_str().unwrap());
        let pool = create_pool(&db_url).await.unwrap();

        create_recipe(
            &pool,
            &make_recipe_with_date("Mit Datum", "Mittagessen", Some("1.1.2020")),
        )
        .await
        .unwrap();
        create_recipe(
            &pool,
            &make_recipe_with_date("Ohne Datum", "Mittagessen", None),
        )
        .await
        .unwrap();

        // When: Filter "Länger nicht gemacht" aktiv
        let results = filter_recipes_not_made_recently(&pool, &[], "", None)
            .await
            .unwrap();

        // Then: Rezept ohne Datum erscheint als erstes
        assert_eq!(results[0].title, "Ohne Datum");
        assert_eq!(results[1].title, "Mit Datum");
    }

    #[tokio::test]
    async fn not_made_recently_sorted_by_date_ascending() {
        // Given: Drei Rezepte mit verschiedenen Vergangenheitsdaten
        let temp_file = NamedTempFile::new().unwrap();
        let db_url = format!("sqlite:{}", temp_file.path().to_str().unwrap());
        let pool = create_pool(&db_url).await.unwrap();

        create_recipe(
            &pool,
            &make_recipe_with_date("Neueres", "Mittagessen", Some("1.6.2025")),
        )
        .await
        .unwrap();
        create_recipe(
            &pool,
            &make_recipe_with_date("Ältestes", "Mittagessen", Some("1.1.2020")),
        )
        .await
        .unwrap();
        create_recipe(
            &pool,
            &make_recipe_with_date("Mittleres", "Mittagessen", Some("1.6.2022")),
        )
        .await
        .unwrap();

        // When: Filter aktiv
        let results = filter_recipes_not_made_recently(&pool, &[], "", None)
            .await
            .unwrap();

        // Then: Ältestes zuerst, dann aufsteigend
        let titles: Vec<&str> = results.iter().map(|r| r.title.as_str()).collect();
        assert_eq!(titles, vec!["Ältestes", "Mittleres", "Neueres"]);
    }

    #[tokio::test]
    async fn not_made_recently_excludes_future_dates() {
        // Given: Rezept mit Zukunftsdatum und Rezept mit Vergangenheitsdatum
        let temp_file = NamedTempFile::new().unwrap();
        let db_url = format!("sqlite:{}", temp_file.path().to_str().unwrap());
        let pool = create_pool(&db_url).await.unwrap();

        create_recipe(
            &pool,
            &make_recipe_with_date("Vergangenes", "Mittagessen", Some("1.1.2020")),
        )
        .await
        .unwrap();
        create_recipe(
            &pool,
            &make_recipe_with_date("Zukünftiges", "Mittagessen", Some("1.1.2099")),
        )
        .await
        .unwrap();

        // When: Filter aktiv
        let results = filter_recipes_not_made_recently(&pool, &[], "", None)
            .await
            .unwrap();

        // Then: Nur das vergangene Rezept erscheint
        let titles: Vec<&str> = results.iter().map(|r| r.title.as_str()).collect();
        assert!(
            titles.contains(&"Vergangenes"),
            "Vergangenes sollte enthalten sein"
        );
        assert!(
            !titles.contains(&"Zukünftiges"),
            "Zukünftiges sollte ausgeschlossen sein"
        );
    }

    #[tokio::test]
    async fn not_made_recently_includes_past_and_null() {
        // Given: Rezept ohne Datum, mit Vergangenheitsdatum, mit Zukunftsdatum
        let temp_file = NamedTempFile::new().unwrap();
        let db_url = format!("sqlite:{}", temp_file.path().to_str().unwrap());
        let pool = create_pool(&db_url).await.unwrap();

        create_recipe(
            &pool,
            &make_recipe_with_date("Kein Datum", "Mittagessen", None),
        )
        .await
        .unwrap();
        create_recipe(
            &pool,
            &make_recipe_with_date("Vergangen", "Mittagessen", Some("1.1.2020")),
        )
        .await
        .unwrap();
        create_recipe(
            &pool,
            &make_recipe_with_date("Zukunft", "Mittagessen", Some("1.1.2099")),
        )
        .await
        .unwrap();

        // When: Filter aktiv
        let results = filter_recipes_not_made_recently(&pool, &[], "", None)
            .await
            .unwrap();

        // Then: NULL und Vergangen enthalten, Zukunft nicht
        let titles: Vec<&str> = results.iter().map(|r| r.title.as_str()).collect();
        assert_eq!(results.len(), 2);
        assert!(titles.contains(&"Kein Datum"));
        assert!(titles.contains(&"Vergangen"));
    }

    #[tokio::test]
    async fn not_made_recently_returns_empty_if_all_future() {
        // Given: Alle Rezepte haben Zukunftsdaten
        let temp_file = NamedTempFile::new().unwrap();
        let db_url = format!("sqlite:{}", temp_file.path().to_str().unwrap());
        let pool = create_pool(&db_url).await.unwrap();

        create_recipe(
            &pool,
            &make_recipe_with_date("Geplant 1", "Mittagessen", Some("1.1.2099")),
        )
        .await
        .unwrap();
        create_recipe(
            &pool,
            &make_recipe_with_date("Geplant 2", "Mittagessen", Some("1.6.2099")),
        )
        .await
        .unwrap();

        // When: Filter aktiv
        let results = filter_recipes_not_made_recently(&pool, &[], "", None)
            .await
            .unwrap();

        // Then: Leere Liste
        assert!(results.is_empty());
    }

    #[tokio::test]
    async fn not_made_recently_same_date_sorted_alphabetically() {
        // Given: Zwei Rezepte mit gleichem Datum
        let temp_file = NamedTempFile::new().unwrap();
        let db_url = format!("sqlite:{}", temp_file.path().to_str().unwrap());
        let pool = create_pool(&db_url).await.unwrap();

        create_recipe(
            &pool,
            &make_recipe_with_date("Zupfbrot", "Brot", Some("1.1.2020")),
        )
        .await
        .unwrap();
        create_recipe(
            &pool,
            &make_recipe_with_date("Apfelbrot", "Brot", Some("1.1.2020")),
        )
        .await
        .unwrap();

        // When: Filter aktiv
        let results = filter_recipes_not_made_recently(&pool, &[], "", None)
            .await
            .unwrap();

        // Then: Alphabetisch sortiert bei gleichem Datum
        let titles: Vec<&str> = results.iter().map(|r| r.title.as_str()).collect();
        assert_eq!(titles, vec!["Apfelbrot", "Zupfbrot"]);
    }

    #[tokio::test]
    async fn not_made_recently_combined_with_category_filter() {
        // Given: Brot-Rezepte und Mittagessen-Rezept mit Vergangenheitsdaten
        let temp_file = NamedTempFile::new().unwrap();
        let db_url = format!("sqlite:{}", temp_file.path().to_str().unwrap());
        let pool = create_pool(&db_url).await.unwrap();

        create_recipe(
            &pool,
            &make_recipe_with_date("Dinkelbrot", "Brot", Some("1.1.2025")),
        )
        .await
        .unwrap();
        create_recipe(
            &pool,
            &make_recipe_with_date("Roggenbrot", "Brot", Some("1.6.2026")),
        )
        .await
        .unwrap();
        create_recipe(
            &pool,
            &make_recipe_with_date("Spaghetti", "Mittagessen", Some("1.1.2020")),
        )
        .await
        .unwrap();

        // When: Filter "Länger nicht gemacht" + Kategorie "Brot"
        let results = filter_recipes_not_made_recently(&pool, &["Brot".to_string()], "", None)
            .await
            .unwrap();

        // Then: Nur Brot-Rezepte in Datumsreihenfolge (Roggenbrot 2026 noch in Zukunft→ausgeschlossen)
        let titles: Vec<&str> = results.iter().map(|r| r.title.as_str()).collect();
        assert!(
            titles.contains(&"Dinkelbrot"),
            "Dinkelbrot sollte enthalten sein"
        );
        assert!(
            !titles.contains(&"Spaghetti"),
            "Spaghetti (Mittagessen) sollte nicht enthalten sein"
        );
        assert!(
            !titles.contains(&"Roggenbrot"),
            "Roggenbrot (Zukunftsdatum) sollte nicht enthalten sein"
        );
    }

    #[tokio::test]
    async fn not_made_recently_combined_with_search_query() {
        // Given: Zwei Rezepte mit Vergangenheitsdatum
        let temp_file = NamedTempFile::new().unwrap();
        let db_url = format!("sqlite:{}", temp_file.path().to_str().unwrap());
        let pool = create_pool(&db_url).await.unwrap();

        create_recipe(
            &pool,
            &make_recipe_with_date("Dinkelbrot", "Brot", Some("1.1.2020")),
        )
        .await
        .unwrap();
        create_recipe(
            &pool,
            &make_recipe_with_date("Roggenbrot", "Brot", Some("1.6.2022")),
        )
        .await
        .unwrap();

        // When: Filter "Länger nicht gemacht" + Suchbegriff "dinkel"
        let results = filter_recipes_not_made_recently(&pool, &[], "dinkel", None)
            .await
            .unwrap();

        // Then: Nur Dinkelbrot erscheint
        assert_eq!(results.len(), 1);
        assert_eq!(results[0].title, "Dinkelbrot");
    }

    /// Berechnet ein relatives Datum als deutschen Datumsstring (T.M.JJJJ).
    fn date_in_days(n: i64) -> String {
        let today = time::OffsetDateTime::now_utc().date();
        let target = today + time::Duration::days(n);
        format!(
            "{}.{}.{}",
            target.day(),
            target.month() as u8,
            target.year()
        )
    }

    #[tokio::test]
    async fn next_seven_days_returns_recipes_within_window() {
        // Given: Rezept morgen (im Fenster), Rezept in 8 Tagen (außerhalb)
        let temp_file = NamedTempFile::new().unwrap();
        let db_url = format!("sqlite:{}", temp_file.path().to_str().unwrap());
        let pool = create_pool(&db_url).await.unwrap();

        let tomorrow = date_in_days(1);
        let day_eight = date_in_days(8);

        create_recipe(
            &pool,
            &make_recipe_with_date("Morgen-Rezept", "Mittagessen", Some(&tomorrow)),
        )
        .await
        .unwrap();
        create_recipe(
            &pool,
            &make_recipe_with_date("Tag-8-Rezept", "Mittagessen", Some(&day_eight)),
        )
        .await
        .unwrap();

        // When: Filter "Nächste 7 Tage"
        let results = filter_recipes_next_seven_days(&pool, &[], "", None)
            .await
            .unwrap();

        // Then: Nur Morgen-Rezept erscheint
        let titles: Vec<&str> = results.iter().map(|r| r.title.as_str()).collect();
        assert!(
            titles.contains(&"Morgen-Rezept"),
            "Morgen-Rezept sollte enthalten sein"
        );
        assert!(
            !titles.contains(&"Tag-8-Rezept"),
            "Tag-8-Rezept sollte nicht enthalten sein"
        );
    }

    #[tokio::test]
    async fn next_seven_days_includes_today() {
        // Given: Rezept mit heutigem Datum
        let temp_file = NamedTempFile::new().unwrap();
        let db_url = format!("sqlite:{}", temp_file.path().to_str().unwrap());
        let pool = create_pool(&db_url).await.unwrap();

        let today = date_in_days(0);
        create_recipe(
            &pool,
            &make_recipe_with_date("Heute-Rezept", "Mittagessen", Some(&today)),
        )
        .await
        .unwrap();

        // When: Filter aktiv
        let results = filter_recipes_next_seven_days(&pool, &[], "", None)
            .await
            .unwrap();

        // Then: Heute-Rezept erscheint
        let titles: Vec<&str> = results.iter().map(|r| r.title.as_str()).collect();
        assert!(
            titles.contains(&"Heute-Rezept"),
            "Heute-Rezept sollte enthalten sein (inklusive Grenze)"
        );
    }

    #[tokio::test]
    async fn next_seven_days_includes_day_seven() {
        // Given: Rezept in genau 7 Tagen
        let temp_file = NamedTempFile::new().unwrap();
        let db_url = format!("sqlite:{}", temp_file.path().to_str().unwrap());
        let pool = create_pool(&db_url).await.unwrap();

        let day_seven = date_in_days(7);
        create_recipe(
            &pool,
            &make_recipe_with_date("Tag-7-Rezept", "Mittagessen", Some(&day_seven)),
        )
        .await
        .unwrap();

        // When: Filter aktiv
        let results = filter_recipes_next_seven_days(&pool, &[], "", None)
            .await
            .unwrap();

        // Then: Tag-7-Rezept erscheint (inklusive Grenze)
        let titles: Vec<&str> = results.iter().map(|r| r.title.as_str()).collect();
        assert!(
            titles.contains(&"Tag-7-Rezept"),
            "Tag-7-Rezept sollte enthalten sein (inklusive Grenze)"
        );
    }

    #[tokio::test]
    async fn next_seven_days_excludes_past_dates() {
        // Given: Rezept mit Datum gestern
        let temp_file = NamedTempFile::new().unwrap();
        let db_url = format!("sqlite:{}", temp_file.path().to_str().unwrap());
        let pool = create_pool(&db_url).await.unwrap();

        let yesterday = date_in_days(-1);
        create_recipe(
            &pool,
            &make_recipe_with_date("Gestern-Rezept", "Mittagessen", Some(&yesterday)),
        )
        .await
        .unwrap();

        // When: Filter aktiv
        let results = filter_recipes_next_seven_days(&pool, &[], "", None)
            .await
            .unwrap();

        // Then: Gestern-Rezept erscheint nicht
        let titles: Vec<&str> = results.iter().map(|r| r.title.as_str()).collect();
        assert!(
            !titles.contains(&"Gestern-Rezept"),
            "Gestern-Rezept sollte ausgeschlossen sein"
        );
    }

    #[tokio::test]
    async fn next_seven_days_excludes_null_dates() {
        // Given: Rezept ohne Datum
        let temp_file = NamedTempFile::new().unwrap();
        let db_url = format!("sqlite:{}", temp_file.path().to_str().unwrap());
        let pool = create_pool(&db_url).await.unwrap();

        create_recipe(
            &pool,
            &make_recipe_with_date("Ohne-Datum-Rezept", "Mittagessen", None),
        )
        .await
        .unwrap();

        // When: Filter aktiv
        let results = filter_recipes_next_seven_days(&pool, &[], "", None)
            .await
            .unwrap();

        // Then: Rezept ohne Datum erscheint nicht
        let titles: Vec<&str> = results.iter().map(|r| r.title.as_str()).collect();
        assert!(
            !titles.contains(&"Ohne-Datum-Rezept"),
            "Rezept ohne Datum sollte ausgeschlossen sein"
        );
    }

    #[tokio::test]
    async fn next_seven_days_excludes_day_eight() {
        // Given: Rezept in 8 Tagen
        let temp_file = NamedTempFile::new().unwrap();
        let db_url = format!("sqlite:{}", temp_file.path().to_str().unwrap());
        let pool = create_pool(&db_url).await.unwrap();

        let day_eight = date_in_days(8);
        create_recipe(
            &pool,
            &make_recipe_with_date("Tag-8-Rezept", "Mittagessen", Some(&day_eight)),
        )
        .await
        .unwrap();

        // When: Filter aktiv
        let results = filter_recipes_next_seven_days(&pool, &[], "", None)
            .await
            .unwrap();

        // Then: Tag-8-Rezept erscheint nicht
        let titles: Vec<&str> = results.iter().map(|r| r.title.as_str()).collect();
        assert!(
            !titles.contains(&"Tag-8-Rezept"),
            "Tag-8-Rezept sollte ausgeschlossen sein"
        );
    }

    #[tokio::test]
    async fn next_seven_days_sorted_chronologically() {
        // Given: Rezept in 5 Tagen, Rezept in 2 Tagen
        let temp_file = NamedTempFile::new().unwrap();
        let db_url = format!("sqlite:{}", temp_file.path().to_str().unwrap());
        let pool = create_pool(&db_url).await.unwrap();

        let day_five = date_in_days(5);
        let day_two = date_in_days(2);

        create_recipe(
            &pool,
            &make_recipe_with_date("Spaetes-Rezept", "Mittagessen", Some(&day_five)),
        )
        .await
        .unwrap();
        create_recipe(
            &pool,
            &make_recipe_with_date("Fruehes-Rezept", "Mittagessen", Some(&day_two)),
        )
        .await
        .unwrap();

        // When: Filter aktiv
        let results = filter_recipes_next_seven_days(&pool, &[], "", None)
            .await
            .unwrap();

        // Then: Fruehes-Rezept (Tag 2) erscheint vor Spaetes-Rezept (Tag 5)
        let titles: Vec<&str> = results.iter().map(|r| r.title.as_str()).collect();
        let fruehes_idx = titles.iter().position(|&t| t == "Fruehes-Rezept").unwrap();
        let spaetes_idx = titles.iter().position(|&t| t == "Spaetes-Rezept").unwrap();
        assert!(
            fruehes_idx < spaetes_idx,
            "Früheres Datum soll zuerst erscheinen"
        );
    }

    #[tokio::test]
    async fn next_seven_days_same_date_sorted_alphabetically() {
        // Given: Zwei Rezepte am gleichen Tag (Umlaut-Test)
        let temp_file = NamedTempFile::new().unwrap();
        let db_url = format!("sqlite:{}", temp_file.path().to_str().unwrap());
        let pool = create_pool(&db_url).await.unwrap();

        let day_three = date_in_days(3);

        create_recipe(
            &pool,
            &make_recipe_with_date("Bananen-Smoothie", "Snacks", Some(&day_three)),
        )
        .await
        .unwrap();
        create_recipe(
            &pool,
            &make_recipe_with_date("Äpfel-Salat", "Snacks", Some(&day_three)),
        )
        .await
        .unwrap();

        // When: Filter aktiv
        let results = filter_recipes_next_seven_days(&pool, &[], "", None)
            .await
            .unwrap();

        // Then: Äpfel-Salat (Ä wie A) erscheint vor Bananen-Smoothie
        let titles: Vec<&str> = results.iter().map(|r| r.title.as_str()).collect();
        let aepfel_idx = titles.iter().position(|&t| t == "Äpfel-Salat").unwrap();
        let bananen_idx = titles
            .iter()
            .position(|&t| t == "Bananen-Smoothie")
            .unwrap();
        assert!(
            aepfel_idx < bananen_idx,
            "Äpfel-Salat (Ä≈A) soll vor Bananen-Smoothie erscheinen"
        );
    }

    #[tokio::test]
    async fn next_seven_days_combined_with_category_filter() {
        // Given: Brot-Rezept und Mittagessen-Rezept, beide im Fenster
        let temp_file = NamedTempFile::new().unwrap();
        let db_url = format!("sqlite:{}", temp_file.path().to_str().unwrap());
        let pool = create_pool(&db_url).await.unwrap();

        let day_two = date_in_days(2);
        let day_three = date_in_days(3);

        create_recipe(
            &pool,
            &make_recipe_with_date("Dinkelbrot", "Brot", Some(&day_two)),
        )
        .await
        .unwrap();
        create_recipe(
            &pool,
            &make_recipe_with_date("Spaghetti", "Mittagessen", Some(&day_three)),
        )
        .await
        .unwrap();

        // When: Filter "Nächste 7 Tage" + Kategorie "Brot"
        let results = filter_recipes_next_seven_days(&pool, &["Brot".to_string()], "", None)
            .await
            .unwrap();

        // Then: Nur Dinkelbrot (Brot), Spaghetti (Mittagessen) ausgeschlossen
        let titles: Vec<&str> = results.iter().map(|r| r.title.as_str()).collect();
        assert!(
            titles.contains(&"Dinkelbrot"),
            "Dinkelbrot sollte enthalten sein"
        );
        assert!(
            !titles.contains(&"Spaghetti"),
            "Spaghetti (Mittagessen) sollte ausgeschlossen sein"
        );
    }

    #[tokio::test]
    async fn next_seven_days_combined_with_search_query() {
        // Given: Zwei Rezepte im Fenster
        let temp_file = NamedTempFile::new().unwrap();
        let db_url = format!("sqlite:{}", temp_file.path().to_str().unwrap());
        let pool = create_pool(&db_url).await.unwrap();

        let day_two = date_in_days(2);
        let day_three = date_in_days(3);

        create_recipe(
            &pool,
            &make_recipe_with_date("Dinkelbrot", "Brot", Some(&day_two)),
        )
        .await
        .unwrap();
        create_recipe(
            &pool,
            &make_recipe_with_date("Roggenbrot", "Brot", Some(&day_three)),
        )
        .await
        .unwrap();

        // When: Filter "Nächste 7 Tage" + Suchbegriff "dinkel"
        let results = filter_recipes_next_seven_days(&pool, &[], "dinkel", None)
            .await
            .unwrap();

        // Then: Nur Dinkelbrot erscheint
        assert_eq!(results.len(), 1);
        assert_eq!(results[0].title, "Dinkelbrot");
    }

    #[tokio::test]
    async fn next_seven_days_returns_empty_when_no_recipes_in_window() {
        // Given: Alle Rezepte außerhalb des Fensters
        let temp_file = NamedTempFile::new().unwrap();
        let db_url = format!("sqlite:{}", temp_file.path().to_str().unwrap());
        let pool = create_pool(&db_url).await.unwrap();

        let yesterday = date_in_days(-1);
        let day_eight = date_in_days(8);

        create_recipe(
            &pool,
            &make_recipe_with_date("Vergangenes", "Mittagessen", Some(&yesterday)),
        )
        .await
        .unwrap();
        create_recipe(
            &pool,
            &make_recipe_with_date("Zu-Weit-Weg", "Mittagessen", Some(&day_eight)),
        )
        .await
        .unwrap();
        create_recipe(
            &pool,
            &make_recipe_with_date("Kein-Datum", "Mittagessen", None),
        )
        .await
        .unwrap();

        // When: Filter aktiv
        let results = filter_recipes_next_seven_days(&pool, &[], "", None)
            .await
            .unwrap();

        // Then: Leere Liste
        assert!(
            results.is_empty(),
            "Keine Rezepte im 7-Tage-Fenster sollten gefunden werden"
        );
    }

    fn make_recipe_with_rating(title: &str, category: &str, rating: Option<i32>) -> CreateRecipe {
        CreateRecipe {
            title: title.to_string(),
            categories: vec![category.to_string()],
            ingredients: None,
            instructions: None,
            planned_date_input: None,
            rating,
        }
    }

    #[tokio::test]
    async fn rating_filter_gut_returns_only_three_plus_stars() {
        // Given: Rezepte mit 1, 2, 3, 4, 5 Sternen und ohne Bewertung
        let temp_file = NamedTempFile::new().unwrap();
        let db_url = format!("sqlite:{}", temp_file.path().to_str().unwrap());
        let pool = create_pool(&db_url).await.unwrap();

        create_recipe(
            &pool,
            &make_recipe_with_rating("EinStern", "Mittagessen", Some(1)),
        )
        .await
        .unwrap();
        create_recipe(
            &pool,
            &make_recipe_with_rating("ZweiSterne", "Mittagessen", Some(2)),
        )
        .await
        .unwrap();
        create_recipe(
            &pool,
            &make_recipe_with_rating("DreiSterne", "Mittagessen", Some(3)),
        )
        .await
        .unwrap();
        create_recipe(
            &pool,
            &make_recipe_with_rating("VierSterne", "Mittagessen", Some(4)),
        )
        .await
        .unwrap();
        create_recipe(
            &pool,
            &make_recipe_with_rating("FünfSterne", "Mittagessen", Some(5)),
        )
        .await
        .unwrap();
        create_recipe(
            &pool,
            &make_recipe_with_rating("KeineSterne", "Mittagessen", None),
        )
        .await
        .unwrap();

        // When: Filter "gut" angewendet
        let results = filter_recipes_by_categories(&pool, &[], "", Some("gut"))
            .await
            .unwrap();

        // Then: Nur Rezepte mit 3+ Sternen
        let titles: Vec<&str> = results.iter().map(|r| r.title.as_str()).collect();
        assert!(
            titles.contains(&"DreiSterne"),
            "3 Sterne soll enthalten sein"
        );
        assert!(
            titles.contains(&"VierSterne"),
            "4 Sterne soll enthalten sein"
        );
        assert!(
            titles.contains(&"FünfSterne"),
            "5 Sterne soll enthalten sein"
        );
        assert!(
            !titles.contains(&"EinStern"),
            "1 Stern soll nicht enthalten sein"
        );
        assert!(
            !titles.contains(&"ZweiSterne"),
            "2 Sterne sollen nicht enthalten sein"
        );
        assert!(
            !titles.contains(&"KeineSterne"),
            "NULL soll nicht enthalten sein"
        );
    }

    #[tokio::test]
    async fn rating_filter_favoriten_returns_only_five_stars() {
        // Given: Rezepte mit 1-5 Sternen
        let temp_file = NamedTempFile::new().unwrap();
        let db_url = format!("sqlite:{}", temp_file.path().to_str().unwrap());
        let pool = create_pool(&db_url).await.unwrap();

        create_recipe(
            &pool,
            &make_recipe_with_rating("DreiSterne", "Mittagessen", Some(3)),
        )
        .await
        .unwrap();
        create_recipe(
            &pool,
            &make_recipe_with_rating("VierSterne", "Mittagessen", Some(4)),
        )
        .await
        .unwrap();
        create_recipe(
            &pool,
            &make_recipe_with_rating("FünfSterne", "Mittagessen", Some(5)),
        )
        .await
        .unwrap();

        // When: Filter "favoriten" angewendet
        let results = filter_recipes_by_categories(&pool, &[], "", Some("favoriten"))
            .await
            .unwrap();

        // Then: Nur 5-Sterne-Rezept
        let titles: Vec<&str> = results.iter().map(|r| r.title.as_str()).collect();
        assert_eq!(titles, vec!["FünfSterne"]);
    }

    #[tokio::test]
    async fn rating_filter_none_returns_all_recipes() {
        // Given: Rezepte mit verschiedenen Bewertungen
        let temp_file = NamedTempFile::new().unwrap();
        let db_url = format!("sqlite:{}", temp_file.path().to_str().unwrap());
        let pool = create_pool(&db_url).await.unwrap();

        create_recipe(
            &pool,
            &make_recipe_with_rating("Rezept1", "Mittagessen", Some(1)),
        )
        .await
        .unwrap();
        create_recipe(
            &pool,
            &make_recipe_with_rating("Rezept2", "Mittagessen", Some(5)),
        )
        .await
        .unwrap();
        create_recipe(
            &pool,
            &make_recipe_with_rating("Rezept3", "Mittagessen", None),
        )
        .await
        .unwrap();

        // When: Kein Bewertungsfilter
        let results = filter_recipes_by_categories(&pool, &[], "", None)
            .await
            .unwrap();

        // Then: Alle 3 Rezepte
        assert_eq!(results.len(), 3);
    }

    #[tokio::test]
    async fn rating_filter_excludes_unrated_recipes() {
        // Given: Ein Rezept ohne Bewertung
        let temp_file = NamedTempFile::new().unwrap();
        let db_url = format!("sqlite:{}", temp_file.path().to_str().unwrap());
        let pool = create_pool(&db_url).await.unwrap();

        create_recipe(
            &pool,
            &make_recipe_with_rating("UnbewertetesRezept", "Brot", None),
        )
        .await
        .unwrap();

        // When: Filter "gut" angewendet
        let results = filter_recipes_by_categories(&pool, &[], "", Some("gut"))
            .await
            .unwrap();

        // Then: Leere Liste — unbewertete Rezepte werden ausgeblendet
        assert!(results.is_empty(), "NULL-Bewertung soll nicht erscheinen");
    }

    #[tokio::test]
    async fn rating_filter_combined_with_category() {
        // Given: Brot 4 Sterne, Mittagessen 4 Sterne, Brot 1 Stern
        let temp_file = NamedTempFile::new().unwrap();
        let db_url = format!("sqlite:{}", temp_file.path().to_str().unwrap());
        let pool = create_pool(&db_url).await.unwrap();

        create_recipe(
            &pool,
            &make_recipe_with_rating("GutesBrot", "Brot", Some(4)),
        )
        .await
        .unwrap();
        create_recipe(
            &pool,
            &make_recipe_with_rating("GutesMittagessen", "Mittagessen", Some(4)),
        )
        .await
        .unwrap();
        create_recipe(
            &pool,
            &make_recipe_with_rating("SchlechtesBrot", "Brot", Some(1)),
        )
        .await
        .unwrap();

        // When: Filter "gut" + Kategorie "Brot"
        let results = filter_recipes_by_categories(&pool, &["Brot".to_string()], "", Some("gut"))
            .await
            .unwrap();

        // Then: Nur "GutesBrot"
        assert_eq!(results.len(), 1);
        assert_eq!(results[0].title, "GutesBrot");
    }

    #[tokio::test]
    async fn rating_filter_combined_with_search() {
        // Given: "Dinkelbrot" 4 Sterne, "Roggenbrot" 4 Sterne, "Dinkelpfannkuchen" 1 Stern
        let temp_file = NamedTempFile::new().unwrap();
        let db_url = format!("sqlite:{}", temp_file.path().to_str().unwrap());
        let pool = create_pool(&db_url).await.unwrap();

        create_recipe(
            &pool,
            &make_recipe_with_rating("Dinkelbrot", "Brot", Some(4)),
        )
        .await
        .unwrap();
        create_recipe(
            &pool,
            &make_recipe_with_rating("Roggenbrot", "Brot", Some(4)),
        )
        .await
        .unwrap();
        create_recipe(
            &pool,
            &make_recipe_with_rating("Dinkelpfannkuchen", "Snacks", Some(1)),
        )
        .await
        .unwrap();

        // When: Filter "gut" + Suche "dinkel"
        let results = filter_recipes_by_categories(&pool, &[], "dinkel", Some("gut"))
            .await
            .unwrap();

        // Then: Nur "Dinkelbrot"
        assert_eq!(results.len(), 1);
        assert_eq!(results[0].title, "Dinkelbrot");
    }

    #[tokio::test]
    async fn rating_filter_gut_returns_empty_if_no_qualifying_recipes() {
        // Given: Rezepte mit 1 und 2 Sternen
        let temp_file = NamedTempFile::new().unwrap();
        let db_url = format!("sqlite:{}", temp_file.path().to_str().unwrap());
        let pool = create_pool(&db_url).await.unwrap();

        create_recipe(
            &pool,
            &make_recipe_with_rating("EinStern", "Mittagessen", Some(1)),
        )
        .await
        .unwrap();
        create_recipe(
            &pool,
            &make_recipe_with_rating("ZweiSterne", "Mittagessen", Some(2)),
        )
        .await
        .unwrap();

        // When: Filter "gut" angewendet
        let results = filter_recipes_by_categories(&pool, &[], "", Some("gut"))
            .await
            .unwrap();

        // Then: Leere Liste
        assert!(
            results.is_empty(),
            "Kein Rezept mit 3+ Sternen soll erscheinen"
        );
    }

    #[tokio::test]
    async fn rating_filter_favoriten_returns_empty_if_no_five_star() {
        // Given: Nur Rezepte mit 1-4 Sternen
        let temp_file = NamedTempFile::new().unwrap();
        let db_url = format!("sqlite:{}", temp_file.path().to_str().unwrap());
        let pool = create_pool(&db_url).await.unwrap();

        create_recipe(
            &pool,
            &make_recipe_with_rating("VierSterne", "Mittagessen", Some(4)),
        )
        .await
        .unwrap();
        create_recipe(
            &pool,
            &make_recipe_with_rating("DreiSterne", "Mittagessen", Some(3)),
        )
        .await
        .unwrap();

        // When: Filter "favoriten" angewendet
        let results = filter_recipes_by_categories(&pool, &[], "", Some("favoriten"))
            .await
            .unwrap();

        // Then: Leere Liste
        assert!(
            results.is_empty(),
            "Ohne 5-Sterne-Rezept soll Favoriten-Filter leer sein"
        );
    }

    fn make_fixed_date(year: i32, month: u8, day: u8) -> time::Date {
        time::Date::from_calendar_date(year, time::Month::try_from(month).unwrap(), day).unwrap()
    }

    #[tokio::test]
    async fn get_recipes_current_week_returns_recipe_in_week() {
        // Given: Pool mit Rezept in der Woche (Mittwoch)
        let temp_file = NamedTempFile::new().unwrap();
        let db_url = format!("sqlite:{}", temp_file.path().to_str().unwrap());
        let pool = create_pool(&db_url).await.unwrap();

        let monday = make_fixed_date(2026, 3, 30);
        let sunday = make_fixed_date(2026, 4, 5);

        // Rezept am Mittwoch 1.4.2026
        create_recipe(
            &pool,
            &make_recipe_with_date("Mittwoch-Gericht", "Mittagessen", Some("1.4.2026")),
        )
        .await
        .unwrap();

        // When: Abfrage für diese Woche
        let recipes = get_recipes_current_week(&pool, monday, sunday)
            .await
            .unwrap();

        // Then: Rezept wird zurückgegeben
        assert_eq!(recipes.len(), 1);
        assert_eq!(recipes[0].title, "Mittwoch-Gericht");
    }

    #[tokio::test]
    async fn get_recipes_current_week_excludes_recipe_before_monday() {
        // Given: Pool mit Rezept vor Montag (Sonntag der Vorwoche, 29.3.2026)
        let temp_file = NamedTempFile::new().unwrap();
        let db_url = format!("sqlite:{}", temp_file.path().to_str().unwrap());
        let pool = create_pool(&db_url).await.unwrap();

        let monday = make_fixed_date(2026, 3, 30);
        let sunday = make_fixed_date(2026, 4, 5);

        create_recipe(
            &pool,
            &make_recipe_with_date("Letzte-Woche-Gericht", "Mittagessen", Some("29.3.2026")),
        )
        .await
        .unwrap();

        // When: Abfrage für diese Woche
        let recipes = get_recipes_current_week(&pool, monday, sunday)
            .await
            .unwrap();

        // Then: Kein Rezept zurückgegeben
        assert!(
            recipes.is_empty(),
            "Rezept von letzter Woche soll nicht erscheinen"
        );
    }

    #[tokio::test]
    async fn get_recipes_current_week_excludes_recipe_after_sunday() {
        // Given: Pool mit Rezept nach Sonntag (Montag nächste Woche, 6.4.2026)
        let temp_file = NamedTempFile::new().unwrap();
        let db_url = format!("sqlite:{}", temp_file.path().to_str().unwrap());
        let pool = create_pool(&db_url).await.unwrap();

        let monday = make_fixed_date(2026, 3, 30);
        let sunday = make_fixed_date(2026, 4, 5);

        create_recipe(
            &pool,
            &make_recipe_with_date("Naechste-Woche-Gericht", "Mittagessen", Some("6.4.2026")),
        )
        .await
        .unwrap();

        // When: Abfrage für diese Woche
        let recipes = get_recipes_current_week(&pool, monday, sunday)
            .await
            .unwrap();

        // Then: Kein Rezept zurückgegeben
        assert!(
            recipes.is_empty(),
            "Rezept der nächsten Woche soll nicht erscheinen"
        );
    }

    #[tokio::test]
    async fn get_recipes_current_week_excludes_recipe_without_date() {
        // Given: Pool mit Rezept ohne Datum
        let temp_file = NamedTempFile::new().unwrap();
        let db_url = format!("sqlite:{}", temp_file.path().to_str().unwrap());
        let pool = create_pool(&db_url).await.unwrap();

        let monday = make_fixed_date(2026, 3, 30);
        let sunday = make_fixed_date(2026, 4, 5);

        create_recipe(&pool, &make_recipe("Ohne-Datum", "Mittagessen"))
            .await
            .unwrap();

        // When: Abfrage für diese Woche
        let recipes = get_recipes_current_week(&pool, monday, sunday)
            .await
            .unwrap();

        // Then: Kein Rezept zurückgegeben
        assert!(
            recipes.is_empty(),
            "Rezept ohne Datum soll nicht in Wochenvorschau erscheinen"
        );
    }

    #[tokio::test]
    async fn get_recipes_current_week_returns_multiple_recipes_same_day() {
        // Given: Zwei Rezepte am gleichen Tag (Dienstag, 31.3.2026)
        let temp_file = NamedTempFile::new().unwrap();
        let db_url = format!("sqlite:{}", temp_file.path().to_str().unwrap());
        let pool = create_pool(&db_url).await.unwrap();

        let monday = make_fixed_date(2026, 3, 30);
        let sunday = make_fixed_date(2026, 4, 5);

        create_recipe(
            &pool,
            &make_recipe_with_date("Gericht A", "Mittagessen", Some("31.3.2026")),
        )
        .await
        .unwrap();
        create_recipe(
            &pool,
            &make_recipe_with_date("Gericht B", "Mittagessen", Some("31.3.2026")),
        )
        .await
        .unwrap();

        // When: Abfrage für diese Woche
        let recipes = get_recipes_current_week(&pool, monday, sunday)
            .await
            .unwrap();

        // Then: Beide Rezepte zurückgegeben
        assert_eq!(recipes.len(), 2);
        let titles: Vec<&str> = recipes.iter().map(|r| r.title.as_str()).collect();
        assert!(titles.contains(&"Gericht A"));
        assert!(titles.contains(&"Gericht B"));
    }

    #[tokio::test]
    async fn get_recipes_current_week_sorts_by_date_then_title() {
        // Given: Drei Rezepte – zwei am Dienstag 31.3., eines am Montag 30.3.
        let temp_file = NamedTempFile::new().unwrap();
        let db_url = format!("sqlite:{}", temp_file.path().to_str().unwrap());
        let pool = create_pool(&db_url).await.unwrap();

        let monday = make_fixed_date(2026, 3, 30);
        let sunday = make_fixed_date(2026, 4, 5);

        create_recipe(
            &pool,
            &make_recipe_with_date("Zebra-Kuchen", "Kuchen", Some("31.3.2026")),
        )
        .await
        .unwrap();
        create_recipe(
            &pool,
            &make_recipe_with_date("Apfel-Brot", "Brot", Some("31.3.2026")),
        )
        .await
        .unwrap();
        create_recipe(
            &pool,
            &make_recipe_with_date("Montag-Essen", "Mittagessen", Some("30.3.2026")),
        )
        .await
        .unwrap();

        // When: Abfrage für diese Woche
        let recipes = get_recipes_current_week(&pool, monday, sunday)
            .await
            .unwrap();

        // Then: 3 Rezepte, Montag-Essen zuerst, dann Apfel-Brot, dann Zebra-Kuchen
        assert_eq!(recipes.len(), 3);
        assert_eq!(recipes[0].title, "Montag-Essen");
        assert_eq!(recipes[1].title, "Apfel-Brot");
        assert_eq!(recipes[2].title, "Zebra-Kuchen");
    }

    // === Tests für get_recipes_drei_tage ===

    #[tokio::test]
    async fn get_recipes_drei_tage_returns_recipe_for_today() {
        let temp_file = NamedTempFile::new().unwrap();
        let db_url = format!("sqlite:{}", temp_file.path().to_str().unwrap());
        let pool = crate::db::create_pool(&db_url).await.unwrap();

        let today = time::OffsetDateTime::now_utc().date();
        let gestern = today - time::Duration::days(1);
        let morgen = today + time::Duration::days(1);
        let today_str = format!("{}.{}.{}", today.day(), today.month() as u8, today.year());

        create_recipe(
            &pool,
            &make_recipe_with_date("Heute-Rezept", "Mittagessen", Some(&today_str)),
        )
        .await
        .unwrap();

        let recipes = get_recipes_drei_tage(&pool, gestern, morgen).await.unwrap();
        assert_eq!(recipes.len(), 1);
        assert_eq!(recipes[0].title, "Heute-Rezept");
    }

    #[tokio::test]
    async fn get_recipes_drei_tage_returns_recipe_for_gestern() {
        let temp_file = NamedTempFile::new().unwrap();
        let db_url = format!("sqlite:{}", temp_file.path().to_str().unwrap());
        let pool = crate::db::create_pool(&db_url).await.unwrap();

        let today = time::OffsetDateTime::now_utc().date();
        let gestern = today - time::Duration::days(1);
        let morgen = today + time::Duration::days(1);
        let gestern_str = format!(
            "{}.{}.{}",
            gestern.day(),
            gestern.month() as u8,
            gestern.year()
        );

        create_recipe(
            &pool,
            &make_recipe_with_date("Gestern-Rezept", "Mittagessen", Some(&gestern_str)),
        )
        .await
        .unwrap();

        let recipes = get_recipes_drei_tage(&pool, gestern, morgen).await.unwrap();
        assert_eq!(recipes.len(), 1);
        assert_eq!(recipes[0].title, "Gestern-Rezept");
    }

    #[tokio::test]
    async fn get_recipes_drei_tage_returns_recipe_for_morgen() {
        let temp_file = NamedTempFile::new().unwrap();
        let db_url = format!("sqlite:{}", temp_file.path().to_str().unwrap());
        let pool = crate::db::create_pool(&db_url).await.unwrap();

        let today = time::OffsetDateTime::now_utc().date();
        let gestern = today - time::Duration::days(1);
        let morgen = today + time::Duration::days(1);
        let morgen_str = format!(
            "{}.{}.{}",
            morgen.day(),
            morgen.month() as u8,
            morgen.year()
        );

        create_recipe(
            &pool,
            &make_recipe_with_date("Morgen-Rezept", "Mittagessen", Some(&morgen_str)),
        )
        .await
        .unwrap();

        let recipes = get_recipes_drei_tage(&pool, gestern, morgen).await.unwrap();
        assert_eq!(recipes.len(), 1);
        assert_eq!(recipes[0].title, "Morgen-Rezept");
    }

    #[tokio::test]
    async fn get_recipes_drei_tage_excludes_recipe_from_two_days_ago() {
        let temp_file = NamedTempFile::new().unwrap();
        let db_url = format!("sqlite:{}", temp_file.path().to_str().unwrap());
        let pool = crate::db::create_pool(&db_url).await.unwrap();

        let today = time::OffsetDateTime::now_utc().date();
        let gestern = today - time::Duration::days(1);
        let morgen = today + time::Duration::days(1);
        let vorgestern = today - time::Duration::days(2);
        let vorgestern_str = format!(
            "{}.{}.{}",
            vorgestern.day(),
            vorgestern.month() as u8,
            vorgestern.year()
        );

        create_recipe(
            &pool,
            &make_recipe_with_date("Vorgestern-Rezept", "Mittagessen", Some(&vorgestern_str)),
        )
        .await
        .unwrap();

        let recipes = get_recipes_drei_tage(&pool, gestern, morgen).await.unwrap();
        assert!(recipes.is_empty());
    }

    #[tokio::test]
    async fn get_recipes_drei_tage_excludes_recipe_without_date() {
        let temp_file = NamedTempFile::new().unwrap();
        let db_url = format!("sqlite:{}", temp_file.path().to_str().unwrap());
        let pool = crate::db::create_pool(&db_url).await.unwrap();

        let today = time::OffsetDateTime::now_utc().date();
        let gestern = today - time::Duration::days(1);
        let morgen = today + time::Duration::days(1);

        create_recipe(&pool, &make_recipe("Kein-Datum-Rezept", "Mittagessen"))
            .await
            .unwrap();

        let recipes = get_recipes_drei_tage(&pool, gestern, morgen).await.unwrap();
        assert!(recipes.is_empty());
    }

    #[tokio::test]
    async fn get_recipes_drei_tage_returns_multiple_recipes_for_today() {
        let temp_file = NamedTempFile::new().unwrap();
        let db_url = format!("sqlite:{}", temp_file.path().to_str().unwrap());
        let pool = crate::db::create_pool(&db_url).await.unwrap();

        let today = time::OffsetDateTime::now_utc().date();
        let gestern = today - time::Duration::days(1);
        let morgen = today + time::Duration::days(1);
        let today_str = format!("{}.{}.{}", today.day(), today.month() as u8, today.year());

        create_recipe(
            &pool,
            &make_recipe_with_date("Spaghetti", "Mittagessen", Some(&today_str)),
        )
        .await
        .unwrap();
        create_recipe(
            &pool,
            &make_recipe_with_date("Salat", "Mittagessen", Some(&today_str)),
        )
        .await
        .unwrap();

        let recipes = get_recipes_drei_tage(&pool, gestern, morgen).await.unwrap();
        assert_eq!(recipes.len(), 2);
        // Alphabetisch: Salat vor Spaghetti
        assert_eq!(recipes[0].title, "Salat");
        assert_eq!(recipes[1].title, "Spaghetti");
    }

    // === Tests für find_similar_recipes ===

    #[tokio::test]
    async fn find_similar_recipes_returns_empty_for_short_title() {
        // Given: Eine leere Datenbank
        let temp_file = NamedTempFile::new().unwrap();
        let db_url = format!("sqlite:{}", temp_file.path().to_str().unwrap());
        let pool = create_pool(&db_url).await.unwrap();

        // When: Suche mit einem 2-Zeichen-Titel
        let results = find_similar_recipes(&pool, "Di", None).await.unwrap();

        // Then: Leere Liste (keine Suche bei < 3 Zeichen)
        assert!(results.is_empty());
    }

    #[tokio::test]
    async fn find_similar_recipes_finds_substring_match() {
        // Given: Rezept "Dinkelbrot" existiert
        let temp_file = NamedTempFile::new().unwrap();
        let db_url = format!("sqlite:{}", temp_file.path().to_str().unwrap());
        let pool = create_pool(&db_url).await.unwrap();
        create_recipe(&pool, &make_recipe("Dinkelbrot", "Brot"))
            .await
            .unwrap();

        // When: Suche nach Teilstring "Dinkel"
        let results = find_similar_recipes(&pool, "Dinkel", None).await.unwrap();

        // Then: Treffer gefunden
        assert_eq!(results.len(), 1);
        assert_eq!(results[0].title, "Dinkelbrot");
    }

    #[tokio::test]
    async fn find_similar_recipes_is_case_insensitive() {
        // Given: Rezept "Dinkelbrot" existiert
        let temp_file = NamedTempFile::new().unwrap();
        let db_url = format!("sqlite:{}", temp_file.path().to_str().unwrap());
        let pool = create_pool(&db_url).await.unwrap();
        create_recipe(&pool, &make_recipe("Dinkelbrot", "Brot"))
            .await
            .unwrap();

        // When: Suche mit Großbuchstaben "DINKEL"
        let results = find_similar_recipes(&pool, "DINKEL", None).await.unwrap();

        // Then: Treffer gefunden (case-insensitiv)
        assert_eq!(results.len(), 1);
        assert_eq!(results[0].title, "Dinkelbrot");
    }

    #[tokio::test]
    async fn find_similar_recipes_excludes_self() {
        // Given: Rezept "Dinkelbrot" mit bekannter ID
        let temp_file = NamedTempFile::new().unwrap();
        let db_url = format!("sqlite:{}", temp_file.path().to_str().unwrap());
        let pool = create_pool(&db_url).await.unwrap();
        let id = create_recipe(&pool, &make_recipe("Dinkelbrot", "Brot"))
            .await
            .unwrap();

        // When: Suche mit exclude_id = id des Rezepts
        let results = find_similar_recipes(&pool, "Dinkelbrot", Some(id))
            .await
            .unwrap();

        // Then: Kein Treffer (eigenes Rezept ausgeschlossen)
        assert!(results.is_empty());
    }

    #[tokio::test]
    async fn find_similar_recipes_limits_to_three() {
        // Given: 5 Rezepte mit "Dinkel" im Titel
        let temp_file = NamedTempFile::new().unwrap();
        let db_url = format!("sqlite:{}", temp_file.path().to_str().unwrap());
        let pool = create_pool(&db_url).await.unwrap();
        for title in [
            "Dinkelbrot",
            "Dinkelbrötchen",
            "Dinkelvollkornbrot",
            "Dinkelkuchen",
            "Dinkelsuppe",
        ] {
            create_recipe(&pool, &make_recipe(title, "Brot"))
                .await
                .unwrap();
        }

        // When: Suche nach "Dinkel"
        let results = find_similar_recipes(&pool, "Dinkel", None).await.unwrap();

        // Then: Maximal 3 Treffer
        assert_eq!(results.len(), 3);
    }

    #[tokio::test]
    async fn find_similar_recipes_no_match_returns_empty() {
        // Given: Rezept "Spaghetti" existiert
        let temp_file = NamedTempFile::new().unwrap();
        let db_url = format!("sqlite:{}", temp_file.path().to_str().unwrap());
        let pool = create_pool(&db_url).await.unwrap();
        create_recipe(&pool, &make_recipe("Spaghetti", "Mittagessen"))
            .await
            .unwrap();

        // When: Suche nach "Dinkel"
        let results = find_similar_recipes(&pool, "Dinkel", None).await.unwrap();

        // Then: Leere Liste
        assert!(results.is_empty());
    }

    // === Tests für find_all_duplicate_pairs ===

    #[tokio::test]
    async fn find_all_duplicate_pairs_zwei_aehnliche_rezepte_ergeben_ein_paar() {
        // Given: Zwei Rezepte, wobei Titel A ein Substring von Titel B ist
        // (find_similar_recipes sucht per LIKE %title%, A als Suchbegriff findet B)
        let temp_file = NamedTempFile::new().unwrap();
        let db_url = format!("sqlite:{}", temp_file.path().to_str().unwrap());
        let pool = create_pool(&db_url).await.unwrap();

        // "Dinkel" ist Substring von "Dinkelbrot"
        create_recipe(&pool, &make_recipe("Dinkel", "Brot"))
            .await
            .unwrap();
        create_recipe(&pool, &make_recipe("Dinkelbrot", "Brot"))
            .await
            .unwrap();

        // When: Alle Dubletten-Paare finden
        let paare = find_all_duplicate_pairs(&pool).await.unwrap();

        // Then: Genau ein Paar
        assert_eq!(paare.len(), 1);
    }

    #[tokio::test]
    async fn find_all_duplicate_pairs_keine_aehnlichen_rezepte_leere_liste() {
        // Given: Zwei Rezepte ohne Ähnlichkeit
        let temp_file = NamedTempFile::new().unwrap();
        let db_url = format!("sqlite:{}", temp_file.path().to_str().unwrap());
        let pool = create_pool(&db_url).await.unwrap();

        create_recipe(&pool, &make_recipe("Spaghetti Bolognese", "Mittagessen"))
            .await
            .unwrap();
        create_recipe(&pool, &make_recipe("Apfelkuchen", "Kuchen"))
            .await
            .unwrap();

        // When: Alle Dubletten-Paare finden
        let paare = find_all_duplicate_pairs(&pool).await.unwrap();

        // Then: Leere Liste
        assert!(paare.is_empty());
    }

    #[tokio::test]
    async fn find_all_duplicate_pairs_paar_erscheint_nur_einmal() {
        // Given: Titel A ist Substring von Titel B
        // A findet B (LIKE %A% matcht B), B findet A nicht (LIKE %B% matcht A nicht)
        // → Trotzdem nur 1 Paar im Ergebnis (Deduplizierung per geordnetem Paar)
        let temp_file = NamedTempFile::new().unwrap();
        let db_url = format!("sqlite:{}", temp_file.path().to_str().unwrap());
        let pool = create_pool(&db_url).await.unwrap();

        create_recipe(&pool, &make_recipe("Dinkel", "Brot"))
            .await
            .unwrap();
        create_recipe(&pool, &make_recipe("Dinkelbrot", "Brot"))
            .await
            .unwrap();

        // When: Alle Dubletten-Paare finden
        let paare = find_all_duplicate_pairs(&pool).await.unwrap();

        // Then: Genau ein Paar (Deduplizierung, auch wenn nur einseitig gefunden)
        assert_eq!(paare.len(), 1, "Paar soll genau einmal erscheinen");
    }

    #[tokio::test]
    async fn find_all_duplicate_pairs_rezept_nicht_mit_sich_selbst_gepaart() {
        // Given: Ein einzelnes Rezept
        let temp_file = NamedTempFile::new().unwrap();
        let db_url = format!("sqlite:{}", temp_file.path().to_str().unwrap());
        let pool = create_pool(&db_url).await.unwrap();

        create_recipe(&pool, &make_recipe("Dinkelbrot", "Brot"))
            .await
            .unwrap();

        // When: Alle Dubletten-Paare finden
        let paare = find_all_duplicate_pairs(&pool).await.unwrap();

        // Then: Leere Liste (kein Selbst-Paar)
        assert!(
            paare.is_empty(),
            "Rezept darf nicht mit sich selbst gepaart werden"
        );
    }

    #[tokio::test]
    async fn find_all_duplicate_pairs_drei_aehnliche_rezepte_korrekte_paare() {
        // Given: Drei verschachtelte Titel: "Brot" ⊂ "Brotkorb" ⊂ "Brotkorb klein"
        // "Brot" findet "Brotkorb" und "Brotkorb klein"
        // "Brotkorb" findet "Brotkorb klein"
        // → 3 eindeutige Paare
        let temp_file = NamedTempFile::new().unwrap();
        let db_url = format!("sqlite:{}", temp_file.path().to_str().unwrap());
        let pool = create_pool(&db_url).await.unwrap();

        create_recipe(&pool, &make_recipe("Brot", "Brot"))
            .await
            .unwrap();
        create_recipe(&pool, &make_recipe("Brotkorb", "Brot"))
            .await
            .unwrap();
        create_recipe(&pool, &make_recipe("Brotkorb klein", "Brot"))
            .await
            .unwrap();

        // When: Alle Dubletten-Paare finden
        let paare = find_all_duplicate_pairs(&pool).await.unwrap();

        // Then: 3 Paare: (Brot↔Brotkorb), (Brot↔Brotkorb klein), (Brotkorb↔Brotkorb klein)
        assert_eq!(
            paare.len(),
            3,
            "Bei 3 verschachtelt-ähnlichen Rezepten sollen 3 eindeutige Paare entstehen"
        );
    }

    #[tokio::test]
    async fn merge_recipes_success() {
        // Given: Zwei Rezepte existieren
        let temp_file = NamedTempFile::new().unwrap();
        let db_url = format!("sqlite:{}", temp_file.path().to_str().unwrap());
        let pool = create_pool(&db_url).await.unwrap();

        let source_id = create_recipe(
            &pool,
            &CreateRecipe {
                title: "Quell-Rezept".to_string(),
                categories: vec!["Mittagessen".to_string()],
                ingredients: Some("Zutat A".to_string()),
                instructions: None,
                planned_date_input: None,
                rating: None,
            },
        )
        .await
        .unwrap();

        let target_id = create_recipe(
            &pool,
            &CreateRecipe {
                title: "Ziel-Rezept".to_string(),
                categories: vec!["Brot".to_string()],
                ingredients: None,
                instructions: Some("Anleitung B".to_string()),
                planned_date_input: None,
                rating: Some(4),
            },
        )
        .await
        .unwrap();

        // When: Merge durchführen
        let merged_data = UpdateRecipe {
            title: "Zusammengeführt".to_string(),
            categories: vec!["Mittagessen".to_string()],
            ingredients: Some("Zutat A".to_string()),
            instructions: Some("Anleitung B".to_string()),
            planned_date_input: None,
            rating: Some(4),
        };
        merge_recipes(&pool, source_id, target_id, &merged_data)
            .await
            .unwrap();

        // Then: Ziel-Rezept ist aktualisiert
        let target = get_recipe_by_id(&pool, target_id).await.unwrap().unwrap();
        assert_eq!(target.title, "Zusammengeführt");
        assert_eq!(target.ingredients, Some("Zutat A".to_string()));
        assert_eq!(target.instructions, Some("Anleitung B".to_string()));
        assert_eq!(target.rating, Some(4));

        // And: Quell-Rezept ist gelöscht
        let source = get_recipe_by_id(&pool, source_id).await.unwrap();
        assert!(source.is_none());
    }

    #[tokio::test]
    async fn merge_recipes_invalid_source_id() {
        // Given: Nur ein Rezept existiert
        let temp_file = NamedTempFile::new().unwrap();
        let db_url = format!("sqlite:{}", temp_file.path().to_str().unwrap());
        let pool = create_pool(&db_url).await.unwrap();

        let target_id = create_recipe(&pool, &make_recipe("Ziel", "Mittagessen"))
            .await
            .unwrap();

        // When: Merge mit ungültiger source_id
        let merged_data = UpdateRecipe {
            title: "Test".to_string(),
            categories: vec!["Mittagessen".to_string()],
            ingredients: None,
            instructions: None,
            planned_date_input: None,
            rating: None,
        };
        let result = merge_recipes(&pool, 99999, target_id, &merged_data).await;

        // Then: RowNotFound-Fehler
        assert!(matches!(result, Err(sqlx::Error::RowNotFound)));

        // And: Ziel-Rezept ist unverändert
        let target = get_recipe_by_id(&pool, target_id).await.unwrap();
        assert!(target.is_some());
    }

    #[tokio::test]
    async fn merge_recipes_invalid_target_id() {
        // Given: Nur ein Rezept existiert
        let temp_file = NamedTempFile::new().unwrap();
        let db_url = format!("sqlite:{}", temp_file.path().to_str().unwrap());
        let pool = create_pool(&db_url).await.unwrap();

        let source_id = create_recipe(&pool, &make_recipe("Quelle", "Mittagessen"))
            .await
            .unwrap();

        // When: Merge mit ungültiger target_id
        let merged_data = UpdateRecipe {
            title: "Test".to_string(),
            categories: vec!["Mittagessen".to_string()],
            ingredients: None,
            instructions: None,
            planned_date_input: None,
            rating: None,
        };
        let result = merge_recipes(&pool, source_id, 99999, &merged_data).await;

        // Then: RowNotFound-Fehler
        assert!(matches!(result, Err(sqlx::Error::RowNotFound)));

        // And: Quell-Rezept ist noch vorhanden
        let source = get_recipe_by_id(&pool, source_id).await.unwrap();
        assert!(source.is_some());
    }
}
