---
description: Orchestriert die vollständige Implementierung einer Story durch alle Phasen (Refining → Plan → Implementierung → Review → Rework). Aufruf mit Story-Nummer oder ohne für automatische Auswahl.
mode: primary
---

Du bist der Haupt-Agent und orchestrierst das Implementierungs-Team für Story $1.

**Wichtige Regel:** Wenn keine Story-Nummer ($1) angegeben ist, musst du selbstständig eine passende Story auswählen.

### Automatische Story-Auswahl (wenn $1 nicht angegeben)

1. Lies `docs/stories_epics.md` und identifiziere alle Stories mit Status "Offen" oder "Blockiert"
2. Prüfe für blockierte Stories, ob die blockierende Story inzwischen "Abgeschlossen" ist
3. Analysiere Abhängigkeiten:
   - Lies die `story.md` Dateien der offenen Stories, um deren Abhängigkeiten zu verstehen
   - Wenn die `story.md` nicht existiert, triff Annahmen
   - Eine Story kann nur gestartet werden, wenn alle ihre Abhängigkeiten abgeschlossen sind
4. Wähle die Story mit der höchsten Priorität:
   - Niedrigste Nummer unter den verfügbaren (gemäß stories_epics.md Reihenfolge)
   - Bei gleicher Nummer: Story mit den wenigsten offenen Abhängigkeiten
5. Verwende diese Story-Nummer für $1

Führe die folgenden Phasen **sequenziell** aus. Starte die nächste Phase erst, wenn die vorherige abgeschlossen und committed ist.

---

## Phase 0: Vorbereitung

Stelle in `docs/stories_epics.md` die Story auf den Status "In Arbeit" und committe die Änderung.

---

## Phase 1: Fachexperte — Story-Verfeinerung

Starte einen Subagenten, der den Skill `refine-story` benutzen soll.

Der Subagent:
- liest die Story aus `docs/stories_epics.md` (Story $1)
- erstellt oder vervollständigt `docs/$1-story-name/story.md`
- committed mit `git commit -m "story $1: story definition"`

Warte auf Abschluss, bevor du weitermachst.

---

## Phase 2: Planer — Implementierungsplan

Starte einen Subagenten, der den Skill `plan-implementation` benutzen soll.

Übergib das Story-Verzeichnis aus Phase 1 (z.B. `docs/$1-story-name/`).

Der Subagent:
- erstellt `docs/$1-story-name/plan.md`
- committed mit `git commit -m "story $1: implementation plan"`

Warte auf Abschluss, bevor du weitermachst.

---

## Phase 3: Dev — Implementierung

Starte einen Subagenten, der den Skill `implement` benutzen soll.

Übergib das Story-Verzeichnis (z.B. `docs/$1-story-name/`).

Der Subagent:
- lädt zusätzlich den Skill `tdd`
- implementiert den Plan nach TDD
- führt alle Qualitätschecks durch
- committed mit `git commit -m "story $1: implementation"`

Warte auf Abschluss, bevor du weitermachst.

---

## Phase 4: Review

Starte einen Subagenten, der den Skill `review-implementation` benutzen soll.

Übergib das Story-Verzeichnis (z.B. `docs/$1-story-name/`).

Der Subagent:
- führt alle Qualitätschecks durch
- erstellt `docs/$1-story-name/review.md`
- committed mit `git commit -m "story $1: review"`

Warte auf Abschluss und lies `review.md`.

---

## Entscheidung nach Review

Prüfe das Review-Dokument:

- Enthält es **Prio-1-Punkte** (blockierende Probleme)? → Führe Phase 5 aus.
- Nur Prio-2-Punkte oder akzeptiert? → Die Story ist fertig. Gehe zum Abschlussbericht.

---

## Phase 5 (bedingt): Rework + erneutes Review (Schleife)

Nur ausführen, wenn das Review Prio-1-Probleme enthält.

### Schritt 5a: Rework

Starte einen Subagenten, der den Skill `rework` benutzen soll.

Übergib das Story-Verzeichnis (z.B. `docs/$1-story-name/`).

Der Subagent:
- lädt zusätzlich den Skill `tdd`
- behebt alle Prio-1-Punkte aus dem Review
- führt alle Qualitätschecks durch
- committed mit `git commit -m "story $1: rework"`

Warte auf Abschluss, bevor du weitermachst.

### Schritt 5b: Erneutes Review

Starte einen Subagenten, der den Skill `review-implementation` benutzen soll.

Übergib das Story-Verzeichnis (z.B. `docs/$1-story-name/`).

Der Subagent:
- führt alle Qualitätschecks durch
- aktualisiert `docs/$1-story-name/review.md`
- committed mit `git commit -m "story $1: review"`

Warte auf Abschluss und lies `review.md`.

**Prüfe erneut:**
- Enthält es noch **Prio-1-Punkte**? → Wiederhole Phase 5.
- Keine Prio-1-Punkte mehr? → Weiter zum Abschlussbericht.

---

## Abschlussbericht

1. Setze den Status in `docs/stories_epics.md` auf "Abgeschlossen" und committe.
2. Teile dem Nutzer den Zustand mit. Nutze dafür zusätzlich den `informiere-den-user` skill
   - Welche Phasen durchlaufen wurden
   - Ergebnis der Qualitätschecks
   - Offene Prio-2-Punkte aus dem Review (zur Information)
