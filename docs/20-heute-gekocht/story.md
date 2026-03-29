# Story 20: "Heute gekocht" Ansicht mit Highlight

**Epic:** Epic 5: Wochenplanung
**Priorität:** Post-MVP (Growth Feature)
**Status:** In Arbeit

---

## 1. Story-Satz

Als **Haushalt** möchte ich beim Abendessen schnell sehen, welches Rezept für heute geplant ist — und dieses direkt bewerten können, damit ich **spontan während des Essens eine Bewertung abgeben kann und der Überblick über gestern, heute und morgen jederzeit griffbereit ist**.

---

## 2. Geschäftsbezogene Details

### Kontext

Beim Abendessen am Donnerstag fragen Anna und Dragon: "War das wirklich gut — haben wir das schon bewertet?" Sie öffnen die App, möchten sofort das heutige Gericht sehen und es direkt bewerten können — ohne erst zu suchen oder zu scrollen.

Die "Heute gekocht"-Ansicht liefert genau diesen Kontext: Das heutige Gericht ist visuell hervorgehoben, und die Rezepte von gestern und morgen sind ebenfalls sichtbar. So entsteht ein natürlicher Tages-Kontext rund um das aktuelle Datum.

Das Feature baut auf dem bestehenden `planned_date`-Feld auf (Story 28). Ein Rezept mit `planned_date = heute` ist das "heutige Gericht". Die Ansicht ist primär für die spontane Nutzung beim Kochen oder Essen gedacht — am Handy, schnell, ohne Umwege.

### Nutzergruppe

Beide Partner des Haushalts, gleichberechtigt. Primär am Handy, beim Kochen oder Essen. Sekundär am Tablet oder Laptop als schneller Check.

### Business-Value

- **Spontane Bewertung als Gewohnheit:** Das heutige Gericht direkt beim Essen zu sehen und zu bewerten senkt die Hürde enorm — keine Navigation, kein Suchen. Bewertungen werden zur natürlichen Gewohnheit.
- **Tages-Kontext mit einem Blick:** Gestern (was hatten wir?), heute (was ist das aktuelle Gericht?), morgen (was kommt als nächstes?) — alles auf einer Seite.
- **Verbindung zur Wochenvorschau:** Die "Heute gekocht"-Ansicht ergänzt die Wochenvorschau (Story 18/19) als täglicher Einstiegspunkt während der Woche.
- **Bessere Datenqualität:** Mehr Bewertungen durch einfacheren Zugang verbessern die Nützlichkeit der Filter "Beliebtheit" und "Länger nicht gemacht".

### Edge Cases

- **Kein Rezept für heute geplant:** Die Seite zeigt eine freundliche Meldung ("Für heute noch kein Rezept geplant") statt einer leeren Ansicht. Gestern und morgen werden trotzdem angezeigt, wenn dort Rezepte geplant sind.
- **Mehrere Rezepte für heute:** Wenn mehrere Rezepte dasselbe `planned_date = heute` haben, werden alle unter "Heute" angezeigt und hervorgehoben.
- **Kein Rezept für gestern oder morgen:** Wenn für gestern oder morgen kein Rezept geplant ist, erscheint der jeweilige Bereich mit einem Hinweis "Nichts geplant" oder wird kompakt leer dargestellt.
- **Bereits bewertetes Rezept:** Das Rezept kann erneut bewertet werden (Bewertung wird überschrieben). Die aktuelle Bewertung wird angezeigt.
- **Rezept ohne planned_date:** Rezepte ohne Datum erscheinen nicht in dieser Ansicht.

---

## 3. Akzeptanzkriterien

### Funktionale Kriterien

- [ ] **K1: "Heute gekocht"-Seite erreichbar**
  - Es gibt eine eigene URL/Seite (z.B. `/heute`)
  - Die Seite ist über einen klickbaren Link in der Hauptnavigation erreichbar
  - Die Seite ist per Bookmark direkt aufrufbar (DeepLink-fähig)

- [ ] **K2: Heutiges Gericht ist hervorgehoben**
  - Rezepte mit `planned_date = heute` werden visuell hervorgehoben (z.B. durch farbliche Markierung, größere Darstellung oder ein "Heute"-Label)
  - Der Rezeptname ist klickbar und führt zur Detailansicht

- [ ] **K3: Gestern und morgen werden angezeigt**
  - Rezepte für gestern (`planned_date = heute - 1`) erscheinen in einem Bereich "Gestern"
  - Rezepte für morgen (`planned_date = heute + 1`) erscheinen in einem Bereich "Morgen"
  - Jeder Bereich zeigt den Wochentag und das Datum (z.B. "Gestern, Mittwoch 1. April")

