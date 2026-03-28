# Implementierungsplan: Story 9 - Filter "Länger nicht gemacht"

## Technische Schritte

### Schritt 1: Datenbank-Layer — Query für "Länger nicht gemacht"

- [ ] Neue Funktion `filter_recipes_not_made_recently` in `src/models/recipe_db.rs` erstellen:
  - Parameter: `pool: &SqlitePool`, `categories: &[String]`, `search_query: &str`
  - Gibt `Vec<Recipe>` zurück
  - WHERE-Bedingung: `planned_date IS NULL OR planned_date <= DATE('now')` — schließt Zukunftsdaten aus
  - Sortierung: `CASE WHEN planned_date IS NULL THEN 0 ELSE 1 END ASC, planned_date ASC` — NULL-Dates zuerst, dann ältestes Datum
  - Innerhalb gleichen Datums zusätzlich alphabetisch sortieren (in Rust via `sort_by` nach `normalize_for_sort`)
  - Kategorie-Filter und Suchbegriff analog zu `filter_recipes_by_categories` kombinieren (UND-Logik)

- [ ] Unit-Tests in `src/models/recipe_db.rs` (im `#[cfg(test)] mod tests`-Block):
  - `not_made_recently_null_dates_appear_first` — Rezepte ohne Datum kommen vor solchen mit Datum
  - `not_made_recently_sorted_by_date_ascending` — ältestes Datum zuerst
  - `not_made_recently_excludes_future_dates` — Rezepte mit Zukunftsdatum werden ausgeschlossen
  - `not_made_recently_includes_past_and_null` — Vergangenheit und NULL werden eingeschlossen
  - `not_made_recently_returns_empty_if_all_future` — leere Liste wenn alle Daten in der Zukunft
  - `not_made_recently_same_date_sorted_alphabetically` — gleiche Daten alphabetisch geordnet
  - `not_made_recently_combined_with_category_filter` — Kategorie-Filter und Datumssortierung kombiniert
  - `not_made_recently_combined_with_search_query` — Suchbegriff und Datumssortierung kombiniert

---

### Schritt 2: Modell-Layer — Export und IndexQuery erweitern

- [ ] Neue Funktion `filter_recipes_not_made_recently` in `src/models/mod.rs` re-exportieren:
  ```rust
  pub use recipe_db::{
      create_recipe, delete_recipe, filter_recipes_by_categories,
      filter_recipes_not_made_recently, get_recipe_by_id, update_recipe,
  };
  ```

- [ ] `IndexQuery`-Struct in `src/routes/recipes.rs` um `filter`-Parameter erweitern:
  ```rust
  pub struct IndexQuery {
      pub deleted: Option<String>,
      pub q: Option<String>,
      pub filter: Option<String>,  // "laenger-nicht-gemacht"
  }
  ```

- [ ] `IndexTemplate` in `src/templates.rs` um `not_made_filter_active: bool` erweitern:
  ```rust
  pub struct IndexTemplate {
      pub recipes: Vec<RecipeListItem>,
      pub deleted_title: Option<String>,
      pub search_query: String,
      pub active_categories: Vec<String>,
      pub category_filters: Vec<CategoryFilterItem>,
      pub reset_categories_url: String,
      pub not_made_filter_active: bool,    // neu
      pub not_made_filter_toggle_url: String,  // neu
  }
  ```

---

### Schritt 3: Route/Handler — Filter-Logik einbauen

- [ ] `index`-Handler in `src/routes/recipes.rs` anpassen:
  - `filter`-Parameter aus `IndexQuery` auslesen
  - `not_made_filter_active = query.filter.as_deref() == Some("laenger-nicht-gemacht")`
  - Wenn aktiv: `filter_recipes_not_made_recently(&pool, &active_categories, &search_query)` aufrufen
  - Wenn inaktiv: bestehende `filter_recipes_by_categories`-Logik unverändert
  - `not_made_filter_toggle_url` berechnen: aktiv → URL ohne `filter`-Parameter, inaktiv → URL mit `?filter=laenger-nicht-gemacht` (Kategorie- und Suchparameter beibehalten)

