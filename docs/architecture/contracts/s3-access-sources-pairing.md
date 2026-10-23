# V1-S3 contract notes — access, sources, pairing

- **Status:** rev. 3 (2026-10-23): HTTP contract **2.0.0**, passwordless sign-in by one-time code (user requirement change; slice-plan V1-S3 rev. 7). The design spec rev. 3 password/TOTP screens are superseded; the S3 design is redone by the Product Designer from slice-plan rev. 7, and §1 maps the new operations. Rev. 2: Code Review 1 (HTTP contract 1.1.0). Owner: Software Architect.
- **Contract:** [`contracts/http/openapi.yaml`](../../../contracts/http/openapi.yaml) (tags `auth`, `me`, `admin`, `sources`, `connections`, `pairing`, `events`).
- **Policies:**
  - [auth-and-session-policy.md](../security/auth-and-session-policy.md) rev. 2 (passwordless; Crivo and Vigia approved, user items open: U16, Q28–Q35, M2/M3);
  - [ADR-0004 §8](../adr/0004-adapter-protocol-v1.md) (pairing).
  Data model: `docs/architecture/data/wave1-data-model.md` §4.
- **Design:** [v1-s3 spec](../../design/specs/v1-s3-access-sources-pairing.md) §3 operations map 1:1 to OpenAPI `operationId`s (table below).

## 1. Design operation → OpenAPI

| Flow (slice-plan V1-S3 rev. 7) | OpenAPI operationId (2.0.0) |
|---|---|
| Sign-in (AC-4, AC-5, AC-7) | `requestLoginCode` (always 202) → `verifyLoginCode`; `resendCode` (60-s cooldown); `logout` |
| Sign out everywhere (Q34) | `signOutEverywhere` |
| Step-up (AC-10) | `requestStepUp` (action + target + channel) → the action's body carries `step_up` (`StepUpProof`) |
| Delivery failure (AC-7, S3b AC-5) | `getChallengeDelivery` (never for login challenges) |
| Sign-up (AC-1, AC-2) | `lookupInvitation` → `startSignup` → `completeSignup` (token in body; account created only at `completeSignup`) |
| Security settings, devices (AC-9) | `getMySecurity`, `listDevices`, `forgetDevice` |
| Channel change (AC-11), first WhatsApp (S3b) | `startChannelChange` → `verifyChannelChangeCode` (per code) → completed or `on_hold`; `cancelChannelChange` ("Cancelar troca") |
| profile time zone / locale (Q23) | `getMe`, `updateMe` (`time_zone`, `locale`) |
| `invites.list/create/revoke` | `listInvitations`, `createInvitation` (step-up), `revokeInvitation` |
| `admin.users.list` | `listUsers` (`whatsapp_verified` replaces `totp_enabled`). Admin-assisted recovery waits for Q28 |
| Removed in 2.0.0 | `login`, `verifyLoginTotp`, `verifyLoginRecoveryCode`, `stepUp`, `acceptInvitation`, `requestPasswordReset`, `confirmPasswordReset`, `startTotpSetup`, `confirmTotpSetup`, `disableTotp`, `regenerateRecoveryCodes`, `changePassword`, `adminCreateResetLink`, `adminResetTotp` |
| `sources.list/create/rotate/revoke/update` | `listSources`, `createSource`, `rotateSource`, `revokeSource`, `renameSource` |
| `connections.list/create/new_pairing_code/unpair/update` | `listConnections`, `createConnection`, `newPairingCode`, `unpairConnection`, `renameConnection` |
| SSE `connection.status_changed`, `connection.paired`, `connection.pairing_code_invalidated` | `eventStream`, schema `Event` (+ `session.ended`) |
| Plugin pairing | `pairAdapter` (`POST /adapter/v1/pairings`) |
| S5 per-account opt-ins (automation, copy) | `setAccountOptIn` (`PUT /api/v1/accounts/{id}/opt-ins/{kind}`); first choice via `opt_ins` (`OptInChoices`) in the S5 create/update requests. Design deltas: [s5-design-deltas.md](s5-design-deltas.md) |

## 2. Answers to the design's Architect items

