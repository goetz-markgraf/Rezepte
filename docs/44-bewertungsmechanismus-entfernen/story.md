# Story 44: Bewertungsmechanismus entfernen

**Epic:** Epic 4: Bewertung & Datums-Tracking
**Status:** Offen

---

## 1. Story-Satz

Als Benutzer möchte ich den gesamten Bewertungsmechanismus (5-Sterne-Rating) aus der Anwendung entfernt sehen, damit die Oberfläche schlanker ist und keine ungenutzte Funktion mehr stört.

---

## 2. Weiterer Kontext

Nach längerer Nutzung der App hat sich herausgestellt, dass die 5-Sterne-Bewertung nicht praktisch genutzt wird. Das Paar entscheidet intuitiv bei der Wochenplanung, was gemacht wird – die formale Bewertung war ein initialer Gedanke, der sich in der Praxis nicht bewährt hat.

Die Sterne tauchen heute an vielen Stellen auf:
- In der Rezept-Listenansicht (neben jedem Titel)
- In der Detailansicht (Inline-Rating-Widget)
- Im Bearbeitungsformular (Sterne-Radio-Buttons)
- In der "Heute gekocht"-Ansicht
- Als Filter auf der Startseite ("Nur Gute", "Favoriten")
- In der Dubletten-Prüfung/Merge-Ansicht

Da das System bereits produktiv mit echten Daten läuft, darf das Löschen der Bewertungsspalte nicht zu Datenverlust führen. Die bestehenden Rating-Werte sollen in der Datenbank erhalten bleiben (z. B. durch Migration in eine separate Historientabelle oder durch Beibehaltung der Spalte ohne UI-Nutzung). Die bevorzugte Lösung ist die vollständige Entfernung aus dem UI, während die Spalte in der DB vorerst beibehalten wird (keine Migration nötig, nur UI-Entfernung).

---

## 3. Akzeptanzkriterien

### Funktionale Kriterien

- [ ] **K1:** Die Startseite zeigt keine Bewertungsfilter-Buttons ("Nur Gute ★★★+", "Favoriten ★★★★★") mehr an.
- [ ] **K2:** Die Rezept-Listenansicht zeigt keine Sterne mehr neben den Rezepttiteln an.
- [ ] **K3:** Die Rezept-Detailansicht zeigt kein Inline-Rating-Widget mehr an (weder Sterne noch "Noch keine Bewertung"-Text).
- [ ] **K4:** Das Bearbeitungsformular für Rezepte enthält kein Bewertungsfeld (Sterne-Radio-Buttons) mehr.
- [ ] **K5:** Das Erstellen eines neuen Rezepts funktioniert ohne Angabe einer Bewertung (kein rating-Parameter im Formular).
- [ ] **K6:** Die "Heute gekocht"-Ansicht zeigt keine Sterne mehr an.
- [ ] **K7:** Die Dubletten-Prüfung und Merge-Ansicht zeigen keine Bewertungen mehr an und bieten keine Option, Bewertungen zu übernehmen.
- [ ] **K8:** Der Bewertungsfilter-Parameter (`?bewertung=...`) wird ignoriert, falls jemand einen alten DeepLink aufruft.
- [ ] **K9:** POST-Endpunkte für Rating-Updates (`/recipes/:id/rating`, `/heute/recipes/:id/rating`) existieren nicht mehr oder geben 404 zurück.
- [ ] **K10:** Bestehende Daten in der Datenbank (rating-Spalte) gehen nicht verloren.

### Nicht-funktionale Kriterien

- [ ] **K11:** Keine JavaScript-/HTMX-Fehler nach Entfernung der Rating-Komponenten.
- [ ] **K12:** Alle AXE-Level-A-Tests bleiben grün (keine regressiven Accessibility-Probleme durch entfernte Elemente).
- [ ] **K13:** URLs, die früher Bewertungsfilter enthalten haben, führen nicht zu Fehlern (Graceful Degradation).
