# Earn2Trade — Prop Firm Rules Research

| | |
|---|---|
| **Firm** | Earn2Trade (E2T). An education and evaluation company, not a funding firm: "Earn2Trade is an educational company headquartered in Sheridan, Wyoming. It provides trading education and simulated evaluation programs that connect successful candidates with proprietary trading firms for funding." (S31). Funded accounts are issued by partner firms (Helios Trading Partners, Appius Trading Limited, Kronos Proprietary Trading, S32) |
| **Official domains** | `www.earn2trade.com` (main site; plain curl works) · `help.earn2trade.com` (help center, Intercom; linked from the site header "Help", `https://help.earn2trade.com/en/`) · `blog.earn2trade.com` (not used). Dashboard (login) not used |
| **Research date** | 2026-10-23 (all URLs below accessed 2026-10-23) |
| **Researcher** | Lupa (Prop Firm Rules Researcher) |
| **File status** | `partial`. Drawdown, daily loss limit amounts, profit targets, consistency, max position (Progression Ladder), trading day, copy-trading and the prohibited-conduct text are verified for every current plan and size. Still open: MNQ commission (not published), the daily-loss-limit behavior on funded accounts, any explicit automation rule, and any VPS / personal-device rule (none published). Some values come from images or from the site's own JS data (see §10) |
| **Instruments covered** | MNQ, NQ: "All CME (CME, CBOT, NYMEX, COMEX) Futures assets can be traded. This includes the Micros." (S33). Prohibited: Forex, Bitcoin futures, Ether futures (S33, S29) |
| **Platforms (for routing)** | Chosen at purchase: Rithmic (Finamark, R\|Trader Pro and more), BlackArrow, NinjaTrader (via the NinjaTrader-Tradovate unified API; includes Tradovate and TradingView), dxFeed/DeepCharts. **NinjaTrader 8 desktop is offered and free.** See §2 |

Evidence (raw text of all 94 English help articles, the product pages, the terms, the site JS data chunk and the commission/drawdown/ladder images): `/tmp/claude-1003/-srv-www-allwin--maestri-roles-6bf77a80-a1ed-43dc-bc00-08a480f178a5/c3ed71e7-3dd4-4f1a-8a03-9bc3a3fa7b49/scratchpad/r3/earn2trade/`. Temporary scratch storage, so the evidence must be captured again at re-verification.

**Programs found (current, purchasable).** **Trader Career Path (TCP)**: evaluation sizes TCP25 ($25,000), TCP50 ($50,000), TCP100 ($100,000), then a growth plan of funded accounts up to $200,000 or $400,000. **The Gauntlet Mini (GAU)**: GAU50, GAU100, GAU150, GAU200, a single evaluation then a funded account of the same size, no growth plan. Funded accounts come in two kinds: **LiveSim** (simulated, live data, EOD drawdown) and **Live** (real brokerage account at a partner FCM, trailing drawdown). The firm sells education too (Beginner Crash Course), not relevant to routing. There is no 300K size. $250K does not exist; $400K exists only as a Live growth-plan step.

**Terms used by the firm.** "EOD Drawdown" = end-of-day drawdown. "Progression Ladder" = maximum position size by profit level. "Maintain Consistency" = the consistency rule. "LiveSim®" = funded simulated account. All contract counts are mini-equivalents: "Up to ten micro contracts (MES, MNQ, MYM, M2K, MGC, MCL, M6B, M6E, MHG) count as only one standard contract toward this limit." (S12).

## 1. Source index (all accessed 2026-10-23)

| Key | URL |
|---|---|
| S1 | https://www.earn2trade.com/trader-career-path (plan card values; JS-rendered, default tab TCP25; text captured with curl, the page text only shows TCP25 and the growth plan) |
| S2 | https://www.earn2trade.com/gauntlet-mini (default tab GAU50) |
| S3 | https://www.earn2trade.com/_next/static/chunks/0d45t7274e6.p.js (the product pages' own JS data array for all sizes; chunk file name is build-specific) |
| S4 | https://help.earn2trade.com/en/articles/3224507-what-is-the-maximum-drawdown-on-the-trader-career-path-and-the-gauntlet-mini-challenges (drawdown tables are images) |
| S5 | https://help.earn2trade.com/en/articles/5372687-how-does-end-of-day-drawdown-work |
| S6 | https://help.earn2trade.com/en/articles/3292356-what-is-a-trailing-drawdown |
| S7 | https://help.earn2trade.com/en/articles/8117088-drawdown-types |
| S8 | https://help.earn2trade.com/en/articles/8129235-what-is-a-fixed-drawdown |
| S9 | https://help.earn2trade.com/en/articles/3395926-how-is-my-daily-loss-calculated |
| S10 | https://help.earn2trade.com/en/articles/3849975-what-is-the-maintain-consistency-rule |
| S11 | https://help.earn2trade.com/en/articles/5941958-what-are-the-evaluation-rules |
| S12 | https://help.earn2trade.com/en/articles/3292351-what-is-the-progression-ladder-in-the-gauntlet-mini (ladder table is an image) |
| S13 | https://help.earn2trade.com/en/articles/5922873-what-is-the-progression-ladder-on-the-trader-career-path (ladder table is an image) |
| S14 | https://help.earn2trade.com/en/articles/3224526-are-there-any-restrictions-on-when-i-can-trade-in-my-evaluation-account |
| S15 | https://help.earn2trade.com/en/articles/2090496-can-i-hold-positions-overnight-on-the-trader-career-path-gauntlet-mini |
| S16 | https://help.earn2trade.com/en/articles/2425493-what-is-the-commission-on-earn2trade-evaluations (commission charts are images) |
| S17 | https://help.earn2trade.com/en/articles/2280137-what-is-the-fee-structure-on-the-live-or-livesim-accounts |
| S18 | https://help.earn2trade.com/en/articles/12034590-am-i-allowed-to-copy-trades-across-multiple-accounts |
| S19 | https://help.earn2trade.com/en/articles/5660666-can-i-have-multiple-trader-career-path-gauntlet-mini-and-live-livesim-accounts |
| S20 | https://help.earn2trade.com/en/articles/9286647-prohibited-conduct-in-earn2trade-evaluations-and-in-the-livesim-live-trading-environment |
| S21 | https://www.earn2trade.com/terms-and-conditions (no "last updated" date on the page) |
| S22 | https://help.earn2trade.com/en/articles/2090521-what-platforms-can-i-use-for-the-gauntlet-mini-trader-career-path |
| S23 | https://help.earn2trade.com/en/articles/15359154-ninjatrader-access-at-earn2trade |
| S24 | https://help.earn2trade.com/en/articles/13460913-ninjatrader-live-account-access |
| S25 | https://help.earn2trade.com/en/articles/6863807-what-is-the-difference-between-the-tcp25-tcp50-and-tcp100 |
| S26 | https://help.earn2trade.com/en/articles/6877364-what-happens-when-i-complete-my-evaluation |
| S27 | https://help.earn2trade.com/en/articles/3313030-what-is-the-livesim |
| S28 | https://help.earn2trade.com/en/articles/5374680-what-are-the-differences-between-the-trader-career-path-gauntlet-mini-and-the-livesim-live-accounts (table is an image) |
| S29 | https://help.earn2trade.com/en/articles/3320460-what-assets-can-i-trade |
| S30 | https://help.earn2trade.com/en/articles/2090520-what-if-i-lose-trades-because-of-platform-connectivity-issues |
| S31 | https://help.earn2trade.com/en/articles/16295637-what-is-earn2trade |
| S32 | https://help.earn2trade.com/en/articles/1585409-which-proprietary-trading-firm-will-contact-me-after-i-pass-my-evaluation |
| S33 | https://help.earn2trade.com/en/articles/3320460-what-assets-can-i-trade (same page as S29, kept for the quote above) |
| S34 | https://help.earn2trade.com/en/articles/14196947-how-do-i-set-up-my-tradovate-evaluation |
| S35 | https://help.earn2trade.com/en/articles/4473826-how-to-connect-tradovate-to-ninjatrader |
| S36 | https://help.earn2trade.com/en/articles/6911655-how-do-i-set-up-my-rithmic-evaluation |
| S37 | https://help.earn2trade.com/en/articles/8572729-can-i-use-my-own-data-feed |
| S38 | https://help.earn2trade.com/en/articles/1518650-during-which-holidays-are-the-markets-closed |
| S39 | https://help.earn2trade.com/en/articles/5452472-what-is-the-withdrawal-policy |
| S40 | https://help.earn2trade.com/en/articles/1795200-what-costs-am-i-responsible-for-once-i-become-funded-with-a-live-livesim-account |
| S41 | https://help.earn2trade.com/en/articles/2402853-do-you-provide-tech-support-for-my-platform |
| S42 | https://help.earn2trade.com/en/articles/14284964-how-to-connect-tradovate-to-tradingview |
| S43 | https://help.earn2trade.com/en/articles/7986692-which-brokerage-s-provide-live-trading-on-the-cme |
| S44 | https://help.earn2trade.com/en/articles/5009408-how-can-i-close-my-positions |
| S45 | https://help.earn2trade.com/en/articles/1541075-what-are-the-recommended-computer-specifications-i-should-have-to-trade-on-gauntlet-mini-trader-career-path-or-the-simulator |
| S46 | https://help.earn2trade.com/en/articles/6863801-do-you-provide-free-resets and https://help.earn2trade.com/en/articles/11183609-reset-pricing-for-earn2trade-accounts |

The help center (7 collections, 94 English articles in the sitemap; the five rule/platform/funding/general/evaluation collection pages list no article outside the sitemap) was crawled in full. Searched keywords: drawdown, daily loss, profit target, consistency, contracts, scaling, reset, session, copy, copier, automat, bot, algo, webhook, TradingView, VPS, VPN, remote, server, device, IP, commission, per side, platform, Rithmic, Tradovate, NinjaTrader, news, overnight, hedging, inactivity, max accounts.

## 2. Platforms

Per plan/phase, as published.

- **Evaluation (TCP and GAU), chosen at purchase** (platform labels in the purchase flow, S1/S2 page data): "Rithmic" ("Finamark, R Trader Pro and more"), "BlackArrow" ("Powerful, fast and accurate"), "NinjaTrader" ("Tradovate and TradingView"), "dxFeed/Deepcharts" ("DeepCharts (free), TradingView, ATAS, Quantower"). The evaluation is a simulated account fed by the firm's data feed provider: "In the E2T evaluation stage you will have to use the data feed provider offered by E2T (currently we work with Rithmic)." (S37, an older article; S34 and S35 show a Tradovate data feed with the "simulation" mode for newer subscriptions).
- **Free platforms** (S22): "NinjaTrader® - Free during the Trader Career Path®/ Gauntlet Mini™.", Finamark, R\|Trader and R\|Trader Pro, Tradovate, TradingView, BlackArrow One, DeepCharts, Quantower. Other listed (user-supplied license): Inside Edge Trader, Investor RT, Motive Wave, MultiCharts, Bookmap, Photon, QScalp, QSI, ScalpTool, Trade Navigator, Volfix.net, Jigsaw Trading, ATAS, Sierra Chart. The page says "If you don't see your platform of choice, please contact our support".
- **NinjaTrader access changed (S23).** "going forward, NinjaTrader will be available through the NinjaTrader–Tradovate unified API, not through the legacy Rithmic connection. Customers who want to use NinjaTrader with a new subscription must select the NinjaTrader–Tradovate option during signup." "Legacy NinjaTrader accounts using a Rithmic connection will continue to be honored up until June 30th, 2026" (that date has passed on the research date). "Can I still use Rithmic with NinjaTrader on a new subscription? Yes, but only if you have your own personal NinjaTrader license that has a Rithmic integration."
- **How NinjaTrader connects to the evaluation (S35):** NinjaTrader Desktop logs in with the Tradovate credentials from the "Tradovate Data Credentials" email, "pick simulation" (or in Multi-Provider mode: Connections > Configure > NinjaTrader, Account Type "simulated"). TradingView connects to the same Tradovate account (S42).
- **LiveSim:** "The simulated environment is populated by a live data feed, and can be traded on using a wide variety of trading platforms." (S27).
- **Live:** "After passing your evaluation, you may be eligible to trade a live account using the NinjaTrader® desktop platform. The live account is created by the prop firm team, and once it is ready, you will receive your live trading credentials directly from them." "NinjaTrader® connects directly to live accounts, and the former leader–follower system is no longer used." (S24). Rithmic Live (R|Trader Pro and compatible platforms) is also published (data fees "$140.00/month/exchange for Rithmic users", S17). Live accounts are housed at partner brokerages (EdgeClear, Dorman Trading, NinjaTrader, Lunaro Markets, PhillipCapital legacy; S43).
- **Order storage warning (S30):** "When using the ATM Strategy feature in NinjaTrader, orders are stored locally on your own computer."
- **No tech support for platforms (S41):** "Earn2Trade does not provide tech support to any of the platforms that you may choose to trade with."

## 3. Plans

All money values in USD. "EOD drawdown" = the minimum balance rises with each day's closing balance and stops when it equals the starting balance; open-equity losses still count intraday. "All rules, including trailing, use closed and open equity intraday." (S6) is the Live (intraday trailing) text.

### 3.1 Evaluations (single phase, no time limit, monthly subscription)

| Plan | Size | Profit target | Drawdown (EOD, S4/S1/S2) | Daily loss limit | Max contracts (ladder top) | Consistency | Min days |
|---|---|---|---|---|---|---|---|
| TCP25 | $25,000 | 1750.00 | 1500.00 | 550.00 | 3 | 30% | none published |
| TCP50 | $50,000 | 3000.00 | 2000.00 | 1100.00 | 6 | 30% | none published |
| TCP100 | $100,000 | 6000.00 | 3500.00 | 2200.00 | 12 | 30% | none published |
| GAU50 | $50,000 | 3000.00 | 2000.00 | 1100.00 | 6 | 30% | none published |
| GAU100 | $100,000 | 6000.00 | 3500.00 | 2200.00 | 12 | 30% | none published |
| GAU150 | $150,000 | 9000.00 | 4500.00 | 3300.00 | 15 | 30% | none published |
| GAU200 | $200,000 | 11000.00 | 6000.00 | 4400.00 | 16 | 30% | none published |

Source of each column: profit target, drawdown, daily loss limit and ladder top are on the product pages (S1 for TCP25 and the growth plan, S2 for GAU50, rendered text) and in the same pages' JS data for every size (S3); drawdown amounts are cross-checked against the help-center drawdown images (S4, TCP25/50/100 and GAU50/100/150/200); ladder tables are cross-checked against the help images (S12, S13). Quotes: "Profit Goal $3,000 EOD Drawdown $2,000 Daily Loss Limit $1,100 Follow Progression Ladder Up to 6 Contracts Maintain Consistency 30%" (S2 page text, GAU50) and "Profit Goal $1,750 EOD Drawdown $1,500 Daily Loss Limit $550 Follow Progression Ladder Up to 3 Contracts" (S1 page text, TCP25).

- **Drawdown (S5):** "The Trader Career Path and Gauntlet Mini™ evaluations calculate drawdown based on the end of the day balance of the trading account." "The minimum account balance is only adjusted at the end of the trading day." "This continues until your Maximum Drawdown reaches the starting balance of your account" ... "at which point it will no longer rise." "open equity losses are taken into consideration when calculating whether or not the account failed on this rule." Type: `trailing_eod`, locks at the starting balance.
- **Daily loss limit (S9):** "Daily loss is based on your Profit & Loss, including both open (real-time/unrealized), closed trades and commissions. It's calculated daily from 5:00 pm to 5:00 pm (Central Time)." "if your PnL ever hits the -$550 limit (even if you don't close your position), your evaluation fails and will be suspended from further trading." S11: "Any deviation results in an evaluation failure, requiring a reset to continue." Treated as `hard` (fails the evaluation).
- **Consistency (S10):** "The "Maintain Consistency" rule states that over the course of your examination, no single trading day can account for 30% or more of your total PnL." A day above the limit does not fail the evaluation: "It just means that you need to trade more, until this single day equals less than 30% of your total profits." "The Maintain Consistency rule does not apply on the LiveSim® and/or Live accounts."
- **Min trading days (S11):** "There is no minimum number of trading days required to pass. However, due to the 30% consistency rule, a trader will need at least four profitable trading days to complete an evaluation."
- **Max position (Progression Ladder, S12/S13):** "The maximum position size defines the total number of contracts a trader can have open across all positions." "Exceeding the allowed contract limit is a hard fail and will immediately end the evaluation attempt." The limit scales with profit (real-time):

| Size | Profit band : contracts |
|---|---|
| $25,000 (TCP25) | $0-$1,500: 2 · $1,501+: 3 |
| $50,000 | $0-$1,500: 2 · $1,501-$2,000: 4 · $2,001+: 6 |
| $100,000 | $0-$1,500: 3 · $1,501-$3,000: 6 · $3,001-$4,500: 9 · $4,501+: 12 |
| $150,000 | $0-$1,500: 4 · $1,501-$3,000: 6 · $3,001-$4,500: 9 · $4,501+: 15 |
| $200,000 | $0-$1,500: 5 · $1,501-$3,000: 7 · $3,001-$4,500: 10 · $4,501+: 16 |
| $400,000 (Live only) | $0-$1,500: 8 · $1,501-$3,000: 12 · $3,001-$4,500: 18 · $4,501+: 30 |

  Micros: 10 micros = 1 contract (S12), so the micro cap is ten times the minis cap in the YAML (a published formula).
- **Free resets, reset prices, subscription:** TCP includes a free reset after every monthly rebill (S46). Not routing-relevant; not modeled.
- **Max concurrent evaluations:** 5 (S19).

### 3.2 Funded accounts (after passing). LiveSim and Live

| Plan | Account | Size | Drawdown | Daily loss limit | Max contracts | Consistency | Min days |
|---|---|---|---|---|---|---|---|
| TCP / GAU | LiveSim | same as the passed evaluation | EOD, same amount as the evaluation | same as the evaluation | same as the evaluation | none | none |
| TCP / GAU | Live | same, or the next growth-plan size | `trailing_intraday`, same amount | same | same | none | none |

Published (S26): "Drawdown: Funded accounts are subject to drawdown rules. LiveSim® accounts use End of Day (EOD) Drawdown. Live accounts use Trailing Drawdown. The $200K and $400K TCP accounts use Fixed Drawdown." "Daily Loss Limit: Funded accounts have a daily loss limit that must not be breached." "Progression Ladder: The maximum position size rules apply and must be respected. Exceeding the limit is a hard fail." "Trading Hours: All positions and working orders must be closed between 3:50 PM CT and 5:00 PM CT. No overnight positions are permitted." "No Consistency Requirement: There is no consistency rule on funded accounts." "No Minimum Trading Days: There are no minimum trading days required to be able to withdraw." "News Trading Permitted: Users are permitted to trade during news events on funded accounts."

