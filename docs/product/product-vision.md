# Product Vision — All Win

> **Canonical location:** `docs/product/product-vision.md`
> **Owner:** the user (business decisions). The Product Owner maintains this document; every agent reads it before working on this project.
> **Status:** approved baseline · **Last updated:** 2026-10-23 (rev. 6 — opt-ins, preset rules, items 1–22, Q24–Q27 recorded; details in [release-1/slice-plan.md](release-1/slice-plan.md))
> Labels used below: **[Decision]** confirmed by the user · **[Proposal]** recommended, awaiting user confirmation · **[Open]** unresolved question.

---

## 1. Purpose

Traders of US index futures often run several accounts at once — prop firm evaluation and funded accounts plus real brokerage accounts — each with its own rules (drawdown, daily loss limit, profit target). Keeping track of those rules, understanding their own performance, and executing the same trade on every account by hand is slow and error-prone.

This platform gives the trader one place to:
1. **Manage accounts and risk** — register every account with its rules, and optionally have the platform enforce them.
2. **Route and replicate orders** — receive trade instructions from any supported source (a trading signal, or a trade placed on a master account) and execute them on any of the trader's accounts, on any supported trading platform.
3. **Analyze performance** — see exactly how each real trade behaved.

## 2. Users

- Individual traders who trade US index futures across multiple accounts they own. **[Decision]**
- Interface in **Brazilian Portuguese and English**. **[Decision]**
- Every user sees and controls only their own accounts and data.
- Release 1 sign-up is **invite-only**; two-factor authentication (authenticator app) is optional and strongly suggested. **[Decision 2026-10-23]**
- Notifications in release 1 are in-app only (web application and platform adapter). **[Decision 2026-10-23]**
- The platform adapter's language follows the user's web language. **[Decision 2026-10-23]**

## 3. Scope

### In scope

| Area | Summary |
|---|---|
| Account & risk management | Manual account registration, prop firm presets, optional automatic enforcement |
| Order routing & replication | Input → Core → Output: any supported source to any of the user's accounts on any supported platform |
| Signal input from the user's TradingView indicator | Phase 1 via webhook; Phase 2 via the platform's own signal engine |
| Performance analysis | Trade metrics captured automatically or entered manually |

### Out of scope for now

- **Copy trading between different people** (leader/member). Removed by the user on 2026-10-22. **[Decision]**
- **Browser extension that reads or automates TradingView.** Rejected 2026-10-22: TradingView draws signals on a canvas (no readable text), automating its interface is fragile and restricted by its terms of use, and it would not remove the alert-recreation problem. **[Decision]**
- **Automatic import of the daily levels** from the paid levels provider. The provider is a paid service whose terms likely forbid automated collection; the user copies the levels manually. **[Decision]**
- **Billing / paid plans.** No charges for now. **[Decision]**
- **Automated strategy backtesting.** "Backtest" in this product means analysis of the trader's real trades (see §9). **[Decision]**
- **Automatic sync of account data from brokers/prop firms.** Account data starts as manual entry. **[Decision]**
- **Mobile apps.** Deferred; the web application must work well in mobile browsers. **[Decision]**

## 4. Instruments and platforms

- **Traded instrument:** MNQ (Micro E-mini Nasdaq-100 futures, CME). **[Decision]** The design must allow adding other futures (NQ, ES, MES) later without rework.
- **Analysis references:** the signal logic reads **QQQ and SPY** price action (with NQ/MNQ as the main instrument). They are references for generating signals, not traded instruments. **[Decision]**
- **Web application:** account management, risk configuration, routing/replication configuration, signal configuration, performance analysis. **[Decision]**
- **Supported platforms (initial and planned):**
  - **TradingView** — signal source (Input). **[Decision]**
  - **NinjaTrader 8 plugin (Windows)** — Input (trades placed on a master account) and Output (execution on NinjaTrader accounts), plus live risk enforcement and trade capture. **[Decision]**
  - **Tradovate** — Output (execution on Tradovate accounts), and potentially Input. **[Decision]** Planned for the **second release**; the first release focuses on NinjaTrader, but the architecture must be ready for Tradovate from the start. **[Decision]**
  - More platforms later. Nothing in the Core may be hard-wired to a specific platform. **[Decision]**
