# Implementierungsplan: Story 2 - Rezept bearbeiten

## Technische Schritte

### Schritt 1: Datenbank-Layer erweitern
- [ ] `update_recipe()` Funktion in `src/models/recipe_db.rs` implementieren
  - SQL UPDATE mit allen Feldern
  - Automatisches Setzen von `updated_at = CURRENT_TIMESTAMP`
  - Prüfung ob Rezept existiert (Fehler wenn nicht gefunden)
- [ ] Unit-Test: Update mit allen Feldern
- [ ] Unit-Test: Update nicht-existentes Rezept (Fehler)
- [ ] Unit-Test: updated_at wird aktualisiert

### Schritt 2: Modell erweitern
- [ ] `UpdateRecipe` Struct in `src/models/recipe.rs` erstellen
  - Gleiche Felder wie `CreateRecipe`
  - Validation-Logik wiederverwenden
- [ ] `Recipe::validate_update()` Methode implementieren
  - Gleiche Validierung wie bei Create

### Schritt 3: Routes und Handler
- [ ] `src/routes/recipes.rs` erweitern:
  - GET `/recipes/:id/edit` - Bearbeiten-Formular anzeigen
  - POST `/recipes/:id` - Rezept aktualisieren
- [ ] Handler implementieren:
  - `edit_recipe_form()` - Formular mit bestehenden Daten laden
  - `update_recipe_handler()` - Validieren + speichern
  - Fehlerbehandlung: 404 wenn Rezept nicht existiert

### Schritt 4: Templates anpassen
- [ ] `templates/recipes/form.html` erweitern:
  - Edit-Modus erkennen (Rezept-ID vorhanden)
  - Alle Felder mit `value="{{ recipe.title }}"` vorausfüllen
  - Dynamischer Titel: "Rezept bearbeiten" vs "Neues Rezept"
  - Action-URL anpassen: POST /recipes/:id vs POST /recipes
- [ ] `templates/recipes/detail.html` erweitern:
  - Button "Bearbeiten" hinzufügen
  - Link zu `/recipes/:id/edit`
  - Temporär rudimentäre Detailseite als Platzhalter

### Schritt 5: Detailseite (rudimentär)
- [ ] `templates/recipes/detail.html` verbessern:
  - Alle Rezept-Felder anzeigen
  - Button "Bearbeiten" prominent platzieren
  - Button "Zurück zur Übersicht"
  - Einfache Formatierung für Zutaten/Anleitung
- [ ] GET `/recipes/:id` Handler verbessern:
  - Rezept aus DB laden
  - 404 wenn nicht gefunden
  - Template mit Daten rendern

### Schritt 6: Validierung
- [ ] Serverseitige Validierung im Update-Handler:
  - Titel nicht leer
  - Mindestens eine Kategorie
  - Gleiche Fehlermeldungen wie bei Erstellung
- [ ] Fehleranzeige im Formular:
  - Eingegebene Werte beibehalten (nicht zurücksetzen)
  - Fehler oberhalb des Formulars anzeigen

### Schritt 7: Redirects und Feedback
- [ ] Nach erfolgreichem Update:
  - Redirect zu GET `/recipes/:id` (Detailseite)
  - Erfolgsmeldung anzeigen (Flash-Message oder Query-Param)
- [ ] Bei "Abbrechen":
  - Redirect zu `/recipes/:id` (ohne Speichern)
- [ ] Bei Validierungsfehler:
  - Formular neu rendern mit Fehlern
  - Eingegebene Werte beibehalten

### Schritt 8: E2E-Tests (Playwright)
- [ ] `tests/e2e/recipe-edit.spec.ts` erstellen:
  - Test 1: Erfolgreiche Bearbeitung (Titel ändern)
  - Test 2: Abbrechen ohne Speichern
  - Test 3: Validierung - Leerer Titel
  - Test 4: Nicht-existentes Rezept (404)
  - Test 5: updated_at Timestamp prüfen
- [ ] Seed-Daten für Tests:
  - Rezept "Testrezept" in Seed-SQL einfügen

### Schritt 9: Integration
- [ ] `src/routes/mod.rs` - Routes registrieren
- [ ] `src/main.rs` - Handler einbinden
- [ ] Navigation prüfen: Übersicht -> Detail -> Bearbeiten

### Schritt 10: Styling
- [ ] `static/css/style.css` erweitern:
  - Button "Bearbeiten" Styling
  - Formular-Edit-Modus visuell unterscheiden (optional)
  - Erfolgsmeldung Styling (grün)

## Abhängigkeiten

- **Story 1 muss abgeschlossen sein** (Datenmodell, Formular-Struktur)
- SQLx Query-Builder wird verwendet
- Askama Templates (wiederverwenden aus Story 1)

## URL-Struktur

```
GET  /recipes/:id         → Detail-Ansicht (mit Bearbeiten-Button)
GET  /recipes/:id/edit    → Bearbeiten-Formular (vorausgefüllt)
POST /recipes/:id         → Rezept aktualisieren
```

## Test-Checkliste

- [ ] Unit-Test: DB-Update Operation
- [ ] Unit-Test: Validierung mit fehlenden Pflichtfeldern
- [ ] E2E-Test: Kompletter Edit-Flow
- [ ] E2E-Test: Abbrechen-Funktionalität
- [ ] Manueller Test: Formular ist vorausgefüllt
- [ ] Manueller Test: updated_at wird aktualisiert
- [ ] Manueller Test: 404 bei nicht-existentem Rezept

## Schätzung

**Geschätzter Aufwand:** 4-6 Stunden
- Datenbank + Update-Logik: 1h
- Routes + Handler: 1.5h
- Templates + HTML: 1.5h
- Tests: 1.5h
- Styling + Integration: 0.5h

## Offene Punkte

- Flash-Messages für Erfolgsmeldungen (Session-basiert oder Query-Param)
- Bestätigungsdialog bei Abbrechen mit ungespeicherten Änderungen (optional, kann später)
