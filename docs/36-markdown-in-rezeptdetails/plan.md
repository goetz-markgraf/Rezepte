# Implementierungsplan: Story 36 - Markdown-Rendering in der Rezept-Detailansicht

## Übersicht

Markdown-Rendering für die Felder `ingredients` und `instructions` in der Detailansicht.
Rendering erfolgt **serverseitig** im Handler, das Ergebnis wird als `PreEscaped`-HTML an
das Askama-Template übergeben. Keine Änderung am Datenmodell.

**Gewählte Crate:** `pulldown-cmark` (de-facto Standard, aktiv gepflegt, ~500k Downloads/Monat)
**Sicherheit:** `ammonia` für HTML-Sanitization nach dem Markdown-Rendering (XSS-Schutz, K10)

---

## Entscheidungen

### Wo wird Markdown gerendert? — Im Handler, nicht im Template-Filter

Askama hat keine eingebaute `safe`-Ausgabe für dynamisch erzeugte HTML-Strings.
Der sicherste und idiomatischste Weg in Askama ist:
1. Handler konvertiert Markdown → HTML-String (mit `pulldown-cmark`)
2. HTML-String wird mit `ammonia` sanitisiert (XSS-Schutz)
3. Template-Struct erhält `askama::Html<String>` (früher `PreEscaped`) — Askama gibt diesen Wert
   **ohne** erneutes HTML-Escaping aus
4. Template nutzt `{{ ingredients_html }}` direkt (kein `|safe`-Filter nötig, kein unsafe Code)

### Checkboxen (K4) — Interaktiv ohne HTMX, ohne Persistenz

`pulldown-cmark` rendert `- [ ]` / `- [x]` als `<input type="checkbox" disabled>`.
Um Interaktivität (anklickbar, kein Speichern) zu ermöglichen, wird das `disabled`-Attribut
via CSS-Override entfernt und JS verhindert das Neuladen. Da die App ohne JS funktionieren
soll (DoD), bleibt das Verhalten ohne JS: Checkboxen sind deaktiviert (nicht anklickbar),
aber korrekt als `☐`/`☑` dargestellt. Mit JS werden sie per kleinem Script anklickbar.

---

## Technische Schritte

### Schritt 1: Abhängigkeiten in Cargo.toml hinzufügen

- [ ] `pulldown-cmark = "0.11"` in `[dependencies]` eintragen
- [ ] `ammonia = "4"` in `[dependencies]` eintragen
- [ ] `cargo build` ausführen und sicherstellen, dass alles kompiliert

**Geänderte Datei:** `Cargo.toml`

---

### Schritt 2: Markdown-Hilfsfunktion erstellen (mit Unit-Tests)

- [ ] Neue Datei `src/markdown.rs` anlegen
- [ ] Funktion `pub fn render_markdown(input: &str) -> String` implementieren:
  - Nutzt `pulldown-cmark` mit Optionen: `TABLES | STRIKETHROUGH | TASKLISTS`
  - Gibt HTML-String zurück (noch nicht sanitisiert)
- [ ] Funktion `pub fn render_and_sanitize(input: Option<&str>) -> Option<String>` implementieren:
  - `None` → `None` (leeres Feld bleibt leer, Sektion wird ausgeblendet)
  - Leerstring / nur Whitespace → `None`
  - Sonst: `render_markdown` aufrufen, dann mit `ammonia::clean()` sanitisieren
- [ ] Modul in `src/lib.rs` exportieren: `pub mod markdown;`

