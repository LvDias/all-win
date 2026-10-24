# V1-S13 — Risk rules per account and live risk state — Screen spec

> **Owner:** Product Designer (Aquarela, took over from Traço) · **Status:** proposed, pending PO review · **Revision:** 2 (PO review 2026-10-23: net basis, commission AC-11, AC-7/8/9 decided, S13b added) · **Date:** 2026-10-23
> **Slice:** [V1-S13](../../product/release-1/slice-plan.md) AC-1 … AC-11 and [V1-S13b](../../product/release-1/slice-plan.md) AC-1 … AC-6 (slice-plan rev. 8); touch points: S5 (account tabs, list), S16 (preset values, provenance), S14 (Core gate), S15 (enforcement, blocks), S18 (notifications), S10 (audit), S11 (data freshness).
> **Foundations:** [foundations.md](../foundations.md): F§6 numbers and time, **F§7.1 risk states**, **F§7.3 `RiskMeter`**, F§7.4 provenance, F§10 feedback · **Related:** [S5](v1-s5-automation-opt-in.md) (§0 tabs, §8 detail), [S16](v1-s16-presets.md) (§8.1 edited values, §5 not-confirmed).
> **Contract:** [HTTP contract 6.2.0](../../../contracts/http/openapi.yaml) (committed on `d/contracts-s8` as 02bf2d2, not yet in `main`; this spec uses 5.4.0/6.0.0 for `evaluation_target_status`, `day_unknown` and `restartTradingDayCount`, and 6.1.0/6.2.0 for the `enforcement_id` episode): `contracts/http/modules/risk.yaml` and the notes [s13-risk](../../architecture/contracts/s13-risk.md). Typed state, rules, preview (`previewAccountRiskRules`), `previewable` limits; ADR-0004 `limits`/`risk_state` for the plugin. **Where this spec and the contract differ, the contract wins.** Operation names below that differ are from the draft: `getAccountRisk`, `listRiskSummaries`, `getAccountRiskRules`, `putAccountRiskRules`, `previewAccountRiskRules`.
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
- The Core keeps each account's **current risk state** from execution results and plugin-reported P&L, **net of commissions** (firms measure on net; Q54 exception, AC-2, AC-11); the user sees **used vs limit** per account. **Only those two sources count:** manual trades (S21) never change the risk state, limits, blocks or consistency figures.
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
   - **Playback account (S15 AC-7):** the strip says "Relógio de Playback: o reset e o P&L do dia seguem o relógio simulado ({simulated date-time}), não o do computador." The Core supplies the simulated time; a Sim reset or Playback rewind is a discontinuity handled by S11 (not counted as P&L).
   - Freshness: "Atualizado às 14:31:05 (GMT-3)" (absolute time, `<time>`). If the Core flags the data stale or the account's connection is down: badge `Sem dados recentes` (F§7.2) and the line "NinjaTrader desconectado: os valores abaixo podem estar desatualizados." The UI never shows stale data as current.
2. **Account risk status** (one line, `StatusBadge`): the worst current state (`Alerta (80 %)`, `Limite atingido`, `Bloqueada até {time}` with reason, `Meta atingida`) or nothing when all is `normal`. Reasons follow F§7.1: a block caused by the profit target uses success color and says "motivo: meta de lucro diária atingida".
3. **Banners:** a limit that is not `normal` shows a `RiskBanner` (C§5) above the meters, with the same wording as the notification (S18) and the shell aggregate.
   - **Basis line (AC-2, AC-11):** under the strip, a muted line "Valores líquidos, depois de comissões." / "Net values, after commissions." (`pnl_basis`). Gross P&L, when shown, sits beside the net value and is labeled "bruto". Every "usado" value in this tab is **net**.
   - **Commission notice (`commission_source = none`):** `Alert` info "Comissão não informada: os limites podem estar folgados demais. O líquido é igual ao bruto nesta conta." + link `Informar comissão` (S13-02, section Comissões). It is a notice, never a block. Other sources (`platform`, `configured`, `mixed`, `manual`) show the source in the basis popover (`Info`): "Comissão da plataforma" / "Comissão configurada (mínimo por contrato)" / "Mista".
   - **Pending lower commission floor (`pending_commission_change`):** "Nova comissão por contrato/lado {value} a partir de {date-time zone}." (change `commission_floor_lowered`) or "A comissão mínima será removida a partir de {date-time zone}." (`commission_floor_removed`, no value), same style as `pending_changes`.
   - **Manual trades:** one muted line "Operações lançadas à mão não entram no risco desta conta." / "Manually entered trades do not count toward this account's risk." (AC-2).
   - **Evaluation progress (S13b):** card of §12 below, between the meters and "O que acontece…", only on evaluation accounts.
