# Story 13: Gespeicherte Filter für Schnellzugriff

**Epic:** Suche & Filterung
**Priorität:** Post-MVP - Phase 2 (Growth Feature)
**Status:** In Arbeit

---

## 1. Story-Satz

**Als** Benutzer möchte ich **häufig genutzte Filterkombinationen unter einem Namen speichern und per Klick aufrufen können**, damit ich **beim Wochenplanen nicht jedes Mal dieselben Filter manuell neu zusammenstellen muss**.

---

## 2. Geschäftsbezogene Details

### Kontext

Mit Story 12 (Kombinierte Filter) können mehrere Filter gleichzeitig aktiv sein und die URL spiegelt den Filterzustand vollständig wider. Das ist eine gute Basis, aber für wiederkehrende Planungsszenarien ist der Aufwand noch zu hoch: Jedes Mal Kategorie wählen, Bewertungsfilter setzen, Datumsfilter aktivieren — drei Klicks, die sich wöchentlich wiederholen.

Typische Planungsroutinen des Haushalts sind immer gleich: "Gute Mittagessen, die wir lange nicht gemacht haben" oder "Brot-Rezepte für das Wochenende". Diese Kombination sollte einmal benannt und dauerhaft gespeichert werden können, um sie per Klick aufzurufen.

Da die Filterparameter bereits als URL Query-Parameter abgebildet sind (Story 12), ist das Speichern eines Filters im Wesentlichen das Speichern einer benannten URL-Kombination. Das Abrufen ist dann einfach das Wiederherstellen dieser URL-Parameter.

### Nutzergruppe

- Beide Partner des Haushalts gleichberechtigt
- Zugriff über Desktop, Tablet und Handy im LAN
- Primär bei der wöchentlichen Wochenplanung (typisch Mittwoch/Donnerstag)

### Business-Value

- Reduziert den Aufwand der Wochenplanung weiter: Statt 3 Klicks für die Filterkombination nur noch 1 Klick
- Ermöglicht benannte Planungsperspektiven, die die eigenen Gewohnheiten widerspiegeln (z.B. "Mittwochs-Planung", "Brot-Inspiration")
- Die gespeicherten Filter sind im LAN für beide Partner gleichzeitig sichtbar und nutzbar — kein Gerät-spezifischer Zustand
- Typische gespeicherte Filterkombinationen:
  - "Mittagessenplanung" = Kategorie Mittagessen + Gute Bewertung + Länger nicht gemacht
  - "Brot-Ideen" = Kategorie Brot + Länger nicht gemacht
  - "Wochenplan-Check" = Nächste 7 Tage
  - "Neue Rezepte ausprobieren" = Kategorie Mittagessen + Keine Bewertung (noch nicht bewertet)

### Edge Cases

- **Kein Name vergeben:** Ein Filter ohne Namen kann nicht gespeichert werden — der Name ist Pflichtfeld
- **Doppelter Name:** Wenn ein Filter mit demselben Namen bereits existiert, wird der Nutzer gefragt ob er den vorhandenen überschreiben möchte
- **Gespeicherter Filter liefert keine Treffer:** Wenn die gespeicherten Filterparameter aktuell kein Rezept finden (z.B. keine geplanten Rezepte in den nächsten 7 Tagen), wird die bekannte "Keine Treffer"-Meldung angezeigt — der Filter selbst bleibt gespeichert und gültig
- **Filter löschen:** Ein gespeicherter Filter kann jederzeit wieder gelöscht werden
- **Umbenennen:** Ein gespeicherter Filter kann umbenannt werden (oder: Löschen + neu speichern)
- **Sehr viele gespeicherte Filter:** Es gibt kein hartes Limit, aber bei mehr als ca. 10 gespeicherten Filtern wird die Ansicht nicht übersichtlicher. Eine visuelle Warnung ist nicht nötig, da dies unwahrscheinlich ist.
- **Nur ein Filter aktiv (kein kombinierter Filter):** Auch ein einzelner aktiver Filter (z.B. nur "Kategorie: Brot") kann als gespeicherter Filter abgelegt werden

---

## 3. Akzeptanzkriterien

### Funktionale Kriterien

- [ ] **K1: Aktuellen Filterzustand speichern**
  - Wenn mindestens ein Filter aktiv ist, kann der Benutzer den aktuellen Filterzustand unter einem frei gewählten Namen speichern
  - Ein Name ist Pflicht — leere Namen werden nicht akzeptiert
  - Nach dem Speichern ist der Filter sofort in der Liste der gespeicherten Filter sichtbar

- [ ] **K2: Gespeicherten Filter aufrufen**
  - Alle gespeicherten Filter werden an gut sichtbarer Stelle auf der Übersichtsseite aufgelistet
  - Ein Klick auf einen gespeicherten Filter aktiviert exakt die gespeicherte Filterkombination
  - Die URL wird entsprechend aktualisiert (alle Filter-Query-Parameter wie beim manuellen Setzen)
  - Die Rezeptliste aktualisiert sich entsprechend

