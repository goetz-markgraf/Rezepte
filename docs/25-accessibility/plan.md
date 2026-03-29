# Implementierungsplan: Story 25 – WCAG 2.1 Level A Accessibility

## Analyse: Ist-Zustand der Barrierefreiheit

Nach gründlicher Inspektion aller Templates und des CSS wurden folgende Befunde ermittelt:

### Bereits gut umgesetzt (kein Handlungsbedarf)

- `<html lang="de">` ist in `base.html` vorhanden (K10)
- `<header>`, `<main>` semantische Landmark-Elemente vorhanden (K1)
- `<nav class="main-nav">` existiert, aber **kein `aria-label`** (Lücke, s.u.)
- `<nav class="category-filter" aria-label="Nach Kategorie filtern">` – korrekt ausgezeichnet (K1)
- `aria-pressed` auf Filter-Buttons (K12)
- `aria-label` auf Icon-only-Buttons in der Liste (Bearbeiten-Button: `aria-label="Rezept bearbeiten"`)
- `aria-hidden="true" focusable="false"` auf allen SVG-Icons (K5)
- `.visually-hidden` CSS-Klasse ist vorhanden und korrekt implementiert
- `focus-visible` Stile für `.btn-icon`, `.category-filter-btn`, `.sort-filter-btn`, `.reset-all-filters-btn`, `.inline-rating-btn` sind definiert (K4)
- Sternebewertung im Formular als Radio-Buttons (`<fieldset>` + `<legend>`) – bevorzugte ARIA-Lösung (K6)
- Radio-Label haben `aria-label` (z.B. "1 Stern", "2 Sterne")
- Inline-Rating-Buttons haben `aria-label` mit Status (z.B. "5 Sterne – aktiv (antippen zum Zurücksetzen)") (K5, K6)
- `aria-live="polite"` auf `#recipe-results` (HTMX Live-Updates) (K8)
- `aria-live="polite"` auf `#duplicate-hint` (HTMX Duplikat-Check) (K8)
- `role="alert"` auf Fehlercontainer in `merge.html` (K8)
- Suchfeld hat `<label for="q">` (K2)
- Formularfelder haben `<label for>` Verknüpfungen (title, ingredients, instructions, planned_date) (K2)
- `aria-describedby="planned_date_error"` auf Datumsfeld (K2) – aber kein Element mit id="planned_date_error" existiert!
- Kategorien-Checkboxen sind innerhalb von `<label>` eingebettet (implizites Label) (K2)
- `<article>` in `detail.html` mit `<header>`, `<section>`, `<footer>` (K1)
- `aria-label` auf Löschen/Abbrechen-Buttons in `confirm_delete.html` (K9)
- `.weekday-btn` hat `role="group"` + `aria-label` + `aria-pressed` (K3, K6)
- Tastatur-Fokus bei Datumsfeld-Widget korrekt behandelt

### Gefundene Lücken (Handlungsbedarf)

**L1 – `<nav>` in `base.html` fehlt `aria-label`**
Die Haupt-Navigation hat kein `aria-label`. Wenn eine Seite mehrere `<nav>` hat (z.B. Kategorie-Filter), können Screenreader die Navigationsbereiche nicht unterscheiden.
Betroffene Datei: `templates/base.html`

**L2 – Fehlermeldungen im Formular nicht mit Feldern verknüpft**
In `form.html` werden Fehler als globale Liste (`<div class="errors">`) angezeigt, aber nicht per `aria-describedby` oder `aria-errormessage` mit dem jeweiligen Feld verknüpft. Pflichtfelder haben kein `required`-Attribut. Das `aria-describedby="planned_date_error"` auf dem Datumsfeld referenziert ein nicht existierendes Element.
Betroffene Datei: `templates/recipes/form.html`

**L3 – Kategorien-Fieldset fehlt**
Die Kategorien-Checkboxen in `form.html` haben ein `<label>Kategorien *</label>`, aber kein `<fieldset>` mit `<legend>`. Eine Gruppe von Checkboxen braucht `<fieldset>/<legend>` für korrekten Screenreader-Kontext (analog zur Sternebewertung).
Betroffene Datei: `templates/recipes/form.html`

