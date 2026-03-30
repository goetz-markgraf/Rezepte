: Story 34: Suche "Länger nicht gemacht" per Klick in der Wochenübersicht

**Epic:** Epic 5: Wochenplanung
**Priorität:** MVP Phase 3
**Status:** Offen

---

## 1. Story-Satz

Als **Haushaltsmitglied** möchte ich **in der Wochenübersicht per Klick auf ein Button die Suche "Länger nicht gemacht" aufrufen**, damit ich **leicht Lücken im Wochenplan mit Rezepten füllen kann, die ich schon lange nicht mehr gekocht habe**.

---

## 2. Geschäftsbezogene Details

### Kontext
Wenn ich die Wochenplanung erstelle, habe ich oft Lücken oder Tage, an denen ich noch keine Idee habe, was ich kochen möchte. Die Suche "Länger nicht gemacht" zeigt mir Rezepte, die ich schon lange nicht zubereitet habe und die mir vielleicht wieder schmecken könnten. Bisher muss ich dafür separat zur Suche navigieren. Ein direkter Zugriff aus der Wochenübersicht würde den Workflow erheblich beschleunigen.

### Nutzergruppe
Haushaltsmitglieder, die den Wochenplan verwalten und suchen nach Inspiration für fehlende Mahlzeiten.

### Business-Value
- Schnelleres Auffüllen des Wochenplans
- Bessere Nutzung der vorhandenen Suche "Länger nicht gemacht"
- Weniger Kontextwechsel beim Planen
- Mehr Abwechslung im Speiseplan durch bewusste Einbindung älterer Rezepte

### Edge Cases
- **Keine Rezepte vorhanden:** Falls die Suche "Länger nicht gemacht" keine Ergebnisse liefert (z.B. alle Rezepte wurden kürzlich gekocht), wird eine entsprechende Meldung angezeigt
- **Keine Wochenplanung vorhanden:** Der Button ist trotzdem sichtbar, da er unabhängig vom aktuellen Plan funktioniert
- **Kategorie-Filter aktiv:** Die Suche respektiert eventuell bereits gesetzte Filter in der Wochenübersicht

---

## 3. Akzeptanzkriterien

### Funktionale Kriterien

- [ ] **K1: Button in der Wochenübersicht**
  - In der Wochenübersicht wird ein Button/Link mit dem Label "Länger nicht gemacht" oder einem passenden Icon angezeigt
  - Der Button ist oberhalb oder neben dem Wochenplan platziert (z.B. in einer Toolbar)

- [ ] **K2: Klick öffnet Suche mit Filter**
  - Beim Klick auf den Button wird die Suche geöffnet
  - Die Suche ist bereits mit dem Filter "Länger nicht gemacht" vorbelegt
  - Die Ergebnisse werden nach dem letzten Kochdatum sortiert (älteste zuerst)

- [ ] **K3: Direkte Zuweisung zum Tag möglich**
  - Aus den Suchergebnissen kann ein Rezept direkt einem Tag in der aktuellen Woche zugewiesen werden
  - Alternativ: Ein Klick auf ein Rezept öffnet die Detailansicht mit Option zur Zuweisung

- [ ] **K4: Keine Datenverlust**
  - Bereits eingeplante Mahlzeiten bleiben beim Öffnen der Suche erhalten
  - Der aktuelle Wochenplan-Status wird nicht verändert

### Nicht-funktionale Kriterien

- [ ] **K5: Performance**
  - Der Button-Klick reagiert ohne spürbare Verzögerung (< 200ms)
  - Die Suchergebnisse laden innerhalb von 1 Sekunde

- [ ] **K6: Barrierefreiheit**
  - Der Button hat ein beschreibendes aria-label (z.B. "Rezepte anzeigen, die länger nicht gemacht wurden")
  - Der Button ist per Tastatur erreichbar und bedienbar

---

## 4. Technische Planung

### Datenmodell
Keine Änderungen am Datenmodell notwendig. Die bestehende Suchfunktionalität für "Länger nicht gemacht" wird wiederverwendet.

