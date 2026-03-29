# Implementierungsplan: Story 20 — "Heute gekocht" Ansicht mit Highlight

## Übersicht

Neue Seite `/heute` mit drei Bereichen: Gestern, Heute (hervorgehoben), Morgen.
Jedes angezeigte Rezept zeigt die Inline-Bewertung (HTMX). Datum-Berechnung serverseitig.
Neuer Nav-Link "Heute" in der Hauptnavigation.

Kein neues Datenbankfeld. Kein neues Migration-Script.
Wiederverwendung von: `get_recipes_current_week`-Pattern (DB-Query), `InlineRatingTemplate` (Rating-Fragment), bestehende `format_date_kurz` / `german_weekday_long` Hilfsfunktionen aus `wochenvorschau.rs`.

---

## Technische Schritte

### Schritt 1: DB-Layer — neue Abfragefunktion `get_recipes_for_date_range`

Die bestehende Funktion `get_recipes_current_week` liefert alle Rezepte zwischen zwei Daten.
Für Story 20 brauchen wir eine ähnliche, aber gezielt für gestern/heute/morgen.

**Strategie:** Die vorhandene SQL-Query ist generisch genug — wir können dieselbe Funktion wiederverwenden
(gestern = `today - 1`, morgen = `today + 1` → Range = gestern bis morgen).

- [ ] In `src/models/recipe_db.rs`: Neue `pub async fn get_recipes_drei_tage` schreiben
  - Parameter: `pool: &SqlitePool, gestern: time::Date, morgen: time::Date`
  - SQL: `WHERE planned_date >= ?1 AND planned_date <= ?2`
  - Rückgabe: `Result<Vec<Recipe>, sqlx::Error>`
  - Sortierung: aufsteigend nach `planned_date`, dann alphabetisch nach Titel
  - Die bestehende `get_recipes_current_week`-Funktion kann als 1:1-Vorlage dienen

- [ ] In `src/models/mod.rs`: `get_recipes_drei_tage` exportieren

- [ ] Unit-Test in `src/models/recipe_db.rs` (im `#[cfg(test)]`-Block):
  - Test: Rezept mit `planned_date = heute` wird zurückgegeben
  - Test: Rezept mit `planned_date = gestern` wird zurückgegeben
  - Test: Rezept mit `planned_date = morgen` wird zurückgegeben
  - Test: Rezept mit `planned_date = heute - 2` erscheint NICHT
  - Test: Rezept ohne `planned_date` erscheint NICHT
  - Test: Mehrere Rezepte für heute werden alle zurückgegeben

---

### Schritt 2: Template-Datenstrukturen in `src/templates.rs`

- [ ] Neues Struct `HeuteRezeptItem`:
  ```rust
  pub struct HeuteRezeptItem {
      pub id: i64,
      pub title: String,
      pub rating: Option<i32>,
  }
  ```

- [ ] Neues Struct `HeuteTagesabschnitt`:
  ```rust
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
  ```

- [ ] Neues Template-Struct `HeuteTemplate`:
  ```rust
  #[derive(Template)]
  #[template(path = "heute.html")]
  pub struct HeuteTemplate {
      /// Alle 3 Tagesabschnitte: gestern, heute, morgen
      pub abschnitte: Vec<HeuteTagesabschnitt>,
      /// Datum-Zeile im Seitenkopf: "Donnerstag, 2. April 2026"
      pub heute_anzeige: String,
  }
  ```

- [ ] Methoden für `HeuteRezeptItem`:
  ```rust
  impl HeuteRezeptItem {
      pub fn rating_is_active(&self, n: i32) -> bool { self.rating == Some(n) }
      pub fn star_filled(&self, n: i32) -> bool { self.rating.unwrap_or(0) >= n }
  }
  ```

---

### Schritt 3: Route-Handler `src/routes/heute.rs` (neue Datei)

- [ ] Neue Datei `src/routes/heute.rs` erstellen

- [ ] Hilfsfunktionen (wiederverwendet aus `wochenvorschau.rs` oder per `pub` exportiert):
  - `format_date_kurz(date: time::Date) -> String` — "2. April"
  - `format_weekday_name(date: time::Date) -> String` — "Donnerstag"
  - `format_heute_anzeige(today: time::Date) -> String` — "Donnerstag, 2. April 2026"

  **Refactoring-Entscheidung:** Da `wochenvorschau.rs` diese Funktionen privat enthält, werden sie in `heute.rs` dupliziert oder in ein gemeinsames Modul `src/routes/date_utils.rs` ausgelagert. Bevorzug: Auslagerung in `date_utils.rs` (vermeidet Duplikation, folgt DRY-Prinzip).

