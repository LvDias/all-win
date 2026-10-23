# ADR-0003 — Standard instruction contract v1 and correlation identifiers

- **Status:** Accepted (2026-10-23) — accepted by the user with answers P1–P5 (§10). Acceptance deltas: §4.1 automation and copy opt-in gates (option A, Q24–Q26) and per-(source, account) signal filter; §7 token format; §5 command step codes
- **Owner:** Software Architect
- **Answers:** product-vision.md §6 (draft fields) and §13 open question #2 (identifiers for modify/cancel and master replication).
- **Implements user decisions:** A3 (secret in the URL, masked in every log), Q1 (conflict semantics), Q5 (followers' stop/target from their own profile), Q14 (out-of-sync gate), Q16 (command TTL; protocol details in ADR-0004).
- **Canonical artifacts:** [`contracts/instruction/v1/instruction.schema.json`](../../../contracts/instruction/v1/instruction.schema.json), fixtures in `contracts/instruction/v1/examples/{valid,invalid}/`, [`contracts/instruction/v1/README.md`](../../../contracts/instruction/v1/README.md).
- **Consumers:**
  - Core: `routing` intake, `core/adapters/tradingview`, and `gateway` for master-account input;
  - NinjaTrader plugin: master-account input;
  - web: read-only display;
  - `integrations/tradingview` alert templates.

## 1. Context

Every Input adapter must hand the Core **one** standard instruction (vision §5–6). The principle is that **the instruction states exactly what must happen**: the Core does not infer intent. V1 needs:

- TradingView signals: `buy`, `sell`, `close`, `reverse`;
- master-account replication (S19/S20), which additionally needs partial exits, non-market orders, stop/target moves and cancellations correlated to the source's orders.

The contract has to be fixed now so that S1 (webhook ingest), S2 (plugin execution) and later S19/S20 can be built in parallel without breaking changes.

## 2. Decision overview

- **JSON Schema draft 2020-12** in `contracts/instruction/v1/`. It is structural, source-neutral, and shared by every language through fixtures (ADR-0002 §4).
- **Strict envelope.**
  - `additionalProperties: false` at every level except `details`, so typos and credentials are rejected.
  - **Decimals are strings.** Quantities are integers.
  - **There are no stop/target fields:** stop and target always come from the target account's profile (vision §8).
- **Two additions to the vision draft**, both needed by decided product behavior:
  - `position_effect` (`open` | `reduce`) on `buy`/`sell`. It distinguishes "add in this direction" from "reduce the opposite position without flipping". Master partial exits need it, and it keeps the Q1 rule unambiguous.
  - `side` (`long` | `short`) on `reverse`, so that "reverse while flat opens the position" (Q1) has a direction.
- **Correlation identifiers** in an optional `refs` object, present from 1.0 so S19/S20 need no breaking change.
- **Source-specific rules are Core policy**, not schema (§6). Example: TradingView cannot send `modify`/`cancel` in V1.

## 3. Field reference

| Field | Type | Required | Notes |
|---|---|---|---|
| `schema_version` | string `1.<minor>` | yes | The Core accepts major 1 with any minor it supports; an unknown minor → `ingress.unsupported_version` |
| `message_id` | string, 1–128, `[A-Za-z0-9._:-]` | yes | Unique **per user**, forever. Dedupe key `(user_id, message_id)` |
| `sent_at` | RFC 3339 UTC, `Z` suffix | yes | Source emission time; used for lag and the expiry base (§7) |
| `source.type` | `[a-z][a-z0-9_]*` | yes | Must equal the kind bound to the credential. Known values: `tradingview`, `master_account`, `signal_engine` |
| `source.name`, `source.version` | string | no | Display and audit (indicator name/version, plugin instance/version) |
| `signal_type` | `trigger` \| `trade_plan` \| `nq_trigger` \| `master_account` | yes | Per-account signal selection (vision §7). New values are a minor bump |
| `action` | `buy` \| `sell` \| `close` \| `reverse` \| `modify` \| `cancel` | yes | See §4 |
| `position_effect` | `open` \| `reduce` | no (default `open`) | Only on `buy`/`sell` |
| `side` | `long` \| `short` | on `reverse` | Final side after the reversal |
| `instrument.symbol` | `[A-Z0-9]{1,10}` | yes | Neutral root symbol (`MNQ`). Adapters map it to platform symbols |
| `instrument.contract_month` | `YYYY-MM` | no | If omitted, the Core uses the catalog's active contract (§9, open item) |
| `quantity` | integer 1–1000 | on `buy`/`sell`/`reverse` | Base size before the account multiplier. **Ignored on `close`** (§4) |
| `order` | object | no (default market) | `type` (`market`/`limit`/`stop`/`stop_limit`), `role` (`entry`/`stop`/`target`), `limit_price`, `stop_price`, `time_in_force` (`day`/`gtc`) |
| `changes` | object | on `modify` | `price_delta` (signed non-zero decimal, price units) and/or `quantity` (new base quantity) |
| `refs` | object | on `modify`/`cancel` (`order_ref`) | `position_ref`, `order_ref`, `parent_order_ref`. See §5 |
| `reference_price` | decimal string > 0 | no | Informational only (audit, slippage); never prices an order in V1 |
| `details` | object (≤ 32 keys) | no | Free metadata (setup, zone, trigger); displayed, never used for routing |

The 1000 cap on `quantity` is a structural sanity bound, not a business limit. The account's maximum position size is the business limit.

## 4. Semantics, evaluated per target account (implements Q1)

For each target account selected by routing, the Core runs the steps below in order. Every outcome that is not an execution is **audited with a reason code and notified** (vision §11, transparency).

### 4.1 Account gates

| Gate | Applies to | Code |
|---|---|---|
| Account disabled or paused | all actions | `routing.account_inactive` |
| Automated source (`tradingview`, `signal_engine`) and the account's **automation** opt-in is required but not `chosen`/`accepted` (option A; required when the firm's automation stance is forbidden, conditional, unknown or not confirmed, Q25; `pending_review` after a stance/text change, Q26) | all actions from automated sources | `routing.automation_not_accepted` |
| Master-account source and the follower's **copy** opt-in is required but not `chosen`/`accepted` (Q24, same applicability and Q26 rule) | all actions from that master | `routing.copy_not_accepted` |
| Signal type not selected for this (source, account) subscription (Q-A4, stored as `routing.subscription`) | all actions | `routing.signal_type_not_selected` |
| Account flagged out of sync (Q14: stops until the user clears it) | position-increasing actions (`buy`/`sell` with `open`, `reverse`, `modify` of an entry) | `routing.out_of_sync` |
| Risk block (account reached a limit; also with enforcement off, Q2) | position-increasing actions | `routing.risk_blocked` |
| Command TTL would be exceeded before delivery | all actions | `routing.expired` (see ADR-0004) |

