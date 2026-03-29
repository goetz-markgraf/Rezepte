import { test, expect } from '@playwright/test';

interface RecipeDetails {
  title: string;
  rating?: number;
  ingredients?: string;
  instructions?: string;
  categories?: string[];
}

/**
 * Erstellt ein Rezept über das Formular und gibt die ID zurück.
 */
async function createRecipeWithDetails(
  page: import('@playwright/test').Page,
  details: RecipeDetails
): Promise<string> {
  await page.goto('/recipes/new');
  await page.fill('input[name="title"]', details.title);

  const categories = details.categories ?? ['Mittagessen'];
  for (const cat of categories) {
    await page.check(`input[name="categories"][value="${cat}"]`);
  }

  if (details.rating !== undefined) {
    // Klick auf das Label des Sterne-Radio-Buttons
    const input = page.locator(`input[name="rating"][value="${details.rating}"]`);
    const label = input.locator('xpath=ancestor::label');
    await label.click();
  }

  if (details.ingredients) {
    await page.fill('textarea[name="ingredients"]', details.ingredients);
  }

  if (details.instructions) {
    await page.fill('textarea[name="instructions"]', details.instructions);
  }

  await page.click('button[type="submit"]');
  await expect(page).toHaveURL(/\/recipes\/\d+/);
  const url = page.url();
  return url.split('/').pop()!.split('?')[0];
}

