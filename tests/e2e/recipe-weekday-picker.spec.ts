import { test, expect } from '@playwright/test';

test.describe.skip('Wochentag-Picker (Story 16 - ersetzt durch Story 29)', () => {

  // Diese Tests wurden durch Story 29 ersetzt
  // Das neue System zeigt 10 Tage ab morgen, nicht 7 Tage ab Montag

  // K1/K2: Wochentag-Buttons werden angezeigt (mit JS)
  test('sollte sieben Wochentag-Buttons Mo–So anzeigen', async ({ page }) => {
    // Given: Das Formular für ein neues Rezept ist geöffnet (JS aktiv)
    await page.goto('/recipes/new');

    // When: Die Seite lädt
    // Then: Sieben Wochentag-Buttons Mo–So sind sichtbar
    await expect(page.locator('.weekday-btn', { hasText: 'Mo' })).toBeVisible();
    await expect(page.locator('.weekday-btn', { hasText: 'Di' })).toBeVisible();
    await expect(page.locator('.weekday-btn', { hasText: 'Mi' })).toBeVisible();
    await expect(page.locator('.weekday-btn', { hasText: 'Do' })).toBeVisible();
    await expect(page.locator('.weekday-btn', { hasText: 'Fr' })).toBeVisible();
    await expect(page.locator('.weekday-btn', { hasText: 'Sa' })).toBeVisible();
    await expect(page.locator('.weekday-btn', { hasText: 'So' })).toBeVisible();
  });

  // K2: Klick auf Wochentag-Button setzt korrektes Datum
  test('sollte beim Klick auf "Do" das Datum des nächsten Donnerstags setzen', async ({ page }) => {
    // Given: Das Formular ist geöffnet, heute ist Mittwoch 01.04.2026 (fixiert per page.clock)
    await page.clock.setFixedTime(new Date('2026-04-01T10:00:00'));
    await page.goto('/recipes/new');

    // When: Der Nutzer auf "Do" klickt
    await page.locator('.weekday-btn', { hasText: 'Do' }).click();

    // Then: Das Datumsfeld enthält das Datum des nächsten Donnerstags (nächste Woche = 9.4.2026)
    // Heute ist Mittwoch (ISO: dayOfWeek=2), daysToNextMonday = 7-2 = 5 → Montag = 6.4.2026
    // Do (offset=3): 6.4.2026 + 3 = 9.4.2026
    await expect(page.locator('input[name="planned_date"]')).toHaveValue('9.4.2026');

    // And: Der Button "Do" hat aria-pressed="true"
    await expect(page.locator('.weekday-btn', { hasText: 'Do' })).toHaveAttribute('aria-pressed', 'true');
  });

  // K3: Aktiver Button ist visuell hervorgehoben
  test('sollte geklickten Button als aktiv markieren und alle anderen demarkieren', async ({ page }) => {
    // Given: Das Formular ist geöffnet
    await page.goto('/recipes/new');

    // When: Der Nutzer auf "Di" klickt
    await page.locator('.weekday-btn', { hasText: 'Di' }).click();

    // Then: Button "Di" hat CSS-Klasse "active" und aria-pressed="true"
    await expect(page.locator('.weekday-btn', { hasText: 'Di' })).toHaveClass(/active/);
    await expect(page.locator('.weekday-btn', { hasText: 'Di' })).toHaveAttribute('aria-pressed', 'true');

    // And: Alle anderen Buttons haben aria-pressed="false"
    await expect(page.locator('.weekday-btn', { hasText: 'Mo' })).toHaveAttribute('aria-pressed', 'false');
    await expect(page.locator('.weekday-btn', { hasText: 'Mi' })).toHaveAttribute('aria-pressed', 'false');
    await expect(page.locator('.weekday-btn', { hasText: 'Do' })).toHaveAttribute('aria-pressed', 'false');
    await expect(page.locator('.weekday-btn', { hasText: 'Fr' })).toHaveAttribute('aria-pressed', 'false');
    await expect(page.locator('.weekday-btn', { hasText: 'Sa' })).toHaveAttribute('aria-pressed', 'false');
    await expect(page.locator('.weekday-btn', { hasText: 'So' })).toHaveAttribute('aria-pressed', 'false');
  });

  // K4: Erneuter Klick auf aktiven Button leert das Feld
  test('sollte bei erneutem Klick auf aktiven Button das Datumsfeld leeren', async ({ page }) => {
    // Given: Der Nutzer hat "Mo" geklickt, das Datumsfeld zeigt ein Datum
    await page.goto('/recipes/new');
    await page.locator('.weekday-btn', { hasText: 'Mo' }).click();
    // Sicherstellen, dass ein Datum gesetzt wurde
    const value = await page.locator('input[name="planned_date"]').inputValue();
    expect(value).not.toBe('');

    // When: Der Nutzer erneut auf "Mo" klickt
    await page.locator('.weekday-btn', { hasText: 'Mo' }).click();

    // Then: Das Datumsfeld ist leer
    await expect(page.locator('input[name="planned_date"]')).toHaveValue('');

    // And: Kein Button ist aktiv (aria-pressed="false" für alle)
    const allButtons = page.locator('.weekday-btn');
    const count = await allButtons.count();
    for (let i = 0; i < count; i++) {
      await expect(allButtons.nth(i)).toHaveAttribute('aria-pressed', 'false');
    }
  });

  // K5: Manuelle Eingabe deaktiviert Wochentag-Markierung
  test('sollte bei manueller Eingabe ohne Wochentag-Match die Markierung entfernen', async ({ page }) => {
    // Given: Der Nutzer hat "Di" gewählt, "Di" ist aktiv
    await page.goto('/recipes/new');
    await page.locator('.weekday-btn', { hasText: 'Di' }).click();
    await expect(page.locator('.weekday-btn', { hasText: 'Di' })).toHaveAttribute('aria-pressed', 'true');

    // When: Der Nutzer das Datumsfeld manuell auf "15.3.2026" ändert
    await page.fill('input[name="planned_date"]', '15.3.2026');

    // Then: Kein Button ist mehr aktiv (15.3.2026 ist kein Di nächste Woche)
    const allButtons = page.locator('.weekday-btn');
    const count = await allButtons.count();
    for (let i = 0; i < count; i++) {
      await expect(allButtons.nth(i)).toHaveAttribute('aria-pressed', 'false');
    }
  });

  // K5b: Manuelle Eingabe mit Wochentag-Match markiert Button
  test('sollte bei manueller Eingabe des nächsten Donnerstags den Button "Do" markieren', async ({ page }) => {
    // Given: Das Formular ist geöffnet, heute ist Mittwoch 01.04.2026 (fixiert)
    await page.clock.setFixedTime(new Date('2026-04-01T10:00:00'));
    await page.goto('/recipes/new');

    // When: Der Nutzer manuell "9.4.2026" eingibt (= Do nächste Woche)
    await page.fill('input[name="planned_date"]', '9.4.2026');

    // Then: Button "Do" ist aktiv markiert
    await expect(page.locator('.weekday-btn', { hasText: 'Do' })).toHaveAttribute('aria-pressed', 'true');
    await expect(page.locator('.weekday-btn', { hasText: 'Do' })).toHaveClass(/active/);
  });

  // K6: Datum wird beim Speichern korrekt gespeichert
  test('sollte das über den Wochentag-Picker gesetzte Datum korrekt speichern', async ({ page }) => {
    // Given: Das Formular ist geöffnet, heute ist Mittwoch 01.04.2026 (fixiert)
    await page.clock.setFixedTime(new Date('2026-04-01T10:00:00'));
    await page.goto('/recipes/new');

    // When: Der Nutzer auf "Fr" klickt und das Formular speichert
    // Fr nächste Woche: Montag=6.4.2026, Fr (offset=4) = 10.4.2026
    await page.fill('input[name="title"]', 'Wochentag-Picker-Test-Rezept');
    await page.check('input[name="categories"][value="Mittagessen"]');
    await page.locator('.weekday-btn', { hasText: 'Fr' }).click();
    await expect(page.locator('input[name="planned_date"]')).toHaveValue('10.4.2026');
    await page.click('button[type="submit"]');

    // Then: Die Detailansicht zeigt das korrekte Datum (Freitag nächste Woche)
    await expect(page).toHaveURL(/\/recipes\/\d+/);
    await expect(page.locator('body')).toContainText('April');
    await expect(page.locator('body')).toContainText('2026');
  });

  // K3 (Edit-Formular): Vorhandenes Datum aus nächster Woche markiert Button
  test('sollte im Edit-Formular vorhandenes Datum der nächsten Woche als aktiven Button markieren', async ({ page }) => {
    // Given: Ein Rezept hat planned_date = nächster Donnerstag, heute ist Mittwoch 01.04.2026 (fixiert)
    // Donnerstag nächste Woche = 9.4.2026
    await page.clock.setFixedTime(new Date('2026-04-01T10:00:00'));

    // Erst Rezept anlegen mit dem Datum
    await page.goto('/recipes/new');
    await page.fill('input[name="title"]', 'Edit-Datum-Picker-Test');
    await page.check('input[name="categories"][value="Brot"]');
    await page.fill('input[name="planned_date"]', '9.4.2026');
    await page.click('button[type="submit"]');
    await expect(page).toHaveURL(/\/recipes\/\d+/);

    // When: Der Nutzer die Bearbeiten-Seite öffnet (Uhr bleibt fixiert)
    await page.locator('a[href*="/edit"]').click();
    await expect(page).toHaveURL(/\/recipes\/\d+\/edit/);

    // Then: Button "Do" ist aktiv markiert
    await expect(page.locator('.weekday-btn', { hasText: 'Do' })).toHaveAttribute('aria-pressed', 'true');
    await expect(page.locator('.weekday-btn', { hasText: 'Do' })).toHaveClass(/active/);
  });

  // K1 (ohne JS): Fallback funktioniert, keine Wochentag-Buttons sichtbar
  test('sollte ohne JavaScript keine Wochentag-Buttons anzeigen', async ({ browser }) => {
    // Given: JavaScript ist deaktiviert (via Playwright context option)
    const context = await browser.newContext({ javaScriptEnabled: false });
    const page = await context.newPage();

    // When: Der Nutzer die Rezept-Bearbeiten-Seite öffnet
    await page.goto('/recipes/new');

    // Then: Keine Wochentag-Buttons sind sichtbar
    await expect(page.locator('.weekday-btn')).toHaveCount(0);

    // And: Das Datumsfeld ist vorhanden und funktioniert
    await expect(page.locator('input[name="planned_date"]')).toBeVisible();

    await context.close();
  });

});