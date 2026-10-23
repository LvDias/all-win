# Frontend handoff — S4 then S3 web

> **From:** Product Designer (Aquarela @ Dev3) · **To:** Frontend (Vitral @ Dev2) · QA · **Date:** 2026-10-23
> **Rule:** where a spec and the contract ([openapi.yaml](../../contracts/http/openapi.yaml), [S3 contract notes](../architecture/contracts/s3-access-sources-pairing.md)) differ, **the contract wins**. Tell the Designer so the spec is fixed.

## Read first

1. [foundations.md](foundations.md):
   - tokens (light/dark, AA-checked) and status tokens;
   - number, money and date formatting;
   - `SecretField`, `StepUpDialog`, `ResponsiveDialog`;
   - mobile rules, feedback states and glossary.
2. [specs/v1-s4-language-locale.md](specs/v1-s4-language-locale.md), rev. 2, final.
3. [specs/v1-s3-access-sources-pairing.md](specs/v1-s3-access-sources-pairing.md), rev. 3, aligned with the contract.

## S4 essentials

- Locale resolution: profile → `NEXT_LOCALE` cookie → `Accept-Language` (`pt*` / `en*`, else `pt-BR`). A signed-in user is redirected to the profile language.
- Switching: `updateMe { locale }`, then set the cookie and navigate to the same path and query.
  - A dirty form asks for confirmation first.
  - The switcher is disabled while a view-once dialog is open.
- Preferences: language plus IANA time zone (default from the browser). 24-hour clock in both languages.
- Message keys: `<feature>.<screen>.<element>`, ICU plurals. `errors.<code>` maps 1:1 to Core codes; an unknown code shows a generic message with the code.
- Missing translation: a visible marker in dev/QA, and Playwright fails on it. Never show a key name to the user.

## S3 rev. 3: changes from the contract

1. **Invite and reset links use the URL fragment.**
   - Routes: `/[locale]/invite` and `/[locale]/reset-password`.
   - Read `location.hash` (`#t=`), remove it at once with `history.replaceState`, and send the token in the POST body (`lookupInvitation` / `acceptInvitation`, `confirmPasswordReset`).
   - No `t` means the generic invalid state.
   - These pages use `Referrer-Policy: no-referrer` and load no third-party resources.
2. **Pairing:** the "invalidated after 5 wrong attempts" state no longer exists. SSE `connection.pairing_code_invalidated` carries a `reason`:
   - `superseded`: a newer code exists; offer `Gerar novo código`.
   - `pairing_locked`: global lock; tell the user to wait a few minutes.

   Wrong codes are never reported to the web.
3. **TOTP enrolment has 3 steps.** Step-up replaces the password step (`startTotpSetup` has `x-step-up`). `auth.totp_setup_expired` shows `Recomeçar`.
4. **`acceptInvitation` sends `time_zone`** (browser default, with an option to change it).
5. **SSE `session.ended`** sends the user to login with a reason:
   - `expired` → `session_expired`;
   - `password_reset`, `password_changed`, `totp_changed`, `admin_totp_reset` → `sessions_ended`;
   - `logout` → `signed_out`.
6. **Re-pairing:** account bindings follow by platform account name. Accounts the new instance does not report show "Não encontrada na nova instância".
7. **Error codes:** S3 spec §3 shows how each new code appears in the UI.

## Cross-cutting rules

- One-time values (`webhook_url`, `invite_url`, `reset_url`, `pairing_code`, TOTP `secret`, `recovery_codes`) live only in component state. Never in a persisted cache, URL, toast, analytics or logs.
- On `auth.step_up_required`: open the step-up dialog, then retry the original request **once**.
- Decimals arrive as strings. Format them with `Intl` from the string, never through `Number`. Inputs use the exact parser (F§6.3).
- Every user-visible string goes in both catalogs. Touch targets are ≥ 44 px, and every screen works at 320 px wide.

## Review

When screens are ready, ask the Designer (`maestri ask "Aquarela @ Dev3" "…"`) to review deviations together with QA. Behavior questions go to the Designer, who routes business questions to the PO.
