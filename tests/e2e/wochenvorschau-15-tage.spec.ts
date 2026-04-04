import { test, expect } from '@playwright/test';

/**
 * E2E Tests für Story 38: Wochenplanung auf 15-Tage-Liste umbauen
 */

test.describe('Wochenvorschau 15-Tage-Liste', () => {
  test.beforeEach(async ({ page }) => {
    // Gegeben: Der Nutzer ist auf der Wochenplanung-Seite
    await page.goto('/wochenvorschau');
  });

  test('15-Tage-Liste wird korrekt angezeigt (ab heute)', async ({ page }) => {
    // When die Seite lädt
    // Then werden die nächsten 15 Tage ab dem aktuellen Datum angezeigt
    const heute = new Date();
    const tagElemente = page.locator('.wochentag-abschnitt');
    await expect(tagElemente).toHaveCount(15);

    // And der aktuelle Tag ist das erste Element der Liste
    const ersterTag = tagElemente.first();
    await expect(ersterTag).toHaveClass(/wochentag-heute/);

    // Überprüfe das Datum des ersten Elements
    const heuteFormatiert = heute.toLocaleDateString('de-DE', {
      day: '2-digit',
      month: '2-digit',
      year: 'numeric'
    });
    const ersterTagDatum = ersterTag.locator('.wochentag-datum');
    await expect(ersterTagDatum).toContainText(heuteFormatiert);
  });

  test('Geplante Rezepte werden unter korrektem Datum angezeigt', async ({ page }) => {
    // Given ein Rezept ist für morgen geplant (via Seed)
    // When der Nutzer die Wochenplanung öffnet
    // Then wird das Rezept unter dem morgigen Datum angezeigt
    const morgen = new Date();
    morgen.setDate(morgen.getDate() + 1);

    const tagElemente = page.locator('.wochentag-abschnitt');
    const zweiterTag = tagElemente.nth(1);

    // Prüfe ob Rezepte im zweiten Tag (Morgen) angezeigt werden
    const rezepte = zweiterTag.locator('.wochentag-rezepte li');
    const count = await rezepte.count();
    expect(count).toBeGreaterThan(0);
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
    // Given ein Rezept ist in der Liste sichtbar
    const rezeptLink = page.locator('.wochentag-rezepte a').first();

    // When der Nutzer auf das Rezept klickt
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

  test('Datumsformat ist "Fr, 04.04.2026"', async ({ page }) => {
    // Given der Nutzer ist auf der Wochenplanung-Seite
    // When die Seite geladen ist
    // Then werden die Daten im Format "Fr, 04.04.2026" angezeigt
    const datumsElemente = page.locator('.wochentag-datum');
    const ersterTagDatum = datumsElemente.first();
    const text = await ersterTagDatum.textContent();

    // Format prüfen: "Fr, 04.04.2026" oder ähnlich
    expect(text).toMatch(/\d{1,2}\.\d{1,2}\.\d{4}/);
  });
});
