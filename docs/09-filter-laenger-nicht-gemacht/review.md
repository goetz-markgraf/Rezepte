# Review: Story 9 - Filter "Länger nicht gemacht"

**Review-Datum:** 2026-03-28
**Story-Status:** Implementiert

---

## Zusammenfassung

Der Filter "Länger nicht gemacht" ist vollständig implementiert: DB-Query, Handler-Logik, Template und CSS. Alle 54 Unit-Tests und 8 Integrationstests bestehen, alle 7 E2E-Tests (K1–K7) sind grün. Die Implementierung hält alle Architektur-Vorgaben (SSR, HTMX, kein JS-Pflicht, DeepLink) ein. Eine kleinere Inkonsistenz im URL-Parameter zwischen architecture.md und der Story bleibt offen; sie ist kein funktionales Problem, aber dokumentarisch unbereinigt.

---

## Prüfung gegen den Plan

| Schritt | Status | Bemerkung |
|---------|--------|-----------|
| 1. DB-Layer: `filter_recipes_not_made_recently` | ✅ | Korrekte WHERE-Bedingung und Sortierung; alle 8 Unit-Tests implementiert und grün |
| 2. Modell-Layer: Re-Export + `IndexQuery.filter` | ✅ | Re-Export in `models/mod.rs`, `filter: Option<String>` in `IndexQuery` |
| 3. Route/Handler: Filter-Logik + `build_not_made_toggle_url` | ✅ | Handler, Toggle-URL, Kategorie-URLs behalten `filter`-Parameter |
| 4. Template: Filter-Button, Keine-Treffer-Meldungen, Hidden-Input | ✅ | Alle Varianten umgesetzt, `aria-pressed`, HTMX-Attribute |
| 5. CSS: `.sort-filter`, `.sort-filter-btn`, aktiver Zustand | ✅ | Visuell abgesetzt (lila), responsiv, Fokus-Indikator |
| 6. Integrationstests (`tests/recipe_not_made_filter.rs`) | ✅ | 8 Tests, alle grün, Given/When/Then-Kommentare vorhanden |
| 7. E2E-Tests (`tests/e2e/recipe-not-made-filter.spec.ts`) | ✅ | 7 Tests (K1–K7), alle grün, deutsche Kommentare vorhanden |
| 8. Qualitätschecks (clippy, fmt, test, e2e) | ✅ | Alle Checks fehlerfrei |

---

## Prüfung gegen Akzeptanzkriterien

| Kriterium | Status | Bemerkung |
|-----------|--------|-----------|
| **K1: Filter aktivierbar** | ✅ | Button "Länger nicht gemacht" sichtbar, Klick aktiviert, `aria-pressed` korrekt, E2E-Test grün |
| **K2: Sortierung aufsteigend, NULL zuerst** | ✅ | SQL: `CASE WHEN planned_date IS NULL THEN 0 ELSE 1 END ASC, planned_date ASC`; Rust-Sekundärsort für gleiche Daten; Unit + E2E grün |
| **K3: Zukunftsdaten ausgeschlossen** | ✅ | `WHERE planned_date IS NULL OR planned_date <= DATE('now')`; Unit + E2E grün |
| **K4: Filter zurücksetzen** | ✅ | Toggle-Logik: aktiver Filter → URL ohne `filter`-Parameter; E2E grün |
| **K5: Keine Treffer** | ✅ | Kontextabhängige Hinweistexte in Template; E2E grün |
| **K6: DeepLink-fähige URL** | ✅ | `?filter=laenger-nicht-gemacht` wird korrekt ausgewertet; E2E grün |
| **K7: Kombination mit Kategorie-Filter** | ✅ | Kategorie-Toggle-URLs tragen `filter`-Parameter weiter; E2E grün |
| **K8: Kombination mit Volltextsuche** | ✅ | `search_clause` in `filter_recipes_not_made_recently`; Integrationstest grün |
| **K9: Performance** | ✅ | Kein N+1, bestehender Index `idx_recipes_planned_date` genutzt (laut architecture.md) |
| **K10: Barrierefreiheit** | ✅ | `aria-pressed` korrekt, Fokus-Indikator (`:focus-visible`), aussagekräftiges Label |

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
- [x] App funktioniert ohne JavaScript (normaler `<a>`-Link, versteckter `<input type="hidden">`)
- [x] Code in korrekten Verzeichnissen

### Testing
- [x] Unit Tests geschrieben und bestanden (`cargo test` — 54/54)
- [x] Integrationstests geschrieben und bestanden (`cargo test` — 8 neue Tests)
- [x] E2E Tests geschrieben und bestanden (`npm run test:e2e` — 7 neue Tests)
- [x] Given/When/Then-Kommentare in allen Tests vorhanden

### Funktionale Anforderungen
- [x] Alle Akzeptanzkriterien erfüllt (K1–K10)
- [x] Edge Cases behandelt (NULL-Daten, Zukunftsdaten, leere Ergebnisse, Kombinationen)
- [x] Öffentliche Funktionen mit Doc-Kommentaren versehen

---

## Test-Ergebnisse

