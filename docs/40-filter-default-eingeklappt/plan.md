# Implementierungsplan: Story 40 - Filter standardmäßig eingeklappt

## Technische Zusammenfassung

Story 40 ändert das Default-Verhalten des Filter-Zustands beim ersten Seitenaufruf. Bisher werden Filter ohne `filter_collapsed` Parameter ausgeklappt angezeigt, zukünftig sollen sie eingeklappt sein.

**Kernänderung:** Die Logik zur Bestimmung des Zustands wird umgekehrt:
- Vorher: `filter_collapsed == Some("1")` → eingeklappt, sonst ausgeklappt
- Nachher: `filter_collapsed == Some("0")` → ausgeklappt, sonst eingeklappt

**Betroffene Dateien:**
- `src/routes/recipes.rs` - Handler-Logik und URL-Builder
- `templates/wochenvorschau.html` - Links mit `filter_collapsed` Parameter
- `tests/e2e/filter-collapse.spec.ts` - E2E-Tests anpassen
- `tests/e2e/wochenvorschau.spec.ts` - E2E-Tests anpassen

---

## Technische Schritte

### Schritt 1: Handler-Logik anpassen (src/routes/recipes.rs)

- [ ] Zeile 496: Logik für `filter_collapsed` umkehren
  - Von: `let filter_collapsed = query.filter_collapsed.as_deref() == Some("1");`
  - Nach: `let filter_collapsed = query.filter_collapsed.as_deref() != Some("0");`
- [ ] Kommentar anpassen (Zeile 495): "filter_collapsed: "0" → ausgeklappt, alles andere → eingeklappt"
- [ ] Unit-Test: `filter_collapsed_default_renders_collapsed` - Testet dass ohne Parameter eingeklappt wird
- [ ] Unit-Test: `filter_collapsed_zero_renders_expanded` - Testet dass `filter_collapsed=0` ausklappt

### Schritt 2: URL-Builder anpassen (src/routes/recipes.rs)

- [ ] `build_filter_collapsed_toggle_url` Funktion umkehren (Zeilen 93-136)
  - Doc-Kommentar anpassen: Eingeklappt → URL mit `filter_collapsed=0`, Ausgeklappt → URL ohne Parameter
  - Logik in Zeile 127-129 umkehren: `if is_collapsed { params.push("filter_collapsed=0".to_string()); }`
- [ ] Unit-Tests für `build_filter_collapsed_toggle_url` anpassen (Zeilen 1289-1379)
  - Test 1: Ausgeklappt (`false`) → URL ohne Parameter
  - Test 2: Eingeklappt (`true`) → URL mit `filter_collapsed=0`
  - Alle weiteren Tests prüfen und anpassen

### Schritt 3: Wochenvorschau-Links anpassen (templates/wochenvorschau.html)

- [ ] Zeile 14: `filter_collapsed=1` entfernen (oder zu `filter_collapsed=0` ändern)
- [ ] Zeile 57: `filter_collapsed=1` entfernen (oder zu `filter_collapsed=0` ändern)
- [ ] Hinweis: Da Default jetzt eingeklappt ist, könnte der Parameter komplett entfallen

### Schritt 4: E2E-Tests anpassen (tests/e2e/filter-collapse.spec.ts)

- [ ] Test "K2/K4" anpassen: Erwartung ändern - bei Start ohne Parameter sind Filter eingeklappt
- [ ] Test "K3/K4" anpassen: URL-Prüfung auf `filter_collapsed=0` statt Parameter-Entfernung
- [ ] Test "K5" anpassen: Teste `filter_collapsed=0` für ausgeklappten Zustand
- [ ] Test "K7" anpassen: Gespeicherte Filter öffnen jetzt eingeklappt (Default)
- [ ] Test "K8" anpassen: Prüfung auf `filter_collapsed=0` statt Parameter-Entfernung
- [ ] Test "K1" anpassen: Button zeigt bei Start "▶" statt "▼"
- [ ] Test "K10" (beide) anpassen: Links enthalten `filter_collapsed=0` statt `filter_collapsed=1`
- [ ] Neuen Test hinzufügen: "K1 Story 40: Filter standardmäßig eingeklappt"

