# Implementierungsplan: Story 13 — Gespeicherte Filter

## Übersicht

Gespeicherte Filter sind benannte Snapshots des aktuellen URL-Filterzustands (`query_string`),
die server-seitig in SQLite persistiert und geräteübergreifend im LAN abrufbar sind.
Die Implementierung folgt dem bestehenden Architektur-Pattern: Server-Side Rendering,
HTMX für partielle Updates, Form-Posts mit Redirects als Progressive-Enhancement-Fallback.

---

## Technische Analyse der bestehenden Codebasis

### Relevante Dateien

| Datei | Rolle |
|-------|-------|
| `src/models/recipe_db.rs` | DB-Queries (Vorbild für neue `saved_filter_db.rs`) |
| `src/models/recipe.rs` | Modell-Structs (Vorbild für `saved_filter.rs`) |
| `src/models/mod.rs` | Re-Exports der Models |
| `src/routes/recipes.rs` | Handler für `/` (muss `saved_filters` einbinden) |
| `src/routes/mod.rs` | Router — neue Routen hinzufügen |
| `src/templates.rs` | Template-Structs (`IndexTemplate` erweitern) |
| `templates/index.html` | Haupt-UI — Filterbereich erweitern |
| `src/static/css/app.css` | CSS-Klassen (z.B. `.sort-filter-btn`, `.reset-all-filters-btn`) |
| `migrations/001_initial.sql` | Vorbild-Migration |

### Bestehende URL-Parameter (vollständige Filterbasis aus Story 12)

```
/?q=&kategorie=Brot&kategorie=Kuchen&filter=laenger-nicht-gemacht&bewertung=gut
```

Der `query_string` in der DB speichert genau diesen Teil (ohne führendes `?`).

### Existierendes `IndexTemplate` (relevant für Erweiterung)

Das Template enthält bereits `any_filter_active: bool`. Daran koppeln wir die Sichtbarkeit
des "Filter speichern"-Elements.

---

## Technische Schritte

### Schritt 1: Datenbank-Migration

- [ ] `migrations/002_saved_filters.sql` erstellen:
  ```sql
  CREATE TABLE saved_filters (
      id INTEGER PRIMARY KEY AUTOINCREMENT,
      name TEXT NOT NULL UNIQUE,
      query_string TEXT NOT NULL,
      created_at TEXT NOT NULL DEFAULT (datetime('now'))
  );
  CREATE INDEX idx_saved_filters_name ON saved_filters(name);
  ```
  Begründung: Der `UNIQUE`-Constraint auf `name` erzwingt Eindeutigkeit direkt auf DB-Ebene
  (ergibt SQLite-Fehler bei Duplikaten, den wir im Handler abfangen).
  Der Index auf `name` ermöglicht schnelle Lookup für das Duplikat-Handling.

### Schritt 2: Modell `SavedFilter` + DB-Layer

- [ ] `src/models/saved_filter.rs` erstellen mit:
  - `struct SavedFilter { id: i64, name: String, query_string: String, created_at: String }`
    mit `#[derive(Debug, FromRow)]`
  - `struct CreateSavedFilter { name: String, query_string: String }` mit Validierung:
    - `name` darf nicht leer sein (Trim-Check)
    - `name` max. 100 Zeichen
    - `query_string` darf nicht leer sein (kein Filter ohne aktiven Filter speichern)
  - Unit-Tests für die Validierung (TDD: Test zuerst, dann Impl.)

- [ ] `src/models/saved_filter_db.rs` erstellen mit folgenden Funktionen:
  - `async fn get_all_saved_filters(pool: &SqlitePool) -> Result<Vec<SavedFilter>, sqlx::Error>`
    Sortierung: `ORDER BY created_at ASC` (älteste zuerst, stabile Reihenfolge)
  - `async fn create_saved_filter(pool: &SqlitePool, filter: &CreateSavedFilter) -> Result<i64, sqlx::Error>`
    Gibt `sqlx::Error::Database` mit UNIQUE-Constraint zurück bei Duplikat
  - `async fn delete_saved_filter(pool: &SqlitePool, id: i64) -> Result<(), sqlx::Error>`
    Gibt `RowNotFound` zurück, wenn ID nicht existiert
  - `async fn update_saved_filter_name(pool: &SqlitePool, id: i64, new_name: &str) -> Result<(), sqlx::Error>`
    Für zukünftige Umbenennung (sauber mithaben, auch wenn UI zunächst kein Umbenennen hat)
  - Unit-Tests (TDD): Tests gegen echte In-Memory-SQLite-DB mit `tempfile::NamedTempFile`
    (Vorbild: `tests/recipe_category_filter.rs`)