> **Decided (P1):** risk and out-of-sync gates do **not** block exits (`close`, `reduce`, `cancel`, `modify` of a protective order). Refusing an exit while at a limit or out of sync would increase exposure, and a flatten is safe regardless of the Core's view of the position. Paused/disabled accounts block everything, because that is an explicit user choice.

### 4.2 Actions

The following rules use `Q = quantity × account multiplier`, `P` = the account's current signed position in the contract, and `M` = the account's maximum position size.

| Action | Current position | Result |
|---|---|---|
| `buy`/`sell`, `open` | flat, or the same direction | Open/add `min(Q, M − abs(P))` with stop/target from the account profile (individual or shared mode). If capped, the excess is skipped (`routing.quantity_capped`); if there is no room, `routing.max_position_reached` |
| `buy`/`sell`, `open` | opposite direction | **Rejected and notified:** `routing.position_conflict` (Q1-a). The source must send `close` or `reverse` explicitly |
| `buy`/`sell`, `reduce` | opposite direction | Reduce by `min(Q, abs(P))`. **Never flips.** Protective orders are resized to the remaining position, oldest entries first in individual mode (P2, FIFO); at flat, orphan-order protection cancels the rest |
| `buy`/`sell`, `reduce` | flat or same direction | Skipped: `routing.nothing_to_reduce` |
| `close` | any non-flat | **Flatten the whole position** in that contract and cancel all its stops/targets (Q1). `quantity`, if present, is ignored and the audit records `quantity_ignored`. A misconfigured close template must never leave a position open |
| `close` | flat | No-op, audited: `routing.nothing_to_close` |
| `reverse`, `side` = X | opposite to X | Flatten, cancel protections, then open X with `min(Q, M)` and profile stop/target |
| `reverse`, `side` = X | flat | **Opens X** with `min(Q, M)` (Q1) |
| `reverse`, `side` = X | already X | **Rejected and notified:** `routing.position_conflict` |
| `modify` | — | Applies `changes` to every child order correlated to `refs.order_ref` on this account (§5). If there is none, `routing.no_correlated_order` (skipped, audited) |
| `cancel` | — | Cancels every working child order correlated to `refs.order_ref`. If there is none, `routing.no_correlated_order` |

