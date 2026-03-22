# Story 2: Rezept bearbeiten

**Epic:** Rezept-Verwaltung (Grundlegendes CRUD)
**Priorität:** MVP - Phase 1
**Status:** Offen

---

## 1. Story-Satz

**Als** Benutzer möchte ich **ein bestehendes Rezept bearbeiten können**, damit ich **Fehler korrigieren, Zutaten ergänzen oder die Anleitung verbessern kann**.

---

## 2. Geschäftsbezogene Details

### Kontext
Nach dem Erstellen eines Rezepts kommt es häufig vor, dass Details geändert werden müssen - sei es ein vergessenes Gewürz, eine Korrektur in der Zubereitungszeit oder eine verbesserte Formulierung der Anleitung. Diese Funktion ist essenziell für die Datenqualität und Nutzerzufriedenheit.

### Nutzergruppe
- Beide Partner des Haushalts (gleiche Berechtigungen)
- LAN-Zugriff über verschiedene Geräte (Desktop, Tablet, Handy)

### Business-Value
- Ermöglicht kontinuierliche Verbesserung der Rezepte
- Vermeidet Löschen und Neu-Erstellen bei kleinen Änderungen
- Wichtig für die Akzeptanz der digitalen Rezeptsammlung

### Edge Cases
- **Gleichzeitige Bearbeitung:** Last-write-wins (beide Partner = gleicher User)
- **Rezept wurde gelöscht:** Angemessene Fehlermeldung anzeigen
- **Verbindungsabbruch:** Formular sollte bei erneutem Laden die Eingaben behalten
- **Ungültige Daten nach Bearbeitung:** Gleiche Validierung wie beim Erstellen

---

## 3. Akzeptanzkriterien

### Funktionale Kriterien

- [ ] **K1: Bearbeiten-Button zugänglich**
  - Auf der Rezept-Detailseite (Story 4) oder in der Rezept-Liste
  - Button "Bearbeiten" ist sichtbar und klickbar
  - Klick öffnet das Bearbeitungsformular

- [ ] **K2: Formular mit bestehenden Daten**
  - Alle Felder sind mit den aktuellen Werten des Rezepts vorausgefüllt
  - Titel, Kategorien, Zutaten, Anleitung werden korrekt geladen
  - Änderungen können vorgenommen werden

- [ ] **K3: Pflichtfeld-Validierung**
  - Bei leerem Titel oder keiner ausgewählten Kategorie: Fehlermeldung "Bitte füllen Sie alle Pflichtfelder aus"
  - Keine Speicherung bei ungültigen Daten
  - Fehler werden direkt am Feld angezeigt

- [ ] **K4: Erfolgreiche Speicherung**
  - Nach erfolgreichem Speichern: Weiterleitung zur Rezept-Detailansicht
  - Erfolgsmeldung "Rezept erfolgreich aktualisiert" (Toast oder Banner)
  - Änderungen sind in der Datenbank gespeichert

- [ ] **K5: Timestamp-Aktualisierung**
  - Feld "updated_at" wird automatisch auf aktuelles Datum/Zeit gesetzt
  - "created_at" bleibt unverändert

- [ ] **K6: Abbrechen-Option**
  - "Abbrechen"-Button vorhanden
  - Bei Klick: Zurück zur Rezept-Detailansicht ohne Speichern
  - Bestätigungsdialog bei ungespeicherten Änderungen (optional)

### Nicht-funktionale Kriterien

- [ ] **K7: Performance**
  - Formular-Ladezeit < 500ms (mit vorausgefüllten Daten)
  - Speichervorgang < 1s
  - Keine Blockierung der UI während des Speicherns

- [ ] **K8: Barrierefreiheit**
  - Alle Formularfelder haben korrekte Labels (WCAG 2.1 Level A)
  - Tastatur-Navigation funktioniert vollständig
  - Fehlermeldungen sind mit Screenreadern lesbar

---

