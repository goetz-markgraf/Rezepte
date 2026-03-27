# Implementierungsplan: Story 04 - Rezept-Detailansicht

## Analyse des Ist-Zustands

Der Großteil der technischen Grundstruktur für die Detailansicht wurde bereits im Rahmen früherer Stories als Platzhalter implementiert. Die Story 04 hat die Aufgabe, diese Ansicht vollwertig und produktionsreif zu machen. Im Einzelnen sind folgende Bausteine bereits vorhanden:

| Baustein | Status | Bemerkung |
|---|---|---|
| Route `GET /recipes/:id` | vorhanden | in `src/routes/mod.rs` |
| Handler `show_recipe` | vorhanden | in `src/routes/recipes.rs` |
| Template-Struct `RecipeDetailTemplate` | vorhanden | in `src/templates.rs` |
| HTML-Template `templates/recipes/detail.html` | vorhanden | Grundstruktur |
| CSS-Klassen `.recipe-detail`, `.category-tag`, `.success`, `.btn-danger` | vorhanden | in `src/static/css/app.css` |
| 404-Handling via `AppError::NotFound` | vorhanden | liefert aber nur Plaintext |
| `?success=1` Query-Parameter | vorhanden | wird in handler ausgewertet |

**Fehlende oder verbesserungswürdige Punkte:**
1. Die 404-Fehlerseite liefert nur einen rohen Textstring ohne HTML-Seite mit Navigation zurück zur Liste
2. Die Datumsformatierung zeigt rohe SQLite-Timestamps (z.B. `2026-03-27 10:45:00`) - kein nutzerfreundliches deutsches Datum
3. Das HTML-Template hat die `.actions`-Div innerhalb des `<footer>`-Elements, aber die CSS-Regel `.actions { margin-bottom: 2rem; }` ist auf die Liste ausgerichtet - Layout des Footers in der Detailansicht muss überprüft werden
4. Es gibt keine dedizierten E2E-Tests für die Detailansicht als eigenständige Spec-Datei (die vorhandenen Tests testen die Detailansicht nur implizit als Teil anderer Flows)
5. Kein Unit-Test, der die HTTP-Antwort des Detailansicht-Handlers auf Inhalte prüft (nur StatusCode 200 wird getestet)
6. Der Flash-Banner (`.success`) ist dauerhaft sichtbar und verschwindet nicht (kein HTMX/JS-gestütztes Auto-Dismiss)
7. Keine responsive CSS-Regeln für die Aktions-Schaltflächen auf mobilen Geräten (Schaltflächen sollen untereinander, volle Breite, gut tippbar sein)
8. Das `<footer>`-Element in der Detailansicht fehlt im semantischen Sinne - die Metainformationen (Erstellt am, Zuletzt bearbeitet) und die Aktionsschaltflächen sind zusammen in `<footer>` gruppiert, aber es fehlt eine klare visuelle Trennung

---

## Technische Schritte

### Schritt 1: HTML-Template für 404-Fehlerseite

Das bestehende `AppError::NotFound` liefert derzeit nur einen Plaintext-Body. Der Nutzer sieht keine HTML-Seite mit sinnvoller Navigation. Gemäß den Akzeptanzkriterien (K4) muss die Fehlermeldung verständlich sein und einen Link zurück zur Liste enthalten.

- [ ] Neues Askama-Template-Struct `NotFoundTemplate` in `src/templates.rs` anlegen mit Feldern `message: String`
- [ ] Neues HTML-Template `templates/error/not_found.html` erstellen (extends `base.html`), das die Fehlermeldung und einen Link zu `/` anzeigt
- [ ] `AppError::NotFound`-Zweig in `src/error.rs` so ändern, dass er die HTML-Seite rendert statt reinen Text
- [ ] Unit-Test: `show_recipe` mit ungültiger ID liefert Status 404 und HTML-Body mit Link-Text "Zurück zur Übersicht" (in `tests/recipe_detail.rs`)

### Schritt 2: Datumsformatierung

Die `created_at`- und `updated_at`-Felder werden als roher SQLite-String gespeichert und angezeigt (z.B. `2026-03-27 10:45:00`). Eine nutzerfreundliche Formatierung auf Deutsch verbessert die Lesbarkeit erheblich.

