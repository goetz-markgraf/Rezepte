# Implementierungsplan: Story 19 – Wochenvorschau nach Wochentagen formatiert

## Ausgangslage

Story 18 ist vollständig implementiert. Die relevanten Dateien:

- `src/templates.rs` — `Wochentag`-Struct mit einem einzigen Feld `datum_anzeige: String`
- `src/routes/wochenvorschau.rs` — Handler mit `format_day_display(datum)` und Mapping auf `Wochentag`
- `templates/wochenvorschau.html` — `<dt class="wochentag-titel">{{ tag.datum_anzeige }}</dt>`
- `src/static/css/app.css` — `.wochentag-abschnitt`, `.wochentag-leer`, `.wochentag-titel` usw.
- `tests/wochenvorschau.rs` — Rust-Integrationstests für Story 18
- `tests/e2e/wochenvorschau.spec.ts` — Playwright-Tests für Story 18

Die Änderungen in Story 19 sind rein additiv: neue Felder in der Struct, eine neue Hilfsfunktion,
Template-Anpassung und CSS-Ergänzung. Keine Datenbankmigrationen nötig.

---

## Technische Schritte

### Schritt 1: `Wochentag`-Struct in `src/templates.rs` erweitern (TDD: Unit-Test zuerst)

**Ziel:** Die Struct bekommt zwei neue boolesche Felder und das Datum wird in Name + Kurzform aufgeteilt.
Das bisherige `datum_anzeige`-Feld wird ersetzt.

- [ ] Unit-Test für die neue Struct-Struktur schreiben (in `src/routes/wochenvorschau.rs` im `#[cfg(test)]`-Block):
  - Test: `wochentag_felder_sind_korrekt_befuellt` — prüft dass bei `ist_heute=true`, `ist_vergangen=false`,
    `wochentag_name="Mittwoch"` und `datum_kurz="1. April"` alle Felder korrekt gesetzt sind
  - Zuerst schreiben, dann sehen wie der Compiler mit dem alten Struct scheitert (rot)
- [ ] `Wochentag`-Struct in `src/templates.rs` anpassen:
  ```rust
  pub struct Wochentag {
      /// Wochentag-Name: "Montag" bis "Sonntag"
      pub wochentag_name: String,
      /// Kurzform des Datums: "30. März"
      pub datum_kurz: String,
      /// true wenn dieser Tag = heute (serverseitig berechnet)
      pub ist_heute: bool,
      /// true wenn dieser Tag vor heute liegt
      pub ist_vergangen: bool,
      /// Liste der an diesem Tag geplanten Rezepte.
      pub rezepte: Vec<WochentagesEintragItem>,
  }
  ```
- [ ] Sicherstellen, dass `datum_anzeige` komplett entfernt wird — alle Verwendungsstellen prüfen
  (nur in `src/routes/wochenvorschau.rs` und `templates/wochenvorschau.html`)

**Reihenfolge:** Struct erst ändern wenn der Unit-Test rot ist, dann grün machen.

---

### Schritt 2: Hilfsfunktionen in `src/routes/wochenvorschau.rs` anpassen (TDD)

**Ziel:** `format_day_display` aufteilen und `ist_heute`/`ist_vergangen` berechnen.

- [ ] Unit-Test `format_weekday_name_returns_correct_name` schreiben — prüft `format_weekday_name(date)` für
  alle 7 Wochentage (dieser Test wird sofort rot, da die Funktion noch nicht existiert)
- [ ] Unit-Test `format_date_kurz_formats_correctly` schreiben — prüft `format_date_kurz(date)` für
  "30. März" und "5. April" (einstellige und zweistellige Tage, Monatswechsel)
- [ ] Neue Hilfsfunktion `format_weekday_name(date: time::Date) -> String` implementieren:
  - Gibt nur den deutschen Wochentag-Namen zurück: "Montag", "Dienstag", …
  - Nutzt die bestehende `GERMAN_WEEKDAYS_LONG`-Konstante
  - Gibt `String` (owned) zurück für Konsistenz mit `datum_kurz`
