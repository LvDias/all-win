# V1-S13 — Risk rules per account and live risk state — Screen spec

> **Owner:** Product Designer (Traço @ Dev3) · **Status:** proposed, pending PO review · **Revision:** 1 · **Date:** 2026-10-23
> **Slice:** [V1-S13](../../product/release-1/slice-plan.md) AC-1 … AC-6 (slice-plan rev. 7); touch points: S5 (account tabs, list), S16 (preset values, provenance), S14 (Core gate), S15 (enforcement, blocks), S18 (notifications), S10 (audit), S11 (data freshness).
> **Foundations:** [foundations.md](../foundations.md): F§6 numbers and time, **F§7.1 risk states**, **F§7.3 `RiskMeter`**, F§7.4 provenance, F§10 feedback · **Related:** [S5](v1-s5-automation-opt-in.md) (§0 tabs, §8 detail), [S16](v1-s16-presets.md) (§8.1 edited values, §5 not-confirmed).
> **Contract:** no HTTP contract for risk rules/state yet. Data model preview: `risk.account_limits` ([data model §5.6](../../architecture/data/wave1-data-model.md)); limits shape: ADR-0004 `limits`. §9 lists what the screens need. Operation names are **design proposals**; the contract wins.
> **Surfaces:** Web (Frontend). The plugin's own warnings and panels (S13 AC-3, S15) are the Plugin Engineer's. State computation, resets and warnings are the Core's (Backend). QA owns the checks.
> **Not in this spec:** the enforcement switch and the flatten/block behavior (S15), and the Core refusal gate (S14): they appear here only as read-only status and explanations.

---

## 1. What this spec covers

| # | Screen / element | Route |
|---|---|---|
| S13-01 | Risk tab: live state per limit (`RiskMeter`s) | `/[locale]/accounts/[id]/risk` |
| S13-02 | Edit risk rules (drawdown, daily loss, daily profit target, consistency, trading day) | `/[locale]/accounts/[id]/risk/edit` |
| S13-03 | Risk column in the account list and risk alerts on the account detail | extension of S5-01, S5-04 |
| S13-04 | Warning, limit-reached and blocked presentation; notifications | all of the above |

---

## 2. Concepts shown to the user (no new rules)

