# Implementierungsplan: Story 23 – Rezepte mergen (Duplikate zusammenführen)

## Technische Analyse

### Bestehende Architektur (relevant für diese Story)

**Datenmodell `Recipe` (src/models/recipe.rs):**
- Felder: `id`, `title`, `categories` (JSON-String), `ingredients`, `instructions`, `planned_date`, `created_at`, `updated_at`, `rating`
- `categories` ist ein JSON-String in der DB, wird via `categories_vec()` zur `Vec<String>`
- Kein neues Schema nötig: UPDATE + DELETE in einer Transaktion

**Story-22-Einstiegspunkt (templates/recipes/duplicates.html):**
- Jedes `.duplicate-pair` zeigt zwei Karten mit Links zu den Detail-Seiten
- Noch kein "Mergen"-Button – muss in diesem Schritt ergänzt werden
- Template nutzt `DublettenPaarItem { id_a, titel_a, bewertung_a, id_b, titel_b, bewertung_b }`

**Routing (src/routes/mod.rs):**
- Neue Routen müssen vor `/recipes/:id` registriert werden (statische Pfade zuerst)
- Muster für statische Unterseiten: `GET /recipes/duplicates` → bereits vorhanden

**Handler-Muster (src/routes/recipes.rs):**
- Query-Parameter via `#[derive(Deserialize)] struct XyzQuery`
- Form-Body via `axum::extract::RawForm(body)` + `parse_form_data()`
- Template-Rendering via `render_template()`
- Redirect nach Erfolg via `Redirect::to()`
- Fehler via `AppError::NotFound` / `AppError::BadRequest`

**Templates:**
- Alle Templates erben von `base.html` via `{% extends "base.html" %}`
- Icons über `{% import "components/icons.html" as icons %}`
- HTMX-Attribute direkt im HTML

---

## URL-Struktur

```
GET  /recipes/merge?source=ID&target=ID  →  Merge-Ansicht (beide Rezepte vergleichen + Felder wählen)
POST /recipes/merge                      →  Merge durchführen (atomarer DB-Vorgang)
```

Die Query-Parameter `source` und `target` definieren, welches Rezept gelöscht (source) und welches
erhalten bleibt (target). Auf der Merge-Seite kann der Nutzer diese Zuweisung tauschen.

---

## Technische Schritte

### Schritt 1: DB-Layer – `merge_recipes` Funktion in `recipe_db.rs`

**Ziel:** Atomare Transaktion, die Ziel-Rezept aktualisiert und Quelle löscht.

- [ ] Funktion `merge_recipes(pool, source_id, target_id, merged_data: &UpdateRecipe) -> Result<(), sqlx::Error>` implementieren
  - SQLite-Transaktion starten (`pool.begin().await?`)
  - Beide IDs vorab validieren: `SELECT id FROM recipes WHERE id IN (?, ?)` → bei 0 oder 1 Treffer → `RowNotFound`
  - UPDATE: Ziel-Rezept mit `merged_data` aktualisieren (gleiche Felder wie `update_recipe`)
  - DELETE: Quell-Rezept löschen (`DELETE FROM recipes WHERE id = ?`)
  - Transaktion committen (`tx.commit().await?`)
  - Bei jedem Fehler: Transaktion wird automatisch via Drop zurückgerollt
- [ ] Unit-Test in `src/models/recipe_db.rs` (eingebettet als `#[cfg(test)]`):
  - Test: Merge erfolgreich – Ziel aktualisiert, Quelle gelöscht
  - Test: Merge mit ungültiger source_id → `RowNotFound`
  - Test: Merge mit ungültiger target_id → `RowNotFound`
  - Test: Merge ist atomar – bei simuliertem Fehler bleibt DB-Zustand unverändert

### Schritt 2: Modell – `MergeFormData` Struct + Merge-Logik in `recipe.rs` / `recipe_db.rs`

**Ziel:** Datenstruktur für das Merge-Formular und die Entscheidungslogik welche Felder aus welchem Rezept übernommen werden.

