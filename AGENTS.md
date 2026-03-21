# Projekt Rezepte

Dieses Repo enthält den Code für eine einfach Rezepte-Verwaltung.

Sprache: Deutsch

## Architektur (Festlegungen)

**Tech Stack:** Rust + Axum + Askama + sqlx + SQLite + HTMX

**Wichtige Constraints:**
- LAN-only Web-App, KEINE Authentifizierung
- Single-User (beide Partner = gleicher User)
- Last-write-wins bei Konflikten
- DeepLink-fähige URLs mit Query-Parametern
- SQLite-Datei wird extern gemountet (Backup-fähig)

**Kategorien (hardcoded):** Mittagessen, Brot, Party, Kuchen, Snacks

**Projektstruktur:**
```
src/
├── main.rs, lib.rs, config.rs, db.rs, error.rs
├── models/recipe.rs
├── routes/recipes.rs, search.rs
├── templates/ (Askama HTML)
└── static/css/ (vanilla CSS)
```

**Key Patterns:**
- Server-Side Rendering, keine JSON-APIs für UI
- HTMX für interaktive Elemente (Progressive Enhancement)
- Form-Posts + Redirects (funktioniert ohne JS)
- SQLite mit JSON-Array für Kategorien

**Deployment:** Docker-Image, Port 8080, Volume /data für DB

## Vorgehensweise

### Organisation der Arbeit

Jede Story hat ein eigenes Verzeichnis im Ordner `docs` mit einer
Nummer und einer Kurzbezeichnung. Die Nummer startet bei 1, die nächste
Story erhält dann immer die jeweils nächste Nummer.

In dem Folder stehen dann die folgenden Dateien:

docs/xx-desc
├── story.md    (Fachliche Beschreibung, ohne Technik aber mit Akzeptanzkriterien)
├── plan.md     (Technische Beschreibung und Liste der einzelnen Schritte, mit [ ] zum Abhaken)
├── adrs.md     (optional, ADR-Beschreibung, falls spezielle Entscheidungen notwendig sind)
└── research.md (optional, Ergebnisse von Forschungen und Websuchen, falls notwendig)

### Implementierung

Jede Implementierung folgt dem TDD-Pattern:
- Test schreiben, rot sehen
- Implementierung schreiben, bis Test grün
- Refactoring, Vereinfachung und Verbesserung der Implementierung, während der Test grün bleibt

### Abnahme-Tests

Sobald ein fachliches Feature erstellt wird, muss es einen UI-Integrationstest dafür geben. Diese
Tests sind in Playwright geschrieben und liegen in einem separaten Verzeichnis.

**Wichtige Details:**
- **Testdaten:** SQL-Seed-Skripte in `tests/seeds/`
- **Isolation:** Separate SQLite-DB pro Test-Run (`TEST_DATABASE_URL`)
- **App-Start:** Playwright startet Rust-App automatisch via `webServer` Config
- **Befehl:** `npm run test:e2e`
