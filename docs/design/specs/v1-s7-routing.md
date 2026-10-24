# V1-S7 — Routing configuration — Screen spec

> **Owner:** Product Designer (Traço @ Dev3) · **Status:** proposed, pending PO review · **Revision:** 1 · **Date:** 2026-10-23
> **Slice:** [V1-S7](../../product/release-1/slice-plan.md) AC-1 … AC-8 (slice-plan rev. 7); touch points: S5 (opt-ins), S3 (sources), S10 (audit), S11/S14 (gates), S16 (stances, manual-only firms), S19 (followers, V1.1).
> **Foundations:** [foundations.md](../foundations.md) · **Related specs:** [S5](v1-s5-automation-opt-in.md) (account states, `OptInSection`, nav tree §0), [S3](v1-s3-access-sources-pairing.md) (sources).
> **Contract:** [HTTP contract 5.1.1](../../../contracts/http/openapi.yaml) (commit 7872ca9, branch `s/v1-contracts-wave2--contracts`; 5.2.0 Q75/Q76 is TODO, see §5.4): `contracts/http/modules/routing.yaml` and the notes [s7-routing](../../architecture/contracts/s7-routing.md). Operations `listRoutes`, `createRoute`, `updateRoute`, `deleteRoute`, `listRouteCandidates`; `listSources` / `Source.is_automated` in the main file; `routing.subscription` in [wave1-data-model.md §5.5](../../architecture/data/wave1-data-model.md). **Where this spec and the contract differ, the contract wins.**
> **Surfaces:** Web (Frontend). Gate logic and audit are the Core's (Backend). QA owns the checks.

---

## 1. What this spec covers

| # | Screen / element | Route |
|---|---|---|
| S7-01 | Routing page: pick a source, see and edit its routes | `/[locale]/routing`, `/[locale]/routing?source={id}` |
| S7-02 | Route dialog: add or edit one (source, account) route | `ResponsiveDialog` over S7-01 |
| S7-03 | Unavailable combinations and inactive routes (AC-6) | inside S7-01/S7-02 |
| S7-04 | Stance line when adding an account (AC-7) | inside S7-02 |
| S7-05 | Manual-only firms explanation (AC-7b) | inside S7-01/S7-02 |
| S7-06 | `Rotas` tab on the account detail (reverse, read-only view) | `/[locale]/accounts/[id]/routes` |
| S7-07 | Follower moment (master replication, V1.1 pointer) | S5 spec §5.6 |

Sidebar item `Roteamento` / `Routing` (S5 spec §0).

---

## 2. Concepts shown to the user (no new rules)

- A **route** is the pair (**source**, **account**) with the **signal types** that become orders for that pair (AC-2) and an on/off switch.
- Signal types for TradingView sources: `Gatilhos` / `Triggers` (`trigger`), `Trade Plan` (`trade_plan`), `Gatilhos NQ` / `NQ Triggers` (`nq_trigger`). `master_account` is not selectable on this page (S19).
- The executed quantity is **instruction quantity × the account's multiplier**, cut to the account's maximum position (AC-3, AC-4). Multiplier and maximum belong to the **account** (S5): this page shows them read-only with a link, never edits them.
- An instruction of a type not selected for the pair is skipped for that account and audited (`routing.signal_type_not_selected`).
- One account's skip, rejection or disconnection never delays routing to the others (AC-5); the page makes no timing promise, it only shows state.

---

## 3. S7-01 Routing page — `/[locale]/routing`

**Purpose:** decide which accounts each source sends to, and with which signal types (AC-1, AC-2).
**Permissions:** any signed-in user; own sources and accounts only. Another user's id in the query gives the S5 `not found` state.

**Layout**