- [ ] Hilfsfunktion `build_not_made_toggle_url` in `src/routes/recipes.rs` erstellen:
  - Parameter: `is_active: bool`, `active_categories: &[String]`, `search_query: &str`
  - Aktiv → URL ohne `filter=laenger-nicht-gemacht` (Kategorie + Suche bleiben erhalten)
  - Inaktiv → URL mit `&filter=laenger-nicht-gemacht` angehängt (Kategorie + Suche bleiben erhalten)

- [ ] `IndexTemplate` im Handler mit `not_made_filter_active` und `not_made_filter_toggle_url` befüllen

- [ ] Kategorie-Toggle-URLs bei aktivem `not_made`-Filter anpassen:
  - `build_category_toggle_url` muss den `filter`-Parameter weitertragen, wenn `not_made_filter_active` aktiv ist
  - Bestehende `build_category_filters`-Funktion um `not_made_filter_active: bool`-Parameter erweitern

---

### Schritt 4: Template — Filter-UI einbauen

- [ ] Neuen Filter-Button "Länger nicht gemacht" in `templates/index.html` hinzufügen:
  - Platzierung: unter der Kategorie-Filter-Leiste, als eigener Bereich (`<div class="sort-filter">`)
  - Visuell klar als Sortier-Filter abgegrenzt (andere CSS-Klasse als Kategorie-Filter)
  - Aktiver Zustand: CSS-Klasse `active`, `aria-pressed="{{ not_made_filter_active }}"`
  - HTMX-Attribute: `hx-get`, `hx-target="#recipe-results"`, `hx-push-url="true"`, `hx-select="#recipe-results"`
  - Funktioniert auch ohne JavaScript (normaler `<a>`-Link)

- [ ] Keine-Treffer-Meldungen in `templates/index.html` ergänzen:
  - Kombination: aktiver `not_made`-Filter + Kategorie ohne Treffer
  - Nur `not_made`-Filter aktiv ohne Treffer: `"Keine Rezepte ohne zukünftiges Datum gefunden."`
  - Kombination mit Suche: `"Keine Rezepte für '...' ohne zukünftiges Datum gefunden."`

- [ ] Datum auf Rezeptkarte in gefilterter Ansicht anzeigen (ist bereits implementiert via `recipe.planned_date`)

- [ ] Suchformular in `templates/index.html`: `filter`-Parameter als Hidden-Input weitertragen wenn aktiv:
  ```html
  {% if not_made_filter_active %}
  <input type="hidden" name="filter" value="laenger-nicht-gemacht">
  {% endif %}
  ```

---

### Schritt 5: CSS — Styling des neuen Filter-Buttons

- [ ] In `src/static/css/app.css` Styles für den Sortier-Filter-Bereich ergänzen:
  - `.sort-filter` Container (visuell abgesetzt von Kategorie-Filtern, z.B. kleiner Abstand oben)
  - `.sort-filter-btn` für inaktiven Zustand
  - `.sort-filter-btn.active` für aktiven Zustand (hervorgehoben, konsistent mit `.category-filter-btn.active`)
  - Responsive: Button skaliert auf kleinen Bildschirmen

---

### Schritt 6: Integrationstests (Rust)

- [ ] Neue Testdatei `tests/recipe_not_made_filter.rs` erstellen:
  - `not_made_filter_returns_200_with_correct_recipes` — Handler liefert 200 mit Rezepten sortiert nach Datum
  - `not_made_filter_excludes_future_dated_recipes` — Rezepte mit Zukunftsdatum nicht im Body
  - `not_made_filter_shows_null_date_recipes_first` — Rezepte ohne Datum erscheinen vor solchen mit Datum
  - `not_made_filter_shows_empty_state_message` — Meldung bei keinen Treffern
  - `not_made_filter_combined_with_category` — kombinierter Filter `?filter=laenger-nicht-gemacht&kategorie=Brot`
  - `not_made_filter_combined_with_search` — kombinierter Filter `?filter=laenger-nicht-gemacht&q=suche`
  - `deeplink_not_made_filter_returns_correct_state` — URL `/?filter=laenger-nicht-gemacht` gibt korrekten HTML-Zustand zurück
  - `no_filter_param_returns_alphabetical_list` — ohne `filter`-Parameter normale alphabetische Liste
  - Jeden Test mit `// Given / // When / // Then`-Kommentaren

