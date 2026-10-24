# ADR-0004 — Core ↔ platform adapter protocol v1

- **Status:** Accepted (2026-10-23) — accepted by the user with answers T1–T3 (§11). Delta 2026-10-23 (HTTP 5.1.0 review, Crivo R1(b) and n2):
  - §5, the expiry exception for the never-sent opening leg of a reverse whose flatten completed;
  - §9, the 1.3.2 note: notification receivers never refuse an unknown code or unknown params;
  - (r4) §4, what a `flatten` cancels and how its wait is bounded; §6, a flatten sent on an out-of-sync account the Core believes flat (ADR-0003 Q75); §5, `already_flat` proof also requires no later report (Vigia I2, Pêndulo).
  - (r5, 2026-10-24; gateway and S11/S13 reviews) §4 intake failure, deferred inbound types, error frames about commands, retransmission after expiry; §5 delivery-time sync check, late exits, flatten outcome reports, catch-up of missed executions (Q80); §6 supersede, unobservable snapshots, the sync-reason canon (§6.2), the reconnect evaluation window (§6.3), clearing (§6.4), binding changes during a session; §7.1 the 1.4 drawdown frame anchored at the trading-day start (Q90, Q92); §9 protocol 1.4.
  - (r6, 2026-10-24; r5 review notes and Q93/Q98) §3 who builds `limits` and the latest-wins rule; §4 dead-letter erasure and retention, unreadable account, quarantine rules, deferred consumers under RLS; §6.2 `sync.command_unknown` triggers; §6.3 first-seen check on the durable receipt; §6.4 strict "covers" and protection of accepted positions (Q98 b); §7.1 Q93 decided, exact re-anchor watermark rule.
  - (r6.1, 2026-10-24; user decisions Q100 a, Q101 yes) §6.4 reasons held at a clear, automatic return to `in_sync`, coverage from the remaining quantity and the order state; §7 a reactive flatten while blocked is reported as `flattened`.
  - (r6.2, 2026-10-24; review notes on r6.1) §3 the test-build exception for missing limits (ADR-0006 rule 8); §6.4 which order states count as working, and where the remaining quantity comes from.
  - (r6.3, 2026-10-24) §2 the V1 single-instance limit of the adapter endpoint (Bigorna); §6.2 the table order is the severity order.

  Acceptance deltas, recorded in `contracts/adapter-protocol/CHANGELOG.md`: §6.1 bracket integrity on local-OCO connections (NT8 profile F5); money type with 4 decimals (ALIGN-1); token formats (ALIGN-3); pairing limits and HMAC storage (ALIGN-4); Q18 unpairing; Q-N1 dollar rounding; `fill.operation` and `account_reset` (NT8 profile §6)
- **Owner:** Software Architect
- **Depends on:** [ADR-0001](0001-technology-stack.md) §6–7, [ADR-0003](0003-instruction-contract-v1.md)
- **Implements user decisions:**
  - Q14: an out-of-sync account stops receiving new instructions until the user clears it.
  - Q16: command TTL with a short default, editable per account.
  - Vision §8: on a disconnect, alert and wait; never auto-resync.
  - Vision §9: enforcement in the adapter as a second barrier.
  - A4: relay through the Core.
  - Also answers **D5** (NinjaTrader instance pairing code design).
- **Canonical artifacts:**
  - [`contracts/adapter-protocol/v1/messages.schema.json`](../../../contracts/adapter-protocol/v1/messages.schema.json);
  - fixtures in `contracts/adapter-protocol/v1/examples/{valid,invalid}/`;
  - [`contracts/adapter-protocol/v1/README.md`](../../../contracts/adapter-protocol/v1/README.md).
- **Consumers:**
  - Core `gateway` module;
  - `AllWin.Adapter.Core` (NinjaTrader 8 plugin);
  - `core/tools/adapter-sim`;
  - future connected-client adapters.

## 1. Context

Connected-client adapters, such as the NinjaTrader 8 AddOn, run on the trader's Windows machine. They execute Core commands on platform accounts, report every execution, enforce risk locally, and act as an Input for master accounts. The channel must survive disconnects, restarts and retransmissions **without ever executing a command twice or late**, and must never silently diverge. Server-side venues (Tradovate in release 2) do not use this protocol: they implement the same `ExecutionVenue` port inside the Core (ADR-0001 §4), reusing the same command and report semantics.

## 2. Transport and authentication

- **Connection:** `wss://<host>/adapter/v1`, a WebSocket over TLS 1.2+, **opened outbound by the adapter**. No inbound port is needed on the trader's machine.
- **Frames:** one JSON object per text frame, UTF-8, at most 256 KiB.
- **Authentication:** `Authorization: Bearer awd_<key_id>_<secret>` on the upgrade request (a device token, §8).
- **Revocation while connected:** besides reacting to the revocation event, the Core re-validates the device token of every live session periodically (default every 60 s, one indexed query) and closes it with 4002 if the token is revoked. A missed or late event therefore never keeps a revoked device connected. The revocation event and its gateway consumer ship together in V1-S2 (outbox rule: no event type is emitted without a consumer).
  - Token format (ALIGN-3): `awd_<key_id>_<secret>`, `key_id` = 12 characters `[A-Za-z0-9]`, `secret` = 43 characters base64url (32 random bytes); pattern `^awd_[A-Za-z0-9]{12}_[A-Za-z0-9_-]{43}$`.
  - The Core stores only `key_id` and a SHA-256 hash of the secret (high entropy, so no pepper is needed).
  - The token is **masked in every log**: only `key_id` is printed, same rule as ADR-0001 §5.
- **One active session per connection** (per NinjaTrader instance). A new authenticated session for the same connection closes the old one with code `4004 superseded`.
- **V1 limit: one Core instance serves `/adapter/v1` (r6.3, Bigorna).** The session state is per process: the session registry, supersede (4004), the flat-disconnect marker, a pending disconnect, the `limits` latest-wins state (§3) and the per-IP failure budget. Delivery is immediate inside that instance (an in-process wake-up), with the PostgreSQL poll tick (at most 250 ms) as the fallback. Running a second instance behind the same endpoint would break supersede and these rules, so V1 does not. Scaling out needs sticky routing by `connection_id` at the proxy plus a session lease in PostgreSQL; that is a future ADR.
- **WebSocket close codes:**

| Code | Meaning | Adapter behavior |
|---|---|---|
| 4000 | Normal shutdown (server restart) | Reconnect with backoff |
| 4001 | Invalid token | Stop. Show "pair again" |
| 4002 | Token revoked | Stop. Show "pair again" |
| 4003 | Unsupported protocol version | Stop. Show "update the plugin" |
| 4004 | Superseded by another session | Stop. Show "connected elsewhere" |
| 4005 | Heartbeat timeout | Reconnect with backoff |
| 4100 | **Sent by the adapter** (1.3): the user pressed "Desconectar" in the plugin | The adapter keeps its device token, stores a "disconnected by the user" flag and does not reconnect until the user presses "Conectar". Local enforcement continues. The Core records `Connection.disconnect_reason = user` (§6) |
| 4006 | Protocol violation (schema, ordering) | Reconnect with backoff; the Core logs the details |

## 3. Envelope and message catalogue

```json
{ "protocol_version": "1.0", "msg_id": "<uuid>", "type": "<type>", "sent_at": "<UTC Z>", "payload": { } }
```

| Type | Direction | Reliable (acked) | Purpose |
|---|---|---|---|
| `hello` | A→C | handshake | Adapter identity and version, supported protocol versions, visible platform accounts with kind (`simulation`/`playback`/`live`/`unknown`), count of pending outbound messages |
| `welcome` | C→A | handshake | Session and connection ids, negotiated version, heartbeat and ack timeouts, `simulation_only`, user locale, mapped accounts (`account_id`, roles `destination`/`master`, `sync_state`). **1.3:** `connection_name`, `time_zone`, `app_url`, `accounts[].display_name` (applied at each `welcome`) |
| `error` | both | no | Stable `code`, optional `ref_msg_id` |
| `ack` | both | no | Acknowledges up to 500 `msg_id`s |
| `heartbeat` | both | no | Liveness |
| `command` | C→A | **yes** | `place` / `modify` / `cancel` / `flatten` on one account |
| `limits` | C→A | **yes** | Risk configuration per account for local enforcement, versioned |
| `account_state` | C→A | no (latest wins) | **1.1:** an account's `sync_state` and full `sync_reasons` after a change during the session (S11 AC-1); sent only on sessions that negotiated 1.1. Applied when its `changed_at` is newer than or equal to the state held (an equal value means the later-received message wins; WebSocket preserves order) |
| `instruction_result` | C→A | **yes** | Outcome of a master `instruction` (accepted, duplicate, rejected + code) |
| `execution_report` | A→C | **yes** | Every order event on mapped accounts: All Win, local enforcement and external origins |
| `position_snapshot` | A→C | **yes** | Positions and working orders per mapped account |
| `instruction` | A→C | **yes** | Master-account input: a standard instruction (ADR-0003), validated with the same schema |
| `risk_event` | A→C | **yes** | Warning (80 %), limit reached, flattened, orders cancelled, blocked/unblocked, blocked-account order cancelled |
| `risk_state` | A→C | no (latest wins) | Live P&L and remaining limits for display; throttled to at most 2 per second per account. **1.3:** `realized_pnl` is net, plus `commissions_today` and `commission_source` (§7) |
| `notifications` | C→A | no (latest wins) | **1.3:** snapshot after `welcome`: up to 200 notifications plus the authoritative unread count (S18) |
| `notification` | C→A | **yes** | **1.3:** upsert of one notification by `notification_id` and monotonic `revision` |
| `notification_read` | A→C | **yes** | **1.3:** notifications the user marked as read in the plugin |
| `trade_completed` | A→C | **yes** | **1.3:** a completed trade computed by the plugin (S21 AC-2), reconciled by the Core with its fills. Fill references carry quantities. A fill shared by several trades (a flip, or a FIFO partial close in individual mode) has its effective commission allocated pro rata by quantity: 4 decimals, half away from zero, and the trade allocated last gets the remainder, so the shares sum to the fill's commission (Crivo F1). The allocation order is the consumption order: closing quantities first (FIFO), then the flip remainder (Pêndulo LA1) |

