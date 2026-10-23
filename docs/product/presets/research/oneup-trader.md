# OneUp Trader — Prop Firm Rules Research

| | |
|---|---|
| **Firm** | OneUp Trader (futures only). Describes itself as a recruitment company: "OneUp Trader is not a Funding company but a Recruitment company for traders who would like to get funded. We are the simplified gateway for traders to get funded. Once you complete the Evaluation, you will receive a funded trading account with one of our Funding Partners." (S9) |
| **Official domains** | `www.oneuptrader.com` (main site, plain curl works) · `help.oneuptrader.com` (official help center, HelpScout Docs, server-rendered, plain curl works; `oneuptrader.helpscoutdocs.com` is the same help center, linked from the site footer) · `app.oneuptrader.com` (login/signup, not used) · `blog.oneuptrader.com` (not used) |
| **Research date** | 2026-10-23 (all URLs below accessed 2026-10-23) |
| **Researcher** | Lupa (Prop Firm Rules Researcher) |
| **File status** | `partial`. Verified: account sizes, profit targets, trailing drawdown amounts and mechanism, no daily loss limit, consistency rule, trading hours, commissions, copy-trading wording for funded accounts, platform list. NOT published anywhere found: any rule on automated trading, bots, webhooks, VPS/VPN/remote servers or personal-device requirement (see §10). Funded scaling table and funded commission table are images (read visually, not text-quotable) |
| **Instruments covered** | MNQ and NQ are both permitted (S1: "Trade permitted products: ... MNQ ... NQ ..."; S7 lists "E-mini NASDAQ 100 (NQ)" and "Micro E-Mini Nasdaq-100 (MNQ)") |
| **Platforms (for routing)** | Rithmic-based. NinjaTrader Desktop 8.1 is the "Preferred Platform" (connection "Rithmic for NinjaTrader Brokerage"), plus R\|Trader / R\|Trader Pro and about 20 other Rithmic-compatible platforms. NinjaTrader Web and Mobile are NOT usable with Rithmic-based evaluation accounts. See §2 |

Evidence (raw text of every cited page, URL on line 1; images in `img/`): `/tmp/claude-1003/-srv-www-allwin--maestri-roles-6bf77a80-a1ed-43dc-bc00-08a480f178a5/c3ed71e7-3dd4-4f1a-8a03-9bc3a3fa7b49/scratchpad/r3/oneup-trader/` (`txt/`, `home.txt`, `terms.txt`, `img/`). Temporary scratch storage: capture again at re-verification.

**Programs found.** One path: Evaluation (1-step) → Funded Trader Account with a "funding partner" (activation fee after passing). Account sizes **$25K, $50K, $100K, $150K, $250K** (S27: "You can get anywhere from $25,000 to $250,000"). There is no 300K size. Two evaluation variants with identical size/target/drawdown: **Evaluation** (minimum 10 trading days) and **Pro Evaluation** (minimum 5 trading days, higher monthly price). **Express Funding** (only for traders whose funded account was terminated in the last 24 hours; same size as the lost account; minimum 3 trading days). A 7-day free test is advertised on the site cards ("...or test FREE for 7 days!"). The funded phase is a simulated/funding-partner account under the firm's rules; no separate "live" tier is published.

**Firm terms.** "Trailing Drawdown" = the drawdown. "Lots" / "Max Contracts" = maximum open positions across all products at any time. "Cash On Hand" = account balance seen in R|Trader Pro, used for funded scaling. "Funding partners" set funded rules ("Traders are expressly prohibited ... by our funding partners").

## 1. Source index (all accessed 2026-10-23)

| Key | URL |
|---|---|
| H | https://www.oneuptrader.com/ (plan cards: size, max contracts, profit target, trailing drawdown, min days, price) |
| T | https://www.oneuptrader.com/terms (Terms & Conditions, generic website terms) |
| S1 | https://help.oneuptrader.com/article/385-evaluation-rules-parameters |
| S2 | https://help.oneuptrader.com/article/387-what-are-the-rules-of-the-evaluation |
| S3 | https://help.oneuptrader.com/article/389-rules-definitions |
| S4 | https://help.oneuptrader.com/article/388-funded-trader-trailing-drawdown |
| S5 | https://help.oneuptrader.com/article/372-funded-trader-rules |
| S6 | https://help.oneuptrader.com/article/386-funded-trader-dynamic-scaling-targets (table is an image) |
| S7 | https://help.oneuptrader.com/article/390-permissible-products-times |
| S8 | https://help.oneuptrader.com/article/367-economic-release-calendar |
| S9 | https://help.oneuptrader.com/article/468-most-popular-questions |
| S10 | https://help.oneuptrader.com/article/354-are-commissions-factored-into-my-evaluation |
| S11 | https://help.oneuptrader.com/article/391-important-evaluation-info |
| S12 | https://help.oneuptrader.com/article/361-funded-trader-commissions (table is an image) |
| S13 | https://help.oneuptrader.com/article/505-can-i-have-multiple-evaluation-accounts |
| S14 | https://help.oneuptrader.com/article/380-supported-platforms |
| S15 | https://help.oneuptrader.com/article/362-what-trading-platforms-can-i-use |
| S16 | https://help.oneuptrader.com/article/491-ninjatrader-desktop-version-v-8-1-connection-guide |
| S17 | https://help.oneuptrader.com/article/511-steps-to-set-up-a-rithmic-based-connection-with-ninja-trader |
| S18 | https://help.oneuptrader.com/article/392-must-start-here (R\|Trader Pro connection guide) |
| S19 | https://help.oneuptrader.com/article/373-account-fees |
| S20 | https://help.oneuptrader.com/article/371-how-does-the-billing-work |
| S21 | https://help.oneuptrader.com/article/376-profit-info |
| S22 | https://help.oneuptrader.com/article/488-express-funding-how-does-it-work |
| S23 | https://help.oneuptrader.com/article/381-can-i-reset-my-balance |
| S24 | https://help.oneuptrader.com/article/486-no-more-daily-loss-limit-restrictions (release note, 2023-02-09) |
| S25 | https://help.oneuptrader.com/article/353-can-i-trade-during-economic-releases |
| S26 | https://help.oneuptrader.com/article/490-get-funded-in-just-5-days-with-an-express-funding-account (release note, 2023-03-09) |
| S27 | https://help.oneuptrader.com/article/379-what-are-the-account-options |
| S28 | https://help.oneuptrader.com/article/475-reduced-fees-on-micros |
| S29 | https://help.oneuptrader.com/article/520-getting-funded-process |

The help center sitemap (`https://help.oneuptrader.com/sitemap.xml`, 73 URLs, 60 articles) was crawled in full and every article saved. The main site sitemap lists only one page-sitemap; no rules page other than the homepage and `/terms` exists on `www`.

## 2. Platforms

Per plan/phase (the firm does not differentiate the list by plan):

- **List (S14, current):** "Ninjatrader - Preferred Platform / Free During Evaluation & Free on the NinjaTrader Desktop Version V.8.1 once Funded." R\|Trader, R\|Trader Pro, Agena Trader, Inside Edge Trader, Investor RT, Motive Wave, MultiCharts, Photon, QScalp, QST (Quick Screen Trading), Track'n Trade, Trade Navigator, Volfix.net, eSignal, Jigsaw Trading, ATAS Order Flow Trading, Bookmap, Quantower, Sierra Chart, Medved Trader. The older article S15 lists "ScalpTool" and "R Investor" and omits Medved Trader (minor difference, see §10).
- **Connection:** everything runs on Rithmic. Quote (S9): "You may use any platform that supports Rithmic connection." R\|Trader Pro must be installed and the CME market data agreements signed in it before any other platform works (S18: "You must first download Rithmic R | TRADER Pro in conjunction with any other platform you may use.").
- **NinjaTrader 8.1 (S16):** add "Rithmic for NinjaTrader Brokerage" in Control Center > Connections > Configure, Multi-provider option enabled, system "Rithmic Paper Trading (Chicago)", optional "Plug-in mode for market data" so R\|Trader Pro and NinjaTrader run together (recommended to watch the trailing drawdown). Quote: "Rithmic-based Evaluation accounts cannot connect on the NinjaTrader Web and Mobile apps and can only be used on the NinjaTrader Desktop app." S17 says NinjaTrader needs "Evaluation (Rithmic)" entitlement; R\|Trader Pro must be open and connected before NinjaTrader can connect when Plug-in Mode is used. S16: "License Keys are no longer needed ... (version 8.1 or later)".
- **Platform cost:** S9: "You can use R|Trader Pro ... entirely for FREE on both the Evaluation and Funded Accounts. We also offer a free NinjaTrader license key which can be used during the Evaluation period. This free license key can also be used on the NinjaTrader Desktop Version V.8.1 once Funded." S19: as a funded trader, platform fees are the trader's responsibility except those free items.
- **Funded-phase connection details:** the help center has no separate funded NinjaTrader connection guide; the funded account credentials come from the funding partner after approval (S29). Whether the system name stays "Rithmic Paper Trading" is not stated (gap).

## 3. Plans

All three plan families share the same size table. Values from the plan cards on the homepage (H), cross-checked with help articles where they publish the same fact. Evaluation money values are prices at the cards' "Evaluation" tab; "Pro Evaluation" tab differs only in min days and price.

### 3.1 Evaluation and Pro Evaluation (phase: evaluation)

| Size | Max contracts (H) | Profit target (H) | Trailing drawdown (H) | Daily loss limit | Min days (Eval / Pro Eval) | Monthly price Eval / Pro Eval (H) | Balance reset (S23) |
|---|---|---|---|---|---|---|---|
| 25K | 3 | 1500.00 | 1500.00 | none | 10 / 5 | $65 / $100 | $50 |
| 50K | 6 | 3000.00 | 2500.00 | none | 10 / 5 | $75 / $115 | $50 |
| 100K | 12 | 6000.00 | 3500.00 | none | 10 / 5 | $150 / $225 | $100 |
| 150K | 15 | 9000.00 | 5000.00 | none | 10 / 5 | $175 / $265 | $100 |
| 250K | 25 | 15000.00 | 5500.00 | none | 10 / 5 | $325 / $490 | $100 |

- **Profit target (preset field Q46):** the profit target is a net-profit target for the whole evaluation: S1 "It is the minimum amount of net profits needed for placement with our funding partners."; S9 "It is not a daily or weekly profit target but an overall profit target for the duration of your Evaluation." Amounts are published only on the plan cards (H), not in the help articles. The help center says only "based on the account you choose".
- **Drawdown:** trailing, real time. S3: "The Trailing Drawdown is calculated in Real-Time during your trading day." S3: "Once your Trailing Drawdown reaches your initial starting balance, it will not change for the rest of your evaluation." It "includes simulated commissions and fees". Whether it trails unrealized (open-position) equity is not stated: the firm says "Account Balance" (S1: "Do not allow your Account Balance to hit or exceed this amount") and tells traders to monitor it in R\|Trader "Auto Liquidate Threshold Value". Classified `trailing_intraday` because it is real-time; unrealized treatment is a gap.
- **Daily loss limit:** none. S9: "there is no daily loss limit amount on both the evaluation and funded accounts." S24 (2023-02-09): "we've made the decision to remove the daily loss limit from both Evaluation & Funded accounts."
- **Consistency (evaluation only):** S1: "Traders must have three (3) other best trading days' net profits total summed up to equal 80% or more of the largest day's net profit." Example in the same article: best day $1,000, three other best days must total at least $800; "The consistency can also be completed within two (2) days." S9 words it "any 3 trading days' net profits" (same 80%; S1/S3 wording used). Net = after commissions.
- **Min trading days:** S1: "You must trade a minimum of 10 trading days for the Evaluation Account or 5 trading days for the Pro Evaluation Account to be eligible for placement with our funding partners. Holidays and weekends are not included." Not a maximum ("even if it takes five (5) months", S3), but billing recurs every 30 days (S20).
- **Max position:** S3: "The Max Position Size is the maximum number of allowable open positions at any given time." Across all products. S9: during the evaluation micros have a limited number of contracts by account size (50K: 6, 100K: 12, "and so on"), i.e. micros are counted 1:1 with minis during the evaluation (no 10:1 ratio), no special micro rule.
- **News:** no restriction in the evaluation (S25 "Yes, Evaluation traders are permitted to trade during economic releases."; S8 "*Evaluation traders do not need to be flat during any releases.").
- **Hours:** see §4. All positions closed by 3:15 PM CT; no trading until 5 PM CT. Violation: reset (not an instant ban text; S7 "Did you violate the Permissible Time rule? No problem—you can reset your balance at any time!").
- **Reset fee** S23: "$50 for 25k and 50k accounts / $100 for all other account sizes".
- **Activation fee** (S20/S29): half the subscription at sign-up and half "as an activation fee once approved"; 25K $65, 50K $75, "Other account sizes follow the same split-payment structure". This is why the card price may be the half price: S20 lists "25k Account: $65/month + $65 activation fee"; the card "Get For $65/mo" for 25K matches. (Whether $150/$175/$325 are also the monthly price is as shown on the cards; S20 does not list them.)

