# Review: Story 05 - Rezept-Liste alphabetisch sortiert

**Review-Datum:** 2026-03-27
**Story-Status:** Implementiert

---

## Zusammenfassung

Die Implementierung der Rezept-Liste ist solide und vollständig. Alle funktionalen Akzeptanzkriterien sind erfüllt: alphabetische Sortierung mit korrekter Umlaut-Behandlung (Rust-seitige Normalisierung statt externer Crate), Leerzustand mit Erstellen-Link, Navigation zur Detailansicht und "Neues Rezept"-Button. Die Test-Abdeckung ist sehr gut – sowohl Unit-Tests als auch E2E-Tests decken alle in der Story definierten Testfälle ab und laufen alle durch. Es gibt keine blockierenden Probleme; lediglich kleinere Verbesserungspunkte (Prio 2).

---

## Prüfung gegen den Plan

| Schritt | Status | Bemerkung |
|---------|--------|-----------|
| 1. Unit-Tests für `get_all_recipes` (TDD: rot) | ✅ | Alle 4 geplanten Tests implementiert |
| 2. DB-Abfrage auf alphabetische Sortierung umstellen | ✅ | Rust-seitige Normalisierung statt `unicode-collation`-Crate (wie im Plan als Alternative erwähnt); `normalize_for_sort` klar dokumentiert |
| 3. Unit-Tests für `index`-Handler (TDD: rot) | ✅ | Alle 9 geplanten Tests in `tests/recipe_list.rs` vorhanden |
| 4. Handler und Template anpassen | ✅ | H1 „Rezepte", Leerzustand-Link und „Neues Rezept"-Button korrekt |
| 5. CSS-Überprüfung und Ergänzungen | ✅ | `.empty-state`, `min-height: 44px`, `:focus-visible`, Mobile Responsive vorhanden |
| 6. E2E-Tests mit Playwright | ✅ | Alle 5 Testfälle implementiert; Seed-Datei erstellt |
| 7. Qualitätssicherung | ✅ | Alle Checks grün |

---

## Prüfung gegen Akzeptanzkriterien

| Kriterium | Status | Bemerkung |
|-----------|--------|-----------|
| **K1: Alle Rezepte werden angezeigt** | ✅ | E2E- und Unit-Tests bestätigen vollständige Anzeige |
| **K2: Alphabetische Sortierung** | ✅ | `normalize_for_sort` behandelt ä→a, ö→o, ü→u, ß→ss; Case-insensitiv; Unit- und E2E-Tests grün |
| **K3: Listeneinträge mit Titel und Kategorie** | ✅ | Template zeigt Titel als H2 im Link, Kategorien als `.category-tag`; Unit- und E2E-Tests prüfen beides |
| **K4: Leere Liste korrekt behandelt** | ✅ | `.empty-state` mit Text „Noch keine Rezepte vorhanden." und Link „Erstes Rezept anlegen" zu `/recipes/new` |
| **K5: Navigation zur Detailansicht** | ✅ | Link `/recipes/{id}` pro Listeneintrag; URL-Wechsel durch E2E-Test bestätigt |
| **K6: Startseite unter `/`** | ✅ | Route `GET /` liefert die Rezeptliste; H1 „Rezepte" |
| **K7: Link zum Erstellen** | ✅ | „Neues Rezept"-Button oben im `.actions`-Bereich immer sichtbar |
| **K8: Performance** | ✅ | Einzelne DB-Abfrage, In-Memory-Sortierung; für reale Datenmengen (< 200 Einträge) unproblematisch |
| **K9: Barrierefreiheit** | ✅ | `<ul>/<li>`-Struktur, H1 vorhanden, `:focus-visible`-Indikator, `min-height: 44px` für Mobile |

---

## Prüfung gegen Definition of Done

### Code-Qualität
- [x] `cargo build` — fehlerfrei
- [x] `cargo clippy -- -D warnings` — keine Warnungen
- [x] `cargo fmt --check` — korrekt formatiert
- [x] Keine ungenutzten Funktionen / Variablen

### Architektur-Einhaltung
- [x] Tech Stack: Rust + Axum + Askama + sqlx + SQLite (kein HTMX in dieser Story nötig)
- [x] SSR, keine JSON-APIs für UI
- [x] App funktioniert ohne JavaScript (reines HTML + CSS, Form-Posts)
- [x] Code in korrekten Verzeichnissen (`src/models/`, `templates/`, `src/static/css/`)

### Testing
- [x] Unit Tests geschrieben und bestanden (`cargo test`: 9 neue Tests in `recipe_list.rs`, 4 neue in `recipe_db.rs`)
- [x] E2E Tests geschrieben und bestanden (`npm run test:e2e`: 5 neue Playwright-Tests)

### Funktionale Anforderungen
- [x] Alle Akzeptanzkriterien erfüllt
- [x] Edge Cases behandelt (Leerzustand, Umlaute, Groß-/Kleinschreibung)
- [x] Keine neue Benutzereingabe in dieser Story (keine Validierung notwendig)

---

## Test-Ergebnisse

### Unit-Tests (`cargo test`)

| Test | Status |
|------|--------|
| `get_all_recipes_returns_alphabetically_sorted` | ✅ |
| `get_all_recipes_sorts_case_insensitively` | ✅ |
| `get_all_recipes_sorts_umlauts_correctly` | ✅ |
| `get_all_recipes_returns_empty_for_empty_db` | ✅ |
| `index_returns_200` | ✅ |
| `index_shows_h1_rezepte` | ✅ |
| `index_shows_all_recipes` | ✅ |
| `index_shows_recipes_in_alphabetical_order` | ✅ |
| `index_shows_empty_state_message` | ✅ |
| `index_shows_create_link_in_empty_state` | ✅ |
| `index_shows_new_recipe_button` | ✅ |
| `index_shows_category_for_recipe` | ✅ |
| `index_recipe_links_to_detail` | ✅ |
| Alle übrigen Regressionstests (21 Unit, 5 recipe_create, 4 recipe_delete, 13 recipe_detail) | ✅ |

