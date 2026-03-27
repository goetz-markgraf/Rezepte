# Implementierungsplan: Story 3 - Rezept löschen mit Sicherheitsabfrage

## Technische Schritte

### Schritt 1: Datenbank-Layer - `delete_recipe()` Funktion (TDD)

- [ ] **Test zuerst:** Unit-Test in `src/models/recipe_db.rs` schreiben:
  - `can_delete_recipe()` - Rezept erstellen, löschen, dann prüfen dass `get_recipe_by_id()` `None` zurückgibt
  - `delete_recipe_returns_error_for_nonexistent()` - Löschen einer nicht existierenden ID gibt `RowNotFound` zurück
  - `delete_recipe_is_idempotent()` - Zweimaliges Löschen: erster Aufruf OK, zweiter gibt `RowNotFound` (kein Crash)
- [ ] **Implementierung:** `delete_recipe()` Funktion in `src/models/recipe_db.rs`:
  - SQL: `DELETE FROM recipes WHERE id = ?1`
  - Prüfung auf `rows_affected == 0` -> `Err(sqlx::Error::RowNotFound)`
  - Signatur: `pub async fn delete_recipe(pool: &SqlitePool, id: i64) -> Result<(), sqlx::Error>`
- [ ] **Export:** `delete_recipe` in `src/models/mod.rs` re-exportieren
- [ ] Tests grün: `cargo test`

### Schritt 2: Bestätigungs-Template erstellen

- [ ] **Template:** `templates/recipes/confirm_delete.html` erstellen:
  - Erweitert `base.html`
  - Titel: "Rezept löschen"
  - Zeigt Rezepttitel an: "Rezept '[Titel]' wirklich löschen?"
  - Warnung: "Diese Aktion kann nicht rückgängig gemacht werden."
  - Hinweis auf Alternative: "Tipp: Statt zu löschen, kannst du eine schlechte Bewertung vergeben. Dann wird es nicht mehr vorgeschlagen, aber bleibt im System."
  - Zwei Buttons:
    - "Abbrechen" als `<a href="/recipes/{id}">` (prominenter Button, `btn-primary`)
    - "Wirklich löschen" als `<form method="POST" action="/recipes/{id}/delete"><button type="submit">` (sekundärer Button, `btn-danger`)
  - Barrierefreiheit: ARIA-Labels, Fokus-Management, Escape-Taste via JS-Enhancement

### Schritt 3: Askama Template-Struct anlegen (TDD)

- [ ] **Template-Struct:** `ConfirmDeleteTemplate` in `src/templates.rs` hinzufügen:
  ```rust
  #[derive(Template)]
  #[template(path = "recipes/confirm_delete.html")]
  pub struct ConfirmDeleteTemplate {
      pub id: i64,
      pub title: String,
  }
  ```

### Schritt 4: Index-Template für Erfolgsmeldung erweitern

- [ ] **Template:** `templates/index.html` erweitern:
  - Query-Parameter `deleted` auswerten
  - Erfolgsmeldung anzeigen: "Rezept '[Titel]' wurde gelöscht"
  - CSS-Klasse `success` verwenden (bereits in `app.css` vorhanden von Story 2)
- [ ] **Template-Struct:** `IndexTemplate` in `src/templates.rs` erweitern:
  - Feld `deleted_title: Option<String>` hinzufügen

### Schritt 5: Route-Handler implementieren (TDD)

- [ ] **Test zuerst:** Integration-Test in `tests/recipe_delete.rs` erstellen:
  - `confirm_delete_shows_recipe_title()` - GET `/recipes/:id/confirm-delete` zeigt den Rezepttitel
  - `confirm_delete_returns_404_for_nonexistent()` - GET `/recipes/99999/confirm-delete` gibt 404
  - `delete_recipe_removes_from_db()` - POST `/recipes/:id/delete` löscht und redirectet
  - `delete_recipe_returns_404_for_nonexistent()` - POST `/recipes/99999/delete` gibt 404
- [ ] **Handler 1:** `confirm_delete()` in `src/routes/recipes.rs`:
  - Route: `GET /recipes/:id/confirm-delete`
  - Rezept per ID laden (404 wenn nicht gefunden)
  - `ConfirmDeleteTemplate` rendern mit `id` und `title`
