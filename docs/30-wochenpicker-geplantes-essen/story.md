# Story 30: Wochenpicker zeigt geplantes Essen

**Epic:** Epic 5: Wochenplanung
**Priorität:** MVP
**Status:** Offen

---

## 1. Story-Satz

Als **Benutzer** möchte ich **im Wochenpicker auf einen Blick sehen, für welche Tage bereits ein Essen geplant ist**, damit ich **schnell den Überblick über die Wochenplanung behalte und gezielt zu geplanten Rezepten navigieren kann**.

---

## 2. Geschäftsbezogene Details

### Kontext
Die Wochenplanung ist ein zentrales Feature der Rezepte-Verwaltung. Benutzer planen täglich ihre Mahlzeiten, aber ohne visuellen Indikator im Wochenpicker lässt sich auf den ersten Blick nicht erkennen, welche Tage bereits belegt sind. Ein visuelles Feedback verbessert die Nutzerführung erheblich.

### Nutzergruppe
Alle Benutzer der Rezepte-App (beide Partner nutzen dieselbe App ohne getrennte Accounts).

### Business-Value
- Bessere Übersicht über die Wochenplanung
- Schnellere Navigation zu geplanten Rezepten
- Reduzierung von Doppelplanungen
- Intuitivere Bedienung des Wochenpickers

### Edge Cases
- **Mehrere Rezepte an einem Tag:** Wenn ein Tag mehrere Rezepte hat, zeigt der Indikator lediglich, dass etwas geplant ist (keine Angabe der Anzahl)
- **Kein geplantes Essen:** Tage ohne Planung zeigen keinen Indikator
- **Tooltip bei langen Rezeptnamen:** Der Tooltip zeigt den vollständigen Rezeptnamen, auch wenn dieser lang ist (mit entsprechender Textumbruch oder Beschränkung)

---

## 3. Akzeptanzkriterien

### Funktionale Kriterien

- [ ] **K1: Visueller Indikator für geplante Tage**
  - Für jeden Tag im Wochenpicker, an dem mindestens ein Rezept geplant ist, wird ein blauer Stern (oder ähnliches Icon) angezeigt
  - Der Indikator ist visuell deutlich erkennbar, ohne den Datumsbereich zu überlagern
  - Der Indikator wird unabhängig davon angezeigt, ob es ein oder mehrere Rezepte an diesem Tag gibt

- [ ] **K2: Tooltip mit Rezeptname beim Hover**
  - Beim Hover über den Indikator wird ein Tooltip eingeblendet
  - Der Tooltip zeigt den Namen des geplanten Rezepts
  - Bei mehreren Rezepten am selben Tag wird der Name des ersten Rezepts angezeigt (oder eine Zusammenfassung wie "3 Rezepte geplant")
  - Der Tooltip erscheint nach kurzer Verzögerung (ca. 300ms) und verschwindet, wenn der Mauszeiger den Bereich verlässt

- [ ] **K3: Navigation zur Rezept-Detailseite**
  - Ein Klick auf den Indikator führt zur Detailseite des Rezepts
  - Bei mehreren Rezepten am selben Tag wird zur Detailseite des ersten Rezepts navigiert
  - Die Navigation erfolgt als Link (kein JavaScript erforderlich für die Grundfunktionalität)

- [ ] **K4: Tage ohne Planung**
  - Tage ohne geplantes Essen zeigen keinen Indikator
  - Die Darstellung solcher Tage bleibt unverändert zum aktuellen Zustand

### Nicht-funktionale Kriterien

- [ ] **K5: Performance**
  - Die Indikatoren werden beim Laden der Seite ohne sichtbare Verzögerung angezeigt
  - Der Tooltip erscheint flüssig innerhalb von 200ms

- [ ] **K6: Barrierefreiheit**
  - Der Indikator ist als Link semantisch korrekt ausgezeichnet
  - Der Indikator hat einen aussagekräftigen aria-label (z.B. "Geplantes Essen: [Rezeptname]")
  - Der Tooltip ist für Screenreader zugänglich (aria-describedby oder title-Attribut)
  - Tastatur-Navigation funktioniert vollständig (Fokus, Enter zum Öffnen)

---

## 4. Technische Planung

### Datenmodell
Keine Änderungen am Datenmodell erforderlich. Die Daten werden aus der bestehenden `meal_plans` Tabelle abgefragt.

