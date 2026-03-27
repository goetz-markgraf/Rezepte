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
| 04 | [Rezept-Detailansicht](./04-rezept-detailansicht/story.md) | In Arbeit |

---

## Epic 2: Rezept-Übersicht & Navigation

Übersichtliche Darstellung aller Rezepte mit alphabetischer Sortierung.

**Stories:**

| Nr | Story | Status |
|----|-------|--------|
| 05 | [Rezept-Liste alphabetisch sortiert](./05-rezept-liste/story.md) | Offen |
| 06 | [Responsive Layout für Desktop und Mobile](./06-responsive-layout/story.md) | Offen |

---

## Epic 3: Suche & Filterung

Intelligente Filter und Suche für schnelle Rezeptfindung.

**Stories:**

| Nr | Story | Status |
|----|-------|--------|
| 07 | [Volltextsuche über Titel, Zutaten und Anleitung](./07-volltextsuche/story.md) | Offen |
| 08 | [Filter nach Kategorien](./08-kategorien-filter/story.md) | Offen |
| 09 | [Filter "Länger nicht gemacht"](./09-filter-laenger-nicht-gemacht/story.md) | Offen |
| 10 | [Filter "Nächste 7 Tage" (geplante Rezepte)](./10-filter-nächste-7-tage/story.md) | Offen |
| 11 | [Filter nach Bewertung (Beliebtheit)](./11-filter-bewertung/story.md) | Offen |
| 12 | [Kombinierte Filter (mehrere Filter gleichzeitig)](./12-kombinierte-filter/story.md) | Offen |
| 13 | [Gespeicherte Filter für Schnellzugriff](./13-gespeicherte-filter/story.md) | Offen |

---

## Epic 4: Bewertung & Datums-Tracking

Rezepte bewerten und Zubereitungsdaten verwalten.

**Stories:**

| Nr | Story | Status |
|----|-------|--------|
| 14 | [Rezept mit 3-5 Sternen bewerten](./14-rezept-bewertung/story.md) | Offen |
| 15 | [Datum "Geplant am" setzen (Vergangenheit und Zukunft)](./15-datum-setzen/story.md) | Offen |
| 16 | [Wochentag-Picker für intuitive Datumauswahl](./16-wochentag-picker/story.md) | Offen |
| 17 | [Inline-Bewertung ohne Edit-Mode](./17-inline-bewertung/story.md) | Offen |

---

## Epic 5: Wochenplanung

Spezifische Features für die Wochenplanung am Mittwoch/Donnerstag.

**Stories:**

| Nr | Story | Status |
|----|-------|--------|
| 18 | [Wochenvorschau für geplante Rezepte](./18-wochenvorschau/story.md) | Offen |
| 19 | [Wochenvorschau nach Wochentagen formatiert](./19-wochentage-formatierung/story.md) | Offen |
| 20 | ["Heute gekocht" Ansicht mit Highlight](./20-heute-gekocht/story.md) | Offen |

---

## Epic 6: Daten-Qualität & Wartung

Features zur Aufrechterhaltung der Datenqualität.

**Stories:**

| Nr | Story | Status |
|----|-------|--------|
| 21 | [Duplikaterkennung während Titeleingabe](./21-duplikaterkennung/story.md) | Offen |
| 22 | [Dubletten-Prüfung und Übersicht](./22-dubletten-pruefung/story.md) | Offen |
| 23 | [Rezepte mergen (Duplikate zusammenführen)](./23-rezepte-mergen/story.md) | Offen |

---

## Epic 7: Technische Grundlagen

Nicht-funktionale Anforderungen und technische Features.

**Stories:**

| Nr | Story | Status |
|----|-------|--------|
| 24 | [Multi-Device Zugriff im LAN](./24-multi-device/story.md) | Offen |
| 25 | [WCAG 2.1 Level A Accessibility](./25-accessibility/story.md) | Offen |

---

## Priorisierung (MVP = Phase 1)

### Phase 1 (MVP) - Must Have
**Epic 1:** Stories 01-04 (Rezept CRUD)  
**Epic 2:** Stories 05-06 (Liste & Layout)  
**Epic 3:** Stories 07-09 (Suche & grundlegende Filter)  
**Epic 4:** Stories 14-16 (Bewertung & Datum)  
**Epic 5:** Story 18 (Wochenvorschau)

### Phase 2 - Should Have
**Epic 3:** Stories 10-13 (Erweiterte Filter & gespeicherte Filter)  
**Epic 5:** Stories 19-20 (Wochenplanung UX)  
**Epic 4:** Story 17 (Inline-Bewertung)

### Phase 3 - Nice to Have
**Epic 6:** Stories 21-23 (Datenqualität)  
**Epic 7:** Stories 24-25 (Technik)

---

## Legende

- **Offen** - Noch nicht begonnen
- **In Arbeit** - Aktuell in Entwicklung
- **Abgeschlossen** - Implementiert und getestet
- **Blockiert** - Wartet auf andere Story/Clarification

---

**Gesamt:** 25 Stories in 7 Epics
