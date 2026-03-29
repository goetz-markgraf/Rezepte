# Story 14: Rezept mit 3-5 Sternen bewerten

**Epic:** Bewertung & Datums-Tracking
**Priorität:** MVP - Phase 1
**Status:** In Arbeit

---

## 1. Story-Satz

**Als** Benutzer möchte ich **ein Rezept mit 3, 4 oder 5 Sternen bewerten**, damit ich **meine Lieblingsrezepte erkenne und schlechte Rezepte aus der Wochenplanung ausschließen kann**.

---

## 2. Geschäftsbezogene Details

### Kontext

Die Bewertung ist ein zentrales Element für die Wochenplanung. Ohne Bewertungen kann der Filter "Beliebtheit" (Story 11) nicht sinnvoll genutzt werden. Die Bewertung ermöglicht außerdem, Rezepte mit schlechtem Ergebnis dauerhaft auszublenden, ohne sie zu löschen - ein ausdrücklich gewünschtes Verhalten laut PRD ("Schlecht bewerten statt löschen").

Das Bewertungsschema ist bewusst auf 3-5 Sterne beschränkt (keine 1-2 Sterne im normalen Gebrauch). Rezepte mit 1-2 Sternen werden als "negativ bewertet" behandelt und aus Vorschlägen ausgeschlossen. Die normale Nutzung verwendet 3 Sterne (gut), 4 Sterne (sehr gut) und 5 Sterne (Favorit).

### Nutzergruppe

- Beide Partner des Haushalts (Anna & Dragon)
- Typischerweise unmittelbar nach oder während des Essens, meist vom Handy

### Business-Value

- Ermöglicht den Beliebtheit-Filter (Story 11) zur intelligenten Wochenplanung
- Fördert die Wiederentdeckung vergessener 5-Sterne-Favoriten
- Alternative zum Löschen bei misslungenen Rezepten (schlechte Bewertung statt Datenverlust)
- Sterne-Anzeige in der Rezeptliste macht Qualität auf einen Blick sichtbar

### Edge Cases

- **Keine Bewertung (leer):** Zulässig - viele Rezepte werden erst nach dem Kochen bewertet. Unbewertete Rezepte erscheinen im Filter "Noch nicht bewertet".
- **Bewertung entfernen:** Benutzer soll eine bereits gesetzte Bewertung wieder auf "keine Bewertung" zurücksetzen können.
- **Bewertung 1-2 Sterne:** Möglich als Negativbewertung (z.B. "Rezept nicht nochmal kochen"). Diese Rezepte sollen aus Vorschlägen und dem Beliebtheit-Filter ausgeschlossen sein.
- **Bewertung ändern:** Bewertung kann jederzeit überschrieben werden (last-write-wins).
- **Ungültige Werte:** Werte außerhalb 1-5 werden abgelehnt.

---

## 3. Akzeptanzkriterien

### Funktionale Kriterien

- [ ] **K1: Bewertungsfeld im Bearbeitungsformular**
  - Im Formular "Rezept bearbeiten" gibt es ein Sterne-Auswahlfeld für 1-5 Sterne
  - Das Feld ist optional (keine Pflichtangabe)
  - Die aktuelle Bewertung ist vorausgewählt, falls vorhanden
  - Eine leere Auswahl (keine Bewertung) ist möglich

- [ ] **K2: Bewertung speichern**
  - Nach dem Speichern des Formulars wird die Bewertung in der Datenbank persistiert
  - Bei leerem Bewertungsfeld wird `NULL` gespeichert (keine Bewertung)
  - Bewertungen 1-5 werden korrekt gespeichert und angezeigt

- [ ] **K3: Bewertung in der Detailansicht anzeigen**
  - Auf der Rezept-Detailseite wird die Bewertung als Sterne-Symbole angezeigt (z.B. ★★★★☆ für 4 Sterne)
  - Unbewertete Rezepte zeigen keinen Sterne-Block oder eine neutrale Kennzeichnung

- [ ] **K4: Bewertung in der Rezeptliste anzeigen**
  - In der Rezeptübersicht (Liste) wird die Bewertung neben dem Rezepttitel angezeigt
  - Unbewertete Rezepte zeigen keine Sterne (kein leerer Platzhalter, der Platz wegnimmt)

- [ ] **K5: Bewertung zurücksetzen**
  - Im Bearbeitungsformular kann die Bewertung auf "keine Bewertung" gesetzt werden
  - Nach dem Speichern ist die Bewertung entfernt

- [ ] **K6: Negativ-Bewertungen (1-2 Sterne)**
  - Rezepte mit 1 oder 2 Sternen sind zulässig und werden gespeichert
  - Diese Rezepte erscheinen in der normalen Listenansicht (kein automatischer Filter)
  - Der spätere Beliebtheit-Filter (Story 11) schließt 1-2 Sterne aus - das ist Teil von Story 11, nicht dieser Story

### Nicht-funktionale Kriterien

- [ ] **K7: Performance**
  - Formular mit Bewertungsfeld lädt in < 500ms
  - Speichervorgang inkl. Bewertung < 1s

