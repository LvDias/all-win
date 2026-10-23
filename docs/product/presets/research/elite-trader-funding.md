# Elite Trader Funding — Prop Firm Rules Research

| | |
|---|---|
| **Firm** | Elite Trader Funding ("ETF"; futures only; simulated accounts, rewards paid from sim profits, LIVE ELITE real capital by invitation) |
| **Official domains** | `elitetraderfunding.app` (the site, the Help Center under `/help`, ToS and legal pages; plain curl works but Vercel rate-limits with a challenge page after about 50 rapid requests, waiting about 5 minutes clears it) · `elitetraderfunding.com` (marketing name; blocks curl with a Cloudflare 403, in the browser portal it shows the same plans; `https://help.elitetraderfunding.com` answers HTTP 301 to `https://elitetraderfunding.app/help`) |
| **Research date** | 2026-10-23 (all URLs below accessed 2026-10-23) |
| **Researcher** | Lupa (Prop Firm Rules Researcher) |
| **File status** | `partial`. Verified: plan tables for 1 Step 50K/100K/250K, End of Day 50K/100K, Static 25K/50K, Diamond Hands 100K, Direct to Funded 10K/50K/100K, commissions, automation/copier/VPS stance. Unpublished or conflicting: 150K drawdown/target/DLL, trading-day reset time, personal-device rule, news rule (funded), VPS wording, 250K 1 Step availability, 25K DTF existence |
| **Instruments covered** | MNQ, NQ (CME). Futures contract specs page lists NQ (tick 0.25, $5, micro MNQ 1/10) (S34); commissions listed for both (S18). Supported exchanges: CBOT, CME, COMEX, NYMEX |
| **Platforms (for routing)** | Account is traded through a third-party platform chosen by the data feed bought: **Tradovate** (web/desktop/mobile; also connects to **NinjaTrader** and **TradingView**), **Rithmic** (R\|Trader Pro, **NinjaTrader**, Quantower, Sierra Chart, Bookmap, ATAS, MotiveWave, eSignal, EdgeProX, Finamark, Investor/RT) or **Tickblaze**. **NinjaTrader 8 is supported on Windows** with either Tradovate (CQG) or Rithmic credentials. See §2 |

Evidence (raw page text of every cited page, URL on line 1 of the HTML capture; ToS 120 KB): `/tmp/claude-1003/-srv-www-allwin--maestri-roles-6bf77a80-a1ed-43dc-bc00-08a480f178a5/c3ed71e7-3dd4-4f1a-8a03-9bc3a3fa7b49/scratchpad/r3/elite-trader-funding/pg/`. Scratch storage, so re-capture at re-verification. Every verbatim quote below was grepped in that captured text.

**Programs found (current).** Six evaluations plus Direct to Funded: **1 Step** (live trailing, 50K/100K/150K, 250K listed but "no longer sold" in one place), **End of Day** (50K/100K/150K), **Static Drawdown** (10K/25K/50K), **Diamond Hands** (100K only), **Fast Track** (10K only, EOD or Static), **Direct to Funded (DTF)** (10K Mini Inferno/50K/100K). Passing gives an **Elite Sim-Funded** account (simulated, up to 100% profit split, $25,000 total reward cap per trader, max 20 active). **LIVE ELITE** is real capital by invitation only. Legacy articles exist for the Legacy End of Day, Legacy Diamond Hands and Legacy Fast Track plans (not purchasable; only noted). Homepage headline: "Get Funded to Trade Futures with Accounts up to $150K" (S32 says 1 Step sizes "50K to 250K"; conflicting size statements, see §10).

**ETF's terms.** "ATD" = Active Trade Day. "Reward" = what ETF used to call a payout. "Safety net" = realized profit equal to max drawdown + $100. "DTF" = Direct to Funded. "ODTP/One Day to Pass" = add-on that lifts the 5-day minimum to 1.

### 1. Source index (all accessed 2026-10-23)

| Key | URL |
|---|---|
| S1 | https://elitetraderfunding.app/help/trade-copier-disclaimer (titled "Trade Copier, EA & Algo Trading Disclaimer") |
| S2 | https://elitetraderfunding.app/terms-of-service (Last updated October 11, 2026; effective October 8, 2026) |
| S3 | https://elitetraderfunding.app/help/vpn-and-ip-masking-policy |
| S4 | https://elitetraderfunding.app/help/contract-limits |
| S5 | https://elitetraderfunding.app/1-step-evaluation |
| S6 | https://elitetraderfunding.app/static-drawdown |
| S7 | https://elitetraderfunding.app/end-of-day-evaluation |
| S8 | https://elitetraderfunding.app/diamond-hands-evaluation |
| S9 | https://elitetraderfunding.app/fast-track-evaluation |
| S10 | https://elitetraderfunding.app/direct-to-funded |
| S11 | https://elitetraderfunding.app/help/how-the-1-step-plan-works |
| S12 | https://elitetraderfunding.app/help/how-the-end-of-day-plan-works |
| S13 | https://elitetraderfunding.app/help/how-the-revamped-diamond-hands-plan-works |
| S14 | https://elitetraderfunding.app/help/how-the-static-account-plan-works |
| S15 | https://elitetraderfunding.app/help/how-the-dtf-plan-works |
| S16 | https://elitetraderfunding.app/help/how-the-revamped-fast-track-plan-works |
| S17 | https://elitetraderfunding.app/help/consistency-rule-and-active-trade-days-atd-explained |
| S18 | https://elitetraderfunding.app/help/guide-to-our-commissions-and-exchanges-we-support |
| S19 | https://elitetraderfunding.app/help/supported-platforms-and-data-feed |
| S20 | https://elitetraderfunding.app/help/connection-guide |
| S21 | https://elitetraderfunding.app/help/ninjatrader-connection-guide |
| S22 | https://elitetraderfunding.app/help/tradovate-connection-guide |
| S23 | https://elitetraderfunding.app/help/max-active-elite-sim-funded-accounts |
| S24 | https://elitetraderfunding.app/help/policy-hedging-against-yourself-using-the-same-or-correlated-instrument-on-one-or-multiple-accounts |
| S25 | https://elitetraderfunding.app/help/policy-trading-for-another-user |
| S26 | https://elitetraderfunding.app/help/trading-major-economic-releases |
| S27 | https://elitetraderfunding.app/help/routine-maintenance-every-weekday-between-4-5-pm-cst |
| S28 | https://elitetraderfunding.app/help/safety-net-requirements |
| S29 | https://elitetraderfunding.app/live-elite |
| S30 | https://elitetraderfunding.app/help/live-elite-information |
| S31 | https://elitetraderfunding.app/trade-copier-disclaimer (legal-page version, dated "Last updated: May 19, 2026") |
| S32 | https://elitetraderfunding.app/evaluations |
| S33 | https://elitetraderfunding.app/elite |
| S34 | https://elitetraderfunding.app/futures-contract-specs |
| S35 | https://elitetraderfunding.app/help/tradovates-group-trading |
| S36 | https://elitetraderfunding.app/help/how-to-setup-bracket-orders-on-tradovate |
| S37 | https://elitetraderfunding.app/ (homepage: plan comparison, "Max Accounts 20") |
| S38 | https://elitetraderfunding.app/help/account-closure-for-non-productive-trading-activity |
| S39 | https://elitetraderfunding.app/help/correlated-and-substantially-similar-instruments |

### 2. Platforms

ETF provides no web trading platform: "Every Elite Trader Funding (ETF) account is traded through a third-party platform, not on our website." (S20). The data feed bought decides the platforms (S20):

| Feed | Platforms (as published) | NinjaTrader 8 |
|---|---|---|
| Tradovate | Tradovate web, Windows, Mac, iOS, Android; TradingView ("TradingView access must be selected at checkout"); NinjaTrader. "Tradovate accounts connect only to the NinjaTrader and Tradingview platforms." (S22) | Yes: "For Tradovate (CQG) credentials, select NinjaTrader and click Add." (S21) |
| Rithmic | R\|Trader Pro (Windows only), NinjaTrader, Quantower, ATAS, Bookmap, EdgeProX, eSignal, Finamark, Investor/RT, MotiveWave, Sierra Chart (S19, S20). Rithmic is not available on macOS | Yes: "For Rithmic credentials, select Rithmic for NinjaTrader Brokerage. Under System, choose Rithmic Paper Trading (Chicago)." (S21) |
| Tickblaze | Self-contained desktop platform with its own data and broker connections | Not stated |

Notes: NinjaTrader is "Windows only" (S19). Since January 28, 2026, new accounts that select a Rithmic data feed "incur an additional separate monthly connection cost, per active Rithmic account" (S19). The platform list is published for all plans; no plan or phase restriction was found (LIVE ELITE broker/platform is not stated, see §10). Bracket orders are documented for Tradovate (S36).

### 3. Plans

All values below are as published on the product pages (S5-S10) and the plan help articles (S11-S16). Per-plan common facts: minimum 5 trading days to pass (Fast Track 3; the One Day to Pass add-on lifts it to 1), eval reset fee $47 with unlimited resets (Fast Track: no resets), evaluations are monthly subscriptions with no time limit (except Fast Track: 10 calendar days), funded activation $87/month or one-time, positions must be flat 1 minute before the instrument's close (except Diamond Hands and DTF), "Trade How You Want" promotion in force (S17, see §7). Position limits (S4): "1 mini = 1 position; 10 micro = 1 position." (S13); S4 adds that minis and micros are enforced independently by the platform but stacking both "We do not consider this acceptable use of the account."

**Consistency rule.** "Consistency rule (Revamped Fast Track only): your single best day cannot exceed a set percentage of your total profit (40% to qualify, 30% at reward). No other plan caps your best day." (S17). On other plans the best-day percentage only decides whether a day counts as an ATD (payout-cycle qualifier): "a day counts as an ATD when it earns at least $250 realized profit AND at least 30% of your best day." (S17, evaluations bought on/after 2026-10-02 2:00 PM PT; earlier ones 23% and $200).

**Drawdown lock.** 1 Step: "It stops trailing once you earn realized profit equal to the max drawdown + $100; from then on, your starting balance + $100 is your minimum balance." (S4). EOD/Diamond Hands/DTF-EOD: trails the highest end-of-day balance ("the drawdown trails your highest end-of-day balance", S12) until realized profit equals max drawdown + $100 (S12, S13, S15). In evaluations there is no lock ("There is no buffer or safety net during the evaluation", S11).

#### 3.1 Evaluation summary (sizes 25K and above, plus 10K for completeness)

| Plan | Size | Drawdown (type) | Daily loss limit | Profit target | Max position | Min days | Source |
|---|---|---|---|---|---|---|---|
| 1 Step | 50K | $2,000 live trailing | none | $3,000 | 8 minis / 80 micros | 5 | S5 |
| 1 Step | 100K | $3,000 live trailing | none | $6,000 | 14 / 140 | 5 | S5 |
| 1 Step | 150K | not published | none | not published | 18 / 180 (S4) | 5 | S4, S32 |
| 1 Step | 250K | $6,500 live trailing | none | $15,000 | 24 / 240 | 5 | S5 (S4: "no longer sold") |
| End of Day | 50K | $2,000 EOD trailing | $1,100 hard | $3,000 | 8 / 80 | 5 | S7 |
| End of Day | 100K | $3,500 EOD trailing | $2,200 hard | $6,000 | 14 / 140 | 5 | S7 |
| End of Day | 150K | not published | not published | not published | 18 / 180 (S4) | 5 | S4, S32 |
| Static | 25K | $1,000 static | none | $2,000 | 2 / 20 | 5 | S6 |
| Static | 50K | $2,000 static | none | $4,000 | 4 / 40 | 5 | S6 |
| Static (low) | 10K | $500 static | none | $1,000 | 1 / 10 | 5 | S6 |
| Diamond Hands | 100K | $3,500 EOD trailing | $1,500 hard | $5,000 | 2 / 20 | 5 | S8 |
| Fast Track (low) | 10K EOD | $500 EOD trailing | none | $2,000 | 1 / 10 | 3 | S9 |
| Fast Track (low) | 10K Static | $500 static | none | $2,000 | 1 / 10 | 3 | S9 |

The EOD page says its daily loss limit is "a hard-breach daily loss limit" (S7); S12: "You also have a daily loss limit calculated from the prior day’s ending balance." and "Losses from open positions count intraday." The evaluations page (S32) shows the daily loss limit as "Yes ($1,100 to $2,200 by size)" for 50K, 100K, 150K, while the 150K amounts are not on any page read.

#### 3.2 Funded (Elite Sim-Funded) summary

Same drawdown amount and contract limits as the evaluation continue after funding (S4: "These limits apply for the life of your account. They do not change when you pass your evaluation"). No profit target. Reward cycles need 8 ATDs (cycle 1) then 10 (cycles 2-4) plus the safety net; reward cap $25,000 total per trader (S33). The loss limit rule: "If you lose more than 35% of total accumulated profit after reaching 20% growth, the account will be removed from the Elite Sim-Funded program." (S12). Funded daily loss limit on EOD/Diamond Hands is removed once the safety net is reached (S12). Weekly trade: "Traders must execute at least one trade per week to keep their account active." (S12). Max active funded accounts: "Traders can hold up to 20 active Elite Sim-Funded accounts" (S23; up to 5 of them DTF).

Direct to Funded (S10, S15): no evaluation, starts as Elite Sim-Funded, one-time price; "DTF accounts have no profit target" (S15; the S10 table label "Profit target $1,600/$5,100/$5,100" is the safety net). 10K Mini Inferno: $1,500 EOD trailing, max 2 minis or 2 micros; 50K: $5,000 EOD trailing, 5 or 5; 100K: $5,000 fixed, 3 or 3; no daily loss limit; "Overnights/weekends allowed — swing trading permitted." (S15).

#### 3.3 LIVE ELITE (live phase)

Not purchasable. "LIVE ELITE is earned, not purchased." (S29). Real capital onboarding balances $1,250 to $2,500 (S30), 80/20 split, $250 minimum withdrawal, end-of-day drawdown, "swing trading is permitted at ETF's discretion" (S29). Daily loss limit, position limits, platform and automation rules for LIVE ELITE are not stated separately. One placeholder block below.

### 4. Trading-day reset

No page states a "trading day reset" time. Published fragments only:
- "Sunday evening and Monday are counted as the same trading day." (S12).
- "Trading platforms will be unavailable between 4:00 PM and 5:00 PM CST on weekdays." (S27); S34: "Electronic session for all: Sun 5:00 PM – Fri 4:00 PM CT (daily maintenance break 4:00–5:00 PM CT)".
- ToS definition used for the Martingale rule: "“Trading Session” means the CME trading day beginning at 6:00 p.m. ET and ending at 5:00 p.m. ET on the following calendar day;" (S2). The ToS "Trading Day" used for the flatten rule is not defined there.
- EOD floor "recalculates once" at "session close" (S7, S12); the exact time of that close and the daily-loss-limit reference time ("prior day’s ending balance", S12) are not defined. Holiday rule: "Can I trade on holidays and during news? Yes, but volatility can be very low or extremely high." (S12). **time = unknown (gap).**

### 5. Trade copying stance

Verbatim (S1, help article titled "Trade Copier, EA & Algo Trading Disclaimer"): "If you choose to use a trade copier, AI, bot, EA, algo, or other automated trading tool, you do so entirely at your own risk. The decision and responsibility to use such tools rest solely with the user. AI, bots, EAs, and algos additionally require ETF's written authorization before use."

Approved copiers (S1): Tradesyncer (cloud, fully approved), Tradecopia (cloud, fully approved), Affordable Indicators "Duplicate Account Actions" (fully approved), Replikanto Flowbot "PropFirm Compliance Edition" (compliance version only), and "Platform Built-in Features — Any trade copying functionality native to your trading platform (examples: group trading via Tradovate, trade copier functionality in Motivewave, Quantower)". Anything else: "Your account might be reset, Any profits might be forfeited, and Repeat violations will result in account failure and a permanent ban from our services." Enforcement "will occur at the reward stage" and will be automated.

- **Same-person copying across own accounts:** allowed only with approved copiers: "Only trade copiers on ETF's approved list can be used." (S23, also S24, S35). Up to 20 active funded accounts (S23).
- **Account-count limits:** 20 active Elite Sim-Funded (max 5 DTF); one registration per user (ToS); household: "Traders from the same household are not allowed to participate at ETF under any circumstances." (S12).
- **Copying with other firms:** not stated on any page read (gap).
- **Hedging:** "Traders cannot open opposite positions on the same instrument, or correlated instrument (e.g. NQ and MNQ) on two or more accounts owned or associated with them" (S24).
- **Trading for others / copying someone else's signals into your account:** "Trading for another user, whether within the same household or in remote locations, is strictly prohibited. Each user must conduct their own trading activity." (S25).
- **Two versions of the disclaimer exist:** S31 (legal page, "Last updated: May 19, 2026") says "Elite Trader Funding permits trade copiers from approved partners only" and does not mention AI/bots/EAs; S1 (Help Center) adds the written-authorization requirement for AI, bots, EAs and algos. Both agree on the approved copier list. ToS (ac) matches S1.

### 6. Automated trading stance

(i) **Classification: `conditional`.** Everything automated is forbidden unless authorized in writing by ETF; the only currently authorized tools are the approved copiers. Applies to all phases (evaluation, funded, DTF); LIVE ELITE not separately stated.

