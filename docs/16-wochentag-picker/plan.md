# Implementierungsplan: Story 16 - Wochentag-Picker für intuitive Datumsauswahl

## Technische Analyse

### Ist-Zustand

- `planned_date`-Feld in der DB und im Rust-Modell existiert bereits (Story 28 abgeschlossen)
- `RecipeFormTemplate` hat `planned_date: String` (deutsches Format `T.M.JJJJ` oder leer)
- `templates/recipes/form.html` zeigt Datumsfeld mit Texteingabe + Kalender-Icon
- Bestehender JavaScript-Block in `form.html` verarbeitet bereits den nativen Date-Picker-Change
- CSS-Klassen für Datumsfeld vorhanden: `.date-input-group`, `.date-picker-hidden`, `.date-picker-btn`
- Pattern für `btn-icon` (44px Tap-Target) bereits in `app.css` vorhanden
- Keine Datenbank-Änderungen notwendig (kein neues DB-Feld)
- Keine Rust-Backend-Änderungen notwendig (Datum kommt als String `T.M.JJJJ` wie bisher)

### Scope dieser Story

Diese Story ist **rein clientseitig** (HTML + CSS + Vanilla JS). Kein Rust-Code muss geändert werden. Die sieben Wochentag-Buttons sind eine komfortierende Eingabehilfe, die das Textfeld `planned_date` befüllen — identisch zum Date-Picker. Der Server empfängt weiterhin einen String im Format `T.M.JJJJ`.

### Entscheidung: JS-only Widget mit Progressive Enhancement

- Ohne JS: Keine Buttons sichtbar, nur das Textfeld funktioniert (wie bisher)
- Die Buttons werden per JavaScript eingefügt (nicht im HTML gerendert), damit kein `display:none` nötig ist und das Markup ohne JS sauber bleibt
- Alternative: Buttons im HTML mit `display:none` und per JS einblenden — **Entscheidung: Buttons per JS einfügen**, da sauberer und kein toter HTML-Ballast ohne JS

### Berechnung "nächste Woche"

Die Berechnung erfolgt nach der ISO-Wochendefinition (Montag = Wochenanfang):

```
today          = new Date()
dayOfWeek      = (today.getDay() + 6) % 7   // 0=Mo, 1=Di, ..., 6=So
nextMonday     = today + (7 - dayOfWeek) Tage  // immer nächsten Montag
targetDate     = nextMonday + offset           // offset: Mo=0, Di=1, ..., So=6
```

Beispiel: Heute ist Mittwoch (dayOfWeek=2), nächster Montag = heute + 5 Tage.

"Nächste Woche" bedeutet: immer die Woche nach der aktuellen ISO-Woche, unabhängig vom heutigen Wochentag. Auch wenn heute Montag ist, ergibt "Mo" den Montag in 7 Tagen (nicht heute).

### Aktiv-Markierung beim Öffnen des Formulars

Wenn das Formular mit einem vorhandenen `planned_date` geöffnet wird: Das Datum wird geparst und geprüft, ob es auf einen Wochentag der nächsten Woche fällt. Falls ja, wird der entsprechende Button aktiv markiert. Die Prüfung erfolgt clientseitig per JS beim DOMContentLoaded-Event.

---

## Technische Schritte

### Schritt 1: JavaScript-Logik (Datumberechnung und Widget)

Implementierung des Wochentag-Picker-Widgets als Vanilla-JS-Modul direkt im `<script>`-Block in `form.html`:

- [ ] Funktion `calculateNextWeekday(offset)` implementieren:
  - `offset`: 0=Mo, 1=Di, 2=Mi, 3=Do, 4=Fr, 5=Sa, 6=So
  - Berechnet `today`, ermittelt `dayOfWeek` nach ISO-Schema: `(today.getDay() + 6) % 7`
  - `daysUntilNextMonday = 7 - dayOfWeek` (wenn heute Mo=0: 7 Tage, Di=1: 6 Tage, ..., So=6: 1 Tag)
  - `nextMonday = today + daysUntilNextMonday Tage`
  - `targetDate = nextMonday + offset Tage`
  - Gibt Datum als String `T.M.JJJJ` zurück (z.B. `"2.4.2026"`)

