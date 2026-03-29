# Implementierungsplan: Story 12 — Kombinierte Filter

## Analyse des Ist-Zustands

### Was bereits vollständig implementiert ist

Durch die Stories 7–11 ist die kombinierte Filterlogik auf **Datenbank- und Routing-Ebene bereits vollständig** vorhanden:

- **DB-Ebene**: Alle drei DB-Filterfunktionen (`filter_recipes_by_categories`, `filter_recipes_not_made_recently`, `filter_recipes_next_seven_days`) akzeptieren alle vier Parameter gleichzeitig: `categories`, `search_query`, `bewertung`. Die SQL-Klauseln werden sauber mit AND verknüpft.
- **URL-Builder**: Alle Toggle-URL-Funktionen (`build_category_toggle_url`, `build_not_made_toggle_url`, `build_next_seven_days_toggle_url`, `build_bewertung_toggle_url`) bewahren jeweils alle anderen aktiven Filter-Parameter in der neuen URL.
- **Template**: Das Template zeigt bereits `aria-pressed` und `active` für alle Filter-Typen korrekt an, inkl. Kombinationen.
- **"Keine Treffer"-Meldungen**: Das Template enthält bereits spezifische Meldungen für viele Kombinationen (Volltextsuche + Kategorie, Bewertung + Kategorie, etc.).
- **Konflikt-Handling**: `build_not_made_toggle_url` und `build_next_seven_days_toggle_url` schließen sich gegenseitig aus (nur EINER der beiden wird in die URL geschrieben).
- **Story-11-E2E-Tests (K8-K10)**: Kategorien + Bewertung, Volltextsuche + Bewertung, "Länger nicht gemacht" + Bewertung sind bereits in `recipe-rating-filter.spec.ts` getestet.

### Was noch fehlt

1. **K10: "Alle Filter zurücksetzen"-Button**: Es fehlt ein dedizierter Button, der alle Filter auf einmal aufhebt (→ `href="/"`). Der vorhandene "Alle"-Button in der Kategorie-Navigation setzt nur Kategorien zurück und bewahrt andere Filter. Es fehlt ein separater "Alle Filter zurücksetzen"-Button, der **nur erscheint**, wenn mindestens ein Filter aktiv ist (Kategorie, Volltextsuche, Datumsfilter oder Bewertungsfilter).

2. **K12: Keine-Treffer-Meldungen für Drei-Filter-Kombinationen**: Das Template hat bereits viele Kombinationen, aber **nicht alle** Drei-Filter-Kombinationen (z.B. `not_made + bewertung` explizit, `next_seven_days + bewertung + kategorie + suche`). Eine allgemeine Meldung für alle drei-Filter-Fälle sollte als Fallback vorhanden sein.

3. **K13: Konflikt "Länger nicht gemacht" + "Nächste 7 Tage" via DeepLink**: Wenn ein Benutzer manuell beide Filter in der URL setzt (`?filter=laenger-nicht-gemacht&filter=naechste-7-tage`), muss die App konsistent reagieren. Aktuell wird in `IndexQuery` nur `filter: Option<String>` geparst — d.h. der erste Wert "gewinnt". Das Verhalten muss explizit definiert und getestet werden.

4. **E2E-Tests für Story 12**: Es fehlen die E2E-Testfälle aus der Story (Testfall 1–7 der Story als eigene Testdatei `tests/e2e/recipe-combined-filters.spec.ts`).

5. **Rust-Integrationstests**: Einige Kombinationen aus der Story sind noch nicht als Rust-Integrationstests vorhanden (insbesondere 3-Filter-Kombinationen).

6. **`IndexTemplate` um `reset_all_filters_url` erweitern**: Damit der "Alle Filter zurücksetzen"-Button nur bei aktiven Filtern erscheint.

---

## Technische Schritte

### Schritt 1: Analyse und Dokumentation des Konflikt-Verhaltens (K13)