**Repeated entries add to the position** (vision §6): two `buy` instructions with different `message_id`s both execute, up to `M`.

**Non-market orders:** `order.type` other than `market` creates a working order of the same type and price on the target account. Prices are copied as-is: same instrument and contract, no conversion. The plugin rounds them to tick size.

## 5. Identifiers and correlation (answers open question #2)

| Identifier | Scope / assigned by | Purpose |
|---|---|---|
| `message_id` | per user, by the source | Dedupe at ingress (`UNIQUE (user_id, message_id)`); a duplicate is acknowledged and never re-executed |
| `refs.position_ref` | per source, by the source | Groups the lifecycle of one source position, from flat to flat (audit, performance, replication grouping). The master plugin assigns a new value each time the master goes from flat to non-flat |
| `refs.order_ref` | per source, by the source | Stable id of one source order across its life. Required by policy on master-account `buy`/`sell`/`reverse`, and by schema on `modify`/`cancel`. The master plugin generates it (a GUID-based string) and maps it to the platform order internally; platform order ids can change and are never used as `order_ref` |
| `refs.parent_order_ref` | per source, by the source | Links a source protective order (`order.role` = `stop`/`target`) to the entry it protects. Used to find the follower protective orders that correspond to it |
| `instruction_id` | Core (UUIDv7) | Accepted instruction; returned by ingress, used in the audit trail |
| `command_id` | Core | One unit of work for one target account. **Deterministic:** UUIDv5 of `(instruction_id, target_account_id, step)`, so retries and restarts never create a second command for the same work (ADR-0004). `step` is one of a fixed list: `place`, `flatten`, `modify:<n>`, `cancel:<n>`, where `<n>` is the zero-based ordinal of that command kind in the routing plan for that (instruction, account) (ALIGN-6); and `orphan_flatten:<event_id>` for the Q-O1 (b) flatten of a position opened by an orphan bracket leg, where `event_id` is the execution report of that orphan fill. It keeps the id deterministic across Core retries and distinct from the instruction's own `flatten` (ALIGN-8) |
| `platform_order_id` | adapter | The venue's order id, reported in execution reports; stored, never sent back as a source reference |

**Correlation map (Core, `routing` module):**

`(user_id, source_id, order_ref) → [(target_account_id, command_id, child_role, platform_order_id?)]`

- `child_role` is one of `entry`, `stop`, `target`.
- A follower entry created from a source entry `E` is recorded under `E`.
- The follower's protective orders, built from **its own profile** (Q5), are recorded under `E` with role `stop`/`target`.

**Master replication rules** (contract level; S19/S20 refine the acceptance criteria):

