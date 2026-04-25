# Architecture Document - Rezepte

**Status:** In Progress  
**Letzte Aktualisierung:** 2026-03-21

---

## 1. Tech Stack Overview

### Backend
- **Sprache:** Rust
- **Web-Framework:** Axum (von Tokio-Entwicklern, modern, gute AI-Unterstützung)
- **Template-Engine:** Askama (Jinja2-ähnlich, compile-time type-safe)
- **Datenbank:** SQLite (eingebettet, einfach zu backupen)
- **ORM/Query Builder:** sqlx (compile-time checked queries)
- **Async Runtime:** Tokio

### Frontend
- **Ansatz:** Server-Side Rendering (SSR) mit minimalen Client-JS
- **Interaktivität:** HTMX (Hypermedia-Driven Architecture)
- **Styling:** CSS (kein Framework, einfache, responsive Gestaltung)
- **Icons:** Lucide oder Heroicons (SVG)

### Deployment
- **Container:** Docker
- **Target:** Raspberry Pi / NAS (ARM64)
- **Reverse Proxy:** Optional traefik oder nginx (außerhalb Docker)

### Entwicklung
- **Build Tool:** cargo
- **Test Framework:** built-in + tokio-test
- **Linting:** clippy, rustfmt

---

## 2. Architektur-Prinzipien

### Hypermedia-Driven Architecture
- Kein API-Layer, keine JSON-Endpunkte für UI
- Server rendert HTML, HTMX tauscht Fragmente aus
- State-Management serverseitig in SQLite
- Formulare über normale POST-Requests

### Einfachheit vor Komplexität
- Keine Authentifizierung (LAN-only)
- Keine Sessions (stateless requests)
- Single-User-Perspektive (beide Partner = gleicher User)
- Last-write-wins bei Konflikten

### Performance
- SQLite mit WAL-Modus (Write-Ahead Logging)
- Connection Pooling (sqlx)
- Minimaler JavaScript-Overhead
- CSS statt schwerer Frameworks

---

## 2. Projektstruktur

*(Die Architektur-Prinzipien stehen oben unter Punkt 1)*

```
rezepte/
├── Cargo.toml              # Dependencies
├── Dockerfile              # Multi-stage build für ARM64
├── src/
│   ├── main.rs            # Application entry, server setup
│   ├── lib.rs             # Module exports
│   ├── config.rs          # Konfiguration (DB-Pfad, Port)
│   ├── db.rs              # Database connection pool
│   ├── error.rs           # Error types und handling
│   ├── models/            # Datenstrukturen
│   │   ├── mod.rs
│   │   └── recipe.rs      # Recipe struct und DB-Methoden
│   ├── routes/            # HTTP Handlers
│   │   ├── mod.rs
│   │   ├── recipes.rs     # CRUD für Rezepte
│   │   ├── search.rs      # Suche und Filter
│   │   └── api.rs         # HTMX Endpoints (Partial-Views)
│   ├── templates/         # Askama Templates
│   │   ├── base.html      # Layout-Template
│   │   ├── recipes/
│   │   │   ├── list.html
│   │   │   ├── detail.html
│   │   │   ├── form.html
│   │   │   └── _recipe_row.html  # HTMX Partial
│   │   └── components/
│   │       ├── filters.html
│   │       └── search.html
│   └── static/            # CSS, JS, Assets
│       └── css/
│           └── app.css
├── migrations/            # sqlx migrations
│   └── 001_initial.sql
└── data/                  # SQLite DB (gitignored)
    └── .gitkeep
```

### Module-Struktur
- **main.rs:** Tokio Runtime, Server-Start, graceful shutdown
- **lib.rs:** Module-Exports für Tests
- **config.rs:** Umgebungsvariablen, Defaults (Port 8080)
- **db.rs:** Sqlx connection pool setup
- **models/recipe.rs:** Recipe struct, DB-Queries, Validierung
- **routes/:** Axum Router mit verschachtelten Sub-Routern

---

## 3. Datenmodell

### Datenbank-Schema (SQLite)

