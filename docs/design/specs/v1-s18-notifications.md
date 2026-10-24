# V1-S18 — Notification center — Screen spec

> **Owner:** Product Designer (Nanquim, from Traço @ Dev3) · **Status:** proposed, pending PO review · **Revision:** 3c (rev. 3: catalog on registry 1.6.0, S15 codes, `day_unknown` variant, "Mesmo evento" group, Bisturi M1/L1/L2 of 7c177284a96e; rev. 3b: episode toast text follows its items (Rumo M1 of a1d5fe1f5882), protocol 1.3 frame cited, §10 gaps; rev. 3c: toast rows for an episode that starts with `risk.flatten` (Bisturi L1 of 65816d2b3ac4)) · **Date:** 2026-10-24
> **Slice:** [V1-S18](../../product/release-1/slice-plan.md) AC-1 … AC-3 (slice-plan rev. 7); sources of notifications: S5, S7, S8, S11, S12, S13, S14, S15, S16 and S3 (instance state); links to S10.
> **Foundations and components:** [foundations.md](../foundations.md) (F§6.4 time, F§7 status, F§10 feedback), [components.md](../components.md) rev. 2 (C§3 `EmptyState`, C§5 `RiskBanner`, **C§6 `NotificationItem`, C§7 `NotificationBell`, C§8 `CriticalBanner`**). **Related specs:** [S10](v1-s10-audit-log.md) (the audit holds the full record), [S13](v1-s13-risk.md), [S5](v1-s5-automation-opt-in.md) §6, [S16](v1-s16-presets.md) §8.2.
> **Contract:** [HTTP contract 6.2.0](../../../contracts/http/openapi.yaml) (commit 02bf2d2 on `s/v1-http-6--contracts`; 6.1.0 = 345f1a2, 6.0.0 = 71fbba4): `contracts/http/modules/notifications.yaml` and the notes [s18-notifications](../../architecture/contracts/s18-notifications.md). Operations `listNotifications`, `getUnreadCount`, `markNotificationRead`, `markNotificationsRead`; plugin messages in protocol 1.3 (`notifications`, `notification`, `notification_read`) and **`contracts/notifications/v1/registry.json` 1.6.0** (the single code list; severity, grouping, `target` and `params` per code). **Where this spec and the contract differ, the contract wins.**
> **Surfaces:** Web (Frontend). Creating, grouping and counting notifications is the Core's (Backend). The plugin panel is the Plugin Engineer's. QA owns the checks.

---

## 1. What this spec covers

| # | Screen / element | Route |
|---|---|---|
| S18-01 | Bell with unread count (top bar) and quick panel | app shell (all app pages) |
| S18-02 | Notification list page | `/[locale]/notifications` |
| S18-03 | Notification catalog (which events notify, text, severity, link) | §6 |
| S18-04 | Critical notification presentation in the shell | app shell |
| S18-05 | Data for the NinjaTrader plugin panel | §9 (data only) |

There is **no** sidebar item (Aquarela confirmed): the bell is the entry point at every size; below `md` it is a link to the list. Components are defined in components.md and are not redefined here: `NotificationBell` (C§7), `NotificationItem` with variants `panel` and `card` (C§6), `CriticalBanner` (C§8).

---

## 2. Concepts shown to the user (no new rules)

- In V1 notifications are **in-app only**: Web and plugin. No e-mail or messaging channel (S18 AC-3, Q8). E-mails for invitations, sign-in codes, new-device notices and the notices sent to every channel when a pending channel change is cancelled (S3 AC-11b, Q88 a, [Decision 2026-10-24]) are account e-mails, not notifications.
- **Notifications start from now** (contract notes §3, `StartPolicy::FromNow`): what happened before S18 is never sent afterwards as a notification. **The history is the audit log (S10)**: every event reaches it in the same transaction, so nothing is lost for the user. The empty state of a new user therefore never implies "nothing happened"; the log is where to look.
- Events that create a notification (AC-1): **skipped excess** (quantity cap), **refusals**, **80 % warnings**, **blocks**, **out-of-sync flags**, **disconnects**. Section 6 lists the concrete events: exactly the codes of the notification registry 1.6.0.
- **Operational events stay out of the user's trail** (Maestro, 2026-10-24): internal events of the platform (queue, retries, health, gateway internals) are neither notifications nor audit entries shown to the user. Only decisions and facts about the user's accounts, orders and connections appear.
- The user can **mark notifications as read**; the **unread count is visible** (AC-2).
- A notification is an **event in time**. The **current state** (account blocked, at 80 %, out of sync, disconnected) stays visible in banners and badges for as long as it lasts (C§5 `RiskBanner`, F§7.2); marking a notification read never changes or hides the state, and it does not resolve the problem.
- The full, immutable record of everything is the audit log (S10). A notification is a short pointer into it: every item links to the account page or to the S10 entry that explains it.

