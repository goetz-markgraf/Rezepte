# Story 4: Rezept-Detailansicht

**Epic:** Rezept-Verwaltung (Grundlegendes CRUD)
**Priorität:** MVP - Phase 1
**Status:** Offen

---

## 1. Story-Satz

**Als** Benutzer möchte ich **ein einzelnes Rezept in einer vollständigen Detailansicht lesen**, damit ich **alle Informationen zu einem Rezept übersichtlich auf einen Blick sehe und von dort direkt Aktionen wie Bearbeiten oder Löschen ausführen kann**.

---

## 2. Geschäftsbezogene Details

### Kontext

Die Detailansicht ist die zentrale Anlaufstelle für ein einzelnes Rezept. In der Küche oder beim Wochenplanen braucht der Nutzer schnellen, übersichtlichen Zugriff auf Zutaten und Anleitung - ohne Ablenkung durch Bearbeitungsformulare. Die bisherige rudimentäre Ansicht (nach dem Erstellen oder Bearbeiten) war nur ein Platzhalter und erfüllt nicht die Anforderungen an Lesbarkeit und Nutzbarkeit im Alltag.

Die Detailansicht ist außerdem der zentrale Ausgangspunkt für alle weiteren Aktionen auf einem Rezept: Bearbeiten, Löschen, und später Bewerten sowie Planungsdatum setzen.

### Nutzergruppe

- Beide Partner des Haushalts (gleiche Berechtigungen, kein Rollenmodell)
- LAN-Zugriff über verschiedene Geräte (Desktop, Tablet, Handy)
- Häufig in der Küche auf dem Handy verwendet: lesbare Schriftgrößen, übersichtliche Struktur

### Business-Value

- Ermöglicht die tatsächliche Nutzung der gespeicherten Rezepte beim Kochen
- DeepLink-fähige URL (`/recipes/{id}`) ermöglicht direktes Teilen und Bookmarken einzelner Rezepte
- Klarer Ausgangspunkt für alle Folge-Aktionen (Bearbeiten, Löschen) - andere Stories (03, 02) bauen darauf auf

### Edge Cases

- **Rezept nicht vorhanden (ungültige ID):** HTTP 404-Antwort mit verständlicher Fehlermeldung; kein Absturz der Anwendung
- **Felder ohne Inhalt (Zutaten oder Anleitung leer):** Die entsprechenden Abschnitte werden nicht angezeigt, kein leerer Platzhalter
- **Sehr langer Text:** Zutaten und Anleitung scrollen innerhalb der Seite; kein horizontales Scrollen
- **Direktaufruf per URL:** Seite funktioniert auch ohne Navigation über die Liste (DeepLink)
- **Erfolgs-Flash nach Bearbeiten:** Nach dem Speichern von Änderungen (Story 02) wird auf der Detailansicht eine Erfolgsmeldung angezeigt

---

## 3. Akzeptanzkriterien

### Funktionale Kriterien

- [ ] **K1: Alle Rezept-Felder werden angezeigt**
  - Titel als Seitenüberschrift (H1)
  - Kategorien als Tags oder Labels unterhalb des Titels
  - Abschnitt "Zutaten" mit dem gespeicherten Text (nur wenn Inhalt vorhanden)
  - Abschnitt "Anleitung" mit dem gespeicherten Text (nur wenn Inhalt vorhanden)
  - Metainformationen: Erstellungsdatum und Datum der letzten Bearbeitung

- [ ] **K2: Aktions-Schaltflächen vorhanden**
  - "Bearbeiten"-Schaltfläche führt zum Bearbeitungsformular des Rezepts (`/recipes/{id}/edit`)
  - "Löschen"-Schaltfläche führt zur Sicherheitsabfrage (`/recipes/{id}/confirm-delete`)
  - "Zurück zur Übersicht"-Link führt zur Rezeptliste (`/`)

- [ ] **K3: DeepLink-URL funktioniert**
  - Jedes Rezept hat eine stabile, direkt aufrufbare URL: `/recipes/{id}`
  - Die Seite lädt korrekt auch ohne vorherige Navigation über die Liste

- [ ] **K4: Fehlerbehandlung bei nicht vorhandener ID**
  - Aufruf einer ungültigen Rezept-ID liefert eine 404-Seite
  - Die Fehlermeldung ist verständlich und enthält einen Link zurück zur Liste

- [ ] **K5: Erfolgs-Flash nach Bearbeiten**
  - Nach einer erfolgreichen Bearbeitung (Weiterleitung von Story 02) wird eine Erfolgsmeldung angezeigt ("Rezept erfolgreich aktualisiert")
  - Die Meldung verschwindet nach kurzer Zeit oder ist wegklickbar (bei JavaScript-Unterstützung)

### Nicht-funktionale Kriterien

- [ ] **K6: Performance**
  - Seite lädt in < 500ms
  - Keine unnötigen Datenbank-Abfragen (eine Abfrage pro Seitenaufruf)

- [ ] **K7: Barrierefreiheit**
  - Semantische HTML-Struktur: `<article>`, `<header>`, `<section>`, `<footer>` korrekt genutzt
  - Überschriften-Hierarchie korrekt (H1 für Titel, H2 für Abschnitte)
  - Schaltflächen und Links haben aussagekräftige Beschriftungen
  - Tastatur-Navigation funktioniert vollständig
  - WCAG 2.1 Level A konform

---

## 4. Technische Planung

### Datenmodell

Keine Änderungen am Datenbankschema erforderlich. Die bestehende `recipes`-Tabelle enthält alle notwendigen Felder:

