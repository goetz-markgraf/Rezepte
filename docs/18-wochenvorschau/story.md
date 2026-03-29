# Story 18: Wochenvorschau für geplante Rezepte

**Epic:** Epic 5: Wochenplanung
**Priorität:** MVP Phase 1
**Status:** In Arbeit

---

## 1. Story-Satz

Als **Haushalt** möchte ich eine dedizierte Wochenvorschau-Seite aufrufen können, die alle für die aktuelle Woche geplanten Rezepte übersichtlich nach Wochentagen sortiert darstellt, damit ich **auf einen Blick sehe, was wann diese Woche auf den Tisch kommt** und die Wochenplanung in wenigen Minuten abschließen kann.

---

## 2. Geschäftsbezogene Details

### Kontext

Das Paar plant typischerweise am Mittwoch oder Donnerstag die Mahlzeiten für die kommende Woche. Dazu setzen sie an einzelnen Rezepten ein Zukunftsdatum (`planned_date`). Der Filter "Nächste 7 Tage" auf der Rezeptliste liefert bereits eine gefilterte Ansicht dieser geplanten Rezepte — aber die Wochenvorschau geht einen Schritt weiter: Sie bietet eine dedizierte, übersichtliche Seite, die die Wochenplanung als strukturierten Wochenkalender darstellt.

Die Wochenvorschau löst das Kernproblem: "Was haben wir diese Woche geplant?" soll mit einem Blick — und ohne Scrollen durch die gesamte Rezeptliste — beantwortet werden. Jeder Wochentag erscheint als eigene Zeile, selbst wenn kein Rezept für diesen Tag geplant ist. Das ermöglicht auf Anhieb zu erkennen, welche Tage noch "frei" sind.

Die Seite ist bewusst einfach gehalten: keine Bearbeitungsfunktion, kein Drag & Drop. Nur eine klare, lesbare Übersicht der Woche.

### Nutzergruppe

Beide Partner des Haushalts, gleichberechtigt. Zugriff über Desktop, Tablet und Handy im LAN. Primär genutzt bei der Wochenplanung (Mittwoch/Donnerstag) und als schneller Check zwischendurch ("Was kommt morgen?").

### Business-Value

- **Planung auf einen Blick:** Die Wochenplanung ist in Sekunden überblickbar — kein Scrollen, kein Filtern nötig.
- **Lücken erkennen:** Tage ohne geplantes Gericht sind sofort sichtbar und können noch befüllt werden.
- **Wochenplanung in 2 Minuten:** Die Kombination aus Filter "Nächste 7 Tage" (Rezepte suchen und planen) + Wochenvorschau (Überblick bestätigen) ermöglicht die Kernfunktion: Wochenplanung von 20+ Minuten auf 2 Minuten reduzieren.
- **Copy-Paste-fähig:** Die strukturierte Darstellung nach Wochentagen ermöglicht unkompliziertes Kopieren der Wochenliste in eine externe Planungshilfe (Notizen, Messenger, Einkaufsliste).
- **Vorfreude:** "Do: Spaghetti Bolognese — Fr: Pfannkuchen — Sa: Thai-Curry" erzeugt echte Vorfreude auf die Woche.

### Edge Cases

- **Kein Rezept geplant:** Wenn kein `planned_date` in der laufenden Woche gesetzt ist, zeigt die Seite eine freundliche Meldung ("Für diese Woche noch nichts geplant") statt einer leeren Tabelle.
- **Mehrere Rezepte am selben Tag:** Wenn mehrere Rezepte dasselbe `planned_date` haben, werden sie beide unter dem entsprechenden Wochentag aufgelistet.
- **Vergangene Wochentage:** Die Wochenvorschau zeigt alle 7 Tage der laufenden Woche (Montag–Sonntag), auch wenn einzelne Tage bereits vergangen sind. Ein bereits vergangener Tag mit gesetztem Datum (z.B. "Montag: Linseneintopf") bleibt sichtbar — er zeigt, was diese Woche bereits gegessen wurde.
- **Wochendefinition:** Die Woche beginnt am Montag der aktuellen Kalenderwoche und endet am Sonntag.
- **Rezept ohne Datum:** Rezepte ohne `planned_date` erscheinen nicht in der Wochenvorschau.
- **Rezepte nächste Woche:** Rezepte mit `planned_date` in der nächsten Kalenderwoche erscheinen nicht in der aktuellen Wochenvorschau (gehören in eine andere Woche).

