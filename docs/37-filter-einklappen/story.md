# Story 37: Einklappen der Filter

**Epic:** Epic 3: Suche & Filterung
**Priorität:** Nice-to-have
**Status:** In Arbeit

---

## 1. Story-Satz

Als **Nutzer** möchte ich **die Filterleiste ein- und ausklappen können**, damit ich **auf kleinen Displays mehr Platz für die Rezeptliste habe**.

---

## 2. Geschäftsbezogene Details

### Kontext

Die Hauptansicht enthält mehrere Filterbereiche: Kategorie-Filter, Spezialfilter (Länger nicht gemacht, Nächste 7 Tage, Bewertungsfilter), gespeicherte Filter und den Bereich zum Speichern neuer Filter. Auf mobilen Geräten nehmen diese Filter einen erheblichen Teil des Bildschirms ein, bevor die eigentliche Rezeptliste beginnt.

Nutzer, die ihre Filter bereits gesetzt haben, wollen danach hauptsächlich die Rezeptliste sehen und nicht ständig durch den Filterblock scrollen müssen.

### Nutzergruppe

Beide Partner, die die App hauptsächlich auf dem Smartphone oder Tablet nutzen. Der Wunsch nach mehr Platz für die Rezeptliste entsteht besonders nach dem initialen Setzen der Filter.

### Business-Value

- Deutlich mehr Rezeptliste auf kleinen Displays sichtbar
- Nutzer können nach dem Filtern bequem in der Liste scrollen, ohne den Filter-Block jedes Mal zu sehen
- Der eingeklappte Zustand ist über die URL teilbar und buchmarkierbar — ein Direktlink kann bereits im eingeklappten Modus ankommen

### Edge Cases

- **Filter eingeklappt, aber aktive Filter:** Wenn Filter aktiv sind und der Block eingeklappt ist, muss trotzdem erkennbar sein, dass aktive Filter gesetzt sind (z.B. durch ein visuelles Indikator am Toggle-Button).
- **Kein JavaScript verfügbar:** Die Seite muss auch ohne JavaScript funktionieren. Das Einklappen über URL-Parameter stellt sicher, dass der Zustand serverseitig bekannt ist und die Filter entsprechend gerendert werden können.
- **URL-Teilen:** Wenn jemand einen Link mit `filter_collapsed=1` teilt, öffnet der Empfänger die Seite mit eingeklappten Filtern — das ist gewünscht.
- **Direkt-Navigation mit gespeichertem Filter:** Gespeicherte Filter-Links enthalten keinen `filter_collapsed`-Parameter. Die Filter sind dann standardmäßig ausgeklappt.

---

## 3. Akzeptanzkriterien

### Funktionale Kriterien

- [ ] **K1: Toggle-Button sichtbar**
  - Ein Button (z.B. "Filter anzeigen / Filter ausblenden") ist oberhalb des Filterblocks dauerhaft sichtbar
  - Der Button zeigt den aktuellen Zustand an (z.B. mit Pfeil-Icon: ▼ ausgeklappt, ▶ eingeklappt)

- [ ] **K2: Filter einklappen**
  - Beim Klick auf den Toggle-Button werden alle Filterbereiche (Kategorien, Spezialfilter, gespeicherte Filter, Filter-Speichern-Bereich) ausgeblendet
  - Die Suchleiste bleibt immer sichtbar — sie wird nicht eingeklappt

- [ ] **K3: Filter ausklappen**
  - Beim erneuten Klick werden alle Filterbereiche wieder eingeblendet

- [ ] **K4: Zustand in der URL**
  - Der eingeklappte Zustand wird über den Query-Parameter `filter_collapsed=1` in der URL abgebildet
  - Im ausgeklappten Zustand ist der Parameter nicht in der URL vorhanden (oder `filter_collapsed=0`)
  - Beim Klick auf den Toggle wird die URL entsprechend aktualisiert (Seitennavigation oder HTMX)

- [ ] **K5: Zustand beim Seitenaufruf**
  - Beim Aufruf der Seite mit `filter_collapsed=1` in der URL sind die Filter direkt eingeklappt
  - Beim Aufruf ohne diesen Parameter sind die Filter ausgeklappt (Standard)

- [ ] **K6: Aktive Filter sichtbar bei eingeklapptem Zustand**
  - Wenn mindestens ein Filter aktiv ist und der Filterbereich eingeklappt ist, zeigt der Toggle-Button einen visuellen Hinweis (z.B. Badge, anderes Icon oder zusätzlicher Text), dass aktive Filter vorhanden sind

- [ ] **K7: Filter-Links aus gespeicherten Filtern**
  - Gespeicherte Filter-Links zeigen nach Klick die Filter standardmäßig ausgeklappt (da kein `filter_collapsed`-Parameter im gespeicherten Link)

### Nicht-funktionale Kriterien

- [ ] **K8: Funktioniert ohne JavaScript**
  - Da der Zustand über URL-Parameter abgebildet wird, funktioniert das Einklappen vollständig ohne JavaScript (serverseitiges Rendering)
  - Mit JavaScript kann der Toggle per HTMX oder direkter Navigation ohne Seitenneuladen erfolgen

