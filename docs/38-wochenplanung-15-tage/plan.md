# Implementierungsplan: Story 38 - Wochenplanung auf 15-Tage-Liste umbauen

## Technische Schritte

### Schritt 1: Route und Handler anpassen
- [ ] `src/routes/wochenvorschau.rs`: Query-Parameter `week` und Navigation entfernen
- [ ] `src/routes/wochenvorschau.rs`: 15-Tage-Logik statt 7-Tage-Wochenlogik implementieren
  - Heute als Startdatum berechnen (`time::OffsetDateTime::now_utc().date()`)
  - +14 Tage für Enddatum (insgesamt 15 Tage)
- [ ] `src/routes/wochenvorschau.rs`: Datumsformatierung anpassen (kurze Form: "Fr, 04.04.2026")
- [ ] `src/routes/wochenvorschau.rs`: Navigation-URLs (`prev_week_url`, `next_week_url`, `is_current_week`) aus Template-Daten entfernen
- [ ] `src/routes/wochenvorschau.rs`: Unit-Tests für 15-Tage-Berechnung und Datumsformatierung

### Schritt 2: Template-Struktur anpassen
- [ ] `src/templates.rs`: `WochenvorschauTemplate` anpassen
  - `kw_anzeige` behalten aber umbenennen in `zeitraum_anzeige` (oder neue Beschreibung)
  - `prev_week_url`, `next_week_url`, `is_current_week` entfernen
  - Kommentare aktualisieren
- [ ] `src/templates.rs`: `Wochentag` Struct anpassen
  - Neue Feld: `wochentag_kurz` (z.B. "Mo", "Di", "Fr")

### Schritt 3: HTML-Template anpassen
- [ ] `templates/wochenvorschau.html`: Navigation mit Pfeilen entfernen (``wochenvorschau-nav``)
- [ ] `templates/wochenvorschau.html`: Überschrift anpassen (keine KW-Anzeige, stattdessen Zeitraum)
- [ ] `templates/wochenvorschau.html`: 15 Tage als vertikale Liste darstellen (statt horizontaler Wochenansicht)
- [ ] `templates/wochenvorschau.html`: Datumsformat anpassen ("Fr, 04.04.2026" statt "Freitag" + "4. April")
- [ ] `templates/wochenvorschau.html`: CSS-Klassen ggf. anpassen für kompakte Listenansicht
- [ ] `templates/wochenvorschau.html`: "Heute"-Badge beibehalten für aktuellen Tag

### Schritt 4: Datenbank-Layer (keine Änderungen)
- [ ] `get_recipes_current_week` wird weiterverwendet, aber mit flexiblem Datumsbereich
- [ ] Keine neuen DB-Methoden nötig (bereits vorhanden: `get_recipes_by_date_range`)

### Schritt 5: CSS/Styling anpassen
- [ ] `src/static/css/app.css`: Neue Styles für 15-Tage-Liste
  - Kompakte Darstellung pro Tag
  - Klare visuelle Trennung zwischen Tagen
  - "Heute"-Hervorhebung beibehalten
- [ ] Prüfung: Responsive Darstellung (Mobile/Desktop)

### Schritt 6: Integrationstests anpassen
- [ ] `tests/wochenvorschau.rs`: Tests aktualisieren für 15-Tage-Logik
  - Test: Zeigt 15 Tage ab heute
  - Test: Erster Tag ist heute
  - Test: Navigation ist nicht mehr vorhanden
  - Test: Keine KW-Anzeige mehr
  - Tests für alte Wochenlogik entfernen/aktualisieren
- [ ] Unit-Tests in `src/routes/wochenvorschau.rs` aktualisieren

### Schritt 7: E2E-Tests erstellen
- [ ] `tests/e2e/wochenvorschau-15-tage.spec.ts` erstellen:
  - Test 1: 15-Tage-Liste wird korrekt angezeigt (ab heute)
  - Test 2: Geplante Rezepte werden unter korrektem Datum angezeigt
  - Test 3: Navigation (Vorherige/Nächste Woche) ist nicht vorhanden
  - Test 4: Tage ohne Rezepte werden trotzdem angezeigt
  - Test 5: Klick auf Rezept führt zur Detailansicht
  - Test 6: Heute-Badge ist sichtbar
- [ ] Bestehende E2E-Tests `tests/e2e/wochenvorschau.spec.ts` aktualisieren
  - Navigation-Tests entfernen
  - Tests an 15-Tage-Ansicht anpassen

### Schritt 8: Barrierefreiheit prüfen
- [ ] Semantische HTML-Struktur für die Liste (``<ul>/<li>`` statt ``<dl>`` falls sinnvoll)
- [ ] ARIA-Labels für Datumsangaben
- [ ] Tastatur-Navigation testen
- [ ] Kontrast prüfen

### Schritt 9: DoD-Check
- [ ] `cargo build` ohne Fehler
- [ ] `cargo clippy -- -D warnings` ohne Warnungen
- [ ] `cargo fmt --check` bestehen
- [ ] `cargo test` alle Tests bestehen
- [ ] `npm run test:e2e` alle E2E-Tests bestehen

---

## URL-Struktur

```
GET  /wochenvorschau  →  15-Tage-Liste (immer ab heute, keine Parameter mehr)
```

**Hinweis:** Die bisherige Navigation mit `?week=YYYY-WNN` entfällt komplett.

---

## Abhängigkeiten

- Story 18 (Wochenvorschau) - bereits implementiert
- Story 28 (Datum-Eingabe) - bereits implementiert
- Keine neuen technischen Abhängigkeiten

---

## Test-Checkliste

### Unit-Tests
- [ ] Datum-Berechnung: 15 Tage ab heute korrekt
- [ ] Datumsformatierung: "Fr, 04.04.2026" Format
- [ ] Rezepte werden korrekt dem Tag zugeordnet

### Integrationstests (Rust)
- [ ] GET /wochenvorschau gibt 200 zurück
- [ ] 15 Tage werden angezeigt (nicht mehr 7)
- [ ] Navigationselemente fehlen
- [ ] Keine KW-Anzeige mehr
- [ ] Rezepte werden unter korrektem Datum angezeigt

### E2E-Tests (Playwright)
- [ ] 15-Tage-Liste lädt korrekt
- [ ] Heute ist das erste Element
- [ ] Geplante Rezepte sichtbar
- [ ] Navigation fehlt
- [ ] Rezept-Links funktionieren
- [ ] Mobile Ansicht funktioniert

### Manueller Test
- [ ] Seite lädt < 1 Sekunde
- [ ] Scrollen ist flüssig
- [ ] Darstellung ist übersichtlich

---

## Offene Punkte

- [ ] Soll die URL weiterhin `/wochenvorschau` heißen oder umbenannt werden (z.B. `/planung`)? → Entscheidung: Beibehalten für Rückwärtskompatibilität
- [ ] Soll es eine "Zurück zur aktuellen Woche"-Funktion geben? → Nein, da immer ab heute angezeigt wird
