# Implementierungsplan: Story 33 - Wochenübersicht Navigation mit Pfeiltasten

## Zusammenfassung der Änderungen

Diese Story erweitert die bestehende Wochenvorschau-Seite (`/wochenvorschau`) um eine Navigation zwischen Wochen mittels `<` und `>` Pfeiltasten. Die aktuelle Woche wird als Standard angezeigt, aber über URL-Parameter (`?week=2025-W02`) kann direkt zu einer bestimmten Woche navigiert werden (DeepLink-fähig).

### Hauptänderungen:
- Handler erweitert: Akzeptiert optionalen `week` Query-Parameter (ISO-Format: `YYYY-WNN`)
- Navigation-Buttons: `<` (vorherige Woche) und `>` (nächste Woche) mit Links
- URL-Struktur: DeepLink-fähige URLs mit `?week=YYYY-WNN`
- Template erweitert: Navigation-Header mit Pfeil-Buttons
- CSS-Erweiterungen: Styling für Navigation-Buttons

---

## Technischer Ansatz

### ISO-Wochenformat Parsing
- Input: `2025-W02` (ISO 8601 Week Date Format)
- Parsing mit `time`-Crate oder manuelles Parsing
- Berechnung des Montags der angegebenen Woche
- Validierung: Ungültige Wochen (z.B. W53 in Jahren ohne 53 Wochen) → Fehler oder Umleitung auf aktuelle Woche

### Navigation-Logik
- Aktuelle Woche: Heute basierend auf Systemdatum
- Vorherige Woche: Aktuelle Woche - 7 Tage
- Nächste Woche: Aktuelle Woche + 7 Tage
- Navigation-URLs: `/wochenvorschau?week=YYYY-WNN`

### Progressive Enhancement
- Funktioniert ohne JavaScript (normale Links)
- HTMX optional für smooth Navigation (Out-of-band Swaps)
- Fallback: Standard-Link-Navigation

---

## Dateien, die geändert/erstellt werden müssen

### Geänderte Dateien:
1. `src/routes/wochenvorschau.rs` - Handler mit Query-Parameter
2. `templates/wochenvorschau.html` - Navigation-Buttons hinzufügen
3. `src/static/css/app.css` - CSS für Navigation-Buttons

### Neue Tests:
4. `tests/integration/wochenvorschau_test.rs` - Integrationstests für Navigation
5. `tests/e2e/wochenvorschau-navigation.spec.ts` - E2E-Tests für UI-Navigation

---

## Datenbank-Änderungen

**Keine Änderungen erforderlich.**

Die bestehenden Queries (`get_recipes_current_week`) arbeiten bereits mit Datumsbereichen und können wiederverwendet werden.

---

## API-Endpunkte/Routen

### Bestehend:
```
GET /wochenvorschau → Aktuelle Woche anzeigen
```

### Erweitert:
```
GET /wochenvorschau?week=2025-W02 → Spezifische Woche anzeigen
```

### URL-Struktur:
```
/wochenvorschau                    → Aktuelle Woche
/wochenvorschau?week=2025-W02      → KW 02 2025
/wochenvorschau?week=2025-W52      → KW 52 2025
```

---

## UI/UX-Änderungen

### Navigation-Header
```
<  KW 14 · 30. März – 5. April 2026  >
```

- Links: `<` Button - Link zur vorherigen Woche
- Mitte: Aktuelle KW-Anzeige (bestehend)
- Rechts: `>` Button - Link zur nächsten Woche

### Design-Anforderungen:
- Buttons als Links mit deutlichem Hover-Zustand
- Aktuelle Woche visuell hervorgehoben (optional: "Heute" Badge)
- Responsive: Buttons auch auf mobilen Geräten gut bedienbar
- Accessible: ARIA-Labels für Screenreader

### CSS-Klassen (neu):
- `.wochenvorschau-nav` - Container für Navigation
- `.wochen-nav-btn` - Button-Styling für < und >
- `.wochen-nav-prev` / `.wochen-nav-next` - Spezifische Modifier

---

## Test-Strategie

### Unit-Tests (Rust)
1. **ISO-Woche Parsing** - `parse_iso_week("2025-W02")` → korrektes Datum
2. **Datum zu ISO-Woche** - Umkehrfunktion testen
3. **Wochenberechnung** - Vorherige/Nächste Woche korrekt?
4. **Edge Cases** - Jahrwechsel, Schaltjahre, ungültige Wochen

### Integration-Tests (Rust)
1. **GET /wochenvorschau** - Ohne Parameter → aktuelle Woche
2. **GET /wochenvorschau?week=2025-W02** - Spezifische Woche
3. **GET /wochenvorschau?week=invalid** - Fehlerbehandlung
4. **Navigation-Links** - Vorherige/Nächste Woche URLs korrekt?

### E2E-Tests (Playwright)
1. **Navigation vorwärts** - Click auf `>` → nächste Woche angezeigt
2. **Navigation rückwärts** - Click auf `<` → vorherige Woche angezeigt
3. **DeepLink** - Direktaufruf `/wochenvorschau?week=2025-W02`
4. **Aktuelle Woche** - Ohne Parameter → aktuelle Woche angezeigt
5. **URL-Update** - Navigation aktualisiert URL korrekt

---

## Implementierungsschritte

