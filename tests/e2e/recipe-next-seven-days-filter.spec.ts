import { test, expect } from '@playwright/test';

/**
 * E2E-Tests für Story 10: Filter "Nächste 7 Tage"
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

function futureDateInDays(days: number): string {
  const d = new Date();
  d.setDate(d.getDate() + days);
  return `${d.getDate()}.${d.getMonth() + 1}.${d.getFullYear()}`;
}

test.describe('Filter "Nächste 7 Tage" (Story 10)', () => {

  test('K1: Filter-Button ist sichtbar und aktivierbar', async ({ page }) => {
    // Given: Die Startseite wird aufgerufen (Filter-Panel aufgeklappt)
    await page.goto('/?filter_collapsed=0');

    // Then: Button "Nächste 7 Tage" ist sichtbar
    const filterBtn = page.locator('a.sort-filter-btn', { hasText: 'Nächste 7 Tage' });
    await expect(filterBtn).toBeVisible();

    // And: Button ist initial nicht aktiv (aria-pressed="false")
    await expect(filterBtn).toHaveAttribute('aria-pressed', 'false');

    // When: Button geklickt wird
    await filterBtn.click();

    // Then: Filter ist aktiv (aria-pressed="true")
    await expect(filterBtn).toHaveAttribute('aria-pressed', 'true');
    await expect(filterBtn).toHaveClass(/active/);

    // And: URL enthält filter=naechste-7-tage
    await expect(page).toHaveURL(/filter=naechste-7-tage/);
  });

  test('K2: Nur Rezepte im Zeitfenster werden angezeigt', async ({ page }) => {
    // Given: "Spaghetti" mit Datum übermorgen, "Pizza" mit Datum in 5 Tagen, "Linseneintopf" ohne Datum
    const suffix = Date.now();
    await createRecipeWithDate(page, `Spaghetti ${suffix}`, ['Mittagessen'], futureDateInDays(2));
    await createRecipeWithDate(page, `Pizza ${suffix}`, ['Mittagessen'], futureDateInDays(5));
    await createRecipeWithDate(page, `Linseneintopf ${suffix}`, ['Mittagessen']);

    // When: Filter "Nächste 7 Tage" aktiviert wird
    await page.goto('/?filter_collapsed=0');
    await page.locator('a.sort-filter-btn', { hasText: 'Nächste 7 Tage' }).click();

    // Then: Spaghetti und Pizza sichtbar, Linseneintopf (ohne Datum) nicht sichtbar
    await expect(page.locator('#recipe-results')).toContainText(`Spaghetti ${suffix}`);
    await expect(page.locator('#recipe-results')).toContainText(`Pizza ${suffix}`);
    await expect(page.locator('#recipe-results')).not.toContainText(`Linseneintopf ${suffix}`);
  });

  test('K3: Chronologische Sortierung (früheres Datum zuerst)', async ({ page }) => {
    // Given: "Pizza" mit Datum in 5 Tagen, "Spaghetti" mit Datum übermorgen
    const suffix = Date.now();
    await createRecipeWithDate(page, `Pizza ${suffix}`, ['Mittagessen'], futureDateInDays(5));
    await createRecipeWithDate(page, `Spaghetti ${suffix}`, ['Mittagessen'], futureDateInDays(2));

    // When: Filter "Nächste 7 Tage" aktiviert wird
    await page.goto('/?filter_collapsed=0');
    await page.locator('a.sort-filter-btn', { hasText: 'Nächste 7 Tage' }).click();

    // Then: Spaghetti (früher) erscheint vor Pizza (später)
    const recipeItems = page.locator('.recipe-item h2');
    const titles = await recipeItems.allTextContents();

    const spaghettiIdx = titles.findIndex(t => t.includes(`Spaghetti ${suffix}`));
    const pizzaIdx = titles.findIndex(t => t.includes(`Pizza ${suffix}`));

    expect(spaghettiIdx).toBeGreaterThanOrEqual(0);
    expect(pizzaIdx).toBeGreaterThanOrEqual(0);
    expect(spaghettiIdx).toBeLessThan(pizzaIdx);
  });

  test('K3: Zeitfenster-Grenzen (heute inklusive, Tag 7 inklusive, Tag 8 und gestern exklusiv)', async ({ page }) => {
    // Given: Rezepte an verschiedenen Terminen
    const suffix = Date.now();
    await createRecipeWithDate(page, `Heute-Rezept ${suffix}`, ['Mittagessen'], futureDateInDays(0));
    await createRecipeWithDate(page, `Tag-7-Rezept ${suffix}`, ['Mittagessen'], futureDateInDays(7));
    await createRecipeWithDate(page, `Tag-8-Rezept ${suffix}`, ['Mittagessen'], futureDateInDays(8));
    await createRecipeWithDate(page, `Vergangen-Rezept ${suffix}`, ['Mittagessen'], futureDateInDays(-1));

    // When: Filter "Nächste 7 Tage" aktiviert wird
    await page.goto('/?filter_collapsed=0');
    await page.locator('a.sort-filter-btn', { hasText: 'Nächste 7 Tage' }).click();

    // Then: Heute-Rezept und Tag-7-Rezept sichtbar, andere nicht
    await expect(page.locator('#recipe-results')).toContainText(`Heute-Rezept ${suffix}`);
    await expect(page.locator('#recipe-results')).toContainText(`Tag-7-Rezept ${suffix}`);
    await expect(page.locator('#recipe-results')).not.toContainText(`Tag-8-Rezept ${suffix}`);
    await expect(page.locator('#recipe-results')).not.toContainText(`Vergangen-Rezept ${suffix}`);
  });

  test('K4: Filter zurücksetzen via Toggle', async ({ page }) => {
    // Given: Filter "Nächste 7 Tage" ist aktiv (direkte URL)
    const suffix = Date.now();
    await createRecipeWithDate(page, `Gulasch ${suffix}`, ['Mittagessen'], futureDateInDays(2));

    await page.goto('/?filter=naechste-7-tage&filter_collapsed=0');
    const filterBtn = page.locator('a.sort-filter-btn', { hasText: 'Nächste 7 Tage' });
    await expect(filterBtn).toHaveAttribute('aria-pressed', 'true');

    // When: Filter-Button erneut geklickt wird (Toggle)
    await filterBtn.click();

    // Then: Filter ist nicht mehr aktiv
    await expect(filterBtn).toHaveAttribute('aria-pressed', 'false');

    // And: URL hat keinen filter=naechste-7-tage Parameter mehr
    const url = page.url();
    expect(url).not.toMatch(/filter=naechste-7-tage/);

    // And: Rezept ist weiterhin sichtbar (alphabetische Ansicht)
    await expect(page.locator('#recipe-results')).toContainText(`Gulasch ${suffix}`);
  });

  test('K5: Keine Treffer → Hinweistext erscheint', async ({ page }) => {
    // Given: Alle Rezepte außerhalb des 7-Tage-Fensters
    const suffix = Date.now();
    await createRecipeWithDate(page, `Fernes-Rezept ${suffix}`, ['Mittagessen'], '1.1.2099');

    // When: Filter "Nächste 7 Tage" aktiviert wird und nach eindeutigem Begriff gesucht wird
    await page.goto(`/?filter=naechste-7-tage&q=Fernes-Rezept+${suffix}`);

    // Then: Hinweistext erscheint
    await expect(page.locator('#recipe-results')).toContainText('Keine Rezepte');

    // And: Keine Rezept-Karte ist sichtbar
    await expect(page.locator('#recipe-results .recipe-item')).toHaveCount(0);
  });

  test('K6: DeepLink /?filter=naechste-7-tage', async ({ page }) => {
    // Given: Rezept mit Datum in 2 Tagen vorhanden
    const suffix = Date.now();
    await createRecipeWithDate(page, `Planungs-Rezept ${suffix}`, ['Snacks'], futureDateInDays(2));

    // When: URL direkt mit ?filter=naechste-7-tage aufgerufen wird
    await page.goto('/?filter=naechste-7-tage');

    // Then: Filter-Button ist als aktiv markiert
    const filterBtn = page.locator('a.sort-filter-btn', { hasText: 'Nächste 7 Tage' });
    await expect(filterBtn).toHaveAttribute('aria-pressed', 'true');
    await expect(filterBtn).toHaveClass(/active/);

    // And: Rezept ist sichtbar
    await expect(page.locator('#recipe-results')).toContainText(`Planungs-Rezept ${suffix}`);
  });

  test('K7: Datum mit Wochentag auf Rezeptkarte bei aktivem Filter', async ({ page }) => {
    // Given: Rezept mit Datum übermorgen
    const suffix = Date.now();
    await createRecipeWithDate(page, `Wochentag-Rezept ${suffix}`, ['Mittagessen'], futureDateInDays(2));

    // When: Filter "Nächste 7 Tage" aktiviert wird
    await page.goto('/?filter=naechste-7-tage');

    // Then: Rezeptkarte zeigt Wochentag + Datum (Format: "Mo, 31.03.2026")
    const dateDisplay = page.locator('.recipe-date-weekday');
    await expect(dateDisplay.first()).toBeVisible();
    // Prüfe dass das Format Wochentag + Datum enthält (z.B. "Mo, " oder "Di, ")
    const dateText = await dateDisplay.first().textContent();
    expect(dateText).toMatch(/^(Mo|Di|Mi|Do|Fr|Sa|So), \d{2}\.\d{2}\.\d{4}$/);
  });

  test('K8: Kombination mit Kategorie-Filter', async ({ page }) => {
    // Given: "Dinkelbrot" (Brot, in 2 Tagen) und "Spaghetti" (Mittagessen, in 3 Tagen)
    const suffix = Date.now();
    await createRecipeWithDate(page, `Dinkelbrot ${suffix}`, ['Brot'], futureDateInDays(2));
    await createRecipeWithDate(page, `Spaghetti ${suffix}`, ['Mittagessen'], futureDateInDays(3));

    // When: Kategorie "Brot" gewählt + Filter "Nächste 7 Tage" aktiviert
    await page.goto('/?kategorie=Brot&filter_collapsed=0');
    await page.locator('a.sort-filter-btn', { hasText: 'Nächste 7 Tage' }).click();

    // Then: Nur "Dinkelbrot" sichtbar, "Spaghetti" nicht sichtbar
    await expect(page.locator('#recipe-results')).toContainText(`Dinkelbrot ${suffix}`);
    await expect(page.locator('#recipe-results')).not.toContainText(`Spaghetti ${suffix}`);

    // And: URL enthält beide Parameter
    await expect(page).toHaveURL(/kategorie=Brot/);
    await expect(page).toHaveURL(/filter=naechste-7-tage/);
  });

  test('K9: Kombination mit Volltextsuche', async ({ page }) => {
    // Given: "Dinkelbrot" (in 2 Tagen) und "Spaghetti" (in 3 Tagen)
    const suffix = Date.now();
    await createRecipeWithDate(page, `Dinkelbrot ${suffix}`, ['Brot'], futureDateInDays(2));
    await createRecipeWithDate(page, `Spaghetti ${suffix}`, ['Mittagessen'], futureDateInDays(3));

    // When: Suche nach "Dinkel" + Filter "Nächste 7 Tage" aktiv
    await page.goto(`/?filter=naechste-7-tage&q=Dinkel`);

    // Then: Nur "Dinkelbrot" sichtbar, "Spaghetti" nicht sichtbar
    await expect(page.locator('#recipe-results')).toContainText(`Dinkelbrot ${suffix}`);
    await expect(page.locator('#recipe-results')).not.toContainText(`Spaghetti ${suffix}`);
  });

});
