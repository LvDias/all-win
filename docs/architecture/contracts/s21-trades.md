# S21 — Trades HTTP contract notes

- **Status:** proposed (2026-10-23), for Code Review. Owner: Software Architect (Compasso). Contract version **3.3.0**, on top of 3.2.1.
- **Contract:** [`contracts/http/modules/trades.yaml`](../../../contracts/http/modules/trades.yaml), plus the `trades.*` audit codes in `modules/audit.yaml`.
- **Inputs:** slice-plan rev. 7, V1-S21 AC-1…AC-4 · design spec `v1-s21-trades.md` rev. 1 §3, §4, §8, §9 (PO answers; Q52–Q57 provisional).

## 1. Operations

| Operation | Method and path | Notes |
|---|---|---|
| `listTrades` | `GET /api/v1/trades` | Filters `from`/`to` (exit time, Q53), `account_id[]`, `source_id[]`, `origin[]`, `signal_type[]`; `sort` = `exit_time` / `pnl` / `duration` + `order`; MAC cursor |
| `getTradesSummary` | `GET /api/v1/trades/summary` | Same filters: `count`, `total_pnl` (the S22 figures) |
| `getTrade` | `GET /api/v1/trades/{trade_id}` | Detail |
| `createManualTrade` | `POST /api/v1/trades` | `Idempotency-Key` (same rules as `createAccount`) |
| `updateManualTrade` / `deleteManualTrade` | `PUT` / `DELETE /api/v1/trades/{trade_id}` | Manual trades only (Q56, provisional); `row_version` |
| `previewManualTrade` | `GET /api/v1/trade-preview` | Stateless duration and P&L |

SSE: `trade.captured` (also for manual trades), which drives the "n novos trades" banner. Instruments: `listInstruments` (S6).

## 2. Decisions

1. **Origins:** `routed` (source + signal type + instruction), `master` (S19), `manual`, `platform` ("Plataforma (sem sinal)", PO). `origin` is a filter of its own, so "Manual" and "Plataforma" are not fake sources.
2. **Numbers are the Core's or the plugin's, never the browser's** (X-3, S21 AC-2).
   - Captured trades carry the plugin's computed values.
   - Manual trades get duration (exit − entry) and P&L (exact, from prices, quantity and the instrument's tick value) from the Core. The preview uses the same computation.
   - `pnl_basis: gross` (Q54, provisional; a later `net` would be an added value).
3. **Prices must be whole ticks** (422 `trades.not_tick_multiple`). Unlike profile distances (S6), a price is a market fact and is never rounded.
4. **MAE/MFE** are in USD (Q52) and optional for manual trades (Q55). They are absent rather than zero, and S22 excludes absent values from its averages.
5. **What one trade is (Q57):** the Core decides (one per entry with FIFO exits in individual mode; flat-to-flat in shared mode) and the UI shows what it receives.
6. **Editing (Q56, provisional):** only manual trades (`editable`, `row_version`). Captured trades get 409 `trades.not_editable`. The audit keeps the before and after values, and the values of a deleted trade.
7. **Retention:** trades are kept for the life of the user account (S10 AC-3). Archived accounts stay listed with `account_deleted`.
8. **Ties and paging:** ordering ties are broken by `trade_id`. The cursor is bound to the filters and the sort.

## 3. Requests to other owners

- **Esquadro (adapter protocol):** the completed-trade message from the plugin. It carries exactly the S21 AC-1 fields with decimals as strings, an id for idempotency (a trade is never stored twice), `direction` ∈ `long` / `short`, the originating `command_id` when routed, and `mae` / `mfe` in USD. The HTTP `Trade` mirrors it.
- **Lastro:** table `trades.trade` (decimals NUMERIC, unique plugin trade id per account, `row_version` for manual rows), with indexes for exit time, P&L and duration sorting per user.

## 4. Also in 3.3.0 (Crivo note on 3.2.1)

`risk.yaml` `pending_changes[].change` gains `daily_loss_changed`. It covers a mixed change of the daily loss (stricter in one part, looser in another), which is treated as looser and carries `new_daily_loss`.
