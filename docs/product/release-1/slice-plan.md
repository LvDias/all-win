# Release 1 (V1) — Slice Plan — All Win

> **Owner:** Product Owner (Bússola @ Dev2) · **Status:** approved by the user on 2026-10-23 ([decision log](../../delivery/decisions.md)) · **Revision:** 3 (Q1–Q16 and A2–A5 decisions recorded) · **Date:** 2026-10-23
> **Baseline:** [product-vision.md](../product-vision.md) rev. 3 (user-approved). This plan does not change any decision there; gaps found are listed in §7 as questions.
> Labels: **[Decision]** = from the vision · **[Proposal]** = recommendation still awaiting user approval (items approved on 2026-10-23 are now marked [Decision 2026-10-23]) · **[Assumption]** · **[Blocker]**.
> Slice status vocabulary: `draft` (criteria incomplete or blocked by a question) · `blocked` (criteria complete, waiting on an external dependency) · `ready` (criteria complete, can start) · then specified → implemented → QA verified → deployed → business accepted (tracked by the Maestro on the board).
> Technology-agnostic: surfaces are named by role (Web, Core, TradingView Input, NinjaTrader plugin), never by implementation.

---

## 1. Release 1 scope recap

| Item | V1? | Basis |
|---|---|---|
| Web application (pt-BR + en), per-user isolation | Yes | §2, §11 [Decision] |
| Account & risk management, optional enforcement, 80% warnings, trading-day reset | Yes | §9 [Decision] |
| Prop firm presets — all 18 firms, verified on official rules pages | Yes | §9 [Decision] |
| TradingView **Phase 1** (webhook) as Input | Yes | §7 [Decision] |
| NinjaTrader 8 plugin: Output, Input (master account), live enforcement, trade capture | Yes | §4 [Decision] |
| Master-account replication (NinjaTrader → NinjaTrader, multi-instance via Core) | Yes, sequenced after the TradingView path | §8 [Decision]; delivered as V1.1 after the TradingView path (Q9) [Decision 2026-10-23] |
| Performance analysis Phase 1 (adapter-computed metrics + manual entry) | Yes | §10 [Decision] |
| **Tradovate** Output | **No — release 2.** V1 must stay platform-neutral in the Core (checked by cross-cutting criterion X-1) | §4 [Decision] |
| TradingView **Phase 2** (own signal engine) | **No.** Depends on licensed market data whose provider, cost and licensing are still open (§7, §13-3); Phase 1 delivers the same user value (signals → orders) without that dependency, and Input is an adapter, so Phase 2 can be added later without touching Core/Output | [Decision 2026-10-23, plan approval] |
| Performance Phase 2 (trade-on-chart with licensed data) | No — same data dependency | [Decision 2026-10-23, plan approval] |
| Out of scope per vision: copy trading between people, browser extension, levels auto-import, billing, strategy backtesting, broker auto-sync, mobile apps | No | §3 [Decision] |

**Release gate (not a slice):** terms of use and privacy policy before public launch (§12, §13-4) — owned by the user with qualified professionals. Development is not blocked by it; launch is. **[Blocker for launch only]**

---

## 2. Cross-cutting acceptance criteria (apply to every slice they touch)

These are checked in QA for every slice that touches the relevant surface, instead of being repeated in each slice.

- **X-1 Platform neutrality.** No Core behavior depends on a specific platform; platform specifics live only in adapters (TradingView Input, NinjaTrader plugin). Verified by Architect review on each Core slice. (§4, §5)
- **X-2 Per-user isolation.** A user can never see, change, route to, or receive data about another user's accounts, keys, instructions, logs, or trades — in the Web, in the Core API, and in the plugin. Each slice touching data includes at least one negative test with two users. (§2, §11)
- **X-3 Exact numbers.** Prices, P&L, money, and limits are stored and computed without floating-point rounding errors; displayed values match stored values. (§11)
- **X-4 Secrets.** Source keys, plugin credentials, and passwords never appear in plain text in storage, logs, audit views, or error messages. (§11)
- **X-5 Transparency.** Every automatic action (route, skip, partial skip, flatten, block, reject, refuse) produces an audit entry with its reason, visible to the user (see V1-S10). (§11)
- **X-6 Bilingual UI.** All user-visible text (Web and plugin) exists in pt-BR and en; no hard-coded single-language strings. (§2)
- **X-7 Simulation only.** All development and QA use simulation, demo, or playback accounts. (§11)
- **X-8 Time.** Every timestamp is stored with its time zone; trading-day boundaries follow the account's reset rule (§9).

---

## 3. Slice overview

