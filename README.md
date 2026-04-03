# Rezepte

Eine einfache Webanwendung zur Verwaltung von Familienrezepten im lokalen Netzwerk.

## Überblick

**Rezepte** konsolidiert verstreute Rezeptquellen (Ordner, Bücher, Zettel) in einem zentralen, schnell durchsuchbaren Repository. Der Fokus liegt auf schneller Inspiration bei der Wochenplanung — durch Filter wie "Länger nicht gemacht", Kategorien und Volltextsuche. Beide Partner können jederzeit auf alle Rezepte zugreifen, sie bearbeiten und das letzte Zubereitungsdatum tracken, um bewusst mehr Variation in den Speiseplan zu bringen.

## Features

- **Rezeptverwaltung** — Erstellen, bearbeiten, löschen mit Markdown-Unterstützung in der Detailansicht
- **Volltextsuche** — Suche über Titel, Zutaten und Anleitung
- **Kategoriefilter** — Mittagessen, Brot, Party, Kuchen, Snacks
- **Spezialfilter** — "Länger nicht gemacht", "Nächste 7 Tage", nach Bewertung
- **Kombinierte Filter** — mehrere Filter gleichzeitig aktiv, als URL speicher- und teilbar
- **Gespeicherte Filter** — häufig genutzte Filterkombinationen für Schnellzugriff
- **Einklappbare Filter** — mehr Platz für die Rezeptliste auf kleinen Displays
- **Bewertung** — 3–5 Sterne, inline ohne Edit-Modus
- **Wochenplanung** — Wochenübersicht geplanter Rezepte, Navigation mit Pfeiltasten
- **Datum-Tracking** — "Geplant am" und "Zuletzt gekocht", Wochentag-Picker
- **Duplikat-Erkennung** — während der Titeleingabe, Übersicht und Merge-Funktion
- **Kein Login** — LAN-only, beide Partner = gleicher Nutzer

## Tech Stack

| Schicht | Technologie |
|---------|-------------|
| Backend | Rust + Axum |
| Templates | Askama (SSR) |
| Datenbank | SQLite + sqlx |
| Frontend | HTMX + vanilla CSS |
| Tests (Unit/Integration) | `cargo test` |
| Tests (E2E) | Playwright |
| Deployment | Docker |

**Architekturprinzipien:**
- Server-Side Rendering — keine JSON-APIs für die UI
- Progressive Enhancement — funktioniert ohne JavaScript
- DeepLink-fähige URLs mit Query-Parametern
- SQLite-Datei extern gemountet (backup-fähig)

## Projektstruktur

```
src/
├── main.rs, lib.rs, config.rs, db.rs, error.rs, markdown.rs
├── models/          # Rust-Datenstrukturen
├── routes/          # Axum-Handler
├── static/css/      # Vanilla CSS
└── templates.rs     # Askama-Template-Bindungen

templates/           # Askama HTML-Templates
├── base.html, index.html, heute.html, wochenvorschau.html
├── components/      # Wiederverwendbare Partial-Templates
├── error/
└── recipes/         # Rezept-spezifische Templates

tests/
├── *.rs             # Integrationstests (Rust)
├── e2e/             # Playwright E2E-Tests
└── seeds/           # SQL-Seed-Skripte für Testdaten

docs/
├── product/         # Product Brief, PRD, Architektur
├── templates/       # Story/Plan/Review-Vorlagen
├── XX-story-name/   # Eine Datei pro Story
└── stories_epics.md # Backlog-Übersicht
```

## Entwicklung

### Voraussetzungen

- Rust (stable)
- Node.js + npm (für Playwright)
- SQLite

### Starten

```bash
# Abhängigkeiten installieren
npm install

# App starten (mit .env oder Umgebungsvariable DATABASE_URL)
DATABASE_URL=sqlite:./data/recipes.db cargo run
```

### Tests

```bash
# Unit- und Integrationstests
cargo test

# Linting
cargo clippy -- -D warnings

# E2E-Tests (startet die App automatisch)
npm run test:e2e
```

### Docker

```bash
docker build -t rezepte .
docker run -p 8080:8080 -v ./data:/data rezepte
```

---