```sql
-- Migration: 001_initial.sql
CREATE TABLE recipes (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    title TEXT NOT NULL,
    categories TEXT,           -- JSON-Array: '["Mittagessen","Party"]'
    ingredients TEXT,          -- Markdown
    instructions TEXT,         -- Markdown
    rating INTEGER CHECK (rating BETWEEN 1 AND 5),
    planned_date DATE,         -- Zukunft oder Vergangenheit
    created_at DATETIME DEFAULT CURRENT_TIMESTAMP,
    updated_at DATETIME DEFAULT CURRENT_TIMESTAMP
);

-- Index für schnelle Suche
CREATE INDEX idx_recipes_title ON recipes(title);
CREATE INDEX idx_recipes_planned_date ON recipes(planned_date);
```

### Rust Models

```rust
// src/models/recipe.rs
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use time::Date;

pub const VALID_CATEGORIES: &[&str] = &[
    "Mittagessen",
    "Brot",
    "Party", 
    "Kuchen",
    "Snacks"
];

#[derive(Debug, FromRow, Serialize, Deserialize)]
pub struct Recipe {
    pub id: i64,
    pub title: String,
    pub categories: Vec<String>,  // JSON-Array parsed
    pub ingredients: Option<String>,
    pub instructions: Option<String>,
    pub rating: Option<i32>,
    pub planned_date: Option<Date>,
    pub created_at: String,
    pub updated_at: String,
}

#[derive(Debug, Deserialize)]
pub struct CreateRecipe {
    pub title: String,
    pub categories: Vec<String>,
    pub ingredients: Option<String>,
    pub instructions: Option<String>,
    pub rating: Option<i32>,
    pub planned_date: Option<Date>,
}
```

### Validierung
- **Titel:** Required, max 255 Zeichen
- **Kategorien:** Müssen aus VALID_CATEGORIES stammen
- **Rating:** 1-5 Sterne oder NULL
- **Datum:** Beliebiges Datum (Vergangenheit oder Zukunft)

---

## 4. Routing & URL-Struktur

### DeepLink-fähige URLs

Alle Funktionen sind über eindeutige URLs erreichbar (Progressive Enhancement mit HTMX):

```
# Dashboard / Startseite
GET  /                     → Liste aller Rezepte (mit Filter)

# Rezept-Verwaltung (CRUD)
GET  /recipes             → Liste mit optionalen Query-Parametern
GET  /recipes?q=pasta     → Volltextsuche
GET  /recipes?category=Mittagessen&sort=date → Filter + Sortierung
GET  /recipes?filter=laenger-nicht-gemacht → "Länger nicht gemacht"
GET  /recipes/new         → Formular: Neues Rezept
POST /recipes             → Rezept erstellen
GET  /recipes/{id}        → Detail-Ansicht
GET  /recipes/{id}/edit   → Formular: Rezept bearbeiten
POST /recipes/{id}        → Rezept aktualisieren
POST /recipes/{id}/delete → Rezept löschen (mit Confirmation)

# Spezielle Ansichten
GET  /recipes?planned=next-7-days → Geplante Rezeite (Wochenplanung)
```

### Query-Parameter für Filter

| Parameter | Wert | Beschreibung |
|-----------|------|--------------|
| `q` | String | Volltextsuche (Titel, Zutaten, Anleitung) |
| `category` | Enum | Einzelne Kategorie filtern |
| `sort` | `title`, `date` | Sortierfeld |
| `order` | `asc`, `desc` | Sortierrichtung |
| `filter` | `laenger-nicht-gemacht` | "Länger nicht gemacht" |
| `planned` | `next-7-days` | Geplante Rezepte |

### HTMX Integration (optional)

HTMX wird für interaktive Verbesserungen eingesetzt, ohne die DeepLink-Struktur zu brechen:

- **Live-Suche:** `hx-get="/recipes" hx-trigger="keyup changed"`
- **Filter anwenden:** `hx-get="/recipes"` mit Query-Parametern
- **Löschen bestätigen:** `hx-confirm="Rezept wirklich löschen?"`

Die App funktioniert komplett ohne JavaScript (Form-Posts + Redirects).

---

## 5. Deployment & Docker

### Docker-Setup

**Multi-Stage Build für ARM64 (Raspberry Pi/NAS):**

