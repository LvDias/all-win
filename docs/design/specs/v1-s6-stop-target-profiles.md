# V1-S6 — Stop and target profiles — Screen spec

> **Owner:** Product Designer (Traço @ Dev3) · **Status:** proposed, pending PO review · **Revision:** 1 · **Date:** 2026-10-23
> **Slice:** [V1-S6](../../product/release-1/slice-plan.md) AC-1 … AC-5 (slice-plan rev. 7); touch points: S5 (accounts, sizes), S8 (shared mode, executions), S10 (audit of the rounding).
> **Foundations:** [foundations.md](../foundations.md) (F§6.2 distance units, F§6.3 number input) · **Related:** [S5 spec](v1-s5-automation-opt-in.md) (nav §0, account detail tabs, account form).
> **Contract:** no HTTP contract for profiles yet; §9 lists what the screens need. Tables: `accounts.default_bracket_profile`, `accounts.account_bracket_profile` ([data model §5.4](../../architecture/data/wave1-data-model.md)). Normalization to whole ticks: ADR-0004 §5. Operation names here are **design proposals**; the contract wins.
> **Surfaces:** Web (Frontend). Tick conversion, rounding and audit are the Core's (Backend). QA owns the checks.

---

## 1. What this spec covers

| # | Screen | Route |
|---|---|---|
| S6-01 | Default profiles by account size | `/[locale]/profiles` (sidebar `Perfis de stop e alvo` / `Stop and target profiles`) |
| S6-02 | Edit one default profile (dialog) | over S6-01 |
| S6-03 | Account profile tab (own copy, customize) | `/[locale]/accounts/[id]/profile` |
| S6-04 | Shared component `DistanceFields` (unit + stop + target + Core preview) | used by S6-02 and S6-03 |

---

## 2. Concepts shown to the user (no new rules)

- A **profile** = stop distance + target distance in one **unit** (points, ticks or dollars, chosen per profile; AC-1), for one **instrument**. V1 has only MNQ; profiles are keyed by instrument so new futures need no rework (Q-A2).
- **Default profiles** exist per standard size: **25k, 50k, 100k, 150k, 200k, 300k**, including sizes the user does not hold (AC-1).
- A **new account starts with the profile of its size** (a real account of another size uses the nearest standard size) and can be customized **without changing the default** (AC-2, Q12).
- **Individual mode (default):** each entry gets its own stop and target from its own fill price; a second entry does not move the first entry's orders (AC-3). Shared mode is S8.
- **Stop and target values coming in an instruction are ignored; the profile always wins** (AC-5).
- Distances in dollars are converted to **whole ticks** with the instrument's exact tick value. A distance that is not a whole number of ticks is **rounded toward the entry**, for stop and target, so risk is never more than configured. The adjustment and the resulting price appear in the audit entry (AC-4, Q-N1).

The conversion happens in the **Core**. The browser never converts or rounds money; it shows what the Core returns (X-3, ADR-0001 §8).

---

## 3. S6-01 Default profiles — `/[locale]/profiles`

**Purpose:** define the starting stop/target for each account size (AC-1).
**Permissions:** any signed-in user; own profiles only (X-2).

**Layout:** `PageHeader` "Perfis de stop e alvo" · intro line · instrument chip · table (cards below `md`, F§9).

- Intro: "O perfil define a que distância ficam o stop e o alvo de cada entrada. Valores que chegam no sinal são ignorados: vale sempre o perfil." / "The profile sets how far the stop and target sit from each entry. Values in the signal are ignored; the profile always wins."
- Instrument: a read-only chip `MNQ` in V1. When the Core returns more than one instrument, it becomes a `Select` (`aria-label` "Instrumento"); selection in the URL (`?instrument=`).

| Column | Content |
|---|---|
| Tamanho / Size | `US$ 25.000,00` … `US$ 300.000,00` (money, F§6.2) |
| Unidade / Unit | `Pontos` · `Ticks` · `Dólares` or `—` |
| Stop | distance with unit (`12,50 pts` · `50 ticks` · `US$ 25,00`, F§6.2) or `Não definido` |
| Alvo / Target | same |
| Estado / Status | `Definido` (neutral) · `Não definido` (warning, `CircleHelp`) |
| Ações | `Definir` / `Editar` (opens S6-02) |

**States**

| State | UI |
|---|---|
| Loading | Skeleton with 6 rows |
| Error | `ErrorState` + retry + code |
| A size has no profile | row shows `Não definido`; a banner (info `Alert`) above the table: "Contas de um tamanho sem perfil não têm stop e alvo definidos. Novas entradas nessas contas são ignoradas e registradas na auditoria (\"sem perfil de stop/alvo\"); saídas continuam. Defina o perfil." **[Provisional, Q43 rec. (a); user decides. No initial values are invented: the user defines the defaults (S6 AC-1).]** |
| Saved | toast "Perfil de {size} salvo." |

No pagination, no sorting controls: six fixed rows in size order.

---

## 4. S6-02 Edit one default profile

`ResponsiveDialog` "Perfil de {size} · {instrument}" with `DistanceFields` (S6-04).