4. **Limits** — one `RiskMeter` (F§7.3) per rule that exists, in this order:

| Meter | Label pt-BR / en | Value shown | Notes |
|---|---|---|---|
| Daily loss | Perda diária / Daily loss | `US$ 820,00 de US$ 1.000,00 · 82 %` | `warning` ≥ 80 %, `reached` at the limit |
| Drawdown | Drawdown ({type}) | `used / limit`, type label under it | Type: `Trailing intradiário`, `Trailing fim do dia`, `Estático`. For trailing types the Core also sends the current floor ("Piso atual: US$ …"). **Q90 (a), user 2026-10-24:** the point where a firm stops trailing (`stops_trailing_at`) becomes a structured rule modelled by the Core. **Interim, until the field exists:** the card keeps showing the trailing floor and adds a `warning` line "A firma pode parar de ajustar o piso (trailing) em um ponto fixo. Confira a regra oficial." / "The firm may stop trailing the floor at a fixed point. Check the official rule." Contract field: request to the Architect. |
| Daily profit target | Meta de lucro diária / Daily profit target | progress toward the target, success color | At target: `Meta atingida` (success). With "flatten and block" chosen, it follows the blocked rule above |
| Consistency | Regra de consistência / Consistency rule | "Maior dia: 28 % do lucro total (limite da firma: 30 %)" + meter of that ratio | `warning` at 80 % of the threshold; never `reached` (never red/blocking). Tracking only |

   - Percent uses truncation (F§6.2): `80 %` is never shown while the Core state is `normal`. Color and badge come from the Core `state` field.
   - A rule that is **not set** is shown as a plain row, not a meter: "Perda diária: sem limite definido. O All Win não avisa nem recusa entradas por esta regra." with `Definir` (opens S13-02).
   - A rule that is **not confirmed** in the preset and unresolved: "Regra de consistência não confirmada — não acompanhada." with `Resolver` (S16 conflict picker). Nothing is warned from it (PO).
5. **What happens at the limit** (`Collapsible` "O que acontece quando eu atingir um limite?", closed): fixed text:
   - "O All Win recusa novas entradas nesta conta até o próximo reset, mesmo com a aplicação automática desligada. Saídas continuam permitidas."
   - "Com a aplicação automática ligada, o plugin fecha posições, cancela ordens e bloqueia a conta até o reset. Não há desbloqueio antecipado." / (link to the S15 screen or setting once it exists).
   - "A meta de lucro diária só avisa, a menos que você escolha fechar e bloquear."
   - "Regras de consistência só avisam; nunca bloqueiam."
   - Last line: link `Ver no registro de atividade` → `/activity?account={id}&category=risk` (S10).
6. **Recent risk events** (last 5 entries of category `risk` for this account, from S10 data): time with seconds, text, link to S10. Empty: "Nenhum evento de risco hoje."

**Live updates (SSE):** values refresh in place without layout shift. Only a **change of state** (normal → warning → reached, or target reached) is announced through a polite live region, and each state change also becomes a notification (S18). Ticks are not announced (F§7.3).

**States**

| State | UI |
|---|---|
| Loading | Skeleton: strip + 3 meter rows |
| No rules set yet | `EmptyState` `Gauge`: "Esta conta ainda não tem regras de risco." / "Defina limites para receber alertas aos 80 % e para o All Win recusar entradas ao atingi-los." + `Definir regras` |
| Error | `ErrorState` + retry + code |
| Stream down | strip shows `Sem dados recentes`; meters dimmed |
| Blocked | prominent `Alert` (danger, `Lock`, or success variant for target reason): "Conta bloqueada até 17:00 (GMT-3) · motivo: {reason}. Não há desbloqueio antecipado. Mudar limites não remove o bloqueio: ele continua até o reset." The block comes from S15; this screen only displays it |

