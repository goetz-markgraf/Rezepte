# Story 7: Volltextsuche über Titel, Zutaten und Anleitung

**Epic:** Suche & Filterung
**Priorität:** MVP - Phase 1
**Status:** In Arbeit

---

## 1. Story-Satz

**Als** Benutzer möchte ich **nach einem Stichwort suchen und dabei automatisch alle passenden Rezepte in Titel, Zutaten und Anleitung finden**, damit ich **schnell das richtige Rezept finde, ohne zu wissen, wo genau das Wort steht**.

---

## 2. Geschäftsbezogene Details

### Kontext

Bei der Wochenplanung entsteht oft folgende Situation: Man möchte ein bestimmtes Gericht kochen, weiß aber nicht mehr genau, unter welchem Titel es gespeichert ist, oder sucht nach Rezepten mit einer bestimmten Zutat (z.B. "Hähnchen", "Vollkornmehl"). Das Durchscrollen der alphabetischen Liste ist mühsam und liefert keine Treffer bei inhaltlichen Übereinstimmungen in Zutaten oder Anleitung.

Die Volltextsuche durchsucht alle drei Felder gleichzeitig und zeigt sofort eine gefilterte Trefferliste — ohne dass die Seite neu geladen werden muss.

### Nutzergruppe

- Beide Partner des Haushalts gleichberechtigt
- Zugriff über Desktop, Tablet und Handy im LAN

### Business-Value

- Ermöglicht schnelles Finden von Rezepten nach Zutaten, Methoden oder Stichworten
- Reduziert die Zeit bei der Wochenplanung erheblich
- Macht auch Rezepte findbar, die nach Titel allein nicht gesucht werden würden

### Edge Cases

- **Leere Suche:** Die vollständige ungefilterte Rezeptliste wird angezeigt (keine Fehlermeldung)
- **Keine Treffer:** Klare Meldung "Keine Rezepte gefunden" statt leerer Seite
- **Sonderzeichen / Umlaute:** Suche nach "Äpfel" findet auch "Äpfel" korrekt (Umlaut-Unterstützung)
- **Leerzeichen / mehrere Wörter:** Suche nach "Dinkel Brot" findet Rezepte, die beide Wörter enthalten
- **Groß-/Kleinschreibung:** Suche ist case-insensitive ("BOLOGNESE" = "bolognese")
- **Sehr kurze Suchbegriffe:** Suche ab 1 Zeichen ist möglich, aber erst ab 2 Zeichen sinnvoll (kein Minimum erzwungen)
- **Sonderzeichen ohne Bedeutung:** Eingabe von "#", "%" etc. führt zu keinem Fehler, einfach keine Treffer

---

## 3. Akzeptanzkriterien

### Funktionale Kriterien

- [ ] **K1: Suchfeld sichtbar und erreichbar**
  - Ein Suchfeld ist auf der Rezept-Übersichtsseite dauerhaft sichtbar
  - Das Feld hat einen beschreibenden Platzhaltertext (z.B. "Rezepte durchsuchen...")
  - Das Suchfeld ist von allen Geräten gut erreichbar (Desktop, Tablet, Handy)

- [ ] **K2: Suche über alle drei Felder**
  - Ein eingegebener Begriff wird gleichzeitig in Titel, Zutaten und Anleitung gesucht
  - Treffer aus allen drei Feldern erscheinen in der Ergebnisliste
  - Ein Rezept erscheint nur einmal in der Ergebnisliste, auch wenn der Begriff in mehreren Feldern vorkommt

- [ ] **K3: Ergebnisliste wird gefiltert**
  - Die Rezeptliste aktualisiert sich ohne Seitenneuladung beim Tippen (oder nach kurzer Verzögerung)
  - Nur Rezepte mit mindestens einem Treffer werden angezeigt
  - Die Ergebnisse sind alphabetisch sortiert (wie die Standardliste)

- [ ] **K4: Groß-/Kleinschreibung ignoriert**
  - Suche nach "bolognese", "Bolognese" und "BOLOGNESE" liefert dieselben Ergebnisse

- [ ] **K5: Leere Suche zeigt alle Rezepte**
  - Wird das Suchfeld geleert (oder ist von Beginn an leer), werden alle Rezepte angezeigt
  - Kein Reload der Seite notwendig

- [ ] **K6: Keine Treffer**
  - Wenn kein Rezept zum Suchbegriff passt, erscheint eine Meldung wie "Keine Rezepte für '[Begriff]' gefunden"
  - Die Meldung ist klar und freundlich formuliert

- [ ] **K7: Suchbegriff bleibt sichtbar**
  - Der eingegebene Suchbegriff bleibt im Suchfeld erhalten, solange die Benutzer auf der Seite sind
  - Beim Zurücknavigieren aus der Detailansicht bleibt der Suchbegriff erhalten (oder die URL spiegelt den Suchbegriff wider)

### Nicht-funktionale Kriterien

- [ ] **K8: Performance**
  - Suchergebnisse erscheinen in < 1 Sekunde bei bis zu 200 Rezepten
  - Das Suchfeld blockiert die Seite nicht (keine UI-Hänger)

