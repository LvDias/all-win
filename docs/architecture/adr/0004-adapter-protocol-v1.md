# ADR-0004 — Core ↔ platform adapter protocol v1

- **Status:** Proposed (2026-10-23) — awaiting user acceptance
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
  - The Core stores only `key_id` and a SHA-256 hash.
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
| `instruction_result` | C→A | **yes** | Outcome of a master `instruction` (accepted, duplicate, rejected + code) |
| `execution_report` | A→C | **yes** | Every order event on mapped accounts: All Win, local enforcement and external origins |
| `position_snapshot` | A→C | **yes** | Positions and working orders per mapped account |
| `instruction` | A→C | **yes** | Master-account input: a standard instruction (ADR-0003), validated with the same schema |
| `risk_event` | A→C | **yes** | Warning (80 %), limit reached, flattened, orders cancelled, blocked/unblocked, blocked-account order cancelled |
| `risk_state` | A→C | no (latest wins) | Live P&L and remaining limits for display; throttled to at most 2 per second per account |

**Handshake:**

1. The adapter sends `hello` first. The Core answers `welcome`, or `error` followed by a close (4001–4003).
2. After `welcome`, the Core sends `limits` for all mapped accounts.
3. The adapter must send a `position_snapshot` (`trigger: reconnect`) **before** the Core sends any new `command` in that session (§6).
4. The Core picks the highest protocol version listed in both `hello.supported_protocol_versions` and its own support.
5. Platform accounts the user has not mapped are ignored by the adapter: no commands, no reports, no enforcement.

## 4. Delivery guarantees

- **At-least-once delivery with idempotent receivers:**
  - **Reliable messages** stay in the sender's **durable outbox** until acknowledged. The adapter outbox is a local file store under the NinjaTrader user folder, chosen in the spike; the Core outbox is a PostgreSQL table.
  - Unacked messages are retransmitted after `ack_timeout_ms` (default 5000) and on every reconnect, in original order, with the **same `msg_id`**.
  - Receivers dedupe by `msg_id` for at least 24 h.
- **Business idempotency on top of transport dedupe:**
  - **`command_id`** is deterministic (ADR-0003 §5). The adapter keeps a **durable record of every `command_id` it has acted on** for at least the current and previous trading day. A command already handled is acknowledged and answered with an `execution_report` `status: duplicate` that repeats the latest known state; it is never executed again.
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
  - For `open`, it carries **`protection`**: mode `individual`|`shared`, `stop_ticks`, `target_ticks`. The Core normalizes the profile's points/ticks/dollars into whole ticks using the catalog's exact tick size and tick value.
  - The adapter computes protective prices from the **fill price** (individual) or the **average position price** (shared, re-computed on every add), rounds to tick size, and reports the protective orders as `execution_report`s with `child_role` `stop`/`target` under the same `command_id`.
- **`modify` / `cancel`:** target orders created by All Win, identified by `(command_id, child_role)`, plus the `platform_order_id` when known. `modify` applies `price_delta` and/or a new total `quantity`.
- **`flatten`:** closes the account's position in that contract and cancels **all** its working orders (orphan-order protection). The reason is `close` or `reverse`.
- **Orphan-order protection in the adapter:** whenever an account's position in a contract becomes flat **for any reason**, the adapter immediately cancels the remaining All Win stop/target orders for it, and reports them `cancelled` with `reason_code: order.orphan_cancelled` (vision §8).
- **Expiry (`expires_at`)**:
  - **Computed by the Core:** `expires_at = expiry_base + account.command_ttl`, where `expiry_base = min(instruction.sent_at, received_at)` (ADR-0003 §7).
  - **Default TTL: 5 seconds.** Editable per account between 1 and 60 seconds (Q16: short default, per-account editable). The 5 s value is a proposal pending user confirmation (§11, T2).
  - **Core side:** the Core never sends a command whose `expires_at` has passed. Pending commands found expired after a reconnect or restart become `expired`, are audited, and the account is flagged **out of sync**.
  - **Adapter side:** the adapter rejects a command received after `expires_at`, using its clock corrected by the skew measured between `welcome.sent_at` and local receipt, and reports `status: expired`, `reason_code: command.expired`. A command already submitted to the platform is never cancelled because of expiry.
  - **Which commands carry it:** the schema **requires** `expires_at` on position-increasing commands (`place` with `position_effect: open`). For exits (`flatten`, `reduce`, `cancel`, protective `modify`) it is optional, and the Core omits it by default, so a late exit still reduces exposure. This depends on user confirmation (§11, T1); if the user wants exits to expire too, the Core simply sets the field and no protocol change is needed.

