---
name: frag-den-user
description: this should be used by an orchestration agent when wanting to have an answer from the user. In this case, the user is probably not sitting and waiting at the computer but somewhere else. It uses iMessage to get to the user
version: 1.0.0
---

# Frag den User per iMessage

Sende dem User eine Frage per iMessage und warte auf die Antwort.

## Wann verwenden

Wenn du den User etwas fragen möchtest und er gerade nicht aktiv im Chat ist — z.B. für Rückfragen während längerer autonomer Arbeit, oder wenn du Input brauchst bevor du weitermachst.

## Vorgehen

1. **Frage senden** (aus dem Projektverzeichnis `scripts/`):
   ```
   osascript ~/bin/send_question.applescript "<Deine Frage hier>"
   ```

2. **Auf Antwort warten** (kann Minuten dauern — geduldig bleiben!):
   ```
   bash ~/bin/wait_for_answer.sh
   ```
   Die Antwort kommt via iCloud-Sync vom iPhone des Users und wird auf STDOUT ausgegeben.

3. **Mit der Antwort weiterarbeiten.**

## Technischer Hintergrund

- `send_question.applescript`: Sendet eine iMessage an `iuranien@web.de` (der User)
- `wait_for_answer.sh`: Wartet auf die Datei `~/Library/Mobile Documents/iCloud~is~workflow~my~workflows/Documents/KI_Antwort.txt`, die ein iOS Shortcut auf dem iPhone des Users anlegt, sobald er antwortet. Löscht die Datei danach automatisch.

## Hinweise

- Timeout ist standardmäßig 2 Minuten im Bash-Tool — bei `/frag-den-user` `timeout: 600000` (10 Min.) setzen
- Die Scripts liegen unter `bin/` relativ zum $HOME
