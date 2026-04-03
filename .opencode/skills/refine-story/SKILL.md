---
name: refine-story
description: Verfeinert eine Story fachlich und erstellt oder vervollständigt die story.md. Zu verwenden wenn eine Story-Nummer bekannt ist und die story.md erstellt oder aktualisiert werden soll.
version: 1.0.0
---

# Story Refinement

Verfeinert die angegebene Story und erstellt oder vervollständigt `story.md` im Story-Verzeichnis.

## Eingabe

Die Story-Nummer und das Verzeichnis werden als Kontext übergeben (z.B. Story 03, Verzeichnis `docs/03-story-name/`).

## Vorbereitung: Story-Zustand prüfen

Bevor du mit dem Refining beginnst:

1. Lies `docs/stories_epics.md` und ermittle Nummer und Verzeichnisname der Story.
2. Prüfe, ob das Verzeichnis `docs/XX-story-name/` bereits existiert.
3. Prüfe, ob darin eine `story.md` existiert und ob sie bereits Inhalt über die Template-Platzhalter hinaus enthält.

**Wenn story.md bereits Inhalt hat:** Lies den vorhandenen Inhalt, identifiziere fehlende oder unvollständige Abschnitte und ergänze bzw. verbessere diese — anstatt die Datei neu zu erstellen. Falls ein Abschnitt `## Zusatzinformationen` vorhanden ist, nutze dessen Inhalt als primäre fachliche Grundlage für das Refining. Entferne diesen Abschnitt nach dem Verarbeiten aus der fertigen story.md.

**Wenn story.md fehlt oder nur Template-Platzhalter enthält:** Erstelle sie vollständig neu.

## Ressourcen

- Vorlage: `docs/templates/story.md`
- Qualitätskriterien: `docs/definition_ready.md`
- Fachlicher Kontext: `docs/product/product-brief-Rezepte.md`, `docs/product/prd.md`

## Fokus

- Nur fachliche Aspekte — keine technischen Details
- Klare, testbare Akzeptanzkriterien
- Definition of Ready muss erfüllt sein

## Abschluss

Erstelle einen Commit mit der Story-Nummer:
```
git commit -m "story XX: story definition"
```
