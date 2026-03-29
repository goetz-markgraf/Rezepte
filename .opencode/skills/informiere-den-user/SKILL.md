---
name: informiere-den-user
description: this should be used by an orchestration agent when wanting to inform the user about an update. In this case, the user is probably not sitting and waiting at the computer but somewhere else. It uses iMessage to get to the user
version: 1.0.0
---

# Informiere den User per iMessage

Sende dem User eine Info-Nachricht per iMessage — ohne auf eine Antwort zu warten.

## Wann verwenden

Wenn du den User über einen Fortschritt oder ein Ergebnis informieren möchtest und er gerade nicht aktiv im Chat ist — z.B. um ihn über den Abschluss einer längeren autonomen Aufgabe zu benachrichtigen.

## Vorgehen

1. **Nachricht senden** (aus dem Projektverzeichnis `scripts/`):
   ```
   osascript ~/bin/send_question.applescript "<Deine Nachricht hier>"
   ```

2. **Weiterarbeiten** — es wird keine Antwort erwartet.

## Technischer Hintergrund

- `send_question.applescript`: Sendet eine iMessage an `iuranien@web.de` (der User)

## Hinweise

- Die Scripts liegen unter `bin/` relativ zum $HOME
