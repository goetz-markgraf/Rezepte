use crate::models::SimilarRecipe;
use askama::Template;

/// Template für den Duplikat-Hinweis (HTMX-Fragment unterhalb des Titelfelds).
#[derive(Template)]
#[template(path = "recipes/_duplicate_hint.html")]
pub struct DuplicateHintTemplate {
    pub candidates: Vec<SimilarRecipe>,
}

/// Ein einzelner Rezepteintrag auf der "Heute gekocht"-Seite.
pub struct HeuteRezeptItem {
    pub id: i64,
    pub title: String,
    pub rating: Option<i32>,
}

impl HeuteRezeptItem {
    /// Gibt true zurück, wenn die aktuelle Bewertung dem Wert `n` entspricht.
    pub fn rating_is_active(&self, n: i32) -> bool {
        self.rating == Some(n)
    }

    /// Gibt true zurück, wenn der Stern `n` ausgefüllt sein soll (rating >= n).
    pub fn star_filled(&self, n: i32) -> bool {
        self.rating.unwrap_or(0) >= n
    }
}

/// Ein Tagesabschnitt auf der "Heute gekocht"-Seite (Gestern, Heute oder Morgen).
pub struct HeuteTagesabschnitt {
    /// Label: "Gestern", "Heute", "Morgen"
    pub label: String,
    /// Wochentag-Name: "Montag" bis "Sonntag"
    pub wochentag_name: String,
    /// Datum: "30. März"
    pub datum_kurz: String,
    /// true wenn dieser Abschnitt = heute (für visuelle Hervorhebung)
    pub ist_heute: bool,
    /// Rezepte dieses Abschnitts (kann leer sein)
    pub rezepte: Vec<HeuteRezeptItem>,
}

/// Template für die "Heute gekocht"-Seite.
#[derive(Template)]
#[template(path = "heute.html")]
pub struct HeuteTemplate {
    /// Alle 3 Tagesabschnitte: gestern, heute, morgen
    pub abschnitte: Vec<HeuteTagesabschnitt>,
    /// Datum-Zeile im Seitenkopf: "Donnerstag, 2. April 2026"
    pub heute_anzeige: String,
}

/// Template für das Inline-Rating-Fragment auf der "Heute gekocht"-Seite.
/// Nutzt `id="inline-rating-{{ id }}"` statt `id="inline-rating"` für eindeutige IDs
/// bei mehreren gleichzeitig sichtbaren Rezepten.
#[derive(Template)]
#[template(path = "recipes/_inline_rating_heute.html")]
pub struct InlineRatingHeuteTemplate {
    pub id: i64,
    pub rating: Option<i32>,
}

impl InlineRatingHeuteTemplate {
    /// Gibt true zurück, wenn die aktuelle Bewertung dem Wert `n` entspricht.
    pub fn rating_is_active(&self, n: i32) -> bool {
        self.rating == Some(n)
    }

    /// Gibt true zurück, wenn der Stern `n` ausgefüllt sein soll (rating >= n).
    pub fn star_filled(&self, n: i32) -> bool {
        self.rating.unwrap_or(0) >= n
    }
}

/// Ein einzelner Rezepteintrag in der Wochenvorschau.
pub struct WochentagesEintragItem {
    pub id: i64,
    pub title: String,
}

/// Ein Wochentag in der Wochenvorschau mit seinen Rezepten.
pub struct Wochentag {
    /// Wochentag-Name: "Montag" bis "Sonntag"
    pub wochentag_name: String,
    /// Kurzform des Datums: "30. März"
    pub datum_kurz: String,
    /// true wenn dieser Tag = heute (serverseitig berechnet)
    pub ist_heute: bool,
    /// true wenn dieser Tag vor heute liegt
    pub ist_vergangen: bool,
    /// Liste der an diesem Tag geplanten Rezepte.
    pub rezepte: Vec<WochentagesEintragItem>,
}

/// Template für die Wochenvorschau-Seite.
#[derive(Template)]
#[template(path = "wochenvorschau.html")]
pub struct WochenvorschauTemplate {
    /// Alle 7 Wochentage (Montag bis Sonntag) mit ihren Rezepten.
    pub tage: Vec<Wochentag>,
    /// KW-Anzeige: "KW 14 · 30. März – 5. April 2026"
    pub kw_anzeige: String,
    /// true wenn mindestens ein Rezept in der Woche geplant ist.
    pub hat_rezepte: bool,
}

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

/// Ein gespeicherter Filter für die Template-Darstellung.
pub struct SavedFilterItem {
    pub id: i64,
    pub name: String,
    /// Ziel-URL für den Klick: "/?<query_string>"
    pub url: String,
    /// ARIA-Label für den Löschen-Button: "Filter '<name>' löschen"
    pub delete_aria_label: String,
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
    /// Aktiver Bewertungsfilter: `Some("gut")` | `Some("favoriten")` | `None`.
    pub bewertung_filter: Option<String>,
    /// Toggle-URL für den "Nur Gute" (3+ Sterne) Bewertungsfilter.
    pub bewertung_gut_toggle_url: String,
    /// Toggle-URL für den "Favoriten" (5 Sterne) Bewertungsfilter.
    pub bewertung_favoriten_toggle_url: String,
    /// Ob irgendein Filter aktiv ist (Kategorie, Suche, Datumsfilter oder Bewertung).
    /// Steuert die Sichtbarkeit des "Alle Filter zurücksetzen"-Buttons.
    pub any_filter_active: bool,
    /// Alle gespeicherten Filter (aus DB, für die Anzeige).
    pub saved_filters: Vec<SavedFilterItem>,
    /// Query-String des aktuellen Filterzustands (für das Speichern-Formular).
    /// Leer wenn kein Filter aktiv. Format: "kategorie=Brot&bewertung=gut"
    pub current_query_string: String,
    /// Fehler beim Speichern: "duplikat" | None
    pub save_error: Option<String>,
    /// Der Name, der beim Speichern verwendet wurde (für Fehlermeldung im Template).
    pub save_name: Option<String>,
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

/// Template für das Inline-Rating-Fragment in der Detailansicht.
#[derive(Template)]
#[template(path = "recipes/_inline_rating.html")]
pub struct InlineRatingTemplate {
    pub id: i64,
    pub rating: Option<i32>,
}

impl InlineRatingTemplate {
    /// Gibt true zurück, wenn die aktuelle Bewertung dem Wert `n` entspricht.
    pub fn rating_is_active(&self, n: i32) -> bool {
        self.rating == Some(n)
    }

    /// Gibt true zurück, wenn der Stern `n` ausgefüllt sein soll (rating >= n).
    pub fn star_filled(&self, n: i32) -> bool {
        self.rating.unwrap_or(0) >= n
    }
}

impl RecipeDetailTemplate {
    /// Gibt true zurück, wenn die aktuelle Bewertung dem Wert `n` entspricht (für Inline-Rating-Partial).
    pub fn rating_is_active(&self, n: i32) -> bool {
        self.rating == Some(n)
    }

    /// Gibt true zurück, wenn der Stern `n` ausgefüllt sein soll (rating >= n).
    pub fn star_filled(&self, n: i32) -> bool {
        self.rating.unwrap_or(0) >= n
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