- [ ] **K3: Gespeicherten Filter löschen**
  - Jeder gespeicherte Filter kann einzeln gelöscht werden
  - Das Löschen eines gespeicherten Filters hat keinen Einfluss auf die aktuell aktiven Filter oder die Rezeptliste
  - Das Löschen entfernt den Eintrag dauerhaft aus der Datenbank

- [ ] **K4: Gespeicherter Filter ist persistent**
  - Gespeicherte Filter bleiben nach Seiten-Reload erhalten
  - Gespeicherte Filter sind von jedem Gerät im LAN abrufbar (server-seitig gespeichert, nicht im Browser)
  - Gespeicherte Filter überleben einen Server-Neustart

- [ ] **K5: Doppelter Name**
  - Wenn ein gespeicherter Filter mit demselben Namen bereits existiert, wird eine klare Meldung angezeigt
  - Der Benutzer kann entscheiden, ob er den vorhandenen Filter überschreiben oder einen anderen Namen wählen möchte

- [ ] **K6: Gespeicherter Filter ohne aktuelle Treffer**
  - Wenn ein gespeicherter Filter aktiviert wird und keine Rezepte findet, erscheint die bekannte "Keine Treffer"-Meldung
  - Der gespeicherte Filter bleibt unverändert erhalten — er ist weiterhin abrufbar

- [ ] **K7: Filterbereich ohne aktiven Filter — kein Speichern-Button**
  - Der Button oder die Möglichkeit zum Speichern eines Filters ist nur sichtbar (oder aktiv), wenn mindestens ein Filter aktiv ist
  - Wenn kein Filter aktiv ist, gibt es nichts zu speichern

- [ ] **K8: Gespeicherter Filter als DeepLink**
  - Ein gespeicherter Filter ist über die URL abrufbar — sowohl durch direktes Aufrufen der URL mit den entsprechenden Query-Parametern, als auch durch Klick auf den Filter
  - Das Setzen der Query-Parameter in der URL entspricht dem Aktivieren der Einzelfilter

### Nicht-funktionale Kriterien

- [ ] **K9: Performance**
  - Das Laden der gespeicherten Filter-Liste beim Seitenaufruf dauert < 500ms
  - Das Aktivieren eines gespeicherten Filters liefert Ergebnisse in < 1 Sekunde (wie normale Filter)

- [ ] **K10: Barrierefreiheit**
  - Alle Interaktionen (Speichern, Aufrufen, Löschen) sind per Tastatur bedienbar
  - Gespeicherte Filter haben aussagekräftige Labels für Screenreader
  - Bestätigungsdialoge / Fehlermeldungen sind per ARIA zugänglich

---

## 4. Technische Planung

### Datenmodell

Neue Tabelle `saved_filters` in der SQLite-Datenbank:

```sql
CREATE TABLE saved_filters (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    name TEXT NOT NULL UNIQUE,
    query_string TEXT NOT NULL,  -- die gespeicherten Query-Parameter als String, z.B. "kategorie=brot&bewertung=gut&sort=oldest"
    created_at TEXT NOT NULL DEFAULT (datetime('now'))
);
```

- `name`: Freitext-Name, vom Benutzer vergeben (Pflicht, eindeutig)
- `query_string`: die URL-Query-Parameter als String (gleiche Syntax wie in URL, ohne führendes `?`)
- `created_at`: Zeitstempel für Sortierung (neueste zuletzt oder älteste zuerst)

Kein neues Rezept-Feld notwendig.

### UI/UX-Spezifikation

**Bereich für gespeicherte Filter (Übersichtsseite):**
- Direkt im oder unterhalb des Filterbereichs wird ein Abschnitt "Gespeicherte Filter" angezeigt
- Jeder gespeicherte Filter erscheint als klickbare Schaltfläche / Chip mit dem gewählten Namen
- Neben oder am Filter gibt es ein Löschen-Symbol (z.B. ×-Icon) zum Entfernen
- Wenn keine gespeicherten Filter existieren, ist der Bereich leer oder wird gar nicht angezeigt

**Speichern eines Filters:**
- Wenn mindestens ein Filter aktiv ist, erscheint ein "Filter speichern..."-Element im Filterbereich
- Ein Klick darauf öffnet ein kleines Formular (inline oder Modal) mit einem Textfeld für den Namen und einem Speichern-Button
- Nach dem Speichern verschwindet das Formular und der neue Filter erscheint in der Liste

**Aktivieren eines gespeicherten Filters:**
- Klick auf den Namen des gespeicherten Filters übernimmt dessen Query-Parameter und aktualisiert die URL
- Die Rezeptliste lädt die gefilterten Ergebnisse
- Die Einzelfilter im Filterbereich sind entsprechend als aktiv markiert (wie bei normalem manuellen Setzen)

**Löschen:**
- Klick auf das Löschen-Symbol entfernt den gespeicherten Filter direkt (mit HTMX ohne Seiten-Reload)
- Kein separater Bestätigungsdialog notwendig (da kein Rezept gelöscht wird)

---

## 5. Nicht-funktionale Anforderungen

### Performance
- Gespeicherte Filter-Liste lädt in < 500ms beim Seitenaufruf
- Aktivieren eines gespeicherten Filters liefert Ergebnisse in < 1 Sekunde (NFR-P1)

