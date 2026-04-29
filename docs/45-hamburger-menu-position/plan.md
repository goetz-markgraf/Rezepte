# Plan: Hamburger-Menü öffnet an der richtigen Position

**Story:** 45 — Hamburger-Menü öffnet nicht an der richtigen Position  
**Status:** Offen → In Arbeit

---

## 1. Problem-Analyse

Das Hamburger-Menü (`hamburger-menu`) hat folgende CSS-Regel:

```css
.hamburger-menu {
    position: absolute;
    top: calc(100% + 0.5rem);
    right: 0;  /* ← BUG: immer rechtsbündig */
    min-width: 180px;
    ...
}
```

Das Menü wird immer am **rechten Rand** des `.hamburger-menu-container` ausgerichtet, unabhängig davon, wo sich das Hamburger-Icon horizontal befindet.

### Layout-Kontext

Das Hamburger-Icon steht in `<nav class="main-nav">`, welches `display: flex` hat. Auf mobilen Bildschirmen (< 768px) ist der Header:

```css
header {
    display: flex;
    align-items: center;
    justify-content: space-between;
    gap: 0.5rem;
    flex-wrap: wrap;
}
```

Da `.hamburger-menu-container` am Ende von `.main-nav` steht, ist es auf mobilen Bildschirmen **rechts** im Header. Aber das Menü mit `right: 0` erscheint weit außerhalb des sichtbaren Bereichs, da der Container relativ zum Icon positioniert ist und `right: 0` nach rechts aligniert.

**Warte** — der Container ist `position: relative`. Das Menü ist `position: absolute` mit `right: 0`. Das bedeutet, das Menü wird am **rechten Rand des Buttons** ausgerichtet und ragt nach **rechts**.

Das Problem ist: Auf mobilen Geräten mit kleinem Bildschirm (z.B. 375px width) und einem 180px breiten Menü:
- Wenn das Hamburger-Icon links im Header steht, ragt das Menü nach rechts — das ist eigentlich in Ordnung
- **Aber**: Das Menü sollte am Icon **nach links** ausgerichtet sein, wenn es am rechten Bildschirmrand steht

Tatsächlich ist die Hauptkomponente: Das Menü öffnet sich immer mit `right: 0`, was bedeutet, es wird am rechten Rand des `.hamburger-menu-container` befestigt und dehnt sich nach links aus. Das Problem tritt auf, wenn sich das Hamburger-Icon auf der **linken** Seite befindet. Dann ist der Container links im Header, und `right: 0` bedeutet, das Menü wird am **rechten Rand des Containers** ausgerichtet — also weiter rechts vom Icon entfernt.

Die Lösung: **Das Menü sollte linksbündig am Container ausgerichtet werden (`left: 0`).**

---

## 2. Implementierung

### Schritt 1: CSS-Änderung für linksbündiges Menü

**Datei:** `src/static/css/app.css`  
**Änderung:** In der `.hamburger-menu` Regel `right: 0` durch `left: 0` ersetzen.

```diff
 .hamburger-menu {
     position: absolute;
     top: calc(100% + 0.5rem);
-    right: 0;
+    left: 0;
     min-width: 180px;
     ...
 }
```

**Begründung:** `left: 0` sorgt dafür, dass das Menü linksbündig am Hamburger-Container beginnt und nach rechts aufspannt. Das funktioniert auf allen Bildschirmgrößen und ist konsistent mit dem üblichen Verhalten von Dropdown-Menüs.

### Schritt 2: E2E-Test für Menü-Position

**Datei:** `tests/e2e/hamburger-menu.spec.ts` (neu)

Tests:
1. Menü öffnet beim Klick auf das Hamburger-Icon
2. Menü ist linksbündig am Hamburger-Icon ausgerichtet
3. Menü schließt bei Klick darauf
4. Menü schließt bei Klick außerhalb
5. Menü schließt bei Escape-Taste
6. Menü ist auf mobilen Bildschirmgrßen vollständig sichtbar

---

## 3. Abhängigkeiten

- **Keine** — Story ist unabhängig von anderen Stories implementierbar
