# Implementierungsplan: Story 21 — Duplikaterkennung während Titeleingabe

## Technische Analyse

### Architekturentscheidung: LIKE-Suche vs. FTS5

Die story.md stellt als offene Frage, ob einfaches `LIKE '%term%'` oder eine
fortgeschrittenere Methode (FTS5, Levenshtein) verwendet werden soll.

**Entscheidung: `LIKE '%term%'` auf dem `title`-Feld.**

Begründung:
- Die Rezeptsammlung hat 40-50 Einträge — kein Performance-Problem.
- Der `idx_recipes_title`-Index beschleunigt die Abfrage ausreichend.
- Das Akzeptanzkriterium K6 ("Pizza Margherita" wird bei "Margherita" gefunden)
  ist mit `LOWER(title) LIKE '%margherita%'` vollständig erfüllbar.
- FTS5 wäre Overkill und bräuchte eine Migration, Levenshtein ist ohne SQLite-Extension
  nicht verfügbar (kein Python auf dem System).

### Endpunkt

`GET /recipes/check-duplicate?title=...&exclude_id=...`

- `title`: Der aktuelle Wert des Titelfelds (URL-kodiert).
- `exclude_id`: Optionale ID des aktuell bearbeiteten Rezepts (beim Bearbeiten).
- Antwortet immer mit einem HTML-Fragment (Askama-Partial-Template).
- Leere Antwort (leerer `<div>`) wenn keine Treffer oder Titel < 3 Zeichen.

### Neues Template

`templates/recipes/_duplicate_hint.html` — ein HTMX-Fragment ohne `{% extends %}`.

### Integration ins Formular

Das bestehende `templates/recipes/form.html` erhält:
- HTMX-Attribute am Titelfeld.
- Einen leeren `<div id="duplicate-hint" aria-live="polite">` unter dem Titelfeld.

### Neue Funktion im DB-Layer

`find_similar_recipes(pool, title, exclude_id) -> Result<Vec<SimilarRecipe>, sqlx::Error>`

Sucht bis zu 3 Rezepte, deren Titel den eingegebenen Begriff enthält
(case-insensitive LIKE), optional exklusive einer bestimmten ID.

`SimilarRecipe` ist ein leichtgewichtiger Struct (nur `id`, `title`, `rating`).

---

## Technische Schritte

### Schritt 1: DB-Layer — `find_similar_recipes`

- [ ] In `src/models/recipe_db.rs` neuen Struct `SimilarRecipe` definieren:
  ```rust
  pub struct SimilarRecipe {
      pub id: i64,
      pub title: String,
      pub rating: Option<i32>,
  }
  ```
- [ ] Funktion `find_similar_recipes` implementieren:
  ```rust
  pub async fn find_similar_recipes(
      pool: &SqlitePool,
      title: &str,
      exclude_id: Option<i64>,
  ) -> Result<Vec<SimilarRecipe>, sqlx::Error>
  ```
  - Gibt leere `Vec` zurück wenn `title.trim().len() < 3`.
  - SQL: `LOWER(title) LIKE '%<term>%'`, `id != exclude_id`, `LIMIT 3`.
  - Alphabetisch sortiert nach Titel.
- [ ] Unit-Test: `find_similar_recipes_returns_empty_for_short_title`
  - Given: Titel mit 2 Zeichen; Then: leere Ergebnisliste.
- [ ] Unit-Test: `find_similar_recipes_finds_substring_match`
  - Given: Rezept "Dinkelbrot"; When: Suche nach "Dinkel"; Then: Treffer gefunden.
- [ ] Unit-Test: `find_similar_recipes_is_case_insensitive`
  - Given: Rezept "Dinkelbrot"; When: Suche nach "DINKEL"; Then: Treffer gefunden.
- [ ] Unit-Test: `find_similar_recipes_excludes_self`
  - Given: Rezept "Dinkelbrot" mit ID 1; When: Suche mit exclude_id=1; Then: kein Treffer.
- [ ] Unit-Test: `find_similar_recipes_limits_to_three`
  - Given: 5 Rezepte mit "Dinkel" im Titel; When: Suche nach "Dinkel"; Then: max. 3 Treffer.
- [ ] Unit-Test: `find_similar_recipes_no_match_returns_empty`
  - Given: Rezept "Spaghetti"; When: Suche nach "Dinkel"; Then: leere Liste.

### Schritt 2: Modell-Export und Rust-Struct

