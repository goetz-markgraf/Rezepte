import { test, expect } from '@playwright/test';
import AxeBuilder from '@axe-core/playwright';

/**
 * E2E-Tests für Story 25: WCAG 2.1 Level A Accessibility
 *
 * Prüft mit axe-core automatisiert auf Accessibility-Verstöße.
 * Testet Tastaturnavigation und korrekte ARIA-Attribute.
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
    const input = page.locator(`input[name="rating"][value="${rating}"]`);
    const label = input.locator('xpath=ancestor::label');
    await label.click();
  }
  await page.click('button[type="submit"]');
  await expect(page).toHaveURL(/\/recipes\/\d+/);
  return page.url();
}

test.describe('Accessibility (Story 25)', () => {

  test('T1: Startseite hat keine axe Level-A-Violations', async ({ page }) => {
    test.setTimeout(90_000);
    // Given: Zwei Rezepte für den Test erstellen
    const suffix = Date.now();
    await createRecipe(page, `Rezept-A11y-1-${suffix}`, 'Mittagessen', 3);
    await createRecipe(page, `Rezept-A11y-2-${suffix}`, 'Kuchen');

    // When: Startseite mit Kategorie-Filter öffnen (nur die zwei Test-Rezepte sichtbar, kleinerer DOM)
    await page.goto(`/?kategorie=Mittagessen&kategorie=Kuchen&q=Rezept-A11y-${suffix}`);
    await expect(page.locator('h1')).toContainText('Rezepte');

    // Then: Keine axe-Violations
    const results = await new AxeBuilder({ page })
      .withTags(['wcag2a', 'wcag21a'])
      .analyze();
    expect(results.violations).toEqual([]);
  });

  test('T2: Detailansicht hat keine axe Level-A-Violations', async ({ page }) => {
    // Given: Ein Rezept mit Bewertung existiert
    const suffix = Date.now();
    const detailUrl = await createRecipe(page, `Detail-A11y-${suffix}`, 'Mittagessen', 4);

    // When: Detailseite öffnen
    await page.goto(detailUrl);
    await expect(page.locator('h1')).toBeVisible();

    // Then: Keine axe-Violations
    const results = await new AxeBuilder({ page })
      .withTags(['wcag2a', 'wcag21a'])
      .analyze();
    expect(results.violations).toEqual([]);
  });

  test('T3: Detailansicht ohne Bewertung hat keine axe Level-A-Violations', async ({ page }) => {
    // Given: Ein Rezept ohne Bewertung existiert
    const suffix = Date.now();
    const detailUrl = await createRecipe(page, `Detail-NoBewertung-${suffix}`, 'Brot');

    // When: Detailseite öffnen
    await page.goto(detailUrl);

    // Then: Keine axe-Violations
    const results = await new AxeBuilder({ page })
      .withTags(['wcag2a', 'wcag21a'])
      .analyze();
    expect(results.violations).toEqual([]);
  });

  test('T4: Erstellen-Formular hat keine axe Level-A-Violations', async ({ page }) => {
    // Given: Die App ist gestartet
    // When: Neues-Rezept-Formular öffnen
    await page.goto('/recipes/new');
    await expect(page.locator('h1')).toContainText('Neues Rezept');

    // Then: Keine axe-Violations
    const results = await new AxeBuilder({ page })
      .withTags(['wcag2a', 'wcag21a'])
      .analyze();
    expect(results.violations).toEqual([]);
  });

  test('T5: Tastaturnavigation – Rezept erstellen ohne Maus (K3)', async ({ page }) => {
    // Given: Die App ist gestartet
    await page.goto('/');

    // When: Per Tab zur "Neues Rezept"-Schaltfläche navigieren und Enter drücken
    // Fokus ins Dokument setzen
    await page.locator('body').click();

    // Tab mehrfach drücken bis wir auf dem "Neues Rezept"-Link sind
    const newRecipeLink = page.locator('.actions a[href="/recipes/new"]');
    await newRecipeLink.focus();
    await page.keyboard.press('Enter');

    await expect(page).toHaveURL('/recipes/new');

    // Titel ausfüllen
    const titleInput = page.locator('input[name="title"]');
    await titleInput.fill('Tastatur-Test-Rezept');

    // Kategorie per Tastatur wählen (Space auf Checkbox)
    const checkbox = page.locator('input[name="categories"][value="Snacks"]');
    await checkbox.focus();
    await page.keyboard.press('Space');
    await expect(checkbox).toBeChecked();

    // Speichern per Tab + Enter (unteren Speichern-Button nutzen)
    const submitBtn = page.locator('button[type="submit"].btn-primary');
    await submitBtn.focus();
    await page.keyboard.press('Enter');

    // Then: Das neue Rezept erscheint (Redirect zur Detailseite)
    await expect(page).toHaveURL(/\/recipes\/\d+/);
    await expect(page.locator('h1')).toContainText('Tastatur-Test-Rezept');
  });

  test('T6: Formular-Labels sind korrekt mit Feldern verknüpft (K2)', async ({ page }) => {
    // Given: Neues-Rezept-Formular öffnen
    await page.goto('/recipes/new');

    // Then: Titel-Feld hat required-Attribut
    const titleInput = page.locator('input[name="title"]');
    await expect(titleInput).toHaveAttribute('required');
    await expect(titleInput).toHaveAttribute('aria-required', 'true');

    // Label ist programmatisch mit Feld verknüpft (via for/id)
    const titleLabel = page.locator('label[for="title"]');
    await expect(titleLabel).toBeVisible();

    // Ingredienten-Textarea hat Label
    const ingredientsLabel = page.locator('label[for="ingredients"]');
    await expect(ingredientsLabel).toBeVisible();

    // Anleitungs-Textarea hat Label
    const instructionsLabel = page.locator('label[for="instructions"]');
    await expect(instructionsLabel).toBeVisible();
  });

  test('T7: Bearbeiten-Button enthält Rezeptname im aria-label (K9)', async ({ page }) => {
    // Given: Ein Rezept mit bekanntem Titel existiert
    const suffix = Date.now();
    const title = `Spaghetti-A11y-${suffix}`;
    await createRecipe(page, title, 'Mittagessen');

    // When: Startseite öffnen
    await page.goto('/');

    // Then: Der Bearbeiten-Button enthält den Rezepttitel im aria-label
    const editBtn = page.locator(`.recipe-item-actions a.btn-icon[aria-label="${title} bearbeiten"]`);
    await expect(editBtn).toBeVisible();
  });

  test('T8: Inline-Rating ohne Bewertung hat aria-label "Noch keine Bewertung" (L10)', async ({ page }) => {
    // Given: Ein Rezept ohne Bewertung existiert
    const suffix = Date.now();
    const detailUrl = await createRecipe(page, `NoBewertung-${suffix}`, 'Party');

    // When: Detailseite öffnen
    await page.goto(detailUrl);

    // Then: Das Inline-Rating-Widget hat aria-label "Noch keine Bewertung"
    const ratingDiv = page.locator('#inline-rating');
    await expect(ratingDiv).toHaveAttribute('aria-label', 'Noch keine Bewertung');
  });

  test('T9: Inline-Rating mit Bewertung hat korrekte Sterne im aria-label (L10)', async ({ page }) => {
    // Given: Ein Rezept mit 4 Sternen existiert
    const suffix = Date.now();
    const detailUrl = await createRecipe(page, `MitBewertung-${suffix}`, 'Mittagessen', 4);

    // When: Detailseite öffnen
    await page.goto(detailUrl);

    // Then: Das Inline-Rating-Widget hat aria-label "4 von 5 Sternen"
    const ratingDiv = page.locator('#inline-rating');
    await expect(ratingDiv).toHaveAttribute('aria-label', '4 von 5 Sternen');
  });

  test('T10: Tastaturnavigation – Lösch-Bestätigung (K7)', async ({ page }) => {
    // Given: Ein Rezept existiert
    const suffix = Date.now();
    const detailUrl = await createRecipe(page, `Loeschen-A11y-${suffix}`, 'Snacks');

    // When: Detailseite öffnen
    await page.goto(detailUrl);

    // Löschen-Link per Tastatur aktivieren
    const deleteLink = page.locator('a[href*="/confirm-delete"]');
    await deleteLink.focus();
    await page.keyboard.press('Enter');

    // Then: Lösch-Bestätigungsseite erscheint
    await expect(page).toHaveURL(/\/confirm-delete/);
    await expect(page.locator('h1')).toContainText('löschen');

    // Abbrechen per Tastatur
    const cancelLink = page.locator('a', { hasText: 'Abbrechen' });
    await cancelLink.focus();
    await page.keyboard.press('Enter');

    // Then: Zurück zur Detailseite
    await expect(page).toHaveURL(/\/recipes\/\d+$/);
  });

  test('T11: Hauptnavigation hat aria-label "Hauptnavigation" (L1)', async ({ page }) => {
    // Given: Startseite
    await page.goto('/');

    // Then: Haupt-Nav hat aria-label
    const mainNav = page.locator('nav.main-nav');
    await expect(mainNav).toHaveAttribute('aria-label', 'Hauptnavigation');
  });

  test('T12: Kategorien-Fieldset ist korrekt ausgezeichnet (L3)', async ({ page }) => {
    // Given: Neues-Rezept-Formular
    await page.goto('/recipes/new');

    // Then: Kategorien sind in einem fieldset mit legend
    // Das Kategorien-Fieldset hat nur die Klasse "form-group" (kein "star-rating")
    const fieldset = page.locator('fieldset.form-group:not(.star-rating)');
    await expect(fieldset).toBeVisible();
    const legend = fieldset.locator('legend');
    await expect(legend).toContainText('Kategorien');
  });

});
