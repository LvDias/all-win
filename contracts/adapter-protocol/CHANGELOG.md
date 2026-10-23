# Adapter protocol — changelog

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
