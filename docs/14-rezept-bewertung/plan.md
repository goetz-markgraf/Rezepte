# Implementierungsplan: Story 14 - Rezept mit 3-5 Sternen bewerten

## Ausgangslage und Analyse

### Was bereits existiert

- Die SQLite-Tabelle `recipes` enthält bereits das Feld `rating INTEGER CHECK (rating BETWEEN 1 AND 5)` (Migration 001_initial.sql)
- Das `Recipe`-Struct in `src/models/recipe.rs` enthält **kein** `rating`-Feld (noch nicht implementiert)
- `CreateRecipe` und `UpdateRecipe` kennen kein `rating`-Feld
- Die Templates `form.html`, `detail.html` und `index.html` haben keine Bewertungsanzeige
- `RecipeFormTemplate`, `RecipeDetailTemplate` und `RecipeListItem` in `src/templates.rs` haben kein `rating`-Feld
- Die DB-Queries in `src/models/recipe_db.rs` selektieren kein `rating` und schreiben es auch nicht

### Was zu tun ist

Das `rating`-Feld ist in der DB vorhanden, muss aber durch alle Schichten (Model → DB-Layer → Route-Handler → Templates) durchgezogen werden.

---

## Technische Schritte

### Schritt 1: Model erweitern (`src/models/recipe.rs`)

- [ ] `Recipe`-Struct: Feld `pub rating: Option<i32>` hinzufügen
- [ ] `CreateRecipe`-Struct: Feld `pub rating: Option<i32>` hinzufügen
- [ ] `UpdateRecipe`-Struct: Feld `pub rating: Option<i32>` hinzufügen
- [ ] `validate_recipe_fields`-Funktion: Parameter `rating: Option<i32>` hinzufügen und Validierung implementieren (Wert muss zwischen 1 und 5 liegen, wenn vorhanden)
- [ ] Hilfsfunktion `validate_rating(rating: Option<i32>) -> Option<String>` extrahieren (oder inline in `validate_recipe_fields`)
- [ ] `CreateRecipe::validate()` und `UpdateRecipe::validate()` die neue Rating-Validierung einbinden
- [ ] Unit-Tests für Rating-Validierung:
  - Test: Rating 0 wird abgelehnt
  - Test: Rating 1 wird akzeptiert
  - Test: Rating 5 wird akzeptiert
  - Test: Rating 6 wird abgelehnt
  - Test: Rating None (leer) wird akzeptiert
  - Test: Bestehende Tests für `create_recipe_accepts_valid_data` anpassen (neues Pflichtfeld `rating`)

### Schritt 2: DB-Layer erweitern (`src/models/recipe_db.rs`)

- [ ] `get_recipe_by_id`: SELECT-Query um `rating` erweitern
- [ ] `get_all_recipes`: SELECT-Query um `rating` erweitern
- [ ] `search_recipes`: SELECT-Query um `rating` erweitern
- [ ] `filter_recipes_by_categories`: SQL-String um `rating` erweitern
- [ ] `filter_recipes_not_made_recently`: SQL-String um `rating` erweitern
- [ ] `filter_recipes_next_seven_days` (falls vorhanden): SQL-String um `rating` erweitern
- [ ] `create_recipe`: INSERT-Query um `rating`-Feld erweitern (`.bind(&recipe.rating)`)
- [ ] `update_recipe`: UPDATE-Query um `rating = ?N` erweitern (`.bind(&recipe.rating)`)
- [ ] Sicherstellen, dass alle `query_as::<_, Recipe>` Statements mit den Feldern des Structs übereinstimmen

### Schritt 3: Template-Structs erweitern (`src/templates.rs`)

- [ ] `RecipeFormTemplate`: Feld `pub rating: Option<i32>` hinzufügen
- [ ] `RecipeFormTemplate::default()` und `::new()`: `rating: None` setzen
- [ ] `RecipeDetailTemplate`: Feld `pub rating: Option<i32>` hinzufügen
- [ ] `RecipeListItem`: Feld `pub rating: Option<i32>` hinzufügen