- [ ] `src/models/mod.rs` erweitern:
  ```rust
  pub mod saved_filter;
  pub mod saved_filter_db;
  pub use saved_filter::{CreateSavedFilter, SavedFilter};
  pub use saved_filter_db::{
      create_saved_filter, delete_saved_filter, get_all_saved_filters,
  };
  ```

### Schritt 3: Template-Structs erweitern

- [ ] `src/templates.rs` erweitern:
  - `SavedFilterItem` struct für die Template-Darstellung:
    ```rust
    pub struct SavedFilterItem {
        pub id: i64,
        pub name: String,
        /// Ziel-URL für den Klick: "/?<query_string>"
        pub url: String,
        /// ARIA-Label für den Löschen-Button: "Filter '<name>' löschen"
        pub delete_aria_label: String,
    }
    ```
  - `IndexTemplate` um folgende Felder erweitern:
    ```rust
    /// Alle gespeicherten Filter (aus DB, für die Anzeige)
    pub saved_filters: Vec<SavedFilterItem>,
    /// Query-String des aktuellen Filterzustands (für das Speichern-Formular)
    /// Leer wenn kein Filter aktiv. Format: "kategorie=Brot&bewertung=gut"
    pub current_query_string: String,
    ```

- [ ] `SavedFilterItem`-Hilfsfunktionen in `templates.rs`:
  Keine speziellen Methoden nötig — `url` und `delete_aria_label` werden schon beim Erstellen befüllt.

### Schritt 4: `index`-Handler erweitern

In `src/routes/recipes.rs`:

- [ ] Hilfsfunktion `build_current_query_string(...)` erstellen:
  Baut aus den aktiven Filterparametern den Query-String (ohne `?`) für das Speichern-Formular.
  Wiederverwendet die bereits extrahierten Werte: `active_categories`, `search_query`,
  `filter_param`, `bewertung`. Ergebnis ist z.B. `"kategorie=Brot&bewertung=gut"`.

- [ ] DB-Aufruf `get_all_saved_filters(&pool)` im `index`-Handler hinzufügen

- [ ] `saved_filters` und `current_query_string` in `IndexTemplate` befüllen:
  ```rust
  let saved_filter_items: Vec<SavedFilterItem> = saved_filters
      .into_iter()
      .map(|sf| SavedFilterItem {
          id: sf.id,
          name: sf.name.clone(),
          url: format!("/?{}", sf.query_string),
          delete_aria_label: format!("Filter '{}' löschen", sf.name),
      })
      .collect();
  ```

### Schritt 5: Neue HTTP-Routen und Handler

In `src/routes/recipes.rs`:

- [ ] `POST /saved-filters` — Handler `create_saved_filter_handler`:
  - Liest `name` und `query_string` aus Formular-Body
  - Validiert: `name` nicht leer, `query_string` nicht leer
  - Ruft `create_saved_filter` auf
  - **Duplikat-Handling:** Wenn UNIQUE-Constraint verletzt →
    Redirect mit `?save_error=duplikat&save_name=<url-encoded-name>` zurück zu `/`
    (die aktiven Filter werden über den `query_string` in der URL nicht verloren,
    denn wir leiten zu `/?<aktueller_query_string>&save_error=duplikat&save_name=...`)
  - Erfolg: Redirect zu `/?<query_string>` (Filter bleiben aktiv)
  - Fehler: Redirect zu `/` mit Fehler-Query-Parameter