- **Master protective order created:** not replicated. Followers already have profile-based protection (Q5-a).
- **Master protective order moved** (`modify`, `order.role` = `stop`/`target`, `changes.price_delta`, `refs.parent_order_ref` = `E`): every follower `stop`/`target` child under `E` moves by the same `price_delta` ("followers move theirs by the same amount"). In shared mode the follower's single position-level stop/target moves.
- **Master working entry modified** (`modify` with `refs.order_ref` = `E`): price moves by `price_delta`, and quantity becomes `changes.quantity × multiplier`, capped at `M`.
- **Master order cancelled** (`cancel` with `refs.order_ref`): the corresponding children are cancelled. Cancelling a master protective order therefore cancels the followers' corresponding protective orders, which may leave followers unprotected. This follows the decided rule "everything done on the master is replicated" and is confirmed (P3): each affected follower gets a warning notification.
- **Master partial exit:** `sell`/`buy` with `position_effect: reduce`. **Master flat:** `close`. Because `reduce` never flips and `close` on flat is a no-op, a follower whose own profile stop already fired cannot be pushed into an opposite position.

## 6. Source policies in V1 (Core, not schema)

| Policy | `tradingview` | `master_account` |
|---|---|---|
| Allowed actions | `buy`, `sell`, `close`, `reverse` | all |
| `modify`/`cancel` | **rejected:** `ingress.action_not_allowed_for_source` (Q1) | allowed |
| `position_effect: reduce` | rejected in V1 (`ingress.action_not_allowed_for_source`) | allowed |
| `order.type` | `market` only (or omitted) | any |
| `refs.order_ref` on `buy`/`sell`/`reverse` | optional | **required** (`ingress.missing_order_ref`) |
| `signal_type` | `trigger`, `trade_plan`, `nq_trigger`; anything else rejected (`ingress.signal_type_not_allowed_for_source`) | `master_account` |
| Transport | HTTPS webhook (§7) | Adapter protocol message `instruction` (ADR-0004), authenticated by the device token |

`source.type` must equal the type bound to the credential (`ingress.source_mismatch`). A master-account instruction is accepted only for an account the user has designated as a master. Followers are configured in routing (S7/S19).

## 7. TradingView ingress (implements A3)

- **URL:** `POST https://<host>/ingress/v1/tradingview/awk_<key_id>_<secret>`. Format (ALIGN-3): `key_id` = 12 characters `[A-Za-z0-9]`, `secret` = 43 characters base64url without padding (32 random bytes); full pattern `^awk_[A-Za-z0-9]{12}_[A-Za-z0-9_-]{43}$`.
  - The token is per source, shown once in the web app, and can be rotated and revoked.
  - **The body carries no credential.**
  - The secret part is **masked in every log** (Caddy, Core, audit). The requirements and the automated masking test are in ADR-0001 §5.
- **Body:** the instruction JSON, with `Content-Type` `application/json` or `text/plain` (TradingView picks one based on whether the message parses). Maximum 16 KiB, UTF-8.
- **Processing:** authenticate the token → parse → validate against the schema → apply source policy (§6) → insert with `UNIQUE (user_id, message_id)` → respond. Routing happens asynchronously after commit. The response budget is well under TradingView's ~3 s timeout.
- **Responses** (TradingView ignores them; they are for diagnostics and the "send test" tool):

| HTTP | Body | When |
|---|---|---|
| 200 | `{"status":"accepted","instruction_id":"…"}` | New instruction stored |
| 200 | `{"status":"duplicate","instruction_id":"…"}` | `message_id` already seen for this user; nothing re-executed |
| 400 | `{"status":"rejected","error":{"code":"ingress.schema_violation","pointer":"/quantity"}}` | Schema or JSON error (`ingress.invalid_json`, `ingress.schema_violation`, `ingress.unsupported_version`) |
| 422 | `{"status":"rejected","error":{"code":"ingress.action_not_allowed_for_source"}}` | Source policy (§6), `ingress.source_mismatch`, `ingress.missing_order_ref` |
| 404 | empty | Unknown, revoked or malformed token. Existence is not revealed. Logged by `key_id` only, when one can be parsed |
| 413 / 415 / 429 | `{"status":"rejected","error":{"code":"ingress.payload_too_large" \| "ingress.unsupported_media_type" \| "ingress.rate_limited"}}` | Limits |

  Every rejection with a valid token is written to the user's audit log, without the token, so the user sees *why* a signal did nothing.

