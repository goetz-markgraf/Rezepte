# Story 10: Filter "Nächste 7 Tage"

**Epic:** Suche & Filterung
**Priorität:** MVP - Phase 1
**Status:** In Arbeit

---

## 1. Story-Satz

**Als** Benutzer möchte ich **die Rezeptliste nach dem Prinzip "Nächste 7 Tage" filtern**, damit ich **auf einen Blick sehe, welche Rezepte ich für die kommende Woche bereits geplant habe**.

---

## 2. Geschäftsbezogene Details

### Kontext

Das `planned_date`-Feld eines Rezepts kann sowohl Vergangenheits- als auch Zukunftsdaten enthalten. Während der Filter "Länger nicht gemacht" vergangene Daten auswertet, nutzt der Filter "Nächste 7 Tage" das umgekehrte Prinzip: Er zeigt gezielt Rezepte, die für die nahe Zukunft geplant sind.

Die Wochenplanung ist eines der Kernszenarien dieser App. Das Paar plant am Mittwoch oder Donnerstag die Mahlzeiten für die kommende Woche, indem sie einzelnen Rezepten ein Zukunftsdatum setzen. Der Filter "Nächste 7 Tage" ist die Übersicht dieser Planung: Er zeigt alle Rezepte, deren `planned_date` zwischen heute und sieben Tagen in der Zukunft liegt — sortiert nach Datum, also in der Reihenfolge, in der die Gerichte auf den Tisch kommen.

Dieser Filter ersetzt eine separate Wochenplanungs-Ansicht: Die App braucht kein eigenes Planungsmodul, weil das einfache Setzen eines Zukunftsdatums am Rezept die Planung abbildet.

### Nutzergruppe

- Beide Partner des Haushalts gleichberechtigt
- Zugriff über Desktop, Tablet und Handy im LAN
- Primär bei der Wochenplanung (typisch Mittwoch/Donnerstag) und zwischendurch als Überblick

### Business-Value

- Macht die Wochenplanung auf einen Blick sichtbar: "Was haben wir diese Woche schon geplant?"
- Verhindert doppelte Planung: Man sieht sofort, welche Tage bereits belegt sind
- Schafft Vorfreude: Die geplanten Gerichte sind übersichtlich nach Wochentag sortiert
- Unterstützt das Kernziel: Wochenplanung von 20+ Minuten auf 2 Minuten reduzieren
- Keine doppelte Buchführung nötig: Die Rezept-App ist gleichzeitig der Wochenplan

### Edge Cases

- **Kein Rezept mit Zukunftsdatum:** Wenn noch kein Rezept für die nächsten 7 Tage geplant ist, erscheint eine klare, freundliche Meldung (keine leere Liste ohne Erklärung)
- **Heutiges Datum:** Rezepte mit `planned_date` = heute gelten als "geplant" und werden angezeigt
- **Genau 7 Tage in der Zukunft:** Das Datum "heute + 7 Tage" wird noch eingeschlossen (inklusive Grenze)
- **Mehr als 7 Tage in der Zukunft:** Rezepte mit `planned_date` > heute + 7 Tage werden nicht angezeigt
- **Vergangenheitsdaten:** Rezepte mit `planned_date` in der Vergangenheit werden nicht angezeigt — sie sind bereits vergangen, nicht "geplant"
- **Kombination mit Kategorie-Filter:** Wenn gleichzeitig ein Kategorie-Filter aktiv ist, werden nur Rezepte der gewählten Kategorien innerhalb der nächsten 7 Tage angezeigt
- **Kombination mit Volltextsuche:** Wenn gleichzeitig ein Suchbegriff aktiv ist, gelten nur Suchergebnisse innerhalb der nächsten 7 Tage
- **Filter zurücksetzen:** Rückkehr zur Standard-Alphabetliste ist jederzeit möglich

---

## 3. Akzeptanzkriterien

### Funktionale Kriterien

- [ ] **K1: Filter aktivierbar**
  - Auf der Rezept-Übersichtsseite gibt es ein klickbares Element (Button o.ä.) mit der Bezeichnung "Nächste 7 Tage"
  - Ein Klick auf dieses Element aktiviert den Filter
  - Der aktive Zustand des Filters ist visuell klar erkennbar (hervorgehoben)

- [ ] **K2: Nur Rezepte innerhalb des Zeitfensters**
  - Nur Rezepte mit `planned_date` zwischen heute (inklusive) und heute + 7 Tage (inklusive) werden angezeigt
  - Rezepte ohne Datum werden nicht angezeigt
  - Rezepte mit Vergangenheitsdatum werden nicht angezeigt
  - Rezepte mit `planned_date` > heute + 7 Tage werden nicht angezeigt

- [ ] **K3: Sortierung chronologisch aufsteigend**
  - Rezepte werden nach `planned_date` aufsteigend sortiert: das nächste Datum zuerst
  - Bei gleichem Datum wird alphabetisch nach Titel sortiert

