# Story 8: Filter nach Kategorien

**Epic:** Suche & Filterung
**Priorität:** MVP - Phase 1
**Status:** In Arbeit

---

## 1. Story-Satz

**Als** Benutzer möchte ich **die Rezeptliste nach einer oder mehreren Kategorien filtern**, damit ich **schnell nur die Rezepte sehe, die für meinen aktuellen Bedarf relevant sind** (z.B. nur Brotrezepte oder nur Partygerichte).

---

## 2. Geschäftsbezogene Details

### Kontext

Bei der Wochenplanung sucht man selten "irgendein Rezept", sondern hat meist einen konkreten Kontext im Kopf: "Was kochen wir mittags?" oder "Was backe ich für die Party?". Die alphabetische Gesamtliste aller Rezepte hilft hier wenig — man möchte nur die relevante Teilmenge sehen.

Die Kategorien sind fachlich vorgegeben und decken die typischen Nutzungskontexte des Haushalts ab: Mittagessen, Brot, Party, Kuchen, Snacks. Ein Rezept kann mehreren Kategorien gleichzeitig zugeordnet sein (z.B. ein Kuchen, der auch für Partys geeignet ist).

### Nutzergruppe

- Beide Partner des Haushalts gleichberechtigt
- Zugriff über Desktop, Tablet und Handy im LAN

### Business-Value

- Reduziert die Wochenplanung auf relevante Rezepte — weniger scrollen, schneller entscheiden
- Macht den Kategorie-Filter zu einem natürlichen Einstiegspunkt in die Wochenplanung ("Was gibt es zum Mittagessen?")
- DeepLink-fähige Filter-URLs ermöglichen es, häufig genutzte Filteransichten als Lesezeichen zu speichern

### Edge Cases

- **Kein Rezept in Kategorie:** Klare Meldung, dass für diese Kategorie keine Rezepte vorhanden sind (statt leerer Seite)
- **Mehrfachfilter:** Wenn mehrere Kategorien gleichzeitig ausgewählt sind, werden Rezepte angezeigt, die mindestens eine der gewählten Kategorien haben (OR-Logik)
- **Filter zurücksetzen:** Es muss eine einfache Möglichkeit geben, alle Kategorie-Filter aufzuheben und die vollständige Liste anzuzeigen
- **Kombination mit Suche:** Wenn gleichzeitig ein Suchbegriff aktiv ist, gelten beide Filter (Kategorien UND Suchbegriff)

---

## 3. Akzeptanzkriterien

### Funktionale Kriterien

- [ ] **K1: Kategorien sichtbar und auswählbar**
  - Alle fünf Kategorien sind auf der Übersichtsseite sichtbar: Mittagessen, Brot, Party, Kuchen, Snacks
  - Jede Kategorie ist als eigenständiges Filter-Element klickbar
  - Die Reihenfolge der Kategorien ist fest und konsistent

- [ ] **K2: Einfacher Kategorie-Filter**
  - Ein Klick auf eine Kategorie filtert die Rezeptliste auf Rezepte dieser Kategorie
  - Die aktiv gewählte Kategorie ist visuell als ausgewählt erkennbar (z.B. hervorgehoben)
  - Die gefilterte Liste ist alphabetisch sortiert (wie die Standardliste)

- [ ] **K3: Mehrere Kategorien gleichzeitig**
  - Es können mehrere Kategorien gleichzeitig ausgewählt werden
  - Die Ergebnisliste zeigt alle Rezepte, die mindestens einer der gewählten Kategorien angehören (ODER-Logik)
  - Mehrere aktive Filter sind gleichzeitig visuell erkennbar

- [ ] **K4: Filter zurücksetzen**
  - Es gibt eine Möglichkeit, alle Kategorie-Filter aufzuheben (z.B. "Alle" oder durch Abwählen aller Filter)
  - Nach dem Zurücksetzen wird die vollständige ungefilterte Rezeptliste angezeigt

- [ ] **K5: Keine Treffer**
  - Wenn eine Kategorie keine Rezepte enthält, erscheint eine klare, freundliche Meldung (z.B. "Keine Rezepte in dieser Kategorie")
  - Die Seite zeigt keinen leeren Bereich ohne Erklärung

- [ ] **K6: DeepLink-fähige URL**
  - Der aktive Kategorie-Filter wird als Query-Parameter in der URL abgebildet (z.B. `?kategorie=brot` oder `?kategorie=brot&kategorie=kuchen`)
  - Das direkte Aufrufen einer solchen URL zeigt die gefilterte Ansicht korrekt an
  - Die URL kann als Lesezeichen gespeichert werden und funktioniert beim nächsten Aufruf

- [ ] **K7: Kombination mit Volltextsuche**
  - Wenn ein Suchbegriff aktiv ist und zusätzlich eine Kategorie gewählt wird, werden nur Rezepte angezeigt, die beide Bedingungen erfüllen
  - Beide Filter sind gleichzeitig in der URL sichtbar

### Nicht-funktionale Kriterien

- [ ] **K8: Performance**
  - Kategorie-Filter liefert Ergebnisse in < 1 Sekunde
  - Die Filterauswahl blockiert die Seite nicht (keine UI-Hänger)

- [ ] **K9: Barrierefreiheit**
  - Kategorie-Filter-Elemente sind per Tastatur bedienbar (Tab + Enter/Space)
  - Aktiver Filter-Status wird für Screenreader korrekt kommuniziert (ARIA-Attribute)
  - Alle Filter-Elemente haben beschreibende Labels (WCAG 2.1 Level A)

---

## 4. Technische Planung

### Datenmodell

