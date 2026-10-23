# Q13 — Audit log retention and secret masking

- **Status:** Accepted (2026-10-23). The user accepted option (b) for Q13 and the 30-day erasure for Q13-b ([decisions.md](../../delivery/decisions.md), "Consolidated open questions answered"). Rev. 2 also applies the final token format (ADR-0003 §7, ADR-0004 §2, ALIGN-3)
- **Owner:** Database Architect (Lastro @ Dev3)
- **Inputs:** [product-vision.md](../../product/product-vision.md) rev. 4 §8, §11; [slice-plan.md](../../product/release-1/slice-plan.md) rev. 3 (V1-S1 AC-6, V1-S10, X-4, X-5); [ADR-0001](../adr/0001-technology-stack.md) §4, §5, §8, §11
- **Evidence level:** all sizes below are **calculated estimates**, not measurements. PostgreSQL is not yet provisioned on the dev host; §7 lists the measurement to run once it is.

## 1. Question

How long does All Win keep the user-visible audit log (every instruction received, every routing decision per account, every execution result, every automatic action with its reason)? Options in the slice plan: **(a)** life of the user account, **(b)** a fixed period such as 2 years. The design must allow retention to be configured (V1-S10 AC-3). This note also fixes the masking rule for secrets in the audit log (V1-S1 AC-6, X-4).

## 2. What is in scope of "retention"

The audit view lets the user "follow one instruction from receipt to every account's result" (V1-S10 AC-2). That needs two groups of rows:

| Group | Tables (see the Wave 1 data model) | Purpose |
|---|---|---|
| **Activity record** | `audit_entry`, `instruction`, `routing_decision`, `command`, `execution_report` | What happened, when and why. Grows with trading activity. |
| **Long-lived user data** | users, accounts, profiles, routing rules, presets copies, **trades** (performance) | Configuration and trade history. Small; grows slowly. |

**Retention only applies to the activity record.** The slim idempotency table `routing.message_receipt` (`user_id`, `message_id`; ≈ 100 B/row) is **not** purged, so ADR-0003's rule that a `message_id` is never executed twice holds forever; it is erased only with the user. Trades (performance analysis, V1-S21/S22) and configuration stay for the life of the user account in every option, so analysis is never affected by the audit retention choice.

## 3. Volume model

### Events per instruction

For one entry instruction routed to one account, the record holds about **8 rows**: routing decision, command dispatched/acknowledged, accepted, filled, stop placed, target placed, stop-or-target filled, the other cancelled (orphan protection). Partial fills, rejections and master-replication modifications add more, so heavy replication users get about 10. Each instruction adds 1 "received" row, and each day adds a few rows for connection, warning and block events.

### Scenarios per user (252 trading days/year)

| Scenario | Instructions/day | Target accounts | Audit rows/day | Audit rows/year |
|---|---|---|---|---|
| Light | 10 | 3 | ≈ 270 | ≈ 68 k |
| Typical | 30 | 10 | ≈ 2,450 | ≈ 620 k |
| Heavy (master replication, many accounts) | 100 | 20 | ≈ 20,000 | ≈ 5.1 M |

### Bytes per row

An `audit_entry` row has about 6 UUID columns (16 B each), timestamps, short codes and a small `jsonb` payload (≈ 200–400 B). With tuple overhead that is ≈ 500 B of heap. Three or four B-tree indexes add ≈ 200 B. **Planning figure: 1 KB per audit row**, which is conservative and leaves room for bloat. The operational tables of the activity record (instruction, routing decision, command, execution report) have about as many rows again, so **activity record ≈ 2 × audit**.

### Storage per user

| Scenario | Activity record / year | 2 years (option b) | 5 years (option a, example) |
|---|---|---|---|
| Light | ≈ 0.14 GB | ≈ 0.3 GB | ≈ 0.7 GB |
| Typical | ≈ 1.2 GB | ≈ 2.5 GB | ≈ 6 GB |
| Heavy | ≈ 10 GB | ≈ 20 GB | ≈ 50 GB |

Trades are tiny by comparison: about 1 row per round trip per account, ≈ 0.5 KB, so tens of MB per user per year even in the heavy case.

## 4. Cost

**Ballpark only; DevOps confirms against the chosen provider.** US block storage costs about **US$ 0.08–0.10 per GB-month**. Off-site object storage for backups costs about **US$ 0.01–0.025 per GB-month**. With 7 daily and 4 weekly compressed full backups, backup storage comes to roughly 3 × the database size. The all-in figure is therefore about **US$ 0.15 per GB of database per month**.

