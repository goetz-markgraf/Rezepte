# Implementierungsplan: Story 27 - Clear-Icon in Volltextsuche triggert neue Suche

## Analyse des Ist-Zustands

Das Suchfeld in `templates/index.html` ist ein `<input type="search">`. Browser (Chrome, Safari, Edge)
rendern für diesen Input-Typ automatisch ein natives Clear-Icon (X), das über das CSS-Pseudo-Element
`::-webkit-search-cancel-button` gesteuert wird.

**Problem:** Das native Clear-Icon des Browsers löst in manchen Browsern kein `input`-Event aus (Firefox)
und ist nicht zuverlässig cross-browser. Außerdem ist es nicht per CSS oder `aria-label` barrierefrei
anpassbar. Das HTMX-Trigger-Attribut `hx-trigger="input changed delay:300ms"` auf dem Suchfeld würde
selbst wenn das Event ausgelöst wird, nur den aktuellen Feldwert senden — der Wert ist zu diesem
Zeitpunkt aber schon leer, was die gewünschte Suche auslösen würde. Das ist jedoch nicht garantiert.

**Gewählter Ansatz:** Das native Browser-Clear-Icon wird per CSS ausgeblendet und durch einen eigenen
`<button>`-Element ersetzt. Dieser Button:

1. Ist nur sichtbar, wenn das Suchfeld einen nicht-leeren Wert enthält (CSS + ggf. JS-Toggle).
2. Löscht das Suchfeld beim Klick (JS-Event-Handler).
3. Löst danach eine HTMX-Anfrage aus (oder submitted das Formular als Fallback ohne JS).

**Technischer Ansatz im Detail:**

- Der Clear-Button wird als `<button type="submit">` im Suchformular platziert. Beim Klick ohne JS
  wird das Formular mit dem dann leeren `q`-Feld abgeschickt (da JS den Wert zuerst geleert hat),
  was `GET /?q=` ergibt — die vollständige Liste.
- Mit JS: Ein `click`-Listener auf dem Button leert das Suchfeld und löst manuell ein `hx-get`
  oder `htmx.trigger` aus, damit die HTMX-Live-Suche die Liste sofort aktualisiert.
- Die Sichtbarkeit des Buttons wird über eine CSS-Klasse gesteuert, die per JS beim `input`-Event
  gesetzt wird. Ohne JS ist der Button immer sichtbar (Progressive Enhancement).

**Warum kein reiner `hx-get`-Ansatz auf dem Button?** HTMX-Attribute auf dem Button könnten direkt
`hx-get="/" hx-target="#recipe-results" hx-vals='{"q": ""}' hx-push-url="true"` setzen. Das hätte den
Vorteil, ohne JS-Event-Handler auszukommen. Nachteil: Das Suchfeld bleibt optisch mit dem alten Wert
gefüllt, bis die Antwort kommt. Ein `hx-on::before-request` könnte das Feld leeren, aber das ist
komplex. Stattdessen: kleines inline-JS beim Click-Event, das das Feld leert und dann HTMX triggert.

---

## Technische Schritte

### Schritt 1: Template anpassen (`templates/index.html`)

- [ ] Natives Browser-Clear-Icon per CSS ausblenden (`::-webkit-search-cancel-button { display: none }`)
  — im gleichen Schritt mit dem CSS-Schritt unten, aber als Kommentar vormerken
- [ ] Custom Clear-Button direkt nach dem `<input id="q">` einfügen:
  ```html
  <button
      type="submit"
      id="clear-search"
      class="btn-icon search-clear-btn"
      aria-label="Suche zurücksetzen"
      onclick="document.getElementById('q').value=''; htmx.trigger('#q', 'input');"
  >{% call icons::icon_x() %}</button>
  ```
- [ ] Den Button per CSS standardmäßig ausblenden (`display: none` oder `visibility: hidden`),
  wenn das Suchfeld leer ist
