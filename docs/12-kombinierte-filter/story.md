# Story 12: Kombinierte Filter (mehrere Filter gleichzeitig)

**Epic:** Suche & Filterung
**Priorität:** MVP - Phase 1
**Status:** In Arbeit

---

## 1. Story-Satz

**Als** Benutzer möchte ich **mehrere Filter gleichzeitig kombinieren können**, damit ich **gezielt genau die Rezepte finde, die in diesem Moment relevant sind** — zum Beispiel "gute Brotrezepte, die ich lange nicht gemacht habe".

---

## 2. Geschäftsbezogene Details

### Kontext

Die einzelnen Filter (Volltextsuche, Kategorien, "Länger nicht gemacht", "Nächste 7 Tage", Bewertung) wurden in den Stories 7–11 schrittweise eingeführt. Jeder Filter löst für sich ein spezifisches Problem. Der eigentliche Mehrwert entsteht jedoch, wenn Filter kombiniert werden können: Die Kombination "Brot + Länger nicht gemacht" zeigt z.B. Brotrezepte, die schon lange nicht mehr gebacken wurden — das ist ein typischer Planungsmoment.

Diese Story stellt sicher, dass alle Filter korrekt zusammenspielen — sowohl auf der Benutzeroberfläche als auch in der Datenbankabfrage. Dabei gilt für die Kombination verschiedener Filter-Typen immer AND-Logik: Ein Rezept muss alle aktiven Bedingungen gleichzeitig erfüllen. Die OR-Logik gilt nur innerhalb des Kategorie-Filters (mehrere Kategorien: Rezept muss mindestens eine davon haben).

Zusätzlich muss die kombinierte Filterung DeepLink-fähig sein: Alle aktiven Filter werden als Query-Parameter in der URL abgebildet, sodass die Filteransicht als Lesezeichen gespeichert und direkt aufgerufen werden kann.

### Nutzergruppe

- Beide Partner des Haushalts gleichberechtigt
- Zugriff über Desktop, Tablet und Handy im LAN
- Primär bei der Wochenplanung (typisch Mittwoch/Donnerstag)

### Business-Value

- Macht die Wochenplanung präziser: Statt mehrfach zu filtern, sieht man auf Anhieb die relevante Teilmenge
- Typische Kombinationen aus dem Alltag:
  - "Gute Mittagessen, die ich lange nicht gemacht habe" (Kategorie + Bewertung + Länger nicht gemacht)
  - "Brot-Rezepte für diese Woche" (Kategorie + Nächste 7 Tage)
  - "Favoriten-Kuchen" (Kategorie + Favoriten-Bewertung)
  - "Suche nach 'Linsen' nur bei Mittagessen" (Volltextsuche + Kategorie)
- Ermöglicht das Speichern von häufig genutzten Filterkombinationen als Lesezeichen (Vorbereitung für Story 13)

### Edge Cases

- **Keine Treffer durch Kombination:** Wenn die Kombination der Filter kein Ergebnis ergibt, erscheint eine klare Meldung — nicht einfach eine leere Liste
- **Konflikt "Länger nicht gemacht" + "Nächste 7 Tage":** Diese beiden Filter schließen sich fachlich aus (einer filtert auf Vergangenheitsdaten, der andere auf Zukunftsdaten). Wenn beide aktiv sind, wird eine klare Meldung angezeigt, dass die Kombination keine Ergebnisse liefern kann, oder der zuletzt gewählte Filter deaktiviert den anderen automatisch
- **Filter einzeln zurücksetzen:** Jeder aktive Filter kann unabhängig von den anderen deaktiviert werden, ohne die restlichen Filter zu verlieren
- **Alle Filter zurücksetzen:** Es gibt eine einfache Möglichkeit, alle aktiven Filter auf einmal aufzuheben
- **URL-Konsistenz:** Beim Setzen und Entfernen von Filtern bleibt die URL immer konsistent mit dem aktuellen Filterzustand

---

## 3. Akzeptanzkriterien

### Funktionale Kriterien

- [ ] **K1: Kategorie + Volltextsuche**
  - Wenn eine Kategorie und ein Suchbegriff gleichzeitig aktiv sind, werden nur Rezepte angezeigt, die sowohl der Kategorie angehören als auch den Suchbegriff treffen (AND-Logik)
  - Die URL enthält beide Parameter gleichzeitig (z.B. `?kategorie=brot&q=dinkel`)

