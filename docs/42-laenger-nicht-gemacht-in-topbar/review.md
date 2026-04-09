# Review: Story 42 - Suche "Länger nicht gemacht" in Top-Bar verschieben

**Review-Datum:** 2026-04-09
**Story-Status:** Implementiert

---

## Zusammenfassung

Die Funktionalität wurde erfolgreich implementiert. Der Link zur gefilterten Suche ("Länger nicht gemacht" + Mittagessen) ist nun global in der Top-Bar verfügbar und wurde aus der Wochenvorschau entfernt. Die neuen E2E-Tests bestätigen die korrekte Funktion. Allerdings führte die Entfernung des Buttons in der Wochenvorschau zu Regressionen in bestehenden Tests.

---

## Prüfung gegen den Plan

| Schritt | Status | Bemerkung |
|---------|--------|-----------|
| 1. Link in `base.html` integrieren | ✅ | Sichtbar und funktionsfähig (siehe E2E) |
| 2. Button in `wochenvorschau.html` entfernen | ✅ | Erfolgreich entfernt |
| 3. Manueller & Responsive Check | ✅ | Durch E2E-Tests verifiziert |
| 4. E2E-Tests implementieren | ✅ | Neue Tests in `navigation-inspiration.spec.ts` bestanden |

---

## Prüfung gegen Akzeptanzkriterien

| Kriterium | Status | Bemerkung |
|-----------|--------|-----------|
| **K1: Link in der Top-Bar** | ✅ | Vorhanden und deutlich sichtbar |
| **K2: Auslösung der Suche** | ✅ | Führt korrekt zu /?filter=laenger-nicht-gemacht&kategorie=Mittagessen |
| **K3: Entfernung aus Wochenvorschau** | ✅ | Button wurde entfernt |
| **K4: Barrierefreiheit** | ✅ | Tastaturnavigation funktioniert |
| **K5: Responsive Design** | ✅ | In mobiler Ansicht erreichbar |

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
- [x] App funktioniert ohne JavaScript
- [x] Code in korrekten Verzeichnissen

### Testing
- [x] Unit Tests geschrieben und bestanden (`cargo test`) — *mit Ausnahme von Regressionen*
- [x] E2E Tests geschrieben und bestanden (`npm run test:e2e`) — *mit Ausnahme von Regressionen*

### Funktionale Anforderungen
- [x] Alle Akzeptanzkriterien erfüllt
- [x] Edge Cases behandelt
- [x] Validierung vorhanden

---

## Test-Ergebnisse

### Unit-Tests / Integration-Tests
| Test | Status |
|------|--------|
| `tests/wochenvorschau.rs` | ❌ (Fails: `wochenvorschau_enthaelt_link_zur_not_made_suche`) |
| Alle anderen tests | ✅ |

### E2E-Tests
| Test | Status |
|------|--------|
| `tests/e2e/navigation-inspiration.spec.ts` | ✅ |
| `tests/e2e/filter-collapse.spec.ts` | ❌ (Fails: Story 40 K10) |
| Alle anderen tests | ✅ |

### Code-Quality Checks
| Check | Ergebnis |
|-------|----------|
| cargo build | ✅ |
| cargo clippy | ✅ |
| cargo fmt | ✅ |

---

## Empfohlene Nacharbeit

### Prio 1 (Muss — blockiert Abschluss)

1. **Update defekter Tests (Regressionen)**
   - Die Entfernung des Buttons in der Wochenvorschau (K3) hat bestehende Tests gebrochen, die genau diesen Button prüfen.
   - **Lösung:** `tests/wochenvorschau.rs` und `tests/e2e/filter-collapse.spec.ts` anpassen, um die Entfernung des Buttons zu validieren statt seine Existenz zu prüfen.

### Prio 2 (Sollte — nice-to-have)

Keine.

---

## Fazit

**Gesamtbewertung:** ⚠️ Nacharbeit erforderlich

Die funktionale Implementierung ist korrekt und erfüllt alle Akzeptanzkriterien. Die Story kann jedoch nicht abgeschlossen werden, da die Testsuite (Regressionen) aktuell fehlerhaft ist.

**Nächste Schritte:**
1. Update der betroffenen Integration- und E2E-Tests.
2. Erneuter Durchlauf aller Tests.