- [ ] `MergeFormData` Struct in `src/models/recipe.rs` definieren:
  ```rust
  pub struct MergeFormData {
      pub target_id: i64,   // bleibt erhalten
      pub source_id: i64,   // wird gelöscht
      pub title: String,           // aus Formular: "source" oder "target"
      pub categories: Vec<String>, // aus Formular
      pub ingredients: Option<String>,
      pub instructions: Option<String>,
      pub planned_date_input: Option<String>,
      pub rating: Option<i32>,
  }
  ```
- [ ] Hilfsfunktion `determine_merge_target(recipe_a: &Recipe, recipe_b: &Recipe) -> (i64, i64)`:
  - Bestimmt Vorschlag für `(target_id, source_id)` basierend auf "mehr Inhalt"
  - Priorisierung: Hat Bewertung → Ziel; hat mehr Felder ausgefüllt → Ziel; neueres `updated_at` → Ziel; sonst: kleinere ID
  - Gibt `(target_id, source_id)` zurück

### Schritt 3: Template-Datenstrukturen in `templates.rs`

**Ziel:** Alle Structs für das Merge-Template definieren.

- [ ] `MergeRezeptInfo` Struct (für die Anzeige eines Rezepts auf der Merge-Seite):
  ```rust
  pub struct MergeRezeptInfo {
      pub id: i64,
      pub title: String,
      pub categories: Vec<String>,
      pub ingredients: Option<String>,
      pub instructions: Option<String>,
      pub rating: Option<i32>,
      pub planned_date: Option<String>,  // formatiert: "5. März 2025" oder None
      pub created_at: String,            // formatiert: "27.03.2026"
      pub updated_at: String,
  }
  ```
  - Methode `sterne()` für Sterndarstellung (wie `DublettenPaarItem`)
  - Methode `hat_inhalt()` → true wenn irgendein Feld ausgefüllt ist
- [ ] `MergeTemplate` Struct:
  ```rust
  #[derive(Template)]
  #[template(path = "recipes/merge.html")]
  pub struct MergeTemplate {
      pub rezept_a: MergeRezeptInfo,   // immer: source (wird gelöscht)
      pub rezept_b: MergeRezeptInfo,   // immer: target (bleibt erhalten)
      pub source_id: i64,
      pub target_id: i64,
      pub fehler: Vec<String>,         // Validierungsfehler beim POST
  }
  ```
  - Methode `hat_konflikt_titel()` → true wenn beide Titel verschieden
  - Methode `hat_konflikt_categories()` → true wenn Categories verschieden
  - Methode `hat_konflikt_ingredients()` → true wenn beide Felder befüllt
  - Methode `hat_konflikt_instructions()` → true wenn beide Felder befüllt
  - Methode `hat_konflikt_rating()` → true wenn beide Bewertungen gesetzt
  - Methode `hat_konflikt_planned_date()` → true wenn beide Daten gesetzt
- [ ] Template-Structs in `templates.rs` exportieren

### Schritt 4: Handler in `recipes.rs`

**Ziel:** GET- und POST-Handler für die Merge-Seite.

#### 4a: Query-Struct + GET-Handler

- [ ] `MergeQuery` Struct definieren:
  ```rust
  #[derive(Deserialize)]
  pub struct MergeQuery {
      pub source: Option<i64>,
      pub target: Option<i64>,
  }
  ```
- [ ] `merge_form_handler` implementieren:
  - `source` und `target` aus Query-Parametern lesen
  - Bei fehlendem/ungültigem Parameter: `AppError::BadRequest("source und target Parameter erforderlich")`
  - Beide Rezepte aus DB laden via `get_recipe_by_id`; bei `None`: `AppError::NotFound`
  - `determine_merge_target` aufrufen um Vorschlag zu ermitteln (kann durch URL überschrieben sein)
  - `MergeTemplate` befüllen und rendern
  - Datum-Formatierung via `format_planned_date_long` (wie in `show_recipe`)

#### 4b: POST-Handler