---

## 3. Akzeptanzkriterien

### Funktionale Kriterien

- [ ] **K1: Wochenvorschau-Seite erreichbar**
  - Es gibt eine eigene URL/Seite für die Wochenvorschau (z.B. `/wochenvorschau`)
  - Die Seite ist über einen klickbaren Link in der Navigation erreichbar (z.B. in der Hauptnavigation oder als Button auf der Startseite)
  - Die Seite ist direkt per URL-Bookmark aufrufbar (DeepLink-fähig)

- [ ] **K2: Alle Wochentage werden angezeigt**
  - Die Seite listet alle 7 Wochentage der aktuellen Woche auf (Montag bis Sonntag)
  - Jeder Wochentag erscheint mit seinem Namen und Datum (z.B. "Montag, 30. März")
  - Tage ohne geplantes Rezept erscheinen mit einem Hinweis "Nichts geplant" oder bleiben leer (aber sichtbar)

- [ ] **K3: Geplante Rezepte werden den Wochentagen zugeordnet**
  - Rezepte mit `planned_date` in der aktuellen Woche erscheinen unter dem entsprechenden Wochentag
  - Die Zuordnung ist korrekt (Rezept mit `planned_date` = Mittwoch erscheint unter "Mittwoch")
  - Der Rezeptname ist klickbar und führt zur Detailansicht des Rezepts

- [ ] **K4: Mehrere Rezepte pro Tag**
  - Wenn mehrere Rezepte dasselbe `planned_date` haben, erscheinen alle unter dem entsprechenden Wochentag untereinander

- [ ] **K5: Keine geplanten Rezepte**
  - Wenn für die aktuelle Woche kein `planned_date` gesetzt ist, erscheint eine freundliche Meldung (z.B. "Für diese Woche noch nichts geplant")
  - Die Seite zeigt keine leere Tabelle ohne Erklärung

- [ ] **K6: Aktuelle Woche korrekt berechnet**
  - Die angezeigte Woche ist immer die laufende Kalenderwoche (Montag bis Sonntag)
  - Das "heutige" Datum wird serverseitig berechnet — nicht vom Client

- [ ] **K7: Verlinkung zur Rezeptliste**
  - Auf der Wochenvorschau-Seite gibt es einen Link zurück zur Rezeptliste (z.B. "Zur Rezeptliste" oder per Navigation)

### Nicht-funktionale Kriterien

- [ ] **K8: Performance**
  - Die Seite lädt in < 500ms
  - Keine sichtbare Verzögerung beim Aufrufen der Wochenvorschau

- [ ] **K9: Barrierefreiheit**
  - Semantisches HTML (z.B. `<dl>`, `<ul>` oder `<table>` mit korrekten Rollen)
  - Wochentag-Überschriften sind als Überschriften ausgezeichnet (korrekte Hierarchie)
  - Rezept-Links haben aussagekräftige Labels
  - WCAG 2.1 Level A konform

---

## 4. Technische Planung

### Datenmodell

Kein neues Datenbankfeld erforderlich. Das vorhandene `planned_date`-Feld (`DATE`, optional) in der `recipes`-Tabelle wird genutzt.

Abfrage-Logik:
- `WHERE planned_date >= [Montag der aktuellen Woche] AND planned_date <= [Sonntag der aktuellen Woche]`
- `ORDER BY planned_date ASC, title ASC`
- Das Ergebnis wird serverseitig nach Wochentagen gruppiert

### UI/UX-Spezifikation

**URL:** `/wochenvorschau`

**Layout:**
- Seitenüberschrift: "Wochenvorschau" mit der Kalenderwochen-Angabe (z.B. "KW 14 · 30. März – 5. April 2026")
- Darunter sieben Abschnitte, je einer pro Wochentag
- Jeder Abschnitt zeigt: Wochentag-Name + Datum + (falls vorhanden) die Rezeptnamen als Liste

**Beispiel-Darstellung:**

```
Wochenvorschau  KW 14 · 30. März – 5. April 2026

Montag, 30. März
  Nichts geplant

Dienstag, 31. März
  • Spaghetti Bolognese

Mittwoch, 1. April
  Nichts geplant

Donnerstag, 2. April
  • Pfannkuchen
  • Rührei (Frühstück)

Freitag, 3. April
  Nichts geplant

Samstag, 4. April
  • Thai-Curry

Sonntag, 5. April
  • Pizza Margherita
```

