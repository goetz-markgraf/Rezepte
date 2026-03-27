import { test, expect } from '@playwright/test';

test.describe('Rezept löschen', () => {
  let recipeUrl: string;
  let uniqueTitle: string;

  test.beforeEach(async ({ page }) => {
    uniqueTitle = `Löschtest ${Date.now()}`;
    // Ein Rezept erstellen, das wir löschen können
    await page.goto('/recipes/new');
    await page.fill('input[name="title"]', uniqueTitle);
    await page.check('input[name="categories"][value="Mittagessen"]');
    await page.fill('textarea[name="ingredients"]', 'Test Zutaten');
    await page.fill('textarea[name="instructions"]', 'Test Anleitung');
    await page.click('button[type="submit"]');
    await expect(page).toHaveURL(/\/recipes\/\d+/);
    recipeUrl = page.url();
  });

  test('sollte Lösch-Button auf Detailansicht anzeigen', async ({ page }) => {
    await expect(page.locator('a.btn-danger')).toBeVisible();
    await expect(page.locator('a.btn-danger')).toContainText('Löschen');
  });

  test('sollte Bestätigungsseite anzeigen', async ({ page }) => {
    await page.click('a.btn-danger');

    await expect(page).toHaveURL(/\/confirm-delete/);
    await expect(page.locator('.confirm-question')).toContainText(uniqueTitle);
    await expect(page.locator('a.btn-primary')).toContainText('Abbrechen');
    await expect(page.locator('button.btn-danger')).toContainText('Wirklich löschen');
    await expect(page.locator('.tip-box')).toBeVisible();
  });

  test('sollte Abbrechen zur Detailseite zurückkehren', async ({ page }) => {
    await page.click('a.btn-danger');
    await expect(page).toHaveURL(/\/confirm-delete/);

    await page.click('a.btn-primary');

    await expect(page).toHaveURL(recipeUrl);
    await expect(page.locator('h1')).toContainText(uniqueTitle);
  });

  test('sollte Rezept erfolgreich löschen', async ({ page }) => {
    await page.click('a.btn-danger');
    await expect(page).toHaveURL(/\/confirm-delete/);

    await page.click('button.btn-danger');

    // Redirect zur Übersicht
    await expect(page).toHaveURL(/\/\?deleted=/);
    await expect(page.locator('.success')).toContainText(uniqueTitle);
    await expect(page.locator('.success')).toContainText('wurde gelöscht');

    // Rezept soll nicht mehr in der Liste sein (nur in der recipe-list prüfen, nicht im success-Banner)
    await expect(page.locator('.recipe-list')).not.toContainText(uniqueTitle);
  });

  test('sollte 404 bei nicht-existentem Rezept anzeigen', async ({ page }) => {
    await page.goto('/recipes/99999/confirm-delete');
    await expect(page.locator('body')).toContainText('Rezept mit ID 99999 nicht gefunden');
  });
});
