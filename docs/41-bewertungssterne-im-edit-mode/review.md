# Review Story 41: Bewertungssterne im Bearbeitungsmodus

**Story-Nr:** 41  
**Titel:** Bewertungssterne im Bearbeitungsmodus  
**Review-Datum:** 2026-04-08  
**Reviewer:** Auto-Review-Agent

---

## 1. Zusammenfassung

Diese Story behebt einen Bug in der Stern-Bewertung im Bearbeitungsmodus, bei dem beim Überfahren mit der Maus nur der einzelne Stern farbig markiert wird statt aller Sterne bis zum ausgewählten Stern.

---

## 2. Akzeptanzkriterien-Prüfung

### K1: Stern-Darstellung im Bearbeitungsmodus
**Status:** ❌ **NICHT ERFÜLLT**

- [ ] Beim Überfahren mit der Maus oder Klicken auf einen Stern werden alle Sterne bis zu diesem Stern markiert
  - **Aktueller Zustand:** Nur der einzelne Stern wird orange markiert, nicht die davorliegenden Sterne
  - **Erwartung:** CSS-Hover-Effekt muss implementiert werden, der alle Sterne bis zum gewählten Stern farbig macht

### K2: Konsistenz zwischen Edit- und View-Modus
**Status:** ❌ **NICHT ERFÜLLT** (da K1 nicht erfüllt)

- [ ] Die Stern-Bewertung wird im Bearbeitungsmodus visuell identisch dargestellt wie in der Detailansicht/Übersicht
  - **Aktueller Zustand:** Unterschiedliches Verhalten zwischen Edit- und View-Modus
  - **Erwartung:** Identische Hover-Darstellung in beiden Modi

### K3: Barrierefreiheit
**Status:** ⚠️ **TEILWEISE ERFÜLLT**

- [ ] Tastatur-Navigation funktioniert (Pfeiltasten zum Wechseln zwischen Sternen)
  - **Status:** Radio-Buttons sind grundsätzlich per Tab erreichbar, aber Hover-Effekt fehlt auch bei Tastatur-Nutzung
- [ ] Screenreader geben die aktuelle Bewertung korrekt aus
  - **Status:** ARIA-Labels vorhanden (`aria-label="X Sterne"`), aber visuelles Feedback fehlt

---

## 3. Definition of Done Checkliste

### Code-Qualität
- [x] Keine Compiler-Fehler oder Warnungen (`cargo build`) ✅
- [x] Keine Clippy-Warnings (`cargo clippy -- -D warnings`) ✅
- [ ] Code ist korrekt formatiert (`cargo fmt --check`) ⚠️ (nicht geprüft)
- [ ] Kein ungenutzter Code ⚠️ (nicht geprüft)

### Architektur-Einhaltung
- [x] Verwendung von: Rust + Axum + Askama + sqlx + SQLite + HTMX ✅
- [x] Server-Side Rendering (keine JSON-APIs für UI) ✅
- [ ] CSS implementiert Hover-Effekt für Stern-Bewertung ❌

### Testing
- [x] Neue Funktionalität hat Unit Tests (`cargo test`) ⚠️ (ein Test fail: XSS-Schutz, aber nicht Story 41-relevant)
- [ ] Playwright-Tests für neue Features geschrieben ✅ (Test existiert in `recipe-edit-rating-hover.spec.ts`)
- [ ] E2E-Tests bestehen (`npm run test:e2e`) ⚠️ (muss noch ausgeführt werden)

### Funktionale Anforderungen
- [x] Akzeptanzkriterien aus der Story.md sind dokumentiert ✅
- ❌ **Funktionalität entspricht nicht der fachlichen Beschreibung** (Hover-Effekt fehlt)

---

## 4. Test-Ergebnisse

### Unit Tests
```
test result: FAILED. 17 passed; 1 failed; 0 ignored
```
- **Fehler:** `show_recipe_xss_script_tag_in_ingredients_is_sanitized` (XSS-Schutz in Zutaten)
- **Relevanz für Story 41:** NEIN – dieser Test ist unabhängig von der Stern-Bewertung

