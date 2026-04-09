# Implementierungsplan: Story 43 - Speichern-Button in Bearbeitungsansicht oben hinzufügen

## Technische Schritte

### Schritt 1: Styling für Formular-Header
- [ ] In `src/static/css/app.css` eine Klasse (z.B. `.form-header`) hinzufügen, um Überschrift und Button mittels Flexbox horizontal auszurichten und vertikal zu zentrieren.
- [ ] Sicherstellen, dass das Layout responsive ist (z.B. `gap` für Abstand).

### Schritt 2: UI-Anpassung in Template
- [ ] In `templates/recipes/form.html` den `<h1>`-Block in ein Container-Element (z.B. `<div class="form-header">`) einschließen.
- [ ] Einen weiteren `<button type="submit">` neben der Überschrift einfügen.
- [ ] Den Button mit den geforderten Attributen versehen:
  - Klasse für Styling (blau, klein).
  - Icon via `{% call icons::icon_check() %}`.
  - `aria-label="Rezept speichern"`.
- [ ] Sicherstellen, dass der Button innerhalb des `<form>`-Tags liegt, damit er den Standard-Submit auslöst.

### Schritt 3: E2E-Tests (Playwright)
- [ ] In `tests/e2e/recipes.spec.ts` (oder einer neuen Datei) folgende Testfälle implementieren:
  - **Test 1: Speichern über oberen Button**:
    - Given: In Bearbeitungsansicht eines Rezepts.
    - When: Feld ändern $\rightarrow$ Klick auf oberen Speichern-Button.
    - Then: Rezept gespeichert $\rightarrow$ Weiterleitung zur Detailansicht.
  - **Test 2: Sichtbarkeit und Platzierung**:
    - Given: In Bearbeitungsansicht eines Rezepts.
    - Then: Oberer Speichern-Button ist sichtbar neben der Überschrift.
    - And: Untere Buttons "Speichern" und "Abbrechen" sind weiterhin vorhanden.

### Schritt 4: Qualitätschecks & DoD
- [ ] `cargo fmt --check`
- [ ] `cargo clippy -- -D warnings`
- [ ] Prüfung der Barrierefreiheit (Tastatur-Navigation/Fokus) im Browser.

---

## URL-Struktur

Keine Änderungen an den URLs erforderlich.

---

## Abhängigkeiten

- Story 02 (Rezept bearbeiten) muss implementiert sein.
- Bestehende Icon-Komponenten in `templates/components/icons.html` werden wiederverwendet.

---

## Test-Checkliste

- [ ] E2E-Test: Speichervorgang über oberen Button erfolgreich.
- [ ] E2E-Test: Sichtbarkeit beider Button-Gruppen (oben und unten).
- [ ] Manueller Test: Button-Styling (blau, klein, Icon) entspricht Vorgabe.
- [ ] Manueller Test: Tastatur-Bedienbarkeit und `aria-label`.

---

## Offene Punkte

- Keine.
