import { test, expect } from '@playwright/test';

test.describe('Rezept löschen', () => {
  let recipeUrl: string;
  let uniqueTitle: string;

  test.beforeEach(async ({ page }) => {
    // Given: Ein Testrezept wird erstellt, das gelöscht werden kann
    uniqueTitle = `Löschtest ${Date.now()}`;
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
    // Given: Ein Testrezept existiert bereits, Detailseite ist aktiv
    // When: Die Detailseite geladen wird
    // Then: Der Löschen-Button ist sichtbar
    await expect(page.locator('a.btn-danger')).toBeVisible();
    await expect(page.locator('a.btn-danger')).toContainText('Löschen');
  });

  test('sollte Bestätigungsseite anzeigen', async ({ page }) => {
    // Given: Ein Testrezept existiert bereits, Detailseite ist aktiv
    // When: Der Löschen-Button angeklickt wird
    await page.click('a.btn-danger');

    // Then: Bestätigungsseite mit Rezepttitel, Abbrechen und Löschen-Button erscheint
    await expect(page).toHaveURL(/\/confirm-delete/);
    await expect(page.locator('.confirm-question')).toContainText(uniqueTitle);
    await expect(page.locator('a.btn-primary')).toContainText('Abbrechen');
    await expect(page.locator('button.btn-danger')).toContainText('Wirklich löschen');
    await expect(page.locator('.tip-box')).toBeVisible();
  });

  test('sollte Abbrechen zur Detailseite zurückkehren', async ({ page }) => {
    // Given: Ein Testrezept existiert bereits, Bestätigungsseite ist geöffnet
    await page.click('a.btn-danger');
    await expect(page).toHaveURL(/\/confirm-delete/);

    // When: Abbrechen angeklickt wird
    await page.click('a.btn-primary');

    // Then: Zurück zur Detailseite, Rezept unverändert vorhanden
    await expect(page).toHaveURL(recipeUrl);
    await expect(page.locator('h1')).toContainText(uniqueTitle);
  });

  test('sollte Rezept erfolgreich löschen', async ({ page }) => {
    // Given: Ein Testrezept existiert bereits, Bestätigungsseite ist geöffnet
    await page.click('a.btn-danger');
    await expect(page).toHaveURL(/\/confirm-delete/);

    // When: "Wirklich löschen" angeklickt wird
    await page.click('button.btn-danger');

    // Then: Weiterleitung zur Startseite mit Erfolgsmeldung, Rezept nicht mehr in der Liste
    await expect(page).toHaveURL(/\/\?deleted=/);
    await expect(page.locator('.success')).toContainText(uniqueTitle);
    await expect(page.locator('.success')).toContainText('wurde gelöscht');
    await expect(page.locator('.recipe-list')).not.toContainText(uniqueTitle);
  });

  test('sollte 404 bei nicht-existentem Rezept anzeigen', async ({ page }) => {
    // Given: Eine nicht existierende Rezept-ID
    // When: /recipes/99999/confirm-delete aufgerufen wird
    await page.goto('/recipes/99999/confirm-delete');

    // Then: Fehlermeldung "Rezept mit ID 99999 nicht gefunden"
    await expect(page.locator('body')).toContainText('Rezept mit ID 99999 nicht gefunden');
  });
});
