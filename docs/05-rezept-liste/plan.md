# Implementierungsplan: Story 05 - Rezept-Liste alphabetisch sortiert

## Analyse des Ist-Zustands

Die Startseite (`GET /`) existiert bereits und zeigt eine Rezeptliste an. Die wesentlichen Bausteine sind vorhanden, aber die Story-Anforderungen werden noch nicht vollständig erfüllt:

| Baustein | Status | Bemerkung |
|---|---|---|
| Route `GET /` | vorhanden | in `src/routes/mod.rs` |
| Handler `index` | vorhanden | in `src/routes/recipes.rs` |
| DB-Funktion `get_all_recipes` | vorhanden | in `src/models/recipe_db.rs` |
| Template-Struct `IndexTemplate` | vorhanden | in `src/templates.rs` |
| HTML-Template `templates/index.html` | vorhanden | Grundstruktur vorhanden |
| CSS-Klassen `.recipe-list`, `.recipe-item` | vorhanden | in `src/static/css/app.css` |

**Defizite gegenüber den Story-Anforderungen:**

1. **Falsche Sortierung:** `get_all_recipes` sortiert nach `created_at DESC`, nicht alphabetisch nach Titel (K2 nicht erfüllt)
2. **Falsche Seitentitel:** Template zeigt H1 "Rezepte Übersicht" statt "Rezepte" (K1-Kleinigkeit, aber Story-Spec sagt "Rezepte")
3. **Keine Unit-Tests für die Index-Route:** Es gibt nur einen simplen Statuscode-200-Test in `recipe_create.rs`, kein Test für alphabetische Sortierung, Umlaut-Handling oder Leerzustand-Darstellung
4. **Keine E2E-Tests für die Listenansicht:** Es gibt keine dedizierte Playwright-Spec für die Rezeptliste
5. **Umlaut-Sortierung:** SQLite `COLLATE NOCASE` behandelt nur ASCII; für korrekte deutsche Umlautsortierung (ä→a, ö→o, ü→u) braucht es entweder `COLLATE NOCASE` mit Umlaut-Normalisierung oder eine Rust-seitige Nachsortierung
6. **Leerzustand-Text:** Aktueller Text "Noch keine Rezepte vorhanden. Erstellen Sie Ihr erstes Rezept!" ist korrekt, aber der Link-Text weicht vom Story-Wording ab ("Erstellen Sie Ihr erstes Rezept!" statt eines direkten Links-Buttons)
7. **Kein dedizierter `<h1>`-Test:** Die Überschrift ist korrekt, aber nicht durch Unit-Tests abgedeckt

---

## Technische Entscheidung: Umlaut-Sortierung

SQLite `COLLATE NOCASE` normalisiert keine deutschen Umlaute. Das Akzeptanzkriterium K2 fordert, dass `ä` wie `a`, `ö` wie `o` und `ü` wie `u` sortiert wird.

**Optionen:**
- Option A: Rust-seitige Nachsortierung mit `unicode-collation` Crate
- Option B: Zusätzliche SQLite-Spalte mit normalisiertem Sortierschlüssel
- Option C: SQLite-Abfrage mit `ORDER BY title COLLATE NOCASE` + Rust-seitige Feinsortierung

**Entscheidung: Option A** – Rust-seitige Nachsortierung direkt in `get_all_recipes`. Die Datenmenge ist realistisch (< 200 Rezepte), sodass In-Memory-Sortierung keine Performanceprobleme verursacht. Dies hält die DB-Schicht einfach und ermöglicht einfache Unit-Tests. Der `unicode-collation`-Crate implementiert Unicode Collation Algorithm (UCA) mit Locale-Unterstützung für DE.

---

## Technische Schritte

### Schritt 1: Unit-Tests für `get_all_recipes` schreiben (TDD: rot)

Neue Tests in `src/models/recipe_db.rs` (im bestehenden `#[cfg(test)]`-Block):

- [ ] Unit-Test `get_all_recipes_returns_alphabetically_sorted`: Legt drei Rezepte an ("Zupfbrot", "Apfelkuchen", "Bolognese"), prüft Reihenfolge [Apfelkuchen, Bolognese, Zupfbrot]
- [ ] Unit-Test `get_all_recipes_sorts_case_insensitively`: Legt Rezepte mit gemischter Groß-/Kleinschreibung an ("zitronenkuchen", "Apfelkuchen"), prüft "Apfelkuchen" vor "zitronenkuchen"
- [ ] Unit-Test `get_all_recipes_sorts_umlauts_correctly`: Legt "Überbackene Nudeln", "Apfelkuchen", "Ährenbrot" an, prüft Reihenfolge (Ährenbrot vor Apfelkuchen oder direkt nach – gemäß DE-Sortierung: ä ≈ a, also Ährenbrot vor Apfelkuchen)
- [ ] Unit-Test `get_all_recipes_returns_empty_for_empty_db`: Prüft, dass leere DB einen leeren Vec zurückgibt

