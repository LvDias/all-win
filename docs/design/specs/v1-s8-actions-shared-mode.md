# V1-S8 — Full actions and shared mode — Screen spec

> **Owner:** Product Designer (Aquarela, taking over D3 from Traço) · **Status:** proposed, pending PO review · **Revision:** 1 · **Date:** 2026-10-23
> **Slice:** [V1-S8](../../product/release-1/slice-plan.md) AC-1 … AC-6 (slice-plan rev. 7). Touch points: S6 (mode card, read-only `Individual`), S7 (route dialog), S10 (audit of every action), S18 (rejection notice).
> **Foundations:** [foundations.md](../foundations.md) · **Components:** [components.md](../components.md) · **Related:** [S6 spec](v1-s6-stop-target-profiles.md) §6, [S10 spec](v1-s10-audit-log.md), [S5 spec](v1-s5-automation-opt-in.md) §0 (nav tree).
> **Contract:** [HTTP contract 5.1.1](../../../contracts/http/openapi.yaml) (commit 7872ca9). It has no S8 operation or mode field (searched `contracts/http/modules/*.yaml`); the rejection of an opposite entry exists as `routing.position_conflict` (notification, S18 catalog). Everything marked **[Contract gap]** below is a request to the Architect through the Maestro; names are proposals.
> **Surfaces:** Web (Frontend). Execution, FIFO and recalculation are the Core's and the plugin's. QA owns the checks.

## 0. What S8 is on screen

S8 is mostly execution behavior with no screen of its own. The user sees it in three places only:

1. **One setting:** the stop/target **mode** of an account (`Individual` / `Compartilhado`), in the card S6 already reserves (S6-03, card 2).
2. **Audit and activity rows** for what each instruction did (S10), including the rejection of an opposite entry.
3. **A notice** when an entry is rejected because it was opposite to the open position (AC-4, "the user is notified").

No new route, no new menu item. Nothing else is drawn: the browser never decides what an instruction does.

---

## S8-01 Mode card — `/[locale]/accounts/[id]/profile` (card "Modo de stop e alvo")

**Purpose:** let the user choose how stop and target are managed when there is more than one entry in the same position (AC-5). Replaces the read-only text of S6-03 card 2.

**Entry point:** tab `Perfil` of the account detail. **Permissions:** owner of the account only (cross-user access returns 404 as elsewhere). The card is shown for every account kind; the choice is meaningful only where stop and target are placed by All Win.

**Layout:** `Card` with `RadioGroup` of two option cards, `Salvar` in the card footer.

| Option | Title | Explanation (pt-BR / en) |
|---|---|---|
| `individual` (default) | Individual | "Cada entrada tem o próprio stop e alvo, calculados a partir do preço em que ela foi preenchida. Uma segunda entrada não move as ordens da primeira." / "Each entry has its own stop and target, calculated from its own fill price. A second entry does not move the first one's orders." |
| `shared` | Compartilhado / Shared | "A posição tem sempre um único stop e um único alvo, recalculados a partir do preço médio depois de cada execução, com as distâncias do perfil da conta." / "The position always has one stop and one target, recalculated from the average price after each fill, using the account profile distances." |

- Under the group, a static line for both: "Valores de stop e alvo que chegam no sinal são ignorados." (kept from S6).
- `Individual` also states the exit rule (AC-4b): "Em saída parcial, o stop e o alvo são reduzidos começando pela entrada mais antiga (FIFO)." / "On a partial exit, stop and target quantities are reduced starting with the oldest entry (FIFO)." Shown only under the `Individual` option.
- The selected option is the saved value; nothing is preselected differently from the Core's value. The state is read on load (`Skeleton`, then `ErrorState` with `Tentar novamente`).

**Fields and API mapping**

| Field | Component | Validation | API |
|---|---|---|---|
| Modo / Mode | `RadioGroup` | required; one of `individual`, `shared` | read: account detail; write: `PUT updateAccount` **[Contract gap: the account field (e.g. `bracket_mode`); the values `individual` / `shared` already exist in `DetailsBracketCalculated.mode`]** |

**Changing the mode while a position or working orders exist is refused (Q86 (b), decided by the user 2026-10-24).** Shared mode recalculates from the average price, so a mid-position switch would silently change protection. With an open position or working orders the `RadioGroup` stays visible but `Salvar` is `aria-disabled` with the reason under the field: "Não dá para mudar o modo com posição aberta ou ordens de trabalho. Zere a conta e cancele as ordens primeiro." / "The mode can't be changed with an open position or working orders. Flatten the account and cancel the orders first." If the state changes between the screen and the save, the Core refuses and the dialog shows the translated refusal code in an inline `Alert` (**[Contract gap: refusal code, request to the Architect]**); nothing is saved.

