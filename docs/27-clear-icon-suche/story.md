# Story 27: Clear-Icon in Volltextsuche triggert neue Suche

**Epic:** Epic 3: Suche & Filterung
**Priorität:** MVP - Phase 1
**Status:** In Arbeit

---

## 1. Story-Satz

Als **Benutzer** möchte ich **durch Klick auf das Clear-Icon im Suchfeld sofort die vollständige Rezeptliste sehen**, damit ich **nach einer Suche schnell und ohne zusätzliche Schritte zur Gesamtübersicht zurückgelangen kann**.

---

## 2. Geschäftsbezogene Details

### Kontext

Die Volltextsuche (Story 07) zeigt ein Clear-Icon (kleines X) an, sobald ein Suchbegriff eingegeben wurde. Dieses Icon löscht aktuell nur den Text im Suchfeld, löst aber keine neue Suche aus. Der Benutzer muss nach dem Leeren des Feldes noch manuell eine Aktion ausführen (z.B. Enter drücken oder auf einen Button klicken), um die vollständige Rezeptliste wieder zu sehen.

Das ist eine Usability-Lücke: Der intuitive Erwartungswert beim Klick auf das X ist, dass man sofort wieder alle Rezepte sieht — nicht, dass das Feld leer ist, aber die gefilterte (oder leere) Liste bestehen bleibt.

### Nutzergruppe

- Beide Partner des Haushalts gleichberechtigt
- Zugriff über Desktop, Tablet und Handy im LAN

### Business-Value

- Verbessert die Bedienbarkeit der Suche deutlich: ein Klick reicht, um zur vollständigen Liste zurückzukehren
- Entspricht dem Standard-Verhalten von Suchfeldern in verbreiteten Apps (Google, iOS Safari etc.)
- Reduziert kognitive Reibung bei der Wochenplanung

### Edge Cases

- **Suchfeld ist bereits leer:** Das Clear-Icon ist nicht sichtbar, d.h. dieser Fall tritt nicht auf. Falls das Icon dennoch sichtbar ist und geklickt wird, bleibt die vollständige Rezeptliste unverändert sichtbar.
- **Suche ohne Treffer:** Klick auf X zeigt sofort alle Rezepte, auch wenn vorher "Keine Rezepte gefunden" angezeigt wurde.
- **Kein JavaScript:** Ohne JS funktioniert das Clear-Icon über einen Form-Submit mit leerem `q`-Parameter, was dieselbe Seite mit vollständiger Liste lädt.

---

## 3. Akzeptanzkriterien

### Funktionale Kriterien

- [ ] **K1: Clear-Icon löst Suche aus**
  - Nach Klick auf das Clear-Icon wird das Suchfeld geleert
  - Gleichzeitig (ohne weiteren Schritt) wird eine neue Suche mit leerem Suchbegriff ausgelöst
  - Die vollständige, ungefilterte Rezeptliste wird sofort angezeigt

- [ ] **K2: Clear-Icon nur bei vorhandenem Suchbegriff sichtbar**
  - Das Clear-Icon ist nur sichtbar, wenn das Suchfeld einen nicht-leeren Suchbegriff enthält
  - Bei leerem Suchfeld ist das Clear-Icon nicht sichtbar

- [ ] **K3: URL wird aktualisiert**
  - Nach dem Klick auf das Clear-Icon enthält die URL keinen `q`-Parameter mehr (oder `q=`), sodass der Zustand Deep-Link-fähig bleibt
  - Beim Neuladen der Seite ohne `q`-Parameter wird die vollständige Rezeptliste angezeigt

- [ ] **K4: Fokus nach Löschen**
  - Nach dem Klick auf das Clear-Icon verbleibt der Fokus sinnvoll (entweder im Suchfeld oder neutral), damit der Benutzer direkt weitertipppen kann

### Nicht-funktionale Kriterien

- [ ] **K5: Performance**
  - Die vollständige Rezeptliste erscheint unmittelbar nach Klick ohne wahrnehmbare Verzögerung (< 500ms)

- [ ] **K6: Barrierefreiheit**
  - Das Clear-Icon hat ein aussagekräftiges `aria-label` (z.B. "Suche zurücksetzen")
  - Es ist per Tastatur erreichbar (Tab + Enter/Space)
  - Die Aktion ist für Screenreader verständlich angekündigt

---

## 4. Technische Planung

