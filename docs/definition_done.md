# Definition of done

Dieses Dokument beschreibt, welche Anforderungen eine Implementierung erfüllen muss,
damit sie fertig ist.

Alle diese Punkte müssen einzeln und nacheinander geprüft werden.

## 1. Code-Qualität

### Compiler & Linting
- [ ] Keine Compiler-Fehler oder Warnungen (`cargo build`)
- [ ] Keine Clippy-Warnings (`cargo clippy -- -D warnings`)
- [ ] Code ist korrekt formatiert (`cargo fmt --check`)
- [ ] Kein ungenutzter Code (`cargo check` ohne Warnungen)

### Lesbarkeit & Wartbarkeit
- [ ] Verständliche Variablen- und Funktionsnamen (keine Abkürzungen)
- [ ] Funktionen haben maximal 50 Zeilen, maximal 3-4 Parameter
- [ ] Komplexe Logik ist dokumentiert oder in kleinere Funktionen aufgeteilt
- [ ] Konsistente Einrückung und Formatierung
- [ ] Der Code ist kurz und auf den Punkt, ohne unnötige Schleifen
- [ ] Es gibt keine unnötigen Duplikationen, wiederverwendete Funktionen sind in eigenen Funktionen mit sprechendem Namen

---

## 2. Architektur-Einhaltung

### Tech Stack Konformität
- [ ] Verwendung von: Rust + Axum + Askama + sqlx + SQLite + HTMX
- [ ] Server-Side Rendering (keine JSON-APIs für UI)
- [ ] Hypermedia-Driven Architecture (HTMX für Interaktivität)
- [ ] Formulare über normale POST-Requests

### Projektstruktur
- [ ] Code liegt im korrekten Verzeichnis (`src/models/`, `src/routes/`, `src/templates/`)
- [ ] Module sind korrekt exportiert in `lib.rs`
- [ ] Askama-Templates im `templates/`-Verzeichnis
- [ ] Statische Assets im `static/`-Verzeichnis

### Datenbank
- [ ] SQLx-Migrationen vorhanden und funktionieren (`sqlx migrate run`)
- [ ] Keine SQL-Injection-Risiken (nur sqlx-Queries mit parametrisierten Statements)
- [ ] Datenbank-Indizes für Performance (falls neue Queries hinzugefügt)

### URLs & Routing
- [ ] DeepLink-fähige URLs mit Query-Parametern
- [ ] App funktioniert ohne JavaScript (Form-Posts + Redirects)
- [ ] HTMX-Attribute korrekt gesetzt für Progressive Enhancement

---

## 3. Testing

### Unit Tests
- [ ] Neue Funktionalität hat Unit Tests (`cargo test`)
- [ ] Tests decken Happy Path und Edge Cases ab
- [ ] Tests laufen erfolgreich durch

### E2E Tests
- [ ] Playwright-Tests für neue Features geschrieben
- [ ] E2E-Tests bestehen (`npm run test:e2e`)
- [ ] Screenshots/Traces bei Fehlern verfügbar

### Testabdeckung
- [ ] Mindestens 80% Code-Coverage für neue Features
- [ ] Kritische Pfade (DB-Queries, Validation) sind getestet

---

## 4. Funktionale Anforderungen

### Akzeptanzkriterien
- [ ] Alle Akzeptanzkriterien aus der Story.md sind erfüllt
- [ ] Funktionalität entspricht der fachlichen Beschreibung
- [ ] Edge Cases sind dokumentiert und behandelt

### Validierung
- [ ] User Input wird validiert (z.B. Titel max 255 Zeichen)
- [ ] Kategorien entsprechen `VALID_CATEGORIES`
- [ ] Error Messages sind verständlich für den User

### Error Handling
- [ ] Keine Panics oder unwraps im Produktivcode
- [ ] Fehler werden korrekt abgefangen und geloggt
- [ ] Benutzer sieht hilfreiche Fehlermeldungen (nicht technische Details)

---

## 5. Deployment & Build

### Docker
- [ ] Docker-Image baut erfolgreich (`docker build`)
- [ ] Multi-stage Build funktioniert für ARM64 (Raspberry Pi/NAS)
- [ ] Container startet ohne Fehler

### Datenbank-Setup
- [ ] SQLite-Datenbank wird korrekt initialisiert
- [ ] Migrationen laufen automatisch beim Start
- [ ] Backup-Pfad ist konfiguriert

### Umgebungsvariablen
- [ ] Neue Konfiguration ist in `config.rs` dokumentiert
- [ ] Defaults sind sinnvoll gesetzt
- [ ] Dokumentation in `architecture.md` aktualisiert

---

## 6. Dokumentation

### Code-Dokumentation
- [ ] Öffentliche Funktionen und Structs haben Doc-Kommentare (`///`)
- [ ] Komplexe Business-Logik ist kommentiert
- [ ] Beispiele in Doc-Kommentaren (falls sinnvoll)

### Projekt-Dokumentation
- [ ] Architektur-Dokumentation ist bei Änderungen aktualisiert
- [ ] Neue Endpunkte/URLs sind dokumentiert
- [ ] ADR erstellt, falls Architektur-Entscheidungen getroffen wurden

---

## 7. Sicherheit & Performance

### Sicherheit
- [ ] Keine hartkodierten Secrets oder Passwörter
- [ ] User Input wird escaped (XSS-Prevention bei Templates)
- [ ] Keine sensiblen Daten im Logging

### Performance
- [ ] Datenbank-Queries sind optimiert (keine N+1 Probleme)
- [ ] Connection Pooling ist konfiguriert
- [ ] Statische Assets werden gecacht

---

## Checkliste vor dem Merge

- [ ] Alle oben genannten Punkte sind erfüllt
- [ ] Code Review durchgeführt (mindestens 1 Approval)
- [ ] Branch ist auf aktuellem Stand mit main
- [ ] Keine Merge-Konflikte
- [ ] Commit-Messages sind verständlich und folgen Konventionen

---

*Letzte Aktualisierung: 2026-03-21*
