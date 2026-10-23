# V1-S5 — Account registration, lifecycle and per-account opt-ins — Screen spec

> **Owner:** Product Designer (Traço @ Dev3, on the Aquarela spec rev. 2) · **Status:** proposed, pending PO review · **Revision:** 3 · **Date:** 2026-10-23
> **Slice:** [V1-S5](../../product/release-1/slice-plan.md) AC-1 … AC-11 (slice-plan rev. 7), with touch points in S7 (routing), S10 (audit), S16 (presets) and S19 (followers).
> **Foundations:** [foundations.md](../foundations.md). Section numbers like "F§7.2" point there.
> **Contract:** [openapi.yaml](../../../contracts/http/openapi.yaml) 1.1.0 (`setAccountOptIn`, `OptInChoiceRequest`, `OptInChoices`, `OptInState`, `FirmStance`), [s5-design-deltas.md](../../architecture/contracts/s5-design-deltas.md) (D1–D16), [wave1-data-model.md §5.2–§5.3](../../architecture/data/wave1-data-model.md). **Where this spec and the contract differ, the contract wins.** The rest of the S5 HTTP contract (list/create/update/lifecycle/delete) does not exist yet: §14 lists what these screens need so the Architect can close it. Operation names in this spec for those are **design proposals**.
> **Rev. 3 (2026-10-23):**
> - Deltas D1–D16 applied (names, atomic create-with-choice, five `FirmStance` values, conditional step-up, Q24/Q25/Q26 approved, provisional text-change trigger).
> - Removed: the "saved without a choice" state, its alert and `Escolher agora`, and the Architect question about it.
> - Added: account list, create/edit form for prop and real accounts, platform-account binding, lifecycle (pause/disable), delete, account detail, `copy` variant of the section, data/action list for the contract.
> - Step-up wording no longer says "password" (S3 is passwordless since slice-plan rev. 7); F§8.2 rev. 3 matches.
> - Rev. 3b: PO answers applied (Q-S5-3/5/8/9/11 closed; Q37–Q42 drawn as provisional recommendations); Aquarela confirmed §0 with adjustments.
>
> **Basis:** user decisions of 2026-10-23 in [decisions.md](../../delivery/decisions.md): option A, Q24, Q25, Q26, Q12, Q-A1. Business rules come only from the slice plan; gaps are in §17.
> **Not legal advice.** All Win shows the firm's own text; summaries are labeled as All Win's reading (vision §12).
> **Surfaces:** Web (Frontend). Core rules (routing skip, audit, step-up) are Backend's. QA owns the checks.

---

## 0. Information architecture added by wave 2 (all wave-2 specs use it)

**Confirmed by Aquarela (design system owner)** with the adjustments below; the S3 tree stays valid.

```
App sidebar (additions in bold)
  Início · Contas · Roteamento · Atividade · Perfis de stop e alvo · Presets · Conexões ·
  Configurações · Administração (admin only)          (order confirmed by Aquarela)
  Contas / Accounts                       /[locale]/accounts                 (S5)
  Roteamento / Routing                    /[locale]/routing                  (S7)
  Atividade / Activity                    /[locale]/activity                 (S10)  audit log
  Perfis de stop e alvo / Stop and target profiles   /[locale]/profiles    (S6)
  Presets                                 /[locale]/presets                  (S16, top-level until S16 is drawn)
```

Account-scoped tabs are **paths**, not query strings (deep link, Back button): `/accounts/[id]` (Resumo), `/accounts/[id]/risk` (S13), `/accounts/[id]/profile` (S6), `/accounts/[id]/routes` (S7).

---

## 1. What this spec covers

| # | Screen / element | Route |
|---|---|---|
| S5-01 | Account list | `/[locale]/accounts` |
| S5-02 | Create / edit form (prop and real) | `/[locale]/accounts/new`, `/[locale]/accounts/[id]/edit` |
| S5-03 | Section "Ordens automatizadas" (`automation`) and its `copy` variant | inside S5-02, S5-04, S7 add-follower flow |
| S5-04 | Account detail (summary tab) incl. opt-in cards | `/[locale]/accounts/[id]` |
| S5-05 | Change / review an opt-in (dialog) | from S5-04, S5-01 |
| S5-06 | Pause / resume / disable / enable | from S5-01, S5-04 |
| S5-07 | Delete account | from S5-01, S5-04 |
| S5-08 | Preset-dependent fields in the form (firm platform, "not confirmed") | inside S5-02 (picker itself: S16 spec) |

Routing screen behavior for accounts that do not accept automation is specified in the **S7 spec**; audit rendering in the **S10 spec**. This spec defines the events S5 emits (§13).

---

## 2. Global rules for this slice

- **Isolation (X-2):** an account id that is not the user's yields the same `ErrorState` as a non-existent id (`common.not_found`). Never distinguish.
- **Money and numbers (X-3):** size is a decimal string; max position and multiplier are integers. Input per F§6.3. The browser does no arithmetic.
- **Duplicate submit:** F§10 "Submitting" on every form and dialog. Create is sent with a client-generated idempotency key (§14), so a double click never creates two accounts.
- **Optimistic concurrency:** edits send the account `row_version`. On conflict (`common.version_conflict`, proposed) show `Alert`: "Esta conta foi alterada em outro lugar. Recarregue para ver os valores atuais." + `Recarregar`. Typed values are kept until the user reloads.
- **Simulation only (X-7):** outside production the Core refuses to bind `live` or `unknown` platform accounts (data model §5.3). The picker shows them disabled with the reason (§4.3).
- **Time:** all timestamps with zone (F§6.4).
- **Standard states** used by every screen below unless a section says otherwise:

| State | UI |
|---|---|
| Loading | `Skeleton` in the final layout (F§10) |
| Request error (page) | `ErrorState` + `Tentar novamente` + code in small mono text |
| Not found / not yours | `ErrorState` "Conta não encontrada." / "Account not found." (`common.not_found`) with link to the list |
| Session ended | redirect to login with `reason=session_expired` (F§10) |
| Step-up required | `StepUpDialog` (F§8.2) asks for the code; the original request goes out once with `challenge_id` + `code`; cancel saves nothing (§12) |

---

## 3. S5-01 Account list — `/[locale]/accounts`

**Purpose:** see every account at a glance and reach its actions (AC-1, AC-4).
**Permissions:** any signed-in user, own accounts only. Archived ("deleted") accounts are not listed.
**Layout:** `PageHeader` (title `Contas` / `Accounts`, primary action `Nova conta` / `New account`) · filter bar · `Table` (≥ `md`) or `Card` list (< `md`, F§9).

**Filters** (client-side on the loaded list; list is small): free-text search on name; `Tipo` (Todas · Prop · Real); `Estado` (Todas · Ativa · Pausada · Desativada · Escolha pendente). The `Escolha pendente` option is the target of the S18 notification link (`?state=choice_pending`). Filter values live in the URL query.

**Columns** (table) / fields (card):

| Column | Content | Notes |
|---|---|---|
| Nome / Name | `display_name` (link to S5-04) | Primary field of the card |
| Tipo / Type | `Prop` or `Real` | |
| Firma · plano · fase · tamanho | `Apex · 50K Rithmic · Avaliação · US$ 50.000,00`; real: `US$ 25.000,00` | Prop only for firm/plan/phase |
| Conta na plataforma | `{platform_account} · {connection name}` + kind badge (`Simulação`, `Playback`, `Live`, `Desconhecida`); `Sem vínculo` if unbound | |
| Mult. | `2×` | F§6.2 |
| Máx. posição | `3 contratos` | |
| Estado | `StatusBadge` per F§7.2 priority; top one plus `+N` popover | Shows: out of sync, disconnected, blocked, risk warning (when S11/S13 deliver them), `Escolha pendente`, `Pausada`/`Desativada`. `Ativa` shows no badge; the row reads "Ativa" as text |
| Ações | kebab `DropdownMenu` | `Ver`, `Editar`, `Pausar`/`Retomar`, `Desativar`/`Ativar`, `Excluir` (destructive, last, separated) |

Default sort: name A→Z. No pagination in V1 (a user has tens of accounts); virtualize only if QA measures a problem.

**States**

| State | UI |
|---|---|
| Empty (no accounts) | `EmptyState` icon `Landmark`: "Você ainda não cadastrou contas." / "Contas recebem as ordens roteadas. Cadastre ao menos uma para começar." + `Nova conta`. If the user has no paired connection yet, a second line "Pareie o NinjaTrader para vincular contas." + link to `/connections/ninjatrader` |
| Empty after filter | "Nenhuma conta com esses filtros." + `Limpar filtros` |
| Row action in progress | row menu disabled, badge `Salvando…`; no layout shift |
| Row action failed | toast (destructive) with translated code; row returns to the previous state |

---

## 4. S5-02 Create / edit form — `/[locale]/accounts/new`, `/[locale]/accounts/[id]/edit`

**Purpose:** register an account with what the platform needs to route safely (AC-1 … AC-7, AC-9b, AC-10).
**Entry:** `Nova conta` (list header, empty state, Getting-started card); `Editar` (list, detail).
**Layout:** single column, max 720 px, sections as `Card`s in this order. A sticky footer with `Cancelar` and `Salvar conta` (primary) never covers the focused field (F§9). On submit: focus the first invalid field; error summary `Alert` when ≥ 3 errors (F§10).

### 4.1 Section "Tipo" (create only)

`RadioGroup` (cards ≥ 44 px): `Conta de prop firm` / `Prop firm account` · `Conta real em corretora` / `Real brokerage account`. Nothing preselected; choosing reveals the rest of the form. **[Proposal]** The type is **not editable** after creation (the data model ties prop-only columns to `kind`); the edit form shows it as read-only text. To change type the user creates a new account.

### 4.2 Section "Dados da conta"

| Field | Component | Validation | API / data |
|---|---|---|---|
| Nome / Name | `Input`, `maxLength` 80 **[proposal]** | required; unique per user, case-insensitive → `accounts.name_taken` inline "Já existe uma conta com este nome." | `display_name` |
| **Prop only** — Firma / Firm | `Select` (searchable `Command` in `Popover` for 18+ options), options from the catalog (S16) | required | `firm_id` |
| Plano / Plan | `Select`, options depend on firm | required | `plan_name` |
| Fase / Phase | `Select` (Avaliação · Financiada · Live — as the preset publishes) | required | `phase` |
| Tamanho / Size | prop: `Select` of the plan's sizes; real: money `Input` (USD, ≤ 2 decimals, > 0, F§6.3) | required; AC-2 | `size_amount` |
| Plataforma da conta na firma / Firm account platform | `Select`, only when the preset has platform-specific rules (§9.1) | required when shown | `firm_platform` (reserved, data model §5.2) |
| Máximo de contratos / Maximum position | `Input inputMode=numeric` | required (AC-3); integer ≥ 1; **prop:** prefilled with the preset maximum, cannot exceed it → "O máximo desta conta na {firm} é {n} contratos." **real:** user sets it, no prefill | `max_position_qty` |
| Multiplicador / Multiplier | `Input inputMode=numeric`, default `1` | integer ≥ 1; fraction or zero refused (AC-5) → "Use um número inteiro maior ou igual a 1." | `multiplier` |

