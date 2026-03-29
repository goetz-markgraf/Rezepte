# Implementierungsplan: Story 22 – Dubletten-Prüfung und Übersicht

## Technische Analyse

### Ausgangslage

Story 21 hat folgende Bausteine bereitgestellt, auf denen Story 22 aufbaut:

- `SimilarRecipe { id, title, rating }` – Struct in `src/models/recipe_db.rs`
- `find_similar_recipes(pool, title, exclude_id)` – LIKE-Suche, min. 3 Zeichen, bis zu 3 Treffer
- `get_all_recipes(pool)` – bereits in `recipe_db.rs` vorhanden, wird aber noch nicht in `models/mod.rs` exportiert
- `DuplicateHintTemplate` – Template für das HTMX-Fragment; wird für die neue Seite nicht wiederverwendet (andere Datenstruktur nötig)

### Kern-Algorithmus für die Dubletten-Übersicht

```
1. Alle Rezepte laden (alphabetisch sortiert via get_all_recipes)
2. Für jedes Rezept R: find_similar_recipes(pool, R.title, Some(R.id)) aufrufen
3. Gefundene Kandidaten → Paare (R.id, Kandidat.id) bilden, wobei immer min(a, b) < max(a, b)
4. Deduplizierung via HashSet<(i64, i64)>
5. Ergebnis: Vec<DublettenPaar>
```

**Wichtig:** `find_similar_recipes` sucht per LIKE `%title%`, d.h. Rezept A findet Rezept B und B findet A.
Die Deduplizierung via geordnetes Paar `(min(id_a, id_b), max(id_a, id_b))` stellt sicher, dass jedes Paar nur einmal erscheint.

**Performance:** Bei N Rezepten entstehen N DB-Anfragen. Bei 100 Rezepten sind das 100 schnelle SQLite-LIKE-Queries – akzeptabel für die Zielgröße. Kein N+1-Problem im kritischen Sinne, da SQLite embedded und ohne Netzwerk-Overhead arbeitet.

### Neue URL

```
GET /recipes/duplicates
```

Diese Route muss **vor** der Route `/recipes/:id` im Router registriert werden (Axum matcht in Reihenfolge und "duplicates" würde sonst als `:id` interpretiert). Der bestehende Router in `src/routes/mod.rs` hat `/recipes/new` und `/recipes/check-duplicate` bereits vor `/recipes/:id` – gleiches Muster verwenden.

---

## Technische Schritte

### Schritt 1: DB-Layer – Funktion `find_all_duplicate_pairs` in `recipe_db.rs`

- [ ] Struct `DublettenPaar` in `recipe_db.rs` definieren:
  ```rust
  pub struct DublettenPaar {
      pub rezept_a: SimilarRecipe,
      pub rezept_b: SimilarRecipe,
  }
  ```
  `SimilarRecipe` hat bereits `id`, `title`, `rating` – das reicht für die Anzeige.

- [ ] Funktion `find_all_duplicate_pairs(pool) -> Result<Vec<DublettenPaar>, sqlx::Error>` implementieren:
  - `get_all_recipes(pool)` aufrufen → `Vec<Recipe>`
  - `HashSet<(i64, i64)>` für Deduplizierung anlegen
  - Für jedes Rezept: `find_similar_recipes(pool, &recipe.title, Some(recipe.id))` aufrufen
  - Für jeden Kandidaten: Paar `(min(recipe.id, kandidat.id), max(recipe.id, kandidat.id))` bilden
  - Wenn Paar noch nicht im HashSet: einfügen und in Ergebnis-Vec aufnehmen
  - Reihenfolge: Paare in der Reihenfolge des ersten Auftretens (natürliche Alphabetreihenfolge der Rezepte)

- [ ] `get_all_recipes` in `models/mod.rs` exportieren (ist derzeit nicht öffentlich re-exportiert)

- [ ] `DublettenPaar` in `models/mod.rs` re-exportieren

- [ ] Unit-Tests in `recipe_db.rs` (TDD):
  - Test: Zwei ähnliche Rezepte → ein Paar
  - Test: Kein ähnliches Rezept → leere Liste
  - Test: Paar erscheint nur einmal (nicht A→B und B→A)
  - Test: Rezept nicht mit sich selbst gepaart
  - Test: Drei Rezepte mit wechselseitiger Ähnlichkeit → korrekte Paare ohne Duplikate

