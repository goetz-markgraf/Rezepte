# Implementierungsplan: Story 30 - Wochenpicker zeigt geplantes Essen

## Technische Schritte

### Schritt 1: Datenbank-Layer
- [ ] Neue Methode `get_recipes_by_date_range()` in `src/models/recipe_db.rs` erstellen
  - Parameter: `start_date` und `end_date` (time::Date)
  - Query: Rezepte im Datumsbereich selektieren
  - Rückgabe: `Vec<Recipe>` sortiert nach Datum
- [ ] Unit-Test: `get_recipes_by_date_range()` testet Datumsbereich und Sortierung

### Schritt 2: Template-Daten-Struktur
- [ ] Neue Struct `WeekdayPickerRecipeInfo` in `src/templates.rs` erstellen
  - Felder: `id: i64`, `title: String`
  - Wird pro Tag 0-9 im Picker benötigt
- [ ] RecipeFormTemplate erweitern um `planned_recipes: Vec<Option<WeekdayPickerRecipeInfo>>`
  - Index 0-9 entspricht den 10 Tagen im Picker (morgen bis +10 Tage)
  - `None` = kein Rezept geplant, `Some(...)` = Rezept vorhanden

### Schritt 3: Routes und Handler
- [ ] Handler `new_recipe_form()` anpassen in `src/routes/recipes.rs`
  - Daten für nächste 10 Tage (morgen bis +10 Tage) laden
  - Pro Tag prüfen: gibt es ein Rezept mit diesem `planned_date`?
  - `planned_recipes` Vektor für Template füllen
- [ ] Handler `edit_recipe_form()` anpassen in `src/routes/recipes.rs`
  - Gleiche Logik wie `new_recipe_form()` für Edit-Modus
- [ ] Unit-Test: Handler testet dass `planned_recipes` korrekt gefüllt wird

### Schritt 4: Templates
- [ ] `templates/recipes/form.html` erweitern
  - Pro Wochenpicker-Button prüfen ob `planned_recipes[index]` Some ist
  - Wenn ja: Link mit Icon anzeigen (Rechts oben im Button)
  - Icon: Blauer Stern (⭐ oder SVG)
  - Tooltip-Attribut (`title` für Basis, `data-tooltip` für custom CSS)
  - Link: `/recipes/{recipe_id}`
  - Aria-Label: "Geplantes Essen: {recipe_name}"
