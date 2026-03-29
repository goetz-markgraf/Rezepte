# Story 11: Filter nach Bewertung (Beliebtheit)

**Epic:** Suche & Filterung
**Priorität:** MVP - Phase 1
**Status:** In Arbeit

---

## 1. Story-Satz

**Als** Benutzer möchte ich **die Rezeptliste nach Bewertung filtern**, damit ich **schnell nur gut bewertete Rezepte sehe und schlechte oder negativ bewertete Rezepte bei der Wochenplanung ausblenden kann**.

---

## 2. Geschäftsbezogene Details

### Kontext

Nachdem Rezepte mit Sternen bewertet wurden (Story 14), soll es möglich sein, diese Bewertungen als Filter zu nutzen. Die typische Situation: Man plant die Woche und möchte gezielt aus den bewährten, gut bewerteten Rezepten wählen — ohne von schlechten Erfahrungen oder noch unbewerteten Rezepten abgelenkt zu werden.

Besonders relevant ist der Ausschluss von Negativ-Bewertungen (1-2 Sterne): Rezepte, die das Paar enttäuscht haben, sollen nicht mehr in der Planungsansicht auftauchen, ohne dass sie gelöscht werden müssen. Die Bewertung dient so als dauerhafter "Ich koche das nicht nochmal"-Marker.

Ergänzend kann man gezielt nur die Favoriten (5 Sterne) oder alle gut bewerteten Rezepte (3+ Sterne) anzeigen lassen.

### Nutzergruppe

- Beide Partner des Haushalts gleichberechtigt
- Zugriff über Desktop, Tablet und Handy im LAN
- Primär bei der Wochenplanung (typisch Mittwoch/Donnerstag)

### Business-Value

- Ermöglicht gezielte Wochenplanung aus dem bewährten Repertoire (nur 3+ Sterne)
- Schließt misslungene Rezepte dauerhaft aus der Planung aus, ohne sie zu löschen
- Macht 5-Sterne-Favoriten auf Knopfdruck sichtbar — schnelle Auswahl bei wenig Zeit
- Kombiniert mit dem Kategorie-Filter (Story 8) entsteht z.B. "Meine liebsten Mittagessen"

### Edge Cases

- **Kein Rezept erfüllt den Filter:** Klare Meldung, dass keine Rezepte mit dieser Bewertung vorhanden sind (statt leerer Seite)
- **Alle Rezepte unbwertet:** Bei aktiven Filtern "3+", "4+" oder "5 Sterne" erscheint kein Ergebnis — Hinweistext erklären, dass keine Bewertungen vorhanden sind
- **Filter zurücksetzen:** Einfache Möglichkeit, den Bewertungsfilter aufzuheben und zur vollständigen Liste zurückzukehren
- **Kombination mit anderen Filtern:** Bewertungsfilter kombiniert sich mit Kategorie-Filter und Volltextsuche (AND-Logik)
- **Sortierung:** Innerhalb des gefilterten Ergebnisses wird alphabetisch sortiert (Standard), es sei denn ein anderer Sortierfilter wie "Länger nicht gemacht" ist aktiv

---

## 3. Akzeptanzkriterien

### Funktionale Kriterien

- [ ] **K1: Bewertungsfilter sichtbar und auswählbar**
  - Auf der Rezept-Übersichtsseite gibt es Filtermöglichkeiten für Bewertungen
  - Folgende Optionen stehen zur Wahl:
    - "Nur Gute" (3 Sterne und mehr, d.h. Rating >= 3) — schließt 1-2 Sterne und unbewertete aus
    - "Favoriten" (5 Sterne) — zeigt nur Top-Rezepte
  - Genau eine Option kann aktiv sein (oder keine — dann kein Bewertungsfilter aktiv)