- [ ] Neue Hilfsfunktion `format_date_kurz(date: time::Date) -> String` implementieren:
  - Gibt "T. Monatsname" zurück, z.B. "30. März" oder "5. April"
  - Nutzt die bestehende `GERMAN_MONTHS_LONG`-Konstante
  - Kein Wochentag-Name, kein Jahr (ist sekundäre Information)
- [ ] Die bisherige `format_day_display`-Funktion und ihre Tests:
  - Die Funktion wird nicht mehr im Handler genutzt — sie kann entfernt werden
  - Die bestehenden Tests `format_day_display_formats_correctly` und `format_day_display_sunday` müssen
    auf die neue Funktion `format_date_kurz` umgeschrieben werden (Erwartungswert ohne Wochentag-Präfix)
- [ ] Unit-Tests `german_weekday_long_returns_correct_names` bleiben unverändert
- [ ] Tests auf grün bringen

---

### Schritt 3: Handler in `src/routes/wochenvorschau.rs` anpassen (TDD)

**Ziel:** Den `.map(|datum| Wochentag { ... })`-Block im Handler auf die neuen Felder umstellen.

- [ ] Rust-Integrationstest `wochenvorschau_heute_hat_css_klasse` vorbereiten (neu in `tests/wochenvorschau.rs`):
  - Given: App, kein Rezept
  - When: GET /wochenvorschau
  - Then: Body enthält `wochentag-heute` (die CSS-Klasse)
  - Test zuerst schreiben — er wird rot, da das Template und der Handler noch nicht die Klasse ausgeben
- [ ] Rust-Integrationstest `wochenvorschau_vergangene_tage_haben_css_klasse` vorbereiten:
  - Given: App, kein Rezept, heute nicht Montag (falls Montag: nur zukünftige Tage testen)
  - When: GET /wochenvorschau
  - Then: Body enthält `wochentag-vergangen` (sofern heute nicht Montag)
  - Hinweis: Dieser Test ist tagabhängig. Wenn heute Montag ist, gibt es keine vergangenen Tage —
    dann prueft der Test nur, dass kein `wochentag-vergangen` vorkommt.
    Besser: Montag hat `ist_vergangen=false`, alle Tage vor heute haben `ist_vergangen=true`.
    Den Test so formulieren, dass er die `ist_vergangen`-Logik auf Unit-Ebene abdeckt (nicht E2E).
- [ ] Rust-Integrationstest `wochenvorschau_zeigt_wochentag_name_und_datum_getrennt` schreiben:
  - Then: Body enthält `wochentag-name` und `wochentag-datum` als CSS-Klassen (nach Template-Anpassung)
- [ ] Handler-Block auf neue Felder umstellen:
  ```rust
  let tage: Vec<Wochentag> = (0..7)
      .map(|i| monday + time::Duration::days(i))
      .map(|datum| {
          let rezepte = recipes
              .iter()
              .filter(|r| r.planned_date == Some(datum))
              .map(|r| WochentagesEintragItem { id: r.id, title: r.title.clone() })
              .collect();
          Wochentag {
              wochentag_name: format_weekday_name(datum),
              datum_kurz: format_date_kurz(datum),
              ist_heute: datum == today,
              ist_vergangen: datum < today,
              rezepte,
          }
      })
      .collect();
  ```
- [ ] `cargo build` → Compilerfehler wegen `datum_anzeige` in Template — Template noch nicht angepasst,
  daher als nächstes Schritt 4 durchführen
- [ ] Alle bestehenden Integrationstests in `tests/wochenvorschau.rs` weiter grün halten
  (die Tests prüfen Inhalt per Textsuche, nicht Struct-Felder — sie sind weitgehend stabil)

---