| Feld | Typ | Angezeigt als |
|------|-----|---------------|
| id | INTEGER | URL-Parameter |
| title | TEXT | H1-Überschrift |
| categories | JSON-Array | Kategorie-Tags |
| ingredients | TEXT (optional) | Abschnitt "Zutaten" |
| instructions | TEXT (optional) | Abschnitt "Anleitung" |
| created_at | TIMESTAMP | Meta: "Erstellt am" |
| updated_at | TIMESTAMP | Meta: "Zuletzt bearbeitet" |

### UI/UX-Spezifikation

**Seitenstruktur:**
- Header: Rezepttitel (H1) + Kategorie-Tags
- Optionaler Abschnitt "Zutaten" (H2) mit Pre-formatiertem Text
- Optionaler Abschnitt "Anleitung" (H2) mit Pre-formatiertem Text
- Footer: Metainformationen (Erstellungs- und Bearbeitungsdatum) + Aktions-Schaltflächen

**Aktions-Schaltflächen:**
- "Bearbeiten" (primäre Schaltfläche, positiv)
- "Zurück zur Übersicht" (sekundäre Schaltfläche, neutral)
- "Löschen" (destruktive Schaltfläche, rot/Warnung)

**Responsive:**
- Desktop und Tablet: Schaltflächen nebeneinander in einer Reihe
- Mobile (Handy in der Küche): Schaltflächen untereinander, volle Breite, gut tippbar

**Flash-Meldung:**
- Erfolgsanzeige nach Bearbeiten: grüner Banner oben auf der Seite
- Wird via Query-Parameter `?success=true` ausgelöst (funktioniert ohne JS)

---

## 5. Nicht-funktionale Anforderungen

### Performance
- Seite lädt ohne sichtbare Verzögerung (< 500ms)
- Eine einzelne Datenbankabfrage pro Seitenaufruf

### Browser-Support
- Aktuelle Chrome, Firefox, Safari, Edge Versionen
- iOS Safari und Android Chrome (letzte 2 Versionen)
- Funktioniert vollständig ohne JavaScript (reines HTML + CSS)

### Barrierefreiheit
- WCAG 2.1 Level A konform
- Semantisches HTML für Screenreader-Kompatibilität
- Fokus-Indikatoren sichtbar
- Ausreichende Schriftgröße für Nutzung in der Küche (min. 16px Brottext)

---

## 6. Teststrategie

### E2E-Tests (Playwright)

**Testfall 1: Vollständiges Rezept anzeigen**
```gherkin
Given Ein Rezept "Spaghetti Bolognese" mit Zutaten und Anleitung existiert in der Datenbank
When Benutzer ruft die Detailseite des Rezepts auf
Then Der Titel "Spaghetti Bolognese" wird als H1 angezeigt
And Die Kategorien werden als Tags angezeigt
And Der Abschnitt "Zutaten" ist sichtbar
And Der Abschnitt "Anleitung" ist sichtbar
And Die Schaltflächen "Bearbeiten", "Löschen" und "Zurück zur Übersicht" sind vorhanden
```

**Testfall 2: Rezept ohne optionale Felder anzeigen**
```gherkin
Given Ein Rezept "Minimalrezept" existiert ohne Zutaten und ohne Anleitung
When Benutzer ruft die Detailseite des Rezepts auf
Then Der Titel "Minimalrezept" wird als H1 angezeigt
And Kein Abschnitt "Zutaten" wird angezeigt
And Kein Abschnitt "Anleitung" wird angezeigt
And Die Aktions-Schaltflächen sind vorhanden
```

**Testfall 3: Nicht vorhandene Rezept-ID**
```gherkin
Given Kein Rezept mit der ID 99999 existiert
When Benutzer ruft /recipes/99999 direkt auf
Then Eine 404-Fehlerseite wird angezeigt
And Die Fehlermeldung ist verständlich
And Ein Link zurück zur Rezeptliste ist vorhanden
```

**Testfall 4: DeepLink funktioniert**
```gherkin
Given Ein Rezept "Testrezept" existiert in der Datenbank
When Benutzer ruft die URL /recipes/{id} direkt auf (ohne vorherige Navigation)
Then Die Detailseite von "Testrezept" wird korrekt angezeigt
```

**Testfall 5: Erfolgs-Flash nach Bearbeiten**
```gherkin
Given Benutzer hat ein Rezept erfolgreich bearbeitet und gespeichert (Story 02)
When Benutzer wird auf die Detailansicht weitergeleitet
Then Die Erfolgsmeldung "Rezept erfolgreich aktualisiert" wird angezeigt
```

---

## 7. Abhängigkeiten & Rahmenbedingungen

### Abhängigkeiten

- **Story 01 (Rezept erstellen)** muss abgeschlossen sein - Rezepte müssen in der Datenbank existieren
- **Story 02 (Rezept bearbeiten)** verlinkt auf die Detailansicht nach erfolgreichem Speichern
- **Story 03 (Rezept löschen)** verwendet die Detailansicht als Ausgangspunkt für den Lösch-Button

### Rahmenbedingungen

- SQLite-Datenbank muss existieren und erreichbar sein
- Keine Authentifizierung erforderlich (LAN-only)
- Single-User-Modell (beide Partner = gleicher User, keine Berechtigungsprüfung)
- Bestehende rudimentäre Detailansicht (als Platzhalter aus Story 01 bekannt) wird durch diese vollwertige Ansicht ersetzt

---

## Offene Punkte / Fragen

- [ ] Soll der Text in Zutaten und Anleitung als Markdown gerendert werden (HTML) oder als reiner Vorformatierungs-Text (`<pre>`)? Markdown-Rendering würde Listen und Fettdruck ermöglichen, erfordert aber einen Markdown-Parser.

---

**Letzte Aktualisierung:** 2026-03-27
