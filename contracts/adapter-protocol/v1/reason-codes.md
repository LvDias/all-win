# Adapter protocol v1 — reason code registry

- **Status:** Ready for implementation (2026-10-23). Owner: Software Architect. Normative semantics: [ADR-0004](../../../docs/architecture/adr/0004-adapter-protocol-v1.md).
- **Scope:** codes that the **adapter** emits in `execution_report.reason_code` and `error.code`. Core-emitted codes stay with their owners: `routing.*` and `ingress.*` (ADR-0003, HTTP contract), `sync.*` (ADR-0004 §6), `pairing.*` (HTTP contract).
- **Rules:**
  - Codes are stable identifiers matching `$defs/code`. Clients (web, plugin) translate them, and every code here needs a key in both catalogs (pt-BR, en).
  - A new code is an additive change: add it here first, then implement it. A receiver that does not know a code shows it as a generic failure and logs it; it never crashes.
  - `reason_code` is allowed on any status. It is **required** on `rejected`, `expired` and `duplicate`.

## 1. Command outcomes (`execution_report.reason_code`)

"Terminal" means the command reaches its final state. "depends_on" says whether a command that depends on this one may proceed (ADR-0004 §4).

| Code | Status | child_role | Terminal | depends_on | Meaning / adapter behavior |
|---|---|---|---|---|---|
| `command.duplicate` | `duplicate` | as in the repeated report | — | Same as the original command's outcome | The `command_id` was already handled. The report repeats the latest known state; nothing is executed (ADR-0004 §4). Also the answer to a stale re-delivery (§1 `command.stale`) of a `command_id` that **has** a dedupe record: the original outcome is repeated, never replaced. The Core treats `duplicate` as a non-transition |
| `command.expired` | `expired` | `entry` | yes | failure | `expires_at` passed, checked on receipt **and** right before submitting to the platform, with the skew-corrected clock. Already-submitted orders are never cancelled because of expiry |
| `command.stale` | `rejected` | as the command | yes | failure | Envelope `sent_at` (creation time, unchanged on retransmission) is older than 6 days on the skew-corrected clock (retention 7 d minus a 1-day margin). Checked on every command before execution. Used only when **no** dedupe record exists for the `command_id` (a record means `command.duplicate` with the original outcome); the `command_id` is then recorded as handled. Never executes a re-delivery after the dedupe record was pruned (ADR-0004 §4 "Bounded re-delivery") |
| `command.prerequisite_failed` | `rejected` | as the command | yes | failure | The `depends_on` command ended in failure |
| `command.account_not_mapped` | `rejected` | as the command | yes | failure | `account_id` / `platform_account` is not among the mapped accounts of this session |
| `command.account_not_destination` | `rejected` | as the command | yes | failure | The account is mapped, but not with role `destination` |
| `command.account_not_simulation` | `rejected` | as the command | yes | failure | `welcome.simulation_only` is true and the account is not a Sim/Playback account (safety rule) |
| `command.platform_disconnected` | `rejected` | as the command | yes | failure | The account's platform connection is down (a snapshot would report `connected: false`). Nothing is submitted |
| `command.instrument_unavailable` | `rejected` | as the command | yes | failure | The instrument / contract month cannot be resolved on the platform |
| `command.target_not_found` | `rejected` | the target's role | yes | failure | `modify`/`cancel` target not found. In shared mode the target must name the bracket owner's `command_id` (§3) |
| `command.already_flat` | `cancelled` | `flatten` | yes | **success** | `flatten` on an account already flat in that contract. Remaining All Win working orders are still cancelled, each with its own report |
| `command.nothing_to_reduce` | `cancelled` | `exit` | yes | failure | `place` with `position_effect: reduce` while the account is flat or the position is on the same side as the order (for example, a race with a stop/target fill). The adapter never opens or flips a position. `cancelled`, not `rejected`, so a benign race does not trip the out-of-sync rule for rejected commands. The Core still compares the outcome with its expected state: if it expected an open position on the opposite side, the account goes `out_of_sync` at once (`sync.*`), without waiting for the 60-s snapshot |
| `risk.account_blocked` | `rejected` | `entry` | yes | failure | Position-increasing command on an account blocked by local enforcement (second barrier, ADR-0004 §7) |
| `order.platform_rejected` | `rejected` | the order's role | yes | failure | The platform rejected the order. The platform's native text is never sent: it may contain account data |
| `order.protection_failed` | `rejected` | `stop` / `target` | yes (for that leg) | failure | Submitting, resizing or re-pricing a protective order failed. The entry stays as filled. The Core raises `sync.bracket_incomplete` with a top-severity alert and waits; nobody retries automatically beyond the platform itself. When the leg already existed (failed resize/re-price), `rejected` refers to the **operation**: the order keeps its last lifecycle state and is not gone. On initial submission it means the leg never existed |
| `order.protection_not_submitted` | `rejected` | `stop` / `target` | yes (for that leg) | failure | An entry fill that the adapter did not observe live: after a plugin/NT8 restart (catch-up), after a broker connection loss while NT8 kept running, or a fill received more than `max_bracket_delay` after its platform time (ADR-0004 §5 "No late bracket"). The adapter deliberately does **not** submit a late protective leg (the old fill price may be through the market): one report per leg it did not submit. The Core then sets `sync.bracket_incomplete` with a top-severity alert and waits for the user |
| `order.ambiguous_reference` | `rejected` | as the command | yes | failure | Before submitting, the adapter found **two or more** platform orders with the same `(command_id, child_role)` reference that would be adopted (any state for `place` and protective legs; live ones for `flatten`). It adopts none and submits nothing; the Core sets the account `out_of_sync` with an alert (alert and wait) |
| `order.cancel_failed` | `rejected` | the target's role | yes | failure | A cancel (from `cancel`, `flatten` or orphan protection) failed. `rejected` refers to the **cancel operation**: the order keeps its last lifecycle state (still working) and the Core must not treat it as gone |

