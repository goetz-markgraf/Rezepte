# Domain

Die Kerndomain dreht sich um Familienrezepte: Erstellen, Suchen, Planen und Variieren des Speiseplans. Alle Nutzer im LAN sind gleichgestellt — kein Authentifizierungssystem.

## Recipe

Das zentrale Datenobjekt. Gespeichert in SQLite, repräsentiert als [[src/models/recipe.rs#Recipe]].

Felder:
- `title` — Pflichtfeld, max. 100 Zeichen
- `categories` — JSON-Array, min. 1 Eintrag aus den [[domain#Kategorien|gültigen Kategorien]]
- `ingredients` — optional, max. 2000 Zeichen, Markdown-fähig
- `instructions` — optional, max. 5000 Zeichen, Markdown-fähig
- `planned_date` — optionales Datum (deutsches Format T.M.JJJJ), für Wochenplanung
- `created_at`, `updated_at` — automatisch verwaltet

## Kategorien

Feste, vordefinierte Liste ohne Konfigurierbarkeit zur Laufzeit. Definiert in [[src/models/recipe.rs#VALID_CATEGORIES]]. Ein Rezept muss mindestens einer Kategorie angehören. Neue Kategorien erfordern eine Code-Änderung.

## Datumsformat

Datumseingaben erfolgen im deutschen Format `T.M.JJJJ` (z.B. `5.3.2025`). Zweistellige Jahreszahlen werden als `20xx` interpretiert. Die Parsing-Logik liegt in [[src/models/recipe.rs#parse_german_date]].

## Saved Filter

Nutzer können aktive Filterkombinationen (URL-Query-String) unter einem Namen speichern. Repräsentiert als [[src/models/saved_filter.rs#SavedFilter]]. Name max. 100 Zeichen, Query-String darf nicht leer sein.

## Duplikat-Erkennung

Live-Duplikatwarnung beim Erstellen, Übersichtsseite für alle Duplikatpaare und Merge-Funktion.

Beim Eingeben eines Titels wird live nach ähnlichen Rezepten gesucht. Auf der Übersichtsseite werden alle Paare mit ähnlichen Titeln angezeigt. [[src/models/recipe.rs#determine_merge_target]] bestimmt automatisch Ziel und Quelle.

## Merge-Strategie

Das "wertvollere" Rezept wird als Ziel vorgeschlagen. Priorisierung: mehr ausgefüllte Felder → Ziel; neueres `updated_at` → Ziel; kleinere ID als Fallback.

## Emoji-Zuordnung

Rezepte erhalten ein Emoji basierend auf Schlüsselwörtern in Titel, Zutaten und Anleitung. Nur Rezepte mit Inhalt (Zutaten oder Anleitung nicht leer) erhalten ein Emoji. Logik in [[src/emoji.rs#recipe_emoji]].
