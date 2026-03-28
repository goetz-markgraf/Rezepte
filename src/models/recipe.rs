use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use time::Date;

pub const VALID_CATEGORIES: &[&str] = &["Mittagessen", "Brot", "Party", "Kuchen", "Snacks"];

/// Parst ein deutsches Datumsformat (T.M.JJJJ oder T.M.JJ) in ein `time::Date`.
/// Akzeptiert führende Nullen optional. Zweistellige Jahre werden als 20xx interpretiert.
/// Gibt `Err` mit einer deutschen Fehlermeldung zurück, wenn das Format ungültig ist.
pub fn parse_german_date(input: &str) -> Result<Date, String> {
    let parts: Vec<&str> = input.trim().split('.').collect();
    if parts.len() != 3 {
        return Err("Kein gültiges Datum. Bitte im Format T.M.JJJJ eingeben.".to_string());
    }

    let day: u8 = parts[0]
        .parse()
        .map_err(|_| "Kein gültiges Datum. Bitte im Format T.M.JJJJ eingeben.".to_string())?;
    let month: u8 = parts[1]
        .parse()
        .map_err(|_| "Kein gültiges Datum. Bitte im Format T.M.JJJJ eingeben.".to_string())?;
    let raw_year: u16 = parts[2]
        .parse()
        .map_err(|_| "Kein gültiges Datum. Bitte im Format T.M.JJJJ eingeben.".to_string())?;

    let year = if raw_year < 100 {
        raw_year + 2000
    } else {
        raw_year
    };

    let month = time::Month::try_from(month)
        .map_err(|_| "Kein gültiges Datum. Bitte im Format T.M.JJJJ eingeben.".to_string())?;

    Date::from_calendar_date(year as i32, month, day)
        .map_err(|_| "Kein gültiges Datum. Bitte im Format T.M.JJJJ eingeben.".to_string())
}

/// Validiert die gemeinsamen Felder eines Rezept-Formulars (Titel, Kategorien, Zutaten, Anleitung).
/// Gibt eine Liste von Fehlermeldungen zurück, falls die Validierung fehlschlägt.
pub fn validate_recipe_fields(
    title: &str,
    categories: &[String],
    ingredients: Option<&str>,
    instructions: Option<&str>,
) -> Vec<String> {
    let mut errors = Vec::new();

    if title.trim().is_empty() {
        errors.push("Titel ist erforderlich".to_string());
    } else if title.len() > 100 {
        errors.push("Titel darf maximal 100 Zeichen haben".to_string());
    }

    if categories.is_empty() {
        errors.push("Mindestens eine Kategorie muss ausgewählt werden".to_string());
    } else {
        for cat in categories {
            if !VALID_CATEGORIES.contains(&cat.as_str()) {
                errors.push(format!("'{}' ist keine gültige Kategorie", cat));
            }
        }
    }

    if let Some(ingredients) = ingredients {
        if ingredients.len() > 2000 {
            errors.push("Zutaten dürfen maximal 2000 Zeichen haben".to_string());
        }
    }

    if let Some(instructions) = instructions {
        if instructions.len() > 5000 {
            errors.push("Anleitung darf maximal 5000 Zeichen haben".to_string());
        }
    }

    errors
}

#[derive(Debug, FromRow, Serialize, Deserialize)]
pub struct Recipe {
    pub id: i64,
    pub title: String,
    pub categories: String,
    pub ingredients: Option<String>,
    pub instructions: Option<String>,
    pub planned_date: Option<Date>,
    pub created_at: String,
    pub updated_at: String,
}

impl Recipe {
    pub fn categories_vec(&self) -> Vec<String> {
        serde_json::from_str(&self.categories).unwrap_or_default()
    }
}

#[derive(Debug, Deserialize)]
pub struct CreateRecipe {
    pub title: String,
    pub categories: Vec<String>,
    pub ingredients: Option<String>,
    pub instructions: Option<String>,
    /// Roheingabe des Datums aus dem Formular (deutsches Format T.M.JJJJ oder leer).
    pub planned_date_input: Option<String>,
}

