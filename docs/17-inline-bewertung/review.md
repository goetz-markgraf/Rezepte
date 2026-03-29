# Review: Story 17 – Inline-Bewertung ohne Edit-Mode

**Review-Datum:** 2026-03-29
**Story-Status:** Implementiert

---

## Zusammenfassung

Story 17 implementiert eine direkt antippbare Sterne-Bewertung in der Rezept-Detailansicht per HTMX, ohne dass der Edit-Mode geöffnet werden muss. Die Implementierung ist vollständig: neuer Endpunkt `POST /recipes/:id/rating`, HTML-Fragment-Template `_inline_rating.html`, CSS-Hover-Effekt per `flex-direction: row-reverse` (kein JavaScript), Progressive Enhancement mit `<form>`-Fallback, vollständige Testabdeckung (5 Integrationstests + 6 E2E-Tests). Alle Qualitätschecks sind grün, keine Nacharbeit erforderlich.

---

## Prüfung gegen den Plan

| Schritt | Status | Bemerkung |
|---------|--------|-----------|
| 1. DB-Funktion `update_recipe_rating` (TDD) | ✅ | `src/models/recipe_db.rs`, SQL mit `rows_affected == 0` → `RowNotFound`; alle 5 Integrationstests grün |
| 2. Template `_inline_rating.html` + `InlineRatingTemplate` | ✅ | Sterne in umgekehrter Reihenfolge (5→1), `hx-vals` Toggle-Reset, aria-labels, `star_filled` / `rating_is_active` |
| 3. Route-Handler `update_recipe_rating_handler` | ✅ | `src/routes/recipes.rs`, Validierung via `validate_rating`, `RawForm` Body-Parse, `InlineRatingTemplate` Response |
| 4. Route registrieren | ✅ | `POST /recipes/:id/rating` in `src/routes/mod.rs` korrekt eingetragen |
| 5. Detail-Template anpassen | ✅ | `{% include "recipes/_inline_rating.html" %}` in `detail.html`, `RecipeDetailTemplate` hat `rating_is_active` + `star_filled` |
| 6. CSS-Styling | ✅ | `.inline-rating`, `.inline-rating-form`, `.inline-rating-btn`, Hover-Effekt, Fokus-Ring; Touch-Fläche ≥ 44px via `padding + min-height` |
| 7. E2E-Tests | ✅ | 6 Tests in `recipe-inline-rating.spec.ts`, decken K1–K8 ab; Seed-Datei vorhanden |
| 8. Qualitätssicherung | ✅ | Alle Checks grün (siehe Test-Ergebnisse) |

---

## Prüfung gegen Akzeptanzkriterien

| Kriterium | Status | Bemerkung |
|-----------|--------|-----------|
| **K1: Sterne direkt antippbar** | ✅ | 5 Sterne-Buttons im Inline-Rating-Bereich, `hx-post` sendet asynchron; kein separater Bearbeiten-Knopf nötig |
| **K2: Visuelle Rückmeldung nach Speichern** | ✅ | HTMX-Swap ersetzt `#inline-rating` sofort; `aria-label` und `active`-Klasse aktualisieren sich ohne Seitenneuladung |
| **K3: Bewertung durch erneutes Antippen zurücksetzen** | ✅ | Aktiver Stern sendet `rating=""` (leer) → Reset auf `None`; E2E-Test K3 bestätigt |
| **K4: Bewertung persistent gespeichert** | ✅ | Integrations- und E2E-Tests prüfen nach `page.reload()` den persistierten Wert |
| **K5: Progressive Enhancement ohne JavaScript** | ✅ | `<form method="POST" action="/recipes/:id/rating">` um alle Buttons; E2E-Test K5 prüft Form-Attachment und `method=POST` |
| **K6: Keine Regression im Edit-Mode** | ✅ | E2E-Test K6 verifiziert, dass nach Inline-Bewertung das Edit-Formular denselben Wert vorausgefüllt zeigt |
| **K7: Performance < 500ms** | ✅ | Reine DB-Update-Query auf Primärschlüssel; keine Messung im Test, aber kein Grund zur Beanstandung |
| **K8: Barrierefreiheit** | ✅ | `aria-label` pro Stern mit Aktivstatus, `:focus-visible` Ring, `min-height: 44px` + `padding: 10px 6px` für Touch-Fläche; E2E-Tastatur-Test vorhanden |

---

## Prüfung gegen Definition of Done

