# Adapter protocol — changelog

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