`automation_rule_text` (verbatim, S1): "Written authorization is required. Per our Terms of Service, you may not use artificial intelligence (AI), bots, automated trading systems, or other automated trading strategies unless expressly authorized in writing by ETF. This includes machine learning, deep learning, neural networks, generative models, natural language processing, and any other automated decision-making system, whether built in-house or provided by a third party. The only automated trading tools currently authorized are the approved trade copiers listed above. An EA, AI tool, or algorithmic trading system that isn't one of those is not authorized for use on your ETF account."

ToS (S2) prohibited conduct (ac), verbatim: "makes use of artificial intelligence (AI), bots, automated trading systems, trade copiers, or other automated trading strategies that are not expressly authorized in writing by ETF. Approved automated trading programs may be found at https://elitetraderfunding.app/help/trade-copier-disclaimer ." Also: "Automated trading that is not expressly authorized in writing by ETF under Section (ac) does not count as Genuine Activity. Trades placed by a bot, script, or any automated system that is not so authorized are not allowed and do not qualify as Genuine Activity, even when the orders look real." (S2). ToS (ah): "Engage in trading strategies that utilize high-frequency trading (HFT) ... in an abusive manner" (the 10-second minimum is suspended by the "Trade How You Want" promotion, S17).

(ii) **Named mechanisms:**

| Mechanism | Stance |
|---|---|
| Semi-automated trading (manual entries, automation assists) | not mentioned. Under the wording above, any "automated trading system" or "script" needs written authorization; where the boundary of "semi-automated" lies is not defined |
| Trade copiers | mentioned: allowed only if approved (list above); "Only trade copiers on ETF's approved list can be used." (S23) |
| Webhooks / alerts (e.g. TradingView alerts to a broker) | not mentioned |
| ATM / auto brackets | not mentioned as a restriction. Bracket orders are documented as a normal Tradovate feature ("Bracket orders in Tradovate allow you to manage risk and potential gains by setting predefined stop loss and profit target levels.", S36); platform-side stops are not "automation" by any published text |
| EA / algo / AI / bots | mentioned: need written authorization (S1, S2) |

(iii) **By phase:** no difference found between evaluation, Elite Sim-Funded and DTF. The enforcement note says checks happen "at the reward stage" (user-session audit). LIVE ELITE: not stated.

### 7. Device / hosting rules and other rules

**Device / hosting (format rev. 3 item 7).**
- **VPS, VPN, proxy: forbidden** (ToS prohibits users who) "utilize virtual private networks (VPNs), The Onion Router (TOR), virtual private servers (VPS), proxy servers, or any other form of technology or technologies in tandem that in any way hinders our ability to ascertain your IP address, MAC address, or any other identifying information about your device" (S2, clause (l)). Help article (S3): "Connecting to your ETF account through a VPN, VPS, proxy, or any service that hides, changes, or spoofs your real IP address or physical location." is prohibited; "The sole exception is an ETF-approved partner service operating within an approved IP range." (S3, e.g. an approved cloud copier). Penalty: "account reset, forfeiture of profits, account failure, and a permanent ban" (S3).
- **Wording inconsistency:** the copier disclaimer (S1) says "We will continue to reward skilled traders who genuinely trade their ETF accounts and use VPNs or VPS for legitimate and benign reasons." This reads as tolerance, but no page withdraws the ToS/S3 prohibition. Recorded as a gap in §10; blocks use `vps_allowed: no` with `vps_allowed` listed in `not_confirmed_fields`.
- **Personal device / orders from own device:** no explicit rule found. Related: "You represent and warrant that you will not share your Account information, User ID, or password with anyone" (S2); remote trading for another user is prohibited (S25); ETF "reserves the right to audit ... IP address logs and device fingerprints" (S2). `personal_device_required: unknown` (gap).
- **Mac/Windows:** NinjaTrader and several Rithmic platforms are Windows only (S19).

**Other rules that affect routing or risk.**
- **News:** CONFLICT. Help article S26: "ETF does not impose any restrictions or limitations on traders during major economic news events." ToS (S2): "all Elite Accounts must be flat 5 minutes before the event to 5 minutes after the event starts. Traders still in evaluation phase do not have this rule." (list of events is on a linked page not read). ToS ranks above help articles ("Order of Precedence", S2).
- **Mandatory flatten:** "No. All trades must be closed one minute before market close. Times vary by instrument." (S12). Exceptions: Diamond Hands and DTF: "Yes. Only Diamond Hands and Direct To Funded accounts are able to hold trades through close, and the weekend." (S13). LIVE ELITE: swing "at ETF's discretion" (S29). Overnight margin charges exist for funded holds (help article "Overnight Margin: Responsibilities and Charges", not read).
- **Mandatory stop:** none found (the daily loss limit / drawdown act as risk limits). Order types and brackets are the trader's choice.
- **Hedging:** prohibited across own accounts and correlated instruments (S24).
- **Martingale / DCA:** ToS (aj) prohibits; the "Trade How You Want" promotion currently allows them: "Martingale and adding to a losing position (DCA, averaging down) are allowed." (S17). Promotion "stays in effect until Elite Trader Funding announces it has ended".
- **Min trade duration / scalping:** ToS (ah) says 10 seconds; promotion: "the 10-second minimum trade duration does not apply. There is no minimum hold time and no maximum number of trades, and scalping is allowed." (S17).
- **Max accounts:** 20 active funded (5 DTF); one registration per user; household rule.
- **Inactivity:** one genuine trade per week per account or the account is closed and rewards voided (S12, S2 "scratch trade placed only to reset the weekly clock does not satisfy"); closure for prolonged non-productive activity (S38).
- **Contract month:** "You must trade the most current contract with the highest volume." (S12).
- **Max position breach:** evaluation fails; funded: profit forfeited and account closed (S12).
- **Scaling:** no scaling plan found (limits fixed per size, S4).
- **Time limit:** none except Fast Track (10 calendar days).
- **Reset policy:** Elite Sim-Funded resets up to ten times (S12).

### 8. Commissions

Published by ETF (S18): "the commissions in the Supported Instruments table are per side. A round turn (entering and exiting one contract) is double the per-side commission."

| Symbol | Per side | Round turn |
|---|---|---|
| NQ | $2.04 | $4.08 |
| MNQ | $0.51 | $1.02 |
| ES | $2.04 | $4.08 |
| MES | $0.56 | $1.12 |

Basis: `per_side`. Whether the figure includes exchange, NFA and platform fees is not stated (gap). Platform and data-feed subscriptions are the trader's responsibility (S19); Rithmic feeds add a monthly connection cost per account since 2026-01-28. Applies to evaluation and funded accounts as published (no per-plan table found). LIVE ELITE commission: not stated.

### 9. Machine-readable blocks

Common to all blocks: `trading_day_reset` is null (not published, §4), `personal_device_required: unknown`, `vps_allowed: no` (with the inconsistency noted in §7), commission MNQ 0.51 / NQ 2.04 per side with `includes: null`. Platforms are the firm-wide list (§2). `status: partial` unless stated. Plan blocks for sizes whose figures are not published carry null values.

#### 1-Step — evaluation — 50K

```yaml
firm: elite-trader-funding
plan: "1 Step"
phase: evaluation
size_label: "50K"
size_usd: "50000.00"
instruments: [MNQ, NQ]
platforms: ["Tradovate (web/desktop/mobile)", "NinjaTrader 8 (Windows only; Tradovate/CQG or Rithmic credentials)", "TradingView (Tradovate feed; selected at checkout)", "Rithmic: R|Trader Pro, ATAS, Bookmap, EdgeProX, eSignal, Finamark, Investor/RT, MotiveWave, Quantower, Sierra Chart", "Tickblaze"]
drawdown: { amount_usd: "2000.00", type: trailing_intraday, stops_trailing_at: "Evaluation: live trailing (highest unrealized profit) with no stop; the safety net / lock applies only after funding" }
daily_loss_limit: { amount_usd: null, kind: none }
profit_target_usd: "3000.00"
consistency_rule: { text: "No best-day consistency cap on this plan (only Revamped Fast Track has one). ATD qualifier: at least $250 realized profit and at least 30% of best day (evals bought on/after 2026-10-02 2:00 PM PT; $125 minimum on 10K/25K Static)", threshold_pct: null }
max_position: { minis: 8, micros: 80, scaling: "1 mini = 10 micros (1 mini = 1 position; 10 micros = 1 position)" }
min_trading_days: 5
trading_day_reset: { time: null, tz_as_published: null }
commission_per_contract: { mnq_usd: "0.51", nq_usd: "2.04", basis: per_side, includes: null }
copy_trading_allowed: conditional
automated_trading_allowed: conditional
automation_rule_text: "If you choose to use a trade copier, AI, bot, EA, algo, or other automated trading tool, you do so entirely at your own risk. The decision and responsibility to use such tools rest solely with the user. AI, bots, EAs, and algos additionally require ETF's written authorization before use. [ToS (ac)] makes use of artificial intelligence (AI), bots, automated trading systems, trade copiers, or other automated trading strategies that are not expressly authorized in writing by ETF."
vps_allowed: no
personal_device_required: unknown
status: partial
not_confirmed_fields: [trading_day_reset, personal_device_required, commission_per_contract.includes, vps_allowed]
notes: "Live trailing drawdown follows highest unrealized profit. No daily loss limit. Must be flat 1 minute before the instrument's close (ToS (ae)); no overnight/weekend holds. Min 5 trading days (One Day to Pass add-on lifts it to 1). Eval reset fee $47, unlimited resets. Elite Sim-Funded: at least one trade per week or the account is closed. 'Trade How You Want' promotion (in effect until ETF ends it) removes the 10-second minimum duration and allows DCA/Martingale on sim accounts. Order sizes: minis and micros are enforced independently by the platform but stacking both is 'not acceptable use' (S4). Entire position limit is hard: exceeding it fails the account."
sources:
  - { field: drawdown, url: "https://elitetraderfunding.app/1-step-evaluation", accessed: "2026-10-23" }
  - { field: profit_target_usd, url: "https://elitetraderfunding.app/1-step-evaluation", accessed: "2026-10-23" }
  - { field: max_position, url: "https://elitetraderfunding.app/help/contract-limits", accessed: "2026-10-23" }
  - { field: min_trading_days, url: "https://elitetraderfunding.app/help/how-the-1-step-plan-works", accessed: "2026-10-23" }
  - { field: consistency_rule, url: "https://elitetraderfunding.app/help/consistency-rule-and-active-trade-days-atd-explained", accessed: "2026-10-23" }
  - { field: commission_per_contract, url: "https://elitetraderfunding.app/help/guide-to-our-commissions-and-exchanges-we-support", accessed: "2026-10-23" }
  - { field: automation_rule_text, url: "https://elitetraderfunding.app/help/trade-copier-disclaimer", accessed: "2026-10-23" }
  - { field: automation_rule_text, url: "https://elitetraderfunding.app/terms-of-service", accessed: "2026-10-23" }
  - { field: vps_allowed, url: "https://elitetraderfunding.app/help/vpn-and-ip-masking-policy", accessed: "2026-10-23" }
  - { field: platforms, url: "https://elitetraderfunding.app/help/supported-platforms-and-data-feed", accessed: "2026-10-23" }
```

#### 1-Step — evaluation — 100K

```yaml
firm: elite-trader-funding
plan: "1 Step"
phase: evaluation
size_label: "100K"
size_usd: "100000.00"
instruments: [MNQ, NQ]
platforms: ["Tradovate (web/desktop/mobile)", "NinjaTrader 8 (Windows only; Tradovate/CQG or Rithmic credentials)", "TradingView (Tradovate feed; selected at checkout)", "Rithmic: R|Trader Pro, ATAS, Bookmap, EdgeProX, eSignal, Finamark, Investor/RT, MotiveWave, Quantower, Sierra Chart", "Tickblaze"]
drawdown: { amount_usd: "3000.00", type: trailing_intraday, stops_trailing_at: "Evaluation: live trailing (highest unrealized profit) with no stop; the safety net / lock applies only after funding" }
daily_loss_limit: { amount_usd: null, kind: none }
profit_target_usd: "6000.00"
consistency_rule: { text: "No best-day consistency cap on this plan (only Revamped Fast Track has one). ATD qualifier: at least $250 realized profit and at least 30% of best day (evals bought on/after 2026-10-02 2:00 PM PT; $125 minimum on 10K/25K Static)", threshold_pct: null }
max_position: { minis: 14, micros: 140, scaling: "1 mini = 10 micros (1 mini = 1 position; 10 micros = 1 position)" }
min_trading_days: 5
trading_day_reset: { time: null, tz_as_published: null }
commission_per_contract: { mnq_usd: "0.51", nq_usd: "2.04", basis: per_side, includes: null }
copy_trading_allowed: conditional
automated_trading_allowed: conditional
automation_rule_text: "If you choose to use a trade copier, AI, bot, EA, algo, or other automated trading tool, you do so entirely at your own risk. The decision and responsibility to use such tools rest solely with the user. AI, bots, EAs, and algos additionally require ETF's written authorization before use. [ToS (ac)] makes use of artificial intelligence (AI), bots, automated trading systems, trade copiers, or other automated trading strategies that are not expressly authorized in writing by ETF."
vps_allowed: no
personal_device_required: unknown
status: partial
not_confirmed_fields: [trading_day_reset, personal_device_required, commission_per_contract.includes, vps_allowed]
notes: "Live trailing drawdown follows highest unrealized profit. No daily loss limit. Must be flat 1 minute before the instrument's close (ToS (ae)); no overnight/weekend holds. Min 5 trading days (One Day to Pass add-on lifts it to 1). Eval reset fee $47, unlimited resets. Elite Sim-Funded: at least one trade per week or the account is closed. 'Trade How You Want' promotion (in effect until ETF ends it) removes the 10-second minimum duration and allows DCA/Martingale on sim accounts. Order sizes: minis and micros are enforced independently by the platform but stacking both is 'not acceptable use' (S4). Entire position limit is hard: exceeding it fails the account."
sources:
  - { field: drawdown, url: "https://elitetraderfunding.app/1-step-evaluation", accessed: "2026-10-23" }
  - { field: profit_target_usd, url: "https://elitetraderfunding.app/1-step-evaluation", accessed: "2026-10-23" }
  - { field: max_position, url: "https://elitetraderfunding.app/help/contract-limits", accessed: "2026-10-23" }
  - { field: min_trading_days, url: "https://elitetraderfunding.app/help/how-the-1-step-plan-works", accessed: "2026-10-23" }
  - { field: consistency_rule, url: "https://elitetraderfunding.app/help/consistency-rule-and-active-trade-days-atd-explained", accessed: "2026-10-23" }
  - { field: commission_per_contract, url: "https://elitetraderfunding.app/help/guide-to-our-commissions-and-exchanges-we-support", accessed: "2026-10-23" }
  - { field: automation_rule_text, url: "https://elitetraderfunding.app/help/trade-copier-disclaimer", accessed: "2026-10-23" }
  - { field: automation_rule_text, url: "https://elitetraderfunding.app/terms-of-service", accessed: "2026-10-23" }
  - { field: vps_allowed, url: "https://elitetraderfunding.app/help/vpn-and-ip-masking-policy", accessed: "2026-10-23" }
  - { field: platforms, url: "https://elitetraderfunding.app/help/supported-platforms-and-data-feed", accessed: "2026-10-23" }
```

#### 1-Step — evaluation — 150K

