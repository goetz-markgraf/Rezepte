# Implementierungsplan: Story 34 - Suche "Länger nicht gemacht" per Klick in der Wochenübersicht

## Übersicht

Diese Story fügt einen Button in der Wochenübersicht hinzu, der direkt zur Suche "Länger nicht gemacht" navigiert. Dies ermöglicht es dem User, schnell Rezepte zu finden, die schon lange nicht mehr gekocht wurden, um Lücken im Wochenplan zu füllen.

**Wichtig:** Die Funktionalität "Länger nicht gemacht" existiert bereits (Story 9). Diese Story fügt nur einen direkten Zugriff aus der Wochenübersicht hinzu.

---

## Technische Schritte

### Schritt 1: Template-Änderungen (Wochenvorschau)

**Datei:** `templates/wochenvorschau.html`

- [ ] Button "Länger nicht gemacht" oberhalb der Wochenliste hinzufügen
- [ ] Der Button ist ein Link zur Startseite mit Query-Parameter `?filter=laenger-nicht-gemacht`
- [ ] Optional: Aktuelle Woche als `return_week` Parameter mitgeben für spätere Rückkehr
- [ ] Icon (SVG) für den Button hinzufügen (z.B. ein Uhr-Icon aus `templates/components/icons.html` wiederverwenden oder ähnliches)
- [ ] ARIA-Label für Barrierefreiheit: `"Rezepte anzeigen, die länger nicht gemacht wurden"`

**Code-Vorlage:**
```html
<div class="wochenvorschau-toolbar">
    <a href="/?filter=laenger-nicht-gemacht" 
       class="btn-secondary not-made-button" 
       aria-label="Rezepte anzeigen, die länger nicht gemacht wurden">
        <svg ...></svg>
        <span>Länger nicht gemacht</span>
    </a>
</div>
```

### Schritt 2: CSS-Styling

**Datei:** `src/static/css/app.css`

- [ ] CSS-Klasse `.wochenvorschau-toolbar` für den Button-Container erstellen
- [ ] Button-Styling konsistent mit anderen sekundären Buttons (z.B. Filter-Buttons auf der Startseite)
- [ ] Responsive Design: Button soll auf Mobile gut aussehen
- [ ] Hover- und Focus-States definieren
- [ ] Optional: Animation für bessere UX

**Vorlage für CSS:**
```css
.wochenvorschau-toolbar {
    margin-bottom: 1rem;
    display: flex;
    justify-content: flex-start;
}

.not-made-button {
    display: inline-flex;
    align-items: center;
    gap: 0.5rem;
    padding: 0.5rem 1rem;
    /* Weitere Styles analog zu bestehenden Buttons */
}
```

### Schritt 3: Template-Datenstruktur erweitern (optional)

**Datei:** `src/templates.rs`

- [ ] `WochenvorschauTemplate` um `not_made_filter_url: String` erweitern
- [ ] URL wird in `wochenvorschau_handler` generiert

**Hinweis:** Dies ist optional, da die URL statisch sein kann (`/?filter=laenger-nicht-gemacht`).

### Schritt 4: Route-Handler anpassen (optional)

**Datei:** `src/routes/wochenvorschau.rs`

- [ ] Falls wir die aktuelle Woche an die Suche übergeben wollen:
  - Query-Parameter `return_week=YYYY-WNN` zur URL hinzufügen
  - Dies ermöglicht später: "Zurück zur Woche X" Button in der Suche

- [ ] `WochenvorschauTemplate` mit der generierten URL befüllen

### Schritt 5: Startseite anpassen - "Zurück zur Wochenübersicht" Link

**Datei:** `templates/index.html`

- [ ] Prüfen ob `return_week` Parameter vorhanden ist
- [ ] Falls ja: "Zurück zur Wochenübersicht" Button anzeigen
- [ ] Link zu `/wochenvorschau?week=YYYY-WNN`

**Datei:** `src/routes/recipes.rs`

- [ ] Query-Parameter `return_week` in `IndexQuery` hinzufügen
- [ ] Template `IndexTemplate` um `return_week_url: Option<String>` erweitern
- [ ] URL bauen und an Template übergeben

### Schritt 6: E2E-Tests

**Datei:** `tests/e2e/wochenvorschau-not-made.spec.ts`

- [ ] Test 1: Button ist in der Wochenübersicht sichtbar
  - Navigiere zu `/wochenvorschau`
  - Prüfe ob Button "Länger nicht gemacht" existiert
  - Prüfe ARIA-Label

- [ ] Test 2: Klick öffnet Suche mit vorbelegtem Filter
  - Klicke auf den Button
  - Prüfe URL enthält `filter=laenger-nicht-gemacht`
  - Prüfe ob Suche geladen ist
  - Prüfe ob Filter aktiv ist (z.B. Button als aktiv markiert)

- [ ] Test 3: Rückkehr zur Wochenübersicht (wenn implementiert)
  - Klicke auf Button in Woche X
  - Suche wird geöffnet
  - Klicke "Zurück zur Wochenübersicht"
  - Prüfe ob Woche X angezeigt wird

- [ ] Test 4: Barrierefreiheit
  - Button ist per Tastatur erreichbar
  - Screenreader-Label korrekt

**Datei:** `tests/e2e/search-from-wochenvorschau.spec.ts` (falls neue Datei gewünscht)

### Schritt 7: Integrationstests (Rust)

**Datei:** `tests/wochenvorschau.rs`

- [ ] Test: Wochenvorschau enthält Link zur Suche
  - GET `/wochenvorschau`
  - Prüfe Response-Body enthält Link zu `/?filter=laenger-nicht-gemacht`

