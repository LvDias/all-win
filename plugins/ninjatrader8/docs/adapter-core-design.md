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
| Store | `FileJournalStore` is an append-only journal plus an atomic snapshot (write tmp → fsync → rename). Each commit is one line, CRC-protected and fsynced before it returns. The **last** line is an unconfirmed (torn) write when it has no newline or fails its CRC (after a power loss its sectors can land out of order). It is dropped on open, logged and shown in the plugin (`StoreRecoveredTornWrite`, Architect L3 option 2). This is safe: the store is write-ahead, so nothing was acted on, and the Core re-delivers the unacked command. A line that fails its CRC before the last one is a `StoreCorruptException`: the adapter stops (fail closed). An exclusive lock file prevents two processes from sharing a store. The directory is not fsynced after the rename (NTFS is fine; ext4 is not the target), to be checked in E12 |
| Write-ahead | State changes are committed before any frame is sent and before any platform call they cause. A command is marked `SubmitAttempted` and flushed before `Submit`. If the process dies after that point and the platform has no order with that name, recovery reports `unknown` and **never resubmits** |
| Unknown outcomes | A host exception other than `PlatformException` during `Submit` means the call died without a verdict. The engine asks the platform again: if an order carries the exact reference, it is adopted; otherwise the result is `unknown`. Before every submission, an order with the same `(command_id, role)` is adopted and its last state reported instead of submitting. For `place` and protective legs any match counts, live or terminal; this covers submit → immediate fill → crash. Only a flatten close ignores terminal matches: its retries reuse `aw:<cmd>:x` after re-evaluating the position. Two or more adoptable matches give `rejected` + `order.ambiguous_reference`, and nothing is submitted (answer G as corrected by Crivo, additions 2/3). If the lookup itself fails, nothing is submitted either (`command.platform_disconnected`; legs: `order.protection_failed`; QA L4). Any other unexpected exception is a bug: nothing more is committed or sent, and the adapter stops (`StopReason.InternalError`, fail closed) |
| Transport | Reliable outbound messages stay in the outbox until acked. They are retransmitted after `ack_timeout_ms` and on reconnect, byte-identical: same `msg_id`, same `sent_at` |
| Inbound dedupe | `msg_id` (transport) and `command_id` (business, answered `duplicate` with the last known state, also when stale; answer E). Retention is 7 days (answer 8). Pruning only drops final commands that no longer protect anything: no live legs, no shared ownership, no open individual bracket on a non-flat contract (Code Review 6 H2). While an account is absent or disconnected (e.g. NT8 started before its connection), its records are kept: unknown means keep. Fill records are kept while the platform still holds the execution, or while the account cannot be observed |
| Staleness | A command with **no** record whose envelope `sent_at` is older than 6 days on the skew-corrected clock is answered `rejected`/`command.stale` and recorded (ADR-0004 §4, "bounded re-delivery"). It is checked at intake and again at dispatch. Invariant: 7 d > 6 d > Core horizon 72 h |
| Fills | `(account, fill_id)` is recorded. Replays after a restart are suppressed. A changed replay becomes `amend`. `remove` needs a known fill |
| Events | `event_id` is generated once per report. It is persisted in the outbox frame before sending |

## 3. Session

1. `hello` → `welcome`. `hello` advertises `[1.0, 1.1]` and always travels as 1.0. Frames created after `welcome` use the negotiated version; outbox frames keep their original bytes (answer F). On 1.1 sessions, `account_state` updates the account view (`changed_at` rule). On a 1.0 session (e.g. after a Core rollback) it is refused with `error` `protocol.unsupported_value` + `ref_msg_id`. A newer-minor frame whose type exists in the negotiated version and whose payload validates is accepted.
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
8. When the engine closes the session (e.g. 4005), the close handshake gets a 2 s grace. After that, the socket is aborted and the receive is cancelled, so a half-open TCP connection never delays the reconnect (Code Review 6 M2).

## 4. Commands

- **Per-account FIFO.**
  - The queue advances when the head command is handed to the platform.
  - `flatten` holds the queue until it is final.
  - `depends_on` waits for a final prerequisite. A failed or unknown prerequisite gives `command.prerequisite_failed`.
- **Pump.** It is not re-entrant: a command finishing during a dispatch does not run a nested loop. The queue removes commands by id.
- **Checks at intake and again at dispatch.** The account checks are deferred to dispatch when the command arrives before this session's `welcome` (queued, never rejected):
  - staleness;
  - mapping and `destination` role;
  - the simulation guard;
  - platform connection (`command.platform_disconnected`);
  - entry expiry with the skew-corrected clock (`command.expired`; exits never expire);
  - `risk.account_blocked`, for entries only, comparing `blocked_until` on the Core clock.
