# Story 25: WCAG 2.1 Level A Accessibility

**Epic:** Technische Grundlagen
**Priorität:** MVP - Phase 1
**Status:** In Arbeit

---

## 1. Story-Satz

**Als** Benutzer möchte ich **die Rezepte-App barrierefrei nutzen können**, damit ich **alle Funktionen mit Tastatur, Screenreader oder anderen assistiven Technologien zuverlässig bedienen kann**.

---

## 2. Geschäftsbezogene Details

### Kontext

Die gesamte Rezepte-App ist bereits implementiert. Diese Story prüft und verbessert die Barrierefreiheit der bestehenden Implementierung gemäß WCAG 2.1 Level A.

Barrierefreiheit ist im PRD explizit als nicht-funktionale Anforderung definiert (NFR-A1 bis NFR-A4). Sie ist keine optionale Ergänzung, sondern Grundvoraussetzung für eine qualitativ hochwertige Web-Anwendung – selbst in einem LAN-only-Kontext.

WCAG 2.1 Level A umfasst die fundamentalen Anforderungen, ohne die eine Webanwendung für viele Nutzer mit assistiven Technologien nicht nutzbar wäre: semantisches HTML, Tastaturbedienbarkeit, Textalternativen, und ausreichende Kontraste.

Die App nutzt den Tech-Stack Rust + Axum + Askama (Server-Side Rendering) + HTMX + vanilla CSS. SSR-Anwendungen haben prinzipiell gute Voraussetzungen für Barrierefreiheit, da sie kein schweres JavaScript-Framework benötigen – die Qualität hängt aber von der sorgfältigen HTML-Struktur ab.

### Nutzergruppe

- Beide Partner des Haushalts (Anna & Dragon)
- Zukünftige Nutzer die assistive Technologien einsetzen (Screenreader, Tastaturnavigation)
- Nutzer die temporär auf eine Maus verzichten (defekte Maus, Arm-/Handverletzung)
- Nutzer die in anderen Kontexten navigieren (TV-Fernbedienung, Trackpad-only auf einem Tablet)

### Business-Value

- Die App funktioniert zuverlässig auch in ungewöhnlichen Bedienszenarien
- Tastaturnavigation ermöglicht schnelle, mausfreie Bedienung (produktiver für erfahrene Nutzer)
- Semantisches HTML verbessert die Lesbarkeit und Wartbarkeit des Codes
- Konformität mit etablierten Web-Standards als Qualitätsmerkmal

### Edge Cases

- **Bilder ohne Alternativtext:** Icon-Elemente die als `<img>` oder CSS-Hintergrund eingebunden sind, müssen korrekt behandelt werden (leerer `alt=""` für dekorative Bilder, beschreibender `alt` für informative Bilder)
- **HTMX-Interaktionen:** Dynamisch nachgeladene Inhalte via HTMX müssen für Screenreader korrekt angekündigt werden (ARIA-Live-Regions wo nötig)
- **Formularvalidierung:** Fehlermeldungen bei Pflichtfeldern (Titel, Kategorie) müssen programmatisch mit dem Feld verknüpft sein, nicht nur visuell erkennbar
- **Sternebewertung:** Die interaktive Sternebewertung (1-5 Sterne) muss per Tastatur bedienbar sein und den Wert für Screenreader kommunizieren
- **Modaler Dialog (Löschen-Bestätigung):** Das Lösch-Bestätigungs-Popup muss den Fokus korrekt verwalten (Fokus-Falle im Dialog, Rückgabe nach Schließen)
- **Filter-Zurücksetzen:** Der "Alle Filter zurücksetzen"-Button muss für Screenreader verständlich sein
- **HTMX-Suche (Live-Search):** Suchergebnisse die sich dynamisch aktualisieren müssen für Screenreader-Nutzer zugänglich sein

---

## 3. Akzeptanzkriterien

### Funktionale Kriterien