- **Technology stack:** as recorded in [ADR-0001](../architecture/adr/0001-technology-stack.md), approved by the user on 2026-10-23. **[Decision]**
- **Hosting:** production in the US central region (near Chicago). **[Decision 2026-10-23]**

## 5. Architecture: Input → Core → Output [Decision]

The system is split into three independent parts. Every platform connects through an **adapter**, so adding a new platform never requires changing the Core.

```
 INPUT (sources)                 CORE                          OUTPUT (executors)
 TradingView indicator ─┐                                   ┌─► NinjaTrader accounts
 NinjaTrader master ────┼─JSON─► validate · apply rules ·  ─┼─► Tradovate accounts
 (future sources) ──────┘        store · route              └─► (future platforms)
                                    ▲                          │
                                    └──── execution results ◄──┘
```

- **Input** receives trade instructions in **one standard JSON format** (§6), whatever the source.
- **Core** ("Bloco") stores all data, holds each account's rules, answers rule queries from the platform adapters, checks every instruction against the target accounts' risk state, and routes it to the right outputs.
- **Output** executes orders on the target accounts and **reports results back to the Core** (accepted, filled at price X, rejected, cancelled). This return path is required for performance analysis, live risk limits, and out-of-sync detection. **[Proposal]**
- **Any source to any destination:** e.g., NinjaTrader master → other NinjaTrader accounts; TradingView → NinjaTrader accounts; TradingView → Tradovate accounts. **[Decision]**
- **Double risk protection:** the Core refuses to route an instruction to an account that has reached a limit, and the platform adapter (e.g., the NinjaTrader plugin) enforces the limits locally as a second barrier, also while the Core is unreachable. **[Decision 2026-10-23]**

## 6. Standard JSON instruction [Decision on principle · Proposal on fields]

**Principle: the JSON instruction states exactly what must happen.** The Core does not infer intent. Whether an opposite signal only closes the position, reverses it, or is ignored is decided by the instruction itself (its `action`), which is decided by the source. **[Decision]**

**Repeated entries add to the position.** If two buy instructions arrive, both are executed and the position grows. **[Decision]**

Draft fields (final contract to be defined by the Software Architect):

```json
{
  "schema_version": "1.0",
  "message_id": "unique id — the Core never executes the same id twice",
  "sent_at": "2026-10-22T14:31:05Z",
  "source": { "type": "tradingview", "name": "strikes-quanticocap", "version": "indicator version" },
  "signal_type": "trigger | trade_plan | nq_trigger | master_account",
  "action": "buy | sell | close | reverse | modify | cancel",
  "instrument": "MNQ",
  "reference_price": 0.0,
  "quantity": 1,
  "details": { "setup": "Setup 6", "zone": "Defense", "trigger": "engulfing" }
}
```

- `quantity` is the base size; each target account applies its own **multiplier** (§8). **[Decision]**
- **Stop and target come from the target account's profile**, not from the instruction. **[Decision]**
- **Authentication:** every source uses its own secret so no one else can send instructions to the user's accounts. For the TradingView webhook the secret travels **in the webhook URL** (not in the message, so the indicator holds no secret), and it is **masked in every log** (proxy, Core, audit). **[Decision 2026-10-23]**
- **Conflicting instructions:** an entry opposite to the current position (e.g., `buy` while short) is rejected for that account and the user is notified; `close` always flattens the whole position; `reverse` while flat opens the position. **[Decision 2026-10-23]**
- `modify`/`cancel` are not accepted from TradingView in release 1; they are used for master-account replication. **[Decision 2026-10-23]**
- `modify`/`cancel` and master-account replication need identifiers that link instructions to the same position/order; the Software Architect defines them. **[Open]**

## 7. Signal input from the TradingView indicator

The user's Pine Script indicator ("NQ/ES Strikes Futuros", QuanticoCap levels) reads QQQ/SPY price action at daily levels and produces three kinds of signals: **triggers** (Setups 1/2/3/4/6), **Trade Plan** activations, and **NQ Triggers**.

