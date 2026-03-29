# Implementierungsplan: Story 18 — Wochenvorschau für geplante Rezepte

## Überblick

Die Wochenvorschau ist eine neue, dedizierte Seite (`/wochenvorschau`), die alle Rezepte der
laufenden Kalenderwoche (Montag–Sonntag) strukturiert nach Wochentagen anzeigt. Sie benötigt
keine neue DB-Migration (das `planned_date`-Feld existiert bereits).

**Neue Dateien:**
- `src/routes/wochenvorschau.rs` — Handler
- `templates/wochenvorschau.html` — Template
- `tests/wochenvorschau.rs` — Rust-Integrationstests
- `tests/e2e/wochenvorschau.spec.ts` — Playwright E2E-Tests

**Geänderte Dateien:**
- `src/models/recipe_db.rs` — neue DB-Abfrage `get_recipes_current_week`
- `src/models/mod.rs` — Re-Export der neuen Funktion
- `src/templates.rs` — neue Structs `WochenvorschauTemplate`, `WochentageintragTemplate`
- `src/routes/mod.rs` — neue Route + Modul
- `templates/base.html` — Navigation-Link zur Wochenvorschau

---

## Technische Analyse

### Wochenberechnung (Rust `time`-Crate)

Die `time`-Crate (v0.3) ist bereits im Projekt. Die relevanten APIs:

```rust
use time::{Date, OffsetDateTime, Weekday};

// Heutiges Datum (UTC, serverseitig)
let today: Date = OffsetDateTime::now_utc().date();

// Montag der laufenden Woche (ISO: Woche beginnt Montag)
// time::Weekday::Monday = 0 Tage von Montag
let days_from_monday = today.weekday().number_days_from_monday() as i64;
let monday: Date = today - time::Duration::days(days_from_monday);
let sunday: Date = monday + time::Duration::days(6);
```

`Weekday::number_days_from_monday()` gibt 0 (Mo) bis 6 (So) zurück — exakt was wir brauchen.

### SQLite-Abfrage

```sql
SELECT id, title, categories, ingredients, instructions, planned_date, created_at, updated_at, rating
FROM recipes
WHERE planned_date >= ?1   -- Montag (ISO-Format: YYYY-MM-DD)
  AND planned_date <= ?2   -- Sonntag
ORDER BY planned_date ASC, title ASC
```

`time::Date` wird von sqlx mit dem `time`-Feature direkt als SQLite DATE gebunden (Format
`YYYY-MM-DD`). Kein manuelles Stringformatieren nötig.

### Serverseitige Gruppierung (kein SQL GROUP BY)

Nach der DB-Abfrage werden die Rezepte in Rust nach Wochentag gruppiert:

```rust
// Alle 7 Tage der Woche als geordnete Liste
let tage: Vec<Wochentag> = (0..7)
    .map(|i| monday + time::Duration::days(i))
    .map(|datum| Wochentag {
        datum,
        wochentag_name: german_weekday_name(datum.weekday()),
        datum_anzeige: format_german_date(datum),  // "Montag, 30. März"
        rezepte: recipes.iter()
            .filter(|r| r.planned_date == Some(datum))
            .map(|r| WochentagesEintragItem { id: r.id, title: r.title.clone() })
            .collect(),
    })
    .collect();
```

### Template-Daten-Struktur

```rust
// In src/templates.rs

pub struct WochentagesEintragItem {
    pub id: i64,
    pub title: String,
}

pub struct Wochentag {
    pub datum: time::Date,             // für Vergleiche
    pub wochentag_name: String,        // "Montag"
    pub datum_anzeige: String,         // "Montag, 30. März"
    pub rezepte: Vec<WochentagesEintragItem>,
}

#[derive(Template)]
#[template(path = "wochenvorschau.html")]
pub struct WochenvorschauTemplate {
    pub tage: Vec<Wochentag>,
    pub kw_anzeige: String,   // "KW 14 · 30. März – 5. April 2026"
    pub hat_rezepte: bool,    // true wenn mindestens 1 Rezept in der Woche
}
```

