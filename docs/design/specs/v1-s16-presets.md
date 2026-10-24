# V1-S16 — Prop firm presets: model and UI — Screen spec

> **Owner:** Product Designer (Traço @ Dev3) · **Status:** proposed, pending PO review · **Revision:** 1 · **Date:** 2026-10-23
> **Slice:** [V1-S16](../../product/release-1/slice-plan.md) AC-1 … AC-12 (slice-plan rev. 9); touch points: S5 (account form, opt-ins, Q40–Q42), S7 (stance in routing, manual-only), S13 (rules editing), S18 (notification).
> **Foundations:** [foundations.md](../foundations.md) (F§7.4 provenance, `FirmRuleQuote`, `ConflictPicker`; F§5 terms) · **Related:** [S5 spec](v1-s5-automation-opt-in.md) §5, §6, §9; [S7 spec](v1-s7-routing.md) §6, §7.
> **Contract:** [HTTP contract 3.4.1](../../../contracts/http/openapi.yaml) (commit 721e0bc): `contracts/http/modules/presets.yaml` and the notes [s16-presets.md](../../architecture/contracts/s16-presets.md): `listFirms`, `getFirm`, `listFirmReleases`, `getPresetVersion`, `getOptInRules`, `getRuleVersion`, `getAccountPreset`, `getAccountPresetDiff`, `applyPresetVersion`, `restorePresetValue`. **Where this spec and the contract differ, the contract wins.**
> **Data source:** [preset-research-format.md](../../product/presets/preset-research-format.md) (fields and YAML), research files in `docs/product/presets/research/`, conflicts in [r1](../../product/presets/r1-conflicts-for-user.md) / [r2](../../product/presets/r2-conflicts-for-user.md). 
> **Surfaces:** Web (Frontend). Preset data, versioning and diffs are the Core's (Backend). QA owns the checks.
> **Not legal advice.** All Win shows the firm's own text with source and date; summaries are labeled as All Win's reading (vision §12). Presets are never invented (vision §9).

---

## 1. What this spec covers

| # | Screen / element | Route |
|---|---|---|
| S16-01 | Preset catalog (browse firms) | `/[locale]/presets` (sidebar `Presets`) |
| S16-02 | Firm detail: stances, plans, versions | `/[locale]/presets/[firm]` |
| S16-03 | Preset picker and rules preview inside the account form | S5-02 form |
| S16-04 | Edited values: `Editado` mark, restore (shared pattern; used on the S13 screen) | account risk tab |
| S16-05 | New preset version: notice, comparison, apply or keep (AC-3) | account detail, S18 link |
| S16-06 | Manual-only presets (AC-7) | catalog, picker, account detail |
| S16-07 | Stance block (automation, copy, VPS, personal device) | reused by S16-02, S16-03, S5, S7 |

**Who edits a preset:** nobody in the app. Presets come from the research seed and are versioned by the team (Core). "Editable by the user" (vision §9, AC-2) means the user edits **the copy on their account**; the preset never changes (S16-04). There is no preset editor in V1.

---

## 2. Concepts shown to the user (no new rules)

