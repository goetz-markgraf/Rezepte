# Implementierungsplan: Story 1 - Rezept erstellen

## Technische Schritte

### Schritt 1: Datenbank-Setup
- [ ] Migration erstellen für `recipes` Tabelle
  - id: INTEGER PRIMARY KEY AUTOINCREMENT
  - title: TEXT NOT NULL
  - categories: TEXT NOT NULL (JSON-Array als String)
  - ingredients: TEXT
  - instructions: TEXT
  - created_at: TIMESTAMP DEFAULT CURRENT_TIMESTAMP
  - updated_at: TIMESTAMP DEFAULT CURRENT_TIMESTAMP
- [ ] Migration ausführen mit sqlx-cli

### Schritt 2: Modell definieren
- [ ] `src/models/recipe.rs` erstellen
  - Struct `Recipe` mit sqlx::FromRow
  - Struct `CreateRecipe` für Form-Daten
  - Validation-Logik für Pflichtfelder
  - Hilfsmethoden für Kategorien (de/serialisieren)
- [ ] `src/models/mod.rs` erstellen und exportieren

### Schritt 3: Datenbank-Layer
- [ ] In `src/db.rs` oder `src/models/recipe.rs`:
  - `create_recipe()` Funktion implementieren
  - `get_recipe_by_id()` Funktion implementieren (für Detailansicht)
  - Unit-Tests für DB-Operationen

### Schritt 4: Routes und Handler
- [ ] `src/routes/recipes.rs` erstellen
  - GET `/recipes/new` - Formular anzeigen
  - POST `/recipes` - Rezept erstellen
  - GET `/recipes/:id` - Detailansicht (rudimentär)
- [ ] Handler implementieren:
  - `new_recipe_form()` - Render leeres Formular
  - `create_recipe_handler()` - Form-Daten validieren + speichern
  - `show_recipe()` - Einzelnes Rezept anzeigen

### Schritt 5: Templates (Askama)
- [ ] `src/templates/recipes.rs` erstellen
  - `RecipeFormTemplate` - Formular-Template
  - `RecipeDetailTemplate` - Detail-Ansicht
  - `RecipeCreatedTemplate` - Erfolgsseite
- [ ] HTML-Templates in `templates/`:
  - `recipe_form.html` - Formular mit Feldern
  - `recipe_detail.html` - Rudimentäre Detailansicht
  - `base.html` - Layout-Template

### Schritt 6: Validierung
- [ ] Serverseitige Validierung in Handler:
  - Titel nicht leer, max 100 Zeichen
  - Mindestens eine Kategorie ausgewählt
  - Zutaten max 2000 Zeichen
  - Anleitung max 5000 Zeichen
- [ ] Fehlermeldungen im Template anzeigen

### Schritt 7: Integration
- [ ] `src/main.rs` anpassen
  - Routes registrieren
  - Template-Engine einbinden
  - Statische Dateien (CSS) servieren
- [ ] `src/lib.rs` aktualisieren mit Modul-Exports

### Schritt 8: Grundlegende Startseite
- [ ] GET `/` Route erstellen (Übersicht)
  - Rudimentäre Seite mit H1 "Rezepte Übersicht"
  - Button "Neues Rezept" -> Link zu `/recipes/new`
- [ ] Template: `templates/index.html`

### Schritt 9: E2E-Tests (Playwright)
- [ ] `tests/e2e/recipe-create.spec.ts` erstellen
  - Test 1: Erfolgreiche Erstellung
  - Test 2: Validierung - Fehlende Pflichtfelder
  - Test 3: Eingabe aller Felder
- [ ] Seed-Daten für Tests vorbereiten

### Schritt 10: Styling
- [ ] `static/css/style.css` erstellen
  - Grundlegende Formular-Styling
  - Responsive Layout
  - Fehler-Status Styling

## Abhängigkeiten

```toml
# Cargo.toml - bereits vorhanden oder hinzufügen:
[dependencies]
axum = "0.7"
tokio = { version = "1", features = ["full"] }
serde = { version = "1.0", features = ["derive"] }
serde_json = "1.0"
sqlx = { version = "0.7", features = ["sqlite", "runtime-tokio"] }
askama = "0.12"
tower = "0.4"
tower-http = { version = "0.5", features = ["fs"] }
chrono = "0.4"
```

## Offene technische Entscheidungen

1. **Kategorien-Speicherung:** JSON-Array als TEXT-Feld in SQLite
   - Speicherung: `["Mittagessen", "Party"]`
   - Parsing mit serde_json

2. **Validierungs-Fehler-Anzeige:**
   - Serverseitige Validierung
   - Fehler im Template als Liste oberhalb des Formulars
   - Rote Markierung der fehlerhaften Felder

3. **Redirect nach Speichern:**
   - POST /recipes -> Redirect zu GET /recipes/:id
   - Flash-Message für Erfolgsmeldung (optional: Query-Param ?created=true)

## Test-Checkliste

- [ ] Unit-Test: DB-Operation create_recipe
- [ ] Unit-Test: Validierung mit fehlenden Pflichtfeldern
- [ ] E2E-Test: Kompletter Flow mit Playwright
- [ ] Manueller Test: Formular-Validierung im Browser
- [ ] Manueller Test: Responsive Design (Mobile/Desktop)

## Schätzung

**Geschätzter Aufwand:** 6-8 Stunden
- Datenbank + Modelle: 1h
- Routes + Handler: 2h
- Templates + HTML: 2h
- Tests: 2h
- Styling + Polish: 1-2h