- [ ] `merge_handler` implementieren:
  - Form-Body via `RawForm` + `parse_form_data()` parsen
  - `source_id` und `target_id` aus Hidden-Feldern lesen (als `i64`)
  - Feld-Auswahlen lesen: `title_source`, `categories_source`, `ingredients_source`, etc. (Radio-Button-Werte: `"a"` oder `"b"`)
  - Konkrete Feldwerte basierend auf Auswahl zusammenführen → `UpdateRecipe` Struct bauen
  - Validierung via `recipe.validate()`; bei Fehler: Merge-Seite mit Fehlern rendern (erneut beide Rezepte laden)
  - `merge_recipes(&pool, source_id, target_id, &recipe)` aufrufen
  - Bei Erfolg: `Redirect::to(&format!("/recipes/{}?success=1", target_id))`
  - Bei DB-Fehler: `AppError::Database`

### Schritt 5: Template `templates/recipes/merge.html`

**Ziel:** Vollständige Merge-Ansicht mit Feldvergleich und Auswahl.

- [ ] Template erstellen `templates/recipes/merge.html`:
  - Erbt von `base.html`
  - Seitentitel: "Rezepte zusammenführen"
  - Erklärender Hinweis-Text (wie in Story spezifiziert)
  - Fehleranzeige wenn `fehler` nicht leer
  - Hidden-Felder: `source_id`, `target_id`
  - **Nebeneinander-Layout** der beiden Rezepte (CSS-Grid, auf Mobil untereinander)
  - **Pro Feld-Zeile:**
    - Wenn kein Konflikt (nur ein Rezept hat Inhalt): Info-Zeile, kein Radio-Button
    - Wenn Konflikt (beide Felder befüllt): Radio-Gruppe mit Auswahl Rezept A / Rezept B
    - Felder: Titel, Kategorien, Zutaten, Anleitung, Bewertung, Datum
  - **Buttons:**
    - Primär: `<button type="submit">Zusammenführen</button>`
    - Sekundär: `<a href="/recipes/duplicates" class="btn-secondary">Abbrechen</a>`
  - Form-Action: `POST /recipes/merge`
  - Vollständige Tastatur-Navigation (alle Labels korrekt verknüpft)

### Schritt 6: Merge-Button in `duplicates.html` hinzufügen

**Ziel:** Einstiegspunkt aus der Dubletten-Übersicht (K1).

- [ ] In `templates/recipes/duplicates.html` pro Paar einen "Mergen"-Link hinzufügen:
  ```html
  <a href="/recipes/merge?source={{ paar.id_a }}&target={{ paar.id_b }}" class="btn-primary">
      Mergen
  </a>
  ```
  - Link erscheint unterhalb der Paar-Karten, innerhalb des `<section>`-Elements
  - `aria-label="Rezepte {{ paar.titel_a }} und {{ paar.titel_b }} zusammenführen"`
  - Kein HTMX nötig – normaler Link

### Schritt 7: Routing in `mod.rs`

**Ziel:** Neue Routen registrieren.

- [ ] In `src/routes/mod.rs` zwei Routen hinzufügen:
  ```rust
  .route("/recipes/merge", get(recipes::merge_form_handler))
  .route("/recipes/merge", post(recipes::merge_handler))
  ```
  - Beide Routen **vor** `.route("/recipes/:id", ...)` registrieren (statisch vor dynamisch)
  - Alternativ via `routing::get(...).post(...)` auf einem einzigen `.route()` Aufruf

### Schritt 8: Export in `models/mod.rs`

**Ziel:** Neue Funktion öffentlich machen.

- [ ] `merge_recipes` in `src/models/mod.rs` re-exportieren:
  ```rust
  pub use recipe_db::{..., merge_recipes};
  ```
- [ ] Import in `src/routes/recipes.rs` ergänzen

### Schritt 9: CSS-Styling in `src/static/css/app.css`

**Ziel:** Merge-Seite ist responsiv und visuell klar strukturiert.

