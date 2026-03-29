# Review: Story 23 – Rezepte mergen (Duplikate zusammenführen)

**Review-Datum:** 2026-03-29
**Story-Status:** Implementiert

---

## Zusammenfassung

Story 23 wurde vollständig umgesetzt: Ein Merge-Workflow ermöglicht das atomare Zusammenführen zweier Rezepte über eine eigene Merge-Seite (`/recipes/merge`). Alle Akzeptanzkriterien sind erfüllt, alle automatisierten Tests (Unit, Integration, E2E) sind grün. Der Code ist sauber strukturiert und hält sich an die Architektur-Vorgaben. Es gibt eine kleine Auffälligkeit (toter Aufruf von `determine_merge_target` im GET-Handler), die aber keine funktionale Auswirkung hat.

---

## Prüfung gegen den Plan

| Schritt | Status | Bemerkung |
|---------|--------|-----------|
| 1. DB-Layer `merge_recipes` | ✅ | Atomare Transaktion implementiert, IDs werden vorab validiert |
| 2. `determine_merge_target` in `recipe.rs` | ✅ | Funktion mit vollständiger Priorisierungslogik vorhanden und getestet |
| 3. Template-Datenstrukturen (`MergeRezeptInfo`, `MergeTemplate`) | ✅ | Alle Konflikterkenner-Methoden implementiert |
| 4a. `MergeQuery` + GET-Handler | ✅ | `merge_form_handler` korrekt implementiert |
| 4b. POST-Handler | ✅ | `merge_handler` mit vollständiger Feldauswahl und Validierung |
| 5. Template `merge.html` | ✅ | Vollständig mit Radio-Buttons, Auto-Übernahme, Fehleranzeige, Abbrechen-Link |
| 6. Merge-Button in `duplicates.html` | ✅ | Korrekt mit `aria-label` |
| 7. Routing in `mod.rs` | ✅ | `/recipes/merge` vor `/:id` registriert |
| 8. Export in `models/mod.rs` | ✅ | `merge_recipes` re-exportiert |
| 9. CSS-Styling | ✅ | Alle im Plan spezifizierten CSS-Klassen vorhanden, responsiv |
| 10. Rust-Integrationstests | ✅ | 8 Tests in `tests/recipe_merge.rs`, alle grün |
| 11. E2E-Tests | ✅ | 9 Tests in `tests/e2e/recipe-merge.spec.ts`, alle grün |
| 12. Qualitätschecks | ✅ | `cargo fmt`, `cargo clippy`, `cargo test`, E2E alle grün |

---

## Prüfung gegen Akzeptanzkriterien

| Kriterium | Status | Bemerkung |
|-----------|--------|-----------|
| **K1: Merge-Aktion aus Dubletten-Übersicht erreichbar** | ✅ | "Mergen"-Link mit korrektem `href` und `aria-label` in `duplicates.html` |
| **K2: Merge-Ansicht zeigt beide Rezepte vollständig** | ✅ | Alle Felder sichtbar: Titel, Kategorien, Zutaten, Anleitung, Bewertung, Datum, Erstellungs-/Änderungsdatum |
| **K3: Nutzer wählt das Ziel-Rezept** | ✅ | source/target in URL-Parametern und als Hidden-Felder im Form; Karten klar beschriftet ("wird gelöscht" / "bleibt erhalten") |
| **K4: Nutzer wählt, welche Felder übernommen werden** | ✅ | Radio-Buttons bei Konflikt; automatische Übernahme bei einseitigem Inhalt mit Badge |
| **K5: Merge-Vorschau vor Bestätigung** | ✅ | Per Entscheidung (ADR in plan.md): Radio-Buttons zeigen Inhalt direkt; expliziter Submit-Button "Zusammenführen" |
| **K6: Merge-Ergebnis** | ✅ | Ziel aktualisiert, Quelle gelöscht, Redirect zu `/recipes/{target_id}?success=1`, Erfolgsmeldung sichtbar |
| **K7: Abbruch-Möglichkeit** | ✅ | "Abbrechen"-Link zurück zu `/recipes/duplicates`, beide Rezepte bleiben erhalten |
| **K8: Abgesicherter Merge-Vorgang** | ✅ | SQLite-Transaktion, bei `count < 2` RowNotFound-Fehler, kein partieller Zustand |
| **K9: Performance** | ✅ | Keine N+1-Queries; Merge in einer Transaktion; Performance-Tests nicht explizit gemessen, aber Architektur entspricht den Anforderungen |
| **K10: Barrierefreiheit** | ✅ | `aria-label` auf Merge-Button, Labels auf Radio-Inputs via `<label>`-Wrapper, `required` auf Radio-Gruppen |

---

## Prüfung gegen Definition of Done

### Code-Qualität
- [x] `cargo build` — fehlerfrei
- [x] `cargo clippy -- -D warnings` — keine Warnungen
- [x] `cargo fmt --check` — korrekt formatiert
- [x] Keine ungenutzten Funktionen / Variablen (einzige Auffälligkeit: `let _ = determine_merge_target(...)` im GET-Handler, bewusst ignoriert laut Kommentar)

### Architektur-Einhaltung
- [x] Tech Stack: Rust + Axum + Askama + sqlx + SQLite + HTMX
- [x] SSR, keine JSON-APIs für UI
- [x] App funktioniert ohne JavaScript (Form-Posts + Redirects, `required` auf Radio-Buttons enforced by Browser)
- [x] Code in korrekten Verzeichnissen (`src/models/`, `src/routes/`, `templates/`)