### Navigation

Der Link zur Wochenvorschau kommt in `templates/base.html` in die `<header>`-Navigation neben
dem bestehenden Site-Title-Link.

---

## Technische Schritte

### Schritt 1: DB-Layer — `get_recipes_current_week`

- [ ] In `src/models/recipe_db.rs`: Funktion `get_recipes_current_week` hinzufügen
  - Parameter: `pool: &SqlitePool, monday: Date, sunday: Date`
  - SQL: `WHERE planned_date >= ?1 AND planned_date <= ?2 ORDER BY planned_date ASC, title ASC`
  - Gibt `Vec<Recipe>` zurück
  - `monday` und `sunday` werden als `time::Date` direkt gebunden (sqlx time-Feature)
- [ ] In `src/models/mod.rs`: `get_recipes_current_week` re-exportieren
- [ ] Unit-Test in `recipe_db.rs` (`#[tokio::test]` im bestehenden `mod tests`):
  - Test: Rezept in der Woche wird zurückgegeben
  - Test: Rezept außerhalb der Woche (vor Montag) wird nicht zurückgegeben
  - Test: Rezept außerhalb der Woche (nach Sonntag) wird nicht zurückgegeben
  - Test: Rezept ohne `planned_date` wird nicht zurückgegeben
  - Test: Mehrere Rezepte am gleichen Tag werden beide zurückgegeben
  - Test: Sortierung: nach Datum aufsteigend, dann alphabetisch

### Schritt 2: Template-Structs in `src/templates.rs`

- [ ] `WochentagesEintragItem`-Struct hinzufügen (id: i64, title: String)
- [ ] `Wochentag`-Struct hinzufügen (datum_anzeige: String, wochentag_name: String, rezepte: Vec)
- [ ] `WochenvorschauTemplate`-Struct mit Askama-`#[template(path = "wochenvorschau.html")]`
  - Felder: `tage: Vec<Wochentag>`, `kw_anzeige: String`, `hat_rezepte: bool`

### Schritt 3: Route und Handler `src/routes/wochenvorschau.rs`

- [ ] Neue Datei `src/routes/wochenvorschau.rs` mit Handler `wochenvorschau_handler`
  - Berechnet `today`, `days_from_monday`, `monday`, `sunday` mit `time`-Crate
  - Ruft `get_recipes_current_week(&pool, monday, sunday)` auf
  - Berechnet `kw_anzeige` (Hilfsfunktion `format_kw_header`)
  - Baut `Vec<Wochentag>` durch Gruppierung der Rezepte nach Datum
  - Setzt `hat_rezepte = tage.iter().any(|t| !t.rezepte.is_empty())`
  - Rendert `WochenvorschauTemplate`
- [ ] Hilfsfunktionen im selben Modul:
  - `german_weekday_long(weekday: time::Weekday) -> &'static str`
    → "Montag", "Dienstag", ..., "Sonntag"
  - `format_day_display(date: time::Date) -> String`
    → "Montag, 30. März" (nutzt bestehenden GERMAN_MONTHS_LONG-Ansatz)
  - `format_kw_header(monday: time::Date, sunday: time::Date) -> String`
    → "KW 14 · 30. März – 5. April 2026"
    → Sonderfall: Wenn Montag und Sonntag im gleichen Monat: "KW 14 · 30. März – 5. April 2026"
    → Monat-Kürzel: voller Monatsname auf Deutsch
- [ ] In `src/routes/mod.rs`:
  - `pub mod wochenvorschau;` hinzufügen
  - Route `GET /wochenvorschau` registrieren

### Schritt 4: Template `templates/wochenvorschau.html`

