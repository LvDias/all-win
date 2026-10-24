# Adapter protocol v1 (Core ↔ connected-client adapters)

WebSocket protocol between the Core and adapters that run inside trading platforms (first: NinjaTrader 8 AddOn). Decision record and normative semantics (delivery, ordering, expiry, reconnect, out-of-sync, pairing): [ADR-0004](../../../docs/architecture/adr/0004-adapter-protocol-v1.md). Owner: Software Architect.

- `messages.schema.json`: one JSON object per text frame; envelope + payload by `type`. The `instruction` payload embeds `../../instruction/v1/instruction.schema.json` by `$id`, so validators must load both schemas.
- `reason-codes.md`: registry of adapter-emitted reason codes (status, terminal, `depends_on` success) and report attribution rules.
- `examples/valid/*.json`, `examples/invalid/*.json`: conformance fixtures for Rust (`core/crates/contracts`, `core/tools/adapter-sim`) and C# (`AllWin.Adapter.Core`). Invalid file names state the reason.
- Self-check: `python3 contracts/scripts/validate_fixtures.py contracts -v`.

Decimals are strings; quantities are integers; timestamps are UTC with `Z`; ids are lowercase UUIDs. Tokens never appear in messages (the device token is only in the upgrade request header).

## Drawdown on distances (1.4)

ADR-0004 r5; Esquadro, 2026-10-24. The anchors are realized-only. The Core and the plugin both measure the day from the trading-day start, each from data it can vouch for, so the offset between the platform balance and the All Win equity cancels out.

- **Core.** `E_start` = size_amount + the effective net REALIZED P&L up to `trading_day.current_start_at`, from the fills the Core applied. It sends `room_to_floor`, `high_water_above_start` and `lock_above_start` from `E_start`. The absolute `floor_balance` / `high_water_balance` / `lock_at_balance` are for display only.
- **Plugin.** `P_reset` = the platform `CashValue` at `current_start_at`. The plugin never computes the boundary itself; in Playback it follows the Playback clock (R5).
  - `day` = (CashValue_now − P_reset) − Σgap_today − expected closing commission + unrealized_now.
  - `peak` = max(`high_water_above_start`, max(day) so far) for trailing intraday.
  - `floor` = −`room_to_floor` for static and trailing EOD, or max(−`room_to_floor`, peak − amount) for trailing intraday.
  - If `lock_above_start` is present, `floor` = min(floor, `lock_above_start`).
  - Breach when `day` ≤ `floor`. A negative `room_to_floor` means already breached: the floor is above 0 and entries are blocked from the start of the day. `high_water_above_start` may be negative, because the high water includes unrealized peaks while E_start is realized only.

  E_start + day is the exact current equity. The unrealized P&L of a position held across the boundary counts in full in the day it is measured: open at +300 at the boundary and closed at +500 gives day = +500 (H1). The same day value feeds the daily loss and the daily profit target (user decision Q93 a, 2026-10-24).
- **Settling the boundary.** The plugin reads CashValue for `P_reset` after a settling interval past the boundary (`max_bracket_delay`, default 10 s, tuned by spike E4), so that fills with platform_time ≤ boundary are processed first. During that interval the previous day's limits apply, with no early unlock.
  - A fill with platform_time ≤ boundary that arrives after the read corrects `P_reset` by that fill's realized net. The correction is journaled and alerted if it is outside the interval (T9).
  - A fill with platform_time > boundary belongs to the new day.
- **Unknown day (fail closed).** The plugin may rebuild `P_reset` from the day's executions only with continuous observation since the boundary, proven in the journal. Rebuilding from `account.Executions` stays OFF until K15/E4 proves completeness for the provider. When it rebuilds, it uses the realized net per execution (FIFO pairing plus commissions, including the late commission E14), not a sum of fills.
  - Otherwise `risk_state.day_state` = `unknown` (with `risk_event` `day_unknown`). Local enforcement refuses entries with `risk.day_unknown`, and exits continue.
  - A Playback rewind or `account_reset` makes the day unknown. The plugin never re-anchors silently.
- **Re-anchor (Q92 c).** It happens only after the user confirms "recomeçar a contagem de hoje" in the web, audited. The Core sends `limits.accounts[].day_reanchor` (own version, `day_realized_net_known`, watermark `last_fill_id`).
  - The plugin applies `P_reset` = CashValue_now − day_realized_net_known and restarts Σgap_today at 0 if and only if either:
    - the last fill of this account that it reported in this trading day, per its CURRENT journal, equals `last_fill_id`; or
    - its current journal has no fill reported this trading day.
  - Otherwise it waits for the Core's re-issued version (T7d, T7e, T7g). After a journal_lost this still allows the re-anchor. Fills never reported are the unseen losses the user accepts (Q92 c).
  - An open position is fine: its unrealized P&L enters through unrealized_now (T7a).
  - The plugin confirms with `risk_state.reanchor_applied_version`.
- **limits is latest-wins (L1, n1).** An unacked limits frame is dropped only when every account in it has a newer version; otherwise the Core rebuilds it with the items that are still the newest. A pending `day_reanchor` rides on every newer limits frame of its account (p1/n2). A frame that carries a new `day_reanchor` version also carries a new limits `version` for the account; the plugin still checks `day_reanchor.version` on its own (1.4.2; Pêndulo n3, T7j). The plugin enforces with `room_to_floor` alone; `high_water_above_start` is for display and tests (p2). A 1.4 limits is never retransmitted to a session below 1.4: a new frame without drawdown is built instead.
- **Sessions below 1.4.** The Core sends NO `drawdown` in `limits`, because the absolute referential is wrong, and it alerts "atualize o plugin". Without the local barrier the Core fails closed: while an account with a drawdown rule is served by a session below 1.4, routing skips its ENTRIES with `routing.adapter_protocol_outdated`, and exits continue. The Core's own risk gate still applies. The gateway (Bigorna) enforces this. Rollout order: plugin 1.4 first, then the Core release (Crivo m3).
- **Known limitation.** A deposit or withdrawal during the day moves `day` (as with the Q89 note). A Sim reset is S11 AC-6.
- **Tests (Pêndulo probes, Elo unit tests):** T1–T9, H1, T7a, T7b, T7d, T7j (a new `day_reanchor` version under a limits version that is not newer is still applied), plus the 52k/50k case.