---

### Schritt 7: E2E-Tests (Playwright)

- [ ] Neue Testdatei `tests/e2e/recipe-not-made-filter.spec.ts` erstellen:
  - Hilfsfunktion `createRecipeWithDate` ergänzen (analog zu `createRecipe` in `recipe-category-filter.spec.ts`, mit `planned_date`-Feld)

  - **K1: Filter-Button sichtbar und aktivierbar**
    - Startseite aufrufen → Button "Länger nicht gemacht" ist sichtbar
    - Button klicken → Filter ist aktiv (visuell hervorgehoben, `aria-pressed="true"`)
    - URL enthält `filter=laenger-nicht-gemacht`

  - **K2: Sortierung nach Datum aufsteigend (mit NULL-Dates zuerst)**
    - "Spaghetti Bolognese" mit `planned_date` 2026-01-01
    - "Pfannkuchen" mit `planned_date` 2025-06-15
    - "Pizza" ohne `planned_date`
    - Filter aktivieren → "Pizza" erscheint als erstes, "Pfannkuchen" als zweites, "Spaghetti Bolognese" als drittes

  - **K3: Zukunftsdaten werden ausgeschlossen**
    - "Sonntagsbraten" mit `planned_date` nächste Woche
    - "Linseneintopf" mit `planned_date` letzten Monat
    - Filter aktivieren → "Linseneintopf" sichtbar, "Sonntagsbraten" nicht sichtbar

  - **K4: Filter zurücksetzen**
    - Filter aktivieren, dann erneut auf Button klicken (oder anderweitig deaktivieren)
    - Rezeptliste wieder alphabetisch, `filter`-Parameter nicht mehr in URL, `aria-pressed="false"`

  - **K5: Keine Treffer**
    - Alle Rezepte haben Zukunftsdatum → Filter aktivieren → Hinweistext erscheint, keine leere Liste

  - **K6: DeepLink**
    - URL `/?filter=laenger-nicht-gemacht` direkt aufrufen
    - Filter-Button ist als aktiv markiert, Liste nach Datum sortiert

  - **K7: Kombination mit Kategorie-Filter**
    - "Dinkelbrot" (Brot, `planned_date` 2025-01-01), "Roggenbrot" (Brot, `planned_date` 2026-01-01), "Spaghetti" (Mittagessen, `planned_date` 2024-01-01)
    - Kategorie "Brot" + "Länger nicht gemacht" → nur Brot-Rezepte in Datumsreihenfolge, "Spaghetti" nicht sichtbar

  - Jeder Test mit deutschen Given/When/Then-Kommentaren

---

### Schritt 8: Qualitätschecks (DoD)

- [ ] `cargo fmt` — Code formatieren
- [ ] `cargo clippy -- -D warnings` — keine Warnungen
- [ ] `cargo build` — keine Compilerfehler
- [ ] `cargo test` — alle Unit- und Integrationstests grün
- [ ] `npm run test:e2e` — alle E2E-Tests grün
- [ ] Öffentliche Funktionen in `recipe_db.rs` mit Doc-Kommentar (`///`) versehen
- [ ] Architektur-Dokumentation in `docs/product/architecture.md` prüfen (URL-Tabelle bereits korrekt: `filter=not-made`)

---

## URL-Struktur