- [ ] **K2: Filter "Nur Gute" (3+ Sterne)**
  - Bei aktivem Filter werden nur Rezepte mit Bewertung >= 3 Sterne angezeigt
  - Rezepte mit 1-2 Sternen sind ausgeblendet
  - Rezepte ohne Bewertung (NULL) sind ausgeblendet
  - Die gefilterte Liste ist alphabetisch sortiert

- [ ] **K3: Filter "Favoriten" (5 Sterne)**
  - Bei aktivem Filter werden nur Rezepte mit exakt 5 Sternen angezeigt
  - Alle anderen Rezepte (1-4 Sterne, unbwertet) sind ausgeblendet
  - Die gefilterte Liste ist alphabetisch sortiert

- [ ] **K4: Aktiver Filter visuell erkennbar**
  - Der aktive Bewertungsfilter ist klar als ausgewählt hervorgehoben
  - Kein Filter aktiv = alle Filteroptionen im Standardzustand

- [ ] **K5: Filter zurücksetzen**
  - Ein Klick auf den bereits aktiven Bewertungsfilter hebt ihn auf
  - Nach dem Zurücksetzen wird die vollständige Rezeptliste wieder angezeigt
  - Kein Bewertungsfilter ist mehr aktiv

- [ ] **K6: Keine Treffer**
  - Wenn kein Rezept den Filterbedingungen entspricht, erscheint eine klare, freundliche Meldung (z.B. "Keine Rezepte mit dieser Bewertung gefunden")
  - Die Seite zeigt keinen leeren Bereich ohne Erklärung

- [ ] **K7: DeepLink-fähige URL**
  - Der aktive Bewertungsfilter wird als Query-Parameter in der URL abgebildet
    - "Nur Gute": `?bewertung=gut` (oder ähnlich)
    - "Favoriten": `?bewertung=favoriten`
  - Das direkte Aufrufen einer solchen URL zeigt die gefilterte Ansicht korrekt an
  - Die URL kann als Lesezeichen gespeichert werden und funktioniert beim nächsten Aufruf

- [ ] **K8: Kombination mit Kategorie-Filter**
  - Wenn gleichzeitig ein Kategorie-Filter aktiv ist, werden nur Rezepte angezeigt, die beide Bedingungen erfüllen (AND-Logik)
  - Beide aktiven Filter sind gleichzeitig in der URL sichtbar

- [ ] **K9: Kombination mit Volltextsuche**
  - Wenn gleichzeitig ein Suchbegriff aktiv ist, werden nur Rezepte angezeigt, die sowohl den Suchbegriff als auch die Bewertungsbedingung erfüllen
  - Beide aktiven Parameter sind gleichzeitig in der URL sichtbar

- [ ] **K10: Kombination mit "Länger nicht gemacht"**
  - Wenn der "Länger nicht gemacht"-Filter gleichzeitig aktiv ist, werden nur Rezepte mit der geforderten Bewertung nach Datum sortiert angezeigt
  - Typischer Use-Case: "Zeig mir meine guten Rezepte, die ich lange nicht mehr gemacht habe"

### Nicht-funktionale Kriterien

- [ ] **K11: Performance**
  - Bewertungsfilter liefert Ergebnisse in < 1 Sekunde (NFR-P1)
  - Keine sichtbare Verzögerung beim Umschalten des Filters

- [ ] **K12: Barrierefreiheit**
  - Bewertungsfilter-Elemente sind per Tastatur bedienbar (Tab + Enter/Space)
  - Aktiver Filter-Status wird für Screenreader korrekt kommuniziert (ARIA-Attribute, z.B. `aria-pressed`)
  - Alle Filter-Elemente haben beschreibende Labels (WCAG 2.1 Level A)

---

## 4. Technische Planung

### Datenmodell

Kein neues Datenbankfeld erforderlich. Das vorhandene `rating`-Feld (INTEGER NULL, Werte 1-5) in der `recipes`-Tabelle wird für die Filterung genutzt (implementiert in Story 14).

