# V1-S22 — Performance metrics view — Screen spec

> **Owner:** Product Designer (Traço @ Dev3) · **Status:** proposed, pending PO review · **Revision:** 1 · **Date:** 2026-10-23
> **Slice:** [V1-S22](../../product/release-1/slice-plan.md) AC-1, AC-2 (slice-plan rev. 7); vision §10. Consumes the data of S21; filters and formats are shared with it.
> **Foundations and components:** [foundations.md](../foundations.md) (F§6 numbers and time, F§7 status, F§10 feedback, F§4 "charts get their palette in this slice"), [components.md](../components.md) (C§3 `EmptyState`, C§4 `DataTable`). **Related:** [S21](v1-s21-trades.md) (trade list, filters, `—` for missing MAE/MFE), [S10](v1-s10-audit-log.md).
> **Contract:** no HTTP contract for metrics yet. §8 lists what the screen needs. Operation names are **design proposals**; the contract wins.
> **Surfaces:** Web (Frontend). Every metric is computed by the Core from stored trades (the plugin computed MAE/MFE/P&L per trade, S21 AC-2). QA owns the checks.
> **Scope note (slice plan):** the metric set is an *assumption refined with the Designer*; **changes beyond the list in AC-1 go to the user**. This spec stays inside AC-1. Extra metrics someone may want (profit factor, best/worst trade, average win and loss, per-day curve) are **not drawn** until the user decides **Q59**.

---

## 1. What this spec covers

| # | Screen / element | Route |
|---|---|---|
| S22-01 | Performance page: filters, summary figures, breakdown by source and signal type | `/[locale]/performance` (sidebar `Desempenho`, after `Trades`) |
| S22-02 | Metric definitions (popovers) | inside S22-01 |

---

## 2. Concepts shown to the user (no new rules)

- For a **selected period** and **set of accounts** the user sees **totals and per-trade averages** of P&L, win rate, time in trade, MAE and MFE, **split by source and signal type** (AC-1).
- The numbers **reconcile exactly** with the trade list (AC-2, X-3): the same filter gives the same trades and the same totals as S21. The page proves it by linking to the list with the same filters and by showing the instant the data is valid for.
- No price data is stored or charted (S21 AC-2). The page shows computed numbers only.
- The browser does no arithmetic on money, durations or rates. Every figure, including bar proportions, comes from the Core as exact strings or integers (F§6.1).

---

## 3. S22-01 Performance page — `/[locale]/performance`

**Purpose:** understand how trading is going, by source and signal type (AC-1).
**Permissions:** any signed-in user, own data only (X-2).

**Layout:** `PageHeader` "Desempenho" with the line "Dados até {date-time, zone}" (`as_of`, `<time>`) · filter bar · summary figures · breakdown · footer link `Ver os trades deste filtro` → `/trades` with the same query.

### 3.1 Filters

| Filter | Component | Values |
|---|---|---|
| Período / Period | chips `Hoje`, `7 dias`, `30 dias`, `Personalizado` (two date pickers, zone shown) | default `30 dias`; the period filters by the trade's **exit time**, in the display zone, exactly as S21 |
| Contas / Accounts | multi-select `Combobox`, all accounts selected by default, includes deleted accounts `(excluída)` | the "set of accounts" of AC-1 |

State lives in the URL (`?period=…&account=…&group=…`) and is carried to S21. Filters apply as they change (debounced); below `md` they open in a `Sheet` with `Aplicar`/`Limpar`. An `aria-live="polite"` region announces "Resultados atualizados: {n} trades" after each change.

### 3.2 Summary figures

A `dl` of seven figure `Card`s (two columns on mobile, four ≥ `lg`), all from the Core for the filtered set:

| Figure | pt-BR / en | Format | Note |
|---|---|---|---|
| Trades | Trades | integer | |
| P&L total | Total P&L | signed money (F§6.2) with `--pnl-*` token **and** sign | |
| P&L médio por trade | Avg P&L per trade | signed money | |
| Taxa de acerto | Win rate | percent with one decimal as sent by the Core, plus "{wins} vitórias · {losses} derrotas · {zero} zero" (counts from the Core) | definition in §3.4 |
| Tempo médio na operação | Avg time in trade | duration, two units (`12 min 30 s`) | |
| MAE médio | Avg MAE | money | basis line when not all trades have it, see below |
| MFE médio | Avg MFE | money | same |