- **Expiry base (input to Q16 TTL):** `expiry_base = min(sent_at, received_at)`. A delayed webhook expires just like a delayed delivery, and a future `sent_at` (clock skew) can never extend the TTL. A command's `expires_at = expiry_base + account TTL` (ADR-0004). This follows the Q16 rationale that a late entry is worse than a missed one (P4, decided).
- **`message_id` guidance for the Pine indicator** (see `integrations/tradingview`, S17): `tv-<symbol>-<signal kind>-<setup>-<bar time ms>`, built with `str.tostring(time)`. The id must differ between distinct signals on the same bar (include the setup or a sequence) and must be identical if the same alert fires twice for the same event.

## 8. Versioning and compatibility

- **Additive changes** (new optional field, new enum value) bump the minor version, e.g. `1.1`. The Core must accept every minor it knows. Producers must not send a minor newer than the Core supports (`ingress.unsupported_version`).
- **Breaking changes** create `contracts/instruction/v2/`. The Core then accepts both majors for a transition period announced to the user. The TradingView alerts and the plugin are updated by the user and the Plugin Engineer.
- Every change is recorded in `contracts/instruction/CHANGELOG.md`, and the Maestro notifies the affected owners (Backend Rust, Plugin Engineer, Frontend, Database Architect).

## 9. Data model implications (for the Database Architect)

These are inputs for `docs/architecture/data/` (owner: Database Architect):

- `instructions`:
  - `user_id`, `instruction_id` (UUIDv7), `message_id`, `source_id`, `key_id`;
  - `received_at`, `sent_at`, `expiry_base`, the raw validated JSON (`jsonb`), `status`;
  - `UNIQUE (user_id, message_id)`.
- Per-target routing outcomes: `instruction_id`, `target_account_id`, `outcome_code`, computed quantities, `command_id`.
- Correlation map: `user_id`, `source_id`, `order_ref`, `target_account_id`, `command_id`, `child_role`, `platform_order_id`.
- Catalog: instruments, holding root symbol, tick size, tick value and point value as exact decimals, sessions, and the **active contract month per instrument**, maintained globally by the admin with an advance-warning date (P5, decided).

## 10. User decisions on the open items (2026-10-23)

| # | Item | Decision |
|---|---|---|
| P1 | Do risk / out-of-sync gates block exits? | **No.** Exits (`close`, `reduce`, protective `modify`/`cancel`) always pass those two gates. Paused/disabled block everything |
| P2 | Partial exit in individual mode: which protective orders are reduced first? | **FIFO** (oldest entry first) |
| P3 | Master cancels a protective order | **Replicated**, with a warning notification on each affected follower |
| P4 | Expiry base | **`min(sent_at, received_at)`** |
| P5 | Active contract month / rollover | **Global rollover catalog maintained by the admin**, with advance warning to users. The Core switches the active contract on the configured date |

Related decisions used by this ADR: Q-A1 (one maximum position per account in V1), Q-A2 (stop/target profiles keyed by instrument), Q-A4 (signal filter per (source, account)), Q-N1 (dollar distances rounded toward the entry, see ADR-0004 §5), Q17 (several named TradingView sources per user, each with its own token), and the per-account opt-ins for automation and copying (§4.1; option A, Q24, Q25, Q26; HTTP contract `setAccountOptIn`).

## 11. Consequences

- **Positive:**
  - One contract for every source.
  - Every V1 action, including master replication, is expressible in 1.0, so S19/S20 need no breaking change.
  - Conflicting or invalid instructions fail loudly and visibly.
  - Credentials never appear in bodies.
- **Negative:**
  - `position_effect` and `side` are new to the vision draft. The TradingView alert templates (S17) must use them correctly.
  - Strict `additionalProperties` means every new field requires a schema release first, which is intended.
- **Reversibility:** additive evolution is cheap, and removal requires v2.

## 12. Adoption

1. ~~The user accepts this ADR.~~ Done 2026-10-23.
2. Backend Rust implements the DTOs in `core/crates/contracts` and the fixture tests. The Plugin Engineer does the same in `AllWin.Adapter.Core` (master input, S19). Frontend generates TS types (ADR-0002 §4).
3. S1 implements §7, including the masking test. S17 produces Pine templates conforming to §3 and §7.