- **Trailing (Live) drawdown (S6):** "Trailing Drawdown - Increases dollar-for-dollar with your positive account performance during the trading day until it reaches the starting balance." (S7). Example in S6: "in a $50,000.00 account your starting trailing drawdown is $48,000.00" ... "at which point the drawdown will no longer trail above $50,000.00."
- **Fixed drawdown (S8):** "Traders who pass the TCP25 and reach the $200,000 funded trading account have their drawdown fixed at $194,000." "Traders who pass the TCP50 or TCP100 and reach the $400,000 funded trading account have their drawdown fixed at $388,000." The $200,000 account reached through TCP50/TCP100 is trailing ($6,000) per the product page data (S3). "Please note that the $200,000 funded trading account received after passing the Gauntlet Mini™ does not use a fixed drawdown."
- **Growth plan (S25, S1):** TCP25 starts at $25,000 and grows to $50,000, $100,000 and $200,000; TCP50 starts at $50,000 and grows to $100,000, $200,000 and $400,000; TCP100 starts at $100,000 and grows to $150,000, $200,000 and $400,000. At each step the trader must hit the step's "Profit Target to Advance" (TCP25 $1,750, then $3,000, $6,000, $11,000 on the page card; each size's advance target equals the evaluation target of the same size in S3) and withdraw the profit share. These are advance targets, not evaluation targets; `profit_target_usd` is `none` for funded blocks.
- **Daily loss limit by size:** $25,000 550.00 · $50,000 1100.00 · $100,000 2200.00 · $150,000 3300.00 · $200,000 4400.00 · $400,000 8800.00 (S1 growth card for 25K to 200K; S3 for 150K and 400K).
- **DLL behavior on funded accounts: not stated.** S9 describes the evaluation breach as a failure. The product pages' legend shows two states ("The account remains active, but trading can only resume the next day." and "The account is failed, requiring a subscription reset or a new purchase to continue.") without saying which applies to the funded DLL. Recorded as `unknown`, in `not_confirmed_fields`.
- **Account limits (S19, S26):** "Traders can have 3 funded trading accounts active concurrently. You can have 3 livesims, or 2 livesims and 1 live account."
- **Payout rules that change what the trader may do (S39):** profit split by single withdrawal size; weekly Wednesday payouts; minimum net withdrawal $100. LiveSim: one-time $139 activation fee deducted from the first withdrawal (non-professional). Live: data fees $140/exchange/month (Rithmic) or $156 (NinjaTrader) (S17, S40). Not a trading restriction.

### 3.3 TCP LiveSim/Live path summary

| Path | Funded steps (account kind at each step) |
|---|---|
| TCP25 | $25,000 (LiveSim or Live) → $50,000 Live → $100,000 Live → $200,000 Live (fixed, minimum balance $194,000) |
| TCP50 | $50,000 (LiveSim or Live) → $100,000 Live → $200,000 Live (trailing 6000.00) → $400,000 Live (fixed, minimum balance $388,000) |
| TCP100 | $100,000 (LiveSim or Live) → $150,000 Live (4500.00) → $200,000 Live (trailing 6000.00) → $400,000 Live (fixed, minimum balance $388,000) |
| GAU | $50,000 / $100,000 / $150,000 / $200,000, LiveSim or Live at the same size, no growth |

## 4. Trading-day reset

- **Trading day: 5:00 PM CT to 5:00 PM CT.** "It's calculated daily from 5:00 pm to 5:00 pm (Central Time)." (S9). "You may trade beginning when the exchange trading opens for your particular asset. Generally this is 5pm CT but may differ based on holiday or asset." (S14).
- **Must be flat:** "All positions and working orders must be closed by 3:50pm CT until 5pm CT." (S14). "you CANNOT hold overnight positions in the Trader Career Path®/ Gauntlet Mini™ and all positions must be closed by 15:50 CT (3:50pm CT)." (S15). The same applies to funded accounts (S26).
- **Holidays:** "Trading can be conducted throughout the day except over public holidays and any times when the exchanges are closed." (S14). Closure details are delegated to the CME holiday calendar (S38: "Please see this link for market closure details: CME Group Holiday Calendar"). No E2T-specific holiday schedule is published.
- **Note:** the product page tooltip for the daily loss limit says it is "based on the CME calendar" ... "calculated on open equity" (S3 data); S9 states 5:00 pm to 5:00 pm CT. They do not conflict on the time.
- **Rithmic end-of-day processing (S5):** "Rithmic processes the regularly uploaded minimum account balance every day, during the 4-5pm CT market close."

## 5. Trade copying stance

Classification: **no** for every plan and phase (evaluation, LiveSim, Live).

- "The use of trade copiers is not allowed on any of our programs." (S18). "copy-trading and batch execution across multiple accounts is not permitted." "Even after you've earned a LiveSim® or Live account, trade copiers remain off-limits. All trading must be your own, so that your progress and performance truly reflect your skills." (S18).
- Terms (S21): "You are not permitted to use trade copiers to pass multiple accounts. If you pass the exam and are found to have used trade copiers to pass your accounts, you will receive only one account (LiveSim® or Live Account)." "You are not permitted to use trade copiers on your live / LiveSim® account once you have passed, either."
- Help center (S18, S19): "you may be denied your accounts or withdrawals. You will not receive a refund on any accounts that you used trade copiers." The consequence text differs between the terms and the help center; the stance (forbidden) does not.
- S19: "The prop firms do not provide a leader-follower or other trade copy system between the accounts." "Hedging between your funded trading accounts is strictly forbidden and your account access will be terminated if you engage in similar behavior."
- **Account-count limits:** 5 concurrent evaluations; 3 funded accounts (3 LiveSim, or 2 LiveSim + 1 Live) (S19).
- **Same person, own accounts:** copying across one's own accounts is forbidden (S18). The text frames it as "Copy-trading between accounts goes against the spirit of this approach".
- **Copying with other firms' accounts:** not mentioned on any official page → `unknown` (§10). Prohibited conduct covers "trading between connected accounts, multiple accounts across different parties, or executing trades that hedge or pool risks among unrelated accounts" (S20).

## 6. Automated trading stance

**(i) Classification: `conditional`** for all plans and phases. No page says "automated trading is allowed" or "forbidden". The only text that reaches automation is one bullet of the prohibited-conduct list (S20, repeated in the terms S21). Because it forbids software only when it "could potentially" manipulate the environment or give "an unfair advantage", the literal reading is conditional. This classification is the researcher's reading of one clause and is recorded as unconfirmed (§10).

Verbatim `automation_rule_text` (shown in the opt-in warning):

> "Utilizing software, artificial intelligence, or ultra-fast data entry techniques that could potentially manipulate the trading environment or provide an unfair advantage"

Context (S20): "Earn2Trade identifies the following as prohibited conduct:" ... "Executing an unrealistic number of contracts in a single day;" ... "Trading in a manner that does not align with actual market practices in the relevant futures markets or could potentially cause financial or other types of harm to Earn2Trade;" ... "Engaging in disruptive trading practices, including spoofing;" ... "Using tools or strategies that negatively impact the site's functionality or exploit any of its technical flaws;" ... "Bypassing geographical or other technical restrictions;". Consequences listed: issuing a warning, deleting the trading data from the affected day, resetting the trader's account, permanently closing the trader's account. "Both the Gauntlet Mini™ and Trader Career Path® programs aim to mirror the conditions of live-market trading" (S20).

**(ii) Named mechanisms:**
- **Semi-automated trading:** not mentioned.
- **Trade copiers:** mentioned and forbidden (§5).
- **Webhooks / alerts:** not mentioned. TradingView is a listed free platform (connects through Tradovate, S34/S40 article), with no statement about alerts or webhooks; S42 only describes the manual connection.
- **ATM / automatic brackets:** mentioned only as a platform-behavior warning: "When using the ATM Strategy feature in NinjaTrader, orders are stored locally on your own computer. This is part of the platform's design and not something we can change or access, as it's a tool you've chosen to use." (S30). Not a prohibition.
- **Bots / algorithms / API:** the words "bot", "algo", "automated trading" and "API" (as a customer trading API) do not appear in the help center or terms in a trading-rule context. The NinjaTrader-Tradovate "unified API" in S23 is the connection, not a customer order API.

**(iii) Differs by phase:** no. The prohibited-conduct text covers evaluations and funded environments alike: "to maintain the integrity of all trading simulations, including the LiveSim® and Live Trading environments." (S20).

## 7. Device / hosting rules and other rules

### Device / hosting rules (format rev. 3 §3 item 7)

`vps_allowed: unknown` and `personal_device_required: unknown` for every block. Gap entry in §10 (searched all 94 articles, the terms, the disclaimer, the support policy and the pledge for VPS, VPN, remote, server, device, IP, computer, location).