## AI-Entwicklungsworkflow

Dieses Projekt nutzt [OpenCode](https://opencode.ai) mit einem strukturierten Multi-Agenten-System für die Entwicklung. Stories werden in Phasen abgearbeitet: Refining → Plan → Implementierung (TDD) → Review.

### Agents

Agents sind Primary-Agenten, die direkt aufgerufen oder per `@`-Mention angesprochen werden können. Sie orchestrieren den gesamten Story-Workflow.

| Agent | Beschreibung |
|-------|-------------|
| `run-story` | Führt eine einzelne Story vollständig durch alle Phasen. Wird mit einer Story-Nummer aufgerufen (z.B. `@run-story 42`) oder wählt automatisch die nächste offene Story |
| `complete-backlog` | Arbeitet das gesamte Backlog autonom ab — Story für Story, in Abhängigkeitsreihenfolge, mit iMessage-Benachrichtigungen nach jeder Story |

**Workflow von `run-story`:**

```
Phase 0: Status → "In Arbeit"
Phase 1: refine-story  → story.md
Phase 2: plan-implementation → plan.md
Phase 3: implement (+ tdd) → Implementierung
Phase 4: review-implementation → review.md
  └─ Prio-1-Findings? → Phase 5: rework (+ tdd) → erneutes review → Schleife
Abschluss: Status → "Abgeschlossen"
```

### Skills

Skills sind wiederverwendbare Instruktionsmodule, die von Agents per `skill`-Tool geladen werden. Sub-Agenten erhalten damit fokussierten Kontext für ihre jeweilige Aufgabe.

| Skill | Wann geladen | Aufgabe |
|-------|--------------|---------|
| `refine-story` | Phase 1 von `run-story` | Erstellt oder vervollständigt `story.md` mit Akzeptanzkriterien (Definition of Ready) |
| `plan-implementation` | Phase 2 von `run-story` | Erstellt `plan.md` mit technischen Implementierungsschritten, betroffenen Dateien und Testplanung |
| `implement` | Phase 3 von `run-story` | Implementiert den Plan nach TDD; führt alle Qualitätschecks durch (`cargo build`, `clippy`, `cargo test`, `npm run test:e2e`) |
| `review-implementation` | Phase 4 + 5b von `run-story` | Prüft Implementierung gegen Akzeptanzkriterien und DoD; erstellt `review.md` mit priorisierten Findings |
| `rework` | Phase 5a von `run-story` | Behebt ausschließlich Prio-1-Findings aus dem Review (blockierende Probleme) |
| `tdd` | Zusammen mit `implement` und `rework` | BDD Dual-Loop TDD: Outer Loop (Integration Test rot → grün), Inner Loop (Unit Test rot → grün → refactor) |
| `frag-den-user` | Bei Blockern in `complete-backlog` | Sendet eine Frage per iMessage an den User und wartet auf die Antwort (für autonome Läufe ohne aktive Benutzerinteraktion) |
| `informiere-den-user` | Nach jeder Story in `complete-backlog` | Sendet eine Statusmeldung per iMessage, ohne auf eine Antwort zu warten |

### Commands

Commands sind direkt in OpenCode per `/`-Prefix aufrufbar.

| Command | Aufruf | Aufgabe |
|---------|--------|---------|
| `add-story` | `/add-story <Beschreibung>` | Fügt eine neue Story zum Backlog hinzu: vergibt Story-Nummer, ordnet einem Epic zu, erstellt das Story-Verzeichnis mit rudimentärer `story.md` und trägt die Story in `stories_epics.md` ein |

### Story-Verzeichnisstruktur

Jede Story hat ein eigenes Verzeichnis unter `docs/`:

```
docs/XX-story-name/
├── story.md    # Fachliche Beschreibung mit Akzeptanzkriterien
├── plan.md     # Technischer Implementierungsplan (Checkboxen)
├── review.md   # Review-Ergebnis nach der Implementierung
├── adrs.md     # (optional) Architekturentscheidungen
└── research.md # (optional) Recherche-Ergebnisse
```

Der Backlog und alle Story-Status sind in [`docs/stories_epics.md`](./docs/stories_epics.md) gepflegt.
