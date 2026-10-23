# V1-S15 — Automatic enforcement: Web screens — Screen spec

> **Owner:** Product Designer (Traço @ Dev3) · **Status:** proposed, pending PO review · **Revision:** 1 · **Date:** 2026-10-23
> **Slice:** [V1-S15](../../product/release-1/slice-plan.md) AC-1 … AC-7 (slice-plan rev. 7); vision §9 "Automatic enforcement". Web part only: the plugin panel (what the user sees in NinjaTrader, AC-6) is the Plugin Engineer's; this spec lists the data it shows (§9).
> **Foundations and components:** [foundations.md](../foundations.md) (F§7.1 risk states, F§7.2 account states, F§6.4 time), [components.md](../components.md) (C§5 `RiskBanner`, C§6–C§8 notifications). **Related:** [S13](v1-s13-risk.md) (Risk tab, meters, rules; this spec adds the enforcement card and the blocked state), [S10](v1-s10-audit-log.md), [S18](v1-s18-notifications.md), [S5](v1-s5-automation-opt-in.md) §12 (step-up mechanics).
> **Contract:** no HTTP contract for enforcement yet. ADR-0004 carries the versioned `limits` and the enforcement `execution_report`s (`origin: local_enforcement`) and `risk_event`. §9 lists what the screens need. Operation names are **design proposals**; the contract wins.
> **Surfaces:** Web (Frontend) and plugin (Plugin Engineer). The Core decides the state; the plugin enforces. QA owns the checks.
> **Simulation only (X-7):** all verification uses Sim/Playback accounts.

---

## 1. What this spec covers

| # | Screen / element | Where |
|---|---|---|
| S15-01 | "Aplicação automática" card: switch, status, explanations | Risk tab (`/accounts/[id]/risk`, S13) |
| S15-02 | Turn on / turn off dialogs | over S15-01 |
| S15-03 | Blocked-until-reset state: banner, detail, event feed | Risk tab, account detail, account list, shell |
| S15-04 | Playback clock variant | trading-day strip (S13 §3) |
| S15-05 | Audit events of enforcement | rendered in S10 |

---

## 2. Concepts shown to the user (no new rules)

- Enforcement is **off by default** and turned on **per account** by the user (AC-1).
- With enforcement on, the plugin retrieves the account's limits from the Core and monitors live P&L; on reaching a limit it **flattens positions, cancels working orders, and blocks trading on that account until the reset** (AC-2).
- **No early unlock:** neither the Web nor the plugin offers a way to unblock before the reset (AC-3). No control, shortcut, support link or hidden menu in this spec removes a block.
- Enforcement **works while the Core is unreachable**, using the last limits the plugin received (AC-4).
- **The block is reactive** (NinjaTrader offers no account lock): while an account is blocked, any order the user places manually on it is cancelled and any resulting position is flattened immediately after submission; every such action is audited and shown in the plugin (AC-6).
- In **Playback** the trading-day reset and daily P&L use the connection's simulated clock (AC-7).
- Every flatten, cancel and block is audited with the limit and the value that triggered it (AC-5, X-5).
- Two independent barriers exist and the UI says so: the **Core gate** refuses new entries at a limit **even with enforcement off** (S14 AC-1; exits are never refused, P1), and the **plugin** closes and blocks when enforcement is on (second barrier, vision §5).

---

## 3. S15-01 "Aplicação automática" card (Risk tab)

**Placement:** a `Card` on the account Risk tab (S13 §3), under the trading-day strip and above the meters. The same state is summarized as a read-only line in the S13 edit form (§4.5 of S13) with a link here.
**Permissions:** own account only (X-2).

**Content**

1. Header: `Aplicação automática` / `Automatic enforcement` and a `Switch` (default **off**), accessible name "Aplicação automática em {account}".
2. **Status line** (`StatusBadge` + text), one of:

| Status | Badge / text pt-BR | Meaning (source of truth: Core) |
|---|---|---|
| Off | `Desligada` (neutral, `ShieldOff`) | "O All Win recusa novas entradas ao atingir um limite, mas não fecha posições nem bloqueia a conta." |
| On, plugin has current limits | `Ligada · ativa no NinjaTrader` (success, `ShieldCheck`) | "Limites versão {n} confirmados pelo plugin às {time}." |
| On, waiting for the plugin | `Ligada · aguardando o NinjaTrader` (warning, `Unplug`) | "O NinjaTrader está desconectado. Última versão de limites confirmada: {n} às {time}. O plugin continua aplicando essa versão." |
| On, plugin limits out of date | `Ligada · limites desatualizados no plugin` (warning, `CircleArrowUp`) | "O plugin ainda aplica a versão {m}; a mais recente é {n}." |
| On, no enforceable rule | `Ligada · sem limites` (warning) | "Defina limite de perda diária ou drawdown; sem eles não há o que aplicar." |
| Unavailable | switch disabled with reason, see below | |