- **Basis line** under MAE/MFE averages when some trades lack the value (manual trades may omit them, S21 §4.4): "Base: {k} de {n} trades" with a `Info` popover "Trades sem este valor não entram na média." When all have it, no line.
- Each figure has an `Info` icon-button (name "Como é calculado: {figure}") that opens the definition (§3.4).
- A figure with no data shows `—`, never `0`.

### 3.3 Breakdown by source and signal type

Control above the table: `ToggleGroup` "Agrupar por" with three options: `Fonte` / `Source`, `Tipo de sinal` / `Signal type`, `Fonte e tipo` / `Source and type`. **Default `Fonte e tipo`** (shows both splits at once; the other two collapse it). The choice is in the URL.

`DataTable` (C§4), one row per group that has at least one trade, plus a `Total` footer row:

| Column | Content |
|---|---|
| Grupo / Group | `Estratégia Quântico · Gatilhos`; `Manual`; `Plataforma (sem sinal)`; `Replicação · {master}` (S19, V1.1 only if present); labels as in S21 §3.4. A revoked or deleted source keeps its name with `(revogada)`; signal type for groups without one is `—` |
| Trades | integer; link `Ver` → S21 filtered to this group |
| P&L total | signed money + proportion bar (below) |
| P&L médio | signed money |
| Taxa de acerto | percent |
| Tempo médio | duration |
| MAE médio / MFE médio | money, `—` when none, basis in the cell's title when partial |

- **Total row** equals the summary figures and, by construction, the S21 totals strip for the same filter (AC-2). It is a data row from the Core, not the sum of the visible rows computed in the browser.
- **Proportion bars (PO: ok, provided the exact number is always visible and color is not the only gain/loss signal):** in the P&L total cell a thin bar whose length is the group's `|P&L total|` relative to the largest group, using `--pnl-positive` / `--pnl-negative`; the exact length is a Core value (`bar_ratio`); the bar is `aria-hidden` because the signed amount in text already carries everything. No categorical palette is needed in V1; the P&L bars need only the two P&L tokens, so no chart palette is requested from Aquarela now (F§4); a later chart slice asks for it.
- Sort: default P&L total, highest first (stated in the caption). Sortable headers: `Trades`, `P&L total`, `P&L médio`, `Taxa de acerto` (`button`, `aria-sort`). Groups never exceed a few dozen, so no paging.
- Below `md`: each group is a `Card` (group name as title, P&L total top-right, then a definition list in the same order); the `Total` card comes first.

### 3.4 Definitions (popovers; text from the PO's AC-1, no invented formulas)

| Figure | Text pt-BR |
|---|---|
| P&L total | "Soma do resultado de todos os trades do filtro." |
| P&L médio | "P&L total dividido pelo número de trades." |
| Taxa de acerto | "Trades com resultado positivo divididos pelo total de trades." **[Provisional, Q58 rec. (a)]** a trade with P&L = 0 is not a win; the total includes all trades; wins, losses and zero are shown as counts |
| Tempo médio | "Média do tempo entre a entrada e a saída de cada trade." |
| MAE / MFE médio | "Média da maior oscilação contra (MAE) ou a favor (MFE) de cada trade, informada pelo NinjaTrader/plugin ou digitada por você." |

The pt-BR/en wording is reviewed by the PO; the definitions must match what the Core computes (QA checks a worked example).

### 3.5 States