```yaml
firm: elite-trader-funding
plan: "1 Step"
phase: evaluation
size_label: "150K"
size_usd: "150000.00"
instruments: [MNQ, NQ]
platforms: ["Tradovate (web/desktop/mobile)", "NinjaTrader 8 (Windows only; Tradovate/CQG or Rithmic credentials)", "TradingView (Tradovate feed; selected at checkout)", "Rithmic: R|Trader Pro, ATAS, Bookmap, EdgeProX, eSignal, Finamark, Investor/RT, MotiveWave, Quantower, Sierra Chart", "Tickblaze"]
drawdown: { amount_usd: null, type: trailing_intraday, stops_trailing_at: "Evaluation: live trailing (highest unrealized profit) with no stop; the safety net / lock applies only after funding" }
daily_loss_limit: { amount_usd: null, kind: none }
profit_target_usd: null
consistency_rule: { text: "No best-day consistency cap on this plan (only Revamped Fast Track has one). ATD qualifier: at least $250 realized profit and at least 30% of best day (evals bought on/after 2026-10-02 2:00 PM PT; $125 minimum on 10K/25K Static)", threshold_pct: null }
max_position: { minis: 18, micros: 180, scaling: "1 mini = 10 micros (1 mini = 1 position; 10 micros = 1 position)" }
min_trading_days: 5
trading_day_reset: { time: null, tz_as_published: null }
commission_per_contract: { mnq_usd: "0.51", nq_usd: "2.04", basis: per_side, includes: null }
copy_trading_allowed: conditional
automated_trading_allowed: conditional
automation_rule_text: "If you choose to use a trade copier, AI, bot, EA, algo, or other automated trading tool, you do so entirely at your own risk. The decision and responsibility to use such tools rest solely with the user. AI, bots, EAs, and algos additionally require ETF's written authorization before use. [ToS (ac)] makes use of artificial intelligence (AI), bots, automated trading systems, trade copiers, or other automated trading strategies that are not expressly authorized in writing by ETF."
vps_allowed: no
personal_device_required: unknown
status: partial
not_confirmed_fields: [trading_day_reset, personal_device_required, commission_per_contract.includes, vps_allowed, drawdown.amount_usd, profit_target_usd]
notes: "Live trailing drawdown follows highest unrealized profit. No daily loss limit. Must be flat 1 minute before the instrument's close (ToS (ae)); no overnight/weekend holds. Min 5 trading days (One Day to Pass add-on lifts it to 1). Eval reset fee $47, unlimited resets. Elite Sim-Funded: at least one trade per week or the account is closed. 'Trade How You Want' promotion (in effect until ETF ends it) removes the 10-second minimum duration and allows DCA/Martingale on sim accounts. Order sizes: minis and micros are enforced independently by the platform but stacking both is 'not acceptable use' (S4). Entire position limit is hard: exceeding it fails the account. 150K drawdown and profit target are not published on any page read (only position limits in S4, size listed on the evaluations page and the cost calculator)."
sources:
  - { field: drawdown, url: "https://elitetraderfunding.app/1-step-evaluation", accessed: "2026-10-23" }
  - { field: profit_target_usd, url: "https://elitetraderfunding.app/1-step-evaluation", accessed: "2026-10-23" }
  - { field: max_position, url: "https://elitetraderfunding.app/help/contract-limits", accessed: "2026-10-23" }
  - { field: min_trading_days, url: "https://elitetraderfunding.app/help/how-the-1-step-plan-works", accessed: "2026-10-23" }
  - { field: consistency_rule, url: "https://elitetraderfunding.app/help/consistency-rule-and-active-trade-days-atd-explained", accessed: "2026-10-23" }
  - { field: commission_per_contract, url: "https://elitetraderfunding.app/help/guide-to-our-commissions-and-exchanges-we-support", accessed: "2026-10-23" }
  - { field: automation_rule_text, url: "https://elitetraderfunding.app/help/trade-copier-disclaimer", accessed: "2026-10-23" }
  - { field: automation_rule_text, url: "https://elitetraderfunding.app/terms-of-service", accessed: "2026-10-23" }
  - { field: vps_allowed, url: "https://elitetraderfunding.app/help/vpn-and-ip-masking-policy", accessed: "2026-10-23" }
  - { field: platforms, url: "https://elitetraderfunding.app/help/supported-platforms-and-data-feed", accessed: "2026-10-23" }
```

#### 1-Step — evaluation — 250K

```yaml
firm: elite-trader-funding
plan: "1 Step"
phase: evaluation
size_label: "250K"
size_usd: "250000.00"
instruments: [MNQ, NQ]
platforms: ["Tradovate (web/desktop/mobile)", "NinjaTrader 8 (Windows only; Tradovate/CQG or Rithmic credentials)", "TradingView (Tradovate feed; selected at checkout)", "Rithmic: R|Trader Pro, ATAS, Bookmap, EdgeProX, eSignal, Finamark, Investor/RT, MotiveWave, Quantower, Sierra Chart", "Tickblaze"]
drawdown: { amount_usd: "6500.00", type: trailing_intraday, stops_trailing_at: "Evaluation: live trailing (highest unrealized profit) with no stop; the safety net / lock applies only after funding" }
daily_loss_limit: { amount_usd: null, kind: none }
profit_target_usd: "15000.00"
consistency_rule: { text: "No best-day consistency cap on this plan (only Revamped Fast Track has one). ATD qualifier: at least $250 realized profit and at least 30% of best day (evals bought on/after 2026-10-02 2:00 PM PT; $125 minimum on 10K/25K Static)", threshold_pct: null }
max_position: { minis: 24, micros: 240, scaling: "1 mini = 10 micros (1 mini = 1 position; 10 micros = 1 position)" }
min_trading_days: 5
trading_day_reset: { time: null, tz_as_published: null }
commission_per_contract: { mnq_usd: "0.51", nq_usd: "2.04", basis: per_side, includes: null }
copy_trading_allowed: conditional
automated_trading_allowed: conditional
automation_rule_text: "If you choose to use a trade copier, AI, bot, EA, algo, or other automated trading tool, you do so entirely at your own risk. The decision and responsibility to use such tools rest solely with the user. AI, bots, EAs, and algos additionally require ETF's written authorization before use. [ToS (ac)] makes use of artificial intelligence (AI), bots, automated trading systems, trade copiers, or other automated trading strategies that are not expressly authorized in writing by ETF."
vps_allowed: no
personal_device_required: unknown
status: partial
not_confirmed_fields: [trading_day_reset, personal_device_required, commission_per_contract.includes, vps_allowed, availability (sold vs no longer sold)]
notes: "Live trailing drawdown follows highest unrealized profit. No daily loss limit. Must be flat 1 minute before the instrument's close (ToS (ae)); no overnight/weekend holds. Min 5 trading days (One Day to Pass add-on lifts it to 1). Eval reset fee $47, unlimited resets. Elite Sim-Funded: at least one trade per week or the account is closed. 'Trade How You Want' promotion (in effect until ETF ends it) removes the 10-second minimum duration and allows DCA/Martingale on sim accounts. Order sizes: minis and micros are enforced independently by the platform but stacking both is 'not acceptable use' (S4). Entire position limit is hard: exceeding it fails the account. Availability conflict: the 1-Step page and drawdown calculator list 250K, but Contract Limits says '250K Live Trailing Eval (no longer sold, existing accounts only)'."
sources:
  - { field: drawdown, url: "https://elitetraderfunding.app/1-step-evaluation", accessed: "2026-10-23" }
  - { field: profit_target_usd, url: "https://elitetraderfunding.app/1-step-evaluation", accessed: "2026-10-23" }
  - { field: max_position, url: "https://elitetraderfunding.app/help/contract-limits", accessed: "2026-10-23" }
  - { field: min_trading_days, url: "https://elitetraderfunding.app/help/how-the-1-step-plan-works", accessed: "2026-10-23" }
  - { field: consistency_rule, url: "https://elitetraderfunding.app/help/consistency-rule-and-active-trade-days-atd-explained", accessed: "2026-10-23" }
  - { field: commission_per_contract, url: "https://elitetraderfunding.app/help/guide-to-our-commissions-and-exchanges-we-support", accessed: "2026-10-23" }
  - { field: automation_rule_text, url: "https://elitetraderfunding.app/help/trade-copier-disclaimer", accessed: "2026-10-23" }
  - { field: automation_rule_text, url: "https://elitetraderfunding.app/terms-of-service", accessed: "2026-10-23" }
  - { field: vps_allowed, url: "https://elitetraderfunding.app/help/vpn-and-ip-masking-policy", accessed: "2026-10-23" }
  - { field: platforms, url: "https://elitetraderfunding.app/help/supported-platforms-and-data-feed", accessed: "2026-10-23" }
```

#### End of Day — evaluation — 50K

```yaml
firm: elite-trader-funding
plan: "End of Day"
phase: evaluation
size_label: "50K"
size_usd: "50000.00"
instruments: [MNQ, NQ]
platforms: ["Tradovate (web/desktop/mobile)", "NinjaTrader 8 (Windows only; Tradovate/CQG or Rithmic credentials)", "TradingView (Tradovate feed; selected at checkout)", "Rithmic: R|Trader Pro, ATAS, Bookmap, EdgeProX, eSignal, Finamark, Investor/RT, MotiveWave, Quantower, Sierra Chart", "Tickblaze"]
drawdown: { amount_usd: "2000.00", type: trailing_eod, stops_trailing_at: "Evaluation: trails highest end-of-day realized balance with no stop; locks only after funding" }
daily_loss_limit: { amount_usd: "1100.00", kind: hard }
profit_target_usd: "3000.00"
consistency_rule: { text: "No best-day consistency cap on this plan (only Revamped Fast Track has one). ATD qualifier: at least $250 realized profit and at least 30% of best day (evals bought on/after 2026-10-02 2:00 PM PT; $125 minimum on 10K/25K Static)", threshold_pct: null }
max_position: { minis: 8, micros: 80, scaling: "1 mini = 10 micros (1 mini = 1 position; 10 micros = 1 position)" }
min_trading_days: 5
trading_day_reset: { time: null, tz_as_published: null }
commission_per_contract: { mnq_usd: "0.51", nq_usd: "2.04", basis: per_side, includes: null }
copy_trading_allowed: conditional
automated_trading_allowed: conditional
automation_rule_text: "If you choose to use a trade copier, AI, bot, EA, algo, or other automated trading tool, you do so entirely at your own risk. The decision and responsibility to use such tools rest solely with the user. AI, bots, EAs, and algos additionally require ETF's written authorization before use. [ToS (ac)] makes use of artificial intelligence (AI), bots, automated trading systems, trade copiers, or other automated trading strategies that are not expressly authorized in writing by ETF."
vps_allowed: no
personal_device_required: unknown
status: partial
not_confirmed_fields: [trading_day_reset, personal_device_required, commission_per_contract.includes, vps_allowed]
notes: "Daily loss limit is calculated from the prior day's ending balance; open losses count intraday; breaching either the daily loss limit or max drawdown fails the account. Must be flat 1 minute before the instrument's close (ToS (ae)); no overnight/weekend holds. Min 5 trading days (One Day to Pass add-on lifts it to 1). Eval reset fee $47, unlimited resets. Elite Sim-Funded: at least one trade per week or the account is closed. 'Trade How You Want' promotion (in effect until ETF ends it) removes the 10-second minimum duration and allows DCA/Martingale on sim accounts. Order sizes: minis and micros are enforced independently by the platform but stacking both is 'not acceptable use' (S4). Entire position limit is hard: exceeding it fails the account."
sources:
  - { field: drawdown, url: "https://elitetraderfunding.app/end-of-day-evaluation", accessed: "2026-10-23" }
  - { field: daily_loss_limit, url: "https://elitetraderfunding.app/end-of-day-evaluation", accessed: "2026-10-23" }
  - { field: profit_target_usd, url: "https://elitetraderfunding.app/end-of-day-evaluation", accessed: "2026-10-23" }
  - { field: max_position, url: "https://elitetraderfunding.app/help/contract-limits", accessed: "2026-10-23" }
  - { field: min_trading_days, url: "https://elitetraderfunding.app/help/how-the-end-of-day-plan-works", accessed: "2026-10-23" }
  - { field: trading_day_reset, url: "https://elitetraderfunding.app/help/how-the-end-of-day-plan-works", accessed: "2026-10-23" }
  - { field: commission_per_contract, url: "https://elitetraderfunding.app/help/guide-to-our-commissions-and-exchanges-we-support", accessed: "2026-10-23" }
  - { field: automation_rule_text, url: "https://elitetraderfunding.app/help/trade-copier-disclaimer", accessed: "2026-10-23" }
  - { field: automation_rule_text, url: "https://elitetraderfunding.app/terms-of-service", accessed: "2026-10-23" }
  - { field: vps_allowed, url: "https://elitetraderfunding.app/help/vpn-and-ip-masking-policy", accessed: "2026-10-23" }
  - { field: platforms, url: "https://elitetraderfunding.app/help/supported-platforms-and-data-feed", accessed: "2026-10-23" }
```

#### End of Day — evaluation — 100K

```yaml
firm: elite-trader-funding
plan: "End of Day"
phase: evaluation
size_label: "100K"
size_usd: "100000.00"
instruments: [MNQ, NQ]
platforms: ["Tradovate (web/desktop/mobile)", "NinjaTrader 8 (Windows only; Tradovate/CQG or Rithmic credentials)", "TradingView (Tradovate feed; selected at checkout)", "Rithmic: R|Trader Pro, ATAS, Bookmap, EdgeProX, eSignal, Finamark, Investor/RT, MotiveWave, Quantower, Sierra Chart", "Tickblaze"]
drawdown: { amount_usd: "3500.00", type: trailing_eod, stops_trailing_at: "Evaluation: trails highest end-of-day realized balance with no stop; locks only after funding" }
daily_loss_limit: { amount_usd: "2200.00", kind: hard }
profit_target_usd: "6000.00"
consistency_rule: { text: "No best-day consistency cap on this plan (only Revamped Fast Track has one). ATD qualifier: at least $250 realized profit and at least 30% of best day (evals bought on/after 2026-10-02 2:00 PM PT; $125 minimum on 10K/25K Static)", threshold_pct: null }
max_position: { minis: 14, micros: 140, scaling: "1 mini = 10 micros (1 mini = 1 position; 10 micros = 1 position)" }
min_trading_days: 5
trading_day_reset: { time: null, tz_as_published: null }
commission_per_contract: { mnq_usd: "0.51", nq_usd: "2.04", basis: per_side, includes: null }
copy_trading_allowed: conditional
automated_trading_allowed: conditional
automation_rule_text: "If you choose to use a trade copier, AI, bot, EA, algo, or other automated trading tool, you do so entirely at your own risk. The decision and responsibility to use such tools rest solely with the user. AI, bots, EAs, and algos additionally require ETF's written authorization before use. [ToS (ac)] makes use of artificial intelligence (AI), bots, automated trading systems, trade copiers, or other automated trading strategies that are not expressly authorized in writing by ETF."
vps_allowed: no
personal_device_required: unknown
status: partial
not_confirmed_fields: [trading_day_reset, personal_device_required, commission_per_contract.includes, vps_allowed]
notes: "Daily loss limit is calculated from the prior day's ending balance; open losses count intraday; breaching either the daily loss limit or max drawdown fails the account. Must be flat 1 minute before the instrument's close (ToS (ae)); no overnight/weekend holds. Min 5 trading days (One Day to Pass add-on lifts it to 1). Eval reset fee $47, unlimited resets. Elite Sim-Funded: at least one trade per week or the account is closed. 'Trade How You Want' promotion (in effect until ETF ends it) removes the 10-second minimum duration and allows DCA/Martingale on sim accounts. Order sizes: minis and micros are enforced independently by the platform but stacking both is 'not acceptable use' (S4). Entire position limit is hard: exceeding it fails the account."
sources:
  - { field: drawdown, url: "https://elitetraderfunding.app/end-of-day-evaluation", accessed: "2026-10-23" }
  - { field: daily_loss_limit, url: "https://elitetraderfunding.app/end-of-day-evaluation", accessed: "2026-10-23" }
  - { field: profit_target_usd, url: "https://elitetraderfunding.app/end-of-day-evaluation", accessed: "2026-10-23" }
  - { field: max_position, url: "https://elitetraderfunding.app/help/contract-limits", accessed: "2026-10-23" }
  - { field: min_trading_days, url: "https://elitetraderfunding.app/help/how-the-end-of-day-plan-works", accessed: "2026-10-23" }
  - { field: trading_day_reset, url: "https://elitetraderfunding.app/help/how-the-end-of-day-plan-works", accessed: "2026-10-23" }
  - { field: commission_per_contract, url: "https://elitetraderfunding.app/help/guide-to-our-commissions-and-exchanges-we-support", accessed: "2026-10-23" }
  - { field: automation_rule_text, url: "https://elitetraderfunding.app/help/trade-copier-disclaimer", accessed: "2026-10-23" }
  - { field: automation_rule_text, url: "https://elitetraderfunding.app/terms-of-service", accessed: "2026-10-23" }
  - { field: vps_allowed, url: "https://elitetraderfunding.app/help/vpn-and-ip-masking-policy", accessed: "2026-10-23" }
  - { field: platforms, url: "https://elitetraderfunding.app/help/supported-platforms-and-data-feed", accessed: "2026-10-23" }
```

#### End of Day — evaluation — 150K