3. **"O que acontece" list** (always visible, short, fixed text): the four facts of §2 in plain words — what is closed and blocked, that the block lasts until the reset ({time, zone}), that the block is reactive, that it works without the Core, and that there is no early unlock.
4. **Two barriers note** (info `Alert`, collapsible): "Duas barreiras: o All Win recusa novas entradas no limite mesmo com esta função desligada. Com ela ligada, o plugin também fecha posições e bloqueia a conta."

**Switch availability** (`aria-disabled`, focusable, reason inline and in `aria-describedby`; the reason is never only a tooltip):

| Case | Reason text | Note |
|---|---|---|
| Account not bound to a NinjaTrader account | "Vincule a conta do NinjaTrader para ligar a aplicação automática." + link | the plugin enforces, so a binding is required **[provisional, Q-S15-4]** |
| Manual-only firm (S16 AC-7) | "Esta firma é só de acompanhamento manual: o All Win não envia nem fecha ordens nesta conta." | **[Q-S15-4: PO to confirm]** |
| Account currently **blocked** (turn-off only) | "Conta bloqueada até {time}: só dá para desligar depois do reset." | consequence of AC-3; **[provisional, Q-S15-1]** |
| Data stale / error | switch disabled, `Sem dados recentes` | never toggle blind |

**States:** loading → skeleton of the card; error → `ErrorState` compact + retry; the switch applies only after the Core answers (no optimistic flip): while saving it shows spinner + "Salvando…" and `aria-busy`.

---

## 4. S15-02 Turn on / turn off

### 4.1 Turn on — `ResponsiveDialog` "Ligar a aplicação automática em {account}?"

