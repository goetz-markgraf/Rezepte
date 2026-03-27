Prüfe die Implementierung der Story im Verzeichnis $ARGUMENTS (z.B. `docs/03-story-name/`).
Lies dir alle Dateien im zugehörigen Story-Verzeichnis durch. Beachte insbesondere, wenn es schon ein Review-Dokument gibt.
Beachte @docs/definition_done.md (Implementierungs-Vollständigkeit) und @docs/product/architecture.md .

Führe die Qualitätschecks aus: `cargo clippy -- -D warnings`, `cargo test`, `npm run test:e2e`.

Erstelle ein Review-Dokument `review.md` im Story-Verzeichnis oder aktualisiere es, wenn es schon existiert.
Nutze @docs/templates/review.md als strukturelle Vorlage. Pflichtinhalt:
- Prüfung aller Akzeptanzkriterien (✅/⚠️/❌)
- Prüfung gegen DoD-Checkliste
- Test-Ergebnisse
- Empfohlene Nacharbeit (priorisiert)
- Fazit mit Gesamtbewertung

## Abschluss

Erstelle nach Fertigstellung einen Commit mit der Story-Nummer aus dem Verzeichnisnamen:
`git commit -m "story XX: review"`