### Schritt 5: E2E-Tests Wochenvorschau anpassen (tests/e2e/wochenvorschau.spec.ts)

- [ ] Zeile 144-145: `filter_collapsed=1` zu `filter_collapsed=0` ändern (oder entfernen)
- [ ] Zeile 152: URL-Prüfung anpassen

### Schritt 6: Manueller Test und Validierung

- [ ] App starten und Seite ohne Parameter aufrufen → Filter eingeklappt
- [ ] Auf "Filter anzeigen" klicken → Filter ausklappen, URL enthält `filter_collapsed=0`
- [ ] Seite neu laden → Filter bleiben ausgeklappt
- [ ] Auf "Filter ausblenden" klicken → Filter einklappen, URL ohne Parameter
- [ ] Gespeicherten Filter aufrufen → Filter eingeklappt (Default)
- [ ] Wochenvorschau → Links funktionieren korrekt

---

## URL-Struktur

Keine Änderungen an der URL-Struktur. Der bestehende Query-Parameter `filter_collapsed` bleibt erhalten, nur die Semantik ändert sich:

| Zustand | Vorher | Nachher |
|---------|--------|---------|
| Kein Parameter | Ausgeklappt | **Eingeklappt** |
| `filter_collapsed=1` | Eingeklappt | Eingeklappt |
| `filter_collapsed=0` | Ausgeklappt | **Ausgeklappt** |

---

## Abhängigkeiten

- Story 37 (Einklappen der Filter) - muss implementiert sein ✅
- Alle Filter-Stories (07, 08, 09, 10, 11, 12, 13) - müssen implementiert sein ✅

---

## Test-Checkliste

### Unit-Tests (Rust)
- [ ] `filter_collapsed_default_renders_collapsed` - Ohne Parameter sind Filter eingeklappt
- [ ] `filter_collapsed_zero_renders_expanded` - `filter_collapsed=0` klapp Filter aus
- [ ] `build_filter_collapsed_toggle_url_collapsed_to_expanded` - Toggle von eingeklappt zu ausgeklappt
- [ ] `build_filter_collapsed_toggle_url_expanded_to_collapsed` - Toggle von ausgeklappt zu eingeklappt
- [ ] `build_filter_collapsed_toggle_url_preserves_other_params` - Andere Parameter bleiben erhalten

### E2E-Tests (Playwright)
- [ ] Test: Filter standardmäßig eingeklappt beim ersten Aufruf
- [ ] Test: Filter ausklappen und Zustand in URL speichern (`filter_collapsed=0`)
- [ ] Test: Filter einklappen und Parameter entfernen
- [ ] Test: Gespeicherte Filter öffnen mit eingeklappten Filtern
- [ ] Test: Wochenvorschau-Links funktionieren korrekt

### Manueller Test
- [ ] Mobile Ansicht: Filter eingeklappt, mehr Platz für Rezeptliste
- [ ] Ohne JavaScript: Toggle-Link funktioniert
- [ ] Barrierefreiheit: `aria-expanded="false"` beim ersten Aufruf

---

## Risiken und Annahmen

### Risiken
| Risiko | Wahrscheinlichkeit | Auswirkung | Mitigation |
|--------|-------------------|------------|------------|
| Bestehende Bookmarks/Links verhalten sich anders | Hoch | Mittel | Dokumentieren, dass Links ohne Parameter jetzt eingeklappt öffnen |
| E2E-Tests von anderen Stories beeinflusst | Mittel | Hoch | Alle Tests durchlaufen lassen und anpassen |
| Wochenvorschau-Links funktionieren nicht mehr | Niedrig | Hoch | Explizit testen |

### Annahmen
- Alle bestehenden Tests werden angepasst und sind danach grün
- Der Parameter-Name `filter_collapsed` bleibt unverändert
- Die Wochenvorschau-Links können den Parameter entfernen (da Default jetzt eingeklappt)
- Keine weiteren Stellen im Code verwenden den `filter_collapsed` Parameter

---

## Offene Punkte

Keine offenen Punkte. Die Story ist klar definiert und die Implementierung ist straightforward.
