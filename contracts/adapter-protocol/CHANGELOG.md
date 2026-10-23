# Adapter protocol — changelog

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