```yaml
firm: elite-trader-funding
plan: "End of Day"
phase: evaluation
size_label: "150K"
size_usd: "150000.00"
instruments: [MNQ, NQ]
platforms: ["Tradovate (web/desktop/mobile)", "NinjaTrader 8 (Windows only; Tradovate/CQG or Rithmic credentials)", "TradingView (Tradovate feed; selected at checkout)", "Rithmic: R|Trader Pro, ATAS, Bookmap, EdgeProX, eSignal, Finamark, Investor/RT, MotiveWave, Quantower, Sierra Chart", "Tickblaze"]
drawdown: { amount_usd: null, type: trailing_eod, stops_trailing_at: "Evaluation: trails highest end-of-day realized balance with no stop; locks only after funding" }
daily_loss_limit: { amount_usd: null, kind: hard }
profit_target_usd: null
consistency_rule: { text: "No best-day consistency cap on this plan (only Revamped Fast Track has one). ATD qualifier: at least $250 realized profit and at least 30% of best day (evals bought on/after 2026-10-02 2:00 PM PT; $125 minimum on 10K/25K Static)", threshold_pct: null }
max_position: { minis: 18, micros: 180, scaling: "1 mini = 10 micros (1 mini = 1 position; 10 micros = 1 position)" }
min_trading_days: 5
trading_day_reset: { time: null, tz_as_published: null }
commission_per_contract: { mnq_usd: "0.51", nq_usd: "2.04", basis: per_side, includes: null }
copy_trading_allowed: conditional
automated_trading_allowed: conditional
automation_rule_text: "If you choose to use a trade copier, AI, bot, EA, algo, or other automated trading tool, you do so entirely at your own risk. The decision and responsibility to use such tools rest solely with the user. AI, bots, EAs, and algos additionally require ETF's written authorization before use. [ToS (ac)] makes use of artificial intelligence (AI), bots, automated trading systems, trade copiers, or other automated trading strategies that are not expressly authorized in writing by ETF."
vps_allowed: no
personal_device_required: unknown
status: partial
not_confirmed_fields: [trading_day_reset, personal_device_required, commission_per_contract.includes, vps_allowed, drawdown.amount_usd, daily_loss_limit.amount_usd, profit_target_usd]
notes: "Daily loss limit is calculated from the prior day's ending balance; open losses count intraday; breaching either the daily loss limit or max drawdown fails the account. Must be flat 1 minute before the instrument's close (ToS (ae)); no overnight/weekend holds. Min 5 trading days (One Day to Pass add-on lifts it to 1). Eval reset fee $47, unlimited resets. Elite Sim-Funded: at least one trade per week or the account is closed. 'Trade How You Want' promotion (in effect until ETF ends it) removes the 10-second minimum duration and allows DCA/Martingale on sim accounts. Order sizes: minis and micros are enforced independently by the platform but stacking both is 'not acceptable use' (S4). Entire position limit is hard: exceeding it fails the account. 150K: only the position limits are published (S4, evaluations page lists the size); drawdown, daily loss limit and profit target not found. The evaluations page says daily loss limit '$1,100 to $2,200 by size' for 50K/100K/150K."
sources:
  - { field: drawdown, url: "https://elitetraderfunding.app/end-of-day-evaluation", accessed: "2026-10-23" }
  - { field: daily_loss_limit, url: "https://elitetraderfunding.app/end-of-day-evaluation", accessed: "2026-10-23" }
  - { field: profit_target_usd, url: "https://elitetraderfunding.app/end-of-day-evaluation", accessed: "2026-10-23" }
  - { field: max_position, url: "https://elitetraderfunding.app/help/contract-limits", accessed: "2026-10-23" }
  - { field: min_trading_days, url: "https://elitetraderfunding.app/help/how-the-end-of-day-plan-works", accessed: "2026-10-23" }
  - { field: trading_day_reset, url: "https://elitetraderfunding.app/help/how-the-end-of-day-plan-works", accessed: "2026-10-23" }
  - { field: commission_per_contract, url: "https://elitetraderfunding.app/help/guide-to-our-commissions-and-exchanges-we-support", accessed: "2026-10-23" }
  - { field: automation_rule_text, url: "https://elitetraderfunding.app/help/trade-copier-disclaimer", accessed: "2026-10-23" }
  - { field: automation_rule_text, url: "https://elitetraderfunding.app/terms-of-service", accessed: "2026-10-23" }
  - { field: vps_allowed, url: "https://elitetraderfunding.app/help/vpn-and-ip-masking-policy", accessed: "2026-10-23" }
  - { field: platforms, url: "https://elitetraderfunding.app/help/supported-platforms-and-data-feed", accessed: "2026-10-23" }
```

#### Static Drawdown — evaluation — 25K

```yaml
firm: elite-trader-funding
plan: "Static Drawdown"
phase: evaluation
size_label: "25K"
size_usd: "25000.00"
instruments: [MNQ, NQ]
platforms: ["Tradovate (web/desktop/mobile)", "NinjaTrader 8 (Windows only; Tradovate/CQG or Rithmic credentials)", "TradingView (Tradovate feed; selected at checkout)", "Rithmic: R|Trader Pro, ATAS, Bookmap, EdgeProX, eSignal, Finamark, Investor/RT, MotiveWave, Quantower, Sierra Chart", "Tickblaze"]
drawdown: { amount_usd: "1000.00", type: static, stops_trailing_at: "Never trails (fixed minimum balance set at start)" }
daily_loss_limit: { amount_usd: null, kind: none }
profit_target_usd: "2000.00"
consistency_rule: { text: "No best-day consistency cap on this plan (only Revamped Fast Track has one). ATD qualifier: at least $250 realized profit and at least 30% of best day (evals bought on/after 2026-10-02 2:00 PM PT; $125 minimum on 10K/25K Static)", threshold_pct: null }
max_position: { minis: 2, micros: 20, scaling: "1 mini = 10 micros (1 mini = 1 position; 10 micros = 1 position)" }
min_trading_days: 5
trading_day_reset: { time: null, tz_as_published: null }
commission_per_contract: { mnq_usd: "0.51", nq_usd: "2.04", basis: per_side, includes: null }
copy_trading_allowed: conditional
automated_trading_allowed: conditional
automation_rule_text: "If you choose to use a trade copier, AI, bot, EA, algo, or other automated trading tool, you do so entirely at your own risk. The decision and responsibility to use such tools rest solely with the user. AI, bots, EAs, and algos additionally require ETF's written authorization before use. [ToS (ac)] makes use of artificial intelligence (AI), bots, automated trading systems, trade copiers, or other automated trading strategies that are not expressly authorized in writing by ETF."
vps_allowed: no
personal_device_required: unknown
status: partial
not_confirmed_fields: [trading_day_reset, personal_device_required, commission_per_contract.includes, vps_allowed]
notes: "No daily loss limit, no consistency rule. Must be flat 1 minute before the instrument's close (ToS (ae)); no overnight/weekend holds. Min 5 trading days (One Day to Pass add-on lifts it to 1). Eval reset fee $47, unlimited resets. Elite Sim-Funded: at least one trade per week or the account is closed. 'Trade How You Want' promotion (in effect until ETF ends it) removes the 10-second minimum duration and allows DCA/Martingale on sim accounts. Order sizes: minis and micros are enforced independently by the platform but stacking both is 'not acceptable use' (S4). Entire position limit is hard: exceeding it fails the account."
sources:
  - { field: drawdown, url: "https://elitetraderfunding.app/static-drawdown", accessed: "2026-10-23" }
  - { field: profit_target_usd, url: "https://elitetraderfunding.app/static-drawdown", accessed: "2026-10-23" }
  - { field: max_position, url: "https://elitetraderfunding.app/help/contract-limits", accessed: "2026-10-23" }
  - { field: min_trading_days, url: "https://elitetraderfunding.app/static-drawdown", accessed: "2026-10-23" }
  - { field: commission_per_contract, url: "https://elitetraderfunding.app/help/guide-to-our-commissions-and-exchanges-we-support", accessed: "2026-10-23" }
  - { field: automation_rule_text, url: "https://elitetraderfunding.app/help/trade-copier-disclaimer", accessed: "2026-10-23" }
  - { field: automation_rule_text, url: "https://elitetraderfunding.app/terms-of-service", accessed: "2026-10-23" }
  - { field: vps_allowed, url: "https://elitetraderfunding.app/help/vpn-and-ip-masking-policy", accessed: "2026-10-23" }
  - { field: platforms, url: "https://elitetraderfunding.app/help/supported-platforms-and-data-feed", accessed: "2026-10-23" }
```

#### Static Drawdown — evaluation — 50K

```yaml
firm: elite-trader-funding
plan: "Static Drawdown"
phase: evaluation
size_label: "50K"
size_usd: "50000.00"
instruments: [MNQ, NQ]
platforms: ["Tradovate (web/desktop/mobile)", "NinjaTrader 8 (Windows only; Tradovate/CQG or Rithmic credentials)", "TradingView (Tradovate feed; selected at checkout)", "Rithmic: R|Trader Pro, ATAS, Bookmap, EdgeProX, eSignal, Finamark, Investor/RT, MotiveWave, Quantower, Sierra Chart", "Tickblaze"]
drawdown: { amount_usd: "2000.00", type: static, stops_trailing_at: "Never trails (fixed minimum balance set at start)" }
daily_loss_limit: { amount_usd: null, kind: none }
profit_target_usd: "4000.00"
consistency_rule: { text: "No best-day consistency cap on this plan (only Revamped Fast Track has one). ATD qualifier: at least $250 realized profit and at least 30% of best day (evals bought on/after 2026-10-02 2:00 PM PT; $125 minimum on 10K/25K Static)", threshold_pct: null }
max_position: { minis: 4, micros: 40, scaling: "1 mini = 10 micros (1 mini = 1 position; 10 micros = 1 position)" }
min_trading_days: 5
trading_day_reset: { time: null, tz_as_published: null }
commission_per_contract: { mnq_usd: "0.51", nq_usd: "2.04", basis: per_side, includes: null }
copy_trading_allowed: conditional
automated_trading_allowed: conditional
automation_rule_text: "If you choose to use a trade copier, AI, bot, EA, algo, or other automated trading tool, you do so entirely at your own risk. The decision and responsibility to use such tools rest solely with the user. AI, bots, EAs, and algos additionally require ETF's written authorization before use. [ToS (ac)] makes use of artificial intelligence (AI), bots, automated trading systems, trade copiers, or other automated trading strategies that are not expressly authorized in writing by ETF."
vps_allowed: no
personal_device_required: unknown
status: partial
not_confirmed_fields: [trading_day_reset, personal_device_required, commission_per_contract.includes, vps_allowed]
notes: "No daily loss limit, no consistency rule. Must be flat 1 minute before the instrument's close (ToS (ae)); no overnight/weekend holds. Min 5 trading days (One Day to Pass add-on lifts it to 1). Eval reset fee $47, unlimited resets. Elite Sim-Funded: at least one trade per week or the account is closed. 'Trade How You Want' promotion (in effect until ETF ends it) removes the 10-second minimum duration and allows DCA/Martingale on sim accounts. Order sizes: minis and micros are enforced independently by the platform but stacking both is 'not acceptable use' (S4). Entire position limit is hard: exceeding it fails the account."
sources:
  - { field: drawdown, url: "https://elitetraderfunding.app/static-drawdown", accessed: "2026-10-23" }
  - { field: profit_target_usd, url: "https://elitetraderfunding.app/static-drawdown", accessed: "2026-10-23" }
  - { field: max_position, url: "https://elitetraderfunding.app/help/contract-limits", accessed: "2026-10-23" }
  - { field: min_trading_days, url: "https://elitetraderfunding.app/static-drawdown", accessed: "2026-10-23" }
  - { field: commission_per_contract, url: "https://elitetraderfunding.app/help/guide-to-our-commissions-and-exchanges-we-support", accessed: "2026-10-23" }
  - { field: automation_rule_text, url: "https://elitetraderfunding.app/help/trade-copier-disclaimer", accessed: "2026-10-23" }
  - { field: automation_rule_text, url: "https://elitetraderfunding.app/terms-of-service", accessed: "2026-10-23" }
  - { field: vps_allowed, url: "https://elitetraderfunding.app/help/vpn-and-ip-masking-policy", accessed: "2026-10-23" }
  - { field: platforms, url: "https://elitetraderfunding.app/help/supported-platforms-and-data-feed", accessed: "2026-10-23" }
```

#### Static Drawdown — evaluation — 10K

```yaml
firm: elite-trader-funding
plan: "Static Drawdown"
phase: evaluation
size_label: "10K"
size_usd: "10000.00"
instruments: [MNQ, NQ]
platforms: ["Tradovate (web/desktop/mobile)", "NinjaTrader 8 (Windows only; Tradovate/CQG or Rithmic credentials)", "TradingView (Tradovate feed; selected at checkout)", "Rithmic: R|Trader Pro, ATAS, Bookmap, EdgeProX, eSignal, Finamark, Investor/RT, MotiveWave, Quantower, Sierra Chart", "Tickblaze"]
drawdown: { amount_usd: "500.00", type: static, stops_trailing_at: "Never trails (fixed minimum balance set at start)" }
daily_loss_limit: { amount_usd: null, kind: none }
profit_target_usd: "1000.00"
consistency_rule: { text: "No best-day consistency cap on this plan (only Revamped Fast Track has one). ATD qualifier: at least $250 realized profit and at least 30% of best day (evals bought on/after 2026-10-02 2:00 PM PT; $125 minimum on 10K/25K Static)", threshold_pct: null }
max_position: { minis: 1, micros: 10, scaling: "1 mini = 10 micros (1 mini = 1 position; 10 micros = 1 position)" }
min_trading_days: 5
trading_day_reset: { time: null, tz_as_published: null }
commission_per_contract: { mnq_usd: "0.51", nq_usd: "2.04", basis: per_side, includes: null }
copy_trading_allowed: conditional
automated_trading_allowed: conditional
automation_rule_text: "If you choose to use a trade copier, AI, bot, EA, algo, or other automated trading tool, you do so entirely at your own risk. The decision and responsibility to use such tools rest solely with the user. AI, bots, EAs, and algos additionally require ETF's written authorization before use. [ToS (ac)] makes use of artificial intelligence (AI), bots, automated trading systems, trade copiers, or other automated trading strategies that are not expressly authorized in writing by ETF."
vps_allowed: no
personal_device_required: unknown
status: partial
not_confirmed_fields: [trading_day_reset, personal_device_required, commission_per_contract.includes, vps_allowed]
notes: "No daily loss limit, no consistency rule. Must be flat 1 minute before the instrument's close (ToS (ae)); no overnight/weekend holds. Min 5 trading days (One Day to Pass add-on lifts it to 1). Eval reset fee $47, unlimited resets. Elite Sim-Funded: at least one trade per week or the account is closed. 'Trade How You Want' promotion (in effect until ETF ends it) removes the 10-second minimum duration and allows DCA/Martingale on sim accounts. Order sizes: minis and micros are enforced independently by the platform but stacking both is 'not acceptable use' (S4). Entire position limit is hard: exceeding it fails the account. LOW PRIORITY (below 25K)."
sources:
  - { field: drawdown, url: "https://elitetraderfunding.app/static-drawdown", accessed: "2026-10-23" }
  - { field: profit_target_usd, url: "https://elitetraderfunding.app/static-drawdown", accessed: "2026-10-23" }
  - { field: max_position, url: "https://elitetraderfunding.app/help/contract-limits", accessed: "2026-10-23" }
  - { field: min_trading_days, url: "https://elitetraderfunding.app/static-drawdown", accessed: "2026-10-23" }
  - { field: commission_per_contract, url: "https://elitetraderfunding.app/help/guide-to-our-commissions-and-exchanges-we-support", accessed: "2026-10-23" }
  - { field: automation_rule_text, url: "https://elitetraderfunding.app/help/trade-copier-disclaimer", accessed: "2026-10-23" }
  - { field: automation_rule_text, url: "https://elitetraderfunding.app/terms-of-service", accessed: "2026-10-23" }
  - { field: vps_allowed, url: "https://elitetraderfunding.app/help/vpn-and-ip-masking-policy", accessed: "2026-10-23" }
  - { field: platforms, url: "https://elitetraderfunding.app/help/supported-platforms-and-data-feed", accessed: "2026-10-23" }
```

#### Diamond Hands — evaluation — 100K

```yaml
firm: elite-trader-funding
plan: "Diamond Hands"
phase: evaluation
size_label: "100K"
size_usd: "100000.00"
instruments: [MNQ, NQ]
platforms: ["Tradovate (web/desktop/mobile)", "NinjaTrader 8 (Windows only; Tradovate/CQG or Rithmic credentials)", "TradingView (Tradovate feed; selected at checkout)", "Rithmic: R|Trader Pro, ATAS, Bookmap, EdgeProX, eSignal, Finamark, Investor/RT, MotiveWave, Quantower, Sierra Chart", "Tickblaze"]
drawdown: { amount_usd: "3500.00", type: trailing_eod, stops_trailing_at: "Evaluation: trails highest end-of-day balance with no stop; locks only after funding" }
daily_loss_limit: { amount_usd: "1500.00", kind: hard }
profit_target_usd: "5000.00"
consistency_rule: { text: "No best-day consistency cap on this plan (only Revamped Fast Track has one). ATD qualifier: at least $250 realized profit and at least 30% of best day (evals bought on/after 2026-10-02 2:00 PM PT; $125 minimum on 10K/25K Static)", threshold_pct: null }
max_position: { minis: 2, micros: 20, scaling: "1 mini = 10 micros (1 mini = 1 position; 10 micros = 1 position)" }
min_trading_days: 5
trading_day_reset: { time: null, tz_as_published: null }
commission_per_contract: { mnq_usd: "0.51", nq_usd: "2.04", basis: per_side, includes: null }
copy_trading_allowed: conditional
automated_trading_allowed: conditional
automation_rule_text: "If you choose to use a trade copier, AI, bot, EA, algo, or other automated trading tool, you do so entirely at your own risk. The decision and responsibility to use such tools rest solely with the user. AI, bots, EAs, and algos additionally require ETF's written authorization before use. [ToS (ac)] makes use of artificial intelligence (AI), bots, automated trading systems, trade copiers, or other automated trading strategies that are not expressly authorized in writing by ETF."
vps_allowed: no
personal_device_required: unknown
status: partial
not_confirmed_fields: [trading_day_reset, personal_device_required, commission_per_contract.includes, vps_allowed]
notes: "Only ETF evaluation (with Direct to Funded) that may hold positions overnight and over the weekend. Daily loss limit is a hard breach measured from the prior day's close; open losses count. Single size (100K). Eval reset $47. Weekly trade requirement applies. Tighter position cap (2 minis / 20 micros) than End of Day."
sources:
  - { field: drawdown, url: "https://elitetraderfunding.app/diamond-hands-evaluation", accessed: "2026-10-23" }
  - { field: daily_loss_limit, url: "https://elitetraderfunding.app/diamond-hands-evaluation", accessed: "2026-10-23" }
  - { field: profit_target_usd, url: "https://elitetraderfunding.app/diamond-hands-evaluation", accessed: "2026-10-23" }
  - { field: max_position, url: "https://elitetraderfunding.app/help/contract-limits", accessed: "2026-10-23" }
  - { field: min_trading_days, url: "https://elitetraderfunding.app/diamond-hands-evaluation", accessed: "2026-10-23" }
  - { field: commission_per_contract, url: "https://elitetraderfunding.app/help/guide-to-our-commissions-and-exchanges-we-support", accessed: "2026-10-23" }
  - { field: automation_rule_text, url: "https://elitetraderfunding.app/help/trade-copier-disclaimer", accessed: "2026-10-23" }
  - { field: automation_rule_text, url: "https://elitetraderfunding.app/terms-of-service", accessed: "2026-10-23" }
  - { field: vps_allowed, url: "https://elitetraderfunding.app/help/vpn-and-ip-masking-policy", accessed: "2026-10-23" }
  - { field: platforms, url: "https://elitetraderfunding.app/help/supported-platforms-and-data-feed", accessed: "2026-10-23" }
```