### Schritt 4: Template `templates/wochenvorschau.html` anpassen (TDD: Compile-Test)

**Ziel:** Das Template nutzt die neuen Felder und rendert bedingte CSS-Klassen.

- [ ] `<div class="wochentag-abschnitt ...">` — bedingte CSS-Klassen ergänzen:
  ```html
  <div class="wochentag-abschnitt
    {%- if tag.ist_heute %} wochentag-heute{% endif %}
    {%- if tag.ist_vergangen %} wochentag-vergangen{% endif %}
    {%- if tag.rezepte.is_empty() %} wochentag-leer{% endif %}">
  ```
  Hinweis: `wochentag-leer` und `wochentag-heute`/`wochentag-vergangen` können kombiniert auftreten
  (heutiger Tag ohne Rezept). Das ist korrekt — CSS-Klassen addieren sich.
- [ ] `<dt class="wochentag-titel">` — strukturierte Ausgabe:
  ```html
  <dt class="wochentag-titel">
      <strong class="wochentag-name">{{ tag.wochentag_name }}</strong>
      <span class="wochentag-datum">{{ tag.datum_kurz }}</span>
  </dt>
  ```
  Begründung: `<strong>` für den Namen erfüllt WCAG 1.4.1 (nicht nur Farbe), `<span>` für das Datum
- [ ] Optionales "Heute"-Badge für Barrierefreiheit ergänzen (Lösung der offenen Frage aus story.md):
  ```html
  {% if tag.ist_heute %}
  <span class="heute-badge" aria-label="Heute">Heute</span>
  {% endif %}
  ```
  Das Badge erscheint innerhalb des `<dt>` nach dem Datum-Span. Es ist für Screenreader hilfreich
  und erfüllt WCAG 1.4.1 (nicht nur durch Farbe erkennbar).
- [ ] `cargo build` → sollte kompilieren (Askama prüft Template zur Compile-Zeit)
- [ ] Integrationstests erneut laufen lassen: `cargo test` → alle grün

---

### Schritt 5: CSS in `src/static/css/app.css` erweitern

**Ziel:** Neue CSS-Klassen für heutigen Tag, vergangene Tage, Wochentag-Name und Datum hinzufügen.

- [ ] Neue Klassen in den bestehenden `/* === Wochenvorschau === */`-Block einfügen (nach `.wochentag-leer`):

  ```css
  /* Heutiger Wochentag — hervorgehoben */
  .wochentag-heute {
      border-color: var(--primary-color);
      border-width: 2px;
      background-color: #eff6ff; /* leicht blaues Hintergrundhighlight */
  }

  /* Vergangene Wochentage — visuell gedämpft */
  .wochentag-vergangen {
      opacity: 0.6;
  }

  /* Wochentag-Name: primär, groß, fett */
  .wochentag-name {
      font-size: 1.1rem;
      font-weight: 700;
      margin-right: 0.5rem;
  }

  /* Kurzdatum: sekundär, kleiner, schwächer */
  .wochentag-datum {
      font-size: 0.875rem;
      font-weight: 400;
      color: #6b7280;
  }

  /* Heute-Badge */
  .heute-badge {
      display: inline-block;
      font-size: 0.75rem;
      font-weight: 600;
      color: var(--primary-color);
      background-color: #dbeafe;
      border-radius: 0.25rem;
      padding: 0 0.375rem;
      margin-left: 0.5rem;
      vertical-align: middle;
  }
  ```

- [ ] Sicherstellen, dass `.wochentag-titel` weiterhin korrekt greift (font-weight und font-size können
  jetzt von den Kind-Elementen übernommen werden — `.wochentag-titel` behält nur `margin-bottom`)
- [ ] Responsiveness prüfen: auf kleinen Bildschirmen (< 400px) soll Name und Datum weiterhin lesbar
  übereinander umbrechen (kein `white-space: nowrap`)