| ID | Slice | Surfaces | Depends on | Status |
|---|---|---|---|---|
| V1-S1 | Webhook ingest: authenticated, validated, idempotent, logged | TradingView Input, Core | S0, ADR-0003 (JSON v1 + ids), adapter-sim | ready |
| V1-S2 | NinjaTrader plugin: connect to Core and execute a routed entry on a sim account | NT plugin, Core | S0, ADR-0004 (Core↔plugin protocol v1), NT8 spike, Windows+NT8 machine (Q15) | ready |
| **V1-M1** | **First end-to-end demo**: TradingView alert → Core → one NinjaTrader sim account, result back to Core | all | S1 + S2 | milestone |
| V1-S3 | User sign-up/login, per-user isolation, source keys and plugin pairing | Web, Core, NT plugin | S0 | ready |
| V1-S4 | Bilingual foundation (pt-BR/en) | Web, NT plugin | S0 | ready |
| V1-S5 | Account registration and lifecycle (enable / disable / pause) | Web, Core, NT plugin | S3 | ready |
| V1-S6 | Stop/target profiles by account size; individual mode | Web, Core, NT plugin | S5 | ready |
| V1-S7 | Routing configuration: source → accounts, signal-type filter, multiplier, max position | Web, Core | S5 | ready |
| V1-S8 | Full instruction actions (close, reverse, add) and shared stop/target mode | Core, NT plugin | S6, S7 | ready |
| V1-S9 | Orphan-order protection | NT plugin, Core | S6 | ready |
| V1-S10 | Audit log and activity view | Core, Web | S1 | ready |
| V1-S11 | Out-of-sync detection and disconnect handling | Core, NT plugin, Web | S2, S10 | ready |
| V1-S12 | End-to-end exactly-once, command expiry (TTL) | Core, NT plugin | S2, S11 | ready |
| V1-S13 | Risk rules per account and live risk state with 80% warnings | Web, Core, NT plugin | S5, S11 | ready |
| V1-S14 | Core risk gate (refuse routing at limit; master-limit behavior) | Core | S13 | ready |
| V1-S15 | Automatic enforcement in the plugin (flatten, cancel, block until reset) | NT plugin, Core, Web | S13 | ready |
| V1-S16 | Prop firm presets: model, versioning, selection, editing | Web, Core | S5 | ready |
| V1-R1 | Preset research — priority firms (Apex, Phidias, Lucid) | research | none | ready |
| V1-R2 | Preset research — Topstep, Take Profit Trader, MyFundedFutures, Tradeify, Bulenox, TradeDay, Top One Futures, Alpha Futures | research | R1 format | ready |
| V1-R3 | Preset research — Elite Trader Funding, Earn2Trade, Funded Futures Family, Phoenix Trader Funding, OneUp Trader, BluSky, Leeloo Trading | research | R1 format | ready |
| V1-S17 | TradingView alert message standard for all three signal types + daily routine guide | TradingView Input, docs | S1; Pine source shared by the user | ready |
| V1-S18 | Notification center (in-app, Web + plugin) | Web, NT plugin, Core | S10 | ready |
| V1-S19 | Master replication — entries and exits (same instance and cross-instance) | NT plugin, Core, Web | S8, S12, ADR-0003 + ADR-0004 accepted by user | blocked (criteria complete; waits for ADR-0003/0004 acceptance) |
| V1-S20 | Master replication — all order types, stop/target moves, cancellations | NT plugin, Core | S19 | blocked (criteria complete; waits for ADR-0003/0004 acceptance) |
| V1-S21 | Trade capture and trade list (automatic + manual) | NT plugin, Core, Web | S2, S5 | ready |
| V1-S22 | Performance metrics view | Web, Core | S21 | ready |

---

## 4. Sequencing and parallel work

```
Wave 0 (Architect, before code)
  ADR-0003 JSON instruction v1 + identifiers (with fixtures)
  ADR-0004 Core↔plugin protocol v1 (with fixtures) + adapter-sim (fake plugin)
  NT8 spike: AddOn loads our library, outbound connection, order + account events on Sim101
Wave 1 (start now, in parallel)
  S1 Webhook ingest ─┐
  S2 Plugin execute ─┴─► M1 FIRST E2E DEMO
  S3 Auth/isolation        S4 i18n foundation        R1 Preset research (priority firms)
Wave 2 (after M1 + S3)
  S5 Accounts ─► S6 Profiles ─► S9 Orphan protection
            └─► S7 Routing   ─► S8 Full actions
  S10 Audit log            S16 Preset model/UI        R2, R3 Preset research
Wave 3
  S11 Out-of-sync ─► S12 Exactly-once
  S13 Risk rules ─► S14 Core risk gate
              └─► S15 Plugin enforcement
  S17 Alert standard       S18 Notifications          S21 Trade capture ─► S22 Metrics
Wave 4
  S19 Master replication entries/exits ─► S20 Full replication
```

- **First usable increment for the user (personal use, sim):** M1 + S3–S10 + S13–S15 + S16 with R1 presets → TradingView signals routed to several NinjaTrader accounts with profiles, multipliers, limits, enforcement, and audit.
- **Parallelism:** Web, Core, and plugin work can proceed in parallel per slice once ADR-0003, ADR-0004, each slice's API contract (with fixtures), and the adapter-sim are available (Architect, 2026-10-23). Core is tested against the adapter-sim; the plugin against the Core or the sim. The Web is **not** needed before M1 (seed data). Research slices (R1–R3) need no code and can start immediately.
- **Bottleneck:** plugin verification requires a Windows machine with NinjaTrader 8 (sim/playback) — see Q15.
- Designer input is needed for S3, S5, S6, S7, S10, S13, S16, S18, S22 (Web screens) and for plugin panels in S13/S15.

