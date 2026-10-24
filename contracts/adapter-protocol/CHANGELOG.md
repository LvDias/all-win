# Adapter protocol — changelog

## 1.4.2 — 2026-10-24 (documentation only; no schema shape change, no version bump on the wire)
- A `limits` frame that carries a new `day_reanchor` version also bumps that account's limits `version`, so a new re-anchor never travels under a version the plugin already saw (Pêndulo n3, decided by Esquadro).
- The plugin still evaluates `day_reanchor.version` on its own, independent of the limits version (Elo; test T7j).

## 1.4.1 — 2026-10-24 (documentation only; no schema shape change, no version bump on the wire)
- `limits` latest-wins: an unacked frame is dropped only when ALL its accounts have a newer version; otherwise it is rebuilt with the still-newest items (Pêndulo n1).
- A pending `day_reanchor` is carried by every newer limits frame of the account (Crivo p1, Pêndulo n2).
- The plugin must not require `high_water_above_start` to enforce; `room_to_floor` suffices (Crivo p2).

## 1.4 — 2026-10-24 (additive minor; Compasso; aligned with ADR-0004 r5 0255502a9fc8 §7.1/§9; Esquadro approves the root)
- **Drawdown on distances (S13 equity referential b; Esquadro's FINAL).**
  - `limits.drawdown` gains `room_to_floor`, `high_water_above_start` and `lock_above_start`, required from 1.4 (`room_to_floor`) and anchored on the Core's realized E_start.
  - `trading_day.current_start_at` (required from 1.4) is the boundary the plugin anchors `P_reset` (CashValue) on.
  - The absolute floor / high water / lock values are display only, and `balance_adjustment` is deprecated (still sent on 1.3).
  - The rules are in the README: the day formula, the settling interval and the T9 correction, rebuild OFF until K15/E4, and the H1 semantics of a position held across the boundary.
- **Unknown day:**
  - `risk_state.day_state` (`known` | `unknown`, required from 1.4) and `day_unknown_reason`;
  - `risk_event` kinds `day_unknown` / `day_known` (reliable transitions);
  - reason code `risk.day_unknown` (entries refused, exits pass);
  - local risk-block codes are NOT a sync trigger (gateway; the Core treats them as a risk block, gate 7).
- **Re-anchor (Q92 c, user):** `limits.accounts[].day_reanchor` {own `version`, `trading_day_start_at`, `day_realized_net_known`, watermark `last_fill_id`, `requested_at`}.
  - It is sent only after the user's explicit "recomeçar a contagem de hoje", audited by the HTTP operation of S13/S15.
  - The plugin applies it only when no fill was reported after the watermark, restarts Σgap_today, and confirms with `risk_state.reanchor_applied_version`.
  - The Core re-issues it after each fill while it is pending.
- **Sessions below 1.4:** the Core sends no `drawdown` and alerts "atualize o plugin". Entries of an account with a drawdown rule are skipped with `routing.adapter_protocol_outdated` and exits continue (r5 §7.1). Roll out plugin 1.4 before the Core.
- `sync_reasons` (welcome, account_state): maxItems 20 → 32 (21 `sync.*` codes with `flatten_failed_unprotected`, ADR-0004 r5; Crivo f1). Envelopes below 1.4 keep maxItems 20 (schema-gated), and the Core sends them at most 20, most severe first (r5 §9).
- `position_snapshot.taken_at` (1.4+): stamped with the skew-corrected Core clock. The Core ends the reconnect window with the snapshot whose msg_id is first received in the session, with taken_at as a second check (r5 §6.3, Pêndulo N1).
- Pêndulo review: W1, the exact watermark rule (the last fill reported this day per the CURRENT journal equals `last_fill_id`, or no fill reported this day; T7e/T7g); L1, limits is latest-wins and a 1.4 limits is never retransmitted to a session below 1.4; L2, `late_anchor` also covers a plugin running without a broker at the boundary.
- Q93 (a), user: the unrealized P&L of a position held across the boundary counts in full in the day it is measured.
- `position_snapshot.trigger` documents the adapter's obligations (the plugin already meets them): reconnect after every welcome, change after every connection change of a mapped account, periodic every 60 s. Any entry with `connected: true` resumes observation, and `connected: false` entries are not compared (r5 §6). Text only.
- All 1.4 fields are schema-gated to 1.4+ envelopes.
- `room_to_floor` and `high_water_above_start` are SIGNED money (Malha; Esquadro r5). A negative `room_to_floor` = already breached at the day start, so the plugin breaches at day = 0 and entries are blocked from the start. A negative `high_water_above_start` is possible with realized anchors, because the high water includes unrealized peaks.
- Fixtures: 6 valid (limits with distances, already breached, with re-anchor, risk_state known/unknown, account_state with 21 reasons) and 9 invalid (incl. 21 reasons on 1.3).
- Owners to notify: Plugin (Elo, Pêndulo: probes, the 52k/50k case), Backend (Malha: E_start and distances; Bigorna: gateway rules), Esquadro (ADR-0004 r5, root).

## 1.3.1 — 2026-10-23 (documentation only; no schema shape change, no version bump on the wire)
- `trade_completed.commission`: allocation order = consumption order (closing quantities first, FIFO; the flip remainder last); an emitted trade keeps its rounded share (Pêndulo LA1).
- `limits.drawdown.balance_adjustment`: the adapter journals the day's Σ(effective − reported), reset at the trading-day boundary; without it, it alerts and uses the Core's value at the next `limits`, never a smaller one (Pêndulo LA2).
- Fixture `valid/trade-completed-flip-opening-part.json`: the trade opened by the flip remainder (exec-2002, 1 contract, commission share 0.37 + the closing fill's 0.37 = 0.74; Crivo).

## 1.3 — 2026-10-23 (additive; Proposed until Code Review) — plugin panels G1–G8, net P&L (U20), evaluation target (Q46), trades (S21), user disconnect
- **Notifications (S18, G1):** C→A `notifications` (snapshot after `welcome`, ≤ 200 items + authoritative `unread_count` / `worst_unread_severity`; latest wins, not acked), C→A `notification` (reliable upsert by `notification_id` + monotonic `revision`; a reopen is a higher revision without `read_at`), A→C `notification_read` (reliable). Items mirror the HTTP `Notification`; codes and `params` schemas come from `contracts/notifications/v1/registry.json`; severities `info | success | warning | danger | critical`; `source_event_ids` (≤ 50) lets the plugin replace its local not-yet-synced copies; `target {kind, id}`, never a URL.
- **`welcome` (G2–G4, G6):** `connection_name`, `time_zone` (IANA), `app_url` (https origin only), `accounts[].display_name`. Renames apply at the next `welcome`. G5 = notifications; G7 documented in ADR-0004 §7; G8 is a build constant (close 4003 precedes `welcome`).
- **Net P&L (U20, user decision 2026-10-23):** `fill.commission` semantics fixed (≥ 0; total platform charges of the execution). It stays **optional**: absent = not reported, present 0 = the platform reported zero (Crivo F1). Per fill the Core derives `commission_source` (platform / configured / none). `limits.accounts[].commission_per_contract_side` = the configured floor (prefilled by the preset, editable); effective commission of a fill = max(platform, floor × quantity). `risk_state.realized_pnl` is **net** from 1.3, plus required `commissions_today` and `commission_source` (`platform | configured | mixed | none`; `none` = net equals gross, the plugin warns). Risk meters and local enforcement (S13/S15) use net.
- **Drawdown on net (Pêndulo M9):** `limits.drawdown.balance_adjustment` = Σ(effective − reported commission) up to the previous trading-day close (Core). The plugin's adjusted equity = platform balance − balance_adjustment − the day's adjustment − expected closing commission, compared with the adjusted floor and high water.
- **Evaluation target (Q46):** `limits.accounts[].evaluation {profit_target, baseline_net_pnl, start_date, as_of_trading_day}`. The Core computes `baseline_net_pnl` from the adjusted net balance (baseline = the adjusted balance at the start of `start_date`'s trading day, else the account size), and it already includes `evaluation_earned_before`; `start_date` defaults to the account registration. The plugin adds the trading day's net. Reaching it only notifies.
- **Trades (S21):** A→C `trade_completed` (reliable): plugin-computed numbers (S21 AC-2), plugin `trade_id` (idempotent), `net_pnl` (main), `gross_pnl`, `commission`, `mae`/`mfe` (absent, never partial, when the interval was not fully observed), the entry's `command_id`, `opening_fill {fill_id, quantity}` + `closing_fills [{fill_id, quantity}]` for the Core's reconciliation; a fill shared by several trades (flip, FIFO partial close) has its effective commission allocated pro rata by quantity (4 decimals, half away from zero, remainder to the trade allocated last) (Crivo F1) (`trades.reconcile_mismatch` audited, never silently corrected).
- **User disconnect:** WebSocket close code **4100 `user_disconnect`** (plugin "Desconectar": the device token is kept, no automatic reconnect). Core: `sync.disconnected_by_user` (ADR-0004 §6) when a mapped account had an open position or a command in flight; otherwise status + audit only. HTTP `Connection.disconnect_reason` (`user | network | outdated | unpaired`).
- All 1.3 fields and messages are schema-gated to 1.3+ envelopes. Fixtures for welcome, risk_state, limits, fill commission (reported, zero, not reported, negative), notifications and trade_completed.
- **Scheduled for the S15 protocol delta (Crivo N3, pre-existing):** `limits.drawdown.type` `trailing_end_of_day` → align with HTTP/data model `trailing_eod`; a `daily_loss` kind (hard / soft_pause / none) in `limits` for local enforcement; `warning_threshold_pct` fixed at 80 (D18).
- Owners to notify: Plugin Engineer (all), Backend Rust (gateway, risk, trades, notifications), Database Architect, Compasso (HTTP Notification/Trade/Connection alignment), Web (catalog keys).

## 1.2 registry addendum — 2026-10-23 (documentation only; no schema change, no version bump)
- `reason-codes.md`: new code `order.live_consent_cancelled` (`cancelled`, `entry`/`exit`, terminal, failure for `depends_on`): the plugin cancelled on its own a working All Win order on a live account without consent (ADR-0005 §N11 row 2). Valid on every version (`reason_code` is the generic `$defs/code`). Web and plugin catalogs need the key in pt-BR and en.

## 1.2 — 2026-10-23 (additive; Proposed until Code Review) — U13 position-identity guard
- `command.refs.position_ref` (C→A): the `command_id` of the All Win `place` (open) command whose fill took the account's position in that instrument from flat to non-flat. Allowed only on `place` with `position_effect: reduce`, and **required** there from 1.2; forbidden on 1.0/1.1 envelopes (schema-enforced).
- The adapter executes a reduce only if its current position has that identity; otherwise `rejected` + new reason code `command.position_changed`, nothing submitted. The Core sets `out_of_sync` (`sync.position_changed`), alerts, audits and waits (alert and wait). `flatten` is unchanged: no guard, no expiry (U13 option A, user decision 2026-10-23).
- Same-side positions now get `command.position_changed` (a different position), not `command.nothing_to_reduce`; a flat account keeps `command.nothing_to_reduce`.
- The Core never sends a `reduce` on a session that negotiated < 1.2 (routing skips it with `routing.adapter_protocol_outdated`, audited, alert). A 1.2 adapter rejects a reduce in a 1.0/1.1 envelope (older Core) with `command.position_changed`. A reduce in a 1.2 envelope without `refs` is schema-invalid: `protocol.invalid_message`, not acked, never executed (a Core bug). A 1.2 reduce retransmitted to a 1.1 adapter after a rollback fails its schema and stays unacked: intended, and `refs` must never be stripped.
- **Upgrade note (Plugin, QA):** a position opened before the plugin ran 1.2 has no identity in the journal. It is unknown, so every reduce on it gets `command.position_changed` → `out_of_sync` until it is flat. This is acceptable before any live use.
- **Behaviour change (PO, S7):** on TradingView-only destinations (T3, audit-only for external fills), a reduce on an externally opened position used to execute. It is now skipped with `routing.position_unknown` (U13 A: an external identity never matches).
- Fixtures: `valid/command-place-reduce-position-ref.json`, `valid/execution-report-position-changed.json`, `invalid/reduce-without-position-ref-on-1-2.json`, `invalid/position-ref-on-protocol-1-1.json`, `invalid/position-ref-on-open-place.json`, `invalid/position-ref-on-flatten.json`.
- Owners to notify: Backend Rust (gateway, routing, expected state, adapter-sim), Plugin Engineer (position identity in the journal), Database Architect (opening command per position in the expected state), Web (new codes in both catalogs).

## 1.1 — 2026-10-23 (additive; Proposed until Code Review)
- New C→A message `account_state` {account_id, platform_account, sync_state, sync_reasons (full list), changed_at}: mid-session sync-state changes for the plugin (S11 AC-1). Latest-value-wins, not acked. Valid only with `protocol_version` ≥ 1.1; the Core sends it only on sessions that negotiated 1.1. 1.0 adapters are unaffected.
- `reason-codes.md`: registry of adapter-emitted reason codes and report-attribution rules (V1-S2 clarifications, ADR-0004 §5). Documentation only; it applies to 1.0 as well.
- `account_state`: `in_sync` ⇔ empty `sync_reasons`; ties on `changed_at` go to the later-received message.
- New reason code `command.stale` and Core retransmission horizon (72 h, `sync.command_undelivered`), ADR-0004 §4 "Bounded re-delivery" (Code Review 4 H1). Retransmissions are byte-identical (same `msg_id` and `sent_at`).
- Fixtures: `valid/account-state-out-of-sync.json`, `valid/account-state-cleared.json`, `invalid/account-state-on-protocol-1-0.json`, `invalid/account-state-without-sync-reasons.json`, `invalid/account-state-out-of-sync-without-reasons.json`.
- Owners to notify: Backend Rust (gateway, adapter-sim), Plugin Engineer.

## 1.0 — 2026-10-23 (Proposed, ADR-0004)
- Initial version: hello/welcome handshake, ack/heartbeat, command (place/modify/cancel/flatten, expires_at, depends_on, protection in ticks), limits, instruction_result, execution_report, position_snapshot, instruction (embeds instruction v1), risk_event, risk_state.

## 1.0 — accepted 2026-10-23, with pre-implementation deltas (no implementation existed yet)
- `$defs/money` / `non_negative_money` (≤ 4 fraction digits) for commission, limits, risk_event value/limit and risk_state (ALIGN-1).
- `hello.accounts[].oco_location` (`local` | `server` | `unknown`), required (ADR-0004 §6.1, NT8 profile F5).
- `welcome.accounts[].sync_reasons` (stable codes, e.g. `sync.bracket_integrity_unknown`).
- `execution_report.status` adds `unknown`; `position_snapshot.working_orders[].state` (`accepted` | `working` | `trigger_pending` | `unknown`).
- `command.flatten.reason` adds `orphan_fill` (used only if the user chooses Q-O1 option b).
- Owners to notify: Backend Rust (gateway, adapter-sim), Plugin Engineer, Database Architect.
- `execution_report.fill.operation` (`add` | `amend` | `remove`): NT8 can amend or remove executions (profile §6).
- `position_snapshot.trigger` adds `account_reset` (Sim reset / Playback rewind = state discontinuity → `sync.simulation_reset`).
