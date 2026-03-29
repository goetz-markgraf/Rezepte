# Implementierungsplan: Story 10 - Filter "Nächste 7 Tage"

## Kontext & Analyse

### Ausgangslage

Story 9 ("Länger nicht gemacht") ist vollständig implementiert und dient als direktes Vorbild. Die
relevanten Muster sind bereits etabliert:

- `filter_recipes_not_made_recently` in `src/models/recipe_db.rs` zeigt das Muster für einen
  datumbasierten Filter mit dynamischen SQL-Klauseln
- `IndexQuery.filter` in `src/routes/recipes.rs` nimmt den Filter-Parameter entgegen
- `IndexTemplate.not_made_filter_active` / `not_made_filter_toggle_url` zeigen das Template-Muster
- `build_not_made_toggle_url` und `build_category_toggle_url` zeigen URL-Aufbaumuster
- `.sort-filter-btn` in `app.css` und der zugehörige `<div class="sort-filter">` in `index.html`
  sind bereits für Story 9 vorhanden — der neue Filter wird in denselben Bereich eingebettet

### Entscheidungen zu offenen Punkten aus der Story

**Datumsformat auf der Karte (K7):** Das bestehende kompakte Format `"DD.MM.JJJJ"` wird
beibehalten (wie für Story 9 bereits via `format_planned_date_short` implementiert). Bei aktivem
"Nächste 7 Tage"-Filter soll zusätzlich der Wochentag vor dem Datum angezeigt werden
(z.B. "Mo, 31.03.2026"), weil das den Hauptnutzwert dieses Filters ausmacht.

**Zeitraum:** Exakt "nächste 7 Tage" (heute bis heute+7, beide Grenzen inklusive), nicht
"aktuelle Woche". Die Story ist hier eindeutig (Abschnitt 4, Edge Cases).

**URL-Parameter:** `?filter=naechste-7-tage` (konsistent mit `laenger-nicht-gemacht` —
Umlaute entfernt, Leerzeichen als Bindestriche). Die Architektur-Doku nennt `planned=next-7-days`,
aber Story-Text hat Vorrang (wie bei Story 9 entschieden).

### Betroffene Dateien

| Datei | Art der Änderung |
|-------|-----------------|
| `src/models/recipe_db.rs` | Neue Funktion `filter_recipes_next_seven_days` |
| `src/models/mod.rs` | Re-Export der neuen Funktion |
| `src/routes/recipes.rs` | Handler erweitern, neue Toggle-URL-Funktion, `IndexTemplate` erweitern |
| `src/templates.rs` | `IndexTemplate` um zwei neue Felder erweitern |
| `templates/index.html` | Neuer Filter-Button, Keine-Treffer-Meldungen, Datum mit Wochentag |
| `src/static/css/app.css` | Keine Änderung nötig (`.sort-filter-btn` ist bereits vorhanden) |
| `tests/recipe_next_seven_days_filter.rs` | Neue Integrationstests |
| `tests/e2e/recipe-next-seven-days-filter.spec.ts` | Neue E2E-Tests |

---

## Technische Schritte

### Schritt 1: Datenbank-Layer — Query "Nächste 7 Tage"

**Neue Funktion `filter_recipes_next_seven_days` in `src/models/recipe_db.rs`:**

```rust
/// Filtert Rezepte nach dem Prinzip "Nächste 7 Tage":
/// - Nur Rezepte mit `planned_date` zwischen heute (inklusive) und heute + 7 Tage (inklusive)
/// - Rezepte ohne Datum, mit Vergangenheitsdatum oder > heute+7 werden ausgeschlossen
/// - Sortierung: chronologisch aufsteigend nach Datum, bei gleichem Datum alphabetisch (deutsche Sortierung)
///
/// Optional kombinierbar mit Kategorie-Filter (ODER-Logik) und Volltextsuche (UND-Logik).
pub async fn filter_recipes_next_seven_days(
    pool: &SqlitePool,
    categories: &[String],
    search_query: &str,
) -> Result<Vec<Recipe>, sqlx::Error>
```