**Entscheidung:** Wenn beide Datumsfilter gleichzeitig per DeepLink gesetzt werden (`?filter=laenger-nicht-gemacht&filter=naechste-7-tage`), gilt: **Der erste Parameter "gewinnt"** (da `IndexQuery.filter: Option<String>` nur einen Wert aufnimmt). Das ist ein technisch konsistentes Verhalten. Es erscheint keine spezielle Fehlermeldung — die App ignoriert den zweiten Parameter still.

- [ ] Kein Code-Change nötig — das Verhalten ist bereits korrekt und konsistent
- [ ] Verhalten in Plan dokumentieren (s.o.)
- [ ] Unit-Test: HTTP-Integrationstest prüft, dass `?filter=laenger-nicht-gemacht&filter=naechste-7-tage` konsistent nur einen der beiden Filter aktiviert

### Schritt 2: "Alle Filter zurücksetzen"-Button (K10)

**Ziel:** Ein Link zu `/` erscheint im Filterbereich, wenn irgendein Filter aktiv ist. Wenn keine Filter aktiv sind, ist dieser Link ausgeblendet.

- [ ] In `IndexTemplate` (`src/templates.rs`) ein Feld `any_filter_active: bool` hinzufügen
  - Berechnung: `!active_categories.is_empty() || !search_query.is_empty() || not_made_filter_active || next_seven_days_filter_active || bewertung_filter.is_some()`
- [ ] In `index`-Handler in `routes/recipes.rs` das neue Feld befüllen
- [ ] In `templates/index.html`: Einen "Alle Filter zurücksetzen"-Link einfügen, der nur erscheint wenn `any_filter_active`

```html
{% if any_filter_active %}
<a href="/" class="btn-link reset-all-filters-btn" aria-label="Alle Filter zurücksetzen">
    Alle Filter zurücksetzen
</a>
{% endif %}
```

- [ ] CSS in `src/static/css/app.css` für `.reset-all-filters-btn` ergänzen (dezenter Link-Stil, gut sichtbar aber nicht dominierend)
- [ ] Unit-Test: Integrationstest prüft, dass der Button bei aktiven Filtern vorhanden ist und bei keinen Filtern fehlt

### Schritt 3: Keine-Treffer-Meldungen vervollständigen (K12)

**Ziel:** Das Template enthält für alle Filterkombinationen klare Hinweistexte.

Analyse der fehlenden Kombinationen im Template:

- `not_made + bewertung` ohne Kategorie/Suche → Meldung vorhanden? Nein, fällt auf `not_made` allein zurück
- `not_made + bewertung + kategorie` → nicht explizit abgedeckt
- `next_seven_days + bewertung` → nicht explizit abgedeckt
- `next_seven_days + bewertung + kategorie` → nicht explizit abgedeckt

**Lösung:** Bestehende if-else-Kette in `templates/index.html` um die fehlenden Fälle ergänzen, sodass alle Zwei- und Drei-Filter-Kombinationen mit Bewertungsfilter eine präzise Meldung erhalten.

- [ ] `templates/index.html`: Neue Zweige in der `{% if recipes.is_empty() %}` Kette ergänzen:
  - `not_made + bewertung + kategorie + suche`
  - `not_made + bewertung + kategorie`
  - `not_made + bewertung + suche`
  - `not_made + bewertung`
  - `next_seven_days + bewertung + kategorie + suche`
  - `next_seven_days + bewertung + kategorie`
  - `next_seven_days + bewertung + suche`
  - `next_seven_days + bewertung`
- [ ] Sicherstellen, dass die Meldungen sprechend und auf Deutsch formuliert sind
- [ ] Hinweis auf "Alle Filter zurücksetzen" in den Meldungen optional verlinken

### Schritt 4: Rust-Integrationstests für kombinierte Filter (TDD)

Datei: `tests/recipe_combined_filters.rs` (neu erstellen)

- [ ] Test: `three_filters_category_rating_not_made_returns_matching_recipes`
  - Given: "Dinkelbrot" (Brot, 5 Sterne, planned_date 2025-06-01), "Roggenbrot" (Brot, 5 Sterne, planned_date 2026-06-01), "Linseneintopf" (Mittagessen, 5 Sterne, planned_date 2024-01-01)
  - When: GET `/?kategorie=Brot&bewertung=favoriten&filter=laenger-nicht-gemacht`
  - Then: HTTP 200, "Dinkelbrot" im Body, "Linseneintopf" nicht im Body, "Roggenbrot" (Zukunft) nicht im Body