**L4 – Bearbeiten-Button in Rezeptliste fehlt Rezept-Kontext im `aria-label`**
Der Bearbeiten-Button in der Liste (`index.html`) hat `aria-label="Rezept bearbeiten"` – ohne den Rezepttitel. K9 fordert: "Bearbeiten-Buttons nennen den Kontext (z.B. `aria-label="Spaghetti Bolognese bearbeiten"`)"
Betroffene Datei: `templates/index.html`

**L5 – Erfolgsmeldung in `detail.html` nicht als Live-Region**
Die Erfolgsmeldung nach dem Speichern (`.success`) erscheint durch Redirect, wird also vom Server gerendert. Sie ist kein `role="status"` oder `role="alert"`, was für Screenreader hilfreich wäre.
Betroffene Datei: `templates/recipes/detail.html`

**L6 – Formular-Titelfeld hat kein `required`-Attribut**
Obwohl Titel und Kategorie Pflichtfelder sind (serverseitige Validierung), fehlt das HTML-`required`-Attribut auf `input[name="title"]` und es gibt keine programmatische Markierung für den Pflichtfeld-Status.
Betroffene Datei: `templates/recipes/form.html`

**L7 – Kontrast: Sekundäre Texte mit niedrigem Kontrast**
Mehrere Farbkombinationen können WCAG AA (4.5:1) nicht erfüllen:
- `.nichts-geplant`, `.tagesabschnitt-datum`, `.wochentag-datum`, `.merge-card-meta`: `#6b7280` auf `#ffffff` → ~4.6:1 (knapp OK)
- `.kw-label`, `.merge-field-label`: `#6b7280` auf weißem Hintergrund → ~4.6:1 (OK, aber nur knapp)
- Sterne-Rating-Buttons inaktiv: `#d1d5db` auf `#ffffff` → ~1.6:1 (NICHT OK für UI-Komponenten!)
- `filter_btn.active` weiß auf `#7c3aed` → muss geprüft werden (~6:1, OK)
- Inline-Rating inaktive Sterne `#d1d5db` auf `#ffffff` → ~1.6:1 (Kontrast für UI-Komponenten < 3:1)

**L8 – `aria-describedby="planned_date_error"` referenziert kein existierendes Element**
Im Formular referenziert `aria-describedby="planned_date_error"` ein nicht existierendes DOM-Element. Das ist ein ARIA-Fehler (ungültige ARIA-Referenz).
Betroffene Datei: `templates/recipes/form.html`

**L9 – Bestätigungs-Seite `confirm_delete.html` ist keine echte Dialog-Komponente**
Die Lösch-Bestätigung ist eine eigene Seite (kein modaler Dialog), daher entfällt K7 (Fokus-Falle). Allerdings fehlt auf der Seite ein logisches Überschriften-Gefüge: `h1` ist "Rezept löschen", was gut ist. Die Escape-Taste funktioniert per JavaScript. Dies ist grundsätzlich zugänglich, aber die Story-Anforderung K7 (Dialog-Semantik) bezieht sich auf die Seiten-Implementierung.

**L10 – Inline-Rating: Fehlt Zusammenfassung des aktuellen Wertes für Screenreader ohne `<form>`-Kontext**
Der `<div id="inline-rating">` hat `aria-label="{{ r }} von 5 Sternen"` nur wenn eine Bewertung vorhanden ist. Wenn noch keine Bewertung gesetzt ist (kein Rating), fehlt das `aria-label` komplett, Screenreader erhalten nur die Buttons ohne Kontext.
Betroffene Dateien: `templates/recipes/_inline_rating.html`, `templates/recipes/_inline_rating_heute.html`

**L11 – `<nav class="saved-filters">` hat `aria-label` aber kein semantisches `<nav>` – es ist ein `<div>`**
In `index.html` ist die gespeicherte-Filter-Liste als `<div class="saved-filters" aria-label="Gespeicherte Filter">`, kein `<nav>`. Das `aria-label` allein reicht ohne Landmark-Rolle nicht aus. Es sollte ein `<nav>` oder `role="navigation"` sein, oder als `<section>` mit `aria-labelledby`.