- [ ] `SimilarRecipe` in `src/models/mod.rs` re-exportieren:
  `pub use recipe_db::{..., find_similar_recipes, SimilarRecipe};`

### Schritt 3: Askama-Template für den Duplikat-Hinweis

- [ ] Neue Datei `templates/recipes/_duplicate_hint.html` erstellen:
  - Kein `{% extends %}`, reines HTML-Fragment.
  - Wenn `candidates` leer: gibt leer aus (nur leeres Fragment, kein sichtbares Element).
  - Wenn Treffer vorhanden: Info-Block mit Liste der Kandidaten.
    - Visuell klar als Info (nicht als Fehler) — CSS-Klasse `duplicate-hint-info`.
    - Jeder Kandidat zeigt: Titel, Sternbewertung (falls vorhanden), klickbarer Link zur Detailansicht.
    - Struktur:
      ```html
      <div class="duplicate-hint-info">
        <strong>Ähnliche Rezepte gefunden:</strong>
        <ul class="duplicate-hint-list">
          {% for c in candidates %}
          <li>
            <a href="/recipes/{{ c.id }}">{{ c.title }}</a>
            {% if let Some(r) = c.rating %}({{ r }}★){% else %}(nicht bewertet){% endif %}
          </li>
          {% endfor %}
        </ul>
      </div>
      ```

### Schritt 4: Template-Struct in `src/templates.rs`

- [ ] Neuen Template-Struct hinzufügen:
  ```rust
  #[derive(Template)]
  #[template(path = "recipes/_duplicate_hint.html")]
  pub struct DuplicateHintTemplate {
      pub candidates: Vec<SimilarRecipe>,
  }
  ```
- [ ] Import von `SimilarRecipe` in `templates.rs` aus `crate::models`.

### Schritt 5: HTTP-Handler `check_duplicate`

- [ ] In `src/routes/recipes.rs` neuen Query-Struct definieren:
  ```rust
  #[derive(Deserialize)]
  pub struct CheckDuplicateQuery {
      pub title: Option<String>,
      pub exclude_id: Option<i64>,
  }
  ```
- [ ] Handler implementieren:
  ```rust
  pub async fn check_duplicate(
      State(pool): State<Arc<SqlitePool>>,
      Query(params): Query<CheckDuplicateQuery>,
  ) -> impl IntoResponse
  ```
  - Liest `title` aus Query-Params (leer-sicher: `unwrap_or_default`).
  - Ruft `find_similar_recipes(&pool, &title, exclude_id)` auf.
  - Rendert `DuplicateHintTemplate { candidates }` → HTML-Fragment.
  - Gibt HTTP 200 mit HTML zurück.
  - Bei DB-Fehler: gibt leeres Fragment zurück (Graceful Degradation).

### Schritt 6: Route registrieren

- [ ] In `src/routes/mod.rs` neue Route hinzufügen:
  ```rust
  .route("/recipes/check-duplicate", get(recipes::check_duplicate))
  ```
  **Wichtig:** Die Route muss VOR `/recipes/:id` registriert werden, damit
  `check-duplicate` nicht als `:id` gematcht wird.

### Schritt 7: Formular-Template anpassen

- [ ] In `templates/recipes/form.html` das Titelfeld um HTMX-Attribute erweitern:
  ```html
  <input
    type="text"
    id="title"
    name="title"
    value="{{ title }}"
    maxlength="100"
    hx-get="/recipes/check-duplicate"
    hx-trigger="input changed delay:400ms"
    hx-target="#duplicate-hint"
    hx-include="[name='title']"
    {% if let Some(id) = recipe_id %}
    hx-vals='{"exclude_id": "{{ id }}"}'
    {% endif %}
  >
  ```
- [ ] Direkt nach dem Titelfeld-`<div class="form-group">` einen leeren Hinweis-Container einfügen:
  ```html
  <div id="duplicate-hint" aria-live="polite"></div>
  ```

### Schritt 8: CSS-Styling

- [ ] In `src/static/css/app.css` Stile für den Duplikat-Hinweis hinzufügen:
  - `.duplicate-hint-info`: Info-Farbe (z.B. gelb-licher Hintergrund, kein Rot),
    deutlich vom Formular abgegrenzt, Padding, Border-left in Akzentfarbe.
  - `.duplicate-hint-list`: Liste ohne Standard-Bullets, kompakter Abstand.
  - Links im Hinweis mit sichtbarem Fokus-Indikator (erfüllt K8).