- [ ] CSS für Indikator im `static/css/app.css` hinzufügen
  - `.weekday-btn-planned-indicator` - Position: absolute, rechts oben
  - Farbe: Blau (#0066CC)
  - Größe: 16x16px
  - Z-Index: Über dem Button-Content
  - Hover-Effekt für Tooltip

### Schritt 5: Tooltip-Implementierung
- [ ] CSS-Only Tooltip für den Indikator
  - `position: relative` auf dem Link
  - `::after` Pseudo-Element für Tooltip-Text
  - `position: absolute`, erscheint bei `:hover` oder `:focus`
  - Max-Width: 200px, Textumbruch erlaubt
  - Verzögerung: CSS transition-delay für 200ms
- [ ] Mobile Fallback: `title` Attribut für Touch-Geräte

### Schritt 6: E2E-Tests
- [ ] `tests/e2e/weekday-picker-planned-indicator.spec.ts` erstellen:
  - **Test 1:** Indikator wird für geplante Tage angezeigt
    - Given: Rezept für morgen geplant
    - When: Formular mit Wochenpicker öffnen
    - Then: Indikator (Stern) sichtbar auf Button für morgen
  - **Test 2:** Kein Indikator für ungeplante Tage
    - Given: Kein Rezept für übermorgen geplant
    - When: Formular öffnen
    - Then: Kein Indikator auf Button für übermorgen
  - **Test 3:** Tooltip zeigt Rezeptname beim Hover
    - Given: Rezept "Pasta Carbonara" für morgen geplant
    - When: Über Indikator hover
    - Then: Tooltip mit Text "Pasta Carbonara" erscheint
  - **Test 4:** Klick navigiert zur Detailseite
    - Given: Rezept ID 123 für morgen geplant
    - When: Auf Indikator klicken
    - Then: Navigation zu `/recipes/123`
  - **Test 5:** Tastatur-Navigation funktioniert
    - Given: Rezept für morgen geplant
    - When: Indikator mit Tab fokussieren
    - Then: Fokus sichtbar, Enter öffnet Detailseite
  - **Test 6:** Mehrere Rezepte an einem Tag
    - Given: 2 Rezepte für morgen geplant
    - When: Formular öffnen
    - Then: Indikator sichtbar, Tooltip zeigt ersten Rezeptnamen

### Schritt 7: Styling und Barrierefreiheit
- [ ] Fokus-Indikator für Tastatur-Navigation sichtbar machen
  - `outline` oder `box-shadow` bei `:focus-visible`
  - Kontrast mindestens 4.5:1
- [ ] Screenreader-Test: aria-label wird korrekt vorgelesen
- [ ] CSS-Kontrast prüfen für blauen Stern

---

## URL-Struktur

Keine neuen Endpunkte nötig - Nutzung bestehender URLs:

```
GET  /recipes/new     →  Neue Formular-Ansicht mit gefülltem planned_recipes
GET  /recipes/{id}/edit  →  Edit-Ansicht mit gefülltem planned_recipes
GET  /recipes/{id}   →  Ziel der Indikator-Links (Detailseite)
```

---

## Abhängigkeiten

- **Story 29:** Wochen-Picker erweitern muss implementiert sein (✅ abgeschlossen)
- **Story 28:** Essen planen muss implementiert sein (✅ abgeschlossen)
- **Technisch:** Vorhandene `planned_date` Spalte in `recipes` Tabelle verwenden

---

## Test-Checkliste

- [ ] **Unit-Test:** `get_recipes_by_date_range()` - Lädt Rezepte korrekt für Datumsbereich
- [ ] **Unit-Test:** Handler setzt `planned_recipes` korrekt im Template
- [ ] **E2E-Test:** Indikator wird nur für Tage mit geplantem Essen angezeigt
- [ ] **E2E-Test:** Tooltip erscheint beim Hover mit korrektem Rezeptnamen
- [ ] **E2E-Test:** Klick auf Indikator navigiert zur Detailseite
- [ ] **E2E-Test:** Tastatur-Navigation (Tab + Enter) funktioniert
- [ ] **Manueller Test:** Screenreader liest aria-label korrekt vor
- [ ] **Manueller Test:** CSS-Kontrast des blauen Sterns prüfen

---

## Offene Punkte

- [x] **Entschieden:** Icon-Auswahl - Verwendung eines blauen SVG-Sterns (⭐) für bessere Skalierung
- [x] **Entschieden:** Bei mehreren Rezepten an einem Tag wird nur der erste im Tooltip angezeigt (einfachere UX)
- [ ] Zu prüfen: Soll der Indikator den Button selbst überlagern oder daneben platziert werden?

---

## Architektur-Entscheidungen (ADR)

**Keine ADR nötig** - Die Implementierung folgt den bestehenden Architektur-Mustern:
- Server-Side Rendering mit Askama
- Daten werden beim Seitenaufruf geladen (keine zusätzlichen Requests)
- Progressive Enhancement: Funktioniert auch ohne JavaScript (title-Attribut als Fallback)

---

## Implementierungsdetails

### Datenbank-Query

```rust
// src/models/recipe_db.rs
pub async fn get_recipes_by_date_range(
    pool: &SqlitePool,
    start_date: time::Date,
    end_date: time::Date,
) -> Result<Vec<Recipe>, sqlx::Error> {
    sqlx::query_as::<_, Recipe>(
        r#"
        SELECT id, title, categories, ingredients, instructions, planned_date, created_at, updated_at, rating
        FROM recipes
        WHERE planned_date >= ?1 AND planned_date <= ?2
        ORDER BY planned_date ASC, title ASC
        "#,
    )
    .bind(start_date)
    .bind(end_date)
    .fetch_all(pool)
    .await
}
```

### Template-Anpassung (Konzept)

```html
<!-- templates/recipes/form.html -->
<button class="weekday-btn" ...>
  Mo 30.3.
  {% if let Some(recipe) = planned_recipes[i] %}
  <a href="/recipes/{{ recipe.id }}" 
     class="planned-indicator"
     title="{{ recipe.title }}"
     aria-label="Geplantes Essen: {{ recipe.title }}">
    <span class="indicator-icon" aria-hidden="true">⭐</span>
  </a>
  {% endif %}
</button>
```

### CSS-Konzept

```css
/* static/css/app.css */
.planned-indicator {
  position: absolute;
  top: 2px;
  right: 2px;
  width: 16px;
  height: 16px;
  color: #0066CC;
  text-decoration: none;
  display: flex;
  align-items: center;
  justify-content: center;
}

.planned-indicator:focus-visible {
  outline: 2px solid #0066CC;
  outline-offset: 2px;
}

/* Tooltip via title-Attribut (Browser-Default) */
/* Zusätzlich CSS-Tooltip für bessere Kontrolle */
.planned-indicator::after {
  content: attr(title);
  position: absolute;
  bottom: 100%;
  left: 50%;
  transform: translateX(-50%);
  background: #333;
  color: white;
  padding: 4px 8px;
  border-radius: 4px;
  font-size: 12px;
  white-space: nowrap;
  opacity: 0;
  visibility: hidden;
  transition: opacity 0.2s ease, visibility 0.2s ease;
  transition-delay: 200ms;
  pointer-events: none;
  max-width: 200px;
  white-space: normal;
}

.planned-indicator:hover::after,
.planned-indicator:focus::after {
  opacity: 1;
  visibility: visible;
}
```

---

## Akzeptanzkriterien-Abdeckung

| Kriterium | Schritt | Test |
|-----------|---------|------|
| K1: Visueller Indikator | 4, 7 | E2E-Test 1 |
| K2: Tooltip mit Rezeptname | 5 | E2E-Test 3 |
| K3: Navigation zur Detailseite | 4 | E2E-Test 4 |
| K4: Tage ohne Planung | 4 | E2E-Test 2 |
| K5: Performance | 3 | Kein zusätzlicher Request |
| K6: Barrierefreiheit | 4, 5, 7 | E2E-Test 5, Manueller Test |
