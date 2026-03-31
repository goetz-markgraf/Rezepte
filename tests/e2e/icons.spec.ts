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
    // Given: Ein Rezept wurde erstellt
    const title = `Icon-Test-Liste-${Date.now()}`;
    await createRecipe(page, title);

    // When: Die Rezeptliste aufgerufen wird
    await page.goto('/');

    // Then: Bearbeiten-Link hat aria-label mit Rezepttitel und enthält ein SVG-Icon
    // aria-label ist jetzt "<Titel> bearbeiten" statt "Rezept bearbeiten" (Story 25, L4)
    const editLink = page.locator(`a[aria-label="${title} bearbeiten"]`).first();
    await expect(editLink).toBeVisible();
    await expect(editLink.locator('svg')).toBeVisible();
  });

  test('"Neues Rezept"-Link ist in der Navigation sichtbar (K1)', async ({ page }) => {
    // Given: Die Startseite ist geöffnet
    // When: Die Startseite aufgerufen wird
    await page.goto('/');

    // Then: Der "Neues Rezept"-Link ist in der Navigation sichtbar
    const newRecipeLink = page.locator('a[href="/recipes/new"]').first();
    await expect(newRecipeLink).toBeVisible();
    await expect(newRecipeLink).toContainText('Neues Rezept');
  });

  test('Löschen-Button auf Detailseite hat SVG und navigiert zur Bestätigungsseite (K2)', async ({ page }) => {
    // Given: Ein Rezept wurde erstellt, Detailseite ist aktiv
    const title = `Icon-Test-Detail-${Date.now()}`;
    await createRecipe(page, title);

    // When: Der Löschen-Button gesucht und geklickt wird
    const deleteLink = page.locator('a.btn-danger');
    await expect(deleteLink).toBeVisible();
    await expect(deleteLink.locator('svg')).toBeVisible();
    await deleteLink.click();

    // Then: Navigation zur Bestätigungsseite erfolgt
    await expect(page).toHaveURL(/\/confirm-delete/);
  });

  test('Bestätigungs-Dialog-Buttons haben Icons (K4)', async ({ page }) => {
    // Given: Die Lösch-Bestätigungsseite eines Rezepts ist geöffnet
    const title = `Icon-Test-Confirm-${Date.now()}`;
    const url = await createRecipe(page, title);
    const id = url.split('/').pop();

    // When: Die Bestätigungsseite aufgerufen wird
    await page.goto(`/recipes/${id}/confirm-delete`);

    // Then: Abbrechen- und Löschen-Button sind sichtbar und enthalten jeweils ein SVG
    const cancelLink = page.locator('a.btn-primary:has-text("Abbrechen")');
    await expect(cancelLink).toBeVisible();
    await expect(cancelLink.locator('svg')).toBeVisible();

    const deleteButton = page.locator('button.btn-danger');
    await expect(deleteButton).toBeVisible();
    await expect(deleteButton.locator('svg')).toBeVisible();
  });

  test('Icon-Buttons sind per Tastatur erreichbar (K8)', async ({ page }) => {
    // Given: Ein Rezept wurde erstellt
    const title = `Icon-Test-Keyboard-${Date.now()}`;
    await createRecipe(page, title);

    // When: Die Rezeptliste aufgerufen und der Bearbeiten-Link fokussiert wird
    await page.goto('/');
    // aria-label ist jetzt "<Titel> bearbeiten" statt "Rezept bearbeiten" (Story 25, L4)
    const editLink = page.locator(`a[aria-label="${title} bearbeiten"]`).first();
    await expect(editLink).toBeVisible();
    await editLink.focus();

    // Then: Der Link ist tatsächlich fokussiert (Tastaturnavigation funktioniert)
    await expect(editLink).toBeFocused();
  });
});
