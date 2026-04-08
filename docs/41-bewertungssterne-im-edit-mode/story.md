# Story 41: Bewertungssterne im Bearbeitungsmodus

**Epic:** Bewertung & Datums-Tracking
**Priorität:** MVP Phase 1 / Bug-Fix
**Status:** Offen

---

## 1. Story-Satz

Als **Benutzer** möchte ich, dass im Bearbeiten-Modus eines Rezeptes der Stern-Bewertung alle Sterne bis zum ausgewählten Stern korrekt markiert werden, damit ich visuell klar erkenne, welche Bewertung ausgewählt ist.

---

## 2. Geschäftsbezogene Details

### Kontext
Beim Bearbeiten von Rezepten gibt es einen Bug in der Darstellung der Stern-Bewertung. Im Bearbeitungsmodus ist nur der ausgewählte Stern markiert (z.B. nur der 4.), nicht aber alle Sterne bis zum ausgewählten Stern (sollte 1., 2., 3., 4. zeigen). In der Übersichtsliste funktioniert die Darstellung korrekt.

### Nutzergruppe
Alle Benutzer, die Rezepte bearbeiten und Bewertungen setzen oder ändern.

### Business-Value
Vermeidung von Verwirrung bei der Bewertungsauswahl und konsistente UX zwischen Bearbeitungsmodus und Übersichtsansicht.

### Edge Cases
- **Leere Bewertung:** Wenn keine Bewertung ausgewählt ist, sollten alle Sterne unmarkiert sein.
- **Bewertung auf 1 setzen:** Nur der erste Stern sollte markiert sein.
- **Bewertung auf 4 gesetzt:** Die ersten 4 Sterne sollten markiert sein.
- **Vollständige Bewertung (5 Sterne):** Alle Sterne sollten markiert sein.
- **Wechsel zwischen Edit- und View-Modus:** Die Darstellung muss in beiden Modi korrekt funktionieren.

---

## 3. Akzeptanzkriterien

### Funktionale Kriterien

- [ ] **K1: Stern-Darstellung im Bearbeitungsmodus**
  - Beim Überfahren mit der Maus oder Klicken auf einen Stern werden alle Sterne bis zu diesem Stern markiert (z.B. bei Auswahl von Stern 4 werden Sterne 1, 2, 3, 4 angezeigt)
  - Die Darstellung entspricht der Darstellungslogik in der Übersichtsliste

- [ ] **K2: Konsistenz zwischen Edit- und View-Modus**
  - Die Stern-Bewertung wird im Bearbeitungsmodus visuell identisch dargestellt wie in der Detailansicht/Übersicht
  - Nach dem Speichern bleibt die korrekte Darstellung erhalten

### Nicht-funktionale Kriterien

- [ ] **K3: Barrierefreiheit**
  - Tastatur-Navigation funktioniert (Pfeiltasten zum Wechseln zwischen Sternen)
  - Screenreader geben die aktuelle Bewertung korrekt aus

---

## 4. Technische Planung

### Datenmodell
Keine Änderungen am Datenmodell notwendig, da es sich um ein reines Frontend-Problem handelt.

### UI/UX-Spezifikation
Die Stern-Bewertungskomponente muss im Bearbeitungsmodus dieselbe Logik verwenden wie in der Detailansicht:
- Beim Hover/Click auf Stern N müssen die Sterne 1 bis N aktiviert werden
- CSS-Klassen müssen korrekt gesetzt werden, um die visuelle Darstellung zu steuern

---

## 5. Nicht-funktionale Anforderungen

### Performance
- Keine spürbare Verzögerung beim Wechseln der Bewertung

### Browser-Support
- Aktuelle Chrome, Firefox, Safari, Edge Versionen
- iOS Safari und Android Chrome (letzte 2 Versionen)

### Barrierefreiheit
- WCAG 2.1 Level A konform
- Fokus-Indikatoren sichtbar

---

## 6. Teststrategie

### E2E-Tests (Playwright)

**Testfall 1: Stern-Bewertung im Bearbeitungsmodus**
```gherkin
Given ich bin auf der Bearbeitungsseite eines Rezepts mit bestehender Bewertung
When ich über einen Stern (z.B. den vierten) fahre oder darauf klicke
Then werden alle Sterne bis zu diesem Stern markiert (Sterne 1-4)
Und die Darstellung entspricht der in der Detailansicht
```

**Testfall 2: Konsistenz zwischen Edit- und View-Modus**
```gherkin
Given ich habe ein Rezept mit einer bestimmten Bewertung erstellt
When ich das Rezept bearbeite und über Sterne fahre
Then zeigt der Bearbeitungsmodus dieselbe Stern-Darstellung wie die Detailansicht
And when I save the changes
The correct rating is displayed in both edit and view modes
```

---

## 7. Abhängigkeiten & Rahmenbedingungen

### Abhängigkeiten
- Keine Abhängigkeiten von anderen Stories

### Rahmenbedingungen
- SQLite-Datenbank muss existieren und erreichbar sein
- Keine Authentifizierung erforderlich (LAN-only)

---

## Offene Punkte / Fragen

- [ ] Lokalisierung des Bugs: Ist es ein CSS-Problem oder eine JavaScript/HTMX-Logik?
- [ ] Welche Komponenten sind betroffen: Nur die Stern-Bewertung oder auch andere Eingabefelder?

---

**Letzte Aktualisierung:** 2026-04-08
