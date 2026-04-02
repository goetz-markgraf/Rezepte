import { test, expect } from '@playwright/test';

/**
 * E2E-Tests für Story 34: Suche "Länger nicht gemacht" per Klick in der Wochenübersicht
 * 
 * Diese Tests prüfen den Button in der Wochenvorschau, der direkt zur Suche
 * "Länger nicht gemacht" navigiert.
 */

test.describe('Story 34: "Länger nicht gemacht" Button in Wochenvorschau', () => {
  
  test('K1: Button "Länger nicht gemacht" ist in der Wochenübersicht sichtbar', async ({ page }) => {
    // Given: Wochenvorschau wird aufgerufen
    await page.goto('/wochenvorschau');
    
    // Then: Button "Länger nicht gemacht" ist sichtbar
    const notMadeButton = page.locator('a[href*="filter=laenger-nicht-gemacht"]');
    await expect(notMadeButton).toBeVisible();
    await expect(notMadeButton).toContainText('Länger nicht gemacht');
  });

  test('K2: Button hat korrektes ARIA-Label für Barrierefreiheit', async ({ page }) => {
    // Given: Wochenvorschau wird aufgerufen
    await page.goto('/wochenvorschau');
    
    // Then: Button hat aussagekräftiges ARIA-Label (Story 35: Mittagessen vorselektiert)
    const notMadeButton = page.locator('a[href*="filter=laenger-nicht-gemacht"]');
    await expect(notMadeButton).toHaveAttribute('aria-label', 'Mittagessen-Rezepte anzeigen, die länger nicht gemacht wurden');
  });

  test('K2: Klick öffnet Suche mit vorbelegtem Filter', async ({ page }) => {
    // Given: Wochenvorschau wird aufgerufen
    await page.goto('/wochenvorschau');
    
    // When: Button "Länger nicht gemacht" wird geklickt
    const notMadeButton = page.locator('a[href*="filter=laenger-nicht-gemacht"]');
    await notMadeButton.click();
    
    // Then: URL enthält filter=laenger-nicht-gemacht
    await expect(page).toHaveURL(/filter=laenger-nicht-gemacht/);
    
    // And: Filter-Button ist auf der Startseite als aktiv markiert
    const filterBtn = page.locator('a.sort-filter-btn', { hasText: 'Länger nicht gemacht' });
    await expect(filterBtn).toHaveAttribute('aria-pressed', 'true');
    await expect(filterBtn).toHaveClass(/active/);
  });

  test('K6: Button ist per Tastatur erreichbar', async ({ page }) => {
    // Given: Wochenvorschau wird aufgerufen
    await page.goto('/wochenvorschau');
    
    // When: Tab-Taste drücken um durch die Seite zu navigieren
    // Der Button ist ein Link und sollte fokussierbar sein
    const notMadeButton = page.locator('a[href*="filter=laenger-nicht-gemacht"]');
    await notMadeButton.focus();
    
    // Then: Button "Länger nicht gemacht" ist fokussiert
    await expect(notMadeButton).toBeFocused();
    
    // When: Enter drücken
    await page.keyboard.press('Enter');
    
    // Then: Navigation zur Suche erfolgt
    await expect(page).toHaveURL(/filter=laenger-nicht-gemacht/);
  });

  test('Button ist in der Toolbar oberhalb der Wochenliste positioniert', async ({ page }) => {
    // Given: Wochenvorschau wird aufgerufen
    await page.goto('/wochenvorschau');
    
    // Then: Button befindet sich in einem Container über der Wochenliste
    const toolbar = page.locator('.wochenvorschau-toolbar');
    await expect(toolbar).toBeVisible();
    
    // And: Button ist innerhalb der Toolbar
    const notMadeButton = toolbar.locator('a[href*="filter=laenger-nicht-gemacht"]');
    await expect(notMadeButton).toBeVisible();
  });

  test('Button hat konsistentes Styling mit anderen Buttons', async ({ page }) => {
    // Given: Wochenvorschau wird aufgerufen
    await page.goto('/wochenvorschau');
    
    // Then: Button hat die korrekte CSS-Klasse
    const notMadeButton = page.locator('a.not-made-button');
    await expect(notMadeButton).toBeVisible();
    
    // And: Button hat Hover- und Focus-States
    await notMadeButton.hover();
    // Visuelle Überprüfung wäre hier manuell notwendig
    
    await notMadeButton.focus();
    await expect(notMadeButton).toHaveCSS('outline-color', 'rgb(37, 99, 235)'); // var(--primary-color)
  });
});