- [ ] Neue Datei `templates/wochenvorschau.html` erstellen, die `base.html` erweitert
- [ ] Seitenüberschrift mit KW-Anzeige: `<h1>Wochenvorschau <span class="kw-label">{{ kw_anzeige }}</span></h1>`
- [ ] Wenn `!hat_rezepte`: freundliche Meldung "Für diese Woche noch nichts geplant"
- [ ] Sieben Abschnitte als semantische Liste/Beschreibungsliste:
  ```html
  <dl class="wochenvorschau-liste">
    {% for tag in tage %}
    <div class="wochentag-abschnitt{% if tag.rezepte.is_empty() %} wochentag-leer{% endif %}">
      <dt class="wochentag-titel">{{ tag.datum_anzeige }}</dt>
      <dd class="wochentag-inhalt">
        {% if tag.rezepte.is_empty() %}
        <span class="nichts-geplant">Nichts geplant</span>
        {% else %}
        <ul class="wochentag-rezepte">
          {% for rezept in tag.rezepte %}
          <li><a href="/recipes/{{ rezept.id }}">{{ rezept.title }}</a></li>
          {% endfor %}
        </ul>
        {% endif %}
      </dd>
    </div>
    {% endfor %}
  </dl>
  ```
- [ ] Link zurück zur Rezeptliste: `<a href="/" class="btn-link">Zur Rezeptliste</a>`

### Schritt 5: Navigation in `templates/base.html`

- [ ] Link "Wochenvorschau" in die `<header>`-Navigation einfügen:
  ```html
  <header>
      <a href="/" class="site-title">Rezepte</a>
      <nav class="main-nav">
          <a href="/wochenvorschau" class="nav-link">Wochenvorschau</a>
      </nav>
  </header>
  ```
  (Struktur und CSS-Klassen so wählen, dass das bestehende Layout nicht bricht)

### Schritt 6: CSS-Styling in `src/static/css/app.css`

- [ ] Styles für `main-nav` im Header (falls noch nicht vorhanden): flexbox-Row, gap
- [ ] Styles für `.kw-label`: dezentere Schriftgröße oder Farbe
- [ ] Styles für `.wochenvorschau-liste`: Liste ohne bullets, mit klarer Trennung
- [ ] Styles für `.wochentag-abschnitt`: Rand/Padding, ggf. leichter Hintergrund
- [ ] Styles für `.wochentag-titel` (`<dt>`): fette Überschrift, größere Schrift
- [ ] Styles für `.wochentag-inhalt` (`<dd>`): Margin-Start zurücksetzen (dt/dd-Standard)
- [ ] Styles für `.nichts-geplant`: gedimmte Farbe (z.B. `color: var(--text-muted)`)
- [ ] Styles für `.wochentag-rezepte`: unstyled list, Einzel-Links normal
- [ ] Responsive: auf Mobile bleibt die Liste vertikal, kein horizontales Scrollen
- [ ] Styles für `.wochentag-leer`: optional leicht gedimmter Hintergrund für leere Tage

### Schritt 7: Rust-Integrationstests `tests/wochenvorschau.rs`

