# Story 16: Wochentag-Picker für intuitive Datumauswahl

**Epic:** Epic 4: Bewertung & Datums-Tracking
**Priorität:** MVP Phase 1
**Status:** In Arbeit

---

## 1. Story-Satz

Als **Haushalt** möchte ich beim Wochenplanen ein Rezept einem Wochentag zuordnen können, indem ich einfach auf einen Wochentag-Button klicke (z.B. "Do"), damit das System automatisch das richtige Datum für diesen Wochentag in der nächsten Woche berechnet und ins Datumsfeld einträgt — ohne dass ich ein Datum eintippen oder in einem Kalender navigieren muss.

---

## 2. Geschäftsbezogene Details

### Kontext

Bei der Wochenplanung (typischerweise Mittwoch oder Donnerstag) wählt der Haushalt 5-7 Rezepte für die kommende Woche. Für jedes ausgewählte Rezept soll ein Wochentag festgelegt werden. Das bestehende Datumsfeld (Story 28) akzeptiert Text im Format `T.M.JJJJ` und einen nativen Browser-Datepicker als Alternative — beides ist für eine schnelle Wochenplanung umständlich.

Der Wochentag-Picker löst dieses Problem: Statt ein konkretes Datum zu berechnen oder in einem Kalender zu navigieren, klickt der Nutzer einfach auf einen Wochentag-Button ("Mo", "Di", "Mi", "Do", "Fr", "Sa", "So"). Das System berechnet daraufhin automatisch das Datum des gewählten Wochentags in der nächsten Woche (ab dem nächsten Montag) und trägt es ins Datumsfeld ein.

**Kernidee aus dem PRD (Journey 1):**
> "Der Wochentag-Picker setzt automatisch das richtige Datum ins Feld 'Geplant am'. Kein Kalender-Fummelei, nur ein Klick."

### Nutzergruppe

Der gemeinsame Zwei-Personen-Haushalt. Beide Partner nutzen die App gleichberechtigt — auf Desktop, Tablet und Mobilgerät — ohne Login.

### Business-Value

- Reduziert den Aufwand für die Wochenplanung drastisch: Ein Klick statt manuelles Datum-Eintippen oder Kalender-Navigation
- Eliminiert das Nachdenken "Welches Datum ist nächster Donnerstag?" — das System berechnet es automatisch
- Macht das `planned_date`-Feature tatsächlich nutzbar: Der Wochentag-Picker senkt die Eingabe-Hürde so weit, dass das Datum regelmäßig gesetzt wird
- Unterstützt das Kernziel: Wochenplanung in 2 Minuten statt 20 Minuten

### Edge Cases

- **Welche Woche ist "nächste Woche"?** Die Wochentage beziehen sich immer auf die kommende Woche (ab dem nächsten Montag). Beispiel: Wenn heute Mittwoch, 26.03.2026 ist, ergibt "Do" → Donnerstag, 02.04.2026 (nicht der gestrige Donnerstag und nicht der übernächste).
  - Ausnahme: Wenn der gewählte Wochentag noch in der aktuellen Woche in der Zukunft liegt (z.B. heute ist Montag und "Fr" wird gewählt), könnte das System Freitag dieser Woche wählen — Entscheidung: immer "nächste Woche" für Vorhersehbarkeit.
- **Datum ist bereits gesetzt:** Wenn im Datumsfeld bereits ein Wert steht und der Nutzer einen Wochentag-Button klickt, wird das bestehende Datum durch das neue ersetzt (kein Dialog nötig, Last-write-wins).
- **Auswahl aufheben:** Ein erneuter Klick auf den aktiv markierten Wochentag-Button leert das Datumsfeld (Datum wird entfernt). Alternativ kann das Datum manuell aus dem Textfeld gelöscht werden.
- **Ohne JavaScript:** Der Wochentag-Picker ist eine JS-gestützte Komfort-Funktion. Ohne JS ist er nicht sichtbar. Die Texteingabe des Datums funktioniert weiterhin als Fallback (Progressive Enhancement).
- **Wochentag und manuell eingegebenes Datum stimmen nicht überein:** Kein Problem — nach einem manuellen Eintippen ins Textfeld wird der Wochentag-Button nicht mehr als aktiv markiert (der Button-Status reflektiert nur, was zuletzt über den Picker gesetzt wurde).

