# Implementierungsplan: Story 44 – Bewertungsmechanismus entfernen

## Technische Schritte

### Schritt 1: Datenbank-Migration
- [ ] **Migration `003_remove_rating.sql` erstellen:** `ALTER TABLE recipes DROP COLUMN rating;`
- [ ] **sqlx-Migrationssystem prüfen:** Das Projekt nutzt bereits `sqlx migrate run` (CLI). Die Migration wird automatisch beim nächsten App-Start oder manuell via `sqlx migrate run` ausgeführt.
- [ ] `cargo sqlx prepare` ausführen, damit sqlx die geänderten Queries bei Compile-Zeit validieren kann (oder DB lokal laufen lassen und `SQLX_OFFLINE=true` verwenden).

**Betroffene Dateien:** `migrations/003_remove_rating.sql`

---

### Schritt 2: Datenbank-Layer
- [ ] `Recipe.rating` in `src/models/recipe.rs` entfernen.
- [ ] `create_recipe` in `src/models/recipe_db.rs`: `rating` aus INSERT-Statement und Bind entfernen.
- [ ] `get_recipe_by_id` in `src/models/recipe_db.rs`: `rating` aus SELECT entfernen.
- [ ] `get_all_recipes` in `src/models/recipe_db.rs`: `rating` aus SELECT entfernen.
- [ ] `update_recipe` in `src/models/recipe_db.rs`: `rating` aus UPDATE-Statement und Bind entfernen.
- [ ] `update_recipe_rating()` in `src/models/recipe_db.rs` entfernen.
- [ ] `search_recipes` in `src/models/recipe_db.rs`: `rating` aus SELECT entfernen.
- [ ] `rating_sql_clause()` und alle Bewertungsfilter-Logiken in `src/models/recipe_db.rs` entfernen (`filter_recipes_by_categories`, `filter_recipes_not_made_recently`, `filter_recipes_next_seven_days`).
- [ ] `get_recipes_current_week`, `get_recipes_drei_tage`, `get_recipes_by_date_range` in `src/models/recipe_db.rs`: `rating` aus SELECT entfernen.
- [ ] `SimilarRecipe.rating` und `DublettenPaar` rating-Felder in `src/models/recipe_db.rs` entfernen.
- [ ] `find_similar_recipes` in `src/models/recipe_db.rs`: `rating` aus SELECT entfernen.
- [ ] `merge_recipes` in `src/models/recipe_db.rs`: `rating` aus UPDATE-Statement und Bind entfernen.
- [ ] Unit-Test: `recipe_db.rs` – alle Rating-Filter-Tests entfernen oder umschreiben.

**Betroffene Dateien:** `src/models/recipe_db.rs`, `src/models/recipe.rs`

---

### Schritt 3: Modelle & Validierung
- [ ] `validate_rating()` in `src/models/recipe.rs` entfernen.
- [ ] `validate_recipe_fields()` in `src/models/recipe.rs`: `rating`-Parameter entfernen.
- [ ] `CreateRecipe.rating` und `UpdateRecipe.rating` in `src/models/recipe.rs` entfernen.
- [ ] `CreateRecipe.validate()` und `UpdateRecipe.validate()` anpassen (kein `validate_rating`-Aufruf).
- [ ] `determine_merge_target()` in `src/models/recipe.rs`: Rating-Logik entfernen (nur noch Felder ausgefüllt, updated_at, ID).
- [ ] Unit-Test: `validate_rating_*`-Tests entfernen.
- [ ] Unit-Test: `determine_merge_target`-Tests anpassen (Rating-Fälle entfernen).
- [ ] Unit-Test: `create_recipe_*`-Tests anpassen (kein `rating`-Feld mehr).

**Betroffene Dateien:** `src/models/recipe.rs`

---

