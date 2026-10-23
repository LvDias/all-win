# V1-S21 — Trade capture and trade list — Screen spec

> **Owner:** Product Designer (Aquarela; written by Traço @ Dev3 in rev. 1) · **Status:** proposed, pending PO review · **Revision:** 2 · **Date:** 2026-10-23
> **Slice:** [V1-S21](../../product/release-1/slice-plan.md) AC-1 … AC-4 (slice-plan rev. 7); vision §10; consumer: S22 (metrics). Touch points: S2/S5 (accounts, capture by the plugin), S7 (source and signal type), S10 (audit).
> **Foundations and components:** [foundations.md](../foundations.md) (F§6 numbers and time, F§7 status, F§10 feedback), [components.md](../components.md) (C§3 `EmptyState`, C§4 `DataTable`). **Related:** [S10](v1-s10-audit-log.md) (filters and time rules), [S6](v1-s6-stop-target-profiles.md) (price/tick input rules), [S18](v1-s18-notifications.md).
> **Contract:** [HTTP contract 3.4.1](../../../contracts/http/openapi.yaml) (commit 721e0bc): `contracts/http/modules/trades.yaml` and the notes [s21-trades](../../architecture/contracts/s21-trades.md). Operations `listTrades`, `getTrade`, `getTradesSummary`, `createManualTrade`, `updateManualTrade`, `deleteManualTrade`, `previewManualTrade`. **Where this spec and the contract differ, the contract wins.**
> **Rev. 2 (2026-10-23, user UX rule, F§10.1):** manual entry (S21-03) is a **step-by-step flow**: Trade · Entrada e saída · Excursões (opcional) · Revisar, with the "Calculado" block and `Salvar e registrar outro`. Edit (S21-04) stays one page with sections. Fields, validation and contract are unchanged.
> **Surfaces:** Web (Frontend). Capture and metric computation are the plugin's (AC-2); storing, filtering and totals are the Core's. QA owns the checks.

---

## 1. What this spec covers

| # | Screen / element | Route |
|---|---|---|
| S21-01 | Trade list with filters and totals | `/[locale]/trades` (sidebar `Trades`) |
| S21-02 | Trade detail (side panel) | over S21-01 |
| S21-03 | Manual trade entry (AC-3) | `/[locale]/trades/new` |
| S21-04 | Edit / delete a manual trade — **provisional (Q56)** | S21-02 |

Sidebar: `Trades` and `Desempenho` (S22) after `Atividade`. (confirmed by Aquarela, foundations §3.4).

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
| Período / Period | chips `Hoje`, `7 dias`, `30 dias`, `Personalizado` (two date pickers, zone shown) | default `30 dias`. Filters by **exit time** in the profile's time zone **[Provisional, Q53 rec. (a)]** |
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
| P&L | signed money (`+US$ 120,00`, `-US$ 45,50`), **right-aligned, `tabular-nums`**, value as the Core's string (F§6.2); `--pnl-positive`/`--pnl-negative` plus the sign (F§3.2), never color alone |
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
| Placed by the user directly in the platform and captured | `Plataforma (sem sinal)` (its own origin, distinct from `Manual`; label confirmed by the PO) | |

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
**Why a stepper:** 9 controls in 3 groups, with times and prices that are easy to mistype (F§10.1: more than 6 controls, 3 or more sections). The flow uses `FormStepper` (C§2.1), page variant, max 720 px, sticky footer. Edit uses a page with sections (§4.5).

| # | Step (pt-BR / en) | Controls | Fields below |
|---|---|---|---|
| 1 | Trade / Trade | Conta, Instrumento (fixed), Direção, Quantidade | rows 1–4 |
| 2 | Entrada e saída / Entry and exit | Entrada data e hora, Entrada preço, Saída data e hora, Saída preço | rows 5–8 + block "Calculado" |
| 3 | Excursões (opcional) / Excursions (optional) | MAE, MFE | rows 9–10; `Pular etapa` |
| 4 | Revisar / Review | – | summary + "Calculado" + `Salvar trade` / `Salvar e registrar outro` |

- `Continuar` validates only its step; the cross-field rule "a saída deve ser depois da entrada" runs on step 2. Going back keeps every value (3.3.7). Cancel / back with typed data asks "Descartar alterações?"; no draft is stored.
- If the user has no accounts the flow does not start (see States).

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
| MAE | money `Input` (≥ 0, ≤ 2 decimals), in USD **[Provisional, Q52]** | optional **[Provisional, Q55]** | `mae` |
| MFE | money `Input` (≥ 0) | optional | `mfe` |