- A **preset** = firm + plan + phase + size (+ firm platform when rules differ by platform) with a **version**: all rules from S13, maximum position, trading-day reset time, the firm's stance on copying and automated trading (classification + verbatim text), the firm's VPS/server rule and whether a personal device is required (verbatim text), source URL(s), verification/access date (AC-1, AC-8).
- Each value is **verified**, **not confirmed** (the firm's pages disagree) or later **edited by the user** (F§7.4). Not-confirmed values are never prefilled (AC-6).
- A preset is selectable when its fields are verified **or** marked not confirmed; a firm with no verified data is not offered (AC-5).
- Plans excluded from V1 (Apex Legacy, Phidias LIVE, Topstep Labs Challenges and Pro Account, Take Profit Trader PRO+ and PRO+ Development) are **not offered**; the user registers them manually (AC-9).
- A new preset version never changes existing accounts silently; the user is told and chooses whether to apply (AC-3).

---

## 3. S16-01 Preset catalog — `/[locale]/presets`

**Purpose:** see which firms are available and each firm's stance before registering an account (AC-4).
**Permissions:** any signed-in user. Presets are shared reference data, not user data.

**Layout:** `PageHeader` "Presets de firmas" · intro "Regras e posições oficiais de cada firma, com fonte e data. O All Win mostra o texto da firma; não é aconselhamento jurídico." · search `Input` · list (table ≥ `md`, cards below).

| Column | Content |
|---|---|
| Firma / Firm | name (link to S16-02) · `Só acompanhamento manual` badge for manual-only firms (neutral, `Eye`) |
| Planos | count + names (first two, `+N`) |
| Automação | `StanceBadge` (§7) |
| Replicação / Copy | `StanceBadge` |
| Não confirmados | "{n} itens não confirmados" in warning text, or `—` |
| Verificado em | date (F§6.4) of the preset version's verification |

Sort: name A→Z. Search filters by firm and plan name. No pagination (18 firms).

**States:** loading skeleton (6 rows) · error `ErrorState` + retry + code · no results "Nenhuma firma encontrada." + `Limpar busca` · empty catalog (should not happen) "Os presets ainda não estão disponíveis." + retry.

---

## 4. S16-02 Firm detail — `/[locale]/presets/[firm]`

**Layout:** `PageHeader` (firm name, verification date, version number) · `Alert` for manual-only (S16-06) · tabs: `Posições da firma` · `Planos e regras` · `Versões`.

### 4.1 Tab "Posições da firma" (S16-07 stance blocks)

Four `StanceBlock`s (§7): **Automação**, **Replicação**, **Servidor/VPS**, **Dispositivo pessoal**. Each shows the `StanceBadge`, All Win's one-line summary (fixed copy per value, S5 §5.3 table), the `FirmRuleQuote`(s) (verbatim, firm's language, source and access date), and `Ver página oficial`. When the stance is `not_confirmed`, all conflicting quotes appear under "As páginas oficiais divergem:". When `unknown`: "Pesquisamos as páginas oficiais em {date} e não encontramos uma regra." plus the sources searched.

### 4.2 Tab "Planos e regras"

- Filters: `Plano`, `Fase`, `Plataforma` (when the plan has platform-specific rules) as `Select`s; then a table of sizes:

| Column | Content |
|---|---|
| Tamanho | `US$ 50.000,00` |
| Drawdown | amount + type (`Trailing intradiário` / `Trailing fim do dia` / `Estático`) |
| Perda diária | amount + `limite rígido` / `pausa do dia` / `sem limite` |
| Meta diária de lucro | amount or `—` (S13 AC-5; the user's choice notify/flatten is on the account, not in the preset) |
| Máx. contratos | as published (minis and micros per the preset) |
| Reset do dia | `17:00 CT (19:00 GMT-3)` (F§6.4: both zones in a tooltip) |
| Regra de consistência | text or `Não confirmado` |

- Each value: `Info` icon-button with the source popover (F§7.4 verified); `Não confirmado` badge and the list of published values with their sources (read-only here; the choice is made on the account, S16-03).
- Dense table: horizontal scroll inside the container with a sticky first column (F§9). Money exactly as published, strings (X-3).
- **Excluded plans note** at the bottom (AC-9): "Estes planos não estão disponíveis como preset: {list}. Cadastre-os manualmente." (list from the Core, not hard-coded) + link `Cadastrar conta sem preset` (S5 form, Q40 path).
- **Evaluation profit target (AC-10, Q46 decided 2026-10-23e):** for presets of an **evaluation** phase the table has the column `Meta da avaliação` (amount, with the F§7.4 source popover and access date; `Não confirmado` + published values when the pages conflict; `—` for funded and live phases). It feeds the S13b progress card ([S13 spec §12](v1-s13-risk.md)). **Minimum trading days and payout rules are stored in the preset and not shown in V1.** The *daily* profit target (S13 AC-5) is part of AC-1 and stays shown.
- **Commission (AC-12):** column `Comissão por contrato/lado` (USD as published, source and date). It prefills the account's commission floor (S13 §4.2b). When the firm does not publish it: `—` with the note "A firma não publica a comissão. A conta mostrará o aviso 'comissão não informada' até você informar um valor." / "The firm does not publish its commission. The account will show 'commission not informed' until you enter a value."

### 4.3 Tab "Versões"

List (newest first): `Versão {n}` · published date · verification date · summary of what changed ("Mudou a posição sobre automação: permitida → proibida" or "Texto da regra de automação alterado — {reason}") · badge `Mudança material` (warning) when the publisher flagged it (Q42 provisional: flagged by the publisher with a reason). Read-only; each version's stance quotes are viewable (`Ver textos desta versão`), so the user can always compare what an account was shown.

---

## 5. S16-03 Preset picker and rules preview (account form)

Placed in the S5 form between "Tipo" and "Dados da conta" for prop accounts. The picker only reads the catalog; the account form saves.

**Cascading fields** (each enabled after the previous one): `Firma` (searchable combobox) → `Plano` → `Fase` → `Tamanho` → `Plataforma da conta na firma` (only when the preset has platform-specific rules; S5 §9.1).

- Firm option: name + `Só acompanhamento manual` badge + "{n} itens não confirmados" in warning text when present (O8).
- Footer of the firm list: `Não encontrou sua firma ou plano?` → popover: "Planos fora dos presets podem ser cadastrados manualmente." + `Cadastrar sem preset` (switches the form to the free-text path, S5 §5.1 row "Prop without a preset", **provisional Q40**).
- A firm with no verified data does not appear (AC-5); no placeholder entry.

**Rules preview** (`Card` "Regras deste preset", appears when size [and platform] are chosen; it is the data that fills the account, AC-2):

- Header: "{firm} · {plan} · {phase} · {size} · versão {n} · verificado em {date}".
- Stance block (compact `StanceBlock` ×4, quotes collapsed behind `Ver regra oficial`): this satisfies AC-4 at registration.
- Rule rows, each with its provenance mark (F§7.4):
  - **Verified:** value + `Info` source popover.
  - **Not confirmed:** `Não confirmado` badge, warning border, **no value**; lead lines and control by reason (`conflicting` → `ConflictPicker` with no preselection; `partial` → `FirmRuleQuote` of the partial text, untranslated, plus the field; `not_published` → search date and sources plus the field), exactly as in S5 §9.2 (`not_confirmed_reason`, preset schema 1.2 committed; HTTP 6.3.0 exposure **TODO**). Required fields block saving: "Resolva este valor para salvar." Required: maximum position, drawdown, daily loss limit, reset time (S16 AC-6). Optional or informational (for example the consistency rule): can stay unresolved; the account then shows "Regra de consistência não confirmada — não acompanhada" (PO).
- Summary `Alert` (info) when any item is not confirmed: S5 §9.2 text.
- The user can edit any filled value before saving (it becomes `Editado`, S16-04). The maximum position cannot exceed the preset's (S5 AC-3).
- After the user resolves a not-confirmed value, it carries `Definido por você` (not `Editado`) and the provenance popover of S5 §9.2 (`conflicting`: "Escolhido por você entre valores divergentes em {date}"; `partial` / `not_published`: "Informado por você em {date}"), with `Revisar`.

**States:** loading → skeleton in the preview; preset service error → `ErrorState` inside the preview area with retry; a prop account cannot be saved until it loads (rules and choice depend on it; real accounts are unaffected); no matching plan after a change of selection → the later fields reset and a polite message "Selecione de novo: o plano mudou."

Changing firm, plan, phase, size or platform after editing values asks first: "Trocar a seleção substitui {n} valores que você editou. Continuar?" (`AlertDialog`, `Cancelar` focused), then S5 §5.2 may clear the opt-in choices.

---

## 6. S16-06 Manual-only presets (AC-7)

For a firm whose platforms cannot be reached by any V1 output (for example Topstep):

- **Catalog and firm detail:** `Só acompanhamento manual` badge and an info `Alert` with the reason from the preset ("A Topstep só aceita ordens originadas do seu dispositivo pessoal…" — the preset's own text wins) and a two-column explanation: **O que funciona:** regras de risco, alertas de 80 %, operações manuais e desempenho. **O que não funciona:** roteamento de sinais e replicação.
- **Picker:** the same `Alert` appears when the firm is chosen, before the user fills the rest; the form still saves normally. Link `Ver regra oficial`.
- **Account detail:** the same `Alert` stays at the top; `Rotas` tab shows the S7 §7 explanation; the platform-account binding is optional (Q37, S5 AC-12).
- The account never appears in routing or as master/follower choices (S7 AC-7b).
- **Driven by preset data, not by a firm flag:** the manual-only state, its reason and the `Alert` come from the preset (and the platform of the preset when the rule differs by platform). Research decision S-R3-4 (user 2026-10-24, option (a); the preset data carries the outcome) may apply "manual-only" **per platform**, not per firm: the UI reads it per preset so that needs no redesign.

---

## 7. S16-07 `StanceBadge` and `StanceBlock`

**Badges** (F§7.2 tokens; always text + icon):

| Value | Label automation / copy | Token, icon |
|---|---|---|
| `allowed` | Permitida / Allowed | success, `CircleCheck` |
| `forbidden` | Proibida / Forbidden | danger, `Ban` |
| `conditional` | Condicional / Conditional | warning, `TriangleAlert` |
| `unknown` | Desconhecida / Unknown | neutral, `CircleHelp` |
| `not_confirmed` | Não confirmada / Not confirmed | warning, `CircleHelp` |

For **VPS/server**: `allowed` / `forbidden` / `conditional` / `unknown`, plus `not_confirmed` when the firm's pages conflict (PO, S16 AC-5/AC-8), with the same badges (`Permitido` / `Proibido` / …). For **personal device required**: contract values `required` / `not_required` / `unknown` → `Exigido` / `Não exigido` / `Desconhecido` (neutral/warning). **`not_confirmed` for this subject is not in contract 5.1.1** (`presets.yaml` enum is `required`, `not_required`, `unknown`); S16 AC-5 says every conflicting field ships as not confirmed, so this is a **contract request to the Architect (Q-S16-7)**; until it exists the UI shows `Desconhecido` **with the notice "As páginas oficiais divergem sobre este ponto." / "The official pages disagree on this point."** and the published conflicting texts, so the user sees a disagreement and not only an unknown; never a chosen value. **The quotes are shown even though the value is `unknown`**: the block lists every published text with source and date. Verbatim text, source and date as for the other stances. *(Value names in the contract: Architect.)*

**Fixed summary per value** (the "Resumo do All Win"; `{firm}` = firm name; automation and copy for all five values are in [S5 §5.3](v1-s5-automation-opt-in.md); `allowed` for those two uses the S5 §5.1 lines). VPS/server and personal device, which S5 does not cover:

| Subject, value | pt-BR | en |
|---|---|---|
| VPS `allowed` | "A {firm} permite o uso de VPS/servidor." | "{firm} allows using a VPS/server." |
| VPS `forbidden` | "A {firm} proíbe o uso de VPS/servidor nas regras oficiais." | "{firm} prohibits using a VPS/server in its official rules." |
| VPS `conditional` | "A {firm} só permite VPS/servidor sob condições. Leia a regra." | "{firm} only allows a VPS/server under conditions. Read the rule." |
| VPS `unknown` | "Não encontramos uma regra oficial da {firm} sobre VPS/servidor." | "We found no official {firm} rule about a VPS/server." |
| VPS `not_confirmed` | "As páginas oficiais da {firm} dizem coisas diferentes sobre VPS/servidor." | "{firm}'s official pages disagree about a VPS/server." |
| Device `required` | "A {firm} exige que as ordens saiam do seu dispositivo pessoal." | "{firm} requires orders to come from your personal device." |
| Device `not_required` | "A {firm} não exige dispositivo pessoal." | "{firm} does not require a personal device." |
| Device `unknown` | "Não encontramos uma regra oficial da {firm} sobre dispositivo pessoal." | "We found no official {firm} rule about a personal device." |
| Device, conflicting pages (Q-S16-7) | badge `Desconhecido` + "As páginas oficiais divergem sobre este ponto." | "The official pages disagree on this point." (all quotes shown; no value chosen) |

`StanceBlock` = heading + badge + All Win summary (labeled "Resumo do All Win (não é aconselhamento jurídico)") + `FirmRuleQuote`(s) + the verification line "Verificado em {date}". Used by the catalog, the picker, the S5 opt-in section and the S7 route dialog (in compact form). **No other disclaimer line:** the label "Resumo do All Win (não é aconselhamento jurídico)" is the only one; an extra sentence such as `stances.disclaimer` is removed (design decision 2026-10-24, reversible).

---

## 8. S16-04 Edited values and S16-05 new version

### 8.1 Edited values (AC-2)

On any screen that shows an account rule copied from a preset (S13 risk tab, account detail):
- Unchanged value: shown normally; verified source popover.
- Changed by the user: `Editado` badge (`PencilLine`); popover: "Valor do preset: {value} · fonte… · versão {n}" + `Restaurar valor do preset` (copies the preset value to the account, audited as an account edit). The preset is never modified.
- Editing a value never changes other accounts or the preset (AC-2).

### 8.2 New preset version (AC-3)

- **Notice:** on the account detail, an info `Alert`: "Há uma nova versão do preset da {firm} ({plan}, {phase}, {size}): versão {n}, de {date}." + `Ver mudanças`. Also on the account list as a small `Preset novo` text link in the state area (not a badge, to keep the list calm), and a notification (S18): "Nova versão do preset da {firm}: {k} contas podem ser atualizadas." → `/accounts?preset_update=1` (filter).
- **If the new version changes the automation or copy stance or its text**, the S5 §6 suspension applies **even before** the user applies the version (PO confirmed); the notice then says: "Esta versão mudou a regra de automação. Ordens automatizadas estão suspensas até você revisar." and the `Revisar agora` of S5 §6 appears.
- **Comparison dialog** (`ResponsiveDialog`, full screen on mobile) "Mudanças da versão {n}": table `Campo` | `Na sua conta` | `Versão atual do preset` | `Nova versão`, with only the fields that differ. Each row shows provenance marks. For fields the user had **edited**, the row says "Você editou este valor" and asks to choose: `Manter meu valor` / `Usar valor do preset` (**nothing preselected**; required to continue). For not-confirmed fields: `ConflictPicker` as in S16-03.
- Maximum position: if the new cap is below the account's value, the row is marked "A nova versão limita a {n} contratos" and `Aplicar` is disabled until the user lowers it (S5 AC-3 cap).
- If the rule version changes for the opt-in, the dialog includes the S5 `OptInSection` (automation and, if the account is a follower, copy), and the update request carries the new `opt_ins` (D16); step-up if `accepted`.
- Actions: `Aplicar versão {n}` (primary) · `Manter versão atual` (secondary; the notice stays and no change is made) · `Cancelar`. Applying is one save, audited with the old and new version. Duplicate submit protected (F§10).
- Applying a version never touches open positions or working orders.

**States:** loading skeleton; error with retry (values kept); stale (`accounts.rule_version_changed`, a newer version appeared meanwhile) → the dialog reloads the table, selections are cleared with `Alert` "Saiu uma versão mais nova enquanto você revisava."; account changed elsewhere (`common.version_conflict`) → reload.

---

## 9. Data and actions this slice needs (for the Architect)

**Reads**

| Need | Used by |
|---|---|
| Firms list: id/slug, name, `manual_only` + reason text, counts of plans, stances per kind (`FirmStance`, VPS, device), `not_confirmed_count`, verification date, version number | S16-01 |
| Firm detail: stance blocks with quotes (text, language, page title, URL, access date), research sources for `unknown`; plans → phases → sizes → platforms; per-size rule values with **provenance per field** (verified / not confirmed + published values with source URL and date); excluded-plans list | S16-02 |
| Version history per preset: number, published and verification dates, change summary, `material_change` flag + reason (Q42), per-version quotes | S16-02 tab, S16-05 |
| Catalog for the account form: cascading option lists with `not_confirmed_count` and `manual_only`; the **preview** for a full selection (rules, provenance, stances, `OptInState` content, `current_rule_version_id`) before any account exists | S16-03, S5 §14 |
| For an account: `preset_version_id`, edited-fields set with the preset value, available newer version | S16-04, S16-05 |
| **Version diff** for an account: fields that differ (account / current preset / new), edited flags, cap conflicts, whether the stance or rule text changes | S16-05 |

**Writes:** preset selection is part of `createAccount` / `updateAccount` (S5 §14); `applyPresetVersion` (account, target version, per-field choices for edited and not-confirmed fields, `opt_ins`, `row_version`); `restorePresetValue` (account, field).

**Events (SSE):** new preset version available for the user's accounts (to refresh the notice).

**Codes the UI maps:** `presets.version_superseded` (409 on apply when a newer version appeared) and `presets.version_not_newer` (422); `accounts.rule_version_changed` for the opt-in rule version, `presets.required_field_unresolved` (pointer to the field), `accounts.max_position_exceeds_preset`, `common.version_conflict`, `common.not_found` (names proposed).

---

## 10. Decisions and open questions

**Closed by the PO (Rumo, 2026-10-23):** Q-S16-2 VPS = allowed / forbidden / conditional / unknown + not confirmed on conflict, device = yes / no / unknown, verbatim text, source and date; Q-S16-4 per-field choice with nothing preselected when applying a version (S16 AC-3/AC-6); Q-S16-5 one notification per firm (S18); Q-S16-6 the manual-only reason is preset data (verbatim, source, date), the frame around it is pt-BR/en UI text.

**Architect (Compasso):** Q-S16-3 stored provenance for a value the user resolved among conflicting ones.

**Closed by the user (Q46, 2026-10-23e):** the evaluation profit target is shown with source and date for evaluation phases; minimum trading days and payout rules are stored and not shown in V1.

**Decided by the user (2026-10-23e), no longer provisional:** Q37 the platform link is optional (S5 AC-12; an unlinked account is tracked manually only and never receives routing, automation or enforcement; manual-only firms cannot be linked); Q40 before the catalog a prop account's firm, plan and phase are free text and the stance is "unknown" (S5 AC-15); Q41 attaching a preset later puts the account in review pending (S5 AC-16); Q42 a material text change flagged by the publisher with a reason suspends automated sources until the user confirms again (S5 AC-11).

## 11. Accessibility

- Cascading selects are comboboxes with type-ahead, `aria-expanded`, Escape closes and restores focus; a dependent field disabled until the previous one is set has `aria-disabled` and a description of why.
- Dense tables keep `th scope`, sticky first column and a visible focus inside the scroll container; card layout below `md`.
- Provenance and stance are text + icon (F§1). `FirmRuleQuote` uses `lang` of the source; links say where they go and that they open a new tab.
- The comparison dialog is a labeled `table` in a `Dialog`; rows requiring a choice use `RadioGroup` with visible legends; focus returns to `Ver mudanças` on close.
- Targets ≥ 44 px on touch; reflow at 320 px.

## 12. AC traceability

| AC | Where |
|---|---|
| AC-1 preset fields (rules, platform, stances, sources, dates, version) | §2, §4.2, §9 |
| AC-2 selecting fills rules and max position; edit the copy without changing the preset | §5, §8.1 |
| AC-3 new version: told, user chooses | §8.2 |
| AC-4 stance with source and date at registration and routing | §4.1, §5, S7 spec §6 |
| AC-5 selectable if verified or not confirmed; no verified data → not offered | §2, §5 |
| AC-6 not confirmed not prefilled; required blocks save | §5 |
| AC-7 manual-only | §6 |
| AC-8 VPS rule and personal device recorded and shown | §4.1, §7 |
| AC-9 excluded plans registered manually | §4.2, §5 footer |
| AC-10 evaluation profit target shown with source and date | §4.2, S13 §12 |
| S5 AC-11 new version changing a stance suspends automation until confirmed | §8.2 |
| AC-11 applying a new version is done field by field, nothing preselected, one notification per firm | §8.2 |
| AC-12 commission per contract per side, prefill, empty + warning | §4.2, S13 §4.2b |
| X-3, X-5, X-6 | strings for money, audit of apply/restore (S10 catalog), pt-BR/en |