- **Guard on every write.** `Submit`, `Change` and `Cancel` all go through `EnsureWritable`. Automatic actions are covered too: orphan cancels, bracket resizes and flatten steps.
- **`reduce`** (protocol 1.2, ADR-0004 §5 "Position-identity guard", U13). Steps in order, after the account checks:
  0. Crash-window lookup by `aw:<command_id>:x`: any match is adopted and reported; 2+ → `order.ambiguous_reference`; a failed lookup never submits.
  1. Flat → `cancelled`/`command.nothing_to_reduce` (answer 11).
  2. `refs.position_ref` ≠ the identity of the current position, an `unknown`/`external` identity, a pre-1.2 envelope (no `refs`), or an order that does not oppose the position → `rejected`/`command.position_changed`, nothing submitted. A same-side position is therefore `position_changed` from 1.2.
  3. Submit, capped to the position (`order.quantity_capped`). A reduce never opens or flips a position.
- **`modify`/`cancel`.**
  - Each target gets one outcome report under the modify/cancel `command_id`, without `fill` or `position_after` (answer 9a).
  - The outcome is decided from the platform's confirmed state, not from the API call.
  - Lifecycle events stay under the owning `(command_id, child_role)`.
  - The command is a success only if every target was applied. There is no rollback on partial application.
- **`flatten`.**
  1. Cancel every working order on the contract.
  2. Wait for the confirmations (10 s timeout → `order.cancel_failed`).
  3. Close the remaining position with a market order (role `x`). If the position changed meanwhile, retry, up to 3 attempts. A close order that is still not final after 60 s releases the account queue as `unknown`.
  4. After a restart: Cancelling resumes. Closing never sends another close order. A close order the platform does not hold gives `already_flat` when flat, else `unknown`.
  4. Already flat → `cancelled`/`command.already_flat`, a terminal success (answer 4).
  5. `reason: orphan_fill` (Q-O1 b) is executed the same way.
- **Entry orders** use `DAY`. Protective orders use `GTC` (approved). A `place` is a success for `depends_on` only when filled (registry §3).

## 5. Brackets and orphan protection

- **Invariant (answer B).** After every fill on a contract (entry, protective leg, exit, flatten or external), the brackets are reconciled so the protective quantity never exceeds the position it protects. This uses the execution's own `position_after`. Individual brackets lose protection oldest entry first (FIFO, P2). The shared bracket is sized to `|position|`. A leg with nothing left to protect, or on the wrong side after a flip, is cancelled with `order.orphan_cancelled`. A modify that asks for more than this is capped (`order.quantity_capped`, answer D).
- **Live fills only (Architect R1).** A bracket is submitted, extended or re-priced only for a fill observed live. Live means all three: (a) the account's connection is up when the fill is received; (b) no connection loss since the entry was submitted, tracked as a persisted per-account epoch; (c) the fill is at most `MaxBracketDelay` (10 s, tuned by E4) old on the platform clock. A fill more than 1 s "in the future" also fails (c), as fail-safe. The platform clock comes from `ITradingPlatform.GetPlatformTime`, in UTC: the PC clock for live and Sim connections, the replay time for Playback. If Playback exposes no replay time, (c) is skipped for Playback only. `Execution.Time` must be UTC, explicitly converted by the host; any other `DateTimeKind` fails (c). The plugin shows a clock warning (`ClockSkewWarning`) when the PC−Core skew, measured at `welcome` and on each heartbeat, exceeds `MaxBracketDelay`. Otherwise every leg gets `order.protection_not_submitted` and existing legs only shrink. This covers a broker outage while NT8 keeps running (S11 AC-5 iv).
- **Restart catch-up (answer A + addition 1).** Fills that happened while the plugin was down are reported. A bracket that was never submitted is not submitted late: each withheld leg is reported `rejected` + `order.protection_not_submitted`, and is never submitted later. Existing legs may only be resized down, never up and never re-priced. A position that grew counts as incomplete. `sync.*` codes stay Core-only; the plugin shows a local hint.

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

## 5a. Position identity (U13)

- **Record** (`PositionIdentityRecord`, collection `pid`, key `account|symbol|month`): identity (`command_id`, `external` or `unknown`), signed quantity, last fill id, opening fill id. It is written in the same journal batch as the fill that changes it and removed when the position goes flat.
- **Rules.** A fill from flat, or a flip, sets the identity: the fill's own command when it is an All Win entry (`e`), `external` for a non-All Win fill, `unknown` for any other All Win role. Adds and partial closes (All Win or external) keep it. It is separate from the shared-bracket owner.
- **Unknown (fail closed)** when the record cannot explain the position: a fill that does not take the recorded quantity to the platform's `position_after`; a correction (`amend`/`remove`) of the opening fill; `SimulationAccountReset`; at startup after catch-up and when an account connection comes back, a quantity or latest execution that differs from the record (`VerifyIdentities`); at the guard, a live position that differs from the record (an execution still queued). A missing record with an open position is `unknown` (journal lost, or a position opened before 1.2).
- **Catch-up.** Executions replayed after a restart update the record like live ones, so an opening fill the platform still lists is observed. Retention and ordering of NT8 executions across a restart are E4/E9 (untested); a gap makes the identity unknown through the consistency check.

## 5b. Live accounts (ADR-0005 N11, U15; U19/U19.1 proposed)

