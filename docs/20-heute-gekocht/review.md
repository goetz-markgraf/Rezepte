# Review: Story 20 — "Heute gekocht" Ansicht mit Highlight

**Review-Datum:** 2026-03-29
**Story-Status:** Implementiert

---

## Zusammenfassung

Story 20 ist vollständig implementiert. Die `/heute`-Seite zeigt die Rezepte von gestern, heute und morgen in drei klar getrennten Abschnitten, das heutige Gericht ist visuell hervorgehoben (blauer Rahmen + Heute-Badge), und die Inline-Bewertung funktioniert per HTMX ohne Seitenreload. Alle Unit-Tests, Integrationstests und E2E-Tests sind grün. Es gibt keine blockierenden Probleme; eine kleinere technische Schuld (Duplikation von Datums-Hilfsfunktionen) ist vorhanden, wurde im Plan jedoch explizit akzeptiert.

---

## Prüfung gegen den Plan

| Schritt | Status | Bemerkung |
|---------|--------|-----------|
| 1. DB-Layer `get_recipes_drei_tage` | ✅ | Implementiert in `src/models/recipe_db.rs`, inkl. 6 Unit-Tests |
| 2. Template-Datenstrukturen | ✅ | `HeuteRezeptItem`, `HeuteTagesabschnitt`, `HeuteTemplate`, `InlineRatingHeuteTemplate` in `src/templates.rs` |
| 3. Route-Handler `src/routes/heute.rs` | ✅ | `heute_handler` und `heute_rating_handler` implementiert, Unit-Tests für Datums-Hilfsfunktionen vorhanden |
| 3a. `date_utils.rs` Auslagerung | ⚠️ | Plan erwähnte Auslagerung als bevorzugte Lösung; stattdessen wurden Hilfsfunktionen in `heute.rs` dupliziert (explizit im Plan als Fallback akzeptiert) |
| 4. Routing | ✅ | `GET /heute` und `POST /heute/recipes/:id/rating` korrekt in `src/routes/mod.rs` eingetragen |
| 5. Template `templates/heute.html` | ✅ | Vollständig mit HTMX-Inline-Rating, eindeutigen IDs pro Rezept |
| 5a. Template `_inline_rating_heute.html` | ✅ | Separates Fragment mit dynamischer `id="inline-rating-{{ id }}"` |
| 6. Navigation anpassen | ✅ | Nav-Link "Heute" in `templates/base.html` ergänzt |
| 7. CSS-Styling | ✅ | Alle geplanten CSS-Klassen in `src/static/css/app.css` vorhanden, Mobile-First |
| 8. Rust-Integrationstests | ✅ | 13 Tests in `tests/heute.rs`, alle grün |
| 9. E2E-Tests | ✅ | 9 Tests in `tests/e2e/heute.spec.ts`, alle grün |
| 10. Qualitätschecks | ✅ | `cargo fmt`, `cargo clippy`, `cargo test`, `npm run test:e2e` alle bestanden |

---

## Prüfung gegen Akzeptanzkriterien