- [ ] Funktion `parseGermanDate(str)` implementieren:
  - Parst `T.M.JJJJ` oder `TT.MM.JJJJ` zu einem Date-Objekt
  - Gibt `null` bei ungültigem Format zurück

- [ ] Funktion `detectActiveWeekday(dateStr)` implementieren:
  - Parst den aktuellen Textfeldwert
  - Vergleicht mit allen 7 möglichen Wochentag-Daten der nächsten Woche
  - Gibt den passenden offset (0-6) zurück oder -1 wenn kein Match

- [ ] Funktion `createWeekdayPicker(textInput)` implementieren:
  - Erstellt Container-`<div class="weekday-picker">` mit:
    - `<span class="weekday-picker-label">Nächste Woche</span>`
    - Sieben `<button type="button">` Elemente: "Mo", "Di", "Mi", "Do", "Fr", "Sa", "So"
    - `aria-label="[Wochentag] nächste Woche wählen"` je Button
    - `aria-pressed="false"` je Button
  - Fügt den Container **nach** dem `.date-input-group`-Container ein
  - Bindet Event-Listener an jeden Button (Click-Handler):
    - Wenn Button bereits aktiv (`aria-pressed="true"`): Textfeld leeren, alle Buttons demarkieren
    - Sonst: `calculateNextWeekday(offset)` aufrufen, Textfeld befüllen, Button als aktiv markieren
  - Bindet `input`-Event an `textInput`:
    - Bei jeder Änderung `detectActiveWeekday(textInput.value)` aufrufen
    - Entsprechenden Button markieren oder alle demarkieren

- [ ] Initialisierung beim Laden:
  - Nach DOMContentLoaded: `createWeekdayPicker(textInput)` aufrufen
  - Initial `detectActiveWeekday` prüfen und ggf. Button aktiv markieren (für Edit-Formular mit vorhandenem Datum)

**TDD-Ansatz für den JS-Teil:**
Da es kein Build-System gibt und kein Node.js für Unit-Tests eingesetzt werden soll, werden die JS-Funktionen direkt per Playwright-E2E-Tests validiert (Browser-seitig ausgeführt). Die Logik wird über `page.evaluate()` testbar sein.

### Schritt 2: HTML-Template anpassen (`templates/recipes/form.html`)

- [ ] Den vorhandenen `<script>`-Block erweitern: Neue Funktionen und `createWeekdayPicker`-Initialisierung hinzufügen
  - Der bestehende Date-Picker-Handler (Kalender-Icon → Change-Event → Textfeld) bleibt unverändert
  - Neuer Code wird **am Ende** des IIFE (Immediately Invoked Function Expression) eingefügt, nach dem bestehenden Date-Picker-Code
  - Kein `display:none` in HTML nötig — Buttons werden per JS eingefügt (Progressive Enhancement)

- [ ] Sicherstellen, dass der Script-Block korrekt strukturiert ist:
  - Alle Variablen innerhalb der IIFE (kein globaler Namespace-Leak)
  - `var` statt `let`/`const` für maximale Browser-Kompatibilität (konsistent mit bestehendem Code)

### Schritt 3: CSS-Styling (`src/static/css/app.css`)

- [ ] CSS-Klasse `.weekday-picker` hinzufügen:
  - `display: flex; flex-wrap: wrap; gap: 0.375rem; align-items: center; margin-top: 0.5rem;`
  - Mobile-first: Alle 7 Buttons + Label passen in eine Zeile auf Mobilgeräten

- [ ] CSS-Klasse `.weekday-picker-label` hinzufügen:
  - `font-size: 0.75rem; color: #6b7280; width: 100%; margin-bottom: 0.125rem;`
  - Kleine graue Beschriftung "Nächste Woche" über den Buttons

- [ ] CSS-Klasse `.weekday-btn` hinzufügen:
  - `min-width: 44px; min-height: 44px;` (Touch-Ziel, konsistent mit `.btn-icon`)
  - `padding: 0.25rem 0.5rem; border: 1px solid var(--border-color); border-radius: 0.375rem;`
  - `background: var(--card-bg); color: var(--text-color); cursor: pointer;`
  - `font-size: 0.875rem; font-weight: 500;`
  - `transition: background-color 0.15s, border-color 0.15s, color 0.15s;`