- **Opt-in document.** `LiveConsentState.Parse` accepts only the canonical text `Serialize` writes; anything else, including any exception while reading it, is off.
- **Gates.** Release builds only. State B = kind `live` + confirmed in the local opt-in (`ILiveAccountConsent`, key connection/provider/platform account) + opt-in enabled + `welcome.simulation_only = false` + the command received after the opt-in was turned on. Development builds never write to a non-simulation account. Unconfirmed accounts are state A. Only a **known** non-live kind (simulation or playback) invalidates a confirmation, whether the account is mapped or connected or not (stateless rule). A transient `kind: unknown` is state A for every Core write but keeps the confirmation: the plugin does not self-cancel row-2 orders then, still performs rows 3–6, returns to the previous state when the kind is live again, and alerts after 10 s (`live.kind_unknown_alert`).
- **Table.** `LiveAccountTable.Allows(row, state, U19, U19.1)`. States C0/C1/C2 come from the position and its identity per contract. U19 (b) makes C behave as A; U19.1 (a) makes row 11 Y in C1 and C2. Defaults are the recommended options (U19 a, U19.1 b).
- **Who initiates.** On effective state A (A, or C under U19 b) a Core modify/cancel is accepted only for rows 2, 3 and 5; rows 4 and 6 there are the plugin's own initiative. A write that classifies to no row is refused outside B.
- **Identity reads.** Table checks read the position identity without persisting `unknown` (a position ahead of its record reads as C2 for that check only); only the reduce guard persists it (ADR-0004 §5 "at the guard").
- **Where it applies.** Commands at intake and dispatch (`CommandAllowedOnAccount`: consent first, then the table, then the command's own checks, so a reduce on A/C2 is `command.account_not_simulation`, never `position_changed`); modify/cancel per target (rows 1, 2, 3, 4, 5, 7, 8, 11, from the platform's own state); every platform write through `EnsureWritable(account, instrument, rows)`; bracket upkeep (row 6 = only consented entries' own fills, capped to what they filled; in shared mode the owner's legs also cover consented shared adds on its side; row 7/8 refused → no upsize/re-price, `sync.bracket_incomplete` hint).
- **On its own** (`EnforceLiveTable`: at startup, on the opt-in change, on an account change, on welcome, on reconnection): outside state B, working All Win entries are cancelled (row 2, `order.live_consent_cancelled`); in effective state A working All Win exits too; orphan legs are cancelled (row 3) and legs only shrink (rows 4–5). Only `aw:` orders are touched.
- **Alerts.** On A every action raises `live.unconsented_actions` in the account view and a log line. A flatten whose close fails on a non-simulation account raises `live.unprotected_position` (the one known I2 gap).
- **Not in the protocol.** The Core learns about the opt-in only from rejections (ADR-0005 N11).
- **Rows not named by the table** (Architect 2026-10-23): undoing a working All Win exit is row 2; a modify of a non-protective order is row 1. `order_targets` (contract v1) cannot name an exit, so the exit case exists only as the plugin's own action.

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
| A | No late bracket after a catch-up fill: `order.protection_not_submitted` per leg. Legs only resized down during catch-up |
| B | Protective qty ≤ position: FIFO / resize, including manual closes. Wrong-side legs are cancelled (`order.orphan_cancelled`) |
| C | `rejected` + `cancel_failed`/`protection_failed` = the operation failed; the order keeps its state |
| D | Modify quantity above the position is capped (`order.quantity_capped`) |
| E | A known `command_id` gets `duplicate` even when stale. `stale` only applies to unknown commands |
| F | Protocol 1.1: `account_state`, negotiated envelope version, outbox bytes unchanged |
| G | Adopt an order with the exact reference before submitting (any state for place/legs, live only for flatten closes). Several matches → `order.ambiguous_reference` |
| U13 | Protocol 1.2 position-identity guard on `reduce` (§4, §5a). `flatten` unchanged |
| N11 | Live-account opt-in and decision table (§5b). U19/U19.1 behind `EngineOptions`, recommended defaults |
| N11+ | `order.live_consent_cancelled` (cancelled, entry/exit) for the plugin's own row-2 cancels; code-only registry addition (all protocol versions) |

## 7. Limits of this step

- Local risk enforcement (S15) is limited to storing `limits` by monotonic version and refusing entries while `blocked_until` is in the future. P&L monitoring, warnings, flatten-and-block and trading-day resets are not built yet.
- Master-account input (`instruction`, S19) is not built. The validator already covers the embedded instruction schema.
- The NT8 AddOn host (`net48`) and its UI are not built (no NT8 DLLs on Linux). That includes the "Allow live accounts" dialog, its DPAPI-protected file (`LiveConsentState.Serialize`/`Parse` is the format) and the window state line.

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
| NT8 execution retention and order across a restart (position identity, `VerifyIdentities`) | E4, E9 |
| `Connection.Options.Provider` names for live connections (the consent key) | E4 |
| Event order and timestamps at a broker reconnect; tuning `MaxBracketDelay`; the platform clock for `GetPlatformTime` (`Connection.Now` vs `PlaybackConnection.Now`) | E4, E5 |
| Does `Account.Orders` keep terminal orders after a restart? Answer G's adoption of terminal matches depends on it | E4 |
