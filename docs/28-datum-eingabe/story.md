# Story 28: Datum-Eingabe am Rezept (geplant / gekocht)

**Epic:** Epic 4: Bewertung & Datums-Tracking
**Priorität:** MVP Phase 1
**Status:** In Arbeit

---

## 1. Story-Satz

Als **Haushalt** möchte ich beim Erfassen und Bearbeiten eines Rezepts ein optionales Datum angeben können (wann das Gericht geplant ist oder zuletzt gekocht wurde), damit ich die Wochenplanung direkt am Rezept festhalten kann und der "Länger nicht gemacht"-Filter sinnvoll funktioniert.

---

## 2. Geschäftsbezogene Details

### Kontext

Bei der Wochenplanung (typischerweise Mittwoch/Donnerstag) wählen die Nutzer Rezepte für die Woche aus und möchten direkt am Rezept festhalten, wann dieses Gericht gemacht wurde oder wann es geplant ist. Das Datum dient zwei Zwecken:

- **Vergangenheitsdatum ("Zuletzt gekocht"):** Wenn ein Rezept bereits zubereitet wurde, wird das Datum eingetragen. Der Filter "Länger nicht gemacht" sortiert Rezepte aufsteigend nach diesem Datum, sodass lange nicht gekochte Gerichte oben erscheinen.
- **Zukunftsdatum ("Geplant am"):** Wenn ein Rezept für einen bestimmten Tag der Woche vorgesehen ist, kann das Datum in die Zukunft gesetzt werden. Der Filter "Nächste 7 Tage" zeigt diese geplanten Gerichte an.

Das Datum ist optional - viele Rezepte werden ohne Datum angelegt und erst nach der ersten Zubereitung mit einem Datum versehen.

### Nutzergruppe

Der gemeinsame Zwei-Personen-Haushalt. Beide Partner greifen gleichberechtigt auf die App zu (Desktop, Tablet, Mobilgerät) ohne Login.

### Business-Value

Das Datum-Tracking ist das zentrale Feature für mehr Variation im Speiseplan. Ohne Datum-Eingabe kann der "Länger nicht gemacht"-Filter nicht sinnvoll genutzt werden. Die einfache Eingabe (Freitext-String oder Date-Picker) senkt die Hürde, das Datum tatsächlich einzutragen - was die Nützlichkeit der gesamten App steigert.

### Edge Cases

- **Kein Datum angegeben:** Das Datum-Feld bleibt leer (`NULL` in der DB). Das Rezept erscheint im "Länger nicht gemacht"-Filter am Ende (nie gemacht = am längsten nicht gemacht, oder wird ausgeblendet - Verhalten folgt in Story 09).
- **Ungültiges Datumsformat:** Der Nutzer gibt einen String ein, der nicht als Datum erkannt werden kann (z.B. "morgen", "nächste Woche"). Das Formular zeigt eine Fehlermeldung, das Rezept wird nicht gespeichert.
- **Datum in der Vergangenheit:** Vollständig erlaubt - entspricht "Zuletzt gekocht".
- **Datum in der Zukunft:** Vollständig erlaubt - entspricht "Geplant am".
- **Zweistelliges Jahr:** `5.3.25` wird als `2025-03-05` interpretiert (aktuelles Jahrhundert).
- **Führende Nullen fehlen:** `5.3.2025` entspricht `05.03.2025` - beide Schreibweisen sind gültig.
- **Datum löschen:** Der Nutzer kann ein bereits eingetragenes Datum wieder entfernen, indem das Feld geleert wird.

---

## 3. Akzeptanzkriterien

### Funktionale Kriterien

- [ ] **K1: Datum im Formular "Neues Rezept"**
  - Das Formular zum Anlegen eines Rezepts enthält ein optionales Datumsfeld mit Label "Datum (geplant / gekocht)"
  - Das Feld ist leer vorbelegt
  - Das Rezept kann ohne Datum angelegt werden

- [ ] **K2: Datum im Formular "Rezept bearbeiten"**
  - Das Bearbeitungsformular zeigt das gespeicherte Datum vorausgefüllt an
  - Wenn kein Datum gespeichert ist, ist das Feld leer
  - Das Datum kann geändert oder gelöscht werden