- [ ] **Handler 2:** `delete_recipe_handler()` in `src/routes/recipes.rs`:
  - Route: `POST /recipes/:id/delete`
  - Rezepttitel laden (für Erfolgsmeldung), 404 wenn nicht gefunden
  - `delete_recipe()` aus DB-Layer aufrufen
  - Redirect zu `/?deleted={url_encoded_title}`
- [ ] **Import:** `ConfirmDeleteTemplate` und `delete_recipe` in `src/routes/recipes.rs` importieren

### Schritt 6: Routen registrieren

- [ ] `src/routes/mod.rs` erweitern:
  - `.route("/recipes/:id/confirm-delete", get(recipes::confirm_delete))`
  - `.route("/recipes/:id/delete", post(recipes::delete_recipe_handler))`
- [ ] `index()` Handler in `src/routes/recipes.rs` anpassen:
  - Query-Parameter `deleted` auslesen (neues Query-Struct oder bestehendes erweitern)
  - `deleted_title` an `IndexTemplate` weitergeben

### Schritt 7: Lösch-Button in bestehende Templates einfügen

- [ ] **Detailansicht:** `templates/recipes/detail.html` erweitern:
  - "Löschen"-Link im `actions`-Bereich hinzufügen: `<a href="/recipes/{{ id }}/confirm-delete" class="btn-danger">Löschen</a>`
  - Visuell als destruktive Aktion erkennbar (rote Farbe)
- [ ] **Bearbeitungsformular:** `templates/recipes/form.html` erweitern (optional):
  - "Löschen"-Link nur im Edit-Modus anzeigen (wenn `recipe_id` vorhanden)
  - Platzierung unterhalb des Formulars oder in der Button-Leiste

### Schritt 8: CSS-Styling

- [ ] `src/static/css/app.css` erweitern:
  - `.btn-danger` Klasse: rote Hintergrundfarbe, weißer Text, Hover-Effekt
  - `.confirm-delete` Container: zentriert, gut lesbarer Warnhinweis
  - `.tip-box` Klasse: Hinweisbox für den Bewertungs-Tipp (optischer Unterschied zur Warnung)
  - Responsive Design: Buttons auf Mobilgeräten gut bedienbar

### Schritt 9: E2E-Tests (Playwright)

- [ ] `tests/e2e/recipe-delete.spec.ts` erstellen:
  - **Setup (beforeEach):** Ein Rezept "Testrezept zum Löschen" erstellen (via UI, wie in `recipe-edit.spec.ts`)
  - **Test 1: Lösch-Button ist sichtbar**
    - Auf Detailansicht navigieren
    - "Löschen"-Link ist vorhanden und sichtbar
  - **Test 2: Bestätigungsseite erscheint**
    - Auf "Löschen" klicken
    - URL enthält `/confirm-delete`
    - Rezepttitel "Testrezept zum Löschen" ist in der Abfrage sichtbar
    - Buttons "Abbrechen" und "Wirklich löschen" sind vorhanden
    - Hinweis auf Bewertungs-Alternative ist sichtbar
  - **Test 3: Abbrechen verhindert Löschung**
    - Auf "Löschen" klicken -> Bestätigungsseite
    - Auf "Abbrechen" klicken
    - Redirect zurück zur Detailseite
    - Rezept ist weiterhin sichtbar
  - **Test 4: Erfolgreiches Löschen**
    - Auf "Löschen" klicken -> Bestätigungsseite
    - Auf "Wirklich löschen" klicken
    - Redirect zur Rezeptliste (`/`)
    - Erfolgsmeldung mit Rezepttitel wird angezeigt
    - Rezept erscheint nicht mehr in der Liste
  - **Test 5: 404 bei nicht-existentem Rezept**
    - Direkt `/recipes/99999/confirm-delete` aufrufen
    - 404-Meldung wird angezeigt

### Schritt 10: Code-Qualität und DoD-Prüfung

- [ ] `cargo build` - Keine Compiler-Fehler oder Warnungen
- [ ] `cargo clippy -- -D warnings` - Keine Clippy-Warnings
- [ ] `cargo fmt --check` - Code ist formatiert
- [ ] `cargo test` - Alle Unit- und Integration-Tests grün
- [ ] `npm run test:e2e` - Alle E2E-Tests grün
- [ ] Kein ungenutzter Code, keine `unwrap()` im Produktivcode
- [ ] Funktionen haben verständliche Namen und sind unter 50 Zeilen
- [ ] Doc-Kommentare (`///`) an allen neuen öffentlichen Funktionen und Structs

