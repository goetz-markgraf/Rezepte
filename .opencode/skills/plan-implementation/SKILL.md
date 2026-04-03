---
name: plan-implementation
description: Erstellt einen technischen Implementierungsplan (plan.md) für eine Story. Zu verwenden nach dem Story-Refining, wenn story.md vorliegt und die technische Planung beginnen soll.
version: 1.0.0
---

# Implementierungsplan erstellen

Erstellt `plan.md` im Story-Verzeichnis mit einem vollständigen technischen Implementierungsplan.

## Eingabe

Das Story-Verzeichnis wird als Kontext übergeben (z.B. `docs/03-story-name/`).

## Vorgehen

1. Lies `story.md` im Story-Verzeichnis — das ist die fachliche Grundlage
2. Lies `docs/product/architecture.md` für technische Rahmenbedingungen
3. Lies `docs/definition_done.md` für DoD-Kriterien
4. Lies `docs/templates/plan.md` als strukturelle Vorlage
5. Analysiere die relevanten Teile der Codebase (`src/`, `templates/`, `tests/`)
6. Erstelle `plan.md` im Story-Verzeichnis

## Anforderungen an den Plan

- Alle DoD-Kriterien müssen abgedeckt sein (Tests, Code-Qualität, Architektur-Konformität)
- Schritte sind klein und einzeln umsetzbar (checkbox-Liste `[ ]`)
- Betroffene Dateien explizit aufgelistet
- Unit-Tests, Integrationstests und E2E-Tests eingeplant
- Tech Stack beachten: Rust + Axum + Askama + sqlx + SQLite + HTMX

## Abschluss

Erstelle einen Commit:
```
git commit -m "story XX: implementation plan"
```