### UI/UX-Spezifikation
- **Platzierung:** Der Button wird in der Toolbar der Wochenübersicht platziert, neben anderen Aktionen wie "Woche exportieren" oder "Einkaufsliste"
- **Darstellung:** Als sekundärer Button mit Icon (z.B. 🕐 oder einem ähnlichen Zeit-Icon) und Text "Länger nicht gemacht"
- **Interaktion:** 
  1. User klickt auf den Button in der Wochenübersicht
  2. Die Suche wird in einem Modal oder auf einer neuen Seite geöffnet
  3. Die Suchergebnisse zeigen Rezepte, sortiert nach letztem Kochdatum (älteste zuerst)
  4. Jedes Rezept hat einen "Zuweisen"-Button oder ist klickbar für Details
  5. Nach der Zuweisung kehrt der User zur Wochenübersicht zurück

---

## 5. Nicht-funktionale Anforderungen

### Performance
- Seite lädt ohne sichtbare Verzögerung (< 500ms)
- Suchergebnisse werden serverseitig berechnet und innerhalb von 1 Sekunde angezeigt
- HTMX für asynchrones Nachladen der Suchergebnisse

### Browser-Support
- Aktuelle Chrome, Firefox, Safari, Edge Versionen
- iOS Safari und Android Chrome (letzte 2 Versionen)

### Barrierefreiheit
- WCAG 2.1 Level A konform
- Fokus-Indikatoren sichtbar
- Tastatur-Navigation vollständig

---

## 6. Teststrategie

### E2E-Tests (Playwright)

**Testfall 1: Button ist in der Wochenübersicht sichtbar**
```gherkin
Given ich bin auf der Wochenübersicht-Seite
Then sehe ich einen Button "Länger nicht gemacht"
```

**Testfall 2: Klick öffnet Suche mit vorbelegtem Filter**
```gherkin
Given ich bin auf der Wochenübersicht-Seite
When ich auf den Button "Länger nicht gemacht" klicke
Then wird die Suche geöffnet
And der Filter "Länger nicht gemacht" ist aktiviert
And die Ergebnisse sind nach letztem Kochdatum sortiert
```

**Testfall 3: Zuweisung eines Rezepts zum Wochenplan**
```gherkin
Given ich habe die Suche "Länger nicht gemacht" geöffnet
And es werden Rezepte angezeigt
When ich auf "Zuweisen" für ein Rezept klicke
And ich einen Tag in der aktuellen Woche auswähle
Then wird das Rezept diesem Tag zugewiesen
And ich kehre zur Wochenübersicht zurück
And das Rezept ist im Wochenplan sichtbar
```

**Testfall 4: Leere Suchergebnisse**
```gherkin
Given alle Rezepte wurden in den letzten 7 Tagen gekocht
When ich auf den Button "Länger nicht gemacht" klicke
Then wird eine Meldung angezeigt "Keine Rezepte gefunden, die länger nicht gemacht wurden"
```

---

## 7. Abhängigkeiten & Rahmenbedingungen

### Abhängigkeiten
- Story 1: Grundlegende Rezeptverwaltung (muss implementiert sein)
- Story X: Suchfunktion "Länger nicht gemacht" (muss bereits existieren)
- Epic 5: Wochenplanung (Wochenübersicht muss vorhanden sein)

### Rahmenbedingungen
- SQLite-Datenbank muss existieren und erreichbar sein
- Keine Authentifizierung erforderlich (LAN-only)
- Die bestehende Suchfunktionalität wird wiederverwendet

---

## Offene Punkte / Fragen

- [ ] Soll die Zuweisung direkt aus der Suchliste möglich sein oder erst nach Öffnen der Rezept-Details?
- [ ] Soll es eine Möglichkeit geben, mehrere Rezepte auf einmal zuzuweisen?
- [ ] Welches Icon soll für den Button verwendet werden?

---

**Letzte Aktualisierung:** 2026-03-30