- [ ] Hilfsfunktion `format_date(date_str: &str) -> String` in einem neuen Modul oder in `src/models/recipe.rs` implementieren, die einen SQLite-Timestamp in ein deutsches Datumsformat umwandelt (z.B. `27.03.2026`)
- [ ] `RecipeDetailTemplate` in `src/templates.rs` die Felder `created_at` und `updated_at` vor dem Befüllen des Templates über die Hilfsfunktion laufen lassen, alternativ die Formatierung direkt im Template via Askama-Filter realisieren
- [ ] Unit-Test: `format_date` korrekt formatiert verschiedene Timestamps (valides Datum, ungültiger String als Fallback)
- [ ] Unit-Test: `show_recipe`-Handler gibt formatierten Datums-String in der HTML-Antwort zurück

### Schritt 3: Layout und CSS-Verbesserungen für Detailansicht

Überprüfung und Verbesserung der Detailansicht hinsichtlich Lesbarkeit, semantischer Struktur und responsiver Darstellung auf mobilen Geräten.

- [ ] In `templates/recipes/detail.html`: Überprüfen, dass Aktionsschaltflächen klar vom Metainformations-Bereich getrennt sind (visuell und semantisch korrekt im `<footer>`)
- [ ] In `src/static/css/app.css`: Responsive CSS-Regeln für `.recipe-detail .actions` hinzufügen, damit auf mobilen Geräten (max-width: 600px) die Schaltflächen untereinander und mit voller Breite erscheinen
- [ ] In `src/static/css/app.css`: Mindestschriftgröße für `.recipe-detail pre` prüfen - muss min. 16px sein für Küchennutzung
- [ ] Sicherstellen, dass `pre`-Elemente kein horizontales Scrollen verursachen (`white-space: pre-wrap` ist bereits vorhanden - prüfen)
- [ ] Sicherstellen, dass Fokus-Indikatoren für Schaltflächen und Links sichtbar sind (WCAG 2.1 Level A)
- [ ] Prüfen der Überschriften-Hierarchie im Template: H1 für Titel, H2 für "Zutaten" und "Anleitung" - bereits korrekt, nur bestätigen

### Schritt 4: Flash-Meldung mit HTMX Auto-Dismiss (Progressive Enhancement)

Die Erfolgsmeldung (`?success=1`) soll optional via HTMX nach einigen Sekunden verschwinden, ohne die Grundfunktionalität (Anzeige ohne JS) zu beeinträchtigen.

- [ ] HTMX-Skript-Tag in `templates/base.html` einbinden (falls noch nicht vorhanden - prüfen)
- [ ] In `templates/recipes/detail.html`: HTMX-Attribut `hx-get` oder eine CSS-Animation zum Auto-Dismiss der `.success`-Meldung hinzufügen (z.B. `hx-swap-oob` oder einfach CSS `animation: fadeOut 5s forwards`)
- [ ] Sicherstellen, dass die Erfolgsmeldung auch ohne JS (reines HTML) sichtbar bleibt und nur mit JS verschwindet
- [ ] Prüfen: Die Meldung soll auch wegklickbar sein (Link oder Button mit `hx-delete` oder einfach `onclick`)

**Entscheidung:** Da das Projekt HTMX nutzt und die HTMX-Bibliothek bereits über eine CDN oder lokale Einbindung genutzt werden soll, bietet sich eine einfache CSS-Animation (`@keyframes fadeOut`) als puristischste Lösung an, die kein zusätzliches JS erfordert und auch ohne HTMX funktioniert. Alternativ reicht ein simples `<button onclick="this.parentElement.remove()">` als Progressive Enhancement.

### Schritt 5: Unit-Tests für den show_recipe-Handler

Bisher wird in `tests/recipe_create.rs` nur der StatusCode 200 für die Detailansicht geprüft. Es fehlen Tests für:
- Korrekte HTML-Inhalte (Titel, Zutaten, Anleitung, Datum-Anzeige)
- Verhalten bei fehlendem Rezept (404)
- Erfolgs-Flash bei `?success=1`
- Verhalten bei fehlenden optionalen Feldern (ohne Zutaten, ohne Anleitung)