---

## 3. S18-01 Bell and quick panel (`NotificationBell`, C§7)

This spec only fixes what the bell shows and links to; behavior (panel of 8 items ordered critical first, then unread by time, then read; focus, arrows, Esc; polite announcement only when the count rises, at most once per 5 s; a critical item is not re-announced by the bell) is C§7.

- **Placement:** top bar, left of the user menu, at every size. At `md` and above it opens the panel with `Marcar todas como lidas` and `Ver todas` → S18-02; below `md` it is a link to S18-02.
- **Name and count:** `aria-label` "Notificações, {n} não lidas" / "Notifications, {n} unread"; `99+` above 99; badge color from the worst unread severity, with the number as text.
- **Item in the panel:** `NotificationItem` variant `panel`: severity icon, title, one-line body, account, relative time with absolute `<time>` and tooltip, unread marker with sr-only text; the stretched link opens the target and marks the item read.
- **States:** loading, error, empty and stale-stream states as C§7 plus: stale → "Sem dados recentes" and the count dimmed.
- **Live behavior (SSE):** a new `danger` item raises a destructive `Sonner` toast with `Ver` (8 s); `warning` and `info` update the bell silently; `critical` goes through `CriticalBanner` (§5). Bursts are coalesced ("3 novas notificações"). **One toast per account and burst:** the `danger` items of one enforcement episode (same `enforcement_id`, same account; for example `risk.blocked` and its `risk.flatten` entries) raise a single toast with `Ver`, never one toast per item. Its text follows the items of the episode received so far, chosen by the web only from the codes and `params` (Rumo, 2026-10-24):

  | Items of the episode | pt-BR | en |
  |---|---|---|
  | `risk.blocked` only | "{account} bloqueada até o reset" | "{account} blocked until the reset" |
  | + `risk.flatten` `action` = `flattened` | "{account}: conta bloqueada e posições fechadas" | "{account}: account blocked and positions closed" |
  | + `risk.flatten` `action` = `orders_cancelled` only | "{account}: conta bloqueada e ordens canceladas" | "{account}: account blocked and orders cancelled" |
  | + both actions | "{account}: conta bloqueada, posições fechadas e ordens canceladas" | "{account}: account blocked, positions closed and orders cancelled" |
  | `risk.flatten` `flattened` only, no `risk.blocked` yet | "{account}: posições fechadas pelo limite de risco" | "{account}: positions closed by the risk limit" |
  | `risk.flatten` `orders_cancelled` only, no `risk.blocked` yet | "{account}: ordens canceladas pelo limite de risco" | "{account}: orders cancelled by the risk limit" |
  | both `risk.flatten` actions, no `risk.blocked` yet | "{account}: posições fechadas e ordens canceladas pelo limite de risco" | "{account}: positions closed and orders cancelled by the risk limit" |

  An episode can start with `risk.flatten` (a `while_disconnected` flatten may come before `risk.blocked`; HTTP 6.2.0). "Conta bloqueada" appears only once a `risk.blocked` item of the episode has arrived; then the same toast switches to the matching row above. The toast never states an action that has no item. If a later item of the same episode arrives while the toast is visible, the same toast updates its text (no second toast). Mobile background tabs refetch and reconnect on return (F§9).
- **"Mesmo evento" group (accepted by the Maestro, 2026-10-24):** in the panel, items of the same account that carry the same `params.enforcement_id` are shown together under a small label "Mesmo evento" / "Same event", in their normal order. This is presentation only: **each item stays its own notification** (own read state, own count, own link); the unread count is the Core's and is not changed by the group. Items without `enforcement_id` (absent only when the Core never recorded the block) stay standalone. Items are never merged into one.

---

## 4. S18-02 Notification list — `/[locale]/notifications`

**Purpose:** read, filter and mark notifications (AC-1, AC-2).
**Permissions:** any signed-in user, own notifications only (X-2). A link to another user's object gives the standard `not found`.

**Layout:** `PageHeader` "Notificações" with primary-secondary action `Marcar todas como lidas` (disabled when none unread) · filter bar · list (`ul` of cards; this is not a `DataTable`, because items are sentences, not columns) · `Carregar mais`.

**Filters** (URL query, e.g. `?status=unread&category=risk&account=…`):

| Filter | Component | Values |
|---|---|---|
| Estado / Status | `ToggleGroup` | `Todas` (default) · `Não lidas` |
| Categoria / Category | `Select` | One value per registry `category`: Todas · Roteamento (`routing`) · Risco (`risk`) · Conexão e sincronia (`connection`: connection status and every `sync.*` code) · Execução (`execution`) · Configuração (`config`: opt-ins and presets) · Sinais recebidos (`ingress`) |
| Conta / Account | `Combobox`, includes deleted accounts `(excluída)` | |

