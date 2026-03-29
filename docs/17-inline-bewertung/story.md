# Story 17: Inline-Bewertung ohne Edit-Mode

**Epic:** Bewertung & Datums-Tracking
**Priorität:** Post-MVP - Phase 2
**Status:** In Arbeit

---

## 1. Story-Satz

**Als** Benutzer möchte ich **die Sterne-Bewertung eines Rezepts direkt in der Detailansicht ändern**, damit ich **schnell und ohne Umweg über den Edit-Mode bewerten kann - zum Beispiel spontan beim Essen**.

---

## 2. Geschäftsbezogene Details

### Kontext

Wenn Anna oder Dragon am Esstisch sitzen und ein Gericht gut fanden, wollen sie es sofort bewerten - ohne in den Bearbeitungsmodus wechseln zu müssen. Heute erfordert das: Detailseite öffnen → "Bearbeiten" klicken → Formular ausfüllen → Speichern. Das sind zu viele Schritte für einen impulsiven Moment beim Essen.

Die PRD-Journey 3 ("Beim Abendessen - Spontane Bewertung") beschreibt explizit diesen Anwendungsfall: "Direktes Bewerten ohne Edit-Mode: Sie tippt 5 Sterne. Fertig. Keine Formulare, kein Speichern-Button, einfach nur: Tap, bewertet."

Das Bewertungsfeld ist ein idealer Kandidat für Inline-Editing, weil:
- Es nur einen einzigen Wert hat (1-5 Sterne oder keine Bewertung)
- Kein Freitext, kein Risiko von unvollständigen Eingaben
- Der Nutzen für spontane Bewertungen deutlich höher ist als der Entwicklungsaufwand

### Nutzergruppe

- Beide Partner des Haushalts (Anna & Dragon)
- Typischerweise unmittelbar nach oder während des Essens, meistens vom Handy

### Business-Value

- Deutlich geringere Barriere für das Bewerten → mehr Bewertungen im System
- Mehr Bewertungen → bessere Filterergebnisse (Story 11 - Filter nach Bewertung)
- Unterstützt die Vision: "Bewertungen werden zur natürlichen Gewohnheit"
- Ergänzt Story 14 (Bewertung im Edit-Mode) ohne es zu ersetzen - komplexe Änderungen bleiben im Formular

### Edge Cases

- **Bewertung auf gleichen Wert tippen:** Wenn der Benutzer dieselbe Stern-Anzahl nochmal antippt, soll die Bewertung zurückgesetzt werden (auf "keine Bewertung"). Das ermöglicht das Entfernen einer Bewertung ohne Formular.
- **Noch keine Bewertung:** Der Sterne-Bereich zeigt leere Sterne oder einen Hinweis, dass noch keine Bewertung gesetzt ist. Ein Antippen setzt sofort die Bewertung.
- **Netzwerkfehler beim Speichern:** Falls die HTMX-Anfrage fehlschlägt, soll die alte Bewertung sichtbar bleiben und kein inkonsistenter Zustand entstehen.
- **Gleichzeitige Bearbeitung (Last-write-wins):** Wenn zwei Personen gleichzeitig bewerten, gewinnt die spätere Anfrage - das ist akzeptiertes Verhalten im Single-User-Modell des Haushalts.
- **Ungültige Werte:** Server-seitige Validierung lehnt Werte außerhalb 1-5 ab.

---

## 3. Akzeptanzkriterien

### Funktionale Kriterien

- [ ] **K1: Sterne sind in der Detailansicht direkt antippbar**
  - In der Rezept-Detailansicht ist der Sterne-Bereich interaktiv (kein separater "Bearbeiten"-Knopf nötig)
  - Das Antippen eines Sterns setzt die Bewertung sofort (ohne zusätzlichen Speichern-Button)
  - Die Änderung wird per HTMX asynchron an den Server gesendet

- [ ] **K2: Visuelle Rückmeldung nach dem Speichern**
  - Nach dem Antippen aktualisiert sich der Sterne-Bereich sofort mit der neuen Bewertung
  - Der Benutzer sieht ohne Seitenneuladung, dass die Bewertung gespeichert wurde

- [ ] **K3: Bewertung zurücksetzen durch erneutes Antippen**
  - Wenn der Benutzer denselben Stern antippt, der bereits die aktuelle Bewertung markiert, wird die Bewertung auf "keine Bewertung" zurückgesetzt
  - Nach dem Reset zeigt der Bereich "keine Bewertung" an

- [ ] **K4: Bewertung wird persistent gespeichert**
  - Die über die Inline-Bewertung gesetzte Bewertung ist nach Seitenneuladung weiterhin vorhanden
  - Sie entspricht exakt dem Wert, der auch über das Edit-Formular gespeichert würde

- [ ] **K5: Funktion ohne JavaScript (Progressive Enhancement)**
  - Ohne JavaScript soll der Sterne-Bereich weiterhin lesbar und die Bewertung sichtbar sein
  - Eine Möglichkeit zur Bewertungsänderung ohne JS ist nicht zwingend erforderlich (der Edit-Mode ist der Fallback)

- [ ] **K6: Keine Regression im Edit-Mode**
  - Die Bewertung ist weiterhin auch über das normale Bearbeitungsformular (Story 14) änderbar
  - Inline-Bewertung und Formular-Bewertung zeigen nach dem Speichern denselben Wert

### Nicht-funktionale Kriterien

- [ ] **K7: Performance**
  - Antippen und Speichern der Bewertung dauert < 500ms (vom Tap bis zur visuellen Bestätigung)
  - Keine Seitenneuladung notwendig

