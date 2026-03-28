# Implementierungsplan: Story 7 - Volltextsuche

## Technische Entscheidung: Suchmechanismus

**Gewählt: LIKE-Queries (kein FTS5)**

Begründung:
- Bis 200 Rezepte ist LIKE performant genug (< 1 Sekunde laut NFR)
- Kein FTS5-Setup, keine Migration für virtuelle Tabellen, keine Sync-Logik
- Umlaut-unterstützung per Rust-seitiger Normalisierung (Muster: `normalize_for_sort` existiert bereits)
- Case-insensitiv: SQLite `LOWER()` + `LOWER(?)` genügt
- Kein FTS5 = weniger Komplexität, leichter zu testen und zu warten
- FTS5 kann nachgerüstet werden, falls Performance-Probleme auftreten (ADR dokumentieren)

---

## Technische Schritte

### Schritt 1: Datenbank-Query — Such-Funktion in `recipe_db.rs`

- [ ] Funktion `search_recipes(pool, query)` in `src/models/recipe_db.rs` hinzufügen
  - SQL: `WHERE LOWER(title) LIKE ? OR LOWER(ingredients) LIKE ? OR LOWER(instructions) LIKE ?`
  - Suchbegriff als `%{term}%` übergeben (LIKE-Wildcard), `term = query.to_lowercase()`
  - Ergebnisse alphabetisch sortieren (gleiche `normalize_for_sort`-Logik wie `get_all_recipes`)
  - Bei leerem `query`: alle Rezepte zurückgeben (delegiert an `get_all_recipes`)
- [ ] Unit-Test: `search_recipes` findet Treffer im Titel
- [ ] Unit-Test: `search_recipes` findet Treffer in Zutaten
- [ ] Unit-Test: `search_recipes` findet Treffer in Anleitung
- [ ] Unit-Test: `search_recipes` ist case-insensitiv
- [ ] Unit-Test: `search_recipes` gibt leere Liste bei keinen Treffern zurück
- [ ] Unit-Test: `search_recipes` gibt alle Rezepte zurück bei leerem Suchbegriff
- [ ] Unit-Test: `search_recipes` zeigt Rezept nur einmal, auch wenn Begriff in mehreren Feldern

### Schritt 2: Model-Export in `src/models/mod.rs`

- [ ] `search_recipes` in `mod.rs` exportieren (analog zu `get_all_recipes`)

### Schritt 3: Template-Datenstruktur in `src/templates.rs`

- [ ] `IndexTemplate` um Feld `search_query: Option<String>` erweitern
  - Wird für Vorausfüllen des Suchfelds und URL-DeepLink benötigt

### Schritt 4: Route und Handler in `src/routes/recipes.rs`

- [ ] `IndexQuery` um Feld `q: Option<String>` erweitern
- [ ] Handler `index` anpassen:
  - Wenn `q` vorhanden und nicht leer: `search_recipes` aufrufen
  - Wenn `q` fehlt oder leer: `get_all_recipes` aufrufen (keine Änderung bestehenden Verhaltens)
  - `search_query` in `IndexTemplate` übergeben
- [ ] Integration-Test: `GET /?q=bolognese` gibt HTTP 200 zurück
- [ ] Integration-Test: `GET /?q=bolognese` enthält nur passende Rezepte im Body
- [ ] Integration-Test: `GET /?q=` zeigt alle Rezepte
- [ ] Integration-Test: `GET /?q=xyzxyz` zeigt "Keine Rezepte gefunden"-Meldung

### Schritt 5: Templates anpassen

- [ ] `src/templates/index.html` — Suchfeld einfügen:
  - `<form>` mit `action="/"` und `method="GET"` oberhalb der Rezeptliste
  - `<label for="q">Rezepte durchsuchen</label>` (Barrierefreiheit)
  - `<input id="q" name="q" type="search" placeholder="Rezepte durchsuchen...">`
  - Wert vorausfüllen mit `{{ search_query | default("") }}`
  - HTMX-Attribute: `hx-get="/" hx-target="#recipe-results" hx-trigger="input changed delay:300ms" hx-push-url="true"`
  - Submit-Button (funktioniert auch ohne JS)
- [ ] `src/templates/index.html` — Ergebnisbereich:
  - `<div id="recipe-results">` als HTMX-Ziel um Rezeptliste und Leer-Meldungen wrappen
  - ARIA live region: `aria-live="polite"` auf `#recipe-results`
  - Meldung bei keinen Treffern: `"Keine Rezepte für '{{ search_query }}' gefunden"` (nur wenn `search_query` gesetzt und Ergebnisliste leer)
  - Bestehende Leerzustand-Meldung ("Noch keine Rezepte") bleibt für leere DB

### Schritt 6: HTMX-Partial-Endpunkt (optional, für Live-Suche ohne Full-Page-Reload)

