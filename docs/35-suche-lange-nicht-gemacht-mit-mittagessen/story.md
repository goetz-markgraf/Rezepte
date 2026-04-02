# Story 35: Suche "Länger nicht gemacht" mit vorselektiertem Mittagessen-Filter

**Epic:** Epic 5: Wochenplanung
**Priorität:** MVP Phase 2
**Status:** In Arbeit

---

## 1. Story-Satz

Als **Nutzerin** möchte ich **beim Aufrufen der Suche "Länger nicht gemacht" aus der Wochenübersicht den Kategorie-Filter "Mittagessen" automatisch vorselektiert haben**, damit ich **sofort die relevanten Rezepte für die Wochenplanung sehe, ohne den Filter manuell setzen zu müssen**.

---

## 2. Geschäftsbezogene Details

### Kontext

Story 34 hat den Einstiegspunkt in der Wochenübersicht geschaffen: Ein Klick öffnet die "Länger nicht gemacht"-Suche. Die Wochenübersicht dient primär der Mittagessen-Planung — Lücken im Wochenplan sind fast ausschließlich Mittagessen-Slots.

Ohne dieses Feature müsste die Nutzerin nach dem Klick noch manuell den Kategorie-Filter "Mittagessen" setzen. Das ist ein überflüssiger Schritt, der den Planungsfluss unterbricht. Story 35 schließt diese Lücke: Der Klick aus der Wochenübersicht öffnet die Suche direkt mit vorselektiertem "Mittagessen"-Filter.

### Nutzergruppe

- Beide Partner des Haushalts gleichberechtigt
- Zugriff über Desktop, Tablet und Handy im LAN
- Primär bei der Wochenplanung (typisch Mittwoch/Donnerstag)

### Business-Value

- Reduziert die Wochenplanung auf den minimalen Aufwand: ein Klick öffnet genau die richtige Ansicht
- Verhindert, dass die Nutzerin manuell Filter setzen muss — der Kontext (Mittagessen-Planung) ist aus der Wochenübersicht bereits bekannt
- Stärkt das Kernziel: Wochenplanung von 20+ Minuten auf 2 Minuten reduzieren
- Die Kombination "Länger nicht gemacht" + "Mittagessen" ist der häufigste Anwendungsfall im Planungskontext

### Edge Cases

- **Keine passenden Mittagessen-Rezepte:** Wenn keine Mittagessen-Rezepte die Bedingung "länger nicht gemacht" erfüllen, erscheint eine freundliche Meldung statt einer leeren Seite
- **Alle Mittagessen-Rezepte haben Zukunftsdaten:** Diese werden korrekt ausgeschlossen; Meldung erscheint, wenn keine Treffer verbleiben
- **Nutzerin möchte andere Kategorie sehen:** Der "Mittagessen"-Filter kann manuell abgewählt oder durch andere Kategorien ergänzt werden — die normale Filter-Interaktion bleibt vollständig erhalten
- **Direkter URL-Aufruf:** Die URL mit beiden Parametern funktioniert als DeepLink und zeigt dieselbe vorselektierte Ansicht

---

## 3. Akzeptanzkriterien

### Funktionale Kriterien

- [ ] **K1: Klick aus Wochenübersicht öffnet Suche mit beiden Filtern**
  - Der Klick-Einstiegspunkt aus Story 34 navigiert zur Rezept-Übersichtsseite
  - Der "Länger nicht gemacht"-Filter ist automatisch aktiv
  - Der Kategorie-Filter "Mittagessen" ist automatisch vorselektiert
  - Beide Filter sind visuell als aktiv erkennbar

- [ ] **K2: URL enthält beide Filter-Parameter**
  - Die resultierende URL enthält sowohl den "Länger nicht gemacht"-Parameter als auch `kategorie=mittagessen`
  - Die URL kann als DeepLink direkt aufgerufen werden und zeigt dieselbe Ansicht

- [ ] **K3: Vorselektierter Filter bleibt interaktiv**
  - Die Nutzerin kann den "Mittagessen"-Filter manuell abwählen
  - Die Nutzerin kann weitere Kategorien hinzufügen oder den "Länger nicht gemacht"-Filter deaktivieren
  - Das normale Filter-Verhalten aus Stories 8 und 9 bleibt vollständig erhalten

- [ ] **K4: Keine Treffer werden klar kommuniziert**
  - Wenn keine Mittagessen-Rezepte die Bedingung erfüllen, erscheint eine freundliche Meldung
  - Die Seite zeigt keinen leeren Bereich ohne Erklärung

### Nicht-funktionale Kriterien

- [ ] **K5: Performance**
  - Die gefilterte Ansicht lädt ohne sichtbare Verzögerung (< 500ms)
  - Kein zusätzlicher Ladeschritt gegenüber manuell gesetzten Filtern

- [ ] **K6: Barrierefreiheit**
  - Beide aktiven Filter-Zustände sind für Screenreader korrekt kommuniziert (ARIA-Attribute, `aria-pressed`)
  - Tastatur-Navigation funktioniert vollständig
  - WCAG 2.1 Level A konform

---

## 4. Technische Planung

### Datenmodell

Kein neues Datenbankfeld erforderlich. Die Kombination aus bestehendem `planned_date`-Feld und `categories`-JSON-Array reicht aus.