### Browser-Support
- Aktuelle Chrome, Firefox, Safari, Edge Versionen
- iOS Safari und Android Chrome (letzte 2 Versionen)

### Barrierefreiheit
- WCAG 2.1 Level A konform
- Gespeicherte Filter-Buttons haben aussagekräftige Labels
- Löschen-Buttons haben aria-label (z.B. "Filter 'Mittagessenplanung' löschen")
- Tastatur-Navigation vollständig unterstützt

---

## 6. Teststrategie

### E2E-Tests (Playwright)

**Testfall 1: Filter speichern und aufrufen**
```gherkin
Given Die App enthält Rezepte in Kategorie "Brot"
And Der Benutzer hat Kategorie "Brot" aktiv gesetzt
When Der Benutzer klickt auf "Filter speichern"
And Der Benutzer gibt den Namen "Brot-Ideen" ein und bestätigt
Then Der Filter "Brot-Ideen" erscheint in der Liste der gespeicherten Filter
When Der Benutzer entfernt alle aktiven Filter manuell
And Der Benutzer klickt auf den gespeicherten Filter "Brot-Ideen"
Then Die Kategorie "Brot" ist wieder aktiv
And Die Rezeptliste zeigt nur Brot-Rezepte
And Die URL enthält den Parameter "kategorie=brot"
```

**Testfall 2: Gespeicherter Filter ist persistent nach Reload**
```gherkin
Given Der Benutzer hat den Filter "Brot-Ideen" gespeichert
When Der Benutzer die Seite neu lädt
Then Der Filter "Brot-Ideen" ist weiterhin in der Liste der gespeicherten Filter sichtbar
```

**Testfall 3: Gespeicherten Filter löschen**
```gherkin
Given Der Benutzer hat den Filter "Brot-Ideen" gespeichert
When Der Benutzer klickt auf das Löschen-Symbol neben "Brot-Ideen"
Then Der Filter "Brot-Ideen" verschwindet aus der Liste
And Nach Seiten-Reload ist der Filter immer noch nicht vorhanden
```

**Testfall 4: Kombinierter Filter speichern und aufrufen**
```gherkin
Given Die App enthält Rezepte in Kategorie "Mittagessen" mit unterschiedlichen Bewertungen
And Der Benutzer hat Kategorie "Mittagessen" und Bewertungsfilter "Nur Gute" aktiv
When Der Benutzer speichert den Filter als "Mittagessenplanung"
And Der Benutzer setzt alle Filter zurück
And Der Benutzer klickt auf den gespeicherten Filter "Mittagessenplanung"
Then Nur Mittagessen-Rezepte mit guter Bewertung werden angezeigt
And Sowohl Kategorie-Filter als auch Bewertungsfilter sind als aktiv markiert
```

**Testfall 5: Doppelter Name**
```gherkin
Given Der Benutzer hat bereits einen Filter namens "Brot-Ideen" gespeichert
When Der Benutzer einen weiteren Filter unter dem Namen "Brot-Ideen" speichern möchte
Then Eine klare Fehlermeldung oder Überschreiben-Nachfrage erscheint
And Der ursprüngliche Filter "Brot-Ideen" bleibt erhalten, falls der Benutzer abbricht
```

**Testfall 6: Keine Treffer beim Aufrufen eines gespeicherten Filters**
```gherkin
Given Der Benutzer hat den Filter "Nächste 7 Tage" als gespeicherten Filter abgelegt
And Es gibt aktuell keine Rezepte mit Datum in den nächsten 7 Tagen
When Der Benutzer klickt auf den gespeicherten Filter
Then Die "Keine Treffer"-Meldung wird angezeigt
And Der gespeicherte Filter bleibt in der Liste erhalten
```

---

## 7. Abhängigkeiten & Rahmenbedingungen

### Abhängigkeiten

- Story 12 (Kombinierte Filter) muss abgeschlossen sein — die Filterparameter müssen vollständig als URL Query-Parameter funktionieren, da gespeicherte Filter auf dieser Basis aufbauen
- Story 12 ist abgeschlossen

### Rahmenbedingungen

- SQLite-Datenbank muss existieren und erreichbar sein
- Keine Authentifizierung erforderlich (LAN-only) — gespeicherte Filter sind für alle Geräte im LAN sichtbar
- DeepLink-fähige URLs mit Query-Parametern (Architektur-Vorgabe aus CLAUDE.md)
- Gespeicherte Filter werden server-seitig in SQLite gespeichert, nicht im Browser-LocalStorage — damit sind sie geräteübergreifend verfügbar

---

## Offene Punkte / Fragen

- [ ] Soll es eine Umbenennen-Funktion geben, oder reicht es, einen Filter zu löschen und neu zu speichern?
- [ ] Soll die Reihenfolge der gespeicherten Filter anpassbar sein (Drag & Drop), oder reicht eine feste Sortierung (z.B. nach Erstellungsdatum)?
- [ ] Soll ein gespeicherter Filter visuell als "aktiv" markiert werden, wenn seine Parameter gerade aktiv sind?

---

**Letzte Aktualisierung:** 2026-03-29
