# Implementierungsplan: Story 35 - Suche "Länger nicht gemacht" mit vorselektiertem Mittagessen-Filter

## Übersicht

Story 35 ist eine minimale Erweiterung von Story 34: Der bestehende Link in der Wochenvorschau-Toolbar (`/?filter=laenger-nicht-gemacht`) wird um den Kategorie-Parameter `kategorie=Mittagessen` ergänzt. Dadurch öffnet die Suche direkt mit vorselektiertem Mittagessen-Filter.

**Einzige Code-Änderung:** Eine URL in `templates/wochenvorschau.html` erhält einen zusätzlichen Query-Parameter.

---

## Technische Schritte

### Schritt 1: Template anpassen (TDD — Test zuerst)

**Datei:** `tests/e2e/wochenvorschau-not-made.spec.ts`

- [ ] Bestehenden Test-Describe-Block erweitern oder neue `test.describe`-Gruppe für Story 35 ergänzen
- [ ] Test T1: Klick aus Wochenübersicht öffnet URL mit `filter=laenger-nicht-gemacht` **und** `kategorie=Mittagessen`
- [ ] Test T2: Kategorie-Filter "Mittagessen" ist auf der Zielseite als aktiv markiert (`aria-pressed="true"`)
- [ ] Test T3: Nur Mittagessen-Rezepte werden angezeigt (mit passendem Testrezept)
- [ ] Test T4: DeepLink `/?filter=laenger-nicht-gemacht&kategorie=Mittagessen` funktioniert direkt
- [ ] Test T5: Mittagessen-Filter kann manuell abgewählt werden

Tests laufen rot → weiter mit Schritt 2.

### Schritt 2: Template-Änderung

**Datei:** `templates/wochenvorschau.html`

- [ ] URL des bestehenden Buttons von `/?filter=laenger-nicht-gemacht` auf `/?filter=laenger-nicht-gemacht&kategorie=Mittagessen` ändern
- [ ] ARIA-Label anpassen: `"Mittagessen-Rezepte anzeigen, die länger nicht gemacht wurden"`

**Konkrete Änderung (Zeile 31):**
```html
<!-- Vorher -->
<a href="/?filter=laenger-nicht-gemacht" 
   class="btn-secondary not-made-button" 
   aria-label="Rezepte anzeigen, die länger nicht gemacht wurden">

<!-- Nachher -->
<a href="/?filter=laenger-nicht-gemacht&kategorie=Mittagessen" 
   class="btn-secondary not-made-button" 
   aria-label="Mittagessen-Rezepte anzeigen, die länger nicht gemacht wurden">
```

### Schritt 3: Rust-Integrationstest aktualisieren

**Datei:** `tests/wochenvorschau.rs` (oder entsprechende bestehende Testdatei)

- [ ] Bestehenden Test `wochenvorschau_enthaltet_link_zur_not_made_suche` prüfen
- [ ] Test aktualisieren, sodass Link auf `/?filter=laenger-nicht-gemacht&kategorie=Mittagessen` geprüft wird
- [ ] `cargo test` ausführen — Tests grün

### Schritt 4: E2E-Tests grün machen

- [ ] `npm run test:e2e` ausführen
- [ ] Alle neuen Tests und bestehende Story-34-Tests müssen grün sein

### Schritt 5: Code-Qualität

- [ ] `cargo build` — keine Fehler
- [ ] `cargo clippy -- -D warnings` — keine Warnungen
- [ ] `cargo fmt --check` — korrekt formatiert

---

## URL-Struktur

```
# Bestehend (Story 34)
GET /?filter=laenger-nicht-gemacht  →  Suche mit "Länger nicht gemacht" Filter (alle Kategorien)

# Neu (Story 35) — Link-Ziel in wochenvorschau.html
GET /?filter=laenger-nicht-gemacht&kategorie=Mittagessen  →  Suche mit beiden Filtern vorselektiert
```

---

## Abhängigkeiten

- Story 34 muss implementiert sein (Link in Wochenvorschau — ist bereits vorhanden)
- Story 9 (Filter "Länger nicht gemacht") — bereits implementiert
- Story 8 / Story 12 (Kategorie-Filter, kombinierte Filter) — bereits implementiert
- Keine Datenbank-Änderungen erforderlich
- Keine neuen Backend-Routen oder Handler erforderlich

---

## Zu ändernde Dateien

| Datei | Art der Änderung |
|-------|-----------------|
| `templates/wochenvorschau.html` | URL-Parameter ergänzen, ARIA-Label anpassen |
| `tests/e2e/wochenvorschau-not-made.spec.ts` | Neue Tests für Story 35 ergänzen |
| `tests/wochenvorschau.rs` (o.ä.) | Bestehenden Link-Test auf neue URL aktualisieren |

---

## Test-Checkliste

### Integrationstests (Rust)
- [ ] `wochenvorschau_enthaltet_link_zur_not_made_suche` — prüft neue URL mit `kategorie=Mittagessen`

### E2E-Tests (Playwright)
- [ ] **T1:** Klick auf Button navigiert zu URL mit `filter=laenger-nicht-gemacht&kategorie=Mittagessen`
- [ ] **T2:** Kategorie-Filter "Mittagessen" ist auf Zielseite als aktiv markiert (`aria-pressed="true"`)
- [ ] **T3:** Nur Mittagessen-Rezepte werden angezeigt (Brot-Rezepte nicht sichtbar)
- [ ] **T4:** DeepLink mit beiden Parametern zeigt korrekte Ansicht
- [ ] **T5:** Mittagessen-Filter kann manuell abgewählt werden — alle Kategorien erscheinen
- [ ] **T6:** Keine passenden Mittagessen-Rezepte — Hinweistext erscheint (nicht leere Seite)
- [ ] **T7:** Bestehende Story-34-Tests weiterhin grün

### Manueller Test
- [ ] Wochenübersicht aufrufen, Button klicken — Mittagessen-Filter ist vorselektiert
- [ ] Mittagessen-Filter manuell abwählen — alle Rezepte erscheinen
- [ ] Direkter URL-Aufruf `/?filter=laenger-nicht-gemacht&kategorie=Mittagessen` funktioniert
- [ ] Browser-Back führt zurück zur Wochenübersicht

---

## Offene Punkte

Keine — die Änderung ist vollständig durch den Kontext definiert. Der `kategorie`-Parameter ist bereits durch Story 8/12 im Backend unterstützt; es ist eine reine Template-Änderung.
