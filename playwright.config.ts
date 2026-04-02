import { defineConfig, devices } from '@playwright/test';

export default defineConfig({
  testDir: './tests/e2e',
  fullyParallel: true,
  forbidOnly: !!process.env.CI,
  retries: process.env.CI ? 2 : 0,
  workers: 1, // Sequential execution for database isolation
  reporter: 'list',
  use: {
    baseURL: 'http://localhost:8081',
    trace: 'on-first-retry',
  },
  projects: [
    {
      name: 'chromium',
      use: { ...devices['Desktop Chrome'] },
    },
  ],
  webServer: {
    command: 'rm -f ./data/test.db && PORT=8081 TEST_DATABASE_URL=./data/test.db cargo run',
    url: 'http://localhost:8081/health',
    reuseExistingServer: false,
    stdout: 'pipe',
    stderr: 'pipe',
  },
});