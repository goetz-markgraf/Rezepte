# Implementierungsplan: Story 32 - Neues-Rezept-Button in der Kopfzeile

## Zusammenfassung der Story

Ein "+ Neues Rezept"-Button soll in der globalen Kopfzeile (Header) hinzugefügt werden, der auf allen Seiten sichtbar ist und direkt zur Seite `/recipes/new` navigiert. Der Button soll visuell als Primary Action hervorgehoben sein, barrierefrei (Tastatur-Navigation, Screenreader), und auf Mobile entweder als Icon oder reduzierter Text dargestellt werden.

**Akzeptanzkriterien:**
- K1: Button ist rechts in der Kopfzeile auf allen Seiten sichtbar
- K2: Klick navigiert zu `/recipes/new`
- K3: Beschriftung "+ Neues Rezept" (oder Icon auf Mobile)
- K4: Barrierefrei (semantisches Element, Aria-Label, Fokus-Indikator)
- K5: Performance: Keine zusätzlichen Requests
- K6: WCAG 2.1 Level AA Konformität

---

## Technische Analyse

### Architektur-Konformität
- **Tech Stack:** Rust + Axum + Askama - keine Änderungen am Backend nötig
- **Server-Side Rendering:** Button wird im Base-Template gerendert
- **Keine JSON-API:** Nur ein einfacher Link/Button im Template
- **Keine DB-Änderungen:** Keine Migrationen nötig

### Bestehende Struktur (aus Architektur-Dokument)
```
src/templates/
├── base.html      # ← Hier muss der Button hin
├── recipes/
│   ├── list.html
│   ├── detail.html
│   └── form.html  # ← Zielseite existiert bereits
```

### Lösungsansatz
Der Button wird im `base.html`-Template hinzugefügt, da dies das Layout-Template ist, das von allen Seiten verwendet wird. Es handelt sich um einen einfachen `<a>`-Link, der als Button gestylt wird, da er eine Navigation auslöst (Progressive Enhancement - funktioniert ohne JS).

**Mobile Strategie:**
- Viewport-abhängige Darstellung mit CSS Media Queries
- Bei kleinen Bildschirmen (< 768px): Nur Icon (+) mit Tooltip
- Bei größeren Bildschirmen: Icon + Text "Neues Rezept"

**Barrierefreiheit:**
- Semantisches `<a>`-Element (da es eine Navigation ist)
- `aria-label` für Screenreader
- Sichtbarer Fokus-Ring (`:focus-visible`)
- Touch-Target mindestens 44x44px

---

## Implementierungsschritte

### Schritt 1: CSS-Styling vorbereiten
- [ ] In `src/static/css/app.css`: Neue CSS-Klasse `.btn-primary` erstellen
- [ ] Styling für Header-Button definieren (Hintergrundfarbe, Padding, Hover-Effekt)
- [ ] Media Query für Mobile-Ansicht (< 768px): Text ausblenden, Icon anzeigen
- [ ] Fokus-Indikator definieren (`outline` für Tastatur-Navigation)
- [ ] Touch-Target-Größe sicherstellen (min-width/min-height: 44px)

### Schritt 2: Button in Base-Template einfügen
- [ ] `src/templates/base.html` öffnen
- [ ] Navigation-Bereich identifizieren (wo andere Nav-Links sind)
- [ ] "+ Neues Rezept"-Button rechts im Header einfügen
- [ ] Verwendung von `<a>`-Element mit `href="/recipes/new"`
- [ ] CSS-Klasse `.btn-primary` zuweisen
- [ ] `aria-label="Neues Rezept erstellen"` hinzufügen
- [ ] Icon (z.B. Lucide "plus") als SVG einfügen

### Schritt 3: Mobile-Optimierung
- [ ] CSS-Klasse `.btn-text` erstellen (wird auf Mobile ausgeblendet)
- [ ] CSS-Klasse `.btn-icon-only` für Mobile-Ansicht
- [ ] Testen: Button ist auf Desktop und Mobile sichtbar und klickbar

### Schritt 4: Barrierefreiheit validieren
- [ ] Fokus-Ring ist sichtbar bei Tab-Navigation
- [ ] Screenreader gibt "Neues Rezept erstellen" aus
- [ ] Kontrastverhältnis prüfen (WCAG AA: mindestens 4.5:1)

### Schritt 5: E2E-Tests erstellen
- [ ] Testdatei `tests/e2e/header-navigation.spec.ts` erstellen
- [ ] Test 1: Button ist auf Startseite sichtbar
- [ ] Test 2: Button ist auf Wochenvorschau-Seite sichtbar
- [ ] Test 3: Button ist auf Dubletten-Prüf-Seite sichtbar
- [ ] Test 4: Klick navigiert zu `/recipes/new`
- [ ] Test 5: Tastatur-Navigation (Tab + Enter) funktioniert
- [ ] Test 6: Mobile Ansicht (Viewport 375px) - Button sichtbar/klickbar

### Schritt 6: Code-Qualität prüfen
- [ ] `cargo build` - keine Compiler-Fehler
- [ ] `cargo clippy -- -D warnings` - keine Linting-Fehler
- [ ] `cargo fmt --check` - korrekte Formatierung
- [ ] CSS validieren (keine Syntaxfehler)

