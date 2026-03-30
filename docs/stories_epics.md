# Epics und Stories - Rezepte

**Erstellt am:** 2026-03-21  
**Basierend auf:** Product Brief und PRD

---

## Epic 1: Rezept-Verwaltung (Grundlegendes CRUD)

Das Herzstück der Anwendung - Rezepte erstellen, bearbeiten, anzeigen und löschen.

**Stories:**

| Nr | Story | Status |
|----|-------|--------|
| 01 | [Rezept erstellen](./01-rezept-erstellen/story.md) | Abgeschlossen |
| 02 | [Rezept bearbeiten](./02-rezept-bearbeiten/story.md) | Abgeschlossen |
| 03 | [Rezept löschen mit Sicherheitsabfrage](./03-rezept-loeschen/story.md) | Abgeschlossen |
| 04 | [Rezept-Detailansicht](./04-rezept-detailansicht/story.md) | Abgeschlossen |

---

## Epic 2: Rezept-Übersicht & Navigation

Übersichtliche Darstellung aller Rezepte mit alphabetischer Sortierung.

**Stories:**

| Nr | Story | Status |
|----|-------|--------|
| 05 | [Rezept-Liste alphabetisch sortiert](./05-rezept-liste/story.md) | Abgeschlossen |
| 06 | [Responsive Layout für Desktop und Mobile](./06-responsive-layout/story.md) | Abgeschlossen |
| 26 | [Nutzung von Icons](./26-nutzung-von-icons/story.md) | Abgeschlossen |

---

## Epic 3: Suche & Filterung

Intelligente Filter und Suche für schnelle Rezeptfindung.

**Stories:**

| Nr | Story | Status |
|----|-------|--------|
| 07 | [Volltextsuche über Titel, Zutaten und Anleitung](./07-volltextsuche/story.md) | Abgeschlossen |
| 27 | [Clear-Icon in Volltextsuche triggert neue Suche](./27-clear-icon-suche/story.md) | Abgeschlossen |
| 08 | [Filter nach Kategorien](./08-kategorien-filter/story.md) | Abgeschlossen |
| 09 | [Filter "Länger nicht gemacht"](./09-filter-laenger-nicht-gemacht/story.md) | Abgeschlossen |
| 10 | [Filter "Nächste 7 Tage" (geplante Rezepte)](./10-filter-nächste-7-tage/story.md) | Abgeschlossen |
| 11 | [Filter nach Bewertung (Beliebtheit)](./11-filter-bewertung/story.md) | Abgeschlossen |
| 12 | [Kombinierte Filter (mehrere Filter gleichzeitig)](./12-kombinierte-filter/story.md) | Abgeschlossen |
| 13 | [Gespeicherte Filter für Schnellzugriff](./13-gespeicherte-filter/story.md) | Abgeschlossen |

---

## Epic 4: Bewertung & Datums-Tracking

Rezepte bewerten und Zubereitungsdaten verwalten.

**Stories:**

| Nr | Story | Status |
|----|-------|--------|
| 14 | [Rezept mit 3-5 Sternen bewerten](./14-rezept-bewertung/story.md) | Abgeschlossen |
| 15 | [Datum "Geplant am" setzen (Vergangenheit und Zukunft)](./15-datum-setzen/story.md) | Dublette |
| 16 | [Wochentag-Picker für intuitive Datumauswahl](./16-wochentag-picker/story.md) | Abgeschlossen |
| 17 | [Inline-Bewertung ohne Edit-Mode](./17-inline-bewertung/story.md) | Abgeschlossen |
| 28 | [Datum-Eingabe am Rezept (geplant / gekocht)](./28-datum-eingabe/story.md) | Abgeschlossen |

---

## Epic 5: Wochenplanung

Spezifische Features für die Wochenplanung am Mittwoch/Donnerstag.

**Stories:**

| Nr | Story | Status |
|----|-------|--------|
| 18 | [Wochenvorschau für geplante Rezepte](./18-wochenvorschau/story.md) | Abgeschlossen |
| 19 | [Wochenvorschau nach Wochentagen formatiert](./19-wochentage-formatierung/story.md) | Abgeschlossen |
| 20 | ["Heute gekocht" Ansicht mit Highlight](./20-heute-gekocht/story.md) | Abgeschlossen |
| 29 | [Wochen-Picker erweitern](./29-wochen-picker-erweitern/story.md) | Offen |

---

## Epic 6: Daten-Qualität & Wartung

Features zur Aufrechterhaltung der Datenqualität.

**Stories:**

| Nr | Story | Status |
|----|-------|--------|
| 21 | [Duplikaterkennung während Titeleingabe](./21-duplikaterkennung/story.md) | Abgeschlossen |
| 22 | [Dubletten-Prüfung und Übersicht](./22-dubletten-pruefung/story.md) | Abgeschlossen |
| 23 | [Rezepte mergen (Duplikate zusammenführen)](./23-rezepte-mergen/story.md) | Abgeschlossen |

---

## Epic 7: Technische Grundlagen

Nicht-funktionale Anforderungen und technische Features.

**Stories:**

| Nr | Story | Status |
|----|-------|--------|
| 24 | [Multi-Device Zugriff im LAN](./24-multi-device/story.md) | Dublette |
| 25 | [WCAG 2.1 Level A Accessibility](./25-accessibility/story.md) | Abgeschlossen |


---

## Legende

- **Offen** - Noch nicht begonnen
- **In Arbeit** - Aktuell in Entwicklung
- **Abgeschlossen** - Implementiert und getestet
- **Blockiert** - Wartet auf andere Story/Clarification
- **Dublette** - Ist durch eine andere Story erledigt und wird daher nicht umgesetzt