Help under *Máximo de contratos*: "Limite de contratos que o All Win envia a esta conta. Instruções acima disso são cortadas e registradas." (vision §8). Under *Multiplicador*: "Quantidade executada = quantidade da instrução × multiplicador." (S7 AC-3).

For a **real** account under "Tamanho" a read-only line: "Perfil de stop/alvo inicial: tamanho padrão mais próximo ({size})." (value from the Core; the profile itself is S6).

### 4.3 Section "Conta na plataforma" (link, AC-1)

- `Select` "Conta no NinjaTrader" grouped by connection (`Connection.name`, with its status badge F§7.2). Options come from `visible_accounts` of the user's paired connections, label `{platform_account}` + kind badge.
- Options already bound to another All Win account: disabled, "Já vinculada a {account name}." (one platform account ↔ at most one account).
- `live` / `unknown` kinds outside production: disabled, "Contas live não são permitidas neste ambiente." / "Live accounts are not allowed in this environment."
- Read-only line after choice: "Conexão de ordens: {local | servidor | desconhecida}" with the S9 limit text (S9 AC-4) when `oco_location` is `local` or `unknown`: "Os stops e alvos desta conta dependem do NinjaTrader aberto." / "This account's stops and targets depend on NinjaTrader staying open."
- **The binding is optional at creation [provisional, Q37 rec. (a); user decides].** Unbound accounts save, appear as `Sem vínculo`, and cannot receive routed instructions; S7 shows them unavailable with the reason.
- No paired connection: the select is replaced by an info `Alert` "Nenhuma conexão pareada. Você pode salvar a conta agora e vincular depois." + link to pairing.
- Disconnected connection: its accounts remain selectable (last known list) with the note "Conexão desconectada: dados da última vez."

### 4.4 Section "Ordens automatizadas nesta conta" (opt-in)

Specified in **§5**. Shown only for prop accounts after firm, plan, phase, size and (when relevant) firm platform are selected (§5.1).

### 4.5 Save, validation and result

| Situation | UI |
|---|---|
| Client-side invalid | Inline errors under fields; focus first invalid; summary `Alert` ≥ 3 errors |
| Server 422 with `pointer` | Map the JSON pointer to the field (`/display_name`, `/max_position_qty`, `/opt_ins/automation`, `/opt_ins/copy`, …) and focus it. `/opt_ins/*` focuses the radio group of that kind (D10) |
| Step-up required (a choice is `accepted` and changes the stored state, D9) | `StepUpDialog` **before** the single save. Cancel: nothing saved, all form values and choices kept |
| 409 `accounts.rule_version_changed` | §5.4 (D4, D11) |
| 409 `common.version_conflict` (edit) | §2 |
| Network or 5xx | destructive `Alert` above the footer with the code; values kept; button re-enabled |
| Success (create) | Toast "Conta criada." → account detail. If a platform account was bound, detail shows it |
| Success (edit) | Toast "Conta salva." → account detail |
| Cancel with unsaved changes | `AlertDialog` "Descartar alterações?" (`Continuar editando` focused / `Descartar`) |

**Edit rules**

- Fields editable: name, multiplier, max position, platform-account binding, and (prop) firm/plan/phase/size/firm platform.
- Changing firm, plan, phase, size or firm platform may select a different rule version. The form then clears the affected choices (§5.2, D16) and asks again; the update carries new `opt_ins` for the affected kinds or the Core answers 422 `accounts.opt_in_required`.
- Changing the bound platform account of an account **with an open position** is **blocked in the UI** with "Há posição aberta. Feche a posição para trocar a conta vinculada." **[provisional, Q38 rec. (a); user decides]**.
- A shrinking *Máximo de contratos* below the current open position is allowed; the note "Posições abertas não são alteradas." appears under the field.

---

## 5. S5-03 Section "Ordens automatizadas" (`automation`) and its `copy` variant

One reusable component `OptInSection` with `kind: "automation" | "copy"`. It renders from the `OptInState` the Core returns for the selected preset version (`firm_stance`, `state`, `current_rule_version_id`, previous `choice` when `pending_review`).

### 5.1 When it appears (D12, AC-10)

| Account / `firm_stance` for the kind | UI |
|---|---|
| Real account | No section. No question |
| Prop, `allowed` | Compact info line + `FirmRuleQuote` collapsed behind `Ver regra oficial`. No question. automation: "A {firm} permite negociação automatizada." · copy: "A {firm} permite a replicação de operações entre contas." |
| Prop, `forbidden` · `conditional` · `unknown` · `not_confirmed` | **Required choice** (§5.3), nothing preselected |
| Prop without a preset (before S16) | **[Provisional, Q40 rec. (a)]** Firm, plan and phase are free text and the maximum position is typed. The stance is treated as `unknown`, so the choice is **required** with the generic warning of the `unknown` summary and no quote ("Não encontramos uma regra oficial da {firm} sobre automação."). Attaching a preset later gives `pending_review` (§7) |

