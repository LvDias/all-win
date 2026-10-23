# S10 — Activity (audit) HTTP contract notes

- **Status:** proposed (2026-10-23), for Code Review. Owner: Software Architect (Compasso). Contract version **2.4.0**, on top of 2.3.0 (S6).
- **Contract:** [`contracts/http/modules/audit.yaml`](../../../contracts/http/modules/audit.yaml).
- **Inputs:** slice-plan rev. 7, V1-S10 AC-1…AC-4 · design spec `v1-s10-audit-log.md` rev. 1 §3.3, §7, §9, §10 · [wave1-data-model.md](../data/wave1-data-model.md) §6.2–§6.5 · ADR-0003 §4–§5 · ADR-0004 §4–§5 · the S5/S6/S7 notes for the codes they emit.

## 1. Operations

| Operation | Method and path | Notes |
|---|---|---|
| `listActivity` | `GET /api/v1/activity` | Filters: `from`, `to`, `account_id[]`, `source_id[]`, `signal_type[]`, `outcome[]`, `category[]`, `code[]`, `q`. Opaque `cursor`, `limit` ≤ 100 (default 50), newest first |
| `getActivityFilterOptions` | `GET /api/v1/activity/filter-options` | Accounts incl. archived, sources incl. revoked, signal types present |
| `getActivityEntry` | `GET /api/v1/activity/{entry_id}` | Detail panel |
| `getInstructionTimeline` | `GET /api/v1/instructions/{instruction_id}/timeline` | One call: summary, `intake`, one block per account, overall `status` |

SSE: `audit.entry_recorded` (`entry_id`, `category`, `outcome`, `account_id`, `instruction_id`; no details). It drives the "n new entries" banner and the live timeline; the client refetches.

Read-only by construction: no write operation exists (S10 AC-3). Isolation: every read is scoped to the caller. Unknown, foreign and purged ids give the same 404, and foreign ids in filters match nothing.

## 2. Decisions

1. **Outcome taxonomy (Q-S10-2), final:**
   - `received`, `routed`, `executed`, `partial`, `skipped`, `refused`, `rejected`, `expired`, `unknown`, `info`.
   - The Core sets it on every entry; clients never derive it from the code.
   - `unknown` is added to the spec's list for order states that cannot be determined after a recovery. These are `execution.state_unknown` entries, which also flag the account out of sync.
   - Mapping from the stored records:
     - `routing_decision.outcome`: `routed` → `routed`; `capped` → `partial`; `skipped` / `refused` / `rejected` / `expired` keep their names.
     - Ingress: `ingress.received` → `received`; `ingress.duplicate_ignored` → `skipped`; `ingress.rejected` / `ingress.rate_limited` → `rejected`.
     - `security.*` (S3, Esquadro): `info`, except `security.sign_in_failed`, `security.step_up_failed` and `security.code_rate_limited` → `rejected`.
     - `execution_report.status`: `filled` → `executed`; `partially_filled` → `partial`; `rejected` → `rejected`; `expired` → `expired`; `unknown` → `unknown`; `received` / `working` / `cancelled` → `info`; `duplicate` is not audited.
2. **Codes are open; `KnownAuditCode` is the list so far.** `AuditEntry.code` is a pattern, not an enum, so a new slice's code never breaks an older client: it falls back to "Evento {code}". The i18n parity check (QA) iterates over `KnownAuditCode`.
   - Defined here: `ingress.*` (S1), `routing.*` decisions (ADR-0003 §4.1), `execution.*`, `connection.status_changed`, and the S5/S6/S7 config codes.
   - Added later with their own contracts: `security.*` (S3, Esquadro), `sync.*` (S11), `risk.*` (S13–S15), `notifications.*` (S18).
3. **`execution.*` codes** replace the single proposed `execution.report`, one code per state, so each has its own text:
   - `command_sent`, `bracket_calculated`;
   - `order_received`, `order_working`, `partially_filled`, `filled`, `cancelled`, `rejected`, `expired`, `state_unknown`;
   - `fill_corrected` (a `fill.operation` of `amend` / `remove`).