- [ ] `POST /saved-filters/:id/delete` — Handler `delete_saved_filter_handler`:
  - Liest `:id` aus Pfad
  - Ruft `delete_saved_filter` auf
  - **HTMX-Antwort:** Gibt leeren 200-Response zurück (HTMX entfernt das Element per `hx-swap="delete"`)
  - **Fallback ohne JS:** Redirect zu `/`
  - Erkennung via `HX-Request`-Header ob HTMX-Request

- [ ] Beide Routen in `src/routes/mod.rs` registrieren:
  ```rust
  .route("/saved-filters", post(recipes::create_saved_filter_handler))
  .route("/saved-filters/:id/delete", post(recipes::delete_saved_filter_handler))
  ```

### Schritt 6: `IndexQuery` erweitern für Fehler-Feedback

In `src/routes/recipes.rs`:

- [ ] `IndexQuery` struct um zwei Felder erweitern:
  ```rust
  pub save_error: Option<String>,   // "duplikat" → Fehlertyp
  pub save_name: Option<String>,    // Name der versuchten Speicherung (für Fehlermeldung)
  ```

- [ ] Im `index`-Handler: `save_error` und `save_name` aus Query lesen und
  ins `IndexTemplate` weitergeben

- [ ] `IndexTemplate` um folgende Felder erweitern:
  ```rust
  /// Fehler beim Speichern: "duplikat" | None
  pub save_error: Option<String>,
  /// Der Name, der beim Speichern verwendet wurde (für Fehlermeldung im Template)
  pub save_name: Option<String>,
  ```

### Schritt 7: Template `templates/index.html` erweitern

- [ ] Bereich "Gespeicherte Filter" nach dem `<div class="sort-filter">` einfügen:
  ```html
  {% if !saved_filters.is_empty() %}
  <div class="saved-filters" aria-label="Gespeicherte Filter">
      {% for sf in saved_filters %}
      <div class="saved-filter-item">
          <a
              href="{{ sf.url }}"
              class="saved-filter-btn"
              hx-get="{{ sf.url }}"
              hx-target="#recipe-results"
              hx-push-url="true"
              hx-select="#recipe-results"
          >{{ sf.name }}</a>
          <form
              method="POST"
              action="/saved-filters/{{ sf.id }}/delete"
              class="saved-filter-delete-form"
          >
              <button
                  type="submit"
                  class="btn-icon saved-filter-delete-btn"
                  aria-label="{{ sf.delete_aria_label }}"
                  hx-post="/saved-filters/{{ sf.id }}/delete"
                  hx-target="closest .saved-filter-item"
                  hx-swap="delete"
              >{% call icons::icon_x() %}</button>
          </form>
      </div>
      {% endfor %}
  </div>
  {% endif %}
  ```

- [ ] Bereich "Filter speichern" innerhalb des `sort-filter`-Blocks (nur wenn `any_filter_active`):
  ```html
  {% if any_filter_active %}
  <div class="save-filter-area">
      {% if let Some(err) = save_error %}
          {% if err == "duplikat" %}
          <p class="save-filter-error" role="alert">
              Filter "{{ save_name.as_deref().unwrap_or("") }}" existiert bereits.
              Wähle einen anderen Namen oder überschreibe den vorhandenen Filter.
          </p>
          {% endif %}
      {% endif %}
      <form
          method="POST"
          action="/saved-filters"
          class="save-filter-form"
      >
          <input type="hidden" name="query_string" value="{{ current_query_string }}">
          <label for="save-filter-name" class="visually-hidden">Name für diesen Filter</label>
          <input
              id="save-filter-name"
              name="name"
              type="text"
              placeholder="Filtername..."
              maxlength="100"
              required
              class="save-filter-input"
              {% if let Some(n) = save_name %}value="{{ n }}"{% endif %}
          >
          <button type="submit" class="btn-secondary save-filter-submit">Filter speichern</button>
      </form>
  </div>
  {% endif %}
  ```

  **Hinweis zu K5 (Doppelter Name / Überschreiben):**
  Für das Überschreiben wird zunächst kein separates Formular implementiert — der Benutzer
  erhält die Fehlermeldung und kann den Filter manuell löschen und dann neu speichern.
  Das Umbenennen/Überschreiben wird als separate Aufgabe zurückgestellt (offener Punkt).

### Schritt 8: CSS-Styling