| State | UI |
|---|---|
| Loading | Skeleton of seven figure cards and 4 table rows; changing a filter keeps the previous values dimmed (`aria-busy`) until the new ones arrive, so no old number looks current |
| First use (no trades at all) | `EmptyState` `first-use`: "Ainda não há trades para analisar." / "Conecte uma conta ou registre um trade manual para ver o desempenho." + links `Ver conexões` and `Registrar trade` |
| Filter returns no trades | `EmptyState` `filtered` with the chips and `Limpar filtros` |
| Error | `ErrorState` + retry + code; filters kept |
| New trades since `as_of` | banner `role="status"`: "Há novos trades desde {time}. Atualizar" (the figures never change under the user's eyes, so they keep reconciling with the list the user is looking at) |
| Stale stream | `Sem dados recentes` next to the `as_of` line |
| Partial MAE/MFE | basis lines as in §3.2 |
| Session ended | per F§10 |

---

## 4. Strings (examples, pt-BR / en; Frontend creates the keys)

| Where | pt-BR | en |
|---|---|---|
| Page | Desempenho | Performance |
| Data as of | Dados até {time} | Data as of {time} |
| Group by | Agrupar por | Group by |
| Source and type | Fonte e tipo | Source and type |
| Win rate | Taxa de acerto | Win rate |
| Basis | Base: {k} de {n} trades | Based on {k} of {n} trades |
| Total row | Total | Total |
| New trades | Há novos trades desde {time}. Atualizar | New trades since {time}. Refresh |

---

## 5. Accessibility

- Figures are a description list (`dl`): term = figure name, description = value + basis; the sign is text; `--pnl-*` color is secondary.
- Breakdown table: `caption` with name, count and default sort; `th scope`; the `Total` row is in `tfoot` with `th scope="row"`; bars are `aria-hidden`; sortable headers expose `aria-sort`.
- Filters and "Agrupar por" are labeled; after a change, one polite announcement of the result count; no per-value announcements.
- Definition popovers open on focus/tap, are dismissible with Esc, and the text is also reachable from the figure's `aria-describedby`.
- Contrast per F§3; targets ≥ 44 px on touch; reflow at 320 px (cards).
- Reduced motion: no animated bars.

## 6. Data and actions this slice needs (for the Architect)

| Need | Notes |
|---|---|
| `getPerformance`: filters `from`, `to`, `account_ids`; `group_by` (`source`, `signal_type`, `source_signal`) | Same filter semantics as `listTrades` (exit time, instants), same trade set; one call |
| Totals object: `trade_count`, `win_count`, `loss_count`, `breakeven_count`, `total_pnl`, `avg_pnl`, `win_rate` (string, one decimal), `avg_duration_seconds`, `avg_mae` + `mae_sample_count`, `avg_mfe` + `mfe_sample_count` | Exact strings; no float in the Core or on the wire |
| Groups: the same object + group keys (`source_id`, `source_name`, `signal_type`, `origin`), `bar_ratio` | Only groups with ≥ 1 trade; sorted as requested |
| `as_of` instant | Same snapshot as `getTradesSummary`, so S21 and S22 can be compared |
| Query-string contract shared with S21 (period, accounts, source, signal) | for the `Ver` links |

**Events (SSE):** `trade.captured` only to raise the "novos trades" banner; the figures refresh on user action.

## 7. Decisions and open questions

**Closed by the PO (Rumo, 2026-10-23):** Q-S22-3 **totals only for P&L and number of trades; time, MAE and MFE only as averages** (these depend on Q52/Q55); Q-S22-5 proportion bars accepted with the exact number always visible and the sign as text; Q-S22-6 default 30 days, "today" follows Q53 (exit time in the profile's zone, provisional); Q-S22-7 `Plataforma (sem sinal)` and `Manual` are their own groups. Sidebar: Aquarela (foundations §3.4).

**Architect (Compasso):** Q-S22-2 percent precision and string format of the win rate.

**User questions:**

| ID | Question | Spec default |
|---|---|---|
| Q58 | Win rate with P&L = 0 | **Provisional (a):** zero is not a win; total = all trades; wins, losses and zero counts shown |
| Q59 | Extra metrics beyond AC-1 (profit factor, best/worst trade, average win/loss, per-day curve) | **Not drawn until decided** |

**Reminder:** S22 reconciles with the trade list (S22 AC-2), so Q52–Q57 (MAE/MFE unit, date filter, gross P&L, optional MAE/MFE, edit/delete of manual trades, what one trade is) define the numbers.

## 8. AC traceability

| AC | Where |
|---|---|
| AC-1 totals and per-trade averages of P&L, win rate, time in trade, MAE, MFE for a period and set of accounts, split by source and signal type | §3.1–§3.3 |
| AC-2 numbers reconcile exactly with the trade list | §2, §3.3 total row, §3.5 `as_of`, `Ver` links, §6 |
| X-2, X-3, X-6, X-8 | permissions, Core-computed values, pt-BR/en, instants |