### 3.2 Funded Trader Account (phase: funded)

| Size | Drawdown (same size card) | Max contracts at top scaling tier | Scaling tiers (image S6; Cash On Hand = profit above starting balance) | Withdrawal profit threshold (S21) |
|---|---|---|---|---|
| 25K | 1500.00 | 3 | 0-1500: 2 lots; over 1501: 3 | 1500 |
| 50K | 2500.00 | 6 | 0-1500: 2; 1501-4000: 4; over 4001: 6 | 2500 |
| 100K | 3500.00 | 12 | 0-2000: 3; 2001-3000: 4; 3001-5000: 6; over 5001: 12 | 3500 |
| 150K | 5000.00 | 15 | 0-4000: 5; 4001-8000: 8; 8001-12000: 10; over 12001: 15 | 5000 |
| 250K | 5500.00 | 25 | 0-5000: 6; 5001-12000: 12; 12001-20000: 18; over 20001: 25 | 5500 |

- **Drawdown:** S4: "Once your Trailing Drawdown reaches your initial starting balance, it will not change. Your Funded Trading account will be terminated if your account balance touches or falls below this value. The Trailing Drawdown is calculated in real-time during your trading session and it includes commissions and fees." The dollar amount is not in S4; the website cards are not phase-specific, so the same-size card amount is used (see gaps).
- **Scaling (S6):** "Your buying power (number of contracts to trade) will be determined by the Cash On Hand value (seen in R|Trader Pro) at the beginning of the trading session. Note that your buying power for the day remains the same regardless of your floating PNL." Table read from the image `Dynamic Scaling Targets` (S6); not a text quote.
- **Micros:** S9: "you may trade up to 10x micro contracts for 1 standard contract upon request from your funding provider. For example, on the $50,000 account, you can request to trade up to 60 micro contracts of MNQ instead of 6 standard contracts of NQ. Note that you must still adhere to the dynamic scaling rule regardless of which contract type you are trading."
- **No consistency rule when funded:** S9: "this rule applies during the evaluation period only and not to funded accounts."
- **Funded rules (S5), verbatim list:** flat before and during major releases; minimum weekly execution: "Must maintain a minimum weekly total trade execution of no less than 50% of the average number of trades successfully completed on the evaluation."; "Must have a positive net PnL greater than the initial starting balance at the end of 15 calendar days and in subsequent 15 calendar day intervals – Rule only applies during the initial 90-day probationary period ... Discretionary warnings are issued if the account balance is below the initial starting balance."; trades shorter than 10 seconds prohibited (quote in §6); hedging across funded accounts prohibited; max three funded accounts ("In the case of an Express Account, Traders are limited to having only one funded account at any given time."). "* Failure to abide by the above rules will risk your account being terminated." The homepage mentions a "2-strike policy when funded" without detail (gap).
- **Withdrawals (S21):** 100% of the first $10,000 net profit then 90%; threshold per size above; minimum payout $1,000; bank wire or crypto. Not rule-changing for routing; recorded only for the thresholds.

### 3.3 Express Funding (low priority)

S22: only for traders whose funded account was terminated within the last 24 hours; same size as the previous funded account; "The rules for an Express Funding Account are the same as for a normal Evaluation account, except traders can now fast-track the process and complete it within three (3) trading days!"; one Express account at a time; at most two Express accounts in 30 days. Prices are in an image on S22 (not read). No blocks are produced for Express (target and drawdown follow the normal size). The older release note S26 says "in just 5 days" (conflict, §10).

### 3.4 Other sizes

None. Only 25K-250K are sold.

## 4. Trading-day reset

- **Published:** "Trading day starts at 5 PM CT and ends at 3:15 PM CT (not including holidays and weekends)." (S1, S7, S2; homepage: "the trading day starts at 5 PM CT and ends at 3:15 PM CT the next day.") No daily reset of a drawdown or loss limit is described: there is no DLL, and the trailing drawdown is real time. "Trading day" is the unit for the minimum days and the consistency calculation.
- **Timezone as published:** "CT" (Central Time). Daylight-saving behavior is not described.
- **Close-out:** all positions must be flat by 3:15 PM CT ("or the market close of that product, whatever comes first", S7); no positions may be opened between 3:15 PM and 5 PM CT; "No trading is allowed after 3:15 PM CT on Fridays until Sundays at 5 PM CT when the market reopens." These apply to both evaluation and funded accounts (S5: "Trade only allowed instruments during permitted times"). 
- **Holidays:** S7: "We at Oneup will remind our traders whenever possible regarding any holiday-shortened sessions, but it is ultimately the trader's responsibility to follow the Holiday calendar." Holidays and weekends are not counted as trading days. Link to the CME holiday calendar; no further holiday rule.

## 5. Trade copying stance

