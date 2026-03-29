# Implementierungsplan: Story 06 - Responsive Layout für Desktop und Mobile

## Technische Analyse des IST-Zustands

### CSS (`src/static/css/app.css`)

Das bestehende CSS hat bereits einige responsive Grundlagen, aber keinen konsequenten Mobile-First-Ansatz:

**Vorhandene Stärken:**
- `max-width: 800px` auf `main` (Desktop-Begrenzung vorhanden)
- `box-sizing: border-box` global gesetzt
- `flex-wrap: wrap` an einigen Stellen
- Zwei `@media (max-width: 600px)` Blöcke (einer für Detail-Buttons, einer für Suchfeld)
- `min-height: 44px` auf `.recipe-item-link` und `.search-input-group`-Elementen
- `font-size: 1rem` auf Inputs und Textareas

**Schwachstellen / Fehlende Punkte (IST vs. SOLL):**
- **Desktop-first statt Mobile-First:** Bestehende Media Queries verwenden `max-width` (Desktop-first), nicht `min-width` (Mobile-first)
- **Kein viewport-Meta in base.html – FALSCH:** Das Meta-Tag ist bereits vorhanden (`<meta name="viewport" content="width=device-width, initial-scale=1.0">`) – kein Handlungsbedarf
- **Header ohne Navigation:** Der `<header>` enthält nur `<span class="site-title">Rezepte</span>` – kein Navigations-Link zur Startseite, keine strukturierte Nav
- **Schriftgrößen in `px` statt `rem/em`:** `h1` ist in `rem` (gut), aber `.category-tag`, `.btn-small`, `.recipe-date`, `.meta` nutzen `0.875rem` – akzeptabel, aber `h1: 2rem` könnte auf Mobile zu groß sein
- **Header-Padding:** `padding: 1rem 2rem` ist fest – auf Mobile sollte dies kleiner sein
- **`main`-Padding:** `padding: 0 1rem` ist gut für Mobile, aber für Tablet/Desktop sollte mehr Padding vorhanden sein
- **Buttons:** `.btn-primary`, `.btn-secondary`, `.btn-danger` haben kein explizites `min-height: 44px`
- **`confirm-delete` und `confirm-actions`:** Auf Mobile werden Buttons eventuell zu schmal oder nebeneinander zu eng
- **Kategorie-Filter:** `min-height: 36px` (unter dem 44px WCAG-Minimum)
- **`sort-filter-btn`:** `min-height: 36px` (unter dem 44px WCAG-Minimum)
- **Kein max-width auf breiten Bildschirmen (>1024px):** `max-width: 800px` für `main` ist zu eng für Desktop; laut Story sollen 960px zentriert werden
- **`form-actions`:** Kein responsive Stacking für Mobile definiert (Buttons stehen horizontal, könnten auf schmalen Displays zu eng werden)
- **`confirm-actions`:** Kein Mobile-Stacking

### Templates (`templates/`)

**`base.html`:**
- Header enthält nur `<span class="site-title">Rezepte</span>` – kein `<a href="/">`, keine Navigation

**`index.html`:**
- Gute Struktur; Filter-Nav, Suchfeld, Rezeptliste vorhanden
- Listeneinträge haben `.recipe-item-link` mit `min-height: 44px` – gut

**`recipes/form.html`:**
- Einspaltiger Fluss (gut für Mobile)
- Keine expliziten Touch-Target-Größen für Labels/Checkboxen

**`recipes/detail.html`:**
- `.actions`-Div mit `flex-wrap: wrap` – grundsätzlich ok
- `pre`-Elemente (Zutaten/Anleitung): `white-space: pre-wrap` gesetzt – gut, kein horizontales Scrollen

**`recipes/confirm_delete.html`:**
- `.confirm-actions` ohne Mobile-Stacking-Fallback

### Playwright-Tests

Die bestehenden Tests laufen alle mit dem Default-Viewport des `Desktop Chrome`-Projekts (`1280×720px`). Responsive Tests mit abweichenden Viewports existieren noch nicht.

---

## Implementierungsschritte

### Schritt 1: CSS – Mobile-First Basis und Breakpoints definieren

- [ ] CSS komplett auf Mobile-First umstrukturieren: Basis-Styles gelten für Mobile (≤767px), Media Queries erweitern für größere Viewports
- [ ] Breakpoints klar als Kommentar-Blöcke im CSS einführen:
  ```
  /* === Mobile: Basis (bis 767px) === */
  /* === Tablet: ab 768px === */
  @media (min-width: 768px) { ... }
  /* === Desktop: ab 1024px === */
  @media (min-width: 1024px) { ... }
  ```
- [ ] `main`: Basis-Padding auf `0 1rem` (Mobile, bereits korrekt), auf Tablet `0 2rem`, auf Desktop `0 1.5rem` mit `max-width: 960px`
- [ ] `header`: Basis-Padding auf `1rem` (Mobile), ab Tablet `1rem 2rem`; `site-title` als klickbaren Link (`<a href="/">`) stylen

