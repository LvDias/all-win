# Release 1 (V1) — Slice Plan — All Win

> **Owner:** Product Owner (Bússola @ Dev2) · **Status:** approved by the user on 2026-10-23 ([decision log](../../delivery/decisions.md)) · **Revision:** 9 (2026-10-23f: evaluation target details, commission floor U20; rev. 8 = user decisions 2026-10-23e: U16, M2, M3, Q28–Q68 accepted with two exceptions, S13b added; rev. 7 = passwordless sign-in replaces password + TOTP in S3; new slice S3b; Q28–Q36 open; rev. 6 = R2 scope decisions: manual-only firms, VPS rules; [Decision 2026-10-23b/c/d] = second/third/fourth batch of user decisions) · **Date:** 2026-10-23
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
- **X-4 Secrets.** Source keys, plugin credentials, and one-time sign-in codes never appear in plain text in storage, logs, audit views, or error messages. (§11)
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
| V1-S3 | Passwordless sign-up/login by e-mail code, per-user isolation, source keys and plugin pairing | Web, Core, NT plugin | S0 | ready |
| V1-S3b | WhatsApp as a second code channel (sign-up, login, step-up, channel change) | Web, Core | S3, Meta business verification (user, started 2026-10-23) | blocked |
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
| V1-S13b | Evaluation progress: profit target left to pass | Web, Core | S13, S16, S21 | ready |
| V1-S14 | Core risk gate (refuse routing at limit; master-limit behavior) | Core | S13 | ready |
| V1-S15 | Automatic enforcement in the plugin (flatten, cancel, block until reset) | NT plugin, Core, Web | S13 | ready |
| V1-S16 | Prop firm presets: model, versioning, selection, editing | Web, Core | S5 (free editing of copied risk rules arrives with S13) | ready |
| V1-R1 | Preset research — priority firms (Apex, Phidias, Lucid) | research | none | ready |
| V1-R2 | Preset research — Topstep, Take Profit Trader, MyFundedFutures, Tradeify, Bulenox, TradeDay, Top One Futures, Alpha Futures | research | R1 format | ready |
| V1-R3 | Preset research — Elite Trader Funding, Earn2Trade, Funded Futures Family, Phoenix Trader Funding, OneUp Trader, BluSky, Leeloo Trading | research | R1 format | ready |
| V1-S17 | TradingView alert message standard for all three signal types + daily routine guide | TradingView Input, docs | S1; Pine source shared by the user | ready |
| V1-S18 | Notification center (in-app, Web + plugin) | Web, NT plugin, Core | S10 | ready |
| V1-S19 | Master replication — entries and exits (same instance and cross-instance) | NT plugin, Core, Web | S8, S12, ADR-0003 + ADR-0004 (accepted 2026-10-23), 2 Windows machines/VMs for cross-instance QA | ready (V1.1; cross-instance QA needs W1–W3) |
| V1-S20 | Master replication — all order types, stop/target moves, cancellations | NT plugin, Core | S19 | ready (V1.1) |
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

### V1-S3 — Passwordless sign-up/login, isolation, source keys, plugin pairing (ready)

**Goal / value:** each trader has a private space; only they can send instructions to their accounts; access is proven by a one-time code sent to a channel the user owns. **There is no password.**
**Surfaces:** Web, Core, NT plugin.
**Change record:** rev. 7 replaced the password + optional TOTP model (Q20 reset part, Q22, A5) by passwordless sign-in **[Decision 2026-10-23, user requirement change]**; rev. 8 records the user's acceptance of U16, M2, M3 and Q28–Q36 as recommended **[Decision 2026-10-23e]**. In this slice the only channel is **e-mail**; WhatsApp is slice **V1-S3b**. Mechanics (limits, hold, step-up table) are in [auth-and-session-policy.md](../../architecture/security/auth-and-session-policy.md); this slice states the observable outcomes.

**Sign-up**

- **AC-1** Sign-up is **invite-only** (Q6). Only a person with a valid, unused, unexpired invitation can start sign-up; otherwise it is refused with a clear message. Invitations are issued only by the admin, single-use, bound to one e-mail, valid 7 days, delivered manually (Q19, unchanged).
- **AC-2** The invitee enters their e-mail, which must equal the invited e-mail (Q36 a), and confirms it with a code sent to it (AC-5). The account is created, and the invitation consumed, only after the code is confirmed. A failed or abandoned confirmation leaves the invitation usable until it expires.
- **AC-3** No password is ever requested, stored or reset. The WhatsApp number is not part of S3 sign-up: WhatsApp is optional and added later in the profile (S3b, Q31 a).

**Sign-in and codes**

- **AC-4** Sign-in: the user enters their e-mail, receives a code at that e-mail and enters it to be signed in. (When S3b ships, the user chooses e-mail or WhatsApp.) Sign out ends the current session.
- **AC-5** Code rules: single-use; expires after 10 minutes; invalidated after 5 wrong attempts (the user must request a new one); the code is never stored, logged or audited in plain text (X-4). **Format (U16, decided 2026-10-23e):** 8 characters of the Crockford Base32 alphabet shown as `XXXX-XXXX`; entry ignores case, spaces and hyphens and reads the look-alike letters as Crockford decoding does. A new code request for another purpose does **not** invalidate other open codes (a resend invalidates only its own challenge); this replaces the earlier assumption, per the policy, so nobody can cancel the code a user is about to type.
- **AC-6** Sending limits apply **per user and per IP** (and per identifier, with separate budgets for known and unknown devices) to every code request (sign-up, sign-in, step-up, channel change). When a limit is hit the request is refused with a stable error code and the user is told to wait; no code is sent. Numeric values: Architect's proposal accepted (Q33 a), see policy §4.
- **AC-7** Failed or unknown-account sign-in attempts do not reveal whether an e-mail is registered: the visible response and timing are the same, and no code is sent to an unregistered e-mail. Wrong, expired, used or invalidated codes all give the same message.
- **AC-8** Sessions: 12 h idle / 24 h absolute; "keep me signed in" 7 d idle / 30 d absolute (Q21, carried over; Q29 a). Expiry returns to sign-in with a clear message.

**New device and sensitive actions**

- **AC-9** **New-device notice:** when a user signs in from a device not previously used with that account, an e-mail is sent to the account e-mail, in the user's language and time zone (S4 AC-2), with: date and time, browser and OS family, the channel used, the IP address (no geolocation in V1), and a "this was not me" guide (sign in, end all sessions, review source tokens and NinjaTrader pairings, check the e-mail account's security). No one-click action link. It is an account e-mail, not a notification (S18 AC-3). The sign-up session itself sends no notice. [Decision 2026-10-23e, Q34 a]
- **AC-10** **Step-up:** each of these actions requires a **fresh code at the time of the action**, with no reuse window (Q30 a): turning automation on for an account or accepting a copy opt-in (S5 AC-7, AC-9b); unpairing a NinjaTrader instance (AC-15); changing the e-mail or WhatsApp number (AC-11); and, by the user's acceptance of Q35 (a server setting, on): creating or rotating a source token, creating a connection, creating a pairing code, issuing an invitation (admin), enabling a route (S7), and changing an account's platform link (S5 AC-12). Without a valid code the action is not performed and the attempt is audited. **M3:** when the session's device was first seen less than 7 days ago, the persistence actions (source token create/rotate, pairing code, opt-in accept) require the code **by e-mail**; once S3b ships they require codes on **both** channels. [Decision 2026-10-23e]
- **AC-11** **Changing the e-mail (or WhatsApp number, S3b):** the user enters the new value and must confirm a code on the **old** channel and a code on the **new** channel; once S3b ships, the change also needs the code of the other confirmed channel. The old value is never replaced without its own code. **M2:** a change requested from a device first seen less than 7 days ago takes a **72-hour hold** with notices to every old channel and a "Cancelar troca" action from any session; after any channel change other sessions end and further changes are frozen for 24 h. A pending change expires after 10 minutes if not confirmed. Each channel value belongs to **one account only** (Q32 a); a clash is refused without revealing who owns it. [Decision 2026-10-23e, Q28, Q32, M2]
- **AC-12** **Loss of every channel (Q28 a):** admin-assisted recovery. The admin verifies the user's identity out of band, sets a new e-mail, confirmed by a code sent to it, with an admin step-up. The change takes effect only after a **72-hour hold** with notices to every old channel and the same "Cancelar troca" action (merely signing in does not cancel). When it takes effect all sessions end and all devices are revoked; source tokens and pairings stay and the user is pointed to review them. A cancelled request followed by a new one raises an admin alert. Everything is audited; an admin cannot use this flow on themselves (the operator procedure is defined in the policy §9). [Decision 2026-10-23e]
- **AC-17** **Sign out of all devices:** the user can see their active sessions/devices, forget a device, and end every session at once (current one included); it needs no step-up. Its confirmation lists the source tokens and paired instances and links to their revoke/unpair actions. [Decision 2026-10-23e, Q34 a]

**Isolation, sources, pairing (unchanged by this change)**

- **AC-13** Every Web view and Core request returns only the signed-in user's data; a two-user negative test exists for each data area, including code and session endpoints (X-2).
- **AC-14** The user can generate, view once, and revoke a per-source secret; after revocation, instructions with the old secret are rejected (S1 AC-2). Secrets are stored only in non-reversible or encrypted form (X-4).
- **AC-15** The user can pair each NinjaTrader instance to their user (each instance has its own credential) and unpair it; an unpaired instance receives nothing; revoking one instance does not affect the others. Unpairing an instance whose accounts are destinations is allowed after a confirmation listing them (and the AC-10 code); those accounts become out of sync and nothing is closed (Q18). **In the plugin** the button only **disconnects locally** (warning with the consequences and a checkbox when a position is open or enforcement is on); the pairing stays registered in the Core as "disconnected", with the out-of-sync rules, and **revoking** it is done in the Web with the AC-10 code (Q66 a, [Decision 2026-10-23e]).
- **AC-16** Transparency: sign-ins, failed or rate-limited attempts, new-device notices, delivery failures, step-up successes and failures, channel changes, "forget device" and "sign out of all devices" appear in the user's audit log with their reason (S10). No audit row holds a code or an unmasked channel value.

