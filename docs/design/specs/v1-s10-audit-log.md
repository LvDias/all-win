# V1-S10 — Audit log and activity view — Screen spec

> **Owner:** Product Designer (Traço @ Dev3) · **Status:** proposed, pending PO review · **Revision:** 1 · **Date:** 2026-10-23
> **Slice:** [V1-S10](../../product/release-1/slice-plan.md) AC-1 … AC-4 (slice-plan rev. 7); cross-cutting X-2, X-4, X-5, X-6, X-8. Entry points from S3 (AC-16 events), S5, S6, S7, S11, S14, S15, S18.
> **Foundations:** [foundations.md](../foundations.md) (F§6.4 time, F§7 status, F§10 feedback) · **Related:** [S5](v1-s5-automation-opt-in.md) §13, [S6](v1-s6-stop-target-profiles.md) §8, [S7](v1-s7-routing.md) §9 (events they emit).
> **Contract:** no HTTP contract yet for reading the audit. Storage: `audit.audit_entry` ([data model §6.4](../../architecture/data/wave1-data-model.md)); codes are contract-owned, clients translate them (ADR-0001 §9). Reason codes: [ADR-0003 §4](../../architecture/adr/0003-instruction-contract-v1.md); ingest codes: [s1-webhook-ingest.md](../../architecture/contracts/s1-webhook-ingest.md). §9 lists what the screens need. Operation names are **design proposals**; the contract wins.
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
| Resultado / Outcome | multi-select: `Recebida`, `Roteada`, `Executada`, `Parcial`, `Ignorada`, `Recusada`, `Rejeitada`, `Expirada` | see §3.3 |
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
| `info` | — (no badge) | — | configuration, connection, security events |

**[Q-S10-2: the outcome taxonomy is a proposal; the Architect owns the final list.]**

---

## 4. S10-02 Entry detail (side panel)

`Sheet` from the right (≥ `md`) or full-screen `Drawer` (< `md`), title = the rendered text, focus moves to the title and returns to the row on close. Content:

1. **Resumo:** time with zone and seconds, outcome badge, category, account, source (name), signal type, `Ver instrução` link when applicable.
2. **Motivo / Why:** the translated reason for skips, refusals, rejections and automatic actions, plus one explanatory sentence from the catalog (§7).
3. **Detalhes:** a definition list of the fields the catalog says to show for that code (for example previous and new state, rule version, source URL and access date of a displayed rule, quantity requested vs executed, price, latency). Money and prices are strings formatted per F§6; source URLs open in a new tab with `rel="noopener noreferrer"`.
4. **Dados técnicos** (`Collapsible`, closed): entry id, `instruction_id`, `command_id`, `message_id`, `source_key_id`, code, recorded-at time; the JSON `details` **only if the Core marks it `safe_to_display`**. Mono font, `Copiar` button for the entry id (support).

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
| routing | `routing.out_of_sync` | Recusada para {account}: conta fora de sincronia | Refused for {account}: account out of sync | sync reason |
| routing | `routing.risk_blocked` | Recusada para {account}: limite de risco atingido | Refused for {account}: risk limit reached | limit, value |
| routing | `routing.expired` | Comando expirado para {account}: não foi executado | Command expired for {account}: not executed | TTL, base time |
| routing | `routing.quantity_capped` | Quantidade cortada em {account}: {executed} de {requested} | Quantity capped on {account}: {executed} of {requested} | requested, multiplier, maximum position, skipped excess |
| routing | `routing.no_bracket_profile` *(proposal, Q43 provisional)* | Entrada ignorada para {account}: sem perfil de stop/alvo (saídas continuam) | Entry skipped for {account}: no stop/target profile (exits continue) | account size |
| routing | `routing.max_position_reached` | Ignorada para {account}: posição máxima atingida | Skipped for {account}: maximum position reached | maximum |
| routing | `routing.position_conflict` | Recusada para {account}: a entrada é oposta à posição atual | Refused for {account}: entry opposes the current position | position, action |
| routing | `routing.nothing_to_close` / `.nothing_to_reduce` | Nada a {fechar / reduzir} em {account} | Nothing to {close / reduce} on {account} | |
| routing | `routing.no_correlated_order` | Ignorada para {account}: nenhuma ordem correspondente | Skipped for {account}: no matching order | |
| execution | `execution.report` *(proposal)* | {account}: ordem {recebida / em andamento / preenchida / parcialmente preenchida / cancelada / rejeitada} | {account}: order {received / working / filled / partially filled / cancelled / rejected} | status, quantity, price, time, platform reason, `child_role` (stop/target), `origin` |
| execution | `execution.bracket_calculated` *(proposal)* | Stop e alvo calculados em {account} | Stop and target calculated on {account} | unit, requested distance, whole ticks used, **rounding adjustment**, resulting prices (S6 AC-4) |
| risk | `risk.warning` / `risk.limit_reached` *(S13/S14)* | {account}: alerta de 80 % de {limit} / limite {limit} atingido | … | limit, value, percent |
| risk | `risk.flatten` / `risk.blocked` *(S15)* | {account}: posições fechadas e conta bloqueada até {time} | … | limit, value, reset time |
| risk | `risk.unblocked` *(S15, PO)* | {account} liberada no reset | {account} released at the reset | reset time, clock (real or simulated) |
| connection | `connection.status_changed` | Conexão {name}: {status} | Connection {name}: {status} | previous status |
| connection | `sync.*` (`sync.bracket_incomplete`, `sync.disconnected_in_trade`, …) | per S11 spec | | |
| security | sign-in, failed or rate-limited attempt, new-device notice, step-up result, channel change (S3 AC-16) | per S3 spec | | never a code or address beyond what S3 allows |
| config | `accounts.created` / `.updated` / `.archived` / `.lifecycle_changed` | S5 §13 | | |
| config | `accounts.opt_in_chosen` / `.opt_in_suspended` | S5 §13 | | rule version, source URL(s), access date(s), `firm_stance` |
| config | `routing.subscription_changed` | S7 §9 | | |
| config | `profiles.default_changed` / `.account_changed` | S6 §8 | | |

