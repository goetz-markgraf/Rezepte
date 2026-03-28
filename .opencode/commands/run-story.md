Du bist der Haupt-Agent und orchestrierst das Implementierungs-Team für Story $1.

Führe die folgenden Phasen **sequenziell** aus. Starte die nächste Phase erst, wenn die vorherige abgeschlossen und committed ist.

---

## Phase 0: Vorbereitung

Stelle in @docs/stories_epics.md die Story auf den Status "in Arbeit"

---

## Phase 1: Fachexperte — Story-Verfeinerung

Starte einen Subagenten mit den Anweisungen aus `.opencode/commands/refine-story.md`.

Der Subagent:
- liest die nächste Story aus `docs/stories_epics.md` (Story $1)
- erstellt `docs/$1-story-name/story.md`
- committed mit `git commit -m "story $1: story definition"`

Warte auf Abschluss, bevor du weitermachst.

---

## Phase 2: Planer — Implementierungsplan

Starte einen Subagenten mit den Anweisungen aus `.opencode/commands/plan-implementation.md`.

Übergib als Argument das Story-Verzeichnis, das in Phase 1 erstellt wurde (z.B. `docs/$1-story-name/`).

Der Subagent:
- erstellt `docs/$1-story-name/plan.md`
- committed mit `git commit -m "story $1: implementation plan"`

Warte auf Abschluss, bevor du weitermachst.

---

## Phase 3: Dev — Implementierung

Starte einen Subagenten mit den Anweisungen aus `.opencode/commands/implement.md`.

Übergib als Argument das Story-Verzeichnis (z.B. `docs/$1-story-name/`).

Der Subagent:
- implementiert den Plan gemäß TDD
- führt alle Qualitätschecks durch
- committed mit `git commit -m "story $1: implementation"`

Warte auf Abschluss, bevor du weitermachst.

---

## Phase 4: Review

Starte einen Subagenten mit den Anweisungen aus `.opencode/commands/review-implementation.md`.

Übergib als Argument das Story-Verzeichnis (z.B. `docs/$1-story-name/`).

Der Subagent:
- führt alle Qualitätschecks durch
- erstellt `docs/$1-story-name/review.md`
- committed mit `git commit -m "story $1: review"`

Warte auf Abschluss und lies `review.md`.

---

## Entscheidung nach Review

Prüfe das Review-Dokument:

- Enthält es **Prio-1-Punkte** (blockierende Probleme)? → Führe Phase 5 aus.
- Nur Prio-2-Punkte oder akzeptiert? → Die Story ist fertig. Teile dem Nutzer das Ergebnis mit.

---

## Phase 5 (bedingt): Dev Rework + erneutes Review (Schleife)

Nur ausführen, wenn das Review Prio-1-Probleme enthält.

### Schritt 5a: Rework

Starte einen Subagenten mit den Anweisungen aus `.opencode/commands/rework.md`.

Übergib als Argument das Story-Verzeichnis (z.B. `docs/$1-story-name/`).

Der Subagent:
- behebt alle Prio-1-Punkte aus dem Review
- führt alle Qualitätschecks durch
- committed mit `git commit -m "story $1: rework"`

Warte auf Abschluss, bevor du weitermachst.

### Schritt 5b: Erneutes Review

Starte erneut einen Subagenten mit den Anweisungen aus `.opencode/commands/review-implementation.md`.

Übergib als Argument das Story-Verzeichnis (z.B. `docs/$1-story-name/`).

Der Subagent:
- führt alle Qualitätschecks durch
- aktualisiert `docs/$1-story-name/review.md`
- committed mit `git commit -m "story $1: review"`

Warte auf Abschluss und lies `review.md`.

**Prüfe erneut:**
- Enthält es noch **Prio-1-Punkte**? → Wiederhole Phase 5 (Rework + Review).
- Keine Prio-1-Punkte mehr? → Weiter zum Abschlussbericht.

---

## Abschlussbericht

Teile dem Nutzer nach Abschluss aller Phasen mit:
- Welche Phasen durchlaufen wurden
- Ergebnis der Qualitätschecks
- Offene Prio-2-Punkte aus dem Review (zur Information)
- Status in @docs/stories_epics.md aktualisieren ("Abgeschlossen") inkl. Commit
