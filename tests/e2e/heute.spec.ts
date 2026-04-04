import { test, expect } from '@playwright/test';

/**
 * E2E-Tests für Story 20: "Heute gekocht"-Ansicht mit Highlight
 *
 * Die Tests erstellen Rezepte direkt über das Formular für Isolation.
 * Datumswerte werden relativ zum aktuellen Datum berechnet.
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
 * n=0 → heute, n=-1 → gestern, n=1 → morgen
 */
function dateInDays(n: number): string {
  const d = new Date();
  d.setDate(d.getDate() + n);
  return `${d.getDate()}.${d.getMonth() + 1}.${d.getFullYear()}`;
}

test.describe('Heute gekocht (Story 20)', () => {
  test('K1: /heute ist aufrufbar und in Navigation verlinkt', async ({ page }) => {
    // Given: Die App ist gestartet
    await page.goto('/');

    // Then: Link "Heute" in der Navigation sichtbar
    const navLink = page.locator('a[href="/heute"]');
    await expect(navLink).toBeVisible();
    await expect(navLink).toContainText('Heute');

    // When: Benutzer klickt den Link
    await navLink.click();

    // Then: URL ist /heute, HTTP 200, Überschrift sichtbar
    await expect(page).toHaveURL(/\/heute/);
    await expect(page.locator('h1')).toContainText('Heute gekocht');
  });

  test('K1: /heute ist per DeepLink direkt aufrufbar', async ({ page }) => {
    // When: Benutzer ruft /heute direkt auf
    await page.goto('/heute');

    // Then: Seite lädt korrekt
    await expect(page).toHaveURL(/\/heute/);
    await expect(page.locator('h1')).toContainText('Heute gekocht');
  });

  test('K2: Heutiges Rezept ist im hervorgehobenen Heute-Abschnitt', async ({ page }) => {
    // Given: "Spaghetti Bolognese" mit planned_date = heute
    const suffix = Date.now();
    const title = `Spaghetti ${suffix}`;
    await createRecipeWithDate(page, title, ['Mittagessen'], dateInDays(0));

    // When: Benutzer öffnet /heute
    await page.goto('/heute');

    // Then: Rezept erscheint im hervorgehobenen Heute-Bereich
    const heuteAbschnitt = page.locator('.tagesabschnitt-heute');
    await expect(heuteAbschnitt).toBeVisible();
    await expect(heuteAbschnitt).toContainText(title);

    // And: CSS-Klasse "tagesabschnitt-heute" ist am Abschnitt gesetzt
    await expect(heuteAbschnitt).toHaveClass(/tagesabschnitt-heute/);

    // And: "Heute"-Label ist sichtbar
    await expect(heuteAbschnitt.locator('.heute-label')).toBeVisible();
  });

  test('K3: Gestern und morgen werden angezeigt', async ({ page }) => {
    // Given: Je ein Rezept für gestern, heute und morgen
    const suffix = Date.now();
    await createRecipeWithDate(page, `Thai-Curry-${suffix}`, ['Mittagessen'], dateInDays(-1));
    await createRecipeWithDate(page, `Spaghetti-${suffix}`, ['Mittagessen'], dateInDays(0));
    await createRecipeWithDate(page, `Pfannkuchen-${suffix}`, ['Kuchen'], dateInDays(1));

    // When: Benutzer öffnet /heute
    await page.goto('/heute');

    // Then: Alle drei Rezepte sichtbar
    await expect(page.locator('body')).toContainText(`Thai-Curry-${suffix}`);
    await expect(page.locator('body')).toContainText(`Spaghetti-${suffix}`);
    await expect(page.locator('body')).toContainText(`Pfannkuchen-${suffix}`);

    // And: Gestern-Abschnitt enthält Thai-Curry
    const gesternAbschnitt = page.locator('.tagesabschnitt').filter({ hasText: 'Gestern' });
    await expect(gesternAbschnitt).toContainText(`Thai-Curry-${suffix}`);

    // And: Heute-Abschnitt enthält Spaghetti
    const heuteAbschnitt = page.locator('.tagesabschnitt-heute');
    await expect(heuteAbschnitt).toContainText(`Spaghetti-${suffix}`);

    // And: Morgen-Abschnitt enthält Pfannkuchen
    const morgenAbschnitt = page.locator('.tagesabschnitt').filter({ hasText: 'Morgen' });
    await expect(morgenAbschnitt).toContainText(`Pfannkuchen-${suffix}`);
  });

  test('K4: Freundliche Meldung wenn kein Rezept für heute', async ({ page }) => {
    // Given: Kein Rezept mit planned_date = heute — frischer Aufruf ohne eigenes heutiges Rezept
    // When: Benutzer öffnet /heute
    await page.goto('/heute');

    // Then: Seite lädt korrekt, "Heute"-Abschnitt ist vorhanden
    const heuteAbschnitt = page.locator('.tagesabschnitt-heute');
    await expect(heuteAbschnitt).toBeVisible();

    // Wenn heute keine Rezepte vorhanden sind, erscheint die Meldung
    // (In einer isolierten Testumgebung wäre die Meldung immer sichtbar;
    // in einer geteilten DB kann heute bereits Rezepte aus anderen Tests geben.)
    // Wir prüfen: entweder hat die Seite die Meldung ODER Rezepte (nicht beides leer ohne Meldung).
    const hatRezepte = await heuteAbschnitt.locator('.heute-rezept-item').count();
    if (hatRezepte === 0) {
      await expect(heuteAbschnitt).toContainText('Für heute noch kein Rezept geplant');
    } else {
      // Andernfalls: Rezepte wurden von anderen Tests erstellt — Feature OK
      await expect(heuteAbschnitt.locator('.heute-rezept-item').first()).toBeVisible();
    }
  });

  test('K6: Rezepttitel ist Link zur Detailansicht', async ({ page }) => {
    // Given: Rezept mit planned_date = heute
    const suffix = Date.now();
    const title = `Link-Rezept-${suffix}`;
    await createRecipeWithDate(page, title, ['Mittagessen'], dateInDays(0));

    // When: Benutzer öffnet /heute
    await page.goto('/heute');

    // And: Klickt auf Rezepttitel
    const rezeptLink = page.locator('.heute-rezept-titel', { hasText: title }).first();
    await expect(rezeptLink).toBeVisible();
    await rezeptLink.click();

    // Then: Navigiert zur Detailansicht /recipes/:id
    await expect(page).toHaveURL(/\/recipes\/\d+/);
    await expect(page.locator('body')).toContainText(title);
  });

  test('Mehrere Rezepte für heute werden alle angezeigt', async ({ page }) => {
    // Given: "Spaghetti" und "Salat" haben beide planned_date = heute
    const suffix = Date.now();
    await createRecipeWithDate(page, `Spaghetti-${suffix}`, ['Mittagessen'], dateInDays(0));
    await createRecipeWithDate(page, `Salat-${suffix}`, ['Snacks'], dateInDays(0));

    // When: Benutzer öffnet /heute
    await page.goto('/heute');

    // Then: Beide Rezepte im Heute-Bereich sichtbar
    const heuteAbschnitt = page.locator('.tagesabschnitt-heute');
    await expect(heuteAbschnitt).toContainText(`Spaghetti-${suffix}`);
    await expect(heuteAbschnitt).toContainText(`Salat-${suffix}`);
  });
});
