# Plan: Story 47 – Rezept aus Foto erstellen

## Technische Entscheidungen

- **Multipart-Upload:** axum `axum::extract::Multipart` (axum 0.7 hat native Multipart-Unterstützung)
- **HTTP-Client:** `reqwest` (async, bereits in dev-dependencies – muss in [dependencies] hinzugefügt werden)
- **Foto-Kodierung:** Base64 (standard für OpenAI-kompatible Vision-APIs)
- **Feature-Toggle:** Fehlt `VISION_API_KEY` oder `VISION_API_URL` in der Config, zeigt die App den „Aus Foto"-Einstieg nicht an
- **Kein Redirect mit vorausgefüllten Daten:** Formular-Daten können nicht per GET-Redirect übergeben werden. Stattdessen: `POST /recipes/from-photo/analyze` gibt direkt das vorausgefüllte Formular als HTML zurück (kein Redirect). Der User kann das Formular dann normal per POST auf `/recipes` absenden.

## Neue Crate-Dependencies

In `Cargo.toml` hinzufügen:
```toml
reqwest = { version = "0.11", features = ["json", "multipart"] }
base64 = "0.22"
```

> reqwest ist bereits in dev-dependencies. Es wird in dependencies verschoben/ergänzt.

## Neue Umgebungsvariablen

| Variable | Bedeutung | Default |
|---|---|---|
| `VISION_API_URL` | Basis-URL der OpenAI-kompatiblen API (z.B. `https://api.openai.com/v1`) | – |
| `VISION_API_KEY` | Bearer-Token | – |
| `VISION_MODEL` | Modellname | `gpt-4o` |

## Dateien & Änderungen

### 1. `src/config.rs`
- `Config`-Struct um `vision_api_url: Option<String>`, `vision_api_key: Option<String>`, `vision_model: String` erweitern
- `vision_enabled: bool` als abgeleitetes Feld (beide gesetzt → true)
- Tests anpassen

### 2. `src/vision.rs` (neu)
- Funktion `analyze_image(config: &Config, image_bytes: &[u8], mime_type: &str) -> Result<RecipeExtract, AppError>`
- `RecipeExtract`-Struct: `title: String`, `ingredients: String`, `instructions: String`, `category: Option<String>`
- OpenAI Chat Completions API mit `vision`-Message (base64 encoded image)
- System-Prompt definiert das JSON-Schema der Antwort
- Parst die JSON-Antwort des Modells

### 3. `src/routes/recipes.rs`
- `GET /recipes/from-photo` → `photo_upload_form` Handler
- `POST /recipes/from-photo/analyze` → `analyze_photo_handler` Handler
  - Empfängt Multipart-Upload
  - Validiert: Datei vorhanden, max. 10 MB, erlaubter MIME-Type
  - Ruft `vision::analyze_image` auf
  - Bei Erfolg: Zeigt `RecipeFormTemplate` mit vorausgefüllten Daten (aber `recipe_id = None`)
  - Bei Fehler: Zeigt `PhotoUploadTemplate` mit Fehlermeldung

### 4. `src/routes/mod.rs`
- Route `GET /recipes/from-photo` registrieren
- Route `POST /recipes/from-photo/analyze` registrieren

### 5. `src/templates.rs`
- `PhotoUploadTemplate` hinzufügen (path: `recipes/from_photo.html`)
  - Felder: `error: Option<String>`, `vision_enabled: bool`

### 6. `templates/recipes/from_photo.html` (neu)
- Upload-Formular mit `enctype="multipart/form-data"`
- `<input type="file" accept="image/*" capture="camera">` für Kamera-Support
- Fehlermeldung wenn `error` gesetzt
- Abbrechen-Link → `/`

### 7. `templates/index.html`
- Wenn `vision_enabled`, Button „Aus Foto" neben „Neues Rezept" in Top-Bar

### 8. `src/lib.rs`
- App-State um Vision-Config erweitern (oder Config aus Arc wrappen)

### 9. `src/main.rs`
- Config an Router übergeben (aktuell wird nur `pool` übergeben – Config muss ebenfalls in State)

## Implementierungsschritte (TDD)

- [ ] **Schritt 1: Config erweitern**
  - Tests für neue Vision-Config-Felder schreiben (rot)
  - `Config` um Vision-Felder erweitern
  - Tests grün

- [ ] **Schritt 2: `vision.rs` – Unit-Tests mit Mock**
  - Test: `analyze_image` gibt bei gültigem JSON `RecipeExtract` zurück
  - Test: `analyze_image` gibt bei API-Fehler `AppError` zurück
  - Implementierung mit `reqwest`
  - Tests grün

- [ ] **Schritt 3: `PhotoUploadTemplate` und Template-Datei**
  - Template-Struct in `templates.rs`
  - `from_photo.html` erstellen
  - `cargo build` grün

- [ ] **Schritt 4: Route `GET /recipes/from-photo`**
  - Integration-Test: GET gibt 200 mit Upload-Formular zurück
  - Handler `photo_upload_form` implementieren
  - Test grün

- [ ] **Schritt 5: Route `POST /recipes/from-photo/analyze`**
  - Integration-Test: POST ohne Datei → 400 mit Fehler
  - Integration-Test: POST mit Datei und gemockter Vision-API → 200 mit vorausgefülltem Formular
  - Handler `analyze_photo_handler` implementieren
  - Tests grün

- [ ] **Schritt 6: Index-Seite anpassen**
  - `IndexTemplate` um `vision_enabled: bool` erweitern
  - `index.html` anpassen (bedingter Button)
  - `cargo build` grün

- [ ] **Schritt 7: State-Erweiterung**
  - Config im App-State verfügbar machen
  - Alle Handler die Config nutzen anpassen

- [ ] **Schritt 8: E2E-Test (Playwright)**
  - Test: Upload-Seite erreichbar wenn Vision konfiguriert
  - Test: Ohne Vision-Konfiguration kein Button sichtbar
  - (Vision-API selbst wird nicht von E2E getestet – zu teuer/fragil)

- [ ] **Schritt 9: Refactoring & Qualitätschecks**
  - `cargo clippy` ohne Warnungen
  - `cargo test` grün
  - `npm run test:e2e` grün

## Offene Fragen

- Soll das Foto nach der Analyse sofort verworfen werden? **Ja**, kein Speichern.
- Soll die Kategorie aus dem Foto immer valide sein? **Ja**, ungültige Kategorie → Kategorie leer lassen.
- Soll die App einen Ladezustand zeigen während die Analyse läuft? **Nein** (JS-frei, normale Form-Submit-Wartezeit).