**L12 – Merge-Formular: Pflichtfelder ohne `required` und Radio-Gruppen ohne klare ARIA-Gruppierung**
In `merge.html` sind die Radio-Buttons für Konfliktfelder nicht in einem `<fieldset>/<legend>` eingebettet. Ein Screenreader kann die Gruppenzugehörigkeit nicht ableiten.
Betroffene Datei: `templates/recipes/merge.html`

**L13 – Bewertungsanzeige in Duplikaten-Übersicht: reine Sternzeichen ohne Screenreader-Text**
In `duplicates.html` hat `<span class="stars" aria-label="Bewertung">{{ paar.sterne_a() }}</span>` nur "Bewertung" als Label, aber nicht die konkrete Anzahl ("3 von 5 Sternen").
Betroffene Datei: `templates/recipes/duplicates.html`

**L14 – Bewertungsanzeige in Listenansicht: `aria-label` gibt nur Zahl an, nicht Kontext**
In `index.html` hat `<span class="recipe-stars recipe-stars-list" aria-label="{{ r }} von 5 Sternen">` bereits einen guten aria-label. Das ist korrekt, aber der Wert befindet sich innerhalb eines `<h2>` innerhalb eines `<a>`. Das führt dazu, dass die Sterne im Überschriftentext mit vorgelesen werden. Das ist akzeptabel, aber der `aria-label` auf dem Span wird von manchen Screenreadern nicht korrekt gehandhabt wenn er innerhalb des Link-Texts liegt.

---

## Technische Schritte

### Schritt 1: `base.html` – `aria-label` für Haupt-Navigation (L1)

- [ ] `aria-label="Hauptnavigation"` zu `<nav class="main-nav">` in `templates/base.html` hinzufügen

**Vorher:**
```html
<nav class="main-nav">
```
**Nachher:**
```html
<nav class="main-nav" aria-label="Hauptnavigation">
```

---

### Schritt 2: `form.html` – Semantik und Barrierefreiheit verbessern (L2, L3, L6, L8)

- [ ] Kategorien-Checkboxen in `<fieldset>/<legend>` einwickeln (L3)
- [ ] `required` auf `input[name="title"]` setzen (L6)
- [ ] `aria-required="true"` auf `input[name="title"]` für ältere AT (L6)
- [ ] `aria-describedby="planned_date_error"` entfernen oder das referenzierte Element erstellen (L8)
  - Lösung: `id="planned_date_error"` auf das globale Fehler-`<div>` setzen und alle Felder mit Fehler via `aria-describedby` verknüpfen
  - Alternative (einfacher): `aria-describedby` Attribut entfernen, da kein Element mit dieser ID existiert
  - **Entscheidung: Einfachere Lösung** – `aria-describedby="planned_date_error"` entfernen und durch korrekte globale Fehler-Region mit `role="alert"` ersetzen
- [ ] Fehler-Container `<div class="errors">` mit `role="alert"` und `id="form-errors"` ausstatten (L2)
- [ ] Pflichtfeld-Marker `*` für Screenreader erklären: `<span class="visually-hidden">Pflichtfeld</span>` bei Labels mit `*`

**Konkrete Änderungen in `templates/recipes/form.html`:**

```html
<!-- Fehler-Container -->
{% if !errors.is_empty() %}
<div class="errors" role="alert" id="form-errors">
    <ul>
        {% for error in errors %}
        <li>{{ error }}</li>
        {% endfor %}
    </ul>
</div>
{% endif %}

<!-- Titel-Feld -->
<div class="form-group">
    <label for="title">Titel <span aria-hidden="true">*</span><span class="visually-hidden">Pflichtfeld</span></label>
    <input
        type="text"
        id="title"
        name="title"
        value="{{ title }}"
        maxlength="100"
        required
        aria-required="true"
        hx-get="/recipes/check-duplicate"
        ...
    >
</div>

<!-- Kategorien als fieldset -->
<fieldset class="form-group">
    <legend>Kategorien <span aria-hidden="true">*</span><span class="visually-hidden">Pflichtfeld</span></legend>
    <div class="checkbox-group">
        {% for category in categories %}
        <label class="checkbox-label">
            <input type="checkbox" name="categories" value="{{ category }}"
                {% if selected_categories.contains(category) %}checked{% endif %}>
            {{ category }}
        </label>
        {% endfor %}
    </div>
</fieldset>

<!-- Datumsfeld: aria-describedby entfernen -->
<input
    type="text"
    id="planned_date"
    name="planned_date"
    placeholder="T.M.JJJJ"
    value="{{ planned_date }}"
    autocomplete="off"
>
```

