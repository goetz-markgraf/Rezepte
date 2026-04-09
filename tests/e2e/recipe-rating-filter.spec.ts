import { test, expect } from '@playwright/test';

/**
 * E2E-Tests für Story 11: Filter nach Bewertung (Beliebtheit)
 *
 * Die Tests erstellen Rezepte direkt über das Formular für Isolation.
 */

async function selectRating(
  page: import('@playwright/test').Page,
  rating: number
): Promise<void> {
  // Klick auf das sichtbare Label des Sterne-Radio-Buttons (Input ist visuell versteckt)
  const input = page.locator(`input[name="rating"][value="${rating}"]`);
  const label = input.locator('xpath=ancestor::label');
  await label.click();
}

async function createRecipeWithRating(
  page: import('@playwright/test').Page,
  title: string,
  categories: string[],
  rating?: number,
  plannedDate?: string
): Promise<void> {
  await page.goto('/recipes/new');
  await page.fill('input[name="title"]', title);
  for (const category of categories) {
    await page.check(`input[name="categories"][value="${category}"]`);
  }
  if (rating !== undefined) {
    await selectRating(page, rating);
  }
  if (plannedDate) {
    await page.fill('input[name="planned_date"]', plannedDate);
  }
  await page.click('button[type="submit"]');
  await expect(page).toHaveURL(/\/recipes\/\d+/);
}