- [ ] Test: `category_and_search_combined_returns_intersection`
  - Given: "Dinkelbrot" (Brot), "Roggenbrot" (Brot), "Dinkel-Müsli" (Snacks)
  - When: GET `/?kategorie=Brot&q=Dinkel`
  - Then: "Dinkelbrot" vorhanden, "Roggenbrot" nicht, "Dinkel-Müsli" nicht

- [ ] Test: `category_and_rating_combined_returns_intersection`
  - Given: "Dinkelbrot" (Brot, 4 Sterne), "Roggenbrot" (Brot, 2 Sterne), "Spaghetti" (Mittagessen, 5 Sterne)
  - When: GET `/?kategorie=Brot&bewertung=gut`
  - Then: "Dinkelbrot" vorhanden, "Roggenbrot" nicht, "Spaghetti" nicht

- [ ] Test: `no_results_from_combination_shows_appropriate_message`
  - Given: Nur "Roggenbrot" (Brot, 2 Sterne)
  - When: GET `/?kategorie=Brot&bewertung=favoriten`
  - Then: Kein Rezept, Hinweistext `class="search-no-results"` vorhanden

- [ ] Test: `reset_all_filters_button_appears_when_filter_active`
  - Given: Ein Filter ist aktiv (`?bewertung=gut`)
  - When: GET `/?bewertung=gut`
  - Then: Body enthält "Alle Filter zurücksetzen"-Link oder Text

- [ ] Test: `reset_all_filters_button_absent_when_no_filter_active`
  - Given: Keine Filter aktiv
  - When: GET `/`
  - Then: Body enthält keinen "Alle Filter zurücksetzen"-Text (oder Link zu `/` mit dem spezifischen CSS-Klasse)

- [ ] Test: `conflict_both_date_filters_in_url_applies_first_one`
  - Given: URL mit `?filter=laenger-nicht-gemacht&filter=naechste-7-tage`
  - When: GET mit diesem URL
  - Then: HTTP 200, nur einer der Filter ist aktiv (kein Absturz, kein 500)

- [ ] Test: `deeplink_multiple_filters_returns_correct_state`
  - Given: "Dinkelbrot" (Brot, 5 Sterne) existiert
  - When: GET `/?kategorie=Brot&bewertung=favoriten`
  - Then: "Dinkelbrot" vorhanden, `aria-pressed="true"` für Bewertungs-Button, Kategorie-Button aktiv

### Schritt 5: E2E-Tests (Playwright)

Datei: `tests/e2e/recipe-combined-filters.spec.ts` (neu erstellen)

Hilfsfunktion `createRecipeWithOptions` (title, categories, rating?, plannedDate?) analog zu anderen Test-Dateien.

- [ ] **Testfall 1: K1 — Kategorie + Volltextsuche**
  ```
  // Given: "Dinkelbrot" (Brot), "Roggenbrot" (Brot), "Dinkel-Müsli" (Snacks)
  // When: Kategorie "Brot" wählen, dann "Dinkel" ins Suchfeld eingeben
  // Then: Nur "Dinkelbrot" sichtbar
  // And: URL enthält kategorie=Brot und q=Dinkel
  ```

- [ ] **Testfall 2: K2 — Kategorie + Bewertungsfilter**
  ```
  // Given: "Dinkelbrot" (Brot, 4★), "Roggenbrot" (Brot, 2★), "Spaghetti" (Mittagessen, 5★)
  // When: Kategorie "Brot" klicken, dann "★★★+ Nur Gute" klicken
  // Then: Nur "Dinkelbrot" sichtbar, URL enthält kategorie + bewertung
  ```

- [ ] **Testfall 3: K5 + K6 — Bewertungsfilter + "Länger nicht gemacht" (2 und 3 Filter)**
  ```
  // Given: "Linseneintopf" (4★, 1.1.2025), "Erbsensuppe" (4★, 1.1.2026), "Kartoffelsuppe" (2★, 1.1.2024)
  // When: /?bewertung=gut&filter=laenger-nicht-gemacht direkt aufrufen
  // Then: "Linseneintopf" sichtbar, "Kartoffelsuppe" nicht (Rating), "Erbsensuppe" nicht (Zukunft)
  ```