- [ ] JS-Snippet im Template hinzufügen (oder als separate Datei, hier: inline `<script>` im Block
  unterhalb des Formulars):
  - Bei `input`-Event auf `#q`: wenn Wert nicht leer → Button anzeigen; wenn leer → verstecken
  - `DOMContentLoaded`: initialen Zustand setzen (wichtig für Browser-Back/DeepLink mit `?q=...`)
- [ ] Template muss weiterhin ohne JS funktionieren: wenn JS deaktiviert, ist der Button immer
  sichtbar (da CSS-Klassen nicht gesetzt werden) und submitted das Formular mit leerem Wert

**Konkrete Änderung in `templates/index.html`:**

```
Vorher:
  <input id="q" name="q" type="search" ... >
  <button type="submit" class="btn-primary">Suchen</button>

Nachher:
  <div class="search-field-wrapper">
      <input id="q" name="q" type="search" ... >
      <button id="clear-search" type="submit" class="btn-icon search-clear-btn"
              aria-label="Suche zurücksetzen"
              onclick="event.preventDefault(); document.getElementById('q').value=''; htmx.trigger(document.getElementById('q'), 'input');"
      >{% call icons::icon_x() %}</button>
  </div>
  <button type="submit" class="btn-primary">Suchen</button>
```

Hinweis: `event.preventDefault()` verhindert den nativen Form-Submit beim Klick mit JS. HTMX
übernimmt die Anfrage. Ohne JS (kein `event.preventDefault()`) submitted das Formular normal.

Alternativ: onclick-Handler in einem separaten `<script>`-Block am Ende des Templates, um den
HTML-Block sauber zu halten.

- [ ] `<script>`-Block am Ende des `{% block content %}`-Bereichs:
  ```javascript
  (function() {
      const input = document.getElementById('q');
      const clearBtn = document.getElementById('clear-search');
      if (!input || !clearBtn) return;

      function updateClearButton() {
          clearBtn.style.display = input.value.length > 0 ? '' : 'none';
      }

      clearBtn.addEventListener('click', function(event) {
          event.preventDefault();
          input.value = '';
          updateClearButton();
          htmx.trigger(input, 'input');
          input.focus();
      });

      input.addEventListener('input', updateClearButton);
      updateClearButton(); // Initialer Zustand
  })();
  ```

### Schritt 2: CSS anpassen (`src/static/css/app.css`)

- [ ] Natives Browser-Clear-Icon des `<input type="search">` ausblenden:
  ```css
  input[type="search"]::-webkit-search-cancel-button {
      display: none;
  }
  ```
- [ ] CSS-Klasse `.search-clear-btn` für Positionierung des Custom-Buttons:
  ```css
  .search-field-wrapper {
      position: relative;
      flex: 1;
      display: flex;
      align-items: center;
  }

  .search-field-wrapper input[type="search"] {
      flex: 1;
      padding-right: 2.5rem; /* Platz für Clear-Button */
  }

  .search-clear-btn {
      position: absolute;
      right: 0.5rem;
      display: none; /* initial versteckt, JS schaltet um */
      color: #6b7280;
  }

  .search-clear-btn:hover {
      color: var(--text-color);
  }
  ```
- [ ] `.search-input-group` anpassen: das `flex: 1` wandert jetzt auf `.search-field-wrapper`

### Schritt 3: Barrierefreiheit prüfen

- [ ] `aria-label="Suche zurücksetzen"` ist am Button gesetzt (bereits in Schritt 1)
- [ ] Fokus-Indikator: `.btn-icon:focus-visible` ist bereits in `app.css` definiert — keine Änderung nötig
- [ ] Button ist im Tab-Fluss: Standard-`<button>` ist immer per Tab erreichbar
- [ ] Wenn Button per Tastatur (Tab + Enter/Space) aktiviert: derselbe onclick-Handler greift

### Schritt 4: Integration-Tests (`tests/recipe_search.rs`)

