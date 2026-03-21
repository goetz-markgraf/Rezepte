---
stepsCompleted: ["step-01-init", "step-02-discovery", "step-02b-vision", "step-02c-executive-summary", "step-03-success", "step-04-journeys", "step-07-project-type", "step-08-scoping", "step-09-functional", "step-10-nonfunctional", "step-11-polish"]
inputDocuments: ["docs/planning-artifacts/product-brief-Rezepte-2026-03-14.md"]
workflowType: 'prd'
briefCount: 1
researchCount: 0
brainstormingCount: 0
projectDocsCount: 0
classification:
  projectType: "web_app"
  domain: "general"
  complexity: "low"
  projectContext: "greenfield"
---

# Product Requirements Document - Rezepte

**Author:** Dragon
**Date:** 2026-03-15

## Executive Summary

**Rezepte** ist eine minimalistische Webanwendung zur Verwaltung von Familienrezepten im LAN. Die Lösung transformiert Wochenplanung von 20+ Minuten (Bücher durchblättern) zu 2 Minuten (filtern und planen). Zielgruppe: Zwei-Personen-Haushalt mit verstreuten Rezeptquellen, der aus Gewohnheit immer dieselben 5-10 Gerichte kocht, aber mehr Variation wünscht. Multi-Device-Zugriff (Handy, Tablet, Laptop) ohne Login kombiniert mit intelligenten Filtern für bewusste Essensplanung.

### Was macht dies besonders

**Datum-Feature als Planungstool:** "Geplant am"-Feld akzeptiert Zukunftsdaten - Wochenplanung ohne separates Feature. Filter "Nächste 7 Tage" zeigt geplante Gerichte sortiert.

**Variations-Fokus:** Filter "Länger nicht gemacht" + Beliebtheit-Ranking (3-5 Sterne) fördert Ausbruch aus Routine und Wiederentdeckung vergessener Favoriten.

**Zero-Overhead:** Kein Login, kein Cloud-Sync, keine Migration - sofort nutzbar. Fokus: Rezept-Repository, kein Feature-Bloat.

**LAN-only Simplicity:** Multi-Device ohne Komplexität - simultaner Zugriff mit last-write-wins.

## Projekt-Klassifizierung

- **Projekt-Typ:** Web App (responsive für Desktop, Tablet, Mobile)
- **Domain:** General (persönliche Produktivität, keine Compliance-Anforderungen)
- **Komplexität:** Low (Standard-Webanwendung)
- **Projekt-Kontext:** Greenfield (neues Produkt)

## Success Criteria

### User Success

**"Aha!"-Momente:**
- Wochenplanung in 2 Minuten: Filter anwenden, 5-7 Rezepte wählen, Zukunftsdaten setzen - fertig
- Wiederentdeckung: Vergessene Favoriten (3+ Monate alt) landen direkt im Wochenplan
- Sofortzugriff: Rezept auf Handy finden statt Bücher durchblättern

**Emotionaler Erfolg:** Mehr Freude an Wochenplanung durch intelligente Filter, die Inspiration statt Routine fördern.

### Business Success

**3-Monats-Meilenstein:**
- 30 Rezepte in Kategorie "Mittagessen"
- 5-10 Rezepte in anderen Kategorien (Brot, Party, Kuchen, Snacks)
- Regelmäßige wöchentliche Nutzung für Wochenplanung etabliert
- Gewohnheit entwickelt: Neue Rezept-Ideen werden spontan in die App eingetragen
- Merklich mehr Variation im Speiseplan durch aktive Nutzung der Filter

**ROI-Indikator:** Mehr Freude an Wochenplanung + messbar mehr Variation = Investment erfolgreich

### Technical Success

**Performance:**
- Filter-Anwendung < 1 Sekunde Reaktionszeit
- Schnelles Laden und Speichern von Rezepten

**Verfügbarkeit:**
- 24/7 Erreichbarkeit im LAN (passiv laufender Server)
- Simultaner Multi-Device-Zugriff funktioniert reibungslos
- Last-write-wins Konfliktlösung ausreichend (simultane Bearbeitung selten, noch seltener am selben Rezept)