/**
 * E2E-Tests für Story 35: Suche "Länger nicht gemacht" mit vorselektiertem Mittagessen-Filter
 *
 * Diese Tests prüfen, dass der Link in der Wochenvorschau direkt mit dem
 * Mittagessen-Filter öffnet.
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

test.describe('Story 35: "Länger nicht gemacht" mit vorselektiertem Mittagessen-Filter', () => {

  test('T1: Klick aus Wochenübersicht öffnet URL mit filter=laenger-nicht-gemacht und kategorie=Mittagessen', async ({ page }) => {
    // Given: Wochenvorschau wird aufgerufen
    await page.goto('/wochenvorschau');

    // When: Button "Länger nicht gemacht" wird geklickt
    const notMadeButton = page.locator('a[href*="filter=laenger-nicht-gemacht"]');
    await notMadeButton.click();

    // Then: URL enthält beide Filter-Parameter
    await expect(page).toHaveURL(/filter=laenger-nicht-gemacht/);
    await expect(page).toHaveURL(/kategorie=Mittagessen/);
  });

  test('T2: Kategorie-Filter "Mittagessen" ist auf der Zielseite als aktiv markiert', async ({ page }) => {
    // Given: Wochenvorschau wird aufgerufen und Link geklickt
    await page.goto('/wochenvorschau');
    const notMadeButton = page.locator('a[href*="filter=laenger-nicht-gemacht"]');
    await notMadeButton.click();

    // Then: Kategorie-Filter "Mittagessen" ist als aktiv markiert
    const mittagssenBtn = page.locator('a.category-filter-btn', { hasText: 'Mittagessen' });
    await expect(mittagssenBtn).toHaveAttribute('aria-pressed', 'true');
  });

  test('T3: Nur Mittagessen-Rezepte werden angezeigt', async ({ page }) => {
    // Given: Mittagessen- und Brot-Rezepte mit Vergangenheitsdaten
    const suffix = `${Date.now()}-${Math.random().toString(36).slice(2, 7)}`;
    await createRecipeWithDate(page, `Spaghetti-${suffix}`, ['Mittagessen'], '1.1.2020');
    await createRecipeWithDate(page, `Dinkelbrot-${suffix}`, ['Brot'], '1.1.2020');

    // When: Klick auf den Button in der Wochenvorschau
    await page.goto('/wochenvorschau');
    const notMadeButton = page.locator('a[href*="filter=laenger-nicht-gemacht"]');
    await notMadeButton.click();

    // Then: Mittagessen-Rezept ist sichtbar
    await expect(page.locator('#recipe-results')).toContainText(`Spaghetti-${suffix}`);

    // And: Brot-Rezept ist nicht sichtbar
    await expect(page.locator('#recipe-results')).not.toContainText(`Dinkelbrot-${suffix}`);
  });

  test('T4: DeepLink /?filter=laenger-nicht-gemacht&kategorie=Mittagessen funktioniert direkt', async ({ page }) => {
    // Given: Mittagessen-Rezept mit Vergangenheitsdatum
    const suffix = `${Date.now()}-${Math.random().toString(36).slice(2, 7)}`;
    await createRecipeWithDate(page, `Gulasch-${suffix}`, ['Mittagessen'], '1.1.2020');

    // When: URL direkt aufgerufen
    await page.goto('/?filter=laenger-nicht-gemacht&kategorie=Mittagessen');

    // Then: Filter "Länger nicht gemacht" ist aktiv
    const filterBtn = page.locator('a.sort-filter-btn', { hasText: 'Länger nicht gemacht' });
    await expect(filterBtn).toHaveAttribute('aria-pressed', 'true');

    // And: Kategorie-Filter "Mittagessen" ist aktiv
    const mittagssenBtn = page.locator('a.category-filter-btn', { hasText: 'Mittagessen' });
    await expect(mittagssenBtn).toHaveAttribute('aria-pressed', 'true');

    // And: Rezept ist sichtbar
    await expect(page.locator('#recipe-results')).toContainText(`Gulasch-${suffix}`);
  });

  test('T5: Mittagessen-Filter kann manuell abgewählt werden', async ({ page }) => {
    // Given: Mittagessen- und Brot-Rezepte mit Vergangenheitsdaten
    const suffix = `${Date.now()}-${Math.random().toString(36).slice(2, 7)}`;
    await createRecipeWithDate(page, `Spaghetti-${suffix}`, ['Mittagessen'], '1.1.2020');
    await createRecipeWithDate(page, `Dinkelbrot-${suffix}`, ['Brot'], '1.1.2020');

    // Given: Gefilterte Suche mit beiden Parametern ist aktiv
    await page.goto('/?filter=laenger-nicht-gemacht&kategorie=Mittagessen');

    // When: Mittagessen-Filter wird abgewählt
    const mittagssenBtn = page.locator('a.category-filter-btn', { hasText: 'Mittagessen' });
    await mittagssenBtn.click();

    // Then: URL enthält nicht mehr den Kategorie-Parameter
    await expect(page).not.toHaveURL(/kategorie=Mittagessen/);

    // And: Brot-Rezept ist jetzt auch sichtbar
    await expect(page.locator('#recipe-results')).toContainText(`Dinkelbrot-${suffix}`);
  });

});

