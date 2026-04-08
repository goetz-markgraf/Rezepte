# Plan 41: Bewertungssterne im Bearbeitungsmodus

**Story-Nr:** 41  
**Titel:** Bewertungssterne im Bearbeitungsmodus  
**Status:** In Arbeit  
**Priorität:** MVP Phase 1 / Bug-Fix  

---

## 1. Problem-Analyse

### Aktueller Zustand
- Im Formular-View (`form.html`) werden Stern-Bewertungen mit Radio-Buttons dargestellt
- Beim Auswählen eines Sterns wird **nur dieser einzelne Stern** farbig markiert (orange `#f59e0b`)
- In der Detailansicht/Übersicht funktioniert die Darstellung korrekt: Alle Sterne bis zum ausgewählten Stern werden markiert

### Erwarteter Zustand
- Beim Überfahren mit der Maus oder Klicken auf einen Stern müssen **alle Sterne bis zu diesem Stern** markiert werden (z.B. bei Auswahl von Stern 4 werden Sterne 1, 2, 3, 4 angezeigt)
- Die Darstellung muss identisch sein wie in der Detailansicht

### Technische Ursache
Das CSS im Formular-View nutzt `label:has(input:checked)` für die farbliche Markierung, aber es fehlt der CSS-Selktor mit dem "sibling-combinator" (`~`), der Sterne bis zum gewählten Stern markiert - analog zur Lösung in `.inline-rating-form`.

---

## 2. Lösungsansatz

### Frontend-CSS Änderungen
Erweitere die CSS-Regeln für `.star-rating-options label` um Hover-Effekte, die alle Sterne bis zum gewählten Stern farbig markieren:

**Logik:**
1. Beim Hover über einen Stern werden dieser und alle davorliegenden Sterne orange (`#f59e0b`)
2. Ungewählte Sterne bleiben grau (`#6b7280`)
3. Bei Auswahl (checked) bleibt die Auswahl dauerhaft orange

### Umsetzung mit CSS `:hover` + `~` Selector
```css
/* Basis: Alle Sterne grau */
.star-rating-options label:not(.star-rating-none) {
    color: #6b7280;
}

/* Beim Hover über Stern N: Stern N und alle davor (durch flex-direction oder Selektoren) orange */
.star-rating-options label:hover,
.star-rating-options label:hover ~ label {
    color: #f59e0b;
}

/* Auswahl dauerhaft orange */
.star-rating-options label:has(input:checked) {
    color: #f59e0b;
}

/* Bei Hover über bereits ausgewählten Stern: keine Änderung */
.star-rating-options label:has(input:checked):hover,
.star-rating-options label:has(input:checked) ~ label {
    color: #f59e0b;
}
```

**Herausforderung:** Radio-Buttons sind nicht in umgekehrter Reihenfolge wie im Inline-Rating. Daher muss die Logik angepasst werden - entweder durch CSS `flex-direction` oder durch gezielte Selektoren.

---

## 3. Implementierungsschritte

### [ ] Schritt 1: Test-Infrastruktur vorbereiten
- Playwright-E2E-Test für Stern-Bewertung im Formular schreiben
- Seed-Daten erstellen (Rezept mit bestehender Bewertung)

### [ ] Schritt 2: CSS-Hover-Effekt implementieren
- CSS-Regeln für `.star-rating-options label` erweitern
- Hover-Effekt hinzufügen, der alle Sterne bis zum gewählten Stern markiert
- Testen im Browser (DevTools)

### [ ] Schritt 3: Barrierefreiheit sicherstellen
- Tastatur-Navigation testen (Tab + Pfeiltasten)
- Fokus-Indikatoren prüfen (`:focus-visible`)
- Screenreader-Kompatibilität (ARIA-Labels bereits vorhanden?)

### [ ] Schritt 4: Unit Tests für CSS/Template
- Askama-Template Validierung (falls applicable)
- CSS-Linting (falls vorhanden)

### [ ] Schritt 5: Integrationstests
- Rust-Tests für Template-Rendering (falls applicable)
- HTTP-Endpunkt-Tests (Bewertung speichern)

### [ ] Schritt 6: E2E-Tests implementieren
- Playwright-Test: Hover über Sterne im Formular
- Playwright-Test: Tastatur-Navigation durch Sterne
- Playwright-Test: Konsistenz zwischen Edit- und View-Mode

### [ ] Schritt 7: Code-Qualität sicherstellen
- `cargo fmt` ausführen
- `cargo clippy -- -D warnings` prüfen
- CSS-Konsistenz mit anderen Stern-Bewertungen prüfen

---

## 4. Betroffene Dateien