**Datei:** `tests/recipe_not_made_filter.rs` (erweitern)

- [ ] Test: DeepLink mit return_week Parameter wird korrekt verarbeitet (falls implementiert)

---

## URL-Struktur

```
# Bestehend
GET  /wochenvorschau              → Wochenübersicht (aktuelle Woche)
GET  /wochenvorschau?week=YYYY-WNN → Wochenübersicht (spezifische Woche)
GET  /?filter=laenger-nicht-gemacht → Suche mit "Länger nicht gemacht" Filter

# Neu (optional)
GET  /?filter=laenger-nicht-gemacht&return_week=YYYY-WNN → Suche mit Rückkehr-Context
```

---

## Abhängigkeiten

- Story 9: Filter "Länger nicht gemacht" (muss implementiert sein - ist bereits vorhanden)
- Story 18: Wochenvorschau (muss implementiert sein - ist bereits vorhanden)
- Story 33: Wochenübersicht Navigation (falls wir die aktuelle Woche weitergeben)

**Keine neuen Datenbank-Änderungen nötig.**
**Keine neuen Backend-Filter nötig - bestehende Funktionalität wird wiederverwendet.**

---

## Test-Checkliste

### Unit-Tests (Rust)
- [ ] `wochenvorschau_enthaltet_link_zur_not_made_suche()` - Link ist im HTML vorhanden
- [ ] `index_handler_erkennt_return_week_parameter()` - Parameter wird korrekt geparsed (optional)

### Integrationstests (Rust)
- [ ] GET `/wochenvorschau` enthält Link zu `/?filter=laenger-nicht-gemacht`
- [ ] Button hat korrekte CSS-Klasse und ARIA-Attribute

### E2E-Tests (Playwright)
- [ ] **T1:** Button ist auf Wochenübersicht sichtbar und erreichbar
- [ ] **T2:** Klick navigiert zu `/` mit `filter=laenger-nicht-gemacht`
- [ ] **T3:** Filter ist in der Suche aktiv
- [ ] **T4:** Button ist per Tastatur erreichbar
- [ ] **T5:** Button hat korrektes ARIA-Label (Barrierefreiheit)
- [ ] **T6:** Responsive Design auf Mobile (optional)

### Manueller Test
- [ ] Button sieht gut aus und ist intuitiv zu finden
- [ ] Hover-Effekt funktioniert
- [ ] Zurück-Navigation funktioniert (Browser-Back)
- [ ] Keine JavaScript-Fehler in der Konsole

---

## Implementierungs-Reihenfolge

1. **Template anpassen** (`wochenvorschau.html`) - Button hinzufügen
2. **CSS hinzufügen** - Button schön stylen
3. **E2E-Test schreiben** (T1-T3) - Sicherstellen dass der Flow funktioniert
4. **Barrierefreiheit testen** (T4-T5)
5. **Optionale Erweiterung:** Return-Week Feature implementieren (Schritt 5)
6. **Alle Tests laufen lassen** - `cargo test` und `npm run test:e2e`
7. **Code-Qualität prüfen** - `cargo clippy`, `cargo fmt`

---

## Offene Punkte / Entscheidungen

### Entscheidung 1: Soll die aktuelle Woche übergeben werden?
**Option A (Empfohlen):** Einfacher Button ohne Return-Parameter
- Vorteil: Einfach zu implementieren, weniger Code
- Nachteil: User muss nach Zuweisung manuell zurück zur Wochenübersicht

**Option B:** Button mit Return-Week Parameter
- Vorteil: Besserer Workflow, direkte Rückkehr zur richtigen Woche
- Nachteil: Mehr Code, Änderungen an der Startseite nötig

**Empfehlung:** Option A für MVP, Option B als spätere Verbesserung.

### Entscheidung 2: Icon für den Button
- Uhr/History-Icon: 🕐 (Unicode) oder SVG aus `templates/components/icons.html`
- Oder Text nur ohne Icon (einfacher)

### Entscheidung 3: Position des Buttons
- **A:** Über der Wochenliste (wie in der Story beschrieben)
- **B:** In der Toolbar neben "Zur Rezeptliste"
- **C:** Beides

**Empfehlung:** Option A - direkt sichtbar ohne scrollen.

---

## Akzeptanzkriterien aus Story.md

- [ ] **K1:** Button "Länger nicht gemacht" ist in der Wochenübersicht sichtbar
- [ ] **K2:** Klick öffnet Suche mit vorbelegtem Filter
- [ ] **K3:** Direkte Zuweisung zum Tag möglich (bereits implementiert in Rezept-Details)
- [ ] **K4:** Keine Datenverlust (bereits gegeben)
- [ ] **K5:** Performance < 200ms (seitlicher Link, kein DB-Query)
- [ ] **K6:** Barrierefreiheit (ARIA-Label, Tastatur)

---

## Schätzung

- **Template-Änderung:** 30 Min
- **CSS-Styling:** 30 Min
- **E2E-Tests:** 45 Min
- **Integrationstests:** 15 Min
- **Manuelles Testen & Feinschliff:** 30 Min

**Gesamt:** ~2.5 Stunden (ohne optionales Return-Week Feature)
**Mit Return-Week Feature:** +1.5 Stunden = ~4 Stunden

---

## Notizen

- Die bestehende Funktionalität in `recipes.rs` (`filter_recipes_not_made_recently`) wird wiederverwendet
- Keine Datenbank-Migration nötig
- Keine neuen Dependencies nötig
- Design sollte konsistent mit anderen Buttons sein (z.B. Filter-Buttons auf Startseite)