- **Tempo na operação** and **P&L** are not typed: after the required fields of step 2 are valid, a read-only block "Calculado" shows it under the fields of step 2 (so a typo is caught before the review) and again, from the same request, in the review. It shows the duration and the P&L as returned by a Core preview (debounced 300 ms; skeleton while loading; on failure "Não foi possível calcular agora. Você ainda pode salvar." and the Core computes at save). Duration = exit − entry; P&L = exact arithmetic of prices, quantity and the instrument's tick value in the Core (PO). **Gross P&L, without commissions [Provisional, Q54 rec. (a)]**, and the detail says so. Help: "Calculado a partir dos preços, da quantidade e do valor do tick do instrumento." Entries never use floating point (X-3).
- Source is fixed: a read-only line "Origem: Manual" in the review.
- Time zone: typed values are interpreted in the display zone (profile, F§6.4); the zone is shown next to each label.
- **Review (step 4):** `dl` groups Trade · Entrada e saída · Excursões with `Editar {group}`; empty MAE/MFE read "Não informado" (§4.4); the "Calculado" block; "Origem: Manual"; the time zone next to every time. Buttons: `Voltar`, **`Salvar e registrar outro`** (secondary) and **`Salvar trade`** (primary).
- **Save:** one idempotency key per form instance (a double click or a retry never creates two trades). `Salvar trade`: toast "Trade registrado." → list with the new row marked `Novo`. `Salvar e registrar outro`: toast "Trade registrado. Pronto para o próximo." (polite), the flow returns to step 1 with a new idempotency key, **keeping Conta only** (everything else cleared), focus on the step 1 heading; the list is not shown. A failed save keeps all values. 422 `pointer` → `/account_id`, `/direction`, `/quantity` step 1; `/entry_*`, `/exit_*` step 2; `/mae`, `/mfe` step 3: the user returns to that step, the field is focused, and the review shows "Corrija a etapa {n}". Errors: destructive `Alert` above the buttons with the code; values kept.
- **States:** loading of the account list (skeleton), no accounts → `EmptyState` `blocked`: "Cadastre uma conta para registrar trades." + `Nova conta`. Account list error → `ErrorState`.

### 4.3 Display of derived values

For manual trades the detail says how the P&L was obtained (gross from prices, no fees) so a manual trade is never confused with a platform-computed one.

### 4.4 Missing MAE/MFE

Manual trades may have no MAE/MFE **[Provisional, Q55 rec. (a)]**. The list and detail show `—` with the tooltip "Não informado". S22 excludes them from MAE/MFE averages and shows how many trades entered each average (S22 spec).

### 4.5 S21-04 Edit and delete a manual trade **[Provisional, Q56 rec. (a); user decides]**

Not in the ACs, but wrong manual entries would pollute S22. The detail panel of a **manual** trade shows `Editar` and `Excluir`; **captured trades never have them** (the panel says "Trade capturado: não pode ser alterado.").
- `Editar`: **one page with sections** `Trade` · `Entrada` · `Saída` · `Excursões`, prefilled (F§10.1: edit is not a stepper; same validation, same "Calculado" preview, a single `Salvar`, route `/[locale]/trades/[id]/edit`, new idempotency key, `row_version` for concurrency → `common.version_conflict` "Este trade mudou em outro lugar."). Success toast "Trade atualizado."
- `Excluir`: `AlertDialog` (cancel focused): "Excluir este trade manual? Ele sai da lista e das métricas. Esta ação não pode ser desfeita." / confirm `Excluir trade` (destructive). Toast "Trade excluído."
- Both are audited with **before and after** values (S10, category `config`). Deleting keeps the audit entry; the trade disappears from S21/S22.

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
| Steps | Trade · Entrada e saída · Excursões (opcional) · Revisar | Trade · Entry and exit · Excursions (optional) · Review |
| Save and repeat | Salvar e registrar outro | Save and add another |
| After repeat | Trade registrado. Pronto para o próximo. | Trade added. Ready for the next one. |

---

## 6. Audit events (render in S10)

| Event (code, proposed) | Entry text | Details |
|---|---|---|
| `trades.captured` (category `execution`) | "Trade concluído em {account}: {direction} {quantity}" | trade id, P&L (string), source link |
| `trades.manual_created` (`config`) | "Trade manual registrado em {account}" | fields entered, user |
| `trades.manual_changed` / `.manual_deleted` (`config`, Q56 provisional) | "Trade manual alterado / excluído" | previous values |

