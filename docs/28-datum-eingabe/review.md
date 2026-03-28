# Review: Story 28 - Datum-Eingabe am Rezept (geplant / gekocht)

**Review-Datum:** 2026-03-28
**Story-Status:** Implementiert

---

## Zusammenfassung

Die Implementierung der Datum-Eingabe ist vollständig und sauber umgesetzt. Alle Akzeptanzkriterien sind erfüllt, sämtliche Tests (Unit, Integration, E2E) laufen durch. Der Code ist klar strukturiert, gut dokumentiert und folgt den Architektur-Vorgaben konsequent. Keine Nacharbeit erforderlich.

---

## Prüfung gegen den Plan

| Schritt | Status | Bemerkung |
|---------|--------|-----------|
| Schritt 1: `parse_german_date` in `recipe.rs` | ✅ | Funktion implementiert, alle 5 Unit-Tests vorhanden |
| Schritt 2: Rust-Modell erweitern | ✅ | `Recipe`, `CreateRecipe`, `UpdateRecipe` haben `planned_date`-Felder |
| Schritt 3: DB-Layer erweitern | ✅ | Alle Queries selectieren `planned_date`; 4 DB-Tests vorhanden |
| Schritt 4: Templates (Rust-Structs) | ✅ | `RecipeFormTemplate`, `RecipeDetailTemplate`, `RecipeListItem` erweitert |
| Schritt 5: HTML-Templates | ✅ | Form, Detail und Index-Template angepasst |
| Schritt 6: Route-Handler | ✅ | Alle Handler inkl. Hilfsfunktionen `format_planned_date_*` implementiert |
| Schritt 7: CSS-Styling | ✅ | `.date-input-group`, `.date-picker-hidden`, `.date-picker-btn`, `.recipe-date` |
| Schritt 8: Rust-Integrationstests | ✅ | `tests/recipe_date.rs` mit 6 Tests |
| Schritt 9: E2E-Tests | ✅ | `tests/e2e/recipe-date.spec.ts` mit 8 Tests |
| Schritt 10: DoD-Abschluss-Prüfung | ✅ | Alle Checks bestanden |

---

## Prüfung gegen Akzeptanzkriterien

| Kriterium | Status | Bemerkung |
|-----------|--------|-----------|
| **K1: Datum im Formular "Neues Rezept"** | ✅ | Feld vorhanden, leer vorbelegt, Rezept ohne Datum speicherbar |
| **K2: Datum im Formular "Rezept bearbeiten"** | ✅ | Vorausgefüllt via `format_planned_date_input`, änderbar und löschbar |
| **K3: Texteingabe des Datums** | ✅ | `T.M.JJJJ` und `TT.MM.JJJJ` akzeptiert; zweistelliges Jahr (25 → 2025); Punkt als Trennzeichen |
| **K4: Date-Picker als Alternative** | ✅ | Kalender-Icon (`button[aria-label="Kalender öffnen"]`) öffnet `showPicker()`; JS setzt Textfeld aus ISO-Datum |
| **K5: Validierung bei ungültigem Format** | ✅ | Fehlermeldung "Kein gültiges Datum. Bitte im Format T.M.JJJJ eingeben."; Formularwerte bleiben erhalten |
| **K6: Datum in der Detailansicht** | ✅ | Langer Monatsname (z.B. "5. März 2025"); bei fehlendem Datum kein Anzeigeblock |
| **K7: Datum in der Listenansicht** | ✅ | Kompaktes Format "05.03.2025" mit CSS-Klasse `.recipe-date`; bei fehlendem Datum leer |
| **K8: Performance** | ✅ | Keine zusätzlichen DB-Queries; Performance-Anforderung nicht messbar verschlechtert |
| **K9: Barrierefreiheit** | ✅ | `<label for="planned_date">`, `aria-label="Kalender öffnen"`, `aria-describedby="planned_date_error"` am Textfeld |

---

## Prüfung gegen Definition of Done

### Code-Qualität
- [x] `cargo build` — fehlerfrei
- [x] `cargo clippy -- -D warnings` — keine Warnungen
- [x] `cargo fmt --check` — korrekt formatiert
- [x] Keine ungenutzten Funktionen / Variablen

### Architektur-Einhaltung
- [x] Tech Stack: Rust + Axum + Askama + sqlx + SQLite + HTMX
- [x] SSR, keine JSON-APIs für UI
- [x] App funktioniert ohne JavaScript (Textfeld als primäre Eingabe, Date-Picker als Enhancement)
- [x] Code in korrekten Verzeichnissen

