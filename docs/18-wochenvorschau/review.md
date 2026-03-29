# Review: Story 18 - Wochenvorschau für geplante Rezepte

**Review-Datum:** 2026-03-29
**Story-Status:** Implementiert

---

## Zusammenfassung

Story 18 wurde vollständig implementiert: Eine dedizierte `/wochenvorschau`-Seite zeigt alle Rezepte der laufenden Kalenderwoche (Montag–Sonntag) strukturiert nach Wochentagen an. Die Implementierung umfasst DB-Layer, Handler, Template, Navigation, CSS-Styling sowie vollständige Unit-, Integrations- und E2E-Tests. Alle Qualitätschecks sind grün, kein Prio-1-Problem.

---

## Prüfung gegen den Plan

| Schritt | Status | Bemerkung |
|---------|--------|-----------|
| 1. DB-Layer — `get_recipes_current_week` | ✅ | In `recipe_db.rs`, inkl. 6 Unit-Tests |
| 2. Template-Structs in `src/templates.rs` | ✅ | `WochentagesEintragItem`, `Wochentag`, `WochenvorschauTemplate` mit Doc-Kommentaren |
| 3. Route und Handler `src/routes/wochenvorschau.rs` | ✅ | Handler + 4 Hilfsfunktionen + Unit-Tests |
| 4. Template `templates/wochenvorschau.html` | ✅ | Semantisches HTML mit `<dl>`, Leer-Meldung, Rezept-Links |
| 5. Navigation in `templates/base.html` | ✅ | `<nav class="main-nav">` mit Link zu `/wochenvorschau` |
| 6. CSS-Styling in `app.css` | ✅ | Alle geplanten CSS-Klassen vorhanden |
| 7. Rust-Integrationstests `tests/wochenvorschau.rs` | ✅ | 11 Tests, alle grün |
| 8. E2E-Tests `tests/e2e/wochenvorschau.spec.ts` | ✅ | 10 Tests zu allen Akzeptanzkriterien, alle grün |
| 9. Qualitätschecks | ✅ | `cargo fmt`, `cargo clippy`, `cargo test`, `npm run test:e2e` alle grün |

---

## Prüfung gegen Akzeptanzkriterien

| Kriterium | Status | Bemerkung |
|-----------|--------|-----------|
| **K1: Wochenvorschau-Seite erreichbar** | ✅ | URL `/wochenvorschau`, Link in `<nav>` in `base.html`, direkt bookmarkbar |
| **K2: Alle Wochentage werden angezeigt** | ✅ | Alle 7 Tage (Mo–So) mit Name und Datum, "Nichts geplant" für leere Tage |
| **K3: Geplante Rezepte werden den Wochentagen zugeordnet** | ✅ | Korrekte Zuordnung nach `planned_date`, Rezeptnamen sind klickbare Links zu `/recipes/{id}` |
| **K4: Mehrere Rezepte pro Tag** | ✅ | Alle Rezepte desselben Tages erscheinen untereinander als `<ul>` |
| **K5: Keine geplanten Rezepte** | ✅ | Freundliche Meldung "Für diese Woche noch nichts geplant" im `empty-state`-Block |
| **K6: Aktuelle Woche korrekt berechnet** | ✅ | Serverseitige Berechnung mit `time`-Crate, ISO-Woche (Mo–So) |
| **K7: Verlinkung zur Rezeptliste** | ✅ | Button "Zur Rezeptliste" (`href="/"`) am Ende der Seite |
| **K8: Performance** | ✅ | Einzelne SQLite-Abfrage, kein N+1, < 500ms in allen Tests |
| **K9: Barrierefreiheit** | ✅ | `<dl>` mit `<dt>`/`<dd>` für Wochentage, 7 `<dt class="wochentag-titel">` bestätigt per E2E-Test |

---

## Prüfung gegen Definition of Done

### Code-Qualität
- [x] `cargo build` — fehlerfrei
- [x] `cargo clippy -- -D warnings` — keine Warnungen
- [x] `cargo fmt --check` — korrekt formatiert
- [x] Keine ungenutzten Funktionen / Variablen

### Architektur-Einhaltung
- [x] Tech Stack: Rust + Axum + Askama + sqlx + SQLite (HTMX nicht nötig — statische Seite)
- [x] SSR, keine JSON-APIs für UI
- [x] App funktioniert ohne JavaScript (reine Server-Renders)
- [x] Code in korrekten Verzeichnissen (`src/routes/`, `src/models/`, `templates/`)
- [x] Module korrekt exportiert in `src/models/mod.rs` und `src/routes/mod.rs`

### Testing
- [x] Unit Tests geschrieben und bestanden (`cargo test` — 90 Tests grün, davon 11 für Wochenvorschau)
- [x] E2E Tests geschrieben und bestanden (`npm run test:e2e` — 10 Tests grün)
- [x] Jeder Test enthält Given/When/Then als Kommentare

### Funktionale Anforderungen
- [x] Alle 9 Akzeptanzkriterien erfüllt
- [x] Edge Cases behandelt (kein Rezept, mehrere am selben Tag, Rezept ohne Datum, letzte/nächste Woche)
- [x] Fehlerbehandlung: Template-Render-Fehler werden als `AppError` zurückgegeben

---

## Test-Ergebnisse

