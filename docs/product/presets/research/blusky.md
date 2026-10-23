# BluSky — Prop Firm Rules Research

| | |
|---|---|
| **Firm** | BluSky Trading Company, LLC dba BluSky.pro (futures plans in scope; BluSky also sells Stock plans, out of scope) |
| **Official domains** | `blusky.pro` (main site; `www.blusky.pro` redirects to it) · `help.blusky.pro` (official "BluSky Rules and Help Center", Intercom, linked from the site footer "Help Center") · `trader.blusky.pro` (checkout/dashboard, behind login, not read) · `volumetrica.blusky.pro` (Volumetrica web platform, not read). Domain verified: the footer of blusky.pro reads "BluSky Trading Company LLC · 7901 4th St N Ste 300, St. Petersburg, FL 33702". |
| **Research date** | 2026-10-23 (all URLs below accessed 2026-10-23) |
| **Researcher** | Lupa (Prop Firm Rules Researcher) |
| **File status** | `partial`. Plan tables (drawdown, profit target, daily loss limit, consistency, contracts) are published per plan and phase for Launch, Propel and Orbit. Open: no official text found on automated trading, VPS/VPN/personal device, or copying with other firms; trading-day reset time/timezone is only given as "6PM"; Tradovate commission basis is not stated; several conflicts between help-center articles (section 10). |
| **Instruments covered** | MNQ and NQ are listed (https://help.blusky.pro/en/articles/17115915-markets-symbols-commissions, accessed 2026-10-23). |
| **Platforms (for routing)** | Rithmic, Tradovate (NinjaTrader Prop / Tradovate Prop), Volumetrica, Tradesea, Tickblaze, BlackArrow. NinjaTrader 8 is supported through both the Tradovate and the Rithmic connection guides (Rithmic scope conflicts, section 2). |

Evidence (raw text of every cited page, URL on line 1; raw HTML in `raw/` and `h/`): `/tmp/claude-1003/-srv-www-allwin--maestri-roles-6bf77a80-a1ed-43dc-bc00-08a480f178a5/c3ed71e7-3dd4-4f1a-8a03-9bc3a3fa7b49/scratchpad/r3/blusky/`. Temporary scratch storage; every quote below was checked by exact match against the captured page text.

**Programs found.** Three futures plan families, each with phases: **Launch** (Evaluation, Buffer Zone, Sim Funded; sizes 50K/100K/200K), **Propel** (Evaluation, Buffer Zone, Sim Funded; 25K/50K/100K/150K/200K/300K) and **Orbit** (Evaluation, Sim Funded; 50K/100K/150K/200K). Also **Instant Sim Funded** (50K) and **Direct to Funded** ($3,500 account), which skip the evaluation. After Sim Funded comes **Live Brokerage** (real market, by call-up only after a Risk Team review). Older help-center pages use legacy labels (Advanced 25K, Premium 50K, Premium+ 100K, Static Growth 150K, Static Growth+ 200K, Static Blu+ 300K, "BluLive/Buffer"). The Stock account plans are not covered.

**BluSky terms.** "Buffer Zone" / "BluLive/Buffer" = the second phase between evaluation and Sim Funded, where the profit builds the Sim Funded starting balance. "Sim Funded" = simulated account with withdrawable profits. "Consistency" = largest single profitable day versus the sum of profitable days (per the Launch/Propel/Orbit pages).

### Main source list (all accessed 2026-10-23)

| Key | URL |
|---|---|
| S1 | https://help.blusky.pro/en/articles/16300846-launch-plans-phases-rules-and-costs — Launch plans |
| S2 | https://help.blusky.pro/en/articles/16300862-propel-plans-phases-rules-and-costs — Propel plans |
| S3 | https://help.blusky.pro/en/articles/16300866-orbit-plans-two-phases-faster-payouts — Orbit plans |
| S4 | https://help.blusky.pro/en/articles/12434059-evaluation-rules — Evaluation Rules |
| S5 | https://help.blusky.pro/en/articles/12433986-static-growth-plan — Static Growth Plan (legacy name) |
| S6 | https://help.blusky.pro/en/articles/12434458-blulive-buffer-stage — BluLive/Buffer Stage |
| S7 | https://help.blusky.pro/en/articles/17297032-sim-funded-accounts-rules — Sim Funded Accounts Rules (effective Oct 1, 2026) |
| S8 | https://help.blusky.pro/en/articles/12434469-sim-funded-accounts-processing — Sim Funded Accounts Processing |
| S9 | https://help.blusky.pro/en/articles/16300869-instant-funding-direct-to-funded-and-instant-sim-funded — Instant Funding: Direct to Funded and Instant Sim Funded |
| S10 | https://help.blusky.pro/en/articles/12433943-multiple-accounts-copy-trading — Multiple Accounts/Copy Trading |
| S11 | https://help.blusky.pro/en/articles/12069208-ethical-trading-conduct-policy-blusky-trading-company — Ethical Trading & Conduct Policy |
| S12 | https://help.blusky.pro/en/articles/12433926-blusky-supported-platforms — BluSky Supported Platforms |
| S13 | https://help.blusky.pro/en/articles/10182306-rithmic-platforms — Rithmic Platforms |
| S14 | https://help.blusky.pro/en/articles/10182216-tradovate-prop-platforms — Tradovate Prop Platforms |
| S15 | https://help.blusky.pro/en/articles/10167334-ninjatrader-tradovate-connection-guide — NinjaTrader Tradovate Connection Guide |
| S16 | https://help.blusky.pro/en/articles/12434521-blusky-rithmic-ninja-trader-connection-guide — BluSky Rithmic NinjaTrader Connection Guide |
| S17 | https://help.blusky.pro/en/articles/17115915-markets-symbols-commissions — Markets, Symbols & Commissions |
| S18 | https://help.blusky.pro/en/articles/14184170-evaluation-overview — Evaluation Overview |
| S19 | https://help.blusky.pro/en/articles/12433870-what-to-expect — What to Expect |
| S20 | https://help.blusky.pro/en/articles/11490284-blusky-trading-company-llc-dba-blusky-pro-terms-of-use — Terms of Use (effective May 31, 2025) |
| S21 | https://blusky.pro/ — Home page (plan picker, platform logos) |
| S22 | https://help.blusky.pro/en/articles/11760172-scaling-contract-size-in-sim-funded-brokerage-accounts — Scaling Contract Size in Sim Funded & Brokerage |
| S23 | https://help.blusky.pro/en/articles/17290897-live-brokerage-qualifying-transitioning-and-trading-a-live-account — Live Brokerage: Qualifying, Transitioning and Trading |
| S24 | https://help.blusky.pro/en/articles/15756814-orbit-v2-accounts-how-do-payouts-work — Orbit V2 payouts |
| S25 | https://help.blusky.pro/en/articles/12292284-sim-funded-payouts — Sim Funded Payouts |
| S26 | https://help.blusky.pro/en/articles/13642215-risk-management-brokerage-account-supervision — Risk Management & Brokerage Account Supervision |
| S27 | https://help.blusky.pro/en/articles/12434092-resets — Resets |
| S28 | https://blusky.pro/tradingview/ — Resource page "TradingView for Futures Traders" (updated Oct 2, 2026) |
| S29 | https://blusky.pro/what-is-a-static-drawdown/ — Static drawdown explainer |
| S30 | https://help.blusky.pro/en/articles/11490260-volumetrica-getting-started — Volumetrica Getting Started |
| S31 | https://blusky.pro/terms-and-conditions/ — Website Terms and Conditions |
| S32 | https://help.blusky.pro/en/articles/12109372-blusky-risk-team-reviews — Risk Team Reviews |
| S33 | https://help.blusky.pro/en/articles/11490309-sim-funded-and-live-brokerage-contract-size — Sim Funded and Live Brokerage Contract Size |

Coverage: the help-center sitemap (`https://help.blusky.pro/sitemap.xml`, 92 articles; a superset of the 73 articles reachable from the 19 collections) was crawled in full and grepped for drawdown, daily loss, profit target, consistency, contracts, scaling, reset, session, copy, copier, automat, bot, algo, webhook, TradingView, VPS, VPN, remote, server, device, IP, commission, platform, news, overnight, hedging, inactivity and max accounts. The main-site sitemap (52 pages) was checked; pages with rules content were read (FAQ, static drawdown, subscription terms, terms-and-conditions, TradingView resource, how-blusky-compares, find-your-plan). Plan prices and the plan x platform selector on blusky.pro load dynamically and were not captured.

---

## 1. Header notes

- The Launch, Propel and Orbit pages (S1-S3) are the most recent plan-specific articles and are used for the plan tables. Older general articles (S4, S5, S6, S8, S18, S25) disagree with them in places; each disagreement is recorded as a conflict in section 10 and in the YAML `not_confirmed_fields`.
- Money is recorded exactly as published. The firm writes drawdowns as "$-2,000 Trailing (EOD)"; the amount is stored as a positive value.

## 2. Platforms

- **Supported platforms (S12):** Volumetrica (VolSys / Deepchart, VolBook / Deepdom); Rithmic (NinjaTrader, Tradesea, R|Trader Pro, MotiveWave, Quantower, Jigsaw, ATAS, Sierra Chart, Bookmap, VolSys / DeepChart, VolBook / Deepdom, "Most other Rithmic Platforms"); Tradovate (NinjaTrader Prop, Tradovate Prop, TradingView, Jigsaw, Bookmap). Also help articles for Tradesea (S-help 14187901), ONYX for Rithmic, Tickblaze (desktop and web trader) and BlackArrow. The home page platform list shows Tradovate, Tradesea, Rithmic, Tickblaze and BlackArrow logos (Volumetrica in the selector).
- **NinjaTrader 8 via Tradovate:** account type "Simulation", provider "NinjaTrader" (S15). Quote: "Enter your BluSky credentials and select" (https://help.blusky.pro/en/articles/10167334-ninjatrader-tradovate-connection-guide, accessed 2026-10-23).
- **NinjaTrader 8 via Rithmic:** provider "Rithmic for NinjaTrader Brokerage", System "Rithmic Paper Trading (Chicago)" (S16). **Phase scope conflict:** "NinjaTrader (Evaluation, BluLive and Sim Funded only)" (https://help.blusky.pro/en/articles/12433926-blusky-supported-platforms, accessed 2026-10-23) versus "NinjaTrader (Evaluation Only)" (https://help.blusky.pro/en/articles/10182306-rithmic-platforms, accessed 2026-10-23).
- **Scaling per platform (S22):** "Tradovate runs their Sim Funded Accounts on the same software and principle's as their LIVE Brokerage account software." (https://help.blusky.pro/en/articles/11760172-scaling-contract-size-in-sim-funded-brokerage-accounts, accessed 2026-10-23) / "Project X scales up with account growth." (https://help.blusky.pro/en/articles/11760172-scaling-contract-size-in-sim-funded-brokerage-accounts, accessed 2026-10-23) / "Rithmic does not scale." (https://help.blusky.pro/en/articles/11760172-scaling-contract-size-in-sim-funded-brokerage-accounts, accessed 2026-10-23). Project X is named here but is not in the supported-platforms list (gap).
- **Sim Funded is simulated:** "Your Sim Funded Account remains simulated. Your orders do not reach the live market until you transition into Live Brokerage." (https://help.blusky.pro/en/articles/17297032-sim-funded-accounts-rules, accessed 2026-10-23)
- **Live Brokerage brokers (S23, market-data table):** Tradovate, Volumetrica, Rithmic. Whether NinjaTrader works on the live account is not stated (gap).
- **Which platform is offered for which plan:** the plan selector on blusky.pro and the checkout are dynamic (not captured). The help center does not map plans to platforms, apart from the commission table (S17) which lists Rithmic & Volumetrica, Tradovate / NinjaTrader and Tradesea. Gap.
- **TradingView:** appears under Tradovate-compatible platforms (S12, S14). The TradingView resource page (S28) describes it as charting/alerts, with execution on a separate platform: "When your TradingView alerts go off, that’s your signal to flip over to your trading platform, like Tradovate, and take action." (https://blusky.pro/tradingview/, accessed 2026-10-23)

## 3. Plans

All three families run in simulated accounts with a "Sim Funded" stage where profits are real and withdrawable. Dollar figures are as published on the plan pages (S1, S2, S3). Per-size YAML is in section 9.

### 3.1 Launch (Evaluation, Buffer Zone, Sim Funded)

Quote: "The launch fee is charged when you pass the evaluation, not when you buy." (https://help.blusky.pro/en/articles/16300846-launch-plans-phases-rules-and-costs, accessed 2026-10-23)

| Size | Launch fee | Eval target | Eval drawdown | Eval consistency (largest day) | Buffer target | Buffer drawdown | Contracts |
|---|---|---|---|---|---|---|---|
| 50K | $99 | $3,000 | $2,000 trailing EOD | 50% ($1,500) | $3,000 | $2,000 trailing EOD | 5 Mini / 50 Micros |
| 100K | $149 | $6,000 | $2,500 trailing EOD | 50% ($3,000) | $3,500 | $2,500 trailing EOD | 10 Mini / 100 Micros |
| 200K | $179 | $6,000 | $2,000 static | 50% ($3,000) | $3,000 | $2,000 static | 2 Mini / 20 Micros |

Daily loss limit: None in the evaluation; none in the Buffer Zone ("There is no daily loss limit." (https://help.blusky.pro/en/articles/12434458-blulive-buffer-stage, accessed 2026-10-23)). Buffer consistency 34%. Sim Funded starting balance $3,000 / $3,500 / $3,000; rolling 7-day payout cap $2,500 / $3,000 / $2,500; payout minimum $500; minimum balance $100.

### 3.2 Propel (Evaluation, Buffer Zone, Sim Funded)

Quote: "There is no launch fee on any Propel plan." (https://help.blusky.pro/en/articles/16300862-propel-plans-phases-rules-and-costs, accessed 2026-10-23)

| Size | Eval target | Eval drawdown | Eval consistency | Contracts (eval) | Buffer target | Buffer drawdown | Sim Funded start | 7-day cap |
|---|---|---|---|---|---|---|---|---|
| 25K | $1,500 | $1,200 trailing EOD | 34% | 2 Mini / 20 Micros | $1,500 | $1,200 trailing EOD | $1,500 | $1,500 |
| 50K | $3,000 | $2,000 trailing EOD | 34% | 5 / 50 | $3,000 | $2,000 trailing EOD | $3,000 | $2,500 |
| 100K | $6,000 | $2,500 trailing EOD | 34% | 10 / 100 | $3,500 | $2,500 trailing EOD | $3,500 | $3,000 |
| 150K | $3,000 | $1,000 static | 34% (conflict: 30%) | 1 / 10 | $2,000 | $1,000 static | $2,000 (conflict: $1,500) | $1,500 |
| 200K | $6,000 | $2,000 static | 34% (conflict: 30%) | 2 / 20 | $3,000 | $2,000 static | $3,000 | $2,500 |
| 300K | $20,000 | $5,000 static | 21% | 5 / 50 | $3,500 | $2,500 static | $3,500 (conflict: $3,000) | $3,000 |

Daily loss limit: None for every size on the Propel page (300K conflicts, see section 10). "Because your biggest day cannot exceed 34% of your profitable days, the target cannot be reached in fewer than 3 profitable days." (https://help.blusky.pro/en/articles/16300862-propel-plans-phases-rules-and-costs, accessed 2026-10-23)

### 3.3 Orbit (Evaluation, Sim Funded)

Quote: "There is no launch fee on any Orbit plan, and no daily loss limit in either phase." (https://help.blusky.pro/en/articles/16300866-orbit-plans-two-phases-faster-payouts, accessed 2026-10-23) One-time purchase, 30-day validity. "There is no consistency rule in the Orbit evaluation" (https://help.blusky.pro/en/articles/16300866-orbit-plans-two-phases-faster-payouts, accessed 2026-10-23).

| Size | Eval target | Eval drawdown | Contracts eval / funded | Funded drawdown (locks after) | 7-day cap |
|---|---|---|---|---|---|
| 50K | $3,000 | $2,000 trailing EOD | 5 Mini 50 Micros / 2 Mini 20 Micros | $2,000 ($2,000 net profit) | $2,000 |
| 100K | $6,000 | $3,000 trailing EOD | 10 / 100 / 3 / 30 | $3,000 ($3,000 net profit) | $2,500 |
| 150K | $9,000 | $4,500 trailing EOD | 15 / 150 / 5 / 50 | $4,500 ($4,500 net profit) | $3,000 |
| 200K | $8,000 | $3,000 static | 3 / 30 (conflict: 2 / 20) / 3 / 30 | $3,000 ($3,000 net profit) | $2,500 |

Funded drawdown: "The floor trails your closed daily profit dollar-for-dollar, and once your lifetime net profit matches your starting drawdown it locks permanently at your starting balance." (https://help.blusky.pro/en/articles/16300866-orbit-plans-two-phases-faster-payouts, accessed 2026-10-23) Funded consistency: "A 40% consistency rule applies while Sim Funded" (https://help.blusky.pro/en/articles/16300866-orbit-plans-two-phases-faster-payouts, accessed 2026-10-23).

### 3.4 Instant Sim Funded and Direct to Funded (low priority)

| | Direct to Funded | Instant Sim Funded |
|---|---|---|
| Account size | $3,500 | $50,000 |
| Drawdown | $2,500 static (closes below $1,000) | $2,000 intraday trailing (closes below $48,000; stops at $51,000 once balance passes $53,000) |
| Daily loss limit | $2,000 | $1,000 |
| Consistency | 21% | 21% |
| Contracts | 5 Mini / 50 Micros | 5 Mini / 50 Micros |

Source S9. The page does not say whether the daily loss limit closes the account or only pauses trading (gap).

### 3.5 Sim Funded and Live Brokerage notes

- Sim Funded: stop trading for the day at $5,000 daily profit and the week at $10,000; $10,000 total profit triggers a Brokerage Review (S7). Up to 3 Sim Funded accounts.
- Risk Team: "Our Risk Team may also apply additional controls when necessary, including:" (https://help.blusky.pro/en/articles/17297032-sim-funded-accounts-rules, accessed 2026-10-23) contract limits, daily loss limits, temporary trading restrictions (S7, S26).
- Live Brokerage (S23): by call-up only; new accounts start at $0 with the qualifying drawdown, Bonus Vault for eligible Sim Funded profit, or a one-time buyout of 50% of the drawdown. Not purchasable. Live parameters are set per account, not published per size (gap).

## 4. Trading-day reset

- **Not published as a session time and timezone.** The only reset-like statements: "The minimum trailing balance increases with your highest EOD balance. It increases with profits, but it does not decrease with losses, and is calculated from your starting balance at 6PM." (https://help.blusky.pro/en/articles/12434059-evaluation-rules, accessed 2026-10-23) and "the BluLive/Buffer account will typically be set up the same evening after 6PM EST" (https://help.blusky.pro/en/articles/12433870-what-to-expect, accessed 2026-10-23). The timezone for the 6PM figure is therefore only implied (EST). Recorded as `18:00` with the tz text as published; marked not confirmed.
- **Flatten/close:** "We recommend closing all trades before 4:45PM EST; our system attempts to limit when you can trade and will auto-liquidate around 15 minutes before markets close. This is not a guarantee of liquidation." (https://help.blusky.pro/en/articles/12434059-evaluation-rules, accessed 2026-10-23) and "We will not adjust accounts that have open positions after 5PM EST." (https://help.blusky.pro/en/articles/12434059-evaluation-rules, accessed 2026-10-23)
- **Holiday rule:** not found (gap). Payout processing cut-off is 11 AM ET on weekdays excluding holidays (S25).

## 5. Trade copying stance

Classification: **conflict** between two official pages.

- Copier allowed, with limits (S10, table "Trade Copier Allowed": "Yes" for every plan/size listed, including Launch, Propel, Orbit, Direct 2 Funded, Sim Funded and Live Brokerage). Quote: "You can have any number of Evaluation Accounts of any combination. Funded accounts can only be 3 at any time" (https://help.blusky.pro/en/articles/12433943-multiple-accounts-copy-trading, accessed 2026-10-23)
- Account limits (S10): Evaluation accounts unlimited; Direct 2 Funded 3; Sim Funded 3; Live Brokerage 2. "If you are in Live Brokerage, you cannot trade in Sim Funded at the same time." (https://help.blusky.pro/en/articles/12433943-multiple-accounts-copy-trading, accessed 2026-10-23)
- Opposite positions across accounts are forbidden: "This means you cannot go long in one account and short in another account under you user ID and control." (https://help.blusky.pro/en/articles/12433943-multiple-accounts-copy-trading, accessed 2026-10-23) / "If this happens, in Sim or Live accounts the we may permanently close your account. No exceptions." (https://help.blusky.pro/en/articles/12433943-multiple-accounts-copy-trading, accessed 2026-10-23)
- Household rule: "Traders in the same household, must be in control of their own accounts, and must be trading on different machines." (https://help.blusky.pro/en/articles/12433943-multiple-accounts-copy-trading, accessed 2026-10-23)
- Conduct policy (S11) lists as forbidden: "Account Mirroring: Copying trades from another trader, account, or signal provider, as this undermines independent trading skill development and risk management." (https://help.blusky.pro/en/articles/12069208-ethical-trading-conduct-policy-blusky-trading-company, accessed 2026-10-23) This can be read as covering a copier between a trader's own accounts, which S10 allows. The pages are not reconciled; recorded as conflict.
- Third parties: "Unauthorized Trading: Conducting trades on behalf of third parties or sharing program incentives without explicit, prior authorization." (https://help.blusky.pro/en/articles/12069208-ethical-trading-conduct-policy-blusky-trading-company, accessed 2026-10-23)
- Copying to/from accounts at other firms: not mentioned (gap).
- Volumetrica has a help article "Volumetrica Web Copy Trading" (S-help 11492231) that contains only related-article links in the captured text (the content is video); no rule text.

## 6. Automated trading stance

**(i) Classification: `unknown`.** No official page found that allows or forbids bots, EAs, algorithms or automated order entry, and none that mentions webhooks. The only related text is in the Ethical Trading & Conduct Policy (S11), shown as `automation_rule_text`:

- "Coordinated Manipulation: Engaging, individually or with others (including across multiple accounts), in trading patterns whose primary intent is to manipulate, abuse, or secure an unfair advantage on our platform. This includes violating our terms and conditions, or using automated software (AI, ultra-high speed, bulk data entry) to unfairly influence outcomes." (https://help.blusky.pro/en/articles/12069208-ethical-trading-conduct-policy-blusky-trading-company, accessed 2026-10-23)
- "Exploitative Strategies: Any method designed to take advantage of system glitches, pricing errors, or latency delays within our services." (https://help.blusky.pro/en/articles/12069208-ethical-trading-conduct-policy-blusky-trading-company, accessed 2026-10-23)
- Strategy freedom (S28): "BluSky allows freedom in your strategy: you can use any setup you want, as long as you follow the rules." (https://blusky.pro/tradingview/, accessed 2026-10-23)

**(ii) Named mechanisms:**
- **Semi-automated trading:** not mentioned.
- **Trade copiers:** mentioned (section 5; allowed in S10, "Account Mirroring" forbidden in S11).
- **Webhooks:** not mentioned. **Alerts:** TradingView alerts are mentioned as a signal for the trader to act manually (S28 quote in section 2).
- **ATM / automatic brackets:** BlackArrow "Trading Brackets" automatically attach stop loss and take profit (help articles 17154432 and 17154172). NinjaTrader ATM: not mentioned. Volumetrica "Daily Lockout" risk settings are trader-set (S-help 12944350; S7: "You may set your own Daily Loss Limit where supported.").
- **Bots / algorithms / API:** the only mention is "automated software (AI, ultra-high speed, bulk data entry)" in S11 above. No API or bot page.

**(iii) Differs by phase:** not stated. The conduct policy is not phase-specific.

## 7. Device / hosting rules and other rules that affect routing or risk

### Device / hosting rules

**`unknown`: no official text found.** All 92 help-center articles and the main-site rule pages were grepped for VPS, VPN, virtual private, remote desktop, remote server, personal device, IP address: no match in any rule text. The only device-related sentence is the household rule: "Traders in the same household, must be in control of their own accounts, and must be trading on different machines." (https://help.blusky.pro/en/articles/12433943-multiple-accounts-copy-trading, accessed 2026-10-23) The Terms of Use (S20) and website Terms (S31) forbid sharing account credentials: "You may not share your login or create multiple accounts using different email addresses." (https://help.blusky.pro/en/articles/11490284-blusky-trading-company-llc-dba-blusky-pro-terms-of-use, accessed 2026-10-23). R4 re-check (2026-10-23): the help-center sitemap (73 article pages) and the main-site sitemap (52 pages) were re-fetched with curl and grepped for device, computer, IP address, remote, VPS, VPN, virtual private, server, cloud, machine, third part, on behalf. Related text, not a device rule: "Unauthorized Trading: Conducting trades on behalf of third parties or sharing program incentives without explicit, prior authorization." (https://help.blusky.pro/en/articles/12069208-ethical-trading-conduct-policy-blusky-trading-company, accessed 2026-10-23). No text requires or forbids a personal device, own IP, VPS or remote server, so `personal_device_required` and `vps_allowed` stay `unknown`. Whether a VPS, a remote server or a relay on another machine is allowed is therefore unverified; it should be asked of BluSky support (gap).

### Other rules

- **News:** "Trading news events at your own risk" (https://help.blusky.pro/en/articles/12434059-evaluation-rules, accessed 2026-10-23) (evaluation). Live: "News trading is permitted unless your account has a specific restriction" (https://help.blusky.pro/en/articles/17290897-live-brokerage-qualifying-transitioning-and-trading-a-live-account, accessed 2026-10-23). Tradovate may scale margin/contracts during news (S22).
- **Hedging:** "Hedging is not allowed in Futures Trading, this refers to the User/Household and may result in Reward denial, account closure, account forfeit or temporary account closure." (https://help.blusky.pro/en/articles/12434059-evaluation-rules, accessed 2026-10-23) Sim Funded payouts page: hedging "even in Sim Environment" closes the account (S25).
- **Overnight / weekend:** positions should be closed before the close (section 4); the system may auto-liquidate about 15 minutes before the close.
- **Contract limits:** exceeding them does not fail an evaluation but the account may be liquidated or restarted ("We will not fail you for using too many contracts and we are able to automatically limit limit your max size if you are trading one product." (https://help.blusky.pro/en/articles/12434059-evaluation-rules, accessed 2026-10-23)). In Sim Funded: "automated account adjustments may be made" (S7: P&L adjustments, balance updates, position liquidations).
- **Price limit:** holding positions within 2% of a product's lock limit is prohibited (S11).
- **Scaling:** Sim Funded contract limits scale on Tradovate by margin, not on Rithmic or Volumetrica; support can raise limits after growth (S22, S33).
- **Max accounts:** evaluation unlimited; Sim Funded / Direct to Funded 3; Live Brokerage 2 (S10). If the drawdown is breached on 3 or more funded accounts, the Risk Team may restrict the trader to a single active account (S7).
- **Inactivity:** "Abandoned Accounts: Evaluation, BluLive, SimFunded, or brokerage accounts inactive for 30 consecutive days may be closed at our discretion." (https://help.blusky.pro/en/articles/11490284-blusky-trading-company-llc-dba-blusky-pro-terms-of-use, accessed 2026-10-23) Live: "Place at least one trade every 30 days to keep your account active" (https://help.blusky.pro/en/articles/17290897-live-brokerage-qualifying-transitioning-and-trading-a-live-account, accessed 2026-10-23).
- **Minimum trading days:** no explicit rule. "Although our minimum trading days are low (2-4 depending on your account type)" (https://help.blusky.pro/en/articles/12433870-what-to-expect, accessed 2026-10-23). Orbit evaluation can be passed in one day.
- **Mandatory stop loss / minimum or maximum trade duration:** none found.
- **During reviews:** do not trade an account under review (S7, S32).

## 8. Commissions

Source S17 (https://help.blusky.pro/en/articles/17115915-markets-symbols-commissions, accessed 2026-10-23).

| Platform | Account type | Published |
|---|---|---|
| Rithmic & Volumetrica | Evaluation | "Minis: $2/side · Micros: $0.50/side" |
| Rithmic & Volumetrica | Funded | "Set by brokerage" |
| Tradesea | Evaluation | Minis: $2/side · Micros: $0.50/side |
| Tradesea | Funded | Set by brokerage |
| Tradovate / NinjaTrader | Evaluation / BluLive / Funded | "View Pricing" (link to Tradovate), then the symbol table below |

"Note: Per side means the applicable commission is charged when opening a position and again when closing it."

Symbol table (Tradovate / NinjaTrader), USD, as published. Columns: Exch. + NFA | Clearing | All-In Lifetime | All-In Monthly | All-In Free. The page does not say per side or round turn for these columns and does not say which tier applies to BluSky accounts.

| Symbol | Exch. + NFA | Clearing | All-In Lifetime | All-In Monthly | All-In Free |
|---|---|---|---|---|---|
| NQ | 1.39 | 0.19 | 2.17 | 2.57 | 2.87 |
| MNQ | 0.36 | 0.19 | 0.64 | 0.84 | 0.94 |

Quote: "Rates are subject to change. Refer to Tradovate's commission schedule for the most current pricing"

Recorded in YAML: MNQ `0.50` and NQ `2.00` per side for Rithmic/Volumetrica/Tradesea evaluation accounts. The page does not say whether these include exchange/NFA/platform fees. For Tradovate/NinjaTrader the per-contract figure is **not confirmed**: no basis (per side or round turn) and no tier is stated. Tickblaze and BlackArrow commissions are not published. Funded and Buffer commissions: "Set by brokerage" or not listed.

## 9. Machine-readable blocks

Common to all blocks: `copy_trading_allowed: conflict` (S10 vs S11), `automated_trading_allowed: unknown` with the only related text in `automation_rule_text`, `vps_allowed: unknown`, `personal_device_required: unknown`, and `trading_day_reset` recorded from the "6PM" statement with the tz not confirmed. `platforms` is the firm-wide list (plan x platform mapping is not published). Buffer Zone blocks use phase `evaluation` with the plan name suffixed "Buffer Zone" and the Buffer target in `profit_target_usd`. Sim Funded / Orbit funded blocks use phase `funded` and `profit_target_usd: "none"`.

#### Launch — evaluation — 50K

```yaml
firm: "blusky"
plan: "Launch"
phase: "evaluation"
size_label: "50K"
size_usd: "50000.00"
instruments: ["MNQ", "NQ"]
platforms: ["NinjaTrader 8 (via Tradovate connection; via Rithmic connection, phase scope conflicts)", "Tradovate", "TradingView (Tradovate list)", "Rithmic platforms (R|Trader Pro, Quantower, ATAS, Sierra Chart, Bookmap, MotiveWave, Jigsaw, ONYX, others)", "Volumetrica web platform (VolSys/Deepchart)", "Tradesea", "Tickblaze", "BlackArrow"]
drawdown: { amount_usd: "2000.00", type: "trailing_eod", stops_trailing_at: null }
daily_loss_limit: { amount_usd: null, kind: "none" }
profit_target_usd: "3000.00"
consistency_rule: { text: "Largest single profitable day must stay at or below 50% of the sum of profitable days (published as 1500 = 50% of target). Evaluation Rules page words the method differently: highest profit day becomes the % of your new target.", threshold_pct: "50" }
max_position: { minis: 5, micros: 50, scaling: null }
min_trading_days: null
trading_day_reset: { time: "18:00", tz_as_published: "6PM, tz not stated on that page; other pages say EST/ET" }
commission_per_contract: { mnq_usd: "0.50", nq_usd: "2.00", basis: "per_side", includes: "Applies to Rithmic, Volumetrica and Tradesea evaluation accounts only. Page does not say what the figure includes. Tradovate/NinjaTrader: table with no basis stated (see section 8)." }
copy_trading_allowed: "conflict"
automated_trading_allowed: "unknown"
automation_rule_text: "Coordinated Manipulation: Engaging, individually or with others (including across multiple accounts), in trading patterns whose primary intent is to manipulate, abuse, or secure an unfair advantage on our platform. This includes violating our terms and conditions, or using automated software (AI, ultra-high speed, bulk data entry) to unfairly influence outcomes."
vps_allowed: "unknown"
personal_device_required: "unknown"
status: "partial"
not_confirmed_fields: ["copy_trading_allowed", "automated_trading_allowed", "vps_allowed", "personal_device_required", "trading_day_reset", "commission_per_contract", "drawdown.stops_trailing_at", "consistency_rule.text"]
notes: "Launch fee $99 is charged when the evaluation is passed (14 days to pay), not at purchase. Subscription billed every 30 days; amount is on the dynamic plans page, not captured. Evaluation reset $49. No explicit minimum trading days rule; target cannot be reached in fewer than 2 profitable days (firm's words). Trailing floor lock point for EOD trailing in evaluation not published."
sources:
  - { field: "drawdown", url: "https://help.blusky.pro/en/articles/16300846-launch-plans-phases-rules-and-costs", accessed: "2026-10-23" }
  - { field: "profit_target_usd", url: "https://help.blusky.pro/en/articles/16300846-launch-plans-phases-rules-and-costs", accessed: "2026-10-23" }
  - { field: "consistency_rule", url: "https://help.blusky.pro/en/articles/16300846-launch-plans-phases-rules-and-costs", accessed: "2026-10-23" }
  - { field: "consistency_rule", url: "https://help.blusky.pro/en/articles/12434059-evaluation-rules", accessed: "2026-10-23" }
  - { field: "max_position", url: "https://help.blusky.pro/en/articles/16300846-launch-plans-phases-rules-and-costs", accessed: "2026-10-23" }
  - { field: "daily_loss_limit", url: "https://help.blusky.pro/en/articles/16300846-launch-plans-phases-rules-and-costs", accessed: "2026-10-23" }
  - { field: "trading_day_reset", url: "https://help.blusky.pro/en/articles/12434059-evaluation-rules", accessed: "2026-10-23" }
  - { field: "commission_per_contract", url: "https://help.blusky.pro/en/articles/17115915-markets-symbols-commissions", accessed: "2026-10-23" }
  - { field: "platforms", url: "https://help.blusky.pro/en/articles/12433926-blusky-supported-platforms", accessed: "2026-10-23" }
  - { field: "copy_trading_allowed", url: "https://help.blusky.pro/en/articles/12433943-multiple-accounts-copy-trading", accessed: "2026-10-23" }
  - { field: "copy_trading_allowed", url: "https://help.blusky.pro/en/articles/12069208-ethical-trading-conduct-policy-blusky-trading-company", accessed: "2026-10-23" }
  - { field: "automation_rule_text", url: "https://help.blusky.pro/en/articles/12069208-ethical-trading-conduct-policy-blusky-trading-company", accessed: "2026-10-23" }
```

#### Launch — Buffer Zone — evaluation — 50K

```yaml
firm: "blusky"
plan: "Launch — Buffer Zone"
phase: "evaluation"
size_label: "50K"
size_usd: "50000.00"
instruments: ["MNQ", "NQ"]
platforms: ["NinjaTrader 8 (via Tradovate connection; via Rithmic connection, phase scope conflicts)", "Tradovate", "TradingView (Tradovate list)", "Rithmic platforms (R|Trader Pro, Quantower, ATAS, Sierra Chart, Bookmap, MotiveWave, Jigsaw, ONYX, others)", "Volumetrica web platform (VolSys/Deepchart)", "Tradesea", "Tickblaze", "BlackArrow"]
drawdown: { amount_usd: "2000.00", type: "trailing_eod", stops_trailing_at: null }
daily_loss_limit: { amount_usd: null, kind: "none" }
profit_target_usd: "3000.00"
consistency_rule: { text: "Largest single profitable day at or below 34% of the sum of profitable days; checked when trying to pass the phase.", threshold_pct: "34" }
max_position: { minis: null, micros: null, scaling: null }
min_trading_days: null
trading_day_reset: { time: "18:00", tz_as_published: "6PM, tz not stated on that page; other pages say EST/ET" }
commission_per_contract: { mnq_usd: null, nq_usd: null, basis: null, includes: "Not published for this phase: funded commission is 'Set by brokerage' (Rithmic/Volumetrica/Tradesea); Buffer not listed for Rithmic/Volumetrica/Tradesea; Tradovate table has no stated basis (section 8)." }
copy_trading_allowed: "conflict"
automated_trading_allowed: "unknown"
automation_rule_text: "Coordinated Manipulation: Engaging, individually or with others (including across multiple accounts), in trading patterns whose primary intent is to manipulate, abuse, or secure an unfair advantage on our platform. This includes violating our terms and conditions, or using automated software (AI, ultra-high speed, bulk data entry) to unfairly influence outcomes."
vps_allowed: "unknown"
personal_device_required: "unknown"
status: "partial"
not_confirmed_fields: ["copy_trading_allowed", "automated_trading_allowed", "vps_allowed", "personal_device_required", "trading_day_reset", "commission_per_contract", "max_position", "drawdown.stops_trailing_at"]
notes: "Buffer Zone is a second phase after the evaluation; the firm states it is 'not another evaluation phase' (BluLive/Buffer page) and that it has no daily loss limit. Contract limits for the Buffer Zone are not stated on the plan page (evaluation limits likely carry over, not confirmed). Buffer reset $250 each for the first three, $300 for resets 4-6."
sources:
  - { field: "drawdown", url: "https://help.blusky.pro/en/articles/16300846-launch-plans-phases-rules-and-costs", accessed: "2026-10-23" }
  - { field: "profit_target_usd", url: "https://help.blusky.pro/en/articles/16300846-launch-plans-phases-rules-and-costs", accessed: "2026-10-23" }
  - { field: "consistency_rule", url: "https://help.blusky.pro/en/articles/16300846-launch-plans-phases-rules-and-costs", accessed: "2026-10-23" }
  - { field: "daily_loss_limit", url: "https://help.blusky.pro/en/articles/12434458-blulive-buffer-stage", accessed: "2026-10-23" }
```

#### Launch — funded — 50K

```yaml
firm: "blusky"
plan: "Launch — Sim Funded"
phase: "funded"
size_label: "50K"
size_usd: "50000.00"
instruments: ["MNQ", "NQ"]
platforms: ["NinjaTrader 8 (via Tradovate connection; via Rithmic connection, phase scope conflicts)", "Tradovate", "TradingView (Tradovate list)", "Rithmic platforms (R|Trader Pro, Quantower, ATAS, Sierra Chart, Bookmap, MotiveWave, Jigsaw, ONYX, others)", "Volumetrica web platform (VolSys/Deepchart)", "Tradesea", "Tickblaze", "BlackArrow"]
drawdown: { amount_usd: null, type: "static", stops_trailing_at: "static; account minimum balance $100 (starting balance 3000.00)" }
daily_loss_limit: { amount_usd: null, kind: "none" }
profit_target_usd: "none"
consistency_rule: { text: "No consistency rule in Sim Funded for Launch (Evaluation Overview).", threshold_pct: null }
max_position: { minis: null, micros: null, scaling: "Sim Funded limits conflict across official pages; actual limits are those displayed on the account (see section 10)." }
min_trading_days: null
trading_day_reset: { time: "18:00", tz_as_published: "6PM, tz not stated on that page; other pages say EST/ET" }
commission_per_contract: { mnq_usd: null, nq_usd: null, basis: null, includes: "Not published for this phase: funded commission is 'Set by brokerage' (Rithmic/Volumetrica/Tradesea); Buffer not listed for Rithmic/Volumetrica/Tradesea; Tradovate table has no stated basis (section 8)." }
copy_trading_allowed: "conflict"
automated_trading_allowed: "unknown"
automation_rule_text: "Coordinated Manipulation: Engaging, individually or with others (including across multiple accounts), in trading patterns whose primary intent is to manipulate, abuse, or secure an unfair advantage on our platform. This includes violating our terms and conditions, or using automated software (AI, ultra-high speed, bulk data entry) to unfairly influence outcomes."
vps_allowed: "unknown"
personal_device_required: "unknown"
status: "conflict"
not_confirmed_fields: ["copy_trading_allowed", "automated_trading_allowed", "vps_allowed", "personal_device_required", "trading_day_reset", "commission_per_contract", "max_position", "drawdown.amount_usd"]
notes: "Sim Funded: simulated account, profits withdrawable. Starting balance $3000, minimum balance $100, payout minimum $500, rolling 7-day payout cap $2500. Stop trading for the day at $5,000 daily profit and for the week at $10,000. Risk Team may add daily loss limits and contract limits at any time."
sources:
  - { field: "drawdown", url: "https://help.blusky.pro/en/articles/16300846-launch-plans-phases-rules-and-costs", accessed: "2026-10-23" }
  - { field: "drawdown", url: "https://help.blusky.pro/en/articles/12434469-sim-funded-accounts-processing", accessed: "2026-10-23" }
  - { field: "consistency_rule", url: "https://help.blusky.pro/en/articles/14184170-evaluation-overview", accessed: "2026-10-23" }
  - { field: "max_position", url: "https://help.blusky.pro/en/articles/12434458-blulive-buffer-stage", accessed: "2026-10-23" }
  - { field: "max_position", url: "https://help.blusky.pro/en/articles/17297032-sim-funded-accounts-rules", accessed: "2026-10-23" }
  - { field: "max_position", url: "https://help.blusky.pro/en/articles/11760172-scaling-contract-size-in-sim-funded-brokerage-accounts", accessed: "2026-10-23" }
  - { field: "daily_loss_limit", url: "https://help.blusky.pro/en/articles/17297032-sim-funded-accounts-rules", accessed: "2026-10-23" }
```

#### Launch — evaluation — 100K

```yaml
firm: "blusky"
plan: "Launch"
phase: "evaluation"
size_label: "100K"
size_usd: "100000.00"
instruments: ["MNQ", "NQ"]
platforms: ["NinjaTrader 8 (via Tradovate connection; via Rithmic connection, phase scope conflicts)", "Tradovate", "TradingView (Tradovate list)", "Rithmic platforms (R|Trader Pro, Quantower, ATAS, Sierra Chart, Bookmap, MotiveWave, Jigsaw, ONYX, others)", "Volumetrica web platform (VolSys/Deepchart)", "Tradesea", "Tickblaze", "BlackArrow"]
drawdown: { amount_usd: "2500.00", type: "trailing_eod", stops_trailing_at: null }
daily_loss_limit: { amount_usd: null, kind: "none" }
profit_target_usd: "6000.00"
consistency_rule: { text: "Largest single profitable day must stay at or below 50% of the sum of profitable days (published as 3000 = 50% of target). Evaluation Rules page words the method differently: highest profit day becomes the % of your new target.", threshold_pct: "50" }
max_position: { minis: 10, micros: 100, scaling: null }
min_trading_days: null
trading_day_reset: { time: "18:00", tz_as_published: "6PM, tz not stated on that page; other pages say EST/ET" }
commission_per_contract: { mnq_usd: "0.50", nq_usd: "2.00", basis: "per_side", includes: "Applies to Rithmic, Volumetrica and Tradesea evaluation accounts only. Page does not say what the figure includes. Tradovate/NinjaTrader: table with no basis stated (see section 8)." }
copy_trading_allowed: "conflict"
automated_trading_allowed: "unknown"
automation_rule_text: "Coordinated Manipulation: Engaging, individually or with others (including across multiple accounts), in trading patterns whose primary intent is to manipulate, abuse, or secure an unfair advantage on our platform. This includes violating our terms and conditions, or using automated software (AI, ultra-high speed, bulk data entry) to unfairly influence outcomes."
vps_allowed: "unknown"
personal_device_required: "unknown"
status: "partial"
not_confirmed_fields: ["copy_trading_allowed", "automated_trading_allowed", "vps_allowed", "personal_device_required", "trading_day_reset", "commission_per_contract", "drawdown.stops_trailing_at", "consistency_rule.text"]
notes: "Launch fee $149 is charged when the evaluation is passed (14 days to pay), not at purchase. Subscription billed every 30 days; amount is on the dynamic plans page, not captured. Evaluation reset $59. No explicit minimum trading days rule; target cannot be reached in fewer than 2 profitable days (firm's words). Trailing floor lock point for EOD trailing in evaluation not published."
sources:
  - { field: "drawdown", url: "https://help.blusky.pro/en/articles/16300846-launch-plans-phases-rules-and-costs", accessed: "2026-10-23" }
  - { field: "profit_target_usd", url: "https://help.blusky.pro/en/articles/16300846-launch-plans-phases-rules-and-costs", accessed: "2026-10-23" }
  - { field: "consistency_rule", url: "https://help.blusky.pro/en/articles/16300846-launch-plans-phases-rules-and-costs", accessed: "2026-10-23" }
  - { field: "consistency_rule", url: "https://help.blusky.pro/en/articles/12434059-evaluation-rules", accessed: "2026-10-23" }
  - { field: "max_position", url: "https://help.blusky.pro/en/articles/16300846-launch-plans-phases-rules-and-costs", accessed: "2026-10-23" }
  - { field: "daily_loss_limit", url: "https://help.blusky.pro/en/articles/16300846-launch-plans-phases-rules-and-costs", accessed: "2026-10-23" }
  - { field: "trading_day_reset", url: "https://help.blusky.pro/en/articles/12434059-evaluation-rules", accessed: "2026-10-23" }
  - { field: "commission_per_contract", url: "https://help.blusky.pro/en/articles/17115915-markets-symbols-commissions", accessed: "2026-10-23" }
  - { field: "platforms", url: "https://help.blusky.pro/en/articles/12433926-blusky-supported-platforms", accessed: "2026-10-23" }
  - { field: "copy_trading_allowed", url: "https://help.blusky.pro/en/articles/12433943-multiple-accounts-copy-trading", accessed: "2026-10-23" }
  - { field: "copy_trading_allowed", url: "https://help.blusky.pro/en/articles/12069208-ethical-trading-conduct-policy-blusky-trading-company", accessed: "2026-10-23" }
  - { field: "automation_rule_text", url: "https://help.blusky.pro/en/articles/12069208-ethical-trading-conduct-policy-blusky-trading-company", accessed: "2026-10-23" }
```

#### Launch — Buffer Zone — evaluation — 100K

```yaml
firm: "blusky"
plan: "Launch — Buffer Zone"
phase: "evaluation"
size_label: "100K"
size_usd: "100000.00"
instruments: ["MNQ", "NQ"]
platforms: ["NinjaTrader 8 (via Tradovate connection; via Rithmic connection, phase scope conflicts)", "Tradovate", "TradingView (Tradovate list)", "Rithmic platforms (R|Trader Pro, Quantower, ATAS, Sierra Chart, Bookmap, MotiveWave, Jigsaw, ONYX, others)", "Volumetrica web platform (VolSys/Deepchart)", "Tradesea", "Tickblaze", "BlackArrow"]
drawdown: { amount_usd: "2500.00", type: "trailing_eod", stops_trailing_at: null }
daily_loss_limit: { amount_usd: null, kind: "none" }
profit_target_usd: "3500.00"
consistency_rule: { text: "Largest single profitable day at or below 34% of the sum of profitable days; checked when trying to pass the phase.", threshold_pct: "34" }
max_position: { minis: null, micros: null, scaling: null }
min_trading_days: null
trading_day_reset: { time: "18:00", tz_as_published: "6PM, tz not stated on that page; other pages say EST/ET" }
commission_per_contract: { mnq_usd: null, nq_usd: null, basis: null, includes: "Not published for this phase: funded commission is 'Set by brokerage' (Rithmic/Volumetrica/Tradesea); Buffer not listed for Rithmic/Volumetrica/Tradesea; Tradovate table has no stated basis (section 8)." }
copy_trading_allowed: "conflict"
automated_trading_allowed: "unknown"
automation_rule_text: "Coordinated Manipulation: Engaging, individually or with others (including across multiple accounts), in trading patterns whose primary intent is to manipulate, abuse, or secure an unfair advantage on our platform. This includes violating our terms and conditions, or using automated software (AI, ultra-high speed, bulk data entry) to unfairly influence outcomes."
vps_allowed: "unknown"
personal_device_required: "unknown"
status: "partial"
not_confirmed_fields: ["copy_trading_allowed", "automated_trading_allowed", "vps_allowed", "personal_device_required", "trading_day_reset", "commission_per_contract", "max_position", "drawdown.stops_trailing_at"]
notes: "Buffer Zone is a second phase after the evaluation; the firm states it is 'not another evaluation phase' (BluLive/Buffer page) and that it has no daily loss limit. Contract limits for the Buffer Zone are not stated on the plan page (evaluation limits likely carry over, not confirmed). Buffer reset $250 each for the first three, $300 for resets 4-6."
sources:
  - { field: "drawdown", url: "https://help.blusky.pro/en/articles/16300846-launch-plans-phases-rules-and-costs", accessed: "2026-10-23" }
  - { field: "profit_target_usd", url: "https://help.blusky.pro/en/articles/16300846-launch-plans-phases-rules-and-costs", accessed: "2026-10-23" }
  - { field: "consistency_rule", url: "https://help.blusky.pro/en/articles/16300846-launch-plans-phases-rules-and-costs", accessed: "2026-10-23" }
  - { field: "daily_loss_limit", url: "https://help.blusky.pro/en/articles/12434458-blulive-buffer-stage", accessed: "2026-10-23" }
```

#### Launch — funded — 100K

```yaml
firm: "blusky"
plan: "Launch — Sim Funded"
phase: "funded"
size_label: "100K"
size_usd: "100000.00"
instruments: ["MNQ", "NQ"]
platforms: ["NinjaTrader 8 (via Tradovate connection; via Rithmic connection, phase scope conflicts)", "Tradovate", "TradingView (Tradovate list)", "Rithmic platforms (R|Trader Pro, Quantower, ATAS, Sierra Chart, Bookmap, MotiveWave, Jigsaw, ONYX, others)", "Volumetrica web platform (VolSys/Deepchart)", "Tradesea", "Tickblaze", "BlackArrow"]
drawdown: { amount_usd: null, type: "static", stops_trailing_at: "static; account minimum balance $100 (starting balance 3500.00)" }
daily_loss_limit: { amount_usd: null, kind: "none" }
profit_target_usd: "none"
consistency_rule: { text: "No consistency rule in Sim Funded for Launch (Evaluation Overview).", threshold_pct: null }
max_position: { minis: null, micros: null, scaling: "Sim Funded limits conflict across official pages; actual limits are those displayed on the account (see section 10)." }
min_trading_days: null
trading_day_reset: { time: "18:00", tz_as_published: "6PM, tz not stated on that page; other pages say EST/ET" }
commission_per_contract: { mnq_usd: null, nq_usd: null, basis: null, includes: "Not published for this phase: funded commission is 'Set by brokerage' (Rithmic/Volumetrica/Tradesea); Buffer not listed for Rithmic/Volumetrica/Tradesea; Tradovate table has no stated basis (section 8)." }
copy_trading_allowed: "conflict"
automated_trading_allowed: "unknown"
automation_rule_text: "Coordinated Manipulation: Engaging, individually or with others (including across multiple accounts), in trading patterns whose primary intent is to manipulate, abuse, or secure an unfair advantage on our platform. This includes violating our terms and conditions, or using automated software (AI, ultra-high speed, bulk data entry) to unfairly influence outcomes."
vps_allowed: "unknown"
personal_device_required: "unknown"
status: "conflict"
not_confirmed_fields: ["copy_trading_allowed", "automated_trading_allowed", "vps_allowed", "personal_device_required", "trading_day_reset", "commission_per_contract", "max_position", "drawdown.amount_usd"]
notes: "Sim Funded: simulated account, profits withdrawable. Starting balance $3500, minimum balance $100, payout minimum $500, rolling 7-day payout cap $3000. Stop trading for the day at $5,000 daily profit and for the week at $10,000. Risk Team may add daily loss limits and contract limits at any time."
sources:
  - { field: "drawdown", url: "https://help.blusky.pro/en/articles/16300846-launch-plans-phases-rules-and-costs", accessed: "2026-10-23" }
  - { field: "drawdown", url: "https://help.blusky.pro/en/articles/12434469-sim-funded-accounts-processing", accessed: "2026-10-23" }
  - { field: "consistency_rule", url: "https://help.blusky.pro/en/articles/14184170-evaluation-overview", accessed: "2026-10-23" }
  - { field: "max_position", url: "https://help.blusky.pro/en/articles/12434458-blulive-buffer-stage", accessed: "2026-10-23" }
  - { field: "max_position", url: "https://help.blusky.pro/en/articles/17297032-sim-funded-accounts-rules", accessed: "2026-10-23" }
  - { field: "max_position", url: "https://help.blusky.pro/en/articles/11760172-scaling-contract-size-in-sim-funded-brokerage-accounts", accessed: "2026-10-23" }
  - { field: "daily_loss_limit", url: "https://help.blusky.pro/en/articles/17297032-sim-funded-accounts-rules", accessed: "2026-10-23" }
```

#### Launch — evaluation — 200K

```yaml
firm: "blusky"
plan: "Launch"
phase: "evaluation"
size_label: "200K"
size_usd: "200000.00"
instruments: ["MNQ", "NQ"]
platforms: ["NinjaTrader 8 (via Tradovate connection; via Rithmic connection, phase scope conflicts)", "Tradovate", "TradingView (Tradovate list)", "Rithmic platforms (R|Trader Pro, Quantower, ATAS, Sierra Chart, Bookmap, MotiveWave, Jigsaw, ONYX, others)", "Volumetrica web platform (VolSys/Deepchart)", "Tradesea", "Tickblaze", "BlackArrow"]
drawdown: { amount_usd: "2000.00", type: "static", stops_trailing_at: "static: never moves" }
daily_loss_limit: { amount_usd: null, kind: "none" }
profit_target_usd: "6000.00"
consistency_rule: { text: "Largest single profitable day must stay at or below 50% of the sum of profitable days (published as 3000 = 50% of target). Evaluation Rules page words the method differently: highest profit day becomes the % of your new target.", threshold_pct: "50" }
max_position: { minis: 2, micros: 20, scaling: null }
min_trading_days: null
trading_day_reset: { time: "18:00", tz_as_published: "6PM, tz not stated on that page; other pages say EST/ET" }
commission_per_contract: { mnq_usd: "0.50", nq_usd: "2.00", basis: "per_side", includes: "Applies to Rithmic, Volumetrica and Tradesea evaluation accounts only. Page does not say what the figure includes. Tradovate/NinjaTrader: table with no basis stated (see section 8)." }
copy_trading_allowed: "conflict"
automated_trading_allowed: "unknown"
automation_rule_text: "Coordinated Manipulation: Engaging, individually or with others (including across multiple accounts), in trading patterns whose primary intent is to manipulate, abuse, or secure an unfair advantage on our platform. This includes violating our terms and conditions, or using automated software (AI, ultra-high speed, bulk data entry) to unfairly influence outcomes."
vps_allowed: "unknown"
personal_device_required: "unknown"
status: "partial"
not_confirmed_fields: ["copy_trading_allowed", "automated_trading_allowed", "vps_allowed", "personal_device_required", "trading_day_reset", "commission_per_contract", "consistency_rule.text"]
notes: "Launch fee $179 is charged when the evaluation is passed (14 days to pay), not at purchase. Subscription billed every 30 days; amount is on the dynamic plans page, not captured. Evaluation reset $69. No explicit minimum trading days rule; target cannot be reached in fewer than 2 profitable days (firm's words). Trailing floor lock point for EOD trailing in evaluation not published."
sources:
  - { field: "drawdown", url: "https://help.blusky.pro/en/articles/16300846-launch-plans-phases-rules-and-costs", accessed: "2026-10-23" }
  - { field: "profit_target_usd", url: "https://help.blusky.pro/en/articles/16300846-launch-plans-phases-rules-and-costs", accessed: "2026-10-23" }
  - { field: "consistency_rule", url: "https://help.blusky.pro/en/articles/16300846-launch-plans-phases-rules-and-costs", accessed: "2026-10-23" }
  - { field: "consistency_rule", url: "https://help.blusky.pro/en/articles/12434059-evaluation-rules", accessed: "2026-10-23" }
  - { field: "max_position", url: "https://help.blusky.pro/en/articles/16300846-launch-plans-phases-rules-and-costs", accessed: "2026-10-23" }
  - { field: "daily_loss_limit", url: "https://help.blusky.pro/en/articles/16300846-launch-plans-phases-rules-and-costs", accessed: "2026-10-23" }
  - { field: "trading_day_reset", url: "https://help.blusky.pro/en/articles/12434059-evaluation-rules", accessed: "2026-10-23" }
  - { field: "commission_per_contract", url: "https://help.blusky.pro/en/articles/17115915-markets-symbols-commissions", accessed: "2026-10-23" }
  - { field: "platforms", url: "https://help.blusky.pro/en/articles/12433926-blusky-supported-platforms", accessed: "2026-10-23" }
  - { field: "copy_trading_allowed", url: "https://help.blusky.pro/en/articles/12433943-multiple-accounts-copy-trading", accessed: "2026-10-23" }
  - { field: "copy_trading_allowed", url: "https://help.blusky.pro/en/articles/12069208-ethical-trading-conduct-policy-blusky-trading-company", accessed: "2026-10-23" }
  - { field: "automation_rule_text", url: "https://help.blusky.pro/en/articles/12069208-ethical-trading-conduct-policy-blusky-trading-company", accessed: "2026-10-23" }
```

#### Launch — Buffer Zone — evaluation — 200K

```yaml
firm: "blusky"
plan: "Launch — Buffer Zone"
phase: "evaluation"
size_label: "200K"
size_usd: "200000.00"
instruments: ["MNQ", "NQ"]
platforms: ["NinjaTrader 8 (via Tradovate connection; via Rithmic connection, phase scope conflicts)", "Tradovate", "TradingView (Tradovate list)", "Rithmic platforms (R|Trader Pro, Quantower, ATAS, Sierra Chart, Bookmap, MotiveWave, Jigsaw, ONYX, others)", "Volumetrica web platform (VolSys/Deepchart)", "Tradesea", "Tickblaze", "BlackArrow"]
drawdown: { amount_usd: "2000.00", type: "static", stops_trailing_at: null }
daily_loss_limit: { amount_usd: null, kind: "none" }
profit_target_usd: "3000.00"
consistency_rule: { text: "Largest single profitable day at or below 34% of the sum of profitable days; checked when trying to pass the phase.", threshold_pct: "34" }
max_position: { minis: null, micros: null, scaling: null }
min_trading_days: null
trading_day_reset: { time: "18:00", tz_as_published: "6PM, tz not stated on that page; other pages say EST/ET" }
commission_per_contract: { mnq_usd: null, nq_usd: null, basis: null, includes: "Not published for this phase: funded commission is 'Set by brokerage' (Rithmic/Volumetrica/Tradesea); Buffer not listed for Rithmic/Volumetrica/Tradesea; Tradovate table has no stated basis (section 8)." }
copy_trading_allowed: "conflict"
automated_trading_allowed: "unknown"
automation_rule_text: "Coordinated Manipulation: Engaging, individually or with others (including across multiple accounts), in trading patterns whose primary intent is to manipulate, abuse, or secure an unfair advantage on our platform. This includes violating our terms and conditions, or using automated software (AI, ultra-high speed, bulk data entry) to unfairly influence outcomes."
vps_allowed: "unknown"
personal_device_required: "unknown"
status: "partial"
not_confirmed_fields: ["copy_trading_allowed", "automated_trading_allowed", "vps_allowed", "personal_device_required", "trading_day_reset", "commission_per_contract", "max_position"]
notes: "Buffer Zone is a second phase after the evaluation; the firm states it is 'not another evaluation phase' (BluLive/Buffer page) and that it has no daily loss limit. Contract limits for the Buffer Zone are not stated on the plan page (evaluation limits likely carry over, not confirmed). Buffer reset $250 each for the first three, $300 for resets 4-6."
sources:
  - { field: "drawdown", url: "https://help.blusky.pro/en/articles/16300846-launch-plans-phases-rules-and-costs", accessed: "2026-10-23" }
  - { field: "profit_target_usd", url: "https://help.blusky.pro/en/articles/16300846-launch-plans-phases-rules-and-costs", accessed: "2026-10-23" }
  - { field: "consistency_rule", url: "https://help.blusky.pro/en/articles/16300846-launch-plans-phases-rules-and-costs", accessed: "2026-10-23" }
  - { field: "daily_loss_limit", url: "https://help.blusky.pro/en/articles/12434458-blulive-buffer-stage", accessed: "2026-10-23" }
```

#### Launch — funded — 200K

```yaml
firm: "blusky"
plan: "Launch — Sim Funded"
phase: "funded"
size_label: "200K"
size_usd: "200000.00"
instruments: ["MNQ", "NQ"]
platforms: ["NinjaTrader 8 (via Tradovate connection; via Rithmic connection, phase scope conflicts)", "Tradovate", "TradingView (Tradovate list)", "Rithmic platforms (R|Trader Pro, Quantower, ATAS, Sierra Chart, Bookmap, MotiveWave, Jigsaw, ONYX, others)", "Volumetrica web platform (VolSys/Deepchart)", "Tradesea", "Tickblaze", "BlackArrow"]
drawdown: { amount_usd: null, type: "static", stops_trailing_at: "static; account minimum balance $100 (starting balance 3000.00)" }
daily_loss_limit: { amount_usd: null, kind: "none" }
profit_target_usd: "none"
consistency_rule: { text: "No consistency rule in Sim Funded for Launch (Evaluation Overview).", threshold_pct: null }
max_position: { minis: null, micros: null, scaling: "Sim Funded limits conflict across official pages; actual limits are those displayed on the account (see section 10)." }
min_trading_days: null
trading_day_reset: { time: "18:00", tz_as_published: "6PM, tz not stated on that page; other pages say EST/ET" }
commission_per_contract: { mnq_usd: null, nq_usd: null, basis: null, includes: "Not published for this phase: funded commission is 'Set by brokerage' (Rithmic/Volumetrica/Tradesea); Buffer not listed for Rithmic/Volumetrica/Tradesea; Tradovate table has no stated basis (section 8)." }
copy_trading_allowed: "conflict"
automated_trading_allowed: "unknown"
automation_rule_text: "Coordinated Manipulation: Engaging, individually or with others (including across multiple accounts), in trading patterns whose primary intent is to manipulate, abuse, or secure an unfair advantage on our platform. This includes violating our terms and conditions, or using automated software (AI, ultra-high speed, bulk data entry) to unfairly influence outcomes."
vps_allowed: "unknown"
personal_device_required: "unknown"
status: "conflict"
not_confirmed_fields: ["copy_trading_allowed", "automated_trading_allowed", "vps_allowed", "personal_device_required", "trading_day_reset", "commission_per_contract", "max_position", "drawdown.amount_usd"]
notes: "Sim Funded: simulated account, profits withdrawable. Starting balance $3000, minimum balance $100, payout minimum $500, rolling 7-day payout cap $2500. Stop trading for the day at $5,000 daily profit and for the week at $10,000. Risk Team may add daily loss limits and contract limits at any time."
sources:
  - { field: "drawdown", url: "https://help.blusky.pro/en/articles/16300846-launch-plans-phases-rules-and-costs", accessed: "2026-10-23" }
  - { field: "drawdown", url: "https://help.blusky.pro/en/articles/12434469-sim-funded-accounts-processing", accessed: "2026-10-23" }
  - { field: "consistency_rule", url: "https://help.blusky.pro/en/articles/14184170-evaluation-overview", accessed: "2026-10-23" }
  - { field: "max_position", url: "https://help.blusky.pro/en/articles/12434458-blulive-buffer-stage", accessed: "2026-10-23" }
  - { field: "max_position", url: "https://help.blusky.pro/en/articles/17297032-sim-funded-accounts-rules", accessed: "2026-10-23" }
  - { field: "max_position", url: "https://help.blusky.pro/en/articles/11760172-scaling-contract-size-in-sim-funded-brokerage-accounts", accessed: "2026-10-23" }
  - { field: "daily_loss_limit", url: "https://help.blusky.pro/en/articles/17297032-sim-funded-accounts-rules", accessed: "2026-10-23" }
```

#### Propel — evaluation — 25K

```yaml
firm: "blusky"
plan: "Propel"
phase: "evaluation"
size_label: "25K"
size_usd: "25000.00"
instruments: ["MNQ", "NQ"]
platforms: ["NinjaTrader 8 (via Tradovate connection; via Rithmic connection, phase scope conflicts)", "Tradovate", "TradingView (Tradovate list)", "Rithmic platforms (R|Trader Pro, Quantower, ATAS, Sierra Chart, Bookmap, MotiveWave, Jigsaw, ONYX, others)", "Volumetrica web platform (VolSys/Deepchart)", "Tradesea", "Tickblaze", "BlackArrow"]
drawdown: { amount_usd: "1200.00", type: "trailing_eod", stops_trailing_at: null }
daily_loss_limit: { amount_usd: null, kind: "none" }
profit_target_usd: "1500.00"
consistency_rule: { text: "Largest single profitable day at or below 34% of the sum of profitable days (published: $510 = 34% of target). Evaluation Rules page words the method as: highest profit day becomes the % of your new target.", threshold_pct: "34" }
max_position: { minis: 2, micros: 20, scaling: null }
min_trading_days: null
trading_day_reset: { time: "18:00", tz_as_published: "6PM, tz not stated on that page; other pages say EST/ET" }
commission_per_contract: { mnq_usd: "0.50", nq_usd: "2.00", basis: "per_side", includes: "Applies to Rithmic, Volumetrica and Tradesea evaluation accounts only. Page does not say what the figure includes. Tradovate/NinjaTrader: table with no basis stated (see section 8)." }
copy_trading_allowed: "conflict"
automated_trading_allowed: "unknown"
automation_rule_text: "Coordinated Manipulation: Engaging, individually or with others (including across multiple accounts), in trading patterns whose primary intent is to manipulate, abuse, or secure an unfair advantage on our platform. This includes violating our terms and conditions, or using automated software (AI, ultra-high speed, bulk data entry) to unfairly influence outcomes."
vps_allowed: "unknown"
personal_device_required: "unknown"
status: "partial"
not_confirmed_fields: ["copy_trading_allowed", "automated_trading_allowed", "vps_allowed", "personal_device_required", "trading_day_reset", "commission_per_contract", "drawdown.stops_trailing_at"]
notes: "No launch fee. Subscription billed every 30 days (amount on the dynamic plans page, not captured). Evaluation reset $79. Target cannot be reached in fewer than 3 profitable days (firm's words); no explicit minimum trading days rule."
sources:
  - { field: "drawdown", url: "https://help.blusky.pro/en/articles/16300862-propel-plans-phases-rules-and-costs", accessed: "2026-10-23" }
  - { field: "profit_target_usd", url: "https://help.blusky.pro/en/articles/16300862-propel-plans-phases-rules-and-costs", accessed: "2026-10-23" }
  - { field: "consistency_rule", url: "https://help.blusky.pro/en/articles/16300862-propel-plans-phases-rules-and-costs", accessed: "2026-10-23" }
  - { field: "consistency_rule", url: "https://help.blusky.pro/en/articles/12434059-evaluation-rules", accessed: "2026-10-23" }
  - { field: "max_position", url: "https://help.blusky.pro/en/articles/16300862-propel-plans-phases-rules-and-costs", accessed: "2026-10-23" }
  - { field: "daily_loss_limit", url: "https://help.blusky.pro/en/articles/16300862-propel-plans-phases-rules-and-costs", accessed: "2026-10-23" }
  - { field: "trading_day_reset", url: "https://help.blusky.pro/en/articles/12434059-evaluation-rules", accessed: "2026-10-23" }
  - { field: "commission_per_contract", url: "https://help.blusky.pro/en/articles/17115915-markets-symbols-commissions", accessed: "2026-10-23" }
  - { field: "platforms", url: "https://help.blusky.pro/en/articles/12433926-blusky-supported-platforms", accessed: "2026-10-23" }
  - { field: "copy_trading_allowed", url: "https://help.blusky.pro/en/articles/12433943-multiple-accounts-copy-trading", accessed: "2026-10-23" }
  - { field: "copy_trading_allowed", url: "https://help.blusky.pro/en/articles/12069208-ethical-trading-conduct-policy-blusky-trading-company", accessed: "2026-10-23" }
```

#### Propel — Buffer Zone — evaluation — 25K

```yaml
firm: "blusky"
plan: "Propel — Buffer Zone"
phase: "evaluation"
size_label: "25K"
size_usd: "25000.00"
instruments: ["MNQ", "NQ"]
platforms: ["NinjaTrader 8 (via Tradovate connection; via Rithmic connection, phase scope conflicts)", "Tradovate", "TradingView (Tradovate list)", "Rithmic platforms (R|Trader Pro, Quantower, ATAS, Sierra Chart, Bookmap, MotiveWave, Jigsaw, ONYX, others)", "Volumetrica web platform (VolSys/Deepchart)", "Tradesea", "Tickblaze", "BlackArrow"]
drawdown: { amount_usd: "1200.00", type: "trailing_eod", stops_trailing_at: null }
daily_loss_limit: { amount_usd: null, kind: "none" }
profit_target_usd: "1500.00"
consistency_rule: { text: "Largest single profitable day at or below 34% of the sum of profitable days; checked when trying to pass the phase.", threshold_pct: "34" }
max_position: { minis: null, micros: null, scaling: null }
min_trading_days: null
trading_day_reset: { time: "18:00", tz_as_published: "6PM, tz not stated on that page; other pages say EST/ET" }
commission_per_contract: { mnq_usd: null, nq_usd: null, basis: null, includes: "Not published for this phase: funded commission is 'Set by brokerage' (Rithmic/Volumetrica/Tradesea); Buffer not listed for Rithmic/Volumetrica/Tradesea; Tradovate table has no stated basis (section 8)." }
copy_trading_allowed: "conflict"
automated_trading_allowed: "unknown"
automation_rule_text: "Coordinated Manipulation: Engaging, individually or with others (including across multiple accounts), in trading patterns whose primary intent is to manipulate, abuse, or secure an unfair advantage on our platform. This includes violating our terms and conditions, or using automated software (AI, ultra-high speed, bulk data entry) to unfairly influence outcomes."
vps_allowed: "unknown"
personal_device_required: "unknown"
status: "partial"
not_confirmed_fields: ["copy_trading_allowed", "automated_trading_allowed", "vps_allowed", "personal_device_required", "trading_day_reset", "commission_per_contract", "max_position", "drawdown.stops_trailing_at"]
notes: "Buffer Zone: second phase, no daily loss limit, 'not another evaluation phase'. Contract limits for the Buffer Zone not stated on the plan page. Buffer reset $250 each for the first three, $300 for 4-6."
sources:
  - { field: "drawdown", url: "https://help.blusky.pro/en/articles/16300862-propel-plans-phases-rules-and-costs", accessed: "2026-10-23" }
  - { field: "profit_target_usd", url: "https://help.blusky.pro/en/articles/16300862-propel-plans-phases-rules-and-costs", accessed: "2026-10-23" }
  - { field: "consistency_rule", url: "https://help.blusky.pro/en/articles/16300862-propel-plans-phases-rules-and-costs", accessed: "2026-10-23" }
  - { field: "daily_loss_limit", url: "https://help.blusky.pro/en/articles/12434458-blulive-buffer-stage", accessed: "2026-10-23" }
```

#### Propel — funded — 25K

```yaml
firm: "blusky"
plan: "Propel — Sim Funded"
phase: "funded"
size_label: "25K"
size_usd: "25000.00"
instruments: ["MNQ", "NQ"]
platforms: ["NinjaTrader 8 (via Tradovate connection; via Rithmic connection, phase scope conflicts)", "Tradovate", "TradingView (Tradovate list)", "Rithmic platforms (R|Trader Pro, Quantower, ATAS, Sierra Chart, Bookmap, MotiveWave, Jigsaw, ONYX, others)", "Volumetrica web platform (VolSys/Deepchart)", "Tradesea", "Tickblaze", "BlackArrow"]
drawdown: { amount_usd: null, type: "static", stops_trailing_at: "static; account minimum balance $100 (starting balance 1500.00)" }
daily_loss_limit: { amount_usd: null, kind: "none" }
profit_target_usd: "none"
consistency_rule: { text: "No consistency rule in Sim Funded for Propel (Evaluation Overview).", threshold_pct: null }
max_position: { minis: null, micros: null, scaling: "Sim Funded limits conflict across official pages; actual limits are those displayed on the account (see section 10)." }
min_trading_days: null
trading_day_reset: { time: "18:00", tz_as_published: "6PM, tz not stated on that page; other pages say EST/ET" }
commission_per_contract: { mnq_usd: null, nq_usd: null, basis: null, includes: "Not published for this phase: funded commission is 'Set by brokerage' (Rithmic/Volumetrica/Tradesea); Buffer not listed for Rithmic/Volumetrica/Tradesea; Tradovate table has no stated basis (section 8)." }
copy_trading_allowed: "conflict"
automated_trading_allowed: "unknown"
automation_rule_text: "Coordinated Manipulation: Engaging, individually or with others (including across multiple accounts), in trading patterns whose primary intent is to manipulate, abuse, or secure an unfair advantage on our platform. This includes violating our terms and conditions, or using automated software (AI, ultra-high speed, bulk data entry) to unfairly influence outcomes."
vps_allowed: "unknown"
personal_device_required: "unknown"
status: "conflict"
not_confirmed_fields: ["copy_trading_allowed", "automated_trading_allowed", "vps_allowed", "personal_device_required", "trading_day_reset", "commission_per_contract", "max_position", "drawdown.amount_usd"]
notes: "Sim Funded: simulated, profits withdrawable. Starting balance $1500, minimum balance $100, payout minimum $500, rolling 7-day payout cap $1500. Stop trading for the day at $5,000 daily profit and for the week at $10,000. Risk Team may add daily loss limits and contract limits."
sources:
  - { field: "drawdown", url: "https://help.blusky.pro/en/articles/16300862-propel-plans-phases-rules-and-costs", accessed: "2026-10-23" }
  - { field: "drawdown", url: "https://help.blusky.pro/en/articles/12434469-sim-funded-accounts-processing", accessed: "2026-10-23" }
  - { field: "consistency_rule", url: "https://help.blusky.pro/en/articles/14184170-evaluation-overview", accessed: "2026-10-23" }
  - { field: "max_position", url: "https://help.blusky.pro/en/articles/12434458-blulive-buffer-stage", accessed: "2026-10-23" }
  - { field: "max_position", url: "https://help.blusky.pro/en/articles/17297032-sim-funded-accounts-rules", accessed: "2026-10-23" }
  - { field: "max_position", url: "https://help.blusky.pro/en/articles/11760172-scaling-contract-size-in-sim-funded-brokerage-accounts", accessed: "2026-10-23" }
```

#### Propel — evaluation — 50K

```yaml
firm: "blusky"
plan: "Propel"
phase: "evaluation"
size_label: "50K"
size_usd: "50000.00"
instruments: ["MNQ", "NQ"]
platforms: ["NinjaTrader 8 (via Tradovate connection; via Rithmic connection, phase scope conflicts)", "Tradovate", "TradingView (Tradovate list)", "Rithmic platforms (R|Trader Pro, Quantower, ATAS, Sierra Chart, Bookmap, MotiveWave, Jigsaw, ONYX, others)", "Volumetrica web platform (VolSys/Deepchart)", "Tradesea", "Tickblaze", "BlackArrow"]
drawdown: { amount_usd: "2000.00", type: "trailing_eod", stops_trailing_at: null }
daily_loss_limit: { amount_usd: null, kind: "none" }
profit_target_usd: "3000.00"
consistency_rule: { text: "Largest single profitable day at or below 34% of the sum of profitable days (published: $1,020 = 34% of target). Evaluation Rules page words the method as: highest profit day becomes the % of your new target.", threshold_pct: "34" }
max_position: { minis: 5, micros: 50, scaling: null }
min_trading_days: null
trading_day_reset: { time: "18:00", tz_as_published: "6PM, tz not stated on that page; other pages say EST/ET" }
commission_per_contract: { mnq_usd: "0.50", nq_usd: "2.00", basis: "per_side", includes: "Applies to Rithmic, Volumetrica and Tradesea evaluation accounts only. Page does not say what the figure includes. Tradovate/NinjaTrader: table with no basis stated (see section 8)." }
copy_trading_allowed: "conflict"
automated_trading_allowed: "unknown"
automation_rule_text: "Coordinated Manipulation: Engaging, individually or with others (including across multiple accounts), in trading patterns whose primary intent is to manipulate, abuse, or secure an unfair advantage on our platform. This includes violating our terms and conditions, or using automated software (AI, ultra-high speed, bulk data entry) to unfairly influence outcomes."
vps_allowed: "unknown"
personal_device_required: "unknown"
status: "partial"
not_confirmed_fields: ["copy_trading_allowed", "automated_trading_allowed", "vps_allowed", "personal_device_required", "trading_day_reset", "commission_per_contract", "drawdown.stops_trailing_at"]
notes: "No launch fee. Subscription billed every 30 days (amount on the dynamic plans page, not captured). Evaluation reset $79. Target cannot be reached in fewer than 3 profitable days (firm's words); no explicit minimum trading days rule."
sources:
  - { field: "drawdown", url: "https://help.blusky.pro/en/articles/16300862-propel-plans-phases-rules-and-costs", accessed: "2026-10-23" }
  - { field: "profit_target_usd", url: "https://help.blusky.pro/en/articles/16300862-propel-plans-phases-rules-and-costs", accessed: "2026-10-23" }
  - { field: "consistency_rule", url: "https://help.blusky.pro/en/articles/16300862-propel-plans-phases-rules-and-costs", accessed: "2026-10-23" }
  - { field: "consistency_rule", url: "https://help.blusky.pro/en/articles/12434059-evaluation-rules", accessed: "2026-10-23" }
  - { field: "max_position", url: "https://help.blusky.pro/en/articles/16300862-propel-plans-phases-rules-and-costs", accessed: "2026-10-23" }
  - { field: "daily_loss_limit", url: "https://help.blusky.pro/en/articles/16300862-propel-plans-phases-rules-and-costs", accessed: "2026-10-23" }
  - { field: "trading_day_reset", url: "https://help.blusky.pro/en/articles/12434059-evaluation-rules", accessed: "2026-10-23" }
  - { field: "commission_per_contract", url: "https://help.blusky.pro/en/articles/17115915-markets-symbols-commissions", accessed: "2026-10-23" }
  - { field: "platforms", url: "https://help.blusky.pro/en/articles/12433926-blusky-supported-platforms", accessed: "2026-10-23" }
  - { field: "copy_trading_allowed", url: "https://help.blusky.pro/en/articles/12433943-multiple-accounts-copy-trading", accessed: "2026-10-23" }
  - { field: "copy_trading_allowed", url: "https://help.blusky.pro/en/articles/12069208-ethical-trading-conduct-policy-blusky-trading-company", accessed: "2026-10-23" }
```

#### Propel — Buffer Zone — evaluation — 50K

```yaml
firm: "blusky"
plan: "Propel — Buffer Zone"
phase: "evaluation"
size_label: "50K"
size_usd: "50000.00"
instruments: ["MNQ", "NQ"]
platforms: ["NinjaTrader 8 (via Tradovate connection; via Rithmic connection, phase scope conflicts)", "Tradovate", "TradingView (Tradovate list)", "Rithmic platforms (R|Trader Pro, Quantower, ATAS, Sierra Chart, Bookmap, MotiveWave, Jigsaw, ONYX, others)", "Volumetrica web platform (VolSys/Deepchart)", "Tradesea", "Tickblaze", "BlackArrow"]
drawdown: { amount_usd: "2000.00", type: "trailing_eod", stops_trailing_at: null }
daily_loss_limit: { amount_usd: null, kind: "none" }
profit_target_usd: "3000.00"
consistency_rule: { text: "Largest single profitable day at or below 34% of the sum of profitable days; checked when trying to pass the phase.", threshold_pct: "34" }
max_position: { minis: null, micros: null, scaling: null }
min_trading_days: null
trading_day_reset: { time: "18:00", tz_as_published: "6PM, tz not stated on that page; other pages say EST/ET" }
commission_per_contract: { mnq_usd: null, nq_usd: null, basis: null, includes: "Not published for this phase: funded commission is 'Set by brokerage' (Rithmic/Volumetrica/Tradesea); Buffer not listed for Rithmic/Volumetrica/Tradesea; Tradovate table has no stated basis (section 8)." }
copy_trading_allowed: "conflict"
automated_trading_allowed: "unknown"
automation_rule_text: "Coordinated Manipulation: Engaging, individually or with others (including across multiple accounts), in trading patterns whose primary intent is to manipulate, abuse, or secure an unfair advantage on our platform. This includes violating our terms and conditions, or using automated software (AI, ultra-high speed, bulk data entry) to unfairly influence outcomes."
vps_allowed: "unknown"
personal_device_required: "unknown"
status: "partial"
not_confirmed_fields: ["copy_trading_allowed", "automated_trading_allowed", "vps_allowed", "personal_device_required", "trading_day_reset", "commission_per_contract", "max_position", "drawdown.stops_trailing_at"]
notes: "Buffer Zone: second phase, no daily loss limit, 'not another evaluation phase'. Contract limits for the Buffer Zone not stated on the plan page. Buffer reset $250 each for the first three, $300 for 4-6."
sources:
  - { field: "drawdown", url: "https://help.blusky.pro/en/articles/16300862-propel-plans-phases-rules-and-costs", accessed: "2026-10-23" }
  - { field: "profit_target_usd", url: "https://help.blusky.pro/en/articles/16300862-propel-plans-phases-rules-and-costs", accessed: "2026-10-23" }
  - { field: "consistency_rule", url: "https://help.blusky.pro/en/articles/16300862-propel-plans-phases-rules-and-costs", accessed: "2026-10-23" }
  - { field: "daily_loss_limit", url: "https://help.blusky.pro/en/articles/12434458-blulive-buffer-stage", accessed: "2026-10-23" }
```

#### Propel — funded — 50K

```yaml
firm: "blusky"
plan: "Propel — Sim Funded"
phase: "funded"
size_label: "50K"
size_usd: "50000.00"
instruments: ["MNQ", "NQ"]
platforms: ["NinjaTrader 8 (via Tradovate connection; via Rithmic connection, phase scope conflicts)", "Tradovate", "TradingView (Tradovate list)", "Rithmic platforms (R|Trader Pro, Quantower, ATAS, Sierra Chart, Bookmap, MotiveWave, Jigsaw, ONYX, others)", "Volumetrica web platform (VolSys/Deepchart)", "Tradesea", "Tickblaze", "BlackArrow"]
drawdown: { amount_usd: null, type: "static", stops_trailing_at: "static; account minimum balance $100 (starting balance 3000.00)" }
daily_loss_limit: { amount_usd: null, kind: "none" }
profit_target_usd: "none"
consistency_rule: { text: "No consistency rule in Sim Funded for Propel (Evaluation Overview).", threshold_pct: null }
max_position: { minis: null, micros: null, scaling: "Sim Funded limits conflict across official pages; actual limits are those displayed on the account (see section 10)." }
min_trading_days: null
trading_day_reset: { time: "18:00", tz_as_published: "6PM, tz not stated on that page; other pages say EST/ET" }
commission_per_contract: { mnq_usd: null, nq_usd: null, basis: null, includes: "Not published for this phase: funded commission is 'Set by brokerage' (Rithmic/Volumetrica/Tradesea); Buffer not listed for Rithmic/Volumetrica/Tradesea; Tradovate table has no stated basis (section 8)." }
copy_trading_allowed: "conflict"
automated_trading_allowed: "unknown"
automation_rule_text: "Coordinated Manipulation: Engaging, individually or with others (including across multiple accounts), in trading patterns whose primary intent is to manipulate, abuse, or secure an unfair advantage on our platform. This includes violating our terms and conditions, or using automated software (AI, ultra-high speed, bulk data entry) to unfairly influence outcomes."
vps_allowed: "unknown"
personal_device_required: "unknown"
status: "conflict"
not_confirmed_fields: ["copy_trading_allowed", "automated_trading_allowed", "vps_allowed", "personal_device_required", "trading_day_reset", "commission_per_contract", "max_position", "drawdown.amount_usd"]
notes: "Sim Funded: simulated, profits withdrawable. Starting balance $3000, minimum balance $100, payout minimum $500, rolling 7-day payout cap $2500. Stop trading for the day at $5,000 daily profit and for the week at $10,000. Risk Team may add daily loss limits and contract limits."
sources:
  - { field: "drawdown", url: "https://help.blusky.pro/en/articles/16300862-propel-plans-phases-rules-and-costs", accessed: "2026-10-23" }
  - { field: "drawdown", url: "https://help.blusky.pro/en/articles/12434469-sim-funded-accounts-processing", accessed: "2026-10-23" }
  - { field: "consistency_rule", url: "https://help.blusky.pro/en/articles/14184170-evaluation-overview", accessed: "2026-10-23" }
  - { field: "max_position", url: "https://help.blusky.pro/en/articles/12434458-blulive-buffer-stage", accessed: "2026-10-23" }
  - { field: "max_position", url: "https://help.blusky.pro/en/articles/17297032-sim-funded-accounts-rules", accessed: "2026-10-23" }
  - { field: "max_position", url: "https://help.blusky.pro/en/articles/11760172-scaling-contract-size-in-sim-funded-brokerage-accounts", accessed: "2026-10-23" }
```

#### Propel — evaluation — 100K

```yaml
firm: "blusky"
plan: "Propel"
phase: "evaluation"
size_label: "100K"
size_usd: "100000.00"
instruments: ["MNQ", "NQ"]
platforms: ["NinjaTrader 8 (via Tradovate connection; via Rithmic connection, phase scope conflicts)", "Tradovate", "TradingView (Tradovate list)", "Rithmic platforms (R|Trader Pro, Quantower, ATAS, Sierra Chart, Bookmap, MotiveWave, Jigsaw, ONYX, others)", "Volumetrica web platform (VolSys/Deepchart)", "Tradesea", "Tickblaze", "BlackArrow"]
drawdown: { amount_usd: "2500.00", type: "trailing_eod", stops_trailing_at: null }
daily_loss_limit: { amount_usd: null, kind: "none" }
profit_target_usd: "6000.00"
consistency_rule: { text: "Largest single profitable day at or below 34% of the sum of profitable days (published: $2,040 = 34% of target). Evaluation Rules page words the method as: highest profit day becomes the % of your new target.", threshold_pct: "34" }
max_position: { minis: 10, micros: 100, scaling: null }
min_trading_days: null
trading_day_reset: { time: "18:00", tz_as_published: "6PM, tz not stated on that page; other pages say EST/ET" }
commission_per_contract: { mnq_usd: "0.50", nq_usd: "2.00", basis: "per_side", includes: "Applies to Rithmic, Volumetrica and Tradesea evaluation accounts only. Page does not say what the figure includes. Tradovate/NinjaTrader: table with no basis stated (see section 8)." }
copy_trading_allowed: "conflict"
automated_trading_allowed: "unknown"
automation_rule_text: "Coordinated Manipulation: Engaging, individually or with others (including across multiple accounts), in trading patterns whose primary intent is to manipulate, abuse, or secure an unfair advantage on our platform. This includes violating our terms and conditions, or using automated software (AI, ultra-high speed, bulk data entry) to unfairly influence outcomes."
vps_allowed: "unknown"
personal_device_required: "unknown"
status: "partial"
not_confirmed_fields: ["copy_trading_allowed", "automated_trading_allowed", "vps_allowed", "personal_device_required", "trading_day_reset", "commission_per_contract", "drawdown.stops_trailing_at"]
notes: "No launch fee. Subscription billed every 30 days (amount on the dynamic plans page, not captured). Evaluation reset $79. Target cannot be reached in fewer than 3 profitable days (firm's words); no explicit minimum trading days rule."
sources:
  - { field: "drawdown", url: "https://help.blusky.pro/en/articles/16300862-propel-plans-phases-rules-and-costs", accessed: "2026-10-23" }
  - { field: "profit_target_usd", url: "https://help.blusky.pro/en/articles/16300862-propel-plans-phases-rules-and-costs", accessed: "2026-10-23" }
  - { field: "consistency_rule", url: "https://help.blusky.pro/en/articles/16300862-propel-plans-phases-rules-and-costs", accessed: "2026-10-23" }
  - { field: "consistency_rule", url: "https://help.blusky.pro/en/articles/12434059-evaluation-rules", accessed: "2026-10-23" }
  - { field: "max_position", url: "https://help.blusky.pro/en/articles/16300862-propel-plans-phases-rules-and-costs", accessed: "2026-10-23" }
  - { field: "daily_loss_limit", url: "https://help.blusky.pro/en/articles/16300862-propel-plans-phases-rules-and-costs", accessed: "2026-10-23" }
  - { field: "trading_day_reset", url: "https://help.blusky.pro/en/articles/12434059-evaluation-rules", accessed: "2026-10-23" }
  - { field: "commission_per_contract", url: "https://help.blusky.pro/en/articles/17115915-markets-symbols-commissions", accessed: "2026-10-23" }
  - { field: "platforms", url: "https://help.blusky.pro/en/articles/12433926-blusky-supported-platforms", accessed: "2026-10-23" }
  - { field: "copy_trading_allowed", url: "https://help.blusky.pro/en/articles/12433943-multiple-accounts-copy-trading", accessed: "2026-10-23" }
  - { field: "copy_trading_allowed", url: "https://help.blusky.pro/en/articles/12069208-ethical-trading-conduct-policy-blusky-trading-company", accessed: "2026-10-23" }
```

#### Propel — Buffer Zone — evaluation — 100K

```yaml
firm: "blusky"
plan: "Propel — Buffer Zone"
phase: "evaluation"
size_label: "100K"
size_usd: "100000.00"
instruments: ["MNQ", "NQ"]
platforms: ["NinjaTrader 8 (via Tradovate connection; via Rithmic connection, phase scope conflicts)", "Tradovate", "TradingView (Tradovate list)", "Rithmic platforms (R|Trader Pro, Quantower, ATAS, Sierra Chart, Bookmap, MotiveWave, Jigsaw, ONYX, others)", "Volumetrica web platform (VolSys/Deepchart)", "Tradesea", "Tickblaze", "BlackArrow"]
drawdown: { amount_usd: "2500.00", type: "trailing_eod", stops_trailing_at: null }
daily_loss_limit: { amount_usd: null, kind: "none" }
profit_target_usd: "3500.00"
consistency_rule: { text: "Largest single profitable day at or below 34% of the sum of profitable days; checked when trying to pass the phase.", threshold_pct: "34" }
max_position: { minis: null, micros: null, scaling: null }
min_trading_days: null
trading_day_reset: { time: "18:00", tz_as_published: "6PM, tz not stated on that page; other pages say EST/ET" }
commission_per_contract: { mnq_usd: null, nq_usd: null, basis: null, includes: "Not published for this phase: funded commission is 'Set by brokerage' (Rithmic/Volumetrica/Tradesea); Buffer not listed for Rithmic/Volumetrica/Tradesea; Tradovate table has no stated basis (section 8)." }
copy_trading_allowed: "conflict"
automated_trading_allowed: "unknown"
automation_rule_text: "Coordinated Manipulation: Engaging, individually or with others (including across multiple accounts), in trading patterns whose primary intent is to manipulate, abuse, or secure an unfair advantage on our platform. This includes violating our terms and conditions, or using automated software (AI, ultra-high speed, bulk data entry) to unfairly influence outcomes."
vps_allowed: "unknown"
personal_device_required: "unknown"
status: "partial"
not_confirmed_fields: ["copy_trading_allowed", "automated_trading_allowed", "vps_allowed", "personal_device_required", "trading_day_reset", "commission_per_contract", "max_position", "drawdown.stops_trailing_at"]
notes: "Buffer Zone: second phase, no daily loss limit, 'not another evaluation phase'. Contract limits for the Buffer Zone not stated on the plan page. Buffer reset $250 each for the first three, $300 for 4-6."
sources:
  - { field: "drawdown", url: "https://help.blusky.pro/en/articles/16300862-propel-plans-phases-rules-and-costs", accessed: "2026-10-23" }
  - { field: "profit_target_usd", url: "https://help.blusky.pro/en/articles/16300862-propel-plans-phases-rules-and-costs", accessed: "2026-10-23" }
  - { field: "consistency_rule", url: "https://help.blusky.pro/en/articles/16300862-propel-plans-phases-rules-and-costs", accessed: "2026-10-23" }
  - { field: "daily_loss_limit", url: "https://help.blusky.pro/en/articles/12434458-blulive-buffer-stage", accessed: "2026-10-23" }
```

#### Propel — funded — 100K

```yaml
firm: "blusky"
plan: "Propel — Sim Funded"
phase: "funded"
size_label: "100K"
size_usd: "100000.00"
instruments: ["MNQ", "NQ"]
platforms: ["NinjaTrader 8 (via Tradovate connection; via Rithmic connection, phase scope conflicts)", "Tradovate", "TradingView (Tradovate list)", "Rithmic platforms (R|Trader Pro, Quantower, ATAS, Sierra Chart, Bookmap, MotiveWave, Jigsaw, ONYX, others)", "Volumetrica web platform (VolSys/Deepchart)", "Tradesea", "Tickblaze", "BlackArrow"]
drawdown: { amount_usd: null, type: "static", stops_trailing_at: "static; account minimum balance $100 (starting balance 3500.00)" }
daily_loss_limit: { amount_usd: null, kind: "none" }
profit_target_usd: "none"
consistency_rule: { text: "No consistency rule in Sim Funded for Propel (Evaluation Overview).", threshold_pct: null }
max_position: { minis: null, micros: null, scaling: "Sim Funded limits conflict across official pages; actual limits are those displayed on the account (see section 10)." }
min_trading_days: null
trading_day_reset: { time: "18:00", tz_as_published: "6PM, tz not stated on that page; other pages say EST/ET" }
commission_per_contract: { mnq_usd: null, nq_usd: null, basis: null, includes: "Not published for this phase: funded commission is 'Set by brokerage' (Rithmic/Volumetrica/Tradesea); Buffer not listed for Rithmic/Volumetrica/Tradesea; Tradovate table has no stated basis (section 8)." }
copy_trading_allowed: "conflict"
automated_trading_allowed: "unknown"
automation_rule_text: "Coordinated Manipulation: Engaging, individually or with others (including across multiple accounts), in trading patterns whose primary intent is to manipulate, abuse, or secure an unfair advantage on our platform. This includes violating our terms and conditions, or using automated software (AI, ultra-high speed, bulk data entry) to unfairly influence outcomes."
vps_allowed: "unknown"
personal_device_required: "unknown"
status: "conflict"
not_confirmed_fields: ["copy_trading_allowed", "automated_trading_allowed", "vps_allowed", "personal_device_required", "trading_day_reset", "commission_per_contract", "max_position", "drawdown.amount_usd"]
notes: "Sim Funded: simulated, profits withdrawable. Starting balance $3500, minimum balance $100, payout minimum $500, rolling 7-day payout cap $3000. Stop trading for the day at $5,000 daily profit and for the week at $10,000. Risk Team may add daily loss limits and contract limits."
sources:
  - { field: "drawdown", url: "https://help.blusky.pro/en/articles/16300862-propel-plans-phases-rules-and-costs", accessed: "2026-10-23" }
  - { field: "drawdown", url: "https://help.blusky.pro/en/articles/12434469-sim-funded-accounts-processing", accessed: "2026-10-23" }
  - { field: "consistency_rule", url: "https://help.blusky.pro/en/articles/14184170-evaluation-overview", accessed: "2026-10-23" }
  - { field: "max_position", url: "https://help.blusky.pro/en/articles/12434458-blulive-buffer-stage", accessed: "2026-10-23" }
  - { field: "max_position", url: "https://help.blusky.pro/en/articles/17297032-sim-funded-accounts-rules", accessed: "2026-10-23" }
  - { field: "max_position", url: "https://help.blusky.pro/en/articles/11760172-scaling-contract-size-in-sim-funded-brokerage-accounts", accessed: "2026-10-23" }
```

#### Propel — evaluation — 150K

```yaml
firm: "blusky"
plan: "Propel"
phase: "evaluation"
size_label: "150K"
size_usd: "150000.00"
instruments: ["MNQ", "NQ"]
platforms: ["NinjaTrader 8 (via Tradovate connection; via Rithmic connection, phase scope conflicts)", "Tradovate", "TradingView (Tradovate list)", "Rithmic platforms (R|Trader Pro, Quantower, ATAS, Sierra Chart, Bookmap, MotiveWave, Jigsaw, ONYX, others)", "Volumetrica web platform (VolSys/Deepchart)", "Tradesea", "Tickblaze", "BlackArrow"]
drawdown: { amount_usd: "1000.00", type: "static", stops_trailing_at: "static: never moves" }
daily_loss_limit: { amount_usd: null, kind: "none" }
profit_target_usd: "3000.00"
consistency_rule: { text: "Largest single profitable day at or below 34% of the sum of profitable days (published: $1,020 = 34% of target). Evaluation Rules page words the method as: highest profit day becomes the % of your new target.", threshold_pct: "34" }
max_position: { minis: 1, micros: 10, scaling: null }
min_trading_days: null
trading_day_reset: { time: "18:00", tz_as_published: "6PM, tz not stated on that page; other pages say EST/ET" }
commission_per_contract: { mnq_usd: "0.50", nq_usd: "2.00", basis: "per_side", includes: "Applies to Rithmic, Volumetrica and Tradesea evaluation accounts only. Page does not say what the figure includes. Tradovate/NinjaTrader: table with no basis stated (see section 8)." }
copy_trading_allowed: "conflict"
automated_trading_allowed: "unknown"
automation_rule_text: "Coordinated Manipulation: Engaging, individually or with others (including across multiple accounts), in trading patterns whose primary intent is to manipulate, abuse, or secure an unfair advantage on our platform. This includes violating our terms and conditions, or using automated software (AI, ultra-high speed, bulk data entry) to unfairly influence outcomes."
vps_allowed: "unknown"
personal_device_required: "unknown"
status: "conflict"
not_confirmed_fields: ["copy_trading_allowed", "automated_trading_allowed", "vps_allowed", "personal_device_required", "trading_day_reset", "commission_per_contract", "consistency_rule.threshold_pct"]
notes: "No launch fee. Subscription billed every 30 days (amount on the dynamic plans page, not captured). Evaluation reset $79. Target cannot be reached in fewer than 3 profitable days (firm's words); no explicit minimum trading days rule. CONFLICT: the Static Growth Plan page says 30% for 150K/200K (https://help.blusky.pro/en/articles/12433986-static-growth-plan); the Propel page, Evaluation Rules and Evaluation Overview say 34%."
sources:
  - { field: "drawdown", url: "https://help.blusky.pro/en/articles/16300862-propel-plans-phases-rules-and-costs", accessed: "2026-10-23" }
  - { field: "profit_target_usd", url: "https://help.blusky.pro/en/articles/16300862-propel-plans-phases-rules-and-costs", accessed: "2026-10-23" }
  - { field: "consistency_rule", url: "https://help.blusky.pro/en/articles/16300862-propel-plans-phases-rules-and-costs", accessed: "2026-10-23" }
  - { field: "consistency_rule", url: "https://help.blusky.pro/en/articles/12434059-evaluation-rules", accessed: "2026-10-23" }
  - { field: "max_position", url: "https://help.blusky.pro/en/articles/16300862-propel-plans-phases-rules-and-costs", accessed: "2026-10-23" }
  - { field: "daily_loss_limit", url: "https://help.blusky.pro/en/articles/16300862-propel-plans-phases-rules-and-costs", accessed: "2026-10-23" }
  - { field: "trading_day_reset", url: "https://help.blusky.pro/en/articles/12434059-evaluation-rules", accessed: "2026-10-23" }
  - { field: "commission_per_contract", url: "https://help.blusky.pro/en/articles/17115915-markets-symbols-commissions", accessed: "2026-10-23" }
  - { field: "platforms", url: "https://help.blusky.pro/en/articles/12433926-blusky-supported-platforms", accessed: "2026-10-23" }
  - { field: "copy_trading_allowed", url: "https://help.blusky.pro/en/articles/12433943-multiple-accounts-copy-trading", accessed: "2026-10-23" }
  - { field: "copy_trading_allowed", url: "https://help.blusky.pro/en/articles/12069208-ethical-trading-conduct-policy-blusky-trading-company", accessed: "2026-10-23" }
  - { field: "daily_loss_limit", url: "https://help.blusky.pro/en/articles/12433986-static-growth-plan", accessed: "2026-10-23" }
  - { field: "consistency_rule", url: "https://help.blusky.pro/en/articles/12433986-static-growth-plan", accessed: "2026-10-23" }
```

#### Propel — Buffer Zone — evaluation — 150K

```yaml
firm: "blusky"
plan: "Propel — Buffer Zone"
phase: "evaluation"
size_label: "150K"
size_usd: "150000.00"
instruments: ["MNQ", "NQ"]
platforms: ["NinjaTrader 8 (via Tradovate connection; via Rithmic connection, phase scope conflicts)", "Tradovate", "TradingView (Tradovate list)", "Rithmic platforms (R|Trader Pro, Quantower, ATAS, Sierra Chart, Bookmap, MotiveWave, Jigsaw, ONYX, others)", "Volumetrica web platform (VolSys/Deepchart)", "Tradesea", "Tickblaze", "BlackArrow"]
drawdown: { amount_usd: "1000.00", type: "static", stops_trailing_at: "static: never moves" }
daily_loss_limit: { amount_usd: null, kind: "none" }
profit_target_usd: "2000.00"
consistency_rule: { text: "Largest single profitable day at or below 34% of the sum of profitable days; checked when trying to pass the phase.", threshold_pct: "34" }
max_position: { minis: null, micros: null, scaling: null }
min_trading_days: null
trading_day_reset: { time: "18:00", tz_as_published: "6PM, tz not stated on that page; other pages say EST/ET" }
commission_per_contract: { mnq_usd: null, nq_usd: null, basis: null, includes: "Not published for this phase: funded commission is 'Set by brokerage' (Rithmic/Volumetrica/Tradesea); Buffer not listed for Rithmic/Volumetrica/Tradesea; Tradovate table has no stated basis (section 8)." }
copy_trading_allowed: "conflict"
automated_trading_allowed: "unknown"
automation_rule_text: "Coordinated Manipulation: Engaging, individually or with others (including across multiple accounts), in trading patterns whose primary intent is to manipulate, abuse, or secure an unfair advantage on our platform. This includes violating our terms and conditions, or using automated software (AI, ultra-high speed, bulk data entry) to unfairly influence outcomes."
vps_allowed: "unknown"
personal_device_required: "unknown"
status: "partial"
not_confirmed_fields: ["copy_trading_allowed", "automated_trading_allowed", "vps_allowed", "personal_device_required", "trading_day_reset", "commission_per_contract", "max_position"]
notes: "Buffer Zone: second phase, no daily loss limit, 'not another evaluation phase'. Contract limits for the Buffer Zone not stated on the plan page. Buffer reset $250 each for the first three, $300 for 4-6."
sources:
  - { field: "drawdown", url: "https://help.blusky.pro/en/articles/16300862-propel-plans-phases-rules-and-costs", accessed: "2026-10-23" }
  - { field: "profit_target_usd", url: "https://help.blusky.pro/en/articles/16300862-propel-plans-phases-rules-and-costs", accessed: "2026-10-23" }
  - { field: "consistency_rule", url: "https://help.blusky.pro/en/articles/16300862-propel-plans-phases-rules-and-costs", accessed: "2026-10-23" }
  - { field: "daily_loss_limit", url: "https://help.blusky.pro/en/articles/12434458-blulive-buffer-stage", accessed: "2026-10-23" }
```

#### Propel — funded — 150K

```yaml
firm: "blusky"
plan: "Propel — Sim Funded"
phase: "funded"
size_label: "150K"
size_usd: "150000.00"
instruments: ["MNQ", "NQ"]
platforms: ["NinjaTrader 8 (via Tradovate connection; via Rithmic connection, phase scope conflicts)", "Tradovate", "TradingView (Tradovate list)", "Rithmic platforms (R|Trader Pro, Quantower, ATAS, Sierra Chart, Bookmap, MotiveWave, Jigsaw, ONYX, others)", "Volumetrica web platform (VolSys/Deepchart)", "Tradesea", "Tickblaze", "BlackArrow"]
drawdown: { amount_usd: null, type: "static", stops_trailing_at: "static; account minimum balance $100 (starting balance 2000.00)" }
daily_loss_limit: { amount_usd: null, kind: "none" }
profit_target_usd: "none"
consistency_rule: { text: "No consistency rule in Sim Funded for Propel (Evaluation Overview).", threshold_pct: null }
max_position: { minis: null, micros: null, scaling: "Sim Funded limits conflict across official pages; actual limits are those displayed on the account (see section 10)." }
min_trading_days: null
trading_day_reset: { time: "18:00", tz_as_published: "6PM, tz not stated on that page; other pages say EST/ET" }
commission_per_contract: { mnq_usd: null, nq_usd: null, basis: null, includes: "Not published for this phase: funded commission is 'Set by brokerage' (Rithmic/Volumetrica/Tradesea); Buffer not listed for Rithmic/Volumetrica/Tradesea; Tradovate table has no stated basis (section 8)." }
copy_trading_allowed: "conflict"
automated_trading_allowed: "unknown"
automation_rule_text: "Coordinated Manipulation: Engaging, individually or with others (including across multiple accounts), in trading patterns whose primary intent is to manipulate, abuse, or secure an unfair advantage on our platform. This includes violating our terms and conditions, or using automated software (AI, ultra-high speed, bulk data entry) to unfairly influence outcomes."
vps_allowed: "unknown"
personal_device_required: "unknown"
status: "conflict"
not_confirmed_fields: ["copy_trading_allowed", "automated_trading_allowed", "vps_allowed", "personal_device_required", "trading_day_reset", "commission_per_contract", "max_position", "starting_balance", "drawdown.amount_usd"]
notes: "Sim Funded: simulated, profits withdrawable. Starting balance $2000, minimum balance $100, payout minimum $500, rolling 7-day payout cap $1500. Stop trading for the day at $5,000 daily profit and for the week at $10,000. Risk Team may add daily loss limits and contract limits. CONFLICT: Sim Funded Accounts Processing lists starting balance $1500 for 150K; Propel Plans lists $2000."
sources:
  - { field: "drawdown", url: "https://help.blusky.pro/en/articles/16300862-propel-plans-phases-rules-and-costs", accessed: "2026-10-23" }
  - { field: "drawdown", url: "https://help.blusky.pro/en/articles/12434469-sim-funded-accounts-processing", accessed: "2026-10-23" }
  - { field: "consistency_rule", url: "https://help.blusky.pro/en/articles/14184170-evaluation-overview", accessed: "2026-10-23" }
  - { field: "max_position", url: "https://help.blusky.pro/en/articles/12434458-blulive-buffer-stage", accessed: "2026-10-23" }
  - { field: "max_position", url: "https://help.blusky.pro/en/articles/17297032-sim-funded-accounts-rules", accessed: "2026-10-23" }
  - { field: "max_position", url: "https://help.blusky.pro/en/articles/11760172-scaling-contract-size-in-sim-funded-brokerage-accounts", accessed: "2026-10-23" }
```

#### Propel — evaluation — 200K

```yaml
firm: "blusky"
plan: "Propel"
phase: "evaluation"
size_label: "200K"
size_usd: "200000.00"
instruments: ["MNQ", "NQ"]
platforms: ["NinjaTrader 8 (via Tradovate connection; via Rithmic connection, phase scope conflicts)", "Tradovate", "TradingView (Tradovate list)", "Rithmic platforms (R|Trader Pro, Quantower, ATAS, Sierra Chart, Bookmap, MotiveWave, Jigsaw, ONYX, others)", "Volumetrica web platform (VolSys/Deepchart)", "Tradesea", "Tickblaze", "BlackArrow"]
drawdown: { amount_usd: "2000.00", type: "static", stops_trailing_at: "static: never moves" }
daily_loss_limit: { amount_usd: null, kind: "none" }
profit_target_usd: "6000.00"
consistency_rule: { text: "Largest single profitable day at or below 34% of the sum of profitable days (published: $2,040 = 34% of target). Evaluation Rules page words the method as: highest profit day becomes the % of your new target.", threshold_pct: "34" }
max_position: { minis: 2, micros: 20, scaling: null }
min_trading_days: null
trading_day_reset: { time: "18:00", tz_as_published: "6PM, tz not stated on that page; other pages say EST/ET" }
commission_per_contract: { mnq_usd: "0.50", nq_usd: "2.00", basis: "per_side", includes: "Applies to Rithmic, Volumetrica and Tradesea evaluation accounts only. Page does not say what the figure includes. Tradovate/NinjaTrader: table with no basis stated (see section 8)." }
copy_trading_allowed: "conflict"
automated_trading_allowed: "unknown"
automation_rule_text: "Coordinated Manipulation: Engaging, individually or with others (including across multiple accounts), in trading patterns whose primary intent is to manipulate, abuse, or secure an unfair advantage on our platform. This includes violating our terms and conditions, or using automated software (AI, ultra-high speed, bulk data entry) to unfairly influence outcomes."
vps_allowed: "unknown"
personal_device_required: "unknown"
status: "conflict"
not_confirmed_fields: ["copy_trading_allowed", "automated_trading_allowed", "vps_allowed", "personal_device_required", "trading_day_reset", "commission_per_contract", "consistency_rule.threshold_pct"]
notes: "No launch fee. Subscription billed every 30 days (amount on the dynamic plans page, not captured). Evaluation reset $79. Target cannot be reached in fewer than 3 profitable days (firm's words); no explicit minimum trading days rule. CONFLICT: the Static Growth Plan page says 30% for 150K/200K (https://help.blusky.pro/en/articles/12433986-static-growth-plan); the Propel page, Evaluation Rules and Evaluation Overview say 34%."
sources:
  - { field: "drawdown", url: "https://help.blusky.pro/en/articles/16300862-propel-plans-phases-rules-and-costs", accessed: "2026-10-23" }
  - { field: "profit_target_usd", url: "https://help.blusky.pro/en/articles/16300862-propel-plans-phases-rules-and-costs", accessed: "2026-10-23" }
  - { field: "consistency_rule", url: "https://help.blusky.pro/en/articles/16300862-propel-plans-phases-rules-and-costs", accessed: "2026-10-23" }
  - { field: "consistency_rule", url: "https://help.blusky.pro/en/articles/12434059-evaluation-rules", accessed: "2026-10-23" }
  - { field: "max_position", url: "https://help.blusky.pro/en/articles/16300862-propel-plans-phases-rules-and-costs", accessed: "2026-10-23" }
  - { field: "daily_loss_limit", url: "https://help.blusky.pro/en/articles/16300862-propel-plans-phases-rules-and-costs", accessed: "2026-10-23" }
  - { field: "trading_day_reset", url: "https://help.blusky.pro/en/articles/12434059-evaluation-rules", accessed: "2026-10-23" }
  - { field: "commission_per_contract", url: "https://help.blusky.pro/en/articles/17115915-markets-symbols-commissions", accessed: "2026-10-23" }
  - { field: "platforms", url: "https://help.blusky.pro/en/articles/12433926-blusky-supported-platforms", accessed: "2026-10-23" }
  - { field: "copy_trading_allowed", url: "https://help.blusky.pro/en/articles/12433943-multiple-accounts-copy-trading", accessed: "2026-10-23" }
  - { field: "copy_trading_allowed", url: "https://help.blusky.pro/en/articles/12069208-ethical-trading-conduct-policy-blusky-trading-company", accessed: "2026-10-23" }
  - { field: "daily_loss_limit", url: "https://help.blusky.pro/en/articles/12433986-static-growth-plan", accessed: "2026-10-23" }
  - { field: "consistency_rule", url: "https://help.blusky.pro/en/articles/12433986-static-growth-plan", accessed: "2026-10-23" }
```

#### Propel — Buffer Zone — evaluation — 200K

```yaml
firm: "blusky"
plan: "Propel — Buffer Zone"
phase: "evaluation"
size_label: "200K"
size_usd: "200000.00"
instruments: ["MNQ", "NQ"]
platforms: ["NinjaTrader 8 (via Tradovate connection; via Rithmic connection, phase scope conflicts)", "Tradovate", "TradingView (Tradovate list)", "Rithmic platforms (R|Trader Pro, Quantower, ATAS, Sierra Chart, Bookmap, MotiveWave, Jigsaw, ONYX, others)", "Volumetrica web platform (VolSys/Deepchart)", "Tradesea", "Tickblaze", "BlackArrow"]
drawdown: { amount_usd: "2000.00", type: "static", stops_trailing_at: "static: never moves" }
daily_loss_limit: { amount_usd: null, kind: "none" }
profit_target_usd: "3000.00"
consistency_rule: { text: "Largest single profitable day at or below 34% of the sum of profitable days; checked when trying to pass the phase.", threshold_pct: "34" }
max_position: { minis: null, micros: null, scaling: null }
min_trading_days: null
trading_day_reset: { time: "18:00", tz_as_published: "6PM, tz not stated on that page; other pages say EST/ET" }
commission_per_contract: { mnq_usd: null, nq_usd: null, basis: null, includes: "Not published for this phase: funded commission is 'Set by brokerage' (Rithmic/Volumetrica/Tradesea); Buffer not listed for Rithmic/Volumetrica/Tradesea; Tradovate table has no stated basis (section 8)." }
copy_trading_allowed: "conflict"
automated_trading_allowed: "unknown"
automation_rule_text: "Coordinated Manipulation: Engaging, individually or with others (including across multiple accounts), in trading patterns whose primary intent is to manipulate, abuse, or secure an unfair advantage on our platform. This includes violating our terms and conditions, or using automated software (AI, ultra-high speed, bulk data entry) to unfairly influence outcomes."
vps_allowed: "unknown"
personal_device_required: "unknown"
status: "partial"
not_confirmed_fields: ["copy_trading_allowed", "automated_trading_allowed", "vps_allowed", "personal_device_required", "trading_day_reset", "commission_per_contract", "max_position"]
notes: "Buffer Zone: second phase, no daily loss limit, 'not another evaluation phase'. Contract limits for the Buffer Zone not stated on the plan page. Buffer reset $250 each for the first three, $300 for 4-6."
sources:
  - { field: "drawdown", url: "https://help.blusky.pro/en/articles/16300862-propel-plans-phases-rules-and-costs", accessed: "2026-10-23" }
  - { field: "profit_target_usd", url: "https://help.blusky.pro/en/articles/16300862-propel-plans-phases-rules-and-costs", accessed: "2026-10-23" }
  - { field: "consistency_rule", url: "https://help.blusky.pro/en/articles/16300862-propel-plans-phases-rules-and-costs", accessed: "2026-10-23" }
  - { field: "daily_loss_limit", url: "https://help.blusky.pro/en/articles/12434458-blulive-buffer-stage", accessed: "2026-10-23" }
```

#### Propel — funded — 200K

```yaml
firm: "blusky"
plan: "Propel — Sim Funded"
phase: "funded"
size_label: "200K"
size_usd: "200000.00"
instruments: ["MNQ", "NQ"]
platforms: ["NinjaTrader 8 (via Tradovate connection; via Rithmic connection, phase scope conflicts)", "Tradovate", "TradingView (Tradovate list)", "Rithmic platforms (R|Trader Pro, Quantower, ATAS, Sierra Chart, Bookmap, MotiveWave, Jigsaw, ONYX, others)", "Volumetrica web platform (VolSys/Deepchart)", "Tradesea", "Tickblaze", "BlackArrow"]
drawdown: { amount_usd: null, type: "static", stops_trailing_at: "static; account minimum balance $100 (starting balance 3000.00)" }
daily_loss_limit: { amount_usd: null, kind: "none" }
profit_target_usd: "none"
consistency_rule: { text: "No consistency rule in Sim Funded for Propel (Evaluation Overview).", threshold_pct: null }
max_position: { minis: null, micros: null, scaling: "Sim Funded limits conflict across official pages; actual limits are those displayed on the account (see section 10)." }
min_trading_days: null
trading_day_reset: { time: "18:00", tz_as_published: "6PM, tz not stated on that page; other pages say EST/ET" }
commission_per_contract: { mnq_usd: null, nq_usd: null, basis: null, includes: "Not published for this phase: funded commission is 'Set by brokerage' (Rithmic/Volumetrica/Tradesea); Buffer not listed for Rithmic/Volumetrica/Tradesea; Tradovate table has no stated basis (section 8)." }
copy_trading_allowed: "conflict"
automated_trading_allowed: "unknown"
automation_rule_text: "Coordinated Manipulation: Engaging, individually or with others (including across multiple accounts), in trading patterns whose primary intent is to manipulate, abuse, or secure an unfair advantage on our platform. This includes violating our terms and conditions, or using automated software (AI, ultra-high speed, bulk data entry) to unfairly influence outcomes."
vps_allowed: "unknown"
personal_device_required: "unknown"
status: "conflict"
not_confirmed_fields: ["copy_trading_allowed", "automated_trading_allowed", "vps_allowed", "personal_device_required", "trading_day_reset", "commission_per_contract", "max_position", "drawdown.amount_usd"]
notes: "Sim Funded: simulated, profits withdrawable. Starting balance $3000, minimum balance $100, payout minimum $500, rolling 7-day payout cap $2500. Stop trading for the day at $5,000 daily profit and for the week at $10,000. Risk Team may add daily loss limits and contract limits."
sources:
  - { field: "drawdown", url: "https://help.blusky.pro/en/articles/16300862-propel-plans-phases-rules-and-costs", accessed: "2026-10-23" }
  - { field: "drawdown", url: "https://help.blusky.pro/en/articles/12434469-sim-funded-accounts-processing", accessed: "2026-10-23" }
  - { field: "consistency_rule", url: "https://help.blusky.pro/en/articles/14184170-evaluation-overview", accessed: "2026-10-23" }
  - { field: "max_position", url: "https://help.blusky.pro/en/articles/12434458-blulive-buffer-stage", accessed: "2026-10-23" }
  - { field: "max_position", url: "https://help.blusky.pro/en/articles/17297032-sim-funded-accounts-rules", accessed: "2026-10-23" }
  - { field: "max_position", url: "https://help.blusky.pro/en/articles/11760172-scaling-contract-size-in-sim-funded-brokerage-accounts", accessed: "2026-10-23" }
```

#### Propel — evaluation — 300K

```yaml
firm: "blusky"
plan: "Propel"
phase: "evaluation"
size_label: "300K"
size_usd: "300000.00"
instruments: ["MNQ", "NQ"]
platforms: ["NinjaTrader 8 (via Tradovate connection; via Rithmic connection, phase scope conflicts)", "Tradovate", "TradingView (Tradovate list)", "Rithmic platforms (R|Trader Pro, Quantower, ATAS, Sierra Chart, Bookmap, MotiveWave, Jigsaw, ONYX, others)", "Volumetrica web platform (VolSys/Deepchart)", "Tradesea", "Tickblaze", "BlackArrow"]
drawdown: { amount_usd: "5000.00", type: "static", stops_trailing_at: "static: never moves" }
daily_loss_limit: { amount_usd: null, kind: "none" }
profit_target_usd: "20000.00"
consistency_rule: { text: "Largest single profitable day at or below 21% of the sum of profitable days (published: $4,200 = 21% of target). Evaluation Rules page words the method as: highest profit day becomes the % of your new target.", threshold_pct: "21" }
max_position: { minis: 5, micros: 50, scaling: null }
min_trading_days: null
trading_day_reset: { time: "18:00", tz_as_published: "6PM, tz not stated on that page; other pages say EST/ET" }
commission_per_contract: { mnq_usd: "0.50", nq_usd: "2.00", basis: "per_side", includes: "Applies to Rithmic, Volumetrica and Tradesea evaluation accounts only. Page does not say what the figure includes. Tradovate/NinjaTrader: table with no basis stated (see section 8)." }
copy_trading_allowed: "conflict"
automated_trading_allowed: "unknown"
automation_rule_text: "Coordinated Manipulation: Engaging, individually or with others (including across multiple accounts), in trading patterns whose primary intent is to manipulate, abuse, or secure an unfair advantage on our platform. This includes violating our terms and conditions, or using automated software (AI, ultra-high speed, bulk data entry) to unfairly influence outcomes."
vps_allowed: "unknown"
personal_device_required: "unknown"
status: "conflict"
not_confirmed_fields: ["copy_trading_allowed", "automated_trading_allowed", "vps_allowed", "personal_device_required", "trading_day_reset", "commission_per_contract", "daily_loss_limit"]
notes: "No launch fee. Subscription billed every 30 days (amount on the dynamic plans page, not captured). Evaluation reset $79. Target cannot be reached in fewer than 3 profitable days (firm's words); no explicit minimum trading days rule. CONFLICT: the Static Growth Plan page lists 'Daily Loss Limit: Capped at $2,500' for the 300K evaluation; the Propel page lists daily loss limit None for every size."
sources:
  - { field: "drawdown", url: "https://help.blusky.pro/en/articles/16300862-propel-plans-phases-rules-and-costs", accessed: "2026-10-23" }
  - { field: "profit_target_usd", url: "https://help.blusky.pro/en/articles/16300862-propel-plans-phases-rules-and-costs", accessed: "2026-10-23" }
  - { field: "consistency_rule", url: "https://help.blusky.pro/en/articles/16300862-propel-plans-phases-rules-and-costs", accessed: "2026-10-23" }
  - { field: "consistency_rule", url: "https://help.blusky.pro/en/articles/12434059-evaluation-rules", accessed: "2026-10-23" }
  - { field: "max_position", url: "https://help.blusky.pro/en/articles/16300862-propel-plans-phases-rules-and-costs", accessed: "2026-10-23" }
  - { field: "daily_loss_limit", url: "https://help.blusky.pro/en/articles/16300862-propel-plans-phases-rules-and-costs", accessed: "2026-10-23" }
  - { field: "trading_day_reset", url: "https://help.blusky.pro/en/articles/12434059-evaluation-rules", accessed: "2026-10-23" }
  - { field: "commission_per_contract", url: "https://help.blusky.pro/en/articles/17115915-markets-symbols-commissions", accessed: "2026-10-23" }
  - { field: "platforms", url: "https://help.blusky.pro/en/articles/12433926-blusky-supported-platforms", accessed: "2026-10-23" }
  - { field: "copy_trading_allowed", url: "https://help.blusky.pro/en/articles/12433943-multiple-accounts-copy-trading", accessed: "2026-10-23" }
  - { field: "copy_trading_allowed", url: "https://help.blusky.pro/en/articles/12069208-ethical-trading-conduct-policy-blusky-trading-company", accessed: "2026-10-23" }
  - { field: "daily_loss_limit", url: "https://help.blusky.pro/en/articles/12433986-static-growth-plan", accessed: "2026-10-23" }
  - { field: "consistency_rule", url: "https://help.blusky.pro/en/articles/12433986-static-growth-plan", accessed: "2026-10-23" }
```

#### Propel — Buffer Zone — evaluation — 300K

```yaml
firm: "blusky"
plan: "Propel — Buffer Zone"
phase: "evaluation"
size_label: "300K"
size_usd: "300000.00"
instruments: ["MNQ", "NQ"]
platforms: ["NinjaTrader 8 (via Tradovate connection; via Rithmic connection, phase scope conflicts)", "Tradovate", "TradingView (Tradovate list)", "Rithmic platforms (R|Trader Pro, Quantower, ATAS, Sierra Chart, Bookmap, MotiveWave, Jigsaw, ONYX, others)", "Volumetrica web platform (VolSys/Deepchart)", "Tradesea", "Tickblaze", "BlackArrow"]
drawdown: { amount_usd: "2500.00", type: "static", stops_trailing_at: "static: never moves" }
daily_loss_limit: { amount_usd: null, kind: "none" }
profit_target_usd: "3500.00"
consistency_rule: { text: "Largest single profitable day at or below 21% of the sum of profitable days; checked when trying to pass the phase.", threshold_pct: "21" }
max_position: { minis: null, micros: null, scaling: null }
min_trading_days: null
trading_day_reset: { time: "18:00", tz_as_published: "6PM, tz not stated on that page; other pages say EST/ET" }
commission_per_contract: { mnq_usd: null, nq_usd: null, basis: null, includes: "Not published for this phase: funded commission is 'Set by brokerage' (Rithmic/Volumetrica/Tradesea); Buffer not listed for Rithmic/Volumetrica/Tradesea; Tradovate table has no stated basis (section 8)." }
copy_trading_allowed: "conflict"
automated_trading_allowed: "unknown"
automation_rule_text: "Coordinated Manipulation: Engaging, individually or with others (including across multiple accounts), in trading patterns whose primary intent is to manipulate, abuse, or secure an unfair advantage on our platform. This includes violating our terms and conditions, or using automated software (AI, ultra-high speed, bulk data entry) to unfairly influence outcomes."
vps_allowed: "unknown"
personal_device_required: "unknown"
status: "partial"
not_confirmed_fields: ["copy_trading_allowed", "automated_trading_allowed", "vps_allowed", "personal_device_required", "trading_day_reset", "commission_per_contract", "max_position"]
notes: "Buffer Zone: second phase, no daily loss limit, 'not another evaluation phase'. Contract limits for the Buffer Zone not stated on the plan page. Buffer reset $250 each for the first three, $300 for 4-6."
sources:
  - { field: "drawdown", url: "https://help.blusky.pro/en/articles/16300862-propel-plans-phases-rules-and-costs", accessed: "2026-10-23" }
  - { field: "profit_target_usd", url: "https://help.blusky.pro/en/articles/16300862-propel-plans-phases-rules-and-costs", accessed: "2026-10-23" }
  - { field: "consistency_rule", url: "https://help.blusky.pro/en/articles/16300862-propel-plans-phases-rules-and-costs", accessed: "2026-10-23" }
  - { field: "daily_loss_limit", url: "https://help.blusky.pro/en/articles/12434458-blulive-buffer-stage", accessed: "2026-10-23" }
```

#### Propel — funded — 300K

```yaml
firm: "blusky"
plan: "Propel — Sim Funded"
phase: "funded"
size_label: "300K"
size_usd: "300000.00"
instruments: ["MNQ", "NQ"]
platforms: ["NinjaTrader 8 (via Tradovate connection; via Rithmic connection, phase scope conflicts)", "Tradovate", "TradingView (Tradovate list)", "Rithmic platforms (R|Trader Pro, Quantower, ATAS, Sierra Chart, Bookmap, MotiveWave, Jigsaw, ONYX, others)", "Volumetrica web platform (VolSys/Deepchart)", "Tradesea", "Tickblaze", "BlackArrow"]
drawdown: { amount_usd: null, type: "static", stops_trailing_at: "static; account minimum balance $100 (starting balance 3500.00)" }
daily_loss_limit: { amount_usd: null, kind: "none" }
profit_target_usd: "none"
consistency_rule: { text: "No consistency rule in Sim Funded for Propel (Evaluation Overview).", threshold_pct: null }
max_position: { minis: null, micros: null, scaling: "Sim Funded limits conflict across official pages; actual limits are those displayed on the account (see section 10)." }
min_trading_days: null
trading_day_reset: { time: "18:00", tz_as_published: "6PM, tz not stated on that page; other pages say EST/ET" }
commission_per_contract: { mnq_usd: null, nq_usd: null, basis: null, includes: "Not published for this phase: funded commission is 'Set by brokerage' (Rithmic/Volumetrica/Tradesea); Buffer not listed for Rithmic/Volumetrica/Tradesea; Tradovate table has no stated basis (section 8)." }
copy_trading_allowed: "conflict"
automated_trading_allowed: "unknown"
automation_rule_text: "Coordinated Manipulation: Engaging, individually or with others (including across multiple accounts), in trading patterns whose primary intent is to manipulate, abuse, or secure an unfair advantage on our platform. This includes violating our terms and conditions, or using automated software (AI, ultra-high speed, bulk data entry) to unfairly influence outcomes."
vps_allowed: "unknown"
personal_device_required: "unknown"
status: "conflict"
not_confirmed_fields: ["copy_trading_allowed", "automated_trading_allowed", "vps_allowed", "personal_device_required", "trading_day_reset", "commission_per_contract", "max_position", "starting_balance", "drawdown.amount_usd"]
notes: "Sim Funded: simulated, profits withdrawable. Starting balance $3500, minimum balance $100, payout minimum $500, rolling 7-day payout cap $3000. Stop trading for the day at $5,000 daily profit and for the week at $10,000. Risk Team may add daily loss limits and contract limits. CONFLICT: Sim Funded Accounts Processing lists starting balance $3000 for 300K; Propel Plans lists $3500."
sources:
  - { field: "drawdown", url: "https://help.blusky.pro/en/articles/16300862-propel-plans-phases-rules-and-costs", accessed: "2026-10-23" }
  - { field: "drawdown", url: "https://help.blusky.pro/en/articles/12434469-sim-funded-accounts-processing", accessed: "2026-10-23" }
  - { field: "consistency_rule", url: "https://help.blusky.pro/en/articles/14184170-evaluation-overview", accessed: "2026-10-23" }
  - { field: "max_position", url: "https://help.blusky.pro/en/articles/12434458-blulive-buffer-stage", accessed: "2026-10-23" }
  - { field: "max_position", url: "https://help.blusky.pro/en/articles/17297032-sim-funded-accounts-rules", accessed: "2026-10-23" }
  - { field: "max_position", url: "https://help.blusky.pro/en/articles/11760172-scaling-contract-size-in-sim-funded-brokerage-accounts", accessed: "2026-10-23" }
```

#### Orbit — evaluation — 50K

```yaml
firm: "blusky"
plan: "Orbit"
phase: "evaluation"
size_label: "50K"
size_usd: "50000.00"
instruments: ["MNQ", "NQ"]
platforms: ["NinjaTrader 8 (via Tradovate connection; via Rithmic connection, phase scope conflicts)", "Tradovate", "TradingView (Tradovate list)", "Rithmic platforms (R|Trader Pro, Quantower, ATAS, Sierra Chart, Bookmap, MotiveWave, Jigsaw, ONYX, others)", "Volumetrica web platform (VolSys/Deepchart)", "Tradesea", "Tickblaze", "BlackArrow"]
drawdown: { amount_usd: "2000.00", type: "trailing_eod", stops_trailing_at: null }
daily_loss_limit: { amount_usd: null, kind: "none" }
profit_target_usd: "3000.00"
consistency_rule: { text: "No consistency rule in the Orbit evaluation.", threshold_pct: null }
max_position: { minis: 5, micros: 50, scaling: null }
min_trading_days: null
trading_day_reset: { time: "18:00", tz_as_published: "6PM, tz not stated on that page; other pages say EST/ET" }
commission_per_contract: { mnq_usd: "0.50", nq_usd: "2.00", basis: "per_side", includes: "Applies to Rithmic, Volumetrica and Tradesea evaluation accounts only. Page does not say what the figure includes. Tradovate/NinjaTrader: table with no basis stated (see section 8)." }
copy_trading_allowed: "conflict"
automated_trading_allowed: "unknown"
automation_rule_text: "Coordinated Manipulation: Engaging, individually or with others (including across multiple accounts), in trading patterns whose primary intent is to manipulate, abuse, or secure an unfair advantage on our platform. This includes violating our terms and conditions, or using automated software (AI, ultra-high speed, bulk data entry) to unfairly influence outcomes."
vps_allowed: "unknown"
personal_device_required: "unknown"
status: "partial"
not_confirmed_fields: ["copy_trading_allowed", "automated_trading_allowed", "vps_allowed", "personal_device_required", "trading_day_reset", "commission_per_contract", "drawdown.stops_trailing_at"]
notes: "One-time purchase (not a subscription); evaluation valid 30 days from purchase, a reset does not extend it. Can be passed in as little as a single day (firm's words). Evaluation reset: 50K $129, 100K $199, 150K $279, 200K $279. Only two phases (no Buffer Zone) per the Orbit page; the Evaluation Overview still lists an Orbit Buffer phase with 50% consistency (conflict listed in section 10)."
sources:
  - { field: "drawdown", url: "https://help.blusky.pro/en/articles/16300866-orbit-plans-two-phases-faster-payouts", accessed: "2026-10-23" }
  - { field: "profit_target_usd", url: "https://help.blusky.pro/en/articles/16300866-orbit-plans-two-phases-faster-payouts", accessed: "2026-10-23" }
  - { field: "consistency_rule", url: "https://help.blusky.pro/en/articles/16300866-orbit-plans-two-phases-faster-payouts", accessed: "2026-10-23" }
  - { field: "max_position", url: "https://help.blusky.pro/en/articles/16300866-orbit-plans-two-phases-faster-payouts", accessed: "2026-10-23" }
  - { field: "daily_loss_limit", url: "https://help.blusky.pro/en/articles/16300866-orbit-plans-two-phases-faster-payouts", accessed: "2026-10-23" }
  - { field: "trading_day_reset", url: "https://help.blusky.pro/en/articles/12434059-evaluation-rules", accessed: "2026-10-23" }
  - { field: "commission_per_contract", url: "https://help.blusky.pro/en/articles/17115915-markets-symbols-commissions", accessed: "2026-10-23" }
  - { field: "platforms", url: "https://help.blusky.pro/en/articles/12433926-blusky-supported-platforms", accessed: "2026-10-23" }
  - { field: "copy_trading_allowed", url: "https://help.blusky.pro/en/articles/12433943-multiple-accounts-copy-trading", accessed: "2026-10-23" }
  - { field: "copy_trading_allowed", url: "https://help.blusky.pro/en/articles/12069208-ethical-trading-conduct-policy-blusky-trading-company", accessed: "2026-10-23" }
```

#### Orbit — funded — 50K

```yaml
firm: "blusky"
plan: "Orbit — Sim Funded"
phase: "funded"
size_label: "50K"
size_usd: "50000.00"
instruments: ["MNQ", "NQ"]
platforms: ["NinjaTrader 8 (via Tradovate connection; via Rithmic connection, phase scope conflicts)", "Tradovate", "TradingView (Tradovate list)", "Rithmic platforms (R|Trader Pro, Quantower, ATAS, Sierra Chart, Bookmap, MotiveWave, Jigsaw, ONYX, others)", "Volumetrica web platform (VolSys/Deepchart)", "Tradesea", "Tickblaze", "BlackArrow"]
drawdown: { amount_usd: "2000.00", type: "trailing_eod", stops_trailing_at: "Locks permanently at the starting balance once lifetime net profit equals the starting drawdown ($2000 net profit); the floor trails closed daily profit dollar-for-dollar" }
daily_loss_limit: { amount_usd: null, kind: "none" }
profit_target_usd: "none"
consistency_rule: { text: "40% consistency rule applies while Sim Funded; largest single profitable day vs the sum of profitable days; resets after each processed payout.", threshold_pct: "40" }
max_position: { minis: 2, micros: 20, scaling: "Generic Sim Funded pages state limits can be scaled by margin (Tradovate) or by support request; Rithmic and Volumetrica do not scale." }
min_trading_days: null
trading_day_reset: { time: "18:00", tz_as_published: "6PM, tz not stated on that page; other pages say EST/ET" }
commission_per_contract: { mnq_usd: null, nq_usd: null, basis: null, includes: "Not published for this phase: funded commission is 'Set by brokerage' (Rithmic/Volumetrica/Tradesea); Buffer not listed for Rithmic/Volumetrica/Tradesea; Tradovate table has no stated basis (section 8)." }
copy_trading_allowed: "conflict"
automated_trading_allowed: "unknown"
automation_rule_text: "Coordinated Manipulation: Engaging, individually or with others (including across multiple accounts), in trading patterns whose primary intent is to manipulate, abuse, or secure an unfair advantage on our platform. This includes violating our terms and conditions, or using automated software (AI, ultra-high speed, bulk data entry) to unfairly influence outcomes."
vps_allowed: "unknown"
personal_device_required: "unknown"
status: "partial"
not_confirmed_fields: ["copy_trading_allowed", "automated_trading_allowed", "vps_allowed", "personal_device_required", "trading_day_reset", "commission_per_contract"]
notes: "Sim Funded opens at the full account size. Payouts every three trading days once balance exceeds the threshold; payout minimum $500; rolling 7-day cap $2000. Sim Funded Accounts Processing describes Orbit as a $0 starting balance with a negative floor (framing difference, same drawdown for 50K/100K/200K; it omits 150K). Orbit Brokerage review ceiling per account: 50K $10,000, 100K $12,500, 150K $15,000, 200K $12,500."
sources:
  - { field: "drawdown", url: "https://help.blusky.pro/en/articles/16300866-orbit-plans-two-phases-faster-payouts", accessed: "2026-10-23" }
  - { field: "consistency_rule", url: "https://help.blusky.pro/en/articles/16300866-orbit-plans-two-phases-faster-payouts", accessed: "2026-10-23" }
  - { field: "max_position", url: "https://help.blusky.pro/en/articles/16300866-orbit-plans-two-phases-faster-payouts", accessed: "2026-10-23" }
  - { field: "payouts", url: "https://help.blusky.pro/en/articles/15756814-orbit-v2-accounts-how-do-payouts-work", accessed: "2026-10-23" }
```

#### Orbit — evaluation — 100K

```yaml
firm: "blusky"
plan: "Orbit"
phase: "evaluation"
size_label: "100K"
size_usd: "100000.00"
instruments: ["MNQ", "NQ"]
platforms: ["NinjaTrader 8 (via Tradovate connection; via Rithmic connection, phase scope conflicts)", "Tradovate", "TradingView (Tradovate list)", "Rithmic platforms (R|Trader Pro, Quantower, ATAS, Sierra Chart, Bookmap, MotiveWave, Jigsaw, ONYX, others)", "Volumetrica web platform (VolSys/Deepchart)", "Tradesea", "Tickblaze", "BlackArrow"]
drawdown: { amount_usd: "3000.00", type: "trailing_eod", stops_trailing_at: null }
daily_loss_limit: { amount_usd: null, kind: "none" }
profit_target_usd: "6000.00"
consistency_rule: { text: "No consistency rule in the Orbit evaluation.", threshold_pct: null }
max_position: { minis: 10, micros: 100, scaling: null }
min_trading_days: null
trading_day_reset: { time: "18:00", tz_as_published: "6PM, tz not stated on that page; other pages say EST/ET" }
commission_per_contract: { mnq_usd: "0.50", nq_usd: "2.00", basis: "per_side", includes: "Applies to Rithmic, Volumetrica and Tradesea evaluation accounts only. Page does not say what the figure includes. Tradovate/NinjaTrader: table with no basis stated (see section 8)." }
copy_trading_allowed: "conflict"
automated_trading_allowed: "unknown"
automation_rule_text: "Coordinated Manipulation: Engaging, individually or with others (including across multiple accounts), in trading patterns whose primary intent is to manipulate, abuse, or secure an unfair advantage on our platform. This includes violating our terms and conditions, or using automated software (AI, ultra-high speed, bulk data entry) to unfairly influence outcomes."
vps_allowed: "unknown"
personal_device_required: "unknown"
status: "partial"
not_confirmed_fields: ["copy_trading_allowed", "automated_trading_allowed", "vps_allowed", "personal_device_required", "trading_day_reset", "commission_per_contract", "drawdown.stops_trailing_at"]
notes: "One-time purchase (not a subscription); evaluation valid 30 days from purchase, a reset does not extend it. Can be passed in as little as a single day (firm's words). Evaluation reset: 50K $129, 100K $199, 150K $279, 200K $279. Only two phases (no Buffer Zone) per the Orbit page; the Evaluation Overview still lists an Orbit Buffer phase with 50% consistency (conflict listed in section 10)."
sources:
  - { field: "drawdown", url: "https://help.blusky.pro/en/articles/16300866-orbit-plans-two-phases-faster-payouts", accessed: "2026-10-23" }
  - { field: "profit_target_usd", url: "https://help.blusky.pro/en/articles/16300866-orbit-plans-two-phases-faster-payouts", accessed: "2026-10-23" }
  - { field: "consistency_rule", url: "https://help.blusky.pro/en/articles/16300866-orbit-plans-two-phases-faster-payouts", accessed: "2026-10-23" }
  - { field: "max_position", url: "https://help.blusky.pro/en/articles/16300866-orbit-plans-two-phases-faster-payouts", accessed: "2026-10-23" }
  - { field: "daily_loss_limit", url: "https://help.blusky.pro/en/articles/16300866-orbit-plans-two-phases-faster-payouts", accessed: "2026-10-23" }
  - { field: "trading_day_reset", url: "https://help.blusky.pro/en/articles/12434059-evaluation-rules", accessed: "2026-10-23" }
  - { field: "commission_per_contract", url: "https://help.blusky.pro/en/articles/17115915-markets-symbols-commissions", accessed: "2026-10-23" }
  - { field: "platforms", url: "https://help.blusky.pro/en/articles/12433926-blusky-supported-platforms", accessed: "2026-10-23" }
  - { field: "copy_trading_allowed", url: "https://help.blusky.pro/en/articles/12433943-multiple-accounts-copy-trading", accessed: "2026-10-23" }
  - { field: "copy_trading_allowed", url: "https://help.blusky.pro/en/articles/12069208-ethical-trading-conduct-policy-blusky-trading-company", accessed: "2026-10-23" }
```

#### Orbit — funded — 100K

```yaml
firm: "blusky"
plan: "Orbit — Sim Funded"
phase: "funded"
size_label: "100K"
size_usd: "100000.00"
instruments: ["MNQ", "NQ"]
platforms: ["NinjaTrader 8 (via Tradovate connection; via Rithmic connection, phase scope conflicts)", "Tradovate", "TradingView (Tradovate list)", "Rithmic platforms (R|Trader Pro, Quantower, ATAS, Sierra Chart, Bookmap, MotiveWave, Jigsaw, ONYX, others)", "Volumetrica web platform (VolSys/Deepchart)", "Tradesea", "Tickblaze", "BlackArrow"]
drawdown: { amount_usd: "3000.00", type: "trailing_eod", stops_trailing_at: "Locks permanently at the starting balance once lifetime net profit equals the starting drawdown ($3000 net profit); the floor trails closed daily profit dollar-for-dollar" }
daily_loss_limit: { amount_usd: null, kind: "none" }
profit_target_usd: "none"
consistency_rule: { text: "40% consistency rule applies while Sim Funded; largest single profitable day vs the sum of profitable days; resets after each processed payout.", threshold_pct: "40" }
max_position: { minis: 3, micros: 30, scaling: "Generic Sim Funded pages state limits can be scaled by margin (Tradovate) or by support request; Rithmic and Volumetrica do not scale." }
min_trading_days: null
trading_day_reset: { time: "18:00", tz_as_published: "6PM, tz not stated on that page; other pages say EST/ET" }
commission_per_contract: { mnq_usd: null, nq_usd: null, basis: null, includes: "Not published for this phase: funded commission is 'Set by brokerage' (Rithmic/Volumetrica/Tradesea); Buffer not listed for Rithmic/Volumetrica/Tradesea; Tradovate table has no stated basis (section 8)." }
copy_trading_allowed: "conflict"
automated_trading_allowed: "unknown"
automation_rule_text: "Coordinated Manipulation: Engaging, individually or with others (including across multiple accounts), in trading patterns whose primary intent is to manipulate, abuse, or secure an unfair advantage on our platform. This includes violating our terms and conditions, or using automated software (AI, ultra-high speed, bulk data entry) to unfairly influence outcomes."
vps_allowed: "unknown"
personal_device_required: "unknown"
status: "partial"
not_confirmed_fields: ["copy_trading_allowed", "automated_trading_allowed", "vps_allowed", "personal_device_required", "trading_day_reset", "commission_per_contract"]
notes: "Sim Funded opens at the full account size. Payouts every three trading days once balance exceeds the threshold; payout minimum $500; rolling 7-day cap $2500. Sim Funded Accounts Processing describes Orbit as a $0 starting balance with a negative floor (framing difference, same drawdown for 50K/100K/200K; it omits 150K). Orbit Brokerage review ceiling per account: 50K $10,000, 100K $12,500, 150K $15,000, 200K $12,500."
sources:
  - { field: "drawdown", url: "https://help.blusky.pro/en/articles/16300866-orbit-plans-two-phases-faster-payouts", accessed: "2026-10-23" }
  - { field: "consistency_rule", url: "https://help.blusky.pro/en/articles/16300866-orbit-plans-two-phases-faster-payouts", accessed: "2026-10-23" }
  - { field: "max_position", url: "https://help.blusky.pro/en/articles/16300866-orbit-plans-two-phases-faster-payouts", accessed: "2026-10-23" }
  - { field: "payouts", url: "https://help.blusky.pro/en/articles/15756814-orbit-v2-accounts-how-do-payouts-work", accessed: "2026-10-23" }
```

#### Orbit — evaluation — 150K

```yaml
firm: "blusky"
plan: "Orbit"
phase: "evaluation"
size_label: "150K"
size_usd: "150000.00"
instruments: ["MNQ", "NQ"]
platforms: ["NinjaTrader 8 (via Tradovate connection; via Rithmic connection, phase scope conflicts)", "Tradovate", "TradingView (Tradovate list)", "Rithmic platforms (R|Trader Pro, Quantower, ATAS, Sierra Chart, Bookmap, MotiveWave, Jigsaw, ONYX, others)", "Volumetrica web platform (VolSys/Deepchart)", "Tradesea", "Tickblaze", "BlackArrow"]
drawdown: { amount_usd: "4500.00", type: "trailing_eod", stops_trailing_at: null }
daily_loss_limit: { amount_usd: null, kind: "none" }
profit_target_usd: "9000.00"
consistency_rule: { text: "No consistency rule in the Orbit evaluation.", threshold_pct: null }
max_position: { minis: 15, micros: 150, scaling: null }
min_trading_days: null
trading_day_reset: { time: "18:00", tz_as_published: "6PM, tz not stated on that page; other pages say EST/ET" }
commission_per_contract: { mnq_usd: "0.50", nq_usd: "2.00", basis: "per_side", includes: "Applies to Rithmic, Volumetrica and Tradesea evaluation accounts only. Page does not say what the figure includes. Tradovate/NinjaTrader: table with no basis stated (see section 8)." }
copy_trading_allowed: "conflict"
automated_trading_allowed: "unknown"
automation_rule_text: "Coordinated Manipulation: Engaging, individually or with others (including across multiple accounts), in trading patterns whose primary intent is to manipulate, abuse, or secure an unfair advantage on our platform. This includes violating our terms and conditions, or using automated software (AI, ultra-high speed, bulk data entry) to unfairly influence outcomes."
vps_allowed: "unknown"
personal_device_required: "unknown"
status: "partial"
not_confirmed_fields: ["copy_trading_allowed", "automated_trading_allowed", "vps_allowed", "personal_device_required", "trading_day_reset", "commission_per_contract", "drawdown.stops_trailing_at"]
notes: "One-time purchase (not a subscription); evaluation valid 30 days from purchase, a reset does not extend it. Can be passed in as little as a single day (firm's words). Evaluation reset: 50K $129, 100K $199, 150K $279, 200K $279. Only two phases (no Buffer Zone) per the Orbit page; the Evaluation Overview still lists an Orbit Buffer phase with 50% consistency (conflict listed in section 10)."
sources:
  - { field: "drawdown", url: "https://help.blusky.pro/en/articles/16300866-orbit-plans-two-phases-faster-payouts", accessed: "2026-10-23" }
  - { field: "profit_target_usd", url: "https://help.blusky.pro/en/articles/16300866-orbit-plans-two-phases-faster-payouts", accessed: "2026-10-23" }
  - { field: "consistency_rule", url: "https://help.blusky.pro/en/articles/16300866-orbit-plans-two-phases-faster-payouts", accessed: "2026-10-23" }
  - { field: "max_position", url: "https://help.blusky.pro/en/articles/16300866-orbit-plans-two-phases-faster-payouts", accessed: "2026-10-23" }
  - { field: "daily_loss_limit", url: "https://help.blusky.pro/en/articles/16300866-orbit-plans-two-phases-faster-payouts", accessed: "2026-10-23" }
  - { field: "trading_day_reset", url: "https://help.blusky.pro/en/articles/12434059-evaluation-rules", accessed: "2026-10-23" }
  - { field: "commission_per_contract", url: "https://help.blusky.pro/en/articles/17115915-markets-symbols-commissions", accessed: "2026-10-23" }
  - { field: "platforms", url: "https://help.blusky.pro/en/articles/12433926-blusky-supported-platforms", accessed: "2026-10-23" }
  - { field: "copy_trading_allowed", url: "https://help.blusky.pro/en/articles/12433943-multiple-accounts-copy-trading", accessed: "2026-10-23" }
  - { field: "copy_trading_allowed", url: "https://help.blusky.pro/en/articles/12069208-ethical-trading-conduct-policy-blusky-trading-company", accessed: "2026-10-23" }
```

#### Orbit — funded — 150K

```yaml
firm: "blusky"
plan: "Orbit — Sim Funded"
phase: "funded"
size_label: "150K"
size_usd: "150000.00"
instruments: ["MNQ", "NQ"]
platforms: ["NinjaTrader 8 (via Tradovate connection; via Rithmic connection, phase scope conflicts)", "Tradovate", "TradingView (Tradovate list)", "Rithmic platforms (R|Trader Pro, Quantower, ATAS, Sierra Chart, Bookmap, MotiveWave, Jigsaw, ONYX, others)", "Volumetrica web platform (VolSys/Deepchart)", "Tradesea", "Tickblaze", "BlackArrow"]
drawdown: { amount_usd: "4500.00", type: "trailing_eod", stops_trailing_at: "Locks permanently at the starting balance once lifetime net profit equals the starting drawdown ($4500 net profit); the floor trails closed daily profit dollar-for-dollar" }
daily_loss_limit: { amount_usd: null, kind: "none" }
profit_target_usd: "none"
consistency_rule: { text: "40% consistency rule applies while Sim Funded; largest single profitable day vs the sum of profitable days; resets after each processed payout.", threshold_pct: "40" }
max_position: { minis: 5, micros: 50, scaling: "Generic Sim Funded pages state limits can be scaled by margin (Tradovate) or by support request; Rithmic and Volumetrica do not scale." }
min_trading_days: null
trading_day_reset: { time: "18:00", tz_as_published: "6PM, tz not stated on that page; other pages say EST/ET" }
commission_per_contract: { mnq_usd: null, nq_usd: null, basis: null, includes: "Not published for this phase: funded commission is 'Set by brokerage' (Rithmic/Volumetrica/Tradesea); Buffer not listed for Rithmic/Volumetrica/Tradesea; Tradovate table has no stated basis (section 8)." }
copy_trading_allowed: "conflict"
automated_trading_allowed: "unknown"
automation_rule_text: "Coordinated Manipulation: Engaging, individually or with others (including across multiple accounts), in trading patterns whose primary intent is to manipulate, abuse, or secure an unfair advantage on our platform. This includes violating our terms and conditions, or using automated software (AI, ultra-high speed, bulk data entry) to unfairly influence outcomes."
vps_allowed: "unknown"
personal_device_required: "unknown"
status: "partial"
not_confirmed_fields: ["copy_trading_allowed", "automated_trading_allowed", "vps_allowed", "personal_device_required", "trading_day_reset", "commission_per_contract"]
notes: "Sim Funded opens at the full account size. Payouts every three trading days once balance exceeds the threshold; payout minimum $500; rolling 7-day cap $3000. Sim Funded Accounts Processing describes Orbit as a $0 starting balance with a negative floor (framing difference, same drawdown for 50K/100K/200K; it omits 150K). Orbit Brokerage review ceiling per account: 50K $10,000, 100K $12,500, 150K $15,000, 200K $12,500."
sources:
  - { field: "drawdown", url: "https://help.blusky.pro/en/articles/16300866-orbit-plans-two-phases-faster-payouts", accessed: "2026-10-23" }
  - { field: "consistency_rule", url: "https://help.blusky.pro/en/articles/16300866-orbit-plans-two-phases-faster-payouts", accessed: "2026-10-23" }
  - { field: "max_position", url: "https://help.blusky.pro/en/articles/16300866-orbit-plans-two-phases-faster-payouts", accessed: "2026-10-23" }
  - { field: "payouts", url: "https://help.blusky.pro/en/articles/15756814-orbit-v2-accounts-how-do-payouts-work", accessed: "2026-10-23" }
```

#### Orbit — evaluation — 200K

```yaml
firm: "blusky"
plan: "Orbit"
phase: "evaluation"
size_label: "200K"
size_usd: "200000.00"
instruments: ["MNQ", "NQ"]
platforms: ["NinjaTrader 8 (via Tradovate connection; via Rithmic connection, phase scope conflicts)", "Tradovate", "TradingView (Tradovate list)", "Rithmic platforms (R|Trader Pro, Quantower, ATAS, Sierra Chart, Bookmap, MotiveWave, Jigsaw, ONYX, others)", "Volumetrica web platform (VolSys/Deepchart)", "Tradesea", "Tickblaze", "BlackArrow"]
drawdown: { amount_usd: "3000.00", type: "static", stops_trailing_at: "static: never moves" }
daily_loss_limit: { amount_usd: null, kind: "none" }
profit_target_usd: "8000.00"
consistency_rule: { text: "No consistency rule in the Orbit evaluation.", threshold_pct: null }
max_position: { minis: 3, micros: 30, scaling: null }
min_trading_days: null
trading_day_reset: { time: "18:00", tz_as_published: "6PM, tz not stated on that page; other pages say EST/ET" }
commission_per_contract: { mnq_usd: "0.50", nq_usd: "2.00", basis: "per_side", includes: "Applies to Rithmic, Volumetrica and Tradesea evaluation accounts only. Page does not say what the figure includes. Tradovate/NinjaTrader: table with no basis stated (see section 8)." }
copy_trading_allowed: "conflict"
automated_trading_allowed: "unknown"
automation_rule_text: "Coordinated Manipulation: Engaging, individually or with others (including across multiple accounts), in trading patterns whose primary intent is to manipulate, abuse, or secure an unfair advantage on our platform. This includes violating our terms and conditions, or using automated software (AI, ultra-high speed, bulk data entry) to unfairly influence outcomes."
vps_allowed: "unknown"
personal_device_required: "unknown"
status: "conflict"
not_confirmed_fields: ["copy_trading_allowed", "automated_trading_allowed", "vps_allowed", "personal_device_required", "trading_day_reset", "commission_per_contract", "max_position"]
notes: "One-time purchase (not a subscription); evaluation valid 30 days from purchase, a reset does not extend it. Can be passed in as little as a single day (firm's words). Evaluation reset: 50K $129, 100K $199, 150K $279, 200K $279. Only two phases (no Buffer Zone) per the Orbit page; the Evaluation Overview still lists an Orbit Buffer phase with 50% consistency (conflict listed in section 10). CONFLICT: Evaluation Overview lists Orbit 200K as '2 Mini & 20 Micro's'; the Orbit page lists 3 Mini / 30 Micros for the evaluation."
sources:
  - { field: "drawdown", url: "https://help.blusky.pro/en/articles/16300866-orbit-plans-two-phases-faster-payouts", accessed: "2026-10-23" }
  - { field: "profit_target_usd", url: "https://help.blusky.pro/en/articles/16300866-orbit-plans-two-phases-faster-payouts", accessed: "2026-10-23" }
  - { field: "consistency_rule", url: "https://help.blusky.pro/en/articles/16300866-orbit-plans-two-phases-faster-payouts", accessed: "2026-10-23" }
  - { field: "max_position", url: "https://help.blusky.pro/en/articles/16300866-orbit-plans-two-phases-faster-payouts", accessed: "2026-10-23" }
  - { field: "daily_loss_limit", url: "https://help.blusky.pro/en/articles/16300866-orbit-plans-two-phases-faster-payouts", accessed: "2026-10-23" }
  - { field: "trading_day_reset", url: "https://help.blusky.pro/en/articles/12434059-evaluation-rules", accessed: "2026-10-23" }
  - { field: "commission_per_contract", url: "https://help.blusky.pro/en/articles/17115915-markets-symbols-commissions", accessed: "2026-10-23" }
  - { field: "platforms", url: "https://help.blusky.pro/en/articles/12433926-blusky-supported-platforms", accessed: "2026-10-23" }
  - { field: "copy_trading_allowed", url: "https://help.blusky.pro/en/articles/12433943-multiple-accounts-copy-trading", accessed: "2026-10-23" }
  - { field: "copy_trading_allowed", url: "https://help.blusky.pro/en/articles/12069208-ethical-trading-conduct-policy-blusky-trading-company", accessed: "2026-10-23" }
  - { field: "max_position", url: "https://help.blusky.pro/en/articles/14184170-evaluation-overview", accessed: "2026-10-23" }
```

#### Orbit — funded — 200K

```yaml
firm: "blusky"
plan: "Orbit — Sim Funded"
phase: "funded"
size_label: "200K"
size_usd: "200000.00"
instruments: ["MNQ", "NQ"]
platforms: ["NinjaTrader 8 (via Tradovate connection; via Rithmic connection, phase scope conflicts)", "Tradovate", "TradingView (Tradovate list)", "Rithmic platforms (R|Trader Pro, Quantower, ATAS, Sierra Chart, Bookmap, MotiveWave, Jigsaw, ONYX, others)", "Volumetrica web platform (VolSys/Deepchart)", "Tradesea", "Tickblaze", "BlackArrow"]
drawdown: { amount_usd: "3000.00", type: "trailing_eod", stops_trailing_at: "Locks permanently at the starting balance once lifetime net profit equals the starting drawdown ($3000 net profit); the floor trails closed daily profit dollar-for-dollar" }
daily_loss_limit: { amount_usd: null, kind: "none" }
profit_target_usd: "none"
consistency_rule: { text: "40% consistency rule applies while Sim Funded; largest single profitable day vs the sum of profitable days; resets after each processed payout.", threshold_pct: "40" }
max_position: { minis: 3, micros: 30, scaling: "Generic Sim Funded pages state limits can be scaled by margin (Tradovate) or by support request; Rithmic and Volumetrica do not scale." }
min_trading_days: null
trading_day_reset: { time: "18:00", tz_as_published: "6PM, tz not stated on that page; other pages say EST/ET" }
commission_per_contract: { mnq_usd: null, nq_usd: null, basis: null, includes: "Not published for this phase: funded commission is 'Set by brokerage' (Rithmic/Volumetrica/Tradesea); Buffer not listed for Rithmic/Volumetrica/Tradesea; Tradovate table has no stated basis (section 8)." }
copy_trading_allowed: "conflict"
automated_trading_allowed: "unknown"
automation_rule_text: "Coordinated Manipulation: Engaging, individually or with others (including across multiple accounts), in trading patterns whose primary intent is to manipulate, abuse, or secure an unfair advantage on our platform. This includes violating our terms and conditions, or using automated software (AI, ultra-high speed, bulk data entry) to unfairly influence outcomes."
vps_allowed: "unknown"
personal_device_required: "unknown"
status: "partial"
not_confirmed_fields: ["copy_trading_allowed", "automated_trading_allowed", "vps_allowed", "personal_device_required", "trading_day_reset", "commission_per_contract"]
notes: "Sim Funded opens at the full account size. Payouts every three trading days once balance exceeds the threshold; payout minimum $500; rolling 7-day cap $2500. Sim Funded Accounts Processing describes Orbit as a $0 starting balance with a negative floor (framing difference, same drawdown for 50K/100K/200K; it omits 150K). Orbit Brokerage review ceiling per account: 50K $10,000, 100K $12,500, 150K $15,000, 200K $12,500."
sources:
  - { field: "drawdown", url: "https://help.blusky.pro/en/articles/16300866-orbit-plans-two-phases-faster-payouts", accessed: "2026-10-23" }
  - { field: "consistency_rule", url: "https://help.blusky.pro/en/articles/16300866-orbit-plans-two-phases-faster-payouts", accessed: "2026-10-23" }
  - { field: "max_position", url: "https://help.blusky.pro/en/articles/16300866-orbit-plans-two-phases-faster-payouts", accessed: "2026-10-23" }
  - { field: "payouts", url: "https://help.blusky.pro/en/articles/15756814-orbit-v2-accounts-how-do-payouts-work", accessed: "2026-10-23" }
```

#### Instant Sim Funded — funded — 50K

```yaml
firm: "blusky"
plan: "Instant Sim Funded"
phase: "funded"
size_label: "50K"
size_usd: "50000.00"
instruments: ["MNQ", "NQ"]
platforms: ["NinjaTrader 8 (via Tradovate connection; via Rithmic connection, phase scope conflicts)", "Tradovate", "TradingView (Tradovate list)", "Rithmic platforms (R|Trader Pro, Quantower, ATAS, Sierra Chart, Bookmap, MotiveWave, Jigsaw, ONYX, others)", "Volumetrica web platform (VolSys/Deepchart)", "Tradesea", "Tickblaze", "BlackArrow"]
drawdown: { amount_usd: "2000.00", type: "trailing_intraday", stops_trailing_at: "The trailing stops at $51,000 once your balance passes $53,000" }
daily_loss_limit: { amount_usd: "1000.00", kind: "unknown" }
profit_target_usd: "none"
consistency_rule: { text: "21% consistency (largest single profitable day vs sum of profitable days).", threshold_pct: "21" }
max_position: { minis: 5, micros: 50, scaling: null }
min_trading_days: null
trading_day_reset: { time: "18:00", tz_as_published: "6PM, tz not stated on that page; other pages say EST/ET" }
commission_per_contract: { mnq_usd: null, nq_usd: null, basis: null, includes: "Not published for this phase: funded commission is 'Set by brokerage' (Rithmic/Volumetrica/Tradesea); Buffer not listed for Rithmic/Volumetrica/Tradesea; Tradovate table has no stated basis (section 8)." }
copy_trading_allowed: "conflict"
automated_trading_allowed: "unknown"
automation_rule_text: "Coordinated Manipulation: Engaging, individually or with others (including across multiple accounts), in trading patterns whose primary intent is to manipulate, abuse, or secure an unfair advantage on our platform. This includes violating our terms and conditions, or using automated software (AI, ultra-high speed, bulk data entry) to unfairly influence outcomes."
vps_allowed: "unknown"
personal_device_required: "unknown"
status: "partial"
not_confirmed_fields: ["copy_trading_allowed", "automated_trading_allowed", "vps_allowed", "personal_device_required", "trading_day_reset", "commission_per_contract", "daily_loss_limit.kind"]
notes: "One-time purchase, no evaluation. Account closes below $48,000. Payouts above $53,000, up to $2,500 per 7-day window; 80% profit split; up to three accounts; resets not available. Daily loss limit $1,000 is published but the page does not say whether breaching it closes the account or pauses it."
sources:
  - { field: "drawdown", url: "https://help.blusky.pro/en/articles/16300869-instant-funding-direct-to-funded-and-instant-sim-funded", accessed: "2026-10-23" }
  - { field: "daily_loss_limit", url: "https://help.blusky.pro/en/articles/16300869-instant-funding-direct-to-funded-and-instant-sim-funded", accessed: "2026-10-23" }
  - { field: "consistency_rule", url: "https://help.blusky.pro/en/articles/16300869-instant-funding-direct-to-funded-and-instant-sim-funded", accessed: "2026-10-23" }
  - { field: "max_position", url: "https://help.blusky.pro/en/articles/16300869-instant-funding-direct-to-funded-and-instant-sim-funded", accessed: "2026-10-23" }
```

#### Direct to Funded — funded — $3,500 account

```yaml
firm: "blusky"
plan: "Direct to Funded"
phase: "funded"
size_label: "3.5K"
size_usd: "3500.00"
instruments: ["MNQ", "NQ"]
platforms: ["NinjaTrader 8 (via Tradovate connection; via Rithmic connection, phase scope conflicts)", "Tradovate", "TradingView (Tradovate list)", "Rithmic platforms (R|Trader Pro, Quantower, ATAS, Sierra Chart, Bookmap, MotiveWave, Jigsaw, ONYX, others)", "Volumetrica web platform (VolSys/Deepchart)", "Tradesea", "Tickblaze", "BlackArrow"]
drawdown: { amount_usd: "2500.00", type: "static", stops_trailing_at: "static: never moves; account closes below $1,000" }
daily_loss_limit: { amount_usd: "2000.00", kind: "unknown" }
profit_target_usd: "none"
consistency_rule: { text: "21% consistency; plus a negative day rule: at most 3 negative days per rolling 10 trading days.", threshold_pct: "21" }
max_position: { minis: 5, micros: 50, scaling: null }
min_trading_days: null
trading_day_reset: { time: "18:00", tz_as_published: "6PM, tz not stated on that page; other pages say EST/ET" }
commission_per_contract: { mnq_usd: null, nq_usd: null, basis: null, includes: "Not published for this phase: funded commission is 'Set by brokerage' (Rithmic/Volumetrica/Tradesea); Buffer not listed for Rithmic/Volumetrica/Tradesea; Tradovate table has no stated basis (section 8)." }
copy_trading_allowed: "conflict"
automated_trading_allowed: "unknown"
automation_rule_text: "Coordinated Manipulation: Engaging, individually or with others (including across multiple accounts), in trading patterns whose primary intent is to manipulate, abuse, or secure an unfair advantage on our platform. This includes violating our terms and conditions, or using automated software (AI, ultra-high speed, bulk data entry) to unfairly influence outcomes."
vps_allowed: "unknown"
personal_device_required: "unknown"
status: "partial"
not_confirmed_fields: ["copy_trading_allowed", "automated_trading_allowed", "vps_allowed", "personal_device_required", "trading_day_reset", "commission_per_contract", "daily_loss_limit.kind"]
notes: "Outside the 25K-300K range (low priority). One-time purchase; payouts above $3,500, weekly cap $3,000; 50% split for 30 trading days then 80%; resets not available. Daily loss limit $2,000 published without saying whether it closes or pauses the account."
sources:
  - { field: "drawdown", url: "https://help.blusky.pro/en/articles/16300869-instant-funding-direct-to-funded-and-instant-sim-funded", accessed: "2026-10-23" }
  - { field: "daily_loss_limit", url: "https://help.blusky.pro/en/articles/16300869-instant-funding-direct-to-funded-and-instant-sim-funded", accessed: "2026-10-23" }
  - { field: "consistency_rule", url: "https://help.blusky.pro/en/articles/16300869-instant-funding-direct-to-funded-and-instant-sim-funded", accessed: "2026-10-23" }
  - { field: "max_position", url: "https://help.blusky.pro/en/articles/16300869-instant-funding-direct-to-funded-and-instant-sim-funded", accessed: "2026-10-23" }
```

## 10. Unverified / gaps / conflicts

| Field | Issue | Detail and sources |
|---|---|---|
| copy_trading_allowed | conflict | Allowed ("Trade Copier Allowed: Yes", S10) vs "Account Mirroring: Copying trades from another trader, account, or signal provider" forbidden (S11). Not reconciled. |
| NinjaTrader on Rithmic, phase scope | conflict | "NinjaTrader (Evaluation, BluLive and Sim Funded only)" (S12) vs "NinjaTrader (Evaluation Only)" (S13). |
| Propel 150K / 200K consistency | conflict | 30% (S5 Static Growth Plan) vs 34% (S2, S4, S18). |
| Propel 300K evaluation daily loss limit | conflict | "Daily Loss Limit: Capped at $2,500" (S5) vs "None" (S2). |
| Orbit 200K evaluation contracts | conflict | "2 Mini & 20 Micro's" (S18) vs 3 Mini / 30 Micros (S3). |
| Orbit phases | conflict | S18 lists an Orbit Buffer phase "50% Consistency"; S3 says Orbit has two phases and no Buffer. Stale S18 suspected, not confirmed. |
| Sim Funded contract limits | conflict | S6 table (25K 2/20, 50K 5/50, 100K 5/50, 150K 2/20, 200K 4/40, 300K 5/50) vs S7 ("Micros: 15-30, E-minis: 1-3" and a balance table 3,500 to 3/35, 3,000 to 3/30, 2,000 to 2/20, 1,500 to 1/15) vs S22 ("Most Sim Funded Accounts start with 5 Mini's and 50 Micro's"). S7 says the limits displayed on the account govern. Recorded null for Launch/Propel. |
| Propel Sim Funded starting balance 150K / 300K | conflict | $2,000 / $3,500 (S2, S6) vs $1,500 / $3,000 (S8). |
| Sim Funded minimum balance | conflict | $100 above/below model (S1, S2, S8) vs old payout-page table (25K $26,500, 50K $53,000, 100K $103,500, S25). S25 may be stale. |
| Consistency method wording | conflict | Largest day vs sum of profitable days (S1, S2, S3) vs "Highest profit day becomes the % of your new target" (S4) vs "profit target will automatically increase" (S5). |
| Orbit Sim Funded start | conflict (framing) | $50,000 start with drawdown lock (S3) vs $0 start with negative floor (S8; omits 150K). |
| automated_trading_allowed, automation_rule_text | unknown | No official allow/forbid text for bots, EAs, algorithms, webhooks; only S11 text on "automated software". |
| vps_allowed, personal_device_required | unknown | No VPS/VPN/remote-server/personal-device text anywhere; only the household "different machines" rule (S10). |
| Copying with other firms | unknown | Not mentioned. |
| trading_day_reset | unverified | Only "6PM" (S4) and "6PM EST" (S19); no session open/close table; no holiday rule. |
| Commission, Tradovate/NinjaTrader | unverified | Table without basis (per side/round turn) or tier (S17). |
| Commission, funded; Buffer; Tickblaze; BlackArrow | unverified | "Set by brokerage" or not listed (S17). |
| Plan price, subscription price | unverified | Dynamic plans page; not captured. List prices shown on how-blusky-compares (Launch $59, Propel $160 list, Orbit $199) are marketing-page figures, not recorded. |
| Plan x platform availability | unverified | Selector and checkout are dynamic or behind login. |
| Evaluation trailing lock point | unverified | EOD trailing described (S4) without a lock; S6 mentions trailing that "becomes static once you reach the balance indicated in the table below" (table shows Sim Funded start balances; unclear). |
| Buffer Zone contract limits | unverified | Not stated on S1/S2. |
| Daily loss limit kind, Instant / Direct | unverified | Amount published, hard vs soft not stated (S9). |
| Sim Funded daily loss limit | unverified | None stated; Risk Team may impose (S7, S26); Volumetrica allows trader-set. |
| Live Brokerage parameters, NT8 on live | unverified | Not published per size (S23). |
| Project X as a platform | unverified | Mentioned in S22 only. |

## 11. Changelog

- 2026-10-23 — initial research (Lupa)
- 2026-10-23 — rev bump R4: device/VPS re-checked on all help-center and main-site pages; no official text found, `vps_allowed` and `personal_device_required` stay unknown (Lupa)
