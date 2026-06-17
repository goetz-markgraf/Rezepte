# Features

Übersicht aller Produktfunktionen mit technischen Ankern.

## Rezeptverwaltung

Erstellen, Anzeigen, Bearbeiten und Löschen von Rezepten. Formulare werden serverseitig gerendert und validiert. Validierungslogik liegt in [[src/models/recipe.rs#validate_recipe_fields]].

## Volltextsuche

Suche über Titel, Zutaten und Anleitung via SQL `LIKE`-Abfrage. Der Suchbegriff wird als URL-Parameter `q` übergeben und bleibt so DeepLink-fähig.

## Kategoriefilter

Filter nach einer oder mehreren Kategorien gleichzeitig. Aktive Kategorien werden als mehrfache `kategorie`-Parameter in der URL gespeichert.

## Spezialfilter

Zwei spezielle Filteransichten:

- **Länger nicht gemacht** — Rezepte, die länger als N Tage nicht gekocht wurden (oder nie).
- **Nächste 7 Tage** — Rezepte mit `planned_date` in den kommenden 7 Tagen.

## Kombinierte Filter

Mehrere Filter (Suche, Kategorien, Spezialfilter) können gleichzeitig aktiv sein. Die Kombination ist als URL speicher- und teilbar.

## Gespeicherte Filter

Häufig genutzte Filterkombinationen können unter einem Namen gespeichert werden. Der aktuelle URL-Query-String wird als `query_string` in der DB abgelegt (Modell: [[domain#Saved Filter]]).

## Einklappbare Filter

Der Filterbereich kann ein- und ausgeklappt werden, um auf kleinen Displays mehr Platz für die Rezeptliste zu schaffen. Der Zustand wird als `filter_collapsed`-Parameter in der URL gespeichert.

## Wochenplanung

Übersichtsseite (`/wochenvorschau`) zeigt geplante Rezepte der aktuellen und umliegenden Wochen. Navigation mit Pfeiltasten. Rezepte können direkt einem Wochentag zugewiesen werden.

## Heute-Ansicht

`/heute` zeigt Rezepte, die für heute geplant sind.

## Datum-Tracking

Jedes Rezept kann ein `planned_date` haben. Der Wochentag-Picker erlaubt die schnelle Zuweisung zu einem Wochentag ohne manuelles Datum eingeben.

## Duplikat-Erkennung und Merge

Live-Duplikat-Warnung beim Erstellen. Übersichtsseite für alle erkannten Duplikatpaare. Merge-Funktion führt zwei Rezepte zusammen. Siehe [[domain#Duplikat-Erkennung]] und [[domain#Merge-Strategie]].

## Markdown-Unterstützung

Zutaten und Anleitung werden als Markdown gespeichert und in der Detailansicht gerendert. Unterstützt: Tabellen, Durchgestrichen, Aufgabenlisten (Checkboxen). XSS-Sanitisierung via ammonia. Siehe [[src/markdown.rs#render_and_sanitize]].

## Emoji-Anzeige

Rezepte mit Inhalt erhalten ein automatisch bestimmtes Emoji in der Listenansicht. Siehe [[domain#Emoji-Zuordnung]].