Filterlogik auf Datenbankebene:
- "Nur Gute" (>= 3 Sterne): `WHERE rating >= 3`
- "Favoriten" (5 Sterne): `WHERE rating = 5`

Der Bewertungsfilter wird als zusätzliche SQL-Bedingung an die bestehende Abfrage angehängt, kombinierbar mit Kategorien, Suchbegriff und Datumsfilter.

### UI/UX-Spezifikation

**Platzierung:**
- Die Bewertungsfilter befinden sich auf der Rezept-Übersichtsseite, im gleichen Bereich wie die Kategorie-Filter und der "Länger nicht gemacht"-Filter

**Darstellung:**
- Klickbare Buttons oder Chips, konsistent mit dem Stil der übrigen Filter
- Beschriftung klar und verständlich: "Nur Gute" und "Favoriten"
- Sterne-Symbole können die Labels ergänzen (z.B. "★★★+ Nur Gute", "★★★★★ Favoriten")
- Aktiver Zustand klar erkennbar (hervorgehoben), inaktiver Zustand im Standardstil

**Verhalten:**
- Klick auf einen inaktiven Bewertungsfilter aktiviert ihn (und deaktiviert einen eventuell zuvor aktiven)
- Klick auf den bereits aktiven Filter deaktiviert ihn (Toggle)
- DeepLink-fähig: Aktiver Filter steht als Query-Parameter in der URL
- Liste wird sofort aktualisiert (Server-Side Rendering mit Redirect oder HTMX)

**Anzeige der Ergebnisse:**
- Gleiche Kartenansicht wie die normale Rezeptliste inkl. Sterne-Anzeige auf den Karten
- Bei keinen Treffern: Hinweistext statt leerer Liste

---

## 5. Nicht-funktionale Anforderungen

### Performance
- Bewertungsfilter liefert Ergebnisse in < 1 Sekunde (NFR-P1 aus PRD)
- Kein separater Index notwendig für den MVP-Umfang (bis ca. 200 Rezepte)

### Browser-Support
- Aktuelle Chrome, Firefox, Safari, Edge Versionen
- iOS Safari und Android Chrome (letzte 2 Versionen)

### Barrierefreiheit
- WCAG 2.1 Level A konform
- Filter-Buttons haben aussagekräftige Labels
- Aktiver Zustand wird per ARIA kommuniziert (`aria-pressed`)
- Fokus-Indikatoren sichtbar

---

## 6. Teststrategie

### E2E-Tests (Playwright)

**Testfall 1: Filter "Nur Gute" — zeigt nur 3+ Sterne**
```gherkin
Given Die App enthält "Spaghetti Bolognese" mit 4 Sternen
And Die App enthält "Pfannkuchen" mit 2 Sternen
And Die App enthält "Pizza" ohne Bewertung
When Benutzer klickt auf "Nur Gute"
Then "Spaghetti Bolognese" wird angezeigt
And "Pfannkuchen" wird nicht angezeigt (zu niedrige Bewertung)
And "Pizza" wird nicht angezeigt (keine Bewertung)
And Die URL enthält den Parameter "bewertung=gut"
```

**Testfall 2: Filter "Favoriten" — zeigt nur 5 Sterne**
```gherkin
Given Die App enthält "Omas Apfelkuchen" mit 5 Sternen
And Die App enthält "Nudelsuppe" mit 4 Sternen
And Die App enthält "Rührei" mit 3 Sternen
When Benutzer klickt auf "Favoriten"
Then Nur "Omas Apfelkuchen" wird angezeigt
And "Nudelsuppe" und "Rührei" werden nicht angezeigt
And Die URL enthält den Parameter "bewertung=favoriten"
```

**Testfall 3: Filter zurücksetzen**
```gherkin
Given Benutzer hat Filter "Nur Gute" aktiv und sieht gefilterte Ergebnisse
When Benutzer klickt erneut auf "Nur Gute"
Then Alle Rezepte werden wieder angezeigt (keine Bewertungsfilterung)
And Die URL hat keinen aktiven Bewertungsfilter-Parameter mehr
```