**Handshake:**

1. The adapter sends `hello` first. The Core answers `welcome`, or `error` followed by a close (4001–4003).
2. After `welcome`, the Core sends `limits` for all mapped accounts. `welcome` itself carries no limits.
   - **Who builds them (r6):** the gateway, from the risk module's current-limits query port (a consumer-side port in the gateway; the adapter is wired in `app` over the risk read model). The gateway computes nothing. An account with no derived limits yet gets no `limits` frame; routing keeps its entries skipped (`routing.limits_pending`, ADR-0003 §4.1), except in the end-to-end test build of ADR-0006 rule 8, where gate 8 passes for simulation accounts only.
   - **During the session** the gateway sends a new `limits` when it consumes `risk.limits_derived` (ADR-0006).
   - **Latest-wins (r6):** `limits` is reliable but superseding. The gateway sends a version only if it is higher than the last one sent in that session. An unacked `limits` frame in the outbox is dropped only when every account in it has a newer version; otherwise the Core rebuilds it with the items that are still the newest (protocol 1.4.1). At every handshake the frame is rebuilt from the current limits for the negotiated version. A frame built for 1.4 (with drawdown, §7.1) is therefore never retransmitted to a session below 1.4. This is an exception to the byte-identical retransmission rule (§4), allowed because the adapter applies only a version higher than the one it holds (§7).
3. The adapter must send a `position_snapshot` (`trigger: reconnect`) **before** the Core sends any new `command` in that session (§6). Adapter order on every (re)connect: `hello` → `welcome` → apply `limits` → retransmit its outbox in original order → `position_snapshot` → process commands. A command received before the adapter's snapshot is sent is queued, never rejected, and processed afterwards with the expiry re-check of §5; it is logged as a Core protocol violation.
4. The Core picks the highest protocol version listed in both `hello.supported_protocol_versions` and its own support.
5. Platform accounts the user has not mapped are ignored by the adapter: no commands, no reports, no enforcement.

## 4. Delivery guarantees

- **At-least-once delivery with idempotent receivers:**
  - **Reliable messages** stay in the sender's **durable outbox** until acknowledged. The adapter outbox is a local file store under the NinjaTrader user folder, chosen in the spike; the Core outbox is a PostgreSQL table.
  - Unacked messages are retransmitted after `ack_timeout_ms` (default 5000) and on every reconnect, in original order. A retransmission is **byte-identical** to the first send: same `msg_id` and same envelope `sent_at`, so `sent_at` is the message's creation time.
  - Receivers dedupe by `msg_id` for at least 24 h.
  - **Durable store integrity (adapter journal/outbox):** every record is written ahead of the platform call or ack it allows, with a checksum. If only the **last** line fails its checksum, it is a torn write from a power loss: it was never confirmed (no platform call, no ack followed it), so the adapter drops it, logs a warning and shows it in the plugin. Damage anywhere else is corruption: the adapter refuses to start until the user intervenes. Dropping a torn tail cannot cause a double execution, because the Core re-delivers the unacked command and the adoption check (§5 "No second submission") finds any order already sent.
- **Business idempotency on top of transport dedupe:**
  - **`command_id`** is deterministic (ADR-0003 §5). The adapter keeps a **durable record of every `command_id` it has acted on** for at least the current and previous trading day, implemented as **at least 7 days of wall clock** (clarified 2026-10-23: weekends and holiday sessions; exits carry no `expires_at`, so this record and the staleness guard below protect them against a late re-delivery). Pruning happens at startup or on a timer, never while a command is in flight. A command already handled is acknowledged and answered with an `execution_report` `status: duplicate` that repeats the latest known state; it is never executed again.
  - **Bounded re-delivery (never execute twice after a long outage; Code Review 4):** dedupe records are finite, so both sides bound how old a command can be when it is delivered. Neither side relies only on the other's clock.
    - **Core, retransmission horizon H = 72 h:** a command not acked within 72 h of its creation is never retransmitted. It becomes `undelivered`, is removed from the outbox and audited, and the account goes `out_of_sync` (`sync.command_undelivered`) with an alert.
    - **Adapter, staleness guard:** on every command, before execution, the adapter refuses a command whose envelope `sent_at` (creation time, unchanged on retransmission) is older than **6 days** on its skew-corrected clock. That is the retention R = 7 days minus a 1-day margin. If a dedupe record exists for that `command_id`, the answer is `duplicate` repeating the original outcome (a filled command stays filled; the Core treats it as a non-transition). Otherwise it reports `rejected` + `command.stale` and records the `command_id` as handled. Either way, a re-delivery can never execute.
    - **Invariant:** retention R (7 d) > staleness limit (6 d) > H (72 h), so R > H + margin. Any change to one of the three must keep this order.
  - **`event_id`** is generated once per platform event by the adapter and persisted in its outbox before sending. The Core enforces `UNIQUE (event_id)`.
  - **`fill_id`** (the platform execution id) is unique per account. The Core also dedupes fills by `(account_id, fill_id)`, which protects against an adapter that loses its outbox and replays platform history.
- **Ack semantics:**
  - Acking a `command` means "durably recorded by the adapter". The result arrives later as `execution_report`s: `received`, then `working`/`filled`/`rejected`/...
  - Acking an `execution_report` means "durably stored by the Core".
  - **Intake failure (r5):** if the Core cannot store an A→C reliable message (for example a query port it needs fails), it does not ack it. It also processes no later message of that session, and closes the session with 4000, reason `intake_retry`. The adapter reconnects and retransmits in original order, so nothing is lost and order is kept. This is a normal disconnect for §6. **Bound:** after 3 consecutive `intake_retry` closes for the same `msg_id`, the Core stores that frame raw in a gateway dead-letter table, acks it, and lets the queue move. It flags the affected account `out_of_sync` (`sync.command_unknown`: the effect of that report is unknown) when the frame names an account, and raises a critical operational alert in every case. A message the Core can never store therefore never stalls a connection's later reports. When the frame has no `command_id`, the reason applies to the account it names, with the instrument as a detail. **When the account cannot be read from the frame (r6, Vigia L5)**, every mapped account of that connection is flagged with that reason, because any of them may be affected.
    - **The dead-letter row may be the only copy.** After the ack the adapter removes the frame from its outbox, and its journal already counts a fill in it as reported, so no catch-up (start or restore, §5) reports it again. The dead-letter therefore keeps the raw frame durably and is **never purged before an operator decides** (reprocess it in the Core once the cause is fixed, or discard it with an audit row). Recovery is always reprocessing in the Core; asking the adapter to report again is not a recovery path.
    - **Erasure and retention (r6; Crivo q3, Vigia L6).** The raw frame holds trading data and account names (personal data). The "never purged" rule is subordinate to account erasure and is not a way around retention:
      - Q13-b erasure (30 days after the user deletes the account) deletes the user's dead-letter rows like every other row; data model 16.16 already includes them. The erasure is itself the operator decision for those rows, recorded in the admin trail, since the user's log is gone.
      - The operator is alerted when an undecided dead-letter row belongs to an account in its grace period, so it can be reprocessed or discarded before the erasure.
      - "No purge by age" applies only to undecided rows. A decided row (reprocessed or discarded) follows the activity-record retention (Q13, 24 months) or is deleted at the decision.
      - The dead-letter is not audit data, so this does not touch audit immutability.
  - **Types whose consumer is a later slice (r5):** reliable A→C types that the Core does not process yet (`risk_event`, `instruction`, `trade_completed`, `notification_read`) are validated against the schema, stored durably by the gateway (type, connection, account, `msg_id`, payload), deduped by `msg_id` and acked in the same transaction. A deferred frame, or an `execution_report`, that names an account not in the current session's `welcome` mapping (for example after an unbind or a rebind) is not refused, because the adapter would retransmit it forever: it is acked and stored **quarantined** as unbound activity (`execution.unbound_activity`, audited), and it is never applied to any All Win account.
    - **Quarantine rules (r6):** the row is stored under the session's `user_id`, whatever account id the frame carries, so it stays isolated by RLS. A later bind or rebind never promotes it to applied; only an explicit, audited operator reprocess could, as for the dead-letter. Quarantined rows count against the connection's storage limits.
    - **A quarantined `execution_report` that carries the `command_id` of an All Win command (r6, Crivo q2)** also flags the account of that command `out_of_sync` with `sync.command_unknown`, because the outcome of that command is now unknown to the Core.
    - **Deferred consumers (r6; data model 16.10):** the worker role only finds pending rows (a column-limited `SELECT`). The work, and marking the row processed, run as the application role with the row's `user_id` set, so RLS applies. The consumer still re-checks `(user_id, account_id)` before using a stored frame.
    - The ack means "durably received", not "processed". The consuming slice reads them later. **Exception:** an `instruction` (master-account input) stored before its consumer exists is never routed late: the consumer expires it at first processing (audited).
  - **`error` about a Core frame (r5):** an adapter `error` whose `ref_msg_id` is a `command` makes that command terminal `failed` at once. It is no longer retransmitted, because the same frame would get the same error. The Core audits `execution.rejected` with the error code as `reason_code` (never the detail), flags the account `out_of_sync` (`sync.command_rejected`, `adapter_reason_code` = the code), and raises an operational alert (a Core bug). An `error` about any other Core frame only alerts and logs.
  - **Retransmission after `expires_at` (r5):** "never send after `expires_at`" applies to the **first** send. A command already sent is retransmitted byte-identical until acked or until the 72-h horizon, even after its expiry. The adapter answers `duplicate` or `command.expired`, and it never executes late. Dropping the retransmission would hide an order that may have run.
