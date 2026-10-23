# S18 — Notification center HTTP contract notes

- **Status:** proposed (2026-10-23), for Code Review. Owner: Software Architect (Compasso). Contract version **3.2.0**, on top of 3.1.0 (S13).
- **Contract:**
  - [`contracts/http/modules/notifications.yaml`](../../../contracts/http/modules/notifications.yaml) (Web);
  - [`contracts/notifications/v1/registry.json`](../../../contracts/notifications/v1/registry.json) (codes, shared with the plugin);
  - the generated [`params.schema.json`](../../../contracts/notifications/v1/params.schema.json).
- **Inputs:** slice-plan rev. 7, V1-S18 AC-1…AC-3 · design spec `v1-s18-notifications.md` rev. 1 §3, §6, §9, §10 · agreement with Esquadro (adapter protocol 1.3: field names, severity, registry ownership).

## 1. Operations

| Operation | Method and path | Notes |
|---|---|---|
| `listNotifications` | `GET /api/v1/notifications?status=&category=&account_id=&cursor=&limit=` | Unread critical first, then newest first (`updated_at`). MAC cursor. Each page carries `unread` |
| `getUnreadCount` | `GET /api/v1/notifications/unread-count` | `{unread_count, worst_severity?}`, the same pair as the 1.3 snapshot |
| `markNotificationRead` | `POST /api/v1/notifications/{notification_id}/read` | Idempotent; bumps `revision` |
| `markNotificationsRead` | `POST /api/v1/notifications/read` | `notification_ids` (≤ 200) **or** `up_to` (+ optional `category` / `account_id`) |

SSE: `notification.created` and `notification.updated` (full item + `unread`), and `notification.read` (ids + `unread`). Another tab or the plugin stays in sync, because reading on either channel counts on both (Q-S18-4).

## 2. Decisions

1. **One state, two channels.** The field names are the adapter protocol 1.3 ones (Esquadro): `notification_id`, `revision` (monotonic, upsert by higher revision), `code`, `category`, `severity`, `account_id`, `params`, `count`, `source_event_ids`, `created_at` / `updated_at` / `read_at`, `target`. The Web adds only `account_name` (snapshot) and `audit_entry_id`.
2. **Severity:** `info`, `success`, `warning`, `danger`, `critical`, on both channels, agreed with Esquadro. `critical` is the ADRs' top severity and drives the `CriticalBanner` (Q51: it stays until read). **In V1 so far no registry code is `critical`:** the banner gets its producers with S11 (`sync.disconnected_in_trade`, `sync.bracket_double_fill`) and S15 (Crivo N1). The plugin may map `success` → `info` and `danger` → `warning` in its own UI; the protocol carries the five values.
3. **The registry is the single code list** (owner: Compasso; 1.3 references it).
   - Each code has `category`, `severity` (or `severity_by_params` for `connection.status_changed`: `disconnected` → danger, `outdated` → warning, other statuses not notified), `grouping`, `target` and `params`.
   - `params` are the audit `details` schemas where the code is an audit code (`DetailsRoutingDecision`, `DetailsRiskState`, `DetailsConnectionStatus`), so the Web and the plugin render the same fields as S10. They are their own schemas for the two per-firm aggregates (`accounts.opt_in_suspended`, `presets.new_version`).
   - `params.schema.json` is generated from the HTTP bundle (`NotificationParamsByCode` → `$defs`), so the C# plugin validates `params` without YAML. `check_openapi.py` fails if the registry and `NotificationParamsByCode` differ, if a name or value is unknown, or if the generated file is stale.
4. **Grouping (Q-S18-1)** is the Core's:
   - `account_trading_day` for the routing skips and refusals: same code + account + trading day → one item with `count` and last time; only the first occurrence is unread;
   - `firm_release` for the per-firm items;
   - `none` for risk (one per state change; the profit target once per day, S13 AC-5) and connections.

   The audit keeps one entry per occurrence. **Q68 (b, decided):** a new occurrence marks a grouped item unread again only for codes with `reopen_on_occurrence: true` in the registry (`routing.risk_blocked`, `routing.out_of_sync`, `routing.expired`). The others stay read, and only `count` and the last time change. Options (a) "always reopen" and (c) "never reopen" are the same flag set on every code, so the contract does not change.
5. **Not notified (Q-S18-2, PO):** skips for an opt-in not accepted, signal type not selected, and duplicates. **Q50 :** `routing.no_bracket_profile` is notified, grouped, `warning`.
6. **Links:** `target` = `{kind, id}` with kind ∈ `account`, `connection`, `instruction`, `route`, `activity_entry`. Never a URL in the payload. The Web builds its route; the plugin uses `welcome.app_url` + route. Which tab (e.g. Risk) is a client catalog decision per code.
7. **Mark all is race-free:** `up_to` marks only items whose `updated_at` ≤ `up_to`, so an item arriving during the click stays unread. Reading never changes the state the item reports (banners and badges stay).
8. **Retention (Q-S18-6):** the same as the audit (24 months; erased 30 days after the account is closed).
9. **Not yet in the registry:** `sync.*` (S11) and `risk.blocked` / `risk.flatten` / `risk.unblocked` / `risk.reactive_block_action` (S15). They are added with those contracts, each with its own `details` schema in `modules/audit.yaml`.

## 3. For the Database Architect and Backend (through the Maestro)

- Table `notifications.notification`:
  - columns: id, user_id, code, category, severity, account_id, params jsonb, count, revision, source_event_ids, audit_entry_id, created_at, updated_at, read_at, group_key;
  - unique (user_id, group_key) for open groups;
  - index (user_id, read_at NULLS FIRST, severity, updated_at DESC) for the list and the counter.
- Index hints (Crivo N3):
  - the unread counter is a partial index `(user_id, severity) WHERE read_at IS NULL`;
  - `group_key` is defined by the Core from the registry grouping: `code|account_id|trading_day` for `account_trading_day`, `code|firm_id|release` for `firm_release`, and none for `none`.
- The producer is the audit consumer (outbox), which applies the registry's grouping and `reopen_on_occurrence`. The 1.3 snapshot and the HTTP list come from the same table.