**Testfall 4: Keine Treffer**
```gherkin
Given Alle Rezepte in der App haben maximal 2 Sterne oder keine Bewertung
When Benutzer klickt auf "Nur Gute"
Then Eine freundliche Meldung wird angezeigt (z.B. "Keine Rezepte mit dieser Bewertung gefunden")
And Die Liste ist leer
```

**Testfall 5: DeepLink**
```gherkin
Given Die App enthält Rezepte mit verschiedenen Bewertungen
When Benutzer ruft die URL direkt mit "?bewertung=favoriten" auf
Then Die Liste zeigt nur 5-Sterne-Rezepte
And Der Filter "Favoriten" ist visuell als aktiv markiert
```

**Testfall 6: Kombination mit Kategorie-Filter**
```gherkin
Given Die App enthält "Dinkelbrot" (Kategorie: Brot, 5 Sterne)
And Die App enthält "Roggenbrot" (Kategorie: Brot, 2 Sterne)
And Die App enthält "Spaghetti Bolognese" (Kategorie: Mittagessen, 5 Sterne)
When Benutzer wählt Kategorie "Brot" und klickt auf "Favoriten"
Then Nur "Dinkelbrot" wird angezeigt
And "Roggenbrot" wird nicht angezeigt (zu niedrige Bewertung)
And "Spaghetti Bolognese" wird nicht angezeigt (falsche Kategorie)
```

**Testfall 7: Kombination mit "Länger nicht gemacht"**
```gherkin
Given Die App enthält "Linseneintopf" (4 Sterne, planned_date 2025-01-01)
And Die App enthält "Erbsensuppe" (4 Sterne, planned_date 2026-01-01)
And Die App enthält "Tomatensuppe" (2 Sterne, planned_date 2024-01-01)
When Benutzer klickt auf "Nur Gute" und aktiviert "Länger nicht gemacht"
Then "Linseneintopf" erscheint vor "Erbsensuppe" (älteres Datum)
And "Tomatensuppe" wird nicht angezeigt (zu niedrige Bewertung)
```

---

## 7. Abhängigkeiten & Rahmenbedingungen

### Abhängigkeiten

- Story 14 (Rezept mit 3-5 Sternen bewerten) muss abgeschlossen sein — das `rating`-Feld muss in der Datenbank existieren und Werte enthalten können. **Story 14 ist bereits abgeschlossen.**
- Story 5 (Rezept-Liste alphabetisch sortiert) muss implementiert sein — der Filter arbeitet auf dieser Liste
- Story 8 (Filter nach Kategorien) sollte implementiert sein — für die Kombination beider Filter
- Story 9 (Filter "Länger nicht gemacht") sollte implementiert sein — für die Kombination mit Datumssortierung

### Rahmenbedingungen

- Das `rating`-Feld (INTEGER NULL, Werte 1-5) existiert bereits in der `recipes`-Tabelle
- Bewertungen 1-2 Sterne gelten als Negativ-Bewertungen und werden durch "Nur Gute" ausgeblendet
- SQLite-Datenbank muss existieren und erreichbar sein
- Keine Authentifizierung erforderlich (LAN-only)
- DeepLink-fähige URLs mit Query-Parametern (Architektur-Vorgabe)

---

## Offene Punkte / Fragen

- [ ] Sollen die genauen URL-Parameter-Namen (`bewertung=gut`, `bewertung=favoriten`) hier bereits festgelegt werden oder bleibt das der technischen Planung (plan.md) überlassen?
- [ ] Soll es eine dritte Option "Nicht bewertet" geben, um gezielt unbewertete Rezepte anzuzeigen (für das Nachbewerten)?

---

**Letzte Aktualisierung:** 2026-03-29
