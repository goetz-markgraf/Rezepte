# Architecture

Rezepte ist eine Server-Side-Rendered Webanwendung zur Verwaltung von Familienrezepten im lokalen Netzwerk. Kein Login, kein JSON-API-Layer für die UI — alle Interaktionen gehen über HTMX und HTML-Forms.

## Tech Stack

Rust + Axum bilden das Backend. [[src/routes/mod.rs#create_router]] registriert alle Routen und nimmt den SQLite-Pool als State. Askama rendert HTML-Templates serverseitig. HTMX ermöglicht partielle Seitenaktualisierungen ohne vollständige SPA-Architektur.

| Schicht   | Technologie                      |
|-----------|----------------------------------|
| Backend   | Rust + Axum                      |
| Templates | Askama (SSR)                     |
| Datenbank | SQLite + sqlx                    |
| Frontend  | HTMX + Vanilla CSS               |
| Tests     | `cargo test` + Playwright (E2E)  |
| Deploy    | Docker                           |

## Architekturprinzipien

Server-Side Rendering ohne JSON-API für die UI. Progressive Enhancement — die App funktioniert ohne JavaScript. URLs mit Query-Parametern sind DeepLink-fähig, speicher- und teilbar. SQLite-Datei wird extern gemountet für einfaches Backup.

## Projektstruktur

Verzeichnisaufbau des Projekts mit Zuordnung von Verantwortlichkeiten.

```
src/
├── main.rs, lib.rs         # Einstiegspunkt, öffentliches API der Lib
├── config.rs               # Konfiguration (DATABASE_URL, Port)
├── db.rs                   # Pool-Setup und Migrationen
├── error.rs                # AppError (Axum IntoResponse)
├── markdown.rs             # Markdown-Rendering + XSS-Sanitisierung
├── emoji.rs                # Emoji-Matching für Rezepte
├── models/                 # Rust-Datenstrukturen + DB-Queries
├── routes/                 # Axum-Handler
├── templates.rs            # Askama-Template-Bindungen
└── static/css/             # Vanilla CSS

templates/                  # Askama HTML-Templates
tests/                      # Integrationstests (Rust) + E2E (Playwright)
migrations/                 # SQLite-Migrationsskripte
```

## Datenbankmigrationen

Migrationen liegen als SQL-Dateien unter `migrations/` und werden beim Start automatisch via `sqlx::migrate!` angewendet. Siehe [[src/db.rs#create_pool]].

## State-Weitergabe

Der `SqlitePool` wird als `Arc<SqlitePool>` über Axum-State an alle Handler weitergegeben. Kein globaler State, keine Singletons.

## Test-API-Endpunkte

`/api/test/clear-recipes` und `/api/test/seed-recipe` sind dedizierte Endpunkte für E2E-Tests — sie erlauben das Zurücksetzen und Befüllen der Datenbank ohne Datenbankzugriff aus dem Testprozess heraus.
