# V1-S10 — Audit log and activity view — Screen spec

> **Owner:** Product Designer (Nanquim) · **Status:** proposed, PO review (Rumo) applied · **Revision:** 3 · **Date:** 2026-10-24 (rev. 3: audit side of Q74/Q75/Q76 per the 5.2.0 draft, Q75-1/Q75-2 decided, late-exit gap noted)
> **Slice:** [V1-S10](../../product/release-1/slice-plan.md) AC-1 … AC-4 (slice-plan rev. 7); cross-cutting X-2, X-4, X-5, X-6, X-8. Entry points from S3 (AC-16 events), S5, S6, S7, S11, S14, S15, S18.
> **Foundations:** [foundations.md](../foundations.md) (F§6.4 time, F§7 status, F§10 feedback) · **Related:** [S5](v1-s5-automation-opt-in.md) §13, [S6](v1-s6-stop-target-profiles.md) §8, [S7](v1-s7-routing.md) §9 (events they emit).
> **Contract:** [HTTP contract 5.4.0 (draft)](../../../contracts/http/openapi.yaml) (draft by Compasso, not yet committed; header to the commit once it is): `contracts/http/modules/audit.yaml` and the notes [s10-audit](../../architecture/contracts/s10-audit.md). Operations `listActivity`, `getActivityFilterOptions`, `getActivityEntry`, `getInstructionTimeline`; storage `audit.audit_entry` ([data model §6.4](../../architecture/data/wave1-data-model.md)). **Where this spec and the contract differ, the contract wins.**
> **Surfaces:** Web (Frontend). Writing entries, redaction and retention are the Core's (Backend). QA owns the checks.

---

## 1. What this spec covers

| # | Screen | Route |
|---|---|---|
| S10-01 | Activity list with filters | `/[locale]/activity` (sidebar `Atividade` / `Activity`, right after `Roteamento`) |
| S10-02 | Entry detail (side panel) | over S10-01 |
| S10-03 | Instruction timeline: one instruction from receipt to every account's result (AC-2) | `/[locale]/activity/instructions/[instruction_id]` |
| S10-04 | Entry text catalog (code → text pt-BR / en) | §7 |

---

## 2. Principles for this slice

- **Read-only (AC-3).** No edit, delete, hide or clear control exists anywhere. A line under the page title says it: "Os registros não podem ser editados nem excluídos." / "Records can't be edited or deleted."
- **Every automatic action has its reason (X-5).** An entry always shows the *what*, the *why* (translated code) and the *who/where* (source, account).
- **No secrets (AC-4, X-4).** A source is shown by **name** and, when needed, its `key_id` (12 characters, the only part of a token allowed anywhere); never the URL, token or any code. The detail never prints the raw `details` unless the Core marked it safe (§4 technical data).
- **Text comes from codes.** The Core sends `code` + `details`; the web renders the text from its own pt-BR/en catalog (§7). A code missing from the catalog shows "Evento {code}" with the technical data, never a blank (S4 AC-4: missing translations are detectable).
- **Exact values.** Prices, quantities and P&L are strings from the Core, formatted per F§6; never recomputed.
- **Time (X-8).** Date-time **with seconds and the time zone**, from the user's display zone: `23/10/2026 14:31:05 (GMT-3)`. The absolute time is always shown; relative time is not used in this log.
- **Isolation (X-2).** Own entries only. An instruction id of another user gives the S5 `not found` state.

---

## 3. S10-01 Activity list — `/[locale]/activity`

**Purpose:** list and filter everything the platform did and decided for the user (AC-1, AC-2).
**Permissions:** any signed-in user, own entries.

**Layout:** `PageHeader` "Atividade" + read-only line + retention line "Os registros ficam guardados por {n} meses." (value from the Core; 24 in V1, configurable, Q13) · filter bar · list · "N novas entradas" banner.

### 3.1 Filters (AC-2)