- **Funded accounts, official text (S5), verbatim:** "Traders are expressly prohibited from engaging in copy-trading activities between funded accounts held under different names or individuals. However, copy-trading is permissible when conducted across multiple funded accounts under the Trader's own name and solely for the purposes of risk management and strategy diversification."
- **Group trading (S5):** "Group trading or trading in concert with other individuals, where identical trades are executed across multiple accounts owned by different Traders, is strictly forbidden."
- **Hedging (S5):** "Hedging positions in any form across multiple funded accounts is strictly prohibited."
- **Evaluation accounts and trade copiers (S13):** "Traders can analyze and execute trades on a single chart while seamlessly replicating them across multiple accounts via a trade copier." and "IMPORTANT NOTE REGARDING TRADE COPIERS: OneUp Trader neither endorses nor is affiliated with any trade copier vendor. OneUp Trader will not be liable for any copier's usage, performance, or technical support. Users are solely responsible for managing multiple accounts under their UserID/email address."
- **Account-count limits (S13, S5):** up to 3 evaluation accounts and 3 funded accounts at once, per email; only one Express account; "Each user is permitted to have only one Rithmic User ID linked to an email address."; "Creating multiple subscriptions using more than one email address to obtain multiple Rithmic user IDs is strictly prohibited."
- **Copying with accounts at other firms:** not mentioned on any official page found (gap). S5 bans copying between funded accounts of different individuals; same-person copying to another firm is neither allowed nor forbidden in the text.
- **Terms (T):** "The account will be personal to you, and you cannot share it with anybody else."
- **Classification:** `conditional` (permitted across the trader's own accounts under one email/UserID; forbidden between different people).

## 6. Automated trading stance

**(i) Classification:** `unknown`. No official page found says that automated trading, algorithms, bots, EAs or webhooks are allowed or forbidden. A search of all 60 help articles and the terms for automat/algo/bot/webhook/TradingView/ninjascript returned no rule (only "renew automatically", billing and platform-connection mentions). `automation_rule_text`: null (nothing to quote). Closest related published text (verbatim, S5): "Trading activities (including, but not limited to Micro scalping, High-Frequency Trading, Latency arbitrage, Tick scalping, etc.) with a duration of less than 10 seconds; and/or that take advantage of market data inefficiencies/errors are strictly prohibited by our funding partners." This bans high-frequency/sub-10-second trading on funded accounts and also applies around news events (S8: "TRADES WITH A DURATION LESS THAN 10 SECONDS PRIOR / DURING / AFTER ANY NEWS EVENT ARE PROHIBITED"). Together with "You may use any platform that supports Rithmic connection." (S9), this does not amount to a published automation permission.

**(ii) Named mechanisms:**
- Semi-automated trading: not mentioned.
- Trade copiers: mentioned (quote in §5; S13 and S5).
- Webhooks / alerts / TradingView: not mentioned.
- ATM / auto brackets / stop and target orders placed by software: not mentioned (a Volfix guide says "All complex orders (T/S, S/L, T/P, Trigger) are processed and stored on server and will work with closed terminal", but that is third-party platform behavior in a connection guide, not a rule).
- Algorithmic / HFT: HFT prohibited (quote above).

**(iii) Differs by phase:** the sub-10-second/HFT ban and the news-flat rule are published as funded rules (S5, S8). The evaluation page (S1/S2) has neither; for evaluation, no restriction on automation or on trade duration is published either way.

## 7. Device / hosting rules and other rules that affect routing or risk

- **VPS / VPN / remote server / personal device:** not published. Searches over all 60 help articles and the terms for vps, vpn, remote, server, device, IP address return nothing relevant (server hits are Rithmic gateway selections, e.g. "Rithmic Paper Chicago server"). The R\|Trader connection guide lets traders pick a gateway by region (Chicago, Europe/Frankfurt, Tokyo/Singapore, S18), and the homepage says "Trade from anywhere in the world, whenever you want!" (H, "Work On Your Terms"); neither states a device rule. `vps_allowed: unknown`, `personal_device_required: unknown` (gaps). Terms only say the account is personal and cannot be shared (quote in §5). R4 re-check 2026-10-23: re-fetched the help sitemap (60 articles), the terms (https://www.oneuptrader.com/terms) and the homepage; searched vps, vpn, remote, virtual private, own/personal computer, device, IP address, shared, third party, location. No device or hosting rule. The only related texts: "Trade from anywhere at any time." (https://www.oneuptrader.com/, marketing copy, not a rule on devices or hosts, so it does not make this `no`) and the card rule "the card used for the transaction must be registered under the user's name" (billing article, payments only). Stays `unknown`.
- **Credentials:** one Rithmic user ID per email; credentials come in the welcome email.
- **News (funded only):** S8: "Funded Traders must be flat one minute before, during, and one minute after a corresponding major economic release" on the listed events (FOMC, NFP, CPI, GDP, ISM, PPI, retail sales, jobless claims, etc. for equity index futures), using Forex Factory as the official source. Unlisted events may be traded through.
- **Mandatory stop / bracket:** not mentioned.
- **Overnight:** not allowed (flat by 3:15 PM CT, §4).
- **Hedging:** prohibited across multiple funded accounts (S5); within one account the firm only describes long one product and short another as allowed ("long 3 CL and Short 2 ES", S3).
- **Scaling:** funded dynamic scaling (§3.2); not in the evaluation.
- **Max accounts:** 3 evaluation, 3 funded, 1 Express (S13, S5).
- **Inactivity (funded):** minimum weekly execution count 50% of the evaluation average; holds need funding-partner approval; no account-hold option on evaluations (S5; help article "Can I put my account on hold and resume later?": "We do not currently offer the option to put your account/subscription on hold").
- **Min trade duration (funded):** 10 seconds (S5 quote in §6).
- **Probation (funded):** positive net PnL above the starting balance at each 15-day check during the first 90 days (S5, discretionary warnings).
- **Data:** no data fees; Professional users pay the exchange fee difference (S18: "$105 per exchange").

## 8. Commissions

| Phase | NQ (per side) | MNQ (per side) | Round turn | Includes | Source |
|---|---|---|---|---|---|
| Evaluation (simulated) | $2.50 | $1.00 | NQ $5.00, MNQ $2.00 (S28: "$2 round trip"; S9 "$2 Round Turn (RT)"; NQ RT derived, only per-side published) | "Commissions and all other fees (exchange, routing, and NFA fees) are factored in on all simulated accounts" | S10, S11 ("Simulated commissions and fees are $2.50/side. Simulated commissions and fees for micros are $1/side for each contract traded."), S28 |
| Funded | $2.04 | $0.60 | NQ $4.08, MNQ $1.20 (both published in the image table) | "the total commissions include regulatory fees, trade commissions, platform fees, and exchange fees" | S12 (image table "TOTAL COMISSIONS PER SIDE / R/T": E-mini NASDAQ 100 (NQ) $2.04 / $4.08; Micro E-Mini Nasdaq-100 (MNQ) $0.60 / $1.20), S9 ("they drop during the Funded Trading account to $1.20 RT") |

Funded exchange data fees are covered by the funding partners (S19); platform fees are the trader's except free R\|Trader Pro and NinjaTrader Desktop 8.1. The funded table values for NQ ($2.04) come only from an image (read visually); the MNQ round-turn figure is also confirmed in text by S9.

## 9. Machine-readable blocks

There are 15 blocks: Evaluation, Pro Evaluation and Funded Trader Account in 25K, 50K, 100K, 150K and 250K. `max_position` for evaluation: minis = contracts published (micros count against the same limit, `micros: null`, no ratio). For funded: minis = top-tier lots, micros = 10x on request (S9), scaling text lists the tiers. `drawdown.type` is `trailing_intraday` (real time), see §3.1. Profit target is `none` for funded.

### Blocks: Evaluation — evaluation

#### Evaluation — evaluation — 25K
```yaml
firm: oneup-trader
plan: "Evaluation"
phase: evaluation
size_label: "25K"
size_usd: "25000.00"
instruments: [MNQ, NQ]
platforms: ["NinjaTrader 8 (Desktop 8.1, via Rithmic for NinjaTrader Brokerage)", "R|Trader Pro", "R|Trader", "Sierra Chart", "ATAS", "Quantower", "Bookmap", "Jigsaw Trading", "MultiCharts", "Motive Wave", "eSignal", "Trade Navigator", "Volfix.net", "Medved Trader", "Agena Trader", "Inside Edge Trader", "Investor RT", "Photon", "QScalp", "QST", "Track'n Trade"]
drawdown: { amount_usd: "1500.00", type: trailing_intraday, stops_trailing_at: "stops trailing once the trailing drawdown reaches the initial account balance (starting balance); fixed there for the rest of the account" }
daily_loss_limit: { amount_usd: null, kind: none }
profit_target_usd: "1500.00"
consistency_rule: { text: "Three other best trading days' net profits must sum to 80% or more of the largest trading day's net profit (net after commissions); can be met within two days", threshold_pct: "80" }
max_position: { minis: 3, micros: null, scaling: "Max position size by account size, all products combined; micros count toward the limit (50K: 6 contracts at any time; micros not given a separate ratio during evaluation)" }
min_trading_days: "10"
trading_day_reset: { time: "17:00", tz_as_published: "CT" }
commission_per_contract: { mnq_usd: "1.00", nq_usd: "2.50", basis: per_side, includes: "simulated commissions and fees; exchange, routing and NFA fees are factored in" }
copy_trading_allowed: conditional
automated_trading_allowed: unknown
automation_rule_text: null
vps_allowed: unknown
personal_device_required: unknown
status: partial
not_confirmed_fields: [automated_trading_allowed, automation_rule_text, vps_allowed, personal_device_required, copy_trading_allowed, drawdown.type]
notes: "Same size/target/drawdown on the Evaluation and Pro Evaluation cards; Pro Evaluation differs only in 5 minimum trading days and price ($100/mo vs $65/mo). Evaluation: all positions flat by 3:15 PM CT; no DLL; no news restriction; trailing drawdown calculated in real time and includes simulated commissions and fees."
sources:
  - { field: drawdown, url: "https://www.oneuptrader.com/", accessed: "2026-10-23" }
  - { field: drawdown, url: "https://help.oneuptrader.com/article/389-rules-definitions", accessed: "2026-10-23" }
  - { field: daily_loss_limit, url: "https://help.oneuptrader.com/article/468-most-popular-questions", accessed: "2026-10-23" }
  - { field: daily_loss_limit, url: "https://help.oneuptrader.com/article/486-no-more-daily-loss-limit-restrictions", accessed: "2026-10-23" }
  - { field: profit_target_usd, url: "https://www.oneuptrader.com/", accessed: "2026-10-23" }
  - { field: max_position, url: "https://www.oneuptrader.com/", accessed: "2026-10-23" }
  - { field: consistency_rule, url: "https://help.oneuptrader.com/article/385-evaluation-rules-parameters", accessed: "2026-10-23" }
  - { field: consistency_rule, url: "https://help.oneuptrader.com/article/468-most-popular-questions", accessed: "2026-10-23" }
  - { field: min_trading_days, url: "https://help.oneuptrader.com/article/385-evaluation-rules-parameters", accessed: "2026-10-23" }
  - { field: trading_day_reset, url: "https://help.oneuptrader.com/article/385-evaluation-rules-parameters", accessed: "2026-10-23" }
  - { field: trading_day_reset, url: "https://help.oneuptrader.com/article/390-permissible-products-times", accessed: "2026-10-23" }
  - { field: commission_per_contract, url: "https://help.oneuptrader.com/article/391-important-evaluation-info", accessed: "2026-10-23" }
  - { field: commission_per_contract, url: "https://help.oneuptrader.com/article/468-most-popular-questions", accessed: "2026-10-23" }
  - { field: platforms, url: "https://help.oneuptrader.com/article/380-supported-platforms", accessed: "2026-10-23" }
  - { field: platforms, url: "https://help.oneuptrader.com/article/491-ninjatrader-desktop-version-v-8-1-connection-guide", accessed: "2026-10-23" }
  - { field: copy_trading_allowed, url: "https://help.oneuptrader.com/article/505-can-i-have-multiple-evaluation-accounts", accessed: "2026-10-23" }
  - { field: copy_trading_allowed, url: "https://help.oneuptrader.com/article/372-funded-trader-rules", accessed: "2026-10-23" }
  - { field: automated_trading_allowed, url: "https://help.oneuptrader.com/article/372-funded-trader-rules", accessed: "2026-10-23" }
  - { field: vps_allowed, url: "https://help.oneuptrader.com/", accessed: "2026-10-23" }
  - { field: personal_device_required, url: "https://www.oneuptrader.com/terms", accessed: "2026-10-23" }
```

#### Evaluation — evaluation — 50K
```yaml
firm: oneup-trader
plan: "Evaluation"
phase: evaluation
size_label: "50K"
size_usd: "50000.00"
instruments: [MNQ, NQ]
platforms: ["NinjaTrader 8 (Desktop 8.1, via Rithmic for NinjaTrader Brokerage)", "R|Trader Pro", "R|Trader", "Sierra Chart", "ATAS", "Quantower", "Bookmap", "Jigsaw Trading", "MultiCharts", "Motive Wave", "eSignal", "Trade Navigator", "Volfix.net", "Medved Trader", "Agena Trader", "Inside Edge Trader", "Investor RT", "Photon", "QScalp", "QST", "Track'n Trade"]
drawdown: { amount_usd: "2500.00", type: trailing_intraday, stops_trailing_at: "stops trailing once the trailing drawdown reaches the initial account balance (starting balance); fixed there for the rest of the account" }
daily_loss_limit: { amount_usd: null, kind: none }
profit_target_usd: "3000.00"
consistency_rule: { text: "Three other best trading days' net profits must sum to 80% or more of the largest trading day's net profit (net after commissions); can be met within two days", threshold_pct: "80" }
max_position: { minis: 6, micros: null, scaling: "Max position size by account size, all products combined; micros count toward the limit (50K: 6 contracts at any time; micros not given a separate ratio during evaluation)" }
min_trading_days: "10"
trading_day_reset: { time: "17:00", tz_as_published: "CT" }
commission_per_contract: { mnq_usd: "1.00", nq_usd: "2.50", basis: per_side, includes: "simulated commissions and fees; exchange, routing and NFA fees are factored in" }
copy_trading_allowed: conditional
automated_trading_allowed: unknown
automation_rule_text: null
vps_allowed: unknown
personal_device_required: unknown
status: partial
not_confirmed_fields: [automated_trading_allowed, automation_rule_text, vps_allowed, personal_device_required, copy_trading_allowed, drawdown.type]
notes: "Same size/target/drawdown on the Evaluation and Pro Evaluation cards; Pro Evaluation differs only in 5 minimum trading days and price ($115/mo vs $75/mo). Evaluation: all positions flat by 3:15 PM CT; no DLL; no news restriction; trailing drawdown calculated in real time and includes simulated commissions and fees."
sources:
  - { field: drawdown, url: "https://www.oneuptrader.com/", accessed: "2026-10-23" }
  - { field: drawdown, url: "https://help.oneuptrader.com/article/389-rules-definitions", accessed: "2026-10-23" }
  - { field: daily_loss_limit, url: "https://help.oneuptrader.com/article/468-most-popular-questions", accessed: "2026-10-23" }
  - { field: daily_loss_limit, url: "https://help.oneuptrader.com/article/486-no-more-daily-loss-limit-restrictions", accessed: "2026-10-23" }
  - { field: profit_target_usd, url: "https://www.oneuptrader.com/", accessed: "2026-10-23" }
  - { field: max_position, url: "https://www.oneuptrader.com/", accessed: "2026-10-23" }
  - { field: consistency_rule, url: "https://help.oneuptrader.com/article/385-evaluation-rules-parameters", accessed: "2026-10-23" }
  - { field: consistency_rule, url: "https://help.oneuptrader.com/article/468-most-popular-questions", accessed: "2026-10-23" }
  - { field: min_trading_days, url: "https://help.oneuptrader.com/article/385-evaluation-rules-parameters", accessed: "2026-10-23" }
  - { field: trading_day_reset, url: "https://help.oneuptrader.com/article/385-evaluation-rules-parameters", accessed: "2026-10-23" }
  - { field: trading_day_reset, url: "https://help.oneuptrader.com/article/390-permissible-products-times", accessed: "2026-10-23" }
  - { field: commission_per_contract, url: "https://help.oneuptrader.com/article/391-important-evaluation-info", accessed: "2026-10-23" }
  - { field: commission_per_contract, url: "https://help.oneuptrader.com/article/468-most-popular-questions", accessed: "2026-10-23" }
  - { field: platforms, url: "https://help.oneuptrader.com/article/380-supported-platforms", accessed: "2026-10-23" }
  - { field: platforms, url: "https://help.oneuptrader.com/article/491-ninjatrader-desktop-version-v-8-1-connection-guide", accessed: "2026-10-23" }
  - { field: copy_trading_allowed, url: "https://help.oneuptrader.com/article/505-can-i-have-multiple-evaluation-accounts", accessed: "2026-10-23" }
  - { field: copy_trading_allowed, url: "https://help.oneuptrader.com/article/372-funded-trader-rules", accessed: "2026-10-23" }
  - { field: automated_trading_allowed, url: "https://help.oneuptrader.com/article/372-funded-trader-rules", accessed: "2026-10-23" }
  - { field: vps_allowed, url: "https://help.oneuptrader.com/", accessed: "2026-10-23" }
  - { field: personal_device_required, url: "https://www.oneuptrader.com/terms", accessed: "2026-10-23" }
```

#### Evaluation — evaluation — 100K
```yaml
firm: oneup-trader
plan: "Evaluation"
phase: evaluation
size_label: "100K"
size_usd: "100000.00"
instruments: [MNQ, NQ]
platforms: ["NinjaTrader 8 (Desktop 8.1, via Rithmic for NinjaTrader Brokerage)", "R|Trader Pro", "R|Trader", "Sierra Chart", "ATAS", "Quantower", "Bookmap", "Jigsaw Trading", "MultiCharts", "Motive Wave", "eSignal", "Trade Navigator", "Volfix.net", "Medved Trader", "Agena Trader", "Inside Edge Trader", "Investor RT", "Photon", "QScalp", "QST", "Track'n Trade"]
drawdown: { amount_usd: "3500.00", type: trailing_intraday, stops_trailing_at: "stops trailing once the trailing drawdown reaches the initial account balance (starting balance); fixed there for the rest of the account" }
daily_loss_limit: { amount_usd: null, kind: none }
profit_target_usd: "6000.00"
consistency_rule: { text: "Three other best trading days' net profits must sum to 80% or more of the largest trading day's net profit (net after commissions); can be met within two days", threshold_pct: "80" }
max_position: { minis: 12, micros: null, scaling: "Max position size by account size, all products combined; micros count toward the limit (50K: 6 contracts at any time; micros not given a separate ratio during evaluation)" }
min_trading_days: "10"
trading_day_reset: { time: "17:00", tz_as_published: "CT" }
commission_per_contract: { mnq_usd: "1.00", nq_usd: "2.50", basis: per_side, includes: "simulated commissions and fees; exchange, routing and NFA fees are factored in" }
copy_trading_allowed: conditional
automated_trading_allowed: unknown
automation_rule_text: null
vps_allowed: unknown
personal_device_required: unknown
status: partial
not_confirmed_fields: [automated_trading_allowed, automation_rule_text, vps_allowed, personal_device_required, copy_trading_allowed, drawdown.type]
notes: "Same size/target/drawdown on the Evaluation and Pro Evaluation cards; Pro Evaluation differs only in 5 minimum trading days and price ($225/mo vs $150/mo). Evaluation: all positions flat by 3:15 PM CT; no DLL; no news restriction; trailing drawdown calculated in real time and includes simulated commissions and fees."
sources:
  - { field: drawdown, url: "https://www.oneuptrader.com/", accessed: "2026-10-23" }
  - { field: drawdown, url: "https://help.oneuptrader.com/article/389-rules-definitions", accessed: "2026-10-23" }
  - { field: daily_loss_limit, url: "https://help.oneuptrader.com/article/468-most-popular-questions", accessed: "2026-10-23" }
  - { field: daily_loss_limit, url: "https://help.oneuptrader.com/article/486-no-more-daily-loss-limit-restrictions", accessed: "2026-10-23" }
  - { field: profit_target_usd, url: "https://www.oneuptrader.com/", accessed: "2026-10-23" }
  - { field: max_position, url: "https://www.oneuptrader.com/", accessed: "2026-10-23" }
  - { field: consistency_rule, url: "https://help.oneuptrader.com/article/385-evaluation-rules-parameters", accessed: "2026-10-23" }
  - { field: consistency_rule, url: "https://help.oneuptrader.com/article/468-most-popular-questions", accessed: "2026-10-23" }
  - { field: min_trading_days, url: "https://help.oneuptrader.com/article/385-evaluation-rules-parameters", accessed: "2026-10-23" }
  - { field: trading_day_reset, url: "https://help.oneuptrader.com/article/385-evaluation-rules-parameters", accessed: "2026-10-23" }
  - { field: trading_day_reset, url: "https://help.oneuptrader.com/article/390-permissible-products-times", accessed: "2026-10-23" }
  - { field: commission_per_contract, url: "https://help.oneuptrader.com/article/391-important-evaluation-info", accessed: "2026-10-23" }
  - { field: commission_per_contract, url: "https://help.oneuptrader.com/article/468-most-popular-questions", accessed: "2026-10-23" }
  - { field: platforms, url: "https://help.oneuptrader.com/article/380-supported-platforms", accessed: "2026-10-23" }
  - { field: platforms, url: "https://help.oneuptrader.com/article/491-ninjatrader-desktop-version-v-8-1-connection-guide", accessed: "2026-10-23" }
  - { field: copy_trading_allowed, url: "https://help.oneuptrader.com/article/505-can-i-have-multiple-evaluation-accounts", accessed: "2026-10-23" }
  - { field: copy_trading_allowed, url: "https://help.oneuptrader.com/article/372-funded-trader-rules", accessed: "2026-10-23" }
  - { field: automated_trading_allowed, url: "https://help.oneuptrader.com/article/372-funded-trader-rules", accessed: "2026-10-23" }
  - { field: vps_allowed, url: "https://help.oneuptrader.com/", accessed: "2026-10-23" }
  - { field: personal_device_required, url: "https://www.oneuptrader.com/terms", accessed: "2026-10-23" }
```

#### Evaluation — evaluation — 150K
```yaml
firm: oneup-trader
plan: "Evaluation"
phase: evaluation
size_label: "150K"
size_usd: "150000.00"
instruments: [MNQ, NQ]
platforms: ["NinjaTrader 8 (Desktop 8.1, via Rithmic for NinjaTrader Brokerage)", "R|Trader Pro", "R|Trader", "Sierra Chart", "ATAS", "Quantower", "Bookmap", "Jigsaw Trading", "MultiCharts", "Motive Wave", "eSignal", "Trade Navigator", "Volfix.net", "Medved Trader", "Agena Trader", "Inside Edge Trader", "Investor RT", "Photon", "QScalp", "QST", "Track'n Trade"]
drawdown: { amount_usd: "5000.00", type: trailing_intraday, stops_trailing_at: "stops trailing once the trailing drawdown reaches the initial account balance (starting balance); fixed there for the rest of the account" }
daily_loss_limit: { amount_usd: null, kind: none }
profit_target_usd: "9000.00"
consistency_rule: { text: "Three other best trading days' net profits must sum to 80% or more of the largest trading day's net profit (net after commissions); can be met within two days", threshold_pct: "80" }
max_position: { minis: 15, micros: null, scaling: "Max position size by account size, all products combined; micros count toward the limit (50K: 6 contracts at any time; micros not given a separate ratio during evaluation)" }
min_trading_days: "10"
trading_day_reset: { time: "17:00", tz_as_published: "CT" }
commission_per_contract: { mnq_usd: "1.00", nq_usd: "2.50", basis: per_side, includes: "simulated commissions and fees; exchange, routing and NFA fees are factored in" }
copy_trading_allowed: conditional
automated_trading_allowed: unknown
automation_rule_text: null
vps_allowed: unknown
personal_device_required: unknown
status: partial
not_confirmed_fields: [automated_trading_allowed, automation_rule_text, vps_allowed, personal_device_required, copy_trading_allowed, drawdown.type]
notes: "Same size/target/drawdown on the Evaluation and Pro Evaluation cards; Pro Evaluation differs only in 5 minimum trading days and price ($265/mo vs $175/mo). Evaluation: all positions flat by 3:15 PM CT; no DLL; no news restriction; trailing drawdown calculated in real time and includes simulated commissions and fees."
sources:
  - { field: drawdown, url: "https://www.oneuptrader.com/", accessed: "2026-10-23" }
  - { field: drawdown, url: "https://help.oneuptrader.com/article/389-rules-definitions", accessed: "2026-10-23" }
  - { field: daily_loss_limit, url: "https://help.oneuptrader.com/article/468-most-popular-questions", accessed: "2026-10-23" }
  - { field: daily_loss_limit, url: "https://help.oneuptrader.com/article/486-no-more-daily-loss-limit-restrictions", accessed: "2026-10-23" }
  - { field: profit_target_usd, url: "https://www.oneuptrader.com/", accessed: "2026-10-23" }
  - { field: max_position, url: "https://www.oneuptrader.com/", accessed: "2026-10-23" }
  - { field: consistency_rule, url: "https://help.oneuptrader.com/article/385-evaluation-rules-parameters", accessed: "2026-10-23" }
  - { field: consistency_rule, url: "https://help.oneuptrader.com/article/468-most-popular-questions", accessed: "2026-10-23" }
  - { field: min_trading_days, url: "https://help.oneuptrader.com/article/385-evaluation-rules-parameters", accessed: "2026-10-23" }
  - { field: trading_day_reset, url: "https://help.oneuptrader.com/article/385-evaluation-rules-parameters", accessed: "2026-10-23" }
  - { field: trading_day_reset, url: "https://help.oneuptrader.com/article/390-permissible-products-times", accessed: "2026-10-23" }
  - { field: commission_per_contract, url: "https://help.oneuptrader.com/article/391-important-evaluation-info", accessed: "2026-10-23" }
  - { field: commission_per_contract, url: "https://help.oneuptrader.com/article/468-most-popular-questions", accessed: "2026-10-23" }
  - { field: platforms, url: "https://help.oneuptrader.com/article/380-supported-platforms", accessed: "2026-10-23" }
  - { field: platforms, url: "https://help.oneuptrader.com/article/491-ninjatrader-desktop-version-v-8-1-connection-guide", accessed: "2026-10-23" }
  - { field: copy_trading_allowed, url: "https://help.oneuptrader.com/article/505-can-i-have-multiple-evaluation-accounts", accessed: "2026-10-23" }
  - { field: copy_trading_allowed, url: "https://help.oneuptrader.com/article/372-funded-trader-rules", accessed: "2026-10-23" }
  - { field: automated_trading_allowed, url: "https://help.oneuptrader.com/article/372-funded-trader-rules", accessed: "2026-10-23" }
  - { field: vps_allowed, url: "https://help.oneuptrader.com/", accessed: "2026-10-23" }
  - { field: personal_device_required, url: "https://www.oneuptrader.com/terms", accessed: "2026-10-23" }
```

#### Evaluation — evaluation — 250K
```yaml
firm: oneup-trader
plan: "Evaluation"
phase: evaluation
size_label: "250K"
size_usd: "250000.00"
instruments: [MNQ, NQ]
platforms: ["NinjaTrader 8 (Desktop 8.1, via Rithmic for NinjaTrader Brokerage)", "R|Trader Pro", "R|Trader", "Sierra Chart", "ATAS", "Quantower", "Bookmap", "Jigsaw Trading", "MultiCharts", "Motive Wave", "eSignal", "Trade Navigator", "Volfix.net", "Medved Trader", "Agena Trader", "Inside Edge Trader", "Investor RT", "Photon", "QScalp", "QST", "Track'n Trade"]
drawdown: { amount_usd: "5500.00", type: trailing_intraday, stops_trailing_at: "stops trailing once the trailing drawdown reaches the initial account balance (starting balance); fixed there for the rest of the account" }
daily_loss_limit: { amount_usd: null, kind: none }
profit_target_usd: "15000.00"
consistency_rule: { text: "Three other best trading days' net profits must sum to 80% or more of the largest trading day's net profit (net after commissions); can be met within two days", threshold_pct: "80" }
max_position: { minis: 25, micros: null, scaling: "Max position size by account size, all products combined; micros count toward the limit (50K: 6 contracts at any time; micros not given a separate ratio during evaluation)" }
min_trading_days: "10"
trading_day_reset: { time: "17:00", tz_as_published: "CT" }
commission_per_contract: { mnq_usd: "1.00", nq_usd: "2.50", basis: per_side, includes: "simulated commissions and fees; exchange, routing and NFA fees are factored in" }
copy_trading_allowed: conditional
automated_trading_allowed: unknown
automation_rule_text: null
vps_allowed: unknown
personal_device_required: unknown
status: partial
not_confirmed_fields: [automated_trading_allowed, automation_rule_text, vps_allowed, personal_device_required, copy_trading_allowed, drawdown.type]
notes: "Same size/target/drawdown on the Evaluation and Pro Evaluation cards; Pro Evaluation differs only in 5 minimum trading days and price ($490/mo vs $325/mo). Evaluation: all positions flat by 3:15 PM CT; no DLL; no news restriction; trailing drawdown calculated in real time and includes simulated commissions and fees."
sources:
  - { field: drawdown, url: "https://www.oneuptrader.com/", accessed: "2026-10-23" }
  - { field: drawdown, url: "https://help.oneuptrader.com/article/389-rules-definitions", accessed: "2026-10-23" }
  - { field: daily_loss_limit, url: "https://help.oneuptrader.com/article/468-most-popular-questions", accessed: "2026-10-23" }
  - { field: daily_loss_limit, url: "https://help.oneuptrader.com/article/486-no-more-daily-loss-limit-restrictions", accessed: "2026-10-23" }
  - { field: profit_target_usd, url: "https://www.oneuptrader.com/", accessed: "2026-10-23" }
  - { field: max_position, url: "https://www.oneuptrader.com/", accessed: "2026-10-23" }
  - { field: consistency_rule, url: "https://help.oneuptrader.com/article/385-evaluation-rules-parameters", accessed: "2026-10-23" }
  - { field: consistency_rule, url: "https://help.oneuptrader.com/article/468-most-popular-questions", accessed: "2026-10-23" }
  - { field: min_trading_days, url: "https://help.oneuptrader.com/article/385-evaluation-rules-parameters", accessed: "2026-10-23" }
  - { field: trading_day_reset, url: "https://help.oneuptrader.com/article/385-evaluation-rules-parameters", accessed: "2026-10-23" }
  - { field: trading_day_reset, url: "https://help.oneuptrader.com/article/390-permissible-products-times", accessed: "2026-10-23" }
  - { field: commission_per_contract, url: "https://help.oneuptrader.com/article/391-important-evaluation-info", accessed: "2026-10-23" }
  - { field: commission_per_contract, url: "https://help.oneuptrader.com/article/468-most-popular-questions", accessed: "2026-10-23" }
  - { field: platforms, url: "https://help.oneuptrader.com/article/380-supported-platforms", accessed: "2026-10-23" }
  - { field: platforms, url: "https://help.oneuptrader.com/article/491-ninjatrader-desktop-version-v-8-1-connection-guide", accessed: "2026-10-23" }
  - { field: copy_trading_allowed, url: "https://help.oneuptrader.com/article/505-can-i-have-multiple-evaluation-accounts", accessed: "2026-10-23" }
  - { field: copy_trading_allowed, url: "https://help.oneuptrader.com/article/372-funded-trader-rules", accessed: "2026-10-23" }
  - { field: automated_trading_allowed, url: "https://help.oneuptrader.com/article/372-funded-trader-rules", accessed: "2026-10-23" }
  - { field: vps_allowed, url: "https://help.oneuptrader.com/", accessed: "2026-10-23" }
  - { field: personal_device_required, url: "https://www.oneuptrader.com/terms", accessed: "2026-10-23" }
```

### Blocks: Pro Evaluation — evaluation

#### Pro Evaluation — evaluation — 25K
```yaml
firm: oneup-trader
plan: "Pro Evaluation"
phase: evaluation
size_label: "25K"
size_usd: "25000.00"
instruments: [MNQ, NQ]
platforms: ["NinjaTrader 8 (Desktop 8.1, via Rithmic for NinjaTrader Brokerage)", "R|Trader Pro", "R|Trader", "Sierra Chart", "ATAS", "Quantower", "Bookmap", "Jigsaw Trading", "MultiCharts", "Motive Wave", "eSignal", "Trade Navigator", "Volfix.net", "Medved Trader", "Agena Trader", "Inside Edge Trader", "Investor RT", "Photon", "QScalp", "QST", "Track'n Trade"]
drawdown: { amount_usd: "1500.00", type: trailing_intraday, stops_trailing_at: "stops trailing once the trailing drawdown reaches the initial account balance (starting balance); fixed there for the rest of the account" }
daily_loss_limit: { amount_usd: null, kind: none }
profit_target_usd: "1500.00"
consistency_rule: { text: "Three other best trading days' net profits must sum to 80% or more of the largest trading day's net profit (net after commissions); can be met within two days", threshold_pct: "80" }
max_position: { minis: 3, micros: null, scaling: "Max position size by account size, all products combined; micros count toward the limit (50K: 6 contracts at any time; micros not given a separate ratio during evaluation)" }
min_trading_days: "5"
trading_day_reset: { time: "17:00", tz_as_published: "CT" }
commission_per_contract: { mnq_usd: "1.00", nq_usd: "2.50", basis: per_side, includes: "simulated commissions and fees; exchange, routing and NFA fees are factored in" }
copy_trading_allowed: conditional
automated_trading_allowed: unknown
automation_rule_text: null
vps_allowed: unknown
personal_device_required: unknown
status: partial
not_confirmed_fields: [automated_trading_allowed, automation_rule_text, vps_allowed, personal_device_required, copy_trading_allowed, drawdown.type]
notes: "Same size/target/drawdown on the Evaluation and Pro Evaluation cards; Pro Evaluation differs only in 5 minimum trading days and price ($100/mo vs $65/mo). Evaluation: all positions flat by 3:15 PM CT; no DLL; no news restriction; trailing drawdown calculated in real time and includes simulated commissions and fees."
sources:
  - { field: drawdown, url: "https://www.oneuptrader.com/", accessed: "2026-10-23" }
  - { field: drawdown, url: "https://help.oneuptrader.com/article/389-rules-definitions", accessed: "2026-10-23" }
  - { field: daily_loss_limit, url: "https://help.oneuptrader.com/article/468-most-popular-questions", accessed: "2026-10-23" }
  - { field: daily_loss_limit, url: "https://help.oneuptrader.com/article/486-no-more-daily-loss-limit-restrictions", accessed: "2026-10-23" }
  - { field: profit_target_usd, url: "https://www.oneuptrader.com/", accessed: "2026-10-23" }
  - { field: max_position, url: "https://www.oneuptrader.com/", accessed: "2026-10-23" }
  - { field: consistency_rule, url: "https://help.oneuptrader.com/article/385-evaluation-rules-parameters", accessed: "2026-10-23" }
  - { field: consistency_rule, url: "https://help.oneuptrader.com/article/468-most-popular-questions", accessed: "2026-10-23" }
  - { field: min_trading_days, url: "https://help.oneuptrader.com/article/385-evaluation-rules-parameters", accessed: "2026-10-23" }
  - { field: trading_day_reset, url: "https://help.oneuptrader.com/article/385-evaluation-rules-parameters", accessed: "2026-10-23" }
  - { field: trading_day_reset, url: "https://help.oneuptrader.com/article/390-permissible-products-times", accessed: "2026-10-23" }
  - { field: commission_per_contract, url: "https://help.oneuptrader.com/article/391-important-evaluation-info", accessed: "2026-10-23" }
  - { field: commission_per_contract, url: "https://help.oneuptrader.com/article/468-most-popular-questions", accessed: "2026-10-23" }
  - { field: platforms, url: "https://help.oneuptrader.com/article/380-supported-platforms", accessed: "2026-10-23" }
  - { field: platforms, url: "https://help.oneuptrader.com/article/491-ninjatrader-desktop-version-v-8-1-connection-guide", accessed: "2026-10-23" }
  - { field: copy_trading_allowed, url: "https://help.oneuptrader.com/article/505-can-i-have-multiple-evaluation-accounts", accessed: "2026-10-23" }
  - { field: copy_trading_allowed, url: "https://help.oneuptrader.com/article/372-funded-trader-rules", accessed: "2026-10-23" }
  - { field: automated_trading_allowed, url: "https://help.oneuptrader.com/article/372-funded-trader-rules", accessed: "2026-10-23" }
  - { field: vps_allowed, url: "https://help.oneuptrader.com/", accessed: "2026-10-23" }
  - { field: personal_device_required, url: "https://www.oneuptrader.com/terms", accessed: "2026-10-23" }
```

#### Pro Evaluation — evaluation — 50K
```yaml
firm: oneup-trader
plan: "Pro Evaluation"
phase: evaluation
size_label: "50K"
size_usd: "50000.00"
instruments: [MNQ, NQ]
platforms: ["NinjaTrader 8 (Desktop 8.1, via Rithmic for NinjaTrader Brokerage)", "R|Trader Pro", "R|Trader", "Sierra Chart", "ATAS", "Quantower", "Bookmap", "Jigsaw Trading", "MultiCharts", "Motive Wave", "eSignal", "Trade Navigator", "Volfix.net", "Medved Trader", "Agena Trader", "Inside Edge Trader", "Investor RT", "Photon", "QScalp", "QST", "Track'n Trade"]
drawdown: { amount_usd: "2500.00", type: trailing_intraday, stops_trailing_at: "stops trailing once the trailing drawdown reaches the initial account balance (starting balance); fixed there for the rest of the account" }
daily_loss_limit: { amount_usd: null, kind: none }
profit_target_usd: "3000.00"
consistency_rule: { text: "Three other best trading days' net profits must sum to 80% or more of the largest trading day's net profit (net after commissions); can be met within two days", threshold_pct: "80" }
max_position: { minis: 6, micros: null, scaling: "Max position size by account size, all products combined; micros count toward the limit (50K: 6 contracts at any time; micros not given a separate ratio during evaluation)" }
min_trading_days: "5"
trading_day_reset: { time: "17:00", tz_as_published: "CT" }
commission_per_contract: { mnq_usd: "1.00", nq_usd: "2.50", basis: per_side, includes: "simulated commissions and fees; exchange, routing and NFA fees are factored in" }
copy_trading_allowed: conditional
automated_trading_allowed: unknown
automation_rule_text: null
vps_allowed: unknown
personal_device_required: unknown
status: partial
not_confirmed_fields: [automated_trading_allowed, automation_rule_text, vps_allowed, personal_device_required, copy_trading_allowed, drawdown.type]
notes: "Same size/target/drawdown on the Evaluation and Pro Evaluation cards; Pro Evaluation differs only in 5 minimum trading days and price ($115/mo vs $75/mo). Evaluation: all positions flat by 3:15 PM CT; no DLL; no news restriction; trailing drawdown calculated in real time and includes simulated commissions and fees."
sources:
  - { field: drawdown, url: "https://www.oneuptrader.com/", accessed: "2026-10-23" }
  - { field: drawdown, url: "https://help.oneuptrader.com/article/389-rules-definitions", accessed: "2026-10-23" }
  - { field: daily_loss_limit, url: "https://help.oneuptrader.com/article/468-most-popular-questions", accessed: "2026-10-23" }
  - { field: daily_loss_limit, url: "https://help.oneuptrader.com/article/486-no-more-daily-loss-limit-restrictions", accessed: "2026-10-23" }
  - { field: profit_target_usd, url: "https://www.oneuptrader.com/", accessed: "2026-10-23" }
  - { field: max_position, url: "https://www.oneuptrader.com/", accessed: "2026-10-23" }
  - { field: consistency_rule, url: "https://help.oneuptrader.com/article/385-evaluation-rules-parameters", accessed: "2026-10-23" }
  - { field: consistency_rule, url: "https://help.oneuptrader.com/article/468-most-popular-questions", accessed: "2026-10-23" }
  - { field: min_trading_days, url: "https://help.oneuptrader.com/article/385-evaluation-rules-parameters", accessed: "2026-10-23" }
  - { field: trading_day_reset, url: "https://help.oneuptrader.com/article/385-evaluation-rules-parameters", accessed: "2026-10-23" }
  - { field: trading_day_reset, url: "https://help.oneuptrader.com/article/390-permissible-products-times", accessed: "2026-10-23" }
  - { field: commission_per_contract, url: "https://help.oneuptrader.com/article/391-important-evaluation-info", accessed: "2026-10-23" }
  - { field: commission_per_contract, url: "https://help.oneuptrader.com/article/468-most-popular-questions", accessed: "2026-10-23" }
  - { field: platforms, url: "https://help.oneuptrader.com/article/380-supported-platforms", accessed: "2026-10-23" }
  - { field: platforms, url: "https://help.oneuptrader.com/article/491-ninjatrader-desktop-version-v-8-1-connection-guide", accessed: "2026-10-23" }
  - { field: copy_trading_allowed, url: "https://help.oneuptrader.com/article/505-can-i-have-multiple-evaluation-accounts", accessed: "2026-10-23" }
  - { field: copy_trading_allowed, url: "https://help.oneuptrader.com/article/372-funded-trader-rules", accessed: "2026-10-23" }
  - { field: automated_trading_allowed, url: "https://help.oneuptrader.com/article/372-funded-trader-rules", accessed: "2026-10-23" }
  - { field: vps_allowed, url: "https://help.oneuptrader.com/", accessed: "2026-10-23" }
  - { field: personal_device_required, url: "https://www.oneuptrader.com/terms", accessed: "2026-10-23" }
```

#### Pro Evaluation — evaluation — 100K
```yaml
firm: oneup-trader
plan: "Pro Evaluation"
phase: evaluation
size_label: "100K"
size_usd: "100000.00"
instruments: [MNQ, NQ]
platforms: ["NinjaTrader 8 (Desktop 8.1, via Rithmic for NinjaTrader Brokerage)", "R|Trader Pro", "R|Trader", "Sierra Chart", "ATAS", "Quantower", "Bookmap", "Jigsaw Trading", "MultiCharts", "Motive Wave", "eSignal", "Trade Navigator", "Volfix.net", "Medved Trader", "Agena Trader", "Inside Edge Trader", "Investor RT", "Photon", "QScalp", "QST", "Track'n Trade"]
drawdown: { amount_usd: "3500.00", type: trailing_intraday, stops_trailing_at: "stops trailing once the trailing drawdown reaches the initial account balance (starting balance); fixed there for the rest of the account" }
daily_loss_limit: { amount_usd: null, kind: none }
profit_target_usd: "6000.00"
consistency_rule: { text: "Three other best trading days' net profits must sum to 80% or more of the largest trading day's net profit (net after commissions); can be met within two days", threshold_pct: "80" }
max_position: { minis: 12, micros: null, scaling: "Max position size by account size, all products combined; micros count toward the limit (50K: 6 contracts at any time; micros not given a separate ratio during evaluation)" }
min_trading_days: "5"
trading_day_reset: { time: "17:00", tz_as_published: "CT" }
commission_per_contract: { mnq_usd: "1.00", nq_usd: "2.50", basis: per_side, includes: "simulated commissions and fees; exchange, routing and NFA fees are factored in" }
copy_trading_allowed: conditional
automated_trading_allowed: unknown
automation_rule_text: null
vps_allowed: unknown
personal_device_required: unknown
status: partial
not_confirmed_fields: [automated_trading_allowed, automation_rule_text, vps_allowed, personal_device_required, copy_trading_allowed, drawdown.type]
notes: "Same size/target/drawdown on the Evaluation and Pro Evaluation cards; Pro Evaluation differs only in 5 minimum trading days and price ($225/mo vs $150/mo). Evaluation: all positions flat by 3:15 PM CT; no DLL; no news restriction; trailing drawdown calculated in real time and includes simulated commissions and fees."
sources:
  - { field: drawdown, url: "https://www.oneuptrader.com/", accessed: "2026-10-23" }
  - { field: drawdown, url: "https://help.oneuptrader.com/article/389-rules-definitions", accessed: "2026-10-23" }
  - { field: daily_loss_limit, url: "https://help.oneuptrader.com/article/468-most-popular-questions", accessed: "2026-10-23" }
  - { field: daily_loss_limit, url: "https://help.oneuptrader.com/article/486-no-more-daily-loss-limit-restrictions", accessed: "2026-10-23" }
  - { field: profit_target_usd, url: "https://www.oneuptrader.com/", accessed: "2026-10-23" }
  - { field: max_position, url: "https://www.oneuptrader.com/", accessed: "2026-10-23" }
  - { field: consistency_rule, url: "https://help.oneuptrader.com/article/385-evaluation-rules-parameters", accessed: "2026-10-23" }
  - { field: consistency_rule, url: "https://help.oneuptrader.com/article/468-most-popular-questions", accessed: "2026-10-23" }
  - { field: min_trading_days, url: "https://help.oneuptrader.com/article/385-evaluation-rules-parameters", accessed: "2026-10-23" }
  - { field: trading_day_reset, url: "https://help.oneuptrader.com/article/385-evaluation-rules-parameters", accessed: "2026-10-23" }
  - { field: trading_day_reset, url: "https://help.oneuptrader.com/article/390-permissible-products-times", accessed: "2026-10-23" }
  - { field: commission_per_contract, url: "https://help.oneuptrader.com/article/391-important-evaluation-info", accessed: "2026-10-23" }
  - { field: commission_per_contract, url: "https://help.oneuptrader.com/article/468-most-popular-questions", accessed: "2026-10-23" }
  - { field: platforms, url: "https://help.oneuptrader.com/article/380-supported-platforms", accessed: "2026-10-23" }
  - { field: platforms, url: "https://help.oneuptrader.com/article/491-ninjatrader-desktop-version-v-8-1-connection-guide", accessed: "2026-10-23" }
  - { field: copy_trading_allowed, url: "https://help.oneuptrader.com/article/505-can-i-have-multiple-evaluation-accounts", accessed: "2026-10-23" }
  - { field: copy_trading_allowed, url: "https://help.oneuptrader.com/article/372-funded-trader-rules", accessed: "2026-10-23" }
  - { field: automated_trading_allowed, url: "https://help.oneuptrader.com/article/372-funded-trader-rules", accessed: "2026-10-23" }
  - { field: vps_allowed, url: "https://help.oneuptrader.com/", accessed: "2026-10-23" }
  - { field: personal_device_required, url: "https://www.oneuptrader.com/terms", accessed: "2026-10-23" }
```

#### Pro Evaluation — evaluation — 150K
```yaml
firm: oneup-trader
plan: "Pro Evaluation"
phase: evaluation
size_label: "150K"
size_usd: "150000.00"
instruments: [MNQ, NQ]
platforms: ["NinjaTrader 8 (Desktop 8.1, via Rithmic for NinjaTrader Brokerage)", "R|Trader Pro", "R|Trader", "Sierra Chart", "ATAS", "Quantower", "Bookmap", "Jigsaw Trading", "MultiCharts", "Motive Wave", "eSignal", "Trade Navigator", "Volfix.net", "Medved Trader", "Agena Trader", "Inside Edge Trader", "Investor RT", "Photon", "QScalp", "QST", "Track'n Trade"]
drawdown: { amount_usd: "5000.00", type: trailing_intraday, stops_trailing_at: "stops trailing once the trailing drawdown reaches the initial account balance (starting balance); fixed there for the rest of the account" }
daily_loss_limit: { amount_usd: null, kind: none }
profit_target_usd: "9000.00"
consistency_rule: { text: "Three other best trading days' net profits must sum to 80% or more of the largest trading day's net profit (net after commissions); can be met within two days", threshold_pct: "80" }
max_position: { minis: 15, micros: null, scaling: "Max position size by account size, all products combined; micros count toward the limit (50K: 6 contracts at any time; micros not given a separate ratio during evaluation)" }
min_trading_days: "5"
trading_day_reset: { time: "17:00", tz_as_published: "CT" }
commission_per_contract: { mnq_usd: "1.00", nq_usd: "2.50", basis: per_side, includes: "simulated commissions and fees; exchange, routing and NFA fees are factored in" }
copy_trading_allowed: conditional
automated_trading_allowed: unknown
automation_rule_text: null
vps_allowed: unknown
personal_device_required: unknown
status: partial
not_confirmed_fields: [automated_trading_allowed, automation_rule_text, vps_allowed, personal_device_required, copy_trading_allowed, drawdown.type]
notes: "Same size/target/drawdown on the Evaluation and Pro Evaluation cards; Pro Evaluation differs only in 5 minimum trading days and price ($265/mo vs $175/mo). Evaluation: all positions flat by 3:15 PM CT; no DLL; no news restriction; trailing drawdown calculated in real time and includes simulated commissions and fees."
sources:
  - { field: drawdown, url: "https://www.oneuptrader.com/", accessed: "2026-10-23" }
  - { field: drawdown, url: "https://help.oneuptrader.com/article/389-rules-definitions", accessed: "2026-10-23" }
  - { field: daily_loss_limit, url: "https://help.oneuptrader.com/article/468-most-popular-questions", accessed: "2026-10-23" }
  - { field: daily_loss_limit, url: "https://help.oneuptrader.com/article/486-no-more-daily-loss-limit-restrictions", accessed: "2026-10-23" }
  - { field: profit_target_usd, url: "https://www.oneuptrader.com/", accessed: "2026-10-23" }
  - { field: max_position, url: "https://www.oneuptrader.com/", accessed: "2026-10-23" }
  - { field: consistency_rule, url: "https://help.oneuptrader.com/article/385-evaluation-rules-parameters", accessed: "2026-10-23" }
  - { field: consistency_rule, url: "https://help.oneuptrader.com/article/468-most-popular-questions", accessed: "2026-10-23" }
  - { field: min_trading_days, url: "https://help.oneuptrader.com/article/385-evaluation-rules-parameters", accessed: "2026-10-23" }
  - { field: trading_day_reset, url: "https://help.oneuptrader.com/article/385-evaluation-rules-parameters", accessed: "2026-10-23" }
  - { field: trading_day_reset, url: "https://help.oneuptrader.com/article/390-permissible-products-times", accessed: "2026-10-23" }
  - { field: commission_per_contract, url: "https://help.oneuptrader.com/article/391-important-evaluation-info", accessed: "2026-10-23" }
  - { field: commission_per_contract, url: "https://help.oneuptrader.com/article/468-most-popular-questions", accessed: "2026-10-23" }
  - { field: platforms, url: "https://help.oneuptrader.com/article/380-supported-platforms", accessed: "2026-10-23" }
  - { field: platforms, url: "https://help.oneuptrader.com/article/491-ninjatrader-desktop-version-v-8-1-connection-guide", accessed: "2026-10-23" }
  - { field: copy_trading_allowed, url: "https://help.oneuptrader.com/article/505-can-i-have-multiple-evaluation-accounts", accessed: "2026-10-23" }
  - { field: copy_trading_allowed, url: "https://help.oneuptrader.com/article/372-funded-trader-rules", accessed: "2026-10-23" }
  - { field: automated_trading_allowed, url: "https://help.oneuptrader.com/article/372-funded-trader-rules", accessed: "2026-10-23" }
  - { field: vps_allowed, url: "https://help.oneuptrader.com/", accessed: "2026-10-23" }
  - { field: personal_device_required, url: "https://www.oneuptrader.com/terms", accessed: "2026-10-23" }
```

#### Pro Evaluation — evaluation — 250K
```yaml
firm: oneup-trader
plan: "Pro Evaluation"
phase: evaluation
size_label: "250K"
size_usd: "250000.00"
instruments: [MNQ, NQ]
platforms: ["NinjaTrader 8 (Desktop 8.1, via Rithmic for NinjaTrader Brokerage)", "R|Trader Pro", "R|Trader", "Sierra Chart", "ATAS", "Quantower", "Bookmap", "Jigsaw Trading", "MultiCharts", "Motive Wave", "eSignal", "Trade Navigator", "Volfix.net", "Medved Trader", "Agena Trader", "Inside Edge Trader", "Investor RT", "Photon", "QScalp", "QST", "Track'n Trade"]
drawdown: { amount_usd: "5500.00", type: trailing_intraday, stops_trailing_at: "stops trailing once the trailing drawdown reaches the initial account balance (starting balance); fixed there for the rest of the account" }
daily_loss_limit: { amount_usd: null, kind: none }
profit_target_usd: "15000.00"
consistency_rule: { text: "Three other best trading days' net profits must sum to 80% or more of the largest trading day's net profit (net after commissions); can be met within two days", threshold_pct: "80" }
max_position: { minis: 25, micros: null, scaling: "Max position size by account size, all products combined; micros count toward the limit (50K: 6 contracts at any time; micros not given a separate ratio during evaluation)" }
min_trading_days: "5"
trading_day_reset: { time: "17:00", tz_as_published: "CT" }
commission_per_contract: { mnq_usd: "1.00", nq_usd: "2.50", basis: per_side, includes: "simulated commissions and fees; exchange, routing and NFA fees are factored in" }
copy_trading_allowed: conditional
automated_trading_allowed: unknown
automation_rule_text: null
vps_allowed: unknown
personal_device_required: unknown
status: partial
not_confirmed_fields: [automated_trading_allowed, automation_rule_text, vps_allowed, personal_device_required, copy_trading_allowed, drawdown.type]
notes: "Same size/target/drawdown on the Evaluation and Pro Evaluation cards; Pro Evaluation differs only in 5 minimum trading days and price ($490/mo vs $325/mo). Evaluation: all positions flat by 3:15 PM CT; no DLL; no news restriction; trailing drawdown calculated in real time and includes simulated commissions and fees."
sources:
  - { field: drawdown, url: "https://www.oneuptrader.com/", accessed: "2026-10-23" }
  - { field: drawdown, url: "https://help.oneuptrader.com/article/389-rules-definitions", accessed: "2026-10-23" }
  - { field: daily_loss_limit, url: "https://help.oneuptrader.com/article/468-most-popular-questions", accessed: "2026-10-23" }
  - { field: daily_loss_limit, url: "https://help.oneuptrader.com/article/486-no-more-daily-loss-limit-restrictions", accessed: "2026-10-23" }
  - { field: profit_target_usd, url: "https://www.oneuptrader.com/", accessed: "2026-10-23" }
  - { field: max_position, url: "https://www.oneuptrader.com/", accessed: "2026-10-23" }
  - { field: consistency_rule, url: "https://help.oneuptrader.com/article/385-evaluation-rules-parameters", accessed: "2026-10-23" }
  - { field: consistency_rule, url: "https://help.oneuptrader.com/article/468-most-popular-questions", accessed: "2026-10-23" }
  - { field: min_trading_days, url: "https://help.oneuptrader.com/article/385-evaluation-rules-parameters", accessed: "2026-10-23" }
  - { field: trading_day_reset, url: "https://help.oneuptrader.com/article/385-evaluation-rules-parameters", accessed: "2026-10-23" }
  - { field: trading_day_reset, url: "https://help.oneuptrader.com/article/390-permissible-products-times", accessed: "2026-10-23" }
  - { field: commission_per_contract, url: "https://help.oneuptrader.com/article/391-important-evaluation-info", accessed: "2026-10-23" }
  - { field: commission_per_contract, url: "https://help.oneuptrader.com/article/468-most-popular-questions", accessed: "2026-10-23" }
  - { field: platforms, url: "https://help.oneuptrader.com/article/380-supported-platforms", accessed: "2026-10-23" }
  - { field: platforms, url: "https://help.oneuptrader.com/article/491-ninjatrader-desktop-version-v-8-1-connection-guide", accessed: "2026-10-23" }
  - { field: copy_trading_allowed, url: "https://help.oneuptrader.com/article/505-can-i-have-multiple-evaluation-accounts", accessed: "2026-10-23" }
  - { field: copy_trading_allowed, url: "https://help.oneuptrader.com/article/372-funded-trader-rules", accessed: "2026-10-23" }
  - { field: automated_trading_allowed, url: "https://help.oneuptrader.com/article/372-funded-trader-rules", accessed: "2026-10-23" }
  - { field: vps_allowed, url: "https://help.oneuptrader.com/", accessed: "2026-10-23" }
  - { field: personal_device_required, url: "https://www.oneuptrader.com/terms", accessed: "2026-10-23" }
```

### Blocks: Funded Trader Account — funded

#### Funded Trader Account — funded — 25K
```yaml
firm: oneup-trader
plan: "Funded Trader Account"
phase: funded
size_label: "25K"
size_usd: "25000.00"
instruments: [MNQ, NQ]
platforms: ["NinjaTrader 8 (Desktop 8.1, via Rithmic for NinjaTrader Brokerage)", "R|Trader Pro", "R|Trader", "Sierra Chart", "ATAS", "Quantower", "Bookmap", "Jigsaw Trading", "MultiCharts", "Motive Wave", "eSignal", "Trade Navigator", "Volfix.net", "Medved Trader", "Agena Trader", "Inside Edge Trader", "Investor RT", "Photon", "QScalp", "QST", "Track'n Trade"]
drawdown: { amount_usd: "1500.00", type: trailing_intraday, stops_trailing_at: "stops trailing once the trailing drawdown reaches the initial account balance (starting balance); fixed there for the rest of the account" }
daily_loss_limit: { amount_usd: null, kind: none }
profit_target_usd: none
consistency_rule: { text: "Evaluation only; not applied to funded accounts", threshold_pct: null }
max_position: { minis: 3, micros: 30, scaling: "Dynamic Scaling Targets by Cash On Hand at session start (profit over starting balance): cash on hand 0-1500: 2 lots; cash on hand over 1501: 3 lots; micros up to 10x minis on request to the funding provider" }
min_trading_days: null
trading_day_reset: { time: "17:00", tz_as_published: "CT" }
commission_per_contract: { mnq_usd: "0.60", nq_usd: "2.04", basis: per_side, includes: "regulatory fees, trade commission, platform fees and exchange fees" }
copy_trading_allowed: conditional
automated_trading_allowed: unknown
automation_rule_text: null
vps_allowed: unknown
personal_device_required: unknown
status: partial
not_confirmed_fields: [automated_trading_allowed, automation_rule_text, vps_allowed, personal_device_required, copy_trading_allowed, drawdown.type]
notes: "Funded cards are not phase-specific on the website: drawdown amount for funded taken from the same size card (the funded drawdown article gives the mechanism but no amount). Funded: flat 1 min before/during/after listed releases; min weekly trade count 50% of evaluation average; no trades under 10 seconds; 2-strike policy (homepage); max 3 funded accounts."
sources:
  - { field: drawdown, url: "https://www.oneuptrader.com/", accessed: "2026-10-23" }
  - { field: drawdown, url: "https://help.oneuptrader.com/article/388-funded-trader-trailing-drawdown", accessed: "2026-10-23" }
  - { field: daily_loss_limit, url: "https://help.oneuptrader.com/article/468-most-popular-questions", accessed: "2026-10-23" }
  - { field: daily_loss_limit, url: "https://help.oneuptrader.com/article/486-no-more-daily-loss-limit-restrictions", accessed: "2026-10-23" }
  - { field: profit_target_usd, url: "https://www.oneuptrader.com/", accessed: "2026-10-23" }
  - { field: max_position, url: "https://help.oneuptrader.com/article/386-funded-trader-dynamic-scaling-targets", accessed: "2026-10-23" }
  - { field: consistency_rule, url: "https://help.oneuptrader.com/article/385-evaluation-rules-parameters", accessed: "2026-10-23" }
  - { field: consistency_rule, url: "https://help.oneuptrader.com/article/468-most-popular-questions", accessed: "2026-10-23" }
  - { field: min_trading_days, url: "https://help.oneuptrader.com/article/385-evaluation-rules-parameters", accessed: "2026-10-23" }
  - { field: trading_day_reset, url: "https://help.oneuptrader.com/article/385-evaluation-rules-parameters", accessed: "2026-10-23" }
  - { field: trading_day_reset, url: "https://help.oneuptrader.com/article/390-permissible-products-times", accessed: "2026-10-23" }
  - { field: commission_per_contract, url: "https://help.oneuptrader.com/article/361-funded-trader-commissions", accessed: "2026-10-23" }
  - { field: commission_per_contract, url: "https://help.oneuptrader.com/article/468-most-popular-questions", accessed: "2026-10-23" }
  - { field: platforms, url: "https://help.oneuptrader.com/article/380-supported-platforms", accessed: "2026-10-23" }
  - { field: platforms, url: "https://help.oneuptrader.com/article/491-ninjatrader-desktop-version-v-8-1-connection-guide", accessed: "2026-10-23" }
  - { field: copy_trading_allowed, url: "https://help.oneuptrader.com/article/505-can-i-have-multiple-evaluation-accounts", accessed: "2026-10-23" }
  - { field: copy_trading_allowed, url: "https://help.oneuptrader.com/article/372-funded-trader-rules", accessed: "2026-10-23" }
  - { field: automated_trading_allowed, url: "https://help.oneuptrader.com/article/372-funded-trader-rules", accessed: "2026-10-23" }
  - { field: vps_allowed, url: "https://help.oneuptrader.com/", accessed: "2026-10-23" }
  - { field: personal_device_required, url: "https://www.oneuptrader.com/terms", accessed: "2026-10-23" }
```

#### Funded Trader Account — funded — 50K
```yaml
firm: oneup-trader
plan: "Funded Trader Account"
phase: funded
size_label: "50K"
size_usd: "50000.00"
instruments: [MNQ, NQ]
platforms: ["NinjaTrader 8 (Desktop 8.1, via Rithmic for NinjaTrader Brokerage)", "R|Trader Pro", "R|Trader", "Sierra Chart", "ATAS", "Quantower", "Bookmap", "Jigsaw Trading", "MultiCharts", "Motive Wave", "eSignal", "Trade Navigator", "Volfix.net", "Medved Trader", "Agena Trader", "Inside Edge Trader", "Investor RT", "Photon", "QScalp", "QST", "Track'n Trade"]
drawdown: { amount_usd: "2500.00", type: trailing_intraday, stops_trailing_at: "stops trailing once the trailing drawdown reaches the initial account balance (starting balance); fixed there for the rest of the account" }
daily_loss_limit: { amount_usd: null, kind: none }
profit_target_usd: none
consistency_rule: { text: "Evaluation only; not applied to funded accounts", threshold_pct: null }
max_position: { minis: 6, micros: 60, scaling: "Dynamic Scaling Targets by Cash On Hand at session start (profit over starting balance): cash on hand 0-1500: 2 lots; cash on hand 1501-4000: 4 lots; cash on hand over 4001: 6 lots; micros up to 10x minis on request to the funding provider" }
min_trading_days: null
trading_day_reset: { time: "17:00", tz_as_published: "CT" }
commission_per_contract: { mnq_usd: "0.60", nq_usd: "2.04", basis: per_side, includes: "regulatory fees, trade commission, platform fees and exchange fees" }
copy_trading_allowed: conditional
automated_trading_allowed: unknown
automation_rule_text: null
vps_allowed: unknown
personal_device_required: unknown
status: partial
not_confirmed_fields: [automated_trading_allowed, automation_rule_text, vps_allowed, personal_device_required, copy_trading_allowed, drawdown.type]
notes: "Funded cards are not phase-specific on the website: drawdown amount for funded taken from the same size card (the funded drawdown article gives the mechanism but no amount). Funded: flat 1 min before/during/after listed releases; min weekly trade count 50% of evaluation average; no trades under 10 seconds; 2-strike policy (homepage); max 3 funded accounts."
sources:
  - { field: drawdown, url: "https://www.oneuptrader.com/", accessed: "2026-10-23" }
  - { field: drawdown, url: "https://help.oneuptrader.com/article/388-funded-trader-trailing-drawdown", accessed: "2026-10-23" }
  - { field: daily_loss_limit, url: "https://help.oneuptrader.com/article/468-most-popular-questions", accessed: "2026-10-23" }
  - { field: daily_loss_limit, url: "https://help.oneuptrader.com/article/486-no-more-daily-loss-limit-restrictions", accessed: "2026-10-23" }
  - { field: profit_target_usd, url: "https://www.oneuptrader.com/", accessed: "2026-10-23" }
  - { field: max_position, url: "https://help.oneuptrader.com/article/386-funded-trader-dynamic-scaling-targets", accessed: "2026-10-23" }
  - { field: consistency_rule, url: "https://help.oneuptrader.com/article/385-evaluation-rules-parameters", accessed: "2026-10-23" }
  - { field: consistency_rule, url: "https://help.oneuptrader.com/article/468-most-popular-questions", accessed: "2026-10-23" }
  - { field: min_trading_days, url: "https://help.oneuptrader.com/article/385-evaluation-rules-parameters", accessed: "2026-10-23" }
  - { field: trading_day_reset, url: "https://help.oneuptrader.com/article/385-evaluation-rules-parameters", accessed: "2026-10-23" }
  - { field: trading_day_reset, url: "https://help.oneuptrader.com/article/390-permissible-products-times", accessed: "2026-10-23" }
  - { field: commission_per_contract, url: "https://help.oneuptrader.com/article/361-funded-trader-commissions", accessed: "2026-10-23" }
  - { field: commission_per_contract, url: "https://help.oneuptrader.com/article/468-most-popular-questions", accessed: "2026-10-23" }
  - { field: platforms, url: "https://help.oneuptrader.com/article/380-supported-platforms", accessed: "2026-10-23" }
  - { field: platforms, url: "https://help.oneuptrader.com/article/491-ninjatrader-desktop-version-v-8-1-connection-guide", accessed: "2026-10-23" }
  - { field: copy_trading_allowed, url: "https://help.oneuptrader.com/article/505-can-i-have-multiple-evaluation-accounts", accessed: "2026-10-23" }
  - { field: copy_trading_allowed, url: "https://help.oneuptrader.com/article/372-funded-trader-rules", accessed: "2026-10-23" }
  - { field: automated_trading_allowed, url: "https://help.oneuptrader.com/article/372-funded-trader-rules", accessed: "2026-10-23" }
  - { field: vps_allowed, url: "https://help.oneuptrader.com/", accessed: "2026-10-23" }
  - { field: personal_device_required, url: "https://www.oneuptrader.com/terms", accessed: "2026-10-23" }
```

#### Funded Trader Account — funded — 100K
```yaml
firm: oneup-trader
plan: "Funded Trader Account"
phase: funded
size_label: "100K"
size_usd: "100000.00"
instruments: [MNQ, NQ]
platforms: ["NinjaTrader 8 (Desktop 8.1, via Rithmic for NinjaTrader Brokerage)", "R|Trader Pro", "R|Trader", "Sierra Chart", "ATAS", "Quantower", "Bookmap", "Jigsaw Trading", "MultiCharts", "Motive Wave", "eSignal", "Trade Navigator", "Volfix.net", "Medved Trader", "Agena Trader", "Inside Edge Trader", "Investor RT", "Photon", "QScalp", "QST", "Track'n Trade"]
drawdown: { amount_usd: "3500.00", type: trailing_intraday, stops_trailing_at: "stops trailing once the trailing drawdown reaches the initial account balance (starting balance); fixed there for the rest of the account" }
daily_loss_limit: { amount_usd: null, kind: none }
profit_target_usd: none
consistency_rule: { text: "Evaluation only; not applied to funded accounts", threshold_pct: null }
max_position: { minis: 12, micros: 120, scaling: "Dynamic Scaling Targets by Cash On Hand at session start (profit over starting balance): cash on hand 0-2000: 3 lots; cash on hand 2001-3000: 4 lots; cash on hand 3001-5000: 6 lots; cash on hand over 5001: 12 lots; micros up to 10x minis on request to the funding provider" }
min_trading_days: null
trading_day_reset: { time: "17:00", tz_as_published: "CT" }
commission_per_contract: { mnq_usd: "0.60", nq_usd: "2.04", basis: per_side, includes: "regulatory fees, trade commission, platform fees and exchange fees" }
copy_trading_allowed: conditional
automated_trading_allowed: unknown
automation_rule_text: null
vps_allowed: unknown
personal_device_required: unknown
status: partial
not_confirmed_fields: [automated_trading_allowed, automation_rule_text, vps_allowed, personal_device_required, copy_trading_allowed, drawdown.type]
notes: "Funded cards are not phase-specific on the website: drawdown amount for funded taken from the same size card (the funded drawdown article gives the mechanism but no amount). Funded: flat 1 min before/during/after listed releases; min weekly trade count 50% of evaluation average; no trades under 10 seconds; 2-strike policy (homepage); max 3 funded accounts."
sources:
  - { field: drawdown, url: "https://www.oneuptrader.com/", accessed: "2026-10-23" }
  - { field: drawdown, url: "https://help.oneuptrader.com/article/388-funded-trader-trailing-drawdown", accessed: "2026-10-23" }
  - { field: daily_loss_limit, url: "https://help.oneuptrader.com/article/468-most-popular-questions", accessed: "2026-10-23" }
  - { field: daily_loss_limit, url: "https://help.oneuptrader.com/article/486-no-more-daily-loss-limit-restrictions", accessed: "2026-10-23" }
  - { field: profit_target_usd, url: "https://www.oneuptrader.com/", accessed: "2026-10-23" }
  - { field: max_position, url: "https://help.oneuptrader.com/article/386-funded-trader-dynamic-scaling-targets", accessed: "2026-10-23" }
  - { field: consistency_rule, url: "https://help.oneuptrader.com/article/385-evaluation-rules-parameters", accessed: "2026-10-23" }
  - { field: consistency_rule, url: "https://help.oneuptrader.com/article/468-most-popular-questions", accessed: "2026-10-23" }
  - { field: min_trading_days, url: "https://help.oneuptrader.com/article/385-evaluation-rules-parameters", accessed: "2026-10-23" }
  - { field: trading_day_reset, url: "https://help.oneuptrader.com/article/385-evaluation-rules-parameters", accessed: "2026-10-23" }
  - { field: trading_day_reset, url: "https://help.oneuptrader.com/article/390-permissible-products-times", accessed: "2026-10-23" }
  - { field: commission_per_contract, url: "https://help.oneuptrader.com/article/361-funded-trader-commissions", accessed: "2026-10-23" }
  - { field: commission_per_contract, url: "https://help.oneuptrader.com/article/468-most-popular-questions", accessed: "2026-10-23" }
  - { field: platforms, url: "https://help.oneuptrader.com/article/380-supported-platforms", accessed: "2026-10-23" }
  - { field: platforms, url: "https://help.oneuptrader.com/article/491-ninjatrader-desktop-version-v-8-1-connection-guide", accessed: "2026-10-23" }
  - { field: copy_trading_allowed, url: "https://help.oneuptrader.com/article/505-can-i-have-multiple-evaluation-accounts", accessed: "2026-10-23" }
  - { field: copy_trading_allowed, url: "https://help.oneuptrader.com/article/372-funded-trader-rules", accessed: "2026-10-23" }
  - { field: automated_trading_allowed, url: "https://help.oneuptrader.com/article/372-funded-trader-rules", accessed: "2026-10-23" }
  - { field: vps_allowed, url: "https://help.oneuptrader.com/", accessed: "2026-10-23" }
  - { field: personal_device_required, url: "https://www.oneuptrader.com/terms", accessed: "2026-10-23" }
```

#### Funded Trader Account — funded — 150K
```yaml
firm: oneup-trader
plan: "Funded Trader Account"
phase: funded
size_label: "150K"
size_usd: "150000.00"
instruments: [MNQ, NQ]
platforms: ["NinjaTrader 8 (Desktop 8.1, via Rithmic for NinjaTrader Brokerage)", "R|Trader Pro", "R|Trader", "Sierra Chart", "ATAS", "Quantower", "Bookmap", "Jigsaw Trading", "MultiCharts", "Motive Wave", "eSignal", "Trade Navigator", "Volfix.net", "Medved Trader", "Agena Trader", "Inside Edge Trader", "Investor RT", "Photon", "QScalp", "QST", "Track'n Trade"]
drawdown: { amount_usd: "5000.00", type: trailing_intraday, stops_trailing_at: "stops trailing once the trailing drawdown reaches the initial account balance (starting balance); fixed there for the rest of the account" }
daily_loss_limit: { amount_usd: null, kind: none }
profit_target_usd: none
consistency_rule: { text: "Evaluation only; not applied to funded accounts", threshold_pct: null }
max_position: { minis: 15, micros: 150, scaling: "Dynamic Scaling Targets by Cash On Hand at session start (profit over starting balance): cash on hand 0-4000: 5 lots; cash on hand 4001-8000: 8 lots; cash on hand 8001-12000: 10 lots; cash on hand over 12001: 15 lots; micros up to 10x minis on request to the funding provider" }
min_trading_days: null
trading_day_reset: { time: "17:00", tz_as_published: "CT" }
commission_per_contract: { mnq_usd: "0.60", nq_usd: "2.04", basis: per_side, includes: "regulatory fees, trade commission, platform fees and exchange fees" }
copy_trading_allowed: conditional
automated_trading_allowed: unknown
automation_rule_text: null
vps_allowed: unknown
personal_device_required: unknown
status: partial
not_confirmed_fields: [automated_trading_allowed, automation_rule_text, vps_allowed, personal_device_required, copy_trading_allowed, drawdown.type]
notes: "Funded cards are not phase-specific on the website: drawdown amount for funded taken from the same size card (the funded drawdown article gives the mechanism but no amount). Funded: flat 1 min before/during/after listed releases; min weekly trade count 50% of evaluation average; no trades under 10 seconds; 2-strike policy (homepage); max 3 funded accounts."
sources:
  - { field: drawdown, url: "https://www.oneuptrader.com/", accessed: "2026-10-23" }
  - { field: drawdown, url: "https://help.oneuptrader.com/article/388-funded-trader-trailing-drawdown", accessed: "2026-10-23" }
  - { field: daily_loss_limit, url: "https://help.oneuptrader.com/article/468-most-popular-questions", accessed: "2026-10-23" }
  - { field: daily_loss_limit, url: "https://help.oneuptrader.com/article/486-no-more-daily-loss-limit-restrictions", accessed: "2026-10-23" }
  - { field: profit_target_usd, url: "https://www.oneuptrader.com/", accessed: "2026-10-23" }
  - { field: max_position, url: "https://help.oneuptrader.com/article/386-funded-trader-dynamic-scaling-targets", accessed: "2026-10-23" }
  - { field: consistency_rule, url: "https://help.oneuptrader.com/article/385-evaluation-rules-parameters", accessed: "2026-10-23" }
  - { field: consistency_rule, url: "https://help.oneuptrader.com/article/468-most-popular-questions", accessed: "2026-10-23" }
  - { field: min_trading_days, url: "https://help.oneuptrader.com/article/385-evaluation-rules-parameters", accessed: "2026-10-23" }
  - { field: trading_day_reset, url: "https://help.oneuptrader.com/article/385-evaluation-rules-parameters", accessed: "2026-10-23" }
  - { field: trading_day_reset, url: "https://help.oneuptrader.com/article/390-permissible-products-times", accessed: "2026-10-23" }
  - { field: commission_per_contract, url: "https://help.oneuptrader.com/article/361-funded-trader-commissions", accessed: "2026-10-23" }
  - { field: commission_per_contract, url: "https://help.oneuptrader.com/article/468-most-popular-questions", accessed: "2026-10-23" }
  - { field: platforms, url: "https://help.oneuptrader.com/article/380-supported-platforms", accessed: "2026-10-23" }
  - { field: platforms, url: "https://help.oneuptrader.com/article/491-ninjatrader-desktop-version-v-8-1-connection-guide", accessed: "2026-10-23" }
  - { field: copy_trading_allowed, url: "https://help.oneuptrader.com/article/505-can-i-have-multiple-evaluation-accounts", accessed: "2026-10-23" }
  - { field: copy_trading_allowed, url: "https://help.oneuptrader.com/article/372-funded-trader-rules", accessed: "2026-10-23" }
  - { field: automated_trading_allowed, url: "https://help.oneuptrader.com/article/372-funded-trader-rules", accessed: "2026-10-23" }
  - { field: vps_allowed, url: "https://help.oneuptrader.com/", accessed: "2026-10-23" }
  - { field: personal_device_required, url: "https://www.oneuptrader.com/terms", accessed: "2026-10-23" }
```

#### Funded Trader Account — funded — 250K
```yaml
firm: oneup-trader
plan: "Funded Trader Account"
phase: funded
size_label: "250K"
size_usd: "250000.00"
instruments: [MNQ, NQ]
platforms: ["NinjaTrader 8 (Desktop 8.1, via Rithmic for NinjaTrader Brokerage)", "R|Trader Pro", "R|Trader", "Sierra Chart", "ATAS", "Quantower", "Bookmap", "Jigsaw Trading", "MultiCharts", "Motive Wave", "eSignal", "Trade Navigator", "Volfix.net", "Medved Trader", "Agena Trader", "Inside Edge Trader", "Investor RT", "Photon", "QScalp", "QST", "Track'n Trade"]
drawdown: { amount_usd: "5500.00", type: trailing_intraday, stops_trailing_at: "stops trailing once the trailing drawdown reaches the initial account balance (starting balance); fixed there for the rest of the account" }
daily_loss_limit: { amount_usd: null, kind: none }
profit_target_usd: none
consistency_rule: { text: "Evaluation only; not applied to funded accounts", threshold_pct: null }
max_position: { minis: 25, micros: 250, scaling: "Dynamic Scaling Targets by Cash On Hand at session start (profit over starting balance): cash on hand 0-5000: 6 lots; cash on hand 5001-12000: 12 lots; cash on hand 12001-20000: 18 lots; cash on hand over 20001: 25 lots; micros up to 10x minis on request to the funding provider" }
min_trading_days: null
trading_day_reset: { time: "17:00", tz_as_published: "CT" }
commission_per_contract: { mnq_usd: "0.60", nq_usd: "2.04", basis: per_side, includes: "regulatory fees, trade commission, platform fees and exchange fees" }
copy_trading_allowed: conditional
automated_trading_allowed: unknown
automation_rule_text: null
vps_allowed: unknown
personal_device_required: unknown
status: partial
not_confirmed_fields: [automated_trading_allowed, automation_rule_text, vps_allowed, personal_device_required, copy_trading_allowed, drawdown.type]
notes: "Funded cards are not phase-specific on the website: drawdown amount for funded taken from the same size card (the funded drawdown article gives the mechanism but no amount). Funded: flat 1 min before/during/after listed releases; min weekly trade count 50% of evaluation average; no trades under 10 seconds; 2-strike policy (homepage); max 3 funded accounts."
sources:
  - { field: drawdown, url: "https://www.oneuptrader.com/", accessed: "2026-10-23" }
  - { field: drawdown, url: "https://help.oneuptrader.com/article/388-funded-trader-trailing-drawdown", accessed: "2026-10-23" }
  - { field: daily_loss_limit, url: "https://help.oneuptrader.com/article/468-most-popular-questions", accessed: "2026-10-23" }
  - { field: daily_loss_limit, url: "https://help.oneuptrader.com/article/486-no-more-daily-loss-limit-restrictions", accessed: "2026-10-23" }
  - { field: profit_target_usd, url: "https://www.oneuptrader.com/", accessed: "2026-10-23" }
  - { field: max_position, url: "https://help.oneuptrader.com/article/386-funded-trader-dynamic-scaling-targets", accessed: "2026-10-23" }
  - { field: consistency_rule, url: "https://help.oneuptrader.com/article/385-evaluation-rules-parameters", accessed: "2026-10-23" }
  - { field: consistency_rule, url: "https://help.oneuptrader.com/article/468-most-popular-questions", accessed: "2026-10-23" }
  - { field: min_trading_days, url: "https://help.oneuptrader.com/article/385-evaluation-rules-parameters", accessed: "2026-10-23" }
  - { field: trading_day_reset, url: "https://help.oneuptrader.com/article/385-evaluation-rules-parameters", accessed: "2026-10-23" }
  - { field: trading_day_reset, url: "https://help.oneuptrader.com/article/390-permissible-products-times", accessed: "2026-10-23" }
  - { field: commission_per_contract, url: "https://help.oneuptrader.com/article/361-funded-trader-commissions", accessed: "2026-10-23" }
  - { field: commission_per_contract, url: "https://help.oneuptrader.com/article/468-most-popular-questions", accessed: "2026-10-23" }
  - { field: platforms, url: "https://help.oneuptrader.com/article/380-supported-platforms", accessed: "2026-10-23" }
  - { field: platforms, url: "https://help.oneuptrader.com/article/491-ninjatrader-desktop-version-v-8-1-connection-guide", accessed: "2026-10-23" }
  - { field: copy_trading_allowed, url: "https://help.oneuptrader.com/article/505-can-i-have-multiple-evaluation-accounts", accessed: "2026-10-23" }
  - { field: copy_trading_allowed, url: "https://help.oneuptrader.com/article/372-funded-trader-rules", accessed: "2026-10-23" }
  - { field: automated_trading_allowed, url: "https://help.oneuptrader.com/article/372-funded-trader-rules", accessed: "2026-10-23" }
  - { field: vps_allowed, url: "https://help.oneuptrader.com/", accessed: "2026-10-23" }
  - { field: personal_device_required, url: "https://www.oneuptrader.com/terms", accessed: "2026-10-23" }
```


## 10. Unverified / gaps / conflicts

| Field | Status | Detail |
|---|---|---|
| automated_trading_allowed, automation_rule_text | unknown | No official page found states a rule on automated trading, algos, bots, webhooks or EAs. Gap: ask the firm (support@oneuptrader.com) before enabling the plugin automation path |
| vps_allowed | unknown | Not published (VPS/VPN/remote server not mentioned) |
| personal_device_required | unknown | Not published; Terms only say the account "will be personal to you, and you cannot share it with anybody else." |
| copy_trading_allowed (to/from other firms) | unknown | Not mentioned |
| copy_trading (funded vs evaluation) | conditional | Funded text in S5; evaluation only S13 note that a copier may be used |
| conflict: balance resets | conflict | Homepage plan cards (H): "Unlimited Balance Resets". Help center (S11, S23, S20): resets cost "$50 for 25k and 50k accounts / $100 for all other account sizes". Not picked; reset cost is a user-facing figure only, no routing effect |
| conflict: Express minimum days | conflict (low) | S22 and S3: 3 trading days; S26 (release note 2023-03-09): "get funded in just 5 days". Homepage: "Get funded in only 3 days with an Express Account". Likely stale note |
| drawdown amount, funded | partial | Funded article S4 has no amount; amounts are from the same-size website card (H), which is not phase-specific |
| drawdown unrealized equity | unknown | Published as "Account Balance" with real-time calculation; unrealized treatment not stated; type set `trailing_intraday` as published ("calculated in Real-Time") |
| profit targets | verified (homepage only) | Not repeated in any help article; help says only "based on the account you choose" |
| funded scaling tiers and funded commission table | image-sourced | S6 and S12 are images; read visually; NQ funded commission $2.04/side and MNQ $0.60/side match the S9 text only for MNQ ($1.20 RT) |
| platform list | minor conflict | S14 (Medved Trader, no ScalpTool) vs S15 (ScalpTool, R Investor, no Medved Trader); S14 used as current |
| permitted products | minor | S1 and S2 lists omit PA (palladium) that S7 and H list; not relevant for MNQ/NQ |
| trading-day reset | partial | "5 PM CT" published only as the start of the trading day; there is no daily reset event of any limit; DST not described |
| funded NinjaTrader connection details | unknown | No funded connection guide; system name for funded accounts not published |
| 2-strike policy | unknown | Mentioned on H only; no detail |
| Express prices | unverified | Image on S22 not read; low priority |
| pre-funded "live" tier | none | Not published |

**Pages not reached:** `https://app.oneuptrader.com` (login), `https://blog.oneuptrader.com`, funding-partner contracts (issued after passing; behind login/email: `unverified (behind login)`), Trader Contract text.

## 11. Changelog

- 2026-10-23 — initial research (Lupa)
- 2026-10-23 — rev bump R4 (question 1 only): personal-device requirement re-checked in 60 help articles, terms and homepage; still not published, stays unknown; no YAML change (Lupa)
