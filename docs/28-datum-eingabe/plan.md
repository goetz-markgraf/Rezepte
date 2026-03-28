# Implementierungsplan: Story 28 - Datum-Eingabe am Rezept (geplant / gekocht)

## Technische Analyse

### Ist-Zustand

- `Recipe`-Struct hat kein `planned_date`-Feld (obwohl DB-Schema bereits `planned_date DATE` enthält)
- `CreateRecipe` und `UpdateRecipe` haben kein `planned_date`-Feld
- DB-Migration `001_initial.sql` enthält bereits `planned_date DATE` und den Index
- SQL-Queries in `recipe_db.rs` selektieren `planned_date` **nicht** (SELECT-Spalten explizit)
- `RecipeFormTemplate` hat kein `planned_date`-Feld
- `RecipeDetailTemplate` hat kein `planned_date`-Feld
- `RecipeListItem` hat kein `planned_date`-Feld
- `time`-Crate ist bereits als Dependency eingebunden (`time = "0.3"`)
- sqlx ist mit dem Feature `time` konfiguriert (kann `time::Date` direkt mappen)

### Keine DB-Migration notwendig

Das Feld `planned_date DATE` existiert bereits in `migrations/001_initial.sql`.
Es ist keine neue Migration nötig.

---

## Technische Schritte

### Schritt 1: Datum-Parsing-Logik in `src/models/recipe.rs`

- [ ] Funktion `parse_german_date(input: &str) -> Result<time::Date, String>` implementieren:
  - Akzeptierte Formate: `T.M.JJJJ`, `TT.MM.JJJJ`, `T.M.JJ`, `TT.MM.JJ`
  - Trennzeichen: Punkt (`.`)
  - Zweistelliges Jahr: `year + 2000` (z.B. `25` → `2025`)
  - Führende Nullen optional: `5.3.2025` und `05.03.2025` sind äquivalent
  - Gibt `Err("Kein gültiges Datum. Bitte im Format T.M.JJJJ eingeben.")` zurück
- [ ] Unit-Tests für `parse_german_date`:
  - Happy Path: `5.3.2025` → `2025-03-05`
  - Happy Path mit Nullen: `05.03.2025` → `2025-03-05`
  - Zweistelliges Jahr: `5.3.25` → `2025-03-05`
  - Ungültige Eingabe `"morgen"` → `Err`
  - Ungültiges Datum `"32.1.2025"` → `Err`
  - Leerer String → `Ok(None)` (im aufrufenden Code behandelt)

### Schritt 2: Rust-Modell erweitern (`src/models/recipe.rs`)

- [ ] `Recipe`-Struct: Feld `planned_date: Option<time::Date>` hinzufügen
- [ ] `CreateRecipe`-Struct: Feld `planned_date_input: Option<String>` hinzufügen (Roheingabe aus Formular)
- [ ] `UpdateRecipe`-Struct: Feld `planned_date_input: Option<String>` hinzufügen (Roheingabe aus Formular)
- [ ] `CreateRecipe::validate()`: Datum parsen und validieren; Fehler zu `errors` hinzufügen
- [ ] `UpdateRecipe::validate()`: identisch wie oben
- [ ] Hilfsmethode `parsed_date(&self) -> Option<time::Date>` an beiden Structs (gibt `None` wenn leer oder ungültig)
- [ ] Unit-Tests:
  - `CreateRecipe` mit gültigem Datum validiert erfolgreich
  - `CreateRecipe` mit ungültigem Datum gibt Fehler
  - `CreateRecipe` ohne Datum (leer) validiert erfolgreich

### Schritt 3: Datenbank-Layer erweitern (`src/models/recipe_db.rs`)

- [ ] `create_recipe`: `planned_date` in INSERT-Query aufnehmen (Binding: `recipe.parsed_date()`)
- [ ] `get_recipe_by_id`: `planned_date` in SELECT-Spalten aufnehmen
- [ ] `get_all_recipes`: `planned_date` in SELECT-Spalten aufnehmen
- [ ] `search_recipes`: `planned_date` in SELECT-Spalten aufnehmen
- [ ] `filter_recipes_by_categories`: `planned_date` in SELECT-Spalten aufnehmen
- [ ] `update_recipe`: `planned_date` im UPDATE-Statement setzen
- [ ] Unit-Tests in `recipe_db.rs`:
  - Rezept mit Datum erstellen und zurücklesen → Datum stimmt überein
  - Rezept ohne Datum erstellen → `planned_date` ist `None`
  - Rezept-Datum über Update ändern → neues Datum wird gespeichert
  - Rezept-Datum über Update löschen (auf `None` setzen)

### Schritt 4: Templates erweitern (`src/templates.rs`)