Body (fixed text, pt-BR / en):
- "Ao atingir o limite de perda diária ou o drawdown, o plugin fecha as posições, cancela as ordens e bloqueia esta conta até o reset ({time} {zone})."
- "O bloqueio é reativo: o NinjaTrader não consegue travar a conta. Enquanto ela estiver bloqueada, toda ordem manual que você enviar será cancelada e a posição resultante será fechada logo depois do envio."
- "Funciona mesmo se o All Win estiver fora do ar, usando os últimos limites recebidos pelo plugin."
- "Não há desbloqueio antecipado, nem pela web nem pelo plugin."
- If the Core says the account is **already at or beyond a limit today** (`would_be_reached`): `Alert` (danger): "Esta conta já passou do limite hoje. Ao ligar, as posições serão fechadas e a conta ficará bloqueada agora, até o reset." **[Provisional, Q-S15-2]**
- Step-up, if the Core requires it (the Architect's wider list, Q35 open): the `StepUpDialog` mechanics of S5 §12 (challenge + original request once, cancel runs nothing). No layout change.

Actions: `Ligar aplicação automática` (primary), `Cancelar` (focused). Submit: spinner + "Salvando…", duplicate submits ignored (F§10). Success: toast "Aplicação automática ligada." and the status line updates; the status may first show `aguardando o NinjaTrader` until the plugin confirms. Failure: destructive `Alert` in the dialog with the translated code; state unchanged.

### 4.2 Turn off — `AlertDialog` (reduces protection) "Desligar a aplicação automática em {account}?"

- Body: "O plugin deixa de fechar posições e bloquear esta conta. O All Win continua recusando novas entradas ao atingir um limite. Posições abertas não são alteradas."
- If the account has a **flatten-and-block profit target** saved (S13 §4.2): "A meta de lucro diária passa a só avisar." **[Q-S13-8 / Q-S15-3: confirm the effect; text provisional]**
- Confirm button repeats the verb: `Desligar aplicação automática` (destructive variant), cancel focused (F§10).
- Not available while the account is blocked (§3): the dialog cannot be opened; the reason is shown on the switch.

---

## 5. S15-03 Blocked until the reset

The Core sends the block (`blocked_until`, reason, trigger). Everything below is display only.

**Where it appears**

| Place | UI |
|---|---|
| Account list, Estado cell | `StatusBadge` `Bloqueada até {time}` (`Lock`), in F§7.2 priority after out of sync and disconnected; color by reason: danger for loss or drawdown, **success for the daily profit target** (F§7.1); the reason text is always present |
| Account detail and Risk tab | `RiskBanner` (C§5) `reached` + enforcement on: "Perda diária: … · conta bloqueada até 17:00 (GMT-3)"; for the target: title "Bloqueada até {time}", body "motivo: meta de lucro diária atingida" |
| Shell | the one aggregate `RiskBanner` counts it (C§5). The state is a banner/badge, not a notification; the event creates one notification (S18 catalog `risk.blocked`) |

**Block card** (Risk tab, replaces the plain status line while blocked; `Card` with danger or success border by reason):
- Title: "Conta bloqueada até {date-time, zone}" and, as secondary text, the relative time left ("daqui a 3 h 12 min", with the absolute time in `<time>`; no ticking countdown).
- **Why:** limit name, limit value, value that triggered it, trigger time with seconds (all from the Core).
- **What the platform did:** positions flattened (count and contracts), working orders cancelled (count), time of each action, from the audit entries; a line "Aplicado pelo plugin com os limites da versão {n}" and, when the Core was unreachable, "Aplicado enquanto o All Win estava fora do ar, com os limites guardados no plugin (versão {n})" (AC-4).
- **What happens now (fixed text):** "Qualquer ordem manual enviada nesta conta será cancelada e a posição resultante será fechada logo depois. O All Win recusa novas entradas. Saídas continuam permitidas. **Não há desbloqueio antecipado.**" (AC-3, AC-6, S14, P1).
- **Events of the day:** list of the blocked account's `risk` entries since the block (reactive cancels, flattens, failures), each with time, text and `Ver no registro de atividade` (S10 filtered by account and `category=risk`). Empty: "Nenhuma ordem manual foi cancelada desde o bloqueio."
- **There is no button, link or shortcut that unblocks** (AC-3). The only exit is the reset; the card says when.
- A flatten or cancel that failed shows `Alert` (danger): "Não foi possível fechar tudo em {account}. A conta foi marcada fora de sincronia." + link `Ver conta` (S11 flag; waits for the user, never auto-resync).

**At the reset:** the state returns to normal live (SSE). The card disappears, the account badge clears, and a polite announcement says "{account} liberada." An info notification for the release is **[Q-S15-3: proposal, not in the S18 catalog yet]**.

**Aggregate:** if several accounts are blocked, the shell banner lists up to three and `+N` (C§5).

**States:** stale data → `Sem dados recentes` over the card (the block may already have ended or started; never present as current); error → `ErrorState`; session ended per F§10.

---

## 6. S15-04 Playback accounts

For an account whose kind is `playback` (data from the connection), the trading-day strip and the block card use the **connection's simulated clock** (AC-7): "Relógio de Playback: o reset e o P&L do dia seguem o relógio simulado ({simulated date-time})." Times shown for the reset and for `blocked_until` are in the simulated clock, labeled as such, never mixed with the computer's clock. A Sim reset or Playback rewind is a discontinuity (S11 AC-6): the Risk tab shows `Alert` (warning) "A simulação foi reiniciada; os valores do dia podem ter recomeçado. Isso não conta como resultado." (S11 spec owns the full flag).

---

## 7. Strings (examples, pt-BR / en; Frontend creates the keys)

| Where | pt-BR | en |
|---|---|---|
| Card title | Aplicação automática | Automatic enforcement |
| Off | Desligada | Off |
| On | Ligada · ativa no NinjaTrader | On · active in NinjaTrader |
| Waiting | Ligada · aguardando o NinjaTrader | On · waiting for NinjaTrader |
| Blocked | Bloqueada até {time} · motivo: {reason} | Blocked until {time} · reason: {reason} |
| No unlock | Não há desbloqueio antecipado. | There is no early unlock. |
| Reactive | O bloqueio é reativo: o NinjaTrader não consegue travar a conta. | The block is reactive: NinjaTrader can't lock the account. |
| Offline | Aplicado enquanto o All Win estava fora do ar | Applied while All Win was unreachable |
| Turn on | Ligar aplicação automática | Turn on enforcement |
| Turn off | Desligar aplicação automática | Turn off enforcement |

---

## 8. Audit events (render in S10; category `risk` unless noted)

| Event (code, proposed) | Entry text | Details |
|---|---|---|
| `risk.enforcement_changed` (`config`) | "Aplicação automática ligada / desligada na conta {account}" | previous and new value, user, limits version |
| `risk.flatten` | "{account}: posições fechadas — {limit} atingido" | limit, limit value, triggering value, positions and contracts, `origin` = local enforcement, limits version, `applied_offline` flag |
| `risk.orders_cancelled` | "{account}: {n} ordens canceladas" | order references, reason |
| `risk.blocked` | "{account} bloqueada até {time}" | reason, `blocked_until`, limit, value |
| `risk.reactive_block_action` | "Ordem manual cancelada em {account}: conta bloqueada" / "Posição resultante fechada" | order or position, time, delay to cancel |
| `risk.unblocked` *(Q-S15-3)* | "{account} liberada no reset" | reset time, clock (real or simulated) |
| `sync.*` | per S11 | when a flatten/cancel fails |

Events produced while the Core was unreachable arrive later and are shown with their true times and the "Aplicado offline" mark; they are never re-dated.

---

## 9. Data and actions this slice needs (for the Architect and the Plugin Engineer)

**Web reads**

| Need | Notes |
|---|---|
| `getAccountEnforcement(account_id)`: `enabled`, `status` (`off`, `active`, `waiting_plugin`, `limits_outdated`, `no_enforceable_rule`), `core_limits_version`, `plugin_limits_version`, `plugin_acked_at`, `can_enable` + `can_enable_reason`, `can_disable` + `can_disable_reason`, `would_be_reached_now` | The reasons are codes the UI translates |
| Block: `blocked` (bool), `blocked_since`, `blocked_until`, `clock` (`real` / `simulated`) + simulated now, `reason_code` (loss, drawdown, profit_target), `limit`, `limit_value`, `trigger_value`, `applied_offline`, `limits_version_used` | All decimals as strings; the UI computes nothing |
| Actions taken: counts and list of flatten/cancel/reactive entries since the block | from the audit (S10) |
| Account facts: binding, connection state, firm manual-only flag, account kind (sim/playback/live) | already in S5 |

**Web writes:** `setAccountEnforcement(account_id, enabled, row_version)` (idempotent; `x-step-up: conditional` if the Architect decides, Q35); errors `risk.enforcement_blocked_until_reset` (turn-off while blocked), `risk.enforcement_not_available` (+ reason), `common.version_conflict` (names proposed).

**Events (SSE):** enforcement status changes, block start and end, plugin limits ack. Mobile background tabs refetch on return (F§9).

**Plugin panel — data the plugin shows (the panel itself is the Plugin Engineer's; text in the user's language, S4 AC-3; terms from F§5):** enforcement on/off; the limits version applied and when; for a blocked account the reason, the limit and the value, `blocked until {time}` (real or simulated clock) and the statement that the block is reactive; each reactive cancel or flatten with its time (AC-6); a notice when it acts with cached limits while the Core is unreachable (AC-4). The plugin has no control that unblocks (AC-3).

---

## 10. Accessibility

- The switch is a real `Switch` with the status in its description; disabled-but-focusable with `aria-disabled` and the reason in `aria-describedby`.
- A change of the block state is announced once: a new block via `role="alert"` (it affects trading, like C§5 `reached`), the release politely; no ticking announcements. The time left is static text, not a live countdown.
- Dialogs: `AlertDialog` for turn-off with the cancel button focused; focus returns to the switch. Turn-on is a `Dialog` with cancel focused.
- Badges and cards carry text + icon; success/danger by reason is never color-only (the reason text is present).
- Contrast and touch targets per F§3 and F§9; 320 px reflow (the block card stacks).
- Reduced motion: no animated state changes.

## 11. Open questions

| ID | Question | Owner | Spec default (provisional) |
|---|---|---|---|
| Q-S15-1 | May enforcement be turned off while the account is blocked? (AC-3 says no early unlock) | PO / Architect | No: switch unavailable until the reset |
| Q-S15-2 | Turning enforcement on while a limit is already exceeded today: enforce at once? | PO | Yes, with a warning in the dialog (consistent with Q48 for tightening) |
| Q-S15-3 | Notification when the block ends; effect on a saved "flatten and block" profit target when enforcement is turned off (Q-S13-8) | PO | Info notification proposed; the profit target falls back to notify-only, stated in the dialog |
| Q-S15-4 | Can enforcement be on for an unbound account or a manual-only firm account? | PO | No, with the reason |
| Q-S15-5 | Can it be on with no enforceable rule? | PO | Allowed, status `sem limites`, with the explanation |
| Q-S15-6 | Is turning enforcement on/off a step-up action (Q35 wider list)? | User via PO | Not required today; mechanics ready |
| Q-S15-7 | Release notification and `risk.unblocked` audit entry | Architect | Proposed |

## 12. AC traceability

| AC | Where |
|---|---|
| AC-1 off by default, per account | §3 |
| AC-2 plugin retrieves limits, monitors P&L, flattens, cancels, blocks until reset | §2, §3 status, §5 |
| AC-3 no early unlock in Web or plugin | §2, §3 (turn-off unavailable while blocked), §5 (no unblock control), §9 plugin |
| AC-4 works while the Core is unreachable (cached limits) | §3 status, §5 "aplicado offline", §8 |
| AC-5 every flatten/cancel/block audited with limit and value | §8 |
| AC-6 reactive block explained to the user (Web and plugin data) | §4.1, §5, §9 |
| AC-7 Playback clock | §6 |
| X-2, X-5, X-6, X-7, X-8 | permissions, §8, strings, simulation only, time |