| Filter | Component | Values |
|---|---|---|
| Período / Period | chips `Hoje`, `24 h`, `7 dias`, `30 dias`, `Personalizado` (two date-time pickers; the zone is shown) | default `24 h`. `Hoje` = calendar day in the display zone |
| Conta / Account | multi-select `Combobox` of the user's accounts, **including deleted ones** (shown with `(excluída)`, from the name snapshot) | |
| Fonte / Source | multi-select, includes revoked sources | by name |
| Tipo de sinal / Signal type | `Gatilhos`, `Trade Plan`, `Gatilhos NQ`, `Master` | as in S7 |
| Resultado / Outcome | multi-select: `Recebida`, `Roteada`, `Executada`, `Parcial`, `Ignorada`, `Recusada`, `Rejeitada`, `Expirada`, `Indeterminada` (`info` entries have no outcome filter) | final list from the Architect, see §3.3 |
| Categoria / Category | `Entrada`, `Roteamento`, `Execução`, `Risco`, `Conexão`, `Segurança`, `Configuração` | the `category` field |
| Busca / Search | `Input` for an instruction id or `message_id` (exact) | not free-text search |

- State lives in the URL query (shareable inside the session, deep links from other screens): `?period=…&account=…&source=…&signal=…&outcome=…&category=…&code=…&q=…`. Deep links may carry `code=` (for example `accounts.opt_in_chosen` from S5): it appears as a removable chip "Evento: {text}".
- Filters apply as they change (debounced); on `< md` they open in a `Sheet` with `Aplicar` and `Limpar`. The active count shows on the trigger. `Limpar filtros` is always reachable.

### 3.2 List

Table (≥ `md`) or cards (< `md`, F§9), newest first, **page size 50** with `Carregar mais` (cursor; no page numbers). Columns:

| Column | Content |
|---|---|
| Hora / Time | `23/10/2026 14:31:05` + zone in the header (`Hora (GMT-3)`); `<time datetime>` |
| Resultado | outcome badge (§3.3) |
| Evento | the rendered text (§7), one line, truncated with the full text in the detail; secondary line: reason text when the code is a skip/refusal |
| Conta | account name (snapshot) or `—` |
| Fonte / Sinal | source name · signal type chip or `—` |
| Instrução | short id in mono + link `Ver instrução` when `instruction_id` exists |

Row click or Enter opens S10-02. Rows with an `instruction_id` that belong to the same instruction are **not** merged here; grouping is the timeline (S10-03).

**Live updates (SSE):** new matching entries do not insert themselves (the list would jump under the cursor). A banner at the top, `role="status"`: "{k} novas entradas · Mostrar" inserts them. When the stream is not fresh: "Sem dados recentes" (F§7.2); the list is never presented as current.

**States**

| State | UI |
|---|---|
| Loading | Skeleton of 8 rows |
| Empty, never any activity | `EmptyState` `ScrollText`: "Ainda não há atividade." / "Quando um sinal chegar ou você mudar uma configuração, o registro aparece aqui." + `Ver fontes de sinal` |
| Empty after filters | "Nenhuma entrada com esses filtros." + `Limpar filtros` |
| Error | `ErrorState` with retry and code; the filters stay |
| Loading more | button shows spinner + "Carregando…"; a failure keeps the loaded rows and shows a retry |
| Period older than retention | info line "O registro só guarda os últimos {n} meses." if the range start is before the horizon |

### 3.3 Outcome badges (text + icon + color, F§7)

Outcomes are **fields from the Core** (`outcome`), not guessed from the code.