- [ ] **K4: Filter zurücksetzen**
  - Es gibt eine Möglichkeit, den "Nächste 7 Tage"-Filter aufzuheben
  - Nach dem Zurücksetzen wird die vollständige Rezeptliste wieder alphabetisch angezeigt
  - Der aktive Filter-Status ist nicht mehr visuell erkennbar

- [ ] **K5: Keine Treffer**
  - Wenn kein Rezept die Filterbedingung erfüllt, erscheint eine klare, freundliche Meldung (z.B. "Keine Rezepte für die nächsten 7 Tage geplant")
  - Die Seite zeigt keinen leeren Bereich ohne Erklärung

- [ ] **K6: DeepLink-fähige URL**
  - Der aktive Filter wird als Query-Parameter in der URL abgebildet (z.B. `?filter=naechste-7-tage`)
  - Das direkte Aufrufen einer solchen URL zeigt die gefilterte Ansicht korrekt an
  - Die URL kann als Lesezeichen gespeichert und geteilt werden

- [ ] **K7: Datum wird auf der Karte angezeigt**
  - Das `planned_date` wird auf der Rezeptkarte angezeigt, damit der Nutzer sieht, für welchen Tag das Gericht geplant ist
  - Das Datum erscheint in einem lesbaren Format (z.B. "Montag, 31. März" oder "31.03.2026")

- [ ] **K8: Kombination mit Kategorie-Filter**
  - Wenn gleichzeitig ein Kategorie-Filter aktiv ist, werden nur Rezepte der gewählten Kategorien innerhalb der nächsten 7 Tage angezeigt
  - Beide Filter sind gleichzeitig in der URL sichtbar

- [ ] **K9: Kombination mit Volltextsuche**
  - Wenn gleichzeitig ein Suchbegriff aktiv ist, werden nur Suchergebnisse angezeigt, die innerhalb der nächsten 7 Tage geplant sind
  - Beide aktiven Filter sind gleichzeitig in der URL sichtbar

### Nicht-funktionale Kriterien

- [ ] **K10: Performance**
  - Filter liefert Ergebnisse in < 1 Sekunde (NFR-P1)
  - Keine sichtbare Verzögerung beim Umschalten des Filters

- [ ] **K11: Barrierefreiheit**
  - Der Filter-Button ist per Tastatur bedienbar (Tab + Enter/Space)
  - Aktiver Filter-Status wird für Screenreader korrekt kommuniziert (ARIA-Attribute, z.B. `aria-pressed`)
  - Filter-Element hat ein beschreibendes Label (WCAG 2.1 Level A)

---

## 4. Technische Planung

### Datenmodell

Kein neues Datenbankfeld erforderlich. Das vorhandene `planned_date`-Feld (optional, `time::Date`) in der `recipes`-Tabelle wird für den Filter genutzt.

Filterlogik auf Datenbankebene:
- `WHERE planned_date >= today AND planned_date <= today + 7 days` — nur Rezepte im Zeitfenster der nächsten 7 Tage (einschließlich heute)
- `ORDER BY planned_date ASC, title ASC` — chronologisch aufsteigend, bei Gleichstand alphabetisch

Das "heute"-Datum wird serverseitig zur Anfragezeit berechnet; kein Client-Datum wird verwendet.

### UI/UX-Spezifikation

**Platzierung:**
- Der "Nächste 7 Tage"-Filter befindet sich auf der Rezept-Übersichtsseite, im gleichen Bereich wie die Kategorie-Filter, die Suchleiste und der "Länger nicht gemacht"-Filter

**Darstellung:**
- Klickbarer Button oder Chip, konsistent mit dem Stil der anderen Filter
- Visuell unterscheidbar von den Kategorie-Filtern, aber konsistent mit dem "Länger nicht gemacht"-Filter (beide sind Sonder-Filter, keine Kategorien)
- Aktiver Zustand klar erkennbar (hervorgehoben)

**Verhalten:**
- Klick aktiviert den Filter: Liste wird sofort auf die geplanten Rezepte der nächsten 7 Tage eingeschränkt und chronologisch sortiert
- Klick auf aktiven Filter oder explizites Zurücksetzen: kehrt zur alphabetischen Standard-Ansicht zurück
- DeepLink-fähig: Aktiver Filter steht als Query-Parameter in der URL

**Anzeige der Ergebnisse:**
- Gleiche Kartenansicht wie die normale Rezeptliste
- Das `planned_date` wird auf der Karte angezeigt (Wochentag + Datum), damit der Nutzer die Planung übersieht
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

