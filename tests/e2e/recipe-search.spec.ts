import { test, expect } from '@playwright/test';

/**
 * E2E-Tests für Story 7: Volltextsuche
 *
 * Die Tests verwenden Rezepte, die direkt über das Formular erstellt werden,
 * um Isolation sicherzustellen (kein separates Seed-SQL nötig).
 */

async function createRecipe(
  page: import('@playwright/test').Page,
  title: string,
  category: string,
  ingredients?: string,
  instructions?: string
): Promise<void> {
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
}

test.describe('Volltextsuche', () => {
  test('K1: Suchfeld ist sichtbar mit Label und Platzhaltertext', async ({ page }) => {
    await page.goto('/');

    // Label ist vorhanden
    const label = page.locator('label[for="q"]');
    await expect(label).toBeVisible();

    // Suchfeld ist vorhanden mit korrektem Platzhaltertext
    const searchInput = page.locator('input#q[name="q"]');
    await expect(searchInput).toBeVisible();
    await expect(searchInput).toHaveAttribute('placeholder', 'Rezepte durchsuchen...');

    // Submit-Button ist vorhanden
    await expect(page.locator('form.search-form button[type="submit"]')).toBeVisible();
  });

  test('K2: Suche nach Titel findet passendes Rezept, nicht das unpassende', async ({ page }) => {
    const suffix = Date.now();
    await createRecipe(page, `Spaghetti Bolognese ${suffix}`, 'Mittagessen', 'Hackfleisch, Tomaten', 'Sauce kochen');
    await createRecipe(page, `Pfannkuchen ${suffix}`, 'Snacks', 'Mehl, Eier', 'In der Pfanne backen');

    await page.goto('/');
    await page.fill('input#q', `Bolognese ${suffix}`);
    await page.click('button[type="submit"]');

    await expect(page.locator('#recipe-results')).toContainText(`Spaghetti Bolognese ${suffix}`);
    await expect(page.locator('#recipe-results')).not.toContainText(`Pfannkuchen ${suffix}`);
  });

  test('K2: Suche nach Zutat findet Rezept mit dieser Zutat', async ({ page }) => {
    const suffix = Date.now();
    await createRecipe(page, `Dinkelkuchen ${suffix}`, 'Kuchen', `Dinkelvollkornmehl ${suffix}, Eier`, 'Backen');
    await createRecipe(page, `Weizenbrot ${suffix}`, 'Brot', 'Weizenmehl, Hefe', 'Kneten');

    await page.goto('/');
    await page.fill('input#q', `Dinkelvollkornmehl ${suffix}`);
    await page.click('button[type="submit"]');

    await expect(page.locator('#recipe-results')).toContainText(`Dinkelkuchen ${suffix}`);
    await expect(page.locator('#recipe-results')).not.toContainText(`Weizenbrot ${suffix}`);
  });

  test('K2: Suche nach Anleitung findet Rezept mit diesem Begriff', async ({ page }) => {
    const suffix = Date.now();
    await createRecipe(page, `Ofenbrot ${suffix}`, 'Brot', 'Mehl, Hefe', `Im Ofen backen ${suffix}`);
    await createRecipe(page, `Pfannengericht ${suffix}`, 'Mittagessen', 'Zutaten', 'In der Pfanne braten');

    await page.goto('/');
    await page.fill('input#q', `Im Ofen backen ${suffix}`);
    await page.click('button[type="submit"]');

    await expect(page.locator('#recipe-results')).toContainText(`Ofenbrot ${suffix}`);
    await expect(page.locator('#recipe-results')).not.toContainText(`Pfannengericht ${suffix}`);
  });

  test('K4: Suche ist case-insensitiv (GROSSBUCHSTABEN)', async ({ page }) => {
    const suffix = Date.now();
    await createRecipe(page, `Spaghetti Bolognese ${suffix}`, 'Mittagessen', 'Hackfleisch', 'Sauce kochen');

    await page.goto('/');
    await page.fill('input#q', `BOLOGNESE ${suffix}`);
    await page.click('button[type="submit"]');

    await expect(page.locator('#recipe-results')).toContainText(`Spaghetti Bolognese ${suffix}`);
  });

  test('K5: Leere Suche zeigt alle Rezepte', async ({ page }) => {
    const suffix = Date.now();
    await createRecipe(page, `Apfelkuchen ${suffix}`, 'Kuchen', 'Äpfel', 'Backen');
    await createRecipe(page, `Bolognese ${suffix}`, 'Mittagessen', 'Hackfleisch', 'Kochen');

    // Erst mit Filter suchen
    await page.goto(`/?q=Apfelkuchen ${suffix}`);
    await expect(page.locator('#recipe-results')).toContainText(`Apfelkuchen ${suffix}`);
    await expect(page.locator('#recipe-results')).not.toContainText(`Bolognese ${suffix}`);

    // Dann Suchfeld leeren und absenden
    await page.fill('input#q', '');
    await page.click('button[type="submit"]');

    // Alle Rezepte sichtbar
    await expect(page.locator('#recipe-results')).toContainText(`Apfelkuchen ${suffix}`);
    await expect(page.locator('#recipe-results')).toContainText(`Bolognese ${suffix}`);
  });

  test('K6: Keine Treffer zeigt Meldung mit Suchbegriff', async ({ page }) => {
    await page.goto('/');
    await page.fill('input#q', 'xyzxyzxyzxyz');
    await page.click('button[type="submit"]');

    await expect(page.locator('#recipe-results')).toContainText('Keine Rezepte');
    await expect(page.locator('#recipe-results')).toContainText('xyzxyzxyzxyz');
  });

  test('K7: Suchbegriff bleibt im Suchfeld erhalten (DeepLink)', async ({ page }) => {
    const suffix = Date.now();
    await createRecipe(page, `Bolognese ${suffix}`, 'Mittagessen', 'Hackfleisch', 'Kochen');

    // Direkt mit URL-Parameter aufrufen
    await page.goto(`/?q=Bolognese+${suffix}`);

    // Suchfeld enthält den Suchbegriff
    const searchInput = page.locator('input#q');
    await expect(searchInput).toHaveValue(`Bolognese ${suffix}`);

    // Ergebnisse werden gefiltert angezeigt
    await expect(page.locator('#recipe-results')).toContainText(`Bolognese ${suffix}`);
  });

  test('K9: Ergebnisbereich hat ARIA live region', async ({ page }) => {
    await page.goto('/');

    const resultsDiv = page.locator('#recipe-results');
    await expect(resultsDiv).toHaveAttribute('aria-live', 'polite');
  });
});
