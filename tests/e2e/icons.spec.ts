import { test, expect } from '@playwright/test';

test.describe('Icons in der UI', () => {
  async function createRecipe(
    page: import('@playwright/test').Page,
    title: string
  ): Promise<string> {
    await page.goto('/recipes/new');
    await page.fill('input[name="title"]', title);
    await page.check('input[name="categories"][value="Mittagessen"]');
    await page.click('button[type="submit"]');
    await expect(page).toHaveURL(/\/recipes\/\d+/);
    return page.url();
  }

  test('Bearbeiten-Link in Rezeptliste hat SVG und aria-label (K2)', async ({ page }) => {
    const title = `Icon-Test-Liste-${Date.now()}`;
    await createRecipe(page, title);

    await page.goto('/');

    // Bearbeiten-Link hat aria-label="Rezept bearbeiten"
    const editLink = page.locator('a[aria-label="Rezept bearbeiten"]').first();
    await expect(editLink).toBeVisible();

    // Bearbeiten-Link enthält ein SVG-Icon
    await expect(editLink.locator('svg')).toBeVisible();
  });

  test('"Neues Rezept"-Button hat Plus-SVG (K1)', async ({ page }) => {
    await page.goto('/');

    const newRecipeLink = page.locator('a[href="/recipes/new"]').first();
    await expect(newRecipeLink).toBeVisible();

    // Link enthält ein SVG-Icon
    await expect(newRecipeLink.locator('svg')).toBeVisible();
  });

  test('Löschen-Button auf Detailseite hat SVG und navigiert zur Bestätigungsseite (K3)', async ({ page }) => {
    const title = `Icon-Test-Detail-${Date.now()}`;
    await createRecipe(page, title);

    // Löschen-Link prüfen: enthält svg
    const deleteLink = page.locator('a.btn-danger');
    await expect(deleteLink).toBeVisible();
    await expect(deleteLink.locator('svg')).toBeVisible();

    // Klick navigiert zur Bestätigungsseite
    await deleteLink.click();
    await expect(page).toHaveURL(/\/confirm-delete/);
  });

  test('Bestätigungs-Dialog-Buttons haben Icons (K4)', async ({ page }) => {
    const title = `Icon-Test-Confirm-${Date.now()}`;
    const url = await createRecipe(page, title);
    const id = url.split('/').pop();

    await page.goto(`/recipes/${id}/confirm-delete`);

    // Abbrechen-Button enthält svg
    const cancelLink = page.locator('a.btn-primary');
    await expect(cancelLink).toBeVisible();
    await expect(cancelLink.locator('svg')).toBeVisible();

    // Löschen-Button enthält svg
    const deleteButton = page.locator('button.btn-danger');
    await expect(deleteButton).toBeVisible();
    await expect(deleteButton.locator('svg')).toBeVisible();
  });

  test('Icon-Buttons sind per Tastatur erreichbar (K5)', async ({ page }) => {
    const title = `Icon-Test-Keyboard-${Date.now()}`;
    await createRecipe(page, title);

    await page.goto('/');

    // Bearbeiten-Link mit aria-label ist im DOM vorhanden und erreichbar
    const editLink = page.locator('a[aria-label="Rezept bearbeiten"]').first();
    await expect(editLink).toBeVisible();

    // Focus auf den Link setzen
    await editLink.focus();
    await expect(editLink).toBeFocused();
  });
});
