# 2026-10-23 — V1-S4 Bilingual foundation and web scaffold

- **Merge:** fast-forward of `main` from `984a8aa` to `7d348f1` (branch `s/v1-s4-i18n-foundation--web`), local, no push, by DevOps, under the user's standing authorization. Integration check in a clean detached worktree at `7d348f1`: `pnpm install --frozen-lockfile` ok; `pnpm check` exit 0 (eslint, tsc, i18n parity, API types match contract, vitest 97/97).
- **Scope:** `web/**` — Next.js 16 App Router + TypeScript + Tailwind + shadcn/ui + next-intl + TanStack Query; routes `/pt-BR` and `/en`; locale resolution (cookie, Accept-Language, profile language), language switcher, Preferences page (language + IANA time zone), pt-BR/en catalogs with `errors.<code>` for contract codes, missing-key marker in dev/QA, exact decimal formatting/parsing, generated API types from contract 1.1.0 with drift check, security headers (nonce CSP, nosniff, Referrer-Policy, Permissions-Policy). Covers S4 ACs and X-6 (see slice-plan).
- **Gate verdicts (all on `7d348f1`, clean QA worktrees):**
  - QA [Frontend] (Lente): APPROVED — check (lint, typecheck, i18n parity with placeholders, API drift, vitest 97/97), build, Playwright e2e 24/24.
  - QA [Security] (Vigia): APPROVED — summary: 0 critical, 0 high, 0 medium, 0 low open; earlier 1 medium + 2 low + 1 info fixed; `pnpm audit --prod` clean.
  - Code Reviewer (Crivo): APPROVED — 10 findings from Code Review 2 fixed; merge precondition (types from contract on main) met.
- **Known limitations:** production missing-key fallback in pt-BR is blank (CI parity mitigates); no "Preference saved" toast on language change; no automated axe/WCAG check, manual keyboard/focus check pending; IANA city names not localized; HSTS still to be set at Caddy before any HTTPS exposure; dev-only `braces` advisory tracked.
- **Provisional decisions involved:** U8 security headers ownership (confirmed by the Architect in ADR-0001 §9); U9 browser floor (Chrome 106+, Safari 15.4+, Firefox 116+), awaiting user confirmation.
- **Accepted risks:** none.
- **Follow-ups:** S3 web on this base; HSTS at Caddy (DevOps); accessibility pass.
