# Story 21: Duplikaterkennung während Titeleingabe

**Epic:** Daten-Qualität & Wartung
**Priorität:** Phase 2 - Growth Feature
**Status:** In Arbeit

---

## 1. Story-Satz

**Als** Benutzer möchte ich **beim Eingeben eines Rezept-Titels sofort sehen, ob ähnliche Rezepte bereits existieren**, damit ich **versehentliche Duplikate vermeiden kann und die Rezeptsammlung sauber bleibt**.

---

## 2. Geschäftsbezogene Details

### Kontext

Mit wachsender Rezeptsammlung (Ziel: 40-50 Rezepte nach 3 Monaten) steigt das Risiko, dass Rezepte doppelt erfasst werden - sei es weil man vergessen hat, ein Rezept bereits angelegt zu haben, oder weil zwei Varianten des gleichen Gerichts unter leicht unterschiedlichen Namen eingetragen werden (z.B. "Pizza Margherita" und "Margherita Pizza").

Die Duplikaterkennung greift bereits beim Erstellen neuer Rezepte, bevor der Nutzer zu tief in die Erfassung einsteigt. Sie informiert proaktiv und nicht-blockierend: Der Nutzer sieht einen Hinweis, kann ihn ignorieren und trotzdem weitermachen - oder das bereits existierende Rezept aufrufen.

Im PRD-Journey 2 ("Sonntagabend - Neues Brotrezept entdeckt") ist dieses Verhalten bereits skizziert: "Dinkel..." → sofort erscheint eine Inline-Vorschau: "Ähnliche Rezepte gefunden: Dinkelbrot (3★)".

### Nutzergruppe

- Beide Partner des Haushalts beim Anlegen eines neuen Rezepts
- Besonders relevant bei mobiler Nutzung, wo der Überblick über die Sammlung schwieriger ist

### Business-Value

- Verhindert Datenmüll und Verwirrung durch Duplikate in der Rezeptsammlung
- Spart Zeit: kein doppeltes Erfassen von Rezepten
- Fördert die Datenqualität ohne harte Sperren oder Fehlermeldungen
- Ergänzt Story 22 (Dubletten-Prüfung und Übersicht) als präventive Maßnahme

### Edge Cases

- **Kein ähnliches Rezept gefunden:** Kein Hinweis, keine UI-Veränderung - der Nutzer tippt ungestört weiter
- **Titel zu kurz (< 3 Zeichen):** Keine Suche wird ausgelöst - zu viele False Positives
- **Exakter Titel-Match:** Deutlicher Hinweis, dass ein Rezept mit genau diesem Titel bereits existiert
- **Ähnlicher, aber nicht gleicher Titel:** Hinweis mit Auflistung der Kandidaten und deren Bewertung
- **Mehrere Kandidaten:** Bis zu 3 ähnliche Rezepte werden aufgelistet
- **Beim Bearbeiten eines bestehenden Rezepts:** Das Rezept selbst wird nicht als Duplikat gezeigt
- **Netzwerkfehler beim HTMX-Request:** Kein Hinweis erscheint - der Nutzer kann trotzdem fortfahren (Graceful Degradation)
- **Nutzer ignoriert den Hinweis:** Der Hinweis verhindert das Speichern nicht - es ist ein Informations-Hinweis, kein Blocker

---

## 3. Akzeptanzkriterien

### Funktionale Kriterien

- [ ] **K1: Duplikaterkennung bei Titeleingabe**
  - Im Rezept-Erstellformular wird die Duplikaterkennung beim Eingeben des Titels ausgelöst
  - Die Suche startet frühestens wenn 3 Zeichen eingegeben wurden
  - Die Suche wird per HTMX ausgelöst (hx-trigger="input changed delay:400ms") um nicht jeden Tastendruck zu verarbeiten

