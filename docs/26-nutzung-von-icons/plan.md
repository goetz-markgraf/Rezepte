# Implementierungsplan: Story 26 - Nutzung von Icons

## Übersicht

Icons werden als Inline-SVG aus der Lucide-Bibliothek in die bestehenden Askama-Templates
eingebettet. Eine zentrale Makro-Datei (`templates/components/icons.html`) hält alle Icons
und wird von den einzelnen Templates importiert. Keine Datenbankänderungen nötig.

---

## Technische Schritte

### Schritt 1: Lucide-SVG-Quellen beschaffen und prüfen

- [ ] Lucide-SVGs für folgende Icons aus dem Lucide-Repository (MIT-Lizenz) entnehmen:
  `pencil`, `trash-2`, `plus`, `arrow-left`, `search`, `check`, `x`, `star`, `home`
- [ ] Jeden SVG-Pfad auf korrekte Lizenzangabe prüfen (MIT, keine Einschränkungen)
- [ ] SVG-Markup auf minimale Attribute reduzieren:
  `xmlns`, `width`, `height`, `viewBox`, `fill`, `stroke`, Pfad-Daten

### Schritt 2: Icon-Makro-Datei anlegen (`templates/components/icons.html`)

- [ ] Datei `templates/components/icons.html` neu anlegen
- [ ] Für jedes Icon ein Askama-Makro definieren, das das Inline-SVG rendert:
  ```
  {% macro icon_pencil() %}
  <svg ... aria-hidden="true" focusable="false">...</svg>
  {% endmacro %}
  ```
- [ ] Attribute für alle Icons:
  - `width="20" height="20"` als Standard
  - `aria-hidden="true"` (Screenreader-Unterdrückung, da Label am Button liegt)
  - `focusable="false"` (IE/Edge-Fix, verhindert Tab-Fokus auf SVG)
  - `class="icon"` für CSS-Basis-Styling
- [ ] Sonder-Makro für Stern-Icons mit gefüllt/ungefüllt-Variante:
  ```
  {% macro icon_star(filled) %}
  <svg ... fill="{% if filled %}currentColor{% else %}none{% endif %}">...</svg>
  {% endmacro %}
  ```

### Schritt 3: CSS-Klassen für Icon-Buttons (`src/static/css/app.css`)

- [ ] Klasse `.icon` hinzufügen:
  ```css
  .icon {
      display: inline-block;
      vertical-align: middle;
      flex-shrink: 0;
  }
  ```
- [ ] Klasse `.btn-icon` hinzufügen (nur Icon, kein sichtbarer Text, große Touch-Fläche):
  ```css
  .btn-icon {
      display: inline-flex;
      align-items: center;
      justify-content: center;
      min-width: 44px;
      min-height: 44px;
      padding: 0.5rem;
      background: none;
      border: none;
      border-radius: 0.375rem;
      cursor: pointer;
      color: inherit;
      text-decoration: none;
  }
  .btn-icon:hover { background-color: var(--bg-color); }
  .btn-icon:focus-visible { outline: 2px solid var(--primary-color); outline-offset: 2px; }
  ```
- [ ] Klasse `.btn-icon-text` hinzufügen (Icon + sichtbarer Text):
  ```css
  .btn-icon-text {
      display: inline-flex;
      align-items: center;
      gap: 0.5rem;
  }
  ```
- [ ] Klasse `.star-rating` für Sterne-Bewertung:
  ```css
  .star-rating { display: inline-flex; gap: 0.125rem; color: #f59e0b; }
  .star-filled { fill: currentColor; }
  .star-empty  { fill: none; }
  ```

### Schritt 4: Template `templates/index.html` anpassen

- [ ] Makro-Datei importieren: `{% import "components/icons.html" as icons %}`
- [ ] "Neues Rezept"-Button: Icon `plus` + Text ergänzen:
  ```html
  <a href="/recipes/new" class="btn-primary btn-icon-text">
      {% call icons::icon_plus() %}Neues Rezept
  </a>
  ```
- [ ] Bearbeiten-Button in Rezeptliste: Nur Icon + `aria-label`:
  ```html
  <a href="/recipes/{{ recipe.id }}/edit" class="btn-icon" aria-label="Rezept bearbeiten">
      {% call icons::icon_pencil() %}
  </a>
  ```

### Schritt 5: Template `templates/recipes/detail.html` anpassen

- [ ] Makro-Datei importieren
- [ ] "Bearbeiten"-Button: Icon `pencil` + Text:
  ```html
  <a href="/recipes/{{ id }}/edit" class="btn-primary btn-icon-text">
      {% call icons::icon_pencil() %}Bearbeiten
  </a>
  ```
- [ ] "Zurück zur Übersicht"-Link: Icon `arrow-left` + Text:
  ```html
  <a href="/" class="btn-secondary btn-icon-text">
      {% call icons::icon_arrow_left() %}Zurück zur Übersicht
  </a>
  ```
- [ ] "Löschen"-Button: Icon `trash-2` + Text:
  ```html
  <a href="/recipes/{{ id }}/confirm-delete" class="btn-danger btn-icon-text">
      {% call icons::icon_trash_2() %}Löschen
  </a>
  ```

### Schritt 6: Template `templates/recipes/confirm_delete.html` anpassen

