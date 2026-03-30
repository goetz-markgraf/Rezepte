# Story 31: Kompakteres Layout der Hauptansicht

**Epic:** Epic 2: Rezept-Übersicht & Navigation
**Priorität:** Nice-to-have
**Status:** Offen

---

## 1. Story-Satz

Als **Benutzer** möchte ich **eine kompaktere Hauptansicht**, damit ich **mehr Rezepte auf einmal sehe, ohne scrollen zu müssen**.

---

## 2. Geschäftsbezogene Details

### Kontext
Aktuell sind die UI-Elemente in der Hauptansicht sehr groß dimensioniert. Auf einem Laptop-Display können oft nur 3-4 Rezepte gleichzeitig gesehen werden, bevor gescrollt werden muss. Dies führt zu ineffizienter Navigation, besonders wenn man die gesamte Rezeptsammlung überblicken oder nach bestimmten Rezepten suchen möchte.

### Nutzergruppe
- Beide Partner (gleichberechtigte Nutzer)
- Nutzer, die primär auf Laptops oder kleineren Displays arbeiten
- Nutzer mit größeren Rezeptsammlungen (>50 Rezepte)

### Business-Value
- **Effizientere Navigation:** Weniger Scrollen = schnellerer Zugriff auf Rezepte
- **Besserer Überblick:** Mehr Rezepte auf einen Blick erfassbar
- **Reduzierte kognitive Belastung:** Weniger visuelles "Springen" beim Scrollen
- **Erhöhte Zufriedenheit:** Schnellerer Workflow bei der Wochenplanung

### Edge Cases
- **Viele Filter aktiv:** Wenn viele Filter gleichzeitig aktiv sind, muss die kompakte Darstellung trotzdem übersichtlich bleiben
- **Lange Rezepttitel:** Titel müssen trotz kompakter Darstellung lesbar bleiben (ggf. mit Tooltip oder abgeschnitten mit Ellipsis)
- **Touch-Interaktion:** Die Touch-Ziele müssen auf Mobilgeräten weiterhin ausreichend groß bleiben (>44px)
- **Verschiedene Bildschirmgrößen:** Das Layout muss von kleinen Laptops (13") bis große Monitore (27") skalieren

---

## 3. Akzeptanzkriterien

### Funktionale Kriterien

- [ ] **K1: Reduzierte vertikale Größe der Rezeptkacheln**
  - Die Höhe der Rezeptkacheln wird um mindestens 25% reduziert
  - Bei einer Standard-Laptop-Ansicht (1366x768) passen mindestens 6-8 Rezepte gleichzeitig ins sichtbare Fenster

- [ ] **K2: Optimierter Whitespace**
  - Padding und Margins werden reduziert, ohne die Lesbarkeit zu beeinträchtigen
  - Das Layout wirkt nicht überladen oder "gequetscht"

- [ ] **K3: Kompakte Darstellung der Filterleiste**
  - Die Filterkomponenten nehmen weniger vertikalen Platz ein
  - Filter bleiben trotzdem intuitiv bedienbar

- [ ] **K4: Konsistente Lesbarkeit**
  - Alle Texte bleiben gut lesbar (Mindest-Schriftgröße 14px)
  - Rezepttitel werden bei Überlänge mit Ellipsis abgeschnitten
  - Bei Hover über abgeschnittenen Titeln wird der vollständige Titel als Tooltip angezeigt

- [ ] **K5: Responsive Fallback**
  - Auf Mobilgeräten (<768px) bleibt die aktuelle, touch-freundliche Größe erhalten
  - Die Kompaktheit gilt primär für Desktop-Ansichten

### Nicht-funktionale Kriterien

- [ ] **K6: Performance**
  - Keine messbare Verschlechterung der Ladezeit durch Layout-Änderungen
  - Rendering bleibt flüssig auch bei langen Listen (>100 Rezepte)

- [ ] **K7: Barrierefreiheit**
  - Touch-Ziele bleiben auf Mobilgeräten mindestens 44x44px
  - Kontrastverhältnisse bleiben WCAG 2.1 Level A konform
  - Fokus-Indikatoren bleiben sichtbar und gut erkennbar

---

## 4. Technische Planung

### Datenmodell
Keine Änderungen am Datenmodell erforderlich. Dies ist eine reine UI-Optimierung.

### UI/UX-Spezifikation