---

### Schritt 6: Rust-Unit-Tests vervollständigen und bereinigen

**Ziel:** Alle Unit-Tests grün, keine toten Tests.

- [ ] Unit-Tests für `format_weekday_name` — alle 7 Wochentage
- [ ] Unit-Tests für `format_date_kurz`:
  - Einstelliger Tag: `make_date(2026, 4, 5)` → `"5. April"`
  - Zweistelliger Tag: `make_date(2026, 3, 30)` → `"30. März"`
  - Jahreswechsel-Monat: `make_date(2026, 1, 1)` → `"1. Januar"`
- [ ] Bestehende Tests `format_day_display_*` umbenennen/entfernen (Funktion existiert nicht mehr)
- [ ] `cargo test` → alle Tests grün
- [ ] `cargo clippy -- -D warnings` → keine Warnungen
- [ ] `cargo fmt --check` → korrekt formatiert

---

### Schritt 7: Rust-Integrationstests in `tests/wochenvorschau.rs` ergänzen

**Ziel:** Neue Testfälle für Story-19-Anforderungen, bestehende Tests bleiben grün.

- [ ] Test `wochenvorschau_hat_css_klasse_wochentag_heute`:
  ```rust
  // Given: App ohne Rezepte
  // When: GET /wochenvorschau
  // Then: Body enthält "wochentag-heute" (Klasse für heutigen Tag)
  assert!(body.contains("wochentag-heute"), ...);
  ```
- [ ] Test `wochenvorschau_hat_css_klasse_wochentag_name`:
  ```rust
  // Then: Body enthält class="wochentag-name" (Klasse für Wochentag-Name-Span)
  assert!(body.contains("wochentag-name"), ...);
  ```
- [ ] Test `wochenvorschau_hat_css_klasse_wochentag_datum`:
  ```rust
  // Then: Body enthält class="wochentag-datum" (Klasse für Datum-Span)
  assert!(body.contains("wochentag-datum"), ...);
  ```
- [ ] Test `wochenvorschau_heute_tag_enthaelt_heute_badge`:
  ```rust
  // Then: Body enthält "Heute" (Badge-Text)
  assert!(body.contains("Heute"), ...);
  ```
- [ ] Test `wochenvorschau_vergangene_tage_korrekt_wenn_nicht_montag`:
  - Berechnet ob heute Montag ist. Falls nicht, dann muss `wochentag-vergangen` im Body vorkommen.
  - Falls heute Montag ist, darf `wochentag-vergangen` NICHT im Body vorkommen.
  - Implementierung über `today.weekday() == time::Weekday::Monday`-Prüfung im Test
- [ ] Alle bestehenden Story-18-Tests weiterhin grün: `cargo test` ausführen

---

### Schritt 8: E2E-Tests in `tests/e2e/wochenvorschau.spec.ts` ergänzen

**Ziel:** Playwright-Tests für alle Akzeptanzkriterien aus Story 19.

Die bestehenden Story-18-Tests bleiben unverändert. Die neuen Tests werden am Ende der
`test.describe('Wochenvorschau (Story 18)', ...)` Suite hinzugefügt (oder in einer neuen
`describe`-Gruppe `'Wochenvorschau Formatierung (Story 19)'` für Klarheit).

- [ ] Test für **K1: Wochentag-Name und Datum getrennt dargestellt**:
  ```typescript
  // Given: Die Wochenvorschau ist geöffnet
  await page.goto('/wochenvorschau');
  // Then: span.wochentag-name Elemente sichtbar (7 Stück)
  const nameElems = page.locator('span.wochentag-name');
  await expect(nameElems).toHaveCount(7);
  // And: Wochentag-Namen sind korrekt
  await expect(page.locator('body')).toContainText('Montag');
  // And: span.wochentag-datum Elemente sichtbar (7 Stück)
  const datumElems = page.locator('span.wochentag-datum');
  await expect(datumElems).toHaveCount(7);
  ```