### Schritt 9: Rust-Integrationstests

- [ ] Neue Datei `tests/recipe_duplicate_check.rs` erstellen.
- [ ] Test: `check_duplicate_returns_empty_for_short_title`
  - Given: App mit Rezept "Dinkelbrot"; When: GET `/recipes/check-duplicate?title=Di`;
    Then: HTTP 200, Body enthält KEINEN Hinweis-Block.
- [ ] Test: `check_duplicate_finds_similar_recipe`
  - Given: App mit Rezept "Dinkelbrot"; When: GET `/recipes/check-duplicate?title=Dinkel`;
    Then: HTTP 200, Body enthält "Dinkelbrot".
- [ ] Test: `check_duplicate_is_case_insensitive`
  - Given: App mit "Dinkelbrot"; When: GET `/recipes/check-duplicate?title=DINKEL`;
    Then: Body enthält "Dinkelbrot".
- [ ] Test: `check_duplicate_excludes_self`
  - Given: Rezept "Dinkelbrot" mit bekannter ID; When: GET `/recipes/check-duplicate?title=Dinkelbrot&exclude_id=<id>`;
    Then: Body enthält "Dinkelbrot" NICHT.
- [ ] Test: `check_duplicate_returns_empty_when_no_match`
  - Given: App mit "Spaghetti"; When: GET `/recipes/check-duplicate?title=Dinkel`;
    Then: Body enthält KEINEN Hinweis-Block.
- [ ] Test: `check_duplicate_returns_200`
  - When: GET `/recipes/check-duplicate?title=Test`; Then: HTTP 200 (auch ohne Treffer).
- [ ] Jeden Test mit Given/When/Then als deutsche Kommentare inline versehen.

### Schritt 10: E2E-Tests (Playwright)

- [ ] Neue Datei `tests/e2e/recipe-duplicate-check.spec.ts` erstellen.
- [ ] Hilfsfunktion `createRecipe` (wie in anderen Specs — Rezept über Formular anlegen).
- [ ] **Testfall K1: Duplikat-Hinweis erscheint bei ähnlichem Titel**
  ```
  // Given: Ein Rezept "Dinkelbrot" existiert
  // And: Formular für neues Rezept geöffnet
  // When: Benutzer gibt "Dinkel" in Titelfeld ein
  // And: Benutzer wartet 500ms (debounce)
  // Then: Hinweis mit "Ähnliche Rezepte gefunden" erscheint
  // And: "Dinkelbrot" ist in der Hinweisliste sichtbar
  // And: Link zur Detailansicht von "Dinkelbrot" ist vorhanden
  ```
- [ ] **Testfall K3: Hinweis verschwindet bei keiner Übereinstimmung**
  ```
  // Given: Hinweis für "Dinkel" ist sichtbar
  // When: Benutzer ändert Titel auf "Spaghetti Bolognese"
  // And: Wartet 500ms
  // Then: Duplikat-Hinweis ist nicht mehr sichtbar
  ```
- [ ] **Testfall K1 (Edge): Kein Hinweis bei kurzem Titel (< 3 Zeichen)**
  ```
  // Given: Rezept "Dinkelbrot" existiert
  // When: Benutzer gibt "Di" ins Titelfeld ein + wartet 500ms
  // Then: Kein Hinweis erscheint
  ```
- [ ] **Testfall K4: Aktuelles Rezept nicht als Duplikat beim Bearbeiten**
  ```
  // Given: Rezept "Dinkelbrot" existiert
  // And: Bearbeitungsformular geöffnet
  // When: Benutzer klickt in Titelfeld (Titel bleibt "Dinkelbrot") + wartet 500ms
  // Then: "Dinkelbrot" erscheint NICHT im Duplikat-Hinweis
  ```
- [ ] **Testfall K5: Speichern trotz Hinweis möglich**
  ```
  // Given: Rezept "Dinkelbrot" existiert
  // And: Neues Formular mit Titel "Dinkelbrot 2" → Hinweis sichtbar
  // When: Benutzer füllt Formular aus und speichert
  // Then: Neues Rezept wird angelegt, Weiterleitung zur Detailansicht
  ```
- [ ] **Testfall K2: Jeder Kandidat enthält Link zur Detailansicht**
  ```
  // Given: Hinweis ist sichtbar mit "Dinkelbrot"
  // Then: Link href="/recipes/<id>" ist klickbar und führt zur Detailseite
  ```
