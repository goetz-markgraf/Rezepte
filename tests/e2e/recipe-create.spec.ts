import { test, expect } from '@playwright/test';

test.describe('Rezept erstellen', () => {
  test('sollte ein neues Rezept erfolgreich erstellen', async ({ page }) => {
    await page.goto('/');
    await page.click('text=Neues Rezept');
    
    await expect(page).toHaveURL('/recipes/new');
    await expect(page.locator('h1')).toContainText('Neues Rezept');
    
    await page.fill('input[name="title"]', 'Spaghetti Carbonara');
    await page.check('input[name="categories"][value="Mittagessen"]');
    await page.fill('textarea[name="ingredients"]', 'Spaghetti\nEier\nSpeck\nParmesan');
    await page.fill('textarea[name="instructions"]', '1. Nudeln kochen\n2. Sauce zubereiten\n3. Mischen');
    
    await page.click('button[type="submit"]');
    
    // Nach dem Speichern auf Detailseite
    await expect(page).toHaveURL(/\/recipes\/\d+/);
    await expect(page.locator('h1')).toContainText('Spaghetti Carbonara');
    await expect(page.getByText('Mittagessen')).toBeVisible();
    await expect(page.getByText('Eier')).toBeVisible();
  });

  test('sollte Fehler bei fehlenden Pflichtfeldern anzeigen', async ({ page }) => {
    await page.goto('/recipes/new');
    
    // Feld leeren
    await page.fill('input[name="title"]', '');
    await page.click('button[type="submit"]');
    
    // Sollte auf der Seite bleiben (POST bleibt auf /recipes)
    await expect(page).toHaveURL('/recipes');
    
    // Fehlermeldungen prüfen
    await expect(page.locator('.errors')).toContainText('Titel ist erforderlich');
  });

  test('sollte alle Felder korrekt speichern', async ({ page }) => {
    await page.goto('/recipes/new');
    
    await page.fill('input[name="title"]', 'Test Rezept');
    await page.check('input[name="categories"][value="Party"]');
    await page.check('input[name="categories"][value="Snacks"]');
    await page.fill('textarea[name="ingredients"]', 'Zutat 1\nZutat 2');
    await page.fill('textarea[name="instructions"]', 'Schritt 1\nSchritt 2');
    
    await page.click('button[type="submit"]');
    
    await expect(page.locator('h1')).toContainText('Test Rezept');
    await expect(page.getByText('Party')).toBeVisible();
    await expect(page.getByText('Zutat 1')).toBeVisible();
  });
});
