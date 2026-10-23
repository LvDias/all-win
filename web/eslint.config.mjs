import { readdirSync } from "node:fs";
import { defineConfig, globalIgnores } from "eslint/config";
import nextVitals from "eslint-config-next/core-web-vitals";
import nextTs from "eslint-config-next/typescript";

// Feature boundaries (ADR-0002 §2): a feature imports only `shared/` and itself.
const features = readdirSync(new URL("./src/features", import.meta.url), { withFileTypes: true })
  .filter((entry) => entry.isDirectory())
  .map((entry) => entry.name);

const featureBoundaries = features.map((feature) => ({
  files: [`src/features/${feature}/**/*.{ts,tsx}`],
  rules: {
    "no-restricted-imports": [
      "error",
      {
        patterns: [
          {
            group: ["@/app/*", "**/tests/qa", "**/tests/qa/**", ...features.filter((other) => other !== feature).map((other) => `@/features/${other}`), ...features.filter((other) => other !== feature).map((other) => `@/features/${other}/*`)],
            message: "Features import only from @/shared and their own folder.",
          },
        ],
      },
    ],
  },
}));

export default defineConfig([
  ...nextVitals,
  ...nextTs,
  {
    rules: {
      "@typescript-eslint/no-explicit-any": "error",
      "@typescript-eslint/no-non-null-assertion": "error",
    },
  },
  {
    // No hard-coded UI text (root CLAUDE.md §6): JSX text must come from the catalogs.
    files: ["src/**/*.tsx"],
    ignores: ["src/**/*.test.tsx"],
    rules: {
      "react/jsx-no-literals": ["error", { noStrings: false, ignoreProps: true }],
    },
  },
  {
    files: ["src/shared/**/*.{ts,tsx}"],
    rules: {
      "no-restricted-imports": [
        "error",
        {
          patterns: [
            { group: ["@/features/*", "@/app/*"], message: "Shared code never imports features or routes." },
            { group: ["**/tests/qa", "**/tests/qa/**"], message: "Application code never imports tests/qa." },
          ],
        },
      ],
    },
  },
  ...featureBoundaries,
  {
    // QA-owned tests exercise a feature only through its public index, plus the shared layer.
    files: ["tests/qa/**/*.{ts,tsx}"],
    rules: {
      "no-restricted-imports": [
        "error",
        {
          patterns: [
            { group: ["@/app/*", "@/features/*/*"], message: "tests/qa imports a feature only through @/features/<feature> (its index) and otherwise @/shared." },
            { group: ["../*", "./*"], message: "tests/qa uses @/ aliases only (@/features/<feature>, @/shared/*, @/test/*)." },
          ],
        },
      ],
    },
  },
  {
    files: ["**/*.test.{ts,tsx}", "e2e/**/*.ts", "src/test/**/*.{ts,tsx}", "tests/qa/**/*.{ts,tsx}"],
    rules: {
      "@typescript-eslint/no-non-null-assertion": "off",
    },
  },
  {
    // Playwright fixtures name their callback `use`, which the hooks rule mistakes for a React hook.
    files: ["e2e/**/*.ts", "tests/qa/e2e/**/*.ts"],
    rules: { "react-hooks/rules-of-hooks": "off" },
  },
  globalIgnores([".next/**", "out/**", "build/**", "next-env.d.ts", "src/shared/api/schema.gen.ts", "playwright-report/**", "test-results/**"]),
]);