**Entscheidung:** HTMX mit `hx-target="#recipe-results"` gegen `GET /` — der Server gibt immer die vollständige Seite zurück, aber HTMX ersetzt nur den Zielbereich. Das reicht für den MVP, kein separater Partial-Endpunkt nötig.

Falls HTMX `HX-Request`-Header erkannt wird, kann der Handler alternativ nur das Fragment rendern. Für MVP: vollständige Seite, HTMX-Target macht Partial-Update im Browser.

- [ ] Prüfen, ob `hx-select="#recipe-results"` ausreicht (kein neuer Endpunkt)
- [ ] Falls nötig: `HX-Request`-Header prüfen und nur Fragment rendern

### Schritt 7: Styling in `src/static/css/app.css`

- [ ] Suchfeld-Styles: Breite, Padding, Border, Focus-Indikator (Barrierefreiheit)
- [ ] Meldung "Keine Rezepte gefunden" stylen (visuell unterscheidbar, nicht als Fehler)
- [ ] Responsiv: Suchfeld auf Mobile gut bedienbar (min-height für Touch-Targets)

### Schritt 8: E2E-Tests in `tests/e2e/recipe-search.spec.ts`

- [ ] Seed-Datei `tests/seeds/recipe-search.sql` erstellen:
  - Rezept "Spaghetti Bolognese" (Mittagessen, Zutaten: Hackfleisch, Anleitung: kochen)
  - Rezept "Pfannkuchen" (Snacks, Zutaten: Dinkelvollkornmehl, Anleitung: in der Pfanne backen)
  - Rezept "Brot im Ofen" (Brot, Zutaten: Mehl, Anleitung: im Ofen backen)

- [ ] **Testfall 1 (K2, Suche nach Titel):**
  ```
  // Given: App enthält "Spaghetti Bolognese" und "Pfannkuchen"
  // When: "Bolognese" in Suchfeld eingeben
  // Then: Nur "Spaghetti Bolognese" sichtbar, "Pfannkuchen" nicht sichtbar
  ```

- [ ] **Testfall 2 (K2, Suche nach Zutat):**
  ```
  // Given: App enthält Rezept mit Zutat "Dinkelvollkornmehl"
  // When: "Dinkel" in Suchfeld eingeben
  // Then: Rezept mit Dinkelvollkornmehl sichtbar, andere Rezepte nicht
  ```

- [ ] **Testfall 3 (K6, Keine Treffer):**
  ```
  // Given: App enthält Rezepte
  // When: "xyzxyzxyz" in Suchfeld eingeben
  // Then: Meldung "Keine Rezepte" sichtbar, Rezeptliste leer
  ```

- [ ] **Testfall 4 (K5, Leere Suche):**
  ```
  // Given: Benutzer hat "Bolognese" gesucht, gefilterte Liste sichtbar
  // When: Suchfeld leeren
  // Then: Alle Rezepte wieder sichtbar
  ```

- [ ] **Testfall 5 (K4, Groß-/Kleinschreibung):**
  ```
  // Given: App enthält "Spaghetti Bolognese"
  // When: "BOLOGNESE" in Suchfeld eingeben
  // Then: "Spaghetti Bolognese" in Ergebnisliste sichtbar
  ```

- [ ] **Testfall 6 (K2, Suche nach Anleitung):**
  ```
  // Given: App enthält Rezept mit "im Ofen backen" in der Anleitung
  // When: "Ofen" in Suchfeld eingeben
  // Then: Entsprechendes Rezept sichtbar
  ```

- [ ] **Testfall 7 (K7, Suchbegriff in URL / DeepLink):**
  ```
  // Given: Benutzer gibt "Bolognese" ein
  // When: Seite lädt mit ?q=Bolognese
  // Then: Suchfeld enthält "Bolognese", gefilterte Ergebnisse sichtbar
  ```

- [ ] **Testfall 8 (K1, Suchfeld sichtbar):**
  ```
  // Given: Rezeptliste-Seite aufgerufen
  // When: Seite gerendert
  // Then: Suchfeld mit Label sichtbar, Platzhaltertext vorhanden
  ```

### Schritt 9: Integrations-Tests in `tests/recipe_search.rs`

- [ ] Neue Testdatei `tests/recipe_search.rs` erstellen
- [ ] Test: `GET /?q=bolognese` gibt HTTP 200
- [ ] Test: `GET /?q=bolognese` enthält "Spaghetti Bolognese" im Body
- [ ] Test: `GET /?q=bolognese` enthält "Pfannkuchen" nicht im Body
- [ ] Test: `GET /?q=dinkel` findet Rezept mit "Dinkel" in Zutaten
- [ ] Test: `GET /?q=ofen` findet Rezept mit "Ofen" in Anleitung
- [ ] Test: `GET /?q=BOLOGNESE` findet "Spaghetti Bolognese" (case-insensitiv)
- [ ] Test: `GET /?q=xyzxyz` zeigt "Keine Rezepte" Meldung
- [ ] Test: `GET /?q=` zeigt alle Rezepte
- [ ] Test: `GET /` ohne q-Parameter zeigt alle Rezepte (Rückwärtskompatibilität)

