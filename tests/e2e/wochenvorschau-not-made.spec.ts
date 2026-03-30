import { test, expect } from '@playwright/test';

/**
 * E2E-Tests für Story 34: Suche "Länger nicht gemacht" per Klick in der Wochenübersicht
 * 
 * Diese Tests prüfen den Button in der Wochenvorschau, der direkt zur Suche
 * "Länger nicht gemacht" navigiert.
 */

test.describe('Story 34: "Länger nicht gemacht" Button in Wochenvorschau', () => {
  
  test('K1: Button "Länger nicht gemacht" ist in der Wochenübersicht sichtbar', async ({ page }) => {
    // Given: Wochenvorschau wird aufgerufen
    await page.goto('/wochenvorschau');
    
    // Then: Button "Länger nicht gemacht" ist sichtbar
    const notMadeButton = page.locator('a[href*="filter=laenger-nicht-gemacht"]');
    await expect(notMadeButton).toBeVisible();
    await expect(notMadeButton).toContainText('Länger nicht gemacht');
  });

  test('K2: Button hat korrektes ARIA-Label für Barrierefreiheit', async ({ page }) => {
    // Given: Wochenvorschau wird aufgerufen
    await page.goto('/wochenvorschau');
    
    // Then: Button hat aussagekräftiges ARIA-Label
    const notMadeButton = page.locator('a[href*="filter=laenger-nicht-gemacht"]');
    await expect(notMadeButton).toHaveAttribute('aria-label', 'Rezepte anzeigen, die länger nicht gemacht wurden');
  });

  test('K2: Klick öffnet Suche mit vorbelegtem Filter', async ({ page }) => {
    // Given: Wochenvorschau wird aufgerufen
    await page.goto('/wochenvorschau');
    
    // When: Button "Länger nicht gemacht" wird geklickt
    const notMadeButton = page.locator('a[href*="filter=laenger-nicht-gemacht"]');
    await notMadeButton.click();
    
    // Then: URL enthält filter=laenger-nicht-gemacht
    await expect(page).toHaveURL(/filter=laenger-nicht-gemacht/);
    
    // And: Filter-Button ist auf der Startseite als aktiv markiert
    const filterBtn = page.locator('a.sort-filter-btn', { hasText: 'Länger nicht gemacht' });
    await expect(filterBtn).toHaveAttribute('aria-pressed', 'true');
    await expect(filterBtn).toHaveClass(/active/);
  });

  test('K6: Button ist per Tastatur erreichbar', async ({ page }) => {
    // Given: Wochenvorschau wird aufgerufen
    await page.goto('/wochenvorschau');
    
    // When: Tab-Taste drücken um durch die Seite zu navigieren
    // Der Button ist ein Link und sollte fokussierbar sein
    const notMadeButton = page.locator('a[href*="filter=laenger-nicht-gemacht"]');
    await notMadeButton.focus();
    
    // Then: Button "Länger nicht gemacht" ist fokussiert
    await expect(notMadeButton).toBeFocused();
    
    // When: Enter drücken
    await page.keyboard.press('Enter');
    
    // Then: Navigation zur Suche erfolgt
    await expect(page).toHaveURL(/filter=laenger-nicht-gemacht/);
  });

  test('Button ist in der Toolbar oberhalb der Wochenliste positioniert', async ({ page }) => {
    // Given: Wochenvorschau wird aufgerufen
    await page.goto('/wochenvorschau');
    
    // Then: Button befindet sich in einem Container über der Wochenliste
    const toolbar = page.locator('.wochenvorschau-toolbar');
    await expect(toolbar).toBeVisible();
    
    // And: Button ist innerhalb der Toolbar
    const notMadeButton = toolbar.locator('a[href*="filter=laenger-nicht-gemacht"]');
    await expect(notMadeButton).toBeVisible();
  });

  test('Button hat konsistentes Styling mit anderen Buttons', async ({ page }) => {
    // Given: Wochenvorschau wird aufgerufen
    await page.goto('/wochenvorschau');
    
    // Then: Button hat die korrekte CSS-Klasse
    const notMadeButton = page.locator('a.not-made-button');
    await expect(notMadeButton).toBeVisible();
    
    // And: Button hat Hover- und Focus-States
    await notMadeButton.hover();
    // Visuelle Überprüfung wäre hier manuell notwendig
    
    await notMadeButton.focus();
    await expect(notMadeButton).toHaveCSS('outline-color', 'rgb(37, 99, 235)'); // var(--primary-color)
  });

});