---

## 4. S13-02 Edit risk rules — `/[locale]/accounts/[id]/risk/edit`

**Purpose:** set the rules (AC-1, AC-4, AC-5). **Layout:** single column, max 720 px, `Card` sections, sticky footer `Cancelar` / `Salvar regras`. On submit: focus the first invalid field; summary `Alert` when ≥ 3 errors (F§10).

**Prop accounts** open with the preset's values filled (S16 AC-2) and the provenance marks of F§7.4: verified (source popover), `Não confirmado` (not prefilled; control by reason, S5 §9.2), `Editado` (with `Restaurar valor do preset`). **Real accounts** have no preset: fields start empty, no marks.

### 4.1 Section "Perda e drawdown"

| Field | Component | Validation | Data |
|---|---|---|---|
| Drawdown — valor / amount | money `Input` (USD, F§6.3) | ≥ 0; ≤ 2 decimals; both drawdown fields set or both empty | `drawdown_amount` |
| Drawdown — tipo / type | `Select`: `Trailing intradiário`, `Trailing fim do dia`, `Estático` | required when an amount is set | `drawdown_type` |
| Limite de perda diária / Daily loss limit | money `Input` | ≥ 0 | `daily_loss_limit` |

- Help for the daily loss limit on prop accounts whose firm states it as **none**: "A firma informa que não há limite de perda diária." (field read-only, source popover).
- **Required fields (Q47 a, decided; AC-7):** for prop accounts the daily loss limit is required unless the preset says "none" explicitly (drawdown and maximum position are already required, S16 AC-6: a required field cannot be saved while not confirmed). For real accounts all are optional; an empty field means "não definido" / "not set" and the Risk tab says so (§3).

### 4.2 Section "Meta de lucro diária"

| Field | Component | Notes |
|---|---|---|
| Valor da meta / Target amount | money `Input`, optional | `daily_profit_target` |
| Ao atingir / When reached | `RadioGroup` of two cards, **"Só avisar" preselected** | `profit_target_on_reach`: `notify` (default, Q3 decided) / `flatten_and_block` |

- "Só avisar": "Você recebe um aviso uma vez quando a meta for atingida. A conta continua operando."
- "Fechar posições e bloquear até o reset": "Ao atingir a meta, o plugin fecha posições, cancela ordens e bloqueia a conta até o reset, como em um limite de perda." *(Q49 b, decided; AC-9)* The option is **unavailable while enforcement is off** on this account: `aria-disabled` (focusable, not removed) with the reason inline and in `aria-describedby`: "Disponível quando a aplicação automática estiver ligada nesta conta." / "Available when automatic enforcement is on for this account." + link `Ligar aplicação automática` (S15 control). If enforcement is later turned off while this option is saved, the target falls back to "só avisar" (S15 §4.2, Q63 provisional; closes Q-S13-8).
- The radio group is the one place in this spec where a default is preselected, because Q3 decided a default; the opt-ins of S5 are different (no default).

### 4.2b Section "Comissões" (AC-11)

| Field | Component | Validation | Data |
|---|---|---|---|
| Comissão por contrato e lado / Commission per contract per side | money `Input` (USD, F§6.3), optional | ≥ 0; decimals as the contract `MoneyAmount` (a preset value is kept exactly as published, up to 4 decimals; the client never rounds a prefilled value and accepts what the Core accepts) | `commission_per_contract_side` (account; `updateAccount`) |

- **Prefilled from the preset** when the firm publishes it, with the provenance mark of F§7.4 (verified popover with source and date, `Editado` when changed, `Restaurar valor do preset`); when the firm does not publish it the field is empty and shows the warning below. Real accounts have no preset: empty, no marks.
- Help: "O All Win usa, em cada execução, o maior valor entre a comissão informada pela plataforma e este valor × quantidade. Os limites usam o resultado líquido." / "For each fill, All Win uses the greater of the platform-reported commission and this value × quantity. Limits use the net result."
- **Empty:** inline warning "Comissão não informada. Sem ela o resultado líquido é igual ao bruto e os limites podem ficar folgados demais." (account-level wording; the per-trade label of S21 is "comissões não informadas", keep both exact).
- **Effect on save [Decided by the user 2026-10-24, Q85 (a)]:** a **higher** value applies at once; a **lower** value (or clearing it) applies from the next trading day, because it would loosen the net limits (contract `AccountRisk.pending_commission_change`). The form says so under the field: "Uma comissão menor vale a partir do próximo dia de trading ({time zone})." After saving, the Risk tab shows the pending line. The Core publishes a new limits version (S13 pending flow, `limits_pending` states as elsewhere).