- [ ] CSS-Klasse `.weekday-btn:hover`:
  - `background-color: #dbeafe; border-color: var(--primary-color);`

- [ ] CSS-Klasse `.weekday-btn:focus-visible`:
  - `outline: 2px solid var(--primary-color); outline-offset: 2px;`

- [ ] CSS-Klasse `.weekday-btn.active` (für `aria-pressed="true"`):
  - `background-color: var(--primary-color); border-color: var(--primary-color); color: white;`

- [ ] CSS-Klasse `.weekday-btn.active:hover`:
  - `background-color: var(--primary-hover); border-color: var(--primary-hover);`

- [ ] Sicherstellen, dass CSS in Mobile-Sektion platziert ist (vor dem Media-Query `@media (min-width: 768px)`)

### Schritt 4: Integrationstests (`tests/`)

Es gibt keine neue serverseitige Logik, daher sind keine Rust-Integrationstests notwendig. Der Server-Teil ist unverändert.

**Hinweis:** Bestehende Integrationstests für das Datum (Story 28) bleiben unberührt und müssen weiterhin bestehen.

### Schritt 5: E2E-Tests (`tests/e2e/recipe-weekday-picker.spec.ts`)

- [ ] `tests/e2e/recipe-weekday-picker.spec.ts` erstellen mit folgenden Tests:

  **Test K1/K2: Wochentag-Buttons werden angezeigt (mit JS)**
  ```
  // Given: Das Formular für ein neues Rezept ist geöffnet (JS aktiv)
  // When: Die Seite lädt
  // Then: Sieben Wochentag-Buttons Mo–So sind sichtbar
  ```

  **Test K2: Klick auf Wochentag-Button setzt korrektes Datum**
  ```
  // Given: Das Formular ist geöffnet, heute ist ein bekannter Wochentag (fixiert per page.clock)
  // When: Der Nutzer auf "Do" klickt
  // Then: Das Datumsfeld enthält das Datum des nächsten Donnerstags (nächste Woche)
  // And: Der Button "Do" hat aria-pressed="true"
  ```
  *Hinweis:* Playwright erlaubt `page.clock.setFixedTime(...)` um das Datum zu fixieren.

  **Test K3: Aktiver Button ist visuell hervorgehoben**
  ```
  // Given: Das Formular ist geöffnet
  // When: Der Nutzer auf "Di" klickt
  // Then: Button "Di" hat CSS-Klasse "active" und aria-pressed="true"
  // And: Alle anderen Buttons haben aria-pressed="false"
  ```

  **Test K4: Erneuter Klick auf aktiven Button leert das Feld**
  ```
  // Given: Der Nutzer hat "Mo" geklickt, das Datumsfeld zeigt ein Datum
  // When: Der Nutzer erneut auf "Mo" klickt
  // Then: Das Datumsfeld ist leer
  // And: Kein Button ist aktiv (aria-pressed="false" für alle)
  ```

  **Test K5: Manuelle Eingabe deaktiviert Wochentag-Markierung**
  ```
  // Given: Der Nutzer hat "Di" gewählt, "Di" ist aktiv
  // When: Der Nutzer das Datumsfeld manuell auf "15.3.2026" ändert
  // Then: Kein Button ist mehr aktiv (15.3.2026 ist kein Di nächste Woche)
  ```

  **Test K5b: Manuelle Eingabe mit Wochentag-Match markiert Button**
  ```
  // Given: Das Formular ist geöffnet, heute ist Mittwoch 01.04.2026 (fixiert)
  // When: Der Nutzer manuell "9.4.2026" eingibt (= Do nächste Woche)
  // Then: Button "Do" ist aktiv markiert
  ```

  **Test K6: Datum wird beim Speichern korrekt gespeichert**
  ```
  // Given: Das Formular ist geöffnet
  // When: Der Nutzer auf "Fr" klickt und das Formular speichert
  // Then: Die Detailansicht zeigt das korrekte Datum (Freitag nächste Woche)
  ```

  **Test K3 (Edit-Formular): Vorhandenes Datum aus nächster Woche markiert Button**
  ```
  // Given: Ein Rezept hat planned_date = nächster Donnerstag, heute ist Mittwoch (fixiert)
  // When: Der Nutzer die Bearbeiten-Seite öffnet
  // Then: Button "Do" ist aktiv markiert
  ```

  **Test K6 (ohne JS): Fallback funktioniert**
  ```
  // Given: JavaScript ist deaktiviert (via Playwright context option)
  // When: Der Nutzer die Rezept-Bearbeiten-Seite öffnet
  // Then: Keine Wochentag-Buttons sind sichtbar
  // And: Das Datumsfeld ist vorhanden und funktioniert
  ```

