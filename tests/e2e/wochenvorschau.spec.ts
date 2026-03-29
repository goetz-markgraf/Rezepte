import { test, expect } from '@playwright/test';

/**
 * E2E-Tests für Story 18: Wochenvorschau
 *
 * Die Tests erstellen Rezepte direkt über das Formular für Isolation.
 * Datumswerte werden relativ zur aktuellen Woche berechnet.
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
 * Berechnet ein Datum in der aktuellen Woche (relativ zu Montag).
 * offset=0 → Montag, offset=2 → Mittwoch, offset=6 → Sonntag
 */
function currentWeekDateFromMonday(offsetFromMonday: number): string {
  const today = new Date();
  const dayOfWeek = today.getDay(); // 0=So, 1=Mo, ..., 6=Sa
  // ISO: Montag = 0 offset
  const daysFromMonday = dayOfWeek === 0 ? 6 : dayOfWeek - 1;
  const monday = new Date(today);
  monday.setDate(today.getDate() - daysFromMonday);
  const target = new Date(monday);
  target.setDate(monday.getDate() + offsetFromMonday);
  return `${target.getDate()}.${target.getMonth() + 1}.${target.getFullYear()}`;
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
    await expect(page.locator('h1')).toContainText('Wochenvorschau');
  });

  test('K2: Alle 7 Wochentage erscheinen auf der Seite', async ({ page }) => {
    // Given: /wochenvorschau wird aufgerufen
    await page.goto('/wochenvorschau');

    // Then: Alle deutschen Wochentag-Namen sichtbar
    for (const weekday of ['Montag', 'Dienstag', 'Mittwoch', 'Donnerstag', 'Freitag', 'Samstag', 'Sonntag']) {
      await expect(page.locator('body')).toContainText(weekday);
    }
  });

  test('K3: Rezept mit planned_date wird auf der Seite angezeigt', async ({ page }) => {
    // Given: "Spaghetti Bolognese" mit planned_date in 2 Tagen (innerhalb der Woche falls möglich)
    const suffix = Date.now();
    const title = `Spaghetti ${suffix}`;
    // Verwende Mittwoch dieser Woche für stabiles Ergebnis
    const wednesday = currentWeekDateFromMonday(2);
    await createRecipeWithDate(page, title, ['Mittagessen'], wednesday);

    // When: Benutzer öffnet /wochenvorschau
    await page.goto('/wochenvorschau');

    // Then: Rezept erscheint auf der Seite
    await expect(page.locator('body')).toContainText(title);

    // And: Rezeptname ist als klickbarer Link vorhanden
    const recipeLink = page.locator(`a:has-text("${title}")`);
    await expect(recipeLink).toBeVisible();
  });

  test('K4: Mehrere Rezepte am gleichen Tag erscheinen beide', async ({ page }) => {
    // Given: Zwei Rezepte mit gleichem planned_date (Donnerstag dieser Woche)
    const suffix = Date.now();
    const title1 = `Pfannkuchen ${suffix}`;
    const title2 = `Rührei ${suffix}`;
    const thursday = currentWeekDateFromMonday(3);
    await createRecipeWithDate(page, title1, ['Mittagessen'], thursday);
    await createRecipeWithDate(page, title2, ['Mittagessen'], thursday);

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
    // (Auch wenn andere Tage Rezepte haben, müssen leere Tage korrekt angezeigt werden)
    // Hinweis: Die globale Leer-Meldung "Für diese Woche noch nichts geplant" erscheint nur
    // wenn KEIN Rezept in der gesamten Woche geplant ist — bereits durch Rust-Tests abgedeckt.
    await expect(page.locator('body')).toContainText('Nichts geplant');
  });

  test('K3/K6: Rezeptlink führt zur Detailansicht', async ({ page }) => {
    // Given: Rezept mit planned_date in der aktuellen Woche (Dienstag)
    const suffix = Date.now();
    const title = `Detaillink-Rezept ${suffix}`;
    const tuesday = currentWeekDateFromMonday(1);
    await createRecipeWithDate(page, title, ['Mittagessen'], tuesday);

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
    // Given: Ein Rezept mit planned_date in der aktuellen Woche (Freitag)
    const suffix = Date.now();
    const title = `Deeplink-Rezept ${suffix}`;
    const friday = currentWeekDateFromMonday(4);
    await createRecipeWithDate(page, title, ['Mittagessen'], friday);

    // When: Benutzer ruft /wochenvorschau direkt auf (Bookmark-Simulation)
    await page.goto('/wochenvorschau');

    // Then: HTTP 200, Wochenvorschau korrekt angezeigt, Rezept sichtbar
    await expect(page).toHaveURL(/\/wochenvorschau/);
    await expect(page.locator('h1')).toContainText('Wochenvorschau');
    await expect(page.locator('body')).toContainText(title);
  });

  test('K7: Link zurück zur Rezeptliste vorhanden', async ({ page }) => {
    // Given: /wochenvorschau wird aufgerufen
    await page.goto('/wochenvorschau');

    // Then: Link "Zur Rezeptliste" vorhanden
    const backLink = page.locator('a[href="/"]', { hasText: 'Zur Rezeptliste' });
    await expect(backLink).toBeVisible();

    // When: Link geklickt
    await backLink.click();

    // Then: Benutzer ist auf der Rezeptliste "/"
    await expect(page).toHaveURL(/\/$/);
  });

  test('K9: Semantisches HTML — Wochentage als dt-Elemente', async ({ page }) => {
    // Given: /wochenvorschau wird aufgerufen
    await page.goto('/wochenvorschau');

    // Then: Wochentag-Bezeichnungen sind in dt-Elementen
    const dtElements = page.locator('dt.wochentag-titel');
    await expect(dtElements).toHaveCount(7);

    // And: Rezept-Links haben aussagekräftige Labels (nicht nur "hier" o.ä.)
    // Prüfen: dl.wochenvorschau-liste vorhanden
    await expect(page.locator('dl.wochenvorschau-liste')).toBeVisible();
  });

  test('KW-Anzeige: Kalenderwochen-Angabe sichtbar', async ({ page }) => {
    // Given: /wochenvorschau wird aufgerufen
    await page.goto('/wochenvorschau');

    // Then: KW-Angabe sichtbar (z.B. "KW 14")
    await expect(page.locator('.kw-label')).toBeVisible();
    const kwText = await page.locator('.kw-label').textContent();
    expect(kwText).toMatch(/KW \d+/);
  });
});
