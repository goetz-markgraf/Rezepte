# Story 36: Markdown-Rendering in der Rezept-Detailansicht

**Epic:** Epic 1: Rezept-Verwaltung (Grundlegendes CRUD)
**Priorität:** MVP
**Status:** In Arbeit

---

## 1. Story-Satz

Als **Nutzer** möchte ich **dass Zutatenliste und Zubereitung in der Rezept-Detailansicht als formatierter Text dargestellt werden**, damit ich **Rezepte übersichtlich erfassen und beim Nachschlagen schnell lesen kann**.

---

## 2. Geschäftsbezogene Details

### Kontext

Beim Eingeben von Rezepten nutzen die Nutzer bereits natürliche Textformatierung: Zutaten werden zeilenweise als Liste getippt, Zubereitungsschritte als nummerierte Folge, wichtige Hinweise fett hervorgehoben. Im PRD ist Markdown-Support für Zutaten und Zubereitung explizit als Must-Have-Capability aufgeführt (FR5). Auch die User Journey 2 ("Neues Brotrezept entdeckt") beschreibt wie Dragon Zutaten als Markdown-Liste eingibt.

Aktuell wird der Rohtext unformatiert angezeigt - die eingetippten Sonderzeichen (`-`, `**`, `[ ]`) sind sichtbar und der Text wirkt unstrukturiert. Das erschwert das schnelle Erfassen eines Rezepts.

### Nutzergruppe

Der Haushalt (Anna & Dragon) - beide Partner, die Rezepte erfassen und beim Planen oder Kochen nachschlagen.

### Business-Value

- **Bessere Lesbarkeit**: Zutaten als Aufzählung, Schritte nummeriert - das Rezept ist auf einen Blick erfassbar.
- **Keine doppelte Arbeit**: Nutzer formatieren schon beim Eintippen intuitiv. Das Rendering macht diese Formatierung sichtbar, ohne dass sie umlernen müssen.
- **Checklisten-Funktion**: Zutaten oder Schritte als `[ ]`-Checkboxen ermöglichen das Abhaken während der Vorbereitung.
- **Erhöhte Motivation**: Ansprechend formatierte Rezepte machen die App attraktiver und fördern die langfristige Nutzung.

### Edge Cases

- **Kein Markdown-Inhalt:** Reiner Fließtext ohne Markdown-Syntax wird unverändert als normaler Absatz angezeigt.
- **Leere Felder:** Ist Zutatenliste oder Zubereitung leer, wird der Abschnitt ausgeblendet (kein leerer Platzhalter).
- **Nur Leerzeichen / Leerzeilen:** Führende und abschließende Leerzeilen werden ignoriert, der Abschnitt wird nicht angezeigt.
- **Sehr langer Text:** Langer Inhalt bricht korrekt um und scrollt innerhalb der Seite - kein horizontaler Überlauf.
- **Ungültiges / kaputtes Markdown:** Unvollständige Syntax (z.B. `**fett` ohne schließendes `**`) wird als Rohtext angezeigt, ohne Fehlermeldung.
- **HTML-Inhalte:** Wenn Nutzer versehentlich HTML-Tags eintippen, werden diese als Text dargestellt und nicht ausgeführt (keine XSS-Gefahr, auch wenn die App LAN-only ist).
- **Checkboxen:** Checkboxen (`[ ]` / `[x]`) in Zutaten oder Zubereitung sind in der Detailansicht interaktiv anklickbar (nur visuell, ohne Persistenz - nach Neuladen wieder zurückgesetzt).

---

## 3. Akzeptanzkriterien

### Funktionale Kriterien

- [ ] **K1: Aufzählungslisten werden gerendert**
  - Zeilen, die mit `- ` oder `* ` beginnen, werden als Aufzählungspunkte dargestellt
  - Verschachtelte Listen (mit Einrückung) werden korrekt dargestellt

- [ ] **K2: Nummerierte Listen werden gerendert**
  - Zeilen, die mit `1.`, `2.`, etc. beginnen, werden als nummerierte Liste dargestellt

- [ ] **K3: Fettschrift und Kursivschrift werden gerendert**
  - `**fett**` erscheint als fetter Text
  - `*kursiv*` erscheint als kursiver Text
  - `***fett und kursiv***` erscheint entsprechend kombiniert

- [ ] **K4: Checkboxen werden dargestellt und sind anklickbar**
  - `- [ ]` wird als leere Checkbox dargestellt
  - `- [x]` wird als angehakte Checkbox dargestellt
  - Checkboxen sind in der Detailansicht anklickbar (visuell, ohne Persistenz)

- [ ] **K5: Überschriften werden gerendert**
  - Zeilen mit `#`, `##`, `###` werden als Überschriften verschiedener Ebenen dargestellt

- [ ] **K6: Horizontale Trennlinien werden gerendert**
  - `---` wird als horizontale Trennlinie dargestellt

- [ ] **K7: Code-Blöcke werden gerendert**
  - Text in Backticks (`` `code` ``) wird als Inline-Code hervorgehoben
  - Text in dreifachen Backticks wird als Codeblock dargestellt

- [ ] **K8: Rohtext bei fehlendem Markdown bleibt lesbar**
  - Text ohne Markdown-Syntax wird als normaler Absatz dargestellt
  - Absätze (durch Leerzeile getrennte Textblöcke) werden als separate Absätze gerendert