impl CreateRecipe {
    /// Validiert das Rezept-Formular und gibt bei Fehlern eine Liste von Fehlermeldungen zurück.
    pub fn validate(&self) -> Result<(), Vec<String>> {
        let mut errors = validate_recipe_fields(
            &self.title,
            &self.categories,
            self.ingredients.as_deref(),
            self.instructions.as_deref(),
        );

        if let Some(ref input) = self.planned_date_input {
            if !input.trim().is_empty() {
                if let Err(e) = parse_german_date(input) {
                    errors.push(e);
                }
            }
        }

        if errors.is_empty() {
            Ok(())
        } else {
            Err(errors)
        }
    }

    /// Gibt die Kategorien als JSON-Array-String zurück.
    pub fn categories_json(&self) -> String {
        serde_json::to_string(&self.categories).unwrap_or_else(|_| "[]".to_string())
    }

    /// Parst die Datumseingabe und gibt `Some(Date)` bei gültigem Datum oder `None` zurück.
    pub fn parsed_date(&self) -> Option<Date> {
        self.planned_date_input
            .as_ref()
            .filter(|s| !s.trim().is_empty())
            .and_then(|s| parse_german_date(s).ok())
    }
}

#[derive(Debug, Deserialize)]
pub struct UpdateRecipe {
    pub title: String,
    pub categories: Vec<String>,
    pub ingredients: Option<String>,
    pub instructions: Option<String>,
    /// Roheingabe des Datums aus dem Formular (deutsches Format T.M.JJJJ oder leer).
    pub planned_date_input: Option<String>,
}

impl UpdateRecipe {
    /// Validiert das Rezept-Formular und gibt bei Fehlern eine Liste von Fehlermeldungen zurück.
    pub fn validate(&self) -> Result<(), Vec<String>> {
        let mut errors = validate_recipe_fields(
            &self.title,
            &self.categories,
            self.ingredients.as_deref(),
            self.instructions.as_deref(),
        );

        if let Some(ref input) = self.planned_date_input {
            if !input.trim().is_empty() {
                if let Err(e) = parse_german_date(input) {
                    errors.push(e);
                }
            }
        }

        if errors.is_empty() {
            Ok(())
        } else {
            Err(errors)
        }
    }

    /// Gibt die Kategorien als JSON-Array-String zurück.
    pub fn categories_json(&self) -> String {
        serde_json::to_string(&self.categories).unwrap_or_else(|_| "[]".to_string())
    }

