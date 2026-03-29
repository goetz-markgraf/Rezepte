# Story 23: Rezepte mergen (Duplikate zusammenführen)

**Epic:** Daten-Qualität & Wartung
**Priorität:** Phase 2 - Growth Feature
**Status:** In Arbeit

---

## 1. Story-Satz

Als **Benutzer** möchte ich **zwei als Dubletten erkannte Rezepte zu einem einzigen zusammenführen**, damit ich **redundante Einträge bereinigen kann, ohne wertvolle Informationen (Bewertungen, Zutaten, Anleitungen) zu verlieren**.

---

## 2. Geschäftsbezogene Details

### Kontext

Story 22 (Dubletten-Prüfung und Übersicht) zeigt, welche Rezepte vermutlich doppelt vorhanden sind. Der nächste logische Schritt ist das tatsächliche Zusammenführen dieser Paare. Ohne eine Merge-Funktion muss der Nutzer manuell vorgehen: ein Rezept öffnen, Informationen merken, das andere bearbeiten, dann das erste löschen - fehleranfällig und zeitaufwendig.

Das PRD beschreibt genau diesen Workflow in Journey 4 ("Wartung & Fehlerszenarien"): Anna öffnet beide Rezepte, vergleicht sie und "merged die Infos ins erste Rezept, löscht das zweite". Story 23 macht diesen Prozess direkt und sicher möglich, ohne die App verlassen zu müssen.

Das Mergen ist inhärent eine menschliche Entscheidung: Welches Rezept ist das "richtige"? Welche Informationen sollen übernommen werden? Die App unterstützt dabei, trifft aber keine automatischen Entscheidungen über den Inhalt.

### Nutzergruppe

- Beide Partner des Haushalts bei gelegentlicher Sammlungs-Wartung
- Typischerweise nach dem Erkennen von Dubletten über die Dubletten-Übersicht (Story 22)

### Business-Value

- Ermöglicht die tatsächliche Bereinigung von Dubletten, die Story 22 nur identifiziert
- Vermeidet Informationsverlust beim manuellen Löschen eines der doppelten Rezepte
- Reduziert Aufwand und Fehlerrisiko im Wartungsprozess
- Hält die Rezeptsammlung langfristig sauber und vertrauenswürdig

### Edge Cases

- **Beide Rezepte haben Bewertungen:** Der Nutzer entscheidet, welche Bewertung übernommen wird (keine automatische Zusammenfassung)
- **Beide Rezepte haben Anleitungen/Zutaten:** Voransicht beider Inhalte ermöglicht bewusste Auswahl
- **Nur ein Rezept hat Inhalt:** Das befüllte Rezept wird als Ziel vorgeschlagen
- **Merge wird abgebrochen:** Kein Rezept wird verändert - der Nutzer landet zurück auf der Dubletten-Übersicht
- **Netzwerkfehler während des Merge:** Fehlermeldung, kein partieller Zustand bleibt in der Datenbank
- **Rezept wurde seit dem Öffnen der Dubletten-Übersicht gelöscht:** Klarer Hinweis, dass das Rezept nicht mehr existiert

---

## 3. Akzeptanzkriterien

### Funktionale Kriterien

- [ ] **K1: Merge-Aktion aus der Dubletten-Übersicht erreichbar**
  - Auf der Dubletten-Übersicht (`/recipes/duplicates`) gibt es pro Paar einen "Mergen"-Button oder -Link
  - Der Button führt zur Merge-Ansicht für dieses spezifische Paar
  - Die Merge-URL ist deeplink-fähig (z.B. `/recipes/merge?source=ID_A&target=ID_B`)

- [ ] **K2: Merge-Ansicht zeigt beide Rezepte vollständig**
  - Die Merge-Seite zeigt beide Rezepte nebeneinander oder untereinander mit allen relevanten Feldern: Titel, Kategorien, Zutaten, Anleitung, Bewertung, Datum
  - Leere Felder sind als solche erkennbar (nicht als fehlerhafter Inhalt)
  - Der Nutzer kann beide Rezepte vollständig vergleichen, ohne extra navigieren zu müssen

- [ ] **K3: Nutzer wählt das Ziel-Rezept**
  - Der Nutzer bestimmt, welches der beiden Rezepte als "Basis" (Ziel) erhalten bleibt und welches gelöscht wird (Quelle)
  - Diese Wahl ist explizit und unmissverständlich im UI dargestellt
  - Standardmäßig wird das Rezept mit mehr Inhalt (Bewertung, Datum, ausgefüllte Felder) als Ziel vorgeschlagen

