# Testing

Teststrategie für Rezepte: Unit-Tests in Rust, Integrationstests gegen eine echte SQLite-Instanz, und E2E-Tests mit Playwright gegen den laufenden Server.

## Unit Tests

Unit-Tests liegen direkt in den Quelldateien als `#[cfg(test)] mod tests`. Getestet werden Parsing, Validierung und Geschäftslogik ohne Datenbankzugriff.

Abgedeckte Bereiche:
- [[src/models/recipe.rs]] — Datumsformat-Parsing, Rezeptvalidierung, Merge-Target-Bestimmung
- [[src/models/saved_filter.rs]] — SavedFilter-Validierung
- [[src/markdown.rs]] — Markdown-Rendering und XSS-Sanitisierung
- [[src/emoji.rs]] — Emoji-Keyword-Matching

## Integrationstests

Rust-Integrationstests unter `tests/*.rs` testen vollständige HTTP-Requests gegen die App (Axum `TestClient`). Jeder Test erhält eine frische SQLite-Datenbank (via `tempfile`).

Testdateien:
- `tests/health_check.rs` — Health-Endpunkt
- `tests/recipe_create.rs`, `tests/recipe_detail.rs`, etc. — Feature-spezifische Tests

## E2E-Tests

Playwright-Tests unter `tests/e2e/*.spec.ts` starten die App als echten Prozess und fahren Browser-Szenarien. Datenbankzustand wird über die [[routes#Test-API-Routen]] kontrolliert.

Test-Seed-Daten liegen unter `tests/seeds/` als SQL-Skripte.

Abgedeckte Szenarien (Auswahl):
- `recipe-create.spec.ts`, `recipe-edit.spec.ts`, `recipe-delete.spec.ts`
- `recipe-search.spec.ts`, `recipe-category-filter.spec.ts`
- `recipe-combined-filters.spec.ts`, `recipe-not-made-filter.spec.ts`, `recipe-next-seven-days-filter.spec.ts`
- `recipe-duplicate-check.spec.ts`, `recipe-duplicates-overview.spec.ts`, `recipe-merge.spec.ts`
- `saved-filters.spec.ts`, `filter-collapse.spec.ts`
- `wochenvorschau.spec.ts`, `wochenvorschau-15-tage.spec.ts`, `heute.spec.ts`
- `recipe-date.spec.ts`, `weekday-picker-extended.spec.ts`, `weekday-picker-planned-indicator.spec.ts`
- `recipe-markdown.spec.ts`, `recipe-emoji.spec.ts`
- `accessibility.spec.ts`, `responsive-layout.spec.ts`
- `navigation-inspiration.spec.ts`, `header-navigation.spec.ts`

## Test ausführen

Befehle zum Starten der verschiedenen Test-Suites.

```bash
# Unit- und Integrationstests
cargo test

# E2E-Tests (startet App automatisch)
npm run test:e2e
```