- [ ] **K2: Kategorie + Bewertungsfilter**
  - Wenn eine Kategorie und ein Bewertungsfilter gleichzeitig aktiv sind, werden nur Rezepte der gewählten Kategorie mit der geforderten Mindestbewertung angezeigt
  - Beispiel: Kategorie "Mittagessen" + Filter "Nur Gute" → nur Mittagessen mit 3+ Sternen

- [ ] **K3: Kategorie + "Länger nicht gemacht"**
  - Wenn eine Kategorie und der Filter "Länger nicht gemacht" aktiv sind, werden nur Rezepte der gewählten Kategorie nach dem Datum-Prinzip sortiert angezeigt
  - Rezepte anderer Kategorien erscheinen nicht, auch wenn sie ein früheres Datum haben

- [ ] **K4: Kategorie + "Nächste 7 Tage"**
  - Wenn eine Kategorie und der Filter "Nächste 7 Tage" aktiv sind, werden nur Rezepte der gewählten Kategorie mit Zukunftsdatum innerhalb der nächsten 7 Tage angezeigt

- [ ] **K5: Bewertungsfilter + "Länger nicht gemacht"**
  - Wenn der Bewertungsfilter und "Länger nicht gemacht" gleichzeitig aktiv sind, werden nur Rezepte mit der geforderten Mindestbewertung nach dem Datum-Prinzip sortiert angezeigt
  - Typischer Use-Case: "Zeig mir meine guten Rezepte, die ich lange nicht mehr gemacht habe"

- [ ] **K6: Drei Filter gleichzeitig (Kategorie + Bewertung + Länger nicht gemacht)**
  - Wenn Kategorie, Bewertungsfilter und "Länger nicht gemacht" gleichzeitig aktiv sind, werden nur Rezepte angezeigt, die alle drei Bedingungen erfüllen
  - Typischer Use-Case: "Gute Brotrezepte, die ich lange nicht gebacken habe"

- [ ] **K7: Volltextsuche + Bewertungsfilter**
  - Wenn ein Suchbegriff und ein Bewertungsfilter aktiv sind, werden nur Rezepte angezeigt, die sowohl den Suchbegriff treffen als auch die geforderte Mindestbewertung haben

- [ ] **K8: Aktive Filter visuell erkennbar**
  - Alle aktiven Filter sind gleichzeitig und unabhängig voneinander visuell als aktiv markiert
  - Man sieht auf einen Blick, welche Filter gerade kombiniert sind

- [ ] **K9: Einzelnen Filter deaktivieren**
  - Jeder aktive Filter kann einzeln deaktiviert werden, ohne andere aktive Filter zu beeinflussen
  - Nach dem Deaktivieren eines Filters aktualisiert sich die Ergebnisliste entsprechend

- [ ] **K10: Alle Filter zurücksetzen**
  - Es gibt eine Möglichkeit, alle aktiven Filter auf einmal aufzuheben (z.B. "Alle Filter zurücksetzen" oder durch Navigation zur Startseite)
  - Nach dem Zurücksetzen wird die vollständige ungefilterte Liste angezeigt

- [ ] **K11: DeepLink mit mehreren Filtern**
  - Alle aktiven Filter sind als Query-Parameter in der URL abgebildet
  - Das direkte Aufrufen einer URL mit mehreren Filtern zeigt die korrekte kombinierte Ansicht
  - Beispiel-URL: `/?kategorie=brot&bewertung=gut&sort=oldest`

- [ ] **K12: Keine Treffer**
  - Wenn die Filterkombination kein Ergebnis ergibt, erscheint eine klare, freundliche Meldung
  - Die Meldung gibt wenn möglich einen Hinweis, welche Filter aktiv sind und warum keine Ergebnisse gefunden wurden

- [ ] **K13: Konflikt "Länger nicht gemacht" + "Nächste 7 Tage"**
  - Wenn beide Datumsfilter gleichzeitig aktiviert werden, wird das Verhalten klar und konsistent gehandhabt (entweder: letzter Filter deaktiviert den vorherigen, oder: klare Fehlermeldung)
  - Die Anwendung hängt sich nicht auf und zeigt keinen inkonsistenten Zustand

### Nicht-funktionale Kriterien

- [ ] **K14: Performance**
  - Kombinierte Filter liefern Ergebnisse in < 1 Sekunde (NFR-P1)
  - Keine sichtbare Verzögerung beim Umschalten oder Hinzufügen von Filtern

