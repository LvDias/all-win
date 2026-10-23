# V1-S5 — Per-account automation opt-in and preset provenance — Screen spec

> **Owner:** Product Designer (Aquarela @ Dev3) · **Status:** proposed, pending PO review · **Revision:** 1 · **Date:** 2026-10-23
> **Slices:** [V1-S5](../../product/release-1/slice-plan.md) (account registration and lifecycle), with touch points in S7 (routing), S10 (audit) and S16 (presets). The PO is adding the opt-in rules to S5/S7/S16 in the slice plan.
> **Foundations:** [foundations.md](../foundations.md): F§7.2 states, F§7.4 provenance and `FirmRuleQuote`, F§8.2 step-up.
> **Basis:**
> - User decision of 2026-10-23, option A ([decisions.md](../../delivery/decisions.md)): for a firm that forbids automation, the user must explicitly choose, per account, whether the account accepts automated sources. There is **no default**. The choice is made after a warning that shows the firm's rule text and the risk of losing the account, and it is audited with date and user. Items that conflict across official pages ship as "not confirmed" without blocking the rest. Presets store the platform.
> - PO answers of 2026-10-23 (Bússola): O2, O4, O6, O7, O8 and O9 confirmed. O1 is the PO's interpretation, to be confirmed by the user.
>
> **Open, user decisions (PO list):**
> - **O1 [Assumption]:** "automated source" = TradingView webhook + the future signal engine. Master replication follows the copy stance instead.
> - **Q24:** the same opt-in for firms that forbid or condition **copying**.
> - **Q25:** which stance values trigger the choice.
> - **Q26:** what happens to the choice when a new preset version changes the rule.
>
> The spec draws the PO recommendation for each and marks where an answer changes it.
> **Not legal advice.** All Win shows the firm's own text. Summaries are labeled as All Win's reading (vision §12).
> **Surfaces:** Web (Frontend). Core rules (routing skip, audit) are Backend's. QA owns the checks.

---

## 1. What this spec covers

| # | Element | Where |
|---|---|---|
| 1 | Automation section inside the account form (create and edit) | `/[locale]/accounts/new`, `/[locale]/accounts/[id]/edit` (full S5 form spec follows separately) |
| 2 | Automation card on the account detail page, including "change choice" | `/[locale]/accounts/[id]` |
| 3 | Suspended / pending state after a rule change (Q26) | account list, account detail, notification |
| 4 | Routing: automated source × account not accepting automation | S7 routing screen |
| 5 | Audit entries for the choice and for skipped instructions | S10 audit view |
| 6 | Firm platform selection (O9) and "not confirmed" preset values (O7, O8) in the account form | account form |

## 2. When the automation choice is required

Inputs come from the preset version selected for the account (S16): `automated_trading_allowed` ∈ `yes | no | conditional | unknown`, its provenance (verified / not confirmed), the official quote(s), and the preset version.

| Account | Firm stance on automation | UI in the account form |
|---|---|---|
| Real brokerage account | — | No section. No question |
| Prop | `yes` (verified) | Compact info line: "A {firm} permite negociação automatizada." + `FirmRuleQuote` collapsed behind `Ver regra oficial`. No question |
| Prop | `no` | **Required choice** (§3), decided |
| Prop | `conditional`, `unknown`, or the stance is **not confirmed** | **Required choice** (§3), per **Q25 recommendation**. If the user decides otherwise, these show the info line instead, with the stance badge (`Condicional` / `Desconhecida` / `Não confirmado`) |
| Prop without a preset (accounts created before S16) | treated as `unknown` | Choice required when S16 lands: the account shows `Escolha pendente` (§5). Sim-only today, so no real account is affected **[PO to confirm the migration behavior]** |

The section appears only after firm, plan, phase, size and (when relevant) firm platform are selected, because the preset version (and so the rule text) depends on them. If any of these change after the user chose, **the choice is cleared** and asked again when the rule text or stance differs. A message explains it: "A regra mudou com a sua seleção. Escolha de novo."

