import { test, expect } from '@playwright/test';

test.describe('Header Navigation - Neues Rezept Button', () => {
  
  test('Button ist auf der Startseite sichtbar', async ({ page }) => {
    // Given: Der Benutzer ist auf der Startseite
    await page.goto('/');
    
    // Then: Der "Neues Rezept"-Link ist in der Kopfzeile sichtbar
    const newRecipeButton = page.getByRole('link', { name: 'Neues Rezept' }).first();
    await expect(newRecipeButton).toBeVisible();
    await expect(newRecipeButton).toContainText('Neues Rezept');
  });

  test('Button ist auf der Wochenvorschau-Seite sichtbar', async ({ page }) => {
    // Given: Der Benutzer ist auf der Wochenvorschau-Seite
    await page.goto('/wochenvorschau');
    
    // Then: Der "Neues Rezept"-Link ist in der Kopfzeile sichtbar
    const newRecipeButton = page.getByRole('link', { name: 'Neues Rezept' }).first();
    await expect(newRecipeButton).toBeVisible();
  });

  test('Button ist auf der Dubletten-Prüf-Seite sichtbar', async ({ page }) => {
    // Given: Der Benutzer ist auf der Dubletten-Prüf-Seite
    await page.goto('/recipes/duplicates');
    
    // Then: Der "Neues Rezept"-Link ist in der Kopfzeile sichtbar
    const newRecipeButton = page.getByRole('link', { name: 'Neues Rezept' }).first();
    await expect(newRecipeButton).toBeVisible();
  });

  test('Button navigiert zur Rezept-Erstell-Seite', async ({ page }) => {
    // Given: Der Benutzer ist auf einer beliebigen Seite
    await page.goto('/');
    
    // When: Der Benutzer auf den "Neues Rezept"-Link klickt
    const newRecipeButton = page.getByRole('link', { name: 'Neues Rezept' }).first();
    await newRecipeButton.click();
    
    // Then: Die Seite "/recipes/new" wird geladen
    await expect(page).toHaveURL('/recipes/new');
    
    // And: Das Formular zum Erstellen eines Rezepts wird angezeigt
    await expect(page.locator('h1')).toContainText('Neues Rezept');
    await expect(page.locator('form')).toBeVisible();
  });

  test('Button ist mit Tastatur erreichbar', async ({ page }) => {
    // Given: Der Benutzer ist auf der Startseite
    await page.goto('/');
    
    // When: Der Benutzer Tab drückt, um den ersten Nav-Link zu fokussieren
    await page.keyboard.press('Tab'); // site-title
    await page.keyboard.press('Tab'); // Neues Rezept (jetzt erster Link in der Nav)
    
    // Then: Der Fokus ist auf dem "Neues Rezept"-Link
    const newRecipeButton = page.getByRole('link', { name: 'Neues Rezept' }).first();
    await expect(newRecipeButton).toBeFocused();
    
    // When: Der Benutzer Enter drückt
    await page.keyboard.press('Enter');
    
    // Then: Die Seite "/recipes/new" wird geladen
    await expect(page).toHaveURL('/recipes/new');
  });

  test('Button ist auf Mobile sichtbar und klickbar', async ({ page }) => {
    // Given: Der Benutzer nutzt ein mobiles Gerät (Viewport < 768px)
    await page.setViewportSize({ width: 375, height: 667 });
    await page.goto('/');
    
    // Then: Der "Neues Rezept"-Link ist sichtbar
    const newRecipeButton = page.getByRole('link', { name: 'Neues Rezept' }).first();
    await expect(newRecipeButton).toBeVisible();
    
    // And: Der Link ist klickbar
    await newRecipeButton.click();
    
    // Then: Navigation funktioniert
    await expect(page).toHaveURL('/recipes/new');
  });

});
