# ADR-0004 — Core ↔ platform adapter protocol v1

- **Status:** Accepted (2026-10-23) — accepted by the user with answers T1–T3 (§11). Acceptance deltas, recorded in `contracts/adapter-protocol/CHANGELOG.md`: §6.1 bracket integrity on local-OCO connections (NT8 profile F5); money type with 4 decimals (ALIGN-1); token formats (ALIGN-3); pairing limits and HMAC storage (ALIGN-4); Q18 unpairing; Q-N1 dollar rounding; `fill.operation` and `account_reset` (NT8 profile §6)
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
- **WebSocket close codes:**

| Code | Meaning | Adapter behavior |
|---|---|---|
| 4000 | Normal shutdown (server restart) | Reconnect with backoff |
| 4001 | Invalid token | Stop. Show "pair again" |
| 4002 | Token revoked | Stop. Show "pair again" |
| 4003 | Unsupported protocol version | Stop. Show "update the plugin" |
| 4004 | Superseded by another session | Stop. Show "connected elsewhere" |
| 4005 | Heartbeat timeout | Reconnect with backoff |
| 4006 | Protocol violation (schema, ordering) | Reconnect with backoff; the Core logs the details |

## 3. Envelope and message catalogue

```json
{ "protocol_version": "1.0", "msg_id": "<uuid>", "type": "<type>", "sent_at": "<UTC Z>", "payload": { } }
```

| Type | Direction | Reliable (acked) | Purpose |
|---|---|---|---|
| `hello` | A→C | handshake | Adapter identity and version, supported protocol versions, visible platform accounts with kind (`simulation`/`playback`/`live`/`unknown`), count of pending outbound messages |
| `welcome` | C→A | handshake | Session and connection ids, negotiated version, heartbeat and ack timeouts, `simulation_only`, user locale, mapped accounts (`account_id`, roles `destination`/`master`, `sync_state`) |
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
| `risk_state` | A→C | no (latest wins) | Live P&L and remaining limits for display; throttled to at most 2 per second per account |

**Handshake:**

1. The adapter sends `hello` first. The Core answers `welcome`, or `error` followed by a close (4001–4003).
2. After `welcome`, the Core sends `limits` for all mapped accounts.
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
- **`flatten`:** closes the account's position in that contract and cancels **all** its working orders (orphan-order protection). The reason is `close` or `reverse`.
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
  - **`reduce` that cannot reduce** (flat, or same side): `cancelled` + `command.nothing_to_reduce`, never opening or flipping. A reduce larger than the position is capped (`order.quantity_capped`).
  - **Modify/cancel outcomes:** one outcome report per target under the modify/cancel `command_id`, never with fills. The Core classifies reports by the action of their command.
  - **Fill corrections** (`amend`/`remove`): the recomputed order-level status, the original `fill_id`, and an authoritative `position_after`.
- **Orphan-order protection in the adapter:** whenever an account's position in a contract becomes flat **for any reason**, the adapter immediately cancels the remaining All Win stop/target orders for it, and reports them `cancelled` with `reason_code: order.orphan_cancelled` (vision §8).
- **Expiry (`expires_at`)**:
  - **Computed by the Core:** `expires_at = expiry_base + account.command_ttl`, where `expiry_base = min(instruction.sent_at, received_at)` (ADR-0003 §7).
  - **Default TTL: 5 seconds.** Editable per account between 1 and 60 seconds (Q16: short default, per-account editable). The 5 s default is decided (T2).
  - **Core side:** the Core never sends a command whose `expires_at` has passed. Pending commands found expired after a reconnect or restart become `expired`, are audited, and the account is flagged **out of sync**.
  - **Adapter side:** the adapter rejects a command received after `expires_at`, using its clock corrected by the skew measured between `welcome.sent_at` and local receipt, and reports `status: expired`, `reason_code: command.expired`. A command already submitted to the platform is never cancelled because of expiry.
  - **Which commands carry it:** the schema **requires** `expires_at` on position-increasing commands (`place` with `position_effect: open`). For exits (`flatten`, `reduce`, `cancel`, protective `modify`) it is optional, and the Core omits it, so a late exit still reduces exposure; late exits are audited as late (T1, decided: only entries expire).

## 6. Synchronization, disconnects and out-of-sync (Q14)

The Core keeps, per account, an **expected state**: position per contract and All Win working orders, derived from commands and execution reports. Each account has a `sync_state` of `in_sync` or `out_of_sync`. Being out of sync is **sticky until the user clears it** in the web app (Q14).