## 3. Account form: "Ordens automatizadas" section (create and edit)

**Structure** (DOM order = reading order; the warning comes **before** the choice):

```
┌ fieldset ──────────────────────────────────────────────────────────────┐
│ legend: Ordens automatizadas nesta conta                                │
│                                                                         │
│ Resumo do All Win (não é aconselhamento jurídico):                      │
│ A Apex Trader Funding proíbe negociação automatizada nas regras         │
│ oficiais. Alertas do TradingView executados automaticamente podem ser   │
│ considerados automação.                                                 │
│                                                                         │
│ ┃ "No Automation or Algorithm Usage allowed: Rewards are intended to … │  ← FirmRuleQuote (lang=en)
│ ┃  Apex Trader Funding · Prohibited Activities · acessado em 23/10/2026│
│ ┃  [Ver página oficial ↗]   [Mostrar texto completo]                   │
│                                                                         │
│ ⚠ Risco: se a firma considerar isso uma violação, você pode perder     │  ← Alert (danger)
│   esta conta. A decisão é sua; o All Win não decide isso por você.     │
│                                                                         │
│ ( ) Não aceitar ordens automatizadas                                    │  ← RadioGroup, required,
│     Alertas do TradingView nunca serão enviados para esta conta.        │     nothing preselected
│ ( ) Aceitar ordens automatizadas, ciente do risco                       │
│     Alertas do TradingView poderão ser executados conforme suas rotas.  │
│                                                                         │
│   [ ] Li a regra oficial acima e entendo que posso perder esta conta.  │  ← only when "Aceitar"
└─────────────────────────────────────────────────────────────────────────┘
```

**Content rules**

- **Summary line:** generated from the stance value with fixed, reviewed copy per value. It is never free text.
  - `no`: "A {firm} proíbe negociação automatizada nas regras oficiais."
  - `conditional`: "A {firm} só permite negociação automatizada sob condições. Leia a regra."
  - `unknown`: "Não encontramos uma regra oficial da {firm} sobre automação."
  - not confirmed: "As páginas oficiais da {firm} dizem coisas diferentes sobre automação."
  - The second sentence is always "Alertas do TradingView executados automaticamente podem ser considerados automação."
- **What counts as automated (O1):** a one-line help under the legend: "Fontes automatizadas: alertas do TradingView (webhook) e, no futuro, o motor de sinais do All Win. Esta escolha não afeta a replicação da conta master." **[O1 assumption pending user confirmation; if Q24 adds a copy opt-in, the last sentence becomes "A replicação da conta master tem uma escolha própria, abaixo."]**
- **Quotes:** one `FirmRuleQuote` (F§7.4) per official quote the preset version holds, in the firm's language, never translated. When the stance is **not confirmed**, all conflicting quotes are shown, each with its own source and date, under the heading "As páginas oficiais divergem:".
  - `unknown`: no quote. Text instead: "Pesquisamos as páginas oficiais em {date} e não encontramos uma regra sobre automação." with the research sources listed.
- **Risk warning:** `Alert` variant destructive, `TriangleAlert` icon. Copy is fixed: "Risco: se a firma considerar isso uma violação, você pode perder esta conta. A decisão é sua; o All Win não decide isso por você." / "Risk: if the firm considers this a violation, you may lose this account. The decision is yours; All Win doesn't make it for you."
- **Choice:** shadcn `RadioGroup`, required, **no option preselected** (decision: no default), including on edit after a reset (§5). Options are full-width cards (`label` wrapping the radio), ≥ 44 px tall.
  - `declined`: "Não aceitar ordens automatizadas" / "Don't accept automated orders". Description: "Alertas do TradingView nunca serão enviados para esta conta. Se algum chegar, ele é ignorado e registrado na auditoria." (O2)
  - `accepted`: "Aceitar ordens automatizadas, ciente do risco" / "Accept automated orders, aware of the risk". Description: "Alertas do TradingView poderão ser executados nesta conta conforme suas rotas."
  - Acknowledgement `Checkbox` (only after `accepted` is selected, required): "Li a regra oficial acima e entendo que posso perder esta conta." / "I read the official rule above and understand I may lose this account."

