:
# Story 1: Rezept erstellen

**Epic:** Rezept-Verwaltung (Grundlegendes CRUD)
**Priorität:** MVP - Phase 1
**Status:** Offen

---

## 1. Story-Satz

**Als** Benutzer möchte ich **ein neues Rezept mit Titel, Kategorie, Zutaten und Anleitung erstellen**, damit ich **meine Lieblingsrezepte digital speichern und jederzeit wiederfinden kann**.

---

## 2. Geschäftsbezogene Details

### Kontext
Das Erstellen neuer Rezepte ist die Kernfunktion der Anwendung. Ohne diese Funktion kann keine Rezeptsammlung aufgebaut werden. Sie ist der Einstiegspunkt für alle weiteren Features (Bewertung, Planung, etc.).

### Nutzergruppe
- Beide Partner des Haushalts (gleiche Berechtigungen)
- LAN-Zugriff über verschiedene Geräte (Desktop, Tablet, Handy)

### Business-Value
- Ermöglicht die digitale Sammlung aller Rezepte an einem zentralen Ort
- Ersetzt lose Zettel und ausgedruckte Webseiten
- Grundlage für alle weiteren Features (Wochenplanung, Favoriten, etc.)

### Edge Cases
- **Leerer Titel:** Validierung erforderlich, Fehlermeldung anzeigen
- **Doppelter Titel:** Hinweis auf mögliches Duplikat (Vorschau auf vorhandenes Rezept)
- **Sehr lange Texte:** Textarea mit angemessener Zeilenbegrenzung
- **Verbindungsabbruch:** Formular sollte bei erneutem Laden die Eingaben behalten (optional: LocalStorage)

---

## 3. Akzeptanzkriterien

### Funktionale Kriterien

- [ ] **K1: Formularfelder vorhanden**
  - Titel (Pflichtfeld, max. 100 Zeichen)
  - Kategorien (Pflichtfeld, Multi-Select: Mittagessen, Brot, Party, Kuchen, Snacks - mindestens eine auswählen)
  - Zutaten (Textarea, optional, max. 2000 Zeichen)
  - Anleitung (Textarea, optional, max. 5000 Zeichen)
  - Speichern-Button

- [ ] **K2: Pflichtfeld-Validierung**
  - Bei leerem Titel oder keiner ausgewählten Kategorie: Fehlermeldung "Bitte füllen Sie alle Pflichtfelder aus"
  - Keine Speicherung bei ungültigen Daten
  - Fehler werden direkt am Feld angezeigt

- [ ] **K3: Erfolgreiche Speicherung**
  - Nach erfolgreichem Speichern: Weiterleitung zu einer rudimentären Bestätigungsseite (H1: "Rezept erstellt", rudimentäre Anzeige des Rezepts)
  - Erfolgsmeldung "Rezept erfolgreich erstellt" (Toast oder Banner)
  - Rezept ist in der Datenbank gespeichert

- [ ] **K4: Datenpersistenz**
  - Rezept wird in SQLite-Datenbank gespeichert
  - Alle Felder werden korrekt abgespeichert
  - Timestamp "created_at" wird automatisch gesetzt

- [ ] **K5: Formular-Reset**
  - "Neues Rezept"-Button auf der rudimentären Startseite (H1: "Rezepte Übersicht") öffnet leeres Formular
  - Nach erfolgreicher Erstellung ist das Formular für weitere Eingaben bereit

### Nicht-funktionale Kriterien

- [ ] **K6: Performance**
  - Formular-Ladezeit < 500ms
  - Speichervorgang < 1s
  - Keine Blockierung der UI während des Speicherns

- [ ] **K7: Barrierefreiheit**
  - Alle Formularfelder haben korrekte Labels (WCAG 2.1 Level A)
  - Tastatur-Navigation funktioniert vollständig
  - Fehlermeldungen sind mit Screenreadern lesbar

---

## 4. Technische Planung

### Datenmodell

**Tabelle: recipes**