### Schritt 2: Template-Struct `DublettenUebersichtTemplate` in `templates.rs`

- [ ] Struct `DublettenPaarItem` in `templates.rs` für die Darstellung im Template definieren:
  ```rust
  pub struct DublettenPaarItem {
      pub id_a: i64,
      pub titel_a: String,
      pub bewertung_a: Option<i32>,
      pub id_b: i64,
      pub titel_b: String,
      pub bewertung_b: Option<i32>,
  }
  ```
  Bewertung wird als `Option<i32>` übergeben; das Template rendert Sterne oder "nicht bewertet".

- [ ] Hilfsmethoden für `DublettenPaarItem` ergänzen (analog zu `RecipeDetailTemplate`):
  ```rust
  pub fn sterne_a(&self) -> String { ... }  // "★★★" oder ""
  pub fn sterne_b(&self) -> String { ... }
  ```

- [ ] Template-Struct `DublettenUebersichtTemplate` in `templates.rs` definieren:
  ```rust
  #[derive(Template)]
  #[template(path = "recipes/duplicates.html")]
  pub struct DublettenUebersichtTemplate {
      pub paare: Vec<DublettenPaarItem>,
  }
  ```

### Schritt 3: Askama-Template `templates/recipes/duplicates.html`

- [ ] Neue Datei `templates/recipes/duplicates.html` anlegen (erbt von `base.html`)

- [ ] Seitenstruktur:
  - `{% block title %}Mögliche Dubletten – Rezepte{% endblock %}`
  - `<h1>Mögliche Dubletten</h1>`
  - Einleitungstext: "Diese Rezepte haben ähnliche Titel. Prüfe, ob es sich um Duplikate handelt."
  - Leer-Zustand wenn `paare.is_empty()`: `<p class="duplicates-empty">Keine ähnlichen Rezepte gefunden – deine Sammlung ist sauber!</p>`
  - Paare-Liste: `<ul class="duplicates-list">` mit je einem `<li class="duplicate-pair">` pro Paar
  - Jedes Paar enthält zwei Rezept-Links als Cards (nebeneinander auf Desktop, übereinander auf Mobile)
  - Jeder Rezept-Link zeigt: Titel (als `<a href="/recipes/{id}">`), Sternebewertung falls vorhanden

- [ ] Semantische HTML-Struktur für Barrierefreiheit:
  - `<section>` pro Paar mit `aria-label="Mögliches Duplikat"`
  - Rezept-Links mit `aria-label="{titel} – Rezept anzeigen"`

### Schritt 4: Route-Handler `duplicates_handler` in `routes/recipes.rs`

- [ ] Import `DublettenUebersichtTemplate` und `DublettenPaarItem` in `routes/recipes.rs` ergänzen
- [ ] Import `find_all_duplicate_pairs` und `DublettenPaar` in `routes/recipes.rs` ergänzen

- [ ] Handler-Funktion `duplicates_handler` implementieren:
  ```rust
  pub async fn duplicates_handler(
      State(pool): State<Arc<SqlitePool>>,
  ) -> Result<impl IntoResponse, AppError> {
      let paare = find_all_duplicate_pairs(&pool).await?;
      let paar_items = paare.into_iter().map(|p| DublettenPaarItem {
          id_a: p.rezept_a.id,
          titel_a: p.rezept_a.title,
          bewertung_a: p.rezept_a.rating,
          id_b: p.rezept_b.id,
          titel_b: p.rezept_b.title,
          bewertung_b: p.rezept_b.rating,
      }).collect();
      let template = DublettenUebersichtTemplate { paare: paar_items };
      Ok(Html(render_template(template)?))
  }
  ```

### Schritt 5: Router-Registrierung in `routes/mod.rs`

- [ ] Route `.route("/recipes/duplicates", get(recipes::duplicates_handler))` hinzufügen
- [ ] Wichtig: Diese Route **vor** `.route("/recipes/:id", ...)` eintragen (analog zu `/recipes/new` und `/recipes/check-duplicate`)

### Schritt 6: Navigation – Link in `templates/base.html`

- [ ] Link "Dubletten prüfen" in der `<nav class="main-nav">` ergänzen:
  ```html
  <a href="/recipes/duplicates" class="nav-link">Dubletten prüfen</a>
  ```