    /// Parst die Datumseingabe und gibt `Some(Date)` bei gültigem Datum oder `None` zurück.
    pub fn parsed_date(&self) -> Option<Date> {
        self.planned_date_input
            .as_ref()
            .filter(|s| !s.trim().is_empty())
            .and_then(|s| parse_german_date(s).ok())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use time::Month;

    #[test]
    fn parse_german_date_parses_short_format() {
        // Given: Datum im Format T.M.JJJJ ohne führende Nullen
        let result = parse_german_date("5.3.2025");
        // Then: Erfolgreich als 2025-03-05 geparst
        assert!(result.is_ok());
        let date = result.unwrap();
        assert_eq!(date.year(), 2025);
        assert_eq!(date.month(), Month::March);
        assert_eq!(date.day(), 5);
    }

    #[test]
    fn parse_german_date_parses_long_format_with_leading_zeros() {
        // Given: Datum im Format TT.MM.JJJJ mit führenden Nullen
        let result = parse_german_date("05.03.2025");
        // Then: Identisches Datum wie ohne führende Nullen
        assert!(result.is_ok());
        let date = result.unwrap();
        assert_eq!(date.year(), 2025);
        assert_eq!(date.month(), Month::March);
        assert_eq!(date.day(), 5);
    }

    #[test]
    fn parse_german_date_parses_two_digit_year() {
        // Given: Datum mit zweistelliger Jahreszahl
        let result = parse_german_date("5.3.25");
        // Then: Jahr wird als 2025 interpretiert
        assert!(result.is_ok());
        let date = result.unwrap();
        assert_eq!(date.year(), 2025);
    }

    #[test]
    fn parse_german_date_rejects_invalid_text() {
        // Given: Kein Datum sondern ein Wort
        let result = parse_german_date("morgen");
        // Then: Fehler zurückgegeben
        assert!(result.is_err());
        assert!(result.unwrap_err().contains("Kein gültiges Datum"));
    }

    #[test]
    fn parse_german_date_rejects_invalid_day() {
        // Given: Tag 32 existiert nicht
        let result = parse_german_date("32.1.2025");
        // Then: Fehler zurückgegeben
        assert!(result.is_err());
    }

    #[test]
    fn create_recipe_validates_title() {
        let recipe = CreateRecipe {
            title: "".to_string(),
            categories: vec!["Mittagessen".to_string()],
            ingredients: None,
            instructions: None,
            planned_date_input: None,
        };

        let result = recipe.validate();
        assert!(result.is_err());
        assert!(result
            .unwrap_err()
            .contains(&"Titel ist erforderlich".to_string()));
    }

    #[test]
    fn create_recipe_validates_categories() {
        let recipe = CreateRecipe {
            title: "Test".to_string(),
            categories: vec![],
            ingredients: None,
            instructions: None,
            planned_date_input: None,
        };

        let result = recipe.validate();
        assert!(result.is_err());
        assert!(result
            .unwrap_err()
            .contains(&"Mindestens eine Kategorie muss ausgewählt werden".to_string()));
    }

    #[test]
    fn create_recipe_accepts_valid_data() {
        let recipe = CreateRecipe {
            title: "Spaghetti Carbonara".to_string(),
            categories: vec!["Mittagessen".to_string()],
            ingredients: Some("Nudeln, Eier, Speck".to_string()),
            instructions: Some("Kochen und mischen".to_string()),
            planned_date_input: None,
        };

        let result = recipe.validate();
        assert!(result.is_ok());
    }

    #[test]
    fn create_recipe_accepts_valid_date() {
        // Given: Gültiges Datum im deutschen Format
        let recipe = CreateRecipe {
            title: "Test".to_string(),
            categories: vec!["Mittagessen".to_string()],
            ingredients: None,
            instructions: None,
            planned_date_input: Some("5.3.2025".to_string()),
        };
        // Then: Validierung erfolgreich
        assert!(recipe.validate().is_ok());
    }

    #[test]
    fn create_recipe_rejects_invalid_date() {
        // Given: Ungültiges Datum
        let recipe = CreateRecipe {
            title: "Test".to_string(),
            categories: vec!["Mittagessen".to_string()],
            ingredients: None,
            instructions: None,
            planned_date_input: Some("morgen".to_string()),
        };
        // Then: Fehlermeldung über das Datum
        let result = recipe.validate();
        assert!(result.is_err());
        let errors = result.unwrap_err();
        assert!(errors.iter().any(|e| e.contains("Kein gültiges Datum")));
    }

    #[test]
    fn create_recipe_accepts_empty_date() {
        // Given: Leeres Datum (optional)
        let recipe = CreateRecipe {
            title: "Test".to_string(),
            categories: vec!["Mittagessen".to_string()],
            ingredients: None,
            instructions: None,
            planned_date_input: Some("".to_string()),
        };
        // Then: Validierung erfolgreich (kein Datum = kein Fehler)
        assert!(recipe.validate().is_ok());
    }

    #[test]
    fn create_recipe_rejects_invalid_category() {
        let recipe = CreateRecipe {
            title: "Test".to_string(),
            categories: vec!["Ungültig".to_string()],
            ingredients: None,
            instructions: None,
            planned_date_input: None,
        };

        let result = recipe.validate();
        assert!(result.is_err());
    }

    #[test]
    fn categories_json_returns_valid_json() {
        let recipe = CreateRecipe {
            title: "Test".to_string(),
            categories: vec!["Mittagessen".to_string(), "Party".to_string()],
            ingredients: None,
            instructions: None,
            planned_date_input: None,
        };

        let json = recipe.categories_json();
        assert_eq!(json, r#"["Mittagessen","Party"]"#);
    }
}
