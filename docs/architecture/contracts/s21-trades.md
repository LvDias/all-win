# S21 — Trades HTTP contract notes

- **Status:** proposed (2026-10-23), for Code Review. Owner: Software Architect (Compasso). Contract version **3.3.0**, on top of 3.2.1.
- **Contract:** [`contracts/http/modules/trades.yaml`](../../../contracts/http/modules/trades.yaml), plus the `trades.*` audit codes in `modules/audit.yaml`.
- **Inputs:** slice-plan rev. 7, V1-S21 AC-1…AC-4 · design spec `v1-s21-trades.md` rev. 1 §3, §4, §8, §9 (PO answers; Q52–Q57 decided 2026-10-23e; Q54 overridden by the user: net P&L is the main value, 3.5.0).

## 1. Operations

| Operation | Method and path | Notes |
|---|---|---|
| `listTrades` | `GET /api/v1/trades` | Filters `from`/`to` (exit time, Q53), `account_id[]`, `source_id[]`, `origin[]`, `signal_type[]`; `sort` = `exit_time` / `pnl` / `duration` + `order`; MAC cursor |
| `getTradesSummary` | `GET /api/v1/trades/summary` | Same filters: `count`, `total_pnl` (the S22 figures) |
| `getTrade` | `GET /api/v1/trades/{trade_id}` | Detail |
| `createManualTrade` | `POST /api/v1/trades` | `Idempotency-Key` (same rules as `createAccount`) |
| `updateManualTrade` / `deleteManualTrade` | `PUT` / `DELETE /api/v1/trades/{trade_id}` | Manual trades only (Q56); `row_version` |
| `previewManualTrade` | `GET /api/v1/trade-preview` | Stateless duration and P&L |

SSE: `trade.captured` (also for manual trades), which drives the "n novos trades" banner. Instruments: `listInstruments` (S6).

## 2. Decisions

1. **Origins:** `routed` (source + signal type + instruction), `master` (S19), `manual`, `platform` ("Plataforma (sem sinal)", PO). `origin` is a filter of its own, so "Manual" and "Plataforma" are not fake sources.
2. **Numbers are the Core's or the plugin's, never the browser's** (X-3, S21 AC-2).
   - Captured trades carry the plugin's computed values.
   - Manual trades get duration (exit − entry) and P&L (exact, from prices, quantity and the instrument's tick value) from the Core. The preview uses the same computation.
   - `pnl_basis: gross` (Q54; a later `net` would be an added value).
3. **Prices must be whole ticks** (422 `trades.not_tick_multiple`). Unlike profile distances (S6), a price is a market fact and is never rounded.
4. **MAE/MFE** are in USD (Q52) and optional for manual trades (Q55). They are absent rather than zero, and S22 excludes absent values from its averages.
5. **What one trade is (Q57):** the Core decides (one per entry with FIFO exits in individual mode; flat-to-flat in shared mode) and the UI shows what it receives.
6. **Editing (Q56):** only manual trades (`editable`, `row_version`). Captured trades get 409 `trades.not_editable`. The audit keeps the before and after values, and the values of a deleted trade.
7. **Retention:** trades are kept for the life of the user account (S10 AC-3). Archived accounts stay listed with `account_deleted`.
8. **Ties and paging:** ordering ties are broken by `trade_id`. The cursor is bound to the filters and the sort.

## 3. Requests to other owners

- **Esquadro (adapter protocol 1.3, agreed):** A→C `trade_completed`, reliable and acked.
  - It carries a plugin-generated `trade_id` (idempotent, deduped like `event_id`), the S21 AC-1 fields as decimal strings, `direction` long / short, gross `pnl`, `mae` / `mfe` in USD (optional, and absent, never partial, when the plugin did not observe the whole interval), the entry's `command_id` when routed, and `opening_fill_id` + `closing_fill_ids[]`.
  - The Core reconciles them with the fills it knows. A difference is kept with the plugin's numbers (S21 AC-2) and audited as `trades.reconcile_mismatch`; it is never corrected silently.
  - HTTP exposes `reconcile_mismatch` (3.4.0). There is no `excursion_complete`: a partial excursion is sent as absent.
- **Lastro:** table `trades.trade` (decimals NUMERIC, unique plugin trade id per account, `row_version` for manual rows), with indexes for exit time, P&L and duration sorting per user.

## 4. 3.4.0 additions

- `Trade.reconcile_mismatch`, audit code `trades.reconcile_mismatch`.
- `recorded_before` on `listTrades` / `getTradesSummary` and `as_of` on the summary (S22 reconciliation).
- **Manual trades and risk (PO rule: S13 AC-2 + S21 AC-3, PO clarification, slice-plan rev. 7; QA Security I1):** manual trades do NOT feed daily loss, drawdown, the profit target, consistency or blocks (S13/S15), and never unblock an account. They feed only the trade list and the metrics (S21/S22). Risk derives only from what the platform reports, so neither a made-up profitable trade nor a typing error can change the day's `used` or block a real account.

## 5. Also in 3.3.0 (Crivo note on 3.2.1)

`risk.yaml` `pending_changes[].change` gains `daily_loss_changed`. It covers a mixed change of the daily loss (stricter in one part, looser in another), which is treated as looser and carries `new_daily_loss`.

## 6. 3.5.0 — net P&L as the main value (user decision 2026-10-23; S21 AC-1 rev. 8)

- `pnl` is NET (after commissions) and is what lists sort and sum. `gross_pnl`, `commission` (total of the trade's fills) and `commission_source` sit alongside. `pnl_basis` stays on the wire, always `net`, so 3.5.0 is additive for 3.4 readers (the field's shape is unchanged; a value was added to its enum).
- Effective commission of a fill = max(platform-reported, the account's `commission_per_contract_side` × quantity) (U20 a). `commission_source` ∈ `platform` / `configured` / `mixed` / `manual` / `none`. `none` means net = gross, with the label "comissões não informadas".
- Manual trades: `commission` is optional. When it is absent, the Core applies floor × quantity × 2 sides (`configured`), or 0 (`none`).
- `TradesSummary`: `total_pnl` (net), `total_gross_pnl`, `total_commission`, `uncommissioned_count`.
