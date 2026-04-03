import { test, expect } from '@playwright/test';

/**
 * E2E-Tests für Story 36: Markdown-Rendering in der Rezept-Detailansicht
 *
 * Alle Rezepte werden direkt über das Formular erstellt (keine separaten Seed-SQLs nötig).
 */

async function createRecipe(
  page: import('@playwright/test').Page,
  title: string,
  category: string,
  ingredients?: string,
  instructions?: string
): Promise<string> {
  // Given: Das Erstellungsformular wird aufgerufen
  await page.goto('/recipes/new');
  await page.fill('input[name="title"]', title);
  await page.check(`input[name="categories"][value="${category}"]`);
  if (ingredients !== undefined) {
    await page.fill('textarea[name="ingredients"]', ingredients);
  }
  if (instructions !== undefined) {
    await page.fill('textarea[name="instructions"]', instructions);
  }
  await page.click('button[type="submit"]');
  await expect(page).toHaveURL(/\/recipes\/\d+/);
  return page.url();
}

test.describe('Markdown-Rendering in der Rezept-Detailansicht (Story 36)', () => {

  test('K1: Aufzählungsliste in Zutaten wird als <ul> gerendert', async ({ page }) => {
    // Given: Ein Rezept mit Zutaten als Aufzählungsliste wurde erstellt
    const suffix = Date.now();
    await createRecipe(
      page,
      `Pfannkuchen ${suffix}`,
      'Kuchen',
      '- 500g Mehl\n- 1 Ei\n- 250ml Milch'
    );

    // When: Die Detailseite wird aufgerufen (bereits navigiert)
    // Then: section.ingredients enthält <ul> mit 3 <li>-Elementen
    const ingredientsSection = page.locator('section.ingredients');
    await expect(ingredientsSection).toBeVisible();

    const listItems = ingredientsSection.locator('ul li');
    await expect(listItems).toHaveCount(3);

    // And: Markdown-Syntax "- " ist nicht als Rohtext sichtbar
    const ingredientsText = await ingredientsSection.textContent();
    expect(ingredientsText).not.toContain('- 500g Mehl');
    expect(ingredientsText).toContain('500g Mehl');
  });

  test('K2: Nummerierte Liste in Zubereitung wird als <ol> gerendert', async ({ page }) => {
    // Given: Ein Rezept mit nummerierter Zubereitungsliste
    const suffix = Date.now();
    await createRecipe(
      page,
      `Brot ${suffix}`,
      'Brot',
      undefined,
      '1. Ofen vorheizen\n2. Teig kneten\n3. Backen'
    );

    // When: Die Detailseite wird aufgerufen
    // Then: section.instructions enthält <ol> mit 3 <li>-Elementen
    const instructionsSection = page.locator('section.instructions');
    await expect(instructionsSection).toBeVisible();

    const listItems = instructionsSection.locator('ol li');
    await expect(listItems).toHaveCount(3);
  });

  test('K3: Fettschrift in Zubereitung wird als <strong> gerendert', async ({ page }) => {
    // Given: Ein Rezept mit Fettschrift in der Zubereitung
    const suffix = Date.now();
    await createRecipe(
      page,
      `Auflauf ${suffix}`,
      'Mittagessen',
      undefined,
      '**Wichtig:** Ofen vorheizen auf 180°C'
    );

    // When: Die Detailseite wird aufgerufen
    // Then: <strong>-Element mit Text "Wichtig:" ist sichtbar
    const strongElement = page.locator('section.instructions strong');
    await expect(strongElement).toBeVisible();
    await expect(strongElement).toContainText('Wichtig:');

    // And: Text "**Wichtig:**" ist nicht als Rohtext sichtbar
    const instructionsText = await page.locator('section.instructions').textContent();
    expect(instructionsText).not.toContain('**Wichtig:**');
  });

  test('K4: Checkboxen in Zutaten werden dargestellt', async ({ page }) => {
    // Given: Ein Rezept mit Checkbox-Zutaten
    const suffix = Date.now();
    await createRecipe(
      page,
      `Einkaufsliste ${suffix}`,
      'Snacks',
      '- [ ] Mehl\n- [x] Eier'
    );

    // When: Die Detailseite wird aufgerufen
    // Then: Zwei input[type="checkbox"] sind in section.ingredients sichtbar
    const checkboxes = page.locator('section.ingredients input[type="checkbox"]');
    await expect(checkboxes).toHaveCount(2);

    // And: Erste ist nicht angehakt, zweite ist angehakt
    const firstCheckbox = checkboxes.nth(0);
    const secondCheckbox = checkboxes.nth(1);
    await expect(firstCheckbox).not.toBeChecked();
    await expect(secondCheckbox).toBeChecked();
  });

  test('K8: Fließtext ohne Markdown-Syntax bleibt lesbar', async ({ page }) => {
    // Given: Ein Rezept mit reinem Fließtext in der Zubereitung
    const suffix = Date.now();
    const text = 'Gemüse schneiden und in der Brühe weichkochen.';
    await createRecipe(
      page,
      `Suppe ${suffix}`,
      'Mittagessen',
      undefined,
      text
    );

    // When: Die Detailseite wird aufgerufen
    // Then: Text ist vollständig in einem <p>-Element sichtbar
    const instructionsSection = page.locator('section.instructions');
    await expect(instructionsSection).toBeVisible();
    await expect(instructionsSection).toContainText('Gemüse schneiden');
  });

  test('K9: Leere Felder - Abschnitte werden ausgeblendet', async ({ page }) => {
    // Given: Ein Rezept ohne Zutaten und Zubereitung
    const suffix = Date.now();
    await createRecipe(page, `Minimal ${suffix}`, 'Snacks');

    // When: Die Detailseite wird aufgerufen
    // Then: section.ingredients ist nicht vorhanden
    await expect(page.locator('section.ingredients')).not.toBeVisible();
    // And: section.instructions ist nicht vorhanden
    await expect(page.locator('section.instructions')).not.toBeVisible();
  });

  test('K10: HTML-Inhalte werden nicht ausgeführt (XSS-Schutz)', async ({ page }) => {
    // Given: Ein Rezept mit Script-Tag in den Zutaten (XSS-Versuch)
    const suffix = Date.now();
    let alertFired = false;
    page.on('dialog', async (dialog) => {
      alertFired = true;
      await dialog.dismiss();
    });

    await createRecipe(
      page,
      `XSS-Test ${suffix}`,
      'Snacks',
      '<script>alert(1)</script>Mehl'
    );

    // When: Die Detailseite aufgerufen wird
    // Then: kein alert() wurde ausgeführt
    expect(alertFired).toBe(false);

    // And: <script> ist nicht im DOM als aktives Element
    const scriptElements = await page.locator('section.ingredients script').count();
    expect(scriptElements).toBe(0);
  });

  test('K5: Überschrift wird gerendert', async ({ page }) => {
    // Given: Ein Rezept mit Markdown-Überschrift in der Zubereitung
    const suffix = Date.now();
    await createRecipe(
      page,
      `Strukturiertes Rezept ${suffix}`,
      'Mittagessen',
      undefined,
      '## Vorbereitung\nOfen vorheizen.\n\n## Zubereitung\nKochen.'
    );

    // When: Die Detailseite wird aufgerufen
    // Then: h2-Element ist innerhalb .markdown-content der Zubereitung sichtbar
    const headings = page.locator('section.instructions .markdown-content h2');
    await expect(headings.first()).toBeVisible();
    await expect(headings.first()).toContainText('Vorbereitung');
  });

  test('Markdown-Content ist in .markdown-content div eingebettet', async ({ page }) => {
    // Given: Ein Rezept mit Zutaten wurde erstellt
    const suffix = Date.now();
    await createRecipe(
      page,
      `Container-Test ${suffix}`,
      'Mittagessen',
      '- Zutat A\n- Zutat B'
    );

    // When: Die Detailseite wird aufgerufen
    // Then: .markdown-content div ist vorhanden und enthält den gerenderten Inhalt
    const markdownDiv = page.locator('section.ingredients .markdown-content');
    await expect(markdownDiv).toBeVisible();
    await expect(markdownDiv.locator('ul')).toBeVisible();
  });

});
