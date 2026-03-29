# Implementierungsplan: Story 11 - Filter nach Bewertung (Beliebtheit)

## Technische Analyse

### Ist-Zustand

Das `rating`-Feld (INTEGER NULL, Werte 1-5) existiert bereits vollständig in allen Schichten:
- **DB:** Spalte `rating` in `recipes`-Tabelle (Story 14 implementiert)
- **Model:** `Recipe.rating: Option<i32>`, `CreateRecipe.rating`, `UpdateRecipe.rating`
- **Template-Structs:** `IndexTemplate`, `RecipeListItem`, `RecipeDetailTemplate`, `RecipeFormTemplate` — alle haben `rating`
- **Handler:** `index`-Handler übergibt `rating` an `RecipeListItem`
- **Template:** `index.html` zeigt Sterne in Rezept-Karten via `recipe.stars_display()`

### URL-Parameter-Entscheidung

Laut Story: `?bewertung=gut` für 3+ Sterne, `?bewertung=favoriten` für 5 Sterne.
Der vorhandene `?filter=`-Parameter wird bereits für `laenger-nicht-gemacht` und `naechste-7-tage` genutzt.
Der Bewertungsfilter bekommt einen **eigenen Query-Parameter** `bewertung`, damit er mit den bestehenden `filter`-Werten kombiniert werden kann (AND-Logik).

### Filter-Kombinationsmatrix

| Aktiver Filter | Datenbankfunktion |
|---|---|
| (kein) | `filter_recipes_by_categories` |
| `filter=laenger-nicht-gemacht` | `filter_recipes_not_made_recently` |
| `filter=naechste-7-tage` | `filter_recipes_next_seven_days` |
| `bewertung=gut` | `filter_recipes_by_rating` (neu) |
| `bewertung=favoriten` | `filter_recipes_by_rating` (neu) |
| Kombinationen | jede der obigen + Bewertungsklausel |

Da der Bewertungsfilter mit **jedem** der bestehenden Filter kombinierbar ist, wird er als zusätzliche SQL-Bedingung in alle drei existierenden Datenbankfunktionen eingebaut — analog zur `category_clause` und `search_clause`.

---

## Technische Schritte

### Schritt 1: Datenbank-Layer — Bewertungsfilter in alle Queries einbauen

- [ ] Neuen Enum-ähnlichen Typ `RatingFilter` in `src/models/recipe_db.rs` definieren (oder als String-Konstante — einfacher Ansatz):
  ```rust
  pub enum RatingFilter {
      Good,      // rating >= 3
      Favorites, // rating = 5
  }
  ```
  Alternativ (einfacher, konsistenter mit bestehenden String-Vergleichen):
  ```rust
  pub fn rating_filter_clause(bewertung: Option<&str>) -> &'static str {
      match bewertung {
          Some("gut") => "AND rating >= 3",
          Some("favoriten") => "AND rating = 5",
          _ => "",
      }
  }
  ```
  **Entscheidung:** Hilfsfunktion `rating_sql_clause(bewertung: Option<&str>) -> &'static str` — analog zu `category_clause` und `search_clause` im bestehenden Code. Kein extra Enum nötig, da der Wert keine gebundenen Parameter erzeugt (nur direkte Wert-Vergleiche gegen konstante Integers — kein SQL-Injection-Risiko).

- [ ] Funktion `filter_recipes_by_categories` in `src/models/recipe_db.rs` erweitern:
  - Neuer Parameter: `bewertung: Option<&str>`
  - Zusätzliche SQL-Klausel via `rating_sql_clause(bewertung)` einbauen
  - Die Klausel wird hinter die `category_clause` / `search_clause` angehängt

- [ ] Funktion `filter_recipes_not_made_recently` in `src/models/recipe_db.rs` erweitern:
  - Neuer Parameter: `bewertung: Option<&str>`
  - Zusätzliche SQL-Klausel via `rating_sql_clause(bewertung)` einbauen