In `src/static/css/app.css`:

- [ ] Neue CSS-Klassen für den Bereich "Gespeicherte Filter":
  - `.saved-filters` — Container (flexbox, wrap, analog zu `.sort-filter`)
  - `.saved-filter-item` — Einzelnes Item mit Button + Löschen
  - `.saved-filter-btn` — Klickbarer Chip (analog zu `.sort-filter-btn`)
  - `.saved-filter-delete-btn` — Kleines ×-Icon neben dem Namen
  - `.saved-filter-delete-form` — Kein visueller Rahmen (inline)

- [ ] Neue CSS-Klassen für das "Filter speichern"-Formular:
  - `.save-filter-area` — Container für Form + Fehlermeldung
  - `.save-filter-form` — Inline-Flexbox (Input + Button nebeneinander)
  - `.save-filter-input` — Text-Input (analog zu Search-Input)
  - `.save-filter-submit` — Submit-Button (`.btn-secondary`)
  - `.save-filter-error` — Fehlermeldung in Fehlerfarbe (`--error-color`)
  - `.visually-hidden` — Screenreader-only (falls noch nicht vorhanden)

### Schritt 9: Rust-Integrationstests

`tests/saved_filters.rs` erstellen mit:

- [ ] Setup-Funktion (gleiche Struktur wie `recipe_category_filter.rs`)
- [ ] Test: **Filter speichern (Happy Path)**
  ```
  // Given: App mit leerem Zustand
  // When: POST /saved-filters mit name="Brot-Ideen" und query_string="kategorie=Brot"
  // Then: GET / enthält "Brot-Ideen" in der HTML-Antwort
  ```
- [ ] Test: **Filter laden (GET / zeigt gespeicherte Filter)**
  ```
  // Given: Ein gespeicherter Filter "Brot-Ideen" in der DB
  // When: GET /
  // Then: Body enthält Link mit href="/?kategorie=Brot"
  ```
- [ ] Test: **Filter löschen (HTMX-Request)**
  ```
  // Given: Gespeicherter Filter mit ID 1
  // When: POST /saved-filters/1/delete mit HX-Request: true Header
  // Then: 200-Antwort, danach GET / enthält "Brot-Ideen" nicht mehr
  ```
- [ ] Test: **Filter löschen (Fallback ohne HTMX)**
  ```
  // Given: Gespeicherter Filter mit ID 1
  // When: POST /saved-filters/1/delete ohne HX-Request Header
  // Then: 303-Redirect zu /
  ```
- [ ] Test: **Duplikater Name → Fehler-Redirect**
  ```
  // Given: Filter "Brot-Ideen" bereits gespeichert
  // When: POST /saved-filters mit name="Brot-Ideen" und query_string="bewertung=gut"
  // Then: Redirect-URL enthält save_error=duplikat
  ```
- [ ] Test: **Leerer Name wird abgelehnt**
  ```
  // Given: App im Basiszustand
  // When: POST /saved-filters mit leerem name
  // Then: Redirect mit save_error=... oder 400
  ```
- [ ] Test: **Leerer query_string wird abgelehnt**
  ```
  // Given: App im Basiszustand
  // When: POST /saved-filters mit name="Test" und leerem query_string
  // Then: Redirect mit Fehler oder 400
  ```
- [ ] Test: **Nicht vorhandener Filter löschen → 404**
  ```
  // Given: App im Basiszustand, kein Filter mit ID 999
  // When: POST /saved-filters/999/delete
  // Then: 404-Antwort
  ```

Alle Tests enthalten `// Given/When/Then`-Kommentare auf Deutsch (DoD-Pflicht).

### Schritt 10: E2E-Tests (Playwright)

`tests/e2e/saved-filters.spec.ts` erstellen:

- [ ] **Testfall 1 (K1/K2): Filter speichern und aufrufen**
  ```
  // Gegeben: Rezept "Vollkornbrot" in Kategorie "Brot"
  // Wenn: Benutzer klickt Kategorie "Brot", gibt "Brot-Ideen" als Filtername ein, speichert
  // Dann: "Brot-Ideen" erscheint als Chip in der Filterleiste
  // Wenn: Benutzer klickt "Alle" (Filter zurücksetzen), dann auf Chip "Brot-Ideen"
  // Dann: URL enthält "kategorie=Brot", nur Brot-Rezepte sichtbar
  ```