```
PageHeader: Roteamento                                         [Adicionar conta]
┌ Fontes ─────────────┐ ┌ Estratégia Quântico (Automatizada) · awk_Ab12… ────────────┐
│ ● Estratégia Quânt… │ │ Último sinal: há 2 min        3 rotas · 2 ativas            │
│   Teste             │ │ ┌ tabela de rotas ─────────────────────────────────────────┐ │
│                     │ │ │ Conta | Tipos | Qtd. | Recebe? | Situação | Ações         │ │
│ [Gerenciar fontes]  │ │ └──────────────────────────────────────────────────────────┘ │
└─────────────────────┘ └──────────────────────────────────────────────────────────────┘
```

- **Source list** (left, ≥ `lg`; a `Select` above the table below `lg`). Each item: name, `Automatizada` badge (from `Source.is_automated`, never a client-side type list), `Revogada` badge for revoked, mono `key_id` only (never the URL). Selection persists in the URL (`?source=`) and in the last used source for the session. Link `Gerenciar fontes` → `/connections/sources`.
- **Routes table** for the selected source (cards below `md`, F§9). The source's active routes are listed; accounts with no route are not listed (they appear in the picker, S7-02).

| Column | Content |
|---|---|
| Conta / Account | name (link to account) · firm and plan (prop) or `Real` · account state badges (F§7.2: out of sync, blocked, `Pausada`, `Desativada`, `Escolha pendente`) |
| Tipos de sinal | chips of the selected types (`Gatilhos`, `Trade Plan`, `Gatilhos NQ`); text, not color |
| Quantidade | `Mult. 2× · Máx. 3 contratos` (read-only, from the account) + link `Editar na conta` |
| Recebe / Receives | `Switch` (route on/off, `is_enabled`), accessible name "Rota {source} → {account}" |
| Situação / Status | see S7-03: `Recebe` (success) or `Não recebe agora` (neutral/warning) + the reason |
| Ações | kebab: `Configurar` (opens S7-02), `Remover rota` |

**Turning the switch** off saves immediately, toast "Rota desativada." / on: "Rota ativada." **Turning on may need a step-up** (Q35 decided; contract `createRoute` / `updateRoute`, `x-step-up: conditional`, action `route_enable`, target = the account): required when the source is automated and the route becomes enabled and the account's automation was **not** accepted with a step-up (a real account, or an opt-in that is `not_required`). **The flow is reactive:** the web sends the request; if the Core answers 403 `auth.step_up_required` the `StepUpDialog` (C§9, F§8.2) opens and the original request is sent again once with `challenge_id` + `code`; cancel leaves the switch off. The web does not decide the condition; it only reacts to that answer. Turning off and creating a route that starts disabled never need it. The same applies to `Ativar rota` in the S7-02 dialog when it saves an enabled route. Failure: switch returns, toast with code. While saving, the switch is `aria-busy` and disabled. The route's saved signal types are kept when switched off. Turning **on** is unavailable while the account is ineligible (§5.2, Q67 provisional); turning off is always allowed.

**States**

| State | UI |
|---|---|
| Loading | Skeleton: source list + 4 rows |
| No sources yet | `EmptyState` `Waypoints`: "Você ainda não tem fontes de sinal." / "Rotas ligam uma fonte (como um alerta do TradingView) às suas contas." + `Criar fonte` → S3 sources |
| No accounts yet | `EmptyState`: "Cadastre uma conta para criar rotas." + `Nova conta` |
| Source has no routes | `EmptyState` inside the table area: "Esta fonte ainda não envia para nenhuma conta." / "Instruções recebidas serão registradas, mas nenhuma conta as executará." + `Adicionar conta` |
| Source revoked | read-only banner (warning `Alert`): "Esta fonte foi revogada e não recebe sinais. As rotas abaixo ficam guardadas." Switches and `Adicionar conta` disabled |
| Error | `ErrorState` with retry and code |
| Remove route | `AlertDialog` "Remover rota {source} → {account}?" body "A conta deixa de receber sinais desta fonte. Posições e ordens abertas não são alteradas." confirm `Remover rota`; audited |

---

## 4. S7-02 Route dialog (add / edit)