- [ ] Funktion `filter_recipes_next_seven_days` in `src/models/recipe_db.rs` erweitern:
  - Neuer Parameter: `bewertung: Option<&str>`
  - Zusätzliche SQL-Klausel via `rating_sql_clause(bewertung)` einbauen

- [ ] Unit-Tests in `src/models/recipe_db.rs` (im `#[cfg(test)] mod tests`-Block):
  - `rating_filter_gut_returns_only_three_plus_stars` — Rezepte mit 3/4/5 Sternen sichtbar, 1/2/NULL nicht
  - `rating_filter_favoriten_returns_only_five_stars` — nur 5-Sterne-Rezepte, alle anderen nicht
  - `rating_filter_none_returns_all_recipes` — ohne Bewertungsfilter alle Rezepte
  - `rating_filter_excludes_unrated_recipes` — unbewertete Rezepte (NULL) werden ausgeblendet
  - `rating_filter_combined_with_category` — Bewertungsfilter + Kategorie (AND-Logik)
  - `rating_filter_combined_with_search` — Bewertungsfilter + Suchbegriff (AND-Logik)
  - `rating_filter_gut_returns_empty_if_no_qualifying_recipes` — leere Liste wenn alle < 3 Sterne
  - `rating_filter_favoriten_returns_empty_if_no_five_star` — leere Liste wenn kein 5-Sterne-Rezept

---

### Schritt 2: Modell-Layer — Exports und IndexQuery erweitern

- [ ] Alle drei erweiterten Datenbankfunktionen in `src/models/mod.rs` korrekt re-exportieren (Signaturen haben neuen Parameter — der Compiler weist darauf hin)

- [ ] `IndexQuery`-Struct in `src/routes/recipes.rs` um `bewertung`-Parameter erweitern:
  ```rust
  #[derive(Deserialize)]
  pub struct IndexQuery {
      pub deleted: Option<String>,
      pub q: Option<String>,
      pub filter: Option<String>,
      pub bewertung: Option<String>,  // neu: "gut" | "favoriten"
  }
  ```

- [ ] `IndexTemplate`-Struct in `src/templates.rs` um Bewertungsfilter-Felder erweitern:
  ```rust
  pub struct IndexTemplate {
      // ... bestehende Felder ...
      pub bewertung_filter: Option<String>,             // neu: aktiver Wert ("gut" | "favoriten" | None)
      pub bewertung_gut_toggle_url: String,             // neu: Toggle-URL für "Nur Gute"
      pub bewertung_favoriten_toggle_url: String,       // neu: Toggle-URL für "Favoriten"
  }
  ```

---

### Schritt 3: Route/Handler — Bewertungsfilter-Logik einbauen

- [ ] Hilfsfunktion `build_bewertung_toggle_url` in `src/routes/recipes.rs` erstellen:
  - Parameter: `value: &str` (z.B. `"gut"`), `current: Option<&str>`, `active_categories: &[String]`, `search_query: &str`, `not_made_filter_active: bool`, `next_seven_days_filter_active: bool`
  - Wenn `current == Some(value)` (Toggle: deaktivieren) → URL ohne `bewertung`-Parameter
  - Wenn `current != Some(value)` (aktivieren) → URL mit `&bewertung={value}`
  - Alle anderen bestehenden Parameter (q, kategorie, filter) bleiben erhalten

- [ ] Bestehende Hilfsfunktionen `build_category_toggle_url`, `build_not_made_toggle_url`, `build_next_seven_days_toggle_url`, `build_reset_url` um `bewertung: Option<&str>` erweitern:
  - Aktiver `bewertung`-Parameter muss beim Kategorie-Toggle erhalten bleiben
  - Aktiver `bewertung`-Parameter bleibt beim Not-Made-Toggle / Seven-Days-Toggle erhalten
  - Bei `build_reset_url` (Kategorie-Reset) bleibt `bewertung` erhalten

- [ ] `index`-Handler in `src/routes/recipes.rs` anpassen:
  - `bewertung`-Parameter aus `IndexQuery` auslesen
  - Bewertungswert validieren (nur `"gut"` und `"favoriten"` akzeptieren, Rest ignorieren)
  - Alle drei Datenbankaufrufe mit `bewertung.as_deref()` ergänzen
  - `bewertung_gut_toggle_url` und `bewertung_favoriten_toggle_url` über `build_bewertung_toggle_url` berechnen
  - `IndexTemplate` mit neuen Feldern befüllen

