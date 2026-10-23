# V1-S21 — Trade capture and trade list — Screen spec

> **Owner:** Product Designer (Traço @ Dev3) · **Status:** proposed, pending PO review · **Revision:** 1 · **Date:** 2026-10-23
> **Slice:** [V1-S21](../../product/release-1/slice-plan.md) AC-1 … AC-4 (slice-plan rev. 7); vision §10; consumer: S22 (metrics). Touch points: S2/S5 (accounts, capture by the plugin), S7 (source and signal type), S10 (audit).
> **Foundations and components:** [foundations.md](../foundations.md) (F§6 numbers and time, F§7 status, F§10 feedback), [components.md](../components.md) (C§3 `EmptyState`, C§4 `DataTable`). **Related:** [S10](v1-s10-audit-log.md) (filters and time rules), [S6](v1-s6-stop-target-profiles.md) (price/tick input rules), [S18](v1-s18-notifications.md).
> **Contract:** no HTTP contract for trades yet and no trade message in ADR-0004. §9 lists what the screens need. Operation names are **design proposals**; the contract wins.
> **Surfaces:** Web (Frontend). Capture and metric computation are the plugin's (AC-2); storing, filtering and totals are the Core's. QA owns the checks.

---

## 1. What this spec covers

| # | Screen / element | Route |
|---|---|---|
| S21-01 | Trade list with filters and totals | `/[locale]/trades` (sidebar `Trades`) |
| S21-02 | Trade detail (side panel) | over S21-01 |
| S21-03 | Manual trade entry (AC-3) | `/[locale]/trades/new` |
| S21-04 | Edit / delete a manual trade — **proposal, not drawn for build until the PO decides (Q-S21-6)** | S21-02 |

Sidebar: `Trades` and `Desempenho` (S22) after `Atividade`. **[Aquarela to confirm the position and names.]**

---

## 2. Concepts shown to the user (no new rules)

- A **trade** is a completed trade captured from a connected account by the plugin, or entered by the user (AC-1, AC-3). Each carries: account, instrument, direction, quantity, entry time and price, exit time and price, time in trade, P&L, MAE (maximum adverse excursion), MFE (maximum favorable excursion), and the originating signal/source.
- **Metrics are computed by the plugin** from the user's own data; **only computed numbers reach the Core; no raw price data is stored** (AC-2). The screens therefore show numbers and never charts of prices.
- A **manual trade** has the same fields and source "manual" (AC-3).
- The list is filterable by account, date, source and signal type (AC-4).
- All amounts, prices and durations come from the Core as exact values; the browser formats them and does no money arithmetic (X-3). Totals and averages on the screen are Core values.

---

## 3. S21-01 Trade list — `/[locale]/trades`

**Purpose:** review completed trades (AC-1, AC-4).
**Permissions:** any signed-in user, own trades only (X-2). A trade id of another user gives the standard `not found`.

**Layout:** `PageHeader` "Trades" with primary action `Registrar trade` / `Add trade` · filter bar · totals strip · `DataTable` (C§4).

### 3.1 Filters (AC-4)

| Filter | Component | Values |
|---|---|---|
| Período / Period | chips `Hoje`, `7 dias`, `30 dias`, `Personalizado` (two date pickers, zone shown) | default `30 dias`. Filters by **exit time** **[Q-S21-2]**, in the display zone |
| Conta / Account | multi-select `Combobox` including deleted accounts `(excluída)` | |
| Fonte / Source | multi-select: the user's sources (incl. revoked), plus `Manual` and `Plataforma (sem sinal)` | see §3.4 |
| Tipo de sinal / Signal type | `Gatilhos`, `Trade Plan`, `Gatilhos NQ`, `Master` (only if the Core returns master trades, S19) | as S7/S10 |

State lives in the URL (`?period=…&account=…&source=…&signal=…`). Filters apply as they change (debounced); below `md` they open in a `Sheet` with `Aplicar`/`Limpar`; the active count shows on the trigger and in text above the list ("2 filtros ativos").

### 3.2 Totals strip

A `Card` above the table with values **from the Core for the whole filtered set** (not only the loaded page): number of trades, total P&L (signed money), and the same period/accounts as the filter in text. S22 uses the same Core figures, so both screens reconcile exactly (S22 AC-2). While loading: `Skeleton`; on error the strip shows `—` with a retry.