- [ ] **K4: Keine geplanten Rezepte — freundliche Meldung**
  - Wenn für heute kein `planned_date` gesetzt ist, erscheint eine freundliche Meldung (z.B. "Für heute noch kein Rezept geplant")
  - Die Seite zeigt keine leere Ansicht ohne Erklärung

- [ ] **K5: Inline-Bewertung direkt auf der Seite**
  - Für jedes angezeigte Rezept (gestern, heute, morgen) ist eine direkte Sternebewertung (3–5 Sterne) möglich, ohne den Edit-Mode zu öffnen
  - Nach dem Tippen auf eine Sternzahl wird die Bewertung sofort gespeichert (ohne Seiten-Reload oder mit HTMX-Update)
  - Die aktuelle Bewertung des Rezepts wird angezeigt (falls bereits bewertet)

- [ ] **K6: Link zur Detailansicht und zum Bearbeiten**
  - Jedes Rezept ist mit einem Link zur Detailansicht versehen
  - Von der Detailansicht kann der Bearbeiten-Modus erreicht werden

- [ ] **K7: Datum wird serverseitig berechnet**
  - "Heute", "Gestern" und "Morgen" werden serverseitig aus dem aktuellen Datum berechnet
  - Kein Client-seitiges Datum-Handling

### Nicht-funktionale Kriterien

- [ ] **K8: Performance**
  - Die Seite lädt in < 500ms
  - Die Inline-Bewertung wird in < 500ms gespeichert und angezeigt

- [ ] **K9: Barrierefreiheit**
  - Sternebewertung ist per Tastatur bedienbar
  - Die Hervorhebung des heutigen Gerichts ist nicht ausschließlich über Farbe erkennbar (z.B. zusätzliches Label oder Icon)
  - Alle Links und interaktiven Elemente haben aussagekräftige Labels (WCAG 2.1 Level A)
  - Semantisches HTML für die Tages-Struktur

---

## 4. Technische Planung

### Datenmodell

Kein neues Datenbankfeld erforderlich. Das vorhandene `planned_date`-Feld aus Story 28 wird genutzt.

Abfrage-Logik:
- Gestern: `WHERE planned_date = [heute - 1 Tag]`
- Heute: `WHERE planned_date = [heute]`
- Morgen: `WHERE planned_date = [heute + 1 Tag]`

Das aktuelle Datum wird serverseitig ermittelt. Alle drei Abfragen können in einer einzelnen SQL-Query zusammengefasst werden:
`WHERE planned_date >= [heute - 1] AND planned_date <= [heute + 1]`

### UI/UX-Spezifikation

**URL:** `/heute`

**Layout:**

```
Heute gekocht           Donnerstag, 2. April 2026

Gestern, Mittwoch 1. April
  Thai-Curry  ★★★★★

[Heute]  Donnerstag, 2. April
  Spaghetti Bolognese   ★★★★☆
  [Bewertung ändern: ★ ★★ ★★★ ★★★★ ★★★★★]

Morgen, Freitag 3. April
  Pfannkuchen  (noch nicht bewertet)
```

**Hervorhebung:**
- Der "Heute"-Bereich hat eine visuelle Abhebung (z.B. farbiger Hintergrund, Rahmen oder prominentes "Heute"-Label)
- Gestern und morgen sind in einer ruhigeren, weniger auffälligen Darstellung

**Inline-Bewertung:**
- Für jedes Rezept erscheinen klickbare Stern-Symbole (1–5)
- Ein Klick auf einen Stern speichert die Bewertung sofort via HTMX-POST
- Die aktuelle Bewertung wird visuell angezeigt (gefüllte vs. leere Sterne)
- Bewertungen unter 3 Sternen sind technisch möglich, aber die App empfiehlt 3–5 Sterne (keine Einschränkung nötig)

**Navigation:**
- Link in der Hauptnavigation: "Heute" oder "Heute gekocht"
- Link zurück zur Wochenvorschau und zur Rezeptliste

**Verhalten:**
- Inline-Bewertung per HTMX (kein Seitenreload)
- Seite selbst: vollständiger Server-Render beim Aufruf

---

## 5. Nicht-funktionale Anforderungen

### Performance
- Seite lädt in < 500ms
- Inline-Bewertung: Speicherung und Anzeige in < 500ms
- Einzelne SQLite-Abfrage mit Datumsbereich — kein Performance-Problem bei bis zu 200 Rezepten