- [ ] Suchformular Hidden-Input: `bewertung`-Parameter weitertragen wenn aktiv (wie bei `filter`)

---

### Schritt 4: Template — Bewertungsfilter-UI einbauen

- [ ] Neuen Bewertungsfilter-Bereich in `templates/index.html` hinzufügen:
  - Platzierung: im `.sort-filter`-Bereich zusammen mit "Länger nicht gemacht" und "Nächste 7 Tage" — oder als eigene Unterzeile (konsistentes visuelles Grouping)
  - Zwei Buttons: "★★★+ Nur Gute" und "★★★★★ Favoriten"
  - CSS-Klasse `sort-filter-btn` (konsistent mit bestehenden Sort-Filter-Buttons)
  - Aktiver Zustand: CSS-Klasse `active`, `aria-pressed="{{ bewertung_filter == Some("gut") }}"`
  - HTMX-Attribute: `hx-get`, `hx-target="#recipe-results"`, `hx-push-url="true"`, `hx-select="#recipe-results"`
  - Funktioniert ohne JavaScript (normale `<a>`-Links)
  - Beispiel-Markup für "Nur Gute"-Button:
    ```html
    <a
        href="{{ bewertung_gut_toggle_url }}"
        class="sort-filter-btn{% if bewertung_filter == Some("gut") %} active{% endif %}"
        aria-pressed="{{ bewertung_filter == Some("gut") }}"
        hx-get="{{ bewertung_gut_toggle_url }}"
        hx-target="#recipe-results"
        hx-push-url="true"
        hx-select="#recipe-results"
    >★★★+ Nur Gute</a>
    ```

- [ ] Suchformular Hidden-Input erweitern:
  ```html
  {% if let Some(b) = bewertung_filter %}
  <input type="hidden" name="bewertung" value="{{ b }}">
  {% endif %}
  ```

- [ ] Keine-Treffer-Meldungen für Bewertungsfilter in `templates/index.html` ergänzen:
  - Bewertungsfilter aktiv ohne Treffer: `"Keine Rezepte mit dieser Bewertung gefunden."`
  - Bewertungsfilter + Kategorie ohne Treffer: `"Keine Rezepte mit dieser Bewertung in dieser Kategorie gefunden."`
  - Bewertungsfilter + Suche ohne Treffer: `"Keine Rezepte für "..." mit dieser Bewertung gefunden."`
  - Hinweis: Das Template hat bereits viele Kombinationen; die neuen Fälle mit Bewertungsfilter analog einbauen

- [ ] "Alle"-Button / Reset: aktiver `bewertung`-Filter muss berücksichtigt werden in der Bedingung für den "Alle"-Button-Zustand:
  ```html
  class="category-filter-btn{% if active_categories.is_empty() && !not_made_filter_active && !next_seven_days_filter_active && bewertung_filter.is_none() %} active{% endif %}"
  ```

---

### Schritt 5: CSS — Styling der Bewertungsfilter-Buttons

- [ ] Kein neues CSS nötig, da `.sort-filter-btn` und `.sort-filter-btn.active` bereits in `src/static/css/app.css` für "Länger nicht gemacht" und "Nächste 7 Tage" existieren
- [ ] Sicherstellen, dass die Sterne-Symbole (★) in den Button-Labels korrekt gerendert werden (kein extra Styling nötig, da Sterne bereits im Template für Rezeptkarten eingesetzt werden)
- [ ] Falls nötig: Bewertungsfilter-Buttons in eigener Zeile oder mit einem kleinen Trenner visuell von den Sortier-Filtern absetzen

---

### Schritt 6: Integrationstests (Rust)

