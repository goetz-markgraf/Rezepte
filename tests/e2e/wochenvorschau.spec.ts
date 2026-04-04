import { test, expect } from '@playwright/test';

/**
 * E2E-Tests für Story 18: Wochenvorschau - aktualisiert für 15-Tage-Liste (Story 38)
 *
 * Die Tests erstellen Rezepte direkt über das Formular für Isolation.
 * Datumswerte werden relativ zu heute berechnet.
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

/**
 * Berechnet ein Datum relativ zu heute.
 */
function futureDateInDays(days: number): string {
  const d = new Date();
  d.setDate(d.getDate() + days);
  return `${d.getDate()}.${d.getMonth() + 1}.${d.getFullYear()}`;
}

test.describe('Wochenvorschau (Story 18)', () => {
  test('K1: /wochenvorschau ist aufrufbar und in Navigation verlinkt', async ({ page }) => {
    // Given: Die App ist gestartet
    await page.goto('/');

    // Then: Link "Wochenvorschau" in der Navigation sichtbar
    const navLink = page.locator('a[href="/wochenvorschau"]');
    await expect(navLink).toBeVisible();
    await expect(navLink).toContainText('Wochenvorschau');

    // When: Benutzer klickt den Link
    await navLink.click();

    // Then: URL ist /wochenvorschau, HTTP 200, Überschrift sichtbar
    await expect(page).toHaveURL(/\/wochenvorschau/);
    await expect(page.locator('h1')).toBeVisible();
  });

  test('K2: Alle 15 Tage erscheinen auf der Seite', async ({ page }) => {
    // Given: /wochenvorschau wird aufgerufen
    await page.goto('/wochenvorschau');

    // Then: 15 Tage sichtbar
    const tagElemente = page.locator('.wochentag-abschnitt');
    await expect(tagElemente).toHaveCount(15);
  });

  test('K3: Rezept mit planned_date wird auf der Seite angezeigt', async ({ page }) => {
    // Given: "Spaghetti Bolognese" mit planned_date = heute
    const suffix = Date.now();
    const title = `Spaghetti ${suffix}`;
    const today = futureDateInDays(0);
    await createRecipeWithDate(page, title, ['Mittagessen'], today);

    // When: Benutzer öffnet /wochenvorschau
    await page.goto('/wochenvorschau');

    // Then: Rezept erscheint auf der Seite
    await expect(page.locator('body')).toContainText(title);

    // And: Rezeptname ist als klickbarer Link vorhanden
    const recipeLink = page.locator(`a:has-text("${title}")`);
    await expect(recipeLink).toBeVisible();
  });

  test('K4: Mehrere Rezepte am gleichen Tag erscheinen beide', async ({ page }) => {
    // Given: Zwei Rezepte mit gleichem planned_date (heute)
    const suffix = Date.now();
    const title1 = `Pfannkuchen ${suffix}`;
    const title2 = `Rührei ${suffix}`;
    const today = futureDateInDays(0);
    await createRecipeWithDate(page, title1, ['Mittagessen'], today);
    await createRecipeWithDate(page, title2, ['Mittagessen'], today);

    // When: Benutzer öffnet /wochenvorschau
    await page.goto('/wochenvorschau');

    // Then: Beide Rezepte sichtbar
    await expect(page.locator('body')).toContainText(title1);
    await expect(page.locator('body')).toContainText(title2);
  });

  test('K5: Seite lädt auch ohne Rezepte in der Woche (Hinweis "Nichts geplant" für leere Tage)', async ({ page }) => {
    // Given: /wochenvorschau wird aufgerufen
    await page.goto('/wochenvorschau');

    // Then: Für Tage ohne Rezept erscheint "Nichts geplant"
    await expect(page.locator('body')).toContainText('Nichts geplant');
  });

  test('K3/K6: Rezeptlink führt zur Detailansicht', async ({ page }) => {
    // Given: Rezept mit planned_date = heute
    const suffix = Date.now();
    const title = `Detaillink-Rezept ${suffix}`;
    const today = futureDateInDays(0);
    await createRecipeWithDate(page, title, ['Mittagessen'], today);

    // When: Benutzer öffnet /wochenvorschau und klickt auf Rezeptnamen
    await page.goto('/wochenvorschau');
    const recipeLink = page.locator(`a:has-text("${title}")`);
    await expect(recipeLink).toBeVisible();
    await recipeLink.click();

    // Then: Benutzer sieht die Detailansicht (/recipes/{id}), Titel sichtbar
    await expect(page).toHaveURL(/\/recipes\/\d+/);
    await expect(page.locator('h1')).toContainText(title);
  });

  test('K5/Deeplink: /wochenvorschau direkt per URL aufrufbar', async ({ page }) => {
    // Given: Ein Rezept mit planned_date = heute
    const suffix = Date.now();
    const title = `Deeplink-Rezept ${suffix}`;
    const today = futureDateInDays(0);
    await createRecipeWithDate(page, title, ['Mittagessen'], today);

    // When: Benutzer ruft /wochenvorschau direkt auf (Bookmark-Simulation)
    await page.goto('/wochenvorschau');

    // Then: HTTP 200, Wochenvorschau korrekt angezeigt, Rezept sichtbar
    await expect(page).toHaveURL(/\/wochenvorschau/);
    await expect(page.locator('body')).toContainText(title);
  });

  test('K7: Link zurück zur Rezeptliste vorhanden', async ({ page }) => {
    // Given: /wochenvorschau wird aufgerufen
    await page.goto('/wochenvorschau');

    // Then: Link "Zur Rezeptliste" vorhanden (mit filter_collapsed=1)
    const backLink = page.locator('a[href="/?filter_collapsed=1"]', { hasText: 'Zur Rezeptliste' });
    await expect(backLink).toBeVisible();

    // When: Link geklickt
    await backLink.click();

    // Then: Benutzer ist auf der Rezeptliste mit eingeklappten Filtern
    await expect(page).toHaveURL(/filter_collapsed=1/);
  });

  test('K9: Semantisches HTML — Tage als dt-Elemente', async ({ page }) => {
    // Given: /wochenvorschau wird aufgerufen
    await page.goto('/wochenvorschau');

    // Then: Wochentag-Bezeichnungen sind in dt-Elementen
    const dtElements = page.locator('dt.wochentag-titel');
    await expect(dtElements).toHaveCount(15);

    // And: dl.wochenvorschau-liste vorhanden
    await expect(page.locator('dl.wochenvorschau-liste')).toBeVisible();
  });

  test('Zeitraum-Anzeige: Zeitraum sichtbar', async ({ page }) => {
    // Given: /wochenvorschau wird aufgerufen
    await page.goto('/wochenvorschau');

    // Then: Zeitraum-Anzeige sichtbar (Format: "04.04.2026 – 18.04.2026")
    await expect(page.locator('.zeitraum-label')).toBeVisible();
    const zeitraumText = await page.locator('.zeitraum-label').textContent();
    expect(zeitraumText).toMatch(/\d{2}\.\d{2}\.\d{4}\s*–\s*\d{2}\.\d{2}\.\d{4}/);
  });
});