---

## 5. Slice specifications

### V1-S1 — Webhook ingest (ready)

**Goal / value:** a TradingView alert reaches All Win safely; the user can see that it arrived, was accepted or rejected, and why. Foundation of every routed trade.
**Surfaces:** TradingView Input, Core. **Depends on:** S0; ADR-0003 (JSON instruction v1 + identifiers); adapter-sim for testing without the plugin.
For M1 the user, secret, and destination account are pre-provisioned (seed) until S3/S5 exist; the Web is not needed for M1 (Architect, approved with the plan).

- **AC-1** Given a valid instruction carrying the correct **per-source secret** **in the webhook URL** (A3 [Decision 2026-10-23]; the indicator and message body carry no secret), when it is posted to the user's webhook address, then the Core accepts it, stores it with its receipt time, and records an audit entry "received".
- **AC-2** Given a wrong, missing, or revoked secret, then the instruction is rejected, nothing is routed, and no information about whether the user or account exists is revealed in the response.
- **AC-3** Given a malformed instruction (missing required field, unknown `action`, unknown `signal_type`, non-positive or non-integer `quantity`, unsupported `instrument`, unsupported `schema_version`), then it is rejected with a reason recorded; nothing is routed.
- **AC-4** Given an instruction whose `message_id` was already received for that user, then it is not routed again; an audit entry "duplicate ignored" is recorded. (§8)
- **AC-5** Given the same `message_id` from two different users, then each is processed independently (ids are scoped per user). [Assumption — Architect to confirm]
- **AC-6** The secret is **masked in every log** — reverse proxy/access logs, Core logs, and audit views — even though it travels in the URL; QA checks each log for the raw secret (A3 [Decision 2026-10-23], X-4).
- **AC-7** Instrument support is a configuration, not a fixed rule: MNQ is accepted; adding NQ/ES/MES requires no change to the ingest rules (§4).

### V1-S2 — NinjaTrader plugin: connect and execute (ready)

**Goal / value:** the plugin running in the user's NinjaTrader 8 connects to All Win and executes what the Core routes to it, reporting results back.
**Surfaces:** NT plugin, Core. **Depends on:** S0; ADR-0004 (Core↔plugin protocol v1); NT8 spike result; a Windows machine with NT8 for verification (Q15).

