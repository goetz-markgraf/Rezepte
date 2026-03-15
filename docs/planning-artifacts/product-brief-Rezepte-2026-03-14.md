---
stepsCompleted: [1, 2, 3, 4, 5]
inputDocuments: ["d_old/produkt_old.md"]
date: 2026-03-14
author: Dragon
---

# Product Brief: Rezepte

## Executive Summary

**Rezepte** ist eine einfache Webanwendung zur Verwaltung von Familienrezepten im lokalen Netzwerk (LAN). Die Lösung konsolidiert verstreute Rezeptquellen (Ordner, Bücher, Zettel) in einem zentralen, schnell durchsuchbaren Repository. Der Fokus liegt auf schneller Inspiration bei der Wochenplanung durch Filter wie "Länger nicht gemacht", Kategorien und Volltextsuche. Ohne Login-Zwang können beide Partner jederzeit auf alle Rezepte zugreifen, sie bearbeiten und das letzte Zubereitungsdatum tracken - um bewusst mehr Variation in den Speiseplan zu bringen.

---

## Core Vision

### Problem Statement

Familienrezepte sind aktuell über verschiedene Orte verstreut (Ordner, Kochbücher, handgeschriebene Zettel). Bei der wöchentlichen Essensplanung (typischerweise Mittwoch/Donnerstag) fehlt eine schnelle Übersicht über verfügbare Rezepte. Das Durchblättern von Büchern und Listen ist zu mühsam im Moment der Planung. Resultat: Das Paar greift aus Erinnerung immer auf dieselben 5-10 Gerichte und Brotrezepte zurück, obwohl mehr Rezepte vorhanden sind.

### Problem Impact

- **Zeitverschwendung**: Suche nach Rezepten in verschiedenen Quellen während der Wochenplanung
- **Fehlende Variation**: Immer dieselben Gerichte, weil keine Übersicht über Alternativen
- **Ungenutzte Ressourcen**: Vorhandene Rezepte werden vergessen oder ignoriert
- **Frustration**: Mangelnde Inspiration führt zu Routine statt Abwechslung

### Why Existing Solutions Fall Short

Bestehende Lösungen (Rezept-Apps, digitale Notiz-Tools, Cloud-Services) bieten zu viel Overhead für den einfachen Use Case:
- **Login/Authentifizierung**: Unnötige Barriere für Zwei-Personen-Haushalt im LAN
- **Cloud-Sync & Features**: Nicht benötigt und lenkt ab
- **Fremde Rezepte**: Plattformen wollen eigene Inhalte pushen
- **Keine "Länger nicht gemacht"-Funktion**: Fokus auf Bewertungen/Favoriten statt zeitbasierter Variation
- **Hoher Migrationsaufwand**: Einpflegen aller Rezepte ohne direkten Mehrwert

### Proposed Solution

Eine minimalistische Webanwendung im LAN, die als zentrales Rezept-Repository dient. Fokus auf:
- **Schnelle Übersicht**: Alle Rezepte alphabetisch sortiert, durchsuchbar
- **Intelligente Filter**: Kategorien (Mittagessen, Brot, Party, etc.) und "Länger nicht gemacht"
- **Datum-Tracking**: Manuell oder per Datepicker das letzte Zubereitungsdatum eintragen
- **Keine Barrieren**: Kein Login, direkter Zugriff für beide Partner im Haushalt
- **Einfachheit**: Nur Rezept-Verwaltung, keine Essensplan- oder Einkaufslistenfunktionen (werden extern geführt)

### Key Differentiators

- **Hyper-fokussiert**: Nur Rezept-Repository, kein Feature-Bloat
- **LAN-only**: Keine Cloud, keine Sync-Komplexität, keine Sicherheitsbedenken
- **"Länger nicht gemacht"-Filter**: Einzigartige Funktion für bewusste Variation
- **Zero-Overhead**: Kein Login, keine Migration erzwungen, sofort nutzbar
- **Built for Two**: Perfekt für Zwei-Personen-Haushalte ohne Rollen/Berechtigungen

## Target Users

### Primary Users

**Der Haushalt (gemeinsamer Nutzer)**

**Kontext:**
Ein Zwei-Personen-Haushalt ohne Login-Unterscheidung. Beide Partner nutzen die Anwendung gleichberechtigt über verschiedene Geräte (Handy, Tablet, Laptop) im lokalen Netzwerk.

**Nutzungsverhalten:**
- **Wochenplanung (Mittwoch/Donnerstag)**: Durchsuchen der Rezeptliste mit Filtern (Kategorien wie Brot, Mittagessen, Party), Nutzung des "Länger nicht gemacht"-Filters für Inspiration. Gespeicherte Filter ermöglichen schnellen Zugriff auf relevante Rezeptkategorien.
- **Rezept-Eingabe**: Spontanes oder geplantes Eintragen neuer Rezepte (abends), meist von Büchern, Websites oder Zetteln übertragen
- **Datum-Tracking**: Aktualisierung des "Zuletzt gemacht"-Datums während der Wochenplanung, nicht während des Kochens

**Geräte & Kontext:**
- Multi-Device: Mobil (Handy/Tablet) und Desktop (Laptop)
- Alle Geräte im lokalen LAN
- App muss sowohl am Desktop als auch mobil bequem nutzbar sein (responsive Design)