- [ ] **K8: Barrierefreiheit**
  - Sterne-Auswahl ist per Tastatur navigierbar (Tab + Pfeiltasten oder Radio-Buttons)
  - Labels für Screenreader vorhanden ("1 Stern", "2 Sterne", ..., "5 Sterne")
  - WCAG 2.1 Level A konform

---

## 4. Technische Planung

### Datenmodell

Das bestehende `Recipe`-Struct und die `recipes`-Tabelle müssen um ein `rating`-Feld erweitert werden:

| Feld | Typ | Beschreibung |
|------|-----|--------------|
| rating | INTEGER NULL | Bewertung 1-5, NULL = keine Bewertung |

**Migration:** ALTER TABLE oder neue Migration-Datei mit `ADD COLUMN rating INTEGER NULL`.

**Validierung:** Werte außerhalb 1-5 werden server-seitig abgelehnt.

### UI/UX-Spezifikation

**Bearbeitungsformular:**
- Sterne-Auswahl als Radio-Buttons (1-5) mit optischem Sterne-Styling
- Zusätzliche "Keine Bewertung"-Option als Reset-Möglichkeit
- Platzierung: nach den Kategorien, vor den optionalen Textfeldern

**Detailansicht:**
- Volle Sterne (★) für gewählte Sterne, leere (☆) für nicht gewählte
- Beispiel: 4 von 5 → ★★★★☆
- Kein Block wenn unbwertet

**Rezeptliste:**
- Kompakte Sterne-Anzeige neben dem Rezepttitel
- Nur ausgefüllte Sterne (z.B. ★★★★), keine leeren Platzhalter
- Responsive: auf kleinen Bildschirmen ausreichend Tippfläche

---

## 5. Nicht-funktionale Anforderungen

### Performance
- Formular-Ladezeit < 500ms
- Speichervorgang < 1s
- Keine sichtbare Verzögerung beim Wechsel der Sterne-Auswahl

### Browser-Support
- Aktuelle Chrome, Firefox, Safari, Edge Versionen
- iOS Safari und Android Chrome (letzte 2 Versionen)
- Touch-freundliche Sterne-Auswahl auf Mobilgeräten (Mindest-Tippfläche 44x44px)

### Barrierefreiheit
- WCAG 2.1 Level A konform
- Sterne-Auswahl per Tastatur bedienbar
- Semantische Labels für Screenreader

---

## 6. Teststrategie

### E2E-Tests (Playwright)

**Testfall 1: Bewertung setzen**
```gherkin
Given Ein Rezept ohne Bewertung existiert in der Datenbank
When Benutzer öffnet das Rezept zum Bearbeiten
And Benutzer wählt 4 Sterne aus
And Benutzer klickt auf "Speichern"
Then Das Rezept wird mit 4 Sternen gespeichert
And In der Detailansicht werden 4 ausgefüllte Sterne angezeigt
```

**Testfall 2: Bewertung ändern**
```gherkin
Given Ein Rezept mit 3-Sterne-Bewertung existiert in der Datenbank
When Benutzer öffnet das Rezept zum Bearbeiten
And Benutzer ändert die Bewertung auf 5 Sterne
And Benutzer klickt auf "Speichern"
Then Das Rezept wird mit 5 Sternen gespeichert
And In der Detailansicht werden 5 ausgefüllte Sterne angezeigt
```

**Testfall 3: Bewertung entfernen**
```gherkin
Given Ein Rezept mit 5-Sterne-Bewertung existiert in der Datenbank
When Benutzer öffnet das Rezept zum Bearbeiten
And Benutzer setzt die Bewertung auf "Keine Bewertung"
And Benutzer klickt auf "Speichern"
Then Das Rezept hat keine Bewertung mehr
And In der Detailansicht wird kein Sterne-Block angezeigt
```

**Testfall 4: Bewertung in der Listenansicht**
```gherkin
Given Rezept A mit 5 Sternen und Rezept B ohne Bewertung existieren
When Benutzer öffnet die Rezeptliste
Then Rezept A zeigt 5 Sterne-Symbole
And Rezept B zeigt keine Sterne
```

---

## 7. Abhängigkeiten & Rahmenbedingungen

### Abhängigkeiten

- Story 02 (Rezept bearbeiten) muss implementiert sein - die Bewertung wird im Bearbeitungsformular gesetzt
- Story 04 (Rezept-Detailansicht) muss implementiert sein - Anzeige der Bewertung in der Detailansicht
- Story 05 (Rezeptliste) muss implementiert sein - Anzeige der Bewertung in der Liste
- Blockiert: Story 11 (Filter nach Bewertung) - benötigt das `rating`-Datenbankfeld

### Rahmenbedingungen

- SQLite-Datenbank muss existieren und erreichbar sein
- Keine Authentifizierung erforderlich (LAN-only)
- Single-User-Modell (beide Partner = gleicher User, last-write-wins)
- Datenbankschema muss per Migration erweitert werden (neues `rating`-Feld)

---

## Offene Punkte / Fragen

- Sollen 1-2 Sterne im UI explizit als "Negativbewertung" gekennzeichnet sein, oder reicht die implizite Konvention?

---

**Letzte Aktualisierung:** 2026-03-29
