import { test, expect } from '@playwright/test';

/**
 * E2E-Tests für Story 37: Einklappen der Filter
 */

async function createRecipe(
  page: import('@playwright/test').Page,
  title: string,
  categories: string[]
): Promise<void> {
  await page.goto('/recipes/new');
  await page.fill('input[name="title"]', title);
  for (const category of categories) {
    await page.check(`input[name="categories"][value="${category}"]`);
  }
  await page.click('button[type="submit"]');
  await expect(page).toHaveURL(/\/recipes\/\d+/);
}

test.describe('Filter-Einklappen (Story 37)', () => {

  test('K2/K4: Filter einklappen - filter-panel verschwindet, Suchleiste bleibt sichtbar', async ({ page }) => {
    // Gegeben: Startseite ist geöffnet, Filterbereich ist sichtbar
    await page.goto('/');
    const filterPanel = page.locator('#filter-panel');
    await expect(filterPanel).toBeVisible();

    // Und: Suchformular ist sichtbar
    const searchForm = page.locator('.search-form');
    await expect(searchForm).toBeVisible();

    // Wenn: Nutzer auf den Toggle-Button "Filter ▼" klickt
    const toggleBtn = page.locator('.filter-toggle-btn');
    await expect(toggleBtn).toBeVisible();
    await toggleBtn.click();

    // Dann: filter-panel ist nicht mehr sichtbar
    await expect(filterPanel).not.toBeVisible();

    // Und: URL enthält filter_collapsed=1
    await expect(page).toHaveURL(/filter_collapsed=1/);

    // Und: Suchformular ist weiterhin sichtbar
    await expect(searchForm).toBeVisible();
  });

  test('K3/K4: Filter ausklappen - filter-panel wird wieder sichtbar', async ({ page }) => {
    // Gegeben: Startseite mit ?filter_collapsed=1 aufgerufen
    await page.goto('/?filter_collapsed=1');

    // Dann: filter-panel ist ausgeblendet
    const filterPanel = page.locator('#filter-panel');
    await expect(filterPanel).not.toBeVisible();

    // Wenn: Nutzer klickt Toggle-Button "Filter ▶"
    const toggleBtn = page.locator('.filter-toggle-btn');
    await expect(toggleBtn).toBeVisible();
    await expect(toggleBtn).toContainText('▶');
    await toggleBtn.click();

    // Dann: filter-panel ist wieder sichtbar
    await expect(filterPanel).toBeVisible();

    // Und: URL enthält keinen filter_collapsed=1-Parameter mehr
    await expect(page).not.toHaveURL(/filter_collapsed=1/);
  });

  test('K5: Eingeklappter Zustand direkt via URL beim Seitenaufruf', async ({ page }) => {
    // Gegeben: URL mit ?filter_collapsed=1 wird direkt aufgerufen
    await page.goto('/?filter_collapsed=1');

    // Dann: Filterbereich ist von Anfang an ausgeblendet
    const filterPanel = page.locator('#filter-panel');
    await expect(filterPanel).not.toBeVisible();

    // Und: Toggle-Button zeigt "Filter ▶"
    const toggleBtn = page.locator('.filter-toggle-btn');
    await expect(toggleBtn).toContainText('▶');
  });

  test('K6: Aktive Filter sichtbar bei eingeklapptem Zustand', async ({ page }) => {
    // Gegeben: Kategorie "Brot" ist aktiv, Seite mit ?filter_collapsed=1 aufgerufen
    await page.goto('/?kategorie=Brot&filter_collapsed=1');

    // Dann: Toggle-Button zeigt Hinweis "(aktiv)"
    const toggleBtn = page.locator('.filter-toggle-btn');
    await expect(toggleBtn).toContainText('(aktiv)');

    // Und: filter-active-indicator ist sichtbar
    const indicator = page.locator('.filter-active-indicator');
    await expect(indicator).toBeVisible();
  });

  test('K7: Gespeicherte Filter öffnen ausgeklappt', async ({ page }) => {
    // Gegeben: Ein Rezept und ein gespeicherter Filter existieren
    const suffix = `fc7-${Date.now()}-${Math.random().toString(36).slice(2, 7)}`;
    await createRecipe(page, `Brot-${suffix}`, ['Brot']);

    // Gespeicherten Filter anlegen
    await page.goto('/?kategorie=Brot');
    await page.fill('#save-filter-name', `BrotFilter-${suffix}`);
    await page.click('.save-filter-submit');

    // Wenn: Nutzer klickt auf gespeicherten Filter
    const savedFilterBtn = page.locator('.saved-filter-btn', { hasText: `BrotFilter-${suffix}` });
    await expect(savedFilterBtn).toBeVisible();
    await savedFilterBtn.click();

    // Dann: URL enthält keinen filter_collapsed-Parameter
    await expect(page).not.toHaveURL(/filter_collapsed/);

    // Und: Filterbereich ist sichtbar (ausgeklappt)
    const filterPanel = page.locator('#filter-panel');
    await expect(filterPanel).toBeVisible();
  });

  test('K8: Funktioniert ohne JavaScript', async ({ browser }) => {
    // Gegeben: JavaScript ist deaktiviert
    const context = await browser.newContext({ javaScriptEnabled: false });
    const page = await context.newPage();

    // Wenn: Seite ohne JS aufgerufen wird und Toggle-Link geklickt wird
    await page.goto('/');
    const filterPanel = page.locator('#filter-panel');
    await expect(filterPanel).toBeVisible();

    // Toggle klicken (normaler Link, kein JS nötig)
    const toggleBtn = page.locator('.filter-toggle-btn');
    await toggleBtn.click();

    // Dann: Seite lädt neu mit korrektem filter_collapsed-Parameter
    await expect(page).toHaveURL(/filter_collapsed=1/);
    await expect(filterPanel).not.toBeVisible();

    await context.close();
  });

  test('K1: Toggle-Button immer sichtbar (ausgeklappt)', async ({ page }) => {
    // Gegeben: Startseite ist geöffnet
    await page.goto('/');

    // Dann: Toggle-Button ist sichtbar und zeigt "Filter ▼"
    const toggleBtn = page.locator('.filter-toggle-btn');
    await expect(toggleBtn).toBeVisible();
    await expect(toggleBtn).toContainText('▼');

    // Und: aria-expanded ist "true"
    await expect(toggleBtn).toHaveAttribute('aria-expanded', 'true');
    await expect(toggleBtn).toHaveAttribute('aria-controls', 'filter-panel');
  });

  test('K9: Barrierefreiheit - aria-expanded korrekt gesetzt', async ({ page }) => {
    // Gegeben: Startseite mit eingeklappten Filtern
    await page.goto('/?filter_collapsed=1');

    // Dann: aria-expanded ist "false"
    const toggleBtn = page.locator('.filter-toggle-btn');
    await expect(toggleBtn).toHaveAttribute('aria-expanded', 'false');

    // Und: filter-panel hat aria-hidden="true"
    const filterPanel = page.locator('#filter-panel');
    await expect(filterPanel).toHaveAttribute('aria-hidden', 'true');
  });

});