- [ ] **Testfall K6: Ähnlichkeitssuche ist case-insensitiv**
  ```
  // Given: Rezept "Dinkelbrot" existiert
  // When: Benutzer gibt "DINKEL" ein + wartet 500ms
  // Then: Hinweis mit "Dinkelbrot" erscheint
  ```
- [ ] Jeder Test mit deutschen Given/When/Then-Kommentaren inline versehen.

### Schritt 11: Qualitätschecks

- [ ] `cargo build` — keine Compiler-Warnungen.
- [ ] `cargo clippy -- -D warnings` — keine Clippy-Warnungen.
- [ ] `cargo fmt --check` — korrekte Formatierung.
- [ ] `cargo test` — alle Unit- und Integrationstests grün.
- [ ] `npm run test:e2e` — alle E2E-Tests grün.

---

## URL-Struktur

```
GET  /recipes/check-duplicate?title=<Suchbegriff>&exclude_id=<id>  →  HTML-Fragment (Duplikat-Hinweis)
```

---

## Abhängigkeiten

- Story 01 (Rezept erstellen) ist abgeschlossen — das Formular `form.html` existiert.
- Story 02 (Rezept bearbeiten) ist abgeschlossen — das Bearbeitungsformular ist dasselbe Template.
- HTMX ist bereits im Projekt integriert (in `base.html` eingebunden).
- Kein Datenbank-Schema-Update erforderlich.
- `idx_recipes_title`-Index existiert bereits (Performance).
- Bestehende Typen: `Recipe`, `SqlitePool`, `AppError` — werden wiederverwendet.

---

## Test-Checkliste

- [ ] Unit-Test: `find_similar_recipes_returns_empty_for_short_title` — keine Suche bei < 3 Zeichen
- [ ] Unit-Test: `find_similar_recipes_finds_substring_match` — LIKE-Suche findet Teilstring
- [ ] Unit-Test: `find_similar_recipes_is_case_insensitive` — Groß-/Kleinschreibung irrelevant
- [ ] Unit-Test: `find_similar_recipes_excludes_self` — eigenes Rezept nicht als Duplikat
- [ ] Unit-Test: `find_similar_recipes_limits_to_three` — maximal 3 Treffer
- [ ] Unit-Test: `find_similar_recipes_no_match_returns_empty` — leere Liste bei keinem Treffer
- [ ] Integrationstest: Endpunkt antwortet mit HTTP 200
- [ ] Integrationstest: Hinweis bei Treffer im Response-Body
- [ ] Integrationstest: Kein Hinweis bei kurzen Titeln
- [ ] Integrationstest: exclude_id filtert das eigene Rezept heraus
- [ ] E2E-Test: Hinweis erscheint bei ähnlichem Titel (K1)
- [ ] E2E-Test: Hinweis verschwindet bei keiner Übereinstimmung (K3)
- [ ] E2E-Test: Kein Hinweis bei < 3 Zeichen (K1 Edge)
- [ ] E2E-Test: Eigenes Rezept beim Bearbeiten nicht als Duplikat (K4)
- [ ] E2E-Test: Speichern trotz Hinweis möglich (K5)
- [ ] E2E-Test: Links im Hinweis funktionieren (K2)
- [ ] E2E-Test: Case-insensitiv (K6)
- [ ] Manuell: HTMX-Anfrage in DevTools prüfen (Timing < 200ms, K7)
- [ ] Manuell: Screenreader — `aria-live="polite"` am Hinweis-Container (K8)
- [ ] Manuell: Ohne JavaScript — kein Hinweis, Speichern weiterhin möglich (K5, Progressive Enhancement)

---

## Offene Punkte

- **Ähnlichkeitssuche:** `LIKE '%term%'` auf `title`-Feld — entschieden (siehe Analyse oben).
- **Maximale Trefferzahl:** 3 Treffer (in story.md vorgeschlagen, hier übernommen).
- **HTMX-Methode:** `hx-get` mit Query-Parametern statt `hx-post` (passt besser zu GET-Semantik,
  da nur Daten abgefragt werden).
- **`hx-include` vs. `hx-vals`:** Das `exclude_id` wird über `hx-vals` als statischer JSON-Wert
  übergeben (beim Bearbeiten bekannt und unveränderlich), der Titel wird automatisch als
  aktueller Input-Wert per HTMX gesendet.