**Superseded by rev. 7 (do not implement):** password and password policy (D10), password reset (Q20 reset part, D9), optional TOTP and its suggestion banner (A5, D13), TOTP recovery codes and admin-assisted TOTP reset (Q22, D12), former AC-1b and AC-5 text. Still valid: Q6, Q19, Q21 (confirmed by Q29 a), the transactional e-mail now also carries sign-in codes and new-device notices (Q8 still excludes e-mail *notifications*).

**Dependencies / risk:** sign-in depends on e-mail delivery being available; if it is down nobody can sign in (SRE/availability input requested through the Maestro; see §6). With e-mail as the only channel an e-mail takeover is an account takeover; the mitigations are the new-device notice, the step-up on persistence actions, the holds, and the user's own e-mail security (policy §10).

### V1-S3b — WhatsApp as a second code channel (blocked)

**Goal / value:** the user can receive codes on WhatsApp as well as e-mail.
**Surfaces:** Web, Core. **Depends on:** S3; the **Meta business verification**, which the user started on 2026-10-23 (U17: Meta WhatsApp Cloud API direct, as recommended; lead time of weeks, outside the team's control). **Status:** blocked until the verified WhatsApp sender and its approved authentication template exist; the criteria are complete. **Supplier research (input to U17, kept for reference; unconfirmed items marked in the file):** [whatsapp-otp-providers.md](../research/whatsapp-otp-providers.md).

- **AC-1** A user can add a WhatsApp number (international format, country code required) and confirm it with a code sent to that number. Unconfirmed numbers are never used.
- **AC-2** Sign-in offers a choice of channel (e-mail or WhatsApp) among the user's confirmed channels; the code rules, limits and wrong-attempt rules of S3 AC-5/AC-6 apply identically to both.
- **AC-3** Step-up (S3 AC-10) may use any confirmed channel, except where S3 AC-10 (M3) asks for both. Changing the WhatsApp number follows S3 AC-11 (code to old and new number plus the e-mail code).
- **AC-4** The new-device notice is still sent by e-mail even if the sign-in used WhatsApp.
- **AC-5** If WhatsApp delivery fails, the user is told and can request a code by e-mail; the Core never silently sends the same code through another channel.
- **AC-6** WhatsApp is **optional** (Q31 a): sign-up and sign-in keep working with e-mail only before and after S3b; after S3b the profile shows a reminder, never a requirement.
- **AC-7** Every WhatsApp send is audited (without the code); the admin can see the number of WhatsApp messages sent per period so the cost can be followed.

### V1-S4 — Bilingual foundation (ready)

- **AC-1** The user can switch the Web interface between pt-BR and en; the choice persists across sessions. First visit: browser language (pt or en), else pt-BR; display time zone from the profile (default = browser), shown next to reset/session times; 24-hour clock in both languages (Q23 [Decision 2026-10-23b]).
- **AC-2** Every Web text, error message, and email (if any) is shown in the chosen language; numbers, dates, and currency follow the chosen locale's formatting while stored values are unchanged (X-3).
- **AC-3** The plugin's user-visible texts are available in pt-BR and en and **follow the user's Web language** (Q11 [Decision 2026-10-23]); a change in the Web applies to the plugin on its next connection or sooner.
- **AC-4** A missing translation is detectable in QA (no silent fallback to the key name).

### V1-S5 — Account registration and lifecycle (ready)

**Goal / value:** the user registers every account they trade, with what the platform needs to route safely.

- **AC-1** The user can register a **prop firm account** (firm, plan, size, phase, and the firm's **platform/connection**, e.g. Rithmic or Tradovate, when the preset has platform-specific rules — the choice selects those rules and is stored on the account) or a **real brokerage account**, on platform NinjaTrader, and link it to an account exposed by one of their paired instances. (§9; [Decision 2026-10-23b])
- **AC-2** Prop accounts use the firm's sizes. The user defines default profiles for 25k, 50k, 100k, 150k, 200k, 300k (S6; the system ships no invented values, §8). **Real accounts may have any size** (positive amount); they start from the default profile of the nearest standard size and the user customizes it (Q12 [Decision 2026-10-23]).
- **AC-3** **Maximum position size** is mandatory and is **one number per account** in V1 (MNQ; Q-A1 [Decision 2026-10-23b]): registration cannot be saved without it. For prop accounts it defaults to the preset's maximum and cannot exceed it (validation message in pt-BR/en); for real accounts the user sets it. (§8)
- **AC-4** The user can **enable, disable, or pause** each account at any time; a disabled or paused account receives no new instructions and the skip is audited with reason (X-5). **Pause** = temporary stop that keeps all configuration and is resumed by the user; **disable** = account off until re-enabled (Q12 [Decision 2026-10-23]). Both keep open positions untouched.
- **AC-5** Multiplier is a whole number ≥ 1; fractional or zero values are refused. (§8)
- **AC-6** Editing or deleting an account never affects other users' data (X-2); deleting an account with an open position is refused with an explanation.
- **AC-7** **Automation opt-in (option A, [Decision 2026-10-23b]):** when registering or editing an account of a firm whose preset says automation is forbidden, the user must explicitly choose "accepts automated sources" or "does not accept" — **no default**; the account cannot be saved without a choice. Before choosing, the user sees a warning with the firm's rule text (verbatim, with source URL and access date) and the risk of losing the account.
- **AC-8** The choice is recorded in the audit with date, user, choice, and the version of the text shown (preset version, source URL, access date). It can be changed at any time: switching to "accepts" shows the warning again and requires recent re-authentication (step-up, auth policy); switching to "does not accept" needs no warning; both are audited; open positions are not touched.
- **AC-9** "Automated sources" = the TradingView webhook and the future signal engine. Master replication is governed by the firm's **copying** stance, not by this opt-in (Q24 [Decision 2026-10-23c]).
- **AC-9b** **Copy opt-in:** a follower account (S19) of a firm whose copying stance is forbidden, conditional, unknown, or not confirmed requires a separate explicit choice "accepts master replication" / "does not accept", with the same rules as AC-7/AC-8 (no default, verbatim rule text with source and date, audited, step-up to accept). A follower that does not accept cannot be subscribed to a master (Q24 [Decision 2026-10-23c]).
- **AC-10** The explicit choice (AC-7, AC-9b) is required for any stance other than "allowed": forbidden, conditional, unknown, or not confirmed. For "allowed" and for real accounts no choice is asked (Q25 [Decision 2026-10-23c]).
- **AC-11** If a new preset version changes the automation (or copying) stance or its text, the account's automated sources (or master replication) are **suspended** until the user confirms again, and the user is notified; open positions are not touched (Q26 [Decision 2026-10-23c]).
- **AC-12** **Platform link is optional** (Q37 a): an account can be saved without being linked to an account of a paired instance; an unlinked account is tracked manually only and never receives routing, automation or enforcement. Accounts of a manual-only firm (S16 AC-7) cannot be linked. [Decision 2026-10-23e]
- **AC-13** Changing the linked platform account is refused while the account has an open position, with the reason (Q38 a). [Decision 2026-10-23e]
- **AC-14** Deleting an account (refused with an open position, AC-6) **archives** it (Q39 a): it disappears from the lists; its routes and any master/follower relation are removed after a confirmation that lists them; its history (audit, trades) is kept (S10 AC-3); the removal is audited. [Decision 2026-10-23e]
- **AC-15** **Before the preset catalog (S16)** (Q40 a): a prop account's firm, plan and phase are free text and its maximum position is typed by the user; its stance is "unknown", so the explicit opt-in (AC-10) is required, with a generic warning that no rule text is available. [Decision 2026-10-23e]
- **AC-16** When S16 later attaches a preset to a prop account created before it, the account goes to **review pending**: automated sources are suspended until the user confirms the stance (AC-11) (Q41 a). [Decision 2026-10-23e]
- **AC-17** A **material change** of a stance text (AC-11) is a change that the person who publishes the preset version flags as material, with a recorded reason; unflagged changes never suspend (Q42 a). [Decision 2026-10-23e]

### V1-S6 — Stop/target profiles; individual mode (ready)

- **AC-1** The user can define a default stop/target profile for each size 25k–300k, including sizes they do not hold, in points, ticks, or dollars per profile. Profiles are **keyed by instrument** (only MNQ in V1), so new futures need no rework (Q-A2 [Decision 2026-10-23b]; §4).
- **AC-2** A new account starts with its size's profile; the user can customize it per account without changing the default.
- **AC-3** **Individual mode (default):** each entry gets its own stop and target computed from its own fill price; a second entry does not move the first entry's orders.
- **AC-4** Dollar-based distances are converted to whole ticks using the instrument's exact tick value (X-3). A distance that is not a whole number of ticks is **rounded toward the entry** for both stop and target (never more risk than configured); the adjustment and the resulting price are shown in the audit entry (Q-N1 [Decision 2026-10-23b]).
- **AC-5** Stop/target values from the instruction, if any, are ignored; the profile always wins (§6).
- **AC-6** An entry instruction that reaches an account whose size has **no stop/target profile** is **skipped and audited** with the reason "no stop/target profile" and notified (S18 AC-4); exits (`close`, reduce) still work (P1); the account screen shows a banner asking the user to define the profile (Q43 a). [Decision 2026-10-23e]
- **AC-7** "Nearest standard size" for a real account is measured by the difference in account amount; below 25k uses 25k and above 300k uses 300k. If the size is exactly halfway between two standard sizes, the user picks one of the two when saving the account (Q60 c). [Decision 2026-10-23e]

### V1-S7 — Routing configuration (ready)

- **AC-1** The user selects which of their accounts receive instructions from each source (e.g., their TradingView webhook).
- **AC-2** Per **(source, account)** pair, the user selects which signal types become orders (trigger, trade_plan, nq_trigger); an instruction of an unselected type is skipped for that account and audited. (§7; Q-A4 [Decision 2026-10-23b]) A user may have several named TradingView sources, each with its own secret URL (Q17 [Decision 2026-10-23b]).
- **AC-3** Executed quantity = instruction quantity × account multiplier. (§8)
- **AC-4** If the resulting position would exceed the account's maximum position size, only the quantity up to the maximum is executed; the excess is skipped, audited, and the user is notified. If the account is already at its maximum, nothing is executed for that account. (§8)
- **AC-5** One account's skip, rejection, or disconnection does not delay or block routing to other accounts. No artificial delay or randomization is added. (§8)
- **AC-6** An account that **does not accept automated sources** (S5 AC-7) cannot be subscribed to an automated source: the combination is shown as unavailable with the reason. If an instruction still reaches it, it is skipped and audited as "account does not accept automated sources" (option A [Decision 2026-10-23b]).
- **AC-7** When adding to routing an account of a firm whose automation or copying stance is not "allowed", the stance and its source are shown again (§12).
- **AC-7b** Accounts of a **manual-only** firm (S16 AC-7, e.g. Topstep) **do not appear** in routing or as master/follower choices at all; the routing screen explains why if the user looks for them [Decision 2026-10-23d].
- **AC-8** Gate order and exits: paused/disabled accounts block every action; **risk and out-of-sync gates never block exits** (`close`, reduce, protective modify/cancel) (P1 [Decision 2026-10-23b]).
- **AC-9** The on/off switch of a route whose account has become ineligible (declined opt-in, review pending): **turning off is always allowed; turning on is unavailable** while the account is ineligible, with the reason; every route starts again only by a new explicit action (Q67 b). Accounts without a platform link are not offered as destinations (S5 AC-12). [Decision 2026-10-23e]

### V1-S8 — Full actions and shared mode (ready)

- **AC-1** `buy`/`sell` when flat opens a position; when already positioned in the same direction, adds to it (§6).
- **AC-2** `close` flattens the account's position in the instrument and cancels its stops/targets. `close` always flattens the whole position; any quantity in the instruction is ignored (Q1 [Decision 2026-10-23]).
- **AC-3** `reverse` closes the current position and opens the opposite one with the instruction quantity × multiplier (capped by AC-4 of S7); when flat, `reverse` simply opens the position in the instruction's direction (Q1 [Decision 2026-10-23]).
- **AC-4** An entry instruction opposite to the current position (e.g., `buy` while short) is **rejected for that account and the user is notified**; nothing is executed — the source must send `close` or `reverse` explicitly (Q1 [Decision 2026-10-23]). Other accounts are unaffected.
- **AC-4b** A partial exit in individual mode reduces protective orders **oldest entry first (FIFO)** (P2 [Decision 2026-10-23b]).
- **AC-5** **Shared mode** (per-account setting): after each fill the position has exactly one stop and one target, recalculated from the average price using the profile distances.
- **AC-6** `modify`/`cancel` from TradingView are **not supported in V1**: rejected with a clear reason; they are supported for master replication in S20 (Q1 [Decision 2026-10-23]).

### V1-S9 — Orphan-order protection (ready)

- **AC-1** Whenever a position becomes flat for any reason (stop or target hit, manual close in NinjaTrader, `close`/`reverse`, enforcement), all remaining stop and target orders of that position are cancelled immediately, in both individual and shared modes. (§8)
- **AC-2** In individual mode, when one entry's stop or target fills while other entries stay open, only that entry's opposite order is cancelled.
- **AC-3** Each cancellation is reported to the Core and audited; if a cancellation fails, the account is flagged out of sync (S11) and the user is alerted.
- **AC-4** **Local OCO limit:** on connections where the stop/target pairing is simulated on the user's PC (NinjaTrader and Rithmic connections; "unknown" treated as local), orphan-order protection works only while NinjaTrader and the plugin are running. The account screen shows the connection's OCO location, and the Web (account registration/screen) and the plugin explain this limit in pt-BR/en. (Architect, ADR-0004 §6.1)

### V1-S10 — Audit log and activity view (ready)

- **AC-1** Every instruction received, every routing decision per account (routed, skipped, partially skipped, refused) with reason, and every execution result is recorded with time, source, and account. (§8, X-5)
- **AC-2** The user can list and filter their activity by date, account, source, signal type, and outcome, and follow one instruction from receipt to every account's result.
- **AC-3** Entries cannot be edited or deleted by the user. Retention: the activity record is kept **24 months rolling** (configurable); trades and configuration are kept for the life of the user account; all data is erased **30 days after the user closes the account** (privacy policy) (Q13, Q13-b [Decision 2026-10-23b]).
- **AC-4** No secrets appear in any entry (X-4).
- **AC-5** V1 has **no export** of the audit log; it is readable and filterable on screen (Q44 a). [Decision 2026-10-23e]
- **AC-6** The retention period is shown to the user as one read-only line; only the admin changes it, in system configuration (Q45 a). [Decision 2026-10-23e]

### V1-S11 — Out-of-sync detection and disconnects (ready)

- **AC-1** An account is flagged **out of sync** when an order it should have received is missed, rejected, or partially filled, or when its instance disconnects with an open position or working orders; the flag and reason are visible in the Web and in the plugin. (§8)
- **AC-2** When a destination disconnects mid-trade and reconnects, the user is alerted and the platform **waits for the user's decision**; it never reopens or resyncs positions automatically. (§8 [Decision])
- **AC-3** The user can acknowledge and clear the flag after resolving it; clearing is audited.
- **AC-4** Decided: an out-of-sync account **receives no new instructions until the user clears the flag**; each skip is audited (Q14 [Decision 2026-10-23]).

- **AC-5** **Reconnect check (local OCO):** after a reconnection, before any new command is accepted, the plugin reports each mapped account's position and working orders. Then:
  - (i) position flat with a stop or target still working (orphan leg) → those orders are **cancelled automatically** (orphan rule, vision §8) and the user is alerted with what was cancelled;
  - (ii) position open without stop or target, or orders in an unknown state (e.g., a simulated trigger lost on restart) → flagged out of sync, alert, **wait for the user**; nothing is recreated;
  - (iii) **both legs filled** while NinjaTrader was down (or the orphan leg filled during reconnection), leaving an unwanted, unprotected opposite position → **only the position created by the orphan leg is flattened automatically**, with a maximum-priority alert and audit; nothing else is changed (Q27 [Decision 2026-10-23c]);
  - (iv) broker connection lost while NinjaTrader stays open → same treatment when it returns; while the account cannot be observed, the Web shows "bracket integrity unknown".
  In every case the account stays out of sync until the user clears it (Q14), and everything is audited. (Architect, ADR-0004 §6.1)
- **AC-6** A Sim account reset or a Playback rewind is a state discontinuity: the account is flagged out of sync and the change is **not** treated as executions or P&L.
- **AC-7** An external order or fill on a destination account (placed outside All Win) flags **master-replication destinations** out of sync with an alert; for TradingView-only destinations it is audited only (T3 [Decision 2026-10-23b]).

### V1-S12 — End-to-end exactly-once (ready)

- **AC-1** The same instruction is never executed twice on the same account, whether the duplicate comes from the source, from Core retries, from a plugin reconnection, or from a Web reload. (§8)
- **AC-2** After a plugin reconnection, instructions routed while it was disconnected are not executed automatically; they are reported as missed (S11 AC-2). [Decision 2026-10-23, plan approval; consistent with "alert and wait"]
- **AC-2b** **Command expiry (TTL):** a command that does not reach its destination account within the account's TTL is never executed late; it is marked expired, audited, and the account is flagged out of sync. Only **entries** expire; exits are never dropped for lateness, and late exits are audited as late (T1 [Decision 2026-10-23b]). TTL default **5 s**, editable per account between 1 and 60 s (Q16, T2 [Decision 2026-10-23b]). Expiry counts from the earlier of the source's send time and the Core's receipt time (P4 [Decision 2026-10-23b]).
- **AC-3** QA evidence includes forced duplicate sends and forced disconnect/reconnect during an order.

### V1-S13 — Risk rules and live risk state (ready)

- **AC-1** Per account, the user can set drawdown (amount and type: trailing intraday, end-of-day trailing, static), daily loss limit, daily profit target, and firm-specific rules such as consistency rules. (§9)
- **AC-2** The Core keeps each account's current risk state from execution results and plugin-reported P&L (**net of commissions**: firms measure drawdown and limits on net, Q54 exception [Decision 2026-10-23e]; commission rule in AC-11), and the user sees it per account (used vs limit). **Only those two sources count** (clarification rev. 7): trades entered manually (S21 AC-3) never change the risk state, the limits, the blocks or the consistency figures; they appear only in the trade list and the metrics (S21, S22).
- **AC-3** At 80% of each limit, a warning is shown in the Web and in the plugin. (§9)
- **AC-4** Trading day resets at the prop firm's reset time (from the preset) or, for real accounts, at the CME futures session close. (§9)
- **AC-5** **Daily profit target:** the user chooses per account between "notify only" (default) and "flatten and block like a loss limit" (enforcement on); with "notify only" the user is notified once when it is reached (Q3 [Decision 2026-10-23]).
- **AC-6** **Consistency rules** are tracked and warned (current value vs the firm's rule, 80% warning) but never enforced in V1 (Q4 [Decision 2026-10-23]).
- **AC-7** A **prop** account cannot be saved without drawdown and daily loss limit, unless its preset states explicitly that the firm has no daily loss limit; every other rule is optional. On **real** accounts every rule is optional and unset rules are shown as "not set" (Q47 a). [Decision 2026-10-23e]
- **AC-8** A limit edit that **tightens** takes effect immediately; one that **loosens** applies from the next trading day; an edit never lifts a block already applied ("no early unlock"). If the new value is already exceeded, the Core stops routing new entries at once (S14 AC-1) and, with enforcement on, the plugin flattens at once, after a warning before saving that names this effect (Q48 b). [Decision 2026-10-23e]
- **AC-9** "Flatten and block at the daily profit target" (AC-5) is **unavailable until enforcement is on for the account**, with the reason shown (Q49 b). [Decision 2026-10-23e]
- **AC-10** The reset time of a prop account is editable on its copy of the preset, marked "edited"; a real account uses the CME session close and cannot edit it. [S16 AC-2]
- **AC-11** **Commission floor (U20 a):** each account has a commission per contract per side, **prefilled by the S16 preset when the firm publishes it** and editable by the user. For every fill the effective commission is the **greater of the platform-reported commission and the configured value × quantity**. When neither exists, the account shows the warning "comissão não informada" and net equals gross for that account. Gross and net P&L both stay available; limits, drawdown and the profit target use net (S15 AC-13). [Decision 2026-10-23f]

### V1-S13b — Evaluation progress: profit target left to pass (ready)

**Goal / value:** on an evaluation account the user sees how much profit is left to reach the firm's evaluation profit target. **Source:** user decision on Q46, 2026-10-23e/f (minimum trading days and payout rules come later). **Depends on:** S13 (risk state, commission rule), S16 (preset with the evaluation profit target, S16 AC-10), S21 (net P&L).

- **AC-1** For an account in an evaluation phase whose preset has a **confirmed** evaluation profit target, the user sees the target, the accumulated **net** P&L since the start of the count, and the amount left to pass (shown as "reached" instead of a negative amount). Numbers are exact (X-3). Funded and live phases have no target and show no progress.
- **AC-2** Accumulated P&L comes from the same sources as the risk state (S13 AC-2): execution results and plugin-reported P&L, net with the S13 AC-11 commission rule. Manual trades do not count.
- **AC-3** If the target is "not confirmed" or absent, the progress is not shown and the screen says why; the product never picks a value (S16 AC-6).
- **AC-4** **Start of the count:** the date given by the user, defaulting to the account's registration date. The user may enter an **amount already earned** in the evaluation (counted as opening net P&L) and may **restart the count**; each is audited. Only what the product captured from the start date counts, plus that entered amount, and the screen says so; a platform reset (Sim reset, Playback rewind) does not restart it (S11 AC-6).
- **AC-5** **On reaching the target** the user gets one notice on the account and one notification (S18), and **nothing is done automatically** (no flatten, no block).
- **AC-6** Minimum trading days and payout rules are **not** part of V1; consistency stays tracked only (Q4). [Decision 2026-10-23e/f]

### V1-S14 — Core risk gate (ready)

- **AC-1** The Core refuses to route a new instruction to an account that has reached a limit — **also when enforcement is off** for that account (refusing new entries never closes a trade); the refusal is audited and notified (§5; Q2 [Decision 2026-10-23]). Exits (`close`, reduce, protective modify/cancel) are never refused by the risk or out-of-sync gates (P1 [Decision 2026-10-23b]).
- **AC-2** When a destination account hits its limit with enforcement on, it is removed from routing until the next trading day; other accounts continue. (§8)
- **AC-3** When the **master** hits its limit, only the master is flattened and blocked; followers keep open positions and receive no new instructions from that master until the next trading day. (§8)

### V1-S15 — Plugin enforcement (ready)

- **AC-1** Enforcement is off by default and enabled per account by the user. (§9)
- **AC-2** With enforcement on, the plugin retrieves the account's limits from the Core and monitors live P&L; on reaching a limit it flattens positions, cancels working orders, and blocks trading on that account until the reset. (§9)
- **AC-3** No early unlock: neither the Web nor the plugin offers a way to unblock before the reset. (§9)
- **AC-4** Enforcement works while the Core is unreachable, using the last limits received (cached in the plugin) — second barrier, §5. (Feasible per Architect, 2026-10-23.)
- **AC-6** **Block is reactive** (NinjaTrader offers no account lock): while an account is blocked, any order the user places manually on it is cancelled, and any resulting position is flattened, immediately after submission; each such action is audited and shown in the plugin. The user is told in the plugin that the account is blocked and why. Exact timing to be measured in the NT8 spike.
- **AC-7** In Playback the trading-day reset and daily P&L use the connection's (simulated) clock, not the computer's clock. A Sim reset or Playback rewind is handled per S11 AC-6 (not counted as P&L).
- **AC-5** Every flatten/cancel/block is audited with the limit and value that triggered it (X-5).
- **AC-8** Turning enforcement **on** while a limit is already exceeded today enforces at once (flatten, cancel working orders, block until the reset), after a confirmation that names any open position that will be closed (Q61 a). [Decision 2026-10-23e]
- **AC-9** When the reset ends a block, the user gets an informational notification and an audit entry with the reason "trading day reset" (Q62 a). [Decision 2026-10-23e]
- **AC-10** Turning enforcement **off** returns a "flatten and block at the profit target" setting to "notify only", stated in the confirmation and audited (Q63 a). [Decision 2026-10-23e]
- **AC-11** Enforcement cannot be turned on for an account with **no enforceable limit** (Q64 b), for an **unlinked** account (S5 AC-12) or for a **manual-only** firm account (S16 AC-7); it cannot be turned off while the account is blocked (AC-3). Each case shows the reason. [Decision 2026-10-23e]
- **AC-12** In the plugin the user is told about a block or warning without the plugin taking keyboard focus: a log entry, a non-activating toast if feasible, and a permanent status on the account in the plugin panel (Q65 a). [Decision 2026-10-23e]
- **AC-13** Limits, drawdown and the daily profit target are measured on **net P&L** (S13 AC-2, AC-11); the plugin monitors net live P&L using the same effective commission. [Decision 2026-10-23f]

### V1-S16 — Prop firm presets: model and UI (ready)

- **AC-1** A preset records firm, plan, account size, phase, **platform/connection** (e.g., Rithmic, Tradovate; rules that differ by platform are stored per platform), all rules from S13, maximum position size, trading-day reset time, the firm's stance on trade copying and automated trading (classification + verbatim text), source URL(s), verification/access date, and version. (§9, §12; [Decision 2026-10-23b])
- **AC-2** Selecting a preset fills the account's rules and max position; the user can edit the copy on their account without changing the preset. **Sequencing (rev. 7, PO):** until S13 ships, S16 delivers fill-from-preset, choosing a published value or typing one for a "not confirmed" field (AC-6), restoring a preset value, and editing max position (S5 AC-3); **free editing of the copied risk rules (drawdown, daily loss, profit target, consistency, reset) is delivered by S13 AC-1** and S16 AC-2 is QA-complete only after S13. No user decision needed: no rule changes, only the slice in which the editing arrives.
- **AC-3** When a preset gets a new version, existing accounts are not changed silently; the user is told and chooses whether to apply. [Decision 2026-10-23, plan approval]
- **AC-4** The firm's stance on copying/automation is shown **with its source and date** when the user registers an account with that firm and when they add it to routing (§12; [Decision 2026-10-23b]). For firms that forbid automation, S5 AC-7 applies.
- **AC-5** A preset is selectable when its fields are verified **or** marked "not confirmed"; fields that conflict across the firm's official pages ship as **"not confirmed"** and do not block the rest of the preset ([Decision 2026-10-23b]). A firm with no verified data is not offered.
- **AC-7** **Manual-only presets:** a firm whose platforms cannot be reached by any V1 output (e.g., **Topstep**: TopstepX/ProjectX only; orders must originate from the personal device) ships a preset for **manual tracking only** — risk rules, 80% warnings, manual trades and performance — with no routing; the limit and its reason are explained at registration and on the account screen (S-1 [Decision 2026-10-23d]). A future on-device ProjectX adapter is only for later evaluation.
- **AC-8** The preset also records the firm's **VPS/server rule** (allowed / forbidden / conditional / unknown) and whether a **personal device** is required, with verbatim text, source and date, shown with the other stances (AC-4) [Decision 2026-10-23d].
- **AC-9** Plans excluded from V1 presets (registered manually if the user has them): Apex Legacy, Phidias LIVE (2026-10-23c), Topstep Labs Challenges and Pro Account, Take Profit Trader PRO+ and PRO+ Development (S-3/S-4 [Decision 2026-10-23d]).
- **AC-6** A "not confirmed" field is **not pre-filled**: the user sees each published value with its source and picks one or types their own; a required field (e.g., max position, drawdown, reset time) cannot be saved until resolved. The product never picks "the most conservative" value. Current list: [r1-conflicts-for-user.md](../presets/r1-conflicts-for-user.md).
- **AC-10** For presets of an **evaluation** phase the screen shows the **evaluation profit target** with its source and date (needed by V1-S13b); minimum trading days and payout rules are stored in the preset and **not shown** in V1 (Q46, user exception [Decision 2026-10-23e]).
- **AC-11** Applying a newer preset version to an account with edited values is chosen **field by field with no preselection**; one notification per firm announces the new version (S18). [Q-S16-4, Q-S16-5]
- **AC-12** The preset stores the firm's **commission per contract per side** when the firm publishes it (with source and date) and prefills the account's commission floor with it (S13 AC-11); when the firm does not publish it, the field starts empty and the account shows the "comissão não informada" warning until the user enters a value. [Decision 2026-10-23f]

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
- **AC-3** V1 has **in-app notifications only** (Web + plugin); no email or messaging-app channels (Q8 [Decision 2026-10-23]). Account e-mails needed by S3 (invitation, sign-in codes, new-device notices) are not notifications.
- **AC-4** An entry skipped because the account has no stop/target profile creates a notification, grouped with repeats (Q50 a). Skips caused by the user's own choice (automation not accepted, signal type not selected) are audited only. [Decision 2026-10-23e]
- **AC-5** The "maximum alert" for the automatic flatten of an orphan-leg position (Q27 b) is a fixed, non-dismissible alert at the top of the Web, with the equivalent in the plugin, that stays until the user marks it as read, plus the audit entry (Q51 a). [Decision 2026-10-23e]
- **AC-6** Identical notifications (same code, account and trading day) are shown as one group with a count and the last time; the first occurrence is unread; when a group already read receives a new occurrence it becomes unread again for **danger** codes (`routing.risk_blocked`, `routing.out_of_sync`, `routing.expired`), while other groups stay read and only the count grows (Q68 b). The audit keeps one entry per event. [Decision 2026-10-23e]
- **AC-7** The read state is shared between the Web and the plugin (one unread count). Notifications are kept for the same period as the audit log; the plugin shows only the last 200 items as a display cache.

### V1-S19 — Master replication: entries and exits (ready — V1.1)

- **AC-1** The user designates one NinjaTrader account as master and selects follower accounts (NinjaTrader, same or other instances of the same user). (§8)
- **AC-2** Entries and exits on the master are replicated to each follower with its multiplier and maximum position cap; each follower's **initial stop and target come from the follower's own profile** (Q5 [Decision 2026-10-23]).
- **AC-3** Works across instances through the Core; all involved instances must be connected, otherwise the follower is flagged out of sync (S11). In V1, replication **always relays through the Core**, also when master and followers share one instance; a local fast path is not built in V1 and may be revisited with measured latency (A4 [Decision 2026-10-23]).
- **AC-4** No artificial delay; latency per follower is recorded in the audit trail.
- **AC-5** QA of cross-instance replication needs **two Windows machines/VMs** (NinjaTrader runs one instance per computer). Access per W1 (reverse SSH tunnel from Windows, no inbound port) and W3 (user types the NT8 password on restarts) [Decision 2026-10-23c]; the machine type (W2: Contabo Windows VPS without GPU if feasible) is being confirmed by the team.

### V1-S20 — Master replication: everything else (ready — V1.1)

- **AC-1** All order types placed on the master, their modifications, and cancellations are replicated. (§8)
- **AC-2** When the master's stop or target moves by X, each follower's moves by the same X. (§8)
- **AC-3** A master cancellation cancels the linked follower orders, **including protective orders**, with a warning notification on each affected follower (it may leave followers unprotected) (P3 [Decision 2026-10-23b]); failures flag out of sync.
- **AC-4** The active contract month comes from a global rollover catalog maintained by the admin; users are warned in advance and the Core switches on the configured date (P5 [Decision 2026-10-23b]).

### V1-S21 — Trade capture and trade list (ready)

- **AC-1** Each completed trade on a connected account is captured with: account, instrument, direction, quantity, entry/exit time and price, time in trade, **net P&L (with commissions) as the main value and gross P&L beside it**, MAE, MFE, and originating signal/source. (§10; commissions: Q54 exception [Decision 2026-10-23e])
- **AC-2** Metrics are computed by the plugin from the user's own data; only computed numbers are sent; no raw price data is stored (§10).
- **AC-3** The user can enter a trade manually with the same fields (source = "manual"); see AC-7 to AC-11.
- **AC-4** The user can list and filter trades by account, date, source, and signal type.
- **AC-5** MAE and MFE are in USD, exact strings like P&L (Q52 a).
- **AC-6** The date filter uses the trade's **exit** time in the profile time zone (Q53 a).
- **AC-7** In a manual trade MAE/MFE are optional: shown as "—" when missing and left out of the MAE/MFE averages, with the number of trades used shown (Q55 a). The manual entry also has an optional commissions field; gross P&L is computed from prices, quantity and tick value, and net = gross − commissions. The effective commission of a captured fill follows S13 AC-11 (account floor). On a manual trade a blank commissions field means **zero**: net equals gross and the trade is labeled "comissões não informadas" **[Provisional — decided provisionally by the user (Q71 b), awaiting confirmation; display only, since manual trades never feed risk]**.
- **AC-8** The user can edit and delete a manually entered trade; each change is audited with before and after and the metrics follow. Captured trades cannot be edited (Q56 a).
- **AC-9** "One trade": in individual mode, one trade per **entry**, exits matched first-in-first-out (S8 AC-4b), a partial exit splits the entry; in shared mode, a flat-to-flat round trip (Q57 c).
- **AC-10** A trade placed directly in the platform without a signal has its own origin category, distinct from "manual" (hand-entered).
- **AC-11** Manual trades feed **only** the trade list and the performance metrics, never the risk state or enforcement (S13 AC-2).

### V1-S22 — Performance metrics view (ready)

- **AC-1** For a selected period and set of accounts, the user sees totals and per-trade averages of P&L, win rate, time in trade, MAE, and MFE, split by source and signal type. [Assumption — exact metric set refined with the Designer; changes beyond this list go to the user]
- **AC-2** Numbers in the view reconcile exactly with the trade list (X-3).
- **AC-3** Win rate counts a trade as a win only if its **net** P&L is greater than 0; the denominator is all trades; the screen states the counts (wins, losses, zero) (Q58 a). Totals and averages of P&L use net P&L (S13 AC-11 commission rule) with gross beside it. [Decision 2026-10-23e/f]
- **AC-4** V1 shows only the AC-1 list; extra metrics (profit factor, best/worst trade, average win/loss, per-day curve) come in a later slice (Q59 a). Totals: total P&L and number of trades; time in trade, MAE and MFE only as averages.

---

## 6. Risks

| Risk | Impact | Mitigation |
|---|---|---|
| NinjaTrader plugin stability (connection, order state, multi-instance) | Wrong or missing executions | Thinnest E2E first (M1); out-of-sync detection (S11) and exactly-once (S12) before any real use; sim-only development (X-7) |
| TradingView webhook delivery is best-effort (no guaranteed retry) | Missed signals | Audit shows received signals; user can compare with TradingView alert log; documented in S17 guide |
| Local OCO on NinjaTrader/Rithmic connections: a stop fill does not cancel the target while NT8 is down | Unprotected or unwanted positions after an outage | S9 AC-4 user-facing limit; S11 AC-5 reconnect check; Q27 |
| Plugin verification needs Windows + NT8 (two machines for S19 cross-instance) | Critical-path bottleneck | Q15; adapter-sim lets Core proceed without it |
| Unique id generation inside Pine alerts | Duplicates or false "duplicate ignored" | Architect defines id rule in JSON contract v1; S17 AC-3 tests it |
| Prop firm rule research volume (18 firms) and frequent rule changes | Release delay or wrong limits | Start R1 now; per-firm "verified" status; report-to-user rule; re-verification before release |
| Firms that forbid copying or automation | User account violations | Stance shown at registration and routing (S16 AC-4); not legal advice (§12) |
| Core accidentally becomes NinjaTrader-specific | Rework for Tradovate in release 2 | X-1 review on every Core slice |
| Risk state in Core diverges from platform's real P&L | Wrong blocks or missed limits | Plugin is the second barrier (§5); S15 AC-4 |
| Sign-in depends on e-mail (and later WhatsApp) delivery; a provider outage or spam-foldering locks users out | Nobody can sign in or confirm sensitive actions | SRE/availability input; Q28 recovery path; resend rules (Q33) |
| Terms of use / privacy policy not ready | Launch blocked | Flagged now as user-owned release gate |

---

## 7. Questions for the user — decided 2026-10-23

> **Decided 2026-10-23e ([decision log](../../delivery/decisions.md)):** the user accepted every recommendation of U16, M2, M3 and **Q28–Q68**, with two exceptions: (1) **commissions (Q54, which the user wrote as "Q55")**: net P&L, with commissions, is the main value and gross is shown beside it, because firms measure drawdown and limits on net; (2) **Q46**: the evaluation profit target (how much is left to pass) is included in V1; minimum trading days and payout later. The Q-tables below keep the original options and recommendation; the resulting criteria are in the slices, tagged [Decision 2026-10-23e].

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

### Follow-up questions from design (S3/S4) — decided 2026-10-23

All accepted per recommendation [Decision 2026-10-23b]; Q21 = 12 h idle / 24 h absolute session, remember-me 7 d idle / 30 d absolute, step-up after 15 min (Architect [auth-and-session-policy.md](../../architecture/security/auth-and-session-policy.md)). S3/S4 criteria follow these answers.

| # | Question | Options | Recommendation |
|---|---|---|---|
| Q17 | How many TradingView sources (each with its own secret URL) per user? | (a) Several named sources (e.g., one per chart/alert); (b) one per user in V1 | **(a)** — the routine is "one alert per chart", and per-source routing (S7) and revocation then stay independent |
| Q18 | Unpairing a NinjaTrader instance whose accounts are routing destinations, possibly with an open position | (a) Allow after a confirmation listing affected accounts; with an open position, a stronger warning that the position stays open in NinjaTrader without platform monitoring/enforcement, and the accounts are flagged out of sync; (b) block unpairing while a position is open | **(a)** — the user stays in control; nothing is closed automatically |
| Q19 | Who issues invitations (Q6) and how? | (a) A platform-admin role (held by the user), single-use link bound to one e-mail, valid 7 days, delivered manually (copy link); (b) any user can invite | **(a)** |
| Q20 | Transactional e-mail in V1 for password reset (and invitations)? Q8 excluded e-mail *notifications* only | (a) Yes, transactional e-mail only for invite and password reset; (b) no e-mail: the admin generates a one-time reset link | **(a)** — self-service reset; (b) as fallback if e-mail is not wanted **Superseded rev. 7 — password reset no longer exists; transactional e-mail now carries invitations, sign-in codes and new-device notices.** |
| Q21 | Session length / "keep me signed in" | Architect proposes values (idle and absolute timeout); user approves | Architect proposal → user **Superseded rev. 7 — values carried over to the passwordless model, pending Q29.** |
| Q22 | Optional TOTP recovery | (a) 10 single-use recovery codes shown once at activation; enabling/disabling requires password + current code; lost device without codes → admin-assisted reset after identity check; (b) no recovery codes, admin reset only | **(a)** **Superseded rev. 7 — TOTP and recovery codes no longer exist (passwordless sign-in).** |
| Q23 | Locale defaults: first-visit language, display time zone, time format | (a) Language from the browser (pt or en, else pt-BR); time zone from the profile (default = browser), always shown next to reset/session times; 24-hour clock in both languages; (b) always pt-BR, fixed America/Chicago | **(a)** |

### Questions for the user — 2026-10-23 (rev. 4) — decided

Q24–Q27 accepted as recommended [Decision 2026-10-23c]. R1 conflict list: recommendations accepted, **Apex Legacy and Phidias LIVE excluded** from V1 presets; the user reviews the 31 conflicts later and until then they ship as "not confirmed".

| # | Question | Options | Recommendation |
|---|---|---|---|
| Q24 | Does the automation opt-in cover only automated sources (TradingView webhook, future signal engine), with master replication governed by the copying stance? And for firms that forbid or condition **copying**, should followers need the same explicit opt-in? | (a) Yes to both: automation opt-in for automated sources; a separate explicit copy opt-in for follower accounts of firms that forbid/condition copying; (b) one opt-in covering both | **(a)** — Apex forbids automation but allows same-person copiers, so the two stances must be separate |
| Q25 | Ask the explicit choice also when the automation stance is conditional, unknown, or not confirmed? | (a) Yes, for any value other than "allowed"; (b) only for "forbidden" | **(a)** — protects the account when the rule is unclear |
| Q26 | A new preset version changes the automation stance or text (e.g., allowed → forbidden) | (a) Keep the choice and notify; (b) suspend automated sources on that account until the user confirms again, and notify | **(b)** |
| Q27 | Both bracket legs filled while NinjaTrader was down, leaving an unwanted unprotected opposite position (S11 AC-5 iii) | (a) Alert and wait for the user; (b) automatically flatten **only** the position created by the orphan leg, with maximum alert and audit | **(b)** (PO + Architect) — a position the user never chose, without a stop; (a) applies until decided |
| — | Preset conflicts R1 (31 items) | See [r1-conflicts-for-user.md](../presets/r1-conflicts-for-user.md) | Recommendations accepted [Decision 2026-10-23c]; item-by-item review later |
| — | R2 scope findings | See [r2-conflicts-for-user.md](../presets/r2-conflicts-for-user.md) | S-1..S-5 decided [Decision 2026-10-23d]; later R2 items open |

### Questions for the user — passwordless sign-in (rev. 7) — decided 2026-10-23e

User requirement 2026-10-23: no password; e-mail/WhatsApp codes ([S3](#v1-s3--passwordless-sign-uplogin-isolation-source-keys-plugin-pairing-ready-ac-12-blocked-by-q28)). Items **U16** (code format) and **U17** (WhatsApp supplier/cost) are already open with the Maestro/Architect and are **not decided here**. New gaps:

| # | Question | Options | Recommendation |
|---|---|---|---|
| Q28 | What happens if the user loses access to **all** verified channels (or to the old channel when changing e-mail/WhatsApp)? | (a) Admin-assisted recovery after an identity check by the admin; the admin issues a one-time recovery for a new e-mail; audited; the old e-mail gets a notice; (b) none — the user asks for a new invitation and starts over (old data stays inaccessible or is merged by the admin manually); (c) a pre-registered third channel/recovery contact | **(a)** — same spirit as the former admin-assisted TOTP reset; (c) adds a channel to secure |
| Q29 | Sessions: keep the Q21 values (12 h idle / 24 h absolute; "keep me signed in" 7 d / 30 d)? Does "remember this device" also skip the new-device notice? | (a) Keep Q21 values; the new-device notice still fires on any first use of a device; (b) shorter sessions now that each sign-in needs an external code; (c) longer | **(a)** |
| Q30 | Step-up frequency: new code on **every** sensitive action, or one code valid for a short window (e.g. the earlier 15 min) covering several actions? | (a) Every action asks a new code (literal reading of the requirement); (b) 15-min window | **(a)** for V1 — sensitive actions are rare; revisit if it annoys the user |
| Q31 | WhatsApp at sign-up before S3b exists, and after: is it mandatory? | (a) Optional: sign up with e-mail only; WhatsApp is added later in the profile; after S3b, a reminder but never forced; (b) mandatory for new invitations once S3b ships, existing users get a deadline; (c) mandatory from day 1 (blocks release until a supplier exists) | **(a)** — the requirement says WhatsApp "may come after e-mail"; the e-mail path must work alone |
| Q32 | Can an e-mail/WhatsApp number be used by more than one account (e.g. a shared family number)? | (a) One account per e-mail and per number; (b) numbers may be shared | **(a)** — a shared channel breaks per-user isolation of codes |
| Q33 | Code sending limits (per user, per IP, resend cooldown)? | (a) Architect proposes numbers; user approves; (b) user sets them | **(a)** |
| Q34 | New-device notice: content and handling | (a) Time, browser/OS summary and approximate location, plus a "this was not me" instruction that leads to ending all sessions; no automatic action; (b) plain information only | **(a)** if the Architect confirms feasibility; "sign out of all devices" was a PO item outside S3 and would enter S3 here |
| Q35 | Step-up list: keep only the user's three actions (automation on, unpair NT8, change e-mail/WhatsApp), or also the Architect's earlier list (source token create/rotate, pairing code, enforcement off, raising max position, admin actions)? | (a) The three now; add the others if the user asks; (b) all of them | **(a)** minimum as stated; the Architect to say which extras are a security must **DECIDED 2026-10-23e (not option a): the user accepted the Architect's set with the server setting ON — the three actions plus `source_create`/rotate, `connection_create`, `pairing_code_new`, `admin_invitation_create`, `route_enable` and `account_binding_change` (S3 AC-10).** |
| Q36 | Does the invitation e-mail have to equal the sign-up e-mail, and may the invitee change it at sign-up? | (a) Must be equal (Q19 binding); (b) invitee may use another e-mail, confirmed by code | **(a)** |

### Questions for the user — S5 follow-ups from design (Q-S5-x) — decided 2026-10-23e

Raised by the Product Designer in the S5 opt-in spec §17. Items already answered by the plan are not repeated here.

| # | Question | Options | Recommendation |
|---|---|---|---|
| Q37 | Is linking an account to an account exposed by a paired NinjaTrader instance required to save it (S5 AC-1 says "link it")? Manual-only accounts (Topstep, S16 AC-7) can never be linked | (a) Optional: an account can be saved unlinked; unlinked accounts are tracked manually and can never be routed or receive automation; (b) required for every account except manual-only presets | **(a)** |
| Q38 | May the linked platform account be changed while the account has an open position? | (a) Blocked while a position is open (same spirit as deleting, S5 AC-6); (b) allowed with a strong warning; the account is flagged out of sync | **(a)** |
| Q39 | Deleting an account (refused with an open position, S5 AC-6): what happens to its routes, to a follower relation, and to its history? | (a) The account is archived: hidden from lists; its routes and any master/follower relation are removed after a confirmation that lists them; history (audit, trades) is kept per S10 AC-3 and the removal is audited; (b) hard delete of everything except the audit | **(a)** — S10 AC-3: entries cannot be deleted by the user |
| Q40 | S5 ships before the preset catalog (S16). How is a prop account registered meanwhile (firm/plan/phase, max position), and what about the automation opt-in? | (a) Firm, plan and phase as free text; max position typed by the user; the firm's stance is "unknown", so the explicit opt-in (Q25) is required with a generic warning that no rule text is available; (b) no opt-in until S16 — prop accounts cannot receive automated sources before S16; (c) no opt-in, the user is responsible (sim only) | **(a)** — applies Q25 consistently ("unknown" needs a choice) and costs little |
| Q41 | When S16 later attaches a preset to prop accounts created before it, do those accounts go to "review pending" (automated sources suspended until the user confirms the stance, as in Q26)? | (a) Yes; (b) keep the previous choice and notify | **(a)** — consistent with Q26 |
| Q42 | What is a **material change of a stance text** that suspends automation (S5 AC-11 includes "or its text")? Typo or format fixes should not | (a) The person who publishes the preset version flags the change as material (meaning changed) and records why; unflagged changes never suspend; (b) any change in the verbatim text suspends; (c) only a change of the stance class | **(a)** |

### Questions for the user — S6 follow-up from design (Q-S6-2) — decided 2026-10-23e

| # | Question | Options | Recommendation |
|---|---|---|---|
| Q43 | A new instruction entry reaches an account whose size has **no stop/target profile defined** (the system ships no invented default values; S6 AC-1, AC-5 make the profile the only source of stop/target). What happens? | (a) The entry is **skipped and audited** with the reason "no stop/target profile"; exits (`close`, reduce) still work (P1); the account screen shows a banner asking the user to define the profile; (b) the entry executes **without** stop/target (unprotected position); (c) the system ships default values (needs the user to supply them) | **(a)** — never opens an unprotected position; (c) only if the user provides the numbers |

### Questions for the user — S10 follow-ups from design (Q-S10-1, Q-S10-4) — decided 2026-10-23e

| # | Question | Options | Recommendation |
|---|---|---|---|
| Q44 | Can the user **export** their audit log (e.g. CSV)? Not in S10 AC-1..4 | (a) Not in V1: the log is readable and filterable on screen; export is a later slice; (b) CSV export of the filtered list in V1 | **(a)** — smaller scope; the log already holds up to 24 months. Whether a data-portability right applies is for the privacy policy (user-owned) |
| Q45 | The retention period (24 months rolling, "configurable", S10 AC-3): can the **user** change it, or is it a system setting changed only by the admin? | (a) Read-only for users, shown as one line; changed only by the admin through system configuration; (b) a user setting within limits | **(a)** |

### Questions for the user — S16 follow-up from design (Q-S16-1) — decided 2026-10-23e

| # | Question | Options | Recommendation |
|---|---|---|---|
| Q46 | Presets store, from the research, the evaluation **profit target**, **minimum trading days** and **payout rules that change what the trader may do** (research format §4). No slice uses or tracks them in V1. Should the preset screen show them? (The daily profit target of S13 AC-5 is a different item and is covered.) | (a) Not shown in V1: stored with source and date in the preset only; tracking an evaluation's progress is a later slice; (b) shown as read-only information with source and date, not tracked or enforced | **(a)** — a read-only list of rules the product does not follow could be read as if it did; revisit with an evaluation-progress feature **DECIDED (exception): the evaluation profit target is included in V1 (see S13b, S16 AC-10); minimum days and payout later.** |

### Questions for the user — S13 risk follow-ups from design (Q-S13-1, 3, 4) — decided 2026-10-23e

Risk rules decide when trading is blocked and positions are closed: nothing below is applied until the user decides.

| # | Question | Options | Recommendation |
|---|---|---|---|
| Q47 | Which risk rules must a **prop** account have before it can be saved? Written today: drawdown and reset time cannot stay "not confirmed" (S16 AC-6); max position is mandatory (S5 AC-3). Not written: the daily loss limit | (a) Prop accounts need drawdown and daily loss limit, unless the preset states the firm has **no** daily loss limit (explicit "none"); everything else optional; real accounts: every rule optional ("can set", S13 AC-1), with unset rules shown as "not set"; (b) only drawdown required | **(a)** |
| Q48 | When does an edited limit take effect, and may lowering it below what the day has already used trigger enforcement at once? Raising a limit must not unblock a blocked account before the reset ("no early unlock", vision §9) | (a) Every edit takes effect immediately; a block already applied stays until the reset; (b) **tightening** is immediate, **loosening** applies from the next trading day; (c) every edit applies from the next trading day. In (a) and (b), if the new value is already exceeded, the Core stops routing new entries at once (S14 AC-1) and, with enforcement on, the plugin flattens at once, after a warning before saving that names this effect | **(b)** — tightening protects at once, loosening cannot be used to bypass a limit mid-day |
| Q49 | S13 AC-5 describes "flatten and block at the daily profit target" as "(enforcement on)". May it be saved while the account's enforcement is **off**? With enforcement off the Core only refuses new entries (S14 AC-1); nothing is flattened | (a) Allowed to save, with a clear warning that nothing will be flattened until enforcement is turned on; (b) the option is unavailable until enforcement is on, with the reason shown; (c) choosing it turns enforcement on for the account | **(b)** — avoids a user believing the account will be flattened when it will not; (c) changes a safety setting as a side effect |

### Questions for the user — S18 follow-ups from design (Q-S18-2, Q-S18-3) — decided 2026-10-23e

| # | Question | Options | Recommendation |
|---|---|---|---|
| Q50 | An entry is skipped because the account has **no stop/target profile** (Q43 option a). S18 AC-1 lists "refusals" as notifiable; skips caused by a choice the user made (automation not accepted, signal type not selected — S7 AC-2, AC-6) are audited only | (a) Notify (a missed trade because of missing configuration is unexpected), repeated ones grouped; (b) audit only | **(a)** |
| Q51 | What is the "maximum alert" for the automatic flatten of an orphan-leg position (Q27 b), which the user did not ask for? | (a) A fixed, non-dismissible alert at the top of the Web (and the same in the plugin) that stays until the user marks it as read, plus the audit entry; (b) a high-priority notification in the list only; V1 has no other channel (Q8) | **(a)** — an automatic closing of a position must not be missable |

### Questions for the user — S21 trade list follow-ups from design (Q-S21-1, 2, 4, 5, 6, 7) — decided 2026-10-23e

These define the numbers the performance view (S22) reconciles with the trade list (S22 AC-2, X-3).

| # | Question | Options | Recommendation |
|---|---|---|---|
| Q52 | Unit of MAE and MFE (vision §10 names them without a unit) | (a) USD, exact strings like P&L; (b) points; (c) ticks | **(a)** — comparable with P&L and averageable across instruments |
| Q53 | The trade list/period filter (S21 AC-4, S22 AC-1) uses which date? | (a) The trade's **exit** time, in the profile time zone (Q23); (b) the trading day (the account's reset time, S13 AC-4); (c) entry time | **(a)** — the P&L is realized at exit; trading-day grouping can come with S22 if wanted |
| Q54 | Do commissions/fees count in a trade's P&L? | (a) Gross P&L only in V1, stated on the trade detail; manual trades are computed from prices, quantity and tick value; (b) net P&L when the platform reports fees, and an optional fee field in manual entry | **(a)** — adapters may not report fees reliably; avoids mixing gross and net in one column **DECIDED (exception): option (b) with net P&L as the main value and gross beside it.** |
| Q55 | MAE/MFE in a **manually entered** trade (S21 AC-3 says "same fields") | (a) Optional: shown as "—" when missing and left out of MAE/MFE averages, with the number of trades used shown; (b) required; (c) missing counts as zero | **(a)** — the user rarely knows them; (c) would distort averages |
| Q56 | May the user **edit or delete** a manually entered trade? Captured trades cannot be edited | (a) Edit and delete allowed for manual trades; each change audited with before/after; the metrics follow; (b) delete only; (c) neither | **(a)** |
| Q57 | What is "one trade" when entries are added to a position or partly closed (S21 AC-1 says "completed trade")? | (a) Flat-to-flat round trip (all entries and exits until the position is flat); (b) one trade per **entry**, exits matched first-in-first-out (as the protective orders, S8 AC-4b), a partial exit splits the entry; (c) hybrid: (b) in individual mode, (a) in shared mode, where the position has one stop and target | **(c)** — in individual mode each entry has its own stop/target and its own signal, so win rate by signal type (S22 AC-1) needs one trade per entry; shared mode manages one position |

### Questions for the user — S22 performance view follow-ups from design (Q-S22-1, Q-S22-4) — decided 2026-10-23e

| # | Question | Options | Recommendation |
|---|---|---|---|
| Q58 | How does **win rate** (S22 AC-1) count a trade with P&L exactly 0? | (a) Not a win: win rate = trades with P&L > 0 divided by all trades; the screen states the counts (wins, losses, zero); (b) zero-P&L trades are left out of both the numerator and the denominator; (c) zero counts as a win | **(a)** |
| Q59 | Which metrics does the view show? AC-1 lists totals/averages of P&L, win rate, time in trade, MAE and MFE, split by source and signal type. Extras the designer could draw: profit factor, best and worst trade, average win and average loss, per-day P&L curve | (a) Only the AC-1 list in V1; extras in a later slice; (b) add profit factor, best/worst trade, average win/loss; (c) add all of them including the per-day curve | **(a)** — extras depend on how a trade is counted (Q57) and on fees (Q54); better to add them once those are decided |

### Questions for the user — real-account profile tie-break (Q-S6-7) — decided 2026-10-23e

| # | Question | Options | Recommendation |
|---|---|---|---|
| Q60 | A real account starts from the profile of the "nearest standard size" (S5 AC-2, Q12). Distance is measured by the difference in account amount; below 25k uses 25k and above 300k uses 300k. If the size is **exactly halfway** between two standard sizes (e.g. 37,500 between 25k and 50k), which one? | (a) The smaller size; (b) the larger size; (c) no automatic choice: the user picks one of the two when saving the account (the account starts from the profile they choose) | **(c)** — the written rule does not say which profile is safer, and the case is rare |

### Questions for the user — S15 enforcement follow-ups from design (Q-S15-2, 3, 5) — decided 2026-10-23e

Enforcement flattens positions and blocks accounts: nothing below is applied until the user decides.

| # | Question | Options | Recommendation |
|---|---|---|---|
| Q61 | The user turns enforcement **on** while a limit is already exceeded today (S15 AC-2 says "on reaching a limit"). The Core already refuses new entries regardless of enforcement (S14 AC-1) | (a) Enforce at once: flatten, cancel working orders and block until the reset, after a confirmation that names any open position that will be closed; (b) enforcement starts at the next trading day; today only the Core refusal applies; (c) the switch stays unavailable until the reset | **(a)** — the user asked for protection explicitly; consistent with Q48 (b) for tightening |
| Q62 | When the reset ends a block, does the user get a notification, and is the release audited? S18 AC-1 lists "blocks" but not their end; X-5 says automatic actions are audited | (a) Both: an informational notification and an audit entry with the reason "trading day reset"; (b) audit entry only; (c) neither | **(a)** |
| Q63 | The user turns enforcement **off** (allowed when the account is not blocked) while "flatten and block at the daily profit target" is set (S13 AC-5, Q49) | (a) The target setting returns to "notify only", stated in the confirmation and audited; (b) the setting stays but only the Core refusal of new entries applies | **(a)** — consistent with Q49 (b), where the option exists only with enforcement on |
| Q64 | May enforcement be turned on for an account that has **no limit set** (possible for real accounts, Q47)? | (a) Allowed, with a permanent status "no limits set — nothing to enforce"; (b) the switch needs at least one enforceable limit | **(b)** — a switch that does nothing can be mistaken for protection |

### Questions for the user — plugin panels follow-ups from design (Q-P1, Q-P2) — decided 2026-10-23e

| # | Question | Options | Recommendation |
|---|---|---|---|
| Q65 | When an account is blocked or a warning arrives, how does the plugin tell the user (S15 AC-6 says the user is told in the plugin and why)? A window that opens by itself takes keyboard focus while the user may be typing an order | (a) Never takes focus: an entry in the plugin log, a non-activating toast if feasible, and a permanent status on the account in the plugin panel; (b) the plugin window opens by itself on a block | **(a)** — stealing focus from a trading window can cause mistaken orders |
| Q66 | Unpair **from the plugin itself** (Q18 allows unpairing with a confirmation listing the affected accounts, a strong warning if a position is open or enforcement is on, and the accounts become out of sync). S3 AC-10 requires a new code for unpairing, which the plugin cannot receive | (a) The plugin offers a **local disconnect** only: warning with the consequences and a checkbox to confirm; the pairing stays registered in the Core, which shows the instance as disconnected and applies the out-of-sync rules; **revoking** the pairing is done in the Web with the new code (S3 AC-10); (b) the plugin unpair also revokes the pairing in the Core, without the code | **(a)** |

### Questions for the user — route switch on an ineligible account (Q-S7-8) — decided 2026-10-23e

| # | Question | Options | Recommendation |
|---|---|---|---|
| Q67 | S7 AC-6 says an account that does not accept automated sources "cannot be subscribed" to an automated source (creation is blocked). What about the on/off switch of a route that already exists when the account becomes ineligible (declined opt-in, pending review after a preset change)? Meanwhile nothing is routed: instructions are skipped and audited | (a) The switch can be turned on and off freely; the choice is saved and the status shows why the route is not active; if the account becomes eligible again, routes left "on" start working; (b) turning **off** is always allowed; turning **on** is unavailable while the account is ineligible, with the reason, so every route starts again only by a new explicit action; (c) the switch is disabled in both directions | **(b)** — it follows AC-6 literally, never traps the user with a route they cannot turn off, and nothing restarts by itself when the opt-in is accepted again |

### Questions for the user — grouped notifications that were already read (Q-S18-1) — decided 2026-10-23e

| # | Question | Options | Recommendation |
|---|---|---|---|
| Q68 | Repeated identical notifications (same code, account and trading day) are shown as one group with a count. If the user already **read** the group and the same problem happens again that day, should it become unread again? Audit is not affected: it keeps one entry per event (S10 AC-1) | (a) Yes for every code; (b) yes for **danger** codes (risk block, out-of-sync, expired command), while the other groups stay read and only the count grows; (c) no, a read group stays read (current spec) | **(b)** — a second block or out-of-sync on the same day must be noticed; routine skips (e.g. excess quantity) would not re-alert |

### Questions for the user — evaluation progress (S13b) — decided 2026-10-23f

| # | Question | Options | Recommendation |
|---|---|---|---|
| Q69 | Where does the evaluation progress start counting for an account already running when it is registered? Platform resets (a Sim reset or rewind) are state discontinuities (S11 AC-6) and cannot restart it | (a) From the moment the account is registered; the user may enter an amount already earned in the evaluation and may **restart the count** (when the firm resets the account), each audited; (b) only from registration, no entry; (c) the user enters the evaluation start date (default: registration date) and the count includes captured trades from that date; earnings before registration are not known to the product | **(a)** — the product only knows trades it captured; an entered amount covers the past, and a restart covers a firm reset **DECIDED 2026-10-23f: start = date given by the user, default = registration date; optional amount already earned; "restart the count" action; each audited.** |
| Q70 | What happens when the target is reached (evaluation phase only; funded and live phases have no target)? | (a) The user is notified once and nothing is done automatically; (b) the account is also blocked or flattened like a loss limit; (c) the user chooses per account | **(a)** — consistent with the daily profit target default (Q3); the firms' pass conditions (minimum days) are not tracked in V1 **DECIDED 2026-10-23f: (a) one notice and one notification, nothing automatic.** |

### Questions for the user — commissions follow-ups (Q54 exception) — Q72 decided by U20; Q71 provisional

| # | Question | Options | Recommendation |
|---|---|---|---|
| Q71 | In a **manually entered** trade the commissions field is optional. If left blank, what commission applies? (U20 sets the account commission floor for captured fills) | (a) The account's commission per contract per side × quantity × 2 sides, labeled "estimated from account setting"; if the account has none, zero with the warning "comissão não informada"; (b) zero, net equals gross, labeled "commissions not entered"; (c) unknown: no net P&L, left out of net totals and win rate, with a count of trades left out; (d) the field is required | **(a)** — consistent with U20 and keeps every total defined; manual trades never feed risk (S13 AC-2) **PROVISIONALLY DECIDED 2026-10-23f: option (b) — zero, net equals gross, labeled "comissões não informadas"; awaiting the user's confirmation.** |
| Q72 | For **captured** trades, net P&L needs the commission from the platform. If an account's platform does not report commissions (e.g. some simulation setups), what do the risk limits and the trade list use? Net matters for risk because firms measure on net | (a) Treat commissions as zero, and the account shows the permanent flag "commissions not reported — gross used"; (b) the user enters a commission per contract on the account, used as an estimate and labeled so; (c) risk limits cannot be enforced on that account | **(a)** for V1; (b) as a later improvement. Esquadro must first confirm what NinjaTrader reports **SUPERSEDED/DECIDED 2026-10-23f by U20 (a): per-account commission floor prefilled by the preset, editable; effective commission per fill = max(platform-reported, configured × quantity); warning "comissão não informada" when neither exists.** |

## 8. Architect inputs requested

Pending answers from Prumo @ Dev3 are recorded in §9 when received.

1. JSON contract v1 (fields, id scope, key placement) — needed by S1, S17.
2. Core↔plugin protocol — needed by S2.
3. Identifiers for modify/cancel and master replication (vision §13-2) — needed by S19/S20.
4. Feasibility of S15 AC-4 (enforcement with last-known limits while disconnected).

## 9. Architect feedback log

**2026-10-23 — Local OCO (F5), Prumo (rev. 4):** S9 AC-4 and S11 AC-5 agreed; plugin reports `oco_location` (local/server/unknown, unknown = local) in hello; reconnect snapshot carries order states and correlation via order name; cases (iii) double fill → Q27, (iv) broker disconnection; Sim reset/Playback rewind = discontinuity (S11 AC-6). S19 needs two Windows machines/VMs.

**2026-10-23 — User decisions on architecture items (rev. 3):** A1 stack per ADR-0001; A2 production in the US central region (near Chicago) — no product AC impact; A3 secret in the URL, masked in every log (→ S1 AC-1, AC-6); A4 replication always via Core in V1 (→ S19 AC-3); A5 optional TOTP (→ superseded by passwordless sign-in, S3 rev. 7); A6 private GitHub + CI (push only with authorization).

**2026-10-23 — Prumo @ Dev3** (incorporated in rev. 2):
- M1 path is viable as walking skeleton. Order: ADR-0003 + ADR-0004 with fixtures → short NT8 spike → Core (against adapter-sim) and plugin in parallel. Web not needed before M1 (seed data).
- Plugin (AddOn) sees all accounts of the instance, receives executions/positions/P&L, can cancel and flatten in Sim and Playback. Limits: block is reactive (→ S15 AC-6); Playback uses simulated clock (→ S15 AC-7); enforcement must work with Core offline (→ S15 AC-4); one connection + credential per instance (→ S3 AC-15).
- ADR-0003 comes right after S0 approval; schema 1.0 reserves correlation fields so basic actions do not break later. Preliminary ids: `message_id` (dedupe), `position_ref`/`order_ref` (source), `instruction_id`/`command_id` per destination (Core), `platform_order_id` (execution report). Master replication blocked until ADR-0003 and ADR-0004 are accepted by the user.
- Architect will recommend the webhook secret in the URL rather than the body (Pine inputs can leak via status line/shared layouts) — user decision; the plan says "per-source secret" without fixing position.
- New product questions: Windows+NT8 machine (Q15), command TTL (Q16).