- [ ] Setup-Funktion `setup_test_app()` (wie in anderen Testdateien)
- [ ] Hilfsfunktion `create_recipe_with_date(app, title, categories, planned_date)` (wie in `recipe_next_seven_days_filter.rs`)
- [ ] Hilfsfunktion `date_in_days(n: i64) -> String` für Offset von heute (wie bestehend)
- [ ] Tests (TDD: Test zuerst, dann Implementation):

  ```
  wochenvorschau_returns_200()
  // Given: App ohne Rezepte
  // When: GET /wochenvorschau
  // Then: HTTP 200

  wochenvorschau_shows_all_seven_weekdays()
  // Given: App ohne Rezepte
  // When: GET /wochenvorschau
  // Then: Body enthält "Montag", "Dienstag", ..., "Sonntag"

  wochenvorschau_shows_recipe_in_current_week()
  // Given: Rezept mit planned_date = Mittwoch dieser Woche
  // When: GET /wochenvorschau
  // Then: Body enthält Rezepttitel

  wochenvorschau_does_not_show_recipe_from_next_week()
  // Given: Rezept mit planned_date = Montag nächste Woche (monday + 7 days)
  // When: GET /wochenvorschau
  // Then: Body enthält Rezepttitel NICHT

  wochenvorschau_does_not_show_recipe_from_last_week()
  // Given: Rezept mit planned_date = Sonntag letzte Woche (monday - 1 day)
  // When: GET /wochenvorschau
  // Then: Body enthält Rezepttitel NICHT

  wochenvorschau_shows_empty_state_when_no_recipes()
  // Given: Keine Rezepte mit planned_date diese Woche
  // When: GET /wochenvorschau
  // Then: Body enthält "Für diese Woche noch nichts geplant"

  wochenvorschau_shows_multiple_recipes_on_same_day()
  // Given: Zwei Rezepte mit gleichem planned_date in dieser Woche
  // When: GET /wochenvorschau
  // Then: Body enthält beide Rezepttitel

  wochenvorschau_recipe_link_leads_to_detail()
  // Given: Rezept mit ID und planned_date diese Woche
  // When: GET /wochenvorschau
  // Then: Body enthält "/recipes/{id}" Link

  wochenvorschau_shows_kw_header()
  // Given: App
  // When: GET /wochenvorschau
  // Then: Body enthält "KW " (Kalenderwochen-Angabe)

  wochenvorschau_does_not_show_recipe_without_date()
  // Given: Rezept ohne planned_date
  // When: GET /wochenvorschau
  // Then: Body enthält Rezepttitel NICHT
  ```

### Schritt 8: E2E-Tests `tests/e2e/wochenvorschau.spec.ts`

- [ ] Hilfsfunktion `createRecipeWithDate(page, title, categories, plannedDate?)` (wie in anderen Specs)
- [ ] Hilfsfunktion `currentWeekDateInDays(offset: number): string` — berechnet Datum in der
  laufenden Woche (Offset relativ zu Montag dieser Woche, um stabile Wochentag-Zuordnung
  zu haben). Alternativ: `futureDateInDays(n)` für Tage relativ zu heute.
