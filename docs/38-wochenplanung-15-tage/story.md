# Story 38: Wochenplanung auf 15-Tage-Liste umbauen

**Epic:** Epic 5: Wochenplanung
**Priorität:** MVP Phase 2
**Status:** In Arbeit

---

## 1. Story-Satz

Als Nutzer der Rezepte-App möchte ich eine kontinuierliche Liste der nächsten 15 Tage sehen, statt einer wochenbasierten Ansicht, damit ich meine Essensplanung flexibler und übersichtlicher gestalten kann.

---

## 2. Geschäftsbezogene Details

### Kontext
Die aktuelle Wochenplanung zeigt Rezepte in einer wochenbasierten Ansicht (Montag bis Sonntag) mit Navigation über Pfeil-Buttons. Die Nutzer haben festgestellt, dass diese starre Wochenstruktur nicht optimal für ihre Planungsbedürfnisse ist. Die neue 15-Tage-Ansicht bietet mehr Flexibilität und zeigt direkt ab dem aktuellen Tag die kommenden 15 Tage als fortlaufende Liste an, was die Planung intuitiver macht.

### Nutzergruppe
Der Haushalt (beide Partner), die die App wöchentlich für die Essensplanung nutzen.

### Business-Value
- Bessere Übersicht über den kommenden Planungszeitraum
- Eliminierung unnötiger Navigation (keine Blättern-Buttons mehr nötig)
- Natürlichere Planung ohne künstliche Wochengrenzen
- Schnellere Erfassung des aktuellen Planungsstands

### Edge Cases
- **Keine geplanten Rezepte:** Die Liste zeigt trotzdem die nächsten 15 Tage mit leeren Slots an
- **Heute ist ein Feiertag/Wochenende:** Die Liste beginnt trotzdem am aktuellen Tag
- **Zeitzone/Datumsgrenze:** Die Liste basiert auf dem Server- oder Client-Datum zur Zeit des Aufrufs
- **Mehrere Rezepte an einem Tag:** Alle geplanten Rezepte eines Tages werden unter dem Datum gruppiert

---

## 3. Akzeptanzkriterien

### Funktionale Kriterien

- [ ] **K1: 15-Tage-Liste anzeigen**
  - Die Wochenplanung zeigt eine fortlaufende Liste der nächsten 15 Tage beginnend mit dem aktuellen Tag
  - Jeder Tag wird mit Datum und Wochentag angezeigt (z.B. "Fr, 04.04.2026")
  - Geplante Rezepte werden unter dem jeweiligen Datum angezeigt

- [ ] **K2: Navigation entfernen**
  - Die Zeile mit den Buttons "Vorherige Woche" und "Nächste Woche" entfällt komplett
  - Die Liste ist statisch und zeigt immer "ab heute" die nächsten 15 Tage

- [ ] **K3: Darstellung pro Tag**
  - Pro Tag werden alle geplanten Rezepte angezeigt
  - Falls ein Tag keine geplanten Rezepte hat, wird der Tag trotzdem angezeigt (leerer Slot)
  - Die Darstellung ist kompakt und scroll-freundlich

- [ ] **K4: Datumsklick-Verhalten**
  - Klick auf ein Datum öffnet die Detailansicht für diesen Tag (wie bisher, falls vorhanden)
  - Alternativ kann ein Klick auf ein Datum ohne geplante Rezepte zur Rezeptsuche führen

### Nicht-funktionale Kriterien

- [ ] **K5: Performance**
  - Die Liste lädt in weniger als 1 Sekunde
  - Scrollen innerhalb der Liste ist flüssig

- [ ] **K6: Barrierefreiheit**
  - Die Liste ist mit der Tastatur navigierbar
  - Datumsangaben sind für Screenreader verständlich
  - Ausreichender Kontrast zwischen Datum und Hintergrund

---

## 4. Technische Planung

### Datenmodell
Keine Änderungen am Datenmodell erforderlich. Das bestehende `planned_date`-Feld wird weiterhin verwendet.

### UI/UX-Spezifikation
- Vertikale Liste statt horizontaler Wochenübersicht
- Jedes Listenelement zeigt: Wochentag + Datum, darunter geplante Rezepte (falls vorhanden)
- Kompakte Darstellung, keine aufwendigen Karten
- Responsive: Am Desktop nebeneinander, mobil untereinander (je nach Design)

---

## 5. Nicht-funktionale Anforderungen

### Performance
- Seite lädt ohne sichtbare Verzögerung (< 500ms)
- Die 15-Tage-Liste rendert sofort

### Browser-Support
- Aktuelle Chrome, Firefox, Safari, Edge Versionen
- iOS Safari und Android Chrome (letzte 2 Versionen)

### Barrierefreiheit
- WCAG 2.1 Level A konform
- Fokus-Indikatoren sichtbar
- Semantische HTML-Struktur für die Liste

---

## 6. Teststrategie

### E2E-Tests (Playwright)

**Testfall 1: 15-Tage-Liste wird korrekt angezeigt**
```gherkin
Given der Nutzer ist auf der Wochenplanung-Seite
When die Seite lädt
Then werden die nächsten 15 Tage ab dem aktuellen Datum angezeigt
And der aktuelle Tag ist das erste Element der Liste
```

**Testfall 2: Geplante Rezepte werden unter dem korrekten Datum angezeigt**
```gherkin
Given ein Rezept ist für morgen geplant
When der Nutzer die Wochenplanung öffnet
Then wird das Rezept unter dem morgigen Datum angezeigt
```

**Testfall 3: Navigation ist nicht mehr vorhanden**
```gherkin
Given der Nutzer ist auf der Wochenplanung-Seite
When die Seite vollständig geladen ist
Then sind keine Buttons "Vorherige Woche" oder "Nächste Woche" sichtbar
```

**Testfall 4: Tage ohne geplante Rezepte werden angezeigt**
```gherkin
Given es gibt keine geplanten Rezepte
When der Nutzer die Wochenplanung öffnet
Then werden trotzdem alle 15 Tage mit ihren Daten angezeigt
```

---

## 7. Abhängigkeiten & Rahmenbedingungen

### Abhängigkeiten
- Story 18 (Wochenvorschau für geplante Rezepte) muss implementiert sein
- Story 28 (Datum-Eingabe am Rezept) muss implementiert sein

### Rahmenbedingungen
- SQLite-Datenbank muss existieren und erreichbar sein
- Keine Authentifizierung erforderlich (LAN-only)
- Die Änderung ist rein UI-bezogen, keine Backend-Änderungen nötig

---

**Letzte Aktualisierung:** 2026-04-04