### Signal selection
- The user chooses which signal types become orders (triggers, Trade Plan, NQ Triggers), **per account** (e.g., an evaluation account takes only Trade Plan signals while a real account takes all). **[Decision]**

### Daily levels
- The user copies the day's levels manually from a paid provider. **[Decision]**

### Phase 1 — TradingView webhook **[Decision]**
- The indicator's alerts send the standard JSON (§6) to the Input via webhook.
- All alert messages must be standardized to JSON (today only the triggers are JSON; Trade Plan and NQ Trigger alerts are free text). Each message must include its `signal_type` and a unique id.
- Use **one alert per chart** ("any alert() function call") so the daily routine is: paste the levels, delete yesterday's alert, create the new one.
- Known limitation: TradingView alerts keep the indicator version and inputs from when they were created, so the alert must be recreated after each level change or indicator update.

### Phase 2 — Platform signal engine **[Decision]**
- The indicator's logic is reimplemented as the platform's own signal engine. The daily levels are pasted into a screen in the web application (keeping the current text formats). No TradingView alerts are needed; TradingView remains only for visual reference.
- Not part of release 1 (depends on the market-data decision below). **[Decision 2026-10-23]**
- The platform engine becomes the official version of the logic. The Pine indicator's backtest serves as a reference: both must produce the same signals on the same days before the engine goes live. **[Proposal]**
- Requires licensed market data for **MNQ and QQQ/SPY (including extended hours)**. Candidate provider: Databento (futures and equities). Cost and licensing must be confirmed first. **[Open]**
- Because Input is an adapter, moving from Phase 1 to Phase 2 does not change the Core or Output.

## 8. Order routing and replication

### Sources and destinations
- Instructions can come from a TradingView signal or from trades placed on a **master account** in NinjaTrader (and later other platforms). **[Decision]**
- Destinations are the user's own accounts on NinjaTrader, Tradovate, and later other platforms. **[Decision]**
- Accounts can be spread across **more than one NinjaTrader instance**. **[Decision]**
  - All involved instances must be open and connected. A local fast path inside one instance is not built in release 1; it may be revisited with measured latency. **[Decision 2026-10-23]**
- Only accounts owned by the same user. **No artificial delay or randomization.** **[Decision]**
- The user can **enable, disable, or pause each destination account** at any time. **[Decision]**

### Master-account replication
- **Everything done on the master** is replicated: entries, exits, all order types, stop and target modifications, cancellations. **[Decision]**
- Each follower's **initial** stop and target come from the follower's own profile; when the user moves the stop or target on the master, followers move theirs by the same amount. **[Decision; initial values 2026-10-23]**
- In release 1, replication always relays through the Core, also when master and followers share one NinjaTrader instance. **[Decision 2026-10-23]** Master replication ships as V1.1, after the TradingView path. **[Decision 2026-10-23]**

### Size, stops, and targets
- **Contracts:** each destination account has its own **multiplier** set by the user. **[Decision]**
  - Multipliers are **whole numbers only** (1x, 2x, 3x, …); fractional multipliers are not allowed, so no rounding is ever needed. **[Decision]**
- **Stop and target distances** come from each account's profile. **[Decision]**
- **Defaults by account size:** the user defines default stop/target profiles for **25k, 50k, 100k, 150k, 200k, and 300k** accounts, even for sizes they don't currently have. A new account starts with its size's profile and can be customized. **[Decision]**
- **Units:** points, ticks, or dollars, chosen by the user per profile. **[Decision]** Profiles are keyed by instrument; dollar distances that are not whole ticks are rounded toward the entry. **[Decision 2026-10-23]**
- Release 1 has one maximum position size per account (MNQ). **[Decision 2026-10-23]**
- **Stop/target mode when entries add to a position** — user setting per account: **[Decision]**
  - **Individual (default):** each entry has its own stop and target, calculated from its own entry price.
  - **Shared:** the whole position uses one stop and one target, recalculated from the average price.