**Gesamt: 53 Tests, 0 Fehler**

### E2E-Tests (`npm run test:e2e`)

| Test | Status |
|------|--------|
| Mehrere Rezepte alphabetisch sortiert anzeigen (K1, K2, K3) | ✅ |
| Leere Liste mit Meldung und Erstellen-Link (K4) | ✅ |
| Klick auf Listeneintrag zur Detailansicht (K5) | ✅ |
| Umlaute korrekt alphabetisch sortieren (K2) | ✅ |
| „Neues Rezept"-Button führt zum Formular (K7) | ✅ |
| Alle übrigen E2E-Tests (health, recipe-create, recipe-delete, recipe-detail, recipe-edit) | ✅ |

**Gesamt: 26 E2E-Tests, 0 Fehler**

### Code-Quality Checks

| Check | Ergebnis |
|-------|----------|
| `cargo build` | ✅ Fehlerfrei |
| `cargo clippy -- -D warnings` | ✅ Keine Warnungen |
| `cargo fmt --check` | ✅ Korrekt formatiert |
| `cargo test` | ✅ 53/53 Tests grün |
| `npm run test:e2e` | ✅ 26/26 Tests grün |

---

## Empfohlene Nacharbeit

### Prio 1 (Muss)

Keine Prio-1-Punkte. Alle Akzeptanzkriterien sind erfüllt, alle Tests grün.

### Prio 2 (Sollte)

1. **E2E-Test K4 (Leere Liste) ist bedingt**
   - Der Test `sollte leere Liste mit Meldung und Erstellen-Link anzeigen (K4)` prüft den Leerzustand nur, wenn zufällig keine Rezepte in der Test-DB vorhanden sind. Da die Tests keine per-Test-DB-Isolation haben und andere Tests Rezepte anlegen, wird der `empty-state`-Zweig im Playwright-Run oft nicht durchlaufen.
   - Empfehlung: Entweder eine separate Playwright-Projektkonfiguration mit frischer DB nur für diesen Test, oder den Test so umbauen, dass er aktiv eine leere DB voraussetzt (z.B. via `beforeEach`-Hook mit DB-Reset).

2. **Seed-Datei `recipe-list.sql` wird nicht genutzt**
   - Die erstellte Seed-Datei `tests/seeds/recipe-list.sql` wird in keinem E2E-Test tatsächlich geladen. Stattdessen legen die Tests Rezepte via UI an. Die Seed-Datei ist vorhanden, aber ungenutzt – ihr Zweck ist unklar.
   - Empfehlung: Datei entweder in den E2E-Tests nutzen (via `beforeAll`/`beforeEach` in der Playwright-Config) oder entfernen, um keine tote Dokumentation zu haben.

3. **Umlaut-Sortierung: `normalize_for_sort` reduziert, nicht äquivalent**
   - Der Kommentar in `recipe_db.rs` beschreibt die Funktion korrekt (ä→a), aber die Implementierung hat einen kleinen Tücken: `'Ä' => vec!['a']` und `'ä' => vec!['a']` geben beide Kleinbuchstaben zurück, was konsistent ist. Das `other.to_lowercase().next().unwrap_or(other)` für alle anderen Zeichen ist korrekt. Jedoch werden nicht-deutschen Sonderzeichen (z.B. é, è, ñ) nicht normalisiert und können je nach UTF-8-Codepunkt unerwartete Sortierposition bekommen. Für den definierten Use Case (deutschsprachige Rezepte) ist das ausreichend; für zukünftige Erweiterungen könnte die Nutzung der `unicode-collation`-Crate (wie im Plan beschrieben) robuster sein.
   - Empfehlung: Akzeptabel für MVP; im ADR festhalten, dass die einfache Normalisierung bewusst gewählt wurde.

4. **Template: `<h2>` für Rezepttitel in der Liste**
   - Die Rezepttitel werden in der Liste als `<h2>` ausgezeichnet. Bei vielen Rezepten entsteht dadurch eine flache `<h2>`-Struktur ohne semantische Hierarchie unterhalb der `<h1>`. Dies ist keine schwere Verletzung von WCAG 2.1 A, aber für Screenreader-Nutzer könnte eine `<span>` mit passender CSS-Klasse sauberer sein.
   - Empfehlung: Für zukünftige Stories prüfen, ob `<h2>` in Listenelementen semantisch gewünscht ist.

---

## Fazit

**Gesamtbewertung:** ✅ Abgenommen

Die Implementierung ist vollständig, korrekt und gut getestet. Alle Akzeptanzkriterien der Story sind erfüllt, alle automatisierten Tests (Unit und E2E) laufen durch, und der Code folgt den Architektur-Vorgaben. Die gewählte Lösung für die Umlaut-Sortierung (`normalize_for_sort` ohne externe Crate) ist pragmatisch und für den definierten Scope ausreichend. Die Prio-2-Punkte (vor allem der bedingte K4-Test und die ungenutzte Seed-Datei) können in einer Folge-Iteration aufgeräumt werden, blockieren aber nicht.

**Nächste Schritte:**
1. Story 07 (Volltextsuche) oder Story 08 (Kategorienfilter) beginnen – beide bauen auf dieser Listen-Ansicht auf
2. Optional: Seed-Datei-Nutzung in E2E-Tests für saubere Test-Isolation evaluieren