- [ ] **Testfall 4: K6 — Drei Filter: Kategorie + Bewertung + Länger nicht gemacht**
  ```
  // Given: "Dinkelbrot" (Brot, 5★, 1.6.2025), "Roggenbrot" (Brot, 5★, 1.1.2026), "Linseneintopf" (Mittagessen, 5★, 1.1.2024)
  // When: /?kategorie=Brot&bewertung=favoriten&filter=laenger-nicht-gemacht
  // Then: "Dinkelbrot" sichtbar, "Roggenbrot" nicht (Zukunft), "Linseneintopf" nicht (Kategorie)
  // And: Alle drei Filter als aktiv markiert
  ```

- [ ] **Testfall 5: K9 — Einzelnen Filter deaktivieren ohne andere zu verlieren**
  ```
  // Given: Kategorie "Brot" + "Nur Gute" aktiv (via URL-Parameter)
  // When: Klick auf "Nur Gute" zum Deaktivieren
  // Then: Kategorie "Brot" noch aktiv, Bewertungsfilter inaktiv, URL nur noch kategorie=Brot
  ```

- [ ] **Testfall 6: K12 — Keine Treffer durch Kombination**
  ```
  // Given: Nur Brot-Rezepte mit ≤ 2 Sternen (eindeutiger Suffix)
  // When: /?q={suffix}&kategorie=Brot&bewertung=favoriten
  // Then: .search-no-results sichtbar, .recipe-item count = 0
  ```

- [ ] **Testfall 7: K11 — DeepLink mit mehreren Filtern**
  ```
  // Given: "Dinkelbrot" (Brot, 5★), "Roggenbrot" (Brot, 2★)
  // When: /?kategorie=Brot&bewertung=favoriten direkt aufrufen
  // Then: Nur "Dinkelbrot" sichtbar, Kategorie "Brot" als aktiv markiert, Bewertungsfilter "Favoriten" aktiv
  ```

- [ ] **Testfall 8: K10 — "Alle Filter zurücksetzen"-Button**
  ```
  // Given: Kategorie "Brot" + Bewertung "Nur Gute" aktiv
  // When: Klick auf "Alle Filter zurücksetzen"
  // Then: URL = /, alle Filter inaktiv, alle Rezepte sichtbar
  ```

### Schritt 6: CSS-Ergänzungen

- [ ] In `src/static/css/app.css` Styling für `.reset-all-filters-btn` ergänzen:
  - Dezenter Link-Stil (kleiner Text, gedämpfte Farbe)
  - Sichtbar aber nicht dominierend gegenüber den Filter-Buttons
  - Hover-Effekt konsistent mit restlichem Design

### Schritt 7: Qualitätschecks

- [ ] `cargo build` — keine Compiler-Fehler
- [ ] `cargo clippy -- -D warnings` — keine Clippy-Warnungen
- [ ] `cargo fmt --check` — korrekte Formatierung
- [ ] `cargo test` — alle Unit-Tests und Integrationstests grün
- [ ] `npm run test:e2e` — alle E2E-Tests grün

---

## URL-Struktur

Keine neuen Routen. Alle kombinierten Filter laufen über die bestehende Startseite:

```
GET  /                                          → alle Rezepte (keine Filter)
GET  /?q=dinkel&kategorie=Brot                  → Suche + Kategorie (K1)
GET  /?kategorie=Brot&bewertung=gut             → Kategorie + Bewertung (K2)
GET  /?kategorie=Brot&filter=laenger-nicht-gemacht  → Kategorie + Datumsfilter (K3)
GET  /?kategorie=Brot&filter=naechste-7-tage    → Kategorie + 7-Tage-Filter (K4)
GET  /?bewertung=gut&filter=laenger-nicht-gemacht   → Bewertung + Datumsfilter (K5)
GET  /?kategorie=Brot&bewertung=favoriten&filter=laenger-nicht-gemacht  → 3 Filter (K6)
GET  /?q=dinkel&bewertung=gut                   → Suche + Bewertung (K7)
```