**SQL-Logik:**
```sql
WHERE planned_date >= DATE('now')
  AND planned_date <= DATE('now', '+7 days')
  [AND (LOWER(categories) LIKE ? OR ...)]  -- wenn Kategorien gesetzt
  [AND (LOWER(title) LIKE ? OR LOWER(ingredients) LIKE ? OR LOWER(instructions) LIKE ?)]  -- wenn Suche gesetzt
ORDER BY planned_date ASC
```

Die Sekundärsortierung (gleiche Daten alphabetisch) erfolgt wie bei Story 9 in Rust via `sort_by`
mit `normalize_for_sort`, weil SQLite keine native Umlaut-Sortierung kennt.

**Aufgaben:**

- [ ] Funktion `filter_recipes_next_seven_days` in `src/models/recipe_db.rs` erstellen
  - Parameter: `pool: &SqlitePool`, `categories: &[String]`, `search_query: &str`
  - Rückgabe: `Result<Vec<Recipe>, sqlx::Error>`
  - WHERE-Bedingung: `planned_date >= DATE('now') AND planned_date <= DATE('now', '+7 days')`
  - Kategorie-Klausel dynamisch aufbauen (exakt wie in `filter_recipes_not_made_recently`)
  - Such-Klausel dynamisch aufbauen (exakt wie in `filter_recipes_not_made_recently`)
  - ORDER BY in SQL: `ORDER BY planned_date ASC`
  - Sekundärsortierung in Rust: `sort_by` mit `normalize_for_sort` bei gleichem Datum
  - Doc-Kommentar (`///`) vollständig schreiben

- [ ] Unit-Tests in `src/models/recipe_db.rs` (im `#[cfg(test)] mod tests`-Block):

  - `next_seven_days_returns_recipes_within_window`
    — Rezept mit `planned_date` morgen erscheint; Rezept mit `planned_date` in 8 Tagen nicht
  - `next_seven_days_includes_today`
    — Rezept mit `planned_date` heute wird angezeigt
  - `next_seven_days_includes_day_seven`
    — Rezept mit `planned_date` in genau 7 Tagen wird angezeigt
  - `next_seven_days_excludes_past_dates`
    — Rezept mit `planned_date` gestern wird nicht angezeigt
  - `next_seven_days_excludes_null_dates`
    — Rezept ohne `planned_date` wird nicht angezeigt
  - `next_seven_days_excludes_day_eight`
    — Rezept mit `planned_date` in 8 Tagen wird nicht angezeigt
  - `next_seven_days_sorted_chronologically`
    — Rezept in 2 Tagen erscheint vor Rezept in 5 Tagen
  - `next_seven_days_same_date_sorted_alphabetically`
    — Zwei Rezepte am gleichen Tag werden alphabetisch sortiert (Umlaut-Test: Äpfel vor Bananen)
  - `next_seven_days_combined_with_category_filter`
    — Kategorie-Filter schränkt Ergebnisse ein; Rezept in anderen Kategorie wird ausgeblendet
  - `next_seven_days_combined_with_search_query`
    — Suchbegriff schränkt Ergebnisse ein; nicht-matchendes Rezept wird ausgeblendet
  - `next_seven_days_returns_empty_when_no_recipes_in_window`
    — Leere Liste wenn alle Rezepte außerhalb des Fensters liegen

  **Test-Hilfsfunktion:** `make_recipe_with_planned_date(title, category, date_str)` analog zur
  bestehenden `make_recipe`-Funktion ergänzen. Da `DATE('now', '+N days')` nicht direkt im Test
  nutzbar ist (Zeitabhängigkeit), wird ein absolutes Datum aus dem Jahr 2099 für "Zukunft außerhalb"
  und ein relatives Datum berechnet:

  ```rust
  fn make_future_date(days_from_now: i64) -> String {
      let today = time::OffsetDateTime::now_utc().date();
      let target = today + time::Duration::days(days_from_now);
      format!("{}.{}.{}", target.day(), target.month() as u8, target.year())
  }
  ```

