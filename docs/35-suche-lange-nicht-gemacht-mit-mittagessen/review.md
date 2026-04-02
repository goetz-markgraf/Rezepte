# Review: Story 35 - Suche "Länger nicht gemacht" mit vorselektiertem Mittagessen-Filter

**Review-Datum:** 2026-04-02
**Story-Status:** Implementiert

---

## Zusammenfassung

Story 35 ist eine minimale, saubere Erweiterung von Story 34: Der bestehende Link in der Wochenvorschau-Toolbar erhält den zusätzlichen Query-Parameter `kategorie=Mittagessen`. Die Implementierung ist korrekt, übersichtlich und entspricht dem Plan. Alle funktionalen Akzeptanzkriterien (K1–K4) sind erfüllt. Ein E2E-Testfall aus dem Plan fehlt (T6: keine passenden Rezepte), was als Prio-2-Punkt bewertet wird.

---

## Prüfung gegen den Plan

| Schritt | Status | Bemerkung |
|---------|--------|-----------|
| 1. E2E-Tests schreiben (TDD, rot) | ✅ | T1–T5 in `tests/e2e/wochenvorschau-not-made.spec.ts` vorhanden; T6 fehlt |
| 2. Template-Änderung | ✅ | URL und ARIA-Label korrekt angepasst (`templates/wochenvorschau.html:31-33`) |
| 3. Rust-Integrationstest aktualisieren | ✅ | `wochenvorschau_enthaelt_link_zur_not_made_suche` prüft jetzt auch `kategorie=Mittagessen` |
| 4. E2E-Tests grün | ✅ | Alle 5 Story-35-Tests bestehen |
| 5. Code-Qualität | ✅ | Clippy, fmt, build fehlerfrei |

---

## Prüfung gegen Akzeptanzkriterien

| Kriterium | Status | Bemerkung |
|-----------|--------|-----------|
| **K1: Klick aus Wochenübersicht öffnet Suche mit beiden Filtern** | ✅ | URL enthält beide Parameter, beide Filter visuell aktiv (E2E T1+T2) |
| **K2: URL enthält beide Filter-Parameter** | ✅ | `?filter=laenger-nicht-gemacht&kategorie=Mittagessen`; DeepLink getestet (E2E T4) |
| **K3: Vorselektierter Filter bleibt interaktiv** | ✅ | Mittagessen-Filter kann manuell abgewählt werden (E2E T5) |
| **K4: Keine Treffer werden klar kommuniziert** | ✅ | Mechanismus existiert (aus Story 9/12); kein dedizierter Test für diesen Edge Case |
| **K5: Performance** | ✅ | Keine zusätzliche Serverlogik — reine Template-Änderung, kein Mehraufwand |
| **K6: Barrierefreiheit** | ✅ | ARIA-Label angepasst; `aria-pressed` auf Kategorie-Button durch bestehende Implementierung; Tastatur-Test besteht |

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
- [x] App funktioniert ohne JavaScript (Link mit Query-Parametern)
- [x] Code in korrekten Verzeichnissen

### Testing
- [x] Rust-Integrationstests aktualisiert und bestanden (`cargo test`)
- [x] E2E-Tests für Story 35 geschrieben und bestanden (`npm run test:e2e`)

### Funktionale Anforderungen
- [x] Alle Akzeptanzkriterien erfüllt
- [x] Edge Cases: "Keine Treffer"-Meldung durch vorhandenen Mechanismus abgedeckt
- [x] Validierung: Kein neuer User-Input — entfällt

---

## Test-Ergebnisse

### Unit-/Integrationstests (Rust)
| Test | Status |
|------|--------|
| `wochenvorschau_enthaelt_link_zur_not_made_suche` | ✅ |
| Alle 19 `wochenvorschau.rs`-Tests | ✅ |
| Alle 278 Tests gesamt | ✅ |

### E2E-Tests (Playwright)
| Test | Status |
|------|--------|
| T1: Klick öffnet URL mit `filter=laenger-nicht-gemacht&kategorie=Mittagessen` | ✅ |
| T2: Kategorie-Filter "Mittagessen" als aktiv markiert (`aria-pressed="true"`) | ✅ |
| T3: Nur Mittagessen-Rezepte angezeigt, Brot-Rezepte nicht | ✅ |
| T4: DeepLink mit beiden Parametern funktioniert | ✅ |
| T5: Mittagessen-Filter kann manuell abgewählt werden | ✅ |
| T6: Keine passenden Mittagessen-Rezepte — Hinweistext | ❌ nicht implementiert |
| Alle Story-34-Tests weiterhin grün | ✅ |
| Gesamtergebnis: 223 passed, 1 skipped (unrelated) | ✅ |

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

1. **Fehlender E2E-Testfall T6**
   - Im Plan (Zeile 110) und Story (K4, Testfall 5) ist ein Test vorgesehen: "Alle Mittagessen-Rezepte haben Zukunftsdaten → Hinweistext erscheint"
   - Der Test fehlt in `tests/e2e/wochenvorschau-not-made.spec.ts`
   - Die Funktionalität selbst ist korrekt (wird durch den bestehenden "empty state"-Mechanismus aus Story 9/12 abgedeckt), aber der Test fehlt zur Dokumentation des Edge Cases
   - Empfehlung: Test T6 analog zu T3/T5 mit Rezept mit Zukunftsdatum ergänzen

---

## Fazit

**Gesamtbewertung:** ✅ Abgenommen

Die Implementierung ist korrekt, minimal und risikolos. Die einzige Code-Änderung (URL-Parameter im Template) ist genau auf das Story-Ziel zugeschnitten. Alle Qualitätschecks bestehen, alle funktionalen Akzeptanzkriterien sind erfüllt. Der fehlende T6-Test ist ein Prio-2-Punkt ohne Auswirkung auf die Funktionalität.

**Nächste Schritte:**
1. Optional: T6-Test in `tests/e2e/wochenvorschau-not-made.spec.ts` ergänzen
2. Story als abgeschlossen markieren