- [ ] **K9: Leere Felder werden ausgeblendet**
  - Ist die Zutatenliste leer, wird der Abschnitt "Zutaten" nicht angezeigt
  - Ist die Zubereitung leer, wird der Abschnitt "Zubereitung" nicht angezeigt

- [ ] **K10: HTML wird nicht ausgeführt**
  - Eingetippte HTML-Tags werden als Rohtext angezeigt, nicht als HTML interpretiert

### Nicht-funktionale Kriterien

- [ ] **K11: Performance**
  - Die Detailansicht lädt in unter 500ms (inkl. Markdown-Rendering)
  - Das Rendering erfolgt serverseitig - keine sichtbare Verzögerung durch clientseitiges Nachladen

- [ ] **K12: Barrierefreiheit**
  - Gerenderte Listen verwenden semantisch korrektes HTML (`<ul>`, `<ol>`, `<li>`)
  - Checkboxen haben eine zugängliche Beschriftung (WCAG 2.1 Level A)
  - Farbkontraste der gerenderten Inhalte erfüllen WCAG 2.1 Level A (4.5:1)

---

## 4. Technische Planung

### Datenmodell

Keine Änderung am Datenmodell erforderlich. Die Felder `Zutaten` und `Anleitung/Zubereitung` speichern weiterhin Markdown-Rohtext. Das Rendering erfolgt bei der Ausgabe.

### UI/UX-Spezifikation

**Detailansicht - Abschnitt Zutaten:**
- Der gespeicherte Rohtext wird beim Anzeigen als Markdown gerendert
- Aufzählungslisten erscheinen als `<ul>` mit `<li>`-Elementen
- Checkboxen erscheinen als anklickbare Checkbox-Elemente (nur visuell, kein Speichern)
- Die visuelle Darstellung orientiert sich am bestehenden Designsystem (Schriftarten, Farben)

**Detailansicht - Abschnitt Zubereitung:**
- Identisches Rendering wie Zutaten
- Nummerierte Schritte erscheinen als `<ol>`

**Editier-Ansicht:**
- Das Eingabefeld bleibt ein einfaches Textfeld (Textarea) - kein WYSIWYG-Editor
- Nutzer tippen weiterhin Markdown-Rohtext

---

## 5. Nicht-funktionale Anforderungen

### Performance
- Detailseite lädt in unter 500ms (inkl. serverseitigem Markdown-Rendering)
- Keine zusätzlichen JS-Bibliotheken für das Rendering erforderlich (serverseitig)

### Browser-Support
- Aktuelle Chrome, Firefox, Safari, Edge Versionen
- iOS Safari und Android Chrome (letzte 2 Versionen)
- Rendering erfolgt serverseitig, daher keine Browser-spezifischen Abhängigkeiten

### Barrierefreiheit
- WCAG 2.1 Level A konform
- Semantisches HTML für alle gerenderten Elemente
- Checkboxen mit zugänglichen Labels

---

## 6. Teststrategie

### E2E-Tests (Playwright)

**Testfall 1: Aufzählungsliste wird in Zutaten gerendert**
```gherkin
Given ein Rezept mit Zutaten "- 500g Mehl\n- 1 Ei\n- 250ml Milch"
When der Nutzer die Detailansicht des Rezepts öffnet
Then werden die Zutaten als Aufzählungsliste mit drei Punkten angezeigt
And die Markdown-Syntax "- " ist nicht als Rohtext sichtbar
```

**Testfall 2: Checkbox in Zutaten wird angezeigt**
```gherkin
Given ein Rezept mit Zutaten "- [ ] Mehl\n- [x] Eier"
When der Nutzer die Detailansicht öffnet
Then wird eine leere Checkbox für "Mehl" angezeigt
And eine angehakte Checkbox für "Eier" angezeigt
```

**Testfall 3: Fettschrift in Zubereitung wird gerendert**
```gherkin
Given ein Rezept mit Zubereitung "**Wichtig:** Ofen vorheizen auf 180°C"
When der Nutzer die Detailansicht öffnet
Then wird "Wichtig:" in Fettschrift dargestellt
And der Text "**Wichtig:**" ist nicht als Rohtext sichtbar
```

**Testfall 4: Leeres Zutatenfeld - Abschnitt ausgeblendet**
```gherkin
Given ein Rezept ohne Zutaten (leeres Feld)
When der Nutzer die Detailansicht öffnet
Then wird der Abschnitt "Zutaten" nicht angezeigt
```

**Testfall 5: Reiner Fließtext bleibt lesbar**
```gherkin
Given ein Rezept mit Zubereitung als Fließtext ohne Markdown-Syntax
When der Nutzer die Detailansicht öffnet
Then wird der Text als normaler Absatz angezeigt
And der Text ist vollständig und unverändert lesbar
```

---

## 7. Abhängigkeiten & Rahmenbedingungen

### Abhängigkeiten
- Story 04 (Rezept-Detailansicht) muss implementiert sein - bildet die Grundlage für das Rendering

### Rahmenbedingungen
- SQLite-Datenbank muss existieren und erreichbar sein
- Keine Authentifizierung erforderlich (LAN-only)
- Rendering erfolgt serverseitig (kein clientseitiges Markdown-Parsing)
- Die Eingabe-Textfelder im Bearbeitungsmodus bleiben unverändert (weiterhin Textareas für Rohtext)

---

## Offene Punkte / Fragen

- [ ] Sollen Checkboxen in der Detailansicht ihren Zustand behalten (persistiert) oder ist rein visuelles Anklicken ohne Speicherung ausreichend?

---

**Letzte Aktualisierung:** 2026-04-03
