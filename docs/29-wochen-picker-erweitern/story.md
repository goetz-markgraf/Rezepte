# Story 29: Wochen-Picker erweitern

**Epic:** Epic 5: Wochenplanung
**Priorität:** MVP Phase 1
**Status:** In Arbeit

---

## 1. Story-Satz

Als **Haushalt** möchte ich im Wochen-Picker灵活的 die nächsten 10 Tage mit Datumsauswahl sehen können (beginnend ab morgen), damit ich die Wochenplanung flexibel gestalten und auf einen Blick sehen kann, welches Rezept an welchem Tag geplant ist.

---

## 2. Geschäftsbezogene Details

### Kontext

Der bestehende Wochentag-Picker aus Story 16 zeigt 7 Buttons (Mo, Di, Mi, ...) für die "nächste Woche" (ab dem nächsten Montag). Dies ist für die flexible Wochenplanung limiting, weil:
- Wenn heute Mittwoch ist, beginnt die Auswahl erst ab dem kommenden Montag (5 Tage "verloren")
- Die Buttons zeigen nur den Wochentag, nicht das konkrete Datum - der Nutzer muss im Kopf umrechnen
- 7 Tage sind oft zu wenig, um die komplette Woche zu planen

Die Erweiterung soll diese Limitationen beheben und die Planung flexibler machen.

### Nutzergruppe

Der gemeinsame Zwei-Personen-Haushalt. Beide Partner nutzen die App gleichberechtigt — primär bei der Wochenplanung (Mittwoch/Donnerstag).

### Business-Value

- **Flexiblere Planung:** Der Picker beginnt beim nächsten Tag (nicht erst beim Montag), sodass auch kurzfristige Planung möglich ist
- **Mehr Kontext:** Das Datum wird immer angezeigt (z.B. "Mo 30.03"), sodass keine mentale Umrechnung nötig ist
- **Längere Planungsperiode:** 10 Tage statt 7 ermöglichen die Planung einer vollständigen Woche plus ein paar Extra-Tage

### Edge Cases

- **Wochenende in der Auswahl:** Wenn die 10 Tage über ein Wochenende reichen, ist das kein Problem — alle Tage werden angezeigt
- **Monatswechsel:** Das Datum zeigt korrekt den Monatstag (z.B. "30.03", "01.04")
- **Jahreswechsel:** Wenn die 10 Tage über den Jahreswechsel reichen, wird das Jahr im Datum angezeigt (z.B. "30.12", "01.01")
- **Aktiver Tag wird angezeigt:** Der aktuell gewählte Tag (falls das planned_date in den nächsten 10 Tagen liegt) ist visuell hervorgehoben

---

## 3. Akzeptanzkriterien

### Funktionale Kriterien

- [ ] **K1: Picker zeigt 10 Tage**
  - Es werden 10 aufeinanderfolgende Tage angezeigt, beginnend mit dem morgigen Tag
  - Jeder Tag zeigt: Wochentag-Kürzel + Datum ohne Jahr (Format: "Mo 30.03")
  - Die Tage sind von morgen bis zum 10. Tag in der Zukunft

- [ ] **K2: Klick auf Tag setzt das Datum**
  - Ein Klick auf einen der 10 Tage berechnet das korrekte Datum
  - Das Datum wird im deutschen Format (`T.M.JJJJ`) in das Datumsfeld eingetragen
  - Der geklickte Tag wird visuell als "aktiv" markiert

- [ ] **K3: Beginn bei morgen (nicht Montag)**
  - Der erste Tag im Picker ist immer "morgen", nicht der nächste Montag
  - Beispiel: Wenn heute Mittwoch ist, zeigt der erste Tag "Do 30.03" (morgen), nicht "Mo 30.03"

- [ ] **K4: Datum im Format TT.MM angezeigt**
  - Jeder Tag zeigt das Datum ohne Jahreszahl (z.B. "Mo 30.03", "Di 31.03")
  - Bei Monatswechsel wird korrekt der neue Monat angezeigt (z.B. "Mo 30.03", "Di 01.04")

- [ ] **K5: Aktiver Tag wird hervorgehoben**
  - Wenn ein Rezept bereits ein `planned_date` hat, das innerhalb der nächsten 10 Tage liegt, wird der entsprechende Tag beim Öffnen des Formulars als aktiv markiert
  - Der aktive Tag ist visuell deutlich erkennbar (z.B. andere Hintergrundfarbe)

- [ ] **K6: Erneuter Klick auf aktiven Tag leert das Datum**
  - Ein erneuter Klick auf den aktiven Tag entfernt das Datum aus dem Feld
  - Alle Tags werden demarkiert

- [ ] **K7: Kompatibilität mit bestehendem planned_date**
  - Die Erweiterung funktioniert mit dem bestehenden `planned_date`-Feld
  - Manuelle Eingabe im Datumsfeld demarkiert alle Tags

### Nicht-funktionale Kriterien

- [ ] **K8: Performance**
  - Die Berechnung der 10 Tage erfolgt clientseitig per JavaScript
  - Reaktion auf Klick ist sofort (< 100ms wahrnehmbare Verzögerung)

- [ ] **K9: Barrierefreiheit**
  - Jeder Tag hat ein aussagekräftiges `aria-label` (z.B. `aria-label="Donnerstag, 30. März wählen"`)
  - Aktiver Zustand wird per `aria-pressed="true"` kommuniziert
  - Alle Elemente sind per Tastatur bedienbar (Tab + Enter/Space)
  - WCAG 2.1 Level A konform

---

## 4. Technische Planung

### Datenmodell

Keine Datenbank-Änderungen erforderlich. Das `planned_date`-Feld in der `recipes`-Tabelle existiert bereits.

