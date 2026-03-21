# ADR 001: UI-Integrationstests mit Playwright

## Status

**Entschieden** - Gültig ab Story 01

## Kontext

Jede Story erfordert UI-Integrationstests, die gegen die laufende Anwendung ausgeführt werden. Wir müssen entscheiden:
1. Welches Test-Framework?
2. Wie werden Testdaten verwaltet?
3. Wie wird die Test-Isolierung gewährleistet?
4. Wie starten wir App + Tests?

## Entscheidungen

### 1. Test-Framework: Playwright

**Entscheidung:** Wir verwenden Playwright für UI-Integrationstests.

**Begründung:**
- Schnellste Execution (parallele Tests)
- Alle Browser (Chrome, Firefox, Safari, Edge)
- Auto-waiting für stabile Tests
- Built-in tracing und debugging
- WebServer-Config kann Rust-App automatisch starten
- Bessere Entwickler-Erfahrung als Cypress oder Selenium

**Alternativen abgelehnt:**
- Cypress: Nur Chrome/Firefox, langsamer
- Puppeteer: Nur Chrome, weniger E2E-Features
- Selenium: Veraltet, langsam, wartungsintensiv

### 2. Testdaten: Seeds über SQL-Skripte

**Entscheidung:** Testdaten werden über SQL-Seed-Skripte in einer separaten Testdatenbank eingespielt.

**Struktur:**
```
tests/
├── seeds/
│   ├── 001_test_recipes.sql    # Basis-Testdaten
│   └── 002_scenario_x.sql      # Spezifische Szenarien
├── playwright.config.ts
└── e2e/
    └── recipes.spec.ts
```

**Begründung:**
- Einfach und versionierbar (Plain SQL)
- Nachvollziehbar und reviewbar
- Schneller als API-Calls für Setup
- Konsistente Ausgangslage für jeden Test

**Prozess vor jedem Test-File:**
1. `TEST_DATABASE_URL` verwenden (z.B. `./data/test.db`)
2. `DELETE FROM recipes;` - Tabelle leeren
3. Seed-Skript ausführen
4. Tests ausführen

### 3. Test-Isolation: Separate SQLite-Datei pro Test-Run

**Entscheidung:** Jeder Test-Run verwendet eine eigene SQLite-Datenbank-Datei.

**Konfiguration:**
- Umgebungsvariable: `TEST_DATABASE_URL=./data/test.db`
- Wird vor Test-Start geleert und mit Seeds befüllt
- Parallelisierung auf File-Level (Playwright handhabt parallele Workers)

**Vorteile:**
- Keine Seiteneffekte zwischen Tests
- Einfaches Reset (Datei löschen/neu erstellen)
- Schnell (SQLite ist schnell genug)

### 4. App-Start: Playwright webServer Config

**Entscheidung:** Playwright startet die Rust-Applikation automatisch via `webServer` Konfiguration.

**Konfiguration in `playwright.config.ts`:**
```typescript
export default defineConfig({
  webServer: {
    command: 'cargo run --bin rezepte',
    url: 'http://localhost:8080/health',
    reuseExistingServer: true,
    env: {
      DATABASE_URL: './data/test.db',
      PORT: '8080'
    }
  },
  // ...
});
```

**Begründung:**
- Entwickler muss nicht manuell `cargo run` starten
- Tests warten automatisch bis Server ready
- Nahtlose Integration in CI/CD

## Konsequenzen

### Positiv
- Schnelle Feedback-Schleife durch automatischen App-Start
- Konsistente, reproduzierbare Tests
- Einfaches Test-Setup für neue Entwickler
- Tests können lokal und in CI gleich ausgeführt werden

### Negativ
- Zusätzliche Toolchain (Node.js + Playwright) neben Rust
- Testdaten müssen gepflegt werden (SQL-Skripte)
- Build-Zeit erhöht sich (Rust binary muss kompiliert werden)

## Implementation

### Story 01 - Projekt Setup

Die folgenden zusätzlichen Schritte sind nötig:

1. **Playwright initialisieren**
   - `npm init -y` im Root
   - `npm install --save-dev @playwright/test`
   - `npx playwright install`

2. **Testdaten-Struktur erstellen**
   - `tests/seeds/001_test_recipes.sql`
   - `tests/e2e/health.spec.ts` (erster Test)

3. **Playwright Konfiguration**
   - `playwright.config.ts` mit webServer
   - Umgebungsvariablen für Test-DB

4. **Test-Skripte**
   - `npm run test:e2e` - Führt Playwright Tests aus
   - `npm run test:e2e:ui` - Mit UI-Modus zum Debuggen

### Beispiel Test (Story 01)

```typescript
// tests/e2e/health.spec.ts
import { test, expect } from '@playwright/test';

test('health check returns OK', async ({ page }) => {
  await page.goto('http://localhost:8080/health');
  await expect(page.locator('body')).toContainText('OK');
});
```

## Verwandte Entscheidungen

- Siehe auch: `architecture.md` Abschnitt "Testing Strategy"
- Siehe auch: `AGENTS.md` Abschnitt "UI-Integrationstests"

## Datum

2026-03-21
