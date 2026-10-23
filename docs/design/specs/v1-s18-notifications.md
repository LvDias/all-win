# V1-S18 — Notification center — Screen spec

> **Owner:** Product Designer (Traço @ Dev3) · **Status:** proposed, pending PO review · **Revision:** 1 · **Date:** 2026-10-23
> **Slice:** [V1-S18](../../product/release-1/slice-plan.md) AC-1 … AC-3 (slice-plan rev. 7); sources of notifications: S5, S7, S8, S11, S12, S13, S14, S15, S16 and S3 (instance state); links to S10.
> **Foundations and components:** [foundations.md](../foundations.md) (F§6.4 time, F§7 status, F§10 feedback), [components.md](../components.md) rev. 2 (C§3 `EmptyState`, C§5 `RiskBanner`, **C§6 `NotificationItem`, C§7 `NotificationBell`, C§8 `CriticalBanner`**). **Related specs:** [S10](v1-s10-audit-log.md) (the audit holds the full record), [S13](v1-s13-risk.md), [S5](v1-s5-automation-opt-in.md) §6, [S16](v1-s16-presets.md) §8.2.
> **Contract:** no HTTP contract for notifications yet. §9 lists what the Web screens need and, for the plugin, the data list the Plugin Engineer needs (the plugin panel itself is not designed here). Operation names are **design proposals**; the contract wins.
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

- In V1 notifications are **in-app only**: Web and plugin. No e-mail or messaging channel (S18 AC-3, Q8). E-mails for invitations, sign-in codes and new-device notices are account e-mails, not notifications.
- Events that create a notification (AC-1): **skipped excess** (quantity cap), **refusals**, **80 % warnings**, **blocks**, **out-of-sync flags**, **disconnects**. Section 6 lists the concrete events drawn from the other slices.
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
- **Live behavior (SSE):** a new `danger` item raises a destructive `Sonner` toast with `Ver` (8 s); `warning` and `info` update the bell silently; `critical` goes through `CriticalBanner` (§5). Bursts are coalesced ("3 novas notificações"). Mobile background tabs refetch and reconnect on return (F§9).

---

## 4. S18-02 Notification list — `/[locale]/notifications`

**Purpose:** read, filter and mark notifications (AC-1, AC-2).
**Permissions:** any signed-in user, own notifications only (X-2). A link to another user's object gives the standard `not found`.

**Layout:** `PageHeader` "Notificações" with primary-secondary action `Marcar todas como lidas` (disabled when none unread) · filter bar · list (`ul` of cards; this is not a `DataTable`, because items are sentences, not columns) · `Carregar mais`.

**Filters** (URL query, e.g. `?status=unread&category=risk&account=…`):

| Filter | Component | Values |
|---|---|---|
| Estado / Status | `ToggleGroup` | `Todas` (default) · `Não lidas` |
| Categoria / Category | `Select` | Todas · Roteamento · Risco · Sincronia · Conexão · Presets e opt-ins |
| Conta / Account | `Combobox`, includes deleted accounts `(excluída)` | |

**Item card** (`NotificationItem` variant `card`, C§6; all fields from the Core):
- Left: severity icon (text via sr-only label) and, for unread, a dot + a `Nova`/`Não lida` text badge (not color alone).
- Title (`font-semibold`), body (one or two lines), account name (link to the account), absolute date-time with seconds and zone (F§6.4), count `×3` and "última às {time}" when grouped (§6 grouping).
- Actions: primary `Ver detalhes` (link per §6 catalog), secondary `Marcar como lida` (a `Button variant=ghost`; for read items the text is the status "Lida" with no action). Opening the link also marks the item read.
- Severity tokens (F§7.2): `info` neutral/info `Info`; `success` `CircleCheck` (target reached); `warning` `TriangleAlert`; `danger` `OctagonX`; `critical` `OctagonX` with a thicker border and the text prefix "Crítica".

