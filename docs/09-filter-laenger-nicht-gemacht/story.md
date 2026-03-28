# Story 9: Filter "Länger nicht gemacht"

**Epic:** Suche & Filterung
**Priorität:** MVP - Phase 1
**Status:** In Arbeit

---

## 1. Story-Satz

**Als** Benutzer möchte ich **die Rezeptliste nach dem Prinzip "Länger nicht gemacht" filtern**, damit ich **Rezepte sehe, die ich schon lange nicht mehr zubereitet habe, und so mehr Abwechslung in die Wochenplanung bringe**.

---

## 2. Geschäftsbezogene Details

### Kontext

Das zentrale Problem, das diese App löst, ist fehlende Variation im Speiseplan: Das Paar greift immer auf dieselben 5-10 Gerichte zurück, obwohl mehr Rezepte vorhanden sind. Der Filter "Länger nicht gemacht" ist die Kernantwort auf dieses Problem — er bringt vergessene Rezepte direkt nach oben und macht bewusst, was schon lange nicht mehr auf dem Tisch stand.

Das `planned_date`-Feld eines Rezepts speichert das zuletzt eingetragene Datum (Vergangenheit: "wann zuletzt gemacht"; Zukunft: "wann geplant"). Für diesen Filter sind nur Vergangenheitsdaten relevant — Rezepte, die zuletzt vor langer Zeit gemacht wurden, sollen ganz oben erscheinen.

### Nutzergruppe

- Beide Partner des Haushalts gleichberechtigt
- Zugriff über Desktop, Tablet und Handy im LAN
- Primär bei der Wochenplanung (typisch Mittwoch/Donnerstag)

### Business-Value

- Bricht aus der "immer dieselben 5 Gerichte"-Routine aus
- Macht vergessene Favoriten sofort sichtbar — auch Rezepte, die seit Monaten nicht gemacht wurden
- Verkürzt die Wochenplanung: Statt durch alle Rezepte zu scrollen, sieht man sofort die "Kandidaten" für diese Woche
- Unterstützt das Kernziel: Wochenplanung von 20+ Minuten auf 2 Minuten reduzieren

### Edge Cases

- **Rezept ohne Datum:** Rezepte ohne eingetragenes `planned_date` gelten als "am längsten nicht gemacht" und erscheinen ganz oben in der Liste (höchste Priorität für Wiederentdeckung)
- **Rezept mit Zukunftsdatum:** Rezepte, die für die Zukunft geplant sind (Datum liegt in der Zukunft), werden aus dem Filter ausgeschlossen — sie sind bereits verplant und kein Kandidat für "länger nicht gemacht"
- **Keine Treffer:** Wenn alle Rezepte ein Zukunftsdatum haben (unwahrscheinlich, aber möglich), erscheint eine klare Meldung
- **Kombination mit Kategorie-Filter:** Wenn gleichzeitig ein Kategorie-Filter aktiv ist, werden nur Rezepte der gewählten Kategorien nach dem "Länger nicht gemacht"-Prinzip sortiert und angezeigt
- **Kombination mit Volltextsuche:** Wenn gleichzeitig ein Suchbegriff aktiv ist, gilt die Sortierung nach Datum zusätzlich zum Suchfilter
- **Filter zurücksetzen:** Eine einfache Möglichkeit, den Filter aufzuheben und zur Standard-Alphabetliste zurückzukehren

---

## 3. Akzeptanzkriterien

### Funktionale Kriterien

- [ ] **K1: Filter aktivierbar**
  - Auf der Rezept-Übersichtsseite gibt es ein klickbares Element (Button o.ä.) mit der Bezeichnung "Länger nicht gemacht"
  - Ein Klick auf dieses Element aktiviert den Filter
  - Der aktive Zustand des Filters ist visuell klar erkennbar (hervorgehoben)

- [ ] **K2: Sortierung nach Datum aufsteigend**
  - Rezepte werden nach ihrem `planned_date` aufsteigend sortiert: ältestes Datum zuerst
  - Rezepte ohne Datum erscheinen ganz oben (vor allen Rezepten mit Datum)
  - Innerhalb von Rezepten gleichen Datums wird alphabetisch sortiert

- [ ] **K3: Zukunftsdaten ausgeschlossen**
  - Rezepte mit einem `planned_date` in der Zukunft (Datum > heute) werden im Filter nicht angezeigt
  - Sie sind bereits für die Zukunft geplant und keine "Länger nicht gemacht"-Kandidaten

