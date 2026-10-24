# HTTP API — changelog

## 6.1.0 — 2026-10-24 (additive; S11 clear protection rules; on top of 6.0.0)
- **"Covers", strictly (Vigia L1; ADR-0004 §6.4):** only working stop and stop-limit orders on the opposite side, same account and same contract, count, with total working quantity ≥ |position| for `full`. A same-side stop, a limit or market order, or another contract never counts. `AccountSync.positions[].protection` states the four values exactly.
- **What a clear keeps (Q98 b, Q100 a; Esmeril, fail closed):** `partial`, `none` AND `unknown` keep (or raise) `sync.bracket_incomplete` per instrument; only `full` releases. 6.0.0 named only `none` / `partial`.
- **The block after a clear (Aquarela / Bisturi):** new `SyncReason.held_at_clear` (optional) on each `sync.bracket_incomplete` a clear kept. The block reason stays `sync.bracket_incomplete` in `AccountSync.reasons`, and routing refuses entries with `routing.out_of_sync`, as before.
- **Re-evaluation and release (Vigia L1):** after a clear, the Core re-evaluates the listed positions on each evaluated snapshot and order report until flat. A held reason resolves when its position is `full` or flat; when the last active reason was held, the account returns to `in_sync` by itself, audited as `sync.restored` (with `reason`), with no second clear (Q101 yes, user 2026-10-24; ADR-0004 §6.4 r6.1). The web says the block ends by itself (Vigia I2). A covered position that loses its cover raises `sync.bracket_incomplete` again (not held, sticky): entries are blocked again at once until a new clear (Q101 user addition).
- `protection_source` is forbidden with `none` / `unknown` (schema `if`/`then`; Crivo q1).
- Idempotency-Key retention 24 h for `clearAccountSync` and `restartTradingDayCount`, as in `createAccount`; an expired key runs as a new request (Crivo q2).
- **S15 audit and notification codes (Rumo's S18 review; registry 1.6.0):** `risk.blocked` and `risk.flatten` (danger), `risk.unblocked` (info, reason `trading_day_reset`), `risk.reactive_block_action` (warning, grouped per account and trading day, no reopen); details `DetailsRiskEnforcement`, `DetailsRiskUnblocked`, `DetailsReactiveBlockAction`. `enforcement_id` (required on blocked / flatten; optional on unblocked / reactive) correlates the entries of one block episode, so S18 groups a limit breach's notifications without heuristics (Nanquim). Severity and grouping of `risk.reactive_block_action` confirmed by Esquadro. A reactive flatten while blocked is `risk.flatten` (ADR-0004 §7 r6.1). With trigger `enabled_over_limit`, `kind` / `limit` / `used` come from the Core's risk state at that `limits_version` (Crivo n1).
- **Q99 (user, 2026-10-24):** a `sync.flagged` with a NEW reason reopens the notification group the user read or dismissed (`reopen_on_occurrence: true`, already set since registry 1.4.0; the rule text now cites Q99).
- **`execution.exit_sent_late` (T1):** audit only, no notification; written with the late exit's `execution.command_sent` (`late_ms` present). Instructions lost in a disconnect keep `routing.expired` + `sync.flagged` (no new code).
- Owners to notify: Core (gateway / sync: evaluation and release; S15 risk_event → audit), Plugin (S15 risk_event kinds), Web (Aquarela, Vitral: `held_at_clear`, the card line, the dialog sentence for `partial` / `unknown`), QA.

## 6.0.0 — 2026-10-24 (BREAKING; S8 stop/target mode, S3 AC-12, S13/S16/S21/S22 gaps, Q79/Q86/Q90/Q92/Q96; on top of 5.3.0)
- **Versioning (ADR-0002 §4, U18; Esquadro, Esmeril M1):** this is a breaking change before deploy, so it is a document MAJOR, and the paths stay `/api/v1`.
  - **What breaks:** `include_archived` defaults to false on `listTrades`, `getTradesSummary` and `getPerformance`. A call that sends nothing new no longer returns archived accounts' trades, so both the rows and the totals change. Marked `x-breaking` on the parameter.
  - **What the client does:** send `include_archived=true` to keep the earlier behaviour.
  - Everything else in this release is additive or text.
  - Owners to notify: Web (Vitral: S21/S22 types and the default), Core (Malha / Ferro: S21/S22 queries).
- Review fixes, Esmeril and Crivo:
  - the Idempotency-Key replay of `restartTradingDayCount` compares the body WITHOUT `step_up`, so a retry with a new code gets the recorded 202 (Esmeril L1);
  - the replay store is keyed by (user_id, operation, account_id, Idempotency-Key), never by the key alone (Vigia L1);
  - a repeat while a request is pending: the same key returns the recorded 202; a new request with a valid step-up returns 202 with the pending request (Crivo L2);
  - `ingress.duplicate_conflict` targets the original instruction (Esmeril L3);
  - `verification_ref` follows auth policy 3.3 §9.1: required with `verified_by: other`, and refuses `@`, `+digit`, more than 6 consecutive digits and token shapes (Crivo L3);
  - `setAccountCommission` checks a stale `row_version` first, giving 409 (n1);
  - `trailing_lock.offset` absent means 0 (n2);
  - `differing_fields` maxItems 10 (n3).
- **`clearAccountSync` (the planned 5.3.1, folded in here):**
  - optional `Idempotency-Key`; the replay comes before the step-up, compares the body without `step_up`, and is keyed by (user_id, operation, account_id, key). This replaces the 5.3.0 replay by `expected_version` (Esquadro; Crivo c1; Vigia L1).
  - Q98 (b), user: ANY working stop covering a position counts as protection, a manual NT8 stop included. New `positions[].protection_source` (`all_win` | `external` | `mixed`). Only a position not fully covered by any stop keeps `sync.bracket_incomplete` and the entry block.
  - `SyncReasonCode` cites only ADR-0004 §6 as canon (Esmeril nit).
- **Page titles (user decision 2026-10-24; preset release 1.1):** `RuleQuote.page_title` is optional. When the firm's site blocks automated access (Apex, TPT), `title_note` replaces it, and clients show the URL as the link; the title is never derived. `SourceRef.title_note` is optional, without `page_title` only. Clients must allow a quote without a title. `page_title`, when present, has minLength 1 (Guarita).
- New `BracketMode` (`individual` | `shared`), now also the type of `DetailsBracketCalculated.mode` (same values).
- `Account.bracket_mode` (required in responses), `AccountCreateRequest.bracket_mode` (default `individual`), and `AccountUpdateRequest.bracket_mode` (absent = unchanged, an exception to the full replacement). A change is audited as `accounts.updated`.
- **Q86 (b), user 2026-10-24:** 409 `accounts.bracket_mode_open_position` (`/bracket_mode`) when the mode changes with an open position or working All Win orders.
- `AccountBinding.activation` (`active` | `pending_handshake`, required; Esquadro X2): a bind or rebind made after the connection's last handshake takes effect at the next handshake. Meanwhile commands stay pending, entries expire by TTL and exits wait. The Web shows "vale quando o plugin reconectar" when the session is open.
- `PerformanceFigures.avg_gross_pnl` (summary and every group; required when `trade_count` > 0, absent at 0). S22 AC-3 applies "gross beside" to averages too (PO).
- `setAccountCommission` `PUT /api/v1/accounts/{account_id}/commission` (Vitral): sets or removes (`null`) the commission floor alone, with the same rules as in `updateAccount` and `row_version`. The web no longer rebuilds the full-replacement body, where an absent `binding` unbinds.
- `AccountRisk.evaluation_target_status` (`set` | `not_confirmed` | `absent`, S13 §12.1 AC-3, Vitral): the evaluation card also for an evaluation-phase account whose target is not confirmed or not published.
- `FirmDetail.platforms[].manual_only` (S-R3-2/4, Torno): a firm platform unreachable by V1. Its accounts are manual only, like a firm-level `manual_only`.
- `personal_device` stance gains `not_confirmed` (`FirmStances`, `FirmSummary.stances`; S16 AC-5, Q-S16-7, PO): conflicting firm pages give at least 2 quotes and nothing prefilled. Same in the preset release schema.
- `AccountRisk.net_pnl_today` and `gross_pnl_today` (with `commissions_today`; S13 AC-11, PO): the day summary line only. Limit meters and the evaluation card stay net. `restartEvaluation`: an omitted `evaluation_earned_before` keeps the stored value with the same start, and is none (0) with a new start (Crivo L2).
- `putAccountRiskRules` bumps only the limits `version`, never `Account.row_version` (Grosa). For a combined save, the rules go first, then `setAccountCommission`, which bumps both. `RiskRules.consistency_threshold_pct` 0 → 422 `risk.amount_invalid` (Malha/Agulha; pattern unchanged, the same convention as zero amounts).
- **S3 AC-12 admin recovery (Cinzel; root parts are Esquadro's, text and enum only):** `ChannelChange.kind` (`user_change` | `admin_recovery`, required; `codes` empty for `admin_recovery`, at least one entry otherwise); `StepUpAction` `admin_recovery` (start: target = the target user's id; confirm and cancel: target = the change id; a new step-up each time; `requestStepUp` refuses it with 400 `/action`); `DetailsSecurityChannelChange` gains `kind`, `verified_by`, `verification_ref` and `break_glass`. The actor is `platform_admin`, and the admin's id is not in the user's trail. The existing codes `security.channel_change_*` are reused; there is no new code.
- **Ingress duplicate with a different body (Esquadro):** the answer is still 200 and nothing is executed. New audit `ingress.duplicate_conflict` (`DetailsIngressDuplicateConflict`: source name, message_id, original instruction, first and second receipt time, and only the NAMES of the differing compared fields). The comparison is a closed list of fields (`source`, `signal_type`, `action`, `position_effect`, `side`, `instrument`, `quantity`, `order`, `changes`, `refs`) kept as the sha256 of the canonical parsed JSON, only while the content is kept: the purge (Q13-b) clears it in the same batch. After the purge, a repeat is `ingress.duplicate_ignored` with the new `content_not_retained: true`, never a conflict, so `differing_fields` stays required (Esquadro, final). `sent_at`, `reference_price`, `details` and `schema_version` are not compared. Expiry comes from the first receipt. Notified as warning, grouped by the new grouping `source_trading_day`, target `activity_entry`. Registry 1.5.0.
- `ReleaseChange.firm_platform` (optional): the platform of a per-platform rule or text change (preset release diff 1.0.1, Torno).
- `listTrades` / `getTradesSummary` `to`: explicitly exclusive (`exit_time < to`, interval [from, to)); text only (Vitral).
- S16 reads (Torno): `RuleVersion.research.sources` becomes `SourceRef` (`accessed_at` required, `page_title` optional), as in `StanceInfo` and the release file. `not_confirmed_count` is defined: per preset the distinct `not_confirmed` rule fields, per firm the sum over the current release, stances excluded. `FirmDetail.plans` order: file order, sizes ascending by `size_amount`.
- `accounts.opt_in_rule_updated` (`DetailsOptInRuleUpdated`; audit only, not notified; Esquadro S16): the automatic re-point of a `chosen` / `accepted` opt-in to a `not_material` rule version, with the same choice and `chosen_at`. Until the path exists in the binary, the `publish-release` CLI refuses a release with `change.cause = not_material` (fail closed, Esmeril L1). If the job finds one anyway, it alerts and does not advance its cursor.
- `include_archived` (boolean, default false) on `listTrades`, `getTradesSummary` and `getPerformance` (Q79 a, user 2026-10-24): archived accounts' trades are kept but hidden unless asked for. The Core applies the filter, so paging and totals match. BREAKING: before 6.0.0 they were listed by default (see Versioning).
- **Unknown trading day (Q92 c, user; protocol 1.4):**
  - `AccountRisk.day_state` / `day_unknown_reason` / `day_restart_pending`;
  - new `restartTradingDayCount` `POST /api/v1/accounts/{account_id}/risk/day-restart`, with `acknowledge_unseen_losses: true`: 202; 409 `risk.day_not_unknown` / `risk.adapter_protocol_outdated`; step-up `trading_day_restart` (see the next item);
  - audit `risk.day_restarted` (`DetailsDayRestarted`, actor user);
  - `DetailsRoutingDecision.risk_detail: day_unknown` for the gate-7 skip.
  - Never automatic.
- `restartTradingDayCount`: step-up `trading_day_restart` (Q96, target account, 403 `StepUpFailed`) and an optional `Idempotency-Key`. The replay is checked BEFORE the step-up and returns the recorded 202 without a code (Esquadro); the same key with another body gives 409 `common.idempotency_key_reused`. The enum value comes from 5.3.0.
- `Trade.recovered` (boolean, S21 AC-13 / Q80 a): built from executions recovered from the platform's history; the "Recuperado" badge (Nanquim).
- `applyPresetVersion`: on a `cap_conflict` row the top-level `max_position_qty` is the answer, and `choices.max_position_qty` must be absent. `restorePresetValue` allows `max_position_qty` (Mosaico).
- **Q90 (a), user: `DrawdownValue.trailing_lock`** {`mode` `floor_reaches_start` | `never`, `offset` ≥ 0 only with `floor_reaches_start`}. Trailing types only. The floor locks at `size_amount` + offset; absent = the interim (keeps trailing + warning); `stops_trailing_at` stays the published text. Q48: adding a lock or lowering the offset loosens (next day, `pending_changes` `drawdown_lock_changed`); removing it or raising the offset tightens (at once). A restart with a new start resets the lock. The protocol already carries `lock_at_balance` / 1.4 `lock_above_start`.
- **Q-S13b-3 (PO, option a; no new operation):** `restartEvaluation` names the audit by effect. A changed start is `accounts.evaluation_restarted`. The same start with another earned-before is new `accounts.evaluation_corrected` (`DetailsEvaluationCorrected`), never a restart. Both the same is a no-op, not audited. An absent earned-before is unchanged for a correction and none for a restart. Either change bumps `limits_version`.

## 5.3.0 — 2026-10-24 (additive; S11 out of sync; on top of 5.2.0)
- `Account.sync` (`AccountSync`, required in responses): `state`, `version`, `flagged_at`, `observable`, `reasons[]` and `resolved_reasons[]` (`SyncReason`: code, detected/resolved time, instrument/instruction/command, expected vs reported position, partial-fill quantities, `core_action` = what All Win did). `SyncReasonCode`: closed list of 21 codes (incl. `command_unknown` and `flatten_failed_unprotected`) in display order (Esquadro's canon, ADR-0004 r5), with `adapter_reason_code` and `snapshot_trigger`. A clear rebases the expected state on the last evaluated snapshot, so an accepted condition never re-flags.
- `clearAccountSync` `POST /api/v1/accounts/{account_id}/sync/clear` (S11 AC-3): `expected_version` precondition, 409 `accounts.sync_state_changed` / `accounts.not_out_of_sync` / `accounts.sync_not_observable` (Q83 a: no clear while not observable; Q84 a: a clear with an open position is allowed, with confirmation), replay → 200, step-up `account_sync_clear` (Q95), audited `sync.cleared`. Nothing is sent to the platform.
- `RouteReasonCode` `routing.connection_checking` (route status only): from `welcome` until the account's reconnect evaluation; no command sent, entries keep their TTL, exits wait (S11-03, ADR-0004 §4 step 3 / §6.1 r5).
- Audit `sync.flagged`, `sync.reason_resolved`, `sync.cleared`, `sync.restored` (`DetailsSync`). Their category is `connection` even for reasons that are not about the connection, accepted for V1 by Esquadro (a new category is a protocol minor).
- `DetailsCommandSent.late_ms` (optional): an exit sent after `expiry_base` + the account TTL ("audited as late", ADR-0004 §5 r5, T1). It is never set on an entry.
- Audit `execution.unbound_activity` (`DetailsUnboundActivity`, category execution): activity on a platform account bound to no All Win account. It is not a sync reason. Registry: warning, target connection. `SyncReason.expired_by` (core | adapter). A `command.nothing_to_reduce` against a non-flat expected position maps to `sync.position_changed` (Esquadro).
- `SyncReason.instrument_symbol` and `contract_month` (wire values; the gateway has no catalog id, so `instrument_id` is optional). They are part of the dedupe key; `DetailsUnboundActivity` follows. `DetailsExecution.child_role` gains `exit` and `flatten` (protocol 1.3 roles; Bigorna).
- Audit-only codes (Bigorna, Esquadro): `execution.undelivered` (`DetailsUndelivered`: command_id, created_at, horizon_h), and `connection.account_kind_changed` (`DetailsAccountKindChanged`: account, connection, platform account, previous and new kind).
- Registry 1.4.0: `sync.flagged` (danger, grouped, reopens), `sync.bracket_orphan_leg` (warning), `sync.bracket_incomplete` (danger), `sync.bracket_double_fill` (critical), `sync.restored` (info). Notes s11.

- Code Review of 5.3.0 (Esmeril, Crivo):
  - `sync.flatten_failed_unprotected` (critical, U19, any origin; 21st code, confirmed by Esquadro);
  - `adapter_reason_code` uses the protocol pattern (one or more dots);
  - `reasons_total` / `resolved_total` cover overflow (the most severe 32 shown; unprotected-exposure reasons never hidden);
  - `out_of_sync` requires at least one reason, active or resolved;
  - `cleared_reasons` lists distinct codes, and `cleared_count` the total;
  - `execution.unbound_activity`: one entry per (connection, platform account, instrument) change, notified grouped by the new grouping `connection_trading_day`;
  - an unbound account cannot be cleared until it is rebound and evaluated (Esquadro Q1);
  - `connection_checking` alerts at 60 s and closes the session at 5 min (Esquadro Q2, provisional);
  - `observable` is the live condition only: an active `bracket_integrity_unknown` does not block a clear once observation resumed (Crivo R1 on r5);
  - `RouteReasonCode` `routing.binding_pending` (route status only; a binding made during an open session is active from the next handshake; web hint "reconecte o plugin para ativar o vínculo"; Crivo R3 on r5);
  - a frame that fails intake 3 times is dead-lettered by the gateway and becomes `sync.command_unknown` plus an alert, with no HTTP field (r5 R2);
  - U13 / r5 §7.1 codes: `routing.adapter_protocol_outdated` (skipped, with `protocol_version`; a reduce below 1.2, or entries of an account with a drawdown rule below 1.4; exits continue) and `routing.position_unknown` (a reduce with no All Win position identity). Both notified as warning, grouped per account and trading day;
  - `RouteReasonCode` `routing.broker_disconnected` (status only): a snapshot reports `connected: false`, so delivery is held. Entries expire, exits wait, and it ends with the next `connected: true` snapshot (r5 §6.3, Crivo m2);
  - **Step-up (users Q95/Q96, 2026-10-24):** `clearAccountSync` requires `step_up` action `account_sync_clear` (target = account; Q35 set; 403 `StepUpFailed`). New `StepUpAction` values `account_sync_clear` and `trading_day_restart` (the latter used by `restartTradingDayCount` in 5.4.0); `requestStepUp` requires `target_id` for both. Root text and enum only (Esquadro's file);
  - `clearAccountSync` order: 404 → the replay check (200 without consuming a code) → step-up → 409 (Crivo R1);
  - `AccountSync.positions` (each open position with its live protection `full | partial | none | unknown`, for the confirmation). A clear never hides an unprotected position: its `sync.bracket_incomplete` is kept or raised, and the account stays out of sync (Vigia L2);
  - 5.3.0 freezes with ADR-0004 r5 `0255502a9fc8` (Q3).
- Text fixes from the 5.2.0 Code Review (Crivo m1–m5):
  - the DisplayName CHANGELOG line now says the limits are counted after trimming;
  - s7 §10 cites ADR-0004 §6 for `already_flat`;
  - the Q76 test line covers a sync state that is unavailable;
  - "no gates 6–9" is used everywhere;
  - `gates_unknown` lists only `position_conflict`, because `nothing_to_reduce` is an exit.

## 5.2.0 — 2026-10-23 (additive; Q75 b, Q76 a; aligned with ADR-0003/0004 r4)
- **Q75 (b, user; extends P1):** on an out-of-sync account the expected position is not trusted for exits. A `close` with a flat expected position is sent as a flatten (decision `routed`); `routing.nothing_to_close` never applies to an out-of-sync account. A barred `reverse` (Q74) with a flat expected position and gate `routing.out_of_sync` sends its close leg as a flatten and never creates the open: decision `capped`, entry `routing.out_of_sync`, outcome `partial`, `executed_qty` = 0, no `reverse_closed_qty`. The other bars keep "flat → skipped" (the account is in sync). Already on the reverse side stays `routing.position_conflict`. New optional `DetailsRoutingDecision.expected_flat` marks these flatten decisions; the adapter answers `command.already_flat` when flat (ADR-0004 §5, protocol unchanged).
- **Decision 4b (A, Esquadro):** under Q75 `routing.reverse_close_only` is not sent at the decision. When the flatten is terminal: filled → sent with `reason_code: routing.out_of_sync` and `closed_qty` / `closed_side` from its fills; `command.already_flat` → not sent (the grouped `routing.out_of_sync` notification covers the skipped open). `ParamsReverseCloseOnly` keeps its shape; it now names the audit entries behind each case (Esmeril P1).
- **Q75 notification (Pêndulo M-Q75, ADR-0003 r4):** every Q75 flatten with a terminal success is notified once. New code `routing.out_of_sync_flatten` (`ParamsOutOfSyncFlatten`: `action` close | reverse, `result` filled | already_flat, `closed_qty` / `closed_side` only when filled; danger, one per occurrence, target instruction) for a `close` (filled or `already_flat`) and a `reverse` with `already_flat`; a reverse that filled stays `routing.reverse_close_only`. The text says that any working orders on the contract were cancelled, manual ones included (a flatten cancels them even when the account is flat); the Core does not count them. `expected_flat` stays on the decision entry, so the audit explains a flatten sent with a flat expected position before its result arrives.
- **Q76 (a, user):** a `reduce` with a flat expected position is never sent, also when out of sync: `routing.nothing_to_reduce`. New notification code `routing.nothing_to_reduce` (`DetailsRoutingDecision`; warning, grouped per account and trading day, not reopened), sent when the account is out of sync, with the sync state read when the notification is produced. If that state is unavailable, the reduce is notified anyway (ADR-0003 r4 §4.1, Crivo R1). In sync it stays audit only. Registry 1.3.0.
- `AuditOutcome.partial` includes a reverse that runs only its close leg; `DetailsRoutingDecision` defines the decision kinds (`capped` = sent with less than requested, outcome `partial`); `skipped` lists nothing to reduce (Esmeril P2).
- `DisplayName` (root, text only; Esquadro's decision on Crivo L1): the Core trims leading and trailing white space before validating and storing, and the 1–60 length is counted on the trimmed value (Crivo R2; the schema bounds are the client-side form). A name that is blank after trimming, contains a control character (Unicode Cc) or contains a token-shaped part (`aw[kd]_…`) is refused with 400 `common.invalid_request` and the field's pointer. `minLength` / `maxLength` are unchanged; they are the client-side form, and the Core counts 1–60 on the trimmed value. There is no schema change.
- **Step-up on route enable (Q35 as recorded, S3 AC-10; PO):** with the setting on, `createRoute` with `is_enabled` true and `updateRoute` from off to on always require `step_up` `route_enable` (target = account). The QA Security M1 narrowing is removed: it covered automated sources only and skipped the code after an opt-in accepted with one, and the user never approved it. The user decided Q87 (a), 2026-10-24: no narrowing. Root `StepUpAction` text follows (Esquadro's file, text only).
- Q74 wording (PO): four bars (`out_of_sync`, `risk_blocked`, `limits_pending`, `no_bracket_profile`). `max_position_reached` (M = 0, ADR-0003 §4.2) and an open expired at routing (T1) give the same close-only outcome by existing rules; they are not Q74 bars. Texts in `DetailsRoutingDecision` and `ParamsReverseCloseOnly`.
- Two-step reverse (Esquadro's S7 rule, ADR-0003 r4): one routing entry per (instruction, account), written when the opening is decided (bounded by the TTL). New audit code `execution.command_requested` (`DetailsCommandSent`, category execution, actor system, outcome `info`, audit only, not notified) is written by routing in the commit of every exit command it creates: the close leg of a two-step reverse, `close`, `reduce`, `cancel` and a protective `modify`. A committed automatic exit is then visible while it waits, even with the gateway down, before the gateway's `execution.command_sent` (Ferro, Esquadro). Outcomes per the AuditOutcome table, unchanged.
- `routing.instrument_unavailable` (Esquadro, S7 D2): the instrument is in the catalog but has no active contract at the decision. Outcome `skipped`; notified as warning, grouped per account and trading day. An instrument outside the catalog stays `ingress.unsupported_instrument`. `DetailsRoutingDecision` gains `instrument_symbol` and `contract_month`, the resolved month: the instruction's month wins, otherwise the rollover catalog's active contract (P5).
- `DetailsRoutingDecision.gates_unknown` (optional, Esquadro S7 N1): when the TTL runs out without an answer from gates 6–9, the decision uses only the rule that does not depend on them (`position_conflict`), or else `routing.expired`. It never names a gate that did not answer. A `close` against a FLAT expected position waits for gate 6 and never expires: out of sync gives a flatten, in sync gives `nothing_to_close`, and with no answer the consumer retries. A `close` against a non-flat position is always a flatten, with no gates 6–9 (ADR-0003 r4 final, Esmeril M1).
- Notes s18 §3: the notification producer pattern. Each producer writes one outbox event per notifiable fact, in the same transaction as its audit event: type = registry code, payload = params + target + `audit_event_id`. S18 consumes them and groups them, starting from now (allowlist `FromNow`; the history is the audit, with no retroactive notices; Esquadro). This is text only.
- 5.1.1 gateway-queue exception: wording "an open leg found expired before it was sent is held until …" (Crivo nit), and the `already_flat` proof requires no later report for that account and contract (ADR-0004 r4, Vigia I2, Crivo n1).

## 5.1.1 — 2026-10-23 (text only; aligned with ADR-0003/0004 r3)
- Expiry of a reverse's open leg in the gateway queue (ADR-0004 §5, "Exception: the opening leg of a reverse"). The open is held until the flatten it depends on is terminal. It does not flag out of sync only when all of these hold: it never left the Core, the flatten succeeded with a proven flat position (`already_flat`, or the latest `position_after` = 0 with no later report), and there is no other sync reason. It is then `execution.expired` plus `routing.reverse_close_only` (`reason_code: routing.expired`), and the decision stays `routed`. Any other case flags out of sync as before. Texts in `DetailsRoutingDecision`, `ParamsReverseCloseOnly.reason_code`, s7 §9 and the registry rule.
- Registry 1.2.1, `$comment`: receivers never refuse an unknown code or params that fail their copy; they render generically and ack (ADR-0004 §9, note 1.3.2). A new severity, target.kind or category is still a protocol change.

## 5.1.0 — 2026-10-23 (additive; Esquadro on 5.0.0 and the Ferro case)
- Q74 extended: a `reverse` with an opposite position whose open leg expires **at routing** (gate `expired`: the decision sees the TTL exceeded, e.g. a late webhook) closes only. The close leg runs (no expiry, audited as late if it is). The entry carries `routing.expired`, outcome `partial`, `executed_qty` = 0 for the open and `reverse_closed_qty`. A flat account gives outcome `expired`; already on the reverse side gives `routing.position_conflict`. `ParamsReverseCloseOnly.reason_code` gains `routing.expired` (registry 1.2.0). Before 5.1.0 the Core notifies this case with `routing.expired` (`DetailsRoutingDecision`). An open leg that expires later in the gateway queue (connection down) is not a routing decision: the decision stays `routed` and ADR-0004 §5 applies as it stands (`execution.expired`, account flagged out of sync); any exception for a reverse whose close ran is Esquadro's next ADR-0004 delta. `reverse_closed_qty` / `closed_qty` are defined as abs(position) read at the decision, the quantity the close leg is sent to flatten.
- Root (Esquadro's file, text only): the `common.invalid_request` pointer rule names `/header/<name>` (e.g. `/header/Idempotency-Key`), next to `/path/` and `/query/`.
- An existing account gaining a target: filling a missing `evaluation_start` bumps `account.limits_version` and emits `account.limits_changed` (ADR-0006 §3.1).
- Notes (s10, Vigia I1): the `ingress.rejected` pointer comes from client input (JSON key names); clients render it as text, never as HTML or a link.

## 5.0.0 — 2026-10-23 (BREAKING: Q73 b, Q74 b, implementation gaps from S5/S6/S7)
- **Versioning (ADR-0002 §4, same rule as 4.0.0):** breaking, so document major; paths stay `/api/v1`. Breaking parts: `accounts.evaluation_restarted` details lose `achieved`; `createAccount` rejects evaluation fields on an account without an evaluation target (422) and may require `profile_size` for a prop account. Everything else is additive or text.
- **Q73 (b, user):** an account whose size is not a standard size, real **or prop**, copies the default profile of the nearest standard size; a tie requires `profile_size` (Q60 c). The nearest size only picks the stop/target profile: balance, limits and risk always use the account's own `size_amount`. `createAccount.profile_size`, `restoreAccountProfile`, `getNearestProfileSize` and `AccountProfile.origin` texts follow; the schema no longer forbids `profile_size` on a prop account.
- **Q74 (b, user; extends P1):** a `reverse` on an account barred for entries (`out_of_sync`, `risk_blocked`, `limits_pending`, `no_bracket_profile`), or with no room (derived M = 0, `max_position_reached`), runs only its close leg (flatten + cancel protections) and skips the open when the position is opposite; the §4.2 position check runs first (flat → skipped with the gate code; already on the reverse side → `routing.position_conflict`, never a flatten). Audit: the gate code, outcome `partial`, new `DetailsRoutingDecision.reverse_closed_qty`. A flat account: skipped with the gate code. New notification code `routing.reverse_close_only` (`ParamsReverseCloseOnly`; danger, one per occurrence, always sent including for `limits_pending`, Vigia I1; replaces the gate's grouped notification for that occurrence). Registry 1.1.0.
- `KnownAuditCode` / `AuditDetailsByCode`: `routing.platform_unbound` (ADR-0008; Crivo on Esquadro's ADR-0003 delta).
- `RouteReasonCode`: display order of `effective.reasons[]` (most severe first) and the ADR-0003 §4.1 gate order (Esquadro's S7 delta): `source_revoked` / `route_disabled` are route status only; `connection_disconnected` never skips an instruction (the command waits; `routing.expired` on TTL).
- `accounts.evaluation_restarted` → new `DetailsEvaluationRestarted` (`start`, `previous_start`, `profit_target`, optional earned-before pair; no `achieved`, which is S13b risk data). `DetailsEvaluation` stays for `risk.evaluation_target_reached`.
- `createAccount`: `evaluation_start` / `evaluation_earned_before` only for an account with an evaluation target, else 422 `accounts.evaluation_not_applicable` (pointer of the first one sent). Invariant: an account with a target always has a start; when an existing account gains a target the Core sets a missing start to the Q46 default (registration date, user's time zone).
- `getOptInRules`: `firm_platform` without `preset_version_id` → 400 `common.invalid_request`, pointer `/query/firm_platform`.
- Notes (s10, normative for implementers): `safe_to_display = true` for `ingress.*` and `routing.subscription_changed`; the `ingress.rejected` pointer is a path only, never a body value; `actor` = `system` for `ingress.*`, and for `routing.subscription_changed` `user` (`cause = user`) or `system` (`cause = account_ineligible`); S3 outcomes `info` except `sign_in_failed` / `step_up_failed` / `code_rate_limited` → `rejected`.
- Exits never expire (ADR-0003 S7 round 2, ADR-0004 T1): `expired` is an entries-only gate; an exit waits for the connection and executes late, audited as late.

## 4.3.0 — 2026-10-23 (S3 audit codes; additive)
- Audit codes `security.invitation_issued` (`invitation_id`, `masked_email`, `email_sent`), `security.invitation_revoked`, `security.account_created` (`DetailsSecurityInvitation`), category `security` (Esquadro, from the S3 implementation).
- `KnownAuditCode` states the category of each code family (e.g. `sources.*` / `connections.*` → config, `connection.status_changed` → connection, `security.*` → security).

## 4.2.1 — 2026-10-23 (text only)
- `Trade.commission`: the allocation of a fill shared by several trades (flip, FIFO partial close) follows the exact protocol 1.3 rule: pro rata by quantity, 4 decimals half away from zero, remainder to the trade allocated last. `pnl` = `gross_pnl` − `commission`.
- Manual trades: the floor used for a blank commission is the EFFECTIVE floor at save time, as derived by risk (data model 15.1); a pending lowered floor is not used early (Crivo note).
- Notes (s21): a manual trade with `commission_source: configured` is labelled "estimada" (S21 AC-7, Q71); it is derivable, with no new field.

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
