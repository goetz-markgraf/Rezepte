# Review: Story 47 – Rezept aus Foto erstellen

**Datum:** 2026-06-14
**Status:** Abgenommen

---

## Ergebnis

Story vollständig implementiert. Alle Anforderungen aus `story.md` wurden umgesetzt.

---

## Implementierte Änderungen

### Neue Dateien
- `src/vision.rs` — Vision-API-Integration (OpenAI-kompatibel, Base64-Encoding, JSON-Parsing)
- `templates/recipes/from_photo.html` — Upload-Formular mit Kamera-Support
- `docs/47-rezept-aus-foto/plan.md` — Technischer Implementierungsplan

### Geänderte Dateien
- `src/config.rs` — Drei neue Env-Vars: `VISION_API_URL`, `VISION_API_KEY`, `VISION_MODEL`
- `src/lib.rs` — `vision`-Modul registriert
- `src/main.rs` — `Config` wird an `create_router` übergeben
- `src/routes/mod.rs` — `AppState` mit `FromRef`-Pattern für Pool + Config; neue Routen
- `src/routes/recipes.rs` — `photo_upload_form` und `analyze_photo_handler` Handler; `index` nutzt `vision_enabled`
- `src/templates.rs` — `PhotoUploadTemplate` hinzugefügt; `IndexTemplate.vision_enabled` ergänzt
- `templates/index.html` — Bedingter „Aus Foto"-Button wenn Vision konfiguriert
- `Cargo.toml` — `reqwest` in dependencies (war nur in dev-dependencies); `axum` mit `multipart`-Feature
- Alle 18 Test-Dateien — `create_router`-Aufrufe auf neue Signatur angepasst

---

## Testergebnisse

| Testtyp | Ergebnis |
|---|---|
| Unit-Tests (`cargo test`) | 127/128 ✅ (1 pre-existing Fehler) |
| E2E-Tests (`npm run test:e2e`) | 234/235 ✅ (1 pre-existing Fehler) |
| Clippy | Keine neuen Warnungen |
| Build | Sauber, keine Warnings |

Der eine fehlschlagende Test in beiden Suites ist `not_made_recently_combined_with_category_filter` — ein pre-existing Bug, der vor Story 47 bereits rot war.

---

## Findings / Prio-1-Issues

Keine.

---

## Architekturentscheidungen

- **`AppState` mit `FromRef`**: Alle bisherigen Handler nutzen `State<Arc<SqlitePool>>` und mussten nicht geändert werden. Nur neue Handler greifen auf `State<Arc<Config>>` zu.
- **Kein Base64-Crate**: Eigene schlanke Base64-Implementierung verwendet, um die Dependency-Liste sauber zu halten.
- **Kein Redirect nach Analyse**: Der vorausgefüllte Formular-HTML wird direkt gerendert, da Formular-Daten nicht per GET-Redirect übergeben werden können.
- **Feature-Toggle via Env**: Fehlt `VISION_API_KEY` oder `VISION_API_URL`, ist der „Aus Foto"-Button nicht sichtbar und der Upload-Endpunkt gibt einen BadRequest zurück.
