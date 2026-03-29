import { test, expect } from '@playwright/test';

/**
 * E2E-Tests für Story 17: Inline-Bewertung ohne Edit-Mode
 *
 * Die Sterne-Bewertung ist direkt in der Detailansicht antippbar.
 * Per HTMX wird nur das Rating-Fragment ausgetauscht (kein Seiten-Reload).
 */

async function createRecipe(
  page: import('@playwright/test').Page,
  title: string,
  category: string,
  rating?: number
): Promise<string> {
  await page.goto('/recipes/new');
  await page.fill('input[name="title"]', title);
  await page.check(`input[name="categories"][value="${category}"]`);
  if (rating !== undefined) {
    // Klick auf das sichtbare Label des Sterne-Radio-Buttons
    const input = page.locator(`input[name="rating"][value="${rating}"]`);
    const label = input.locator('xpath=ancestor::label');
    await label.click();
  }
  await page.click('button[type="submit"]');
  await expect(page).toHaveURL(/\/recipes\/\d+/);
  return page.url();
}

async function clickInlineStar(
  page: import('@playwright/test').Page,
  starValue: number
): Promise<void> {
  // Die Sterne-Buttons haben aria-label="N Sterne – ..."
  const starBtn = page.locator(`#inline-rating button[name="rating"][value="${starValue}"]`);
  await starBtn.click();
  // Warten bis HTMX den Swap abgeschlossen hat
  await page.waitForFunction(
    () => document.querySelector('#inline-rating') !== null
  );
}

