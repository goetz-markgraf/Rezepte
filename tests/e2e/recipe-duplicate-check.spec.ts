import { test, expect } from '@playwright/test';

/**
 * Erzeugt einen eindeutigen String aus Timestamp + Zufallszahl,
 * damit parallele Test-Worker keine Kollisionen erzeugen.
 */
function uid(): string {
  return `${Date.now()}${Math.floor(Math.random() * 100000)}`;
}

/**
 * Erstellt ein Rezept über das Formular und wartet auf die Detailseite.
 * Gibt die ID des neuen Rezepts zurück.
 */
async function createRecipe(page: import('@playwright/test').Page, title: string): Promise<string> {
  await page.goto('/recipes/new');
  await page.fill('input[name="title"]', title);
  await page.check('input[name="categories"][value="Mittagessen"]');
  await page.click('button[type="submit"]');
  await page.waitForURL(/\/recipes\/\d+$/);
  const url = page.url();
  const id = url.match(/\/recipes\/(\d+)$/)?.[1];
  return id!;
}

/**
 * Gibt den Titel-Input ein und wartet auf HTMX-Debounce + Antwort.
 * Verwendet pressSequentially() für echte Tastaturevents, die HTMX triggern.
 *
 * Das Template nutzt hx-sync="this:replace", wodurch HTMX ältere laufende
 * Requests abbricht und nur die letzte Antwort in den DOM injiziert.
 *
 * Wartet explizit länger als der HTMX-Debounce (400ms) damit der Request
 * abgesendet wird und die Antwort im DOM landet.
 */
async function typeTitle(page: import('@playwright/test').Page, title: string): Promise<void> {
  const input = page.locator('input[name="title"]');
  // Triple-click markiert den gesamten Text, pressSequentially überschreibt ihn.
  await input.click({ clickCount: 3 });
  await input.pressSequentially(title);
  // Mindestens Debounce (400ms) + Netzwerk-Puffer warten.
  // Die nachfolgenden Assertions (toContainText etc.) haben eigene Timeouts
  // und warten auf den stabilen DOM-Zustand.
  await page.waitForTimeout(600);
}