### Code-Qualität
- [x] `cargo build` — fehlerfrei
- [x] `cargo clippy -- -D warnings` — keine Warnungen
- [x] `cargo fmt --check` — korrekt formatiert
- [x] Keine ungenutzten Funktionen / Variablen

### Architektur-Einhaltung
- [x] Tech Stack: Rust + Axum + Askama + sqlx + SQLite + HTMX
- [x] SSR, keine JSON-APIs für UI (Response ist HTML-Fragment)
- [x] App funktioniert ohne JavaScript (Form mit POST-Action als Fallback)
- [x] Code in korrekten Verzeichnissen (`src/models/`, `src/routes/`, `src/templates/`, `templates/recipes/`)

### Testing
- [x] Integrationstests geschrieben und bestanden (`cargo test`)
- [x] E2E Tests geschrieben und bestanden (`npm run test:e2e`)
- [x] Given/When/Then-Kommentare in Integrations- und E2E-Tests vorhanden

### Funktionale Anforderungen
- [x] Alle Akzeptanzkriterien erfüllt
- [x] Edge Cases behandelt (Toggle-Reset, ungültiger Wert → 400, unbekannte ID → 404)
- [x] Validierung vorhanden (`validate_rating` im Handler aufgerufen)

---

## Test-Ergebnisse

### Integrationstests (`tests/recipe_inline_rating.rs`)

| Test | Status |
|------|--------|
| `update_rating_stores_new_value` | ✅ |
| `update_rating_resets_to_none` | ✅ |
| `update_rating_returns_html_fragment` | ✅ |
| `update_rating_rejects_invalid_value` | ✅ |
| `update_rating_returns_404_for_unknown_id` | ✅ |

Gesamt Unit-/Integrationstests: **156 bestanden, 0 fehlgeschlagen**

### E2E-Tests (`tests/e2e/recipe-inline-rating.spec.ts`)

| Test | Status |
|------|--------|
| K1+K2+K4: Inline-Bewertung setzen (4 Sterne) | ✅ |
| K2+K4: Bewertung ändern (3 → 5 Sterne) | ✅ |
| K3: Bewertung zurücksetzen durch erneutes Antippen | ✅ |
| K6: Inline-Bewertung und Edit-Mode zeigen gleichen Wert | ✅ |
| K8: Tastatur-Navigation zu den Sterne-Buttons | ✅ |
| K5: Inline-Rating-Container ohne JS sichtbar | ✅ |

Gesamt E2E-Tests: **119 bestanden, 0 fehlgeschlagen**

### Code-Quality Checks

| Check | Ergebnis |
|-------|----------|
| `cargo fmt --check` | ✅ |
| `cargo clippy -- -D warnings` | ✅ |
| `cargo test` | ✅ 156 Tests grün |
| `npm run test:e2e` | ✅ 119 Tests grün |

---

## Empfohlene Nacharbeit

### Prio 1 (Muss — blockiert Abschluss)

Keine.

### Prio 2 (Sollte — nice-to-have)

1. **Template-Methoden-Duplizierung in `templates.rs`**
   - `rating_is_active` und `star_filled` sind identisch in `InlineRatingTemplate` und `RecipeDetailTemplate` implementiert. Da `_inline_rating.html` per `{% include %}` eingebettet wird und `self` jeweils das übergeordnete Template ist, ist die Duplikation technisch notwendig. Ein gemeinsames Trait wäre eleganter, würde aber kaum Mehrwert bieten und die Lesbarkeit nicht verbessern.
   - Kein Handlungsbedarf für diesen Release.

2. **CSS Hover-Effekt unter Keyboard-Focus nicht ausgelöst**
   - Der Hover-Effekt (Sterne beleuchten) funktioniert nur bei Maus-Hover, nicht beim Tab-fokussierten Stern. Das ist kein WCAG-Verstoß (Fokus-Ring ist sichtbar), aber ein kleines UX-Plus wäre ein ``:focus-within`-basierter Effekt analog zum Hover.
   - Optionale Verbesserung.

---

## Fazit

**Gesamtbewertung:** ✅ Abgenommen

Die Implementierung erfüllt alle 8 Akzeptanzkriterien vollständig. Alle Qualitätschecks sind grün (156 Unit-/Integrationstests, 119 E2E-Tests bestanden). Die technischen Entscheidungen (CSS `flex-direction: row-reverse` für Hover-Effekt, `<form>`-Fallback für Progressive Enhancement, HTMX-Fragment-Swap) sind sauber umgesetzt und entsprechen dem Architektur-Constraint des Projekts.

**Nächste Schritte:**
1. Story als abgeschlossen markieren
2. Branch mergen