- [ ] **K4: Filter zurücksetzen**
  - Es gibt eine Möglichkeit, den "Länger nicht gemacht"-Filter aufzuheben
  - Nach dem Zurücksetzen wird die vollständige Rezeptliste wieder alphabetisch angezeigt
  - Der aktive Filter-Status ist nicht mehr visuell erkennbar

- [ ] **K5: Keine Treffer**
  - Wenn kein Rezept die Filterbedingung erfüllt (z.B. alle haben Zukunftsdaten), erscheint eine klare, freundliche Meldung
  - Die Seite zeigt keinen leeren Bereich ohne Erklärung

- [ ] **K6: DeepLink-fähige URL**
  - Der aktive Filter wird als Query-Parameter in der URL abgebildet (z.B. `?filter=laenger-nicht-gemacht`)
  - Das direkte Aufrufen einer solchen URL zeigt die gefilterte Ansicht korrekt an
  - Die URL kann als Lesezeichen gespeichert und geteilt werden

- [ ] **K7: Kombination mit Kategorie-Filter**
  - Wenn gleichzeitig ein Kategorie-Filter aktiv ist, werden nur Rezepte der gewählten Kategorien angezeigt — und innerhalb davon nach "Länger nicht gemacht" sortiert
  - Beide Filter sind gleichzeitig in der URL sichtbar

- [ ] **K8: Kombination mit Volltextsuche**
  - Wenn gleichzeitig ein Suchbegriff aktiv ist, werden nur Suchergebnisse angezeigt — und innerhalb davon nach "Länger nicht gemacht" sortiert
  - Beide aktiven Filter sind gleichzeitig in der URL sichtbar

### Nicht-funktionale Kriterien

- [ ] **K9: Performance**
  - Filter liefert Ergebnisse in < 1 Sekunde (NFR-P1)
  - Keine sichtbare Verzögerung beim Umschalten des Filters

- [ ] **K10: Barrierefreiheit**
  - Der Filter-Button ist per Tastatur bedienbar (Tab + Enter/Space)
  - Aktiver Filter-Status wird für Screenreader korrekt kommuniziert (ARIA-Attribute, z.B. `aria-pressed`)
  - Filter-Element hat ein beschreibendes Label (WCAG 2.1 Level A)

---

## 4. Technische Planung

### Datenmodell

Kein neues Datenbankfeld erforderlich. Das vorhandene `planned_date`-Feld (optional, `time::Date`) in der `recipes`-Tabelle wird für die Sortierung genutzt.

Filterlogik auf Datenbankebene:
- `WHERE planned_date IS NULL OR planned_date <= today` — schließt Zukunftsdaten aus
- `ORDER BY CASE WHEN planned_date IS NULL THEN 0 ELSE 1 END ASC, planned_date ASC` — NULL-Dates zuerst, dann aufsteigend nach Datum

### UI/UX-Spezifikation

**Platzierung:**
- Der "Länger nicht gemacht"-Filter befindet sich auf der Rezept-Übersichtsseite, im gleichen Bereich wie die Kategorie-Filter und die Suchleiste

**Darstellung:**
- Klickbarer Button oder Chip, konsistent mit dem Stil der Kategorie-Filter
- Visuell unterscheidbar von den Kategorie-Filtern (z.B. anderer Stil oder eigener Bereich), da es sich um einen Sortier-Filter handelt
- Aktiver Zustand klar erkennbar (hervorgehoben)

**Verhalten:**
- Klick aktiviert den Filter: Liste wird sofort nach "Länger nicht gemacht" sortiert
- Klick auf aktiven Filter oder explizites Zurücksetzen: kehrt zur alphabetischen Standard-Ansicht zurück
- DeepLink-fähig: Aktiver Filter steht als Query-Parameter in der URL

**Anzeige der Ergebnisse:**
- Gleiche Kartenansicht wie die normale Rezeptliste
- Das `planned_date` wird auf der Karte angezeigt (sofern vorhanden), damit der Nutzer sieht, wann das Rezept zuletzt gemacht wurde
- Bei keinen Treffern: Hinweistext statt leerer Liste

---

## 5. Nicht-funktionale Anforderungen

### Performance
- Filter liefert Ergebnisse in < 1 Sekunde (NFR-P1 aus PRD)
- Kein separater Index notwendig für den MVP-Umfang (bis ca. 200 Rezepte)

### Browser-Support
- Aktuelle Chrome, Firefox, Safari, Edge Versionen
- iOS Safari und Android Chrome (letzte 2 Versionen)