- [ ] Neue Testdatei `tests/recipe_rating_filter.rs` erstellen:

  **Hilfsfunktion** `create_recipe_with_rating` (analog zu `create_recipe_with_date` in `recipe_not_made_filter.rs`):
  ```rust
  async fn create_recipe_with_rating(
      app: &axum::Router,
      title: &str,
      categories: &[&str],
      rating: Option<i32>,
  )
  ```

  **Testfälle:**
  - `rating_filter_gut_returns_200_with_three_plus_recipes`
    - Given: Rezept mit 4 Sternen, Rezept mit 2 Sternen, Rezept ohne Bewertung
    - When: GET `/?bewertung=gut`
    - Then: 200, nur 4-Sterne-Rezept im Body
  - `rating_filter_gut_excludes_one_and_two_stars`
    - Given: 1-Stern-Rezept, 2-Sterne-Rezept
    - When: GET `/?bewertung=gut`
    - Then: Keine Rezepte im Body, Hinweistext sichtbar
  - `rating_filter_gut_excludes_unrated_recipes`
    - Given: Rezept ohne Bewertung
    - When: GET `/?bewertung=gut`
    - Then: Rezept nicht im Body
  - `rating_filter_favoriten_returns_only_five_star_recipes`
    - Given: 5-Sterne-Rezept, 4-Sterne-Rezept, 3-Sterne-Rezept
    - When: GET `/?bewertung=favoriten`
    - Then: Nur 5-Sterne-Rezept im Body
  - `rating_filter_favoriten_empty_when_no_five_star`
    - Given: Nur 4-Sterne-Rezepte
    - When: GET `/?bewertung=favoriten`
    - Then: Hinweistext sichtbar, kein Rezept
  - `rating_filter_toggle_deactivates_when_same_value_clicked`
    - Given: URL `/?bewertung=gut` wird aufgerufen
    - Then: Toggle-URL für "Nur Gute" enthält **kein** `bewertung=gut`
  - `rating_filter_combined_with_category`
    - Given: Brot-Rezept 5 Sterne, Mittagessen-Rezept 5 Sterne
    - When: GET `/?bewertung=favoriten&kategorie=Brot`
    - Then: Nur Brot-Rezept im Body
  - `rating_filter_combined_with_search`
    - Given: "Dinkelbrot" 4 Sterne, "Roggenbrot" 4 Sterne
    - When: GET `/?bewertung=gut&q=dinkel`
    - Then: Nur "Dinkelbrot" im Body
  - `rating_filter_combined_with_not_made_filter`
    - Given: "Linseneintopf" 4 Sterne, Vergangenheitsdatum; "Tomatensuppe" 2 Sterne, Vergangenheitsdatum
    - When: GET `/?bewertung=gut&filter=laenger-nicht-gemacht`
    - Then: Nur "Linseneintopf" im Body
  - `deeplink_rating_filter_returns_correct_state`
    - Given: URL `/?bewertung=favoriten` direkt aufgerufen
    - Then: 200, `aria-pressed="true"` für Favoriten-Button im Body
  - `invalid_rating_filter_value_returns_all_recipes`
    - Given: URL `/?bewertung=ungueltig`
    - Then: Alle Rezepte werden angezeigt (ungültige Werte werden ignoriert)
  - Jeden Test mit `// Given / // When / // Then`-Kommentaren

---

### Schritt 7: E2E-Tests (Playwright)

