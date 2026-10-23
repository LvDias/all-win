# Take Profit Trader — Prop Firm Rules Research

| | |
|---|---|
| **Firm** | Take Profit Trader (legal name in the terms: "TakeProfitTrader LLC") |
| **Official domains** | `takeprofittrader.com` (main site. The homepage answers plain curl; subpages such as `/terms` return Cloudflare 403, so they were read through the browser portal "Lupa TPT") · `takeprofittraderhelp.zendesk.com` (official help center / "Knowledge Base", linked from the homepage menu `https://takeprofittraderhelp.zendesk.com/hc/en-us`. All 88 en-us articles were read through the public Zendesk API `/api/v2/help_center/en-us/articles.json`) |
| **Research date** | 2026-10-23 (every URL below was accessed 2026-10-23) |
| **Researcher** | Lupa (Prop Firm Rules Researcher) |
| **File status** | `partial`. Test and PRO are verified for every required field except the open items in §9. PRO+ has no published max position or platform list. PRO+ Development has no published stop-trailing point or DLL pause end. |
| **Instruments covered** | MNQ ("Micro E-mini Nasdaq-100 Index Futures", CME) and NQ ("E-mini Nasdaq-100 Futures", CME). Both are on the official list at https://takeprofittraderhelp.zendesk.com/hc/en-us/articles/15172629238301-Approved-Instruments-Permitted-Products-List |

Evidence (raw text of every cited page, URL on line 1): `/tmp/claude-1003/-srv-www-allwin--maestri-roles-6bf77a80-a1ed-43dc-bc00-08a480f178a5/33652d13-4dae-46b6-ae06-44a72b375517/scratchpad/r2/take-profit-trader/`. Help-center articles are saved as `<article-id>-<slug>.txt`. Homepage captures are `home.txt`, `home-size-cards.txt` (each size tab clicked) and `home-journey-tooltips.txt` (comparison-table tooltips). The terms are in `terms.txt`, and the raw API JSON and HTML are under `raw/`. This is temporary scratch storage, so the evidence must be captured again at re-verification.

**Programs found.** The homepage "Your Take Profit Funded Journey" table has three columns: **Test** (SIM), **PRO** (SIM, funded) and **PRO +** (LIVE). The help center has one category each for "Test Rules", "PRO Account" and "PRO+ Account". Inside the PRO+ category there is also **PRO+ Development**, a live variant with smaller limits. Traders cannot buy it. TPT assigns it after a risk review: "Placement in PRO+ Development is based on a review of your overall trading performance, strategy, and risk behavior across both PRO and live accounts." PRO+ itself is invitation-only: "the decision to upgrade a trader to a PRO+ account is made solely at the discretion of Take Profit Trader." Sizes are 25K, 50K, 75K, 100K and 150K for all programs. No size outside 25K–300K was found. TPT trades futures only. Terms: "our program and platforms do not support or facilitate trading in stocks, options, forex, cryptocurrencies, or CFDs."

---

## 2. Platforms

Source: https://takeprofittraderhelp.zendesk.com/hc/en-us/articles/15173017163549-Choosing-Your-Platform (table "Platform / Free During Test / Web/Mobile Based / Data Feed").

| Data feed | Platforms (as published) |
|---|---|
| **CQG** | TradingView, NinjaTrader, Tradovate |
| **Rithmic** | Bookmap, MultiCharts, MotiveWave, Trade Navigator, R\|Trader, VolFix, ATAS, Investor R/T, Quantower, Finamark, Jigsaw Trader |