- **AC-1** Given the plugin is installed and configured with the user's credentials, when NinjaTrader starts, then the plugin connects to the Core and the Core shows that instance as "connected", with the list of NinjaTrader accounts it exposes.
- **AC-2** Given a routed entry (buy/sell, quantity N) for a connected sim account, then the plugin submits the order on that account and reports back to the Core: accepted, filled (price, quantity, time), partially filled, or rejected (with the platform's reason).
- **AC-3** Given the account's stop/target distances (for M1: one fixed profile), when the entry fills, then a stop and a target are placed at those distances from the fill price, and their placement is reported back.
- **AC-4** Given the plugin loses connection, then the Core marks the instance "disconnected" within a bounded time (value set by Architect/SRE) and the plugin shows its own connection state in NinjaTrader.
- **AC-5** The plugin refuses to execute on any account not designated as a destination for this user (X-2).
- **AC-6** Development and QA use only Sim/playback accounts (X-7).

### V1-M1 — First end-to-end demo (milestone)

**Given** a test TradingView alert (or an equivalent HTTP post of the same message) with a valid key and a `buy 1 MNQ` instruction, **when** it is sent, **then** within seconds one NinjaTrader sim account receives a market buy of 1 MNQ with stop and target from the profile, the fill and bracket are reported back, and the user can see the full chain (received → routed → filled) in the audit trail (minimal view acceptable). Sending the same message again produces no second order.

### V1-S3 — Sign-up/login, isolation, source keys, plugin pairing (ready)

**Goal / value:** each trader has a private space; only they can send instructions to their accounts.
**Surfaces:** Web, Core, NT plugin.

- **AC-1** Sign-up is **invite-only** in V1 (Q6 [Decision 2026-10-23]): only a person with a valid, unused, unexpired invitation can create an account; without one, sign-up is refused with a clear message. Invited users can sign in and out. Who issues invitations: [Assumption] an administrator (the product owner-user) — Designer/Architect to confirm the minimal admin path.
- **AC-1b** **Two-factor authentication (TOTP, authenticator app) is optional** and strongly suggested in the UI; once enabled, sign-in requires the code; the user can disable it only after re-authenticating (A5 [Decision 2026-10-23]).
- **AC-2** Every Web view and Core request returns only the signed-in user's data; a two-user negative test exists for each data area (X-2).
- **AC-3** The user can generate, view once, and revoke a per-source secret; after revocation, instructions with the old secret are rejected (S1 AC-2). Secrets are stored only in non-reversible or encrypted form (X-4).
- **AC-4** The user can pair each NinjaTrader instance to their user (each instance has its own credential) and unpair it; an unpaired instance receives nothing; revoking one instance does not affect the others.
- **AC-5** Password reset and session expiry exist; failed sign-in attempts do not reveal whether an email is registered.

### V1-S4 — Bilingual foundation (ready)

- **AC-1** The user can switch the Web interface between pt-BR and en; the choice persists across sessions.
- **AC-2** Every Web text, error message, and email (if any) is shown in the chosen language; numbers, dates, and currency follow the chosen locale's formatting while stored values are unchanged (X-3).
- **AC-3** The plugin's user-visible texts are available in pt-BR and en and **follow the user's Web language** (Q11 [Decision 2026-10-23]); a change in the Web applies to the plugin on its next connection or sooner.
- **AC-4** A missing translation is detectable in QA (no silent fallback to the key name).

### V1-S5 — Account registration and lifecycle (ready)

**Goal / value:** the user registers every account they trade, with what the platform needs to route safely.

- **AC-1** The user can register a **prop firm account** (firm, plan, size, phase) or a **real brokerage account**, on platform NinjaTrader, and link it to an account exposed by one of their paired instances. (§9)
- **AC-2** Prop accounts use the firm's sizes. Default profiles exist for 25k, 50k, 100k, 150k, 200k, 300k (§8). **Real accounts may have any size** (positive amount); they start from the default profile of the nearest standard size and the user customizes it (Q12 [Decision 2026-10-23]).
- **AC-3** **Maximum position size** is mandatory: registration cannot be saved without it. For prop accounts it defaults to the preset's maximum and cannot exceed it (validation message in pt-BR/en); for real accounts the user sets it. (§8)
- **AC-4** The user can **enable, disable, or pause** each account at any time; a disabled or paused account receives no new instructions and the skip is audited with reason (X-5). **Pause** = temporary stop that keeps all configuration and is resumed by the user; **disable** = account off until re-enabled (Q12 [Decision 2026-10-23]). Both keep open positions untouched.
- **AC-5** Multiplier is a whole number ≥ 1; fractional or zero values are refused. (§8)
- **AC-6** Editing or deleting an account never affects other users' data (X-2); deleting an account with an open position is refused with an explanation.

### V1-S6 — Stop/target profiles; individual mode (ready)

- **AC-1** The user can define a default stop/target profile for each size 25k–300k, including sizes they do not hold, in points, ticks, or dollars per profile. (§8)
- **AC-2** A new account starts with its size's profile; the user can customize it per account without changing the default.
- **AC-3** **Individual mode (default):** each entry gets its own stop and target computed from its own fill price; a second entry does not move the first entry's orders.
- **AC-4** Dollar-based distances are converted to price using the instrument's tick value exactly (X-3); the computed price is shown to the user in the audit entry.
- **AC-5** Stop/target values from the instruction, if any, are ignored; the profile always wins (§6).

### V1-S7 — Routing configuration (ready)

- **AC-1** The user selects which of their accounts receive instructions from each source (e.g., their TradingView webhook).
- **AC-2** Per account, the user selects which signal types become orders (trigger, trade_plan, nq_trigger); an instruction of an unselected type is skipped for that account and audited. (§7)
- **AC-3** Executed quantity = instruction quantity × account multiplier. (§8)
- **AC-4** If the resulting position would exceed the account's maximum position size, only the quantity up to the maximum is executed; the excess is skipped, audited, and the user is notified. If the account is already at its maximum, nothing is executed for that account. (§8)
- **AC-5** One account's skip, rejection, or disconnection does not delay or block routing to other accounts. No artificial delay or randomization is added. (§8)

### V1-S8 — Full actions and shared mode (ready)

- **AC-1** `buy`/`sell` when flat opens a position; when already positioned in the same direction, adds to it (§6).
- **AC-2** `close` flattens the account's position in the instrument and cancels its stops/targets. `close` always flattens the whole position; any quantity in the instruction is ignored (Q1 [Decision 2026-10-23]).
- **AC-3** `reverse` closes the current position and opens the opposite one with the instruction quantity × multiplier (capped by AC-4 of S7); when flat, `reverse` simply opens the position in the instruction's direction (Q1 [Decision 2026-10-23]).
- **AC-4** An entry instruction opposite to the current position (e.g., `buy` while short) is **rejected for that account and the user is notified**; nothing is executed — the source must send `close` or `reverse` explicitly (Q1 [Decision 2026-10-23]). Other accounts are unaffected.
- **AC-5** **Shared mode** (per-account setting): after each fill the position has exactly one stop and one target, recalculated from the average price using the profile distances.
- **AC-6** `modify`/`cancel` from TradingView are **not supported in V1**: rejected with a clear reason; they are supported for master replication in S20 (Q1 [Decision 2026-10-23]).

### V1-S9 — Orphan-order protection (ready)

- **AC-1** Whenever a position becomes flat for any reason (stop or target hit, manual close in NinjaTrader, `close`/`reverse`, enforcement), all remaining stop and target orders of that position are cancelled immediately, in both individual and shared modes. (§8)
- **AC-2** In individual mode, when one entry's stop or target fills while other entries stay open, only that entry's opposite order is cancelled.
- **AC-3** Each cancellation is reported to the Core and audited; if a cancellation fails, the account is flagged out of sync (S11) and the user is alerted.

### V1-S10 — Audit log and activity view (ready)

- **AC-1** Every instruction received, every routing decision per account (routed, skipped, partially skipped, refused) with reason, and every execution result is recorded with time, source, and account. (§8, X-5)
- **AC-2** The user can list and filter their activity by date, account, source, signal type, and outcome, and follow one instruction from receipt to every account's result.
- **AC-3** Entries cannot be edited or deleted by the user. Retention period: decided by the user after Database Architect input on cost/feasibility (Q13 [Decision 2026-10-23 on the process]); **pending before release, not blocking development** — the design must allow retention to be configured.
- **AC-4** No secrets appear in any entry (X-4).

### V1-S11 — Out-of-sync detection and disconnects (ready)

- **AC-1** An account is flagged **out of sync** when an order it should have received is missed, rejected, or partially filled, or when its instance disconnects with an open position or working orders; the flag and reason are visible in the Web and in the plugin. (§8)
- **AC-2** When a destination disconnects mid-trade and reconnects, the user is alerted and the platform **waits for the user's decision**; it never reopens or resyncs positions automatically. (§8 [Decision])
- **AC-3** The user can acknowledge and clear the flag after resolving it; clearing is audited.
- **AC-4** Accounts flagged out of sync keep receiving or stop receiving new instructions → decided: an out-of-sync account **receives no new instructions until the user clears the flag**; each skip is audited (Q14 [Decision 2026-10-23]).

### V1-S12 — End-to-end exactly-once (ready)

- **AC-1** The same instruction is never executed twice on the same account, whether the duplicate comes from the source, from Core retries, from a plugin reconnection, or from a Web reload. (§8)
- **AC-2** After a plugin reconnection, instructions routed while it was disconnected are not executed automatically; they are reported as missed (S11 AC-2). [Decision 2026-10-23, plan approval; consistent with "alert and wait"]
- **AC-2b** **Command expiry (TTL):** a command that does not reach its destination account within the account's TTL is never executed late; it is marked expired, audited, and the account is flagged out of sync. TTL is editable per account; the default is **short — a few seconds** (Q16 [Decision 2026-10-23]). The exact default number is a configuration value the Architect proposes within that range and the user confirms before release; not blocking development.
- **AC-3** QA evidence includes forced duplicate sends and forced disconnect/reconnect during an order.

### V1-S13 — Risk rules and live risk state (ready)

- **AC-1** Per account, the user can set drawdown (amount and type: trailing intraday, end-of-day trailing, static), daily loss limit, daily profit target, and firm-specific rules such as consistency rules. (§9)
- **AC-2** The Core keeps each account's current risk state from execution results and plugin-reported P&L, and the user sees it per account (used vs limit).
- **AC-3** At 80% of each limit, a warning is shown in the Web and in the plugin. (§9)
- **AC-4** Trading day resets at the prop firm's reset time (from the preset) or, for real accounts, at the CME futures session close. (§9)
- **AC-5** **Daily profit target:** the user chooses per account between "notify only" (default) and "flatten and block like a loss limit" (enforcement on); with "notify only" the user is notified once when it is reached (Q3 [Decision 2026-10-23]).
- **AC-6** **Consistency rules** are tracked and warned (current value vs the firm's rule, 80% warning) but never enforced in V1 (Q4 [Decision 2026-10-23]).

### V1-S14 — Core risk gate (ready)

- **AC-1** The Core refuses to route a new instruction to an account that has reached a limit — **also when enforcement is off** for that account (refusing new entries never closes a trade); the refusal is audited and notified (§5; Q2 [Decision 2026-10-23]). Exits (`close`) are never refused by the gate. [Assumption — consistent with the intent; flag if the user disagrees]
- **AC-2** When a destination account hits its limit with enforcement on, it is removed from routing until the next trading day; other accounts continue. (§8)
- **AC-3** When the **master** hits its limit, only the master is flattened and blocked; followers keep open positions and receive no new instructions from that master until the next trading day. (§8)

### V1-S15 — Plugin enforcement (ready)

- **AC-1** Enforcement is off by default and enabled per account by the user. (§9)
- **AC-2** With enforcement on, the plugin retrieves the account's limits from the Core and monitors live P&L; on reaching a limit it flattens positions, cancels working orders, and blocks trading on that account until the reset. (§9)
- **AC-3** No early unlock: neither the Web nor the plugin offers a way to unblock before the reset. (§9)
- **AC-4** Enforcement works while the Core is unreachable, using the last limits received (cached in the plugin) — second barrier, §5. (Feasible per Architect, 2026-10-23.)
- **AC-6** **Block is reactive** (NinjaTrader offers no account lock): while an account is blocked, any order the user places manually on it is cancelled, and any resulting position is flattened, immediately after submission; each such action is audited and shown in the plugin. The user is told in the plugin that the account is blocked and why. Exact timing to be measured in the NT8 spike.
- **AC-7** In Playback the trading-day reset and daily P&L use the connection's (simulated) clock, not the computer's clock.
- **AC-5** Every flatten/cancel/block is audited with the limit and value that triggered it (X-5).

### V1-S16 — Prop firm presets: model and UI (ready)

- **AC-1** A preset records firm, plan, account size, phase, all rules from S13, maximum position size, trading-day reset time, the firm's stance on trade copying and automated trading, source URL(s), verification date, and version. (§9, §12)
- **AC-2** Selecting a preset fills the account's rules and max position; the user can edit the copy on their account without changing the preset.
- **AC-3** When a preset gets a new version, existing accounts are not changed silently; the user is told and chooses whether to apply. [Decision 2026-10-23, plan approval]
- **AC-4** The firm's stance on copying/automation is shown when the user registers an account with that firm and when they add it to routing. (§12)
- **AC-5** Only presets with status "verified" are selectable; unverified firms are not shown (or shown as unavailable) per the user's decision from the research report.

### V1-R1 / R2 / R3 — Prop firm preset research (ready)

**Goal:** produce verified preset data for the 18 firms. Pure research, no code; can start now. Needs an agent with web access.

- **AC-1** For each firm, every preset field (S16 AC-1) is taken from the firm's **official** rules/help pages, with the exact URL and the date consulted; no third-party summaries as the sole source. (§9)
- **AC-2** Each firm's stance on trade copying (same person, multiple accounts) and automated trading is quoted with its source.
- **AC-3** Any field that cannot be verified is marked "unverified" — never guessed — and the firm is listed in a report to the user, who decides to delay that preset or release without it (§9 rule). Report goes via the Maestro.
- **AC-4** Output is one file per firm under `docs/product/presets/research/` (owned by the researcher "Lupa"), following [preset-research-format.md](../presets/preset-research-format.md); reviewed by the PO before data entry.
- **AC-5** Re-verification is planned shortly before release, because firms change rules often; changes found are reported.

### V1-S17 — TradingView alert standard and daily routine (ready)

Q7 [Decision 2026-10-23]: the user shares the Pine indicator source and the team proposes the change (stored under `integrations/tradingview/`, ADR-0002); the user reviews and applies it in TradingView. Starting this slice requires the user to hand over the source.


- **AC-1** Message templates exist for all three signal types (trigger, trade_plan, nq_trigger) in the standard JSON, each with `signal_type` and a unique id. (§7)
- **AC-2** A pt-BR/en guide describes the daily routine: paste levels, delete yesterday's alert, create one alert per chart ("any alert() function call"), and recreate after level or indicator changes.
- **AC-3** A test alert of each signal type reaches the Core and is accepted (S1).

### V1-S18 — Notification center (ready)

- **AC-1** Skipped excess, refusals, 80% warnings, blocks, out-of-sync flags, and disconnects create notifications visible in the Web and in the plugin.
- **AC-2** The user can mark notifications as read; unread count is visible.
- **AC-3** V1 has **in-app notifications only** (Web + plugin); no email or messaging-app channels (Q8 [Decision 2026-10-23]). Account e-mails needed by S3 (invite, password reset) are not notifications.

### V1-S19 — Master replication: entries and exits (blocked — waits for ADR-0003/0004 acceptance; V1.1)

- **AC-1** The user designates one NinjaTrader account as master and selects follower accounts (NinjaTrader, same or other instances of the same user). (§8)
- **AC-2** Entries and exits on the master are replicated to each follower with its multiplier and maximum position cap; each follower's **initial stop and target come from the follower's own profile** (Q5 [Decision 2026-10-23]).
- **AC-3** Works across instances through the Core; all involved instances must be connected, otherwise the follower is flagged out of sync (S11). In V1, replication **always relays through the Core**, also when master and followers share one instance; a local fast path is not built in V1 and may be revisited with measured latency (A4 [Decision 2026-10-23]).
- **AC-4** No artificial delay; latency per follower is recorded in the audit trail.

### V1-S20 — Master replication: everything else (blocked — waits for ADR-0003/0004 acceptance; V1.1)

- **AC-1** All order types placed on the master, their modifications, and cancellations are replicated. (§8)
- **AC-2** When the master's stop or target moves by X, each follower's moves by the same X. (§8)
- **AC-3** A master cancellation cancels the linked follower orders; failures flag out of sync.

### V1-S21 — Trade capture and trade list (ready)

- **AC-1** Each completed trade on a connected account is captured with: account, instrument, direction, quantity, entry/exit time and price, time in trade, P&L, MAE, MFE, and originating signal/source. (§10)
- **AC-2** Metrics are computed by the plugin from the user's own data; only computed numbers are sent; no raw price data is stored (§10).
- **AC-3** The user can enter a trade manually with the same fields (source = "manual").
- **AC-4** The user can list and filter trades by account, date, source, and signal type.

### V1-S22 — Performance metrics view (ready)

- **AC-1** For a selected period and set of accounts, the user sees totals and per-trade averages of P&L, win rate, time in trade, MAE, and MFE, split by source and signal type. [Assumption — exact metric set refined with the Designer; changes beyond this list go to the user]
- **AC-2** Numbers in the view reconcile exactly with the trade list (X-3).

---

## 6. Risks

| Risk | Impact | Mitigation |
|---|---|---|
| NinjaTrader plugin stability (connection, order state, multi-instance) | Wrong or missing executions | Thinnest E2E first (M1); out-of-sync detection (S11) and exactly-once (S12) before any real use; sim-only development (X-7) |
| TradingView webhook delivery is best-effort (no guaranteed retry) | Missed signals | Audit shows received signals; user can compare with TradingView alert log; documented in S17 guide |
| Plugin verification needs Windows + NT8 | Critical-path bottleneck | Q15; adapter-sim lets Core proceed without it |
| Unique id generation inside Pine alerts | Duplicates or false "duplicate ignored" | Architect defines id rule in JSON contract v1; S17 AC-3 tests it |
| Prop firm rule research volume (18 firms) and frequent rule changes | Release delay or wrong limits | Start R1 now; per-firm "verified" status; report-to-user rule; re-verification before release |
| Firms that forbid copying or automation | User account violations | Stance shown at registration and routing (S16 AC-4); not legal advice (§12) |
| Core accidentally becomes NinjaTrader-specific | Rework for Tradovate in release 2 | X-1 review on every Core slice |
| Risk state in Core diverges from platform's real P&L | Wrong blocks or missed limits | Plugin is the second barrier (§5); S15 AC-4 |
| Terms of use / privacy policy not ready | Launch blocked | Flagged now as user-owned release gate |

---

## 7. Questions for the user — decided 2026-10-23

Each has options and a recommendation. **All decided on 2026-10-23: the user accepted every recommendation below** ([decision log](../../delivery/decisions.md)). Conditions: **Q15** — before the user provides the Windows + NT8 machine, the Architect defines in the NT8 spike how agents will build and test the plugin there (remote agent tooling may not work on Windows) and presents options. Remaining non-blocking items: Q13 exact retention (after DBA input), Q16 exact default seconds.

| # | Question | Options | Recommendation |
|---|---|---|---|
| Q1 | What happens when an instruction conflicts with the current position — e.g., `buy` while short, `reverse` while flat, `close` with a quantity? And are `modify`/`cancel` needed from TradingView in V1? | (a) Reject and notify, the source must send `close`/`reverse` explicitly; (b) execute as a netting order (a buy reduces the short); (c) treat as reverse | **(a)** — matches "the instruction states exactly what must happen"; `reverse` while flat opens the position; `close` always flattens the whole position; no `modify`/`cancel` from TradingView in V1 |
| Q2 | With enforcement **off**, should the Core still refuse new instructions to an account that reached a limit? | (a) Yes — refusing new entries never closes a trade, it is a cheap safety net; (b) No — enforcement off means warnings only | **(a)**, with a clear notification |
| Q3 | When the **daily profit target** is reached? | (a) Notify only; (b) flatten and block like a loss limit; (c) per-account choice | **(c)**, default (a) |
| Q4 | Consistency rules in V1? | (a) Track and warn only; (b) also enforce | **(a)** — rules vary by firm and are complex; enforcement later |
| Q5 | Master replication: followers' **initial** stop/target? | (a) From each follower's profile, then later master moves applied as the same delta; (b) copy the master's distances | **(a)** — consistent with "stop and target come from the target account's profile" plus "followers move theirs by the same amount" |
| Q6 | Who can sign up during V1? | (a) Invite-only; (b) open sign-up | **(a)** until terms/privacy are ready |
| Q7 | Who changes the Pine indicator so all alerts emit the standard JSON? | (a) Team writes templates + guide, user applies to the indicator; (b) user shares the Pine source and the team proposes the change | **(b)** if the user agrees to share; otherwise (a) |
| Q8 | Notification channels in V1? | (a) In-app only (Web + plugin); (b) plus email; (c) plus a messaging app | **(a)** for V1 |
| Q9 | Must master replication be in the first release that the user starts using, or can it follow shortly after the TradingView path? | (a) Ship in one release; (b) V1.0 TradingView path, V1.1 master replication | **(b)** — real use starts earlier; both remain in release 1 |
| Q10 | If a firm forbids copying or automation, ship its preset anyway? | (a) Ship with the stance clearly shown; (b) exclude the firm | **(a)** — the user decides per account |
| Q11 | Plugin language? | (a) Follows the user's Web language; (b) own setting in the plugin | **(a)** |
| Q12 | Account sizes outside 25k–300k for real accounts, and the meaning of "pause" vs "disable"? | (a) Real accounts may have any size, using the nearest/custom profile; pause = temporary, disable = until re-enabled; (b) fixed sizes only | **(a)** |
| Q13 | Audit log retention? | (a) Keep for the life of the user account; (b) fixed period (e.g., 2 years) | Ask Database Architect for cost/feasibility, then user decides |
| Q14 | Does an out-of-sync account keep receiving new instructions until the user clears it? | (a) No, stops until cleared; (b) Yes, keeps receiving | **(a)** — never silently diverge |
| Q15 | Is there a Windows machine with NinjaTrader 8 (sim/playback) the team can use to verify the plugin? Who operates it? | (a) User provides a machine/VM with remote access for agents; (b) user runs the checks manually from a script the team provides | **(a)** — plugin verification is the critical path; (b) slows every plugin slice |
| Q16 | Command expiry (TTL): if an instruction cannot reach an account within N seconds (e.g., disconnection), it is never executed late. What default N? | (a) Short (a few seconds), editable per account; (b) longer (e.g., 30–60 s), editable per account | User decides the value; PO recommends **(a)** for entries — a late entry at a different price is worse than a missed one (to be reported as missed, S11) |

### Follow-up questions from design (S3/S4) — open, 2026-10-23

Raised by the Product Designer while specifying S3/S4. They do **not** block starting S3/S4 implementation (auth basics, i18n), but must be answered before the S3/S4 screen specs close.

| # | Question | Options | Recommendation |
|---|---|---|---|
| Q17 | How many TradingView sources (each with its own secret URL) per user? | (a) Several named sources (e.g., one per chart/alert); (b) one per user in V1 | **(a)** — the routine is "one alert per chart", and per-source routing (S7) and revocation then stay independent |
| Q18 | Unpairing a NinjaTrader instance whose accounts are routing destinations, possibly with an open position | (a) Allow after a confirmation listing affected accounts; with an open position, a stronger warning that the position stays open in NinjaTrader without platform monitoring/enforcement, and the accounts are flagged out of sync; (b) block unpairing while a position is open | **(a)** — the user stays in control; nothing is closed automatically |
| Q19 | Who issues invitations (Q6) and how? | (a) A platform-admin role (held by the user), single-use link bound to one e-mail, valid 7 days, delivered manually (copy link); (b) any user can invite | **(a)** |
| Q20 | Transactional e-mail in V1 for password reset (and invitations)? Q8 excluded e-mail *notifications* only | (a) Yes, transactional e-mail only for invite and password reset; (b) no e-mail: the admin generates a one-time reset link | **(a)** — self-service reset; (b) as fallback if e-mail is not wanted |
| Q21 | Session length / "keep me signed in" | Architect proposes values (idle and absolute timeout); user approves | Architect proposal → user |
| Q22 | Optional TOTP recovery | (a) 10 single-use recovery codes shown once at activation; enabling/disabling requires password + current code; lost device without codes → admin-assisted reset after identity check; (b) no recovery codes, admin reset only | **(a)** |
| Q23 | Locale defaults: first-visit language, display time zone, time format | (a) Language from the browser (pt or en, else pt-BR); time zone from the profile (default = browser), always shown next to reset/session times; 24-hour clock in both languages; (b) always pt-BR, fixed America/Chicago | **(a)** |

## 8. Architect inputs requested

Pending answers from Prumo @ Dev3 are recorded in §9 when received.

1. JSON contract v1 (fields, id scope, key placement) — needed by S1, S17.
2. Core↔plugin protocol — needed by S2.
3. Identifiers for modify/cancel and master replication (vision §13-2) — needed by S19/S20.
4. Feasibility of S15 AC-4 (enforcement with last-known limits while disconnected).

## 9. Architect feedback log

**2026-10-23 — User decisions on architecture items (rev. 3):** A1 stack per ADR-0001; A2 production in the US central region (near Chicago) — no product AC impact; A3 secret in the URL, masked in every log (→ S1 AC-1, AC-6); A4 replication always via Core in V1 (→ S19 AC-3); A5 optional TOTP (→ S3 AC-1b); A6 private GitHub + CI (push only with authorization).

**2026-10-23 — Prumo @ Dev3** (incorporated in rev. 2):
- M1 path is viable as walking skeleton. Order: ADR-0003 + ADR-0004 with fixtures → short NT8 spike → Core (against adapter-sim) and plugin in parallel. Web not needed before M1 (seed data).
- Plugin (AddOn) sees all accounts of the instance, receives executions/positions/P&L, can cancel and flatten in Sim and Playback. Limits: block is reactive (→ S15 AC-6); Playback uses simulated clock (→ S15 AC-7); enforcement must work with Core offline (→ S15 AC-4); one connection + credential per instance (→ S3 AC-4).
- ADR-0003 comes right after S0 approval; schema 1.0 reserves correlation fields so basic actions do not break later. Preliminary ids: `message_id` (dedupe), `position_ref`/`order_ref` (source), `instruction_id`/`command_id` per destination (Core), `platform_order_id` (execution report). Master replication blocked until ADR-0003 and ADR-0004 are accepted by the user.
- Architect will recommend the webhook secret in the URL rather than the body (Pine inputs can leak via status line/shared layouts) — user decision; the plan says "per-source secret" without fixing position.
- New product questions: Windows+NT8 machine (Q15), command TTL (Q16).