### 3.3 Table

Default sort: exit time, newest first (stated in the caption). Sortable headers (`button`, `aria-sort`): `Saída`, `P&L`, `Tempo na operação`. Cursor paging, 50 per page, `Carregar mais`, count "50 de 132" (C§4).

| Column | Content |
|---|---|
| Conta | account name (primary cell, sticky) |
| Direção | `Compra` / `Venda` with `ArrowUpRight` / `ArrowDownRight` (text + icon) |
| Qtd. | `3 contratos` (F§6.2) |
| Entrada | date-time with seconds, and the price below (`21.234,50`, F§6.2) |
| Saída | date-time with seconds, and the price below |
| Tempo | duration, up to two units: `12 min 30 s`, `2 h 05 min` |
| P&L | signed money (`+US$ 120,00`, `-US$ 45,50`), `--pnl-positive`/`--pnl-negative` plus the sign (F§3.2), never color alone |
| MAE / MFE | two values, labeled; `—` when not available (§4.4) |
| Origem | source name + signal type chip; `Manual`; `Plataforma (sem sinal)` (§3.4) |
| Ações | kebab: `Ver detalhes`, `Ver instrução` (when originating instruction exists, → S10 timeline), `Ver conta` |

The table scrolls horizontally inside its container with a sticky first column (F§9). Below `md`, each trade is a `Card`: account + direction as title, P&L top-right, then the definition list in the same order.

### 3.4 Origin labels

| Origin | Label | Notes |
|---|---|---|
| Created by a routed instruction | source name · signal chip (`Gatilhos`, `Trade Plan`, `Gatilhos NQ`) | link to the instruction timeline |
| Master replication (S19, V1.1) | `Replicação · {master account}` | |
| Entered by the user | `Manual` badge (neutral, `PencilLine`) | AC-3 |
| Placed by the user directly in the platform and captured | `Plataforma (sem sinal)` **[Q-S21-8: the slice plan only defines source "manual" for user-entered trades]** | |

### 3.5 States

| State | UI |
|---|---|
| Loading | Skeleton of 5 rows + totals strip |
| Empty, no trades yet | `EmptyState` `first-use`: "Nenhum trade ainda." / "Trades concluídos nas suas contas conectadas aparecem aqui automaticamente. Você também pode registrar um trade manualmente." + `Registrar trade` and link `Ver conexões` |
| Empty after filter | `EmptyState` `filtered` with chips and `Limpar filtros` (never offers "create") |
| Error | `ErrorState` + retry + code; filters kept |
| Live update (SSE) | a captured trade does not insert itself under the cursor; banner `role="status"`: "{k} novos trades · Mostrar"; new rows are marked `Novo` (text badge) for 10 s (C§4) |
| Stale | `Sem dados recentes` (F§7.2) next to the totals when the stream is down |
| Session ended | per F§10 |

---

## 4. S21-02 / S21-03 Detail and manual entry

### 4.1 Detail panel

`Sheet` from the right (≥ `md`) or full-screen `Drawer` (< `md`); title "Trade {account} · {direction} {quantity}"; focus moves to the title and returns to the row on close. Sections:
1. **Resumo:** all AC-1 fields in a definition list, same formatting as the table, times with the zone.
2. **Origem:** source, signal type, `Ver instrução` (S10 timeline), `Ver conta`; for manual trades "Registrado por você em {date-time}".
3. **Como foi calculado:** one line for captured trades, "Calculado pelo NinjaTrader/plugin com os seus dados. O All Win guarda só os números." (AC-2). For manual: "Valores digitados por você." with the P&L line explained (§4.3).
4. **Dados técnicos** (`Collapsible`, closed): trade id, `instruction_id`, `command_id`, captured-at time, account id; mono; `Copiar id`.

### 4.2 Manual entry — `/[locale]/trades/new`

**Purpose:** record a trade the system did not capture (AC-3).
**Layout:** single column, max 720 px, `Card`s "Trade", "Entrada", "Saída", "Excursões (opcional)". Sticky footer `Cancelar` / `Salvar trade`. Submit: focus first invalid field; summary `Alert` at ≥ 3 errors (F§10). Unsaved-changes guard on Cancel/back.

