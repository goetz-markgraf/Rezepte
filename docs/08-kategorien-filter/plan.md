# Implementierungsplan: Story 8 - Filter nach Kategorien

## Technische Schritte

### Schritt 1: Datenbank-Layer — Kategorie-Filter-Query

- [ ] Neue Funktion `filter_recipes_by_categories` in `src/models/recipe_db.rs` erstellen
  - Parameter: `pool: &SqlitePool`, `categories: &[String]`, `search_query: &str`
  - Gibt `Vec<Recipe>` zurück, alphabetisch sortiert (gleiche `normalize_for_sort`-Logik wie bei `search_recipes`)
  - Wenn `categories` leer ist und `search_query` leer → alle Rezepte (`get_all_recipes`)
  - Wenn nur `search_query` gesetzt → bestehende `search_recipes`-Logik
  - Wenn `categories` gesetzt → ODER-Logik: Rezept erscheint, wenn mindestens eine Kategorie passt
  - Wenn beides gesetzt → UND-Verknüpfung: Kategorie-Filter UND Suchbegriff
  - SQLite JSON-Abfrage: `LOWER(categories) LIKE '%"mittagessen"%'` pro Kategorie (case-insensitiv, sicher via Parameter)
  - Alternativ: Dynamisch mehrere `json_each`-Bedingungen aufbauen mit OR

- [ ] SQL-Strategie: LIKE-Abfragen auf den JSON-String (einfach, sicher für kleinen Datenbestand)
  - Jede Kategorie wird als `%"Kategorie"%` gebunden (Anführungszeichen sichern Teilstring-Grenzen)
  - Beispiel: `WHERE LOWER(categories) LIKE '%"brot"%' OR LOWER(categories) LIKE '%"kuchen"%'`
  - Kategorienamen sind hardcoded und valide (kein SQL-Injection-Risiko bei Verwendung von sqlx-Parametern)

- [ ] Unit-Test in `recipe_db.rs`:
  - `filter_by_single_category_returns_matching_recipes`
  - `filter_by_multiple_categories_uses_or_logic`
  - `filter_returns_empty_for_category_without_recipes`
  - `filter_combined_with_search_uses_and_logic`
  - `filter_with_no_categories_returns_all_recipes`
  - `filter_result_is_alphabetically_sorted`

---

### Schritt 2: Modell — IndexQuery erweitern

- [ ] `IndexQuery`-Struct in `src/routes/recipes.rs` um Kategorie-Parameter erweitern:
  ```rust
  pub struct IndexQuery {
      pub deleted: Option<String>,
      pub q: Option<String>,
      pub kategorie: Option<Vec<String>>,  // ?kategorie=brot&kategorie=kuchen
  }
  ```
  - `serde` mit `#[serde(default)]` für leere Liste wenn nicht gesetzt
  - Axum deserialisiert mehrfache Query-Parameter automatisch in `Vec<String>`

- [ ] `IndexTemplate` in `src/templates.rs` um aktive Kategorien erweitern:
  ```rust
  pub struct IndexTemplate {
      pub recipes: Vec<RecipeListItem>,
      pub deleted_title: Option<String>,
      pub search_query: String,
      pub active_categories: Vec<String>,   // neu
      pub all_categories: Vec<String>,      // neu (für Render-Logik)
  }
  ```

- [ ] Neue DB-Funktion in `src/models/mod.rs` re-exportieren

---

### Schritt 3: Route/Handler — Kategorie-Filter anwenden

- [ ] `index`-Handler in `src/routes/recipes.rs` anpassen:
  - `kategorie`-Parameter aus `IndexQuery` auslesen (Fallback: leere `Vec`)
  - Kategorienamen normalisieren (Großschreibung wie in `VALID_CATEGORIES`): `"brot"` → `"Brot"`
  - Ungültige Kategorien aus Query-Parametern stillschweigend ignorieren
  - `filter_recipes_by_categories(&pool, &selected_categories, &search_query)` aufrufen
  - `IndexTemplate` mit `active_categories` und `all_categories` befüllen

