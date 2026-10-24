# V1-S22 — Performance metrics view — Screen spec

> **Owner:** Product Designer (Nanquim) · **Status:** proposed, PO review (Rumo) applied · **Revision:** 3 · **Date:** 2026-10-24
> **Slice:** [V1-S22](../../product/release-1/slice-plan.md) AC-1 … AC-5 (slice-plan rev. 7 plus the decisions of 2026-10-23/24); vision §10. Consumes the data of S21; filters and formats are shared with it.
> **Foundations and components:** [foundations.md](../foundations.md) (F§6 numbers and time, F§7 status, F§10 feedback, F§4 "charts get their palette in this slice"), [components.md](../components.md) (C§3 `EmptyState`, C§4 `DataTable`). **Related:** [S21](v1-s21-trades.md) (trade list, filters, `—` for missing MAE/MFE), [S10](v1-s10-audit-log.md).
> **Contract:** [HTTP contract 6.1.0 (approved, hash 4d4f6cda82d4)](../../../contracts/http/openapi.yaml) (not yet committed; header to the commit once it is. 6.0.0 is committed as 71fbba4): `contracts/http/modules/performance.yaml` and the notes [s22-performance](../../architecture/contracts/s22-performance.md). Operation `getPerformance`. **Where this spec and the contract differ, the contract wins.**
> **Surfaces:** Web (Frontend). Every metric is computed by the Core from stored trades (the plugin computed MAE/MFE/P&L per trade, S21 AC-2). QA owns the checks.
> **Scope note (slice plan):** the metric set is an *assumption refined with the Designer*; **changes beyond the list in AC-1 go to the user**. This spec stays inside AC-1. Extra metrics someone may want (profit factor, best/worst trade, average win and loss, per-day curve) are **not drawn**: Q59 (a) decided, V1 shows only the AC-1 list [Decision 2026-10-23].

---

## 1. What this spec covers

| # | Screen / element | Route |
|---|---|---|
| S22-01 | Performance page: filters, summary figures, breakdown by source and signal type | `/[locale]/performance` (sidebar `Desempenho`, after `Trades`) |
| S22-02 | Metric definitions (popovers) | inside S22-01 |

---

## 2. Concepts shown to the user (no new rules)

- For a **selected period** and **set of accounts** the user sees **totals and per-trade averages** of **net P&L (gross beside it, AC-3)**, win rate, time in trade, MAE and MFE, **split by source and signal type** (AC-1).
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
| Contas / Accounts | multi-select `Combobox`, all accounts selected by default, includes deleted accounts `(excluída)` | the "set of accounts" of AC-1. Q79 (a), S21 AC-12 [Decision 2026-10-24]: trades of archived accounts stay in the data but are **excluded by default**; they enter the metrics only with the `Arquivadas` filter (a `Switch` "Incluir contas arquivadas", off by default, in the URL), the same control as S21 §3.1. |

State lives in the URL (`?period=…&account=…&group=…`) and is carried to S21. Filters apply as they change (debounced); below `md` they open in a `Sheet` with `Aplicar`/`Limpar`. An `aria-live="polite"` region announces "Resultados atualizados: {n} trades" after each change.

### 3.2 Summary figures

A `dl` of seven figure `Card`s (two columns on mobile, four ≥ `lg`), all from the Core for the filtered set:

| Figure | pt-BR / en | Format | Note |
|---|---|---|---|
| Trades | Trades | integer | |
| P&L líquido total | Total net P&L | signed money (F§6.2) with `--pnl-*` token **and** sign; the **gross** total beside it, smaller, labeled `bruto` (AC-3) | definition in §3.4 |
| P&L líquido médio por trade | Avg net P&L per trade | signed money with `--pnl-*` token and sign; the **gross average** (`avg_gross_pnl`) beside it, smaller, labeled `bruto` (AC-3, like the totals) | definition in §3.4 |
| Taxa de acerto | Win rate | percent with one decimal as sent by the Core, plus "{wins} vitórias · {losses} derrotas · {zero} zero" (counts from the Core) | definition in §3.4 |
| Tempo médio na operação | Avg time in trade | duration, two units (`12 min 30 s`) | |
| MAE médio | Avg MAE | money | basis line when not all trades have it, see below |
| MFE médio | Avg MFE | money | same |

- **Basis line** under MAE/MFE averages when some trades lack the value (manual trades may omit them, S21 §4.4): "Base: {k} de {n} trades" with a `Info` popover "Trades sem este valor não entram na média." When all have it, no line.
- Each figure has an `Info` icon-button (name "Como é calculado: {figure}") that opens the definition (§3.4).
- A figure with no data shows `—`, never `0`, **except** the trade count and the net/gross totals: with zero trades they show `0` and `US$ 0,00` (S22 AC-5 [Decision 2026-10-24]), while averages (net and gross), rates and ratios show `—`, and the page says "Nenhum trade neste filtro." (en: "No trades in this filter.", §3.5).
- **Commissions notice** (PO decision (a), [Decision 2026-10-24]): shown only when `uncommissioned_count` > 0 in the filtered set (no new field); an `Info` line under the P&L figures. pt-BR: "{n} trades sem comissão informada: neles, o líquido é igual ao bruto." (n = 1: "1 trade sem comissão informada: nele, o líquido é igual ao bruto."); en: "{n} trades have no commission entered: for those, net equals gross." (n = 1: "1 trade has no commission entered: for it, net equals gross."). No popover with account names: the Core does not return them. Estimated commissions (S21 Q71 `configured`) are not mentioned; they are part of the net like any other commission.