### 4.3 Section "Regra de consistência" (prop accounts only)

- Shows the firm's rule **verbatim** (`FirmRuleQuote`, F§7.4) and the tracked threshold (`Input`, percent 0–100, integer or one decimal per the preset; edited values get `Editado`).
- Help: "O All Win acompanha esta regra e avisa aos 80 %, mas não bloqueia a conta." / "All Win tracks this rule and warns at 80%, but never blocks the account."
- Not confirmed: control by reason (S5 §9.2); unresolved → saved as not tracked ("Regra de consistência não confirmada — não acompanhada."). Real accounts: section absent.

### 4.4 Section "Dia de trading"

**Day start unknown (Q92 c, user 2026-10-24).** When the plugin cannot establish where the trading day started (platform closed at the boundary, no proven reconstruction) the Core blocks new automated entries (exits pass) and reports `routing.risk_blocked` with `risk_detail: day_unknown` (HTTP 6.0.0, committed on `d/contracts-s8`, not yet in `main`). The Risk tab shows a `warning` `Alert`: "O All Win não sabe quando o dia de trading começou. Novas entradas automáticas estão bloqueadas; saídas continuam." / "All Win doesn't know when the trading day started. New automated entries are blocked; exits continue." and the button `Reiniciar contagem de hoje`. It never happens automatically. The `AlertDialog` "Reiniciar a contagem de hoje?" says: "A contagem do dia recomeça agora. Perdas que o All Win não viu podem ficar de fora, e os limites podem ficar folgados demais." Buttons `Cancelar` / `Reiniciar contagem` (destructive-neutral); busy and disabled after the first click. **Step-up (Q96 yes):** action `trading_day_restart`, operation `restartTradingDayCount` (HTTP 6.0.0, committed on `d/contracts-s8`, not yet in `main`; optional `Idempotency-Key`), reactive 403 `auth.step_up_required` → `StepUpDialog` → one resend (F§8.2). Success: toast "Contagem reiniciada." and the block lifts; audit entry with who and when. This is distinct from the S13b evaluation restart.

**Position across the day boundary (Q93 a):** a position carried across the boundary counts its whole unrealized P&L in the day being measured. Help line under the day block: "Uma posição aberta na virada do dia conta todo o P&L não realizado no dia medido." / "A position open at the day boundary counts its whole unrealized P&L in the day being measured."

**Holidays (Q89 b, user 2026-10-24):** holidays count as normal trading days. Next to the day block, a help line states the limitation: "Feriados contam como dias normais de trading. O All Win não ajusta o dia para feriados." / "Holidays count as normal trading days. All Win doesn't adjust the day for holidays."

Read-only block: "Reset do dia: {time} ({zone})" with source. Prop: from the preset (value shown with the provenance mark; **the prop account may edit the reset time of its own copy, marked `Editado` (PO, S16 AC-2, S13 AC-4)**). Real: "Fechamento da sessão CME" (**no editing**, PO). The display zone follows the user's profile (F§6.4); the firm's zone appears in the tooltip.

### 4.5 Aplicação automática (read-only line)

"Aplicação automática: {Ligada | Desligada}" with `Alterar` linking to the S15 control, which lives with the S15 spec. Default off (S15 AC-1).

### 4.6 Save, validation and effects