- [ ] **K1: Semantisches HTML in allen Seitenbereichen**
  - Seiten haben eine logische Überschriftenhierarchie (`h1` → `h2` → `h3`), keine Lücken
  - Navigations-Bereiche sind mit `<nav>` und `aria-label` ausgezeichnet
  - Hauptinhalt ist mit `<main>` umschlossen
  - Kopfbereich mit `<header>`, Fußbereich mit `<footer>` (falls vorhanden)
  - Listen (Rezept-Liste, Filterliste) sind als `<ul>`/`<li>` oder `<ol>`/`<li>` strukturiert

- [ ] **K2: Alle Formularfelder haben zugängliche Labels**
  - Jedes `<input>`, `<textarea>`, `<select>` ist mit einem `<label>` (via `for`/`id`) verknüpft
  - Pflichtfelder sind programmatisch als `required` markiert
  - Fehlermeldungen sind via `aria-describedby` oder `aria-errormessage` mit dem Feld verknüpft
  - Kein Feld nutzt nur `placeholder` als Ersatz für ein Label

- [ ] **K3: Vollständige Tastaturnavigation**
  - Alle interaktiven Elemente (Links, Buttons, Formularfelder, Sternebewertung) sind per Tab erreichbar
  - Die Tab-Reihenfolge ist logisch und folgt der visuellen Leserichtung (oben-links nach unten-rechts)
  - Kein interaktives Element ist per Tastatur unerreichbar (kein `tabindex="-1"` ohne guten Grund)
  - Alle Aktionen sind ohne Maus ausführbar: Rezept erstellen, bearbeiten, löschen, Bewertung setzen, Filter anwenden, suchen

- [ ] **K4: Sichtbare Fokus-Indikatoren**
  - Alle fokussierbaren Elemente zeigen einen deutlich sichtbaren Fokus-Indikator (kein `outline: none` ohne Alternative)
  - Der Fokus-Indikator ist auf allen Hintergründen erkennbar (nicht nur auf weißem Hintergrund)

- [ ] **K5: Textalternativen für Nicht-Text-Inhalte**
  - Alle informativen Icons/Bilder haben einen beschreibenden `alt`-Text
  - Dekorative Icons/Bilder haben `alt=""` oder sind via CSS eingebunden
  - Die Sternebewertung kommuniziert den aktuellen Wert textuell (z.B. "3 von 5 Sternen")

- [ ] **K6: Sternebewertung per Tastatur bedienbar**
  - Die Sternebewertung kann per Tastatur gesetzt werden (z.B. via Radio-Buttons oder Buttons mit Pfeiltasten)
  - Der ausgewählte Wert ist für Screenreader lesbar
  - Die Aktion "Bewertung setzen" ist eindeutig beschriftet

- [ ] **K7: Modaler Löschen-Dialog ist zugänglich**
  - Nach Öffnen des Dialogs liegt der Fokus innerhalb des Dialogs (erstes interaktives Element oder Dialog selbst)
  - Fokus bleibt innerhalb des Dialogs (Fokus-Falle)
  - Nach Schließen (Abbrechen oder Bestätigen) kehrt der Fokus zum auslösenden Element zurück
  - Dialog ist mit `role="dialog"` und `aria-labelledby` korrekt ausgezeichnet

- [ ] **K8: Dynamische HTMX-Inhalte für Screenreader zugänglich**
  - Nach einer HTMX-Aktualisierung (Suche, Filter) werden neue Inhalte für Screenreader angekündigt (aria-live oder Fokus-Management)
  - Die Suchergebnisanzahl oder eine vergleichbare Statusmeldung wird mitgeteilt

- [ ] **K9: Links und Buttons sind klar beschriftet**
  - Alle Links haben einen aussagekräftigen Text (kein "Hier klicken", kein "Mehr")
  - Icon-only Buttons haben ein `aria-label` mit der Funktion
  - "Bearbeiten"- und "Löschen"-Buttons auf der Detailseite und in der Liste nennen den Kontext (z.B. `aria-label="Spaghetti Bolognese bearbeiten"`)