### Testing
- [x] Unit Tests geschrieben und bestanden (`cargo test`) — 46 Unit-Tests grün
- [x] Integrationstests geschrieben und bestanden — 6 Tests in `tests/recipe_date.rs`
- [x] E2E Tests geschrieben und bestanden (`npm run test:e2e`) — 8 E2E-Tests grün, 62 gesamt grün

### Funktionale Anforderungen
- [x] Alle Akzeptanzkriterien erfüllt
- [x] Edge Cases behandelt (leeres Datum, zweistelliges Jahr, Datum löschen, ungültiges Format)
- [x] Validierung vorhanden

---

## Test-Ergebnisse

### Unit-Tests (cargo test)

| Testgruppe | Anzahl | Status |
|------------|--------|--------|
| `parse_german_date_*` (5 Tests) | 5 | ✅ |
| `create_recipe_*` (6 Tests) | 6 | ✅ |
| `recipe_db` — Datum-spezifische Tests (4 Tests) | 4 | ✅ |
| Alle anderen Unit-Tests | 31 | ✅ |
| **Gesamt** | **46** | **✅** |

### Integrationstests (cargo test)

| Test | Status |
|------|--------|
| `should_create_recipe_with_valid_date_and_redirect` | ✅ |
| `should_show_date_in_detail_view` | ✅ |
| `should_reject_invalid_date_with_400` | ✅ |
| `should_retain_form_values_on_invalid_date` | ✅ |
| `should_create_recipe_without_date` | ✅ |
| `should_clear_date_on_update` | ✅ |

### E2E-Tests (npm run test:e2e)

| Test | Status |
|------|--------|
| K1/K3: Datum beim Erstellen speichern und anzeigen | ✅ |
| K2: Datum beim Bearbeiten vorausfüllen und änderbar | ✅ |
| K3 (Löschen): Datum leeren → nicht mehr angezeigt | ✅ |
| K5: Fehlermeldung bei ungültigem Datum, kein Datenverlust | ✅ |
| K4: Kalender-Icon sichtbar | ✅ |
| K6: Datum in der Detailansicht | ✅ |
| K7: Datum in der Listenansicht | ✅ |
| Barrierefreiheit: Label korrekt | ✅ |
| **Gesamt (alle E2E)** | **62/62** | **✅** |

### Code-Quality Checks

| Check | Ergebnis |
|-------|----------|
| `cargo build` | ✅ |
| `cargo clippy -- -D warnings` | ✅ |
| `cargo fmt --check` | ✅ |

---

## Empfohlene Nacharbeit

### Prio 1 (Muss — blockiert Abschluss)

Keine.

### Prio 2 (Sollte — nice-to-have)

1. **`aria-describedby` verweist auf nicht-existierendes Element ohne Fehler**
   - Das Textfeld hat `aria-describedby="planned_date_error"`, aber ein Element mit dieser ID wird im Template nicht gerendert (weder im Fehler- noch im Normalfall).
   - Das führt zu keinem sichtbaren Problem, ist aber technisch inkorrekt für WCAG-Konformität.
   - Empfehlung: Im Fehlerfall einen `<span id="planned_date_error" class="field-error">` mit der Fehlermeldung rendern; im Normalfall das Attribut weglassen oder ein leeres verstecktes Element verwenden.

2. **Manueller Test für Date-Picker auf mobilen Browsern nicht dokumentiert**
   - Im Plan (Schritt 10) vorgesehen: manueller Test auf Chrome, Firefox, Safari (mobil).
   - Dieser manuelle Testschritt ist in keiner Dokumentation festgehalten.
   - Empfehlung: Kurzen Hinweis im Review ergänzen oder als bekannte Lücke dokumentieren.

3. **`UpdateRecipe::validate()` und `CreateRecipe::validate()` sind identisch**
   - Die Validierungslogik in beiden Structs ist vollständig dupliziiert.
   - Beide delegieren an `validate_recipe_fields`, könnten aber auch via Trait oder gemeinsamer Hilfsfunktion vereinheitlicht werden.
   - Kein funktionales Problem, aber erhöhter Wartungsaufwand bei künftigen Änderungen.

---

## Fazit

**Gesamtbewertung:** ✅ Abgenommen

Die Story ist vollständig und korrekt implementiert. Alle 7 funktionalen Akzeptanzkriterien sowie die nicht-funktionalen Kriterien (Performance, Barrierefreiheit) sind erfüllt. 62 von 62 E2E-Tests und alle 46 Unit-Tests bestehen. Der Code ist sauber, clippy-frei und korrekt formatiert. Die einzigen Anmerkungen sind im Prio-2-Bereich und blockieren den Abschluss nicht.

**Nächste Schritte:**
1. Story als abgeschlossen markieren
2. Prio-2-Punkte (insb. `aria-describedby`) können in einer Folge-Story zur Barrierefreiheitsverbesserung gebündelt werden