- [ ] Neue Testdatei `tests/e2e/recipe-rating-filter.spec.ts` erstellen

  **Hilfsfunktion** `createRecipeWithRating` (erweitert `createRecipe` aus `recipe-rating.spec.ts`):
  ```typescript
  async function createRecipeWithRating(
    page: Page,
    title: string,
    categories: string[],
    rating?: number,
    plannedDate?: string
  ): Promise<void>
  ```
  Wiederverwenden der `selectRating`-Hilfsfunktion aus `recipe-rating.spec.ts`.

  **Testfälle:**

  - **K1: Filter-Buttons sichtbar und auswählbar**
    - Given: Startseite wird aufgerufen
    - Then: "★★★+ Nur Gute"-Button ist sichtbar (`aria-pressed="false"`)
    - And: "★★★★★ Favoriten"-Button ist sichtbar (`aria-pressed="false"`)

  - **K2: Filter "Nur Gute" (3+ Sterne)**
    - Given: "Spaghetti Bolognese" mit 4 Sternen, "Pfannkuchen" mit 2 Sternen, "Pizza" ohne Bewertung
    - When: Klick auf "Nur Gute"-Button
    - Then: "Spaghetti Bolognese" sichtbar
    - And: "Pfannkuchen" nicht sichtbar
    - And: "Pizza" nicht sichtbar
    - And: URL enthält `bewertung=gut`
    - And: "Nur Gute"-Button hat `aria-pressed="true"` und Klasse `active`

  - **K3: Filter "Favoriten" (5 Sterne)**
    - Given: "Omas Apfelkuchen" mit 5 Sternen, "Nudelsuppe" mit 4 Sternen, "Rührei" mit 3 Sternen
    - When: Klick auf "Favoriten"-Button
    - Then: Nur "Omas Apfelkuchen" sichtbar
    - And: URL enthält `bewertung=favoriten`
    - And: "Favoriten"-Button hat `aria-pressed="true"` und Klasse `active`

  - **K4: Aktiver Filter visuell erkennbar**
    - Given: URL `/?bewertung=gut` direkt aufgerufen
    - Then: "Nur Gute"-Button hat `aria-pressed="true"` und Klasse `active`
    - And: "Favoriten"-Button hat `aria-pressed="false"` (nicht aktiv)

  - **K5: Filter zurücksetzen (Toggle)**
    - Given: Filter "Nur Gute" ist aktiv (URL `/?bewertung=gut`)
    - When: Erneuter Klick auf "Nur Gute"-Button
    - Then: Filter deaktiviert (`aria-pressed="false"`)
    - And: URL enthält kein `bewertung`-Parameter mehr
    - And: Alle Rezepte wieder sichtbar

  - **K6: Keine Treffer — Hinweistext erscheint**
    - Given: Alle Rezepte in der App haben maximal 2 Sterne oder keine Bewertung (eindeutiger Suffix)
    - When: Klick auf "Nur Gute"
    - Then: Hinweistext "Keine Rezepte mit dieser Bewertung gefunden" sichtbar
    - And: Keine Rezept-Karte vorhanden (`.recipe-item` count 0)

  - **K7: DeepLink `?bewertung=favoriten`**
    - Given: "Omas Apfelkuchen" mit 5 Sternen existiert
    - When: URL `/?bewertung=favoriten` direkt aufgerufen
    - Then: Nur "Omas Apfelkuchen" sichtbar
    - And: "Favoriten"-Button ist aktiv markiert

  - **K8: Kombination mit Kategorie-Filter**
    - Given: "Dinkelbrot" (Brot, 5 Sterne), "Roggenbrot" (Brot, 2 Sterne), "Spaghetti Bolognese" (Mittagessen, 5 Sterne)
    - When: Kategorie "Brot" gewählt + Klick auf "Favoriten"
    - Then: Nur "Dinkelbrot" sichtbar
    - And: URL enthält sowohl `kategorie=Brot` als auch `bewertung=favoriten`

  - **K9: Kombination mit Volltextsuche**
    - Given: "Dinkelbrot" (4 Sterne), "Roggenbrot" (4 Sterne), "Dinkelpfannkuchen" (1 Stern)
    - When: Suche "Dinkel" + Klick auf "Nur Gute"
    - Then: "Dinkelbrot" sichtbar, "Roggenbrot" nicht, "Dinkelpfannkuchen" nicht
    - And: URL enthält `q=Dinkel` und `bewertung=gut`

  - **K10: Kombination mit "Länger nicht gemacht"**
    - Given: "Linseneintopf" (4 Sterne, planned_date 2025-01-01), "Erbsensuppe" (4 Sterne, planned_date 2026-01-01), "Tomatensuppe" (2 Sterne, planned_date 2024-01-01)
    - When: Klick auf "Nur Gute" + Klick auf "Länger nicht gemacht"
    - Then: "Linseneintopf" erscheint vor "Erbsensuppe" (älteres Datum zuerst)
    - And: "Tomatensuppe" nicht sichtbar (zu niedrige Bewertung)
    - And: URL enthält `bewertung=gut` und `filter=laenger-nicht-gemacht`

  - Jeder Test mit deutschen Given/When/Then-Kommentaren

