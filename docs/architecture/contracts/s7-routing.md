# S7 — Routing HTTP contract notes

- **Status:** proposed (2026-10-23), for Code Review. Owner: Software Architect (Compasso). Contract version **2.2.0**, on top of 2.1.0 (S5).
- **Contract:** [`contracts/http/modules/routing.yaml`](../../../contracts/http/modules/routing.yaml).
- **Inputs:** slice-plan rev. 7, V1-S7 AC-1…AC-8 · design spec `v1-s7-routing.md` rev. 1 §10/§11 (PO answers Q-S7-1…6) · ADR-0003 §4.1 (gates) · [wave1-data-model.md](../data/wave1-data-model.md) §5.5 · [S5 notes](s5-accounts.md).

## 1. Operations

| Operation | Method and path | Notes |
|---|---|---|
| `listRoutes` | `GET /api/v1/routes?source_id=&account_id=` | Each route carries `effective` (status + ordered reasons), computed by the Core |
| `createRoute` | `POST /api/v1/routes` | Natural key (source, account); 409 `routing.route_exists` |
| `updateRoute` | `PUT /api/v1/routes/{route_id}` | `signal_types` + `is_enabled` + `row_version` (the switch sends all three) |
| `deleteRoute` | `DELETE /api/v1/routes/{route_id}` | |
| `listRouteCandidates` | `GET /api/v1/route-candidates?source_id=` | Picker: selectable or blocked-with-reasons, plus `manual_only` |

SSE: `route.changed` (create, update, delete). Status changes caused by the account, the opt-in, sync, risk or the connection arrive as `account.changed` / `connection.status_changed`; the client then refetches `listRoutes`.

Step-up (3.0.0, QA Security M1; Q35 decided 2026-10-23: setting `step_up.q35_persistence_actions` on, server configuration only, missing/invalid = on): `createRoute` / `updateRoute` need `step_up` (action `route_enable`, target = the account) when a route from an automated source is switched on for an account whose automation was not accepted with a step-up (a real account, or an opt-in that is `not_required`). Otherwise a stolen session could start automated trading without a fresh code. Follower routes with the copy choice (`account_opt_in_accept`, target = the follower) come with S19 (V1.1).

## 2. Decisions

