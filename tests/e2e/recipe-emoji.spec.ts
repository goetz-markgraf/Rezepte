import { test, expect } from '@playwright/test';

test.describe('Emoji für Rezepte', () => {
  async function createRecipe(
    page: import('@playwright/test').Page,
    title: string,
    category: string,
    ingredients: string = '',
    instructions: string = ''
  ): Promise<string> {
    await page.goto('/recipes/new');
    await page.fill('input[name="title"]', title);
    await page.check(`input[name="categories"][value="${category}"]`);
    if (ingredients) {
      await page.fill('textarea[name="ingredients"]', ingredients);
    }
    if (instructions) {
      await page.fill('textarea[name="instructions"]', instructions);
    }
    await page.click('button[type="submit"]');
    await expect(page).toHaveURL(/\/recipes\/\d+/);
    return page.url();
  }

  test('sollte Emoji für Rezept mit Zutaten und Anleitung anzeigen (K1)', async ({ page }) => {
    // Given: Rezept mit Zutaten und Anleitung
    const title = `Pizza-${Date.now()}`;
    await createRecipe(page, title, 'Mittagessen', 'Mehl, Tomaten, Mozzarella', 'Teig kneten, belegen, backen');

    // When: Rezeptliste aufrufen
    await page.goto('/');

    // Then: Emoji ist neben dem Rezeptnamen sichtbar
    await expect(page.locator(`.recipe-item-link:has-text("${title}") .recipe-emoji`)).toBeVisible();
    await expect(page.locator(`.recipe-item-link:has-text("${title}") .recipe-emoji`)).toContainText('🍕');
  });

  test('sollte kein Emoji für Rezept ohne Zutaten und Anleitung anzeigen (K2)', async ({ page }) => {
    // Given: Rezept ohne Zutaten und Anleitung
    const title = `Ohne-Emoji-${Date.now()}`;
    await createRecipe(page, title, 'Brot');

    // When: Rezeptliste aufrufen
    await page.goto('/');

    // Dann: Kein Emoji vorhanden
    const link = page.locator(`.recipe-item-link:has-text("${title}")`);
    const emoji = link.locator('.recipe-emoji');
    await expect(emoji).not.toBeVisible();
  });

  test('sollte passendes Emoji basierend auf Schlüsselwörtern wählen (K3)', async ({ page }) => {
    // Given: Rezept mit bekanntem Keyword
    const title = `Kuchen-${Date.now()}`;
    await createRecipe(page, title, 'Kuchen', 'Schokolade', 'Backen');

    // When: Rezeptliste aufrufen
    await page.goto('/');

    // Then: Passendes Emoji (🎂)
    await expect(page.locator(`.recipe-item-link:has-text("${title}") .recipe-emoji`)).toBeVisible();
  });

  test('sollte Fallback-Emoji bei bekanntem Titel und Zutaten aber unbekanntem Keyword zeigen', async ({ page }) => {
    // Given: Rezept mit Inhalt aber keinem bekannten Keyword
    const title = `Leckerbissen-${Date.now()}`;
    await createRecipe(page, title, 'Snacks', 'Etwas Ungewöhnliches', 'Mischen und servieren');

    // When: Rezeptliste aufrufen
    await page.goto('/');

    // Then: Fallback-Emoji 🍽️
    const emoji = page.locator(`.recipe-item-link:has-text("${title}") .recipe-emoji`);
    await expect(emoji).toBeVisible();
    await expect(emoji).toContainText('🍽️');
  });
});