---

### Schritt 8: Seed-Daten für E2E-Tests

- [ ] Neue Seed-Datei `tests/seeds/recipe-rating-filter.sql` erstellen:
  ```sql
  -- Seed-Daten für Story 11: Filter nach Bewertung
  INSERT INTO recipes (title, categories, rating, planned_date) VALUES
      ('Fünf-Sterne-Gericht', '["Mittagessen"]', 5, NULL),
      ('Vier-Sterne-Gericht', '["Mittagessen"]', 4, '2025-01-01'),
      ('Drei-Sterne-Gericht', '["Brot"]', 3, NULL),
      ('Zwei-Sterne-Gericht', '["Snacks"]', 2, '2024-06-01'),
      ('Ein-Stern-Gericht', '["Party"]', 1, NULL),
      ('Unbewertetes-Gericht', '["Kuchen"]', NULL, NULL);
  ```
  Hinweis: Die E2E-Tests erstellen Rezepte direkt über das Formular (via `createRecipeWithRating`) für Test-Isolation — der Seed ist optional und nur für Tests nützlich, die keine eigene DB-Isolation benötigen.

---

### Schritt 9: Qualitätschecks (DoD)

- [ ] `cargo fmt` — Code formatieren
- [ ] `cargo clippy -- -D warnings` — keine Warnungen
- [ ] `cargo build` — keine Compilerfehler
- [ ] `cargo test` — alle Unit- und Integrationstests grün
- [ ] `npm run test:e2e` — alle E2E-Tests grün
- [ ] Öffentliche Funktionen in `recipe_db.rs` mit Doc-Kommentar (`///`) versehen
- [ ] ARIA-Attribute manuell prüfen (`aria-pressed` korrekt für beide Buttons)
- [ ] Tastaturnavigation manuell prüfen (Tab + Enter/Space für Filter-Buttons)
- [ ] DeepLinks manuell prüfen (Bookmark setzen und URL neu laden)

---

## URL-Struktur

```
GET  /                                              → Alle Rezepte (alphabetisch)
GET  /?bewertung=gut                                → Nur Rezepte mit rating >= 3
GET  /?bewertung=favoriten                          → Nur Rezepte mit rating = 5
GET  /?bewertung=gut&kategorie=Brot                 → Gute Brot-Rezepte
GET  /?bewertung=gut&q=pasta                        → Gute Rezepte mit "pasta"
GET  /?bewertung=gut&filter=laenger-nicht-gemacht   → Gute Rezepte nach Datum sortiert
GET  /?bewertung=favoriten&kategorie=Mittagessen    → Favoriten aus Mittagessen
GET  /?bewertung=gut&kategorie=Brot&q=dinkel        → Alle drei Filter kombiniert
```

---

## Abhängigkeiten

- Story 14 (Bewertung implementieren) ist abgeschlossen — `rating`-Feld existiert in DB und allen Schichten
- Story 5 (alphabetische Sortierung) ist implementiert — `normalize_for_sort` wird für alphabetische Sortierung in gefilterten Ergebnissen genutzt
- Story 8 (Kategorie-Filter) ist implementiert — `build_category_toggle_url` muss `bewertung`-Parameter weitertragen
- Story 9 ("Länger nicht gemacht") ist implementiert — `filter_recipes_not_made_recently` wird erweitert, Toggle-URLs tragen `bewertung` mit
- Story 10 ("Nächste 7 Tage") ist implementiert — `filter_recipes_next_seven_days` wird erweitert, Toggle-URLs tragen `bewertung` mit
- Keine neue Datenbank-Migration nötig — `rating`-Feld existiert bereits
- `urlencoding`-Crate ist bereits als Abhängigkeit vorhanden

---

## Test-Checkliste

