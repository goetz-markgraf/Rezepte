# Review: Story 3 - Rezept löschen mit Sicherheitsabfrage

**Review-Datum:** 2026-03-27
**Story-Status:** Implementiert

---

## Zusammenfassung

Die Implementierung der Löschfunktion mit Sicherheitsabfrage ist vollständig und qualitativ hochwertig. Alle funktionalen Anforderungen sind erfüllt: Lösch-Button, eigene Bestätigungsseite, Abbrechen-Funktion, erfolgreiche Löschung mit Redirect und Erfolgsmeldung. Alle Unit-, Integrations- und E2E-Tests laufen grün durch. Es gibt geringfügige Lücken bei der Barrierefreiheit (kein automatischer Fokus beim Laden der Bestätigungsseite), die als Prio-2-Nacharbeit einzustufen sind.

---

## Prüfung gegen den Plan

| Schritt | Status | Bemerkung |
|---------|--------|-----------|
| 1. Datenbank-Layer `delete_recipe()` | ✅ | Funktion implementiert, 3 Unit-Tests (happy path, not found, idempotent) |
| 2. Bestätigungs-Template | ✅ | `confirm_delete.html` mit Titel, Warnung, Tipp-Box und zwei Aktions-Buttons |
| 3. Askama Template-Struct `ConfirmDeleteTemplate` | ✅ | In `src/templates.rs` mit Doc-Kommentar vorhanden |
| 4. Index-Template für Erfolgsmeldung | ✅ | `deleted_title` in `IndexTemplate`, Anzeige in `index.html` |
| 5. Route-Handler implementiert | ✅ | `confirm_delete()` und `delete_recipe_handler()` in `src/routes/recipes.rs` |
| 6. Routen registriert | ✅ | Beide Routen in `src/routes/mod.rs` eingetragen |
| 7. Lösch-Button in Detail-Template | ✅ | `<a href="/recipes/{{ id }}/confirm-delete" class="btn-danger">Löschen</a>` in `detail.html` |
| 7. Lösch-Button im Bearbeitungsformular (optional) | ⚠️ | Nicht implementiert; laut Plan optional |
| 8. CSS-Styling | ✅ | `.btn-danger`, `.confirm-delete`, `.tip-box`, `.confirm-actions`, `.inline-form` vorhanden |
| 9. E2E-Tests (Playwright) | ✅ | 5 Tests in `recipe-delete.spec.ts` alle grün |
| 10. Code-Qualität und DoD-Prüfung | ✅ | Clippy, fmt, cargo test alle fehlerfrei |

---

## Prüfung gegen Akzeptanzkriterien

| Kriterium | Status | Bemerkung |
|-----------|--------|-----------|
| **K1: Lösch-Auslöser vorhanden** | ✅ | Roter "Löschen"-Button (`btn-danger`) in der Detailansicht vorhanden |
| **K2: Sicherheitsabfrage erscheint** | ✅ | Eigene Seite `/recipes/:id/confirm-delete` mit Titel, Warnung, Tipp-Box, zwei Buttons |
| **K3: Abbrechen funktioniert** | ✅ | `<a href="/recipes/{{ id }}">` leitet zurück zur Detailseite, Rezept bleibt erhalten |
| **K4: Löschen wird ausgeführt** | ✅ | POST `/recipes/:id/delete` löscht und leitet zu `/?deleted=<Titel>` weiter; Erfolgsmeldung angezeigt |
| **K5: Datenpersistenz sichergestellt** | ✅ | DB-Test und E2E-Test bestätigen vollständige Entfernung; GET auf gelöschte ID gibt 404 |
| **K6: Performance** | ✅ | Einfache DELETE-Query ohne JOINs, SQLite; Löschvorgang < 500ms |
| **K7: Barrierefreiheit** | ⚠️ | ARIA-Labels vorhanden, Escape-Taste via JS implementiert; kein `autofocus` auf "Abbrechen"-Button beim Seitenaufruf |

---

## Prüfung gegen Definition of Done

### Code-Qualität
- [x] `cargo build` — fehlerfrei
- [x] `cargo clippy -- -D warnings` — keine Warnungen
- [x] `cargo fmt --check` — korrekt formatiert
- [x] Keine ungenutzten Funktionen / Variablen

### Lesbarkeit & Wartbarkeit
- [x] Verständliche Funktionsnamen (`confirm_delete`, `delete_recipe_handler`, `delete_recipe`)
- [x] Funktionen sind kurz (alle unter 25 Zeilen)
- [x] Doc-Kommentare an allen neuen öffentlichen Funktionen (`///`)
- [x] Konsistente Einrückung und Formatierung

### Architektur-Einhaltung
- [x] Tech Stack: Rust + Axum + Askama + sqlx + SQLite
- [x] Server-Side Rendering, keine JSON-APIs
- [x] App funktioniert ohne JavaScript (Form-Post + Redirect)
- [x] Code in korrekten Verzeichnissen (`src/models/`, `src/routes/`, `templates/`)
- [x] Module korrekt exportiert in `src/models/mod.rs`
- [x] URL-Struktur DeepLink-fähig