Keine neuen Integrationstests für dieses Feature nötig: Das Verhalten bei `GET /?q=` ist bereits
durch bestehende Tests abgedeckt (`search_with_empty_query_shows_all_recipes`). Das Senden des
Formulars mit leerem Wert ist identisch.

- [ ] Bestehende Integrationstests laufen weiterhin durch (`cargo test`)

### Schritt 5: E2E-Tests (`tests/e2e/recipe-search.spec.ts`)

Neue Testfälle an die bestehende Datei `tests/e2e/recipe-search.spec.ts` anhängen:

- [ ] **Testfall K1: Klick auf Clear-Icon zeigt vollständige Liste**
  ```typescript
  // Given: App enthält mehrere Rezepte, Benutzer hat "Bolognese" gesucht
  // When: Klick auf Clear-Button (#clear-search)
  // Then: Suchfeld ist leer UND vollständige Rezeptliste sichtbar UND URL ohne q-Parameter
  ```

- [ ] **Testfall K2: Clear-Icon nur sichtbar wenn Suchfeld gefüllt**
  ```typescript
  // Given: Startseite geladen, Suchfeld leer
  // Then: #clear-search ist nicht sichtbar (display: none)
  // When: "Salat" in Suchfeld eingegeben
  // Then: #clear-search ist sichtbar
  ```

- [ ] **Testfall K3 (Edge Case): Suche ohne Treffer, dann Clear-Icon**
  ```typescript
  // Given: Benutzer hat "xyzxyzxyz" gesucht, "Keine Rezepte gefunden" sichtbar
  // When: Klick auf Clear-Icon
  // Then: Suchfeld ist leer UND vollständige Liste sichtbar
  ```

- [ ] **Testfall K4: Tastatur-Navigation zum Clear-Icon**
  ```typescript
  // Given: Suchbegriff eingegeben, Clear-Icon sichtbar
  // When: Tab-Taste navigiert zum Clear-Icon, Enter gedrückt
  // Then: Suchfeld leer UND vollständige Liste sichtbar
  ```

- [ ] **Testfall K5 (Initaler Zustand): DeepLink mit q-Parameter**
  ```typescript
  // Given: Seite wird mit ?q=Bolognese aufgerufen
  // Then: Clear-Icon ist sofort sichtbar (JS setzt initialen Zustand)
  ```

### Schritt 6: DoD-Checkliste

- [ ] `cargo build` ohne Warnings
- [ ] `cargo clippy -- -D warnings` ohne Warnings
- [ ] `cargo fmt --check` ohne Diff
- [ ] `cargo test` alle Tests grün
- [ ] `npm run test:e2e` alle E2E-Tests grün

---

## URL-Struktur

Keine neuen Endpunkte. Das Feature nutzt die bestehende URL-Struktur:

```
GET  /           →  Alle Rezepte (leere Suche, nach Clear-Icon-Klick)
GET  /?q=        →  Alle Rezepte (Form-Submit mit leerem q, Fallback ohne JS)
```

HTMX-Flow nach Clear-Icon-Klick (mit JS):
```
#clear-search [onclick]
  → input.value = ''
  → htmx.trigger(input, 'input')
  → HTMX-Trigger auf #q feuert: GET /?q=
  → Server rendert volle Seite
  → HTMX tauscht #recipe-results aus, URL wird zu / aktualisiert (hx-push-url="true")
```

Fallback ohne JS (Form-Submit):
```
#clear-search [type="submit"]
  → Formular wird abgeschickt mit q= (leer, da onclick-Handler ohne JS nicht läuft, Feld bleibt leer nach Klick nicht — daher button immer sichtbar ohne JS)
  → GET /?q= → vollständige Liste
```

