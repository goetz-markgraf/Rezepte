# Implementierungsplan: Story 17 – Inline-Bewertung ohne Edit-Mode

## Übersicht

Die Story ergänzt die bestehende Detailansicht (`/recipes/:id`) um eine direkt antippbare
Sterne-Bewertung per HTMX. Das `rating`-Feld existiert bereits vollständig in DB, Model,
Routen und Templates (Story 14). Es wird kein neues Datenbankfeld benötigt.

Neuer Endpunkt: `POST /recipes/:id/rating`
Response: HTML-Fragment mit dem aktualisierten Sterne-Bereich (HTMX-Swap)

---

## Technische Schritte

### Schritt 1: DB-Funktion `update_recipe_rating` (TDD: erst Test, dann Implementierung)

- [ ] Integrations-Test in `tests/recipe_inline_rating.rs` schreiben:
  - Test: `update_rating_stores_new_value` – POST auf `/recipes/:id/rating` mit `rating=4`, dann GET der Detailseite prüft `4 von 5 Sternen`
  - Test: `update_rating_resets_to_none` – POST auf `/recipes/:id/rating` mit leerem `rating=`, dann GET prüft kein `recipe-stars`
  - Test: `update_rating_returns_html_fragment` – Response-Body enthält `inline-rating` (den Container-Selektor)
  - Test: `update_rating_rejects_invalid_value` – POST mit `rating=6` gibt HTTP 400
  - Test: `update_rating_returns_404_for_unknown_id` – POST auf `/recipes/99999/rating` gibt HTTP 404
- [ ] Neue DB-Funktion `update_recipe_rating(pool, id, rating: Option<i32>)` in `src/models/recipe_db.rs` erstellen:
  - SQL: `UPDATE recipes SET rating = ?1, updated_at = CURRENT_TIMESTAMP WHERE id = ?2`
  - Gibt `sqlx::Error::RowNotFound` zurück, wenn keine Zeile betroffen (rows_affected == 0)
- [ ] Funktion in `src/models/mod.rs` re-exportieren

### Schritt 2: Neues HTMX-Partial-Template `templates/recipes/_inline_rating.html`

- [ ] Neue Template-Datei `templates/recipes/_inline_rating.html` erstellen
- [ ] Das Partial rendert nur den Sterne-Container (kein `extends base.html`):
  - Container `<div id="inline-rating" class="inline-rating" ...>`
  - 5 einzelne `<button>`-Elemente (Stern 1–5), je mit:
    - `hx-post="/recipes/{{ id }}/rating"`
    - `hx-vals='{"rating": "N"}` – wobei N der Sternwert ist; wenn dieser Stern bereits die aktuelle Bewertung ist, `hx-vals='{"rating": ""}` (Toggle → Reset)
    - `hx-target="#inline-rating"`
    - `hx-swap="outerHTML"`
    - `aria-label="N Stern(e) – aktiv/inaktiv"`
    - Visuelle Darstellung: ausgefüllt (★) wenn `n <= rating`, sonst leer (☆)
  - Hinweis „Bewertung antippen zum Ändern" oder visuelles Signal (z.B. kleiner Stift-Hinweis)
- [ ] Askama-Template-Struct `InlineRatingTemplate` in `src/templates.rs` anlegen:
  ```rust
  #[derive(Template)]
  #[template(path = "recipes/_inline_rating.html")]
  pub struct InlineRatingTemplate {
      pub id: i64,
      pub rating: Option<i32>,
  }
  ```
- [ ] Hilfsmethode `rating_is_active(n: i32) -> bool` im Impl-Block (gibt `true` zurück wenn `self.rating == Some(n)`)

### Schritt 3: Route-Handler `update_recipe_rating_handler` in `src/routes/recipes.rs`

- [ ] Neuen Handler schreiben (TDD: Tests aus Schritt 1 sind bereits rot):
  ```rust
  pub async fn update_recipe_rating_handler(
      State(pool): State<Arc<SqlitePool>>,
      Path(id): Path<i64>,
      axum::extract::RawForm(body): axum::extract::RawForm,
  ) -> Result<impl IntoResponse, AppError>
  ```
- [ ] Logik:
  1. Body parsen mit `parse_form_data` (bereits vorhanden), `rating`-Feld extrahieren
  2. Rating validieren mit `parse_rating` (bereits vorhanden) → `Option<i32>`
  3. Wenn Wert vorhanden und außerhalb 1–5: `AppError::BadRequest` zurückgeben
  4. `update_recipe_rating(&pool, id, rating)` aufrufen
  5. Fehler `RowNotFound` → `AppError::NotFound`
  6. `InlineRatingTemplate { id, rating }` rendern und als `Html(...)` zurückgeben

### Schritt 4: Route registrieren in `src/routes/mod.rs`

- [ ] Route eintragen:
  ```rust
  .route("/recipes/:id/rating", post(recipes::update_recipe_rating_handler))
  ```

### Schritt 5: Detail-Template anpassen (`templates/recipes/detail.html`)