- The trader picks the data feed at purchase. Quote from https://takeprofittraderhelp.zendesk.com/hc/en-us/articles/15172514256669-How-to-Purchase-An-Account: "Pick your desired data feed, either Rithmic or CQG, from the available options."
- Platforms on the same feed can be switched freely. Quote: "Depending on the data feed you select, you can seamlessly switch between compatible platforms during your subscription period."
- **Test and PRO:** use this list. The help center does not say whether it applies to Test only or also to PRO. The PRO drawdown-tracking article gives instructions for both "Rithmic Accounts" and "CQG Accounts" (https://takeprofittraderhelp.zendesk.com/hc/en-us/articles/15171820366109-How-to-Keep-Track-Of-Your-Drawdown).
- **PRO+ / PRO+ Development:** no platform list is published. Indirect hints only, none of which is a platform rule:
  - The commissions article links to NinjaTrader and Tradovate pricing pages (https://takeprofittraderhelp.zendesk.com/hc/en-us/articles/15172012844957-Commissions-for-PRO).
  - The PRO+ upgrade "will be upgraded to professional market data access".
  - On simultaneous connections: "in PRO+ accounts, this capability may be affected" (https://takeprofittraderhelp.zendesk.com/hc/en-us/articles/15172958310173-Being-Connected-in-Two-Places-At-the-Same-Time).
  - Recorded as a gap (§9).
- Connection differences by feed, quoted from the same "two places" article: "CQG accounts — For test and PRO accounts, you can be connected in two platforms at the same time." "Rithmic accounts — You cannot have two connections open at the same time" (except R|Trader Pro + NinjaTrader in plug-in mode). This is a connectivity rule, not a trading rule. No trading rule differs by platform, so there is one YAML block per size.
- The homepage shows logos for NinjaTrader, TradingView, Tradovate, RTrader, Quantower and MotiveWave under "Your Platform, Your Choice — Connect to any of our 15+ available trading platforms".

## 3. Plans

Sources used in this section:
- Rule 1: https://takeprofittraderhelp.zendesk.com/hc/en-us/articles/15169070804125-Rule-1-Hit-Your-Profit-Target
- Rule 2: https://takeprofittraderhelp.zendesk.com/hc/en-us/articles/15169066911133-Rule-2-Do-Not-Exceed-Maximum-Position-Size
- Rule 3: https://takeprofittraderhelp.zendesk.com/hc/en-us/articles/15170265979165-Rule-3-Do-Not-Hit-End-Of-Day-EOD-Maximum-Trailing-Drawdown
- Rule 5: https://takeprofittraderhelp.zendesk.com/hc/en-us/articles/15170316538013-Rule-5-Be-Consistent
- PRO Account Rules: https://takeprofittraderhelp.zendesk.com/hc/en-us/articles/15171769361053-PRO-Account-Rules
- PRO+ Account Rules: https://takeprofittraderhelp.zendesk.com/hc/en-us/articles/15172006753821-PRO-Account-Rules
- PRO+ Upgrade: https://takeprofittraderhelp.zendesk.com/hc/en-us/articles/15171978600349-PRO-Account-Upgrade-Process-Overview-and-Guidelines
- PRO+ Development: https://takeprofittraderhelp.zendesk.com/hc/en-us/articles/36429526878237-PRO-Development-Accounts
- Homepage size cards and journey table + tooltips: https://takeprofittrader.com/

### 3.1 Test (evaluation, SIM)

| Size | Profit target | Max drawdown (EOD trailing) | Daily loss limit | Max position | Consistency | Min days |
|---|---|---|---|---|---|---|
| 25K ($25,000) | $1,500 | $1,500 | none ("$500 Removed") | 3 Contracts / 30 Micros | best day below 50% of net profit | 3 |
| 50K ($50,000) | $3,000 | $2,000 | none ("$1100 Removed") | 6 Contracts / 60 Micros | same | 3 |
| 75K ($75,000) | $4,500 | $2,500 | none ("$1600 Removed") | 9 Contracts / 90 Micros | same | 3 |
| 100K ($100,000) | $6,000 | $3,000 | none ("$2200 Removed") | 12 Contracts / 120 Micros | same | 3 |
| 150K ($150,000) | $9,000 | $4,500 | none ("$3300 Removed") | 15 Contracts / 150 Micros | same | 3 |

**Drawdown** (Rule 3), quoted:
- "the drawdown for test accounts is calculated only at the end of the trading day, not during open trades."
- "the minimum account balance follows your highest end-of-day balance upward … This continues until the minimum account balance reaches the original starting balance of 25,000. Once it reaches that point, it stops trailing and remains fixed."
- "If your account drops to the Minimum Account Balance at any time — through realized or unrealized losses — the account is immediately liquidated."

The type is `trailing_eod`: the level updates at EOD, but the breach is checked in real time. The homepage tooltip says the same thing: "The drawdown is End Of Day on test accounts, meaning it updates at the end of the trading day (5PM EST) … The drawdown across all three account types will STOP trailing once it has reached the initial account balance."

**Daily loss limit: none.**
- The homepage journey table row "Daily loss rule" reads "None / None / None" for Test / PRO / PRO +.
- Every homepage size card shows "Daily Loss Limit $500 Removed" (and $1100 / $1600 / $2200 / $3300 for the other sizes). "Removed" is a separate `<span class="removed">`, which means the old DLL value is shown as withdrawn.
- The help-center Test rules (the "6 Core Rules") have no DLL.

**Max position** (Rule 2), quoted:
- "Your maximum position size is a static number of contracts that you can have open at once … You cannot exceed this limit for your account."
- "you are allowed 10x position size on permitted Micro products. For example on the $50K account you can trade up to 60 micro contracts." The mini:micro ratio is 1:10.
- No page says whether minis and micros can be combined in one position (§9).

**Consistency** (Rule 5), quoted:
- "your best trading day must be below 50% of your total net profits … Highest profit day / Net P/L = Consistency Percentage. This percentage must be below 50% to qualify for a PRO account."
- "If your consistency percentage is above 50%, you do not meet the consistency rule. However, you have not failed the test account; you simply need to increase your total profit goal".
- The firm also publishes this formula: "Updated Profit Goal > High Profit Day's Net P/L × 2".
- The homepage tooltip words it differently: "Your biggest trading day cannot be more than 50% of your total profit." The two wordings disagree only at exactly 50% ("below 50%" vs "not more than 50%") → `conflict` on the boundary (§9).

**Minimum trading days** (Rule 5), quoted:
- "you must trade for a minimum of 3 trading days. A trading day is defined as any day in which you place at least one trade."
- From the 50AND3 FAQ (https://takeprofittraderhelp.zendesk.com/hc/en-us/articles/38320927906717-50AND3-PROMO-FAQS): "evaluations are moving from 5 days to 3 days. This is a program change … The 3-day eval applies to new test accounts purchased from August 17th onward. Existing tests and resets associated with those tests will still have the 5-day eval period." The page does not state the year.
- The homepage comparison table says: "Number of Trading Days to "PRO Account" — 3 Days".

**Other Test facts:**
- No time limit. Quote from https://takeprofittraderhelp.zendesk.com/hc/en-us/articles/15141145057053-Test-Subscriptions: "there is no time limit to reach your profit target".
- News trading is allowed in Test. Homepage tooltip: "This does not apply for test accounts and you are free to trade through all news events."

### 3.2 PRO (funded, SIM)

| Size | Max drawdown (intraday trailing) | Daily loss limit | Max position | Consistency | Profit target | Buffer zone (withdrawal) |
|---|---|---|---|---|---|---|
| 25K | $1,500 | none | 3 / 30 micros | none | none | $26,500 |
| 50K | $2,000 | none | 6 / 60 | none | none | $52,000 |
| 75K | $2,500 | none | 9 / 90 | none | none | $77,500 |
| 100K | $3,000 | none | 12 / 120 | none | none | $103,000 |
| 150K | $4,500 | none | 15 / 150 | none | none | $154,500 |

**Drawdown** (PRO Account Rules §5), quoted:
- "The Trailing Drawdown is calculated intraday using your peak balance, which includes realized gains and unrealized gains. The drawdown trails intraday as your unrealized profits rise. The drawdown will never exceed your starting account balance. Maximum drawdown in PRO = maximum drawdown from your passed test."
- "The drawdown continues to trail profits until it reaches your starting balance; once it reaches that level, it no longer moves."
- "If your account balance drops to the Minimum Account Balance at any time — through realized or unrealized losses — the account is immediately liquidated."

The type is `trailing_intraday`. The amounts in the table come from the Test drawdown table, because the firm publishes the rule that the PRO drawdown equals the passed test's drawdown.

**Max position.** The PRO rules article has no size table. The firm publishes that the size equals the Test size:
- Homepage tooltip: "At Take Profit Trader, you get the exact same buying power in your PRO Account that you had in your test account. No gimmicks or "scaling"."
- Homepage comparison table: "Contract Limits: Test vs. PRO Account — No".

**No DLL, no consistency rule, no scaling plan:**
- Homepage table: "Daily loss rule None", "Consistency rule None", "Scaling plan None".
- 50AND3 FAQ: "No funded consistency rule." "No scaling plan on any account."

**Withdrawal buffer** (it affects payouts only, not trading). Source: https://takeprofittraderhelp.zendesk.com/hc/en-us/articles/15172219527581-PRO-Account-Profit-Split-Withdrawal-Rules. Quoted: "The buffer is equal to the maximum drawdown amount on your account." The table values are shown above.

**Other PRO rules** (PRO Account Rules): no bots/algos, limit up/down exit, weekly trading requirement, no counter positions, news restriction (see §6–§7).

**PRO reset**, quoted from https://takeprofittraderhelp.zendesk.com/hc/en-us/articles/15171895352733-Resetting-A-PRO-Account: "we allow you to reset your PRO Account up to three times per account."

### 3.3 PRO+ (live; invitation-only upgrade from PRO)

| From PRO size | Starting balance | Drawdown (EOD) | Stops trailing at | Daily loss limit | Max position | Consistency |
|---|---|---|---|---|---|---|
| 25K / 50K / 75K / 100K / 150K | $0 | "equivalent to the starting drawdown of the original PRO account" (e.g. 50K → "-$2,000") | $0 | none | **not published** | none |

- Structure, quoted from the Upgrade article: "The PRO+ account will begin with a $0 balance and an initial EOD drawdown equivalent to the starting drawdown of the original PRO account. For example, if the PRO account was a $50,000 account, the PRO+ account would start at $0 with an EOD drawdown of -$2,000. As trades become profitable, the minimum account balance will trail accordingly. If the trader earns $500 in realized profit at the end of trading day, the minimum balance would trail $500 up, stopping once it reaches $0, where it will remain."
  - Only the 50K example amount is published as a PRO+ number. For the other sizes, the YAML uses the PRO drawdown amount under the firm's published equivalence rule.
- PRO+ Account Rules §2, quoted: "Your account balance—including open positions—must not reach or exceed the maximum EOD drawdown limit."
- Also from the Upgrade article: "$5,000 in profit from the PRO account will be frozen and must remain in the account while the PRO+ is active." And: "Once the upgrade to PRO+ is complete, all future trading and withdrawals must be done on the PRO+ account only."
- DLL / consistency / scaling: the homepage table shows "None" for PRO +.
- Max position is not stated for standard PRO+. The scaling tooltip covers only PRO vs Test → gap (§9).
- PRO+ cannot be reset. Quote from https://takeprofittraderhelp.zendesk.com/hc/en-us/articles/30166235705629-Resetting-A-PRO-Account: "resetting a PRO+ account directly is not possible."
- Self-Match Prevention applies to all PRO+ accounts. CME may reject an order that would self-match (https://takeprofittraderhelp.zendesk.com/hc/en-us/articles/31254068167837-Self-Match-Prevention-SMP-Implementation-for-PRO-Accounts).

### 3.4 PRO+ Development (live; assigned by TPT)

| Size | Contract Maximum (Mini/Micro) | Daily Loss Limit (Soft Breach) | End of Day Drawdown |
|---|---|---|---|
| 25K | 1/10 | $500 | $750 |
| 50K | 2/20 | $1,000 | $1,250 |
| 75K | 3/30 | $1,250 | $1,500 |
| 100K | 4/40 | $1,500 | $1,750 |
| 150K | 5/50 | $2,000 | $2,250 |

- Quoted: "PRO+ Development is designed to protect both, with three differences from standard PRO+: Reduced contract sizing; Soft breach daily loss limit (pauses you, doesn't fail you); Smaller end of day drawdown." The DLL kind is `soft_pause`. When the pause ends is not stated (§9).
- The article does not say where the drawdown stops trailing (§9).
- 30-day cooldown, quoted from https://takeprofittraderhelp.zendesk.com/hc/en-us/articles/39331980656925-PRO-Development-30-Day-Live-Capital-Cooldown: "Over any rolling 30-day window, we track a trader's net loss of TPT's live capital across their PRO+ Development accounts. If that total loss reaches $10,000, a "cooldown" is initiated and new purchases are paused for 30 days. Every account the trader already owns keeps trading normally".

## 4. Trading-day reset

Source: Rule 4, https://takeprofittraderhelp.zendesk.com/hc/en-us/articles/15170347090461-Rule-4-Trade-Approved-Products-During-Approved-Hours. Quoted:

> "Trading window: 6 PM Eastern – 5 PM Eastern
> The trading day ends at 5 PM Eastern
> TPT will automatically close any open position at 4:55 PM Eastern.
> You may reopen positions starting at 6 PM Eastern, but any trade opened at or after 6 PM counts toward the next trading day."

- **No overnight holding:** "Positions cannot be carried from one trading day into the next." UTP #6 applies this to all accounts: "positions may not be held from one session to the next."
- **Early market close:** "If the market for your product closes before 5 PM Eastern and you cannot exit before the deadline, the account will be liquidated."
- **Holidays:** "Market hours can change due to holidays. It is the trader's responsibility to monitor holiday schedules. Missing a holiday-adjusted close time will result in account liquidation."
- **Time-zone label:** the homepage drawdown tooltip says "(5PM EST)". Both pages say 5 PM, but the labels differ ("Eastern" vs "EST") → the label is recorded as not confirmed (§9).
- **Dashboard timing:** the dashboard updates balances "between 08:00 PM and 09:00 PM Eastern Time" (https://takeprofittraderhelp.zendesk.com/hc/en-us/articles/15168980013085-Keeping-Track-of-Your-Progress). This is a display delay, not a reset.

## 5. Trade copying stance — `conditional` (allowed across your own accounts, approved copiers only)

Source: Trade Copier Policy, https://takeprofittraderhelp.zendesk.com/hc/en-us/articles/34431176505245-Trade-Copier-Policy (updated 2026-10-22). Quoted verbatim:

> "TakeProfitTrader permits the use of trade copiers solely for managing accounts owned and controlled by you.
> Trade copiers may not be used, directly or indirectly, to coordinate trading activity between different users, identities, or accounts in violation of the Independent Trade Execution Policy."

> "Approved Trade Copiers
> The following trade copiers are approved for use:
> - Tradesyncer
> - TradeCopia
> - Affordable Indicators
> - Replikanto Flowbot - Compliance Edition only
> - Platform-Native Copier Tools (Tradovate, NinjaTrader, MotiveWave, Quantower, etc)
>
> Use of unapproved trade copiers, modified versions, or non-compliance editions, or copier-based services may result in enforcement action. TakeProfitTrader reserves the right to verify compliance and modify the list of approved trade copiers at any time."

> "Permitted uses include:
> - Copy trades across your accounts
> - Manage multiple accounts as part of your individual trading strategy
> - Improve execution efficiency across your own accounts"

> "Trade copiers may not be used to:
> - Copy or synchronize trades between different users or identities
> - Coordinate execution across accounts not owned and controlled by you
> - Enter opposing or offsetting positions designed to reduce, transfer, or neutralize risk
> - Participate in pass services, payout services, or coordinated trading schemes
> - Attempt to circumvent the Independent Trade Execution Policy"

> "Please note, traders are fully responsible for the use of any trade copiers or third-party automation tools connected to their Take Profit Trader accounts. … Any trades executed through a copier or automation tool are treated as the trader's own actions under all rules and policies."

Independent Trade Execution Policy, https://takeprofittraderhelp.zendesk.com/hc/en-us/articles/34431223270557-Independent-Trade-Execution-Policy, quoted:

> "Synchronization across accounts solely owned and controlled by you is permitted, provided such synchronization does not involve coordinated opposing or offsetting positions designed to transfer, reduce, or neutralize market risk.
> Synchronization across accounts operated under separated identities is not permitted."

> "Tools that assist with trade execution or management, including trade copiers, are allowed, but only for accounts solely owned and controlled by you."

- **Same person's accounts:** allowed, using approved copiers only.
- **Product impact:** All Win is not on the approved list. "Platform-Native Copier Tools (Tradovate, NinjaTrader, …)" are approved. The policy does not say whether an external routing service such as All Win counts as an approved tool or as an "unapproved trade copier" / "copier-based service" → open question for the user (§9).
- **Account-count limits:** see §7. Test is unlimited, and up to 5 active PRO/PRO+ accounts are allowed.
- **Copying to/from accounts at other firms:** not mentioned. The policy only limits copying to "accounts owned and controlled by you". It does not address accounts at other firms or a personal brokerage account (§9).
- **Counter positions across your own accounts are forbidden.** This matters for copy setups and reverse copying. Rule 6 (https://takeprofittraderhelp.zendesk.com/hc/en-us/articles/30331694826909-Rule-6-No-Counter-Positions), quoted:
  - "Take Profit Trader strictly prohibits holding opposite positions in the same or closely related products across any accounts under the same beneficial control."
  - The related pairs include "NQ ↔ MNQ".
  - FAQ Q11: "If a trade copier causes opposite positions, does that count as a Rule 6 violation? Yes."
  - Penalty: "Both accounts will be automatically liquidated."
  - FAQ Q5: "Yes — multiple accounts are allowed, as long as you never hold opposite positions (long vs short) in the same or correlated contracts across those accounts."

## 6. Automated trading stance — `forbidden` (YAML `automated_trading_allowed: no`)

**(i) Classification:** `forbidden` for bots/algos/automated execution in every phase. Trade copiers are a separate, conditional exception (§5).

Universal Trading Policies, https://takeprofittraderhelp.zendesk.com/hc/en-us/articles/34431153546397-TakeProfitTrader-Universal-Trading-Policies-UTP (applies to "all TakeProfitTrader accounts, including Test, PRO, and PRO+"). Verbatim, used as `automation_rule_text`:

> "#1: No Trading Bots or Algos. Automated trading systems, bots, or algorithmic execution tools are not permitted."

PRO Account Rules, https://takeprofittraderhelp.zendesk.com/hc/en-us/articles/15171769361053-PRO-Account-Rules. Verbatim:

> "1. No Trading bots/Algos
> - We do not allow any automated or bot trading of any kind.
> - All trades must be manually executed by the trader."

The homepage Test size cards say "Be consistent  No trading bots  No counter positions" (https://takeprofittrader.com/).

**(ii) Specific mentions:**
- **Semi-automated trading:** not mentioned.
- **Trade copiers:** mentioned and allowed under conditions (quotes in §5).
- **"Third-party automation tools":** mentioned only in the liability clause, not as a permission.
  - Trade Copier Policy: "traders are fully responsible for the use of any trade copiers or third-party automation tools connected to their Take Profit Trader accounts."
  - Terms §VII (https://takeprofittrader.com/terms), verbatim: "Traders are fully responsible for the use of any trade copiers or third-party automation tools connected to their TakeProfit Trader accounts. This includes ensuring the proper configuration, monitoring all copied activity, and accepting all risks or consequences resulting from their use. TakeProfit Trader is not liable for losses, violations, or account actions arising from trade copier activity, and all such activity will be treated as the trader's own execution."
  - These pages acknowledge such tools but do not permit automated execution, while UTP #1 forbids it → ambiguity noted in §9.
- **Execution aids:** the Independent Trade Execution Policy says: "You are free to use tools that assist with order execution or trade management, including trade copiers."
- **Webhooks / alerts:** not mentioned.
- **ATM / automatic brackets:** not mentioned.

**(iii) Differs by phase?** No:
- UTP #1 applies to Test, PRO and PRO+.
- The PRO article repeats the rule and adds "All trades must be manually executed by the trader."
- The PRO+ Account Rules article does not repeat the bot rule, but UTP covers PRO+.

## 7. Other rules that affect routing or risk

- **Max accounts per person**, quoted from https://takeprofittraderhelp.zendesk.com/hc/en-us/articles/15172695563933-Rules-for-Multiple-Accounts:
  - "Traders can have as many test accounts as they want and trade them simultaneously."
  - "You can trade up to five active funded accounts. When you move to PRO+, your full five-account allowance moves with you."
  - "The trader has the privilege to activate up to 10 successfully passed tests within a span of 30 calendar days."
  - "The limit is 5 active funded accounts per person."
  - Terms (https://takeprofittrader.com/terms): "each customer is permitted to register only one email address" and "Traders may have a maximum of 5 active PRO/PRO+ accounts simultaneously. Any account beyond this initial limit of 5 will be queued for future activation."
  - Homepage tooltip: "up to five PRO/PRO+ accounts and trade them at the same time (e.g., 3 PRO and 2 PRO+, or vice versa, or 5 PRO / 5 PRO+)".
- **News (PRO, PRO+)**, quoted from the PRO and PRO+ rules:
  - "All PRO/PRO+ Accounts must be out of all open positions and have no open orders one minute before, during and one minute after any prohibited news event."
  - The events: "FOMC statements/announcements Wednesdays at 2:00 PM ET (FED speakers, FOMC meeting minutes are allowed)", "Non-Farm Payroll (NFP) Monthly, Fridays at 8:30 AM ET", "Consumer Price Index (CPI)".
  - Test: free to trade news (§3.1).
- **Limit up/down (PRO, PRO+)**, quoted: "You must exit all open positions before a price limit is reached. … If a price limit is hit, and you have not exited your position, you will lose your PRO Account."
- **Weekly activity (PRO, PRO+)**, quoted: "you must trade at least one day per calendar week (Sunday–Friday). A "traded day" is defined as a day where at least one round-trip has been executed".
- **Counter positions (all accounts):** see §5. It covers the same and related products (NQ ↔ MNQ) across all accounts under the same beneficial control.
- **Independent trading:** coordinated trading with other users is forbidden (§5).
- **Discretionary risk review** (https://takeprofittraderhelp.zendesk.com/hc/en-us/articles/36569493361693-Sustainable-Trading-Policy). The listed risk patterns include "Using maximum position size on the majority of trades" and "Account churning". Quoted: "TPT may part ways as funding partners without prior notice." This is not a numeric rule, but it is relevant to automated sizing.
- **Overnight / weekend holding:** not allowed. Positions are auto-closed at 4:55 PM Eastern (§4).
- **Mandatory stop loss, minimum/maximum trade duration:** not mentioned.
- **Scaling plan:** none on any account (§3.2), except the fixed PRO+ Development table.
- **Inactivity:** the weekly trading requirement above applies to PRO/PRO+. No inactivity rule is published for Test (a monthly subscription).

### 7.1 Device / hosting rules

- **VPS / server hosting:** not mentioned. I searched all 88 help-center articles (raw/zd_articles_1.json and the .txt files), the terms and the homepage text for "VPS", "virtual private server", "server", "remote", "device" and "personal device". No rule allows or forbids trading from a VPS or server → `vps_allowed: unknown` (§9 #14).
- **Personal device:** not mentioned. No page requires orders to come from the trader's own device → `personal_device_required: unknown` (§9 #14).
- **VPN** (this is about VPNs, not VPS hosting). Registration Denial FAQ, https://takeprofittraderhelp.zendesk.com/hc/en-us/articles/37085469996573-Registration-Denial-FAQ (accessed 2026-10-23), verbatim:
  > "Q: Is VPN usage prohibited on TakeProfitTrader?
  > A: VPN usage is specifically restricted during the account registration process. New customers must complete registration without an active VPN connection. This registration restriction should not be interpreted as a general prohibition on VPN usage after an account has been successfully registered."
- **Remote connections from restricted countries.** Restricted Countries, https://takeprofittraderhelp.zendesk.com/hc/en-us/articles/22447557656477-Restricted-Countries (accessed 2026-10-23), verbatim:
  > "We kindly ask all users to ensure that they do not access their accounts from restricted jurisdictions at any time. This includes the use of VPNs, remote connections, or shared networks that may route activity through a restricted country."

  A VPS located in, or routed through, a restricted country would therefore be a risk.
- **R4 re-check, personal device (2026-10-23).** All 88 help-center articles (Zendesk API) and the Terms of Service (https://takeprofittrader.com/terms, read through the portal) were grepped for device, computer, IP address, remote, VPS, virtual private, server, cloud, machine, third party, behalf, share, credentials. No text requires orders to come from the trader's own device or IP, and none permits other or remote devices, so `personal_device_required` stays `unknown`. Related, not a device rule: Universal Trading Policies, https://takeprofittraderhelp.zendesk.com/hc/en-us/articles/34431153546397-TakeProfitTrader-Universal-Trading-Policies-UTP (accessed 2026-10-23), #4:
  > "Coordinated trading, third-party account management, or participation in structured trading schemes designed to reduce or neutralize risk is prohibited."

  Terms of Service §VII: "all such activity will be treated as the trader’s own execution." (https://takeprofittrader.com/terms, accessed 2026-10-23).
- **Shared infrastructure as a surveillance signal.** Independent Trade Execution Policy, https://takeprofittraderhelp.zendesk.com/hc/en-us/articles/34431223270557-Independent-Trade-Execution-Policy, verbatim:
  > "Device or infrastructure clustering"

  > "No single signal (such as shared IP address or device) constitutes a violation in isolation. Combined indicators reasonably suggesting centralized control, coordinated execution, or structured risk dispersion may result in enforcement action."


---

## 8. Machine-readable blocks

Notes:
- Platforms are listed as published (data feeds first, then platforms on them).
- `copy_trading_allowed: conditional` = own accounts only, approved copiers only, no opposite positions.
- PRO+ blocks carry the originating PRO size. The PRO+ account itself starts at $0.

### 8.1 Test (evaluation)


```yaml
firm: take-profit-trader
plan: "Test"
phase: evaluation
size_label: "25K"
size_usd: "25000.00"
instruments: [MNQ, NQ]
platforms: [CQG, Rithmic, TradingView, NinjaTrader, Tradovate, Bookmap, MultiCharts, MotiveWave, Trade Navigator, "R|Trader", VolFix, ATAS, "Investor R/T", Quantower, Finamark, Jigsaw Trader]
drawdown: { amount_usd: "1500.00", type: trailing_eod, stops_trailing_at: "stops trailing when the minimum account balance reaches the starting balance (the size); then fixed" }
daily_loss_limit: { amount_usd: null, kind: none }
profit_target_usd: "1500.00"
consistency_rule: { text: "your best trading day must be below 50% of your total net profits", threshold_pct: "50" }
max_position: { minis: 3, micros: 30, scaling: "none (no scaling plan)" }
min_trading_days: 3   # tests purchased August 17th onward; earlier tests: 5 (50AND3 FAQ)
trading_day_reset: { time: "17:00", tz_as_published: "Eastern (Rule 4 article: '5 PM Eastern'); homepage tooltip: '5PM EST'" }
copy_trading_allowed: conditional   # own accounts only, approved copiers only; see §5
automated_trading_allowed: no
automation_rule_text: "#1: No Trading Bots or Algos. Automated trading systems, bots, or algorithmic execution tools are not permitted."
vps_allowed: unknown                # VPS not mentioned; VPN allowed after registration (§7.1)
personal_device_required: unknown   # not mentioned (§7.1)
status: partial
not_confirmed_fields: [consistency_rule.boundary, trading_day_reset.tz_label]
sources:
  - { field: profit_target, url: "https://takeprofittraderhelp.zendesk.com/hc/en-us/articles/15169070804125-Rule-1-Hit-Your-Profit-Target", accessed: "2026-10-23" }
  - { field: max_position, url: "https://takeprofittraderhelp.zendesk.com/hc/en-us/articles/15169066911133-Rule-2-Do-Not-Exceed-Maximum-Position-Size", accessed: "2026-10-23" }
  - { field: drawdown, url: "https://takeprofittraderhelp.zendesk.com/hc/en-us/articles/15170265979165-Rule-3-Do-Not-Hit-End-Of-Day-EOD-Maximum-Trailing-Drawdown", accessed: "2026-10-23" }
  - { field: daily_loss_limit, url: "https://takeprofittrader.com/", accessed: "2026-10-23" }
  - { field: consistency_rule, url: "https://takeprofittraderhelp.zendesk.com/hc/en-us/articles/15170316538013-Rule-5-Be-Consistent", accessed: "2026-10-23" }
  - { field: min_trading_days, url: "https://takeprofittraderhelp.zendesk.com/hc/en-us/articles/15170316538013-Rule-5-Be-Consistent", accessed: "2026-10-23" }
  - { field: min_trading_days, url: "https://takeprofittraderhelp.zendesk.com/hc/en-us/articles/38320927906717-50AND3-PROMO-FAQS", accessed: "2026-10-23" }
  - { field: trading_day_reset, url: "https://takeprofittraderhelp.zendesk.com/hc/en-us/articles/15170347090461-Rule-4-Trade-Approved-Products-During-Approved-Hours", accessed: "2026-10-23" }
  - { field: platforms, url: "https://takeprofittraderhelp.zendesk.com/hc/en-us/articles/15173017163549-Choosing-Your-Platform", accessed: "2026-10-23" }
  - { field: copy_trading, url: "https://takeprofittraderhelp.zendesk.com/hc/en-us/articles/34431176505245-Trade-Copier-Policy", accessed: "2026-10-23" }
  - { field: automation, url: "https://takeprofittraderhelp.zendesk.com/hc/en-us/articles/34431153546397-TakeProfitTrader-Universal-Trading-Policies-UTP", accessed: "2026-10-23" }
  - { field: vps_allowed, url: "https://takeprofittraderhelp.zendesk.com/hc/en-us/articles/37085469996573-Registration-Denial-FAQ", accessed: "2026-10-23" }
  - { field: vps_allowed, url: "https://takeprofittraderhelp.zendesk.com/hc/en-us/articles/22447557656477-Restricted-Countries", accessed: "2026-10-23" }
```

```yaml
firm: take-profit-trader
plan: "Test"
phase: evaluation
size_label: "50K"
size_usd: "50000.00"
instruments: [MNQ, NQ]
platforms: [CQG, Rithmic, TradingView, NinjaTrader, Tradovate, Bookmap, MultiCharts, MotiveWave, Trade Navigator, "R|Trader", VolFix, ATAS, "Investor R/T", Quantower, Finamark, Jigsaw Trader]
drawdown: { amount_usd: "2000.00", type: trailing_eod, stops_trailing_at: "stops trailing when the minimum account balance reaches the starting balance (the size); then fixed" }
daily_loss_limit: { amount_usd: null, kind: none }
profit_target_usd: "3000.00"
consistency_rule: { text: "your best trading day must be below 50% of your total net profits", threshold_pct: "50" }
max_position: { minis: 6, micros: 60, scaling: "none (no scaling plan)" }
min_trading_days: 3   # tests purchased August 17th onward; earlier tests: 5 (50AND3 FAQ)
trading_day_reset: { time: "17:00", tz_as_published: "Eastern (Rule 4 article: '5 PM Eastern'); homepage tooltip: '5PM EST'" }
copy_trading_allowed: conditional   # own accounts only, approved copiers only; see §5
automated_trading_allowed: no
automation_rule_text: "#1: No Trading Bots or Algos. Automated trading systems, bots, or algorithmic execution tools are not permitted."
vps_allowed: unknown                # VPS not mentioned; VPN allowed after registration (§7.1)
personal_device_required: unknown   # not mentioned (§7.1)
status: partial
not_confirmed_fields: [consistency_rule.boundary, trading_day_reset.tz_label]
sources:
  - { field: profit_target, url: "https://takeprofittraderhelp.zendesk.com/hc/en-us/articles/15169070804125-Rule-1-Hit-Your-Profit-Target", accessed: "2026-10-23" }
  - { field: max_position, url: "https://takeprofittraderhelp.zendesk.com/hc/en-us/articles/15169066911133-Rule-2-Do-Not-Exceed-Maximum-Position-Size", accessed: "2026-10-23" }
  - { field: drawdown, url: "https://takeprofittraderhelp.zendesk.com/hc/en-us/articles/15170265979165-Rule-3-Do-Not-Hit-End-Of-Day-EOD-Maximum-Trailing-Drawdown", accessed: "2026-10-23" }
  - { field: daily_loss_limit, url: "https://takeprofittrader.com/", accessed: "2026-10-23" }
  - { field: consistency_rule, url: "https://takeprofittraderhelp.zendesk.com/hc/en-us/articles/15170316538013-Rule-5-Be-Consistent", accessed: "2026-10-23" }
  - { field: min_trading_days, url: "https://takeprofittraderhelp.zendesk.com/hc/en-us/articles/15170316538013-Rule-5-Be-Consistent", accessed: "2026-10-23" }
  - { field: min_trading_days, url: "https://takeprofittraderhelp.zendesk.com/hc/en-us/articles/38320927906717-50AND3-PROMO-FAQS", accessed: "2026-10-23" }
  - { field: trading_day_reset, url: "https://takeprofittraderhelp.zendesk.com/hc/en-us/articles/15170347090461-Rule-4-Trade-Approved-Products-During-Approved-Hours", accessed: "2026-10-23" }
  - { field: platforms, url: "https://takeprofittraderhelp.zendesk.com/hc/en-us/articles/15173017163549-Choosing-Your-Platform", accessed: "2026-10-23" }
  - { field: copy_trading, url: "https://takeprofittraderhelp.zendesk.com/hc/en-us/articles/34431176505245-Trade-Copier-Policy", accessed: "2026-10-23" }
  - { field: automation, url: "https://takeprofittraderhelp.zendesk.com/hc/en-us/articles/34431153546397-TakeProfitTrader-Universal-Trading-Policies-UTP", accessed: "2026-10-23" }
  - { field: vps_allowed, url: "https://takeprofittraderhelp.zendesk.com/hc/en-us/articles/37085469996573-Registration-Denial-FAQ", accessed: "2026-10-23" }
  - { field: vps_allowed, url: "https://takeprofittraderhelp.zendesk.com/hc/en-us/articles/22447557656477-Restricted-Countries", accessed: "2026-10-23" }
```

```yaml
firm: take-profit-trader
plan: "Test"
phase: evaluation
size_label: "75K"
size_usd: "75000.00"
instruments: [MNQ, NQ]
platforms: [CQG, Rithmic, TradingView, NinjaTrader, Tradovate, Bookmap, MultiCharts, MotiveWave, Trade Navigator, "R|Trader", VolFix, ATAS, "Investor R/T", Quantower, Finamark, Jigsaw Trader]
drawdown: { amount_usd: "2500.00", type: trailing_eod, stops_trailing_at: "stops trailing when the minimum account balance reaches the starting balance (the size); then fixed" }
daily_loss_limit: { amount_usd: null, kind: none }
profit_target_usd: "4500.00"
consistency_rule: { text: "your best trading day must be below 50% of your total net profits", threshold_pct: "50" }
max_position: { minis: 9, micros: 90, scaling: "none (no scaling plan)" }
min_trading_days: 3   # tests purchased August 17th onward; earlier tests: 5 (50AND3 FAQ)
trading_day_reset: { time: "17:00", tz_as_published: "Eastern (Rule 4 article: '5 PM Eastern'); homepage tooltip: '5PM EST'" }
copy_trading_allowed: conditional   # own accounts only, approved copiers only; see §5
automated_trading_allowed: no
automation_rule_text: "#1: No Trading Bots or Algos. Automated trading systems, bots, or algorithmic execution tools are not permitted."
vps_allowed: unknown                # VPS not mentioned; VPN allowed after registration (§7.1)
personal_device_required: unknown   # not mentioned (§7.1)
status: partial
not_confirmed_fields: [consistency_rule.boundary, trading_day_reset.tz_label]
sources:
  - { field: profit_target, url: "https://takeprofittraderhelp.zendesk.com/hc/en-us/articles/15169070804125-Rule-1-Hit-Your-Profit-Target", accessed: "2026-10-23" }
  - { field: max_position, url: "https://takeprofittraderhelp.zendesk.com/hc/en-us/articles/15169066911133-Rule-2-Do-Not-Exceed-Maximum-Position-Size", accessed: "2026-10-23" }
  - { field: drawdown, url: "https://takeprofittraderhelp.zendesk.com/hc/en-us/articles/15170265979165-Rule-3-Do-Not-Hit-End-Of-Day-EOD-Maximum-Trailing-Drawdown", accessed: "2026-10-23" }
  - { field: daily_loss_limit, url: "https://takeprofittrader.com/", accessed: "2026-10-23" }
  - { field: consistency_rule, url: "https://takeprofittraderhelp.zendesk.com/hc/en-us/articles/15170316538013-Rule-5-Be-Consistent", accessed: "2026-10-23" }
  - { field: min_trading_days, url: "https://takeprofittraderhelp.zendesk.com/hc/en-us/articles/15170316538013-Rule-5-Be-Consistent", accessed: "2026-10-23" }
  - { field: min_trading_days, url: "https://takeprofittraderhelp.zendesk.com/hc/en-us/articles/38320927906717-50AND3-PROMO-FAQS", accessed: "2026-10-23" }
  - { field: trading_day_reset, url: "https://takeprofittraderhelp.zendesk.com/hc/en-us/articles/15170347090461-Rule-4-Trade-Approved-Products-During-Approved-Hours", accessed: "2026-10-23" }
  - { field: platforms, url: "https://takeprofittraderhelp.zendesk.com/hc/en-us/articles/15173017163549-Choosing-Your-Platform", accessed: "2026-10-23" }
  - { field: copy_trading, url: "https://takeprofittraderhelp.zendesk.com/hc/en-us/articles/34431176505245-Trade-Copier-Policy", accessed: "2026-10-23" }
  - { field: automation, url: "https://takeprofittraderhelp.zendesk.com/hc/en-us/articles/34431153546397-TakeProfitTrader-Universal-Trading-Policies-UTP", accessed: "2026-10-23" }
  - { field: vps_allowed, url: "https://takeprofittraderhelp.zendesk.com/hc/en-us/articles/37085469996573-Registration-Denial-FAQ", accessed: "2026-10-23" }
  - { field: vps_allowed, url: "https://takeprofittraderhelp.zendesk.com/hc/en-us/articles/22447557656477-Restricted-Countries", accessed: "2026-10-23" }
```

```yaml
firm: take-profit-trader
plan: "Test"
phase: evaluation
size_label: "100K"
size_usd: "100000.00"
instruments: [MNQ, NQ]
platforms: [CQG, Rithmic, TradingView, NinjaTrader, Tradovate, Bookmap, MultiCharts, MotiveWave, Trade Navigator, "R|Trader", VolFix, ATAS, "Investor R/T", Quantower, Finamark, Jigsaw Trader]
drawdown: { amount_usd: "3000.00", type: trailing_eod, stops_trailing_at: "stops trailing when the minimum account balance reaches the starting balance (the size); then fixed" }
daily_loss_limit: { amount_usd: null, kind: none }
profit_target_usd: "6000.00"
consistency_rule: { text: "your best trading day must be below 50% of your total net profits", threshold_pct: "50" }
max_position: { minis: 12, micros: 120, scaling: "none (no scaling plan)" }
min_trading_days: 3   # tests purchased August 17th onward; earlier tests: 5 (50AND3 FAQ)
trading_day_reset: { time: "17:00", tz_as_published: "Eastern (Rule 4 article: '5 PM Eastern'); homepage tooltip: '5PM EST'" }
copy_trading_allowed: conditional   # own accounts only, approved copiers only; see §5
automated_trading_allowed: no
automation_rule_text: "#1: No Trading Bots or Algos. Automated trading systems, bots, or algorithmic execution tools are not permitted."
vps_allowed: unknown                # VPS not mentioned; VPN allowed after registration (§7.1)
personal_device_required: unknown   # not mentioned (§7.1)
status: partial
not_confirmed_fields: [consistency_rule.boundary, trading_day_reset.tz_label]
sources:
  - { field: profit_target, url: "https://takeprofittraderhelp.zendesk.com/hc/en-us/articles/15169070804125-Rule-1-Hit-Your-Profit-Target", accessed: "2026-10-23" }
  - { field: max_position, url: "https://takeprofittraderhelp.zendesk.com/hc/en-us/articles/15169066911133-Rule-2-Do-Not-Exceed-Maximum-Position-Size", accessed: "2026-10-23" }
  - { field: drawdown, url: "https://takeprofittraderhelp.zendesk.com/hc/en-us/articles/15170265979165-Rule-3-Do-Not-Hit-End-Of-Day-EOD-Maximum-Trailing-Drawdown", accessed: "2026-10-23" }
  - { field: daily_loss_limit, url: "https://takeprofittrader.com/", accessed: "2026-10-23" }
  - { field: consistency_rule, url: "https://takeprofittraderhelp.zendesk.com/hc/en-us/articles/15170316538013-Rule-5-Be-Consistent", accessed: "2026-10-23" }
  - { field: min_trading_days, url: "https://takeprofittraderhelp.zendesk.com/hc/en-us/articles/15170316538013-Rule-5-Be-Consistent", accessed: "2026-10-23" }
  - { field: min_trading_days, url: "https://takeprofittraderhelp.zendesk.com/hc/en-us/articles/38320927906717-50AND3-PROMO-FAQS", accessed: "2026-10-23" }
  - { field: trading_day_reset, url: "https://takeprofittraderhelp.zendesk.com/hc/en-us/articles/15170347090461-Rule-4-Trade-Approved-Products-During-Approved-Hours", accessed: "2026-10-23" }
  - { field: platforms, url: "https://takeprofittraderhelp.zendesk.com/hc/en-us/articles/15173017163549-Choosing-Your-Platform", accessed: "2026-10-23" }
  - { field: copy_trading, url: "https://takeprofittraderhelp.zendesk.com/hc/en-us/articles/34431176505245-Trade-Copier-Policy", accessed: "2026-10-23" }
  - { field: automation, url: "https://takeprofittraderhelp.zendesk.com/hc/en-us/articles/34431153546397-TakeProfitTrader-Universal-Trading-Policies-UTP", accessed: "2026-10-23" }
  - { field: vps_allowed, url: "https://takeprofittraderhelp.zendesk.com/hc/en-us/articles/37085469996573-Registration-Denial-FAQ", accessed: "2026-10-23" }
  - { field: vps_allowed, url: "https://takeprofittraderhelp.zendesk.com/hc/en-us/articles/22447557656477-Restricted-Countries", accessed: "2026-10-23" }
```

```yaml
firm: take-profit-trader
plan: "Test"
phase: evaluation
size_label: "150K"
size_usd: "150000.00"
instruments: [MNQ, NQ]
platforms: [CQG, Rithmic, TradingView, NinjaTrader, Tradovate, Bookmap, MultiCharts, MotiveWave, Trade Navigator, "R|Trader", VolFix, ATAS, "Investor R/T", Quantower, Finamark, Jigsaw Trader]
drawdown: { amount_usd: "4500.00", type: trailing_eod, stops_trailing_at: "stops trailing when the minimum account balance reaches the starting balance (the size); then fixed" }
daily_loss_limit: { amount_usd: null, kind: none }
profit_target_usd: "9000.00"
consistency_rule: { text: "your best trading day must be below 50% of your total net profits", threshold_pct: "50" }
max_position: { minis: 15, micros: 150, scaling: "none (no scaling plan)" }
min_trading_days: 3   # tests purchased August 17th onward; earlier tests: 5 (50AND3 FAQ)
trading_day_reset: { time: "17:00", tz_as_published: "Eastern (Rule 4 article: '5 PM Eastern'); homepage tooltip: '5PM EST'" }
copy_trading_allowed: conditional   # own accounts only, approved copiers only; see §5
automated_trading_allowed: no
automation_rule_text: "#1: No Trading Bots or Algos. Automated trading systems, bots, or algorithmic execution tools are not permitted."
vps_allowed: unknown                # VPS not mentioned; VPN allowed after registration (§7.1)
personal_device_required: unknown   # not mentioned (§7.1)
status: partial
not_confirmed_fields: [consistency_rule.boundary, trading_day_reset.tz_label]
sources:
  - { field: profit_target, url: "https://takeprofittraderhelp.zendesk.com/hc/en-us/articles/15169070804125-Rule-1-Hit-Your-Profit-Target", accessed: "2026-10-23" }
  - { field: max_position, url: "https://takeprofittraderhelp.zendesk.com/hc/en-us/articles/15169066911133-Rule-2-Do-Not-Exceed-Maximum-Position-Size", accessed: "2026-10-23" }
  - { field: drawdown, url: "https://takeprofittraderhelp.zendesk.com/hc/en-us/articles/15170265979165-Rule-3-Do-Not-Hit-End-Of-Day-EOD-Maximum-Trailing-Drawdown", accessed: "2026-10-23" }
  - { field: daily_loss_limit, url: "https://takeprofittrader.com/", accessed: "2026-10-23" }
  - { field: consistency_rule, url: "https://takeprofittraderhelp.zendesk.com/hc/en-us/articles/15170316538013-Rule-5-Be-Consistent", accessed: "2026-10-23" }
  - { field: min_trading_days, url: "https://takeprofittraderhelp.zendesk.com/hc/en-us/articles/15170316538013-Rule-5-Be-Consistent", accessed: "2026-10-23" }
  - { field: min_trading_days, url: "https://takeprofittraderhelp.zendesk.com/hc/en-us/articles/38320927906717-50AND3-PROMO-FAQS", accessed: "2026-10-23" }
  - { field: trading_day_reset, url: "https://takeprofittraderhelp.zendesk.com/hc/en-us/articles/15170347090461-Rule-4-Trade-Approved-Products-During-Approved-Hours", accessed: "2026-10-23" }
  - { field: platforms, url: "https://takeprofittraderhelp.zendesk.com/hc/en-us/articles/15173017163549-Choosing-Your-Platform", accessed: "2026-10-23" }
  - { field: copy_trading, url: "https://takeprofittraderhelp.zendesk.com/hc/en-us/articles/34431176505245-Trade-Copier-Policy", accessed: "2026-10-23" }
  - { field: automation, url: "https://takeprofittraderhelp.zendesk.com/hc/en-us/articles/34431153546397-TakeProfitTrader-Universal-Trading-Policies-UTP", accessed: "2026-10-23" }
  - { field: vps_allowed, url: "https://takeprofittraderhelp.zendesk.com/hc/en-us/articles/37085469996573-Registration-Denial-FAQ", accessed: "2026-10-23" }
  - { field: vps_allowed, url: "https://takeprofittraderhelp.zendesk.com/hc/en-us/articles/22447557656477-Restricted-Countries", accessed: "2026-10-23" }
```


### 8.2 PRO (funded, simulated)


```yaml
firm: take-profit-trader
plan: "PRO"
phase: funded
size_label: "25K"
size_usd: "25000.00"
# drawdown amount: 'Maximum drawdown in PRO = maximum drawdown from your passed test.' (PRO Account Rules)
instruments: [MNQ, NQ]
platforms: [CQG, Rithmic, TradingView, NinjaTrader, Tradovate, Bookmap, MultiCharts, MotiveWave, Trade Navigator, "R|Trader", VolFix, ATAS, "Investor R/T", Quantower, Finamark, Jigsaw Trader]
drawdown: { amount_usd: "1500.00", type: trailing_intraday, stops_trailing_at: "stops trailing when it reaches the starting balance; it no longer moves" }
daily_loss_limit: { amount_usd: null, kind: none }
profit_target_usd: null
consistency_rule: { text: "none (homepage table: Consistency rule — PRO: None)", threshold_pct: null }
max_position: { minis: 3, micros: 30, scaling: "none; same buying power as the test account" }
min_trading_days: null
trading_day_reset: { time: "17:00", tz_as_published: "Eastern (Rule 4 article: '5 PM Eastern'); homepage tooltip: '5PM EST'" }
copy_trading_allowed: conditional   # own accounts only, approved copiers only; see §5
automated_trading_allowed: no
automation_rule_text: "#1: No Trading Bots or Algos. Automated trading systems, bots, or algorithmic execution tools are not permitted."
vps_allowed: unknown                # VPS not mentioned; VPN allowed after registration (§7.1)
personal_device_required: unknown   # not mentioned (§7.1)
status: partial
not_confirmed_fields: [trading_day_reset.tz_label]
sources:
  - { field: drawdown, url: "https://takeprofittraderhelp.zendesk.com/hc/en-us/articles/15171769361053-PRO-Account-Rules", accessed: "2026-10-23" }
  - { field: drawdown_amount, url: "https://takeprofittraderhelp.zendesk.com/hc/en-us/articles/15170265979165-Rule-3-Do-Not-Hit-End-Of-Day-EOD-Maximum-Trailing-Drawdown", accessed: "2026-10-23" }
  - { field: daily_loss_limit, url: "https://takeprofittrader.com/", accessed: "2026-10-23" }
  - { field: consistency_rule, url: "https://takeprofittrader.com/", accessed: "2026-10-23" }
  - { field: max_position, url: "https://takeprofittraderhelp.zendesk.com/hc/en-us/articles/15169066911133-Rule-2-Do-Not-Exceed-Maximum-Position-Size", accessed: "2026-10-23" }
  - { field: max_position, url: "https://takeprofittrader.com/", accessed: "2026-10-23" }
  - { field: trading_day_reset, url: "https://takeprofittraderhelp.zendesk.com/hc/en-us/articles/15170347090461-Rule-4-Trade-Approved-Products-During-Approved-Hours", accessed: "2026-10-23" }
  - { field: platforms, url: "https://takeprofittraderhelp.zendesk.com/hc/en-us/articles/15173017163549-Choosing-Your-Platform", accessed: "2026-10-23" }
  - { field: copy_trading, url: "https://takeprofittraderhelp.zendesk.com/hc/en-us/articles/34431176505245-Trade-Copier-Policy", accessed: "2026-10-23" }
  - { field: automation, url: "https://takeprofittraderhelp.zendesk.com/hc/en-us/articles/34431153546397-TakeProfitTrader-Universal-Trading-Policies-UTP", accessed: "2026-10-23" }
  - { field: vps_allowed, url: "https://takeprofittraderhelp.zendesk.com/hc/en-us/articles/37085469996573-Registration-Denial-FAQ", accessed: "2026-10-23" }
  - { field: vps_allowed, url: "https://takeprofittraderhelp.zendesk.com/hc/en-us/articles/22447557656477-Restricted-Countries", accessed: "2026-10-23" }
  - { field: automation, url: "https://takeprofittraderhelp.zendesk.com/hc/en-us/articles/15171769361053-PRO-Account-Rules", accessed: "2026-10-23" }
```

```yaml
firm: take-profit-trader
plan: "PRO"
phase: funded
size_label: "50K"
size_usd: "50000.00"
# drawdown amount: 'Maximum drawdown in PRO = maximum drawdown from your passed test.' (PRO Account Rules)
instruments: [MNQ, NQ]
platforms: [CQG, Rithmic, TradingView, NinjaTrader, Tradovate, Bookmap, MultiCharts, MotiveWave, Trade Navigator, "R|Trader", VolFix, ATAS, "Investor R/T", Quantower, Finamark, Jigsaw Trader]
drawdown: { amount_usd: "2000.00", type: trailing_intraday, stops_trailing_at: "stops trailing when it reaches the starting balance; it no longer moves" }
daily_loss_limit: { amount_usd: null, kind: none }
profit_target_usd: null
consistency_rule: { text: "none (homepage table: Consistency rule — PRO: None)", threshold_pct: null }
max_position: { minis: 6, micros: 60, scaling: "none; same buying power as the test account" }
min_trading_days: null
trading_day_reset: { time: "17:00", tz_as_published: "Eastern (Rule 4 article: '5 PM Eastern'); homepage tooltip: '5PM EST'" }
copy_trading_allowed: conditional   # own accounts only, approved copiers only; see §5
automated_trading_allowed: no
automation_rule_text: "#1: No Trading Bots or Algos. Automated trading systems, bots, or algorithmic execution tools are not permitted."
vps_allowed: unknown                # VPS not mentioned; VPN allowed after registration (§7.1)
personal_device_required: unknown   # not mentioned (§7.1)
status: partial
not_confirmed_fields: [trading_day_reset.tz_label]
sources:
  - { field: drawdown, url: "https://takeprofittraderhelp.zendesk.com/hc/en-us/articles/15171769361053-PRO-Account-Rules", accessed: "2026-10-23" }
  - { field: drawdown_amount, url: "https://takeprofittraderhelp.zendesk.com/hc/en-us/articles/15170265979165-Rule-3-Do-Not-Hit-End-Of-Day-EOD-Maximum-Trailing-Drawdown", accessed: "2026-10-23" }
  - { field: daily_loss_limit, url: "https://takeprofittrader.com/", accessed: "2026-10-23" }
  - { field: consistency_rule, url: "https://takeprofittrader.com/", accessed: "2026-10-23" }
  - { field: max_position, url: "https://takeprofittraderhelp.zendesk.com/hc/en-us/articles/15169066911133-Rule-2-Do-Not-Exceed-Maximum-Position-Size", accessed: "2026-10-23" }
  - { field: max_position, url: "https://takeprofittrader.com/", accessed: "2026-10-23" }
  - { field: trading_day_reset, url: "https://takeprofittraderhelp.zendesk.com/hc/en-us/articles/15170347090461-Rule-4-Trade-Approved-Products-During-Approved-Hours", accessed: "2026-10-23" }
  - { field: platforms, url: "https://takeprofittraderhelp.zendesk.com/hc/en-us/articles/15173017163549-Choosing-Your-Platform", accessed: "2026-10-23" }
  - { field: copy_trading, url: "https://takeprofittraderhelp.zendesk.com/hc/en-us/articles/34431176505245-Trade-Copier-Policy", accessed: "2026-10-23" }
  - { field: automation, url: "https://takeprofittraderhelp.zendesk.com/hc/en-us/articles/34431153546397-TakeProfitTrader-Universal-Trading-Policies-UTP", accessed: "2026-10-23" }
  - { field: vps_allowed, url: "https://takeprofittraderhelp.zendesk.com/hc/en-us/articles/37085469996573-Registration-Denial-FAQ", accessed: "2026-10-23" }
  - { field: vps_allowed, url: "https://takeprofittraderhelp.zendesk.com/hc/en-us/articles/22447557656477-Restricted-Countries", accessed: "2026-10-23" }
  - { field: automation, url: "https://takeprofittraderhelp.zendesk.com/hc/en-us/articles/15171769361053-PRO-Account-Rules", accessed: "2026-10-23" }
```

```yaml
firm: take-profit-trader
plan: "PRO"
phase: funded
size_label: "75K"
size_usd: "75000.00"
# drawdown amount: 'Maximum drawdown in PRO = maximum drawdown from your passed test.' (PRO Account Rules)
instruments: [MNQ, NQ]
platforms: [CQG, Rithmic, TradingView, NinjaTrader, Tradovate, Bookmap, MultiCharts, MotiveWave, Trade Navigator, "R|Trader", VolFix, ATAS, "Investor R/T", Quantower, Finamark, Jigsaw Trader]
drawdown: { amount_usd: "2500.00", type: trailing_intraday, stops_trailing_at: "stops trailing when it reaches the starting balance; it no longer moves" }
daily_loss_limit: { amount_usd: null, kind: none }
profit_target_usd: null
consistency_rule: { text: "none (homepage table: Consistency rule — PRO: None)", threshold_pct: null }
max_position: { minis: 9, micros: 90, scaling: "none; same buying power as the test account" }
min_trading_days: null
trading_day_reset: { time: "17:00", tz_as_published: "Eastern (Rule 4 article: '5 PM Eastern'); homepage tooltip: '5PM EST'" }
copy_trading_allowed: conditional   # own accounts only, approved copiers only; see §5
automated_trading_allowed: no
automation_rule_text: "#1: No Trading Bots or Algos. Automated trading systems, bots, or algorithmic execution tools are not permitted."
vps_allowed: unknown                # VPS not mentioned; VPN allowed after registration (§7.1)
personal_device_required: unknown   # not mentioned (§7.1)
status: partial
not_confirmed_fields: [trading_day_reset.tz_label]
sources:
  - { field: drawdown, url: "https://takeprofittraderhelp.zendesk.com/hc/en-us/articles/15171769361053-PRO-Account-Rules", accessed: "2026-10-23" }
  - { field: drawdown_amount, url: "https://takeprofittraderhelp.zendesk.com/hc/en-us/articles/15170265979165-Rule-3-Do-Not-Hit-End-Of-Day-EOD-Maximum-Trailing-Drawdown", accessed: "2026-10-23" }
  - { field: daily_loss_limit, url: "https://takeprofittrader.com/", accessed: "2026-10-23" }
  - { field: consistency_rule, url: "https://takeprofittrader.com/", accessed: "2026-10-23" }
  - { field: max_position, url: "https://takeprofittraderhelp.zendesk.com/hc/en-us/articles/15169066911133-Rule-2-Do-Not-Exceed-Maximum-Position-Size", accessed: "2026-10-23" }
  - { field: max_position, url: "https://takeprofittrader.com/", accessed: "2026-10-23" }
  - { field: trading_day_reset, url: "https://takeprofittraderhelp.zendesk.com/hc/en-us/articles/15170347090461-Rule-4-Trade-Approved-Products-During-Approved-Hours", accessed: "2026-10-23" }
  - { field: platforms, url: "https://takeprofittraderhelp.zendesk.com/hc/en-us/articles/15173017163549-Choosing-Your-Platform", accessed: "2026-10-23" }
  - { field: copy_trading, url: "https://takeprofittraderhelp.zendesk.com/hc/en-us/articles/34431176505245-Trade-Copier-Policy", accessed: "2026-10-23" }
  - { field: automation, url: "https://takeprofittraderhelp.zendesk.com/hc/en-us/articles/34431153546397-TakeProfitTrader-Universal-Trading-Policies-UTP", accessed: "2026-10-23" }
  - { field: vps_allowed, url: "https://takeprofittraderhelp.zendesk.com/hc/en-us/articles/37085469996573-Registration-Denial-FAQ", accessed: "2026-10-23" }
  - { field: vps_allowed, url: "https://takeprofittraderhelp.zendesk.com/hc/en-us/articles/22447557656477-Restricted-Countries", accessed: "2026-10-23" }
  - { field: automation, url: "https://takeprofittraderhelp.zendesk.com/hc/en-us/articles/15171769361053-PRO-Account-Rules", accessed: "2026-10-23" }
```

```yaml
firm: take-profit-trader
plan: "PRO"
phase: funded
size_label: "100K"
size_usd: "100000.00"
# drawdown amount: 'Maximum drawdown in PRO = maximum drawdown from your passed test.' (PRO Account Rules)
instruments: [MNQ, NQ]
platforms: [CQG, Rithmic, TradingView, NinjaTrader, Tradovate, Bookmap, MultiCharts, MotiveWave, Trade Navigator, "R|Trader", VolFix, ATAS, "Investor R/T", Quantower, Finamark, Jigsaw Trader]
drawdown: { amount_usd: "3000.00", type: trailing_intraday, stops_trailing_at: "stops trailing when it reaches the starting balance; it no longer moves" }
daily_loss_limit: { amount_usd: null, kind: none }
profit_target_usd: null
consistency_rule: { text: "none (homepage table: Consistency rule — PRO: None)", threshold_pct: null }
max_position: { minis: 12, micros: 120, scaling: "none; same buying power as the test account" }
min_trading_days: null
trading_day_reset: { time: "17:00", tz_as_published: "Eastern (Rule 4 article: '5 PM Eastern'); homepage tooltip: '5PM EST'" }
copy_trading_allowed: conditional   # own accounts only, approved copiers only; see §5
automated_trading_allowed: no
automation_rule_text: "#1: No Trading Bots or Algos. Automated trading systems, bots, or algorithmic execution tools are not permitted."
vps_allowed: unknown                # VPS not mentioned; VPN allowed after registration (§7.1)
personal_device_required: unknown   # not mentioned (§7.1)
status: partial
not_confirmed_fields: [trading_day_reset.tz_label]
sources:
  - { field: drawdown, url: "https://takeprofittraderhelp.zendesk.com/hc/en-us/articles/15171769361053-PRO-Account-Rules", accessed: "2026-10-23" }
  - { field: drawdown_amount, url: "https://takeprofittraderhelp.zendesk.com/hc/en-us/articles/15170265979165-Rule-3-Do-Not-Hit-End-Of-Day-EOD-Maximum-Trailing-Drawdown", accessed: "2026-10-23" }
  - { field: daily_loss_limit, url: "https://takeprofittrader.com/", accessed: "2026-10-23" }
  - { field: consistency_rule, url: "https://takeprofittrader.com/", accessed: "2026-10-23" }
  - { field: max_position, url: "https://takeprofittraderhelp.zendesk.com/hc/en-us/articles/15169066911133-Rule-2-Do-Not-Exceed-Maximum-Position-Size", accessed: "2026-10-23" }
  - { field: max_position, url: "https://takeprofittrader.com/", accessed: "2026-10-23" }
  - { field: trading_day_reset, url: "https://takeprofittraderhelp.zendesk.com/hc/en-us/articles/15170347090461-Rule-4-Trade-Approved-Products-During-Approved-Hours", accessed: "2026-10-23" }
  - { field: platforms, url: "https://takeprofittraderhelp.zendesk.com/hc/en-us/articles/15173017163549-Choosing-Your-Platform", accessed: "2026-10-23" }
  - { field: copy_trading, url: "https://takeprofittraderhelp.zendesk.com/hc/en-us/articles/34431176505245-Trade-Copier-Policy", accessed: "2026-10-23" }
  - { field: automation, url: "https://takeprofittraderhelp.zendesk.com/hc/en-us/articles/34431153546397-TakeProfitTrader-Universal-Trading-Policies-UTP", accessed: "2026-10-23" }
  - { field: vps_allowed, url: "https://takeprofittraderhelp.zendesk.com/hc/en-us/articles/37085469996573-Registration-Denial-FAQ", accessed: "2026-10-23" }
  - { field: vps_allowed, url: "https://takeprofittraderhelp.zendesk.com/hc/en-us/articles/22447557656477-Restricted-Countries", accessed: "2026-10-23" }
  - { field: automation, url: "https://takeprofittraderhelp.zendesk.com/hc/en-us/articles/15171769361053-PRO-Account-Rules", accessed: "2026-10-23" }
```

```yaml
firm: take-profit-trader
plan: "PRO"
phase: funded
size_label: "150K"
size_usd: "150000.00"
# drawdown amount: 'Maximum drawdown in PRO = maximum drawdown from your passed test.' (PRO Account Rules)
instruments: [MNQ, NQ]
platforms: [CQG, Rithmic, TradingView, NinjaTrader, Tradovate, Bookmap, MultiCharts, MotiveWave, Trade Navigator, "R|Trader", VolFix, ATAS, "Investor R/T", Quantower, Finamark, Jigsaw Trader]
drawdown: { amount_usd: "4500.00", type: trailing_intraday, stops_trailing_at: "stops trailing when it reaches the starting balance; it no longer moves" }
daily_loss_limit: { amount_usd: null, kind: none }
profit_target_usd: null
consistency_rule: { text: "none (homepage table: Consistency rule — PRO: None)", threshold_pct: null }
max_position: { minis: 15, micros: 150, scaling: "none; same buying power as the test account" }
min_trading_days: null
trading_day_reset: { time: "17:00", tz_as_published: "Eastern (Rule 4 article: '5 PM Eastern'); homepage tooltip: '5PM EST'" }
copy_trading_allowed: conditional   # own accounts only, approved copiers only; see §5
automated_trading_allowed: no
automation_rule_text: "#1: No Trading Bots or Algos. Automated trading systems, bots, or algorithmic execution tools are not permitted."
vps_allowed: unknown                # VPS not mentioned; VPN allowed after registration (§7.1)
personal_device_required: unknown   # not mentioned (§7.1)
status: partial
not_confirmed_fields: [trading_day_reset.tz_label]
sources:
  - { field: drawdown, url: "https://takeprofittraderhelp.zendesk.com/hc/en-us/articles/15171769361053-PRO-Account-Rules", accessed: "2026-10-23" }
  - { field: drawdown_amount, url: "https://takeprofittraderhelp.zendesk.com/hc/en-us/articles/15170265979165-Rule-3-Do-Not-Hit-End-Of-Day-EOD-Maximum-Trailing-Drawdown", accessed: "2026-10-23" }
  - { field: daily_loss_limit, url: "https://takeprofittrader.com/", accessed: "2026-10-23" }
  - { field: consistency_rule, url: "https://takeprofittrader.com/", accessed: "2026-10-23" }
  - { field: max_position, url: "https://takeprofittraderhelp.zendesk.com/hc/en-us/articles/15169066911133-Rule-2-Do-Not-Exceed-Maximum-Position-Size", accessed: "2026-10-23" }
  - { field: max_position, url: "https://takeprofittrader.com/", accessed: "2026-10-23" }
  - { field: trading_day_reset, url: "https://takeprofittraderhelp.zendesk.com/hc/en-us/articles/15170347090461-Rule-4-Trade-Approved-Products-During-Approved-Hours", accessed: "2026-10-23" }
  - { field: platforms, url: "https://takeprofittraderhelp.zendesk.com/hc/en-us/articles/15173017163549-Choosing-Your-Platform", accessed: "2026-10-23" }
  - { field: copy_trading, url: "https://takeprofittraderhelp.zendesk.com/hc/en-us/articles/34431176505245-Trade-Copier-Policy", accessed: "2026-10-23" }
  - { field: automation, url: "https://takeprofittraderhelp.zendesk.com/hc/en-us/articles/34431153546397-TakeProfitTrader-Universal-Trading-Policies-UTP", accessed: "2026-10-23" }
  - { field: vps_allowed, url: "https://takeprofittraderhelp.zendesk.com/hc/en-us/articles/37085469996573-Registration-Denial-FAQ", accessed: "2026-10-23" }
  - { field: vps_allowed, url: "https://takeprofittraderhelp.zendesk.com/hc/en-us/articles/22447557656477-Restricted-Countries", accessed: "2026-10-23" }
  - { field: automation, url: "https://takeprofittraderhelp.zendesk.com/hc/en-us/articles/15171769361053-PRO-Account-Rules", accessed: "2026-10-23" }
```


### 8.3 PRO+ (live, standard)


```yaml
firm: take-profit-trader
plan: "PRO+"
phase: live
size_label: "25K"
size_usd: "25000.00"
# invitation-only upgrade from a PRO 25K; size_usd is the originating PRO size, the PRO+ account itself starts at a $0 balance (Upgrade article)
instruments: [MNQ, NQ]
platforms: []   # not published per plan; see §2
drawdown: { amount_usd: "1500.00", type: trailing_eod, stops_trailing_at: "PRO+ starts at a $0 balance; the minimum balance trails EOD realized profit and stops once it reaches $0" }
daily_loss_limit: { amount_usd: null, kind: none }
profit_target_usd: null
consistency_rule: { text: "none (homepage table: Consistency rule — PRO+: None)", threshold_pct: null }
max_position: { minis: null, micros: null, scaling: "none (homepage table: Scaling plan — PRO+: None)" }
min_trading_days: null
trading_day_reset: { time: "17:00", tz_as_published: "Eastern (Rule 4 article: '5 PM Eastern'); homepage tooltip: '5PM EST'" }
copy_trading_allowed: conditional   # own accounts only, approved copiers only; see §5
automated_trading_allowed: no
automation_rule_text: "#1: No Trading Bots or Algos. Automated trading systems, bots, or algorithmic execution tools are not permitted."
vps_allowed: unknown                # VPS not mentioned; VPN allowed after registration (§7.1)
personal_device_required: unknown   # not mentioned (§7.1)
status: partial
excluded: true
excluded_reason: "User decision 2026-10-23 (S-4): not offered as V1 preset"
not_confirmed_fields: [platforms, max_position, trading_day_reset.tz_label]
sources:
  - { field: drawdown, url: "https://takeprofittraderhelp.zendesk.com/hc/en-us/articles/15171978600349-PRO-Account-Upgrade-Process-Overview-and-Guidelines", accessed: "2026-10-23" }
  - { field: drawdown, url: "https://takeprofittraderhelp.zendesk.com/hc/en-us/articles/15172006753821-PRO-Account-Rules", accessed: "2026-10-23" }
  - { field: daily_loss_limit, url: "https://takeprofittrader.com/", accessed: "2026-10-23" }
  - { field: consistency_rule, url: "https://takeprofittrader.com/", accessed: "2026-10-23" }
  - { field: trading_day_reset, url: "https://takeprofittraderhelp.zendesk.com/hc/en-us/articles/15170347090461-Rule-4-Trade-Approved-Products-During-Approved-Hours", accessed: "2026-10-23" }
  - { field: copy_trading, url: "https://takeprofittraderhelp.zendesk.com/hc/en-us/articles/34431176505245-Trade-Copier-Policy", accessed: "2026-10-23" }
  - { field: automation, url: "https://takeprofittraderhelp.zendesk.com/hc/en-us/articles/34431153546397-TakeProfitTrader-Universal-Trading-Policies-UTP", accessed: "2026-10-23" }
  - { field: vps_allowed, url: "https://takeprofittraderhelp.zendesk.com/hc/en-us/articles/37085469996573-Registration-Denial-FAQ", accessed: "2026-10-23" }
  - { field: vps_allowed, url: "https://takeprofittraderhelp.zendesk.com/hc/en-us/articles/22447557656477-Restricted-Countries", accessed: "2026-10-23" }
```

```yaml
firm: take-profit-trader
plan: "PRO+"
phase: live
size_label: "50K"
size_usd: "50000.00"
# invitation-only upgrade from a PRO 50K; size_usd is the originating PRO size, the PRO+ account itself starts at a $0 balance (Upgrade article)
instruments: [MNQ, NQ]
platforms: []   # not published per plan; see §2
drawdown: { amount_usd: "2000.00", type: trailing_eod, stops_trailing_at: "PRO+ starts at a $0 balance; the minimum balance trails EOD realized profit and stops once it reaches $0" }
daily_loss_limit: { amount_usd: null, kind: none }
profit_target_usd: null
consistency_rule: { text: "none (homepage table: Consistency rule — PRO+: None)", threshold_pct: null }
max_position: { minis: null, micros: null, scaling: "none (homepage table: Scaling plan — PRO+: None)" }
min_trading_days: null
trading_day_reset: { time: "17:00", tz_as_published: "Eastern (Rule 4 article: '5 PM Eastern'); homepage tooltip: '5PM EST'" }
copy_trading_allowed: conditional   # own accounts only, approved copiers only; see §5
automated_trading_allowed: no
automation_rule_text: "#1: No Trading Bots or Algos. Automated trading systems, bots, or algorithmic execution tools are not permitted."
vps_allowed: unknown                # VPS not mentioned; VPN allowed after registration (§7.1)
personal_device_required: unknown   # not mentioned (§7.1)
status: partial
excluded: true
excluded_reason: "User decision 2026-10-23 (S-4): not offered as V1 preset"
not_confirmed_fields: [platforms, max_position, trading_day_reset.tz_label]
sources:
  - { field: drawdown, url: "https://takeprofittraderhelp.zendesk.com/hc/en-us/articles/15171978600349-PRO-Account-Upgrade-Process-Overview-and-Guidelines", accessed: "2026-10-23" }
  - { field: drawdown, url: "https://takeprofittraderhelp.zendesk.com/hc/en-us/articles/15172006753821-PRO-Account-Rules", accessed: "2026-10-23" }
  - { field: daily_loss_limit, url: "https://takeprofittrader.com/", accessed: "2026-10-23" }
  - { field: consistency_rule, url: "https://takeprofittrader.com/", accessed: "2026-10-23" }
  - { field: trading_day_reset, url: "https://takeprofittraderhelp.zendesk.com/hc/en-us/articles/15170347090461-Rule-4-Trade-Approved-Products-During-Approved-Hours", accessed: "2026-10-23" }
  - { field: copy_trading, url: "https://takeprofittraderhelp.zendesk.com/hc/en-us/articles/34431176505245-Trade-Copier-Policy", accessed: "2026-10-23" }
  - { field: automation, url: "https://takeprofittraderhelp.zendesk.com/hc/en-us/articles/34431153546397-TakeProfitTrader-Universal-Trading-Policies-UTP", accessed: "2026-10-23" }
  - { field: vps_allowed, url: "https://takeprofittraderhelp.zendesk.com/hc/en-us/articles/37085469996573-Registration-Denial-FAQ", accessed: "2026-10-23" }
  - { field: vps_allowed, url: "https://takeprofittraderhelp.zendesk.com/hc/en-us/articles/22447557656477-Restricted-Countries", accessed: "2026-10-23" }
```

```yaml
firm: take-profit-trader
plan: "PRO+"
phase: live
size_label: "75K"
size_usd: "75000.00"
# invitation-only upgrade from a PRO 75K; size_usd is the originating PRO size, the PRO+ account itself starts at a $0 balance (Upgrade article)
instruments: [MNQ, NQ]
platforms: []   # not published per plan; see §2
drawdown: { amount_usd: "2500.00", type: trailing_eod, stops_trailing_at: "PRO+ starts at a $0 balance; the minimum balance trails EOD realized profit and stops once it reaches $0" }
daily_loss_limit: { amount_usd: null, kind: none }
profit_target_usd: null
consistency_rule: { text: "none (homepage table: Consistency rule — PRO+: None)", threshold_pct: null }
max_position: { minis: null, micros: null, scaling: "none (homepage table: Scaling plan — PRO+: None)" }
min_trading_days: null
trading_day_reset: { time: "17:00", tz_as_published: "Eastern (Rule 4 article: '5 PM Eastern'); homepage tooltip: '5PM EST'" }
copy_trading_allowed: conditional   # own accounts only, approved copiers only; see §5
automated_trading_allowed: no
automation_rule_text: "#1: No Trading Bots or Algos. Automated trading systems, bots, or algorithmic execution tools are not permitted."
vps_allowed: unknown                # VPS not mentioned; VPN allowed after registration (§7.1)
personal_device_required: unknown   # not mentioned (§7.1)
status: partial
excluded: true
excluded_reason: "User decision 2026-10-23 (S-4): not offered as V1 preset"
not_confirmed_fields: [platforms, max_position, trading_day_reset.tz_label]
sources:
  - { field: drawdown, url: "https://takeprofittraderhelp.zendesk.com/hc/en-us/articles/15171978600349-PRO-Account-Upgrade-Process-Overview-and-Guidelines", accessed: "2026-10-23" }
  - { field: drawdown, url: "https://takeprofittraderhelp.zendesk.com/hc/en-us/articles/15172006753821-PRO-Account-Rules", accessed: "2026-10-23" }
  - { field: daily_loss_limit, url: "https://takeprofittrader.com/", accessed: "2026-10-23" }
  - { field: consistency_rule, url: "https://takeprofittrader.com/", accessed: "2026-10-23" }
  - { field: trading_day_reset, url: "https://takeprofittraderhelp.zendesk.com/hc/en-us/articles/15170347090461-Rule-4-Trade-Approved-Products-During-Approved-Hours", accessed: "2026-10-23" }
  - { field: copy_trading, url: "https://takeprofittraderhelp.zendesk.com/hc/en-us/articles/34431176505245-Trade-Copier-Policy", accessed: "2026-10-23" }
  - { field: automation, url: "https://takeprofittraderhelp.zendesk.com/hc/en-us/articles/34431153546397-TakeProfitTrader-Universal-Trading-Policies-UTP", accessed: "2026-10-23" }
  - { field: vps_allowed, url: "https://takeprofittraderhelp.zendesk.com/hc/en-us/articles/37085469996573-Registration-Denial-FAQ", accessed: "2026-10-23" }
  - { field: vps_allowed, url: "https://takeprofittraderhelp.zendesk.com/hc/en-us/articles/22447557656477-Restricted-Countries", accessed: "2026-10-23" }
```

```yaml
firm: take-profit-trader
plan: "PRO+"
phase: live
size_label: "100K"
size_usd: "100000.00"
# invitation-only upgrade from a PRO 100K; size_usd is the originating PRO size, the PRO+ account itself starts at a $0 balance (Upgrade article)
instruments: [MNQ, NQ]
platforms: []   # not published per plan; see §2
drawdown: { amount_usd: "3000.00", type: trailing_eod, stops_trailing_at: "PRO+ starts at a $0 balance; the minimum balance trails EOD realized profit and stops once it reaches $0" }
daily_loss_limit: { amount_usd: null, kind: none }
profit_target_usd: null
consistency_rule: { text: "none (homepage table: Consistency rule — PRO+: None)", threshold_pct: null }
max_position: { minis: null, micros: null, scaling: "none (homepage table: Scaling plan — PRO+: None)" }
min_trading_days: null
trading_day_reset: { time: "17:00", tz_as_published: "Eastern (Rule 4 article: '5 PM Eastern'); homepage tooltip: '5PM EST'" }
copy_trading_allowed: conditional   # own accounts only, approved copiers only; see §5
automated_trading_allowed: no
automation_rule_text: "#1: No Trading Bots or Algos. Automated trading systems, bots, or algorithmic execution tools are not permitted."
vps_allowed: unknown                # VPS not mentioned; VPN allowed after registration (§7.1)
personal_device_required: unknown   # not mentioned (§7.1)
status: partial
excluded: true
excluded_reason: "User decision 2026-10-23 (S-4): not offered as V1 preset"
not_confirmed_fields: [platforms, max_position, trading_day_reset.tz_label]
sources:
  - { field: drawdown, url: "https://takeprofittraderhelp.zendesk.com/hc/en-us/articles/15171978600349-PRO-Account-Upgrade-Process-Overview-and-Guidelines", accessed: "2026-10-23" }
  - { field: drawdown, url: "https://takeprofittraderhelp.zendesk.com/hc/en-us/articles/15172006753821-PRO-Account-Rules", accessed: "2026-10-23" }
  - { field: daily_loss_limit, url: "https://takeprofittrader.com/", accessed: "2026-10-23" }
  - { field: consistency_rule, url: "https://takeprofittrader.com/", accessed: "2026-10-23" }
  - { field: trading_day_reset, url: "https://takeprofittraderhelp.zendesk.com/hc/en-us/articles/15170347090461-Rule-4-Trade-Approved-Products-During-Approved-Hours", accessed: "2026-10-23" }
  - { field: copy_trading, url: "https://takeprofittraderhelp.zendesk.com/hc/en-us/articles/34431176505245-Trade-Copier-Policy", accessed: "2026-10-23" }
  - { field: automation, url: "https://takeprofittraderhelp.zendesk.com/hc/en-us/articles/34431153546397-TakeProfitTrader-Universal-Trading-Policies-UTP", accessed: "2026-10-23" }
  - { field: vps_allowed, url: "https://takeprofittraderhelp.zendesk.com/hc/en-us/articles/37085469996573-Registration-Denial-FAQ", accessed: "2026-10-23" }
  - { field: vps_allowed, url: "https://takeprofittraderhelp.zendesk.com/hc/en-us/articles/22447557656477-Restricted-Countries", accessed: "2026-10-23" }
```

```yaml
firm: take-profit-trader
plan: "PRO+"
phase: live
size_label: "150K"
size_usd: "150000.00"
# invitation-only upgrade from a PRO 150K; size_usd is the originating PRO size, the PRO+ account itself starts at a $0 balance (Upgrade article)
instruments: [MNQ, NQ]
platforms: []   # not published per plan; see §2
drawdown: { amount_usd: "4500.00", type: trailing_eod, stops_trailing_at: "PRO+ starts at a $0 balance; the minimum balance trails EOD realized profit and stops once it reaches $0" }
daily_loss_limit: { amount_usd: null, kind: none }
profit_target_usd: null
consistency_rule: { text: "none (homepage table: Consistency rule — PRO+: None)", threshold_pct: null }
max_position: { minis: null, micros: null, scaling: "none (homepage table: Scaling plan — PRO+: None)" }
min_trading_days: null
trading_day_reset: { time: "17:00", tz_as_published: "Eastern (Rule 4 article: '5 PM Eastern'); homepage tooltip: '5PM EST'" }
copy_trading_allowed: conditional   # own accounts only, approved copiers only; see §5
automated_trading_allowed: no
automation_rule_text: "#1: No Trading Bots or Algos. Automated trading systems, bots, or algorithmic execution tools are not permitted."
vps_allowed: unknown                # VPS not mentioned; VPN allowed after registration (§7.1)
personal_device_required: unknown   # not mentioned (§7.1)
status: partial
excluded: true
excluded_reason: "User decision 2026-10-23 (S-4): not offered as V1 preset"
not_confirmed_fields: [platforms, max_position, trading_day_reset.tz_label]
sources:
  - { field: drawdown, url: "https://takeprofittraderhelp.zendesk.com/hc/en-us/articles/15171978600349-PRO-Account-Upgrade-Process-Overview-and-Guidelines", accessed: "2026-10-23" }
  - { field: drawdown, url: "https://takeprofittraderhelp.zendesk.com/hc/en-us/articles/15172006753821-PRO-Account-Rules", accessed: "2026-10-23" }
  - { field: daily_loss_limit, url: "https://takeprofittrader.com/", accessed: "2026-10-23" }
  - { field: consistency_rule, url: "https://takeprofittrader.com/", accessed: "2026-10-23" }
  - { field: trading_day_reset, url: "https://takeprofittraderhelp.zendesk.com/hc/en-us/articles/15170347090461-Rule-4-Trade-Approved-Products-During-Approved-Hours", accessed: "2026-10-23" }
  - { field: copy_trading, url: "https://takeprofittraderhelp.zendesk.com/hc/en-us/articles/34431176505245-Trade-Copier-Policy", accessed: "2026-10-23" }
  - { field: automation, url: "https://takeprofittraderhelp.zendesk.com/hc/en-us/articles/34431153546397-TakeProfitTrader-Universal-Trading-Policies-UTP", accessed: "2026-10-23" }
  - { field: vps_allowed, url: "https://takeprofittraderhelp.zendesk.com/hc/en-us/articles/37085469996573-Registration-Denial-FAQ", accessed: "2026-10-23" }
  - { field: vps_allowed, url: "https://takeprofittraderhelp.zendesk.com/hc/en-us/articles/22447557656477-Restricted-Countries", accessed: "2026-10-23" }
```


### 8.4 PRO+ Development (live, assigned by TPT)


```yaml
firm: take-profit-trader
plan: "PRO+ Development"
phase: live
size_label: "25K"
size_usd: "25000.00"
# placement decided by TPT risk review, not purchasable; 30-day cooldown rule applies (§7)
instruments: [MNQ, NQ]
platforms: []   # not published; see §2
drawdown: { amount_usd: "750.00", type: trailing_eod, stops_trailing_at: null }   # not stated in the PRO+ Development article
daily_loss_limit: { amount_usd: "500.00", kind: soft_pause }
profit_target_usd: null
consistency_rule: { text: null, threshold_pct: null }
max_position: { minis: 1, micros: 10, scaling: "fixed table (Contract Maximum Mini/Micro)" }
min_trading_days: null
trading_day_reset: { time: "17:00", tz_as_published: "Eastern (Rule 4 article: '5 PM Eastern'); homepage tooltip: '5PM EST'" }
copy_trading_allowed: conditional   # own accounts only, approved copiers only; see §5
automated_trading_allowed: no
automation_rule_text: "#1: No Trading Bots or Algos. Automated trading systems, bots, or algorithmic execution tools are not permitted."
vps_allowed: unknown                # VPS not mentioned; VPN allowed after registration (§7.1)
personal_device_required: unknown   # not mentioned (§7.1)
status: partial
excluded: true
excluded_reason: "User decision 2026-10-23 (S-4): not offered as V1 preset"
not_confirmed_fields: [platforms, drawdown.stops_trailing_at, daily_loss_limit.pause_end, trading_day_reset.tz_label]
sources:
  - { field: max_position, url: "https://takeprofittraderhelp.zendesk.com/hc/en-us/articles/36429526878237-PRO-Development-Accounts", accessed: "2026-10-23" }
  - { field: daily_loss_limit, url: "https://takeprofittraderhelp.zendesk.com/hc/en-us/articles/36429526878237-PRO-Development-Accounts", accessed: "2026-10-23" }
  - { field: drawdown, url: "https://takeprofittraderhelp.zendesk.com/hc/en-us/articles/36429526878237-PRO-Development-Accounts", accessed: "2026-10-23" }
  - { field: trading_day_reset, url: "https://takeprofittraderhelp.zendesk.com/hc/en-us/articles/15170347090461-Rule-4-Trade-Approved-Products-During-Approved-Hours", accessed: "2026-10-23" }
  - { field: copy_trading, url: "https://takeprofittraderhelp.zendesk.com/hc/en-us/articles/34431176505245-Trade-Copier-Policy", accessed: "2026-10-23" }
  - { field: automation, url: "https://takeprofittraderhelp.zendesk.com/hc/en-us/articles/34431153546397-TakeProfitTrader-Universal-Trading-Policies-UTP", accessed: "2026-10-23" }
  - { field: vps_allowed, url: "https://takeprofittraderhelp.zendesk.com/hc/en-us/articles/37085469996573-Registration-Denial-FAQ", accessed: "2026-10-23" }
  - { field: vps_allowed, url: "https://takeprofittraderhelp.zendesk.com/hc/en-us/articles/22447557656477-Restricted-Countries", accessed: "2026-10-23" }
```

```yaml
firm: take-profit-trader
plan: "PRO+ Development"
phase: live
size_label: "50K"
size_usd: "50000.00"
# placement decided by TPT risk review, not purchasable; 30-day cooldown rule applies (§7)
instruments: [MNQ, NQ]
platforms: []   # not published; see §2
drawdown: { amount_usd: "1250.00", type: trailing_eod, stops_trailing_at: null }   # not stated in the PRO+ Development article
daily_loss_limit: { amount_usd: "1000.00", kind: soft_pause }
profit_target_usd: null
consistency_rule: { text: null, threshold_pct: null }
max_position: { minis: 2, micros: 20, scaling: "fixed table (Contract Maximum Mini/Micro)" }
min_trading_days: null
trading_day_reset: { time: "17:00", tz_as_published: "Eastern (Rule 4 article: '5 PM Eastern'); homepage tooltip: '5PM EST'" }
copy_trading_allowed: conditional   # own accounts only, approved copiers only; see §5
automated_trading_allowed: no
automation_rule_text: "#1: No Trading Bots or Algos. Automated trading systems, bots, or algorithmic execution tools are not permitted."
vps_allowed: unknown                # VPS not mentioned; VPN allowed after registration (§7.1)
personal_device_required: unknown   # not mentioned (§7.1)
status: partial
excluded: true
excluded_reason: "User decision 2026-10-23 (S-4): not offered as V1 preset"
not_confirmed_fields: [platforms, drawdown.stops_trailing_at, daily_loss_limit.pause_end, trading_day_reset.tz_label]
sources:
  - { field: max_position, url: "https://takeprofittraderhelp.zendesk.com/hc/en-us/articles/36429526878237-PRO-Development-Accounts", accessed: "2026-10-23" }
  - { field: daily_loss_limit, url: "https://takeprofittraderhelp.zendesk.com/hc/en-us/articles/36429526878237-PRO-Development-Accounts", accessed: "2026-10-23" }
  - { field: drawdown, url: "https://takeprofittraderhelp.zendesk.com/hc/en-us/articles/36429526878237-PRO-Development-Accounts", accessed: "2026-10-23" }
  - { field: trading_day_reset, url: "https://takeprofittraderhelp.zendesk.com/hc/en-us/articles/15170347090461-Rule-4-Trade-Approved-Products-During-Approved-Hours", accessed: "2026-10-23" }
  - { field: copy_trading, url: "https://takeprofittraderhelp.zendesk.com/hc/en-us/articles/34431176505245-Trade-Copier-Policy", accessed: "2026-10-23" }
  - { field: automation, url: "https://takeprofittraderhelp.zendesk.com/hc/en-us/articles/34431153546397-TakeProfitTrader-Universal-Trading-Policies-UTP", accessed: "2026-10-23" }
  - { field: vps_allowed, url: "https://takeprofittraderhelp.zendesk.com/hc/en-us/articles/37085469996573-Registration-Denial-FAQ", accessed: "2026-10-23" }
  - { field: vps_allowed, url: "https://takeprofittraderhelp.zendesk.com/hc/en-us/articles/22447557656477-Restricted-Countries", accessed: "2026-10-23" }
```

```yaml
firm: take-profit-trader
plan: "PRO+ Development"
phase: live
size_label: "75K"
size_usd: "75000.00"
# placement decided by TPT risk review, not purchasable; 30-day cooldown rule applies (§7)
instruments: [MNQ, NQ]
platforms: []   # not published; see §2
drawdown: { amount_usd: "1500.00", type: trailing_eod, stops_trailing_at: null }   # not stated in the PRO+ Development article
daily_loss_limit: { amount_usd: "1250.00", kind: soft_pause }
profit_target_usd: null
consistency_rule: { text: null, threshold_pct: null }
max_position: { minis: 3, micros: 30, scaling: "fixed table (Contract Maximum Mini/Micro)" }
min_trading_days: null
trading_day_reset: { time: "17:00", tz_as_published: "Eastern (Rule 4 article: '5 PM Eastern'); homepage tooltip: '5PM EST'" }
copy_trading_allowed: conditional   # own accounts only, approved copiers only; see §5
automated_trading_allowed: no
automation_rule_text: "#1: No Trading Bots or Algos. Automated trading systems, bots, or algorithmic execution tools are not permitted."
vps_allowed: unknown                # VPS not mentioned; VPN allowed after registration (§7.1)
personal_device_required: unknown   # not mentioned (§7.1)
status: partial
excluded: true
excluded_reason: "User decision 2026-10-23 (S-4): not offered as V1 preset"
not_confirmed_fields: [platforms, drawdown.stops_trailing_at, daily_loss_limit.pause_end, trading_day_reset.tz_label]
sources:
  - { field: max_position, url: "https://takeprofittraderhelp.zendesk.com/hc/en-us/articles/36429526878237-PRO-Development-Accounts", accessed: "2026-10-23" }
  - { field: daily_loss_limit, url: "https://takeprofittraderhelp.zendesk.com/hc/en-us/articles/36429526878237-PRO-Development-Accounts", accessed: "2026-10-23" }
  - { field: drawdown, url: "https://takeprofittraderhelp.zendesk.com/hc/en-us/articles/36429526878237-PRO-Development-Accounts", accessed: "2026-10-23" }
  - { field: trading_day_reset, url: "https://takeprofittraderhelp.zendesk.com/hc/en-us/articles/15170347090461-Rule-4-Trade-Approved-Products-During-Approved-Hours", accessed: "2026-10-23" }
  - { field: copy_trading, url: "https://takeprofittraderhelp.zendesk.com/hc/en-us/articles/34431176505245-Trade-Copier-Policy", accessed: "2026-10-23" }
  - { field: automation, url: "https://takeprofittraderhelp.zendesk.com/hc/en-us/articles/34431153546397-TakeProfitTrader-Universal-Trading-Policies-UTP", accessed: "2026-10-23" }
  - { field: vps_allowed, url: "https://takeprofittraderhelp.zendesk.com/hc/en-us/articles/37085469996573-Registration-Denial-FAQ", accessed: "2026-10-23" }
  - { field: vps_allowed, url: "https://takeprofittraderhelp.zendesk.com/hc/en-us/articles/22447557656477-Restricted-Countries", accessed: "2026-10-23" }
```

```yaml
firm: take-profit-trader
plan: "PRO+ Development"
phase: live
size_label: "100K"
size_usd: "100000.00"
# placement decided by TPT risk review, not purchasable; 30-day cooldown rule applies (§7)
instruments: [MNQ, NQ]
platforms: []   # not published; see §2
drawdown: { amount_usd: "1750.00", type: trailing_eod, stops_trailing_at: null }   # not stated in the PRO+ Development article
daily_loss_limit: { amount_usd: "1500.00", kind: soft_pause }
profit_target_usd: null
consistency_rule: { text: null, threshold_pct: null }
max_position: { minis: 4, micros: 40, scaling: "fixed table (Contract Maximum Mini/Micro)" }
min_trading_days: null
trading_day_reset: { time: "17:00", tz_as_published: "Eastern (Rule 4 article: '5 PM Eastern'); homepage tooltip: '5PM EST'" }
copy_trading_allowed: conditional   # own accounts only, approved copiers only; see §5
automated_trading_allowed: no
automation_rule_text: "#1: No Trading Bots or Algos. Automated trading systems, bots, or algorithmic execution tools are not permitted."
vps_allowed: unknown                # VPS not mentioned; VPN allowed after registration (§7.1)
personal_device_required: unknown   # not mentioned (§7.1)
status: partial
excluded: true
excluded_reason: "User decision 2026-10-23 (S-4): not offered as V1 preset"
not_confirmed_fields: [platforms, drawdown.stops_trailing_at, daily_loss_limit.pause_end, trading_day_reset.tz_label]
sources:
  - { field: max_position, url: "https://takeprofittraderhelp.zendesk.com/hc/en-us/articles/36429526878237-PRO-Development-Accounts", accessed: "2026-10-23" }
  - { field: daily_loss_limit, url: "https://takeprofittraderhelp.zendesk.com/hc/en-us/articles/36429526878237-PRO-Development-Accounts", accessed: "2026-10-23" }
  - { field: drawdown, url: "https://takeprofittraderhelp.zendesk.com/hc/en-us/articles/36429526878237-PRO-Development-Accounts", accessed: "2026-10-23" }
  - { field: trading_day_reset, url: "https://takeprofittraderhelp.zendesk.com/hc/en-us/articles/15170347090461-Rule-4-Trade-Approved-Products-During-Approved-Hours", accessed: "2026-10-23" }
  - { field: copy_trading, url: "https://takeprofittraderhelp.zendesk.com/hc/en-us/articles/34431176505245-Trade-Copier-Policy", accessed: "2026-10-23" }
  - { field: automation, url: "https://takeprofittraderhelp.zendesk.com/hc/en-us/articles/34431153546397-TakeProfitTrader-Universal-Trading-Policies-UTP", accessed: "2026-10-23" }
  - { field: vps_allowed, url: "https://takeprofittraderhelp.zendesk.com/hc/en-us/articles/37085469996573-Registration-Denial-FAQ", accessed: "2026-10-23" }
  - { field: vps_allowed, url: "https://takeprofittraderhelp.zendesk.com/hc/en-us/articles/22447557656477-Restricted-Countries", accessed: "2026-10-23" }
```

```yaml
firm: take-profit-trader
plan: "PRO+ Development"
phase: live
size_label: "150K"
size_usd: "150000.00"
# placement decided by TPT risk review, not purchasable; 30-day cooldown rule applies (§7)
instruments: [MNQ, NQ]
platforms: []   # not published; see §2
drawdown: { amount_usd: "2250.00", type: trailing_eod, stops_trailing_at: null }   # not stated in the PRO+ Development article
daily_loss_limit: { amount_usd: "2000.00", kind: soft_pause }
profit_target_usd: null
consistency_rule: { text: null, threshold_pct: null }
max_position: { minis: 5, micros: 50, scaling: "fixed table (Contract Maximum Mini/Micro)" }
min_trading_days: null
trading_day_reset: { time: "17:00", tz_as_published: "Eastern (Rule 4 article: '5 PM Eastern'); homepage tooltip: '5PM EST'" }
copy_trading_allowed: conditional   # own accounts only, approved copiers only; see §5
automated_trading_allowed: no
automation_rule_text: "#1: No Trading Bots or Algos. Automated trading systems, bots, or algorithmic execution tools are not permitted."
vps_allowed: unknown                # VPS not mentioned; VPN allowed after registration (§7.1)
personal_device_required: unknown   # not mentioned (§7.1)
status: partial
excluded: true
excluded_reason: "User decision 2026-10-23 (S-4): not offered as V1 preset"
not_confirmed_fields: [platforms, drawdown.stops_trailing_at, daily_loss_limit.pause_end, trading_day_reset.tz_label]
sources:
  - { field: max_position, url: "https://takeprofittraderhelp.zendesk.com/hc/en-us/articles/36429526878237-PRO-Development-Accounts", accessed: "2026-10-23" }
  - { field: daily_loss_limit, url: "https://takeprofittraderhelp.zendesk.com/hc/en-us/articles/36429526878237-PRO-Development-Accounts", accessed: "2026-10-23" }
  - { field: drawdown, url: "https://takeprofittraderhelp.zendesk.com/hc/en-us/articles/36429526878237-PRO-Development-Accounts", accessed: "2026-10-23" }
  - { field: trading_day_reset, url: "https://takeprofittraderhelp.zendesk.com/hc/en-us/articles/15170347090461-Rule-4-Trade-Approved-Products-During-Approved-Hours", accessed: "2026-10-23" }
  - { field: copy_trading, url: "https://takeprofittraderhelp.zendesk.com/hc/en-us/articles/34431176505245-Trade-Copier-Policy", accessed: "2026-10-23" }
  - { field: automation, url: "https://takeprofittraderhelp.zendesk.com/hc/en-us/articles/34431153546397-TakeProfitTrader-Universal-Trading-Policies-UTP", accessed: "2026-10-23" }
  - { field: vps_allowed, url: "https://takeprofittraderhelp.zendesk.com/hc/en-us/articles/37085469996573-Registration-Denial-FAQ", accessed: "2026-10-23" }
  - { field: vps_allowed, url: "https://takeprofittraderhelp.zendesk.com/hc/en-us/articles/22447557656477-Restricted-Countries", accessed: "2026-10-23" }
```


---

## 9. Unverified / gaps / conflicts (for the user report)

| # | Field | Status | Detail / where searched |
|---|---|---|---|
| 1 | Test consistency threshold boundary | `conflict` (wording) | Rule 5 says "must be below 50%". The homepage tooltip says "cannot be more than 50%". The two differ only at exactly 50%. In the YAML, `consistency_rule.boundary` is not confirmed. |
| 2 | Trading-day reset time-zone label | `conflict` (label only) | Rule 4 says "5 PM Eastern" and the homepage drawdown tooltip says "5PM EST". Both say 5 PM. In the YAML, `trading_day_reset.tz_label` is not confirmed. |
| 3 | PRO+ (standard) max position, minis/micros | `unverified` | Not stated anywhere. Searched: PRO+ Account Rules, Advantages of PRO+, Upgrade article, homepage table/tooltips (the scaling tooltip covers PRO vs Test only), all 88 help-center articles (grep "contract"), and the terms. |
| 4 | PRO+ / PRO+ Development platforms | `unverified` | No list is published. There are only indirect hints: NinjaTrader/Tradovate commission links, "professional market data", and "in PRO+ accounts, this capability may be affected". |
| 5 | PRO+ Development drawdown stop-trailing point | `unverified` | The article gives only "End of Day Drawdown" amounts. The general homepage tooltip ("STOP trailing once it has reached the initial account balance", for "all three account types") does not name Development. |
| 6 | PRO+ Development soft DLL pause end | `unverified` | The article says only "pauses you, doesn't fail you". It does not say until when. |
| 7 | Mini + micro combination | `unverified` | Rule 2 gives "6 Contracts / 60 Micros" and a "10x" ratio. No page says whether mixed minis and micros count against one shared limit. |
| 8 | Copying to/from accounts at other firms or personal brokerage | `not mentioned` | The Trade Copier Policy and the Independent Trade Execution Policy talk only about "accounts owned and controlled by you" and other users. |
| 9 | Is All Win an "approved" copier? | open question for the user | Only Tradesyncer, TradeCopia, Affordable Indicators, Replikanto Flowbot Compliance Edition and platform-native copiers are approved. "Use of unapproved trade copiers … or copier-based services may result in enforcement action." |
| 10 | Automation vs "third-party automation tools" | ambiguity | UTP #1 forbids "Automated trading systems, bots, or algorithmic execution tools". The copier policy and terms §VII make traders responsible for "third-party automation tools connected" to accounts, which acknowledges such tools but does not permit them. Webhooks, alerts, ATM/brackets and semi-automated trading are not mentioned. |
| 11 | Year of the 3-day minimum change | wording | The 50AND3 FAQ (updated 2026-08-25) says "purchased from August 17th onward" with no year. Tests bought before then keep the 5-day minimum. |
| 12 | PRO+ drawdown amounts per size | partial | Only the 50K example ("-$2,000") is published as a PRO+ number. The other sizes follow the published rule "equivalent to the starting drawdown of the original PRO account". |
| 13 | Max-accounts tooltip wording | wording | The homepage tooltip example "or 5 PRO / 5 PRO+" reads like 10 accounts. The help center and terms say "5 active funded accounts per person" / "maximum of 5 active PRO/PRO+ accounts simultaneously". This file uses 5. |
| 14 | VPS / server hosting; personal-device requirement | `not mentioned` | Searched all 88 help-center articles, the terms and the homepage text. Only VPN rules exist: VPNs are blocked during registration and are not a "general prohibition" afterwards, and access must not be routed through restricted countries (§7.1). YAML: `vps_allowed: unknown`, `personal_device_required: unknown`. |

**Pages not reached:** none blocked. `takeprofittrader.com/terms` and other subpages return 403 to curl, so they were read through the portal. The trader calendar `https://takeprofittrader.com/user-info/calendar`, cited for news events, was not opened because it is a logged-in user area.

## 10. Changelog

- 2026-10-23 — initial research (Lupa).
- 2026-10-23 — format rev. 3: added §7.1 Device / hosting rules, `vps_allowed` / `personal_device_required` in all 20 YAML blocks, gap #14 (Lupa).
- 2026-10-23 — PRO+ and PRO+ Development marked `excluded: true` in their 10 YAML blocks, per user decision S-4 (relayed by the PO); values and status unchanged, not re-validated (Lupa).
- 2026-10-23 — rev bump R4: personal_device_required re-checked on all help-center articles and the Terms; no explicit device requirement or permission, stays `unknown`; `vps_allowed` not in scope, unchanged (Lupa)
