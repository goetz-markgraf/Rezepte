import { test, expect } from '@playwright/test';

test.describe('Responsive Layout', () => {
  async function createRecipe(
    page: import('@playwright/test').Page,
    title: string,
    category: string
  ): Promise<string> {
    await page.goto('/recipes/new');
    await page.fill('input[name="title"]', title);
    await page.check(`input[name="categories"][value="${category}"]`);
    await page.click('button[type="submit"]');
    await expect(page).toHaveURL(/\/recipes\/\d+/);
    return page.url();
  }

  test.skip('Test 1: Rezept-Liste auf Mobile – kein horizontales Scrollen, Listeneinträge ≥44px hoch', async ({ page }) => {
    // Given: Mobile Viewport (iPhone 14)
    await page.setViewportSize({ width: 390, height: 844 });

    // Sicherstellen, dass mindestens ein Rezept vorhanden ist
    await createRecipe(page, `Mobile-Test-${Date.now()}`, 'Mittagessen');

    // When: Startseite aufrufen
    await page.goto('/');

    // Then: Kein horizontales Scrollen
    const scrollWidth = await page.evaluate(() => document.body.scrollWidth);
    const innerWidth = await page.evaluate(() => window.innerWidth);
    expect(scrollWidth).toBeLessThanOrEqual(innerWidth);

    // Then: Listeneintrag mindestens 44px hoch
    const recipeLink = page.locator('.recipe-item-link').first();
    await expect(recipeLink).toBeVisible();
    const box = await recipeLink.boundingBox();
    expect(box).not.toBeNull();
    expect(box!.height).toBeGreaterThanOrEqual(44);
  });

  test('Test 2: Rezept-Liste auf Desktop – main nicht über volle Breite, zentriert', async ({ page }) => {
    // Given: Desktop Viewport
    await page.setViewportSize({ width: 1280, height: 800 });

    // When: Startseite aufrufen
    await page.goto('/');

    // Then: main-Element nicht über volle Bildschirmbreite
    const mainBox = await page.locator('main').boundingBox();
    expect(mainBox).not.toBeNull();
    expect(mainBox!.width).toBeLessThan(1280);

    // Then: main-Element ist horizontal zentriert (linker Rand > 0)
    expect(mainBox!.x).toBeGreaterThan(0);
  });

  test('Test 3: Formular auf Mobile – kein horizontales Scrollen, Felder ≥44px, Speichern-Button ≥44px hoch', async ({ page }) => {
    // Given: Mobile Viewport
    await page.setViewportSize({ width: 390, height: 844 });

    // When: Neues Rezept Formular aufrufen
    await page.goto('/recipes/new');

    // Then: Kein horizontales Scrollen
    const scrollWidth = await page.evaluate(() => document.body.scrollWidth);
    const innerWidth = await page.evaluate(() => window.innerWidth);
    expect(scrollWidth).toBeLessThanOrEqual(innerWidth);

    // Then: Titel-Input mindestens 44px breit
    const titleInput = page.locator('input[name="title"]');
    await expect(titleInput).toBeVisible();
    const inputBox = await titleInput.boundingBox();
    expect(inputBox).not.toBeNull();
    expect(inputBox!.width).toBeGreaterThanOrEqual(44);

    // Then: Speichern-Button sichtbar und mindestens 44px hoch
    const submitBtn = page.locator('button[type="submit"]');
    await expect(submitBtn).toBeVisible();
    const btnBox = await submitBtn.boundingBox();
    expect(btnBox).not.toBeNull();
    expect(btnBox!.height).toBeGreaterThanOrEqual(44);
  });

  test('Test 4: Navigation auf Mobile – Header vollständig sichtbar, site-title-Link korrekt', async ({ page }) => {
    // Given: Mobile Viewport
    await page.setViewportSize({ width: 390, height: 844 });

    // When: Startseite aufrufen
    await page.goto('/');

    // Then: Header vollständig sichtbar (kein Clipping)
    const headerBox = await page.locator('header').boundingBox();
    expect(headerBox).not.toBeNull();
    expect(headerBox!.x).toBeGreaterThanOrEqual(0);
    expect(headerBox!.width).toBeLessThanOrEqual(390);

    // Then: site-title-Link sichtbar mit korrektem href="/"
    const siteTitle = page.locator('.site-title');
    await expect(siteTitle).toBeVisible();
    const href = await siteTitle.getAttribute('href');
    expect(href).toBe('/');
  });

  test('Test 5: Rezept-Detailansicht auf Mobile – kein horizontales Scrollen, Aktionsbuttons ≥44px hoch', async ({ page }) => {
    // Given: Mobile Viewport und ein vorhandenes Rezept
    await page.setViewportSize({ width: 390, height: 844 });
    const detailUrl = await createRecipe(page, `Detail-Mobile-Test-${Date.now()}`, 'Snacks');
    const id = detailUrl.split('/').pop();

    // When: Detailseite mit Mobile Viewport aufrufen
    await page.goto(`/recipes/${id}`);

    // Then: Kein horizontales Scrollen
    const scrollWidth = await page.evaluate(() => document.body.scrollWidth);
    const innerWidth = await page.evaluate(() => window.innerWidth);
    expect(scrollWidth).toBeLessThanOrEqual(innerWidth);

    // Then: Aktionsbuttons (Bearbeiten, Zurück, Löschen) mindestens 44px hoch
    const editBtn = page.locator('.actions a.btn-primary').first();
    await expect(editBtn).toBeVisible();
    const editBox = await editBtn.boundingBox();
    expect(editBox).not.toBeNull();
    expect(editBox!.height).toBeGreaterThanOrEqual(44);

    const backBtn = page.locator('.actions a.btn-secondary').first();
    await expect(backBtn).toBeVisible();
    const backBox = await backBtn.boundingBox();
    expect(backBox).not.toBeNull();
    expect(backBox!.height).toBeGreaterThanOrEqual(44);

    const deleteBtn = page.locator('.actions a.btn-danger').first();
    await expect(deleteBtn).toBeVisible();
    const deleteBox = await deleteBtn.boundingBox();
    expect(deleteBox).not.toBeNull();
    expect(deleteBox!.height).toBeGreaterThanOrEqual(44);
  });
});