---

### Schritt 3: `index.html` – Rezept-Kontext in Bearbeiten-Button (L4) und gespeicherte Filter (L11)

- [ ] `aria-label="Rezept bearbeiten"` durch `aria-label="{{ recipe.title }} bearbeiten"` ersetzen (L4)
- [ ] Gespeicherte-Filter-Container: `<div>` zu `<nav>` mit `aria-label` ändern oder `role="navigation"` hinzufügen (L11)

**Konkrete Änderung in `templates/index.html`:**

```html
<!-- Bearbeiten-Button mit Rezept-Kontext -->
<a href="/recipes/{{ recipe.id }}/edit" class="btn-icon" aria-label="{{ recipe.title }} bearbeiten">
    {% call icons::icon_pencil() %}
</a>

<!-- Gespeicherte Filter als nav -->
<nav class="saved-filters" aria-label="Gespeicherte Filter">
    ...
</nav>
```

---

### Schritt 4: `detail.html` – Erfolgsmeldung mit `role="status"` (L5)

- [ ] `<div class="success">` erhält `role="status"` damit Screenreader die Meldung ankündigen

**Konkrete Änderung in `templates/recipes/detail.html`:**

```html
{% if success %}
<div class="success" role="status">
    <span>Rezept erfolgreich aktualisiert</span>
    <button class="success-close" onclick="this.parentElement.remove()" aria-label="Meldung schließen">&times;</button>
</div>
{% endif %}
```

---

### Schritt 5: `_inline_rating.html` und `_inline_rating_heute.html` – Screenreader-Fallback wenn kein Rating (L10)

- [ ] `aria-label` auch setzen wenn kein Rating vorhanden ("Noch keine Bewertung")
- [ ] Beide Partial-Templates aktualisieren

**Konkrete Änderung in beiden `_inline_rating*.html`:**

```html
<!-- Vorher: nur bei vorhandenem Rating -->
{% if let Some(r) = rating %}aria-label="{{ r }} von 5 Sternen"{% endif %}

<!-- Nachher: immer ein aussagekräftiges Label -->
aria-label="{% if let Some(r) = rating %}{{ r }} von 5 Sternen{% else %}Noch keine Bewertung{% endif %}"
```

---

### Schritt 6: `merge.html` – Radio-Gruppen in `<fieldset>/<legend>` (L12)

Jede Konflikt-Gruppe (Titel, Kategorien, Zutaten, Anleitung, Bewertung, Datum) in `merge.html` hat Radio-Buttons ohne `<fieldset>/<legend>`. Screenreader können die Zugehörigkeit nicht ableiten.

- [ ] Jede `merge-conflict-row` mit Radio-Buttons bekommt `<fieldset>/<legend>` statt `<div>`
- [ ] Das `<div class="merge-field-label">` wird zur `<legend>` im `<fieldset>`

**Konzept (Beispiel für Titel-Konfliktzeile):**

```html
<!-- Vorher -->
<div class="merge-field-row merge-conflict-row">
    <div class="merge-field-label">Titel</div>
    <div class="merge-field-options">
        <label class="merge-option">...</label>
        <label class="merge-option">...</label>
    </div>
</div>

<!-- Nachher -->
<fieldset class="merge-field-row merge-conflict-row">
    <legend class="merge-field-label">Titel</legend>
    <div class="merge-field-options">
        <label class="merge-option">...</label>
        <label class="merge-option">...</label>
    </div>
</fieldset>
```

