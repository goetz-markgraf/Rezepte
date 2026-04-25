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

/// Template für die Wochenvorschau-Seite (jetzt 15-Tage-Liste).
#[derive(Template)]
#[template(path = "wochenvorschau.html")]
pub struct WochenvorschauTemplate {
    /// Alle 15 Tage (ab heute) mit ihren Rezepten.
    pub tage: Vec<Wochentag>,
    /// Zeitraum-Anzeige: "04.04.2026 – 18.04.2026"
    pub zeitraum_anzeige: String,
    /// true wenn mindestens ein Rezept in den 15 Tagen geplant ist.
    #[allow(dead_code)]
    pub hat_rezepte: bool,
}

#[derive(Template)]
#[template(path = "error/not_found.html")]
pub struct NotFoundTemplate {
    pub message: String,
}

/// Leichtgewichtige Rezept-Information für den Wochenpicker.
/// Enthält nur ID und Titel für den Indikator und Tooltip.
#[derive(Debug, Clone)]
pub struct WeekdayPickerRecipeInfo {
    pub id: i64,
    pub title: String,
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
    /// Geplante Rezepte für die nächsten 10 Tage (morgen bis +10 Tage).
    /// Index 0-9 entspricht den Tagen im Picker.
    /// None = kein Rezept geplant an diesem Tag.
    pub planned_recipes: Vec<Option<WeekdayPickerRecipeInfo>>,
}

#[derive(Template)]
#[template(path = "recipes/detail.html")]
pub struct RecipeDetailTemplate {
    pub id: i64,
    pub title: String,
    pub categories: Vec<String>,
    /// Gerendetes Markdown-HTML (sanitiert). Wird im Template per `|safe` ausgegeben.
    pub ingredients: Option<String>,
    /// Gerendetes Markdown-HTML (sanitiert). Wird im Template per `|safe` ausgegeben.
    pub instructions: Option<String>,
    pub created_at: String,
    pub updated_at: String,
    pub success: bool,
    /// Datum im langen deutschen Format (z.B. "5. März 2025") oder None.
    pub planned_date: Option<String>,
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
    /// Ob irgendein Filter aktiv ist (Kategorie, Suche, Datumsfilter).
    /// Steuert die Sichtbarkeit des "Alle Filter zurücksetzen"-Buttons.
    pub any_filter_active: bool,
    /// Alle gespeicherten Filter (aus DB, für die Anzeige).
    pub saved_filters: Vec<SavedFilterItem>,
    /// Query-String des aktuellen Filterzustands (für das Speichern-Formular).
    /// Leer wenn kein Filter aktiv. Format: "kategorie=Brot"
    pub current_query_string: String,
    /// Fehler beim Speichern: "duplikat" | None
    pub save_error: Option<String>,
    /// Der Name, der beim Speichern verwendet wurde (für Fehlermeldung im Template).
    pub save_name: Option<String>,
    /// Ob der Filterbereich eingeklappt ist (via `?filter_collapsed=1`).
    pub filter_collapsed: bool,
    /// URL für den Toggle-Button (ein-/ausklappen).
    pub filter_collapsed_toggle_url: String,
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
            planned_recipes: vec![None; 10], // 10 Tage initialisieren
        }
    }
}

impl RecipeFormTemplate {
    pub fn new() -> Self {
        Self::default()
    }

}

/// Ein einzelnes Dubletten-Paar für das Template.
pub struct DublettenPaarItem {
    pub id_a: i64,
    pub titel_a: String,
    pub id_b: i64,
    pub titel_b: String,
}

/// Template für die Dubletten-Übersichtsseite.
#[derive(Template)]
#[template(path = "recipes/duplicates.html")]
pub struct DublettenUebersichtTemplate {
    pub paare: Vec<DublettenPaarItem>,
}

/// Informationen zu einem Rezept auf der Merge-Seite.
pub struct MergeRezeptInfo {
    pub title: String,
    pub categories: Vec<String>,
    pub ingredients: Option<String>,
    pub instructions: Option<String>,
    pub planned_date: Option<String>,
    pub created_at: String,
    pub updated_at: String,
}

/// Template für die Merge-Seite (Rezepte zusammenführen).
#[derive(Template)]
#[template(path = "recipes/merge.html")]
pub struct MergeTemplate {
    pub rezept_a: MergeRezeptInfo,
    pub rezept_b: MergeRezeptInfo,
    pub source_id: i64,
    pub target_id: i64,
    pub fehler: Vec<String>,
}

impl MergeTemplate {
    pub fn hat_konflikt_titel(&self) -> bool {
        self.rezept_a.title != self.rezept_b.title
    }

    pub fn hat_konflikt_categories(&self) -> bool {
        self.rezept_a.categories != self.rezept_b.categories
    }

    pub fn hat_konflikt_ingredients(&self) -> bool {
        self.rezept_a.ingredients.is_some() && self.rezept_b.ingredients.is_some()
    }

    pub fn hat_konflikt_instructions(&self) -> bool {
        self.rezept_a.instructions.is_some() && self.rezept_b.instructions.is_some()
    }

    pub fn hat_konflikt_planned_date(&self) -> bool {
        self.rezept_a.planned_date.is_some() && self.rezept_b.planned_date.is_some()
    }
}