- [ ] **Testfall 2 (K4): Filter ist persistent nach Reload**
  ```
  // Gegeben: Filter "Brot-Ideen" gespeichert
  // Wenn: Seite neu laden (page.reload())
  // Dann: "Brot-Ideen" immer noch sichtbar
  ```

- [ ] **Testfall 3 (K3): Filter löschen**
  ```
  // Gegeben: Filter "Brot-Ideen" gespeichert
  // Wenn: Klick auf Löschen-Button neben "Brot-Ideen"
  // Dann: "Brot-Ideen" verschwindet (HTMX-Delete), nach Reload immer noch weg
  ```

- [ ] **Testfall 4 (K1/K2): Kombinierten Filter speichern und aufrufen**
  ```
  // Gegeben: Rezepte mit Kategorie "Mittagessen" und Bewertung 4
  // Wenn: Kategorie + Bewertungsfilter aktiv, speichern als "Mittagessenplanung"
  // Und: Alle Filter zurücksetzen, dann Chip "Mittagessenplanung" klicken
  // Dann: Kategorie-Button und Bewertungsfilter sind aktiv markiert
  ```

- [ ] **Testfall 5 (K5): Doppelter Name zeigt Fehlermeldung**
  ```
  // Gegeben: "Brot-Ideen" bereits gespeichert
  // Wenn: Erneut Filter speichern unter "Brot-Ideen"
  // Dann: Fehlermeldung "Filter existiert bereits" sichtbar
  // Und: Ursprünglicher "Brot-Ideen"-Filter weiterhin in Liste
  ```

- [ ] **Testfall 6 (K6): Keine Treffer beim Aufrufen — Filter bleibt erhalten**
  ```
  // Gegeben: Filter "Nächste 7 Tage" gespeichert (filter=naechste-7-tage)
  //          und keine Rezepte mit Datum in nächsten 7 Tagen
  // Wenn: Klick auf gespeicherten Filter "Nächste 7 Tage"
  // Dann: "Keine Treffer"-Meldung sichtbar, Filter-Chip weiterhin vorhanden
  ```

- [ ] **Testfall 7 (K7): Kein Speichern-Button ohne aktiven Filter**
  ```
  // Gegeben: Startseite ohne aktive Filter
  // Dann: Speichern-Formular nicht sichtbar
  // Wenn: Kategorie "Brot" aktiviert
  // Dann: Speichern-Formular sichtbar
  ```

Alle Tests enthalten `// Gegeben/Wenn/Dann`-Kommentare (DoD-Pflicht).
Daten werden pro Test mit einem eindeutigen Suffix erstellt (`Date.now()` / `Math.random()`).

### Schritt 11: Qualitätschecks

- [ ] `cargo build` — keine Compiler-Fehler oder Warnungen
- [ ] `cargo clippy -- -D warnings` — keine Clippy-Warnungen
- [ ] `cargo fmt --check` — korrekte Formatierung
- [ ] `cargo test` — alle Unit- und Integrationstests grün
- [ ] `npm run test:e2e` — alle Playwright-Tests grün

---

## URL-Struktur

```
GET  /                              →  Startseite mit allen Filtern und gespeicherten Filtern
POST /saved-filters                 →  Filter speichern (Form-Post), Redirect zurück zu /
POST /saved-filters/:id/delete      →  Filter löschen (HTMX oder Form-Post)
```

Neue Query-Parameter auf `GET /`:
```
/?save_error=duplikat&save_name=<encoded-name>   →  Fehlermeldung nach Duplikat
```

---

## Datenmodell

```sql
-- migrations/002_saved_filters.sql
CREATE TABLE saved_filters (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    name TEXT NOT NULL UNIQUE,
    query_string TEXT NOT NULL,
    created_at TEXT NOT NULL DEFAULT (datetime('now'))
);
CREATE INDEX idx_saved_filters_name ON saved_filters(name);
```

