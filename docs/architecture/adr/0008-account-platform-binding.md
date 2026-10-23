# ADR-0008 — Account ↔ platform account binding is owned by `accounts`

- **Status:** Accepted (2026-10-23, user approved). Architect decision within ADR-0001 §4.
- **Owner:** Software Architect.
- **Asked by:** Malha (S5 Core), through the Maestro.
- **Depends on:**
  - [ADR-0001](0001-technology-stack.md) §4;
  - [ADR-0004](0004-adapter-protocol-v1.md) §3 and §8 (`hello.accounts`, re-pairing, environment guard);
  - [ADR-0006](0006-derived-state-across-modules.md);
  - HTTP `modules/accounts.yaml` (`createAccount`, `updateAccount`, `archiveAccount`);
  - data model rev. 16 §5.2–§5.3 (Database Architect).

## 1. Context

Data model rev. 15 put the binding (`account_id`, `bound_at`) on `gateway.venue_account`, a table owned by `gateway`. `createAccount` / `updateAccount` belong to `accounts`, and they must answer synchronously, with nothing saved on refusal:
- 409 `accounts.platform_account_taken`;
- 422 `accounts.platform_account_unknown`;
- 422 `accounts.platform_account_kind_not_allowed`;
- 409 `accounts.binding_open_position` (Q38).

If `gateway` wrote the binding, `accounts` would need a cross-module command port (an ADR of its own) or an event. An event cannot give these synchronous answers.

## 2. Options

| Option | Trade-off |
|---|---|
| (A) `gateway` owns the binding; `accounts` calls a `bind` command port | A cross-module command and two transactions, so "nothing saved" fails when one commits and the other does not |
| (B) `gateway` owns it; `accounts` emits an event | Asynchronous: the 409/422 answers become impossible |
| **(C) `accounts` owns the binding as an opaque id; `gateway` owns the platform facts** | One transaction, one writer. `accounts` stores no platform name (X-1) |

## 3. Decision

**(C).**