### Unit-Tests (cargo test)
| Test | Status |
|------|--------|
| `routes::wochenvorschau::tests::format_day_display_formats_correctly` | ✅ |
| `routes::wochenvorschau::tests::format_day_display_sunday` | ✅ |
| `routes::wochenvorschau::tests::format_kw_header_same_month` | ✅ |
| `routes::wochenvorschau::tests::format_kw_header_different_months` | ✅ |
| `routes::wochenvorschau::tests::german_weekday_long_returns_correct_names` | ✅ |
| `routes::wochenvorschau::tests::iso_week_number_kw14_2026` | ✅ |
| `models::recipe_db::tests::get_recipes_current_week_returns_recipe_in_week` | ✅ |
| `models::recipe_db::tests::get_recipes_current_week_excludes_recipe_before_monday` | ✅ |
| `models::recipe_db::tests::get_recipes_current_week_excludes_recipe_after_sunday` | ✅ |
| `models::recipe_db::tests::get_recipes_current_week_excludes_recipe_without_date` | ✅ |
| `models::recipe_db::tests::get_recipes_current_week_returns_multiple_recipes_same_day` | ✅ |
| `models::recipe_db::tests::get_recipes_current_week_sorts_by_date_then_title` | ✅ |

### Integrationstests (tests/wochenvorschau.rs)
| Test | Status |
|------|--------|
| `wochenvorschau_returns_200` | ✅ |
| `wochenvorschau_shows_all_seven_weekdays` | ✅ |
| `wochenvorschau_shows_recipe_in_current_week` | ✅ |
| `wochenvorschau_does_not_show_recipe_from_next_week` | ✅ |
| `wochenvorschau_does_not_show_recipe_from_last_week` | ✅ |
| `wochenvorschau_shows_empty_state_when_no_recipes` | ✅ |
| `wochenvorschau_shows_multiple_recipes_on_same_day` | ✅ |
| `wochenvorschau_recipe_link_leads_to_detail` | ✅ |
| `wochenvorschau_shows_kw_header` | ✅ |
| `wochenvorschau_does_not_show_recipe_without_date` | ✅ |
| `wochenvorschau_shows_empty_state_when_recipe_is_next_week` | ✅ |

### E2E-Tests (Playwright)
| Test | Status |
|------|--------|
| K1: /wochenvorschau ist aufrufbar und in Navigation verlinkt | ✅ |
| K2: Alle 7 Wochentage erscheinen auf der Seite | ✅ |
| K3: Rezept mit planned_date wird auf der Seite angezeigt | ✅ |
| K4: Mehrere Rezepte am gleichen Tag erscheinen beide | ✅ |
| K5: Seite lädt auch ohne Rezepte (Hinweis "Nichts geplant") | ✅ |
| K3/K6: Rezeptlink führt zur Detailansicht | ✅ |
| K5/Deeplink: /wochenvorschau direkt per URL aufrufbar | ✅ |
| K7: Link zurück zur Rezeptliste vorhanden | ✅ |
| K9: Semantisches HTML — Wochentage als dt-Elemente | ✅ |
| KW-Anzeige: Kalenderwochen-Angabe sichtbar | ✅ |

### Code-Quality Checks
| Check | Ergebnis |
|-------|----------|
| cargo fmt --check | ✅ |
| cargo clippy -- -D warnings | ✅ |
| cargo test (90 Tests) | ✅ |
| npm run test:e2e (Wochenvorschau: 10 Tests) | ✅ |

---

## Empfohlene Nacharbeit

### Prio 1 (Muss — blockiert Abschluss)

Keine Prio-1-Probleme gefunden.

### Prio 2 (Sollte — nice-to-have)

1. **E2E-Test für K5 (leere Woche) nicht isoliert**
   - Test `K5` prüft nur, ob "Nichts geplant" auf der Seite erscheint (was immer der Fall ist, wenn mindestens ein Tag leer ist). Die globale Leer-Meldung "Für diese Woche noch nichts geplant" wird nicht per E2E getestet, nur per Integrationstest. Der Test-Kommentar erklärt dies explizit und ist korrekt — der Rust-Integrationstest deckt diesen Fall ab.
   - Kein Handlungsbedarf, aber ein optionaler E2E-Test mit Seed-Daten ohne Wochenrezepte wäre vollständiger.

2. **ISO-Wochennummer im gleichen Monat: KW-Format leicht von Spezifikation abweichend**
   - Laut Spezifikation: "KW 14 · 30. März – 5. April 2026" (immer beide Monatsnamen im Beispiel).
   - Implementierung bei gleichem Monat: "KW 15 · 6. – 12. April 2026" (Monatsname nur einmal).
   - Dies ist ein sinnvolles Kompaktformat und wurde im Plan explizit vorgesehen. Kein Problem.

3. **`GERMAN_MONTHS_LONG` dupliziert**
   - Die Konstante existiert sowohl in `src/routes/recipes.rs` als auch in `src/routes/wochenvorschau.rs`. Beide Stellen sind identisch. Für zukünftige Wartbarkeit könnte sie in ein gemeinsames Modul (z.B. `src/utils.rs`) extrahiert werden.
   - Kein funktionales Problem, reine Code-Hygiene.

---

## Fazit

**Gesamtbewertung:** ✅ Abgenommen

Die Implementierung ist vollständig und korrekt. Alle 9 Akzeptanzkriterien sind erfüllt, alle Tests (90 Unit/Integrationstests + 10 E2E-Tests) sind grün, kein Clippy-Warning, Code ist formatiert. Die Architektur-Constraints (SSR, kein JS-Zwang, DeepLink-fähig, semantisches HTML) werden eingehalten. Edge Cases (leere Woche, mehrere Rezepte pro Tag, Rezepte ohne Datum, Wochengrenzen) sind getestet.

**Nächste Schritte:**
1. Story 18 als abgeschlossen markieren
2. Optional (Prio 2): `GERMAN_MONTHS_LONG` in gemeinsames Hilfmodul extrahieren