**Unit-Tests in `src/markdown.rs`:**
- [ ] Test: Aufzählung `- Item` → enthält `<ul>` und `<li>`
- [ ] Test: Nummerierte Liste `1. Item` → enthält `<ol>` und `<li>`
- [ ] Test: `**fett**` → enthält `<strong>`
- [ ] Test: `*kursiv*` → enthält `<em>`
- [ ] Test: `- [ ] Aufgabe` → enthält `<input type="checkbox"`
- [ ] Test: `- [x] Erledigt` → enthält `checked`
- [ ] Test: `# Überschrift` → enthält `<h1>`
- [ ] Test: `---` → enthält `<hr`
- [ ] Test: `` `code` `` → enthält `<code>`
- [ ] Test: Fließtext ohne Markdown → enthält `<p>` mit dem Text
- [ ] Test: `None` → gibt `None` zurück
- [ ] Test: Leerstring → gibt `None` zurück
- [ ] Test: Nur Whitespace → gibt `None` zurück
- [ ] Test: `<script>alert(1)</script>` als Input → `<script>`-Tag fehlt im Output (XSS-Schutz)

**Geänderte Dateien:** `src/markdown.rs` (neu), `src/lib.rs`

---

### Schritt 3: Template-Struct anpassen

- [ ] `RecipeDetailTemplate` in `src/templates.rs` erweitern:
  - Felder `ingredients` und `instructions` von `Option<String>` auf `Option<askama::Html<String>>` ändern
  - **Hinweis:** `askama::Html<String>` verhindert doppeltes Escaping in Templates
- [ ] Handler `show_recipe` in `src/routes/recipes.rs` anpassen:
  - `use crate::markdown::render_and_sanitize;` importieren
  - `ingredients` und `instructions` vor der Template-Übergabe durch `render_and_sanitize` konvertieren
  - Ergebnis in `askama::Html(html_string)` wrappen

**Geänderte Dateien:** `src/templates.rs`, `src/routes/recipes.rs`

---

### Schritt 4: Template anpassen

- [ ] `templates/recipes/detail.html` anpassen:
  - `<pre>{{ ingredients }}</pre>` → `<div class="markdown-content">{{ ingredients }}</div>`
  - `<pre>{{ instructions }}</pre>` → `<div class="markdown-content">{{ instructions }}</div>`
  - Askama gibt `askama::Html<String>` ohne Escaping aus — kein `|safe`-Filter nötig
- [ ] Sicherstellen, dass die `{% if let Some(...) %}`-Bedingungen weiterhin korrekt funktionieren
  (bei `None` wird der Abschnitt ausgeblendet, K9)

**Geänderte Datei:** `templates/recipes/detail.html`

---

### Schritt 5: CSS für Markdown-Ausgabe ergänzen

- [ ] In `src/static/css/app.css` Styles für `.markdown-content` ergänzen:
  - Listen: `ul`, `ol`, `li` — Einrückung, Listenzeichen
  - Tabellen (falls verwendet): `table`, `th`, `td`
  - Code: `code`, `pre` — Monospace-Font, Hintergrundfarbe
  - Checkboxen: `input[type="checkbox"]` ohne `disabled` (via JS-Klasse oder direkt)
  - Überschriften `h1`–`h3` in `.markdown-content` — visuelle Hierarchie ohne Konflikte
    mit dem Seiten-`h1`/`h2` (z.B. etwas kleiner)
  - `hr` — horizontale Linie
  - Kein horizontaler Overflow (K: Sehr langer Text)

**Geänderte Datei:** `src/static/css/app.css`

---

### Schritt 6: Checkbox-Interaktivität (progressiv)

- [ ] Kleines Inline-Script oder separate JS-Datei, das `disabled` von Checkboxen
  in `.markdown-content` entfernt und `change`-Events verhindert (kein Submit, kein Reload)
- [ ] Ohne JS: Checkboxen bleiben `disabled` (sichtbar, aber nicht anklickbar) — akzeptabel
- [ ] Mit JS: Checkboxen sind anklickbar, Zustand wird nicht gespeichert (K4)

**Geänderte Datei:** `src/static/css/app.css` oder neues `src/static/js/checkboxes.js`

---

### Schritt 7: Rust-Integrationstests erweitern