---

## 3. Akzeptanzkriterien

### Funktionale Kriterien

- [ ] **K1: Wochentag-Buttons werden angezeigt**
  - Im Rezeptformular (Erstellen und Bearbeiten) werden unterhalb oder neben dem Datumsfeld sieben Buttons angezeigt: "Mo", "Di", "Mi", "Do", "Fr", "Sa", "So"
  - Die Buttons sind nur sichtbar, wenn JavaScript verfügbar ist (progressive Enhancement)

- [ ] **K2: Klick auf Wochentag-Button setzt das Datum**
  - Ein Klick auf einen Wochentag-Button berechnet das Datum dieses Wochentags in der nächsten Woche (ab dem nächsten Montag)
  - Das berechnete Datum wird im deutschen Format (`T.M.JJJJ`) in das Datumsfeld eingetragen
  - Das Datumsfeld wird als normaler Formular-Wert gespeichert — der Wochentag-Picker ist nur ein Eingabehilfe-Widget

- [ ] **K3: Aktiver Wochentag wird visuell hervorgehoben**
  - Der zuletzt per Picker gewählte Wochentag-Button ist visuell als "aktiv" markiert (z.B. farblich hervorgehoben)
  - Bei initialem Öffnen des Formulars ist kein Button aktiv (es sei denn, ein vorhandenes Datum fällt auf einen Wochentag der nächsten Woche)
  - Wenn das Formular mit einem vorhandenen `planned_date` geöffnet wird, das auf einen Wochentag der nächsten Woche fällt, ist der entsprechende Button aktiv markiert

- [ ] **K4: Wochentag-Auswahl aufheben**
  - Ein erneuter Klick auf den aktiv markierten Wochentag-Button leert das Datumsfeld und entfernt die aktive Markierung
  - Das Leeren des Textfelds per Tastatur entfernt ebenfalls die aktive Markierung

- [ ] **K5: Manuelle Texteingabe und Wochentag-Picker koexistieren**
  - Nach einer manuellen Texteingabe ins Datumsfeld ist kein Wochentag-Button aktiv markiert (außer das eingetippte Datum entspricht zufällig einem Wochentag der nächsten Woche)
  - Der Wochentag-Picker überschreibt das Textfeld (der zuletzt gesetzte Wert gilt)

- [ ] **K6: Formular-Speichern funktioniert wie gewohnt**
  - Das über den Wochentag-Picker gesetzte Datum wird beim Speichern korrekt in der Datenbank gespeichert
  - Das Verhalten entspricht dem einer manuellen Texteingabe desselben Datums
  - Alle bestehenden Validierungen (K5 aus Story 28) gelten weiterhin

- [ ] **K7: Wochentag-Definition — "nächste Woche"**
  - Die Berechnung orientiert sich an der ISO-Woche: Montag ist der erste Tag der Woche
  - "Nächste Woche" bedeutet immer die Woche nach der aktuellen ISO-Woche, unabhängig vom heutigen Wochentag
  - Das berechnete Datum liegt immer in der Zukunft (mindestens in 1 Tag, maximal in 7 Tagen nach dem letzten Montag)

### Nicht-funktionale Kriterien

- [ ] **K8: Performance**
  - Die Datumberechnung erfolgt clientseitig per JavaScript, keine Server-Anfrage notwendig
  - Reaktion auf Button-Klick ist sofort (< 100ms wahrnehmbare Verzögerung)

- [ ] **K9: Barrierefreiheit**
  - Jeder Wochentag-Button hat ein aussagekräftiges `aria-label` (z.B. `aria-label="Montag nächste Woche wählen"`)
  - Aktiver Zustand wird per `aria-pressed="true"` kommuniziert
  - Buttons sind per Tastatur bedienbar (Tab + Enter/Space)
  - WCAG 2.1 Level A konform