- [ ] `RecipeFormTemplate`: Feld `planned_date: String` hinzufügen (angezeigtes Datum im deutschen Format oder leer)
- [ ] `RecipeDetailTemplate`: Feld `planned_date: Option<String>` hinzufügen (formatiertes Datum oder `None`)
- [ ] `RecipeListItem`: Feld `planned_date: Option<String>` hinzufügen (kompaktes Datum oder `None`)
- [ ] `RecipeFormTemplate::default()` / `new()`: `planned_date: String::new()` als Default

### Schritt 5: HTML-Templates anpassen

- [ ] `templates/recipes/form.html`: Datumsfeld hinzufügen
  - Label: "Datum (geplant / gekocht)"
  - `<input type="text" name="planned_date" placeholder="T.M.JJJJ" ...>`
  - Verstecktes `<input type="date" id="date-picker">` direkt daneben
  - Kalender-Icon (SVG, z.B. Lucide `calendar`) als klickbares Element
  - Kleines JavaScript-Snippet: Klick auf Icon → `showPicker()` auf Hidden-Input → `change`-Event schreibt in Textfeld
  - Fehlermeldung des Datumfeldes aus `errors`-Array anzeigen (Formular behält Eingabewerte)
  - Barrierefreiheit: `<label for="planned_date">`, `aria-label` am Kalender-Icon, `aria-describedby` für Fehler
- [ ] `templates/recipes/detail.html`: Datum in Detailansicht hinzufügen
  - Nur anzeigen wenn `planned_date` vorhanden
  - Format: `5. März 2025` (langer Monatsname auf Deutsch)
- [ ] `templates/index.html` (oder partials): Datum in Listenzeile anzeigen
  - Nur wenn `planned_date` vorhanden, kompaktes Format: `05.03.2025`

### Schritt 6: Route-Handler anpassen (`src/routes/recipes.rs`)

- [ ] `new_recipe_form`: `planned_date: String::new()` im Template setzen (kein Änderungsbedarf wenn Default verwendet)
- [ ] `create_recipe_handler`:
  - `planned_date`-Formularwert aus `params` extrahieren
  - `CreateRecipe { ..., planned_date_input: planned_date_raw }` befüllen
  - Bei Validierungsfehler: `planned_date`-Eingabewert ins Fehler-Template übernehmen (kein Datenverlust)
- [ ] `edit_recipe_form`:
  - `recipe.planned_date` in deutsches Format umwandeln für Vorausfüllung
  - `RecipeFormTemplate { ..., planned_date: formatted_date }` befüllen
- [ ] `update_recipe_handler`:
  - `planned_date`-Formularwert aus `params` extrahieren
  - `UpdateRecipe { ..., planned_date_input: planned_date_raw }` befüllen
  - Bei Validierungsfehler: `planned_date`-Eingabewert ins Fehler-Template übernehmen
- [ ] `show_recipe`:
  - `recipe.planned_date` zu `Option<String>` formatieren (langer Monatsname)
  - `RecipeDetailTemplate { ..., planned_date: formatted_date }` befüllen
- [ ] `index`-Handler:
  - `recipe.planned_date` zu `Option<String>` formatieren (kompakt)
  - `RecipeListItem { ..., planned_date: formatted_date }` befüllen
- [ ] Hilfsfunktion `format_planned_date_long(date: Option<time::Date>) -> Option<String>` (für Detailansicht)
- [ ] Hilfsfunktion `format_planned_date_short(date: Option<time::Date>) -> Option<String>` (für Listenansicht)

### Schritt 7: CSS-Styling (`src/static/css/app.css`)

- [ ] Stil für das Datums-Eingabefeld mit Kalender-Icon (Flex-Layout: Textfeld + Icon nebeneinander)
- [ ] Kalender-Icon dezent und klickbar
- [ ] Responsives Layout (mobile-first)

### Schritt 8: Rust-Integrationstests (`tests/`)

- [ ] `tests/recipe_date.rs` erstellen mit folgenden Tests:
  - **Given/When/Then** als deutsche Kommentare in jedem Test
  - Rezept mit gültigem Datum `5.3.2025` erstellen → Redirect auf Detailseite
  - Rezept-Detailseite zeigt Datum `05.03.2025` an (HTTP-Response)
  - POST mit ungültigem Datum `"morgen"` → HTTP 400, Fehlermeldung im Body
  - POST mit leerem Datum → Rezept wird ohne Datum gespeichert (HTTP 303)
  - Rezept mit Datum über Update löschen → Datum nicht mehr angezeigt

### Schritt 9: E2E-Tests (`tests/e2e/`)