**Item card** (`NotificationItem` variant `card`, C§6; all fields from the Core):
- Left: severity icon (text via sr-only label) and, for unread, a dot + a `Nova`/`Não lida` text badge (not color alone).
- Title (`font-semibold`), body (one or two lines), account name (link to the account), absolute date-time with seconds and zone (F§6.4), count `×3` and "última às {time}" when grouped (§6 grouping).
- Actions: primary `Ver detalhes` (the target per §6 catalog; **an item whose code has no `target` shows no link and no `Ver detalhes`**, only the text), secondary `Marcar como lida` (a `Button variant=ghost`; for read items the text is the status "Lida" with no action). Opening the link also marks the item read.
- Severity tokens (F§7.2): `info` neutral/info `Info`; `success` `CircleCheck` (target reached); `warning` `TriangleAlert`; `danger` `OctagonX`; `critical` `OctagonX` with a thicker border and the text prefix "Crítica".

**Marking read**
- Single: button on the card, or opening the link. After the action the card updates in place (dot removed) and focus stays on the same button position (moves to the card's link). A polite toast is **not** shown for read changes (noise); the count change is the feedback.
- All: `Marcar todas como lidas` marks every unread item that matches the current filters; its label then states the scope: "Marcar as {n} como lidas". The action is not destructive and has no confirmation. Duplicate-submit protection per F§10.
- A read item can stay in the list (default). There is no delete or archive in V1 (not in the ACs).
- **"Mesmo evento":** as in the panel (§3), cards of one account with the same `params.enforcement_id` sit next to each other with the label "Mesmo evento" / "Same event" (a visual group, an `li` that holds a nested `ul` with an accessible name "Mesmo evento: {account}"). Marking one card read never marks the others; `Marcar todas como lidas` works on items as usual.

**States**

| State | UI |
|---|---|
| Loading | Skeleton of 6 cards |
| Empty, never any notification | `EmptyState` (C§3 `first-use`): "Nenhuma notificação ainda." / "Avisos sobre limites, bloqueios, desconexões e ordens ignoradas aparecem aqui e no sino." |
| Empty after filter | `EmptyState` `filtered` (chips + `Limpar filtros`) |
| Error | `ErrorState` + retry + code |
| New items while reading | banner `role="status"` "{k} novas notificações · Mostrar" (items never insert under the cursor, like S10) |
| Mark-read failure | the card reverts, destructive toast with the translated code |
| Page size | 50 with `Carregar mais` and a count ("50 de 132"), as C§4 |

---

## 5. S18-04 Critical notifications in the shell (`CriticalBanner`, C§8)

Only **two** cases are `critical` (PO, 2026-10-23/24; registry 1.6.0). Everything else, including a disconnect with an open position and a failed flatten whose protection stayed intact, is `danger`:

| Case | Code | Why |
|---|---|---|
| Q27 orphan leg | `sync.bracket_double_fill` | Both bracket legs filled while NinjaTrader was down; the position created by the orphan leg was flattened automatically. |
| U19 failed flatten | `sync.flatten_failed_unprotected` | A flatten of any origin (close, reverse including Q75, the Q27 flatten, S15 local enforcement) failed after cancelling protection: the position stays open and protection is lost, fully or in part. |

The slice plan asks for the **maximum alert** in these cases (Q51 (a), [Decision 2026-10-23]: a banner that cannot be dismissed until the item is read, an equivalent in the plugin, and the audit entry).

- A `critical` notification shows the **`CriticalBanner`** below the top bar: no close button and no "mark as read" shortcut; it stays until the item is read by opening it or marking it in the list. Reading acknowledges that the user saw it; it never resolves the state.
- `role="alert"` only when a critical item **arrives live**; on page load the banner is a labeled region and does not re-announce. It is fixed under the bar unless the viewport is shorter than 20 dvh or zoom is 200 % or more, where it is static (WCAG 2.4.11).
- Shell order: offline banner → `CriticalBanner` → aggregate `RiskBanner` (C§5).
- With several: "{n} alertas críticos não lidos" + `Ver`.
- The same event has the same wording in the shell, the account page and the list (C§5 rule). Critical items sort first in the panel and list.
- Nothing here offers to resync, reopen or unblock: after a disconnect the platform **alerts and waits for the user** (S11 AC-2), and there is no early unlock (S15 AC-3). The link goes to the account page where the S11/S15 controls live.

---

## 6. S18-03 Notification catalog

The catalog is **the notification registry 1.6.0** (`contracts/notifications/v1/registry.json`), code by code. Severity, grouping, `target` and the `params` schema come from the registry, so this table does not add or rename codes; **where they differ, the registry wins**. Texts are examples (pt-BR / en); the Core sends `code` + `params`, and the web renders from its catalog, with the generic fallback "Aviso {code}" and the severity when a code or its `params` are unknown (a missing translation is detectable, S4 AC-4; the item, its count and its read state are kept). Title and body wording for risk events is the **same as C§5/S13** (no per-screen wording). All values are strings from the Core (X-3).

**Grouping** is the Core's, one of five registry values:

| Grouping | Rule | Codes |
|---|---|---|
| `account_trading_day` | Same code + account + trading day: one item with a count and a last time. The first occurrence is a new unread item; a later one only updates count and last time, except where `reopen_on_occurrence` is true (below). | the `routing.*` skips and refusals, `sync.flagged` and `risk.reactive_block_action` (never reopens) |
| `firm_release` | One item per firm and release, with `account_count` (the audit keeps one entry per account). | `accounts.opt_in_suspended`, `presets.new_version` |
| `connection_trading_day` | Same code + connection + trading day. Never reopens. | `execution.unbound_activity` |
| `source_trading_day` | Same code + source + trading day. Never reopens. | `ingress.duplicate_conflict` |
| `none` | Every occurrence is its own notification. | `routing.reverse_close_only`, `routing.out_of_sync_flatten`, `risk.*` except `risk.reactive_block_action`, `connection.status_changed`, the other `sync.*` codes |

The audit keeps **one entry per occurrence** (S10 AC-1). "Notify once" of the daily profit target (S13 AC-5) and the 80 % warning **per state change** are not affected by grouping (`none`).

**Reopening (Q68 (b), [Decision 2026-10-23]; registry flag `reopen_on_occurrence`):** the three decided refusal codes `routing.risk_blocked`, `routing.out_of_sync` and `routing.expired`, **and `sync.flagged` (a new reason marks its group unread again; Q99 yes, [Decision 2026-10-24]: a group the user read or dismissed reopens on a new reason)**, reopen the group as unread on each new occurrence in the same trading day (`read_at` cleared, count and last time updated, a new `revision`). Every other grouped code, `risk.reactive_block_action` included (`reopen_on_occurrence: false`), stays read and only the count and last time rise. A reopened group counts again in the unread count, bumps the bell and, as a `danger` item, raises the toast (§3). **The Q68 list is the three routing codes**: `routing.reverse_close_only` and `routing.out_of_sync_flatten` are `danger` but are never grouped, so each occurrence is already a new unread item and nothing reopens.

Which tab a link opens (for example the Risk tab of the account) is the client catalog's choice per code, built from the registry `target` = `{kind, id}` (`account`, `connection`, `instruction`, `route`, `activity_entry`). A notification never carries a URL.

### 6.1 Roteamento — `routing.*`

| Code | Severity | Grouping | Target | Title / body pt-BR | Note |
|---|---|---|---|---|---|
| `routing.quantity_capped` (S7 AC-4) | warning | account_trading_day | instruction | "Quantidade cortada em {account}" / "{executed} de {requested} contratos; o excedente foi ignorado." | |
| `routing.max_position_reached` | warning | account_trading_day | instruction | "Posição máxima atingida em {account}" / "Nada foi executado nesta conta." | |
| `routing.position_conflict` (S8 AC-4) | warning | account_trading_day | instruction | "Entrada recusada em {account}" / "A entrada é oposta à posição atual. A fonte precisa enviar fechar ou reverter." | |
| `routing.no_bracket_profile` (S6, Q50 (a), [Decision 2026-10-23]) | warning | account_trading_day | account | "Entrada ignorada em {account}: sem perfil de stop/alvo" / "Escolha um perfil na aba Perfil da conta." | link to the profile tab |
| `routing.instrument_unavailable` | warning | account_trading_day | instruction | "Instrumento indisponível em {account}" / "{instrument_symbol} não tem contrato ativo agora. Nada foi executado." | |
| `routing.adapter_protocol_outdated` | warning | account_trading_day | account | "Plugin desatualizado em {account}" / "Entradas foram ignoradas; as saídas continuam. Atualize o plugin." | entries skipped, exits continue |
| `routing.position_unknown` | warning | account_trading_day | instruction | "Redução ignorada em {account}" / "O All Win não reconhece a origem da posição aberta." | a reduce skipped (position opened outside All Win) |
| **`routing.nothing_to_reduce`** (Q76) | **warning** | account_trading_day | instruction | "Redução sem posição em {account}" / "A conta está fora de sincronia e não havia posição para reduzir. Nada foi enviado." | **Normal group (not reopened, not in the Q68 list).** Notified **only when the account is out of sync**, or when the sync state is unavailable at that moment (more transparency, never less). On an in-sync account it stays **audit only** (a benign race with a stop or target fill). A reduce is never sent as a flatten. |
| **`routing.reverse_close_only`** (Q74/Q75) | **danger** | none | instruction | "Inversão em {account}: só fechou a posição" / "{closed_qty} {long\|short} fechados; a nova entrada não foi aberta ({reason})." | One per occurrence, for every gate (including `routing.limits_pending`, otherwise not notified), for an open leg that expired, and, under Q75, after the flatten fills. It **replaces** the gate's own grouped notification for that occurrence: one item, not two. `reason` is `reason_code`: `routing.out_of_sync`, `routing.risk_blocked`, `routing.limits_pending`, `routing.no_bracket_profile`, `routing.max_position_reached` or `routing.expired`. For `routing.risk_blocked` the text is neutral, "bloqueio de risco" / "risk block", never "limite atingido": `ParamsReverseCloseOnly` carries no `risk_detail`, so the client cannot tell a reached limit from an unknown day (gap with Compasso, §10). |
| **`routing.out_of_sync_flatten`** (Q75) | **danger** | none | instruction | "Fechamento em {account}: conta fora de sincronia" / filled: "{closed_qty} {long\|short} fechados. Ordens pendentes no contrato foram canceladas, inclusive as manuais."; already flat: "Não havia posição. Ordens pendentes no contrato foram canceladas, inclusive as manuais." | Once per flatten on an out-of-sync account whose expected position was flat, ending in a terminal success (`filled` or `already_flat`). A `reverse` that filled is `routing.reverse_close_only` instead. The text always says working orders were cancelled, manual ones included (Q75-1, [Decision 2026-10-24]); the Core does not count them. |
| `routing.risk_blocked` (S14 AC-1) | danger | account_trading_day, **reopens** | account | Limit: "Entrada recusada em {account}: limite de risco atingido" / "Saídas continuam permitidas." · **`params.risk_detail` = `day_unknown`** (Q92 c): "Entrada recusada em {account}: início do dia desconhecido" / "O All Win não sabe quando o dia de trading começou. Novas entradas automáticas estão bloqueadas; saídas continuam." (en: "Entry refused on {account}: trading day start unknown" / "All Win doesn't know when the trading day started. New automated entries are blocked; exits continue.") | link to the Risk tab, where the `day_unknown` case shows `Reiniciar contagem de hoje` ([S13](v1-s13-risk.md) §4.4). The `day_unknown` text never says a limit was reached. |
| `routing.out_of_sync` | danger | account_trading_day, **reopens** | account | "Entrada recusada em {account}: conta fora de sincronia" | |
| `routing.expired` (S12) | danger | account_trading_day, **reopens** | instruction | "Comando expirado em {account}" / "Não foi executado." | An exit never expires; only an entry leg (or the open leg of a reverse). |

`routing.reverse_close_only` and `routing.out_of_sync_flatten` are **notification codes**, not audit codes. S10 holds the audit side (the gate's own code with outcome `partial`, `executed_qty` 0 and `reverse_closed_qty`; for Q75 the routing entry `routing.routed` or `routing.out_of_sync` with `expected_flat` and the result in the execution entries). The wording of the entries is in [S10](v1-s10-audit-log.md).

### 6.2 Risco — `risk.*`

| Code | Severity | Grouping | Target | Title / body pt-BR | Note |
|---|---|---|---|---|---|
| `risk.warning` (S13 AC-3) | warning | none | account | "Alerta (80 %) em {account}" / "{limit}: {used} de {limit_value} · {percent} %" | One per state change (normal → warning). |
| `risk.limit_reached` | danger | none | account | "Limite atingido em {account}" / "{limit}: …" | One per state change. |
| `risk.target_reached` (S13 AC-5) | success | none | account | "Meta diária atingida em {account}" | Once per trading day. |
| `risk.consistency_warning` | warning | none | account | "Regra de consistência em {account}: {percent} % do limite" | One per state change. |
| **`risk.evaluation_target_reached`** (S13b AC-5, Q70) | **success** | none | account | "Meta da avaliação atingida em {account}" / "Lucro de {achieved} sobre a meta de {profit_target}, contado desde {start}. O All Win não faz nada sozinho." | **Once per evaluation count**, with the Web notice on the account (one notice and one notification). Notify only: nothing is closed, blocked or restarted (Q70). Money is net (the figures come from the Core as strings). |

**S15 local enforcement (registry 1.6.0, HTTP 6.1.0).** `params` is `DetailsRiskEnforcement` (`enforcement_id`, `kind`, `limit`, `used`, `limits_version`, plus `trigger` on `risk.blocked`, `action` on `risk.flatten`, optional `while_disconnected` and `platform_time`). These `params` carry no block-end time: the text says "até o reset" and the account page shows the time (S15 §5).

| Code | Severity | Grouping | Target | Title / body pt-BR | Note |
|---|---|---|---|---|---|
| **`risk.blocked`** (S15 AC-2, AC-8) | danger | none | account | "{account} bloqueada até o reset" / "{kind}: {used} de {limit}. Novas ordens serão canceladas; não há desbloqueio antecipado." · `trigger` = `enabled_over_limit`: the body starts "Você ligou a proteção com o limite já passado hoje." | One per block. Link: account page, Risk tab (S15-03 card with the time). With `while_disconnected`: extra line "O plugin agiu sem conexão com o All Win." |
| **`risk.flatten`** (S15 AC-2) | danger | none | account | `action` = `flattened`: "Posições fechadas em {account}" / "Limite de risco: {kind}." · `orders_cancelled`: "Ordens canceladas em {account}" / "Limite de risco: {kind}." | One per enforcement action. Same `enforcement_id` as its `risk.blocked`: shown under "Mesmo evento" (§3) and in one toast. |
| **`risk.unblocked`** (S15 AC-9, Q62) | info | none | account | "{account} liberada no reset" / "O bloqueio terminou. A conta volta a receber ordens." | No toast. Same `enforcement_id` as the ended block when the Core recorded it. |
| **`risk.reactive_block_action`** (S15 AC-6) | **warning** | account_trading_day, never reopens | account | "Ordem manual cancelada em {account}: conta bloqueada" / "Enquanto a conta estiver bloqueada, ordens manuais são canceladas." | Grouped per account and trading day (×n, última às {time}); a later one does not mark the group unread (Q68 b). A resulting position is flattened and reported as `risk.flatten`. |

A flatten that **fails** under enforcement already has a code: `sync.flatten_failed_unprotected` when protection was lost (critical, §6.3). Whether S15 AC-6 "could not close everything" maps to U19 is open with Esquadro via Compasso (§10).

### 6.3 Conexão e sincronia — `connection.*`, `sync.*`, `execution.*`

| Code | Severity | Grouping | Target | Title / body pt-BR | Note |
|---|---|---|---|---|---|
| `connection.status_changed` | `disconnected` → danger · `outdated` → warning · other statuses not notified | none | connection | "NinjaTrader {name} desconectado" · "…precisa de atualização" | |
| `sync.flagged` (S11 AC-1) | danger | account_trading_day, **reopens** | account | "{account} fora de sincronia" / reason text per `params.reason` (the S11 reason codes) | One per reason added. Covers every reason that has no code of its own below, **including `sync.disconnected_in_trade` (a disconnect with an open position is `danger`, not `critical`) and `sync.disconnected_by_user`**. A failed flatten with the protection intact is `sync.flagged` (danger). |
| `sync.bracket_orphan_leg` (S11 AC-5 i) | warning | none | account | "Ordens órfãs canceladas em {account}" / list of what was cancelled | Replaces `sync.flagged` for that occurrence. |
| `sync.bracket_incomplete` (S11 AC-5 ii) | danger | none | account | "Proteção incompleta em {account}" / "Posição aberta sem stop ou alvo, ou com ordens em estado desconhecido. Nada foi recriado. Novas entradas automáticas ficam bloqueadas até a posição estar totalmente coberta ou zerada." (en: "… New automated entries stay blocked until the position is fully covered or closed.") | Replaces `sync.flagged`. Q100 (a), [Decision 2026-10-24]: partial or unknown protection counts as unprotected ([S11](v1-s11-out-of-sync.md) §4). |
| **`sync.bracket_double_fill`** (S11 AC-5 iii, Q27) | **critical** | none | account | "Posição aberta pela perna órfã fechada automaticamente em {account}" | Replaces `sync.flagged`. `CriticalBanner` (§5). |
| **`sync.flatten_failed_unprotected`** (U19) | **critical** | none | account | "Não foi possível fechar a posição em {account}" / "A proteção foi cancelada e a posição continua aberta. Decida o que fazer." | A flatten of any origin failed after cancelling protection. `CriticalBanner` (§5). |
| `sync.restored` (ADR-0004 §6) | info | none | account | After a disconnect: "{account} voltou ao normal" / "Houve uma desconexão sem posição; ao reconectar, tudo conferiu." · Automatic return (Q101 yes, [Decision 2026-10-24]): "{account} voltou ao normal" / "A posição ficou coberta por um stop ou foi zerada; novas entradas automáticas voltaram." | Also on the automatic return to in sync when the last reason held at the clear resolves ([S11](v1-s11-out-of-sync.md) §4, no actor). If cover is lost again, entries block again as a new reason (`sync.flagged`, reopens) that needs a new clear (Q14). |
| `execution.unbound_activity` (S11) | warning | connection_trading_day | connection | "Atividade em conta não vinculada" / "{platform_account} em {connection_name}: nenhuma conta do All Win está vinculada a ela." | Not a sync reason; the account is not flagged. |

**Disconnect by the user:** when the user disconnects the plugin with an open position or a command in flight, the account is flagged with the reason `sync.disconnected_by_user` and the item is `sync.flagged` (danger): "Você desconectou o plugin com posição aberta; o All Win não está acompanhando esta conta" / en: "You disconnected the plugin with an open position; All Win is not tracking this account". With **no** open position and no command in flight there is only the connection status and the audit entry, no notification. (The earlier proposal of a code of its own is replaced by the reason, per the contract.) Local enforcement keeps running after a disconnect by the user (a requirement, [plugin panels](v1-plugin-panels.md)).

### 6.4 Config, execução e entrada — `accounts.*`, `presets.*`, `ingress.*`

| Code | Severity | Grouping | Target | Title / body pt-BR | Note |
|---|---|---|---|---|---|
| `accounts.opt_in_suspended` (S5 §6) | warning | firm_release | **none** | "Regra de automação da {firm_name} mudou" / "{account_count} contas com ordens automatizadas suspensas. Confira em Contas." | **No link** (the registry has no `target`): the body says where to go. |
| `presets.new_version` (S16 §8.2) | info | firm_release | **none** | "Nova versão do preset da {firm_name}" / "{account_count} contas podem ser atualizadas. Veja em Contas." | **No link**, as above. |
| `ingress.duplicate_conflict` | warning | source_trading_day | instruction (the original) | "Sinal duplicado com conteúdo diferente" / "O mesmo identificador chegou com outro conteúdo. Nada foi executado." | No account on the item (the list shows no account for it, and the account filter does not match it). |

### 6.5 Not notified

- **Audit only (PO):** skips because the account does not accept automation (S7 AC-6) or the signal type is not selected (S7 AC-2); duplicates with identical content; `routing.nothing_to_close` on an in-sync flat account (S10); `routing.nothing_to_reduce` on an in-sync account (above).
- **Operational events** of the platform: never reach the user's trail (§2).
- **Missed instructions while disconnected (S12 AC-2):** **[Gap]** there is no registry code for it in 1.6.0. The facts already surface as `routing.expired` and `sync.flagged` (`sync.command_expired`, `sync.command_undelivered`). A separate notice needs a code from the Architect; it is not drawn here.
- **Late exit (T1):** `execution.exit_sent_late` (HTTP 6.1.0) is **audit only**, with `late_ms` ([S10](v1-s10-audit-log.md)); no notification.

---

## 7. Strings (examples)

| Where | pt-BR | en |
|---|---|---|
| Panel title | Notificações | Notifications |
| Mark all | Marcar todas como lidas | Mark all as read |
| Mark one | Marcar como lida | Mark as read |
| Unread (sr-only) | Não lida | Unread |
| Critical prefix | Crítica | Critical |
| Empty | Nenhuma notificação ainda. | No notifications yet. |
| Grouped | ×{n} · última às {time} | ×{n} · last at {time} |
| Shell critical | Alerta crítico: {title} | Critical alert: {title} |

---

## 8. Accessibility

- Bell: button with the count in its accessible name; the badge text is not duplicated for screen readers (`aria-hidden`).
- Panel: `Popover` with focus moved to the first item, arrow navigation, `Esc` to close and restore focus to the bell; the list inside is a real `ul`.
- Severity and read state are text + icon; the unread dot has an sr-only label. Nothing relies on color (F§1).
- Live announcements: count increases polite (coalesced); `critical` via `role="alert"`; no announcement for read changes.
- Items are links/buttons, never click-only containers; targets ≥ 44 px on touch; the list reflows at 320 px.
- Reduced motion: no slide-in; toasts follow the Sonner reduced-motion setting.

## 9. Data and actions this slice needs (for the Architect)

**Web reads**

| Need | Notes |
|---|---|
| `listNotifications`: filters `status` (`all`/`unread`/`read`), `category` (the registry categories), `account_id` (repeatable); opaque cursor; limit 50 (max 100). Item (the field names of protocol 1.3): `notification_id`, `revision`, `code`, `category`, `severity` (`info`/`success`/`warning`/`danger`/`critical`), `account_id`, `account_name` (snapshot, Web only), `params` (typed per code, from `params.schema.json`), `count`, `source_event_ids`, `audit_entry_id`, `created_at`, `updated_at` (last occurrence), `read_at`, `target` = `{kind, id}` (absent for the two per-firm codes; never a URL) | Own user only; unread critical first, then newest `updated_at`. Every page carries the current unread count |
| `getUnreadCount` (total and worst severity) | for the bell and shell; the web does not count |
| Aggregate banner data (C§5) comes from the risk endpoints, not from here | |

**Web writes:** `markNotificationRead` (id), `markNotificationsRead` (`notification_ids`, up to 200, **or** `up_to` with optional `category` and `account_id`; returns the new unread count). `up_to` marks only items updated at or before it, so an item that arrives while the user clicks "Marcar todas como lidas" stays unread. Idempotent.

**Events (SSE):** `notification.created`, `notification.updated` (full item: count and last time on a grouped item, or a reopened group), `notification.read` (so another tab or the plugin keeps the count in sync).

**Retention and start:** the same as the audit (24 months). The consumer starts from now (`StartPolicy::FromNow`); there is no backfill, and the history is the audit (§2).

**Plugin panel — data list (the panel is designed by the Plugin Engineer, per the NinjaTrader host's UI conventions):**
- Same fields as above: id, time (UTC instant, shown in the user's time zone), `severity`, `code` + `params` (protocol 1.3 `notification_item` in `contracts/adapter-protocol/v1/messages.schema.json`, commit 2b45236, with fixtures) rendered by the plugin's own catalog in the user's language (S4 AC-3), account (platform account name when mapped), count, read state, short link: the plugin builds it from the paired app address plus the route of the `target`, as plain text with the parameters (a hint such as "Abrir no All Win"). An item without `target` shows no link. Unknown codes or `params` that fail the plugin's copy are shown generic (code and severity), never refused.
- Unread count and worst severity for the plugin's status area. Protocol 1.3 (Esquadro): the snapshot after `welcome` carries the last 200 items and an **authoritative `unread_count`** covering everything; each `notification` is an upsert with a monotonic `revision`; codes, severities and `params` schemas are in `contracts/notifications/v1/registry.json`, while the pt-BR/en texts stay in each client's catalog with a drift test (every registry code has a key in both languages in both clients).
- Mark read from the plugin; there is **one** set of notifications and **one** unread count, so reading on either side counts on both (PO).
- Critical items and the state `blocked until {time}` (S15 AC-6) are shown in the plugin even when the Core is unreachable, from the plugin's cached state (PO, consistent with S15 AC-4; cache detail: Plugin Engineer/Architect).
- Terms for plugin text: F§5 glossary.

## 10. Decisions and open questions

**Closed by the PO (Rumo, 2026-10-23):** Q-S18-1 grouping with count and last time, within the three limits in §6; Q-S18-2 skips for opt-in not accepted and signal type not selected are audit only; Q-S18-4 one set of notifications and **one unread count** across Web and plugin, reading on either side counts on both; Q-S18-6 retention equals the audit (24 months; erased 30 days after the account is closed, Q13-b); Q-S18-5 the plugin shows the blocked state and critical items from its cache, consistent with S15 AC-4 (cache detail: Plugin Engineer/Architect).

**Closed by Aquarela:** Q-S18-7 bell only, no sidebar item; Q-S18-8 components in components.md rev. 2 (C§6, C§7, C§8).

**Decided by the user ([Decision 2026-10-23]):** Q50 (a) a skip for a missing stop/target profile is notified, grouped, `warning` (`routing.no_bracket_profile`); Q51 (a) the maximum alert is a `CriticalBanner` that cannot be dismissed until read, plus the plugin equivalent and the audit entry, and it applies to **two** cases only (Q27 orphan leg, U19 failed flatten); Q68 (b) the three refusal codes reopen their group, all others only count up. **[Decision 2026-10-24]:** Q99 yes (`sync.flagged` reopens on a new reason), Q100 (a) (partial/unknown protection = unprotected), Q101 yes (automatic `sync.restored`); the "Mesmo evento" group by `enforcement_id` and one toast per account and burst (Maestro). Q74, Q75 and Q76 define the three routing notifications in §6.1 (Q75-1: the text says manual orders were cancelled; [Decision 2026-10-24]).

**Gaps handed to others (not drawn here):**

| Gap | Owner |
|---|---|
| `ParamsReverseCloseOnly` has no `risk_detail`: a reverse cut by an unknown day cannot be told from a reached limit (neutral text until then) | Architect (Compasso), planned for HTTP 6.5.0 |
| S15 AC-6 "could not close everything" vs U19 | Esquadro via Compasso, planned for HTTP 6.5.0 |
| A code for "instructions missed while disconnected" (S12 AC-2) | Architect (the facts appear as `routing.expired` and `sync.*` today) |
| `risk.evaluation_target_reached` wording of the "restart evaluation" step is S13b's, not this spec's | Aquarela |

## 11. AC traceability

| AC | Where |
|---|---|
| AC-1 notifications for skipped excess, refusals, 80 % warnings, blocks, out-of-sync, disconnects; Web and plugin | §6 catalog (registry 1.6.0), §9 plugin data |
| AC-2 mark as read; unread count visible | §3 badge, §4 marking |
| AC-3 in-app only; account e-mails are not notifications | §2 |
| X-2, X-3, X-5 (audit pointers), X-6 | §4 permissions, §9, §6 links to S10, §7 pt-BR/en |
