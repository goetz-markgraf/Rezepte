import { test, expect } from '@playwright/test';

test.describe('Rezept erstellen', () => {
  test('sollte ein neues Rezept erfolgreich erstellen', async ({ page }) => {
    // Given: Die Startseite ist geöffnet
    await page.goto('/');
    await page.click('text=Neues Rezept');

    await expect(page).toHaveURL('/recipes/new');
    await expect(page.locator('h1')).toContainText('Neues Rezept');

    // When: Titel, Kategorie, Zutaten und Anleitung ausgefüllt und gespeichert werden
    await page.fill('input[name="title"]', 'Spaghetti Carbonara');
    await page.check('input[name="categories"][value="Mittagessen"]');
    await page.fill('textarea[name="ingredients"]', 'Spaghetti\nEier\nSpeck\nParmesan');
    await page.fill('textarea[name="instructions"]', '1. Nudeln kochen\n2. Sauce zubereiten\n3. Mischen');
    await page.click('button[type="submit"]');

    // Then: Weiterleitung zur Detailseite, alle Daten korrekt angezeigt
    await expect(page).toHaveURL(/\/recipes\/\d+/);
    await expect(page.locator('h1')).toContainText('Spaghetti Carbonara');
    await expect(page.getByText('Mittagessen')).toBeVisible();
    await expect(page.getByText('Eier')).toBeVisible();
  });

  test('sollte Fehler bei fehlenden Pflichtfeldern anzeigen', async ({ page }) => {
    // Given: Das Formular für ein neues Rezept ist geöffnet
    await page.goto('/recipes/new');

    // When: Ohne Titel gespeichert wird
    await page.fill('input[name="title"]', '');
    await page.click('button[type="submit"]');

    // Then: Fehlermeldung "Titel ist erforderlich" erscheint
    await expect(page).toHaveURL('/recipes');
    await expect(page.locator('.errors')).toContainText('Titel ist erforderlich');
  });

  test('sollte alle Felder korrekt speichern', async ({ page }) => {
    // Given: Das Erstellungsformular ist geöffnet
    await page.goto('/recipes/new');

    // When: Titel, mehrere Kategorien, Zutaten und Anleitung gespeichert werden
    await page.fill('input[name="title"]', 'Test Rezept');
    await page.check('input[name="categories"][value="Party"]');
    await page.check('input[name="categories"][value="Snacks"]');
    await page.fill('textarea[name="ingredients"]', 'Zutat 1\nZutat 2');
    await page.fill('textarea[name="instructions"]', 'Schritt 1\nSchritt 2');
    await page.click('button[type="submit"]');

    // Then: Detailseite zeigt alle gespeicherten Daten korrekt an
    await expect(page.locator('h1')).toContainText('Test Rezept');
    await expect(page.getByText('Party')).toBeVisible();
    await expect(page.getByText('Zutat 1')).toBeVisible();
  });
});