- [ ] In `tests/recipe_detail.rs` neue Tests ergänzen:

  ```rust
  // Given/When/Then als Kommentare inline
  ```

  - [ ] Test: Aufzählungsliste in `ingredients` → HTML enthält `<ul>`
  - [ ] Test: Nummerierte Liste in `instructions` → HTML enthält `<ol>`
  - [ ] Test: `**fett**` in `instructions` → HTML enthält `<strong>`
  - [ ] Test: `<script>` in `ingredients` → HTML enthält **nicht** `<script>` (XSS)
  - [ ] Test: Leere `ingredients` → Abschnitt `class="ingredients"` nicht im HTML
    (bereits vorhanden, bleibt bestehen)
  - [ ] Test: Nur Whitespace in `ingredients` → Abschnitt nicht im HTML

**Geänderte Datei:** `tests/recipe_detail.rs`

---

### Schritt 8: Seed-Daten für E2E-Tests erstellen

- [ ] `tests/seeds/recipe-markdown.sql` erstellen mit Rezepten:
  - Rezept 1: Zutaten als Aufzählungsliste (`- 500g Mehl\n- 1 Ei\n- 250ml Milch`)
  - Rezept 2: Zutaten mit Checkboxen (`- [ ] Mehl\n- [x] Eier`)
  - Rezept 3: Zubereitung mit Fettschrift (`**Wichtig:** Ofen vorheizen auf 180°C`)
  - Rezept 4: Nur Titel, leere Felder (für K9)
  - Rezept 5: Fließtext ohne Markdown-Syntax

**Neue Datei:** `tests/seeds/recipe-markdown.sql`

---

### Schritt 9: E2E-Tests (Playwright)

- [ ] `tests/e2e/recipe-markdown.spec.ts` erstellen mit folgenden Tests:

  **Test 1 — K1: Aufzählungsliste in Zutaten wird gerendert**
  ```
  // Given: Rezept mit Zutaten "- 500g Mehl\n- 1 Ei\n- 250ml Milch" (aus Seed)
  // When: Detailseite wird aufgerufen
  // Then: section.ingredients enthält <ul> mit 3 <li>-Elementen
  // And: Text "- 500g Mehl" ist nicht als Rohtext sichtbar
  ```

  **Test 2 — K4: Checkboxen werden dargestellt**
  ```
  // Given: Rezept mit Zutaten "- [ ] Mehl\n- [x] Eier" (aus Seed)
  // When: Detailseite wird aufgerufen
  // Then: Zwei input[type="checkbox"] sind sichtbar
  // And: Eine ist nicht angehakt, eine ist angehakt (checked)
  ```

  **Test 3 — K3: Fettschrift in Zubereitung wird gerendert**
  ```
  // Given: Rezept mit Zubereitung "**Wichtig:** Ofen vorheizen auf 180°C" (aus Seed)
  // When: Detailseite wird aufgerufen
  // Then: <strong>-Element mit Text "Wichtig:" ist sichtbar
  // And: Text "**Wichtig:**" ist nicht als Rohtext sichtbar
  ```

  **Test 4 — K9: Leere Felder — Abschnitt ausgeblendet**
  ```
  // Given: Rezept ohne Zutaten und Zubereitung (aus Seed)
  // When: Detailseite wird aufgerufen
  // Then: section.ingredients ist nicht vorhanden
  // And: section.instructions ist nicht vorhanden
  ```

  **Test 5 — K8: Fließtext bleibt lesbar**
  ```
  // Given: Rezept mit Fließtext in Zubereitung (aus Seed)
  // When: Detailseite wird aufgerufen
  // Then: Text ist vollständig in einem <p>-Element sichtbar
  ```

  **Test 6 — K10: HTML wird nicht ausgeführt (XSS)**
  ```
  // Given: Rezept mit Zutaten "<script>alert(1)</script>" (inline erstellt per Formular)
  // When: Detailseite wird aufgerufen
  // Then: kein alert() wurde ausgeführt
  // And: "<script>" ist nicht im DOM als aktives Element
  ```

  **Test 7 — K2: Nummerierte Liste in Zubereitung**
  ```
  // Given: Rezept mit Zubereitung "1. Ofen vorheizen\n2. Teig kneten" (inline erstellt)
  // When: Detailseite wird aufgerufen
  // Then: section.instructions enthält <ol> mit 2 <li>-Elementen
  ```