### Schritt 10: DoD-Checkliste und Abschluss

- [ ] `cargo build` ohne Warnings
- [ ] `cargo clippy -- -D warnings` ohne Warnings
- [ ] `cargo fmt --check` ohne Diff
- [ ] `cargo test` alle Tests grün
- [ ] `npm run test:e2e` alle E2E-Tests grün
- [ ] Öffentliche Funktionen haben Doc-Kommentare (`///`)
- [ ] `architecture.md` — URL-Tabelle für `?q=` ist bereits dokumentiert (keine Änderung nötig)

---

## URL-Struktur

```
GET  /           →  Alle Rezepte (leere Suche)
GET  /?q=pasta   →  Volltextsuche nach "pasta" in Titel, Zutaten, Anleitung
GET  /?q=        →  Alle Rezepte (leere Suche, äquivalent zu /)
GET  /?deleted=X →  Bestehend: Erfolgsmeldung nach Löschen (bleibt unverändert)
```

HTMX-Flow:
```
input#q [hx-get="/"] [hx-trigger="input changed delay:300ms"]
        [hx-target="#recipe-results"] [hx-push-url="true"] [hx-select="#recipe-results"]
  → GET /?q={term}
  → Server rendert volle Seite
  → HTMX tauscht #recipe-results aus, URL wird aktualisiert
```

---

## Abhängigkeiten

- Story 5 (Rezept-Liste alphabetisch sortiert) muss abgeschlossen sein — Suche erweitert diese Liste
- `normalize_for_sort` aus `recipe_db.rs` wird für Sortierung der Suchergebnisse wiederverwendet
- Bestehender `IndexTemplate` und `IndexQuery` werden erweitert (keine Breaking Changes)
- Kein neues Datenbankschema, keine Migration erforderlich

---

## Test-Checkliste

- [ ] Unit-Test: `search_recipes` findet Treffer in Titel (case-insensitiv)
- [ ] Unit-Test: `search_recipes` findet Treffer in Zutaten
- [ ] Unit-Test: `search_recipes` findet Treffer in Anleitung
- [ ] Unit-Test: `search_recipes` gibt leere Liste bei keinen Treffern zurück
- [ ] Unit-Test: `search_recipes` gibt alle Rezepte bei leerem Suchbegriff zurück
- [ ] Unit-Test: Rezept erscheint nur einmal, auch bei Treffer in mehreren Feldern
- [ ] Integration-Test: `GET /?q=<term>` HTTP 200, richtiger Body
- [ ] Integration-Test: `GET /?q=` zeigt alle Rezepte
- [ ] Integration-Test: `GET /?q=<kein-treffer>` zeigt Keine-Treffer-Meldung
- [ ] Integration-Test: Rückwärtskompatibilität `GET /` ohne q-Parameter
- [ ] E2E-Test: Suche nach Titel (Testfall 1)
- [ ] E2E-Test: Suche nach Zutat (Testfall 2)
- [ ] E2E-Test: Keine Treffer mit Meldung (Testfall 3)
- [ ] E2E-Test: Leere Suche zeigt alle Rezepte (Testfall 4)
- [ ] E2E-Test: Groß-/Kleinschreibung (Testfall 5)
- [ ] E2E-Test: Suche nach Anleitung (Testfall 6)
- [ ] E2E-Test: DeepLink mit ?q=... (Testfall 7)
- [ ] E2E-Test: Suchfeld sichtbar mit Label (Testfall 8)
- [ ] Manueller Test: Responsivität auf Mobile (Chrome DevTools)
- [ ] Manueller Test: Tastatur-Navigation (Tab, Eingabe ohne Mausklick)
- [ ] Manueller Test: Screenreader-Ankündigung bei Ergebniswechsel (ARIA live region)

---

## Offene Punkte

- Soll die Suche beim Tippen (HTMX live, 300ms delay) oder nur nach Drücken von Enter/Button auslösen?
  → **Empfehlung:** HTMX live mit `delay:300ms` + Form-Submit als Fallback (Progressive Enhancement)
- Sollen Treffer in der Ergebnisliste visuell hervorgehoben werden (Stichwort fett/markiert)?
  → **Empfehlung:** Nicht im MVP, da server-seitiges Highlighting komplex ist; als Backlog-Item dokumentieren
- FTS5 als zukünftige Alternative: ADR in `docs/07-volltextsuche/adrs.md` festhalten, falls Performance-Probleme auftreten