### Unit-Tests (in `src/models/recipe_db.rs`)
- [ ] Unit-Test: `rating_filter_gut_returns_only_three_plus_stars`
- [ ] Unit-Test: `rating_filter_favoriten_returns_only_five_stars`
- [ ] Unit-Test: `rating_filter_none_returns_all_recipes`
- [ ] Unit-Test: `rating_filter_excludes_unrated_recipes`
- [ ] Unit-Test: `rating_filter_combined_with_category`
- [ ] Unit-Test: `rating_filter_combined_with_search`
- [ ] Unit-Test: `rating_filter_gut_returns_empty_if_no_qualifying_recipes`
- [ ] Unit-Test: `rating_filter_favoriten_returns_empty_if_no_five_star`

### Integrationstests (in `tests/recipe_rating_filter.rs`)
- [ ] Integrationstest: `rating_filter_gut_returns_200_with_three_plus_recipes`
- [ ] Integrationstest: `rating_filter_gut_excludes_one_and_two_stars`
- [ ] Integrationstest: `rating_filter_gut_excludes_unrated_recipes`
- [ ] Integrationstest: `rating_filter_favoriten_returns_only_five_star_recipes`
- [ ] Integrationstest: `rating_filter_favoriten_empty_when_no_five_star`
- [ ] Integrationstest: `rating_filter_toggle_deactivates_when_same_value_clicked`
- [ ] Integrationstest: `rating_filter_combined_with_category`
- [ ] Integrationstest: `rating_filter_combined_with_search`
- [ ] Integrationstest: `rating_filter_combined_with_not_made_filter`
- [ ] Integrationstest: `deeplink_rating_filter_returns_correct_state`
- [ ] Integrationstest: `invalid_rating_filter_value_returns_all_recipes`

### E2E-Tests (in `tests/e2e/recipe-rating-filter.spec.ts`)
- [ ] E2E-Test: K1 – Filter-Buttons sichtbar und auswählbar
- [ ] E2E-Test: K2 – Filter "Nur Gute" zeigt 3+ Sterne
- [ ] E2E-Test: K3 – Filter "Favoriten" zeigt nur 5 Sterne
- [ ] E2E-Test: K4 – Aktiver Filter visuell erkennbar (DeepLink)
- [ ] E2E-Test: K5 – Filter zurücksetzen (Toggle)
- [ ] E2E-Test: K6 – Keine Treffer, Hinweistext erscheint
- [ ] E2E-Test: K7 – DeepLink `?bewertung=favoriten`
- [ ] E2E-Test: K8 – Kombination Bewertung + Kategorie
- [ ] E2E-Test: K9 – Kombination Bewertung + Volltextsuche
- [ ] E2E-Test: K10 – Kombination Bewertung + "Länger nicht gemacht"

### Manuelle Tests
- [ ] Manueller Test: Tastaturnavigation (Tab + Enter/Space) für beide Filter-Buttons
- [ ] Manueller Test: `aria-pressed` korrekt (aktiv/inaktiv) in Browser-DevTools prüfen
- [ ] Manueller Test: Beide Filter gleichzeitig aktiv (visuelle Darstellung korrekt)
- [ ] Manueller Test: DeepLink als Lesezeichen speichern und neu laden
- [ ] Manueller Test: Suchformular mit aktivem `bewertung`-Filter — Parameter wird korrekt weitergeleitet

---

## Offene Punkte

- **Bewertungsfilter im `build_reset_url`:** Der "Alle"-Kategorie-Reset (`reset_categories_url`) soll nur die Kategorien zurücksetzen, aber `bewertung` und `filter` beibehalten. Soll `build_reset_url` erweitert werden oder bleibt der "Alle"-Button ein vollständiger Reset aller Filter? **Empfehlung:** Nur Kategorien zurücksetzen — analog zum bestehenden Verhalten, das `q` und `filter` beibehält.
- **Sterndarstellung in Buttons:** Die Sterne in Button-Labels (★★★+ Nur Gute) sind Unicode-Zeichen. Falls Rendering-Probleme auftreten, können CSS-Stile oder HTML-Entities verwendet werden.