### Schritt 1: Route-Handler erweitern
- [ ] `axum::extract::Query` Import hinzufügen
- [ ] Neue Struct `WeekQuery { week: Option<String> }` definieren
- [ ] Handler-Signatur erweitern: `Query(query): Query<WeekQuery>`
- [ ] Funktion `parse_iso_week(week_str: &str) -> Result<time::Date, AppError>` implementieren
- [ ] Funktion `format_iso_week(date: time::Date) -> String` implementieren (für Links)
- [ ] Logik für Wochenberechnung: Wenn `week` vorhanden → parse, sonst aktuelle Woche
- [ ] Vorherige/Nächste Woche berechnen für Navigation-Links
- [ ] Unit-Tests für Parsing-Funktionen

### Schritt 2: Template-Datenstruktur erweitern
- [ ] `WochenvorschauTemplate` erweitern um:
  - `prev_week_url: String`
  - `next_week_url: String`
  - `is_current_week: bool` (optional: Badge anzeigen)
- [ ] Template-Werte im Handler befüllen

### Schritt 3: Template anpassen
- [ ] Navigation-Container um Überschrift herum ergänzen
- [ ] `<` Link mit `href="{{ prev_week_url }}"` hinzufügen
- [ ] `>` Link mit `href="{{ next_week_url }}"` hinzufügen
- [ ] ARIA-Labels für Accessibility: `aria-label="Vorherige Woche"`
- [ ] Optional: "Heute"-Link zurück zur aktuellen Woche

### Schritt 4: CSS-Styling
- [ ] `.wochenvorschau-nav` - Flexbox für Layout
- [ ] `.wochen-nav-btn` - Button-Styling (Padding, Border, Hover)
- [ ] Responsive: Touch-friendly Größen (min 44px)
- [ ] Active-State für besseres Feedback

### Schritt 5: Integration-Tests
- [ ] `tests/integration/wochenvorschau_test.rs` erstellen
- [ ] Test: GET /wochenvorschau ohne Parameter
- [ ] Test: GET /wochenvorschau?week=2025-W02
- [ ] Test: Navigation-Links enthalten korrekte URLs
- [ ] Test: Ungültige Woche → Fehler oder Umleitung

### Schritt 6: E2E-Tests
- [ ] `tests/e2e/wochenvorschau-navigation.spec.ts` erstellen
- [ ] Test: Navigation zur nächsten Woche
- [ ] Test: Navigation zur vorherigen Woche
- [ ] Test: DeepLink mit URL-Parameter
- [ ] Test: Aktuelle Woche ohne Parameter
- [ ] Test: URL wird nach Navigation korrekt aktualisiert

### Schritt 7: Validierung & Fehlerbehandlung
- [ ] Ungültige Wochenformate abfangen (z.B. "2025-W99")
- [ ] Fehlermeldung oder Umleitung auf aktuelle Woche
- [ ] Logging für Debug-Zwecke

### Schritt 8: Manuelle Tests
- [ ] Navigation durch mehrere Wochen testen
- [ ] Jahrwechsel testen (KW 52 → KW 01)
- [ ] Mobile Ansicht prüfen
- [ ] Ohne JavaScript testen (Links funktionieren)

---

## Abhängigkeiten und Risiken

### Abhängigkeiten:
- **Keine externen Abhängigkeiten** - Nutzt bestehende `time`-Crate
- **Story 18-30** - Wochenvorschau existiert bereits (vorausgesetzt)

### Risiken:
| Risiko | Wahrscheinlichkeit | Auswirkung | Mitigation |
|--------|-------------------|------------|------------|
| ISO-Wochenberechnung komplex | Mittel | Hoch | Gut testen, insb. Jahrwechsel |
| Ungültige URL-Parameter | Niedrig | Mittel | Graceful Fallback auf aktuelle Woche |
| Mobile Darstellung | Niedrig | Mittel | Touch-friendly Größen, ausreichend Tests |

### Technische Entscheidungen:
- **ISO 8601 Format** - Standardkonform, international verständlich
- **Query-Parameter statt Path** - Einfacher zu parsen, konsistent mit anderen Filtern
- **Keine HTMX-Enhancement zwingend** - Progressive Enhancement, funktioniert ohne JS

---

## URL-Struktur

```
GET  /wochenvorschau               → Aktuelle Woche anzeigen
GET  /wochenvorschau?week=2025-W02 → KW 02 2025 anzeigen
GET  /wochenvorschau?week=2025-W52 → KW 52 2025 anzeigen
```

---

## Test-Checkliste

- [ ] Unit-Test: `parse_iso_week("2025-W02")` gibt korrekten Montag zurück
- [ ] Unit-Test: `format_iso_week(date)` gibt korrektes Format zurück
- [ ] Unit-Test: Vorherige/Nächste Woche-Berechnung bei Jahrwechsel
- [ ] Unit-Test: Ungültige Woche wird abgelehnt
- [ ] Integration-Test: GET /wochenvorschau ohne Parameter zeigt aktuelle Woche
- [ ] Integration-Test: GET /wochenvorschau?week=2025-W02 zeigt korrekte Woche
- [ ] Integration-Test: Navigation-Links enthalten korrekte URLs
- [ ] E2E-Test: Click auf `>` navigiert zur nächsten Woche
- [ ] E2E-Test: Click auf `<` navigiert zur vorherigen Woche
- [ ] E2E-Test: DeepLink /wochenvorschau?week=2025-W02 funktioniert
- [ ] Manueller Test: Ohne JavaScript funktioniert Navigation
- [ ] Manueller Test: Mobile Darstellung ist bedienbar

---

## Offene Punkte

- [ ] Soll es einen "Heute"-Button geben, um schnell zur aktuellen Woche zurückzukehren?
- [ ] Soll HTMX für smooth Navigation ohne Page-Reload verwendet werden?
- [ ] Wie weit in Vergangenheit/Zukunft soll Navigation möglich sein? (unbegrenzt?)
