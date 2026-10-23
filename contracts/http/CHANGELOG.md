# HTTP API — changelog

## 4.2.0 — 2026-10-23 (Q71 decided; no schema change)
- Manual trades: a blank `commission` is computed by the Core with the ADR-0004 §7 per-fill rule on both sides: floor × quantity × 2 (`configured`), or 0 with no floor (`none`, "comissões não informadas"). The floor in force at save time is used, with no later recompute. `previewManualTrade` returns the value, and a typed commission overrides it (`manual`).
- Notes: s13 wording `none` = not reported and no floor (Crivo nit).

## 4.1.0 — 2026-10-23 (Code Review of 4.0.0; additive)
- F1: a lower (or removed) `commission_per_contract_side` applies from the next trading day (it loosens net limits; ADR-0006 §3.7, ADR-0004 §7); a higher one at once. New optional `AccountRisk.pending_commission_change` (`effective_from`, `new_commission_per_contract_side`, `change`: `commission_floor_lowered` / `commission_floor_removed`).
- QA Security (low, same as F1): in `updateAccount`, an absent `commission_per_contract_side` now means unchanged (exception to the full replacement), and `null` removes the floor explicitly. The removal applies from the next trading day and is audited.
- F2: `CommissionSource` uses the same per-fill sentence as adapter protocol 1.3 `fill.commission` / ADR-0004 §7: `platform` when reported ≥ floor × quantity (it won); `configured` when the floor won (reported lower, zero, or absent with a floor); `none` when absent and no floor.

## 4.0.0 — 2026-10-23 (BREAKING: re-release of 3.5.0 as a major; + Connection.disconnect_reason)
- **Versioning (Esquadro, ADR-0002 §4 / U18):** `Trade.pnl`, `TradesSummary.total_pnl` and the performance P&L figures change meaning from gross to net. A client that reads only `pnl` would show another number, so this is breaking: document major, paths stay `/api/v1`. The content of 3.5.0 (2778318, never merged or consumed) is unchanged; its "minor" reasoning is withdrawn.
- Root (Esquadro): `Connection.disconnect_reason` and optional `disconnect_reason` on the SSE `connection.status_changed` (`user`, `network`, `outdated`, `unpaired`; adapter protocol 1.3, close 4100; ADR-0004 §6).
- Name mapping to adapter protocol 1.3 (confirmed): `trade_completed.net_pnl` / `gross_pnl` / `commission` → HTTP `pnl` (net) / `gross_pnl` / `commission`; HTTP `commission_source` adds `manual` (manual trades only); `commission_per_contract_side` has the same name; `limits.evaluation` (protocol) and `AccountRisk.evaluation` (HTTP) are different views of the same data.

## 3.5.0 — 2026-10-23 (user decisions: net P&L, commission floor, evaluation target; Q37–Q68 decided)
- **Versioning:** (superseded by 4.0.0: re-released as a major) minor. `Trade.pnl`, the summary and the performance figures keep their shapes. `pnl_basis` already qualified `pnl`, and it gains the value `net`, now the only one sent. New fields are added. The meaning of `pnl` is therefore declared by `pnl_basis` (ADR-0002 §4; recorded for Esquadro and Crivo).
- **Net P&L as the main value** (S21 AC-1, S22 AC-3, S13 AC-2 rev. 8):
  - `Trade`: `gross_pnl`, `commission`, `commission_source` (`platform` / `configured` / `mixed` / `manual` / `none`).
  - Manual trades take an optional `commission`; the preview takes it too.
  - `TradesSummary` and `PerformanceFigures`: `total_gross_pnl`, `total_commission`, `uncommissioned_count`. A win needs net P&L > 0.
  - `AccountRisk`: `pnl_basis`, `commission_source`, `commissions_today`.
  - Audit `DetailsTrade` carries the gross P&L and the commission.
- **Commission floor (U20 a):** `commission_per_contract_side` on the account (create, update, response) and as a preset rule (`CommissionRule`, `RuleField`, apply choices).
- **Evaluation target (Q46, S13b; Q69 a, Q70 a):**
  - preset rule `evaluation_profit_target` (also in `RiskRules` and `PresetRuleValues`);
  - account `evaluation_start` + `evaluation_earned_before` (create);
  - `restartEvaluation`;
  - `AccountRisk.evaluation`;
  - codes `risk.evaluation_target_reached` (audit + notification registry, `success`) and `accounts.evaluation_restarted`; SSE `risk.state_changed.reason: evaluation_progress`.