### Templates
- `templates/recipes/form.html` (Stern-Bewertung im Formular)

### CSS
- `src/static/css/app.css` (Sterne-Styling für Formular)

### Tests
- `tests/e2e/rating.spec.ts` oder `tests/e2e/recipes.spec.ts` (Playwright-E2E-Test)

---

## 5. Teststrategie

### E2E-Tests (Playwright)

**Testfall 1: Hover-Effekt im Formular**
```typescript
// Given ich bin auf der Bearbeitungsseite eines Rezepts mit bestehender Bewertung
// When ich über den vierten Stern fahre
// Then werden Sterne 1, 2, 3, 4 orange markiert
// And Sterne 5 bleibt grau
```

**Testfall 2: Tastatur-Navigation**
```typescript
// Given ich bin auf der Bearbeitungsseite
// When ich mit Tab zu den Sternen navigiere und Pfeiltasten verwende
// Then werden Sterne korrekt fokussiert (Fokus-Ring sichtbar)
// And Hover-Effekt funktioniert mit Tastatur
```

**Testfall 3: Konsistenz zwischen Edit- und View-Mode**
```typescript
// Given ich habe ein Rezept mit Bewertung 4 erstellt
// When ich das Rezept bearbeite und über Sterne fahre
// Then zeigt der Bearbeitungsmodus dieselbe Stern-Darstellung wie die Detailansicht
// And when I save the changes
// The correct rating is displayed in both edit and view modes
```

---

## 6. Risikobewertung

### Niedrige Risiken
- Keine Datenbank-Änderungen notwendig (reines Frontend)
- Bestehende CSS-Muster können wiederverwendet werden
- Keine Breaking Changes erwartet

### Mögliche Herausforderungen
- **CSS-Spezifität:** `:has()` Selector wird in älteren Browsern nicht unterstützt (Support ab Safari 15.4, Chrome 105)
  - Fallback: Feature-Detection oder JavaScript-Polyfill falls nötig
- **Radio-Button-Reihenfolge:** Im Gegensatz zu `.inline-rating-form` sind Radio-Buttons nicht umgekehrt angeordnet
  - Lösung: CSS `flex-direction: row` + gezielte Selektoren oder JavaScript für Hover-Effekt

---

## 7. Offene Fragen / Entscheidungen erforderlich

### [ ] Frage 1: Browser-Support für `:has()` Selector
- **Option A:** `:has()` verwenden (moderner CSS-Selector, guter Support in aktuellen Browsern)
- **Option B:** JavaScript-Polyfill hinzufügen (maximale Kompatibilität, aber mehr Code)
- **Empfehlung:** Option A versuchen, falls Tests zeigen dass es nicht funktioniert → zu Option B wechseln

### [ ] Frage 2: Reihenfolge der Radio-Buttons im HTML
- **Aktueller Zustand:** Sterne in aufsteigender Reihenfolge (1, 2, 3, 4, 5)
- **Option A:** HTML ändern und Radio-Button-Reihenfolge umkehren (5, 4, 3, 2, 1) → ermöglicht use von `~` Selector wie im Inline-Rating
- **Option B:** CSS mit komplexeren Selektoren lösen (z.B. `:hover` + parent-Selector)
- **Empfehlung:** Option A bevorzugen (einfacheres CSS), aber nur wenn keine negativen Auswirkungen auf Accessibility

---

## 8. Definition of Done Checkliste

### Code-Qualität
- [ ] Keine Compiler-Fehler oder Warnungen (`cargo build`)
- [ ] Keine Clippy-Warnings (`cargo clippy -- -D warnings`)
- [ ] Code ist korrekt formatiert (`cargo fmt --check`)
- [ ] CSS konsistent mit bestehenden Mustern

### Architektur-Einhaltung
- [ ] Keine Backend-Änderungen (reines Frontend)
- [ ] HTMX-Integration bleibt erhalten
- [ ] DeepLink-fähige URLs funktionieren weiterhin

### Testing
- [ ] E2E-Tests für Hover-Effekt geschrieben und bestanden
- [ ] Tastatur-Navigation getestet
- [ ] Screenreader-Kompatibilität geprüft
- [ ] Alle Tests erfolgreich durchlaufen

### Funktionale Anforderungen
- [ ] Hover über Stern markiert alle Sterne bis zu diesem Stern orange
- [ ] Auswahl dauerhaft orange dargestellt
- [ ] Darstellung identisch wie in Detailansicht/Übersicht
- [ ] Keine Regressionen in anderen Bereichen

---

**Letzte Aktualisierung:** 2026-04-08  
**Nächster Review:** Nach Implementierung von Schritt 2 (CSS)