### Schritt 7: Manueller Test
- [ ] App starten (`cargo run`)
- [ ] Startseite öffnen - Button ist sichtbar
- [ ] Auf Button klicken - Navigation zu `/recipes/new`
- [ ] Tastatur-Navigation testen (Tab bis Button, Enter drücken)
- [ ] Mobile Viewport im Browser testen (DevTools)

---

## Dateien die geändert werden müssen

| Datei | Änderung |
|-------|----------|
| `src/templates/base.html` | Button im Header-Navigation-Bereich hinzufügen |
| `src/static/css/app.css` | CSS-Klassen für Button-Styling hinzufügen |

---

## Tests die geschrieben werden müssen

### Unit Tests
- **Keine Unit Tests nötig**, da keine Rust-Logik implementiert wird (nur Template- und CSS-Änderungen)

### Integration Tests
- **Keine Integration Tests nötig**, da keine neuen Endpunkte erstellt werden

### E2E Tests (Playwright)
**Datei:** `tests/e2e/header-navigation.spec.ts`

1. **Button ist auf allen Seiten sichtbar**
   - Startseite (`/`)
   - Rezept-Liste (`/recipes`)
   - Wochenvorschau (sofern existiert)
   - Dubletten-Prüfung (sofern existiert)

2. **Button navigiert korrekt**
   - Klick auf Button → URL ist `/recipes/new`
   - Formular zum Rezept-Erstellen ist sichtbar

3. **Tastatur-Navigation**
   - Tab bis Button fokussiert ist
   - Enter drücken navigiert zu `/recipes/new`

4. **Mobile Darstellung**
   - Viewport auf 375px setzen
   - Button ist noch sichtbar und klickbar
   - Entweder Icon-only oder Text reduziert

---

## Risiken und Abhängigkeiten

### Abhängigkeiten
- **Story 01 (Rezept erstellen)** muss implementiert sein
  - Die Zielseite `/recipes/new` muss existieren und funktionieren
  - **Status:** Voraussetzung erfüllt (laut Story-Dokument)

### Risiken
| Risiko | Wahrscheinlichkeit | Auswirkung | Mitigation |
|--------|-------------------|------------|------------|
| Button überlappt mit anderen Header-Elementen auf Mobile | Mittel | Mittel | Responsive Design mit Media Queries, ggf. Hamburger-Menü |
| Zielseite `/recipes/new` existiert nicht oder hat andere URL | Niedrig | Hoch | Vor Implementierung URL prüfen, ggf. anpassen |
| CSS-Konflikte mit bestehenden Header-Styles | Niedrig | Niedrig | Bestehende CSS-Struktur analysieren, konsistente Klassen verwenden |
| Barrierefreiheit nicht vollständig (Fokus, Screenreader) | Niedrig | Mittel | Manuelle Tests mit Screenreader (VoiceOver/NVDA) |

### Offene Punkte (zu klären)
- [ ] **Icon-Quelle:** Sollen Lucide-Icons verwendet werden (wie in Architektur-Dokument erwähnt) oder einfaches Unicode-Plus (+)?
- [ ] **Farbschema:** Welche Akzent-Farbe soll für den Primary-Button verwendet werden? (muss zu bestehendem Design passen)
- [ ] **Mobile Strategie:** Soll auf Mobile der Text komplett ausgeblendet werden oder nur gekürzt werden?

---

## Definition of Done Checkliste

### Code-Qualität
- [ ] Keine Compiler-Fehler (`cargo build`)
- [ ] Keine Clippy-Warnings
- [ ] Code formatiert (`cargo fmt`)
- [ ] CSS validiert

### Architektur-Einhaltung
- [ ] Server-Side Rendering (keine JS-API)
- [ ] Button funktioniert ohne JavaScript (normaler Link)
- [ ] Keine DB-Änderungen nötig
- [ ] DeepLink-fähig (direkte URL `/recipes/new`)

### Testing
- [ ] E2E-Tests für alle Akzeptanzkriterien geschrieben
- [ ] Tests laufen erfolgreich (`npm run test:e2e`)
- [ ] Manueller Test auf Desktop und Mobile

### Funktionale Anforderungen
- [ ] Alle Akzeptanzkriterien (K1-K6) erfüllt
- [ ] Button auf allen Seiten sichtbar
- [ ] Navigation zu `/recipes/new` funktioniert
- [ ] Barrierefreiheit gewährleistet (Tastatur, Screenreader)

### Performance & Sicherheit
- [ ] Keine zusätzlichen Server-Requests
- [ ] Ladezeit nicht beeinträchtigt
- [ ] XSS-sicher (kein User-Input im Button)

---

## Aufwandsschätzung

- **CSS-Styling:** 30 Min
- **Template-Änderung:** 15 Min
- **E2E-Tests:** 45 Min
- **Testing & Bugfixing:** 30 Min
- **Gesamt:** ca. 2 Stunden

---

**Erstellt:** 2026-03-30
**Geplant für:** Sprint MVP Phase 2