```
GET  /                                              → Alle Rezepte (alphabetisch)
GET  /?filter=laenger-nicht-gemacht                → Nur Vergangenheits-/NULL-Datum, nach Datum sortiert
GET  /?filter=laenger-nicht-gemacht&kategorie=Brot → Brot-Rezepte, nach Datum sortiert
GET  /?filter=laenger-nicht-gemacht&q=dinkel       → Suche + Datumssortierung
GET  /?filter=laenger-nicht-gemacht&kategorie=Brot&q=dinkel  → Alle drei Filter kombiniert
```

---

## Abhängigkeiten

- Story 5 (alphabetische Sortierung) ist implementiert — `normalize_for_sort` wird für Sekundärsortierung wiederverwendet
- Story 8 (Kategorie-Filter) ist implementiert — `filter_recipes_by_categories`, `build_category_filters`, `extract_kategorie_params` werden wiederverwendet und erweitert
- Kein neues Datenbankfeld nötig — `planned_date` existiert bereits
- Keine neue Migration nötig — Index `idx_recipes_planned_date` existiert bereits laut `architecture.md`
- `urlencoding`-Crate ist bereits als Abhängigkeit vorhanden

---

## Test-Checkliste

- [ ] Unit-Test: `not_made_recently_null_dates_appear_first`
- [ ] Unit-Test: `not_made_recently_sorted_by_date_ascending`
- [ ] Unit-Test: `not_made_recently_excludes_future_dates`
- [ ] Unit-Test: `not_made_recently_includes_past_and_null`
- [ ] Unit-Test: `not_made_recently_returns_empty_if_all_future`
- [ ] Unit-Test: `not_made_recently_same_date_sorted_alphabetically`
- [ ] Unit-Test: `not_made_recently_combined_with_category_filter`
- [ ] Unit-Test: `not_made_recently_combined_with_search_query`
- [ ] Integrationstest: `not_made_filter_returns_200_with_correct_recipes`
- [ ] Integrationstest: `not_made_filter_excludes_future_dated_recipes`
- [ ] Integrationstest: `not_made_filter_shows_null_date_recipes_first`
- [ ] Integrationstest: `not_made_filter_shows_empty_state_message`
- [ ] Integrationstest: `not_made_filter_combined_with_category`
- [ ] Integrationstest: `not_made_filter_combined_with_search`
- [ ] Integrationstest: `deeplink_not_made_filter_returns_correct_state`
- [ ] Integrationstest: `no_filter_param_returns_alphabetical_list`
- [ ] E2E-Test: K1 – Filter-Button sichtbar und aktivierbar
- [ ] E2E-Test: K2 – Sortierung nach Datum aufsteigend, NULL-Dates zuerst
- [ ] E2E-Test: K3 – Zukunftsdaten ausgeschlossen
- [ ] E2E-Test: K4 – Filter zurücksetzen
- [ ] E2E-Test: K5 – Keine Treffer, Hinweistext erscheint
- [ ] E2E-Test: K6 – DeepLink `/?filter=laenger-nicht-gemacht`
- [ ] E2E-Test: K7 – Kombination mit Kategorie-Filter
- [ ] Manueller Test: Tastatur-Navigation zum Filter-Button (Tab + Enter/Space)
- [ ] Manueller Test: `aria-pressed` korrekt gesetzt (aktiv/inaktiv)
- [ ] Manueller Test: Filter-Zustand bleibt beim Navigieren erhalten (DeepLink)

---

## Offene Punkte

- Architektur-Dokument (`architecture.md`) verwendet `filter=not-made` als URL-Parameter, die Story spricht von `filter=laenger-nicht-gemacht`. Empfehlung: `laenger-nicht-gemacht` verwenden, da der Story-Text maßgeblich ist und die URL für Endnutzer lesbar sein soll. Vor Implementierung kurz abstimmen.
- `build_category_filters` muss den `filter`-Parameter weitertragen: Wenn "Länger nicht gemacht" aktiv ist und der Nutzer eine Kategorie toggled, soll der `not_made`-Filter erhalten bleiben. Dafür braucht `build_category_toggle_url` einen zusätzlichen `not_made_filter_active: bool`-Parameter.