### Barrierefreiheit
- WCAG 2.1 Level A konform
- Filter-Button hat aussagekräftiges Label
- Aktiver Zustand wird per ARIA kommuniziert (`aria-pressed`)
- Fokus-Indikatoren sichtbar

---

## 6. Teststrategie

### E2E-Tests (Playwright)

**Testfall 1: Grundlegende Sortierung nach Datum**
```gherkin
Given Die App enthält "Spaghetti Bolognese" mit planned_date 2026-01-01
And Die App enthält "Pfannkuchen" mit planned_date 2025-06-15
And Die App enthält "Pizza" ohne planned_date
When Benutzer klickt auf "Länger nicht gemacht"
Then "Pizza" erscheint als erstes (kein Datum = länger nicht gemacht)
And "Pfannkuchen" erscheint als zweites (älteres Datum)
And "Spaghetti Bolognese" erscheint als drittes (neueres Datum)
```

**Testfall 2: Zukunftsdaten werden ausgeschlossen**
```gherkin
Given Die App enthält "Sonntagsbraten" mit planned_date nächste Woche
And Die App enthält "Linseneintopf" mit planned_date letzten Monat
When Benutzer klickt auf "Länger nicht gemacht"
Then "Linseneintopf" wird angezeigt
And "Sonntagsbraten" wird nicht angezeigt (Zukunftsdatum)
```

**Testfall 3: Filter zurücksetzen**
```gherkin
Given Benutzer hat "Länger nicht gemacht"-Filter aktiv
When Benutzer klickt erneut auf "Länger nicht gemacht" (oder auf "Zurücksetzen")
Then Alle Rezepte werden wieder alphabetisch angezeigt
And Die URL hat keinen aktiven "Länger nicht gemacht"-Parameter mehr
```

**Testfall 4: DeepLink**
```gherkin
Given Die App enthält Rezepte mit verschiedenen Daten
When Benutzer ruft die URL direkt mit "?filter=laenger-nicht-gemacht" auf
Then Die Liste zeigt Rezepte nach "Länger nicht gemacht" sortiert
And Der Filter-Button ist visuell als aktiv markiert
```

**Testfall 5: Kombination mit Kategorie-Filter**
```gherkin
Given Die App enthält "Dinkelbrot" (Kategorie: Brot, planned_date 2025-01-01)
And Die App enthält "Roggenbrot" (Kategorie: Brot, planned_date 2026-01-01)
And Die App enthält "Spaghetti" (Kategorie: Mittagessen, planned_date 2024-01-01)
When Benutzer wählt Kategorie "Brot" und aktiviert "Länger nicht gemacht"
Then Nur "Dinkelbrot" und "Roggenbrot" werden angezeigt (nur Kategorie Brot)
And "Dinkelbrot" erscheint vor "Roggenbrot" (älteres Datum)
And "Spaghetti" wird nicht angezeigt (falsche Kategorie)
```

**Testfall 6: Keine passenden Rezepte**
```gherkin
Given Alle Rezepte in der App haben ein Datum in der Zukunft
When Benutzer klickt auf "Länger nicht gemacht"
Then Eine freundliche Meldung wird angezeigt (z.B. "Keine Rezepte gefunden")
And Die Liste ist leer
```

---

## 7. Abhängigkeiten & Rahmenbedingungen

### Abhängigkeiten

- Story 5 (Rezept-Liste alphabetisch sortiert) muss implementiert sein — der Filter arbeitet auf dieser Liste
- Story 28 (Datum-Eingabe am Rezept) muss implementiert sein — Rezepte brauchen ein `planned_date`-Feld
- Story 8 (Filter nach Kategorien) sollte implementiert sein — für die Kombination beider Filter

### Rahmenbedingungen

- Das `planned_date`-Feld gilt sowohl für vergangene ("wann zuletzt gemacht") als auch zukünftige Daten ("wann geplant") — der Filter muss Zukunftsdaten explizit ausschließen
- SQLite-Datenbank muss existieren und erreichbar sein
- Keine Authentifizierung erforderlich (LAN-only)
- DeepLink-fähige URLs mit Query-Parametern (Architektur-Vorgabe)

---

## Offene Punkte / Fragen

- [ ] Soll das `planned_date` auf der Rezeptkarte in der gefilterten Liste angezeigt werden, damit der Nutzer sieht "zuletzt gemacht am ..."?
- [ ] Wie soll der Filter-Button benannt sein — "Länger nicht gemacht" oder eine kürzere Variante?

---

**Letzte Aktualisierung:** 2026-03-28