- [ ] Neues Hilfsmodul `src/routes/date_utils.rs`:
  - `pub fn format_date_kurz(date: time::Date) -> String`
  - `pub fn format_weekday_name(date: time::Date) -> String`
  - `pub fn format_heute_anzeige(today: time::Date) -> String`
  - Die bestehenden Konstanten `GERMAN_MONTHS_LONG` und `GERMAN_WEEKDAYS_LONG` werden hierhin verschoben
  - Refactoring: `wochenvorschau.rs` nutzt dann die Funktionen aus `date_utils`

- [ ] Handler `pub async fn heute_handler`:
  ```rust
  pub async fn heute_handler(
      State(pool): State<Arc<SqlitePool>>,
  ) -> Result<Html<String>, AppError>
  ```
  - Berechnet `today`, `gestern = today - 1 Tag`, `morgen = today + 1 Tag`
  - Ruft `get_recipes_drei_tage(&pool, gestern, morgen).await?` auf
  - Baut drei `HeuteTagesabschnitt`-Einträge auf (gestern, heute, morgen)
  - Filtert Rezepte nach Datum auf den jeweiligen Abschnitt
  - Erstellt `HeuteTemplate` und rendert es

- [ ] Unit-Tests in `heute.rs` (im `#[cfg(test)]`-Block):
  - Test: `format_heute_anzeige` liefert korrektes Format (z.B. "Sonntag, 5. April 2026")
  - Test: `format_date_kurz` und `format_weekday_name` (falls in `date_utils.rs`, dort testen)

---

### Schritt 4: Routing — neuen Handler eintragen

- [ ] In `src/routes/mod.rs`:
  - `pub mod heute;` hinzufügen
  - Falls `date_utils.rs` erstellt wurde: `pub mod date_utils;` hinzufügen
  - In `create_router`: Route eintragen:
    ```rust
    .route("/heute", get(heute::heute_handler))
    ```

- [ ] In `src/lib.rs` prüfen, ob neue Module exportiert werden müssen (i.d.R. nicht, da über `routes::create_router` eingebunden)

---

### Schritt 5: Template `templates/heute.html`

- [ ] Neue Datei `templates/heute.html` erstellen:
  - Extends `base.html`
  - Block title: "Heute gekocht"
  - Seitenkopf mit Datum-Anzeige (`heute_anzeige`)
  - Drei `<section>`-Abschnitte für Gestern, Heute, Morgen:
    - Abschnitts-Header: Label + Wochentag + Datum
    - Heute-Abschnitt: CSS-Klasse `tagesabschnitt-heute` + `aria-label="Hervorgehoben: heute"`
    - "Nichts geplant"-Meldung wenn `rezepte.is_empty()`
    - Für jedes Rezept: Titel als Link + Inline-Rating
  - Freundliche Meldung wenn kein Rezept für heute: "Für heute noch kein Rezept geplant"
  - Link zurück zur Wochenvorschau und zur Rezeptliste