- [ ] URL-Generierung für Kategorie-Buttons (Toggle-Links):
  - Aktive Kategorie klicken → aus URL entfernen
  - Inaktive Kategorie klicken → zur URL hinzufügen
  - Bestehende `q`-Parameter bleiben erhalten
  - Template-Helper-Funktion oder Logik im Template (Askama-Filter)

---

### Schritt 4: Template — Kategorie-Filter-UI

- [ ] Kategorie-Filter-Bereich in `templates/index.html` vor der Rezeptliste einfügen:
  - Alle fünf Kategorien als klickbare `<a>`-Elemente (Links, keine `<button>`)
  - Link-URL enthält alle aktiven Kategorien + ggf. bestehenden Suchbegriff
  - Aktive Kategorie erhält CSS-Klasse `active` und `aria-pressed="true"`
  - Inaktive Kategorie: `aria-pressed="false"`
  - "Alle"-Link (ohne Kategorie-Parameter) zum Zurücksetzen aller Filter
  - Tastatur-bedienbar (Links sind nativ fokussierbar per Tab)

- [ ] Kein-Treffer-Meldung anpassen:
  - Bei aktiven Kategorien ohne Treffer: `"Keine Rezepte in dieser Kategorie"`
  - Bei Suche ohne Treffer: bestehende Meldung mit Suchbegriff
  - Bei beidem: kombinierte Meldung

- [ ] HTMX-Integration (Progressive Enhancement):
  - Kategorie-Links erhalten `hx-get="/"`, `hx-target="#recipe-results"`, `hx-push-url="true"`, `hx-select="#recipe-results"`
  - Gleiche Attribute wie Suche → Filter ohne Seitenneuladung, DeepLink bleibt erhalten
  - Funktioniert auch ohne JavaScript (normale Link-Navigation)

- [ ] CSS für Kategorie-Filter in `src/static/css/app.css`:
  - `.category-filter` Container
  - `.category-filter-btn` für inaktive Buttons
  - `.category-filter-btn.active` für aktive Buttons (visuell hervorgehoben)
  - Responsive: Buttons wrappen auf kleinen Bildschirmen

---

### Schritt 5: Integrationstests (Rust)

- [ ] Neue Testdatei `tests/recipe_category_filter.rs` erstellen:
  - `filter_by_single_category_shows_matching_recipe`
  - `filter_by_single_category_hides_non_matching_recipe`
  - `filter_by_multiple_categories_shows_all_matching`
  - `filter_returns_empty_state_message_for_category_without_recipes`
  - `filter_combined_with_search_applies_and_logic`
  - `filter_resets_when_no_kategorie_param`
  - `deeplink_with_kategorie_param_returns_200`
  - Jeder Test mit `// Given / // When / // Then`-Kommentaren

---

### Schritt 6: E2E-Tests (Playwright)

- [ ] Neue Testdatei `tests/e2e/recipe-category-filter.spec.ts` erstellen:

  - **K1: Kategorien sichtbar**
    - Alle 5 Kategorien sind auf der Startseite sichtbar
    - Reihenfolge ist korrekt: Mittagessen, Brot, Party, Kuchen, Snacks

  - **K2: Einzelne Kategorie filtern**
    - Rezept "Vollkornbrot" (Brot) und "Spaghetti" (Mittagessen) anlegen
    - Klick auf "Brot" → nur "Vollkornbrot" sichtbar
    - URL enthält `kategorie=Brot`
    - "Brot"-Button ist visuell aktiv (`aria-pressed="true"`)

  - **K3: Mehrere Kategorien gleichzeitig**
    - "Käsekuchen" (Kuchen) und "Partybrot" (Brot, Party) und "Spaghetti" (Mittagessen)
    - Klick auf "Kuchen", dann "Brot" → beide Rezepte sichtbar, "Spaghetti" nicht

  - **K4: Filter zurücksetzen**
    - "Brot" aktiv → Klick auf "Alle" → vollständige Liste, keine `kategorie`-Parameter in URL
    - Alternativ: erneuter Klick auf aktive Kategorie hebt Filter auf

  - **K5: Keine Treffer**
    - Keine Rezepte in "Snacks" vorhanden → Klick auf "Snacks" → Meldung sichtbar

  - **K6: DeepLink**
    - URL `/?kategorie=Party` direkt aufrufen → Party-Rezepte angezeigt, "Party" visuell aktiv

  - **K7: Kombination mit Suche**
    - "Dinkelbrot" (Brot) und "Roggenbrot" (Brot) anlegen
    - Kategorie "Brot" + Suchbegriff "Dinkel" → nur "Dinkelbrot" sichtbar

  - Jeder Test mit deutschen Given/When/Then-Kommentaren
  - `createRecipe`-Hilfsfunktion (wie in `recipe-search.spec.ts`) wiederverwenden