## 7. Accessibility

- Table semantics from C§4 (`caption` with name, count and default sort, `th scope`, sortable headers as buttons with `aria-sort`); the sticky first column keeps its row header role.
- P&L sign is part of the text (`+`/`-`), and the color is secondary; direction has text + icon.
- Filters labeled; a text summary of active filters precedes the table; changes of the result count are announced politely once per filter change.
- Date-time inputs accept typing; pickers are optional helpers, fully keyboard operable; errors linked with `aria-describedby`, focus to the first invalid field (3.3.1).
- Panel/Drawer: focus to the title, trapped, restored to the row; Escape closes.
- Manual entry flow: `FormStepper` rules (C§2.1): step heading focused, polite "Etapa n de 4", review `dl` with `Editar {group}`; after `Salvar e registrar outro` focus goes to the step 1 heading and the live region says the trade was saved. Targets ≥ 44 px on touch; reflow at 320 px (cards; footer buttons stack).

## 8. Data and actions this slice needs (for the Architect)

**Reads**

| Need | Notes |
|---|---|
| `listTrades`: filters `from`, `to`, `account_ids`, `source_ids` (incl. `manual`, `platform`), `signal_types`; sort (`exit_time`, `pnl`, `duration`) and direction; cursor; limit 50 | Instants (X-8) |
| Trade fields: `id`, `account_id`, `account_name` (snapshot), `account_deleted`, `instrument`, `direction`, `quantity`, `entry_time`, `entry_price`, `exit_time`, `exit_price`, `duration_seconds`, `pnl`, `mae`, `mfe` (nullable), `origin` (`routed` / `master` / `manual` / `platform`), `source_id`/`source_name`, `signal_type`, `instruction_id`, `command_id`, `created_at`, `created_by` (manual) | Money and prices as strings; no raw price data |
| `getTradesSummary` with the same filters: `count`, `total_pnl` | Same numbers S22 uses |
| Instruments with tick size and value | for input validation messages |
| **Manual-trade preview** (stateless): duration and P&L for given account, instrument, direction, quantity, times, prices | no persistence |

**Writes:** `createManualTrade` (idempotency key; 422 pointers `/entry_time`, `/exit_time`, `/entry_price`, `/exit_price`, `/quantity`, `/account_id`; codes `trades.exit_before_entry`, `trades.not_tick_multiple`, `trades.in_future`, names proposed); `updateManualTrade` / `deleteManualTrade` (Q56, provisional; `row_version`).

**Events (SSE):** `trade.captured` (id, account) for the "novos trades" banner.

**Plugin capture (data only, for the Plugin Engineer/Architect):** a completed-trade message carrying exactly the AC-1 fields with exact decimals as strings, an id for idempotency (a trade is never stored twice, X-?/S12 spirit), and the originating `command_id` when the trade came from a routed instruction.

## 9. Decisions and open questions

**Closed by the PO (Rumo, 2026-10-23):** duration = exit − entry (derived); the P&L of a manual trade is exact arithmetic in the Core from prices, quantity and tick value; a trade placed directly in the platform without a signal has its own origin, distinct from `Manual`, labeled `Plataforma (sem sinal)`. Sidebar position: Aquarela (foundations §3.4).

**User questions drawn as recommendation (a), provisional:**

| ID | Question | Spec default (provisional) |
|---|---|---|
| Q52 | Unit of MAE and MFE | USD |
| Q53 | Date filter | By **exit time**, in the profile's time zone |
| Q54 | P&L definition | Gross (no commissions); the detail says so |
| Q55 | MAE/MFE in manual entry | Optional; `—` and left out of the averages, with the count of trades that entered each average (S22) |
| Q56 | Edit/delete manual trades | Allowed, audited before/after; captured trades are never editable |
| Q57 | What one "trade" is | One per entry with FIFO exits in individual mode; flat-to-flat in shared mode (the Core sends the trades; the UI shows what it receives) |

Q52–Q57 define the numbers that S22 reconciles with this list.

## 10. AC traceability

| AC | Where |
|---|---|
| AC-1 captured fields incl. time in trade, P&L, MAE, MFE, originating signal/source | §3.3, §4.1 |
| AC-2 computed by the plugin; only numbers; no raw price data | §2, §4.1, §8 |
| AC-3 manual entry, same fields, source "manual" | §4.2 |
| AC-4 list and filter by account, date, source, signal type | §3.1 |
| X-2, X-3, X-6, X-8 | permissions, exact values, pt-BR/en, instants |