- [ ] **K10: Sprache der Seite ist definiert**
  - Das `<html>`-Element hat `lang="de"` gesetzt
  - Fremdsprachige Einschübe (falls vorhanden) sind mit `lang`-Attribut ausgezeichnet

### Nicht-funktionale Kriterien

- [ ] **K11: Farbkontraste erfüllen WCAG 2.1 Level AA für Text**
  - Normaler Text (unter 18pt/14pt fett): Kontrastverhältnis ≥ 4.5:1
  - Großer Text (ab 18pt oder 14pt fett): Kontrastverhältnis ≥ 3:1
  - Interaktive Komponenten und Grafiken: Kontrastverhältnis ≥ 3:1 (Level AA für UI-Komponenten)

- [ ] **K12: Keine Inhalte nur über Farbe kommuniziert**
  - Fehlermarkierungen nutzen nicht nur rote Farbe, sondern auch Text oder Symbol
  - Aktive/inaktive Filter-Zustände sind nicht nur durch Farbe unterscheidbar

---

## 4. Technische Planung

### Datenmodell

Keine Änderungen am Datenbankschema. Diese Story betrifft ausschließlich HTML-Struktur, CSS und ggf. minimale JavaScript-Ergänzungen für ARIA.

### UI/UX-Spezifikation

**Prüfmethodik (in Reihenfolge):**

1. **Automatisierter Scan:** Einsatz von axe-core (via Playwright axe-Plugin oder Browser-Extension) für erste Fehlerermittlung
2. **Manuelle Tastaturnavigation:** Alle Seiten einmal komplett ohne Maus durchnavigieren (Tab, Enter, Space, Pfeiltasten)
3. **Screenreader-Test:** Stichproben mit macOS VoiceOver oder NVDA (Windows) auf kritischen Seiten (Liste, Detail, Formular, Filter)

**Kritische Bereiche (Fokus der Überprüfung):**

- `templates/index.html` – Rezept-Liste mit Filter-Panel und Suche
- `templates/recipe/show.html` – Detailansicht mit Bewertung und Aktions-Buttons
- `templates/recipe/form.html` – Erstellen/Bearbeiten-Formular
- `templates/recipe/delete_confirm.html` – Lösch-Bestätigungs-Dialog

**Sternebewertung (technische Umsetzung):**

Falls die Sternebewertung aktuell als reine visuelle Darstellung implementiert ist, muss sie als zugängliches Widget überarbeitet werden:
- Option A: Radio-Buttons mit visuell verstecktem Label aber korrekt im DOM (bevorzugt, funktioniert ohne JS)
- Option B: Buttons mit `aria-pressed` und `aria-label` (erfordert JS)

**HTMX und ARIA Live Regions:**

Suchergebnisse und Filter-Updates via HTMX sollten mit einem `aria-live="polite"`-Bereich angekündigt werden. Die Ergebnisanzahl ("5 Rezepte gefunden") muss in einem Live-Region-Element stehen.

---

## 5. Nicht-funktionale Anforderungen

### Performance

- Keine Performance-Auswirkungen erwartet; semantisches HTML hat keinen Overhead
- ARIA-Attribute sind reine HTML-Attribute ohne Laufzeit-Kosten

### Browser-Support

- Aktuelle Chrome, Firefox, Safari, Edge (Desktop und Mobile)
- Screenreader-Tests: VoiceOver (macOS/iOS), NVDA (Windows) als primäre Ziele
- Tastaturnavigation in allen unterstützten Browsern

### Barrierefreiheit

- WCAG 2.1 Level A vollständig erfüllt (alle 30 Erfolgskriterien der Stufe A)
- Angestrebt: WCAG 2.1 Level AA für Kontrast (da dies praxisnah und einfach erreichbar ist)
- Kein automatischer Scanner (axe-core) meldet Level-A-Fehler

---

## 6. Teststrategie