- [ ] **K9: Barrierefreiheit**
  - Das Suchfeld hat ein korrektes Label (für Screenreader)
  - Die Ergebnisliste wird nach jeder Sucheingabe für Screenreader angekündigt (ARIA live region)
  - Tastatur-Navigation: Suchfeld per Tab erreichbar, Suche ohne Mausklick bedienbar

---

## 4. Technische Planung

### Datenmodell

Kein neues Datenbankfeld erforderlich. Die Suche nutzt die vorhandenen Felder:
- `title` (TEXT)
- `ingredients` (TEXT)
- `instructions` (TEXT)

### UI/UX-Spezifikation

**Platzierung:**
- Das Suchfeld befindet sich oben auf der Rezept-Übersichtsseite, gut sichtbar vor der Rezeptliste

**Verhalten:**
- Eingabe im Suchfeld filtert die angezeigte Liste ohne Seitenneuladung (HTMX oder direktes Neuladen via Form-Submit)
- Bei leerem Feld: vollständige Liste sichtbar
- DeepLink-fähig: Der Suchbegriff steht als Query-Parameter in der URL (`?q=bolognese`), sodass Suchergebnisse verlinkt werden können

**Anzeige der Ergebnisse:**
- Gleiche Kartenansicht wie die normale Rezeptliste
- Bei keinen Treffern: Hinweistext statt leerer Liste

---

## 5. Nicht-funktionale Anforderungen

### Performance
- Volltextsuche liefert Ergebnisse in < 1 Sekunde (NFR-P4 aus PRD)
- Keine Indexierung notwendig für den MVP-Umfang (bis ca. 200 Rezepte)

### Browser-Support
- Aktuelle Chrome, Firefox, Safari, Edge Versionen
- iOS Safari und Android Chrome (letzte 2 Versionen)

### Barrierefreiheit
- WCAG 2.1 Level A konform
- Suchfeld mit `<label>` versehen
- Fokus-Indikatoren sichtbar
- Ergebnisbereich als ARIA live region (bei HTMX-Ansatz)

---

## 6. Teststrategie

### E2E-Tests (Playwright)

**Testfall 1: Suche nach Titel**
```gherkin
Given Die App enthält ein Rezept mit Titel "Spaghetti Bolognese"
And Die App enthält ein Rezept mit Titel "Pfannkuchen"
When Benutzer gibt "Bolognese" in das Suchfeld ein
Then Nur "Spaghetti Bolognese" wird in der Liste angezeigt
And "Pfannkuchen" wird nicht angezeigt
```

**Testfall 2: Suche nach Zutat**
```gherkin
Given Die App enthält ein Rezept mit Zutat "Dinkelvollkornmehl"
And Die App enthält ein Rezept ohne "Dinkel" in Zutaten oder Titel
When Benutzer gibt "Dinkel" in das Suchfeld ein
Then Das Rezept mit "Dinkelvollkornmehl" wird angezeigt
And Das andere Rezept wird nicht angezeigt
```

**Testfall 3: Keine Treffer**
```gherkin
Given Die App enthält Rezepte
When Benutzer gibt "xyzxyzxyz" in das Suchfeld ein
Then Eine Meldung "Keine Rezepte gefunden" wird angezeigt
And Die Liste ist leer
```

**Testfall 4: Leere Suche zeigt alle Rezepte**
```gherkin
Given Benutzer hat "Bolognese" gesucht und sieht gefilterte Ergebnisse
When Benutzer löscht den Suchbegriff aus dem Suchfeld
Then Alle Rezepte werden wieder angezeigt
```

**Testfall 5: Groß-/Kleinschreibung**
```gherkin
Given Die App enthält ein Rezept mit Titel "Spaghetti Bolognese"
When Benutzer gibt "BOLOGNESE" in das Suchfeld ein
Then "Spaghetti Bolognese" wird in der Ergebnisliste angezeigt
```

**Testfall 6: Suche nach Anleitung**
```gherkin
Given Die App enthält ein Rezept mit "im Ofen backen" in der Anleitung
When Benutzer gibt "Ofen" in das Suchfeld ein
Then Das entsprechende Rezept wird angezeigt
```

---

## 7. Abhängigkeiten & Rahmenbedingungen

### Abhängigkeiten

- Story 5 (Rezept-Liste alphabetisch sortiert) muss implementiert sein — die Suche filtert diese Liste
- Keine weiteren Abhängigkeiten

### Rahmenbedingungen

- SQLite-Datenbank muss existieren und erreichbar sein
- Keine Authentifizierung erforderlich (LAN-only)
- DeepLink-fähige URLs mit Query-Parametern (Architektur-Vorgabe)

---

## Offene Punkte / Fragen

- [ ] Soll die Suche direkt beim Tippen (live) oder erst nach Drücken von Enter/Button auslösen?
- [ ] Sollen Treffer in der Ergebnisliste visuell hervorgehoben werden (Stichwort fett/markiert)?

---

**Letzte Aktualisierung:** 2026-03-28