### Schritt 2: DB-Abfrage auf alphabetische Sortierung umstellen (TDD: grün)

Änderung in `src/models/recipe_db.rs`:

- [ ] `get_all_recipes` DB-Query ändern: `ORDER BY created_at DESC` → `ORDER BY title COLLATE NOCASE ASC`
- [ ] `unicode-collation`-Crate in `Cargo.toml` hinzufügen (für Umlaut-korrekte Sortierung)
- [ ] Rust-seitige Nachsortierung in `get_all_recipes` mit dem UCA-Algorithmus implementieren (Locale DE)
- [ ] Alle Unit-Tests aus Schritt 1 müssen jetzt grün sein: `cargo test`

### Schritt 3: Unit-Tests für den `index`-Handler schreiben (TDD: rot)

Neue Test-Datei `tests/recipe_list.rs`:

- [ ] Hilfsfunktionen `setup_test_app` und `get_body` (analog zu `recipe_detail.rs`)
- [ ] Hilfsfunktion `create_recipe_with_title(app, title, category)` für präzise Tests
- [ ] Unit-Test `index_returns_200`: Prüft HTTP-Status 200
- [ ] Unit-Test `index_shows_h1_rezepte`: Prüft, dass `<h1>` den Text "Rezepte" enthält
- [ ] Unit-Test `index_shows_all_recipes`: Legt 3 Rezepte an, prüft dass alle 3 Titel im HTML vorkommen
- [ ] Unit-Test `index_shows_recipes_in_alphabetical_order`: Legt "Zupfbrot", "Apfelkuchen" an, prüft im HTML-Body, dass "Apfelkuchen" vor "Zupfbrot" auftaucht
- [ ] Unit-Test `index_shows_empty_state_message`: Ohne Rezepte prüft, dass "Noch keine Rezepte" oder ähnlicher Text vorhanden ist
- [ ] Unit-Test `index_shows_create_link_in_empty_state`: Ohne Rezepte prüft, dass ein Link zu `/recipes/new` vorhanden ist
- [ ] Unit-Test `index_shows_new_recipe_button`: Prüft, dass der "Neues Rezept"-Link zu `/recipes/new` auf der Seite ist
- [ ] Unit-Test `index_shows_category_for_recipe`: Legt Rezept mit Kategorie "Mittagessen" an, prüft, dass "Mittagessen" im HTML erscheint
- [ ] Unit-Test `index_recipe_links_to_detail`: Prüft, dass jeder Listeneintrag einen Link `/recipes/{id}` enthält

### Schritt 4: Handler und Template anpassen (TDD: grün)

- [ ] `templates/index.html`: H1-Text von "Rezepte Übersicht" auf "Rezepte" ändern (K1)
- [ ] `templates/index.html`: Leerzustand-Bereich erweitern – neben dem Text einen direkten Link-Button zu `/recipes/new` einfügen (K4, K7)
- [ ] Sicherstellen, dass der "Neues Rezept"-Button oben auf der Seite gut sichtbar ist (K7)
- [ ] Alle Unit-Tests aus Schritt 3 müssen jetzt grün sein: `cargo test`

### Schritt 5: CSS-Überprüfung und Ergänzungen

- [ ] Prüfen, ob `.empty-state` eine CSS-Klasse hat; falls nicht, ergänzen in `src/static/css/app.css`
- [ ] Sicherstellen, dass `.recipe-item-link` min. 44px Tipp-Zielfläche hat (Mobile K9)
- [ ] Sicherstellen, dass Fokus-Indikatoren für `.recipe-item-link` sichtbar sind (`:focus-visible`, WCAG 2.1 A)
- [ ] Prüfen, dass Responsive Layout für Mobile (max-width: 600px) korrekt ist – Listeneinträge volle Breite, gut tippbar

### Schritt 6: E2E-Tests mit Playwright

- [ ] Neue Datei `tests/e2e/recipe-list.spec.ts` erstellen
- [ ] Seed-Datei `tests/seeds/recipe-list.sql` mit Testrezepten erstellen (mindestens: "Apfelkuchen", "Bolognese", "Zupfbrot" + je eine Kategorie)
- [ ] **Testfall 1 (K1, K2, K3): Liste mit mehreren Rezepten anzeigen**
  - Rezepte per Seed oder UI anlegen (Apfelkuchen, Bolognese, Zupfbrot)
  - Startseite "/" öffnen
  - Alle 3 Rezepte sind als Liste sichtbar
  - Alphabetische Reihenfolge prüfen (Apfelkuchen vor Bolognese vor Zupfbrot)
  - Jeder Eintrag enthält Titel und mindestens eine Kategorie
- [ ] **Testfall 2 (K4): Leere Liste**
  - Frische DB ohne Rezepte
  - Startseite "/" öffnen
  - Meldung "Noch keine Rezepte" oder ähnlich sichtbar
  - Link zu `/recipes/new` vorhanden