| Field | Component | Validation | Data |
|---|---|---|---|
| Conta / Account | `Select` of the user's accounts (active or not) | required | `account_id` |
| Instrumento / Instrument | `Select`; MNQ is the only option in V1 and is shown preselected and enabled (it is a fact, not a choice) | required | `instrument` |
| Direção / Direction | `RadioGroup` `Compra` / `Venda`, **nothing preselected** | required | `direction` |
| Quantidade / Quantity | `Input inputMode=numeric` | integer ≥ 1 | `quantity` |
| Entrada — data e hora | date + time `Input` with picker (typing in locale format `dd/mm/aaaa hh:mm:ss`), label includes the zone "(GMT-3)" | required; not in the future | `entry_time` |
| Entrada — preço | price `Input` (F§6.3) | required; multiple of the tick (values from the Core): "Use múltiplos de 0,25" | `entry_price` |
| Saída — data e hora | same | required; after the entry time → "A saída deve ser depois da entrada." | `exit_time` |
| Saída — preço | same | same | `exit_price` |
| MAE | money `Input` (≥ 0, ≤ 2 decimals) | optional **[Q-S21-5]** | `mae` |
| MFE | money `Input` (≥ 0) | optional | `mfe` |

- **Tempo na operação** and **P&L** are not typed: after the required fields are valid, a read-only block "Calculado" shows the duration and the P&L as returned by a Core preview (debounced 300 ms; skeleton while loading; on failure "Não foi possível calcular agora. Você ainda pode salvar." and the Core computes at save). **[Q-S21-3/4: PO to confirm that duration and P&L are derived from times, prices, quantity and the instrument's exact tick value, and whether fees are included.]** Help: "Calculado a partir dos preços, da quantidade e do valor do tick do instrumento." Entries never use floating point (X-3).
- Source is fixed: a read-only line "Origem: Manual".
- Time zone: typed values are interpreted in the display zone (profile, F§6.4); the zone is shown next to each label.
- **Save:** idempotency key per form (a double click or a retry never creates two trades). Success: toast "Trade registrado." → list with the new row marked `Novo`. 422 `pointer` maps to the field and focuses it. Errors: destructive `Alert` above the footer with the code; values kept.
- **States:** loading of the account list (skeleton), no accounts → `EmptyState` `blocked`: "Cadastre uma conta para registrar trades." + `Nova conta`. Account list error → `ErrorState`.

### 4.3 Display of derived values

For manual trades the detail says how the P&L was obtained (gross from prices, no fees unless Q-S21-4 says otherwise) so a manual trade is never confused with a platform-computed one.

### 4.4 Missing MAE/MFE

Manual trades may have no MAE/MFE. The list and detail show `—` with the tooltip "Não informado". S22 excludes them from MAE/MFE averages and says how many trades were excluded (S22 spec).

### 4.5 S21-04 Edit and delete manual trades — proposal only

Not in the ACs. Wrong manual entries would pollute S22. If the PO approves **[Q-S21-6]**: the detail panel of a **manual** trade gets `Editar` (same form, prefilled) and `Excluir` (`AlertDialog`: "Excluir este trade manual? Ele sai das métricas. Esta ação não pode ser desfeita." / `Excluir trade`); both are audited with previous values (S10, category `config`). Captured trades can never be edited or deleted.

---

## 5. Strings (examples, pt-BR / en; Frontend creates the keys)

| Where | pt-BR | en |
|---|---|---|
| Page | Trades | Trades |
| Add | Registrar trade | Add trade |
| Direction | Compra · Venda | Buy · Sell |
| Time in trade | Tempo na operação | Time in trade |
| MAE / MFE | Excursão adversa máx. (MAE) · Excursão favorável máx. (MFE) | Max adverse excursion (MAE) · Max favorable excursion (MFE) |
| Manual badge | Manual | Manual |
| Not provided | Não informado | Not provided |
| Exit before entry | A saída deve ser depois da entrada. | Exit must be after entry. |

---

## 6. Audit events (render in S10)

| Event (code, proposed) | Entry text | Details |
|---|---|---|
| `trades.captured` (category `execution`) | "Trade concluído em {account}: {direction} {quantity}" | trade id, P&L (string), source link |
| `trades.manual_created` (`config`) | "Trade manual registrado em {account}" | fields entered, user |
| `trades.manual_changed` / `.manual_deleted` (`config`, only if Q-S21-6) | "Trade manual alterado / excluído" | previous values |

## 7. Accessibility

- Table semantics from C§4 (`caption` with name, count and default sort, `th scope`, sortable headers as buttons with `aria-sort`); the sticky first column keeps its row header role.
- P&L sign is part of the text (`+`/`-`), and the color is secondary; direction has text + icon.
- Filters labeled; a text summary of active filters precedes the table; changes of the result count are announced politely once per filter change.
- Date-time inputs accept typing; pickers are optional helpers, fully keyboard operable; errors linked with `aria-describedby`, focus to the first invalid field (3.3.1).
- Panel/Drawer: focus to the title, trapped, restored to the row; Escape closes.
- Targets ≥ 44 px on touch; reflow at 320 px (cards).

## 8. Data and actions this slice needs (for the Architect)

**Reads**

| Need | Notes |
|---|---|
| `listTrades`: filters `from`, `to`, `account_ids`, `source_ids` (incl. `manual`, `platform`), `signal_types`; sort (`exit_time`, `pnl`, `duration`) and direction; cursor; limit 50 | Instants (X-8) |
| Trade fields: `id`, `account_id`, `account_name` (snapshot), `account_deleted`, `instrument`, `direction`, `quantity`, `entry_time`, `entry_price`, `exit_time`, `exit_price`, `duration_seconds`, `pnl`, `mae`, `mfe` (nullable), `origin` (`routed` / `master` / `manual` / `platform`), `source_id`/`source_name`, `signal_type`, `instruction_id`, `command_id`, `created_at`, `created_by` (manual) | Money and prices as strings; no raw price data |
| `getTradesSummary` with the same filters: `count`, `total_pnl` | Same numbers S22 uses |
| Instruments with tick size and value | for input validation messages |
| **Manual-trade preview** (stateless): duration and P&L for given account, instrument, direction, quantity, times, prices | no persistence |

**Writes:** `createManualTrade` (idempotency key; 422 pointers `/entry_time`, `/exit_time`, `/entry_price`, `/exit_price`, `/quantity`, `/account_id`; codes `trades.exit_before_entry`, `trades.not_tick_multiple`, `trades.in_future`, names proposed); `updateManualTrade` / `deleteManualTrade` only if Q-S21-6.

**Events (SSE):** `trade.captured` (id, account) for the "novos trades" banner.

**Plugin capture (data only, for the Plugin Engineer/Architect):** a completed-trade message carrying exactly the AC-1 fields with exact decimals as strings, an id for idempotency (a trade is never stored twice, X-?/S12 spirit), and the originating `command_id` when the trade came from a routed instruction.

## 9. Open questions

| ID | Question | Owner | Spec default |
|---|---|---|---|
| Q-S21-1 | Unit of MAE and MFE (USD, points or ticks) | PO / Architect | USD strings like P&L |
| Q-S21-2 | Date filter applies to exit time | PO | Exit time |
| Q-S21-3 | Duration derived from entry and exit times for manual trades | PO | Derived |
| Q-S21-4 | P&L of manual trades computed from prices; are fees/commissions included in captured P&L? | PO / Architect | Computed gross; the detail states it |
| Q-S21-5 | MAE/MFE optional in manual entry; excluded from averages when missing | PO | Optional |
| Q-S21-6 | Can the user edit or delete manual trades? | PO | Not built until decided |
| Q-S21-7 | Definition of one "trade": flat-to-flat round trip vs one per entry (individual mode adds, partial exits) | PO / Architect | As the Core sends |
| Q-S21-8 | Source label for trades the user places directly in the platform | PO | `Plataforma (sem sinal)` |
| Q-S21-9 | Position in the sidebar / names `Trades`, `Desempenho` | Aquarela | After `Atividade` |

## 10. AC traceability

| AC | Where |
|---|---|
| AC-1 captured fields incl. time in trade, P&L, MAE, MFE, originating signal/source | §3.3, §4.1 |
| AC-2 computed by the plugin; only numbers; no raw price data | §2, §4.1, §8 |
| AC-3 manual entry, same fields, source "manual" | §4.2 |
| AC-4 list and filter by account, date, source, signal type | §3.1 |
| X-2, X-3, X-6, X-8 | permissions, exact values, pt-BR/en, instants |
