# Story 40: Filter standardmäßig eingeklappt

**Epic:** Epic 3: Suche & Filterung
**Priorität:** MVP Phase 2
**Status:** Offen

---

## 1. Story-Satz

Als **User der Rezepte-App** möchte ich **beim ersten Aufruf der Seite die Filter standardmäßig eingeklappt sehen**, damit ich **eine übersichtlichere Ansicht habe und nur bei Bedarf die Filter aufklappe**.

---

## 2. Geschäftsbezogene Details

### Kontext

Story 37 (Einklappen der Filter) hat die Möglichkeit geschaffen, Filter ein- und auszuklappen. Aktuell werden Filter beim ersten Seitenaufruf aber noch ausgeklappt angezeigt, was auf mobilen Geräten viel Platz einnimmt und den Nutzer zwingt, erst durch den Filterbereich zu scrollen, bevor er die Rezeptliste sieht.

Da Filter nicht bei jedem Besuch gebraucht werden (z.B. wenn man nur die Rezeptliste durchblättern möchte), soll der Standardzustand geändert werden: Filter sind initial eingeklappt und müssen bei Bedarf aufgeklappt werden.

### Nutzergruppe

Beide Partner, die die App hauptsächlich auf dem Smartphone nutzen und oft nur die Rezeptliste sehen wollen, ohne aktiv zu filtern.

### Business-Value

- Mehr Platz für die Rezeptliste beim ersten Öffnen der App
- Schnellerer Zugriff auf Rezepte ohne Scrollen durch den Filterbereich
- Bessere Nutzererfahrung auf mobilen Geräten
- Der Nutzer entscheidet selbst, wann er Filter braucht

### Edge Cases

- **Filter explizit ausklappen:** Wenn der Nutzer die Filter aufklappt und dann die Seite neu lädt, soll der ausgeklappte Zustand erhalten bleiben (URL-Parameter `filter_collapsed=0` oder kein Parameter)
- **Filter aktiv und eingeklappt:** Wenn aktive Filter gesetzt sind und die Seite neu geladen wird, bleiben die Filter eingeklappt - der visuelle Indikator am Toggle-Button (aus Story 37) zeigt weiterhin an, dass Filter aktiv sind
- **Deep-Links mit gespeicherten Filtern:** Links zu gespeicherten Filtern enthalten keinen `filter_collapsed` Parameter, daher werden die Filter standardmäßig eingeklappt angezeigt

---

## 3. Akzeptanzkriterien

### Funktionale Kriterien

- [ ] **K1: Standardmäßig eingeklappt**
  - Beim Aufruf der Hauptseite ohne `filter_collapsed` Parameter sind die Filter eingeklappt
  - Die Suchleiste bleibt wie bisher immer sichtbar

- [ ] **K2: Toggle-Button funktioniert wie bisher**
  - Der Button "Filter anzeigen/ausblenden" ist weiterhin verfügbar
  - Beim Klick aufklappen, erneuter Klick zum Einklappen funktioniert weiterhin

- [ ] **K3: Ausgeklappter Zustand bleibt erhalten**
  - Wenn der Nutzer die Filter aufklappt, wird der Zustand in der URL gespeichert (`filter_collapsed=0` oder Parameter entfernt)
  - Bei Seitenneuladen bleiben die Filter ausgeklappt

### Nicht-funktionale Kriterien

- [ ] **K4: Performance**
  - Keine zusätzliche Ladezeit durch die Änderung
  - Serverseitiges Rendering bleibt unverändert schnell

- [ ] **K5: Barrierefreiheit**
  - Der eingeklappte Zustand wird korrekt per `aria-expanded="false"` kommuniziert
  - Toggle-Button bleibt fokussierbar und bedienbar

---

## 4. Technische Planung

### Datenmodell

Keine Datenbankänderungen notwendig. Es handelt sich nur um eine Änderung des Default-Verhaltens des bestehenden `filter_collapsed` URL-Parameters.

### UI/UX-Spezifikation

**Aktuelles Verhalten (Story 37):**
- Ohne `filter_collapsed` Parameter: Filter sind ausgeklappt
- Mit `filter_collapsed=1`: Filter sind eingeklappt

**Neues Verhalten:**
- Ohne `filter_collapsed` Parameter: Filter sind eingeklappt (Default geändert)
- Mit `filter_collapsed=0`: Filter sind ausgeklappt (explizit)
- Mit `filter_collapsed=1`: Filter sind eingeklappt (wie bisher)

**Umsetzungsansatz:**
- Die Logik für den Toggle-Button bleibt unverändert
- Die serverseitige Logik zur Bestimmung des Zustands wird angepasst:
  - Vorher: `filter_collapsed == Some("1")` → eingeklappt, sonst ausgeklappt
  - Nachher: `filter_collapsed == Some("0")` → ausgeklappt, sonst eingeklappt
- Der Toggle-Button generiert Links mit dem jeweils anderen Zustand

---

## 5. Nicht-funktionale Anforderungen

### Performance
- Seite lädt ohne sichtbare Verzögerung (< 500ms)
- Die Umkehrung des Defaults hat keinen Einfluss auf die Ladezeit

### Browser-Support
- Aktuelle Chrome, Firefox, Safari, Edge Versionen
- iOS Safari und Android Chrome (letzte 2 Versionen)

### Barrierefreiheit
- WCAG 2.1 Level A konform
- Fokus-Indikatoren sichtbar

---

## 6. Teststrategie

### E2E-Tests (Playwright)

**Testfall 1: Filter standardmäßig eingeklappt**
```gherkin
Given Die Hauptseite wird ohne URL-Parameter aufgerufen
Then Sind alle Filterbereiche (Kategorien, Spezialfilter, gespeicherte Filter) eingeklappt
And Die Suchleiste ist sichtbar
And Der Toggle-Button zeigt "Filter anzeigen ▶" an
```

**Testfall 2: Filter ausklappen und Zustand erhalten**
```gherkin
Given Die Hauptseite ist geöffnet (Filter sind eingeklappt)
When Der Nutzer auf "Filter anzeigen" klickt
Then Werden alle Filterbereiche eingeblendet
And Die URL enthält "filter_collapsed=0" oder keinen Parameter
When Die Seite neu geladen wird
Then Bleiben die Filter ausgeklappt
```

**Testfall 3: Eingeklappter Zustand mit Parameter**
```gherkin
Given Die Hauptseite wird mit "?filter_collapsed=1" aufgerufen
Then Sind die Filterbereiche eingeklappt
And Der Toggle-Button zeigt "Filter anzeigen ▶" an
```

---

## 7. Abhängigkeiten & Rahmenbedingungen

### Abhängigkeiten
- Story 37 (Einklappen der Filter) muss implementiert sein — ist abgeschlossen
- Alle Filter-Stories (07, 08, 09, 10, 11, 12, 13) müssen implementiert sein — sind alle abgeschlossen

### Rahmenbedingungen
- SQLite-Datenbank muss existieren und erreichbar sein
- Keine Authentifizierung erforderlich (LAN-only)
- Die Änderung ist rein clientseitig/serverseitig im Rendering, keine Datenmigration notwendig

---

## Offene Punkte / Fragen

- [ ] Keine offenen Punkte

---

**Letzte Aktualisierung:** 2026-04-04