#### Fast Track EOD — evaluation — 10K

```yaml
firm: elite-trader-funding
plan: "Fast Track"
phase: evaluation
size_label: "10K"
size_usd: "10000.00"
instruments: [MNQ, NQ]
platforms: ["Tradovate (web/desktop/mobile)", "NinjaTrader 8 (Windows only; Tradovate/CQG or Rithmic credentials)", "TradingView (Tradovate feed; selected at checkout)", "Rithmic: R|Trader Pro, ATAS, Bookmap, EdgeProX, eSignal, Finamark, Investor/RT, MotiveWave, Quantower, Sierra Chart", "Tickblaze"]
drawdown: { amount_usd: "500.00", type: trailing_eod, stops_trailing_at: "Evaluation: trails highest end-of-day balance" }
daily_loss_limit: { amount_usd: null, kind: none }
profit_target_usd: "2000.00"
consistency_rule: { text: "Revamped Fast Track: best single day may not exceed 40% of total profit to qualify (30% at reward)", threshold_pct: 40 }
max_position: { minis: 1, micros: 10, scaling: "1 mini = 10 micros (1 mini = 1 position; 10 micros = 1 position)" }
min_trading_days: 3
trading_day_reset: { time: null, tz_as_published: null }
commission_per_contract: { mnq_usd: "0.51", nq_usd: "2.04", basis: per_side, includes: null }
copy_trading_allowed: conditional
automated_trading_allowed: conditional
automation_rule_text: "If you choose to use a trade copier, AI, bot, EA, algo, or other automated trading tool, you do so entirely at your own risk. The decision and responsibility to use such tools rest solely with the user. AI, bots, EAs, and algos additionally require ETF's written authorization before use. [ToS (ac)] makes use of artificial intelligence (AI), bots, automated trading systems, trade copiers, or other automated trading strategies that are not expressly authorized in writing by ETF."
vps_allowed: no
personal_device_required: unknown
status: partial
not_confirmed_fields: [trading_day_reset, personal_device_required, commission_per_contract.includes, vps_allowed]
notes: "LOW PRIORITY (10K, below 25K). Variant: EOD. Minimum 3 trading days; the evaluation auto-fails if not passed within 10 calendar days and cannot be reset (Fast Track is not eligible for resets). No daily loss limit."
sources:
  - { field: drawdown, url: "https://elitetraderfunding.app/fast-track-evaluation", accessed: "2026-10-23" }
  - { field: profit_target_usd, url: "https://elitetraderfunding.app/fast-track-evaluation", accessed: "2026-10-23" }
  - { field: consistency_rule, url: "https://elitetraderfunding.app/help/how-the-revamped-fast-track-plan-works", accessed: "2026-10-23" }
  - { field: min_trading_days, url: "https://elitetraderfunding.app/fast-track-evaluation", accessed: "2026-10-23" }
  - { field: max_position, url: "https://elitetraderfunding.app/help/contract-limits", accessed: "2026-10-23" }
  - { field: commission_per_contract, url: "https://elitetraderfunding.app/help/guide-to-our-commissions-and-exchanges-we-support", accessed: "2026-10-23" }
  - { field: automation_rule_text, url: "https://elitetraderfunding.app/help/trade-copier-disclaimer", accessed: "2026-10-23" }
  - { field: automation_rule_text, url: "https://elitetraderfunding.app/terms-of-service", accessed: "2026-10-23" }
  - { field: vps_allowed, url: "https://elitetraderfunding.app/help/vpn-and-ip-masking-policy", accessed: "2026-10-23" }
  - { field: platforms, url: "https://elitetraderfunding.app/help/supported-platforms-and-data-feed", accessed: "2026-10-23" }
```

#### Fast Track Static — evaluation — 10K

```yaml
firm: elite-trader-funding
plan: "Fast Track"
phase: evaluation
size_label: "10K"
size_usd: "10000.00"
instruments: [MNQ, NQ]
platforms: ["Tradovate (web/desktop/mobile)", "NinjaTrader 8 (Windows only; Tradovate/CQG or Rithmic credentials)", "TradingView (Tradovate feed; selected at checkout)", "Rithmic: R|Trader Pro, ATAS, Bookmap, EdgeProX, eSignal, Finamark, Investor/RT, MotiveWave, Quantower, Sierra Chart", "Tickblaze"]
drawdown: { amount_usd: "500.00", type: static, stops_trailing_at: "Fixed floor, never moves" }
daily_loss_limit: { amount_usd: null, kind: none }
profit_target_usd: "2000.00"
consistency_rule: { text: "Revamped Fast Track: best single day may not exceed 40% of total profit to qualify (30% at reward)", threshold_pct: 40 }
max_position: { minis: 1, micros: 10, scaling: "1 mini = 10 micros (1 mini = 1 position; 10 micros = 1 position)" }
min_trading_days: 3
trading_day_reset: { time: null, tz_as_published: null }
commission_per_contract: { mnq_usd: "0.51", nq_usd: "2.04", basis: per_side, includes: null }
copy_trading_allowed: conditional
automated_trading_allowed: conditional
automation_rule_text: "If you choose to use a trade copier, AI, bot, EA, algo, or other automated trading tool, you do so entirely at your own risk. The decision and responsibility to use such tools rest solely with the user. AI, bots, EAs, and algos additionally require ETF's written authorization before use. [ToS (ac)] makes use of artificial intelligence (AI), bots, automated trading systems, trade copiers, or other automated trading strategies that are not expressly authorized in writing by ETF."
vps_allowed: no
personal_device_required: unknown
status: partial
not_confirmed_fields: [trading_day_reset, personal_device_required, commission_per_contract.includes, vps_allowed]
notes: "LOW PRIORITY (10K, below 25K). Variant: Static. Minimum 3 trading days; the evaluation auto-fails if not passed within 10 calendar days and cannot be reset (Fast Track is not eligible for resets). No daily loss limit."
sources:
  - { field: drawdown, url: "https://elitetraderfunding.app/fast-track-evaluation", accessed: "2026-10-23" }
  - { field: profit_target_usd, url: "https://elitetraderfunding.app/fast-track-evaluation", accessed: "2026-10-23" }
  - { field: consistency_rule, url: "https://elitetraderfunding.app/help/how-the-revamped-fast-track-plan-works", accessed: "2026-10-23" }
  - { field: min_trading_days, url: "https://elitetraderfunding.app/fast-track-evaluation", accessed: "2026-10-23" }
  - { field: max_position, url: "https://elitetraderfunding.app/help/contract-limits", accessed: "2026-10-23" }
  - { field: commission_per_contract, url: "https://elitetraderfunding.app/help/guide-to-our-commissions-and-exchanges-we-support", accessed: "2026-10-23" }
  - { field: automation_rule_text, url: "https://elitetraderfunding.app/help/trade-copier-disclaimer", accessed: "2026-10-23" }
  - { field: automation_rule_text, url: "https://elitetraderfunding.app/terms-of-service", accessed: "2026-10-23" }
  - { field: vps_allowed, url: "https://elitetraderfunding.app/help/vpn-and-ip-masking-policy", accessed: "2026-10-23" }
  - { field: platforms, url: "https://elitetraderfunding.app/help/supported-platforms-and-data-feed", accessed: "2026-10-23" }
```

#### 1 Step — funded — 50K

```yaml
firm: elite-trader-funding
plan: "1 Step"
phase: funded
size_label: "50K"
size_usd: "50000.00"
instruments: [MNQ, NQ]
platforms: ["Tradovate (web/desktop/mobile)", "NinjaTrader 8 (Windows only; Tradovate/CQG or Rithmic credentials)", "TradingView (Tradovate feed; selected at checkout)", "Rithmic: R|Trader Pro, ATAS, Bookmap, EdgeProX, eSignal, Finamark, Investor/RT, MotiveWave, Quantower, Sierra Chart", "Tickblaze"]
drawdown: { amount_usd: "2000.00", type: trailing_intraday, stops_trailing_at: "Trails highest unrealized profit until realized profits equal max drawdown + $100; then the drawdown is removed and starting balance + $100 becomes the permanent minimum balance" }
daily_loss_limit: { amount_usd: null, kind: none }
profit_target_usd: null
consistency_rule: { text: "No best-day consistency cap on this plan (only Revamped Fast Track has one). ATD qualifier: at least $250 realized profit and at least 30% of best day (evals bought on/after 2026-10-02 2:00 PM PT; $125 minimum on 10K/25K Static)", threshold_pct: null }
max_position: { minis: 8, micros: 80, scaling: "1 mini = 10 micros (1 mini = 1 position; 10 micros = 1 position)" }
min_trading_days: null
trading_day_reset: { time: null, tz_as_published: null }
commission_per_contract: { mnq_usd: "0.51", nq_usd: "2.04", basis: per_side, includes: null }
copy_trading_allowed: conditional
automated_trading_allowed: conditional
automation_rule_text: "If you choose to use a trade copier, AI, bot, EA, algo, or other automated trading tool, you do so entirely at your own risk. The decision and responsibility to use such tools rest solely with the user. AI, bots, EAs, and algos additionally require ETF's written authorization before use. [ToS (ac)] makes use of artificial intelligence (AI), bots, automated trading systems, trade copiers, or other automated trading strategies that are not expressly authorized in writing by ETF."
vps_allowed: no
personal_device_required: unknown
status: partial
not_confirmed_fields: [trading_day_reset, personal_device_required, commission_per_contract.includes, vps_allowed, news rule (conflict ToS vs help article)]
notes: "Same contract limits and drawdown as the evaluation continue after funding (S4). Must be flat 1 minute before close (no overnight). Phase 'funded' = Elite Sim-Funded (simulated funded account; real money is paid out from simulated profits). No profit target. Rewards require Active Trade Days (8 for cycle 1, 10 for cycles 2-4) and the safety net. After +20% profit, giving back more than 35% of accumulated profit removes the account (loss limit rule). Major-news flat rule conflicts between pages (see notes in section 7). Max 20 active funded accounts."
sources:
  - { field: drawdown, url: "https://elitetraderfunding.app/help/how-the-1-step-plan-works", accessed: "2026-10-23" }
  - { field: max_position, url: "https://elitetraderfunding.app/help/contract-limits", accessed: "2026-10-23" }
  - { field: news_rule, url: "https://elitetraderfunding.app/terms-of-service", accessed: "2026-10-23" }
  - { field: commission_per_contract, url: "https://elitetraderfunding.app/help/guide-to-our-commissions-and-exchanges-we-support", accessed: "2026-10-23" }
  - { field: automation_rule_text, url: "https://elitetraderfunding.app/help/trade-copier-disclaimer", accessed: "2026-10-23" }
  - { field: automation_rule_text, url: "https://elitetraderfunding.app/terms-of-service", accessed: "2026-10-23" }
  - { field: vps_allowed, url: "https://elitetraderfunding.app/help/vpn-and-ip-masking-policy", accessed: "2026-10-23" }
  - { field: platforms, url: "https://elitetraderfunding.app/help/supported-platforms-and-data-feed", accessed: "2026-10-23" }
```

#### 1 Step — funded — 100K

```yaml
firm: elite-trader-funding
plan: "1 Step"
phase: funded
size_label: "100K"
size_usd: "100000.00"
instruments: [MNQ, NQ]
platforms: ["Tradovate (web/desktop/mobile)", "NinjaTrader 8 (Windows only; Tradovate/CQG or Rithmic credentials)", "TradingView (Tradovate feed; selected at checkout)", "Rithmic: R|Trader Pro, ATAS, Bookmap, EdgeProX, eSignal, Finamark, Investor/RT, MotiveWave, Quantower, Sierra Chart", "Tickblaze"]
drawdown: { amount_usd: "3000.00", type: trailing_intraday, stops_trailing_at: "Trails highest unrealized profit until realized profits equal max drawdown + $100; then the drawdown is removed and starting balance + $100 becomes the permanent minimum balance" }
daily_loss_limit: { amount_usd: null, kind: none }
profit_target_usd: null
consistency_rule: { text: "No best-day consistency cap on this plan (only Revamped Fast Track has one). ATD qualifier: at least $250 realized profit and at least 30% of best day (evals bought on/after 2026-10-02 2:00 PM PT; $125 minimum on 10K/25K Static)", threshold_pct: null }
max_position: { minis: 14, micros: 140, scaling: "1 mini = 10 micros (1 mini = 1 position; 10 micros = 1 position)" }
min_trading_days: null
trading_day_reset: { time: null, tz_as_published: null }
commission_per_contract: { mnq_usd: "0.51", nq_usd: "2.04", basis: per_side, includes: null }
copy_trading_allowed: conditional
automated_trading_allowed: conditional
automation_rule_text: "If you choose to use a trade copier, AI, bot, EA, algo, or other automated trading tool, you do so entirely at your own risk. The decision and responsibility to use such tools rest solely with the user. AI, bots, EAs, and algos additionally require ETF's written authorization before use. [ToS (ac)] makes use of artificial intelligence (AI), bots, automated trading systems, trade copiers, or other automated trading strategies that are not expressly authorized in writing by ETF."
vps_allowed: no
personal_device_required: unknown
status: partial
not_confirmed_fields: [trading_day_reset, personal_device_required, commission_per_contract.includes, vps_allowed, news rule (conflict ToS vs help article)]
notes: "Same contract limits and drawdown as the evaluation continue after funding (S4). Must be flat 1 minute before close (no overnight). Phase 'funded' = Elite Sim-Funded (simulated funded account; real money is paid out from simulated profits). No profit target. Rewards require Active Trade Days (8 for cycle 1, 10 for cycles 2-4) and the safety net. After +20% profit, giving back more than 35% of accumulated profit removes the account (loss limit rule). Major-news flat rule conflicts between pages (see notes in section 7). Max 20 active funded accounts."
sources:
  - { field: drawdown, url: "https://elitetraderfunding.app/help/how-the-1-step-plan-works", accessed: "2026-10-23" }
  - { field: max_position, url: "https://elitetraderfunding.app/help/contract-limits", accessed: "2026-10-23" }
  - { field: news_rule, url: "https://elitetraderfunding.app/terms-of-service", accessed: "2026-10-23" }
  - { field: commission_per_contract, url: "https://elitetraderfunding.app/help/guide-to-our-commissions-and-exchanges-we-support", accessed: "2026-10-23" }
  - { field: automation_rule_text, url: "https://elitetraderfunding.app/help/trade-copier-disclaimer", accessed: "2026-10-23" }
  - { field: automation_rule_text, url: "https://elitetraderfunding.app/terms-of-service", accessed: "2026-10-23" }
  - { field: vps_allowed, url: "https://elitetraderfunding.app/help/vpn-and-ip-masking-policy", accessed: "2026-10-23" }
  - { field: platforms, url: "https://elitetraderfunding.app/help/supported-platforms-and-data-feed", accessed: "2026-10-23" }
```

#### 1 Step — funded — 150K