- [ ] Den bestehenden `<span class="recipe-stars">` Block ersetzen durch das neue interaktive Partial
- [ ] Für HTMX: Den Sterne-Container mit `id="inline-rating"` versehen, damit HTMX-Swap funktioniert
- [ ] Das Partial wird direkt eingebettet (Askama `{% include "recipes/_inline_rating.html" %}`)
- [ ] Für Progressive Enhancement (kein JS): Die Buttons sind normale HTML-Buttons in einem `<form>` mit `action="/recipes/{{ id }}/rating" method="POST"` – dadurch funktioniert die Bewertung auch ohne HTMX
- [ ] HTMX-Attribute verhindern den Form-Submit bei aktivem JS (da hx-post den Submit übernimmt)

### Schritt 6: CSS-Styling (`src/static/css/app.css`)

- [ ] Neue CSS-Klassen für den Inline-Bewertungs-Container und die Sterne-Buttons:
  - `.inline-rating` – Flex-Container mit `gap` und `align-items: center`
  - `.inline-rating-btn` – Button-Reset, mindestens 44×44px (Touch-Anforderung K8), Cursor pointer
  - `.inline-rating-btn.active` – Ausgefüllter Stern, Farbe `#f59e0b`
  - `.inline-rating-btn:hover` – Hover-Effekt: alle Sterne bis zum gehoverten gelb
  - `.inline-rating-none` – Anzeige wenn keine Bewertung gesetzt ist (optionaler Hinweis-Text)
  - Fokus-Ring (`:focus-visible`) für Tastatur-Navigation (WCAG K8)
- [ ] Hover-Effekt über CSS (`:hover ~ button` / Flex-Reihenfolge umkehren oder JS-Fallback):
  - Da "alle Sterne bis N" in CSS mit normaler DOM-Reihenfolge schwierig ist, Strategie:
    - **Option A (bevorzugt):** Sterne in umgekehrter Reihenfolge rendern (5→1) und mit CSS `flex-direction: row-reverse` + `:hover` & `~ button` arbeiten → rein CSS, kein JS
    - **Option B:** Kleines JavaScript für den Hover-Effekt (akzeptabel als Enhancement)
  - Entscheidung für **Option A** (kein zusätzliches JS, pure CSS)

### Schritt 7: E2E-Tests (Playwright) in `tests/e2e/recipe-inline-rating.spec.ts`

- [ ] Seed-Datei `tests/seeds/recipe-inline-rating.sql` erstellen:
  ```sql
  INSERT INTO recipes (title, categories, rating) VALUES
      ('Testrezept ohne Bewertung', '["Mittagessen"]', NULL),
      ('Testrezept mit 3 Sternen', '["Kuchen"]', 3),
      ('Testrezept mit 4 Sternen', '["Party"]', 4);
  ```
- [ ] `tests/e2e/recipe-inline-rating.spec.ts` erstellen mit allen 4 Testfällen aus der Story:

  **Testfall 1: Inline-Bewertung setzen (K1, K2, K4)**
  ```gherkin
  // Given: Ein Rezept ohne Bewertung existiert in der Datenbank
  // When: Benutzer öffnet die Detailseite des Rezepts
  // And: Benutzer klickt auf den 4. Stern
  // Then: Die Bewertung wird sofort als 4 Sterne angezeigt (ohne Seitenneuladung)
  // And: Nach einer Seitenneuladung ist die Bewertung immer noch 4 Sterne
  ```

  **Testfall 2: Bewertung ändern (K2, K4)**
  ```gherkin
  // Given: Ein Rezept mit 3-Sterne-Bewertung existiert
  // When: Benutzer öffnet die Detailseite
  // And: Benutzer klickt auf den 5. Stern
  // Then: Die Bewertung wird sofort als 5 Sterne angezeigt
  // And: Nach einer Seitenneuladung ist die Bewertung 5 Sterne
  ```

  **Testfall 3: Bewertung zurücksetzen durch erneutes Antippen (K3)**
  ```gherkin
  // Given: Ein Rezept mit 4-Sterne-Bewertung existiert
  // When: Benutzer öffnet die Detailseite
  // And: Benutzer klickt auf den 4. Stern (den aktuell aktiven)
  // Then: Die Bewertung wird entfernt
  // And: Nach einer Seitenneuladung hat das Rezept keine Bewertung
  ```

  **Testfall 4: Inline-Bewertung und Edit-Mode zeigen gleichen Wert (K6)**
  ```gherkin
  // Given: Ein Rezept ohne Bewertung existiert
  // When: Benutzer setzt per Inline-Bewertung 5 Sterne
  // And: Benutzer öffnet den Edit-Mode des Rezepts
  // Then: Im Edit-Formular sind 5 Sterne vorausgewählt
  ```

  **Testfall 5: Tastatur-Navigation (K8)**
  ```gherkin
  // Given: Detailseite eines Rezepts ist geöffnet
  // When: Benutzer navigiert per Tab zu den Sterne-Buttons
  // And: Benutzer drückt Enter/Leertaste auf einem Stern
  // Then: Die Bewertung wird gesetzt
  ```