- [ ] **K15: Barrierefreiheit**
  - Alle Filter-Elemente sind per Tastatur bedienbar (Tab + Enter/Space)
  - Aktive Filter-Zustände werden für Screenreader korrekt kommuniziert (ARIA-Attribute)
  - Alle Filter-Elemente haben beschreibende Labels (WCAG 2.1 Level A)

---

## 4. Technische Planung

### Datenmodell

Kein neues Datenbankfeld erforderlich. Die Filterlogik baut auf den bereits vorhandenen Feldern auf:
- `categories` (JSON-Array): für Kategorie-Filter
- `title`, `ingredients`, `instructions` (TEXT): für Volltextsuche
- `planned_date` (TEXT/DATE): für "Länger nicht gemacht" und "Nächste 7 Tage"
- `rating` (INTEGER NULL, 1-5): für Bewertungsfilter

Die kombinierten Filter werden als aufeinander aufbauende SQL-Bedingungen (AND-verknüpft) an die bestehende Basisabfrage angehängt. Die Sortierung (alphabetisch oder nach Datum) ergibt sich aus dem aktiven Sortierfilter.

### UI/UX-Spezifikation

**Filterbereich auf der Übersichtsseite:**
- Alle Filter (Volltextsuche, Kategorien, Datumsfilter, Bewertungsfilter) sind im gleichen Filterbereich sichtbar und können unabhängig voneinander gesetzt und entfernt werden
- Aktive Filter werden visuell hervorgehoben (konsistenter Stil für alle Filter-Typen)
- Es gibt einen "Alle Filter zurücksetzen"-Mechanismus (z.B. ein dedizierter Button, der nur erscheint wenn mindestens ein Filter aktiv ist)

**Verhalten:**
- Jeder Filter wirkt sofort auf die Liste (Server-Side Rendering mit Formular-Submit oder HTMX)
- Alle aktiven Filter sind gleichzeitig in der URL als Query-Parameter sichtbar
- Beim Hinzufügen eines weiteren Filters bleiben alle anderen aktiven Filter erhalten

**Anzeige der Ergebnisse:**
- Gleiche Kartenansicht wie die normale Rezeptliste
- Anzahl der gefundenen Rezepte kann optional angezeigt werden
- Bei keinen Treffern: Hinweistext mit Erklärung

---

## 5. Nicht-funktionale Anforderungen

### Performance
- Kombinierte Filter liefern Ergebnisse in < 1 Sekunde (NFR-P1 aus PRD)
- Kein separater Index notwendig für den MVP-Umfang (bis ca. 200 Rezepte)

### Browser-Support
- Aktuelle Chrome, Firefox, Safari, Edge Versionen
- iOS Safari und Android Chrome (letzte 2 Versionen)

### Barrierefreiheit
- WCAG 2.1 Level A konform
- Alle Filter-Buttons haben aussagekräftige Labels
- Aktiver Zustand wird per ARIA kommuniziert (`aria-pressed`)
- Fokus-Indikatoren sichtbar

---

## 6. Teststrategie

### E2E-Tests (Playwright)

**Testfall 1: Kategorie + Volltextsuche**
```gherkin
Given Die App enthält "Dinkelbrot" (Kategorie: Brot)
And Die App enthält "Roggenbrot" (Kategorie: Brot)
And Die App enthält "Dinkel-Müsli" (Kategorie: Snacks)
When Benutzer wählt Kategorie "Brot" und gibt "Dinkel" ins Suchfeld ein
Then Nur "Dinkelbrot" wird angezeigt
And "Roggenbrot" wird nicht angezeigt (kein Suchtreffer)
And "Dinkel-Müsli" wird nicht angezeigt (falsche Kategorie)
And Die URL enthält beide Parameter gleichzeitig
```

**Testfall 2: Kategorie + Bewertungsfilter**
```gherkin
Given Die App enthält "Dinkelbrot" (Kategorie: Brot, 4 Sterne)
And Die App enthält "Roggenbrot" (Kategorie: Brot, 2 Sterne)
And Die App enthält "Spaghetti" (Kategorie: Mittagessen, 5 Sterne)
When Benutzer wählt Kategorie "Brot" und aktiviert "Nur Gute"
Then Nur "Dinkelbrot" wird angezeigt
And "Roggenbrot" wird nicht angezeigt (zu niedrige Bewertung)
And "Spaghetti" wird nicht angezeigt (falsche Kategorie)
```