### UI/UX-Spezifikation

**Platzierung im Formular:**

```
Datum (geplant / gekocht)
[ 02.04.2026          ] [Kalender-Icon]

Mo 30.03  Di 31.03  Mi 01.04  Do 02.04  Fr 03.04  Sa 04.04  So 05.04  Mo 06.04  Di 07.04  Mi 08.04
```

**Änderungen gegenüber Story 16:**

1. **10 statt 7 Buttons:** Alle 10 Tage werden angezeigt
2. **Datum im Button:** Jeder Button zeigt "Wochentag + Datum" (z.B. "Mo 30.03")
3. **Beginn bei morgen:** Der erste Tag ist immer "morgen", nicht der nächste Montag

**Berechnung der 10 Tage (JavaScript):**

```
Für i = 1 bis 10:
  tag = heute + i Tage
  wochentagKuerzel = locale-kurzer Wochentag (Mo, Di, ...)
  datum = tag.getDate() + "." + (tag.getMonth() + 1) + "."
  buttonText = wochentagKuerzel + " " + datum
```

**Interaktionslogik:**

```
Beim Klick auf Tag "X":
  1. Das Datum des Tags berechnen
  2. Datum im Format T.M.JJJJ ins Textfeld schreiben
  3. Geklickten Tag als "aktiv" markieren

Beim erneuten Klick auf aktiven Tag:
  1. Textfeld leeren
  2. Alle Tags demarkieren

Beim Öffnen des Formulars:
  1. Wenn planned_date existiert und in den nächsten 10 Tagen liegt:
     - Den entsprechenden Tag als aktiv markieren
```

**Progressive Enhancement:**

- Die 10 Tage werden initial mit `display: none` gerendert oder per JS eingefügt
- JavaScript macht sie sichtbar und fügt Event-Listener hinzu
- Ohne JS: Nur das Textfeld und der Kalender-Icon sind sichtbar

---

## 5. Nicht-funktionale Anforderungen

### Performance

- Seite lädt ohne sichtbare Verzögerung (< 500ms)
- Datumsberechnung erfolgt clientseitig, keine Server-Anfrage
- Kein zusätzlicher Server-Overhead durch das Feature

### Browser-Support

- Aktuelle Chrome, Firefox, Safari, Edge Versionen
- iOS Safari und Android Chrome (letzte 2 Versionen)
- Ohne JavaScript: vollständiger Fallback auf Texteingabe

### Barrierefreiheit

- WCAG 2.1 Level A konform
- `aria-pressed` für aktiven Zustand
- Aussagekräftige `aria-label`-Attribute
- Fokus-Indikatoren sichtbar
- Tastatur-Navigation vollständig funktionsfähig

---

## 6. Teststrategie

### E2E-Tests (Playwright)

**Testfall 1: Picker zeigt 10 Tage ab morgen**
```gherkin
Given Heute ist Mittwoch, 29.03.2026
When Der Nutzer die Rezept-Bearbeiten-Seite öffnet
Then Werden 10 Tage angezeigt: Do 30.03 bis Sa 08.04
```

**Testfall 2: Klick auf Tag setzt korrektes Datum**
```gherkin
Given Heute ist Mittwoch, 29.03.2026
When Der Nutzer auf "Fr 03.04" klickt
Then Enthält das Datumsfeld "3.4.2026"
And Der Tag "Fr 03.04" ist als aktiv markiert
```

**Testfall 3: Monatswechsel wird korrekt angezeigt**
```gherkin
Given Heute ist Montag, 30.03.2026
When Der Nutzer die Rezept-Bearbeiten-Seite öffnet
Then Zeigt der 2. Tag "Di 31.03"
And Zeigt der 3. Tag "Mi 01.04"
```

**Testfall 4: Vorhandenes planned_date in den nächsten 10 Tagen**
```gperkin
Given Ein Rezept hat planned_date = übermorgen (Freitag)
And Heute ist Mittwoch
When Der Nutzer die Bearbeiten-Seite öffnet
Then Ist der Tag "Fr [übermorgen]" als aktiv markiert
```

**Testfall 5: Erneuter Klick auf aktiven Tag leert das Datum**
```gherkin
Given Der Nutzer hat einen Tag gewählt und das Datumsfeld zeigt ein Datum
When Der Nutzer erneut auf denselben Tag klickt
Then Ist das Datumsfeld leer
And Kein Tag ist als aktiv markiert
```

**Testfall 6: Ohne JavaScript — Fallback funktioniert**
```gherkin
Given JavaScript ist deaktiviert
When Der Nutzer die Rezept-Bearbeiten-Seite öffnet
Then Sind keine Tag-Buttons sichtbar
And Das Datumsfeld (Texteingabe) ist vorhanden und funktioniert
```

---

## 7. Abhängigkeiten & Rahmenbedingungen

### Abhängigkeiten

- Story 16 (Wochentag-Picker) muss implementiert sein — zeigt die Grundfunktionalität des Pickers
- Story 28 (Datum-Eingabe am Rezept) muss implementiert sein — das `planned_date`-Feld existiert bereits

### Rahmenbedingungen

- SQLite-Datenbank muss existieren und erreichbar sein
- Keine Authentifizierung erforderlich (LAN-only)
- Vanilla JavaScript — kein Framework, kein Build-Step
- Das Datumsfeld wird weiterhin im deutschen Format `T.M.JJJJ` an den Server gesendet

---

## Offene Punkte / Fragen

- [ ] Soll der Picker auch auf der Rezeptliste/Detailansicht als Schnellaktion erscheinen, oder nur im Edit-Formular? (MVP: nur im Formular)

---

**Letzte Aktualisierung:** 2026-03-30