1. **Account facts are not repeated** on routes or candidates. The page calls `listAccounts` (S5) once and joins by `account_id`, which is the same payload the S5 list uses (spec §10).
2. **Effective status:** `status` ∈ `receiving` / `exits_only` / `not_receiving`, plus `reasons[]`, most severe first. Each reason has `blocks` ∈ `all_actions` / `entries_only` (S7 AC-8, P1: out-of-sync and risk never block exits). That lets the UI say "exits still go through" without knowing the gate rules.
3. **Reason codes (Q-S7-6), all `routing.*`:**
   - `account_inactive` (+ `lifecycle_state`);
   - `source_revoked`;
   - `platform_unbound` (Q-S5-13: a routing reason, not an S5 write error);
   - `automation_not_accepted` / `copy_not_accepted` (+ `opt_in_state` ∈ `declined` / `pending_review`);
   - `connection_disconnected`;
   - `out_of_sync`;
   - `risk_blocked` (from S14);
   - `no_bracket_profile` (S6, Q43; + `instrument_ids`: profiles are per account × instrument);
   - `limits_pending` (ADR-0006: the account's limits are not yet derived; entries only);
   - `route_disabled`.

   `blocks` is fixed per code: `entries_only` for `out_of_sync`, `risk_blocked`, `no_bracket_profile` and `limits_pending`; `all_actions` otherwise. `connection_disconnected` covers every connection status other than `connected`.

   The skip audit uses the gate codes of ADR-0003 §4.1. The enum may grow in a minor version, and clients show a generic text for an unknown code.
4. **What blocks creation:** only `automation_not_accepted` (automated source) and `platform_unbound` (Q37). Every other reason, paused accounts included, only affects `status_if_routed`. Manual-only accounts are never candidates (S7 AC-7b) and are listed apart, with the preset's reason (empty before S16).
5. **Inactive routes are kept (Q-S7-2).** A route whose account declined automation stays saved, with its reason.
   - Q67 (b, decided; 3.5.0): switching on is refused with 422 while the account is ineligible (`/is_enabled`), and the Core switches the routes off when their account becomes ineligible (audited `routing.subscription_changed`, cause `account_ineligible`). They stay off when it becomes eligible again, until the user switches them on.
6. **Live status (3.0.0, S7-1):** `route.changed` (`cause: status`) is sent whenever any route's effective status changes, whatever the cause.
7. **Revoked sources (Q-S7-3):** their routes are listed and are read-only. `updateRoute` and `deleteRoute` get 409 `routing.source_revoked`.
8. **Disconnected connection (Q-S7-5):** configuration is allowed; the status shows `routing.connection_disconnected`.
9. **Signal types:** `trigger`, `trade_plan`, `nq_trigger`, at least one, none preselected (Q-S7-1). An empty list is a schema error: 400 `common.invalid_request`, pointer `/signal_types`. `master_account` is not accepted here (S19).
10. **Isolation:** a foreign `source_id` / `account_id` in a body gets the same 422 as an unknown one (`routing.source_unknown` / `routing.account_unknown`). In a query filter it gives an empty list (`listRoutes`) or 404 (`listRouteCandidates`).

## 3. Error codes

| Code | Status | Pointer | Operations |
|---|---|---|---|
| `routing.route_exists` | 409 | — | create |
| `common.version_conflict` | 409 | — | update |
| `routing.source_revoked` | 409 / 422 | — / `/source_id` | update, delete / create |
| `routing.source_unknown` | 422 | `/source_id` | create |
| `routing.account_unknown` | 422 | `/account_id` | create |
| `routing.account_manual_only` | 422 | `/account_id` | create |
| `routing.automation_not_accepted` | 422 | `/account_id` | create |
| `routing.platform_unbound` | 422 | `/account_id` | create (Q37) |

## 4. Audit

`routing.subscription_changed` (category `config`), with `action` ∈ `created` / `updated` / `enabled` / `disabled` / `removed`, the previous and new signal types and enabled flag, and the user. Routes removed by `archiveAccount` are audited the same way. Skips use the ADR-0003 §4.1 codes. Rendering belongs to S10.

## 5. Data-model deltas (for the Database Architect, through the Maestro)

- `routing.subscription.row_version integer NOT NULL DEFAULT 1` (optimistic concurrency).
- `signal_types` for these routes are limited to `trigger`, `trade_plan`, `nq_trigger`; `master_account` stays for S19 follower subscriptions.

## 6. 4.0.0 (written as 3.5.0)

- Q67 (b) is decided and active (see §2.5).
- Q35 is decided with the setting on: the `route_enable` step-up is required.

## 7. Implementation clarifications (Ferro, S7 Core; no contract change)

- **Display order of `RouteReasonCode`** (`effective.reasons[]`, most severe first), extending F§7.2 (out of sync → disconnected → blocked → automation pending → paused/disabled) with the route-level states first: `source_revoked`, `route_disabled`, `platform_unbound`, `out_of_sync`, `connection_disconnected`, `risk_blocked`, `limits_pending`, `no_bracket_profile`, `automation_not_accepted`, `copy_not_accepted`, `account_inactive`. This goes into the `RouteReasonCode` description at the next contract touch. It is a display order only: the audit code of a skipped instruction is the first gate that fails, in ADR-0003 §4.1 order.
- **Gate order (ADR-0003 §4.1; confirmed by Esquadro for his next ADR-0003 delta, with `platform_unbound` from ADR-0008):** gates for all actions first, then entry gates; the audited code is the first gate that fails. `account_inactive` → `platform_unbound` → `automation_not_accepted` → `copy_not_accepted` → `signal_type_not_selected` → `out_of_sync` → `risk_blocked` → `limits_pending` → `no_bracket_profile` → action rules (§4.2) → `expired`.
  - `source_revoked` and `route_disabled` are route status only, not instruction gates: a revoked token never passes ingress, and a disabled route yields no candidate.
  - `connection_disconnected` does not skip the instruction: the command is created and waits. An entry's command expires (`routing.expired`) if its TTL runs out before delivery (never execute late); exits (close, reduce, cancel, a protective modify, the close leg of a reverse) carry no expiry, wait, and execute late, audited as late (ADR-0003 §4.1, ADR-0004 T1). `expired` is an entries-only gate.
  - `limits_pending` and `no_bracket_profile` apply to position-increasing actions only; an action that cannot be classified counts as an entry (ADR-0006 §3.3).
  - ~~A `reverse` stopped by an entry gate is skipped whole.~~ Superseded by Q74 (b): see §8.
- **`reverse` with no room:** the contract has `max_position_qty ≥ 1`. If a derived limit (S13/S14) leaves `M = 0`, the flatten runs and cancels the protections, and the open of `min(Q, M) = 0` is skipped with `routing.max_position_reached`. The decision outcome is `capped` with `executed_qty = 0` for the open, and the flatten is audited as usual.
- **Expected position** is owned by the gateway, which recomputes it from execution reports (`position_after`, data model §6.3). Routing reads it once per decision through the query port `PositionFor(account_id, instrument)`. The port returns the signed position and the position identity (`position_ref` from U13, or unknown), so a `reduce` can resolve to `routing.position_unknown` without a second read. The trait lives on the routing side and the adapter in `crates/app` (ADR-0006, Ports). `routing_decision.position_before` is the value read. Trades (S21) only derive reporting from fills.

## 8. 5.0.0

- The `RouteReasonCode` description carries the display order and the gate order of §7 (Esquadro's ADR-0003 delta). `routing.platform_unbound` is now an audit code.
- **Q74 (b, user)** replaces "skipped whole" in §7. For a `reverse` the ADR-0003 §4.2 position check runs before the entry gates. On an account barred for entries:
  - position opposite → the close leg runs (flatten + cancel protections) and the open is skipped with the gate code: outcome `capped` (audit `partial`), `reverse_closed_qty`, notification `routing.reverse_close_only`;
  - flat → skipped with the gate code;
  - already on the reverse side → `routing.position_conflict` (rejected and notified, never a flatten).

  A reverse never opens on a barred account.

## 9. 5.1.0

- An open leg of a `reverse` with an opposite position that expires **at routing** (the decision sees the TTL exceeded, e.g. a late webhook) closes only: the close leg runs (exits never expire), and the open is skipped with `routing.expired`. Outcome `capped` (audit `partial`), `reverse_closed_qty`, notification `routing.reverse_close_only` with `reason_code: routing.expired`. Flat → outcome `expired`. Already on the reverse side → `routing.position_conflict`, since the position check comes first. ADR-0003 gets the same sentence in Esquadro's next delta.
- An open leg that expires **later, in the gateway queue** (connection down, close executed on reconnect) is not a routing decision. The decision stays `routed`, and ADR-0004 §5 applies, including "Exception: the opening leg of a reverse" (5.1.1, ADR-0004 r3). The open is held until the flatten in its `depends_on` is terminal. If it then expires, it becomes `expired` and is audited as `execution.expired`. It does **not** flag the account out of sync only when all of these hold:
  - it never left the Core (`pending`, never sent);
  - the flatten succeeded with a proven flat position: `already_flat`, or for `filled` the latest report with `position_after` shows 0 and no later report exists;
  - the account has no other sync reason.

  Then the user gets `routing.reverse_close_only` with `reason_code: routing.expired`. Any other case flags out of sync as before.
- `reverse_closed_qty` / `closed_qty` = abs(position) read at the decision: the quantity the close leg is sent to flatten. The executed result is in the execution entries.