- [ ] Alle Tests mit Given/When/Then als deutsche Kommentare inline im Code
- [ ] `page.clock.setFixedTime(new Date('2026-04-01T10:00:00'))` für Datum-abhängige Tests

### Schritt 6: Aria-Attribute und Barrierefreiheit sicherstellen

- [ ] Sicherstellen, dass alle Buttons `type="button"` haben (kein Submit-Verhalten)
- [ ] `aria-label="[Wochentag] nächste Woche wählen"` je Button (z.B. "Montag nächste Woche wählen")
- [ ] `aria-pressed="false"` initial, `aria-pressed="true"` wenn aktiv
- [ ] Fokus-Indikator über `.weekday-btn:focus-visible` (bereits in CSS geplant)
- [ ] Button-Gruppe mit `role="group"` und `aria-label="Wochentag nächste Woche"` wrappen

### Schritt 7: DoD-Abschluss-Prüfung

- [ ] `cargo build` → keine Fehler, keine Warnungen
- [ ] `cargo clippy -- -D warnings` → sauber (keine Rust-Änderungen, sollte clean sein)
- [ ] `cargo fmt --check` → sauber
- [ ] `cargo test` → alle bestehenden Tests weiterhin grün
- [ ] `npm run test:e2e` → alle E2E-Tests grün (neue + bestehende)

---

## URL-Struktur

Keine neuen URLs. Der Wochentag-Picker ist ein clientseitiges Widget. Die bestehenden URLs bleiben unverändert:

```
GET  /recipes/new         → Formular enthält jetzt Wochentag-Picker (via JS)
POST /recipes             → Datum kommt als T.M.JJJJ String (unverändert)
GET  /recipes/{id}/edit   → Formular vorausgefüllt, Button ggf. aktiv markiert
POST /recipes/{id}        → Datum-Update unverändert
```

---

## Abhängigkeiten

- Story 28 (Datum-Eingabe) ist abgeschlossen — `planned_date`-Feld, Parsing-Logik und Formular-Grundstruktur existieren
- Story 01 (Rezept erstellen) und Story 02 (Rezept bearbeiten) sind abgeschlossen
- Kein neuer Rust-Code notwendig
- Kein neuer Build-Step, kein npm-Paket, kein externes Framework
- `page.clock` API von Playwright wird für zeit-abhängige Tests benötigt (Playwright ≥ 1.45, bereits im Projekt)

---

## Technische Details: JavaScript-Implementierung

### Vollständige Algorithmus-Beschreibung

```
// ISO-Wochenberechnung: Montag = 0, Sonntag = 6
WOCHENTAGE = ["Mo", "Di", "Mi", "Do", "Fr", "Sa", "So"]
ARIA_LABELS = ["Montag", "Dienstag", "Mittwoch", "Donnerstag", "Freitag", "Samstag", "Sonntag"]

calculateNextWeekday(offset):
  today = new Date() (nur Datum, Zeit auf 00:00:00)
  dayOfWeek = (today.getDay() + 6) % 7  // getDay(): So=0,Mo=1,...,Sa=6 → ISO: Mo=0,...,So=6
  daysToNextMonday = 7 - dayOfWeek       // Mo: 7, Di: 6, Mi: 5, Do: 4, Fr: 3, Sa: 2, So: 1
  nextMonday = today + daysToNextMonday Tage
  target = nextMonday + offset Tage
  return formatAsGerman(target)          // "T.M.JJJJ" ohne führende Nullen

formatAsGerman(date):
  day = date.getDate()    // keine führende Null
  month = date.getMonth() + 1  // keine führende Null
  year = date.getFullYear()
  return day + "." + month + "." + year

parseGermanDate(str):
  parts = str.split(".")
  if parts.length !== 3: return null
  day, month, year = parseInt(parts[0]), parseInt(parts[1]), parseInt(parts[2])
  if isNaN(day) or isNaN(month) or isNaN(year): return null
  if year < 100: year += 2000
  date = new Date(year, month-1, day)
  if date.getDate() !== day or date.getMonth()+1 !== month: return null (z.B. 32.1.2026)
  return date

detectActiveWeekday(str):
  parsed = parseGermanDate(str)
  if parsed is null: return -1
  for offset in 0..6:
    expected = calculateNextWeekday(offset)
    if expected === str: return offset  // direkter String-Vergleich
  return -1
```

