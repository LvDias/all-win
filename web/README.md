# All Win — web

Next.js 16 + TypeScript + shadcn/ui. Business rules and authorization live in the Core; this app renders and calls `/api/v1`.

## Run

```bash
pnpm install
ALLWIN_CORE_URL=http://localhost:8080 pnpm dev   # forwards /api to the Core; omit to work with mocked/no API
```

Open `http://localhost:3000`: it redirects to `/pt-BR` or `/en` from `Accept-Language`.
Without a Core, `/api/v1/me` fails and pages behave as signed out. `ALLWIN_CORE_URL` is for development only: a production build fails if it is set.

## Screens (V1-S3)

Public: `/login`, `/login/verify`, `/invite#t=…`, `/forgot-password`, `/reset-password#t=…`.
App (session required): `/connections/sources`, `/connections/ninjatrader`, `/settings/security`, `/settings/preferences`; platform admin only: `/admin/invites`, `/admin/users`.
Invite and reset tokens travel in the URL fragment, are removed at once with `history.replaceState`, and are sent only in the request body. One-time values (webhook URL, pairing code, TOTP key, recovery codes, invite and reset links) live only in component state.
The TOTP QR code is drawn locally from the `otpauth_uri` with `uqr` (no network).

## Checks

| Command | What |
|---|---|
| `pnpm lint` / `pnpm typecheck` | ESLint (feature boundaries, no `any`, no hard-coded JSX text) / `tsc` |
| `pnpm i18n:check` | key parity `messages/pt-BR.json` = `messages/en.json`; empty strings count as missing |
| `pnpm test` | Vitest (unit + component tests with MSW) |
| `pnpm test:e2e` | Playwright against a production build with `NEXT_PUBLIC_I18N_MISSING=marker` (fails on `⟦missing:` markers) |
| `pnpm gen:api:check` | fails when `schema.gen.ts` differs from what the contract generates |
| `pnpm gen:api` | regenerates `src/shared/api/schema.gen.ts` from `contracts/http/openapi.yaml` (`ALLWIN_OPENAPI=<path>` to override) |

First run of e2e: `pnpm exec playwright install chromium` (the host also needs Chromium's system libraries).

## Layout

`src/app` routes · `src/features/<feature>` feature modules (import only `shared` and themselves) · `src/shared` ui, i18n, format, api, session · `messages/` catalogs.
