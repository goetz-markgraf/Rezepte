# Story 3: Rezept löschen mit Sicherheitsabfrage

**Epic:** Rezept-Verwaltung (Grundlegendes CRUD)
**Priorität:** MVP - Phase 1
**Status:** Offen

---

## 1. Story-Satz

**Als** Benutzer möchte ich **ein Rezept löschen können, wobei ich vorher um Bestätigung gebeten werde**, damit ich **versehentliches Löschen von Rezepten verhindere und trotzdem die volle Kontrolle über meine Rezeptsammlung behalte**.

---

## 2. Geschäftsbezogene Details

### Kontext

Das Löschen von Rezepten ist ein notwendiger Bestandteil der Rezeptverwaltung. Rezepte können aus verschiedenen Gründen gelöscht werden: Duplikate, veraltete Rezepte oder fehlerhafte Einträge. Da die Anwendung ohne Login und ohne Undo-Funktion arbeitet, ist ein versehentliches Löschen unwiderruflich. Eine Sicherheitsabfrage schützt vor diesem Datenverlust.

Gleichzeitig soll die Sicherheitsabfrage den Nutzer auf eine Alternative hinweisen: Statt ein Rezept zu löschen, könnte es sinnvoller sein, es mit einer schlechten Bewertung zu versehen, damit es nicht mehr in Vorschlägen auftaucht, aber dennoch im System erhalten bleibt.

### Nutzergruppe

- Beide Partner des Haushalts (gleiche Berechtigungen, kein Rollenmodell)
- LAN-Zugriff über verschiedene Geräte (Desktop, Tablet, Handy)

### Business-Value

- Schutz vor unbeabsichtigtem Datenverlust in einer Anwendung ohne Authentifizierung und ohne Undo
- Ermöglicht das Bereinigen der Rezeptsammlung (Duplikate, Fehleingaben)
- Bewusstes Lenken des Nutzers zur Bewertungs-Alternative statt hartem Löschen

### Edge Cases

- **Versehentliches Tippen auf "Löschen":** Die Sicherheitsabfrage verhindert sofortige Löschung; "Abbrechen" ist die prominente Option
- **Rezept mit Planungsdatum in der Zukunft:** Kein spezieller Hinweis notwendig; das Rezept wird wie jedes andere gelöscht
- **Netzwerkfehler beim Löschen:** Fehlermeldung anzeigen, Rezept bleibt erhalten
- **Doppeltes Absenden (doppelter Klick):** Idempotente Verarbeitung, Rezept wird genau einmal gelöscht

---

## 3. Akzeptanzkriterien

### Funktionale Kriterien

- [ ] **K1: Lösch-Auslöser vorhanden**
  - Auf der Rezept-Detailansicht oder im Bearbeitungsformular gibt es eine "Löschen"-Schaltfläche
  - Die Schaltfläche ist klar als destruktive Aktion erkennbar (z.B. durch rote Farbe oder Warnsymbol)

- [ ] **K2: Sicherheitsabfrage erscheint**
  - Nach Klick auf "Löschen" erscheint eine Sicherheitsabfrage
  - Die Abfrage nennt den Titel des Rezepts: "Rezept '[Titel]' wirklich löschen?"
  - Die Abfrage enthält einen Hinweis auf die Bewertungs-Alternative
  - Die Abfrage bietet drei Optionen: "Abbrechen", "Wirklich löschen"
  - Die Abfrage blockiert die weitere Interaktion mit der Seite (Modal oder eigene Seite)

- [ ] **K3: Abbrechen funktioniert**
  - Klick auf "Abbrechen" schließt die Abfrage ohne Änderungen
  - Der Benutzer verbleibt auf der aktuellen Seite
  - Das Rezept ist unverändert in der Datenbank vorhanden

- [ ] **K4: Löschen wird ausgeführt**
  - Klick auf "Wirklich löschen" entfernt das Rezept dauerhaft aus der Datenbank
  - Nach erfolgreichem Löschen wird der Benutzer zur Rezeptliste weitergeleitet
  - Eine Erfolgsmeldung bestätigt das Löschen: "Rezept '[Titel]' wurde gelöscht"

- [ ] **K5: Datenpersistenz sichergestellt**
  - Das gelöschte Rezept ist nach der Aktion nicht mehr in der Datenbank vorhanden
  - Alle zugehörigen Daten (Kategorien, Zutaten, Anleitung, Datum) werden mit entfernt
  - Das Rezept erscheint nicht mehr in der Rezeptliste oder Suchergebnissen

### Nicht-funktionale Kriterien

- [ ] **K6: Performance**
  - Löschvorgang dauert < 500ms
  - Weiterleitung zur Rezeptliste erfolgt unmittelbar nach Löschbestätigung

