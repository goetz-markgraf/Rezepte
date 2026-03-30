# Implementierungsplan: Story 29 - Wochen-Picker erweitern

## Technische Analyse

### Ist-Zustand

- Story 16 (Wochentag-Picker) ist implementiert: 7 Buttons (Mo-So) für die "nächste Woche" (ab dem nächsten Montag)
- `templates/recipes/form.html` enthält den `<script>`-Block mit `createWeekdayPicker()`
- CSS-Klassen `.weekday-picker`, `.weekday-btn` etc. existieren in `src/static/css/app.css`
- `planned_date`-Feld existiert in DB und wird korrekt verarbeitet (Story 28)

### Scope dieser Story

Diese Story ist **rein clientseitig** (HTML + CSS + Vanilla JS). Kein Rust-Code muss geändert werden.
Die Erweiterung ändert:
1. Anzahl Tage: 7 → 10
2. Beginn: nächster Montag → morgen
3. Button-Text: nur Wochentag → "Wochentag + Datum" (z.B. "Mo 30.03")

### Entscheidung: Progressive Enhancement

- Ohne JS: Keine Buttons sichtbar, nur das Textfeld funktioniert (wie bisher)
- Die Buttons werden per JavaScript eingefügt (wie in Story 16)
- Bestehendes Verhalten bei fehlendem JS bleibt erhalten

---

## Technische Schritte

### Schritt 1: JavaScript-Logik anpassen

Erweiterung des bestehenden `<script>`-Blocks in `form.html`:

- [ ] Funktion `calculateNextWeekday(offset)` anpassen:
  - `offset` 0-9 statt 0-6 (10 Tage statt 7)
  - Berechnung beginnt bei morgen statt beim nächsten Montag
  - Gibt Datum als String `T.M.JJJJ` zurück

- [ ] Funktion `calculateDateFromTomorrow(offset)` neu erstellen:
  - `offset` 0-9 (0=morgen, 1=übermorgen, ..., 9=in 10 Tagen)
  - `tomorrow = today + 1 Tag`
  - `targetDate = tomorrow + offset Tage`
  - Gibt Datum als String `T.M.JJJJ` zurück

- [ ] Button-Labels anpassen:
  - `getDayLabel(offset)` gibt "Mo 30.03" zurück statt nur "Mo"
  - Wochentag-Kürzel aus `Intl.DateTimeFormat('de-DE', { weekday: 'short' })`
  - Datum ohne Jahr: `date.getDate() + "." + (date.getMonth() + 1) + "."`

- [ ] Funktion `detectActiveDate(dateStr)` anpassen:
  - Vergleicht mit allen 10 möglichen Daten (morgen bis +10 Tage)
  - Gibt den passenden offset (0-9) zurück oder -1 wenn kein Match

- [ ] Funktion `createWeekdayPicker(textInput)` anpassen:
  - Erstellt 10 statt 7 Buttons
  - Button-Text = `getDayLabel(offset)` (z.B. "Mo 30.03")
  - `aria-label` = z.B. "Montag, 30. März wählen"
  - Click-Handler: Toggle-Verhalten bleibt gleich

### Schritt 2: HTML-Template anpassen

- [ ] Bestehenden `<script>`-Block erweitern:
  - Neue Funktionen hinzufügen
  - `createWeekdayPicker()` Aufruf bleibt (generiert jetzt 10 Buttons)
  - Keine HTML-Änderungen nötig — JS fügt Buttons dynamisch ein

### Schritt 3: CSS-Styling

- [ ] Prüfen ob bestehendes CSS `.weekday-btn` ausreicht:
  - 10 Buttons müssen in eine Zeile passen (eventuell `gap` reduzieren)
  - `min-width` eventuell verringern (44px → 40px) für mehr Platz
  - Oder `flex-wrap: wrap` erlauben

- [ ] Falls nötig: Neue CSS-Klasse `.day-btn` mit angepasstem Styling

### Schritt 4: Tests

- [ ] Bestehende E2E-Tests für Story 16 prüfen und ggf. anpassen

### Schritt 5: E2E-Tests (`tests/e2e/weekday-picker-extended.spec.ts`)

- [ ] `tests/e2e/weekday-picker-extended.spec.ts` erstellen:

  **Test K1: Picker zeigt 10 Tage ab morgen**
  ```
  // Given: Heute ist Mittwoch, 29.03.2026 (fixiert per page.clock)
  // When: Der Nutzer die Rezept-Bearbeiten-Seite öffnet
  // Then: Werden 10 Tage angezeigt: Do 30.03 bis Sa 08.04
  ```

  **Test K2: Klick auf Tag setzt korrektes Datum**
  ```
  // Given: Heute ist Mittwoch, 29.03.2026
  // When: Der Nutzer auf "Fr 03.04" klickt
  // Then: Enthält das Datumsfeld "3.4.2026"
  // And: Der Tag "Fr 03.04" ist als aktiv markiert (aria-pressed="true")
  ```

  **Test K3: Beginn bei morgen (nicht Montag)**
  ```
  // Given: Heute ist Mittwoch
  // When: Der Picker wird angezeigt
  // Then: Der erste Tag ist morgen (Do), nicht der nächste Montag
  ```

  **Test K4: Datum im Format TT.MM angezeigt**
  ```
  // Given: Heute ist Montag, 30.03.2026
  // When: Der Nutzer die Rezept-Bearbeiten-Seite öffnet
  // Then: Zeigt der 2. Tag "Di 31.03"
  // And: Zeigt der 3. Tag "Mi 01.04" (Monatswechsel)
  ```

  **Test K5: Aktiver Tag wird hervorgehoben**
  ```
  // Given: Ein Rezept hat planned_date = übermorgen
  // When: Der Nutzer die Bearbeiten-Seite öffnet
  // Then: Ist der entsprechende Tag als aktiv markiert
  ```

  **Test K6: Erneuter Klick auf aktiven Tag leert das Datum**
  ```
  // Given: Der Nutzer hat einen Tag gewählt
  // When: Der Nutzer erneut auf denselben Tag klickt
  // Then: Ist das Datumsfeld leer
  // And: Kein Tag ist als aktiv markiert
  ```

  **Test K7: Manuelle Eingabe demarkiert alle Tags**
  ```
  // Given: Ein Tag ist aktiv markiert
  // When: Der Nutzer das Datumsfeld manuell auf ein anderes Datum ändert
  // Then: Sind alle Tags demarkiert
  ```

  **Test K8: Ohne JavaScript — Fallback funktioniert**
  ```
  // Given: JavaScript ist deaktiviert
  // When: Der Nutzer die Rezept-Bearbeiten-Seite öffnet
  // Then: Sind keine Tag-Buttons sichtbar
  // And: Das Datumsfeld ist vorhanden und funktioniert
  ```

