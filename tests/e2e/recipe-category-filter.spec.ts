import { test, expect } from '@playwright/test';

/**
 * E2E-Tests für Story 8: Filter nach Kategorien
 *
 * Die Tests erstellen Rezepte direkt über das Formular für Isolation.
 */

async function createRecipe(
  page: import('@playwright/test').Page,
  title: string,
  categories: string[],
  ingredients?: string,
  instructions?: string
): Promise<void> {
  await page.goto('/recipes/new');
  await page.fill('input[name="title"]', title);
  for (const category of categories) {
    await page.check(`input[name="categories"][value="${category}"]`);
  }
  if (ingredients) {
    await page.fill('textarea[name="ingredients"]', ingredients);
  }
  if (instructions) {
    await page.fill('textarea[name="instructions"]', instructions);
  }
  await page.click('button[type="submit"]');
  await expect(page).toHaveURL(/\/recipes\/\d+/);
}

test.describe('Kategorie-Filter (Story 8)', () => {

  test('K1: Alle fünf Kategorien sind auf der Startseite sichtbar', async ({ page }) => {
    // Given: Die Startseite wird aufgerufen
    await page.goto('/');

    // Then: Alle 5 Kategorie-Buttons sind in der richtigen Reihenfolge sichtbar
    const filterNav = page.locator('nav.category-filter');
    await expect(filterNav).toBeVisible();

    const buttons = filterNav.locator('a.category-filter-btn');
    // "Alle" + 5 Kategorien = 6 Buttons
    await expect(buttons).toHaveCount(6);

    // Reihenfolge prüfen: Alle, Mittagessen, Brot, Party, Kuchen, Snacks
    await expect(buttons.nth(0)).toContainText('Alle');
    await expect(buttons.nth(1)).toContainText('Mittagessen');
    await expect(buttons.nth(2)).toContainText('Brot');
    await expect(buttons.nth(3)).toContainText('Party');
    await expect(buttons.nth(4)).toContainText('Kuchen');
    await expect(buttons.nth(5)).toContainText('Snacks');
  });

  test('K2: Einzelne Kategorie filtern zeigt korrekte Rezepte und markiert Button aktiv', async ({ page }) => {
    // Given: "Vollkornbrot" (Brot) und "Spaghetti Bolognese" (Mittagessen) existieren
    const suffix = Date.now();
    await createRecipe(page, `Vollkornbrot ${suffix}`, ['Brot']);
    await createRecipe(page, `Spaghetti Bolognese ${suffix}`, ['Mittagessen']);

    // When: Benutzer klickt auf Kategorie "Brot"
    await page.goto('/');
    await page.locator('nav.category-filter a').filter({ hasText: 'Brot' }).click();

    // Then: Nur "Vollkornbrot" ist sichtbar
    await expect(page.locator('#recipe-results')).toContainText(`Vollkornbrot ${suffix}`);
    await expect(page.locator('#recipe-results')).not.toContainText(`Spaghetti Bolognese ${suffix}`);

    // And: URL enthält kategorie=Brot
    await expect(page).toHaveURL(/kategorie=Brot/);

    // And: "Brot"-Button ist visuell aktiv (aria-pressed="true")
    const brotButton = page.locator('nav.category-filter a').filter({ hasText: 'Brot' });
    await expect(brotButton).toHaveAttribute('aria-pressed', 'true');
    await expect(brotButton).toHaveClass(/active/);
  });

  test('K3: Mehrere Kategorien gleichzeitig auswählen (ODER-Logik)', async ({ page }) => {
    // Given: Käsekuchen (Kuchen), Partybrot (Brot + Party), Spaghetti (Mittagessen)
    const suffix = Date.now();
    await createRecipe(page, `Käsekuchen ${suffix}`, ['Kuchen']);
    await createRecipe(page, `Partybrot ${suffix}`, ['Brot', 'Party']);
    await createRecipe(page, `Spaghetti ${suffix}`, ['Mittagessen']);

    // When: Benutzer wählt Kategorie "Kuchen", dann "Brot"
    await page.goto('/');
    await page.locator('nav.category-filter a').filter({ hasText: 'Kuchen' }).click();
    await page.locator('nav.category-filter a').filter({ hasText: 'Brot' }).click();

    // Then: Käsekuchen und Partybrot sind sichtbar, Spaghetti nicht
    await expect(page.locator('#recipe-results')).toContainText(`Käsekuchen ${suffix}`);
    await expect(page.locator('#recipe-results')).toContainText(`Partybrot ${suffix}`);
    await expect(page.locator('#recipe-results')).not.toContainText(`Spaghetti ${suffix}`);

    // And: Beide Buttons sind als aktiv markiert
    const kuchenButton = page.locator('nav.category-filter a').filter({ hasText: 'Kuchen' });
    const brotButton = page.locator('nav.category-filter a').filter({ hasText: 'Brot' });
    await expect(kuchenButton).toHaveAttribute('aria-pressed', 'true');
    await expect(brotButton).toHaveAttribute('aria-pressed', 'true');
  });

  test('K4: Filter zurücksetzen via "Alle"-Link', async ({ page }) => {
    // Given: Brot-Filter ist aktiv, ein Rezept wird angezeigt
    const suffix = Date.now();
    await createRecipe(page, `Vollkornbrot ${suffix}`, ['Brot']);
    await createRecipe(page, `Spaghetti ${suffix}`, ['Mittagessen']);

    await page.goto(`/?kategorie=Brot`);
    await expect(page.locator('#recipe-results')).toContainText(`Vollkornbrot ${suffix}`);
    await expect(page.locator('#recipe-results')).not.toContainText(`Spaghetti ${suffix}`);

    // When: Benutzer klickt auf "Alle"
    await page.locator('nav.category-filter a').filter({ hasText: 'Alle' }).click();

    // Then: Alle Rezepte sind wieder sichtbar
    await expect(page.locator('#recipe-results')).toContainText(`Vollkornbrot ${suffix}`);
    await expect(page.locator('#recipe-results')).toContainText(`Spaghetti ${suffix}`);

    // And: URL hat keinen kategorie-Parameter mehr
    const url = page.url();
    expect(url).not.toMatch(/kategorie=/);
  });

  test('K4: Aktive Kategorie nochmals klicken hebt Filter auf', async ({ page }) => {
    // Given: Brot-Filter ist aktiv
    const suffix = Date.now();
    await createRecipe(page, `Vollkornbrot ${suffix}`, ['Brot']);
    await createRecipe(page, `Spaghetti ${suffix}`, ['Mittagessen']);

    await page.goto(`/?kategorie=Brot`);
    await expect(page.locator('#recipe-results')).toContainText(`Vollkornbrot ${suffix}`);

    // When: Benutzer klickt erneut auf "Brot" (Toggle)
    await page.locator('nav.category-filter a').filter({ hasText: 'Brot' }).click();

    // Then: Alle Rezepte sind wieder sichtbar
    await expect(page.locator('#recipe-results')).toContainText(`Vollkornbrot ${suffix}`);
    await expect(page.locator('#recipe-results')).toContainText(`Spaghetti ${suffix}`);
  });

  test('K5: Kategorie ohne eigene Rezepte zeigt korrekte Leer-Meldung im URL-Direktaufruf', async ({ page }) => {
    // Given: Ein Brot-Rezept mit eindeutigem Suffix existiert, aber kein Rezept dieser Kategorie wird erwartet
    const suffix = Date.now();
    await createRecipe(page, `Roggenbrot ${suffix}`, ['Brot']);

    // When: URL direkt mit einer Kategorie aufgerufen wird, die dieses Rezept nicht enthält
    // Wir nutzen einen direkten GET um den Leer-Zustand sicher zu testen
    await page.goto(`/?kategorie=Brot&q=xyzxyzxyz_unique_${suffix}`);

    // Then: Eine Meldung für leere Ergebnisse erscheint, das Rezept ist nicht sichtbar
    await expect(page.locator('#recipe-results')).toContainText('Keine Rezepte');
    await expect(page.locator('#recipe-results')).not.toContainText(`Roggenbrot ${suffix}`);
  });

  test('K6: DeepLink mit ?kategorie= zeigt gefilterte Ansicht', async ({ page }) => {
    // Given: Ein Party-Rezept existiert
    const suffix = Date.now();
    await createRecipe(page, `Partykuchen ${suffix}`, ['Party']);
    await createRecipe(page, `Salat ${suffix}`, ['Mittagessen']);

    // When: URL direkt mit ?kategorie=Party aufgerufen wird
    await page.goto(`/?kategorie=Party`);

    // Then: Nur Party-Rezepte werden angezeigt
    await expect(page.locator('#recipe-results')).toContainText(`Partykuchen ${suffix}`);
    await expect(page.locator('#recipe-results')).not.toContainText(`Salat ${suffix}`);

    // And: "Party"-Button ist visuell aktiv
    const partyButton = page.locator('nav.category-filter a').filter({ hasText: 'Party' });
    await expect(partyButton).toHaveAttribute('aria-pressed', 'true');
  });

  test('K7: Kombination aus Kategorie-Filter und Volltextsuche', async ({ page }) => {
    // Given: Zwei eindeutige Brot-Rezepte mit Suffix: eines mit "Dinkel", eines mit "Roggen" im Titel
    const suffix = Date.now();
    const dinkelTitle = `Dinkel-${suffix}`;
    const roggenTitle = `Roggen-${suffix}`;
    await createRecipe(page, dinkelTitle, ['Brot']);
    await createRecipe(page, roggenTitle, ['Brot']);

    // When: Kategorie "Brot" aktiv und Suchbegriff "Dinkel-<suffix>" eingegeben
    // Der Suchbegriff ist ein exakter Substring des Dinkel-Titels, aber nicht des Roggen-Titels
    await page.goto(`/?kategorie=Brot&q=Dinkel-${suffix}`);

    // Then: Nur das Dinkel-Rezept wird angezeigt
    await expect(page.locator('#recipe-results')).toContainText(dinkelTitle);
    await expect(page.locator('#recipe-results')).not.toContainText(roggenTitle);

    // And: Beide Filter sind in der URL sichtbar
    await expect(page).toHaveURL(/kategorie=Brot/);
    await expect(page).toHaveURL(/q=/);
  });

  test('K1/K9: Kategorie-Buttons haben korrekte ARIA-Attribute', async ({ page }) => {
    // Given: Die Startseite ist geöffnet (kein Filter aktiv)
    await page.goto('/');

    // Then: "Alle"-Button hat aria-pressed="true" (da kein Filter aktiv)
    const alleButton = page.locator('nav.category-filter a').filter({ hasText: 'Alle' });
    await expect(alleButton).toHaveAttribute('aria-pressed', 'true');

    // And: Alle Kategorie-Buttons haben aria-pressed="false"
    for (const cat of ['Mittagessen', 'Brot', 'Party', 'Kuchen', 'Snacks']) {
      const btn = page.locator('nav.category-filter a').filter({ hasText: cat });
      await expect(btn).toHaveAttribute('aria-pressed', 'false');
    }

    // And: nav hat aria-label
    await expect(page.locator('nav.category-filter')).toHaveAttribute('aria-label', 'Nach Kategorie filtern');
  });

});
