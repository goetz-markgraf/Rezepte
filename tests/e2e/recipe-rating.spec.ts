import { test, expect } from '@playwright/test';

test.describe('Rezept-Bewertung', () => {
  async function selectRating(page: import('@playwright/test').Page, rating: number): Promise<void> {
    // Klick auf das sichtbare Label des Sterne-Radio-Buttons (Input ist visuell versteckt)
    const input = page.locator(`input[name="rating"][value="${rating}"]`);
    const label = input.locator('xpath=ancestor::label');
    await label.click();
  }

  async function createRecipe(
    page: import('@playwright/test').Page,
    title: string,
    category: string,
    rating?: number
  ): Promise<string> {
    // Given: Neues-Rezept-Formular öffnen
    await page.goto('/recipes/new');
    await page.fill('input[name="title"]', title);
    await page.check(`input[name="categories"][value="${category}"]`);
    if (rating !== undefined) {
      await selectRating(page, rating);
    }
    await page.click('button[type="submit"]');
    await expect(page).toHaveURL(/\/recipes\/\d+/);
    return page.url();
  }

  test('K1: Bewertungsfeld im Bearbeitungsformular vorhanden', async ({ page }) => {
    // Given: Ein Rezept wurde erstellt
    const suffix = Date.now();
    const detailUrl = await createRecipe(page, `Rating-Test-${suffix}`, 'Mittagessen');

    // When: Bearbeitungsformular öffnen
    await page.goto(detailUrl.replace('/recipes/', '/recipes/') + '/edit');

    // Then: Bewertungsfeld ist vorhanden
    await expect(page.locator('fieldset.star-rating')).toBeVisible();
    await expect(page.locator('input[name="rating"][value="1"]')).toBeAttached();
    await expect(page.locator('input[name="rating"][value="5"]')).toBeAttached();
    await expect(page.locator('input[name="rating"][value=""]')).toBeAttached();
  });

  test('K2: Bewertung setzen und speichern (4 Sterne)', async ({ page }) => {
    // Given: Ein Rezept ohne Bewertung existiert
    const suffix = Date.now();
    const detailUrl = await createRecipe(page, `Bewertung-Setzen-${suffix}`, 'Kuchen');
    const id = detailUrl.split('/').pop();

    // When: Bearbeitungsformular öffnen, 4 Sterne auswählen und speichern
    await page.goto(`/recipes/${id}/edit`);
    await selectRating(page, 4);
    await page.click('button[type="submit"]');

    // Then: Detail-Seite zeigt 4 Sterne (Inline-Rating-Container)
    await expect(page).toHaveURL(/\/recipes\/\d+/);
    await expect(page.locator('#inline-rating')).toBeVisible();
    await expect(page.locator('#inline-rating')).toHaveAttribute('aria-label', '4 von 5 Sternen');
  });

  test('K2: 1 Stern speichern (Negativbewertung)', async ({ page }) => {
    // Given: Neues Rezept mit 1-Stern-Bewertung
    const suffix = Date.now();
    await createRecipe(page, `Ein-Stern-${suffix}`, 'Snacks', 1);

    // Then: Detail-Seite zeigt 1 Stern (Inline-Rating-Container)
    await expect(page.locator('#inline-rating')).toBeVisible();
    await expect(page.locator('#inline-rating')).toHaveAttribute('aria-label', '1 von 5 Sternen');
  });

  test('K3: Bewertung in Detailansicht (5 Sterne)', async ({ page }) => {
    // Given: Rezept mit 5-Sterne-Bewertung erstellt
    const suffix = Date.now();
    await createRecipe(page, `Fuenf-Sterne-${suffix}`, 'Mittagessen', 5);

    // When: Detail-Seite ist geöffnet (schon nach createRecipe)
    // Then: 5 ausgefüllte Sterne sind sichtbar (Inline-Rating-Container)
    await expect(page.locator('#inline-rating')).toBeVisible();
    await expect(page.locator('#inline-rating')).toHaveAttribute('aria-label', '5 von 5 Sternen');
    // Alle 5 Sterne-Buttons zeigen ★ (ausgefüllt)
    const starBtns = page.locator('#inline-rating button');
    await expect(starBtns).toHaveCount(5);
    for (const btn of await starBtns.all()) {
      expect(await btn.textContent()).toContain('★');
    }
  });

  test('K3: Kein Sterne-Block bei unbewerteten Rezepten (Detailansicht)', async ({ page }) => {
    // Given: Rezept ohne Bewertung erstellt
    const suffix = Date.now();
    await createRecipe(page, `Unbewertet-Detail-${suffix}`, 'Brot');

    // When: Detail-Seite ist geöffnet
    // Then: Inline-Rating-Container vorhanden, aber ohne aria-label (keine Bewertung)
    await expect(page.locator('#inline-rating')).toBeVisible();
    await expect(page.locator('#inline-rating')).not.toHaveAttribute('aria-label');
  });

  test('K4: Bewertung in der Listenansicht', async ({ page }) => {
    // Given: Rezept mit 5 Sternen und Rezept ohne Bewertung
    const suffix = Date.now();
    await createRecipe(page, `Sterne-Liste-${suffix}`, 'Party', 5);
    await createRecipe(page, `Keine-Sterne-Liste-${suffix}`, 'Party');

    // When: Rezeptliste öffnen
    await page.goto('/');

    // Then: Rezept mit Bewertung zeigt Sterne
    const sternTitle = page.getByText(`Sterne-Liste-${suffix}`).locator('..');
    await expect(sternTitle.locator('.recipe-stars')).toBeVisible();

    // And: Unbewertetes Rezept zeigt keine Sterne
    const keineSternTitle = page.getByText(`Keine-Sterne-Liste-${suffix}`).locator('..');
    await expect(keineSternTitle.locator('.recipe-stars')).not.toBeAttached();
  });

  test('K5: Bewertung zurücksetzen auf "Keine Bewertung"', async ({ page }) => {
    // Given: Rezept mit 5-Sterne-Bewertung
    const suffix = Date.now();
    const detailUrl = await createRecipe(page, `Reset-Bewertung-${suffix}`, 'Mittagessen', 5);
    const id = detailUrl.split('/').pop();

    // When: Bearbeiten → "Keine Bewertung" wählen → speichern
    await page.goto(`/recipes/${id}/edit`);
    const noRatingInput = page.locator('input[name="rating"][value=""]');
    const noRatingLabel = noRatingInput.locator('xpath=ancestor::label');
    await noRatingLabel.click();
    await page.click('button[type="submit"]');

    // Then: Detail-Seite zeigt keinen bewerteten Zustand mehr (kein aria-label)
    await expect(page).toHaveURL(/\/recipes\/\d+/);
    await expect(page.locator('#inline-rating')).toBeVisible();
    await expect(page.locator('#inline-rating')).not.toHaveAttribute('aria-label');
  });

  test('K6: Negativbewertung (1-2 Sterne) speicherbar und sichtbar', async ({ page }) => {
    // Given: Neues Rezept mit 2 Sternen
    const suffix = Date.now();
    await createRecipe(page, `Zwei-Sterne-${suffix}`, 'Snacks', 2);

    // Then: Rezept ist in Detailansicht sichtbar und zeigt 2 Sterne (Inline-Rating-Container)
    await expect(page.locator('#inline-rating')).toBeVisible();
    await expect(page.locator('#inline-rating')).toHaveAttribute('aria-label', '2 von 5 Sternen');

    // And: Rezept ist in der Liste sichtbar
    await page.goto('/');
    await expect(page.getByText(`Zwei-Sterne-${suffix}`)).toBeVisible();
  });

  test('K1: Formular vorausgefüllt mit aktueller Bewertung', async ({ page }) => {
    // Given: Rezept mit 3 Sternen existiert
    const suffix = Date.now();
    const detailUrl = await createRecipe(page, `Vorausgefuellt-${suffix}`, 'Mittagessen', 3);
    const id = detailUrl.split('/').pop();

    // When: Bearbeitungsformular öffnen
    await page.goto(`/recipes/${id}/edit`);

    // Then: Radio-Button für 3 Sterne ist ausgewählt
    const radio3 = page.locator('input[name="rating"][value="3"]');
    await expect(radio3).toBeChecked();

    // And: Andere Sterne-Buttons sind nicht ausgewählt
    await expect(page.locator('input[name="rating"][value="1"]')).not.toBeChecked();
    await expect(page.locator('input[name="rating"][value="5"]')).not.toBeChecked();
  });

  test('K8: Keyboard-Navigation durch Sterne-Auswahl', async ({ page }) => {
    // Given: Bearbeitungsformular ist geöffnet
    const suffix = Date.now();
    const detailUrl = await createRecipe(page, `Keyboard-Test-${suffix}`, 'Mittagessen');
    const id = detailUrl.split('/').pop();
    await page.goto(`/recipes/${id}/edit`);

    // When: Tab-Taste zur Sterne-Gruppe navigieren
    const ratingGroup = page.locator('fieldset.star-rating');
    await expect(ratingGroup).toBeVisible();

    // Then: Sterne-Gruppe ist per Tastatur erreichbar
    const firstRadio = page.locator('input[name="rating"]').first();
    await firstRadio.focus();
    await expect(firstRadio).toBeFocused();

    // And: Pfeiltasten wechseln zwischen den Sternen
    await page.keyboard.press('ArrowRight');
    const secondRadio = page.locator('input[name="rating"]').nth(1);
    await expect(secondRadio).toBeFocused();
  });
});
