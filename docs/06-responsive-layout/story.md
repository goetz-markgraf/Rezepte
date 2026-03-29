# Story 06: Responsive Layout für Desktop und Mobile

**Epic:** Rezept-Übersicht & Navigation
**Priorität:** MVP - Phase 1
**Status:** In Arbeit

---

## 1. Story-Satz

**Als** Benutzer möchte ich **die App komfortabel auf jedem meiner Geräte nutzen können – vom Smartphone bis zum Laptop**, damit ich **unabhängig vom genutzten Gerät flüssig durch meine Rezeptsammlung browsen, Rezepte lesen und bearbeiten kann**.

---

## 2. Geschäftsbezogene Details

### Kontext

Der Haushalt (Anna & Dragon) greift situationsabhängig von verschiedenen Geräten auf die Rezepte-App zu:
- **Wochenplanung:** Abends auf dem Sofa mit Tablet oder Laptop
- **Schnelles Nachschlagen:** Unterwegs oder in der Küche mit dem Smartphone
- **Rezept erfassen:** Je nach Situation Laptop (bequemes Tippen) oder Handy (spontan)

Ein Layout, das nur für Desktop optimiert ist, würde auf dem Smartphone zur Qual werden – zu kleiner Text, nicht tippbare Elemente, horizontales Scrollen. Ein nur für Mobile optimiertes Layout würde den Platz auf dem Laptop verschwenden.

Responsive Design ist kein optionales Feature, sondern Grundvoraussetzung dafür, dass die App in allen realen Nutzungsszenarien funktioniert.

### Nutzergruppe

- Beide Partner des Haushalts
- Nutzen gleichberechtigt verschiedene Geräte: Smartphone (iOS/Android), Tablet, Laptop
- Alle Geräte befinden sich im selben LAN

### Business-Value

- Die App wird tatsächlich genutzt, weil sie auf dem Alltagsgerät (Smartphone) genauso bequem funktioniert wie am Laptop
- Kein Frustrations-Moment durch unlesbare oder nicht tippbare Elemente auf dem Handy
- Schnelles Erfassen neuer Rezepte vom Smartphone wird zur reibungslosen Gewohnheit
- Wochenplanung am Tablet/Laptop profitiert von einem gut genutzten großen Display

### Edge Cases

- **Sehr schmale Bildschirme (320px):** Layout darf nicht brechen; alle Inhalte bleiben lesbar und bedienbar
- **Sehr breite Bildschirme (2560px+):** Inhaltsbereich wird zentriert und auf eine lesbare Maximalbreite begrenzt; kein 100%-Breit-Chaos
- **Orientierungswechsel (Portrait ↔ Landscape):** Layout passt sich korrekt an, kein Überlappen von Elementen
- **Touch-Geräte ohne Hover:** Interaktive Elemente, die nur per Hover zugänglich sind, sind nicht erlaubt – alle Aktionen müssen per Tap erreichbar sein
- **Systemschriftgröße vergrößert (Accessibility-Einstellung):** Layout bricht nicht, Text bleibt lesbar (kein `px`-fixierter Textzoom-Lock)
- **Formulare auf Mobile:** Tastatur erscheint beim Fokussieren eines Textfelds; das Layout schiebt sich korrekt hoch; kein verdecktes Feld durch die virtuelle Tastatur

---

## 3. Akzeptanzkriterien

### Funktionale Kriterien

- [ ] **K1: Breakpoints definiert und umgesetzt**
  - Mobile: bis 767px (Smartphones im Portrait)
  - Tablet: 768px–1023px (Tablets im Portrait und kleinere Laptops)
  - Desktop: ab 1024px (Laptops, Desktop-Monitore)
  - Das Layout wechselt fließend zwischen diesen Breakpoints

- [ ] **K2: Navigation auf Mobile bedienbar**
  - Haupt-Navigation ist auf kleinen Bildschirmen vollständig zugänglich
  - Navigations-Elemente sind mit dem Daumen tippbar (Touch-Target min. 44×44px)
  - Kein horizontales Scrollen auf Smartphones notwendig

- [ ] **K3: Rezept-Liste auf allen Geräten übersichtlich**
  - Mobile: Einspaltiges Layout, volle Breite, gut lesbare Listeneinträge
  - Tablet/Desktop: Gleicher einspaltiger Listenbereich oder alternativ besser genutzter Platz (z.B. breitere Einträge mit mehr Infos)
  - Jeder Listeneintrag ist per Touch und per Klick komfortabel aktivierbar