- **Q67 (b) active:** `updateRoute` refuses switching on an ineligible route (422, `/is_enabled`); the Core switches routes off when their account becomes ineligible (audit `cause: account_ineligible`).
- **Q35 decided** (setting on): the `route_enable` / `account_binding_change` step-ups are required; module texts updated.
- "Provisional" marks for Q37–Q68 removed (decided 2026-10-23e).

## 3.4.1 — 2026-10-23 (review notes on 3.4.0; text only)
- `getPerformance`: a foreign or unknown `account_id` matches nothing (empty figures, never 404); two-user isolation test required (QA Security info).
- Rounding stated as half away from zero (negative averages); `bar_ratio` is `0.0000` when the largest |total_pnl| is 0 (Crivo N1, N2).

## 3.4.0 — 2026-10-23 (V1-S22 performance; additive) — notes: docs/architecture/contracts/s22-performance.md
- Module `performance`: `getPerformance` (totals and per-group figures by source / signal type, win / loss / breakeven counts, averages with sample counts, `bar_ratio`, `as_of`; rounding rules stated).
- `trades`: `recorded_before` on `listTrades` / `getTradesSummary`, `as_of` on the summary (S22 AC-2 reconciliation); `Trade.reconcile_mismatch` (adapter protocol 1.3 `trade_completed`, Esquadro; a partial excursion is sent as absent `mae` / `mfe`); audit code `trades.reconcile_mismatch`.
- Manual trades do not feed daily loss, drawdown, profit target, consistency or blocks; only the trade list and the metrics (PO rule: S13 AC-2 + S21 AC-3, PO clarification, slice-plan rev. 7; QA Security I1).

## 3.3.0 — 2026-10-23 (V1-S21 trades; additive) — notes: docs/architecture/contracts/s21-trades.md
- Module `trades`: `listTrades` (exit-time window, account / source / origin / signal filters, sort by exit time / P&L / duration, MAC cursor), `getTradesSummary`, `getTrade`, `createManualTrade` (`Idempotency-Key`), `updateManualTrade` / `deleteManualTrade` (manual only, Q56 provisional), `previewManualTrade`; SSE `trade.captured`.
- Audit codes `trades.captured`, `trades.manual_created`, `trades.manual_changed`, `trades.manual_deleted`.
- `risk.yaml`: `pending_changes[].change` gains `daily_loss_changed` (mixed change, treated as looser; Crivo note on 3.2.1).

## 3.2.1 — 2026-10-23 (Code Review of 3.2.0; additive)
- `AccountRisk.pending_reset_change` (a saved reset-time change, always from the next trading day) and `pending_changes[].change: drawdown_type_changed` with `new_drawdown`; `daily_loss_softened` carries `new_daily_loss` (Crivo F1).
- Notification registry: `reopen_on_occurrence` per `account_trading_day` code, true for `routing.risk_blocked`, `routing.out_of_sync`, `routing.expired` (provisional Q68 b; a / c only flip the flags); checked by `check_openapi.py`.
- Notes: no `critical` code yet in V1 (S11 / S15 add them); index hints and `group_key` for the notification table.

## 3.2.0 — 2026-10-23 (V1-S18 notifications; additive) — notes: docs/architecture/contracts/s18-notifications.md
- Module `notifications`: `listNotifications` (status, category, account; MAC cursor; critical unread first), `getUnreadCount`, `markNotificationRead`, `markNotificationsRead` (ids or `up_to`); SSE `notification.created`, `.updated`, `.read`. Fields identical to adapter protocol 1.3 (Esquadro): `notification_id`, `revision`, `code`, `category`, `severity` (info, success, warning, danger, critical), `params`, `count`, `source_event_ids`, `target` (structured, never a URL).
- New `contracts/notifications/v1/registry.json` (codes, category, severity, grouping, target, params) and the generated `params.schema.json` (JSON Schema `$defs` from the bundle); `check_openapi.py` checks registry ↔ `NotificationParamsByCode` and the drift of `params.schema.json`.
- S13 review notes (Crivo / Vigia on 3.1.0): consistency threshold pattern refuses 100.5 (A3); money patterns on `RiskMeter.limit`/`used`, `pending_changes.new_limit` and `DetailsRiskState` (A4); "looser" defined (removed rule, `flatten_and_block` → `notify`, `hard` → `soft_pause`/`none`) with `pending_changes[].change` (L1); any limits write → 409 on an open form (A2); `risk.state_changed` only to the owner (I2); ownership per ADR-0006 §3.7 (A1, notes only).

