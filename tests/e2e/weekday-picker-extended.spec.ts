import { test, expect } from '@playwright/test';

test.describe('Wochen-Picker erweitert (Story 29)', () => {

  // K1: Picker zeigt 10 Tage ab morgen
  test('sollte 10 Tage anzeigen, beginnend mit morgen', async ({ page }) => {
    // Given: Das Formular ist geöffnet (heute ist Montag, 30.03.2026)
    await page.goto('/recipes/new');
    
    // Then: Werden 10 Tage angezeigt
    const buttons = page.locator('.weekday-btn');
    await expect(buttons).toHaveCount(10);
    
    // Erster Button ist morgen (Di 31.3)
    await expect(buttons.nth(0)).toContainText('Di 31.3');
    
    // Letzter Button ist in 10 Tagen (Do 9.4)
    await expect(buttons.nth(9)).toContainText('Do 9.4');
  });

  // K2: Klick auf Tag setzt korrektes Datum
  test('sollte beim Klick auf den 5. Tag das korrekte Datum setzen', async ({ page }) => {
    // Given: Heute ist Montag, 30.03.2026
    await page.goto('/recipes/new');
    
    // When: Der Nutzer auf den 5. Tag klickt (Offset 4 = Sa 04.04)
    await page.locator('.weekday-btn').nth(4).click();
    
    // Then: Enthält das Datumsfeld "4.4.2026"
    await expect(page.locator('input[name="planned_date"]')).toHaveValue('4.4.2026');
    
    // And: Der Tag ist als aktiv markiert
    await expect(page.locator('.weekday-btn').nth(4)).toHaveAttribute('aria-pressed', 'true');
  });

  // K3: Beginn bei morgen (nicht Montag)
  test('sollte mit morgen beginnen, nicht mit dem nächsten Montag', async ({ page }) => {
    // Given: Heute ist Montag, 30.03.2026
    await page.goto('/recipes/new');
    
    // When: Der Picker wird angezeigt
    // Then: Der erste Tag ist morgen (Di 31.3), nicht der nächste Montag
    const firstButton = page.locator('.weekday-btn').nth(0);
    await expect(firstButton).toContainText('Di 31.3');
    
    // Es gibt KEINEN Montag-Button vor dem ersten Tag
    const firstButtonText = await firstButton.innerText();
    expect(firstButtonText).not.toContain('Mo 30.3');
  });

  // K4: Monatswechsel wird korrekt angezeigt
  test('sollte Monatswechsel korrekt anzeigen', async ({ page }) => {
    // Given: Heute ist Freitag, 27.03.2026
    await page.clock.install();
    await page.clock.setFixedTime(new Date('2026-03-27T10:00:00'));
    await page.goto('/recipes/new');
    
    // When: Der Nutzer die Rezept-Bearbeiten-Seite öffnet
    // Then: Zeigt der 1. Tag "Sa 28.3"
    await expect(page.locator('.weekday-btn').nth(0)).toContainText('Sa 28.3');
    
    // And: Zeigt der 5. Tag "Mi 1.4" (Monatswechsel)
    await expect(page.locator('.weekday-btn').nth(4)).toContainText('Mi 1.4');
  });

  // K5: Aktiver Tag wird hervorgehoben (Edit-Formular)
  test('sollte vorhandenes planned_date in den nächsten 10 Tagen als aktiv markieren', async ({ page }) => {
    // Given: Heute ist Montag, 30.03.2026
    
    // Erst Rezept anlegen mit Datum übermorgen (Mi 1.4)
    await page.goto('/recipes/new');
    await page.fill('input[name="title"]', 'Test aktiver Tag');
    await page.check('input[name="categories"][value="Mittagessen"]');
    await page.fill('input[name="planned_date"]', '1.4.2026');
    await page.click('button[type="submit"]');
    await expect(page).toHaveURL(/\/recipes\/\d+/);
    
    // When: Der Nutzer die Bearbeiten-Seite öffnet
    await page.locator('a[href*="/edit"]').click();
    await expect(page).toHaveURL(/\/recipes\/\d+\/edit/);
    
    // Then: Ist der Tag "Mi 1.4" als aktiv markiert (Offset 1 = Mi 1.4)
    await expect(page.locator('.weekday-btn').nth(1)).toHaveAttribute('aria-pressed', 'true');
    await expect(page.locator('.weekday-btn').nth(1)).toHaveClass(/active/);
  });

  // K6: Erneuter Klick auf aktiven Tag leert das Datum
  test('sollte bei erneutem Klick auf aktiven Tag das Datum leeren', async ({ page }) => {
    // Given: Das Formular ist geöffnet
    await page.goto('/recipes/new');
    
    // Der Nutzer hat einen Tag gewählt
    await page.locator('.weekday-btn').first().click();
    await expect(page.locator('input[name="planned_date"]')).not.toHaveValue('');
    
    // When: Der Nutzer erneut auf denselben Tag klickt
    await page.locator('.weekday-btn').first().click();
    
    // Then: Ist das Datumsfeld leer
    await expect(page.locator('input[name="planned_date"]')).toHaveValue('');
    
    // And: Kein Tag ist als aktiv markiert
    const allButtons = page.locator('.weekday-btn');
    const count = await allButtons.count();
    for (let i = 0; i < count; i++) {
      await expect(allButtons.nth(i)).toHaveAttribute('aria-pressed', 'false');
    }
  });

  // K7: Manuelle Eingabe demarkiert alle Tags
  test('sollte bei manueller Eingabe alle Tags demarkieren', async ({ page }) => {
    // Given: Ein Tag ist aktiv markiert
    await page.goto('/recipes/new');
    await page.locator('.weekday-btn').first().click();
    await expect(page.locator('.weekday-btn').first()).toHaveAttribute('aria-pressed', 'true');
    
    // When: Der Nutzer das Datumsfeld manuell auf ein anderes Datum ändert
    await page.fill('input[name="planned_date"]', '15.3.2026');
    
    // Then: Sind alle Tags demarkiert
    const allButtons = page.locator('.weekday-btn');
    const count = await allButtons.count();
    for (let i = 0; i < count; i++) {
      await expect(allButtons.nth(i)).toHaveAttribute('aria-pressed', 'false');
    }
  });

  // K8: Ohne JavaScript — Fallback funktioniert
  test('sollte ohne JavaScript keine Tag-Buttons anzeigen', async ({ browser }) => {
    // Given: JavaScript ist deaktiviert
    const context = await browser.newContext({ javaScriptEnabled: false });
    const page = await context.newPage();
    
    // When: Der Nutzer die Rezept-Bearbeiten-Seite öffnet
    await page.goto('/recipes/new');
    
    // Then: Sind keine Tag-Buttons sichtbar
    await expect(page.locator('.weekday-btn')).toHaveCount(0);
    
    // And: Das Datumsfeld ist vorhanden und funktioniert
    await expect(page.locator('input[name="planned_date"]')).toBeVisible();
    
    await context.close();
  });

  // Barrierefreiheit: aria-label und Tastatur-Navigation
  test('sollte barrierefreie Attribute haben und Buttons erreichbar sein', async ({ page }) => {
    await page.goto('/recipes/new');
    
    // Jeder Button hat aussagekräftiges aria-label
    const firstButton = page.locator('.weekday-btn').nth(0);
    await expect(firstButton).toHaveAttribute('aria-label', /wählen/);
    
    // Alle Buttons haben aria-pressed
    const allButtons = page.locator('.weekday-btn');
    const count = await allButtons.count();
    for (let i = 0; i < count; i++) {
      await expect(allButtons.nth(i)).toHaveAttribute('aria-pressed');
    }
  });

});