- [ ] Inline-Rating im Template für `HeuteRezeptItem`:
  Die Sterne-Buttons (`hx-post`, `hx-target`, `hx-swap`) verweisen auf `/recipes/{{ rezept.id }}/rating`.
  HTMX `hx-target` muss eine eindeutige ID pro Rezept haben: `#inline-rating-{{ rezept.id }}`.
  Das zurückgelieferte Fragment aus `POST /recipes/:id/rating` hat feste ID `#inline-rating` — dies muss
  per `id="inline-rating-{{ rezept.id }}"` angepasst werden **oder** wir nutzen `hx-swap="outerHTML"`
  mit einem eindeutigen Container.

  **Lösung:** Das `InlineRatingTemplate` rendert `id="inline-rating"` fix. Da auf der Heute-Seite mehrere
  Rezepte gleichzeitig sichtbar sind, brauchen wir eindeutige IDs pro Rezept.

  **Strategie:** Das Template in `heute.html` rendert den Inline-Rating-Block direkt (ohne `_inline_rating.html`
  einzubinden), mit `id="inline-rating-{{ rezept.id }}"` statt `id="inline-rating"`.
  `hx-target="#inline-rating-{{ rezept.id }}"` — HTMX tauscht dann den richtigen Container aus.

  **Wichtig:** `POST /recipes/:id/rating` antwortet mit `InlineRatingTemplate`, das `id="inline-rating"`
  rendert. Auf der Heute-Seite muss der Container ebenfalls `id="inline-rating-{{ id }}"` haben, damit
  `hx-swap="outerHTML"` funktioniert. Das bedeutet: Entweder wird `InlineRatingTemplate` für die Heute-Seite
  nicht verwendet (eigene Fragment-Antwort), oder wir akzeptieren, dass HTMX `outerHTML` den Container
  korrekt ersetzt, wenn `hx-target` auf `#inline-rating-{{ id }}` zeigt und die Response `id="inline-rating"`
  enthält (die ID ändert sich nach dem Swap — kein Problem, da jede Interaktion genau einen Button betrifft).

  **Gewählte Lösung:** Inline-Rating in `heute.html` inline ausschreiben mit `id="inline-rating-{{ rezept.id }}"`.
  Der HTMX-POST auf `/recipes/:id/rating` liefert `InlineRatingTemplate` zurück (mit `id="inline-rating"`).
  Nach dem Swap hat der Container `id="inline-rating"` — das ist in Ordnung, da nur eine Bewertungsinteraktion
  gleichzeitig stattfindet. Bei mehreren Rezepten kann es zu Namenskollisionen kommen.

  **Bessere Lösung:** Neuer Endpunkt `GET /recipes/:id/rating-fragment` oder angepasster Template-Path.
  Alternativ: Template-Parameter `context_id` für eindeutige IDs.

  **Finale Entscheidung:** Wir erstellen eine neue Variante des InlineRating-Fragments, die eine dynamische
  ID akzeptiert. Dafür fügen wir `InlineRatingWithContextTemplate` in `templates.rs` hinzu:
  ```rust
  #[derive(Template)]
  #[template(path = "recipes/_inline_rating_ctx.html")]
  pub struct InlineRatingWithContextTemplate {
      pub id: i64,
      pub rating: Option<i32>,
      pub context_id: String,  // z.B. "inline-rating-42"
  }
  ```
  Und einen neuen Endpunkt `POST /heute/recipes/:id/rating` der auf die Heute-Seite kontextualisiert antwortet.

  **Vereinfachte Finale Entscheidung (KISS):** Die Heute-Seite rendert für jedes Rezept den Inline-Rating-Block
  mit `id="inline-rating-{{ rezept.id }}"`. Der bestehende Endpunkt `POST /recipes/:id/rating` gibt weiterhin
  `InlineRatingTemplate` mit `id="inline-rating"` zurück. Auf der Heute-Seite verwenden wir
  `hx-target` ohne feste ID, sondern `hx-target="closest .inline-rating-wrapper"` mit `hx-swap="innerHTML"`.
  Das Fragment-HTML enthält dann nur den inneren Inhalt (Buttons), kein äußeres `<div>`.

  **Pragmatische Entscheidung (wie in Story 17):** Der Endpunkt `/recipes/:id/rating` gibt `id="inline-rating"`
  zurück. Für die Heute-Seite nutzen wir denselben Endpunkt, aber mit einem wrapper-`<div>` pro Rezept:
  `id="inline-rating-{{ rezept.id }}"`. Das HTMX `hx-target="#inline-rating-{{ rezept.id }}"` + `hx-swap="outerHTML"`
  ersetzt diesen Container mit dem Fragment (das `id="inline-rating"` hat). Nach dem Swap haben wir
  `id="inline-rating"` statt `id="inline-rating-{{ rezept.id }}"` — eine erneute Bewertung desselben Rezepts
  funktioniert, weil die Buttons auf `#inline-rating-{{ rezept.id }}` zeigen (das nach dem ersten Swap
  nicht mehr existiert). Das ist ein Bug.

  **Korrekte Lösung:** Neues Template `_inline_rating_heute.html` und neuer Handler
  `POST /heute/recipes/:id/rating` der das neue Template nutzt (mit dynamischer ID per `id`-Parameter).