- [ ] Navigation bleibt damit: "Heute" | "Wochenvorschau" | "Dubletten prüfen"

### Schritt 7: CSS-Styling in `src/static/css/app.css`

- [ ] Neuen Abschnitt `/* === Dubletten-Übersicht (Story 22) === */` am Ende der CSS-Datei hinzufügen

- [ ] CSS-Klassen:
  ```css
  .duplicates-list        /* ul – Liste aller Paare */
  .duplicate-pair         /* li – ein Paar */
  .duplicate-pair-cards   /* div – Wrapper für zwei Cards nebeneinander */
  .duplicate-card         /* div – eine Rezept-Card */
  .duplicate-card a       /* Link-Styling */
  .duplicate-card .stars  /* Sternebewertung */
  .duplicates-empty       /* Leerzustand-Text */
  ```

- [ ] Mobile-First: Cards übereinander auf Mobile (`flex-direction: column`), nebeneinander auf Desktop (`flex-direction: row` ab 600px)

- [ ] Fokus-Indikatoren für alle Links (WCAG 2.1 A: `focus-visible`)

### Schritt 8: Rust-Integrationstests in `tests/recipe_duplicates.rs`

Jeder Test enthält Given/When/Then als deutsche Kommentare.

- [ ] Neue Datei `tests/recipe_duplicates.rs` anlegen

- [ ] Tests (TDD: Tests zuerst schreiben, dann Handler implementieren):
  - `duplicates_page_returns_200` – Leere DB → HTTP 200
  - `duplicates_page_shows_similar_pair` – Zwei ähnliche Rezepte → beide Titel im HTML sichtbar
  - `duplicates_page_shows_empty_message_when_no_duplicates` – Keine ähnlichen Rezepte → "sauber"-Meldung
  - `duplicates_page_links_to_recipe_detail` – Link `href="/recipes/{id}"` vorhanden
  - `duplicates_page_pair_appears_only_once` – Paar A+B erscheint genau einmal (Deduplizierung)
  - `duplicates_page_recipe_not_paired_with_itself` – Rezept A erscheint nicht mit sich selbst

- [ ] In `Cargo.toml` die Integration-Test-Datei registrieren (falls nötig – prüfen ob auto-discovery aktiv)

### Schritt 9: E2E-Tests in `tests/e2e/recipe-duplicates-overview.spec.ts`

Jeder Test enthält Given/When/Then als deutsche Kommentare.

- [ ] Neue Datei `tests/e2e/recipe-duplicates-overview.spec.ts` anlegen

- [ ] Testfälle entsprechend der Story-Akzeptanzkriterien:

  **K1: Seite erreichbar**
  ```
  Given: App läuft
  When: Benutzer navigiert zu /recipes/duplicates
  Then: Seite lädt mit HTTP 200 und zeigt "Mögliche Dubletten" als Überschrift
  ```

  **K2: Dubletten-Paare werden angezeigt**
  ```
  Given: Zwei Rezepte mit ähnlichen Titeln existieren ("Pizza Margherita" und "Margherita Pizza")
  When: Benutzer öffnet /recipes/duplicates
  Then: Beide Rezepte erscheinen auf der Seite als Paar
  And: Jedes Rezept zeigt Titel und ist klickbarer Link
  ```

  **K3: Navigation zu Einzelrezept funktioniert**
  ```
  Given: Ein Dubletten-Paar ist sichtbar
  When: Benutzer klickt auf den Link eines Rezepts
  Then: Detailansicht des Rezepts wird geöffnet
  ```

  **K4: Leerer Zustand**
  ```
  Given: Die Sammlung enthält keine ähnlichen Rezepte (z.B. nur ein Rezept oder alle Titel unähnlich)
  When: Benutzer öffnet /recipes/duplicates
  Then: Meldung "sauber" oder "keine ähnlichen" wird angezeigt
  And: Kein Paar wird angezeigt
  ```

  **K5: Paar erscheint nur einmal**
  ```
  Given: Zwei ähnliche Rezepte "Rezept Alpha" und "Alpha Rezept" existieren
  When: Benutzer öffnet /recipes/duplicates
  Then: Das Paar erscheint genau einmal (nicht zweimal: A→B und B→A)
  ```

  **Navigation-Test: Link in der Nav-Leiste**
  ```
  Given: Startseite ist geöffnet
  When: Benutzer schaut auf die Navigation
  Then: Link "Dubletten prüfen" ist sichtbar und führt zu /recipes/duplicates
  ```