- [ ] **K2: Hinweis bei ähnlichen Rezepten**
  - Wenn ähnliche Rezepte gefunden werden, erscheint unterhalb des Titelfelds ein Hinweis-Block
  - Der Hinweis zeigt: "Ähnliche Rezepte gefunden:" gefolgt von einer Liste der Kandidaten
  - Jeder Kandidat zeigt: Titel und Bewertung (falls vorhanden)
  - Jeder Kandidat ist ein klickbarer Link zur Detailansicht des bestehenden Rezepts
  - Der Hinweis ist als Information gestaltet (nicht als Fehler), damit er nicht bedrohlich wirkt

- [ ] **K3: Hinweis verschwindet bei keiner Übereinstimmung**
  - Wenn der Nutzer den Titel ändert und keine ähnlichen Rezepte mehr gefunden werden, verschwindet der Hinweis
  - Wenn der Titel gelöscht oder kürzer als 3 Zeichen wird, verschwindet der Hinweis

- [ ] **K4: Duplikaterkennung beim Bearbeiten**
  - Auch beim Bearbeiten eines bestehenden Rezepts ist die Duplikaterkennung aktiv
  - Das aktuell bearbeitete Rezept wird dabei nicht als Duplikat angezeigt (Filter per ID)

- [ ] **K5: Kein Blocker - Speichern bleibt möglich**
  - Der Hinweis blockiert nicht das Speichern des Rezepts
  - Der Nutzer kann trotz Hinweis das Rezept anlegen (bewusste Entscheidung für ein Duplikat ist möglich)

- [ ] **K6: Ähnlichkeitssuche**
  - Die Suche erkennt ähnliche Titel auch bei leichten Abweichungen (Teilstring-Übereinstimmung)
  - "Pizza Margherita" wird bei Eingabe von "Margherita" gefunden
  - Die Suche ist case-insensitiv

### Nicht-funktionale Kriterien

- [ ] **K7: Performance**
  - Die HTMX-Anfrage zur Duplikatprüfung antwortet in < 200ms
  - Der Input-Delay verhindert übermäßige Serveranfragen (min. 300ms debounce)

- [ ] **K8: Barrierefreiheit**
  - Der Hinweis-Block hat ein passendes ARIA-Live-Attribut, damit Screenreader ihn ankündigen
  - Links zu ähnlichen Rezepten sind per Tastatur erreichbar
  - Der Hinweis ist visuell klar vom Formular-Eingabebereich abgegrenzt

---

## 4. Technische Planung

### Datenmodell

Keine Änderungen am Datenbankschema erforderlich. Die Duplikatprüfung nutzt die bestehende `recipes`-Tabelle mit einer `LIKE`-Suche auf dem `title`-Feld.

### UI/UX-Spezifikation

**Titelfeld im Rezept-Formular:**
- Das bestehende Titelfeld erhält HTMX-Attribute für die Live-Suche:
  - `hx-post="/recipes/check-duplicate"` (oder GET mit Query-Parameter)
  - `hx-trigger="input changed delay:400ms"`
  - `hx-target="#duplicate-hint"` (ein leerer div unterhalb des Feldes)

