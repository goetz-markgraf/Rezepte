# Story 46: Emoji für Rezepte mit Beschreibung in Listen

**Epic:** Epic 2: Rezept-Übersicht & Navigation
**Status:** Offen

---

## 1. Story-Satz

Als **Nutzer der Rezepte-App** möchte ich, dass Rezepte mit einer Beschreibung (Zutaten oder Anleitung, die nicht nur Whitespace enthalten) ein passendes Emoji neben dem Namen angezeigt bekommen, damit ich in der Listenübersicht und Wochenplanung auf einen Blick erkenne, um welche Art von Rezept es sich handelt.

---

## 2. Weiterer Kontext

Nicht alle Rezepte haben eine ausformulierte Beschreibung. In der Hauptliste und der Wochenplanung soll visuell deutlich werden, welche Rezepte beschrieben sind und welche nicht. Das Emoji sollte aus dem Inhalt (Zutaten und/oder Beschreibung) abgeleitet und "passend" sein.

---

## 3. Akzeptanzkriterien

### Funktionale Kriterien

- Angenommen ein Rezept hat Zutaten und/oder eine Beschreibung (kein reiner Whitespace)
  Wenn das Rezept in der Liste (Übersicht oder Wochenplanung) angezeigt wird
  Dann wird dem Rezeptnamen ein passendes emoji vorangestellt

- Angenommen ein Rezept hat keine Zutaten und keine Beschreibung (oder nur Whitespace)
  Wenn das Rezept in der Liste angezeigt wird
  Dann wird kein emoji angezeigt

- Angenommen das Emoji wird angezeigt
  Wenn der Nutzer das Rezept ansieht
  Dann steht das Emoji direkt vor/hinter dem Rezeptnamen

### Nicht-funktionale Kriterien

- Das Emoji wird serverseitig berechnet (HTMX-freundlich, kein JS nötig)
- Fallback-Emoji falls kein passendes gefunden werden kann
- Die Emoji-Auswahl sollte auf den Inhalten der Zutaten und der Beschreibung basieren

---

## 4. Technische Planung

### Emoji-Logik

- **Neues Modul**: `src/utils/emoji.rs` — eine Pub-Funktion `fn recipe_emoji(title: &str, ingredients: Option<&str>, instructions: Option<&str>) -> Option<&'static str>`
- Die Funktion prüft: Sind ingredients oder instructions vorhanden und nicht nur Whitespace? Wenn nein → `None`
- Wenn ja → extrahiere Schlüsselwörter aus title, ingredients und instructions → wende Mapping-Regeln an → bestes Emoji
- **Fallback**: 🍽️ (allgemein, wenn nichts passt)
- **Keine externen Dependencies** — alles hardcoded in einem kleinen Mapping (Wort → Emoji)

### Templates

- Die Listen-Template (`_recipe_row.html` oder äquivalent) wird um ein Emoji-Feld erweitert
- Die Wochenplan-Template zeigt ebenfalls das Emoji
- Das Emoji wird direkt vor dem Rezeptnamen gerendert

### Datenmodell

- **Keine Änderung** am Datenmodell — Emoji wird serverseitig berechnet, nicht persistiert

---

## 5. Teststrategie

### Unit Tests (Rust)
- Test `recipe_emoji()` Funktion für jedes Akzeptanzkriterium:
  - ingredients vorhanden → ein Emoji zurückgeben
  - instructions vorhanden → ein Emoji zurückgeben
  - nur Whitespace → `None`
  - title enthält bekannte Schlüsselwörter → passendes Schlüsselwort-Emoji
  - keine Übereinstimmung → Fallback-Emoji
- Test Kategorie-Mapping (Mittagessen → 🥘, Brot → 🍞, Kuchen → 🎂, Snacks → 🍿, Party → 🥳)

### UI-Integrationstest (Playwright)
- SQL-Seed: Rezept mit Zutaten/Anleitung erstellen
- Navigiere zur Rezeptliste
- Prüfe: Emoji ist neben dem Rezeptnamen sichtbar
- SQL-Seed: Rezept ohne Zutaten/Anleitung
- Prüfe: Kein Emoji ist sichtbar

---

## 6. Abhängigkeiten & Rahmenbedingungen

- **Keine externen Abhängigkeiten** von anderen Stories
- Alle Templates müssen geändert werden (Liste + Wochenplanung + HTMX-Fragmente)
- Muss in responsive Layout passen (insbesondere Mobile) — Emoji darf Platz nicht überladen