4. **New routing decision codes** named here: `routing.routed` (a full-quantity decision) and `routing.no_bracket_profile` (Q43). `routing.quantity_capped` keeps the spec's name, for the `capped` decision.
5. **Typed `details`:** `AuditDetailsByCode` is a type map (code → schema); the client types `details` by `code`. Money, prices and distances are decimal strings, and the UI does no arithmetic. `execution.bracket_calculated` carries the S6 AC-4 adjustment (requested distance, whole ticks, rounded flags, resulting prices).
6. **`safe_to_display`:** the raw `details` JSON appears under "technical data" only when the Core marks it safe. The catalog's rendered fields are always shown. Secrets never reach `details` (redaction + the storage `CHECK`). `safe_to_display = true` for `ingress.*` and `routing.subscription_changed` (Esquadro, agreed with Ferro). In `ingress.rejected` the `pointer` is a path only, never a value from the body.
7. **Exits:** `is_exit` on routing and execution entries lets the text say "saída" (spec §7 rule) without the client knowing the action rules.
8. **Duplicates (Q-S10-6):** `ingress.duplicate_ignored` entries carry the original `instruction_id`, so they appear in the original instruction's `intake`.
9. **Timeline:**
   - account blocks are sorted by name, each with `outcome` (last decisive step), `final`, and `quantity` (instruction qty → × multiplier → cap → executed, from the decision snapshot);
   - `status` is `completed` once every block is final.
10. **Retention (Q45):** `ActivityPage.retention_months` and `horizon` feed the info lines. CSV export (Q44) is not in V1.
11. **Cursor:** opaque and bound to the filters, keyset over `(occurred_at, id)` on the user/time index. It is not a page number, and no total count is returned (none is drawn, and it would cost a scan).

## 3. Data-model notes (for the Database Architect, through the Maestro)

- `audit.audit_entry` needs these columns for the response without joins:
  - `outcome text NOT NULL CHECK IN (…10 values…)`;
  - `is_exit boolean`;
  - `message_id text`;
  - `source_name text` (snapshot);
  - `safe_to_display boolean NOT NULL` with **no default**, so a producer that forgets it fails on insert instead of exposing raw JSON (Lastro, rev. 9).
- Rows written before migration #7b (dev/test only) read `outcome = info` and `safe_to_display = false`: the immutability trigger blocks a backfill.
- `account_deleted` is computed when reading, from `accounts.account.archived_at`.
- Filtering on `outcome` and `category` stays inside the user's time range (no new index until measured, §6.4).
- `q` (instruction id or `message_id`) uses `audit_entry_user_instruction_idx`. A `message_id` is resolved to its instruction id through `routing.message_receipt`.

## 4. Requests to other owners

- **Esquadro (S3), done:** the 14 `security.*` codes and their `details` (`DetailsSecurity*`) are in `KnownAuditCode` / `AuditDetailsByCode`. Masked values always contain `*`; no code, challenge, token, cookie, or unmasked e-mail or number.
- **S11 / S13–S15 / S18 contracts:** `sync.*`, `risk.*`, `notifications.*`, same procedure.

## 5. 4.3.0 additions (Esquadro, from the S3 implementation)

- `security.invitation_issued` (`invitation_id`, `masked_email`, `email_sent`; the admin's trail), `security.invitation_revoked`, and `security.account_created` (the new user's log), all in category `security`.
- The category of each code family is stated in `KnownAuditCode`:
  - `sources.*`, `connections.*` → config;
  - `connection.status_changed` → connection (a plugin restart is a `connection.status_changed` with `previous_status`);
  - `security.*` → security.
- **`actor` (Crivo note on 4.3.0), so the Core fills it consistently:**
  - `security.invitation_issued` / `security.invitation_revoked` → `user`: the platform admin, in their own log.
  - `security.account_created` → `user`: the invitee who completed the sign-up, in the new user's log.
  - `ingress.*` → `system` (the source is identified by `source_id`).
  - `routing.subscription_changed` → `user` with `cause = user`; `system` with `cause = account_ineligible` (Q67).
  - General rule: `user` when a person's request caused the entry; `system` for everything the Core or the plugin does on its own (routing decisions, executions, risk states, connection status, scheduled derivations, Core-side switch-offs such as Q67); `platform_admin` only for an admin acting on another user's data (Q28 recovery).