---

### Schritt 2: Modell-Layer — Re-Export und `IndexQuery` erweitern

- [ ] `filter_recipes_next_seven_days` in `src/models/mod.rs` re-exportieren:
  ```rust
  pub use recipe_db::{
      create_recipe, delete_recipe, filter_recipes_by_categories,
      filter_recipes_next_seven_days, filter_recipes_not_made_recently,
      get_recipe_by_id, update_recipe,
  };
  ```

- [ ] `IndexQuery` in `src/routes/recipes.rs` ist bereits korrekt (`filter: Option<String>`) —
  **keine Änderung notwendig**. Der neue Filter nutzt denselben `filter`-Parameter mit dem
  Wert `"naechste-7-tage"`.

- [ ] `IndexTemplate` in `src/templates.rs` um zwei neue Felder erweitern:
  ```rust
  pub struct IndexTemplate {
      // ... bestehende Felder ...
      pub not_made_filter_active: bool,
      pub not_made_filter_toggle_url: String,
      // neu:
      pub next_seven_days_filter_active: bool,
      pub next_seven_days_filter_toggle_url: String,
  }
  ```

---

### Schritt 3: Route/Handler — Filter-Logik einbauen

**Neue Hilfsfunktion `build_next_seven_days_toggle_url` in `src/routes/recipes.rs`:**

```rust
/// Baut die Toggle-URL für den "Nächste 7 Tage"-Filter.
/// Aktiv → URL ohne `filter`-Parameter (Kategorie + Suche bleiben erhalten).
/// Inaktiv → URL mit `filter=naechste-7-tage` (Kategorie + Suche bleiben erhalten).
fn build_next_seven_days_toggle_url(
    is_active: bool,
    active_categories: &[String],
    search_query: &str,
) -> String
```

Die Funktion hat dieselbe Signatur wie `build_not_made_toggle_url` und setzt nur den anderen
Filter-Wert.

**Anpassungen am `index`-Handler:**

- [ ] Neuen Hilfsfunktion `build_next_seven_days_toggle_url` in `src/routes/recipes.rs` erstellen
  - Analog zu `build_not_made_toggle_url`, aber mit `filter=naechste-7-tage`

- [ ] `index`-Handler in `src/routes/recipes.rs` anpassen:
  - `next_seven_days_filter_active = query.filter.as_deref() == Some("naechste-7-tage")`
  - Wenn aktiv: `filter_recipes_next_seven_days(&pool, &active_categories, &search_query).await?`
  - Die drei Filter sind **gegenseitig exklusiv**: `not_made_filter_active` und
    `next_seven_days_filter_active` können nicht gleichzeitig true sein (da nur ein `filter`-Wert
    gesetzt werden kann) — keine zusätzliche Prüfung nötig
  - `next_seven_days_filter_toggle_url` berechnen

- [ ] `build_category_toggle_url` um `next_seven_days_filter_active: bool` erweitern:
  - Wenn aktiv: `filter=naechste-7-tage` in URL aufnehmen
  - Analog zur bestehenden `not_made_filter_active`-Behandlung

- [ ] `build_category_filters` um `next_seven_days_filter_active: bool` erweitern:
  - Parameter an `build_category_toggle_url` durchreichen

- [ ] `build_reset_url` um `next_seven_days_filter_active: bool` erweitern:
  - Wenn aktiv: `filter=naechste-7-tage` in der Reset-URL beibehalten

- [ ] Doc-Kommentar des `index`-Handlers um neuen Filter ergänzen

- [ ] `IndexTemplate` im Handler mit beiden neuen Feldern befüllen

