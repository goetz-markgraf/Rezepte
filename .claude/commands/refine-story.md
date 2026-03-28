Ich möchte die nächste Story aus @docs/stories_epics.md refinen.
Stelle mir Fragen, wenn etwas unklar ist.
Erstelle nur die Story, mit Fokus auf die fachlichen Aspekte.
Beachte die @docs/definition_ready.md , die eine fertige Story erfüllen muss.

## Vorbereitung: Story-Zustand prüfen

Bevor du mit dem Refining beginnst:

1. Lies `docs/stories_epics.md` und ermittle Nummer und Verzeichnisname der Story.
2. Prüfe, ob das Verzeichnis `docs/XX-story-name/` bereits existiert.
3. Prüfe, ob darin eine `story.md` existiert und ob sie bereits Inhalt über die Template-Platzhalter hinaus enthält.

**Wenn story.md bereits Inhalt hat:** Lies den vorhandenen Inhalt, identifiziere fehlende oder unvollständige Abschnitte und ergänze bzw. verbessere diese — anstatt die Datei neu zu erstellen.

**Wenn story.md fehlt oder nur Template-Platzhalter enthält:** Erstelle sie vollständig neu.

Nutze @docs/templates/story.md als strukturelle Vorlage.

## Abschluss

Erstelle nach Fertigstellung einen Commit mit der Story-Nummer aus dem Verzeichnisnamen:
`git commit -m "story XX: story definition"`
