# AllWin.Adapter.Core — design (V1-S2 prep)

- **Owner:** Plugin Engineer
- **Inputs:** ADR-0004 (protocol v1), ADR-0005 (NT8 decisions, Proposed), NT8 platform profile, spike plan rev. 3, slice plan S2/S9/S11/S15.
- **Status:** implemented and verified on Linux. NT8 host-level behavior is unverified (§8).

## 1. Shape

```
NT8 callbacks ──copy──▶ EngineLoop (one thread) ◀── WebSocketConnector (receive task)
                          │                             ▲ serialized send queue
                          ▼                             │
                     AdapterEngine ── IFrameSink ───────┘
                      │        │
          ITradingPlatform   IDurableStore (FileJournalStore)
          (NT8 host / SimulatedPlatform)
```

- **`AdapterEngine`** is single-threaded. Its inputs are:
  - `OnConnected`, `OnFrame` and `OnDisconnected` from the transport;
  - `OnPlatformEvent` from the platform;
  - `OnTimer`, about every 250 ms.

  `EngineLoop` serializes these inputs. NT event handlers only copy the event args and post them (ADR-0005 N6).
- **`ITradingPlatform`** is the only platform port. Order identity is a host-assigned `Handle` per order object, because NT `Order.OrderId` changes. Reconciliation after a restart uses `Order.Name = "aw:<command_id>:<e|s|t|x>"` (N5).
- **Validation.** Every frame is validated in both directions against the canonical `messages.schema.json`, using a dependency-free JSON Schema 2020-12 validator. The schemas are embedded as generated C# constants, so the NT8 source loop gets them. An unknown schema keyword fails at load time.

## 2. Durability and exactly-once

| Layer | Mechanism |
|---|---|
| Store | `FileJournalStore` is an append-only journal plus an atomic snapshot (write tmp → fsync → rename). Each commit is one line, CRC-protected and fsynced before it returns. A torn last line is dropped on open. Damage anywhere else is a `StoreCorruptException`: the adapter stops (fail closed). An exclusive lock file prevents two processes from sharing a store |
| Write-ahead | State changes are committed before any frame is sent and before any platform call they cause. A command is marked `SubmitAttempted` and flushed before `Submit`. If the process dies after that point and the platform has no order with that name, recovery reports `unknown` and **never resubmits** |
| Transport | Reliable outbound messages stay in the outbox until acked. They are retransmitted after `ack_timeout_ms` and on reconnect, byte-identical: same `msg_id`, same `sent_at` |
| Inbound dedupe | `msg_id` (transport) and `command_id` (business, answered `duplicate` with the last known state). Retention is 7 days (answer 8). Pruning only drops final commands |
| Staleness | Any command whose envelope `sent_at` is older than 6 days on the skew-corrected clock is answered `rejected`/`command.stale` and recorded, with or without a dedupe record (ADR-0004 §4, "bounded re-delivery"). Invariant: 7 d > 6 d > Core horizon 72 h |
| Fills | `(account, fill_id)` is recorded. Replays after a restart are suppressed. A changed replay becomes `amend`. `remove` needs a known fill |
| Events | `event_id` is generated once per report. It is persisted in the outbox frame before sending |

## 3. Session

1. `hello` → `welcome`.
2. The skew is measured as `welcome.sent_at − local receipt`, giving `CoreNow = local + skew`.
3. The mapping is persisted. The outbox is retransmitted in its original order. Then `position_snapshot(reconnect)` is sent. Only after that are commands dispatched (answer 6).
4. A command that arrives before the snapshot is queued, never rejected, and a protocol warning is logged.
5. Heartbeats run every `heartbeat_interval_ms`. Three missed intervals close the session with 4005.
6. Close codes:
   - 4001 and 4002 stop the adapter (pair again);
   - 4003 stops it (update);
   - 4004 stops it (connected elsewhere);
   - anything else reconnects. Backoff starts at 1 s, doubles, caps at 30 s, ±20 % jitter, and resets after a session that reached Ready.
7. The transport refuses non-`wss` endpoints, except `ws://` on loopback for the harness.

## 4. Commands

- **Per-account FIFO.**
  - The queue advances when the head command is handed to the platform.
  - `flatten` holds the queue until it is final.
  - `depends_on` waits for a final prerequisite. A failed or unknown prerequisite gives `command.prerequisite_failed`.
- **Checks at intake and again at dispatch:**
  - staleness;
  - mapping and `destination` role;
  - the simulation guard;
  - platform connection (`command.platform_disconnected`);
  - entry expiry with the skew-corrected clock (`command.expired`; exits never expire);
  - `risk.account_blocked`, for entries only.
- **Guard on every write.** `Submit`, `Change` and `Cancel` all go through `EnsureWritable`. Automatic actions are covered too: orphan cancels, bracket resizes and flatten steps.
- **`reduce`.**
  - A reduce never opens or flips a position.
  - When the account is flat or on the same side: `cancelled`/`command.nothing_to_reduce` (answer 11).
  - When the quantity is larger than the position: it is capped, with `order.quantity_capped` on the working report.
- **`modify`/`cancel`.**
  - Each target gets one outcome report under the modify/cancel `command_id`, without `fill` or `position_after` (answer 9a).
  - The outcome is decided from the platform's confirmed state, not from the API call.
  - Lifecycle events stay under the owning `(command_id, child_role)`.
  - The command is a success only if every target was applied. There is no rollback on partial application.