Hinweis zum No-JS-Fallback: Ohne JS ist der Button immer sichtbar (kein JS-gesteuerte Anzeige).
Beim Klick submitted er das Formular — aber das Suchfeld ist dann noch gefüllt (JS leert es ja nicht).
Um den No-JS-Fallback zu verbessern: Der Button könnte stattdessen als Link auf `/?` gestaltet
werden, also `<a href="/" class="btn-icon ...">`. Das setzt voraus, dass das Suchfeld innerhalb des
Formulars bleibt und der Link außerhalb des Formulars liegt oder das Formular per Redirect löscht.

**Einfacherer No-JS-Fallback:** Der Button bleibt `type="submit"`. Da ohne JS der onclick-Handler
nicht ausgeführt wird, submitted das Formular mit dem noch gefüllten Wert — was das gleiche Ergebnis
wie "Suchen" ergibt. Das ist nicht ideal aber akzeptabel als Fallback. Alternativ: ein `<a href="/">`
statt `<button>`, das ohne JS funktioniert und mit JS den HTMX-Trigger übernimmt. Dieser Ansatz
wird bevorzugt:

```html
<a  id="clear-search"
    href="/"
    class="btn-icon search-clear-btn"
    aria-label="Suche zurücksetzen"
>{% call icons::icon_x() %}</a>
```

Mit JS: `click`-Listener verhindert Navigation, leert Feld, triggert HTMX.
Ohne JS: klickt auf `/`, was die vollständige Rezeptliste zeigt. Korrekte Funktion ohne JS!

Dieser Ansatz (`<a href="/">`) ist der sauberste und wird implementiert.

---

## Abhängigkeiten

- Story 07 (Volltextsuche) muss abgeschlossen sein — ist es.
- Story 26 (Icons) muss abgeschlossen sein — `icon_x()`-Makro ist in `templates/components/icons.html` vorhanden.
- Bestehende HTMX-Infrastruktur (`hx-get`, `hx-target`, `hx-push-url` auf dem Suchfeld) wird direkt genutzt.
- `htmx.trigger()` ist eine öffentliche HTMX-API — kein zusätzliches JS-Framework nötig.

---

## Betroffene Dateien

| Datei | Art der Änderung |
|-------|-----------------|
| `templates/index.html` | Clear-Button + Script-Block hinzufügen |
| `src/static/css/app.css` | Natives Clear-Icon ausblenden, `.search-clear-btn` und `.search-field-wrapper` hinzufügen |
| `tests/e2e/recipe-search.spec.ts` | 5 neue Testfälle anhängen |

Keine Änderungen an Rust-Code (keine neuen Endpunkte, keine neuen Structs).

---

## Test-Checkliste

- [ ] E2E-Test: K1 — Klick auf Clear-Icon leert Feld und zeigt alle Rezepte
- [ ] E2E-Test: K2 — Clear-Icon nur bei gefülltem Suchfeld sichtbar
- [ ] E2E-Test: K3 — Clear-Icon nach Suche ohne Treffer
- [ ] E2E-Test: K4 — Tastatur-Navigation (Tab + Enter) zum Clear-Icon
- [ ] E2E-Test: K5 — Clear-Icon sichtbar bei DeepLink mit ?q=...
- [ ] Manueller Test: Kein JS — Klick auf Clear-Icon navigiert zu `/`, vollständige Liste erscheint
- [ ] Manueller Test: Responsivität auf Mobile (Button passt ins Layout)
- [ ] Manueller Test: Fokus-Indikator auf Clear-Icon sichtbar (Tastaturnavigation)

---

## Offene Punkte

- **Fokus nach Klick:** Der JS-Handler setzt `input.focus()` nach dem Leeren — damit kann direkt
  neu getippt werden. Das ist dem neutralen Fokus vorzuziehen (Story-Frage K4 beantwortet).
- **No-JS-Fallback:** Mit `<a href="/">` statt `<button type="submit">` ist der Fallback korrekt
  und ohne Einschränkungen. Kein zusätzlicher Endpunkt nötig.
