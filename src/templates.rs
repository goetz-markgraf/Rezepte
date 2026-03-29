use askama::Template;

#[derive(Template)]
#[template(path = "error/not_found.html")]
pub struct NotFoundTemplate {
    pub message: String,
}

#[derive(Template)]
#[template(path = "recipes/form.html")]
pub struct RecipeFormTemplate {
    pub categories: Vec<String>,
    pub errors: Vec<String>,
    pub title: String,
    pub selected_categories: Vec<String>,
    pub ingredients: String,
    pub instructions: String,
    pub recipe_id: Option<i64>,
    /// Datum im deutschen Format (T.M.JJJJ) oder leer.
    pub planned_date: String,
    /// Bewertung 1-5 Sterne. None bedeutet keine Bewertung.
    pub rating: Option<i32>,
}

#[derive(Template)]
#[template(path = "recipes/detail.html")]
pub struct RecipeDetailTemplate {
    pub id: i64,
    pub title: String,
    pub categories: Vec<String>,
    pub ingredients: Option<String>,
    pub instructions: Option<String>,
    pub created_at: String,
    pub updated_at: String,
    pub success: bool,
    /// Datum im langen deutschen Format (z.B. "5. März 2025") oder None.
    pub planned_date: Option<String>,
    /// Bewertung 1-5 Sterne. None bedeutet keine Bewertung.
    pub rating: Option<i32>,
}

/// Template für die Bestätigungsseite zum Löschen eines Rezepts.
#[derive(Template)]
#[template(path = "recipes/confirm_delete.html")]
pub struct ConfirmDeleteTemplate {
    pub id: i64,
    pub title: String,
}

/// Ein Kategorie-Filter-Button mit vorberechneter Toggle-URL.
pub struct CategoryFilterItem {
    pub name: String,
    pub is_active: bool,
    /// URL, die beim Klick aufgerufen wird (Toggle: aktiv→entfernen, inaktiv→hinzufügen).
    pub toggle_url: String,
}

/// Template für die Rezept-Übersichtsseite.
#[derive(Template)]
#[template(path = "index.html")]
pub struct IndexTemplate {
    pub recipes: Vec<RecipeListItem>,
    pub deleted_title: Option<String>,
    pub search_query: String,
    /// Aktuell aktive Kategorien (aus URL-Parametern).
    pub active_categories: Vec<String>,
    /// Alle Kategorien mit Toggle-URLs für die Filter-Buttons.
    pub category_filters: Vec<CategoryFilterItem>,
    /// URL zum Zurücksetzen aller Kategorie-Filter.
    pub reset_categories_url: String,
    /// Ob der Filter "Länger nicht gemacht" aktiv ist.
    pub not_made_filter_active: bool,
    /// URL zum Umschalten des "Länger nicht gemacht"-Filters.
    pub not_made_filter_toggle_url: String,
    /// Ob der Filter "Nächste 7 Tage" aktiv ist.
    pub next_seven_days_filter_active: bool,
    /// URL zum Umschalten des "Nächste 7 Tage"-Filters.
    pub next_seven_days_filter_toggle_url: String,
}

#[derive(Debug)]
pub struct RecipeListItem {
    pub id: i64,
    pub title: String,
    pub categories: Vec<String>,
    /// Datum im kompakten Format (TT.MM.JJJJ) oder None.
    pub planned_date: Option<String>,
    /// Datum mit Wochentag (z.B. "Mo, 31.03.2026") oder None.
    /// Wird nur beim aktiven "Nächste 7 Tage"-Filter befüllt.
    pub planned_date_weekday: Option<String>,
    /// Bewertung 1-5 Sterne. None bedeutet keine Bewertung.
    pub rating: Option<i32>,
}

impl Default for RecipeFormTemplate {
    fn default() -> Self {
        Self {
            categories: crate::models::VALID_CATEGORIES
                .iter()
                .map(|&s| s.to_string())
                .collect(),
            errors: Vec::new(),
            title: String::new(),
            selected_categories: Vec::new(),
            ingredients: String::new(),
            instructions: String::new(),
            recipe_id: None,
            planned_date: String::new(),
            rating: None,
        }
    }
}

impl RecipeFormTemplate {
    pub fn new() -> Self {
        Self::default()
    }

    /// Gibt true zurück, wenn die aktuelle Bewertung dem Wert `n` entspricht.
    pub fn rating_is(&self, n: i32) -> bool {
        self.rating == Some(n)
    }
}

impl RecipeDetailTemplate {
    /// Gibt die Sterndarstellung für die Detailansicht zurück (z.B. "★★★★☆" für 4).
    /// Gibt einen leeren String zurück, wenn keine Bewertung vorhanden ist.
    pub fn stars_display(&self) -> String {
        match self.rating {
            Some(r) => (1..=5).map(|n| if n <= r { '★' } else { '☆' }).collect(),
            None => String::new(),
        }
    }
}

impl RecipeListItem {
    /// Gibt die Sterndarstellung für die Listenansicht zurück (nur ausgefüllte Sterne).
    /// Gibt einen leeren String zurück, wenn keine Bewertung vorhanden ist.
    pub fn stars_display(&self) -> String {
        match self.rating {
            Some(r) => "★".repeat(r as usize),
            None => String::new(),
        }
    }
}
