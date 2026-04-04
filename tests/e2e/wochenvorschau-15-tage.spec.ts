import { test, expect } from '@playwright/test';

/**
 * E2E Tests für Story 38: Wochenplanung auf 15-Tage-Liste umbauen
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

test.describe('Wochenvorschau 15-Tage-Liste', () => {
  test.beforeEach(async ({ page }) => {
    // Gegeben: Der Nutzer ist auf der Wochenplanung-Seite
    await page.goto('/wochenvorschau');
  });

  test('15-Tage-Liste wird korrekt angezeigt (ab heute)', async ({ page }) => {
    // When die Seite lädt
    // Then werden die nächsten 15 Tage ab dem aktuellen Datum angezeigt
    const tagElemente = page.locator('.wochentag-abschnitt');
    await expect(tagElemente).toHaveCount(15);

    // And der aktuelle Tag ist das erste Element der Liste
    const ersterTag = tagElemente.first();
    await expect(ersterTag).toHaveClass(/wochentag-heute/);
  });

  test('Geplante Rezepte werden unter korrektem Datum angezeigt', async ({ page }) => {
    // Given: Ein Rezept für morgen
    const title = `Morgen-Rezept-${Date.now()}`;
    const tomorrow = futureDateInDays(1);
    await createRecipeWithDate(page, title, ['Mittagessen'], tomorrow);

    // When der Nutzer die Wochenplanung öffnet
    await page.goto('/wochenvorschau');

    // Then wird das Rezept unter dem morgigen Datum angezeigt
    const tagElemente = page.locator('.wochentag-abschnitt');
    const zweiterTag = tagElemente.nth(1);

    // Prüfe ob Rezept im zweiten Tag (Morgen) angezeigt wird
    await expect(zweiterTag).toContainText(title);
  });

  test('Navigation (Vorherige/Nächste Woche) ist nicht vorhanden', async ({ page }) => {
    // Given der Nutzer ist auf der Wochenplanung-Seite
    // When die Seite vollständig geladen ist
    // Then sind keine Buttons "Vorherige Woche" oder "Nächste Woche" sichtbar
    const navButtons = page.locator('.wochen-nav-btn');
    await expect(navButtons).toHaveCount(0);

    // Auch keine Links mit den entsprechenden aria-labels
    const prevWeekLink = page.locator('[aria-label="Vorherige Woche"]');
    await expect(prevWeekLink).toHaveCount(0);

    const nextWeekLink = page.locator('[aria-label="Nächste Woche"]');
    await expect(nextWeekLink).toHaveCount(0);
  });

  test('Tage ohne geplante Rezepte werden trotzdem angezeigt', async ({ page }) => {
    // Given es gibt keine geplanten Rezepte
    // When der Nutzer die Wochenplanung öffnet
    // Then werden trotzdem alle 15 Tage mit ihren Daten angezeigt
    const tagElemente = page.locator('.wochentag-abschnitt');
    await expect(tagElemente).toHaveCount(15);

    // Jeder Tag sollte ein Datum haben
    for (let i = 0; i < 15; i++) {
      const tag = tagElemente.nth(i);
      const datum = tag.locator('.wochentag-datum');
      await expect(datum).toBeVisible();
      await expect(datum).not.toBeEmpty();
    }
  });

  test('Klick auf Rezept führt zur Detailansicht', async ({ page }) => {
    // Given: Ein Rezept für heute
    const title = `Heute-Link-${Date.now()}`;
    const today = futureDateInDays(0);
    await createRecipeWithDate(page, title, ['Mittagessen'], today);

    // When: Nutzer öffnet Wochenvorschau und klickt auf Rezept
    await page.goto('/wochenvorschau');
    const rezeptLink = page.locator('.wochentag-rezepte a').first();
    await rezeptLink.click();

    // Then wird die Detailansicht des Rezepts angezeigt
    await expect(page).toHaveURL(/\/recipes\/\d+/);
    await expect(page.locator('h1')).toBeVisible();
  });

  test('Heute-Badge ist sichtbar', async ({ page }) => {
    // Given der Nutzer ist auf der Wochenplanung-Seite
    // When die Seite geladen ist
    // Then ist das Heute-Badge beim aktuellen Tag sichtbar
    const heuteBadge = page.locator('.heute-badge');
    await expect(heuteBadge).toBeVisible();
    await expect(heuteBadge).toHaveText('Heute');
  });

  test('Datumsformat enthält korrekte Datumsangabe', async ({ page }) => {
    // Given der Nutzer ist auf der Wochenplanung-Seite
    // When die Seite geladen ist
    // Then werden die Daten im Format "T. Monat" angezeigt
    const datumsElemente = page.locator('.wochentag-datum');
    const ersterTagDatum = datumsElemente.first();
    const text = await ersterTagDatum.textContent();

    // Format prüfen: "4. April" oder ähnlich
    expect(text).toMatch(/\d{1,2}\.\s*\w+/);
  });

  test('Wochentag-Name enthält kurzes Format', async ({ page }) => {
    // Given der Nutzer ist auf der Wochenplanung-Seite
    // When die Seite geladen ist
    // Then werden die Wochentage im kurzen Format angezeigt
    const nameElemente = page.locator('.wochentag-name');
    const ersterTagName = nameElemente.first();
    const text = await ersterTagName.textContent();

    // Format prüfen: "Sa, 04.04.2026" oder ähnlich
    expect(text).toMatch(/(Mo|Di|Mi|Do|Fr|Sa|So),\s*\d{2}\.\d{2}\.\d{4}/);
  });
});
