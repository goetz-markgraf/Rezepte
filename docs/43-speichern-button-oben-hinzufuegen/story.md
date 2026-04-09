# Story 43: Speichern-Button in Bearbeitungsansicht oben hinzufügen

**Epic:** Rezept-Verwaltung (Grundlegendes CRUD)
**Priorität:** Nice-to-have
**Status:** Offen

---

## 1. Story-Satz

Als **Benutzer** möchte ich **in der Rezept-Bearbeitungsansicht zusätzlich einen Speichern-Button am oberen Rand des Formulars haben**, damit ich **Änderungen schnell sichern kann, ohne zum Ende der Seite scrollen zu müssen**.

---

## 2. Geschäftsbezogene Details

### Kontext
Bei längeren Rezeptbeschreibungen oder Zutatenlisten ist der Weg zum Speichern-Button am Ende der Seite weit. Ein redundanter Button am Anfang verbessert den Workflow.

### Nutzergruppe
Alle Benutzer, die Rezepte bearbeiten.

### Business-Value
Verbesserte Usability und Effizienz bei der Rezeptpflege.

### Edge Cases
- **Validierungsfehler:** Falls das Formular ungültig ist, wird der Speichervorgang wie gewohnt abgebrochen und die entsprechenden Fehlermeldungen angezeigt.
- **Mobile Ansicht:** Der Button muss auch in der mobilen Ansicht gut positioniert sein und die Bedienbarkeit nicht einschränken.

---

## 3. Akzeptanzkriterien

### Funktionale Kriterien

- [ ] **K1: Platzierung des Buttons**
  - In der Bearbeitungsansicht befindet sich ein Speichern-Button in derselben Zeile wie die Überschrift "Rezept bearbeiten".
- [ ] **K2: Optische Gestaltung**
  - Der Button ist klein.
  - Der Button ist blau.
  - Der Button verwendet das bestehende Speichern-Icon.
- [ ] **K3: Funktionalität**
  - Ein Klick auf den oberen Button löst den identischen Speichervorgang aus wie der primäre Speichern-Button am Ende des Formulars.
- [ ] **K4: Erhalt bestehender Elemente**
  - Die ursprünglichen Buttons "Speichern" und "Abbrechen" am Ende der Seite bleiben unverändert an ihrer Position erhalten.

### Nicht-funktionale Kriterien

- [ ] **K5: Barrierefreiheit**
  - Der Button ist vollständig per Tastatur bedienbar.
  - Der Button besitzt ein aussagekräftiges Label (z.B. `aria-label="Rezept speichern"`).

---

## 4. Technische Planung

### Datenmodell
Keine Änderungen am Datenmodell erforderlich.

### UI/UX-Spezifikation
- Ergänzung im HTML-Template der Bearbeitungsansicht.
- Integration in die Kopfzeile mittels Flexbox/Grid, um die Überschrift und den Button auf einer Linie zu halten.
- Verwendung existierender CSS-Klassen für das Styling (Blau, Icon).

---

## 5. Nicht-funktionale Anforderungen

### Performance
- Die Hinzufügung eines weiteren Buttons hat keinen messbaren Einfluss auf die Ladezeit.

### Browser-Support
- Aktuelle Chrome, Firefox, Safari, Edge Versionen
- iOS Safari und Android Chrome (letzte 2 Versionen)

### Barrierefreiheit
- WCAG 2.1 Level A konform
- Fokus-Indikatoren sichtbar

---

## 6. Teststrategie

### E2E-Tests (Playwright)

**Testfall 1: Speichern über oberen Button**
```gherkin
Given ich befinde mich in der Bearbeitungsansicht eines Rezepts
When ich ein Feld ändere und auf den oberen Speichern-Button klicke
Then wird das Rezept gespeichert
And ich werde zur Detailansicht zurückgeleitet
```

**Testfall 2: Sichtbarkeit beider Buttons**
```gherkin
Given ich befinde mich in der Bearbeitungsansicht eines Rezepts
Then sehe ich einen Speichern-Button neben der Überschrift "Rezept bearbeiten"
And sehe ich am Ende des Formulars weiterhin die Buttons "Speichern" und "Abbrechen"
```

---

## 7. Abhängigkeiten & Rahmenbedingungen

### Abhängigkeiten
- Story 02 (Rezept bearbeiten) muss implementiert sein.

### Rahmenbedingungen
- SQLite-Datenbank muss existieren und erreichbar sein
- Keine Authentifizierung erforderlich (LAN-only)

---

## Offene Punkte / Fragen
- Keine

---

**Letzte Aktualisierung:** 2026-04-09
