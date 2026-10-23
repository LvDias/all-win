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

Step-up (3.0.0, QA Security M1; superset pending the user with Q35, Core setting, default on): `createRoute` / `updateRoute` need `step_up` (action `route_enable`, target = the account) when a route from an automated source is switched on for an account whose automation was not accepted with a step-up (a real account, or an opt-in that is `not_required`). Otherwise a stolen session could start automated trading without a fresh code. Follower routes with the copy choice (`account_opt_in_accept`, target = the follower) come with S19 (V1.1).

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
   - `no_bracket_profile` (S6, Q43 provisional; + `instrument_ids`: profiles are per account × instrument);
   - `limits_pending` (ADR-0006: the account's limits are not yet derived; entries only);
   - `route_disabled`.

   `blocks` is fixed per code: `entries_only` for `out_of_sync`, `risk_blocked`, `no_bracket_profile` and `limits_pending`; `all_actions` otherwise. `connection_disconnected` covers every connection status other than `connected`.

   The skip audit uses the gate codes of ADR-0003 §4.1. The enum may grow in a minor version, and clients show a generic text for an unknown code.
4. **What blocks creation:** only `automation_not_accepted` (automated source) and `platform_unbound` (provisional, Q37). Every other reason, paused accounts included, only affects `status_if_routed`. Manual-only accounts are never candidates (S7 AC-7b) and are listed apart, with the preset's reason (empty before S16).
5. **Inactive routes are kept (Q-S7-2).** A route whose account declined automation stays saved, with its reason.
   - Turning it on is allowed today: it is saved, and its status still shows the reason.
   - Q67 (with the user), provisional option (b): switching on would be refused with 422 (reserved on `updateRoute`, `/is_enabled`), and the Core would switch routes off when their account becomes ineligible. Clients already handle that 422, so (b) needs no breaking change.
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
| `routing.platform_unbound` | 422 | `/account_id` | create (provisional, Q37) |

## 4. Audit

`routing.subscription_changed` (category `config`), with `action` ∈ `created` / `updated` / `enabled` / `disabled` / `removed`, the previous and new signal types and enabled flag, and the user. Routes removed by `archiveAccount` are audited the same way. Skips use the ADR-0003 §4.1 codes. Rendering belongs to S10.

## 5. Data-model deltas (for the Database Architect, through the Maestro)

- `routing.subscription.row_version integer NOT NULL DEFAULT 1` (optimistic concurrency).
- `signal_types` for these routes are limited to `trigger`, `trade_plan`, `nq_trigger`; `master_account` stays for S19 follower subscriptions.
