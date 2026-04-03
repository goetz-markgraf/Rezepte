# Implementierungsplan: Story 37 - Einklappen der Filter

## Technische Schritte

### Schritt 1: Query-Parameter-Parsing im Handler (`src/routes/recipes.rs`)

- [ ] `IndexQuery`-Struct um Feld `filter_collapsed: Option<String>` erweitern
- [ ] Im `index`-Handler: `filter_collapsed`-Parameter auslesen und in `bool` umwandeln
      (`"1"` → `true`, alles andere → `false`)
- [ ] Hilfsfunktion `build_filter_collapsed_toggle_url(...)` schreiben:
      - Eingeklappt (`true`) → gibt URL ohne `filter_collapsed`-Parameter zurück
      - Ausgeklappt (`false`) → gibt URL mit `filter_collapsed=1` zurück
      - Alle anderen aktiven Parameter (q, kategorie, filter, bewertung) bleiben erhalten
- [ ] Unit-Test: `build_filter_collapsed_toggle_url` mit und ohne bestehende Parameter

### Schritt 2: Template-Datenmodell (`src/templates.rs`)

- [ ] `IndexTemplate`-Struct um zwei Felder erweitern:
  - `filter_collapsed: bool` — ob der Filterbereich eingeklappt ist
  - `filter_collapsed_toggle_url: String` — URL für den Toggle-Button
- [ ] Im `index`-Handler die neuen Felder befüllen und an das Template übergeben
- [ ] Unit-Test: Korrekte Weitergabe des `filter_collapsed`-Flags im Handler

### Schritt 3: HTML-Template anpassen (`templates/index.html`)

- [ ] Toggle-Button zwischen Suchformular und Filterbereich einfügen:
  ```html
  <a href="{{ filter_collapsed_toggle_url }}"
     class="filter-toggle-btn"
     aria-expanded="{{ !filter_collapsed }}"
     aria-controls="filter-panel">
    {% if filter_collapsed %}
      Filter ▶{% if any_filter_active %} <span class="filter-active-indicator">(aktiv)</span>{% endif %}
    {% else %}
      Filter ▼
    {% endif %}
  </a>
  ```
- [ ] Alle Filterbereiche (`category-filter`, `sort-filter`, `saved-filters`, `save-filter-area`)
      in ein `<div id="filter-panel">` zusammenfassen
- [ ] Bei `filter_collapsed == true`: CSS-Klasse `filter-panel--collapsed` an das `<div>` setzen
- [ ] `aria-hidden`-Attribut am `filter-panel` korrekt setzen: `"true"` wenn eingeklappt
- [ ] Suchformular bleibt **außerhalb** des `filter-panel` (unverändert, immer sichtbar)

### Schritt 4: CSS-Styles (`src/static/css/app.css`)

- [ ] `.filter-toggle-btn` — schlanker Button-Stil, ähnlich den bestehenden `sort-filter-btn`
- [ ] `.filter-panel--collapsed` — blendet den Filterbereich aus (`display: none`)
- [ ] `.filter-active-indicator` — visuelle Markierung (z.B. fett oder farbiger Text)
- [ ] Fokus-Indikator für `.filter-toggle-btn` sichtbar (Barrierefreiheit)

### Schritt 5: Toggle-URL in bestehenden URL-Builder-Funktionen berücksichtigen

- [ ] `build_category_toggle_url`: `filter_collapsed`-Parameter **nicht** übernehmen
      (nach Klick auf Kategorie soll der Zustand ausgeklappt sein — klarer Benutzerfluss)
- [ ] `build_not_made_toggle_url`: ebenso, kein `filter_collapsed`
- [ ] `build_next_seven_days_toggle_url`: ebenso
- [ ] `build_bewertung_toggle_url`: ebenso
- [ ] `build_current_query_string`: `filter_collapsed` **nicht** im gespeicherten Filter speichern
      (K7: gespeicherte Filter öffnen immer ausgeklappt)

> **Begründung:** Filterwechsel impliziert Interesse an den Filtereinstellungen → immer ausgeklappt anzeigen.
> Nur der dedizierte Toggle-Button steuert den eingeklappten Zustand.

### Schritt 6: Integrations-Tests (`tests/`)

- [ ] `tests/recipe_filter_collapse.rs` erstellen:
  - Test 1 (Given/When/Then): Seite mit `?filter_collapsed=1` liefert HTML mit `filter-panel--collapsed`
  - Test 2: Seite ohne Parameter liefert HTML **ohne** `filter-panel--collapsed`
  - Test 3: `any_filter_active=true` + `filter_collapsed=true` → `filter-active-indicator` im HTML
  - Test 4: `build_filter_collapsed_toggle_url` hin- und herschaltet korrekt
- [ ] `cargo test` läuft durch

### Schritt 7: E2E-Tests (`tests/e2e/filter-collapse.spec.ts`)

- [ ] Testfall 1 (K2, K4): Filter einklappen
  ```
  // Gegeben: Startseite ist geöffnet, Filterbereich ist sichtbar
  // Wenn: Nutzer klickt auf Toggle-Button "Filter ▼"
  // Dann: filter-panel ist nicht sichtbar
  // Und:  URL enthält filter_collapsed=1
  // Und:  Suchformular ist weiterhin sichtbar
  ```
- [ ] Testfall 2 (K3, K4): Filter ausklappen
  ```
  // Gegeben: Startseite mit ?filter_collapsed=1 aufgerufen
  // Dann:    filter-panel ist ausgeblendet
  // Wenn:    Nutzer klickt Toggle-Button "Filter ▶"
  // Dann:    filter-panel ist wieder sichtbar
  // Und:     URL enthält keinen filter_collapsed=1-Parameter
  ```
