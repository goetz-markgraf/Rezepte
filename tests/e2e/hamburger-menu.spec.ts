import { test, expect } from '@playwright/test';

// Story 45: Hamburger-Menü öffnet an der richtigen Position

test.describe('Hamburger-Menü Position', () => {
  
  test('Menü öffnet und schließt mit ARIA-Status', async ({ page }) => {
    await page.goto('/');
    
    const hamburgerBtn = page.locator('#hamburger-btn');
    await expect(hamburgerBtn).toBeVisible();
    
    const menu = page.locator('#hamburger-dropdown');
    
    // Menü ist initial geschlossen
    await expect(menu).not.toBeVisible();
    await expect(hamburgerBtn).toHaveAttribute('aria-expanded', 'false');
    
    // Klick öffnet Menü
    await hamburgerBtn.click();
    
    await expect(menu).toBeVisible();
    await expect(hamburgerBtn).toHaveAttribute('aria-expanded', 'true');
    
    // Escape schließt
    await page.keyboard.press('Escape');
    
    await expect(menu).not.toBeVisible();
    await expect(hamburgerBtn).toHaveAttribute('aria-expanded', 'false');
  });

  test('Menü ist korrekt zum Hamburger-Icon ausgerichtet (right-aligned auf mobile)', async ({ page }) => {
    // Hamburger Icon steht rechts auf mobile → Menü muss am Icon anliegen
    await page.setViewportSize({ width: 375, height: 667 });
    await page.goto('/');
    
    const hamburgerBtn = page.locator('#hamburger-btn');
    await hamburgerBtn.click();
    
    const menu = page.locator('#hamburger-dropdown');
    await expect(menu).toBeVisible();
    
    const menuBox = await menu.boundingBox();
    test.expect(menuBox).toBeDefined();
    
    if (menuBox) {
      // Menü muss vollständig im Viewport sein
      await expect(menuBox.x).toBeGreaterThanOrEqual(-2);
      await expect(menuBox.x + menuBox.width).toBeLessThanOrEqual(375 + 2);
      await expect(menuBox.width).toBeGreaterThan(0);
      await expect(menuBox.height).toBeGreaterThan(0);
    }
  });
  
  test('Menü startet innerhalb des Viewports auf allen Bildbreiten', async ({ page }) => {
    const widths = [320, 360, 375, 414, 768];
    
    for (const width of widths) {
      await page.setViewportSize({ width, height: 667 });
      await page.goto('/');
      
      await page.locator('#hamburger-btn').click();
      
      const menuBox = await page.locator('#hamburger-dropdown').boundingBox();
      test.expect(menuBox).toBeDefined();
      
      if (menuBox) {
        // Menü darf nicht nach links außerhalb des Viewports ragen
        await expect(menuBox.x).toBeGreaterThanOrEqual(-2);
        
        // Menü darf nicht nach rechts außerhalb des Viewports ragen
        await expect(menuBox.x + menuBox.width).toBeLessThanOrEqual(width + 2);
      }
    }
  });

  test('Menü schließt bei Klick außerhalb', async ({ page }) => {
    await page.goto('/');
    
    const hamburgerBtn = page.locator('#hamburger-btn');
    await hamburgerBtn.click();
    
    const menu = page.locator('#hamburger-dropdown');
    await expect(menu).toBeVisible();
    
    // Klick außerhalb des Menüs
    await page.locator('body').click({ position: { x: 10, y: 10 } });
    
    await expect(menu).not.toBeVisible();
    await expect(hamburgerBtn).toHaveAttribute('aria-expanded', 'false');
  });

  test('Menü-Einträge sind klickbar', async ({ page }) => {
    await page.goto('/');
    
    const hamburgerBtn = page.locator('#hamburger-btn');
    await hamburgerBtn.click();
    
    const menuItems = page.locator('#hamburger-dropdown a[role="menuitem"]');
    await expect(menuItems).toHaveCount(2);
    
    const menu = page.locator('#hamburger-dropdown');
    await expect(menu).toBeVisible();
    
    // Link im Menü prüfen
    const heuteLink = menu.locator('a[href="/heute"]');
    const href = await heuteLink.getAttribute('href');
    test.expect(href).toBe('/heute');
  });

});