- [ ] **K4: Nutzer wählt, welche Felder übernommen werden**
  - Für jedes Feld, das in beiden Rezepten vorhanden ist, kann der Nutzer auswählen, welche Version übernommen wird
  - Felder, die nur in einem Rezept vorhanden sind, werden automatisch übernommen (kein manueller Eingriff nötig)
  - Felder, die in beiden Rezepten leer sind, bleiben leer

- [ ] **K5: Merge-Vorschau vor Bestätigung**
  - Vor dem endgültigen Merge sieht der Nutzer eine Zusammenfassung, wie das zusammengeführte Rezept aussehen wird
  - Erst nach expliziter Bestätigung wird der Merge durchgeführt

- [ ] **K6: Merge-Ergebnis**
  - Nach dem Merge existiert genau ein Rezept mit den gewählten Inhalten
  - Das als Quelle gewählte Rezept ist vollständig gelöscht
  - Der Nutzer wird zur Detailansicht des zusammengeführten Rezepts weitergeleitet
  - Eine Erfolgsmeldung bestätigt den abgeschlossenen Merge

- [ ] **K7: Abbruch-Möglichkeit**
  - Der Nutzer kann den Merge-Prozess jederzeit abbrechen
  - Bei Abbruch bleiben beide Rezepte unverändert
  - Der Nutzer kehrt zur Dubletten-Übersicht zurück

- [ ] **K8: Abgesicherter Merge-Vorgang**
  - Der Merge wird atomar durchgeführt (Ziel-Rezept aktualisieren + Quelle löschen in einer Transaktion)
  - Bei Fehler bleibt der ursprüngliche Zustand erhalten

### Nicht-funktionale Kriterien

- [ ] **K9: Performance**
  - Die Merge-Seite lädt in < 1 Sekunde
  - Der Merge-Vorgang selbst (Speichern) dauert < 500ms

- [ ] **K10: Barrierefreiheit**
  - Alle Formularfelder und Auswahl-Elemente haben korrekte Labels (WCAG 2.1 Level A)
  - Tastatur-Navigation durch die Merge-Ansicht funktioniert vollständig
  - Die Auswahl des Ziel-Rezepts und der Felder ist auch ohne Maus bedienbar

---

## 4. Technische Planung

### Datenmodell

Kein neues Datenbankschema erforderlich. Die bestehende `recipes`-Tabelle wird genutzt:
- Ziel-Rezept wird mit den gewählten Feldinhalten aktualisiert (`UPDATE`)
- Quelle-Rezept wird gelöscht (`DELETE`)
- Beide Operationen werden in einer SQLite-Transaktion ausgeführt

### UI/UX-Spezifikation

**Einstiegspunkt:**
- "Mergen"-Button pro Dubletten-Paar auf der Dubletten-Übersicht (`/recipes/duplicates`)

**Merge-Ansicht (`/recipes/merge?source=ID&target=ID`):**
- Seitenüberschrift: "Rezepte zusammenführen"
- Erklärender Hinweis: "Wähle, welche Informationen das zusammengeführte Rezept enthalten soll. Das nicht gewählte Rezept wird anschließend gelöscht."
- Nebeneinander-Darstellung beider Rezepte (auf mobil untereinander)
- Pro Feld mit Inhalt in beiden Rezepten: Radio-Button-Auswahl (Rezept A / Rezept B)
- Felder die nur in einem Rezept vorhanden: automatisch übernommen, klar als solche markiert
- Bereich "Welches Rezept bleibt erhalten?" mit klarer Auswahl (betrifft die ID, die erhalten bleibt)
- "Vorschau" zeigt das zusammengeführte Ergebnis (oder wird direkt über die Radio-Buttons aktualisiert per HTMX)
- Buttons: "Zusammenführen" (primär) und "Abbrechen" (sekundär)

**Nach dem Merge:**
- Weiterleitung zur Detailansicht des zusammengeführten Rezepts
- Kurze Erfolgsmeldung (Flash-Message oder im Template)

---

## 5. Nicht-funktionale Anforderungen