- [ ] Test für **K2: Heutiger Tag hat CSS-Klasse `wochentag-heute`**:
  ```typescript
  // Given: /wochenvorschau
  await page.goto('/wochenvorschau');
  // Then: Genau ein Element mit Klasse wochentag-heute
  const heuteElems = page.locator('.wochentag-heute');
  await expect(heuteElems).toHaveCount(1);
  // And: Kein anderes Element hat wochentag-heute
  ```
- [ ] Test für **K2: Heute-Badge sichtbar und nicht nur per Farbe**:
  ```typescript
  // Then: .heute-badge mit Text "Heute" sichtbar
  const badge = page.locator('.heute-badge');
  await expect(badge).toBeVisible();
  await expect(badge).toContainText('Heute');
  ```
- [ ] Test für **K3: Vergangene Tage haben CSS-Klasse `wochentag-vergangen`**:
  ```typescript
  // Given: heute ist nicht Montag (sonst keine vergangenen Tage)
  // When: /wochenvorschau aufgerufen
  await page.goto('/wochenvorschau');
  // Then: Mindestens 0 Elemente mit wochentag-vergangen (0 wenn heute Montag)
  // Wochentag-Index berechnen im Test
  const today = new Date();
  const dayOfWeek = today.getDay(); // 0=So, 1=Mo, …
  const daysFromMonday = dayOfWeek === 0 ? 6 : dayOfWeek - 1;
  // Anzahl vergangener Tage = Offset des heutigen Tags
  await expect(page.locator('.wochentag-vergangen')).toHaveCount(daysFromMonday);
  ```
- [ ] Test für **K4: Alle Story-18-Kriterien bleiben erfüllt** (Smoke-Test):
  ```typescript
  // Erstellt Rezept für heute
  const suffix = Date.now();
  const title = `Story19-Smoke-${suffix}`;
  const todayDate = currentWeekDateFromMonday(daysFromMonday); // heute
  await createRecipeWithDate(page, title, ['Mittagessen'], todayDate);
  await page.goto('/wochenvorschau');
  // Rezept unter heutigem Tag sichtbar
  await expect(page.locator('.wochentag-heute')).toContainText(title);
  // Link zur Detailansicht vorhanden
  await expect(page.locator(`.wochentag-heute a:has-text("${title}")`)).toBeVisible();
  ```
- [ ] `npm run test:e2e` → alle Tests grün

---

### Schritt 9: Qualitätschecks (Definition of Done)

- [ ] `cargo build` → keine Fehler, keine Warnungen
- [ ] `cargo clippy -- -D warnings` → keine Clippy-Warnungen
- [ ] `cargo fmt --check` → korrekt formatiert
- [ ] `cargo test` → alle Unit- und Integrationstests grün
- [ ] `npm run test:e2e` → alle Playwright-Tests grün
- [ ] Manuelle Prüfung im Browser (Story-19-Checkliste):
  - [ ] Heutiger Tag ist optisch klar hervorgehoben (Rahmen + Hintergrund)
  - [ ] Wochentag-Name ist größer/fetter als das Datum
  - [ ] "Heute"-Badge neben dem Namen sichtbar
  - [ ] Vergangene Tage sind gedämpft (geringere Opacity)
  - [ ] Rezeptnamen weiterhin klickbar
  - [ ] "Nichts geplant" bei leeren Tagen
  - [ ] KW-Anzeige im Header korrekt

---

## URL-Struktur

```
GET  /wochenvorschau  →  Wochenvorschau mit formatierter Tagesdarstellung (unveränderter Endpunkt)
```

Keine neuen Endpunkte nötig.

---

## Abhängigkeiten

- Story 18 ist vollständig abgeschlossen (alle Tests grün, alle Felder vorhanden)
- Story 28 (planned_date) ist implementiert — `planned_date`-Feld in DB und Formularen vorhanden
- `time`-Crate ist bereits eingebunden (für Datums-Arithmetik)
- Keine neuen externen Abhängigkeiten