### Schritt 2: CSS – Touch-Targets auf 44px anheben (WCAG 2.1)

- [ ] `.btn-primary`, `.btn-secondary`, `.btn-danger`: `min-height: 44px` hinzufügen
- [ ] `.category-filter-btn`: `min-height` von `36px` auf `44px` anheben
- [ ] `.sort-filter-btn`: `min-height` von `36px` auf `44px` anheben
- [ ] `.checkbox-label`: `min-height: 44px` und `padding: 0.5rem 0` für komfortable Tap-Ziele
- [ ] Fokus-Indikatoren für alle interaktiven Elemente sicherstellen (`:focus-visible` mit `outline`)

### Schritt 3: CSS – Mobile-Layout für Header und Navigation

- [ ] `.site-title` als `<a>`-Tag in `base.html` gestalten (Farbe, kein Underline)
- [ ] Header: auf Mobile `display: flex; align-items: center; justify-content: space-between`
- [ ] Header: "Neues Rezept"-Link in der Header-Nav nur auf Mobile kompakt anzeigen (Icon only oder kleiner Button), da er auf der Hauptseite bereits als Button vorhanden ist – **ENTSCHEIDUNG:** Navigation bleibt minimal (nur App-Name als Link); der "Neues Rezept"-Button bleibt in der Seitenstruktur, nicht im Header

### Schritt 4: CSS – Formular auf Mobile (form.html)

- [ ] `input[type="text"]`, `textarea`, `input[type="search"]`: Bereits `width: 100%` und `font-size: 1rem` – prüfen und ggf. `min-height: 44px` auf `input[type="text"]` setzen
- [ ] `.form-actions`: Auf Mobile `flex-direction: column; gap: 1rem` damit Buttons untereinander stehen (volle Breite)
- [ ] `.btn-secondary` in `.form-actions`: `margin-left: 0` auf Mobile (derzeit `margin-left: 1rem`)
- [ ] `.date-input-group`: Sicherstellen, dass Kalender-Button `min-width: 44px; min-height: 44px` hat

### Schritt 5: CSS – Detailansicht auf Mobile (detail.html)

- [ ] `.recipe-detail`: `padding: 1rem` auf Mobile (derzeit `2rem` für alle Viewports); ab Tablet `2rem`
- [ ] `.actions` in `.recipe-detail`: Auf Mobile `flex-direction: column; align-items: stretch` (bereits in `@media (max-width: 600px)` – umstellen auf Mobile-First)
- [ ] `.btn-danger` in `.actions`: `margin-left: 0` auf Mobile
- [ ] `h1`: `font-size: 1.5rem` auf Mobile, `2rem` ab Tablet (weniger Umbrüche bei langen Titeln auf Mobile)

### Schritt 6: CSS – Lösch-Bestätigung auf Mobile (confirm_delete.html)

- [ ] `.confirm-delete`: `padding: 1rem` auf Mobile, `2rem` ab Tablet; `max-width: none` auf Mobile, `600px` ab Tablet (zentriert)
- [ ] `.confirm-actions`: Auf Mobile `flex-direction: column; align-items: stretch` mit `gap: 1rem`
- [ ] `.btn-primary`, `.btn-danger` in `.confirm-actions`: `text-align: center; width: 100%` auf Mobile

### Schritt 7: CSS – Großbildschirm-Begrenzung (Desktop ab 1024px)

- [ ] `main`: `max-width: 960px` (anstatt derzeit `800px`) auf Desktop; auf Tablet `max-width: 720px` oder volle Breite mit mehr Padding
- [ ] Header-Inhalt auf Desktop ebenfalls auf `max-width: 960px` begrenzen und zentrieren (Wrapper-Element)
- [ ] Sicherstellen, dass auf 2560px kein gestreckt-leeres Layout entsteht

### Schritt 8: Template – base.html anpassen

- [ ] `<span class="site-title">Rezepte</span>` → `<a href="/" class="site-title">Rezepte</a>` (App-Name als Link zur Startseite)
- [ ] `<header>` in einen semantisch korrekten Bereich mit `<nav>`-Unterbereich überführen falls Navigation erweitert wird – **für jetzt:** nur site-title als Link

### Schritt 9: Playwright E2E-Tests schreiben (TDD – Tests zuerst)