## 3.1.0 — 2026-10-23 (V1-S13 risk; additive) — notes: docs/architecture/contracts/s13-risk.md
- Module `risk`: `getAccountRisk` (trading day, meters with used / limit / truncated percent / state, unset and untracked rules, block, staleness, `limits_pending`, pending changes), `listRiskSummaries`, `getAccountRiskRules`, `putAccountRiskRules` (account limits version, `acknowledge_reached`), `previewAccountRiskRules`; SSE `risk.state_changed`.
- Audit codes `risk.rules_changed`, `risk.warning`, `risk.limit_reached`, `risk.target_reached`, `risk.consistency_warning`.
- Rule values stay owned by `accounts`; risk derives its limits from `account.limits_changed` (ADR-0006).

## 3.0.0 — 2026-10-23 (BREAKING for 2.1.0–2.5.0 readers; Code Review fixes of 2.4.0 and 2.5.0)
- **Versioning:** 2.1.0 changed response shapes (`OptInState.firm_stance` / `current_rule_version_id` became optional; `copy` may be `not_required`), and 2.5.0 changed `updateAccount` semantics. Following ADR-0002 §4 (before the first deploy, a breaking change bumps the document major; paths stay `/api/v1`), the modules are released as 3.0.0. 2.1.0–2.5.0 were never merged or deployed; 3.0.0 is the first consumable release of the modules (agreed with Esquadro).
- **accounts:**
  - `updateAccount` requires `opt_ins.automation` only when that request changes the automation rule version; `copy` is never required there.
  - `rule_values` on update only with a changed preset selection (422 `accounts.rule_values_without_selection_change`).
  - `AccountUpdateRequest` mirrors the create rules. `SizeAmount` refuses zero.
  - `setAccountOptIn` is idempotent (same choice and version: 200, no step-up).
  - `Account.preset_update_available`; `maxLength` on the firm/plan/phase names.
  - `preset_attached` is reserved (Q41).
- **presets:**
  - `RuleVersion.change.reason` and the material `ReleaseChange.reason` are optional (Q42 provisional).
  - `RuleVersion.firm_platform`. Stances are listed per platform (`FirmStances` lists, `StanceSummary.varies_by_platform`). `FirmRelease.rule_versions` (labelled, replaces `rule_version_ids`), with `vps` / `personal_device`.
  - `applyPresetVersion` takes `firm_platform`; `PresetDiff.firm_platform_required` / `firm_platforms`.
  - Stricter provenance and change schemas. A field may not be in both `choices` and `rule_values`.
- **routing:**
  - `route.changed` (+ `cause`) on every effective-status change.
  - `routing.no_bracket_profile` carries `instrument_ids`. New `routing.limits_pending` (ADR-0006).
  - `blocks` and `status` are enforced per code.
  - Reserved 422 on `updateRoute` (Q67 option b). The manual-only reason is a `RuleQuote`.
- **profiles:**
  - `restoreAccountProfile` reuses `copied_from_size`, requires `profile_size` on a tie (Q60), and takes an optional `row_version`; its no-op rules are stated.
  - `putAccountProfile` rejects a `row_version` sent without a profile (409). Money patterns on `tick_value`, `point_value` and `usd`.
- **security (QA Security, Vigia):** step-up supersets, pending the user with Q35 (Core setting, default on).
  - M1: `createRoute` / `updateRoute` require `step_up` (action `route_enable`, target = account) when they switch on a route from an automated source for an account whose automation was not accepted with step-up (real or `not_required`).
  - L1: `updateAccount` requires `step_up` (action `account_binding_change`, target = account) when it changes the binding of an account that receives automation.
  - Root (agreed with Esquadro, root owner): `StepUpAction` gains `route_enable` and `account_binding_change` (pending Q35, setting `step_up.q35_persistence_actions`), both with target = account; `requestStepUp` requires `target_id` for them; the description lists `applyPresetVersion` under `account_opt_in_accept`.
  - I2: `RuleQuote` is plain text, never rendered as HTML.