### Datenmodell

Keine Datenbankänderungen erforderlich. Die bestehende Suche über `q`-Parameter wird weiterverwendet.

### UI/UX-Spezifikation

**Aktueller Zustand:**
- Suchfeld mit Clear-Icon (X) — klickt man darauf, wird nur das Textfeld geleert
- Die Rezeptliste aktualisiert sich nicht automatisch

**Ziel-Verhalten:**
- Klick auf Clear-Icon: Text wird geleert UND eine neue HTMX-Anfrage mit leerem `q`-Parameter wird ausgelöst
- Die Rezeptliste wird durch das HTMX-Target ersetzt (vollständige Liste)
- Alternativ (ohne JS): Das Clear-Icon ist ein Submit-Button innerhalb des Suchformulars, das den `q`-Parameter leer übermittelt

**Technischer Ansatz:**
- Das Clear-Icon erhält HTMX-Attribute (`hx-get`, `hx-target`, `hx-vals`) um eine leere Suche auszulösen
- Oder: Das Icon ist ein `<button type="submit">` im Suchformular, der das Feld leert und das Formular abschickt (JS-Event-Handler + Fallback)

---

## 5. Nicht-funktionale Anforderungen

### Performance
- Vollständige Liste erscheint in < 500ms nach Klick auf Clear-Icon
- Keine merkliche Verzögerung gegenüber dem normalen Such-Flow

### Browser-Support
- Aktuelle Chrome, Firefox, Safari, Edge Versionen
- iOS Safari und Android Chrome (letzte 2 Versionen)

### Barrierefreiheit
- WCAG 2.1 Level A konform
- Clear-Icon mit `aria-label="Suche zurücksetzen"` versehen
- Per Tastatur (Tab + Enter/Space) bedienbar
- Fokus-Indikatoren sichtbar

---

## 6. Teststrategie

### E2E-Tests (Playwright)

**Testfall 1: Klick auf Clear-Icon zeigt vollständige Liste**
```gherkin
Given Die App enthält mehrere Rezepte
And Der Benutzer hat "Bolognese" in das Suchfeld eingegeben
And Die gefilterte Rezeptliste wird angezeigt
When Der Benutzer klickt auf das Clear-Icon im Suchfeld
Then Das Suchfeld ist leer
And Die vollständige Rezeptliste wird angezeigt (alle Rezepte sichtbar)
And Der URL-Parameter `q` ist nicht mehr gesetzt
```

**Testfall 2: Clear-Icon nur bei gefülltem Suchfeld sichtbar**
```gherkin
Given Das Suchfeld ist leer
Then Das Clear-Icon ist nicht sichtbar
When Der Benutzer tippt "Salat" in das Suchfeld
Then Das Clear-Icon wird sichtbar
```

**Testfall 3: Clear-Icon nach Suche ohne Treffer**
```gherkin
Given Der Benutzer hat "xyzxyzxyz" gesucht
And Die Meldung "Keine Rezepte gefunden" wird angezeigt
When Der Benutzer klickt auf das Clear-Icon
Then Das Suchfeld ist leer
And Die vollständige Rezeptliste wird angezeigt
```

**Testfall 4: Tastatur-Navigation**
```gherkin
Given Der Benutzer hat einen Suchbegriff eingegeben
When Der Benutzer navigiert per Tab zum Clear-Icon und drückt Enter
Then Das Suchfeld wird geleert und die vollständige Rezeptliste angezeigt
```

---

## 7. Abhängigkeiten & Rahmenbedingungen

### Abhängigkeiten
- Story 07 (Volltextsuche über Titel, Zutaten und Anleitung) muss implementiert und abgeschlossen sein — dieses Feature erweitert das bestehende Suchfeld
- Story 26 (Nutzung von Icons) — das Clear-Icon selbst wurde im Rahmen der Icon-Integration eingeführt

### Rahmenbedingungen
- SQLite-Datenbank muss existieren und erreichbar sein
- Keine Authentifizierung erforderlich (LAN-only)
- DeepLink-fähige URLs mit Query-Parametern (Architektur-Vorgabe)

---

## Offene Punkte / Fragen

- [ ] Soll nach dem Klick auf das Clear-Icon der Fokus im Suchfeld verbleiben, damit der Benutzer direkt einen neuen Begriff eingeben kann?

---

**Letzte Aktualisierung:** 2026-03-28