- [ ] Tests (je Akzeptanzkriterium mindestens ein Test, Given/When/Then als Kommentare):

  **K1: Seite erreichbar und in Navigation verlinkt**
  ```
  test('K1: /wochenvorschau ist aufrufbar und in Navigation verlinkt', ...)
  // Given: Die App ist gestartet
  // When: Benutzer öffnet die Startseite
  // Then: Link "Wochenvorschau" in der Navigation sichtbar
  // When: Benutzer klickt den Link
  // Then: URL ist /wochenvorschau, HTTP 200, Überschrift "Wochenvorschau" sichtbar
  ```

  **K2: Alle 7 Wochentage werden angezeigt**
  ```
  test('K2: Alle 7 Wochentage erscheinen auf der Seite', ...)
  // Given: /wochenvorschau wird aufgerufen
  // Then: "Montag", "Dienstag", "Mittwoch", "Donnerstag", "Freitag", "Samstag", "Sonntag" sichtbar
  ```

  **K3: Rezept erscheint unter richtigem Wochentag**
  ```
  test('K3: Rezept mit planned_date wird unter korrektem Wochentag angezeigt', ...)
  // Given: "Spaghetti Bolognese" hat planned_date = in 2 Tagen (innerhalb Woche)
  // When: Benutzer öffnet /wochenvorschau
  // Then: "Spaghetti Bolognese" erscheint auf der Seite
  // And: Rezeptname ist als klickbarer Link vorhanden
  ```

  **K4: Mehrere Rezepte am selben Tag**
  ```
  test('K4: Mehrere Rezepte am gleichen Tag erscheinen beide', ...)
  // Given: "Pfannkuchen" und "Rührei" haben beide das gleiche planned_date (heute oder morgen)
  // When: Benutzer öffnet /wochenvorschau
  // Then: Beide Rezepte sind unter demselben Tag sichtbar
  ```

  **K5: Keine geplanten Rezepte → freundliche Meldung**
  ```
  test('K5: Ohne Rezepte in der Woche erscheint freundliche Meldung', ...)
  // Given: Kein Rezept hat ein planned_date in der aktuellen Woche
  // When: Benutzer öffnet /wochenvorschau
  // Then: Text "Für diese Woche noch nichts geplant" sichtbar
  ```

  **K3+K6: Klick auf Rezeptnamen führt zur Detailansicht**
  ```
  test('K3/K6: Rezeptlink führt zur Detailansicht', ...)
  // Given: "Spaghetti Bolognese" hat planned_date in der aktuellen Woche
  // When: Benutzer öffnet /wochenvorschau und klickt auf "Spaghetti Bolognese"
  // Then: Benutzer sieht die Detailansicht (/recipes/{id}), Titel "Spaghetti Bolognese" sichtbar
  ```

  **K6: DeepLink / Bookmark**
  ```
  test('K5/Deeplink: /wochenvorschau direkt per URL aufrufbar', ...)
  // Given: Ein Rezept hat planned_date in der aktuellen Woche
  // When: Benutzer ruft /wochenvorschau direkt auf (Bookmark-Simulation)
  // Then: HTTP 200, Wochenvorschau korrekt angezeigt, Rezept sichtbar
  ```

  **K7: Link zurück zur Rezeptliste**
  ```
  test('K7: Link zurück zur Rezeptliste vorhanden', ...)
  // Given: /wochenvorschau wird aufgerufen
  // Then: Ein Link "Zur Rezeptliste" oder Navigation zurück zu "/" vorhanden
  // When: Link geklickt
  // Then: Benutzer ist auf der Rezeptliste "/"
  ```

  **Barrierefreiheit (K9)**
  ```
  test('K9: Semantisches HTML — Wochentage als dt-Elemente oder Überschriften', ...)
  // Given: /wochenvorschau wird aufgerufen
  // Then: Wochentag-Bezeichnungen sind in semantischen Elementen (dt oder h2/h3)
  // And: Rezept-Links haben aussagekräftige Labels (nicht nur "hier")
  ```

### Schritt 9: Qualitätschecks

- [ ] `cargo build` — keine Compiler-Fehler oder Warnungen
- [ ] `cargo clippy -- -D warnings` — keine Clippy-Warnungen
- [ ] `cargo fmt --check` — Code korrekt formatiert
- [ ] `cargo test` — alle Unit- und Integrationstests grün
- [ ] `npm run test:e2e` — alle E2E-Tests grün

---

## URL-Struktur

```
GET  /wochenvorschau  →  Wochenvorschau der aktuellen Kalenderwoche (Montag–Sonntag)
```

Keine Query-Parameter nötig (MVP: nur aktuelle Woche). Die Seite ist direkt per Bookmark
aufrufbar (DeepLink-fähig).

---

## Abhängigkeiten

- Story 28 (Datum-Eingabe, `planned_date`-Feld in DB) muss abgeschlossen sein — ist es bereits
- Story 10 (Filter "Nächste 7 Tage") ist implementiert — Muster für DB-Abfrage und
  Datums-Hilfsfunktionen können wiederverwendet werden (`date_in_days`, `format_planned_date_*`)
- Story 04 (Rezept-Detailansicht) muss implementiert sein — Rezeptlinks verweisen auf `/recipes/{id}` — ist bereits implementiert
- Keine neue DB-Migration nötig

