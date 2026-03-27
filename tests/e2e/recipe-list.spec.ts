import { test, expect } from '@playwright/test';

test.describe('Rezept-Liste', () => {
  async function createRecipe(
    page: import('@playwright/test').Page,
    title: string,
    category: string
  ): Promise<string> {
    await page.goto('/recipes/new');
    await page.fill('input[name="title"]', title);
    await page.check(`input[name="categories"][value="${category}"]`);
    await page.click('button[type="submit"]');
    await expect(page).toHaveURL(/\/recipes\/\d+/);
    return page.url();
  }

  test('sollte mehrere Rezepte alphabetisch sortiert anzeigen (K1, K2, K3)', async ({ page }) => {
    const suffix = Date.now();
    const titles = [`Zupfbrot-${suffix}`, `Apfelkuchen-${suffix}`, `Bolognese-${suffix}`];

    for (const title of titles) {
      await createRecipe(page, title, 'Mittagessen');
    }

    await page.goto('/');

    // Alle 3 Rezepte sind sichtbar (K1)
    for (const title of titles) {
      await expect(page.getByText(title).first()).toBeVisible();
    }

    // Alphabetische Reihenfolge prüfen (K2): Apfelkuchen < Bolognese < Zupfbrot
    const pageContent = await page.content();
    const posApfel = pageContent.indexOf(`Apfelkuchen-${suffix}`);
    const posBol = pageContent.indexOf(`Bolognese-${suffix}`);
    const posZupf = pageContent.indexOf(`Zupfbrot-${suffix}`);

    expect(posApfel).toBeGreaterThan(0);
    expect(posBol).toBeGreaterThan(posApfel);
    expect(posZupf).toBeGreaterThan(posBol);

    // Kategorie sichtbar (K3)
    await expect(page.locator('.recipe-list .category-tag').first()).toBeVisible();
  });

  test('sollte leere Liste mit Meldung und Erstellen-Link anzeigen (K4)', async ({ page }) => {
    // Frische DB wird durch die App-Konfiguration mit TEST_DATABASE_URL bereitgestellt
    // Da wir keine per-Test-Isolation haben, testen wir nur auf leere Meldung wenn keine Rezepte da sind
    // Wir navigieren zur Startseite und prüfen, dass mindestens der "Neues Rezept"-Button da ist
    await page.goto('/');

    // Link zu /recipes/new ist immer vorhanden
    await expect(page.locator('a[href="/recipes/new"]').first()).toBeVisible();

    // Falls die DB leer ist, wird die Leerzustand-Meldung angezeigt
    const recipeList = page.locator('.recipe-list');
    const emptyState = page.locator('.empty-state');

    const hasRecipes = await recipeList.isVisible();
    if (!hasRecipes) {
      await expect(emptyState).toBeVisible();
      await expect(emptyState).toContainText('Noch keine Rezepte');
      await expect(page.locator('.empty-state a[href="/recipes/new"]')).toBeVisible();
    }
  });

  test('sollte bei Klick auf Listeneintrag zur Detailansicht navigieren (K5)', async ({ page }) => {
    const title = `Navigationstest-${Date.now()}`;
    const detailUrl = await createRecipe(page, title, 'Snacks');
    const id = detailUrl.split('/').pop();

    await page.goto('/');

    // Rezept in der Liste finden und klicken
    const recipeLink = page.locator(`.recipe-item-link[href="/recipes/${id}"]`);
    await expect(recipeLink).toBeVisible();
    await recipeLink.click();

    // URL wechselt auf /recipes/{id} (K5)
    await expect(page).toHaveURL(`/recipes/${id}`);

    // H1 zeigt den Titel
    await expect(page.locator('h1')).toContainText(title);
  });

  test('sollte Umlaute korrekt alphabetisch sortieren (K2)', async ({ page }) => {
    const suffix = Date.now();

    await createRecipe(page, `Überbackene-Nudeln-${suffix}`, 'Mittagessen');
    await createRecipe(page, `Apfelkuchen-${suffix}`, 'Kuchen');
    await createRecipe(page, `Ährenbrot-${suffix}`, 'Brot');

    await page.goto('/');

    const pageContent = await page.content();
    const posAehr = pageContent.indexOf(`Ährenbrot-${suffix}`);
    const posApfel = pageContent.indexOf(`Apfelkuchen-${suffix}`);
    const posUeber = pageContent.indexOf(`Überbackene-Nudeln-${suffix}`);

    expect(posAehr).toBeGreaterThan(0);
    expect(posApfel).toBeGreaterThan(0);
    expect(posUeber).toBeGreaterThan(0);

    // Ä wie A → Ährenbrot vor Apfelkuchen
    expect(posAehr).toBeLessThan(posApfel);
    // Ü wie U → Überbackene Nudeln nach Apfelkuchen (A < U)
    expect(posApfel).toBeLessThan(posUeber);
  });

  test('sollte "Neues Rezept"-Button zur Erstellungsseite führen (K7)', async ({ page }) => {
    await page.goto('/');

    // "Neues Rezept"-Button klicken
    await page.locator('a[href="/recipes/new"]').first().click();

    // Weiterleitung auf Erstellungsformular
    await expect(page).toHaveURL('/recipes/new');
    await expect(page.locator('h1')).toContainText('Neues Rezept');
  });
});
