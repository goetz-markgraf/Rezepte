import { test, expect } from '@playwright/test';

test.describe('Rezept-Rating im Edit-Mode (Story 41)', () => {
  test('Sterne 1 bis N sollten markiert sein, wenn Stern N ausgewählt ist', async ({ page }) => {
    // Given: Ein Testrezept wird erstellt
    await page.goto('/recipes/new');
    await page.fill('input[name="title"]', 'Rating Test');
    await page.check('input[name="categories"][value="Mittagessen"]');
    
    // When: 4 Sterne ausgewählt
    const star4 = page.locator('input[name="rating"][value="4"]');
    await star4.locator('xpath=ancestor::label').click();
    await page.click('button[type="submit"]');
    
    // Gehe zum Edit-Mode
    await page.click('text=Bearbeiten');
    await expect(page).toHaveURL(/\/recipes\/\d+\/edit/);
    
    // Then: Die Sterne 1, 2, 3 und 4 sollten die Farbe #f59e0b (rgb(245, 158, 11)) haben
    const ratingColor = 'rgb(245, 158, 11)';
    for (let i = 1; i <= 4; i++) {
      const label = page.locator(`input[name="rating"][value="${i}"]`).locator('xpath=ancestor::label');
      await expect(label).toHaveCSS('color', ratingColor);
    }
    
    // Aber Stern 5 sollte nicht markiert sein (Farbe #6b7280 -> rgb(107, 119, 128))
    const label5 = page.locator(`input[name="rating"][value="5"]`).locator('xpath=ancestor::label');
    await expect(label5).not.toHaveCSS('color', ratingColor);
  });

  test('Hover über Stern N markiert Sterne 1 bis N', async ({ page }) => {
    await page.goto('/recipes/new');
    await page.fill('input[name="title"]', 'Hover Test');
    await page.check('input[name="categories"][value="Mittagessen"]');
    await page.click('button[type="submit"]');
    await page.click('text=Bearbeiten');
    
    // Hover über 3. Stern
    const star3Label = page.locator('input[name="rating"][value="3"]').locator('xpath=ancestor::label');
    await star3Label.hover();
    
    const ratingColor = 'rgb(245, 158, 11)';
    for (let i = 1; i <= 3; i++) {
      const label = page.locator(`input[name="rating"][value="${i}"]`).locator('xpath=ancestor::label');
      await expect(label).toHaveCSS('color', ratingColor);
    }
    
    const label4 = page.locator(`input[name="rating"][value="4"]`).locator('xpath=ancestor::label');
    await expect(label4).not.toHaveCSS('color', ratingColor);
  });
});
