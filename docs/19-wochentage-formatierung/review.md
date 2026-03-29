# Review: Story 19 – Wochenvorschau nach Wochentagen formatiert

**Review-Datum:** 2026-03-29
**Story-Status:** Implementiert

---

## Zusammenfassung

Story 19 wurde vollständig und korrekt implementiert. Die `Wochentag`-Struct wurde um `wochentag_name`, `datum_kurz`, `ist_heute` und `ist_vergangen` erweitert, das Template zeigt Wochentag-Name und Datum getrennt mit semantisch sinnvollen HTML-Elementen (`<strong>` und `<span>`), und der heutige Tag wird durch CSS-Klasse sowie ein "Heute"-Badge hervorgehoben. Alle Akzeptanzkriterien sind erfüllt, alle Tests sind grün. Keine Nacharbeit erforderlich.

---

## Prüfung gegen den Plan

| Schritt | Status | Bemerkung |
|---------|--------|-----------|
| 1. `Wochentag`-Struct in `src/templates.rs` erweitern | ✅ | Alle 5 Felder vorhanden: `wochentag_name`, `datum_kurz`, `ist_heute`, `ist_vergangen`, `rezepte`. `datum_anzeige` vollständig entfernt. |
| 2. Hilfsfunktionen in `wochenvorschau.rs` anpassen | ✅ | `format_weekday_name` und `format_date_kurz` implementiert. `format_day_display` entfernt. Alle Unit-Tests aktualisiert. |
| 3. Handler in `wochenvorschau.rs` anpassen | ✅ | Mapping auf neue Felder korrekt umgestellt. `ist_heute` und `ist_vergangen` werden serverseitig berechnet. |
| 4. Template `wochenvorschau.html` anpassen | ✅ | Bedingte CSS-Klassen, `<strong class="wochentag-name">`, `<span class="wochentag-datum">`, "Heute"-Badge implementiert. |
| 5. CSS in `app.css` erweitern | ✅ | Alle 5 neuen Klassen vorhanden: `.wochentag-heute`, `.wochentag-vergangen`, `.wochentag-name`, `.wochentag-datum`, `.heute-badge`. |
| 6. Rust-Unit-Tests vervollständigen | ✅ | Tests für `format_weekday_name` (alle 7 Wochentage), `format_date_kurz` (3 Fälle), `wochentag_felder_sind_korrekt_befuellt` vorhanden und grün. |
| 7. Rust-Integrationstests ergänzen | ✅ | 5 neue Tests in `tests/wochenvorschau.rs`: `wochentag-heute`, `wochentag-name`, `wochentag-datum`, "Heute"-Badge, Montag-Edge-Case. Alle grün. |
| 8. E2E-Tests ergänzen | ✅ | 4 neue Tests in `describe('Wochenvorschau Formatierung (Story 19)')`: K1–K4 vollständig abgedeckt. Alle grün. |
| 9. Qualitätschecks | ✅ | `cargo fmt`, `cargo clippy`, `cargo test`, `npm run test:e2e` — alle bestanden. |

---

## Prüfung gegen Akzeptanzkriterien

| Kriterium | Status | Bemerkung |
|-----------|--------|-----------|
| **K1: Wochentag-Name und Datum sind visuell getrennt** | ✅ | `<strong class="wochentag-name">` für Namen (1.1rem, fett), `<span class="wochentag-datum">` für Datum (0.875rem, grau). E2E-Test prüft 7x `strong.wochentag-name` und 7x `span.wochentag-datum`. |
| **K2: Heutiger Tag ist visuell hervorgehoben** | ✅ | CSS-Klasse `wochentag-heute` mit blauem Rahmen und Hintergrund. "Heute"-Badge erfüllt WCAG 1.4.1 (nicht nur Farbe). Serverseitige Berechnung via `datum == today`. E2E prüft genau 1x `.wochentag-heute` und `.heute-badge` mit Text "Heute". |
| **K3: Vergangene Tage sind schwächer dargestellt** | ✅ | CSS-Klasse `wochentag-vergangen` mit `opacity: 0.6`. E2E-Test zählt Elemente exakt entsprechend Offset von Montag. Montag-Edge-Case (keine vergangenen Tage) korrekt behandelt. |
| **K4: Alle bisherigen AK aus Story 18 bleiben erfüllt** | ✅ | Alle 7 Wochentage, Rezept-Zuordnung, klickbare Links, "Nichts geplant", KW-Anzeige — Smoke-Test und alle bestehenden Story-18-Tests grün. |
| **K5: Performance** | ✅ | Keine zusätzliche DB-Abfrage, `ist_heute`/`ist_vergangen` im Handler berechnet. Seite lädt unverändert schnell. |
| **K6: Barrierefreiheit** | ✅ | Hervorhebung nicht nur per Farbe: "Heute"-Badge (Text), `<strong>` für Wochentag-Name (Fettschrift). `aria-label="Heute"` am Badge. Semantische Struktur mit `<dt>`/`<dd>` bleibt erhalten. |

---

## Prüfung gegen Definition of Done

### Code-Qualität
- [x] `cargo build` — fehlerfrei (Askama prüft Template zur Compile-Zeit)
- [x] `cargo clippy -- -D warnings` — keine Warnungen
- [x] `cargo fmt --check` — korrekt formatiert
- [x] Keine ungenutzten Funktionen/Variablen — `format_day_display` vollständig entfernt