- [ ] Neues Template `templates/recipes/_inline_rating_heute.html`:
  Identisch zu `_inline_rating.html`, aber `id="inline-rating-{{ id }}"` statt `id="inline-rating"`.
  Alle `hx-target="#inline-rating-{{ id }}"` statt `#inline-rating`.

- [ ] Neues Template-Struct `InlineRatingHeuteTemplate` in `src/templates.rs`:
  ```rust
  #[derive(Template)]
  #[template(path = "recipes/_inline_rating_heute.html")]
  pub struct InlineRatingHeuteTemplate {
      pub id: i64,
      pub rating: Option<i32>,
  }
  ```
  Mit denselben Methoden wie `InlineRatingTemplate`.

- [ ] Neuer Endpunkt `POST /heute/recipes/:id/rating` in `src/routes/heute.rs`:
  - Gleiche Logik wie `update_recipe_rating_handler`
  - Antwortet mit `InlineRatingHeuteTemplate` statt `InlineRatingTemplate`

---

### Schritt 6: Navigation anpassen (`templates/base.html`)

- [ ] In `templates/base.html` den Nav-Link "Heute" hinzufügen:
  ```html
  <a href="/heute" class="nav-link">Heute</a>
  ```
  Position: vor oder nach dem "Wochenvorschau"-Link (konsistent mit bestehendem Stil).

---

### Schritt 7: CSS-Styling (`src/static/css/app.css`)

- [ ] CSS-Klassen für die Heute-Seite ergänzen:
  - `.tagesabschnitt` — Basis-Stil für Abschnitte (Gestern, Heute, Morgen)
  - `.tagesabschnitt-heute` — Hervorhebung des Heute-Abschnitts (farbiger Hintergrund oder Rahmen)
  - `.tagesabschnitt-header` — Abschnitts-Kopfzeile mit Label + Datum
  - `.heute-label` — Großes "Heute"-Badge (analog zu `.heute-badge` in `wochenvorschau.html`)
  - `.heute-rezept-liste` — Liste der Rezepte im Abschnitt
  - `.heute-rezept-item` — Einzelner Rezept-Eintrag
  - `.nichts-geplant-heute` — Freundliche Meldung wenn kein Rezept
  - `.heute-seite-header` — Seitenkopf mit Datum-Anzeige

- [ ] Responsive Design: Mobile-First (Primär-Usecase Handy beim Essen)
  - Große Schrift für Rezepttitel (min. 18px)
  - Sterne-Buttons: Mindestgröße 44px (WCAG Touch-Target)
  - Heute-Bereich: Visuell klar abgesetzt (nicht nur über Farbe — zusätzlich "Heute"-Label + ARIA)

---

### Schritt 8: Rust-Integrationstests (`tests/heute.rs`)

- [ ] Neue Datei `tests/heute.rs` erstellen (analog zu `tests/wochenvorschau.rs`)

- [ ] Hilfsfunktionen:
  - `setup_test_app()` — standard Muster aus anderen Tests
  - `get_body(app, uri)` — standard Muster
  - `create_recipe_with_date(app, title, categories, planned_date)` — standard Muster
  - `date_in_days(n)` — berechnet Datum +/- n Tage vom heute (für Tests)