- [ ] **K4: Rezept-Detailansicht auf Mobile lesbar**
  - Langer Fließtext (Zutaten, Anleitung) ist ohne horizontales Scrollen lesbar
  - Mindestschriftgröße 16px für Texte
  - Bilder (falls vorhanden) skalieren korrekt auf Bildschirmbreite

- [ ] **K5: Formulare auf Mobile bedienbar**
  - Formularfelder (Erstellen, Bearbeiten) sind ausreichend groß für Tap-Eingabe
  - Kein Feld wird durch die virtuelle Tastatur dauerhaft verdeckt
  - Speichern- und Abbrechen-Buttons sind auf Mobile mit dem Daumen erreichbar

- [ ] **K6: Inhaltsbereich auf großen Bildschirmen begrenzt**
  - Auf breiten Desktop-Bildschirmen wird der Inhalt auf eine lesbare Maximalbreite begrenzt (ca. 900–1200px)
  - Inhalt wird horizontal zentriert

- [ ] **K7: Alle Kernfunktionen ohne JavaScript nutzbar**
  - Das responsive Layout basiert auf CSS (Media Queries); keine JS-gesteuerte Layout-Umschaltung
  - Kernfunktionen (Liste ansehen, Rezept öffnen, Formular absenden) funktionieren ohne JS

### Nicht-funktionale Kriterien

- [ ] **K8: Performance auf Mobile**
  - Seite lädt auf einem durchschnittlichen Mobilnetz in unter 2 Sekunden (kein unnötiger JS-Overhead, optimierte Assets)

- [ ] **K9: Barrierefreiheit**
  - Touch-Targets für interaktive Elemente min. 44×44px (WCAG 2.1, Success Criterion 2.5.5)
  - Fokus-Indikatoren sind auf allen Geräten sichtbar
  - Kein Inhalt verschwindet durch das responsive Layout (kein `display: none` für funktionsrelevante Inhalte)

---

## 4. Technische Planung

### Datenmodell

Keine Änderungen am Datenbankschema. Diese Story betrifft ausschließlich Präsentation und CSS.

### UI/UX-Spezifikation

**Allgemeines Layout-Prinzip: Mobile-First**

CSS wird mobile-first geschrieben: Basis-Styles gelten für kleine Bildschirme, `min-width`-Media-Queries erweitern für größere Bildschirme.

**Breakpoints:**

```css
/* Mobile: Standard (kein Breakpoint, Basis-CSS) */
/* Tablet: ab 768px */
@media (min-width: 768px) { ... }
/* Desktop: ab 1024px */
@media (min-width: 1024px) { ... }
```

**Seitenstruktur (alle Breakpoints):**

```
┌─────────────────────────┐
│         Header          │  ← Logo/App-Name, Navigation
├─────────────────────────┤
│         Main            │  ← Inhalt (Liste, Detail, Formular)
├─────────────────────────┤
│         Footer          │  ← Optional, minimal
└─────────────────────────┘
```

**Mobile-Layout (bis 767px):**
- Header: App-Name links, ggf. kompakte Navigation (alle Links erreichbar)
- Main: volle Breite, 16px horizontales Padding
- Listeneinträge: volle Breite, min. 48px Zeilenhöhe für Tippbarkeit
- Buttons: volle Breite oder min. 44px hoch

**Tablet-Layout (768px–1023px):**
- Header: horizontal ausgerichtet, Navigation als Zeile
- Main: max. 720px zentriert oder volle Breite mit mehr Padding (32px)
- Formulare: Labels und Felder bleiben einspaltiger Strom (kein 2-Spalten-Layout)

**Desktop-Layout (ab 1024px):**
- Header: horizontal, Navigation vollständig sichtbar
- Main: max. 960px zentriert, 24px horizontales Padding
- Formulare: Labels möglicherweise neben Feldern (optional, nur wenn es die Lesbarkeit verbessert)

**Touch-Target-Mindestgrößen:**
- Alle Buttons, Links und interaktiven Elemente: min. 44×44px
- Listeneinträge in der Rezept-Liste: min. 48px Höhe, gesamter Eintrag klickbar

---

## 5. Nicht-funktionale Anforderungen

### Performance

- Keine externen CSS-Frameworks (kein Bootstrap, kein Tailwind) – vanilla CSS für minimale Asset-Größe
- Kein JavaScript für Layout-Anpassungen
- CSS-Datei bleibt unter 20 KB (unkomprimiert)