What is published:
- Not found: any statement about VPS, VPN, remote desktop, remote server or an order-origin / personal-device requirement. R4 re-check 2026-10-23: re-fetched the help-center sitemap (202 English URLs) and the terms (https://www.earn2trade.com/terms-and-conditions); zero hits for vps, vpn, remote desktop, own/personal computer, IP address rules.
- Related, not a device rule (account sharing): "Data credentials, which are provisioned for the sole usage of the Account Holder, shall not be shared with, or used by any other individual or entity. It is explicitly prohibited for any person, other than the Account Holder, to gain access to the evaluation account or to execute trades on behalf of the Account Holder." (terms, accessed 2026-10-23). Also "Trading on behalf of others or sharing incentives through any form of business arrangement." (prohibited conduct, https://help.earn2trade.com/en/articles/9286647-prohibited-conduct-in-earn2trade-evaluations-and-in-the-livesim-live-trading-environment, accessed 2026-10-23). These ban third-party operation of the account, not remote or VPS hosting by the account holder.
- Related: "Bypassing geographical or other technical restrictions;" is prohibited conduct (S20). It is about geographic restrictions, not hosting.
- Related: the site requires a Chrome browser for the dashboard (S45: "Google Chrome Browser v63 or better", "currently the system will block any non Chrome browser"), minimum specs "Windows 10 or OSX High Sierra", "Quad core 2.3 Ghz or better", "6 Gigs of RAM or better". These are specs for the dashboard and simulator, not a rule on where orders come from.
- Related: the clause on software and ultra-fast data entry techniques (§6) could apply to server-side order relays; the page does not say so.
- Related: the funded Live account is a real brokerage account (FCM), so the FCM or partner firm may impose its own rules that E2T does not publish (S28 image footnote: "Live accounts are funded brokerage accounts trading on the CME through the funding firm's FCM accounts.").

### Other rules

- **News:** allowed. "You are allowed to trade during news events — there are no restrictions on trading around economic announcements." (S14).
- **Overnight / weekend:** not allowed (§4).
- **Mandatory stop loss:** none published. **Min / max trade duration:** none published.
- **Hedging:** hedging between funded accounts forbidden (S19); trades that "hedge or pool risks among unrelated accounts" are prohibited (S20).
- **Unrealistic number of contracts in a single day:** prohibited, no number published (S20).
- **Scaling:** Progression Ladder by profit (§3.1), hard fail when exceeded. S44 also warns about double orders: "Pressing the "close position" button at this time multiple times will result in multiple orders being sent, in many cases, failing the trader on the progression ladder.".
- **Max accounts:** 5 evaluations; 3 funded (S19).
- **Inactivity:** no inactivity rule found.
- **Prohibited assets:** Forex, Bitcoin futures, Ether futures (S29). Low-liquidity contracts: Rithmic open equity uses bid/ask (S29 note).
- **Data (S17, S40):** non-professional CME status: data covered in evaluations; Market Depth (DOM) costs extra. Professional status: $140.00 per exchange per month (Rithmic), $156.00 (NinjaTrader) after funding.

## 8. Commissions

Commissions on evaluation accounts mirror the partner firms' live schedule, charged per side: "These live commissions mirror the commissions schedule offered by our proprietary trading partners, reflecting actual market conditions and execution costs. The cost associated with this live commission schedule varies by asset. For example, micro contracts range from $0.82 to $1.20 per contract per side." (S16). "live commission costs are liable to change at any time without advance warning." (S16).

The firm publishes three per-platform charts (images in S16, column "Total Per Side"). Transcribed from the images (NQ row; the charts list ES, NQ, MYM, M2K, UB/ZB, ZN, GC, MGC, CL, MCL, ZC, 6E, E7):

| Chart (platform) | NQ per side | MNQ per side | MYM / M2K per side |
|---|---|---|---|
| Rithmic | 2.04 | not listed | 0.93 / 0.93 |
| Tradovate (NinjaTrader option) | 2.88 | not listed | 0.95 / 0.95 |
| BlackArrow | 2.04 | not listed | 0.93 / 0.93 |

- **MNQ commission: not published** (the charts do not list MNQ; S16 text gives only the micro range $0.82 to $1.20 per side) → `null` + gap.
- **NQ commission** depends on the platform option chosen at purchase, so the YAML records `nq_usd: null` and the three values in `includes`. For a NinjaTrader-via-Tradovate account the published figure is 2.88 per side. The column label is "Total Per Side" (image text); whether it includes exchange, NFA and platform fees is not stated.
- **Funded Live / LiveSim:** "You are responsible for platform and exchange fees as well as the brokerage contract and commission fees. Brokerage, exchange, contract and commission fees are deducted directly from your account balance." (S40). "Traders are responsible for FCM commissions, NFA fees, and exchange fees per trade." (S17). No funded-account fee schedule is published.
- Product pages (S1/S2): "Commissions vary by instrument and are consistent with industry-standard rates for futures trading. Specific commission schedules are provided upon account activation and may depend on the provider or trading platform selected (Tradovate or Rithmic)."

## 9. Machine-readable blocks

Common to all blocks: `trading_day_reset` is 17:00 CT, flat by 15:50 CT (§4). `daily_loss_limit.kind` is `hard` for evaluations (published failure) and `unknown` for funded accounts (not stated; outside the format's `hard|soft_pause|none` set on purpose). `commission_per_contract` is `null` with `basis: per_side` and the published figures in `includes`. `copy_trading_allowed: no`; `automated_trading_allowed: conditional` (one clause, §6); `vps_allowed` and `personal_device_required` `unknown`. `max_position.micros` is minis x 10 (published formula, S12). `profit_target_usd` is `"none"` on funded blocks (the advance target is in the notes). The `platforms` list for the evaluation is the set of purchase-time platform options; the user picks one per subscription.


### Trader Career Path

#### Trader Career Path — evaluation — 25K

```yaml
firm: earn2trade
plan: "Trader Career Path (TCP25)"
phase: evaluation
size_label: "25K"
size_usd: "25000.00"
instruments: [MNQ, NQ]
platforms: ["Rithmic (Finamark, R|Trader Pro and more)", "BlackArrow", "NinjaTrader (via Tradovate)", "Tradovate", "TradingView (via Tradovate)", "dxFeed/DeepCharts"]
drawdown: { amount_usd: "1500.00", type: trailing_eod, stops_trailing_at: "Minimum balance rises with each end-of-day balance high and stops rising once it reaches the starting balance ($25,000)" }
daily_loss_limit: { amount_usd: "550.00", kind: hard }
profit_target_usd: "1750.00"
consistency_rule: { text: "No single trading day can account for 30% or more of total PnL over the evaluation (the evaluation continues until the day is below 30% of total profit; it is not an instant fail).", threshold_pct: "30" }
max_position: { minis: 3, micros: 30, scaling: "Progression Ladder by profit: $0-$1,500: 2; $1,501+: 3. Exceeding it is a hard fail; up to ten micros count as one standard contract." }
min_trading_days: null            # no minimum published
trading_day_reset: { time: "17:00", tz_as_published: "CT" }
commission_per_contract: { mnq_usd: null, nq_usd: null, basis: per_side, includes: "Published as 'Total Per Side' in the firm's per-platform chart; what it includes is not stated. Platform-dependent NQ per side: Rithmic 2.04, BlackArrow 2.04, Tradovate/NinjaTrader 2.88. MNQ is not listed (text only: micro contracts range from 0.82 to 1.20 per contract per side)" }
copy_trading_allowed: no
automated_trading_allowed: conditional
automation_rule_text: "Utilizing software, artificial intelligence, or ultra-fast data entry techniques that could potentially manipulate the trading environment or provide an unfair advantage"
vps_allowed: unknown
personal_device_required: unknown
status: partial
not_confirmed_fields: [commission_per_contract.mnq_usd, commission_per_contract.nq_usd, vps_allowed, personal_device_required, automated_trading_allowed]
notes: "Single-phase evaluation, no time limit. Daily loss breach fails the evaluation (hard). Must be flat by 15:50 CT. News trading allowed. Rithmic eval accounts: min balance only adjusted at end of day, open-equity losses count intraday. Profit target and per-size values from the product page; sizes 50K/100K read from the site's JS data (see S3)."
sources:
  - { field: drawdown, url: "https://help.earn2trade.com/en/articles/3224507-what-is-the-maximum-drawdown-on-the-trader-career-path-and-the-gauntlet-mini-challenges", accessed: "2026-10-23" }
  - { field: drawdown_rule, url: "https://help.earn2trade.com/en/articles/5372687-how-does-end-of-day-drawdown-work", accessed: "2026-10-23" }
  - { field: daily_loss_limit, url: "https://help.earn2trade.com/en/articles/3395926-how-is-my-daily-loss-calculated", accessed: "2026-10-23" }
  - { field: profit_target_usd, url: "https://www.earn2trade.com/trader-career-path", accessed: "2026-10-23" }
  - { field: profit_target_usd_all_sizes, url: "https://www.earn2trade.com/_next/static/chunks/0d45t7274e6.p.js", accessed: "2026-10-23" }
  - { field: consistency_rule, url: "https://help.earn2trade.com/en/articles/3849975-what-is-the-maintain-consistency-rule", accessed: "2026-10-23" }
  - { field: max_position, url: "https://help.earn2trade.com/en/articles/5922873-what-is-the-progression-ladder-on-the-trader-career-path", accessed: "2026-10-23" }
  - { field: trading_day_reset, url: "https://help.earn2trade.com/en/articles/3395926-how-is-my-daily-loss-calculated", accessed: "2026-10-23" }
  - { field: copy_trading_allowed, url: "https://help.earn2trade.com/en/articles/12034590-am-i-allowed-to-copy-trades-across-multiple-accounts", accessed: "2026-10-23" }
  - { field: automated_trading_allowed, url: "https://help.earn2trade.com/en/articles/9286647-prohibited-conduct-in-earn2trade-evaluations-and-in-the-livesim-live-trading-environment", accessed: "2026-10-23" }
  - { field: commission_per_contract, url: "https://help.earn2trade.com/en/articles/2425493-what-is-the-commission-on-earn2trade-evaluations", accessed: "2026-10-23" }
  - { field: platforms, url: "https://help.earn2trade.com/en/articles/2090521-what-platforms-can-i-use-for-the-gauntlet-mini-trader-career-path", accessed: "2026-10-23" }
```

#### Trader Career Path — evaluation — 50K

```yaml
firm: earn2trade
plan: "Trader Career Path (TCP50)"
phase: evaluation
size_label: "50K"
size_usd: "50000.00"
instruments: [MNQ, NQ]
platforms: ["Rithmic (Finamark, R|Trader Pro and more)", "BlackArrow", "NinjaTrader (via Tradovate)", "Tradovate", "TradingView (via Tradovate)", "dxFeed/DeepCharts"]
drawdown: { amount_usd: "2000.00", type: trailing_eod, stops_trailing_at: "Minimum balance rises with each end-of-day balance high and stops rising once it reaches the starting balance ($50,000)" }
daily_loss_limit: { amount_usd: "1100.00", kind: hard }
profit_target_usd: "3000.00"
consistency_rule: { text: "No single trading day can account for 30% or more of total PnL over the evaluation (the evaluation continues until the day is below 30% of total profit; it is not an instant fail).", threshold_pct: "30" }
max_position: { minis: 6, micros: 60, scaling: "Progression Ladder by profit: $0-$1,500: 2; $1,501-$2,000: 4; $2,001+: 6. Exceeding it is a hard fail; up to ten micros count as one standard contract." }
min_trading_days: null            # no minimum published
trading_day_reset: { time: "17:00", tz_as_published: "CT" }
commission_per_contract: { mnq_usd: null, nq_usd: null, basis: per_side, includes: "Published as 'Total Per Side' in the firm's per-platform chart; what it includes is not stated. Platform-dependent NQ per side: Rithmic 2.04, BlackArrow 2.04, Tradovate/NinjaTrader 2.88. MNQ is not listed (text only: micro contracts range from 0.82 to 1.20 per contract per side)" }
copy_trading_allowed: no
automated_trading_allowed: conditional
automation_rule_text: "Utilizing software, artificial intelligence, or ultra-fast data entry techniques that could potentially manipulate the trading environment or provide an unfair advantage"
vps_allowed: unknown
personal_device_required: unknown
status: partial
not_confirmed_fields: [commission_per_contract.mnq_usd, commission_per_contract.nq_usd, vps_allowed, personal_device_required, automated_trading_allowed]
notes: "Single-phase evaluation, no time limit. Daily loss breach fails the evaluation (hard). Must be flat by 15:50 CT. News trading allowed. Rithmic eval accounts: min balance only adjusted at end of day, open-equity losses count intraday. Profit target and per-size values from the product page; sizes 50K/100K read from the site's JS data (see S3)."
sources:
  - { field: drawdown, url: "https://help.earn2trade.com/en/articles/3224507-what-is-the-maximum-drawdown-on-the-trader-career-path-and-the-gauntlet-mini-challenges", accessed: "2026-10-23" }
  - { field: drawdown_rule, url: "https://help.earn2trade.com/en/articles/5372687-how-does-end-of-day-drawdown-work", accessed: "2026-10-23" }
  - { field: daily_loss_limit, url: "https://help.earn2trade.com/en/articles/3395926-how-is-my-daily-loss-calculated", accessed: "2026-10-23" }
  - { field: profit_target_usd, url: "https://www.earn2trade.com/trader-career-path", accessed: "2026-10-23" }
  - { field: profit_target_usd_all_sizes, url: "https://www.earn2trade.com/_next/static/chunks/0d45t7274e6.p.js", accessed: "2026-10-23" }
  - { field: consistency_rule, url: "https://help.earn2trade.com/en/articles/3849975-what-is-the-maintain-consistency-rule", accessed: "2026-10-23" }
  - { field: max_position, url: "https://help.earn2trade.com/en/articles/5922873-what-is-the-progression-ladder-on-the-trader-career-path", accessed: "2026-10-23" }
  - { field: trading_day_reset, url: "https://help.earn2trade.com/en/articles/3395926-how-is-my-daily-loss-calculated", accessed: "2026-10-23" }
  - { field: copy_trading_allowed, url: "https://help.earn2trade.com/en/articles/12034590-am-i-allowed-to-copy-trades-across-multiple-accounts", accessed: "2026-10-23" }
  - { field: automated_trading_allowed, url: "https://help.earn2trade.com/en/articles/9286647-prohibited-conduct-in-earn2trade-evaluations-and-in-the-livesim-live-trading-environment", accessed: "2026-10-23" }
  - { field: commission_per_contract, url: "https://help.earn2trade.com/en/articles/2425493-what-is-the-commission-on-earn2trade-evaluations", accessed: "2026-10-23" }
  - { field: platforms, url: "https://help.earn2trade.com/en/articles/2090521-what-platforms-can-i-use-for-the-gauntlet-mini-trader-career-path", accessed: "2026-10-23" }
```

#### Trader Career Path — evaluation — 100K

```yaml
firm: earn2trade
plan: "Trader Career Path (TCP100)"
phase: evaluation
size_label: "100K"
size_usd: "100000.00"
instruments: [MNQ, NQ]
platforms: ["Rithmic (Finamark, R|Trader Pro and more)", "BlackArrow", "NinjaTrader (via Tradovate)", "Tradovate", "TradingView (via Tradovate)", "dxFeed/DeepCharts"]
drawdown: { amount_usd: "3500.00", type: trailing_eod, stops_trailing_at: "Minimum balance rises with each end-of-day balance high and stops rising once it reaches the starting balance ($100,000)" }
daily_loss_limit: { amount_usd: "2200.00", kind: hard }
profit_target_usd: "6000.00"
consistency_rule: { text: "No single trading day can account for 30% or more of total PnL over the evaluation (the evaluation continues until the day is below 30% of total profit; it is not an instant fail).", threshold_pct: "30" }
max_position: { minis: 12, micros: 120, scaling: "Progression Ladder by profit: $0-$1,500: 3; $1,501-$3,000: 6; $3,001-$4,500: 9; $4,501+: 12. Exceeding it is a hard fail; up to ten micros count as one standard contract." }
min_trading_days: null            # no minimum published
trading_day_reset: { time: "17:00", tz_as_published: "CT" }
commission_per_contract: { mnq_usd: null, nq_usd: null, basis: per_side, includes: "Published as 'Total Per Side' in the firm's per-platform chart; what it includes is not stated. Platform-dependent NQ per side: Rithmic 2.04, BlackArrow 2.04, Tradovate/NinjaTrader 2.88. MNQ is not listed (text only: micro contracts range from 0.82 to 1.20 per contract per side)" }
copy_trading_allowed: no
automated_trading_allowed: conditional
automation_rule_text: "Utilizing software, artificial intelligence, or ultra-fast data entry techniques that could potentially manipulate the trading environment or provide an unfair advantage"
vps_allowed: unknown
personal_device_required: unknown
status: partial
not_confirmed_fields: [commission_per_contract.mnq_usd, commission_per_contract.nq_usd, vps_allowed, personal_device_required, automated_trading_allowed]
notes: "Single-phase evaluation, no time limit. Daily loss breach fails the evaluation (hard). Must be flat by 15:50 CT. News trading allowed. Rithmic eval accounts: min balance only adjusted at end of day, open-equity losses count intraday. Profit target and per-size values from the product page; sizes 50K/100K read from the site's JS data (see S3)."
sources:
  - { field: drawdown, url: "https://help.earn2trade.com/en/articles/3224507-what-is-the-maximum-drawdown-on-the-trader-career-path-and-the-gauntlet-mini-challenges", accessed: "2026-10-23" }
  - { field: drawdown_rule, url: "https://help.earn2trade.com/en/articles/5372687-how-does-end-of-day-drawdown-work", accessed: "2026-10-23" }
  - { field: daily_loss_limit, url: "https://help.earn2trade.com/en/articles/3395926-how-is-my-daily-loss-calculated", accessed: "2026-10-23" }
  - { field: profit_target_usd, url: "https://www.earn2trade.com/trader-career-path", accessed: "2026-10-23" }
  - { field: profit_target_usd_all_sizes, url: "https://www.earn2trade.com/_next/static/chunks/0d45t7274e6.p.js", accessed: "2026-10-23" }
  - { field: consistency_rule, url: "https://help.earn2trade.com/en/articles/3849975-what-is-the-maintain-consistency-rule", accessed: "2026-10-23" }
  - { field: max_position, url: "https://help.earn2trade.com/en/articles/5922873-what-is-the-progression-ladder-on-the-trader-career-path", accessed: "2026-10-23" }
  - { field: trading_day_reset, url: "https://help.earn2trade.com/en/articles/3395926-how-is-my-daily-loss-calculated", accessed: "2026-10-23" }
  - { field: copy_trading_allowed, url: "https://help.earn2trade.com/en/articles/12034590-am-i-allowed-to-copy-trades-across-multiple-accounts", accessed: "2026-10-23" }
  - { field: automated_trading_allowed, url: "https://help.earn2trade.com/en/articles/9286647-prohibited-conduct-in-earn2trade-evaluations-and-in-the-livesim-live-trading-environment", accessed: "2026-10-23" }
  - { field: commission_per_contract, url: "https://help.earn2trade.com/en/articles/2425493-what-is-the-commission-on-earn2trade-evaluations", accessed: "2026-10-23" }
  - { field: platforms, url: "https://help.earn2trade.com/en/articles/2090521-what-platforms-can-i-use-for-the-gauntlet-mini-trader-career-path", accessed: "2026-10-23" }
```

#### Trader Career Path — LiveSim (funded) — 25K

```yaml
firm: earn2trade
plan: "Trader Career Path (TCP25)"
phase: funded
size_label: "25K"
size_usd: "25000.00"
instruments: [MNQ, NQ]
platforms: ["Any platform on the published list (NinjaTrader, Finamark, R|Trader Pro, Tradovate, TradingView, BlackArrow, DeepCharts, Quantower and others)"]
drawdown: { amount_usd: "1500.00", type: trailing_eod, stops_trailing_at: "Minimum balance rises with each end-of-day balance high and stops rising once it reaches the starting balance ($25,000)" }
daily_loss_limit: { amount_usd: "550.00", kind: unknown }
profit_target_usd: "none"
consistency_rule: { text: "none (published: No Consistency Requirement)", threshold_pct: null }
max_position: { minis: 3, micros: 30, scaling: "Progression Ladder by profit: $0-$1,500: 2; $1,501+: 3. Exceeding it is a hard fail; up to ten micros count as one standard contract." }
min_trading_days: null            # no minimum published
trading_day_reset: { time: "17:00", tz_as_published: "CT" }
commission_per_contract: { mnq_usd: null, nq_usd: null, basis: per_side, includes: "Published as 'Total Per Side' in the firm's per-platform chart; what it includes is not stated. Platform-dependent NQ per side: Rithmic 2.04, BlackArrow 2.04, Tradovate/NinjaTrader 2.88. MNQ is not listed (text only: micro contracts range from 0.82 to 1.20 per contract per side)" }
copy_trading_allowed: no
automated_trading_allowed: conditional
automation_rule_text: "Utilizing software, artificial intelligence, or ultra-fast data entry techniques that could potentially manipulate the trading environment or provide an unfair advantage"
vps_allowed: unknown
personal_device_required: unknown
status: partial
not_confirmed_fields: [commission_per_contract.mnq_usd, commission_per_contract.nq_usd, vps_allowed, personal_device_required, automated_trading_allowed, daily_loss_limit.kind]
notes: "LiveSim = simulated funded account with live data (funded account kind: LiveSim). No consistency rule, no minimum trading days, no subscription. Profit target to advance to the next Live size (not an evaluation target): $1,750. Daily loss limit behavior on funded accounts is not stated (page legend only: warning = account stays active and trading resumes the next day; failed = account failed). $139 one-time activation fee is deducted from the first withdrawal."
sources:
  - { field: drawdown, url: "https://help.earn2trade.com/en/articles/3224507-what-is-the-maximum-drawdown-on-the-trader-career-path-and-the-gauntlet-mini-challenges", accessed: "2026-10-23" }
  - { field: drawdown_types, url: "https://help.earn2trade.com/en/articles/8117088-drawdown-types", accessed: "2026-10-23" }
  - { field: daily_loss_limit, url: "https://help.earn2trade.com/en/articles/3395926-how-is-my-daily-loss-calculated", accessed: "2026-10-23" }
  - { field: profit_target_to_advance, url: "https://www.earn2trade.com/trader-career-path", accessed: "2026-10-23" }
  - { field: consistency_rule, url: "https://help.earn2trade.com/en/articles/6877364-what-happens-when-i-complete-my-evaluation", accessed: "2026-10-23" }
  - { field: max_position, url: "https://help.earn2trade.com/en/articles/5922873-what-is-the-progression-ladder-on-the-trader-career-path", accessed: "2026-10-23" }
  - { field: trading_day_reset, url: "https://help.earn2trade.com/en/articles/3395926-how-is-my-daily-loss-calculated", accessed: "2026-10-23" }
  - { field: copy_trading_allowed, url: "https://help.earn2trade.com/en/articles/12034590-am-i-allowed-to-copy-trades-across-multiple-accounts", accessed: "2026-10-23" }
  - { field: automated_trading_allowed, url: "https://help.earn2trade.com/en/articles/9286647-prohibited-conduct-in-earn2trade-evaluations-and-in-the-livesim-live-trading-environment", accessed: "2026-10-23" }
  - { field: commission_per_contract, url: "https://help.earn2trade.com/en/articles/2425493-what-is-the-commission-on-earn2trade-evaluations", accessed: "2026-10-23" }
  - { field: platforms, url: "https://help.earn2trade.com/en/articles/2090521-what-platforms-can-i-use-for-the-gauntlet-mini-trader-career-path", accessed: "2026-10-23" }
```

#### Trader Career Path — LiveSim (funded) — 50K

```yaml
firm: earn2trade
plan: "Trader Career Path (TCP50)"
phase: funded
size_label: "50K"
size_usd: "50000.00"
instruments: [MNQ, NQ]
platforms: ["Any platform on the published list (NinjaTrader, Finamark, R|Trader Pro, Tradovate, TradingView, BlackArrow, DeepCharts, Quantower and others)"]
drawdown: { amount_usd: "2000.00", type: trailing_eod, stops_trailing_at: "Minimum balance rises with each end-of-day balance high and stops rising once it reaches the starting balance ($50,000)" }
daily_loss_limit: { amount_usd: "1100.00", kind: unknown }
profit_target_usd: "none"
consistency_rule: { text: "none (published: No Consistency Requirement)", threshold_pct: null }
max_position: { minis: 6, micros: 60, scaling: "Progression Ladder by profit: $0-$1,500: 2; $1,501-$2,000: 4; $2,001+: 6. Exceeding it is a hard fail; up to ten micros count as one standard contract." }
min_trading_days: null            # no minimum published
trading_day_reset: { time: "17:00", tz_as_published: "CT" }
commission_per_contract: { mnq_usd: null, nq_usd: null, basis: per_side, includes: "Published as 'Total Per Side' in the firm's per-platform chart; what it includes is not stated. Platform-dependent NQ per side: Rithmic 2.04, BlackArrow 2.04, Tradovate/NinjaTrader 2.88. MNQ is not listed (text only: micro contracts range from 0.82 to 1.20 per contract per side)" }
copy_trading_allowed: no
automated_trading_allowed: conditional
automation_rule_text: "Utilizing software, artificial intelligence, or ultra-fast data entry techniques that could potentially manipulate the trading environment or provide an unfair advantage"
vps_allowed: unknown
personal_device_required: unknown
status: partial
not_confirmed_fields: [commission_per_contract.mnq_usd, commission_per_contract.nq_usd, vps_allowed, personal_device_required, automated_trading_allowed, daily_loss_limit.kind]
notes: "LiveSim = simulated funded account with live data (funded account kind: LiveSim). No consistency rule, no minimum trading days, no subscription. Profit target to advance to the next Live size (not an evaluation target): $3,000. Daily loss limit behavior on funded accounts is not stated (page legend only: warning = account stays active and trading resumes the next day; failed = account failed). $139 one-time activation fee is deducted from the first withdrawal."
sources:
  - { field: drawdown, url: "https://help.earn2trade.com/en/articles/3224507-what-is-the-maximum-drawdown-on-the-trader-career-path-and-the-gauntlet-mini-challenges", accessed: "2026-10-23" }
  - { field: drawdown_types, url: "https://help.earn2trade.com/en/articles/8117088-drawdown-types", accessed: "2026-10-23" }
  - { field: daily_loss_limit, url: "https://help.earn2trade.com/en/articles/3395926-how-is-my-daily-loss-calculated", accessed: "2026-10-23" }
  - { field: profit_target_to_advance, url: "https://www.earn2trade.com/trader-career-path", accessed: "2026-10-23" }
  - { field: consistency_rule, url: "https://help.earn2trade.com/en/articles/6877364-what-happens-when-i-complete-my-evaluation", accessed: "2026-10-23" }
  - { field: max_position, url: "https://help.earn2trade.com/en/articles/5922873-what-is-the-progression-ladder-on-the-trader-career-path", accessed: "2026-10-23" }
  - { field: trading_day_reset, url: "https://help.earn2trade.com/en/articles/3395926-how-is-my-daily-loss-calculated", accessed: "2026-10-23" }
  - { field: copy_trading_allowed, url: "https://help.earn2trade.com/en/articles/12034590-am-i-allowed-to-copy-trades-across-multiple-accounts", accessed: "2026-10-23" }
  - { field: automated_trading_allowed, url: "https://help.earn2trade.com/en/articles/9286647-prohibited-conduct-in-earn2trade-evaluations-and-in-the-livesim-live-trading-environment", accessed: "2026-10-23" }
  - { field: commission_per_contract, url: "https://help.earn2trade.com/en/articles/2425493-what-is-the-commission-on-earn2trade-evaluations", accessed: "2026-10-23" }
  - { field: platforms, url: "https://help.earn2trade.com/en/articles/2090521-what-platforms-can-i-use-for-the-gauntlet-mini-trader-career-path", accessed: "2026-10-23" }
```

#### Trader Career Path — LiveSim (funded) — 100K

```yaml
firm: earn2trade
plan: "Trader Career Path (TCP100)"
phase: funded
size_label: "100K"
size_usd: "100000.00"
instruments: [MNQ, NQ]
platforms: ["Any platform on the published list (NinjaTrader, Finamark, R|Trader Pro, Tradovate, TradingView, BlackArrow, DeepCharts, Quantower and others)"]
drawdown: { amount_usd: "3500.00", type: trailing_eod, stops_trailing_at: "Minimum balance rises with each end-of-day balance high and stops rising once it reaches the starting balance ($100,000)" }
daily_loss_limit: { amount_usd: "2200.00", kind: unknown }
profit_target_usd: "none"
consistency_rule: { text: "none (published: No Consistency Requirement)", threshold_pct: null }
max_position: { minis: 12, micros: 120, scaling: "Progression Ladder by profit: $0-$1,500: 3; $1,501-$3,000: 6; $3,001-$4,500: 9; $4,501+: 12. Exceeding it is a hard fail; up to ten micros count as one standard contract." }
min_trading_days: null            # no minimum published
trading_day_reset: { time: "17:00", tz_as_published: "CT" }
commission_per_contract: { mnq_usd: null, nq_usd: null, basis: per_side, includes: "Published as 'Total Per Side' in the firm's per-platform chart; what it includes is not stated. Platform-dependent NQ per side: Rithmic 2.04, BlackArrow 2.04, Tradovate/NinjaTrader 2.88. MNQ is not listed (text only: micro contracts range from 0.82 to 1.20 per contract per side)" }
copy_trading_allowed: no
automated_trading_allowed: conditional
automation_rule_text: "Utilizing software, artificial intelligence, or ultra-fast data entry techniques that could potentially manipulate the trading environment or provide an unfair advantage"
vps_allowed: unknown
personal_device_required: unknown
status: partial
not_confirmed_fields: [commission_per_contract.mnq_usd, commission_per_contract.nq_usd, vps_allowed, personal_device_required, automated_trading_allowed, daily_loss_limit.kind]
notes: "LiveSim = simulated funded account with live data (funded account kind: LiveSim). No consistency rule, no minimum trading days, no subscription. Profit target to advance to the next Live size (not an evaluation target): $6,000. Daily loss limit behavior on funded accounts is not stated (page legend only: warning = account stays active and trading resumes the next day; failed = account failed). $139 one-time activation fee is deducted from the first withdrawal."
sources:
  - { field: drawdown, url: "https://help.earn2trade.com/en/articles/3224507-what-is-the-maximum-drawdown-on-the-trader-career-path-and-the-gauntlet-mini-challenges", accessed: "2026-10-23" }
  - { field: drawdown_types, url: "https://help.earn2trade.com/en/articles/8117088-drawdown-types", accessed: "2026-10-23" }
  - { field: daily_loss_limit, url: "https://help.earn2trade.com/en/articles/3395926-how-is-my-daily-loss-calculated", accessed: "2026-10-23" }
  - { field: profit_target_to_advance, url: "https://www.earn2trade.com/trader-career-path", accessed: "2026-10-23" }
  - { field: consistency_rule, url: "https://help.earn2trade.com/en/articles/6877364-what-happens-when-i-complete-my-evaluation", accessed: "2026-10-23" }
  - { field: max_position, url: "https://help.earn2trade.com/en/articles/5922873-what-is-the-progression-ladder-on-the-trader-career-path", accessed: "2026-10-23" }
  - { field: trading_day_reset, url: "https://help.earn2trade.com/en/articles/3395926-how-is-my-daily-loss-calculated", accessed: "2026-10-23" }
  - { field: copy_trading_allowed, url: "https://help.earn2trade.com/en/articles/12034590-am-i-allowed-to-copy-trades-across-multiple-accounts", accessed: "2026-10-23" }
  - { field: automated_trading_allowed, url: "https://help.earn2trade.com/en/articles/9286647-prohibited-conduct-in-earn2trade-evaluations-and-in-the-livesim-live-trading-environment", accessed: "2026-10-23" }
  - { field: commission_per_contract, url: "https://help.earn2trade.com/en/articles/2425493-what-is-the-commission-on-earn2trade-evaluations", accessed: "2026-10-23" }
  - { field: platforms, url: "https://help.earn2trade.com/en/articles/2090521-what-platforms-can-i-use-for-the-gauntlet-mini-trader-career-path", accessed: "2026-10-23" }
```

#### Trader Career Path — Live (funded) — 25K

```yaml
firm: earn2trade
plan: "Trader Career Path (growth plan Live 25K)"
phase: live
size_label: "25K"
size_usd: "25000.00"
instruments: [MNQ, NQ]
platforms: ["NinjaTrader desktop (live credentials from the prop firm)", "Rithmic (R|Trader Pro and compatible platforms)"]
drawdown: { amount_usd: "1500.00", type: trailing_intraday, stops_trailing_at: "Trails the highest balance intraday and locks at the starting balance ($25,000)" }
daily_loss_limit: { amount_usd: "550.00", kind: unknown }
profit_target_usd: "none"
consistency_rule: { text: "none (published: No Consistency Requirement)", threshold_pct: null }
max_position: { minis: 3, micros: 30, scaling: "Progression Ladder by profit: $0-$1,500: 2; $1,501+: 3. Exceeding it is a hard fail; up to ten micros count as one standard contract." }
min_trading_days: null            # no minimum published
trading_day_reset: { time: "17:00", tz_as_published: "CT" }
commission_per_contract: { mnq_usd: null, nq_usd: null, basis: per_side, includes: "Published as 'Total Per Side' in the firm's per-platform chart; what it includes is not stated. Platform-dependent NQ per side: Rithmic 2.04, BlackArrow 2.04, Tradovate/NinjaTrader 2.88. MNQ is not listed (text only: micro contracts range from 0.82 to 1.20 per contract per side)" }
copy_trading_allowed: no
automated_trading_allowed: conditional
automation_rule_text: "Utilizing software, artificial intelligence, or ultra-fast data entry techniques that could potentially manipulate the trading environment or provide an unfair advantage"
vps_allowed: unknown
personal_device_required: unknown
status: partial
not_confirmed_fields: [commission_per_contract.mnq_usd, commission_per_contract.nq_usd, vps_allowed, personal_device_required, automated_trading_allowed, daily_loss_limit.kind]
notes: "Live brokerage account (real capital via the prop firm's FCM). Reached via: TCP25 start (LiveSim or Live at the same size). Profit target to advance: $1750.00 (not an evaluation target). Max 1 Live account at a time. Daily loss limit behavior on funded accounts is not stated. Live data fees: $140 per exchange per month (Rithmic), $156 (NinjaTrader). Size 200K values (and 150K) come from the site's JS data and the Progression Ladder image; the TCP page growth card shows 25K/50K/100K/200K."
sources:
  - { field: drawdown, url: "https://help.earn2trade.com/en/articles/3224507-what-is-the-maximum-drawdown-on-the-trader-career-path-and-the-gauntlet-mini-challenges", accessed: "2026-10-23" }
  - { field: drawdown_types, url: "https://help.earn2trade.com/en/articles/8117088-drawdown-types", accessed: "2026-10-23" }
  - { field: daily_loss_limit, url: "https://help.earn2trade.com/en/articles/3395926-how-is-my-daily-loss-calculated", accessed: "2026-10-23" }
  - { field: profit_target_to_advance, url: "https://www.earn2trade.com/trader-career-path", accessed: "2026-10-23" }
  - { field: consistency_rule, url: "https://help.earn2trade.com/en/articles/6877364-what-happens-when-i-complete-my-evaluation", accessed: "2026-10-23" }
  - { field: max_position, url: "https://help.earn2trade.com/en/articles/5922873-what-is-the-progression-ladder-on-the-trader-career-path", accessed: "2026-10-23" }
  - { field: trading_day_reset, url: "https://help.earn2trade.com/en/articles/3395926-how-is-my-daily-loss-calculated", accessed: "2026-10-23" }
  - { field: copy_trading_allowed, url: "https://help.earn2trade.com/en/articles/12034590-am-i-allowed-to-copy-trades-across-multiple-accounts", accessed: "2026-10-23" }
  - { field: automated_trading_allowed, url: "https://help.earn2trade.com/en/articles/9286647-prohibited-conduct-in-earn2trade-evaluations-and-in-the-livesim-live-trading-environment", accessed: "2026-10-23" }
  - { field: commission_per_contract, url: "https://help.earn2trade.com/en/articles/2425493-what-is-the-commission-on-earn2trade-evaluations", accessed: "2026-10-23" }
  - { field: platforms, url: "https://help.earn2trade.com/en/articles/2090521-what-platforms-can-i-use-for-the-gauntlet-mini-trader-career-path", accessed: "2026-10-23" }
  - { field: growth_plan_values, url: "https://www.earn2trade.com/trader-career-path", accessed: "2026-10-23" }
  - { field: growth_plan_values_js, url: "https://www.earn2trade.com/_next/static/chunks/0d45t7274e6.p.js", accessed: "2026-10-23" }
```

#### Trader Career Path — Live (funded) — 50K

```yaml
firm: earn2trade
plan: "Trader Career Path (growth plan Live 50K)"
phase: live
size_label: "50K"
size_usd: "50000.00"
instruments: [MNQ, NQ]
platforms: ["NinjaTrader desktop (live credentials from the prop firm)", "Rithmic (R|Trader Pro and compatible platforms)"]
drawdown: { amount_usd: "2000.00", type: trailing_intraday, stops_trailing_at: "Trails the highest balance intraday and locks at the starting balance ($50,000)" }
daily_loss_limit: { amount_usd: "1100.00", kind: unknown }
profit_target_usd: "none"
consistency_rule: { text: "none (published: No Consistency Requirement)", threshold_pct: null }
max_position: { minis: 6, micros: 60, scaling: "Progression Ladder by profit: $0-$1,500: 2; $1,501-$2,000: 4; $2,001+: 6. Exceeding it is a hard fail; up to ten micros count as one standard contract." }
min_trading_days: null            # no minimum published
trading_day_reset: { time: "17:00", tz_as_published: "CT" }
commission_per_contract: { mnq_usd: null, nq_usd: null, basis: per_side, includes: "Published as 'Total Per Side' in the firm's per-platform chart; what it includes is not stated. Platform-dependent NQ per side: Rithmic 2.04, BlackArrow 2.04, Tradovate/NinjaTrader 2.88. MNQ is not listed (text only: micro contracts range from 0.82 to 1.20 per contract per side)" }
copy_trading_allowed: no
automated_trading_allowed: conditional
automation_rule_text: "Utilizing software, artificial intelligence, or ultra-fast data entry techniques that could potentially manipulate the trading environment or provide an unfair advantage"
vps_allowed: unknown
personal_device_required: unknown
status: partial
not_confirmed_fields: [commission_per_contract.mnq_usd, commission_per_contract.nq_usd, vps_allowed, personal_device_required, automated_trading_allowed, daily_loss_limit.kind]
notes: "Live brokerage account (real capital via the prop firm's FCM). Reached via: TCP25 step 2 and TCP50 start. Profit target to advance: $3000.00 (not an evaluation target). Max 1 Live account at a time. Daily loss limit behavior on funded accounts is not stated. Live data fees: $140 per exchange per month (Rithmic), $156 (NinjaTrader). Size 200K values (and 150K) come from the site's JS data and the Progression Ladder image; the TCP page growth card shows 25K/50K/100K/200K."
sources:
  - { field: drawdown, url: "https://help.earn2trade.com/en/articles/3224507-what-is-the-maximum-drawdown-on-the-trader-career-path-and-the-gauntlet-mini-challenges", accessed: "2026-10-23" }
  - { field: drawdown_types, url: "https://help.earn2trade.com/en/articles/8117088-drawdown-types", accessed: "2026-10-23" }
  - { field: daily_loss_limit, url: "https://help.earn2trade.com/en/articles/3395926-how-is-my-daily-loss-calculated", accessed: "2026-10-23" }
  - { field: profit_target_to_advance, url: "https://www.earn2trade.com/trader-career-path", accessed: "2026-10-23" }
  - { field: consistency_rule, url: "https://help.earn2trade.com/en/articles/6877364-what-happens-when-i-complete-my-evaluation", accessed: "2026-10-23" }
  - { field: max_position, url: "https://help.earn2trade.com/en/articles/5922873-what-is-the-progression-ladder-on-the-trader-career-path", accessed: "2026-10-23" }
  - { field: trading_day_reset, url: "https://help.earn2trade.com/en/articles/3395926-how-is-my-daily-loss-calculated", accessed: "2026-10-23" }
  - { field: copy_trading_allowed, url: "https://help.earn2trade.com/en/articles/12034590-am-i-allowed-to-copy-trades-across-multiple-accounts", accessed: "2026-10-23" }
  - { field: automated_trading_allowed, url: "https://help.earn2trade.com/en/articles/9286647-prohibited-conduct-in-earn2trade-evaluations-and-in-the-livesim-live-trading-environment", accessed: "2026-10-23" }
  - { field: commission_per_contract, url: "https://help.earn2trade.com/en/articles/2425493-what-is-the-commission-on-earn2trade-evaluations", accessed: "2026-10-23" }
  - { field: platforms, url: "https://help.earn2trade.com/en/articles/2090521-what-platforms-can-i-use-for-the-gauntlet-mini-trader-career-path", accessed: "2026-10-23" }
  - { field: growth_plan_values, url: "https://www.earn2trade.com/trader-career-path", accessed: "2026-10-23" }
  - { field: growth_plan_values_js, url: "https://www.earn2trade.com/_next/static/chunks/0d45t7274e6.p.js", accessed: "2026-10-23" }
```

#### Trader Career Path — Live (funded) — 100K

```yaml
firm: earn2trade
plan: "Trader Career Path (growth plan Live 100K)"
phase: live
size_label: "100K"
size_usd: "100000.00"
instruments: [MNQ, NQ]
platforms: ["NinjaTrader desktop (live credentials from the prop firm)", "Rithmic (R|Trader Pro and compatible platforms)"]
drawdown: { amount_usd: "3500.00", type: trailing_intraday, stops_trailing_at: "Trails the highest balance intraday and locks at the starting balance ($100,000)" }
daily_loss_limit: { amount_usd: "2200.00", kind: unknown }
profit_target_usd: "none"
consistency_rule: { text: "none (published: No Consistency Requirement)", threshold_pct: null }
max_position: { minis: 12, micros: 120, scaling: "Progression Ladder by profit: $0-$1,500: 3; $1,501-$3,000: 6; $3,001-$4,500: 9; $4,501+: 12. Exceeding it is a hard fail; up to ten micros count as one standard contract." }
min_trading_days: null            # no minimum published
trading_day_reset: { time: "17:00", tz_as_published: "CT" }
commission_per_contract: { mnq_usd: null, nq_usd: null, basis: per_side, includes: "Published as 'Total Per Side' in the firm's per-platform chart; what it includes is not stated. Platform-dependent NQ per side: Rithmic 2.04, BlackArrow 2.04, Tradovate/NinjaTrader 2.88. MNQ is not listed (text only: micro contracts range from 0.82 to 1.20 per contract per side)" }
copy_trading_allowed: no
automated_trading_allowed: conditional
automation_rule_text: "Utilizing software, artificial intelligence, or ultra-fast data entry techniques that could potentially manipulate the trading environment or provide an unfair advantage"
vps_allowed: unknown
personal_device_required: unknown
status: partial
not_confirmed_fields: [commission_per_contract.mnq_usd, commission_per_contract.nq_usd, vps_allowed, personal_device_required, automated_trading_allowed, daily_loss_limit.kind]
notes: "Live brokerage account (real capital via the prop firm's FCM). Reached via: TCP25 step 3, TCP50 step 2 and TCP100 start. Profit target to advance: $6000.00 (not an evaluation target). Max 1 Live account at a time. Daily loss limit behavior on funded accounts is not stated. Live data fees: $140 per exchange per month (Rithmic), $156 (NinjaTrader). Size 200K values (and 150K) come from the site's JS data and the Progression Ladder image; the TCP page growth card shows 25K/50K/100K/200K."
sources:
  - { field: drawdown, url: "https://help.earn2trade.com/en/articles/3224507-what-is-the-maximum-drawdown-on-the-trader-career-path-and-the-gauntlet-mini-challenges", accessed: "2026-10-23" }
  - { field: drawdown_types, url: "https://help.earn2trade.com/en/articles/8117088-drawdown-types", accessed: "2026-10-23" }
  - { field: daily_loss_limit, url: "https://help.earn2trade.com/en/articles/3395926-how-is-my-daily-loss-calculated", accessed: "2026-10-23" }
  - { field: profit_target_to_advance, url: "https://www.earn2trade.com/trader-career-path", accessed: "2026-10-23" }
  - { field: consistency_rule, url: "https://help.earn2trade.com/en/articles/6877364-what-happens-when-i-complete-my-evaluation", accessed: "2026-10-23" }
  - { field: max_position, url: "https://help.earn2trade.com/en/articles/5922873-what-is-the-progression-ladder-on-the-trader-career-path", accessed: "2026-10-23" }
  - { field: trading_day_reset, url: "https://help.earn2trade.com/en/articles/3395926-how-is-my-daily-loss-calculated", accessed: "2026-10-23" }
  - { field: copy_trading_allowed, url: "https://help.earn2trade.com/en/articles/12034590-am-i-allowed-to-copy-trades-across-multiple-accounts", accessed: "2026-10-23" }
  - { field: automated_trading_allowed, url: "https://help.earn2trade.com/en/articles/9286647-prohibited-conduct-in-earn2trade-evaluations-and-in-the-livesim-live-trading-environment", accessed: "2026-10-23" }
  - { field: commission_per_contract, url: "https://help.earn2trade.com/en/articles/2425493-what-is-the-commission-on-earn2trade-evaluations", accessed: "2026-10-23" }
  - { field: platforms, url: "https://help.earn2trade.com/en/articles/2090521-what-platforms-can-i-use-for-the-gauntlet-mini-trader-career-path", accessed: "2026-10-23" }
  - { field: growth_plan_values, url: "https://www.earn2trade.com/trader-career-path", accessed: "2026-10-23" }
  - { field: growth_plan_values_js, url: "https://www.earn2trade.com/_next/static/chunks/0d45t7274e6.p.js", accessed: "2026-10-23" }
```

#### Trader Career Path — Live (funded) — 150K

```yaml
firm: earn2trade
plan: "Trader Career Path (growth plan Live 150K)"
phase: live
size_label: "150K"
size_usd: "150000.00"
instruments: [MNQ, NQ]
platforms: ["NinjaTrader desktop (live credentials from the prop firm)", "Rithmic (R|Trader Pro and compatible platforms)"]
drawdown: { amount_usd: "4500.00", type: trailing_intraday, stops_trailing_at: "Trails the highest balance intraday and locks at the starting balance ($150,000)" }
daily_loss_limit: { amount_usd: "3300.00", kind: unknown }
profit_target_usd: "none"
consistency_rule: { text: "none (published: No Consistency Requirement)", threshold_pct: null }
max_position: { minis: 15, micros: 150, scaling: "Progression Ladder by profit: $0-$1,500: 4; $1,501-$3,000: 6; $3,001-$4,500: 9; $4,501+: 15. Exceeding it is a hard fail; up to ten micros count as one standard contract." }
min_trading_days: null            # no minimum published
trading_day_reset: { time: "17:00", tz_as_published: "CT" }
commission_per_contract: { mnq_usd: null, nq_usd: null, basis: per_side, includes: "Published as 'Total Per Side' in the firm's per-platform chart; what it includes is not stated. Platform-dependent NQ per side: Rithmic 2.04, BlackArrow 2.04, Tradovate/NinjaTrader 2.88. MNQ is not listed (text only: micro contracts range from 0.82 to 1.20 per contract per side)" }
copy_trading_allowed: no
automated_trading_allowed: conditional
automation_rule_text: "Utilizing software, artificial intelligence, or ultra-fast data entry techniques that could potentially manipulate the trading environment or provide an unfair advantage"
vps_allowed: unknown
personal_device_required: unknown
status: partial
not_confirmed_fields: [commission_per_contract.mnq_usd, commission_per_contract.nq_usd, vps_allowed, personal_device_required, automated_trading_allowed, daily_loss_limit.kind]
notes: "Live brokerage account (real capital via the prop firm's FCM). Reached via: TCP100 step 2. Profit target to advance: $9000.00 (not an evaluation target). Max 1 Live account at a time. Daily loss limit behavior on funded accounts is not stated. Live data fees: $140 per exchange per month (Rithmic), $156 (NinjaTrader). Size 200K values (and 150K) come from the site's JS data and the Progression Ladder image; the TCP page growth card shows 25K/50K/100K/200K."
sources:
  - { field: drawdown, url: "https://help.earn2trade.com/en/articles/3224507-what-is-the-maximum-drawdown-on-the-trader-career-path-and-the-gauntlet-mini-challenges", accessed: "2026-10-23" }
  - { field: drawdown_types, url: "https://help.earn2trade.com/en/articles/8117088-drawdown-types", accessed: "2026-10-23" }
  - { field: daily_loss_limit, url: "https://help.earn2trade.com/en/articles/3395926-how-is-my-daily-loss-calculated", accessed: "2026-10-23" }
  - { field: profit_target_to_advance, url: "https://www.earn2trade.com/trader-career-path", accessed: "2026-10-23" }
  - { field: consistency_rule, url: "https://help.earn2trade.com/en/articles/6877364-what-happens-when-i-complete-my-evaluation", accessed: "2026-10-23" }
  - { field: max_position, url: "https://help.earn2trade.com/en/articles/5922873-what-is-the-progression-ladder-on-the-trader-career-path", accessed: "2026-10-23" }
  - { field: trading_day_reset, url: "https://help.earn2trade.com/en/articles/3395926-how-is-my-daily-loss-calculated", accessed: "2026-10-23" }
  - { field: copy_trading_allowed, url: "https://help.earn2trade.com/en/articles/12034590-am-i-allowed-to-copy-trades-across-multiple-accounts", accessed: "2026-10-23" }
  - { field: automated_trading_allowed, url: "https://help.earn2trade.com/en/articles/9286647-prohibited-conduct-in-earn2trade-evaluations-and-in-the-livesim-live-trading-environment", accessed: "2026-10-23" }
  - { field: commission_per_contract, url: "https://help.earn2trade.com/en/articles/2425493-what-is-the-commission-on-earn2trade-evaluations", accessed: "2026-10-23" }
  - { field: platforms, url: "https://help.earn2trade.com/en/articles/2090521-what-platforms-can-i-use-for-the-gauntlet-mini-trader-career-path", accessed: "2026-10-23" }
  - { field: growth_plan_values, url: "https://www.earn2trade.com/trader-career-path", accessed: "2026-10-23" }
  - { field: growth_plan_values_js, url: "https://www.earn2trade.com/_next/static/chunks/0d45t7274e6.p.js", accessed: "2026-10-23" }
```

#### Trader Career Path — Live (funded) — 200K

```yaml
firm: earn2trade
plan: "Trader Career Path (growth plan Live 200K)"
phase: live
size_label: "200K"
size_usd: "200000.00"
instruments: [MNQ, NQ]
platforms: ["NinjaTrader desktop (live credentials from the prop firm)", "Rithmic (R|Trader Pro and compatible platforms)"]
drawdown: { amount_usd: "6000.00", type: trailing_intraday, stops_trailing_at: "Trails the highest balance intraday and locks at the starting balance ($200,000)" }
daily_loss_limit: { amount_usd: "4400.00", kind: unknown }
profit_target_usd: "none"
consistency_rule: { text: "none (published: No Consistency Requirement)", threshold_pct: null }
max_position: { minis: 16, micros: 160, scaling: "Progression Ladder by profit: $0-$1,500: 5; $1,501-$3,000: 7; $3,001-$4,500: 10; $4,501+: 16. Exceeding it is a hard fail; up to ten micros count as one standard contract." }
min_trading_days: null            # no minimum published
trading_day_reset: { time: "17:00", tz_as_published: "CT" }
commission_per_contract: { mnq_usd: null, nq_usd: null, basis: per_side, includes: "Published as 'Total Per Side' in the firm's per-platform chart; what it includes is not stated. Platform-dependent NQ per side: Rithmic 2.04, BlackArrow 2.04, Tradovate/NinjaTrader 2.88. MNQ is not listed (text only: micro contracts range from 0.82 to 1.20 per contract per side)" }
copy_trading_allowed: no
automated_trading_allowed: conditional
automation_rule_text: "Utilizing software, artificial intelligence, or ultra-fast data entry techniques that could potentially manipulate the trading environment or provide an unfair advantage"
vps_allowed: unknown
personal_device_required: unknown
status: partial
not_confirmed_fields: [commission_per_contract.mnq_usd, commission_per_contract.nq_usd, vps_allowed, personal_device_required, automated_trading_allowed, daily_loss_limit.kind]
notes: "Live brokerage account (real capital via the prop firm's FCM). Reached via: TCP50 step 3 and TCP100 step 3 (trailing, not fixed). Profit target to advance: $11000.00 (not an evaluation target). Max 1 Live account at a time. Daily loss limit behavior on funded accounts is not stated. Live data fees: $140 per exchange per month (Rithmic), $156 (NinjaTrader). Size 200K values (and 150K) come from the site's JS data and the Progression Ladder image; the TCP page growth card shows 25K/50K/100K/200K."
sources:
  - { field: drawdown, url: "https://help.earn2trade.com/en/articles/3224507-what-is-the-maximum-drawdown-on-the-trader-career-path-and-the-gauntlet-mini-challenges", accessed: "2026-10-23" }
  - { field: drawdown_types, url: "https://help.earn2trade.com/en/articles/8117088-drawdown-types", accessed: "2026-10-23" }
  - { field: daily_loss_limit, url: "https://help.earn2trade.com/en/articles/3395926-how-is-my-daily-loss-calculated", accessed: "2026-10-23" }
  - { field: profit_target_to_advance, url: "https://www.earn2trade.com/trader-career-path", accessed: "2026-10-23" }
  - { field: consistency_rule, url: "https://help.earn2trade.com/en/articles/6877364-what-happens-when-i-complete-my-evaluation", accessed: "2026-10-23" }
  - { field: max_position, url: "https://help.earn2trade.com/en/articles/5922873-what-is-the-progression-ladder-on-the-trader-career-path", accessed: "2026-10-23" }
  - { field: trading_day_reset, url: "https://help.earn2trade.com/en/articles/3395926-how-is-my-daily-loss-calculated", accessed: "2026-10-23" }
  - { field: copy_trading_allowed, url: "https://help.earn2trade.com/en/articles/12034590-am-i-allowed-to-copy-trades-across-multiple-accounts", accessed: "2026-10-23" }
  - { field: automated_trading_allowed, url: "https://help.earn2trade.com/en/articles/9286647-prohibited-conduct-in-earn2trade-evaluations-and-in-the-livesim-live-trading-environment", accessed: "2026-10-23" }
  - { field: commission_per_contract, url: "https://help.earn2trade.com/en/articles/2425493-what-is-the-commission-on-earn2trade-evaluations", accessed: "2026-10-23" }
  - { field: platforms, url: "https://help.earn2trade.com/en/articles/2090521-what-platforms-can-i-use-for-the-gauntlet-mini-trader-career-path", accessed: "2026-10-23" }
  - { field: growth_plan_values, url: "https://www.earn2trade.com/trader-career-path", accessed: "2026-10-23" }
  - { field: growth_plan_values_js, url: "https://www.earn2trade.com/_next/static/chunks/0d45t7274e6.p.js", accessed: "2026-10-23" }
```

#### Trader Career Path — Live (funded) — 200K fixed (via TCP25)

```yaml
firm: earn2trade
plan: "Trader Career Path (growth plan Live 200K, TCP25 path)"
phase: live
size_label: "200K"
size_usd: "200000.00"
instruments: [MNQ, NQ]
platforms: ["NinjaTrader desktop (live credentials from the prop firm)", "Rithmic (R|Trader Pro and compatible platforms)"]
drawdown: { amount_usd: null, type: static, stops_trailing_at: "Drawdown fixed: minimum balance 194000.00 (published as 'Drawdown fixed to: $194,000')" }
daily_loss_limit: { amount_usd: "4400.00", kind: unknown }
profit_target_usd: "none"
consistency_rule: { text: "none (published: No Consistency Requirement)", threshold_pct: null }
max_position: { minis: 16, micros: 160, scaling: "Progression Ladder by profit: $0-$1,500: 5; $1,501-$3,000: 7; $3,001-$4,500: 10; $4,501+: 16. Exceeding it is a hard fail; up to ten micros count as one standard contract." }
min_trading_days: null            # no minimum published
trading_day_reset: { time: "17:00", tz_as_published: "CT" }
commission_per_contract: { mnq_usd: null, nq_usd: null, basis: per_side, includes: "Published as 'Total Per Side' in the firm's per-platform chart; what it includes is not stated. Platform-dependent NQ per side: Rithmic 2.04, BlackArrow 2.04, Tradovate/NinjaTrader 2.88. MNQ is not listed (text only: micro contracts range from 0.82 to 1.20 per contract per side)" }
copy_trading_allowed: no
automated_trading_allowed: conditional
automation_rule_text: "Utilizing software, artificial intelligence, or ultra-fast data entry techniques that could potentially manipulate the trading environment or provide an unfair advantage"
vps_allowed: unknown
personal_device_required: unknown
status: partial
not_confirmed_fields: [commission_per_contract.mnq_usd, commission_per_contract.nq_usd, vps_allowed, personal_device_required, automated_trading_allowed, daily_loss_limit.kind, drawdown.amount_usd]
notes: "Reached via TCP25 step 4. Fixed minimum balance $194,000 is published; the drawdown amount itself is not published (arithmetic 200000-194000 not recorded as a value). Daily loss limit behavior on funded accounts is not stated. "
sources:
  - { field: drawdown, url: "https://help.earn2trade.com/en/articles/3224507-what-is-the-maximum-drawdown-on-the-trader-career-path-and-the-gauntlet-mini-challenges", accessed: "2026-10-23" }
  - { field: drawdown_types, url: "https://help.earn2trade.com/en/articles/8117088-drawdown-types", accessed: "2026-10-23" }
  - { field: daily_loss_limit, url: "https://help.earn2trade.com/en/articles/3395926-how-is-my-daily-loss-calculated", accessed: "2026-10-23" }
  - { field: profit_target_to_advance, url: "https://www.earn2trade.com/trader-career-path", accessed: "2026-10-23" }
  - { field: consistency_rule, url: "https://help.earn2trade.com/en/articles/6877364-what-happens-when-i-complete-my-evaluation", accessed: "2026-10-23" }
  - { field: max_position, url: "https://help.earn2trade.com/en/articles/5922873-what-is-the-progression-ladder-on-the-trader-career-path", accessed: "2026-10-23" }
  - { field: trading_day_reset, url: "https://help.earn2trade.com/en/articles/3395926-how-is-my-daily-loss-calculated", accessed: "2026-10-23" }
  - { field: copy_trading_allowed, url: "https://help.earn2trade.com/en/articles/12034590-am-i-allowed-to-copy-trades-across-multiple-accounts", accessed: "2026-10-23" }
  - { field: automated_trading_allowed, url: "https://help.earn2trade.com/en/articles/9286647-prohibited-conduct-in-earn2trade-evaluations-and-in-the-livesim-live-trading-environment", accessed: "2026-10-23" }
  - { field: commission_per_contract, url: "https://help.earn2trade.com/en/articles/2425493-what-is-the-commission-on-earn2trade-evaluations", accessed: "2026-10-23" }
  - { field: platforms, url: "https://help.earn2trade.com/en/articles/2090521-what-platforms-can-i-use-for-the-gauntlet-mini-trader-career-path", accessed: "2026-10-23" }
  - { field: fixed_drawdown, url: "https://help.earn2trade.com/en/articles/8129235-what-is-a-fixed-drawdown", accessed: "2026-10-23" }
  - { field: growth_plan_values, url: "https://www.earn2trade.com/trader-career-path", accessed: "2026-10-23" }
```

#### Trader Career Path — Live (funded) — 400K fixed (via TCP50 / TCP100)

```yaml
firm: earn2trade
plan: "Trader Career Path (growth plan Live 400K)"
phase: live
size_label: "400K"
size_usd: "400000.00"
instruments: [MNQ, NQ]
platforms: ["NinjaTrader desktop (live credentials from the prop firm)", "Rithmic (R|Trader Pro and compatible platforms)"]
drawdown: { amount_usd: null, type: static, stops_trailing_at: "Drawdown fixed: minimum balance 388000.00 (published: 'drawdown fixed at $388,000')" }
daily_loss_limit: { amount_usd: "8800.00", kind: unknown }
profit_target_usd: "none"
consistency_rule: { text: "none (published: No Consistency Requirement)", threshold_pct: null }
max_position: { minis: 30, micros: 300, scaling: "Progression Ladder by profit: $0-$1,500: 8; $1,501-$3,000: 12; $3,001-$4,500: 18; $4,501+: 30. Exceeding it is a hard fail; up to ten micros count as one standard contract." }
min_trading_days: null            # no minimum published
trading_day_reset: { time: "17:00", tz_as_published: "CT" }
commission_per_contract: { mnq_usd: null, nq_usd: null, basis: per_side, includes: "Published as 'Total Per Side' in the firm's per-platform chart; what it includes is not stated. Platform-dependent NQ per side: Rithmic 2.04, BlackArrow 2.04, Tradovate/NinjaTrader 2.88. MNQ is not listed (text only: micro contracts range from 0.82 to 1.20 per contract per side)" }
copy_trading_allowed: no
automated_trading_allowed: conditional
automation_rule_text: "Utilizing software, artificial intelligence, or ultra-fast data entry techniques that could potentially manipulate the trading environment or provide an unfair advantage"
vps_allowed: unknown
personal_device_required: unknown
status: partial
not_confirmed_fields: [commission_per_contract.mnq_usd, commission_per_contract.nq_usd, vps_allowed, personal_device_required, automated_trading_allowed, daily_loss_limit.kind, drawdown.amount_usd]
notes: "Reached via TCP50 step 4 or TCP100 step 4. Fixed minimum balance $388,000 is published; the drawdown amount itself is not published. Daily loss limit 8800 and 400K ladder come from the site JS data and the Progression Ladder image (the TCP page text does not list 400K). Fixed 60/40 profit split. Daily loss limit behavior on funded accounts is not stated."
sources:
  - { field: drawdown, url: "https://help.earn2trade.com/en/articles/3224507-what-is-the-maximum-drawdown-on-the-trader-career-path-and-the-gauntlet-mini-challenges", accessed: "2026-10-23" }
  - { field: drawdown_types, url: "https://help.earn2trade.com/en/articles/8117088-drawdown-types", accessed: "2026-10-23" }
  - { field: daily_loss_limit, url: "https://help.earn2trade.com/en/articles/3395926-how-is-my-daily-loss-calculated", accessed: "2026-10-23" }
  - { field: profit_target_to_advance, url: "https://www.earn2trade.com/trader-career-path", accessed: "2026-10-23" }
  - { field: consistency_rule, url: "https://help.earn2trade.com/en/articles/6877364-what-happens-when-i-complete-my-evaluation", accessed: "2026-10-23" }
  - { field: max_position, url: "https://help.earn2trade.com/en/articles/5922873-what-is-the-progression-ladder-on-the-trader-career-path", accessed: "2026-10-23" }
  - { field: trading_day_reset, url: "https://help.earn2trade.com/en/articles/3395926-how-is-my-daily-loss-calculated", accessed: "2026-10-23" }
  - { field: copy_trading_allowed, url: "https://help.earn2trade.com/en/articles/12034590-am-i-allowed-to-copy-trades-across-multiple-accounts", accessed: "2026-10-23" }
  - { field: automated_trading_allowed, url: "https://help.earn2trade.com/en/articles/9286647-prohibited-conduct-in-earn2trade-evaluations-and-in-the-livesim-live-trading-environment", accessed: "2026-10-23" }
  - { field: commission_per_contract, url: "https://help.earn2trade.com/en/articles/2425493-what-is-the-commission-on-earn2trade-evaluations", accessed: "2026-10-23" }
  - { field: platforms, url: "https://help.earn2trade.com/en/articles/2090521-what-platforms-can-i-use-for-the-gauntlet-mini-trader-career-path", accessed: "2026-10-23" }
  - { field: fixed_drawdown, url: "https://help.earn2trade.com/en/articles/8129235-what-is-a-fixed-drawdown", accessed: "2026-10-23" }
  - { field: daily_loss_limit_js, url: "https://www.earn2trade.com/_next/static/chunks/0d45t7274e6.p.js", accessed: "2026-10-23" }
```

### Gauntlet Mini

#### Gauntlet Mini — evaluation — 50K

```yaml
firm: earn2trade
plan: "The Gauntlet Mini (GAU50)"
phase: evaluation
size_label: "50K"
size_usd: "50000.00"
instruments: [MNQ, NQ]
platforms: ["Rithmic (Finamark, R|Trader Pro and more)", "BlackArrow", "NinjaTrader (via Tradovate)", "Tradovate", "TradingView (via Tradovate)", "dxFeed/DeepCharts"]
drawdown: { amount_usd: "2000.00", type: trailing_eod, stops_trailing_at: "Minimum balance rises with each end-of-day balance high and stops rising once it reaches the starting balance ($50,000)" }
daily_loss_limit: { amount_usd: "1100.00", kind: hard }
profit_target_usd: "3000.00"
consistency_rule: { text: "No single trading day can account for 30% or more of total PnL over the evaluation (the evaluation continues until the day is below 30% of total profit; it is not an instant fail).", threshold_pct: "30" }
max_position: { minis: 6, micros: 60, scaling: "Progression Ladder by profit: $0-$1,500: 2; $1,501-$2,000: 4; $2,001+: 6. Exceeding it is a hard fail; up to ten micros count as one standard contract." }
min_trading_days: null            # no minimum published
trading_day_reset: { time: "17:00", tz_as_published: "CT" }
commission_per_contract: { mnq_usd: null, nq_usd: null, basis: per_side, includes: "Published as 'Total Per Side' in the firm's per-platform chart; what it includes is not stated. Platform-dependent NQ per side: Rithmic 2.04, BlackArrow 2.04, Tradovate/NinjaTrader 2.88. MNQ is not listed (text only: micro contracts range from 0.82 to 1.20 per contract per side)" }
copy_trading_allowed: no
automated_trading_allowed: conditional
automation_rule_text: "Utilizing software, artificial intelligence, or ultra-fast data entry techniques that could potentially manipulate the trading environment or provide an unfair advantage"
vps_allowed: unknown
personal_device_required: unknown
status: partial
not_confirmed_fields: [commission_per_contract.mnq_usd, commission_per_contract.nq_usd, vps_allowed, personal_device_required, automated_trading_allowed]
notes: "Single-phase evaluation, no time limit, no minimum trading days. Daily loss breach fails the evaluation (hard). Must be flat by 15:50 CT. Max 5 concurrent evaluation accounts. Only GAU50 values were rendered in the page text; GAU100/150/200 come from the site's JS data and agree with the help-center drawdown image and ladder image."
sources:
  - { field: drawdown, url: "https://help.earn2trade.com/en/articles/3224507-what-is-the-maximum-drawdown-on-the-trader-career-path-and-the-gauntlet-mini-challenges", accessed: "2026-10-23" }
  - { field: drawdown_rule, url: "https://help.earn2trade.com/en/articles/5372687-how-does-end-of-day-drawdown-work", accessed: "2026-10-23" }
  - { field: daily_loss_limit, url: "https://help.earn2trade.com/en/articles/3395926-how-is-my-daily-loss-calculated", accessed: "2026-10-23" }
  - { field: profit_target_usd, url: "https://www.earn2trade.com/gauntlet-mini", accessed: "2026-10-23" }
  - { field: profit_target_usd_all_sizes, url: "https://www.earn2trade.com/_next/static/chunks/0d45t7274e6.p.js", accessed: "2026-10-23" }
  - { field: consistency_rule, url: "https://help.earn2trade.com/en/articles/3849975-what-is-the-maintain-consistency-rule", accessed: "2026-10-23" }
  - { field: max_position, url: "https://help.earn2trade.com/en/articles/3292351-what-is-the-progression-ladder-in-the-gauntlet-mini", accessed: "2026-10-23" }
  - { field: trading_day_reset, url: "https://help.earn2trade.com/en/articles/3395926-how-is-my-daily-loss-calculated", accessed: "2026-10-23" }
  - { field: copy_trading_allowed, url: "https://help.earn2trade.com/en/articles/12034590-am-i-allowed-to-copy-trades-across-multiple-accounts", accessed: "2026-10-23" }
  - { field: automated_trading_allowed, url: "https://help.earn2trade.com/en/articles/9286647-prohibited-conduct-in-earn2trade-evaluations-and-in-the-livesim-live-trading-environment", accessed: "2026-10-23" }
  - { field: commission_per_contract, url: "https://help.earn2trade.com/en/articles/2425493-what-is-the-commission-on-earn2trade-evaluations", accessed: "2026-10-23" }
  - { field: platforms, url: "https://help.earn2trade.com/en/articles/2090521-what-platforms-can-i-use-for-the-gauntlet-mini-trader-career-path", accessed: "2026-10-23" }
```

#### Gauntlet Mini — evaluation — 100K

```yaml
firm: earn2trade
plan: "The Gauntlet Mini (GAU100)"
phase: evaluation
size_label: "100K"
size_usd: "100000.00"
instruments: [MNQ, NQ]
platforms: ["Rithmic (Finamark, R|Trader Pro and more)", "BlackArrow", "NinjaTrader (via Tradovate)", "Tradovate", "TradingView (via Tradovate)", "dxFeed/DeepCharts"]
drawdown: { amount_usd: "3500.00", type: trailing_eod, stops_trailing_at: "Minimum balance rises with each end-of-day balance high and stops rising once it reaches the starting balance ($100,000)" }
daily_loss_limit: { amount_usd: "2200.00", kind: hard }
profit_target_usd: "6000.00"
consistency_rule: { text: "No single trading day can account for 30% or more of total PnL over the evaluation (the evaluation continues until the day is below 30% of total profit; it is not an instant fail).", threshold_pct: "30" }
max_position: { minis: 12, micros: 120, scaling: "Progression Ladder by profit: $0-$1,500: 3; $1,501-$3,000: 6; $3,001-$4,500: 9; $4,501+: 12. Exceeding it is a hard fail; up to ten micros count as one standard contract." }
min_trading_days: null            # no minimum published
trading_day_reset: { time: "17:00", tz_as_published: "CT" }
commission_per_contract: { mnq_usd: null, nq_usd: null, basis: per_side, includes: "Published as 'Total Per Side' in the firm's per-platform chart; what it includes is not stated. Platform-dependent NQ per side: Rithmic 2.04, BlackArrow 2.04, Tradovate/NinjaTrader 2.88. MNQ is not listed (text only: micro contracts range from 0.82 to 1.20 per contract per side)" }
copy_trading_allowed: no
automated_trading_allowed: conditional
automation_rule_text: "Utilizing software, artificial intelligence, or ultra-fast data entry techniques that could potentially manipulate the trading environment or provide an unfair advantage"
vps_allowed: unknown
personal_device_required: unknown
status: partial
not_confirmed_fields: [commission_per_contract.mnq_usd, commission_per_contract.nq_usd, vps_allowed, personal_device_required, automated_trading_allowed]
notes: "Single-phase evaluation, no time limit, no minimum trading days. Daily loss breach fails the evaluation (hard). Must be flat by 15:50 CT. Max 5 concurrent evaluation accounts. Only GAU50 values were rendered in the page text; GAU100/150/200 come from the site's JS data and agree with the help-center drawdown image and ladder image."
sources:
  - { field: drawdown, url: "https://help.earn2trade.com/en/articles/3224507-what-is-the-maximum-drawdown-on-the-trader-career-path-and-the-gauntlet-mini-challenges", accessed: "2026-10-23" }
  - { field: drawdown_rule, url: "https://help.earn2trade.com/en/articles/5372687-how-does-end-of-day-drawdown-work", accessed: "2026-10-23" }
  - { field: daily_loss_limit, url: "https://help.earn2trade.com/en/articles/3395926-how-is-my-daily-loss-calculated", accessed: "2026-10-23" }
  - { field: profit_target_usd, url: "https://www.earn2trade.com/gauntlet-mini", accessed: "2026-10-23" }
  - { field: profit_target_usd_all_sizes, url: "https://www.earn2trade.com/_next/static/chunks/0d45t7274e6.p.js", accessed: "2026-10-23" }
  - { field: consistency_rule, url: "https://help.earn2trade.com/en/articles/3849975-what-is-the-maintain-consistency-rule", accessed: "2026-10-23" }
  - { field: max_position, url: "https://help.earn2trade.com/en/articles/3292351-what-is-the-progression-ladder-in-the-gauntlet-mini", accessed: "2026-10-23" }
  - { field: trading_day_reset, url: "https://help.earn2trade.com/en/articles/3395926-how-is-my-daily-loss-calculated", accessed: "2026-10-23" }
  - { field: copy_trading_allowed, url: "https://help.earn2trade.com/en/articles/12034590-am-i-allowed-to-copy-trades-across-multiple-accounts", accessed: "2026-10-23" }
  - { field: automated_trading_allowed, url: "https://help.earn2trade.com/en/articles/9286647-prohibited-conduct-in-earn2trade-evaluations-and-in-the-livesim-live-trading-environment", accessed: "2026-10-23" }
  - { field: commission_per_contract, url: "https://help.earn2trade.com/en/articles/2425493-what-is-the-commission-on-earn2trade-evaluations", accessed: "2026-10-23" }
  - { field: platforms, url: "https://help.earn2trade.com/en/articles/2090521-what-platforms-can-i-use-for-the-gauntlet-mini-trader-career-path", accessed: "2026-10-23" }
```

#### Gauntlet Mini — evaluation — 150K

```yaml
firm: earn2trade
plan: "The Gauntlet Mini (GAU150)"
phase: evaluation
size_label: "150K"
size_usd: "150000.00"
instruments: [MNQ, NQ]
platforms: ["Rithmic (Finamark, R|Trader Pro and more)", "BlackArrow", "NinjaTrader (via Tradovate)", "Tradovate", "TradingView (via Tradovate)", "dxFeed/DeepCharts"]
drawdown: { amount_usd: "4500.00", type: trailing_eod, stops_trailing_at: "Minimum balance rises with each end-of-day balance high and stops rising once it reaches the starting balance ($150,000)" }
daily_loss_limit: { amount_usd: "3300.00", kind: hard }
profit_target_usd: "9000.00"
consistency_rule: { text: "No single trading day can account for 30% or more of total PnL over the evaluation (the evaluation continues until the day is below 30% of total profit; it is not an instant fail).", threshold_pct: "30" }
max_position: { minis: 15, micros: 150, scaling: "Progression Ladder by profit: $0-$1,500: 4; $1,501-$3,000: 6; $3,001-$4,500: 9; $4,501+: 15. Exceeding it is a hard fail; up to ten micros count as one standard contract." }
min_trading_days: null            # no minimum published
trading_day_reset: { time: "17:00", tz_as_published: "CT" }
commission_per_contract: { mnq_usd: null, nq_usd: null, basis: per_side, includes: "Published as 'Total Per Side' in the firm's per-platform chart; what it includes is not stated. Platform-dependent NQ per side: Rithmic 2.04, BlackArrow 2.04, Tradovate/NinjaTrader 2.88. MNQ is not listed (text only: micro contracts range from 0.82 to 1.20 per contract per side)" }
copy_trading_allowed: no
automated_trading_allowed: conditional
automation_rule_text: "Utilizing software, artificial intelligence, or ultra-fast data entry techniques that could potentially manipulate the trading environment or provide an unfair advantage"
vps_allowed: unknown
personal_device_required: unknown
status: partial
not_confirmed_fields: [commission_per_contract.mnq_usd, commission_per_contract.nq_usd, vps_allowed, personal_device_required, automated_trading_allowed]
notes: "Single-phase evaluation, no time limit, no minimum trading days. Daily loss breach fails the evaluation (hard). Must be flat by 15:50 CT. Max 5 concurrent evaluation accounts. Only GAU50 values were rendered in the page text; GAU100/150/200 come from the site's JS data and agree with the help-center drawdown image and ladder image."
sources:
  - { field: drawdown, url: "https://help.earn2trade.com/en/articles/3224507-what-is-the-maximum-drawdown-on-the-trader-career-path-and-the-gauntlet-mini-challenges", accessed: "2026-10-23" }
  - { field: drawdown_rule, url: "https://help.earn2trade.com/en/articles/5372687-how-does-end-of-day-drawdown-work", accessed: "2026-10-23" }
  - { field: daily_loss_limit, url: "https://help.earn2trade.com/en/articles/3395926-how-is-my-daily-loss-calculated", accessed: "2026-10-23" }
  - { field: profit_target_usd, url: "https://www.earn2trade.com/gauntlet-mini", accessed: "2026-10-23" }
  - { field: profit_target_usd_all_sizes, url: "https://www.earn2trade.com/_next/static/chunks/0d45t7274e6.p.js", accessed: "2026-10-23" }
  - { field: consistency_rule, url: "https://help.earn2trade.com/en/articles/3849975-what-is-the-maintain-consistency-rule", accessed: "2026-10-23" }
  - { field: max_position, url: "https://help.earn2trade.com/en/articles/3292351-what-is-the-progression-ladder-in-the-gauntlet-mini", accessed: "2026-10-23" }
  - { field: trading_day_reset, url: "https://help.earn2trade.com/en/articles/3395926-how-is-my-daily-loss-calculated", accessed: "2026-10-23" }
  - { field: copy_trading_allowed, url: "https://help.earn2trade.com/en/articles/12034590-am-i-allowed-to-copy-trades-across-multiple-accounts", accessed: "2026-10-23" }
  - { field: automated_trading_allowed, url: "https://help.earn2trade.com/en/articles/9286647-prohibited-conduct-in-earn2trade-evaluations-and-in-the-livesim-live-trading-environment", accessed: "2026-10-23" }
  - { field: commission_per_contract, url: "https://help.earn2trade.com/en/articles/2425493-what-is-the-commission-on-earn2trade-evaluations", accessed: "2026-10-23" }
  - { field: platforms, url: "https://help.earn2trade.com/en/articles/2090521-what-platforms-can-i-use-for-the-gauntlet-mini-trader-career-path", accessed: "2026-10-23" }
```

#### Gauntlet Mini — evaluation — 200K

```yaml
firm: earn2trade
plan: "The Gauntlet Mini (GAU200)"
phase: evaluation
size_label: "200K"
size_usd: "200000.00"
instruments: [MNQ, NQ]
platforms: ["Rithmic (Finamark, R|Trader Pro and more)", "BlackArrow", "NinjaTrader (via Tradovate)", "Tradovate", "TradingView (via Tradovate)", "dxFeed/DeepCharts"]
drawdown: { amount_usd: "6000.00", type: trailing_eod, stops_trailing_at: "Minimum balance rises with each end-of-day balance high and stops rising once it reaches the starting balance ($200,000)" }
daily_loss_limit: { amount_usd: "4400.00", kind: hard }
profit_target_usd: "11000.00"
consistency_rule: { text: "No single trading day can account for 30% or more of total PnL over the evaluation (the evaluation continues until the day is below 30% of total profit; it is not an instant fail).", threshold_pct: "30" }
max_position: { minis: 16, micros: 160, scaling: "Progression Ladder by profit: $0-$1,500: 5; $1,501-$3,000: 7; $3,001-$4,500: 10; $4,501+: 16. Exceeding it is a hard fail; up to ten micros count as one standard contract." }
min_trading_days: null            # no minimum published
trading_day_reset: { time: "17:00", tz_as_published: "CT" }
commission_per_contract: { mnq_usd: null, nq_usd: null, basis: per_side, includes: "Published as 'Total Per Side' in the firm's per-platform chart; what it includes is not stated. Platform-dependent NQ per side: Rithmic 2.04, BlackArrow 2.04, Tradovate/NinjaTrader 2.88. MNQ is not listed (text only: micro contracts range from 0.82 to 1.20 per contract per side)" }
copy_trading_allowed: no
automated_trading_allowed: conditional
automation_rule_text: "Utilizing software, artificial intelligence, or ultra-fast data entry techniques that could potentially manipulate the trading environment or provide an unfair advantage"
vps_allowed: unknown
personal_device_required: unknown
status: partial
not_confirmed_fields: [commission_per_contract.mnq_usd, commission_per_contract.nq_usd, vps_allowed, personal_device_required, automated_trading_allowed]
notes: "Single-phase evaluation, no time limit, no minimum trading days. Daily loss breach fails the evaluation (hard). Must be flat by 15:50 CT. Max 5 concurrent evaluation accounts. Only GAU50 values were rendered in the page text; GAU100/150/200 come from the site's JS data and agree with the help-center drawdown image and ladder image."
sources:
  - { field: drawdown, url: "https://help.earn2trade.com/en/articles/3224507-what-is-the-maximum-drawdown-on-the-trader-career-path-and-the-gauntlet-mini-challenges", accessed: "2026-10-23" }
  - { field: drawdown_rule, url: "https://help.earn2trade.com/en/articles/5372687-how-does-end-of-day-drawdown-work", accessed: "2026-10-23" }
  - { field: daily_loss_limit, url: "https://help.earn2trade.com/en/articles/3395926-how-is-my-daily-loss-calculated", accessed: "2026-10-23" }
  - { field: profit_target_usd, url: "https://www.earn2trade.com/gauntlet-mini", accessed: "2026-10-23" }
  - { field: profit_target_usd_all_sizes, url: "https://www.earn2trade.com/_next/static/chunks/0d45t7274e6.p.js", accessed: "2026-10-23" }
  - { field: consistency_rule, url: "https://help.earn2trade.com/en/articles/3849975-what-is-the-maintain-consistency-rule", accessed: "2026-10-23" }
  - { field: max_position, url: "https://help.earn2trade.com/en/articles/3292351-what-is-the-progression-ladder-in-the-gauntlet-mini", accessed: "2026-10-23" }
  - { field: trading_day_reset, url: "https://help.earn2trade.com/en/articles/3395926-how-is-my-daily-loss-calculated", accessed: "2026-10-23" }
  - { field: copy_trading_allowed, url: "https://help.earn2trade.com/en/articles/12034590-am-i-allowed-to-copy-trades-across-multiple-accounts", accessed: "2026-10-23" }
  - { field: automated_trading_allowed, url: "https://help.earn2trade.com/en/articles/9286647-prohibited-conduct-in-earn2trade-evaluations-and-in-the-livesim-live-trading-environment", accessed: "2026-10-23" }
  - { field: commission_per_contract, url: "https://help.earn2trade.com/en/articles/2425493-what-is-the-commission-on-earn2trade-evaluations", accessed: "2026-10-23" }
  - { field: platforms, url: "https://help.earn2trade.com/en/articles/2090521-what-platforms-can-i-use-for-the-gauntlet-mini-trader-career-path", accessed: "2026-10-23" }
```

#### Gauntlet Mini — LiveSim (funded) — 50K

```yaml
firm: earn2trade
plan: "The Gauntlet Mini (GAU50)"
phase: funded
size_label: "50K"
size_usd: "50000.00"
instruments: [MNQ, NQ]
platforms: ["Any platform on the published list (NinjaTrader, Finamark, R|Trader Pro, Tradovate, TradingView, BlackArrow, DeepCharts, Quantower and others)"]
drawdown: { amount_usd: "2000.00", type: trailing_eod, stops_trailing_at: "Minimum balance rises with each end-of-day balance high and stops rising once it reaches the starting balance ($50,000)" }
daily_loss_limit: { amount_usd: "1100.00", kind: unknown }
profit_target_usd: "none"
consistency_rule: { text: "none (published: No Consistency Requirement)", threshold_pct: null }
max_position: { minis: 6, micros: 60, scaling: "Progression Ladder by profit: $0-$1,500: 2; $1,501-$2,000: 4; $2,001+: 6. Exceeding it is a hard fail; up to ten micros count as one standard contract." }
min_trading_days: null            # no minimum published
trading_day_reset: { time: "17:00", tz_as_published: "CT" }
commission_per_contract: { mnq_usd: null, nq_usd: null, basis: per_side, includes: "Published as 'Total Per Side' in the firm's per-platform chart; what it includes is not stated. Platform-dependent NQ per side: Rithmic 2.04, BlackArrow 2.04, Tradovate/NinjaTrader 2.88. MNQ is not listed (text only: micro contracts range from 0.82 to 1.20 per contract per side)" }
copy_trading_allowed: no
automated_trading_allowed: conditional
automation_rule_text: "Utilizing software, artificial intelligence, or ultra-fast data entry techniques that could potentially manipulate the trading environment or provide an unfair advantage"
vps_allowed: unknown
personal_device_required: unknown
status: partial
not_confirmed_fields: [commission_per_contract.mnq_usd, commission_per_contract.nq_usd, vps_allowed, personal_device_required, automated_trading_allowed, daily_loss_limit.kind]
notes: "LiveSim = simulated funded account with live data. No consistency rule, no minimum trading days. Daily loss limit behavior on funded accounts is not stated. Max 3 funded accounts (3 LiveSim, or 2 LiveSim and 1 Live). Same size as the passed evaluation."
sources:
  - { field: drawdown, url: "https://help.earn2trade.com/en/articles/3224507-what-is-the-maximum-drawdown-on-the-trader-career-path-and-the-gauntlet-mini-challenges", accessed: "2026-10-23" }
  - { field: drawdown_types, url: "https://help.earn2trade.com/en/articles/8117088-drawdown-types", accessed: "2026-10-23" }
  - { field: daily_loss_limit, url: "https://help.earn2trade.com/en/articles/3395926-how-is-my-daily-loss-calculated", accessed: "2026-10-23" }
  - { field: consistency_rule, url: "https://help.earn2trade.com/en/articles/6877364-what-happens-when-i-complete-my-evaluation", accessed: "2026-10-23" }
  - { field: max_position, url: "https://help.earn2trade.com/en/articles/3292351-what-is-the-progression-ladder-in-the-gauntlet-mini", accessed: "2026-10-23" }
  - { field: trading_day_reset, url: "https://help.earn2trade.com/en/articles/3395926-how-is-my-daily-loss-calculated", accessed: "2026-10-23" }
  - { field: copy_trading_allowed, url: "https://help.earn2trade.com/en/articles/12034590-am-i-allowed-to-copy-trades-across-multiple-accounts", accessed: "2026-10-23" }
  - { field: automated_trading_allowed, url: "https://help.earn2trade.com/en/articles/9286647-prohibited-conduct-in-earn2trade-evaluations-and-in-the-livesim-live-trading-environment", accessed: "2026-10-23" }
  - { field: commission_per_contract, url: "https://help.earn2trade.com/en/articles/2425493-what-is-the-commission-on-earn2trade-evaluations", accessed: "2026-10-23" }
  - { field: platforms, url: "https://www.earn2trade.com/gauntlet-mini", accessed: "2026-10-23" }
  - { field: values_js, url: "https://www.earn2trade.com/_next/static/chunks/0d45t7274e6.p.js", accessed: "2026-10-23" }
```

#### Gauntlet Mini — LiveSim (funded) — 100K

```yaml
firm: earn2trade
plan: "The Gauntlet Mini (GAU100)"
phase: funded
size_label: "100K"
size_usd: "100000.00"
instruments: [MNQ, NQ]
platforms: ["Any platform on the published list (NinjaTrader, Finamark, R|Trader Pro, Tradovate, TradingView, BlackArrow, DeepCharts, Quantower and others)"]
drawdown: { amount_usd: "3500.00", type: trailing_eod, stops_trailing_at: "Minimum balance rises with each end-of-day balance high and stops rising once it reaches the starting balance ($100,000)" }
daily_loss_limit: { amount_usd: "2200.00", kind: unknown }
profit_target_usd: "none"
consistency_rule: { text: "none (published: No Consistency Requirement)", threshold_pct: null }
max_position: { minis: 12, micros: 120, scaling: "Progression Ladder by profit: $0-$1,500: 3; $1,501-$3,000: 6; $3,001-$4,500: 9; $4,501+: 12. Exceeding it is a hard fail; up to ten micros count as one standard contract." }
min_trading_days: null            # no minimum published
trading_day_reset: { time: "17:00", tz_as_published: "CT" }
commission_per_contract: { mnq_usd: null, nq_usd: null, basis: per_side, includes: "Published as 'Total Per Side' in the firm's per-platform chart; what it includes is not stated. Platform-dependent NQ per side: Rithmic 2.04, BlackArrow 2.04, Tradovate/NinjaTrader 2.88. MNQ is not listed (text only: micro contracts range from 0.82 to 1.20 per contract per side)" }
copy_trading_allowed: no
automated_trading_allowed: conditional
automation_rule_text: "Utilizing software, artificial intelligence, or ultra-fast data entry techniques that could potentially manipulate the trading environment or provide an unfair advantage"
vps_allowed: unknown
personal_device_required: unknown
status: partial
not_confirmed_fields: [commission_per_contract.mnq_usd, commission_per_contract.nq_usd, vps_allowed, personal_device_required, automated_trading_allowed, daily_loss_limit.kind]
notes: "LiveSim = simulated funded account with live data. No consistency rule, no minimum trading days. Daily loss limit behavior on funded accounts is not stated. Max 3 funded accounts (3 LiveSim, or 2 LiveSim and 1 Live). Same size as the passed evaluation."
sources:
  - { field: drawdown, url: "https://help.earn2trade.com/en/articles/3224507-what-is-the-maximum-drawdown-on-the-trader-career-path-and-the-gauntlet-mini-challenges", accessed: "2026-10-23" }
  - { field: drawdown_types, url: "https://help.earn2trade.com/en/articles/8117088-drawdown-types", accessed: "2026-10-23" }
  - { field: daily_loss_limit, url: "https://help.earn2trade.com/en/articles/3395926-how-is-my-daily-loss-calculated", accessed: "2026-10-23" }
  - { field: consistency_rule, url: "https://help.earn2trade.com/en/articles/6877364-what-happens-when-i-complete-my-evaluation", accessed: "2026-10-23" }
  - { field: max_position, url: "https://help.earn2trade.com/en/articles/3292351-what-is-the-progression-ladder-in-the-gauntlet-mini", accessed: "2026-10-23" }
  - { field: trading_day_reset, url: "https://help.earn2trade.com/en/articles/3395926-how-is-my-daily-loss-calculated", accessed: "2026-10-23" }
  - { field: copy_trading_allowed, url: "https://help.earn2trade.com/en/articles/12034590-am-i-allowed-to-copy-trades-across-multiple-accounts", accessed: "2026-10-23" }
  - { field: automated_trading_allowed, url: "https://help.earn2trade.com/en/articles/9286647-prohibited-conduct-in-earn2trade-evaluations-and-in-the-livesim-live-trading-environment", accessed: "2026-10-23" }
  - { field: commission_per_contract, url: "https://help.earn2trade.com/en/articles/2425493-what-is-the-commission-on-earn2trade-evaluations", accessed: "2026-10-23" }
  - { field: platforms, url: "https://www.earn2trade.com/gauntlet-mini", accessed: "2026-10-23" }
  - { field: values_js, url: "https://www.earn2trade.com/_next/static/chunks/0d45t7274e6.p.js", accessed: "2026-10-23" }
```

#### Gauntlet Mini — LiveSim (funded) — 150K

```yaml
firm: earn2trade
plan: "The Gauntlet Mini (GAU150)"
phase: funded
size_label: "150K"
size_usd: "150000.00"
instruments: [MNQ, NQ]
platforms: ["Any platform on the published list (NinjaTrader, Finamark, R|Trader Pro, Tradovate, TradingView, BlackArrow, DeepCharts, Quantower and others)"]
drawdown: { amount_usd: "4500.00", type: trailing_eod, stops_trailing_at: "Minimum balance rises with each end-of-day balance high and stops rising once it reaches the starting balance ($150,000)" }
daily_loss_limit: { amount_usd: "3300.00", kind: unknown }
profit_target_usd: "none"
consistency_rule: { text: "none (published: No Consistency Requirement)", threshold_pct: null }
max_position: { minis: 15, micros: 150, scaling: "Progression Ladder by profit: $0-$1,500: 4; $1,501-$3,000: 6; $3,001-$4,500: 9; $4,501+: 15. Exceeding it is a hard fail; up to ten micros count as one standard contract." }
min_trading_days: null            # no minimum published
trading_day_reset: { time: "17:00", tz_as_published: "CT" }
commission_per_contract: { mnq_usd: null, nq_usd: null, basis: per_side, includes: "Published as 'Total Per Side' in the firm's per-platform chart; what it includes is not stated. Platform-dependent NQ per side: Rithmic 2.04, BlackArrow 2.04, Tradovate/NinjaTrader 2.88. MNQ is not listed (text only: micro contracts range from 0.82 to 1.20 per contract per side)" }
copy_trading_allowed: no
automated_trading_allowed: conditional
automation_rule_text: "Utilizing software, artificial intelligence, or ultra-fast data entry techniques that could potentially manipulate the trading environment or provide an unfair advantage"
vps_allowed: unknown
personal_device_required: unknown
status: partial
not_confirmed_fields: [commission_per_contract.mnq_usd, commission_per_contract.nq_usd, vps_allowed, personal_device_required, automated_trading_allowed, daily_loss_limit.kind]
notes: "LiveSim = simulated funded account with live data. No consistency rule, no minimum trading days. Daily loss limit behavior on funded accounts is not stated. Max 3 funded accounts (3 LiveSim, or 2 LiveSim and 1 Live). Same size as the passed evaluation."
sources:
  - { field: drawdown, url: "https://help.earn2trade.com/en/articles/3224507-what-is-the-maximum-drawdown-on-the-trader-career-path-and-the-gauntlet-mini-challenges", accessed: "2026-10-23" }
  - { field: drawdown_types, url: "https://help.earn2trade.com/en/articles/8117088-drawdown-types", accessed: "2026-10-23" }
  - { field: daily_loss_limit, url: "https://help.earn2trade.com/en/articles/3395926-how-is-my-daily-loss-calculated", accessed: "2026-10-23" }
  - { field: consistency_rule, url: "https://help.earn2trade.com/en/articles/6877364-what-happens-when-i-complete-my-evaluation", accessed: "2026-10-23" }
  - { field: max_position, url: "https://help.earn2trade.com/en/articles/3292351-what-is-the-progression-ladder-in-the-gauntlet-mini", accessed: "2026-10-23" }
  - { field: trading_day_reset, url: "https://help.earn2trade.com/en/articles/3395926-how-is-my-daily-loss-calculated", accessed: "2026-10-23" }
  - { field: copy_trading_allowed, url: "https://help.earn2trade.com/en/articles/12034590-am-i-allowed-to-copy-trades-across-multiple-accounts", accessed: "2026-10-23" }
  - { field: automated_trading_allowed, url: "https://help.earn2trade.com/en/articles/9286647-prohibited-conduct-in-earn2trade-evaluations-and-in-the-livesim-live-trading-environment", accessed: "2026-10-23" }
  - { field: commission_per_contract, url: "https://help.earn2trade.com/en/articles/2425493-what-is-the-commission-on-earn2trade-evaluations", accessed: "2026-10-23" }
  - { field: platforms, url: "https://www.earn2trade.com/gauntlet-mini", accessed: "2026-10-23" }
  - { field: values_js, url: "https://www.earn2trade.com/_next/static/chunks/0d45t7274e6.p.js", accessed: "2026-10-23" }
```

#### Gauntlet Mini — LiveSim (funded) — 200K

```yaml
firm: earn2trade
plan: "The Gauntlet Mini (GAU200)"
phase: funded
size_label: "200K"
size_usd: "200000.00"
instruments: [MNQ, NQ]
platforms: ["Any platform on the published list (NinjaTrader, Finamark, R|Trader Pro, Tradovate, TradingView, BlackArrow, DeepCharts, Quantower and others)"]
drawdown: { amount_usd: "6000.00", type: trailing_eod, stops_trailing_at: "Minimum balance rises with each end-of-day balance high and stops rising once it reaches the starting balance ($200,000)" }
daily_loss_limit: { amount_usd: "4400.00", kind: unknown }
profit_target_usd: "none"
consistency_rule: { text: "none (published: No Consistency Requirement)", threshold_pct: null }
max_position: { minis: 16, micros: 160, scaling: "Progression Ladder by profit: $0-$1,500: 5; $1,501-$3,000: 7; $3,001-$4,500: 10; $4,501+: 16. Exceeding it is a hard fail; up to ten micros count as one standard contract." }
min_trading_days: null            # no minimum published
trading_day_reset: { time: "17:00", tz_as_published: "CT" }
commission_per_contract: { mnq_usd: null, nq_usd: null, basis: per_side, includes: "Published as 'Total Per Side' in the firm's per-platform chart; what it includes is not stated. Platform-dependent NQ per side: Rithmic 2.04, BlackArrow 2.04, Tradovate/NinjaTrader 2.88. MNQ is not listed (text only: micro contracts range from 0.82 to 1.20 per contract per side)" }
copy_trading_allowed: no
automated_trading_allowed: conditional
automation_rule_text: "Utilizing software, artificial intelligence, or ultra-fast data entry techniques that could potentially manipulate the trading environment or provide an unfair advantage"
vps_allowed: unknown
personal_device_required: unknown
status: partial
not_confirmed_fields: [commission_per_contract.mnq_usd, commission_per_contract.nq_usd, vps_allowed, personal_device_required, automated_trading_allowed, daily_loss_limit.kind]
notes: "LiveSim = simulated funded account with live data. No consistency rule, no minimum trading days. Daily loss limit behavior on funded accounts is not stated. Max 3 funded accounts (3 LiveSim, or 2 LiveSim and 1 Live). Same size as the passed evaluation."
sources:
  - { field: drawdown, url: "https://help.earn2trade.com/en/articles/3224507-what-is-the-maximum-drawdown-on-the-trader-career-path-and-the-gauntlet-mini-challenges", accessed: "2026-10-23" }
  - { field: drawdown_types, url: "https://help.earn2trade.com/en/articles/8117088-drawdown-types", accessed: "2026-10-23" }
  - { field: daily_loss_limit, url: "https://help.earn2trade.com/en/articles/3395926-how-is-my-daily-loss-calculated", accessed: "2026-10-23" }
  - { field: consistency_rule, url: "https://help.earn2trade.com/en/articles/6877364-what-happens-when-i-complete-my-evaluation", accessed: "2026-10-23" }
  - { field: max_position, url: "https://help.earn2trade.com/en/articles/3292351-what-is-the-progression-ladder-in-the-gauntlet-mini", accessed: "2026-10-23" }
  - { field: trading_day_reset, url: "https://help.earn2trade.com/en/articles/3395926-how-is-my-daily-loss-calculated", accessed: "2026-10-23" }
  - { field: copy_trading_allowed, url: "https://help.earn2trade.com/en/articles/12034590-am-i-allowed-to-copy-trades-across-multiple-accounts", accessed: "2026-10-23" }
  - { field: automated_trading_allowed, url: "https://help.earn2trade.com/en/articles/9286647-prohibited-conduct-in-earn2trade-evaluations-and-in-the-livesim-live-trading-environment", accessed: "2026-10-23" }
  - { field: commission_per_contract, url: "https://help.earn2trade.com/en/articles/2425493-what-is-the-commission-on-earn2trade-evaluations", accessed: "2026-10-23" }
  - { field: platforms, url: "https://www.earn2trade.com/gauntlet-mini", accessed: "2026-10-23" }
  - { field: values_js, url: "https://www.earn2trade.com/_next/static/chunks/0d45t7274e6.p.js", accessed: "2026-10-23" }
```

#### Gauntlet Mini — Live (funded) — 50K

```yaml
firm: earn2trade
plan: "The Gauntlet Mini (GAU50)"
phase: live
size_label: "50K"
size_usd: "50000.00"
instruments: [MNQ, NQ]
platforms: ["NinjaTrader desktop (live credentials from the prop firm)", "Rithmic (R|Trader Pro and compatible platforms)"]
drawdown: { amount_usd: "2000.00", type: trailing_intraday, stops_trailing_at: "Trails the highest balance intraday and locks at the starting balance ($50,000)" }
daily_loss_limit: { amount_usd: "1100.00", kind: unknown }
profit_target_usd: "none"
consistency_rule: { text: "none (published: No Consistency Requirement)", threshold_pct: null }
max_position: { minis: 6, micros: 60, scaling: "Progression Ladder by profit: $0-$1,500: 2; $1,501-$2,000: 4; $2,001+: 6. Exceeding it is a hard fail; up to ten micros count as one standard contract." }
min_trading_days: null            # no minimum published
trading_day_reset: { time: "17:00", tz_as_published: "CT" }
commission_per_contract: { mnq_usd: null, nq_usd: null, basis: per_side, includes: "Published as 'Total Per Side' in the firm's per-platform chart; what it includes is not stated. Platform-dependent NQ per side: Rithmic 2.04, BlackArrow 2.04, Tradovate/NinjaTrader 2.88. MNQ is not listed (text only: micro contracts range from 0.82 to 1.20 per contract per side)" }
copy_trading_allowed: no
automated_trading_allowed: conditional
automation_rule_text: "Utilizing software, artificial intelligence, or ultra-fast data entry techniques that could potentially manipulate the trading environment or provide an unfair advantage"
vps_allowed: unknown
personal_device_required: unknown
status: partial
not_confirmed_fields: [commission_per_contract.mnq_usd, commission_per_contract.nq_usd, vps_allowed, personal_device_required, automated_trading_allowed, daily_loss_limit.kind]
notes: "Live brokerage account. Not a fixed drawdown (published: the 200K Gauntlet Mini account does not use a fixed drawdown). Max 1 Live account. Daily loss limit behavior on funded accounts is not stated. Live data fees $140 (Rithmic) / $156 (NinjaTrader) per exchange per month."
sources:
  - { field: drawdown, url: "https://help.earn2trade.com/en/articles/3224507-what-is-the-maximum-drawdown-on-the-trader-career-path-and-the-gauntlet-mini-challenges", accessed: "2026-10-23" }
  - { field: drawdown_types, url: "https://help.earn2trade.com/en/articles/8117088-drawdown-types", accessed: "2026-10-23" }
  - { field: daily_loss_limit, url: "https://help.earn2trade.com/en/articles/3395926-how-is-my-daily-loss-calculated", accessed: "2026-10-23" }
  - { field: consistency_rule, url: "https://help.earn2trade.com/en/articles/6877364-what-happens-when-i-complete-my-evaluation", accessed: "2026-10-23" }
  - { field: max_position, url: "https://help.earn2trade.com/en/articles/3292351-what-is-the-progression-ladder-in-the-gauntlet-mini", accessed: "2026-10-23" }
  - { field: trading_day_reset, url: "https://help.earn2trade.com/en/articles/3395926-how-is-my-daily-loss-calculated", accessed: "2026-10-23" }
  - { field: copy_trading_allowed, url: "https://help.earn2trade.com/en/articles/12034590-am-i-allowed-to-copy-trades-across-multiple-accounts", accessed: "2026-10-23" }
  - { field: automated_trading_allowed, url: "https://help.earn2trade.com/en/articles/9286647-prohibited-conduct-in-earn2trade-evaluations-and-in-the-livesim-live-trading-environment", accessed: "2026-10-23" }
  - { field: commission_per_contract, url: "https://help.earn2trade.com/en/articles/2425493-what-is-the-commission-on-earn2trade-evaluations", accessed: "2026-10-23" }
  - { field: platforms, url: "https://www.earn2trade.com/gauntlet-mini", accessed: "2026-10-23" }
  - { field: values_js, url: "https://www.earn2trade.com/_next/static/chunks/0d45t7274e6.p.js", accessed: "2026-10-23" }
  - { field: fixed_drawdown_exclusion, url: "https://help.earn2trade.com/en/articles/8129235-what-is-a-fixed-drawdown", accessed: "2026-10-23" }
```

#### Gauntlet Mini — Live (funded) — 100K

```yaml
firm: earn2trade
plan: "The Gauntlet Mini (GAU100)"
phase: live
size_label: "100K"
size_usd: "100000.00"
instruments: [MNQ, NQ]
platforms: ["NinjaTrader desktop (live credentials from the prop firm)", "Rithmic (R|Trader Pro and compatible platforms)"]
drawdown: { amount_usd: "3500.00", type: trailing_intraday, stops_trailing_at: "Trails the highest balance intraday and locks at the starting balance ($100,000)" }
daily_loss_limit: { amount_usd: "2200.00", kind: unknown }
profit_target_usd: "none"
consistency_rule: { text: "none (published: No Consistency Requirement)", threshold_pct: null }
max_position: { minis: 12, micros: 120, scaling: "Progression Ladder by profit: $0-$1,500: 3; $1,501-$3,000: 6; $3,001-$4,500: 9; $4,501+: 12. Exceeding it is a hard fail; up to ten micros count as one standard contract." }
min_trading_days: null            # no minimum published
trading_day_reset: { time: "17:00", tz_as_published: "CT" }
commission_per_contract: { mnq_usd: null, nq_usd: null, basis: per_side, includes: "Published as 'Total Per Side' in the firm's per-platform chart; what it includes is not stated. Platform-dependent NQ per side: Rithmic 2.04, BlackArrow 2.04, Tradovate/NinjaTrader 2.88. MNQ is not listed (text only: micro contracts range from 0.82 to 1.20 per contract per side)" }
copy_trading_allowed: no
automated_trading_allowed: conditional
automation_rule_text: "Utilizing software, artificial intelligence, or ultra-fast data entry techniques that could potentially manipulate the trading environment or provide an unfair advantage"
vps_allowed: unknown
personal_device_required: unknown
status: partial
not_confirmed_fields: [commission_per_contract.mnq_usd, commission_per_contract.nq_usd, vps_allowed, personal_device_required, automated_trading_allowed, daily_loss_limit.kind]
notes: "Live brokerage account. Not a fixed drawdown (published: the 200K Gauntlet Mini account does not use a fixed drawdown). Max 1 Live account. Daily loss limit behavior on funded accounts is not stated. Live data fees $140 (Rithmic) / $156 (NinjaTrader) per exchange per month."
sources:
  - { field: drawdown, url: "https://help.earn2trade.com/en/articles/3224507-what-is-the-maximum-drawdown-on-the-trader-career-path-and-the-gauntlet-mini-challenges", accessed: "2026-10-23" }
  - { field: drawdown_types, url: "https://help.earn2trade.com/en/articles/8117088-drawdown-types", accessed: "2026-10-23" }
  - { field: daily_loss_limit, url: "https://help.earn2trade.com/en/articles/3395926-how-is-my-daily-loss-calculated", accessed: "2026-10-23" }
  - { field: consistency_rule, url: "https://help.earn2trade.com/en/articles/6877364-what-happens-when-i-complete-my-evaluation", accessed: "2026-10-23" }
  - { field: max_position, url: "https://help.earn2trade.com/en/articles/3292351-what-is-the-progression-ladder-in-the-gauntlet-mini", accessed: "2026-10-23" }
  - { field: trading_day_reset, url: "https://help.earn2trade.com/en/articles/3395926-how-is-my-daily-loss-calculated", accessed: "2026-10-23" }
  - { field: copy_trading_allowed, url: "https://help.earn2trade.com/en/articles/12034590-am-i-allowed-to-copy-trades-across-multiple-accounts", accessed: "2026-10-23" }
  - { field: automated_trading_allowed, url: "https://help.earn2trade.com/en/articles/9286647-prohibited-conduct-in-earn2trade-evaluations-and-in-the-livesim-live-trading-environment", accessed: "2026-10-23" }
  - { field: commission_per_contract, url: "https://help.earn2trade.com/en/articles/2425493-what-is-the-commission-on-earn2trade-evaluations", accessed: "2026-10-23" }
  - { field: platforms, url: "https://www.earn2trade.com/gauntlet-mini", accessed: "2026-10-23" }
  - { field: values_js, url: "https://www.earn2trade.com/_next/static/chunks/0d45t7274e6.p.js", accessed: "2026-10-23" }
  - { field: fixed_drawdown_exclusion, url: "https://help.earn2trade.com/en/articles/8129235-what-is-a-fixed-drawdown", accessed: "2026-10-23" }
```

#### Gauntlet Mini — Live (funded) — 150K

```yaml
firm: earn2trade
plan: "The Gauntlet Mini (GAU150)"
phase: live
size_label: "150K"
size_usd: "150000.00"
instruments: [MNQ, NQ]
platforms: ["NinjaTrader desktop (live credentials from the prop firm)", "Rithmic (R|Trader Pro and compatible platforms)"]
drawdown: { amount_usd: "4500.00", type: trailing_intraday, stops_trailing_at: "Trails the highest balance intraday and locks at the starting balance ($150,000)" }
daily_loss_limit: { amount_usd: "3300.00", kind: unknown }
profit_target_usd: "none"
consistency_rule: { text: "none (published: No Consistency Requirement)", threshold_pct: null }
max_position: { minis: 15, micros: 150, scaling: "Progression Ladder by profit: $0-$1,500: 4; $1,501-$3,000: 6; $3,001-$4,500: 9; $4,501+: 15. Exceeding it is a hard fail; up to ten micros count as one standard contract." }
min_trading_days: null            # no minimum published
trading_day_reset: { time: "17:00", tz_as_published: "CT" }
commission_per_contract: { mnq_usd: null, nq_usd: null, basis: per_side, includes: "Published as 'Total Per Side' in the firm's per-platform chart; what it includes is not stated. Platform-dependent NQ per side: Rithmic 2.04, BlackArrow 2.04, Tradovate/NinjaTrader 2.88. MNQ is not listed (text only: micro contracts range from 0.82 to 1.20 per contract per side)" }
copy_trading_allowed: no
automated_trading_allowed: conditional
automation_rule_text: "Utilizing software, artificial intelligence, or ultra-fast data entry techniques that could potentially manipulate the trading environment or provide an unfair advantage"
vps_allowed: unknown
personal_device_required: unknown
status: partial
not_confirmed_fields: [commission_per_contract.mnq_usd, commission_per_contract.nq_usd, vps_allowed, personal_device_required, automated_trading_allowed, daily_loss_limit.kind]
notes: "Live brokerage account. Not a fixed drawdown (published: the 200K Gauntlet Mini account does not use a fixed drawdown). Max 1 Live account. Daily loss limit behavior on funded accounts is not stated. Live data fees $140 (Rithmic) / $156 (NinjaTrader) per exchange per month."
sources:
  - { field: drawdown, url: "https://help.earn2trade.com/en/articles/3224507-what-is-the-maximum-drawdown-on-the-trader-career-path-and-the-gauntlet-mini-challenges", accessed: "2026-10-23" }
  - { field: drawdown_types, url: "https://help.earn2trade.com/en/articles/8117088-drawdown-types", accessed: "2026-10-23" }
  - { field: daily_loss_limit, url: "https://help.earn2trade.com/en/articles/3395926-how-is-my-daily-loss-calculated", accessed: "2026-10-23" }
  - { field: consistency_rule, url: "https://help.earn2trade.com/en/articles/6877364-what-happens-when-i-complete-my-evaluation", accessed: "2026-10-23" }
  - { field: max_position, url: "https://help.earn2trade.com/en/articles/3292351-what-is-the-progression-ladder-in-the-gauntlet-mini", accessed: "2026-10-23" }
  - { field: trading_day_reset, url: "https://help.earn2trade.com/en/articles/3395926-how-is-my-daily-loss-calculated", accessed: "2026-10-23" }
  - { field: copy_trading_allowed, url: "https://help.earn2trade.com/en/articles/12034590-am-i-allowed-to-copy-trades-across-multiple-accounts", accessed: "2026-10-23" }
  - { field: automated_trading_allowed, url: "https://help.earn2trade.com/en/articles/9286647-prohibited-conduct-in-earn2trade-evaluations-and-in-the-livesim-live-trading-environment", accessed: "2026-10-23" }
  - { field: commission_per_contract, url: "https://help.earn2trade.com/en/articles/2425493-what-is-the-commission-on-earn2trade-evaluations", accessed: "2026-10-23" }
  - { field: platforms, url: "https://www.earn2trade.com/gauntlet-mini", accessed: "2026-10-23" }
  - { field: values_js, url: "https://www.earn2trade.com/_next/static/chunks/0d45t7274e6.p.js", accessed: "2026-10-23" }
  - { field: fixed_drawdown_exclusion, url: "https://help.earn2trade.com/en/articles/8129235-what-is-a-fixed-drawdown", accessed: "2026-10-23" }
```

#### Gauntlet Mini — Live (funded) — 200K

```yaml
firm: earn2trade
plan: "The Gauntlet Mini (GAU200)"
phase: live
size_label: "200K"
size_usd: "200000.00"
instruments: [MNQ, NQ]
platforms: ["NinjaTrader desktop (live credentials from the prop firm)", "Rithmic (R|Trader Pro and compatible platforms)"]
drawdown: { amount_usd: "6000.00", type: trailing_intraday, stops_trailing_at: "Trails the highest balance intraday and locks at the starting balance ($200,000)" }
daily_loss_limit: { amount_usd: "4400.00", kind: unknown }
profit_target_usd: "none"
consistency_rule: { text: "none (published: No Consistency Requirement)", threshold_pct: null }
max_position: { minis: 16, micros: 160, scaling: "Progression Ladder by profit: $0-$1,500: 5; $1,501-$3,000: 7; $3,001-$4,500: 10; $4,501+: 16. Exceeding it is a hard fail; up to ten micros count as one standard contract." }
min_trading_days: null            # no minimum published
trading_day_reset: { time: "17:00", tz_as_published: "CT" }
commission_per_contract: { mnq_usd: null, nq_usd: null, basis: per_side, includes: "Published as 'Total Per Side' in the firm's per-platform chart; what it includes is not stated. Platform-dependent NQ per side: Rithmic 2.04, BlackArrow 2.04, Tradovate/NinjaTrader 2.88. MNQ is not listed (text only: micro contracts range from 0.82 to 1.20 per contract per side)" }
copy_trading_allowed: no
automated_trading_allowed: conditional
automation_rule_text: "Utilizing software, artificial intelligence, or ultra-fast data entry techniques that could potentially manipulate the trading environment or provide an unfair advantage"
vps_allowed: unknown
personal_device_required: unknown
status: partial
not_confirmed_fields: [commission_per_contract.mnq_usd, commission_per_contract.nq_usd, vps_allowed, personal_device_required, automated_trading_allowed, daily_loss_limit.kind]
notes: "Live brokerage account. Not a fixed drawdown (published: the 200K Gauntlet Mini account does not use a fixed drawdown). Max 1 Live account. Daily loss limit behavior on funded accounts is not stated. Live data fees $140 (Rithmic) / $156 (NinjaTrader) per exchange per month."
sources:
  - { field: drawdown, url: "https://help.earn2trade.com/en/articles/3224507-what-is-the-maximum-drawdown-on-the-trader-career-path-and-the-gauntlet-mini-challenges", accessed: "2026-10-23" }
  - { field: drawdown_types, url: "https://help.earn2trade.com/en/articles/8117088-drawdown-types", accessed: "2026-10-23" }
  - { field: daily_loss_limit, url: "https://help.earn2trade.com/en/articles/3395926-how-is-my-daily-loss-calculated", accessed: "2026-10-23" }
  - { field: consistency_rule, url: "https://help.earn2trade.com/en/articles/6877364-what-happens-when-i-complete-my-evaluation", accessed: "2026-10-23" }
  - { field: max_position, url: "https://help.earn2trade.com/en/articles/3292351-what-is-the-progression-ladder-in-the-gauntlet-mini", accessed: "2026-10-23" }
  - { field: trading_day_reset, url: "https://help.earn2trade.com/en/articles/3395926-how-is-my-daily-loss-calculated", accessed: "2026-10-23" }
  - { field: copy_trading_allowed, url: "https://help.earn2trade.com/en/articles/12034590-am-i-allowed-to-copy-trades-across-multiple-accounts", accessed: "2026-10-23" }
  - { field: automated_trading_allowed, url: "https://help.earn2trade.com/en/articles/9286647-prohibited-conduct-in-earn2trade-evaluations-and-in-the-livesim-live-trading-environment", accessed: "2026-10-23" }
  - { field: commission_per_contract, url: "https://help.earn2trade.com/en/articles/2425493-what-is-the-commission-on-earn2trade-evaluations", accessed: "2026-10-23" }
  - { field: platforms, url: "https://www.earn2trade.com/gauntlet-mini", accessed: "2026-10-23" }
  - { field: values_js, url: "https://www.earn2trade.com/_next/static/chunks/0d45t7274e6.p.js", accessed: "2026-10-23" }
  - { field: fixed_drawdown_exclusion, url: "https://help.earn2trade.com/en/articles/8129235-what-is-a-fixed-drawdown", accessed: "2026-10-23" }
```


## 10. Unverified / gaps / conflicts

| # | Field | Status | What is missing / found | Where searched |
|---|---|---|---|---|
| 1 | MNQ commission per contract | not published | The charts list ES, NQ, MYM, M2K but not MNQ; text gives only "micro contracts range from $0.82 to $1.20 per contract per side" | S16, S1, S2, S17, S40 |
| 2 | NQ commission | platform-dependent | Rithmic 2.04, BlackArrow 2.04, Tradovate 2.88 per side ("Total Per Side"). What the total includes is not stated. Not recorded as one value in YAML | S16 (images) |
| 3 | Funded (LiveSim / Live) commission schedule | not published | Only "FCM commissions, NFA fees, and exchange fees per trade", no amounts | S17, S40 |
| 4 | Daily loss limit behavior on funded accounts (hard vs soft pause) | unknown | Evaluation breach fails the evaluation (S9, S11). Funded: "must not be breached" (S26); the page legend has a "warning" state and a "failed" state with no mapping | S1, S2, S9, S26 |
| 5 | Explicit automated-trading rule | unknown | Only the "software, artificial intelligence, or ultra-fast data entry techniques" clause (S20). Classified `conditional` from that clause alone | full crawl, S20, S21 |
| 6 | Webhooks, TradingView alerts, semi-automated trading | not mentioned | — | full crawl |
| 7 | VPS / VPN / remote server / personal device | unknown | Not mentioned anywhere (searched all articles, terms, disclaimer, support policy, pledge; re-checked R4 2026-10-23). Related only: credentials not shareable, no trading on behalf of others (§7) | full crawl |
| 8 | Copying with other firms' accounts | unknown | Not mentioned; copying across own accounts and with others is forbidden | S18, S19, S20, S21 |
| 9 | Consequence text for trade copiers | note | Terms: "you will receive only one account". Help: "you may be denied your accounts or withdrawals". Same stance, different consequence | S18, S19, S21 |
| 10 | Values read from images | note | Drawdown tables (S4), Progression Ladder tables (S12, S13), commission charts (S16), account comparison (S28) are images. Values were read visually and agree with the product page text and JS data | S4, S12, S13, S16, S28 |
| 11 | Values from the site's JS data, not rendered text | partial | The product pages render only TCP25 and GAU50 in static text (tab switching did not work in the shared browser portal). TCP50, TCP100, GAU100, GAU150, GAU200, TCP funded steps 100K/150K/200K/400K come from the page's own JS data array (S3), cross-checked with S4, S12, S13, S8, S25 | S1, S2, S3 |
| 12 | Fixed-drawdown amount for TCP 200K (via TCP25) and 400K | partial | Published as a minimum balance ($194,000 and $388,000), not as an amount. `drawdown.amount_usd` left `null` | S8, S1, S3 |
| 13 | Evaluation min trading days | no explicit rule | "no minimum number of trading days"; at least four profitable days follow from the 30% rule | S11 |
| 14 | Inactivity rule | not found | — | full crawl |
| 15 | Legacy NinjaTrader on Rithmic | discontinued | "Legacy NinjaTrader accounts using a Rithmic connection will continue to be honored up until June 30th, 2026" (date passed) | S23 |
| 16 | Platform data feed wording | note | S37 says E2T data feed is "currently Rithmic"; newer articles (S23, S34, S35) describe Tradovate (NinjaTrader option) and BlackArrow data feeds. Not treated as a conflict (different purchase options) | S23, S34, S35, S36, S37 |
| 17 | Terms date | note | The terms page has no "last updated" date | S21 |
| 18 | "Unrealistic number of contracts in a single day" | not quantified | No number published | S20 |
| 19 | Partner-firm (Helios / Appius / Kronos) and brokerage rules | not published on E2T pages | Funded accounts are issued by partner firms; their own rules are not covered by the E2T pages and were not researched (official E2T pages only) | S26, S32 |
| 20 | Account-size 300K | not offered | Largest standard evaluation $200,000; $400,000 only as a Live growth step | S1, S2 |

No conflicts between two official pages were found for the required fields.

## 11. Changelog

- 2026-10-23 — initial research (Lupa)
- 2026-10-23 — rev bump R4: VPS and personal device re-crawled (202 help URLs, terms); still not mentioned, both stay unknown (gap 7); related account-sharing ban recorded in §7; no YAML change (Lupa)