**Datensicherheit:**
- Grundlegendes Backup/Datenverlust-Schutz für Rezept-Repository
- Keine Cloud-Sync-Komplexität erforderlich

### Measurable Outcomes

- Wochenplanung dauert ≤ 2 Minuten (vs. 20+ Minuten vorher)
- Filter werden aktiv genutzt (Kategorien, "Länger nicht gemacht", Beliebtheit)
- Repository wächst kontinuierlich (3-Monats-Ziel: 40-50 Rezepte gesamt)
- Multi-Device wird praktisch genutzt (Handy, Tablet, Laptop im Wechsel)

## Product Scope

### MVP - Minimum Viable Product

**Core Rezept-Management:**
- Rezept erstellen/bearbeiten/löschen
- Felder: Titel, Kategorien (Mittagessen, Brot, Party, Kuchen, Snacks), Anleitung, Zutaten, Datum, Beliebtheit (3-5 Sterne)
- Vollständige CRUD-Operationen für alle Nutzer (keine Berechtigungen)

**Filter & Suche:**
- Volltextsuche über Titel, Zutaten, Anleitung
- Filter nach Kategorien/Tags
- Filter "Länger nicht gemacht" (aufsteigend nach Datum)
- Filter "Nächste 7 Tage" (geplante Rezepte mit Zukunftsdatum)
- Beliebtheit-Filter (nach Sterne-Ranking)

**Datum-Feature:**
- "Zuletzt gemacht" akzeptiert Vergangenheits- UND Zukunftsdaten
- Manuell editierbar oder per Datepicker
- Ermöglicht Wochenplanung ohne separates Planungs-Feature

**Technisch:**
- Responsive Design (Desktop, Tablet, Mobile)
- LAN-Zugriff ohne Login/Authentifizierung
- Last-write-wins Konfliktlösung
- Multi-Device gleichzeitiger Zugriff

### Growth Features (Post-MVP)

**Erweiterte Filter & Organisation:**
- Gespeicherte Filter-Kombinationen für Schnellzugriff
- Erweiterte Sortier-Optionen

**Bilder:**
- Fotos zu Rezepten hinzufügen (optional)

**Einkaufsplanung:**
- Erweiterte strukturierte Zutatenliste
- Export für Einkaufsplanung

**Export/Import:**
- Rezept-Export in verschiedene Formate
- Bulk-Import von Rezepten

### Vision (Future)

Iterativer Ansatz basierend auf Nutzungserfahrung. Weitere Features ergeben sich organisch bei MVP-Erfolg. Fokus bleibt: Einfachheit, schneller Rezeptzugriff, intelligente Wochenplanung.

## User Journeys

### Journey 1: Wochenplanung am Mittwochabend

**Der Haushalt - Anna & Dragon**

Es ist Mittwochabend, 19:30 Uhr. Anna und Dragon sitzen auf dem Sofa, beide mit Tablet oder Laptop. Die wöchentliche Frage steht im Raum: "Was kochen wir nächste Woche?"

**Opening Scene:** Früher bedeutete das: Kochbücher vom Regal holen, Schnellhefter durchblättern, 20 Minuten später frustriert dieselben 5 Gerichte wählen wie immer. Heute öffnen beide die Rezepte-App.

**Rising Action:** Anna öffnet den gespeicherten Filter "Mittagessenplanung". Sofort sieht sie beliebte Gerichte - die am längsten nicht gemachten stehen oben. "Oh, Spaghetti Bolognese! Das hatten wir ja ewig nicht." Sie klickt ins Rezept, überfliegt die Zutaten. Dragon scrollt parallel durch "Noch nicht bewertet" - neue Rezepte, die sie ausprobieren wollten.

**Interaktion:** Für Spaghetti wählt Anna den Wochentag: "Donnerstag nächste Woche". Der Wochentag-Picker setzt automatisch das richtige Datum ins Feld "Geplant am". Kein Kalender-Fummelei, nur ein Klick. Dragon findet ein Pfannkuchen-Rezept: "Freitag passt!" Klick, gespeichert.

**Climax:** Nach 5 Minuten öffnen sie die Wochenvorschau: Eine übersichtliche Liste formatiert nach Wochentag:
- Do: Spaghetti Bolognese  
- Fr: Pfannkuchen
- Sa: Thai-Curry
- So: Pizza