test.describe('Duplikaterkennung bei Titeleingabe', () => {
  test('K1: Duplikat-Hinweis erscheint bei ähnlichem Titel', async ({ page }) => {
    // Given: Ein eindeutiges Rezept existiert
    // Strategie: rezeptTitel = "Dinkel<ts>brot", suchbegriff = "<ts>" (Substring)
    const ts = uid();
    const rezeptTitel = `Dinkel${ts}brot`;
    const suchbegriff = String(ts);
    await createRecipe(page, rezeptTitel);

    // And: Formular für neues Rezept geöffnet
    await page.goto('/recipes/new');

    // When: Benutzer gibt den Timestamp-basierten Suchbegriff ein
    await typeTitle(page, suchbegriff);

    // Then: Hinweis mit "Ähnliche Rezepte gefunden" erscheint
    await expect(page.locator('#duplicate-hint')).toContainText('Ähnliche Rezepte gefunden');

    // And: Rezept ist in der Hinweisliste sichtbar
    await expect(page.locator('#duplicate-hint')).toContainText(rezeptTitel);

    // And: Link zur Detailansicht ist vorhanden
    await expect(page.locator('#duplicate-hint a')).toBeVisible();
  });

  test('K3: Hinweis verschwindet bei keiner Übereinstimmung', async ({ page }) => {
    // Given: Ein eindeutiges Rezept existiert
    const ts = uid();
    const rezeptTitel = `Dinkel${ts}brot`;
    const suchbegriff = String(ts);
    await createRecipe(page, rezeptTitel);
    await page.goto('/recipes/new');

    // Given: Hinweis ist sichtbar
    await typeTitle(page, suchbegriff);
    await expect(page.locator('#duplicate-hint')).toContainText(rezeptTitel);

    // When: Benutzer ändert Titel auf etwas völlig anderes (ohne Timestamp)
    await typeTitle(page, 'Spaghetti Bolognese');

    // Then: Duplikat-Hinweis zeigt das Dinkel-Rezept nicht mehr
    await expect(page.locator('#duplicate-hint')).not.toContainText(rezeptTitel);
  });

  test('K1 Edge: Kein Hinweis bei kurzem Titel (< 3 Zeichen)', async ({ page }) => {
    // Given: Rezept existiert
    const ts = uid();
    await createRecipe(page, `Dinkel${ts}brot`);
    await page.goto('/recipes/new');

    // When: Benutzer gibt "Di" ins Titelfeld ein + wartet
    await typeTitle(page, 'Di');

    // Then: Kein Hinweis erscheint
    await expect(page.locator('#duplicate-hint')).not.toContainText('Ähnliche Rezepte gefunden');
  });

  test('K4: Aktuelles Rezept nicht als Duplikat beim Bearbeiten', async ({ page }) => {
    // Given: Ein eindeutiges Rezept existiert
    const ts = uid();
    const rezeptTitel = `Dinkel${ts}brot`;
    const id = await createRecipe(page, rezeptTitel);

    // And: Bearbeitungsformular geöffnet
    await page.goto(`/recipes/${id}/edit`);
    await expect(page).toHaveURL(`/recipes/${id}/edit`);

    // When: Benutzer tippt nochmal den gleichen Titel ein + wartet
    await typeTitle(page, rezeptTitel);

    // Then: Rezept erscheint NICHT im Duplikat-Hinweis (eigenes Rezept ausgeschlossen)
    await expect(page.locator('#duplicate-hint')).not.toContainText(rezeptTitel);
  });

  test('K5: Speichern trotz Hinweis möglich', async ({ page }) => {
    // Given: Ein eindeutiges Rezept existiert
    const ts = uid();
    const rezeptTitel = `Dinkel${ts}brot`;
    const neuerTitel = `Dinkel${ts}kuchen`;
    const suchbegriff = String(ts);
    await createRecipe(page, rezeptTitel);

    // And: Neues Formular mit ähnlichem Titel → Hinweis sichtbar
    await page.goto('/recipes/new');
    await typeTitle(page, suchbegriff);
    await expect(page.locator('#duplicate-hint')).toContainText('Ähnliche Rezepte gefunden');

    // Titel auf neuen, eindeutigen Wert setzen
    const input = page.locator('input[name="title"]');
    await input.clear();
    await input.fill(neuerTitel);

    // When: Benutzer füllt Formular aus und speichert
    await page.check('input[name="categories"][value="Mittagessen"]');
    await page.click('button[type="submit"]');

    // Then: Neues Rezept wird angelegt, Weiterleitung zur Detailansicht
    await expect(page).toHaveURL(/\/recipes\/\d+/);
    await expect(page.locator('h1')).toContainText(neuerTitel);
  });

  test('K2: Jeder Kandidat enthält Link zur Detailansicht', async ({ page }) => {
    // Given: Ein eindeutiges Rezept existiert
    const ts = uid();
    const rezeptTitel = `Dinkel${ts}brot`;
    const suchbegriff = String(ts);
    const id = await createRecipe(page, rezeptTitel);
    await page.goto('/recipes/new');

    // Given: Hinweis ist sichtbar
    await typeTitle(page, suchbegriff);
    await expect(page.locator('#duplicate-hint')).toContainText(rezeptTitel);

    // Then: Link href="/recipes/<id>" ist vorhanden
    const link = page.locator(`#duplicate-hint a[href="/recipes/${id}"]`);
    await expect(link).toBeVisible();

    // And: Link führt zur Detailseite
    await link.click();
    await expect(page).toHaveURL(`/recipes/${id}`);
    await expect(page.locator('h1')).toContainText(rezeptTitel);
  });

  test('K6: Ähnlichkeitssuche ist case-insensitiv', async ({ page }) => {
    // Given: Ein Rezept in Kleinbuchstaben existiert
    const ts = uid();
    const rezeptTitel = `dinkel${ts}brot`;
    const suchbegriffGross = String(ts).toUpperCase();
    // Timestamps bestehen nur aus Ziffern, daher ist toUpperCase() gleichwertig
    // Stattdessen: Buchstaben-Teil in Großbuchstaben testen
    const rezeptTitel2 = `dinkelbrot${ts}`;
    const suchbegriffMixed = `DINKELBROT${ts}`.substring(0, 10 + String(ts).length);
    await createRecipe(page, rezeptTitel2);
    await page.goto('/recipes/new');

    // When: Benutzer gibt Suchbegriff in Großbuchstaben ein + wartet
    await typeTitle(page, `DINKELBROT${ts}`);

    // Then: Hinweis mit dem Rezept erscheint (case-insensitiv)
    await expect(page.locator('#duplicate-hint')).toContainText(rezeptTitel2);
  });
});