**Achtung bei der Filter-Logik:** Die `build_category_toggle_url`-Funktion trägt den aktiven
`not_made`-Filter bereits weiter. Beim Erweitern um `next_seven_days` muss sichergestellt werden,
dass stets nur maximal ein Filter-Wert in der URL steht — d.h. beide Flags können nicht
gleichzeitig true sein (ist bereits durch den Query-Parameter-Mechanismus garantiert).

---

### Schritt 4: Template — Filter-UI einbauen

**Änderungen in `templates/index.html`:**

- [ ] Neuen Filter-Button "Nächste 7 Tage" im `<div class="sort-filter">` ergänzen (neben
  "Länger nicht gemacht"):
  ```html
  <a
      href="{{ next_seven_days_filter_toggle_url }}"
      class="sort-filter-btn{% if next_seven_days_filter_active %} active{% endif %}"
      aria-pressed="{{ next_seven_days_filter_active }}"
      hx-get="{{ next_seven_days_filter_toggle_url }}"
      hx-target="#recipe-results"
      hx-push-url="true"
      hx-select="#recipe-results"
  >Nächste 7 Tage</a>
  ```

- [ ] Suchformular: `filter=naechste-7-tage` als Hidden-Input hinzufügen wenn aktiv:
  ```html
  {% if next_seven_days_filter_active %}
  <input type="hidden" name="filter" value="naechste-7-tage">
  {% endif %}
  ```

- [ ] `"Alle"`-Button: `aria-pressed` anpassen:
  - Aktiv wenn: `active_categories.is_empty() && !not_made_filter_active && !next_seven_days_filter_active`
  - Analog für `class="category-filter-btn"` und `{% if ... %} active{% endif %}`

- [ ] Neue Keine-Treffer-Meldungen für `next_seven_days_filter_active` ergänzen:
  - Alle vier Kombinationen (wie für `not_made_filter_active` vorhanden):
    - Nur Filter aktiv: `"Keine Rezepte für die nächsten 7 Tage geplant."`
    - Filter + Suche: `"Keine Rezepte für '{{ search_query }}' in den nächsten 7 Tagen gefunden."`
    - Filter + Kategorie: `"Keine Rezepte in dieser Kategorie für die nächsten 7 Tage geplant."`
    - Filter + Suche + Kategorie: `"Keine Rezepte für '{{ search_query }}' in den gewählten Kategorien für die nächsten 7 Tage gefunden."`

- [ ] Datum mit Wochentag auf Rezeptkarte bei aktivem `next_seven_days_filter_active`:
  - Das Template bekommt entweder einen aufbereiteten String oder eine neue `planned_date_with_weekday`-Formatfunktion
  - Einfachste Lösung: In `RecipeListItem` ein neues Feld `planned_date_with_weekday: Option<String>` ergänzen und im Handler befüllen — aber das verändert das bestehende Modell für alle Kontexte.
  - **Bessere Lösung:** Neues Feld `planned_date_display: Option<String>` in `RecipeListItem` einführen, das je nach aktivem Filter ein anderes Format enthält. Beim normalen Filter wird `format_planned_date_short` verwendet, beim 7-Tage-Filter `format_planned_date_with_weekday`.
  - Alternativ: Im Template prüfen — aber `RecipeListItem` enthält schon nur einen vorformatierten String.
  - **Entscheidung:** Neues Feld `planned_date_weekday: Option<String>` in `RecipeListItem` ergänzen (enthält "Mo, 31.03.2026" oder None). Im Handler nur bei `next_seven_days_filter_active` befüllen; im Template wird `planned_date_weekday` angezeigt wenn vorhanden, sonst `planned_date`.