- [ ] CSS-Klassen ergänzen:
  - `.merge-layout` – CSS-Grid, 2 Spalten auf Desktop, 1 Spalte auf Mobil
  - `.merge-recipe-card` – Karte für jedes Rezept (ähnlich `.duplicate-card`)
  - `.merge-field-row` – Zeile pro Feld (label + Inhalt + ggf. Radio-Buttons)
  - `.merge-field-label` – Label für das Feld (Fett)
  - `.merge-field-value` – Feldinhalt (pre für mehrzeiligen Text)
  - `.merge-conflict-row` – Hervorhebung wenn Konflikt vorhanden
  - `.merge-auto-row` – dezente Darstellung bei automatischer Übernahme
  - `.merge-actions` – Button-Bereich (wie `.actions` in `detail.html`)
  - Responsive Breakpoint: bei < 768px auf einspaltig (wie `.duplicates-list`)

### Schritt 10: Rust-Integrationstests in `tests/recipe_merge.rs`

**Ziel:** HTTP-Ebene testen (Happy Path + Fehlerfälle).

- [ ] Datei `tests/recipe_merge.rs` erstellen:
  - Hilfsfunktionen `setup_test_app()`, `create_recipe_with_data(app, title, rating, ingredients, instructions)` → gibt `i64` zurück
  - **Test 1: GET /recipes/merge zeigt 200 bei gültigen IDs**
    ```rust
    // Given: Zwei Rezepte existieren
    // When: GET /recipes/merge?source=1&target=2
    // Then: HTTP 200, Body enthält beide Titel
    ```
  - **Test 2: GET /recipes/merge gibt 400 bei fehlendem source**
    ```rust
    // Given: App läuft
    // When: GET /recipes/merge?target=1 (ohne source)
    // Then: HTTP 400
    ```
  - **Test 3: GET /recipes/merge gibt 400 bei fehlendem target**
  - **Test 4: GET /recipes/merge gibt 404 wenn Rezept nicht existiert**
    ```rust
    // Given: Nur ein Rezept mit bekannter ID
    // When: GET /recipes/merge?source=1&target=99999
    // Then: HTTP 404
    ```
  - **Test 5: POST /recipes/merge führt Merge durch und redirectet**
    ```rust
    // Given: Zwei Rezepte A (Titel "Pizza A") und B (Titel "Pizza B", mit Zutaten)
    // When: POST /recipes/merge mit source=A, target=B, title_from=b, ingredients_from=b, categories=["Mittagessen"]
    // Then: HTTP 302, Location: /recipes/{target_id}?success=1
    //   And: GET /recipes/{target_id} zeigt Titel aus B, Zutaten aus B
    //   And: GET /recipes/{source_id} → HTTP 404
    ```
  - **Test 6: POST /recipes/merge – Quelle existiert nicht → 404**
  - **Test 7: POST /recipes/merge – Ziel existiert nicht → 404**
  - **Test 8: POST /recipes/merge – leerer Titel → 400 (Validierungsfehler)**
  - **Test 9: Merge ist atomar (Transaktion)**
    - Implizit durch Test 5: nach erfolgreichem Merge existiert source nicht mehr

### Schritt 11: E2E-Tests in `tests/e2e/recipe-merge.spec.ts`

**Ziel:** Alle Akzeptanzkriterien aus der Story per E2E-Test abgedeckt.