- [ ] Testfall 3 (K5): Zustand via URL beim Seitenaufruf
  ```
  // Gegeben: URL mit ?filter_collapsed=1 wird direkt aufgerufen
  // Dann:    Filterbereich ist von Anfang an ausgeblendet
  // Und:     Toggle-Button zeigt "Filter ▶"
  ```
- [ ] Testfall 4 (K6): Aktive Filter sichtbar bei eingeklapptem Zustand
  ```
  // Gegeben: Kategorie "Brot" ist aktiv, Seite mit ?filter_collapsed=1 aufgerufen
  // Dann:    Toggle-Button zeigt Hinweis "(aktiv)"
  ```
- [ ] Testfall 5 (K7): Gespeicherte Filter öffnen ausgeklappt
  ```
  // Gegeben: Ein gespeicherter Filter existiert
  // Wenn:    Nutzer klickt auf gespeicherten Filter
  // Dann:    URL enthält keinen filter_collapsed-Parameter
  // Und:     Filterbereich ist sichtbar
  ```
- [ ] Testfall 6 (K8): Funktioniert ohne JavaScript
  ```
  // Gegeben: JavaScript ist deaktiviert
  // Wenn:    Nutzer klickt Toggle-Link
  // Dann:    Seite lädt neu mit korrektem filter_collapsed-Parameter
  ```
- [ ] `npm run test:e2e` läuft durch

### Schritt 8: Barrierefreiheit & Qualitätssicherung

- [ ] `aria-expanded` am Toggle-Button korrekt gesetzt (`"true"` ausgeklappt, `"false"` eingeklappt)
- [ ] `aria-controls="filter-panel"` am Toggle-Button
- [ ] `id="filter-panel"` am Filterbereich-Container
- [ ] `aria-hidden="true"` am `filter-panel` wenn eingeklappt (Template-Logik)
- [ ] Tastaturnavigation: Toggle per Enter/Space bedienbar (ist bei `<a>`-Element Standard)
- [ ] `cargo clippy -- -D warnings` ohne Warnungen
- [ ] `cargo fmt --check` ok

---

## URL-Struktur

```
GET  /?filter_collapsed=1                           → Filterbereich eingeklappt
GET  /?filter_collapsed=1&kategorie=Brot            → Eingeklappt + aktiver Kategorie-Filter
GET  /?kategorie=Brot                               → Ausgeklappt (Standard) + aktiver Kategorie-Filter
GET  /                                              → Ausgeklappt (Standard)
```

**Kein neuer HTTP-Endpunkt** — der Toggle ist ein einfacher Link, der dieselbe Route `GET /` mit
geändertem `filter_collapsed`-Parameter aufruft.

---

## Betroffene Dateien

| Datei | Art der Änderung |
|-------|-----------------|
| `src/routes/recipes.rs` | `IndexQuery` erweitern, `index`-Handler, neue Hilfsfunktion |
| `src/templates.rs` | `IndexTemplate` um 2 Felder erweitern |
| `templates/index.html` | Toggle-Button + `filter-panel`-Container + CSS-Klasse |
| `src/static/css/app.css` | Styles für Toggle-Button und eingeklappten Zustand |
| `tests/recipe_filter_collapse.rs` | Neue Integrations-Testdatei |
| `tests/e2e/filter-collapse.spec.ts` | Neue E2E-Testdatei |

---

## Abhängigkeiten

- Stories 07, 08, 09, 10, 11, 13 müssen abgeschlossen sein — sind alle abgeschlossen
- Bestehende `IndexTemplate`- und URL-Builder-Funktionen werden wiederverwendet
- Keine Datenbankänderungen, keine neuen Migrationen

---

## Test-Checkliste

- [ ] Unit-Test: `build_filter_collapsed_toggle_url` — false→URL mit `filter_collapsed=1`
- [ ] Unit-Test: `build_filter_collapsed_toggle_url` — true→URL ohne `filter_collapsed`
- [ ] Unit-Test: Parameter q, kategorie, filter, bewertung bleiben bei Toggle erhalten
- [ ] Integrationstest: `GET /?filter_collapsed=1` rendert HTML mit `filter-panel--collapsed`
- [ ] Integrationstest: `GET /` rendert HTML **ohne** `filter-panel--collapsed`
- [ ] Integrationstest: Aktiv-Indikator erscheint bei aktivem Filter + eingeklappt
- [ ] E2E-Test: Filter einklappen (K2, K4) — Suchleiste bleibt sichtbar
- [ ] E2E-Test: Filter ausklappen (K3, K4)
- [ ] E2E-Test: URL-basierter Startzustand (K5)
- [ ] E2E-Test: Aktiver-Filter-Indikator bei eingeklappt (K6)
- [ ] E2E-Test: Gespeicherte Filter öffnen ausgeklappt (K7)
- [ ] E2E-Test: Funktion ohne JavaScript (K8)
- [ ] Manuell: Toggle-Button Fokus-Indikator auf mobilen Browsern prüfen
- [ ] Manuell: `aria-expanded` im Browser-Accessibility-Tree verifizieren

---

## Offene Punkte

- Soll beim Aktivieren eines Kategorie-Filters über den URL-Toggle der `filter_collapsed`-Zustand
  erhalten bleiben? → Aktueller Plan: **nein** (Filterklick impliziert Interesse an Filtern → ausgeklappt).
  Bei anderem Wunsch müssen alle URL-Builder-Funktionen `filter_collapsed` durchreichen.
