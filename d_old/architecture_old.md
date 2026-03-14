# Technische Architektur: Familien-Rezeptverwaltung

## Überblick
Monolithische Webanwendung mit Server-Side Rendering für den Betrieb im lokalen Netzwerk.

## Technology Stack

### Backend
- **Sprache:** Rust
- **Web Framework:** Rocket
- **Template Engine:** Tera
- **Datenbank:** SQLite
- **Query Builder:** sqlx (compile-time checked SQL)

### Frontend
- **Rendering:** Server-Side Rendering (SSR) via Tera Templates
- **Styling:** Modernes CSS (CSS Custom Properties, Nesting)
- **JavaScript:** Minimal, nur für ausgewählte interaktive Features
- **Progressive Enhancement:** Grundfunktionen arbeiten ohne JavaScript

## Architektur-Prinzipien

### Einfachheit
- Monorepo: Backend und Frontend in einem Projekt
- Keine Build-Tools für Frontend (natives CSS/JS)
- SQLite als embedded Datenbank (keine separate DB-Installation)
- Minimale Dependencies

### Deployment
- **Entwicklung:** `cargo run`
- **Produktion:** Docker Image mit eingebetteter SQLite-Datenbank

## Projekt-Struktur

```
rezepte/
├── src/
│   ├── main.rs              # Application entry point
│   ├── routes/              # HTTP route handlers
│   ├── models/              # Data models
│   ├── db/                  # Database logic
│   └── lib.rs               # Shared utilities
├── templates/               # Tera templates
│   ├── base.html.tera       # Base layout
│   ├── list.html.tera       # Recipe list view
│   ├── detail.html.tera     # Recipe detail view
│   └── form.html.tera       # Create/Edit form
├── static/                  # Static assets
│   ├── css/
│   │   └── styles.css       # Main stylesheet
│   └── js/
│       └── app.js           # Minimal client-side JS
├── migrations/              # Database migrations
├── Cargo.toml
├── Dockerfile
└── docs/
    ├── produkt.md
    └── architecture.md
```

## Datenmodell

### Recipe Table
```sql
CREATE TABLE recipes (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    title TEXT NOT NULL,
    ingredients TEXT,
    instructions TEXT,
    last_made DATE,
    created_at DATETIME DEFAULT CURRENT_TIMESTAMP,
    updated_at DATETIME DEFAULT CURRENT_TIMESTAMP
);
```

### Category Table
```sql
CREATE TABLE categories (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    name TEXT UNIQUE NOT NULL
);
```

### Recipe_Category Junction Table
```sql
CREATE TABLE recipe_categories (
    recipe_id INTEGER NOT NULL,
    category_id INTEGER NOT NULL,
    PRIMARY KEY (recipe_id, category_id),
    FOREIGN KEY (recipe_id) REFERENCES recipes(id) ON DELETE CASCADE,
    FOREIGN KEY (category_id) REFERENCES categories(id) ON DELETE CASCADE
);
```

## HTTP Endpoints

### Pages (SSR)
- `GET /` - Recipe list (mit Suche/Filter-Parametern)
- `GET /recipes/:id` - Recipe detail view
- `GET /recipes/new` - Create recipe form
- `GET /recipes/:id/edit` - Edit recipe form

### Actions
- `POST /recipes` - Create new recipe
- `POST /recipes/:id` - Update recipe
- `POST /recipes/:id/delete` - Delete recipe
- `POST /recipes/:id/mark-made` - Update "last_made" to today

### API (optional, für JS-Features)
- `GET /api/search?q=...` - Live-Suche (optional)

## Features & Implementierung

### Suche & Filter
- **Volltextsuche:** SQLite FTS5 (Full-Text Search) über title, ingredients, instructions
- **Kategorie-Filter:** Query-Parameter `?categories=Party,Kuchen`
- **"Lange nicht gemacht":** Sortierung nach `last_made ASC NULLS FIRST`

### Sortierung
- Default: `ORDER BY title ASC`
- Umschaltbar via Query-Parameter `?sort=title|last_made`

### JavaScript-Features (Progressive Enhancement)
- Live-Suche (debounced input)
- Filter-Toggle ohne Page Reload
- "Heute gemacht" Button (AJAX statt Form Submit)

## Deployment

### Entwicklung
```bash
cargo run
# App läuft auf http://localhost:8000
```

### Produktion (Docker)
```dockerfile
FROM rust:1.75 as builder
WORKDIR /app
COPY . .
RUN cargo build --release

FROM debian:bookworm-slim
COPY --from=builder /app/target/release/rezepte /usr/local/bin/rezepte
COPY --from=builder /app/templates /templates
COPY --from=builder /app/static /static
EXPOSE 8000
CMD ["rezepte"]
```

```bash
docker build -t rezepte:latest .
docker run -p 8000:8000 -v ./data:/data rezepte:latest
```

## Sicherheit & Betrieb

### LAN-Only
- Keine HTTPS erforderlich (nur LAN)
- Keine Authentifizierung
- CSRF-Protection optional (da vertrauenswürdiges Netzwerk)

### Datensicherung
- SQLite-Datei regelmäßig sichern (`data/recipes.db`)
- Volume Mount für Persistenz in Docker

## CSS Reset
Minimaler eigener Reset:
```css
*, *::before, *::after {
    box-sizing: border-box;
}

body {
    margin: 0;
    font-family: system-ui, -apple-system, sans-serif;
    line-height: 1.5;
}
```
