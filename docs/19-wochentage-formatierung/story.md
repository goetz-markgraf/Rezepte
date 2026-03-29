# Story 19: Wochenvorschau nach Wochentagen formatiert

**Epic:** Epic 5: Wochenplanung
**Priorität:** MVP Phase 1
**Status:** In Arbeit

---

## 1. Story-Satz

Als **Haushalt** möchte ich in der Wochenvorschau den Wochentag-Namen und das Datum visuell klar getrennt und den heutigen Tag hervorgehoben sehen, damit ich **auf einen Blick den aktuellen Wochentag erkenne und die Woche schneller überfliegen kann**.

---

## 2. Geschäftsbezogene Details

### Kontext

Story 18 hat die Wochenvorschau grundlegend implementiert: Alle 7 Tage der Woche werden als `<dl>`-Liste dargestellt, jeder Tag mit `datum_anzeige` (z.B. "Montag, 30. März") und darunter die geplanten Rezepte. Die Darstellung ist funktional korrekt, aber noch nicht optimal scanbar.

Das Problem: "Montag, 30. März" und "Dienstag, 31. März" sehen visuell identisch aus. Der heutige Tag ist nicht hervorgehoben. Beim schnellen Durchscrollen muss man erst lesen, welcher Tag heute ist, bevor man weiß, wo man sich in der Woche befindet.

Story 19 verfeinert die visuelle Formatierung der Wochentag-Einträge:
- Der **Wochentag-Name** (z.B. "Montag") ist der primäre, prominent angezeigte Bezeichner
- Das **Datum** (z.B. "30. März") ist sekundär und kleiner/schwächer dargestellt
- Der **heutige Tag** ist visuell hervorgehoben, damit er sofort erkennbar ist

Diese Verbesserung macht die Wochenvorschau zum echten "Wochenkalender-Gefühl" — statt einer simplen Liste von Datum+Rezept-Einträgen.

### Nutzergruppe

Beide Partner des Haushalts. Primär genutzt beim morgendlichen Check ("Was gibt es heute?") sowie bei der Wochenplanung. Der heutige Tag soll dabei sofort auffallen.

### Business-Value

- **Schnelleres Scannen:** "Montag", "Dienstag" usw. als primäre Beschriftung ist leichter zu überfliegen als "Montag, 30. März" als einheitlicher Block.
- **"Was gibt es heute?"** — der häufigste Use Case. Mit visueller Hervorhebung des heutigen Tags ist die Antwort sofort sichtbar, ohne scrollen oder lesen zu müssen.
- **Wochengefühl:** Die separate Darstellung von Wochentag-Name und Datum vermittelt ein echter Kalender-Feeling und macht die Wochenplanung angenehmer.
- **Orientierung:** Vergangene, heutige und zukünftige Tage unterscheiden sich sichtbar — leere vergangene Tage sind weniger prominent, der heutige Tag sticht hervor.

### Edge Cases

- **Heute ist Montag:** Der erste Wochentag ist hervorgehoben.
- **Heute ist Sonntag:** Der letzte Wochentag ist hervorgehoben.
- **Wochenvorschau wird an einem Wochenende aufgerufen:** Der Samstag oder Sonntag der aktuellen Woche wird korrekt hervorgehoben.
- **Vergangene Tage:** Bereits vergangene Wochentage (z.B. Montag, obwohl heute Mittwoch ist) erhalten keine Heute-Hervorhebung, aber bleiben sichtbar.
- **Heute ohne geplantes Rezept:** Der heutige Tag wird hervorgehoben, auch wenn kein Rezept geplant ist — er zeigt dann "Nichts geplant" mit Hervorhebung.
- **Mehrere Rezepte am heutigen Tag:** Die Hervorhebung des heutigen Tags funktioniert unabhängig von der Anzahl der Rezepte.

---

## 3. Akzeptanzkriterien

### Funktionale Kriterien

- [ ] **K1: Wochentag-Name und Datum sind visuell getrennt**
  - Der Wochentag-Name (z.B. "Montag") ist prominent und groß dargestellt
  - Das Datum (z.B. "30. März") ist daneben oder darunter kleiner / in einer schwächeren Farbe dargestellt
  - Beide Angaben sind in einem `<dt>`-Element (oder mit semantisch sinnvoller Struktur) enthalten

