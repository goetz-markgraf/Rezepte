## Implementierungsplan für Story 41: Bewertungssterne im Bearbeitungsmodus

### 🎯 Zielsetzung
Das Ziel ist die Behebung des Bugs, bei dem im Bearbeitungsmodus eines Rezepts nicht korrekt alle Sterne bis zur gewählten Bewertung markiert werden. Die Darstellung muss konsistent mit der Detailansicht sein.

### 🛠️ Technische Basis
*   **Kontext:** Story 41/docs/41-bewertungssterne-im-edit-mode/
*   **Story:** Star rating visualization in edit mode.
*   **Architektur-Anforderung:** Einhaltung des SSR/HTMX-Patterns. Die Komponente muss Client-seitiges State-Handling (Hover/Click) über HTMX-Events oder minimales JS (wenn zwingend nötig, falls HTMX nicht reicht) lösen.
*   **DoD-Anforderung:** Volle Abdeckung von Testen (Unit, Integration, E2E).

### 🚀 Phasenplan

#### Phase 1: Analyse & Proof of Concept ($\sim$1 Day)
[ ] **Analyse des Defekts:** Exakte Ursache im SPA/HTMX-Flow bestimmen. Ist es ein reines Klassendefekt (CSS) oder ein State-Management-Fehler (JS/HTMX)?
[ ] **Minimaler Proof of Concept (POC):** Erstellen eines isolierten Testcontainers/Test-Templates, um die korrekte Ausgabe von Stern-DOM-Elementen (1 bis N Sterne) zu validieren.
[ ] **Betroffene Dateien identifizieren:** Bestätigen, welche Teile der `recipes.rs` und welche View-Teile (z.B. Komponente) angepasst werden müssen.

#### Phase 2: Implementierung (Core Logic) ($\sim$1-2 Days)
[ ] **Component-Refactoring (Frontend View):** Die Logik zur Generierung der Stern-HTML-Struktur (`_rating_component.html` oder ähnliches) muss angepasst werden.
    *   **Logik:** Implementieren des Hover/Click-Verhaltens in der View, das **alle** Sterne bis zu `N` aktiviert, nicht nur den N-ten.
    *   **HTMX Konformität:** Wenn ein Klick stattfindet, muss ein HTMX-Event ausgelöst werden, um den angedrückten Wert an den Server zu senden, oder die gesamte Stern-Komponente muss durch ein Client-State-Update ersetzt werden, das die SSR-Logik triggert.
[ ] **Backend-Anpassung (Rust/Axum):** Prüfen, ob die Aktualisierung des Bewertungszustands nur durch ein simples POST-Request ausreichend ist oder ob es State-Validierungen gibt. (Wahrscheinlich: `POST /recipes/{id}/rate`).
[ ] **Validierung:** Sicherstellen, dass die Serialisierung/Deserialisierung von `rating: i32` im Backend robust bleibt, unabhängig von der Quelle.

#### Phase 3: Testendeckung (DoD-Einhaltung) ($\sim$1 Day)
[ ] **Unit Tests:**
    [ ] Unit Tests für die Geschäftslogik der Bewertungsberechnung (z.B. `fn calculate_lit_stars(rating: i32) -> Vec<Star>`) in `src/models/recipe.rs`.
    [ ] Tests für die Validierung von `rating` bei eingehenden Daten.
[ ] **Integrationstests:**
    [ ] Integrationstest für den neuen POST-Endpunkt `/recipes/{id}/rate` um sicherzustellen, dass der Datenbank-State korrekt aktualisiert wird.
    [ ] Test des gesamten Schreibzyklus: Bearbeiten $\rightarrow$ Rating setzen $\rightarrow$ Speichern $\rightarrow$ DB-Check.
[ ] **E2E Tests (Playwright):**
    [ ] Implementierung des Testfalls "Hover/Click Stern N zeigt Sterne 1 bis N". (Testfall 1 aus Story.md).
    [ ] Implementierung des Testfalls "Konsistenz zwischen Edit- und View-Modus". (Testfall 2 aus Story.md).

#### Phase 4: Review & Polish
[ ] **Review:** Code-Review durch einen Kollegen durchführen.
[ ] **Finalisierung:** Abschluss des PR mit korrektem Status in `review.md`.

### 📂 Betroffene Dateien (Vorschlag)
*   `docs/41-bewertungssterne-im-edit-mode/story.md` (Quelle)
*   `src/models/recipe.rs` (Business Logic, ggf. neue Methoden)
*   `src/routes/recipes.rs` oder `src/routes/api.rs` (Handler für Rating-Update)
*   `src/templates/recipes/form.html` (oder die zugrundeliegende Komponente)
*   `tests/e2e/recipes.spec.ts` (Playwright-Hinzufügung)

### ✅ Abschluss-Commit
Der finale Commit wird wie gewünscht erstellt: `git commit -m "story 41: implementation plan"`