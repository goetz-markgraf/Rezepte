# Implementierungsplan: Story 39

## Ziel
Die Buttons "Heute" und "Dubletten prüfen" aus der Top-Bar in ein Hamburger-Menü verschieben.

## Analyse

### Aktueller Zustand (base.html)
```html
<nav class="main-nav" aria-label="Hauptnavigation">
    {% block header_actions %}
    <a href="/recipes/new" class="nav-link">Neues Rezept</a>
    {% endblock %}
    <a href="/heute" class="nav-link">Heute</a>
    <a href="/wochenvorschau" class="nav-link">Wochenvorschau</a>
    <a href="/recipes/duplicates" class="nav-link">Dubletten prüfen</a>
</nav>
```

### Gewünschter Zustand
- "Heute" und "Dubletten prüfen" sind nicht mehr direkt in der Navigation sichtbar
- Stattdessen: Hamburger-Menü-Button (☰) rechts in der Top-Bar
- Beim Klick öffnet sich ein Dropdown mit den beiden Links

## Implementierungsschritte

### [ ] Schritt 1: Icons erweitern
**Datei:** `templates/components/icons.html`
- Hamburger-Menü Icon (☰) hinzufügen
- Optional: X-Icon für geschlossenen Zustand (falls gewünscht)

### [ ] Schritt 2: CSS für Hamburger-Menü
**Datei:** `src/static/css/app.css`
- Styles für Hamburger-Button (Positionierung rechts)
- Dropdown-Menü-Styling
- Mobile-optimierte Darstellung
- Animation für Öffnen/Schließen
- Barrierefreiheit (Focus-Styles)

**Neue CSS-Klassen:**
- `.hamburger-btn` - Der Menü-Button
- `.hamburger-menu` - Das Dropdown-Menü
- `.hamburger-menu.open` - Geöffneter Zustand
- `.hamburger-menu-item` - Einzelne Menüpunkte

### [ ] Schritt 3: Template base.html anpassen
**Datei:** `templates/base.html`
- Navigationsstruktur anpassen:
  - "Neues Rezept" bleibt als Button (links)
  - "Wochenvorschau" bleibt als Link (mittig)
  - Hamburger-Menü rechts mit "Heute" und "Dubletten prüfen"

**Struktur:**
```html
<header>
    <a href="/" class="site-title">Rezepte</a>
    <nav class="main-nav" aria-label="Hauptnavigation">
        {% block header_actions %}
        <a href="/recipes/new" class="btn-header-new-recipe">
            {% call icons::icon_plus() %}<span class="btn-text">Neues Rezept</span>
        </a>
        {% endblock %}
        <a href="/wochenvorschau" class="nav-link">Wochenvorschau</a>
        
        <!-- Hamburger Menü -->
        <div class="hamburger-menu-container">
            <button class="hamburger-btn" aria-label="Menü öffnen" aria-expanded="false" aria-controls="hamburger-dropdown">
                {% call icons::icon_menu() %}
            </button>
            <div id="hamburger-dropdown" class="hamburger-menu" role="menu" aria-hidden="true">
                <a href="/heute" class="hamburger-menu-item" role="menuitem">Heute</a>
                <a href="/recipes/duplicates" class="hamburger-menu-item" role="menuitem">Dubletten prüfen</a>
            </div>
        </div>
    </nav>
</header>
```

### [ ] Schritt 4: JavaScript für Interaktivität
**Datei:** `templates/base.html` (Inline-Script am Ende)
- Event-Handler für Hamburger-Button
- Toggle für aria-expanded und aria-hidden
- Schließen bei Klick außerhalb
- Schließen bei Escape-Taste
- Optional: Fokus-Management

**Funktionalität:**
```javascript
// Toggle Menü öffnen/schließen
// Klick außerhalb schließt Menü
// Escape-Taste schließt Menü
```

### [ ] Schritt 5: Barrierefreiheit prüfen
- aria-label für Hamburger-Button
- aria-expanded für Zustand
- aria-controls für Dropdown-Bezug
- role="menu" und role="menuitem"
- Keyboard-Navigation funktioniert

## Teststrategie

### Manuelle Tests
1. Hamburger-Menü öffnet sich bei Klick
2. Links "Heute" und "Dubletten prüfen" sind im Menü sichtbar
3. Navigation zu beiden Seiten funktioniert
4. Menü schließt sich nach Klick auf Link
5. Menü schließt sich bei Klick außerhalb
6. Menü schließt sich bei Escape-Taste
7. Mobile: Touch-Targets sind ausreichend groß

### E2E-Tests (Playwright)
- Test: Hamburger-Menü öffnen und schließen
- Test: Navigation über Hamburger-Menü
- Test: Tastatur-Navigation

## Definition of Done
- [ ] Hamburger-Menü ist in der Top-Bar sichtbar
- [ ] "Heute" und "Dubletten prüfen" sind im Menü
- [ ] Beide Links funktionieren korrekt
- [ ] Menü lässt sich öffnen und schließen
- [ ] Mobile-Ansicht ist nutzbar
- [ ] WCAG 2.1 Level A konform
- [ ] E2E-Tests bestehen

## Anpassungen
Keine Backend-Änderungen notwendig - reine Frontend/UI-Änderung.
