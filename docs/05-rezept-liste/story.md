# Story 5: Rezept-Liste alphabetisch sortiert

**Epic:** Rezept-Übersicht & Navigation
**Priorität:** MVP - Phase 1
**Status:** In Arbeit

---

## 1. Story-Satz

**Als** Benutzer möchte ich **alle gespeicherten Rezepte in einer alphabetisch sortierten Liste sehen**, damit ich **schnell einen Überblick über meine gesamte Rezeptsammlung bekomme und gezielt ein Rezept auswählen kann**.

---

## 2. Geschäftsbezogene Details

### Kontext

Die Rezept-Liste ist die Startseite der Anwendung und der zentrale Einstiegspunkt für alle Nutzungsszenarien. Ob bei der Wochenplanung am Mittwochabend oder beim schnellen Nachschlagen eines Rezepts - die Liste ist die erste Anlaufstelle. Die alphabetische Sortierung erlaubt intuitives Browsen ohne Suche, wenn der Nutzer ungefähr weiß, was er sucht.

Ohne eine funktionierende Liste bleibt die gesamte Rezeptsammlung unzugänglich. Stories wie Suche (07), Kategorienfilter (08) und Wochenplanung (18) bauen alle auf dieser Grundansicht auf.

### Nutzergruppe

- Beide Partner des Haushalts (gleiche Berechtigungen, kein Rollenmodell)
- LAN-Zugriff über verschiedene Geräte (Desktop, Tablet, Handy)
- Typische Situation: Tablet auf dem Sofa beim Wochenplanen, Handy in der Küche

### Business-Value

- Ermöglicht erstmals eine vollständige, schnelle Übersicht über alle gespeicherten Rezepte
- Ersetzt das mühsame Durchblättern von Büchern, Ordnern und Zetteln
- Grundlage für alle nachgelagerten Features (Suche, Filter, Wochenplanung)
- DeepLink-fähige Startseite (`/`) ermöglicht direkten Einstieg per Browser-Lesezeichen oder Homescreen-Shortcut

### Edge Cases

- **Keine Rezepte vorhanden:** Leere Liste mit verständlicher Meldung und deutlichem Hinweis, ein erstes Rezept anzulegen
- **Nur ein Rezept vorhanden:** Liste zeigt genau einen Eintrag, keinerlei Darstellungsfehler
- **Sehr viele Rezepte (100+):** Liste bleibt scrollbar und nutzbar; keine Pagination im MVP erforderlich (realistische Datenmenge: 40-60 Rezepte)
- **Sonderzeichen im Titel:** Rezepte mit Umlauten (ä, ö, ü) oder anderen Sonderzeichen werden korrekt alphabetisch einsortiert (deutschsprachige Sortierung)
- **Sehr langer Titel:** Titel wird sauber abgeschnitten oder umbgebrochen, kein Layout-Bruch

---

## 3. Akzeptanzkriterien

### Funktionale Kriterien

- [ ] **K1: Alle Rezepte werden angezeigt**
  - Jedes in der Datenbank gespeicherte Rezept erscheint als Listeneintrag
  - Kein Rezept wird ausgelassen oder doppelt angezeigt

- [ ] **K2: Alphabetische Sortierung**
  - Die Liste ist aufsteigend nach Titel sortiert (A → Z)
  - Deutsche Sonderzeichen (Umlaute: ä, ö, ü) werden korrekt einsortiert (ä nach a, ö nach o, ü nach u)
  - Groß- und Kleinschreibung wird beim Sortieren nicht unterschieden

- [ ] **K3: Listeneinträge enthalten relevante Informationen**
  - Titel des Rezepts (gut lesbar, primäres Element)
  - Mindestens eine der zugeordneten Kategorien ist sichtbar
  - Jeder Eintrag ist anklickbar und führt zur Detailansicht des Rezepts (`/recipes/{id}`)

- [ ] **K4: Leere Liste wird korrekt behandelt**
  - Bei keinen vorhandenen Rezepten wird eine freundliche Meldung angezeigt (z.B. "Noch keine Rezepte vorhanden")
  - Ein deutlicher Hinweis oder direkter Link zum Erstellen des ersten Rezepts ist vorhanden

- [ ] **K5: Navigation zur Detailansicht**
  - Klick auf einen Listeneintrag öffnet die Detailansicht des jeweiligen Rezepts
  - Die URL wechselt auf `/recipes/{id}`

- [ ] **K6: Startseite der Anwendung**
  - Die Rezept-Liste ist unter der Root-URL (`/`) erreichbar
  - Sie ist die Standard-Landingpage beim Öffnen der App

- [ ] **K7: Link zum Erstellen eines neuen Rezepts**
  - Ein gut sichtbarer "Neues Rezept"-Button oder -Link ist auf der Seite vorhanden
  - Er führt zum Erstellungsformular (Story 01)

### Nicht-funktionale Kriterien

- [ ] **K8: Performance**
  - Seite lädt in < 500ms (bei realistischer Datenmenge bis 200 Rezepte)
  - Keine sichtbare Verzögerung beim Seitenaufruf

- [ ] **K9: Barrierefreiheit**
  - Semantische HTML-Struktur: Liste als `<ul>/<li>` oder `<ol>/<li>` mit aussagekräftigen Links
  - Überschriften-Hierarchie korrekt (H1 für Seitentitel)
  - Tastatur-Navigation: Alle Listeneinträge sind per Tab erreichbar und per Enter aktivierbar
  - WCAG 2.1 Level A konform

---

## 4. Technische Planung

### Datenmodell

Keine Änderungen am Datenbankschema erforderlich. Die bestehende `recipes`-Tabelle liefert alle notwendigen Felder:

| Feld | Typ | Genutzt für |
|------|-----|-------------|
| id | INTEGER | URL-Link zur Detailansicht |
| title | TEXT | Primäres Anzeigeelement, Sortierschlüssel |
| categories | JSON-Array | Kategorie-Anzeige im Listeneintrag |

Die Datenbankabfrage sortiert die Ergebnisse direkt nach Titel (ORDER BY title COLLATE NOCASE ASC).

### UI/UX-Spezifikation

**Seitenstruktur:**
- Seitentitel (H1): "Rezepte"
- Aktionsbereich oben: "Neues Rezept"-Button
- Listenbereich: Alle Rezepte als scrollbare Liste
- Leerzustand: Freundliche Meldung mit Aufforderung, das erste Rezept anzulegen

**Listeneintrag:**
- Titel des Rezepts als klickbarer Link (gesamte Eintragszeile oder dedizierter Link)
- Kategorie-Tags oder -Labels unterhalb oder neben dem Titel
- Kein Bild, keine weiteren Metadaten (Datum, Bewertung) in dieser Story - die Liste bleibt übersichtlich

**Responsive:**
- Desktop/Tablet: Einspaltiger, kompakter Listenbereich mit angemessenen Zeilenabständen
- Mobile: Volle Breite, gut tippbare Einträge (min. 44px Tipp-Zielfläche)

---

## 5. Nicht-funktionale Anforderungen

### Performance
- Seite lädt ohne sichtbare Verzögerung (< 500ms)
- Eine einzige Datenbankabfrage pro Seitenaufruf
- Keine Paginierung notwendig für realistische Datenmengen (bis 200 Rezepte)

### Browser-Support
- Aktuelle Chrome, Firefox, Safari, Edge Versionen
- iOS Safari und Android Chrome (letzte 2 Versionen)
- Funktioniert vollständig ohne JavaScript (reines HTML + CSS)

### Barrierefreiheit
- WCAG 2.1 Level A konform
- Semantisches HTML für Screenreader-Kompatibilität
- Fokus-Indikatoren sichtbar
- Ausreichende Schriftgröße (min. 16px Brottext)

---

## 6. Teststrategie

### E2E-Tests (Playwright)

**Testfall 1: Liste mit mehreren Rezepten anzeigen**
```gherkin
Given Mehrere Rezepte existieren in der Datenbank (z.B. "Zupfbrot", "Apfelkuchen", "Bolognese")
When Benutzer öffnet die Startseite "/"
Then Alle Rezepte werden als Liste angezeigt
And Die Liste ist alphabetisch sortiert (Apfelkuchen → Bolognese → Zupfbrot)
And Jeder Eintrag enthält den Titel und mindestens eine Kategorie
```

**Testfall 2: Leere Liste**
```gherkin
Given Keine Rezepte sind in der Datenbank vorhanden
When Benutzer öffnet die Startseite "/"
Then Eine verständliche Meldung "Noch keine Rezepte vorhanden" (oder ähnlich) wird angezeigt
And Ein deutlicher Hinweis oder Link zum Erstellen eines Rezepts ist sichtbar
```

**Testfall 3: Navigation zur Detailansicht**
```gherkin
Given Mindestens ein Rezept "Spaghetti Bolognese" existiert in der Datenbank
When Benutzer öffnet die Startseite "/"
And Benutzer klickt auf den Listeneintrag "Spaghetti Bolognese"
Then Benutzer wird auf die Detailansicht des Rezepts weitergeleitet
And Die URL enthält die ID des Rezepts (/recipes/{id})
And Der Titel "Spaghetti Bolognese" wird als H1 angezeigt
```

**Testfall 4: Alphabetische Sortierung mit Umlauten**
```gherkin
Given Rezepte "Überbackene Nudeln", "Apfelkuchen", "Zupfbrot" und "Ährenblüten-Tee" existieren
When Benutzer öffnet die Startseite "/"
Then Die Rezepte erscheinen in korrekter alphabetischer Reihenfolge
And Umlaute werden korrekt einsortiert (ä/ö/ü nach a/o/u)
```

**Testfall 5: Link zu neuem Rezept**
```gherkin
Given Benutzer ist auf der Startseite "/"
When Benutzer klickt auf "Neues Rezept"
Then Benutzer wird auf das Erstellungsformular weitergeleitet
```

---

## 7. Abhängigkeiten & Rahmenbedingungen

### Abhängigkeiten

- **Story 01 (Rezept erstellen)** muss abgeschlossen sein - Rezepte müssen in der Datenbank existieren und angelegt werden können
- **Story 04 (Rezept-Detailansicht)** muss abgeschlossen sein - Listeneinträge verlinken auf die Detailansicht
- Blockiert: Story 07 (Volltextsuche), Story 08 (Kategorienfilter), Story 09 (Filter "Länger nicht gemacht") - alle bauen auf der Listen-Ansicht auf

### Rahmenbedingungen

- SQLite-Datenbank muss existieren und erreichbar sein
- Keine Authentifizierung erforderlich (LAN-only)
- Single-User-Modell (beide Partner = gleicher User, keine Berechtigungsprüfung)
- Die rudimentäre Startseite aus Story 01 (nur H1 "Rezepte Übersicht") wird durch diese vollwertige Listen-Ansicht ersetzt

---

## Offene Punkte / Fragen

- [ ] Sollen Bewertungssterne (Story 14) oder das Datum "zuletzt gemacht" (Story 15) bereits in der Liste sichtbar sein, oder nur Titel und Kategorie? (Empfehlung: erst in dieser Story nur Titel + Kategorie, Erweiterungen in späteren Stories)
- [ ] Soll der "Neues Rezept"-Button dauerhaft sichtbar bleiben (z.B. als Floating Action Button auf Mobile) oder nur oben auf der Seite?

---

**Letzte Aktualisierung:** 2026-03-27