- [ ] **K3: Texteingabe des Datums**
  - Der Nutzer kann das Datum als Text eingeben im Format `T.M.JJJJ` (z.B. `5.3.2025`)
  - Führende Nullen sind optional: `05.03.2025` und `5.3.2025` werden beide akzeptiert
  - Zweistellige Jahresangaben werden akzeptiert: `5.3.25` entspricht `2025-03-05`
  - Trennzeichen ist der Punkt (`.`)

- [ ] **K4: Date-Picker als Alternative**
  - Neben dem Texteingabefeld befindet sich ein Kalender-Icon
  - Ein Klick auf das Icon öffnet einen nativen Browser-Date-Picker
  - Die Auswahl im Date-Picker füllt das Texteingabefeld aus
  - Vergangenheits- und Zukunftsdaten sind gleichermaßen auswählbar (keine Einschränkung des auswählbaren Bereichs)

- [ ] **K5: Validierung bei ungültigem Format**
  - Wird ein Text eingegeben, der kein gültiges Datum ergibt, zeigt das Formular eine Fehlermeldung ("Kein gültiges Datum. Bitte im Format T.M.JJJJ eingeben.")
  - Das Rezept wird nicht gespeichert
  - Die anderen Formularfelder behalten ihre Werte (kein Datenverlust)

- [ ] **K6: Datum in der Detailansicht**
  - Das gespeicherte Datum wird in der Rezept-Detailansicht angezeigt
  - Anzeige im deutschen Format: `5. März 2025` oder `05.03.2025`
  - Wenn kein Datum gespeichert ist, wird das Feld nicht angezeigt (oder ein Hinweis "noch nicht gekocht")

- [ ] **K7: Datum in der Rezeptliste**
  - Das Datum wird in der Listenansicht pro Rezept angezeigt (kompaktes Format, z.B. `05.03.2025`)
  - Wenn kein Datum gespeichert ist, bleibt die Stelle leer

### Nicht-funktionale Kriterien

- [ ] **K8: Performance**
  - Speichern des Datums erfolgt in < 500ms
  - Seite lädt ohne sichtbare Verzögerung (< 500ms)

- [ ] **K9: Barrierefreiheit**
  - Das Datumsfeld hat ein korrektes `<label>`-Element (WCAG 2.1 Level A)
  - Das Kalender-Icon hat ein `aria-label` oder `title`-Attribut
  - Tastatur-Navigation funktioniert vollständig (Tab-Reihenfolge korrekt)

---

## 4. Technische Planung

### Datenmodell

Das Datenbankschema benötigt ein neues Feld `planned_date` in der `recipes`-Tabelle:

```sql
-- Migration: 00X_add_planned_date.sql
ALTER TABLE recipes ADD COLUMN planned_date DATE;
CREATE INDEX idx_recipes_planned_date ON recipes(planned_date);
```

Das Rust-Modell `Recipe`, `CreateRecipe` und `UpdateRecipe` in `src/models/recipe.rs` erhalten das optionale Feld `planned_date: Option<NaiveDate>` (oder als String gespeichert und geparst).

**Parsing-Logik (serverseitig in Rust):**
- Eingabe kommt als Text vom Formular
- Akzeptierte Formate: `T.M.JJJJ`, `TT.MM.JJJJ`, `T.M.JJ`, `TT.MM.JJ`
- Parsing mit der `time`- oder `chrono`-Crate
- Bei zweistelligem Jahr: aktuelles Jahrhundert (20xx)
- Ergebnis: `NaiveDate` oder Fehler → Validierungsfehler im Formular

### UI/UX-Spezifikation

**Formular-Layout:**

```
Datum (geplant / gekocht)
[ 05.03.2025          ] [Kalender-Icon]
```

- Texteingabefeld (type="text", nicht type="date", damit das Format kontrolliert werden kann)
- Direkt rechts daneben ein Kalender-Icon (SVG, klickbar)
- Das Icon triggert ein `<input type="date">` (versteckt), dessen `change`-Event das Texteingabefeld befüllt
- Umsetzung ohne JS: Das versteckte `<input type="date">` ist sichtbar als Fallback