The section appears only after firm, plan, phase, size and firm platform (when relevant) are selected, because the rule version depends on them. The `automation` section is part of the S5 form. The **`copy` section is not part of the S5 create/edit form**: it appears when the account is added as a follower (§5.6, D13).

### 5.2 Rule selection changes (D16)

If the user changes firm, plan, phase, size or firm platform after choosing, and the Core returns a different `current_rule_version_id`, the choices of the affected kinds are **cleared** and an info `Alert` explains: "A regra mudou com a sua seleção. Escolha de novo." The Core is asked for the new `OptInState` on each such change (so the quote and `firm_stance` are always for the displayed selection).

### 5.3 Structure (DOM order = reading order; warning before choice)

```
┌ fieldset ──────────────────────────────────────────────────────────────┐
│ legend: Ordens automatizadas nesta conta                                │
│ help:   Fontes automatizadas: alertas do TradingView (webhook) e, no    │
│         futuro, o motor de sinais do All Win. A replicação da conta     │
│         master tem uma escolha própria.                                 │
│                                                                         │
│ Resumo do All Win (não é aconselhamento jurídico):                      │
│ A Apex Trader Funding proíbe negociação automatizada nas regras         │
│ oficiais. Alertas do TradingView executados automaticamente podem ser   │
│ considerados automação.                                                 │
│ ┃ "No Automation or Algorithm Usage allowed: …"                        │ ← FirmRuleQuote (lang=en)
│ ┃  Apex Trader Funding · Prohibited Activities · acessado em 23/10/2026│
│ ┃  [Ver página oficial ↗]   [Mostrar texto completo]                   │
│ ⚠ Risco: se a firma considerar isso uma violação, você pode perder     │ ← Alert (danger)
│   esta conta. A decisão é sua; o All Win não decide isso por você.     │
│ ( ) Não aceitar ordens automatizadas                                    │ ← RadioGroup, required
│ ( ) Aceitar ordens automatizadas, ciente do risco                       │
│ [ ] Li a regra oficial acima e entendo que posso perder esta conta.    │ ← only when "Aceitar"
└─────────────────────────────────────────────────────────────────────────┘
```

**Summary line** — fixed, reviewed copy per `FirmStance` (D6); never free text. The second sentence of the automation summary is always "Alertas do TradingView executados automaticamente podem ser considerados automação."

| `FirmStance` | automation (pt-BR / en) | copy (pt-BR / en) |
|---|---|---|
| `forbidden` | "A {firm} proíbe negociação automatizada nas regras oficiais." / "{firm} prohibits automated trading in its official rules." | "A {firm} proíbe a replicação de operações entre contas nas regras oficiais." / "{firm} prohibits copying trades between accounts in its official rules." |
| `conditional` | "A {firm} só permite negociação automatizada sob condições. Leia a regra." / "{firm} only allows automated trading under conditions. Read the rule." | "A {firm} só permite a replicação sob condições. Leia a regra." / "{firm} only allows copying under conditions. Read the rule." |
| `unknown` | "Não encontramos uma regra oficial da {firm} sobre automação." / "We found no official {firm} rule about automation." | "Não encontramos uma regra oficial da {firm} sobre replicação." / "We found no official {firm} rule about copying." |
| `not_confirmed` | "As páginas oficiais da {firm} dizem coisas diferentes sobre automação." / "{firm}'s official pages disagree about automation." | "As páginas oficiais da {firm} dizem coisas diferentes sobre replicação." / "{firm}'s official pages disagree about copying." |

copy second sentence: "A replicação da conta master para esta conta pode ser considerada cópia de operações pela firma." / "Replicating the master account to this account may be considered trade copying by the firm."

**Quotes (O7):** one `FirmRuleQuote` (F§7.4) per official quote the rule version holds, in the firm's language, never translated. `not_confirmed`: all conflicting quotes under "As páginas oficiais divergem:", each with its own source and date. `unknown`: no quote; text "Pesquisamos as páginas oficiais em {date} e não encontramos uma regra sobre automação." + the research sources listed. Quote data for the copy kind comes from the same rule version.

**Risk warning (fixed copy):** `Alert` destructive, `TriangleAlert`: "Risco: se a firma considerar isso uma violação, você pode perder esta conta. A decisão é sua; o All Win não decide isso por você." / "Risk: if the firm considers this a violation, you may lose this account. The decision is yours; All Win doesn't make it for you."

**Choice:** shadcn `RadioGroup`, required, **no option preselected** (no default), including after a reset and in `pending_review`. Options are full-width cards, ≥ 44 px, `aria-describedby` → the risk warning.

| Option | `choice` | automation label / description | copy label / description |
|---|---|---|---|
| decline | `declined` | "Não aceitar ordens automatizadas" / "Don't accept automated orders" — "Alertas do TradingView nunca serão enviados para esta conta. Se algum chegar, ele é ignorado e registrado na auditoria." | "Não aceitar replicação da conta master" / "Don't accept master replication" — "Esta conta não poderá ser seguidora de uma conta master." |
| accept | `accepted` | "Aceitar ordens automatizadas, ciente do risco" / "Accept automated orders, aware of the risk" — "Alertas do TradingView poderão ser executados nesta conta conforme suas rotas." | "Aceitar replicação, ciente do risco" / "Accept replication, aware of the risk" — "Operações da conta master poderão ser replicadas nesta conta." |