Rules for the catalog: skip/refusal texts always name the account; texts for exits say "saída" when the Core flags it; wording must not sound like legal advice (vision §12). Detail fields that cannot be shown (secret-like) are removed by the Core before sending.

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

**Reads**

| Need | Notes |
|---|---|
| `listActivity`: filters `from`, `to`, `account_ids`, `source_ids`, `signal_types`, `outcomes`, `categories`, `codes`, `q` (instruction id / `message_id`), cursor, limit (50) | Newest first. `from/to` as instants (X-8). Served by the user/time and user/account indexes |
| Entry fields: `id`, `occurred_at`, `category`, `code`, **`outcome`**, `instruction_id`, `account_id`, `account_name` (snapshot), `account_deleted`, `source_id`, `source_name`, `source_key_id`, `signal_type`, `command_id`, `details` (typed per code), `safe_to_display` | The UI never receives a secret (CHECK in the data model) |
| `getInstructionTimeline(instruction_id)`: instruction summary + steps grouped by account, ordered, with each account's final outcome and the instruction's overall status | One call; no N+1 |
| Filter option lists: accounts incl. archived, sources incl. revoked | |
| Retention horizon (months) | for the info lines |

**Events (SSE):** `audit.entry_recorded` (id, category, outcome, account id, instruction id) to drive the banner and the in-progress timeline.

**Codes the UI needs from the Architect:** the final `outcome` list, `execution.*` codes, the `details` fields per code (§7), `safe_to_display`.

## 10. Decisions and open questions

**Closed by the PO (Rumo, 2026-10-23):** Q-S10-3 exact search by `message_id` accepted; Q-S10-5 security events share this list under category `Segurança` (S3 AC-16: sign-ins, failed or rate-limited attempts, new-device notice, step-up, channel change), and **no code is ever shown** (X-4, S3 AC-5); Q-S10-7 the `Master` signal filter appears only when the Core returns such entries (S19 is V1.1).

**Architect (Compasso), no product objection:** Q-S10-2 final outcome taxonomy (§3.3); Q-S10-6 duplicate shown inside the original instruction's timeline.

**User questions drawn as recommendation (a), provisional:**

| ID | Question | Spec default (provisional) |
|---|---|---|
| Q44 | CSV export of the log | Not in V1: **not drawn** |
| Q45 | Retention of 24 months | Read-only line for the user ("Os registros ficam guardados por {n} meses."); only the admin changes it |

## 11. AC traceability

| AC | Where |
|---|---|
| AC-1 every instruction, routing decision per account with reason, execution result: time, source, account | §3.2, §5, §7 |
| AC-2 list/filter by date, account, source, signal type, outcome; follow one instruction | §3.1, §5 |
| AC-3 entries not editable; retention 24 months rolling (configurable) | §2, §3 header, §9 |
| AC-4 no secrets | §2, §4 technical data, §9 |
| X-2, X-5, X-6, X-8 | §2, §7 (pt-BR/en), time rule |
