import { test, expect } from '@playwright/test';

/**
 * E2E-Tests für Story 12: Kombinierte Filter (mehrere Filter gleichzeitig)
 *
 * Die Tests erstellen Rezepte direkt über das Formular für Isolation.
 */

async function createRecipeWithOptions(
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

test.describe('Kombinierte Filter (Story 12)', () => {

  test('K1: Kategorie + Volltextsuche zeigt nur Schnittmenge', async ({ page }) => {
    // Gegeben: "Dinkelbrot" (Brot), "Roggenbrot" (Brot), "Dinkel-Müsli" (Snacks)
    const suffix = `k1-${Date.now()}-${Math.random().toString(36).slice(2, 7)}`;
    await createRecipeWithOptions(page, `Dinkelbrot-${suffix}`, ['Brot']);
    await createRecipeWithOptions(page, `Roggenbrot-${suffix}`, ['Brot']);
    await createRecipeWithOptions(page, `Dinkelmuesli-${suffix}`, ['Snacks']);

    // Wenn: Kategorie "Brot" wählen und "Dinkel" ins Suchfeld eingeben
    await page.goto('/?filter_collapsed=0');
    await page.locator('a.category-filter-btn', { hasText: 'Brot' }).click();
    await page.fill('input[name="q"]', `Dinkel`);
    await page.click('button[type="submit"]');

    // Dann: Nur "Dinkelbrot" sichtbar
    await expect(page.locator('.recipe-item h2', { hasText: `Dinkelbrot-${suffix}` })).toBeVisible();

    // Und: "Roggenbrot" nicht sichtbar (kein Suchtreffer)
    await expect(page.locator('.recipe-item h2', { hasText: `Roggenbrot-${suffix}` })).not.toBeVisible();

    // Und: "Dinkel-Müsli" nicht sichtbar (falsche Kategorie)
    await expect(page.locator('.recipe-item h2', { hasText: `Dinkelmuesli-${suffix}` })).not.toBeVisible();

    // Und: URL enthält beide Parameter
    await expect(page).toHaveURL(/kategorie=Brot/);
    await expect(page).toHaveURL(/q=Dinkel/);
  });

  test('K5/K6: "Länger nicht gemacht" Filter funktioniert', async ({ page }) => {
    // Gegeben: "Linseneintopf" (1.1.2025), "Erbsensuppe" (1.1.2027), "Kartoffelsuppe" (1.1.2024)
    const suffix = `k5-${Date.now()}-${Math.random().toString(36).slice(2, 7)}`;
    await createRecipeWithOptions(page, `Linseneintopf-${suffix}`, ['Mittagessen'], '1.1.2025');
    await createRecipeWithOptions(page, `Erbsensuppe-${suffix}`, ['Mittagessen'], '1.1.2027');
    await createRecipeWithOptions(page, `Kartoffelsuppe-${suffix}`, ['Mittagessen'], '1.1.2024');

    // Wenn: URL direkt aufrufen mit Filter
    await page.goto(`/?filter=laenger-nicht-gemacht`);

    // Dann: "Linseneintopf" sichtbar (Vergangenheitsdatum)
    await expect(page.locator('.recipe-item h2', { hasText: `Linseneintopf-${suffix}` })).toBeVisible();

    // Und: "Kartoffelsuppe" sichtbar (Vergangenheitsdatum)
    await expect(page.locator('.recipe-item h2', { hasText: `Kartoffelsuppe-${suffix}` })).toBeVisible();

    // Und: "Erbsensuppe" nicht sichtbar (Zukunftsdatum)
    await expect(page.locator('.recipe-item h2', { hasText: `Erbsensuppe-${suffix}` })).not.toBeVisible();

    // Und: URL enthält Filter-Parameter
    await expect(page).toHaveURL(/filter=laenger-nicht-gemacht/);
  });

  test('K6: Zwei Filter: Kategorie + "Länger nicht gemacht"', async ({ page }) => {
    // Gegeben: "Dinkelbrot" (Brot, 1.6.2025), "Roggenbrot" (Brot, 1.6.2026), "Linseneintopf" (Mittagessen, 1.1.2024)
    const suffix = `k6-${Date.now()}-${Math.random().toString(36).slice(2, 7)}`;
    await createRecipeWithOptions(page, `Dinkelbrot-${suffix}`, ['Brot'], '1.6.2025');
    await createRecipeWithOptions(page, `Roggenbrot-${suffix}`, ['Brot'], '1.6.2026');
    await createRecipeWithOptions(page, `Linseneintopf-${suffix}`, ['Mittagessen'], '1.1.2024');

    // Wenn: URL direkt aufrufen mit zwei Filtern
    await page.goto(`/?kategorie=Brot&filter=laenger-nicht-gemacht`);

    // Dann: "Dinkelbrot" sichtbar (Brot, Vergangenheitsdatum)
    await expect(page.locator('.recipe-item h2', { hasText: `Dinkelbrot-${suffix}` })).toBeVisible();

    // Und: "Roggenbrot" nicht sichtbar (Zukunftsdatum)
    await expect(page.locator('.recipe-item h2', { hasText: `Roggenbrot-${suffix}` })).not.toBeVisible();

    // Und: "Linseneintopf" nicht sichtbar (falsche Kategorie)
    await expect(page.locator('.recipe-item h2', { hasText: `Linseneintopf-${suffix}` })).not.toBeVisible();

    // Und: Beide Filter sind als aktiv markiert
    await expect(page.locator('a.category-filter-btn', { hasText: 'Brot' })).toHaveAttribute('aria-pressed', 'true');
    await expect(page.locator('a.sort-filter-btn', { hasText: 'Länger nicht gemacht' })).toHaveAttribute('aria-pressed', 'true');
  });

  test('K9: Einzelnen Filter deaktivieren ohne andere zu verlieren', async ({ page }) => {
    // Gegeben: Kategorie "Brot" aktiv (via URL-Parameter)
    const suffix = `k9-${Date.now()}-${Math.random().toString(36).slice(2, 7)}`;
    await createRecipeWithOptions(page, `Dinkelbrot-${suffix}`, ['Brot']);
    await createRecipeWithOptions(page, `Roggenbrot-${suffix}`, ['Brot']);

    await page.goto(`/?kategorie=Brot&filter_collapsed=0`);

    // Dann: Beide Brot-Rezepte sichtbar
    await expect(page.locator('.recipe-item h2', { hasText: `Dinkelbrot-${suffix}` })).toBeVisible();
    await expect(page.locator('.recipe-item h2', { hasText: `Roggenbrot-${suffix}` })).toBeVisible();

    // Wenn: Klick auf "Länger nicht gemacht" zum Aktivieren
    await page.locator('a.sort-filter-btn', { hasText: 'Länger nicht gemacht' }).click();

    // Dann: Kategorie "Brot" noch aktiv
    await expect(page.locator('a.category-filter-btn', { hasText: 'Brot' })).toHaveAttribute('aria-pressed', 'true');

    // Und: URL enthält beide Parameter
    await expect(page).toHaveURL(/kategorie=Brot/);
    await expect(page).toHaveURL(/filter=laenger-nicht-gemacht/);
  });

  test('K12: Keine Treffer durch Kombination zeigt Hinweistext', async ({ page }) => {
    // Gegeben: Nur Brot-Rezepte (eindeutiger Suffix)
    const suffix = `k12-${Date.now()}-${Math.random().toString(36).slice(2, 7)}`;
    await createRecipeWithOptions(page, `Brötchen-${suffix}`, ['Brot']);

    // Wenn: Kombination, die kein Ergebnis liefert
    await page.goto(`/?q=Unbekannt-${suffix}&kategorie=Brot`);

    // Dann: .search-no-results sichtbar
    await expect(page.locator('.search-no-results')).toBeVisible();

    // Und: Keine Rezept-Items sichtbar
    await expect(page.locator('.recipe-item')).toHaveCount(0);
  });

  test('K11: DeepLink mit mehreren Filtern zeigt korrekte Ansicht', async ({ page }) => {
    // Gegeben: "Dinkelbrot" (Brot) und "Roggenbrot" (Brot) existieren
    const suffix = `k11-${Date.now()}-${Math.random().toString(36).slice(2, 7)}`;
    await createRecipeWithOptions(page, `Dinkelbrot-${suffix}`, ['Brot']);
    await createRecipeWithOptions(page, `Roggenbrot-${suffix}`, ['Brot']);

    // Wenn: URL direkt aufrufen
    await page.goto(`/?kategorie=Brot`);

    // Dann: Beide Brot-Rezepte sichtbar
    await expect(page.locator('.recipe-item h2', { hasText: `Dinkelbrot-${suffix}` })).toBeVisible();
    await expect(page.locator('.recipe-item h2', { hasText: `Roggenbrot-${suffix}` })).toBeVisible();

    // Und: Kategorie "Brot" als aktiv markiert
    await expect(page.locator('a.category-filter-btn', { hasText: 'Brot' })).toHaveAttribute('aria-pressed', 'true');
  });

  test('K13: Kombination Kategorie + Volltextsuche', async ({ page }) => {
    // Gegeben: "Dinkelbrot" (Brot), "Dinkelpfannkuchen" (Snacks) — beide enthalten "Dinkel"
    const suffix = `k13-${Date.now()}-${Math.random().toString(36).slice(2, 7)}`;
    await createRecipeWithOptions(page, `Dinkelbrot-${suffix}`, ['Brot']);
    await createRecipeWithOptions(page, `Dinkelpfannkuchen-${suffix}`, ['Snacks']);

    // Wenn: Suche nach "Dinkelbrot-{suffix}" + Kategorie "Brot" — direkte URL
    await page.goto(`/?q=Dinkelbrot-${suffix}&kategorie=Brot`);

    // Dann: "Dinkelbrot" sichtbar (Brot, enthält Suchbegriff)
    await expect(page.locator('.recipe-item h2', { hasText: `Dinkelbrot-${suffix}` })).toBeVisible();

    // Und: "Dinkelpfannkuchen" nicht sichtbar (Snacks, nicht im Suchbegriff enthalten)
    await expect(page.locator('.recipe-item h2', { hasText: `Dinkelpfannkuchen-${suffix}` })).not.toBeVisible();

    // Und: URL enthält beide Parameter
    await expect(page).toHaveURL(/kategorie=Brot/);
  });

  test('K10: "Alle Filter zurücksetzen"-Button erscheint nur bei aktiven Filtern', async ({ page }) => {
    // Gegeben: Keine Filter aktiv
    await page.goto('/');

    // Dann: Kein "Alle Filter zurücksetzen"-Button
    await expect(page.locator('a.reset-all-filters-btn')).not.toBeVisible();

    // Wenn: Ein Filter aktiviert wird
    await page.goto('/?kategorie=Brot&filter_collapsed=0');

    // Dann: "Alle Filter zurücksetzen"-Button erscheint
    await expect(page.locator('a.reset-all-filters-btn')).toBeVisible();
  });

  test('K10: Klick auf "Alle Filter zurücksetzen" setzt alle Filter zurück', async ({ page }) => {
    // Gegeben: Kategorie "Brot" aktiv
    const suffix = `k10-${Date.now()}-${Math.random().toString(36).slice(2, 7)}`;
    await createRecipeWithOptions(page, `Dinkelbrot-${suffix}`, ['Brot']);
    await createRecipeWithOptions(page, `Roggenbrot-${suffix}`, ['Brot']);
    await createRecipeWithOptions(page, `Spaghetti-${suffix}`, ['Mittagessen']);

    await page.goto(`/?kategorie=Brot&filter_collapsed=0`);

    // Wenn: Klick auf "Alle Filter zurücksetzen"
    await page.locator('a.reset-all-filters-btn').click();

    // Dann: URL = /
    await expect(page).toHaveURL('/');

    // Und: Alle Filter inaktiv
    await expect(page.locator('a.category-filter-btn', { hasText: 'Brot' })).toHaveAttribute('aria-pressed', 'false');

    // Und: Kein "Alle Filter zurücksetzen"-Button mehr sichtbar
    await expect(page.locator('a.reset-all-filters-btn')).not.toBeVisible();

    // Und: Alle Rezepte sichtbar
    await expect(page.locator('.recipe-item h2', { hasText: `Dinkelbrot-${suffix}` })).toBeVisible();
    await expect(page.locator('.recipe-item h2', { hasText: `Roggenbrot-${suffix}` })).toBeVisible();
    await expect(page.locator('.recipe-item h2', { hasText: `Spaghetti-${suffix}` })).toBeVisible();
  });

});
