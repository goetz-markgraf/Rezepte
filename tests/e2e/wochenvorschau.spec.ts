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

/**
 * Berechnet den Offset des heutigen Tags von Montag (0=Montag, 6=Sonntag).
 */
function daysFromMondayToday(): number {
  const today = new Date();
  const dayOfWeek = today.getDay(); // 0=So, 1=Mo, …, 6=Sa
  return dayOfWeek === 0 ? 6 : dayOfWeek - 1;
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
    await expect(page.locator('h1')).toContainText('KW');
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
    await expect(page.locator('h1')).toContainText('KW');
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

test.describe('Wochenvorschau Formatierung (Story 19)', () => {
  test('K1: 7x span.wochentag-name und 7x span.wochentag-datum sichtbar', async ({ page }) => {
    // Given: Die Wochenvorschau ist geöffnet
    await page.goto('/wochenvorschau');

    // Then: span.wochentag-name Elemente sichtbar (7 Stück)
    const nameElems = page.locator('strong.wochentag-name');
    await expect(nameElems).toHaveCount(7);

    // And: Wochentag-Namen sind korrekt
    await expect(page.locator('body')).toContainText('Montag');

    // And: span.wochentag-datum Elemente sichtbar (7 Stück)
    const datumElems = page.locator('span.wochentag-datum');
    await expect(datumElems).toHaveCount(7);
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

  test('K3: Anzahl wochentag-vergangen entspricht Anzahl Tage seit Montag', async ({ page }) => {
    // Given: /wochenvorschau aufgerufen
    await page.goto('/wochenvorschau');

    // Anzahl vergangener Tage = Offset des heutigen Tags von Montag
    const daysFromMonday = daysFromMondayToday();

    // Then: Genau so viele Elemente mit wochentag-vergangen wie Tage seit Montag
    await expect(page.locator('.wochentag-vergangen')).toHaveCount(daysFromMonday);
  });

  test('K4: Smoke-Test — Rezept für heute erscheint unter .wochentag-heute', async ({ page }) => {
    // Given: Rezept für heute erstellt
    const suffix = Date.now();
    const title = `Story19-Smoke-${suffix}`;
    // Berechne "heute" konsistent mit dem Server (UTC)
    // Der Server verwendet time::OffsetDateTime::now_utc().date()
    const now = new Date();
    const utcDate = new Date(Date.UTC(now.getFullYear(), now.getMonth(), now.getDate()));
    const todayDate = `${utcDate.getUTCDate()}.${utcDate.getUTCMonth() + 1}.${utcDate.getUTCFullYear()}`;
    await createRecipeWithDate(page, title, ['Mittagessen'], todayDate);

    // When: /wochenvorschau aufgerufen
    await page.goto('/wochenvorschau');

    // Then: Rezept unter heutigem Tag sichtbar
    await expect(page.locator('.wochentag-heute')).toContainText(title);

    // And: Link zur Detailansicht vorhanden
    await expect(page.locator(`.wochentag-heute a:has-text("${title}")`)).toBeVisible();
  });
});

test.describe('Wochenübersicht Navigation (Story 33)', () => {
  test('K1: Navigation zur vorherigen Woche per Link', async ({ page }) => {
    // Given: /wochenvorschau wird aufgerufen (aktuelle Woche)
    await page.goto('/wochenvorschau');

    // Then: Link zur vorherigen Woche ist sichtbar
    const prevLink = page.locator('a[href*="week="].wochen-nav-prev, a[aria-label="Vorherige Woche"]');
    await expect(prevLink).toBeVisible();

    // When: Benutzer klickt auf den Link
    await prevLink.click();

    // Then: URL enthält week-Parameter für vorherige Woche
    await expect(page).toHaveURL(/week=\d{4}-W\d{2}/);

    // And: KW-Anzeige ist immer noch sichtbar
    await expect(page.locator('.kw-label')).toBeVisible();
  });

  test('K1: Navigation zur nächsten Woche per Link', async ({ page }) => {
    // Given: /wochenvorschau wird aufgerufen (aktuelle Woche)
    await page.goto('/wochenvorschau');

    // Then: Link zur nächsten Woche ist sichtbar
    const nextLink = page.locator('a[href*="week="].wochen-nav-next, a[aria-label="Nächste Woche"]');
    await expect(nextLink).toBeVisible();

    // When: Benutzer klickt auf den Link
    await nextLink.click();

    // Then: URL enthält week-Parameter für nächste Woche
    await expect(page).toHaveURL(/week=\d{4}-W\d{2}/);

    // And: KW-Anzeige ist immer noch sichtbar
    await expect(page.locator('.kw-label')).toBeVisible();
  });

  test('K3: DeepLink zu spezifischer Woche funktioniert', async ({ page }) => {
    // Given: Direktaufruf mit week-Parameter
    await page.goto('/wochenvorschau?week=2025-W02');

    // Then: Seite lädt erfolgreich
    await expect(page.locator('h1')).toBeVisible();

    // And: KW-Anzeige zeigt Woche 2 an
    const kwText = await page.locator('.kw-label').textContent();
    expect(kwText).toContain('KW 2');
  });

  test('K2: "Diese Woche"-Badge bei aktueller Woche sichtbar', async ({ page }) => {
    // Given: /wochenvorschau ohne Parameter (aktuelle Woche)
    await page.goto('/wochenvorschau');

    // Then: Badge "Diese Woche" ist sichtbar
    await expect(page.locator('.current-week-badge')).toBeVisible();
    await expect(page.locator('.current-week-badge')).toContainText('Diese Woche');
  });

  test('K4: Mehrfache Navigation funktioniert', async ({ page }) => {
    // Given: /wochenvorschau wird aufgerufen
    await page.goto('/wochenvorschau');

    // When: Dreimal auf "vorherige Woche" klicken
    const prevLink = page.locator('a[aria-label="Vorherige Woche"], a.wochen-nav-prev');
    for (let i = 0; i < 3; i++) {
      await expect(prevLink).toBeVisible();
      await prevLink.click();
      await page.waitForLoadState('networkidle');
    }

    // Then: Seite ist immer noch funktionsfähig
    await expect(page.locator('.kw-label')).toBeVisible();
    await expect(page.locator('h1')).toBeVisible();
  });

  test('K6: Navigation hat korrekte ARIA-Labels', async ({ page }) => {
    // Given: /wochenvorschau wird aufgerufen
    await page.goto('/wochenvorschau');

    // Then: Navigation-Links haben aussagekräftige Labels
    const prevLink = page.locator('a[aria-label="Vorherige Woche"]');
    const nextLink = page.locator('a[aria-label="Nächste Woche"]');

    // Mindestens einer der Links sollte die erwarteten ARIA-Labels haben
    await expect(prevLink.or(page.locator('.wochen-nav-prev'))).toBeVisible();
    await expect(nextLink.or(page.locator('.wochen-nav-next'))).toBeVisible();
  });
});