### Schritt 4: Routes & Handler
- [ ] `parse_rating()` in `src/routes/recipes.rs` entfernen.
- [ ] `update_recipe_rating_handler()` in `src/routes/recipes.rs` entfernen.
- [ ] `create_recipe_handler` in `src/routes/recipes.rs`: Kein `rating` mehr aus Form-Daten parsen.
- [ ] `update_recipe_handler` in `src/routes/recipes.rs`: Kein `rating` mehr aus Form-Daten parsen.
- [ ] `merge_handler` in `src/routes/recipes.rs`: Kein `rating_from` mehr parsen, `rating` nicht mehr in `UpdateRecipe` setzen.
- [ ] `IndexQuery.bewertung` behalten (für Graceful Degradation), aber in `index()` ignorieren (keine Filterlogik, keine URL-Builder für Bewertung).
- [ ] `build_bewertung_toggle_url()`, `build_current_query_string()`, `build_filter_collapsed_toggle_url()`, `build_category_toggle_url()`, `build_reset_url()`, `build_not_made_toggle_url()`, `build_next_seven_days_toggle_url()` in `src/routes/recipes.rs`: `bewertung`-Parameter entfernen.
- [ ] `index()` in `src/routes/recipes.rs`: `bewertung_gut_toggle_url`, `bewertung_favoriten_toggle_url`, `bewertung_filter` aus `IndexTemplate` entfernen.
- [ ] `recipe_to_merge_info()` in `src/routes/recipes.rs`: `rating` nicht mehr übergeben.
- [ ] `duplicates_handler()` in `src/routes/recipes.rs`: `bewertung_a/b` nicht mehr übergeben.
- [ ] `src/routes/heute.rs`: `heute_rating_handler()` entfernen, `parse_rating()` entfernen, `HeuteRezeptItem` ohne Rating aufbauen.
- [ ] `src/routes/mod.rs`: POST-Routes `/recipes/:id/rating` und `/heute/recipes/:id/rating` entfernen.
- [ ] Integrationstest: `tests/recipe_rating_filter.rs` entfernen.
- [ ] Integrationstest: `tests/recipe_rating.rs` entfernen.
- [ ] Integrationstest: `tests/saved_filters.rs` anpassen (keine `bewertung` in Query-Strings).
- [ ] Unit-Test in `routes/recipes.rs`: URL-Builder-Tests mit `bewertung` entfernen.

**Betroffene Dateien:** `src/routes/recipes.rs`, `src/routes/heute.rs`, `src/routes/mod.rs`, `tests/recipe_rating_filter.rs`, `tests/recipe_rating.rs`, `tests/saved_filters.rs`

---

### Schritt 5: Templates (Rust structs)
- [ ] `RecipeListItem.rating` und `stars_display()` in `src/templates.rs` entfernen.
- [ ] `RecipeDetailTemplate.rating` und `star_filled()`/`rating_is_active()` in `src/templates.rs` entfernen.
- [ ] `RecipeFormTemplate.rating` und `rating_is()` in `src/templates.rs` entfernen.
- [ ] `IndexTemplate.bewertung_filter`, `bewertung_gut_toggle_url`, `bewertung_favoriten_toggle_url` in `src/templates.rs` entfernen.
- [ ] `HeuteRezeptItem.rating`, `rating_is_active()`, `star_filled()` in `src/templates.rs` entfernen.
- [ ] `InlineRatingTemplate` in `src/templates.rs` entfernen.
- [ ] `InlineRatingHeuteTemplate` in `src/templates.rs` entfernen.
- [ ] `DublettenPaarItem.bewertung_a/b`, `sterne_a()`, `sterne_b()`, `rating_label_a()`, `rating_label_b()` in `src/templates.rs` entfernen.
- [ ] `MergeRezeptInfo.rating`, `sterne()`, `hat_konflikt_rating()` in `src/templates.rs` entfernen.

**Betroffene Dateien:** `src/templates.rs`

---