- Note at the top: "Vale para contas novas deste tamanho. Contas existentes mantêm o próprio perfil." **[Confirmed by PO: only new accounts are affected; each account keeps its own copy (S6 AC-2).]**
- Actions: `Salvar perfil` (primary), `Cancelar`. Submit: spinner + "Salvando…", disabled, duplicate-submit ignored (F§10). Unsaved changes + Cancel → `AlertDialog` "Descartar alterações?".
- Errors: 422 with `pointer` mapped to `/stop_distance`, `/target_distance`, `/distance_unit`; other errors as a destructive `Alert` in the dialog; values kept.
- Success: closes, focus returns to the row's button, toast, audited (§8).
- No step-up (not in the S3 list, Q35 recommended list).

---

## 5. S6-04 `DistanceFields` (shared)

| Field | Component | Validation (client; the Core is the authority) |
|---|---|---|
| Unidade / Unit | `RadioGroup` of three cards: `Pontos` / `Points`, `Ticks`, `Dólares` / `Dollars (US$)`. Nothing preselected for a new profile | required |
| Stop | `Input inputMode=decimal` (`numeric` for ticks) with the unit as a suffix | required; > 0; **points:** multiple of the tick size ("Use múltiplos de 0,25" / "Use multiples of 0.25", values from the Core); **ticks:** whole number ("Digite um número inteiro de ticks."); **dollars:** ≤ 2 decimals **[proposal; the data model allows 4, Architect to widen if needed]** |
| Alvo / Target | same | same |

Number parsing follows F§6.3 (no thousands separators; pt-BR accepts `,` and `.`). The value sent is a canonical decimal string.

**Preview (from the Core, debounced 300 ms after both fields are valid):** a read-only block `Equivale a` under the fields:

- Stop: `50 ticks · 12,50 pts · US$ 25,00`; Alvo: same.
- If the unit is **dollars** (or points not a whole number of ticks) and the Core had to round, an info `Alert`: "US$ 100,10 não é um número inteiro de ticks. Será usado {n} ticks (US$ 100,00), arredondado em direção à entrada para nunca arriscar mais do que o configurado." / English equivalent. The text is built from fields the Core returns; the browser only inserts them.
- While loading: the block shows `Skeleton` and keeps the last result dimmed (`aria-busy`). On preview failure: muted line "Não foi possível calcular o equivalente agora. Você ainda pode salvar." The save still validates on the server.
- The block is `aria-live="polite"`; it announces only when the text changes after the user stops typing.

Help under the unit: "Dólares são convertidos em ticks inteiros usando o valor exato do tick do instrumento." / "Dollars are converted to whole ticks using the instrument's exact tick value."

---

## 6. S6-03 Account profile tab — `/[locale]/accounts/[id]/profile`

**Purpose:** see and customize the account's own profile (AC-2) without touching the default.
**Permissions:** own account only; another user's id gives S5 `not found`.

**Layout:** tab `Perfil` on the account detail (S5 §8), content in `Card`s:

1. **Perfil de stop e alvo ({instrument})** card: unit, stop, target (formatted per F§6.2), the "Equivale a" values from the Core (ticks/points/dollars), and the origin line:
   - copied: "Copiado do perfil padrão de {size} em {date time zone}." / for a real account of another size: "Baseado no perfil padrão do tamanho mais próximo ({size}), copiado em {date}."
   - customized: badge `Personalizado` / `Customized` (`PencilLine`, as F§7.4 `Editado`) + "Padrão do tamanho {size} hoje: {values}" in a popover.
   - Actions: `Personalizar` / `Editar` (opens the same `DistanceFields` dialog, S6-02 layout, title "Perfil da conta {name}"); `Restaurar padrão do tamanho` (`AlertDialog`: "Substituir o perfil desta conta pelo padrão atual de {size}?" confirm `Restaurar`) **[Confirmed by PO: copies the current default to the account, audited]**.
2. **Modo de stop e alvo** card: "Individual (padrão)" with the AC-3 explanation: "Cada entrada tem o próprio stop e alvo, calculados a partir do preço em que ela foi preenchida. Uma segunda entrada não move as ordens da primeira." The shared-mode selector belongs to S8 and is **not drawn here** (confirmed by PO: per-account setting owned by S8 AC-5; S6 shows `Individual` read-only).
3. A static note: "Valores de stop e alvo que chegam no sinal são ignorados."

**Edit rules:** saving changes only this account (copy). Open positions and working orders are not modified; note in the dialog: "Vale para as próximas entradas. Posições e ordens abertas não são alteradas." **[PO: edits apply to the next entries only, S6 AC-3]**.

**States**

| State | UI |
|---|---|
| Loading | Skeleton of both cards |
| Account has no profile (its size had no default when created) | card with `EmptyState` "Esta conta ainda não tem perfil. Novas entradas são ignoradas até você definir um." + `Definir perfil` (same dialog, nothing preselected) **[Provisional, Q43]** |
| Error / not found | per S5 §2 |
| Concurrent edit | 409 `common.version_conflict` → toast "O perfil mudou em outro lugar." + refetch |
| Saved | toast "Perfil da conta salvo." |