- [ ] Tests:

  - `heute_returns_200` — GET /heute liefert HTTP 200

  - `heute_zeigt_rezept_fuer_heute`
    ```
    // Given: Rezept mit planned_date = heute
    // When: GET /heute
    // Then: Rezept im Body sichtbar
    ```

  - `heute_zeigt_rezept_fuer_gestern`
    ```
    // Given: Rezept mit planned_date = gestern
    // When: GET /heute
    // Then: Rezept im Body sichtbar
    ```

  - `heute_zeigt_rezept_fuer_morgen`
    ```
    // Given: Rezept mit planned_date = morgen
    // When: GET /heute
    // Then: Rezept im Body sichtbar
    ```

  - `heute_zeigt_keine_rezepte_von_vorgestern`
    ```
    // Given: Rezept mit planned_date = heute - 2
    // When: GET /heute
    // Then: Rezept NICHT im Body
    ```

  - `heute_zeigt_freundliche_meldung_wenn_kein_rezept_fuer_heute`
    ```
    // Given: Kein Rezept mit planned_date = heute (aber evtl. morgen oder gestern)
    // When: GET /heute
    // Then: Body enthält "Für heute noch kein Rezept geplant"
    ```

  - `heute_zeigt_mehrere_rezepte_fuer_heute`
    ```
    // Given: Zwei Rezepte mit planned_date = heute
    // When: GET /heute
    // Then: Beide Rezepte im Body sichtbar
    ```

  - `heute_hat_link_zur_detailansicht`
    ```
    // Given: Rezept mit planned_date = heute
    // When: GET /heute
    // Then: Body enthält /recipes/{id} Link
    ```

  - `heute_hat_css_klasse_tagesabschnitt_heute`
    ```
    // When: GET /heute
    // Then: Body enthält CSS-Klasse "tagesabschnitt-heute"
    ```

  - `heute_rating_post_gibt_200_zurueck`
    ```
    // Given: Rezept mit planned_date = heute
    // When: POST /heute/recipes/:id/rating mit rating=4
    // Then: HTTP 200, Body enthält "inline-rating"
    ```

  - `heute_rating_post_speichert_bewertung`
    ```
    // Given: Rezept ohne Bewertung mit planned_date = heute
    // When: POST /heute/recipes/:id/rating mit rating=5
    // Then: GET /heute zeigt 5 Sterne für das Rezept
    ```

  - `heute_rating_post_gibt_404_fuer_unbekannte_id`
    ```
    // When: POST /heute/recipes/99999/rating mit rating=4
    // Then: HTTP 404
    ```

  - `heute_rating_post_gibt_400_fuer_ungueltige_bewertung`
    ```
    // When: POST /heute/recipes/:id/rating mit rating=6
    // Then: HTTP 400
    ```

---

### Schritt 9: E2E-Tests (`tests/e2e/heute.spec.ts`)

- [ ] Neue Datei `tests/e2e/heute.spec.ts` erstellen

- [ ] Helper-Funktionen:
  - `createRecipeWithDate(page, title, categories, plannedDate)` — analog zu `wochenvorschau.spec.ts`
  - `dateInDays(n)` — berechnet Datum +/- n Tage (client-seitig, als String im deutschen Format)

- [ ] Tests (einem Akzeptanzkriterium pro Test):

  **K1: Seite erreichbar und in Navigation verlinkt**
  ```
  // Given: App läuft
  // When: Benutzer öffnet /
  // Then: Nav-Link "Heute" sichtbar und klickbar → /heute
  // And: Seite lädt ohne Fehler
  ```

  **K2: Heutiges Gericht ist hervorgehoben**
  ```
  // Given: "Spaghetti Bolognese" mit planned_date = heute
  // When: Benutzer öffnet /heute
  // Then: "Spaghetti Bolognese" erscheint im hervorgehobenen Heute-Bereich
  // And: CSS-Klasse "tagesabschnitt-heute" ist am Abschnitt gesetzt
  // And: "Heute"-Label ist sichtbar
  ```

  **K3: Gestern und morgen werden angezeigt**
  ```
  // Given: "Thai-Curry" mit planned_date = gestern
  // And: "Pfannkuchen" mit planned_date = morgen
  // And: "Spaghetti" mit planned_date = heute
  // When: Benutzer öffnet /heute
  // Then: Alle drei Rezepte sichtbar in den richtigen Abschnitten
  ```

  **K4: Freundliche Meldung wenn kein Rezept für heute**
  ```
  // Given: Kein Rezept mit planned_date = heute
  // When: Benutzer öffnet /heute
  // Then: Meldung "Für heute noch kein Rezept geplant" sichtbar
  ```

  **K5: Inline-Bewertung direkt auf der Seite**
  ```
  // Given: "Spaghetti Bolognese" mit planned_date = heute, ohne Bewertung
  // When: Benutzer öffnet /heute
  // And: Klickt auf 5-Sterne-Button
  // Then: Bewertung gespeichert und angezeigt (ohne Seitenreload)
  // And: Sterne-Anzeige aktualisiert sich via HTMX
  ```

  **K6: Link zur Detailansicht**
  ```
  // Given: Rezept mit planned_date = heute
  // When: Benutzer öffnet /heute
  // And: Klickt auf Rezepttitel
  // Then: Navigiert zur Detailansicht /recipes/:id
  ```

  **K7 (implizit): DeepLink — URL /heute direkt aufrufbar**
  ```
  // When: Benutzer ruft /heute direkt auf (ohne Navigation)
  // Then: Seite lädt korrekt
  ```

  **Mehrere Rezepte für heute (Edge Case)**
  ```
  // Given: "Spaghetti" und "Salat" haben beide planned_date = heute
  // When: Benutzer öffnet /heute
  // Then: Beide Rezepte im Heute-Bereich sichtbar
  ```