- [ ] **K9: Barrierefreiheit**
  - Der Toggle-Button hat ein korrektes `aria-expanded`-Attribut (true/false)
  - Der Filterbereich hat ein passendes `aria-label`
  - Tastaturnavigation: Toggle per Enter/Space bedienbar

---

## 4. Technische Planung

### Datenmodell

Keine Datenbankänderungen notwendig. Der Zustand wird ausschließlich über einen URL-Query-Parameter `filter_collapsed` (Wert `1` = eingeklappt) transportiert.

### UI/UX-Spezifikation

**Struktur der Hauptansicht (aktuell):**
1. Suchformular
2. Kategoriefilter-Leiste (`category-filter`)
3. Spezialfilter (`sort-filter`)
4. Gespeicherte Filter (`saved-filters`)
5. Filter-Speichern-Bereich (`save-filter-area`)
6. Rezeptliste

**Neue Struktur:**
1. Suchformular *(bleibt immer sichtbar)*
2. **Toggle-Button** "Filter anzeigen/ausblenden" mit Zustandsanzeige
3. Filterbereich (ein-/ausklappbar):
   - Kategoriefilter-Leiste
   - Spezialfilter
   - Gespeicherte Filter
   - Filter-Speichern-Bereich
4. Rezeptliste

**Toggle-Button-Design:**
- Schlanker Button, ähnlich dem Stil der restlichen App
- Zeigt entweder "Filter ▼" (ausgeklappt) oder "Filter ▶" (eingeklappt)
- Bei aktiven Filtern und eingeklapptem Zustand: zusätzliche Markierung (z.B. Punkt oder "(aktiv)")

**Umsetzungsansatz:**
- Der Server rendert den Filterbereich eingeklappt (`display: none` via CSS-Klasse `filter-collapsed`) basierend auf dem URL-Parameter
- Der Toggle-Button ist ein einfacher Link, der den aktuellen URL-Zustand mit umgekehrtem `filter_collapsed`-Wert aufruft
- Optional: Mit JavaScript kann der Toggle ohne Seitenneuladen funktionieren (HTMX oder vanilla JS), muss aber als Progressive Enhancement umgesetzt sein

---

## 5. Nicht-funktionale Anforderungen

### Performance
- Seite lädt ohne sichtbare Verzögerung (< 500ms)
- Kein zusätzlicher Server-Request für den Toggle notwendig (URL-Navigation reicht)

### Browser-Support
- Aktuelle Chrome, Firefox, Safari, Edge Versionen
- iOS Safari und Android Chrome (letzte 2 Versionen)

### Barrierefreiheit
- WCAG 2.1 Level A konform
- Toggle-Button: `aria-expanded` korrekt gesetzt
- Filterbereich: bei eingeklappt `hidden` oder `aria-hidden="true"`
- Fokus-Indikatoren sichtbar

---

## 6. Teststrategie

### E2E-Tests (Playwright)

**Testfall 1: Filter einklappen**
```gherkin
Given Die Hauptseite ist geöffnet und die Filter sind sichtbar
When Der Nutzer auf den Toggle-Button "Filter ausblenden" klickt
Then Verschwinden alle Filterbereiche (Kategorien, Spezialfilter, gespeicherte Filter)
And Die Suchleiste bleibt sichtbar
And Die URL enthält den Parameter "filter_collapsed=1"
```

**Testfall 2: Filter ausklappen**
```gherkin
Given Die Hauptseite ist mit "filter_collapsed=1" in der URL geöffnet
Then Sind alle Filterbereiche ausgeblendet
When Der Nutzer auf den Toggle-Button "Filter anzeigen" klickt
Then Werden alle Filterbereiche wieder eingeblendet
And Die URL enthält keinen "filter_collapsed=1"-Parameter mehr
```

**Testfall 3: Eingeklappter Zustand via URL**
```gherkin
Given Ein Nutzer ruft die Seite direkt mit "?filter_collapsed=1" auf
Then Sind die Filterbereiche von Anfang an ausgeblendet
And Der Toggle-Button zeigt an, dass Filter eingeklappt sind
```

**Testfall 4: Aktive Filter sichtbar bei eingeklapptem Zustand**
```gherkin
Given Ein aktiver Kategoriefilter ist gesetzt und der Filterbereich ist eingeklappt
Then Zeigt der Toggle-Button einen Hinweis auf aktive Filter
```

---

## 7. Abhängigkeiten & Rahmenbedingungen

### Abhängigkeiten
- Story 07 (Volltextsuche), Story 08 (Kategoriefilter), Story 09/10/11 (Spezialfilter), Story 13 (Gespeicherte Filter) müssen implementiert sein — sind alle abgeschlossen.

### Rahmenbedingungen
- SQLite-Datenbank muss existieren und erreichbar sein
- Keine Authentifizierung erforderlich (LAN-only)
- Der Suchbereich (`search-form`) wird **nicht** eingeklappt — er bleibt immer sichtbar
- Progressive Enhancement: Basisverhalten muss ohne JavaScript funktionieren

---

## Offene Punkte / Fragen

- [ ] Soll der Toggle-Button innerhalb oder außerhalb des Suchformulars platziert werden?
- [ ] Soll die genaue visuelle Kennzeichnung aktiver Filter bei eingeklapptem Zustand (Badge vs. Text vs. Icon) im Plan festgelegt werden?

---

**Letzte Aktualisierung:** 2026-04-03