Acknowledgement `Checkbox` (only after `accepted`, required): "Li a regra oficial acima e entendo que posso perder esta conta." / "I read the official rule above and understand I may lose this account." Sent value: only `choice` and `rule_version_id`; the checkbox is a client-side gate.

### 5.4 Validation, save, stale rule (D4, D9, D10, D11)

| Situation | UI |
|---|---|
| Save with no choice | Inline error under the legend: "Escolha se esta conta aceita ordens automatizadas." Focus to the radio group; included in the error summary. Server 422 `accounts.opt_in_required` with pointer `/opt_ins/automation` or `/opt_ins/copy` maps to the group of that kind and focuses it |
| `accepted` without the checkbox | Inline error on the checkbox: "Confirme que leu a regra para aceitar." |
| Save with `accepted` that changes stored state | Step-up before the single save (D9, both kinds). `declined`: no step-up, no extra dialog |
| 409 `accounts.rule_version_changed` (create, update or `setAccountOptIn`) | Body has `error.current_rule_version_id`. The section reloads the quote for that version, **clears the choice**, shows an `Alert`: "A regra da firma foi atualizada enquanto você preenchia. Leia de novo e escolha." Other fields keep their values; nothing was saved |
| 422 `accounts.opt_in_not_applicable` (D5) | Refetch; the section disappears; **no error shown** (only happens with stale data) |
| Success | Toast "Conta salva." (create-with-choice is one save; there is no partial state) |

The request carries `opt_ins.automation = { choice, rule_version_id }` where `rule_version_id` is the version whose text was displayed (O6).

### 5.5 Account detail cards (O4) and states

On S5-04, one card per kind where `state ≠ not_required`: **Automação** (`automation`) and **Replicação** (`copy`, only when the account has or had a copy choice or is a follower).

| `state` / `choice` | Badge (F§7.2) | Card content |
|---|---|---|
| `chosen` + `accepted` | `Aceita automação` (info, `Bot`) / copy: `Aceita replicação` | "Escolhido por você em {date-time zone} · regra da {firm} versão {n} (acessada em {date})" · `Ver regra oficial` (collapsed quote of the version chosen) · link `Ver no registro de auditoria` (S10, filtered to this account and event) · `Alterar escolha` |
| `chosen` + `declined` | `Sem automação` (neutral, `BotOff`) / `Sem replicação` | same |
| `pending_review` | `Escolha pendente` (warning, `CircleHelp`) | warning `Alert`, see §6 · `Revisar agora` |
| `not_required` + `firm_stance = allowed` | no badge | info line from §5.1 |

New terms to add to F§5: `Aceita replicação` / `Accepts replication`, `Sem replicação` / `No replication` (request to Aquarela, §17 Q-S5-11, closed).

### 5.6 `copy` variant at the follower moment (D13, Q24)

- **Where:** when an account is added as a follower of a master (S7 follower picker / S19). It is the same `OptInSection` with `kind = "copy"`, inside a `ResponsiveDialog` titled "Adicionar {account} como seguidora" / "Add {account} as a follower", shown only if the follower's copy `state ≠ not_required` and no valid choice exists.
- **Atomic:** the dialog's `Salvar` sends the copy choice together with the subscription in **one request** (S7/S19 contract); step-up if `accepted`; cancel adds nothing.
- **Declined:** the account cannot be subscribed as a follower (S5 AC-9b). The dialog closes with the subscription not created and the toast "Esta conta não aceita replicação e não foi adicionada." The S7 spec shows the combination unavailable with the reason.
- Same states, same quote/risk/step-up/409 handling as §5.3–§5.4, with copy wording.

---

## 6. Rule changed: suspended until reviewed (Q26) — S5-05

Drawn for Q26 option (b), **approved for a stance change**. A material change of the official **text** with the **same stance** also gives `pending_review`: **provisional Architect decision, awaiting user confirmation (D14)**. The UI treats both causes the same way and labels the cause.

- **Account list:** badge `Escolha pendente` (warning), F§7.2 priority after risk warning.
- **Account detail:** warning `Alert` at the top: automation: "A regra da {firm} sobre automação mudou em {date} (versão {n}). Ordens automatizadas estão suspensas nesta conta até você revisar." · copy: "…sobre replicação mudou… A replicação para esta conta está suspensa até você revisar." + `Revisar agora`.
  - Cause line under the text: stance change → "O que mudou: a posição da firma passou de {old} para {new}." · text change → "O que mudou: o texto da regra oficial foi alterado. Motivo informado: {reason}." / "What changed: the official rule text was edited. Reason given: {reason}." *(Provisional, Q42 rec. (a): a material change is flagged by whoever publishes the version, with a reason; user decides.)*
- **Review dialog (`ResponsiveDialog`)**: the §5.3 section for the new version, plus a `Collapsible` "Texto anterior (versão {m})" with the old quote for comparison, and a line "Sua escolha anterior: {Aceitar | Não aceitar}" (from `choice` of the `pending_review` state; shown as text, **never preselected**). `Salvar` calls `setAccountOptIn` (`PUT …/opt-ins/{kind}`).
- **No previous choice:** a `pending_review` state with no previous choice (D15) shows no "Sua escolha anterior" line.
- **Notification (S18):** "Regra de automação da {firm} mudou: {k} contas com ordens automatizadas suspensas." → `/accounts?state=choice_pending`. The Core supplies the text and count; the web only links.
- **Routing:** these accounts show the pending reason (S7 spec).
- **Interaction with S16 AC-3** (a new preset version is applied only if the user chooses): the design assumes the stance/text change suspends **even before** the user applies the new version to the account **[confirmed by PO: S5 AC-11, Q26b]**.