**Hinweis-Block (#duplicate-hint):**
- Standardmäßig leer und unsichtbar
- Erscheint bei Treffern als sanfter Info-Block (z.B. gelber/blauer Hintergrund, kein Rot)
- Aufbau:
  ```
  Hinweis: Ähnliche Rezepte wurden gefunden:
  • Dinkelbrot (3★) → [Link zur Detailansicht]
  • Dinkelvollkornbrot (nicht bewertet) → [Link]
  ```
- Maximal 3 Treffer werden angezeigt

**Endpunkt:**
- `GET /recipes/check-duplicate?title=...&exclude_id=...` (exclude_id beim Bearbeiten)
- Gibt ein HTML-Fragment zurück (Askama-Template)
- Leere Antwort wenn keine Treffer → Hinweis-Block wird geleert

---

## 5. Nicht-funktionale Anforderungen

### Performance
- HTMX-Anfrage antwortet in < 200ms
- Debounce von mindestens 300ms verhindert Serverüberlastung

### Browser-Support
- Aktuelle Chrome, Firefox, Safari, Edge Versionen
- iOS Safari und Android Chrome (letzte 2 Versionen)
- Ohne JavaScript: kein Hinweis erscheint, Speichern bleibt vollständig möglich (Progressive Enhancement)

### Barrierefreiheit
- WCAG 2.1 Level A konform
- ARIA-Live-Region für dynamisch erscheinenden Hinweis (`aria-live="polite"`)
- Fokus-Indikatoren auf Links zu ähnlichen Rezepten sichtbar

---

## 6. Teststrategie

### E2E-Tests (Playwright)

**Testfall 1: Duplikat-Hinweis erscheint bei ähnlichem Titel**
```gherkin
Given Ein Rezept "Dinkelbrot" existiert in der Datenbank
And Benutzer öffnet das Formular zum Erstellen eines neuen Rezepts
When Benutzer gibt "Dinkel" in das Titelfeld ein
And Benutzer wartet 500ms
Then Ein Hinweis mit "Ähnliche Rezepte gefunden" erscheint unter dem Titelfeld
And "Dinkelbrot" ist in der Hinweisliste sichtbar
And Ein klickbarer Link zur Detailansicht von "Dinkelbrot" ist vorhanden
```

**Testfall 2: Hinweis verschwindet bei keiner Übereinstimmung**
```gherkin
Given Ein Rezept "Dinkelbrot" existiert in der Datenbank
And Benutzer sieht einen Duplikat-Hinweis für "Dinkel"
When Benutzer ändert den Titel auf "Spaghetti Bolognese"
And Benutzer wartet 500ms
Then Der Duplikat-Hinweis ist nicht mehr sichtbar
```

**Testfall 3: Kein Hinweis bei kurzem Titel**
```gherkin
Given Ein Rezept "Dinkelbrot" existiert in der Datenbank
And Benutzer öffnet das Formular zum Erstellen eines neuen Rezepts
When Benutzer gibt "Di" in das Titelfeld ein
And Benutzer wartet 500ms
Then Kein Duplikat-Hinweis erscheint
```

**Testfall 4: Aktuelles Rezept wird beim Bearbeiten nicht als Duplikat angezeigt**
```gherkin
Given Ein Rezept "Dinkelbrot" existiert in der Datenbank
And Benutzer öffnet das Bearbeitungsformular für "Dinkelbrot"
When Benutzer klickt in das Titelfeld (Titel bleibt "Dinkelbrot")
And Benutzer wartet 500ms
Then "Dinkelbrot" erscheint nicht im Duplikat-Hinweis
```

**Testfall 5: Speichern trotz Duplikat-Hinweis möglich**
```gherkin
Given Ein Rezept "Dinkelbrot" existiert in der Datenbank
And Benutzer gibt "Dinkelbrot 2" in das Titelfeld ein
And Ein Duplikat-Hinweis ist sichtbar
When Benutzer füllt das Formular aus und klickt auf "Speichern"
Then Das neue Rezept "Dinkelbrot 2" wird angelegt
And Benutzer wird zur Detailansicht des neuen Rezepts weitergeleitet
```

---

## 7. Abhängigkeiten & Rahmenbedingungen

### Abhängigkeiten

- **Story 01 (Rezept erstellen)** muss abgeschlossen sein - das Rezept-Erstellformular muss existieren
- **Story 02 (Rezept bearbeiten)** muss abgeschlossen sein - die Duplikaterkennung gilt auch im Bearbeitungsformular
- Ergänzt Story 22 (Dubletten-Prüfung und Übersicht) als präventive Maßnahme

### Rahmenbedingungen

- SQLite-Datenbank muss existieren und erreichbar sein
- Keine Authentifizierung erforderlich (LAN-only)
- HTMX ist bereits im Projekt integriert
- Duplikaterkennung ist informativ, nicht blockierend - Last-write-wins bleibt das Prinzip

---

## Offene Punkte / Fragen

- [ ] Soll die Ähnlichkeitssuche nur einfaches `LIKE '%term%'` verwenden oder eine fortgeschrittenere Methode (z.B. FTS5 oder Levenshtein-Distanz in SQLite)?
- [ ] Wie viele ähnliche Rezepte sollen maximal angezeigt werden (Vorschlag: 3)?

---

**Letzte Aktualisierung:** 2026-03-29