### Performance
- Merge-Seite lädt ohne sichtbare Verzögerung (< 1s)
- Merge-Vorgang (POST) wird in < 500ms abgeschlossen
- Atomarer Datenbankvorgang (Transaktion) verhindert inkonsistente Zustände

### Browser-Support
- Aktuelle Chrome, Firefox, Safari, Edge Versionen
- iOS Safari und Android Chrome (letzte 2 Versionen)

### Barrierefreiheit
- WCAG 2.1 Level A konform
- Alle interaktiven Elemente per Tastatur erreichbar
- Klare Beschriftung aller Auswahl-Optionen
- Fokus-Indikatoren sichtbar

---

## 6. Teststrategie

### E2E-Tests (Playwright)

**Testfall 1: Merge erfolgreich durchführen**
```gherkin
Given Zwei ähnliche Rezepte "Pizza Margherita" (mit Bewertung 5★) und "Margherita Pizza" (mit Zutaten-Liste) existieren
And Die Dubletten-Übersicht zeigt dieses Paar
When Benutzer klickt "Mergen" beim Paar
And Benutzer wählt "Pizza Margherita" als Basis
And Benutzer wählt die Zutaten-Liste aus "Margherita Pizza"
And Benutzer bestätigt den Merge
Then Nur noch ein Rezept "Pizza Margherita" existiert
And Das Rezept hat Bewertung 5★ und die Zutaten-Liste
And "Margherita Pizza" ist gelöscht
And Benutzer sieht die Detailansicht von "Pizza Margherita"
```

**Testfall 2: Merge abbrechen**
```gherkin
Given Zwei ähnliche Rezepte "Dinkelbrot" und "Dinkel Brot" existieren
When Benutzer klickt "Mergen" beim Paar
And Benutzer klickt "Abbrechen"
Then Beide Rezepte existieren weiterhin unverändert
And Benutzer sieht die Dubletten-Übersicht
```

**Testfall 3: Einseitiger Inhalt wird automatisch übernommen**
```gherkin
Given Rezept A hat Titel, Bewertung, aber keine Zutaten
And Rezept B hat Titel und Zutaten, aber keine Bewertung
When Benutzer öffnet die Merge-Ansicht für dieses Paar
Then Bewertung aus Rezept A ist automatisch für das Ergebnis markiert (kein Konflikt)
And Zutaten aus Rezept B sind automatisch für das Ergebnis markiert (kein Konflikt)
And Der Nutzer muss nur den Titel auswählen (einziger Konflikt)
```

**Testfall 4: Direktlink zur Merge-Ansicht**
```gherkin
Given Zwei Rezepte mit bekannten IDs existieren
When Benutzer ruft /recipes/merge?source=ID_A&target=ID_B direkt auf
Then Die Merge-Ansicht lädt korrekt mit beiden Rezepten
```

---

## 7. Abhängigkeiten & Rahmenbedingungen

### Abhängigkeiten

- **Story 22 (Dubletten-Prüfung und Übersicht)** muss abgeschlossen sein - der Einstiegspunkt für den Merge kommt aus der Dubletten-Übersicht
- **Story 21 (Duplikaterkennung während Titeleingabe)** ist indirekt abhängig (gleiche Ähnlichkeitslogik)
- **Story 01 (Rezept erstellen)**, **Story 02 (Rezept bearbeiten)**, **Story 03 (Rezept löschen)** müssen abgeschlossen sein

### Rahmenbedingungen

- SQLite-Datenbank muss existieren und erreichbar sein
- Kein Authentifizierungskonzept (LAN-only)
- Last-write-wins Ansatz gilt: Der Merge überschreibt das Ziel-Rezept ohne weitere Konfliktprüfung
- HTMX ist bereits im Projekt integriert und kann für Live-Vorschau der Feldauswahl genutzt werden
- Der Merge-Vorgang muss als SQLite-Transaktion implementiert sein

---

## Offene Punkte / Fragen

- [ ] Soll der Merge auch direkt aus der Detailansicht eines Rezepts initiiert werden können (nicht nur aus der Dubletten-Übersicht)?
- [ ] Wie wird die Vorschau umgesetzt - als separater Schritt (eigene Seite) oder live per HTMX innerhalb der Merge-Ansicht?
- [ ] Soll nach einem erfolgreichen Merge die Dubletten-Übersicht neu berechnet werden, oder reicht die Weiterleitung zur Detailansicht?

---

**Letzte Aktualisierung:** 2026-03-29