- **Ordering:**
  - Within a session, WebSocket preserves order.
  - The adapter executes commands **for one account strictly in arrival order**.
  - A command with `depends_on` waits until that command reaches a final state. If the prerequisite failed, the command is reported `rejected` with `command.prerequisite_failed`. Example: `reverse` = `flatten` followed by `place` with `depends_on`, and the `place` is not sent to the platform until the account is flat.
- **Heartbeat:** both sides send every `heartbeat_interval_ms` (default 10 000). Three missed intervals → close 4005 and reconnect.
- **Reconnect:** exponential backoff starting at 1 s, factor 2, capped at 30 s, with ±20 % jitter. It never stops while the plugin is enabled.

## 5. Command semantics and expiry (Q16)

- **`place`:** side, quantity (already multiplied and capped by the Core), `order` (market/limit/stop/stop_limit) and `position_effect`.
  - For `open`, it carries **`protection`**: mode `individual`|`shared`, `stop_ticks`, `target_ticks`. The Core normalizes the profile's points/ticks/dollars into whole ticks using the catalog's exact tick size and tick value. A dollar distance that is not a whole number of ticks is **rounded toward the entry** (the tighter distance), and the adjustment is shown in the audit (Q-N1, decided). Profiles are keyed by instrument (Q-A2).
  - The adapter computes protective prices from the **fill price** (individual) or the **average position price** (shared, re-computed on every add), rounds to tick size, and reports the protective orders as `execution_report`s with `child_role` `stop`/`target` under the same `command_id`.