---

## URL-Struktur

```
GET  /recipes/:id/confirm-delete  →  Bestätigungsseite anzeigen
POST /recipes/:id/delete          →  Rezept löschen + Redirect zu /
GET  /?deleted=Rezepttitel        →  Rezeptliste mit Erfolgsmeldung
```

---

## Betroffene Dateien

| Datei | Aktion |
|-------|--------|
| `src/models/recipe_db.rs` | `delete_recipe()` Funktion + Unit-Tests hinzufügen |
| `src/models/mod.rs` | `delete_recipe` re-exportieren |
| `src/templates.rs` | `ConfirmDeleteTemplate` Struct hinzufügen, `IndexTemplate` erweitern |
| `src/routes/recipes.rs` | `confirm_delete()` und `delete_recipe_handler()` Handler hinzufügen, `index()` erweitern |
| `src/routes/mod.rs` | Neue Routen registrieren |
| `templates/recipes/confirm_delete.html` | Neu: Bestätigungsseite |
| `templates/recipes/detail.html` | "Löschen"-Button hinzufügen |
| `templates/index.html` | Erfolgsmeldung nach Löschen anzeigen |
| `src/static/css/app.css` | `.btn-danger`, `.confirm-delete`, `.tip-box` Styles |
| `tests/recipe_delete.rs` | Neu: Rust Integration-Tests |
| `tests/e2e/recipe-delete.spec.ts` | Neu: Playwright E2E-Tests |

---

## Abhängigkeiten

- **Story 01 (Rezept erstellen)** muss abgeschlossen sein - Rezepte müssen existieren
- **Story 02 (Rezept bearbeiten)** muss abgeschlossen sein - Detailseite und Routing vorhanden
- Bestehende Structs wiederverwenden: `Recipe`, `AppError`, `get_recipe_by_id()`
- Bestehendes CSS-Pattern für `.success` wiederverwenden

---

## Test-Checkliste

- [ ] Unit-Test: `delete_recipe()` löscht erfolgreich
- [ ] Unit-Test: `delete_recipe()` gibt Fehler bei nicht-existenter ID
- [ ] Unit-Test: Idempotenz bei doppeltem Löschen
- [ ] Integration-Test: GET `/recipes/:id/confirm-delete` zeigt Bestätigungsseite
- [ ] Integration-Test: POST `/recipes/:id/delete` löscht und redirectet
- [ ] Integration-Test: 404 bei nicht-existenter ID (beide Routen)
- [ ] E2E-Test: Lösch-Button auf Detailseite sichtbar
- [ ] E2E-Test: Bestätigungsseite mit Rezepttitel und Buttons
- [ ] E2E-Test: Abbrechen kehrt zur Detailseite zurück
- [ ] E2E-Test: Erfolgreiches Löschen mit Redirect und Erfolgsmeldung
- [ ] E2E-Test: 404 bei nicht-existentem Rezept
- [ ] Manueller Test: Funktioniert ohne JavaScript (Form-Post + Redirect)
- [ ] Manueller Test: Responsive auf Mobilgeräten

---

## Design-Entscheidungen

1. **Bestätigungsseite statt Modal:** Eigene Seite unter `/recipes/:id/confirm-delete`, funktioniert ohne JavaScript. Kein HTMX-Modal, da Progressive Enhancement wichtiger ist.
2. **POST statt DELETE:** Route `POST /recipes/:id/delete` statt `DELETE /recipes/:id`, da HTML-Formulare nur GET und POST unterstützen.
3. **Erfolgsmeldung via Query-Parameter:** `/?deleted=Rezepttitel` statt Session-basierter Flash-Messages, da die App stateless ist (keine Sessions).
4. **"Abbrechen" als prominenter Button:** "Abbrechen" ist `btn-primary`, "Wirklich löschen" ist `btn-danger` (sekundär), um versehentliches Löschen zu erschweren.

---

## Offene Punkte

- Keine offenen Punkte. Alle Entscheidungen aus der Story sind getroffen (siehe Design-Entscheidungen oben).