```yaml
firm: elite-trader-funding
plan: "1 Step"
phase: funded
size_label: "150K"
size_usd: "150000.00"
instruments: [MNQ, NQ]
platforms: ["Tradovate (web/desktop/mobile)", "NinjaTrader 8 (Windows only; Tradovate/CQG or Rithmic credentials)", "TradingView (Tradovate feed; selected at checkout)", "Rithmic: R|Trader Pro, ATAS, Bookmap, EdgeProX, eSignal, Finamark, Investor/RT, MotiveWave, Quantower, Sierra Chart", "Tickblaze"]
drawdown: { amount_usd: null, type: trailing_intraday, stops_trailing_at: "Trails highest unrealized profit until realized profits equal max drawdown + $100; then the drawdown is removed and starting balance + $100 becomes the permanent minimum balance" }
daily_loss_limit: { amount_usd: null, kind: none }
profit_target_usd: null
consistency_rule: { text: "No best-day consistency cap on this plan (only Revamped Fast Track has one). ATD qualifier: at least $250 realized profit and at least 30% of best day (evals bought on/after 2026-10-02 2:00 PM PT; $125 minimum on 10K/25K Static)", threshold_pct: null }
max_position: { minis: 18, micros: 180, scaling: "1 mini = 10 micros (1 mini = 1 position; 10 micros = 1 position)" }
min_trading_days: null
trading_day_reset: { time: null, tz_as_published: null }
commission_per_contract: { mnq_usd: "0.51", nq_usd: "2.04", basis: per_side, includes: null }
copy_trading_allowed: conditional
automated_trading_allowed: conditional
automation_rule_text: "If you choose to use a trade copier, AI, bot, EA, algo, or other automated trading tool, you do so entirely at your own risk. The decision and responsibility to use such tools rest solely with the user. AI, bots, EAs, and algos additionally require ETF's written authorization before use. [ToS (ac)] makes use of artificial intelligence (AI), bots, automated trading systems, trade copiers, or other automated trading strategies that are not expressly authorized in writing by ETF."
vps_allowed: no
personal_device_required: unknown
status: partial
not_confirmed_fields: [trading_day_reset, personal_device_required, commission_per_contract.includes, vps_allowed, news rule (conflict ToS vs help article), drawdown.amount_usd]
notes: "Same contract limits and drawdown as the evaluation continue after funding (S4). 150K drawdown not published. Must be flat 1 minute before close (no overnight). Phase 'funded' = Elite Sim-Funded (simulated funded account; real money is paid out from simulated profits). No profit target. Rewards require Active Trade Days (8 for cycle 1, 10 for cycles 2-4) and the safety net. After +20% profit, giving back more than 35% of accumulated profit removes the account (loss limit rule). Major-news flat rule conflicts between pages (see notes in section 7). Max 20 active funded accounts."
sources:
  - { field: drawdown, url: "https://elitetraderfunding.app/help/how-the-1-step-plan-works", accessed: "2026-10-23" }
  - { field: max_position, url: "https://elitetraderfunding.app/help/contract-limits", accessed: "2026-10-23" }
  - { field: news_rule, url: "https://elitetraderfunding.app/terms-of-service", accessed: "2026-10-23" }
  - { field: commission_per_contract, url: "https://elitetraderfunding.app/help/guide-to-our-commissions-and-exchanges-we-support", accessed: "2026-10-23" }
  - { field: automation_rule_text, url: "https://elitetraderfunding.app/help/trade-copier-disclaimer", accessed: "2026-10-23" }
  - { field: automation_rule_text, url: "https://elitetraderfunding.app/terms-of-service", accessed: "2026-10-23" }
  - { field: vps_allowed, url: "https://elitetraderfunding.app/help/vpn-and-ip-masking-policy", accessed: "2026-10-23" }
  - { field: platforms, url: "https://elitetraderfunding.app/help/supported-platforms-and-data-feed", accessed: "2026-10-23" }
```

#### 1 Step — funded — 250K

```yaml
firm: elite-trader-funding
plan: "1 Step"
phase: funded
size_label: "250K"
size_usd: "250000.00"
instruments: [MNQ, NQ]
platforms: ["Tradovate (web/desktop/mobile)", "NinjaTrader 8 (Windows only; Tradovate/CQG or Rithmic credentials)", "TradingView (Tradovate feed; selected at checkout)", "Rithmic: R|Trader Pro, ATAS, Bookmap, EdgeProX, eSignal, Finamark, Investor/RT, MotiveWave, Quantower, Sierra Chart", "Tickblaze"]
drawdown: { amount_usd: "6500.00", type: trailing_intraday, stops_trailing_at: "Trails highest unrealized profit until realized profits equal max drawdown + $100; then the drawdown is removed and starting balance + $100 becomes the permanent minimum balance" }
daily_loss_limit: { amount_usd: null, kind: none }
profit_target_usd: null
consistency_rule: { text: "No best-day consistency cap on this plan (only Revamped Fast Track has one). ATD qualifier: at least $250 realized profit and at least 30% of best day (evals bought on/after 2026-10-02 2:00 PM PT; $125 minimum on 10K/25K Static)", threshold_pct: null }
max_position: { minis: 24, micros: 240, scaling: "1 mini = 10 micros (1 mini = 1 position; 10 micros = 1 position)" }
min_trading_days: null
trading_day_reset: { time: null, tz_as_published: null }
commission_per_contract: { mnq_usd: "0.51", nq_usd: "2.04", basis: per_side, includes: null }
copy_trading_allowed: conditional
automated_trading_allowed: conditional
automation_rule_text: "If you choose to use a trade copier, AI, bot, EA, algo, or other automated trading tool, you do so entirely at your own risk. The decision and responsibility to use such tools rest solely with the user. AI, bots, EAs, and algos additionally require ETF's written authorization before use. [ToS (ac)] makes use of artificial intelligence (AI), bots, automated trading systems, trade copiers, or other automated trading strategies that are not expressly authorized in writing by ETF."
vps_allowed: no
personal_device_required: unknown
status: partial
not_confirmed_fields: [trading_day_reset, personal_device_required, commission_per_contract.includes, vps_allowed, news rule (conflict ToS vs help article)]
notes: "Same contract limits and drawdown as the evaluation continue after funding (S4). 250K marked 'no longer sold, existing accounts only' in Contract Limits. Must be flat 1 minute before close (no overnight). Phase 'funded' = Elite Sim-Funded (simulated funded account; real money is paid out from simulated profits). No profit target. Rewards require Active Trade Days (8 for cycle 1, 10 for cycles 2-4) and the safety net. After +20% profit, giving back more than 35% of accumulated profit removes the account (loss limit rule). Major-news flat rule conflicts between pages (see notes in section 7). Max 20 active funded accounts."
sources:
  - { field: drawdown, url: "https://elitetraderfunding.app/help/how-the-1-step-plan-works", accessed: "2026-10-23" }
  - { field: max_position, url: "https://elitetraderfunding.app/help/contract-limits", accessed: "2026-10-23" }
  - { field: news_rule, url: "https://elitetraderfunding.app/terms-of-service", accessed: "2026-10-23" }
  - { field: commission_per_contract, url: "https://elitetraderfunding.app/help/guide-to-our-commissions-and-exchanges-we-support", accessed: "2026-10-23" }
  - { field: automation_rule_text, url: "https://elitetraderfunding.app/help/trade-copier-disclaimer", accessed: "2026-10-23" }
  - { field: automation_rule_text, url: "https://elitetraderfunding.app/terms-of-service", accessed: "2026-10-23" }
  - { field: vps_allowed, url: "https://elitetraderfunding.app/help/vpn-and-ip-masking-policy", accessed: "2026-10-23" }
  - { field: platforms, url: "https://elitetraderfunding.app/help/supported-platforms-and-data-feed", accessed: "2026-10-23" }
```

#### End of Day — funded — 50K

```yaml
firm: elite-trader-funding
plan: "End of Day"
phase: funded
size_label: "50K"
size_usd: "50000.00"
instruments: [MNQ, NQ]
platforms: ["Tradovate (web/desktop/mobile)", "NinjaTrader 8 (Windows only; Tradovate/CQG or Rithmic credentials)", "TradingView (Tradovate feed; selected at checkout)", "Rithmic: R|Trader Pro, ATAS, Bookmap, EdgeProX, eSignal, Finamark, Investor/RT, MotiveWave, Quantower, Sierra Chart", "Tickblaze"]
drawdown: { amount_usd: "2000.00", type: trailing_eod, stops_trailing_at: "Trails highest end-of-day balance until realized profits equal max drawdown + $100; then locks permanently at starting balance + $100 and the daily loss limit is removed" }
daily_loss_limit: { amount_usd: "1100.00", kind: hard }
profit_target_usd: null
consistency_rule: { text: "No best-day consistency cap on this plan (only Revamped Fast Track has one). ATD qualifier: at least $250 realized profit and at least 30% of best day (evals bought on/after 2026-10-02 2:00 PM PT; $125 minimum on 10K/25K Static)", threshold_pct: null }
max_position: { minis: 8, micros: 80, scaling: "1 mini = 10 micros (1 mini = 1 position; 10 micros = 1 position)" }
min_trading_days: null
trading_day_reset: { time: null, tz_as_published: null }
commission_per_contract: { mnq_usd: "0.51", nq_usd: "2.04", basis: per_side, includes: null }
copy_trading_allowed: conditional
automated_trading_allowed: conditional
automation_rule_text: "If you choose to use a trade copier, AI, bot, EA, algo, or other automated trading tool, you do so entirely at your own risk. The decision and responsibility to use such tools rest solely with the user. AI, bots, EAs, and algos additionally require ETF's written authorization before use. [ToS (ac)] makes use of artificial intelligence (AI), bots, automated trading systems, trade copiers, or other automated trading strategies that are not expressly authorized in writing by ETF."
vps_allowed: no
personal_device_required: unknown
status: partial
not_confirmed_fields: [trading_day_reset, personal_device_required, commission_per_contract.includes, vps_allowed, news rule (conflict ToS vs help article)]
notes: "Daily loss limit (hard) is removed once the safety net is reached. Must be flat 1 minute before close (no overnight). Phase 'funded' = Elite Sim-Funded (simulated funded account; real money is paid out from simulated profits). No profit target. Rewards require Active Trade Days (8 for cycle 1, 10 for cycles 2-4) and the safety net. After +20% profit, giving back more than 35% of accumulated profit removes the account (loss limit rule). Major-news flat rule conflicts between pages (see notes in section 7). Max 20 active funded accounts."
sources:
  - { field: drawdown, url: "https://elitetraderfunding.app/help/how-the-end-of-day-plan-works", accessed: "2026-10-23" }
  - { field: max_position, url: "https://elitetraderfunding.app/help/contract-limits", accessed: "2026-10-23" }
  - { field: news_rule, url: "https://elitetraderfunding.app/terms-of-service", accessed: "2026-10-23" }
  - { field: commission_per_contract, url: "https://elitetraderfunding.app/help/guide-to-our-commissions-and-exchanges-we-support", accessed: "2026-10-23" }
  - { field: automation_rule_text, url: "https://elitetraderfunding.app/help/trade-copier-disclaimer", accessed: "2026-10-23" }
  - { field: automation_rule_text, url: "https://elitetraderfunding.app/terms-of-service", accessed: "2026-10-23" }
  - { field: vps_allowed, url: "https://elitetraderfunding.app/help/vpn-and-ip-masking-policy", accessed: "2026-10-23" }
  - { field: platforms, url: "https://elitetraderfunding.app/help/supported-platforms-and-data-feed", accessed: "2026-10-23" }
```

#### End of Day — funded — 100K

```yaml
firm: elite-trader-funding
plan: "End of Day"
phase: funded
size_label: "100K"
size_usd: "100000.00"
instruments: [MNQ, NQ]
platforms: ["Tradovate (web/desktop/mobile)", "NinjaTrader 8 (Windows only; Tradovate/CQG or Rithmic credentials)", "TradingView (Tradovate feed; selected at checkout)", "Rithmic: R|Trader Pro, ATAS, Bookmap, EdgeProX, eSignal, Finamark, Investor/RT, MotiveWave, Quantower, Sierra Chart", "Tickblaze"]
drawdown: { amount_usd: "3500.00", type: trailing_eod, stops_trailing_at: "Trails highest end-of-day balance until realized profits equal max drawdown + $100; then locks permanently at starting balance + $100 and the daily loss limit is removed" }
daily_loss_limit: { amount_usd: "2200.00", kind: hard }
profit_target_usd: null
consistency_rule: { text: "No best-day consistency cap on this plan (only Revamped Fast Track has one). ATD qualifier: at least $250 realized profit and at least 30% of best day (evals bought on/after 2026-10-02 2:00 PM PT; $125 minimum on 10K/25K Static)", threshold_pct: null }
max_position: { minis: 14, micros: 140, scaling: "1 mini = 10 micros (1 mini = 1 position; 10 micros = 1 position)" }
min_trading_days: null
trading_day_reset: { time: null, tz_as_published: null }
commission_per_contract: { mnq_usd: "0.51", nq_usd: "2.04", basis: per_side, includes: null }
copy_trading_allowed: conditional
automated_trading_allowed: conditional
automation_rule_text: "If you choose to use a trade copier, AI, bot, EA, algo, or other automated trading tool, you do so entirely at your own risk. The decision and responsibility to use such tools rest solely with the user. AI, bots, EAs, and algos additionally require ETF's written authorization before use. [ToS (ac)] makes use of artificial intelligence (AI), bots, automated trading systems, trade copiers, or other automated trading strategies that are not expressly authorized in writing by ETF."
vps_allowed: no
personal_device_required: unknown
status: partial
not_confirmed_fields: [trading_day_reset, personal_device_required, commission_per_contract.includes, vps_allowed, news rule (conflict ToS vs help article)]
notes: "Daily loss limit (hard) is removed once the safety net is reached. Must be flat 1 minute before close (no overnight). Phase 'funded' = Elite Sim-Funded (simulated funded account; real money is paid out from simulated profits). No profit target. Rewards require Active Trade Days (8 for cycle 1, 10 for cycles 2-4) and the safety net. After +20% profit, giving back more than 35% of accumulated profit removes the account (loss limit rule). Major-news flat rule conflicts between pages (see notes in section 7). Max 20 active funded accounts."
sources:
  - { field: drawdown, url: "https://elitetraderfunding.app/help/how-the-end-of-day-plan-works", accessed: "2026-10-23" }
  - { field: max_position, url: "https://elitetraderfunding.app/help/contract-limits", accessed: "2026-10-23" }
  - { field: news_rule, url: "https://elitetraderfunding.app/terms-of-service", accessed: "2026-10-23" }
  - { field: commission_per_contract, url: "https://elitetraderfunding.app/help/guide-to-our-commissions-and-exchanges-we-support", accessed: "2026-10-23" }
  - { field: automation_rule_text, url: "https://elitetraderfunding.app/help/trade-copier-disclaimer", accessed: "2026-10-23" }
  - { field: automation_rule_text, url: "https://elitetraderfunding.app/terms-of-service", accessed: "2026-10-23" }
  - { field: vps_allowed, url: "https://elitetraderfunding.app/help/vpn-and-ip-masking-policy", accessed: "2026-10-23" }
  - { field: platforms, url: "https://elitetraderfunding.app/help/supported-platforms-and-data-feed", accessed: "2026-10-23" }
```

#### End of Day — funded — 150K

```yaml
firm: elite-trader-funding
plan: "End of Day"
phase: funded
size_label: "150K"
size_usd: "150000.00"
instruments: [MNQ, NQ]
platforms: ["Tradovate (web/desktop/mobile)", "NinjaTrader 8 (Windows only; Tradovate/CQG or Rithmic credentials)", "TradingView (Tradovate feed; selected at checkout)", "Rithmic: R|Trader Pro, ATAS, Bookmap, EdgeProX, eSignal, Finamark, Investor/RT, MotiveWave, Quantower, Sierra Chart", "Tickblaze"]
drawdown: { amount_usd: null, type: trailing_eod, stops_trailing_at: "Trails highest end-of-day balance until realized profits equal max drawdown + $100; then locks permanently at starting balance + $100 and the daily loss limit is removed" }
daily_loss_limit: { amount_usd: null, kind: none }
profit_target_usd: null
consistency_rule: { text: "No best-day consistency cap on this plan (only Revamped Fast Track has one). ATD qualifier: at least $250 realized profit and at least 30% of best day (evals bought on/after 2026-10-02 2:00 PM PT; $125 minimum on 10K/25K Static)", threshold_pct: null }
max_position: { minis: 18, micros: 180, scaling: "1 mini = 10 micros (1 mini = 1 position; 10 micros = 1 position)" }
min_trading_days: null
trading_day_reset: { time: null, tz_as_published: null }
commission_per_contract: { mnq_usd: "0.51", nq_usd: "2.04", basis: per_side, includes: null }
copy_trading_allowed: conditional
automated_trading_allowed: conditional
automation_rule_text: "If you choose to use a trade copier, AI, bot, EA, algo, or other automated trading tool, you do so entirely at your own risk. The decision and responsibility to use such tools rest solely with the user. AI, bots, EAs, and algos additionally require ETF's written authorization before use. [ToS (ac)] makes use of artificial intelligence (AI), bots, automated trading systems, trade copiers, or other automated trading strategies that are not expressly authorized in writing by ETF."
vps_allowed: no
personal_device_required: unknown
status: partial
not_confirmed_fields: [trading_day_reset, personal_device_required, commission_per_contract.includes, vps_allowed, news rule (conflict ToS vs help article), drawdown.amount_usd, daily_loss_limit.amount_usd]
notes: "Daily loss limit (hard) is removed once the safety net is reached. 150K drawdown and daily loss limit not published. Must be flat 1 minute before close (no overnight). Phase 'funded' = Elite Sim-Funded (simulated funded account; real money is paid out from simulated profits). No profit target. Rewards require Active Trade Days (8 for cycle 1, 10 for cycles 2-4) and the safety net. After +20% profit, giving back more than 35% of accumulated profit removes the account (loss limit rule). Major-news flat rule conflicts between pages (see notes in section 7). Max 20 active funded accounts."
sources:
  - { field: drawdown, url: "https://elitetraderfunding.app/help/how-the-end-of-day-plan-works", accessed: "2026-10-23" }
  - { field: max_position, url: "https://elitetraderfunding.app/help/contract-limits", accessed: "2026-10-23" }
  - { field: news_rule, url: "https://elitetraderfunding.app/terms-of-service", accessed: "2026-10-23" }
  - { field: commission_per_contract, url: "https://elitetraderfunding.app/help/guide-to-our-commissions-and-exchanges-we-support", accessed: "2026-10-23" }
  - { field: automation_rule_text, url: "https://elitetraderfunding.app/help/trade-copier-disclaimer", accessed: "2026-10-23" }
  - { field: automation_rule_text, url: "https://elitetraderfunding.app/terms-of-service", accessed: "2026-10-23" }
  - { field: vps_allowed, url: "https://elitetraderfunding.app/help/vpn-and-ip-masking-policy", accessed: "2026-10-23" }
  - { field: platforms, url: "https://elitetraderfunding.app/help/supported-platforms-and-data-feed", accessed: "2026-10-23" }
```