**Aktuelles Problem:**
- Rezeptkacheln sind visuell sehr groß dimensioniert
- Üppige Padding-Werte und Whitespace
- Filterleiste nimmt viel vertikalen Platz ein
- Bei Standard-Laptop-Ansicht werden nur 3-4 Rezepte sichtbar

**Gewünschte Verbesserungen:**

1. **Rezeptkacheln:**
   - Reduzierung der Padding-Werte (z.B. von 24px auf 12-16px)
   - Kompaktere Darstellung der Metadaten (Bewertung, Kategorien)
   - Optional: Kompaktere Darstellung des "Zuletzt gekocht"-Datums

2. **Filterleiste:**
   - Horizontale Anordnung von Filtern statt vertikal, falls Platz vorhanden
   - Oder: Reduzierte Höhe der einzelnen Filter-Controls
   - Kompaktere Darstellung der aktiven Filter-Badges

3. **Gesamtlayout:**
   - Optimierung des Grid-Layouts für mehr Kacheln pro Zeile
   - Reduzierung der Abstände zwischen den Kacheln

4. **Typography:**
   - Sicherstellung, dass Schriftgrößen nicht zu klein werden
   - Eventuell leichte Reduktion der Überschriften-Größe

---

## 5. Nicht-funktionale Anforderungen

### Performance
- Seite lädt weiterhin ohne sichtbare Verzögerung (< 500ms)
- Layout-Berechnungen beeinträchtigen die Interaktivität nicht

### Browser-Support
- Aktuelle Chrome, Firefox, Safari, Edge Versionen
- iOS Safari und Android Chrome (letzte 2 Versionen)
- Desktop-Optimierung gilt für alle gängigen Desktop-Browser

### Barrierefreiheit
- WCAG 2.1 Level A konform bleiben
- Touch-Ziele auf Mobilgeräten mindestens 44x44px
- Fokus-Indikatoren klar sichtbar
- Keine Verschlechterung der Screenreader-Kompatibilität

---

## 6. Teststrategie

### E2E-Tests (Playwright)

**Testfall 1: Kompakte Darstellung auf Desktop**
```gherkin
Given der Benutzer ist auf der Hauptansicht
And die Viewport-Größe beträgt 1366x768 (Laptop)
When die Rezeptliste geladen ist
Then sind mindestens 6 Rezeptkacheln gleichzeitig sichtbar ohne Scrollen
```

**Testfall 2: Tooltip bei langem Titel**
```gherkin
Given ein Rezept mit einem sehr langen Titel wird angezeigt
When der Benutzer mit der Maus über den abgeschnittenen Titel fährt
Then wird der vollständige Titel als Tooltip angezeigt
```

**Testfall 3: Mobile Touch-Ziele**
```gherkin
Given der Benutzer ist auf der Hauptansicht
And die Viewport-Größe beträgt 375x667 (Mobile)
When die Rezeptliste geladen ist
Then sind alle Touch-Ziele mindestens 44x44 Pixel groß
```

**Testfall 4: Lesbarkeit bei Kompaktheit**
```gherkin
Given der Benutzer ist auf der Hauptansicht mit kompaktem Layout
When der Benutzer die Seite betrachtet
Then sind alle Texte klar lesbar ohne Zoom
And alle Rezeptinformationen sind eindeutig erkennbar
```

---

## 7. Abhängigkeiten & Rahmenbedingungen

### Abhängigkeiten
- Story 05 (Rezept-Liste alphabetisch sortiert) muss implementiert sein
- Story 06 (Responsive Layout) muss implementiert sein
- Story 26 (Nutzung von Icons) kann hilfreich sein für kompaktere Darstellung

### Rahmenbedingungen
- Keine Änderungen an der Datenbank erforderlich
- Reine CSS/HTML-Template-Änderung
- Keine neuen Abhängigkeiten
- LAN-only Zugriff bleibt unverändert

---

## Offene Punkte / Fragen

- [ ] Soll es einen Toggle geben, um zwischen "Kompakt" und "Komfort"-Ansicht zu wechseln?
- [ ] Soll die Kompaktheit auch auf Tablet-Größen (768px-1024px) angewendet werden?
- [ ] Welche spezifischen Elemente sollen priorisiert verkleinert werden?

---

**Letzte Aktualisierung:** 2026-03-30

---

## Zusatzinformationen

Die Ansicht soll insgesamt kleiner werden. Die Elemente/Komponenten sind aktuell recht groß, was schon auf einem Laptop dazu führt, dass man zu wenig Elemente sieht. Strukturiere die Haupt-Ansicht um, sodass die kleiner und übersichtlicher wird.