**Resolution:** Anna kopiert die Liste per Copy/Paste in ihre externe Planungsliste. Fertig. 5 Minuten statt 20, 7 verschiedene Gerichte statt immer dieselben 5, und sie freuen sich schon auf das Thai-Curry, das sie seit 4 Monaten nicht mehr gemacht haben.

**Emotionaler Erfolg:** Mehr Vorfreude, weniger Routine, null Frustration.

---

### Journey 2: Sonntagabend - Neues Brotrezept entdeckt

**Der Haushalt - Dragon in der Küche**

Sonntagabend, 20:00 Uhr. Dragon steht am Küchentisch mit einem Kochbuch. Ein interessantes Brotrezept: "Dinkelvollkornbrot mit Walnüssen". Das will er festhalten.

**Opening Scene:** Früher hätte er einen Zettel genommen, irgendwo abgelegt, nie wiedergefunden. Heute: iPhone aus der Tasche, Homescreen-Shortcut "Neues Rezept" antippen - direkt in der App.

**Rising Action:** Dragon tippt den Titel: "Dinkel..." - sofort erscheint eine Inline-Vorschau: "Ähnliche Rezepte gefunden: Dinkelbrot (3★)". Er klappt es auf, checkt kurz - nein, anderes Rezept. Weiter.

**Interaktion:** Er tippt die Zutaten ins Feld, Markdown-Liste:
```
- 500g Dinkelvollkornmehl
- 100g Walnüsse
```
ENTER → automatisch nächste Zeile beginnt mit "- ". Smooth. Dann Zubereitung, ebenfalls mit Markdown. Kategorie-Auswahl: "Brot" - sichtbar, kein Dropdown-Gefummel.

**Validierung:** Er will speichern - Titel und Kategorie ausgefüllt. Zubereitung und Zutaten können auch leer bleiben (er kann sie später nachtragen). Bewertung und Datum lässt er ebenfalls leer. Die App akzeptiert es.

**Resolution:** Rezept gespeichert. 2 Minuten, Buch kann zurück ins Regal. Beim nächsten "Brot"-Filter wird es auftauchen, noch nicht bewertet, bereit zum Ausprobieren.

**Emotionaler Erfolg:** Null Barriere zwischen Idee und Repository. Spontanes Festhalten wird zur Gewohnheit.

---

### Journey 3: Beim Abendessen - Spontane Bewertung

**Der Haushalt - Anna & Dragon am Esstisch**

Donnerstagabend, 19:00 Uhr. Anna und Dragon essen Spaghetti Bolognese. Nach dem ersten Bissen: "Wow, das ist wirklich gut. Hatten wir das schon bewertet?"

**Opening Scene:** Früher: mentale Notiz, vergessen, wiederholen. Heute: Anna zückt das Handy während des Essens.

**Navigation:** App öffnen → "Heute gekocht"-Ansicht. Das heutige Gericht (Spaghetti Bolognese) ist hervorgehoben. Sie sieht auch gestern (Thai-Curry, 5★) und morgen (Pfannkuchen, noch nicht bewertet).

**Climax:** Direktes Bewerten ohne Edit-Mode: Sie tippt 5 Sterne. Fertig. Keine Formulare, kein Speichern-Button, einfach nur: Tap, bewertet.

**Alternative Action:** Dragon will eine Notiz hinzufügen: "Nächstes Mal mehr Knoblauch." Er wechselt in den Bearbeiten-Modus, fügt die Notiz in der Zubereitung hinzu, speichert.

**Resolution:** Beim nächsten Filter "Länger nicht gemacht" wird Spaghetti jetzt als 5-Sterne-Gericht auftauchen - klare Empfehlung für die Zukunft.

**Emotionaler Erfolg:** Bewertungen werden zur natürlichen Gewohnheit. Das System lernt ihre Vorlieben.

---

### Journey 4: Wartung & Fehlerszenarien

**Der Haushalt - Dragon als technischer Verwalter**

**Szenario A: Versehentliches Löschen**

Dragon ist im Bearbeiten-Modus eines Rezepts. Aus Versehen tippt er auf "Löschen". Die App zeigt sofort eine Sicherheitsabfrage:

