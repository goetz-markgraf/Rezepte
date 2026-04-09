import { test, expect } from '@playwright/test';

/**
 * E2E-Tests für Story 42: Suche "Länger nicht gemacht" in Top-Bar verschieben
 * 
 * Diese Tests prüfen den neuen Link in der Top-Bar, der direkt zur Suche
 * "Länger nicht gemacht" kombiniert mit "Mittagessen" navigiert, sowie
 * die Entfernung des Buttons aus der Wochenvorschau.
 */

test.describe('Story 42: "Inspiration" Link in Top-Bar', () => {
  
  test('K1: Link "Länger nicht gemacht (Mittagessen)" ist in der Top-Bar sichtbar', async ({ page }) => {
    // Given: Die Startseite wird aufgerufen
    await page.goto('/');
    
    // Then: Der Link in der Top-Bar ist sichtbar
    // Wir suchen nach dem Link anhand des Textes oder der Ziel-URL
    const inspirationLink = page.locator('nav.main-nav a[href*="filter=laenger-nicht-gemacht"], nav.main-nav a:has-text("Länger nicht gemacht")').first();
    await expect(inspirationLink).toBeVisible();
  });

  test('K2: Klick auf den Link führt zur gefilterten Suche (Länger nicht gemacht + Mittagessen)', async ({ page }) => {
    // Given: Die Startseite wird aufgerufen
    await page.goto('/');
    
    // When: Der Link in der Top-Bar wird geklickt
    const inspirationLink = page.locator('nav.main-nav a[href*="filter=laenger-nicht-gemacht"]').first();
    await inspirationLink.click();
    
    // Then: URL enthält sowohl den Länger-nicht-gemacht-Filter als auch die Kategorie Mittagessen
    await expect(page).toHaveURL(/filter=laenger-nicht-gemacht/);
    await expect(page).toHaveURL(/kategorie=Mittagessen/);
    
    // And: Die entsprechende Filter-UI-Elemente sind als aktiv markiert
    const filterBtn = page.locator('a.sort-filter-btn', { hasText: 'Länger nicht gemacht' });
    await expect(filterBtn).toHaveAttribute('aria-pressed', 'true');
    
    const mittagssenBtn = page.locator('a.category-filter-btn', { hasText: 'Mittagessen' });
    await expect(mittagssenBtn).toHaveAttribute('aria-pressed', 'true');
  });

  test('K3: Der Button in der Wochenvorschau wurde entfernt', async ({ page }) => {
    // Given: Die Wochenvorschau wird aufgerufen
    await page.goto('/wochenvorschau');
    
    // Then: Der alte Button ".not-made-button" ist nicht mehr vorhanden
    const notMadeButton = page.locator('.not-made-button');
    await expect(notMadeButton).not.toBeVisible();
  });

  test('K4: Link ist per Tastatur erreichbar', async ({ page }) => {
    // Given: Startseite wird aufgerufen
    await page.goto('/');
    
    // When: Tab-Taste drücken bis der Link fokussiert ist
    // (Je nach Position in der Nav varies, wir nutzen .focus() für den Test des Zustands)
    const inspirationLink = page.locator('nav.main-nav a[href*="filter=laenger-nicht-gemacht"]').first();
    await inspirationLink.focus();
    
    // Then: Der Link ist fokussiert
    await expect(inspirationLink).toBeFocused();
    
    // When: Enter drücken
    await page.keyboard.press('Enter');
    
    // Then: Navigation erfolgt
    await expect(page).toHaveURL(/filter=laenger-nicht-gemacht/);
  });

  test('K5: Link ist auch in mobiler Ansicht erreichbar', async ({ page }) => {
    // Given: Mobiler Viewport
    await page.setViewportSize({ width: 375, height: 667 });
    await page.goto('/');
    
    // Then: Der Link ist sichtbar/vorhanden
    const inspirationLink = page.locator('nav.main-nav a[href*="filter=laenger-nicht-gemacht"]').first();
    await expect(inspirationLink).toBeVisible();
  });

});