Alle 5 möglichen Konfliktzeilen (Titel, Kategorien, Zutaten, Anleitung, Bewertung, Datum) werden entsprechend angepasst.

---

### Schritt 7: `duplicates.html` – Konkrete Bewertungsanzahl im `aria-label` (L13)

- [ ] `aria-label="Bewertung"` durch aussagekräftiges Label ersetzen

**Konkrete Änderung in `templates/recipes/duplicates.html`:**

Dazu muss das Template Zugriff auf die konkrete Anzahl haben. Die `sterne_a()`/`sterne_b()`-Methode gibt Sternzeichen zurück. Das `aria-label` sollte numerisch sein. Entweder wird eine neue Methode `rating_a()` exponiert, die die Zahl zurückgibt, oder es wird ein visuell versteckter Text hinzugefügt:

```html
<!-- Lösung: aria-label mit Textbeschreibung über die Methode -->
{% if !paar.sterne_a().is_empty() %}
<span class="stars" aria-label="{{ paar.sterne_a() }}">{{ paar.sterne_a() }}</span>
{% endif %}
```

Alternativ (wenn Ratings als Zahlen verfügbar): Methoden `rating_label_a()` und `rating_label_b()` auf dem `DuplikatPaar`-Struct anlegen, die "3 von 5 Sternen" zurückgeben.

**Entscheidung: Methoden `rating_label_a()` / `rating_label_b()` anlegen** in `src/routes/recipes.rs` (oder wo `DuplikatPaar` definiert ist).

---

### Schritt 8: CSS – Kontrast für inaktive Sterne-Buttons verbessern (L7)

Die inaktiven Sterne (`#d1d5db` auf `#ffffff`) haben ein Kontrastverhältnis von ~1.6:1 – weit unter dem WCAG AA-Minimum von 3:1 für UI-Komponenten.

