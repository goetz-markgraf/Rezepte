# Story 01: Projekt-Setup und Initialisierung

## Beschreibung

Als Entwickler möchte ich das Rust-Projekt initialisieren und die grundlegende Projektstruktur aufsetzen, damit die weitere Entwicklung beginnen kann.

## Kontext

Das Projekt "Rezepte" ist bisher nur in Dokumenten beschrieben. Diese erste Story legt das technische Fundament:
- Rust-Projektstruktur mit Cargo
- Alle notwendigen Dependencies (Axum, Askama, sqlx, Tokio, etc.)
- Ordnerstruktur für Source-Code, Templates, Static-Assets und Datenbank-Migrationen
- Grundlegende Datenbank-Konfiguration und Migrationen
- Erste lauffähige Version mit "Hello World"

## Akzeptanzkriterien

1. **Projektstruktur existiert**
   - [x] Cargo.toml mit allen Dependencies ist erstellt
   - [x] Verzeichnisstruktur `src/`, `templates/`, `static/css/`, `migrations/` existiert
   - [x] `.gitignore` für Rust-Projekt ist vorhanden

2. **Datenbank ist konfiguriert**
   - [x] Erste Migration `001_initial.sql` erstellt die recipes-Tabelle
   - [x] sqlx ist konfiguriert für compile-time query checking

3. **Grundgerüst läuft**
   - [x] `cargo run` startet den Server erfolgreich
   - [x] Server antwortet auf Port 8080
   - [x] Health-Check Endpunkt `/health` gibt "OK" zurück

4. **TDD-Setup funktioniert**
   - [x] Erster Integrationstest existiert und ist grün
   - [x] Test läuft mit `cargo test` erfolgreich

## Out of Scope

- Keine CRUD-Operationen für Rezepte
- Keine Templates oder HTML-Ausgabe
- Keine HTMX-Integration
- Kein Docker-Setup (kommt in späterer Story)

## Notizen

Dies ist eine reine Infrastruktur-Story. Sie schafft die technische Basis für alle folgenden Stories.