Kein neues Datenbankfeld erforderlich. Rezepte haben bereits ein `categories`-Feld (JSON-Array in SQLite), das die zugewiesenen Kategorien speichert. Die fünf Kategorien sind hardcoded:
- Mittagessen
- Brot
- Party
- Kuchen
- Snacks

### UI/UX-Spezifikation

**Platzierung:**
- Die Kategorie-Filter befinden sich auf der Rezept-Übersichtsseite, im Bereich der Suchleiste/Filteroptionen, gut sichtbar vor der Rezeptliste

**Darstellung:**
- Kategorien als klickbare Schaltflächen oder Chips, die sich visuell als "aktiv" / "inaktiv" unterscheiden
- Keine Dropdown-Lösung — alle Kategorien sind direkt sichtbar, kein Untermenü

**Verhalten:**
- Klick auf eine Kategorie: Filtert die Liste sofort (ohne Seitenneuladung oder mit minimalem Reload)
- Klick auf eine bereits aktive Kategorie: Hebt den Filter wieder auf
- DeepLink-fähig: Aktive Kategorien stehen als Query-Parameter in der URL (`?kategorie=brot`)

**Anzeige der Ergebnisse:**
- Gleiche Kartenansicht wie die normale Rezeptliste
- Bei keinen Treffern: Hinweistext statt leerer Liste

---

## 5. Nicht-funktionale Anforderungen

### Performance
- Kategorie-Filter liefert Ergebnisse in < 1 Sekunde (NFR-P1 aus PRD)
- Kein separater Index notwendig für den MVP-Umfang (bis ca. 200 Rezepte)

### Browser-Support
- Aktuelle Chrome, Firefox, Safari, Edge Versionen
- iOS Safari und Android Chrome (letzte 2 Versionen)

### Barrierefreiheit
- WCAG 2.1 Level A konform
- Kategorie-Buttons haben aussagekräftige Labels
- Aktiver Zustand wird per ARIA kommuniziert (`aria-pressed`)
- Fokus-Indikatoren sichtbar

---

## 6. Teststrategie

### E2E-Tests (Playwright)

**Testfall 1: Einzelne Kategorie filtern**
```gherkin
Given Die App enthält ein Rezept "Vollkornbrot" in Kategorie "Brot"
And Die App enthält ein Rezept "Spaghetti Bolognese" in Kategorie "Mittagessen"
When Benutzer klickt auf die Kategorie "Brot"
Then Nur "Vollkornbrot" wird in der Liste angezeigt
And "Spaghetti Bolognese" wird nicht angezeigt
And Die URL enthält den Parameter "kategorie=brot"
```

**Testfall 2: Mehrere Kategorien gleichzeitig**
```gherkin
Given Die App enthält "Käsekuchen" in Kategorie "Kuchen"
And Die App enthält "Partybrot" in Kategorie "Brot" und "Party"
And Die App enthält "Spaghetti Bolognese" in Kategorie "Mittagessen"
When Benutzer wählt Kategorien "Kuchen" und "Brot"
Then "Käsekuchen" und "Partybrot" werden angezeigt
And "Spaghetti Bolognese" wird nicht angezeigt
```

**Testfall 3: Filter zurücksetzen**
```gherkin
Given Benutzer hat Kategorie "Brot" aktiv und sieht gefilterte Ergebnisse
When Benutzer klickt erneut auf "Brot" (oder auf "Alle")
Then Alle Rezepte werden wieder angezeigt
And Die URL hat keinen aktiven Kategorie-Parameter mehr
```

**Testfall 4: Keine Treffer**
```gherkin
Given Keine Rezepte in der App haben Kategorie "Snacks"
When Benutzer klickt auf "Snacks"
Then Eine Meldung "Keine Rezepte in dieser Kategorie" wird angezeigt
And Die Liste ist leer
```

**Testfall 5: DeepLink**
```gherkin
Given Die App enthält Rezepte in Kategorie "Party"
When Benutzer ruft die URL direkt mit "?kategorie=party" auf
Then Die Liste zeigt nur Rezepte der Kategorie "Party"
And Der Kategorie-Filter "Party" ist visuell als aktiv markiert
```

**Testfall 6: Kombination mit Suche**
```gherkin
Given Die App enthält "Dinkelbrot" (Kategorie: Brot) und "Roggenbrot" (Kategorie: Brot)
When Benutzer wählt Kategorie "Brot" und gibt "Dinkel" ins Suchfeld ein
Then Nur "Dinkelbrot" wird angezeigt
And "Roggenbrot" wird nicht angezeigt
```

---

## 7. Abhängigkeiten & Rahmenbedingungen

### Abhängigkeiten

- Story 5 (Rezept-Liste alphabetisch sortiert) muss implementiert sein — der Filter arbeitet auf dieser Liste
- Story 1 (Rezept erstellen mit Kategorien) muss implementiert sein — Rezepte müssen Kategorien haben
- Story 7 (Volltextsuche) sollte implementiert sein — für die Kombination aus Suche und Kategorie-Filter

### Rahmenbedingungen

- Kategorien sind fachlich fest vorgegeben: Mittagessen, Brot, Party, Kuchen, Snacks — keine dynamische Verwaltung
- SQLite-Datenbank muss existieren und erreichbar sein
- Keine Authentifizierung erforderlich (LAN-only)
- DeepLink-fähige URLs mit Query-Parametern (Architektur-Vorgabe)

---

## Offene Punkte / Fragen

- [ ] Soll bei Mehrfachauswahl eine ODER- oder UND-Logik gelten? (Aktuell angenommen: ODER — Rezepte die mindestens eine der Kategorien haben)
- [ ] Soll die Anzahl der Rezepte je Kategorie als Badge angezeigt werden (z.B. "Mittagessen (12)")?

---

**Letzte Aktualisierung:** 2026-03-28