- **Orphan-order protection (both modes):** whenever a position becomes flat for any reason (targets/stops hit, manual close, enforcement), all remaining stop and target orders for it are cancelled immediately. **[Decision]**
- **Maximum position size (contracts)** — mandatory on every account, never unlimited: **[Decision]**
  - Prop firm accounts default to the firm's maximum for that account size (from the preset); the user can lower it but not exceed it.
  - Real accounts: the user sets it at registration.
  - If an instruction would exceed the maximum, only the quantity up to the maximum is executed; the excess is skipped, logged, and the user is notified.

### Risk interaction
- When a destination account hits its own limit (with enforcement on), it is flattened and removed from routing until the next trading day; other accounts continue. **[Decision]**
- When the **master** account hits its limit: only the master is flattened and blocked. Followers keep their open positions (each is protected by its own rules) but receive no new instructions from that master until the next trading day. **[Decision]**

### Reliability requirements [Decision 2026-10-23, plan approval]
- Never execute the same instruction twice (retries, reconnections, reloads) — enforced by `message_id`.
- Detect and clearly show when an account is out of sync (missed, rejected, or partially filled order; disconnected instance) and never silently diverge.
- When a destination disconnects mid-trade and reconnects: **alert the user and wait** for their decision; never automatically reopen or resync positions. **[Decision]**
- An account flagged out of sync receives no new instructions until the user clears the flag. **[Decision 2026-10-23]**
- **Command expiry:** an instruction that cannot reach an account within its expiry time is never executed late; it is reported as missed. The expiry is editable per account, default short (a few seconds). **[Decision 2026-10-23]**
- Log every instruction and every execution result for audit and support. The activity record is kept 24 months rolling (configurable); trades and configuration for the life of the account; all data erased 30 days after the user closes the account. **[Decision 2026-10-23]**
- Exits are never blocked by risk or out-of-sync gates (only pause/disable block them); only entries expire (default 5 s, editable 1–60 s per account). **[Decision 2026-10-23]**
- On NinjaTrader and Rithmic connections the stop/target pairing (OCO) lives on the user's PC, so orphan-order protection needs NinjaTrader running; the product shows this limit and checks brackets on reconnection. **[Decision 2026-10-23]**
- If both bracket legs filled while NinjaTrader was down, leaving an unwanted opposite position, only the position created by the orphan leg is flattened automatically, with maximum alert and audit; nothing else is corrected automatically. **[Decision 2026-10-23]**

## 9. Account & risk management

### Accounts
- Manual registration. Types: **prop firm account** (firm, plan, size, phase) and **real brokerage account**, on any supported platform. **[Decision]**
- Real accounts may have any size; they start from the default profile of the nearest standard size. **Pause** = temporary stop keeping all configuration; **disable** = off until re-enabled; both leave open positions untouched. **[Decision 2026-10-23]**

