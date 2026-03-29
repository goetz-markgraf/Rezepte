import { test, expect } from '@playwright/test';

/**
 * E2E-Tests für Story 9: Filter "Länger nicht gemacht"
 *
 * Die Tests erstellen Rezepte direkt über das Formular für Isolation.
 */

async function createRecipeWithDate(
  page: import('@playwright/test').Page,
  title: string,
  categories: string[],
  plannedDate?: string
): Promise<void> {
  await page.goto('/recipes/new');
  await page.fill('input[name="title"]', title);
  for (const category of categories) {
    await page.check(`input[name="categories"][value="${category}"]`);
  }
  if (plannedDate) {
    await page.fill('input[name="planned_date"]', plannedDate);
  }
  await page.click('button[type="submit"]');
  await expect(page).toHaveURL(/\/recipes\/\d+/);
}

test.describe('Filter "Länger nicht gemacht" (Story 9)', () => {

  test('K1: Filter-Button ist sichtbar und aktivierbar', async ({ page }) => {
    // Given: Die Startseite wird aufgerufen
    await page.goto('/');

    // Then: Button "Länger nicht gemacht" ist sichtbar
    const filterBtn = page.locator('a.sort-filter-btn', { hasText: 'Länger nicht gemacht' });
    await expect(filterBtn).toBeVisible();
    await expect(filterBtn).toContainText('Länger nicht gemacht');

    // And: Button ist initial nicht aktiv (aria-pressed="false")
    await expect(filterBtn).toHaveAttribute('aria-pressed', 'false');

    // When: Button geklickt wird
    await filterBtn.click();

    // Then: Filter ist aktiv (aria-pressed="true")
    await expect(filterBtn).toHaveAttribute('aria-pressed', 'true');
    await expect(filterBtn).toHaveClass(/active/);

    // And: URL enthält filter=laenger-nicht-gemacht
    await expect(page).toHaveURL(/filter=laenger-nicht-gemacht/);
  });

  test('K2: Sortierung nach Datum aufsteigend, NULL-Daten zuerst', async ({ page }) => {
    // Given: Drei Rezepte mit unterschiedlichen Daten
    const suffix = Date.now();
    await createRecipeWithDate(page, `Spaghetti Bolognese ${suffix}`, ['Mittagessen'], '1.1.2026');
    await createRecipeWithDate(page, `Pfannkuchen ${suffix}`, ['Snacks'], '15.6.2025');
    await createRecipeWithDate(page, `Pizza ${suffix}`, ['Mittagessen']); // kein Datum

    // When: Filter "Länger nicht gemacht" aktiviert wird
    await page.goto('/');
    await page.locator('a.sort-filter-btn', { hasText: 'Länger nicht gemacht' }).click();

    // Then: Pizza (kein Datum) erscheint als erstes
    const recipeItems = page.locator('.recipe-item h2');
    const titles = await recipeItems.allTextContents();

    const pizzaIdx = titles.findIndex(t => t.includes(`Pizza ${suffix}`));
    const pfannkuchenIdx = titles.findIndex(t => t.includes(`Pfannkuchen ${suffix}`));
    const spaghettiIdx = titles.findIndex(t => t.includes(`Spaghetti Bolognese ${suffix}`));

    // Pizza (NULL) zuerst
    expect(pizzaIdx).toBeGreaterThanOrEqual(0);
    expect(pfannkuchenIdx).toBeGreaterThanOrEqual(0);
    expect(spaghettiIdx).toBeGreaterThanOrEqual(0);

    expect(pizzaIdx).toBeLessThan(pfannkuchenIdx);
    expect(pfannkuchenIdx).toBeLessThan(spaghettiIdx);
  });

  test('K3: Zukunftsdaten werden ausgeschlossen', async ({ page }) => {
    // Given: Sonntagsbraten mit Zukunftsdatum, Linseneintopf mit Vergangenheitsdatum
    const suffix = Date.now();
    await createRecipeWithDate(page, `Sonntagsbraten ${suffix}`, ['Mittagessen'], '1.1.2099');
    await createRecipeWithDate(page, `Linseneintopf ${suffix}`, ['Mittagessen'], '1.1.2020');

    // When: Filter "Länger nicht gemacht" aktiviert wird
    await page.goto('/');
    await page.locator('a.sort-filter-btn', { hasText: 'Länger nicht gemacht' }).click();

    // Then: Linseneintopf ist sichtbar
    await expect(page.locator('#recipe-results')).toContainText(`Linseneintopf ${suffix}`);

    // And: Sonntagsbraten ist nicht sichtbar (Zukunftsdatum)
    await expect(page.locator('#recipe-results')).not.toContainText(`Sonntagsbraten ${suffix}`);
  });

  test('K4: Filter zurücksetzen', async ({ page }) => {
    // Given: Filter "Länger nicht gemacht" ist aktiv
    const suffix = Date.now();
    await createRecipeWithDate(page, `Gulasch ${suffix}`, ['Mittagessen'], '1.1.2020');

    await page.goto('/?filter=laenger-nicht-gemacht');
    const filterBtn = page.locator('a.sort-filter-btn', { hasText: 'Länger nicht gemacht' });
    await expect(filterBtn).toHaveAttribute('aria-pressed', 'true');

    // When: Filter-Button erneut geklickt wird (Toggle)
    await filterBtn.click();

    // Then: Filter ist nicht mehr aktiv
    await expect(filterBtn).toHaveAttribute('aria-pressed', 'false');

    // And: URL hat keinen filter-Parameter mehr
    const url = page.url();
    expect(url).not.toMatch(/filter=laenger-nicht-gemacht/);

    // And: Rezept ist weiterhin sichtbar (alphabetische Ansicht)
    await expect(page.locator('#recipe-results')).toContainText(`Gulasch ${suffix}`);
  });

  test('K5: Keine passenden Rezepte — Hinweistext erscheint', async ({ page }) => {
    // Given: Alle Rezepte für diesen Suchbegriff haben ein Zukunftsdatum
    const suffix = Date.now();
    await createRecipeWithDate(page, `ZukunftGericht ${suffix}`, ['Mittagessen'], '1.1.2099');

    // When: Filter "Länger nicht gemacht" + Suche nach eindeutigem Suchbegriff aktiviert wird
    await page.goto(`/?filter=laenger-nicht-gemacht&q=ZukunftGericht+${suffix}`);

    // Then: Hinweistext erscheint
    await expect(page.locator('#recipe-results')).toContainText('Keine Rezepte');

    // And: Keine Rezept-Karte ist sichtbar (nur die Meldung, nicht das Rezept als Link)
    await expect(page.locator('#recipe-results .recipe-item')).toHaveCount(0);
  });

  test('K6: DeepLink ?filter=laenger-nicht-gemacht', async ({ page }) => {
    // Given: Rezept mit Vergangenheitsdatum existiert
    const suffix = Date.now();
    await createRecipeWithDate(page, `Pfannkuchen ${suffix}`, ['Snacks'], '15.6.2025');

    // When: URL direkt mit ?filter=laenger-nicht-gemacht aufgerufen wird
    await page.goto('/?filter=laenger-nicht-gemacht');

    // Then: Filter-Button ist als aktiv markiert
    const filterBtn = page.locator('a.sort-filter-btn', { hasText: 'Länger nicht gemacht' });
    await expect(filterBtn).toHaveAttribute('aria-pressed', 'true');
    await expect(filterBtn).toHaveClass(/active/);

    // And: Rezept ist sichtbar und nach Datum sortiert
    await expect(page.locator('#recipe-results')).toContainText(`Pfannkuchen ${suffix}`);
  });

  test('K7: Kombination mit Kategorie-Filter', async ({ page }) => {
    // Given: Brot-Rezepte und Mittagessen-Rezept mit verschiedenen Daten
    const suffix = Date.now();
    await createRecipeWithDate(page, `Dinkelbrot ${suffix}`, ['Brot'], '1.1.2025');
    await createRecipeWithDate(page, `Roggenbrot ${suffix}`, ['Brot'], '1.1.2026');
    await createRecipeWithDate(page, `Spaghetti ${suffix}`, ['Mittagessen'], '1.1.2020');

    // When: Kategorie "Brot" UND Filter "Länger nicht gemacht" aktiv
    await page.goto(`/?kategorie=Brot`);
    await page.locator('a.sort-filter-btn', { hasText: 'Länger nicht gemacht' }).click();

    // Then: Nur Brot-Rezepte sichtbar
    await expect(page.locator('#recipe-results')).toContainText(`Dinkelbrot ${suffix}`);
    await expect(page.locator('#recipe-results')).not.toContainText(`Spaghetti ${suffix}`);

    // And: URL enthält beide Filter
    await expect(page).toHaveURL(/kategorie=Brot/);
    await expect(page).toHaveURL(/filter=laenger-nicht-gemacht/);

    // And: Dinkelbrot (2025) erscheint vor Roggenbrot (2026)
    const recipeItems = page.locator('.recipe-item h2');
    const titles = await recipeItems.allTextContents();
    const dinkelbrotIdx = titles.findIndex(t => t.includes(`Dinkelbrot ${suffix}`));
    const roggenbrotIdx = titles.findIndex(t => t.includes(`Roggenbrot ${suffix}`));

    expect(dinkelbrotIdx).toBeGreaterThanOrEqual(0);
    expect(roggenbrotIdx).toBeGreaterThanOrEqual(0);
    expect(dinkelbrotIdx).toBeLessThan(roggenbrotIdx);
  });

});
