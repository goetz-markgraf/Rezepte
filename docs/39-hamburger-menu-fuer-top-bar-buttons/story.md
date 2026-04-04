# Story 39: Button "Heute" und "Dubletten Prüfen" ins Hamburger-Menü verschieben

**Epic:** Epic 2: Rezept-Übersicht & Navigation
**Priorität:** [MVP Phase X / Nice-to-have]
**Status:** Offen

---

## 1. Story-Satz

Als **Benutzer** möchte ich die Buttons "Heute" und "Dubletten prüfen" in einem Hamburger-Menü haben, damit die Top-Bar übersichtlicher bleibt und mehr Platz für die wichtigsten Navigationselemente zur Verfügung steht.

---

## 2. Geschäftsbezogene Details

### Kontext
Die Top-Bar der Anwendung enthält aktuell die Links "Heute" und "Dubletten prüfen". Diese nehmen wertvollen Platz weg, besonders auf mobilen Geräten. Durch das Verschieben in ein Hamburger-Menü (☰) wird die Oberfläche aufgeräumter und der Fokus bleibt auf den wichtigsten Navigationselementen.

### Nutzergruppe
- Beide Partner (Single-User-System)
- Nutzer, die die Anwendung auf verschiedenen Geräten (Desktop und Mobile) verwenden

### Business-Value
- Bessere Nutzung des begrenzten Platzes in der Top-Bar
- Sauberere, fokussiertere Benutzeroberfläche
- Bessere mobile Nutzererfahrung

### Edge Cases
- **Mobile Ansicht:** Das Hamburger-Menü muss auf kleinen Bildschirmen gut bedienbar sein
- **Tastatur-Navigation:** Das Menü muss vollständig per Tastatur erreichbar sein (WCAG 2.1 Level A)
- **Screenreader:** Aria-Attribute müssen korrekt gesetzt sein, damit Screenreader den Menü-Zustand verstehen

---

## 3. Akzeptanzkriterien

### Funktionale Kriterien

- [ ] **K1: Hamburger-Menü existiert**
  - In der Top-Bar wird ein Hamburger-Menü-Button (☰) angezeigt
  - Der Button ist rechts in der Top-Bar positioniert
  - Bei Klick auf den Button öffnet sich ein Dropdown-Menü

- [ ] **K2: Menü-Items verschoben**
  - Die Links "Heute" und "Dubletten prüfen" sind aus der Hauptnavigation entfernt
  - Beide Links erscheinen im Hamburger-Menü
  - Die Links funktionieren wie bisher (korrekte URLs: `/heute` und `/recipes/duplicates`)

- [ ] **K3: Menü-Verhalten**
  - Klick außerhalb des Menüs schließt es
  - Klick auf einen Menüpunkt schließt das Menü und navigiert zur Seite
  - Das Menü kann per Escape-Taste geschlossen werden

- [ ] **K4: Mobile Kompatibilität**
  - Das Hamburger-Menü ist auf mobilen Geräten gut bedienbar
  - Touch-Targets sind mindestens 44x44px groß

### Nicht-funktionale Kriterien

- [ ] **K5: Barrierefreiheit**
  - Der Hamburger-Button hat ein aria-label (z.B. "Menü öffnen")
  - Das Menü hat korrekte aria-expanded Attribute
  - Tastatur-Navigation funktioniert vollständig (Tab, Enter, Escape)
  - Fokus wird beim Öffnen des Menüs auf das erste Element gesetzt
  - Fokus-Trap im geöffneten Menü (optional aber empfohlen)

---

## 4. Technische Planung

### Datenmodell
Keine Änderungen am Datenmodell notwendig.

### UI/UX-Spezifikation
- Hamburger-Menü-Button mit Icon (☰ oder SVG-Icon) rechts in der Top-Bar
- Dropdown-Menü unter dem Button mit den Links "Heute" und "Dubletten prüfen"
- CSS-Transition für sanftes Öffnen/Schließen
- Mobile-first Ansatz: Menü passt sich der Bildschirmbreite an

---

## 5. Nicht-funktionale Anforderungen

### Performance
- Keine zusätzliche Ladezeit durch das Hamburger-Menü
- CSS-Animationen unter 300ms

### Browser-Support
- Aktuelle Chrome, Firefox, Safari, Edge Versionen
- iOS Safari und Android Chrome (letzte 2 Versionen)

### Barrierefreiheit
- WCAG 2.1 Level A konform
- Fokus-Indikatoren sichtbar
- Ausreichender Kontrast für Icons und Menü-Items

---

## 6. Teststrategie

### E2E-Tests (Playwright)

**Testfall 1: Hamburger-Menü öffnen und schließen**
```gherkin
Given Der Benutzer ist auf der Startseite
When Der Benutzer auf den Hamburger-Menü-Button klickt
Then Das Menü öffnet sich und zeigt "Heute" und "Dubletten prüfen"
When Der Benutzer außerhalb des Menüs klickt
Then Das Menü schließt sich
```

**Testfall 2: Navigation über Hamburger-Menü**
```gherkin
Given Der Benutzer ist auf der Startseite
When Der Benutzer auf den Hamburger-Menü-Button klickt
And Der Benutzer auf "Heute" klickt
Then Der Benutzer wird zur Seite "/heute" weitergeleitet
```

**Testfall 3: Tastatur-Navigation**
```gherkin
Given Der Benutzer ist auf der Startseite
When Der Benutzer den Hamburger-Button mit Tab fokussiert
And Der Benutzer Enter drückt
Then Das Menü öffnet sich
When Der Benutzer Escape drückt
Then Das Menü schließt sich
```

---

## 7. Abhängigkeiten & Rahmenbedingungen

### Abhängigkeiten
- Story 32: Neues-Rezept-Button in der Kopfzeile (bereits abgeschlossen)
- Keine weiteren Abhängigkeiten

### Rahmenbedingungen
- Keine Änderungen am Backend notwendig
- Reine UI-Änderung in den Templates

---

## Offene Punkte / Fragen

- [x] Keine offenen Punkte

---

## Zusatzinformationen

Schiebe die Button Heute und Dubletten Prüfen aus der Top-Bar in ein Hamburger-Menü, damit sie weniger Platz wegnehmen.

---

**Letzte Aktualisierung:** 2026-04-04