**Validation and save**

| Situation | UI |
|---|---|
| Save with no choice | Inline error under the legend: "Escolha se esta conta aceita ordens automatizadas." Focus moves to the radio group. Included in the form's error summary |
| `accepted` without the checkbox | Inline error on the checkbox: "Confirme que leu a regra para aceitar." |
| Save with `accepted` | Step-up (F§8.2) if the last authentication is older than 15 min (O4; **Architect to add this action to the policy list**). Then save |
| Save with `declined` | No step-up, no extra dialog |
| The preset version changed between loading and saving (Core returns `accounts.rule_version_changed`) | The section reloads with the new quote, the choice is cleared, and an `Alert` explains it: "A regra da firma foi atualizada enquanto você preenchia. Leia de novo e escolha." Other form fields keep their values |
| Success | Toast "Conta salva." The audit entry is written by the Core (§7) |

**Contract (proposal for the Architect):** the save sends `automation_choice: "accepted" | "declined"` **and** `automation_rule_version_id`, the preset version whose text the user saw. The Core rejects a stale version, so the audit always records exactly what was displayed (O6). The account returns `automation_choice`, `automation_state: "chosen" | "pending_review" | "not_required"`, `automation_chosen_at`, `automation_rule_version_id`.

## 4. Account detail: "Automação" card and changing the choice (O4)

**Card** (only for accounts where §2 requires a choice):
- Badge per F§7.2: `Aceita automação` (info, `Bot`) / `Sem automação` (neutral, `BotOff`) / `Escolha pendente` (warning, `CircleHelp`).
- "Escolhido por você em {date time zone} · regra da {firm} versão {n} (acessada em {date})" + link `Ver no registro de auditoria` (S10 filtered to this account and event type).
- `Ver regra oficial` (collapsed `FirmRuleQuote` of the version the user accepted/declined).
- Action `Alterar escolha` / `Change choice`.

**Change to accepted** (from declined or pending): `ResponsiveDialog` with the full §3 section (current preset version, quote, warning, radio with nothing preselected, checkbox) → step-up if needed → save → toast "Esta conta agora aceita ordens automatizadas." Audited.
**Change to declined:** the same dialog. Selecting `declined` shows no warning or checkbox (O4: no warning), only the note "Posições e ordens abertas não são alteradas." / "Open positions and orders are not changed." → save → toast "Esta conta não aceita mais ordens automatizadas." Audited.
- Open positions and working orders are never touched by a change (O4); the note says so in both directions.
- Cancel / close = nothing changes.

## 5. Rule changed: suspended until reviewed (Q26)

Drawn for **Q26 recommendation (b)**: when a new preset version changes the automation stance **or** its official text, automated sources are **suspended** for every account that uses that firm and needs a choice, until the user chooses again; the user is notified.

- **Account list:** badge `Escolha pendente` (warning) in the account's state cell (F§7.2 priority: after risk warning, before paused/disabled).
- **Account detail:** warning `Alert` at the top: "A regra da {firm} sobre automação mudou em {date} (versão {n}). Ordens automatizadas estão suspensas nesta conta até você revisar." + `Revisar agora`.
- **Review dialog:** same as §4 "change", plus a `Collapsible` "Texto anterior (versão {m})" with the old quote, so the user can compare. **Nothing preselected**, including the previous choice (decision: no default).
- **Notification (S18):** "Regra de automação da {firm} mudou: {k} contas com ordens automatizadas suspensas." → link to the account list filtered by `Escolha pendente`.
- **Routing (S7):** cells for these accounts show the pending reason (§6).
- *If Q26 = (a), keep the choice:* no suspension. The account shows an info `Alert` "A regra da {firm} mudou. Revise sua escolha." with `Revisar`, and the badge keeps the current choice plus a dot indicator.
- **Interaction with S16 AC-3** (a new preset version is applied only if the user chooses): **[PO to confirm]** whether a stance/text change suspends automation even before the user applies the new version to the account. The design assumes **yes**, since the rule changed at the firm regardless of what the account applies.