**Actions and flow:** `Salvar` (disabled until the value differs; no-op save is not sent) → pending state on the button (`aria-busy`, disabled, so a double click sends once) → success: `Toast` "Modo de stop e alvo atualizado." and the card shows the new value → the change is audited (S10).
**Cancel / dirty guard:** leaving the tab with an unsaved selection opens `Descartar alterações?` (F§10).

**States**

| State | UI |
|---|---|
| Loading | `Skeleton` of the card |
| Error (read) | `ErrorState` with retry; the rest of the tab stays |
| Saving | button busy, radios disabled |
| Save error | inline `Alert` in the card (stable code translated by key); selection kept; retry possible |
| Conflict (`409`/stale) | `Alert` "A conta mudou. Recarregue para ver o valor atual." + `Recarregar`; selection kept until reload |
| Success | toast + card shows saved value |
| Manual-only or unlinked account (no orders placed by All Win: S5 AC-12, S16 AC-7) | the card is **hidden**; the Perfil tab shows only the profile |
| Account paused/disabled | radios are enabled (the mode is a setting, not an order); the card does not claim anything about running orders |

**Accessibility:** `fieldset`/`legend` "Modo de stop e alvo"; each option card is a real radio with its explanation as `aria-describedby`; arrow keys move the choice; the selected state is text ("Selecionado" sr-only) plus border, not color alone; 44 px touch targets (F§9).

---

## S8-02 Instruction outcomes in the activity view (S10)

S8 adds no screen; it fixes the **wording and fields** S10 shows for the actions, so the Core can return stable codes and the client translates them. Rows follow the S10 layout (reason first, details in the `Sheet`). Codes are **proposals [Contract gap: audit code names belong to the Architect]**.

| Case | AC | Row text (pt-BR) | Details in the sheet |
|---|---|---|---|
| Entry opens or adds | 1 | "{account}: {Compra\|Venda} de {qty} {instrument} executada" (adds: "…aumentou a posição de {a} para {b}") | quantity = instruction × multiplier (strings), prior and new position, fill price |
| `close` | 2 | "{account}: posição de {instrument} fechada; stops e alvos cancelados" | "Quantidade do sinal ignorada" when the instruction carried one |
| `close` when flat | 2 | "{account}: nada a fechar em {instrument}" (status *Ignorada*) | reason `routing.nothing_to_close`, which per the PO occurs only on an in-sync flat account (an out-of-sync believed-flat account is sent as a flatten: Q75 appears in contract 5.2.0 notes and the notifications registry, not yet in decisions.md or the slice plan; TODO until the PO records it) |
| `reverse` | 3 | "{account}: posição invertida em {instrument}: de {dir} {a} para {dir} {b}" | quantity cap from S7 AC-4 if it applied (“limitada pelo máximo da conta”) |
| `reverse` when flat | 3 | "{account}: sem posição; aberta {dir} {b}" | — |
| **Opposite entry rejected** (`routing.position_conflict`) | 4 | "{account}: **entrada recusada** — {Compra\|Venda} contra posição {vendida\|comprada} aberta. Envie fechar ou inverter." | account, instrument, current position, instruction direction; "Outras contas não foram afetadas." when the same instruction ran elsewhere |
| Partial exit (individual) | 4b | "{account}: saída parcial de {q}; stops e alvos reduzidos pela entrada mais antiga" | per-entry quantities removed (strings) |
| Shared mode recalculation | 5 | "{account}: stop e alvo recalculados a partir do preço médio {price}" | average price, stop and target prices, distances and unit used (S6 rounding fields) |
| `modify` / `cancel` from TradingView | 6 | "Instrução {modify\|cancel} não é suportada na V1" (status *Rejeitada*) | contract: ingress 422 `ingress.action_not_allowed_for_source`, audited `ingress.rejected` with outcome `rejected` (S10 label *Rejeitada*, danger); reason shown as text; master replication is S20 |
| `close` on an out-of-sync account believed flat (Q75) | 2 | "{account}: fechamento enviado à plataforma; resultado: {fechado\|já estava zerada}" | flatten sent (Q75-1, decided 2026-10-24: it also cancels **every working order on the contract, manual ones included**, and the notification says so); result `flattened`, or `already_flat` audited as `routing.out_of_sync_flatten` (**TODO**: the code is in the 5.2.0 draft only, not in 5.1.1) |
| Reduce with nothing to reduce (Q76) | 4b | "{account}: nada para reduzir em {instrument}" (status *Ignorada*) | `routing.nothing_to_reduce`; the **notification** (S18 *warning*, not in the reopen list) and its use for an out-of-sync believed-flat account are **TODO** until Q76 is in the contract (5.2.0 draft) |
| **`reverse` on a barred account or with no room** (Q74 b) | 3 | "{account}: inversão só fechou a posição ({closed_qty} {long\|short}); a nova entrada não foi aberta: {reason}" | gate code (`routing.out_of_sync`, `risk_blocked`, `limits_pending`, `no_bracket_profile`, `max_position_reached`, or `routing.expired`), outcome *Parcial*, `reverse_closed_qty`; the open leg's quantity executed is 0. Notification `routing.reverse_close_only` (S7 §5.4) |
| `reverse` when already on the reverse side | 3 | "{account}: já está {comprada\|vendida}; nada foi alterado" | `routing.position_conflict`, never a flatten (status *Recusada*) |