## 7. Prop accounts created before S16, or when a preset is attached (D15, Q41 provisional)

When S16 lands (or the user attaches a preset to a prop account), existing prop accounts whose stance needs a choice show `state = pending_review` with no previous choice: badge `Escolha pendente`, same detail alert (copy: "Esta conta ainda não tem escolha sobre ordens automatizadas. Elas ficam suspensas até você escolher.") and `Revisar agora`. Sim-only today, so no real account is affected. **[provisional, Q41 rec. (a): attaching a preset gives `pending_review`; user decides]**

---

## 8. S5-04 Account detail — `/[locale]/accounts/[id]`

**Purpose:** the account's state, configuration and actions in one place.
**Layout:** `PageHeader` (name, type/firm line, state badges, actions: `Editar`, lifecycle button, kebab with `Excluir`). Tabs: `Resumo` (this slice) · `Risco` (S13) · `Perfil` (S6) · `Rotas` (S7); a tab with no data yet is hidden until its slice ships. On mobile the tabs become a horizontally scrollable `TabsList` (≥ 44 px).

**Resumo cards** (order):

1. **Alerts**: out of sync, pending choice (§6), paused/disabled explanation (§10). Most severe first (F§7.2).
2. **Dados:** type, firm · plan · phase · size · firm platform ("Plataforma na firma: Rithmic"), max position, multiplier, created at.
3. **Conta na plataforma:** platform account, connection name + status, kind badge, OCO location and its limit text (S9 AC-4), `Alterar vínculo` (edit form anchor).
4. **Automação** and **Replicação** cards (§5.5).
5. **Posição aberta:** read-only indicator "Posição aberta: sim/não" from the gateway (needed for lifecycle and delete messages).

**States:** loading skeleton of the five cards; error/not found per §2; unbound account shows card 3 as `Sem vínculo` with `Vincular conta`.

---

## 9. S5-08 Preset-dependent fields in the form

The preset picker, versioning and `FirmRuleQuote` data are defined in the S16 spec. This section fixes what the S5 form does with them.

### 9.1 Firm platform (O9, S5 AC-1)
- When the selected preset has rules that differ by platform, the form shows a required `Select` "Plataforma da conta na firma" / "Firm account platform" (options from the preset, e.g. `Rithmic`, `Tradovate`), right after plan/size. Help: "A firma tem regras diferentes por plataforma. Escolha a plataforma em que esta conta foi aberta." / "The firm has different rules per platform. Choose the platform this account was opened on."
- Until chosen, platform-dependent fields show "Escolha a plataforma" and are not editable.
- Changing the platform re-fills platform-dependent fields. Fields the user edited ask first: "Trocar a plataforma substitui {n} valores que você editou. Continuar?" (`AlertDialog`, `Cancelar` focused). Then §5.2 may clear the choices.
- Stored on the account and shown on the detail ("Plataforma na firma: Rithmic"). Not the **execution platform** (NinjaTrader, F§5).