- **audit:**
  - `listActivity`: `from` defaults to the horizon; `from` > `to` → 400; the MAC cursor captures the resolved window and filters; order (`occurred_at`, event sequence, `id`).
  - The timeline is ordered by the causal event sequence.
  - `audit.entry_recorded` carries the filter keys. New `actor`. `safe_to_display` is presentation only.
  - Codes `sources.*`, `connections.*`, `routing.limits_pending`. `DetailsPresetApplied` has both version numbers; `FieldChange` serialization is defined.

## 2.5.0 — 2026-10-23 (V1-S16 prop firm presets; additive) — notes: docs/architecture/contracts/s16-presets.md
- Module `presets`: `listFirms`, `getFirm`, `listFirmReleases`, `getPresetVersion`, `getAccountPreset`, `getAccountPresetDiff`, `applyPresetVersion` (per-field `choices` / `rule_values`, `opt_ins`, step-up `account_opt_in_accept`), `restorePresetValue`; SSE `presets.version_available`. Typed rule values with provenance (`verified` / `not_confirmed` + published values), VPS and personal-device stances, manual-only reason.
- `createAccount` / `updateAccount` gain `rule_values` (prop with a preset); `updateAccount` to a newer version of the same preset → 422 `presets.use_apply_version`.
- Audit codes `presets.version_applied`, `presets.value_restored`.

## 2.4.0 — 2026-10-23 (V1-S10 activity; additive) — notes: docs/architecture/contracts/s10-audit.md
- Module `audit`: `listActivity` (filters, opaque cursor, `retention_months`, `horizon`), `getActivityFilterOptions`, `getActivityEntry`, `getInstructionTimeline`; SSE `audit.entry_recorded`.
- Final `AuditOutcome` (10 values, incl. `unknown`), open `AuditCode` + `KnownAuditCode`, typed `details` per code (`AuditDetailsByCode`), `safe_to_display`, `is_exit`; `execution.*` codes; `security.*` codes (S3 AC-16, masked values only).

## 2.3.0 — 2026-10-23 (V1-S6 stop/target profiles; additive) — notes: docs/architecture/contracts/s6-profiles.md
- Module `profiles`: `listInstruments`, `listDefaultProfiles`, `putDefaultProfile`, `listAccountProfiles`, `putAccountProfile`, `restoreAccountProfile`, `getNearestProfileSize` (Q60 tie, provisional), `previewProfile` (stateless; Core arithmetic, rounding toward the entry).
- `RouteReasonCode` gains `routing.no_bracket_profile` (Q43, provisional).

## 2.2.0 — 2026-10-23 (V1-S7 routing; additive) — notes: docs/architecture/contracts/s7-routing.md
- Module `routing`: `listRoutes` (effective status + ordered reason codes), `createRoute`, `updateRoute`, `deleteRoute`, `listRouteCandidates` (blocking reasons, manual-only list); SSE `route.changed`.

## 2.1.0 — 2026-10-23 (V1-S5 accounts) — notes: docs/architecture/contracts/s5-accounts.md
- Module `accounts`: `listAccounts`, `createAccount` (`Idempotency-Key`, atomic `opt_ins`, step-up `account_create_opt_in_accept`), `getAccount`, `updateAccount` (PUT, full replacement, `row_version`), `archiveAccount`, `getAccountArchiveImpact`, `setAccountLifecycle`; `setAccountOptIn` moved from the root; SSE `account.changed`, `account.archived`.
- Module `presets` (S5 part of the S16 catalog): `getOptInRules` (incl. the generic rule versions, Q40 provisional), `getRuleVersion` (`change` with cause, material flag reason, Q42 provisional).
- `OptInKind`, `FirmStance`, `OptInChoiceRequest`, `OptInChoices`, `OptInState` move to `modules/accounts.yaml` (names unchanged). `OptInState`: `firm_stance` / `current_rule_version_id` optional (absent only for real accounts); new `pending_reason` (required with `pending_review`); `copy` is `not_required` for an account that is not a follower (Q-S5-12).