Kein neues Feld in der `recipes`-Tabelle nötig.

---

## Abhängigkeiten

- Story 12 (Kombinierte Filter) ist abgeschlossen — alle Filter sind als URL-Parameter verfügbar
- Bestehende Helper-Funktionen in `recipes.rs` (`build_current_query_string` neu hinzufügen,
  analog zu `build_category_toggle_url`)
- `sqlx::FromRow` für `SavedFilter` (wie bei `Recipe`)
- `tempfile`-Crate bereits als dev-Dependency vorhanden (für Integrationstests)
- `urlencoding`-Crate bereits in Scope

---

## Technische Entscheidungen

### Duplikat-Handling: Redirect statt Überschreiben
Das Überschreiben eines gleichnamigen Filters (K5) wird über eine Fehlermeldung + manuelles
Löschen realisiert. Der Name wird als `save_name`-Parameter zurückgegeben, sodass der Benutzer
ihn nicht neu eingeben muss. Ein One-Step-Überschreiben (Bestätigungsdialog) ist als optionaler
Ausbau zurückgestellt.

### HTMX-Löschen: `hx-swap="delete"` auf `closest .saved-filter-item`
Das Elternelement `.saved-filter-item` wird bei HTMX-Delete vollständig entfernt.
Kein separater Bestätigungsdialog (da kein Rezept betroffen ist — DoD-konform).

### Kein Inline-Umbenennen
Umbenennen = Löschen + Neu speichern. Kein separater UI-Weg. Offener Punkt bleibt offen.

### HX-Request-Header für HTMX-Erkennung
Im `delete`-Handler: wenn `HX-Request: true` Header vorhanden → 200 (leere Antwort für HTMX-Swap).
Ohne Header → 303-Redirect zu `/` (Progressive Enhancement).

### Sortierung gespeicherter Filter
`ORDER BY created_at ASC` — älteste zuerst. Einfach, stabil, vorhersehbar.
Keine Drag-Drop-Reihenfolge (zu komplex für den Use Case).

---

## Test-Checkliste (DoD)

- [ ] Unit-Test: `SavedFilter`-Validierung — leerer Name abgelehnt
- [ ] Unit-Test: `SavedFilter`-Validierung — Name zu lang abgelehnt
- [ ] Unit-Test: `SavedFilter`-Validierung — leerer `query_string` abgelehnt
- [ ] Unit-Test: `SavedFilter`-Validierung — gültige Eingaben akzeptiert
- [ ] Integrationstest: Filter speichern (Happy Path)
- [ ] Integrationstest: Filter laden (Anzeige auf Startseite)
- [ ] Integrationstest: Filter löschen (HTMX-Antwort)
- [ ] Integrationstest: Filter löschen (Fallback-Redirect)
- [ ] Integrationstest: Duplikater Name → Fehler-Redirect
- [ ] Integrationstest: Leerer Name → Fehler
- [ ] Integrationstest: Nicht vorhandener Filter löschen → 404
- [ ] E2E-Test: Filter speichern und aufrufen (K1/K2)
- [ ] E2E-Test: Persistenz nach Reload (K4)
- [ ] E2E-Test: Filter löschen (K3)
- [ ] E2E-Test: Kombinierten Filter speichern und aufrufen
- [ ] E2E-Test: Doppelter Name (K5)
- [ ] E2E-Test: Keine Treffer (K6)
- [ ] E2E-Test: Kein Speichern-Button ohne aktiven Filter (K7)
- [ ] Manueller Test: Filterbereich auf Mobil (responsiv)
- [ ] Manueller Test: Tastatur-Navigation durch gespeicherte Filter

---

## Offene Punkte

- Soll ein gespeicherter Filter visuell als "aktiv" markiert werden, wenn seine Parameter
  gerade in der URL aktiv sind? (Story-Frage, zurückgestellt — technisch machbar durch
  Vergleich von `current_query_string` mit `sf.query_string`)
- Überschreiben eines gespeicherten Filters in einem Schritt (Bestätigungsdialog) — aktuell
  nur via Löschen + Neu speichern
- Anpassbare Sortierung der gespeicherten Filter (zurückgestellt, zu komplex)