- [ ] Neue Formatfunktion `format_planned_date_with_weekday` in `src/routes/recipes.rs`:
  ```rust
  const GERMAN_WEEKDAYS: &[&str] = &["Mo", "Di", "Mi", "Do", "Fr", "Sa", "So"];

  fn format_planned_date_with_weekday(date: Option<time::Date>) -> Option<String> {
      date.map(|d| {
          let weekday_idx = d.weekday().number_days_from_monday() as usize;
          let weekday = GERMAN_WEEKDAYS[weekday_idx];
          format!("{}, {:02}.{:02}.{}", weekday, d.day(), d.month() as u8, d.year())
      })
  }
  ```

- [ ] `RecipeListItem` in `src/templates.rs` um `planned_date_weekday: Option<String>` ergänzen

- [ ] Im `index`-Handler: bei `next_seven_days_filter_active` wird `planned_date_weekday` befüllt,
  sonst bleibt es `None`

- [ ] In `templates/index.html`: Anzeige-Logik anpassen:
  ```html
  {% if let Some(date) = recipe.planned_date_weekday %}
  <span class="recipe-date recipe-date-weekday">{{ date }}</span>
  {% else if let Some(date) = recipe.planned_date %}
  <span class="recipe-date">{{ date }}</span>
  {% endif %}
  ```

---

### Schritt 5: CSS — Styling

- [ ] Prüfen ob `.sort-filter` bereits mehrere Buttons nebeneinander unterstützt (aktuell: ein
  Button). Falls nötig: `display: flex; flex-wrap: wrap; gap: 0.5rem;` für `.sort-filter`-Container
  ergänzen.

- [ ] Neue CSS-Klasse `.recipe-date-weekday` für die Wochentag-Anzeige (optional: leicht
  hervorgehoben, da die Info besonders relevant ist):
  ```css
  .recipe-date-weekday {
      font-weight: 500;
  }
  ```

---

### Schritt 6: Integrationstests (Rust)

- [ ] Neue Testdatei `tests/recipe_next_seven_days_filter.rs` erstellen.

  Hilfsfunktionen analog zu `tests/recipe_not_made_filter.rs`:
  - `setup_test_app()` — gleich
  - `get_body(app, uri)` — gleich
  - `create_recipe_with_date(app, title, categories, planned_date)` — gleich

  **Tests:**

  - `next_seven_days_filter_returns_200_with_recipes_in_window`
    — Rezept mit Datum morgen ist im Body; HTTP 200
  - `next_seven_days_filter_excludes_past_dates`
    — Rezept mit Vergangenheitsdatum nicht im Body
  - `next_seven_days_filter_excludes_null_dates`
    — Rezept ohne planned_date nicht im Body
  - `next_seven_days_filter_excludes_dates_beyond_seven_days`
    — Rezept mit Datum in 8 Tagen nicht im Body
  - `next_seven_days_filter_includes_today`
    — Rezept mit heutigem Datum im Body
  - `next_seven_days_filter_shows_empty_state_message`
    — Keine Rezepte im Fenster → Hinweistext "Nächsten 7 Tage" im Body
  - `next_seven_days_filter_combined_with_category`
    — `?filter=naechste-7-tage&kategorie=Brot` zeigt nur Brot-Rezepte in Fenster
  - `next_seven_days_filter_combined_with_search`
    — `?filter=naechste-7-tage&q=suche` zeigt nur Suchergebnisse in Fenster
  - `deeplink_next_seven_days_filter_returns_correct_state`
    — URL `/?filter=naechste-7-tage` gibt 200 zurück, Filter-Button als aktiv markiert
    (`aria-pressed="true"`)
  - `no_filter_param_returns_alphabetical_list`
    — Ohne `filter`-Parameter: keine Filterung (Referenztest, um Regression zu erkennen)

  Jeden Test mit `// Given / // When / // Then`-Kommentaren.

  **Anmerkung:** Da "heute" zur Testlaufzeit berechnet wird, werden Testdaten mit relativen Daten
  über die Helper-Funktion erzeugt (z.B. `planned_date: Some("morgen")` nicht möglich in SQL,
  daher im Rust-Test via `time`-Crate berechnet):
  ```rust
  fn date_in_days(n: i64) -> String {
      let d = time::OffsetDateTime::now_utc().date() + time::Duration::days(n);
      format!("{}.{}.{}", d.day(), d.month() as u8, d.year())
  }
  ```

