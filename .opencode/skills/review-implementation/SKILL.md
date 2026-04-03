---
name: review-implementation
description: Reviewt eine fertige Implementierung gegen Akzeptanzkriterien und DoD. Erstellt oder aktualisiert review.md mit priorisierten Findings. Zu verwenden nach der Implementierung.
version: 1.0.0
---

# Implementierung reviewen

Prüft die Implementierung einer Story und erstellt oder aktualisiert `review.md`.

## Eingabe

Das Story-Verzeichnis wird als Kontext übergeben (z.B. `docs/03-story-name/`).

## Vorbereitung

1. Lies alle Dateien im Story-Verzeichnis (`story.md`, `plan.md`, ggf. vorhandenes `review.md`)
2. Lies `docs/definition_done.md` — DoD-Checkliste
3. Lies `docs/product/architecture.md` — Architektur-Constraints
4. Lies `docs/templates/review.md` — Vorlage für review.md
5. Analysiere die implementierten Dateien aus dem Plan

## Qualitätschecks

```bash
cargo clippy -- -D warnings
cargo test
npm run test:e2e
```

## review.md Pflichtinhalt

- Prüfung aller Akzeptanzkriterien aus story.md (✅/⚠️/❌)
- DoD-Checkliste (✅/⚠️/❌)
- Test-Ergebnisse (Anzahl Tests, Fehler)
- Empfohlene Nacharbeit — priorisiert nach Prio 1 / Prio 2
- Fazit mit Gesamtbewertung

## Prioritäten

**Prio 1 (blockiert Abschluss):**
- Failing Tests (`cargo test`, `cargo clippy`, E2E)
- Nicht erfüllte Akzeptanzkriterien
- Datenverlust- oder Sicherheitsprobleme
- Architektur-Verstöße (CDN-Abhängigkeit, JSON-API statt SSR, etc.)

**Prio 2 (nice-to-have):**
- Code-Verbesserungen ohne funktionale Auswirkung
- Fehlende optionale Kriterien
- Stilistische oder strukturelle Anmerkungen

## Abschluss

Erstelle einen Commit:
```
git commit -m "story XX: review"
```
