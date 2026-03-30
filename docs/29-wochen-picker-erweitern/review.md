# Review: Story 29 - Wochen-Picker erweitern

**Review-Datum:** 2026-03-30
**Reviewer:** opencode
**Status:** ✅ Abgenommen

---

## 1. Akzeptanzkriterien Prüfung

### Funktionale Kriterien

| Kriterium | Status | Begründung |
|-----------|--------|------------|
| **K1:** Picker zeigt 10 Tage | ✅ | `createWeekdayPicker()` erstellt 10 Buttons (Schleife 0-9) |
| **K2:** Klick auf Tag setzt Datum | ✅ | Click-Handler berechnet Datum und trägt es in `input.value` ein |
| **K3:** Beginn bei morgen | ✅ | `calculateDateFromTomorrow(offset)` beginnt bei `today + 1` |
| **K4:** Datum TT.MM angezeigt | ✅ | `getDayLabel()` formatiert als "Mo 30.03" |
| **K5:** Aktiver Tag hervorgehoben | ✅ | `detectActiveDate()` prüft initiales Datum, markiert Button mit `class="active"` und `aria-pressed="true"` |
| **K6:** Erneuter Klick leert Datum | ✅ | Toggle-Logik in Click-Handler: bei erneutem Klick wird `input.value = ''` gesetzt |
| **K7:** Kompatibilität mit bestehendem planned_date | ✅ | Input-Event-Listener demarkiert alle Tags bei manueller Eingabe |

### Nicht-funktionale Kriterien

| Kriterium | Status | Begründung |
|-----------|--------|------------|
| **K8:** Performance | ✅ | Client-seitige JavaScript-Berechnung, keine Server-Anfrage |
| **K9:** Barrierefreiheit | ✅ | `aria-label`, `aria-pressed`, Tastatur-Navigation (focus-visible) |

---

## 2. Qualitätschecks

| Check | Ergebnis |
|-------|----------|
| `cargo build` | ✅ Erfolgreich |
| `cargo clippy -- -D warnings` | ✅ Keine Warnungen |
| `cargo fmt --check` | ✅ Formatiert |
| `cargo test` | ✅ 127 Tests bestanden |
| `npm run test:e2e` | ✅ 192 Tests bestanden (10 übersprungen - Story 16 Tests) |

---

## 3. Implementierungsdetails

### Geänderte Dateien

1. **templates/recipes/form.html:153-293**
   - Wochen-Picker-Widget komplett überarbeitet
   - 10 statt 7 Buttons
   - Beginn bei morgen statt nächster Montag
   - Datum im Format "Wochentag TT.MM" (z.B. "Mo 30.03")

2. **src/static/css/app.css:878-926**
   - Bestehende CSS-Klassen wiederverwendet
   - `flex-wrap: wrap` erlaubt Umbruch bei kleinen Bildschirmen

3. **tests/e2e/weekday-picker-extended.spec.ts**
   - 9 E2E-Tests für Story 29
   - Alle Tests bestanden

### Technische Entscheidungen

- **Progressive Enhancement:** Buttons werden per JavaScript eingefügt, ohne JS bleibt nur das Textfeld sichtbar
- **Kein Rust-Code:** Story ist rein clientseitig implementiert
- **Toggle-Verhalten:** Erneuter Klick auf aktiven Tag leert das Datum (wie in Story 16)

---

## 4. Bekannte Einschränkungen

- Keine Jahreswechsel-Behandlung explizit getestet (aber Datumsberechnung nutzt native `Date`-API)
- Keine explizite Story 16 Testsuite (wurde durch Story 29 ersetzt - übersprungene Tests)

---

## 5. Fazit

✅ **Story 29 ist abnahmefertig.** Alle funktionalen und nicht-funktionalen Akzeptanzkriterien sind erfüllt. Die Implementierung ist vollständig, getestet und bereit für die Produktion.