#### Static Drawdown — funded — 25K

```yaml
firm: elite-trader-funding
plan: "Static Drawdown"
phase: funded
size_label: "25K"
size_usd: "25000.00"
instruments: [MNQ, NQ]
platforms: ["Tradovate (web/desktop/mobile)", "NinjaTrader 8 (Windows only; Tradovate/CQG or Rithmic credentials)", "TradingView (Tradovate feed; selected at checkout)", "Rithmic: R|Trader Pro, ATAS, Bookmap, EdgeProX, eSignal, Finamark, Investor/RT, MotiveWave, Quantower, Sierra Chart", "Tickblaze"]
drawdown: { amount_usd: "1000.00", type: static, stops_trailing_at: "Never trails; safety net (max drawdown + $100 realized profit) is still required for rewards" }
daily_loss_limit: { amount_usd: null, kind: none }
profit_target_usd: null
consistency_rule: { text: "No best-day consistency cap on this plan (only Revamped Fast Track has one). ATD qualifier: at least $250 realized profit and at least 30% of best day (evals bought on/after 2026-10-02 2:00 PM PT; $125 minimum on 10K/25K Static)", threshold_pct: null }
max_position: { minis: 2, micros: 20, scaling: "1 mini = 10 micros (1 mini = 1 position; 10 micros = 1 position)" }
min_trading_days: null
trading_day_reset: { time: null, tz_as_published: null }
commission_per_contract: { mnq_usd: "0.51", nq_usd: "2.04", basis: per_side, includes: null }
copy_trading_allowed: conditional
automated_trading_allowed: conditional
automation_rule_text: "If you choose to use a trade copier, AI, bot, EA, algo, or other automated trading tool, you do so entirely at your own risk. The decision and responsibility to use such tools rest solely with the user. AI, bots, EAs, and algos additionally require ETF's written authorization before use. [ToS (ac)] makes use of artificial intelligence (AI), bots, automated trading systems, trade copiers, or other automated trading strategies that are not expressly authorized in writing by ETF."
vps_allowed: no
personal_device_required: unknown
status: partial
not_confirmed_fields: [trading_day_reset, personal_device_required, commission_per_contract.includes, vps_allowed, news rule (conflict ToS vs help article), drawdown (min balance 24100 vs 24000)]
notes: "Fixed minimum balance. CONFLICT: Safety Net table lists 25K Static minimum balance $24,100 while the Static plan page says the floor is $24,000 (25000-1000); drawdown kept as published on the plan page. Must be flat 1 minute before close (no overnight). Phase 'funded' = Elite Sim-Funded (simulated funded account; real money is paid out from simulated profits). No profit target. Rewards require Active Trade Days (8 for cycle 1, 10 for cycles 2-4) and the safety net. After +20% profit, giving back more than 35% of accumulated profit removes the account (loss limit rule). Major-news flat rule conflicts between pages (see notes in section 7). Max 20 active funded accounts."
sources:
  - { field: drawdown, url: "https://elitetraderfunding.app/static-drawdown", accessed: "2026-10-23" }
  - { field: max_position, url: "https://elitetraderfunding.app/help/contract-limits", accessed: "2026-10-23" }
  - { field: news_rule, url: "https://elitetraderfunding.app/terms-of-service", accessed: "2026-10-23" }
  - { field: commission_per_contract, url: "https://elitetraderfunding.app/help/guide-to-our-commissions-and-exchanges-we-support", accessed: "2026-10-23" }
  - { field: automation_rule_text, url: "https://elitetraderfunding.app/help/trade-copier-disclaimer", accessed: "2026-10-23" }
  - { field: automation_rule_text, url: "https://elitetraderfunding.app/terms-of-service", accessed: "2026-10-23" }
  - { field: vps_allowed, url: "https://elitetraderfunding.app/help/vpn-and-ip-masking-policy", accessed: "2026-10-23" }
  - { field: platforms, url: "https://elitetraderfunding.app/help/supported-platforms-and-data-feed", accessed: "2026-10-23" }
```

#### Static Drawdown — funded — 50K

```yaml
firm: elite-trader-funding
plan: "Static Drawdown"
phase: funded
size_label: "50K"
size_usd: "50000.00"
instruments: [MNQ, NQ]
platforms: ["Tradovate (web/desktop/mobile)", "NinjaTrader 8 (Windows only; Tradovate/CQG or Rithmic credentials)", "TradingView (Tradovate feed; selected at checkout)", "Rithmic: R|Trader Pro, ATAS, Bookmap, EdgeProX, eSignal, Finamark, Investor/RT, MotiveWave, Quantower, Sierra Chart", "Tickblaze"]
drawdown: { amount_usd: "2000.00", type: static, stops_trailing_at: "Never trails; safety net (max drawdown + $100 realized profit) is still required for rewards" }
daily_loss_limit: { amount_usd: null, kind: none }
profit_target_usd: null
consistency_rule: { text: "No best-day consistency cap on this plan (only Revamped Fast Track has one). ATD qualifier: at least $250 realized profit and at least 30% of best day (evals bought on/after 2026-10-02 2:00 PM PT; $125 minimum on 10K/25K Static)", threshold_pct: null }
max_position: { minis: 4, micros: 40, scaling: "1 mini = 10 micros (1 mini = 1 position; 10 micros = 1 position)" }
min_trading_days: null
trading_day_reset: { time: null, tz_as_published: null }
commission_per_contract: { mnq_usd: "0.51", nq_usd: "2.04", basis: per_side, includes: null }
copy_trading_allowed: conditional
automated_trading_allowed: conditional
automation_rule_text: "If you choose to use a trade copier, AI, bot, EA, algo, or other automated trading tool, you do so entirely at your own risk. The decision and responsibility to use such tools rest solely with the user. AI, bots, EAs, and algos additionally require ETF's written authorization before use. [ToS (ac)] makes use of artificial intelligence (AI), bots, automated trading systems, trade copiers, or other automated trading strategies that are not expressly authorized in writing by ETF."
vps_allowed: no
personal_device_required: unknown
status: partial
not_confirmed_fields: [trading_day_reset, personal_device_required, commission_per_contract.includes, vps_allowed, news rule (conflict ToS vs help article)]
notes: "Fixed minimum balance. Must be flat 1 minute before close (no overnight). Phase 'funded' = Elite Sim-Funded (simulated funded account; real money is paid out from simulated profits). No profit target. Rewards require Active Trade Days (8 for cycle 1, 10 for cycles 2-4) and the safety net. After +20% profit, giving back more than 35% of accumulated profit removes the account (loss limit rule). Major-news flat rule conflicts between pages (see notes in section 7). Max 20 active funded accounts."
sources:
  - { field: drawdown, url: "https://elitetraderfunding.app/static-drawdown", accessed: "2026-10-23" }
  - { field: max_position, url: "https://elitetraderfunding.app/help/contract-limits", accessed: "2026-10-23" }
  - { field: news_rule, url: "https://elitetraderfunding.app/terms-of-service", accessed: "2026-10-23" }
  - { field: commission_per_contract, url: "https://elitetraderfunding.app/help/guide-to-our-commissions-and-exchanges-we-support", accessed: "2026-10-23" }
  - { field: automation_rule_text, url: "https://elitetraderfunding.app/help/trade-copier-disclaimer", accessed: "2026-10-23" }
  - { field: automation_rule_text, url: "https://elitetraderfunding.app/terms-of-service", accessed: "2026-10-23" }
  - { field: vps_allowed, url: "https://elitetraderfunding.app/help/vpn-and-ip-masking-policy", accessed: "2026-10-23" }
  - { field: platforms, url: "https://elitetraderfunding.app/help/supported-platforms-and-data-feed", accessed: "2026-10-23" }
```

#### Diamond Hands — funded — 100K

```yaml
firm: elite-trader-funding
plan: "Diamond Hands"
phase: funded
size_label: "100K"
size_usd: "100000.00"
instruments: [MNQ, NQ]
platforms: ["Tradovate (web/desktop/mobile)", "NinjaTrader 8 (Windows only; Tradovate/CQG or Rithmic credentials)", "TradingView (Tradovate feed; selected at checkout)", "Rithmic: R|Trader Pro, ATAS, Bookmap, EdgeProX, eSignal, Finamark, Investor/RT, MotiveWave, Quantower, Sierra Chart", "Tickblaze"]
drawdown: { amount_usd: "3500.00", type: trailing_eod, stops_trailing_at: "Trails highest end-of-day balance until realized profits equal max drawdown + $100; then locks permanently at starting balance + $100 and the daily loss limit is removed" }
daily_loss_limit: { amount_usd: "1500.00", kind: hard }
profit_target_usd: null
consistency_rule: { text: "No best-day consistency cap on this plan (only Revamped Fast Track has one). ATD qualifier: at least $250 realized profit and at least 30% of best day (evals bought on/after 2026-10-02 2:00 PM PT; $125 minimum on 10K/25K Static)", threshold_pct: null }
max_position: { minis: 2, micros: 20, scaling: "1 mini = 10 micros (1 mini = 1 position; 10 micros = 1 position)" }
min_trading_days: null
trading_day_reset: { time: null, tz_as_published: null }
commission_per_contract: { mnq_usd: "0.51", nq_usd: "2.04", basis: per_side, includes: null }
copy_trading_allowed: conditional
automated_trading_allowed: conditional
automation_rule_text: "If you choose to use a trade copier, AI, bot, EA, algo, or other automated trading tool, you do so entirely at your own risk. The decision and responsibility to use such tools rest solely with the user. AI, bots, EAs, and algos additionally require ETF's written authorization before use. [ToS (ac)] makes use of artificial intelligence (AI), bots, automated trading systems, trade copiers, or other automated trading strategies that are not expressly authorized in writing by ETF."
vps_allowed: no
personal_device_required: unknown
status: partial
not_confirmed_fields: [trading_day_reset, personal_device_required, commission_per_contract.includes, vps_allowed, news rule (conflict ToS vs help article)]
notes: "Positions may be held overnight and over the weekend. Overnight and weekend holds allowed on Diamond Hands. Phase 'funded' = Elite Sim-Funded (simulated funded account; real money is paid out from simulated profits). No profit target. Rewards require Active Trade Days (8 for cycle 1, 10 for cycles 2-4) and the safety net. After +20% profit, giving back more than 35% of accumulated profit removes the account (loss limit rule). Major-news flat rule conflicts between pages (see notes in section 7). Max 20 active funded accounts."
sources:
  - { field: drawdown, url: "https://elitetraderfunding.app/help/how-the-revamped-diamond-hands-plan-works", accessed: "2026-10-23" }
  - { field: max_position, url: "https://elitetraderfunding.app/help/contract-limits", accessed: "2026-10-23" }
  - { field: news_rule, url: "https://elitetraderfunding.app/terms-of-service", accessed: "2026-10-23" }
  - { field: commission_per_contract, url: "https://elitetraderfunding.app/help/guide-to-our-commissions-and-exchanges-we-support", accessed: "2026-10-23" }
  - { field: automation_rule_text, url: "https://elitetraderfunding.app/help/trade-copier-disclaimer", accessed: "2026-10-23" }
  - { field: automation_rule_text, url: "https://elitetraderfunding.app/terms-of-service", accessed: "2026-10-23" }
  - { field: vps_allowed, url: "https://elitetraderfunding.app/help/vpn-and-ip-masking-policy", accessed: "2026-10-23" }
  - { field: platforms, url: "https://elitetraderfunding.app/help/supported-platforms-and-data-feed", accessed: "2026-10-23" }
```

#### Direct to Funded — funded — 10K Mini Inferno

```yaml
firm: elite-trader-funding
plan: "Direct to Funded"
phase: funded
size_label: "10K Mini Inferno"
size_usd: "10000.00"
instruments: [MNQ, NQ]
platforms: ["Tradovate (web/desktop/mobile)", "NinjaTrader 8 (Windows only; Tradovate/CQG or Rithmic credentials)", "TradingView (Tradovate feed; selected at checkout)", "Rithmic: R|Trader Pro, ATAS, Bookmap, EdgeProX, eSignal, Finamark, Investor/RT, MotiveWave, Quantower, Sierra Chart", "Tickblaze"]
drawdown: { amount_usd: "1500.00", type: trailing_eod, stops_trailing_at: "Trails highest end-of-day balance until realized profits equal max drawdown + $100; then locks permanently at starting balance + $100 and the daily loss limit is removed" }
daily_loss_limit: { amount_usd: null, kind: none }
profit_target_usd: null
consistency_rule: { text: "No best-day cap; ATD qualifier only (DTF: 10K $150 & 50% of best day; 50K $600 & 62%; 100K $500 & 50%)", threshold_pct: null }
max_position: { minis: 2, micros: 2, scaling: "1 mini = 1 micro (no 10:1 micro scaling on DTF)" }
min_trading_days: null
trading_day_reset: { time: null, tz_as_published: null }
commission_per_contract: { mnq_usd: "0.51", nq_usd: "2.04", basis: per_side, includes: null }
copy_trading_allowed: conditional
automated_trading_allowed: conditional
automation_rule_text: "If you choose to use a trade copier, AI, bot, EA, algo, or other automated trading tool, you do so entirely at your own risk. The decision and responsibility to use such tools rest solely with the user. AI, bots, EAs, and algos additionally require ETF's written authorization before use. [ToS (ac)] makes use of artificial intelligence (AI), bots, automated trading systems, trade copiers, or other automated trading strategies that are not expressly authorized in writing by ETF."
vps_allowed: no
personal_device_required: unknown
status: partial
not_confirmed_fields: [trading_day_reset, personal_device_required, commission_per_contract.includes, vps_allowed, news rule (conflict ToS vs help article)]
notes: "No daily loss limit.  Direct to Funded has no evaluation: it starts as an Elite Sim-Funded account, 1 mini = 1 micro for position math, one-time purchase. The page label 'Profit target' on the DTF page is the safety net; the DTF help article says 'DTF accounts have no profit target'. Swing trading allowed (overnight/weekend). Rewards need ATDs (5/10/15/20 by size) and the safety net. LOW PRIORITY (below 25K). Flatten rule does not apply (holds allowed). Major-news flat rule conflicts between pages. Rewards require Active Trade Days (8 for cycle 1, 10 for cycles 2-4) and the safety net. After +20% profit, giving back more than 35% of accumulated profit removes the account (loss limit rule). Major-news flat rule conflicts between pages (see notes in section 7). Max 20 active funded accounts."
sources:
  - { field: drawdown, url: "https://elitetraderfunding.app/direct-to-funded", accessed: "2026-10-23" }
  - { field: max_position, url: "https://elitetraderfunding.app/direct-to-funded", accessed: "2026-10-23" }
  - { field: consistency_rule, url: "https://elitetraderfunding.app/help/how-the-dtf-plan-works", accessed: "2026-10-23" }
  - { field: profit_target_usd, url: "https://elitetraderfunding.app/help/how-the-dtf-plan-works", accessed: "2026-10-23" }
  - { field: commission_per_contract, url: "https://elitetraderfunding.app/help/guide-to-our-commissions-and-exchanges-we-support", accessed: "2026-10-23" }
  - { field: automation_rule_text, url: "https://elitetraderfunding.app/help/trade-copier-disclaimer", accessed: "2026-10-23" }
  - { field: automation_rule_text, url: "https://elitetraderfunding.app/terms-of-service", accessed: "2026-10-23" }
  - { field: vps_allowed, url: "https://elitetraderfunding.app/help/vpn-and-ip-masking-policy", accessed: "2026-10-23" }
  - { field: platforms, url: "https://elitetraderfunding.app/help/supported-platforms-and-data-feed", accessed: "2026-10-23" }
```

#### Direct to Funded — funded — 50K