### 3.3 Breakdown by source and signal type

Control above the table: `ToggleGroup` "Agrupar por" with three options: `Fonte` / `Source`, `Tipo de sinal` / `Signal type`, `Fonte e tipo` / `Source and type`. **Default `Fonte e tipo`** (shows both splits at once; the other two collapse it). The choice is in the URL.

`DataTable` (C§4), one row per group that has at least one trade, plus a `Total` footer row:

| Column | Content |
|---|---|
| Grupo / Group | `Estratégia Quântico · Gatilhos`; `Manual`; `Plataforma (sem sinal)`; `Replicação · {master}` (S19, V1.1 only if present); labels as in S21 §3.4. A revoked or deleted source keeps its name with `(revogada)`; signal type for groups without one is `—` |
| Trades | integer; link `Ver` → S21 filtered to this group |
| P&L líquido total | signed net money + gross beside it + proportion bar (below, on the net) |
| P&L líquido médio | signed net money + gross average beside it (`bruto`), no bar |
| Taxa de acerto | percent |
| Tempo médio | duration |
| MAE médio / MFE médio | money, `—` when none, basis in the cell's title when partial |

- **Total row** equals the summary figures and, by construction, the S21 totals strip for the same filter (AC-2). It is a data row from the Core, not the sum of the visible rows computed in the browser.
- **Proportion bars (PO: ok, provided the exact number is always visible and color is not the only gain/loss signal):** in the P&L total cell a thin bar whose length is the group's `|net P&L total|` relative to the largest group, using `--pnl-positive` / `--pnl-negative`; the exact length is a Core value (`bar_ratio`); the bar is `aria-hidden` because the signed amount in text already carries everything. No categorical palette is needed in V1; the P&L bars need only the two P&L tokens, so no chart palette is requested from Aquarela now (F§4); a later chart slice asks for it.
- Sort: default net P&L total, highest first (stated in the caption). Sortable headers: `Trades`, `P&L líquido total`, `P&L líquido médio`, `Taxa de acerto` (`button`, `aria-sort`). Groups never exceed a few dozen, so no paging.
- Below `md`: each group is a `Card` (group name as title, net P&L total top-right, then a definition list in the same order); the `Total` card comes first. The cards keep the table's sort control (C§4, rule from Aquarela): a `Select` "Ordenar por" / "Sort by" above the cards with the same keys and directions as the sortable headers (`Trades`, `P&L líquido total`, `P&L líquido médio`, `Taxa de acerto`) and the same default; the `Total` card stays first whatever the sort. One option per key and direction (labels pt-BR / en; API `sort`, `order` in the last column):

  | Option pt-BR | Option en | `sort`, `order` |
  |---|---|---|
  | P&L líquido total: maior primeiro *(padrão)* | Net P&L total: highest first *(default)* | `total_pnl`, `desc` |
  | P&L líquido total: menor primeiro | Net P&L total: lowest first | `total_pnl`, `asc` |
  | P&L líquido médio: maior primeiro | Avg net P&L: highest first | `avg_pnl`, `desc` |
  | P&L líquido médio: menor primeiro | Avg net P&L: lowest first | `avg_pnl`, `asc` |
  | Taxa de acerto: maior primeiro | Win rate: highest first | `win_rate`, `desc` |
  | Taxa de acerto: menor primeiro | Win rate: lowest first | `win_rate`, `asc` |
  | Trades: mais primeiro | Trades: most first | `trade_count`, `desc` |
  | Trades: menos primeiro | Trades: fewest first | `trade_count`, `asc` |

  Behavior as C§4 "Sorting below `md` (cards)": the visible label "Ordenar por" / "Sort by"; desktop and mobile share one sort state and send the same request (the browser does not sort money); changing it refetches the groups from the Core, as a header click does. **On change, focus stays on the `Select` and a polite live region says "Ordenado por {campo}, {direção}" / "Sorted by {field}, {direction}"** (for example "Ordenado por Taxa de acerto, maior primeiro" / "Sorted by Win rate, highest first"). The header buttons are not rendered below `md`.

### 3.4 Definitions (popovers; text from the PO's AC-1, no invented formulas)