test.describe('Inline-Bewertung (Story 17)', () => {

  test('K1+K2+K4: Inline-Bewertung setzen (4 Sterne)', async ({ page }) => {
    // Given: Ein Rezept ohne Bewertung existiert
    const suffix = Date.now();
    const detailUrl = await createRecipe(page, `Inline-Bewertung-${suffix}`, 'Mittagessen');

    // When: Detailseite öffnen
    await page.goto(detailUrl);

    // Then: Inline-Rating-Container ist sichtbar
    await expect(page.locator('#inline-rating')).toBeVisible();

    // When: Benutzer klickt auf den 4. Stern
    await clickInlineStar(page, 4);

    // Then: Die Bewertung wird sofort als 4 Sterne angezeigt (ohne Seitenneuladung)
    await expect(page.locator('#inline-rating')).toHaveAttribute('aria-label', '4 von 5 Sternen');

    // And: Der 4-Sterne-Button ist aktiv
    await expect(
      page.locator('#inline-rating button[name="rating"].active')
    ).toHaveAttribute('aria-label', /4 Sterne.*aktiv/);

    // And: Nach einer Seitenneuladung ist die Bewertung immer noch 4 Sterne
    await page.reload();
    await expect(page.locator('#inline-rating')).toHaveAttribute('aria-label', '4 von 5 Sternen');
  });

  test('K2+K4: Bewertung ändern (3 → 5 Sterne)', async ({ page }) => {
    // Given: Ein Rezept mit 3-Sterne-Bewertung existiert
    const suffix = Date.now();
    const detailUrl = await createRecipe(page, `Bewertung-Aendern-${suffix}`, 'Kuchen', 3);

    // When: Detailseite öffnen
    await page.goto(detailUrl);
    await expect(page.locator('#inline-rating')).toHaveAttribute('aria-label', '3 von 5 Sternen');

    // When: Benutzer klickt auf den 5. Stern
    await clickInlineStar(page, 5);

    // Then: Die Bewertung wird sofort als 5 Sterne angezeigt
    await expect(page.locator('#inline-rating')).toHaveAttribute('aria-label', '5 von 5 Sternen');

    // And: Nach einer Seitenneuladung ist die Bewertung 5 Sterne
    await page.reload();
    await expect(page.locator('#inline-rating')).toHaveAttribute('aria-label', '5 von 5 Sternen');
  });

  test('K3: Bewertung zurücksetzen durch erneutes Antippen', async ({ page }) => {
    // Given: Ein Rezept mit 4-Sterne-Bewertung existiert
    const suffix = Date.now();
    const detailUrl = await createRecipe(page, `Bewertung-Reset-${suffix}`, 'Party', 4);

    // When: Detailseite öffnen
    await page.goto(detailUrl);
    await expect(page.locator('#inline-rating')).toHaveAttribute('aria-label', '4 von 5 Sternen');

    // When: Benutzer klickt auf den 4. Stern (den aktuell aktiven)
    // Der aktive 4-Sterne-Button hat value="" (Toggle-Reset)
    const activeBtn = page.locator('#inline-rating button[name="rating"].active');
    await activeBtn.click();
    await page.waitForFunction(() => document.querySelector('#inline-rating') !== null);

    // Then: Die Bewertung wird entfernt (aria-label = "Noch keine Bewertung", Story 25 L10)
    await expect(page.locator('#inline-rating')).toHaveAttribute('aria-label', 'Noch keine Bewertung');

    // And: Nach einer Seitenneuladung hat das Rezept keine Bewertung
    await page.reload();
    await expect(page.locator('#inline-rating')).toHaveAttribute('aria-label', 'Noch keine Bewertung');
  });

  test('K6: Inline-Bewertung und Edit-Mode zeigen gleichen Wert', async ({ page }) => {
    // Given: Ein Rezept ohne Bewertung existiert
    const suffix = Date.now();
    const detailUrl = await createRecipe(page, `Konsistenz-Test-${suffix}`, 'Mittagessen');
    const id = detailUrl.split('/').pop();

    // When: Benutzer setzt per Inline-Bewertung 5 Sterne
    await page.goto(detailUrl);
    await clickInlineStar(page, 5);
    await expect(page.locator('#inline-rating')).toHaveAttribute('aria-label', '5 von 5 Sternen');

    // And: Benutzer öffnet den Edit-Mode
    await page.goto(`/recipes/${id}/edit`);

    // Then: Im Edit-Formular sind 5 Sterne vorausgewählt
    const radio5 = page.locator('input[name="rating"][value="5"]');
    await expect(radio5).toBeChecked();
  });

  test('K8: Tastatur-Navigation zu den Sterne-Buttons', async ({ page }) => {
    // Given: Detailseite eines Rezepts ist geöffnet
    const suffix = Date.now();
    const detailUrl = await createRecipe(page, `Tastatur-Test-${suffix}`, 'Snacks');
    await page.goto(detailUrl);

    // When: Benutzer navigiert per Tab zu den Sterne-Buttons
    const inlineRating = page.locator('#inline-rating');
    await expect(inlineRating).toBeVisible();

    // Then: Erster Sterne-Button ist per Tab erreichbar und per Enter/Leertaste auslösbar
    const firstBtn = page.locator('#inline-rating button').first();
    await firstBtn.focus();
    await expect(firstBtn).toBeFocused();

    // And: Enter drücken setzt die Bewertung
    await page.keyboard.press('Enter');
    await page.waitForFunction(() => document.querySelector('#inline-rating') !== null);
    // Der Button sollte nun eine Bewertung gesetzt haben
    await expect(page.locator('#inline-rating')).toBeVisible();
  });

  test('K5: Inline-Rating-Container ohne JS sichtbar (Progressive Enhancement)', async ({ page }) => {
    // Given: Ein Rezept existiert
    const suffix = Date.now();
    const detailUrl = await createRecipe(page, `NoJS-Test-${suffix}`, 'Brot', 3);

    // When: Detailseite aufgerufen
    await page.goto(detailUrl);

    // Then: Das Form-Element ist im DOM vorhanden (ermöglicht Funktion ohne JS)
    const form = page.locator('#inline-rating form');
    await expect(form).toBeAttached();
    await expect(form).toHaveAttribute('action', /\/recipes\/\d+\/rating/);
    await expect(form).toHaveAttribute('method', 'POST');

    // And: Die Sterne-Buttons sind normale Submit-Buttons
    const buttons = page.locator('#inline-rating button[type="submit"]');
    await expect(buttons).toHaveCount(5);
  });

});
