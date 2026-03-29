# Story 22: Dubletten-Prüfung und Übersicht

**Epic:** Daten-Qualität & Wartung
**Priorität:** Phase 2 - Growth Feature
**Status:** In Arbeit

---

## 1. Story-Satz

Als **Benutzer** möchte ich **eine Übersicht aller potentiellen Dubletten in meiner Rezeptsammlung sehen**, damit ich **Duplikate gezielt erkennen und bereinigen kann, bevor die Sammlung unübersichtlich wird**.

---

## 2. Geschäftsbezogene Details

### Kontext

Mit wachsender Rezeptsammlung (Ziel: 40-50 Rezepte nach 3 Monaten) steigt das Risiko, dass Rezepte doppelt erfasst wurden - sei es weil zwei Varianten desselben Gerichts unter ähnlichen Namen existieren (z.B. "Pizza Margherita" und "Margherita Pizza"), oder weil ein Rezept schlicht vergessen und nochmals angelegt wurde.

Story 21 (Duplikaterkennung während Titeleingabe) greift präventiv beim Erstellen. Story 22 ist der kurative Ansatz: Nutzer können jederzeit aktiv prüfen, ob sich in der bestehenden Sammlung Dubletten eingeschlichen haben - auch für Rezepte, die vor Einführung der Duplikaterkennung angelegt wurden.

Im PRD ist dieses Szenario in Journey 4 ("Wartung & Fehlerszenarien") beschrieben: Nach 6 Monaten mit 60 Rezepten vermutet Anna Duplikate. Sie öffnet die Dubletten-Prüfung, sieht "Pizza Margherita" und "Margherita Pizza" gegenübergestellt und kann gezielt handeln.

### Nutzergruppe

- Beide Partner des Haushalts bei gelegentlicher Sammlungs-Wartung
- Besonders relevant wenn die Sammlung auf 30+ Rezepte angewachsen ist und der manuelle Überblick schwieriger wird

### Business-Value

- Erhält die Datenqualität der Rezeptsammlung langfristig
- Schafft Vertrauen: Nutzer wissen, dass die Sammlung keine redundanten Einträge enthält
- Ergänzt die präventive Duplikaterkennung (Story 21) um eine kurative Maßnahme
- Legt die Grundlage für Story 23 (Rezepte mergen), indem Dubletten-Paare identifiziert werden

### Edge Cases

- **Keine Dubletten vorhanden:** Eine leere Seite mit einem positiven Hinweis ("Keine ähnlichen Rezepte gefunden - deine Sammlung ist sauber!") wird angezeigt
- **Sehr große Sammlung (100+ Rezepte):** Die Berechnung kann einige Sekunden dauern; eine Lade-Anzeige informiert den Nutzer
- **Falsch-Positiv (kein echter Duplikat):** Nutzer kann das Paar ignorieren oder schließen - es gibt keine Aktion erzwingen
- **Echter Dubletten-Fall:** Nutzer kann zu den Einzelansichten navigieren und manuell entscheiden (merge in Story 23, oder manuell löschen)
- **Rezept mit sich selbst:** Das gleiche Rezept darf nicht mit sich selbst als Dublette erscheinen
- **Alle Ähnlichkeiten bereits bekannt:** Die Übersicht bleibt konsistent und zeigt alle Paare, unabhängig davon, ob der Nutzer sie bereits gesehen hat

---

## 3. Akzeptanzkriterien

### Funktionale Kriterien

- [ ] **K1: Dubletten-Prüfungs-Seite erreichbar**
  - Es gibt eine eigene Seite/Ansicht für die Dubletten-Prüfung
  - Die Seite ist über einen Link in der Navigation oder der Rezeptliste erreichbar
  - Die URL ist deeplink-fähig (z.B. `/recipes/duplicates`)

- [ ] **K2: Anzeige potentieller Dubletten-Paare**
  - Die Seite listet alle Rezept-Paare, die einen ähnlichen Titel haben
  - Jedes Paar zeigt beide Rezepte mit: Titel, Bewertung (falls vorhanden), und letztem Datum
  - Die Paare sind nach Ähnlichkeit sortiert (die wahrscheinlichsten Dubletten zuerst)

- [ ] **K3: Navigation zu den Einzelrezepten**
  - Jedes Rezept in einem Paar ist ein klickbarer Link zur Detailansicht
  - Der Nutzer kann so beide Rezepte in separaten Tabs vergleichen

- [ ] **K4: Leere Zustand**
  - Wenn keine ähnlichen Rezepte gefunden werden, wird eine positive Meldung angezeigt
  - Kein leeres oder fehlendes UI-Element

- [ ] **K5: Nutzung der bestehenden Ähnlichkeitslogik**
  - Die Dubletten-Prüfung verwendet die gleiche `find_similar_recipes()` Funktion aus Story 21
  - Alle Rezepte werden paarweise gegen die restlichen Rezepte geprüft
  - Jedes Paar erscheint nur einmal (nicht A→B und B→A doppelt)

### Nicht-funktionale Kriterien

- [ ] **K6: Performance**
  - Die Seite lädt bei bis zu 100 Rezepten in < 2 Sekunden
  - Kein Timeout bei normaler Sammlungsgröße (< 200 Rezepte)