### E2E-Tests (Playwright)

Playwright unterstützt Tastaturnavigation und axe-core-Integration für automatisierte Accessibility-Prüfungen.

**Testfall 1: Kein axe-Fehler auf der Startseite**
```gherkin
Given Mehrere Rezepte existieren in der Datenbank
When Benutzer öffnet die Startseite "/"
Then axe-core meldet keine Violations der Level-A-Kriterien
```

**Testfall 2: Kein axe-Fehler auf der Detailansicht**
```gherkin
Given Ein Rezept existiert in der Datenbank
When Benutzer öffnet die Detailansicht des Rezepts
Then axe-core meldet keine Violations der Level-A-Kriterien
```

**Testfall 3: Kein axe-Fehler auf dem Erstellen-Formular**
```gherkin
Given Die App ist gestartet
When Benutzer navigiert zu "Neues Rezept erstellen"
Then axe-core meldet keine Violations der Level-A-Kriterien
```

**Testfall 4: Tastaturnavigation – Rezept erstellen**
```gherkin
Given Die App ist gestartet
When Benutzer navigiert per Tab zur "Neues Rezept"-Schaltfläche und aktiviert sie per Enter
And Benutzer füllt Titel-Feld per Tastatur aus
And Benutzer wählt eine Kategorie per Tastatur aus
And Benutzer aktiviert "Speichern" per Enter
Then Das neue Rezept erscheint in der Liste
And Kein Maus-Klick war erforderlich
```

**Testfall 5: Tastaturnavigation – Lösch-Dialog**
```gherkin
Given Ein Rezept existiert in der Datenbank
When Benutzer navigiert zur Detailansicht per Tastatur
And Benutzer aktiviert "Löschen" per Enter
Then Ein Bestätigungs-Dialog erscheint
And Der Fokus liegt innerhalb des Dialogs
When Benutzer drückt "Abbrechen" per Enter oder Escape
Then Der Dialog schließt sich
And Der Fokus kehrt zum "Löschen"-Button zurück
```

**Testfall 6: Formular-Labels sind vorhanden**
```gherkin
Given Die App ist gestartet
When Benutzer öffnet das "Neues Rezept erstellen"-Formular
Then Jedes Eingabefeld hat ein zugehöriges Label-Element
And Das Label ist programmatisch mit dem Feld verknüpft (for/id)
```

---

## 7. Abhängigkeiten & Rahmenbedingungen

### Abhängigkeiten

- Alle vorherigen Stories (01–23, 26–28) müssen implementiert sein, da diese Story die bestehende Implementierung prüft und verbessert
- Story 06 (Responsive Layout) hat bereits erste Accessibility-Grundlagen geschaffen (Touch-Targets, Schriftgrößen)
- Keine Story wird durch diese Story blockiert; sie kann parallel zu anderen laufenden Stories bearbeitet werden

### Rahmenbedingungen

- Tech-Stack bleibt unverändert: Rust + Axum + Askama + HTMX + vanilla CSS
- Keine externen Accessibility-Bibliotheken – Korrekturen via HTML-Attribute und CSS
- HTMX muss weiterhin funktionieren; ARIA-Ergänzungen müssen HTMX-kompatibel sein
- Keine Authentifizierung (LAN-only) – keine Login-Formulare zu prüfen
- SQLite-Datenbankschema wird nicht verändert

---

## Offene Punkte / Fragen

- [ ] Wie ist die Sternebewertung aktuell technisch umgesetzt? (Radio-Buttons, CSS-Hack, JS-Widget?) – beeinflusst den Aufwand für K6
- [ ] Wird der Löschen-Dialog als nativer `<dialog>` oder als div-Overlay implementiert? – beeinflusst die Fokus-Verwaltungs-Strategie für K7
- [ ] Sollen axe-core Playwright-Tests für alle oder nur für die kritischsten Seiten hinzugefügt werden?

---

**Letzte Aktualisierung:** 2026-03-29
