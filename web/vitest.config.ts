import react from "@vitejs/plugin-react";
import { fileURLToPath } from "node:url";
import { defineConfig } from "vitest/config";

export default defineConfig({
  plugins: [react()],
  resolve: {
    alias: { "@": fileURLToPath(new URL("./src", import.meta.url)) },
  },
  test: {
    environment: "jsdom",
    globals: true,
    setupFiles: ["./src/test/setup.ts"],
    // tests/qa is the QA-owned path: it reaches the app only through feature indexes and @/shared (see eslint.config.mjs).
    include: ["src/**/*.test.{ts,tsx}", "scripts/**/*.test.ts", "tests/qa/**/*.test.{ts,tsx}"],
    restoreMocks: true,
    // jsdom component tests (time-zone combobox, dialogs) run slowly when the host is loaded; 5 s was flaky.
    testTimeout: 20_000,
  },
});