**Navigation:**
- Link zur Wochenvorschau in der Hauptnavigation oder als prominenter Link auf der Startseite
- Link von der Wochenvorschau zurück zur Rezeptliste

**Verhalten:**
- Seite ist statisch (kein HTMX-Update nötig) — vollständiger Server-Render beim Aufruf
- Keine Inline-Bearbeitungsfunktion auf dieser Seite

---

## 5. Nicht-funktionale Anforderungen

### Performance
- Seite lädt in < 500ms (NFR-P2)
- Einzelne SQLite-Abfrage für die Woche — kein Performance-Problem bei bis zu 200 Rezepten

### Browser-Support
- Aktuelle Chrome, Firefox, Safari, Edge Versionen
- iOS Safari und Android Chrome (letzte 2 Versionen)
- Responsive Design für Desktop, Tablet und Mobile

### Barrierefreiheit
- WCAG 2.1 Level A konform
- Semantisches HTML für Wochentag-Struktur
- Fokus-Indikatoren sichtbar auf Rezept-Links
- Screenreader-kompatible Struktur

---

## 6. Teststrategie

### E2E-Tests (Playwright)

**Testfall 1: Wochenvorschau mit geplanten Rezepten**
```gherkin
Given "Spaghetti Bolognese" hat planned_date = nächsten Dienstag
And "Pizza Margherita" hat planned_date = nächsten Sonntag
When Benutzer öffnet /wochenvorschau
Then "Spaghetti Bolognese" erscheint unter "Dienstag"
And "Pizza Margherita" erscheint unter "Sonntag"
And alle anderen Wochentage zeigen "Nichts geplant"
```

**Testfall 2: Keine geplanten Rezepte**
```gherkin
Given Kein Rezept hat ein planned_date in der aktuellen Woche
When Benutzer öffnet /wochenvorschau
Then Eine freundliche Meldung "Für diese Woche noch nichts geplant" wird angezeigt
And keine leere Tabelle ohne Erklärung ist sichtbar
```

**Testfall 3: Mehrere Rezepte am selben Tag**
```gherkin
Given "Pfannkuchen" hat planned_date = nächsten Donnerstag
And "Rührei" hat planned_date = nächsten Donnerstag
When Benutzer öffnet /wochenvorschau
Then Unter "Donnerstag" erscheinen sowohl "Pfannkuchen" als auch "Rührei"
```

**Testfall 4: Rezeptname ist klickbar**
```gherkin
Given "Spaghetti Bolognese" hat planned_date in der aktuellen Woche
When Benutzer öffnet /wochenvorschau
And klickt auf "Spaghetti Bolognese"
Then Benutzer wird zur Detailansicht von "Spaghetti Bolognese" weitergeleitet
```

**Testfall 5: DeepLink**
```gherkin
Given Die App enthält Rezepte mit planned_date in der aktuellen Woche
When Benutzer ruft die URL /wochenvorschau direkt auf (Bookmark)
Then Die Wochenvorschau der aktuellen Woche wird korrekt angezeigt
```

---

## 7. Abhängigkeiten & Rahmenbedingungen

### Abhängigkeiten

- Story 28 (Datum-Eingabe am Rezept) muss implementiert sein — `planned_date`-Feld muss in der DB existieren und befüllbar sein
- Story 10 (Filter "Nächste 7 Tage") sollte implementiert sein — baut auf demselben Konzept auf (geplante Rezepte anzeigen)
- Story 04 (Rezept-Detailansicht) muss implementiert sein — Rezeptlinks verweisen auf Detailansicht

### Rahmenbedingungen

- Das `planned_date`-Feld ist optional; Rezepte ohne Datum erscheinen nicht in der Wochenvorschau
- Die Woche beginnt montags (ISO-Wochenstandard)
- Das "heutige" Datum wird serverseitig berechnet
- SQLite-Datenbank muss existieren und erreichbar sein
- Keine Authentifizierung erforderlich (LAN-only)
- DeepLink-fähige URL für die Wochenvorschau

---

## Offene Punkte / Fragen

- [ ] Soll die Wochenvorschau nur die aktuelle Woche zeigen, oder soll man per Navigation zur nächsten/vorherigen Woche wechseln können? → Für MVP: nur aktuelle Woche

---

**Letzte Aktualisierung:** 2026-03-29
