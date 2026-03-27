import { test, expect } from '@playwright/test';

test.describe('Rezept-Detailansicht', () => {
  test('sollte vollständiges Rezept mit allen Feldern anzeigen', async ({ page }) => {
    // Rezept mit allen Feldern erstellen
    await page.goto('/recipes/new');
    await page.fill('input[name="title"]', 'Spaghetti Bolognese');
    await page.check('input[name="categories"][value="Mittagessen"]');
    await page.fill('textarea[name="ingredients"]', 'Spaghetti\nHackfleisch\nTomaten');
    await page.fill('textarea[name="instructions"]', '1. Nudeln kochen\n2. Sauce zubereiten');
    await page.click('button[type="submit"]');

    // Auf der Detailseite landen
    await expect(page).toHaveURL(/\/recipes\/\d+/);

    // Titel als H1 prüfen
    await expect(page.locator('h1')).toContainText('Spaghetti Bolognese');

    // Kategorie-Tag prüfen
    await expect(page.getByText('Mittagessen')).toBeVisible();

    // Abschnitt "Zutaten" sichtbar
    await expect(page.locator('section.ingredients h2')).toContainText('Zutaten');
    await expect(page.getByText('Hackfleisch')).toBeVisible();

    // Abschnitt "Anleitung" sichtbar
    await expect(page.locator('section.instructions h2')).toContainText('Anleitung');
    await expect(page.getByText('Nudeln kochen')).toBeVisible();

    // Aktions-Schaltflächen prüfen
    await expect(page.locator('a', { hasText: 'Bearbeiten' })).toBeVisible();
    await expect(page.locator('a', { hasText: 'Zurück zur Übersicht' })).toBeVisible();
    await expect(page.locator('a', { hasText: 'Löschen' })).toBeVisible();

    // Metainformationen prüfen (Datum im deutschen Format)
    await expect(page.locator('.meta').first()).toBeVisible();
  });

  test('sollte Rezept ohne optionale Felder anzeigen (keine leeren Abschnitte)', async ({ page }) => {
    // Rezept nur mit Titel und Kategorie erstellen
    await page.goto('/recipes/new');
    await page.fill('input[name="title"]', 'Minimalrezept');
    await page.check('input[name="categories"][value="Snacks"]');
    await page.click('button[type="submit"]');

    // Auf der Detailseite landen
    await expect(page).toHaveURL(/\/recipes\/\d+/);

    // Titel sichtbar
    await expect(page.locator('h1')).toContainText('Minimalrezept');

    // Keine Zutaten-Sektion
    await expect(page.locator('section.ingredients')).not.toBeVisible();

    // Keine Anleitungs-Sektion
    await expect(page.locator('section.instructions')).not.toBeVisible();

    // Aktions-Schaltflächen trotzdem vorhanden
    await expect(page.locator('a', { hasText: 'Bearbeiten' })).toBeVisible();
    await expect(page.locator('a', { hasText: 'Löschen' })).toBeVisible();
  });

  test('sollte bei nicht vorhandener ID eine 404-Seite anzeigen', async ({ page }) => {
    const response = await page.goto('/recipes/99999');

    // HTTP-Status 404 prüfen
    expect(response?.status()).toBe(404);

    // Verständliche Fehlermeldung prüfen
    await expect(page.locator('body')).toContainText('nicht gefunden');

    // Link zurück zur Rezeptliste prüfen
    await expect(page.locator('a', { hasText: 'Zurück zur Übersicht' })).toBeVisible();
  });

  test('sollte bei 404 den Link zur Übersicht haben, der zur Startseite führt', async ({ page }) => {
    await page.goto('/recipes/99999');

    await page.click('text=Zurück zur Übersicht');
    await expect(page).toHaveURL('/');
  });

  test('sollte DeepLink ohne vorherige Navigation funktionieren', async ({ page }) => {
    // Rezept erstellen und ID ermitteln
    await page.goto('/recipes/new');
    await page.fill('input[name="title"]', 'DeepLink Testrezept');
    await page.check('input[name="categories"][value="Party"]');
    await page.click('button[type="submit"]');
    await expect(page).toHaveURL(/\/recipes\/\d+/);

    const url = page.url();

    // Direktaufruf der URL ohne vorherige Navigation
    await page.goto(url);
    await expect(page).toHaveURL(url);
    await expect(page.locator('h1')).toContainText('DeepLink Testrezept');
  });

  test('sollte Erfolgs-Flash nach Bearbeiten anzeigen', async ({ page }) => {
    // Rezept erstellen
    await page.goto('/recipes/new');
    await page.fill('input[name="title"]', 'Flash Test Rezept');
    await page.check('input[name="categories"][value="Kuchen"]');
    await page.click('button[type="submit"]');
    await expect(page).toHaveURL(/\/recipes\/\d+/);

    // Rezept bearbeiten
    await page.click('text=Bearbeiten');
    await expect(page).toHaveURL(/\/recipes\/\d+\/edit/);
    await page.fill('input[name="title"]', 'Flash Test Rezept Geändert');
    await page.click('button[type="submit"]');

    // Auf der Detailseite mit Erfolgsmeldung
    await expect(page).toHaveURL(/\/recipes\/\d+/);
    await expect(page.locator('.success')).toContainText('Rezept erfolgreich aktualisiert');
  });

  test('sollte Navigationslinks korrekt verknüpfen', async ({ page }) => {
    // Rezept erstellen
    await page.goto('/recipes/new');
    await page.fill('input[name="title"]', 'Navigationstest');
    await page.check('input[name="categories"][value="Brot"]');
    await page.click('button[type="submit"]');
    await expect(page).toHaveURL(/\/recipes\/\d+/);

    const detailUrl = page.url();
    const id = detailUrl.split('/').pop();

    // "Bearbeiten" führt zu /recipes/{id}/edit
    await expect(page.locator(`a[href="/recipes/${id}/edit"]`)).toBeVisible();

    // "Löschen" führt zu /recipes/{id}/confirm-delete
    await expect(page.locator(`a[href="/recipes/${id}/confirm-delete"]`)).toBeVisible();

    // "Zurück zur Übersicht" führt zu /
    const backLink = page.locator('a[href="/"]', { hasText: 'Zurück zur Übersicht' });
    await expect(backLink).toBeVisible();
    await backLink.click();
    await expect(page).toHaveURL('/');
  });
});