- **Per account** the user sets: drawdown (amount and type: trailing intraday, end-of-day trailing, static), daily loss limit, daily profit target, and firm-specific rules such as consistency (AC-1).
- The Core keeps each account's **current risk state** from execution results and plugin-reported P&L; the user sees **used vs limit** per account (AC-2).
- **Exactly three levels:** `normal` (< 80 %), `warning` (≥ 80 %), `reached` (limit reached). There is **no 90 % band** (F§7.1, D18). The state is a Core field; the UI maps it.
- **Warning at 80 %** of each limit appears in the Web and in the plugin (AC-3).
- **Trading day** resets at the prop firm's reset time (from the preset) or, for real accounts, at the CME futures session close (AC-4).
- **Daily profit target:** per account, **"notify only" (default)** or **"flatten and block like a loss limit"**; with "notify only" the user is notified once when it is reached (AC-5, Q3).
- **Consistency rules** are tracked and warned (current value vs the firm's rule, 80 % warning) but **never enforced in V1** (AC-6, Q4). They use `warning` at most, never `reached` (F§7.1).
- When a limit is reached, the **Core refuses new entries even if enforcement is off** (S14 AC-1; refusing entries never closes a trade). **Exits are never refused** by risk or out-of-sync gates (P1). With enforcement on, the plugin flattens, cancels and blocks until the reset, with **no early unlock** (S15). This spec states these facts in text; the controls belong to S15.
- All limit figures and percentages come from the Core. The browser does no money arithmetic (X-3).

---

## 3. S13-01 Risk tab — `/[locale]/accounts/[id]/risk`

**Purpose:** see, at a glance, how close the account is to each limit today (AC-2, AC-3).
**Permissions:** own account only; another user's id gives the S5 `not found` state (X-2).
**Layout:** tab `Risco` on the account detail (S5 §8; the tab appears when the account has at least one rule or the user can add one). On top: `PageHeader` actions `Editar regras`.

1. **Trading-day strip** (`Card`, compact):
   - "Dia de trading: 22/10/2026 17:00 → 23/10/2026 17:00 (GMT-3)" and "Próximo reset: 23/10/2026 17:00 (GMT-3)". Source line: "Reset da firma: 17:00 CT" (tooltip: `17:00 CT (19:00 GMT-3)`, F§6.4) or "Reset: fechamento da sessão CME" for real accounts.
   - Freshness: "Atualizado às 14:31:05 (GMT-3)" (absolute time, `<time>`). If the Core flags the data stale or the account's connection is down: badge `Sem dados recentes` (F§7.2) and the line "NinjaTrader desconectado: os valores abaixo podem estar desatualizados." The UI never shows stale data as current.
2. **Account risk status** (one line, `StatusBadge`): the worst current state (`Alerta (80 %)`, `Limite atingido`, `Bloqueada até {time}` with reason, `Meta atingida`) or nothing when all is `normal`. Reasons follow F§7.1: a block caused by the profit target uses success color and says "motivo: meta de lucro diária atingida".
3. **Limits** — one `RiskMeter` (F§7.3) per rule that exists, in this order:

| Meter | Label pt-BR / en | Value shown | Notes |
|---|---|---|---|
| Daily loss | Perda diária / Daily loss | `US$ 820,00 de US$ 1.000,00 · 82 %` | `warning` ≥ 80 %, `reached` at the limit |
| Drawdown | Drawdown ({type}) | `used / limit`, type label under it | Type: `Trailing intradiário`, `Trailing fim do dia`, `Estático`. For trailing types the Core also sends the current floor ("Piso atual: US$ …") |
| Daily profit target | Meta de lucro diária / Daily profit target | progress toward the target, success color | At target: `Meta atingida` (success). With "flatten and block" chosen, it follows the blocked rule above |
| Consistency | Regra de consistência / Consistency rule | "Maior dia: 28 % do lucro total (limite da firma: 30 %)" + meter of that ratio | `warning` at 80 % of the threshold; never `reached` (never red/blocking). Tracking only |

   - Percent uses truncation (F§6.2): `80 %` is never shown while the Core state is `normal`. Color and badge come from the Core `state` field.
   - A rule that is **not set** is shown as a plain row, not a meter: "Perda diária: sem limite definido. O All Win não avisa nem recusa entradas por esta regra." with `Definir` (opens S13-02).
   - A rule that is **not confirmed** in the preset and unresolved: "Regra de consistência não confirmada — não acompanhada." with `Resolver` (S16 conflict picker). Nothing is warned from it (PO).
4. **What happens at the limit** (`Collapsible` "O que acontece quando eu atingir um limite?", closed): fixed text:
   - "O All Win recusa novas entradas nesta conta até o próximo reset, mesmo com a aplicação automática desligada. Saídas continuam permitidas."
   - "Com a aplicação automática ligada, o plugin fecha posições, cancela ordens e bloqueia a conta até o reset. Não há desbloqueio antecipado." / (link to the S15 screen or setting once it exists).
   - "A meta de lucro diária só avisa, a menos que você escolha fechar e bloquear."
   - "Regras de consistência só avisam; nunca bloqueiam."
   - Last line: link `Ver no registro de atividade` → `/activity?account={id}&category=risk` (S10).
5. **Recent risk events** (last 5 entries of category `risk` for this account, from S10 data): time with seconds, text, link to S10. Empty: "Nenhum evento de risco hoje."

**Live updates (SSE):** values refresh in place without layout shift. Only a **change of state** (normal → warning → reached, or target reached) is announced through a polite live region, and each state change also becomes a notification (S18). Ticks are not announced (F§7.3).

**States**

| State | UI |
|---|---|
| Loading | Skeleton: strip + 3 meter rows |
| No rules set yet | `EmptyState` `Gauge`: "Esta conta ainda não tem regras de risco." / "Defina limites para receber alertas aos 80 % e para o All Win recusar entradas ao atingi-los." + `Definir regras` |
| Error | `ErrorState` + retry + code |
| Stream down | strip shows `Sem dados recentes`; meters dimmed |
| Blocked | prominent `Alert` (danger, `Lock`, or success variant for target reason): "Conta bloqueada até 17:00 (GMT-3) · motivo: {reason}. Não há desbloqueio antecipado." The block comes from S15; this screen only displays it |

---

## 4. S13-02 Edit risk rules — `/[locale]/accounts/[id]/risk/edit`

**Purpose:** set the rules (AC-1, AC-4, AC-5). **Layout:** single column, max 720 px, `Card` sections, sticky footer `Cancelar` / `Salvar regras`. On submit: focus the first invalid field; summary `Alert` when ≥ 3 errors (F§10).

**Prop accounts** open with the preset's values filled (S16 AC-2) and the provenance marks of F§7.4: verified (source popover), `Não confirmado` (not prefilled, `ConflictPicker`), `Editado` (with `Restaurar valor do preset`). **Real accounts** have no preset: fields start empty, no marks.

### 4.1 Section "Perda e drawdown"

| Field | Component | Validation | Data |
|---|---|---|---|
| Drawdown — valor / amount | money `Input` (USD, F§6.3) | ≥ 0; ≤ 2 decimals; both drawdown fields set or both empty | `drawdown_amount` |
| Drawdown — tipo / type | `Select`: `Trailing intradiário`, `Trailing fim do dia`, `Estático` | required when an amount is set | `drawdown_type` |
| Limite de perda diária / Daily loss limit | money `Input` | ≥ 0 | `daily_loss_limit` |

- Help for the daily loss limit on prop accounts whose firm states it as **none**: "A firma informa que não há limite de perda diária." (field read-only, source popover).
- **Required fields [Provisional, Q-S13-1]:** for prop accounts, drawdown and the daily loss limit are required unless the preset says "none" (S16 AC-6: a required field cannot be saved while not confirmed). For real accounts all are optional; an empty field means "sem limite definido" and the Risk tab says so (§3).

### 4.2 Section "Meta de lucro diária"

| Field | Component | Notes |
|---|---|---|
| Valor da meta / Target amount | money `Input`, optional | `daily_profit_target` |
| Ao atingir / When reached | `RadioGroup` of two cards, **"Só avisar" preselected** | `profit_target_on_reach`: `notify` (default, Q3 decided) / `flatten_and_block` |

- "Só avisar": "Você recebe um aviso uma vez quando a meta for atingida. A conta continua operando."
- "Fechar posições e bloquear até o reset": "Ao atingir a meta, o plugin fecha posições, cancela ordens e bloqueia a conta até o reset, como em um limite de perda. **Exige a aplicação automática ligada (S15).**" **[Whether this option can be saved with enforcement off is an S15/Architect question, Q-S13-4; the spec shows the sentence and does not block saving.]**
- The radio group is the one place in this spec where a default is preselected, because Q3 decided a default; the opt-ins of S5 are different (no default).

### 4.3 Section "Regra de consistência" (prop accounts only)

- Shows the firm's rule **verbatim** (`FirmRuleQuote`, F§7.4) and the tracked threshold (`Input`, percent 0–100, integer or one decimal per the preset; edited values get `Editado`).
- Help: "O All Win acompanha esta regra e avisa aos 80 %, mas não bloqueia a conta." / "All Win tracks this rule and warns at 80%, but never blocks the account."
- Not confirmed: `ConflictPicker`; unresolved → saved as not tracked ("Regra de consistência não confirmada — não acompanhada."). Real accounts: section absent.

### 4.4 Section "Dia de trading"

Read-only block: "Reset do dia: {time} ({zone})" with source. Prop: from the preset (value shown with the provenance mark; **editing the preset's reset time is allowed only as an edited copy**, like any preset value **[Q-S13-2]**). Real: "Fechamento da sessão CME" (no editing). The display zone follows the user's profile (F§6.4); the firm's zone appears in the tooltip.

### 4.5 Aplicação automática (read-only line)

"Aplicação automática: {Ligada | Desligada}" with `Alterar` linking to the S15 control, which lives with the S15 spec. Default off (S15 AC-1).

### 4.6 Save, validation and effects

| Situation | UI |
|---|---|
| Client-side invalid | Inline errors, focus first invalid, summary at ≥ 3 |
| Server 422 with `pointer` | Map to the field (`/daily_loss_limit`, `/drawdown_type`, `/profit_target_on_reach`, …) and focus it |
| Unresolved not-confirmed required value | "Resolva este valor para salvar." on the field |
| New value makes the limit already reached today (the Core returns `would_be_reached`) | `AlertDialog` before saving: "Com este limite, a conta já estaria no limite hoje. Novas entradas serão recusadas até o reset{; com a aplicação automática ligada, posições serão fechadas e a conta bloqueada}. Posições abertas não são alteradas por esta edição." Buttons `Voltar` (focused) / `Salvar mesmo assim`. **[Provisional, Q-S13-3: PO to confirm when a limit edit takes effect and whether it can trigger enforcement immediately]** |
| Conflict (`common.version_conflict`) | "Estas regras mudaram em outro lugar." + `Recarregar` |
| Success | Toast "Regras de risco salvas." → Risk tab. The Core publishes a new limits version to the plugin; the audit entry records previous and new values |
| Network or 5xx | destructive `Alert` with code; values kept |
| Cancel with unsaved changes | `AlertDialog` "Descartar alterações?" |

No step-up in V1 (the S3 list excludes it); if the Architect's wider list (Q35: enforcement off, raising limits) is later approved, the uniform `StepUpDialog` mechanics of S5 §12 apply with no layout change.

---

## 5. S13-03 Account list and detail integration

- **Account list (S5-01):** add column `Risco hoje` / `Risk today`: the account's worst state as a `StatusBadge` plus the limit name and percent (`Perda diária 82 %`); nothing when all `normal`; `—` when no rules; `Sem dados recentes` when stale. The F§7.2 priority order applies to the Estado cell; the new column is a summary and does not replace it. Cards (< `md`) show it under the name.
- **Account detail (S5-04):** the alerts card shows the risk `Alert` (warning/limit reached/blocked) above the opt-in alert, linking `Ver risco`.
- **Top bar / notifications (S18):** a state change creates a notification: `Alerta (80 %)` "Conta {name}: {limit} em {percent} %" (once per state change), `Limite atingido`, `Meta atingida` (once), consistency warning. Links go to the Risk tab. Text and counts come from the Core.
- **Routing (S7):** an account at its limit appears with `routing.risk_blocked` for entries (S7 §5.3).

---

## 6. Strings (examples, pt-BR / en; Frontend creates the keys)

| Where | pt-BR | en |
|---|---|---|
| Tab | Risco | Risk |
| Warning | Alerta (80 %) | Warning (80%) |
| Reached | Limite atingido | Limit reached |
| Target reached | Meta atingida | Target reached |
| Blocked | Bloqueada até {time} · motivo: {reason} | Blocked until {time} · reason: {reason} |
| Not set | Sem limite definido | No limit set |
| Not tracked | Regra de consistência não confirmada — não acompanhada. | Consistency rule not confirmed — not tracked. |
| Notify only | Só avisar | Notify only |
| Flatten and block | Fechar posições e bloquear até o reset | Flatten and block until the reset |
| No unlock | Não há desbloqueio antecipado. | There is no early unlock. |

---

## 7. Audit events emitted (render in S10; category `risk` or `config`)

| Event (code, proposed) | Entry text | Details |
|---|---|---|
| `risk.rules_changed` (`config`) | "Regras de risco da conta {account} alteradas" | previous and new values (strings), limits version, user |
| `risk.warning` | "{account}: alerta de 80 % de {limit}" | limit, value, percent, state |
| `risk.limit_reached` | "{account}: limite {limit} atingido" | limit, value |
| `risk.target_reached` | "{account}: meta de lucro diária atingida" | target, action taken (notify / flatten and block) |
| `risk.consistency_warning` | "{account}: regra de consistência a {percent} % do limite" | current value, threshold |
| `routing.risk_blocked` | S7 / S10 catalog | |

S15 adds `risk.flatten` and `risk.blocked` (S10 catalog).

---

## 8. Accessibility

- `RiskMeter`: `role="meter"` with `aria-valuemin=0`, `aria-valuemax=100`, `aria-valuenow`, and `aria-valuetext` carrying the full sentence (F§7.3). The 80 % tick is decorative; the state badge carries the meaning (text + icon).
- Live regions: only state changes, polite; one announcement per change, even when several meters change in the same update (the most severe is announced first).
- Form: labels, units in the label ("Drawdown, em dólares"), errors linked; money input rules F§6.3; the profit-target `RadioGroup` has a legend; focus to the first invalid field.
- The collapsible "O que acontece…" is a `button` with `aria-expanded`; its content is in the DOM.
- Contrast and touch targets per F§3 and F§9; the Risk tab reflows at 320 px (meters stack, labels above values).
- Do not rely on color: success/danger/warning appear with text and icon; the "target" block uses success color but carries the text "motivo: meta de lucro diária atingida".

## 9. Data and actions this slice needs (for the Architect)

**Reads**

| Need | Notes |
|---|---|
| `getAccountRisk(account_id)`: `trading_day` (start, end, next reset, zone, source: firm / CME), `as_of` time, `stale` flag, per rule: `kind` (daily_loss, drawdown, profit_target, consistency), `limit`, `used`, `percent` (integer, truncated), **`state`** (`normal` / `warning` / `reached` / `target_reached`), drawdown `type` and current floor (trailing), consistency `current_value` and `threshold` and `percent_of_threshold`, **block info** (`blocked_until`, `reason_code`) | All decimals as strings. The Core computes every figure and state; the UI maps |
| Rules and provenance per field (verified / not confirmed with published values / edited with preset value) | same shape as the S16 preview |
| Risk summary per account for the list (worst state, limit name, percent) | Same call as the S5 list where possible |
| Last events of category `risk` per account | from S10 |
| `would_be_reached` preview for a proposed edit | needed by §4.6; can be part of a validate-only call |

**Writes**

| Action | Notes |
|---|---|
| `putAccountRiskRules` (rules, `profit_target_on_reach`, `row_version`) | 422 pointers listed in §4.6; `common.version_conflict`; the Core bumps the limits `version` for the plugin |
| `restorePresetValue` (S16) | for edited fields |

**Events (SSE):** risk state changes per account (state, percent, stale), block/unblock, trading-day reset. Mobile background tabs refetch on return (F§9).

**Codes the UI maps:** `risk.rules_invalid` (pointer), `risk.drawdown_incomplete` (type and amount must go together), `risk.required_value_unresolved` (pointer), `common.version_conflict` (names proposed).

---

## 10. Open questions

| ID | Question | Owner | Spec default (provisional) |
|---|---|---|---|
| Q-S13-1 | Which rules are required for prop and for real accounts? | PO | Prop: drawdown and daily loss required unless the preset says "none"; real: all optional, unset shown explicitly |
| Q-S13-2 | May a prop account edit the reset time of its preset copy? | PO | Yes, as an `Editado` copy; real accounts cannot |
| Q-S13-3 | When does a limit edit take effect, and can lowering a limit below today's used amount trigger enforcement at once? | PO / Architect | Immediately; the user is warned before saving (§4.6) |
| Q-S13-4 | Can "flatten and block at the profit target" be saved while enforcement is off? | PO / S15 | Allowed, with the text warning |
| Q-S13-5 | Display of the current trailing floor | Architect | Shown when the Core sends it |
| Q-S13-6 | Does the user get a notification for a consistency warning on every day, or once per state change? | PO | Once per state change (S18) |
| Q-S13-7 | Percent precision for consistency threshold | Architect | Integer or one decimal as the preset publishes |

## 11. AC traceability

| AC | Where |
|---|---|
| AC-1 rules per account (drawdown amount/type, daily loss, daily profit target, consistency) | §4 |
| AC-2 current risk state, used vs limit, per account | §3 |
| AC-3 80 % warning in the Web (plugin: S13 plugin panel) | §3, §5 notifications |
| AC-4 trading-day reset (firm / CME) | §3 strip, §4.4 |
| AC-5 profit target: notify only (default) or flatten and block; notify once | §4.2, §3 meter |
| AC-6 consistency tracked and warned, never enforced | §3, §4.3 |
| F§7.1 three levels, no 90 % band; F§7.3 RiskMeter | §2, §3, §8 |
| X-2, X-3, X-5, X-6, X-8 | §3 permissions, strings from the Core, §7, §6, time rules |