### Browser-Support
- Aktuelle Chrome, Firefox, Safari, Edge Versionen
- iOS Safari und Android Chrome (letzte 2 Versionen)
- Responsive Design für Desktop, Tablet und Mobile — Primär-Usecase: Handy beim Essen

### Barrierefreiheit
- WCAG 2.1 Level A konform
- Inline-Bewertung per Tastatur bedienbar
- Hervorhebung des heutigen Gerichts nicht nur über Farbe
- Fokus-Indikatoren sichtbar
- Screenreader-kompatible Sternebewertung (aria-label für Stern-Buttons)

---

## 6. Teststrategie

### E2E-Tests (Playwright)

**Testfall 1: Heutiges Gericht wird hervorgehoben**
```gherkin
Given "Spaghetti Bolognese" hat planned_date = heute
When Benutzer öffnet /heute
Then "Spaghetti Bolognese" erscheint hervorgehoben im "Heute"-Bereich
And ist als Link zur Detailansicht klickbar
```

**Testfall 2: Gestern und morgen werden angezeigt**
```gherkin
Given "Thai-Curry" hat planned_date = gestern
And "Pfannkuchen" hat planned_date = morgen
And "Spaghetti Bolognese" hat planned_date = heute
When Benutzer öffnet /heute
Then "Thai-Curry" erscheint unter "Gestern"
And "Spaghetti Bolognese" erscheint hervorgehoben unter "Heute"
And "Pfannkuchen" erscheint unter "Morgen"
```

**Testfall 3: Kein Rezept für heute**
```gherkin
Given Kein Rezept hat planned_date = heute
When Benutzer öffnet /heute
Then Eine freundliche Meldung "Für heute noch kein Rezept geplant" wird angezeigt
And keine leere Ansicht ohne Erklärung ist sichtbar
```

**Testfall 4: Inline-Bewertung**
```gherkin
Given "Spaghetti Bolognese" hat planned_date = heute und noch keine Bewertung
When Benutzer öffnet /heute
And tippt auf 5 Sterne beim heutigen Rezept
Then die Bewertung "5 Sterne" wird sofort gespeichert und angezeigt
And kein Seiten-Reload findet statt
```

**Testfall 5: Mehrere Rezepte für heute**
```gherkin
Given "Spaghetti Bolognese" und "Salat" haben beide planned_date = heute
When Benutzer öffnet /heute
Then beide Rezepte erscheinen hervorgehoben unter "Heute"
```

**Testfall 6: DeepLink**
```gherkin
Given Die App ist erreichbar
When Benutzer ruft die URL /heute direkt auf (Bookmark)
Then Die "Heute gekocht"-Ansicht wird korrekt angezeigt
```

---

## 7. Abhängigkeiten & Rahmenbedingungen

### Abhängigkeiten
- Story 28 (Datum-Eingabe am Rezept) muss implementiert sein — `planned_date`-Feld muss in der DB existieren
- Story 04 (Rezept-Detailansicht) muss implementiert sein — Rezeptlinks verweisen auf Detailansicht
- Story 17 (Inline-Bewertung ohne Edit-Mode) sollte als Referenz-Implementierung bekannt sein — die Bewertungslogik kann analog übernommen werden
- Story 18/19 (Wochenvorschau) sind implementiert — "Heute"-Ansicht ergänzt die Wochenvorschau

### Rahmenbedingungen
- Das `planned_date`-Feld ist optional; Rezepte ohne Datum erscheinen nicht in dieser Ansicht
- "Heute", "Gestern" und "Morgen" werden serverseitig berechnet — kein Client-seitiges Datum
- SQLite-Datenbank muss existieren und erreichbar sein
- Keine Authentifizierung erforderlich (LAN-only)
- DeepLink-fähige URL für die Ansicht
- Inline-Bewertung nutzt HTMX für reaktive Updates ohne Seitenreload

---

## Offene Punkte / Fragen

- [ ] Soll die Navigation "Heute" in der Hauptnavigation dauerhaft sichtbar sein, oder nur als Schnellzugriff auf der Wochenvorschau-Seite? → Bevorzugt: dauerhaft in der Hauptnavigation (da primärer Usecase beim Essen)
- [ ] Soll auch `cooked_date` (falls vorhanden) als Alternative zu `planned_date` ausgewertet werden? → Zunächst nur `planned_date`, da `cooked_date` ggf. rückwirkend gesetzt wird

---

**Letzte Aktualisierung:** 2026-03-29
