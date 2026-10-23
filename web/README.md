# All Win — web

Next.js 16 + TypeScript + shadcn/ui. Business rules and authorization live in the Core; this app renders and calls `/api/v1`.

## Run

```bash
pnpm install
ALLWIN_CORE_URL=http://localhost:8080 pnpm dev   # forwards /api to the Core; omit to work with mocked/no API
```

Open `http://localhost:3000`: it redirects to `/pt-BR` or `/en` from `Accept-Language`.
Without a Core, `/api/v1/me` fails and pages behave as signed out.

## Checks

| Command | What |
|---|---|
| `pnpm lint` / `pnpm typecheck` | ESLint (feature boundaries, no `any`, no hard-coded JSX text) / `tsc` |
| `pnpm i18n:check` | key parity `messages/pt-BR.json` = `messages/en.json`; empty strings count as missing |
| `pnpm test` | Vitest (unit + component tests with MSW) |
| `pnpm test:e2e` | Playwright against a production build with `NEXT_PUBLIC_I18N_MISSING=marker` (fails on `⟦missing:` markers) |
| `pnpm gen:api` | regenerates `src/shared/api/schema.gen.ts` from `contracts/http/openapi.bundled.yaml` (`ALLWIN_OPENAPI=<path>` to override; `pnpm gen:api:check` fails on drift) |

First run of e2e: `pnpm exec playwright install chromium` (the host also needs Chromium's system libraries).

## Layout

`src/app` routes · `src/features/<feature>` feature modules (import only `shared` and themselves) · `src/shared` ui, i18n, format, api, session · `messages/` catalogs.

## Sign-in model

Passwordless: a one-time code by e-mail (`CodeChallenge`, `shared/components`). Sensitive actions run through `useStepUp().run({ action, targetId, description }, send)`: `send` is called without proof first; on `auth.step_up_required` the dialog asks a new code for that action and `send(proof)` is called once with `{ challenge_id, code }`. Codes, challenge ids, view-once URLs and pairing codes live only in component state.