```yaml
firm: elite-trader-funding
plan: "Direct to Funded"
phase: funded
size_label: "50K"
size_usd: "50000.00"
instruments: [MNQ, NQ]
platforms: ["Tradovate (web/desktop/mobile)", "NinjaTrader 8 (Windows only; Tradovate/CQG or Rithmic credentials)", "TradingView (Tradovate feed; selected at checkout)", "Rithmic: R|Trader Pro, ATAS, Bookmap, EdgeProX, eSignal, Finamark, Investor/RT, MotiveWave, Quantower, Sierra Chart", "Tickblaze"]
drawdown: { amount_usd: "5000.00", type: trailing_eod, stops_trailing_at: "Trails highest end-of-day balance until realized profits equal max drawdown + $100; then locks permanently at starting balance + $100 and the daily loss limit is removed" }
daily_loss_limit: { amount_usd: null, kind: none }
profit_target_usd: null
consistency_rule: { text: "No best-day cap; ATD qualifier only (DTF: 10K $150 & 50% of best day; 50K $600 & 62%; 100K $500 & 50%)", threshold_pct: null }
max_position: { minis: 5, micros: 5, scaling: "1 mini = 1 micro (no 10:1 micro scaling on DTF)" }
min_trading_days: null
trading_day_reset: { time: null, tz_as_published: null }
commission_per_contract: { mnq_usd: "0.51", nq_usd: "2.04", basis: per_side, includes: null }
copy_trading_allowed: conditional
automated_trading_allowed: conditional
automation_rule_text: "If you choose to use a trade copier, AI, bot, EA, algo, or other automated trading tool, you do so entirely at your own risk. The decision and responsibility to use such tools rest solely with the user. AI, bots, EAs, and algos additionally require ETF's written authorization before use. [ToS (ac)] makes use of artificial intelligence (AI), bots, automated trading systems, trade copiers, or other automated trading strategies that are not expressly authorized in writing by ETF."
vps_allowed: no
personal_device_required: unknown
status: partial
not_confirmed_fields: [trading_day_reset, personal_device_required, commission_per_contract.includes, vps_allowed, news rule (conflict ToS vs help article)]
notes: "No daily loss limit.  Direct to Funded has no evaluation: it starts as an Elite Sim-Funded account, 1 mini = 1 micro for position math, one-time purchase. The page label 'Profit target' on the DTF page is the safety net; the DTF help article says 'DTF accounts have no profit target'. Swing trading allowed (overnight/weekend). Rewards need ATDs (5/10/15/20 by size) and the safety net. Flatten rule does not apply (holds allowed). Major-news flat rule conflicts between pages. Rewards require Active Trade Days (8 for cycle 1, 10 for cycles 2-4) and the safety net. After +20% profit, giving back more than 35% of accumulated profit removes the account (loss limit rule). Major-news flat rule conflicts between pages (see notes in section 7). Max 20 active funded accounts."
sources:
  - { field: drawdown, url: "https://elitetraderfunding.app/direct-to-funded", accessed: "2026-10-23" }
  - { field: max_position, url: "https://elitetraderfunding.app/direct-to-funded", accessed: "2026-10-23" }
  - { field: consistency_rule, url: "https://elitetraderfunding.app/help/how-the-dtf-plan-works", accessed: "2026-10-23" }
  - { field: profit_target_usd, url: "https://elitetraderfunding.app/help/how-the-dtf-plan-works", accessed: "2026-10-23" }
  - { field: commission_per_contract, url: "https://elitetraderfunding.app/help/guide-to-our-commissions-and-exchanges-we-support", accessed: "2026-10-23" }
  - { field: automation_rule_text, url: "https://elitetraderfunding.app/help/trade-copier-disclaimer", accessed: "2026-10-23" }
  - { field: automation_rule_text, url: "https://elitetraderfunding.app/terms-of-service", accessed: "2026-10-23" }
  - { field: vps_allowed, url: "https://elitetraderfunding.app/help/vpn-and-ip-masking-policy", accessed: "2026-10-23" }
  - { field: platforms, url: "https://elitetraderfunding.app/help/supported-platforms-and-data-feed", accessed: "2026-10-23" }
```

#### Direct to Funded — funded — 100K

```yaml
firm: elite-trader-funding
plan: "Direct to Funded"
phase: funded
size_label: "100K"
size_usd: "100000.00"
instruments: [MNQ, NQ]
platforms: ["Tradovate (web/desktop/mobile)", "NinjaTrader 8 (Windows only; Tradovate/CQG or Rithmic credentials)", "TradingView (Tradovate feed; selected at checkout)", "Rithmic: R|Trader Pro, ATAS, Bookmap, EdgeProX, eSignal, Finamark, Investor/RT, MotiveWave, Quantower, Sierra Chart", "Tickblaze"]
drawdown: { amount_usd: "5000.00", type: static, stops_trailing_at: "Fixed floor (static), never moves; safety net (max drawdown + $100 realized profit) still required for rewards" }
daily_loss_limit: { amount_usd: null, kind: none }
profit_target_usd: null
consistency_rule: { text: "No best-day cap; ATD qualifier only (DTF: 10K $150 & 50% of best day; 50K $600 & 62%; 100K $500 & 50%)", threshold_pct: null }
max_position: { minis: 3, micros: 3, scaling: "1 mini = 1 micro (no 10:1 micro scaling on DTF)" }
min_trading_days: null
trading_day_reset: { time: null, tz_as_published: null }
commission_per_contract: { mnq_usd: "0.51", nq_usd: "2.04", basis: per_side, includes: null }
copy_trading_allowed: conditional
automated_trading_allowed: conditional
automation_rule_text: "If you choose to use a trade copier, AI, bot, EA, algo, or other automated trading tool, you do so entirely at your own risk. The decision and responsibility to use such tools rest solely with the user. AI, bots, EAs, and algos additionally require ETF's written authorization before use. [ToS (ac)] makes use of artificial intelligence (AI), bots, automated trading systems, trade copiers, or other automated trading strategies that are not expressly authorized in writing by ETF."
vps_allowed: no
personal_device_required: unknown
status: partial
not_confirmed_fields: [trading_day_reset, personal_device_required, commission_per_contract.includes, vps_allowed, news rule (conflict ToS vs help article)]
notes: "No daily loss limit.  Direct to Funded has no evaluation: it starts as an Elite Sim-Funded account, 1 mini = 1 micro for position math, one-time purchase. The page label 'Profit target' on the DTF page is the safety net; the DTF help article says 'DTF accounts have no profit target'. Swing trading allowed (overnight/weekend). Rewards need ATDs (5/10/15/20 by size) and the safety net. Flatten rule does not apply (holds allowed). Major-news flat rule conflicts between pages. Rewards require Active Trade Days (8 for cycle 1, 10 for cycles 2-4) and the safety net. After +20% profit, giving back more than 35% of accumulated profit removes the account (loss limit rule). Major-news flat rule conflicts between pages (see notes in section 7). Max 20 active funded accounts."
sources:
  - { field: drawdown, url: "https://elitetraderfunding.app/direct-to-funded", accessed: "2026-10-23" }
  - { field: max_position, url: "https://elitetraderfunding.app/direct-to-funded", accessed: "2026-10-23" }
  - { field: consistency_rule, url: "https://elitetraderfunding.app/help/how-the-dtf-plan-works", accessed: "2026-10-23" }
  - { field: profit_target_usd, url: "https://elitetraderfunding.app/help/how-the-dtf-plan-works", accessed: "2026-10-23" }
  - { field: commission_per_contract, url: "https://elitetraderfunding.app/help/guide-to-our-commissions-and-exchanges-we-support", accessed: "2026-10-23" }
  - { field: automation_rule_text, url: "https://elitetraderfunding.app/help/trade-copier-disclaimer", accessed: "2026-10-23" }
  - { field: automation_rule_text, url: "https://elitetraderfunding.app/terms-of-service", accessed: "2026-10-23" }
  - { field: vps_allowed, url: "https://elitetraderfunding.app/help/vpn-and-ip-masking-policy", accessed: "2026-10-23" }
  - { field: platforms, url: "https://elitetraderfunding.app/help/supported-platforms-and-data-feed", accessed: "2026-10-23" }
```

#### Direct to Funded — funded — 25K

```yaml
firm: elite-trader-funding
plan: "Direct to Funded"
phase: funded
size_label: "25K"
size_usd: "25000.00"
instruments: [MNQ, NQ]
platforms: ["Tradovate (web/desktop/mobile)", "NinjaTrader 8 (Windows only; Tradovate/CQG or Rithmic credentials)", "TradingView (Tradovate feed; selected at checkout)", "Rithmic: R|Trader Pro, ATAS, Bookmap, EdgeProX, eSignal, Finamark, Investor/RT, MotiveWave, Quantower, Sierra Chart", "Tickblaze"]
drawdown: { amount_usd: null, type: other, stops_trailing_at: null }
daily_loss_limit: { amount_usd: null, kind: none }
profit_target_usd: null
consistency_rule: { text: "No best-day cap; ATD qualifier: 10 ATDs, $300 minimum, 38% of best day", threshold_pct: null }
max_position: { minis: null, micros: null, scaling: null }
min_trading_days: null
trading_day_reset: { time: null, tz_as_published: null }
commission_per_contract: { mnq_usd: "0.51", nq_usd: "2.04", basis: per_side, includes: null }
copy_trading_allowed: conditional
automated_trading_allowed: conditional
automation_rule_text: "If you choose to use a trade copier, AI, bot, EA, algo, or other automated trading tool, you do so entirely at your own risk. The decision and responsibility to use such tools rest solely with the user. AI, bots, EAs, and algos additionally require ETF's written authorization before use. [ToS (ac)] makes use of artificial intelligence (AI), bots, automated trading systems, trade copiers, or other automated trading strategies that are not expressly authorized in writing by ETF."
vps_allowed: no
personal_device_required: unknown
status: unverified
not_confirmed_fields: [trading_day_reset, personal_device_required, commission_per_contract.includes, vps_allowed, drawdown.amount_usd, max_position, size availability]
notes: "UNVERIFIED SIZE: 25K DTF appears in the DTF help article's ATD table (safety net $2,600, End-of-Day) and in LIVE ELITE onboarding balances, but not on the DTF product page (10K/50K/100K) nor in Contract Limits. If safety net = max drawdown + $100, drawdown would be $2,500 (derived, not published). Do not use as a preset without confirmation."
sources:
  - { field: consistency_rule, url: "https://elitetraderfunding.app/help/how-the-dtf-plan-works", accessed: "2026-10-23" }
  - { field: drawdown, url: "https://elitetraderfunding.app/help/how-the-dtf-plan-works", accessed: "2026-10-23" }
  - { field: commission_per_contract, url: "https://elitetraderfunding.app/help/guide-to-our-commissions-and-exchanges-we-support", accessed: "2026-10-23" }
  - { field: automation_rule_text, url: "https://elitetraderfunding.app/help/trade-copier-disclaimer", accessed: "2026-10-23" }
  - { field: automation_rule_text, url: "https://elitetraderfunding.app/terms-of-service", accessed: "2026-10-23" }
  - { field: vps_allowed, url: "https://elitetraderfunding.app/help/vpn-and-ip-masking-policy", accessed: "2026-10-23" }
  - { field: platforms, url: "https://elitetraderfunding.app/help/supported-platforms-and-data-feed", accessed: "2026-10-23" }
```

#### LIVE ELITE — live — onboarding balances

```yaml
firm: elite-trader-funding
plan: "LIVE ELITE"
phase: live
size_label: "n/a"
size_usd: "0.00"
instruments: [MNQ, NQ]
platforms: ["Tradovate (web/desktop/mobile)", "NinjaTrader 8 (Windows only; Tradovate/CQG or Rithmic credentials)", "TradingView (Tradovate feed; selected at checkout)", "Rithmic: R|Trader Pro, ATAS, Bookmap, EdgeProX, eSignal, Finamark, Investor/RT, MotiveWave, Quantower, Sierra Chart", "Tickblaze"]
drawdown: { amount_usd: null, type: trailing_eod, stops_trailing_at: null }
daily_loss_limit: { amount_usd: null, kind: none }
profit_target_usd: null
consistency_rule: { text: "Sim consistency rule no longer applies", threshold_pct: null }
max_position: { minis: null, micros: null, scaling: null }
min_trading_days: null
trading_day_reset: { time: null, tz_as_published: null }
commission_per_contract: { mnq_usd: "0.51", nq_usd: "2.04", basis: per_side, includes: null }
copy_trading_allowed: conditional
automated_trading_allowed: conditional
automation_rule_text: "If you choose to use a trade copier, AI, bot, EA, algo, or other automated trading tool, you do so entirely at your own risk. The decision and responsibility to use such tools rest solely with the user. AI, bots, EAs, and algos additionally require ETF's written authorization before use. [ToS (ac)] makes use of artificial intelligence (AI), bots, automated trading systems, trade copiers, or other automated trading strategies that are not expressly authorized in writing by ETF."
vps_allowed: no
personal_device_required: unknown
status: unverified
not_confirmed_fields: [trading_day_reset, personal_device_required, commission_per_contract.includes, vps_allowed, drawdown.amount_usd, max_position, platforms (live broker), copy/automation rules for LIVE]
notes: "Not purchasable ('no checkout button'); invitation after sim stage; real capital with starting balances of $1,250-$2,500 (not 25K-300K sizes). 80/20 split, $250 minimum withdrawal, end-of-day drawdown, Level 2 data in a $197 market-data fee, swing trading 'permitted at ETF's discretion'. The trade copier / automation rules for LIVE ELITE are not separately stated; live-account platform/broker not stated. size_usd 0.00 is a placeholder. Record for completeness only."
sources:
  - { field: drawdown, url: "https://elitetraderfunding.app/live-elite", accessed: "2026-10-23" }
  - { field: max_position, url: "https://elitetraderfunding.app/help/live-elite-information", accessed: "2026-10-23" }
  - { field: commission_per_contract, url: "https://elitetraderfunding.app/help/guide-to-our-commissions-and-exchanges-we-support", accessed: "2026-10-23" }
  - { field: automation_rule_text, url: "https://elitetraderfunding.app/help/trade-copier-disclaimer", accessed: "2026-10-23" }
  - { field: automation_rule_text, url: "https://elitetraderfunding.app/terms-of-service", accessed: "2026-10-23" }
  - { field: vps_allowed, url: "https://elitetraderfunding.app/help/vpn-and-ip-masking-policy", accessed: "2026-10-23" }
  - { field: platforms, url: "https://elitetraderfunding.app/help/supported-platforms-and-data-feed", accessed: "2026-10-23" }
```

### 10. Unverified / gaps / conflicts

| # | Field / topic | Kind | Detail |
|---|---|---|---|
| 1 | News flat rule (funded) | conflict | S26 (Help Center): "ETF does not impose any restrictions or limitations on traders during major economic news events." vs S2 (ToS): "all Elite Accounts must be flat 5 minutes before the event to 5 minutes after the event starts. Traders still in evaluation phase do not have this rule." Not resolved. The event list is on a page linked from the ToS and not read |
| 2 | 1 Step 250K availability | conflict | S5 lists 250K ($6,500 / $15,000 / 24 minis) and S32 says "Sizes 50K to 250K"; S4 says "250K Live Trailing Eval (no longer sold, existing accounts only)"; homepage S37 says "Accounts up to $150K" |
| 3 | 1 Step 150K | gap | Listed in S4 (18 minis / 180 micros) and S32 but absent from S5 (50K, 100K, 250K); drawdown and profit target not published on any page read |
| 4 | End of Day 150K | gap | Only position limits (S4) and existence (S32); drawdown, daily loss limit, profit target not published |
| 5 | Static 25K minimum balance | conflict | S28 and "Elite Minimum Balances": 25K Static $24,100 (10K Static $9,600) vs S6: floor $24,000 on a $1,000 static drawdown. Blocks use the plan-page drawdown |
| 6 | VPS wording | conflict (wording) | S2 clause (l) and S3 prohibit VPS (exception: approved partner IP range) vs S1: "use VPNs or VPS for legitimate and benign reasons". Blocks use `vps_allowed: no` |
| 7 | 25K DTF | unverified | In the DTF ATD table (S15, S17) and LIVE ELITE balances (S30), not on S10 or in S4. Block is `unverified` with nulls |
| 8 | DTF "Profit target" | conflict (label) | S10 table shows "Profit target $1,600 / $5,100 / $5,100"; S15: "DTF accounts have no profit target" (those figures are the safety nets). Blocks use no profit target |
| 9 | Trading-day / session reset time | gap | Not published as a reset time (see §4). Closest: ToS Trading Session 6:00 p.m. ET to 5:00 p.m. ET; maintenance 4:00-5:00 PM CST |
| 10 | Personal device / orders from own device | gap | No explicit rule found; only account-sharing, trading-for-others and VPS/IP rules |
| 11 | Copying with accounts at other firms | gap | Not mentioned |
| 12 | Webhooks, alerts, semi-automated | gap | Not mentioned; "automated trading systems" need written authorization; boundary undefined. Contact route: support ticket via the "Boomeo AI chat" or support@elitetraderfunding.com (S4) |
| 13 | Commission inclusions | gap | Not stated whether $0.51 / $2.04 per side include exchange, NFA, clearing or platform fees |
| 14 | LIVE ELITE rules | unverified | Platform/broker, position limits, daily loss limit, copier and automation rules for LIVE not stated; ToS has a LIVE ELITE section not fully read |
| 15 | Fast Track funded phase | gap | Funded reward structure for Fast Track (10K, below 25K) not extracted; low priority |
| 16 | Legacy plans | noted | Legacy End of Day, Legacy Diamond Hands, Legacy Fast Track and Legacy 1-Step promos (5/5/3/2 ATD cycle for 1 Step bought July 6-27, 2026) exist for older purchases only; not captured in blocks |
| 17 | Terms drift | noted | ToS "Last updated: October 11, 2026"; ATD rule changed for evaluations bought on/after October 2, 2026 2:00 PM PT; "Trade How You Want" promotion can end at any time; copier disclaimer is enforced at reward stage. Re-verify before use |
| 18 | Other sizes (low priority) | noted | S28 lists funded types 75K, 250K, 300K and Static 100K/150K, EOD 25K/75K/250K, 25K..300K Elite classes; none is on a current product page read, so no blocks |
| 19 | Overnight margin charges, reward payout tables | not read | Help articles "Overnight Margin: Responsibilities and Charges" and "How Do Payouts Work" were not analysed beyond the stated safety-net and ATD rules |

### 11. Changelog

- 2026-10-23 — initial research (Lupa)
