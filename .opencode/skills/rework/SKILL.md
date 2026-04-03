---
name: rework
description: Behebt Prio-1-Punkte aus einem Review. Zu verwenden wenn review.md Prio-1-Probleme enthält die vor dem Abschluss behoben werden müssen.
version: 1.0.0
---

# Rework nach Review

Behebt alle Prio-1-Punkte aus `review.md` im Story-Verzeichnis.

## Eingabe

Das Story-Verzeichnis wird als Kontext übergeben (z.B. `docs/03-story-name/`).

## Vorbereitung

1. Lies `docs/XX-story-name/review.md` — alle Prio-1-Punkte identifizieren
2. Lies `docs/XX-story-name/story.md` — Akzeptanzkriterien als Referenz
3. Lies `docs/XX-story-name/plan.md` — ursprünglicher Plan als Kontext
4. Lies `docs/product/architecture.md` — Architektur-Constraints
5. Lies `docs/definition_done.md` — DoD-Kriterien
6. Lade den `tdd` Skill für die TDD-Methodik

## Vorgehen

- Behebe **ausschließlich** die Prio-1-Punkte aus dem Review
- Prio-2-Punkte sind optional — nur beheben wenn sinnvoll und risikoarm
- TDD-Ansatz beibehalten: Tests zuerst, dann Implementierung
- Keine neuen Features oder Scope-Erweiterungen

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
git commit -m "story XX: rework"
```
