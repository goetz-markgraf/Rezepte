# Review: Story N - [Titel]

**Review-Datum:** YYYY-MM-DD
**Story-Status:** Implementiert

---

## Zusammenfassung

[2-3 Sätze: Was wurde implementiert? Was ist der Gesamteindruck? Gibt es Nacharbeit?]

---

## Prüfung gegen den Plan

| Schritt | Status | Bemerkung |
|---------|--------|-----------|
| 1. [Schritt-Titel] | ✅ / ⚠️ / ❌ | [Bemerkung] |
| 2. [Schritt-Titel] | ✅ / ⚠️ / ❌ | [Bemerkung] |

---

## Prüfung gegen Akzeptanzkriterien

| Kriterium | Status | Bemerkung |
|-----------|--------|-----------|
| **K1: [Titel]** | ✅ / ⚠️ / ❌ | [Bemerkung] |
| **K2: [Titel]** | ✅ / ⚠️ / ❌ | [Bemerkung] |

---

## Prüfung gegen Definition of Done

### Code-Qualität
- [ ] `cargo build` — fehlerfrei
- [ ] `cargo clippy -- -D warnings` — keine Warnungen
- [ ] `cargo fmt --check` — korrekt formatiert
- [ ] Keine ungenutzten Funktionen / Variablen

### Architektur-Einhaltung
- [ ] Tech Stack: Rust + Axum + Askama + sqlx + SQLite + HTMX
- [ ] SSR, keine JSON-APIs für UI
- [ ] App funktioniert ohne JavaScript
- [ ] Code in korrekten Verzeichnissen

### Testing
- [ ] Unit Tests geschrieben und bestanden (`cargo test`)
- [ ] E2E Tests geschrieben und bestanden (`npm run test:e2e`)

### Funktionale Anforderungen
- [ ] Alle Akzeptanzkriterien erfüllt
- [ ] Edge Cases behandelt
- [ ] Validierung vorhanden

---

## Test-Ergebnisse

### Unit-Tests
| Test | Status |
|------|--------|
| [Test-Name] | ✅ / ❌ |

### E2E-Tests
| Test | Status |
|------|--------|
| [Test-Name] | ✅ / ❌ |

### Code-Quality Checks
| Check | Ergebnis |
|-------|----------|
| cargo build | ✅ / ❌ |
| cargo clippy | ✅ / ❌ |
| cargo fmt | ✅ / ❌ |

---

## Empfohlene Nacharbeit

### Prio 1 (Muss)

1. **[Problem]**
   - [Beschreibung des Problems]
   - [Empfohlene Lösung]

### Prio 2 (Sollte)

2. **[Problem]**
   - [Beschreibung]

---

## Fazit

**Gesamtbewertung:** ✅ Abgenommen / ⚠️ Nacharbeit erforderlich / ❌ Abgelehnt

[Begründung der Bewertung in 1-2 Sätzen]

**Nächste Schritte:**
1. [Aktion]
2. [Aktion]