- [ ] **K10: Mobilfreundlichkeit**
  - Die sieben Wochentag-Buttons passen auf einen schmalen Bildschirm (Mobile-first Layout)
  - Buttons sind groß genug für Touch-Bedienung (mindestens 44x44px Tap-Ziel)

---

## 4. Technische Planung

### Datenmodell

Keine Datenbank-Änderungen erforderlich. Das `planned_date`-Feld in der `recipes`-Tabelle existiert bereits (Story 28). Der Wochentag-Picker schreibt lediglich einen anderen Wert ins selbe Textfeld.

### UI/UX-Spezifikation

**Platzierung im Formular:**

```
Datum (geplant / gekocht)
[ 02.04.2026          ] [Kalender-Icon]

[Mo] [Di] [Mi] [Do] [Fr] [Sa] [So]
       Nächste Woche
```

- Die Wochentag-Buttons erscheinen direkt unterhalb des Datumsfelds
- Eine kleine Beschriftung ("Nächste Woche") kontextualisiert die Buttons
- Die Buttons sind kompakt, aber touch-freundlich

**Interaktionslogik (JavaScript):**

```
WOCHENTAG_OFFSET = { Mo: 0, Di: 1, Mi: 2, Do: 3, Fr: 4, Sa: 5, So: 6 }

Beim Klick auf Button "X":
  1. Aktuelles Datum bestimmen (clientseitig)
  2. Nächsten Montag berechnen (ISO-Woche + 1)
  3. Offset des gewählten Wochentags addieren
  4. Datum im Format T.M.JJJJ ins Textfeld schreiben
  5. Geklickten Button als "aktiv" markieren, alle anderen demarkieren

Beim erneuten Klick auf aktiven Button:
  1. Textfeld leeren
  2. Alle Buttons demarkieren

Beim manuellen Ändern des Textfelds:
  1. Alle Buttons demarkieren
  2. Prüfen: Entspricht der Wert einem Wochentag der nächsten Woche?
     Falls ja: Entsprechenden Button aktiv markieren
```

**Berechnung "nächste Woche":**
- `today` = aktuelles Datum auf dem Client-Gerät
- `dayOfWeek` = 0 (Mo) bis 6 (So) im ISO-Schema
- `daysUntilNextMonday` = (7 - dayOfWeek + 1) % 7 + 7
  (wenn heute Mo: 7 Tage, wenn heute Di: 6 Tage, ..., wenn heute So: 1 Tag — immer auf den übernächsten Montag)

  Vereinfachte Variante: `nextMonday` = heute + 7 Tage zurück auf Montag dieser Woche + 7 Tage

**Progressive Enhancement:**
- Die Wochentag-Buttons werden initial mit `display: none` gerendert (oder komplett weglassen und per JS einfügen)
- JavaScript macht sie sichtbar und fügt die Event-Listener hinzu
- Ohne JS: Nur das Textfeld und der Kalender-Icon sind sichtbar (wie in Story 28 definiert)

---

## 5. Nicht-funktionale Anforderungen

### Performance
- Seite lädt ohne sichtbare Verzögerung (< 500ms)
- Wochentag-Berechnung erfolgt clientseitig, keine Netzwerk-Anfrage notwendig
- Kein zusätzlicher Server-Overhead durch das Feature

### Browser-Support
- Aktuelle Chrome, Firefox, Safari, Edge Versionen
- iOS Safari und Android Chrome (letzte 2 Versionen)
- Ohne JavaScript: vollständiger Fallback auf Texteingabe (Story 28)
- Vanilla JS, keine Abhängigkeiten, kein Build-Step notwendig

### Barrierefreiheit
- WCAG 2.1 Level A konform
- `aria-pressed` für aktiven Zustand der Buttons
- Aussagekräftige `aria-label`-Attribute
- Fokus-Indikatoren sichtbar
- Tastatur-Navigation vollständig funktionsfähig

---

## 6. Teststrategie

### E2E-Tests (Playwright)