- [ ] **K8: Barrierefreiheit**
  - Die Sterne-Auswahl ist per Tastatur bedienbar (Tab-Navigation zu den einzelnen Sternen, Leertaste/Enter zum Auswählen)
  - Screen-Reader erhalten sinnvolle Labels ("1 Stern", "2 Sterne", ... "5 Sterne", "Bewertung entfernen")
  - WCAG 2.1 Level A konform
  - Touch-Zielfläche mindestens 44x44px pro Stern (mobile-freundlich)

---

## 4. Technische Planung

### Datenmodell

Kein neues Datenbankfeld erforderlich. Das `rating`-Feld (INTEGER NULL, 1-5) aus Story 14 wird weiterverwendet.

Es wird ein neuer, separater HTTP-Endpunkt benötigt, der nur das `rating`-Feld eines Rezepts aktualisiert:

```
POST /recipes/:id/rating
Body: rating=4  (oder rating= für "keine Bewertung")
Response: HTML-Fragment mit dem aktualisierten Sterne-Bereich (HTMX-Swap)
```

### UI/UX-Spezifikation

**Detailansicht - Sterne-Bereich:**
- Die Sterne werden als klickbare Elemente dargestellt (z.B. als `<button>`-Elemente oder als Radio-Buttons mit CSS-Styling)
- HTMX-Attribut `hx-post="/recipes/:id/rating"` und `hx-target` auf den Sterne-Container
- Bei Hover/Focus: optische Hervorhebung des aktiven Sterns und aller Sterne darunter (Standard-Sterne-Interaktion)
- Der aktive Stern der aktuellen Bewertung ist visuell hervorgehoben

**Interaktionsablauf:**
1. Benutzer tippt auf Stern 4
2. HTMX sendet `POST /recipes/:id/rating` mit `rating=4`
3. Server speichert und antwortet mit aktualisiertem HTML-Fragment für den Sterne-Bereich
4. HTMX ersetzt den Sterne-Container im DOM - die neuen 4 Sterne sind sofort sichtbar

**Reset-Mechanismus:**
- Ist der aktuelle Stern bereits der ausgewählte, sendet der Klick `rating=` (leer) → kein Rating

---

## 5. Nicht-funktionale Anforderungen

### Performance
- Inline-Speicherung < 500ms (vom Klick bis zum aktualisierten UI)
- Keine sichtbare Verzögerung beim Hover-Effekt auf die Sterne

### Browser-Support
- Aktuelle Chrome, Firefox, Safari, Edge Versionen
- iOS Safari und Android Chrome (letzte 2 Versionen)
- Touch-freundliche Sterne (Mindest-Tippfläche 44x44px)

### Barrierefreiheit
- WCAG 2.1 Level A konform
- Tastatur-Navigation zu jedem Stern
- Sinnvolle ARIA-Labels für Screenreader

---

## 6. Teststrategie

### E2E-Tests (Playwright)

**Testfall 1: Inline-Bewertung setzen**
```gherkin
Given Ein Rezept ohne Bewertung existiert in der Datenbank
When Benutzer öffnet die Detailseite des Rezepts
And Benutzer klickt auf den 4. Stern
Then Die Bewertung wird sofort als 4 Sterne angezeigt (ohne Seitenneuladung)
And Nach einer Seitenneuladung ist die Bewertung immer noch 4 Sterne
```

**Testfall 2: Bewertung ändern**
```gherkin
Given Ein Rezept mit 3-Sterne-Bewertung existiert in der Datenbank
When Benutzer öffnet die Detailseite des Rezepts
And Benutzer klickt auf den 5. Stern
Then Die Bewertung wird sofort als 5 Sterne angezeigt
And Nach einer Seitenneuladung ist die Bewertung 5 Sterne
```

**Testfall 3: Bewertung zurücksetzen durch erneutes Antippen**
```gherkin
Given Ein Rezept mit 4-Sterne-Bewertung existiert in der Datenbank
When Benutzer öffnet die Detailseite des Rezepts
And Benutzer klickt auf den 4. Stern (den aktuell aktiven)
Then Die Bewertung wird entfernt (keine Sterne angezeigt)
And Nach einer Seitenneuladung hat das Rezept keine Bewertung
```

**Testfall 4: Inline-Bewertung und Edit-Mode zeigen gleichen Wert**
```gherkin
Given Ein Rezept ohne Bewertung existiert in der Datenbank
When Benutzer setzt per Inline-Bewertung 5 Sterne
And Benutzer öffnet den Edit-Mode des Rezepts
Then Im Edit-Formular sind 5 Sterne vorausgewählt
```

---

## 7. Abhängigkeiten & Rahmenbedingungen

### Abhängigkeiten

- Story 14 (Rezept mit 3-5 Sternen bewerten) muss abgeschlossen sein - das `rating`-Feld in DB und allen Schichten muss existieren
- Story 04 (Rezept-Detailansicht) muss abgeschlossen sein - die Inline-Bewertung wird in der Detailansicht integriert

### Rahmenbedingungen

- SQLite-Datenbank muss existieren und erreichbar sein
- Keine Authentifizierung erforderlich (LAN-only)
- HTMX ist bereits im Projekt eingebunden und aktiv genutzt
- Single-User-Modell (beide Partner = gleicher User, last-write-wins)

---

## Offene Punkte / Fragen

- Soll der Inline-Bereich visuell (z.B. durch ein kleines Stift-Icon) signalisieren, dass er klickbar ist - oder soll er wie normale Sterne aussehen und die Interaktivität implizit sein?

---

**Letzte Aktualisierung:** 2026-03-29
