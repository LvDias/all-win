# QA plan — V1-S22 Performance (web)  · Retina · 2026-10-24 · status: PRE-WRITTEN, not run (no implementation yet)

Inputs: slice-plan S22 AC-1..5 (rev. 9), spec `v1-s22-performance.md` (main rev. 1 + Nanquim delta 231a28b8df44 (read from /tmp/allwin-review-vigia/nanquim-batch1-231a28b8df44.patch): rev. 2 net labels "P&L líquido total/médio", gross total beside, commissions notice, Q58/Q59 decided, AC-5 empty figures PENDING PO approval, Q79 archived PENDING), contract `performance.yaml` 5.1.1 (`getPerformance`) + 5.4.0 draft (`avg_gross_pnl`, to confirm by name when published).
Rule: the browser does no money/rate/duration arithmetic; every figure, count and bar length is the Core's string/integer.

## AC → evidence

| AC | Check | Level | Test (in `performance.qa.test.tsx.pending`) |
|---|---|---|---|
| AC-1 | Seven figures: Trades, P&L total, P&L médio, Taxa de acerto, Tempo médio, MAE médio, MFE médio; breakdown by Fonte / Tipo de sinal / Fonte e tipo (default Fonte e tipo, in URL `group`); groups come from the Core, one row per group, `Total` is the Core's `totals` (not a browser sum) | vitest | figures, group-by, total-row-not-summed |
| AC-2 | Query = `from`/`to` (exit time, profile TZ, `to` exclusive = next local midnight) + repeated `account_id`; same window rules as S21; `as_of` shown; footer link "Ver os trades deste filtro" carries the same query to `/trades`; each `Ver` link carries group filters (`source`/`signal`/manual/platform origins as S21) | vitest | filters-same-as-S21, links |
| AC-3 | Net is main, gross beside (`total_gross_pnl`, `avg_gross_pnl` once in 5.4.0; commission); win rate = Core's `win_rate` string with counts "N vitórias · N derrotas · N zero"; a +2.00 gross / −0.50 net trade is a LOSS in the counts the Core sends (UI shows counts as received, win rate is not recomputed); uncommissioned warning when `uncommissioned_count` > 0 | vitest | net-gross, winrate-counts, uncommissioned |
| AC-4 | Only the AC-1 list: no profit factor / best / worst / avg win / avg loss / per-day curve anywhere (Q59) | vitest | q59-negative |
| AC-5 (PENDING PO approval, delta rev. 2) | No trades: count 0 and net/gross totals "US$ 0,00" shown, averages and rates "—", sentence "Nenhum trade neste filtro."; totals show 0 (count, total P&L), every average/rate shows "—" (absent fields from the Core, never "NaN"/"0%"/"0 s"), sentence that there are no trades; first-use vs filtered empty states; filtered never offers "create" | vitest | empty-states |
| Q55 | MAE/MFE basis line "Base: k de n trades" only when `*_sample_count` < `trade_count`; none when equal; "—" when sample count 0 | vitest | basis-line |
| Q-S22-5 | Bars `aria-hidden`; exact signed amount always in text; width from `bar_ratio` string (no arithmetic); positive/negative bar carries sign in text | vitest | bars |
| Isolation (X-2) | Another user's `account_id` → Core returns empty totals; UI shows empty state, never an error/404 leak; filters keep ids as sent | vitest | other-user-account |
| States | Loading dims previous values with `aria-busy` (no stale number looks current); error with retry keeps filters; "Há novos trades desde {time}. Atualizar" on `trade.captured`, figures do NOT change until Atualizar; stale badge | vitest | states |
| Money | Core strings exact (4 decimals / 14 digits), signed text, rounding only by formatting; durations two units | vitest | exactness |
| A11y | `dl` figures; table caption (name, count, default sort), `th scope`, `Total` in `tfoot` with `th scope=row`, `aria-sort` on sortable headers; definition popovers by focus/click, Esc, `aria-describedby`; polite "Resultados atualizados: n trades" once per change | vitest + Playwright | a11y tests, e2e |
| Browser | axe (wcag2a/2aa/21aa/22aa) on page + popover open + empty state; keyboard through filters, group-by toggle, sort buttons, popovers; 320 px cards and 400 % zoom (1.4.10) no 2-D scroll, `Total` card first on mobile; reduced motion: no animated bars | Playwright | `web/tests/qa/e2e/performance.qa.spec.ts` (to write once the route exists) |
| i18n | pt-BR = en keys; no hard-coded text; `⟦missing:` marker never visible | existing gate + e2e fixtures | — |
| Cross-slice | S21 totals strip and S22 totals for the same filter/`as_of` show identical strings (mock both with the same numbers; link carries `recorded_before` = `as_of` if the Frontend uses it) | vitest | reconcile |

## Not testable by the web QA (needs Core / real data)
Worked example of the metric definitions (§3.4 "QA checks a worked example"), real reconciliation list ↔ performance, rounding half away from zero, FIFO trade definition (Q57), X-2 at the API. These belong to Core QA / full-stack E2E; I will list them as unverified in the verdict.

## Open points for the Maestro
1. RESOLVED by the PO (literal AC-3): the gross average IS drawn beside the net average, from `avg_gross_pnl` (5.4.0 draft), on totals and groups; the pending test requires it. Nanquim delta to be updated.
2. Public API assumed: `PerformancePage` exported from `@/features/performance`, route `/[locale]/performance`, request `GET /api/v1/performance`. If Frontend names differ, only the import line changes.
3. 5.4.0 field name for gross average assumed `avg_gross_pnl` (on figures and groups).
4. The test file has the extension `.pending` so neither vitest nor tsc/eslint collect it (a missing import would break `pnpm typecheck`) until the feature exists; drop the `.pending` suffix then.
