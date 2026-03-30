# Story 32: Neues-Rezept-Button in der Kopfzeile

**Epic:** Rezept-Übersicht & Navigation
**Priorität:** MVP Phase 2
**Status:** Ready

---

## 1. Story-Satz

Als **Benutzer** möchte ich **einen "Neues Rezept"-Button in der Kopfzeile haben**, damit ich **jederzeit schnell und ohne Umweg ein neues Rezept erstellen kann**.

---

## 2. Geschäftsbezogene Details

### Kontext
Aktuell muss der Benutzer erst zur Rezept-Liste navigieren, um ein neues Rezept zu erstellen. Ein prominent platzierten Button in der globalen Kopfzeile ermöglicht den direkten Zugriff von jeder Seite aus, was den Workflow erheblich beschleunigt.

### Nutzergruppe
Alle Benutzer der Anwendung, insbesondere beim Erstellen mehrerer Rezepte hintereinander oder beim schnellen Erfassen einer Idee.

### Business-Value
- Reduzierte Klickzahl für die Hauptaktion "Rezept erstellen"
- Bessere UX durch direkten Zugriff auf Kernfunktionalität
- Konsistente Navigation über alle Seiten hinweg

### Edge Cases
- **Mobile Ansicht:** Button muss auf kleinen Bildschirmen gut erreichbar sein (entweder sichtbar oder im Menü)
- **Aktive Seite:** Wenn der Benutzer bereits auf der "Neues Rezept"-Seite ist, sollte der Button deaktiviert oder nicht klickbar sein (optional)
- **Keyboard-Navigation:** Button muss mit Tab erreichbar sein und visuell fokussierbar

---

## 3. Akzeptanzkriterien

### Funktionale Kriterien

- [ ] **K1: Button ist in der Kopfzeile sichtbar**
  - Der Button wird rechts in der globalen Header-Navigation angezeigt
  - Der Button ist auf allen Seiten der Anwendung sichtbar

- [ ] **K2: Button führt zur Rezept-Erstellen-Seite**
  - Klick auf den Button navigiert zu `/recipes/new`
  - Die Navigation erfolgt ohne Zwischenseite

- [ ] **K3: Button hat klare Beschriftung**
  - Text: "+ Neues Rezept" oder ein Plus-Icon mit Text
  - Alternativ nur Icon auf Mobile mit Tooltip/Title

- [ ] **K4: Button ist barrierefrei**
  - Button hat semantisches `<button>` oder `<a>` Element
  - Aria-Label für Screenreader vorhanden
  - Fokus-Indikator ist sichtbar

### Nicht-funktionale Kriterien

- [ ] **K5: Performance**
  - Button rendert ohne spürbare Verzögerung
  - Keine zusätzlichen Server-Requests für die Anzeige

- [ ] **K6: Barrierefreiheit**
  - Button ist mit Tastatur erreichbar (Tab-Reihenfolge)
  - Kontrast entspricht WCAG 2.1 Level AA
  - Touch-Target mindestens 44x44px auf Mobile

---

## 4. Technische Planung

### Datenmodell
Keine Änderungen am Datenmodell erforderlich.

### UI/UX-Spezifikation

**Platzierung:**
- Rechts in der Kopfzeile (nach den Navigationslinks)
- Visuell hervorgehoben als Primary Action (z.B. mit Akzentfarbe oder als Button statt Link)

**Design-Vorschlag:**
```
[Rezepte]  [Heute] [Wochenvorschau] [Dubletten prüfen]    [+ Neues Rezept]
```

**Mobile:**
- Entweder: Text wird zu Icon (+) reduziert
- Oder: Button wandert in ein Hamburger-Menü

**Interaktion:**
- Hover: Leichte Hervorhebung (z.B. Hintergrundfarbe ändern)
- Focus: Sichtbarer Fokus-Ring
- Active: Leichte Skalierung oder Farbänderung beim Klick

---

## 5. Nicht-funktionale Anforderungen

### Performance
- Button ist Teil des Base-Templates, kein zusätzlicher Request
- Ladezeit der Seite bleibt unter 500ms

### Browser-Support
- Aktuelle Chrome, Firefox, Safari, Edge Versionen
- iOS Safari und Android Chrome (letzte 2 Versionen)

### Barrierefreiheit
- WCAG 2.1 Level A konform (mindestens)
- Fokus-Indikatoren sichtbar
- Tastatur-Navigation funktioniert vollständig

---

## 6. Teststrategie

### E2E-Tests (Playwright)

**Testfall 1: Button ist auf allen Seiten sichtbar**
```gherkin
Given der Benutzer ist auf der Startseite
Then ist der "Neues Rezept"-Button in der Kopfzeile sichtbar

Given der Benutzer ist auf der Wochenvorschau-Seite
Then ist der "Neues Rezept"-Button in der Kopfzeile sichtbar

Given der Benutzer ist auf der Dubletten-Prüf-Seite
Then ist der "Neues Rezept"-Button in der Kopfzeile sichtbar
```

**Testfall 2: Button navigiert zur Erstell-Seite**
```gherkin
Given der Benutzer ist auf einer beliebigen Seite
When der Benutzer auf den "Neues Rezept"-Button klickt
Then wird die Seite "/recipes/new" geladen
And das Formular zum Erstellen eines Rezepts wird angezeigt
```

**Testfall 3: Button ist mit Tastatur erreichbar**
```gherkin
Given der Benutzer ist auf der Startseite
When der Benutzer mehrmals Tab drückt
Then erreicht der Fokus den "Neues Rezept"-Button
When der Benutzer Enter drückt
Then wird die Seite "/recipes/new" geladen
```

**Testfall 4: Mobile Darstellung**
```gherkin
Given der Benutzer nutzt ein mobiles Gerät (Viewport < 768px)
When der Benutzer die Seite öffnet
Then ist der "Neues Rezept"-Button entweder als Icon oder im Menü sichtbar
```

---

## 7. Abhängigkeiten & Rahmenbedingungen

### Abhängigkeiten
- Story 01 (Rezept erstellen) muss implementiert sein - die Zielseite muss existieren
- Keine weiteren Abhängigkeiten

### Rahmenbedingungen
- Änderung am Base-Template (templates/base.html)
- CSS-Anpassungen für das Header-Styling
- Keine Datenbank-Änderungen erforderlich

---

## Offene Punkte / Fragen

- [ ] Soll der Button auf Mobile als Text oder Icon dargestellt werden?
- [ ] Soll der Button eine spezielle Akzent-Farbe erhalten (Primary Action)?

---

**Letzte Aktualisierung:** 2026-03-30