**Testfall 3: Bewertungsfilter + "Länger nicht gemacht"**
```gherkin
Given Die App enthält "Linseneintopf" (4 Sterne, planned_date 2025-01-01)
And Die App enthält "Erbsensuppe" (4 Sterne, planned_date 2026-01-01)
And Die App enthält "Kartoffelsuppe" (2 Sterne, planned_date 2024-01-01)
When Benutzer aktiviert "Nur Gute" und dann "Länger nicht gemacht"
Then "Linseneintopf" erscheint vor "Erbsensuppe" (älteres Datum zuerst)
And "Kartoffelsuppe" wird nicht angezeigt (zu niedrige Bewertung)
And Die URL enthält beide Filter-Parameter
```

**Testfall 4: Drei Filter — Kategorie + Bewertung + Länger nicht gemacht**
```gherkin
Given Die App enthält "Dinkelbrot" (Kategorie: Brot, 5 Sterne, planned_date 2025-06-01)
And Die App enthält "Roggenbrot" (Kategorie: Brot, 5 Sterne, planned_date 2026-01-01)
And Die App enthält "Linseneintopf" (Kategorie: Mittagessen, 5 Sterne, planned_date 2024-01-01)
When Benutzer wählt "Brot", aktiviert "Favoriten" und "Länger nicht gemacht"
Then "Dinkelbrot" erscheint vor "Roggenbrot" (älteres Datum zuerst)
And "Linseneintopf" wird nicht angezeigt (falsche Kategorie)
And Die URL enthält alle drei Filter-Parameter
```

**Testfall 5: Einzelnen Filter deaktivieren ohne andere zu verlieren**
```gherkin
Given Benutzer hat Kategorie "Brot" und Bewertungsfilter "Nur Gute" aktiv
When Benutzer klickt auf "Nur Gute" um den Bewertungsfilter zu deaktivieren
Then Die Liste zeigt alle Brot-Rezepte unabhängig von der Bewertung
And Der Kategorie-Filter "Brot" ist weiterhin aktiv
And Die URL enthält nur noch den Kategorie-Parameter, nicht mehr den Bewertungsparameter
```

**Testfall 6: Keine Treffer durch Kombination**
```gherkin
Given Die App enthält nur Brot-Rezepte mit maximal 2 Sternen
When Benutzer wählt Kategorie "Brot" und aktiviert "Nur Gute"
Then Eine freundliche Meldung wird angezeigt
And Die Liste ist leer
```

**Testfall 7: DeepLink mit mehreren Filtern**
```gherkin
Given Die App enthält "Dinkelbrot" (Kategorie: Brot, 5 Sterne)
And Die App enthält "Roggenbrot" (Kategorie: Brot, 2 Sterne)
When Benutzer ruft die URL direkt mit "?kategorie=brot&bewertung=favoriten" auf
Then Nur "Dinkelbrot" wird angezeigt
And Kategorie "Brot" ist als aktiv markiert
And Bewertungsfilter "Favoriten" ist als aktiv markiert
```

---

## 7. Abhängigkeiten & Rahmenbedingungen

### Abhängigkeiten

- Story 07 (Volltextsuche) muss abgeschlossen sein
- Story 08 (Filter nach Kategorien) muss abgeschlossen sein
- Story 09 (Filter "Länger nicht gemacht") muss abgeschlossen sein
- Story 10 (Filter "Nächste 7 Tage") muss abgeschlossen sein
- Story 11 (Filter nach Bewertung) muss abgeschlossen sein
- Alle genannten Stories sind bereits abgeschlossen — die Einzelfilter existieren, aber ihre korrekte Kombination muss explizit sichergestellt und getestet werden

### Rahmenbedingungen

- SQLite-Datenbank muss existieren und erreichbar sein
- Keine Authentifizierung erforderlich (LAN-only)
- DeepLink-fähige URLs mit Query-Parametern (Architektur-Vorgabe)
- Die Kombination verschiedener Filter-Typen gilt immer AND-Logik
- OR-Logik gilt nur innerhalb des Kategorie-Filters (mehrere Kategorien gleichzeitig)

---

## Offene Punkte / Fragen

- [ ] Wie soll der Konflikt "Länger nicht gemacht" + "Nächste 7 Tage" behandelt werden? Automatisches Deaktivieren des jeweils anderen Filters oder Fehlermeldung?
- [ ] Soll es einen dedizierten "Alle Filter zurücksetzen"-Button geben, oder reicht es, jeden Filter einzeln deaktivieren zu können?

---

**Letzte Aktualisierung:** 2026-03-29