**Technische Wiederverwendung:**
- `GERMAN_MONTHS_LONG` aus `src/routes/recipes.rs` → für Datumsformatierung nutzen
  (ggf. in ein gemeinsames Modul extrahieren oder duplizieren)
- `normalize_for_sort` aus `recipe_db.rs` — nicht nötig für Wochenvorschau (Sortierung via SQL)
- Pattern für `setup_test_app()` und `get_body()` aus `tests/recipe_next_seven_days_filter.rs`

---

## Test-Checkliste

- [ ] Unit-Test: `get_recipes_current_week` gibt Rezepte im Wochenfenster zurück
- [ ] Unit-Test: `get_recipes_current_week` schließt Rezepte vor Montag aus
- [ ] Unit-Test: `get_recipes_current_week` schließt Rezepte nach Sonntag aus
- [ ] Unit-Test: `get_recipes_current_week` schließt Rezepte ohne `planned_date` aus
- [ ] Unit-Test: `get_recipes_current_week` gibt mehrere Rezepte am selben Tag zurück
- [ ] Integrationstest: GET /wochenvorschau gibt HTTP 200 zurück
- [ ] Integrationstest: Alle 7 Wochentage im HTML-Body
- [ ] Integrationstest: Rezept in der Woche erscheint im Body
- [ ] Integrationstest: Rezept der nächsten Woche erscheint NICHT
- [ ] Integrationstest: Rezept der letzten Woche erscheint NICHT
- [ ] Integrationstest: Leerzustand-Meldung bei keinen Woche-Rezepten
- [ ] Integrationstest: Zwei Rezepte am gleichen Tag erscheinen beide
- [ ] Integrationstest: Rezept-Link enthält korrekte ID
- [ ] Integrationstest: KW-Angabe im Body
- [ ] E2E-Test (K1): Seite erreichbar, in Navigation verlinkt
- [ ] E2E-Test (K2): Alle 7 Wochentage sichtbar
- [ ] E2E-Test (K3): Rezept unter korrektem Tag mit klickbarem Link
- [ ] E2E-Test (K4): Mehrere Rezepte am gleichen Tag
- [ ] E2E-Test (K5): Freundliche Leer-Meldung
- [ ] E2E-Test (K3/K6): Klick auf Rezeptnamen → Detailansicht
- [ ] E2E-Test (DeepLink): URL direkt aufrufbar (Bookmark)
- [ ] E2E-Test (K7): Link zurück zur Rezeptliste
- [ ] Manuell: Responsive Layout auf Mobile (Handy-Viewport) überprüfen
- [ ] Manuell: KW-Anzeige im Header korrekt (aktuelle KW + Datumsbereich)
- [ ] Manuell: "Nichts geplant"-Text für Tage ohne Rezept sichtbar

---

## Offene Punkte

- **Monatsname bei Wochenüberschnitt:** "KW 14 · 30. März – 5. April 2026" — wenn Woche
  Monatswechsel überspannt, beide Monatsnamen anzeigen. Sonderfall implementieren:
  `if monday.month() != sunday.month() { voller_text } else { kompakter_text }`
- **Wochennummer-Berechnung:** `time::Date` hat keine direkte `iso_week()`-Methode in v0.3.
  Alternative: Wochennummer manuell berechnen über `ordinal()` und Wochentag des 1. Januar,
  oder einfachste Annäherung: `((today.ordinal() - 1 + day_offset_of_jan1) / 7) + 1`.
  Prüfen ob `time` v0.3 `Date::iso_year_week()` oder `Date::sunday_based_week()` anbietet.
  → Recherche vor Implementierung: `time::Date` API-Dokumentation prüfen.
- **CSS-Header-Layout:** Aktuell hat `base.html` nur `<a href="/" class="site-title">Rezepte</a>`
  im Header. Das Einfügen einer `<nav>` erfordert minimale CSS-Anpassung damit der Header
  weiterhin gut aussieht (z.B. `header { display: flex; justify-content: space-between; }`).
