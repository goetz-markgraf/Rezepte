# Story 42: Suche "Länger nicht gemacht" in Top-Bar verschieben

**Epic:** Epic 5: Wochenplanung
**Priorität:** Medium
**Status:** In Arbeit

---

## 1. Story-Satz

Als **Nutzer (Haushalt)** möchte ich **die Suche "Länger nicht gemacht & Mittagessen" direkt in der Top-Bar finden**, damit ich **unabhängig von meiner aktuellen Position in der App schnell eine Inspiration für das Mittagessen der nächsten Woche erhalten kann**.

---

## 2. Geschäftsbezogene Details

### Kontext
Aktuell gibt es in der Wochenvorschau einen Button, der eine gefilterte Suche auslöst ("Länger nicht gemacht" kombiniert mit der Kategorie "Mittagessen"). Da dies ein zentraler Workflow bei der wöchentlichen Planung (Mittwoch/Donnerstag) ist, soll dieser Zugriff global über die Top-Bar ermöglicht werden.

### Nutzergruppe
Die beiden Partner im Haushalt.

### Business-Value
Steigerung der Effizienz bei der Wochenplanung. Die Hürde, zur Wochenvorschau zu navigieren, um dann die Suche zu starten, wird eliminiert.

### Edge Cases
- **Keine Rezepte vorhanden:** Die Suche führt zu einer leeren Liste mit entsprechendem Hinweis.
- **Kategorie "Mittagessen" existiert nicht/ist leer:** Die Liste zeigt keine Ergebnisse.

---

## 3. Akzeptanzkriterien

### Funktionale Kriterien

- [ ] **K1: Link in der Top-Bar**
  - In der Kopfzeile (Top-Bar) der Anwendung ist ein deutlich sichtbarer Link oder Button mit der Bezeichnung "Länger nicht gemacht (Mittagessen)" (oder ähnlich prägnant) integriert.
- [ ] **K2: Auslösung der Suche**
  - Ein Klick auf diesen Link führt direkt zur Rezept-Übersicht.
  - Die Übersicht ist automatisch gefiltert nach der Kategorie "Mittagessen".
  - Die Sortierung ist auf "Länger nicht gemacht" (Datum aufsteigend) gesetzt.
- [ ] **K3: Entfernung aus Wochenvorschau**
  - Der bisherige Button für diese spezifische Suche in der Wochenvorschau wird entfernt, da er durch die Top-Bar redundant wird.

### Nicht-funktionale Kriterien

- [ ] **K4: Barrierefreiheit**
  - Der Link in der Top-Bar ist tastaturnavigierbar und besitzt einen korrekten Fokus-Indikator.
  - Der Kontrast des Links entspricht WCAG 2.1 Level A.
- [ ] **K5: Responsive Design**
  - Der Link ist auch in der mobilen Ansicht der Top-Bar (z.B. im Hamburger-Menü oder als kompaktes Icon/Text) gut erreichbar und bedienbar.

---

## 4. Technische Planung

*(Wird im Rahmen von Phase 2: Implementierungsplan detailliert)*

---

## 5. Nicht-funktionale Anforderungen

### Performance
- Der Wechsel von der aktuellen Seite zur gefilterten Liste erfolgt ohne spürbare Verzögerung (< 500ms).

### Browser-Support
- Aktuelle Chrome, Firefox, Safari, Edge Versionen
- iOS Safari und Android Chrome (letzte 2 Versionen)

### Barrierefreiheit
- WCAG 2.1 Level A konform

---

## 6. Teststrategie

### E2E-Tests (Playwright)

**Testfall 1: Navigation über Top-Bar**
```gherkin
Given der Nutzer befindet sich auf einer beliebigen Seite (z.B. Detailansicht)
When der Nutzer in der Top-Bar auf "Länger nicht gemacht (Mittagessen)" klickt
Then wird die Rezept-Liste angezeigt
And die Liste ist nach der Kategorie "Mittagessen" gefiltert
And die Rezepte sind nach dem Datum "Zuletzt gemacht" aufsteigend sortiert
```

**Testfall 2: Entfernung aus Wochenvorschau**
```gherkin
Given der Nutzer befindet sich in der Wochenvorschau
Then ist der Button "Länger nicht gemacht (Mittagessen)" dort nicht mehr vorhanden
```

---

## 7. Abhängigkeiten & Rahmenbedingungen

### Abhängigkeiten
- Die Funktionen für Kategorien-Filter und die Sortierung "Länger nicht gemacht" müssen bereits implementiert sein (Story 08 & 09).

### Rahmenbedingungen
- LAN-only, keine Authentifizierung.

---

**Letzte Aktualisierung:** 2026-04-09