### Betroffene Dateien (vollständige Liste)

| Datei | Art der Änderung |
|-------|-----------------|
| `src/templates.rs` | `Wochentag`-Struct: `datum_anzeige` → `wochentag_name` + `datum_kurz` + `ist_heute` + `ist_vergangen` |
| `src/routes/wochenvorschau.rs` | `format_day_display` entfernen, `format_weekday_name` + `format_date_kurz` hinzufügen; Handler-Mapping anpassen; Unit-Tests aktualisieren |
| `templates/wochenvorschau.html` | `<dt>` strukturieren; bedingte CSS-Klassen am `<div>` |
| `src/static/css/app.css` | Neue Klassen: `.wochentag-heute`, `.wochentag-vergangen`, `.wochentag-name`, `.wochentag-datum`, `.heute-badge` |
| `tests/wochenvorschau.rs` | Neue Integrationstests für Story-19-Anforderungen |
| `tests/e2e/wochenvorschau.spec.ts` | Neue E2E-Tests für K1–K4 |

---

## Test-Checkliste

- [ ] Unit-Test: `format_weekday_name` — alle 7 Wochentage
- [ ] Unit-Test: `format_date_kurz` — einstelliger Tag, zweistelliger Tag, Jahresanfang
- [ ] Integrationstest: `wochenvorschau_hat_css_klasse_wochentag_heute` — prüft CSS-Klasse im HTML-Body
- [ ] Integrationstest: `wochenvorschau_hat_css_klasse_wochentag_name` — prüft Klasse im HTML
- [ ] Integrationstest: `wochenvorschau_hat_css_klasse_wochentag_datum` — prüft Klasse im HTML
- [ ] Integrationstest: `wochenvorschau_heute_tag_enthaelt_heute_badge` — prüft "Heute"-Text im HTML
- [ ] Integrationstest: `wochenvorschau_vergangene_tage_korrekt_wenn_nicht_montag` — tagabhängige Logik
- [ ] E2E-Test: K1 — 7x `span.wochentag-name` und 7x `span.wochentag-datum` sichtbar
- [ ] E2E-Test: K2 — genau 1x `.wochentag-heute`, `.heute-badge` mit Text "Heute"
- [ ] E2E-Test: K3 — Anzahl `.wochentag-vergangen` == Anzahl Tage seit Montag
- [ ] E2E-Test: K4 — Smoke-Test: Rezept für heute erscheint unter `.wochentag-heute`
- [ ] Manueller Test: Visueller Check im Browser (Hervorhebung, Gedämpfung, Lesbarkeit)

---

## Offene Punkte

- **"Heute"-Badge:** Die story.md formuliert dies als "Empfehlung: optional". Die Entscheidung hier:
  Das Badge wird implementiert, weil es WCAG 1.4.1 (nicht nur per Farbe) ohne Aufwand erfüllt.
  Es kann via CSS auf sehr kleinen Screens ausgeblendet werden, falls es den Platz stört.

- **Kombination `wochentag-heute` + `wochentag-leer`:** Wenn heute kein Rezept geplant ist,
  erhält der `<div>` beide Klassen. Das ist beabsichtigt und korrekt: der heutige Tag soll auch
  dann hervorgehoben sein, wenn er leer ist. CSS regelt das Aussehen sinnvoll über Spezifität
  (`.wochentag-heute` hat Vorrang vor `.wochentag-leer`).

- **`wochentag-vergangen` auf Montag (heute = Montag):** Wenn heute Montag ist, gibt es keine
  vergangenen Tage in dieser Woche. Der Integrationstest muss darauf eingehen (tagabhängige Logik).
  Lösung: Der Test prüft per `is_monday`-Flag die erwartete Anzahl.