### Schritt 8: Qualitätssicherung

- [ ] `cargo build` – keine Compiler-Fehler oder Warnungen
- [ ] `cargo clippy -- -D warnings` – keine Clippy-Warnings
- [ ] `cargo fmt --check` – korrekte Formatierung
- [ ] `cargo test` – alle Unit- und Integrationstests grün
- [ ] `npm run test:e2e` – alle E2E-Tests grün
- [ ] Manueller Test: Detail-Seite auf Mobilgerät/Browser – Touch-Fläche ausreichend groß?

---

## URL-Struktur

```
POST  /recipes/:id/rating  →  Nur das Rating-Feld aktualisieren; antwortet mit HTML-Fragment
```

Der Endpunkt ist HTMX-aware: er gibt kein komplettes HTML-Dokument zurück, sondern
nur das aktualisierte Sterne-Fragment (`<div id="inline-rating">...</div>`).

---

## Abhängigkeiten

- Story 14 (rating-Feld) muss abgeschlossen sein ✓ (bereits implementiert)
- Story 04 (Detailansicht) muss abgeschlossen sein ✓ (bereits implementiert)
- HTMX ist bereits eingebunden ✓
- `parse_form_data` und `parse_rating` in `routes/recipes.rs` können wiederverwendet werden ✓
- `validate_rating` in `models/recipe.rs` kann wiederverwendet werden ✓

---

## Technische Entscheidungen

### CSS-Hover-Effekt für Sterne (ADR)

**Problem:** Bei Hover über Stern 4 sollen die Sterne 1–4 gelb leuchten. In normaler DOM-Reihenfolge
(1, 2, 3, 4, 5) ist "alle vorherigen Geschwister" in CSS nicht direkt möglich (`~` selektiert
nur nachfolgende Geschwister).

**Entscheidung:** Sterne werden in umgekehrter Reihenfolge gerendert (5, 4, 3, 2, 1) und mit
`flex-direction: row-reverse` visuell wieder umgekehrt dargestellt. Dadurch kann CSS mit dem
`~`-Selektor alle "nachfolgenden" (DOM-technisch linkseren) Sterne beim Hover einfärben:
```css
.inline-rating-btn:hover,
.inline-rating-btn:hover ~ .inline-rating-btn {
    color: #f59e0b; /* gelb */
}
```

**Vorteil:** Kein zusätzliches JavaScript nötig. Progressive Enhancement bleibt erhalten.

### HTML-Struktur: Form + HTMX-Overlay

Damit die Inline-Bewertung **ohne JavaScript** funktioniert (K5), werden die Sterne-Buttons
innerhalb eines `<form method="POST" action="/recipes/{{ id }}/rating">` gerendert.
Mit aktivem HTMX wird der Form-Submit durch `hx-post` übernommen und der Response als
HTML-Fragment in `#inline-rating` geswappt (kein Seiten-Reload).
Ohne JavaScript führt der normale Form-Submit zu einem Redirect auf die Detailseite.

---

## Test-Checkliste

- [ ] Unit-Test: `validate_rating` mit Grenzwerten (bereits vorhanden, kein Aufwand)
- [ ] Integrationstest: `update_rating_stores_new_value` – POST `/recipes/:id/rating` speichert Wert
- [ ] Integrationstest: `update_rating_resets_to_none` – leeres Rating löscht Bewertung
- [ ] Integrationstest: `update_rating_returns_html_fragment` – Response enthält `inline-rating`
- [ ] Integrationstest: `update_rating_rejects_invalid_value` – rating=6 → HTTP 400
- [ ] Integrationstest: `update_rating_returns_404_for_unknown_id` – unbekannte ID → HTTP 404
- [ ] E2E-Test: Inline-Bewertung setzen (4 Sterne, Detailansicht aktualisiert sich)
- [ ] E2E-Test: Bewertung ändern (3→5 Sterne, HTMX-Swap sichtbar)
- [ ] E2E-Test: Bewertung zurücksetzen (selber Stern → kein Rating)
- [ ] E2E-Test: Konsistenz Edit-Mode (Inline-Bewertung spiegelt sich im Formular)
- [ ] E2E-Test: Tastatur-Navigation zu den Sterne-Buttons
- [ ] Manueller Test: Touch-Fläche ≥ 44×44px auf Mobilgerät prüfen

---

## Offene Punkte

- Visuelles Signal "klickbar": Der Inline-Bereich soll erkennbar interaktiv sein. Entscheidung:
  Ein dezentes Stift-Icon (SVG, bereits in `icons.html` vorhanden) rechts neben den Sternen,
  das die Editierbarkeit signalisiert – ohne die Sterne selbst zu überladen. Dieses Icon ist
  nur dekorativ (aria-hidden) und erscheint nur wenn HTMX aktiv ist (via `.htmx-settled` oder
  ähnlicher CSS-Klasse). Alternativ reicht auch ein `title`-Attribut auf dem Container.
  → **Entscheidung während der Implementierung treffen**, ggf. in `adrs.md` dokumentieren.