Opened by `Adicionar conta` (create) or `Configurar` (edit). `ResponsiveDialog`, title "Rota de {source}".

| Field | Component | Validation | Data |
|---|---|---|---|
| Conta / Account | `Combobox` (create only; edit shows it as text) | required. Options: the user's accounts that can be routed (see S7-03 for unavailable ones, listed disabled with reason) | `account_id` |
| Tipos de sinal / Signal types | `Checkbox` group (`fieldset`/`legend`), **nothing preselected** | at least one → "Escolha ao menos um tipo de sinal." | `signal_types` |
| Rota ativa / Route enabled | `Switch`, default on | — | `is_enabled` |

- **Why nothing preselected [Proposal, §11 Q-S7-1]:** the slice plan fixes no default signal types, and a pre-checked box would send orders the user never chose.
- Help under *Tipos de sinal*: "Instruções de um tipo não marcado são ignoradas nesta conta e registradas na auditoria."
- Below the fields, a read-only block "Nesta conta": multiplier, maximum position, kind of platform account bound (`Simulação` badge) and the stance line (S7-04). Link `Editar na conta`.
- Actions: `Salvar rota` (primary), `Cancelar`. Duplicate-submit protection per F§10. Saving a route for a pair that already exists is not possible (the picker hides it); a race answers `routing.route_exists` → refetch, toast "Esta rota já existe." and the dialog switches to edit.
- Unsaved changes + Cancel → `AlertDialog` "Descartar alterações?".
- Success: toast "Rota salva." Audited (`routing.subscription_changed`, §9).

**Account options in the combobox** (one line each: name · firm/plan · state):

| Case | Option state |
|---|---|
| Eligible | selectable |
| Already has a route for this source | hidden |
| Not eligible, see S7-03 | disabled (`aria-disabled`, focusable), `Lock` icon and reason text; reason also in `aria-describedby` |
| Manual-only firm (AC-7b) | **not listed at all** (S7-05) |

---

## 5. S7-03 Unavailable combinations and inactive routes (AC-6, AC-8)

The Core computes eligibility and the effective status; the UI renders the reason codes. The UI never re-implements a gate.

### 5.1 Unavailable at creation (account picker)

For an **automated** source (`is_automated`), an account whose `automation` opt-in is not accepted cannot get a route. For a non-automated source this rule does not apply.

| Reason code (Core) | Text pt-BR / en | Link |
|---|---|---|
| `routing.automation_not_accepted` + `declined` | "Esta conta não aceita ordens automatizadas." / "This account doesn't accept automated orders." | `Alterar na conta` → account detail, Automação card |
| `routing.automation_not_accepted` + `pending_review` | "Escolha de automação pendente nesta conta." / "Automation choice pending on this account." | `Revisar` → opens the S5 review dialog |
| `routing.platform_unbound` (proposed) | "Esta conta não está vinculada a uma conta do NinjaTrader." | `Vincular` → account edit |
| `accounts.archived` | not listed | — |

The control is **disabled but focusable** (`aria-disabled="true"`), with `Lock`; the reason is inline on touch layouts and a `Tooltip` on pointer layouts (also on focus and tap, F§9), and is the control's `aria-describedby`.

### 5.2 Existing route that became inactive **[Provisional, Q67 rec. (b); user decides]**

Compliance rule (S7 AC-6, S5 AC-11): nothing restarts by itself when an account that stopped accepting automation accepts again.