| Outcome | Label pt-BR / en | Token, icon | Used for |
|---|---|---|---|
| `received` | Recebida / Received | info, `Inbox` | `ingress.received` |
| `routed` | Roteada / Routed | neutral, `Route` | routing decision to an account |
| `executed` | Executada / Executed | success, `CircleCheck` | fills |
| `partial` | Parcial / Partial | warning, `TriangleAlert` | quantity capped, partial fill |
| `skipped` | Ignorada / Skipped | neutral, `SkipForward` | duplicate, not selected signal type, not accepted, inactive |
| `refused` | Recusada / Refused | warning, `Ban` | risk or out-of-sync gate, position conflict |
| `rejected` | Rejeitada / Rejected | danger, `OctagonX` | ingress rejection, platform rejection |
| `expired` | Expirada / Expired | danger, `Clock` | `routing.expired` |
| `unknown` | Indeterminada / Unknown | warning, `CircleHelp` | order state unknown after recovery (`execution.state_unknown`) |
| `info` | — (no badge) | — | configuration, connection, security events and neutral steps |

The ten values are the Architect's final list (Compasso, 2026-10-23): `received`, `routed`, `executed`, `partial`, `skipped`, `refused`, `rejected`, `expired`, `unknown`, `info`.

**How the Core sets `outcome` (contract s10-audit §2; the client never derives it from the code):** routing decisions `routed` → `routed`, `capped` → `partial`, and `skipped` / `refused` / `rejected` / `expired` keep their names; `ingress.received` → `received`, `ingress.duplicate_ignored` → `skipped`, `ingress.rejected` and `ingress.rate_limited` → `rejected`; execution reports `filled` → `executed`, `partially_filled` → `partial`, `rejected` → `rejected`, `expired` → `expired`, `unknown` (`execution.state_unknown`, which also flags the account out of sync) → `unknown`, and `received` / `working` / `cancelled` (and, by default, `command_sent`, `bracket_calculated`, `fill_corrected`) → `info`; `duplicate` reports are not audited. A step with `info` shows its icon and text without a badge.

---

## 4. S10-02 Entry detail (side panel)

`Sheet` from the right (≥ `md`) or full-screen `Drawer` (< `md`), title = the rendered text, focus moves to the title and returns to the row on close. Content:

1. **Resumo:** time with zone and seconds, outcome badge, category, account, source (name), signal type, `Ver instrução` link when applicable.
2. **Motivo / Why:** the translated reason for skips, refusals, rejections and automatic actions, plus one explanatory sentence from the catalog (§7).
3. **Detalhes:** a definition list of the fields the catalog says to show for that code (for example previous and new state, rule version, source URL and access date of a displayed rule, quantity requested vs executed, price, latency). Money and prices are strings formatted per F§6; source URLs open in a new tab with `rel="noopener noreferrer"`.
4. **Dados técnicos** (`Collapsible`, closed): entry id, `instruction_id`, `command_id`, `message_id`, `source_key_id`, code, recorded-at time. The fields rendered by the catalog (§7) are **always** shown in section 3; the raw JSON `details` appears here **only when the entry has `safe_to_display = true`**. Mono font, `Copiar` button for the entry id (support).

No actions other than `Fechar`, `Copiar id` and links. States: loading skeleton; error `ErrorState`; entry not found (`common.not_found`).

---

## 5. S10-03 Instruction timeline — `/[locale]/activity/instructions/[instruction_id]` (AC-2)

**Purpose:** follow one instruction from receipt to every account's result.
**Entry:** `Ver instrução` from any row, S18 notification links, the search box.

**Layout**

```
PageHeader: Instrução {short id}                         [← Voltar à atividade]
Resumo: Recebida 14:31:05 (GMT-3) · Fonte: Estratégia Quântico · Sinal: Gatilhos · buy 1 MNQ · message_id …
┌ Recebimento ─────────────────────────────────────────────────────────────┐
│ 14:31:05 Recebida · 14:31:05 Validada                                    │
├ Conta: Apex 50K ─────────────────────────────── Executada ──────────────┤
│ 14:31:05 Roteada · 14:31:05 Comando enviado · 14:31:05 Preenchida 1 @ … │
├ Conta: Lucid 100K ───────────────────────────── Ignorada ───────────────┤
│ 14:31:05 Ignorada: a conta não aceita fontes automatizadas               │
└──────────────────────────────────────────────────────────────────────────┘
```