- [ ] `tests/e2e/responsive-layout.spec.ts` erstellen mit folgenden Tests:

  **Test 1: Rezept-Liste auf Mobile (Testfall 1)**
  - Viewport auf `390×844px` setzen
  - Startseite aufrufen
  - Prüfen: kein horizontales Scrollen (`document.body.scrollWidth <= window.innerWidth`)
  - Prüfen: Listeneintrag mind. 44px hoch

  **Test 2: Rezept-Liste auf Desktop (Testfall 2)**
  - Viewport auf `1280×800px` setzen
  - Startseite aufrufen
  - Prüfen: `main`-Element ist nicht über volle Bildschirmbreite gedehnt (bounding box width < 1280)
  - Prüfen: `main`-Element ist horizontal zentriert (left margin > 0)

  **Test 3: Formular auf Mobile (Testfall 3)**
  - Viewport auf `390×844px` setzen
  - `/recipes/new` aufrufen
  - Prüfen: kein horizontales Scrollen
  - Prüfen: Titel-Input ist mind. 44px breit
  - Prüfen: "Rezept speichern"-Button ist sichtbar und mind. 44px hoch

  **Test 4: Navigation auf Mobile zugänglich (Testfall 4)**
  - Viewport auf `390×844px` setzen
  - Startseite aufrufen
  - Prüfen: `header` ist vollständig sichtbar (kein Clipping)
  - Prüfen: `.site-title`-Link ist sichtbar und hat korrekten href="/"

  **Test 5: Rezept-Detailansicht auf Mobile (Testfall 5)**
  - Rezept erstellen, dann Detailseite mit Viewport `390×844px` aufrufen
  - Prüfen: kein horizontales Scrollen
  - Prüfen: Aktionsbuttons (Bearbeiten, Zurück, Löschen) sind min. 44px hoch

### Schritt 10: Verifikation und Qualitätssicherung

- [ ] `cargo build` – keine Compiler-Fehler oder Warnungen
- [ ] `cargo clippy -- -D warnings` – keine Clippy-Warnungen
- [ ] `cargo fmt --check` – korrekt formatiert
- [ ] CSS-Datei auf Größe prüfen: unter 20 KB unkomprimiert
- [ ] `npm run test:e2e` – alle Tests bestehen (bestehende + neue responsive Tests)
- [ ] Manueller Test: Seiten im Browser auf 320px, 390px, 768px, 1024px, 1280px, 2560px überprüfen

---

## TDD-Vorgehensweise

Die Implementierung folgt dem TDD-Pattern:

1. **Schritt 9 zuerst:** Responsive Playwright-Tests schreiben, die alle FEHLSCHLAGEN (weil CSS noch nicht angepasst ist)
2. **Schritte 1–8 umsetzen:** CSS und Templates anpassen, bis alle Tests grün werden
3. **Schritt 10:** Refactoring, Aufräumen, finale Verifikation

**Begründung:** Da diese Story ausschließlich CSS/HTML betrifft und keine Rust-Logik, sind Playwright-E2E-Tests die primäre Test-Ebene. Unit-Tests auf Rust-Ebene sind nicht notwendig (keine neue Backend-Logik).

---

## URL-Struktur

Keine Änderungen an URLs. Diese Story betrifft ausschließlich Präsentation (CSS + HTML-Templates).

---

## Abhängigkeiten

- Stories 01–05 sind implementiert (Rezept-Liste, Detail, Formular, Suche, Filter existieren bereits)
- Keine neuen technischen Abhängigkeiten
- Bestehende Rust-Strukturen (`templates.rs`, `routes/recipes.rs`) bleiben unverändert

---

## Test-Checkliste

- [ ] E2E-Test: Rezept-Liste auf Mobile – kein horizontales Scrollen, Listeneinträge ≥44px hoch
- [ ] E2E-Test: Rezept-Liste auf Desktop – `main` nicht über volle Bildschirmbreite, horizontal zentriert
- [ ] E2E-Test: Formular auf Mobile – kein horizontales Scrollen, Felder ≥44px, Speichern-Button sichtbar
- [ ] E2E-Test: Navigation auf Mobile – Header vollständig sichtbar, site-title-Link korrekt
- [ ] E2E-Test: Detailansicht auf Mobile – kein horizontales Scrollen, Aktionsbuttons ≥44px hoch
- [ ] Manueller Test: Breakpoint-Übergänge bei 768px und 1024px fließend
- [ ] Manueller Test: Orientierungswechsel (Portrait ↔ Landscape) auf Smartphone-Viewport
- [ ] Manueller Test: Systemschriftgröße vergrößert – Layout bricht nicht

---

## Offene Punkte / Entscheidungen

- **Navigation:** CSS-only kompakte Leiste (KEINE Hamburger-Menü-Implementierung mit JS). Der App-Name wird als Link zur Startseite ausgebaut; die Navigation bleibt horizontal und kompakt auf allen Viewports.
- **Formulare Desktop:** Einheitlich einspaltig (kein 2-Spalten-Layout für Labels/Felder). Einfacher, konsistenter, leichter wartbar.
- **`main`-Maximalbreite:** 960px auf Desktop (laut Story: 900–1200px); auf Tablet volle Breite mit 32px Padding (kein festes max-width).
- **Bestehende `@media (max-width: ...)` Blöcke:** Diese werden in Mobile-First-Regeln umgeschrieben (Basis = Mobile, `min-width`-Queries für größere Viewports).