## 4. Technische Planung

### Datenmodell

**Tabelle: recipes** (bereits aus Story 1 vorhanden)

| Feld | Typ | Beschreibung |
|------|-----|--------------|
| id | INTEGER PRIMARY KEY | Auto-increment |
| title | TEXT NOT NULL | Rezept-Titel |
| categories | JSON-Array NOT NULL | Mehrere Kategorien |
| ingredients | TEXT | Zutaten-Liste |
| instructions | TEXT | Zubereitungsanleitung |
| created_at | TIMESTAMP | Unverändert bei Bearbeitung |
| updated_at | TIMESTAMP | Automatisch aktualisiert |

### UI/UX-Spezifikation

**Zugang zum Bearbeiten:**
- Button "Bearbeiten" auf der Rezept-Detailseite (oben rechts oder unter dem Titel)
- Alternativ: Bearbeiten-Icon in der Rezept-Liste (optional für später)

**Bearbeitungsformular:**
- Identisch zum Erstellungsformular (Story 1)
- Alle Felder mit bestehenden Werten vorausgefüllt
- "Speichern" und "Abbrechen" Buttons
- Visuelle Hervorhebung, dass dies ein Bearbeiten-Modus ist (z.B. "Rezept bearbeiten" als Überschrift)

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

**Testfall 1: Erfolgreiche Bearbeitung**
```gherkin
Given Ein Rezept "Tomatensuppe" existiert in der Datenbank
And Benutzer ist auf der Rezept-Detailseite von "Tomatensuppe"
When Benutzer klickt auf "Bearbeiten"
And Benutzer ändert den Titel zu "Cremige Tomatensuppe"
And Benutzer fügt "Basilikum" zu den Zutaten hinzu
And Benutzer klickt auf "Speichern"
Then Rezept wird aktualisiert
And Benutzer sieht die Rezept-Detailseite mit neuem Titel "Cremige Tomatensuppe"
And Erfolgsmeldung wird angezeigt
And "updated_at" wurde aktualisiert
```

**Testfall 2: Abbrechen ohne Speichern**
```gherkin
Given Ein Rezept "Tomatensuppe" existiert in der Datenbank
And Benutzer ist auf der Bearbeiten-Seite
When Benutzer ändert den Titel zu "Neuer Titel"
And Benutzer klickt auf "Abbrechen"
Then Benutzer wird zur Detailseite zurückgeleitet
And Der Titel ist weiterhin "Tomatensuppe" (nicht geändert)
```

**Testfall 3: Validierung bei Bearbeitung**
```gherkin
Given Ein Rezept existiert in der Datenbank
And Benutzer ist auf der Bearbeiten-Seite
When Benutzer löscht den Titel (leeres Feld)
And Benutzer klickt auf "Speichern"
Then Fehlermeldung wird angezeigt
And Keine Änderung wird gespeichert
```

---

## 7. Abhängigkeiten & Rahmenbedingungen

### Abhängigkeiten
- **Story 1 (Rezept erstellen):** Muss implementiert sein (Datenmodell, Formular-Struktur)
- **Story 4 (Rezept-Detailansicht):** Notwendig für den Zugang zum Bearbeiten-Button
  - Alternativ: Temporäre rudimentäre Detailseite als Platzhalter

### Rahmenbedingungen
- SQLite-Datenbank muss existieren und erreichbar sein
- Keine Authentifizierung erforderlich (LAN-only)
- Single-User-Modell (beide Partner = gleicher User)
- Last-write-wins bei Konflikten

---

## Offene Punkte / Fragen

- [ ] Soll es eine Warnung geben, wenn ein anderer Benutzer das Rezept gerade bearbeitet?
- [ ] Soll es ein "Rückgängig"-Feature geben nach dem Speichern?
- [ ] Soll es eine Versionshistorie geben (wer hat wann was geändert)?

---

**Letzte Aktualisierung:** 2026-03-22