---

### Schritt 7: Qualitätschecks

- [ ] `cargo fmt` — Code formatieren
- [ ] `cargo clippy -- -D warnings` — keine Warnungen
- [ ] `cargo build` — keine Compilerfehler
- [ ] `cargo test` — alle Unit- und Integrationstests grün
- [ ] `npm run test:e2e` — alle E2E-Tests grün

---

## URL-Struktur

```
GET  /                           → Alle Rezepte (keine Filter)
GET  /?kategorie=Brot            → Nur Brot-Rezepte
GET  /?kategorie=Brot&kategorie=Kuchen  → Brot ODER Kuchen
GET  /?q=dinkel&kategorie=Brot   → Suche UND Kategorie-Filter
GET  /?kategorie=Brot&q=dinkel   → Gleich wie oben (Parameter-Reihenfolge egal)
```

---

## Abhängigkeiten

- Story 5 (alphabetische Sortierung) ist implementiert — `normalize_for_sort` wird wiederverwendet
- Story 7 (Volltextsuche) ist implementiert — `IndexQuery.q` und `search_query` werden kombiniert
- Keine neuen Datenbank-Migrationen nötig (`categories` als JSON-Array existiert bereits)
- `VALID_CATEGORIES`-Konstante in `src/models/recipe.rs` wird wiederverwendet

---

## Test-Checkliste

- [ ] Unit-Test: `filter_by_single_category_returns_matching_recipes`
- [ ] Unit-Test: `filter_by_multiple_categories_uses_or_logic`
- [ ] Unit-Test: `filter_returns_empty_for_category_without_recipes`
- [ ] Unit-Test: `filter_combined_with_search_uses_and_logic`
- [ ] Unit-Test: `filter_result_is_alphabetically_sorted`
- [ ] Integrationstest: HTTP-Handler mit `?kategorie=`-Parameter gibt 200 zurück
- [ ] Integrationstest: Gefilterter Body enthält korrekte Rezepte
- [ ] Integrationstest: Keine-Treffer-Meldung erscheint bei leerer Kategorie
- [ ] E2E-Test: K1 – Alle Kategorien sichtbar
- [ ] E2E-Test: K2 – Einzelfilter zeigt korrekte Rezepte und aktiven Zustand
- [ ] E2E-Test: K3 – Mehrfachfilter mit ODER-Logik
- [ ] E2E-Test: K4 – Filter zurücksetzen via "Alle"
- [ ] E2E-Test: K5 – Keine-Treffer-Meldung
- [ ] E2E-Test: K6 – DeepLink mit `?kategorie=` funktioniert
- [ ] E2E-Test: K7 – Kombination Suche + Kategorie
- [ ] Manueller Test: Tastatur-Navigation durch Kategorie-Buttons (Tab + Enter)
- [ ] Manueller Test: ARIA `aria-pressed` korrekt gesetzt

---

## Offene Punkte

- Normalisierung der URL-Parameterwerte: Soll `?kategorie=brot` (lowercase) oder `?kategorie=Brot` (wie in DB) verwendet werden? Empfehlung: Groß wie in `VALID_CATEGORIES` für direkte DB-Kompatibilität, bei Eingabe case-insensitiv matchen.
- HTMX-Target: Sollen auch die Kategorie-Buttons bei HTMX-Requests korrekt ihren Zustand aktualisieren? Da die Buttons im selben `#recipe-results`-Bereich liegen oder außerhalb, muss `hx-select` ggf. den gesamten Filterbereich einschließen (oder ein eigenes HTMX-Target für den Filter).
