# Review: Story 10 - Filter "Nächste 7 Tage"

**Review-Datum:** 2026-03-29
**Story-Status:** Implementiert

---

## Zusammenfassung

Der Filter "Nächste 7 Tage" ist vollständig implementiert und folgt dem etablierten Muster aus Story 9. Alle 9 funktionalen Akzeptanzkriterien (K1–K9) sind erfüllt, alle Tests (65 Unit-Tests, 10 Integrationstests, 9 E2E-Tests für Story 10, insgesamt 84 E2E-Tests) sind grün. Ein kleiner Nachbesserungspunkt besteht bei der Architektur-Dokumentation (`architecture.md`), die den neuen URL-Parameter noch nicht enthält.

---

## Prüfung gegen den Plan

| Schritt | Status | Bemerkung |
|---------|--------|-----------|
| 1. DB-Layer `filter_recipes_next_seven_days` | ✅ | Funktion vollständig implementiert mit SQL-Zeitfenster, Kategorie- und Suchklausel, Sekundärsortierung via Rust `sort_by` |
| 2. Modell-Layer Re-Export + `IndexQuery` | ✅ | Re-Export in `mod.rs` vorhanden, `IndexQuery.filter` unverändert korrekt wiederverwendet |
| 3. Route/Handler — Filter-Logik | ✅ | `build_next_seven_days_toggle_url`, `build_category_toggle_url`, `build_reset_url`, `build_category_filters` alle erweitert, `IndexTemplate` befüllt |
| 4. Template — Filter-UI | ✅ | Filter-Button, Hidden-Input im Suchformular, "Alle"-Button mit korrekter `aria-pressed`-Logik, alle 4 Keine-Treffer-Meldungen, Wochentag-Datum auf Karte |
| 5. CSS — Styling | ✅ | `.sort-filter` mit `flex-wrap`, `.recipe-date-weekday` mit `font-weight: 500` vorhanden |
| 6. Integrationstests | ✅ | 10 Tests in `tests/recipe_next_seven_days_filter.rs`, alle grün |
| 7. E2E-Tests | ✅ | 9 Tests für K1–K9 in `tests/e2e/recipe-next-seven-days-filter.spec.ts`, alle grün |
| 8. Qualitätschecks DoD | ⚠️ | `cargo fmt`, `clippy`, `cargo test`, `npm run test:e2e` bestanden; `architecture.md` nicht aktualisiert (s. Nacharbeit) |

---

## Prüfung gegen Akzeptanzkriterien

| Kriterium | Status | Bemerkung |
|-----------|--------|-----------|
| **K1: Filter aktivierbar** | ✅ | Button "Nächste 7 Tage" sichtbar, `aria-pressed` korrekt, aktiver Zustand mit CSS-Klasse `active` hervorgehoben |
| **K2: Nur Rezepte innerhalb des Zeitfensters** | ✅ | SQL `WHERE planned_date >= DATE('now') AND planned_date <= DATE('now', '+7 days')` schließt NULL, Vergangenheit und >Tag 7 aus |
| **K3: Sortierung chronologisch aufsteigend** | ✅ | `ORDER BY planned_date ASC` in SQL, Sekundärsortierung alphabetisch via `normalize_for_sort` in Rust |
| **K4: Filter zurücksetzen** | ✅ | Toggle-Mechanismus: Klick auf aktiven Filter entfernt `filter`-Parameter, alphabetische Ansicht kehrt zurück |
| **K5: Keine Treffer** | ✅ | 4 Kombinationen abgedeckt: nur Filter, Filter+Suche, Filter+Kategorie, Filter+Suche+Kategorie |
| **K6: DeepLink-fähige URL** | ✅ | `?filter=naechste-7-tage` korrekt implementiert, direkte URL-Aufrufe funktionieren |
| **K7: Datum mit Wochentag auf Karte** | ✅ | `planned_date_weekday` in `RecipeListItem`, Format "Mo, 31.03.2026", nur bei aktivem Filter angezeigt |
| **K8: Kombination mit Kategorie-Filter** | ✅ | Beide Parameter gleichzeitig in URL, kombinierte Filterung getestet |
| **K9: Kombination mit Volltextsuche** | ✅ | Suche + Filter kombiniert getestet |
| **K10: Performance** | ✅ | SQLite-Index `idx_recipes_planned_date` bereits vorhanden, kein N+1-Problem |
| **K11: Barrierefreiheit** | ✅ | `aria-pressed` korrekt (true/false), beschreibendes Label "Nächste 7 Tage", Fokus-Indikator via CSS |

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
- [x] App funktioniert ohne JavaScript (Form-Post + Redirect-Muster, Hidden-Inputs im Suchformular)
- [x] Code in korrekten Verzeichnissen

### Testing
- [x] Unit Tests geschrieben und bestanden (`cargo test`) — 11 Unit-Tests in `recipe_db.rs`
- [x] Integrationstests geschrieben und bestanden (`cargo test`) — 10 Tests in `tests/recipe_next_seven_days_filter.rs`
- [x] E2E Tests geschrieben und bestanden (`npm run test:e2e`) — 9 Tests für K1–K9