- [ ] Neue Test-Datei `tests/recipe_detail.rs` erstellen mit Hilfsfunktion `setup_test_app` und `create_test_recipe`
- [ ] Unit-Test: `show_recipe_displays_title` - Prüft, dass der Titel im HTML-Body enthalten ist
- [ ] Unit-Test: `show_recipe_displays_ingredients_section` - Prüft, dass Zutaten angezeigt werden
- [ ] Unit-Test: `show_recipe_hides_ingredients_when_empty` - Prüft, dass Abschnitt "Zutaten" fehlt, wenn leer
- [ ] Unit-Test: `show_recipe_hides_instructions_when_empty` - Prüft, dass Abschnitt "Anleitung" fehlt, wenn leer
- [ ] Unit-Test: `show_recipe_returns_404_for_missing_id` - Prüft StatusCode 404
- [ ] Unit-Test: `show_recipe_404_contains_back_link` - Prüft, dass 404-Seite einen Link zurück enthält
- [ ] Unit-Test: `show_recipe_displays_success_flash` - Prüft, dass bei `?success=1` die `.success`-Klasse erscheint
- [ ] Unit-Test: `show_recipe_no_success_flash_without_param` - Prüft, dass ohne Parameter kein Flash erscheint
- [ ] Unit-Test: `show_recipe_displays_edit_link` - Prüft, dass der Bearbeiten-Link vorhanden ist
- [ ] Unit-Test: `show_recipe_displays_delete_link` - Prüft, dass der Löschen-Link vorhanden ist

### Schritt 6: E2E-Tests mit Playwright

Gemäß Story und DoD müssen alle Akzeptanzkriterien durch E2E-Tests abgedeckt sein. Die bestehenden E2E-Tests (recipe-create, recipe-edit, recipe-delete) testen die Detailansicht nur indirekt.

- [ ] Neue Datei `tests/e2e/recipe-detail.spec.ts` erstellen
- [ ] **Testfall 1:** Vollständiges Rezept anzeigen (K1, K2, K3)
  - Rezept mit Titel, Kategorien, Zutaten und Anleitung anlegen
  - Detailseite aufrufen
  - H1 mit Titel prüfen
  - Kategorie-Tags prüfen
  - Abschnitt "Zutaten" sichtbar prüfen
  - Abschnitt "Anleitung" sichtbar prüfen
  - Schaltflächen "Bearbeiten", "Löschen", "Zurück zur Übersicht" prüfen
  - Metainformationen (Erstellungsdatum, letztes Bearbeitungsdatum) sichtbar prüfen
- [ ] **Testfall 2:** Rezept ohne optionale Felder (K1 Edge Case)
  - Rezept nur mit Titel und Kategorie anlegen (keine Zutaten, keine Anleitung)
  - Detailseite aufrufen
  - Prüfen, dass kein `<section class="ingredients">` vorhanden
  - Prüfen, dass kein `<section class="instructions">` vorhanden
  - Titel und Schaltflächen sind trotzdem sichtbar
- [ ] **Testfall 3:** Nicht vorhandene Rezept-ID liefert 404 (K4)
  - URL `/recipes/99999` direkt aufrufen
  - Verständliche Fehlermeldung prüfen
  - Link zurück zur Rezeptliste prüfen
- [ ] **Testfall 4:** DeepLink funktioniert (K3)
  - Rezept anlegen, ID merken
  - Direkt (ohne vorherige Navigation) `/recipes/{id}` aufrufen
  - Detailseite wird korrekt geladen
- [ ] **Testfall 5:** Erfolgs-Flash nach Bearbeiten (K5)
  - Rezept bearbeiten und speichern
  - Prüfen, dass auf der Detailansicht die `.success`-Klasse mit "Rezept erfolgreich aktualisiert" erscheint
- [ ] **Testfall 6:** Navigationslinks funktionieren (K2)
  - "Zurück zur Übersicht" führt zu `/`
  - "Bearbeiten" führt zu `/recipes/{id}/edit`
  - "Löschen" führt zu `/recipes/{id}/confirm-delete`

### Schritt 7: Cargo-Tests und Linting

- [ ] Alle Unit-Tests lokal ausführen: `cargo test` - alle Tests müssen grün sein
- [ ] Clippy ausführen: `cargo clippy -- -D warnings` - keine Warnungen
- [ ] Formatierung prüfen: `cargo fmt --check` - formatierungskonform
- [ ] E2E-Tests ausführen: `npm run test:e2e` - alle Tests müssen grün sein

---

## URL-Struktur

```
GET  /recipes/{id}           →  Detailansicht des Rezepts (zeigt alle Felder)
GET  /recipes/{id}?success=1 →  Detailansicht mit Erfolgs-Flash (nach Bearbeiten)
```

Alle anderen Routes (edit, confirm-delete, delete) sind in anderen Stories implementiert und werden hier nur als Linkziele referenziert.

---

## Abhängigkeiten