- [ ] **K7: Barrierefreiheit**
  - Die Sicherheitsabfrage ist per Tastatur vollständig bedienbar (Tab, Enter, Escape)
  - "Abbrechen" ist per Escape-Taste erreichbar
  - Die Schaltflächen haben korrekte ARIA-Labels
  - Der Fokus wird beim Öffnen der Abfrage korrekt gesetzt

---

## 4. Technische Planung

### Datenmodell

Keine Änderungen am Datenbankschema erforderlich. Das Löschen entfernt den bestehenden Eintrag aus der `recipes`-Tabelle via `DELETE FROM recipes WHERE id = ?`.

### UI/UX-Spezifikation

**Lösch-Schaltfläche:**
- Platzierung im Bearbeitungsformular und/oder der Detailansicht des Rezepts
- Visuell als destruktive Aktion gekennzeichnet

**Sicherheitsabfrage (Confirmation Dialog):**
- Zeigt Rezepttitel an
- Erklärt die Konsequenz (unwiderruflich)
- Hinweis auf Alternative: "Tipp: Statt zu löschen, kannst du eine schlechte Bewertung vergeben. Dann wird es nicht mehr vorgeschlagen, aber bleibt im System."
- Drei Aktionsoptionen:
  - "Abbrechen" (primäre/prominente Schaltfläche)
  - "Wirklich löschen" (sekundäre, als Warnung gestaltete Schaltfläche)
- Umsetzung: Eigene Bestätigungsseite (funktioniert ohne JS) oder HTMX-Modal

**Nach dem Löschen:**
- Weiterleitung zur Rezeptliste
- Erfolgsmeldung (Toast oder Banner) mit Rezepttitel

---

## 5. Nicht-funktionale Anforderungen

### Performance
- Löschvorgang abgeschlossen in < 500ms
- Seiten-Weiterleitung ohne wahrnehmbare Verzögerung

### Browser-Support
- Aktuelle Chrome, Firefox, Safari, Edge Versionen
- iOS Safari und Android Chrome (letzte 2 Versionen)
- Funktioniert auch ohne JavaScript (Form-Post + Redirect)

### Barrierefreiheit
- WCAG 2.1 Level A konform
- Fokus-Indikatoren sichtbar
- Tastatur-Navigation vollständig (Tab, Escape, Enter)

---

## 6. Teststrategie

### E2E-Tests (Playwright)

**Testfall 1: Sicherheitsabfrage erscheint**
```gherkin
Given Ein Rezept "Testrezept" existiert in der Datenbank
And Benutzer befindet sich auf der Detailansicht oder dem Bearbeitungsformular von "Testrezept"
When Benutzer klickt auf "Löschen"
Then Die Sicherheitsabfrage wird angezeigt
And Der Titel "Testrezept" ist in der Abfrage sichtbar
And Die Schaltflächen "Abbrechen" und "Wirklich löschen" sind vorhanden
```

**Testfall 2: Abbrechen verhindert Löschung**
```gherkin
Given Die Sicherheitsabfrage für "Testrezept" ist geöffnet
When Benutzer klickt auf "Abbrechen"
Then Die Abfrage wird geschlossen
And "Testrezept" ist weiterhin in der Datenbank vorhanden
And Benutzer verbleibt auf der vorherigen Seite
```

**Testfall 3: Erfolgreiches Löschen**
```gherkin
Given Die Sicherheitsabfrage für "Testrezept" ist geöffnet
When Benutzer klickt auf "Wirklich löschen"
Then "Testrezept" wird aus der Datenbank entfernt
And Benutzer wird zur Rezeptliste weitergeleitet
And Eine Erfolgsmeldung mit dem Rezepttitel wird angezeigt
And "Testrezept" erscheint nicht mehr in der Rezeptliste
```

---

## 7. Abhängigkeiten & Rahmenbedingungen

### Abhängigkeiten

- **Story 01 (Rezept erstellen)** muss abgeschlossen sein - Rezepte müssen existieren, um gelöscht werden zu können
- Wird durch Story 04 (Rezept-Detailansicht) ergänzt - der genaue Platzierungsort des Lösch-Buttons hängt von der Detailansicht ab

### Rahmenbedingungen

- SQLite-Datenbank muss existieren und erreichbar sein
- Keine Authentifizierung erforderlich (LAN-only)
- Löschung ist unwiderruflich - kein Papierkorb oder Undo-Funktion geplant
- Single-User-Modell (beide Partner = gleicher User, keine Berechtigungsprüfung)

---

## Offene Punkte / Fragen

- [ ] Wird die Sicherheitsabfrage als eigene Seite (funktioniert ohne JS) oder als HTMX-Modal umgesetzt?
- [ ] Soll der "Löschen"-Button nur im Bearbeitungsformular oder auch in der Detailansicht vorhanden sein?

---

**Letzte Aktualisierung:** 2026-03-27