### Schritt 6: HTML-Templates (Askama)
- [ ] `templates/index.html`: Bewertungsfilter-Buttons ("★★★+ Nur Gute", "★★★★★ Favoriten") entfernen. Bewertungs-Query-Parameter im Suchformular entfernen. Sterne in der Rezeptliste (`recipe.stars_display()`) entfernen. "Keine Rezepte mit dieser Bewertung"-Meldungen entfernen.
- [ ] `templates/recipes/detail.html`: `{% include "recipes/_inline_rating.html" %}` entfernen.
- [ ] `templates/recipes/form.html`: `star-rating`-Fieldset komplett entfernen.
- [ ] `templates/heute.html`: Inline-Rating-Formular (alle 5 Sterne-Buttons) entfernen, nur noch Titel-Link anzeigen.
- [ ] `templates/recipes/duplicates.html`: Sterne-Anzeige (`sterne_a`, `sterne_b`) entfernen.
- [ ] `templates/recipes/merge.html`: Gesamten "Bewertung"-Block (Konflikt + Auto-Übernahme) entfernen, `rating_from` Hidden-Input entfernen.
- [ ] `templates/recipes/_inline_rating.html` löschen.
- [ ] `templates/recipes/_inline_rating_heute.html` löschen.

**Betroffene Dateien:** `templates/index.html`, `templates/recipes/detail.html`, `templates/recipes/form.html`, `templates/heute.html`, `templates/recipes/duplicates.html`, `templates/recipes/merge.html`, `templates/recipes/_inline_rating.html`, `templates/recipes/_inline_rating_heute.html`

---

### Schritt 7: Styling
- [ ] `src/static/css/app.css`: Alle `.star-rating`, `.star-filled`, `.star-empty`, `.star-rating-options`, `.star-rating-none`, `.recipe-stars`, `.recipe-stars-list`, `.inline-rating`, `.inline-rating-form`, `.inline-rating-btn`, `.duplicate-card .stars` Regeln entfernen.

**Betroffene Dateien:** `src/static/css/app.css`

---

### Schritt 8: E2E-Tests
- [ ] `tests/e2e/recipe-rating.spec.ts` löschen (Story 14/17/41 Tests entfallen).
- [ ] `tests/e2e/recipe-rating-filter.spec.ts` löschen (Story 11 Tests entfallen).
- [ ] `tests/e2e/recipe-edit-rating.spec.ts` löschen (Story 41 Tests entfallen).
- [ ] `tests/e2e/recipe-combined-filters.spec.ts` anpassen: Keine `bewertung`-Parameter in URLs, keine `selectRating`-Hilfsfunktion.
- [ ] `tests/e2e/saved-filters.spec.ts` anpassen: Keine `bewertung`-Parameter in URLs, keine `selectRating`-Hilfsfunktion.
- [ ] `tests/e2e/accessibility.spec.ts` anpassen: Keine Prüfung auf `#inline-rating`, kein `star-rating`-Fieldset-Check.
- [ ] `tests/e2e/recipe-merge.spec.ts` anpassen: Kein `rating`-Parameter beim Rezept-Erstellen.
- [ ] Neuen E2E-Test `tests/e2e/remove-rating.spec.ts` erstellen:
  - Test 1: Startseite zeigt keine Bewertungsfilter-Buttons.
  - Test 2: Rezept-Liste zeigt keine Sterne.
  - Test 3: Detailansicht zeigt kein Inline-Rating.
  - Test 4: Bearbeitungsformular hat kein Bewertungsfeld.
  - Test 5: Neues Rezept ohne Bewertung funktioniert.
  - Test 6: "Heute gekocht" zeigt keine Sterne.
  - Test 7: Dubletten/Merge zeigen keine Bewertungen.
  - Test 8: DeepLink `?bewertung=gut` führt nicht zu Fehler (Graceful Degradation).
  - Test 9: POST `/recipes/:id/rating` gibt 404 zurück.
  - Test 10: POST `/heute/recipes/:id/rating` gibt 404 zurück.