## 6. Synchronization, disconnects and out-of-sync (Q14)

The Core keeps, per account, an **expected state**: position per contract and All Win working orders, derived from commands and execution reports. Each account has a `sync_state` of `in_sync` or `out_of_sync`. Being out of sync is **sticky until the user clears it** in the web app (Q14).

| Trigger | Result |
|---|---|
| Command `rejected`, `expired`, or `partially_filled` without completion before the order's end of life | `out_of_sync` + alert |
| Disconnect while the account had an open position or a command in flight | `out_of_sync` + alert ("alert and wait") |
| Disconnect while flat with nothing in flight | `disconnected` only. On reconnect, if the snapshot shows flat with no All Win working orders, the state returns to normal automatically and the user is notified |
| Reconnect snapshot differs from the expected state (position or All Win working orders) | `out_of_sync` + alert listing the differences |
| Periodic snapshot (every 60 s while connected) differs | `out_of_sync` + alert |
| External order or fill on a destination account (`origin: external`) | Reported and audited. Whether it marks the account out of sync is decided in S11 (§11, T3) |

**While `out_of_sync`:**

- The Core refuses new position-increasing instructions for the account (`routing.out_of_sync`, ADR-0003 §4.1).
- **It never sends corrective orders on its own:** no reopen, no resync, no catch-up.
- The user sees the differences and either fixes the account manually and clears the flag, or flattens and clears.
- `welcome.accounts[].sync_state` tells the plugin to show the state in its window.

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
| Storage | SHA-256 hash only; never logged in clear (code prefix at most) |
| Brute force | 5 wrong attempts against a connection's active code invalidate it. Per source IP: 10 attempts per minute on the pairing endpoint. Global anomaly alert in Core logs |
| Re-pairing | Generating a new code for an already paired connection lets the user move to a new machine. Successful re-pairing **revokes the previous device token** (its session closes with 4002) |
| Revocation | "Unpair" in the web app revokes the token immediately |
| Device token | `awd_<key_id>_<secret>`, 256-bit secret, no expiry while in use. It is revoked by unpairing or re-pairing. Rotation on demand comes later if required |
| Environment guard | Outside production, `simulation_only` is always true and the Core refuses to map platform accounts reported as `live` or `unknown`. Development adapter builds refuse non-simulation accounts on their own as well |

## 9. Versioning

- `protocol_version` is `major.minor`. Additive optional fields and new enum values bump the minor, and receivers must ignore nothing silently: unknown enum values from a newer minor are answered with `error` `protocol.unsupported_value`.
- A new major lives in `contracts/adapter-protocol/v2/`, and the Core serves both for a transition period.
- An adapter older than the Core's minimum version is refused with 4003. The plugin shows "update required", and the web app shows the connection as *outdated*.

## 10. Testing

- **Fixtures:** in `contracts/adapter-protocol/v1/examples/`. Rust and C# must pass them (ADR-0002 §4).
- **`adapter-sim`:** implements the adapter side, with scripted fills, partial fills, rejects, slow acks, duplicate deliveries, disconnects mid-command and expired commands. It is used for Core integration tests and the M1 demo without NinjaTrader.
- **Plugin core library tests:**
  - outbox and dedupe across a simulated restart;
  - expiry with clock skew;
  - `depends_on` ordering;
  - orphan cancellation;
  - offline enforcement.
- **Windows checklist** on NinjaTrader 8 Sim/Playback: see the [NinjaTrader 8 spike plan](../spikes/nt8-spike-plan.md).

## 11. Open items for the user (through the PO)

| # | Item | Proposal |
|---|---|---|
| T1 | Do exits (`flatten`, `reduce`, `cancel`, protective `modify`) expire like entries? | No. Late exits still reduce exposure, and the audit marks them as late. Entries always expire |
| T2 | Default command TTL value | **5 s** (editable 1–60 s per account) |
| T3 | Does an external (manual) order or fill on a destination account mark it out of sync? | Yes for destinations of master replication. For TradingView-only destinations, audit only |

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
