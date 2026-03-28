# add-story

Du bist ein Planungs-Agent. Deine Aufgabe ist es, eine neue Story in das Projekt einzufügen.

Die Eingabe des Users lautet: "$1"

Extrahiere daraus:
- **Titel:** Die erste Zeile oder der prägnante Kurztitel (wird für den Slug und die Tabelle in stories_epics.md verwendet)
- **Zusatzinformationen:** Alle weiteren Beschreibungen, Anforderungen oder Details, die der User mitgegeben hat

## Schritt 1: Kontext verstehen

Lies `docs/stories_epics.md` vollständig. Verstehe:
- Welche Epics existieren
- Welche Story-Nummern bereits vergeben sind
- Wie die Nummerierung aufgebaut ist

## Schritt 2: Epic bestimmen

Entscheide, in welches Epic die Story am besten passt.

Wenn kein bestehendes Epic passt, erstelle ein neues Epic (mit dem nächsten freien Epic-Nummer).

Wenn du unsicher bist, frage den User.

## Schritt 3: Story-Nummer vergeben

Vergib die nächste freie Story-Nummer (fortlaufend nach der aktuell höchsten Nummer, egal in welchem Epic).

Wenn die Story in ein bestehendes Epic eingefügt wird, ist ab dann die Nummerierung nicht mehr fortlaufend. Das ist OK. Ändere nicht die Nummern der bestehenden Stories.

## Schritt 4: `docs/stories_epics.md` aktualisieren

- Trage die neue Story in die Tabelle des passenden Epics ein (Status: Offen)
- Falls ein neues Epic erstellt wurde, füge es an der passenden Stelle ein

## Schritt 5: Story-Verzeichnis und rudimentäre story.md anlegen

Erstelle das Verzeichnis `docs/XX-story-name/` (XX = vergebene Nummer, story-name = kurzer Slug aus dem Titel in Kleinbuchstaben mit Bindestrichen).

Erstelle darin eine minimale `story.md` mit folgendem Inhalt (nutze `docs/templates/story.md` als Vorlage):
- Titel und Nummer korrekt gesetzt
- Epic-Name eingetragen
- Alle Platzhalter aus der Vorlage belassen (werden später im refine-Schritt gefüllt)
- Status: Offen
- Falls der User Zusatzinformationen mitgegeben hat: Füge am **Ende der Datei** einen Abschnitt `## Zusatzinformationen` ein und schreibe den vollständigen Originaltext des Users dort hinein (unverändert, als Fließtext)

## Schritt 6: Commit

Erstelle einen Commit:
`git commit -m "story XX: add story placeholder for '<Titel>'"`

## Abschluss

Teile dem Nutzer mit:
- Welche Story-Nummer vergeben wurde
- In welches Epic die Story eingeordnet wurde
- Ob eine Re-Nummerierung stattgefunden hat (und welche Stories betroffen waren)
- Pfad zur neuen story.md
