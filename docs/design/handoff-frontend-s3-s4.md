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

## S3 rev. 4: passwordless (supersedes the rev. 3 items 1, 3 and 5 below where they mention passwords, reset or TOTP)

Slice-plan rev. 7 removed passwords, reset, TOTP and recovery codes. The contract (HTTP 3.4.1, auth published at 2.0.0) already has the passwordless operations; the S3 spec §3 lists them by `operationId` with their error codes. Where the spec and the contract differ, the contract wins.

1. **New shared components:** [components.md](components.md) C§1 `CodeInput` / `CodeChallenge` (one text input, uppercase alphanumeric, paste normalization, resend cooldown), C§2 `Stepper`, C§4 `DataTable`. `StepUpDialog` (F§8.2) now asks a code.
2. **Code format is open (U16):** read `length` and `alphabet` from config; do not hard-code 8 or the alphabet in features.
3. **Non-enumeration:** public pages show identical results for known and unknown e-mails (S3 §1). No inline existence check, no attempts counter.
4. **Invitation link:** fragment handling is unchanged (`#t=`), but the token is now used for `startSignup` and `completeSignup` (`locale`, `time_zone` and `remember_me` travel with `completeSignup`); the account exists only after the code is confirmed.
5. **Removed:** forgot/reset password routes, TOTP, recovery codes, admin Users page, password-related `session.ended` reasons.

6. **Policy rev. 2 rules the UI follows** ([auth-and-session-policy.md](../architecture/security/auth-and-session-policy.md)): one error `auth.code_invalid` for every verification failure (the UI counts its own 5 attempts); the `challenge_id` lives in memory only; a resend keeps the same challenge, a closed challenge needs a **new** one; step-up sends the action **once with `challenge_id` + `code`** (no verify-then-retry); a channel change never reveals "address in use"; no link in the new-device e-mail; sign-in delivery failures are never shown on public pages.
7. **Provisional (Q34/Q35, M2/M3):** `listDevices` and "Sair de todos os dispositivos", step-up on source URL / pairing code / invitation, 24 h freeze and 72 h hold on channel change. Build them behind the contract markers; they change when the user decides.

Items 2, 4, 6 and 7 below remain valid. Item 3 (TOTP) and the password parts of items 1 and 5 are void.

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