| Situation | UI |
|---|---|
| Client-side invalid | Inline errors, focus first invalid, summary at ≥ 3 |
| Server 422 with `pointer` | Map to the field (`/daily_loss_limit`, `/drawdown_type`, `/profit_target_on_reach`, …) and focus it |
| Unresolved not-confirmed required value | "Resolva este valor para salvar." on the field |
| Tightening (a stricter value) | **Applies at once** (Q48 b, decided; AC-8). The form says so under the field: "Um limite mais apertado vale na hora." |
| Loosening (a looser value) | **Applies only from the next trading day.** If the account is **already blocked**, the edit never lifts the block; the form and the toast say "O bloqueio continua até o reset." (AC-8). Under the field: "Um limite mais folgado vale a partir do próximo dia de trading ({time zone})." After saving, the Risk tab shows the current limit and a line "Novo limite {value} a partir de {date-time zone}" (`pending_changes` from the Core) |
| New value already exceeded today (the Core returns `would_be_reached`) | `AlertDialog` before saving: "Com este limite a conta já passou do limite hoje. O All Win vai parar de aceitar novas entradas agora{; com a aplicação automática ligada, o plugin fecha as posições e bloqueia a conta até o reset}. Saídas continuam permitidas." Buttons `Voltar` (focused) / `Salvar e aplicar agora`. The dialog is shown only when the Core says the change would be effective now (tightening) |
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

S15 adds `risk.flatten`, `risk.blocked`, `risk.unblocked` and `risk.reactive_block_action` (S10 catalog; HTTP 6.1.0, registry 1.6.0). `enforcement_id` (HTTP 6.1.0, committed on `d/contracts-s8`, not yet in `main`) links `risk.blocked`, `risk.flatten`, `risk.reactive_block_action` and `risk.unblocked` of the same block; S10 can group them by it (S15 owns the screen).

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
| `would_be_reached` and `effective_from` (now / next trading day) per changed field for a proposed edit; `pending_changes` (value, `effective_from`) in `getAccountRisk` | needed by §4.6 (Q48); can be a validate-only call |

**Writes**

| Action | Notes |
|---|---|
| `putAccountRiskRules` (rules, `profit_target_on_reach`, `row_version`) | 422 pointers listed in §4.6; `common.version_conflict`; the Core bumps the limits `version` for the plugin |
| `restorePresetValue` (S16) | for edited fields |

**Events (SSE):** risk state changes per account (state, percent, stale), block/unblock, trading-day reset. Mobile background tabs refetch on return (F§9).

**Codes the UI maps (contract 3.4.1):** 422 `risk.daily_loss_required` (pointer `/rules/daily_loss_limit`), `risk.amount_invalid` (the field), `risk.required_value_unresolved` (pointer), `risk.flatten_requires_enforcement` (pointer `/rules/profit_target_on_reach`), `risk.reset_not_editable`; a half-filled drawdown is a schema error (400 `common.invalid_request` with the pointer, since type and amount are one object); `common.version_conflict`.

---

## 10. Decisions and open questions

**Closed by the PO (Rumo, 2026-10-23):** Q-S13-2 a prop account may edit the reset time of its own copy, marked `Editado`; a real account cannot (it uses the CME session close). Q-S13-6 one notification per state change.

**Architect (Compasso):** Q-S13-5 display of the trailing floor; Q-S13-7 percent precision of the consistency threshold.

**Decided by the user on 2026-10-23e (Q47, Q48, Q49), no longer provisional:**

| ID | Question | Decision |
|---|---|---|
| Q47 | Daily loss limit required? | Prop: required unless the preset says "none" explicitly (drawdown and max position already are); real: all optional, shown as "não definido" |
| Q48 | When does a limit edit take effect? | Tightening at once; loosening from the next trading day; if the new value is already exceeded: warn before saving, the Core stops routing entries, and with enforcement on the plugin flattens at once |
| Q49 | "Flatten and block" at the profit target | Unavailable while enforcement is off, with the reason |

**Closed:** Q-S13-8 — turning enforcement off returns "flatten and block" at the profit target to "só avisar"; the change is stated in the confirmation and audited (S15 AC-10, Q63).

## 11. AC traceability