1. **Schema:**
   - `accounts.account` gains `venue_account_id uuid NULL` (gateway's id, no cross-schema FK, like `connection_id`) and `bound_at`, with `CHECK ((venue_account_id IS NULL) = (bound_at IS NULL))`.
   - A unique partial index on `venue_account_id` (`WHERE venue_account_id IS NOT NULL`) keeps "one platform account ↔ at most one All Win account".
   - `gateway.venue_account` keeps only the platform facts: connection, `platform_account`, `kind`, `oco_location`, first/last seen. It loses `account_id`, `bound_at`, `is_master` and `venue_account_bound_key`. Migration #9 does not exist yet, so nothing is migrated. `is_master` is decided when S19 opens.
2. **Write path (one `accounts` transaction):**
   1. The request keeps `binding {connection_id, platform_account}`. **The HTTP contract does not change.**
   2. `accounts` calls the gateway query port `VenueAccounts::resolve(user_id, connection_id, platform_account) -> Option<{venue_account_id, kind}>`.
      - `None` gives 422 `accounts.platform_account_unknown`. This covers no such account, another user's connection and an unpaired connection, which all answer the same way.
      - `kind` `live`/`unknown` outside production gives 422 `accounts.platform_account_kind_not_allowed`. The environment comes from injected configuration.
      - `accounts` passes the string through and never stores or interprets it.
   3. A unique-index violation gives 409 `accounts.platform_account_taken`. It can only concern the caller's own accounts, because `resolve` is scoped by `user_id`.
   4. A change or removal of a binding first asks `Positions::has_open_position(current venue_account_id)`, a query port. An open position gives 409 `accounts.binding_open_position`.
      - A position that opens between this read and the commit is an accepted window. Q38 covers open positions only, not working orders, so an order already routed can also fill after an unbind.
      - **Residual:** in either case the platform account holds a position with no binding. Routing then skips every action for it, exits included, with `routing.platform_unbound` (§3.5), so the position is no longer managed by All Win.
      - **Mitigation:** when `gateway` receives a fill or a non-flat `position_snapshot` for a venue account that `account_for_venue` reports as unbound, it writes an audit entry and alerts the user, so the position can be closed on the platform or the account rebound. The code is defined in the S11 sync contract. The plugin's own enforcement is not affected.
   5. The same transaction bumps `limits_version`, because the binding is a limits input (ADR-0006 §3.1). It writes two events to `app.outbox`:
      - `account.limits_changed {account_id, limits_version}`, the ADR-0006 producer event that `risk` consumes, so entries leave `routing.limits_pending` as soon as risk re-derives;
      - `account.binding_changed {account_id, venue_account_id | null, limits_version}`, which `gateway` consumes for side effects, for example sending `welcome`/`limits` again.
3. **Archive releases the binding:**
   - `CHECK (archived_at IS NULL OR venue_account_id IS NULL)`. The archive transaction nulls the binding, bumps `limits_version` and writes both events of §3.2 step 5. Otherwise an archived account would keep the platform account taken forever.
   - An open position already refuses the archive with 409 `accounts.delete_open_position`, so `archiveAccount` needs no new code.
4. **Read path:**
   - `gateway` and routing read the `accounts` query ports `binding_of(account_id)` and `account_for_venue(venue_account_id)` at the decision. There is no derived copy; if one is ever needed, it follows ADR-0006.
   - `VisibleAccount.account_id` is filled by `gateway` through `account_for_venue`.
   - The traits are defined on the consumer side and wired in `crates/app` (ADR-0006 §3.7, Ports).
5. **Lifetime of `venue_account` rows:**
   - `gateway` never deletes a `venue_account` row at runtime, and a revoked connection keeps its rows.
   - A reader that finds a bound id with no row treats the account as unbound. Routing skips every action for an unbound account with `routing.platform_unbound`. It applies to all actions, because nothing can be delivered. The code is added to the ADR-0003 §4.1 gates and the instruction reason-code registry in the next ADR-0003 delta.
   - **Re-pairing keeps `connection_id`** (`newPairingCode` on the same connection, ADR-0004 §8). The rows keyed by `(connection_id, platform_account)` survive, so the binding follows the connection by platform account name, as ADR-0004 §8 already states.
   - Only a new connection after an unpair produces new ids. The user rebinds then, which is intended: the old connection is history.
   - Because the binding follows the name, and names such as `Sim101` exist on every install, `gateway` writes an audit entry when the `kind` of a **bound** venue account changes on a `hello` (QA Security I1). Outside production, a `live` account is still protected by the delivery guard (§3.6), and in the plugin by the N11 consent (ADR-0005).
6. **The environment guard stays at delivery:** `kind` is refreshed on each `hello`. `gateway` still refuses delivery to a `live`/`unknown` account outside production (ADR-0004 §8). The check at bind time is a UX check, not the guard.

## 4. Consequences

- **Positive:**
  - synchronous errors and "nothing saved" hold with one writer per schema;
  - no cross-module command;
  - X-1 holds: `accounts` sees only an opaque id.
- **Negative:**
  - routing and `gateway` need a query-port read of the binding per decision;
  - the open-position check is not atomic with the commit, and working orders are not covered by Q38. A position left without a binding is no longer managed by All Win; it is audited and alerted (accepted, §3.2 step 4).
- **Tests:**
  - binding another user's platform account gives 422 `platform_account_unknown`, never 409;
  - binding the same platform account to two of the caller's accounts gives 409 on the second, and the first is unchanged;
  - an unbind with an open position gives 409 and nothing changes;
  - archive releases the binding, and the platform account can be bound again;
  - a binding change and an archive bump `limits_version` and emit `account.limits_changed`; entries are skipped with `routing.limits_pending` until risk re-derives, then route without waiting for the reconciliation;
  - a fill arriving for an unbound venue account is audited and alerts the user;
  - `live` kind outside production gives 422 at bind time, and delivery is still refused after a later `hello` changes the kind to `live`;
  - after re-pairing, the binding persists for a platform account with the same name.
- **Owners:**
  - Database Architect: data model rev. 16 §5.2–§5.3;
  - Backend S5 (Malha): write path. Its provisional `VenueAccounts` fake is query-only (`resolve`, `has_open_position`); there is no `bind`;
  - Backend gateway (S2/M1): `venue_account`, the query ports, the consumer;
  - Software Architect: this ADR, the ADR-0003 delta for `routing.platform_unbound`, and the S11 sync contract code for the unbound-fill alert and the bound-kind-change audit (§3.2 step 4, §3.5).
