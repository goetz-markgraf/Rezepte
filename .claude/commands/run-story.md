Du bist der Haupt-Agent und orchestrierst das Implementierungs-Team für Story $ARGUMENTS.

Führe die folgenden Phasen **sequenziell** aus. Starte die nächste Phase erst, wenn die vorherige abgeschlossen und committed ist.

---

## Phase 0: Vorbereitung

Stelle in @docs/stories_epics.md die Story auf den Status "in Arbeit"

---

## Phase 1: Fachexperte — Story-Verfeinerung

Starte einen Subagenten (Agent-Tool) mit diesen Anweisungen:

> Lies `.claude/commands/refine-story.md` und führe die Anweisungen aus.
> Arbeite für Story $ARGUMENTS.

Der Subagent soll:
- die Story `$ARGUMENTS` aus `docs/stories_epics.md` lesen
- `docs/$ARGUMENTS-story-name/story.md` erstellen
- mit `git commit -m "story $ARGUMENTS: story definition"` committen

Warte auf Abschluss. Lies das erstellte Story-Verzeichnis, um den genauen Pfad zu ermitteln.

---

## Phase 2: Planer — Implementierungsplan

Starte einen Subagenten (Agent-Tool) mit diesen Anweisungen:

> Lies `.claude/commands/plan-story.md` und führe die Anweisungen aus.
> Das Story-Verzeichnis ist: `docs/$ARGUMENTS-story-name/`

Der Subagent soll einen hohen Aufwand betreiben (/effort high)

Der Subagent soll:
- `docs/$ARGUMENTS-story-name/plan.md` erstellen
- mit `git commit -m "story $ARGUMENTS: implementation plan"` committen

Warte auf Abschluss, bevor du weitermachst.

---

## Phase 3: Dev — Implementierung

Starte einen Subagenten (Agent-Tool) mit diesen Anweisungen:

> Lies `.claude/commands/implement-story.md` und führe die Anweisungen aus.
> Das Story-Verzeichnis ist: `docs/$ARGUMENTS-story-name/`

Der Subagent soll:
- den Plan gemäß TDD implementieren
- alle Qualitätschecks durchführen
- mit `git commit -m "story $ARGUMENTS: implementation"` committen

Warte auf Abschluss, bevor du weitermachst.

---

## Phase 4: Review

Starte einen Subagenten (Agent-Tool) mit diesen Anweisungen:

> Lies `.claude/commands/review-story.md` und führe die Anweisungen aus.
> Das Story-Verzeichnis ist: `docs/$ARGUMENTS-story-name/`

Der Subagent soll:
- alle Qualitätschecks durchführen
- `docs/$ARGUMENTS-story-name/review.md` erstellen
- mit `git commit -m "story $ARGUMENTS: review"` committen

Warte auf Abschluss und lies `review.md`.

---

## Entscheidung nach Review

Prüfe das Review-Dokument:

- Enthält es **Prio-1-Punkte** (blockierende Probleme)? → Führe Phase 5 aus.
- Nur Prio-2-Punkte oder akzeptiert? → Story fertig. Abschlussbericht erstellen.

---

## Phase 5 (bedingt): Dev Rework

Nur ausführen, wenn das Review Prio-1-Probleme enthält.

Starte einen Subagenten (Agent-Tool) mit diesen Anweisungen:

> Lies `.claude/commands/rework-story.md` und führe die Anweisungen aus.
> Das Story-Verzeichnis ist: `docs/$ARGUMENTS-story-name/`

Der Subagent soll:
- alle Prio-1-Punkte aus dem Review beheben
- alle Qualitätschecks durchführen
- mit `git commit -m "story $ARGUMENTS: rework"` committen

---

## Abschlussbericht

Teile dem Nutzer nach Abschluss aller Phasen mit:
- Welche Phasen durchlaufen wurden
- Ergebnis der Qualitätschecks
- Offene Prio-2-Punkte aus dem Review (zur Information)
- Status in @docs/stories_epics.md aktualisieren ("Abgeschlossen") inkl. Commit
