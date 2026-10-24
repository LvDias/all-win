# S11 — Out-of-sync HTTP contract notes

- **Status:** in Code Review (2026-10-24). Owner: Software Architect (Compasso). Contract version **5.3.0** (additive), registry **1.4.0**, on top of 5.2.0 (`dc89d1b`). It freezes together with ADR-0004 r5 (Esquadro, Crivo Q3).
- **Contract:** [`contracts/http/modules/accounts.yaml`](../../../contracts/http/modules/accounts.yaml) (`AccountSync`, `clearAccountSync`), `routing.yaml` (`routing.connection_checking`), `audit.yaml` (`sync.*`, `DetailsSync`), [`registry.json`](../../../contracts/notifications/v1/registry.json).
- **Inputs:**
  - slice plan V1-S11 AC-1…AC-7;
  - ADR-0004 §6 / §6.1 (r4);
  - design spec `v1-s11-out-of-sync.md` rev. 1 (Aquarela), Q-S11-1…4.

## 1. Operations

| Operation | Method and path | Notes |
|---|---|---|
| `getAccount` / `listAccounts` | existing | New required `sync` (`AccountSync`); changes arrive as `account.changed` |
| `clearAccountSync` | `POST /api/v1/accounts/{account_id}/sync/clear` | Body `{expected_version, step_up}`; 409 `accounts.sync_state_changed` / `accounts.not_out_of_sync` / `accounts.sync_not_observable`; replay of a successful clear → 200; step-up `account_sync_clear` (Q95); audited `sync.cleared` |

Cross-user access answers 404, as for every account operation. The double-submit guard is `expected_version` rather than an `Idempotency-Key`. A second click carries the same version: it either replays the clear that succeeded, or finds a newer reason and gets 409. So a reason the user has not seen is never cleared.

## 2. State model

- `state` is `in_sync` or `out_of_sync`. It is sticky until the user clears it (Q14).
- `reasons[]` holds the active causes, most severe first, in `SyncReasonCode` enum order.
- `resolved_reasons[]` holds the causes the Core removed while the account stays out of sync (ADR-0004 §6.1, "the reason is removed").
- `in_sync` ⇔ both lists empty and no `flagged_at` (schema-enforced).
- `version` bumps on every change and is the clear's precondition. It is not part of `row_version`, since the user never edits it.
- `observable: false` drives the "Integridade do bracket desconhecida" badge (S11-03, AC-5 iv). It applies while:
  - the adapter session is down;
  - the broker connection is lost;
  - the reconnect check has not finished.
- `SyncReason.core_action` is "what All Win did", exactly as done:
  - `orphan_orders_cancelled` lists the cancelled legs (AC-5 i);
  - `orphan_position_flattened` gives the quantity and side (AC-5 iii).
- The platform's native text is never included (S11-01 "Rejected order": the stable reason only).

## 3. Reason codes

`SyncReasonCode` covers every ADR-0004 §6 trigger: 21 codes (`command_unknown` added for a status `unknown`, and `flatten_failed_unprotected` for U19), as canonized by Esquadro (2026-10-23, ADR-0004 r5 "sync reason" column, protocol `reason-codes.md`). The protocol field stays open (`$defs/code`). HTTP uses a closed enum, and clients tolerate an unknown code. A new code is a minor version in both. The display order is the enum order. Renames from the proposal: `partial_fill` → `command_partially_filled`, `disconnected` → `disconnected_in_trade` (already in the protocol fixture `welcome-out-of-sync-with-reasons.json`).

`cancel_failed` is only for a cancel the adapter started by itself (orphan protection). A failed cancel inside a command is `command_rejected`, with the adapter's code in `adapter_reason_code`. `command.position_changed` maps to `position_changed`. `snapshot_trigger` tells whether a `snapshot_mismatch` came from a reconnect or a periodic snapshot. `expired_by` tells whether the Core or the adapter expired the command.

A `command.nothing_to_reduce` against a non-flat expected position is `position_changed`. Activity on an unbound platform account is not a sync reason: it is the audit `execution.unbound_activity` plus a warning notification, with target the connection.

## 4. Route status: `routing.connection_checking`

It is route status only, like `connection_disconnected`: commands wait and nothing is skipped (S11-03). The window is per mapped account, from `welcome` until the Core has evaluated the reconnect snapshot and the replayed reports (ADR-0004 §4 step 3, §6.1 r5), or until the session ends:
- no command is sent to the account;
- entries keep their TTL and expire if the window outlasts it;
- exits wait.

Bounds (Crivo Q2, Esquadro): after 60 s, an alert, and the route shows the elapsed time. After 5 min, the Core closes the session (4000 `snapshot_timeout`) and the adapter reconnects; that is a normal disconnect. The values are provisional, for Ferro and Pulso to tune.

