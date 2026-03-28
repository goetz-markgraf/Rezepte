import { test, expect } from '@playwright/test';

test('health check returns OK', async ({ page }) => {
  // Given: Die App läuft
  // When: /health aufgerufen wird
  await page.goto('/health');

  // Then: Body enthält "OK"
  await expect(page.locator('body')).toContainText('OK');
});