- [ ] Testdaten: Rezepte werden per `createRecipe(page, title)` Hilfsfunktion direkt über das UI angelegt (kein Seed nötig, da Zeitstempel-Strategie wie in Story 21)

### Schritt 10: Qualitätssicherung

- [ ] `cargo build` – keine Fehler
- [ ] `cargo clippy -- -D warnings` – keine Warnungen
- [ ] `cargo fmt --check` – korrekte Formatierung
- [ ] `cargo test` – alle Unit- und Integrationstests grün
- [ ] `npm run test:e2e` – alle E2E-Tests grün
- [ ] Manuelle Prüfung: Link in Navigation sichtbar, Dubletten-Seite lädt, Links zur Detailansicht funktionieren
- [ ] Manuelle Prüfung: Leerer Zustand (frische DB oder Rezepte ohne ähnliche Titel)
- [ ] Manuelle Prüfung: Barrierefreiheit – Tab-Navigation durch alle Links auf der Seite möglich

---

## URL-Struktur

```
GET  /recipes/duplicates  →  Dubletten-Übersichtsseite (vollständige HTML-Seite)
```

Diese Route ergänzt die bestehende Routing-Struktur und ist deeplink-fähig.

---

## Abhängigkeiten

- Story 21 ist abgeschlossen: `find_similar_recipes()` und `SimilarRecipe` existieren in `recipe_db.rs`
- Story 01 (Rezept erstellen) und Story 04 (Rezept-Detailansicht) sind abgeschlossen
- `get_all_recipes()` in `recipe_db.rs` existiert, wird aber noch nicht in `models/mod.rs` re-exportiert → Export ergänzen

---

## Test-Checkliste

- [ ] Unit-Test: `find_all_duplicate_pairs` – Zwei ähnliche Rezepte → ein Paar
- [ ] Unit-Test: `find_all_duplicate_pairs` – Keine ähnlichen → leere Liste
- [ ] Unit-Test: `find_all_duplicate_pairs` – Paar A+B nur einmal (Deduplizierung)
- [ ] Unit-Test: `find_all_duplicate_pairs` – Rezept nicht mit sich selbst gepaart
- [ ] Integrationstest: GET /recipes/duplicates → HTTP 200
- [ ] Integrationstest: Zwei ähnliche Rezepte → beide Titel im HTML
- [ ] Integrationstest: Keine ähnlichen Rezepte → Leerzustand-Meldung im HTML
- [ ] Integrationstest: Links href="/recipes/{id}" im HTML vorhanden
- [ ] Integrationstest: Paar erscheint nur einmal
- [ ] E2E-Test: Seite erreichbar (K1)
- [ ] E2E-Test: Paar wird angezeigt (K2)
- [ ] E2E-Test: Navigation zu Einzelrezept (K3)
- [ ] E2E-Test: Leerer Zustand (K4)
- [ ] E2E-Test: Paar nur einmal sichtbar (K5)
- [ ] E2E-Test: Nav-Link führt zur Seite
- [ ] Manueller Test: Tab-Navigation (Barrierefreiheit)
- [ ] Manueller Test: Mobile-Layout (Cards übereinander)

---

## Offene Punkte

- **Einstiegspunkt:** Nav-Link "Dubletten prüfen" wird in `base.html` ergänzt → keine offene Frage mehr.
- **"Kein Duplikat"-Markierung:** In dieser Story nicht implementiert (würde Datenbankschema-Änderung erfordern). Als Akzeptanzkriterium nicht gefordert → Scope-Out.
- **Löschen-Button im Paar:** Nur Links zur Detailansicht; Löschen erfolgt dort. Entspricht K3 und vermeidet ungewollte Datenverluste.
- **Bewertungsanzeige:** `SimilarRecipe` enthält bereits `rating: Option<i32>`. Das Template kann Sterne darstellen ohne zusätzliche DB-Abfragen.
- **`get_all_recipes` Export:** Muss in `models/mod.rs` ergänzt werden. Prüfen, ob die Funktion bereits `pub` ist (ja, sie ist `pub async fn`) – nur der Re-Export in `mod.rs` fehlt.