**Testfall 1: Geplante Rezepte werden angezeigt**
```gherkin
Given Die App enthält "Spaghetti Bolognese" mit planned_date übermorgen
And Die App enthält "Pizza" mit planned_date in 5 Tagen
And Die App enthält "Linseneintopf" ohne planned_date
When Benutzer klickt auf "Nächste 7 Tage"
Then "Spaghetti Bolognese" wird angezeigt
And "Pizza" wird angezeigt
And "Linseneintopf" wird nicht angezeigt (kein Datum)
```

**Testfall 2: Chronologische Sortierung**
```gherkin
Given Die App enthält "Pizza" mit planned_date in 5 Tagen
And Die App enthält "Spaghetti Bolognese" mit planned_date übermorgen
When Benutzer klickt auf "Nächste 7 Tage"
Then "Spaghetti Bolognese" erscheint vor "Pizza" (früheres Datum zuerst)
```

**Testfall 3: Zeitfenster-Grenzen**
```gherkin
Given Die App enthält "Heute-Rezept" mit planned_date heute
And Die App enthält "Morgen-Rezept" mit planned_date in 7 Tagen
And Die App enthält "Zukunfts-Rezept" mit planned_date in 8 Tagen
And Die App enthält "Vergangenes-Rezept" mit planned_date gestern
When Benutzer klickt auf "Nächste 7 Tage"
Then "Heute-Rezept" wird angezeigt (heute inklusive)
And "Morgen-Rezept" wird angezeigt (Tag 7 inklusive)
And "Zukunfts-Rezept" wird nicht angezeigt (außerhalb des Fensters)
And "Vergangenes-Rezept" wird nicht angezeigt (Vergangenheit)
```

**Testfall 4: Filter zurücksetzen**
```gherkin
Given Benutzer hat "Nächste 7 Tage"-Filter aktiv
When Benutzer klickt erneut auf "Nächste 7 Tage" (oder auf "Zurücksetzen")
Then Alle Rezepte werden wieder alphabetisch angezeigt
And Die URL hat keinen aktiven "Nächste 7 Tage"-Parameter mehr
```

**Testfall 5: DeepLink**
```gherkin
Given Die App enthält Rezepte mit planned_date in den nächsten 7 Tagen
When Benutzer ruft die URL direkt mit "?filter=naechste-7-tage" auf
Then Die Liste zeigt nur Rezepte der nächsten 7 Tage, chronologisch sortiert
And Der Filter-Button ist visuell als aktiv markiert
```

**Testfall 6: Keine passenden Rezepte**
```gherkin
Given Die App enthält keine Rezepte mit planned_date in den nächsten 7 Tagen
When Benutzer klickt auf "Nächste 7 Tage"
Then Eine freundliche Meldung wird angezeigt (z.B. "Keine Rezepte für die nächsten 7 Tage geplant")
And Die Liste ist leer
```

**Testfall 7: Kombination mit Kategorie-Filter**
```gherkin
Given Die App enthält "Dinkelbrot" (Kategorie: Brot, planned_date in 2 Tagen)
And Die App enthält "Spaghetti" (Kategorie: Mittagessen, planned_date in 3 Tagen)
When Benutzer wählt Kategorie "Brot" und aktiviert "Nächste 7 Tage"
Then Nur "Dinkelbrot" wird angezeigt
And "Spaghetti" wird nicht angezeigt (falsche Kategorie)
```

---

## 7. Abhängigkeiten & Rahmenbedingungen

### Abhängigkeiten

- Story 5 (Rezept-Liste alphabetisch sortiert) muss implementiert sein — der Filter arbeitet auf dieser Liste
- Story 28 (Datum-Eingabe am Rezept) muss implementiert sein — Rezepte brauchen ein `planned_date`-Feld
- Story 8 (Filter nach Kategorien) sollte implementiert sein — für die Kombination beider Filter
- Story 9 (Filter "Länger nicht gemacht") sollte implementiert sein — UI-Konsistenz zwischen den beiden Sonder-Filtern

### Rahmenbedingungen

- Das `planned_date`-Feld gilt sowohl für vergangene ("wann zuletzt gemacht") als auch zukünftige Daten ("wann geplant") — der Filter zeigt ausschließlich Zukunftsdaten im 7-Tage-Fenster
- Das "heute"-Datum wird serverseitig berechnet (keine Abhängigkeit vom Client-Datum)
- SQLite-Datenbank muss existieren und erreichbar sein
- Keine Authentifizierung erforderlich (LAN-only)
- DeepLink-fähige URLs mit Query-Parametern (Architektur-Vorgabe)

---

## Offene Punkte / Fragen

- [ ] In welchem Format soll das `planned_date` auf der Rezeptkarte angezeigt werden — nur das Datum (z.B. "31.03.") oder auch der Wochentag (z.B. "Mo, 31.03.")?
- [ ] Soll der Zeitraum exakt "nächste 7 Tage" sein (heute bis heute+7) oder "aktuelle Woche" (Montag bis Sonntag der laufenden Woche)?

---

**Letzte Aktualisierung:** 2026-03-29