---

### Schritt 7: E2E-Tests (Playwright)

- [ ] Neue Testdatei `tests/e2e/recipe-next-seven-days-filter.spec.ts` erstellen.

  **Hilfsfunktionen:**
  - `createRecipeWithDate` aus `recipe-not-made-filter.spec.ts` wiederverwenden (kopieren
    oder importieren)
  - `futureDateInDays(n: number): string` — berechnet "heute + n Tage" als deutschen Datumsstring:
    ```typescript
    function futureDateInDays(days: number): string {
        const d = new Date();
        d.setDate(d.getDate() + days);
        return `${d.getDate()}.${d.getMonth() + 1}.${d.getFullYear()}`;
    }
    ```

  **Tests (entsprechen K1–K9 aus der Story):**

  - **K1: Filter-Button sichtbar und aktivierbar**
    ```
    // Given: Startseite aufgerufen
    // Then: Button "Nächste 7 Tage" ist sichtbar, aria-pressed="false"
    // When: Button geklickt
    // Then: aria-pressed="true", class enthält "active", URL enthält filter=naechste-7-tage
    ```

  - **K2: Nur Rezepte im Zeitfenster** (+ K5 Keine Treffer kombiniert)
    ```
    // Given: "Spaghetti" mit Datum übermorgen, "Pizza" mit Datum in 5 Tagen,
    //        "Linseneintopf" ohne Datum
    // When: Filter "Nächste 7 Tage" aktiviert
    // Then: "Spaghetti" und "Pizza" sichtbar, "Linseneintopf" nicht sichtbar
    ```

  - **K3: Chronologische Sortierung**
    ```
    // Given: "Pizza" mit Datum in 5 Tagen, "Spaghetti" mit Datum übermorgen
    // When: Filter aktiviert
    // Then: "Spaghetti" erscheint vor "Pizza" (früheres Datum zuerst)
    ```

  - **K3: Zeitfenster-Grenzen**
    ```
    // Given: "Heute-Rezept" mit heute, "Tag-7-Rezept" mit heute+7,
    //        "Tag-8-Rezept" mit heute+8, "Vergangen" mit gestern
    // When: Filter aktiviert
    // Then: "Heute-Rezept" und "Tag-7-Rezept" sichtbar, andere nicht
    ```

  - **K4: Filter zurücksetzen**
    ```
    // Given: Filter ist aktiv (direkte URL /?filter=naechste-7-tage)
    // When: Filter-Button erneut geklickt
    // Then: aria-pressed="false", keine filter-URL, alle Rezepte alphabetisch
    ```

  - **K5: Keine Treffer → Hinweistext**
    ```
    // Given: Alle Rezepte außerhalb des 7-Tage-Fensters
    // When: Filter aktiviert
    // Then: Hinweistext "Keine Rezepte für die nächsten 7 Tage geplant" sichtbar,
    //       keine .recipe-item vorhanden
    ```

  - **K6: DeepLink**
    ```
    // Given: Rezept mit Datum in 2 Tagen vorhanden
    // When: URL /?filter=naechste-7-tage direkt aufgerufen
    // Then: Filter-Button aktiv (aria-pressed="true"), Rezept sichtbar
    ```

  - **K7: Datum mit Wochentag auf Karte**
    ```
    // Given: Rezept mit Datum übermorgen
    // When: Filter "Nächste 7 Tage" aktiv
    // Then: Rezeptkarte zeigt Wochentag + Datum (z.B. "Di, 31.03.2026")
    ```

  - **K8: Kombination mit Kategorie-Filter**
    ```
    // Given: "Dinkelbrot" (Brot, in 2 Tagen), "Spaghetti" (Mittagessen, in 3 Tagen)
    // When: Kategorie "Brot" + Filter "Nächste 7 Tage"
    // Then: Nur "Dinkelbrot" sichtbar, "Spaghetti" nicht sichtbar
    //       URL enthält kategorie=Brot und filter=naechste-7-tage
    ```

  - **K9: Kombination mit Volltextsuche**
    ```
    // Given: "Dinkelbrot" (in 2 Tagen) und "Spaghetti" (in 3 Tagen)
    // When: Suche nach "Dinkel" + Filter "Nächste 7 Tage"
    // Then: Nur "Dinkelbrot" sichtbar, "Spaghetti" nicht sichtbar
    ```

  Jeden Test mit deutschen Given/When/Then-Kommentaren.

