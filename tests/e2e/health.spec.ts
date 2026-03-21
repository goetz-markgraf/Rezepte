import { test, expect } from '@playwright/test';

test('health check returns OK', async ({ page }) => {
  await page.goto('/health');
  await expect(page.locator('body')).toContainText('OK');
});