**Problem-Erfahrung:**
Rezepte aktuell in Ordnern, Büchern und auf Zetteln verstreut. Während der Wochenplanung keine schnelle Übersicht, was zu Routine und fehlender Variation führt. Blättern in Büchern zu mühsam im Planungsmoment.

**Erfolgs-Vision:**
- Schneller Zugriff auf alle Rezepte mit intelligenten Filtern
- Inspiration durch "Länger nicht gemacht"-Feature
- Mehr Variation bei Gerichten und Brot
- Einfache Rezept-Eingabe ohne Hürden
- Reibungsloser Zugriff von allen Geräten im Haushalt

### Secondary Users

N/A - Die Anwendung ist ausschließlich für den Zwei-Personen-Haushalt konzipiert ohne externe Nutzer.

### User Journey

**Szenario 1: Wochenplanung (Mittwoch/Donnerstag)**

1. **Start**: Öffnen der App auf Tablet oder Laptop
2. **Filter anwenden**: Nutzung gespeicherter Filter (z.B. "Mittagessen") oder "Länger nicht gemacht"
3. **Inspiration sammeln**: Durchsehen der gefilterten Rezeptliste
4. **Auswahl treffen**: Entscheidung für Rezepte der kommenden Woche
5. **Datum aktualisieren**: Markieren der ausgewählten Rezepte mit aktuellem Datum (manuell oder Datepicker)
6. **Externe Planung**: Notieren des Essensplans und der Einkaufsliste außerhalb der App

**Szenario 2: Neues Rezept eintragen**

1. **Auslöser**: Neues Rezept gefunden (Buch, Website, Zettel) - spontan oder abends geplant
2. **Gerät wählen**: Je nach Situation Laptop (bequemes Tippen) oder Mobil
3. **Rezept erstellen**: Titel, Kategorien, Zutaten, Anleitung eingeben
4. **Speichern**: Rezept wird sofort in der Liste verfügbar

**Szenario 3: Langfristige Nutzung**

- **Kein Einsatz beim Kochen**: Die App wird nicht in der Küche während des Kochens genutzt (dafür andere Quellen)
- **Regelmäßige Wochenplanung**: Die App wird zum festen Bestandteil der wöchentlichen Essensplanung
- **Wachsendes Repository**: Über Zeit werden mehr Rezepte aus Büchern/Zetteln migriert
- **Mehr Variation**: Durch "Länger nicht gemacht"-Filter bewusste Diversifizierung des Speiseplans

## Success Metrics

N/A - Übersprungen auf Nutzerwunsch

## MVP Scope

### Core Features

**Rezept-Management:**
- Rezept erstellen mit folgenden Feldern:
  - Titel (Pflichtfeld)
  - Kategorien (mehrere möglich: Mittagessen, Party, Kuchen, Brot, Snacks)
  - Anleitung (Freitext)
  - Zutaten (Freitext)
  - "Wann zuletzt gemacht" (Datum, manuell oder per Datepicker aktualisierbar)
- Rezept bearbeiten (alle Nutzer können alle Rezepte bearbeiten)
- Rezept löschen (alle Nutzer können alle Rezepte löschen)

**Übersicht & Navigation:**
- Listendarstellung aller Rezepte
- Standardsortierung: Alphabetisch nach Titel
- Detailansicht für einzelnes Rezept

**Suche & Filter:**
- Volltextsuche über Titel, Zutaten und Anleitung
- Filter nach Kategorien/Tags
- Filter "Länger nicht gemacht" (Rezepte nach Datum aufsteigend sortiert)
- **Gespeicherte Filter**: Schnellzugriff auf häufig genutzte Filter (z.B. "Brot", "Mittagessen", "Party")

**Technische Anforderungen:**
- Webanwendung erreichbar im LAN
- Responsive Design für Desktop (Laptop) und Mobile (Handy/Tablet)
- Kein Login/Authentifizierung erforderlich
- Alle Nutzer haben volle Berechtigungen (erstellen, bearbeiten, löschen)

### Out of Scope for MVP

**Explizit NICHT enthalten:**
- Benutzer-Login/Authentifizierung oder Rollen
- Bewertungen oder Favoriten-Funktion
- Bilder/Fotos von Rezepten
- PDF-Export oder Druck-Optimierung
- Automatische Mengenumrechnung/Portionsskalierung
- Cloud-Synchronisation oder externes Backup
- Automatischer Import/Export (Copy/Paste der Inhalte genügt)
- Essensplan-Erstellung (wird extern geführt)
- Einkaufsliste-Generierung (wird extern geführt)
- Integrierte Kochansicht (App wird nicht während des Kochens genutzt)

### MVP Success Criteria

**Praktische Nutzungs-Validierung:**
- Die App wird regelmäßig (wöchentlich) für die Wochenplanung genutzt
- Rezepte werden aktiv eingetragen und migriert
- Filter und Suche werden praktisch verwendet
- Multi-Device-Zugriff funktioniert reibungslos im LAN

### Future Vision

**Iterativer Ansatz:**
Die App bleibt bewusst minimalistisch fokussiert. Zukünftige Features werden nicht im Voraus geplant, sondern basierend auf praktischer Nutzungserfahrung evaluiert und bei Bedarf hinzugefügt. Der Fokus liegt darauf, das MVP zu nutzen und daraus zu lernen, was tatsächlich gebraucht wird.