| Feld | Typ | Beschreibung |
|------|-----|--------------|
| id | INTEGER PRIMARY KEY | Auto-increment |
| title | TEXT NOT NULL | Rezept-Titel |
| categories | JSON-Array NOT NULL | Mehrere Kategorien aus: Mittagessen, Brot, Party, Kuchen, Snacks |
| ingredients | TEXT | Zutaten-Liste |
| instructions | TEXT | Zubereitungsanleitung |
| created_at | TIMESTAMP | Automatisch gesetzt |
| updated_at | TIMESTAMP | Automatisch gesetzt |

### UI/UX-Spezifikation

**Layout:**
- Zentriertes Formular (max. 800px Breite)
- Klare Abschnitte: Titel → Kategorien → Zutaten → Anleitung
- Markdown-ähnliche Textareas für bessere Lesbarkeit (optional)
- Cancel-Button zum Abbrechen (zurück zur rudimentären Startseite)

**Responsive:**
- Desktop: Zweispaltige Labels
- Mobile: Einspaltig, volle Breite

---

## 5. Nicht-funktionale Anforderungen

### Performance
- Seite lädt ohne sichtbare Verzögerung
- Formular ist nach < 500ms interaktiv

### Browser-Support
- Aktuelle Chrome, Firefox, Safari, Edge Versionen
- iOS Safari (letzte 2 Versionen)
- Android Chrome (letzte 2 Versionen)

### Barrierefreiheit
- WCAG 2.1 Level A konform
- ARIA-Labels für Formularfelder
- Fokus-Indikatoren sichtbar

---

## 6. Teststrategie

### E2E-Tests (Playwright)

**Testfall 1: Erfolgreiche Erstellung**
```gherkin
Given Benutzer ist auf der rudimentären Startseite (H1: "Rezepte Übersicht")
When Benutzer klickt auf "Neues Rezept"
And Benutzer gibt Titel "Testrezept" ein
And Benutzer wählt Kategorie "Mittagessen"
And Benutzer klickt auf "Speichern"
Then Rezept wird erstellt
And Benutzer sieht eine Seite mit H1 "Rezept erstellt"
And Der Titel "Testrezept" wird auf der Seite angezeigt
And Erfolgsmeldung wird angezeigt
```

**Testfall 2: Validierung - Fehlende Pflichtfelder**
```gherkin
Given Benutzer ist auf der "Neues Rezept"-Seite
When Benutzer klickt auf "Speichern" ohne Eingaben
Then Fehlermeldung wird angezeigt
And Kein Rezept wird erstellt
```

**Testfall 3: Eingabe aller Felder**
```gherkin
Given Benutzer ist auf der "Neues Rezept"-Seite
When Benutzer füllt alle Felder aus (Titel, Kategorie, Zutaten, Anleitung)
And Benutzer klickt auf "Speichern"
Then Rezept wird mit allen Daten gespeichert
And Benutzer sieht eine Seite mit H1 "Rezept erstellt"
And Der Titel und die eingegebenen Daten werden auf der Seite angezeigt
```

---

## 7. Abhängigkeiten & Rahmenbedingungen

### Abhängigkeiten
- Keine externen Abhängigkeiten
- Blockiert: Story 2 (Rezept bearbeiten) - benötigt diese Story
- Story 4 (Rezept-Detailansicht) wird erst später implementiert, daher rudimentäre Seite als Platzhalter

### Rahmenbedingungen
- SQLite-Datenbank muss existieren und erreichbar sein
- Keine Authentifizierung erforderlich (LAN-only)
- Single-User-Modell (beide Partner = gleicher User)

---

## Offene Punkte / Fragen

- [ ] Soll es einen "Speichern & Weiteres erstellen"-Button geben?
- [ ] Sollen Zutaten strukturiert (Menge, Einheit, Zutat) oder als Freitext eingegeben werden?
- [ ] Soll es eine Vorschau-Funktion vor dem Speichern geben?

---

**Letzte Aktualisierung:** 2026-03-21