### Schritt 4: Route-Handler anpassen (`src/routes/recipes.rs`)

- [ ] `create_recipe_handler`: `rating`-Feld aus Formulardaten lesen (als `Option<i32>`, leerer String → `None`)
- [ ] Hilfsfunktion `parse_rating(raw: Option<&str>) -> Option<i32>` implementieren (leerer String → None, Zahl parsen)
- [ ] `create_recipe_handler`: `rating` in `CreateRecipe`-Struct befüllen
- [ ] `create_recipe_handler`: Bei Validierungsfehlern `rating` im Fehler-Template zurückgeben
- [ ] `update_recipe_handler`: analog zu `create_recipe_handler` für `UpdateRecipe`
- [ ] `edit_recipe_form`: `recipe.rating` in `RecipeFormTemplate` befüllen
- [ ] `show_recipe`: `recipe.rating` in `RecipeDetailTemplate` befüllen
- [ ] `index`-Handler: `r.rating` in `RecipeListItem` befüllen (in der `recipe_items`-Map)

### Schritt 5: Templates anpassen

#### 5a: Formular-Template (`templates/recipes/form.html`)

- [ ] Sterne-Auswahl als Radio-Button-Gruppe implementieren (1-5 Sterne + "Keine Bewertung" als Reset)
- [ ] Platzierung: nach den Kategorien, vor dem Datum-Feld
- [ ] `name="rating"`, Wert leer für "Keine Bewertung", Wert "1"-"5" für Sterne
- [ ] Aktuelle Bewertung vorauswählen: `{% if rating == Some(n) %}checked{% endif %}`
- [ ] "Keine Bewertung" ist vorausgewählt, wenn `rating == None`
- [ ] Label-Text für Screenreader: `aria-label="1 Stern"`, `"2 Sterne"`, ..., `"5 Sterne"`
- [ ] Sterne-Symbole (★) als visuelles Label, aber auch expliziter Text via `aria-label`
- [ ] Touch-freundliche Mindestgröße: `min-height: 44px` für die Radio-Labels

#### 5b: Detail-Template (`templates/recipes/detail.html`)

- [ ] Bewertungsanzeige in der `<header>`-Sektion (nach den Kategorien)
- [ ] Nur anzeigen, wenn `rating != None`
- [ ] Darstellung: ausgefüllte Sterne (★) für gewählte, leere (☆) für nicht gewählte
- [ ] Beispiel: 4 Sterne → `★★★★☆`
- [ ] `aria-label` für Screenreader: "4 von 5 Sternen"
- [ ] Kein Sterne-Block bei unbewerteten Rezepten

#### 5c: Listen-Template (`templates/index.html`)

- [ ] Sterne-Anzeige neben oder unter dem Rezepttitel in der Listenansicht
- [ ] Nur ausgefüllte Sterne (★), keine leeren Platzhalter
- [ ] Keine Anzeige bei unbewerteten Rezepten (kein leerer Platzhalter)
- [ ] Responsive: ausreichend Touch-Fläche auf Mobile

### Schritt 6: CSS-Styling (`src/static/css/app.css`)

- [ ] `.star-rating` für die Formular-Sterne-Auswahl (Radio-Buttons als Sterne)
- [ ] `.star-rating input[type="radio"]` visuell verstecken (Screenreader behält Zugang)
- [ ] `.star-rating label` als klickbarer Stern gestylt (Cursor pointer, min 44x44px)
- [ ] Aktiver/ausgewählter Stern in goldgelb (`#f59e0b` oder ähnlich)
- [ ] `.recipe-stars` für die reine Anzeige der Bewertung in Detail- und Listenansicht
- [ ] Mobile-first: Sterne groß genug für Touch-Bedienung im Formular
- [ ] Keine Sterne erscheinen in der CSS-Basis ohne Rating-Klasse

### Schritt 7: Integrationstests (`tests/recipe_rating.rs`)

Neue Test-Datei `tests/recipe_rating.rs` erstellen:

- [ ] Test: `create_recipe_with_rating_stores_it` — POST mit `rating=4` erstellt Rezept, Detail-Seite zeigt 4 Sterne
- [ ] Test: `create_recipe_without_rating_stores_null` — POST ohne `rating` erstellt Rezept ohne Bewertung, Detail-Seite zeigt keine Sterne
- [ ] Test: `update_recipe_changes_rating` — Rezept mit Rating 3 → Update auf Rating 5 → Detail-Seite zeigt 5 Sterne
- [ ] Test: `update_recipe_removes_rating` — Rezept mit Rating 5 → Update mit leerem Rating → kein Rating mehr
- [ ] Test: `create_recipe_rejects_rating_zero` — POST mit `rating=0` → HTTP 400, Fehlermeldung
- [ ] Test: `create_recipe_rejects_rating_six` — POST mit `rating=6` → HTTP 400, Fehlermeldung
- [ ] Test: `create_recipe_accepts_rating_one` — Rating 1 ist gültig (Negativbewertung)
- [ ] Test: `create_recipe_accepts_rating_five` — Rating 5 ist gültig
- [ ] Test: `recipe_list_shows_rating` — Liste zeigt Sterne für bewertete Rezepte
- [ ] Test: `recipe_list_hides_rating_when_none` — Liste zeigt keine Sterne bei unbewerteten Rezepten
- [ ] Test: `form_prefills_rating_when_editing` — Edit-Formular hat aktuelles Rating vorausgewählt
- [ ] Jeder Test: Given/When/Then als deutsche Kommentare

### Schritt 8: E2E-Tests (`tests/e2e/recipe-rating.spec.ts`)

Neue Playwright-Datei `tests/e2e/recipe-rating.spec.ts` erstellen:

- [ ] **Testfall K1/K2: Bewertung setzen**
  ```
  Given: Rezept ohne Bewertung
  When: Bearbeitungsformular öffnen, 4 Sterne auswählen, speichern
  Then: Detail-Seite zeigt 4 Sterne (★★★★☆)
  ```
- [ ] **Testfall K2: Alle Werte 1-5 speicherbar**
  ```
  Given: Neues Rezept anlegen
  When: 1 Stern auswählen, speichern
  Then: Detail-Seite zeigt 1 Stern (★☆☆☆☆)
  ```
- [ ] **Testfall K3: Bewertung in Detailansicht**
  ```
  Given: Rezept mit 5-Sterne-Bewertung
  When: Detail-Seite öffnen
  Then: 5 ausgefüllte Sterne sichtbar (★★★★★)
  ```
- [ ] **Testfall K3: Kein Sterne-Block bei unbewerteten Rezepten**
  ```
  Given: Rezept ohne Bewertung
  When: Detail-Seite öffnen
  Then: Kein Sterne-Block sichtbar
  ```
- [ ] **Testfall K4: Bewertung in der Listenansicht**
  ```
  Given: Rezept A mit 5 Sternen, Rezept B ohne Bewertung
  When: Rezeptliste öffnen
  Then: Rezept A zeigt 5 Sterne, Rezept B zeigt keine Sterne
  ```
- [ ] **Testfall K5: Bewertung zurücksetzen**
  ```
  Given: Rezept mit 5-Sterne-Bewertung
  When: Bearbeiten → "Keine Bewertung" wählen → speichern
  Then: Detail-Seite zeigt keinen Sterne-Block
  ```
- [ ] **Testfall K6: Negativbewertung 1-2 Sterne**
  ```
  Given: Neues Rezept
  When: 1 Stern setzen, speichern
  Then: Rezept in Liste sichtbar, 1 Stern angezeigt
  ```
- [ ] **Testfall K1: Formular vorausgefüllt**
  ```
  Given: Rezept mit 3 Sternen existiert
  When: Bearbeitungsformular öffnen
  Then: Radio-Button für 3 Sterne ist ausgewählt
  ```