- [ ] Datei `tests/e2e/recipe-merge.spec.ts` erstellen:
  - Hilfsfunktion `createRecipeWithDetails(page, {title, rating, ingredients, instructions, categories})` → gibt ID zurück

  - **Test K1: Merge-Button auf Dubletten-Übersicht**
    ```typescript
    // Given: Zwei ähnliche Rezepte "PizzaTS" und "PizzaTSMargherita" existieren
    // When: Benutzer öffnet /recipes/duplicates
    // Then: Ein "Mergen"-Link für dieses Paar ist sichtbar
    // And: Link führt zu /recipes/merge?source=...&target=...
    ```

  - **Test K2: Merge-Seite zeigt beide Rezepte vollständig**
    ```typescript
    // Given: Rezept A mit Bewertung 5, Rezept B mit Zutaten
    // When: Benutzer öffnet /recipes/merge?source=idA&target=idB
    // Then: Seite lädt (h1 "zusammenführen")
    // And: Titel beider Rezepte sind sichtbar
    // And: Bewertung aus Rezept A ist sichtbar
    // And: Zutaten aus Rezept B sind sichtbar
    ```

  - **Test K3: Nutzer wählt Ziel-Rezept (target/source tauschbar)**
    ```typescript
    // Given: Merge-Seite ist geöffnet
    // Then: source_id und target_id sind als Hidden-Felder im Form vorhanden
    // And: Beide Rezept-Spalten sind klar beschriftet (welche bleibt, welche wird gelöscht)
    ```

  - **Test K4: Feldauswahl bei Konflikt**
    ```typescript
    // Given: Beide Rezepte haben unterschiedliche Titel
    // When: Merge-Seite wird geöffnet
    // Then: Radio-Buttons für Titel-Auswahl sind sichtbar
    // And: Nutzer kann Rezept A oder B für den Titel wählen
    ```

  - **Test K4b: Automatische Übernahme bei einseitigem Inhalt**
    ```typescript
    // Given: Rezept A hat Zutaten, Rezept B hat keine
    // When: Merge-Seite wird geöffnet
    // Then: Zutaten aus A werden automatisch übernommen (kein Radio-Button, aber Info-Text)
    ```

  - **Test K5+K6: Erfolgreicher Merge-Durchlauf**
    ```typescript
    // Given: Rezept "PizzaTS" (Bewertung 5) und "PizzaTSMargherita" (mit Zutaten)
    // When: Benutzer öffnet Merge-Seite
    // And: Benutzer wählt Titel aus Rezept A
    // And: Benutzer wählt Zutaten aus Rezept B
    // And: Benutzer klickt "Zusammenführen"
    // Then: Weiterleitung zur Detailansicht des Ziel-Rezepts
    // And: Erfolgsmeldung ist sichtbar
    // And: Zusammengeführtes Rezept enthält gewählte Felder
    // And: Quell-Rezept existiert nicht mehr (GET → 404)
    ```

  - **Test K7: Abbrechen**
    ```typescript
    // Given: Merge-Seite für zwei Rezepte ist geöffnet
    // When: Benutzer klickt "Abbrechen"
    // Then: Weiterleitung zu /recipes/duplicates
    // And: Beide Rezepte existieren noch (je GET → 200)
    ```

  - **Test K8: Direktlink (Deeplink)**
    ```typescript
    // Given: Zwei Rezepte mit bekannten IDs
    // When: Benutzer ruft /recipes/merge?source=idA&target=idB direkt auf
    // Then: Seite lädt korrekt (HTTP 200, beide Titel sichtbar)
    ```

  - **Test: Ungültige IDs → Fehlermeldung**
    ```typescript
    // Given: source=99999 existiert nicht
    // When: Benutzer ruft /recipes/merge?source=99999&target=1 auf
    // Then: Fehlermeldung wird angezeigt (404 oder Fehlerseite)
    ```

### Schritt 12: Qualitätschecks

- [ ] `cargo build` – keine Compiler-Fehler
- [ ] `cargo clippy -- -D warnings` – keine Clippy-Warnungen
- [ ] `cargo fmt --check` – korrekt formatiert
- [ ] `cargo test` – alle Unit- und Integrationstests grün
- [ ] `npm run test:e2e` – alle E2E-Tests grün
- [ ] Manuelle Prüfung: Merge-Flow komplett durchspielen
  - Zwei ähnliche Rezepte erstellen
  - Über Dubletten-Übersicht zur Merge-Seite navigieren
  - Felder auswählen und Merge bestätigen
  - Ziel-Rezept hat zusammengeführte Daten, Quelle ist weg
  - Abbruch-Flow testen
  - Direktlink-URL testen

---

## Abhängigkeiten

- Story 22 (Dubletten-Prüfung und Übersicht) muss abgeschlossen sein (Einstiegspunkt)
- Story 01, 02, 03 (CRUD) müssen abgeschlossen sein (wird intern genutzt)
- Technisch: `UpdateRecipe`-Struct und `update_recipe`-Funktion aus `recipe_db.rs` werden wiederverwendet
- Technisch: `format_planned_date_long` und `format_date` aus `recipes.rs` werden für die Formatierung genutzt
- Technisch: `parse_form_data` und `parse_rating` aus `recipes.rs` für Form-Parsing

---

## Test-Checkliste

