# Plan: Hamburger-Menü öffnet an der richtigen Position

**Story:** 45 — Hamburger-Menü öffnet nicht an der richtige Position  
**Status:** Offen → In Arbeit

---

## 1. Problem-Analyse

Das Hamburger-Menü (`hamburger-menu`) hatte folgende CSS-Regel:

```css
.hamburger-menu {
    position: absolute;
    top: calc(100% + 0.5rem);
    right: 0;  /* ← BUG: immer rechtsbündig, egal wo das Icon steht */
    min-width: 180px;
}
```

Das Menü wurde immer am **rechten Rand** des `.hamburger-menu-container` ausgerichtet. Wenn sich das Hamburger-Icon auf der linken Seite befand, ragte das Menü nach links außerhalb des Viewports.

### Lösung

Dynamische Positionierung via JavaScript:
- **Icon in rechter Bildschirmhälfte** → Menü `right: 0` (rechtsbündig)
- **Icon in linker Bildschirmhälfte** → Menü `left: 0` (linksbündig)

---

## 2. Implementierung

### Schritt 1: JS-Fix für dynamische Menü-Position

**Datei:** `templates/base.html`

In der `openMenu()`-Funktion die Menü-Position dynamisch berechnen:

```javascript
function openMenu() {
    // Menü-Position dynamisch an Hamburger-Icon anpassen
    const btnRect = hamburgerBtn.getBoundingClientRect();
    const viewportWidth = window.innerWidth;
    
    // Wenn Icon in rechter Bildschirmhälfte → right: 0
    // Wenn Icon in linker Bildschirmhälfte → left: 0
    if (btnRect.right > viewportWidth / 2) {
        hamburgerMenu.style.left = 'auto';
        hamburgerMenu.style.right = '0';
    } else {
        hamburgerMenu.style.right = 'auto';
        hamburgerMenu.style.left = '0';
    }
    
    hamburgerMenu.classList.add('open');
    // ... rest bleibt gleich
}
```

### Schritt 2: CSS-Korrekturen

**Datei:** `src/static/css/app.css`

1. `.hamburger-menu` → `left: 0; right: auto;` als Default (JS überschreibt dies)
2. Mobile Media Query → `right: auto;` hinzufügen (kein statisches `right: 0` mehr)

### Schritt 3: E2E-Test für Menü-Position

**Datei:** `tests/e2e/hamburger-menu.spec.ts` (neu)

Tests:
1. Menü öffnet beim Klick auf das Hamburger-Icon (ARIA-Status korrekt)
2. Menü ist rechtsbündig zum Hamburger-Icon ausgerichtet (mobile)
3. Menü ist auf mobilen Bildschirmgrößen vollständig sichtbar (kein Überlaufen)
4. Menü schließt bei Klick außerhalb
5. Menü-Elemente sind klickbar

---

## 3. Abhängigkeiten

- **Keine** — Story ist unabhängig von anderen Stories implementierbar