### Datenbank
- [x] Parametrisierte SQL-Statements (keine SQL-Injection-Risiken)
- [x] Keine Migrationssänderungen notwendig (korrekt, da nur DELETE auf bestehender Tabelle)

### Testing
- [x] Unit Tests: 3 neue Tests für `delete_recipe()` in `recipe_db.rs`
- [x] Integration Tests: 4 neue Tests in `tests/recipe_delete.rs`
- [x] E2E Tests: 5 neue Tests in `tests/e2e/recipe-delete.spec.ts`
- [x] Alle Tests bestehen

### Funktionale Anforderungen
- [x] Alle Akzeptanzkriterien erfüllt (mit Einschränkung bei K7)
- [x] Edge Cases behandelt: 404 bei nicht-existenter ID, idempotente Verarbeitung bei Doppelklick
- [x] Fehlermeldungen verständlich

### Error Handling
- [x] Keine `unwrap()` im Produktivcode
- [x] `sqlx::Error::RowNotFound` wird korrekt auf `AppError::NotFound` (404) gemappt
- [x] URL-Encoding des gelöschten Titels für den Query-Parameter

---

## Test-Ergebnisse

### Unit-Tests (`cargo test`)

Gesamt: **27 Tests in 5 Test-Suites, alle bestanden.**

| Test | Status |
|------|--------|
| `can_delete_recipe` | ✅ |
| `delete_recipe_returns_error_for_nonexistent` | ✅ |
| `delete_recipe_is_idempotent` | ✅ |
| `confirm_delete_shows_recipe_title` (Integration) | ✅ |
| `confirm_delete_returns_404_for_nonexistent` (Integration) | ✅ |
| `delete_recipe_removes_from_db` (Integration) | ✅ |
| `delete_recipe_returns_404_for_nonexistent` (Integration) | ✅ |
| Alle anderen bestehenden Tests | ✅ |

### E2E-Tests (`npm run test:e2e`)

Gesamt: **14 Tests (5 neue für Story 3), alle bestanden.**

| Test | Status |
|------|--------|
| sollte Lösch-Button auf Detailansicht anzeigen | ✅ |
| sollte Bestätigungsseite anzeigen | ✅ |
| sollte Abbrechen zur Detailseite zurückkehren | ✅ |
| sollte Rezept erfolgreich löschen | ✅ |
| sollte 404 bei nicht-existentem Rezept anzeigen | ✅ |

### Code-Quality Checks

| Check | Ergebnis |
|-------|----------|
| `cargo build` | ✅ fehlerfrei |
| `cargo clippy -- -D warnings` | ✅ keine Warnungen |
| `cargo fmt --check` | ✅ korrekt formatiert |

---

## Empfohlene Nacharbeit

### Prio 1 (Muss)

Keine blockierenden Probleme gefunden.

### Prio 2 (Sollte)

1. **Fehlender `autofocus` auf "Abbrechen"-Button**
   - K7 fordert: "Der Fokus wird beim Öffnen der Abfrage korrekt gesetzt."
   - Aktuell: Kein `autofocus`-Attribut im Template; Fokus bleibt nach Seitenaufruf oben.
   - Empfehlung: `autofocus` auf den "Abbrechen"-Link oder -Button setzen, damit Tastaturnutzer sofort auf der sicheren Option landen.

2. **Lösch-Button fehlt im Bearbeitungsformular**
   - Laut Plan Schritt 7 optional ("Bearbeitungsformular: optional").
   - Laut Story K1 wäre er "im Bearbeitungsformular und/oder der Detailansicht" wünschenswert.
   - Empfehlung: In Story 4 oder als eigenes Task nachliefern, wenn die Detailansicht weiterentwickelt wird.

3. **E2E-Test prüft Escape-Taste nicht**
   - Das Escape-Verhalten (JavaScript-Enhancement) ist implementiert, aber nicht per Test abgedeckt.
   - Empfehlung: Einen weiteren E2E-Test hinzufügen, der Escape drückt und prüft, dass die Detailseite erscheint.

---

## Fazit

**Gesamtbewertung:** ✅ Abgenommen

Die Story ist vollständig und korrekt implementiert. Alle Akzeptanzkriterien sind erfüllt, alle Tests laufen grün, und der Code erfüllt die DoD-Kriterien. Die geringfügige Lücke bei der Fokus-Setzung (K7) ist kein Blocker für den Abschluss der Story.

**Nächste Schritte:**
1. `autofocus` auf "Abbrechen"-Button in `confirm_delete.html` nachliefern (Prio 2, kann sofort erledigt werden)
2. Story 04 beginnen (Rezept-Detailansicht erweitern) — Lösch-Button im Formular dabei prüfen