test.describe('Filter nach Bewertung (Story 11)', () => {

  test('K1: Filter-Buttons sichtbar und auswählbar', async ({ page }) => {
    // Given: Startseite wird aufgerufen (Filter-Panel aufgeklappt)
    await page.goto('/?filter_collapsed=0');

    // Then: "★★★+ Nur Gute"-Button ist sichtbar (aria-pressed="false")
    const gutBtn = page.locator('a.sort-filter-btn', { hasText: '★★★+ Nur Gute' });
    await expect(gutBtn).toBeVisible();
    await expect(gutBtn).toHaveAttribute('aria-pressed', 'false');

    // And: "★★★★★ Favoriten"-Button ist sichtbar (aria-pressed="false")
    const favoritenBtn = page.locator('a.sort-filter-btn', { hasText: '★★★★★ Favoriten' });
    await expect(favoritenBtn).toBeVisible();
    await expect(favoritenBtn).toHaveAttribute('aria-pressed', 'false');
  });

  test('K2: Filter "Nur Gute" zeigt 3+ Sterne', async ({ page }) => {
    // Given: "Spaghetti Bolognese" mit 4 Sternen, "Pfannkuchen" mit 2 Sternen, "Pizza" ohne Bewertung
    const suffix = Date.now();
    await createRecipeWithRating(page, `Spaghetti-Bolognese-${suffix}`, ['Mittagessen'], 4);
    await createRecipeWithRating(page, `Pfannkuchen-${suffix}`, ['Snacks'], 2);
    await createRecipeWithRating(page, `Pizza-${suffix}`, ['Mittagessen']);

    // When: Klick auf "Nur Gute"-Button
    await page.goto('/?filter_collapsed=0');
    await page.locator('a.sort-filter-btn', { hasText: '★★★+ Nur Gute' }).click();

    // Then: "Spaghetti Bolognese" sichtbar
    await expect(page.locator('.recipe-item h2', { hasText: `Spaghetti-Bolognese-${suffix}` })).toBeVisible();

    // And: "Pfannkuchen" nicht sichtbar
    await expect(page.locator('.recipe-item h2', { hasText: `Pfannkuchen-${suffix}` })).not.toBeVisible();

    // And: "Pizza" nicht sichtbar
    await expect(page.locator('.recipe-item h2', { hasText: `Pizza-${suffix}` })).not.toBeVisible();

    // And: URL enthält bewertung=gut
    await expect(page).toHaveURL(/bewertung=gut/);

    // And: "Nur Gute"-Button hat aria-pressed="true" und Klasse active
    const gutBtn = page.locator('a.sort-filter-btn', { hasText: '★★★+ Nur Gute' });
    await expect(gutBtn).toHaveAttribute('aria-pressed', 'true');
    await expect(gutBtn).toHaveClass(/active/);
  });

  test('K3: Filter "Favoriten" zeigt nur 5 Sterne', async ({ page }) => {
    // Given: "Omas Apfelkuchen" mit 5 Sternen, "Nudelsuppe" mit 4 Sternen, "Rührei" mit 3 Sternen
    const suffix = Date.now();
    await createRecipeWithRating(page, `Omas-Apfelkuchen-${suffix}`, ['Kuchen'], 5);
    await createRecipeWithRating(page, `Nudelsuppe-${suffix}`, ['Mittagessen'], 4);
    await createRecipeWithRating(page, `Rührei-${suffix}`, ['Mittagessen'], 3);

    // When: Klick auf "Favoriten"-Button
    await page.goto('/?filter_collapsed=0');
    await page.locator('a.sort-filter-btn', { hasText: '★★★★★ Favoriten' }).click();

    // Then: Nur "Omas Apfelkuchen" sichtbar
    await expect(page.locator('.recipe-item h2', { hasText: `Omas-Apfelkuchen-${suffix}` })).toBeVisible();
    await expect(page.locator('.recipe-item h2', { hasText: `Nudelsuppe-${suffix}` })).not.toBeVisible();
    await expect(page.locator('.recipe-item h2', { hasText: `Rührei-${suffix}` })).not.toBeVisible();

    // And: URL enthält bewertung=favoriten
    await expect(page).toHaveURL(/bewertung=favoriten/);

    // And: "Favoriten"-Button hat aria-pressed="true" und Klasse active
    const favoritenBtn = page.locator('a.sort-filter-btn', { hasText: '★★★★★ Favoriten' });
    await expect(favoritenBtn).toHaveAttribute('aria-pressed', 'true');
    await expect(favoritenBtn).toHaveClass(/active/);
  });

  test('K4: Aktiver Filter visuell erkennbar (DeepLink)', async ({ page }) => {
    // Given: URL /?bewertung=gut direkt aufgerufen
    await page.goto('/?bewertung=gut');

    // Then: "Nur Gute"-Button hat aria-pressed="true" und Klasse active
    const gutBtn = page.locator('a.sort-filter-btn', { hasText: '★★★+ Nur Gute' });
    await expect(gutBtn).toHaveAttribute('aria-pressed', 'true');
    await expect(gutBtn).toHaveClass(/active/);

    // And: "Favoriten"-Button hat aria-pressed="false" (nicht aktiv)
    const favoritenBtn = page.locator('a.sort-filter-btn', { hasText: '★★★★★ Favoriten' });
    await expect(favoritenBtn).toHaveAttribute('aria-pressed', 'false');
    await expect(favoritenBtn).not.toHaveClass(/active/);
  });

  test('K5: Filter zurücksetzen (Toggle)', async ({ page }) => {
    // Given: Rezept mit 4 Sternen und Filter "Nur Gute" ist aktiv
    const suffix = Date.now();
    await createRecipeWithRating(page, `ToggleRezept-${suffix}`, ['Mittagessen'], 4);

    await page.goto('/?bewertung=gut&filter_collapsed=0');
    await expect(page.locator('a.sort-filter-btn', { hasText: '★★★+ Nur Gute' })).toHaveAttribute('aria-pressed', 'true');

    // When: Erneuter Klick auf "Nur Gute"-Button
    await page.locator('a.sort-filter-btn', { hasText: '★★★+ Nur Gute' }).click();

    // Then: Filter deaktiviert (aria-pressed="false")
    const gutBtn = page.locator('a.sort-filter-btn', { hasText: '★★★+ Nur Gute' });
    await expect(gutBtn).toHaveAttribute('aria-pressed', 'false');
    await expect(gutBtn).not.toHaveClass(/active/);

    // And: URL enthält kein bewertung-Parameter mehr
    await expect(page).not.toHaveURL(/bewertung/);

    // And: Alle Rezepte wieder sichtbar
    await expect(page.locator('.recipe-item h2', { hasText: `ToggleRezept-${suffix}` })).toBeVisible();
  });

  test('K6: Keine Treffer — Hinweistext erscheint', async ({ page }) => {
    // Given: Alle Rezepte in der App haben maximal 2 Sterne oder keine Bewertung (eindeutiger Suffix)
    const suffix = `low-${Date.now()}`;
    await createRecipeWithRating(page, `NiedrigRezept1-${suffix}`, ['Mittagessen'], 1);
    await createRecipeWithRating(page, `NiedrigRezept2-${suffix}`, ['Mittagessen'], 2);

    // When: Klick auf "Nur Gute" mit Suche, die nur die Test-Rezepte zeigt
    await page.goto(`/?q=${suffix}&filter_collapsed=0`);
    await page.locator('a.sort-filter-btn', { hasText: '★★★+ Nur Gute' }).click();

    // Then: Hinweistext sichtbar
    await expect(page.locator('.search-no-results')).toBeVisible();
    await expect(page.locator('.search-no-results')).toContainText('Keine Rezepte');

    // And: Keine Rezept-Karte vorhanden
    await expect(page.locator('.recipe-item')).toHaveCount(0);
  });

  test('K7: DeepLink ?bewertung=favoriten', async ({ page }) => {
    // Given: "Omas Apfelkuchen" mit 5 Sternen existiert
    const suffix = Date.now();
    await createRecipeWithRating(page, `OmasApfelkuchen-${suffix}`, ['Kuchen'], 5);

    // When: URL /?bewertung=favoriten direkt aufgerufen
    await page.goto('/?bewertung=favoriten');

    // Then: "Omas Apfelkuchen" sichtbar
    await expect(page.locator('.recipe-item h2', { hasText: `OmasApfelkuchen-${suffix}` })).toBeVisible();

    // And: "Favoriten"-Button ist aktiv markiert
    await expect(page.locator('a.sort-filter-btn', { hasText: '★★★★★ Favoriten' })).toHaveAttribute('aria-pressed', 'true');
  });

});