### Einfügepunkt im HTML

Der Wochentag-Picker wird per JS **nach** dem `.date-input-group`-Element in `form.html` eingefügt:

```
<div class="form-group">
  <label for="planned_date">Datum (geplant / gekocht)</label>
  <div class="date-input-group">
    <input type="text" id="planned_date" ...>
    <input type="date" id="date-picker" ...>
    <button ... aria-label="Kalender öffnen">...</button>
  </div>
  <!-- ↓ Per JS eingefügt: -->
  <div class="weekday-picker" role="group" aria-label="Wochentag nächste Woche">
    <span class="weekday-picker-label">Nächste Woche</span>
    <button type="button" class="weekday-btn" aria-label="Montag nächste Woche wählen" aria-pressed="false">Mo</button>
    ...
  </div>
</div>
```

### Aktiv-State-Verwaltung

Beim Klick auf Button:
1. `isActive = button.getAttribute('aria-pressed') === 'true'`
2. Alle Buttons auf `aria-pressed="false"` und Klasse `active` entfernen
3. Wenn `!isActive`: Datum berechnen, Textfeld befüllen, Button auf `aria-pressed="true"` und Klasse `active` setzen
4. Wenn `isActive`: Textfeld leeren (Toggle-Verhalten)

Beim `input`-Event am Textfeld:
1. `activeOffset = detectActiveWeekday(textInput.value)`
2. Alle Buttons auf `aria-pressed="false"` und Klasse `active` entfernen
3. Wenn `activeOffset >= 0`: entsprechenden Button aktivieren

---

## Test-Checkliste

- [ ] E2E-Test: Wochentag-Buttons Mo–So sind sichtbar (K1)
- [ ] E2E-Test: Klick auf "Do" setzt korrektes Datum (K2) — mit fixiertem Datum per `page.clock`
- [ ] E2E-Test: Aktiver Button ist visuell hervorgehoben (K3) — aria-pressed + CSS-Klasse active
- [ ] E2E-Test: Erneuter Klick auf aktiven Button leert das Datumsfeld (K4)
- [ ] E2E-Test: Manuelle Texteingabe ohne Wochentag-Match entfernt aktive Markierung (K5)
- [ ] E2E-Test: Manuelle Texteingabe mit Wochentag-Match setzt aktive Markierung (K5)
- [ ] E2E-Test: Formular speichert korrekt (K6) — Datum in Detailansicht sichtbar
- [ ] E2E-Test: Edit-Formular mit vorhandenem Datum nächste Woche markiert korrekten Button (K3)
- [ ] E2E-Test: Ohne JavaScript — keine Wochentag-Buttons, Texteingabe funktioniert (K1)
- [ ] Manueller Test: Buttons auf Mobilgerät (44px Touch-Ziel prüfen)
- [ ] Manueller Test: Tastatur-Navigation (Tab → Button → Enter/Space)
- [ ] Manueller Test: Progressive Enhancement — JS deaktivieren, Formular weiterhin benutzbar

---

## Offene Punkte

- `page.clock.setFixedTime()` in Playwright: Wird `new Date()` im Browser-JS auch auf den fixierten Zeitpunkt gesetzt? → Ja, Playwright's `page.clock` überschreibt die Browser-Uhr, sodass `new Date()` im ausgeführten Script den fixierten Wert zurückgibt. Dies ist Voraussetzung für deterministische E2E-Tests.
- Position des CSS-Codes: Neue `.weekday-*`-Klassen in die allgemeine Sektion (vor dem Media-Query) in `app.css` einfügen. Kein Responsive-Override nötig, da `flex-wrap: wrap` und `min-width: 44px` bereits mobile-first funktionieren.