- [ ] **K2: Heutiger Tag ist visuell hervorgehoben**
  - Der Abschnitt des heutigen Wochentags ist optisch klar von den anderen Tagen unterschieden (z.B. durch einen farbigen Rahmen, einen farbigen Hintergrund oder einen "Heute"-Badge)
  - Die Hervorhebung ist auch ohne Farbe erkennbar (z.B. durch Fettschrift oder ein Label "Heute")
  - Die serverseitige Berechnung des heutigen Tages ist korrekt (kein Client-seitiges JavaScript nötig)

- [ ] **K3: Vergangene Tage sind schwächer dargestellt**
  - Bereits vergangene Wochentage (vor heute) sind visuell gedämpft (z.B. leicht ausgegraut oder mit niedrigerer Opacity), um die zeitliche Orientierung zu verbessern
  - Zukünftige Tage und der heutige Tag sind normal oder hervorgehoben dargestellt

- [ ] **K4: Alle bisherigen Akzeptanzkriterien aus Story 18 bleiben erfüllt**
  - Alle 7 Wochentage werden weiterhin angezeigt
  - Rezepte werden korrekt den Wochentagen zugeordnet
  - Rezeptnamen sind weiterhin klickbar
  - "Nichts geplant" erscheint für leere Tage
  - KW-Anzeige im Header bleibt erhalten

### Nicht-funktionale Kriterien

- [ ] **K5: Performance**
  - Die Seite lädt weiterhin in < 500ms
  - Die Berechnung des heutigen Tags erfolgt serverseitig (kein zusätzlicher JS-Request)

- [ ] **K6: Barrierefreiheit**
  - Die Hervorhebung des heutigen Tags ist nicht nur durch Farbe erkennbar (WCAG 1.4.1: Use of Color)
  - Wochentag-Name und Datum haben im Template klar unterschiedliche Rollen (z.B. `<strong>` für den Wochentag-Namen, `<small>` oder `<span>` mit abweichender Klasse für das Datum)
  - WCAG 2.1 Level A weiterhin konform

---

## 4. Technische Planung

### Datenmodell

Kein neues Datenbankfeld erforderlich.

Anpassung der Template-Structs: Die `Wochentag`-Struct benötigt zusätzliche Felder, damit das Template zwischen Wochentag-Name, Datum und "heute"-Status unterscheiden kann:

```
Wochentag {
    wochentag_name: String,   // z.B. "Montag"
    datum_kurz: String,       // z.B. "30. März"
    ist_heute: bool,          // true wenn dieser Tag = today
    ist_vergangen: bool,      // true wenn dieser Tag < today
    rezepte: Vec<WochentagesEintragItem>,
}
```

Das bisherige Feld `datum_anzeige: String` (z.B. "Montag, 30. März") wird durch `wochentag_name` und `datum_kurz` ersetzt — oder `datum_anzeige` bleibt als kombiniertes Feld zusätzlich erhalten, falls es anderswo noch verwendet wird.

Der Handler `wochenvorschau_handler` berechnet `today` bereits serverseitig — die Felder `ist_heute` und `ist_vergangen` können direkt im bestehenden `(0..7).map(...)` Block befüllt werden.

### UI/UX-Spezifikation

**Angepasste Darstellung je Wochentag-Abschnitt:**

```
┌─────────────────────────────────────┐  ← Hervorhebung für "heute"
│  Mittwoch          1. April         │
│  ● Spaghetti Bolognese             │
└─────────────────────────────────────┘

┌─────────────────────────────────────┐  ← Normal für zukünftige Tage
│  Donnerstag        2. April         │
│  Nichts geplant                    │
└─────────────────────────────────────┘

░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░  ← Gedämpft für vergangene Tage
  Montag             30. März
  ● Linseneintopf
░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░
```

Der Wochentag-Name ist größer / fetter als das Datum. Das Datum steht rechts daneben oder in einer kleineren Zeile darunter.

**CSS-Klassen (Erweiterung der bestehenden):**
- `.wochentag-abschnitt.wochentag-heute` — Hervorhebungs-Stil für den heutigen Tag
- `.wochentag-abschnitt.wochentag-vergangen` — Gedämpfter Stil für vergangene Tage
- `.wochentag-name` — Primärer Name, groß und fett
- `.wochentag-datum` — Sekundäres Datum, kleiner und schwächer