### Architektur-Einhaltung
- [x] Tech Stack: Rust + Axum + Askama + sqlx + SQLite + HTMX
- [x] SSR, keine JSON-APIs für UI
- [x] App funktioniert ohne JavaScript (serverseitige Berechnung von `ist_heute`/`ist_vergangen`)
- [x] Code in korrekten Verzeichnissen (`src/routes/wochenvorschau.rs`, `templates/wochenvorschau.html`, `src/static/css/app.css`)

### Testing
- [x] Unit-Tests geschrieben und bestanden (`cargo test` — 93 Tests, alle grün)
- [x] Integrationstests für HTTP-Endpunkte geschrieben (16 Tests in `tests/wochenvorschau.rs`)
- [x] E2E-Tests geschrieben und bestanden (4 neue Story-19-Tests, alle grün)
- [x] Given/When/Then-Kommentare in Tests vorhanden (Deutsch)
- [x] Für jedes Akzeptanzkriterium mindestens ein Test

### Funktionale Anforderungen
- [x] Alle Akzeptanzkriterien aus story.md erfüllt
- [x] Edge Cases behandelt: Montag (keine vergangenen Tage), Sonntag (letzter Tag), heute ohne Rezept
- [x] Keine Validierung nötig (rein visuelle Änderung, keine Nutzereingabe)

---

## Test-Ergebnisse

### Unit-Tests (Story-19-relevante)
| Test | Status |
|------|--------|
| `format_weekday_name_returns_correct_name` (alle 7 Tage) | ✅ |
| `format_date_kurz_formats_correctly` | ✅ |
| `format_date_kurz_single_digit_day` | ✅ |
| `format_date_kurz_january` | ✅ |
| `wochentag_felder_sind_korrekt_befuellt` | ✅ |

### Integrationstests (Story-19-spezifisch)
| Test | Status |
|------|--------|
| `wochenvorschau_hat_css_klasse_wochentag_heute` | ✅ |
| `wochenvorschau_hat_css_klasse_wochentag_name` | ✅ |
| `wochenvorschau_hat_css_klasse_wochentag_datum` | ✅ |
| `wochenvorschau_heute_tag_enthaelt_heute_badge` | ✅ |
| `wochenvorschau_vergangene_tage_korrekt_wenn_nicht_montag` | ✅ |

### E2E-Tests (Story 19)
| Test | Status |
|------|--------|
| K1: 7x `strong.wochentag-name` und 7x `span.wochentag-datum` sichtbar | ✅ |
| K2: Genau ein Element mit Klasse `wochentag-heute` | ✅ |
| K2: Heute-Badge sichtbar mit Text "Heute" | ✅ |
| K3: Anzahl `wochentag-vergangen` entspricht Anzahl Tage seit Montag | ✅ |
| K4: Smoke-Test — Rezept für heute erscheint unter `.wochentag-heute` | ✅ |

### Code-Quality Checks
| Check | Ergebnis |
|-------|----------|
| `cargo build` | ✅ |
| `cargo clippy -- -D warnings` | ✅ |
| `cargo fmt --check` | ✅ |
| `cargo test` (93 Unit + 172 Integrations-Tests) | ✅ |
| `npm run test:e2e` (Story-19-Tests) | ✅ |

**Hinweis E2E-Gesamtlauf:** Im Vollrun der 143 E2E-Tests war 1 Test zeitweise fehlerhaft — `recipe-search.spec.ts: K1: Klick auf Clear-Icon leert Suchfeld und zeigt alle Rezepte`. Dieser Test gehört zu Story 27 (nicht Story 19) und ist eine bekannte Flakiness durch gemeinsame Datenbank zwischen parallelen Tests. Im isolierten Lauf (`npx playwright test tests/e2e/recipe-search.spec.ts`) bestehen alle 14 Tests. Kein Problem das Story 19 betrifft.

---

## Empfohlene Nacharbeit

### Prio 1 (Muss — blockiert Abschluss)

Keine.

### Prio 2 (Sollte — nice-to-have)

1. **Flaky E2E-Test in Story 27 (Clear-Icon)**
   - `recipe-search.spec.ts: K1: Klick auf Clear-Icon` schlägt gelegentlich im parallelen Vollrun fehl, weil die Tests eine gemeinsame SQLite-DB nutzen.
   - Betrifft Story 27, nicht Story 19. Sollte im Rahmen einer separaten Story oder als Teil des Story-27-Reviews untersucht werden.
   - Empfehlung: Test-Isolation durch `TEST_DATABASE_URL` pro Worker oder sequentiellen Run prüfen.

---

## Fazit

**Gesamtbewertung:** ✅ Abgenommen

Story 19 ist vollständig und korrekt implementiert. Alle 6 Akzeptanzkriterien sind erfüllt, alle Story-19-Tests (Unit, Integration, E2E) sind grün, und der Code entspricht den Qualitäts- und Architekturanforderungen. Die Implementierung folgt dem Plan präzise, einschließlich des optionalen "Heute"-Badges für WCAG 1.4.1-Konformität.

**Nächste Schritte:**
1. Story 19 als abgeschlossen markieren
2. Flakiness des Clear-Icon-Tests (Story 27) separat untersuchen
