import { test, expect } from '@playwright/test';

test.describe('Rezept-Detailansicht', () => {
  test('sollte vollständiges Rezept mit allen Feldern anzeigen', async ({ page }) => {
    // Given: Ein Rezept mit Titel, Kategorie, Zutaten und Anleitung wurde erstellt
    await page.goto('/recipes/new');
    await page.fill('input[name="title"]', 'Spaghetti Bolognese');
    await page.check('input[name="categories"][value="Mittagessen"]');
    await page.fill('textarea[name="ingredients"]', 'Spaghetti\nHackfleisch\nTomaten');
    await page.fill('textarea[name="instructions"]', '1. Nudeln kochen\n2. Sauce zubereiten');
    await page.click('button[type="submit"]');

    // When: Die Detailseite aufgerufen wird
    await expect(page).toHaveURL(/\/recipes\/\d+/);

    // Then: Alle Felder sind sichtbar (Titel, Kategorie, Zutaten, Anleitung, Aktions-Links, Meta)
    await expect(page.locator('h1')).toContainText('Spaghetti Bolognese');
    await expect(page.getByText('Mittagessen')).toBeVisible();
    await expect(page.locator('section.ingredients h2')).toContainText('Zutaten');
    await expect(page.getByText('Hackfleisch')).toBeVisible();
    await expect(page.locator('section.instructions h2')).toContainText('Anleitung');
    await expect(page.getByText('Nudeln kochen')).toBeVisible();
    await expect(page.locator('a', { hasText: 'Bearbeiten' })).toBeVisible();
    await expect(page.locator('a', { hasText: 'Zurück zur Übersicht' })).toBeVisible();
    await expect(page.locator('a', { hasText: 'Löschen' })).toBeVisible();
    await expect(page.locator('.meta').first()).toBeVisible();
  });

  test('sollte Rezept ohne optionale Felder anzeigen (keine leeren Abschnitte)', async ({ page }) => {
    // Given: Ein Rezept nur mit Titel und Kategorie (keine Zutaten/Anleitung)
    await page.goto('/recipes/new');
    await page.fill('input[name="title"]', 'Minimalrezept');
    await page.check('input[name="categories"][value="Snacks"]');
    await page.click('button[type="submit"]');

    // When: Die Detailseite aufgerufen wird
    await expect(page).toHaveURL(/\/recipes\/\d+/);

    // Then: Titel sichtbar, Zutaten- und Anleitungs-Sektion werden nicht angezeigt
    await expect(page.locator('h1')).toContainText('Minimalrezept');
    await expect(page.locator('section.ingredients')).not.toBeVisible();
    await expect(page.locator('section.instructions')).not.toBeVisible();
    await expect(page.locator('a', { hasText: 'Bearbeiten' })).toBeVisible();
    await expect(page.locator('a', { hasText: 'Löschen' })).toBeVisible();
  });

  test('sollte bei nicht vorhandener ID eine 404-Seite anzeigen', async ({ page }) => {
    // Given: ID 99999 existiert nicht
    // When: /recipes/99999 aufgerufen wird
    const response = await page.goto('/recipes/99999');

    // Then: HTTP 404, Fehlermeldung und Zurück-Link sind sichtbar
    expect(response?.status()).toBe(404);
    await expect(page.locator('body')).toContainText('nicht gefunden');
    await expect(page.locator('a', { hasText: 'Zurück zur Übersicht' })).toBeVisible();
  });

  test('sollte bei 404 den Link zur Übersicht haben, der zur Startseite führt', async ({ page }) => {
    // Given: Die 404-Seite für ein nicht existierendes Rezept ist geöffnet
    await page.goto('/recipes/99999');

    // When: "Zurück zur Übersicht" angeklickt wird
    await page.click('text=Zurück zur Übersicht');

    // Then: Navigation zur Startseite /
    await expect(page).toHaveURL('/');
  });

  test('sollte DeepLink ohne vorherige Navigation funktionieren', async ({ page }) => {
    // Given: Ein Rezept wurde erstellt und die direkte URL ist bekannt
    await page.goto('/recipes/new');
    await page.fill('input[name="title"]', 'DeepLink Testrezept');
    await page.check('input[name="categories"][value="Party"]');
    await page.click('button[type="submit"]');
    await expect(page).toHaveURL(/\/recipes\/\d+/);
    const url = page.url();

    // When: Die URL direkt aufgerufen wird (ohne Klick-Navigation)
    await page.goto(url);

    // Then: Seite lädt korrekt mit dem richtigen Rezept
    await expect(page).toHaveURL(url);
    await expect(page.locator('h1')).toContainText('DeepLink Testrezept');
  });

  test('sollte Erfolgs-Flash nach Bearbeiten anzeigen', async ({ page }) => {
    // Given: Ein Rezept wurde erstellt und bearbeitet
    await page.goto('/recipes/new');
    await page.fill('input[name="title"]', 'Flash Test Rezept');
    await page.check('input[name="categories"][value="Kuchen"]');
    await page.click('button[type="submit"]');
    await expect(page).toHaveURL(/\/recipes\/\d+/);
    await page.click('text=Bearbeiten');
    await expect(page).toHaveURL(/\/recipes\/\d+\/edit/);

    // When: Das Bearbeitungsformular gespeichert wird
    await page.fill('input[name="title"]', 'Flash Test Rezept Geändert');
    await page.click('button[type="submit"]');

    // Then: Detailseite zeigt Erfolgsmeldung "Rezept erfolgreich aktualisiert"
    await expect(page).toHaveURL(/\/recipes\/\d+/);
    await expect(page.locator('.success')).toContainText('Rezept erfolgreich aktualisiert');
  });

  test('sollte Navigationslinks korrekt verknüpfen', async ({ page }) => {
    // Given: Ein Rezept wurde erstellt, Detailseite ist aktiv
    await page.goto('/recipes/new');
    await page.fill('input[name="title"]', 'Navigationstest');
    await page.check('input[name="categories"][value="Brot"]');
    await page.click('button[type="submit"]');
    await expect(page).toHaveURL(/\/recipes\/\d+/);
    const detailUrl = page.url();
    const id = detailUrl.split('/').pop();

    // When: Die Links auf der Seite geprüft werden
    // Then: Bearbeiten-, Löschen- und Zurück-Link haben korrekte URLs
    await expect(page.locator(`a[href="/recipes/${id}/edit"]`)).toBeVisible();
    await expect(page.locator(`a[href="/recipes/${id}/confirm-delete"]`)).toBeVisible();
    const backLink = page.locator('a[href="/"]', { hasText: 'Zurück zur Übersicht' });
    await expect(backLink).toBeVisible();
    await backLink.click();
    await expect(page).toHaveURL('/');
  });
});
