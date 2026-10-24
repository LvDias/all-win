# V1-S11 — Out-of-sync detection and disconnects — Screen spec

> **Owner:** Product Designer (Aquarela, taking over D3) · **Status:** proposed, pending PO review · **Revision:** 1 · **Date:** 2026-10-23
> **Slice:** [V1-S11](../../product/release-1/slice-plan.md) AC-1 … AC-7 (Web part; the plugin side is in [plugin panels](v1-plugin-panels.md)). Touch points: S5 (account list/detail alerts, §0/§8), S7 (route status reason), S10 (audit), S12 (missed instructions), S18 (notifications), S15 (plugin enforcement).
> **Foundations:** [foundations.md](../foundations.md) (F§7.2 status priority, F§5 terms) · **Components:** [components.md](../components.md).
> **Contract:** [HTTP contract 5.1.1](../../../contracts/http/openapi.yaml) (commit 7872ca9). It has **no operation to clear the flag and no account-level sync state** in `contracts/http/modules/*.yaml`. What exists and should be reused: `DetailsRoutingDecision.sync_reasons[]` (`sync.*` codes, audit), `Connection.disconnect_reason` (`user`, `network`, `outdated`, `unpaired`), the gate reason and notification `routing.out_of_sync`. Items marked **[Contract gap]** are requests to the Architect through the Maestro; names are proposals.
> **Surfaces:** Web (Frontend). Detection and reconnect check are the Core's and the plugin's. The plugin shows state and never clears the flag (Q-P3, accepted by the PO).

## 0. What S11 is on screen

No new route. S11 adds, on screens that already exist:

1. **The out-of-sync state and its reason** on the account list and detail (S5), and as a route reason (S7 §5.3, already drawn).
2. **One action: `Limpar alerta`** (AC-3), with a confirmation.
3. **The reconnect-check outcomes** (AC-5) as an alert on the account, notifications (S18) and audit rows (S10).
4. **"Integridade do bracket desconhecida"** (AC-5 iv) while an account cannot be observed.

The user is always told what happened and what All Win did or did not do. All Win never resyncs by itself (AC-2); every automatic action is explained.

---

## S11-01 Out-of-sync alert on the account detail — `/[locale]/accounts/[id]`

**Purpose:** show that new instructions are blocked, why, and let the user clear the flag after resolving the problem.
**Entry point:** top of the account detail (S5 §8 "Alertas", most severe first, F§7.2), `StatusBadge` `Fora de sincronia` on the list and detail header. **Permissions:** owner only; cross-user access answers 404.

**Layout:** `Alert` (danger, `OctagonX`) in the alerts area:

- Title "Conta fora de sincronia" / "Account out of sync".
- Body: "Novas entradas não são abertas. Fechamentos continuam, inclusive o de uma inversão, e o fechamento é sempre enviado à plataforma." / "New entries are not opened. Closings continue, including the closing leg of a reverse, and a close is always sent to the platform." (Q74, Q75; decided 2026-10-24: a flatten on an out-of-sync account cancels every working order on the contract, manual ones included) *(AC-4; exits per S7 AC-8.)*
- **Reason** (from the Core, text + date-time): one of the table below; with details when present (what was missed, expected vs reported position).
- **What All Win did** (when applicable): "Cancelamos automaticamente {n} ordens órfãs: {list}." / "We automatically flattened the position created by the orphan leg ({qty})." Exactly what the Core reports; the browser invents nothing.
- Actions: `Limpar alerta` (primary, see S11-02) and `Ver atividade` (link to the S10 timeline filtered by the account and the reason's time window).

**Reasons (AC-1, 5, 6, 7)** — the Core sends a stable code; texts below are proposals **[Contract gap: reason code names belong to the Architect]**.

| Reason | AC | Text pt-BR / en | Extra details |
|---|---|---|---|
| Missed order | 1 | "Uma ordem que esta conta deveria ter recebido não chegou." / "An order this account should have received was missed." | instruction id, time |
| Rejected order | 1 | "Uma ordem foi rejeitada pela plataforma." | platform message as text, never HTML |
| Partial fill | 1 | "Uma ordem foi preenchida só em parte: {filled} de {quantity}." | strings |
| Disconnected with position or orders | 1, 2 | "O NinjaTrader desconectou com posição ou ordens abertas. Confira a plataforma e decida o que fazer." | position and working orders as last known, with time ("último dado às {time}") |
| Bracket incomplete after reconnect | 5 ii | "Posição aberta sem stop ou alvo, ou ordens em estado desconhecido. Nada foi recriado." | what the plugin reported |
| Unwanted opposite position flattened | 5 iii | "Os dois lados do bracket foram executados enquanto o NinjaTrader estava fora. A posição criada pela perna órfã foi fechada." | quantity, price (strings); **maximum-priority** (`danger`, critical notification, see S11-04) |
| Simulation reset / Playback rewind | 6 | "A simulação foi reiniciada ou o Playback voltou no tempo. Os valores deste dia não foram contados como execuções nem como resultado." | — |
| Entry command expired | S12 AC-2b | "Uma ordem de entrada expirou sem ser executada." | `execution.expired`; **does not** flag when it is the open leg of a reverse that never left the Core (5.1.1 exception, ADR-0004 §5) |
| Order state unknown after recovery | — | "Não sabemos o estado de uma ordem depois da reconexão." | `execution.state_unknown` (S10 *Indeterminada*) |
| Orphan cancellation failed | S9 AC-3 | "Não conseguimos cancelar uma ordem órfã." | which order, platform message as text |
| Flatten failed **after cancelling protections** and left a position open whose All Win protection was cancelled by that flatten (all legs or only some) | U19 (user decision 2026-10-23, residual risk accepted; Architect/Esquadro, ADR-0005 §N11: covers **every** flatten origin: Core command close, closing leg of a reverse incl. the Q75 flatten, orphan-leg flatten (Q27), and the local enforcement flatten) | "Posição aberta sem proteção; gerencie no NinjaTrader." | **top severity: critical** (`CriticalBanner`) + out of sync, like 5 iii. The plugin shows the same maximum alert |
| Flatten or cancel failed but the open position keeps its protection | S15 AC-6 (same event as U19 when protection was already cancelled; Architect/Esquadro) | "Não foi possível fechar tudo em {account}." | `danger` + out of sync. Cases: the cancel failed before any leg went out (`order.cancel_failed`); a position with no All Win protection from the start (a manual order blocked reactively); only entry orders were cancelled. **Never** the words "sem proteção" here |
| External order or fill | 7 | "Uma ordem ou execução fora do All Win foi detectada nesta conta." | only on master-replication destinations (S19, V1.1); TradingView-only destinations are audited only, no flag |
| Disconnected by the user in the plugin | — | "Você desconectou este computador no plugin com posição aberta." | `sync.disconnected_by_user` (S10, proposal already in plugin panels) |

More than one reason can be active; the alert lists the **most severe first** and the rest under `Mostrar outros motivos ({n})` (`Collapsible`). Clearing clears the flag as a whole.

**Fields and API mapping**

| Element | API |
|---|---|
| State, reasons, "what All Win did", timestamps | account detail and list **[Contract gap: sync state object with `reasons[]`]** |
| `Limpar alerta` | **[Contract gap: operation, e.g. `POST /accounts/{id}/sync/clear`]**; audited (AC-3) |
| Live updates | SSE account events already include out-of-sync flag changes (S5 §"Events (SSE)") |

**States:** loading → `Skeleton` of the alerts area; error → `ErrorState` with retry; flag cleared elsewhere (another tab) → the alert disappears with a polite announcement "Alerta limpo."; stream not fresh → "Sem dados recentes" (F§7.2), never a stale "em sincronia".

**Responsive:** the alert is full width; actions stack below `md`, `Limpar alerta` on top. **Accessibility:** `role="alert"` only when the flag appears while the page is open; on load it is a plain `Alert` (no repeated announcement). The reason and status are text plus icon, never color alone.

## S11-02 Clear the alert (AC-3)

Clearing the out-of-sync alert does **not** depend on unpairing, unlinking or disabling, and those actions are **not** refused while the account is out of sync. Q82 (a, 2026-10-24) is about an account **blocked until the reset** (S15 enforcement block): there those actions are refused until the reset; the text and the error code (`risk.unlink_blocked_until_reset`, name still a proposal) are in [S15 §3](v1-s15-enforcement.md). With Q83 (a) the way out of a dead connection is unpair or disable. Clearing with a position still open is allowed with the confirmation below (Q84 a).

`Limpar alerta` opens an `AlertDialog` "Limpar o alerta de {account}?":

- Body: "Confirme que você conferiu a posição e as ordens na plataforma." / "Confirm that you checked the position and orders on the platform." plus **one closing sentence chosen from the protection the Core reports** (Q98 b: only a position with no stop keeps the account blocked for automated entries):
  - no open position, or every position `full`: "Depois de limpar, esta conta volta a receber novas entradas." / "After clearing, this account receives new entries again."
  - any position `none`: "A posição sem stop continua bloqueando novas entradas automáticas até ser protegida ou zerada." / "The position without a stop keeps blocking new automated entries until it is protected or closed."
  - any position `partial` or `unknown` (and none `none`): **[Pending: the post-clear block rule for `partial` / `unknown` is not stated in the contract; request to Compasso (6.0.1) and decision with the PO.]** Until then the dialog says only "O bloqueio de novas entradas automáticas depende da proteção desta posição." / "Whether new automated entries stay blocked depends on this position's protection." and never promises that entries resume.
- **Not observable (AC-5 iv), Q83 (a), decided by the user 2026-10-24:** the flag is not cleared while the account is unobservable or its position is unknown. `Limpar alerta` is `aria-disabled` with the reason "A conta ainda não está observável. Espere o plugin reconectar e conferir a posição." / "The account is not observable yet. Wait for the plugin to reconnect and check the position." The dialog does not open in that state.
- **Open position in the last data (Q84 a, S11 AC-3b ii; Q98 b):** the dialog lists each position with instrument, side and quantity (strings from the Core) and its protection, from `AccountSync.positions[].protection` (6.0.0 draft, TODO until published): `full` "Protegida" / "Protected" · `partial` "Parcialmente protegida" / "Partially protected" · `none` "Sem proteção. Continua bloqueando novas entradas automáticas." / "Not protected. It keeps blocking new automated entries." (warning) · `unknown` "Proteção desconhecida" / "Protection unknown" (warning). **Q98 (b):** a stop placed manually on the platform counts as protection. "Covers" means a stop or stop-limit on the opposite side, same account and contract, with quantity ≥ |position| (stricter rule from the security review); the Core re-evaluates the protection on every snapshot, so the dialog and the card follow the latest one. The origin label comes from `positions[].protection_source` (6.0.0 draft, TODO until published): `all_win` no label · `external` "(externo)" / "(external)" · `mixed` "(All Win e externo)" / "(All Win and external)", e.g. "Protegida (externo)". The confirmation adds: "Você pode manter a posição." / "You can keep the position."
- **After a clear with protection `none` (Q98 b):** the alert disappears but the account card keeps a `warning` line "Bloqueada para novas entradas: posição sem stop." / "Blocked for new entries: position without a stop." with the instrument, and it disappears on the first snapshot in which the position is covered or flat. Audit and card use the reason from the Core. **[Pending: reason code for the block that remains after the clear, request to Compasso (6.0.1).]**
- **Step-up (Q95 yes, 2026-10-24):** clearing needs a fresh code: action `account_sync_clear`, operation `clearAccountSync` (HTTP 5.3.0, approved). Reactive flow as in S7: the first request goes without a code; on 403 `auth.step_up_required` the `StepUpDialog` opens and the request is resent once with `challenge_id` + `code` (F§8.2); the idempotency key is kept.
- A required `Checkbox` "Conferi a posição e as ordens na plataforma." gates the confirm button (a deliberate speed bump; the same pattern as S5 acknowledgement). Buttons `Cancelar` / `Limpar alerta` (destructive-neutral style, not danger).
- Duplicate-submit: button busy and disabled after the first click; the request carries an idempotency key.
- Success: toast "Alerta limpo. Os motivos ficam no registro de atividade." and the alert disappears; audit entry with who and when. Failure: inline `Alert` in the dialog with the translated code, dialog stays open; `409` (state changed) → "O estado da conta mudou. Recarregue para ver o motivo atual." + `Recarregar`.
- Step-up: **not** required (Q35 decided: the S3 AC-10 list does not include it). The plugin has no clear control (Q-P3).
- Focus: opens on the checkbox; returns to the alert area or to the account heading when the alert is gone.

**Never automatic:** the Web has no "Limpar tudo" and no auto-clear after a reconnect (AC-2).

## S11-03 Reconnect check and "bracket integrity unknown" (AC-5)

- While an account **cannot be observed** (broker connection lost with NinjaTrader open, AC-5 iv, or the check has not finished): a `Badge` warning `Integridade do bracket desconhecida` / `Bracket integrity unknown` (`CircleHelp`) on the account detail header and an `Alert` explaining: "Não conseguimos confirmar se o stop e o alvo desta conta estão ativos. Confira na plataforma." The badge disappears when the check reports.
- After the check, the result is shown as the **reason and "what All Win did"** of S11-01 (cases i–iii). Case (i) alone (orphan leg cancelled, position flat) still leaves the account out of sync until cleared (AC-5 last line, Q14), so the alert shows "Cancelamos {n} ordens órfãs" with the list.
- New commands are not accepted before the check completes: the route status shows the reason `connection_disconnected` / `Verificando a conta` **[Contract gap: reason for "check in progress"]**. While disconnected: **entries that cannot be sent expire per TTL (`routing.expired`) and are reported as missed; exits wait for the connection and run late, audited as late** (S12 AC-2/AC-2b, T1: exits never expire). The route status says "NinjaTrader desconectado".

## S11-04 Notifications (S18) and audit (S10)

Notifications are one pointer per event with a link to the account; the current state stays in the alert. Codes are **proposals [Contract gap]** except `routing.out_of_sync`, already in the S18 catalog.

| Event | AC | Severity | Text pt-BR | Link |
|---|---|---|---|---|
| Account flagged out of sync | 1, 6, 7 | danger | "{account} ficou fora de sincronia: {reason}" | account detail |
| Disconnect with position/orders | 1, 2 | danger | "{account}: desconectou com posição aberta. Nada será reenviado sozinho." | account detail |
| Orphan legs cancelled | 5 i | warning | "Ordens órfãs canceladas em {account}: {list}" | S10 |
| Bracket incomplete | 5 ii | danger | "{account}: posição sem stop/alvo ou ordens em estado desconhecido" | account detail |
| Opposite position flattened | 5 iii | **critical** (`CriticalBanner`, C§8; one of **two** critical cases, with U19 below: S18 AC-5 / Q51, Q27) | "{account}: fechamos {qty} criados pela perna órfã. Confira a conta." | account detail |
| `reverse` with an opposite position on an out-of-sync account | 4 | danger | "Inversão em {account}: só a posição foi fechada ({closed_qty} {long\|short})" (`routing.reverse_close_only`, one per occurrence, S7 §5.4) | S10 row |
| Entry command expired / state unknown / orphan cancellation failed | S12 AC-2b, S9 AC-3 | danger | "{account}: {reason}" | account detail |
| Flatten failed after cancelling protections, position left unprotected (U19, any flatten origin) | U19 | **critical** (`CriticalBanner`, C§8) | "{account}: posição aberta sem proteção. Gerencie no NinjaTrader." | account detail |
| Flatten or cancel failed, position keeps its protection | S15 AC-6 | danger | "{account}: não foi possível fechar tudo." | account detail |
| Entry skipped on an out-of-sync account | 4 | per the S18 reopen rule (`routing.out_of_sync`, danger, Q68 b) | "Entrada recusada em {account}: conta fora de sincronia" | S10 row |
| Flag cleared | 3 | none (audit only) | — | — |

Audit rows (S10, one per automatic action and per clear): the S10 outcome and wording rule applies: **entries are refused, exits are never refused** (P1); keep "entrada" in the text. Clear: "Alerta de {account} limpo por você" with the reasons that were active.

## S11-05 Account list and filters

`Estado` column already shows the top status plus `+N` (S5 account list table). S11 adds a filter value `Fora de sincronia` to the accounts list and a count in the page header when > 0 ("2 contas fora de sincronia", `aria-live="polite"` on change). Below `md` the badge sits top-right of the card. No other change.

---

## 5. Accessibility and i18n

Keys: `accounts.sync.*`, `activity.sync.*`, `notifications.sync.*` (pt-BR = en). Reason text is composed from fields the Core returns; platform messages are inserted as plain text. Keyboard: dialog traps focus, Esc cancels, checkbox first. 320 px reflow, no horizontal scroll. Terms follow F§5 ("fora de sincronia", "limpar alerta").

## 6. Open questions

| Id | Question | Owner | Blocks |
|---|---|---|---|
| Q83 (Q-S11-1) | **Decided (a), 2026-10-24:** no clear while the account is unobservable. | — | closed |
| Q84 (Q-S11-2) | **Decided (a), 2026-10-24:** clearing is allowed with a position open, with the same confirmation. | — | closed |
| Q82 | **Decided (a), 2026-10-24, for the account blocked until the reset** (S15 §3, not the out-of-sync state): unpair, unlink and disable are refused until the reset. Not applied to S11. | — | closed (S15 owns it) |
| Q-S11-6 | **Closed by the Architect (Esquadro):** U19 covers every flatten origin; S15 AC-6 is the same event when the flatten failed after cancelling protection of an open position (critical), and danger when the position keeps its protection. Criteria in §S11-01 / S11-04. | — | closed |
| Q-S11-3 | Contract: sync state object (reasons[], what the Core did), clear operation, reason codes, check-in-progress reason, notification codes. | Architect | whole spec |
| Q-S11-4 | Audit/notification entries for reconnect-check results (S11 AC-5 i–iv) and missed instructions while disconnected (S12 AC-2) are requested by the PO for the S10/S18 catalogs; this spec lists them, the catalogs must follow. | Aquarela (S10/S18) | S10, S18 |

## 7. Implementation handoff

**Frontend:** S11-01, S11-02, S11-03 on the account detail; the list filter (S11-05); reuse `Alert`, `AlertDialog`, `StatusBadge`, `Collapsible`, `CriticalBanner`. Depends on the contract gap. **Backend/Plugin:** detection (AC-1), reconnect check (AC-5), wait-for-user (AC-2), skip + audit (AC-4). **QA:** cross-user access on the clear operation; double-click sends once; no auto-clear after reconnect; each reason renders from Core data; critical presentation for AC-5 iii; axe and keyboard pass on the dialog; stale-stream state never shows "em sincronia".

## 8. Criteria covered

| Criterion | Where |
|---|---|
| AC-1 flag + reason visible in Web and plugin | S11-01, S11-05; plugin panels |
| AC-2 alert and wait for the user | S11-01, S11-04; "never automatic" in S11-02 |
| AC-3 acknowledge and clear, audited | S11-02, S11-04 |
| AC-4 no new instructions until cleared, each skip audited | S11-01 body, S11-04 |
| AC-5 i–iv reconnect check | S11-01, S11-03, S11-04 |
| AC-6 Sim reset / Playback rewind | S11-01 reasons |
| AC-7 external order flags replication destinations | S11-01 reasons |