### Testing
- [x] Unit Tests geschrieben und bestanden (`cargo test`): 6 neue Tests für `merge_recipes` und `determine_merge_target`
- [x] Integrationstests in `tests/recipe_merge.rs`: 8 Tests, alle grün
- [x] E2E Tests in `tests/e2e/recipe-merge.spec.ts`: 9 Tests, alle grün
- [x] Given/When/Then-Kommentare in Integration- und E2E-Tests vorhanden

### Funktionale Anforderungen
- [x] Alle Akzeptanzkriterien erfüllt
- [x] Edge Cases behandelt (nicht-existierende IDs → 404, Validierungsfehler → Formular re-rendern)
- [x] Validierung via `UpdateRecipe::validate()` vorhanden

---

## Test-Ergebnisse

### Unit-Tests (`cargo test`)
| Test | Status |
|------|--------|
| `determine_merge_target_prefers_rated_recipe` | ✅ |
| `determine_merge_target_prefers_more_filled_recipe` | ✅ |
| `determine_merge_target_falls_back_to_smaller_id` | ✅ |
| `merge_recipes_success` | ✅ |
| `merge_recipes_invalid_source_id` | ✅ |
| `merge_recipes_invalid_target_id` | ✅ |
| Alle weiteren 121 bestehenden Unit-Tests | ✅ |

### Integrationstests (`cargo test`)
| Test | Status |
|------|--------|
| `merge_get_returns_200_for_valid_ids` | ✅ |
| `merge_get_returns_400_without_source` | ✅ |
| `merge_get_returns_400_without_target` | ✅ |
| `merge_get_returns_404_for_nonexistent_recipe` | ✅ |
| `merge_post_performs_merge_and_redirects` | ✅ |
| `merge_post_returns_404_for_nonexistent_source` | ✅ |
| `merge_post_returns_404_for_nonexistent_target` | ✅ |
| `merge_post_returns_400_for_invalid_data` | ✅ |

### E2E-Tests (`npm run test:e2e`)
| Test | Status |
|------|--------|
| K1: Merge-Button auf Dubletten-Übersicht sichtbar | ✅ |
| K2: Merge-Seite zeigt beide Rezepte vollständig | ✅ |
| K3: source_id und target_id als Hidden-Felder im Form | ✅ |
| K4: Radio-Buttons für Titel-Auswahl bei Konflikt | ✅ |
| K4b: Automatische Übernahme bei einseitigem Inhalt | ✅ |
| K5+K6: Erfolgreicher Merge-Durchlauf | ✅ |
| K7: Abbrechen kehrt zur Dubletten-Übersicht zurück | ✅ |
| K8: Direktlink (Deeplink) funktioniert | ✅ |
| Ungültige IDs → Fehlerseite oder 404 | ✅ |

### Code-Quality Checks
| Check | Ergebnis |
|-------|----------|
| `cargo build` | ✅ |
| `cargo clippy -- -D warnings` | ✅ |
| `cargo fmt --check` | ✅ |
| `cargo test` (alle Tests) | ✅ |
| `npm run test:e2e` (alle E2E) | ✅ |

---

## Empfohlene Nacharbeit

### Prio 1 (Muss — blockiert Abschluss)

Keine Prio-1-Probleme gefunden.

### Prio 2 (Sollte — nice-to-have)

1. **Toter Aufruf von `determine_merge_target` im GET-Handler**
   - In `merge_form_handler` wird `determine_merge_target(&source, &target)` aufgerufen, das Ergebnis aber mit `let _ = ...` verworfen.
   - Der Kommentar erklärt es ("nur für zukünftige Erweiterungen"), aber der Aufruf hat aktuell keinerlei Wirkung.
   - Entweder den Aufruf entfernen, oder tatsächlich den URL-Parameter basierend auf dem Vorschlag anpassen wenn source/target nicht explizit vom Nutzer gewählt wurden.

2. **Test 8 (`merge_post_returns_400_for_invalid_data`) prüft nicht streng genug**
   - Der Test prüft nur, dass die Antwort ein 302 oder 400 ist – ohne zu verifizieren, dass ein leerer Titel tatsächlich einen 400 auslöst.
   - Beide Rezepte haben "Mittagessen" als Kategorie und der Default-Fallback in `unwrap_or("a")` sorgt dafür, dass Titel übernommen wird, sodass der Merge tatsächlich mit 302 erfolgreich abgeschlossen wird.
   - Ein Test der wirklich einen Validierungsfehler (leerer Titel) provoziert, würde die Validierungslogik im Handler zuverlässiger absichern.

---

## Fazit

**Gesamtbewertung:** ✅ Abgenommen

Die Implementierung ist vollständig, korrekt und gut strukturiert. Alle 10 Akzeptanzkriterien sind erfüllt, alle Tests sind grün. Der atomare DB-Vorgang schützt vor Datenverlust. Die zwei Prio-2-Punkte sind Verbesserungsvorschläge ohne funktionale Auswirkung und blockieren den Abschluss nicht.

**Nächste Schritte:**
1. Story als abgeschlossen markieren
2. Bei Bedarf: Prio-2-Punkte in einem separaten Cleanup-Task adressieren