**HTMX / Progressive Enhancement:**
- Ohne JS: Das Formular enthält ein `<input type="date">` neben dem Textfeld (oder statt des Icons)
- Mit JS/HTMX: Das Kalender-Icon öffnet den nativen Date-Picker per kleinem JavaScript-Snippet

**Detailansicht:**

```
Datum: 5. März 2025
```

Nur anzeigen wenn Datum vorhanden. Kurze, lesbare Darstellung.

---

## 5. Nicht-funktionale Anforderungen

### Performance
- Seite lädt ohne sichtbare Verzögerung (< 500ms)
- Speichern des Datums erfolgt in < 500ms

### Browser-Support
- Aktuelle Chrome, Firefox, Safari, Edge Versionen
- iOS Safari und Android Chrome (letzte 2 Versionen)
- Nativer Date-Picker variiert je Browser - beide Eingabewege (Text + Picker) müssen funktionieren

### Barrierefreiheit
- WCAG 2.1 Level A konform
- Korrekte Labels für alle Formularfelder
- Fokus-Indikatoren sichtbar
- Fehlermeldungen sind programmatisch mit dem Feld verknüpft (`aria-describedby`)

---

## 6. Teststrategie

### E2E-Tests (Playwright)

**Testfall 1: Datum beim Erstellen eines Rezepts angeben**
```gherkin
Given die Seite "Neues Rezept" ist geöffnet
When der Nutzer Titel und Kategorie ausfüllt
And das Datum "5.3.2025" eingibt
And das Formular abspeichert
Then wird das Rezept mit dem Datum 05.03.2025 in der Detailansicht angezeigt
```

**Testfall 2: Datum beim Bearbeiten ändern**
```gherkin
Given ein Rezept ohne Datum existiert
When der Nutzer das Rezept bearbeitet
And das Datum "15.4.2026" eingibt
And speichert
Then zeigt die Detailansicht das Datum 15.04.2026
```

**Testfall 3: Datum löschen**
```gherkin
Given ein Rezept mit Datum "01.01.2025" existiert
When der Nutzer das Rezept bearbeitet
And das Datumsfeld leert
And speichert
Then wird in der Detailansicht kein Datum angezeigt
```

**Testfall 4: Ungültiges Datumsformat**
```gherkin
Given die Seite "Neues Rezept" ist geöffnet
When der Nutzer "morgen" als Datum eingibt
And das Formular absendet
Then wird eine Fehlermeldung angezeigt ("Kein gültiges Datum")
And das Formular wird nicht gespeichert
And die anderen Felder behalten ihre Werte
```

**Testfall 5: Datum über Date-Picker auswählen**
```gherkin
Given die Seite "Neues Rezept" ist geöffnet
When der Nutzer auf das Kalender-Icon klickt
And ein Datum im Picker auswählt
Then wird das gewählte Datum im Textfeld angezeigt
And wird beim Speichern korrekt übernommen
```

---

## 7. Abhängigkeiten & Rahmenbedingungen

### Abhängigkeiten
- Story 01 (Rezept erstellen) muss implementiert sein - ist abgeschlossen
- Story 02 (Rezept bearbeiten) muss implementiert sein - ist abgeschlossen
- Story 04 (Rezept-Detailansicht) muss implementiert sein - ist abgeschlossen

### Rahmenbedingungen
- SQLite-Datenbank muss existieren und erreichbar sein
- Keine Authentifizierung erforderlich (LAN-only)
- Datumsspeicherung im ISO-Format `YYYY-MM-DD` in SQLite (`DATE`-Spalte)
- Darstellung für den Nutzer in deutschem Format (`T.M.JJJJ` bzw. `TT. Monat JJJJ`)
- Das bestehende Datenbankschema hat noch kein `planned_date`-Feld - eine Migration ist notwendig

---

## Offene Punkte / Fragen

- [ ] Wie wird das Datum in der Listenansicht dargestellt? Kompakt (`05.03.2025`) oder als relative Angabe ("vor 3 Monaten")? → Kompaktes Format bevorzugt, Details in der Detailansicht
- [ ] Was passiert mit Rezepten ohne Datum im "Länger nicht gemacht"-Filter? → Wird in Story 09 spezifiziert

---

**Letzte Aktualisierung:** 2026-03-28
