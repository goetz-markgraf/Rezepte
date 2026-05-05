# Implementierungsplan: Story 46 — Emoji für Rezepte mit Beschreibung in Listen

## Technische Schritte

### Schritt 1: Emoji-Modul erstellen (Unit Tests First!)
- [ ] `src/emoji.rs` erstellen mit Funktion `pub fn recipe_emoji(title: &str, ingredients: Option<&str>, instructions: Option<&str>) -> Option<&'static str>`
- [ ] Mapping: Keywörter → Emoji (z.B. "Nudeln" → 🍝, "Brot" → 🍞, "Kuchen" → 🎂, "Salat" → 🥗, "Suppe" → 🍲, "Party" → 🎉, etc.)
- [ ] Kategoriefallback: Bei leeren title/ingredients/instructions aber bekannten Kategorien (Mittagessen, Brot, Party, Kuchen, Snacks) passendste Kategoriewahl nutzen
- [ ] Fallback-Emoji: 🍽️ wenn nichts passt
- [ ] Unit-Tests schreiben:
  - `title "Kuchen" → "🎂"`
  - `ingredients "Nudeln" → "🍝"`
  - `title + ingredients + instructions kombinieren`
  - `leere Felder + keine Kategorie → None`
  - `nur whitespace ingredients → None`
  - `Kategorie "Brot" mit null Inhalt → "🍞"`
  - `Fallback bei keiner Übereinstimmung → "🍽️"`

### Schritt 2: RecipeListItem erweitern
- [ ] `src/templates.rs`: `RecipeListItem` um Feld `pub emoji: Option<&'static str>` erweitern
- [ ] `src/routes/recipes.rs`: Bei Erstellung von `RecipeListItem` den `emoji`-Value aus der recipe berechnen
- [ ] `src/models/recipe.rs`: Falls nötig, ingredients/instructions direkt an die Route übergeben (jetzt über `map` verloren)

### Schritt 3: Templates anpassen
- [ ] `templates/index.html`: Emoji vor dem Rezeptnamen in `.recipe-item-link` einfügen
- [ ] `templates/wochenvorschau.html`: Emoji vor dem Rezeptnamen einfügen
- [ ] `templates/heute.html`: Emoji vor dem Rezeptnamen einfügen
- [ ] Prüfen: Auch HTMX-Fragmente (`.recipe-item`-Partial falls vorhanden) enthalten Emoji

### Schritt 4: CSS anpassen
- [ ] `src/static/css/app.css`: Styling für Emoji in der Liste (`emoji-font-size`, `margin-right`, etc.)
- [ ] Mobile-Responsive: Emoji auf kleinem Bildschirm nicht zu groß

### Schritt 5: UI-Integrationstest (Playwright)
- [ ] `tests/seeds/46-emoji-recipes.sql` erstellen mit:
  - Rezept `Pizza` mit Zutaten und Anleitung (→ sollte Emoji zeigen)
  - Rezept `Ohne Beschreibung` ohne Zutaten und Anleitung (→ kein Emoji)
  - Rezept `Brot` mit nur Titel (→ sollte Kategoriefallback Emoji zeigen)
- [ ] `tests/e2e/recipe-emoji.spec.ts` schreiben:
  - `sollte Emoji für Rezept mit Zutaten und Anleitung anzeigen (K1)`
  - `sollte kein Emoji für Rezept ohne Zutaten und Anleitung anzeigen (K2)`
  - `sollte passendes Emoji basierend auf Schlüsselwörtern wählen (K3)`
  - `sollte Emoji in der Wochenplanung anzeigen (K1)`

---

## Betroffene Dateien

| Datei | Änderung |
|-------|----------|
| `src/emoji.rs` | **Neu** — Emoji-Berechnung |
| `src/lib.rs` | `pub mod emoji;` hinzufügen |
| `src/models/recipe.rs` | Recipe struct bleibt unverändert |
| `src/templates.rs` | `RecipeListItem.emoji` hinzufügen |
| `src/routes/recipes.rs` | Emoji bei RecipeListItem-Berechnung |
| `templates/index.html` | Emoji im Template rendering |
| `templates/wochenvorschau.html` | Emoji im Template rendering |
| `templates/heute.html` | Emoji im Template rendering |
| `src/static/css/app.css` | CSS für Emoji-Styling |
| `tests/seeds/46-emoji-recipes.sql` | **Neu** — Tests seeds |
| `tests/e2e/recipe-emoji.spec.ts` | **Neu** — E2E Tests |

## Abhängigkeiten

- **Keine externen Abhängigkeiten** von anderen Stories
- Verwendet bestehende `VALID_CATEGORIES` in `src/models/recipe.rs`
- Verwendet bestehende `Recipe` struct ohne Änderungen

## Test-Checkliste

- [ ] Unit-Tests: `recipe_emoji()` Funktion (mindestens 6 Fälle)
- [ ] Unit-Tests: Kategoriefallback (alle 5 Kategorien)
- [ ] E2E-Test: Emoji auf Rezeptliste sichtbar
- [ ] E2E-Test: Kein Emoji bei leerem Rezept
- [ ] E2E-Test: Emoji in Wochenvorschau sichtbar
- [ ] E2E-Test: Keyword-basierte Emoji-Auswahl korrekt
- [ ] Manuell: Responsive-Test auf Mobile (Emoji lesbar, nicht zu groß)
- [ ] `cargo test` alle grün
- [ ] `npm run test:e2e` alle grün
- [ ] `cargo fmt --check` ✅
- [ ] `cargo clippy -- -D warnings` ✅

## Offene Punkte

- Keine — kein Query-Parameter nötig, Emoji wird serverseitig berechnet aus vorhandenen Feldern