**Neue Datei:** `tests/e2e/recipe-markdown.spec.ts`

---

### Schritt 10: Qualitätsprüfung

- [ ] `cargo fmt` — Formatierung prüfen
- [ ] `cargo clippy -- -D warnings` — keine Warnungen
- [ ] `cargo test` — alle Unit- und Integrationstests grün
- [ ] `npm run test:e2e` — alle E2E-Tests grün
- [ ] Manuelle Prüfung: Detailseite mit Beispiel-Markdown im Browser öffnen

---

## Geänderte und neue Dateien

| Datei | Aktion |
|---|---|
| `Cargo.toml` | `pulldown-cmark` und `ammonia` hinzufügen |
| `src/markdown.rs` | **Neu** — Hilfsfunktionen für Markdown-Rendering |
| `src/lib.rs` | `pub mod markdown;` exportieren |
| `src/templates.rs` | `RecipeDetailTemplate`: Felder auf `Option<askama::Html<String>>` |
| `src/routes/recipes.rs` | `show_recipe`-Handler: `render_and_sanitize` aufrufen |
| `templates/recipes/detail.html` | `<pre>` durch `<div class="markdown-content">` ersetzen |
| `src/static/css/app.css` | Styles für `.markdown-content` |
| `src/static/js/checkboxes.js` | **Neu (optional)** — Checkbox-Interaktivität |
| `tests/recipe_detail.rs` | Neue Unit-/Integrationstests für Markdown-Rendering |
| `tests/seeds/recipe-markdown.sql` | **Neu** — Testdaten mit Markdown-Inhalten |
| `tests/e2e/recipe-markdown.spec.ts` | **Neu** — Playwright E2E-Tests |

---

## URL-Struktur

Keine Änderung. Markdown-Rendering ist rein intern:

```
GET  /recipes/{id}  →  Detailansicht mit gerendertem Markdown
```

---

## Abhängigkeiten

- Story 04 (Rezept-Detailansicht) muss implementiert sein — ist der Fall
- `RecipeDetailTemplate` und `show_recipe`-Handler existieren bereits

---

## Test-Checkliste

- [ ] Unit-Test: `render_markdown("- Item")` → enthält `<ul><li>`
- [ ] Unit-Test: `render_markdown("**fett**")` → enthält `<strong>`
- [ ] Unit-Test: `render_and_sanitize(None)` → `None`
- [ ] Unit-Test: `render_and_sanitize(Some(""))` → `None`
- [ ] Unit-Test: XSS-Input → `<script>` im Output nicht vorhanden
- [ ] Integrationstest: Aufzählungsliste → `<ul>` im HTML der Detailseite
- [ ] Integrationstest: Fettschrift → `<strong>` im HTML der Detailseite
- [ ] Integrationstest: XSS in Zutaten → kein `<script>` im HTML
- [ ] E2E-Test: Aufzählung in Zutaten (K1)
- [ ] E2E-Test: Checkboxen anzeigen (K4)
- [ ] E2E-Test: Fettschrift in Zubereitung (K3)
- [ ] E2E-Test: Leere Felder ausgeblendet (K9)
- [ ] E2E-Test: Fließtext (K8)
- [ ] E2E-Test: XSS verhindert (K10)
- [ ] E2E-Test: Nummerierte Liste (K2)
- [ ] Manueller Test: Alle Markdown-Features im Browser prüfen

---

## Offene Punkte

- Checkbox-Persistenz (Story-Frage): Laut Story nur visuell ohne Speicherung → kein HTMX-Endpoint nötig
- `askama` Version 0.12 (aktuell): Unterstützt `askama::Html<String>` als Escape-freien Wrapper
  — vor der Implementierung in der Askama-Doku bestätigen, falls die API geändert wurde
