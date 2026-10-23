# Wave 1 data model — ingest (S1), identity (S3), accounts core (S5–S7)

- **Status:** Accepted (2026-10-24), rev. 16.8.1 (4fbbda5); **rev. 16.9 proposed** (Brasa #14 early review, Esquadro B5): the `outbox_notify` statement trigger and its LISTEN rule; worker grants narrowed (no `UPDATE` on `outbox_consumer`, column `UPDATE` on `outbox_delivery`), §6.4 and §8.
- Rev. 16.8.1 (4fbbda5) (Esmeril L1/L2/N3/N4, Cinzel): body hash without `sent_at` (§6.1); admin self-recovery is always and only break-glass, admin step-up without a session (`admin_recovery` action) (§4); two-key advisory publish lock (§5.7); `routing_decision` `ON CONFLICT` target (§6.2).
- Rev. 16.8 (208e920): S16 publication (Esquadro (B): `publish_seq`, advisory publish lock, `allwin_publisher`, release-file deltas, cursor table #11c) in §5.7; two-step reverse `leg` on `routing_decision` (§6.2); duplicate-with-different-body hash `…061250` (§6.1); S3 AC-12 admin recovery `…061800` (§4); §8 rows.
- Rev. 16.7 (1c93eb2; Bigorna): the `gateway.execution_report` `child_role` and `reason_code` `CHECK`s match the adapter protocol schema (`exit`, `flatten`; codes with one or more dots), §6.3.
- Rev. 16.6 (11dfba5): migration #9 `…061600` (Bigorna, gateway gaps G1–G8: stored `frame` for byte-identical retransmission, `undelivered`, `position_effect`, per-contract columns and `execution_report.seq`, `reason_code` CHECK, role split; no connection table per Esquadro B4 = b); S11 sync storage (`account_sync.version`, per-occurrence `account_sync_reason` with `dedupe_key` and `resolved_at`); §5.6 `risk.applied_fill` (#10b `…061510`), the #10 grants (DELETE only on `scheduled_limit`, worker column grants) and #10 `…061500`; §6.4 backfill alert note and indentation (Esmeril notes b, c); §6.5 and §8 rows.
- Rev. 16.4/16.5 (aa60b31; Brasa's access patterns, Esmeril): §6.4 rule 2 gets the DDL of `app.outbox_consumer` and `app.outbox_delivery` (per-consumer retries), registration, retirement, fetch and dead-letter rules; `published_at` is materialized as "done for all", so the purge needs no join; §8 #14 = `…061400`; §8 also lists `…060630` (#2c) and reserves `…061310` (#2d, `identity.source.name` CHECK, Crivo L1, needs user authorization); §6.1 lists the four ingress codes. **Rev. 16.5** (Esmeril on 16.4, Brasa aligned): a new `backfill` consumer resets `published_at` on retained rows; `outbox_delivery_event_idx` for the purge cascade; `app.outbox.dead_lettered_at` is legacy-only (#1b) and filtered in the fetch; L3 accepted for V1; cutover rules; delivery-consumer test instead of an FK; §8 row order.
- Rev. 16.3 (4cc8085; Esquadro, Crivo R1/R2): §6.4 rules 1–2 define completion per consumer (consumed, or before a `from_now` consumer's `start_seq` = first receivable `seq`), and state the order rule and the retention predicate.
- Rev. 16.2 (7acd73c): the outbox no-consumer allowlist (Esquadro, decision (b)) in §6.4 rule 1 and §6.5 retention; §8 versions `…060610` (`app.outbox.seq`) and `…060620` (#7b); a §9 list fix (Crivo nit on 16.1).
- Rev. 16.1 (1a634da; Crivo N1–N3 and ADR-0008 round 2): binding, change, release and archive write both `account.limits_changed` and `account.binding_changed`; the open-position residual and the gateway mitigation are mirrored in §5.3; §9 gains the rev. 16 constraint and grant tests.
- Rev. 16 (9dcc772, c509981):
  - **Binding (Esquadro, ADR-0008, approved by the user):** `accounts` owns the account ↔ platform-account binding. `accounts.account` gains `venue_account_id` and `bound_at`, and `gateway.venue_account` loses `account_id`, `bound_at` and `venue_account_bound_key` (§5.2, §5.3).
  - **Wave 1 versions assigned (§8):** old #5 split into a catalog file and an accounts file; `account_rule_value` (old #12) moves to S5; bracket profiles get their own file (S6).
  - **From the S5 migration review (Malha):** `display_name` 1..80 and `row_version ≥ 1` `CHECK`s on `account`; `DELETE` on `account` revoked from `allwin_app`; `opt_in_rule_version` is `SELECT`-only until S16; the Q13-b erasure order is stated (§5.2).
- Rev. 15.2 (e39d83b): protocol 1.3 shared fills. The trade's fill references become `trades.trade_fill` rows with the quantity and the allocated commission. Trade commission = Σ allocated shares (pro rata, 4 decimals half away from zero, remainder to the trade allocated last).
- Rev. 15.1 (65d257e): a lower `commission_per_contract_side` is scheduled like other looser inputs (Esquadro, ADR-0006 §3.7), and the adjusted-balance window is stated.
- Rev. 15 (8101a15) — user decisions of 2026-10-23: net P&L as the main value, risk on net, V1 evaluation target, U20 (a) commission floor. Rev. 15 (§5.6, §5.7, §6.3, §6.6):
  - per-fill effective commission with its source;
  - gross / commission / net on trades;
  - `commission_per_contract_side` and `evaluation_profit_target` as account rule fields, also in the preset;
  - `account.evaluation_start`;
  - the risk evaluation baseline, derived under ADR-0006.
- Rev. 14 (b45a02c, 6b4e8e3): Rev. 14: new `trades` schema with `trades.trade` (§6.6). Captured trades are immutable and only manual ones are editable (Q56, provisional); money is `NUMERIC`; prices are whole ticks (a Core rule). Q52–Q57 are not encoded in constraints.
- Rev. 13 (2ceab9a): Rev. 13:
  - turns §5.6 from a preview into the S13 schema: `risk.account_limits` (+ `effective_as_of`) aligned with the S16 value types, the derived `scheduled_limit`, `trading_day`, `drawdown_tracker` and `account_block`;
  - `account_rule_value` serves accounts without a preset (`set_by_user`), adds the fields `profit_target_on_reach` and `consistency_threshold_pct`, a per-field `saved_at` and an `is_unset` tombstone. Under ADR-0006 §3.7 (Q48), the inputs live in `accounts` and the looser-value schedule is derived in `risk`.
  - Q47, Q48 and Q49 are provisional and no constraint encodes them.
- Rev. 12 (1647af5): `audit_entry.actor` (`user` / `system` / `platform_admin`, HTTP 3.0.0 `AuditEntry.actor`), in #7b (§6.4).
- Rev. 11 (7b976d6): `one_time_code.action` gains `route_enable` and `account_binding_change` (HTTP 3.0.0, pending Q35), both targeted (target = account).
- Rev. 10 (49dbea0):
  - adds the preset catalog: `prop_firm`, `firm_release`, `firm_release_platform`, `firm_plan`, `preset`, `preset_version`, `preset_rule_set` and `firm_stance`;
  - adds `release_id` and the in-catalog FKs on `opt_in_rule_version`;
  - adds `accounts.account_rule_value`;
  - adds `account.limits_version` / `limits_changed_at` and `risk.account_limits.derived_from_version` (ADR-0006, option B; §5.7);
  - adds `app.outbox.seq` → `audit_entry.event_seq` for the S10 order (§6.4, agreed with Compasso for contract 3.0.0).
- Rev. 9 (83a4f52):
  - adds `outcome`, `is_exit`, `message_id`, `source_name` and `safe_to_display` to `audit.audit_entry`, in forward migration #7b (§6.4, §8);
  - widens the no-secret `CHECK` to the text snapshots;
  - **corrects rev. 8**: `account.*_rule_version_id` are plain `uuid`, not FKs to `catalog`, because the only cross-module FK is to `identity.app_user` (§1, rule 1).
- Rev. 8 (5dd2d45, 09cd281) adds:
  - on `accounts.account`: `firm_name`, `firm_platform`, `*_pending_reason`, the create idempotency key, and the preset-or-manual `CHECK` (§5.2);
  - `catalog.opt_in_rule_version`, with the generic versions seeded in S5 (§5.1);
  - `row_version`/`origin`/`copied_at`/`customized` on `account_bracket_profile` (§5.4);
  - `row_version` on `routing.subscription` (§5.5).
- Rev. 7.1: step-up `action` = the 8 `StepUpAction` values of HTTP 2.0.0, with `target_id` per action (D1); code-role mapping (D2); shared-browser device rule and `device_key_in_use` (C4). Rev. 7 aligns with HTTP contract 2.0.0: the channel change becomes an entity, `identity.channel_change`, with status, hold and cancellation, and the code roles become `current_value` / `new_value` / `other_channel` (§3, §4.1, §4.3, §6.5, §8, §9). Rev. 6 (Crivo-approved, 539d45e): passwordless sign-in replaces password + TOTP (user requirement change 2026-10-23, Q21/Q22 superseded). It removes `password_hash`, `totp_credential`, `recovery_code` and `password_reset`, and adds verified channels on `app_user`, `signup_code`, `one_time_code`, `code_send_counter` and `known_device` (§3, §4.1, §4.2, §4.3, §6.5, §8, §9). The account is created only when the signup e-mail code is confirmed (S3 AC-2). It ships as a forward migration #2b, `identity_passwordless` (§8). Rev. 5: outbox dispatcher rules per the Architect's decision (A) (§6.4). Rev. 4: outbox delivery attempts and dead-letter (§6.4, migration 1b, Code Review 5 C2). Rev. 3 applies Code Review 1 (Crivo/Esquadro) for HTTP contract 1.1.0: connection name/status, `admin_totp_reset`, copy opt-in (Q24). It also adds the S3 migration review deltas (invitation `locale`/`superseded_at`, `session.login_at`, owner read policies, `email_registered`/`admin_list_users`) and updates the §8 migration order: #1 is merged alone and first, #6 is split into 6/6b, and the cross-branch dependencies are listed. The user accepted Q-A1, Q-A2, Q-A4, Q-A5 and Q-N1 ([decisions.md](../../delivery/decisions.md)). The Architect accepted ALIGN-1…7, now in ADR-0003/0004 and the contracts. Rev. 2 applies those decisions and the adapter-protocol 1.0 acceptance deltas (§10). ALIGN-8 (orphan-fill command step) was accepted the same day. Still open: Q-A3 and the user's Q-O1. Preview sections are finalized by their own slices. Migrations #1, #1b, #2, #3, #4, #6 and #7 are DBA-approved, and their authors ran them on PostgreSQL 16. #8 is pending the startup status reset.
- **Owner:** Database Architect (Lastro @ Dev3)
- **Inputs:**
  - [product-vision.md](../../product/product-vision.md) rev. 4
  - [slice-plan.md](../../product/release-1/slice-plan.md) rev. 3
  - ADRs (Accepted): [0001](../adr/0001-technology-stack.md) §4–§8, [0002](../adr/0002-repository-structure.md)
  - ADRs (Accepted 2026-10-23): [0003](../adr/0003-instruction-contract-v1.md), [0004](../adr/0004-adapter-protocol-v1.md); `contracts/adapter-protocol/CHANGELOG.md` (1.0 acceptance deltas)
  - [v1-s5-automation-opt-in.md](../../design/specs/v1-s5-automation-opt-in.md) (design spec, proposed) for the automation opt-in fields
  - [auth-and-session-policy.md](../security/auth-and-session-policy.md) (Proposed)
  - [q13-audit-retention.md](q13-audit-retention.md)
- **Engine:** PostgreSQL ≥ 16 through sqlx. All SQL below is PostgreSQL 16 syntax. It is illustrative DDL that Backend Rust turns into migrations, and the DBA reviews those migrations.
- **Markers:**
  - **[ALIGN-n]** — item agreed with the Architect (§10); all eight are accepted.
  - **[Q-…]** — business question for the PO/user (§11); Q-A3 and Q-O1 are still open.
  - **Preview** — a table shown so keys and retention are consistent, finalized by its own slice.

## 1. Principles

1. **One PostgreSQL schema per Core module.** Schemas: `identity`, `catalog`, `accounts`, `routing`, `gateway`, `audit`, `risk`, `trades` (rev. 14); plus `app` for shared infrastructure (context function, outbox).
   - A module's `infra` crate reads and writes **only its own schema**.
   - Foreign keys stay inside a module. The only cross-module FK is to `identity.app_user(id)`, the stable owner key.
   - Other cross-module ids are plain `uuid` columns, resolved through query ports and events (ADR-0001 §4).
2. **Every user-owned row has `user_id`.** Isolation has two barriers: application scoping, then Row-Level Security (§3).
3. **References inside a module are composite:** `(user_id, x_id) → parent(user_id, id)`. The database itself refuses a row that points at another user's entity.
4. **Exact numbers:** `NUMERIC` with fixed precision and scale matching the wire contracts, and `integer` for contracts (§2).
5. **Time:** every instant is `timestamptz`. Trading days are `date` plus an IANA zone from the rule that defines them (X-8).
6. **Ids:** UUIDv7 generated by the Core (`uuid` crate), except the deterministic UUIDv5 `command_id` (ADR-0003 §5). There is no database default, because PostgreSQL 16/17 has no `uuidv7()`.
7. **Enumerations:** `text` + `CHECK (… IN (…))`, using the same snake_case codes as the contracts. No PostgreSQL `ENUM` types: enum values cannot be removed, and a newly added value cannot be used in the same transaction.
8. **Soft retirement** (`archived_at`) for configuration that history points to (accounts, sources, connections). History rows keep the id plus a display-name snapshot.
9. **Secrets never leave `identity`** (§4.2). No secret, hash or ciphertext goes into events, audit rows, logs or API responses.

## 2. Numeric types (exact decimals)

| Value kind | PostgreSQL | Rust newtype | Wire (contract) | Notes |
|---|---|---|---|---|
| **Price**: fill, average, limit/stop price, `reference_price`, tick size, distance in points, `price_delta` | `NUMERIC(22,10)` | `Price` / `Points` (`rust_decimal`) | string, ≤ 12 integer + ≤ 10 fraction digits (`$defs/price`) | Matches the contract exactly. Covers every CME tick (0.25; 1/128 = 0.0078125; 0.0000005). 22 digits fits `rust_decimal` and C# `decimal` (28) |
| **Money** (USD): P&L, commission, account size, limits, point/tick value, distance in dollars | `NUMERIC(18,4)` | `Money` | string, `$defs/money` / `non_negative_money`: ≤ 14 integer + ≤ 4 fraction digits (ALIGN-1). `NUMERIC(18,4)` holds exactly that range | 4 decimals hold exact tick values (ZN US$ 15.625, ZT US$ 7.8125). P&L = ticks × tick value, so it stays exact at 4 decimals. Display rounds to cents and never changes the stored value |
| **Contracts**: quantity, multiplier, max position, fill quantity | `integer` with `CHECK` (≥ 1, or ≥ 0 for "filled so far") | `Quantity`, `Multiplier` | integer | Whole numbers only (vision §8). Signed position is a signed `integer` |
| **Distance in ticks** | stored in the profile's `NUMERIC(22,10)` value column with a whole-number `CHECK`; `integer` where the column is only ticks | `Ticks` | integer (`stop_ticks`, `target_ticks`) | — |
| **Warning threshold** | `smallint` 1–99 | — | integer (`warning_threshold_pct`) | — |
| **Currency** | `char(3)`, `'USD'` in V1 | — | — | Explicit, so non-USD contracts need no rework |

Rules for the Core:

- **PostgreSQL silently rounds input to the column scale.** The Core must reject any value whose scale exceeds the column's, in the newtype constructors, and must never rely on the database to round.
- Every rounding is an explicit domain rule:
  - the plugin converts NinjaTrader `double` values at its boundary (ADR-0001 §7, ADR-0005); the Core never rounds silently;
  - the Core normalizes profile distances to whole ticks (ADR-0004 §5). A dollar distance that is not a whole number of ticks is rounded **toward the entry**, and the adjustment is audited (Q-N1, decided).
- Price-on-tick validation is a domain rule. A `CHECK` cannot read the instrument row.

## 3. Per-user isolation: application scoping + RLS

### Evaluation

| | Application scoping only | **Application scoping + RLS (recommended)** |
|---|---|---|
| Mechanism | Every repository method takes `UserId`; every statement filters `user_id` | Same, plus `ENABLE` and `FORCE ROW LEVEL SECURITY` with a policy `user_id = app.current_user_id()` on every user-owned table |
| A forgotten filter | Reads or changes another user's rows; caught only by tests/review | Zero rows / insert refused (**fail-closed**) |
| Runtime cost | none | One `set_config` per transaction (one extra statement, sub-millisecond on a local socket). Every user-data statement runs inside a transaction |
| Build cost | none | A role split, a few `SECURITY DEFINER` lookup functions, a CI lint test |
| Retrofit later | — | Expensive: every query path and test must adopt the context |

**Recommendation: both, from the first migration** (confirms ADR-0001 §8). In All Win a cross-user mistake is worse than a privacy leak: it could route one user's trade to another user's account. A fail-closed second barrier is worth its small cost.

### Mechanism

```sql
CREATE FUNCTION app.current_user_id() RETURNS uuid
  LANGUAGE sql STABLE PARALLEL SAFE
  AS $$ SELECT NULLIF(current_setting('app.user_id', true), '')::uuid $$;
-- NULLIF: once a session has set the GUC, it reverts to '' (not NULL) after the transaction.

ALTER TABLE accounts.account ENABLE ROW LEVEL SECURITY;
ALTER TABLE accounts.account FORCE  ROW LEVEL SECURITY;
CREATE POLICY account_owner ON accounts.account
  USING (user_id = app.current_user_id())
  WITH CHECK (user_id = app.current_user_id());
```

- **Every user-scoped transaction** starts with `SELECT set_config('app.user_id', $1, true)`. The setting is transaction-local, so it never leaks through the pool. If it is missing, policies evaluate to false.
- **Roles.** DevOps creates the cluster roles with per-environment secrets; migrations only `GRANT`.

| Role | Used by | Rights |
|---|---|---|
| `allwin_owner` | `sqlx migrate` | Owns all objects. Never used at runtime |
| `allwin_app` | Request handlers and per-user work | DML on module tables, `NOBYPASSRLS`. On `audit.audit_entry` only `INSERT`/`SELECT`. No DDL |
| `allwin_worker` | Loops that must **find** work across users: outbox dispatcher, command delivery, expiry sweeper, adapter session handler | `BYPASSRLS`, limited to `SELECT` + narrow `UPDATE` on queue tables (`app.outbox`, `gateway.command`, `gateway.outbound_message`); `SELECT` on `identity.channel_change` to find holds that are due (rev. 7). Each item's business work then runs on the `allwin_app` pool with that item's `user_id` set |
| `allwin_maint` | Partition creation, retention purge, erasure after account closure | `BYPASSRLS`; partition DDL on `audit.audit_entry`; `DELETE` on activity tables |

- **Pre-authentication lookups** (the user is not known yet) go through `SECURITY DEFINER` functions in `identity`. Each one returns only what its caller needs:
  - `identity.resolve_source_key(key_id)` → `(user_id, source_id, source_type, secret_hash, revoked)`; `revoked` is true when the key is revoked, the source is archived or the user is not `active`
  - `identity.resolve_device_token(key_id)` → `(user_id, connection_id, token_hash, revoked)`
  - `identity.resolve_session(token_hash)`
  - `identity.resolve_invitation(token_hash)`
  - `identity.find_user_for_login(email_normalized)` → `(id, status, locale, email, whatsapp_e164)`. The Core answers the same way for an unknown and a non-`active` user and sends nothing, so the login form is not an existence oracle
  - `identity.device_key_in_use(device_key_hash)` → boolean: a non-revoked `known_device` of any user has it (shared-browser rule, §4.1; rev. 7)
  - `identity.resolve_one_time_code(id)` → `(user_id, purpose)` only. The code row itself is then read and updated under RLS (§4.3)
  - `identity.email_registered(email_normalized)` → boolean (createInvitation). Gated like `admin_list_users`: it returns false unless `app.current_user_id()` is an active platform admin, so it is never an e-mail existence oracle
  - `identity.admin_list_users()`: returns rows only when `app.current_user_id()` is an active platform admin. Rev. 6: `totp_enabled` becomes `whatsapp_verified`, and `last_sign_in_at` is `max(session.login_at)` (no MFA filter) (HTTP contract 2.0.0)
  - `identity.resolve_pairing_code(code_hmacs bytea[])`: read-only, one HMAC per accepted pepper version. Redemption is a conditional `UPDATE … SET used_at = now() WHERE used_at IS NULL AND invalidated_at IS NULL AND expires_at > now()`, run in the user-scoped transaction that records the instance and swaps the device token. A concurrent second redemption updates 0 rows

  `allwin_app` may execute these functions but cannot read other users' rows.
- **FORCE RLS also binds the owner.** So each table read by a `SECURITY DEFINER` function gets a policy `FOR SELECT TO allwin_owner USING (true)`. `allwin_owner` never logs in at runtime, and this is narrower than giving it `BYPASSRLS`. Each function sets `search_path = pg_catalog, pg_temp` and uses schema-qualified names.
- **Migrations run as `allwin_owner`**, the login used by `sqlx migrate`. The default privileges of #1 apply only to objects that `allwin_owner` creates. A superuser run would leave new tables without the `allwin_app` grants.
- **Unscoped tables** (no RLS): `catalog.*` (global reference data), `identity.invitation` and its `identity.signup_code` (admin data; reached only through the invitation token, `resolve_invitation` and admin use cases) and `identity.code_send_counter` (rev. 6; no `user_id`, keyed by identifier HMAC so it covers unregistered identifiers).
- **CI lint test:** a query on `pg_class`/`pg_policy` asserts that every table with a `user_id` column has `relrowsecurity` and `relforcerowsecurity` set and at least one policy. The X-2 cross-user tests run with RLS on.

## 4. Identity (S3) and ingress sources (S1)

### 4.1 Tables

```sql
CREATE TABLE identity.app_user (
  id                   uuid PRIMARY KEY,
  email                text NOT NULL,                    -- as entered; fixed by the invitation at signup
  email_normalized     text NOT NULL,                    -- trimmed + lower-cased by the Core
  email_verified_at    timestamptz NOT NULL,             -- signup e-mail code; set again by an e-mail change
  whatsapp_e164        text CHECK (whatsapp_e164 ~ '^\+[1-9][0-9]{7,14}$'),  -- verified number only (E.164)
  whatsapp_verified_at timestamptz,                      -- added after signup by a channel change (S3b; S3 AC-3, Q31)
  locale               text NOT NULL DEFAULT 'pt-BR' CHECK (locale IN ('pt-BR','en')),
  time_zone            text NOT NULL,                    -- IANA, validated by the Core (Q23)
  is_platform_admin    boolean NOT NULL DEFAULT false,   -- issues invitations (Q19)
  status               text NOT NULL DEFAULT 'active' CHECK (status IN ('active','closed')),
  created_at           timestamptz NOT NULL DEFAULT now(),
  updated_at           timestamptz NOT NULL DEFAULT now(),
  closed_at            timestamptz,                      -- erase after grace (Q13-b)
  channel_changed_at   timestamptz,                      -- last e-mail/WhatsApp change; next one refused for 24 h (policy §7)
  CONSTRAINT app_user_email_normalized_key UNIQUE (email_normalized),
  CONSTRAINT app_user_whatsapp_key UNIQUE (whatsapp_e164),            -- one number, one user (PO Q32, open)
  CONSTRAINT app_user_whatsapp_check CHECK ((whatsapp_e164 IS NULL) = (whatsapp_verified_at IS NULL))
);  -- RLS policy: id = app.current_user_id()
-- No 'pending' status: the row is inserted only when the signup e-mail code is confirmed (S3 AC-2), so an
-- abandoned or repeated signup leaves no user row behind (§4.3).

CREATE TABLE identity.invitation (                       -- admin data, no RLS
  id               uuid PRIMARY KEY,
  email_normalized text NOT NULL,
  locale           text NOT NULL CHECK (locale IN ('pt-BR','en')),  -- language of the invitation e-mail/page
  token_hash       bytea NOT NULL UNIQUE,                -- SHA-256 of 256-bit token
  issued_by        uuid NOT NULL REFERENCES identity.app_user(id),
  created_at       timestamptz NOT NULL DEFAULT now(),
  expires_at       timestamptz NOT NULL,                 -- +7 days (security policy §3)
  accepted_at      timestamptz,
  accepted_user_id uuid REFERENCES identity.app_user(id),
  revoked_at       timestamptz,
  superseded_at    timestamptz,                          -- replaced by a new invitation to the same e-mail
  CHECK ((accepted_at IS NULL) = (accepted_user_id IS NULL))
);
CREATE UNIQUE INDEX invitation_open_email_key ON identity.invitation (email_normalized)
  WHERE accepted_at IS NULL AND revoked_at IS NULL AND superseded_at IS NULL;
-- Re-inviting after expiry sets superseded_at on the expired row in the same transaction, so the admin
-- list still shows it as expired, not revoked. An unexpired open invitation → 409 invites.email_taken (HTTP 1.1.0).

CREATE TABLE identity.known_device (                     -- browsers the user has signed in from
  id              uuid PRIMARY KEY,
  user_id         uuid NOT NULL REFERENCES identity.app_user(id),
  device_key_hash bytea NOT NULL CHECK (octet_length(device_key_hash) = 32),  -- SHA-256 of a 256-bit device cookie
  label           text CHECK (char_length(label) <= 120),    -- from the user agent, e.g. "Chrome on Windows"
  first_seen_at   timestamptz NOT NULL DEFAULT now(),
  last_seen_at    timestamptz NOT NULL DEFAULT now(),
  last_ip         inet,
  revoked_at      timestamptz,                           -- "forget this device": the next sign-in from it is new again
  UNIQUE (user_id, id),
  UNIQUE (user_id, device_key_hash)
);
-- The device cookie is long-lived, HttpOnly and separate from the session cookie, so logout or session
-- expiry does not make a known browser "new". At sign-in, with the presented cookie's hash:
--   * a non-revoked row of THIS user → known device (last_seen_at updated, no notice);
--   * valid only for ANOTHER user (shared browser, C4) → the value is kept and a row with the same hash is
--     inserted for this user (UNIQUE (user_id, device_key_hash) allows it);
--   * absent, unknown to every user, or revoked for this user ("forget device" / "sign out everywhere";
--     a revoked cookie counts as no cookie) → a NEW random value and a new row. The revoked row is never
--     reactivated and is kept, so the UNIQUE key holds.
-- Telling "another user's" from "unknown" crosses users: identity.device_key_in_use(hash) → boolean
-- (SECURITY DEFINER, §3). It reveals nothing the browser holding the cookie does not already have.
-- A new row emits the new-device e-mail notice through app.outbox in the same transaction (no secret in it).
-- Exception: the device created by signup completion sends no notice (S3 AC-9).

CREATE TABLE identity.session (
  id                  uuid PRIMARY KEY,
  user_id             uuid NOT NULL REFERENCES identity.app_user(id),
  device_id           uuid NOT NULL,
  token_hash          bytea NOT NULL UNIQUE,             -- SHA-256 of the cookie value
  is_persistent       boolean NOT NULL,                  -- "keep me signed in"
  login_channel       text NOT NULL CHECK (login_channel IN ('email','whatsapp')),
  created_at          timestamptz NOT NULL DEFAULT now(),
  login_at            timestamptz NOT NULL,              -- code sign-in; absolute-timeout base, kept across rotation
  last_activity_at    timestamptz NOT NULL,              -- user-initiated requests only (SSE excluded)
  idle_expires_at     timestamptz NOT NULL,
  absolute_expires_at timestamptz NOT NULL,
  ended_at            timestamptz,
  end_reason          text CHECK (end_reason IN ('logout','expired','rotated','channel_changed','device_revoked',
                                                 'signed_out_everywhere')),
  ip_address          inet,                              -- security; same retention as the activity record
  user_agent          text,
  UNIQUE (user_id, id),
  FOREIGN KEY (user_id, device_id) REFERENCES identity.known_device (user_id, id)
);
CREATE INDEX session_user_active_idx ON identity.session (user_id) WHERE ended_at IS NULL;
-- Removed: mfa_state (no second factor) and authenticated_at (the 15-minute step-up window is replaced by
-- a fresh code per sensitive action, §4.3). End reasons (policy §7): a channel change ends every other session
-- ('channel_changed') and rotates the current one ('rotated'); "forget device" ends that device's sessions
-- ('device_revoked'); "sign out everywhere" (S3, Q34) ends all sessions and revokes all devices.

CREATE TABLE identity.signup_code (                      -- signup challenges; no user exists yet. Admin-side data like invitation: no RLS
  id                 uuid PRIMARY KEY,                   -- challenge_id; never the code
  invitation_id      uuid NOT NULL REFERENCES identity.invitation(id),
  channel            text NOT NULL DEFAULT 'email' CHECK (channel = 'email'),  -- S3: e-mail only (AC-3); S3b/Q31 may widen it
  code_hmac          bytea NOT NULL CHECK (octet_length(code_hmac) = 32),  -- HMAC-SHA-256(k_code, challenge_id ‖ normalized code)
  pepper_version     smallint NOT NULL,
  failed_attempts    smallint NOT NULL DEFAULT 0 CHECK (failed_attempts BETWEEN 0 AND 5),
  created_at         timestamptz NOT NULL DEFAULT now(),
  sent_at            timestamptz NOT NULL DEFAULT now(),
  expires_at         timestamptz NOT NULL,
  consumed_at        timestamptz,
  invalidated_at     timestamptz,
  invalidated_reason text CHECK (invalidated_reason IN ('attempts','invitation_closed')),
  request_ip         inet,
  delivery_state     text NOT NULL DEFAULT 'pending' CHECK (delivery_state IN ('pending','sent','failed')),  -- §4.3 Delivery
  CONSTRAINT signup_code_expiry_check
    CHECK (sent_at >= created_at AND expires_at > sent_at AND expires_at <= sent_at + interval '10 minutes'),
  CONSTRAINT signup_code_invalidated_check CHECK ((invalidated_at IS NULL) = (invalidated_reason IS NULL)),
  CONSTRAINT signup_code_attempts_check CHECK (failed_attempts < 5 OR invalidated_reason = 'attempts'),
  CONSTRAINT signup_code_final_check CHECK (consumed_at IS NULL OR invalidated_at IS NULL)
);
CREATE INDEX signup_code_open_idx ON identity.signup_code (invitation_id, created_at)
  WHERE consumed_at IS NULL AND invalidated_at IS NULL;
-- The destination is the invitation's e-mail (Q36 a), so it is not copied here. Same lifecycle as one_time_code.

CREATE TABLE identity.channel_change (                   -- one e-mail/WhatsApp change (HTTP 2.0.0 ChannelChange; id = change_id)
  id                      uuid PRIMARY KEY,
  user_id                 uuid NOT NULL REFERENCES identity.app_user(id),
  channel                 text NOT NULL CHECK (channel IN ('email','whatsapp')),
  new_value               text NOT NULL,                 -- normalized; shown only masked; applied when completed
  requested_session_id    uuid NOT NULL,                 -- the session that started it (codes are bound to it)
  status                  text NOT NULL DEFAULT 'awaiting_codes'
                          CHECK (status IN ('awaiting_codes','on_hold','completed','cancelled','expired')),
  created_at              timestamptz NOT NULL DEFAULT now(),
  expires_at              timestamptz NOT NULL,          -- every code confirmed within 10 min (policy §7)
  hold_until              timestamptz,                   -- on_hold: effective time (policy §9 hold; §7 M2 if adopted)
  completed_at            timestamptz,
  cancelled_at            timestamptz,
  cancelled_by_session_id uuid,                          -- "Cancelar troca"; NULL = cancelled by the Core, generic reason (§4.3)
  UNIQUE (user_id, id),
  CONSTRAINT channel_change_whatsapp_check CHECK (channel <> 'whatsapp' OR new_value ~ '^\+[1-9][0-9]{7,14}$'),
  CONSTRAINT channel_change_expiry_check CHECK (expires_at > created_at AND expires_at <= created_at + interval '10 minutes'),
  CONSTRAINT channel_change_hold_check CHECK (status <> 'on_hold' OR hold_until IS NOT NULL),
  CONSTRAINT channel_change_completed_check CHECK ((status = 'completed') = (completed_at IS NOT NULL)),
  CONSTRAINT channel_change_cancelled_check CHECK ((status = 'cancelled') = (cancelled_at IS NOT NULL)),
  CONSTRAINT channel_change_cancelled_by_check CHECK (cancelled_by_session_id IS NULL OR cancelled_at IS NOT NULL),
  FOREIGN KEY (user_id, requested_session_id)    REFERENCES identity.session (user_id, id),
  FOREIGN KEY (user_id, cancelled_by_session_id) REFERENCES identity.session (user_id, id)
);
CREATE UNIQUE INDEX channel_change_one_open ON identity.channel_change (user_id)
  WHERE status IN ('awaiting_codes','on_hold');          -- one pending change per user (SecurityState.pending_channel_change)
CREATE INDEX channel_change_hold_due_idx ON identity.channel_change (hold_until) WHERE status = 'on_hold';
-- RLS: owner policy. allwin_worker SELECT only (finds due holds); the effect runs in the user's allwin_app transaction.

-- rev. 16.8, forward migration #2e `…061800` identity_admin_channel_recovery (Cinzel; S3 AC-12; expand-only):
ALTER TABLE identity.channel_change
  ADD COLUMN kind text NOT NULL DEFAULT 'user_change' CHECK (kind IN ('user_change','admin_recovery')),
  ALTER COLUMN requested_session_id DROP NOT NULL,
  ADD COLUMN admin_user_id    uuid,                      -- the admin who acted (no FK: erasure stays decoupled; audit has the actor)
  ADD COLUMN verified_by      text CHECK (verified_by IN ('video_call','phone_known_number','in_person','other')),
  ADD COLUMN verification_ref text CHECK (char_length(verification_ref) <= 200),  -- never the new value or a document number (Core)
  ADD COLUMN break_glass      boolean NOT NULL DEFAULT false,
  ADD CONSTRAINT channel_change_session_kind_check CHECK ((kind = 'user_change') = (requested_session_id IS NOT NULL)),
  ADD CONSTRAINT channel_change_kind_fields CHECK (
        (kind = 'admin_recovery' AND admin_user_id IS NOT NULL AND verified_by IS NOT NULL AND verification_ref IS NOT NULL)
     OR (kind = 'user_change' AND admin_user_id IS NULL AND verified_by IS NULL AND verification_ref IS NULL AND NOT break_glass)),
  ADD CONSTRAINT channel_change_break_glass_check CHECK (kind <> 'admin_recovery' OR break_glass = (admin_user_id = user_id)),  -- rev. 16.8.1: I3, self-recovery is always and only break-glass
  ADD CONSTRAINT channel_change_verification_ref_no_token CHECK (verification_ref !~ 'aw[kd]_[A-Za-z0-9]{12}_[A-Za-z0-9_-]{43}');
-- An admin_recovery goes through the same 10-min code phase and 72-h hold (ApplyDueChannelHolds). An open
-- user_change is first cancelled by the Core (cancelled_by_session_id NULL), so channel_change_one_open holds.
-- The command runs as allwin_app with app.user_id = the target (Esquadro); the owner policy covers it, no new grants.

CREATE TABLE identity.one_time_code (                    -- login, step-up and channel-change codes (signup: signup_code)
  id                 uuid PRIMARY KEY,                   -- uuid inside the sealed challenge_id token (§4.3); never the code
  user_id            uuid NOT NULL REFERENCES identity.app_user(id),
  channel_change_id  uuid,                              -- channel_change: the change this code confirms
  purpose            text NOT NULL CHECK (purpose IN ('login','step_up','channel_change')),
  channel            text NOT NULL CHECK (channel IN ('email','whatsapp')),  -- delivery channel
  destination        text NOT NULL,                      -- normalized address the code was sent to; the proof is for this address
  action             text CHECK (action IN ('source_create','source_rotate','connection_create','pairing_code_new',
                                        'connection_unpair','account_opt_in_accept','account_create_opt_in_accept',
                                        'admin_invitation_create','route_enable','account_binding_change')),
                                                         -- step_up only; = HTTP 3.0.0 StepUpAction (rev. 11: +2, pending Q35)
  target_id          uuid,                               -- step_up: the source, connection or account acted on (see CHECK)
  session_id         uuid,                               -- step_up / channel_change: only the requesting session may use the code
  code_role          text CHECK (code_role IN ('current_value','new_value','other_channel')),  -- channel_change only; = HTTP 2.0.0 role; other_channel is [Proposed] (S3b, policy §7)
  code_hmac          bytea NOT NULL CHECK (octet_length(code_hmac) = 32),  -- HMAC-SHA-256(k_code, challenge_id ‖ normalized code)
  pepper_version     smallint NOT NULL,
  failed_attempts    smallint NOT NULL DEFAULT 0 CHECK (failed_attempts BETWEEN 0 AND 5),
  created_at         timestamptz NOT NULL DEFAULT now(),
  sent_at            timestamptz NOT NULL DEFAULT now(),  -- last (re)send; a resend replaces this challenge's code
  expires_at         timestamptz NOT NULL,
  consumed_at        timestamptz,
  invalidated_at     timestamptz,
  invalidated_reason text CHECK (invalidated_reason IN ('attempts','channel_changed','change_closed','user_closed')),
  request_ip         inet,                               -- send limits and investigation; same retention as the row
  delivery_state     text NOT NULL DEFAULT 'pending' CHECK (delivery_state IN ('pending','sent','failed')),  -- §4.3 Delivery
  CONSTRAINT one_time_code_expiry_check
    CHECK (sent_at >= created_at AND expires_at > sent_at AND expires_at <= sent_at + interval '10 minutes'),
  CONSTRAINT one_time_code_invalidated_check CHECK ((invalidated_at IS NULL) = (invalidated_reason IS NULL)),
  CONSTRAINT one_time_code_attempts_check CHECK (failed_attempts < 5 OR invalidated_reason = 'attempts'),
  CONSTRAINT one_time_code_final_check CHECK (consumed_at IS NULL OR invalidated_at IS NULL),
  CONSTRAINT one_time_code_step_up_check
    CHECK ((purpose = 'step_up') = (action IS NOT NULL)
           AND (target_id IS NOT NULL) = COALESCE(action IN ('source_rotate','pairing_code_new','connection_unpair',
                                                            'account_opt_in_accept','route_enable',
                                                            'account_binding_change'), false)),
  -- route_enable / account_binding_change: target = the account (HTTP 3.0.0). Q35 turns each action on or off by
  -- Core setting (step_up.q35_persistence_actions), with no migration. Values not in this list need a contract
  -- change and a migration; these two go straight into #2b, which is not written yet.
  CONSTRAINT one_time_code_role_check
    CHECK ((purpose = 'channel_change') = (code_role IS NOT NULL) AND (code_role IS NULL) = (channel_change_id IS NULL)),
  CONSTRAINT one_time_code_session_check CHECK ((purpose IN ('step_up','channel_change')) = (session_id IS NOT NULL)),
  -- rev. 16.8 (#2e …061800) replaces it with: CHECK (CASE purpose WHEN 'login' THEN session_id IS NULL
  --   WHEN 'step_up' THEN session_id IS NOT NULL ELSE (session_id IS NOT NULL OR code_role = 'new_value') END).
  -- A session-less code is the new-value code of an admin_recovery change; the Core allows it only for that
  -- kind (a CHECK cannot read channel_change), with DB tests in both directions.
  -- rev. 16.8.1 (Cinzel; Esquadro (a)/(b), HTTP 5.4.0 StepUpAction admin_recovery): the operator command makes the
  -- ADMIN do a step-up (user_id = the admin; start: target_id = the target's user_id; confirm/cancel: target_id =
  -- the change id). The CLI has no session, so #2e also adds 'admin_recovery' to the action CHECK and to the
  -- target-required list of one_time_code_step_up_check, and the step_up branch of the session CHECK becomes
  -- (session_id IS NULL) = (action = 'admin_recovery'):
  --   CASE purpose WHEN 'login' THEN session_id IS NULL
  --                WHEN 'step_up' THEN (session_id IS NULL) = (action = 'admin_recovery')
  --                ELSE (session_id IS NOT NULL OR code_role = 'new_value') END
  -- Tests: an admin_recovery step-up without a session passes, with a session fails; another action without a session fails.
  CONSTRAINT one_time_code_whatsapp_check CHECK (channel <> 'whatsapp' OR destination ~ '^\+[1-9][0-9]{7,14}$'),
  FOREIGN KEY (user_id, session_id) REFERENCES identity.session (user_id, id),
  FOREIGN KEY (user_id, channel_change_id) REFERENCES identity.channel_change (user_id, id)
);
CREATE INDEX one_time_code_user_created_idx ON identity.one_time_code (user_id, created_at);
  -- per-signed-in-user limit on step_up / channel_change sends (policy §4: 10 per hour)
CREATE UNIQUE INDEX one_time_code_change_role_key ON identity.one_time_code (channel_change_id, code_role)
  WHERE channel_change_id IS NOT NULL;                   -- one code per role per change (a resend replaces it in place)
-- No UNIQUE on code_hmac: codes are looked up by id. No cap on open challenges and no eviction (policy §2.2):
-- a new challenge never invalidates another; the send limits bound how many exist.

CREATE TABLE identity.code_send_counter (              -- durable send limits per identifier (policy §3/§4); no RLS, no user_id
  identifier_hmac bytea NOT NULL CHECK (octet_length(identifier_hmac) = 32),  -- HMAC-SHA-256(k_ident, normalized e-mail or number)
  pepper_version  smallint NOT NULL,
  device_class    text NOT NULL CHECK (device_class IN ('known','unknown')),  -- separate budgets (policy §4)
  window_seconds  integer NOT NULL CHECK (window_seconds > 0),               -- policy §4: 60 (resend cooldown), 900 (5), 86400 (20)
  window_start    timestamptz NOT NULL,                  -- aligned to window_seconds by the Core
  sends           integer NOT NULL CHECK (sends > 0),
  PRIMARY KEY (identifier_hmac, pepper_version, device_class, window_seconds, window_start)
);
-- Counted for EVERY identifier, registered or not, so a 429 never reveals registration. The address itself is
-- never stored. One statement per window: INSERT … ON CONFLICT DO UPDATE SET sends = sends + 1 RETURNING sends.
-- Every send and resend counts, for known and unknown identifiers alike. allwin_app: SELECT, INSERT, UPDATE;
-- allwin_maint: DELETE. Pepper rotation: the Core sums both versions for the 24-h window and writes only the new
-- one; old-version rows age out with the 24-h purge.

-- Instruction sources: several named sources per user (Q17 recommendation)
CREATE TABLE identity.source (
  id          uuid PRIMARY KEY,
  user_id     uuid NOT NULL REFERENCES identity.app_user(id),
  source_type text NOT NULL CHECK (source_type ~ '^[a-z][a-z0-9_]{0,31}$'),  -- 'tradingview' (ADR-0003 §3)
  name        text NOT NULL,
  created_at  timestamptz NOT NULL DEFAULT now(),
  archived_at timestamptz,
  UNIQUE (user_id, id)
);
CREATE UNIQUE INDEX source_user_name_key ON identity.source (user_id, lower(name)) WHERE archived_at IS NULL;
-- Proposed (rev. 16.4, …061310, needs user authorization): CONSTRAINT source_name_shape_check CHECK (char_length(name) BETWEEN 1 AND 60
-- AND name ~ '\S' AND name !~ 'aw[kd]_[A-Za-z0-9]{12}_[A-Za-z0-9_-]{43}'); the audit copies the name.

CREATE TABLE identity.source_key (                       -- token awk_<key_id>_<secret>
  id          uuid PRIMARY KEY,
  user_id     uuid NOT NULL,
  source_id   uuid NOT NULL,
  key_id      text NOT NULL UNIQUE CHECK (key_id ~ '^[A-Za-z0-9]{12}$'),  -- public part; may appear in audit/logs (ALIGN-3)
  secret_hash bytea NOT NULL CHECK (octet_length(secret_hash) = 32),    -- SHA-256 of the 32-byte secret; never the token
  created_at  timestamptz NOT NULL DEFAULT now(),
  revoked_at  timestamptz,
  FOREIGN KEY (user_id, source_id) REFERENCES identity.source (user_id, id)
);
CREATE UNIQUE INDEX source_key_one_active ON identity.source_key (source_id) WHERE revoked_at IS NULL;
-- rotation = revoke + insert in one transaction (immediate, ADR-0001 §5)

-- One connection per NinjaTrader instance (S3 AC-15, ADR-0004 §8)
CREATE TABLE identity.adapter_connection (
  id               uuid PRIMARY KEY,                     -- connection_id (welcome)
  user_id          uuid NOT NULL REFERENCES identity.app_user(id),
  name             text NOT NULL,                        -- "NinjaTrader – Desk A"
  adapter_type     text CHECK (adapter_type ~ '^[a-z][a-z0-9_]{0,31}$'),  -- from pairing ('ninjatrader8')
  instance_id      uuid,                                 -- hello.instance_id
  instance_name    text,
  adapter_version  text,
  status           text NOT NULL DEFAULT 'awaiting_pairing'
                   CHECK (status IN ('awaiting_pairing','connected','disconnected','outdated','unpaired')),  -- HTTP contract Connection.status
  status_changed_at timestamptz NOT NULL DEFAULT now(),  -- = created_at until the first change
  paired_at        timestamptz,
  last_seen_at     timestamptz,
  created_at       timestamptz NOT NULL DEFAULT now(),
  archived_at      timestamptz,                          -- unpaired (Q18); row becomes read-only history
  UNIQUE (user_id, id),
  CHECK ((status = 'unpaired') = (archived_at IS NOT NULL)),
  CHECK (status = 'awaiting_pairing' OR status = 'unpaired' OR paired_at IS NOT NULL)
);
CREATE UNIQUE INDEX adapter_connection_user_name_key ON identity.adapter_connection (user_id, lower(name))
  WHERE archived_at IS NULL;                             -- connections.name_taken, case-insensitive, ignores unpaired
-- status: 'unpaired' is set by the unpair use case; the live states (connected/disconnected/outdated) are
-- written by identity from gateway session events (outbox), so status_changed_at survives a Core restart.
-- At Core startup, before accepting adapter connections, identity sets every 'connected'/'outdated' row to
-- 'disconnected' with status_changed_at = now() and audits it: after a crash no socket exists, and the UI must
-- not show a live plugin that is not there.

CREATE TABLE identity.pairing_code (
  id            uuid PRIMARY KEY,
  user_id       uuid NOT NULL,
  connection_id uuid NOT NULL,
  code_hmac     bytea NOT NULL UNIQUE CHECK (octet_length(code_hmac) = 32),  -- 40-bit code → HMAC-SHA-256 with pepper (ALIGN-4)
  pepper_version smallint NOT NULL,
  created_at    timestamptz NOT NULL DEFAULT now(),
  expires_at    timestamptz NOT NULL,                    -- +10 min
  used_at       timestamptz,
  invalidated_at timestamptz,                            -- superseded by a new code, or connection unpaired
  FOREIGN KEY (user_id, connection_id) REFERENCES identity.adapter_connection (user_id, id)
);
CREATE UNIQUE INDEX pairing_code_one_active ON identity.pairing_code (connection_id)
  WHERE used_at IS NULL AND invalidated_at IS NULL;

CREATE TABLE identity.device_token (                     -- token awd_<key_id>_<secret>
  id            uuid PRIMARY KEY,
  user_id       uuid NOT NULL,
  connection_id uuid NOT NULL,
  key_id        text NOT NULL UNIQUE CHECK (key_id ~ '^[A-Za-z0-9]{12}$'),  -- ALIGN-3
  token_hash    bytea NOT NULL CHECK (octet_length(token_hash) = 32),    -- SHA-256 of the 32-byte secret
  created_at    timestamptz NOT NULL DEFAULT now(),
  revoked_at    timestamptz,
  FOREIGN KEY (user_id, connection_id) REFERENCES identity.adapter_connection (user_id, id)
);
CREATE UNIQUE INDEX device_token_one_active ON identity.device_token (connection_id) WHERE revoked_at IS NULL;
-- re-pairing revokes the previous token in the same transaction (ADR-0004 §8)
```

- Token format (ALIGN-3): `awk_<key_id>_<secret>` (webhook) and `awd_<key_id>_<secret>` (device); `key_id` = 12 characters `[A-Za-z0-9]`, `secret` = 43 characters base64url (32 bytes). Only `key_id` and the SHA-256 of the secret are stored. The Core generates `key_id` at random and retries on a unique violation.
- **Throttling counters live in memory**, which is enough for a single Core instance (ADR-0001 §4):
  - code sends and code checks per IP and per IP prefix (security policy §3/§4);
  - pairing (ADR-0004 §8 as amended by U11): 10 attempts per minute per IP and per-/24 (IPv4) or /64 (IPv6) prefix limits. There is no global lock; the global failure threshold only raises an alert.

  The pairing request carries only the code, so there is no per-connection counter and no table. A Core restart resets the counters. That is acceptable: the brute-force margin rests on the 40-bit space and the 10-minute code lifetime.
- **Per-identifier send limits are durable** (`identity.code_send_counter`): keyed by the HMAC of the identifier, existing or not, so a restart does not reset them and a 429 is not an existence oracle. Wrong attempts are counted on the code row itself (`failed_attempts`).
- The "usual IPs" of a user come from `identity.session.ip_address` over the last 30 days, so no extra table is needed.

### 4.2 Secret storage

| Secret | Entropy | Stored as |
|---|---|---|
| Webhook secret, device token, session cookie, device cookie, invitation token | 256 bits | SHA-256, `bytea(32)`, looked up by `key_id` or hash, compared in constant time |
| Pairing code (40 bits), one-time code (8 Crockford Base32 symbols = 40 bits, U16 format 1), identifier for limits | low, human-typed | **HMAC-SHA-256 with keys derived by HKDF from the server pepper** (environment secret): `k_code` (code HMAC over `challenge_id ‖ code`), `k_ident` (identifier HMAC), `k_challenge` (seals the challenge token, Core only) and a pairing key. `pepper_version` stored per row; plus expiry and attempt limits (ALIGN-4, policy rev. 2). A plain SHA-256 of 40 bits can be brute-forced offline from a database dump |

No password, TOTP seed or recovery code is stored (rev. 6). The plaintext one-time code exists only in memory between generation and delivery. It is never written to `app.outbox`, the audit log or logs, so delivery is a direct call after the commit, not an outbox event (§4.3; policy §2.3).

### 4.3 Passwordless sign-in: data flows (rev. 6, proposed)

- **Code format (U16 format 1, policy §2.1)** does not change the schema: only the HMAC is stored. Eight Crockford Base32 symbols (`0–9 A–Z` without `I L O U`, 32 symbols, 40 bits). Before the HMAC, the Core normalizes the input: trim, remove spaces and `-`, upper-case, then `O→0`, `I→1`, `L→1`.
- **Signup (invitation only; S3 AC-2/AC-3):**
  1. The invitation token (`resolve_invitation`, open and unexpired) lets the invitee request a `signup_code` to the invited e-mail. No user row exists yet. The lifecycle (10 min, 5 attempts, resend, send limits keyed by the e-mail HMAC) is the same as `one_time_code`. WhatsApp is not part of S3 signup; it is added later as a channel change (S3b, Q31).
  2. On a matching code, one transaction locks the invitation (`SELECT … FOR UPDATE`) and checks it is still open. Then it inserts `app_user` (`email_verified_at = now()`) and marks the code consumed and the invitation accepted (`accepted_user_id`). It also creates the first session and device (policy §5.2, §10). A concurrent second confirmation finds the invitation accepted and fails with `auth.code_invalid`.
  3. **Repeated or abandoned signup:** there is no pending row to reuse or clean. A new request on the same open invitation is just another `signup_code` challenge. An abandoned one leaves the invitation usable until it expires (policy §10). Expired, revoking or superseding the invitation invalidates its open codes (`invitation_closed`). Rows are purged like `one_time_code` (§6.5).
- **Login:**
  1. Email + chosen channel → `find_user_for_login`, then the device class: `known` only when the device cookie maps to a non-revoked `known_device` of that user, so an unknown identifier is always `unknown`. Then the `code_send_counter` upserts for `(identifier_hmac, device_class)` (every identifier). All of this is inside the same equal-statement sequence. The answer is always 202 with a **sealed `challenge_id`** built by the Core: uuid ‖ identifier HMAC ‖ device class ‖ issued_at, MACed with `k_challenge`. The database stores only the uuid, as `one_time_code.id`, and only for a registered, `active` user with that channel. For anyone else, the Core runs an equivalent statement that writes nothing, so statements and timing match up to the commit (policy §3).
  2. **No cap, no eviction** (policy §2.2): a new challenge never invalidates another. Open challenges are bounded by the send limits per `(identifier_hmac, device_class)`: 5 per 15 min and 20 per 24 h, each living 10 min.
  3. **Resend** (the token says which identifier and device class):
     - the 60-s cooldown is a `code_send_counter` window keyed by `(identifier_hmac, device_class)`, not by the code row, so it is identical for unknown identifiers;
     - every resend counts in the counter;
     - with no row (unknown identifier) nothing is sent;
     - for an open, unexpired row the same transaction replaces `code_hmac`, `pepper_version`, `sent_at` and `expires_at`, and keeps `failed_attempts`, since resetting it would grant 5 more guesses per resend;
     - a closed row is never reopened and nothing is sent. The answer is always 202.
  4. **Verification:** the Core opens the token (bad MAC → `auth.code_invalid`). Then `resolve_one_time_code(uuid)` gives `user_id`; no row → `auth.code_invalid`. In that user's transaction, the Core runs `SELECT … FOR UPDATE` on the row and checks it is open and unexpired. Then it checks the destination per purpose (policy rev. 2):
     - `login`, `step_up`: `destination` is still a current confirmed value of the user;
     - `channel_change`, `current_value` / `other_channel`: `destination` is still a current confirmed value (of the changed channel / of the other channel), and the change is `awaiting_codes`;
     - `channel_change`, `new_value`: `destination` equals `channel_change.new_value` of a change still `awaiting_codes`;
     - signup (`signup_code`): the invited e-mail of a still-open invitation.
  5. It compares the HMAC in constant time.
     - On a mismatch: `failed_attempts + 1`, and at 5 `invalidated_reason = 'attempts'`. The failure is **committed**, even though the request fails.
     - On a match: `consumed_at`, plus the session, plus the known-device check, in the same transaction.
  6. `FOR UPDATE` serializes concurrent guesses, so 5 is a hard limit.
  7. **One error for every failure** (policy §3): a wrong code, expired, used, attempts exhausted, another session or an unknown challenge all return `auth.code_invalid`. Only the row keeps the internal reason.
- **Delivery (policy §2.3/§3):** the code is sent after the commit, by a task detached from the request, so the HTTP response time does not depend on whether a code was sent. A lost send is not retried; the user asks for a resend.
  - `delivery_state` on `signup_code` and `one_time_code` (all purposes) starts `pending`. A resend sets it back to `pending`. The detached task sets `sent` or `failed`.
  - A send suppressed by a channel clash is set to `sent` and writes no `auth.code_delivery_failed`, so a clash and a free value end in the same state.
  - Signup, step-up and channel-change clients read it per challenge. Login never exposes it: an unknown identifier has no row, so for login the column only drives the audit.
  - A failed send of a registered user also writes an `auth.code_delivery_failed` audit row.
- **Step-up** (the 8 HTTP 2.0.0 `StepUpAction` values, each on or off by Core setting per Q35; a channel change uses its own codes):
  1. A `step_up` code is bound to `session_id`, `action` and, for the six targeted actions (`source_rotate`, `pairing_code_new`, `connection_unpair`, `account_opt_in_accept`, `route_enable`, `account_binding_change`), `target_id`. Sends count against the per-user hourly limit (`one_time_code_user_created_idx`).
  2. The action request carries the challenge and code. The Core consumes it in the same transaction as the action, so no "elevated" state is stored.
  3. A channel change uses `purpose = 'channel_change'` instead, and its old-channel code is the step-up.
- **Channel change / adding WhatsApp later** (HTTP 2.0.0 `ChannelChange`):
  1. Refused while `channel_changed_at` is less than 24 h old (policy §7) or while another change is open (`channel_change_one_open`), checked under the user-row lock. Under the same lock, the start path first expires the user's stale `awaiting_codes` row (past `expires_at`), so it never blocks `channel_change_one_open` until the sweeper runs.
  2. Insert `channel_change` (`awaiting_codes`, `expires_at` = +10 min), then its codes, all bound to the requesting session:
     - `current_value` to the current value;
     - `new_value` to the new value;
     - `other_channel` to the other confirmed channel when the user has two (policy §7, [Proposed]).
     Adding WhatsApp later works the same way: `current_value` goes to the e-mail and `new_value` to the number.
  3. **Channel clash** (policy §3): if the new value belongs to another account, the change proceeds and the `new_value` row exists, but nothing is sent to it. Its `delivery_state` still ends `sent`, and its confirmation fails with `auth.code_invalid`. `app_user_email_normalized_key` / `app_user_whatsapp_key` are the backstop.
  4. When every code of the change is consumed:
     - **without a hold**, one transaction:
       - sets `completed` and updates the address, its `*_verified_at` and `channel_changed_at`;
       - invalidates the user's other open codes (`channel_changed`);
       - ends every other session (`channel_changed`) and rotates the current one;
     - **with a hold** (policy §9; §7 M2 if adopted), the change becomes `on_hold` with `hold_until`, and the address stays as it is. A worker finds due rows (`channel_change_hold_due_idx`) and applies them in the user's transaction:
       - it re-checks the 24-h `channel_changed_at` rule and that the new value is free; if either fails, it is a Core cancel (step 5);
       - it updates the address, `*_verified_at` and `channel_changed_at`;
       - it invalidates the user's open codes;
       - there is no current session to rotate, so **every** session ends (`channel_changed`);
       - for the policy §9 admin recovery, every device is also revoked;
       - for an M2 hold, whether devices are revoked as well is decided with M2 (policy).
  5. **Cancel ("Cancelar troca")**: `cancelled`, with `cancelled_at` and `cancelled_by_session_id`. The change's open codes are invalidated (`change_closed`). It is allowed from any session of the user while `awaiting_codes` or `on_hold`. If the new value has meanwhile been taken by another account when a hold ends, the Core cancels it with `cancelled_by_session_id` NULL. The clash is not revealed (policy §3): the user-visible audit row and the notice carry the same generic reason as any change that could not be completed (`me.channel_change_not_completed`). The real reason goes only to the internal/admin log.
  6. `expired`: an `awaiting_codes` change past `expires_at`, set lazily on the next read, by the start path (step 1) or by the expiry sweeper. Its open codes are invalidated (`change_closed`).
- **Not modelled yet (user decision pending):** a hold on a channel change requested from a new device, and an e-mail step-up for persistence from a new device. If approved, they need only `known_device.first_seen_at` (already there; device age < 7 days) and the `on_hold` state of `channel_change` (rev. 7; already modelled).
- **Sign out everywhere** (S3, Q34): one transaction ends every session (`signed_out_everywhere`) and sets `revoked_at` on every device.
- **Admin-assisted recovery** when a user loses access to every channel is **not modelled until the user decides Q28**. The Architect's recommendation is: an admin sets a new e-mail, confirmed by code, with a 72-h hold, cancellable, and notices to the old channels. It would likely add a pending-change row with `effective_at`/`cancelled_at`.

## 5. Accounts core (S5–S7)

### 5.1 Catalog (global, no RLS)

```sql
CREATE TABLE catalog.instrument (
  id           uuid PRIMARY KEY,
  symbol       text NOT NULL UNIQUE CHECK (symbol ~ '^[A-Z0-9]{1,10}$'),  -- neutral root, 'MNQ'
  name         text NOT NULL,
  exchange     text NOT NULL,                            -- 'CME'
  currency     char(3) NOT NULL DEFAULT 'USD',
  tick_size    NUMERIC(22,10) NOT NULL CHECK (tick_size > 0),
  point_value  NUMERIC(18,4)  NOT NULL CHECK (point_value > 0),   -- money per 1.0 price point
  session_code text NOT NULL,                            -- trading calendar id ('cme_equity_index')
  is_enabled   boolean NOT NULL DEFAULT false            -- instrument support is data (S1 AC-7)
);
-- tick value = tick_size × point_value, computed (not stored) so the two can never disagree

CREATE TABLE catalog.instrument_contract (               -- active contract month (ADR-0003 P5) — preview
  instrument_id  uuid NOT NULL REFERENCES catalog.instrument(id),
  contract_month date NOT NULL CHECK (extract(day FROM contract_month) = 1),   -- 'YYYY-MM' as first day
  active_from    timestamptz NOT NULL,                   -- rollover instant
  PRIMARY KEY (instrument_id, contract_month),
  UNIQUE (instrument_id, active_from)
);  -- global, maintained by the admin, with an advance warning to users (Q-A5 decided)

CREATE TABLE catalog.standard_account_size (             -- 25k, 50k, 100k, 150k, 200k, 300k (vision §8)
  size_amount NUMERIC(18,4) PRIMARY KEY CHECK (size_amount > 0)
);
```

Prop firm presets (`catalog.prop_firm`, `catalog.preset`, `catalog.preset_version`) come with S16 (Wave 2), shaped by the R1 research format. Accounts reserve nullable `firm_id` and `preset_version_id` for them.

**Opt-in rule versions (rev. 8; S5 contract §5.5).** The table exists from S5, because a prop account without a preset chooses against a **generic** version per kind (Q40, provisional). S16 fixes the non-generic shape and adds the foreign keys to firm and preset.

```sql
CREATE TABLE catalog.opt_in_rule_version (               -- global, no RLS; append-only (published text the user saw)
  id                uuid PRIMARY KEY,
  kind              text NOT NULL CHECK (kind IN ('automation','copy')),
  firm_stance       text NOT NULL CHECK (firm_stance IN ('allowed','forbidden','conditional','unknown','not_confirmed')),
  is_generic        boolean NOT NULL,
  firm_id           uuid,                                -- S16: FK to catalog.prop_firm
  firm_platform     text CHECK (firm_platform ~ '^[a-z][a-z0-9_]{0,31}$'),  -- when the rule is platform-specific
  version_number    integer NOT NULL CHECK (version_number >= 1),
  published_at      timestamptz NOT NULL DEFAULT now(),
  quotes            jsonb NOT NULL DEFAULT '[]' CHECK (jsonb_typeof(quotes) = 'array'),   -- RuleQuote[], validated by the Core
  research          jsonb CHECK (research IS NULL OR jsonb_typeof(research) = 'object'), -- unknown, not generic
  change_cause      text CHECK (change_cause IN ('stance_changed','text_changed','not_material')),
  change_reason     jsonb CHECK (change_reason IS NULL OR jsonb_typeof(change_reason) = 'object'),  -- {"pt-BR": …, "en": …}
  previous_id       uuid REFERENCES catalog.opt_in_rule_version(id),
  UNIQUE (id, kind),
  CONSTRAINT rule_version_generic_check CHECK (NOT is_generic OR (firm_stance = 'unknown' AND quotes = '[]'::jsonb
             AND research IS NULL AND firm_id IS NULL AND firm_platform IS NULL)),
  CONSTRAINT rule_version_firm_check CHECK (is_generic OR firm_id IS NOT NULL),
  CONSTRAINT rule_version_change_check CHECK ((previous_id IS NULL) = (change_cause IS NULL)),
  CONSTRAINT rule_version_reason_check CHECK (change_cause IS DISTINCT FROM 'text_changed' OR change_reason IS NOT NULL)
);
CREATE UNIQUE INDEX opt_in_rule_version_generic_key ON catalog.opt_in_rule_version (kind, version_number) WHERE is_generic;
-- Non-generic: one version per (release, firm_platform, kind), unique index added in S16 (§5.7).
-- Grants: allwin_app SELECT (rev. 16: SELECT only in #5a; INSERT is granted by the S16 migration with the admin
-- publish use case); no UPDATE/DELETE for anyone at runtime, so a published text cannot change under a recorded choice. A correction is a new version (change_cause).
-- The current version of a selection is the highest version_number.
```

- **S5 seeds** two generic rows, `automation` and `copy`, at version 1, inside migration #5. They hold no text: no quotes, and the UI shows its own generic warning. So the seed has no product content to wait for.
- `account.automation_rule_version_id` / `copy_rule_version_id` hold ids of this table as **plain `uuid`**, with no foreign key (§1, rule 1; rev. 9 corrects rev. 8). The `accounts` module checks through the catalog query port that the id exists and has the matching `kind`, in the same request that records the choice. Rows are append-only and never deleted, so an id once valid stays valid. Both rules have §9 tests.

### 5.2 Trading accounts

```sql
CREATE TABLE accounts.account (
  id                uuid PRIMARY KEY,
  user_id           uuid NOT NULL REFERENCES identity.app_user(id),
  display_name      text NOT NULL CHECK (char_length(display_name) BETWEEN 1 AND 80),  -- rev. 16: AccountName
  kind              text NOT NULL CHECK (kind IN ('prop','real')),
  firm_id           uuid,                                -- prop with a preset; catalog (S16)
  firm_name         text CHECK (char_length(firm_name) BETWEEN 1 AND 80),  -- prop, manual mode (rev. 8)
  plan_name         text CHECK (char_length(plan_name) BETWEEN 1 AND 80),  -- manual mode
  phase             text CHECK (char_length(phase) BETWEEN 1 AND 80),      -- manual mode; [Q-A3] open
  preset_version_id uuid,                                -- S16
  firm_platform     text CHECK (firm_platform ~ '^[a-z][a-z0-9_]{0,31}$'),  -- with a preset only (rev. 8)
  size_amount       NUMERIC(18,4) NOT NULL CHECK (size_amount > 0),   -- real: any size (Q12)
  currency          char(3) NOT NULL DEFAULT 'USD',
  multiplier        integer NOT NULL DEFAULT 1 CHECK (multiplier BETWEEN 1 AND 1000),
  max_position_qty  integer NOT NULL CHECK (max_position_qty BETWEEN 1 AND 10000),  -- one per account in V1 (Q-A1)
  lifecycle_state   text NOT NULL DEFAULT 'active' CHECK (lifecycle_state IN ('active','paused','disabled')),
  bracket_mode      text NOT NULL DEFAULT 'individual' CHECK (bracket_mode IN ('individual','shared')),
  command_ttl_ms    integer NOT NULL DEFAULT 5000 CHECK (command_ttl_ms BETWEEN 1000 AND 60000),  -- Q16/T2
  automation_state  text NOT NULL DEFAULT 'not_required'
                    CHECK (automation_state IN ('not_required','chosen','pending_review')),  -- opt-in (decision 2026-10-23, A)
  automation_choice text CHECK (automation_choice IN ('accepted','declined')),  -- no default: NULL until the user chooses
  automation_rule_version_id uuid,                       -- catalog.opt_in_rule_version (cross-module id): the rule text the user saw
  automation_pending_reason text CHECK (automation_pending_reason IN ('stance_changed','text_changed','preset_attached')),
  automation_chosen_at timestamptz,
  copy_state        text NOT NULL DEFAULT 'not_required'
                    CHECK (copy_state IN ('not_required','chosen','pending_review')),  -- copy opt-in (Q24, approved)
  copy_choice       text CHECK (copy_choice IN ('accepted','declined')),
  copy_rule_version_id uuid,                             -- catalog.opt_in_rule_version (cross-module id)
  copy_pending_reason text CHECK (copy_pending_reason IN ('stance_changed','text_changed','preset_attached')),
  copy_chosen_at    timestamptz,
  created_at        timestamptz NOT NULL DEFAULT now(),
  updated_at        timestamptz NOT NULL DEFAULT now(),
  archived_at       timestamptz,                         -- "deleted" (S5 AC-6); history keeps the id
  row_version       integer NOT NULL DEFAULT 1 CHECK (row_version >= 1),  -- optimistic concurrency
  limits_version    integer NOT NULL DEFAULT 1 CHECK (limits_version >= 1),  -- ADR-0006: bumped with every limits-relevant change
  limits_changed_at timestamptz NOT NULL DEFAULT now(),  -- ADR-0006 §3.5: age of a version gap (60-s alert)
  evaluation_start  date,                                -- rev. 15 (S13b): user-given; Core default = registration date (user's time zone); limits input
  evaluation_earned_before NUMERIC(18,4) CHECK (evaluation_earned_before >= 0),  -- rev. 15 (S13b): optional profit already earned before evaluation_start
  venue_account_id  uuid,                                -- rev. 16 (ADR-0008): bound gateway.venue_account (cross-module id, no FK); limits input
  bound_at          timestamptz,                         -- rev. 16: time of the current binding
  create_idempotency_key uuid,                           -- Idempotency-Key of the create (rev. 8)
  create_request_hash    bytea CHECK (octet_length(create_request_hash) = 32),  -- SHA-256 of the canonical body, step_up excluded
  UNIQUE (user_id, id),
  CONSTRAINT account_create_key UNIQUE (user_id, create_idempotency_key),
  CHECK ((create_idempotency_key IS NULL) = (create_request_hash IS NULL)),
  CHECK ((venue_account_id IS NULL) = (bound_at IS NULL)),                 -- rev. 16
  CHECK (archived_at IS NULL OR venue_account_id IS NULL),                 -- rev. 16: archiving releases the binding (§5.3)
  CHECK (kind = 'prop' OR (firm_id IS NULL AND firm_name IS NULL AND plan_name IS NULL AND phase IS NULL
                           AND preset_version_id IS NULL AND firm_platform IS NULL)),
  -- prop: a preset XOR the three manual fields (S5 contract §2.4); with a preset they come from the catalog
  CHECK (kind = 'real'
         OR (preset_version_id IS NOT NULL AND firm_name IS NULL AND plan_name IS NULL AND phase IS NULL)
         OR (preset_version_id IS NULL AND firm_id IS NULL AND firm_platform IS NULL
             AND firm_name IS NOT NULL AND plan_name IS NOT NULL AND phase IS NOT NULL)),
  CHECK ((automation_state = 'pending_review') = (automation_pending_reason IS NOT NULL)),
  CHECK ((copy_state = 'pending_review') = (copy_pending_reason IS NOT NULL)),
  CHECK (kind = 'prop' OR (automation_state = 'not_required' AND copy_state = 'not_required')),
  -- OptInState: chosen → choice + time set; pending_review → previous choice kept (shown, never preselected)
  -- or NULL if never chosen; not_required → all NULL. choice and chosen_at are set together.
  CHECK ((automation_choice IS NULL) = (automation_chosen_at IS NULL)),
  CHECK (automation_state <> 'chosen' OR automation_choice IS NOT NULL),
  CHECK (automation_state <> 'not_required' OR (automation_choice IS NULL AND automation_rule_version_id IS NULL)),
  CHECK ((copy_choice IS NULL) = (copy_chosen_at IS NULL)),
  CHECK (copy_state <> 'chosen' OR copy_choice IS NOT NULL),
  CHECK (copy_state <> 'not_required' OR (copy_choice IS NULL AND copy_rule_version_id IS NULL)),
  CHECK (automation_choice IS NULL OR automation_rule_version_id IS NOT NULL),  -- a choice always names the rule seen
  CHECK (copy_choice IS NULL OR copy_rule_version_id IS NOT NULL)               -- (choices exist only from S16)
);
CREATE UNIQUE INDEX account_user_name_key ON accounts.account (user_id, lower(display_name)) WHERE archived_at IS NULL;
CREATE INDEX account_user_live_idx ON accounts.account (user_id) WHERE archived_at IS NULL;
CREATE UNIQUE INDEX account_venue_account_key ON accounts.account (venue_account_id) WHERE venue_account_id IS NOT NULL;  -- rev. 16
-- Grants (rev. 16): allwin_app SELECT, INSERT, UPDATE; DELETE revoked, because an account is archived, never deleted at runtime.
```

- **Q13-b erasure (rev. 16):** no `ON DELETE CASCADE` reaches `account`, and `routing.subscription` has no FK to it. So the erasure job deletes `account_rule_value`, `account_bracket_profile` and the routing subscriptions (through the routing port) before the account. It runs as `allwin_maint`, with its own grant in a later migration whose number the DBA assigns.

- The bounds `multiplier ≤ 1000` and `max_position_qty ≤ 10000` mirror the structural caps in the contracts (instruction `quantity` ≤ 1000, command `quantity` ≤ 10000). The business limit is the user's value.
- **Prop selection (rev. 8):** a preset (`preset_version_id`, with `firm_id` and optional `firm_platform`) **or** the manual `firm_name` / `plan_name` / `phase`, never both. Manual mode covers the time before S16 (Q40) and plans excluded from presets (S16 AC-9). With a preset, firm, plan and phase are read from the catalog, not copied. `size_amount` is always stored, since risk reads it.
- **Create idempotency (rev. 8):**
  - The key and request hash stay on the row; there is no separate table. Only a successful create stores them.
  - A concurrent second request with the same key hits `account_create_key`. The Core then re-reads the row: the same hash within 24 h replays 200, otherwise 409.
  - The key is never released (contract `createAccount`, s5 §2.2):
    - the same key and body within 24 h of `created_at` → 200 with the account as it is, with `archived_at` if it was archived since. That is the only read that shows an archived account;
    - a different body, or any reuse after 24 h → 409 `accounts.idempotency_key_reused`;
    - never a second account.
- **`*_pending_reason`** (Q26, Q41, Q42) is set exactly with `pending_review` and cleared on the next choice.
- **Generic rule version (Q40):** a manual-mode prop account whose choice is required names the generic version of its kind, so "a choice always names the rule seen" holds before S16.
- "Max position cannot exceed the preset's" (S5 AC-3) is a domain rule checked against the preset version.
- **Automation opt-in** (decision 2026-10-23, option A; ADR-0003 §4.1 gate `routing.automation_not_accepted`):
  - Routing reads it through the `accounts` query port. An account accepts automated sources only when `automation_state = 'not_required'`, or when `automation_state = 'chosen'` and `automation_choice = 'accepted'`. A `declined` or `pending_review` account is skipped and audited.
  - The row holds only the **current** choice. Each change is written to the audit log **in the same transaction** through `app.outbox`: user, time, choice, preset version, and the source URL and access date of the rule text (spec O6). That gives the "date and user" record the decision requires, and it cannot be lost on a crash.
  - Real accounts are always `not_required` (enforced by a `CHECK`). Prop accounts stay `not_required` until S16 brings presets. The S16 migration then moves prop accounts whose stance requires a choice to `pending_review` (spec §2, PO to confirm).
  - **Copy opt-in (Q24, approved):** the `copy_*` columns mirror `automation_*` with the same states and `CHECK`s, and gate master replication (S19).
  - **`pending_review`** keeps the previous choice, its time and its rule version, so the UI can show them. The UI never preselects them. With `not_required`, all of them are `NULL`. Q25 and Q26 fit these columns without a schema change.
  - Not stored, because they are derived: `Source.is_automated` comes from `source_type`, and event `at` comes from the audit `occurred_at`.
  - The firm platform (spec O9) is a nullable prop-only column reserved for S16, like `firm_id`.
- **Single writer per flag:**
  - out-of-sync state belongs to `gateway` (§6.3);
  - risk limits, enforcement toggle and blocks belong to `risk` (§5.6).

  Routing reads both through query ports.

### 5.3 Binding a platform account (S5 AC-1)

Platform accounts arrive in `hello.accounts` (ADR-0004 §3) and belong to `gateway`. The `accounts` module never sees platform names (X-1).

**Ownership (rev. 16, Esquadro, ADR-0008):** `gateway` owns the platform facts, and `accounts` owns the binding (`account.venue_account_id`, §5.2). #9 is unwritten, so nothing is migrated.

```sql
CREATE TABLE gateway.venue_account (
  id               uuid PRIMARY KEY,
  user_id          uuid NOT NULL REFERENCES identity.app_user(id),
  connection_id    uuid NOT NULL,                        -- identity.adapter_connection (cross-module id)
  platform_account text NOT NULL CHECK (length(platform_account) BETWEEN 1 AND 128),
  connection_name  text,                                 -- hello.accounts[].connection_name
  kind             text NOT NULL CHECK (kind IN ('simulation','playback','live','unknown')),
  oco_location     text NOT NULL CHECK (oco_location IN ('local','server','unknown')),  -- hello.accounts[], refreshed on each hello
  first_seen_at    timestamptz NOT NULL DEFAULT now(),
  last_seen_at     timestamptz NOT NULL DEFAULT now(),
  UNIQUE (user_id, id),
  UNIQUE (connection_id, platform_account)
);
-- rev. 16: account_id, bound_at, is_master and venue_account_bound_key removed (the binding is in accounts).
-- is_master (S19, V1.1) is decided when S19 opens; it is probably an account role in accounts (Esquadro).
```

- **Binding in `createAccount` / `updateAccount`**, all in the `accounts` transaction:
  - `VenueAccounts::resolve(user_id, connection_id, platform_account) → Option<{venue_account_id, kind}>` is a `gateway` query port, scoped by `user_id`.
    - `None` → 422 `accounts.platform_account_unknown`.
    - `kind IN ('live','unknown')` outside production → 422 (ADR-0004 §8 environment guard).
    - The gateway still refuses commands to a `live`/`unknown` venue account outside production at delivery, because `kind` is refreshed on each `hello`.
  - **1 platform account ↔ at most 1 All Win account** is the unique index `account_venue_account_key`. A violation → 409 `accounts.platform_account_taken`.
    - The index is not scoped by RLS, so it also sees another user's rows. They cannot collide, because `resolve` returns only the user's own venue accounts.
    - Two concurrent binds of the same platform account serialize on the index: one wins and the other gets 409.
  - Changing or removing a binding first checks `Positions::has_open_position(venue_account_id)` → 409 `accounts.binding_open_position`.
    - A position that opens between this read and the commit, or a working order that fills after an unbind, is an accepted window (ADR-0008 §3.2).
    - **Residual:** the platform account then holds a position with no binding, and routing skips every action for it, exits included (`routing.platform_unbound`).
    - **Mitigation:** `gateway` audits and alerts the user on a fill or a non-flat `position_snapshot` for a venue account that `account_for_venue` reports as unbound. The code comes from the S11 sync contract, and no schema change is needed here.
  - The same transaction bumps `limits_version` (the binding is a limits input, ADR-0006 §3.1) and writes **two** events to `app.outbox` (rev. 16.1, ADR-0008 §3.2 step 5):
    - `account.limits_changed {account_id, limits_version}`, the ADR-0006 producer event that `risk` consumes;
    - `account.binding_changed {account_id, venue_account_id|null, limits_version}`, which `gateway` consumes for side effects (`welcome` / `limits` again).
- **Archiving releases the binding** (`CHECK`; ADR-0008): the archive transaction sets `venue_account_id` and `bound_at` to `NULL`, bumps `limits_version` and writes both events above. Otherwise an archived account would keep the platform account taken forever.
  - An open position already refuses `archiveAccount` with 409 `accounts.delete_open_position` (S5 AC-6), so there is no new code.
- **No FK across schemas** (§1): `gateway` must keep a `venue_account` row as long as an account may name it.
  - Rows are never deleted at runtime. A revoked connection keeps its rows.
  - A re-pair keeps the `connection_id` (`newPairingCode` on the same connection, ADR-0004 §8), so the `(connection_id, platform_account)` rows and their bindings survive it. Only an unpair followed by a new connection yields new ids, and the user rebinds (intended, ADR-0008).
  - Readers treat an id with no row as unbound (`routing.platform_unbound`).
- **Readers:** `gateway` and `routing` use the `accounts` query ports `binding_of(account_id)` and `account_for_venue(venue_account_id)`, read at the decision; there is no derived copy. `account_for_venue` uses `account_venue_account_key`. Gateway fills `VisibleAccount.account_id` this way; a batch variant for a `welcome` list is a port detail, with no schema impact.

- `oco_location` is stored on `venue_account`, not on `accounts.account`, for two reasons:
  - `hello` reports it per platform account;
  - it is a platform fact that `gateway` owns, and the `accounts` module never sees platform data (X-1).

  The Core treats `unknown` like `local` (ADR-0004 §6.1). The web warning reads it through a gateway query port.

### 5.4 Stop/target profiles (S6)

```sql
CREATE TABLE accounts.default_bracket_profile (          -- per user × standard size × instrument
  user_id         uuid NOT NULL REFERENCES identity.app_user(id),
  size_amount     NUMERIC(18,4) NOT NULL,                -- one of catalog.standard_account_size (domain-validated)
  instrument_id   uuid NOT NULL,                         -- profiles keyed by instrument (Q-A2 decided)
  distance_unit   text NOT NULL CHECK (distance_unit IN ('points','ticks','usd')),
  stop_distance   NUMERIC(22,10) NOT NULL CHECK (stop_distance > 0),
  target_distance NUMERIC(22,10) NOT NULL CHECK (target_distance > 0),
  updated_at      timestamptz NOT NULL DEFAULT now(),
  PRIMARY KEY (user_id, size_amount, instrument_id),
  CHECK (distance_unit <> 'ticks' OR (stop_distance = trunc(stop_distance) AND target_distance = trunc(target_distance))),
  CHECK (distance_unit <> 'usd'   OR (stop_distance = round(stop_distance, 4) AND target_distance = round(target_distance, 4)))
);

CREATE TABLE accounts.account_bracket_profile (          -- the account's own copy (S6 AC-2)
  user_id          uuid NOT NULL,
  account_id       uuid NOT NULL,
  instrument_id    uuid NOT NULL,
  distance_unit    text NOT NULL CHECK (distance_unit IN ('points','ticks','usd')),
  stop_distance    NUMERIC(22,10) NOT NULL CHECK (stop_distance > 0),
  target_distance  NUMERIC(22,10) NOT NULL CHECK (target_distance > 0),
  copied_from_size NUMERIC(18,4),                        -- provenance (nearest standard size for real accounts)
  origin           text NOT NULL CHECK (origin IN ('size_default','nearest_size_default','set_by_user')),  -- rev. 8 (S6 2.3.0)
  copied_at        timestamptz,                          -- initial copy or last restore
  customized       boolean NOT NULL DEFAULT false,       -- edited after the copy; false again on restore
  row_version      integer NOT NULL DEFAULT 1 CHECK (row_version >= 1),  -- optimistic concurrency (putAccountProfile)
  updated_at       timestamptz NOT NULL DEFAULT now(),
  PRIMARY KEY (account_id, instrument_id),
  CHECK ((origin = 'set_by_user') = (copied_from_size IS NULL)),
  CHECK ((copied_from_size IS NULL) = (copied_at IS NULL)),
  CHECK (origin <> 'set_by_user' OR NOT customized),     -- nothing copied to diverge from (Compasso, N3)
  FOREIGN KEY (user_id, account_id) REFERENCES accounts.account (user_id, id),
  CHECK (distance_unit <> 'ticks' OR (stop_distance = trunc(stop_distance) AND target_distance = trunc(target_distance))),
  CHECK (distance_unit <> 'usd'   OR (stop_distance = round(stop_distance, 4) AND target_distance = round(target_distance, 4)))
);
```

- **Provenance (rev. 8, S6 contract §4):**
  - A copy (`size_default`, `nearest_size_default`) carries `copied_from_size` and `copied_at`, and `set_by_user` carries neither, matching the contract's `if/then`.
  - `default_now` is derived, not stored: it is the current `default_bracket_profile` of `copied_from_size`.
  - Restore sets `copied_at = now()` and `customized = false`. Editing a copy sets `customized = true` and keeps the origin. A `set_by_user` profile is never `customized` (`CHECK`; contract `AccountProfile`).
  - **Exact tie between two standard sizes for a real account (Q60, provisional c):** the user picks the size at create (`createAccount.profile_size`). The copy is still `nearest_size_default` + `copied_from_size`, so there is no new column.
- **No profile (Q43, provisional):** an account created while its size had no default has no row, and routing skips its entries with `routing.no_bracket_profile`. The nearest-size tie rule (Q-S6-7) is a Core/PO rule with no schema impact.
- USD distances: the contract takes 2 decimals and the `CHECK` keeps 4 as headroom (Q-S6-6).

### 5.5 Routing configuration (S7)

```sql
CREATE TABLE routing.subscription (                      -- source → account, with signal-type filter
  id           uuid PRIMARY KEY,
  user_id      uuid NOT NULL REFERENCES identity.app_user(id),
  source_id    uuid NOT NULL,                            -- identity.source (cross-module id)
  account_id   uuid NOT NULL,                            -- accounts.account (cross-module id)
  signal_types text[] NOT NULL CHECK (
                 cardinality(signal_types) >= 1
                 AND signal_types <@ ARRAY['trigger','trade_plan','nq_trigger','master_account']::text[]),
  is_enabled   boolean NOT NULL DEFAULT true,
  row_version  integer NOT NULL DEFAULT 1 CHECK (row_version >= 1),  -- optimistic concurrency (updateRoute; rev. 8)
  created_at   timestamptz NOT NULL DEFAULT now(),
  updated_at   timestamptz NOT NULL DEFAULT now(),
  UNIQUE (user_id, source_id, account_id)
);
CREATE INDEX subscription_source_idx ON routing.subscription (user_id, source_id) WHERE is_enabled;
```

- **Signal filter per (source, account)** (Q-A4 decided, ALIGN-5 accepted; ADR-0003 §4.1 gate `routing.signal_type_not_selected`). Per-account behavior is the special case where the user sets the same filter on every source.
- The S7 route endpoints accept only `trigger`, `trade_plan` and `nq_trigger`; `master_account` stays in the `CHECK` for S19 follower subscriptions (S7 contract §5). This is a Core validation, with no schema change.
- Multiplier and max position are read from `accounts` at routing time. Every routing decision **snapshots** the values it used (§6), so the audit trail stays explainable after an edit.

### 5.6 Risk core (S13; rev. 13, proposed — contract 3.1.0)

All `risk.*` tables are user-owned (RLS `user_id = app.current_user_id()`), and the `risk` module is their only writer. **The rule inputs are not here.** They belong to `accounts` (`account_rule_value`, the account columns below, §5.7), and `risk.account_limits` is **derived** from them (ADR-0006).

```sql
CREATE TABLE risk.account_limits (                       -- derived, typed copy of the account's tracked rules (ADR-0006)
  user_id                   uuid NOT NULL REFERENCES identity.app_user(id),
  account_id                uuid NOT NULL,               -- accounts.account (cross-module id)
  version                   integer NOT NULL CHECK (version >= 1),   -- monotonic; sent in plugin `limits`
  derived_from_version      integer NOT NULL CHECK (derived_from_version >= 1),  -- account.limits_version of the consistent read
  effective_as_of           timestamptz NOT NULL,        -- the trading-day start these effective values cover (re-derived at each reset)
  enforcement_enabled       boolean NOT NULL DEFAULT false,  -- S15 AC-1; risk-owned, never touched by the derivation upsert
  daily_loss_kind           text CHECK (daily_loss_kind IN ('hard','soft_pause','none')),   -- NULL = unset
  daily_loss_limit          NUMERIC(18,4) CHECK (daily_loss_limit > 0),
  drawdown_type             text CHECK (drawdown_type IN ('trailing_intraday','trailing_eod','static')),  -- 'other' is untracked → NULL
  drawdown_amount           NUMERIC(18,4) CHECK (drawdown_amount > 0),
  daily_profit_target       NUMERIC(18,4) CHECK (daily_profit_target > 0),
  profit_target_on_reach    text NOT NULL DEFAULT 'notify' CHECK (profit_target_on_reach IN ('notify','flatten_and_block')),
  consistency_threshold_pct NUMERIC(4,1) CHECK (consistency_threshold_pct BETWEEN 0 AND 100),  -- tracked, never enforced (AC-6)
  evaluation_profit_target  NUMERIC(18,4) CHECK (evaluation_profit_target > 0),   -- rev. 15 (Q46, V1): notice only
  commission_per_contract_side NUMERIC(18,4) CHECK (commission_per_contract_side >= 0),  -- rev. 15.1: the rate IN FORCE today (a lower one waits in scheduled_limit); NULL = none
  evaluation_start          date,                        -- rev. 15: copy of account.evaluation_start
  evaluation_earned_before  NUMERIC(18,4),                -- rev. 15: copy of account.evaluation_earned_before
  reset_source              text NOT NULL CHECK (reset_source IN ('firm','cme_session_close')),
  reset_local_time          time,                        -- firm reset (account copy; Q-S13-2)
  reset_time_zone           text,                        -- IANA
  updated_at                timestamptz NOT NULL DEFAULT now(),
  PRIMARY KEY (account_id),
  UNIQUE (user_id, account_id),
  CHECK ((daily_loss_kind IS NULL OR daily_loss_kind = 'none') = (daily_loss_limit IS NULL)),
  CHECK ((drawdown_type IS NULL) = (drawdown_amount IS NULL)),
  CHECK ((reset_source = 'firm') = (reset_local_time IS NOT NULL AND reset_time_zone IS NOT NULL))
);
-- 'cme_session_close' (real accounts): the Core takes the close from the exchange calendar, not a stored time.

CREATE TABLE risk.scheduled_limit (                      -- DERIVED (ADR-0006 §3.7): a looser value in force from a later trading day
  user_id               uuid NOT NULL,
  account_id            uuid NOT NULL,
  kind                  text NOT NULL CHECK (kind IN ('daily_loss','drawdown','daily_profit_target','consistency','profit_target_on_reach',
                                                   'trading_day_reset',     -- reset changes are ALWAYS scheduled (F1)
                                                   'commission_per_contract_side')),  -- rev. 15.1: a LOWER rate only
  new_value             jsonb CHECK (new_value IS NULL OR (jsonb_typeof(new_value) = 'object'
                          AND NOT jsonb_path_exists(new_value, '$.**.amount ? (@.type() == "number")'))),  -- money as strings
  is_unset              boolean NOT NULL DEFAULT false,  -- the scheduled change removes the rule
  effective_from        timestamptz NOT NULL,            -- start of the trading day after the field's saved_at, under the CURRENT reset
  derived_from_version  integer NOT NULL CHECK (derived_from_version >= 1),
  PRIMARY KEY (account_id, kind),                        -- latest save wins → at most one schedule per kind
  CHECK ((new_value IS NULL) = is_unset),
  FOREIGN KEY (user_id, account_id) REFERENCES risk.account_limits (user_id, account_id)
);
CREATE INDEX scheduled_limit_due_idx ON risk.scheduled_limit (effective_from);

CREATE TABLE risk.trading_day (                          -- one row per account and trading day; P&L inputs, not percentages
  user_id         uuid NOT NULL,
  account_id      uuid NOT NULL,
  trading_date    date NOT NULL,                         -- the day label in the reset's time zone
  starts_at       timestamptz NOT NULL,
  ends_at         timestamptz NOT NULL,                  -- = next_reset
  source          text NOT NULL CHECK (source IN ('firm','cme_session_close')),
  start_balance   NUMERIC(18,4),                         -- balance at the reset, from the platform snapshot
  realized_pnl_gross NUMERIC(18,4) NOT NULL DEFAULT 0,
  realized_commission NUMERIC(18,4) NOT NULL DEFAULT 0 CHECK (realized_commission >= 0),   -- rev. 15: Σ fills' commission_effective
  commission_gap  NUMERIC(18,4) NOT NULL DEFAULT 0 CHECK (commission_gap >= 0),   -- Σ (effective − reported): what the platform balance lacks
  realized_pnl_net NUMERIC(18,4) GENERATED ALWAYS AS (realized_pnl_gross - realized_commission) STORED,  -- risk runs on net
  unrealized_pnl  NUMERIC(18,4),                         -- last known; NULL when unknown
  as_of           timestamptz,                           -- time of the last P&L input (staleness, §AccountRisk.as_of)
  PRIMARY KEY (account_id, trading_date),
  CHECK (ends_at > starts_at),
  FOREIGN KEY (user_id, account_id) REFERENCES risk.account_limits (user_id, account_id)
);

CREATE TABLE risk.evaluation_progress (                  -- rev. 15, DERIVED (ADR-0006): "how much is left to pass" (Q46)
  user_id               uuid NOT NULL,
  account_id            uuid NOT NULL PRIMARY KEY,
  evaluation_start      date NOT NULL,
  baseline_balance      NUMERIC(18,4) NOT NULL,          -- net balance at the start of evaluation_start's trading day
  baseline_source       text NOT NULL CHECK (baseline_source IN ('platform_snapshot','account_size')),
  derived_from_version  integer NOT NULL CHECK (derived_from_version >= 1),
  target_reached_at     timestamptz,                     -- notice + notification once; no automatic action (user decision)
  FOREIGN KEY (user_id, account_id) REFERENCES risk.account_limits (user_id, account_id)
);

CREATE TABLE risk.drawdown_tracker (                     -- trailing drawdown state (Q-S13-5)
  user_id             uuid NOT NULL,
  account_id          uuid NOT NULL PRIMARY KEY,
  high_water_balance  NUMERIC(18,4) NOT NULL,            -- intraday or end-of-day per drawdown_type
  floor_balance       NUMERIC(18,4) NOT NULL,            -- balance at which the drawdown is breached, now
  stopped_trailing_at timestamptz,                       -- when the firm's "stops trailing at" condition was met
  as_of               timestamptz NOT NULL,
  FOREIGN KEY (user_id, account_id) REFERENCES risk.account_limits (user_id, account_id)
);

CREATE TABLE risk.account_block (                        -- blocked until the reset (S15 writes it; no early unlock)
  id          uuid PRIMARY KEY,
  user_id     uuid NOT NULL,
  account_id  uuid NOT NULL,
  cause       text NOT NULL CHECK (cause IN ('daily_loss','drawdown','daily_profit_target')),
  blocked_at  timestamptz NOT NULL DEFAULT now(),
  until       timestamptz NOT NULL,                      -- the trading-day end
  ended_at    timestamptz,                               -- set at the reset
  CHECK (until > blocked_at),
  FOREIGN KEY (user_id, account_id) REFERENCES risk.account_limits (user_id, account_id)
);
CREATE UNIQUE INDEX account_block_active_key ON risk.account_block (account_id) WHERE ended_at IS NULL;

-- rev. 16.6, migration #10b `…061510` (Malha; Esquadro decision): which fills are in the trading_day sums.
CREATE TABLE risk.applied_fill (
  user_id              uuid NOT NULL,
  account_id           uuid NOT NULL,
  fill_id              text NOT NULL CHECK (char_length(fill_id) BETWEEN 1 AND 128),   -- platform execution id
  trading_date         date NOT NULL,                    -- fixed at the first application; never moves
  qty                  integer NOT NULL CHECK (qty >= 0),          -- as applied now (0 after a remove)
  gross                NUMERIC(18,4) NOT NULL,                      -- realized gross P&L this fill contributed
  commission_effective NUMERIC(18,4) NOT NULL CHECK (commission_effective >= 0),
  commission_gap       NUMERIC(18,4) NOT NULL CHECK (commission_gap >= 0),
  last_event_id        uuid NOT NULL,                    -- gateway execution_report.event_id applied last (trace)
  applied_at           timestamptz NOT NULL DEFAULT now(),
  removed_at           timestamptz,                      -- fill.operation = remove; the row stays as the dedupe record
  PRIMARY KEY (account_id, fill_id),                     -- fill_id is unique per account, not globally (ADR-0004 §4)
  CHECK (removed_at IS NULL OR (qty = 0 AND gross = 0 AND commission_effective = 0 AND commission_gap = 0)),
  FOREIGN KEY (user_id, account_id) REFERENCES risk.account_limits (user_id, account_id),
  FOREIGN KEY (account_id, trading_date) REFERENCES risk.trading_day (account_id, trading_date)
);
CREATE INDEX applied_fill_trading_date_idx ON risk.applied_fill (trading_date);   -- retention purge
```

- **Applied fills (rev. 16.6; Esquadro decision, Malha #10b):**
  - **Never count twice:** `add` → `INSERT … ON CONFLICT (account_id, fill_id) DO NOTHING`; only an inserted row adds its values to `trading_day` (same risk transaction). `amend` → the difference between the new values and the stored ones is applied to the row's own `trading_date`, and the row takes the new values, so a redelivered amend applies 0. `remove` → the stored values are subtracted, the row is zeroed and gets `removed_at`; a redelivered remove is a no-op. A correction for an unknown or purged fill changes nothing and alerts (code from the S13 contract).
  - The `trading_day` FK fixes the day: a late correction updates the day it was first counted in, never today's.
  - **Retention:** `allwin_maint` deletes rows with `trading_date` older than **30 days**, in batches. That is longer than the adapter's 7-day dedupe record (ADR-0004 §4) and covers month-end corrections. `trading_day` is not purged.
  - Consumer `from_now`, plus the startup reconciliation through `PlatformPnl::fills_since(account_id, trading_day_start)`, which goes through the same `ON CONFLICT` path.
  - Tests: the same `add` twice counts once; an amend redelivered applies once; a remove zeroes the row and restores the sums; a correction after midnight updates the original day; a purge older than 30 days leaves `trading_day` intact; a cross-user read returns 0 rows.
- **Grants (rev. 16.6, DBA review of #10):**
  - `allwin_app` keeps `DELETE` only on `scheduled_limit` (reset step 2). `REVOKE DELETE` on `account_limits`, `trading_day`, `evaluation_progress`, `drawdown_tracker`, `account_block` and `applied_fill`: no runtime path deletes them, and a block row never vanishes (no early unlock). Erasure (Q13-b) and the `applied_fill` purge run as `allwin_maint`; #10b grants it `SELECT, DELETE` on `applied_fill`, and the erasure grant on the other tables comes in a later migration that the DBA numbers.
  - `allwin_worker` finds due work with column grants only: `SELECT (user_id, account_id, effective_from)` on `scheduled_limit`, `SELECT (user_id, account_id, until, ended_at)` on `account_block`, and `SELECT (user_id, account_id, effective_as_of, reset_source, reset_local_time, reset_time_zone)` on `account_limits` (stuck-reset alert). Each item's work then runs on the `allwin_app` pool with `app.user_id` set (§2 roles). A test asserts that the worker can read those columns and nothing else.

- **Net (rev. 15; user decision: risk computed on net):**
  - Daily loss, drawdown, the daily profit target and S15 enforcement all use net figures.
  - `realized_pnl_net` = gross − effective commissions.
  - The trailing drawdown's high water and floor follow the **adjusted** balance = platform balance − Σ `commission_gap` **over every `trading_day` row since the account's tracking start** (cumulative, not the day's). For the evaluation baseline and progress, the window is since `evaluation_start`. A simulation account that reports no commission therefore still pays the configured floor.
  - **Live usage** (daily loss and the intraday drawdown on equity) also deducts the **expected closing commission** = rate × open quantity, using the floor only, since the platform's real one is not known yet. This is fail-closed: a limit triggers a few dollars earlier, never later. End-of-day and realized values stay realized-only. The plugin uses the same rule (ADR-0004 §7, protocol 1.3). The expected commission is computed per request, not stored.
- **Evaluation target (rev. 15, Q46 in V1):**
  - Inputs are in `accounts`: `evaluation_start`, defaulting to the registration date, and the `evaluation_profit_target` rule field.
  - `risk.evaluation_progress` is derived under ADR-0006, through the same consistent read and `derived_from_version`. The baseline is the net balance at the start of `evaluation_start`'s trading day, from `trading_day.start_balance` adjusted by the commission gap (`platform_snapshot`). Before that data exists (e.g. a start date earlier than the account's first day), it is the account's `size_amount` (`account_size`). Esquadro agreed. While `baseline_source = 'account_size'`, the progress view shows "base estimada pelo tamanho da conta".
  - **Protocol 1.3:** `limits.evaluation.baseline_net_pnl` = `evaluation_earned_before` + (adjusted balance at the previous day's close − baseline). The plugin adds the day's net to it.
  - Remaining = target − (`evaluation_earned_before` + current adjusted net balance − baseline). It is computed per request, not stored.
  - **"Reiniciar contagem"** (S13b; `POST /accounts/{id}/evaluation/restart`, HTTP 3.5.0) is the only way to change `evaluation_start` after create. It is an `accounts` write: a new `evaluation_start` (today by default), with `evaluation_earned_before` cleared unless re-entered. It bumps `limits_version` and is audited (`accounts.evaluation_restarted`, old and new start), so `risk` re-derives a fresh `evaluation_progress` with `target_reached_at` NULL.
  - Reaching the target sets `target_reached_at` and emits a notice and notification once, with no automatic action. Minimum days and payout come later.
- **Derived, not computed in SQL:** `used`, `limit`, `percent` (truncated), `state`, `unset_rules`, `untracked_rules`, `limits_pending` (ADR-0006 versions) and `stale` / `stale_reason` (connection state from `gateway`, plus `as_of`). Each is computed by the Core per request from the rows above. The 80 % warning level is fixed (D18), so the preview's `warning_threshold_pct` column is dropped.
- **Provisional decisions stay out of the schema:**
  - **Q47:** no `NOT NULL` on the daily loss; "required for prop unless the preset says `none`" is a Core check.
  - **Q49 / Q63:** no `CHECK` couples `flatten_and_block` with `enforcement_enabled`. The Core treats it as `notify` when enforcement is off.
  - **Q48:** only `risk.scheduled_limit` serves it. If Q48 becomes "immediately", the table stays empty, and the `saved_at` / `is_unset` inputs stay useful anyway.
- **Q48 (ADR-0006 §3.7, decided by Esquadro; Code Review A1):**
  - **Input** (`accounts`): the value, or `is_unset`, plus the field's own `saved_at`. The latest save of a field wins.
  - **Derivation** (`risk`, from the consistent read plus the limits in force), per kind:
    - stricter → effective now, in `account_limits`;
    - looser → `scheduled_limit`, with `effective_from` = the start of the trading day after **that field's** `saved_at`. It is never the account-wide `limits_changed_at`, which an unrelated later save would move.
    - "Looser" follows the contract: removing a rule, `flatten_and_block` → `notify`, and `daily_loss` `hard` → `soft_pause` / `none`.
    - **An incomparable change of one kind counts as looser** (e.g. drawdown `static` 2000 → `trailing_eod` 2500), so it is scheduled and today's limit stays (fail closed).
    - **A `trading_day_reset` change is always scheduled, never effective now** (Code Review F1). Its `effective_from` is the start of the trading day after its `saved_at`, computed with the **current** reset. Moving the reset mid-day would otherwise re-cut today: an earlier reset would end the block and restart the daily counters early (an early unlock, vision §9), and a later one would stretch today.
    - A stricter save replaces the pending one at the input, so after the reset the stricter value is still in force.
    - Only the derivation writes `account_limits` and `scheduled_limit`, never an API use case. Both can be rebuilt from the accounts read.
  - **Reset step** (one `risk` step per trading-day start):
    1. end the previous day's blocks (`account_block.ended_at`);
    2. apply the due `scheduled_limit` rows (`scheduled_limit_due_idx`) and delete them;
    3. re-derive, setting `effective_as_of` = that day's start and bumping `version`;
    4. send `limits` to the plugin.
    - There is no `limits_version` bump, since no input changed. A looser value never ends a block early (no early unlock).
  - **Monitoring:** an alert fires when `effective_as_of` stays behind the current trading-day start for more than 60 s (stuck reset). Meanwhile the previous, stricter or equal, limits stay in force.
  - **A pending-only save still bumps `limits_version`.** That briefly gates entries; it is intended, since it keeps `pending_changes` visible.
  - `AccountRisk.pending_changes` reads `scheduled_limit`.
  - **Ports:** only risk → accounts (query port); the trait lives in risk's application crate and is wired in `crates/app`.
- **Consistency** needs every day's profit, so `trading_day` rows are kept for the account's lifetime and erased with it (Q13-b). The volume is about 250 rows per account per year.
- **Playback (S15 AC-7):** the day boundaries and P&L follow the simulated clock. `starts_at` / `ends_at` / `as_of` hold simulated instants for playback accounts. This is to be confirmed with S15 when it writes the playback clock source.
- The P&L inputs (fills, platform balance snapshots) arrive as `gateway` events. Their mapping is S13 Backend work, under ADR-0006 rules for derived state.

### 5.7 Prop firm presets (S16; rev. 10, proposed)

Global catalog, no RLS, written only by the publishing use case.
- **Publication (rev. 16.8; Esquadro (B), Torno #11 `…061700`, #11b `…061710`, #11c `…061720`):**
  - Only the publish-release CLI writes the catalog, as `allwin_publisher` (`NOBYPASSRLS`, created by DevOps in every environment before #11): `SELECT, INSERT` on the immutable tables, the column `UPDATE`s below, `SELECT` on `instrument` and `standard_account_size`; nothing outside `catalog`, no outbox, no audit.
  - The CLI takes `pg_advisory_xact_lock(hashtext('catalog'), hashtext('publish'))` as the first statement of its transaction (rev. 16.8.1, Esmeril N4: the two-key form; PostgreSQL keeps the one-key `bigint` space, used by the outbox consumer-name locks, separate from the two-key space). Two-key locks are namespaced by `hashtext('<module>')` as the first key. `LOCK TABLE … EXCLUSIVE` would need table-level `UPDATE`, which the publisher must not have. So publications are serial and commit in `publish_seq` order; a rollback leaves a gap, and the cursor reads `> last`, so it never skips a committed release.
  - An accounts worker job polls `releases_published_after(publish_seq)` through the catalog query port (every 30 s and at start). For each release in order, it suspends the affected opt-ins in one user-scoped transaction per user (`accounts.opt_in_suspended`, `presets.version_available`, idempotent by `(release_id, account_id)`), then advances the cursor by compare-and-set. The routing gate keeps the window fail-closed.
  - The publication's trail is the immutable `firm_release` row (`published_at`, `source_sha256`, `published_by`) plus redacted structured logs.
  - #11b uses the composite FK `(firm_id, release_id) → firm_release (firm_id, id)`, so a rule version cannot name another firm's release.
- **Published content is immutable.** `firm_release`, `firm_release_platform`, `firm_stance`, `preset_version`, `preset_rule_set` and `opt_in_rule_version` get `SELECT, INSERT` only, with no `UPDATE`/`DELETE`. A correction is a new release.
- **Identity rows can be renamed** (`prop_firm`, `firm_plan`, `preset`). Their ids are stable across releases. A rename is published with the release: same id, a column-level `GRANT UPDATE (name)` on `prop_firm` and `firm_plan` and `UPDATE (size_label)` on `preset`, and no `DELETE`.
  - Names are current display labels. History keeps its own snapshot: audit `details`, and `account` manual fields only for manual mode.
  - A plan that really changes (new rules, new sizes) is a new `plan_id`, decided by the publisher. All foreign keys below stay inside `catalog` (§1, rule 1).

```sql
CREATE TABLE catalog.prop_firm (
  id          uuid PRIMARY KEY,
  slug        text NOT NULL UNIQUE CHECK (slug ~ '^[a-z0-9]+(-[a-z0-9]+)*$' AND char_length(slug) <= 64),
  name        text NOT NULL CHECK (char_length(name) BETWEEN 1 AND 120),
  created_at  timestamptz NOT NULL DEFAULT now()
);

CREATE TABLE catalog.firm_release (                      -- versioning unit: one researched, published firm (S16 §2.1)
  id             uuid PRIMARY KEY,
  publish_seq    bigint GENERATED ALWAYS AS IDENTITY UNIQUE,   -- rev. 16.8: the accounts job's cursor (Esquadro (B))
  firm_id        uuid NOT NULL REFERENCES catalog.prop_firm(id),
  version_number integer NOT NULL CHECK (version_number >= 1),
  published_at   timestamptz NOT NULL DEFAULT now(),
  verified_on    date NOT NULL,
  research_file  text NOT NULL CHECK (research_file ~ '^docs/product/presets/research/[a-z0-9-]+\.md$'),  -- rev. 16.8
  source_sha256  bytea NOT NULL CHECK (octet_length(source_sha256) = 32),   -- rev. 16.8: canonical release file
  published_by   text NOT NULL CHECK (published_by ~ '^[a-z0-9][a-z0-9._-]{0,63}$'),  -- rev. 16.8: operator label, never an e-mail
  changes        jsonb NOT NULL DEFAULT '[]' CHECK (jsonb_typeof(changes) = 'array'),       -- ReleaseChange[]
  manual_only    jsonb CHECK (manual_only IS NULL OR jsonb_typeof(manual_only) = 'object'), -- RuleQuote (S16 AC-7)
  excluded_plans jsonb NOT NULL DEFAULT '[]' CHECK (jsonb_typeof(excluded_plans) = 'array'),  -- S16 AC-9 [{name, reason}] (rev. 16.8)
  UNIQUE (firm_id, version_number),
  UNIQUE (firm_id, id),
  UNIQUE (firm_id, source_sha256)                        -- rev. 16.8: re-importing the same file conflicts
);

CREATE TABLE catalog.firm_release_platform (             -- FirmDetail.platforms
  release_id uuid NOT NULL REFERENCES catalog.firm_release(id),
  code       text NOT NULL CHECK (code ~ '^[a-z][a-z0-9_]{0,31}$'),
  label      text NOT NULL CHECK (char_length(label) BETWEEN 1 AND 60),
  manual_only jsonb CHECK (manual_only IS NULL OR jsonb_typeof(manual_only) = 'object'),  -- rev. 16.8 (S-R3-2/4): {reason}
  PRIMARY KEY (release_id, code)
);

CREATE TABLE catalog.firm_stance (                       -- vps / personal_device per release (automation/copy = opt_in_rule_version)
  release_id uuid NOT NULL REFERENCES catalog.firm_release(id),
  kind       text NOT NULL CHECK (kind IN ('vps','personal_device')),
  value      text NOT NULL,
  quotes     jsonb NOT NULL DEFAULT '[]' CHECK (jsonb_typeof(quotes) = 'array'),   -- RuleQuote[], verbatim
  research   jsonb CHECK (research IS NULL OR jsonb_typeof(research) = 'object'),
  PRIMARY KEY (release_id, kind),
  CHECK ((kind = 'vps' AND value IN ('allowed','forbidden','conditional','unknown','not_confirmed'))
      OR (kind = 'personal_device' AND value IN ('required','not_required','unknown','not_confirmed')))  -- rev. 16.8 (HTTP 5.4.0)
);

CREATE TABLE catalog.firm_plan (                         -- stable across releases (plan_id on the wire)
  id      uuid PRIMARY KEY,
  firm_id uuid NOT NULL REFERENCES catalog.prop_firm(id),
  key     text NOT NULL CHECK (key ~ '^[a-z0-9]+(-[a-z0-9]+)*$' AND char_length(key) <= 64),  -- rev. 16.8: release-file key
  name    text NOT NULL CHECK (char_length(name) BETWEEN 1 AND 120),
  UNIQUE (firm_id, id),
  UNIQUE (firm_id, key)
);
CREATE UNIQUE INDEX firm_plan_name_key ON catalog.firm_plan (firm_id, lower(name));

CREATE TABLE catalog.preset (                            -- firm × plan × phase × size; stable across releases
  id          uuid PRIMARY KEY,
  firm_id     uuid NOT NULL,
  plan_id     uuid NOT NULL,
  phase       text NOT NULL CHECK (phase IN ('evaluation','funded','live')),
  size_amount NUMERIC(18,4) NOT NULL CHECK (size_amount > 0),
  size_key    text NOT NULL CHECK (size_key ~ '^[a-z0-9]+(-[a-z0-9]+)*$' AND char_length(size_key) <= 64),  -- rev. 16.8
  size_label  text CHECK (char_length(size_label) <= 32),
  UNIQUE (plan_id, phase, size_amount),
  UNIQUE (plan_id, phase, size_key),
  UNIQUE (firm_id, id),
  FOREIGN KEY (firm_id, plan_id) REFERENCES catalog.firm_plan (firm_id, id)
);

CREATE TABLE catalog.preset_version (                    -- one per preset per release that offers it
  id          uuid PRIMARY KEY,
  preset_id   uuid NOT NULL,
  firm_id     uuid NOT NULL,
  release_id  uuid NOT NULL,
  manual_only jsonb CHECK (manual_only IS NULL OR jsonb_typeof(manual_only) = 'object'),  -- RuleQuote; risk only, no routing
  UNIQUE (preset_id, release_id),
  FOREIGN KEY (firm_id, preset_id)  REFERENCES catalog.preset (firm_id, id),
  FOREIGN KEY (firm_id, release_id) REFERENCES catalog.firm_release (firm_id, id)   -- same firm, by construction
);
-- superseded_by is derived (the same preset's version in a later release), not stored, so rows stay immutable.

CREATE TABLE catalog.preset_rule_set (                   -- PresetVersion.rule_sets
  preset_version_id uuid NOT NULL REFERENCES catalog.preset_version(id),
  firm_platform     text CHECK (firm_platform ~ '^[a-z][a-z0-9_]{0,31}$'),  -- NULL = one set for every platform
  rules             jsonb NOT NULL,                      -- PresetRules with provenance per field (verified / not_confirmed)
  stored_only       jsonb CHECK (stored_only IS NULL OR (jsonb_typeof(stored_only) = 'object'
                      AND NOT jsonb_path_exists(stored_only, '$.**.amount ? (@.type() == "number")'))),  -- rev. 16.8: AC-10, never exposed in V1
  CHECK (jsonb_typeof(rules) = 'object'
         AND rules ?& ARRAY['max_position_qty','drawdown','daily_loss_limit','trading_day_reset']),  -- S16 AC-6 fields
  CHECK (NOT jsonb_path_exists(rules, '$.**.amount ? (@.type() == "number")'))   -- money only as decimal strings (X-3)
  -- Every money field in PresetRules / PresetRuleValues is named `amount` (MoneyAmount). A future money field with
  -- another key must be named `amount` or added to this CHECK (and to account_rule_value's).
);
CREATE UNIQUE INDEX preset_rule_set_key ON catalog.preset_rule_set (preset_version_id, coalesce(firm_platform, ''));
-- "One set without a platform OR one per platform" is a publishing rule (Core) plus a §9 test.

-- rev. 16.8, #11c `…061720` (Torno): the accounts job's cursor over published releases (Esquadro (B)).
CREATE TABLE accounts.catalog_release_cursor (           -- system table: one row, no user_id, no RLS
  singleton        boolean PRIMARY KEY DEFAULT true CHECK (singleton),
  last_publish_seq bigint NOT NULL DEFAULT 0 CHECK (last_publish_seq >= 0),
  updated_at       timestamptz NOT NULL DEFAULT now()
);   -- seeded with its one row; REVOKE ALL FROM allwin_app; allwin_worker SELECT, UPDATE (last_publish_seq, updated_at)
```

- **Why the rules are stored as `jsonb`:** they are published documents. Each holds a value or ≥ 2 published values, sources, published texts and free wording (`consistency_rule.text`), and is shown and diffed but never computed in SQL. Their exact schema is the contract (`PresetRules`), validated by the publishing use case. Money inside is a **JSON string** (`MoneyAmount`), never a JSON number, and the second `CHECK` enforces that. Arithmetic happens only on the typed `NUMERIC` copy in `risk.account_limits`.
- **`opt_in_rule_version` (rev. 8 table), forward change:**
  - A rule version is **per firm release × `firm_platform` × kind** (Compasso, S16). The stance belongs to the firm, not to a plan, phase or size; `getOptInRules` takes `preset_version_id` only to find the release and platform. So the table has **no `preset_version_id`**: rev. 10 removes it from the §5.1 DDL. #5 is not written yet, so nothing needs to be dropped.
  - Add `release_id uuid` with `CHECK (is_generic = (release_id IS NULL))`, the composite FK `(firm_id, release_id) → firm_release (firm_id, id)` (rev. 16.8: same firm by construction; generic rows have both NULL) and the FK `firm_id → prop_firm`, all inside `catalog` (#11b `…061710`; `…060900` is never edited). Grants: `allwin_publisher` `SELECT, INSERT`; the app stays `SELECT`-only.
  - `firm_platform` (already there) stays NULL for generic rows and for firms whose rules do not vary by platform.
  - Non-generic uniqueness: `CREATE UNIQUE INDEX … (release_id, firm_platform, kind) NULLS NOT DISTINCT WHERE NOT is_generic` (PG ≥ 15). `version_number` keeps counting per `(firm_id, kind, firm_platform)`.
  - The existing rows are the 2 generic ones, so the new constraints validate trivially.
- **Accounts keep plain ids:** `account.firm_id`, `account.preset_version_id` and `*_rule_version_id` are plain `uuid` (§1, rule 1). The catalog query port resolves them and checks existence and coherence:
  - the version belongs to the firm;
  - the platform is offered by the release;
  - `account.size_amount` equals the preset's `size_amount`.
  These are Core rules with §9 tests, since cross-module `CHECK`s are not possible.

```sql
CREATE TABLE accounts.account_rule_value (               -- the account's own rule values and their origin (S16 §2.4, Q-S16-3)
  user_id     uuid NOT NULL,
  account_id  uuid NOT NULL,
  field       text NOT NULL CHECK (field IN ('max_position_qty','drawdown','daily_loss_limit',
                                             'daily_profit_target','trading_day_reset','consistency_rule',
                                             'profit_target_on_reach','consistency_threshold_pct',  -- S13 (rev. 13)
                                             'commission_per_contract_side','evaluation_profit_target')),  -- rev. 15
  origin      text NOT NULL CHECK (origin IN ('edited','chosen_from_conflict','set_by_user')),  -- set_by_user: no preset (rev. 13)
  value       jsonb,                                     -- typed by field (PresetRuleValues); NULL for max_position_qty
  chose       text CHECK (chose IN ('published_value','other_value')),
  chosen_at   timestamptz,
  is_unset    boolean NOT NULL DEFAULT false,          -- the user removed this rule (S13); a tombstone that keeps its saved_at
  saved_at    timestamptz NOT NULL DEFAULT now(),        -- this field's own save time (ADR-0006 §3.7; never account-wide)
  updated_at  timestamptz NOT NULL DEFAULT now(),
  PRIMARY KEY (account_id, field),
  FOREIGN KEY (user_id, account_id) REFERENCES accounts.account (user_id, id),
  CHECK (CASE WHEN field = 'max_position_qty' THEN value IS NULL AND NOT is_unset  -- value is account.max_position_qty (S5); never removed
              ELSE (value IS NULL) = is_unset END),
  CHECK (value IS NULL OR jsonb_typeof(value) = 'object'),
  CHECK (value IS NULL OR NOT jsonb_path_exists(value, '$.**.amount ? (@.type() == "number")')),
  CHECK ((origin = 'chosen_from_conflict') = (chose IS NOT NULL)),
  CHECK ((chose IS NULL) = (chosen_at IS NULL))
);  -- RLS: user_id = app.current_user_id()
```

- **Only deviations are stored.** `preset` is the absence of a row (the value comes from the preset version), and `unresolved` is an optional not-confirmed field with no row. So applying a version or restoring a value deletes or rewrites rows, and the getAccountPreset `origin` is derived.
- **New S13 fields:**
  - `profit_target_on_reach`: value `{"mode":"notify"|"flatten_and_block"}`. No row means `notify`.
  - `consistency_threshold_pct`: value `{"pct":"30.0"}`, at most one decimal (Q-S13-7).
  - They are rule rows so that a looser change (`flatten_and_block` → `notify`, a higher threshold) can be pending like the others.
- **Rev. 15 fields:**
  - `commission_per_contract_side`: `{"amount": "0.37"}` (U20 a). It is prefilled from the preset when the firm publishes it (origin = preset, i.e. no row), and editable (`edited` / `set_by_user`).
    - It **is a limits input** (Esquadro): the plugin applies the same floor in local enforcement and in `trade_completed`, and receives it in `limits.commission_per_contract_side` (protocol 1.3). A change bumps `limits_version` so risk re-sends `limits`.
    - **A lower rate is looser** (Esquadro, ADR-0006 §3.7): it is scheduled for the next trading day (`scheduled_limit` kind `commission_per_contract_side`), and a higher rate applies now. Fail-closed: live usage deducts less expected commission only from the next day.
    - The bump recomputes nothing already recorded. It only enters new fills' effective commission, on the Core (gateway snapshot) and in the plugin alike, and past fills keep their rate. A fill at the exact boundary where plugin and Core diverge goes to `trades.reconcile_mismatch`.
  - `evaluation_profit_target`: `{"amount": "3000.00"}` (Q46). It is a limits input, together with `account.evaluation_start`.
    - A change applies at once and is never scheduled, because it is a target, not a limit.
  - Both are optional preset keys too (`PresetRules`; not in the `?&` required list).
- **Inputs only (Q48, provisional; ADR-0006 §3.7):**
  - A save writes the user's value, or `is_unset` for a removed rule, plus that field's `saved_at`. The **latest save of a field wins** and replaces any earlier value, whether pending or not.
  - `accounts` computes no trading day and no effective date. The superseded values are in the audit (`risk.rules_changed`).
  - A removal is kept as an `is_unset` row (a tombstone), because risk needs its `saved_at` to schedule it.
- **Accounts without a preset** (real, and manual-mode prop) store each rule they set as `origin = 'set_by_user'` (rev. 13, S13 `putAccountRiskRules`). With a preset, only `edited` / `chosen_from_conflict` occur. These are Core rules, since a cross-table `CHECK` is not possible. A real account has no `trading_day_reset` row (`risk.reset_not_editable`).
- Applying, restoring and re-selecting rewrite `account` (bumping `row_version`) and its `account_rule_value` rows **in one `accounts` transaction**: same module, so this is atomic by construction.
- Q13-b erasure deletes the rows before the account (no `CASCADE`; §5.2). The catalog is reference data and is kept.

**Account ↔ `risk.account_limits` consistency: option (B), [ADR-0006](../adr/0006-derived-state-across-modules.md)** (Esquadro, 2026-10-23):
- **Producer (`accounts`):** **every write that changes an input of the derived limits** (rev. 13: including `putAccountRiskRules` and pending-only saves) bumps `account.limits_version` and sets `limits_changed_at = now()`, in the same transaction as the change. The inputs are:
  - account create;
  - an update of firm, plan, phase, size, `firm_platform` or `preset_version_id` (re-selection);
  - `max_position_qty`;
  - any `account_rule_value` write: an edit, a `chosen_from_conflict` resolution, `restorePresetValue` (rev. 15: including `commission_per_contract_side`, sent in plugin `limits`);
  - `evaluation_start` / `evaluation_earned_before`, including "reiniciar contagem" (rev. 15);
  - a binding change or release (`venue_account_id`, rev. 16, ADR-0008);
  - a preset apply;
  - a newly adopted rule version.

  A write that changes none of them (e.g. `display_name`) does not bump. The same transaction writes `account.limits_changed {account_id, limits_version}` to `app.outbox`.
- **Consumer (`risk`):**
  - It recomputes from the accounts query port, which returns the inputs **together with their `limits_version` in one consistent read** (one statement or one snapshot). It stores that version, not the event's, as `derived_from_version`. So the stored limits always match the version they claim, even if a newer change landed after the event.
  - Idempotent: `… ON CONFLICT (account_id) DO UPDATE … WHERE risk.account_limits.derived_from_version < EXCLUDED.derived_from_version`, so a redelivered or out-of-order event never downgrades. `version` (the plugin `limits` token) increments on every applied upsert.
  - Start policy `from_now`, plus a startup reconciliation. The accounts query port `list_limits_versions() → [(account_id, limits_version)]` is compared with risk's own `derived_from_version`, and every account that differs is derived through the same path as the consumer. There is no SQL across schemas. The live account list is small and needs no index until measured.
- **Gate (routing):**
  - It reads `limits_version` (accounts port) and `derived_from_version` (risk port).
  - An action that cannot be classified as entry or exit counts as position-increasing (fail closed). The two versions are read once per instruction, at the decision.
  - The gate ships with the S13 `risk` consumer, never before.
  - If there is no row, or the row is older, position-increasing actions are skipped with `routing.limits_pending` (audited, no alert). Exits continue (P1). The code is in contract 3.0.0 (`RouteReasonCode`, `KnownAuditCode`, `details.limits_version`) and in ADR-0003 with Esquadro's next delta.
- **Plugin:** `limits` are sent only after the upsert.
- **Monitoring:** a worker job every 30 s runs the same comparison and raises an operational alert when a gap is older than 60 s, measured from `limits_changed_at`. It **only alerts** and never derives outside the consumer path.
- **Tests:** ADR-0006 §4 (eb7a6d017161), and §9 below.

**S13 alignment (rev. 13):** §5.6 now uses the S16 value types (`trailing_eod`, `daily_loss_kind`); a drawdown of type `other` is untracked and not derived.

## 6. Ingest, routing and execution records (S1 + M1)

### 6.1 Idempotency kept separate from retention

- **Duplicate with a different body (rev. 16.8, Esquadro):** `body_sha256` hashes the canonical JSON, after parsing, of a **closed list of compared fields** (rev. 16.8.1, Esquadro's final rule; Esmeril L1): `source`, `signal_type`, `action`, `position_effect`, `side`, `instrument`, `quantity`, `order`, `changes`, `refs` (absent ≠ null, as in the schema). Excluded: `message_id` (the key), `sent_at`, `reference_price`, `details` and `schema_version`. A new instruction field joins the list only by an explicit contract decision; the list lives in the instruction/v1 README and the audit code's description, not in the schema. `differing_fields` names compared fields only, never values. Expiry and everything else keep the first receipt's values. Tests: only `sent_at`, only `details` or only `reference_price` changed → no conflict; `quantity` changed → conflict `[quantity]`. The same `message_id` with the same `body_sha256` → `200` duplicate, as before. A different hash → still `200`, nothing executed, and audit `ingress.duplicate_conflict` (code from the next 5.x contract; details: source name, `message_id`, the first and second `received_at`, the names of the differing fields only, never a body). The field names come from comparing with `routing.instruction.payload`. Once that instruction is purged by retention (the receipt is kept forever), the entry carries no field names. A row from before `…061250` has `body_sha256` NULL: the `NOT VALID` constraint applies to new rows only and is never validated. Such a duplicate is treated as same-body, which concerns DEV data only. Only accepted messages have a receipt, so a duplicate of a rejected message is a new message.

ADR-0003 §5 makes `message_id` unique **per user, forever**. Q13 option (b) purges instructions after the retention window. To keep both promises, the dedupe key lives in a slim table that is never purged; it is erased only with the user:

```sql
CREATE TABLE routing.message_receipt (                   -- ≈ 100 B/row; ≈ 2.5 MB/year for a heavy user
  user_id        uuid NOT NULL REFERENCES identity.app_user(id),
  message_id     text NOT NULL CHECK (message_id ~ '^[A-Za-z0-9._:-]{1,128}$'),
  instruction_id uuid NOT NULL,
  received_at    timestamptz NOT NULL,
  PRIMARY KEY (user_id, message_id)
);
-- rev. 16.8, forward migration #6d `…061250` routing_message_receipt_body_hash (Ferro; Esquadro: duplicate with a
-- different body; expand-only):
ALTER TABLE routing.message_receipt
  ADD COLUMN body_sha256 bytea CHECK (octet_length(body_sha256) = 32);   -- sha256 of the canonical JSON of the compared fields only (Core; rev. 16.8.1)
ALTER TABLE routing.message_receipt
  ADD CONSTRAINT message_receipt_body_sha256_present CHECK (body_sha256 IS NOT NULL) NOT VALID;  -- new rows only

CREATE TABLE routing.instruction (                       -- accepted instructions; purged by retention
  id               uuid PRIMARY KEY,                     -- instruction_id (UUIDv7)
  user_id          uuid NOT NULL REFERENCES identity.app_user(id),
  source_id        uuid NOT NULL,
  source_type      text NOT NULL,
  source_key_id    text,                                 -- key_id only (webhook); NULL for master input
  connection_id    uuid,                                 -- master input via adapter
  message_id       text NOT NULL,
  schema_version   text NOT NULL,
  signal_type      text NOT NULL,
  action           text NOT NULL CHECK (action IN ('buy','sell','close','reverse','modify','cancel')),
  instrument_symbol text NOT NULL,
  contract_month   date,                                 -- resolved by the Core if omitted (P5)
  quantity         integer CHECK (quantity BETWEEN 1 AND 1000),   -- NULL / ignored on close
  position_ref     text,
  order_ref        text,
  sent_at          timestamptz NOT NULL,
  received_at      timestamptz NOT NULL,
  expiry_base      timestamptz NOT NULL,                 -- min(sent_at, received_at), ADR-0003 §7
  payload          jsonb NOT NULL,                       -- validated instruction; no credentials
  status           text NOT NULL DEFAULT 'accepted' CHECK (status IN ('accepted','routing','routed','failed')),
  UNIQUE (user_id, id),
  CHECK (expiry_base <= received_at),
  CONSTRAINT instruction_no_token CHECK (payload::text !~ 'aw[kd]_[A-Za-z0-9]{12}_[A-Za-z0-9_-]{43}')  -- ALIGN-3 format
);
CREATE INDEX instruction_user_received_idx ON routing.instruction (user_id, received_at DESC);
CREATE INDEX instruction_pending_idx ON routing.instruction (received_at) WHERE status IN ('accepted','routing');
```

**Idempotent ingest:** one transaction, READ COMMITTED.

```sql
SELECT set_config('app.user_id', $user, true);
INSERT INTO routing.message_receipt (user_id, message_id, instruction_id, received_at)
VALUES ($user, $msg, $new_id, $now)
ON CONFLICT (user_id, message_id) DO NOTHING
RETURNING instruction_id;
-- 1 row  → INSERT instruction + app.outbox('instruction.accepted') → audit 'ingress.received' → COMMIT → 200 accepted
-- 0 rows → SELECT instruction_id FROM routing.message_receipt …  → audit 'ingress.duplicate_ignored' → 200 duplicate
```

- Ingress audit codes (contract, S1 notes §3): `ingress.received` | `ingress.duplicate_ignored` | `ingress.rejected` | `ingress.rate_limited` (rev. 16.4: four codes, as in the §6.4 outcome mapping). `audit_entry.code` has no enumerating `CHECK` (codes are contract-owned); the S1 integration tests assert these four.
- A concurrent duplicate blocks on the primary-key entry until the first transaction commits, then takes the `DO NOTHING` path. Exactly one insert wins, as covered by the ADR-0001 §10 race test.
- Rejections create no receipt and no instruction. With a valid token they produce an audit entry `ingress.rejected` holding only the error code, JSON pointer and size (Q13 §8). An unknown token goes to the redacted technical log only.

### 6.2 Routing outcomes and correlation

```sql
CREATE TABLE routing.routing_decision (
  id                 uuid PRIMARY KEY,
  user_id            uuid NOT NULL,
  instruction_id     uuid NOT NULL,
  account_id         uuid NOT NULL,                      -- cross-module id
  account_name       text NOT NULL,                      -- snapshot
  leg                text NOT NULL DEFAULT 'whole' CHECK (leg IN ('whole','close','open')),  -- rev. 16.8: two-step reverse
  outcome            text NOT NULL CHECK (outcome IN ('routed','capped','skipped','rejected','refused','expired')),
  reason_code        text,                               -- e.g. 'routing.quantity_capped'
  multiplier         integer NOT NULL,                   -- snapshots used for the decision
  max_position_qty   integer NOT NULL,
  position_before    integer,                            -- signed, Core's expected state
  requested_qty      integer,                            -- quantity × multiplier
  executed_qty       integer,                            -- after cap
  decided_at         timestamptz NOT NULL,
  FOREIGN KEY (user_id, instruction_id) REFERENCES routing.instruction (user_id, id) ON DELETE CASCADE
);
CREATE UNIQUE INDEX routing_decision_leg_key ON routing.routing_decision
  (instruction_id, account_id, (CASE leg WHEN 'open' THEN 'open' ELSE 'first' END));  -- rev. 16.8: replaces UNIQUE (instruction_id, account_id)
-- Two-step reverse (rev. 16.8; ADR-0003 §4.1 r4, Esquadro): with an opposite position the close leg is decided and
-- committed at once ('close'); the opening is decided later, idempotently ('open': opened, close-only with the
-- gate's code, or routing.expired). A one-step decision is 'whole'. 'whole' and 'close' exclude each other, and a
-- pair has at most one 'open'; an 'open' row only after a 'close' row is a Core rule plus a DB test. Still
-- append-only. The outcome list narrows (no 'rejected') in the same unmerged …061200 once contract 5.2.0 freezes.
-- rev. 16.8.1 (Esmeril N3): inserts name the expression index, ON CONFLICT (instruction_id, account_id,
-- (CASE leg WHEN 'open' THEN 'open' ELSE 'first' END)) DO NOTHING; the edit keeps RLS and REVOKE UPDATE/DELETE.
-- Audit: one decision entry per (instruction, account), written at step 2 or by the TTL sweeper (expired
-- close-only); the step-1 flatten shows at once through gateway's execution.* entries (Esquadro, option 1).
CREATE INDEX routing_decision_user_account_idx ON routing.routing_decision (user_id, account_id, decided_at DESC);

CREATE TABLE routing.order_correlation (                 -- ADR-0003 §5 map — preview (S19/S20)
  user_id           uuid NOT NULL,
  source_id         uuid NOT NULL,
  order_ref         text NOT NULL,
  account_id        uuid NOT NULL,
  child_role        text NOT NULL CHECK (child_role IN ('entry','stop','target')),
  command_id        uuid NOT NULL,
  platform_order_id text,
  created_at        timestamptz NOT NULL,
  PRIMARY KEY (user_id, source_id, order_ref, account_id, child_role, command_id)
);
```

### 6.3 Gateway: commands, reports, sync state

```sql
CREATE TABLE gateway.command (                           -- also the delivery outbox for commands
  id             uuid PRIMARY KEY,                       -- command_id = UUIDv5(instruction_id, account_id, step)
  user_id        uuid NOT NULL,
  instruction_id uuid NOT NULL,
  account_id     uuid NOT NULL,
  connection_id  uuid NOT NULL,
  step           text NOT NULL CHECK (step ~ '^(place|flatten|(modify|cancel):(0|[1-9][0-9]{0,3})|orphan_flatten:[0-9a-f]{8}-[0-9a-f]{4}-[0-9a-f]{4}-[0-9a-f]{4}-[0-9a-f]{12})$'),  -- ALIGN-6, ALIGN-8
  kind           text NOT NULL CHECK (kind IN ('place','modify','cancel','flatten')),
  position_effect text CHECK (position_effect IN ('open','reduce')),  -- rev. 16.6 (G2): place only; finds pending reduces (Vigia I1)
  instrument_symbol text NOT NULL CHECK (instrument_symbol ~ '^[A-Z0-9]{1,10}$'),   -- rev. 16.6 (G3): protocol `instrument`
  contract_month    date NOT NULL CHECK (extract(day FROM contract_month) = 1),
  msg_id         uuid NOT NULL UNIQUE,                   -- rev. 16.6 (G1): envelope msg_id, the same on every retransmission
  protocol_version text NOT NULL CHECK (protocol_version ~ '^1\.(0|[1-9][0-9]*)$'),  -- the frame's envelope version
  frame          text NOT NULL,                          -- the exact envelope sent (byte-identical retransmission, ADR-0004 §4)
  depends_on     uuid REFERENCES gateway.command(id) ON DELETE SET NULL,
  expires_at     timestamptz,                            -- required for position-increasing (ADR-0004 §5)
  state          text NOT NULL CHECK (state IN ('pending','sent','acked','completed','rejected','expired','failed','unknown','undelivered')),
  payload        jsonb NOT NULL,                         -- protocol `command` body (queryable copy of the frame's payload)
  created_at     timestamptz NOT NULL,                   -- = the envelope sent_at (creation time, ADR-0004 §4)
  sent_at        timestamptz,
  acked_at       timestamptz,
  finished_at    timestamptz,
  UNIQUE (instruction_id, account_id, step),
  CHECK ((kind = 'place') = (position_effect IS NOT NULL)),
  CONSTRAINT command_frame_no_token CHECK (frame !~ 'aw[kd]_[A-Za-z0-9]{12}_[A-Za-z0-9_-]{43}'),
  CHECK (CASE split_part(step, ':', 1) WHEN 'orphan_flatten' THEN 'flatten' ELSE split_part(step, ':', 1) END = kind)
);
CREATE INDEX command_open_idx ON gateway.command (connection_id, created_at) WHERE state IN ('pending','sent');
CREATE INDEX command_user_instruction_idx ON gateway.command (user_id, instruction_id);
CREATE INDEX command_pending_reduce_idx ON gateway.command (account_id)
  WHERE position_effect = 'reduce' AND state IN ('pending','sent');   -- rev. 16.6: the clear expires them (Vigia I1)
-- step codes (ADR-0003 §5): <n> = zero-based ordinal of that kind in the routing plan of the
-- (instruction, account) pair; reverse = 'flatten' + 'place'; reduce = 'place'.
-- 'unknown': the order's state could not be determined after a recovery (execution_report status 'unknown').
-- 'orphan_flatten:<event_id>' (ALIGN-8, used only if Q-O1 = b): kind 'flatten', reason 'orphan_fill'; keeps the
-- instruction_id that created the bracket; <event_id> = execution_report of the orphan leg's fill, in canonical
-- lower-case UUID text (the UUIDv5 input must be byte-identical across Core retries).

CREATE TABLE gateway.execution_report (
  event_id          uuid PRIMARY KEY,                    -- adapter-generated; UNIQUE (ADR-0004 §4)
  user_id           uuid NOT NULL,
  account_id        uuid NOT NULL,
  connection_id     uuid NOT NULL,
  command_id        uuid,                                -- NULL for origin = external
  seq               bigint GENERATED ALWAYS AS IDENTITY (CACHE 1) UNIQUE,  -- rev. 16.6 (G3): Core arrival order
  origin            text NOT NULL CHECK (origin IN ('all_win','local_enforcement','external')),
  instrument_symbol text CHECK (instrument_symbol ~ '^[A-Z0-9]{1,10}$'),   -- rev. 16.6 (G3): protocol `instrument`, when sent
  contract_month    date CHECK (extract(day FROM contract_month) = 1),
  child_role        text CHECK (child_role IN ('entry','stop','target','exit','flatten')),   -- rev. 16.7: protocol $defs/child_role (exit = reduce, flatten)
  status            text NOT NULL CHECK (status IN ('received','working','partially_filled','filled','cancelled','rejected','expired','duplicate','unknown')),
  platform_order_id text,
  fill_id           text,
  fill_qty          integer CHECK (fill_qty >= 1),
  fill_price        NUMERIC(22,10),
  fill_operation    text CHECK (fill_operation IN ('add','amend','remove')),  -- wire default 'add', stored explicitly
  commission        NUMERIC(18,4) CHECK (commission >= 0),   -- platform-reported (ALIGN-1); NULL = not reported
  commission_rate      NUMERIC(18,4) CHECK (commission_rate >= 0),       -- rev. 15: account's commission_per_contract_side at record time (snapshot)
  commission_effective NUMERIC(18,4) CHECK (commission_effective >= 0),  -- rev. 15 (U20 a): max(reported, rate × fill_qty)
  commission_source    text CHECK (commission_source IN ('platform','configured','none')),  -- which one won; 'none' = neither exists (warning)
  position_after    integer,                             -- signed
  avg_price_after   NUMERIC(22,10),
  reason_code       text CHECK (reason_code ~ '^[a-z][a-z0-9_]*(\.[a-z][a-z0-9_]*)+$'),   -- rev. 16.6 (G5), 16.7: the protocol $defs/code pattern; never platform text
  occurred_at       timestamptz NOT NULL,
  platform_time     timestamptz,
  received_at       timestamptz NOT NULL DEFAULT now(),
  payload           jsonb NOT NULL,
  CHECK ((fill_id IS NULL) = (fill_qty IS NULL) AND (fill_id IS NULL) = (fill_price IS NULL)
         AND (fill_id IS NULL) = (fill_operation IS NULL)),
  CHECK ((fill_id IS NULL) = (commission_effective IS NULL) AND (fill_id IS NULL) = (commission_source IS NULL)),
  CHECK (commission_source IS DISTINCT FROM 'none' OR commission_effective = 0),
  CHECK ((instrument_symbol IS NULL) = (contract_month IS NULL)),
  CHECK (position_after IS NULL OR instrument_symbol IS NOT NULL)
);
-- rev. 15/15.1: gateway computes the effective commission once per fill when it records the report. The rate is
-- the one IN FORCE, read from risk's derived limits through the risk query port
-- (risk.account_limits.commission_per_contract_side; Code Review F1). It is not the latest input in accounts,
-- because a lowered rate only applies from the next trading day (ADR-0006 §3.7). Gateway, risk and the plugin's
-- `limits` therefore use the same rate, also during a derivation lag. It is a point-in-time snapshot, like
-- routing's multiplier snapshot. Trades and risk consume it, never recompute it, and a later change never rewrites
-- past fills.
CREATE UNIQUE INDEX execution_report_fill_key ON gateway.execution_report (account_id, fill_id) WHERE fill_operation = 'add';
CREATE INDEX execution_report_fill_history_idx ON gateway.execution_report (account_id, fill_id, occurred_at)
  WHERE fill_id IS NOT NULL;
CREATE INDEX execution_report_command_idx ON gateway.execution_report (command_id) WHERE command_id IS NOT NULL;
CREATE INDEX execution_report_user_account_idx ON gateway.execution_report (user_id, account_id, occurred_at DESC);
CREATE INDEX execution_report_position_idx ON gateway.execution_report (account_id, instrument_symbol, contract_month, seq DESC)
  WHERE position_after IS NOT NULL;                       -- rev. 16.6 (G3): latest position per (account, contract)

CREATE TABLE gateway.account_sync (                      -- Q14 sticky flag, single writer = gateway
  user_id     uuid NOT NULL,
  account_id  uuid PRIMARY KEY,
  sync_state  text NOT NULL CHECK (sync_state IN ('in_sync','out_of_sync')),
  flagged_at  timestamptz,
  cleared_at  timestamptz,
  cleared_by  uuid,                                      -- user who cleared (S11 AC-3, audited)
  version     integer NOT NULL DEFAULT 1 CHECK (version >= 1),  -- rev. 16.6 (S11): +1 on every flag, reason add/resolve and clear
  UNIQUE (user_id, account_id)
);

CREATE TABLE gateway.account_sync_reason (               -- reasons → welcome.accounts[].sync_reasons; rev. 16.6 (S11)
  id          uuid PRIMARY KEY,
  user_id     uuid NOT NULL,
  account_id  uuid NOT NULL,
  reason_code text NOT NULL CHECK (reason_code ~ '^sync\.[a-z][a-z0-9_]*$'),
  dedupe_key  text NOT NULL CHECK (char_length(dedupe_key) <= 200),   -- built by the Core: code + instrument/command id
  raised_at   timestamptz NOT NULL,                      -- SyncReason.detected_at
  details     jsonb NOT NULL DEFAULT '{}',               -- the other SyncReason fields, incl. core_action
  resolved_at timestamptz,                               -- the Core removed it while still out_of_sync (resolved_reasons)
  CHECK (jsonb_typeof(details) = 'object'
         AND NOT jsonb_path_exists(details, '$.**.fill_price ? (@.type() == "number")')),   -- Price as a string
  CONSTRAINT account_sync_reason_details_no_token CHECK (details::text !~ 'aw[kd]_[A-Za-z0-9]{12}_[A-Za-z0-9_-]{43}'),
  FOREIGN KEY (user_id, account_id) REFERENCES gateway.account_sync (user_id, account_id) ON DELETE CASCADE
);
CREATE UNIQUE INDEX account_sync_reason_active_key ON gateway.account_sync_reason (account_id, dedupe_key)
  WHERE resolved_at IS NULL;                             -- also serves the active-reasons read

CREATE TABLE gateway.outbound_message (                  -- reliable C→A messages other than commands (limits, instruction_result)
  msg_id        uuid PRIMARY KEY,
  user_id       uuid NOT NULL,
  connection_id uuid NOT NULL,
  type          text NOT NULL,
  payload       jsonb NOT NULL,
  protocol_version text NOT NULL CHECK (protocol_version ~ '^1\.(0|[1-9][0-9]*)$'),  -- rev. 16.6 (G1)
  frame         text NOT NULL,                           -- rev. 16.6 (G1): the exact envelope sent
  created_at    timestamptz NOT NULL,                    -- = the envelope sent_at
  acked_at      timestamptz,
  CONSTRAINT outbound_message_frame_no_token CHECK (frame !~ 'aw[kd]_[A-Za-z0-9]{12}_[A-Za-z0-9_-]{43}')
);
CREATE INDEX outbound_message_unacked_idx ON gateway.outbound_message (connection_id, created_at) WHERE acked_at IS NULL;

CREATE TABLE gateway.inbound_receipt (                   -- msg_id dedupe ≥ 24 h (ADR-0004 §4); purged daily after 48 h
  connection_id uuid NOT NULL,
  msg_id        uuid NOT NULL,
  received_at   timestamptz NOT NULL,
  PRIMARY KEY (connection_id, msg_id)
);
```

- **Rev. 16.6 (migration #9 `…061600`, Bigorna; S11 storage, Compasso; Esquadro B4 = b):**
  - **G1, byte-identical retransmission:** `command` and `outbound_message` store the exact envelope as `frame text` (`jsonb` does not keep bytes), with its `msg_id` and `protocol_version`. `created_at` is the envelope `sent_at`. A retransmission sends `frame` unchanged; `payload` is a queryable copy and is never re-serialized for the wire.
  - **G2:** `state` gains `undelivered` (72-h horizon, ADR-0004 §4: never retransmitted, audited, `sync.command_undelivered`). `position_effect` (`open` | `reduce`, `place` only) is a column, so the clear finds the account's pending reduces through `command_pending_reduce_idx` and marks them `expired` (Vigia I1).
  - **G3, per-contract expected state:** both tables carry the protocol `instrument` as `instrument_symbol` + `contract_month` (the wire form, as in `routing.instruction`; no catalog FK, §1). `execution_report.seq` gives the Core's arrival order, which the expected state and `position_ref` follow (ADR-0004 §5); `execution_report_position_idx` reads the latest `position_after` per account and contract.
  - **G4:** no gateway connection table. Gateway emits `gateway.connection_session_changed`, and identity writes `adapter_connection` status, `last_seen` and the disconnect reason (Esquadro B4 = b). Session supersede is in memory.
  - **G5:** `while_disconnected` belongs to `risk_event`, and the `local_enforcement` linkage stays in `payload`; no columns. `reason_code` has a format `CHECK` (a protocol code, never platform text).
  - **Rev. 16.7 (Bigorna):** the `execution_report` `CHECK`s copy the protocol schema exactly: `child_role` is `$defs/child_role` (`entry`, `stop`, `target`, `exit` for reduce reports, `flatten`), and `reason_code` is the `$defs/code` pattern (one or more dots). A stricter `CHECK` would fail the intake insert, and under the no-ack rule (Esquadro P1) the report would be retransmitted in an endless reconnect loop. Test: one fixture per protocol enum value inserts. (`routing.order_correlation.child_role` stays `entry`/`stop`/`target`: it records only bracket legs.)
  - **G6/S11, sync state version:** `account_sync.version` is bumped in the same transaction as every flag, reason add or resolve, and clear. The user's clear is `UPDATE … SET sync_state = 'in_sync', version = version + 1 … WHERE account_id = $1 AND version = $expected`; 0 rows → 409 `accounts.sync_state_changed` (or `accounts.not_out_of_sync` after a re-read). The same compare-and-set serves gateway's writes, so no `FOR UPDATE` is needed.
  - **S11 reasons:** one row per occurrence (`id`). `dedupe_key` (code + instrument or command id, built by the Core) keeps one **active** row per occurrence. `resolved_at` set = a `resolved_reasons` entry: the Core removed it while the account stays `out_of_sync`, and it survives a re-flag. The clear deletes **all** the account's reason rows, active and resolved, in its transaction. The ≤ 32 cap is a Core rule. `observable` is derived (connection state + `sync.bracket_integrity_unknown`), never stored. Audit `sync.*` codes use category `connection` (Esquadro A); no audit `CHECK` change.
  - **G7:** a snapshot's `connected: false` is not stored: its lasting effect is the `sync.bracket_integrity_unknown` reason, and a gateway restart re-makes connections and receives a fresh snapshot.
  - **G8, roles:** `allwin_worker` (`BYPASSRLS`) only **finds** work: `SELECT` and `UPDATE (state, sent_at, acked_at, finished_at)` on `command`, `SELECT` and `UPDATE (acked_at)` on `outbound_message`. Intake (`inbound_receipt` dedupe + `execution_report` insert + sync changes) runs on the `allwin_app` pool, in one transaction with `app.user_id` set to the session's resolved user. `inbound_receipt` has no `user_id` and no RLS (connection-scoped); `allwin_maint` purges it after 48 h. `routing.commands_requested` is consumed into `command` (`ON CONFLICT (id) DO NOTHING`), so routing never writes gateway tables.
  - **Tests:** a retransmitted frame equals the first byte for byte; a token in a `frame` or in `details` is refused; `kind = 'place'` ⇔ `position_effect`; two active reasons with the same `dedupe_key` fail, and a resolved one does not block a new one; a clear with a stale `version` changes nothing; the worker cannot `UPDATE payload` or `frame`; cross-user reads return 0 rows.
- **Fill dedupe with `fill.operation`** (adapter-protocol 1.0 delta):
  - Every report is first deduped by `event_id` (primary key), so a redelivered report is a no-op.
  - `add` is unique per `(account_id, fill_id)`, so the same execution is never counted twice.
  - `amend` and `remove` are new events with their own `event_id` for an existing `fill_id`. They are inserted as new rows, never as updates: the report history stays immutable, and the audit can show the correction.
  - The effective fill is the latest row by `occurred_at` for that `(account_id, fill_id)`, read through `execution_report_fill_history_idx`. The Core recomputes position and P&L from it, audits the correction, and flags `out_of_sync` when the expected state no longer matches (ADR-0004 §5).
  - An `amend`/`remove` whose `add` is unknown is stored, audited and flags the account. It is never dropped.
- **Sync reasons are a set**, one row per code. The adapter's window shows them, and so does the web app.
  - Known codes, from ADR-0004 §5–§6.1 and the contract examples: `sync.bracket_integrity_unknown`, `sync.bracket_orphan_leg`, `sync.bracket_double_fill`, `sync.bracket_incomplete`, `sync.simulation_reset`, `sync.disconnected_in_trade`.
  - The `CHECK` validates only the format, so a new code from the contract needs no migration. The Core rejects codes that are not in its own list.
  - A reason is removed when its condition resolves. `sync_state` stays `out_of_sync` until the user clears it (Q14; "alert and wait"). Reason history lives in the audit log, not here.
- **Position snapshots are not stored.** The gateway compares each `position_snapshot` with the expected state, raises sync reasons and writes audit entries.
  - `trigger: account_reset` raises `sync.simulation_reset`; the discontinuity is never read as fills.
  - Working orders in state `unknown` feed `sync.bracket_incomplete`.
  - If S11 (reconciliation view) needs the last snapshot per account, it adds a table then.

### 6.4 Audit and event outbox

```sql
CREATE TABLE app.outbox (                                -- domain events, written in the producer's transaction (ALIGN-2)
  id           uuid PRIMARY KEY,                         -- event id
  user_id      uuid NOT NULL,
  event_type   text NOT NULL,                            -- 'routing.decision_made', …
  occurred_at  timestamptz NOT NULL,
  payload      jsonb NOT NULL,
  published_at timestamptz,
  attempts         integer NOT NULL DEFAULT 0 CHECK (attempts >= 0),          -- delivery retries (#1b)
  last_error       text CHECK (char_length(last_error) <= 500),               -- consumer + error, never payload
  next_attempt_at  timestamptz NOT NULL DEFAULT now(),                        -- backoff 2^n s, max 600 s
  dead_lettered_at timestamptz,                                               -- after 20 attempts; kept, alerted
  seq              bigint GENERATED ALWAYS AS IDENTITY (CACHE 1) UNIQUE,        -- rev. 10 (S10-3): causal order
  CONSTRAINT outbox_last_error_no_token CHECK (last_error !~ 'aw[kd]_[A-Za-z0-9]{12}_[A-Za-z0-9_-]{43}')
);
CREATE INDEX outbox_due_idx ON app.outbox (next_attempt_at, occurred_at)
  WHERE published_at IS NULL AND dead_lettered_at IS NULL;
-- dispatcher (allwin_worker) delivers due, live events at least once; a failure sets attempts+1,
-- last_error, next_attempt_at, so it never blocks the head of the queue. allwin_worker has
-- UPDATE (published_at) from #1 and UPDATE (attempts, last_error, next_attempt_at, dead_lettered_at) from #1b.
-- Published rows are purged after 7 days, and so are rows whose type is on the no-consumer allowlist
-- (rev. 16.2, §6.4 rule 1); dead-lettered rows are never purged automatically: they are
-- an operational alert (a lost audit entry would break X-5) and are replayed or closed by an operator.

CREATE TABLE audit.audit_entry (
  id              uuid NOT NULL,
  user_id         uuid NOT NULL,
  occurred_at     timestamptz NOT NULL,                  -- from the event (deterministic on redelivery)
  recorded_at     timestamptz NOT NULL DEFAULT now(),
  source_event_id uuid NOT NULL,                         -- app.outbox id → dedupe
  category        text NOT NULL CHECK (category IN ('ingress','routing','execution','risk','connection','security','config')),
  code            text NOT NULL,                         -- stable code; clients translate (ADR-0001 §9)
  instruction_id  uuid,
  account_id      uuid,
  account_name    text,                                  -- snapshot
  source_id       uuid,
  source_key_id   text,                                  -- key_id only
  command_id      uuid,
  signal_type     text,                                  -- denormalized for S10 filters
  details         jsonb NOT NULL DEFAULT '{}'::jsonb,
  -- rev. 9 (S10 contract 2.4.0, migration #7b):
  outcome         text NOT NULL CHECK (outcome IN ('received','routed','executed','partial','skipped','refused',
                                                   'rejected','expired','unknown','info')),  -- set by the Core, never derived by clients
  is_exit         boolean CHECK (is_exit IS NULL OR category IN ('routing','execution')),  -- exit action (close, reduce, protective modify/cancel)
  message_id      text CHECK (message_id ~ '^[A-Za-z0-9._:-]{1,128}$'),                    -- same rule as routing.message_receipt
  source_name     text CHECK (char_length(source_name) BETWEEN 1 AND 60),                  -- snapshot, like account_name
  safe_to_display boolean NOT NULL,                      -- raw details may be shown; the Core decides each time (no default)
  event_seq       bigint NOT NULL,                       -- rev. 10 (S10-3): app.outbox.seq of the source event
  actor           text CHECK (actor IN ('user','system','platform_admin')),  -- rev. 12: who caused it; NULL only on rows before #7b
  PRIMARY KEY (occurred_at, id),
  UNIQUE (occurred_at, source_event_id),
  CONSTRAINT audit_entry_source_name_check  CHECK (source_id IS NULL OR source_name IS NOT NULL),   -- #7b: NOT VALID
  CONSTRAINT audit_entry_account_name_check CHECK (account_id IS NULL OR account_name IS NOT NULL), -- #7b: NOT VALID
  CONSTRAINT audit_entry_actor_check        CHECK (actor IS NOT NULL),                              -- #7b: NOT VALID (rev. 12)
  CONSTRAINT audit_entry_no_token CHECK (                -- rev. 9: also the free-text snapshots (immutable operators only)
        details::text            !~ 'aw[kd]_[A-Za-z0-9]{12}_[A-Za-z0-9_-]{43}'
    AND coalesce(account_name,'') !~ 'aw[kd]_[A-Za-z0-9]{12}_[A-Za-z0-9_-]{43}'
    AND coalesce(source_name,'')  !~ 'aw[kd]_[A-Za-z0-9]{12}_[A-Za-z0-9_-]{43}'
    AND coalesce(message_id,'')   !~ 'aw[kd]_[A-Za-z0-9]{12}_[A-Za-z0-9_-]{43}')  -- its pattern allows '_' (F2)
) PARTITION BY RANGE (occurred_at);
CREATE INDEX audit_entry_user_time_idx        ON audit.audit_entry (user_id, occurred_at DESC, event_seq DESC, id DESC);  -- rev. 10 (#7b)
CREATE INDEX audit_entry_user_instruction_idx ON audit.audit_entry (user_id, instruction_id) WHERE instruction_id IS NOT NULL;
CREATE INDEX audit_entry_user_account_idx     ON audit.audit_entry (user_id, account_id, occurred_at DESC) WHERE account_id IS NOT NULL;
-- partitions audit.audit_entry_YYYYMM created 3 months ahead + audit.audit_entry_default (must stay empty)
-- REVOKE UPDATE, DELETE FROM allwin_app; BEFORE UPDATE OR DELETE trigger raises (immutability, S10 AC-3)
```

- **Filters (S10 AC-2):** date, account, source, signal type and outcome are served by the user/time and user/account indexes, with the rest filtered inside the user's time range. No further index is added until measured (no speculative indexes). `outcome` and `category` filters stay inside the time range too (S10 contract §3).
- **S10 columns (rev. 9):**
  - `outcome` is mapped by the Core from the decision or report (S10 contract §2.1), and every new code must state its outcome. The ingress codes map as follows: `received` → `received`, `duplicate_ignored` → `skipped`, `rejected` / `rate_limited` → `rejected`. The `security.*` codes carry masked `details` and need no schema change.
  - `account_deleted` is computed when reading, from `accounts.account.archived_at` through the accounts query port, and is not stored.
  - `q` = an instruction id uses `audit_entry_user_instruction_idx`. `q` = a `message_id` is first resolved to its instruction id through `routing.message_receipt` (PK `(user_id, message_id)`), so `message_id` needs no index here.
  - **`safe_to_display` has no default**, so a producer that forgets it fails on insert instead of exposing raw JSON. Fail-closed was agreed with Compasso, who corrected S10 §3; the API value is unchanged.
- **Order (rev. 10; Code Review S10-3, agreed with Compasso, contract 3.0.0).** The S10 contract orders steps by "the Core's sequence", and today no column holds one.
  - `occurred_at` is the producer's wall clock and can tie inside one transaction.
  - `recorded_at` and `audit_entry.id` are taken when the audit consumer runs, so they follow delivery order, which redelivery and backfill can change.
  - **Proposal:**
    - `app.outbox.seq bigint GENERATED ALWAYS AS IDENTITY (CACHE 1)` is drawn when the producer inserts the event;
    - it is copied into `audit_entry.event_seq`;
    - the timeline orders by `event_seq`, falling back to `(occurred_at, id)` for rows that do not have one.
  - It is causal where it matters: a later step runs in a later transaction that saw the earlier one committed, so it draws a larger value. Events of one transaction get values in insert order.
  - Concurrent, unrelated transactions may interleave, which does not matter for one instruction's timeline.
  - `CACHE 1` keeps one sequence order across sessions.
  - Migration plan, both before production:
    - `app.outbox.seq`: a forward `app` migration. `ADD COLUMN … GENERATED ALWAYS AS IDENTITY` fills existing rows.
    - `event_seq`: added in #7b with the metadata-default trick (`DEFAULT 0` then `DROP DEFAULT`). Old rows read 0 and sort by the fallback.
  - **Contract (3.0.0):** ordering text only, with no new field.
    - The timeline orders steps by `event_seq` (causal).
    - `listActivity` orders by `(occurred_at, event_seq, id)`, newest first. Its opaque cursor carries that position. `id` is the final tie-breaker, because rows written before #7b all have `event_seq = 0`.
  - **Index:** #7b replaces `audit_entry_user_time_idx (user_id, occurred_at DESC)` with `(user_id, occurred_at DESC, event_seq DESC, id DESC)`, the exact keyset order of `listActivity`. That makes it a measured need, not a speculative index.
    - The timeline reads one instruction's rows through `audit_entry_user_instruction_idx` and sorts that handful in memory.
    - Before production, a plain `CREATE INDEX` on the partitioned parent, then `DROP INDEX` of the old one, is acceptable.
  - The rejected alternative was a declared non-causal `(occurred_at, recorded_at, id)`.
  - **Rule:** every `audit_entry` row comes from an `app.outbox` event. This is already implied by `source_event_id NOT NULL` and the dedupe key, and today's audit consumer (`core/crates/audit`) only maps events. A producer that wants an audit row (e.g. `security.*` from auth) writes an outbox event in its own transaction, never an audit row directly. So `event_seq` is always available, and the user-visible log keeps the outbox's crash safety (X-5).
- **`actor` (rev. 12; HTTP 3.0.0 `AuditEntry.actor`):** who caused the entry.
  - `user`: the account owner's own action.
  - `system`: an automatic action (route, skip, cap, expire, flatten…).
  - `platform_admin`: reserved for admin actions on the user's data (Q28 recovery).
  - It is **stored, not derived**. The same code (e.g. `accounts.updated`, `security.channel_changed`) can come from the user or an admin, so the code alone cannot tell. The producer puts it in the outbox payload.
  - The admin's identity is not a column; if a code needs it, it goes into `details`, masked per policy.
  - Nullable only for rows written before #7b, which the contract shows as absent. `audit_entry_actor_check` is added `NOT VALID`, so every new row must set it, and the old rows stay exempt (same pattern as the snapshot checks).
- **No secret in `audit_entry`** (§1 rule 9; CLAUDE.md):
  - The Core redacts before writing.
  - The storage `CHECK` refuses an `awk_`/`awd_` token in `details`, in the two free-text snapshots (`account_name`, `source_name`), since a user could paste a token into a name, and in `message_id`. Its pattern allows `_`, so a 61-character token would otherwise match.
  - Today the S1 credential guard (HTTP 1.1.2 step 5a) already refuses token-shaped strings before insert; the `CHECK` is the storage barrier.
  - **Recommendation for routing:** the same no-token `CHECK` on `routing.message_receipt.message_id` and `routing.instruction.message_id`. #6 is on `main`, so this would be a forward routing migration (Ferro). It is not part of #7b, because each module migrates only its own schema.
  - `source_key_id` is the public `key_id` only. Webhook URLs, codes, cookies and hashes are never audit fields.
- **Q13 retention is unchanged:** the new columns live in the same monthly partitions and go with them (24 months rolling; Q13-b erasure deletes the user's rows). They add about 70–200 bytes per row (`message_id` ≤ 128, `source_name` ≤ 60), which is within the Q13 §9 size margin, to be re-measured with the 1 M-row test.
- **Dispatcher rules** (Architect decision (A), 2026-10-23; replaces rev. 4 "marked published on the first pass"):
  1. **An event is never marked published unless it was consumed.**
     - Consumers are registered before start.
     - Startup **fails** when an event type that a module emits has no consumer.
     - Events whose type has no registered consumer are not fetched (`event_type = ANY(<registered types>)`). They stay visible and are counted by a metric and alert: unpublished, not dead-lettered, older than N minutes. That catches, for example, a type retired by a deploy.
     - **Exception: the no-consumer allowlist** (Esquadro, decision (b), rev. 16.2). A type may be emitted before its consumer slice exists when it is on a versioned allowlist file in the core, owned by the dispatcher owner. Each entry names the type, the future consumer slice and its start policy. Wave 1 entries: `account.limits_changed`, `account.binding_changed`, `account.changed`, `account.archived`.
       - Only a type whose future consumer is `from_now` may be listed. A type that needs `backfill` (anything that becomes an audit entry, X-5) never is, because the purge would lose it.
       - Startup does not fail for a listed type. Its rows are excluded from the old-unpublished alert and are purged after 7 days (`occurred_at`), like published rows. They are **not** marked published, so `published_at` is still set only by a consumption (rule 2).
       - A test fails when a listed type has a registered consumer, and another when a listed type is emitted by no module.
       - **Per-consumer completion** (rev. 16.3; Esquadro, Crivo R1/R2). When a consumer registers, its type leaves the allowlist (the test forces that), and the older rows of that type are neither published nor listed. Without a rule they would stay forever and trip the old-unpublished alert on the first deploy with the consumer. The same happens to a type that gains a second `from_now` consumer later. So completion is defined per consumer:
         - **`start_seq`**, stored per `(consumer, event_type)` with the rule 2 delivery tracking, is the **first** `app.outbox.seq` that the consumer may receive: the highest `seq` at registration + 1 (1 on an empty outbox). A `from_now` consumer receives only `seq >= start_seq`. No per-event "skipped" marks are written.
         - A row is **done for consumer C** when C consumed it, or when C is `from_now` and `seq < C.start_seq`. A `backfill` consumer is done only by consuming, so a type with a `backfill` consumer never ages out unconsumed (X-5).
         - A row is **done** when it is done for **every** registered consumer of its type. `published_at` (rule 2) is set when the last consumption makes the row done, and at registration for the rows that a new start cursor makes done (rev. 16.4).
         - `seq` is drawn at insert, so a transaction still open at registration can commit a row with `seq < start_seq` afterwards. That row counts as before the start and is never delivered.
         - **Order rule (Esquadro):** every `from_now` consumer records its `start_seq` **before** it runs its startup reconciliation or resync. The reconciliation reads the current state, not events, so such a row's effect is still applied: risk via `list_limits_versions` (ADR-0006 §3.2), gateway via `welcome` / `limits` re-sent on connect (connections are re-made at gateway start), and SSE clients refetch on reconnect. Waiting for in-flight transactions is not needed, because a `from_now` consumer never depends on the event itself.
         - Tests:
           - the newest row at registration (`seq = start_seq - 1`) is neither delivered nor left unpurged;
           - for a type with `from_now` consumer A and a consumer B registered later, a row between the two starts, consumed by A, becomes done and is purged; a row still in A's retry queue when B registers becomes done once A consumes it;
           - an event committed between the registration and the end of the reconciliation has its effect applied (risk derives the new version).
       - **Retention predicate** (purge job in the worker, as `allwin_maint`, batches of 1000 by `seq`), never for dead-lettered rows: `dead_lettered_at IS NULL AND occurred_at < now() - 7 days AND (published_at IS NOT NULL OR event_type = ANY($listed))`. `published_at` already includes the rows done through start cursors (rule 2, rev. 16.4). The old-unpublished alert uses the same exclusions without the age bound.
       - No schema change: the purge job receives the listed types from the core, so there is no allowlist table.
  2. **Delivery is tracked per consumer** (rev. 16.4, migration `…061400`, Brasa; DDL below). `app.outbox.published_at` is the summary "done for every registered consumer" (rule 1): the dispatcher sets it when a delivery completes a row, and at registration (step 1 below) for the rows that the new start cursor completes. So retention and the alert need no join.
     ```sql
     CREATE TABLE app.outbox_consumer (                    -- rev. 16.4: one row per (consumer, event type); system table
       consumer      text NOT NULL CHECK (consumer ~ '^[a-z][a-z0-9_.]{0,63}$'),
       event_type    text NOT NULL CHECK (event_type ~ '^[a-z][a-z0-9_]*(\.[a-z][a-z0-9_]*)+$'),
       start_policy  text NOT NULL CHECK (start_policy IN ('from_now','backfill')),
       start_seq     bigint NOT NULL CHECK (start_seq >= 1), -- first receivable seq; never changes
       registered_at timestamptz NOT NULL DEFAULT now(),
       retired_at    timestamptz,                          -- set by an operator step only (never automatically)
       PRIMARY KEY (consumer, event_type),
       CHECK (start_policy = 'from_now' OR start_seq = 1)
     );

     CREATE TABLE app.outbox_delivery (                    -- rev. 16.4: per-consumer delivery state; created on the first attempt
       consumer         text NOT NULL,
       event_id         uuid NOT NULL REFERENCES app.outbox (id) ON DELETE CASCADE,
       attempts         integer NOT NULL DEFAULT 0 CHECK (attempts >= 0),
       next_attempt_at  timestamptz NOT NULL,              -- backoff 2^n s, max 600 s
       last_error       text CHECK (char_length(last_error) <= 500),  -- error class, never payload
       delivered_at     timestamptz,
       dead_lettered_at timestamptz,                       -- after 20 attempts; kept, alerted
       PRIMARY KEY (consumer, event_id),
       CHECK (delivered_at IS NULL OR dead_lettered_at IS NULL),
       CONSTRAINT outbox_delivery_last_error_no_token CHECK (last_error !~ 'aw[kd]_[A-Za-z0-9]{12}_[A-Za-z0-9_-]{43}')
     );
     CREATE INDEX outbox_delivery_event_idx ON app.outbox_delivery (event_id);  -- rev. 16.5: the FK cascade on purge (Esmeril M2)
    CREATE INDEX outbox_pending_idx ON app.outbox (event_type, seq) WHERE published_at IS NULL;  -- per-consumer fetch
     DROP INDEX app.outbox_due_idx;                        -- retries move to outbox_delivery (index only, no data)
     -- No user_id and no RLS (system tables, like app.outbox). Grants (rev. 16.9): allwin_worker SELECT, INSERT on
     -- outbox_consumer (no UPDATE: start_seq never changes, retired_at is an operator step) and SELECT, INSERT,
     -- UPDATE (attempts, next_attempt_at, last_error, delivered_at, dead_lettered_at) on outbox_delivery, plus
     -- UPDATE (published_at, dead_lettered_at) on app.outbox (from #1/#1b); allwin_maint SELECT on both,
     -- UPDATE (retired_at) on outbox_consumer, UPDATE (attempts, next_attempt_at, last_error, dead_lettered_at) on
     -- outbox_delivery (replay) and UPDATE (dead_lettered_at) on app.outbox (legacy replay); allwin_app nothing.
     -- Wake-up (rev. 16.9, Esquadro B5): the dispatcher is woken right after a producing commit, so the
     -- routing.commands_requested → gateway hop stays well under the command TTL.
     CREATE FUNCTION app.outbox_notify() RETURNS trigger LANGUAGE plpgsql SET search_path = pg_catalog
       AS $$ BEGIN PERFORM pg_notify('allwin_outbox', ''); RETURN NULL; END $$;   -- empty payload, never an id or type
     CREATE TRIGGER outbox_notify AFTER INSERT ON app.outbox FOR EACH STATEMENT EXECUTE FUNCTION app.outbox_notify();
     -- outbox_delivery rows go with their event through the FK cascade: the referential
     -- action runs as the table owner, so allwin_maint needs no DELETE on it (verified at the #6b review).
     ```

     - **Registration** (worker startup, before the dispatcher and before any reconciliation; rule 1 order rule):
       - `INSERT … ON CONFLICT (consumer, event_type) DO NOTHING`, with `start_seq = coalesce(max(seq), 0) + 1` for `from_now` (an index-only read through `outbox_seq_key`) and `1` for `backfill`. Then the row is read back. A different `start_policy` than the code declares is a startup error, and `start_seq` never changes.
       - A row committed later with `seq < start_seq` (a transaction in flight at registration) is never delivered to that consumer. That is accepted for `from_now` (rule 1, order rule). `nextval` would not close that window either, because the in-flight row already holds its lower `seq`.
       - In the same transaction, the dispatcher sets `published_at` on the rows of that type with `seq < start_seq` that are now done for every registered consumer (rule 1). This is what releases rows from the allowlist period. It is bounded by the 7-day window of one type.
       - **A new `backfill` consumer** (rev. 16.5, Esmeril M1): when the `INSERT` actually inserts a `backfill` pair, the same transaction runs `UPDATE app.outbox SET published_at = NULL WHERE event_type = $t AND published_at IS NOT NULL AND dead_lettered_at IS NULL`. Rows that other consumers had completed are not done for the new consumer, so they go back to the fetch (one fetch shape, `outbox_pending_idx`). Only retained rows come back: purged rows are gone, which X-5 already rules out for audit types because they are never allowlisted. `published_at` is set again when the new consumer's delivery completes the row. Until it catches up, the old-unpublished alert may fire for that type; that is expected, and the alert names the consumer that is behind (rev. 16.6, Esmeril note b). Registration of a consumer runs under that consumer's advisory lock, before its dispatcher loop starts; the purge `DELETE` re-checks `published_at IS NOT NULL` on the locked row, so a reset row is never purged.
       - **Retirement:** a pair present in the table but not registered by the code fails startup. Removing a consumer is an explicit operator step that sets `retired_at`. A retired consumer no longer counts in "every registered consumer", and its rows are then re-checked as above. So a consumer that is dropped by mistake can never make its events count as done silently.
     - **Fetch per consumer C**, in batches of 100, ordered by `seq`: `o.event_type = ANY(<C's types>) AND o.seq >= c.start_seq AND o.published_at IS NULL`, joined to `outbox_delivery d` (C, `o.id`) with `d IS NULL OR (d.delivered_at IS NULL AND d.dead_lettered_at IS NULL AND d.next_attempt_at <= now())`.
       - Also `o.dead_lettered_at IS NULL` (rev. 16.5, reverses 16.4): after `…061400` nothing writes `app.outbox.dead_lettered_at`. It only carries rows dead-lettered under #1b, which stay out of the fetch and the purge, visible to the alert and to replay. A per-consumer dead letter lives only in `outbox_delivery`, so it never stops the other consumers (for example the `backfill` audit consumer, X-5).
       - Do not use `FOR UPDATE … SKIP LOCKED` on `app.outbox`: it would make consumers block or skip each other on the same row. One dispatcher loop per consumer holds `pg_try_advisory_xact_lock` on the consumer name, and the delivery row (PK) carries the state.
       - Uses `outbox_pending_idx` and the `outbox_delivery` PK.
     - **Dead letter:** after 20 attempts, `d.dead_lettered_at` is set; `app.outbox` is not touched (rev. 16.5). The row is not done for that consumer, so `published_at` stays `NULL`: it is never purged and it trips the old-unpublished alert. An operator replay resets the delivery row. A legacy #1b dead letter (`o.dead_lettered_at`) is replayed by clearing that column; the row then re-enters the fetch of every consumer.
     - **Scan cost behind a stuck consumer (Esmeril L3, accepted for V1):** a row dead-lettered or retrying for one consumer stays in `outbox_pending_idx` and is skipped by the anti-join in every other consumer's fetch. At V1 volume (< 1e6 rows in 7 days) that is bounded by the dead-letter and old-unpublished alerts and the replay/retire operator steps. A per-consumer high-water mark (`outbox_consumer` column) is a later expand-only change if the metrics show it is needed.
     - **Cutover to `…061400`** (Esmeril L4; only DEV databases exist): `outbox_consumer` starts empty, so the first startup registers every consumer. `from_now` consumers start at `max(seq) + 1`; rows still in #1b retry count as before their start (rule 1, order rule). `backfill` consumers reset `published_at` on the retained rows of their types and receive up to 7 days again, with `attempts` from 0; that is safe because consumers dedupe on the event id (audit on `source_event_id`). Legacy #1b dead letters keep `o.dead_lettered_at` (fetch filter above).
     - **No FK from `outbox_delivery.consumer`** (Esmeril N5): the key of `outbox_consumer` is `(consumer, event_type)`, and `event_type` is not repeated on the delivery row. A test asserts instead that every delivery row's `(consumer, event type of its event)` exists in `outbox_consumer`.
     - **Tests (#14, rev. 16.5):** a `backfill` consumer registered after a `from_now` consumer completed rows receives them, and `published_at` is set again only after it consumes; a purge that deletes 1000 outbox rows uses `outbox_delivery_event_idx` for the cascade (plan check, no `EXPLAIN ANALYZE` outside test DBs); a per-consumer dead letter does not stop the other consumer of the same row; a legacy row with `o.dead_lettered_at` is neither fetched nor purged; the consumer test above.
     - The #1b columns `attempts`, `last_error` and `next_attempt_at` on `app.outbox` are no longer written after `…061400`. Dropping them is a destructive change for a later migration, with user authorization.
     - **NOTIFY (rev. 16.9):** delivered at commit, at most one per producing transaction (same channel and payload). The dispatcher `LISTEN`s on one dedicated connection that runs autocommit and never holds a transaction open. If a listener held an old transaction, the notification queue (8 GB) could fill, and NOTIFY would make the **producing** transactions fail at commit. Alert when `pg_notification_queue_usage()` > 0.5. The poll stays as the fallback, so a lost notification only delays. Tests: an insert wakes the dispatcher; two inserts in one transaction give one notification; `allwin_app` inserts with the trigger.
     - **Volume (Brasa's V1 estimate):** ≤ 1e5 rows a day, < 1e6 in 7 days, and 1–3 consumers per type. The delivery table holds at most a few times the outbox, and it shrinks with the purge.
  3. **Each consumer declares a start policy at registration:**
     - `from_now`: cursor at the registration instant, no backfill. This is the default for consumers that **act** (routing, commands).
     - `backfill`: from the oldest unconsumed event. Consumers that **record** (audit) use it, so the user-visible log never misses an entry (X-5).
  4. **"Never execute late" is enforced at the point of action**, not by the dispatcher. Late entries never execute (they expire by `expiry_base` + the account's TTL; ADR-0003 §7, ADR-0004 §5). Late exits may execute and are audited as late, per T1 (pending the user's decision on the `position_ref` proposal).
- **Durability of audit:** ADR-0001 §4 publishes events "in-process, after commit". A crash between commit and handler would lose the audit entry, which breaks X-5. The outbox above closes that gap (ALIGN-2, accepted: at-least-once dispatcher, consumers dedupe; audit on `source_event_id`).

### 6.5 Activity record, retention and volume

| Table | Dedupe key | Partitioned | Retention (Q13 b) |
|---|---|---|---|
| `audit.audit_entry` | `(occurred_at, source_event_id)` | monthly | drop partitions older than N months |
| `routing.instruction`, `routing.routing_decision` | `id`, `(instruction_id, account_id)` | no | batched `DELETE` older than N months |
| `gateway.command`, `gateway.execution_report` | `id`, `event_id`, `(account_id, fill_id)` | no | batched `DELETE` older than N months, children first |
| `routing.message_receipt` | `(user_id, message_id)` | no | **kept** until the user is erased |
| `routing.order_correlation` | — | no | deleted 30 days after its position closes (S19) |
| `app.outbox` (done for every registered consumer, or of an allowlisted no-consumer type; §6.4), `gateway.inbound_receipt`, `gateway.outbound_message` (acked) | — | no | 7 days (by `occurred_at`) / 48 h / 7 days |
| `risk.applied_fill` (rev. 16.6) | `(account_id, fill_id)` | no | 30 days after its `trading_date` (`allwin_maint`) |
| `identity.pairing_code` (used, expired or invalidated) | `code_hmac` | no | 30 days |
| `identity.one_time_code` (consumed, expired or invalidated) | `id` | no | 30 days (holds the destination address and IP) |
| `identity.channel_change` (completed, cancelled or expired) | `id` | no | 30 days after its end, after its codes; it holds the new value (the audit keeps the masked values) |
| `identity.known_device` | `(user_id, device_key_hash)` | no | deleted 30 days after `revoked_at`, or when unseen for 13 months (accepted; cookie Max-Age 400 days) |
| `identity.code_send_counter` | primary key | no | deleted when `window_start + window_seconds` is older than 24 h |
| `identity.signup_code` (consumed, expired or invalidated) | `id` | no | 30 days |

Nightly maintenance (`allwin_maint`, outside trading hours):
1. create the next 3 partitions;
2. alert if the default partition holds rows;
3. detach and drop expired partitions;
4. batched deletes of 5,000 rows each, with `lock_timeout`;
5. erase users whose `closed_at` is past the grace period (Q13-b).

### 6.6 Trades (S21; rev. 14, proposed — contract 3.3.0)

```sql
CREATE TABLE trades.trade (                              -- one completed trade as the Core defines it (Q57: no schema impact)
  id                  uuid PRIMARY KEY,                  -- trade_id
  user_id             uuid NOT NULL REFERENCES identity.app_user(id),
  account_id          uuid NOT NULL,                     -- accounts.account (cross-module id)
  account_name        text NOT NULL CHECK (char_length(account_name) BETWEEN 1 AND 80),   -- snapshot
  instrument_id       uuid NOT NULL,                     -- catalog.instrument (cross-module id)
  contract_month      date CHECK (extract(day FROM contract_month) = 1),
  direction           text NOT NULL CHECK (direction IN ('long','short')),
  quantity            integer NOT NULL CHECK (quantity BETWEEN 1 AND 10000),
  entry_time          timestamptz NOT NULL,
  entry_price         NUMERIC(22,10) NOT NULL CHECK (entry_price > 0),   -- whole ticks (Core rule, see below)
  exit_time           timestamptz NOT NULL,
  exit_price          NUMERIC(22,10) NOT NULL CHECK (exit_price > 0),
  duration_seconds    integer GENERATED ALWAYS AS (floor(extract(epoch FROM (exit_time - entry_time)))::integer) STORED,  -- exit − entry (PO)
  pnl_gross           NUMERIC(18,4) NOT NULL,            -- money (X-3); plugin's value when captured, Core's when manual
  commission          NUMERIC(18,4) NOT NULL CHECK (commission >= 0),  -- rev. 15.2: Σ the trade's ALLOCATED fill shares (protocol 1.3)
  commission_source   text NOT NULL CHECK (commission_source IN ('platform','configured','mixed','manual','none')),
  pnl_net             NUMERIC(18,4) GENERATED ALWAYS AS (pnl_gross - commission) STORED,  -- main value (user, 2026-10-23)
  mae                 NUMERIC(18,4) CHECK (mae >= 0),    -- USD (Q52); NULL = not observed (partial interval, protocol 1.3) or not provided (manual, Q55); never 0 by default
  mfe                 NUMERIC(18,4) CHECK (mfe >= 0),
  origin              text NOT NULL CHECK (origin IN ('routed','master','manual','platform')),
  source_id           uuid,                              -- identity.source (cross-module id)
  source_name         text CHECK (char_length(source_name) BETWEEN 1 AND 60),            -- snapshot
  signal_type         text CHECK (signal_type IN ('trigger','trade_plan','nq_trigger','master_account')),
  instruction_id      uuid,
  command_id          uuid,
  master_account_id   uuid,                              -- S19
  master_account_name text CHECK (char_length(master_account_name) BETWEEN 1 AND 80),
  plugin_trade_id     uuid,                              -- trade_completed.trade_id (protocol 1.3), generated by the plugin; idempotency
  captured_at         timestamptz,
  create_idempotency_key uuid,                           -- manual create (same rules as createAccount)
  create_request_hash    bytea CHECK (octet_length(create_request_hash) = 32),
  row_version         integer NOT NULL DEFAULT 1 CHECK (row_version >= 1),  -- exposed for manual trades only
  created_at          timestamptz NOT NULL DEFAULT now(),
  updated_at          timestamptz NOT NULL DEFAULT now(),
  UNIQUE (user_id, id, account_id),                      -- target of trade_fill's composite FK
  CHECK (exit_time >= entry_time),
  CHECK (origin <> 'routed' OR (source_id IS NOT NULL AND source_name IS NOT NULL
                                AND signal_type IS NOT NULL AND instruction_id IS NOT NULL)),
  CHECK (origin <> 'master' OR (master_account_id IS NOT NULL AND master_account_name IS NOT NULL)),
  CHECK ((origin = 'manual') = (captured_at IS NULL)),
  CHECK ((origin = 'manual') = (plugin_trade_id IS NULL)),
  CHECK (origin <> 'manual' OR command_id IS NULL),
  CHECK (origin = 'manual' OR create_idempotency_key IS NULL),
  CHECK ((create_idempotency_key IS NULL) = (create_request_hash IS NULL)),
  CHECK (coalesce(account_name,'') || ' ' || coalesce(source_name,'') || ' ' || coalesce(master_account_name,'')
         !~ 'aw[kd]_[A-Za-z0-9]{12}_[A-Za-z0-9_-]{43}')   -- no token in user-typed snapshots
);  -- RLS: user_id = app.current_user_id()
CREATE UNIQUE INDEX trade_plugin_id_key   ON trades.trade (account_id, plugin_trade_id) WHERE plugin_trade_id IS NOT NULL;  -- never stored twice
CREATE UNIQUE INDEX trade_create_key      ON trades.trade (user_id, create_idempotency_key) WHERE create_idempotency_key IS NOT NULL;
CREATE INDEX trade_user_exit_idx          ON trades.trade (user_id, exit_time, id);           -- default sort + from/to (Q53)
CREATE INDEX trade_user_pnl_idx           ON trades.trade (user_id, pnl_net, id);             -- sort = P&L (net, both orders)
CREATE INDEX trade_user_duration_idx      ON trades.trade (user_id, duration_seconds, id);    -- sort = duration

CREATE TABLE trades.trade_fill (                         -- rev. 15.2 (protocol 1.3): the fills that make up a captured trade
  user_id               uuid NOT NULL,
  trade_id              uuid NOT NULL,
  account_id            uuid NOT NULL,
  role                  text NOT NULL CHECK (role IN ('opening','closing')),
  fill_id               text NOT NULL,                   -- gateway.execution_report.fill_id (cross-module, no FK)
  quantity              integer NOT NULL CHECK (quantity BETWEEN 1 AND 10000),  -- the part of the fill in THIS trade
  commission_allocated  NUMERIC(18,4) NOT NULL CHECK (commission_allocated >= 0),  -- Core's share of the fill's effective commission
  PRIMARY KEY (trade_id, role, fill_id),
  FOREIGN KEY (user_id, trade_id, account_id) REFERENCES trades.trade (user_id, id, account_id)  -- same user AND same account as the trade
);  -- RLS: user_id = app.current_user_id(); same immutability trigger as the trade (captured only)
CREATE INDEX trade_fill_fill_idx ON trades.trade_fill (account_id, fill_id);   -- every trade sharing a fill (reconciliation)
-- Immutability: a BEFORE UPDATE OR DELETE trigger raises unless OLD.origin = 'manual'
-- (or current_user = 'allwin_maint' for the Q13-b erasure).
```

- **Money and prices** are `NUMERIC` (§2), and decimal strings on the wire.
- **Whole ticks** cannot be a `CHECK`, because the tick size lives in `catalog`, another module.
  - Manual trades are checked by the Core, with 422 `trades.not_tick_multiple` and never rounded.
  - Captured prices are already tick-rounded at the plugin boundary (CLAUDE.md "Exact decimals").
  - A §9 test covers both.
- **Captured trades are immutable** (`routed`, `master`, `platform`). These are facts from the platform, so the trigger refuses any change and only the Q13-b erasure (`allwin_maint`) deletes them.
  - **Manual trades** are updated (with `row_version` +1) or deleted by their use case. The audit keeps the before and after values, and the values of a deleted trade, through `app.outbox` (§6.4 rule). No history table is needed.
  - If Q56 changes and captured trades become correctable, the trigger condition changes in a forward migration. The facts would better stay as they are, with the correction stored beside them, but that is for Q56's answer.
- **The provisional decisions stay out of the schema:**
  - **Q52:** `mae`/`mfe` are nullable USD amounts ≥ 0.
  - **Q53:** filters use `exit_time`, which is the leading sort index. Another time base would need only an index.
  - **Q54 (decided, rev. 15):** both values are stored: `pnl_gross`, plus `commission` and a generated `pnl_net`. Wire mapping (HTTP 3.5.0): `Trade.pnl` ← `pnl_net`, `gross_pnl` ← `pnl_gross`, `commission`, `commission_source`; `pnl_basis` stays on the wire, always `net`. The summary's `total_pnl` / `total_gross_pnl` / `total_commission` are sums, and `uncommissioned_count` counts `commission_source = 'none'`. Net is the main value, sorted and summed; gross is shown beside it. `pnl_basis` is removed. #13 is unwritten, so the table takes this shape directly.
  - **Q55:** `mae`/`mfe` are nullable, and NULL means "not provided", never 0.
  - **Q56:** the trigger condition, as above.
  - **Q57:** the Core decides what one trade is; nothing in the schema assumes it.
- **Indexes:** the contract sorts by exit time, P&L or duration, in both directions, with `id` as the tie-breaker and a keyset cursor. Each `(user_id, key, id)` B-tree serves both directions, so these are required orders, not speculative indexes. `getTradesSummary` (count, sum) scans the user's filtered range through `trade_user_exit_idx`.
  - `duration_seconds` is a stored generated column so it can be sorted and indexed. It is computed from the two instants, which matches the PO's definition.
- **Snapshots:** `account_name`, `source_name` and `master_account_name` are kept as they were, like the audit's. `account_deleted` is derived when reading. `instrument_symbol` is resolved from `catalog`, which is global, immutable reference data.
- **Retention:** trades are kept for the life of the user account (S10 AC-3) and erased with it (Q13-b). The volume is a few rows per account per day.
- **Commission on trades (rev. 15, U20 a):**
  - **Captured:** the sum of the trade's **allocated** fill shares (protocol 1.3, Crivo F1 on 1.3). A fill shared by several trades (a flip, or a FIFO partial close) has its effective commission split **pro rata by quantity**.
    - Each share is rounded to 4 decimals, half away from zero, and the trade allocated **last** from that fill takes the remainder. So the shares of a fill sum exactly to its `commission_effective`.
    - Gross and net use the same quantities.
    - `trades.trade_fill` keeps each fill's quantity and the Core's `commission_allocated`, so a fill's allocations can be summed and checked (`trade_fill_fill_idx`).
    - Fixture example: long 2, then a sell flip of 3 with commission 1.11. Trade N gets 0.74 (2/3) on the closing side plus 0.74 opening = 1.48; trade N+1 gets 0.37.
    - Fills are read through the gateway query port with the fill ids of the message.
    - A captured trade has **exactly one opening row and at least one closing row**. This is a Core rule, with a §9 test, since a cross-row `CHECK` is not possible. `commission_source` is `platform`, `configured` or `none` when every fill agrees, else `mixed`.
  - If a fill is unknown, it uses rate × quantity for that side, with source `configured` (or `none` without a rate), plus `trades.reconcile_mismatch`.
  - **Manual:** the user's typed commission (`manual`), else rate × quantity × 2 sides (`configured`), else 0 (`none`, warning).
  - Fixed at capture, like the other numbers.
- **The adapter's `trade_completed` message** (protocol 1.3, Esquadro; reliable and acked) supplies:
  - `plugin_trade_id`, the plugin's uuid. It is deduplicated like `event_id` through `trade_plugin_id_key`, so a trade is never stored twice;
  - the plugin's computed numbers (S21 AC-2): duration, P&L (gross), and `mae`/`mfe`. A partial excursion is sent **absent** (Esquadro, 1.3), so NULL `mae`/`mfe` means "not observed or not provided", and S22 excludes them from averages;
  - `opening_fill {fill_id, quantity}` + `closing_fills [{fill_id, quantity}]` (rev. 15.2), stored as `trades.trade_fill` rows.
- **Reconciliation:**
  - The Core matches the fill ids with `gateway.execution_report`, cross-module, through the gateway query port; there is no FK.
  - Unknown fills, or a P&L that differs from fills × tick value, are stored **with the plugin's numbers**, plus an audit row `trades.reconcile_mismatch`. They are never corrected silently.
  - The fill ids use the same `text` type as `gateway.execution_report.fill_id`.
  - Manual trades' `mae`/`mfe` are the user's input, or absent (Q55).
  - **Manual trades do not feed S13 risk** (Esquadro's recommendation, PO to confirm), so no trades → risk derivation is needed. If the PO decides otherwise, it follows the ADR-0006 pattern.

## 7. Indexing policy

- Every index above serves a named access path: authentication lookups, the ingest dedupe, the worker queues (partial indexes on open states), and the per-user list/filter views.
- Nothing else is added until `EXPLAIN (ANALYZE, BUFFERS)` on the dev/staging database with representative data shows a need. Production is never used for that analysis.
- All user-facing list indexes lead with `user_id`. That matches both the application filter and the RLS predicate.

## 8. Migration conventions and Wave 1 order

### Conventions

- **Tooling:** `sqlx migrate`, forward-only, in `core/migrations/`.
  - Backend Rust writes migrations; the DBA reviews every one (ADR-0002 §3).
  - **A committed migration is never edited.** sqlx checksums would fail, and environments would diverge.
- **File name:** `<YYYYMMDDHHMMSS>_<module>_<verb>_<object>.sql`, e.g. `20261101120000_identity_create_users_sessions.sql`. One logical change per file. A file touches one module's schema; the bootstrap is the only exception.
- **Ordering across parallel branches:**
  - Branches pick any timestamp while in progress.
  - **At integration the version must sort after the latest on `main`.** The DBA or DevOps renames it at review if needed.
  - A CI check fails if a new migration sorts before an already-merged one.
  - For each wave, the DBA publishes the order up front (below).
- **Transactions and locks:**
  - sqlx wraps each file in a transaction. Statements that cannot run in one (`CREATE INDEX CONCURRENTLY`, `DETACH PARTITION … CONCURRENTLY`) go in their own file with the sqlx `-- no-transaction` header.
  - Each file starts with `SET lock_timeout = '5s';`.
  - Populated tables change by expand → backfill (batched, separate file) → contract. A new `CHECK` / `NOT NULL` on a large table uses `NOT VALID`, then `VALIDATE CONSTRAINT`.
- **Naming:**
  - singular snake_case tables;
  - primary key `id`; references `<entity>_id`;
  - suffixes `*_at` (time), `*_amount` (money), `*_price` (price), `*_qty` (contracts);
  - constraints follow the PostgreSQL defaults `<table>_<cols>_{pkey,key,fkey,check}`; indexes `<table>_<purpose>_idx`.
- **Every user-owned table**, in the migration that creates it:
  - `user_id`;
  - `ENABLE` + `FORCE ROW LEVEL SECURITY` and a policy;
  - grants to `allwin_app`;
  - a composite FK to its owner when the owner is in the same module.
- **Destructive or narrowing changes** (drop, narrow, delete data) require explicit user authorization. Seeds for development are scripts, not migrations; reference data that production needs (MNQ, standard sizes) **is** a migration.

### Wave 1 order (versions assigned at integration)

| # | Migration | Slice |
|---|---|---|
| 1 | `app_bootstrap`: schemas, `app.current_user_id()`, default privileges, `app.outbox` | S1 (Ferro); merged alone and first, since S1 and S3 both build on it |
| 1b | `app_outbox_delivery_attempts`: outbox retry/backoff/dead-letter columns, `outbox_due_idx`, `GRANT UPDATE (attempts, last_error, next_attempt_at, dead_lettered_at)` to `allwin_worker` (expand only) | S1 (Ferro); depends only on #1; version re-stamped after the latest merged migration |
| 2 | `identity_create_users_sessions`: `app_user`, `invitation`, `session`, `password_reset`, `totp_credential`, `recovery_code`, `resolve_*` / `find_user_for_login` (on `main`; never edited) | S3 |
| 2b | `identity_passwordless` (rev. 6): drops the password/TOTP/recovery/reset objects and adds channels, `known_device`, `signup_code`, `channel_change`, `one_time_code`, `code_send_counter` and the new session columns; see below | S3 (Brasa); version stamped after the latest merged migration |
| 3 | `identity_create_sources`: `source`, `source_key`, `resolve_source_key` | S3 (Brasa); S1 and the M1 seed use it |
| 4 | `catalog_create_instrument`: `instrument`, `instrument_contract`, `standard_account_size` + reference rows (MNQ, 6 sizes) | S1 |
| 5a | `catalog_create_opt_in_rule_version` (`…060900`, rev. 16): `catalog.opt_in_rule_version` + the 2 generic seed rows, with fixed literal UUIDs (no DB default, §1) so the Core and tests can name them | S5 (Malha) |
| 5 | `accounts_create_account` (`…061000`, rev. 16): `account` with every rev. 8–16 column, including `limits_version` / `limits_changed_at`, `evaluation_*` and `venue_account_id` / `bound_at` | S5 (Malha) |
| 5b | `accounts_create_rule_values` (`…061050`, rev. 16; was #12): `account_rule_value` (§5.7); no FK to the catalog, so it does not wait for #11 | S5 (Malha); after #5 |
| 5c | `accounts_create_bracket_profiles` (`…061100`, rev. 16): `default_bracket_profile`, `account_bracket_profile` | S6 (Torno); after #5 |
| 6 | `routing_create_instruction`: `message_receipt`, `instruction` | S1 |
| 6b | `routing_create_decision_subscription` (`…061200`): `routing_decision`, `subscription` (with `row_version`, rev. 8) | S7 (Ferro); after #6 |
| 6c | `routing_message_id_no_token_check` (`…061300`, reserved): the §6.4 no-token `CHECK` on `routing.message_receipt` / `routing.instruction`; narrowing, so it needs explicit user authorization first | Ferro |
| 2d | `identity_source_name_check` (`…061310`, reserved, rev. 16.4): `CHECK` `source_name_shape_check` on `identity.source.name`: 1..60 chars as the contract `DisplayName`, not blank, no token shape (`NOT VALID` + `VALIDATE`); an empty or token-shaped name would dead-letter every ingress audit event of the source; narrowing, so it needs explicit user authorization first | Brasa (identity) |
| 7 | `audit_create_audit_entry`: partitioned table, first partitions, default partition, immutability trigger and grants | S1 (M1 shows "received") |
| 1c | `app_outbox_seq` (`…060610`, rev. 16.2): `app.outbox.seq` identity column (§6.4) | Ferro (`s/v1-audit-codes--core`); before #7b |
| 7b | `audit_entry_activity_columns` (`…060620`, rev. 16.2): the S10 columns and constraints on the partitioned table (see below) | Ferro (`s/v1-audit-codes--core`); after #1c |
| 2c | `identity_resolve_source_key_name` (`…060630`, rev. 16.4): `identity.resolve_source_key` also returns `source_name` (audit snapshot) | Ferro writes, Brasa approves (`s/v1-audit-codes--core`, 196a30a); after #7b |
| 14 | `app_create_outbox_consumer_tracking` (`…061400`, rev. 16.4): `app.outbox_consumer`, `app.outbox_delivery`, `outbox_delivery_event_idx` (rev. 16.5), `outbox_pending_idx`, drop `outbox_due_idx`, the `outbox_notify` trigger and narrowed worker grants (rev. 16.9) (§6.4 rule 2) | Brasa; after S3 and S5–S7 |
| 8 | `identity_create_adapter_connections`: `adapter_connection`, `pairing_code`, `device_token`, `resolve_device_token`, `resolve_pairing_code` | S3 (Brasa); S2 builds on it |
| 10 | `risk_create_account_limits` (`…061500`, rev. 13/16.6): `account_limits`, `scheduled_limit` (derived), `trading_day`, `evaluation_progress`, `drawdown_tracker`, `account_block`, with RLS; the rev. 16.6 grants (§5.6) | S13 (Malha) |
| 10b | `risk_create_applied_fill` (`…061510`, rev. 16.6): `risk.applied_fill` + maint grants (§5.6) | S13 (Malha); after #10 |
| 6d | `routing_message_receipt_body_hash` (`…061250`, rev. 16.8): `body_sha256` + `NOT VALID` presence `CHECK` (§6.1) | Ferro; expand-only |
| 9 | `gateway_create_delivery` (`…061600`, rev. 16.6): `venue_account` (without the binding, rev. 16), `command` (with `frame`, `position_effect`, contract), `execution_report` (with `seq`, contract), `account_sync` (with `version`), `account_sync_reason` (per occurrence), `outbound_message` (with `frame`), `inbound_receipt` | Bigorna (gateway); S2 / M1 |
| 11 | `catalog_create_presets` (`…061700`, rev. 10/16.8): the §5.7 catalog tables, `publish_seq`, `allwin_publisher` grants; #11b `catalog_opt_in_rule_version_release` (`…061710`) is the forward change on `opt_in_rule_version` | S16 (Torno); after S5 (#5a) |
| 11c | `accounts_create_catalog_release_cursor` (`…061720`, rev. 16.8): `accounts.catalog_release_cursor` (§5.7) | S16 (Torno); after #11 |
| 2e | `identity_admin_channel_recovery` (`…061800`, rev. 16.8): `channel_change` kind / admin fields, `one_time_code` session `CHECK` widened (§4) | S3 AC-12 (Cinzel); expand-only |
| 13 | `trades_create_trade` (rev. 14/15.2): `CREATE SCHEMA trades` + its grants, `trades.trade`, `trades.trade_fill`, indexes, RLS, immutability triggers | S21; version stamped after the latest merged migration |
| — | `routing_create_order_correlation` | S19 (V1.1) |

**Dependencies across branches:**
- #6 has a foreign key to `identity.app_user` (#2).
- S1 authenticates the webhook through `identity.source_key` / `resolve_source_key` (#3).
- So #2 and #3 (S3, Brasa) merge before #6 and #7. Until then the S1 branch builds on the S3 branch.
- No branch writes its own copy of another module's table.
- Versions on `main`: `20261023060000` #1, `…060010` #1b, `…060100`/`…060200` #2/#3, `…060300` #4, `…060500` #6, `…060600` #7.
- **Assigned by the DBA (rev. 16):** `…060700` #8 and `…060800` #2b (S3 branch), then `…060900` #5a, `…061000` #5, `…061050` #5b, `…061100` #5c, `…061200` #6b, `…061300` #6c (reserved) and `…061310` #2d (reserved, rev. 16.4).
  - `…060610` #1c and `…060620` #7b (rev. 16.2) sit between `main` and S3, because their branch merges before S3. If S3 merges first, the DBA re-stamps them to `…0614xx` before their merge.
  - `…061400` #14 is Brasa's outbox tracking (rev. 16.4). Rev. 16.6: `…061500` #10 and `…061510` #10b (Malha, S13), `…061600` #9 (Bigorna). Rev. 16.8: `…061250` #6d (Ferro), `…061700` #11, `…061710` #11b, `…061720` #11c (Torno), `…061800` #2e (Cinzel); `…061650` was assigned and withdrawn (Esquadro chose (B)). From `…061900` on, numbers are assigned on request (#13); no branch picks its own.
  - Branches merge in numeric order. A branch that must merge before a lower number is re-stamped by the DBA before the merge, never after.

**Audit columns (rev. 9): forward migration #7b `audit_entry_activity_columns`.** #7 is on `main`, so it is not edited.
- Plan:
  1. `ADD COLUMN outcome text NOT NULL DEFAULT 'info'` and `safe_to_display boolean NOT NULL DEFAULT false`, then `ALTER … DROP DEFAULT`. These are fast metadata defaults (PG ≥ 11), and dropping the defaults makes every new insert set the values.
  2. Add the nullable `is_exit`, `message_id`, `source_name` and `actor` (with its `IN` check validated, since all existing values are NULL), and `event_seq bigint NOT NULL DEFAULT 0` then `DROP DEFAULT` (rev. 10). Replace `audit_entry_user_time_idx` with `(user_id, occurred_at DESC, event_seq DESC, id DESC)`. `app.outbox.seq` comes first, in its own forward `app` migration.
  3. Add the `CHECK`s:
     - the column `CHECK`s on `outcome`, `is_exit`, `message_id` and `source_name`, validated, since only new values can fail them;
     - `audit_entry_source_name_check`, `audit_entry_account_name_check` and (rev. 12) `audit_entry_actor_check` added **`NOT VALID` and never validated**. Old rows have `source_id` with a just-added NULL `source_name`, or a NULL `account_name`, and cannot be backfilled. `NOT VALID` enforces the rule on every new insert and exempts the old rows. Verified on PostgreSQL 16.15 in a scratch database:
       - it is accepted on the partitioned parent;
       - it propagates to the existing partitions as not validated;
       - new rows are refused;
       - a partition created later gets it validated;
     - `audit_entry_no_token` dropped and re-added **validated**, now also covering `account_name`, `source_name` and `message_id`. Old rows already passed the `details` part, and the new columns are NULL on them.
- **Existing rows cannot be backfilled.** The immutability trigger refuses `UPDATE`, and a migration must not disable it. Rows written before #7b (dev and test databases only; nothing in production) keep `outcome = 'info'` and `safe_to_display = false`, which is the safe reading.
- All statements run on the partitioned parent (they cascade to the partitions) under `lock_timeout`. Each `CHECK` validates by scanning the partitions, which is small before production.

**Passwordless change (rev. 6): forward migration #2b `identity_passwordless`** (Architect decision; `20261023060100` is not rewritten, because rewriting would break the sqlx checksum of every database that already applied it).
- Order inside the file:
  1. `DROP FUNCTION` `resolve_password_reset`, `find_user_for_login`, `resolve_session` and `admin_list_users`, since their return types change, and re-`CREATE` the last three with owner and grants.
  2. `DROP TABLE` `recovery_code`, `totp_credential` and `password_reset`.
  3. `ALTER TABLE app_user`:
     - `DROP COLUMN password_hash`;
     - add the channel columns and `channel_changed_at`;
     - backfill the existing rows (dev seeds only) with `email_verified_at = created_at`, then `SET NOT NULL`;
     - then add the constraints.
  4. `CREATE` `known_device`, `code_send_counter` and `signup_code`.
  5. Sessions (needs `known_device`):
     - `DELETE FROM identity.session`: password sessions are void, and no production data exists;
     - `DROP COLUMN mfa_state, authenticated_at`;
     - add `device_id`, `login_channel` and `UNIQUE (user_id, id)` with its FK;
     - replace the `end_reason` CHECK.
  6. `CREATE` `channel_change`, then `one_time_code`: both reference `session (user_id, id)`.
  6b. `CREATE FUNCTION identity.device_key_in_use(bytea)` and `identity.resolve_one_time_code(uuid)`: owner `allwin_owner`, `EXECUTE` to `allwin_app`, owner-read policies on `known_device` and `one_time_code`.
  7. Steps 4 and 6 also cover RLS, the owner-read policy where needed, and grants.
- **Destructive changes need explicit user authorization** (DBA rule): the `DROP` statements and the session `DELETE`. There is nothing to lose: no production deploy, dev data only.
- Ferro updates `core/crates/app/src/dev.rs`, whose seed inserts `password_hash`, and the matching `core/.sqlx` query in the same change. No database recreation is needed.

## 9. Validation plan (once PostgreSQL is provisioned)

1. Migration review checklist per file:
   - RLS on, forced, with a policy;
   - grants;
   - composite FKs;
   - `NUMERIC` precision/scale;
   - no `float`;
   - `lock_timeout`;
   - no edits to committed files.
2. Integration tests (Backend Rust, against real PostgreSQL):
   - concurrent duplicate `message_id` (exactly one instruction);
   - cross-user reads and writes return nothing or fail, **with and without** application filters, to prove RLS on its own;
   - a missing `app.user_id` returns zero rows;
   - `UPDATE`/`DELETE` on `audit_entry` fails;
   - rev. 9, `audit_entry`: an insert without `outcome` or `safe_to_display` fails; (rev. 12) an insert without `actor`, or with an unknown one, fails, while pre-#7b rows with NULL `actor` remain; an unknown `outcome` fails; `is_exit` on an `ingress` entry fails; `source_id` without `source_name` fails; a token in `source_name`, `account_name` or `message_id` fails; a `message_id` with `_` that is not a token passes; #7b applies on a database with pre-existing S1 audit rows (a `source_id` and no `source_name`), and a new insert of that shape then fails;
   - the token-pattern `CHECK` rejects a planted `awk_`/`awd_` token, and the `key_id` / hash-length `CHECK`s reject malformed rows;
   - fills: a redelivered `event_id` is a no-op; a second `add` for the same `(account_id, fill_id)` fails; an `amend` and a `remove` with new `event_id`s are accepted, and the effective fill is the latest one;
   - `command.step`: `modify:0` with kind `modify` and `orphan_flatten:<uuid>` with kind `flatten` pass; `modify:0` with kind `cancel`, `orphan_flatten:<uuid>` with kind `cancel`, `place:1`, `modify:01` and an upper-case UUID fail;
   - opt-in (`automation_*` and `copy_*`): `chosen` with no choice fails; `not_required` with a choice fails; `pending_review` keeping the previous choice passes; a choice without `chosen_at` fails; a real account with `pending_review` fails;
   - `adapter_connection`: a second live connection named `Desk A`/`desk a` fails; reusing the name of an unpaired one passes; `status = 'unpaired'` without `archived_at` fails; after a simulated Core restart, rows left `connected`/`outdated` become `disconnected` with a new `status_changed_at` and an audit entry, before any adapter can connect;
   - `email_registered` and `admin_list_users` return false / no rows for a non-admin `app.user_id`;
   - `one_time_code` (rev. 6): `expires_at` beyond 10 minutes fails; `failed_attempts = 5` without `invalidated_reason = 'attempts'` fails; consumed and invalidated together fails; `step_up` without `action`, `source_rotate` or `route_enable` without `target_id`, `source_create` with a `target_id`, `login` with `session_id` and `channel_change` without `code_role` fail; a second `new_value` code in one change fails; a `new_value` code whose destination is not yet the user's address is accepted; a resend moves `expires_at` to at most `sent_at + 10 min` and keeps `failed_attempts`; 3 logins from an unknown browser do not invalidate the user's open challenge; the 60-s resend cooldown and the send budgets are identical for a registered and an unregistered identifier (counter rows, no code row needed); a `new_value` code is refused once its change is no longer `awaiting_codes`; two concurrent wrong guesses on a code at 4 failures leave it invalidated at 5, never 6; another user's challenge id resolves but its row is invisible under RLS;
   - `app_user` (rev. 6): no `email_verified_at` fails; a second channel change within 24 h of `channel_changed_at` is refused; `whatsapp_e164` without `whatsapp_verified_at` fails; a second user with the same number fails;
   - `known_device` (rev. 6/7): a cookie valid for another user is kept and a second row with the same hash is inserted for this user; after "forget device" or "sign out everywhere", the next sign-in from that browser gets a new cookie and a new row (no unique violation) and a new-device notice; signup completion creates a device without a notice;
   - `signup_code` (rev. 6): an abandoned signup leaves no `app_user` row; two concurrent confirmations on one invitation create exactly one user; a code on a revoked or expired invitation fails; `channel` other than `email` fails;
   - `channel_change` (rev. 7): a second open change for the user fails; `on_hold` without `hold_until` fails; `cancelled` without `cancelled_at` fails; a code with `code_role` and no `channel_change_id` fails; a second `new_value` code for one change fails; a `new_value` code is refused once its change is cancelled or expired; an `on_hold` change does not alter the address until a worker applies it after `hold_until`; a cancel or an expiry leaves the change's open codes invalidated with `change_closed`; a stale `awaiting_codes` row does not block a new start; a hold applied by the worker ends every session of the user; a cancel from another session of the same user works, and one from another user is invisible under RLS;
   - `delivery_state` (rev. 6): a channel-change send suppressed by a clash and one to a free value both end `sent`, and the clash writes no `auth.code_delivery_failed`; a resend resets it to `pending`; a failed send of a registered user writes `auth.code_delivery_failed`;
   - `code_send_counter` (rev. 6): a registered and an unregistered identifier hit the limit at the same count; concurrent upserts never lose a send; no column holds the address;
   - a choice without `*_rule_version_id` fails;
   - rev. 8, `account`:
     - a prop account with a preset and `firm_name`, a prop account with neither, and a manual one missing `phase` all fail;
     - a real account with `firm_platform` fails;
     - `pending_review` without `*_pending_reason`, and a reason with `chosen`, fail;
     - the same `create_idempotency_key` twice for one user fails, and for two users passes;
     - a key without a hash fails;
   - rev. 16, binding (`account.venue_account_id`):
     - a second account of the same user with the same `venue_account_id` fails (`account_venue_account_key` → 409 `accounts.platform_account_taken`); two concurrent binds of one platform account leave exactly one bound;
     - `venue_account_id` without `bound_at` fails, and an archived account that is still bound fails;
     - archiving releases the binding, and the same platform account can then be bound to another account;
     - a bind, rebind, release or archive bumps `limits_version` and writes both `account.limits_changed` and `account.binding_changed` in the same transaction (rev. 16.1);
     - `resolve` with another user's `connection_id` returns `None` (422), so no cross-user binding is possible;
   - rev. 16, `account` constraints and grants: a `display_name` of `''` or of 81 characters fails; `row_version = 0` fails; `allwin_app` `DELETE` on `account` is denied;
   - rev. 8, `opt_in_rule_version`:
     - a generic row with quotes, a firm or a non-`unknown` stance fails;
     - a second generic version 1 of the same kind fails;
     - `text_changed` without `change_reason` fails;
     - `allwin_app` cannot `UPDATE` or `DELETE`, and (rev. 16) cannot `INSERT` until the S16 migration grants it;
     - an account's `automation_rule_version_id` pointing to a `copy` row, or to an unknown id, is rejected by the Core (no FK, §1);
   - rev. 8, `account_bracket_profile`:
     - `set_by_user` with `copied_from_size` fails, and so does `set_by_user` with `customized = true`;
     - a copy without `copied_at` fails;
     - a stale `row_version` updates 0 rows;
   - rev. 8, `subscription`: a stale `row_version` updates 0 rows;
   - rev. 10, catalog:
     - a rule set missing `drawdown`, and one with a JSON-number `amount`, fail;
     - a second rule set with the same `(version, platform)` fails;
     - a `preset_version` whose release belongs to another firm fails (composite FK);
     - a `personal_device` stance `allowed` fails;
     - `allwin_app` cannot `UPDATE`/`DELETE` releases, versions or rule sets;
   - rev. 10, coherence (Core): an account with a preset whose `size_amount` differs from the preset's, whose `firm_id` is not the version's firm, or whose `firm_platform` the release does not offer, is refused;
   - rev. 10, `account_rule_value`:
     - `max_position_qty` with a value, and `drawdown` without one, fail;
     - `chosen_from_conflict` without `chose`/`chosen_at` fails;
     - a JSON-number `amount` fails;
     - another user's account is refused by the composite FK and invisible under RLS;
     - apply rewrites the account and its rows atomically (a forced failure leaves both unchanged);
   - rev. 15, commission and net:
     - a fill with reported 0.50 and a rate of 0.37 × 2 → effective 0.74, `configured`;
     - reported 1.00 → 1.00, `platform`;
     - neither → 0, `none`;
     - a non-fill report with a commission source fails;
     - `pnl_net` = gross − commission;
     - the sort by P&L uses net;
     - a mixed trade → `mixed`;
     - a rate change does not alter past fills or trades, and it does bump `limits_version` (the plugin gets the new floor);
     - a lower rate mid-day → a `scheduled_limit` row for the next trading day, while today's floor stays; a higher rate applies at once;
     - after a mid-day lowering, today's fills still use the old, higher floor in gateway (`commission_rate`), risk and plugin `limits` alike, so no `trades.reconcile_mismatch` results; from the next trading day all three use the lower rate;
     - the adjusted balance subtracts the cumulative gap since tracking start (drawdown) or since `evaluation_start` (evaluation), never only today's;
     - live daily-loss usage deducts rate × open quantity, while end-of-day and realized values do not;
     - `evaluation_start` does bump it;
     - the risk daily loss uses `realized_pnl_net`;
     - the drawdown on a sim account with no reported commission uses the adjusted balance;
     - evaluation progress uses the baseline from the snapshot, else the account size, plus `evaluation_earned_before`;
     - "reiniciar contagem" bumps `limits_version`, writes the audit row, and yields fresh progress with `target_reached_at` NULL;
     - reaching the target sets `target_reached_at` once and takes no action;
   - rev. 14, trades:
     - a routed trade without a source or instruction fails;
     - a manual trade with `captured_at`, `command_id` or `plugin_trade_id` fails;
     - a captured trade without them fails;
     - the same `plugin_trade_id` twice on one account fails (a redelivered `trade_completed` is a no-op), and on two accounts passes;
     - a manual trade with `trade_fill` rows is refused by the Core;
     - a flip fill shared by two trades: the allocated shares sum exactly to the fill's `commission_effective` (1.11 → 0.74 + 0.37), with the remainder on the trade allocated last;
     - a `trade_fill` with quantity 0 fails;
     - a `trade_fill` naming another account of the same user fails (composite FK);
     - a captured trade without an opening row, with two opening rows, or without a closing row is refused by the Core;
     - `trade_fill` rows of a captured trade cannot be updated or deleted;
     - a captured trade with absent `mae`/`mfe` is accepted;
     - a P&L mismatch with the fills keeps the plugin's numbers and emits `trades.reconcile_mismatch`;
     - `exit_time < entry_time` fails;
     - a negative `mae` fails;
     - `UPDATE` or `DELETE` of a captured trade fails for `allwin_app`; a manual one succeeds, and `allwin_maint` can delete for erasure;
     - an off-tick manual price → 422 (Core);
     - `duration_seconds` equals `exit − entry`;
     - a token in `source_name` fails;
     - another user's trades are invisible;
   - rev. 13, risk:
     - `daily_loss_kind = 'none'` with a limit fails, and `hard` without one fails;
     - `drawdown_type = 'other'` fails, because it is untracked and never derived;
     - an amount of 0 fails;
     - `reset_source = 'firm'` without a time fails;
     - a second active block fails;
     - another user's rows are invisible;
     - the derivation upsert never changes `enforcement_enabled`;
     - Q48:
       - a looser save → a `scheduled_limit` row at the start of the trading day after that field's `saved_at`, while the stricter limit holds;
       - an unrelated later save of another field does not move it;
       - looser then stricter → only the stricter value remains, and it is still in force after the reset;
       - the reset applies due rows with no `limits_version` bump and sets `effective_as_of`;
       - an active block stays until the reset even if a looser value is due;
       - a mid-day `trading_day_reset` edit is scheduled: today's `trading_day` boundaries and the active block are unchanged, and the new reset applies from the next trading day;
       - an incomparable drawdown change (`static` → `trailing_eod`) is scheduled, not applied now;
       - `is_unset` with a value, or `max_position_qty` unset, fails;
   - rev. 10, ADR-0006:
     - an entry for an account whose limits are stale (or missing) is skipped with `routing.limits_pending`, and an exit goes through;
     - a redelivered or out-of-order `limits_changed` never lowers `derived_from_version`;
     - after a crash between commit and consumption, startup reconciliation closes the gap;
     - a `limits_version` below 1 fails;
     - lowering `max_position_qty` (and editing a rule value, or restoring one) bumps `limits_version` and gates entries until risk has derived the new version; renaming the account does not bump;
   - decimal round-trip at maximum scale (`NUMERIC(22,10)`, `NUMERIC(18,4)`) through sqlx `rust_decimal`.
3. The RLS lint test (§3) runs in CI.
4. The Q13 §9 size measurement on 1 M synthetic audit rows.

## 10. Alignment with the Architect (Prumo)

ALIGN-1…8 were accepted on 2026-10-23 and are reflected in ADR-0003/0004 and the contracts.

| # | Item | Decision | Where in this model |
|---|---|---|---|
| ALIGN-1 | Money on the wire | `$defs/money` / `non_negative_money`, ≤ 14 integer + ≤ 4 fraction digits, for commission, limits, `risk_event` value/limit and `risk_state`. Prices and `price_delta` keep ≤ 10 fraction digits | §2: `NUMERIC(18,4)` |
| ALIGN-2 | Durable domain events | Transactional `app.outbox` + at-least-once dispatcher; consumers dedupe (audit on `source_event_id`) | §6.4 |
| ALIGN-3 | Token format | `key_id` = 12 `[A-Za-z0-9]`; secret = 43 base64url (32 bytes); `^aw[kd]_[A-Za-z0-9]{12}_[A-Za-z0-9_-]{43}$`. Store `key_id` + SHA-256 of the secret only | §4.1 `CHECK`s; §6.1/§6.4 no-token `CHECK`s (unanchored) |
| ALIGN-4 | Low-entropy codes | HMAC-SHA-256 with a server pepper for pairing and recovery codes. Pairing limits: 10 attempts/min per IP; 100 failures per 10 min globally → endpoint locked 10 min + alert (superseded by U11: per-IP and per-prefix limits, global threshold alert only). No per-connection counter | §4.1, §4.2 |
| ALIGN-5 | Signal-type filter | `routing.subscription` per (source, account) | §5.5 |
| ALIGN-6 | `command.step` codes | `place`, `flatten`, `modify:<n>`, `cancel:<n>`; `<n>` = zero-based ordinal of that kind in the routing plan of the (instruction, account) pair; reverse = `flatten` + `place`; reduce = `place` | §6.3 `CHECK`s |
| ALIGN-7 | `message_id` unique forever vs. retention | `routing.message_receipt` is never purged; erased only with the user | §6.1 |
| ALIGN-8 | `command_id` for a Core-initiated `flatten` with reason `orphan_fill` (only if Q-O1 = b) | Accepted 2026-10-23 (ADR-0003 §5, ADR-0004 §6.1). Step `orphan_flatten:<event_id>`, where `<event_id>` belongs to the execution report of the orphan leg's fill. Kind stays `flatten`, and the command keeps the `instruction_id` that created the bracket. The id is deterministic across Core retries and unique per orphan fill. It does not collide with the `flatten` step of that instruction's own `close`/`reverse`, which a plain `flatten` step would, making the adapter answer `duplicate` | §6.3 `step` `CHECK` and `kind` mapping |

Adapter-protocol 1.0 acceptance deltas applied in rev. 2:
- `oco_location` → `gateway.venue_account` (§5.3);
- `sync_reasons` → `gateway.account_sync_reason` (§6.3);
- `unknown` status → `gateway.execution_report.status` and `gateway.command.state`;
- `fill.operation` → `fill_operation` and the dedupe rules (§6.3);
- `account_reset` → `sync.simulation_reset`, with snapshots not stored (§6.3);
- the `routing.automation_not_accepted` gate → automation columns on `accounts.account` (§5.2).

## 11. Business questions (PO/user, through the Maestro)

| # | Question | Decision (user, 2026-10-23) |
|---|---|---|
| Q-A1 | Maximum position: one number per account, or per instrument / mini-vs-micro? | One number per account in V1 (MNQ); revisit with R1 research |
| Q-A2 | Stop/target profiles per instrument? | Keyed by instrument from day one |
| Q-A3 | List of account phases (evaluation, funded, others)? | **Open**: from R1–R3 research. `phase` stays free `text` until then; S16 adds the `CHECK` or catalog |
| Q-A4 | Signal-type filter per account, or per (source, account)? | Per (source, account) |
| Q-A5 | Contract rollover: global (admin) or per user? | Global catalog maintained by the admin, with an advance warning |
| Q-N1 | A dollar distance that is not a whole number of ticks: round which way? | Toward the entry (the tighter distance); the adjustment is shown in the audit |
| Q32 (PO) | (rev. 6, open) One account per e-mail and per WhatsApp number? The model says yes (`app_user_whatsapp_key`, like e-mail) | **Open**, user; dropping the constraint is the only change if not |
| Q35 (PO) | (rev. 6, open) Which step-up actions are on (source token create/rotate, pairing code, admin actions…) | **Open**, user. No schema impact for the 10 HTTP 3.0.0 `StepUpAction` values (rev. 11 adds `route_enable` and `account_binding_change`, target = account): Q35 switches Core settings (`step_up.q35_persistence_actions`). New actions it may add (enforcement-off, max-position changes) need a contract change and a migration extending the `action` CHECK |
| Q28 (PO) | (rev. 6, open) Admin-assisted recovery when every channel is lost | **Open**, user; not modelled until decided (§4.3) |
| Q-O1 | (ADR-0004, open) Double fill of a local bracket while NT8 was down: (a) alert and wait, (b) flatten the orphan position | **Open**, user. The schema already supports (b) (ALIGN-8); nothing changes if the user picks (a) |