- [ ] **Testfall K8: Keyboard-Navigation**
  ```
  Given: Bearbeitungsformular ist geöffnet
  When: Tab-Taste zur Sterne-Auswahl navigieren
  Then: Rating-Gruppe ist per Tastatur erreichbar und Sterne per Pfeiltaste wechselbar
  ```
- [ ] Jeder Test: Given/When/Then als deutsche Kommentare

### Schritt 9: Seed-Daten für E2E-Tests (`tests/seeds/recipe-rating.sql`)

- [ ] Seed-Datei erstellen mit:
  - Rezept mit Rating 5 (für Listensicht-Test)
  - Rezept ohne Rating (für Vergleich)
  - Rezept mit Rating 3 (für Änderungs-Test)

---

## URL-Struktur

Keine neuen URLs. Bestehende Endpunkte werden erweitert:

```
GET   /recipes/new         →  Formular mit Rating-Feld (leer)
POST  /recipes             →  Rating aus Form-Daten wird gespeichert
GET   /recipes/{id}        →  Detailansicht zeigt Sterne-Bewertung
GET   /recipes/{id}/edit   →  Formular mit vorausgewähltem Rating
POST  /recipes/{id}        →  Rating-Änderung wird gespeichert
GET   /                    →  Liste zeigt Sterne neben Rezepttiteln
```

---

## Abhängigkeiten

- Story 02 (Rezept bearbeiten): Abgeschlossen — Formular-Handler existiert
- Story 04 (Rezept-Detailansicht): Abgeschlossen — Template existiert
- Story 05 (Rezeptliste): Abgeschlossen — Listen-Template existiert
- Das `rating`-Feld ist bereits in der SQLite-Migration vorhanden (keine neue Migration nötig)
- Blockiert durch diese Story: Story 11 (Filter nach Bewertung)

---

## Wichtige Implementierungsdetails

### Radio-Button-Gruppe für Sterne-Auswahl

Die Sterne-Auswahl im Formular wird als klassische Radio-Button-Gruppe implementiert. Das ermöglicht:
- Barrierefreiheit ohne JavaScript (Tab + Pfeiltasten)
- Funktioniert ohne JS (reines HTML-Formular)
- Visuelles Sterne-Styling via CSS (Radio-Buttons versteckt, Labels als Sterne)

HTML-Struktur (Askama-Template):
```html
<fieldset class="star-rating">
  <legend>Bewertung</legend>
  <label>
    <input type="radio" name="rating" value="" {% if rating.is_none() %}checked{% endif %} aria-label="Keine Bewertung">
    Keine Bewertung
  </label>
  {% for n in [1i32, 2, 3, 4, 5] %}
  <label aria-label="{% if n == 1 %}1 Stern{% else %}{{ n }} Sterne{% endif %}">
    <input type="radio" name="rating" value="{{ n }}" {% if rating == Some(n) %}checked{% endif %}>
    ★
  </label>
  {% endfor %}
</fieldset>
```

### Sterne-Anzeige in Detail und Liste

Für die Anzeige wird eine Schleife 1..=5 gerendert:
```
{% if let Some(r) = rating %}
<span class="recipe-stars" aria-label="{{ r }} von 5 Sternen">
  {% for n in [1i32, 2, 3, 4, 5] %}
    {% if n <= r %}★{% else %}☆{% endif %}
  {% endfor %}
</span>
{% endif %}
```

In der Listenansicht: nur ausgefüllte Sterne (keine leeren), also simpler `"★".repeat(r as usize)`.

### Rating-Parsing im Form-Handler

Da `RawForm` + `parse_form_data` verwendet wird:
```rust
fn parse_rating(raw: Option<&str>) -> Option<i32> {
    raw.filter(|s| !s.trim().is_empty())
       .and_then(|s| s.trim().parse::<i32>().ok())
}
```

### Validierung im Model

```rust
fn validate_rating(rating: Option<i32>) -> Option<String> {
    rating.filter(|&r| r < 1 || r > 5)
          .map(|_| "Bewertung muss zwischen 1 und 5 Sternen liegen".to_string())
}
```

---

## Test-Checkliste (DoD)