- [ ] **K7: Barrierefreiheit**
  - Alle Rezept-Links sind per Tastatur erreichbar
  - Dubletten-Paare sind semantisch klar strukturiert (z.B. als Listen oder Tabellen mit korrekten Labels)
  - WCAG 2.1 Level A konform

---

## 4. Technische Planung

### Datenmodell

Keine Änderungen am Datenbankschema erforderlich. Die bestehende `find_similar_recipes()` Funktion aus Story 21 wird wiederverwendet, um für jedes Rezept ähnliche Kandidaten zu finden.

Der Algorithmus für die Übersicht:
1. Alle Rezepte laden
2. Für jedes Rezept `find_similar_recipes(title, exclude_id: recipe.id)` aufrufen
3. Gefundene Paare deduplizieren (Paar A+B = Paar B+A)
4. Paare nach Trefferzahl oder Ähnlichkeit sortieren

### UI/UX-Spezifikation

**Einstiegspunkt:**
- Link "Dubletten prüfen" im Navigationsbereich oder als spezieller Filterbereich der Rezeptliste

**Dubletten-Übersicht (`/recipes/duplicates`):**
- Seitenüberschrift: "Mögliche Dubletten"
- Untertitel oder Erklärungstext: "Diese Rezepte haben ähnliche Titel. Prüfe, ob es sich um Duplikate handelt."
- Pro Paar: eine visuelle Einheit (Card oder Tabellenzeile) mit beiden Rezepten nebeneinander oder übereinander
- Jedes Rezept zeigt: Titel (als Link), Bewertung in Sternen (falls vorhanden), letztes Datum (falls vorhanden)
- Leerer Zustand: "Keine ähnlichen Rezepte gefunden. Deine Sammlung ist sauber!"

---

## 5. Nicht-funktionale Anforderungen

### Performance
- Seite lädt bei bis zu 100 Rezepten ohne sichtbare Verzögerung (< 2s)
- Datenbankabfragen werden effizient gebündelt (nicht N+1)

### Browser-Support
- Aktuelle Chrome, Firefox, Safari, Edge Versionen
- iOS Safari und Android Chrome (letzte 2 Versionen)

### Barrierefreiheit
- WCAG 2.1 Level A konform
- Fokus-Indikatoren auf allen Rezept-Links sichtbar
- Semantische HTML-Struktur für Screenreader-Kompatibilität

---

## 6. Teststrategie

### E2E-Tests (Playwright)

**Testfall 1: Dubletten werden angezeigt**
```gherkin
Given Zwei Rezepte mit ähnlichen Titeln existieren ("Pizza Margherita" und "Margherita Pizza")
When Benutzer öffnet die Dubletten-Übersicht (/recipes/duplicates)
Then Beide Rezepte erscheinen als Paar auf der Seite
And Jedes Rezept ist ein klickbarer Link zur Detailansicht
```

**Testfall 2: Leere Sammlung oder keine Dubletten**
```gherkin
Given Die Rezeptsammlung enthält keine ähnlichen Rezepte
When Benutzer öffnet die Dubletten-Übersicht
Then Die Seite zeigt eine Meldung "Keine ähnlichen Rezepte gefunden"
And Kein Paar wird angezeigt
```

**Testfall 3: Navigation zu Einzelrezepten**
```gherkin
Given Ein Dubletten-Paar ("Dinkelbrot" und "Dinkel Brot") ist sichtbar
When Benutzer klickt auf den Link "Dinkelbrot"
Then Benutzer wird zur Detailansicht von "Dinkelbrot" navigiert
```

**Testfall 4: Paar erscheint nur einmal**
```gherkin
Given Zwei ähnliche Rezepte "Rezept A" und "Rezept B" existieren
When Benutzer öffnet die Dubletten-Übersicht
Then Das Paar A+B erscheint genau einmal (nicht als A→B und B→A)
```

---

## 7. Abhängigkeiten & Rahmenbedingungen

### Abhängigkeiten

- **Story 21 (Duplikaterkennung während Titeleingabe)** muss abgeschlossen sein - die `find_similar_recipes()` Funktion muss existieren und wird wiederverwendet
- **Story 01 (Rezept erstellen)** und **Story 04 (Rezept-Detailansicht)** müssen abgeschlossen sein
- Ergänzt Story 23 (Rezepte mergen) - diese Story identifiziert die Paare, Story 23 wird das Zusammenführen ermöglichen

### Rahmenbedingungen

- SQLite-Datenbank muss existieren und erreichbar sein
- Keine Authentifizierung erforderlich (LAN-only)
- HTMX ist bereits im Projekt integriert (kann für optionale Interaktivität genutzt werden)

---

## Offene Punkte / Fragen

- [ ] Wo genau ist der Einstiegspunkt in der Navigation? (separater Menüpunkt, Link in der Rezeptliste, oder im Footer-Bereich?)
- [ ] Soll es eine Möglichkeit geben, ein Paar als "kein Duplikat" zu markieren, damit es nicht erneut erscheint?
- [ ] Soll bereits ein "Löschen"-Button pro Rezept im Paar angeboten werden, oder nur Links zur Detailansicht (und Löschen dann dort)?

---

**Letzte Aktualisierung:** 2026-03-29
