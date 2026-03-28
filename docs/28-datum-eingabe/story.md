# Story 28: Datum-Eingabe am Rezept (geplant / gekocht)

**Epic:** Epic 4: Bewertung & Datums-Tracking
**Priorität:** [MVP Phase X / Nice-to-have]
**Status:** Offen

---

## 1. Story-Satz

Als **[Rolle]** möchte ich **[Ziel/Wunsch]**, damit ich **[Nutzen]**.

---

## 2. Geschäftsbezogene Details

### Kontext
[Warum ist diese Funktion wichtig? Was ist der Hintergrund?]

### Nutzergruppe
[Wer nutzt diese Funktion?]

### Business-Value
[Was ist der konkrete Mehrwert?]

### Edge Cases
- **[Fall 1]:** [Beschreibung und erwartetes Verhalten]
- **[Fall 2]:** [Beschreibung und erwartetes Verhalten]
...

---

## 3. Akzeptanzkriterien

### Funktionale Kriterien

- [ ] **K1: [Kriterium-Titel]**
  - [Detail-Bedingung]
  - [Detail-Bedingung]

- [ ] **K2: [Kriterium-Titel]**
  - [Detail-Bedingung]

### Nicht-funktionale Kriterien

- [ ] **K[N]: Performance**
  - [Ladezeit-Ziel]
  - [Speichervorgang-Ziel]

- [ ] **K[N+1]: Barrierefreiheit**
  - Alle Formularfelder haben korrekte Labels (WCAG 2.1 Level A)
  - Tastatur-Navigation funktioniert vollständig

---

## 4. Technische Planung

### Datenmodell
[Falls neue Felder/Tabellen notwendig: Beschreibung der Änderungen am Schema]

### UI/UX-Spezifikation
[Beschreibung des Layouts, der Interaktionen, des Flows]

---

## 5. Nicht-funktionale Anforderungen

### Performance
- Seite lädt ohne sichtbare Verzögerung (< 500ms)
- [Weitere spezifische Ziele]

### Browser-Support
- Aktuelle Chrome, Firefox, Safari, Edge Versionen
- iOS Safari und Android Chrome (letzte 2 Versionen)

### Barrierefreiheit
- WCAG 2.1 Level A konform
- Fokus-Indikatoren sichtbar

---

## 6. Teststrategie

### E2E-Tests (Playwright)

**Testfall 1: [Bezeichnung]**
```gherkin
Given [Ausgangszustand]
When [Aktion des Benutzers]
Then [Erwartetes Ergebnis]
```

**Testfall 2: [Bezeichnung]**
```gherkin
Given [Ausgangszustand]
When [Aktion des Benutzers]
Then [Erwartetes Ergebnis]
```

---

## 7. Abhängigkeiten & Rahmenbedingungen

### Abhängigkeiten
- [Story X muss implementiert sein / keine Abhängigkeiten]
- [Blockiert: Story Y]

### Rahmenbedingungen
- SQLite-Datenbank muss existieren und erreichbar sein
- Keine Authentifizierung erforderlich (LAN-only)

---

## Offene Punkte / Fragen

- [ ] [Offene Frage oder Entscheidung]

---

**Letzte Aktualisierung:** 2026-03-28

---

## Zusatzinformationen

An jedem Rezept soll es ein optionales Datum geben, wann geplant ist, dass das Essen gemacht wird oder gemacht wurde. Beim Neu-Erfassen und bei der Bearbeitung soll man dieses Datum angeben können. Es kann auch leer sein. Für die Eingabe soll sowohl ein String (d.m.y) möglich sein (mit und ohne führende Nullen und mit 2 oder 4stelligem Jahr). Zusätzlich soll auch ein Date Picker möglich sein, über ein Icon neben dem Eingabefeld.