---

## 7. Strings (examples, pt-BR / en; Frontend creates the keys)

| Where | pt-BR | en |
|---|---|---|
| Units | Pontos · Ticks · Dólares | Points · Ticks · Dollars |
| Not defined | Não definido | Not defined |
| Customized | Personalizado | Customized |
| Dialog title | Perfil de {size} · {instrument} | {size} profile · {instrument} |
| Rounding | Será usado {n} ticks ({value}), arredondado em direção à entrada. | {n} ticks ({value}) will be used, rounded toward the entry. |
| Whole ticks | Digite um número inteiro de ticks. | Enter a whole number of ticks. |
| Tick multiple | Use múltiplos de {tick}. | Use multiples of {tick}. |

---

## 8. Audit events emitted (render in S10)

| Event (code, proposed) | Entry text | Details |
|---|---|---|
| `profiles.default_changed` | "Perfil padrão de {size} alterado" | instrument, previous and new unit/stop/target (strings), user |
| `profiles.account_changed` | "Perfil da conta {account} alterado" / "…restaurado do padrão" | same + `copied_from_size` |
| `routing.no_bracket_profile` *(proposal, Q43 provisional)* | "Entrada ignorada para {account}: sem perfil de stop/alvo" | instruction id, account size |
| Execution entries (S10 AC-1) | "Stop e alvo calculados" with unit, ticks used, **adjustment** (requested value → whole ticks → resulting price) | defined in the S10 spec; the Core supplies the fields |

---

## 9. Data and actions this slice needs (for the Architect)

**Reads**

| Need | Notes |
|---|---|
| Instruments available with `tick_size`, `tick_value` (strings), display precision | Never hard-coded in the UI |
| Standard sizes list (6) | from `catalog.standard_account_size` |
| Default profiles of the user: `size_amount`, `instrument`, `distance_unit`, `stop_distance`, `target_distance`, `updated_at` | missing rows = `Não definido` |
| Account profile: same + `copied_from_size`, `customized` flag, `row_version` | |
| **Profile preview** (stateless): given `instrument`, `unit`, stop, target → whole ticks, points, dollars (strings) for each, `rounded` boolean per side | needed by `DistanceFields`; no persistence |

**Writes**

| Action | Notes |
|---|---|
| `putDefaultProfile` (instrument, size, unit, stop, target) | idempotent upsert; 422 pointers `/distance_unit`, `/stop_distance`, `/target_distance`; codes e.g. `profiles.not_whole_ticks`, `profiles.not_tick_multiple`, `profiles.not_positive` (names proposed) |
| `putAccountProfile` (account, instrument, unit, stop, target, `row_version`) | `common.version_conflict` |
| `restoreAccountProfile` (account, instrument) | copies the current default of the account's size; audited (`profiles.account_changed`, "restaurado do padrão") |

**Events (SSE):** none required; the tab refetches on focus.

---

## 10. Decisions and open questions

**Closed by the PO (Rumo, 2026-10-23):** Q-S6-1 the mode is per account (S8 AC-5), read-only `Individual` here; Q-S6-2 no invented values, `Não definido` + banner; Q-S6-3 editing a default affects only new accounts; Q-S6-4 `Restaurar padrão do tamanho` accepted (copies the current default, audited); Q-S6-5 edits apply to the next entries only.

**Provisional until the user decides Q43:** an entry that reaches an account with no stop/target profile is **ignored and audited** with the reason "sem perfil de stop/alvo" (`routing.no_bracket_profile`, proposed code); exits continue (P1). The banner in §3 and the empty state in §6 say so.

**Architect (Compasso):** Q-S6-6 dollar input decimals (form uses 2; data model allows 4); Q-S6-7 which size a real account between two sizes uses ("nearest"), computed by the Core; the new reason code and the preview endpoint (§9).

## 11. Accessibility

- Unit `RadioGroup` with a visible legend; stop/target inputs have labels including the unit ("Stop, em ticks"). Errors are linked and focus moves to the first invalid field.
- The preview region is polite and does not announce each keystroke; the rounding `Alert` is not a live alarm.
- The table has `th scope`; row buttons read "Editar perfil de {size}".
- `Personalizado` badge carries text; the popover with the default values opens on focus/tap, not hover only (F§9).
- Dialogs trap focus and return it to the trigger. Targets ≥ 44 px on touch.

## 12. AC traceability

| AC | Where |
|---|---|
| AC-1 default per size 25k–300k, any unit, keyed by instrument | S6-01, S6-02, S6-04 |
| AC-2 new account starts with size profile; customize without changing default | S6-03, §4 note |
| AC-3 individual mode default | §2, S6-03 card 2 |
| AC-4 dollars → whole ticks, rounded toward entry, adjustment audited | S6-04 preview, §8 |
| AC-5 profile wins over the instruction | §3 intro, S6-03 note |
| X-3, X-5, X-6 | Core conversion; §8; §7 |