### Funktionale Anforderungen
- [x] Alle Akzeptanzkriterien erfüllt
- [x] Edge Cases behandelt (NULL-Datum, Vergangenheit, exakt Tag 7, Tag 8)
- [x] Validierung nicht tangiert (kein neues User-Input-Feld)

### Dokumentation
- [x] `filter_recipes_next_seven_days` hat vollständigen Doc-Kommentar (`///`)
- [x] `index`-Handler Doc-Kommentar aktualisiert (neuer Filter dokumentiert)
- [ ] `architecture.md` URL-Tabelle nicht aktualisiert — `?filter=naechste-7-tage` fehlt

---

## Test-Ergebnisse

### Unit-Tests (`src/models/recipe_db.rs`)

| Test | Status |
|------|--------|
| `next_seven_days_returns_recipes_within_window` | ✅ |
| `next_seven_days_includes_today` | ✅ |
| `next_seven_days_includes_day_seven` | ✅ |
| `next_seven_days_excludes_past_dates` | ✅ |
| `next_seven_days_excludes_null_dates` | ✅ |
| `next_seven_days_excludes_day_eight` | ✅ |
| `next_seven_days_sorted_chronologically` | ✅ |
| `next_seven_days_same_date_sorted_alphabetically` | ✅ |
| `next_seven_days_combined_with_category_filter` | ✅ |
| `next_seven_days_combined_with_search_query` | ✅ |
| `next_seven_days_returns_empty_when_no_recipes_in_window` | ✅ |

### Integrationstests (`tests/recipe_next_seven_days_filter.rs`)

| Test | Status |
|------|--------|
| `next_seven_days_filter_returns_200_with_recipes_in_window` | ✅ |
| `next_seven_days_filter_excludes_past_dates` | ✅ |
| `next_seven_days_filter_excludes_null_dates` | ✅ |
| `next_seven_days_filter_excludes_dates_beyond_seven_days` | ✅ |
| `next_seven_days_filter_includes_today` | ✅ |
| `next_seven_days_filter_shows_empty_state_message` | ✅ |
| `next_seven_days_filter_combined_with_category` | ✅ |
| `next_seven_days_filter_combined_with_search` | ✅ |
| `deeplink_next_seven_days_filter_returns_correct_state` | ✅ |
| `no_filter_param_returns_alphabetical_list` | ✅ |

### E2E-Tests (`tests/e2e/recipe-next-seven-days-filter.spec.ts`)

| Test | Status |
|------|--------|
| K1: Filter-Button ist sichtbar und aktivierbar | ✅ |
| K2: Nur Rezepte im Zeitfenster werden angezeigt | ✅ |
| K3: Chronologische Sortierung (früheres Datum zuerst) | ✅ |
| K3: Zeitfenster-Grenzen (heute inklusive, Tag 7 inklusive, Tag 8 und gestern exklusiv) | ✅ |
| K4: Filter zurücksetzen via Toggle | ✅ |
| K5: Keine Treffer — Hinweistext erscheint | ✅ |
| K6: DeepLink `/?filter=naechste-7-tage` | ✅ |
| K7: Datum mit Wochentag auf Rezeptkarte bei aktivem Filter | ✅ |
| K8: Kombination mit Kategorie-Filter | ✅ |
| K9: Kombination mit Volltextsuche | ✅ |

### Code-Quality Checks

| Check | Ergebnis |
|-------|----------|
| `cargo build` | ✅ |
| `cargo clippy -- -D warnings` | ✅ |
| `cargo fmt --check` | ✅ |
| `cargo test` (65 Unit-Tests + 78 Integrationstests) | ✅ |
| `npm run test:e2e` (84 Tests gesamt) | ✅ |

---

## Empfohlene Nacharbeit

### Prio 1 (Muss — blockiert Abschluss)

Keine blockierenden Probleme.

### Prio 2 (Sollte — nice-to-have)

1. **`architecture.md` URL-Tabelle aktualisieren**
   - Zeile 216 enthält noch den veralteten Entwurf `| planned | next-7-days | Geplante Rezepte |`
   - Sollte ersetzt werden durch: `| filter | naechste-7-tage | "Nächste 7 Tage"-Filter |`
   - Zeile 194 enthält auch `GET /recipes?filter=laenger-nicht-gemacht` (falsch: `/recipes` statt `/`) und es fehlt die entsprechende Zeile für `?filter=naechste-7-tage`
   - Kein funktionaler Einfluss, aber die Dokumentation beschreibt nicht den tatsächlichen Ist-Zustand

---

## Fazit

**Gesamtbewertung:** ✅ Abgenommen

Die Implementierung ist vollständig, korrekt und folgt konsistent den Mustern aus Story 9. Alle 11 Akzeptanzkriterien sind erfüllt, der gesamte Testaufbau ist sauber mit Given/When/Then-Kommentaren dokumentiert. Das einzige offene Thema ist die veraltete `architecture.md`-Tabelle, die als Prio-2-Aufgabe nachgezogen werden kann.

**Nächste Schritte:**
1. `architecture.md` URL-Tabelle um `?filter=naechste-7-tage` ergänzen (Prio 2, kann im Rahmen der nächsten Story oder als separater Commit erfolgen)
2. Story als abgeschlossen markieren