### Unit-Tests (`src/models/recipe.rs`)
- [ ] Unit-Test: Rating 0 wird abgelehnt
- [ ] Unit-Test: Rating 1 wird akzeptiert (Negativbewertung)
- [ ] Unit-Test: Rating 5 wird akzeptiert
- [ ] Unit-Test: Rating 6 wird abgelehnt
- [ ] Unit-Test: Rating None wird akzeptiert (kein Pflichtfeld)
- [ ] Unit-Test: Bestehende Tests weiterhin grün (ggf. Rating-Feld ergänzen)

### Integrationstests (`tests/recipe_rating.rs`)
- [ ] Integrationstest: POST mit Rating 4 → Detail-Seite zeigt Rating
- [ ] Integrationstest: POST ohne Rating → Detail-Seite zeigt kein Rating
- [ ] Integrationstest: Update ändert Rating von 3 auf 5
- [ ] Integrationstest: Update entfernt Rating (leerer Wert)
- [ ] Integrationstest: Rating 0 → HTTP 400
- [ ] Integrationstest: Rating 6 → HTTP 400
- [ ] Integrationstest: Rating 1 → gültig
- [ ] Integrationstest: Listenansicht zeigt Rating
- [ ] Integrationstest: Edit-Formular hat Rating vorausgefüllt

### E2E-Tests (`tests/e2e/recipe-rating.spec.ts`)
- [ ] E2E-Test: K1 — Bewertungsfeld im Bearbeitungsformular vorhanden
- [ ] E2E-Test: K2 — Bewertung setzen und speichern
- [ ] E2E-Test: K3 — Bewertung in Detailansicht (Sterne-Symbole)
- [ ] E2E-Test: K3 — Unbewertetes Rezept: kein Sterne-Block
- [ ] E2E-Test: K4 — Bewertung in der Listenansicht
- [ ] E2E-Test: K4 — Unbewertetes Rezept: keine Sterne in Liste
- [ ] E2E-Test: K5 — Bewertung zurücksetzen auf "Keine Bewertung"
- [ ] E2E-Test: K6 — Negativbewertung (1-2 Sterne) speicherbar
- [ ] E2E-Test: K1 — Formular vorausgefüllt mit aktueller Bewertung
- [ ] E2E-Test: K8 — Keyboard-Navigation durch Sterne-Auswahl

### Manuelle Tests
- [ ] Manuell: Sterne im Formular auf Mobilgerät (Touch, Mindest-Tappfläche 44x44px)
- [ ] Manuell: Screenreader liest Rating-Labels korrekt vor
- [ ] Manuell: Sterne erscheinen in Liste und Detailansicht korrekt
- [ ] Manuell: Leere Bewertung (None) hinterlässt keinen leeren Platzhalter

---

## Code-Qualität (DoD)

- [ ] `cargo build` ohne Warnungen
- [ ] `cargo clippy -- -D warnings` ohne Befunde
- [ ] `cargo fmt --check` sauber
- [ ] `cargo test` alle Tests grün
- [ ] `npm run test:e2e` alle E2E-Tests grün
- [ ] Alle öffentlichen Funktionen haben `///`-Doc-Kommentare
- [ ] Hilfsfunktionen haben maximal 50 Zeilen und maximal 4 Parameter

---

## Offene Punkte

- Die Askama-Template-Syntax für Schleifen über Integer-Ranges (`{% for n in [1i32, 2, 3, 4, 5] %}`) muss verifiziert werden — alternativ eine Hilfsmethode `fn star_range() -> &'static [i32]` im Template-Struct ergänzen
- Visuelles Design der Sterne-Auswahl: CSS-only (Radio-Buttons mit Haken-Styling) oder Unicode-Sterne als Labels — Unicode-Sterne sind barrierefreier und einfacher zu implementieren
- Ob 1-2 Sterne im UI explizit als "Negativbewertung" markiert werden sollen (z.B. andere Farbe), bleibt offen — zunächst einheitliche Darstellung für alle Sterne 1-5
