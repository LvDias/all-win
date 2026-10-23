// All Win E2E (owner: QA). API-level suite against the composed stack; see README.md.
import { defineConfig } from '@playwright/test';
import { config } from './src/config.ts';

export default defineConfig({
  testDir: './specs',
  // One worker, files in name order: 10-ingest → 20-latency → 30-masking → 40-disruptive.
  // Later files revoke tokens, exhaust rate limits and stop services.
  workers: 1,
  fullyParallel: false,
  retries: 0,
  timeout: 10 * 60_000,
  expect: { timeout: 10_000 },
  reporter: [['list'], ['html', { open: 'never', outputFolder: 'playwright-report' }]],
  globalSetup: './src/global-setup.ts',
  globalTeardown: './src/global-teardown.ts',
  outputDir: 'test-results',
  metadata: { mode: config.mode, project: config.project },
});