```dockerfile
# Dockerfile
FROM rust:1.75-slim-bookworm as builder
WORKDIR /app
COPY . .
RUN cargo build --release

FROM debian:bookworm-slim
RUN apt-get update && apt-get install -y libsqlite3-0 && rm -rf /var/lib/apt/lists/*
WORKDIR /app
COPY --from=builder /app/target/release/rezepte /app/rezepte
COPY --from=builder /app/migrations /app/migrations
COPY --from=builder /app/src/static /app/src/static
COPY --from=builder /app/src/templates /app/src/templates

ENV DATABASE_URL=/data/recipes.db
ENV RUST_LOG=info
EXPOSE 8080

VOLUME ["/data"]

CMD ["./rezepte"]
```

### Deployment-Kommandos

**Build & Run:**
```bash
# Image bauen (für ARM64)
docker buildx build --platform linux/arm64 -t rezepte:latest .

# Container starten mit Volume
docker run -d \
  --name rezepte \
  -p 8080:8080 \
  -v $(pwd)/data:/data \
  --restart unless-stopped \
  rezepte:latest
```

**Docker Compose (empfohlen):**
```yaml
# docker-compose.yml
version: '3.8'
services:
  rezepte:
    image: rezepte:latest
    container_name: rezepte
    ports:
      - "8080:8080"
    volumes:
      - ./data:/data
    environment:
      - DATABASE_URL=/data/recipes.db
      - RUST_LOG=info
    restart: unless-stopped
```

### Datenbank-Backup

**Einfaches Backup:**
```bash
# Backup erstellen
cp data/recipes.db backups/recipes-$(date +%Y%m%d).db

# Oder mit SQLite (konsistentes Backup während Betrieb)
sqlite3 data/recipes.db ".backup backups/recipes-$(date +%Y%m%d).db"
```

**Automatisches Backup (Cron):**
```bash
# Tägliches Backup um 3 Uhr
0 3 * * * sqlite3 /path/to/data/recipes.db ".backup /path/to/backups/recipes-$(date +\%Y\%m\%d).db"
```

### Konfiguration

**Umgebungsvariablen:**

| Variable | Default | Beschreibung |
|----------|---------|--------------|
| `DATABASE_URL` | `/data/recipes.db` | Pfad zur SQLite-Datei |
| `PORT` | `8080` | Server-Port |
| `RUST_LOG` | `info` | Logging-Level |
| `STATIC_DIR` | `src/static` | Pfad zu CSS/Assets |
| `TEMPLATE_DIR` | `src/templates` | Pfad zu Askama-Templates |

### Netzwerk-Zugriff

**LAN-Zugriff:**
- Container läuft auf Port 8080
- NAS/Docker-Host: `http://localhost:8080`
- Andere Geräte im LAN: `http://<NAS-IP>:8080`

**Optional: Reverse Proxy (traefik/nginx):**
- HTTPS-Terminierung (wenn gewünscht)
- Custom Domain (z.B. `rezepte.local`)

---

## 6. Testing Strategy

### Unit Tests
- Rust built-in Test-Framework mit tokio-test
- Tests für Models, Services, Utilities
- Integration in `cargo test`

### UI Integration Tests
- **Framework:** Playwright (Node.js)
- **Testdaten:** SQL Seed-Skripte in `tests/seeds/`
- **Isolation:** Separate SQLite-Datenbank pro Test-Run via `TEST_DATABASE_URL`
- **App-Start:** Automatisch via Playwright `webServer` Config
- **Befehl:** `npm run test:e2e`

**Struktur:**
```
tests/
├── seeds/              # SQL-Dateien mit Testdaten
│   └── 001_test_recipes.sql
├── e2e/                # Playwright Test-Dateien
│   └── recipes.spec.ts
└── playwright.config.ts
```

**Prozess:**
1. Playwright startet Rust-App mit `TEST_DATABASE_URL`
2. Vor jedem Test-File: DB wird geleert + Seeds geladen
3. Tests werden gegen laufende App ausgeführt
4. Bei Fehlern: Traces und Screenshots verfügbar

---

Das Architecture-Dokument ist nun vollständig!