- **`flatten`.**
  1. Cancel every working order on the contract.
  2. Wait for the confirmations (10 s timeout → `order.cancel_failed`).
  3. Close the remaining position with a market order (role `x`). If the position changed meanwhile, retry, up to 3 attempts.
  4. Already flat → `cancelled`/`command.already_flat`, a terminal success (answer 4).
  5. `reason: orphan_fill` (Q-O1 b) is executed the same way.
- **Entry orders** use `DAY`. Protective orders use `GTC` (approved).

## 5. Brackets and orphan protection

- **Individual mode.**
  - The bracket is submitted on the entry's first fill, sized to the filled quantity, as a stop-market plus a limit sharing an OCO id (`aw:<command_id>`).
  - It is resized and re-priced on each further fill with one `Change`.
  - Prices come from the entry's VWAP. Off-tick results are rounded toward the entry, i.e. the tighter side (answer 7).
  - A failed resize or re-price gives `order.protection_failed`. There is no automatic retry.
- **Shared mode (answer 1).**
  - The command that opened the position owns one bracket sized to the whole position and priced from the platform's average price, using the owner's ticks.
  - Adds get entry reports only.
  - Ownership ends when the position is flat.
- **Orphan rule (S9).**
  - On every flat `PositionUpdate`, the remaining All Win stop/target orders on that contract are cancelled, with `order.orphan_cancelled`.
  - In individual mode, the opposite leg of a fully closed entry is cancelled even while other entries stay open (S9 AC-2).
  - A failed cancel is reported as `working` + `order.cancel_failed`.
- **Recovery after a restart** (ADR-0004 §6.1). Orders are re-linked by name. Lifecycle changes that happened while the adapter was down are reported. Executions are caught up (deduped). Then:

  | Found | Adapter action | Local reason shown in the plugin |
  |---|---|---|
  | Flat with a leg still working | Cancel it (orphan rule) | `sync.bracket_orphan_leg` |
  | Both legs filled | Nothing; the Core sends `flatten(orphan_fill)` | `sync.bracket_double_fill` |
  | Open position with a leg `unknown` or missing | Nothing (alert and wait) | `sync.bracket_incomplete` |
  | Sim reset / Playback rewind | Snapshot `account_reset`. Platform history is recorded as already seen, never as fills | `sync.simulation_reset` |

  Local reasons are hints and persist across restarts until the next `welcome`. The Core's `sync_state`/`sync_reasons` are authoritative. `sync.bracket_integrity_unknown` is derived live: protective orders are working, OCO is not `server`, and the Core session or the broker connection is down.
- **Account view.** `ApplyCoreAccountState` already takes protocol 1.1 `account_state`, with the `changed_at` ordering: older updates are ignored and equal ones are applied.

## 6. Architect decisions applied (Esquadro, 2026-10-23)

| # | Decision |
|---|---|
| 1 | The shared bracket is owned by the opening command. A target that names an add's `command_id` gives `command.target_not_found` |
| 2 | Adapter reason codes as in `ReasonCodes.cs`. `protocol.*` codes appear only in `error` messages |
| 3 | Interim view from `welcome`. 1.1 `account_state` comes later |
| 4 | flatten on a flat account gives `cancelled` + `command.already_flat` (success) |
| 5 | Expiry is re-checked right before submission |
| 6 | Order: welcome → outbox → reconnect snapshot → commands |
| 7 | Bracket from the entry VWAP, rounded toward the entry. One Change for quantity and price |
| 8 | Dedupe retention 7 days; prune only final records |
| 9 | Outcome reports for modify/cancel, without fills |
| 10 | Amend/remove keep the recomputed label and the original `fill_id` |
| 11 | `command.nothing_to_reduce` / `order.quantity_capped`. GTC protection. Owner's ticks in shared mode |
| CR-4 | `command.stale` (6 days), byte-identical retransmission |

## 7. Limits of this step

- Local risk enforcement (S15) is limited to storing `limits` by monotonic version and refusing entries while `blocked_until` is in the future. P&L monitoring, warnings, flatten-and-block and trading-day resets are not built yet.
- Master-account input (`instruction`, S19) is not built. The validator already covers the embedded instruction schema.
- The NT8 AddOn host (`net48`) and its UI are not built (no NT8 DLLs on Linux).

## 8. Open items for the NT8 spike (Windows)

| Item | Experiment |
|---|---|
| Event order of `OrderUpdate` / `ExecutionUpdate` / `PositionUpdate`. Is the position updated when `ExecutionUpdate` fires (if not, the host omits `PositionAfter` and the engine queries)? | E4 |
| `Order.Name` survives an NT8 restart. `Account.Orders` / `Executions` contents after a restart | E4, E12 |
| Provider enum names for Simulated Data Feed and Playback (`AccountClassifier`), and `Mode` on Sim101 under each connection | E4 |
| A rejected `Change`: does the order return to Working with the old values? (the engine treats that as a failed target) | E4 |
| OCO partial-fill behavior of local simulated OCO (the engine resizes the other leg itself) | E4, E11 |
| Durable store on NTFS: rename semantics, the lock, fsync cost per commit | E12 |
| `Dispatcher.InvokeAsync` for the window. WebSocket under load next to order traffic | E9 |