### 9.2 "Not confirmed" values (O7, O8)
- Preset options with not-confirmed items stay selectable; each shows "{n} itens não confirmados" in warning text. A firm with no verified data is not listed.
- Each not-confirmed field uses F§7.4: `Não confirmado` badge, warning border, **no prefilled value**, `ConflictPicker` with each published value (source, date) plus "Outro valor" (input with the field's normal validation). Nothing chosen for the user.
- Required fields (max position, drawdown, daily loss limit, reset time) block save; the error summary lists them: "Resolva os valores não confirmados: Drawdown, Regra de consistência."
- Optional or informational fields (e.g. consistency rule) can be saved unresolved; the detail shows `Não confirmado`, and they are not used for warnings until resolved **[confirmed by PO, Q-S5-5]**; the detail shows "Regra de consistência não confirmada — não acompanhada" / "Consistency rule not confirmed — not tracked".
- A resolved value is stored as the user's own value with provenance "chosen among conflicting values" and audited like any account edit.
- Summary `Alert` (info) below the form when the preset has any: "{n} regras desta firma não estão confirmadas porque as páginas oficiais divergem. Os valores marcados como “Não confirmado” precisam da sua escolha."
- Note: the max position is **required** in S5 AC-3, so a `not_confirmed` max position must be resolved before saving.

---

## 10. S5-06 Lifecycle: pause, resume, disable, enable (AC-4)

**States** (F§7.2): `Ativa` (no badge), `Pausada` (`Pause`, neutral), `Desativada` (`CircleOff`, neutral). Definitions shown in help text, never invented: **Pausar** = parada temporária que mantém toda a configuração; **Desativar** = conta desligada até ser reativada.

| Action | From | Confirmation | Result toast |
|---|---|---|---|
| `Pausar` / Pause | Ativa | `AlertDialog` | "Conta pausada." |
| `Retomar` / Resume | Pausada | none (one click) | "Conta retomada." |
| `Desativar` / Disable | Ativa or Pausada | `AlertDialog` | "Conta desativada." |
| `Ativar` / Enable | Desativada | none | "Conta ativada." |

**Pause / disable dialog** (destructive-weight, confirm button repeats the verb: `Pausar conta` / `Desativar conta`; cancel focused):
- Body: "Enquanto estiver {pausada | desativada}, esta conta não recebe nenhuma instrução nova, **nem de saída (fechar)**. Posições e ordens abertas não são alteradas." / "While {paused | disabled}, this account receives no instruction, **including exits (close)**. Open positions and orders are not changed." (S7 AC-8, ADR-0003 §4.1: paused/disabled block every action.)
- If the account has an open position: extra warning `Alert`: "Esta conta tem posição aberta. Ela continuará aberta e sem o All Win para fechá-la." / "This account has an open position. It stays open and All Win won't close it."
- Each skip while paused/disabled is audited with reason (X-5; `routing.account_inactive`).

**States:** submitting (button `Salvando…`, dialog stays open); failure (destructive `Alert` in the dialog, state unchanged); concurrent change (`common.version_conflict`) → toast "A conta mudou em outro lugar." + refetch.

---

## 11. S5-07 Delete account (AC-6)

- Entry: kebab `Excluir` in list/detail.
- **With an open position:** the action opens an info `AlertDialog` instead of the confirmation, **no confirm button**: "Não é possível excluir {account}: há posição aberta. Feche a posição no NinjaTrader e tente de novo." / "Can't delete {account}: it has an open position. Close it in NinjaTrader and try again." Button `Entendi`. The server also refuses with a stable code (`accounts.delete_open_position`, proposed); if it arrives anyway, the same text is shown.
- **Without open position:** `AlertDialog` (cancel focused): title "Excluir conta {name}?"; body: "A conta deixa de aparecer e não recebe mais instruções. O registro de auditoria e o histórico permanecem. Esta ação não pode ser desfeita." / confirm `Excluir conta` (destructive). The body adds, when they exist, a list: "Serão removidos: {n} rotas ({source → …}) e {m} relações de replicação ({master → …})." / "Will be removed: …"; if none, the line is omitted. **[Provisional, Q39 rec. (a): archive; routes and master/follower relations are removed after a confirmation that lists them; history kept (S10 AC-3). The dialog lists them, see below.]**
- After success: toast "Conta excluída." and return to the list. Deleting "archives" the account (`archived_at`); history keeps the id and name snapshot.
- A deleted account's name becomes available again (unique index is on live accounts).

---

## 12. Step-up (D9) — what the UI does

- Operations that may need it: create/update with an `accepted` choice that changes the stored state; `setAccountOptIn` with `accepted`. `declined` never does.
- The Core decides which operations need it. The web asks the Core for a challenge, opens the `StepUpDialog` (F§8.2: channel, action, target), keeps the form values in memory, and sends the **original request once** with `challenge_id` + `code`, verified in the same transaction as the save. There is no "open dialog, then retry" loop.
- The dialog is F§8.2 (new code to the verified channel, `CodeChallenge`). Single error `auth.code_invalid`: "Código inválido ou expirado. Confira ou peça um novo." / "Invalid or expired code. Check it or request a new one." (form values and choices are kept). Cancel runs nothing. The window length (Q30) does not change the UI.
- Cancel = nothing saved, form keeps all values, choices and the acknowledgement.

---

## 13. Audit events emitted by S5 (render in S10)

Codes are **proposals** for the Architect; clients translate them (ADR-0001 §9). Fields shown in S10 are in the "Details" column. Category is `config` unless noted.

| Event (code, proposed) | Entry text (pt-BR / en) | Details |
|---|---|---|
| `accounts.created` / `.updated` / `.archived` | "Conta {account} criada" / "…editada" / "…excluída" | changed fields (names and new values; money as strings), user |
| `accounts.lifecycle_changed` | "Conta {account} pausada" · "desativada" · "retomada" · "ativada" | previous and new state, had open position (yes/no) |
| `accounts.opt_in_chosen` | "Automação aceita na conta {account}" / "Automation accepted on account {account}" · "Automação recusada…" · copy: "Replicação aceita/recusada…" | date-time with zone · user (e-mail) · kind · choice · previous choice · firm · rule version · source URL(s) and access date(s) of the displayed text · `firm_stance` (five values) |
| `accounts.opt_in_suspended` | "Ordens automatizadas suspensas na conta {account}: a regra da {firm} mudou (versão {n})" | cause (`stance_changed` / `text_changed`), old and new rule version, sources |
| `routing.automation_not_accepted` (category `routing`) | "Instrução ignorada para {account}: a conta não aceita fontes automatizadas" or "…: escolha de automação pendente" | instruction id, source name (never its URL), time |
| `routing.copy_not_accepted` (category `routing`) | "Replicação ignorada para {account}: a conta não aceita replicação" or "…: escolha de replicação pendente" | same |

D7: the two routing codes replace the former text-only skip reason. Entries are read-only (S10 AC-3). Source links open the official page in a new tab.

---

## 14. Data and actions this slice needs (for the Architect to close the contract)

**Reads**

| Need | Used by | Notes |
|---|---|---|
| Account list: id, name, kind, firm/plan/phase names, size (string), firm platform, multiplier, max position, `lifecycle_state`, bound platform account (name, connection id/name, kind, `oco_location`), `automation` and `copy` `OptInState`, `row_version`, `has_open_position`, state flags from gateway (out-of-sync, connection status) | S5-01 | One call; no N+1 |
| Account detail: all of the above + `created_at`, `copied_from_size` (profile provenance), preset version id/number | S5-04 | |
| Catalog for the form: firms, plans, phases, sizes per plan, platforms per plan, preset max position per size, per-field provenance (verified / not confirmed with published values, sources, dates) | S5-02, §9 | From S16 |
| `OptInState` + rule content for a selection (firm, plan, phase, size, platform) **before** the account exists: `firm_stance`, `current_rule_version_id`, quotes (text, language, page title, URL, access date), research sources for `unknown` | §5 | The form needs a "preview" query because there is no account id yet |
| Previous rule version quote for comparison | §6 | |
| Platform accounts of paired connections, with bound/unbound flag and the name of the account they are bound to (own user only) | §4.3 | `VisibleAccount` today has `account_id` only |

**Writes**

| Action | Notes |
|---|---|
| `createAccount` (with `opt_ins`, idempotency key) | One save; `x-step-up: conditional` (D9). 422 pointers listed in §4.5 |
| `updateAccount` (with `row_version`, `opt_ins`) | 409 `common.version_conflict`, 409 `accounts.rule_version_changed`, 422 `accounts.opt_in_required` |
| `setAccountOptIn` | In the contract (1.1.0) |
| `setAccountLifecycle` (`pause` / `resume` / `disable` / `enable`) | audited with previous state |
| `archiveAccount` | refuses with `accounts.delete_open_position` |
| `bindPlatformAccount` / unbind | may be part of update; clash → `accounts.platform_account_taken` (without revealing the other user's data) |

**Events (SSE):** account `lifecycle_state`, opt-in `state`, connection status and out-of-sync flag changes, so list and detail update without reload.

**Error codes the UI maps** (proposed names where new): `accounts.name_taken`, `accounts.max_position_exceeds_preset`, `accounts.multiplier_invalid`, `accounts.platform_account_taken`, `accounts.delete_open_position`, `accounts.opt_in_required` (pointer), `accounts.opt_in_not_applicable`, `accounts.rule_version_changed` (+ `current_rule_version_id`), `common.version_conflict`, `common.not_found`, `auth.code_invalid`.

---

## 15. Accessibility

- Opt-in section: `fieldset`/`legend`; summary, quote and warning precede the radio group in the DOM; the group has `aria-describedby` → warning. `FirmRuleQuote` uses `lang` of the source text. Links say where they go ("Ver página oficial da Apex Trader Funding (abre em nova aba)").
- Form: visible labels, errors linked with `aria-describedby`/`aria-invalid`, focus to first invalid field (F§11). The sticky footer must not hide the focused field (2.4.11).
- Selects with search use the `Combobox` pattern (shadcn `Command` in `Popover`): type-ahead, `aria-expanded`, Escape closes and returns focus.
- Lifecycle and delete dialogs: `AlertDialog`, cancel focused, focus returns to the kebab/trigger on close. Result toasts are polite.
- Badges carry text and icon; nothing depends on color. Account list badge `+N` popover is keyboard-reachable.
- Tabs follow the WAI-ARIA tabs pattern (arrow keys).
- Disabled platform-account options stay focusable with `aria-disabled` and the reason in `aria-describedby`.

## 16. AC / decision traceability

| Requirement | Where |
|---|---|
| S5 AC-1 prop/real account, firm platform, link to platform account | §4.2, §4.3, §9.1 |
| AC-2 sizes; real any size | §4.2 |
| AC-3 max position mandatory, preset cap | §4.2, §9.2 |
| AC-4 enable/disable/pause, skip audited, positions untouched | §10, §13 |
| AC-5 multiplier whole ≥ 1 | §4.2 |
| AC-6 delete refused with open position; isolation | §11, §2 |
| AC-7/AC-8 option A, no default, audited, step-up to accept, change flow | §5, §5.5, §12 |
| AC-9 automated sources vs copy | §5.3 help text, D13 |
| AC-9b copy opt-in at follower add | §5.6 |
| AC-10 choice for any stance ≠ allowed | §5.1 |
| AC-11 suspension on stance (and provisionally text) change | §6 |
| D1–D16 | §5 (D1–D6, D9–D13), §6 (D14), §7 (D15), §5.2 (D16), §13 (D7), §5.4 (D8: atomic) |
| X-2, X-3, X-5, X-6 | §2, §13 |

## 17. Open questions

**Closed by the PO (Rumo, 2026-10-23):** Q-S5-3 yes (suspend even before applying the version); Q-S5-5 unresolved consistency rule gives no warning, detail says "não acompanhada"; Q-S5-8 copy choice exists only for a follower, a non-follower has no copy card (state name: Architect); Q-S5-9 no business rule (design/Architect); Q-S5-11 terms confirmed by Aquarela and PO (must not sound like legal advice).

**Now user questions (slice-plan §7), drawn with the recommendation and labeled provisional:**

| ID | Question | Spec default (provisional) |
|---|---|---|
| Q37 (was Q-S5-1) | Platform-account binding optional | Optional; an unbound account never routes |
| Q38 (was Q-S5-2) | Change binding with an open position | Blocked |
| Q39 (was Q-S5-6) | Delete account | Archive; routes and master/follower relations removed after a confirmation that lists them; history kept |
| Q40 (was Q-S5-7) | Before S16 | Free-text firm/plan/phase, typed max, stance `unknown` ⇒ choice required with generic warning |
| Q41 (was Q-S5-4) | `pending_review` when a preset is attached | Yes |
| Q42 (was Q-S5-10) | "Material" text change (D14) | Flagged by the publisher of the version, with a reason shown to the user |

**Still open (Architect):** Q-S5-12 the state name for a copy opt-in of a non-follower (no card in the UI either way); Q-S5-13 `accounts.platform_unbound` and the other proposed error codes (§14).