| Kriterium | Status | Bemerkung |
|-----------|--------|-----------|
| **K1: "Heute gekocht"-Seite erreichbar** | ✅ | URL `/heute` verfügbar, Nav-Link in Hauptnavigation, DeepLink-fähig (E2E-Tests bestätigt) |
| **K2: Heutiges Gericht ist hervorgehoben** | ✅ | CSS-Klasse `tagesabschnitt-heute` (blauer Rahmen + Hintergrund), zusätzliches "Heute"-Badge (kein reines Farb-Highlight), Rezeptname als klickbarer Link |
| **K3: Gestern und morgen werden angezeigt** | ✅ | Drei Abschnitte mit Wochentag und Datum (z.B. "Montag, 30. März"), E2E-Test K3 bestätigt |
| **K4: Keine geplanten Rezepte — freundliche Meldung** | ✅ | Meldung "Für heute noch kein Rezept geplant" erscheint, Integrationstest und E2E-Test bestätigt |
| **K5: Inline-Bewertung direkt auf der Seite** | ✅ | Sterne-Buttons für alle Rezepte (gestern, heute, morgen), HTMX-POST ohne Seitenreload, aktuelle Bewertung wird angezeigt, E2E-Test K5 bestätigt |
| **K6: Link zur Detailansicht und zum Bearbeiten** | ✅ | Rezepttitel ist Link zur Detailansicht (`/recipes/:id`), von dort Bearbeiten erreichbar |
| **K7: Datum wird serverseitig berechnet** | ✅ | `time::OffsetDateTime::now_utc().date()` im Handler, kein Client-seitiges Datum |
| **K8: Performance** | ✅ | Einzelne SQL-Query mit Datumsbereich, kein N+1, SQLite-Abfrage auf ≤200 Rezepten unkritisch |
| **K9: Barrierefreiheit** | ✅ | `aria-label` an Stern-Buttons ("5 Sterne – inaktiv" / "aktiv"), `aria-label` am inline-rating-div ("N von 5 Sternen"), Hervorhebung nicht nur über Farbe (Badge), semantische `<section>`- und `<header>`-Tags |

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
- [x] App funktioniert ohne JavaScript (Form-POST an `/heute/recipes/:id/rating`)
- [x] Code in korrekten Verzeichnissen (`src/routes/heute.rs`, `templates/heute.html`, `templates/recipes/_inline_rating_heute.html`)

### Testing
- [x] Unit Tests geschrieben und bestanden (`cargo test`) — 5 Unit-Tests in `heute.rs`, 6 Unit-Tests in `recipe_db.rs`
- [x] Integrationstests geschrieben und bestanden (`cargo test`) — 13 Tests in `tests/heute.rs`
- [x] E2E Tests geschrieben und bestanden (`npm run test:e2e`) — 9 Tests in `tests/e2e/heute.spec.ts`

### Funktionale Anforderungen
- [x] Alle Akzeptanzkriterien erfüllt
- [x] Edge Cases behandelt (kein Rezept heute, mehrere Rezepte heute, Rezept ohne Datum)
- [x] Validierung für Rating vorhanden (400 bei ungültigem Wert, 404 bei unbekannter ID)

---

## Test-Ergebnisse

### Unit-Tests (cargo test)
| Test | Status |
|------|--------|
| `format_heute_anzeige_formats_correctly` | ✅ |
| `format_heute_anzeige_sunday` | ✅ |
| `format_heute_anzeige_january` | ✅ |
| `format_date_kurz_formats_correctly` | ✅ |
| `format_weekday_name_returns_correct_name` | ✅ |
| `get_recipes_drei_tage_returns_recipe_for_today` | ✅ |
| `get_recipes_drei_tage_returns_recipe_for_gestern` | ✅ |
| `get_recipes_drei_tage_returns_recipe_for_morgen` | ✅ |
| `get_recipes_drei_tage_excludes_recipe_from_two_days_ago` | ✅ |
| `get_recipes_drei_tage_excludes_recipe_without_date` | ✅ |
| `get_recipes_drei_tage_returns_multiple_recipes_for_today` | ✅ |
| **Gesamt: 104 Unit-Tests, 0 Fehler** | ✅ |

### Integrationstests (cargo test)
| Test | Status |
|------|--------|
| `heute_returns_200` | ✅ |
| `heute_zeigt_rezept_fuer_heute` | ✅ |
| `heute_zeigt_rezept_fuer_gestern` | ✅ |
| `heute_zeigt_rezept_fuer_morgen` | ✅ |
| `heute_zeigt_keine_rezepte_von_vorgestern` | ✅ |
| `heute_zeigt_freundliche_meldung_wenn_kein_rezept_fuer_heute` | ✅ |
| `heute_zeigt_mehrere_rezepte_fuer_heute` | ✅ |
| `heute_hat_link_zur_detailansicht` | ✅ |
| `heute_hat_css_klasse_tagesabschnitt_heute` | ✅ |
| `heute_rating_post_gibt_200_zurueck` | ✅ |
| `heute_rating_post_speichert_bewertung` | ✅ |
| `heute_rating_post_gibt_404_fuer_unbekannte_id` | ✅ |
| `heute_rating_post_gibt_400_fuer_ungueltige_bewertung` | ✅ |
| **Gesamt: 13/13 Tests, 0 Fehler** | ✅ |