| Trigger | Result |
|---|---|
| Command `rejected`, `expired`, or `partially_filled` without completion before the order's end of life | `out_of_sync` + alert |
| Disconnect while the account had an open position or a command in flight | `out_of_sync` + alert ("alert and wait") |
| Disconnect while flat with nothing in flight | `disconnected` only. On reconnect, if the snapshot shows flat with no All Win working orders, the state returns to normal automatically and the user is notified |
| Reconnect snapshot differs from the expected state (position or All Win working orders) | `out_of_sync` + alert listing the differences |
| Periodic snapshot (every 60 s while connected) differs | `out_of_sync` + alert |
| External order or fill on a destination account (`origin: external`) | Destinations of master replication: `out_of_sync` + alert. TradingView-only destinations: audit only (T3, decided) |
| Sim account reset or Playback rewind (`position_snapshot.trigger: account_reset`, NT8 `SimulationAccountReset`) | `out_of_sync` (`sync.simulation_reset`); the discontinuity is never interpreted as fills |
| Execution amended or removed by the platform (`fill.operation` `amend`/`remove`) | The Core corrects the position and P&L, audits it, and flags `out_of_sync` if the expected state no longer matches |
| Command not acked within the 72-h retransmission horizon (§4) | `out_of_sync` (`sync.command_undelivered`) + alert; the command is `undelivered` and never retransmitted |
| User unpairs the connection (Q18) | Every mapped account of that connection becomes `out_of_sync`; nothing is closed or cancelled |
| Bracket integrity cases on local-OCO connections | See §6.1 |

**While `out_of_sync`:**

- The Core refuses new position-increasing instructions for the account (`routing.out_of_sync`, ADR-0003 §4.1).
- **It never sends corrective orders on its own:** no reopen, no resync, no catch-up.
- The user sees the differences and either fixes the account manually and clears the flag, or flattens and clears.
- `welcome.accounts[].sync_state` and `sync_reasons` tell the plugin to show the state and its reasons in its window. Changes during the session reach the plugin through the C→A message `account_state` (protocol 1.1, sent only when 1.1 is negotiated). On 1.0 the plugin shows the `welcome` state plus its locally observed reasons until the next `welcome`.

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

## 7. Local risk enforcement (second barrier)

- **Limits:** the adapter caches the latest `limits` per account (`version` is monotonic) in its durable store, so enforcement works **with the Core offline** (S15 AC-4: feasible).
  - Monitored metrics: daily loss, drawdown (trailing intraday / end-of-day / static), optional daily profit target with `on_reach` (Q3), and maximum position.
- **Warnings:** at `warning_threshold_pct` (80 %), a warning is shown in the plugin and sent as a `risk_event`.
- **On a limit, with enforcement enabled:**
  1. Cancel working orders.
  2. Flatten.
  3. Block the account until the trading-day reset (`reset_local_time` in `time_zone`; no early unlock).
- **Blocking is reactive:** any new order on a blocked account is immediately cancelled, or flattened if already filled, and reported as `blocked_order_cancelled`. NinjaTrader has no known API to prevent order entry; the spike confirms.
- **Playback** uses the connection's time (`platform_time`) for resets and daily P&L.
- **Reporting:**
  - Every enforcement action produces `execution_report`s with `origin: local_enforcement` plus a `risk_event`.
  - `while_disconnected: true` marks actions taken offline. They are delivered from the outbox on reconnect.
- **Core-side gate:** the Core's own gate (ADR-0003 §4.1) uses the same limits and the reported state. The adapter never relaxes a block because of a Core message, except an `unblocked` state that follows a reset.

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
- **Rollback (newer frame, older session):** an outbox frame created in a newer-minor session is still retransmitted byte-identical if a later session negotiates an older minor (e.g. Core rollback from 1.1 to 1.0). The receiver accepts it when its `type` exists in the negotiated version and its payload validates against the schema the receiver knows; otherwise it answers `error` `protocol.unsupported_value` with `ref_msg_id` and does **not** ack it, so the frame stays in the outbox and the session is flagged for the user. Every A→C type exists in 1.0, so this case cannot lose an execution report.
- A new major lives in `contracts/adapter-protocol/v2/`, and the Core serves both for a transition period.
- An adapter older than the Core's minimum version is refused with 4003. The plugin shows "update required", and the web app shows the connection as *outdated*.

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
- **Reversibility:** the transport can change in a new major without touching the Core domain, because the `gateway` maps protocol messages to venue-neutral commands and reports.