## 2.0.0 — 2026-10-23 (BREAKING: passwordless sign-in; auth-and-session-policy.md rev. 2)
- **Versioning:** major bump of the document; paths stay `/api/v1` because no version was ever deployed and there are no external clients. The path major moves only for breaking changes after the first production deploy (Maestro/Architect decision 2026-10-23; ADR-0002 §4).
- **Removed:** `login`, `verifyLoginTotp`, `verifyLoginRecoveryCode`, `stepUp` (password/TOTP), `acceptInvitation`, `requestPasswordReset`, `confirmPasswordReset`, `startTotpSetup`, `confirmTotpSetup`, `disableTotp`, `regenerateRecoveryCodes`, `changePassword`, `adminCreateResetLink`, `adminResetTotp`; security scheme `pendingSession`; schemas `NewPassword`, `PasswordAndCode`, `RecoveryCodes`; responses `InvalidCredentials`, `PasswordRejected`, `StepUpRequired` (→ `StepUpFailed`); `LoginResult.state`/`pending_expires_at`/`recovery_codes_remaining`; `SecurityState.totp_enabled`/`recovery_codes_remaining`/`session.step_up_valid_until`; `AdminUser.totp_enabled` (→ `whatsapp_verified`).
- **Added — sign-in and sign-up by one-time code:** `requestLoginCode` (always 202, sealed `challenge_id`), `verifyLoginCode`, `resendCode`, `getChallengeDelivery` (never for login), `startSignup`, `completeSignup`, `signOutEverywhere`; schemas `CodeChannel`, `ChallengeId`, `OneTimeCode`, `ChallengeIssued`; header `SessionAndDeviceCookies` (`__Host-aw_device`); `LoginResult.new_device`.
- **Added — devices and channels:** `listDevices`, `forgetDevice`, `startChannelChange`, `verifyChannelChangeCode`, `cancelChannelChange`; schemas `Device`, `ChannelChange` (status `on_hold` + `hold_until` ready for the §9 hold and the [Proposed] M2 hold, so adopting M2 does not change the contract); `SecurityState.email`/`whatsapp`/`channel_change_allowed_at`/`pending_channel_change`.
- **Step-up:** `requestStepUp` (`StepUpAction`, `target_id`, `channel`) sends a code bound to session + action + target; every `x-step-up` operation is POST/PUT with an optional `step_up` body property (`StepUpProof`); `rotateSource` and `newPairingCode` gain a body (`StepUpBody`); `createInvitation` now requires step-up (with `source_create`/`source_rotate`/`connection_create`/`pairing_code_new`: **pending the user's Q35**: `x-step-up: conditional` on a Core setting `step_up.q35_persistence_actions`, default on; the `StepUpAction` enum is a superset, so the decision never changes the contract); `setAccountOptIn` body is inline with `step_up`. Errors: 403 `auth.step_up_required` (missing) / `auth.code_invalid` (wrong); 422 `auth.step_up_channel_not_allowed` reserved for the [Proposed] M3 rule. `check_openapi.py` enforces the body proof.
- **Errors:** single `auth.code_invalid` for every code failure (401 when signing in/up, 403 when authenticated); new `auth.channel_unavailable`, `me.channel_change_*`, `me.channel_unchanged`, `me.invalid_channel_value`.
- **Rules fixed in review:** `requestStepUp` target rule (required/forbidden per action → 400 `/target_id`; foreign or unknown target → 404, no code sent); `resendCode` response and counting depend only on the sealed token and counters; `completeSignup` has no 404 (closed invitation → 401 `auth.code_invalid`); a device cookie valid for another user of the same browser is kept; `getChallengeDelivery` gets 429.
- **U11:** `pairing.temporarily_locked` and the SSE reason `pairing_locked` removed (no global pairing lock).
- **SSE:** `session.ended` reasons are `logout`, `expired`, `signed_out_everywhere`, `device_revoked`, `channel_changed`.

## Tooling (no contract change, no version bump) — 2026-10-23
- Multi-file layout: root `openapi.yaml` + `modules/<module>.yaml` (paths and module components), one owner per file (README).
- `openapi.bundled.yaml` is generated by `check_openapi.py --write`, committed, and drift-checked; all checks run on the bundle. Web and Core drift checks use the bundle.

## 1.1.3 — 2026-10-23 (YAML fix, no semantic change)
- Two flow-mapping descriptions containing a comma were parsed as an extra key with a null value (`changePassword` 204, `startTotpSetup` `secret`); they are now quoted. `check_openapi.py` fails on any null value. Found by Backend (S3 drift check).

## 1.1.2 — 2026-10-23 (S1 implementation deltas)
- Ingress credential guard (s1 notes §2 step 5a, after parsing and before schema validation): a body string shaped like an All Win token (`aw[kd]_…`) → 400 `ingress.schema_violation` with an empty `pointer`, audited with the empty pointer; the body is not stored or logged.
- Ingress rate limiting (QA Security M1): the per-IP limit before auth becomes a flood cap, with a much higher cap for configured TradingView source ranges; token failures charge a per-IP failure budget (429 on failures only, valid tokens never refused); the per-key limit stays. The over-budget 429 is identical for every failure kind; unknown key_ids go into a 60-s negative cache; trusting X-Forwarded-For is a deployment requirement. Defaults in s1 notes §2 (resolves U14). IPv6 is keyed by /64.

## 1.1.1 — 2026-10-23 (clarifications for S1 implementation)
- Ingress 422 adds `ingress.signal_type_not_allowed_for_source` (ADR-0003 §6 policy had no code). Ingress 400 `ingress.unsupported_version` documented with pointer `/schema_version` for a minor newer than the Core supports.

## 1.1.0 — 2026-10-23 (Code Review 1 fixes; additive except where noted)
- Root `security: [{session: []}]`; public operations keep `security: []`; `logout` accepts any session state; 401 `auth.unauthenticated` on every session operation.
- Common errors: 400 `common.invalid_request` (+ `pointer`) on every `/api/v1` and `/adapter/v1` operation with input; 500 `common.internal` everywhere. Ingress path/method violations stay the empty 404.
- `Origin` check scoped to state-changing `/api/v1` requests; 403 `common.forbidden_origin` documented.
- Check precedence defined (`auth.forbidden` before `auth.step_up_required` on admin operations).
- `Cache-Control: no-store` declared on one-time-secret responses; `nosniff` + `no-store` on every Core response (ADR-0001 §9).
- `x-step-up: conditional` + `x-step-up-when` replaces the free-text value on `setAccountOptIn`.
- 429 on `changePassword`, `regenerateRecoveryCodes`, `confirmTotpSetup`. `stepUp` without `code` (TOTP user) → 401 `auth.invalid_code`.
- `renameSource` 409 `sources.revoked`; `renameConnection` 409 `connections.name_taken` / `connections.unpaired`.
- New response fields (required): `Source.is_automated` (derived), `Connection.status_changed_at`, `at` on every SSE event; optional `Connection.instance_id`.
- `OptInState` conditional requirements (`chosen` / `pending_review` / `not_required`); `OptInChoices` embedded as `opt_ins`. Q26 text-change trigger marked provisional.
- SSE: no event ids, no `Last-Event-ID`; refetch after reconnect.
- Ingress processing order: per-IP rate limit first; token check before reading the body. Masking rule replaced (ADR-0001 §5, S1 notes §4).

## 1.0.0 — 2026-10-23 (ready for Wave 1 implementation)
- S1: `POST /ingress/v1/tradingview/{token}`.
- S3: auth (login, TOTP/recovery verify, logout, step-up, invitations lookup/accept, password reset), me (profile with time_zone, security, TOTP setup/confirm/disable, recovery codes, password), admin (invitations, users, reset link, TOTP reset), sources, connections (create, rename, pairing code / re-pair, unpair), plugin pairing `POST /adapter/v1/pairings`, SSE `/api/v1/events`.
- S5 (requested by design; Q24/Q25/Q26): `PUT /api/v1/accounts/{account_id}/opt-ins/{kind}` (`automation` | `copy`) with `rule_version_id`, `accounts.rule_version_changed`, `accounts.opt_in_not_applicable`; schemas `OptInChoices` (atomic create-with-choice for S5, `accounts.opt_in_required`), `OptInState` (`not_required` | `chosen` | `pending_review`), `FirmStance`.