- **Story 01 (Rezept erstellen)** muss abgeschlossen sein - bildet Grundlage für Datenbankstruktur und CREATE-Handler
- **Story 02 (Rezept bearbeiten)** leitet nach erfolgreichem Speichern auf `/recipes/{id}?success=1` weiter - Detailansicht muss diesen Parameter verarbeiten
- **Story 03 (Rezept löschen)** verwendet Detailansicht als Ausgangspunkt für den Lösch-Button
- Technisch: Wiederverwendung von `get_recipe_by_id` aus `src/models/recipe_db.rs` (bereits vorhanden)
- Technisch: Wiederverwendung von `RecipeDetailTemplate` aus `src/templates.rs` (bereits vorhanden, ggf. erweitern)

---

## Test-Checkliste

### Unit-Tests (`cargo test`)

- [ ] `show_recipe_displays_title` - Titel im HTML-Body
- [ ] `show_recipe_displays_ingredients_section` - Zutaten sichtbar wenn befüllt
- [ ] `show_recipe_hides_ingredients_when_empty` - kein Zutaten-Abschnitt wenn leer
- [ ] `show_recipe_hides_instructions_when_empty` - kein Anleitungs-Abschnitt wenn leer
- [ ] `show_recipe_returns_404_for_missing_id` - Status 404
- [ ] `show_recipe_404_contains_back_link` - HTML-404-Seite mit Link
- [ ] `show_recipe_displays_success_flash` - `.success` bei `?success=1`
- [ ] `show_recipe_no_success_flash_without_param` - kein Flash ohne Parameter
- [ ] `show_recipe_displays_edit_link` - Link zu `/recipes/{id}/edit`
- [ ] `show_recipe_displays_delete_link` - Link zu `/recipes/{id}/confirm-delete`
- [ ] `format_date_formats_correctly` - Datum wird korrekt formatiert
- [ ] `format_date_handles_invalid_input` - Fallback für ungültige Eingabe

### E2E-Tests (`npm run test:e2e`)

- [ ] Vollständiges Rezept anzeigen (alle Felder sichtbar, alle Schaltflächen vorhanden)
- [ ] Rezept ohne optionale Felder (keine leeren Abschnitte)
- [ ] Nicht vorhandene ID → 404-Seite mit Rücklink
- [ ] DeepLink: Direktaufruf per URL ohne vorherige Navigation
- [ ] Erfolgs-Flash nach Bearbeiten (`?success=1`)
- [ ] Navigationslinks korrekt verknüpft

### Manuell zu prüfen

- [ ] Responsive Darstellung auf mobilem Gerät (iPhone-Viewport): Schaltflächen untereinander, volle Breite
- [ ] Sehr langer Text in Zutaten und Anleitung: kein horizontales Scrollen, Text bricht um
- [ ] Fokus-Navigation mit Tab-Taste funktioniert vollständig
- [ ] Schriftgröße ist auch auf kleinen Bildschirmen gut lesbar (min. 16px)
- [ ] Datumsanzeige ist verständlich formatiert (kein roher Timestamp)

---

## Offene Punkte

- **Datumsformatierung:** Es muss eine Entscheidung getroffen werden, ob die Formatierung im Rust-Code oder per Askama-Template-Filter erfolgt. Askama unterstützt Custom-Filter, was die Template-Logik sauber halten würde. Alternativ kann `format_date` direkt im Handler vor dem Template-Rendering aufgerufen werden. Empfehlung: Rust-seitige Formatierung im Handler für bessere Testbarkeit.

- **Flash-Meldung Auto-Dismiss:** Die Story erwähnt "verschwindet nach kurzer Zeit oder ist wegklickbar (bei JavaScript-Unterstützung)". Die einfachste konforme Lösung ist eine CSS-Animation (`animation: fadeOut 5s forwards; animation-delay: 2s`), die ohne zusätzliche JS-Abhängigkeit auskommt. Alternativ ein kleines `<button onclick="this.parentElement.remove()">×</button>` als Schließen-Button per inline-JS. Da HTMX im Projekt vorhanden ist, kann auch `hx-on:load="setTimeout(() => this.remove(), 5000)"` genutzt werden.

- **404-Fehlerseite HTML-Rendering:** Das bestehende `AppError`-Enum konvertiert in `IntoResponse` ohne Zugriff auf den Askama-Template-Engine. Für eine HTML-404-Seite muss entweder (a) das Template im `error.rs` direkt gerendert werden, oder (b) die Fehlerbehandlung in den Route-Handlern direkt eine HTML-Antwort erzeugen statt `AppError::NotFound` zurückzugeben. Empfehlung: Option (b) - der `show_recipe`-Handler gibt bei `None` direkt eine `NotFoundTemplate`-Antwort zurück, damit der Template-Kontext klar bleibt. Das `AppError::NotFound` bleibt für interne Fehler bestehen.
