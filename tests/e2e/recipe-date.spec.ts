import { test, expect } from '@playwright/test';

test.describe('Rezept Datum-Eingabe', () => {

  // K1/K3: Datum beim Erstellen eingeben und in der Detailansicht anzeigen
  test('sollte Datum beim Erstellen eines Rezepts speichern und anzeigen', async ({ page }) => {
    // Given: Das Formular für ein neues Rezept ist geöffnet
    await page.goto('/recipes/new');
    await expect(page.locator('input[name="planned_date"]')).toBeVisible();

    // When: Titel, Kategorie und Datum "5.3.2025" ausgefüllt und gespeichert werden
    await page.fill('input[name="title"]', 'Rezept mit Datum');
    await page.check('input[name="categories"][value="Mittagessen"]');
    await page.fill('input[name="planned_date"]', '5.3.2025');
    await page.click('button[type="submit"]');

    // Then: Detailseite zeigt das Datum "März 2025" an
    await expect(page).toHaveURL(/\/recipes\/\d+/);
    await expect(page.locator('body')).toContainText('März');
    await expect(page.locator('body')).toContainText('2025');
  });

  // K2: Datum beim Bearbeiten vorausfüllen und ändern
  test('sollte Datum beim Bearbeiten vorausfüllen und änderbar sein', async ({ page }) => {
    // Given: Ein Rezept mit Datum "1.1.2025" wird erstellt
    await page.goto('/recipes/new');
    await page.fill('input[name="title"]', 'Rezept zum Bearbeiten');
    await page.check('input[name="categories"][value="Brot"]');
    await page.fill('input[name="planned_date"]', '1.1.2025');
    await page.click('button[type="submit"]');
    await expect(page).toHaveURL(/\/recipes\/\d+/);

    // When: Das Bearbeitungsformular geöffnet wird (via Edit-Link in der Detailseite)
    await page.locator('a[href*="/edit"]').click();
    await expect(page).toHaveURL(/\/recipes\/\d+\/edit/);

    // Then: Das Datum-Feld ist mit "1.1.2025" vorausgefüllt
    await expect(page.locator('input[name="planned_date"]')).toHaveValue('1.1.2025');

    // When: Das Datum auf "15.4.2026" geändert und gespeichert wird
    await page.fill('input[name="planned_date"]', '15.4.2026');
    await page.click('button[type="submit"]');

    // Then: Die Detailseite zeigt "April 2026"
    await expect(page).toHaveURL(/\/recipes\/\d+/);
    await expect(page.locator('body')).toContainText('April');
    await expect(page.locator('body')).toContainText('2026');
  });

  // K3 (Löschen): Datum leeren und Datum verschwindet
  test('sollte Datum löschen wenn das Feld geleert wird', async ({ page }) => {
    // Given: Ein Rezept mit Datum "1.1.2025" existiert
    await page.goto('/recipes/new');
    await page.fill('input[name="title"]', 'Rezept Datum löschen');
    await page.check('input[name="categories"][value="Kuchen"]');
    await page.fill('input[name="planned_date"]', '1.1.2025');
    await page.click('button[type="submit"]');
    await expect(page).toHaveURL(/\/recipes\/\d+/);
    await expect(page.locator('body')).toContainText('Januar');

    // When: Das Bearbeitungsformular geöffnet und das Datum geleert wird
    await page.click('text=Bearbeiten');
    await page.fill('input[name="planned_date"]', '');
    await page.click('button[type="submit"]');

    // Then: Die Detailseite zeigt kein Datum mehr an
    await expect(page).toHaveURL(/\/recipes\/\d+/);
    await expect(page.locator('body')).not.toContainText('Januar');
  });

  // K5: Ungültiges Datum → Fehlermeldung, kein Datenverlust
  test('sollte Fehlermeldung bei ungültigem Datum anzeigen und Formularwerte behalten', async ({ page }) => {
    // Given: Das Formular für ein neues Rezept ist geöffnet
    await page.goto('/recipes/new');

    // When: Ungültiges Datum "morgen" eingegeben und gespeichert wird
    await page.fill('input[name="title"]', 'Mein Lieblingsrezept');
    await page.check('input[name="categories"][value="Snacks"]');
    await page.fill('textarea[name="ingredients"]', 'Zutat A, Zutat B');
    await page.fill('input[name="planned_date"]', 'morgen');
    await page.click('button[type="submit"]');

    // Then: Fehlermeldung erscheint, andere Felder behalten ihre Werte
    await expect(page.locator('.errors')).toContainText('Kein gültiges Datum');
    await expect(page.locator('input[name="title"]')).toHaveValue('Mein Lieblingsrezept');
    await expect(page.locator('textarea[name="ingredients"]')).toHaveValue('Zutat A, Zutat B');
    await expect(page.locator('input[name="planned_date"]')).toHaveValue('morgen');
  });

  // K4: Kalender-Icon ist vorhanden
  test('sollte Kalender-Icon neben dem Datums-Eingabefeld anzeigen', async ({ page }) => {
    // Given: Das Formular für ein neues Rezept ist geöffnet
    await page.goto('/recipes/new');

    // Then: Kalender-Icon (Button mit aria-label) ist sichtbar
    await expect(page.locator('button[aria-label="Kalender öffnen"]')).toBeVisible();
  });

  // K6: Datum in der Detailansicht
  test('sollte Datum in der Detailansicht anzeigen', async ({ page }) => {
    // Given: Ein Rezept mit Datum "15.6.2025" wurde erstellt
    await page.goto('/recipes/new');
    await page.fill('input[name="title"]', 'Detail-Datum-Rezept');
    await page.check('input[name="categories"][value="Party"]');
    await page.fill('input[name="planned_date"]', '15.6.2025');
    await page.click('button[type="submit"]');

    // When: Die Detailseite aufgerufen wird
    await expect(page).toHaveURL(/\/recipes\/\d+/);

    // Then: Das Datum ist in einem deutschen Format sichtbar (langer Monatsname)
    await expect(page.locator('body')).toContainText('Juni');
    await expect(page.locator('body')).toContainText('2025');
  });

  // K7: Datum in der Listenansicht
  test('sollte Datum in der Rezeptliste anzeigen', async ({ page }) => {
    // Given: Ein Rezept mit Datum "20.12.2025" wurde erstellt
    await page.goto('/recipes/new');
    await page.fill('input[name="title"]', 'Listen-Datum-Rezept');
    await page.check('input[name="categories"][value="Mittagessen"]');
    await page.fill('input[name="planned_date"]', '20.12.2025');
    await page.click('button[type="submit"]');

    // When: Die Startseite (Listenansicht) aufgerufen wird
    await page.goto('/');

    // Then: Das kompakte Datum "20.12.2025" ist in der Liste sichtbar
    await expect(page.locator('.recipe-date').filter({ hasText: '20.12.2025' }).first()).toBeVisible();
  });

  // Datum-Feld Label und Barrierefreiheit
  test('sollte korrektes Label für das Datums-Feld haben', async ({ page }) => {
    // Given: Das Formular für ein neues Rezept ist geöffnet
    await page.goto('/recipes/new');

    // Then: Das Label "Datum (geplant / gekocht)" ist sichtbar
    await expect(page.locator('label[for="planned_date"]')).toContainText('Datum (geplant / gekocht)');
  });
});
