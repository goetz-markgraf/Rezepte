# Story 33: Wochenübersicht Navigation mit Pfeiltasten

**Epic:** Epic 5: Wochenplanung
**Priorität:** MVP Phase 5
**Status:** In Arbeit

---

## 1. Story-Satz

Als **Nutzer der Wochenplanung** möchte ich **mit Pfeiltasten (< und >) wochenweise durch die Wochenübersicht blättern können**, damit ich **schnell und bequem vergangene und zukünftige Wochen einsehen kann**, ohne den Wochen-Picker öffnen zu müssen.

---

## 2. Geschäftsbezogene Details

### Kontext
Die Wochenübersicht ist bereits implementiert (Stories 18, 19, 20, 29, 30) und zeigt geplante Rezepte für die aktuelle Woche an. Aktuell muss der Nutzer den Wochen-Picker öffnen, um eine andere Woche zu sehen. Eine direkte Navigation mit Pfeiltasten würde den Workflow erheblich beschleunigen, besonders wenn man mehrere Wochen zurück oder voraus schauen möchte.

### Nutzergruppe
Die beiden Partner, die gemeinsam die Rezepte-App im LAN nutzen.

### Business-Value
- **Effizienz**: Schneller Zugriff auf andere Wochen ohne Umweg über den Wochen-Picker
- **Benutzerfreundlichkeit**: Intuitive Navigation, die dem bekannten Muster von Kalender-Apps entspricht
- **Zeitersparnis**: Besonders nützlich beim Planen über mehrere Wochen hinweg

### Edge Cases
- **Erste Woche mit Daten**: Wenn es keine geplanten Rezepte in früheren Wochen gibt, sollte die Navigation trotzdem funktionieren (leere Wochen anzeigen)
- **Weit in die Zukunft**: Navigation sollte unbegrenzt in die Zukunft möglich sein
- **Aktuelle Woche als Start**: Beim ersten Aufruf der Wochenübersicht soll immer die aktuelle Woche angezeigt werden
- **DeepLink-Kompatibilität**: URLs mit Wochen-Parameter sollten direkt funktionieren

---

## 3. Akzeptanzkriterien

### Funktionale Kriterien

- [ ] **K1: Navigation mit Pfeiltasten**
  - Ein Pfeil nach links (<) navigiert zur vorherigen Woche
  - Ein Pfeil nach rechts (>) navigiert zur nächsten Woche
  - Die Navigation erfolgt ohne Seitenreload (HTMX)
  - Nur die Wochenübersicht wird aktualisiert, nicht die gesamte Seite

- [ ] **K2: Anzeige des aktuellen Zeitraums**
  - Zwischen den Pfeiltasten wird der aktuell angezeigte Zeitraum angezeigt (z.B. "13.01. - 19.01.2025" oder "KW 2")
  - Die aktuelle Woche wird optisch hervorgehoben, wenn sie angezeigt wird

- [ ] **K3: Standardverhalten beim Öffnen**
  - Beim Aufruf der Wochenübersicht wird immer die aktuelle Woche angezeigt
  - Die URL enthält optional einen Parameter für die Woche (z.B. `?week=2025-W02`)

- [ ] **K4: URL-Updates**
  - Bei Navigation per Pfeiltaste wird die URL im Browser aktualisiert (History API)
  - DeepLinks zu bestimmten Wochen funktionieren direkt

### Nicht-funktionale Kriterien

- [ ] **K5: Performance**
  - Navigation zu einer anderen Woche lädt in < 300ms
  - Keine sichtbaren Layout-Shifts während des Ladens

- [ ] **K6: Barrierefreiheit**
  - Pfeiltasten haben aussagekräftige aria-labels (z.B. "Vorherige Woche", "Nächste Woche")
  - Die Navigation ist per Tastatur bedienbar
  - Der aktuelle Zeitraum wird für Screenreader als live-region markiert

---

## 4. Technische Planung

### Datenmodell
Keine Änderungen am Datenmodell erforderlich. Die bestehenden Tabellen für `recipes` mit den Feldern `planned_at` werden verwendet.

### UI/UX-Spezifikation

**Layout der Navigationsleiste:**
```
[ < ]  [ Zeitraum: 13.01. - 19.01.2025 ]  [ > ]
```

