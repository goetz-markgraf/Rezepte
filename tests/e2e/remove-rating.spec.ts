import { test, expect } from '@playwright/test';

async function createRecipe(
  page: import('@playwright/test').Page,
  title: string,
  category: string,
  plannedDate?: string
): Promise<string> {
  await page.goto('/recipes/new');
  await page.fill('input[name="title"]', title);
  await page.check(`input[name="categories"][value="${category}"]`);
  if (plannedDate) {
    await page.fill('input[name="planned_date"]', plannedDate);
  }
  await page.click('button[type="submit"]');
  await expect(page).toHaveURL(/\/recipes\/\d+/);
  return page.url();
}

test.describe('Bewertungsmechanismus entfernt (Story 44)', () => {

  test('K1: Startseite zeigt keine Bewertungsfilter-Buttons', async ({ page }) => {
    await page.goto('/?filter_collapsed=0');

    await expect(page.locator('a.sort-filter-btn', { hasText: '★★★+ Nur Gute' })).not.toBeVisible();
    await expect(page.locator('a.sort-filter-btn', { hasText: '★★★★★ Favoriten' })).not.toBeVisible();
  });

  test('K2: Rezept-Liste zeigt keine Sterne', async ({ page }) => {
    const suffix = Date.now();
    await createRecipe(page, `Sterne-Test-${suffix}`, 'Party');

    await page.goto('/');
    const item = page.locator('.recipe-item h2', { hasText: `Sterne-Test-${suffix}` });
    await expect(item).toBeVisible();

    // Keine Sterne-Anzeige innerhalb des Rezept-Items
    const parent = item.locator('xpath=ancestor::li');
    await expect(parent.locator('.recipe-stars')).not.toBeAttached();
  });

  test('K3: Detailansicht zeigt kein Inline-Rating', async ({ page }) => {
    const suffix = Date.now();
    const detailUrl = await createRecipe(page, `Detail-Rating-${suffix}`, 'Mittagessen');

    await page.goto(detailUrl);
    await expect(page.locator('#inline-rating')).not.toBeAttached();
  });

  test('K4: Bearbeitungsformular hat kein Bewertungsfeld', async ({ page }) => {
    const suffix = Date.now();
    const detailUrl = await createRecipe(page, `Edit-Rating-${suffix}`, 'Kuchen');
    const id = detailUrl.split('/').pop();

    await page.goto(`/recipes/${id}/edit`);
    await expect(page.locator('fieldset.star-rating')).not.toBeAttached();
    await expect(page.locator('input[name="rating"]')).not.toBeAttached();
  });

  test('K5: Neues Rezept ohne Bewertung funktioniert', async ({ page }) => {
    const suffix = Date.now();
    await page.goto('/recipes/new');
    await page.fill('input[name="title"]', `Neues-Rezept-${suffix}`);
    await page.check('input[name="categories"][value="Snacks"]');
    await page.click('button[type="submit"]');

    await expect(page).toHaveURL(/\/recipes\/\d+/);
    await expect(page.locator('h1')).toContainText(`Neues-Rezept-${suffix}`);
  });

  test('K6: "Heute gekocht" zeigt keine Sterne', async ({ page }) => {
    const suffix = Date.now();
    const today = new Date();
    const day = today.getDate();
    const month = today.getMonth() + 1;
    const year = today.getFullYear();
    const dateStr = `${day}.${month}.${year}`;
    await createRecipe(page, `Heute-Test-${suffix}`, 'Mittagessen', dateStr);

    await page.goto('/heute');
    const item = page.locator('.heute-rezept-titel', { hasText: `Heute-Test-${suffix}` });
    await expect(item).toBeVisible();

    // Keine Inline-Rating-Buttons
    const parent = item.locator('xpath=ancestor::li');
    await expect(parent.locator('.inline-rating-btn')).not.toBeAttached();
  });

  test('K7: Dubletten/Merge zeigen keine Bewertungen', async ({ page }) => {
    const ts = Date.now();
    const titelA = `DubletteA${ts}`;
    const titelB = `DubletteA${ts}B`;
    await createRecipe(page, titelA, 'Mittagessen');
    await createRecipe(page, titelB, 'Mittagessen');

    await page.goto('/recipes/duplicates');
    const pair = page.locator('.duplicate-pair').filter({ hasText: titelA });
    await expect(pair).toBeVisible();

    // Keine Sterne in Dubletten-Karten
    await expect(pair.locator('.stars')).not.toBeAttached();

    // Merge-Seite öffnen
    const mergeLink = pair.locator('a[href*="/recipes/merge"]');
    const href = await mergeLink.getAttribute('href');
    await page.goto(href);

    // Kein Bewertungs-Block auf Merge-Seite
    await expect(page.locator('text=Bewertung')).not.toBeVisible();
    await expect(page.locator('input[name="rating_from"]')).not.toBeAttached();
  });

  test('K8: DeepLink ?bewertung=gut führt nicht zu Fehler', async ({ page }) => {
    await page.goto('/?bewertung=gut');
    await expect(page.locator('h1')).toContainText('Rezepte');
    await expect(page).toHaveURL('/?bewertung=gut');
  });

  test('K9: POST /recipes/:id/rating gibt 404 zurück', async ({ page }) => {
    const response = await page.request.post('/recipes/1/rating', {
      form: { rating: '5' },
    });
    expect(response.status()).toBe(404);
  });

  test('K10: POST /heute/recipes/:id/rating gibt 404 zurück', async ({ page }) => {
    const response = await page.request.post('/heute/recipes/1/rating', {
      form: { rating: '5' },
    });
    expect(response.status()).toBe(404);
  });

});