---

## Abhängigkeiten

- Stories 7–11 sind abgeschlossen (alle Einzelfilter implementiert)
- Keine neuen technischen Abhängigkeiten

---

## Offene Entscheidungen

### Entscheidung: Konflikt "Länger nicht gemacht" + "Nächste 7 Tage" (K13)

**Gewählt:** "Erster Parameter gewinnt" — da `IndexQuery.filter: Option<String>` nur einen Wert aufnimmt, wird bei `?filter=laenger-nicht-gemacht&filter=naechste-7-tage` der erste Wert genommen (Axum-Behavior). Die Toggle-URLs verhindern durch ihre Logik bereits, dass beide Filter gleichzeitig via UI gesetzt werden können.

Kein explizites Deaktivieren des anderen Filters nötig, da die URL-Builder dies bereits ausschließen (see `build_not_made_toggle_url` / `build_next_seven_days_toggle_url` — beide fügen nie beide `filter`-Parameter gleichzeitig ein).

### Entscheidung: "Alle Filter zurücksetzen"-Button (K10)

**Gewählt:** Dedizierter Button/Link `href="/"` mit Text "Alle Filter zurücksetzen", sichtbar nur wenn `any_filter_active`. Alternativ könnte der "Alle"-Kategorie-Button auf `/` zeigen (würde alle Filter löschen), aber das widerspräche dem bestehenden Verhalten (andere Filter bleiben erhalten).

**Neue Felder in `IndexTemplate`:**
- `any_filter_active: bool` — true wenn irgendein Filter aktiv ist

---

## Test-Checkliste

- [ ] Rust-Unit-Test: `three_filters_category_rating_not_made_returns_matching_recipes`
- [ ] Rust-Unit-Test: `category_and_search_combined_returns_intersection`
- [ ] Rust-Unit-Test: `category_and_rating_combined_returns_intersection`
- [ ] Rust-Unit-Test: `no_results_from_combination_shows_appropriate_message`
- [ ] Rust-Unit-Test: `reset_all_filters_button_appears_when_filter_active`
- [ ] Rust-Unit-Test: `reset_all_filters_button_absent_when_no_filter_active`
- [ ] Rust-Unit-Test: `conflict_both_date_filters_in_url_applies_first_one`
- [ ] Rust-Unit-Test: `deeplink_multiple_filters_returns_correct_state`
- [ ] E2E-Test: K1 — Kategorie + Volltextsuche
- [ ] E2E-Test: K2 — Kategorie + Bewertung
- [ ] E2E-Test: K5/K6 — Bewertung + "Länger nicht gemacht"
- [ ] E2E-Test: K6 — Drei Filter gleichzeitig
- [ ] E2E-Test: K9 — Einzelnen Filter deaktivieren
- [ ] E2E-Test: K12 — Keine Treffer
- [ ] E2E-Test: K11 — DeepLink
- [ ] E2E-Test: K10 — "Alle Filter zurücksetzen"
- [ ] Manueller Test: Verschiedene Filter-Kombinationen im Browser durchspielen (inkl. Bookmarking)

---

## Definition of Done (Checkliste)

- [ ] Keine Compiler-Fehler oder -Warnungen (`cargo build`)
- [ ] Keine Clippy-Warnings (`cargo clippy -- -D warnings`)
- [ ] Korrekte Formatierung (`cargo fmt --check`)
- [ ] Alle Unit-Tests und Integrationstests grün (`cargo test`)
- [ ] Alle E2E-Tests grün (`npm run test:e2e`)
- [ ] Alle 15 Akzeptanzkriterien aus der story.md erfüllt
- [ ] Keine Panics oder `unwrap()` im Produktivcode
- [ ] Neue öffentliche Funktionen haben Doc-Kommentare (`///`)
- [ ] Jeder Test enthält Given/When/Then als deutsche Kommentare
- [ ] DeepLink-fähige URLs mit allen Filter-Kombinationen funktionieren
- [ ] App funktioniert ohne JavaScript (Form-Posts + Redirects)