**Testfall 1: Wochentag wählen setzt korrektes Datum**
```gherkin
Given Die Seite "Rezept bearbeiten" ist geöffnet
And Heute ist Mittwoch, 01.04.2026
When Der Nutzer auf den Button "Do" klickt
Then Enthält das Datumsfeld "9.4.2026" (Donnerstag nächste Woche)
And Der Button "Do" ist visuell als aktiv markiert
```

**Testfall 2: Datum wird beim Speichern korrekt übernommen**
```gherkin
Given Die Seite "Rezept bearbeiten" ist geöffnet
And Heute ist Mittwoch, 01.04.2026
When Der Nutzer auf den Button "Fr" klickt
And Das Formular speichert
Then Zeigt die Detailansicht das Datum 10.04.2026 (Freitag nächste Woche)
```

**Testfall 3: Erneuter Klick auf aktiven Button leert das Feld**
```gherkin
Given Der Nutzer hat "Mo" gewählt und das Datumsfeld zeigt ein Datum
When Der Nutzer erneut auf "Mo" klickt
Then Ist das Datumsfeld leer
And Kein Wochentag-Button ist aktiv markiert
```

**Testfall 4: Manuelle Eingabe deaktiviert Wochentag-Markierung**
```gherkin
Given Der Nutzer hat "Di" gewählt und "Di" ist aktiv markiert
When Der Nutzer das Datumsfeld manuell ändert (auf "15.3.2026")
Then Ist kein Wochentag-Button aktiv markiert (außer 15.3.2026 fällt auf Di nächste Woche)
```

**Testfall 5: Vorhandenes Datum mit Wochentag der nächsten Woche**
```gherkin
Given Ein Rezept hat planned_date = nächsten Donnerstag
And Heute ist Mittwoch
When Der Nutzer die Bearbeiten-Seite öffnet
Then Ist der Button "Do" aktiv markiert
```

**Testfall 6: Ohne JavaScript — Fallback funktioniert**
```gherkin
Given JavaScript ist deaktiviert
When Der Nutzer die Rezept-Bearbeiten-Seite öffnet
Then Sind keine Wochentag-Buttons sichtbar
And Das Datumsfeld (Texteingabe) ist vorhanden und funktioniert
```

---

## 7. Abhängigkeiten & Rahmenbedingungen

### Abhängigkeiten

- Story 28 (Datum-Eingabe am Rezept: geplant/gekocht) muss implementiert sein — ist abgeschlossen
  - Das `planned_date`-Feld und das Datumsfeld im Formular (Texteingabe + Kalender-Icon) existieren bereits
- Story 01 (Rezept erstellen) und Story 02 (Rezept bearbeiten) müssen implementiert sein — beide abgeschlossen

### Rahmenbedingungen

- SQLite-Datenbank muss existieren und erreichbar sein
- Keine Authentifizierung erforderlich (LAN-only)
- Vanilla JavaScript — kein Framework, kein Build-Step (Projekt-Constraint)
- Das Datumsfeld wird weiterhin im deutschen Format `T.M.JJJJ` an den Server gesendet
- "Nächste Woche" bezieht sich auf den Client-Zeitstempel (Gerät des Nutzers) — serverseitig wird das Datum als String empfangen und geparst

---

## Offene Punkte / Fragen

- [ ] Soll der Wochentag-Picker auch direkt auf der Rezeptliste/Detailansicht als Schnellaktion erscheinen, oder nur im Edit-Formular? (MVP: nur im Formular, Post-MVP: ggf. Inline-Aktion)
- [ ] Soll "nächste Woche" immer die ISO-Woche nach der aktuellen sein, oder kann es auch "ab heute + 1 Tag" bedeuten (wenn heute z.B. Montag ist, soll "Mo" dann den nächsten Montag = 7 Tage später ergeben, oder den übernächsten = 14 Tage später)? → Vorschlag: Immer nächste ISO-Woche (Montag bis Sonntag), Montag ist immer mindestens 2 Tage und maximal 8 Tage in der Zukunft.

---

**Letzte Aktualisierung:** 2026-03-29
