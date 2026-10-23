# V1-S3 contract notes — access, sources, pairing

- **Status:** Ready for implementation (2026-10-23). Owner: Software Architect.
- **Contract:** [`contracts/http/openapi.yaml`](../../../contracts/http/openapi.yaml) (tags `auth`, `me`, `admin`, `sources`, `connections`, `pairing`, `events`).
- **Policies:**
  - [auth-and-session-policy.md](../security/auth-and-session-policy.md) (Accepted);
  - [ADR-0004 §8](../adr/0004-adapter-protocol-v1.md) (pairing).
  Data model: `docs/architecture/data/wave1-data-model.md` §4.
- **Design:** [v1-s3 spec](../../design/specs/v1-s3-access-sources-pairing.md) §3 operations map 1:1 to OpenAPI `operationId`s (table below).

## 1. Design operation → OpenAPI

| Design (spec §3) | OpenAPI operationId |
|---|---|
| `auth.login`, `auth.verify_totp`, `auth.verify_recovery_code`, `auth.logout` | `login`, `verifyLoginTotp`, `verifyLoginRecoveryCode`, `logout` |
| `auth.step_up` | `stepUp` (sensitive operations carry `x-step-up`) |
| `auth.get_invitation`, `auth.accept_invitation` | `lookupInvitation`, `acceptInvitation` (token in body) |
| `auth.request_password_reset`, `auth.reset_password` | `requestPasswordReset` (always 202), `confirmPasswordReset` |
| `auth.totp_setup_start/confirm`, `auth.totp_disable`, `auth.recovery_codes_regenerate`, `auth.change_password`, `me.security` | `startTotpSetup` (step-up replaces the password field of step 1), `confirmTotpSetup`, `disableTotp`, `regenerateRecoveryCodes`, `changePassword`, `getMySecurity` |
| profile time zone / locale (Q23) | `getMe`, `updateMe` (`time_zone`, `locale`) |
| `invites.list/create/revoke` | `listInvitations`, `createInvitation`, `revokeInvitation` |
| `admin.users.list/create_reset_link/reset_totp` | `listUsers`, `adminCreateResetLink`, `adminResetTotp` (both step-up) |
| `sources.list/create/rotate/revoke/update` | `listSources`, `createSource`, `rotateSource`, `revokeSource`, `renameSource` |
| `connections.list/create/new_pairing_code/unpair/update` | `listConnections`, `createConnection`, `newPairingCode`, `unpairConnection`, `renameConnection` |
| SSE `connection.status_changed`, `connection.paired`, `connection.pairing_code_invalidated` | `eventStream`, schema `Event` (+ `session.ended`) |
| Plugin pairing | `pairAdapter` (`POST /adapter/v1/pairings`) |
| S5 per-account opt-ins (automation, copy) | `setAccountOptIn` (`PUT /api/v1/accounts/{id}/opt-ins/{kind}`) |

## 2. Answers to the design's Architect items

| Item | Decision |
|---|---|
| Step-up for "accept automation" and admin actions (reset link, TOTP reset) | **Yes.** Added to the policy list (auth policy §1). Declining automation needs no step-up |
| Opt-in contract (Q24/Q25/Q26 approved) | `PUT /api/v1/accounts/{id}/opt-ins/{kind}` with `kind` = `automation` or `copy`, `choice` and `rule_version_id`. A stale version returns 409 `accounts.rule_version_changed` with `current_rule_version_id`, and nothing is saved. The choice is required when the stance for that kind is forbidden, conditional, unknown or not confirmed (Q25). `state` is `not_required`, `chosen` or `pending_review`; Q26 stance/text changes move it to `pending_review`. `accepted` requires step-up; `declined` does not |
| Account saved without a choice? | **Atomic create-with-choice.** S5 create/update requests embed `OptInChoices`; a missing required choice gives 422 `accounts.opt_in_required` and nothing is saved. Step-up happens **before** the single save, so cancelling it saves nothing. The `copy` choice is made atomically when the account is added as a follower (S7/S19). Consequently no new state value is needed and no account exists in a "saved, never chose" state. `pending_review` is reserved for Q26 (and for a requirement that appears later), and routing treats it as not accepted |
| Error codes (spec §3) | All adopted. Added:<br>• `auth.unauthenticated`, `auth.forbidden`, `auth.totp_already_enabled`, `auth.totp_setup_expired`;<br>• `connections.name_taken`, `connections.unpaired`, `connections.open_positions_not_acknowledged`, `sources.revoked`;<br>• `pairing.invalid_code`, `pairing.rate_limited`, `pairing.temporarily_locked`, `pairing.adapter_version_unsupported`;<br>• `me.invalid_time_zone`, `common.forbidden_origin`.<br>`connections.pairing_code_expired` is not used in responses; the web derives expiry from `expires_at` and SSE |
| `time_zone` in the profile | **Yes:** `Me.time_zone` (IANA), editable through `updateMe`, required at `acceptInvitation` (default from the browser, Q23) |
| Account bindings after re-pairing ("move to another machine") | **They follow the connection by `platform_account` name.** Bound accounts not reported by the new instance become out of sync (`sync.account_not_found_after_repair`) until the user rebinds or clears them. The old instance keeps working until the new code is used (ADR-0004 §8) |
| **Changes the design must apply** | (1) **Invitation and reset links use a URL fragment** (`/{locale}/invite#t=<token>`, `/{locale}/reset-password#t=<token>`) instead of a path segment, so tokens never reach server logs or `Referer` headers. The route becomes `/[locale]/invite` and `/[locale]/reset-password`, reading `location.hash`. (2) **No "invalidated after 5 wrong attempts" state**: the plugin's request carries only the code, so failures cannot be attributed to a connection (ALIGN-4). The SSE `connection.pairing_code_invalidated` now has reason `superseded` or `pairing_locked` (global lock after 100 failures in 10 minutes); the plugin shows the existing rate-limited state for `pairing.temporarily_locked` |

## 3. Cross-cutting implementation rules

- **Not found vs forbidden:** another user's resource returns **404 `common.not_found`**, never 403, so existence is not revealed (S3 AC-2). Admin endpoints return 403 `auth.forbidden` to non-admins.
- **Isolation tests:** every operation gets a two-user negative test (X-2).
- **One-time values:** `webhook_url`, `invite_url`, `reset_url`, `pairing_code`, TOTP `secret`/`otpauth_uri`, `recovery_codes` and `device_token` are never returned again and never logged. The web must not keep them in persisted client state.
- **Session events:** `session.ended` is sent on the SSE stream before it is closed, so open tabs can redirect to login.