| Fleet example | Option (b) 2 years, steady state | Option (a) after 5 years |
|---|---|---|
| 1 user, heavy (the user's own use) | ≈ 20 GB → ≈ US$ 3/month | ≈ 50 GB → ≈ US$ 8/month |
| 10 users: 7 typical + 3 heavy | ≈ 80 GB → ≈ US$ 12/month | ≈ 190 GB → ≈ US$ 29/month |
| 50 users: 40 typical + 10 heavy | ≈ 300 GB → ≈ US$ 45/month | ≈ 740 GB → ≈ US$ 110/month |

**Conclusion: money is not what decides this at V1 scale (invite-only).** The deciding factors are the ones in §5.

## 5. Factors that matter more than cost

1. **Restore time (single VM, ADR-0001 §11).** A host failure during market hours means missed signals until the database is restored. Restore time grows with database size: tens of GB restore in minutes, hundreds of GB take hours. A bounded activity record keeps recovery time bounded. Monthly partitions also allow restoring the configuration and recent months first and older months afterwards.
2. **Data minimization (LGPD/GDPR-style principle).** The user and likely users are in Brazil. Data should be kept only as long as its purpose requires, and the retention period must be stated in the privacy policy (a release gate already owned by the user, vision §12). This favors a fixed period. *Not legal advice: confirm with the professionals drafting the privacy policy.*
3. **Usefulness of old detail.** The activity record serves support, disputes and the user's "why did this happen" questions. Prop accounts usually live weeks to months, and disputes with a firm or broker are raised soon after the event. Detail older than about 2 years is rarely useful, while trades remain available for analysis.
4. **Tax or legal record keeping.** Official trade records are the broker's and prop firm's statements. If the user wants All Win itself to serve as a tax record, Brazilian practice suggests at least 5 years, which points to option (a) or a longer period. *To confirm with an accountant.*
5. **Reversibility.** A fixed period only deletes data when the oldest month passes the limit. With 24 months, **the first deletion happens no earlier than 24 months after launch**, so the user can extend the period before any data is lost. Data deleted later cannot be recovered once backups rotate out.

## 6. Options

| | (a) Life of the user account | (b) Fixed period, 24 months rolling (configurable) | (c) 24 months online + cold archive for life of account |
|---|---|---|---|
| What happens | Everything kept online until the user account is deleted | Monthly partitions older than N months are dropped; trades/config kept for life | As (b), but each month is exported (compressed, encrypted) to object storage before being dropped; restored on request |
| Storage | Unbounded growth (≈ 1.2–10 GB/user/year) | Bounded (≈ 2.5–20 GB/user) | Bounded online; archive ≈ 10–20 % of raw size |
| Restore time | Grows forever | Bounded | Bounded |
| Privacy (minimization) | Weakest | Good, with a clear policy | Medium (data still exists) |
| Build effort | Lowest | Low: partitions + monthly purge job | Medium: export, archive storage, restore tooling, archive retention and deletion on account closure |
| Reversible later | Yes (can switch to b) | Yes until the first drop (≥ 24 months after launch) | Yes |

In **all options**:
- When the user deletes their All Win account, all their rows (activity record, trades, configuration) are deleted after a grace period stated in the privacy policy (user decision; 30 days is a common value).
- Backups age out on their own rotation.

## 7. Recommendation

**Option (b): keep the activity record online for 24 months rolling, as a configuration value (`audit_retention_months`, default 24, `null` = unlimited). Keep trades and configuration for the life of the user account.**

Why:
- It keeps the database and restore time bounded on the single VM.
- It supports a clear privacy-policy statement.
- It covers the realistic dispute and support window.
- It costs the least to build.

Since the first deletion happens no earlier than 24 months after launch, the user can still switch to (a) or (c) later without losing data. If the user wants All Win to serve as a multi-year tax or legal record, choose (a), or (c) if the database should stay small. Either way, confirm the period with the privacy-policy professionals.

**Design that holds for any option (no rework if the user changes their mind):**
- **Monthly range partitioning of `audit_entry`** (the largest table, about half of the activity record, append-only) by `occurred_at` from the **first migration**. It is cheap now and expensive to retrofit. Purging means dropping a partition (`DETACH PARTITION … CONCURRENTLY` then `DROP`), not a large `DELETE`. Old months stay pruned from queries.
- The other activity-record tables (`instruction`, `routing_decision`, `command`, `execution_report`) are **not partitioned**: they need globally unique dedupe keys (`(user_id, message_id)`, `command_id`, `event_id`), which PostgreSQL cannot enforce across partitions unless the key includes the partition column. They are purged by the same job with **batched `DELETE`s** (for example 5,000 rows per batch, outside trading hours). Their volume is a few million rows per heavy user per year, which is well within that approach. See [wave1-data-model.md](wave1-data-model.md) §6.
- **Partition maintenance in the Core** (scheduled job): create partitions 3 months ahead and drop expired ones. No new extension such as `pg_partman` is needed. A `DEFAULT` partition exists as a safety net so that an audit insert never fails the business transaction (for example, an ingest). Monitoring alerts if it ever holds rows.
- **Immutability (V1-S10 AC-3):** the application database role has `INSERT`/`SELECT` only on `audit_entry`; `UPDATE`/`DELETE` are revoked, and a trigger rejects them as a second barrier. Only the maintenance role can detach or drop partitions, and the per-user deletion on account closure runs under that role too.
- **Retention is global** (one value for all users), so purging stays partition-based. Per-user retention would require row deletes and is not recommended.

## 8. Masking rule: what the audit log may and may not contain

This applies the A3 user condition and ADR-0001 §5 to storage.

**May contain** (identify the source and the key without revealing them):
- `source_id`, `source_type`, the key's `key_id`, and the masked display form `awk_<key_id>_REDACTED`;
- `connection_id` (NinjaTrader instance) and the device token's `key_id` (never the token itself);
- the route **template** (`/ingress/v1/tradingview/{token}`), never the raw URI.

**Must never contain** (in `audit_entry`, `instruction`, any `jsonb` detail, or error text):
- the webhook token or its secret part;
- the raw request URI, query string or headers (`Authorization`);
- secret or token **hashes** — the audit log does not need them;
- device tokens, pairing codes, passwords, TOTP secrets or codes, recovery codes, session ids;
- third-party credentials (release 2).

**Instruction bodies:**
- **Accepted:** store the **validated, normalized** instruction. Schema v1 has no credential field and rejects unknown top-level properties (ADR-0001 §5).
- **Rejected** (malformed, unknown version, bad or revoked secret): **do not store the raw body**. Store the error code, the JSON pointer of the failing field, the body size and the content type. The user still sees *why* (V1-S1 AC-3) without the risk of persisting a secret the user pasted into the message by mistake.

**Defense in depth in the database:**
- a `CHECK` constraint on `audit_entry.details` and `instruction.payload` rejecting text that matches the token pattern. The regex is the final token format (ALIGN-3: `key_id` = 12 characters `[A-Za-z0-9]`, secret = 43 characters base64url), unanchored so it finds a token anywhere in the text: `aw[kd]_[A-Za-z0-9]{12}_[A-Za-z0-9_-]{43}` (webhook `awk_` and device `awd_` tokens);
- the V1-S1 redaction test also asserts that the secret is absent from `audit_entry` and `instruction`.

**Security events** (sign-in, TOTP on/off, key create/rotate/revoke, pairing/unpairing) are audited with ids only. IP address and user agent, if stored for security, follow the same retention and are shown only to the owning user.

## 9. Measurement once PostgreSQL is provisioned (to replace estimates)

On the dev database only, with synthetic data:
1. Create the proposed `audit_entry` table and indexes.
2. Insert 1 M synthetic rows shaped like the typical scenario.
3. Read `pg_total_relation_size` and `pg_relation_size` per index.
4. Time `pg_dump -Fc` and `pg_restore` for that size.

Then update §3–§5 with measured bytes per row and restore throughput.

## 10. Decisions (user, 2026-10-23)

| # | Question | Options | Recommendation → **Decision** |
|---|---|---|---|
| Q13 | How long is the activity record (audit log, instructions, executions) kept? Trades and configuration are kept for the life of the account in all options. | (a) Life of the user account; (b) 24 months rolling, configurable; (c) 24 months online + cold archive for life of account | **(b)**, reversible until the first purge (≥ 24 months after launch) → **Decided: (b), 24 months rolling, configurable** |
| Q13-b | After the user deletes their All Win account, how long until their data is erased? | e.g. 30 days grace, or immediate | 30 days, stated in the privacy policy (to confirm with the privacy-policy professionals) → **Decided: 30 days** |
