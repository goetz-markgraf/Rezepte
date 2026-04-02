import { test, expect } from '@playwright/test';

// Hilfsfunktion: Datum formatieren wie der Picker ("Di 31.3", "Fr 3.4")
function formatPickerDate(date: Date): string {
  const weekdays = ['So', 'Mo', 'Di', 'Mi', 'Do', 'Fr', 'Sa'];
  const day = weekdays[date.getDay()];
  const d = date.getDate();
  const m = date.getMonth() + 1;
  return `${day} ${d}.${m}`;
}

// Hilfsfunktion: Datum als Eingabestring ("4.4.2026")
function formatInputDate(date: Date): string {
  return `${date.getDate()}.${date.getMonth() + 1}.${date.getFullYear()}`;
}

// Morgen berechnen
function tomorrow(): Date {
  const d = new Date();
  d.setDate(d.getDate() + 1);
  return d;
}

// Datum in N Tagen berechnen
function inDays(n: number): Date {
  const d = new Date();
  d.setDate(d.getDate() + n);
  return d;
}

test.describe('Wochen-Picker erweitert (Story 29)', () => {

  // K1: Picker zeigt 10 Tage ab morgen
  test('sollte 10 Tage anzeigen, beginnend mit morgen', async ({ page }) => {
    // Given: Das Formular ist geöffnet
    await page.goto('/recipes/new');
    
    // Then: Werden 10 Tage angezeigt
    const buttons = page.locator('.weekday-btn');
    await expect(buttons).toHaveCount(10);
    
    // Erster Button ist morgen
    const expectedFirst = formatPickerDate(tomorrow());
    await expect(buttons.nth(0)).toContainText(expectedFirst);
    
    // Letzter Button ist in 10 Tagen
    const expectedLast = formatPickerDate(inDays(10));
    await expect(buttons.nth(9)).toContainText(expectedLast);
  });

  // K2: Klick auf Tag setzt korrektes Datum
  test('sollte beim Klick auf den 5. Tag das korrekte Datum setzen', async ({ page }) => {
    // Given: Das Formular ist geöffnet
    await page.goto('/recipes/new');
    
    // When: Der Nutzer auf den 5. Tag klickt (Offset 4 = morgen + 4 Tage)
    await page.locator('.weekday-btn').nth(4).click();
    
    // Then: Enthält das Datumsfeld das korrekte Datum (morgen + 4 Tage)
    const expectedDate = formatInputDate(inDays(5));
    await expect(page.locator('input[name="planned_date"]')).toHaveValue(expectedDate);
    
    // And: Der Tag ist als aktiv markiert
    await expect(page.locator('.weekday-btn').nth(4)).toHaveAttribute('aria-pressed', 'true');
  });

  // K3: Beginn bei morgen (nicht Montag)
  test('sollte mit morgen beginnen, nicht mit dem nächsten Montag', async ({ page }) => {
    // Given: Das Formular ist geöffnet
    await page.goto('/recipes/new');
    
    // When: Der Picker wird angezeigt
    // Then: Der erste Tag ist morgen
    const firstButton = page.locator('.weekday-btn').nth(0);
    const expectedFirst = formatPickerDate(tomorrow());
    await expect(firstButton).toContainText(expectedFirst);
    
    // Es gibt KEINEN heutigen Tag als ersten Button
    const todayText = formatPickerDate(new Date());
    const firstButtonText = await firstButton.innerText();
    expect(firstButtonText).not.toContain(todayText);
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
    // Übermorgen als planned_date wählen (Offset 1 im Picker = morgen+1 = übermorgen)
    const targetDate = inDays(2);
    const targetDateInput = formatInputDate(targetDate);

    // Erst Rezept anlegen mit Datum übermorgen
    await page.goto('/recipes/new');
    await page.fill('input[name="title"]', 'Test aktiver Tag');
    await page.check('input[name="categories"][value="Mittagessen"]');
    await page.fill('input[name="planned_date"]', targetDateInput);
    await page.click('button[type="submit"]');
    await page.waitForURL(/\/recipes\/\d+$/);

    // ID aus der URL extrahieren und direkt zur Edit-Seite navigieren
    const url = page.url();
    const id = url.match(/\/recipes\/(\d+)$/)?.[1];
    await page.goto(`/recipes/${id}/edit`);
    await expect(page).toHaveURL(/\/recipes\/\d+\/edit/);

    // Warten bis die Buttons existieren (dynamisch per JS erstellt)
    await expect(page.locator('.weekday-btn')).toHaveCount(10);

    // Then: Ist der Tag übermorgen als aktiv markiert (Offset 1 im Picker = übermorgen)
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