| AC | Where |
|---|---|
| AC-1 rules per account (drawdown amount/type, daily loss, daily profit target, consistency) | §4 |
| AC-2 current risk state, used vs limit, per account | §3 |
| AC-3 80 % warning in the Web (plugin: S13 plugin panel) | §3, §5 notifications |
| AC-4 trading-day reset (firm / CME) | §3 strip, §4.4 |
| AC-5 profit target: notify only (default) or flatten and block; notify once | §4.2, §3 meter |
| AC-6 consistency tracked and warned, never enforced | §3, §4.3 |
| AC-7 prop requires drawdown and daily loss unless the preset says none; real all optional | §4.1 |
| AC-8 tighten at once, loosen next day, never lifts a block, warning before saving | §4.6 |
| AC-9 flatten-and-block unavailable until enforcement is on | §4.2 |
| AC-10 reset time editable on prop copy, real fixed | §4.4 |
| AC-11 commission floor, prefilled from preset, "comissão não informada" | §4.2b, §3 |
| S13b AC-1..AC-6 evaluation progress | §12 |
| F§7.1 three levels, no 90 % band; F§7.3 RiskMeter | §2, §3, §8 |
| X-2, X-3, X-5, X-6, X-8 | §3 permissions, strings from the Core, §7, §6, time rules |

---

## 12. V1-S13b — Evaluation progress (profit target left to pass)

**Purpose:** on an account in an **evaluation** phase, show how much net profit is left to reach the firm's evaluation profit target (S13b AC-1). Funded and live phases, and real accounts, show nothing (no empty card).
**Entry point:** card "Meta da avaliação" on the Risk tab (§3) and one line on the account detail header (S5 §8): "Avaliação: faltam US$ 1.200,00". **Permissions:** own account only (X-2).
**Contract:** `getAccountRisk` → `AccountRisk.evaluation` (`start`, `profit_target`, `earned_before`, `achieved`, `remaining`, `percent`, `state`, `reached_at`, `as_of`); `restartEvaluation` (`POST /accounts/{id}/evaluation/restart`, `evaluation_start`, `evaluation_earned_before`, `row_version`); account fields `evaluation_start` / `evaluation_earned_before` (create/update); preset `evaluation_profit_target` (S16 AC-10). Notification `risk.evaluation_target_reached` (S18).

### 12.1 Card "Meta da avaliação" (AC-1, AC-2, AC-3)