**Marking read**
- Single: button on the card, or opening the link. After the action the card updates in place (dot removed) and focus stays on the same button position (moves to the card's link). A polite toast is **not** shown for read changes (noise); the count change is the feedback.
- All: `Marcar todas como lidas` marks every unread item that matches the current filters; its label then states the scope: "Marcar as {n} como lidas". The action is not destructive and has no confirmation. Duplicate-submit protection per F§10.
- A read item can stay in the list (default). There is no delete or archive in V1 (not in the ACs).

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

Some events leave the user with a position or an account in an unsafe state and require an informed decision: both bracket legs filled while NinjaTrader was down (the orphan-leg position was flattened automatically, Q27), a disconnect with an open position, a bracket that could not be protected, a reactive-block flatten (S15). The slice plan asks for the **maximum alert** in the Q27 case. **[Provisional, Q51 rec. (a); user decides.]**

- A `critical` notification shows the **`CriticalBanner`** below the top bar: no close button and no "mark as read" shortcut; it stays until the item is read by opening it or marking it in the list. Reading acknowledges that the user saw it; it never resolves the state.
- `role="alert"` only when a critical item **arrives live**; on page load the banner is a labeled region and does not re-announce. It is fixed under the bar unless the viewport is shorter than 20 dvh or zoom is 200 % or more, where it is static (WCAG 2.4.11).
- Shell order: offline banner → `CriticalBanner` → aggregate `RiskBanner` (C§5).
- With several: "{n} alertas críticos não lidos" + `Ver`.
- The same event has the same wording in the shell, the account page and the list (C§5 rule). Critical items sort first in the panel and list.
- Nothing here offers to resync, reopen or unblock: after a disconnect the platform **alerts and waits for the user** (S11 AC-2), and there is no early unlock (S15 AC-3). The link goes to the account page where the S11/S15 controls live.

---

## 6. S18-03 Notification catalog

Texts are examples (pt-BR / en); the Core sends `code` + `details`, and the web renders from its catalog, with fallback "Aviso {code}" (a missing translation is detectable, S4 AC-4). Title and body wording for risk events is the **same as C§5/S13** (no per-screen wording). All values are strings from the Core (X-3). **Grouping** is the Core's (PO-approved, with three limits): the same `code` + account + trading day collapses into one item with a count and a last-time; **the audit keeps one entry per event** (S10 AC-1); **the first occurrence of each group creates a new unread notification** (a later occurrence updates count and last time of the group, it does not mark it unread again); and the "notify once" of the profit target (S13 AC-5) and the 80 % warning **per state change** are unaffected by grouping.

| Category | Code (S10 catalog / proposal) | Severity | Title / body pt-BR | Link |
|---|---|---|---|---|
| Roteamento | `routing.quantity_capped` (S7 AC-4) | warning | "Quantidade cortada em {account}" / "{executed} de {requested} contratos; o excedente foi ignorado." | S10 instruction timeline |
| Roteamento | `routing.max_position_reached` | warning | "Posição máxima atingida em {account}" / "Nada foi executado nesta conta." | S10 timeline |
| Roteamento | `routing.position_conflict` (S8 AC-4) | warning | "Entrada recusada em {account}" / "A entrada é oposta à posição atual. A fonte precisa enviar fechar ou reverter." | S10 timeline |
| Roteamento | `routing.risk_blocked` (S14 AC-1) | danger | "Entrada recusada em {account}: limite de risco atingido" / "Saídas continuam permitidas." | account Risk tab |
| Roteamento | `routing.out_of_sync` | danger | "Entrada recusada em {account}: conta fora de sincronia" | account detail |
| Roteamento | `routing.expired` (S12) | danger | "Comando expirado em {account}" / "Não foi executado." | S10 timeline |
| Risco | `risk.warning` (S13 AC-3) | warning | "Alerta (80 %) em {account}" / "{limit}: {used} de {limit_value} · {percent} %" | Risk tab |
| Risco | `risk.limit_reached` | danger | "Limite atingido em {account}" / "{limit}: …" | Risk tab |
| Risco | `risk.target_reached` (AC-5) | success | "Meta diária atingida em {account}" (once) | Risk tab |
| Risco | `risk.consistency_warning` | warning | "Regra de consistência em {percent} % do limite" | Risk tab |
| Risco | `risk.blocked` / `risk.flatten` (S15) | danger | "{account}: posições fechadas e conta bloqueada até {time}" / "Motivo: {reason}. Não há desbloqueio antecipado." | S15 account screen |
| Risco | `risk.unblocked` (S15, Q62 provisional) | info | "{account} liberada no reset" / "O bloqueio terminou. A conta volta a receber ordens." | Risk tab |
| Risco | `risk.reactive_block_action` (S15 AC-6) | danger | "Ordem manual cancelada em {account}: conta bloqueada" | S10 entry |
| Sincronia | `sync.*` flag set (S11 AC-1): `sync.command_undelivered`, `sync.bracket_incomplete`, `sync.simulation_reset`, … | danger | "{account} fora de sincronia" / reason text per code | account detail (S11) |
| Sincronia | `sync.disconnected_in_trade` (S11 AC-2) | **critical** | "{account} desconectou com posição aberta" / "O All Win não reabre nem ressincroniza sozinho. Decida o que fazer." | account detail |
| Sincronia | `sync.bracket_orphan_leg` (S11 AC-5 i) | danger | "Ordens órfãs canceladas em {account}" / list of what was cancelled | S10 entry |
| Sincronia | `sync.bracket_double_fill` (S11 AC-5 iii, Q27) | **critical** | "Posição aberta pela perna órfã fechada automaticamente em {account}" | S10 entry |
| Sincronia | `sync.bracket_integrity_unknown` | warning | "Integridade das ordens de proteção desconhecida em {account}" | account detail |
| Conexão | `connection.status_changed` to `disconnected` / `outdated` (S3) | danger / warning | "NinjaTrader {name} desconectado" · "…precisa de atualização" | Connections |
| Presets e opt-ins | `accounts.opt_in_suspended` (S5 §6) | warning | "Regra de automação da {firm} mudou: {k} contas com ordens automatizadas suspensas." | `/accounts?state=choice_pending` |
| Presets e opt-ins | `presets.new_version` (S16 §8.2, one per firm) | info | "Nova versão do preset da {firm}: {k} contas podem ser atualizadas." | `/accounts?preset_update=1` |

**Not notified (audit only, PO):** skips because the account does not accept automation (S7 AC-6) or the signal type is not selected (S7 AC-2), and duplicates. **Skip for a missing stop/target profile** (S6 Q43): notify, grouped, `warning` — **[Provisional, Q50 rec. (a); user decides]**; code `routing.no_bracket_profile`, link to the account's profile tab.

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
| `listNotifications`: filters `status`, `category`, `account_ids`; cursor; limit 50. Item: `id`, `created_at`, `updated_at` (last occurrence), `code`, `category`, `severity` (`info`/`success`/`warning`/`danger`/`critical`), `details` (typed per code), `account_id`, `account_name` (snapshot), `count`, `read_at`, `link` (a structured target: route + params, not a URL string), `audit_entry_id`/`instruction_id` | Own user only; newest first, critical unread pinned |
| `getUnreadCount` (total and worst severity) | for the bell and shell; the web does not count |
| Aggregate banner data (C§5) comes from the risk endpoints, not from here | |

**Web writes:** `markNotificationRead` (id), `markNotificationsRead` (filter or id list; returns the new unread count). Idempotent.

**Events (SSE):** `notification.created`, `notification.updated` (count/last time on a grouped item), `notification.read` (so another tab or the plugin keeps the count in sync).

**Plugin panel — data list (the panel is designed by the Plugin Engineer, per the NinjaTrader host's UI conventions):**
- Same fields as above: id, time (UTC instant, shown in the user's time zone), `severity`, `code` + `details` rendered by the plugin's own catalog in the user's language (S4 AC-3), account (platform account name when mapped), count, read state, short link target (a text hint such as "Abrir no All Win", not a URL with a secret).
- Unread count and worst severity for the plugin's status area.
- Mark read from the plugin; there is **one** set of notifications and **one** unread count, so reading on either side counts on both (PO).
- Critical items and the state `blocked until {time}` (S15 AC-6) are shown in the plugin even when the Core is unreachable, from the plugin's cached state (PO, consistent with S15 AC-4; cache detail: Plugin Engineer/Architect).
- Terms for plugin text: F§5 glossary.

## 10. Decisions and open questions

**Closed by the PO (Rumo, 2026-10-23):** Q-S18-1 grouping with count and last time, within the three limits in §6; Q-S18-2 skips for opt-in not accepted and signal type not selected are audit only; Q-S18-4 one set of notifications and **one unread count** across Web and plugin, reading on either side counts on both; Q-S18-6 retention equals the audit (24 months; erased 30 days after the account is closed, Q13-b); Q-S18-5 the plugin shows the blocked state and critical items from its cache, consistent with S15 AC-4 (cache detail: Plugin Engineer/Architect).

**Closed by Aquarela:** Q-S18-7 bell only, no sidebar item; Q-S18-8 components in components.md rev. 2 (C§6, C§7, C§8).

**User questions drawn as recommendation (a), provisional:**

| ID | Question | Spec default (provisional) |
|---|---|---|
| Q50 | Skip for a missing stop/target profile | Notify, grouped (`warning`) |
| Q51 | "Maximum alert" (Q27) | A top-of-page alert in the Web that cannot be dismissed until the item is marked read (`CriticalBanner`), an equivalent in the plugin, and the audit entry |

## 11. AC traceability

| AC | Where |
|---|---|
| AC-1 notifications for skipped excess, refusals, 80 % warnings, blocks, out-of-sync, disconnects; Web and plugin | §6 catalog, §9 plugin data |
| AC-2 mark as read; unread count visible | §3 badge, §4 marking |
| AC-3 in-app only; account e-mails are not notifications | §2 |
| X-2, X-3, X-5 (audit pointers), X-6 | §4 permissions, §9, §6 links to S10, §7 pt-BR/en |