## 2. Informational codes on other statuses

| Code | Status | child_role | Meaning |
|---|---|---|---|
| `order.orphan_cancelled` | `cancelled` | `stop` / `target` | A protective leg no longer protects the position, so it was cancelled: the position became flat, the leg was resized to 0 (partial exit), or the leg is on the wrong side after a flip (orphan-order protection, ADR-0004 §5) |
| `order.quantity_capped` | `working` | `exit`, `stop`, `target` | A quantity exceeded what the position allows and was capped: a reduce larger than the position, or a `modify` of a protective leg above \|position\| (then on the modify outcome report). The report carries the capped quantity |

## 3. Report attribution and success rules

- **Order identity.** Order lifecycle reports (`working` with new price/quantity, `cancelled`, fills) always carry the **owning** `(command_id, child_role)`.
  - Shared protection mode: the command that opened the position owns the shared stop/target. Adds get entry reports only. Resize and re-price reports carry the owner's `command_id`. Ownership ends when the position goes flat, and the next opening command becomes the owner. Re-pricing uses the owner's protection ticks.
- **Command outcome reports** (`modify` / `cancel`):
  - One report per target, under the modify/cancel command's own `command_id`.
  - It carries `child_role` = the target's role and the target's `platform_order_id`. Its status is `working` (modify applied), `cancelled` (cancel applied) or `rejected` (+ code).
  - It **never** carries `fill` or `position_after`.
  - The Core classifies a report by the action of its `command_id`: a report under a modify/cancel command is an outcome, never a new order identity.
  - Partial application (one target applied, another rejected) is reported per target, with no rollback.
- **Success for `depends_on`:**
  - `place`: the entry is `filled`.
  - `flatten`: flat (`filled`, or `cancelled` + `command.already_flat`).
  - `modify` / `cancel`: every target applied.
  - Every other terminal outcome is a failure, so the dependent command gets `command.prerequisite_failed`.
- **No second submission (crash window):** before submitting, the adapter looks on the platform for orders with the same `(command_id, child_role)` reference.
  - `place` (entry or reduce exit) and protective legs: **any** match, live or terminal, counts as already submitted. The adapter adopts it and reports its last state (filled → `filled`, cancelled/rejected → that outcome) and never resubmits. This covers submit → immediate fill → crash before "submitted" was persisted.
  - `flatten` is the only exception. A terminal match is ignored and the adapter re-evaluates the **current** position (flat → `command.already_flat`, otherwise a new close), so a retry that reuses the name still closes what remains. A live match is adopted.
  - Two or more matches that would be adopted (any state for `place`/legs, live for `flatten`) → `order.ambiguous_reference`.
- **Protective quantity invariant:** a protective leg's quantity never exceeds |position|. After any partial exit (All Win `reduce`, or an external/manual partial close in the platform), individual mode resizes legs FIFO (oldest entry first) and shared mode resizes to |position|. A leg reduced to 0 or left on the wrong side is cancelled (`order.orphan_cancelled`). A resize that fails is `order.protection_failed` (→ `sync.bracket_incomplete`).
- **Fill corrections** (`fill.operation` `amend` / `remove`): status is the order-level label recomputed after the correction (`filled` if filled quantity equals order quantity, else `partially_filled`, even when a removal brings it to 0). The report carries the original `fill_id`, and `position_after` is authoritative. Protocol 1.1 may add a dedicated status.

## 4. Protocol errors (`error.code`)

| Code | Sent by | Meaning |
|---|---|---|
| `protocol.unsupported_version` | Core | No common protocol version (close 4001–4003 follows) |
| `protocol.invalid_message` | either side | A frame that is not valid JSON or does not match the schema. `ref_msg_id` when it can be read. The message is not acked |
| `protocol.unsupported_value` | either side | A schema-valid value that the receiver does not support (for example, an unknown `order.type` on this platform) |

`protocol.*` codes appear only in `error` messages, never in `execution_report`. `detail` never contains secrets, tokens or account data.
