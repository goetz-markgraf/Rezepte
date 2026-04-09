import { test, expect } from '@playwright/test';

/**
 * E2E-Tests für Story 12: Kombinierte Filter (mehrere Filter gleichzeitig)
 *
 * Die Tests erstellen Rezepte direkt über das Formular für Isolation.
 */

async function selectRating(
  page: import('@playwright/test').Page,
  rating: number
): Promise<void> {
  const input = page.locator(`input[name="rating"][value="${rating}"]`);
  const label = input.locator('xpath=ancestor::label');
  await label.click();
}

async function createRecipeWithOptions(
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

  test('K2: Kategorie + Bewertungsfilter zeigt nur Schnittmenge', async ({ page }) => {
    // Gegeben: "Dinkelbrot" (Brot, 4 Sterne), "Roggenbrot" (Brot, 2 Sterne), "Spaghetti" (Mittagessen, 5 Sterne)
    const suffix = `k2-${Date.now()}-${Math.random().toString(36).slice(2, 7)}`;
    await createRecipeWithOptions(page, `Dinkelbrot-${suffix}`, ['Brot'], 4);
    await createRecipeWithOptions(page, `Roggenbrot-${suffix}`, ['Brot'], 2);
    await createRecipeWithOptions(page, `Spaghetti-${suffix}`, ['Mittagessen'], 5);

    // Wenn: Kategorie "Brot" und "Nur Gute" direkt via URL aufrufen
    await page.goto('/?kategorie=Brot&bewertung=gut&filter_collapsed=0');
    // Verifiziere dass die Filter korrekt aktiv sind
    await expect(page.locator('a.category-filter-btn', { hasText: 'Brot' })).toHaveAttribute('aria-pressed', 'true');
    await expect(page.locator('a.sort-filter-btn', { hasText: '★★★+ Nur Gute' })).toHaveAttribute('aria-pressed', 'true');

    // Dann: Nur "Dinkelbrot" sichtbar
    await expect(page.locator('.recipe-item h2', { hasText: `Dinkelbrot-${suffix}` })).toBeVisible();

    // Und: "Roggenbrot" nicht sichtbar (zu niedrige Bewertung)
    await expect(page.locator('.recipe-item h2', { hasText: `Roggenbrot-${suffix}` })).not.toBeVisible();

    // Und: "Spaghetti" nicht sichtbar (falsche Kategorie)
    await expect(page.locator('.recipe-item h2', { hasText: `Spaghetti-${suffix}` })).not.toBeVisible();

    // Und: URL enthält beide Parameter
    await expect(page).toHaveURL(/kategorie=Brot/);
    await expect(page).toHaveURL(/bewertung=gut/);
  });

  test('K5/K6: Bewertungsfilter + "Länger nicht gemacht" — zwei Filter gleichzeitig', async ({ page }) => {
    // Gegeben: "Linseneintopf" (4 Sterne, 1.1.2025), "Erbsensuppe" (4 Sterne, 1.1.2026), "Kartoffelsuppe" (2 Sterne, 1.1.2024)
    const suffix = `k5-${Date.now()}-${Math.random().toString(36).slice(2, 7)}`;
    await createRecipeWithOptions(page, `Linseneintopf-${suffix}`, ['Mittagessen'], 4, '1.1.2025');
    await createRecipeWithOptions(page, `Erbsensuppe-${suffix}`, ['Mittagessen'], 4, '1.1.2027');
    await createRecipeWithOptions(page, `Kartoffelsuppe-${suffix}`, ['Mittagessen'], 2, '1.1.2024');

    // Wenn: URL direkt aufrufen mit beiden Filtern
    await page.goto(`/?bewertung=gut&filter=laenger-nicht-gemacht`);

    // Dann: "Linseneintopf" sichtbar (4 Sterne, Vergangenheitsdatum)
    await expect(page.locator('.recipe-item h2', { hasText: `Linseneintopf-${suffix}` })).toBeVisible();

    // Und: "Kartoffelsuppe" nicht sichtbar (zu niedrige Bewertung)
    await expect(page.locator('.recipe-item h2', { hasText: `Kartoffelsuppe-${suffix}` })).not.toBeVisible();

    // Und: "Erbsensuppe" nicht sichtbar (Zukunftsdatum)
    await expect(page.locator('.recipe-item h2', { hasText: `Erbsensuppe-${suffix}` })).not.toBeVisible();

    // Und: URL enthält beide Filter-Parameter
    await expect(page).toHaveURL(/bewertung=gut/);
    await expect(page).toHaveURL(/filter=laenger-nicht-gemacht/);
  });

  test('K6: Drei Filter: Kategorie + Bewertung + "Länger nicht gemacht"', async ({ page }) => {
    // Gegeben: "Dinkelbrot" (Brot, 5 Sterne, 1.6.2025), "Roggenbrot" (Brot, 5 Sterne, 1.6.2026), "Linseneintopf" (Mittagessen, 5 Sterne, 1.1.2024)
    const suffix = `k6-${Date.now()}-${Math.random().toString(36).slice(2, 7)}`;
    await createRecipeWithOptions(page, `Dinkelbrot-${suffix}`, ['Brot'], 5, '1.6.2025');
    await createRecipeWithOptions(page, `Roggenbrot-${suffix}`, ['Brot'], 5, '1.6.2026');
    await createRecipeWithOptions(page, `Linseneintopf-${suffix}`, ['Mittagessen'], 5, '1.1.2024');

    // Wenn: URL direkt aufrufen mit drei Filtern
    await page.goto(`/?kategorie=Brot&bewertung=favoriten&filter=laenger-nicht-gemacht`);

    // Dann: "Dinkelbrot" sichtbar (Brot, 5 Sterne, Vergangenheitsdatum)
    await expect(page.locator('.recipe-item h2', { hasText: `Dinkelbrot-${suffix}` })).toBeVisible();

    // Und: "Roggenbrot" nicht sichtbar (Zukunftsdatum)
    await expect(page.locator('.recipe-item h2', { hasText: `Roggenbrot-${suffix}` })).not.toBeVisible();

    // Und: "Linseneintopf" nicht sichtbar (falsche Kategorie)
    await expect(page.locator('.recipe-item h2', { hasText: `Linseneintopf-${suffix}` })).not.toBeVisible();

    // Und: Alle drei Filter sind als aktiv markiert
    await expect(page.locator('a.category-filter-btn', { hasText: 'Brot' })).toHaveAttribute('aria-pressed', 'true');
    await expect(page.locator('a.sort-filter-btn', { hasText: '★★★★★ Favoriten' })).toHaveAttribute('aria-pressed', 'true');
    await expect(page.locator('a.sort-filter-btn', { hasText: 'Länger nicht gemacht' })).toHaveAttribute('aria-pressed', 'true');
  });

  test('K9: Einzelnen Filter deaktivieren ohne andere zu verlieren', async ({ page }) => {
    // Gegeben: Kategorie "Brot" und "Nur Gute" aktiv (via URL-Parameter)
    const suffix = `k9-${Date.now()}-${Math.random().toString(36).slice(2, 7)}`;
    await createRecipeWithOptions(page, `Dinkelbrot-${suffix}`, ['Brot'], 4);
    await createRecipeWithOptions(page, `Roggenbrot-${suffix}`, ['Brot'], 2);

    await page.goto(`/?kategorie=Brot&bewertung=gut&filter_collapsed=0`);

    // Dann: "Dinkelbrot" sichtbar, "Roggenbrot" nicht (4 Sterne >= 3, 2 Sterne < 3)
    await expect(page.locator('.recipe-item h2', { hasText: `Dinkelbrot-${suffix}` })).toBeVisible();
    await expect(page.locator('.recipe-item h2', { hasText: `Roggenbrot-${suffix}` })).not.toBeVisible();

    // Wenn: Klick auf "Nur Gute" zum Deaktivieren
    await page.locator('a.sort-filter-btn', { hasText: '★★★+ Nur Gute' }).click();

    // Dann: Kategorie "Brot" noch aktiv
    await expect(page.locator('a.category-filter-btn', { hasText: 'Brot' })).toHaveAttribute('aria-pressed', 'true');

    // Und: Bewertungsfilter inaktiv
    await expect(page.locator('a.sort-filter-btn', { hasText: '★★★+ Nur Gute' })).toHaveAttribute('aria-pressed', 'false');

    // Und: URL enthält nur noch Kategorie-Parameter
    await expect(page).toHaveURL(/kategorie=Brot/);
    await expect(page).not.toHaveURL(/bewertung/);

    // Und: Beide Brot-Rezepte jetzt sichtbar
    await expect(page.locator('.recipe-item h2', { hasText: `Dinkelbrot-${suffix}` })).toBeVisible();
    await expect(page.locator('.recipe-item h2', { hasText: `Roggenbrot-${suffix}` })).toBeVisible();
  });

  test('K12: Keine Treffer durch Kombination zeigt Hinweistext', async ({ page }) => {
    // Gegeben: Nur Brot-Rezepte mit max. 2 Sternen (eindeutiger Suffix)
    const suffix = `k12-${Date.now()}-${Math.random().toString(36).slice(2, 7)}`;
    await createRecipeWithOptions(page, `Brötchen-${suffix}`, ['Brot'], 2);

    // Wenn: Kombination, die kein Ergebnis liefert
    await page.goto(`/?q=Brötchen-${suffix}&kategorie=Brot&bewertung=favoriten`);

    // Dann: .search-no-results sichtbar
    await expect(page.locator('.search-no-results')).toBeVisible();

    // Und: Keine Rezept-Items sichtbar
    await expect(page.locator('.recipe-item')).toHaveCount(0);
  });

  test('K11: DeepLink mit mehreren Filtern zeigt korrekte Ansicht', async ({ page }) => {
    // Gegeben: "Dinkelbrot" (Brot, 5 Sterne) und "Roggenbrot" (Brot, 2 Sterne) existieren
    const suffix = `k11-${Date.now()}-${Math.random().toString(36).slice(2, 7)}`;
    await createRecipeWithOptions(page, `Dinkelbrot-${suffix}`, ['Brot'], 5);
    await createRecipeWithOptions(page, `Roggenbrot-${suffix}`, ['Brot'], 2);

    // Wenn: URL direkt aufrufen
    await page.goto(`/?kategorie=Brot&bewertung=favoriten`);

    // Dann: Nur "Dinkelbrot" sichtbar
    await expect(page.locator('.recipe-item h2', { hasText: `Dinkelbrot-${suffix}` })).toBeVisible();
    await expect(page.locator('.recipe-item h2', { hasText: `Roggenbrot-${suffix}` })).not.toBeVisible();

    // Und: Kategorie "Brot" als aktiv markiert
    await expect(page.locator('a.category-filter-btn', { hasText: 'Brot' })).toHaveAttribute('aria-pressed', 'true');

    // Und: Bewertungsfilter "Favoriten" aktiv
    await expect(page.locator('a.sort-filter-btn', { hasText: '★★★★★ Favoriten' })).toHaveAttribute('aria-pressed', 'true');
  });

  test('K13: Kombination Bewertung + Volltextsuche', async ({ page }) => {
    // Gegeben: "Dinkelbrot" (4 Sterne), "Dinkelpfannkuchen" (1 Stern) — beide enthalten "Dinkel"
    const suffix = `k13-${Date.now()}-${Math.random().toString(36).slice(2, 7)}`;
    await createRecipeWithOptions(page, `Dinkelbrot-${suffix}`, ['Brot'], 4);
    await createRecipeWithOptions(page, `Dinkelpfannkuchen-${suffix}`, ['Snacks'], 1);

    // Wenn: Suche nach "Dinkelbrot-{suffix}" + Filter "gut" — direkte URL
    await page.goto(`/?q=Dinkelbrot-${suffix}&bewertung=gut`);

    // Dann: "Dinkelbrot" sichtbar (4 Sterne, enthält Suchbegriff)
    await expect(page.locator('.recipe-item h2', { hasText: `Dinkelbrot-${suffix}` })).toBeVisible();

    // Und: "Dinkelpfannkuchen" nicht sichtbar (1 Stern, nicht im Suchbegriff enthalten)
    await expect(page.locator('.recipe-item h2', { hasText: `Dinkelpfannkuchen-${suffix}` })).not.toBeVisible();

    // Und: URL enthält bewertung=gut
    await expect(page).toHaveURL(/bewertung=gut/);
  });

  test('K10: "Alle Filter zurücksetzen"-Button erscheint nur bei aktiven Filtern', async ({ page }) => {
    // Gegeben: Keine Filter aktiv
    await page.goto('/');

    // Dann: Kein "Alle Filter zurücksetzen"-Button
    await expect(page.locator('a.reset-all-filters-btn')).not.toBeVisible();

    // Wenn: Ein Filter aktiviert wird
    await page.goto('/?bewertung=gut&filter_collapsed=0');

    // Dann: "Alle Filter zurücksetzen"-Button erscheint
    await expect(page.locator('a.reset-all-filters-btn')).toBeVisible();
  });

  test('K10: Klick auf "Alle Filter zurücksetzen" setzt alle Filter zurück', async ({ page }) => {
    // Gegeben: Kategorie "Brot" und Bewertung "Nur Gute" aktiv
    const suffix = `k10-${Date.now()}-${Math.random().toString(36).slice(2, 7)}`;
    await createRecipeWithOptions(page, `Dinkelbrot-${suffix}`, ['Brot'], 4);
    await createRecipeWithOptions(page, `Roggenbrot-${suffix}`, ['Brot'], 2);
    await createRecipeWithOptions(page, `Spaghetti-${suffix}`, ['Mittagessen'], 4);

    await page.goto(`/?kategorie=Brot&bewertung=gut&filter_collapsed=0`);

    // Wenn: Klick auf "Alle Filter zurücksetzen"
    await page.locator('a.reset-all-filters-btn').click();

    // Dann: URL = /
    await expect(page).toHaveURL('/');

    // Und: Alle Filter inaktiv
    await expect(page.locator('a.category-filter-btn', { hasText: 'Brot' })).toHaveAttribute('aria-pressed', 'false');
    await expect(page.locator('a.sort-filter-btn', { hasText: '★★★+ Nur Gute' })).toHaveAttribute('aria-pressed', 'false');

    // Und: Kein "Alle Filter zurücksetzen"-Button mehr sichtbar
    await expect(page.locator('a.reset-all-filters-btn')).not.toBeVisible();

    // Und: Alle Rezepte sichtbar
    await expect(page.locator('.recipe-item h2', { hasText: `Dinkelbrot-${suffix}` })).toBeVisible();
    await expect(page.locator('.recipe-item h2', { hasText: `Roggenbrot-${suffix}` })).toBeVisible();
    await expect(page.locator('.recipe-item h2', { hasText: `Spaghetti-${suffix}` })).toBeVisible();
  });

});