**Template-Anpassung:**

Das `<dt>`-Element wird von einem einzelnen `{{ tag.datum_anzeige }}` auf eine strukturiertere Ausgabe umgestellt:

```html
<dt class="wochentag-titel">
    <span class="wochentag-name">{{ tag.wochentag_name }}</span>
    <span class="wochentag-datum">{{ tag.datum_kurz }}</span>
</dt>
```

Die `class` des `<div class="wochentag-abschnitt">` erhält je nach Status zusätzliche Klassen: `wochentag-heute` oder `wochentag-vergangen`.

---

## 5. Nicht-funktionale Anforderungen

### Performance
- Seite lädt in < 500ms (keine Änderung gegenüber Story 18)
- Keine zusätzliche Datenbankabfrage — `ist_heute` und `ist_vergangen` werden im Handler berechnet

### Browser-Support
- Aktuelle Chrome, Firefox, Safari, Edge Versionen
- iOS Safari und Android Chrome (letzte 2 Versionen)
- Responsive Design für Desktop, Tablet und Mobile

### Barrierefreiheit
- WCAG 2.1 Level A konform
- Hervorhebung des heutigen Tags nicht nur per Farbe (WCAG 1.4.1)
- Sinnvolle semantische Struktur für Screenreader

---

## 6. Teststrategie

### E2E-Tests (Playwright)

**Testfall 1: Wochentag-Name und Datum sind getrennt dargestellt**
```gherkin
Given Die Wochenvorschau ist geöffnet
When Benutzer sieht den ersten Wochentag
Then Der Wochentag-Name (z.B. "Montag") ist als separates Element sichtbar
And Das Datum (z.B. "30. März") ist als separates Element sichtbar
```

**Testfall 2: Heutiger Tag ist hervorgehoben**
```gherkin
Given Heute ist ein bestimmter Wochentag (z.B. Mittwoch)
When Benutzer öffnet /wochenvorschau
Then Der Abschnitt für Mittwoch hat die CSS-Klasse "wochentag-heute"
And Die anderen Abschnitte haben diese Klasse nicht
```

**Testfall 3: Vergangene Tage sind gedämpft**
```gherkin
Given Heute ist Mittwoch
When Benutzer öffnet /wochenvorschau
Then Die Abschnitte für Montag und Dienstag haben die CSS-Klasse "wochentag-vergangen"
And Die Abschnitte für Mittwoch bis Sonntag haben diese Klasse nicht
```

**Testfall 4: Alle Story-18-Funktionen bleiben erhalten**
```gherkin
Given Ein Rezept hat planned_date = heute
When Benutzer öffnet /wochenvorschau
Then Das Rezept erscheint unter dem korrekten Wochentag
And Der Rezeptname ist ein klickbarer Link zur Detailansicht
And "Nichts geplant" erscheint bei leeren Tagen
```

---

## 7. Abhängigkeiten & Rahmenbedingungen

### Abhängigkeiten

- Story 18 (Wochenvorschau für geplante Rezepte) muss implementiert sein und abgeschlossen sein — Story 19 baut direkt auf der bestehenden `WochenvorschauTemplate`, `Wochentag`-Struct und `wochenvorschau.html` auf
- Story 28 (Datum-Eingabe am Rezept) muss implementiert sein — `planned_date`-Feld muss in der DB existieren

### Rahmenbedingungen

- Die Berechnung des heutigen Tags erfolgt serverseitig in `wochenvorschau_handler` — kein Client-seitiges JavaScript nötig
- Das Template `wochenvorschau.html` wird angepasst, nicht neu erstellt
- Die bestehende `Wochentag`-Struct in `src/templates.rs` wird um `ist_heute: bool` und `ist_vergangen: bool` sowie separate Felder für Name und Datum erweitert
- SQLite-Datenbank muss existieren und erreichbar sein
- Keine Authentifizierung erforderlich (LAN-only)

---

## Offene Punkte / Fragen

- [ ] Soll der heutige Tag zusätzlich mit einem "Heute"-Label (Text) oder nur durch visuelles Styling hervorgehoben werden? → Empfehlung: visuelles Styling + CSS-Klasse; ein "Heute"-Label ist optional für Barrierefreiheit

---

**Letzte Aktualisierung:** 2026-03-29
