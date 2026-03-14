# Produktdokument: Familien-Rezeptverwaltung

## Vision
Eine einfache Webanwendung zur Verwaltung von Familienrezepten im lokalen Netzwerk (LAN), ohne Login-Zwang.

## Zielgruppe
Familienmitglieder im gemeinsamen Haushalt mit Zugriff zum LAN.

## Kernfunktionen

### Rezept-Daten
Jedes Rezept enthält:
- **Titel** (Pflichtfeld)
- **Kategorien** (mehrere möglich, z.B. Mittagessen, Party, Kuchen, Brot, Snacks)
- **Anleitung** (Freitext)
- **Zutaten** (Freitext)
- **Wann zuletzt gemacht** (Datum, manuell oder per Klick aktualisierbar)

### Berechtigungen
- Kein Login erforderlich
- Alle Nutzer können alle Rezepte erstellen, bearbeiten und löschen

### Suche und Filter
- Volltextsuche über Titel, Zutaten und Anleitung
- Filter nach Kategorien/Tags
- Filter "Lange nicht gemacht" (Rezepte nach Datum aufsteigend)

### Darstellung
- Standardsortierung: Alphabetisch nach Titel
- Übersichtliche Listendarstellung aller Rezepte
- Detailansicht für einzelnes Rezept

### Technische Anforderungen
- Webanwendung, erreichbar im LAN
- Keine Bilder/Fotos
- Kein automatischer Export/Import (Copy/Paste genügt)
- Keine Portionsskalierung

## Nicht im Scope
- Benutzer-Login/Authentifizierung
- Bewertungen oder Favoriten
- Bilder/Fotos
- PDF-Export
- Automatische Mengenumrechnung
- Cloud-Synchronisation
