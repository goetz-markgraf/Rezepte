# Review: Story 38 - Wochenplanung auf 15-Tage-Liste umbauen

**Review-Datum:** 2026-04-04
**Story-Status:** Implementiert — **Nacharbeit erforderlich**

---

## Zusammenfassung

Die Implementierung der Story 38 wurde begonnen, aber es fehlen wesentliche Funktionen. Der Code kompiliert nicht aufgrund fehlender Funktionsdefinitionen (`format_date_with_short_weekday`, `german_weekday_short`, `format_date_kurz`). Die Route und das Template wurden angepasst, aber die Kernlogik für die Datumsformatierung ist unvollständig.

---

## Prüfung gegen den Plan

| Schritt | Status | Bemerkung |
|---------|--------|-----------|
| 1. Route und Handler anpassen | ⚠️ | Handler angepasst für 15-Tage-Logik, aber fehlende Importe/Funktionen |
| 2. Template-Struktur anpassen | ✅ | `WochenvorschauTemplate` angepasst, Navigation-URLs entfernt |
| 3. HTML-Template anpassen | ✅ | Navigation entfernt, vertikale Liste umgesetzt |
| 4. Datenbank-Layer | ✅ | Keine Änderungen nötig — korrekt |
| 5. CSS/Styling anpassen | ✅ | Bestehende Styles werden verwendet |
| 6. Integrationstests anpassen | ❌ | Tests prüfen noch 7-Tage-Logik und KW-Anzeige |
| 7. E2E-Tests erstellen | ✅ | E2E-Tests für 15-Tage-Liste vorhanden |
| 8. Barrierefreiheit prüfen | ⚠️ | Semantische Struktur ok, aber keine ARIA-Labels für Datumsangaben |
| 9. DoD-Check | ❌ | Build fehlschlägt |

---

## Prüfung gegen Akzeptanzkriterien

| Kriterium | Status | Bemerkung |
|-----------|--------|-----------|
| **K1: 15-Tage-Liste anzeigen** | ❌ | Funktionen fehlen, Build schlägt fehl |
| **K2: Navigation entfernen** | ✅ | Navigation entfernt aus Template und Handler |
| **K3: Darstellung pro Tag** | ⚠️ | Template vorhanden, aber nicht kompilierbar |
| **K4: Datumsklick-Verhalten** | ✅ | Links zu Rezepten vorhanden |
| **K5: Performance** | ⚠️ | Nicht testbar — Build fehlschlägt |
| **K6: Barrierefreiheit** | ⚠️ | Grundstruktur ok, aber ARIA-Labels fehlen für Datumsangaben |

---

## Prüfung gegen Definition of Done

### Code-Qualität
- [ ] `cargo build` — ❌ Fehler: Fehlende Funktionen
- [ ] `cargo clippy -- -D warnings` — ❌ Nicht durchführbar (Build-Fehler)
- [ ] `cargo fmt --check` — ⚠️ Nicht geprüft
- [ ] Keine ungenutzten Funktionen / Variablen — ⚠️ Alte Funktionen noch mit `#[allow(dead_code)]`

### Architektur-Einhaltung
- [ ] Tech Stack: Rust + Axum + Askama + sqlx + SQLite + HTMX — ✅
- [ ] SSR, keine JSON-APIs für UI — ✅
- [ ] App funktioniert ohne JavaScript — ✅
- [ ] Code in korrekten Verzeichnissen — ✅

### Testing
- [ ] Unit Tests geschrieben und bestanden — ❌ Tests für fehlende Funktionen vorhanden, aber Funktionen fehlen
- [ ] E2E Tests geschrieben und bestanden — ❌ Build-Fehler verhindert Test-Ausführung

### Funktionale Anforderungen
- [ ] Alle Akzeptanzkriterien erfüllt — ❌ K1 nicht erfüllt
- [ ] Edge Cases behandelt — ⚠️ Leere Tage werden im Template behandelt
- [ ] Validierung vorhanden — ✅

---

## Test-Ergebnisse

### Unit-Tests
| Test | Status |
|------|--------|
| cargo test | ❌ Build fehlschlägt mit 14 Fehlern |

**Fehler:**
```
error[E0425]: cannot find function `format_date_with_short_weekday` in this scope
error[E0425]: cannot find function `german_weekday_short` in this scope
error[E0425]: cannot find function `format_date_kurz` in this scope
```

### E2E-Tests
| Test | Status |
|------|--------|
| npm run test:e2e | ❌ WebServer startet nicht (Build-Fehler) |

### Code-Quality Checks
| Check | Ergebnis |
|-------|----------|
| cargo build | ❌ Fehler: Fehlende Funktionen |
| cargo clippy | ❌ Nicht durchführbar |
| cargo fmt | ⚠️ Nicht geprüft |

---

## Empfohlene Nacharbeit

### Prio 1 (Muss — blockiert Abschluss)

1. **Fehlende Funktionen implementieren**
   - `german_weekday_short()` — Kurze Wochentagsnamen ("Mo", "Di", ...)
   - `format_date_with_short_weekday()` — Format "Fr, 04.04.2026"
   - `format_date_kurz()` — Import aus `heute.rs` oder neu implementieren
   - **Datei:** `src/routes/wochenvorschau.rs` (vor den Tests, Zeile ~226)

2. **Integrationstests aktualisieren**
   - `tests/wochenvorschau.rs` prüft noch auf 7 Tage statt 15
   - Tests für KW-Anzeige entfernen/aktualisieren
   - Tests für Navigation entfernen
   - **Zeilen:** 88-111, 247-259, 296-330

3. **Dead Code entfernen**
   - Alte Wochen-Logik-Funktionen mit `#[allow(dead_code)]` entfernen oder bereinigen
   - `format_kw_header`, `iso_week_number`, `parse_iso_week` etc.

### Prio 2 (Sollte — nice-to-have)

1. **ARIA-Labels für Datumsangaben**
   - Aktuell: `<span class="wochentag-datum">{{ tag.datum_kurz }}</span>`
   - Besser: `<time datetime="..." aria-label="...">`

2. **Template-Text anpassen**
   - Zeile 27: "Für diese Woche noch nichts geplant" → "Für die nächsten 15 Tage noch nichts geplant"

3. **Leere Import-Anweisung**
   - `use crate::models::get_recipes_current_week;` — prüfen ob diese Funktion existiert oder `get_recipes_by_date_range` verwenden

---

## Fazit

**Gesamtbewertung:** ❌ Abgelehnt — Prio-1-Nacharbeit erforderlich

Die grundlegende Struktur ist vorhanden, aber die Implementierung ist unvollständig. Die fehlenden Funktionen für die Datumsformatierung verhindern einen erfolgreichen Build. Nachdem diese Funktionen implementiert und die Tests angepasst wurden, sollte die Story erneut geprüft werden.

**Nächste Schritte:**
1. Fehlende Funktionen in `src/routes/wochenvorschau.rs` implementieren
2. Integrationstests aktualisieren (`tests/wochenvorschau.rs`)
3. Neuen Review durchführen
