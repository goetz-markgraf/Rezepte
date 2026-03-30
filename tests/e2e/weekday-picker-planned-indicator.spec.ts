import { test, expect } from '@playwright/test';

// Helper: Datum in deutschem Format T.M.JJJJ
function getGermanDate(daysOffset: number): string {
    const date = new Date();
    date.setDate(date.getDate() + daysOffset);
    const day = date.getDate();
    const month = date.getMonth() + 1;
    const year = date.getFullYear();
    return `${day}.${month}.${year}`;
}

// Helper: Datum-String für SQL INSERT (YYYY-MM-DD)
function getSqlDate(daysOffset: number): string {
    const date = new Date();
    date.setDate(date.getDate() + daysOffset);
    const day = String(date.getDate()).padStart(2, '0');
    const month = String(date.getMonth() + 1).padStart(2, '0');
    const year = date.getFullYear();
    return `${year}-${month}-${day}`;
}

test.describe.serial('Wochenpicker geplantes Essen Indikator', () => {
    test.beforeEach(async ({ request }) => {
        // Alle Rezepte vor jedem Test löschen
        await request.post('/api/test/clear-recipes');
    });

    test('Indikator wird für geplante Tage angezeigt', async ({ page, request }) => {
        // Given: Ein Rezept für morgen in der Datenbank
        const tomorrow = getSqlDate(1);
        await request.post('/api/test/clear-recipes');
        await request.post('/api/test/seed-recipe', {
            data: {
                title: 'Pasta Carbonara',
                categories: ['Mittagessen'],
                planned_date: tomorrow
            }
        });

        // When: Das Formular wird neu geladen
        await page.goto('/recipes/new');

        // Then: Ein Indikator (Stern) ist auf dem Button für morgen sichtbar
        const weekdayButtons = page.locator('.weekday-btn');
        const firstButton = weekdayButtons.first();

        // Warte auf das Widget
        await expect(firstButton).toBeVisible();

        // Der erste Button sollte einen Indikator haben (morgen)
        const indicator = firstButton.locator('.planned-indicator');
        await expect(indicator).toBeVisible();
        await expect(indicator.locator('.indicator-icon')).toHaveText('★');
    });

    test('Kein Indikator für ungeplante Tage', async ({ page, request }) => {
        // Given: Keine Rezepte in der Datenbank
        await request.post('/api/test/clear-recipes');

        // When: Das Formular wird geladen
        await page.goto('/recipes/new');

        // Then: Keine Indikatoren auf den Buttons
        const indicators = page.locator('.planned-indicator');
        await expect(indicators).toHaveCount(0);
    });

    test('Tooltip zeigt Rezeptname beim Hover', async ({ page, request }) => {
        // Given: Ein Rezept für morgen
        const tomorrow = getSqlDate(1);
        await request.post('/api/test/clear-recipes');
        await request.post('/api/test/seed-recipe', {
            data: {
                title: 'Pasta Carbonara',
                categories: ['Mittagessen'],
                planned_date: tomorrow
            }
        });

        await page.goto('/recipes/new');

        // When: Über den Indikator auf dem ersten Button hovern
        const firstButton = page.locator('.weekday-btn').first();
        const indicator = firstButton.locator('.planned-indicator');

        await indicator.hover();

        // Then: Der Tooltip zeigt den Rezeptnamen
        const tooltip = indicator.locator('.indicator-tooltip');
        await expect(tooltip).toBeVisible();
        await expect(tooltip).toHaveText('Pasta Carbonara');
    });

    test('Klick auf Indikator navigiert zur Detailseite', async ({ page, request }) => {
        // Given: Ein Rezept für morgen
        const tomorrow = getSqlDate(1);
        await request.post('/api/test/clear-recipes');
        const response = await request.post('/api/test/seed-recipe', {
            data: {
                title: 'Spaghetti Bolognese',
                categories: ['Mittagessen'],
                planned_date: tomorrow
            }
        });
        const recipeId = await response.text();

        await page.goto('/recipes/new');

        // When: Auf den Indikator klicken
        const firstButton = page.locator('.weekday-btn').first();
        const indicator = firstButton.locator('.planned-indicator');

        await indicator.click();

        // Then: Navigation zur Detailseite
        await expect(page).toHaveURL(`/recipes/${recipeId}`);
        await expect(page.locator('h1')).toContainText('Spaghetti Bolognese');
    });

    test('Tastatur-Navigation funktioniert mit Indikator', async ({ page, request }) => {
        // Given: Ein Rezept für morgen
        const tomorrow = getSqlDate(1);
        await request.post('/api/test/clear-recipes');
        const response = await request.post('/api/test/seed-recipe', {
            data: {
                title: 'Schnitzel',
                categories: ['Mittagessen'],
                planned_date: tomorrow
            }
        });
        const recipeId = await response.text();

        await page.goto('/recipes/new');

        // When: Den Indikator mit Tab fokussieren und Enter drücken
        const firstButton = page.locator('.weekday-btn').first();
        const indicator = firstButton.locator('.planned-indicator');

        await indicator.focus();

        // Then: Der Indikator ist fokussiert (outline sollte sichtbar sein)
        await expect(indicator).toBeFocused();

        // When: Enter drücken
        await indicator.press('Enter');

        // Then: Navigation zur Detailseite
        await expect(page).toHaveURL(`/recipes/${recipeId}`);
    });

    test('Mehrere Rezepte an einem Tag - nur erstes wird angezeigt', async ({ page, request }) => {
        // Given: Zwei Rezepte für morgen
        const tomorrow = getSqlDate(1);
        await request.post('/api/test/clear-recipes');
        await request.post('/api/test/seed-recipe', {
            data: {
                title: 'Erstes Rezept',
                categories: ['Mittagessen'],
                planned_date: tomorrow
            }
        });
        await request.post('/api/test/seed-recipe', {
            data: {
                title: 'Zweites Rezept',
                categories: ['Kuchen'],
                planned_date: tomorrow
            }
        });

        await page.goto('/recipes/new');

        // Then: Nur ein Indikator auf dem ersten Button
        const firstButton = page.locator('.weekday-btn').first();
        const indicators = firstButton.locator('.planned-indicator');
        await expect(indicators).toHaveCount(1);

        // Und der Tooltip zeigt das erste Rezept
        await indicators.first().hover();
        await expect(indicators.first().locator('.indicator-tooltip')).toHaveText('Erstes Rezept');
    });

    test('aria-label enthält Rezeptname für Screenreader', async ({ page, request }) => {
        // Given: Ein Rezept für morgen
        const tomorrow = getSqlDate(1);
        await request.post('/api/test/clear-recipes');
        await request.post('/api/test/seed-recipe', {
            data: {
                title: 'Pizza Margherita',
                categories: ['Mittagessen'],
                planned_date: tomorrow
            }
        });

        await page.goto('/recipes/new');

        // Then: Der Indikator hat ein aussagekräftiges aria-label
        const firstButton = page.locator('.weekday-btn').first();
        const indicator = firstButton.locator('.planned-indicator');

        await expect(indicator).toHaveAttribute('aria-label', 'Geplantes Essen: Pizza Margherita');
    });

    test('Indikator ist auch auf der Edit-Seite sichtbar', async ({ page, request }) => {
        // Given: Ein Rezept für morgen
        const tomorrow = getSqlDate(1);
        await request.post('/api/test/clear-recipes');
        const response = await request.post('/api/test/seed-recipe', {
            data: {
                title: 'Caesar Salad',
                categories: ['Mittagessen'],
                planned_date: tomorrow
            }
        });
        const recipeId = await response.text();

        // When: Die Edit-Seite eines anderen Rezepts öffnen
        await page.goto(`/recipes/${recipeId}/edit`);

        // Then: Der Indikator ist auf dem Wochenpicker sichtbar
        const firstButton = page.locator('.weekday-btn').first();
        const indicator = firstButton.locator('.planned-indicator');
        await expect(indicator).toBeVisible();
    });
});