Die kombinierte Filterlogik:
- `WHERE (planned_date IS NULL OR planned_date <= today)` — schließt Zukunftsdaten aus
- `AND categories JSON-Array enthält 'Mittagessen'` — nur Mittagessen-Rezepte
- `ORDER BY CASE WHEN planned_date IS NULL THEN 0 ELSE 1 END ASC, planned_date ASC` — älteste zuerst

### UI/UX-Spezifikation

**Änderung im Wochenübersicht-Template:**
- Der bestehende Klick-Einstiegspunkt aus Story 34 generiert eine URL mit beiden Parametern: `?filter=laenger-nicht-gemacht&kategorie=mittagessen`
- Keine sichtbare UI-Änderung an der Wochenübersicht selbst — nur die Ziel-URL wird um den Kategorie-Parameter ergänzt

**Ziel-Ansicht (Rezept-Übersicht):**
- Beide Filter erscheinen als aktiv markiert: "Länger nicht gemacht"-Button ist hervorgehoben, "Mittagessen"-Kategorie-Chip ist hervorgehoben
- Die Rezeptliste zeigt nur Mittagessen-Rezepte mit Vergangenheitsdaten oder ohne Datum, sortiert nach "Länger nicht gemacht"
- Bei keinen Treffern: Hinweistext statt leerer Liste

---

## 5. Nicht-funktionale Anforderungen

### Performance
- Seite lädt ohne sichtbare Verzögerung (< 500ms)
- Kein separater Index notwendig für den MVP-Umfang (bis ca. 200 Rezepte)

### Browser-Support
- Aktuelle Chrome, Firefox, Safari, Edge Versionen
- iOS Safari und Android Chrome (letzte 2 Versionen)

### Barrierefreiheit
- WCAG 2.1 Level A konform
- Beide aktive Filter-Zustände werden per ARIA kommuniziert (`aria-pressed`)
- Fokus-Indikatoren sichtbar

---

## 6. Teststrategie

### E2E-Tests (Playwright)

**Testfall 1: Klick aus Wochenübersicht öffnet Suche mit beiden Filtern**
```gherkin
Given Die App enthält Mittagessen-Rezepte mit vergangenen Daten
And Die Wochenübersicht ist geöffnet
When Nutzerin klickt auf den "Länger nicht gemacht"-Link in der Wochenübersicht
Then Die Rezept-Übersicht öffnet sich
And Der Filter "Länger nicht gemacht" ist aktiv (visuell hervorgehoben)
And Der Kategorie-Filter "Mittagessen" ist vorselektiert (visuell hervorgehoben)
And Nur Mittagessen-Rezepte mit Vergangenheitsdaten oder ohne Datum werden angezeigt
```

**Testfall 2: URL enthält beide Parameter**
```gherkin
Given Nutzerin klickt auf den "Länger nicht gemacht"-Link in der Wochenübersicht
When Die Rezept-Übersicht lädt
Then Die URL enthält den Parameter für "Länger nicht gemacht"
And Die URL enthält den Parameter "kategorie=mittagessen"
```

**Testfall 3: DeepLink mit beiden Parametern**
```gherkin
Given Die App enthält Mittagessen-Rezepte und Brot-Rezepte
When Nutzerin ruft direkt die URL mit beiden Parametern auf
Then Nur Mittagessen-Rezepte werden angezeigt (nach "Länger nicht gemacht" sortiert)
And Brot-Rezepte werden nicht angezeigt
And Beide Filter-Elemente sind visuell als aktiv markiert
```

**Testfall 4: Vorselektierter Filter kann abgewählt werden**
```gherkin
Given Die gefilterte Suche (Mittagessen + Länger nicht gemacht) ist aktiv
When Nutzerin klickt auf den "Mittagessen"-Kategorie-Filter
Then Der "Mittagessen"-Filter wird deaktiviert
And Alle Rezepte (aller Kategorien) werden nach "Länger nicht gemacht" sortiert angezeigt
And Die URL enthält nicht mehr den Parameter "kategorie=mittagessen"
```

**Testfall 5: Keine passenden Mittagessen-Rezepte**
```gherkin
Given Alle Mittagessen-Rezepte haben ein Datum in der Zukunft
When Nutzerin klickt auf den "Länger nicht gemacht"-Link in der Wochenübersicht
Then Eine freundliche Meldung wird angezeigt (keine leere Seite)
And Beide Filter sind weiterhin als aktiv erkennbar
```

---

## 7. Abhängigkeiten & Rahmenbedingungen

### Abhängigkeiten

- **Story 34** muss implementiert sein — der Klick-Einstiegspunkt in der Wochenübersicht muss existieren; Story 35 passt nur die Ziel-URL an
- **Story 9** (Filter "Länger nicht gemacht") muss implementiert sein
- **Story 8** (Filter nach Kategorien) muss implementiert sein
- **Story 12** (Kombinierte Filter) muss implementiert sein — die Kombination beider Filter muss funktionieren

### Rahmenbedingungen

- SQLite-Datenbank muss existieren und erreichbar sein
- Keine Authentifizierung erforderlich (LAN-only)
- DeepLink-fähige URLs mit Query-Parametern (Architektur-Vorgabe)
- Die Kategorie "Mittagessen" ist hardcoded (keine dynamische Verwaltung)

---

## Offene Punkte / Fragen

- Keine offenen Fragen — Story ist vollständig durch den Kontext der Vorgänger-Stories definiert.

---

**Letzte Aktualisierung:** 2026-04-02
