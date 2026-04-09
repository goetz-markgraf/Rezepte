import { test, expect } from '@playwright/test';

/**
 * E2E-Tests für Story 37: Einklappen der Filter
 * E2E-Tests für Story 40: Filter standardmäßig eingeklappt
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

  test('Story 40 K1: Filter standardmäßig eingeklappt beim ersten Aufruf', async ({ page }) => {
    // Gegeben: Startseite wird ohne URL-Parameter aufgerufen
    await page.goto('/');

    // Dann: Filterbereich ist eingeklappt (nicht sichtbar)
    const filterPanel = page.locator('#filter-panel');
    await expect(filterPanel).not.toBeVisible();

    // Und: Suchformular ist weiterhin sichtbar
    const searchForm = page.locator('.search-form');
    await expect(searchForm).toBeVisible();

    // Und: Toggle-Button zeigt "Filter anzeigen ▶"
    const toggleBtn = page.locator('.filter-toggle-btn');
    await expect(toggleBtn).toBeVisible();
    await expect(toggleBtn).toContainText('▶');

    // Und: URL enthält keinen filter_collapsed Parameter
    await expect(page).not.toHaveURL(/filter_collapsed/);
  });

  test('Story 40 K2: Filter ausklappen - URL enthält filter_collapsed=0', async ({ page }) => {
    // Gegeben: Startseite ist geöffnet, Filter sind eingeklappt
    await page.goto('/');
    const filterPanel = page.locator('#filter-panel');
    await expect(filterPanel).not.toBeVisible();

    // Wenn: Nutzer auf "Filter anzeigen" klickt
    const toggleBtn = page.locator('.filter-toggle-btn');
    await toggleBtn.click();

    // Dann: Filterbereich ist sichtbar
    await expect(filterPanel).toBeVisible();

    // Und: URL enthält filter_collapsed=0
    await expect(page).toHaveURL(/filter_collapsed=0/);

    // Und: Toggle-Button zeigt "▼"
    await expect(toggleBtn).toContainText('▼');
  });

  test('Story 40 K3: Ausgeklappter Zustand bleibt nach Reload erhalten', async ({ page }) => {
    // Gegeben: Filter sind ausgeklappt (filter_collapsed=0)
    await page.goto('/?filter_collapsed=0');
    const filterPanel = page.locator('#filter-panel');
    await expect(filterPanel).toBeVisible();

    // Wenn: Seite neu geladen wird
    await page.reload();

    // Dann: Filter bleiben ausgeklappt
    await expect(filterPanel).toBeVisible();
  });

  test('Story 40 K4: Filter einklappen - URL ohne Parameter', async ({ page }) => {
    // Gegeben: Filter sind ausgeklappt
    await page.goto('/?filter_collapsed=0');
    const filterPanel = page.locator('#filter-panel');
    await expect(filterPanel).toBeVisible();

    // Wenn: Nutzer auf "Filter ausblenden" klickt
    const toggleBtn = page.locator('.filter-toggle-btn');
    await toggleBtn.click();

    // Dann: Filterbereich ist nicht mehr sichtbar
    await expect(filterPanel).not.toBeVisible();

    // Und: URL enthält keinen filter_collapsed Parameter
    await expect(page).not.toHaveURL(/filter_collapsed/);
  });

  test('K2/K4: Filter einklappen - filter-panel verschwindet, Suchleiste bleibt sichtbar', async ({ page }) => {
    // Gegeben: Startseite ist geöffnet mit ausgeklappten Filtern
    await page.goto('/?filter_collapsed=0');
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

    // Und: URL enthält keinen filter_collapsed Parameter (Standard: eingeklappt)
    await expect(page).not.toHaveURL(/filter_collapsed/);

    // Und: Suchformular ist weiterhin sichtbar
    await expect(searchForm).toBeVisible();
  });

  test('K3/K4: Filter ausklappen - filter-panel wird wieder sichtbar', async ({ page }) => {
    // Gegeben: Startseite ist geöffnet (Filter standardmäßig eingeklappt)
    await page.goto('/');

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

    // Und: URL enthält filter_collapsed=0-Parameter
    await expect(page).toHaveURL(/filter_collapsed=0/);
  });

  test('K5: Eingeklappter Zustand direkt via URL beim Seitenaufruf', async ({ page }) => {
    // Gegeben: URL ohne Parameter wird direkt aufgerufen (Story 40: Default ist eingeklappt)
    await page.goto('/');

    // Dann: Filterbereich ist von Anfang an ausgeblendet
    const filterPanel = page.locator('#filter-panel');
    await expect(filterPanel).not.toBeVisible();

    // Und: Toggle-Button zeigt "Filter ▶"
    const toggleBtn = page.locator('.filter-toggle-btn');
    await expect(toggleBtn).toContainText('▶');
  });

  test('K6: Aktive Filter sichtbar bei eingeklapptem Zustand', async ({ page }) => {
    // Gegeben: Kategorie "Brot" ist aktiv, Seite ohne Parameter aufgerufen (Story 40: Default eingeklappt)
    await page.goto('/?kategorie=Brot');

    // Dann: Toggle-Button zeigt Hinweis "(aktiv)"
    const toggleBtn = page.locator('.filter-toggle-btn');
    await expect(toggleBtn).toContainText('(aktiv)');

    // Und: filter-active-indicator ist sichtbar
    const indicator = page.locator('.filter-active-indicator');
    await expect(indicator).toBeVisible();
  });

  test('Story 40 K5: Gespeicherte Filter öffnen eingeklappt (Default)', async ({ page }) => {
    // Gegeben: Ein Rezept und ein gespeicherter Filter existieren
    const suffix = `fc7-${Date.now()}-${Math.random().toString(36).slice(2, 7)}`;
    await createRecipe(page, `Brot-${suffix}`, ['Brot']);

    // Gespeicherten Filter anlegen (mit ausgeklappten Filtern, um den Button zu sehen)
    await page.goto('/?kategorie=Brot&filter_collapsed=0');
    await page.fill('#save-filter-name', `BrotFilter-${suffix}`);
    await page.click('.save-filter-submit');

    // Warte bis die Seite neu geladen ist
    await page.waitForLoadState('networkidle');

    // Panel aufklappen, damit der gespeicherte Filter-Button sichtbar wird
    const toggleBtn = page.locator('.filter-toggle-btn');
    await toggleBtn.click();
    await page.waitForLoadState('networkidle');

    // Wenn: Nutzer klickt auf gespeicherten Filter
    const savedFilterBtn = page.locator('.saved-filter-btn', { hasText: `BrotFilter-${suffix}` });
    await savedFilterBtn.waitFor({ state: 'visible' });
    await savedFilterBtn.click();

    // Dann: URL enthält keinen filter_collapsed-Parameter
    await expect(page).not.toHaveURL(/filter_collapsed/);

    // Und: Filterbereich ist eingeklappt (Story 40: Default ist eingeklappt)
    const filterPanel = page.locator('#filter-panel');
    await expect(filterPanel).not.toBeVisible();
  });

  test('K8: Funktioniert ohne JavaScript', async ({ browser }) => {
    // Gegeben: JavaScript ist deaktiviert
    const context = await browser.newContext({ javaScriptEnabled: false });
    const page = await context.newPage();

    // Wenn: Seite ohne JS aufgerufen wird und Toggle-Link geklickt wird
    await page.goto('/');
    const filterPanel = page.locator('#filter-panel');
    // Story 40: Filter sind standardmäßig eingeklappt
    await expect(filterPanel).not.toBeVisible();

    // Toggle klicken (normaler Link, kein JS nötig) - Filter ausklappen
    const toggleBtn = page.locator('.filter-toggle-btn');
    await toggleBtn.click();

    // Dann: Seite lädt neu mit korrektem filter_collapsed=0-Parameter
    await expect(page).toHaveURL(/filter_collapsed=0/);
    await expect(filterPanel).toBeVisible();

    await context.close();
  });

  test('Story 40 K1b: Toggle-Button zeigt bei Start "▶" (eingeklappt)', async ({ page }) => {
    // Gegeben: Startseite ist geöffnet (ohne Parameter = Story 40: eingeklappt)
    await page.goto('/');

    // Dann: Toggle-Button ist sichtbar und zeigt "Filter ▶"
    const toggleBtn = page.locator('.filter-toggle-btn');
    await expect(toggleBtn).toBeVisible();
    await expect(toggleBtn).toContainText('▶');

    // Und: aria-expanded ist "false" (eingeklappt)
    await expect(toggleBtn).toHaveAttribute('aria-expanded', 'false');
    await expect(toggleBtn).toHaveAttribute('aria-controls', 'filter-panel');
  });

  test('K1: Toggle-Button immer sichtbar (ausgeklappt)', async ({ page }) => {
    // Gegeben: Startseite ist geöffnet mit ausgeklappten Filtern
    await page.goto('/?filter_collapsed=0');

    // Dann: Toggle-Button ist sichtbar und zeigt "Filter ▼"
    const toggleBtn = page.locator('.filter-toggle-btn');
    await expect(toggleBtn).toBeVisible();
    await expect(toggleBtn).toContainText('▼');

    // Und: aria-expanded ist "true"
    await expect(toggleBtn).toHaveAttribute('aria-expanded', 'true');
    await expect(toggleBtn).toHaveAttribute('aria-controls', 'filter-panel');
  });

  test('K9: Barrierefreiheit - aria-expanded korrekt gesetzt', async ({ page }) => {
    // Gegeben: Startseite ohne Parameter (Story 40: Default eingeklappt)
    await page.goto('/');

    // Dann: aria-expanded ist "false"
    const toggleBtn = page.locator('.filter-toggle-btn');
    await expect(toggleBtn).toHaveAttribute('aria-expanded', 'false');

    // Und: filter-panel hat aria-hidden="true"
    const filterPanel = page.locator('#filter-panel');
    await expect(filterPanel).toHaveAttribute('aria-hidden', 'true');
  });

  test('Story 40 K10: "Länger nicht gemacht"-Button wurde aus Wochenübersicht entfernt', async ({ page }) => {
    // Gegeben: Wochenübersicht ist geöffnet
    await page.goto('/wochenvorschau');

    // Dann: Der "Länger nicht gemacht"-Button sollte nicht mehr sichtbar sein
    const notMadeBtn = page.locator('.not-made-button');
    await expect(notMadeBtn).not.toBeVisible();
  });

  test('Story 40 K10: "Zur Rezeptliste"-Link von Wochenübersicht - Filter eingeklappt (Default)', async ({ page }) => {
    // Gegeben: Wochenübersicht ist geöffnet
    await page.goto('/wochenvorschau');

    // Wenn: Nutzer klickt auf "Zur Rezeptliste"-Link
    const rezeptlisteLink = page.locator('a', { hasText: 'Zur Rezeptliste' });
    await expect(rezeptlisteLink).toBeVisible();
    await rezeptlisteLink.click();

    // Dann: Landet auf der Rezeptliste (nicht mehr auf /wochenvorschau)
    await expect(page).not.toHaveURL(/wochenvorschau/);

    // Und: URL enthält keinen filter_collapsed Parameter (Story 40: Default eingeklappt)
    await expect(page).not.toHaveURL(/filter_collapsed/);

    // Und: Filter sind eingeklappt (Story 40: Default)
    const filterPanel = page.locator('#filter-panel');
    await expect(filterPanel).not.toBeVisible();
  });

});