**Positionierung:**
- Die Navigation befindet sich über der Wochenübersicht
- Zentrierte Ausrichtung der gesamten Navigationsleiste
- Pfeiltasten als Buttons mit Icons oder Unicode-Zeichen (< / >)
- Der Zeitraum-Text ist zwischen den Pfeilen zentriert

**Interaktionen:**
- Klick auf < lädt die vorherige Woche per HTMX-Request
- Klick auf > lädt die nächste Woche per HTMX-Request
- Die Wochenübersicht wird im gleichen Container ersetzt
- URL wird via History API aktualisiert

**URL-Schema:**
- Standard: `/week-overview` zeigt aktuelle Woche
- Mit Parameter: `/week-overview?week=2025-W02` zeigt spezifische Woche

---

## 5. Nicht-funktionale Anforderungen

### Performance
- Seite lädt ohne sichtbare Verzögerung (< 300ms für Wochenwechsel)
- HTMX-Request sollte nur die notwendigen Daten laden

### Browser-Support
- Aktuelle Chrome, Firefox, Safari, Edge Versionen
- iOS Safari und Android Chrome (letzte 2 Versionen)
- HTMX muss aktiviert sein (Progressive Enhancement: ohne JS funktioniert der Seitenaufruf mit week-Parameter)

### Barrierefreiheit
- WCAG 2.1 Level A konform
- Fokus-Indikatoren sichtbar auf den Pfeil-Buttons
- aria-live Region für den Zeitraum-Text
- Tastatur-Navigation funktioniert vollständig

---

## 6. Teststrategie

### E2E-Tests (Playwright)

**Testfall 1: Navigation zur vorherigen Woche**
```gherkin
Given ich befinde mich auf der Wochenübersicht
And die aktuelle Woche wird angezeigt
When ich auf den linken Pfeil (<) klicke
Then wird die vorherige Woche angezeigt
And die URL enthält den Parameter für die vorherige Woche
```

**Testfall 2: Navigation zur nächsten Woche**
```gherkin
Given ich befinde mich auf der Wochenübersicht
And die aktuelle Woche wird angezeigt
When ich auf den rechten Pfeil (>) klicke
Then wird die nächste Woche angezeigt
And die URL enthält den Parameter für die nächste Woche
```

**Testfall 3: Standardverhalten beim Öffnen**
```gherkin
Given ich rufe die Wochenübersicht ohne Parameter auf
Then wird die aktuelle Woche angezeigt
And die Navigation zeigt den aktuellen Zeitraum an
```

**Testfall 4: DeepLink zu spezifischer Woche**
```gherkin
Given ich rufe die Wochenübersicht mit Parameter ?week=2025-W01 auf
Then wird die Woche 1 des Jahres 2025 angezeigt
And die Navigation zeigt den Zeitraum für KW 1 an
```

**Testfall 5: Mehrfache Navigation**
```gherkin
Given ich befinde mich auf der Wochenübersicht
When ich dreimal auf den linken Pfeil klicke
Then wird die Woche von vor 3 Wochen angezeigt
And die Navigation funktioniert weiterhin für vorherige/nächste Woche
```

---

## 7. Abhängigkeiten & Rahmenbedingungen

### Abhängigkeiten
- Story 18 (Wochenvorschau für geplante Rezepte) - muss implementiert sein ✓
- Story 19 (Wochenvorschau nach Wochentagen formatiert) - muss implementiert sein ✓
- Story 20 ("Heute gekocht" Ansicht mit Highlight) - muss implementiert sein ✓
- Story 29 (Wochen-Picker erweitern) - muss implementiert sein ✓
- Story 30 (Wochenpicker zeigt geplantes Essen) - muss implementiert sein ✓

### Rahmenbedingungen
- SQLite-Datenbank muss existieren und erreichbar sein
- Keine Authentifizierung erforderlich (LAN-only)
- HTMX muss im Frontend verfügbar sein

---

## Offene Punkte / Fragen

- [ ] Soll es eine "Heute"-Schaltfläche geben, um schnell zur aktuellen Woche zurückzukehren?
- [ ] Welches Datumsformat soll für den Zeitraum verwendet werden? (13.01. - 19.01. vs. "KW 2, 2025")

---

**Letzte Aktualisierung:** 2026-03-30

---

## Zusatzinformationen

In der Wochenübersicht möchte ich mit < und > Pfeiltasten wochenweise blättern können. Wenn die Übersicht aufgerufen wird, soll es weiterhin immer mit der aktuellen Woche beginnen.