| Figure | Text pt-BR |
|---|---|
| P&L líquido total | "Soma do resultado líquido (já descontadas as comissões) de todos os trades do filtro. O resultado bruto aparece ao lado." |
| P&L líquido médio | "P&L líquido total dividido pelo número de trades." |
| Taxa de acerto | "Trades com resultado líquido positivo divididos pelo total de trades." [Decision 2026-10-23, Q58 (a)]: a win is a trade with **net** P&L > 0; the denominator is all trades; wins, losses and zero are shown as counts |
| Tempo médio | "Média do tempo entre a entrada e a saída de cada trade." |
| MAE / MFE médio | "Média da maior oscilação contra (MAE) ou a favor (MFE) de cada trade, informada pelo NinjaTrader/plugin ou digitada por você." |

The pt-BR/en wording is reviewed by the PO; the definitions must match what the Core computes (QA checks a worked example).

### 3.5 States

| State | UI |
|---|---|
| Loading | Skeleton of seven figure cards and 4 table rows; changing a filter keeps the previous values dimmed (`aria-busy`) until the new ones arrive, so no old number looks current |
| Filter returns no trades (S22 AC-5, decided) | The summary keeps its figures: trades `0`, net and gross totals `US$ 0,00`, **averages (net and gross), rates and ratios `—`**, with the sentence "Nenhum trade neste filtro." / "No trades in this filter." The breakdown table is replaced by an `EmptyState` `filtered` with the chips and `Limpar filtros`. Archived accounts are excluded unless the filter includes them (Q79) |
| First use (no trades at all) | `EmptyState` `first-use`: "Ainda não há trades para analisar." / "Conecte uma conta ou registre um trade manual para ver o desempenho." + links `Ver conexões` and `Registrar trade` |
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
| Totals object: `trade_count`, `win_count`, `loss_count`, `breakeven_count`, `total_pnl` (net, `pnl_basis: net`), `total_gross_pnl`, `total_commission`, `avg_pnl` (net average) and `avg_gross_pnl` (gross average; `PerformanceFigures`, summary and groups, 6.0.0), `uncommissioned_count`, `win_rate` (string, one decimal), `avg_duration_seconds`, `avg_mae` + `mae_sample_count`, `avg_mfe` + `mfe_sample_count` | Exact strings; no float in the Core or on the wire; names from the contract performance.yaml, the contract wins. `uncommissioned_count` (trades with commission source `none`) drives the §3.2 notice |
| Groups: the same object + group keys (`source_id`, `source_name`, `signal_type`, `origin`), `bar_ratio` | Only groups with ≥ 1 trade; sorted as requested |
| `as_of` instant | Same snapshot as `getTradesSummary`, so S21 and S22 can be compared |
| Query-string contract shared with S21 (period, accounts, source, signal) | for the `Ver` links |

**Events (SSE):** `trade.captured` only to raise the "novos trades" banner; the figures refresh on user action.

## 7. Decisions and open questions

**Closed by the PO (Rumo, 2026-10-23):** Q-S22-3 **totals only for P&L and number of trades; time, MAE and MFE only as averages** (these depend on Q52/Q55); Q-S22-5 proportion bars accepted with the exact number always visible and the sign as text; Q-S22-6 default 30 days, "today" follows Q53 (exit time in the profile's zone, decided); Q-S22-7 `Plataforma (sem sinal)` and `Manual` are their own groups. Sidebar: Aquarela (foundations §3.4).

**Architect (Compasso):** Q-S22-2 percent precision and string format of the win rate.

**User decisions (2026-10-23/24):**

| ID | Question | Spec default |
|---|---|---|
| Q58 | Win rate with net P&L = 0 | **Decided (a):** win = net P&L > 0; total = all trades; wins, losses and zero counts shown |
| Q59 | Extra metrics beyond AC-1 (profit factor, best/worst trade, average win/loss, per-day curve) | **Decided (a):** V1 shows only the AC-1 list; the extras are not drawn |
| Q79 | Trades of archived accounts | **Decided (a)** [2026-10-24]: kept, excluded by default, `Arquivadas` filter (S21 AC-12) |

**Reminder:** S22 reconciles with the trade list (S22 AC-2), so Q52–Q57 and Q71 (MAE/MFE unit, date filter, net P&L with gross beside it, optional MAE/MFE, edit/delete of manual trades, what one trade is, manual-trade commissions) define the numbers.

## 8. AC traceability

| AC | Where |
|---|---|
| AC-1 totals and per-trade averages of P&L, win rate, time in trade, MAE, MFE for a period and set of accounts, split by source and signal type | §3.1–§3.3 |
| AC-4 breakdown by source and signal type; Total row reconciles with S21 | §3.3 |
| AC-5 empty-state figures (count 0, totals US$ 0,00, averages/rates/ratios `—`, sentence) | §3.2, §3.5 |
| AC-12 (S21) archived accounts only with the filter | §3.1 |
| AC-3 net P&L is the main value, gross beside it; win = net > 0 | §3.2–§3.4 |
| AC-2 numbers reconcile exactly with the trade list | §2, §3.3 total row, §3.5 `as_of`, `Ver` links, §6 |
| X-2, X-3, X-6, X-8 | permissions, Core-computed values, pt-BR/en, instants |