- When the account changes to `declined` or `pending_review`, the **route stays visible and is not deleted** (PO, Q-S7-2), but the Core **turns it off** and records why (audit `routing.subscription_changed` with the cause). The `Situação` cell reads `Não recebe agora` with the reason, for example "Esta conta não aceita ordens automatizadas" or "Escolha de automação pendente nesta conta".
- **Turning a route off is always allowed**, whatever the account state.
- **Turning it on is unavailable while the account is ineligible**: the switch is `aria-disabled` (focusable) with the reason inline and in `aria-describedby`, plus the links `Alterar na conta` or `Revisar` (S7-03 §5.1).
- When the account accepts again, the route is **still off**: nothing resumes on its own. The switch becomes available and the user turns it on deliberately; the toast says "Rota ativada." The `Situação` cell for an off route reads `Desligada` (not `Não recebe agora`) once the account is eligible again.
- A race (the account changed while the user clicked) is answered by the Core with a refusal (`routing.route_not_eligible`, proposed) and the switch returns to off with the reason.

### 5.3 Effective status (all reasons, one cell)

`Situação` shows `Recebe` or `Não recebe agora` + the **most severe reason**; a popover (`+N`) lists all. Reasons come from the Core as codes, ordered by F§7.2 priority.

| Code | Text pt-BR | Note |
|---|---|---|
| `routing.account_inactive` | "Conta pausada." / "Conta desativada." | Blocks every action, **exits included** (AC-8). The text says so: "Também não recebe ordens de saída." |
| `routing.out_of_sync` | "Conta fora de sincronia: novas entradas bloqueadas até você limpar o alerta." | Exits still go through (AC-8). Link `Ver conta` |
| `routing.risk_blocked` | "Limite de risco atingido: novas entradas bloqueadas." | Exits still go through. Shown when S14 ships |
| `routing.automation_not_accepted` | as §5.1 | |
| `routing.copy_not_accepted` | follower routes only (S19) | |
| `routing.platform_unbound` | "Sem conta do NinjaTrader vinculada." | |
| connection disconnected | "NinjaTrader desconectado." | live state from S3/S11 |

The status is live (SSE). When the stream is not fresh the cell shows `Sem dados recentes` (F§7.2), never a stale `Recebe`.

---

### 5.4 Reverse on an account barred for entries (Q74 b, contract 5.0.0+)

No new screen. When a `reverse` arrives for an account that is barred for entries (`out_of_sync`, `risk_blocked`, `limits_pending`, `no_bracket_profile`) or has no room (`max_position_reached`) and the position is opposite, the Core runs **only the close leg** and skips the open. What the user sees:

- **Notification** `routing.reverse_close_only` (S18 catalog, severity `danger`, one per occurrence, always sent, including when the gate is `limits_pending`; it replaces the gate's grouped notification for that occurrence). Params: `reason_code` (the gate that skipped the open), `closed_qty`, `closed_side`. Text: "Inversão em {account}: só a posição foi fechada ({closed_qty} {long|short}). A nova entrada não foi aberta: {reason}." / "Reverse on {account}: only the position was closed ({closed_qty} {long|short}). The new entry was not opened: {reason}."
- **Activity (S10):** the routing entry carries the gate code with outcome *Parcial* and `reverse_closed_qty` in the details.
- **Flat account or position already on the reverse side:** with the account **in sync**, a flat account is skipped with the gate code and nothing is flattened; a position already on the reverse side is `routing.position_conflict`, never a flatten. **Exception (Q75 b, contract 5.2.0 draft, TODO):** on an **out-of-sync** account a flat *expected* position is not trusted for exits, so a barred `reverse` with gate `routing.out_of_sync` sends its close leg as a flatten (the open is never created; audit *Parcial*, no `reverse_closed_qty`), and the result is `flattened` or `already_flat`. Same texts as §5.3 otherwise.
- The route status cell is unchanged: the account is `Não recebe agora` with the gate reason; the reverse special case is an event, not a state.

**TODO (contract 5.2.0, Q75/Q76, not final):** when the flatten ends terminal (filled vs already flat) and the account is out of sync the notification is sent only when the flatten filled, with `reason_code: routing.out_of_sync`; update this section when 5.2.0 is approved.

---

## 6. S7-04 Stance line when adding an account (AC-7, S16 AC-4)

In the route dialog, once an account is chosen, if its firm's automation stance (for an automated source) is **not `allowed`**, show a compact line (info `Alert`, `Scale` icon) under the account field:

- "Posição da {firm} sobre automação: {Proíbe | Condicional | Desconhecida | Não confirmada}. Sua escolha nesta conta: {Aceita | Não aceita | Pendente}." + `Ver regra oficial` (opens the collapsed `FirmRuleQuote` in a popover/accordion with source and access date). 
- For `allowed`: no line is required; the compact info line of S5 §5.1 may show.
- The data is the account's `OptInState` (`firm_stance`, `state`, `choice`) and the quote of its current rule version; nothing is computed in the browser.
- If the account's choice is not accepted, the account is already disabled in the picker (S7-03), so the line is informational about stances `allowed`-except cases; for a **chosen + accepted** account it serves as the repeat of stance and source (AC-7).
- Copy stance for follower routes: S5 spec §5.6.

---

## 7. S7-05 Manual-only firms (AC-7b, S16 AC-7)

Accounts of a **manual-only** firm (e.g., Topstep) never appear in the account picker, in routes, or as master/follower choices.

- Below the routes table and at the bottom of the account picker: link `Não vê uma conta?` / `Can't find an account?` → `Popover` (mobile: `Drawer`):
  - Title "Contas só de acompanhamento manual" / "Manual-tracking-only accounts".
  - Text: "Algumas firmas só aceitam ordens originadas do seu dispositivo pessoal. Para elas o All Win acompanha riscos, alertas de 80 % e operações manuais, mas não envia ordens." (wording to match the preset's own reason text; **the preset text wins**).
  - A list of the user's own manual-only accounts: name · firm · reason from the preset · link to the account. If the user has none: "Você não tem contas assim."
  - Link to the S16 preset page with the verbatim rule and source/date.
- The same explanation is shown at account registration and on the account detail (S5/S16); this page only has to explain "why isn't my account here".

---

## 8. S7-06 `Rotas` tab on account detail

Read-only reverse view: one row per source that has a route for this account: source name + `Automatizada` badge, signal-type chips, `Situação` (same cell as S7-03), link `Configurar` → `/routing?source={id}` with the dialog opened for this account. Empty: "Esta conta não recebe de nenhuma fonte." + `Ir para Roteamento`. For an unbound or manual-only account: the S7-03/S7-05 reason instead of the empty state.

## 9. Audit events emitted by S7 (render in S10)

| Event (code, proposed) | Entry text | Details |
|---|---|---|
| `routing.subscription_changed` (category `config`) | "Rota {source} → {account} criada / alterada / ativada / desativada / removida" | previous and new types, enabled flag, user |
| `routing.automation_not_accepted` | per S5 §13 | instruction id, source name |
| `routing.signal_type_not_selected` | "Instrução ignorada para {account}: tipo de sinal {type} não selecionado nesta rota" | instruction id, source, type |
| `routing.account_inactive` / `.out_of_sync` / `.risk_blocked` | "Instrução ignorada para {account}: {reason}" (exits listed as allowed/blocked) | |

---

## 10. Data and actions this slice needs (for the Architect to close the contract)

**Reads**

| Need | Notes |
|---|---|
| Sources of the user (`listSources` with `include_revoked`) | `is_automated`, `status`, `key_id`, `last_received_at`, never the URL |
| Routes per source: `account_id`, `signal_types`, `is_enabled`, `updated_at`, `row_version` | |
| For each route and each candidate account: **effective status** and an **ordered list of reason codes** (all gates of ADR-0003 §4.1 plus `platform_unbound` / disconnected), computed by the Core | The UI renders; it does not decide |
| Account facts used on this page: name, kind, firm/plan, multiplier, max position, lifecycle state, sync/risk flags, platform account kind, `OptInState` (automation) with `firm_stance`, quote of the current rule version | Same payload as S5 list where possible |
| Candidate accounts for a source: eligible / unavailable-with-reason, and **excluded manual-only** accounts with the preset's reason | Manual-only never in the eligible list |
| Manual-only accounts of the user with reason text, firm and link target | S7-05 |

**Writes**

| Action | Notes |
|---|---|
| `createRoute` (source, account, signal types, enabled) | 400 `common.invalid_request` with pointer `/signal_types` (empty list); 409 `routing.route_exists`; 422 `routing.automation_not_accepted` / `routing.platform_unbound` when not eligible (race) |
| `updateRoute` (types, enabled, `row_version`) | `common.version_conflict` |
| `deleteRoute` | |
| `copy` opt-in with `createRoute` for followers | S5 spec §5.6 / S19 |

**Events (SSE):** account state changes, opt-in state changes and connection status, so the `Situação` column updates live.

---

## 11. Decisions and open questions

**Closed by the PO (Rumo, 2026-10-23), no user question needed:**

| ID | Decision | Where |
|---|---|---|
| Q-S7-1 | No signal type preselected, at least one required (design decision, S7 AC-2, Q-A4) | §4 |
| Q-S7-2 | Route of an account that declined automation stays **inactive with its reason**, never removed. What the switch does is Q67 (provisional). An instruction that arrives is skipped and audited as "conta não aceita fontes automatizadas" (S7 AC-6, S5 AC-11) | §5.2 |
| Q-S7-3 | Routes of a revoked source are read-only; the source shows as revoked; nothing is deleted (S10 AC-3) | §3 states |
| Q-S7-5 | Creating or editing a route while the connection is disconnected is allowed (configuration only; nothing executes late, S12 T1/T2); the status shows the reason | §5.3 |
| Q-S7-6 | Reason-code names belong to the Architect; those in this spec are proposals | §5, §10 |

**User question drawn as recommendation (b), provisional:** **Q67** (compliance, S7 AC-6) — turning a route **off** is always allowed; turning it **on** is unavailable while the account is ineligible, with the reason; when the account accepts again nothing restarts by itself (the route stays off until the user turns it on). *Contract changes this implies, for the Architect:* the Core sets `is_enabled = false` and audits it when the account becomes ineligible for the source; `updateRoute` with `is_enabled = true` for an ineligible account is refused (`routing.route_not_eligible`); the effective status of an off route whose account is eligible again is `route_disabled`.

**Provisional until the user decides Q37:** Q-S7-4 an account with no bound platform account is not offered for routing (`platform_unbound`).

## 12. Accessibility

- Source list: `nav` with `aria-current` on the selected source; below `lg` a labeled `Select`.
- Route table: real `table` semantics with `th scope`; the cards at < `md` keep a heading per card.
- Switch: name includes source and account; state change announced by the switch itself, result by polite toast.
- Disabled-but-focusable options and switches use `aria-disabled` and `aria-describedby` for the reason; reasons never only in a tooltip (F§9).
- Combobox: type-ahead, `aria-expanded`, Escape closes and restores focus; disabled options stay readable.
- `Situação` is text + icon, updated live without announcing every tick; a **change of effective status of a route the user is looking at** is announced once, politely.
- Targets ≥ 44 px on touch; dialog is a `Drawer` below `md`; focus returns to the trigger on close.

## 13. AC traceability

| AC | Where |
|---|---|
| AC-1 select accounts per source | S7-01, S7-02 |
| AC-2 signal types per (source, account); several named sources | S7-01 source list, S7-02 |
| AC-3 quantity = instruction × multiplier | §2, `Quantidade` column |
| AC-4 cap at max position | §2 (stated), audit in S10; notification S18 |
| AC-5 isolation of accounts' failures | §2 |
| AC-6 declined/pending unavailable with reason; skipped and audited | §5.1, §5.2, §9 |
| AC-7 stance shown when adding | §6 |
| AC-7b manual-only accounts absent, explained | §7 |
| AC-8 gates and exits | §5.3 texts |
| X-2, X-5, X-6 | §3 permissions, §9, all strings in pt-BR/en |