## 6. Routing (S7): unavailable combinations (O2)

In the routing configuration (source × destination account), when the source is automated (S3-09 badge `Automatizada`) and the account is `declined` or `pending_review`:
- The control (checkbox/switch in the routing matrix, or the account option in a source's destination list) is **disabled** (`aria-disabled`, still focusable so the reason can be read) with a `Lock` icon.
- The reason is shown inline on touch layouts and in a tooltip on pointer layouts; it is also the control's `aria-describedby`:
  - declined: "Esta conta não aceita ordens automatizadas." + link `Alterar na conta`.
  - pending: "Escolha de automação pendente nesta conta." + link `Revisar`.
- If a route existed before the account changed to declined or pending, the route stays visible but is shown inactive with the same reason. It is not silently deleted. **[PO/S7 to confirm: keep the route and skip at runtime, vs. remove it]**
- S16 AC-4 also requires the firm's stance to be shown when the account is added to routing: a compact stance line with `Ver regra oficial` next to the account name in the destination picker.

## 7. Audit entries (O6, X-5, S10)

| Event | Entry (pt-BR / en) | Fields shown |
|---|---|---|
| Choice made or changed | "Automação aceita na conta {account}" / "Automation accepted on account {account}" · "Automação recusada na conta {account}" | Date-time with zone · user (e-mail) · choice · previous choice · firm · preset version · source URL(s) and access date(s) of the displayed text · stance value and provenance (verified / not confirmed) |
| Suspended after a rule change (Q26 b) | "Ordens automatizadas suspensas na conta {account}: a regra da {firm} mudou (versão {n})" | Old and new preset version, sources |
| Instruction skipped (O2) | "Instrução ignorada para {account}: a conta não aceita fontes automatizadas" or "…: escolha de automação pendente" | Instruction id, source name (never its URL), time |

Audit entries are read-only (S10 AC-3). Source links in entries open the official page in a new tab.

## 8. Preset values in the account form: platform and "not confirmed"

### 8.1 Firm platform (O9)
- When the selected preset has rules that differ by platform, the form shows a required `Select` "Plataforma da conta na firma" / "Firm account platform" (options from the preset, e.g. `Rithmic`, `Tradovate`), right after the plan/size.
  - Help text: "A firma tem regras diferentes por plataforma. Escolha a plataforma em que esta conta foi aberta." / "The firm has different rules per platform. Choose the platform this account was opened on."
- Until chosen, the platform-dependent fields show a placeholder "Escolha a plataforma" and are not editable.
- Changing the platform re-fills platform-dependent fields from the preset. Fields the user had edited ask first: "Trocar a plataforma substitui {n} valores que você editou. Continuar?". Then the automation choice (§2) may be cleared.
- The platform is stored on the account and shown on the detail page ("Plataforma na firma: Rithmic").
- Not to be confused with the **execution platform** (NinjaTrader, chosen in S5 AC-1). Glossary entry added in F§5 terms: "Plataforma da conta na firma" / "Firm account platform".

### 8.2 "Not confirmed" values (O7, O8)
- Preset picker: presets with not-confirmed items stay selectable (O8). Each option shows "{n} itens não confirmados" in `--status-warning` text, and a firm with no verified data is not listed (S16 AC-5 as amended by the PO).
- Each not-confirmed field uses the F§7.4 treatment: `Não confirmado` badge, warning border, **no prefilled value**, and the `ConflictPicker` listing each published value with its source and date, plus "Outro valor" (an input with the field's normal validation, F§6.3). Nothing is chosen for the user (no "most conservative" default).
- Required fields (e.g. max position, drawdown, daily loss limit) block save until resolved; the form error summary lists them: "Resolva os valores não confirmados: Drawdown, Regra de consistência."
- Optional or informational fields (e.g. consistency rule text, which only tracks and warns per Q4) can be saved unresolved; they show `Não confirmado` on the detail page and are not used for warnings until resolved **[PO to confirm: an unresolved consistency rule produces no warnings]**.
- A resolved value is stored as the user's own value with provenance "chosen among conflicting values" (F§7.4) and audited like any account edit.
- Below the form, a summary `Alert` (info) when the preset has any not-confirmed items: "{n} regras desta firma não estão confirmadas porque as páginas oficiais divergem. Os valores abaixo marcados como “Não confirmado” precisam da sua escolha."

## 9. States summary

| Situation | UI |
|---|---|
| Preset data loading | Skeleton for the rules and automation sections |
| Preset service error | `ErrorState` inside the rules area, with retry. The form cannot be saved for a prop account until it loads (the rules and the choice depend on it). Real accounts are unaffected |
| Firm stance not in the preset at all (data gap) | Treated as `unknown` (§2) |
| Quote URL unreachable | The link still opens; the text shown is the stored quote, which is the evidence |
| Step-up cancelled | Nothing saved; the form keeps all values and the choice |
| Save failed | Destructive `Alert` with the code; values kept |

## 10. Accessibility

- The section is a `fieldset` with `legend`. The summary, quote and warning come before the radio group in DOM order. The radio group has `aria-describedby` pointing to the warning, so screen-reader users hear the risk before choosing.
- `FirmRuleQuote` uses `lang` of the source text. Links say where they go ("Ver página oficial da Apex Trader Funding (abre em nova aba)").
- Disabled routing controls stay focusable with `aria-disabled="true"`, and the reason is linked with `aria-describedby`.
- Error messages follow F§10/F§11. Nothing relies on color: badges carry text, and the warning has an icon and the word "Risco".

## 11. AC / decision traceability

| Requirement | Where |
|---|---|
| Decision 2026-10-23: explicit per-account choice, no default, after warning with rule text + account-loss risk | §2, §3 |
| Choice audited with date and user (O6: plus preset version, source URL, access date) | §3 contract, §7 |
| O2 declined = never receives automated instructions; S7 combination unavailable with reason; skipped + audited | §6, §7 |
| O4 change anytime; accept = warning + step-up + audit; decline = no warning + audit; positions untouched | §4 |
| O7 not-confirmed values not prefilled, user picks or enters, required ones block save | §8.2, F§7.4 |
| O8 presets with not-confirmed items selectable | §8.2 |
| O9 firm platform chosen at registration, defines rules, stored on the account | §8.1 |
| S16 AC-4 stance shown at registration and when adding to routing | §2, §3, §6 |
| X-5 transparency, X-6 bilingual (quotes stay in the source language by design) | §7, §3 |

## 12. Open questions (owner)

| ID | Question | Owner | Spec default |
|---|---|---|---|
| O1 | "Automated source" = TradingView webhook + future signal engine; master replication follows the copy stance | User (via PO) | As stated |
| Q24 | Same explicit opt-in for firms that forbid/condition copying (master replication) | User | Not drawn; the component is reusable (`kind: automation | copy`) |
| Q25 | Choice for `conditional` / `unknown` / not confirmed stances too | User | Yes |
| Q26 | Rule change: suspend until reviewed (b) vs keep the choice and notify (a) | User | (b) |
| — | Does a stance change suspend before the account applies the new preset version (S16 AC-3)? | PO | Yes |
| — | Prop accounts created before S16 become `Escolha pendente` | PO | Yes |
| — | An unresolved not-confirmed consistency rule gives no warnings | PO | Yes |
| — | Existing route to an account that becomes declined: kept inactive vs removed | PO (S7) | Kept inactive |
| — | Add "accept automation" to the step-up list; `automation_rule_version_id` in the account contract; `accounts.rule_version_changed` error | Architect | Proposed |