`Card` with:
- Heading and a progress bar (`RiskMeter`-style but **success** tone, not a limit: it never uses warning/danger; `role="progressbar"` with text value; the percent is the Core's truncated `percent`).
- Three figures (definition list): **Meta** US$ 6.000,00 · **Acumulado (líquido)** US$ 4.800,00 · **Falta** US$ 1.200,00. In `target_reached`: "Meta atingida em {date-time zone}" in place of "Falta" (never a negative amount). All values are strings from the Core (F§6), no arithmetic in the browser.
- Count line: "Contagem desde {start}{; inclui US$ {earned_before} já ganhos antes}. Só entram operações capturadas pelo All Win a partir dessa data, mais o valor informado. Operações lançadas à mão não contam." (AC-2, AC-4).
- Source line for the target (from the preset): "Meta da firma: US$ 6.000,00 · fonte {page title}, verificado em {date}" with the F§7.4 popover.
- Actions: `Ajustar contagem` (S13b-02).
- **Header line (PO, Q-S13b-2):** the account detail header shows "Avaliação: faltam {remaining}"; **hidden** when the target is not confirmed or absent.
- **Not confirmed or absent target (AC-3):** the card shows no progress and says why: "A meta desta avaliação não está confirmada, então não mostramos o progresso." / "This evaluation's target is not confirmed, so we don't show progress." with `Resolver` / `Resolve`, which opens the same control by reason as S16 §5 / S5 §9.2 (reason from `PresetRules.not_confirmed_reason`; HTTP 6.3.0 **TODO** until committed): `conflicting` → `ConflictPicker`; `partial` or `not_published` → the field only. The product never picks a value. **Absent target on an evaluation account** (the preset has no published evaluation target): the card is shown with the explanation "Esta avaliação não tem meta publicada, então não mostramos o progresso." / "This evaluation has no published target, so we don't show progress." (no `Resolver`). **No field to type a target in V1** (Rumo, 2026-10-24; provisional until the user decides), although the contract allows the user to type one. A funded or live phase has no card. **The card is driven by `AccountRisk.evaluation_target_status`** (HTTP 5.4.0 / 6.0.0, committed on `d/contracts-s8`, not yet in `main`): `set` → progress card from `evaluation`; `not_confirmed` → the "não está confirmada" text with `Resolver`; `absent` → the "não tem meta publicada" text; field missing → no card (not an evaluation account).
- Stale data: `Sem dados recentes` badge (F§7.2) like the meters; `as_of` shown as absolute time.
- Net-commission note (keeps the user from being misled): `commission_source = none` → "Comissão não informada: o acumulado pode estar acima do real."; `configured` or `mixed` (estimated from the configured minimum) → "Comissão estimada pelo mínimo configurado: o acumulado pode estar acima do real." `platform` and `manual`: no note.

### 12.2 Dialog "Ajustar contagem da avaliação" (AC-4)

`ResponsiveDialog` with one action. After creation, the start date and the amount already earned change **only through `restartEvaluation`** (the contract has no other edit; they exist in `AccountCreateRequest` only), so the dialog is a restart:

| Field | Component | Validation | API |
|---|---|---|---|
| Início da contagem / Count start | date `Input` (F§6.4, user time zone), default the current start | required; not in the future (422 `accounts.evaluation_start_invalid`, pointer `/evaluation_start`) | `evaluation_start` |
| Já ganho antes do início (líquido) / Already earned before the start (net) | money `Input`, optional | ≥ 0 (V1: a loss already made before the start is not recorded; if the user wants it, that is a PO/contract change); decimals as `MoneyAmount` (see §4.2b) | `evaluation_earned_before` |

- Text above the fields: "Só entra o que o All Win capturou a partir da data de início, mais o valor que você informar aqui. Uma reinicialização da simulação ou do Playback não reinicia a contagem." (AC-4, S11 AC-6).
- Button `Reiniciar contagem` opens an `AlertDialog` "Reiniciar a contagem da avaliação?": "A contagem passa a valer a partir da data e do valor acima. O histórico anterior continua no registro de atividade." Confirm → `restartEvaluation` `{evaluation_start, evaluation_earned_before?, row_version}`; audited as `accounts.evaluation_restarted` (previous and new start). No step-up (not in the S3 list).
- Errors: pointers map to fields; 409 `common.version_conflict` → "A conta mudou. Recarregue." + `Recarregar`; 422 `accounts.evaluation_not_applicable` → dialog closes with an `Alert` "Esta conta não tem meta de avaliação." Double submit prevented (busy button). Dirty guard like other forms. Focus on the first field; returns to `Ajustar contagem`.
- If the user wants to edit without restarting (change the earned amount only), that needs a contract operation first: not drawn **[Q-S13b-3 to the Architect]**.

### 12.3 Reaching the target (AC-5)

When `state` becomes `target_reached`: **one** notice on the account (the card turns to success with a `Check` icon and "Meta atingida", `role="status"` announced politely once) and **one** notification (`risk.evaluation_target_reached`, success, S18). **Nothing else happens**: the text says "Nada é fechado nem bloqueado automaticamente." No `CriticalBanner`, no block, no flatten. Minimum trading days and payout rules are not shown (AC-6).

### 12.4 States

Loading `Skeleton`; error `ErrorState` with retry; no evaluation → no card; target not confirmed → §12.1 explanation; in progress; reached; stale.

### 12.5 Accessibility and i18n

Keys `risk.evaluation.*` (pt-BR = en). The bar has a text equivalent ("80 % da meta"); status by text and icon, not color alone; the dialog is keyboard operable and reflows at 320 px; the figures are a `dl`.

### 12.6 Open questions (S13b)

| Id | Question | Owner |
|---|---|---|
| Q-S13b-1 | The registration-date default of `evaluation_start` and the `evaluation_earned_before` field in the **create** account form (S5 step "Regras") — spec proposes both optional in the S5 rules step, defaults as in the contract. S5 needs a one-line addition (Nanquim/Aquarela after the S5 delta commit). | Aquarela |
| Q-S13b-3 | Edit the earned amount or start without restarting: no contract operation (only `restartEvaluation`). | Architect |
| Q-S13b-2 | **Answered by the PO:** the header shows "faltam", hidden when not confirmed or absent (§12.1). | PO |