"Rezept 'Spaghetti Bolognese' wirklich löschen? 
**Tipp:** Statt zu löschen, kannst du eine schlechte Bewertung (1★) vergeben. Dann wird es nicht mehr vorgeschlagen, aber bleibt im System."

[Abbrechen] [Schlecht bewerten] [Wirklich löschen]

Dragon wählt "Abbrechen". Krise abgewendet.

**Szenario B: Dubletten-Prüfung**

Nach 6 Monaten Nutzung: 60 Rezepte im System. Anna vermutet, dass sie "Pizza Margherita" doppelt angelegt haben.

Sie öffnet die Dubletten-Prüfung (spezielle Suche). Die App zeigt:
- "Pizza Margherita" (5★, zuletzt 2026-02-10)
- "Margherita Pizza" (nicht bewertet, zuletzt 2026-01-15)

Hohe Wahrscheinlichkeit: Duplikat. Anna öffnet beide, vergleicht, merged die Infos ins erste Rezept, löscht das zweite.

**Szenario C: Server Down**

Dragon startet seinen Laptop, will die Wochenplanung machen. App lädt nicht - Server ist down. Als technischer Admin checkt er den Server-Status auf seinem Raspberry Pi/NAS, startet den Service neu. 2 Minuten später läuft alles wieder.

## Web App Specific Requirements

### Project-Type Overview

**Rezepte** ist eine responsive Web-Anwendung für LAN-Deployment. Fokus: einfache Technologie-Wahl, unkompliziertes Docker-Deployment.

### Technical Architecture Considerations

**Application Architecture:**
- **SPA vs MPA:** Flexible Technologie-Wahl basierend auf einfachster Implementation
- **Deployment:** Docker-Image für Raspberry Pi/NAS
- **Konfliktlösung:** Last-write-wins ausreichend (kein Real-time erforderlich)
- **Infrastruktur:** LAN-only, keine Cloud, keine externe Erreichbarkeit

**Browser Support:**
- Alle aktuellen Browser (Chrome, Firefox, Safari, Edge)
- Mobile Browser (iOS Safari, Chrome Mobile)
- Responsive Design für Desktop, Tablet, Mobile

**SEO & Discovery:**
- Kein SEO erforderlich (LAN-only)
- Direkter Zugriff via IP/Hostname

**Accessibility:**
- WCAG 2.1 Level A Konformität
- Tastaturnavigation für Kernfunktionen
- Lesbare Schriftgrößen und Kontraste
- Semantisches HTML für Screenreader-Kompatibilität

### Implementation Considerations

**Deployment:**
- Single Docker-Image enthält Frontend + Backend (wenn nötig)
- Einfache Installation via `docker run` Kommando
- Persistente Datenspeicherung via Volume-Mapping
- Kein komplexes Setup erforderlich

**Performance:**
- Filter-Anwendung < 1 Sekunde Reaktionszeit
- Schnelles Laden der Rezeptliste
- Optimierte Assets für mobile Geräte

**Multi-Device Support:**
- Simultaner Zugriff von mehreren Geräten
- Last-write-wins für Konflikte
- Keine Session-Verwaltung oder Login erforderlich

## Project Scoping & Phased Development

### MVP Strategy & Philosophy

**MVP Approach:** Problem-Solving MVP

Kernproblem lösen: Wochenplanung von 20+ Minuten auf 2 Minuten reduzieren und mehr Variation in den Speiseplan bringen. MVP liefert sofort nutzbaren Wert.

**Resource Requirements:** 
- 1 Full-Stack Developer mit KI-Unterstützung
- Technologie-Stack: Einfachheit vor Perfektion
- Deployment-Target: Docker-Container für einfache Installation

**Primary Risk:** Zeit bis zum ersten nutzbaren Prototyp

Strategie: Schneller iterativer Ansatz mit frühzeitigem LAN-Deployment für sofortiges Feedback.

### MVP Feature Set (Phase 1)

**Core User Journeys Supported:**
- **Journey 1:** Wochenplanung in 2 Minuten (Filter, Auswahl, Zukunftsdaten setzen)
- **Journey 2:** Neues Rezept schnell erfassen (mobile-optimiert)
- **Journey 3:** Spontane Bewertung während des Essens