| Item | Decision |
|---|---|
| Step-up for "accept automation" and admin actions | **Yes** (auth policy rev. 2 §6.2; action list in `StepUpAction`). Declining needs no step-up |
| Opt-in contract (Q24/Q25/Q26 approved) | `PUT /api/v1/accounts/{id}/opt-ins/{kind}` with `kind` = `automation` or `copy`, `choice` and `rule_version_id`. A stale version returns 409 `accounts.rule_version_changed` with `current_rule_version_id`, and nothing is saved. The choice is required when the stance for that kind is forbidden, conditional, unknown or not confirmed (Q25). `state` is `not_required`, `chosen` or `pending_review`. Q26 (approved as "changing the stance") moves it to `pending_review` on a stance change. **Provisional, awaiting user confirmation:** a material change of the official rule text with the same stance also moves it to `pending_review`, because the audit stores the exact text the user saw (typo/formatting-only edits do not). `accepted` requires step-up; `declined` does not |
| Account saved without a choice? | **Atomic create-with-choice.** S5 create/update requests embed `OptInChoices` as the `opt_ins` property (pointers `/opt_ins/automation`, `/opt_ins/copy`) and carry conditional step-up (when an `accepted` choice changes the stored state); a missing required choice gives 422 `accounts.opt_in_required` and nothing is saved. Step-up happens **before** the single save, so cancelling it saves nothing. The `copy` choice is made atomically when the account is added as a follower (S7/S19). Consequently no new state value is needed and no account exists in a "saved, never chose" state. `pending_review` is reserved for Q26 (and for a requirement that appears later), and routing treats it as not accepted |
| Error codes (spec §3) | All adopted. Added:<br>• `auth.unauthenticated`, `auth.forbidden`; (2.0.0) `auth.code_invalid`, `auth.step_up_required`, `auth.channel_unavailable`, `auth.step_up_channel_not_allowed`, `me.channel_change_too_soon`, `me.channel_change_pending`, `me.channel_change_closed`, `me.channel_unchanged`, `me.invalid_channel_value`; removed: `auth.invalid_credentials`, `auth.invalid_code`, `auth.reauth_failed`, `auth.pending_expired`, `auth.password_*`, `auth.totp_*`, `auth.reset_token_invalid`;<br>• `connections.name_taken`, `connections.unpaired`, `connections.open_positions_not_acknowledged`, `sources.revoked`;<br>• `pairing.invalid_code`, `pairing.rate_limited`, `pairing.adapter_version_unsupported` (`pairing.temporarily_locked` removed in 2.0.0, U11);<br>• `me.invalid_time_zone`, `common.forbidden_origin`;<br>• (1.1.0) `common.invalid_request` (+ `pointer`), `common.internal`.<br>`connections.pairing_code_expired` is not used in responses; the web derives expiry from `expires_at` and SSE |
| `time_zone` in the profile | **Yes:** `Me.time_zone` (IANA), editable through `updateMe`, required at `completeSignup` (default from the browser, Q23) |
| Account bindings after re-pairing ("move to another machine") | **They follow the connection by `platform_account` name.** Bound accounts not reported by the new instance become out of sync (`sync.account_not_found_after_repair`) until the user rebinds or clears them. The old instance keeps working until the new code is used (ADR-0004 §8) |
| **Changes the design must apply** | (1) **Invitation links use a URL fragment** (`/{locale}/invite#t=<token>`) instead of a path segment, so tokens never reach server logs or `Referer` headers. The route becomes `/[locale]/invite`, reading `location.hash`. (Reset links no longer exist, 2.0.0.) (2) **No "invalidated after 5 wrong attempts" state**: the plugin's request carries only the code, so failures cannot be attributed to a connection (ALIGN-4). The SSE `connection.pairing_code_invalidated` has reason `superseded` (2.0.0: `pairing_locked` removed with the global lock, U11) |

## 3. Cross-cutting implementation rules

- **Not found vs forbidden:** another user's resource returns **404 `common.not_found`**, never 403, so existence is not revealed (S3 AC-2). Admin endpoints return 403 `auth.forbidden` to non-admins.
- **Isolation tests:** every operation gets a two-user negative test (X-2).
- **One-time values:** `webhook_url`, `invite_url`, `pairing_code` and `device_token` are never returned again and never logged. One-time codes, `challenge_id` tokens and the device cookie are never logged either. The web must not keep them in persisted client state.
- **Session events:** `session.ended` is sent on the SSE stream before it is closed, so open tabs can redirect to login.
- **Authentication (1.1.0):** the root `security` requires a full session; public operations declare `security: []`. Every session operation answers 401 `auth.unauthenticated`. `logout` is idempotent and accepts a session or none. The `pendingSession` scheme is gone (2.0.0).
- **CSRF / Origin:** the strict `Origin` check applies to state-changing `/api/v1` requests only (public ones such as `requestLoginCode` included), with 403 `common.forbidden_origin`. It does not apply to `/ingress/v1` or `/adapter/v1`.
- **Common errors:** schema violations of any `/api/v1` or `/adapter/v1` request (body, path or query) give 400 `common.invalid_request` with a JSON `pointer` (`/path/<name>`, `/query/<name>` for parameters); unexpected failures give 500 `common.internal` with no detail. Framework default bodies are never sent.
- **Check precedence:** 401 → 403 `common.forbidden_origin` → 403 `auth.forbidden` (admin operations) → 400 → 404 → 403 `auth.step_up_required` / `auth.code_invalid` → 409/422. A non-admin never gets a step-up prompt on an admin operation.
- **Step-up markers:** `x-step-up: true`, or `x-step-up: conditional` plus `x-step-up-when` (`setAccountOptIn`: `choice == accepted`). The operation is POST/PUT and its body has the optional `step_up` property (`StepUpProof`); `check_openapi.py` enforces both. The proof never travels in a header (access logs record headers).
- **Response headers:** every Core response carries `X-Content-Type-Options: nosniff` and `Cache-Control: no-store`; one-time-secret responses declare `Cache-Control: no-store` explicitly (ADR-0001 §9).
- **Admin actions on a user** (Q28 recovery, when decided) are written to the **affected user's** visible audit log, with the admin's id and time, as well as to the admin trail.
- **Throttling (2.0.0):** every code request (`requestLoginCode`, `resendCode`, `startSignup`, `requestStepUp`, `startChannelChange`) follows the send limits of auth policy §4 (429 `auth.rate_limited` + `Retry-After`), identical for unknown identifiers. Code verifications follow the wrong-attempt limits (5 per code, per-IP).
- **Enumeration (2.0.0):** code requests always answer 202; every verification failure is `auth.code_invalid`; delivery of a login code is never reported; a channel clash is never revealed (auth policy §3).
- **Read-only history rows:** renaming a revoked source gives 409 `sources.revoked`; renaming an unpaired connection gives 409 `connections.unpaired`. Connection names are unique case-insensitively among the user's connections that are not unpaired (`connections.name_taken`; unique index requested from the Database Architect).
- **Design fields:** `Source.is_automated` is derived by the Core from `source_type` (never stored); `Connection.instance_id` and `Connection.status_changed_at` exist; every SSE event carries `at`.
- **SSE:** events have no `id` and are not replayed; after every (re)connect the client refetches what it displays and applies only events with a newer `at`. `Last-Event-ID` is not used.
