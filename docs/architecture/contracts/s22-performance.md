# S22 — Performance HTTP contract notes

- **Status:** proposed (2026-10-23), for Code Review. Owner: Software Architect (Compasso). Contract version **3.4.0**, on top of 3.3.0 (S21).
- **Contract:** [`contracts/http/modules/performance.yaml`](../../../contracts/http/modules/performance.yaml), plus `recorded_before` / `as_of` in `modules/trades.yaml`.
- **Inputs:** slice-plan rev. 7, V1-S22 AC-1, AC-2 · design spec `v1-s22-performance.md` rev. 1 §3, §6, §7 · [S21 notes](s21-trades.md).

## 1. Operation

`getPerformance` — `GET /api/v1/performance?from=&to=&account_id=&group_by=&sort=&order=`. One call returns:
- `as_of`;
- `totals`: the figures of the whole set;
- `groups`: the same figures per group, with `bar_ratio`.

SSE: none of its own. `trade.captured` raises the "novos trades" banner; the figures refresh on user action, so they never change under the user's eyes.

## 2. Decisions

1. **Reconciliation (S22 AC-2).** The trade set is `listTrades` with the same `from` / `to` (exit time, Q53) and `account_id`, restricted to trades recorded at or before `as_of`. `listTrades` and `getTradesSummary` accept `recorded_before`, and `getTradesSummary` returns `as_of`. The "Ver os trades deste filtro" link passes `recorded_before = as_of`, so both screens show the same trades and totals. Only a manual edit or delete after `as_of` can make them differ; it is audited and raises the banner.
2. **Figures (Q-S22-3):**
   - totals only for P&L and the trade count; time, MAE and MFE as averages only;
   - win / loss / breakeven counts;
   - `win_rate` = wins ÷ all trades, where net P&L ≤ 0 is not a win (Q58 a; net from 3.5.0);
   - MAE / MFE averages over the trades that have the value, with `*_sample_count` (Q55);
   - averages absent (`—`) when nothing enters them.
3. **Rounding** (decided here, since no business rule exists): half away from zero, so negative averages are symmetric with positive ones (−12.345 → −12.35). Money averages to 2 decimals; `win_rate` to one decimal, always with one decimal, e.g. `"62.5"` (Q-S22-2); the duration average to whole seconds; `bar_ratio` to 4 decimals. Totals are exact sums.
4. **Groups:**
   - `group_by` ∈ `source_signal` (default), `source`, `signal_type`;
   - `manual` and `platform` trades are their own groups (Q-S22-7); a revoked source keeps its name (`source_revoked`);
   - only groups with at least one trade are returned;
   - sorted by `total_pnl` desc by default;
   - the `Total` row is `totals`, not a browser sum.
5. **Bars:** `bar_ratio` = |group P&L| ÷ the largest |group P&L|, computed by the Core. The sign comes from `total_pnl`. When the largest |group P&L| is 0, every group's `bar_ratio` is `0.0000` (no division by zero).
6. **Isolation (X-2, QA Security on 3.4.0):** a foreign or unknown `account_id` matches nothing (empty totals and groups, never 404), as in `listTrades` / `listActivity`. Required test: two users, each filtering by the other's account id, get `trade_count` 0 and no groups.
7. **Not in V1:** extra metrics (Q59: profit factor, best/worst trade, average win/loss, per-day curve) until the user decides.

## 3. 3.5.0 — net figures (S22 AC-3 rev. 8)

Every P&L figure is net. A win is a trade with net P&L > 0, so a trade of +2.00 gross with 2.50 commission is a loss. Totals add `total_gross_pnl`, `total_commission` and `uncommissioned_count`.
