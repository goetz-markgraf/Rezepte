import { test, expect } from '@playwright/test';

test.describe('Rezept bearbeiten', () => {
  test.beforeEach(async ({ page }) => {
    // Given: Ein Testrezept "Testrezept Original" wird erstellt
    await page.goto('/recipes/new');
    await page.fill('input[name="title"]', 'Testrezept Original');
    await page.check('input[name="categories"][value="Mittagessen"]');
    await page.fill('textarea[name="ingredients"]', 'Original Zutat');
    await page.fill('textarea[name="instructions"]', 'Original Anleitung');
    await page.click('button[type="submit"]');
    await expect(page).toHaveURL(/\/recipes\/\d+/);
  });

  test('sollte ein Rezept erfolgreich bearbeiten', async ({ page }) => {
    // Given: Ein Testrezept existiert bereits, Detailseite ist aktiv
    // When: Das Bearbeitungsformular geöffnet, Titel und Zutaten geändert und gespeichert werden
    await page.click('text=Bearbeiten');

    await expect(page).toHaveURL(/\/recipes\/\d+\/edit/);
    await expect(page.locator('h1')).toContainText('Rezept bearbeiten');

    // Formular ist mit bestehenden Werten vorausgefüllt
    await expect(page.locator('input[name="title"]')).toHaveValue('Testrezept Original');
    await expect(page.locator('input[name="categories"][value="Mittagessen"]')).toBeChecked();
    await expect(page.locator('textarea[name="ingredients"]')).toHaveValue('Original Zutat');

    await page.fill('input[name="title"]', 'Testrezept Geändert');
    await page.fill('textarea[name="ingredients"]', 'Neue Zutat');
    await page.click('button[type="submit"]');

    // Then: Detailseite zeigt neue Werte und Erfolgsmeldung
    await expect(page).toHaveURL(/\/recipes\/\d+/);
    await expect(page.locator('h1')).toContainText('Testrezept Geändert');
    await expect(page.getByText('Neue Zutat')).toBeVisible();
    await expect(page.locator('.success')).toContainText('Rezept erfolgreich aktualisiert');
  });

  test('sollte Bearbeiten-Button in der Rezept-Liste anzeigen', async ({ page }) => {
    // Given: Ein Testrezept existiert bereits
    // When: Die Rezeptliste aufgerufen wird
    await page.goto('/');

    // Then: Bearbeiten-Link ist im Listeneintrag sichtbar
    await expect(page.locator('.recipe-item a[href*="/edit"]').first()).toBeVisible();
  });

  test('sollte Bearbeitung abbrechen ohne Speichern', async ({ page }) => {
    // Given: Ein Testrezept existiert bereits, Detailseite ist aktiv
    const currentUrl = page.url();
    await page.click('text=Bearbeiten');
    await expect(page).toHaveURL(/\/recipes\/\d+\/edit/);

    // When: Titel geändert, aber Abbrechen angeklickt wird
    await page.fill('input[name="title"]', 'Nicht gespeichert');
    await page.click('text=Abbrechen');

    // Then: Zurück zur Detailseite mit ursprünglichem Titel
    await expect(page).toHaveURL(currentUrl);
    await expect(page.locator('h1')).toContainText('Testrezept Original');
  });

  test('sollte Validierungsfehler anzeigen', async ({ page }) => {
    // Given: Ein Testrezept existiert bereits, Bearbeitungsformular ist geöffnet
    await page.click('text=Bearbeiten');

    // When: Browser-Validierung umgangen (required-Attribut) und Titel geleert und gespeichert wird
    // Story 25 (L6) fügte required-Attribut hinzu – wir testen Server-seitige Validierung
    await page.locator('input[name="title"]').evaluate((el: HTMLInputElement) => el.removeAttribute('required'));
    await page.fill('input[name="title"]', '');
    await page.click('button[type="submit"]');

    // Then: Fehlermeldung erscheint, Zutaten-Feld behält seinen Wert
    await expect(page).toHaveURL(/\/recipes\/\d+/);
    await expect(page.locator('.errors')).toContainText('Titel ist erforderlich');
    await expect(page.locator('textarea[name="ingredients"]')).toHaveValue('Original Zutat');
  });

  test('sollte 404 bei nicht-existentem Rezept zeigen', async ({ page }) => {
    // Given: ID 99999 existiert nicht
    // When: /recipes/99999/edit aufgerufen wird
    await page.goto('/recipes/99999/edit');

    // Then: Fehlermeldung "Rezept mit ID 99999 nicht gefunden"
    await expect(page.locator('body')).toContainText('Rezept mit ID 99999 nicht gefunden');
  });

  test('sollte ein Rezept über den oberen Speichern-Button bearbeiten', async ({ page }) => {
    // Given: Ein Testrezept existiert bereits, Bearbeitungsformular ist geöffnet
    await page.click('text=Bearbeiten');

    // When: Titel geändert und der OBERE Speichern-Button geklickt wird
    await page.fill('input[name="title"]', 'Testrezept Oben Geändert');
    await page.click('button[aria-label="Rezept speichern"]');

    // Then: Detailseite zeigt neue Werte
    await expect(page).toHaveURL(/\/recipes\/\d+/);
    await expect(page.locator('h1')).toContainText('Testrezept Oben Geändert');
  });

  test('sollte Sichtbarkeit beider Speichern-Buttons prüfen', async ({ page }) => {
    // Given: Ein Testrezept existiert bereits, Bearbeitungsformular ist geöffnet
    await page.click('text=Bearbeiten');

    // Then: Oberer Button ist sichtbar neben der Überschrift
    const topButton = page.locator('button[aria-label="Rezept speichern"]');
    await expect(topButton).toBeVisible();

    // Und: Untere Buttons sind weiterhin vorhanden
    const submitButton = page.locator('button[type="submit"]:not([aria-label="Rezept speichern"])');
    const cancelButton = page.locator('text=Abbrechen');
    await expect(submitButton).toBeVisible();
    await expect(cancelButton).toBeVisible();
  });
});