- [ ] Makro-Datei importieren
- [ ] "Abbrechen"-Button: Icon `x` + Text
- [ ] "Wirklich löschen"-Button: Icon `trash-2` + Text

### Schritt 7: Template `templates/recipes/form.html` anpassen

- [ ] Makro-Datei importieren
- [ ] "Rezept speichern"-Button: Icon `check` + Text
- [ ] "Abbrechen"-Link: Icon `x` + Text

### Schritt 8: E2E-Tests schreiben (`tests/e2e/icons.spec.ts`)

- [ ] Datei `tests/e2e/icons.spec.ts` erstellen mit folgenden Tests:

  **Test 1: Bearbeiten-Button hat Icon und Accessibility-Label**
  - Rezept anlegen, Startseite öffnen
  - Prüfen: Bearbeiten-Button (`a[aria-label="Rezept bearbeiten"]`) enthält `svg`-Element
  - Prüfen: `aria-label` ist vorhanden

  **Test 2: Löschen-Icon-Button öffnet Bestätigungsseite**
  - Rezept anlegen, Detailansicht öffnen
  - Löschen-Button klicken
  - Prüfen: Bestätigungsseite erscheint
  - Prüfen: Buttons auf Bestätigungsseite enthalten SVG-Elemente

  **Test 3: "Neues Rezept"-Button hat Plus-Icon**
  - Startseite öffnen
  - Prüfen: `a[href="/recipes/new"]` enthält `svg`-Element

  **Test 4: Suche-Icon im Suchfeld sichtbar** *(falls Suchfeld implementiert ist)*
  - Startseite öffnen
  - Prüfen: SVG-Icon neben oder im Suchfeld sichtbar

  **Test 5: Icon-Buttons per Tastatur erreichbar**
  - Startseite öffnen, Rezept vorhanden
  - Mit Tab durch Seite navigieren
  - Prüfen: Bearbeiten-Button erhält Fokus (`focus-visible`-Outline sichtbar)

### Schritt 9: Build und Tests ausführen

- [ ] `cargo build` – keine Compiler-Fehler
- [ ] `cargo clippy -- -D warnings` – keine Warnungen
- [ ] `cargo fmt --check` – korrektes Format
- [ ] `npm run test:e2e` – alle Playwright-Tests grün

---

## URL-Struktur

Keine neuen Routen. Alle Änderungen sind rein auf Template- und CSS-Ebene.

```
GET  /                     →  Startseite mit Icon-Buttons (unverändert)
GET  /recipes/{id}         →  Detailansicht mit Icon-Buttons (unverändert)
GET  /recipes/{id}/edit    →  Bearbeiten-Formular mit Icon-Buttons (unverändert)
GET  /recipes/{id}/confirm-delete  →  Bestätigungsseite mit Icon-Buttons (unverändert)
```

---

## Abhängigkeiten

- Story 01–05 (Rezept-CRUD und Liste) müssen implementiert sein – Icons werden den
  bestehenden UI-Elementen hinzugefügt
- Keine neuen Cargo-Dependencies
- Keine Datenbankmigrationen
- Lucide-SVG-Code: MIT-lizenziert, wird statisch als Text in Templates eingebettet

---

## Technische Entscheidung: Askama-Makros statt Partials

Askama unterstützt `{% macro %}` / `{% call %}` (analog Jinja2-Makros). Dies ist die
bevorzugte Methode für wiederverwendbare SVG-Schnipsel:

- **Vorteil:** Compile-time geprüft, kein Laufzeitfehler
- **Vorteil:** Import pro Template explizit (`{% import ... as icons %}`)
- **Vorteil:** Parameter möglich (z.B. `filled` bei Sternen)
- **Alternative wäre:** Askama `include` für jedes Icon einzeln – zu viele Dateien

---

## Test-Checkliste

- [ ] E2E-Test: Bearbeiten-Button in Liste enthält `svg` und `aria-label`
- [ ] E2E-Test: Löschen-Button auf Detailseite enthält `svg`, navigiert zur Bestätigungsseite
- [ ] E2E-Test: Bestätigungsseite-Buttons enthalten SVG-Icons
- [ ] E2E-Test: "Neues Rezept"-Button enthält `svg`
- [ ] E2E-Test: Icon-Buttons sind per Tab erreichbar
- [ ] Manueller Test: Icons erscheinen auf Desktop, Tablet (DevTools-Simulation) und Handy
- [ ] Manueller Test: Fokus-Indikator sichtbar bei Tastaturnavigation
- [ ] Manueller Test: Touch-Fläche mindestens 44×44px (Chrome DevTools → Mobile-Simulation)
- [ ] Manueller Test: SVGs bei 20×20px noch erkennbar

---

## Offene Punkte

- Kategorie-Filter-Buttons (K6): Falls noch kein Suchfilter in der UI vorhanden, entfällt
  dieser Schritt. Sobald implementiert, Icons nachträglich ergänzen.
- Bewertungssterne (K5): Sterne-Icons sind im Plan enthalten, aber nur falls die
  Rating-Anzeige in Templates existiert – andernfalls bei Rating-Story implementieren.
- Bearbeiten-Button in Liste: Laut Story-K2 als Icon-only (`.btn-icon`) mit `aria-label`,
  platzsparend für mobile Nutzung.
