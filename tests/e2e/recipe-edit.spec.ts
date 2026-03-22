import { test, expect } from '@playwright/test';

test.describe('Rezept bearbeiten', () => {
  test.beforeEach(async ({ page }) => {
    // Ein Rezept erstellen, das wir bearbeiten können
    await page.goto('/recipes/new');
    await page.fill('input[name="title"]', 'Testrezept Original');
    await page.check('input[name="categories"][value="Mittagessen"]');
    await page.fill('textarea[name="ingredients"]', 'Original Zutat');
    await page.fill('textarea[name="instructions"]', 'Original Anleitung');
    await page.click('button[type="submit"]');
    await expect(page).toHaveURL(/\/recipes\/\d+/);
  });

  test('sollte ein Rezept erfolgreich bearbeiten', async ({ page }) => {
    // Auf Bearbeiten klicken
    await page.click('text=Bearbeiten');
    
    await expect(page).toHaveURL(/\/recipes\/\d+\/edit/);
    await expect(page.locator('h1')).toContainText('Rezept bearbeiten');
    
    // Prüfen, dass Formular vorausgefüllt ist
    await expect(page.locator('input[name="title"]')).toHaveValue('Testrezept Original');
    await expect(page.locator('input[name="categories"][value="Mittagessen"]')).toBeChecked();
    await expect(page.locator('textarea[name="ingredients"]')).toHaveValue('Original Zutat');
    
    // Titel ändern
    await page.fill('input[name="title"]', 'Testrezept Geändert');
    await page.fill('textarea[name="ingredients"]', 'Neue Zutat');
    
    await page.click('button[type="submit"]');
    
    // Zurück auf Detailseite mit geändertem Titel
    await expect(page).toHaveURL(/\/recipes\/\d+/);
    await expect(page.locator('h1')).toContainText('Testrezept Geändert');
    await expect(page.getByText('Neue Zutat')).toBeVisible();
  });

  test('sollte Bearbeitung abbrechen ohne Speichern', async ({ page }) => {
    const currentUrl = page.url();
    
    await page.click('text=Bearbeiten');
    await expect(page).toHaveURL(/\/recipes\/\d+\/edit/);
    
    // Titel ändern aber nicht speichern
    await page.fill('input[name="title"]', 'Nicht gespeichert');
    
    // Abbrechen
    await page.click('text=Abbrechen');
    
    // Zurück auf Detailseite mit unverändertem Titel
    await expect(page).toHaveURL(currentUrl);
    await expect(page.locator('h1')).toContainText('Testrezept Original');
  });

  test('sollte Validierungsfehler anzeigen', async ({ page }) => {
    await page.click('text=Bearbeiten');
    
    // Titel leeren
    await page.fill('input[name="title"]', '');
    await page.click('button[type="submit"]');
    
    // Sollte auf der Seite bleiben
    await expect(page).toHaveURL(/\/recipes\/\d+/); // POST auf /recipes/:id
    await expect(page.locator('.errors')).toContainText('Titel ist erforderlich');
    
    // Eingegebene Werte sollten erhalten bleiben (kein Zurücksetzen)
    await expect(page.locator('textarea[name="ingredients"]')).toHaveValue('Original Zutat');
  });

  test('sollte 404 bei nicht-existentem Rezept zeigen', async ({ page }) => {
    await page.goto('/recipes/99999/edit');
    
    await expect(page.locator('body')).toContainText('Rezept mit ID 99999 nicht gefunden');
  });
});