All prices and quantities are strings formatted per F§6; the browser does no arithmetic. Status is text plus icon (F§1). A gate or position-conflict refusal uses *Recusada* (S10 `refused`); an ingress rejection (AC-6) uses *Rejeitada* (`rejected`); neither is an error style.

## S8-03 Rejection notice (AC-4)

"The user is notified" is fulfilled through the S18 notification center, not a new surface: the existing S18 notification `routing.position_conflict` (*warning*, registry grouping `account_trading_day`, params `DetailsRoutingDecision`), with the S8-02 text and link `Ver na atividade` to the S10 row. Severity and grouping follow the registry; asking for one notification per instruction across accounts would be a **registry change request to the Architect**, not drawn here. It is an **ordinary warning**, never critical (PO: critical is only Q27 orphan-leg flatten and U19 failed flatten, S18 AC-5); grouping stays as decided (S18 AC-6: same code + account + trading day).

---

## 4. Responsive behavior

The card is one column at every width; option cards stack below `md` and sit side by side from `md`. No table is added. Minimum usable width 320 px with no horizontal scroll (reflow 1.4.10).

## 5. Accessibility and i18n checklist

- Keys under `accounts.profile.mode.*` and `activity.s8.*` (pt-BR = en parity). No hard-coded text.
- Rejection and recalculation texts are built from fields the Core returns; the browser only inserts them.
- Live region: the save result is announced politely; the card does not announce on load.
- Keyboard: radio group by arrow keys, `Salvar` reachable after the group, focus returns to the group after an error.

## 6. Open questions

| Id | Question | Owner | Blocks |
|---|---|---|---|
| Q86 (Q-S8-1) | **Decided by the user (2026-10-24), option (b):** refused while the account has an open position or working orders. | — | closed (refusal code: contract request) |
| Q-S8-2 | **Answered by the PO:** ordinary warning, registry grouping. | PO | closed |
| Q-S8-3 | Contract: account field for the mode and the audit codes for S8-02 (`routing.position_conflict` and `routing.nothing_to_close` exist). | Architect | whole spec |
| Q-S8-4 | Current stop/target line in `shared` mode: **not drawn** (PO). | PO | none |

## 7. Implementation handoff

- **Frontend:** only S8-01 (one card) and the S10 text keys; reuse `RadioGroup`, `Card`, `Toast`, `AlertDialog` (dirty guard). Depends on the contract gap (Q-S8-3).
- **Backend / Plugin:** the behavior of AC-1…AC-6; the Core returns stable codes and the fields in S8-02.
- **QA:** cross-user access on the mode write; double-click sends once; mode change audited; keyboard-only and axe on the card; 320 px reflow; for each S8-02 row, the text matches the Core's fields.

## 8. Criteria covered

| Criterion | Where |
|---|---|
| AC-1 add/open | S8-02 |
| AC-2 close | S8-02 |
| AC-3 reverse, incl. close-only on a barred account (Q74 b) | S8-02 |
| AC-4 opposite entry rejected + user notified | S8-02, S8-03 |
| AC-4b FIFO partial exit | S8-01 (explanation), S8-02 |
| AC-5 shared mode | S8-01, S8-02 |
| AC-6 modify/cancel unsupported | S8-02 |
