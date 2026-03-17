---
stepsCompleted: ["step-01-init", "step-02-discovery", "step-02b-vision", "step-02c-executive-summary", "step-03-success", "step-04-journeys"]
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

**Rezepte** ist eine minimalistische Webanwendung zur Verwaltung von Familienrezepten im lokalen Netzwerk (LAN). Die Lösung transformiert Wochenplanung von mühsam (Bücher und Schnellhefter durchblättern) zu effizient (2 Minuten filtern und planen). Zielgruppe ist ein Zwei-Personen-Haushalt, der aktuell unter verstreuten Rezeptquellen leidet und immer auf dieselben 5-10 Gerichte zurückgreift, obwohl mehr Variation gewünscht ist. Die Anwendung ermöglicht schnellen Multi-Device-Zugriff (Handy, Tablet, Laptop) im LAN ohne Login-Barrieren, kombiniert mit intelligenten Filtern für bewusste Essensplanung und mehr Abwechslung im Speiseplan.

### Was macht dies besonders

**Cleveres Datum-Feature:** Das "Zuletzt gemacht"-Feld akzeptiert Zukunftsdaten und wird dadurch automatisch zum Planungstool - ohne separates Wochenplan-Feature. Ein Filter "Nächste 7 Tage" zeigt geplante Gerichte nach Datum sortiert.

**Variations-Fokus:** Filter "Länger nicht gemacht" kombiniert mit Beliebtheit-Ranking (3-5 Sterne) hilft aktiv, aus der Routine auszubrechen und vergessene Favoriten wiederzuentdecken.

**Zero-Overhead:** Kein Login, kein Cloud-Sync, keine erzwungene Migration - sofort nutzbar. Hyper-fokussiert auf Rezept-Repository, kein Feature-Bloat (keine Einkaufslisten, keine Social Features).

**LAN-only Simplicity:** Multi-Device ohne Komplexität - simultaner Zugriff von allen Geräten im Haushalt mit last-write-wins statt Over-Engineering.

## Projekt-Klassifizierung

- **Projekt-Typ:** Web App (responsive für Desktop, Tablet, Mobile)
- **Domain:** General (persönliche Produktivität, keine Compliance-Anforderungen)
- **Komplexität:** Low (Standard-Webanwendung)
- **Projekt-Kontext:** Greenfield (neues Produkt)

## Success Criteria

### User Success

**"Aha!"-Momente:**
- Wochenplanung in 2 Minuten: Filter anwenden, 5-7 Rezepte auswählen, Zukunftsdaten eintragen - fertig
- "Länger nicht gemacht"-Wiederentdeckung: Vergessene Favoriten (3+ Monate nicht gemacht) landen direkt im Wochenplan
- Sofortiger Rezeptzugriff: Rezept auf dem Handy finden statt Bücher durchblättern

**Emotionaler Erfolg:**
Mehr Freude an der Wochenplanung durch einfachen Zugang und intelligente Filter, die Inspiration statt Routine fördern.

### Business Success

**3-Monats-Meilenstein:**
- 30 Rezepte in Kategorie "Mittagessen"
- 5-10 Rezepte in anderen Kategorien (Brot, Party, Kuchen, Snacks)
- Regelmäßige wöchentliche Nutzung für Wochenplanung etabliert
- Gewohnheit entwickelt: Neue Rezept-Ideen werden spontan in die App eingetragen
- Merklich mehr Variation im Speiseplan durch aktive Nutzung der Filter

**ROI-Indikator:**
Mehr Freude an Wochenplanung + messbar mehr Variation im Speiseplan = Investment hat sich gelohnt

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

Iterativer Ansatz basierend auf praktischer Nutzungserfahrung. Wenn der MVP erfolgreich ist, werden sich weitere sinnvolle Features organisch ergeben. Fokus bleibt auf Einfachheit und dem Kern-Use-Case: Schneller Rezeptzugriff und intelligente Wochenplanung.

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

### Journey Requirements Summary

**Capabilities, die aus den Journeys hervorgehen:**

**Core Recipe Management:**
- CRUD-Operationen (Create, Read, Update, Delete mit Sicherheitsabfrage)
- Felder: Titel (Pflicht), Kategorie (Pflicht), Zutaten (optional), Zubereitung (optional), Bewertung (optional), Datum "Geplant am" (optional)
- Markdown-Support für mehrzeilige Felder mit Smart-Listen (auto "- " nach ENTER)
- Duplikaterkennung während Titeleingabe (inline, aufklappbar)
- Dubletten-Prüfung (spezielle Suche für wahrscheinliche Duplikate)

**Filter & Suche:**
- Gespeicherte Filter mit Namen (z.B. "Mittagessenplanung")
- Volltextsuche über alle Felder
- Filter: Kategorien, "Länger nicht gemacht", "Noch nicht bewertet", "Beliebte Gerichte"
- Sortierung: "Noch nie gemacht" erscheinen oben (außer negativ bewertet)
- Filter "Nächste 7 Tage" (Wochenvorschau formatiert nach Wochentag)

**Datum & Bewertung:**
- Wochentag-Picker für intuitive Zukunftsplanung (setzt automatisch richtiges Datum)
- "Heute gekocht"-Ansicht mit hervorgehobenem heutigem Gericht, Navigation zu gestern/morgen
- Inline-Bewertung (ohne Edit-Mode, direkt in Bewertungs-Ansicht)
- 1-5 Sterne-System, negativ bewertete Gerichte werden ausgeschlossen

**Mobile Optimierung:**
- Deep Links / Lesezeichen zu wichtigen Ansichten (z.B. "Neues Rezept"-Seite)
- iPhone-optimiert, Android-kompatibel
- Responsive für alle Geräte (Desktop/Tablet/Mobile)
- Schnelle Navigation, keine unnötigen Seitenwechsel

**Wochenvorschau & Export:**
- Liste formatiert nach Wochentag (Do: Gericht, Fr: Gericht)
- Copy/Paste-freundlich für externe Nutzung
- Übersichtliche Darstellung geplanter Rezepte

**Sicherheit & Wartung:**
- Löschen-Sicherheitsabfrage mit Alternative "Schlecht bewerten"
- Grundlegendes Backup/Recovery (technisch selbst verwaltbar)
- Last-write-wins für simultane Zugriffe