---

### Schritt 8: Qualitätschecks (DoD)

- [ ] `cargo fmt` — Code formatieren
- [ ] `cargo clippy -- -D warnings` — keine Warnungen
- [ ] `cargo build` — keine Compilerfehler
- [ ] `cargo test` — alle Unit- und Integrationstests grün
- [ ] `npm run test:e2e` — alle E2E-Tests grün
- [ ] Öffentliche Funktion `filter_recipes_next_seven_days` in `recipe_db.rs` mit vollständigem
  Doc-Kommentar (`///`)
- [ ] `index`-Handler Doc-Kommentar aktualisieren
- [ ] `architecture.md` URL-Tabelle prüfen: `?filter=naechste-7-tage` eintragen wenn nicht bereits
  korrekt vorhanden

---

## URL-Struktur

```
GET  /                                                         → Alle Rezepte (alphabetisch)
GET  /?filter=naechste-7-tage                                  → Nur Rezepte der nächsten 7 Tage, chronologisch
GET  /?filter=naechste-7-tage&kategorie=Brot                   → Brot-Rezepte in den nächsten 7 Tagen
GET  /?filter=naechste-7-tage&q=pasta                         → Suchergebnisse in den nächsten 7 Tagen
GET  /?filter=naechste-7-tage&kategorie=Brot&q=dinkel          → Alle drei Filter kombiniert
```

---

## Abhängigkeiten

- Story 5 (alphabetische Sortierung) ist implementiert — `normalize_for_sort` wird für
  Sekundärsortierung wiederverwendet
- Story 8 (Kategorie-Filter) ist implementiert — `filter_recipes_by_categories`,
  `build_category_filters`, `extract_kategorie_params` werden wiederverwendet und erweitert
- Story 9 ("Länger nicht gemacht") ist implementiert — Muster für `filter_recipes_not_made_recently`,
  `build_not_made_toggle_url`, Template-Felder und CSS-Klassen werden direkt als Vorlage genutzt
- Kein neues Datenbankfeld nötig — `planned_date` existiert bereits
- Keine neue Migration nötig — Index `idx_recipes_planned_date` existiert bereits
- `urlencoding`-Crate ist bereits als Abhängigkeit vorhanden
- `time`-Crate ist bereits als Abhängigkeit vorhanden (für `OffsetDateTime::now_utc().date()` in Tests)

---

## Test-Checkliste

### Unit-Tests (`src/models/recipe_db.rs`)

- [ ] `next_seven_days_returns_recipes_within_window`
- [ ] `next_seven_days_includes_today`
- [ ] `next_seven_days_includes_day_seven`
- [ ] `next_seven_days_excludes_past_dates`
- [ ] `next_seven_days_excludes_null_dates`
- [ ] `next_seven_days_excludes_day_eight`
- [ ] `next_seven_days_sorted_chronologically`
- [ ] `next_seven_days_same_date_sorted_alphabetically`
- [ ] `next_seven_days_combined_with_category_filter`
- [ ] `next_seven_days_combined_with_search_query`
- [ ] `next_seven_days_returns_empty_when_no_recipes_in_window`