**Must-Have Capabilities:**

**Core Rezept-Management:**
- CRUD-Operationen (Create, Read, Update, Delete mit Sicherheitsabfrage)
- Felder: Titel (Pflicht), Kategorien (Pflicht), Zutaten (optional), Anleitung (optional), Bewertung (optional), Datum "Geplant am" (optional)
- Markdown-Support für Zutaten und Zubereitung

**Filter & Suche:**
- Volltextsuche über Titel, Zutaten, Anleitung
- Filter nach Kategorien (Mittagessen, Brot, Party, Kuchen, Snacks)
- Filter "Länger nicht gemacht" (aufsteigend nach Datum)
- Filter "Nächste 7 Tage" (geplante Rezepte mit Zukunftsdatum)
- Beliebtheit-Filter (3-5 Sterne)

**Datum & Planning Feature:**
- "Geplant am" akzeptiert Zukunftsdaten
- Wochentag-Picker für intuitive Planung
- Wochenvorschau formatiert nach Wochentag

**Technisch:**
- Responsive Design (Desktop, Tablet, Mobile)
- LAN-Zugriff ohne Login
- Last-write-wins Konfliktlösung
- Docker-Deployment

### Post-MVP Features

**Phase 2 (Growth Features):**

**Erweiterte Organisation:**
- Gespeicherte Filter-Kombinationen für Schnellzugriff
- Erweiterte Sortier-Optionen
- Dubletten-Prüfung und Merge-Funktionalität

**Verbesserte UX:**
- Bilder zu Rezepten hinzufügen (optional)
- "Heute gekocht"-Ansicht mit Highlight
- Inline-Bewertung ohne Edit-Mode
- Duplikaterkennung während Titeleingabe

**Phase 3 (Vision/Expansion):**

**Erweiterte Planung:**
- Strukturierte Zutatenliste
- Export für Einkaufsplanung
- Rezept-Export in verschiedene Formate

**Import/Migration:**
- Bulk-Import von Rezepten
- Migration aus anderen Formaten

### Risk Mitigation Strategy

**Technical Risks:**
- **Risiko:** Zu komplexe Technologie-Wahl verzögert ersten Prototyp
- **Mitigation:** Pragmatische Tech-Stack-Wahl, bewährte Technologien bevorzugen
- **Mitigation:** Docker-first Ansatz für einfaches Deployment und Testing

**Market Risks:**
- **Risiko:** Features passen nicht zu tatsächlichem Nutzungsverhalten
- **Mitigation:** Frühes Deployment im LAN für sofortiges Real-World-Feedback
- **Mitigation:** Iterative Entwicklung basierend auf tatsächlicher Nutzung

**Resource Risks:**
- **Risiko:** Zeit bis zum ersten nutzbaren Prototyp (Hauptrisiko)
- **Mitigation:** Strikte MVP-Disziplin - nur essenzielle Features in Phase 1
- **Mitigation:** KI-Unterstützung für schnellere Entwicklung
- **Mitigation:** Early-Exit-Strategie: Minimal funktionsfähige Version nach 2 Wochen im Einsatz

**De-Risking Approach:**
- Woche 1-2: Minimal funktionsfähige Version (CRUD + einfache Liste)
- Woche 3-4: Filter und Datum-Feature
- Woche 5+: Iterative Verbesserungen basierend auf praktischer Nutzung

## Functional Requirements

### Rezept-Management

- **FR1:** Benutzer können ein neues Rezept erstellen mit Titel (Pflicht) und Kategorie (Pflicht)
- **FR2:** Benutzer können optionale Rezept-Details hinzufügen (Zutaten, Anleitung)
- **FR3:** Benutzer können bestehende Rezepte bearbeiten
- **FR4:** Benutzer können Rezepte löschen mit Sicherheitsabfrage
- **FR5:** Benutzer können Rezepte mit Markdown-Formatierung für Zutaten und Anleitung erfassen
- **FR6:** Benutzer können Rezepte in vordefinierte Kategorien einordnen (Mittagessen, Brot, Party, Kuchen, Snacks)

### Rezept-Bewertung & Planung

