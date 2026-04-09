# Implementierungsplan: Suche "Länger nicht gemacht" in Top-Bar verschieben

Dieser Plan beschreibt die technische Umsetzung von Story 42. Ziel ist es, den schnellen Zugriff auf die gefilterte Suche (Länger nicht gemacht + Mittagessen) global in der Top-Bar verfügbar zu machen und aus der Wochenvorschau zu entfernen.

## 1. Analyse der Ist-Situation
- **Aktueller Ort:** In `templates/wochenvorschau.html` gibt es einen Button, der auf `/?filter=laenger-nicht-gemacht&kategorie=Mittagessen` verlinkt.
- **Ziel-Ort:** In `templates/base.html` in der Hauptnavigation.
- **Funktionalität:** Der Link führt direkt zur Index-Seite mit den entsprechenden Query-Parametern. Da die Filter-Logik in `src/routes/recipes.rs` bereits implementiert ist, sind keine Backend-Änderungen notwendig.

## 2. Betroffene Dateien
- `templates/base.html`: Hinzufügen des Navigationslinks.
- `templates/wochenvorschau.html`: Entfernen des redundanten Buttons.

## 3. Implementierungsschritte

### Phase 1: UI-Anpassungen
- [ ] **Link in `base.html` integrieren**
  - Füge in der `main-nav` einen Link mit dem Text "Inspiration" oder "Länger nicht gemacht" hinzu.
  - Ziel-URL: `/?filter=laenger-nicht-gemacht&kategorie=Mittagessen`
  - CSS-Klasse: `nav-link` für konsistentes Styling.
  - Aria-Label hinzufügen für Barrierefreiheit.
- [ ] **Button in `wochenvorschau.html` entfernen**
  - Lösche den Block `<div class="wochenvorschau-toolbar">...</div>` (Zeilen 13-23), da die Funktion nun global verfügbar ist.

### Phase 2: Qualitätssicherung & Tests
- [ ] **Manueller Check**
  - Prüfen, ob der Link in der Top-Bar von jeder Seite aus erreichbar ist.
  - Prüfen, ob die Zielseite korrekt gefiltert ist (Kategorie: Mittagessen, Sortierung: Datum aufsteigend).
  - Prüfen, ob der Button in der Wochenvorschau verschwunden ist.
- [ ] **Responsive Check**
  - Prüfen, ob der Link in der mobilen Ansicht die Navigation zerschießt oder korrekt dargestellt wird.
- [ ] **E2E-Tests (Playwright)**
  - Implementierung eines Tests, der den Klick in der Top-Bar simuliert und die Filterung verifiziert.
  - Implementierung eines Tests, der sicherstellt, dass der Button in der Wochenvorschau nicht mehr existiert.

## 4. Definition of Done (DoD)
- [ ] Funktionale Kriterien K1, K2 und K3 aus der `story.md` sind erfüllt.
- [ ] Link ist in der Top-Bar vorhanden und führt zur korrekten gefilterten Liste.
- [ ] Button in `wochenvorschau.html` ist entfernt.
- [ ] Barrierefreiheit (WCAG 2.1 Level A) durch korrekte Aria-Labels und Kontraste gewährleistet.
- [ ] E2E-Tests sind geschrieben und erfolgreich durchgelaufen.
- [ ] Code-Review durchgeführt.