- Header summary: receipt time (with zone, seconds), source name (and `key_id` in mono), signal type, `action`, `instrument`, `quantity` (instruction), `message_id`, status of the instruction as a whole. It comes from the Core; no field is secret.
- **Blocks:** `Recebimento` first; then **one card per account**, ordered by name (not by speed), each with an overall outcome badge (the account's final result) and an ordered list (`ol`) of steps (time with seconds, outcome icon, text). Each step opens S10-02. Steps are ordered by `occurred_at`, ties by the Core's sequence.
- Per-account facts that explain the result: quantity requested → multiplier → cap → executed (strings from the Core), reason code text, stop/target calculation with the **tick rounding adjustment** (S6 AC-4), fills (price, quantity, time), latency per follower when present (S19 AC-4).
- A skip on one account never hides the others: all accounts appear (AC-5 of S7 makes them independent).
- Cards are collapsible (`Collapsible`), **open by default for ≤ 6 accounts**, collapsed above that with the outcome badge visible.
- Duplicate sends (`ingress.duplicate_ignored`) appear in the **original** instruction's timeline as an extra step in `Recebimento` ("Duplicata ignorada às …"), since they carry the same `message_id`.

**States**

| State | UI |
|---|---|
| Loading | Skeleton: summary + 3 cards |
| Instruction still in progress | live badge `Em andamento` / `In progress`; steps are added through SSE with a polite announcement of the account's **final** outcome only |
| Not found / not yours / purged by retention | `ErrorState` "Instrução não encontrada. Ela pode ter passado do período de retenção." (same text for all three; no distinction) |
| Error | `ErrorState` with retry and code |

---

## 6. Entry points and links from other specs

| From | Link | Filter |
|---|---|---|
| S5 account Automação card `Ver no registro de auditoria` | `/activity?account={id}&code=accounts.opt_in_chosen` (and `.opt_in_suspended`) | account + event |
| S5 account detail | `Atividade desta conta` | `?account={id}` |
| S7 route `Situação` reason | `/activity?account={id}&source={id}&category=routing` | |
| S18 notification | `/activity/instructions/{id}` or a filtered list | |
| S3 security events (AC-16) | `?category=security` | |

---

## 7. S10-04 Entry text catalog (code → text)

Texts use `{placeholders}` filled from `details`. Codes marked *(proposal)* are not yet fixed in a contract; codes from ADR-0003/ADR-0004/S1 notes are listed as they are. Entries not listed use the fallback "Evento {code}". The Frontend keeps this table in the translation files (S4); QA checks that every contract code has a pt-BR and en text.

| Category | Code | pt-BR | en | Detail fields |
|---|---|---|---|---|
| ingress | `ingress.received` | Sinal recebido da fonte {source} | Signal received from source {source} | signal type, action, instrument, quantity, `message_id`, `sent_at`, receipt time |
| ingress | `ingress.duplicate_ignored` | Duplicata ignorada: esta instrução já foi recebida | Duplicate ignored: this instruction was already received | original instruction link |
| ingress | `ingress.rejected` | Sinal rejeitado: {reason} | Signal rejected: {reason} | error code, JSON pointer, size (never the body) |
| ingress | `ingress.rate_limited` *(S1)* | Sinal recusado por excesso de envios | Signal refused: too many requests | wait seconds |
| routing | `routing.automation_not_accepted` | Instrução ignorada para {account}: a conta não aceita fontes automatizadas / escolha de automação pendente | Instruction skipped for {account}: the account doesn't accept automated sources / automation choice pending | choice state, rule version |
| routing | `routing.copy_not_accepted` | Replicação ignorada para {account}: a conta não aceita replicação / escolha pendente | Replication skipped for {account}: … | same |
| routing | `routing.signal_type_not_selected` | Ignorada para {account}: tipo de sinal {type} não selecionado nesta rota | Skipped for {account}: signal type {type} isn't selected on this route | route |
| routing | `routing.account_inactive` | Ignorada para {account}: conta {pausada / desativada} (também bloqueia saídas) | Skipped for {account}: account {paused / disabled} (exits are blocked too) | state |
| routing | `routing.out_of_sync` | Entrada recusada para {account}: conta fora de sincronia (saídas continuam) | Entry refused for {account}: account out of sync (exits continue) | sync reason |
| routing | `routing.risk_blocked` | Entrada recusada para {account}: limite de risco atingido (saídas continuam) | Entry refused for {account}: risk limit reached (exits continue) | limit, value |
| routing | `routing.expired` | Comando expirado para {account}: não foi executado | Command expired for {account}: not executed | TTL, base time |
| routing | `routing.quantity_capped` | Quantidade cortada em {account}: {executed} de {requested} | Quantity capped on {account}: {executed} of {requested} | requested, multiplier, maximum position, skipped excess |
| routing | `routing.routed` *(new)* | Roteada para {account}: {quantity} (instrução × {multiplier}) | Routed to {account}: {quantity} (instruction × {multiplier}) | requested quantity, multiplier, maximum position, signal type |
| routing | `routing.no_bracket_profile` *(new; Q43 decided)* | Entrada ignorada para {account}: sem perfil de stop/alvo (saídas continuam) | Entry skipped for {account}: no stop/target profile (exits continue) | account size |
| routing | `routing.max_position_reached` | Ignorada para {account}: posição máxima atingida | Skipped for {account}: maximum position reached | maximum |
| routing | `routing.position_conflict` | Recusada para {account}: a entrada é oposta à posição atual | Refused for {account}: entry opposes the current position | position, action |
| routing | `routing.nothing_to_close` | Nada a fechar em {account}: a conta está sem posição | Nothing to close on {account}: the account is flat | Occurs **only on an in-sync flat account** (Q75). On an out-of-sync account believed flat, a close is sent as a flatten (next rows) |
| routing | `routing.nothing_to_reduce` | Redução não enviada para {account}: nada a reduzir | Reduce not sent to {account}: nothing to reduce | Audited on an in-sync flat account **and** on an out-of-sync account believed flat (Q76 (a), S7 AC-8b): the reduce is never sent and never becomes a flatten. The user is **notified only when out of sync** (S18; ADR-0003 r4); on an in-sync flat account it is audit only |
| routing | *reverse on a barred account (Q74)* — gate code of the entry (`routing.out_of_sync`, `routing.risk_blocked`, `routing.limits_pending`, `routing.no_bracket_profile`) | Reversão em {account}: só a saída foi feita; a entrada foi ignorada ({reason}) | Reverse on {account}: only the exit was done; the entry was skipped ({reason}) | Audit entry carries the **gate's own code**, outcome `partial`, `executed_qty` 0 for the open and `reverse_closed_qty` (the closing leg: flatten and cancel protective orders; Q74 (b), S7 AC-8). On an out-of-sync account the closed quantity is unknown at the decision: `expected_flat = true`, no `reverse_closed_qty`, and the result arrives in the execution entries. The user notification `routing.reverse_close_only` is an **S18 notification code**, not an audit code |
| routing | *close on an out-of-sync account believed flat (Q75)* — decision `routing.routed` with `expected_flat = true` | Fechamento em {account} enviado como zeragem: a conta está fora de sincronia | Close on {account} sent as a flatten: the account is out of sync | The result comes in the execution entries: `execution.cancelled` plus `command.already_flat`, or fills (Q75 (b), S7 AC-8b). Audited. Every such flatten that ends in a terminal success (`filled` or `already_flat`) is notified **once** with `routing.out_of_sync_flatten` (S18; S7 AC-8b); `already_flat` raises no additional alert beyond that notification. The sentence "ordens pendentes no contrato, inclusive manuais, foram canceladas" belongs to the **notification** `routing.out_of_sync_flatten` (S18) and is decided by Q75-1 [Decision 2026-10-24]: a flatten on an out-of-sync account that is really flat cancels every working order on the contract, manual ones included, and is notified. Q75-2 [Decision 2026-10-24]: a `reverse X` on a believed-flat account that really holds X closes X and opens nothing (`closed_side` = that side) |
| routing | `routing.no_correlated_order` | Ignorada para {account}: nenhuma ordem correspondente | Skipped for {account}: no matching order | |
| execution | `execution.command_sent` | Comando enviado para {account}: {action} {quantity} | Command sent to {account}: {action} {quantity} | `command_id`, expiry time |
| execution | `execution.bracket_calculated` | Stop e alvo calculados em {account} | Stop and target calculated on {account} | unit, requested distance, whole ticks used, **rounding adjustment**, resulting prices (S6 AC-4) |
| execution | `execution.order_received` | {account}: ordem recebida pela plataforma | {account}: order received by the platform | order role (entry / stop / target / exit), time |
| execution | `execution.order_working` | {account}: ordem em andamento | {account}: order working | role, type, price |
| execution | `execution.partially_filled` | {account}: preenchimento parcial — {filled} de {quantity} a {price} | {account}: partially filled — {filled} of {quantity} at {price} | fill price, quantity, time, remaining |
| execution | `execution.filled` | {account}: ordem preenchida — {quantity} a {price} | {account}: order filled — {quantity} at {price} | fill price (string), quantity, time, latency |
| execution | `execution.cancelled` | {account}: ordem cancelada | {account}: order cancelled | role, reason (for example orphan protection, enforcement) |
| execution | `execution.rejected` | {account}: ordem rejeitada pela plataforma — {reason} | {account}: order rejected by the platform — {reason} | platform reason, role |
| execution | `execution.expired` | {account}: comando expirado antes de chegar — não foi executado | {account}: command expired before delivery — not executed | TTL, expiry base |
| execution | `execution.state_unknown` | {account}: estado da ordem indeterminado após a reconexão | {account}: order state unknown after reconnection | role, last known state; the account stays out of sync until you clear it |
| execution | `execution.fill_corrected` | {account}: preenchimento corrigido pela plataforma — de {old} para {new} | {account}: fill corrected by the platform — {old} to {new} | previous and new price/quantity, correction type (amend / remove) |
| risk | `risk.warning` / `risk.limit_reached` *(S13/S14)* | {account}: alerta de 80 % de {limit} / limite {limit} atingido | … | limit, value, percent |
| risk | `risk.flatten` / `risk.blocked` *(S15)* | {account}: posições fechadas e conta bloqueada até {time} | … | limit, value, reset time |
| risk | `risk.unblocked` *(S15, PO)* | {account} liberada no reset | {account} released at the reset | reset time, clock (real or simulated) |
| sync | `sync.disconnected_by_user` **[Proposal: not yet in the contract; added with the 1.3 / S11 delta]** | {account}: plugin desconectado por você{ com posição aberta; o All Win não está acompanhando esta conta} | {account}: plugin disconnected by you{ with an open position; All Win is not tracking this account} | had open position / command in flight (yes/no), time; audited in both cases |
| connection | `connection.status_changed` | Conexão {name}: {status} | Connection {name}: {status} | previous status |
| connection | `sync.*` (`sync.bracket_incomplete`, `sync.disconnected_in_trade`, …) | per S11 spec | | |
| security | `security.signed_in` | Entrada na conta ({channel}){; novo dispositivo} | Signed in ({channel}){; new device} | channel, `new_device`, device label (coarse browser/OS family, never a full user agent) |
| security | `security.sign_in_failed` | Tentativa de entrada falhou ({channel}) | Sign-in attempt failed ({channel}) | channel only; the reason is never detailed |
| security | `security.code_rate_limited` | Envio de código limitado ({purpose}) | Code sending rate-limited ({purpose}) | purpose (`login`, `signup`, `step_up`, `channel_change`) |
| security | `security.code_delivery_failed` | Falha ao enviar o código por {channel} ({purpose}) | Failed to deliver the code by {channel} ({purpose}) | channel, purpose |
| security | `security.new_device_notice_sent` | Aviso de novo dispositivo enviado | New-device notice sent | device label |
| security | `security.step_up_succeeded` / `.step_up_failed` | Confirmação de segurança concluída / falhou: {action} | Security confirmation succeeded / failed: {action} | action, target id, reason (`missing`, `code_invalid`) |
| security | `security.channel_change_requested` / `.on_hold` / `.completed` / `.cancelled` / `.not_completed` | Troca de {channel}: solicitada / em espera até {time} / concluída / cancelada ({by}) / não concluída | {Channel} change: requested / on hold until {time} / completed / cancelled ({by}) / not completed | change id, **masked** old and new values (always contain `*`, e.g. `j***@d***.com`), `hold_until`, `by`; the reason of `not_completed` is never shown |
| security | `security.device_forgotten` | Dispositivo esquecido: {device label} | Device forgotten: {device label} | device id, label |
| security | `security.signed_out_everywhere` | Sessões encerradas em todos os dispositivos | Signed out everywhere | — |
| config | `accounts.created` / `.updated` / `.archived` / `.lifecycle_changed` | S5 §13 | | |
| config | `accounts.opt_in_chosen` / `.opt_in_suspended` | S5 §13 | | rule version, source URL(s), access date(s), `firm_stance` |
| config | `routing.subscription_changed` | S7 §9 | | |
| config | `profiles.default_changed` / `.account_changed` | S6 §8 | | |

Rules for the catalog: skip/refusal texts always name the account; **only entries are refused or blocked**: exits are never refused (P1), so refusal texts say "entrada" and the wording never suggests an exit was refused; texts for exits say "saída" when the Core flags the entry with `is_exit` (the client does not know the action rules); security entries never show a code, challenge id, token, cookie, or an unmasked e-mail or number (X-4, S3 AC-5), only the Core's masked values; wording must not sound like legal advice (vision §12). Detail fields that cannot be shown (secret-like) are removed by the Core before sending.

---

## 8. Accessibility

- The list is a real `table` (`th scope`, a caption "Atividade") with a visible focus on rows; rows are buttons or links (not click-only `div`s). Cards at < `md` keep the same order.
- The banner of new entries is `role="status"` (polite); entries are never inserted under the user's cursor.
- Filters: every control labeled; the active filter set is summarized in text above the list ("3 filtros ativos") and announced after changes.
- Outcome badges carry text and icon; nothing relies on color (F§1). Times are `<time datetime>`.
- The timeline is an ordered list per account; each card has a heading; collapsed cards expose `aria-expanded`.
- `Sheet`/`Drawer`: focus moves to the title, trapped, restored to the row. Escape closes.
- Targets ≥ 44 px on touch; the filter sheet respects safe areas; 320 px reflow without horizontal page scroll.

## 9. Data and actions this slice needs (for the Architect)

**Contract (draft 2.4.0, [s10-audit.md](../../architecture/contracts/s10-audit.md), `audit.yaml`):** read-only by construction (no write operation exists, AC-3).

| Operation | Use |
|---|---|
| `listActivity` `GET /api/v1/activity` | filters `from`, `to`, `account_id[]`, `source_id[]`, `signal_type[]`, `outcome[]`, `category[]`, `code[]`, `q` (instruction id or `message_id`, resolved through the receipt table); opaque `cursor` bound to the filters, `limit` ≤ 100 (UI uses 50), newest first; **no total count** (none is drawn); the page carries `retention_months` and `horizon` for the info lines |
| `getActivityFilterOptions` | accounts including archived, sources including revoked, signal types present |
| `getActivityEntry` | the detail panel |
| `getInstructionTimeline` | one call: summary, `intake` (receipt and duplicates), one block per account sorted by name with `outcome`, `final` and `quantity` (instruction → × multiplier → cap → executed), overall `status` (`completed` once every block is final) |

- **Entry fields** include `outcome`, `is_exit`, `message_id`, `source_name` (snapshot), `account_deleted` (computed), typed `details` (`AuditDetailsByCode`: the client types `details` by `code`; codes without a schema have empty details) and `safe_to_display`.
- **Open codes:** `code` is a pattern, not an enum, so a new slice's code never breaks an older client: it renders "Evento {code}" (§2). `KnownAuditCode` is the list the i18n parity check (QA) iterates over; `sync.*` (S11), `risk.*` (S13–S15) and `notifications.*` (S18) join with their own contracts.
- **SSE** `audit.entry_recorded` carries no details (`entry_id`, `category`, `outcome`, `account_id`, `instruction_id`); the client refetches. It drives the "novas entradas" banner and the live timeline.
- Isolation: unknown, foreign and purged ids give the same 404; foreign ids in filters match nothing.
- CSV export is not in V1 (Q44 (a) [Decision 2026-10-23]).

## 10. Decisions and open questions

**Closed by the PO (Rumo, 2026-10-23):** Q-S10-3 exact search by `message_id` accepted; Q-S10-5 security events share this list under category `Segurança` (S3 AC-16: sign-ins, failed or rate-limited attempts, new-device notice, step-up, channel change), and **no code is ever shown** (X-4, S3 AC-5); Q-S10-7 the `Master` signal filter appears only when the Core returns such entries (S19 is V1.1).

**Gap, not drawn (T1):** "late exits are audited as late" has no audit code or field in the contract today (5.1.1 / 5.2.0 draft). The catalog lists nothing for it. Compasso queued the gap for Esquadro (proposal: an optional field such as `late_ms` on `execution.command_sent` / `execution.filled` of an exit sent after the account TTL); S10 adds a detail field when it has a name.

**Architect (Compasso):** Q-S10-2 closed — final outcome list, `execution.*` codes, `details` per code and `safe_to_display` are in the contract (§3.3, §7, §9); Q-S10-6 duplicate shown inside the original instruction's timeline.

**Decided by the user (2026-10-23):**

| ID | Question | Decision |
|---|---|---|
| Q44 | CSV export of the log | (a): not in V1, **not drawn** |
| Q43 | Entry with no stop/target profile | Entry skipped with `routing.no_bracket_profile`; exits continue |
| Q74 / Q75 / Q76 | Reverse, close and reduce on barred or out-of-sync accounts | Audit side in §7 (gate code with `reverse_closed_qty` / `expected_flat`, `routing.routed` + `expected_flat`, `routing.nothing_to_reduce`); the notification codes `routing.reverse_close_only` and `routing.out_of_sync_flatten` are S18's (registry 1.2.1) |
| Q45 | Retention of 24 months | (a): read-only line for the user ("Os registros ficam guardados por {n} meses."); only the admin changes it |

## 11. AC traceability

| AC | Where |
|---|---|
| AC-1 every instruction, routing decision per account with reason, execution result: time, source, account | §3.2, §5, §7 |
| AC-2 list/filter by date, account, source, signal type, outcome; follow one instruction | §3.1, §5 |
| Q74–Q76 behaviour with audit codes | §7 |
| AC-3 entries not editable; retention 24 months rolling (configurable) | §2, §3 header, §9 |
| AC-4 no secrets | §2, §4 technical data, §9 |
| X-2, X-5, X-6, X-8 | §2, §7 (pt-BR/en), time rule |