### Browser-Support

- Aktuelle Chrome, Firefox, Safari, Edge (Desktop und Mobile)
- iOS Safari ab Version 15
- Android Chrome ab Version 110
- Bildschirmbreiten: 320px bis 2560px

### Barrierefreiheit

- WCAG 2.1 Level A konform
- Touch-Targets min. 44×44px
- Schriftgröße mind. 16px für Texte, 14px als absolutes Minimum
- Keine Layout-Falle: Alle Inhalte bleiben bei vergrößerter Systemschrift lesbar (rem/em statt px für Schriftgrößen)

---

## 6. Teststrategie

### E2E-Tests (Playwright)

Playwright unterstützt Viewport-Simulation – jeder Test kann mit spezifischen Viewport-Dimensionen ausgeführt werden.

**Testfall 1: Rezept-Liste auf Mobile (Smartphone)**
```gherkin
Given Mehrere Rezepte existieren in der Datenbank
And Viewport ist 390×844px (iPhone-Größe)
When Benutzer öffnet die Startseite "/"
Then Alle Rezepte werden in einer einspaltigen Liste angezeigt
And Kein horizontales Scrollen ist erforderlich
And Jeder Listeneintrag ist min. 44px hoch
```

**Testfall 2: Rezept-Liste auf Desktop**
```gherkin
Given Mehrere Rezepte existieren in der Datenbank
And Viewport ist 1280×800px (Desktop-Größe)
When Benutzer öffnet die Startseite "/"
Then Die Rezept-Liste wird korrekt angezeigt
And Der Inhaltsbereich ist zentriert und nicht über die volle Bildschirmbreite gedehnt
```

**Testfall 3: Formular auf Mobile bedienbar**
```gherkin
Given Viewport ist 390×844px (iPhone-Größe)
When Benutzer navigiert zu "Neues Rezept erstellen"
Then Das Formular wird einspaltiger ohne horizontales Scrollen angezeigt
And Alle Felder sind tippbar (Breite mind. 44px)
And "Speichern"-Button ist sichtbar und tippbar
```

**Testfall 4: Navigation auf Mobile zugänglich**
```gherkin
Given Viewport ist 390×844px (iPhone-Größe)
When Benutzer öffnet die App
Then Die Navigation ist vollständig zugänglich (alle wichtigen Links erreichbar)
And Kein Navigationselement ist abgeschnitten oder unsichtbar
```

**Testfall 5: Rezept-Detailansicht auf Mobile lesbar**
```gherkin
Given Ein Rezept mit langer Anleitung existiert in der Datenbank
And Viewport ist 390×844px
When Benutzer öffnet die Detailansicht des Rezepts
Then Der Text ist ohne horizontales Scrollen vollständig lesbar
And Alle Aktions-Buttons (Bearbeiten, Löschen) sind tippbar
```

---

## 7. Abhängigkeiten & Rahmenbedingungen

### Abhängigkeiten

- **Story 01–05** sollten implementiert sein, damit alle Seiten (Liste, Detailansicht, Formulare) responsiv gestaltet werden können
- Blockiert keine weiteren Stories direkt, verbessert aber die Nutzbarkeit aller bestehenden und künftigen UI-Features
- Story 25 (WCAG 2.1 Accessibility) baut auf dem durch diese Story etablierten Touch-Target- und Schriftgrößen-Standard auf

### Rahmenbedingungen

- Keine Authentifizierung, kein Login-Screen (LAN-only)
- Keine externen CSS-Bibliotheken – nur vanilla CSS mit Media Queries
- Der bestehende Tech-Stack (Rust + Axum + Askama + vanilla CSS) bleibt unverändert
- CSS wird als statische Datei unter `static/css/` ausgeliefert

---

## Offene Punkte / Fragen

- [ ] Soll die Navigation auf sehr kleinen Bildschirmen als Hamburger-Menü (JS-abhängig) oder als kompakte horizontale Leiste (CSS-only) umgesetzt werden? Empfehlung: CSS-only Variante bevorzugen (weniger JS, robuster).
- [ ] Sollen Formulare auf Desktop zweispaltig dargestellt werden (Label links, Feld rechts), oder bleibt es einheitlich einspaltiger Fluss? Empfehlung: einspaltig – einfacher und konsistenter.

---

**Letzte Aktualisierung:** 2026-03-29
