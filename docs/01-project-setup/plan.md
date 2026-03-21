# Technischer Plan: Story 01 - Projekt-Setup

## Überblick

Dieser Plan beschreibt die technische Umsetzung der ersten Story: Initialisierung eines Rust-Webprojekts mit Axum, Askama, sqlx und SQLite.

## Schritte

### Schritt 1: Rust-Projekt initialisieren

**Beschreibung:**
Neues Cargo-Projekt erstellen und konfigurieren.

**Dateien:**
- `Cargo.toml` - Dependencies und Metadata
- `.gitignore` - Rust-spezifische Ausnahmen

**Dependencies (Cargo.toml):**
```toml
[dependencies]
tokio = { version = "1", features = ["full"] }
axum = "0.7"
askama = "0.12"
sqlx = { version = "0.7", features = ["runtime-tokio", "sqlite", "time"] }
tower = "0.4"
tower-http = { version = "0.5", features = ["fs", "trace"] }
time = "0.3"
serde = { version = "1.0", features = ["derive"] }
serde_json = "1.0"
anyhow = "1.0"
tracing = "0.1"
tracing-subscriber = "0.3"
```

**TDD Ansatz:**
Kein Test nötig, nur Konfiguration.

---

### Schritt 2: Verzeichnisstruktur erstellen

**Beschreibung:**
Alle notwendigen Verzeichnisse für das Projekt anlegen.

**Verzeichnisse:**
```
src/
├── main.rs
├── lib.rs
├── config.rs
├── db.rs
├── error.rs
├── models/
│   └── mod.rs
├── routes/
│   └── mod.rs
├── templates/
│   └── base.html
└── static/
    └── css/
        └── app.css
migrations/
└── .gitkeep
data/
└── .gitkeep
```

**Befehle:**
```bash
mkdir -p src/models src/routes src/templates src/static/css migrations data
```

**TDD Ansatz:**
Kein Test nötig, nur Verzeichnisse erstellen.

---

### Schritt 3: Datenbank-Migration erstellen

**Beschreibung:**
Erste Migration für die recipes-Tabelle erstellen.

**Datei:** `migrations/001_initial.sql`

**SQL:**
```sql
CREATE TABLE recipes (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    title TEXT NOT NULL,
    categories TEXT,
    ingredients TEXT,
    instructions TEXT,
    rating INTEGER CHECK (rating BETWEEN 1 AND 5),
    planned_date DATE,
    created_at DATETIME DEFAULT CURRENT_TIMESTAMP,
    updated_at DATETIME DEFAULT CURRENT_TIMESTAMP
);

CREATE INDEX idx_recipes_title ON recipes(title);
CREATE INDEX idx_recipes_planned_date ON recipes(planned_date);
```

**TDD Ansatz:**
Test: Migration kann angewendet werden auf SQLite-Datenbank.

---

### Schritt 4: Konfigurationsmodul erstellen

**Datei:** `src/config.rs`

**Inhalt:**
- Umgebungsvariablen laden (DATABASE_URL, PORT, RUST_LOG)
- Default-Werte definieren
- Config-Struktur mit Validierung

**TDD Ansatz:**
Test: Config wird korrekt aus Umgebungsvariablen geladen.
Test: Default-Werte funktionieren wenn keine ENV vars gesetzt.

---

### Schritt 5: Datenbank-Modul erstellen

**Datei:** `src/db.rs`

**Inhalt:**
- Connection Pool mit sqlx erstellen
- Migration ausführen
- DB-Handle für Routes bereitstellen

**TDD Ansatz:**
Test: Datenbankverbindung kann hergestellt werden.
Test: Migration läuft erfolgreich durch.

---

### Schritt 6: Error-Handling Modul erstellen

**Datei:** `src/error.rs`

**Inhalt:**
- Eigener AppError-Typ
- Konvertierung von sqlx::Error zu HTTP-Response
- Implementierung von axum::response::IntoResponse

**TDD Ansatz:**
Test: Fehler werden korrekt in HTTP-Responses konvertiert.

---

### Schritt 7: Hauptanwendung (main.rs)

**Datei:** `src/main.rs`

**Inhalt:**
- Tokio-Runtime starten
- Config laden
- DB-Pool initialisieren
- Axum-Router erstellen
- Server auf Port 8080 starten
- Graceful shutdown handhaben

**TDD Ansatz:**
Integrationstest: Server startet und antwortet auf /health.

---

### Schritt 8: Health-Check Endpunkt

**Datei:** `src/routes/mod.rs` und Handler

**Inhalt:**
- GET /health - gibt einfaches "OK" zurück
- Router-Struktur für zukünftige Erweiterungen

**TDD Ansatz:**
Test: GET /health gibt Status 200 und Text "OK" zurück.

---

### Schritt 9: Basis-Template erstellen

**Datei:** `src/templates/base.html`

**Inhalt:**
- HTML5 Grundstruktur
- Platzhalter für Titel und Content
- CSS-Link

**TDD Ansatz:**
Kein Test nötig für statisches HTML.

---

### Schritt 10: Basis-CSS erstellen

**Datei:** `src/static/css/app.css`

**Inhalt:**
- Reset/Normalize
- Basis-Typografie
- Responsive Breakpoints (optional für später)

**TDD Ansatz:**
Kein Test nötig für statisches CSS.

---

### Schritt 11: lib.rs für Tests

**Datei:** `src/lib.rs`

**Inhalt:**
- Alle Module exportieren
- Test-Utilities (optional)
- Integration-Test-Setup

**TDD Ansatz:**
Tests können aus tests/-Verzeichnis laufen.

---

### Schritt 12: Integrationstests

**Datei:** `tests/health_check.rs`

**Inhalt:**
- Test-Server starten (random Port)
- HTTP-Request an /health
- Assertion: Status 200, Body "OK"

**TDD Ansatz:**
Dies ist der erste Integrationstest - er muss laufen.

---

### Schritt 13: Build & Verify

**Befehle:**
```bash
cargo build
cargo test
cargo run
```

**TDD Ansatz:**
Alle Tests müssen grün sein.

---

## Abhängigkeiten

- Rust-Toolchain installiert (cargo, rustc)
- SQLite installiert (für sqlx-cli, optional)

## Risiken & Mitigation

| Risiko | Mitigation |
|--------|------------|
| sqlx compile-time checking braucht DB | `sqlx prepare` oder SQLite-Datei vorab erstellen |
| Dependency-Konflikte | Versionskompatibilität vorher prüfen |
| Port 8080 belegt | Konfigurierbar machen, Default auf 8080 |

## Erfolgskriterien

- [ ] `cargo build` läuft ohne Fehler
- [ ] `cargo test` läuft erfolgreich (mindestens 1 Test)
- [ ] `cargo run` startet Server auf Port 8080
- [ ] `curl http://localhost:8080/health` gibt "OK" zurück
- [ ] Datenbank-Datei wird erstellt (./data/recipes.db)

## Nächste Story

Nach diesem Setup kann mit Story 02 begonnen werden: 
"Rezept-Liste anzeigen" - erste CRUD-Operation mit Templates.
