use serde::{Deserialize, Serialize};
use sqlx::FromRow;

pub const VALID_CATEGORIES: &[&str] = &["Mittagessen", "Brot", "Party", "Kuchen", "Snacks"];

#[derive(Debug, FromRow, Serialize, Deserialize)]
pub struct Recipe {
    pub id: i64,
    pub title: String,
    pub categories: String,
    pub ingredients: Option<String>,
    pub instructions: Option<String>,
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
}

impl CreateRecipe {
    pub fn validate(&self) -> Result<(), Vec<String>> {
        let mut errors = Vec::new();

        if self.title.trim().is_empty() {
            errors.push("Titel ist erforderlich".to_string());
        } else if self.title.len() > 100 {
            errors.push("Titel darf maximal 100 Zeichen haben".to_string());
        }

        if self.categories.is_empty() {
            errors.push("Mindestens eine Kategorie muss ausgewählt werden".to_string());
        } else {
            for cat in &self.categories {
                if !VALID_CATEGORIES.contains(&cat.as_str()) {
                    errors.push(format!("'{}' ist keine gültige Kategorie", cat));
                }
            }
        }

        if let Some(ref ingredients) = self.ingredients {
            if ingredients.len() > 2000 {
                errors.push("Zutaten dürfen maximal 2000 Zeichen haben".to_string());
            }
        }

        if let Some(ref instructions) = self.instructions {
            if instructions.len() > 5000 {
                errors.push("Anleitung darf maximal 5000 Zeichen haben".to_string());
            }
        }

        if errors.is_empty() {
            Ok(())
        } else {
            Err(errors)
        }
    }

    pub fn categories_json(&self) -> String {
        serde_json::to_string(&self.categories).unwrap_or_else(|_| "[]".to_string())
    }
}

#[derive(Debug, Deserialize)]
pub struct UpdateRecipe {
    pub title: String,
    pub categories: Vec<String>,
    pub ingredients: Option<String>,
    pub instructions: Option<String>,
}

impl UpdateRecipe {
    pub fn validate(&self) -> Result<(), Vec<String>> {
        let mut errors = Vec::new();

        if self.title.trim().is_empty() {
            errors.push("Titel ist erforderlich".to_string());
        } else if self.title.len() > 100 {
            errors.push("Titel darf maximal 100 Zeichen haben".to_string());
        }

        if self.categories.is_empty() {
            errors.push("Mindestens eine Kategorie muss ausgewählt werden".to_string());
        } else {
            for cat in &self.categories {
                if !VALID_CATEGORIES.contains(&cat.as_str()) {
                    errors.push(format!("'{}' ist keine gültige Kategorie", cat));
                }
            }
        }

        if let Some(ref ingredients) = self.ingredients {
            if ingredients.len() > 2000 {
                errors.push("Zutaten dürfen maximal 2000 Zeichen haben".to_string());
            }
        }

        if let Some(ref instructions) = self.instructions {
            if instructions.len() > 5000 {
                errors.push("Anleitung darf maximal 5000 Zeichen haben".to_string());
            }
        }

        if errors.is_empty() {
            Ok(())
        } else {
            Err(errors)
        }
    }

    pub fn categories_json(&self) -> String {
        serde_json::to_string(&self.categories).unwrap_or_else(|_| "[]".to_string())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn create_recipe_validates_title() {
        let recipe = CreateRecipe {
            title: "".to_string(),
            categories: vec!["Mittagessen".to_string()],
            ingredients: None,
            instructions: None,
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
        };

        let result = recipe.validate();
        assert!(result.is_ok());
    }

    #[test]
    fn create_recipe_rejects_invalid_category() {
        let recipe = CreateRecipe {
            title: "Test".to_string(),
            categories: vec!["Ungültig".to_string()],
            ingredients: None,
            instructions: None,
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
        };

        let json = recipe.categories_json();
        assert_eq!(json, r#"["Mittagessen","Party"]"#);
    }
}