### Rules per account
- Drawdown (with its type, e.g., trailing intraday, end-of-day trailing, static), daily profit target, daily loss limit, and firm-specific rules such as consistency rules. **[Decision]**
- **Prop firm presets.** Presets are versioned, dated, sourced from official rules, and editable by the user. **[Proposal]** Presets are never invented. **[Decision]**
  - **All of the following firms ship in the first release** (user's priority firms listed first): **[Decision]**
    - Apex Trader Funding, Phidias Propfirm (https://phidiaspropfirm.com/), Lucid Trading.
    - Topstep, Take Profit Trader, MyFundedFutures, Tradeify, Bulenox, TradeDay, Top One Futures, Alpha Futures.
    - Elite Trader Funding, Earn2Trade, Funded Futures Family, Phoenix Trader Funding, OneUp Trader, BluSky, Leeloo Trading.
  - Each firm's rules — including its stance on trade copying and automated trading — must be verified on its official rules pages before its preset is published. Presets show each stance with its source and date, and store the firm's **platform/connection** (e.g., Rithmic, Tradovate) because some rules differ by platform. **[Decision 2026-10-23]**
  - Items that conflict across a firm's official pages ship as **"not confirmed"** (not pre-filled; the user picks a published value or types one) and do not block the rest of the preset; the conflict list goes to the user for a later decision. **[Decision 2026-10-23]**
  - **Firms that forbid automation (option A):** when registering or editing such an account, the user must explicitly choose whether it accepts automated sources — no default — after a warning with the firm's rule text and the risk of losing the account. The choice is recorded in the audit with date and user; the risk is accepted by the user per account. **[Decision 2026-10-23]**
  - The same explicit choice is required for any automation stance other than "allowed" (conditional, unknown, not confirmed). Followers of firms whose copying stance is not "allowed" need a separate explicit **copy opt-in** for master replication. A new preset version that changes a stance suspends the affected automated sources or replication until the user confirms again. **[Decision 2026-10-23]**
  - Apex Legacy and Phidias LIVE programs are not offered as presets in release 1 (registered manually). **[Decision 2026-10-23]**
  - **Topstep** (TopstepX/ProjectX only; orders must originate from the trader's personal device, VPS forbidden) ships in release 1 as a **manual-tracking-only** preset, with no routing and the limit explained; an adapter on the user's device is only for later evaluation. Topstep Labs Challenges/Pro Account and Take Profit Trader PRO+/PRO+ Development are not offered as presets. Take Profit Trader's copying stance is conditional (approved copiers only; opt-in; the user asks the firm). Presets record each firm's VPS/server and personal-device rules. **[Decision 2026-10-23]** Firms change rules often; rules are never guessed. If a firm's rules cannot be verified, the team reports it to the user, who decides whether to delay that preset or release without it.

### Automatic enforcement (optional)
- Off by default; the user enables it per account. **[Decision]**
- The platform adapter (e.g., NinjaTrader plugin) retrieves limits from the Core and monitors live P&L. On a limit: flatten positions, cancel working orders, block trading on that account for the rest of the trading day. **[Decision]**
- **Trading-day reset:** each prop firm's own reset time (stored in its preset); real accounts use the CME futures session close. **[Decision]**
- **No early unlock:** a blocked account stays blocked until the reset. **[Decision]**
- **Warnings at 80%** of each limit, shown in the web application and in the platform adapter (e.g., NinjaTrader). **[Decision]**
- **Daily profit target:** per account, "notify only" (default) or "flatten and block". **[Decision 2026-10-23]**
- **Consistency rules:** tracked and warned in release 1, not enforced. **[Decision 2026-10-23]**
- **Core gate with enforcement off:** the Core still refuses new instructions to an account that has reached a limit (it never closes trades). **[Decision 2026-10-23]**
- In NinjaTrader the block is reactive: orders placed manually on a blocked account are cancelled/flattened right after submission. **[Decision 2026-10-23, plan approval]**
- If a firm forbids copying or automation, its preset still ships with the stance clearly shown. **[Decision 2026-10-23]**

## 10. Performance analysis

- Trades are captured automatically from execution results (Output) or entered manually. **[Decision]**
- Per trade: account, instrument, direction, quantity, entry and exit time and price, **time in trade**, P&L, **maximum adverse excursion**, maximum favorable excursion, and the originating signal/source. **[Decision]**
- **Phase 1:** metrics computed by the platform adapter (e.g., in NinjaTrader with the user's own data); only computed numbers are sent. **[Decision]**
- **Phase 2:** trade-on-chart views with licensed data (same provider decision as §7 Phase 2). The platform must not store or redistribute price data from the user's own data feeds. **[Decision]**

## 11. Cross-cutting requirements

- **Safety during development:** agents develop and test only with simulation, demo, or playback accounts. Live-account use is done exclusively by the user.
- **Precision:** prices, P&L, and money use exact numeric representations.
- **Security and privacy:** strict per-user isolation; secrets and credentials never stored in plain text or logs.
- **Transparency:** every automatic action (route, skip, flatten, block) is visible to the user with its reason.

## 12. Compliance notes (to verify with qualified professionals)

- Prop firms have rules about automated trading and trade copying; many allow replication only across accounts of the same person and some restrict automation. Presets record each firm's stance, shown to the user. **[Proposal]**
- TradingView terms of use apply to the webhook integration; no scraping or interface automation.
- CME and equity market data licensing (affects Phase 2).
- Terms of use and privacy policy are required before launch. **[Open]**
- This document is not legal advice.

## 13. Open questions (summary)

1. ~~Technology stack~~ — resolved by ADR-0001 (2026-10-23).
2. Identifiers for `modify`/`cancel` and master-account replication — Software Architect (ADR-0003, pending).
3. Market data provider, cost, and licensing for Phase 2 — research before Phase 2.
4. Terms of use and privacy policy before launch — user, with qualified professionals.

## 14. Decision log (2026-10-22)

- Focus on MNQ; QQQ and SPY are analysis references for the signal logic.
- Accounts entered manually; prop firm presets wanted; optional automatic flatten-and-block.
- "Backtest" = analysis of real trades.
- Copy trade between different people removed; routing/replication only across the user's own accounts, without artificial delay.
- Architecture: Input → Core → Output with a standard JSON; any source to any destination (NinjaTrader, TradingView, Tradovate, future platforms).
- The JSON instruction decides the behavior (open, close, reverse); repeated entries add to the position.
- Stop and target always come from the account profile (defaults by account size 25k–300k; points, ticks, or dollars); individual per entry by default, shared as an option; orphan orders always cancelled.
- Mandatory maximum position size per account (prop default from preset); excess quantity is skipped and reported.
- User selects which signal types become orders.
- TradingView input in two phases: webhook first, own signal engine later; no browser extension.
- Daily levels copied manually from the paid provider.
- Bilingual interface (pt-BR and en); no billing for now; mobile apps deferred.
- Product name: **All Win**.
- Whole-number multipliers only; master hitting its limit blocks only the master; disconnects alert and wait for the user.
- Trading day resets per firm (CME session close for real accounts); no early unlock; warnings at 80%.
- Tradovate output in the second release; signal types selected per account.
- Prop presets: all 18 listed firms in the first release (Apex, Phidias, Lucid as priority).

## 15. Decision log (2026-10-23)

- V1 slice plan approved ([release-1/slice-plan.md](release-1/slice-plan.md)); all recommendations Q1–Q16 and A1–A6 accepted ([delivery decision log](../delivery/decisions.md)).
- Stack per ADR-0001; production in the US central region; webhook secret in the URL, masked in every log; replication always via the Core in V1; optional TOTP; invite-only sign-up.
- Conflicting instructions rejected; `close` flattens all; no `modify`/`cancel` from TradingView in V1; followers' initial stop/target from their own profile; master replication as V1.1.
- Profit target per account (notify only by default); consistency rules tracked only; Core gate refuses new instructions at limit even with enforcement off; out-of-sync accounts stop receiving until cleared; short command expiry, editable per account.
- In-app notifications only; plugin follows web language; real accounts of any size; pause vs disable defined; firms forbidding copying still ship with stance shown.
- The user shares the Pine indicator source; the team proposes the change.
- Still open: audit retention (after Database Architect input), exact default expiry seconds, Windows + NinjaTrader machine setup (Architect options first), and design follow-ups Q17–Q23 in the slice plan.

## 16. Decision log (2026-10-23, second batch)

- Firms that forbid automation: explicit per-account opt-in, no default, warning with the firm's rule text, audited (option A). Master replication stays V1.1.
- Presets: stance with source and date; conflicting items ship "not confirmed"; platform stored.
- ADR-0003/0004 accepted (P1–P5, T1–T3); audit retention 24 months; one max position per account; profiles keyed by instrument; signal filter per (source, account); several named TradingView sources; invite/e-mail/session/TOTP/locale rules per slice-plan Q17–Q23.
- User-owned pending: review NinjaTrader terms (ToS §5(a)(viii), EULA) before offering the product to other people.
- Third batch: Q24–Q27 accepted (copy opt-in, opt-in for any non-"allowed" stance, suspension on stance change, auto-flatten only the orphan-leg position); R1 conflict recommendations accepted with Apex Legacy and Phidias LIVE excluded, the 31 conflicts reviewed later ("not confirmed" until then).
- Fourth batch: Topstep manual-only preset; TPT conditional copying; Topstep Labs/Pro and TPT PRO+ excluded; VPS/personal-device rules recorded per firm.
- Open: remaining R2 items ([presets/r2-conflicts-for-user.md](presets/r2-conflicts-for-user.md)).