### Integrationstests (`tests/recipe_next_seven_days_filter.rs`)

- [ ] `next_seven_days_filter_returns_200_with_recipes_in_window`
- [ ] `next_seven_days_filter_excludes_past_dates`
- [ ] `next_seven_days_filter_excludes_null_dates`
- [ ] `next_seven_days_filter_excludes_dates_beyond_seven_days`
- [ ] `next_seven_days_filter_includes_today`
- [ ] `next_seven_days_filter_shows_empty_state_message`
- [ ] `next_seven_days_filter_combined_with_category`
- [ ] `next_seven_days_filter_combined_with_search`
- [ ] `deeplink_next_seven_days_filter_returns_correct_state`
- [ ] `no_filter_param_returns_alphabetical_list`

### E2E-Tests (`tests/e2e/recipe-next-seven-days-filter.spec.ts`)

- [ ] K1 — Filter-Button sichtbar und aktivierbar, aria-pressed korrekt
- [ ] K2 — Nur Rezepte im Zeitfenster (Linseneintopf ohne Datum ausgeblendet)
- [ ] K3 — Chronologische Sortierung (früheres Datum zuerst)
- [ ] K3 — Zeitfenster-Grenzen (heute inklusive, Tag 7 inklusive, Tag 8 und gestern exklusiv)
- [ ] K4 — Filter zurücksetzen via Toggle
- [ ] K5 — Keine Treffer: Hinweistext erscheint, keine .recipe-item
- [ ] K6 — DeepLink `/?filter=naechste-7-tage` zeigt korrekten Status
- [ ] K7 — Datum mit Wochentag auf Karte (bei aktivem Filter)
- [ ] K8 — Kombination mit Kategorie-Filter (beide Parameter in URL, nur Kategorie-Matches)
- [ ] K9 — Kombination mit Volltextsuche

### Manuelle Tests

- [ ] Tastatur-Navigation: Tab-Fokus auf "Nächste 7 Tage"-Button, Enter aktiviert Filter
- [ ] `aria-pressed` korrekt (aktiv: `"true"`, inaktiv: `"false"`)
- [ ] Beide Filter-Buttons ("Länger nicht gemacht" und "Nächste 7 Tage") sind gleichzeitig
  sichtbar und korrekt dargestellt
- [ ] Bei aktivem "Nächste 7 Tage"-Filter ist "Alle"-Button nicht aktiv
- [ ] Wechsel zwischen beiden Sonder-Filtern funktioniert korrekt (einer aktiv → anderer inaktiv)
- [ ] Filter-Zustand bleibt nach Seitenneuladung via URL erhalten

---

## Offene Punkte

- **Gegenseitige Exklusivität der Filter:** "Länger nicht gemacht" und "Nächste 7 Tage" sind
  inhaltlich entgegengesetzt und sollten nie gleichzeitig aktiv sein. Da beide über `?filter=`
  abgebildet werden (nur ein Wert möglich), ist die Exklusivität durch die URL-Struktur
  automatisch gewährleistet — keine zusätzliche Prüfung nötig. Der UI muss nur sicherstellen,
  dass beim Aktivieren von Filter A die Toggle-URL von Filter B den `not_made`-Filter nicht
  durchreicht (und umgekehrt). Dies ist durch die getrennte `build_..._toggle_url`-Logik
  sichergestellt.

- **`planned_date_weekday` im Template:** Wenn das Datum mit Wochentag nur beim "Nächste 7 Tage"-
  Filter angezeigt werden soll, braucht `RecipeListItem` ein zusätzliches Feld. Dies ist eine
  bewusste kleine Erweiterung des Datenmodells; alternativ könnte das Datum immer mit Wochentag
  angezeigt werden — aber das ist laut Story nicht gefordert und würde den Normalzustand unruhiger
  machen.