- [ ] Unit-Test: `merge_recipes` – erfolgreicher Merge (Ziel aktualisiert, Quelle gelöscht)
- [ ] Unit-Test: `merge_recipes` – ungültige source_id → `RowNotFound`
- [ ] Unit-Test: `merge_recipes` – ungültige target_id → `RowNotFound`
- [ ] Unit-Test: `determine_merge_target` – Rezept mit Bewertung wird als Ziel vorgeschlagen
- [ ] Integrationstest: GET /recipes/merge → 200 mit beiden Rezepten
- [ ] Integrationstest: GET /recipes/merge ohne Parameter → 400
- [ ] Integrationstest: GET /recipes/merge mit unbekannter ID → 404
- [ ] Integrationstest: POST /recipes/merge → 302 + Ziel aktualisiert + Quelle gelöscht
- [ ] Integrationstest: POST /recipes/merge mit ungültigem Titel → Formular wieder angezeigt
- [ ] E2E-Test: Merge-Button auf Dubletten-Übersicht (K1)
- [ ] E2E-Test: Merge-Seite zeigt beide Rezepte vollständig (K2)
- [ ] E2E-Test: Ziel-Rezept-Wahl sichtbar (K3)
- [ ] E2E-Test: Feldauswahl bei Konflikt (K4)
- [ ] E2E-Test: Automatische Übernahme bei einseitigem Inhalt (K4b)
- [ ] E2E-Test: Erfolgreicher Merge-Durchlauf (K5+K6)
- [ ] E2E-Test: Abbrechen – beide Rezepte bleiben erhalten (K7)
- [ ] E2E-Test: Direktlink (K8)
- [ ] Manueller Test: Tastatur-Navigation durch die Merge-Seite vollständig

---

## Offene Punkte / Technische Entscheidungen

### Entschieden:

1. **Keine Live-Vorschau via HTMX** – Die Merge-Seite zeigt direkt beide Rezepte mit Radio-Buttons.
   Das Ergebnis ist durch die Auswahl implizit klar. Ein separater Vorschau-Schritt (K5) wird
   dadurch gelöst, dass die Radio-Button-Auswahl direkt das Merge-Ergebnis repräsentiert.
   Begründung: Einfachere Implementierung, weniger JS-Abhängigkeit, funktioniert ohne JS.

2. **source/target im URL-Query** – Der Nutzer sieht initial `source=idA&target=idB` aus der
   Dubletten-Übersicht. Auf der Merge-Seite sind source und target als Hidden-Felder im Formular,
   sodass der POST klar weiß, welches Rezept gelöscht wird. Das "Tauschen" wäre ein separater
   Link-Wechsel (source↔target in der URL), aber da der Nutzer per Radio-Button alle Felder
   frei wählen kann, ist ein explizites "Tauschen" nicht nötig.

3. **Merge-Ergebnis-Vorschau (K5)** – entfällt als separater Schritt; die Radio-Button-Zeilen
   zeigen direkt den Inhalt beider Optionen. Nutzer sieht was er wählt und bestätigt per Submit.

4. **Validierung beim Merge** – Gleiche Validierungsregeln wie beim Erstellen/Bearbeiten
   (`validate_recipe_fields`): Titel required, max 100 Zeichen; Kategorien aus VALID_CATEGORIES;
   Felder size-limits. Bei Validierungsfehler: Merge-Seite mit Fehlermeldung und vorausgefüllten
   Feldern erneut rendern (beide Rezepte neu laden).

5. **Nach erfolgreichem Merge**: Weiterleitung zu `/recipes/{target_id}?success=1`.
   Die Detailansicht zeigt die bestehende Erfolgsmeldung ("Rezept erfolgreich aktualisiert"),
   die semantisch passt. Keine weitere Änderung nötig.

### Noch offen:

- Soll nach einem Merge die Dubletten-Übersicht neu berechnet werden? → Entscheidung: Nein.
  Weiterleitung zur Detailansicht ist ausreichend. Der Nutzer kann danach manuell zur
  Dubletten-Übersicht zurücknavigieren.
- Soll der Merge auch direkt aus der Detailansicht initiiert werden können? → Nicht im Scope
  von Story 23.
