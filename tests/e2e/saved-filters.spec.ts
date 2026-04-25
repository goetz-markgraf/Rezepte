import { test, expect } from '@playwright/test';

/**
 * E2E-Tests für Story 13: Gespeicherte Filter
 *
 * Die Tests erstellen Rezepte und Filter direkt über das Formular für Isolation.
 */

async function createRecipe(
  page: import('@playwright/test').Page,
  title: string,
  categories: string[]
): Promise<void> {
  await page.goto('/recipes/new');
  await page.fill('input[name="title"]', title);
  for (const category of categories) {
    await page.check(`input[name="categories"][value="${category}"]`);
  }
  await page.click('button[type="submit"]');
  await expect(page).toHaveURL(/\/recipes\/\d+/);
}

test.describe('Gespeicherte Filter (Story 13)', () => {

  test('K1/K2: Filter speichern und aufrufen', async ({ page }) => {
    // Gegeben: Rezept "Vollkornbrot" in Kategorie "Brot"
    const suffix = `sf1-${Date.now()}-${Math.random().toString(36).slice(2, 7)}`;
    await createRecipe(page, `Vollkornbrot-${suffix}`, ['Brot']);

    // Wenn: Benutzer navigiert direkt zu Kategorie "Brot" (Vollseitenaufruf wegen HTMX-Partial-Update)
    await page.goto('/?kategorie=Brot&filter_collapsed=0');
    await expect(page).toHaveURL(/kategorie=Brot/);

    // Dann: Speichern-Formular sichtbar (Filter aktiv)
    await expect(page.locator('.save-filter-form')).toBeVisible();

    // Wenn: Benutzer gibt "Brot-Ideen" als Filtername ein und speichert
    const filterName = `Brot-Ideen-${suffix}`;
    await page.fill('#save-filter-name', filterName);
    await page.click('.save-filter-submit');
    await page.waitForURL(/kategorie=Brot/);

    // Panel aufklappen (Toggle ist ein einfacher Link → vollständige Seitennavigation)
    await page.locator('.filter-toggle-btn').click();
    await page.waitForURL(/filter_collapsed=0/);

    // Dann: "Brot-Ideen" erscheint als Chip in der gespeicherten Filter-Liste
    await expect(page.locator('.saved-filter-btn', { hasText: filterName })).toBeVisible();

    // Wenn: Benutzer setzt alle Filter zurück
    await page.goto('/');

    // Panel aufklappen
    await page.locator('.filter-toggle-btn').click();
    await page.waitForURL(/filter_collapsed=0/);

    // Dann: Klick auf gespeicherten Filter "Brot-Ideen"
    await page.locator('.saved-filter-btn', { hasText: filterName }).click();

    // Dann: URL enthält "kategorie=Brot", nur Brot-Rezepte sichtbar
    await expect(page).toHaveURL(/kategorie=Brot/);
    await expect(page.locator('.recipe-item', { hasText: `Vollkornbrot-${suffix}` })).toBeVisible();
  });

  test('K4: Filter ist persistent nach Reload', async ({ page }) => {
    // Gegeben: Filter wird angelegt
    const suffix = `sf2-${Date.now()}-${Math.random().toString(36).slice(2, 7)}`;
    await createRecipe(page, `Brot-Persistenz-${suffix}`, ['Brot']);

    // Wenn: Kategorie "Brot" aktiv, Filter speichern
    await page.goto('/?kategorie=Brot&filter_collapsed=0');
    const filterName = `Persistenz-${suffix}`;
    await page.fill('#save-filter-name', filterName);
    await page.click('.save-filter-submit');
    await page.waitForURL(/kategorie=Brot/);

    // Panel aufklappen (Toggle ist ein einfacher Link → vollständige Seitennavigation)
    await page.locator('.filter-toggle-btn').click();
    await page.waitForURL(/filter_collapsed=0/);

    // Dann: Filter gespeichert und sichtbar
    await expect(page.locator('.saved-filter-btn', { hasText: filterName })).toBeVisible();

    // Wenn: Seite neu laden (Reload behält URL-Parameter bei)
    await page.reload();

    // Dann: Filter "Persistenz-..." immer noch sichtbar
    await expect(page.locator('.saved-filter-btn', { hasText: filterName })).toBeVisible();
  });

  test('K3: Filter löschen', async ({ page }) => {
    // Gegeben: Filter wird angelegt
    const suffix = `sf3-${Date.now()}-${Math.random().toString(36).slice(2, 7)}`;
    await createRecipe(page, `Brot-Loeschen-${suffix}`, ['Brot']);

    // Wenn: Kategorie "Brot" aktiv, Filter speichern
    await page.goto('/?kategorie=Brot&filter_collapsed=0');
    const filterName = `Loeschen-${suffix}`;
    await page.fill('#save-filter-name', filterName);
    await page.click('.save-filter-submit');
    await page.waitForURL(/kategorie=Brot/);

    // Panel aufklappen (Toggle ist ein einfacher Link → vollständige Seitennavigation)
    await page.locator('.filter-toggle-btn').click();
    await page.waitForURL(/filter_collapsed=0/);

    // Dann: Filter sichtbar
    await expect(page.locator('.saved-filter-btn', { hasText: filterName })).toBeVisible();

    // Wenn: Klick auf Löschen-Button neben dem Filter
    const filterItem = page.locator('.saved-filter-item', { hasText: filterName });
    await filterItem.locator('.saved-filter-delete-btn').click();

    // Dann: Filter verschwindet (HTMX-Delete)
    await expect(page.locator('.saved-filter-btn', { hasText: filterName })).not.toBeVisible();

    // Und: Nach Reload immer noch weg (URL hat bereits filter_collapsed=0 → Panel bleibt offen)
    await page.reload();
    await expect(page.locator('.saved-filter-btn', { hasText: filterName })).not.toBeVisible();
  });

  test('Kombinierten Filter speichern und aufrufen', async ({ page }) => {
    // Gegeben: Rezept mit Kategorie "Mittagessen"
    const suffix = `sf4-${Date.now()}-${Math.random().toString(36).slice(2, 7)}`;
    await createRecipe(page, `Spaghetti-${suffix}`, ['Mittagessen']);
    await createRecipe(page, `Roggenbrot-${suffix}`, ['Brot']);

    // Wenn: Kategorie "Mittagessen" und "Länger nicht gemacht" Filter aktiv
    await page.goto('/?kategorie=Mittagessen&filter=laenger-nicht-gemacht&filter_collapsed=0');
    await expect(page).toHaveURL(/kategorie=Mittagessen/);
    await expect(page).toHaveURL(/filter=laenger-nicht-gemacht/);

    // Wenn: Filter speichern als "Mittagessenplanung-..."
    const filterName = `Mittagessenplanung-${suffix}`;
    await page.fill('#save-filter-name', filterName);
    await page.click('.save-filter-submit');
    await page.waitForURL(/kategorie=Mittagessen/);

    // Panel aufklappen (Toggle ist ein einfacher Link → vollständige Seitennavigation)
    await page.locator('.filter-toggle-btn').click();
    await page.waitForURL(/filter_collapsed=0/);

    // Dann: Filter sichtbar
    await expect(page.locator('.saved-filter-btn', { hasText: filterName })).toBeVisible();

    // Wenn: Alle Filter zurücksetzen
    await page.goto('/');

    // Panel aufklappen
    await page.locator('.filter-toggle-btn').click();
    await page.waitForURL(/filter_collapsed=0/);

    // Dann: Klick auf gespeicherten Filter
    await page.locator('.saved-filter-btn', { hasText: filterName }).click();

    // Dann: URL enthält beide Parameter
    await expect(page).toHaveURL(/kategorie=Mittagessen/);
    await expect(page).toHaveURL(/filter=laenger-nicht-gemacht/);

    // Und: Spaghetti sichtbar, Roggenbrot nicht
    await expect(page.locator('.recipe-item', { hasText: `Spaghetti-${suffix}` })).toBeVisible();
    await expect(page.locator('.recipe-item', { hasText: `Roggenbrot-${suffix}` })).not.toBeVisible();
  });

  test('K5: Doppelter Name zeigt Fehlermeldung', async ({ page }) => {
    // Gegeben: "Brot-Ideen" bereits gespeichert
    const suffix = `sf5-${Date.now()}-${Math.random().toString(36).slice(2, 7)}`;
    await createRecipe(page, `Brot-Duplikat-${suffix}`, ['Brot']);

    await page.goto('/?kategorie=Brot&filter_collapsed=0');
    const filterName = `Duplikat-${suffix}`;
    await page.fill('#save-filter-name', filterName);
    await page.click('.save-filter-submit');
    await page.waitForURL(/kategorie=Brot/);

    // Panel aufklappen (Toggle ist ein einfacher Link → vollständige Seitennavigation)
    await page.locator('.filter-toggle-btn').click();
    await page.waitForURL(/filter_collapsed=0/);

    // Filter gespeichert
    await expect(page.locator('.saved-filter-btn', { hasText: filterName })).toBeVisible();

    // Wenn: Erneut Filter speichern unter demselben Namen
    await page.fill('#save-filter-name', filterName);
    await page.click('.save-filter-submit');
    await page.waitForURL(/save_error=duplikat/);

    // Panel aufklappen: Toggle-Button bewahrt save_error nicht → URL manuell mit filter_collapsed=0 laden
    const errorPageUrl = page.url();
    await page.goto(errorPageUrl + '&filter_collapsed=0');

    // Dann: Fehlermeldung "existiert bereits" sichtbar
    await expect(page.locator('.save-filter-error')).toBeVisible();
    await expect(page.locator('.save-filter-error')).toContainText('existiert bereits');

    // Und: Ursprünglicher Filter weiterhin in Liste
    await expect(page.locator('.saved-filter-btn', { hasText: filterName })).toBeVisible();
  });

  test('K6: Keine Treffer beim Aufrufen — Filter bleibt erhalten', async ({ page }) => {
    // Gegeben: Filter "Naechste-7-Tage" für Datumsfilter gespeichert
    const suffix = `sf6-${Date.now()}-${Math.random().toString(36).slice(2, 7)}`;

    // Wenn: Benutzer aktiviert den "Nächste 7 Tage" Filter
    await page.goto('/?filter=naechste-7-tage&filter_collapsed=0');
    await expect(page).toHaveURL(/filter=naechste-7-tage/);

    // Filter speichern
    const filterName = `Naechste7Tage-${suffix}`;
    await page.fill('#save-filter-name', filterName);
    await page.click('.save-filter-submit');
    await page.waitForURL(/filter=naechste-7-tage/);

    // Panel aufklappen (Toggle ist ein einfacher Link → vollständige Seitennavigation)
    await page.locator('.filter-toggle-btn').click();
    await page.waitForURL(/filter_collapsed=0/);

    // Dann: Filter sichtbar
    await expect(page.locator('.saved-filter-btn', { hasText: filterName })).toBeVisible();

    // Wenn: Alle Filter zurücksetzen, dann auf gespeicherten Filter klicken
    await page.goto('/');

    // Panel aufklappen
    await page.locator('.filter-toggle-btn').click();
    await page.waitForURL(/filter_collapsed=0/);

    await page.locator('.saved-filter-btn', { hasText: filterName }).click();

    // Dann: URL enthält naechste-7-tage (keine Rezepte → Keine-Treffer-Meldung erscheint oder nicht — Filter bleibt)
    await expect(page).toHaveURL(/filter=naechste-7-tage/);

    // Panel aufklappen um gespeicherte Filter sichtbar zu machen
    await page.locator('.filter-toggle-btn').click();
    await page.waitForURL(/filter_collapsed=0/);

    // Und: Gespeicherter Filter bleibt in der Liste erhalten
    await expect(page.locator('.saved-filter-btn', { hasText: filterName })).toBeVisible();
  });

  test('K7: Kein Speichern-Button ohne aktiven Filter', async ({ page }) => {
    // Gegeben: Startseite ohne aktive Filter
    await page.goto('/');

    // Dann: Speichern-Formular nicht sichtbar
    await expect(page.locator('.save-filter-form')).not.toBeVisible();

    // Wenn: Kategorie "Brot" aktiviert (Panel aufgeklappt für sichtbares Speichern-Formular)
    await page.goto('/?kategorie=Brot&filter_collapsed=0');

    // Dann: Speichern-Formular sichtbar
    await expect(page.locator('.save-filter-form')).toBeVisible();
  });

});
