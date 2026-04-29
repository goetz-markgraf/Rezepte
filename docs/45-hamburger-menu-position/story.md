# Story 45: Hamburger-Menü öffnet nicht an der richtigen Position

**Epic:** [Epic 2: Rezept-Übersicht & Navigation](../stories_epics.md#epic-2-rezept-uebersicht--navigation)
**Status:** Offen

---

## 1. Story-Satz

Als **mobiler Nutzer** möchte ich, **dass das Hamburger-Menü an der richtigen Seiten öffnet (nahe dem Icon)**, damit ich **alle Menüpunkte sehen und anklicken kann**.

---

## 2. Weiterer Kontext

Das Hamburger-Menü (Navigation Drawer) öffnet sich aktuell immer rechtsbündig am rechten Bildschirmrand. Dies ist problematisch, wenn das Hamburger-Icon links in der Leiste steht (was der übliche Fall ist). Dadurch sind die Menüpunkte nicht vollständig sichtbar oder nur schwer erreichbar.

Kontakt zu Story 39: Das Hamburger-Menü wurde in Story 39 eingeführt (`39-hamburger-menu-fuer-top-bar-buttons`).

---

## 3. Akzeptanzkriterien

### Funktionale Kriterien

- **Angenommen** das Hamburger-Icon befindet sich links in der Top-Bar
  **Wenn** der Nutzer auf das Hamburger-Icon tippt
  **Dann** öffnet sich das Menü linksbündig am Bildschirmrand direkt neben dem Icon

- **Angenommen** das Hamburger-Icon befindet sich rechts in der Top-Bar
  **Wenn** der Nutzer auf das Hamburger-Icon tippt
  **Dann** öffnet sich das Menü rechtsbündig am Bildschirmrand direkt neben dem Icon

- **Angenommen** das Menü öffnet sich auf dem Bildschirm
  **Wenn** der Nutzer außerhalb des Menüs tippt
  **Dann** schließt sich das Menü

### Nicht-funktionale Kriterien

- Das Menü muss vollständig auf dem Bildschirm sichtbar sein (kein horizontaler Scroll)
- Das Menü muss eine halbtransparente Hintergrundabdeckung haben (Overlay)
- Die Menübreite ist nicht Teil dieser Story (kann weiterhin festgelegt werden)

---

## Zusatzinformationen

The hamburger menu always opens right aligned, even when the hamburger menu icon is on the left of the screen. This way, the items are not entirely visible
