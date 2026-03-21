# Definition of Ready

Eine Story gilt als "ready" (bereit für Umsetzung), wenn folgende Kriterien erfüllt sind:

## Pflicht-Bestandteile

### 1. Story-Satz
**Format:** "Als [Rolle] möchte ich [Ziel/Wunsch], damit [Nutzen]."

Beispiel:  
*"Als Benutzer möchte ich nach Zutaten suchen können, damit ich schnell passende Rezepte finde."*

### 2. Geschäftsbezogene Details
- **Kontext:** Warum ist diese Funktion wichtig?
- **Nutzergruppe:** Wer profitiert davon?
- **Business-Value:** Was ist der konkrete Mehrwert?
- **Edge Cases:** Was passiert bei Fehleingaben/Fehlern?

### 3. Akzeptanzkriterien
Klar formulierte, testbare Kriterien in Gherkin-Syntax oder als checkliste:

```
- [ ] Kriterium 1: Beschreibung + Erwartetes Ergebnis
- [ ] Kriterium 2: Beschreibung + Erwartetes Ergebnis
...
```

## Weitere Pflicht-Aspekte

### 4. Technische Planung
- Datenmodell definiert (falls relevant)
- UI/UX-Spezifikationen (Wireframes, Mockups oder Beschreibung)

### 5. Nicht-funktionale Anforderungen
- Performance-Ziele (z.B. Ladezeit < 1s)
- Browser-/Geräte-Support definiert
- Barrierefreiheit (WCAG 2.1 AA Standard)
- Security-Aspekte berücksichtigt

### 6. Teststrategie
- E2E-Tests: welche Userflows müssen funktionieren?

### 7. Abhängigkeiten & Rahmenbedingungen
- Abhängigkeiten von anderen Stories
- Externe Abhängigkeiten (APIs, Drittanbieter)