---

### Schritt 10: Qualitätschecks und DoD

- [ ] `cargo fmt` — Code formatieren
- [ ] `cargo clippy -- -D warnings` — keine Warnings
- [ ] `cargo build` — kompiliert ohne Fehler
- [ ] `cargo test` — alle Tests grün
- [ ] `npm run test:e2e` — alle E2E-Tests grün
- [ ] Kein ungenutzter Code, keine Panics/unwraps im Produktivcode
- [ ] Öffentliche Funktionen und Structs haben Doc-Kommentare (`///`)
- [ ] Alle Akzeptanzkriterien aus story.md manuell geprüft

---

## URL-Struktur

```
GET   /heute                       → Heute-gekocht-Ansicht (gestern/heute/morgen)
POST  /heute/recipes/:id/rating    → Inline-Bewertung speichern, liefert Rating-Fragment mit kontextueller ID
```

---

## Abhängigkeiten

- `planned_date`-Feld in der DB (Story 28 / bereits implementiert)
- `POST /recipes/:id/rating`-Endpunkt (Story 17 / bereits implementiert)
- Wochenvorschau-Implementierung als Referenz (Story 18/19 / bereits implementiert)
- Datum-Hilfsfunktionen aus `wochenvorschau.rs` → werden nach `date_utils.rs` refactored

---

## Test-Checkliste

- [ ] Unit-Test: `get_recipes_drei_tage` — Rezept heute wird zurückgegeben
- [ ] Unit-Test: `get_recipes_drei_tage` — Rezept gestern wird zurückgegeben
- [ ] Unit-Test: `get_recipes_drei_tage` — Rezept morgen wird zurückgegeben
- [ ] Unit-Test: `get_recipes_drei_tage` — Rezept vorgestern erscheint NICHT
- [ ] Unit-Test: `get_recipes_drei_tage` — Rezept ohne Datum erscheint NICHT
- [ ] Unit-Test: `format_heute_anzeige` — korrektes Format
- [ ] Integration: GET /heute → HTTP 200
- [ ] Integration: Rezept für heute wird angezeigt
- [ ] Integration: Rezept für gestern wird angezeigt
- [ ] Integration: Rezept für morgen wird angezeigt
- [ ] Integration: Kein Rezept für heute → freundliche Meldung
- [ ] Integration: Mehrere Rezepte für heute werden alle angezeigt
- [ ] Integration: Rezept von vorgestern erscheint nicht
- [ ] Integration: POST /heute/recipes/:id/rating → HTTP 200, Rating-Fragment
- [ ] Integration: POST /heute/recipes/:id/rating mit rating=6 → HTTP 400
- [ ] Integration: POST /heute/recipes/:id/rating für unbekannte ID → HTTP 404
- [ ] E2E: Nav-Link "Heute" vorhanden und führt zu /heute
- [ ] E2E: Heute-Abschnitt ist hervorgehoben (CSS-Klasse)
- [ ] E2E: Gestern/Heute/Morgen korrekt dargestellt
- [ ] E2E: Freundliche Meldung wenn kein Rezept für heute
- [ ] E2E: Inline-Bewertung per HTMX ohne Seitenreload
- [ ] E2E: Rezepttitel ist Link zur Detailansicht
- [ ] E2E: Mehrere Rezepte für heute
- [ ] E2E: DeepLink /heute direkt aufrufbar

---

## Offene Punkte

- Datum-Hilfsfunktionen (`format_date_kurz`, `format_weekday_name`) sind in `wochenvorschau.rs` privat.
  Refactoring nach `src/routes/date_utils.rs` ist im Plan enthalten, aber falls Scope zu groß: Funktionen
  in `heute.rs` duplizieren und als technische Schuld vermerken.

- `InlineRatingHeuteTemplate` und `_inline_rating_heute.html` sind neue Artefakte für die eindeutigen IDs
  auf der Heute-Seite. Falls zukünftig weitere Seiten Inline-Rating mit dynamischen IDs brauchen,
  sollte ein generisches Template mit `context_id`-Parameter erstellt werden (dann ADR erstellen).