Benötigte Daten pro Tag im Wochenpicker:
- `recipe_id` (für Link zur Detailseite)
- `recipe_name` (für Tooltip-Anzeige)
- Anzahl der Rezepte (optional, für erweiterte Tooltip-Logik)

### UI/UX-Spezifikation

**Layout:**
- Der Indikator (z.B. ⭐ oder 🍽️) wird rechts oben oder rechts unten in der Datumszelle positioniert
- Größe: ca. 16x16px
- Farbe: Blau (#0066CC oder ähnlich) zur Unterscheidung vom restlichen UI

**Interaktion:**
- **Hover-Zustand:** Tooltip erscheint oberhalb oder unterhalb des Indikators
- **Klick-Zustand:** Navigation zu `/recipes/{recipe_id}`
- **Fokus-Zustand:** Deutlicher Fokus-Rahmen für Tastatur-Navigation

**Tooltip-Design:**
- Hintergrund: Dunkelgrau oder Schwarz mit hohem Kontrast
- Text: Weiß
- Padding: 8px
- Border-Radius: 4px
- Max-Width: 200px (mit Textumbruch)

---

## 5. Nicht-funktionale Anforderungen

### Performance
- Seite lädt ohne sichtbare Verzögerung (< 500ms)
- Tooltip-Rendering flüssig (< 200ms)
- Keine zusätzlichen Server-Requests beim Hover (alle Daten bereits beim Seitenaufruf vorhanden)

### Browser-Support
- Aktuelle Chrome, Firefox, Safari, Edge Versionen
- iOS Safari und Android Chrome (letzte 2 Versionen)
- CSS-Feature "position: absolute" muss unterstützt werden (kein IE11-Support erforderlich)

### Barrierefreiheit
- WCAG 2.1 Level A konform
- Fokus-Indikatoren sichtbar
- Tooltips für Screenreader verständlich
- Kontrastverhältnis des Indikators mindestens 4.5:1

---

## 6. Teststrategie

### E2E-Tests (Playwright)

**Testfall 1: Indikator wird für geplante Tage angezeigt**
```gherkin
Given ein Tag hat ein geplantes Rezept
When der Benutzer den Wochenpicker öffnet
Then wird ein Indikator (z.B. blauer Stern) für diesen Tag angezeigt
```

**Testfall 2: Kein Indikator für ungeplante Tage**
```gherkin
Given ein Tag hat kein geplantes Rezept
When der Benutzer den Wochenpicker öffnet
Then wird kein Indikator für diesen Tag angezeigt
```

**Testfall 3: Tooltip zeigt Rezeptname beim Hover**
```gherkin
Given ein Tag hat ein geplantes Rezept "Pasta Carbonara"
When der Benutzer über den Indikator hovert
Then erscheint ein Tooltip mit dem Text "Pasta Carbonara"
```

**Testfall 4: Klick navigiert zur Detailseite**
```gherkin
Given ein Tag hat ein geplantes Rezept mit ID 123
When der Benutzer auf den Indikator klickt
Then wird zur Detailseite /recipes/123 navigiert
```

**Testfall 5: Tastatur-Navigation funktioniert**
```gherkin
Given ein Tag hat ein geplantes Rezept
When der Benutzer den Indikator mit der Tab-Taste fokussiert
Then ist der Indikator sichtbar fokussiert
And ein Enter öffnet die Detailseite
```

---

## 7. Abhängigkeiten & Rahmenbedingungen

### Abhängigkeiten
- Story 28: Wochenpicker Grundfunktionalität muss implementiert sein
- Story 29: Essen planen muss implementiert sein (damit geplante Essen existieren können)

### Rahmenbedingungen
- SQLite-Datenbank muss existieren und erreichbar sein
- Keine Authentifizierung erforderlich (LAN-only)
- Server-Side Rendering mit Askama Templates
- HTMX kann für Tooltips verwendet werden (Progressive Enhancement)

---

## Offene Punkte / Fragen

- [ ] Icon-Auswahl: Stern ⭐ oder Alternative (z.B. Gabel/Messer 🍽️, Punkt 🔵)?
- [ ] Soll bei mehreren Rezepten am selben Tag der Tooltip alle Rezeptnamen auflisten oder nur die Anzahl zeigen?

---

**Letzte Aktualisierung:** 2026-03-30