test.describe('Wochenvorschau Formatierung (Story 19)', () => {
  test('K1: 15x span.wochentag-name und 15x span.wochentag-datum sichtbar', async ({ page }) => {
    // Given: Die Wochenvorschau ist geöffnet
    await page.goto('/wochenvorschau');

    // Then: span.wochentag-name Elemente sichtbar (15 Stück)
    const nameElems = page.locator('strong.wochentag-name');
    await expect(nameElems).toHaveCount(15);

    // And: span.wochentag-datum Elemente sichtbar (15 Stück)
    const datumElems = page.locator('span.wochentag-datum');
    await expect(datumElems).toHaveCount(15);
  });

  test('K2: Genau ein Element mit Klasse wochentag-heute', async ({ page }) => {
    // Given: /wochenvorschau
    await page.goto('/wochenvorschau');

    // Then: Genau ein Element mit Klasse wochentag-heute
    const heuteElems = page.locator('.wochentag-heute');
    await expect(heuteElems).toHaveCount(1);
  });

  test('K2: Heute-Badge sichtbar mit Text "Heute"', async ({ page }) => {
    // Given: /wochenvorschau
    await page.goto('/wochenvorschau');

    // Then: .heute-badge mit Text "Heute" sichtbar
    const badge = page.locator('.heute-badge');
    await expect(badge).toBeVisible();
    await expect(badge).toContainText('Heute');
  });

  test('K3: Keine vergangenen Tage in der Liste', async ({ page }) => {
    // Given: /wochenvorschau aufgerufen
    await page.goto('/wochenvorschau');

    // Then: Keine Elemente mit wochentag-vergangen (15-Tage-Liste beginnt ab heute)
    await expect(page.locator('.wochentag-vergangen')).toHaveCount(0);
  });

  test('K4: Smoke-Test — Rezept für heute erscheint unter .wochentag-heute', async ({ page }) => {
    // Given: Rezept für heute erstellt
    const suffix = Date.now();
    const title = `Story19-Smoke-${suffix}`;
    const today = futureDateInDays(0);
    await createRecipeWithDate(page, title, ['Mittagessen'], today);

    // When: /wochenvorschau aufgerufen
    await page.goto('/wochenvorschau');

    // Then: Rezept unter heutigem Tag sichtbar
    await expect(page.locator('.wochentag-heute')).toContainText(title);

    // And: Link zur Detailansicht vorhanden
    await expect(page.locator(`.wochentag-heute a:has-text("${title}")`)).toBeVisible();
  });
});

test.describe('Wochenübersicht Navigation (Story 38)', () => {
  test('Navigation ist nicht vorhanden', async ({ page }) => {
    // Given: /wochenvorschau wird aufgerufen (aktuelle Woche)
    await page.goto('/wochenvorschau');

    // Then: Keine Navigation-Links sichtbar
    const prevLink = page.locator('a[href*="week="], .wochen-nav-prev, [aria-label="Vorherige Woche"]');
    await expect(prevLink).toHaveCount(0);

    const nextLink = page.locator('a[href*="week="], .wochen-nav-next, [aria-label="Nächste Woche"]');
    await expect(nextLink).toHaveCount(0);
  });

  test('"Nächste 15 Tage"-Badge bei der Liste sichtbar', async ({ page }) => {
    // Given: /wochenvorschau ohne Parameter
    await page.goto('/wochenvorschau');

    // Then: Badge "Nächste 15 Tage" ist sichtbar
    await expect(page.locator('.current-week-badge')).toBeVisible();
    await expect(page.locator('.current-week-badge')).toContainText('Nächste 15 Tage');
  });
});
