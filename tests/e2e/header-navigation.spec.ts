import { test, expect } from '@playwright/test';

test.describe('Header Navigation - Neues Rezept Button', () => {
  
  test('Button ist auf der Startseite sichtbar', async ({ page }) => {
    // Given: Der Benutzer ist auf der Startseite
    await page.goto('/');
    
    // Then: Der "Neues Rezept"-Button ist in der Kopfzeile sichtbar
    const newRecipeButton = page.getByRole('link', { name: /neues rezept erstellen/i });
    await expect(newRecipeButton).toBeVisible();
    
    // Der Button sollte das Plus-Icon und den Text enthalten
    await expect(newRecipeButton).toContainText('Neues Rezept');
  });

  test('Button ist auf der Wochenvorschau-Seite sichtbar', async ({ page }) => {
    // Given: Der Benutzer ist auf der Wochenvorschau-Seite
    await page.goto('/wochenvorschau');
    
    // Then: Der "Neues Rezept"-Button ist in der Kopfzeile sichtbar
    const newRecipeButton = page.getByRole('link', { name: /neues rezept erstellen/i });
    await expect(newRecipeButton).toBeVisible();
  });

  test('Button ist auf der Dubletten-Prüf-Seite sichtbar', async ({ page }) => {
    // Given: Der Benutzer ist auf der Dubletten-Prüf-Seite
    await page.goto('/recipes/duplicates');
    
    // Then: Der "Neues Rezept"-Button ist in der Kopfzeile sichtbar
    const newRecipeButton = page.getByRole('link', { name: /neues rezept erstellen/i });
    await expect(newRecipeButton).toBeVisible();
  });

  test('Button navigiert zur Rezept-Erstell-Seite', async ({ page }) => {
    // Given: Der Benutzer ist auf einer beliebigen Seite
    await page.goto('/');
    
    // When: Der Benutzer auf den "Neues Rezept"-Button klickt
    const newRecipeButton = page.getByRole('link', { name: /neues rezept erstellen/i });
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
    
    // When: Der Benutzer mehrmals Tab drückt, um den Button zu fokussieren
    await page.keyboard.press('Tab'); // site-title
    await page.keyboard.press('Tab'); // Heute
    await page.keyboard.press('Tab'); // Wochenvorschau
    await page.keyboard.press('Tab'); // Dubletten prüfen
    await page.keyboard.press('Tab'); // Neues Rezept Button
    
    // Then: Der Fokus ist auf dem "Neues Rezept"-Button
    const newRecipeButton = page.getByRole('link', { name: /neues rezept erstellen/i });
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
    
    // When: Die Seite geöffnet wird
    // Then: Der "Neues Rezept"-Button ist als Icon oder mit Text sichtbar
    const newRecipeButton = page.getByRole('link', { name: /neues rezept erstellen/i });
    await expect(newRecipeButton).toBeVisible();
    
    // And: Der Button ist klickbar
    await expect(newRecipeButton).toBeEnabled();
    
    // When: Auf den Button geklickt wird
    await newRecipeButton.click();
    
    // Then: Navigation funktioniert
    await expect(page).toHaveURL('/recipes/new');
  });

  test('Button hat korrekte ARIA-Attribute für Barrierefreiheit', async ({ page }) => {
    // Given: Der Benutzer ist auf der Startseite
    await page.goto('/');
    
    // Then: Der Button hat ein aria-label für Screenreader
    const newRecipeButton = page.getByRole('link', { name: /neues rezept erstellen/i });
    await expect(newRecipeButton).toHaveAttribute('aria-label', 'Neues Rezept erstellen');
  });

});