- [ ] Alle Tests mit Given/When/Then als deutsche Kommentare inline im Code

### Schritt 6: Barrierefreiheit sicherstellen

- [ ] `aria-label="[Wochentag], [Datum] wählen"` je Button
- [ ] `aria-pressed="false"` initial, `aria-pressed="true"` wenn aktiv
- [ ] `role="group"` und `aria-label="Tag wählen"` am Container
- [ ] Tastatur-Navigation: Tab zu allen 10 Buttons, Enter/Space zum Aktivieren

### Schritt 7: DoD-Abschluss-Prüfung

- [ ] `cargo build` → keine Fehler, keine Warnungen
- [ ] `cargo clippy -- -D warnings` → sauber
- [ ] `cargo fmt --check` → sauber
- [ ] `cargo test` → alle bestehenden Tests weiterhin grün
- [ ] `npm run test:e2e` → alle E2E-Tests grün

---

## URL-Struktur

Keine neuen URLs. Der erweiterte Wochen-Picker ist ein clientseitiges Widget:

```
GET  /recipes/new         → Formular mit erweitertem Picker (10 Tage)
POST /recipes             → Datum kommt als T.M.JJJJ String (unverändert)
GET  /recipes/{id}/edit   → Formular vorausgefüllt, Button ggf. aktiv markiert
POST /recipes/{id}        → Datum-Update unverändert
```

---

## Abhängigkeiten

- Story 16 (Wochentag-Picker) muss abgeschlossen sein — Grundfunktionalität des Pickers
- Story 28 (Datum-Eingabe) muss abgeschlossen sein — `planned_date`-Feld existiert
- Kein Rust-Code notwendig
- Kein neues Build-Step, kein npm-Paket, kein externes Framework

---

## Test-Checkliste

- [ ] E2E-Test: Picker zeigt 10 Tage ab morgen (K1)
- [ ] E2E-Test: Klick auf Tag setzt korrektes Datum (K2)
- [ ] E2E-Test: Beginn bei morgen, nicht Montag (K3)
- [ ] E2E-Test: Datum im Format TT.MM angezeigt (K4)
- [ ] E2E-Test: Aktiver Tag wird hervorgehoben (K5)
- [ ] E2E-Test: Erneuter Klick auf aktiven Tag leert das Datum (K6)
- [ ] E2E-Test: Manuelle Eingabe demarkiert alle Tags (K7)
- [ ] E2E-Test: Ohne JavaScript — Fallback funktioniert (K8)
- [ ] E2E-Test: Barrierefreiheit — aria-label, aria-pressed, Tastatur-Navigation
- [ ] Manueller Test: Monatswechsel (Ende März → Anfang April)
- [ ] Manueller Test: Jahreswechsel (Dezember → Januar)

---

## Offene Punkte

- [ ] 10 Buttons passen nicht in eine Zeile auf kleinen Bildschirmen → flex-wrap prüfen
- [ ] Touch-Target-Größe (44px) muss auch bei kleineren Buttons gewährleistet sein

---

## Technische Details: JavaScript-Algorithmus

```
WOCHENTAGE_KURZ = ["So", "Mo", "Di", "Mi", "Do", "Fr", "Sa"]

calculateDateFromTomorrow(offset):
  today = new Date()
  today.setHours(0, 0, 0, 0)
  tomorrow = new Date(today)
  tomorrow.setDate(tomorrow.getDate() + 1)
  target = new Date(tomorrow)
  target.setDate(target.getDate() + offset)
  return target

getDayLabel(offset):
  date = calculateDateFromTomorrow(offset)
  wochentag = WOCHENTAGE_KURZ[date.getDay()]
  tag = date.getDate()
  monat = date.getMonth() + 1
  return wochentag + " " + tag + "." + monat + "."

formatAsGerman(date):
  day = date.getDate()
  month = date.getMonth() + 1
  year = date.getFullYear()
  return day + "." + month + "." + year

detectActiveDate(str):
  parsed = parseGermanDate(str)
  if parsed is null: return -1
  today = new Date(); today.setHours(0,0,0,0)
  tomorrow = new Date(today); tomorrow.setDate(tomorrow.getDate() + 1)
  for offset in 0..9:
    expected = calculateDateFromTomorrow(offset)
    if expected.getTime() === parsed.getTime(): return offset
  return -1
```