- [ ] **Testfall 3 (K5, K3): Navigation zur Detailansicht**
  - Rezept anlegen
  - Startseite "/" öffnen
  - Klick auf Rezept-Listeneintrag
  - URL wechselt auf `/recipes/{id}`
  - H1 zeigt den Rezepttitel
- [ ] **Testfall 4 (K2): Alphabetische Sortierung mit Umlauten**
  - Rezepte "Überbackene Nudeln", "Apfelkuchen", "Ährenbrot" anlegen
  - Startseite "/" öffnen
  - Reihenfolge prüfen: Ährenbrot und Überbackene Nudeln korrekt nach ä/ü einsortiert
- [ ] **Testfall 5 (K7): Link zu neuem Rezept**
  - Startseite "/" öffnen
  - "Neues Rezept"-Button klicken
  - Weiterleitung auf Erstellungsformular

### Schritt 7: Qualitätssicherung

- [ ] `cargo test` – alle Unit-Tests grün
- [ ] `cargo clippy -- -D warnings` – keine Warnungen
- [ ] `cargo fmt --check` – korrekt formatiert
- [ ] `npm run test:e2e` – alle E2E-Tests grün

---

## URL-Struktur

```
GET  /   →  Rezept-Liste (alphabetisch sortiert), Startseite der Anwendung
```

Die bestehenden URLs bleiben unverändert. Keine neuen Endpunkte notwendig.

---

## Abhängigkeiten

- **Story 01 (Rezept erstellen)** – abgeschlossen, Grundlage für Datenbankstruktur und CREATE-Handler
- **Story 04 (Rezept-Detailansicht)** – abgeschlossen, Listeneinträge verlinken auf `GET /recipes/{id}`
- Technisch: `get_all_recipes` in `src/models/recipe_db.rs` wird geändert (Sortierung)
- Technisch: Neuer Crate `unicode-collation` in `Cargo.toml` (für Umlaut-Sortierung)
- Technisch: Kein Schema-Änderung, keine Migration notwendig

---

## Test-Checkliste

### Unit-Tests (`cargo test`)

- [ ] `get_all_recipes_returns_alphabetically_sorted` – Reihenfolge [A, B, Z]
- [ ] `get_all_recipes_sorts_case_insensitively` – Groß-/Kleinschreibung ignoriert
- [ ] `get_all_recipes_sorts_umlauts_correctly` – ä/ö/ü korrekt einsortiert
- [ ] `get_all_recipes_returns_empty_for_empty_db` – leere DB → leerer Vec
- [ ] `index_returns_200` – Status 200
- [ ] `index_shows_h1_rezepte` – H1 "Rezepte"
- [ ] `index_shows_all_recipes` – alle Titel im HTML
- [ ] `index_shows_recipes_in_alphabetical_order` – Reihenfolge im HTML korrekt
- [ ] `index_shows_empty_state_message` – Leerzustand-Text sichtbar
- [ ] `index_shows_create_link_in_empty_state` – Link zu `/recipes/new` im Leerzustand
- [ ] `index_shows_new_recipe_button` – "Neues Rezept"-Button vorhanden
- [ ] `index_shows_category_for_recipe` – Kategorie im Listeneintrag sichtbar
- [ ] `index_recipe_links_to_detail` – Link `/recipes/{id}` pro Eintrag vorhanden

### E2E-Tests (`npm run test:e2e`)

- [ ] Mehrere Rezepte alphabetisch sortiert anzeigen (K1, K2, K3)
- [ ] Leere Liste mit Meldung und Erstellen-Link (K4)
- [ ] Klick auf Eintrag öffnet Detailansicht, URL wechselt (K5)
- [ ] Umlaut-Sortierung korrekt (K2)
- [ ] "Neues Rezept"-Button führt zum Formular (K7)

### Manuell zu prüfen

- [ ] Responsive auf mobilem Gerät: Einträge volle Breite, mind. 44px tippbar
- [ ] Sehr langer Rezepttitel bricht sauber um, kein Layout-Bruch
- [ ] Tastatur-Navigation: Alle Listeneinträge per Tab erreichbar, per Enter aktivierbar
- [ ] Fokus-Indikatoren sichtbar
- [ ] Seite lädt in < 500ms bei 100+ Rezepten

---

## Offene Punkte

- **`unicode-collation` vs. einfache Umlaut-Ersetzung:** Falls der Crate zu schwer ist, kann alternativ eine einfache Hilfsfunktion `normalize_for_sort(title: &str)` implementiert werden, die ä→a, ö→o, ü→u, ß→ss ersetzt und dann case-insensitiv sortiert. Dies deckt den deutschen Use-Case vollständig ab ohne externe Abhängigkeit. Entscheidung fällen während der Implementierung nach Prüfung der Crate-Größe.