test.describe('Rezepte-Merge', () => {
  test('K1: Merge-Button auf Dubletten-Übersicht sichtbar', async ({ page }) => {
    // Given: Zwei ähnliche Rezepte existieren
    const ts = Date.now();
    const titelA = `PizzaTS${ts}`;
    const titelB = `PizzaTS${ts}Margherita`;
    await createRecipeWithDetails(page, { title: titelA });
    await createRecipeWithDetails(page, { title: titelB });

    // When: Benutzer öffnet /recipes/duplicates
    await page.goto('/recipes/duplicates');

    // Then: Ein "Mergen"-Link für dieses Paar ist sichtbar
    const pair = page.locator('.duplicate-pair').filter({ hasText: titelA });
    await expect(pair).toBeVisible();
    const mergeLink = pair.locator('a[href*="/recipes/merge"]');
    await expect(mergeLink).toBeVisible();
    await expect(mergeLink).toContainText('Mergen');

    // And: Link führt zu /recipes/merge?source=...&target=...
    const href = await mergeLink.getAttribute('href');
    expect(href).toContain('/recipes/merge');
    expect(href).toContain('source=');
    expect(href).toContain('target=');
  });

  test('K2: Merge-Seite zeigt beide Rezepte vollständig', async ({ page }) => {
    // Given: Rezept A mit Bewertung, Rezept B mit Zutaten
    const ts = Date.now();
    const idA = await createRecipeWithDetails(page, {
      title: `MergeTest${ts}A`,
      rating: 5,
    });
    const idB = await createRecipeWithDetails(page, {
      title: `MergeTest${ts}B`,
      ingredients: `Mehl, Wasser, Hefe (${ts})`,
    });

    // When: Benutzer öffnet /recipes/merge?source=idA&target=idB
    await page.goto(`/recipes/merge?source=${idA}&target=${idB}`);

    // Then: Seite lädt mit "zusammenführen" im Titel
    await expect(page.locator('h1')).toContainText('zusammenführen');

    // And: Titel beider Rezepte sind sichtbar
    await expect(page.locator('body')).toContainText(`MergeTest${ts}A`);
    await expect(page.locator('body')).toContainText(`MergeTest${ts}B`);

    // And: Zutaten aus Rezept B sind sichtbar
    await expect(page.locator('body')).toContainText('Mehl, Wasser, Hefe');
  });

  test('K3: source_id und target_id als Hidden-Felder im Form', async ({ page }) => {
    // Given: Zwei Rezepte existieren
    const ts = Date.now();
    const idA = await createRecipeWithDetails(page, { title: `HiddenTest${ts}A` });
    const idB = await createRecipeWithDetails(page, { title: `HiddenTest${ts}B` });

    // When: Merge-Seite wird geöffnet
    await page.goto(`/recipes/merge?source=${idA}&target=${idB}`);

    // Then: Hidden-Felder sind vorhanden
    const sourceInput = page.locator('input[type="hidden"][name="source_id"]');
    const targetInput = page.locator('input[type="hidden"][name="target_id"]');
    await expect(sourceInput).toBeAttached();
    await expect(targetInput).toBeAttached();

    const sourceValue = await sourceInput.getAttribute('value');
    const targetValue = await targetInput.getAttribute('value');
    expect(sourceValue).toBe(idA);
    expect(targetValue).toBe(idB);

    // And: Beide Rezept-Spalten sind klar beschriftet
    await expect(page.locator('.merge-recipe-a')).toContainText('wird gelöscht');
    await expect(page.locator('.merge-recipe-b')).toContainText('bleibt erhalten');
  });

  test('K4: Radio-Buttons für Titel-Auswahl bei Konflikt', async ({ page }) => {
    // Given: Beide Rezepte haben unterschiedliche Titel
    const ts = Date.now();
    const idA = await createRecipeWithDetails(page, { title: `KonfliktA${ts}` });
    const idB = await createRecipeWithDetails(page, { title: `KonfliktB${ts}` });

    // When: Merge-Seite wird geöffnet
    await page.goto(`/recipes/merge?source=${idA}&target=${idB}`);

    // Then: Radio-Buttons für Titel-Auswahl sind sichtbar
    const radioA = page.locator('input[type="radio"][name="title_from"][value="a"]');
    const radioB = page.locator('input[type="radio"][name="title_from"][value="b"]');
    await expect(radioA).toBeAttached();
    await expect(radioB).toBeAttached();

    // And: Nutzer kann Rezept A oder B für den Titel wählen
    await radioA.check();
    await expect(radioA).toBeChecked();
    await radioB.check();
    await expect(radioB).toBeChecked();
  });

  test('K4b: Automatische Übernahme bei einseitigem Inhalt', async ({ page }) => {
    // Given: Rezept A hat Zutaten, Rezept B hat keine
    const ts = Date.now();
    const idA = await createRecipeWithDetails(page, {
      title: `AutoA${ts}`,
      ingredients: `Nur-A-Zutaten-${ts}`,
    });
    const idB = await createRecipeWithDetails(page, { title: `AutoB${ts}` });

    // When: Merge-Seite wird geöffnet
    await page.goto(`/recipes/merge?source=${idA}&target=${idB}`);

    // Then: Zutaten aus A sind automatisch übernommen (kein Radio-Button, Info-Text)
    await expect(page.locator('body')).toContainText(`Nur-A-Zutaten-${ts}`);
    // Radio-Button für Zutaten existiert nur bei Konflikt
    const radioIngredients = page.locator('input[type="radio"][name="ingredients_from"]');
    await expect(radioIngredients).toHaveCount(0);
    // Auto-Badge ist sichtbar
    await expect(page.locator('.merge-auto-badge').first()).toBeVisible();
  });

  test('K5+K6: Erfolgreicher Merge-Durchlauf', async ({ page }) => {
    // Given: Rezept A mit Bewertung, Rezept B mit Zutaten
    const ts = Date.now();
    const idA = await createRecipeWithDetails(page, {
      title: `MergeA${ts}`,
      rating: 5,
    });
    const idB = await createRecipeWithDetails(page, {
      title: `MergeB${ts}`,
      ingredients: `Spezial-Zutaten-${ts}`,
    });

    // When: Benutzer öffnet Merge-Seite
    await page.goto(`/recipes/merge?source=${idA}&target=${idB}`);

    // And: Titel aus Rezept A wählen (Radio-Button)
    const titleRadioA = page.locator('input[type="radio"][name="title_from"][value="a"]');
    if (await titleRadioA.isVisible()) {
      await titleRadioA.check();
    }

    // And: Benutzer klickt "Zusammenführen"
    await page.click('button[type="submit"]');

    // Then: Weiterleitung zur Detailansicht des Ziel-Rezepts (idB)
    await expect(page).toHaveURL(new RegExp(`/recipes/${idB}`));

    // And: Erfolgsmeldung ist sichtbar
    await expect(page.locator('.success')).toBeVisible();

    // And: Quell-Rezept existiert nicht mehr
    const sourceResponse = await page.request.get(`/recipes/${idA}`);
    expect(sourceResponse.status()).toBe(404);
  });

  test('K7: Abbrechen kehrt zur Dubletten-Übersicht zurück', async ({ page }) => {
    // Given: Zwei Rezepte existieren
    const ts = Date.now();
    const idA = await createRecipeWithDetails(page, { title: `AbbrechA${ts}` });
    const idB = await createRecipeWithDetails(page, { title: `AbbrechB${ts}` });

    // When: Merge-Seite ist geöffnet
    await page.goto(`/recipes/merge?source=${idA}&target=${idB}`);

    // And: Benutzer klickt "Abbrechen"
    await page.click('a:has-text("Abbrechen")');

    // Then: Weiterleitung zu /recipes/duplicates
    await expect(page).toHaveURL('/recipes/duplicates');

    // And: Beide Rezepte existieren noch
    const responseA = await page.request.get(`/recipes/${idA}`);
    expect(responseA.status()).toBe(200);
    const responseB = await page.request.get(`/recipes/${idB}`);
    expect(responseB.status()).toBe(200);
  });

  test('K8: Direktlink (Deeplink) funktioniert', async ({ page }) => {
    // Given: Zwei Rezepte mit bekannten IDs
    const ts = Date.now();
    const idA = await createRecipeWithDetails(page, { title: `DeepA${ts}` });
    const idB = await createRecipeWithDetails(page, { title: `DeepB${ts}` });

    // When: Benutzer ruft /recipes/merge?source=idA&target=idB direkt auf
    await page.goto(`/recipes/merge?source=${idA}&target=${idB}`);

    // Then: Seite lädt korrekt (h1 enthält "zusammenführen", beide Titel sichtbar)
    await expect(page.locator('h1')).toContainText('zusammenführen');
    await expect(page.locator('body')).toContainText(`DeepA${ts}`);
    await expect(page.locator('body')).toContainText(`DeepB${ts}`);
  });

  test('Ungültige IDs → Fehlerseite oder 404', async ({ page }) => {
    // Given: source=99999 existiert nicht
    await page.goto('/recipes/merge?source=99999&target=1');

    // Then: Fehlerseite oder 404-Status
    // (Entweder 404-Page oder eine Fehlermeldung auf der Seite)
    const status = page.locator('body');
    // Die App gibt eine 404-Antwort zurück
    // Playwright lädt die Seite, daher prüfen wir den HTTP-Status
    const response = await page.request.get('/recipes/merge?source=99999&target=1');
    expect(response.status()).toBe(404);
  });
});