**Betroffene Dateien:** `tests/e2e/recipe-rating.spec.ts`, `tests/e2e/recipe-rating-filter.spec.ts`, `tests/e2e/recipe-edit-rating.spec.ts`, `tests/e2e/recipe-combined-filters.spec.ts`, `tests/e2e/saved-filters.spec.ts`, `tests/e2e/accessibility.spec.ts`, `tests/e2e/recipe-merge.spec.ts`, `tests/e2e/remove-rating.spec.ts` (neu)

---

### Schritt 9: Qualitätschecks & Dokumentation
- [ ] `cargo build` – keine Compiler-Fehler.
- [ ] `cargo clippy -- -D warnings` – keine Warnungen.
- [ ] `cargo fmt --check` – korrekt formatiert.
- [ ] `cargo test` – alle Unit- und Integrationstests grün.
- [ ] `npm run test:e2e` – alle Playwright-Tests grün.
- [ ] AXE-Level-A-Tests prüfen (keine regressiven Accessibility-Probleme durch entfernte Elemente).
- [ ] `architecture.md` aktualisieren: Rating-Query-Parameter und Endpunkte aus der Routing-Dokumentation entfernen.
- [ ] Prüfen, dass keine Dead Code-Warnungen verbleiben (`cargo check`).

---

## URL-Struktur (Änderungen)

```
ENTFALLEN:
POST /recipes/:id/rating       → Rating-Update (entfällt komplett)
POST /heute/recipes/:id/rating → Rating-Update (entfällt komplett)

UNVERÄNDERT (Graceful Degradation):
GET  /?bewertung=gut            → Parameter wird stillschweigend ignoriert
GET  /?bewertung=favoriten      → Parameter wird stillschweigend ignoriert
```

---

## Abhängigkeiten

- Keine Story-Abhängigkeiten (Story 44 ist die letzte im Epic 4).
- Technisch: Alle vorherigen Rating-Stories (11, 14, 17, 41) müssen abgeschlossen sein – sind es.

---

## Test-Checkliste

- [ ] Unit-Test: `determine_merge_target` bevorzugt mehr ausgefüllte Felder (ohne Rating).
- [ ] Unit-Test: `validate_recipe_fields` akzeptiert Rezepte ohne Rating-Parameter.
- [ ] Integrationstest: GET `/?bewertung=gut` gibt 200 zurück, zeigt aber alle Rezepte (Filter ignoriert).
- [ ] Integrationstest: POST `/recipes/:id/rating` gibt 404 zurück.
- [ ] Integrationstest: POST `/heute/recipes/:id/rating` gibt 404 zurück.
- [ ] Integrationstest: Migration `003` läuft erfolgreich auf frischer DB.
- [ ] E2E-Test: Startseite ohne Bewertungsfilter-Buttons.
- [ ] E2E-Test: Rezept erstellen ohne Bewertung funktioniert.
- [ ] E2E-Test: Rezept bearbeiten ohne Bewertungsfeld.
- [ ] E2E-Test: Detailansicht ohne Inline-Rating.
- [ ] E2E-Test: Listenansicht ohne Sterne.
- [ ] E2E-Test: "Heute gekocht" ohne Sterne.
- [ ] E2E-Test: Dubletten/Merge ohne Bewertungsanzeige.
- [ ] E2E-Test: AXE-Level-A bleibt grün.

---

## Offene Punkte

- **Hinweis zur Datenmigration:** Die `003_remove_rating.sql` entfernt die Spalte physisch. Bestehende Bewertungswerte gehen damit verloren. Das ist beabsichtigt, da das Feature komplett abgeschaltet wird und kein toter Code im Schema verbleiben soll. Für ein Backup der Daten vor dem Deployment sollte eine manuelle DB-Sicherung erfolgen.