### E2E-Tests (npm run test:e2e)
| Test | Status |
|------|--------|
| K1: `/heute` ist aufrufbar und in Navigation verlinkt | ✅ |
| K1: `/heute` ist per DeepLink direkt aufrufbar | ✅ |
| K2: Heutiges Rezept ist im hervorgehobenen Heute-Abschnitt | ✅ |
| K3: Gestern und morgen werden angezeigt | ✅ |
| K4: Freundliche Meldung wenn kein Rezept für heute | ✅ |
| K5: Inline-Bewertung direkt auf der Seite (speichern und anzeigen) | ✅ |
| K6: Rezepttitel ist Link zur Detailansicht | ✅ |
| Mehrere Rezepte für heute werden alle angezeigt | ✅ |
| **Gesamt: 9/9 Tests, 0 Fehler (151 gesamt, alle grün)** | ✅ |

### Code-Quality Checks
| Check | Ergebnis |
|-------|----------|
| cargo build | ✅ |
| cargo clippy -- -D warnings | ✅ |
| cargo fmt --check | ✅ |

---

## Empfohlene Nacharbeit

### Prio 1 (Muss — blockiert Abschluss)

Keine blockierenden Probleme gefunden.

### Prio 2 (Sollte — nice-to-have)

1. **Duplikation der Datums-Hilfsfunktionen**
   - `GERMAN_MONTHS_LONG`, `GERMAN_WEEKDAYS_LONG`, `format_date_kurz()` und `format_weekday_name()` sind identisch in `src/routes/heute.rs` und `src/routes/wochenvorschau.rs` vorhanden.
   - Der Implementierungsplan hat dies explizit als akzeptierten Fallback erwähnt (bevorzugt wäre `src/routes/date_utils.rs`).
   - Empfehlung: Bei der nächsten Story, die diese Funktionen ebenfalls braucht, als Refactoring in `date_utils.rs` auslagern.

2. **Sternebewertung beginnt bei 5 (nicht bei 1)**
   - Im Template werden die Sterne in absteigender Reihenfolge (5, 4, 3, 2, 1) gerendert. Dies entspricht der bestehenden Implementierung in `_inline_rating.html` (Story 17) und ist konsistent, könnte aber für neue Entwickler unintuitiv sein.
   - Kein Handlungsbedarf, nur zur Dokumentation.

3. **Keine Testdaten-Isolation im E2E-Test K4**
   - Der K4-Test ("Freundliche Meldung wenn kein Rezept für heute") prüft nur dann die Meldung, wenn kein anderes Rezept für heute vorhanden ist. Da die Tests eine geteilte DB nutzen, ist dieser Test bedingt korrekt (der Code-Kommentar erklärt dies).
   - Könnte durch eine isolierte Seed-DB robuster gestaltet werden, ist aber kein Blockierer.

---

## Fazit

**Gesamtbewertung:** ✅ Abgenommen

Die Implementierung erfüllt alle 9 Akzeptanzkriterien vollständig. Alle 104 Unit-Tests, 13 Integrationstests und 151 E2E-Tests (davon 9 für Story 20) sind grün. Der Code ist sauber formatiert, clippy-clean und folgt den Architektur-Constraints (SSR, HTMX, kein CDN, serverseitiges Datum). Die technische Schuld (Duplikation der Datums-Hilfsfunktionen) ist dokumentiert und wurde bewusst als Fallback akzeptiert.

**Nächste Schritte:**
1. Story 20 als abgeschlossen markieren
2. Optional: Refactoring der Datums-Hilfsfunktionen nach `src/routes/date_utils.rs` als eigenständige technische Story einplanen