### Unit-Tests (Story-9-spezifisch, in `src/models/recipe_db.rs`)

| Test | Status |
|------|--------|
| `not_made_recently_null_dates_appear_first` | ✅ |
| `not_made_recently_sorted_by_date_ascending` | ✅ |
| `not_made_recently_excludes_future_dates` | ✅ |
| `not_made_recently_includes_past_and_null` | ✅ |
| `not_made_recently_returns_empty_if_all_future` | ✅ |
| `not_made_recently_same_date_sorted_alphabetically` | ✅ |
| `not_made_recently_combined_with_category_filter` | ✅ |
| `not_made_recently_combined_with_search_query` | ✅ |

### Integrationstests (`tests/recipe_not_made_filter.rs`)

| Test | Status |
|------|--------|
| `not_made_filter_returns_200_with_correct_recipes` | ✅ |
| `not_made_filter_excludes_future_dated_recipes` | ✅ |
| `not_made_filter_shows_null_date_recipes_first` | ✅ |
| `not_made_filter_shows_empty_state_message` | ✅ |
| `not_made_filter_combined_with_category` | ✅ |
| `not_made_filter_combined_with_search` | ✅ |
| `deeplink_not_made_filter_returns_correct_state` | ✅ |
| `no_filter_param_returns_alphabetical_list` | ✅ |

### E2E-Tests (`tests/e2e/recipe-not-made-filter.spec.ts`)

| Test | Status |
|------|--------|
| K1: Filter-Button sichtbar und aktivierbar | ✅ |
| K2: Sortierung nach Datum aufsteigend, NULL-Daten zuerst | ✅ |
| K3: Zukunftsdaten werden ausgeschlossen | ✅ |
| K4: Filter zurücksetzen | ✅ |
| K5: Keine passenden Rezepte — Hinweistext erscheint | ✅ |
| K6: DeepLink `?filter=laenger-nicht-gemacht` | ✅ |
| K7: Kombination mit Kategorie-Filter | ✅ |

### Code-Quality Checks

| Check | Ergebnis |
|-------|----------|
| `cargo build` | ✅ |
| `cargo clippy -- -D warnings` | ✅ |
| `cargo fmt --check` | ✅ |
| `cargo test` (gesamt: 118 Tests) | ✅ |
| `npm run test:e2e` (gesamt: 69 Tests) | ✅ |

---

## Empfohlene Nacharbeit

### Prio 1 (Muss — blockiert Abschluss)

Keine.

### Prio 2 (Sollte — nice-to-have)

1. **URL-Parameter-Inkonsistenz in `docs/product/architecture.md`**
   - Die Architektur-Dokumentation verwendet `filter=not-made` (Zeilen 194 und 215), die Implementierung und Story verwenden `filter=laenger-nicht-gemacht`.
   - Das war als offener Punkt im `plan.md` dokumentiert und zugunsten der nutzerlesbaren URL entschieden worden, aber `architecture.md` wurde nicht nachgezogen.
   - Empfehlung: `architecture.md` auf `filter=laenger-nicht-gemacht` aktualisieren.

2. **Sekundärsortierung: doppelter Sort (SQL + Rust)**
   - `filter_recipes_not_made_recently` sortiert zuerst per SQL (`ORDER BY ... planned_date ASC`), dann nochmals per Rust-`sort_by` über das gleiche `planned_date`-Feld plus Titel.
   - Der Rust-Sort ist eine stabile Operation und korrekt für die alphabetische Sekundärsortierung bei gleichem Datum — aber er überschreibt dabei auch die SQL-Primärsortierung neu (da `Option<T>: Ord` `None < Some(...)`).
   - Das Verhalten ist in Tests korrekt belegt, aber es wäre klarer, den Rust-Sort auf reine Titel-Sortierung bei gleichem Datum zu beschränken und die Datumsreihenfolge ausschließlich der DB zu überlassen. Aktuell ist es zufällig konsistent, da `Option<Date>` dieselbe Reihenfolge produziert wie der SQL-`CASE`.
   - Empfehlung: Rust-Sort explizit nur auf Titel beschränken wenn `a.planned_date == b.planned_date`, ohne `date_cmp` als Fallback — oder einen erklärenden Kommentar ergänzen, der die Redundanz bewusst benennt.

---

## Fazit

**Gesamtbewertung:** ✅ Abgenommen

Die Implementierung ist vollständig, alle Tests grün, kein Clippy-Warning, kein Formatierungsfehler. Alle 10 Akzeptanzkriterien sind erfüllt, die Architektur-Vorgaben werden eingehalten. Die zwei Prio-2-Punkte (Docs-Inkonsistenz und redundanter Doppel-Sort) sind funktional unbedenklich und können in einem Folge-Commit bereinigt werden.

**Nächste Schritte:**
1. `docs/product/architecture.md` — URL-Parameter auf `laenger-nicht-gemacht` korrigieren (Prio 2)
2. Kommentar oder Refactoring des Rust-Sekundärsorts in `filter_recipes_not_made_recently` (Prio 2)
3. Story-Status auf "Abgeschlossen" setzen
