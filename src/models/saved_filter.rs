use sqlx::FromRow;

/// Ein gespeicherter Filter in der Datenbank.
#[derive(Debug, FromRow)]
pub struct SavedFilter {
    pub id: i64,
    pub name: String,
    pub query_string: String,
    #[allow(dead_code)]
    pub created_at: String,
}

/// Eingabedaten zum Erstellen eines gespeicherten Filters.
#[derive(Debug)]
pub struct CreateSavedFilter {
    pub name: String,
    pub query_string: String,
}

/// Validierungsfehler für `CreateSavedFilter`.
#[derive(Debug, PartialEq)]
pub enum SavedFilterValidationError {
    NameEmpty,
    NameTooLong,
    QueryStringEmpty,
}

impl CreateSavedFilter {
    /// Validiert die Eingabedaten. Gibt eine Liste von Validierungsfehlern zurück.
    pub fn validate(&self) -> Vec<SavedFilterValidationError> {
        let mut errors = Vec::new();

        if self.name.trim().is_empty() {
            errors.push(SavedFilterValidationError::NameEmpty);
        } else if self.name.len() > 100 {
            errors.push(SavedFilterValidationError::NameTooLong);
        }

        if self.query_string.trim().is_empty() {
            errors.push(SavedFilterValidationError::QueryStringEmpty);
        }

        errors
    }

    /// Gibt `true` zurück, wenn die Eingabedaten gültig sind.
    pub fn is_valid(&self) -> bool {
        self.validate().is_empty()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn leerer_name_wird_abgelehnt() {
        // Given: Ein Filter mit leerem Namen
        let filter = CreateSavedFilter {
            name: "".to_string(),
            query_string: "kategorie=Brot".to_string(),
        };

        // When: Validierung
        let errors = filter.validate();

        // Then: Fehler "NameEmpty" wird zurückgegeben
        assert!(errors.contains(&SavedFilterValidationError::NameEmpty));
    }

    #[test]
    fn name_nur_leerzeichen_wird_abgelehnt() {
        // Given: Ein Filter mit Name aus nur Leerzeichen
        let filter = CreateSavedFilter {
            name: "   ".to_string(),
            query_string: "kategorie=Brot".to_string(),
        };

        // When: Validierung
        let errors = filter.validate();

        // Then: Fehler "NameEmpty" wird zurückgegeben
        assert!(errors.contains(&SavedFilterValidationError::NameEmpty));
    }

    #[test]
    fn zu_langer_name_wird_abgelehnt() {
        // Given: Ein Filter mit 101-Zeichen-Name
        let filter = CreateSavedFilter {
            name: "a".repeat(101),
            query_string: "kategorie=Brot".to_string(),
        };

        // When: Validierung
        let errors = filter.validate();

        // Then: Fehler "NameTooLong" wird zurückgegeben
        assert!(errors.contains(&SavedFilterValidationError::NameTooLong));
    }

    #[test]
    fn leerer_query_string_wird_abgelehnt() {
        // Given: Ein Filter mit leerem query_string
        let filter = CreateSavedFilter {
            name: "Brot-Ideen".to_string(),
            query_string: "".to_string(),
        };

        // When: Validierung
        let errors = filter.validate();

        // Then: Fehler "QueryStringEmpty" wird zurückgegeben
        assert!(errors.contains(&SavedFilterValidationError::QueryStringEmpty));
    }

    #[test]
    fn gueltige_eingaben_werden_akzeptiert() {
        // Given: Ein Filter mit gültigem Namen und query_string
        let filter = CreateSavedFilter {
            name: "Brot-Ideen".to_string(),
            query_string: "kategorie=Brot".to_string(),
        };

        // When: Validierung
        let errors = filter.validate();

        // Then: Keine Fehler
        assert!(errors.is_empty());
        assert!(filter.is_valid());
    }

    #[test]
    fn name_mit_exakt_100_zeichen_wird_akzeptiert() {
        // Given: Ein Filter mit genau 100-Zeichen-Name (Grenzwert)
        let filter = CreateSavedFilter {
            name: "a".repeat(100),
            query_string: "kategorie=Brot".to_string(),
        };

        // When: Validierung
        let errors = filter.validate();

        // Then: Keine Fehler
        assert!(errors.is_empty());
    }
}