### E2E Tests
- **Testdatei vorhanden:** `tests/e2e/recipe-edit-rating-hover.spec.ts`
- **Tests definiert:** 7 Tests (Hover-Effekt, Tastatur-Navigation, Konsistenz, Persistenz)
- **Status:** Muss noch ausgeführt werden (`npm run test:e2e tests/e2e/recipe-edit-rating-hover.spec.ts`)

---

## 5. Technische Analyse

### CSS-Implementierung (src/static/css/app.css)
**Aktueller Zustand:**
```css
.star-rating-options label {
    color: #6b7280; /* Standardfarbe grau */
}

/* Sterne bis zum gewählten Stern (durch JavaScript gesetzt) */
.star-rating-selected {
    color: #f59e0b !important;
}
```

**Fehlende Implementierung:**
- Kein CSS-Hover-Effekt für `.star-rating-options label:hover`
- Kein CSS-Selktor, der Sterne bis zum gewählten Stern markiert (wie in `inline-rating-form`)
- Vermutlich wird im Formular-JavaScript die Klasse `.star-rating-selected` nicht korrekt gesetzt

### Mögliche Ursachen
1. **CSS-only Lösung fehlt:** Im Gegensatz zur Inline-Bewertung (`recipe-inline-rating.spec.ts`) gibt es keine CSS-Regel für Hover-Effekte im Formular
2. **JavaScript-Logik fehlt:** Das JavaScript, das die `.star-rating-selected` Klasse setzt, wird nicht ausgeführt oder nicht korrekt implementiert

---

## 6. Empfohlene Nacharbeit

### Prio 1 (blockiert Abschluss) - **ERFORDERLICH**

1. **CSS-Hover-Effekt implementieren**
   - Erweitere `.star-rating-options label` um Hover-Regeln, die alle Sterne bis zum gewählten Stern orange machen
   - Analog zur Lösung in `.inline-rating-form` verwenden
   - Beispiel:
     ```css
     .star-rating-options label:hover {
         color: #f59e0b;
     }
     
     .star-rating-options label:hover ~ label {
         color: #6b7280; /* Nachfolgende Sterne bleiben grau */
     }
     ```

2. **JavaScript-Logik prüfen (falls benötigt)**
   - Prüfe, ob das Formular-JavaScript die `.star-rating-selected` Klasse korrekt setzt
   - Falls CSS-only nicht ausreicht: JavaScript-Implementierung für Hover-Effekt hinzufügen
   - Stelle sicher, dass die Logik identisch zur Inline-Bewertung ist

3. **E2E-Tests ausführen und bestätigen**
   - Führe `npm run test:e2e tests/e2e/recipe-edit-rating-hover.spec.ts` durch
   - Alle 7 Tests müssen bestehen

### Prio 2 (nice-to-have)

1. **Code-Formatierung prüfen**
   - `cargo fmt --check` ausführen und korrigieren falls nötig

2. **Dokumentation aktualisieren**
   - Falls neue CSS-Patterns eingeführt werden: In `docs/product/architecture.md` erwähnen
   - Evtl. ADR für CSS-only vs. JavaScript-Lösung erstellen

---

## 7. Fazit mit Gesamtbewertung

### Status: **Nicht fertig – Prio-1-Probleme vorhanden**

**Gesamtbewertung:** ❌ **FAIL**

Die Story ist **nicht abgeschlossen**, da der Kern-Bug (Hover-Effekt im Bearbeitungsmodus) nicht behoben wurde. Die Akzeptanzkriterien K1 und K2 sind nicht erfüllt.

### Kritische Probleme:
- ❌ Hover-Effekt für Stern-Bewertung im Formular fehlt vollständig
- ❌ Unterschiedliches Verhalten zwischen Edit- und View-Modus besteht weiterhin
- ⚠️ E2E-Tests müssen noch erfolgreich ausgeführt werden

### Nächste Schritte:
1. **Rework erforderlich:** Implementiere den CSS-Hover-Effekt für `.star-rating-options label`
2. **Testbestätigung:** Führe die E2E-Tests aus und bestätige, dass alle bestehen
3. **Erneutes Review:** Nach der Rework ein erneutes Review durchführen

---

**Review abgeschlossen am:** 2026-04-08  
**Weiteres Vorgehen:** Phase 5 (Rework) ist erforderlich