- **FR7:** Benutzer können Rezepte mit 3-5 Sternen bewerten
- **FR8:** Benutzer können für Rezepte ein "Geplant am"-Datum setzen (Vergangenheit oder Zukunft)
- **FR9:** Benutzer können das "Geplant am"-Datum mit einem Wochentag-Picker setzen
- **FR10:** System setzt automatisch korrektes Datum basierend auf gewähltem Wochentag

### Suche & Filterung

- **FR11:** Benutzer können Volltextsuche über Titel, Zutaten und Anleitung durchführen
- **FR12:** Benutzer können Rezepte nach Kategorien filtern
- **FR13:** Benutzer können Rezepte nach Bewertung filtern (Beliebtheit)
- **FR14:** Benutzer können "Länger nicht gemacht" Filter anwenden (aufsteigend nach Datum sortiert)
- **FR15:** Benutzer können "Nächste 7 Tage" Filter anwenden für geplante Rezepte
- **FR16:** System schließt negativ bewertete Rezepte (1-2 Sterne) aus Vorschlägen aus

### Wochenplanung & Übersicht

- **FR17:** Benutzer können Wochenvorschau für geplante Rezepte anzeigen
- **FR18:** System formatiert Wochenvorschau nach Wochentagen (Do: Gericht, Fr: Gericht)
- **FR19:** Benutzer können Wochenvorschau-Daten kopieren für externe Nutzung

### Multi-Device & Datenspeicherung

- **FR20:** System ermöglicht simultanen Zugriff von mehreren Geräten im LAN
- **FR21:** System löst Konflikte mit Last-write-wins Strategie
- **FR22:** System speichert alle Rezeptdaten persistent
- **FR23:** System benötigt keine Benutzer-Authentifizierung oder Login

### Responsive Zugriff

- **FR24:** Benutzer können die App von Desktop-Browsern nutzen
- **FR25:** Benutzer können die App von Tablet-Browsern nutzen
- **FR26:** Benutzer können die App von Mobile-Browsern nutzen
- **FR27:** System passt UI automatisch an Bildschirmgröße an (responsive)

## Non-Functional Requirements

### Performance

- **NFR-P1:** Filter-Anwendung liefert Ergebnisse in < 1 Sekunde
- **NFR-P2:** Rezeptliste lädt in < 2 Sekunden beim initialen Seitenaufruf
- **NFR-P3:** Rezept-Speicherung erfolgt in < 500ms
- **NFR-P4:** Volltextsuche liefert Ergebnisse in < 1 Sekunde bei bis zu 200 Rezepten

### Accessibility

- **NFR-A1:** Anwendung erfüllt WCAG 2.1 Level A Konformität
- **NFR-A2:** Alle Kernfunktionen (CRUD, Filter, Suche) sind per Tastatur navigierbar
- **NFR-A3:** Farbkontraste erfüllen WCAG Level A Mindestanforderungen (4.5:1 für normalen Text)
- **NFR-A4:** Semantisches HTML ermöglicht Screenreader-Navigation

### Reliability & Availability

- **NFR-R1:** System ist 24/7 im LAN erreichbar (passive Verfügbarkeit)
- **NFR-R2:** Datenverlust-Risiko wird durch grundlegendes Backup minimiert
- **NFR-R3:** System startet automatisch nach Server-Neustart (Docker restart policy)
- **NFR-R4:** Simultaner Multi-Device-Zugriff funktioniert ohne Datenverlust (last-write-wins)

### Deployment & Operations

- **NFR-D1:** Anwendung wird als einzelnes Docker-Image ausgeliefert
- **NFR-D2:** Installation erfolgt mit einem einzelnen `docker run` Kommando
- **NFR-D3:** Persistente Daten werden via Volume-Mapping gespeichert
- **NFR-D4:** Kein komplexes Setup oder Konfiguration erforderlich

### Browser Compatibility

- **NFR-B1:** Volle Funktionalität in aktuellen Versionen von Chrome, Firefox, Safari, Edge
- **NFR-B2:** Mobile Browser (iOS Safari, Chrome Mobile) werden vollständig unterstützt
- **NFR-B3:** Responsive Design funktioniert auf Bildschirmgrößen von 320px bis 2560px Breite