Route status `broker_disconnected` (r5 §6.3): a snapshot reports `connected: false`. Delivery is held, so entries expire and exits wait, until the next `connected: true` snapshot. It never skips anything.

Route status `binding_pending` (r5 R3): the account was bound while its session was open, so the binding is active from the next handshake. It never skips anything; entries expire by their TTL and exits wait. The web says "reconecte o plugin para ativar o vínculo".

## 4a. Clear and re-flag (ADR-0004 r5)

- The clear empties both lists and bumps `version`.
- The expected state's base becomes the last evaluated snapshot: position and All Win orders as reported.
- After a clear, only the normal triggers flag again:
  - a new difference against that base → `snapshot_mismatch`;
  - a new event → its own code.
- An accepted condition that stays the same never re-flags, so there is no loop.
- A clear is refused while the account is not observable: 409 `accounts.sync_not_observable` (Q83 a, user 2026-10-24). A clear with an open position is allowed, with a UI confirmation (Q84 a).

## 4b. Unbound accounts, overflow and the U19 code (review of 5.3.0)

- **Clear order (Crivo R1).** The order is 404 → replay → step-up → 409. A double submit gets 200 without consuming a code.
- **Live protection (Vigia L2).** The dialog lists `AccountSync.positions` with their protection. An open position without full protection keeps or raises `sync.bracket_incomplete`, so the account stays out of sync after the clear.

- **Unbound account.** An out-of-sync account without a binding cannot be cleared. The web says "vincule a conta (ou re-pareie) para verificar e liberar". A rebind keeps the reasons; the first evaluated snapshot makes the account observable and becomes the base, and a difference adds `snapshot_mismatch`. Archiving is the other way out.
- **Overflow.** The Core keeps every reason. The response shows the 32 most severe of each list, with `reasons_total` / `resolved_total`; unprotected-exposure reasons are never left out. The web shows "e mais N" from `reasons_total`, because the clear covers the hidden reasons too (Crivo f3). `sync.cleared` carries the distinct codes and `cleared_count`.
- **U19.** `sync.flatten_failed_unprotected` is critical, for a flatten of any origin that failed after cancelling protection. It is the 21st code, confirmed by Esquadro (ADR-0004 r5). The gateway produces it from the flatten's report and the protection it cancelled.
- **Unbound activity.** One audit entry per (connection, platform account, instrument) position change, with fills not one by one, and a notification grouped per connection and trading day.

## 5. Audit and notifications

- Audit (category `connection`, `DetailsSync`):
  - `sync.flagged`: one per added reason;
  - `sync.reason_resolved`;
  - `sync.cleared` (user, `cleared_reasons`);
  - `sync.restored`: a flat disconnect back to normal, never flagged.
- No new audit category: `AuditCategory` is also the notification `category` in protocol 1.3, and a new value would be a protocol change.
- Registry 1.4.0 (target `account`, params `DetailsSync`). The severity is fixed per code, so the three bracket cases get codes of their own, and each replaces `sync.flagged` for its occurrence:

| Code | Severity | Grouping | Case |
|---|---|---|---|
| `sync.flagged` | danger | per account and trading day, reopens | any other reason added |
| `sync.bracket_orphan_leg` | warning | none | AC-5 i, orphan legs cancelled |
| `sync.bracket_incomplete` | danger | none | AC-5 ii, unprotected position or unknown orders |
| `sync.bracket_double_fill` | critical | none | AC-5 iii, orphan position flattened |
| `sync.restored` | info | none | ADR-0004 §6 row 3 |

- `sync.cleared` is audit only (S11-04).
- Entries skipped while out of sync remain `routing.out_of_sync` (S7).

## 6. Open questions

- **Esquadro:** answered 2026-10-23 (§3, §4, §4a; ADR-0004 r5 to follow). The `connection` category for `sync.*` is accepted for V1. `late_ms` was added here (T1 late exits, Nanquim).
- **User, 2026-10-24:** Q83 (a) no clear while not observable (409 `accounts.sync_not_observable`); Q84 (a) clear with an open position allowed, with a confirmation.
- **Lastro:** answered 2026-10-23. It goes into data model rev. 16.6 with Bigorna's #9:
  - `version` is the clear's `WHERE version = $expected`;
  - reason rows carry an id, a dedupe key (code + instrument or command, unique while active) and `resolved_at`;
  - `observable` is derived, never stored;
  - `sync.*` stays in category `connection`, so there is no audit CHECK change;
  - resolved reasons survive a new flag until the clear.
- **Aquarela:** S11-01…05 mapped to §1–§5. The platform message row is dropped (never sent).