- **`modify` / `cancel`:** target orders created by All Win, identified by `(command_id, child_role)`, plus the `platform_order_id` when known. `modify` applies `price_delta` and/or a new total `quantity`.
- **`flatten`:** cancels **every** working order on that account and contract, manual and other All Win orders included, waits for the confirmations, then closes the remaining position (orphan-order protection; closing over working orders could flip the position). The wait is bounded by the adapter's flatten step timeout. A cancel refused because the order has just filled is not a failure: the fill is reported and the flatten re-reads the position and closes what remains. A cancel that fails for any other reason, or a step that times out, ends the flatten `rejected` (`order.cancel_failed`), and the Core flags the account `out_of_sync` (with the ADR-0005 top-severity alert when protective legs were already cancelled). A flatten never waits forever and never closes over a working order. The reason is `close` or `reverse`.
- **Adapter execution rules (clarified 2026-10-23 for V1-S2; reason codes and report attribution in [`reason-codes.md`](../../../contracts/adapter-protocol/v1/reason-codes.md)):**
  - **Shared-mode ownership:** the command that opened the position owns the shared stop/target. Adds get entry reports only, and resize/re-price reports carry the owner's `command_id`. Modify/cancel address `(owner command_id, stop|target)`. Ownership ends when the position goes flat. Re-pricing uses the owner's protection ticks.
  - **Clock skew** is estimated only from fresh messages (`welcome`, `heartbeat`), never from the `sent_at` of a retransmitted frame, which keeps its original value.
  - **Protective quantity invariant:** a protective leg never exceeds |position|. After any partial exit, whether an All Win `reduce` or an external/manual partial close, individual mode resizes legs FIFO (oldest entry first) and shared mode resizes to |position|. Legs reduced to 0 or left on the wrong side after a flip are cancelled (`order.orphan_cancelled`). A failed resize is `order.protection_failed` → `sync.bracket_incomplete` (alert and wait).
  - **No late bracket (catch-up and observation gaps):** an entry fill that the adapter did not observe live never gets a late bracket (the old fill price may be through the market). For each protective leg it deliberately does not submit, the adapter sends an `execution_report` `rejected` + `order.protection_not_submitted` under the entry's `(command_id, stop|target)`; the **Core** then sets `sync.bracket_incomplete` with a top-severity alert and waits for the user (§6.1). (`sync.*` codes are Core-emitted only.) During catch-up, existing legs may only be resized **down** to the position, never up and never re-priced.
    - "Observed live" means: the fill event was received while the account's platform connection was up, no connection-loss transition happened between the entry's submission and the fill's receipt, and receipt time minus the fill's platform time is at most `max_bracket_delay` (default 10 s; tuned by spike E4).
    - **Clock for that bound:** both times are taken on the **same local PC clock** and converted explicitly to UTC on the host (NT8 `Execution.Time` is in NT8's configured time zone). A delay below −1 s (platform time in the future) counts as **not observed live** (fail-safe), never as 0. If the PC clock differs from the Core by more than `max_bracket_delay` (skew from `welcome`/`heartbeat`), the plugin shows a clock warning, because brackets would otherwise be withheld.
    - **Replay clocks (Playback):** `Execution.Time` is the replay time, so the bound compares it with the connection's current replay time when NT8 exposes it. If it does not, the time bound is skipped for Playback accounts and only the connection and transition conditions apply. Spike E4 confirms which applies. This covers a plugin or NT8 restart **and** a broker connection loss while NT8 kept running (S11 AC-5). Otherwise every leg is reported `order.protection_not_submitted`.
    - Spike E4 confirms the NT8 event order and timestamps for a fill that arrives after a broker reconnect. Until then the time bound is the backstop.
  - **No second submission (crash window):** before submitting, the adapter looks on the platform for orders with the same `(command_id, child_role)` reference. For `place` (entry or reduce exit) and protective legs, **any** match, live or terminal, counts as already submitted: the adapter adopts it, reports its last state and never resubmits (this covers submit → immediate fill → crash before "submitted" was persisted). `flatten` alone ignores terminal matches and re-evaluates the current position (flat → `command.already_flat`, otherwise a new close), because it is idempotent by nature; a live match is adopted. Two or more adoptable matches → no adoption, no submission, `rejected` + `order.ambiguous_reference`, and the Core sets `out_of_sync` (alert and wait).
  - **Individual mode with partial entry fills:** the bracket is submitted on the first fill for the filled quantity and resized on each later fill. Prices come from the volume-weighted average fill price of that entry order, re-priced on later fills. An off-tick price is rounded toward the entry (tighter). If the rest of the entry is cancelled, the bracket stays at the filled quantity.
  - **Time in force:** protective orders are GTC (a DAY order would drop protection at the session end). Entry orders are DAY.
  - **Expiry re-check:** `expires_at` is checked on receipt **and** right before submitting to the platform, which covers waits on `depends_on` and the per-account queue.
  - **`flatten` on a flat account:** `cancelled` + `command.already_flat`, a terminal success for `depends_on`.
  - **`reduce` that cannot reduce** (flat): `cancelled` + `command.nothing_to_reduce`, never opening or flipping. A reduce larger than the position is capped (`order.quantity_capped`).
  - **Position-identity guard on `reduce` (protocol 1.2, U13 option A, user decision 2026-10-23):**
    - **Identity record** (adapter, every protection mode): a durable record per (account, instrument) holding `identity` (an All Win `command_id`, `external` or `unknown`), the **signed quantity** it describes and the **id of the last fill** applied to it. It is written in the same journal batch as the fill that changes it.
      - It is set only when a fill takes the position from flat to non-flat, or flips it. The identity is then the `command_id` of that fill's All Win `place` (open) command; for a non-All Win fill (`origin: external`) it is `external`.
      - Adds and partial closes, All Win or external, keep the identity and update quantity and last fill. External adds are accepted: an All Win reduce may then reduce contracts added by hand, which only reduces exposure (Vigia I2).
      - Going flat ends it.
      - It is a concept **separate from the shared-bracket owner** (which exists only in shared mode and keeps its own rule above). The two may differ, for example on an external position with an All Win shared add.
    - **Identity becomes `unknown`** (fail closed) whenever the adapter cannot vouch for it:
      - journal lost, reset or pruned for that instrument;
      - at startup, reconnect or catch-up, when the platform position or the latest execution for that instrument differs from the record's quantity or last fill (for example the user closed and reopened the same size while the plugin or NT8 was down);
      - `SimulationAccountReset` / `account_reset`;
      - a fill correction (`amend`/`remove`) touching the opening fill;
      - at the guard, when the live platform position differs from the record (an execution still queued, ADR-0005 N6).
      - An `unknown` or `external` identity never matches a `position_ref`. NT8 execution retention and ordering across a restart are verified by spikes E4/E9 (untested).
    - **Core:** every `reduce` carries `refs.position_ref` = the identity in the Core's expected state, derived from the **order of execution reports** with the same rule as the adapter: the All Win fill whose `position_after` takes the position from 0 to non-0, or flips it (`position_after` crosses 0 with the opposite sign). It is never derived from routing order: with two entries in flight from flat, the first to fill is the identity. If the Core holds no All Win identity, it does not send the reduce; routing skips it with `routing.position_unknown`, audited.
    - **Adapter steps for a `reduce`, in order** (after the account checks):
      0. **crash-window lookup** by order name `(command_id, x)`: any match is adopted and reported (filled, partially filled, cancelled…), never re-checked; 2+ matches → `order.ambiguous_reference`; a failed lookup → never submit. This comes first, so a re-delivery after a lost journal reports what already happened instead of "nothing to reduce" or "position changed";
      1. flat → `cancelled` + `command.nothing_to_reduce` (benign race: the bracket closed it);
      2. identity ≠ `position_ref`, `unknown` or `external`, **or** the order side does not oppose the position (a defensive check, never open or flip) → `rejected` + `command.position_changed`, nothing submitted. A reduce in a **1.0/1.1 envelope** (an older Core, which cannot send `refs`) takes this path too. A reduce in a **1.2 envelope** without `refs` is schema-invalid: the adapter answers `error` `protocol.invalid_message` and does not ack or execute it; under the §4 rule for an `error` about a command (r5) the Core then makes that command `failed` at once, never retransmits it, and flags the account. It is a Core bug, never a reason-code outcome, and the Core raises an alert on any `protocol.invalid_message` about its own frames;
      3. otherwise submit, capped to the position.
    - **Core on `command.position_changed`:** the account goes `out_of_sync` (`sync.position_changed`) with an alert and an audit row, and the Core waits for the user (alert and wait). A dependent command gets `command.prerequisite_failed`.
    - **Clearing out-of-sync discards pending reduces** (Vigia I1): when the user clears an account's `out_of_sync`, the Core marks that account's `reduce` commands still in the outbox (for example stuck after a rollback to a 1.1 session) as `expired`, audited, and never retransmits them.
    - **`flatten` is unchanged:** no guard, no expiry. Closing whatever is open always reduces exposure.
    - **Not in 1.2** (Pêndulo I2): reporting the adapter's identity in `position_snapshot` so the Core can re-learn it after `out_of_sync`. Until the position is flat, the Core keeps skipping reduces with `routing.position_unknown`. Revisit in a later minor.
  - **Modify/cancel outcomes:** one outcome report per target under the modify/cancel `command_id`, never with fills. The Core classifies reports by the action of their command.
  - **Fill corrections** (`amend`/`remove`): the recomputed order-level status, the original `fill_id`, and an authoritative `position_after`.
- **Orphan-order protection in the adapter:** whenever an account's position in a contract becomes flat **for any reason**, the adapter immediately cancels the remaining All Win stop/target orders for it, and reports them `cancelled` with `reason_code: order.orphan_cancelled` (vision §8).
- **Expiry (`expires_at`)**:
  - **Computed by the Core:** `expires_at = expiry_base + account.command_ttl`, where `expiry_base = min(instruction.sent_at, received_at)` (ADR-0003 §7).
  - **Default TTL: 5 seconds.** Editable per account between 1 and 60 seconds (Q16: short default, per-account editable). The 5 s default is decided (T2).
  - **Core side:** the Core never sends a command whose `expires_at` has passed. Pending commands found expired after a reconnect or restart become `expired`, are audited, and the account is flagged **out of sync**.
    - **Exception: the opening leg of a reverse** (HTTP 5.1.0, Crivo R1 b and E1). An expired opening `place` that never left the Core, state `pending` with no `sent_at`, is **held**: it is never sent once expired, and it is decided only when its `depends_on` flatten reaches a terminal state. It is **not** flagged when all of these hold at that moment:
      - it never left the Core. **Requirement on the delivery loop** (gateway, S2/M1, not yet implemented; Ferro): the loop commits `sent`/`sent_at` in a transaction **before** writing to the connection. A socket failure after that commit counts as sent, so the command is flagged and never exempt. With that order, `pending` proves that nothing was delivered. A test that crashes between the commit and the write is an acceptance criterion for the gateway;
      - its `depends_on` flatten reached a terminal success, and the flat position is proven:
        - `cancelled` + `command.already_flat`: the adapter checked a zero position at that moment, so the report itself is the proof (it carries no `position_after`), provided no later report exists for that account and contract, the same rule as for `filled` (Crivo n1);
        - `filled`: the latest report for that account and contract that carries `position_after` has 0, and no later report exists (Pêndulo L1);
      - the account has no other open sync reason.

      In that case the position is known (flat), so the command becomes `expired`, is audited as `execution.expired`, and is notified as `routing.reverse_close_only` with `reason_code` `routing.expired`; the routing decision stays `routed`. In every other case, including a flatten that ends rejected or `unknown` or never reaches a terminal state within its normal report timeout, the opening leg becomes `expired` and the account is flagged, as above.
    - Tests:
      - a reconnect with both legs pending, where the flatten fills **after** the expiry check, is not flagged;
      - a pending opening leg after a filled flatten is not flagged;
      - an opening leg already `sent` is flagged;
      - a flatten that ends rejected or `unknown`, or never becomes terminal, is flagged;
      - a crash between the send and the `sent` commit never qualifies.

      The exception covers only the reverse. Not flagging any never-sent entry would change Q14 and needs a decision.
  - **Adapter side:** the adapter rejects a command received after `expires_at`, using its clock corrected by the skew measured between `welcome.sent_at` and local receipt, and reports `status: expired`, `reason_code: command.expired`. A command already submitted to the platform is never cancelled because of expiry.
  - **Which commands carry it:** the schema **requires** `expires_at` on position-increasing commands (`place` with `position_effect: open`). For exits (`flatten`, `reduce`, `cancel`, protective `modify`) it is optional, and the Core omits it, so a late exit still reduces exposure; late exits are audited as late (T1, decided: only entries expire). From 1.2 a late `reduce` is protected by the position-identity guard instead of an expiry (U13): it executes only against the position it was meant for.
  - **Late exits (r5):** an exit sent after `expiry_base + account.command_ttl` is audited as late: `execution.command_sent` carries `late_ms` = sent time minus that instant (an integer > 0; absent when on time). Entries never carry it, because a late entry expires.
- **Delivery-time sync check (r5):** at delivery the gateway checks the account's `sync_state`. A pending **position-increasing** command (`place` with `position_effect: open`, the opening leg of a reverse, `modify` of an entry) for an account that became `out_of_sync` after the routing decision is not sent. It becomes `expired`, audited `execution.expired` with `reason_code` = `routing.out_of_sync` (a routing code in an execution entry). Exits are sent (P1). The routing gate (ADR-0003 §4.1 gate 6) only covers the decision instant; Q14 stops the account until the user clears it.
- **Flatten outcomes (r5):**
  - Every `flatten` that ends successfully emits a terminal report that **proves** the flat position: `filled` with `position_after` = 0, or `cancelled` + `command.already_flat`. This is an adapter requirement. Without it, the §5 exception and the Q75 notifications cannot recognise the flat account, and the Core fails closed and flags the account.
  - The **cancel** step is bounded (adapter setting, 10 s today); a cancel that fails or times out ends `rejected` + `order.cancel_failed`. A cancel refused because the order has just filled is not a failure. The **close** step that times out (60 s today) ends `unknown`. The Core flags the account in every non-success case.
  - **U19:** when a failed flatten leaves an open position that lost all or part of its All Win protection because of that flatten, the reason is `sync.flatten_failed_unprotected` (critical). This applies whatever the origin: close, reverse (including Q75), the Q27 orphan-leg flatten, or local enforcement. With protection intact the reason is `sync.command_rejected`.
- **Catch-up of executions missed while the plugin or NinjaTrader was off (r5, Q80):**
  - Transport needs no change. On catch-up the adapter walks the platform's executions in time order and reports every one it has not reported yet as a normal `execution_report` (`operation: add`; `origin` `all_win` when the order name carries an All Win reference, otherwise `external`). The adapter dedupes on its journal and the Core on `(account_id, fill_id)`. No late bracket (above).
  - **Catch-up runs at start and on every connection restore of the account** (requirement; the plugin ran it only at start, gap G1).
  - The Core labels such a fill **recovered** when its `platform_time` is earlier than the start of the session that reported it, or falls in an interval with no session. It is a derived label on the trade and in the audit, not a protocol field.
  - **Source:** NinjaTrader documents no supported way to read historical executions. Whether the period while NT8 was closed is readable is spike E4 / K15. Until that is proven for a provider, Q80 (b) applies to it.

## 6. Synchronization, disconnects and out-of-sync (Q14)

The Core keeps, per account, an **expected state**: position per contract and All Win working orders, derived from commands and execution reports. Each account has a `sync_state` of `in_sync` or `out_of_sync`. Being out of sync is **sticky until the user clears it** in the web app (Q14). The only exception is the release of reasons held at a clear (§6.4, Q101).

| Trigger | Result |
|---|---|
| Command `rejected`, `expired`, or `partially_filled` without completion before the order's end of life | `out_of_sync` + alert (`command.position_changed` → `sync.position_changed`, 1.2) |
| Disconnect while the account had an open position or a command in flight | `out_of_sync` + alert ("alert and wait") |
| Disconnect while flat with nothing in flight | `disconnected` only. On reconnect, if the snapshot shows flat with no All Win working orders, the state returns to normal automatically and the user is notified |
| Reconnect snapshot differs from the expected state (position or All Win working orders) | `out_of_sync` + alert listing the differences |
| Periodic snapshot (every 60 s while connected) differs | `out_of_sync` + alert |
| External order or fill on a destination account (`origin: external`) | Destinations of master replication: `out_of_sync` + alert. TradingView-only destinations: audit only (T3, decided) |
| Sim account reset or Playback rewind (`position_snapshot.trigger: account_reset`, NT8 `SimulationAccountReset`) | `out_of_sync` (`sync.simulation_reset`); the discontinuity is never interpreted as fills |
| Execution amended or removed by the platform (`fill.operation` `amend`/`remove`) | The Core corrects the position and P&L, audits it, and flags `out_of_sync` if the expected state no longer matches |
| Command not acked within the 72-h retransmission horizon (§4) | `out_of_sync` (`sync.command_undelivered`) + alert; the command is `undelivered` and never retransmitted |
| User unpairs the connection (Q18) | Every mapped account of that connection becomes `out_of_sync`; nothing is closed or cancelled |
| User disconnects in the plugin (close 4100, 1.3) | Mapped accounts with an open position or a command in flight: `out_of_sync` (`sync.disconnected_by_user`) + alert, because routing stops for them. Flat with nothing in flight: status + audit row only, no alert. `Connection.disconnect_reason = user`. Local enforcement continues |
| Bracket integrity cases on local-OCO connections | See §6.1 |

**While `out_of_sync`:**

- The Core refuses new position-increasing instructions for the account (`routing.out_of_sync`, ADR-0003 §4.1).
- **It never sends corrective orders on its own:** no reopen, no resync, no catch-up. Forwarding a source's `close`, or a reverse's closing leg, as a `flatten` while the expected position is flat (ADR-0003 Q75) is not a corrective order: it executes the source's instruction.
- **Such a flatten** is handled like any flatten. `command.already_flat` neither clears `out_of_sync` (Q14) nor adds a sync reason. Fills are applied as usual (`execution.filled`, `position_after`), and their `position_after` of 0 keeps the expected position flat; they add no sync reason. The account stays out of sync until the user clears it. Adapters need no change: `flatten` has no position-identity guard and no expiry. As for every flatten, the adapter first cancels **every** working order on the contract, manual and other All Win orders included, even when the position turns out flat. On an account the user may be fixing by hand this can cancel the user's own resting orders; ADR-0003 Q75 accepts this and notifies it (`routing.out_of_sync_flatten`).
- The user sees the differences and either fixes the account manually and clears the flag, or flattens and clears.
- `welcome.accounts[].sync_state` and `sync_reasons` tell the plugin to show the state and its reasons in its window. Changes during the session reach the plugin through the C→A message `account_state` (protocol 1.1, sent only when 1.1 is negotiated). On 1.0 the plugin shows the `welcome` state plus its locally observed reasons until the next `welcome`. When an account is out of sync and every reason has been resolved by the Core (§6.4), the plugin receives the distinct codes of the resolved reasons, most severe first, because the protocol requires at least one reason while out of sync.

**Further rules (r5):**

- **A superseded session is a disconnect** (4004). After a network drop the adapter usually reconnects before the Core sees the half-open socket. The disconnect rule above still runs for the replaced session, as a network loss: `sync.disconnected_in_trade` when the account had a position or a command in flight, and §6.1 `sync.bracket_integrity_unknown` where it applies. The connection status shown to the user stays `connected`, and no session-ended event is emitted for the replaced session.
- **A snapshot of an unobservable account is not compared.** A `position_snapshot` entry with `connected: false` (the platform lost its broker connection) is neither compared with the expected state nor allowed to resolve a reason; only §6.1 applies. The next connected snapshot does the comparison.
- **A binding made during a live session takes effect at the next handshake** (as a rename does, G3). The gateway sends commands only to accounts in the current session's `welcome` mapping. A command for an account bound later stays pending: entries expire by their TTL, and exits wait for the next handshake. This is made visible: the route status shows `binding_pending` (route status only, it never skips) and the web app says "reconecte o plugin para ativar o vínculo". Closing the session on every bind would flag every account of that connection that has a position (Q14), so the Core does not do it. A protocol message `accounts_changed` for immediate effect is queued for a later minor.

### 6.1 Bracket integrity on connections with locally simulated OCO

**Fact** (NT8 profile §7.3, official NinjaTrader documentation): on the **NinjaTrader** and **Rithmic** connections, OCO is simulated on the local PC. While NT8 is closed, crashed, or has lost its connection, a filled stop does **not** cancel its target, and vice versa. Locally simulated orders in `TriggerPending` may come back in state `Unknown`. Our orphan-order protection (vision §8) runs in the plugin, so it cannot act while NT8 is down either.

**Protocol and state rules:**

- **Reporting the OCO location.** In `hello.accounts[]`, the adapter reports `oco_location` as `local`, `server` or `unknown`, derived from the connection provider: NinjaTrader and Rithmic → `local`; providers documented as server-side → `server`; anything else → `unknown`. The Core treats `unknown` like `local`.
- **Warning in the UI.** The web app and the plugin show a permanent notice on accounts with `oco_location` `local` or `unknown`: brackets are only fully protected while NinjaTrader is running and connected.
- **While unobservable.** An account with All Win protective orders working on a `local`/`unknown` connection gets the sync reason `sync.bracket_integrity_unknown` when either of these happens:
  - the adapter session drops;
  - a snapshot reports `connected: false` for that account (the platform's broker connection was lost while NT8 kept running).
  Because the account had working orders, it is already `out_of_sync` under the disconnect rule; the reason makes the cause explicit.
- **Reconnect evaluation.** This uses the mandatory `position_snapshot` with order `state`, plus the execution reports replayed from the outbox:

| Case found at reconnect | Action | Sync reason |
|---|---|---|
| Position flat, one leg filled, the other leg still working | The adapter applies **orphan-order protection** immediately (decided rule: cancel remaining protective orders whenever flat) and reports `order.orphan_cancelled` | `sync.bracket_orphan_leg` (informational; the account still needs the user to clear it, per the disconnect rule) |
| **Both legs filled**: the orphan leg opened an unintended opposite position | **Decided (Q-O1 / Q27 = b):** the Core sends `flatten` with reason `orphan_fill` for that position only, with a top-severity alert, and audits it. The command keeps the `instruction_id` that created the bracket and uses step `orphan_flatten:<event_id of the orphan fill>`, so it never collides with that instruction's own `flatten` (ALIGN-8) | `sync.bracket_double_fill` |
| Position open, protective orders missing or in state `unknown` (unprotected position) | Alert at the highest severity and wait. The Core never re-submits protection on its own (vision §8, alert and wait) | `sync.bracket_incomplete` |
| Position and orders match the expected state | Nothing to do. The reason is removed; the account stays `out_of_sync` until the user clears it if the disconnect rule applied | — |

- **Detection after a plugin restart** relies on the `Order.Name` reference (`command_id` + `child_role`, at most 50 characters; profile §6) and on object identity. Spike experiment E4 verifies that the name survives a restart.
- **The race at reconnect is covered.** If the remaining leg fills before the orphan cancel, the case becomes the double-fill row.
- **A protective leg that filled while the account was unobservable never resolves `sync.bracket_integrity_unknown`.** The reason stays active until the user clears the account. Its presence alone does not make the account unobservable (§6.4): once observation resumes, the user can clear it.

### 6.2 Sync reason codes (r5)

The canon has **21** codes. **The table order is the severity order** (most severe first, r6.3), the same order as the HTTP `SyncReasonCode` enum; "most severe first" anywhere in this ADR or the contracts means this order. The protocol field is an open pattern; the HTTP contract uses a closed enum and receivers tolerate unknown codes. A new code is a minor version on both sides. Detail fields (adapter reason code, snapshot trigger, instrument and contract month from the wire) carry the specifics, never the code.

| Trigger (§6, §6.1, §5) | Code |
|---|---|
| A failed flatten removed protection from an open position (U19, §5) | `sync.flatten_failed_unprotected` (critical) |
| Both legs of a bracket filled (Q27) | `sync.bracket_double_fill` |
| Open position with missing or `unknown` protection | `sync.bracket_incomplete` |
| Local-OCO account with working protection while unobservable | `sync.bracket_integrity_unknown` |
| A report with status `unknown`; a frame moved to the intake dead-letter (§4); a quarantined report carrying an All Win `command_id` (§4, r6) | `sync.command_unknown` |
| `command.position_changed` (U13); also a `nothing_to_reduce` that contradicts a non-flat expected position | `sync.position_changed` |
| Reconnect or periodic snapshot differs (the trigger is a detail) | `sync.snapshot_mismatch` |
| `partially_filled` without completion before the order's end of life | `sync.command_partially_filled` |
| Command `rejected`, any other reason (the adapter code is a detail; includes a cancel failure inside a command and an `error` about a command) | `sync.command_rejected` |
| Command `expired` (adapter or Core) | `sync.command_expired` |
| Not acked within the 72-h horizon | `sync.command_undelivered` |
| An orphan-protection cancel started by the adapter itself failed | `sync.cancel_failed` |
| `order.ambiguous_reference` | `sync.ambiguous_reference` |
| External activity on a master-replication destination | `sync.external_activity` |
| Fill amend/remove that breaks the expected state, or amend/remove of an unknown add | `sync.fill_corrected` |
| Sim reset or Playback rewind | `sync.simulation_reset` |
| Disconnect (including supersede) with a position or a command in flight | `sync.disconnected_in_trade` |
| User disconnect in the plugin (4100) in that state | `sync.disconnected_by_user` |
| User unpairs the connection (Q18) | `sync.connection_unpaired` |
| Bound account not reported after a re-pair | `sync.account_not_found_after_repair` |
| Position flat, one leg filled, the other working (orphan cancelled; informational, so last) | `sync.bracket_orphan_leg` |

Not sync reasons, audited and alerted on their own:
- `execution.unbound_activity`: a fill or non-flat snapshot on a venue account with no binding (ADR-0008); one entry per position change of (connection, platform account, instrument), not per fill.
- `execution.undelivered` (outcome `unknown`: the command may have run).
- `connection.account_kind_changed` (ADR-0008 §3.5).

**Outside production**, a command for a bound account now reported `live` or `unknown` ends at delivery as `rejected` + `command.account_not_simulation`, and the account is flagged (`sync.command_rejected`). This applies to exits too: outside production nothing touches a real account.

### 6.3 Reconnect evaluation window (r5)

- Per mapped account, the window runs from `welcome` until the Core has evaluated that account's reconnect `position_snapshot` **of this session**, together with the reports retransmitted from the outbox (§6.1 table). A snapshot counts when its `msg_id` was first received in this session; a retransmission from an earlier session keeps its old `msg_id` and does not count. **"First received" is decided against the durable inbound receipt `(connection_id, msg_id)` (r6, Vigia I5)**, never an in-memory set, so a restart of the Core does not make an old snapshot look new. As a second check, its `taken_at` must not be earlier than the session start minus the clock-warning tolerance. From 1.4 the adapter stamps `taken_at` with its skew-corrected Core clock, never its raw local clock. The Core sends **no command** to the account during the window. Entries keep their TTL and expire if the window outlasts it; exits wait. Route status shows `connection_checking`, which never skips.
- **Unobservable account (r5):** a snapshot entry with `connected: false` (the platform lost its broker) ends the window for that account: no `snapshot_timeout` for it. Delivery to it stays **held**: entries keep their TTL and expire, and exits wait. Delivery resumes with the next snapshot that reports `connected: true` for the account; the adapter sends a `change` snapshot on every connection change. The same hold applies during a session when a `change` snapshot reports `connected: false`. Sending would only produce `command.account_not_simulation` from the adapter (the account kind reads `unknown` without a broker) and lose an exit. While the hold lasts, the route status shows `broker_disconnected` (status only, it never skips) so the user sees why exits wait.
- **Bounds (provisional; Backend and SRE tune them with data):** if the evaluation has not finished after **60 s**, the Core raises an alert to the operator and to the user ("verificação da conexão não concluída; ordens aguardando"). After **5 min** it closes the session with 4000, reason `snapshot_timeout`; the adapter reconnects and sends the snapshot again. That close is a normal disconnect for §6. The alert repeats on every such close, and after 3 consecutive ones for the same connection it is raised as critical (the plugin is not sending its snapshot).

### 6.4 Clearing out of sync (r5; Q83, Q84)

- **Clear** is a user action in the web app, with optimistic concurrency on the account's sync version. It removes every active and resolved reason and sets `in_sync`.
- **Baseline:** the account's last evaluated snapshot (position and All Win working orders, as reported) becomes the new expected state. Without that rebase the next snapshot would flag the account again forever.
- **After a clear,** only new triggers flag it again: a new difference against that baseline (`sync.snapshot_mismatch`) or a new event. A condition the user accepted at the clear and that stays the same does not flag again.
- **Not observable** is a live condition only: no session, a snapshot with `connected: false`, or the §6.3 window still open. While it holds, clear is refused (Q83 a). An active reason, `sync.bracket_integrity_unknown` included, never blocks the clear by itself once observation has resumed and the reconnect evaluation is done. **With an open position:** allowed with a confirmation (Q84 a).
- **Protection of open positions at the clear (r6; Q98 b, user decision 2026-10-24; Vigia L4):** any working stop that covers a position counts as protection, a manual one placed in the platform included (shown as external). A clear therefore never silently accepts an unprotected position as the new baseline, whatever reasons were active (`sync.bracket_integrity_unknown` included):
  - **"Covers" is strict (Vigia L1 on Q98):** working stop or stop-limit orders on the opposite side, on the same account and contract, whose total working quantity is at least |position|. The working quantity is the **remaining** (unfilled) quantity of each order (r6.1, Pêndulo n2): in a snapshot, `working_orders[].quantity` is the remaining quantity (platform quantity minus filled); in an order's `execution_report` the quantity is the order's total, and the Core derives the remaining one from the fills it applied for that order (r6.2). Only the states `accepted`, `working` and `trigger_pending` count as working (r6.2). Nothing else counts: no target or limit order, no order in an `unknown` state or with no reported state, no order on another contract month. The adapter always reports the order state in snapshots (Pêndulo n1). Coverage is `full` (|position| covered), `partial` (some) or `none`; with any protective order in an `unknown` or unreported state it is `unknown`.
  - **Only `full` is protected (Q100 a, user decision 2026-10-24):** `partial`, `none` and `unknown` are not. Each such position keeps `sync.bracket_incomplete`, or gets it if absent, one per instrument, and each reason the clear keeps carries **`held_at_clear`** (the time of the clear). Every other reason is cleared. The account stays `out_of_sync` with the held reasons only, and its entries stay blocked; exits pass.
  - **Re-evaluated after the clear:** the Core checks the coverage of each position listed at the clear on every evaluated snapshot (periodic, change or reconnect) and every order report, until that position is flat. Positions opened by All Win later follow the normal bracket rules (§5, §6.1), so the check does not race their own protective orders.
  - **Automatic return (Q101, user decision 2026-10-24):** a held reason resolves when its position is `full` or flat. When the last active reason was a held one and it resolves, the account returns to `in_sync` by itself, audited as `sync.restored` with that reason: the user already confirmed this account at the clear, and the remaining condition is objective and observed by the Core. This is the only automatic return to `in_sync`.
  - **Losing the cover blocks again automatically (user, 2026-10-24):** a position fully covered at the clear, or after it, that loses its cover (a covering stop cancelled, removed, filled, or reduced below |position| while the position stays open) raises `sync.bracket_incomplete` again **without** `held_at_clear`. Entries are blocked at once. Like any reason raised after a clear, it is sticky and needs a new clear (Q14).
- **Unbound account:** it cannot be cleared, because there is nothing to observe. A rebind or re-pair keeps it out of sync with its reasons. Once the first snapshot under the new binding is evaluated, the user can clear it, and a difference found then adds `sync.snapshot_mismatch`. Archiving the account (ADR-0008) is the other way out.
- **Resolved reasons:** only the Core moves a reason to resolved (the §6.1 "matches" row). The account stays out of sync until the user clears it.
- **Clearing expires pending reduces** of that account (§5, Vigia I1).

## 7. Local risk enforcement (second barrier)

- **Limits:** the adapter caches the latest `limits` per account (`version` is monotonic) in its durable store, so enforcement works **with the Core offline** (S15 AC-4: feasible).
  - Monitored metrics: daily loss, drawdown (trailing intraday / end-of-day / static), optional daily profit target with `on_reach` (Q3), and maximum position.
- **Warnings:** at `warning_threshold_pct` (80 %), a warning is shown in the plugin and sent as a `risk_event`.
- **On a limit, with enforcement enabled:**
  1. Cancel working orders.
  2. Flatten.
  3. Block the account until the trading-day reset (`reset_local_time` in `time_zone`; no early unlock).
- **Blocking is reactive:** any new order on a blocked account is immediately cancelled, or flattened if already filled. The cancel is reported as `risk_event` `blocked_order_cancelled`; the flatten of a resulting position is reported as `risk_event` `flattened` (r6.1), so the Core records them as `risk.reactive_block_action` and `risk.flatten`. NinjaTrader has no known API to prevent order entry; the spike confirms.
- **Playback** uses the connection's time (`platform_time`) for resets and daily P&L.
- **Reporting:**
  - Every enforcement action produces `execution_report`s with `origin: local_enforcement` plus a `risk_event`.
  - `while_disconnected: true` marks actions taken offline. They are delivered from the outbox on reconnect.
- **Net P&L (1.3, U20 user decision 2026-10-23):** every risk metric uses **net** P&L.
  - **Effective commission of a fill** = max(platform-reported `fill.commission`, `commission_per_contract_side` × quantity). It is fixed when the fill is recorded, with the floor in force at that time, and never recomputed.
  - **Live usage** (daily loss, intraday drawdown on equity) also deducts the **expected closing-side commission** (floor × open quantity). This is fail-closed: a limit triggers slightly earlier, never later. End-of-day and realized values are realized-only.
  - **Drawdown** high water and floor use the **adjusted balance** = platform balance − Σ(effective − platform-reported commission). The sum is cumulative since the account's tracking start, never just the day's; for the evaluation target it runs from `evaluation_start` (Crivo N1). The plugin cannot rebuild it, so `limits.drawdown.balance_adjustment` carries the sum up to the previous trading-day close. Plugin adjusted equity = platform balance − balance_adjustment − the day's Σ(effective − reported) − expected closing commission (Pêndulo M9). The adapter keeps the day's Σ(effective − reported) in its durable journal, reset at the trading-day boundary. If it cannot vouch for it (for example after a restart without the journal), it alerts the user and uses the Core's value at the next `limits` push, never a smaller one (Pêndulo LA2).
  - **Floor timing:** the plugin uses the floor of the last applied `limits` version at fill time. A difference with the Core's snapshot (a higher floor not yet received) is reconciled by the Core and affects only display (`risk_state`) and `trades.reconcile_mismatch`. A `fill.commission` the platform fills in late is reported as absent (the floor applies) and never amended afterwards (spike E14).
  - **Not reported vs zero:** an absent `fill.commission` means the platform reported nothing; a present `0` means zero. Per-fill `commission_source`: `platform` when the reported value is ≥ floor × quantity (it won); `configured` when the floor won (reported lower, zero, or absent with a floor); `none` when absent and no floor.
  - **`commission_source`** `none` (no platform commission and no floor) means net equals gross, and the plugin shows "comissão não informada; os limites podem estar folgados demais".
  - **A lower floor** is scheduled for the next trading day (ADR-0006 §3.7); a higher one applies at once.
- **Evaluation target (1.3, Q46):** `limits.evaluation` gives the target and the baseline up to the previous trading-day close (computed by the Core). The plugin shows progress = baseline + the day's net, which also deducts the expected closing commission (Pêndulo L-E), so the target is never reported early. Reaching the target only notifies; it never flattens or blocks.
- **Unpairing stops local enforcement** for that connection (G7): the plugin no longer receives `limits` and clears its cache for those accounts. Before unpairing, the web app and the plugin say so.
- **Core-side gate:** the Core's own gate (ADR-0003 §4.1) uses the same limits and the reported state. The adapter never relaxes a block because of a Core message, except an `unblocked` state that follows a reset.

### 7.1 Drawdown frame anchored at the trading-day start (protocol 1.4, r5)

**Problem.** The Core measures risk on the All Win equity: `size_amount` plus the realized net since `evaluation_start` plus unrealized (S13). The 1.3 plugin compared absolute balances (`floor_balance`, `high_water_balance`, `lock_at_balance`) with an equity derived from the **platform** balance. The two frames differ by the platform's earlier P&L. Example: a Sim account at 52 000 with size 50 000 and a static 2 000 drawdown would block only after a 4 000 loss.

**Decision (realized anchors and day distances).**
- **Core:** `E_start` = `size_amount` + the realized net (effective commission) up to the trading-day boundary, from the fills it has applied. That is reliable data; nothing depends on the best-effort `risk_state`. Each `limits` carries distances from `E_start`:
  - `room_to_floor` = `E_start` − floor (signed: a negative value means the floor is already breached at the day start, so the plugin is in breach from the first moment and entries stay blocked);
  - `high_water_above_start` = high water − `E_start` (trailing intraday; signed: with realized anchors the high water, which includes unrealized peaks, can be below `E_start`);
  - `lock_above_start` = (`size_amount` + offset) − `E_start`. It is sent only when the firm's drawdown locks (Q90 `trailing_lock` `floor_reaches_start`) and may be negative. When the drawdown never locks, or the lock rule is not modelled, it is omitted and the floor keeps trailing (the stricter side).
- **Plugin:** `P_reset` = the platform `CashValue` at the boundary.
  - `day` = (`CashValue` now − `P_reset`) − the day's Σ(effective − platform commission) − expected closing commission + unrealized now (computed by the plugin from positions and price; `NetLiquidation` is not used).
  - `E_start` + `day` is then the exact current equity. The platform's earlier P&L cancels out.
  - Enforcement compares `day` with the distances: floor = −`room_to_floor`; for trailing intraday, the floor rises with the day's peak above `high_water_above_start`; the lock caps it at `lock_above_start`.
- **Positions across the boundary (Q93 a, user decision 2026-10-24):** their whole open P&L counts in the day it is measured (daily loss and daily target).
- **The boundary:** the plugin uses the boundary instant the Core sends (UTC, from the firm's reset time and time zone; the Playback clock in Playback), never its own.
  - It reads `CashValue` after a settle interval (`max_bracket_delay`, 10 s default) so that fills with `platform_time` ≤ boundary are processed first.
  - A fill with `platform_time` ≤ boundary that arrives after the read corrects `P_reset` by its realized net. It is journaled, with an alert when it arrives after the interval.
  - Until the read, the previous day's limits stay in force (no early unlock).
  - `P_reset` is journaled.
- **Unknown day:** when the plugin cannot vouch for `P_reset`, the day is unknown and the plugin blocks new **entries** for that account locally. Examples: it was not running at the boundary, the journal was lost, or a Playback rewind or account reset happened.
  - The block uses a dedicated risk code (`risk.day_unknown`, name fixed by the protocol text). It is **not** a sync reason: local risk-block codes do not trigger `sync.command_rejected`.
  - The state is reported in `risk_state`, and the Core skips entries at gate 7 (`routing.risk_blocked`, detail `day_unknown`). Exits always pass.
  - Rebuilding `P_reset` from the platform's executions is allowed only after continuous observation since the boundary, proven by the journal. Rebuilding from the execution history stays off until spike E4 / K15 proves it is complete for that provider.
  - The plugin never re-anchors silently.
- **Re-anchor (Q92 c, user decision 2026-10-24):** only the user re-anchors, by confirming "recomeçar a contagem de hoje" in the web app. The confirmation warns that losses All Win did not see may be left out, and it is audited.
  - The Core sends `day_realized_net_known` (net of effective commission, from the fills it has applied) in a reliable `limits` field with its own version and a **watermark**: the last (account, fill) it includes.
  - **Watermark rule (exact, r6; Pêndulo W1):** the plugin applies the field only if either (a) the last fill of that account that it reported today, as recorded in its current journal, is the watermark fill, or (b) its current journal records no reported fill of that account today. Then `P_reset` = `CashValue` now − `day_realized_net_known`, and the day's commission-gap sum restarts at 0. Open positions count through the unrealized term, so the account need not be flat.
  - Otherwise the plugin keeps the request pending, the day unknown, and waits for the newer version that the Core re-issues after each fill it receives while a re-anchor is pending.
  - Each new re-anchor version also bumps `limits.version`, so the frame is never dropped by the latest-wins rule (§3) and the plugin applies it (protocol 1.4.2 text, Pêndulo n3). The plugin still checks the re-anchor version on its own.
  - Older or repeated versions are ignored.
- **Sessions below 1.4** receive no drawdown in `limits` (the absolute frame is wrong) and are told to update. Without the local barrier the Core fails closed: while an account with a drawdown rule is served by a session below 1.4, routing skips its **entries** with `routing.adapter_protocol_outdated` (as for a reduce below 1.2); exits continue. The Core gate still applies. The absolute fields remain for display only; `balance_adjustment` is display-only in 1.4.
- **Known limitation:** a deposit or withdrawal during the day moves `day`. A Sim reset is handled as `sync.simulation_reset`.

## 8. Pairing a NinjaTrader instance (answers D5)

**Flow:**

1. **Web:** the user creates a *connection* ("NinjaTrader – Desk A"). The Core returns a **pairing code** and shows it with a countdown and the instructions "open All Win in NinjaTrader → Pair → type the code".
2. **Plugin:** the user types the code. The plugin calls `POST https://<host>/adapter/v1/pairings` with `{ pairing_code, instance_id, instance_name, adapter: {type, version, build} }`.
3. **Core:** validates the code and returns `201 { connection_id, device_token }`. The device token appears exactly once. The plugin stores it with **Windows DPAPI** (`ProtectedData`, CurrentUser scope) and connects.
4. **Web:** shows the connection as *paired* (instance name, plugin version, last seen) and lists the instance's visible accounts, so the user can map them to registered accounts.

**Pairing code design:**

| Property | Value |
|---|---|
| Alphabet / length | 8 characters, Crockford Base32 (the displayed codes never contain I, L, O, U), shown as `XXXX-XXXX`. Input is case-insensitive, ignores spaces and hyphens, and maps O→0 and I/L→1, so the visual confusions the design worried about (0/O, 1/I) cannot cause a failed pairing. 40 bits |
| Lifetime | 10 minutes, **single use**; at most one active code per connection (a new code invalidates the previous one) |
| Storage | **HMAC-SHA-256 with a server-side pepper** (ALIGN-4): a 40-bit code hashed with plain SHA-256 could be brute-forced offline after a database leak. Never logged in clear, not even a prefix. TOTP recovery codes (50 bits) use the same scheme |
| Brute force | The pairing request carries only the code, so a wrong attempt cannot be attributed to a connection (ALIGN-4). Limits: **per source IP, 10 attempts per minute**; **globally, 100 failed attempts per 10 minutes** locks the pairing endpoint for 10 minutes and raises an operational alert. With a 40-bit space, at most a few codes active at once and the 10-minute lifetime, the chance of guessing any active code is about 10⁻⁹ per lockout window. The web shows an active code as *used* or *expired*; it is never revealed again |
| Re-pairing | Generating a new code for an already paired connection lets the user move to a new machine. Successful re-pairing **revokes the previous device token** (its session closes with 4002) **Account bindings follow the connection by `platform_account` name**: accounts reported by the new instance with the same name keep their All Win binding. Bound accounts the new instance does not report become `out_of_sync` with reason `sync.account_not_found_after_repair` and receive nothing until the user rebinds or clears them (design S3-10 question) |
| Revocation | "Unpair" in the web app revokes the token immediately |
| Device token | `awd_<key_id>_<secret>`, 256-bit secret, no expiry while in use. It is revoked by unpairing or re-pairing. Rotation on demand comes later if required |
| Environment guard | Outside production, `simulation_only` is always true and the Core refuses to map platform accounts reported as `live` or `unknown`. Development adapter builds refuse non-simulation accounts on their own as well |

## 9. Versioning

- `protocol_version` is `major.minor`. Additive optional fields and new enum values bump the minor, and receivers must ignore nothing silently: unknown enum values from a newer minor are answered with `error` `protocol.unsupported_value`.
- A new message type is also a minor bump. The sender uses it only when the negotiated version includes it (the schema enforces it for `account_state`).
- Envelopes created after `welcome` carry the negotiated version. Outbox frames keep their original bytes on retransmission, so a receiver accepts any envelope minor that is less than or equal to the negotiated one (e.g. a `1.0` frame inside a `1.1` session).
- **Rollback (newer frame, older session):** an outbox frame created in a newer-minor session is still retransmitted byte-identical if a later session negotiates an older minor (e.g. Core rollback from 1.1 to 1.0). The receiver accepts it when its `type` exists in the negotiated version and its payload validates against the schema the receiver knows; otherwise it answers `error` `protocol.unsupported_value` with `ref_msg_id` and does **not** ack it. An A→C frame refused this way stays in the adapter's outbox and the session is flagged for the user; a refused `command` follows the §4 rule for an `error` about a command (r5): `failed` at once, never retransmitted, the account flagged. Every A→C type of 1.0 still exists, so this case cannot lose an execution report. **1.3-only A→C messages** (`trade_completed`, `notification_read`) are **held** by the adapter while the session negotiated less than 1.3, and are sent at the next ≥ 1.3 session (Pêndulo L-R). A 1.2 `reduce` frame (with `refs`) retransmitted to an adapter that negotiated 1.1 fails the 1.1 schema and is refused the same way: it is never executed, and under the §4 rule for an `error` about a command the Core makes it `failed` at once and never retransmits it. This is intended. **Never "fix" it by stripping `refs`**, because that would run the reduce without the guard.
- **1.3-only C→A frames on an older session (Pêndulo L2):** while a session negotiated less than 1.3, the Core **drops** pending `notification` frames from that connection's outbox instead of sending them, and sends no `notifications` snapshot. This mirrors the A→C hold (Pêndulo L-R). Nothing is lost, because the `notifications` snapshot after the next `welcome` that negotiates 1.3 or later is authoritative. A 1.2 plugin (for example an older build reinstalled) therefore never sees a frame type it would refuse.
- **Notifications are an exception to the rollback rule (1.3.2 note, no schema change):** the receiver acks every `notification` / `notifications` frame that is valid against the protocol schema. It validates `params` against its own copy of the notification registry. When the `code` is unknown, or the `params` do not validate (for example a new enum value in a newer registry minor), it keeps the item, its count and its read state, and renders a generic text with the code and the severity, without params. It **never** answers `protocol.unsupported_value` for a code or params, because that would leave the reliable channel stuck on a display-only item. The generic text is an i18n key (pt-BR and en), never built in code, and the params are never rendered or logged raw. The receiver stores the item's `code` and `params` as received, so a later client version that knows the code upgrades the render in place, with nothing new on the wire. The web client follows the same rule. The registry therefore evolves in minor versions with closed enums in `params`. Fields that the protocol schema itself closes (`severity`, `target.kind`, and `category` where the schema version closes it) are not covered: a new value there is a protocol minor, and older receivers fail it as before. Plugin tests for the 1.3 slice (Pêndulo):
  - T-n1: an unknown code is acked, rendered generically, counted, and can be marked read;
  - T-n2: params that fail the registry copy behave as T-n1, and are not rendered;
  - T-n3: a frame invalid against the protocol schema still gets an error and no ack;
  - T-n4: a replay of the same item is idempotent by id and revision.
- A new major lives in `contracts/adapter-protocol/v2/`, and the Core serves both for a transition period.
- **Reduce needs 1.2:** the Core never sends a `reduce` on a session that negotiated a version below 1.2. **At the Core upgrade to 1.2**, undelivered `reduce` frames created before it (1.0/1.1, without `refs`) are marked `expired`, audited, and never retransmitted, so an old frame cannot reach a still-1.1 plugin unguarded (Pêndulo L3). Routing skips it with `routing.adapter_protocol_outdated` (audited, alert), so a reduce never runs without the position-identity guard (§5). A 1.2 adapter rejects a reduce in a 1.0/1.1 envelope with `command.position_changed`; a 1.2-envelope reduce without `refs` is `protocol.invalid_message`, not acked (§5).
- An adapter older than the Core's minimum version is refused with 4003. The plugin shows "update required", and the web app shows the connection as *outdated*.
- **Minimum version (r5):** the Core negotiates 1.2 to the newest it knows and refuses older adapters with 4003. Nothing older was ever deployed, so the < 1.2 rules above remain only as defence.
- **`sync_reasons` on older sessions (r5):** sessions below 1.4 keep `maxItems` 20. The Core sends at most 20 reasons to them in `welcome` and `account_state`, most severe first.
- **Protocol 1.4 (r5, minor):** the §7.1 drawdown distances, the boundary instant, the re-anchor field with its watermark, the unknown-day risk state and code, and `sync_reasons` `maxItems` raised from 20 to 32 (the canon has 21 codes, §6.2). It also adds the text that `lock_at_balance` stops the floor from trailing. Drawdown in `limits` is sent only to sessions of 1.4 or later.

## 10. Testing

- **Fixtures:** in `contracts/adapter-protocol/v1/examples/`. Rust and C# must pass them (ADR-0002 §4).
- **`adapter-sim`:** implements the adapter side, with scripted fills, partial fills, rejects, slow acks, duplicate deliveries, disconnects mid-command and expired commands. It is used for Core integration tests and the M1 demo without NinjaTrader.
- **Plugin core library tests:**
  - outbox and dedupe across a simulated restart;
  - crash window: submit a `place`, let it fill at once, crash before persisting "submitted", restart. Expected: the filled order is adopted and reported, and no second order is submitted. Same for a protective leg; for `flatten`, the retry re-evaluates and reports `command.already_flat`;
  - stale re-delivery: record and execute a command, lose its ack, advance the clock 8 days (pruning runs), reconnect, and receive the same frame again. Expected: `rejected` + `command.stale` and no second execution. Core side: no retransmission after 72 h (`undelivered`, `out_of_sync`);
  - expiry with clock skew;
  - `depends_on` ordering;
  - orphan cancellation;
  - offline enforcement.
- **Windows checklist** on NinjaTrader 8 Sim/Playback: see the [NinjaTrader 8 spike plan](../spikes/nt8-spike-plan.md).

## 11. User decisions (2026-10-23)

| # | Item | Decision |
|---|---|---|
| T1 | Do exits expire? | **No.** Only entries expire; late exits are audited as late |
| T2 | Default command TTL | **5 s**, editable 1–60 s per account |
| T3 | External order or fill on a destination | **Out of sync** for master-replication destinations; **audit only** for TradingView-only destinations |
| Q18 | Unpairing a connection | Allowed with confirmation; its accounts become out of sync; nothing is closed |
| U13 | Late partial exit (`reduce`) when the position changed | **(A)** position-identity guard: `refs.position_ref`, `command.position_changed`, `out_of_sync`, audit; `flatten` unchanged (decided 2026-10-23; protocol 1.2, §5) |
| U15 | Live accounts in release plugin builds | A local opt-in in the plugin, "Allow live accounts", **off by default**, required in addition to the Core's `welcome.simulation_only = false` (decided 2026-10-23; ADR-0005 N11) |
| Q-O1 (Q27) | Double fill of a locally simulated bracket while NT8 was down (§6.1) | **(b)**: automatically flatten only the position created by the orphan leg, with a top-severity alert and audit (decided 2026-10-23) |

## 12. Consequences

- **Positive:**
  - No double execution across retries, reconnects and restarts: three independent dedupe layers (`msg_id`, `command_id`, `fill_id`/`event_id`).
  - No late entries.
  - Divergence is always surfaced, never auto-corrected.
  - Enforcement keeps working offline.
- **Negative:**
  - The adapter needs a durable local store and careful restart logic, which is the biggest plugin risk and the reason the spike covers it.
  - The protocol is chattier than fire-and-forget, which is acceptable at this scale.
  - V1 serves the adapter endpoint from a single Core instance (§2); horizontal scaling needs a later ADR.
- **Reversibility:** the transport can change in a new major without touching the Core domain, because the `gateway` maps protocol messages to venue-neutral commands and reports.