- [ ] `tests/e2e/recipe-date.spec.ts` erstellen:
  - **Given/When/Then** als deutsche Kommentare in jedem Test
  - **Testfall K1/K3:** Datum beim Erstellen eingeben (`5.3.2025`) → Detailseite zeigt `05.03.2025`
  - **Testfall K2:** Datum beim Bearbeiten vorausfüllt anzeigen → Datum ändern → gespeichert
  - **Testfall K3 (Löschen):** Datum leeren → Datum nicht mehr angezeigt
  - **Testfall K5:** Ungültiges Datum `"morgen"` → Fehlermeldung, andere Felder behalten Werte
  - **Testfall K4:** Kalender-Icon vorhanden, Klick öffnet Picker (sofern Browser unterstützt)
  - **Testfall K6:** Datum in Detailansicht sichtbar
  - **Testfall K7:** Datum in Listenansicht sichtbar
- [ ] Seeds erweitern oder neue Seed-Datei `tests/seeds/recipe-date.sql` erstellen

### Schritt 10: DoD-Abschluss-Prüfung

- [ ] `cargo build` → keine Fehler, keine Warnungen
- [ ] `cargo clippy -- -D warnings` → sauber
- [ ] `cargo fmt --check` → sauber
- [ ] `cargo test` → alle Tests grün
- [ ] `npm run test:e2e` → alle E2E-Tests grün

---

## URL-Struktur

Keine neuen URLs. Bestehende URLs werden erweitert:

```
GET  /recipes/new         → Formular enthält jetzt Datumsfeld
POST /recipes             → Datum wird verarbeitet und gespeichert
GET  /recipes/{id}        → Detailansicht zeigt Datum an
GET  /recipes/{id}/edit   → Formular vorausgefüllt mit gespeichertem Datum
POST /recipes/{id}        → Datum-Update verarbeitet
GET  /                    → Listenansicht zeigt Datum pro Rezept an
```

---

## Abhängigkeiten

- Story 01 (Rezept erstellen) ist abgeschlossen
- Story 02 (Rezept bearbeiten) ist abgeschlossen
- Story 04 (Rezept-Detailansicht) ist abgeschlossen
- `time`-Crate bereits in `Cargo.toml` eingebunden (`time = "0.3"`)
- sqlx mit `time`-Feature bereits konfiguriert → `time::Date` wird direkt aus SQLite gemappt
- DB-Schema enthält `planned_date DATE` bereits in `001_initial.sql`

---

## Test-Checkliste

- [ ] Unit-Test: `parse_german_date` - gültiges Format `T.M.JJJJ`
- [ ] Unit-Test: `parse_german_date` - gültiges Format mit führenden Nullen `TT.MM.JJJJ`
- [ ] Unit-Test: `parse_german_date` - zweistelliges Jahr `T.M.JJ`
- [ ] Unit-Test: `parse_german_date` - ungültige Eingabe `"morgen"` → Fehler
- [ ] Unit-Test: `parse_german_date` - ungültiges Datum `"32.1.2025"` → Fehler
- [ ] Unit-Test: `CreateRecipe::validate()` mit gültigem Datum → OK
- [ ] Unit-Test: `CreateRecipe::validate()` mit ungültigem Datum → Fehler
- [ ] Unit-Test: `CreateRecipe::validate()` ohne Datum → OK
- [ ] Unit-Test: `recipe_db` - Rezept mit Datum erstellen und zurücklesen
- [ ] Unit-Test: `recipe_db` - Rezept-Datum über Update ändern/löschen
- [ ] Integrationstest: POST /recipes mit Datum → 303, Datum in Detailseite
- [ ] Integrationstest: POST /recipes mit ungültigem Datum → 400, Fehlermeldung
- [ ] Integrationstest: POST /recipes/{id} Datum löschen → Datum nicht mehr sichtbar
- [ ] E2E-Test: Datum eingeben beim Erstellen (K1, K3)
- [ ] E2E-Test: Datum beim Bearbeiten vorausfüllt und änderbar (K2)
- [ ] E2E-Test: Datum löschen (K3)
- [ ] E2E-Test: Ungültiges Datum → Fehlermeldung, kein Datenverlust (K5)
- [ ] E2E-Test: Kalender-Icon vorhanden (K4)
- [ ] E2E-Test: Datum in Detailansicht (K6)
- [ ] E2E-Test: Datum in Listenansicht (K7)
- [ ] Manueller Test: Date-Picker auf Chrome, Firefox, Safari (mobil)
- [ ] Manueller Test: Formular funktioniert ohne JavaScript

---

## Offene Punkte

- Deutschen langen Monatsnamen für Detailansicht: `time`-Crate unterstützt keine deutsche Lokalisierung out-of-the-box → eigene Mapping-Funktion oder statisches Array `["Januar", "Februar", ..., "Dezember"]` verwenden
- Progressive Enhancement ohne JS: Das `<input type="date">` kann als Fallback sichtbar sein wenn JS deaktiviert → Spezifikation sagt "als Fallback sichtbar" (story.md, Abschnitt 4 UI/UX)
