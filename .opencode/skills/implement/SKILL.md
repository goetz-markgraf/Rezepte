---
name: implement
description: Implementiert den Plan einer Story nach TDD. Zu verwenden wenn plan.md vorliegt und die Implementierung beginnen soll. Führt alle Qualitätschecks durch.
version: 1.0.0
---

# Implementierung nach TDD

Implementiert den Plan aus `plan.md` im Story-Verzeichnis nach dem TDD-Ansatz.

## Eingabe

Das Story-Verzeichnis wird als Kontext übergeben (z.B. `docs/03-story-name/`).

## Vorbereitung

1. Lies `docs/XX-story-name/story.md` — Akzeptanzkriterien verstehen
2. Lies `docs/XX-story-name/plan.md` — Implementierungsschritte
3. Lies `docs/product/architecture.md` — technische Constraints
4. Lies `docs/definition_done.md` — DoD-Kriterien
5. Lade den `tdd` Skill für die TDD-Methodik

## TDD-Vorgehen

Folge dem BDD Dual-Loop TDD (siehe `tdd` Skill):

1. **Outer Loop**: Integration/Acceptance Test schreiben (rot)
2. **Inner Loop**: Unit Tests + Implementierung (rot → grün → refactor)
3. Integration Test grün machen
4. Refactoring mit allen Tests grün

**Nie den roten Schritt überspringen.**

## Tech Stack

- Rust + Axum + Askama + sqlx + SQLite + HTMX
- Server-Side Rendering (keine JSON-APIs für UI)
- Progressive Enhancement (funktioniert ohne JS)
- E2E-Tests: Playwright (`npm run test:e2e`)
- Testdaten: SQL-Seed-Skripte in `tests/seeds/`

## Qualitätschecks (alle müssen bestehen)

```bash
cargo build
cargo clippy -- -D warnings
cargo test
npm run test:e2e
```

## Abschluss

Erstelle einen Commit:
```
git commit -m "story XX: implementation"
```