- [ ] Inaktive Stern-Farbe in `.inline-rating-btn` von `#d1d5db` auf `#9ca3af` ändern → ~2.5:1 (besser, aber noch unter 3:1)
- [ ] Für WCAG AA 3:1 bei UI-Komponenten: `#6b7280` auf `#ffffff` → ~4.6:1 (erfüllt 3:1 und 4.5:1)
- [ ] **Empfehlung:** `#9ca3af` (#9ca3af auf #fff = Kontrast ~2.5:1) reicht nicht. `#6b7280` verwenden: ~4.6:1

Da die Sterne informativ sind (der aktive Zustand wird durch Farbe UND `aria-label` kommuniziert), ist dies ein K12-Problem (Farbe als einziges Signal). Aber da die Buttons `aria-label` haben, ist die Farbeinfo nur ergänzend – kein WCAG-A-Verstoß. Trotzdem verbessern wir es für AA.

**Konkrete Änderungen in `src/static/css/app.css`:**

```css
/* Inaktive Sterne: besserer Kontrast (#6b7280 statt #d1d5db) */
.inline-rating-btn {
    color: #6b7280;  /* war: #d1d5db */
}

/* Sterne im Formular: gleiches Problem */
.star-rating-options label {
    color: #6b7280;  /* war: #d1d5db */
}
```

Gleichzeitig anpassen: Der Hover-Effekt bei `.inline-rating-form:hover .inline-rating-btn:not(:hover)` auf `#9ca3af` setzen, damit der visuelle Unterschied zwischen hover/nicht-hover erhalten bleibt.

---

### Schritt 9: CSS – `fieldset` Styling für Kategorien anpassen

Durch Schritt 2 wird das Kategorien-`<div class="form-group">` zu einem `<fieldset class="form-group">`. Fieldsets haben Browser-Default-Borders, die mit dem bestehenden Styling kollidieren.

- [ ] CSS für `fieldset.form-group` ergänzen, um Browser-Defaults zurückzusetzen
- [ ] `legend` analog zu `label` stylen

**Konkrete CSS-Ergänzungen:**

```css
fieldset.form-group {
    border: none;
    padding: 0;
    margin: 0;
    margin-bottom: 1.5rem;
}

fieldset.form-group legend {
    display: block;
    margin-bottom: 0.5rem;
    font-weight: 500;
    float: left;  /* für korrekte Darstellung im Fieldset */
    width: 100%;
}

/* Nach legend float: clearfix für den Inhalt */
fieldset.form-group legend + * {
    clear: both;
}
```

Gleiches für `.merge-field-row` wenn es als `<fieldset>` verwendet wird (Schritt 6).

---

### Schritt 10: E2E-Tests – Accessibility-Tests mit axe-core (Playwright)

axe-core ist nicht im aktuellen `package.json`. Es muss als Dev-Dependency hinzugefügt werden.

- [ ] `@axe-core/playwright` als Dev-Dependency installieren: `npm install --save-dev @axe-core/playwright`
- [ ] Neue Testdatei `tests/e2e/accessibility.spec.ts` erstellen

**Tests nach Story-Testfällen (aus `story.md`):**

**Testfall 1: Startseite – kein axe-Fehler**
```gherkin
Given: Mehrere Rezepte existieren in der Datenbank
When: Benutzer öffnet die Startseite "/"
Then: axe-core meldet keine Level-A-Violations
```

**Testfall 2: Detailansicht – kein axe-Fehler**
```gherkin
Given: Ein Rezept existiert in der Datenbank
When: Benutzer öffnet die Detailansicht
Then: axe-core meldet keine Level-A-Violations
```

**Testfall 3: Erstellen-Formular – kein axe-Fehler**
```gherkin
Given: Die App ist gestartet
When: Benutzer navigiert zu "Neues Rezept erstellen"
Then: axe-core meldet keine Level-A-Violations
```

**Testfall 4: Tastaturnavigation – Rezept erstellen**
```gherkin
Given: Die App ist gestartet
When: Benutzer navigiert per Tab zur "Neues Rezept"-Schaltfläche
And: Aktiviert sie per Enter
And: Füllt Titel per Tastatur aus
And: Wählt Kategorie per Tastatur (Space auf Checkbox)
And: Aktiviert "Speichern" per Enter
Then: Das neue Rezept erscheint in der Liste
```

**Testfall 5: Tastaturnavigation – Lösch-Dialog (Seite)**
```gherkin
Given: Ein Rezept existiert
When: Benutzer navigiert zur Detailansicht per Tastatur
And: Aktiviert "Löschen" per Enter
Then: Lösch-Bestätigungsseite erscheint
And: Escape-Taste navigiert zurück
When: Benutzer drückt "Abbrechen" per Enter
Then: Die Detailseite wird wieder angezeigt
```

**Testfall 6: Formular-Labels sind vorhanden**
```gherkin
Given: Die App ist gestartet
When: Benutzer öffnet das "Neues Rezept erstellen"-Formular
Then: Jedes Eingabefeld hat ein zugehöriges Label-Element (via axe)
And: Titelfeld hat required-Attribut
```

**Testfall 7: Bearbeiten-Button enthält Rezeptname**
```gherkin
Given: Ein Rezept "Spaghetti Bolognese" existiert
When: Benutzer öffnet die Startseite
Then: Der Bearbeiten-Button enthält "Spaghetti Bolognese bearbeiten" im aria-label
```

**Testfall 8: Inline-Rating ohne Bewertung hat aria-label**
```gherkin
Given: Ein Rezept ohne Bewertung existiert
When: Benutzer öffnet die Detailseite
Then: Das Inline-Rating-Widget hat aria-label "Noch keine Bewertung"
```

---

### Schritt 11: Seed-Dateien und Qualitätschecks

- [ ] Bestehende Seeds für axe-Tests wiederverwenden (z.B. `tests/seeds/recipe-list.sql`)
- [ ] Für Accessibility-Tests neuen Seed `tests/seeds/accessibility.sql` anlegen mit einem vollständigen Rezept (Titel, Kategorie, Bewertung, Datum, Zutaten, Anleitung)
- [ ] `cargo build` ausführen und auf Fehler prüfen
- [ ] `cargo clippy -- -D warnings` ausführen
- [ ] `cargo fmt --check` ausführen
- [ ] `npm run test:e2e` ausführen

---

## Implementierungsreihenfolge (empfohlen)

Die Schritte bauen nicht aufeinander auf und können weitgehend unabhängig umgesetzt werden. Empfohlene Reihenfolge:

1. **Schritt 10 zuerst (Setup axe-core)** – damit die axe-Tests nach jeder Template-Änderung verifiziert werden können
2. **Schritte 1–9 (Template- und CSS-Änderungen)** – in der obigen Reihenfolge
3. **Schritt 11 (Qualitätschecks)** – abschließend

---

## URL-Struktur

Keine neuen Endpunkte. Alle Änderungen betreffen bestehende Seiten:

```
GET  /                       → index.html (L4, L11 behoben)
GET  /recipes/new            → form.html (L2, L3, L6, L8 behoben)
GET  /recipes/{id}           → detail.html (L5, L10 behoben)
GET  /recipes/{id}/edit      → form.html (L2, L3, L6, L8 behoben)
GET  /recipes/{id}/confirm-delete → confirm_delete.html (keine Änderungen)
GET  /recipes/merge          → merge.html (L12 behoben)
GET  /recipes/duplicates     → duplicates.html (L13 behoben)
GET  /heute                  → heute.html (L10 behoben via _inline_rating_heute.html)
```

Mögliche neue Route für `rating_label_a/b` wenn als Template-Methode implementiert:
```
Keine neuen HTTP-Endpunkte
```

---

## Abhängigkeiten

- Alle vorherigen Stories (01–23) müssen implementiert sein (Story ist Querschnittsaufgabe)
- `@axe-core/playwright` muss als npm-Paket installiert werden
- Keine Datenbankschema-Änderungen
- Keine neuen Rust-Structs (außer evtl. Helper-Methoden auf `DuplikatPaar` für L13)

---

## Test-Checkliste

- [ ] E2E-Test: axe-core findet keine Level-A-Violations auf der Startseite
- [ ] E2E-Test: axe-core findet keine Level-A-Violations auf der Detailseite
- [ ] E2E-Test: axe-core findet keine Level-A-Violations auf dem Erstellen-Formular
- [ ] E2E-Test: Tastaturnavigation – Rezept erstellen ohne Maus möglich
- [ ] E2E-Test: Tastaturnavigation – Lösch-Seite per Tastatur navigierbar (Escape, Enter)
- [ ] E2E-Test: Formular-Labels programmatisch mit Feldern verknüpft
- [ ] E2E-Test: Bearbeiten-Button enthält Rezeptname im aria-label
- [ ] E2E-Test: Inline-Rating ohne Bewertung hat aria-label "Noch keine Bewertung"
- [ ] Manueller Test: Tab-Reihenfolge auf allen Hauptseiten logisch und vollständig
- [ ] Manueller Test: Alle fokussierbaren Elemente haben sichtbaren Fokus-Indikator
- [ ] Manueller Test: VoiceOver (macOS) auf Startseite, Formular, Detailseite
- [ ] `cargo build` ohne Fehler
- [ ] `cargo clippy -- -D warnings` ohne Warnungen
- [ ] `cargo fmt --check` erfolgreich
- [ ] `npm run test:e2e` alle Tests grün

---

## Offene Punkte

- **L13 Implementierungsdetail:** `rating_label_a()` / `rating_label_b()` als Methoden auf `DuplikatPaar` oder als Template-Logik lösen? Wenn das Struct in `src/routes/recipes.rs` oder einem Modell-File definiert ist, dort Methode anlegen, die "N von 5 Sternen" oder "" zurückgibt.
- **axe-core Playwright-Integration:** `@axe-core/playwright` liefert `checkA11y()` – prüfen ob WCAG 2.1 Level A als Ruleset konfigurierbar ist. Falls nicht, werden alle axe-Regeln geprüft (ist für unsere Zwecke ausreichend und strenger).
- **Kontrast-Check:** Genaue Kontrastverhältnisse sollten mit einem Kontrast-Checker (z.B. WebAIM Contrast Checker) verifiziert werden, bevor CSS-Werte finalisiert werden.
