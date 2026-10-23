# TradeDay — Preset Research

| | |
|---|---|
| **Firm** | TradeDay (TradeDay LLC, 2nd Floor, 412 S. Wells, Chicago, Illinois 60607) |
| **Official domains** | `tradeday.com` (site, pricing, terms). The help center is `tradeday.freshdesk.com/en/support/solutions`, linked as "FAQs" from the tradeday.com header and footer. `members.tradeday.com` is the login and sign-up area and was **not** used. |
| **Research date** | 2026-10-23 (every URL below accessed 2026-10-23) |
| **Researcher** | Lupa (Prop Firm Rules Researcher) |
| **Format** | preset-research-format.md rev. 2 + PO addition rev. 3 (device and hosting rules) |
| **File status** | `partial`. Evaluation and Funded Sim numbers are sourced and agree across pages. Several official pages conflict: Fast Pass minimum days, Funded Live drawdown and position limits, and slippage buffer basis. Some fields are not published (§9). |
| **Instruments covered** | MNQ, NQ. Both are on the permitted products list, a table with these rows: E-mini Nasdaq-100 Futures | NQ | $5.00 | CME, and Micro E-mini Nasdaq-100 Index Futures | MNQ | $0.50 | CME ([S-PROD](#1-source-index-all-accessed-2026-10-23)). The rules below apply to every permitted CME Group product. |
| **Platforms** | Tradovate (CQG) or Rithmic for every plan and size. A trader may use only one broker. NinjaTrader Desktop works only with Tradovate/CQG accounts (§2). The rules do not differ by platform, so each plan/phase/size has one YAML block covering both. |

**Product context.** TradeDay launched "TradeDay 2.0" on May 29, 2026. It now sells two evaluation programs, each in 25K / 50K / 100K / 150K:

- **Quick Pay**, sold as **Intraday** or **End of Day** drawdown. The homepage labels the EOD version "End of Day (Intraday Funded Drawdown)".
- **Fast Pass**, sold as **End of Day** only.

Each program has three phases: Evaluation, Funded Sim, and Funded Live. No size outside 25K–150K is published (no 300K). Older programs are no longer sold:

> "Existing TradeDay traders currently participating in evaluations or funded programs will continue under their current account structures and rules. While previous programs are no longer available for new purchases, there will be no disruption or change to active trader accounts already operating within the TradeDay ecosystem." — S-BLOG

The homepage pricing widget shows the same rule card for the Tradovate and the Rithmic version of every plan and size (evidence: `home.txt`).

---

## 1. Source index (all accessed 2026-10-23)

Help-center base: `https://tradeday.freshdesk.com/en/support/solutions/articles/`. The "Modified" date is the date shown on the article.

| Key | URL | Modified |
|---|---|---|
| S-HOME | https://www.tradeday.com/ (pricing widget "BUILD YOUR ACCOUNT": Eval Rules / Funded Rules card per plan, size and platform; FAQ) | — |
| S-HOW | https://www.tradeday.com/how-it-works | — |
| S-TC | https://www.tradeday.com/terms-and-conditions | Last Modified: August 11, 2026 |
| S-BLOG | https://www.tradeday.com/blog-posts/tradeday-2-0---the-next-evolution-of-prop-trading | May 29, 2026 |
| S-OBJ | …/103000008847-what-are-the-objectives-and-rules-of-the-quick-pay-and-fast-pass-evaluation- | Mon, 27 Jul |
| S-SIZES | …/103000304248-what-are-the-differences-between-each-account-size- | Tue, 1 Sep |
| S-QP | …/103000405228-what-is-quick-pay- | Wed, 9 Sep |
| S-FP | …/103000405229-what-is-fast-pass- | Wed, 9 Sep |
| S-MDD | …/103000008855-what-is-the-maximum-drawdown-rule- | Sun, 31 May |
| S-EODINT | …/103000335948-definition-and-differences-between-end-of-day-trailing-max-drawdown-and-intraday-trailing-maximum-dr | Sun, 31 May |
| S-CONS | …/103000008849-what-is-the-consistency-objective- | Tue, 28 Jul |
| S-DAYS | …/103000008850-what-is-the-trade-for-a-minimum-number-of-days-objective- | Mon, 1 Jun |
| S-POS | …/103000008891-what-are-the-position-limits-guidelines-and-contracts-vs-micros- | Tue, 1 Jul, 2025 |
| S-TDAY | …/103000008864-what-is-the-definition-of-a-trading-day- | Thu, 9 Jan, 2025 |
| S-TIMES | …/103000018874-what-is-the-permitted-trading-times-guideline- | Mon, 27 Apr |
| S-NEWS | …/103000081072-can-i-trade-news-or-data-releases- | Sat, 22 Feb, 2025 |
| S-NEWS2 | …/103000063272-why-do-we-ban-tier-1-news-and-data-release-trading- | Mon, 7 Jul, 2025 |
| S-FUNDED | …/103000008889-what-are-the-rules-for-funded-funded-sim-and-funded-live-traders- | Sun, 31 May |
| S-SIMLIVE | …/103000180686-what-is-a-funded-sim-account-and-what-is-a-funded-live-account- | Sun, 31 May |
| S-QPPAY | …/103000335937-quick-pay-funded-sim-payout-policy | Wed, 22 Jul |
| S-QPLIVE | …/103000353468-quick-pay-funded-sim-path-to-live-trading | Mon, 31 Aug |
| S-FPPAY | …/103000404096-fast-pass-funded-sim-payout-policy | Tue, 8 Sep |
| S-FPLIVE | …/103000404097-fast-pass-funded-sim-route-to-funded-live | Tue, 1 Sep |
| S-LIVE | …/103000411652-quick-pay-and-fast-pass-funded-live-trading-rules | Wed, 9 Sep |
| S-WDD | …/103000250682-funded-accounts-withdrawals-and-the-impact-of-drawdowns | Sun, 31 May |
| S-SLIP | …/103000089243-slippage-rule-for-funded-live-traders | Wed, 9 Sep |
| S-COPY | …/103000008833-can-i-have-multiple-accounts-can-i-use-copy-trader- | Tue, 22 Sep |
| S-MULTI | …/103000354796-multiple-accounts | Tue, 8 Sep |
| S-AUTO | …/103000085101-automated-algo-and-bot-trading | Fri, 28 Nov, 2025 |
| S-PROHIB | …/103000121031-prohibited-trade-practices | Tue, 10 Jun, 2025 |
| S-SCALP | …/103000008883-will-tradeday-evaluate-and-fund-scalpers- | Fri, 3 Jan, 2025 |
| S-SPLIT | …/103000397952-order-splitting-and-microscalping | Thu, 3 Sep |
| S-VPS | …/103000295384-using-a-vpn-vps-or-ip-masking-add-on | Sun, 31 May |
| S-PLAT | …/103000008813-which-trading-platforms-does-tradeday-use- | Wed, 2 Sep |
| S-RISK | …/103000244846-can-i-access-risk-settings- | Mon, 9 Mar |
| S-PROD | …/103000008862-what-are-the-permitted-products-guidelines- | Tue, 6 Oct |
| S-PRICE | …/103000307192-price-limits-and-circuit-breakers | Thu, 17 Jul, 2025 |
| S-INACT | …/103000008899-are-there-any-inactivity-fees-or-minimum-trading-requirements- | Wed, 30 Sep |
| S-NET | …/103000008820-what-happens-if-i-lose-internet-connection- | Thu, 2 Jan, 2025 |

**Evidence.** The text of every page cited is saved with its URL on the first line in `/tmp/claude-1003/-srv-www-allwin--maestri-roles-6bf77a80-a1ed-43dc-bc00-08a480f178a5/33652d13-4dae-46b6-ae06-44a72b375517/scratchpad/r2/tradeday/`:

- `home.txt` and `site-*.txt` (site pages);
- `art/<id>-<slug>.txt` (all 85 public help-center articles, crawled from the 8 help-center folders);
- raw HTML under `raw/`.

All pages were fetched with `curl`; no browser portal was needed.

---

## 2. Platforms

TradeDay offers two broker/data connections. A trader picks one for all of their accounts.

> "Traders can only have multiple accounts on 1 broker. A trader cannot split the trading account across different brokers (for example, you cannot have a 50k evaluation on Tradovate and a 50k evaluation on Rithmic)." — S-MULTI

| Connection | Front ends published (S-PLAT) |
|---|---|
| **Tradovate (CQG data feed)** | Tradovate; NinjaTrader (free NT8 version, or the trader's own Multi-broker or Single-broker license); TradingView and Jigsaw ("You will need your own license for each") |
| **Rithmic** ("Rithmic Paper Trading", gateway "Chicago Area") | R\|Trader Pro, Quantower - Rithmic, ATAS, Motivewave, BookMap, Jigsaw - Rithmic, Sierra Chart, EdgeProX, Finamark, Volfix, WealthCharts, Multicharts, Overcharts |

> "Please note that we do not support NinjaTrader Desktop with Rithmic accounts. Users who wish to use NinjaTrader Desktop must have a Tradovate/CQG account." — S-PLAT

> "Our version rides on the more stable CQG data feed and is free for all TradeDay members. It is also fully integrated with Tradovate - what you do on Ninja will appear on Tradovate, and what you do on Tradovate will appear on Ninja." — S-PLAT

> "Please note that NinjaTrader's own mobile application does not work with the evaluation account." — S-PLAT

**What this means for All Win (NinjaTrader 8 plugin):** at TradeDay, NT8 can be used only on a **Tradovate/CQG** account.

**Does any rule differ by platform?** The homepage cards are identical for Tradovate and Rithmic. One platform-specific item exists. Tradovate accounts let the trader set their own risk limits:

> "Yes, traders can now use the risk settings on Tradovate evaluation and funded Sim/Live accounts. You can set a Daily Loss Limit, Weekly Loss Limit, Daily Profit Trigger, or Weekly Profit Trigger directly in Tradovate." — S-RISK

This is a trader-chosen limit, not a firm rule, so one YAML block per plan/phase/size covers both platforms.

---

## 3. Plans

**The one firm rule (all plans and phases):**

> "TradeDay has just one rule - Don't break the Maximum Drawdown Limit!" — S-OBJ

> "ONE RULE. ONE WAY TO FAIL / Maximum Drawdown / Don’t exceed the maximum drawdown." — S-HOW

### Drawdown mechanics (all plans)

> "Trailing max drawdown limits continues to trail your account growth until they reach the starting account balance." / "These limits are enforced in real time, regardless of how the limit is calculated. You cannot go below the max drawdown limit with an open position during the trading session, if you do the evaluation will be over." — S-MDD

> "If you join a $100k evaluation program, the starting balance will be $100,000, the TMD will be set at $97,000. When you have an account balance of $103,000 your TMD will be at $100,000 where it freezes, and you are not allowed to go below this balance." — S-MDD

> "An End of Day Trailing Maximum Drawdown Limit (EOD TMD) calculates the maximum drawdown from the realized profits on the end-of-day balance (at the 16.00 CT futures market close)." / "An Intraday Trailing Maximum Drawdown (Intraday TMD) calculates the drawdown from unrealized profits in real-time during a trading session." — S-EODINT

> "Important: Even though the max drawdown only moves up at day’s end, you must never let your equity fall below the active drawdown level at any point during the day, doing so will still trigger auto-liquidation." — S-MDD

> "For evaluations and Funded Sims, the Trailing Max Drawdown will trail until they reach the starting account balance, at which time the trailing drawdown will freeze and no longer trail." — S-WDD

### Daily loss limit (all plans)

**Kind `none`.** No firm daily loss limit is published on any page. Several pages state the max drawdown is the only rule (quotes above: S-OBJ, S-HOW, S-QP "There's only one rule on the risk side"). The only daily limit mentioned is optional and set by the trader on Tradovate (S-RISK, §2):

> "The manual lockout is a self-set trading stop. This means that the trader sets a daily loss or risk limit, and once that limit is reached, the account is automatically locked from placing new trades for the rest of the session." — S-RISK

No page says "there is no daily loss limit" in those words, so see gap G7.

### 3.1 Quick Pay — Evaluation (Intraday, and End of Day)

> "Quick Pay comes in four account sizes, and each is available with either an Intraday or End of Day trailing drawdown" — S-QP

| Field | 25K | 50K | 100K | 150K |
|---|---|---|---|---|
| Nominal USD | 25,000 | 50,000 | 100,000 | 150,000 |
| Profit target | $1,500 | $3,000 | $6,000 | $9,000 |
| Trailing max drawdown | $1,000 | $2,000 | $3,000 | $4,500 |
| Drawdown type | Intraday version: `trailing_intraday`. EOD version: `trailing_eod` (16.00 CT). Both stop trailing at the starting balance (S-MDD). | ← | ← | ← |
| Daily loss limit | none (see above) | ← | ← | ← |
| Consistency | 30%: "No day greater than 30% of your total profits." | ← | ← | ← |
| Max position | 2 contracts (20 micros) | 5 (50 micros) | 10 (50 micros) | 15 (50 micros) |
| Min trading days | 5 (conflict with S-TC "minimum 7-day period", see C1) | 5 | 5 | 5 |

Sources: S-HOME cards ("Profit Target $1,500 / Trailing Max Drawdown $1,000 (calculated Intraday) / Position Limits - 2 Contracts (20 Micros) / Consistency 30% / Min trading days 5", and the same for every size); S-SIZES tables "Quick Pay End of Day Evaluation" and "Quick Pay Intraday Evaluation"; S-QP; S-OBJ; S-DAYS ("For QuickPay accounts the minimum number of trading days is 5."); S-CONS.

**Consistency is a soft rule.** Breaking it raises the target; it does not fail the account:

> "If you have a profitable day in excess of the consistency percentage of your profit target you do not fail the evaluation (but your profit target does increase)." / "Best days profit / 30% = Total profits needed to pass the evaluation" — S-CONS

**Micros.** Micros count as one-tenth of a contract, with a cap of 50:

> "We have Micro equivalency. This means if your maximum position limit is 5 lots contracts, you will be able to trade up to 50 lots Micro. Contracts and Micro positions can be combined, up to the equivalent of the contract maximum position limit." / "Please note we limit the maximum Micro position to 50 lots" — S-POS

**Position-limit breaches are not an automatic fail:**

> "We do not fail traders for accidental position limit breaks, but traders who look to exploit the system and continually breach the maximum position limit will lose their account." — S-POS

### 3.2 Quick Pay — Funded Sim (both Quick Pay evaluation types)

> "All Quick Pay Funded Sim accounts will have an intraday drawdown, regardless of the drawdown had on the evaluation." — S-FUNDED

| Field | 25K | 50K | 100K | 150K |
|---|---|---|---|---|
| Trailing max drawdown | $1,000 (intraday) | $2,000 | $3,000 | $4,500 |
| Stops trailing | At the starting balance (S-WDD). TMD is calculated on **gross** profits: "So the TMD will continue to trail, even if users take payouts." (S-QPPAY) | ← | ← | ← |
| Profit target | n/a | | | |
| Consistency | "Consistency: None" | ← | ← | ← |
| Max position | 2 contracts (20 micros) | 5 (50) | 10 (50) | 15 (50) |
| Min days to payout | 1 ("Min Days to Payout: 1") | ← | ← | ← |
| Daily loss limit | none | ← | ← | ← |

Sources: S-HOME "Funded Rules" cards ("Trailing Max Drawdown: $1,000 (calculated intraday) / Milestones: None / Consistency: None / No Payout Buffer / Min Days to Payout: 1 … Position Limit: 2 Contracts (20 Micros)"); S-QP; S-QPPAY; S-FUNDED.

**$10,000 gross-profit pause.** This is a trading constraint:

> "On the day the $10,000 gross profit is reached, during the End of Day run (4:30pm CT) access to the trader’s account will be automatically paused, and the account will be reviewed by the trading team overnight. It’s important to note that any excess profits above the threshold will be removed, and the balance will be revised back to the threshold level." — S-QPLIVE

### 3.3 Quick Pay — Funded Live

> "When the profits are transferred to the Funded Live account, no additional drawdown will be added at this stage." … "The maximum drawdown limit will be set to $0." — S-WDD

Position and slippage limits depend on the balance brought over:

> "contract size and slippage limits are determined based on the balance carried over into the Funded Live, as outlined below:" — S-LIVE. The table on that page has columns Balance brought over / Mini / Micro / Slippage, with these rows:
>
> | Balance brought over | Mini | Micro | Slippage |
> |---|---|---|---|
> | $0–3K | 2 | 20 | $500 |
> | $3K–6K | 3 | 30 | $750 |
> | $6K–10K | 4 | 40 | $1,000 |

**Conflict C2 (position limits).** S-MULTI says instead:

> "The position limits for Funded Live will be the same as those for the Funded Sim that passed, but they may be reduced to reflect the account balance and risk."

**Conflict C3 (slippage buffer basis).** S-SLIP gives the buffer by size, not by balance:

> "$25K & $50K account: $500 / $100K account: $750 / $150K account: $1,000"

How the slippage rule works:

> "If trading losses cause your Funded Live account balance to fall below the required slippage buffer, all open positions will be automatically liquidated." — S-SLIP

### 3.4 Fast Pass — Evaluation (End of Day only)

| Field | 25K | 50K | 100K | 150K |
|---|---|---|---|---|
| Nominal USD | 25,000 | 50,000 | 100,000 | 150,000 |
| Profit target | $1,500 | $3,000 | $6,000 | $9,000 |
| Max drawdown | $1,000 | $2,000 | $3,000 | $4,500 |
| Drawdown type | `trailing_eod`: "All drawdowns are calculated at the End-of-Day." (S-OBJ). Stops trailing at the starting balance (S-MDD). | ← | ← | ← |
| Daily loss limit | none | ← | ← | ← |
| Consistency | 45%: "No day greater than 45% of your total profits." (soft; the target increases) | ← | ← | ← |
| Max position | 2 contracts (20 micros) | 5 (50) | 10 (50) | 15 (50) |
| Min trading days | **conflict C1** | | | |

Sources: S-HOME Fast Pass cards; S-SIZES "Fast Pass Evaluations"; S-FP; S-OBJ; S-CONS ("The consistency percentage for FastPass accounts is 45%.").

**Conflict C1 (minimum trading days).** Official pages give three different answers:

- **3 days:**
  - S-OBJ: "Objective 1: 3 Minimum Trading days"
  - S-DAYS: "For FastPass accounts the minimum number of trading days is 3."
  - S-HOME cards: "Min trading days 3"
- **No minimum:**
  - S-FP: "There are no minimum trading days in the evaluation, if you can hit the target while staying disciplined and consistent, you can pass in 3 days."
  - S-HOW: "5 days on Quick Pay. No minimum on Fast Pass."
  - S-HOME FAQ: "pass in 3 days with no minimum trading day requirement"
  - S-BLOG: "No minimum trading days to pass"
- **7 days (generic, every evaluation):** S-TC: "reach profit targets set for you over a minimum 7-day period"

### 3.5 Fast Pass — Funded Sim

> "Passing moves you into a Funded Sim account with the same End of Day drawdown as your evaluation." / "Position limits work differently here than in the evaluation. You start at 1 contract on a 25K, 2 contracts on a 50K account, 3 on a 100K, and 4 on a 150K (or 20/30/40 micros), and your limit scales up by 1 contract for every $2,000 of profit, calculated at End of Day." — S-FP

| Field | 25K | 50K | 100K | 150K |
|---|---|---|---|---|
| Trailing max drawdown | $1,000 (EOD) | $2,000 | $3,000 | $4,500 |
| Stops trailing | At the starting balance. It also jumps to the starting balance at the first payout: "On the first payout request, if the Trailing Maximum Drawdown (TMD) has not yet reached the starting account balance, it will automatically move up to the starting account balance and remain fixed there moving forward." (S-FPPAY) | ← | ← | ← |
| Consistency | 45% for accounts opened on/after July 26 2026 (payout condition, on gross profits); none before that date | ← | ← | ← |
| Starting max position (S-HOME) | "1 contracts (or 10 Micros) … max 2 minis" | "2 contracts (or 20 Micros)" | "3 contracts (or 30 Micros)" | "4 contracts (or 40 Micros)" |
| Scaling | "+1 contract per $2k profit at EOD". Ceiling published only for 25K ("max 2 minis"); see gap G3. | ← | ← | ← |
| Payout: profitable days | 5 × $100 | 5 × $150 | 5 × $200 | 5 × $250 |
| Daily loss limit | none | ← | ← | ← |

Payout timing also restricts trading:

> "Once you submit a payout request, do not resume trading until you have received your payout." — S-FPPAY

> "Upon a trader’s 5th payout request, the Funded Sim account will transition to a Funded Live account." — S-FPLIVE

**Conflict C5 (100K payout cap; low priority).** Caps do not change what the trader may do, but the pages disagree: S-FP says "$1,850 on a 100K"; S-FPPAY says "$1,875 on a 100K"; S-HOME says "Max Payout Request Per Cycle: $1,875".

### 3.6 Fast Pass — Funded Live

> "The existing Funded Sim account will be closed, and a new Funded Live account will be opened with a $0 starting balance. The same End-of-Day (EOD) drawdown will be assigned from the original evaluation account tier. Funded Live accounts will begin with the same position limits originally assigned to the Funded Sim account: $25K Account: 1 contract / $50K Account: 2 contracts / $100K Account: 3 contracts / $150K Account: 4 contracts / Any future increases to position limits will be granted solely at the discretion of the TradeDay trading team." — S-FPLIVE (same text in S-LIVE)

**Conflict C4 (drawdown amount).** The text says the drawdown equals the original tier (50K = $2,000), but the worked example on the same pages says:

> "Using a $50K account as an example, the TMD is $1500, so the floor starts at -$1500." — S-FPLIVE, S-LIVE, S-FP

Slippage buffer: per S-SLIP (see §3.3).

Micros for Fast Pass Funded Live are not published.

### 3.7 Funded Sim versus Funded Live

> "Funded Sim account / You are placed in a simulation account and paid from the simulated profits you make in the account. … There is no slippage rule." / "A funded live account is a trading account that is a sub-account of our master account at our broker. … You are trading the live markets" — S-SIMLIVE

> "Rule 2: Funded Traders are not permitted to trade within 2% of a price limit. Traders in breach of this policy will lose their funded account." — S-FUNDED, S-PRICE

---

## 4. Trading-day reset

> "Typically the trading day, or trading session, for the majority of products permitted at TradeDay, runs from 5 pm CT, (the market open) until 4 pm CT (the market close) the next day." … "All Evaluation Challenge metrics are based on this trading day session." — S-TDAY

- **Reset:** 5 pm CT. YAML: `{ time: "17:00", tz_as_published: "CT" }`.
- **EOD drawdown calculation:** "at the 16.00 CT futures market close" (S-EODINT).
- **End of Day run:** 4:30pm CT (S-QPLIVE, S-WDD).
- **Forced flat before the close:**

> "Exit all positions 10 minutes before the close. No holding positions when the market closes." / "TradeDay will auto-liquidate all open positions 10 minutes before the daily market close and 2 minutes before TradeDay's tier 1 economic data releases" / "TradeDay can liquidate open positions at 3:50 pm CT only" — S-TIMES

- **Holidays and early closes:**

> "Also, please be aware of any trading holiday market closures or shortened sessions. We will do our best to forewarn you of any impending changes to a market closing time on our site, but it is the trader’s responsibility to be aware of any changes to their own products." (links the CME holiday calendar) — S-TIMES

No TradeDay holiday table is published.

---

## 5. Trade copying stance

**Classification: `conditional`.** Copying across the same person's TradeDay accounts is allowed within account-type groups. Copying in from the trader's own accounts elsewhere is allowed. Different users making the same trades is forbidden.

> "Trade copiers are allowed from your personal accounts or other evaluations in to your TradeDay account." — S-COPY

> "We now support multiple accounts, so you’re welcome to use copy trading across our evaluations or funded accounts. Note, you can use a copy trader to copy trades from your personal account elsewhere into your TradeDay account.
> - Evaluation & Funded Sim: You may copy trade any combination of Evaluation and Funded Sim accounts, with 2 to 10 accounts total.
> - Funded Live: You may copy trade between up to 5 Funded Live accounts.
> - Funded Live, Evaluation & Funded Sim: Copy trading between these account types is not permitted." — S-COPY

> "No. You cannot have active trades simultaneously across a Funded Live account and an Evaluation or Funded Sim account. This restriction applies whether or not the trades are being copy-traded." — S-COPY

> "Traders can use a copy trader to copy trades from their personal account elsewhere into their TradeDay evaluation and Funded accounts." — S-MULTI

S-MULTI names third-party copiers TradeDay has discounts for: "Tradesyncer", "Flowbots Replikanto", "Expert Trading Programmers". It also says: "some platforms have built-in trade copiers (for example, Tradovate, and Quantower)".

**Other limits that apply to copying:**

- **Copying between different users:** "If multiple users are making the same trades, then all the accounts will be shut down, and the users suspended." (S-AUTO)
- **Hedging across accounts:** "Hedging is prohibited at TradeDay (the practice of simultaneously going long in one account and short in another account for the same product)." (S-MULTI, S-PROHIB)
- **Copying from or to other firms:** S-COPY allows copying "from your personal accounts or other evaluations in to your TradeDay account". Copying **out** of TradeDay to other firms is not mentioned (gap G6).

**Account counts** (S-COPY, S-MULTI):

> "A trader can trade up to 10 accounts total." / "The 10 accounts can be any mix of accounts types, with a max of 10 evaluation accounts, 5 Funded Sim accounts and 5 Funded live accounts" / "A trader can only have one username, and we only allow 1 user per household."

> "If a trader has an active Funded Live account, they will not be able to purchase additional accounts until they have been off-boarded from the Funded Live." — S-MULTI

S-MULTI also says "When in Funded Live, you will not be able to purchase additional accounts." This seems to sit uneasily with copying across up to 5 Funded Live accounts. The 5 accounts can come from Funded Sims that graduate, so it is not recorded as a conflict.

---

## 6. Automated trading stance

**Classification: `conditional`** (YAML `automated_trading_allowed: conditional`). A trader may run their own automated trading system on a supported platform. Bots or algorithms bought from a third party are forbidden. High-frequency scalping systems (more than 200 trades a day) are forbidden. TradeDay does not expose the Tradovate API.

**Verbatim rule** (S-AUTO, used as `automation_rule_text`):

> "To use an automated trading system, you must use one of the platforms we support: NinjaTrader, Tradovate, TradingView, Jigsaw and Quantower. TradeDay does not expose the Tradovate API to allow traders to connect automated trading systems directly. Algos and Bots purchased from a third party and employed by the trader are forbidden. If multiple users are making the same trades, then all the accounts will be shut down, and the users suspended."

Same article, summary bullets:

> "We do not make platform APIs available to traders." / "Please read our prohibited trade practices to ensure your ATS is not in breach of our policies." / "TradeDay does not evaluate systems you have purchased from third parties."

Prohibited practices (S-PROHIB; the same text is in S-TC §"Prohibited Conduct"):

> "Using trading bots or Automated Trading Systems (ATS) purchased from a third party. Or multiple users using the same trading bots and ATS's."
> "Using automated trading systems designed to scalp at an extremely high frequency. We do not allow strategies that produce more than 200 trades in a day."
> "Using any software, artificial intelligence, ultra-high speed, or mass Data entry which might manipulate, abuse, or give the user an unfair advantage;"
> "Order splitting. Entering multiple orders at the same price at the same time."

### 6.1 PO details

**(i) Classification:** conditional.

**(ii) Named items:**

- **Semi-automated trading:** not mentioned.
- **Trade copiers:** mentioned and allowed (§5). S-MULTI recommends third-party copiers.
- **Webhooks / alerts:** not mentioned. TradingView is listed as a supported platform for automated trading, and S-PLAT says "Our trading platform, Tradovate, has full integration with these two tools", but webhooks are not named.
- **ATM / automatic brackets:** not named as ATM. Related text:
  - Platform-generated exits, S-SPLIT: "Platform-Generated Exit Orders / Some platforms (such as NinjaTrader trailing stop features) may automatically split exits into smaller orders when closing a position. … This can occur due to platform order management and does not automatically indicate order splitting."
  - Server-side protective stops, S-NET: "we recommend that all traders enter a protective stop at the time of establishing a position. This stop-loss order is worked server side".
  - Bracketing around news is forbidden, S-PROHIB: "Continued exploiting the lack of slippage in the sim environment to achieve unrealistic stop-loss execution, such as bracketing data releases."
  - Copier order splitting is not addressed. S-SPLIT defines order splitting as "multiple identical orders (same instrument, same price level, same side, and submitted at the same time…)", and applies it "only to limit orders and stop orders".

**(iii) Differs by phase?** No difference is published.

- S-PROHIB and S-AUTO do not name a phase.
- S-LIVE: "The trading rules for Quick Pay Funded Live and Fast Pass Funded Live include all prohibited trading practices."
- S-FUNDED: Funded Sim and Funded Live keep the same rules.

---

## 7. Device / hosting rules (PO rev. 3)

**VPS: `no`.**

> "TradeDay does not allow the use of virtual private servers (VPS)" — S-VPS

> "Traders using VPNs, or any software designed to mask the IP address location (Including Apple’s Private Relay service or privacy extensions built into browsers) are required to switch off the service and trade from the address under which the account is registered." — S-VPS

> "Using a VPN or VPS to mask the location of the trading. Please see our policy here." — S-PROHIB (listed as prohibited); the same line is in S-TC.

When the IP check happens:

> "a trader using any IP masking software will be required to switch it off and trade for a full day in line with how they have been trading during their evaluation or funded account, from the location registered with TradeDay." The checks happen "When moving to a Funded Live account / On payout requests from Quick Pay and Fast Pass Funded Sims / When achieving the Quick Pay Funded Sim $10,000 gross profit review" — S-VPS

S-VPS and S-PROHIB word this differently. S-VPS bans VPS use outright ("does not allow the use of virtual private servers"). S-PROHIB bans VPN or VPS use "to mask the location". Both lead to `vps_allowed: no`, so this is recorded as context, not as a conflict.

**Personal device required: `unknown`.** No page requires orders to come from the trader's own device. The firm requires trading from the **registered address/location**. Searched: all 85 help-center articles (grep "device", "VPS", "server", "computer"), the T&C, the homepage and how-it-works. Gap G8.

**Impact for All Win:** running NT8 with the All Win plugin on a VPS is **not allowed** at TradeDay.

---

## 7b. Other rules that affect routing or risk

**Session and news**

- **Day-trading only; no positions over the close.** S-TC: "Permitted times for trading: Day-trading only and all positions must be closed at least 10 minutes prior to the end of any session". S-TIMES: "Overnight trading is allowed, but any open positions must be closed before the market closes the next day." Auto-flat at 3:50 pm CT (§4).
- **Tier 1 news.** S-NEWS: "TradeDay will auto-liquidate all open positions 2 minutes before TradeDay's tier 1 economic data releases, and open markets back up to trade 2 minutes after these releases." Tier 1 list:
  - FOMC minutes 1:00 PM CT (all products)
  - FOMC rate decision 1:00 PM CT (all products)
  - US CPI 7:30 AM CT (all products)
  - NFP 7:30 AM CT (all products)
  - EIA crude 9:30 AM CT / 10:00 AM CT (oil only)
  - EIA nat gas 9:30 AM CT (gas only)
  - Crop reports 11:00 AM CT (ags only)

  Breaches are not an automatic fail: "We do not fail traders for accidental breaches of these guidelines, but traders found to be ignoring these guidelines and abusing the policy will lose their account."

**Order behaviour**

- **Scalping limits.** More than 200 trades a day is forbidden (S-PROHIB, S-SCALP). S-SCALP also forbids "Strategies that trade in a few seconds, designed to capture just a couple of ticks at a time." S-SPLIT defines microscalping as "typically under 10 seconds" when ultra-short trades dominate activity or profit. Enforcement: first instance reset and warning; repeat leads to profit deduction or suspension.
- **Order splitting.** Identical limit or stop orders at the same price and time are prohibited (S-SPLIT). Legitimate scaling at different prices and partial fills are allowed.
- **Hedging.** Prohibited across accounts (§5).
- **Mandatory stop loss.** Not required; recommended only (S-NET).

**Account rules**

- **Max accounts.** 10 total; at most 10 evaluations, 5 Funded Sim, 5 Funded Live; one broker; one user per household (§5).
- **Funded Live and other accounts at the same time.** Not allowed: no open positions in Funded Live while Evaluation or Funded Sim positions are open (S-COPY).
- **Funded Live loss.** S-FP: "If you lose a Funded Live account, you'll enter a mandatory 3-month cooling-off period".
- **Inactivity.** S-INACT: "any TradeDay account that remains dormant or inactive for 30 days will be considered inactive and may be offboarded." Tradovate archives an account after one calendar month without login, and "the account will be considered failed".
- **Minimum or maximum trade duration.** None published apart from the scalping and microscalping rules.
- **Price limits.** Funded traders may not trade within 2% of a price limit (S-PRICE).
- **Metals.** S-PROD gives separate metal position limits ("25K account tier: 2 contracts / 50K … 5 / 100K … 10 / 150K … 15"). Not relevant to MNQ/NQ.

---

## 8. Machine-readable blocks

28 blocks: 7 plan/phase groups × 4 sizes. Each block covers both Tradovate and Rithmic (see §2).

Short source keys used in comments: OBJ, DAYS, HOME, FP, HOW, TC, LIVE, MULTI, SLIP. Each block lists its full URLs under `sources`.

Notes on encoding:

- `drawdown.amount_usd: null` in the Fast Pass Funded Live blocks marks conflict C4.
- Quick Pay Funded Live `amount_usd: "0.00"` is the published "$0" floor, not a loss allowance.

#### Quick Pay — Intraday — evaluation — 25K
```yaml
firm: tradeday
plan: "Quick Pay — Intraday"
phase: evaluation
size_label: "25K"
size_usd: "25000.00"
instruments: [MNQ, NQ]
platforms: [Tradovate, Rithmic]   # one broker per trader; NinjaTrader Desktop only via Tradovate/CQG (see §2)
drawdown: { amount_usd: "1000.00", type: trailing_intraday, stops_trailing_at: "Trailing max drawdown limits continues to trail your account growth until they reach the starting account balance." }
daily_loss_limit: { amount_usd: null, kind: none }   # firm: "ONE RULE. ONE WAY TO FAIL" (max drawdown); optional self-set Tradovate DLL only
profit_target_usd: "1500.00"
consistency_rule: { text: "No day greater than 30% of your total profits.", threshold_pct: "30" }
max_position: { minis: "2", micros: "20", scaling: null }
min_trading_days: 5   # CONFLICT: 5 (S-OBJ, S-DAYS, S-QP, S-HOW, S-HOME) vs T&C "over a minimum 7-day period"
trading_day_reset: { time: "17:00", tz_as_published: "CT" }   # session 5 pm CT - 4 pm CT; EOD drawdown at 16.00 CT; auto-flat 3:50 pm CT
copy_trading_allowed: conditional   # same person only; eval+Funded Sim 2-10 accts; Funded Live up to 5; no Live<->Eval/Sim copying
automated_trading_allowed: conditional   # own ATS on supported platforms; third-party-purchased bots forbidden; >200 trades/day forbidden
automation_rule_text: "To use an automated trading system, you must use one of the platforms we support: NinjaTrader, Tradovate, TradingView, Jigsaw and Quantower. TradeDay does not expose the Tradovate API to allow traders to connect automated trading systems directly. Algos and Bots purchased from a third party and employed by the trader are forbidden. If multiple users are making the same trades, then all the accounts will be shut down, and the users suspended."
vps_allowed: "no"   # "TradeDay does not allow the use of virtual private servers (VPS)"
personal_device_required: unknown   # device not mentioned; firm requires trading from the registered location/address (see §7)
status: conflict
not_confirmed_fields: [min_trading_days]
sources:
  - { field: min_trading_days (7-day, generic), url: "https://www.tradeday.com/terms-and-conditions", accessed: "2026-10-23" }
  - { field: size/drawdown/profit_target/max_position, url: "https://www.tradeday.com/", accessed: "2026-10-23" }
  - { field: size/drawdown/profit_target/max_position, url: "https://tradeday.freshdesk.com/en/support/solutions/articles/103000304248-what-are-the-differences-between-each-account-size-", accessed: "2026-10-23" }
  - { field: drawdown.type/stops_trailing_at, url: "https://tradeday.freshdesk.com/en/support/solutions/articles/103000008855-what-is-the-maximum-drawdown-rule-", accessed: "2026-10-23" }
  - { field: consistency_rule, url: "https://tradeday.freshdesk.com/en/support/solutions/articles/103000008849-what-is-the-consistency-objective-", accessed: "2026-10-23" }
  - { field: min_trading_days, url: "https://tradeday.freshdesk.com/en/support/solutions/articles/103000008850-what-is-the-trade-for-a-minimum-number-of-days-objective-", accessed: "2026-10-23" }
  - { field: daily_loss_limit, url: "https://www.tradeday.com/how-it-works", accessed: "2026-10-23" }
  - { field: max_position.micros (50 cap), url: "https://tradeday.freshdesk.com/en/support/solutions/articles/103000008891-what-are-the-position-limits-guidelines-and-contracts-vs-micros-", accessed: "2026-10-23" }
  - { field: automated_trading_allowed/automation_rule_text, url: "https://tradeday.freshdesk.com/en/support/solutions/articles/103000085101-automated-algo-and-bot-trading", accessed: "2026-10-23" }
  - { field: automation (prohibited practices), url: "https://tradeday.freshdesk.com/en/support/solutions/articles/103000121031-prohibited-trade-practices", accessed: "2026-10-23" }
  - { field: copy_trading_allowed, url: "https://tradeday.freshdesk.com/en/support/solutions/articles/103000008833-can-i-have-multiple-accounts-can-i-use-copy-trader-", accessed: "2026-10-23" }
  - { field: max accounts / one broker, url: "https://tradeday.freshdesk.com/en/support/solutions/articles/103000354796-multiple-accounts", accessed: "2026-10-23" }
  - { field: vps_allowed, url: "https://tradeday.freshdesk.com/en/support/solutions/articles/103000295384-using-a-vpn-vps-or-ip-masking-add-on", accessed: "2026-10-23" }
  - { field: trading_day_reset, url: "https://tradeday.freshdesk.com/en/support/solutions/articles/103000008864-what-is-the-definition-of-a-trading-day-", accessed: "2026-10-23" }
  - { field: trading_day_reset (auto-flat), url: "https://tradeday.freshdesk.com/en/support/solutions/articles/103000018874-what-is-the-permitted-trading-times-guideline-", accessed: "2026-10-23" }
```

#### Quick Pay — Intraday — evaluation — 50K
```yaml
firm: tradeday
plan: "Quick Pay — Intraday"
phase: evaluation
size_label: "50K"
size_usd: "50000.00"
instruments: [MNQ, NQ]
platforms: [Tradovate, Rithmic]   # one broker per trader; NinjaTrader Desktop only via Tradovate/CQG (see §2)
drawdown: { amount_usd: "2000.00", type: trailing_intraday, stops_trailing_at: "Trailing max drawdown limits continues to trail your account growth until they reach the starting account balance." }
daily_loss_limit: { amount_usd: null, kind: none }   # firm: "ONE RULE. ONE WAY TO FAIL" (max drawdown); optional self-set Tradovate DLL only
profit_target_usd: "3000.00"
consistency_rule: { text: "No day greater than 30% of your total profits.", threshold_pct: "30" }
max_position: { minis: "5", micros: "50", scaling: null }
min_trading_days: 5   # CONFLICT: 5 (S-OBJ, S-DAYS, S-QP, S-HOW, S-HOME) vs T&C "over a minimum 7-day period"
trading_day_reset: { time: "17:00", tz_as_published: "CT" }   # session 5 pm CT - 4 pm CT; EOD drawdown at 16.00 CT; auto-flat 3:50 pm CT
copy_trading_allowed: conditional   # same person only; eval+Funded Sim 2-10 accts; Funded Live up to 5; no Live<->Eval/Sim copying
automated_trading_allowed: conditional   # own ATS on supported platforms; third-party-purchased bots forbidden; >200 trades/day forbidden
automation_rule_text: "To use an automated trading system, you must use one of the platforms we support: NinjaTrader, Tradovate, TradingView, Jigsaw and Quantower. TradeDay does not expose the Tradovate API to allow traders to connect automated trading systems directly. Algos and Bots purchased from a third party and employed by the trader are forbidden. If multiple users are making the same trades, then all the accounts will be shut down, and the users suspended."
vps_allowed: "no"   # "TradeDay does not allow the use of virtual private servers (VPS)"
personal_device_required: unknown   # device not mentioned; firm requires trading from the registered location/address (see §7)
status: conflict
not_confirmed_fields: [min_trading_days]
sources:
  - { field: min_trading_days (7-day, generic), url: "https://www.tradeday.com/terms-and-conditions", accessed: "2026-10-23" }
  - { field: size/drawdown/profit_target/max_position, url: "https://www.tradeday.com/", accessed: "2026-10-23" }
  - { field: size/drawdown/profit_target/max_position, url: "https://tradeday.freshdesk.com/en/support/solutions/articles/103000304248-what-are-the-differences-between-each-account-size-", accessed: "2026-10-23" }
  - { field: drawdown.type/stops_trailing_at, url: "https://tradeday.freshdesk.com/en/support/solutions/articles/103000008855-what-is-the-maximum-drawdown-rule-", accessed: "2026-10-23" }
  - { field: consistency_rule, url: "https://tradeday.freshdesk.com/en/support/solutions/articles/103000008849-what-is-the-consistency-objective-", accessed: "2026-10-23" }
  - { field: min_trading_days, url: "https://tradeday.freshdesk.com/en/support/solutions/articles/103000008850-what-is-the-trade-for-a-minimum-number-of-days-objective-", accessed: "2026-10-23" }
  - { field: daily_loss_limit, url: "https://www.tradeday.com/how-it-works", accessed: "2026-10-23" }
  - { field: max_position.micros (50 cap), url: "https://tradeday.freshdesk.com/en/support/solutions/articles/103000008891-what-are-the-position-limits-guidelines-and-contracts-vs-micros-", accessed: "2026-10-23" }
  - { field: automated_trading_allowed/automation_rule_text, url: "https://tradeday.freshdesk.com/en/support/solutions/articles/103000085101-automated-algo-and-bot-trading", accessed: "2026-10-23" }
  - { field: automation (prohibited practices), url: "https://tradeday.freshdesk.com/en/support/solutions/articles/103000121031-prohibited-trade-practices", accessed: "2026-10-23" }
  - { field: copy_trading_allowed, url: "https://tradeday.freshdesk.com/en/support/solutions/articles/103000008833-can-i-have-multiple-accounts-can-i-use-copy-trader-", accessed: "2026-10-23" }
  - { field: max accounts / one broker, url: "https://tradeday.freshdesk.com/en/support/solutions/articles/103000354796-multiple-accounts", accessed: "2026-10-23" }
  - { field: vps_allowed, url: "https://tradeday.freshdesk.com/en/support/solutions/articles/103000295384-using-a-vpn-vps-or-ip-masking-add-on", accessed: "2026-10-23" }
  - { field: trading_day_reset, url: "https://tradeday.freshdesk.com/en/support/solutions/articles/103000008864-what-is-the-definition-of-a-trading-day-", accessed: "2026-10-23" }
  - { field: trading_day_reset (auto-flat), url: "https://tradeday.freshdesk.com/en/support/solutions/articles/103000018874-what-is-the-permitted-trading-times-guideline-", accessed: "2026-10-23" }
```

#### Quick Pay — Intraday — evaluation — 100K
```yaml
firm: tradeday
plan: "Quick Pay — Intraday"
phase: evaluation
size_label: "100K"
size_usd: "100000.00"
instruments: [MNQ, NQ]
platforms: [Tradovate, Rithmic]   # one broker per trader; NinjaTrader Desktop only via Tradovate/CQG (see §2)
drawdown: { amount_usd: "3000.00", type: trailing_intraday, stops_trailing_at: "Trailing max drawdown limits continues to trail your account growth until they reach the starting account balance." }
daily_loss_limit: { amount_usd: null, kind: none }   # firm: "ONE RULE. ONE WAY TO FAIL" (max drawdown); optional self-set Tradovate DLL only
profit_target_usd: "6000.00"
consistency_rule: { text: "No day greater than 30% of your total profits.", threshold_pct: "30" }
max_position: { minis: "10", micros: "50", scaling: null }
min_trading_days: 5   # CONFLICT: 5 (S-OBJ, S-DAYS, S-QP, S-HOW, S-HOME) vs T&C "over a minimum 7-day period"
trading_day_reset: { time: "17:00", tz_as_published: "CT" }   # session 5 pm CT - 4 pm CT; EOD drawdown at 16.00 CT; auto-flat 3:50 pm CT
copy_trading_allowed: conditional   # same person only; eval+Funded Sim 2-10 accts; Funded Live up to 5; no Live<->Eval/Sim copying
automated_trading_allowed: conditional   # own ATS on supported platforms; third-party-purchased bots forbidden; >200 trades/day forbidden
automation_rule_text: "To use an automated trading system, you must use one of the platforms we support: NinjaTrader, Tradovate, TradingView, Jigsaw and Quantower. TradeDay does not expose the Tradovate API to allow traders to connect automated trading systems directly. Algos and Bots purchased from a third party and employed by the trader are forbidden. If multiple users are making the same trades, then all the accounts will be shut down, and the users suspended."
vps_allowed: "no"   # "TradeDay does not allow the use of virtual private servers (VPS)"
personal_device_required: unknown   # device not mentioned; firm requires trading from the registered location/address (see §7)
status: conflict
not_confirmed_fields: [min_trading_days]
sources:
  - { field: min_trading_days (7-day, generic), url: "https://www.tradeday.com/terms-and-conditions", accessed: "2026-10-23" }
  - { field: size/drawdown/profit_target/max_position, url: "https://www.tradeday.com/", accessed: "2026-10-23" }
  - { field: size/drawdown/profit_target/max_position, url: "https://tradeday.freshdesk.com/en/support/solutions/articles/103000304248-what-are-the-differences-between-each-account-size-", accessed: "2026-10-23" }
  - { field: drawdown.type/stops_trailing_at, url: "https://tradeday.freshdesk.com/en/support/solutions/articles/103000008855-what-is-the-maximum-drawdown-rule-", accessed: "2026-10-23" }
  - { field: consistency_rule, url: "https://tradeday.freshdesk.com/en/support/solutions/articles/103000008849-what-is-the-consistency-objective-", accessed: "2026-10-23" }
  - { field: min_trading_days, url: "https://tradeday.freshdesk.com/en/support/solutions/articles/103000008850-what-is-the-trade-for-a-minimum-number-of-days-objective-", accessed: "2026-10-23" }
  - { field: daily_loss_limit, url: "https://www.tradeday.com/how-it-works", accessed: "2026-10-23" }
  - { field: max_position.micros (50 cap), url: "https://tradeday.freshdesk.com/en/support/solutions/articles/103000008891-what-are-the-position-limits-guidelines-and-contracts-vs-micros-", accessed: "2026-10-23" }
  - { field: automated_trading_allowed/automation_rule_text, url: "https://tradeday.freshdesk.com/en/support/solutions/articles/103000085101-automated-algo-and-bot-trading", accessed: "2026-10-23" }
  - { field: automation (prohibited practices), url: "https://tradeday.freshdesk.com/en/support/solutions/articles/103000121031-prohibited-trade-practices", accessed: "2026-10-23" }
  - { field: copy_trading_allowed, url: "https://tradeday.freshdesk.com/en/support/solutions/articles/103000008833-can-i-have-multiple-accounts-can-i-use-copy-trader-", accessed: "2026-10-23" }
  - { field: max accounts / one broker, url: "https://tradeday.freshdesk.com/en/support/solutions/articles/103000354796-multiple-accounts", accessed: "2026-10-23" }
  - { field: vps_allowed, url: "https://tradeday.freshdesk.com/en/support/solutions/articles/103000295384-using-a-vpn-vps-or-ip-masking-add-on", accessed: "2026-10-23" }
  - { field: trading_day_reset, url: "https://tradeday.freshdesk.com/en/support/solutions/articles/103000008864-what-is-the-definition-of-a-trading-day-", accessed: "2026-10-23" }
  - { field: trading_day_reset (auto-flat), url: "https://tradeday.freshdesk.com/en/support/solutions/articles/103000018874-what-is-the-permitted-trading-times-guideline-", accessed: "2026-10-23" }
```

#### Quick Pay — Intraday — evaluation — 150K
```yaml
firm: tradeday
plan: "Quick Pay — Intraday"
phase: evaluation
size_label: "150K"
size_usd: "150000.00"
instruments: [MNQ, NQ]
platforms: [Tradovate, Rithmic]   # one broker per trader; NinjaTrader Desktop only via Tradovate/CQG (see §2)
drawdown: { amount_usd: "4500.00", type: trailing_intraday, stops_trailing_at: "Trailing max drawdown limits continues to trail your account growth until they reach the starting account balance." }
daily_loss_limit: { amount_usd: null, kind: none }   # firm: "ONE RULE. ONE WAY TO FAIL" (max drawdown); optional self-set Tradovate DLL only
profit_target_usd: "9000.00"
consistency_rule: { text: "No day greater than 30% of your total profits.", threshold_pct: "30" }
max_position: { minis: "15", micros: "50", scaling: null }
min_trading_days: 5   # CONFLICT: 5 (S-OBJ, S-DAYS, S-QP, S-HOW, S-HOME) vs T&C "over a minimum 7-day period"
trading_day_reset: { time: "17:00", tz_as_published: "CT" }   # session 5 pm CT - 4 pm CT; EOD drawdown at 16.00 CT; auto-flat 3:50 pm CT
copy_trading_allowed: conditional   # same person only; eval+Funded Sim 2-10 accts; Funded Live up to 5; no Live<->Eval/Sim copying
automated_trading_allowed: conditional   # own ATS on supported platforms; third-party-purchased bots forbidden; >200 trades/day forbidden
automation_rule_text: "To use an automated trading system, you must use one of the platforms we support: NinjaTrader, Tradovate, TradingView, Jigsaw and Quantower. TradeDay does not expose the Tradovate API to allow traders to connect automated trading systems directly. Algos and Bots purchased from a third party and employed by the trader are forbidden. If multiple users are making the same trades, then all the accounts will be shut down, and the users suspended."
vps_allowed: "no"   # "TradeDay does not allow the use of virtual private servers (VPS)"
personal_device_required: unknown   # device not mentioned; firm requires trading from the registered location/address (see §7)
status: conflict
not_confirmed_fields: [min_trading_days]
sources:
  - { field: min_trading_days (7-day, generic), url: "https://www.tradeday.com/terms-and-conditions", accessed: "2026-10-23" }
  - { field: size/drawdown/profit_target/max_position, url: "https://www.tradeday.com/", accessed: "2026-10-23" }
  - { field: size/drawdown/profit_target/max_position, url: "https://tradeday.freshdesk.com/en/support/solutions/articles/103000304248-what-are-the-differences-between-each-account-size-", accessed: "2026-10-23" }
  - { field: drawdown.type/stops_trailing_at, url: "https://tradeday.freshdesk.com/en/support/solutions/articles/103000008855-what-is-the-maximum-drawdown-rule-", accessed: "2026-10-23" }
  - { field: consistency_rule, url: "https://tradeday.freshdesk.com/en/support/solutions/articles/103000008849-what-is-the-consistency-objective-", accessed: "2026-10-23" }
  - { field: min_trading_days, url: "https://tradeday.freshdesk.com/en/support/solutions/articles/103000008850-what-is-the-trade-for-a-minimum-number-of-days-objective-", accessed: "2026-10-23" }
  - { field: daily_loss_limit, url: "https://www.tradeday.com/how-it-works", accessed: "2026-10-23" }
  - { field: max_position.micros (50 cap), url: "https://tradeday.freshdesk.com/en/support/solutions/articles/103000008891-what-are-the-position-limits-guidelines-and-contracts-vs-micros-", accessed: "2026-10-23" }
  - { field: automated_trading_allowed/automation_rule_text, url: "https://tradeday.freshdesk.com/en/support/solutions/articles/103000085101-automated-algo-and-bot-trading", accessed: "2026-10-23" }
  - { field: automation (prohibited practices), url: "https://tradeday.freshdesk.com/en/support/solutions/articles/103000121031-prohibited-trade-practices", accessed: "2026-10-23" }
  - { field: copy_trading_allowed, url: "https://tradeday.freshdesk.com/en/support/solutions/articles/103000008833-can-i-have-multiple-accounts-can-i-use-copy-trader-", accessed: "2026-10-23" }
  - { field: max accounts / one broker, url: "https://tradeday.freshdesk.com/en/support/solutions/articles/103000354796-multiple-accounts", accessed: "2026-10-23" }
  - { field: vps_allowed, url: "https://tradeday.freshdesk.com/en/support/solutions/articles/103000295384-using-a-vpn-vps-or-ip-masking-add-on", accessed: "2026-10-23" }
  - { field: trading_day_reset, url: "https://tradeday.freshdesk.com/en/support/solutions/articles/103000008864-what-is-the-definition-of-a-trading-day-", accessed: "2026-10-23" }
  - { field: trading_day_reset (auto-flat), url: "https://tradeday.freshdesk.com/en/support/solutions/articles/103000018874-what-is-the-permitted-trading-times-guideline-", accessed: "2026-10-23" }
```

#### Quick Pay — End of Day (Intraday Funded Drawdown) — evaluation — 25K
```yaml
firm: tradeday
plan: "Quick Pay — End of Day (Intraday Funded Drawdown)"
phase: evaluation
size_label: "25K"
size_usd: "25000.00"
instruments: [MNQ, NQ]
platforms: [Tradovate, Rithmic]   # one broker per trader; NinjaTrader Desktop only via Tradovate/CQG (see §2)
drawdown: { amount_usd: "1000.00", type: trailing_eod, stops_trailing_at: "Trailing max drawdown limits continues to trail your account growth until they reach the starting account balance." }
daily_loss_limit: { amount_usd: null, kind: none }   # firm: "ONE RULE. ONE WAY TO FAIL" (max drawdown); optional self-set Tradovate DLL only
profit_target_usd: "1500.00"
consistency_rule: { text: "No day greater than 30% of your total profits.", threshold_pct: "30" }
max_position: { minis: "2", micros: "20", scaling: null }
min_trading_days: 5   # CONFLICT: 5 (S-OBJ, S-DAYS, S-QP, S-HOW, S-HOME) vs T&C "over a minimum 7-day period"
trading_day_reset: { time: "17:00", tz_as_published: "CT" }   # session 5 pm CT - 4 pm CT; EOD drawdown at 16.00 CT; auto-flat 3:50 pm CT
copy_trading_allowed: conditional   # same person only; eval+Funded Sim 2-10 accts; Funded Live up to 5; no Live<->Eval/Sim copying
automated_trading_allowed: conditional   # own ATS on supported platforms; third-party-purchased bots forbidden; >200 trades/day forbidden
automation_rule_text: "To use an automated trading system, you must use one of the platforms we support: NinjaTrader, Tradovate, TradingView, Jigsaw and Quantower. TradeDay does not expose the Tradovate API to allow traders to connect automated trading systems directly. Algos and Bots purchased from a third party and employed by the trader are forbidden. If multiple users are making the same trades, then all the accounts will be shut down, and the users suspended."
vps_allowed: "no"   # "TradeDay does not allow the use of virtual private servers (VPS)"
personal_device_required: unknown   # device not mentioned; firm requires trading from the registered location/address (see §7)
status: conflict
not_confirmed_fields: [min_trading_days]
sources:
  - { field: min_trading_days (7-day, generic), url: "https://www.tradeday.com/terms-and-conditions", accessed: "2026-10-23" }
  - { field: size/drawdown/profit_target/max_position, url: "https://www.tradeday.com/", accessed: "2026-10-23" }
  - { field: size/drawdown/profit_target/max_position, url: "https://tradeday.freshdesk.com/en/support/solutions/articles/103000304248-what-are-the-differences-between-each-account-size-", accessed: "2026-10-23" }
  - { field: drawdown.type/stops_trailing_at, url: "https://tradeday.freshdesk.com/en/support/solutions/articles/103000008855-what-is-the-maximum-drawdown-rule-", accessed: "2026-10-23" }
  - { field: drawdown.type (EOD at 16.00 CT), url: "https://tradeday.freshdesk.com/en/support/solutions/articles/103000335948-definition-and-differences-between-end-of-day-trailing-max-drawdown-and-intraday-trailing-maximum-dr", accessed: "2026-10-23" }
  - { field: consistency_rule, url: "https://tradeday.freshdesk.com/en/support/solutions/articles/103000008849-what-is-the-consistency-objective-", accessed: "2026-10-23" }
  - { field: min_trading_days, url: "https://tradeday.freshdesk.com/en/support/solutions/articles/103000008850-what-is-the-trade-for-a-minimum-number-of-days-objective-", accessed: "2026-10-23" }
  - { field: daily_loss_limit, url: "https://www.tradeday.com/how-it-works", accessed: "2026-10-23" }
  - { field: max_position.micros (50 cap), url: "https://tradeday.freshdesk.com/en/support/solutions/articles/103000008891-what-are-the-position-limits-guidelines-and-contracts-vs-micros-", accessed: "2026-10-23" }
  - { field: automated_trading_allowed/automation_rule_text, url: "https://tradeday.freshdesk.com/en/support/solutions/articles/103000085101-automated-algo-and-bot-trading", accessed: "2026-10-23" }
  - { field: automation (prohibited practices), url: "https://tradeday.freshdesk.com/en/support/solutions/articles/103000121031-prohibited-trade-practices", accessed: "2026-10-23" }
  - { field: copy_trading_allowed, url: "https://tradeday.freshdesk.com/en/support/solutions/articles/103000008833-can-i-have-multiple-accounts-can-i-use-copy-trader-", accessed: "2026-10-23" }
  - { field: max accounts / one broker, url: "https://tradeday.freshdesk.com/en/support/solutions/articles/103000354796-multiple-accounts", accessed: "2026-10-23" }
  - { field: vps_allowed, url: "https://tradeday.freshdesk.com/en/support/solutions/articles/103000295384-using-a-vpn-vps-or-ip-masking-add-on", accessed: "2026-10-23" }
  - { field: trading_day_reset, url: "https://tradeday.freshdesk.com/en/support/solutions/articles/103000008864-what-is-the-definition-of-a-trading-day-", accessed: "2026-10-23" }
  - { field: trading_day_reset (auto-flat), url: "https://tradeday.freshdesk.com/en/support/solutions/articles/103000018874-what-is-the-permitted-trading-times-guideline-", accessed: "2026-10-23" }
```

#### Quick Pay — End of Day (Intraday Funded Drawdown) — evaluation — 50K
```yaml
firm: tradeday
plan: "Quick Pay — End of Day (Intraday Funded Drawdown)"
phase: evaluation
size_label: "50K"
size_usd: "50000.00"
instruments: [MNQ, NQ]
platforms: [Tradovate, Rithmic]   # one broker per trader; NinjaTrader Desktop only via Tradovate/CQG (see §2)
drawdown: { amount_usd: "2000.00", type: trailing_eod, stops_trailing_at: "Trailing max drawdown limits continues to trail your account growth until they reach the starting account balance." }
daily_loss_limit: { amount_usd: null, kind: none }   # firm: "ONE RULE. ONE WAY TO FAIL" (max drawdown); optional self-set Tradovate DLL only
profit_target_usd: "3000.00"
consistency_rule: { text: "No day greater than 30% of your total profits.", threshold_pct: "30" }
max_position: { minis: "5", micros: "50", scaling: null }
min_trading_days: 5   # CONFLICT: 5 (S-OBJ, S-DAYS, S-QP, S-HOW, S-HOME) vs T&C "over a minimum 7-day period"
trading_day_reset: { time: "17:00", tz_as_published: "CT" }   # session 5 pm CT - 4 pm CT; EOD drawdown at 16.00 CT; auto-flat 3:50 pm CT
copy_trading_allowed: conditional   # same person only; eval+Funded Sim 2-10 accts; Funded Live up to 5; no Live<->Eval/Sim copying
automated_trading_allowed: conditional   # own ATS on supported platforms; third-party-purchased bots forbidden; >200 trades/day forbidden
automation_rule_text: "To use an automated trading system, you must use one of the platforms we support: NinjaTrader, Tradovate, TradingView, Jigsaw and Quantower. TradeDay does not expose the Tradovate API to allow traders to connect automated trading systems directly. Algos and Bots purchased from a third party and employed by the trader are forbidden. If multiple users are making the same trades, then all the accounts will be shut down, and the users suspended."
vps_allowed: "no"   # "TradeDay does not allow the use of virtual private servers (VPS)"
personal_device_required: unknown   # device not mentioned; firm requires trading from the registered location/address (see §7)
status: conflict
not_confirmed_fields: [min_trading_days]
sources:
  - { field: min_trading_days (7-day, generic), url: "https://www.tradeday.com/terms-and-conditions", accessed: "2026-10-23" }
  - { field: size/drawdown/profit_target/max_position, url: "https://www.tradeday.com/", accessed: "2026-10-23" }
  - { field: size/drawdown/profit_target/max_position, url: "https://tradeday.freshdesk.com/en/support/solutions/articles/103000304248-what-are-the-differences-between-each-account-size-", accessed: "2026-10-23" }
  - { field: drawdown.type/stops_trailing_at, url: "https://tradeday.freshdesk.com/en/support/solutions/articles/103000008855-what-is-the-maximum-drawdown-rule-", accessed: "2026-10-23" }
  - { field: drawdown.type (EOD at 16.00 CT), url: "https://tradeday.freshdesk.com/en/support/solutions/articles/103000335948-definition-and-differences-between-end-of-day-trailing-max-drawdown-and-intraday-trailing-maximum-dr", accessed: "2026-10-23" }
  - { field: consistency_rule, url: "https://tradeday.freshdesk.com/en/support/solutions/articles/103000008849-what-is-the-consistency-objective-", accessed: "2026-10-23" }
  - { field: min_trading_days, url: "https://tradeday.freshdesk.com/en/support/solutions/articles/103000008850-what-is-the-trade-for-a-minimum-number-of-days-objective-", accessed: "2026-10-23" }
  - { field: daily_loss_limit, url: "https://www.tradeday.com/how-it-works", accessed: "2026-10-23" }
  - { field: max_position.micros (50 cap), url: "https://tradeday.freshdesk.com/en/support/solutions/articles/103000008891-what-are-the-position-limits-guidelines-and-contracts-vs-micros-", accessed: "2026-10-23" }
  - { field: automated_trading_allowed/automation_rule_text, url: "https://tradeday.freshdesk.com/en/support/solutions/articles/103000085101-automated-algo-and-bot-trading", accessed: "2026-10-23" }
  - { field: automation (prohibited practices), url: "https://tradeday.freshdesk.com/en/support/solutions/articles/103000121031-prohibited-trade-practices", accessed: "2026-10-23" }
  - { field: copy_trading_allowed, url: "https://tradeday.freshdesk.com/en/support/solutions/articles/103000008833-can-i-have-multiple-accounts-can-i-use-copy-trader-", accessed: "2026-10-23" }
  - { field: max accounts / one broker, url: "https://tradeday.freshdesk.com/en/support/solutions/articles/103000354796-multiple-accounts", accessed: "2026-10-23" }
  - { field: vps_allowed, url: "https://tradeday.freshdesk.com/en/support/solutions/articles/103000295384-using-a-vpn-vps-or-ip-masking-add-on", accessed: "2026-10-23" }
  - { field: trading_day_reset, url: "https://tradeday.freshdesk.com/en/support/solutions/articles/103000008864-what-is-the-definition-of-a-trading-day-", accessed: "2026-10-23" }
  - { field: trading_day_reset (auto-flat), url: "https://tradeday.freshdesk.com/en/support/solutions/articles/103000018874-what-is-the-permitted-trading-times-guideline-", accessed: "2026-10-23" }
```

#### Quick Pay — End of Day (Intraday Funded Drawdown) — evaluation — 100K
```yaml
firm: tradeday
plan: "Quick Pay — End of Day (Intraday Funded Drawdown)"
phase: evaluation
size_label: "100K"
size_usd: "100000.00"
instruments: [MNQ, NQ]
platforms: [Tradovate, Rithmic]   # one broker per trader; NinjaTrader Desktop only via Tradovate/CQG (see §2)
drawdown: { amount_usd: "3000.00", type: trailing_eod, stops_trailing_at: "Trailing max drawdown limits continues to trail your account growth until they reach the starting account balance." }
daily_loss_limit: { amount_usd: null, kind: none }   # firm: "ONE RULE. ONE WAY TO FAIL" (max drawdown); optional self-set Tradovate DLL only
profit_target_usd: "6000.00"
consistency_rule: { text: "No day greater than 30% of your total profits.", threshold_pct: "30" }
max_position: { minis: "10", micros: "50", scaling: null }
min_trading_days: 5   # CONFLICT: 5 (S-OBJ, S-DAYS, S-QP, S-HOW, S-HOME) vs T&C "over a minimum 7-day period"
trading_day_reset: { time: "17:00", tz_as_published: "CT" }   # session 5 pm CT - 4 pm CT; EOD drawdown at 16.00 CT; auto-flat 3:50 pm CT
copy_trading_allowed: conditional   # same person only; eval+Funded Sim 2-10 accts; Funded Live up to 5; no Live<->Eval/Sim copying
automated_trading_allowed: conditional   # own ATS on supported platforms; third-party-purchased bots forbidden; >200 trades/day forbidden
automation_rule_text: "To use an automated trading system, you must use one of the platforms we support: NinjaTrader, Tradovate, TradingView, Jigsaw and Quantower. TradeDay does not expose the Tradovate API to allow traders to connect automated trading systems directly. Algos and Bots purchased from a third party and employed by the trader are forbidden. If multiple users are making the same trades, then all the accounts will be shut down, and the users suspended."
vps_allowed: "no"   # "TradeDay does not allow the use of virtual private servers (VPS)"
personal_device_required: unknown   # device not mentioned; firm requires trading from the registered location/address (see §7)
status: conflict
not_confirmed_fields: [min_trading_days]
sources:
  - { field: min_trading_days (7-day, generic), url: "https://www.tradeday.com/terms-and-conditions", accessed: "2026-10-23" }
  - { field: size/drawdown/profit_target/max_position, url: "https://www.tradeday.com/", accessed: "2026-10-23" }
  - { field: size/drawdown/profit_target/max_position, url: "https://tradeday.freshdesk.com/en/support/solutions/articles/103000304248-what-are-the-differences-between-each-account-size-", accessed: "2026-10-23" }
  - { field: drawdown.type/stops_trailing_at, url: "https://tradeday.freshdesk.com/en/support/solutions/articles/103000008855-what-is-the-maximum-drawdown-rule-", accessed: "2026-10-23" }
  - { field: drawdown.type (EOD at 16.00 CT), url: "https://tradeday.freshdesk.com/en/support/solutions/articles/103000335948-definition-and-differences-between-end-of-day-trailing-max-drawdown-and-intraday-trailing-maximum-dr", accessed: "2026-10-23" }
  - { field: consistency_rule, url: "https://tradeday.freshdesk.com/en/support/solutions/articles/103000008849-what-is-the-consistency-objective-", accessed: "2026-10-23" }
  - { field: min_trading_days, url: "https://tradeday.freshdesk.com/en/support/solutions/articles/103000008850-what-is-the-trade-for-a-minimum-number-of-days-objective-", accessed: "2026-10-23" }
  - { field: daily_loss_limit, url: "https://www.tradeday.com/how-it-works", accessed: "2026-10-23" }
  - { field: max_position.micros (50 cap), url: "https://tradeday.freshdesk.com/en/support/solutions/articles/103000008891-what-are-the-position-limits-guidelines-and-contracts-vs-micros-", accessed: "2026-10-23" }
  - { field: automated_trading_allowed/automation_rule_text, url: "https://tradeday.freshdesk.com/en/support/solutions/articles/103000085101-automated-algo-and-bot-trading", accessed: "2026-10-23" }
  - { field: automation (prohibited practices), url: "https://tradeday.freshdesk.com/en/support/solutions/articles/103000121031-prohibited-trade-practices", accessed: "2026-10-23" }
  - { field: copy_trading_allowed, url: "https://tradeday.freshdesk.com/en/support/solutions/articles/103000008833-can-i-have-multiple-accounts-can-i-use-copy-trader-", accessed: "2026-10-23" }
  - { field: max accounts / one broker, url: "https://tradeday.freshdesk.com/en/support/solutions/articles/103000354796-multiple-accounts", accessed: "2026-10-23" }
  - { field: vps_allowed, url: "https://tradeday.freshdesk.com/en/support/solutions/articles/103000295384-using-a-vpn-vps-or-ip-masking-add-on", accessed: "2026-10-23" }
  - { field: trading_day_reset, url: "https://tradeday.freshdesk.com/en/support/solutions/articles/103000008864-what-is-the-definition-of-a-trading-day-", accessed: "2026-10-23" }
  - { field: trading_day_reset (auto-flat), url: "https://tradeday.freshdesk.com/en/support/solutions/articles/103000018874-what-is-the-permitted-trading-times-guideline-", accessed: "2026-10-23" }
```

#### Quick Pay — End of Day (Intraday Funded Drawdown) — evaluation — 150K
```yaml
firm: tradeday
plan: "Quick Pay — End of Day (Intraday Funded Drawdown)"
phase: evaluation
size_label: "150K"
size_usd: "150000.00"
instruments: [MNQ, NQ]
platforms: [Tradovate, Rithmic]   # one broker per trader; NinjaTrader Desktop only via Tradovate/CQG (see §2)
drawdown: { amount_usd: "4500.00", type: trailing_eod, stops_trailing_at: "Trailing max drawdown limits continues to trail your account growth until they reach the starting account balance." }
daily_loss_limit: { amount_usd: null, kind: none }   # firm: "ONE RULE. ONE WAY TO FAIL" (max drawdown); optional self-set Tradovate DLL only
profit_target_usd: "9000.00"
consistency_rule: { text: "No day greater than 30% of your total profits.", threshold_pct: "30" }
max_position: { minis: "15", micros: "50", scaling: null }
min_trading_days: 5   # CONFLICT: 5 (S-OBJ, S-DAYS, S-QP, S-HOW, S-HOME) vs T&C "over a minimum 7-day period"
trading_day_reset: { time: "17:00", tz_as_published: "CT" }   # session 5 pm CT - 4 pm CT; EOD drawdown at 16.00 CT; auto-flat 3:50 pm CT
copy_trading_allowed: conditional   # same person only; eval+Funded Sim 2-10 accts; Funded Live up to 5; no Live<->Eval/Sim copying
automated_trading_allowed: conditional   # own ATS on supported platforms; third-party-purchased bots forbidden; >200 trades/day forbidden
automation_rule_text: "To use an automated trading system, you must use one of the platforms we support: NinjaTrader, Tradovate, TradingView, Jigsaw and Quantower. TradeDay does not expose the Tradovate API to allow traders to connect automated trading systems directly. Algos and Bots purchased from a third party and employed by the trader are forbidden. If multiple users are making the same trades, then all the accounts will be shut down, and the users suspended."
vps_allowed: "no"   # "TradeDay does not allow the use of virtual private servers (VPS)"
personal_device_required: unknown   # device not mentioned; firm requires trading from the registered location/address (see §7)
status: conflict
not_confirmed_fields: [min_trading_days]
sources:
  - { field: min_trading_days (7-day, generic), url: "https://www.tradeday.com/terms-and-conditions", accessed: "2026-10-23" }
  - { field: size/drawdown/profit_target/max_position, url: "https://www.tradeday.com/", accessed: "2026-10-23" }
  - { field: size/drawdown/profit_target/max_position, url: "https://tradeday.freshdesk.com/en/support/solutions/articles/103000304248-what-are-the-differences-between-each-account-size-", accessed: "2026-10-23" }
  - { field: drawdown.type/stops_trailing_at, url: "https://tradeday.freshdesk.com/en/support/solutions/articles/103000008855-what-is-the-maximum-drawdown-rule-", accessed: "2026-10-23" }
  - { field: drawdown.type (EOD at 16.00 CT), url: "https://tradeday.freshdesk.com/en/support/solutions/articles/103000335948-definition-and-differences-between-end-of-day-trailing-max-drawdown-and-intraday-trailing-maximum-dr", accessed: "2026-10-23" }
  - { field: consistency_rule, url: "https://tradeday.freshdesk.com/en/support/solutions/articles/103000008849-what-is-the-consistency-objective-", accessed: "2026-10-23" }
  - { field: min_trading_days, url: "https://tradeday.freshdesk.com/en/support/solutions/articles/103000008850-what-is-the-trade-for-a-minimum-number-of-days-objective-", accessed: "2026-10-23" }
  - { field: daily_loss_limit, url: "https://www.tradeday.com/how-it-works", accessed: "2026-10-23" }
  - { field: max_position.micros (50 cap), url: "https://tradeday.freshdesk.com/en/support/solutions/articles/103000008891-what-are-the-position-limits-guidelines-and-contracts-vs-micros-", accessed: "2026-10-23" }
  - { field: automated_trading_allowed/automation_rule_text, url: "https://tradeday.freshdesk.com/en/support/solutions/articles/103000085101-automated-algo-and-bot-trading", accessed: "2026-10-23" }
  - { field: automation (prohibited practices), url: "https://tradeday.freshdesk.com/en/support/solutions/articles/103000121031-prohibited-trade-practices", accessed: "2026-10-23" }
  - { field: copy_trading_allowed, url: "https://tradeday.freshdesk.com/en/support/solutions/articles/103000008833-can-i-have-multiple-accounts-can-i-use-copy-trader-", accessed: "2026-10-23" }
  - { field: max accounts / one broker, url: "https://tradeday.freshdesk.com/en/support/solutions/articles/103000354796-multiple-accounts", accessed: "2026-10-23" }
  - { field: vps_allowed, url: "https://tradeday.freshdesk.com/en/support/solutions/articles/103000295384-using-a-vpn-vps-or-ip-masking-add-on", accessed: "2026-10-23" }
  - { field: trading_day_reset, url: "https://tradeday.freshdesk.com/en/support/solutions/articles/103000008864-what-is-the-definition-of-a-trading-day-", accessed: "2026-10-23" }
  - { field: trading_day_reset (auto-flat), url: "https://tradeday.freshdesk.com/en/support/solutions/articles/103000018874-what-is-the-permitted-trading-times-guideline-", accessed: "2026-10-23" }
```

#### Quick Pay — funded — 25K
```yaml
firm: tradeday
plan: "Quick Pay"
phase: funded
size_label: "25K"
size_usd: "25000.00"
instruments: [MNQ, NQ]
platforms: [Tradovate, Rithmic]   # one broker per trader; NinjaTrader Desktop only via Tradovate/CQG (see §2)
drawdown: { amount_usd: "1000.00", type: trailing_intraday, stops_trailing_at: "For evaluations and Funded Sims, the Trailing Max Drawdown will trail until they reach the starting account balance, at which time the trailing drawdown will freeze and no longer trail. (TMD calculated on gross profits: current profits plus payouts.)" }
daily_loss_limit: { amount_usd: null, kind: none }   # firm: "ONE RULE. ONE WAY TO FAIL" (max drawdown); optional self-set Tradovate DLL only
profit_target_usd: null
consistency_rule: { text: "Consistency: None", threshold_pct: null }
max_position: { minis: "2", micros: "20", scaling: null }
min_trading_days: null
trading_day_reset: { time: "17:00", tz_as_published: "CT" }   # session 5 pm CT - 4 pm CT; EOD drawdown at 16.00 CT; auto-flat 3:50 pm CT
copy_trading_allowed: conditional   # same person only; eval+Funded Sim 2-10 accts; Funded Live up to 5; no Live<->Eval/Sim copying
automated_trading_allowed: conditional   # own ATS on supported platforms; third-party-purchased bots forbidden; >200 trades/day forbidden
automation_rule_text: "To use an automated trading system, you must use one of the platforms we support: NinjaTrader, Tradovate, TradingView, Jigsaw and Quantower. TradeDay does not expose the Tradovate API to allow traders to connect automated trading systems directly. Algos and Bots purchased from a third party and employed by the trader are forbidden. If multiple users are making the same trades, then all the accounts will be shut down, and the users suspended."
vps_allowed: "no"   # "TradeDay does not allow the use of virtual private servers (VPS)"
personal_device_required: unknown   # device not mentioned; firm requires trading from the registered location/address (see §7)
status: verified
not_confirmed_fields: []
# profit_target n/a. Account access is paused at $10,000 gross profit (EOD run 4:30pm CT) for Funded Live review; profits above are removed.
# Min Days to Payout: 1 (homepage). Funded Sim cap: max 5 active Funded Sim accounts.
sources:
  - { field: drawdown.amount/max_position/consistency, url: "https://www.tradeday.com/", accessed: "2026-10-23" }
  - { field: drawdown.type (intraday regardless of eval), url: "https://tradeday.freshdesk.com/en/support/solutions/articles/103000008889-what-are-the-rules-for-funded-funded-sim-and-funded-live-traders-", accessed: "2026-10-23" }
  - { field: drawdown gross-profit basis, url: "https://tradeday.freshdesk.com/en/support/solutions/articles/103000405228-what-is-quick-pay-", accessed: "2026-10-23" }
  - { field: stops_trailing_at, url: "https://tradeday.freshdesk.com/en/support/solutions/articles/103000250682-funded-accounts-withdrawals-and-the-impact-of-drawdowns", accessed: "2026-10-23" }
  - { field: daily_loss_limit, url: "https://www.tradeday.com/how-it-works", accessed: "2026-10-23" }
  - { field: automated_trading_allowed/automation_rule_text, url: "https://tradeday.freshdesk.com/en/support/solutions/articles/103000085101-automated-algo-and-bot-trading", accessed: "2026-10-23" }
  - { field: automation (prohibited practices), url: "https://tradeday.freshdesk.com/en/support/solutions/articles/103000121031-prohibited-trade-practices", accessed: "2026-10-23" }
  - { field: copy_trading_allowed, url: "https://tradeday.freshdesk.com/en/support/solutions/articles/103000008833-can-i-have-multiple-accounts-can-i-use-copy-trader-", accessed: "2026-10-23" }
  - { field: max accounts / one broker, url: "https://tradeday.freshdesk.com/en/support/solutions/articles/103000354796-multiple-accounts", accessed: "2026-10-23" }
  - { field: vps_allowed, url: "https://tradeday.freshdesk.com/en/support/solutions/articles/103000295384-using-a-vpn-vps-or-ip-masking-add-on", accessed: "2026-10-23" }
  - { field: trading_day_reset, url: "https://tradeday.freshdesk.com/en/support/solutions/articles/103000008864-what-is-the-definition-of-a-trading-day-", accessed: "2026-10-23" }
  - { field: trading_day_reset (auto-flat), url: "https://tradeday.freshdesk.com/en/support/solutions/articles/103000018874-what-is-the-permitted-trading-times-guideline-", accessed: "2026-10-23" }
```

#### Quick Pay — funded — 50K
```yaml
firm: tradeday
plan: "Quick Pay"
phase: funded
size_label: "50K"
size_usd: "50000.00"
instruments: [MNQ, NQ]
platforms: [Tradovate, Rithmic]   # one broker per trader; NinjaTrader Desktop only via Tradovate/CQG (see §2)
drawdown: { amount_usd: "2000.00", type: trailing_intraday, stops_trailing_at: "For evaluations and Funded Sims, the Trailing Max Drawdown will trail until they reach the starting account balance, at which time the trailing drawdown will freeze and no longer trail. (TMD calculated on gross profits: current profits plus payouts.)" }
daily_loss_limit: { amount_usd: null, kind: none }   # firm: "ONE RULE. ONE WAY TO FAIL" (max drawdown); optional self-set Tradovate DLL only
profit_target_usd: null
consistency_rule: { text: "Consistency: None", threshold_pct: null }
max_position: { minis: "5", micros: "50", scaling: null }
min_trading_days: null
trading_day_reset: { time: "17:00", tz_as_published: "CT" }   # session 5 pm CT - 4 pm CT; EOD drawdown at 16.00 CT; auto-flat 3:50 pm CT
copy_trading_allowed: conditional   # same person only; eval+Funded Sim 2-10 accts; Funded Live up to 5; no Live<->Eval/Sim copying
automated_trading_allowed: conditional   # own ATS on supported platforms; third-party-purchased bots forbidden; >200 trades/day forbidden
automation_rule_text: "To use an automated trading system, you must use one of the platforms we support: NinjaTrader, Tradovate, TradingView, Jigsaw and Quantower. TradeDay does not expose the Tradovate API to allow traders to connect automated trading systems directly. Algos and Bots purchased from a third party and employed by the trader are forbidden. If multiple users are making the same trades, then all the accounts will be shut down, and the users suspended."
vps_allowed: "no"   # "TradeDay does not allow the use of virtual private servers (VPS)"
personal_device_required: unknown   # device not mentioned; firm requires trading from the registered location/address (see §7)
status: verified
not_confirmed_fields: []
# profit_target n/a. Account access is paused at $10,000 gross profit (EOD run 4:30pm CT) for Funded Live review; profits above are removed.
# Min Days to Payout: 1 (homepage). Funded Sim cap: max 5 active Funded Sim accounts.
sources:
  - { field: drawdown.amount/max_position/consistency, url: "https://www.tradeday.com/", accessed: "2026-10-23" }
  - { field: drawdown.type (intraday regardless of eval), url: "https://tradeday.freshdesk.com/en/support/solutions/articles/103000008889-what-are-the-rules-for-funded-funded-sim-and-funded-live-traders-", accessed: "2026-10-23" }
  - { field: drawdown gross-profit basis, url: "https://tradeday.freshdesk.com/en/support/solutions/articles/103000405228-what-is-quick-pay-", accessed: "2026-10-23" }
  - { field: stops_trailing_at, url: "https://tradeday.freshdesk.com/en/support/solutions/articles/103000250682-funded-accounts-withdrawals-and-the-impact-of-drawdowns", accessed: "2026-10-23" }
  - { field: daily_loss_limit, url: "https://www.tradeday.com/how-it-works", accessed: "2026-10-23" }
  - { field: automated_trading_allowed/automation_rule_text, url: "https://tradeday.freshdesk.com/en/support/solutions/articles/103000085101-automated-algo-and-bot-trading", accessed: "2026-10-23" }
  - { field: automation (prohibited practices), url: "https://tradeday.freshdesk.com/en/support/solutions/articles/103000121031-prohibited-trade-practices", accessed: "2026-10-23" }
  - { field: copy_trading_allowed, url: "https://tradeday.freshdesk.com/en/support/solutions/articles/103000008833-can-i-have-multiple-accounts-can-i-use-copy-trader-", accessed: "2026-10-23" }
  - { field: max accounts / one broker, url: "https://tradeday.freshdesk.com/en/support/solutions/articles/103000354796-multiple-accounts", accessed: "2026-10-23" }
  - { field: vps_allowed, url: "https://tradeday.freshdesk.com/en/support/solutions/articles/103000295384-using-a-vpn-vps-or-ip-masking-add-on", accessed: "2026-10-23" }
  - { field: trading_day_reset, url: "https://tradeday.freshdesk.com/en/support/solutions/articles/103000008864-what-is-the-definition-of-a-trading-day-", accessed: "2026-10-23" }
  - { field: trading_day_reset (auto-flat), url: "https://tradeday.freshdesk.com/en/support/solutions/articles/103000018874-what-is-the-permitted-trading-times-guideline-", accessed: "2026-10-23" }
```

#### Quick Pay — funded — 100K
```yaml
firm: tradeday
plan: "Quick Pay"
phase: funded
size_label: "100K"
size_usd: "100000.00"
instruments: [MNQ, NQ]
platforms: [Tradovate, Rithmic]   # one broker per trader; NinjaTrader Desktop only via Tradovate/CQG (see §2)
drawdown: { amount_usd: "3000.00", type: trailing_intraday, stops_trailing_at: "For evaluations and Funded Sims, the Trailing Max Drawdown will trail until they reach the starting account balance, at which time the trailing drawdown will freeze and no longer trail. (TMD calculated on gross profits: current profits plus payouts.)" }
daily_loss_limit: { amount_usd: null, kind: none }   # firm: "ONE RULE. ONE WAY TO FAIL" (max drawdown); optional self-set Tradovate DLL only
profit_target_usd: null
consistency_rule: { text: "Consistency: None", threshold_pct: null }
max_position: { minis: "10", micros: "50", scaling: null }
min_trading_days: null
trading_day_reset: { time: "17:00", tz_as_published: "CT" }   # session 5 pm CT - 4 pm CT; EOD drawdown at 16.00 CT; auto-flat 3:50 pm CT
copy_trading_allowed: conditional   # same person only; eval+Funded Sim 2-10 accts; Funded Live up to 5; no Live<->Eval/Sim copying
automated_trading_allowed: conditional   # own ATS on supported platforms; third-party-purchased bots forbidden; >200 trades/day forbidden
automation_rule_text: "To use an automated trading system, you must use one of the platforms we support: NinjaTrader, Tradovate, TradingView, Jigsaw and Quantower. TradeDay does not expose the Tradovate API to allow traders to connect automated trading systems directly. Algos and Bots purchased from a third party and employed by the trader are forbidden. If multiple users are making the same trades, then all the accounts will be shut down, and the users suspended."
vps_allowed: "no"   # "TradeDay does not allow the use of virtual private servers (VPS)"
personal_device_required: unknown   # device not mentioned; firm requires trading from the registered location/address (see §7)
status: verified
not_confirmed_fields: []
# profit_target n/a. Account access is paused at $10,000 gross profit (EOD run 4:30pm CT) for Funded Live review; profits above are removed.
# Min Days to Payout: 1 (homepage). Funded Sim cap: max 5 active Funded Sim accounts.
sources:
  - { field: drawdown.amount/max_position/consistency, url: "https://www.tradeday.com/", accessed: "2026-10-23" }
  - { field: drawdown.type (intraday regardless of eval), url: "https://tradeday.freshdesk.com/en/support/solutions/articles/103000008889-what-are-the-rules-for-funded-funded-sim-and-funded-live-traders-", accessed: "2026-10-23" }
  - { field: drawdown gross-profit basis, url: "https://tradeday.freshdesk.com/en/support/solutions/articles/103000405228-what-is-quick-pay-", accessed: "2026-10-23" }
  - { field: stops_trailing_at, url: "https://tradeday.freshdesk.com/en/support/solutions/articles/103000250682-funded-accounts-withdrawals-and-the-impact-of-drawdowns", accessed: "2026-10-23" }
  - { field: daily_loss_limit, url: "https://www.tradeday.com/how-it-works", accessed: "2026-10-23" }
  - { field: automated_trading_allowed/automation_rule_text, url: "https://tradeday.freshdesk.com/en/support/solutions/articles/103000085101-automated-algo-and-bot-trading", accessed: "2026-10-23" }
  - { field: automation (prohibited practices), url: "https://tradeday.freshdesk.com/en/support/solutions/articles/103000121031-prohibited-trade-practices", accessed: "2026-10-23" }
  - { field: copy_trading_allowed, url: "https://tradeday.freshdesk.com/en/support/solutions/articles/103000008833-can-i-have-multiple-accounts-can-i-use-copy-trader-", accessed: "2026-10-23" }
  - { field: max accounts / one broker, url: "https://tradeday.freshdesk.com/en/support/solutions/articles/103000354796-multiple-accounts", accessed: "2026-10-23" }
  - { field: vps_allowed, url: "https://tradeday.freshdesk.com/en/support/solutions/articles/103000295384-using-a-vpn-vps-or-ip-masking-add-on", accessed: "2026-10-23" }
  - { field: trading_day_reset, url: "https://tradeday.freshdesk.com/en/support/solutions/articles/103000008864-what-is-the-definition-of-a-trading-day-", accessed: "2026-10-23" }
  - { field: trading_day_reset (auto-flat), url: "https://tradeday.freshdesk.com/en/support/solutions/articles/103000018874-what-is-the-permitted-trading-times-guideline-", accessed: "2026-10-23" }
```

#### Quick Pay — funded — 150K
```yaml
firm: tradeday
plan: "Quick Pay"
phase: funded
size_label: "150K"
size_usd: "150000.00"
instruments: [MNQ, NQ]
platforms: [Tradovate, Rithmic]   # one broker per trader; NinjaTrader Desktop only via Tradovate/CQG (see §2)
drawdown: { amount_usd: "4500.00", type: trailing_intraday, stops_trailing_at: "For evaluations and Funded Sims, the Trailing Max Drawdown will trail until they reach the starting account balance, at which time the trailing drawdown will freeze and no longer trail. (TMD calculated on gross profits: current profits plus payouts.)" }
daily_loss_limit: { amount_usd: null, kind: none }   # firm: "ONE RULE. ONE WAY TO FAIL" (max drawdown); optional self-set Tradovate DLL only
profit_target_usd: null
consistency_rule: { text: "Consistency: None", threshold_pct: null }
max_position: { minis: "15", micros: "50", scaling: null }
min_trading_days: null
trading_day_reset: { time: "17:00", tz_as_published: "CT" }   # session 5 pm CT - 4 pm CT; EOD drawdown at 16.00 CT; auto-flat 3:50 pm CT
copy_trading_allowed: conditional   # same person only; eval+Funded Sim 2-10 accts; Funded Live up to 5; no Live<->Eval/Sim copying
automated_trading_allowed: conditional   # own ATS on supported platforms; third-party-purchased bots forbidden; >200 trades/day forbidden
automation_rule_text: "To use an automated trading system, you must use one of the platforms we support: NinjaTrader, Tradovate, TradingView, Jigsaw and Quantower. TradeDay does not expose the Tradovate API to allow traders to connect automated trading systems directly. Algos and Bots purchased from a third party and employed by the trader are forbidden. If multiple users are making the same trades, then all the accounts will be shut down, and the users suspended."
vps_allowed: "no"   # "TradeDay does not allow the use of virtual private servers (VPS)"
personal_device_required: unknown   # device not mentioned; firm requires trading from the registered location/address (see §7)
status: verified
not_confirmed_fields: []
# profit_target n/a. Account access is paused at $10,000 gross profit (EOD run 4:30pm CT) for Funded Live review; profits above are removed.
# Min Days to Payout: 1 (homepage). Funded Sim cap: max 5 active Funded Sim accounts.
sources:
  - { field: drawdown.amount/max_position/consistency, url: "https://www.tradeday.com/", accessed: "2026-10-23" }
  - { field: drawdown.type (intraday regardless of eval), url: "https://tradeday.freshdesk.com/en/support/solutions/articles/103000008889-what-are-the-rules-for-funded-funded-sim-and-funded-live-traders-", accessed: "2026-10-23" }
  - { field: drawdown gross-profit basis, url: "https://tradeday.freshdesk.com/en/support/solutions/articles/103000405228-what-is-quick-pay-", accessed: "2026-10-23" }
  - { field: stops_trailing_at, url: "https://tradeday.freshdesk.com/en/support/solutions/articles/103000250682-funded-accounts-withdrawals-and-the-impact-of-drawdowns", accessed: "2026-10-23" }
  - { field: daily_loss_limit, url: "https://www.tradeday.com/how-it-works", accessed: "2026-10-23" }
  - { field: automated_trading_allowed/automation_rule_text, url: "https://tradeday.freshdesk.com/en/support/solutions/articles/103000085101-automated-algo-and-bot-trading", accessed: "2026-10-23" }
  - { field: automation (prohibited practices), url: "https://tradeday.freshdesk.com/en/support/solutions/articles/103000121031-prohibited-trade-practices", accessed: "2026-10-23" }
  - { field: copy_trading_allowed, url: "https://tradeday.freshdesk.com/en/support/solutions/articles/103000008833-can-i-have-multiple-accounts-can-i-use-copy-trader-", accessed: "2026-10-23" }
  - { field: max accounts / one broker, url: "https://tradeday.freshdesk.com/en/support/solutions/articles/103000354796-multiple-accounts", accessed: "2026-10-23" }
  - { field: vps_allowed, url: "https://tradeday.freshdesk.com/en/support/solutions/articles/103000295384-using-a-vpn-vps-or-ip-masking-add-on", accessed: "2026-10-23" }
  - { field: trading_day_reset, url: "https://tradeday.freshdesk.com/en/support/solutions/articles/103000008864-what-is-the-definition-of-a-trading-day-", accessed: "2026-10-23" }
  - { field: trading_day_reset (auto-flat), url: "https://tradeday.freshdesk.com/en/support/solutions/articles/103000018874-what-is-the-permitted-trading-times-guideline-", accessed: "2026-10-23" }
```

#### Quick Pay — live — 25K
```yaml
firm: tradeday
plan: "Quick Pay"
phase: live
size_label: "25K"
size_usd: "25000.00"
instruments: [MNQ, NQ]
platforms: [Tradovate, Rithmic]   # one broker per trader; NinjaTrader Desktop only via Tradovate/CQG (see §2)
drawdown: { amount_usd: "0.00", type: other, stops_trailing_at: "\"The maximum drawdown limit will be set to $0.\" Plus slippage buffer: positions liquidated if balance falls below the slippage buffer." }   # published as $0 floor; not a USD loss allowance
daily_loss_limit: { amount_usd: null, kind: none }   # firm: "ONE RULE. ONE WAY TO FAIL" (max drawdown); optional self-set Tradovate DLL only
profit_target_usd: null
consistency_rule: { text: null, threshold_pct: null }
max_position: { minis: null, micros: null, scaling: "Balance brought over $0–3K: 2 minis/20 micros/$500 slippage; $3K–6K: 3/30/$750; $6K–10K: 4/40/$1,000" }
min_trading_days: null
trading_day_reset: { time: "17:00", tz_as_published: "CT" }   # session 5 pm CT - 4 pm CT; EOD drawdown at 16.00 CT; auto-flat 3:50 pm CT
copy_trading_allowed: conditional   # same person only; eval+Funded Sim 2-10 accts; Funded Live up to 5; no Live<->Eval/Sim copying
automated_trading_allowed: conditional   # own ATS on supported platforms; third-party-purchased bots forbidden; >200 trades/day forbidden
automation_rule_text: "To use an automated trading system, you must use one of the platforms we support: NinjaTrader, Tradovate, TradingView, Jigsaw and Quantower. TradeDay does not expose the Tradovate API to allow traders to connect automated trading systems directly. Algos and Bots purchased from a third party and employed by the trader are forbidden. If multiple users are making the same trades, then all the accounts will be shut down, and the users suspended."
vps_allowed: "no"   # "TradeDay does not allow the use of virtual private servers (VPS)"
personal_device_required: unknown   # device not mentioned; firm requires trading from the registered location/address (see §7)
status: conflict
not_confirmed_fields: [max_position, drawdown.slippage_buffer]
# Starting balance = profits transferred from Funded Sim (no additional drawdown added). Size label = original account tier.
# CONFLICT max_position: LIVE table sets limits by balance brought over; MULTI says 'The position limits for Funded Live will be the same as those for the Funded Sim that passed, but they may be reduced'.
# CONFLICT slippage buffer: SLIP gives by size $500.00 for 25K; LIVE gives by balance brought over ($500/$750/$1,000).
sources:
  - { field: drawdown ($0), url: "https://tradeday.freshdesk.com/en/support/solutions/articles/103000250682-funded-accounts-withdrawals-and-the-impact-of-drawdowns", accessed: "2026-10-23" }
  - { field: max_position/slippage by balance, url: "https://tradeday.freshdesk.com/en/support/solutions/articles/103000411652-quick-pay-and-fast-pass-funded-live-trading-rules", accessed: "2026-10-23" }
  - { field: slippage by size (conflict), url: "https://tradeday.freshdesk.com/en/support/solutions/articles/103000089243-slippage-rule-for-funded-live-traders", accessed: "2026-10-23" }
  - { field: position = Funded Sim limits (conflict), url: "https://tradeday.freshdesk.com/en/support/solutions/articles/103000354796-multiple-accounts", accessed: "2026-10-23" }
  - { field: daily_loss_limit, url: "https://www.tradeday.com/how-it-works", accessed: "2026-10-23" }
  - { field: automated_trading_allowed/automation_rule_text, url: "https://tradeday.freshdesk.com/en/support/solutions/articles/103000085101-automated-algo-and-bot-trading", accessed: "2026-10-23" }
  - { field: automation (prohibited practices), url: "https://tradeday.freshdesk.com/en/support/solutions/articles/103000121031-prohibited-trade-practices", accessed: "2026-10-23" }
  - { field: copy_trading_allowed, url: "https://tradeday.freshdesk.com/en/support/solutions/articles/103000008833-can-i-have-multiple-accounts-can-i-use-copy-trader-", accessed: "2026-10-23" }
  - { field: max accounts / one broker, url: "https://tradeday.freshdesk.com/en/support/solutions/articles/103000354796-multiple-accounts", accessed: "2026-10-23" }
  - { field: vps_allowed, url: "https://tradeday.freshdesk.com/en/support/solutions/articles/103000295384-using-a-vpn-vps-or-ip-masking-add-on", accessed: "2026-10-23" }
  - { field: trading_day_reset, url: "https://tradeday.freshdesk.com/en/support/solutions/articles/103000008864-what-is-the-definition-of-a-trading-day-", accessed: "2026-10-23" }
  - { field: trading_day_reset (auto-flat), url: "https://tradeday.freshdesk.com/en/support/solutions/articles/103000018874-what-is-the-permitted-trading-times-guideline-", accessed: "2026-10-23" }
```

#### Quick Pay — live — 50K
```yaml
firm: tradeday
plan: "Quick Pay"
phase: live
size_label: "50K"
size_usd: "50000.00"
instruments: [MNQ, NQ]
platforms: [Tradovate, Rithmic]   # one broker per trader; NinjaTrader Desktop only via Tradovate/CQG (see §2)
drawdown: { amount_usd: "0.00", type: other, stops_trailing_at: "\"The maximum drawdown limit will be set to $0.\" Plus slippage buffer: positions liquidated if balance falls below the slippage buffer." }   # published as $0 floor; not a USD loss allowance
daily_loss_limit: { amount_usd: null, kind: none }   # firm: "ONE RULE. ONE WAY TO FAIL" (max drawdown); optional self-set Tradovate DLL only
profit_target_usd: null
consistency_rule: { text: null, threshold_pct: null }
max_position: { minis: null, micros: null, scaling: "Balance brought over $0–3K: 2 minis/20 micros/$500 slippage; $3K–6K: 3/30/$750; $6K–10K: 4/40/$1,000" }
min_trading_days: null
trading_day_reset: { time: "17:00", tz_as_published: "CT" }   # session 5 pm CT - 4 pm CT; EOD drawdown at 16.00 CT; auto-flat 3:50 pm CT
copy_trading_allowed: conditional   # same person only; eval+Funded Sim 2-10 accts; Funded Live up to 5; no Live<->Eval/Sim copying
automated_trading_allowed: conditional   # own ATS on supported platforms; third-party-purchased bots forbidden; >200 trades/day forbidden
automation_rule_text: "To use an automated trading system, you must use one of the platforms we support: NinjaTrader, Tradovate, TradingView, Jigsaw and Quantower. TradeDay does not expose the Tradovate API to allow traders to connect automated trading systems directly. Algos and Bots purchased from a third party and employed by the trader are forbidden. If multiple users are making the same trades, then all the accounts will be shut down, and the users suspended."
vps_allowed: "no"   # "TradeDay does not allow the use of virtual private servers (VPS)"
personal_device_required: unknown   # device not mentioned; firm requires trading from the registered location/address (see §7)
status: conflict
not_confirmed_fields: [max_position, drawdown.slippage_buffer]
# Starting balance = profits transferred from Funded Sim (no additional drawdown added). Size label = original account tier.
# CONFLICT max_position: LIVE table sets limits by balance brought over; MULTI says 'The position limits for Funded Live will be the same as those for the Funded Sim that passed, but they may be reduced'.
# CONFLICT slippage buffer: SLIP gives by size $500.00 for 50K; LIVE gives by balance brought over ($500/$750/$1,000).
sources:
  - { field: drawdown ($0), url: "https://tradeday.freshdesk.com/en/support/solutions/articles/103000250682-funded-accounts-withdrawals-and-the-impact-of-drawdowns", accessed: "2026-10-23" }
  - { field: max_position/slippage by balance, url: "https://tradeday.freshdesk.com/en/support/solutions/articles/103000411652-quick-pay-and-fast-pass-funded-live-trading-rules", accessed: "2026-10-23" }
  - { field: slippage by size (conflict), url: "https://tradeday.freshdesk.com/en/support/solutions/articles/103000089243-slippage-rule-for-funded-live-traders", accessed: "2026-10-23" }
  - { field: position = Funded Sim limits (conflict), url: "https://tradeday.freshdesk.com/en/support/solutions/articles/103000354796-multiple-accounts", accessed: "2026-10-23" }
  - { field: daily_loss_limit, url: "https://www.tradeday.com/how-it-works", accessed: "2026-10-23" }
  - { field: automated_trading_allowed/automation_rule_text, url: "https://tradeday.freshdesk.com/en/support/solutions/articles/103000085101-automated-algo-and-bot-trading", accessed: "2026-10-23" }
  - { field: automation (prohibited practices), url: "https://tradeday.freshdesk.com/en/support/solutions/articles/103000121031-prohibited-trade-practices", accessed: "2026-10-23" }
  - { field: copy_trading_allowed, url: "https://tradeday.freshdesk.com/en/support/solutions/articles/103000008833-can-i-have-multiple-accounts-can-i-use-copy-trader-", accessed: "2026-10-23" }
  - { field: max accounts / one broker, url: "https://tradeday.freshdesk.com/en/support/solutions/articles/103000354796-multiple-accounts", accessed: "2026-10-23" }
  - { field: vps_allowed, url: "https://tradeday.freshdesk.com/en/support/solutions/articles/103000295384-using-a-vpn-vps-or-ip-masking-add-on", accessed: "2026-10-23" }
  - { field: trading_day_reset, url: "https://tradeday.freshdesk.com/en/support/solutions/articles/103000008864-what-is-the-definition-of-a-trading-day-", accessed: "2026-10-23" }
  - { field: trading_day_reset (auto-flat), url: "https://tradeday.freshdesk.com/en/support/solutions/articles/103000018874-what-is-the-permitted-trading-times-guideline-", accessed: "2026-10-23" }
```

#### Quick Pay — live — 100K
```yaml
firm: tradeday
plan: "Quick Pay"
phase: live
size_label: "100K"
size_usd: "100000.00"
instruments: [MNQ, NQ]
platforms: [Tradovate, Rithmic]   # one broker per trader; NinjaTrader Desktop only via Tradovate/CQG (see §2)
drawdown: { amount_usd: "0.00", type: other, stops_trailing_at: "\"The maximum drawdown limit will be set to $0.\" Plus slippage buffer: positions liquidated if balance falls below the slippage buffer." }   # published as $0 floor; not a USD loss allowance
daily_loss_limit: { amount_usd: null, kind: none }   # firm: "ONE RULE. ONE WAY TO FAIL" (max drawdown); optional self-set Tradovate DLL only
profit_target_usd: null
consistency_rule: { text: null, threshold_pct: null }
max_position: { minis: null, micros: null, scaling: "Balance brought over $0–3K: 2 minis/20 micros/$500 slippage; $3K–6K: 3/30/$750; $6K–10K: 4/40/$1,000" }
min_trading_days: null
trading_day_reset: { time: "17:00", tz_as_published: "CT" }   # session 5 pm CT - 4 pm CT; EOD drawdown at 16.00 CT; auto-flat 3:50 pm CT
copy_trading_allowed: conditional   # same person only; eval+Funded Sim 2-10 accts; Funded Live up to 5; no Live<->Eval/Sim copying
automated_trading_allowed: conditional   # own ATS on supported platforms; third-party-purchased bots forbidden; >200 trades/day forbidden
automation_rule_text: "To use an automated trading system, you must use one of the platforms we support: NinjaTrader, Tradovate, TradingView, Jigsaw and Quantower. TradeDay does not expose the Tradovate API to allow traders to connect automated trading systems directly. Algos and Bots purchased from a third party and employed by the trader are forbidden. If multiple users are making the same trades, then all the accounts will be shut down, and the users suspended."
vps_allowed: "no"   # "TradeDay does not allow the use of virtual private servers (VPS)"
personal_device_required: unknown   # device not mentioned; firm requires trading from the registered location/address (see §7)
status: conflict
not_confirmed_fields: [max_position, drawdown.slippage_buffer]
# Starting balance = profits transferred from Funded Sim (no additional drawdown added). Size label = original account tier.
# CONFLICT max_position: LIVE table sets limits by balance brought over; MULTI says 'The position limits for Funded Live will be the same as those for the Funded Sim that passed, but they may be reduced'.
# CONFLICT slippage buffer: SLIP gives by size $750.00 for 100K; LIVE gives by balance brought over ($500/$750/$1,000).
sources:
  - { field: drawdown ($0), url: "https://tradeday.freshdesk.com/en/support/solutions/articles/103000250682-funded-accounts-withdrawals-and-the-impact-of-drawdowns", accessed: "2026-10-23" }
  - { field: max_position/slippage by balance, url: "https://tradeday.freshdesk.com/en/support/solutions/articles/103000411652-quick-pay-and-fast-pass-funded-live-trading-rules", accessed: "2026-10-23" }
  - { field: slippage by size (conflict), url: "https://tradeday.freshdesk.com/en/support/solutions/articles/103000089243-slippage-rule-for-funded-live-traders", accessed: "2026-10-23" }
  - { field: position = Funded Sim limits (conflict), url: "https://tradeday.freshdesk.com/en/support/solutions/articles/103000354796-multiple-accounts", accessed: "2026-10-23" }
  - { field: daily_loss_limit, url: "https://www.tradeday.com/how-it-works", accessed: "2026-10-23" }
  - { field: automated_trading_allowed/automation_rule_text, url: "https://tradeday.freshdesk.com/en/support/solutions/articles/103000085101-automated-algo-and-bot-trading", accessed: "2026-10-23" }
  - { field: automation (prohibited practices), url: "https://tradeday.freshdesk.com/en/support/solutions/articles/103000121031-prohibited-trade-practices", accessed: "2026-10-23" }
  - { field: copy_trading_allowed, url: "https://tradeday.freshdesk.com/en/support/solutions/articles/103000008833-can-i-have-multiple-accounts-can-i-use-copy-trader-", accessed: "2026-10-23" }
  - { field: max accounts / one broker, url: "https://tradeday.freshdesk.com/en/support/solutions/articles/103000354796-multiple-accounts", accessed: "2026-10-23" }
  - { field: vps_allowed, url: "https://tradeday.freshdesk.com/en/support/solutions/articles/103000295384-using-a-vpn-vps-or-ip-masking-add-on", accessed: "2026-10-23" }
  - { field: trading_day_reset, url: "https://tradeday.freshdesk.com/en/support/solutions/articles/103000008864-what-is-the-definition-of-a-trading-day-", accessed: "2026-10-23" }
  - { field: trading_day_reset (auto-flat), url: "https://tradeday.freshdesk.com/en/support/solutions/articles/103000018874-what-is-the-permitted-trading-times-guideline-", accessed: "2026-10-23" }
```

#### Quick Pay — live — 150K
```yaml
firm: tradeday
plan: "Quick Pay"
phase: live
size_label: "150K"
size_usd: "150000.00"
instruments: [MNQ, NQ]
platforms: [Tradovate, Rithmic]   # one broker per trader; NinjaTrader Desktop only via Tradovate/CQG (see §2)
drawdown: { amount_usd: "0.00", type: other, stops_trailing_at: "\"The maximum drawdown limit will be set to $0.\" Plus slippage buffer: positions liquidated if balance falls below the slippage buffer." }   # published as $0 floor; not a USD loss allowance
daily_loss_limit: { amount_usd: null, kind: none }   # firm: "ONE RULE. ONE WAY TO FAIL" (max drawdown); optional self-set Tradovate DLL only
profit_target_usd: null
consistency_rule: { text: null, threshold_pct: null }
max_position: { minis: null, micros: null, scaling: "Balance brought over $0–3K: 2 minis/20 micros/$500 slippage; $3K–6K: 3/30/$750; $6K–10K: 4/40/$1,000" }
min_trading_days: null
trading_day_reset: { time: "17:00", tz_as_published: "CT" }   # session 5 pm CT - 4 pm CT; EOD drawdown at 16.00 CT; auto-flat 3:50 pm CT
copy_trading_allowed: conditional   # same person only; eval+Funded Sim 2-10 accts; Funded Live up to 5; no Live<->Eval/Sim copying
automated_trading_allowed: conditional   # own ATS on supported platforms; third-party-purchased bots forbidden; >200 trades/day forbidden
automation_rule_text: "To use an automated trading system, you must use one of the platforms we support: NinjaTrader, Tradovate, TradingView, Jigsaw and Quantower. TradeDay does not expose the Tradovate API to allow traders to connect automated trading systems directly. Algos and Bots purchased from a third party and employed by the trader are forbidden. If multiple users are making the same trades, then all the accounts will be shut down, and the users suspended."
vps_allowed: "no"   # "TradeDay does not allow the use of virtual private servers (VPS)"
personal_device_required: unknown   # device not mentioned; firm requires trading from the registered location/address (see §7)
status: conflict
not_confirmed_fields: [max_position, drawdown.slippage_buffer]
# Starting balance = profits transferred from Funded Sim (no additional drawdown added). Size label = original account tier.
# CONFLICT max_position: LIVE table sets limits by balance brought over; MULTI says 'The position limits for Funded Live will be the same as those for the Funded Sim that passed, but they may be reduced'.
# CONFLICT slippage buffer: SLIP gives by size $1000.00 for 150K; LIVE gives by balance brought over ($500/$750/$1,000).
sources:
  - { field: drawdown ($0), url: "https://tradeday.freshdesk.com/en/support/solutions/articles/103000250682-funded-accounts-withdrawals-and-the-impact-of-drawdowns", accessed: "2026-10-23" }
  - { field: max_position/slippage by balance, url: "https://tradeday.freshdesk.com/en/support/solutions/articles/103000411652-quick-pay-and-fast-pass-funded-live-trading-rules", accessed: "2026-10-23" }
  - { field: slippage by size (conflict), url: "https://tradeday.freshdesk.com/en/support/solutions/articles/103000089243-slippage-rule-for-funded-live-traders", accessed: "2026-10-23" }
  - { field: position = Funded Sim limits (conflict), url: "https://tradeday.freshdesk.com/en/support/solutions/articles/103000354796-multiple-accounts", accessed: "2026-10-23" }
  - { field: daily_loss_limit, url: "https://www.tradeday.com/how-it-works", accessed: "2026-10-23" }
  - { field: automated_trading_allowed/automation_rule_text, url: "https://tradeday.freshdesk.com/en/support/solutions/articles/103000085101-automated-algo-and-bot-trading", accessed: "2026-10-23" }
  - { field: automation (prohibited practices), url: "https://tradeday.freshdesk.com/en/support/solutions/articles/103000121031-prohibited-trade-practices", accessed: "2026-10-23" }
  - { field: copy_trading_allowed, url: "https://tradeday.freshdesk.com/en/support/solutions/articles/103000008833-can-i-have-multiple-accounts-can-i-use-copy-trader-", accessed: "2026-10-23" }
  - { field: max accounts / one broker, url: "https://tradeday.freshdesk.com/en/support/solutions/articles/103000354796-multiple-accounts", accessed: "2026-10-23" }
  - { field: vps_allowed, url: "https://tradeday.freshdesk.com/en/support/solutions/articles/103000295384-using-a-vpn-vps-or-ip-masking-add-on", accessed: "2026-10-23" }
  - { field: trading_day_reset, url: "https://tradeday.freshdesk.com/en/support/solutions/articles/103000008864-what-is-the-definition-of-a-trading-day-", accessed: "2026-10-23" }
  - { field: trading_day_reset (auto-flat), url: "https://tradeday.freshdesk.com/en/support/solutions/articles/103000018874-what-is-the-permitted-trading-times-guideline-", accessed: "2026-10-23" }
```

#### Fast Pass — End of Day — evaluation — 25K
```yaml
firm: tradeday
plan: "Fast Pass — End of Day"
phase: evaluation
size_label: "25K"
size_usd: "25000.00"
instruments: [MNQ, NQ]
platforms: [Tradovate, Rithmic]   # one broker per trader; NinjaTrader Desktop only via Tradovate/CQG (see §2)
drawdown: { amount_usd: "1000.00", type: trailing_eod, stops_trailing_at: "Trailing max drawdown limits continues to trail your account growth until they reach the starting account balance." }
daily_loss_limit: { amount_usd: null, kind: none }   # firm: "ONE RULE. ONE WAY TO FAIL" (max drawdown); optional self-set Tradovate DLL only
profit_target_usd: "1500.00"
consistency_rule: { text: "No day greater than 45% of your total profits.", threshold_pct: "45" }
max_position: { minis: "2", micros: "20", scaling: null }
min_trading_days: null   # CONFLICT: 3 (OBJ, DAYS, HOME cards) vs 'no minimum trading days' (FP, HOW, HOME FAQ) vs T&C 'minimum 7-day period'
trading_day_reset: { time: "17:00", tz_as_published: "CT" }   # session 5 pm CT - 4 pm CT; EOD drawdown at 16.00 CT; auto-flat 3:50 pm CT
copy_trading_allowed: conditional   # same person only; eval+Funded Sim 2-10 accts; Funded Live up to 5; no Live<->Eval/Sim copying
automated_trading_allowed: conditional   # own ATS on supported platforms; third-party-purchased bots forbidden; >200 trades/day forbidden
automation_rule_text: "To use an automated trading system, you must use one of the platforms we support: NinjaTrader, Tradovate, TradingView, Jigsaw and Quantower. TradeDay does not expose the Tradovate API to allow traders to connect automated trading systems directly. Algos and Bots purchased from a third party and employed by the trader are forbidden. If multiple users are making the same trades, then all the accounts will be shut down, and the users suspended."
vps_allowed: "no"   # "TradeDay does not allow the use of virtual private servers (VPS)"
personal_device_required: unknown   # device not mentioned; firm requires trading from the registered location/address (see §7)
status: conflict
not_confirmed_fields: [min_trading_days]
sources:
  - { field: size/drawdown/profit_target/max_position/min_days(3), url: "https://www.tradeday.com/", accessed: "2026-10-23" }
  - { field: size/drawdown/profit_target/max_position, url: "https://tradeday.freshdesk.com/en/support/solutions/articles/103000304248-what-are-the-differences-between-each-account-size-", accessed: "2026-10-23" }
  - { field: drawdown.type (all EOD), url: "https://tradeday.freshdesk.com/en/support/solutions/articles/103000008847-what-are-the-objectives-and-rules-of-the-quick-pay-and-fast-pass-evaluation-", accessed: "2026-10-23" }
  - { field: stops_trailing_at, url: "https://tradeday.freshdesk.com/en/support/solutions/articles/103000008855-what-is-the-maximum-drawdown-rule-", accessed: "2026-10-23" }
  - { field: consistency_rule, url: "https://tradeday.freshdesk.com/en/support/solutions/articles/103000008849-what-is-the-consistency-objective-", accessed: "2026-10-23" }
  - { field: min_trading_days (3), url: "https://tradeday.freshdesk.com/en/support/solutions/articles/103000008850-what-is-the-trade-for-a-minimum-number-of-days-objective-", accessed: "2026-10-23" }
  - { field: min_trading_days (none), url: "https://tradeday.freshdesk.com/en/support/solutions/articles/103000405229-what-is-fast-pass-", accessed: "2026-10-23" }
  - { field: min_trading_days (none), url: "https://www.tradeday.com/how-it-works", accessed: "2026-10-23" }
  - { field: min_trading_days (7-day, generic), url: "https://www.tradeday.com/terms-and-conditions", accessed: "2026-10-23" }
  - { field: daily_loss_limit, url: "https://www.tradeday.com/how-it-works", accessed: "2026-10-23" }
  - { field: max_position.micros (50 cap), url: "https://tradeday.freshdesk.com/en/support/solutions/articles/103000008891-what-are-the-position-limits-guidelines-and-contracts-vs-micros-", accessed: "2026-10-23" }
  - { field: automated_trading_allowed/automation_rule_text, url: "https://tradeday.freshdesk.com/en/support/solutions/articles/103000085101-automated-algo-and-bot-trading", accessed: "2026-10-23" }
  - { field: automation (prohibited practices), url: "https://tradeday.freshdesk.com/en/support/solutions/articles/103000121031-prohibited-trade-practices", accessed: "2026-10-23" }
  - { field: copy_trading_allowed, url: "https://tradeday.freshdesk.com/en/support/solutions/articles/103000008833-can-i-have-multiple-accounts-can-i-use-copy-trader-", accessed: "2026-10-23" }
  - { field: max accounts / one broker, url: "https://tradeday.freshdesk.com/en/support/solutions/articles/103000354796-multiple-accounts", accessed: "2026-10-23" }
  - { field: vps_allowed, url: "https://tradeday.freshdesk.com/en/support/solutions/articles/103000295384-using-a-vpn-vps-or-ip-masking-add-on", accessed: "2026-10-23" }
  - { field: trading_day_reset, url: "https://tradeday.freshdesk.com/en/support/solutions/articles/103000008864-what-is-the-definition-of-a-trading-day-", accessed: "2026-10-23" }
  - { field: trading_day_reset (auto-flat), url: "https://tradeday.freshdesk.com/en/support/solutions/articles/103000018874-what-is-the-permitted-trading-times-guideline-", accessed: "2026-10-23" }
```

#### Fast Pass — End of Day — evaluation — 50K
```yaml
firm: tradeday
plan: "Fast Pass — End of Day"
phase: evaluation
size_label: "50K"
size_usd: "50000.00"
instruments: [MNQ, NQ]
platforms: [Tradovate, Rithmic]   # one broker per trader; NinjaTrader Desktop only via Tradovate/CQG (see §2)
drawdown: { amount_usd: "2000.00", type: trailing_eod, stops_trailing_at: "Trailing max drawdown limits continues to trail your account growth until they reach the starting account balance." }
daily_loss_limit: { amount_usd: null, kind: none }   # firm: "ONE RULE. ONE WAY TO FAIL" (max drawdown); optional self-set Tradovate DLL only
profit_target_usd: "3000.00"
consistency_rule: { text: "No day greater than 45% of your total profits.", threshold_pct: "45" }
max_position: { minis: "5", micros: "50", scaling: null }
min_trading_days: null   # CONFLICT: 3 (OBJ, DAYS, HOME cards) vs 'no minimum trading days' (FP, HOW, HOME FAQ) vs T&C 'minimum 7-day period'
trading_day_reset: { time: "17:00", tz_as_published: "CT" }   # session 5 pm CT - 4 pm CT; EOD drawdown at 16.00 CT; auto-flat 3:50 pm CT
copy_trading_allowed: conditional   # same person only; eval+Funded Sim 2-10 accts; Funded Live up to 5; no Live<->Eval/Sim copying
automated_trading_allowed: conditional   # own ATS on supported platforms; third-party-purchased bots forbidden; >200 trades/day forbidden
automation_rule_text: "To use an automated trading system, you must use one of the platforms we support: NinjaTrader, Tradovate, TradingView, Jigsaw and Quantower. TradeDay does not expose the Tradovate API to allow traders to connect automated trading systems directly. Algos and Bots purchased from a third party and employed by the trader are forbidden. If multiple users are making the same trades, then all the accounts will be shut down, and the users suspended."
vps_allowed: "no"   # "TradeDay does not allow the use of virtual private servers (VPS)"
personal_device_required: unknown   # device not mentioned; firm requires trading from the registered location/address (see §7)
status: conflict
not_confirmed_fields: [min_trading_days]
sources:
  - { field: size/drawdown/profit_target/max_position/min_days(3), url: "https://www.tradeday.com/", accessed: "2026-10-23" }
  - { field: size/drawdown/profit_target/max_position, url: "https://tradeday.freshdesk.com/en/support/solutions/articles/103000304248-what-are-the-differences-between-each-account-size-", accessed: "2026-10-23" }
  - { field: drawdown.type (all EOD), url: "https://tradeday.freshdesk.com/en/support/solutions/articles/103000008847-what-are-the-objectives-and-rules-of-the-quick-pay-and-fast-pass-evaluation-", accessed: "2026-10-23" }
  - { field: stops_trailing_at, url: "https://tradeday.freshdesk.com/en/support/solutions/articles/103000008855-what-is-the-maximum-drawdown-rule-", accessed: "2026-10-23" }
  - { field: consistency_rule, url: "https://tradeday.freshdesk.com/en/support/solutions/articles/103000008849-what-is-the-consistency-objective-", accessed: "2026-10-23" }
  - { field: min_trading_days (3), url: "https://tradeday.freshdesk.com/en/support/solutions/articles/103000008850-what-is-the-trade-for-a-minimum-number-of-days-objective-", accessed: "2026-10-23" }
  - { field: min_trading_days (none), url: "https://tradeday.freshdesk.com/en/support/solutions/articles/103000405229-what-is-fast-pass-", accessed: "2026-10-23" }
  - { field: min_trading_days (none), url: "https://www.tradeday.com/how-it-works", accessed: "2026-10-23" }
  - { field: min_trading_days (7-day, generic), url: "https://www.tradeday.com/terms-and-conditions", accessed: "2026-10-23" }
  - { field: daily_loss_limit, url: "https://www.tradeday.com/how-it-works", accessed: "2026-10-23" }
  - { field: max_position.micros (50 cap), url: "https://tradeday.freshdesk.com/en/support/solutions/articles/103000008891-what-are-the-position-limits-guidelines-and-contracts-vs-micros-", accessed: "2026-10-23" }
  - { field: automated_trading_allowed/automation_rule_text, url: "https://tradeday.freshdesk.com/en/support/solutions/articles/103000085101-automated-algo-and-bot-trading", accessed: "2026-10-23" }
  - { field: automation (prohibited practices), url: "https://tradeday.freshdesk.com/en/support/solutions/articles/103000121031-prohibited-trade-practices", accessed: "2026-10-23" }
  - { field: copy_trading_allowed, url: "https://tradeday.freshdesk.com/en/support/solutions/articles/103000008833-can-i-have-multiple-accounts-can-i-use-copy-trader-", accessed: "2026-10-23" }
  - { field: max accounts / one broker, url: "https://tradeday.freshdesk.com/en/support/solutions/articles/103000354796-multiple-accounts", accessed: "2026-10-23" }
  - { field: vps_allowed, url: "https://tradeday.freshdesk.com/en/support/solutions/articles/103000295384-using-a-vpn-vps-or-ip-masking-add-on", accessed: "2026-10-23" }
  - { field: trading_day_reset, url: "https://tradeday.freshdesk.com/en/support/solutions/articles/103000008864-what-is-the-definition-of-a-trading-day-", accessed: "2026-10-23" }
  - { field: trading_day_reset (auto-flat), url: "https://tradeday.freshdesk.com/en/support/solutions/articles/103000018874-what-is-the-permitted-trading-times-guideline-", accessed: "2026-10-23" }
```

#### Fast Pass — End of Day — evaluation — 100K
```yaml
firm: tradeday
plan: "Fast Pass — End of Day"
phase: evaluation
size_label: "100K"
size_usd: "100000.00"
instruments: [MNQ, NQ]
platforms: [Tradovate, Rithmic]   # one broker per trader; NinjaTrader Desktop only via Tradovate/CQG (see §2)
drawdown: { amount_usd: "3000.00", type: trailing_eod, stops_trailing_at: "Trailing max drawdown limits continues to trail your account growth until they reach the starting account balance." }
daily_loss_limit: { amount_usd: null, kind: none }   # firm: "ONE RULE. ONE WAY TO FAIL" (max drawdown); optional self-set Tradovate DLL only
profit_target_usd: "6000.00"
consistency_rule: { text: "No day greater than 45% of your total profits.", threshold_pct: "45" }
max_position: { minis: "10", micros: "50", scaling: null }
min_trading_days: null   # CONFLICT: 3 (OBJ, DAYS, HOME cards) vs 'no minimum trading days' (FP, HOW, HOME FAQ) vs T&C 'minimum 7-day period'
trading_day_reset: { time: "17:00", tz_as_published: "CT" }   # session 5 pm CT - 4 pm CT; EOD drawdown at 16.00 CT; auto-flat 3:50 pm CT
copy_trading_allowed: conditional   # same person only; eval+Funded Sim 2-10 accts; Funded Live up to 5; no Live<->Eval/Sim copying
automated_trading_allowed: conditional   # own ATS on supported platforms; third-party-purchased bots forbidden; >200 trades/day forbidden
automation_rule_text: "To use an automated trading system, you must use one of the platforms we support: NinjaTrader, Tradovate, TradingView, Jigsaw and Quantower. TradeDay does not expose the Tradovate API to allow traders to connect automated trading systems directly. Algos and Bots purchased from a third party and employed by the trader are forbidden. If multiple users are making the same trades, then all the accounts will be shut down, and the users suspended."
vps_allowed: "no"   # "TradeDay does not allow the use of virtual private servers (VPS)"
personal_device_required: unknown   # device not mentioned; firm requires trading from the registered location/address (see §7)
status: conflict
not_confirmed_fields: [min_trading_days]
sources:
  - { field: size/drawdown/profit_target/max_position/min_days(3), url: "https://www.tradeday.com/", accessed: "2026-10-23" }
  - { field: size/drawdown/profit_target/max_position, url: "https://tradeday.freshdesk.com/en/support/solutions/articles/103000304248-what-are-the-differences-between-each-account-size-", accessed: "2026-10-23" }
  - { field: drawdown.type (all EOD), url: "https://tradeday.freshdesk.com/en/support/solutions/articles/103000008847-what-are-the-objectives-and-rules-of-the-quick-pay-and-fast-pass-evaluation-", accessed: "2026-10-23" }
  - { field: stops_trailing_at, url: "https://tradeday.freshdesk.com/en/support/solutions/articles/103000008855-what-is-the-maximum-drawdown-rule-", accessed: "2026-10-23" }
  - { field: consistency_rule, url: "https://tradeday.freshdesk.com/en/support/solutions/articles/103000008849-what-is-the-consistency-objective-", accessed: "2026-10-23" }
  - { field: min_trading_days (3), url: "https://tradeday.freshdesk.com/en/support/solutions/articles/103000008850-what-is-the-trade-for-a-minimum-number-of-days-objective-", accessed: "2026-10-23" }
  - { field: min_trading_days (none), url: "https://tradeday.freshdesk.com/en/support/solutions/articles/103000405229-what-is-fast-pass-", accessed: "2026-10-23" }
  - { field: min_trading_days (none), url: "https://www.tradeday.com/how-it-works", accessed: "2026-10-23" }
  - { field: min_trading_days (7-day, generic), url: "https://www.tradeday.com/terms-and-conditions", accessed: "2026-10-23" }
  - { field: daily_loss_limit, url: "https://www.tradeday.com/how-it-works", accessed: "2026-10-23" }
  - { field: max_position.micros (50 cap), url: "https://tradeday.freshdesk.com/en/support/solutions/articles/103000008891-what-are-the-position-limits-guidelines-and-contracts-vs-micros-", accessed: "2026-10-23" }
  - { field: automated_trading_allowed/automation_rule_text, url: "https://tradeday.freshdesk.com/en/support/solutions/articles/103000085101-automated-algo-and-bot-trading", accessed: "2026-10-23" }
  - { field: automation (prohibited practices), url: "https://tradeday.freshdesk.com/en/support/solutions/articles/103000121031-prohibited-trade-practices", accessed: "2026-10-23" }
  - { field: copy_trading_allowed, url: "https://tradeday.freshdesk.com/en/support/solutions/articles/103000008833-can-i-have-multiple-accounts-can-i-use-copy-trader-", accessed: "2026-10-23" }
  - { field: max accounts / one broker, url: "https://tradeday.freshdesk.com/en/support/solutions/articles/103000354796-multiple-accounts", accessed: "2026-10-23" }
  - { field: vps_allowed, url: "https://tradeday.freshdesk.com/en/support/solutions/articles/103000295384-using-a-vpn-vps-or-ip-masking-add-on", accessed: "2026-10-23" }
  - { field: trading_day_reset, url: "https://tradeday.freshdesk.com/en/support/solutions/articles/103000008864-what-is-the-definition-of-a-trading-day-", accessed: "2026-10-23" }
  - { field: trading_day_reset (auto-flat), url: "https://tradeday.freshdesk.com/en/support/solutions/articles/103000018874-what-is-the-permitted-trading-times-guideline-", accessed: "2026-10-23" }
```

#### Fast Pass — End of Day — evaluation — 150K
```yaml
firm: tradeday
plan: "Fast Pass — End of Day"
phase: evaluation
size_label: "150K"
size_usd: "150000.00"
instruments: [MNQ, NQ]
platforms: [Tradovate, Rithmic]   # one broker per trader; NinjaTrader Desktop only via Tradovate/CQG (see §2)
drawdown: { amount_usd: "4500.00", type: trailing_eod, stops_trailing_at: "Trailing max drawdown limits continues to trail your account growth until they reach the starting account balance." }
daily_loss_limit: { amount_usd: null, kind: none }   # firm: "ONE RULE. ONE WAY TO FAIL" (max drawdown); optional self-set Tradovate DLL only
profit_target_usd: "9000.00"
consistency_rule: { text: "No day greater than 45% of your total profits.", threshold_pct: "45" }
max_position: { minis: "15", micros: "50", scaling: null }
min_trading_days: null   # CONFLICT: 3 (OBJ, DAYS, HOME cards) vs 'no minimum trading days' (FP, HOW, HOME FAQ) vs T&C 'minimum 7-day period'
trading_day_reset: { time: "17:00", tz_as_published: "CT" }   # session 5 pm CT - 4 pm CT; EOD drawdown at 16.00 CT; auto-flat 3:50 pm CT
copy_trading_allowed: conditional   # same person only; eval+Funded Sim 2-10 accts; Funded Live up to 5; no Live<->Eval/Sim copying
automated_trading_allowed: conditional   # own ATS on supported platforms; third-party-purchased bots forbidden; >200 trades/day forbidden
automation_rule_text: "To use an automated trading system, you must use one of the platforms we support: NinjaTrader, Tradovate, TradingView, Jigsaw and Quantower. TradeDay does not expose the Tradovate API to allow traders to connect automated trading systems directly. Algos and Bots purchased from a third party and employed by the trader are forbidden. If multiple users are making the same trades, then all the accounts will be shut down, and the users suspended."
vps_allowed: "no"   # "TradeDay does not allow the use of virtual private servers (VPS)"
personal_device_required: unknown   # device not mentioned; firm requires trading from the registered location/address (see §7)
status: conflict
not_confirmed_fields: [min_trading_days]
sources:
  - { field: size/drawdown/profit_target/max_position/min_days(3), url: "https://www.tradeday.com/", accessed: "2026-10-23" }
  - { field: size/drawdown/profit_target/max_position, url: "https://tradeday.freshdesk.com/en/support/solutions/articles/103000304248-what-are-the-differences-between-each-account-size-", accessed: "2026-10-23" }
  - { field: drawdown.type (all EOD), url: "https://tradeday.freshdesk.com/en/support/solutions/articles/103000008847-what-are-the-objectives-and-rules-of-the-quick-pay-and-fast-pass-evaluation-", accessed: "2026-10-23" }
  - { field: stops_trailing_at, url: "https://tradeday.freshdesk.com/en/support/solutions/articles/103000008855-what-is-the-maximum-drawdown-rule-", accessed: "2026-10-23" }
  - { field: consistency_rule, url: "https://tradeday.freshdesk.com/en/support/solutions/articles/103000008849-what-is-the-consistency-objective-", accessed: "2026-10-23" }
  - { field: min_trading_days (3), url: "https://tradeday.freshdesk.com/en/support/solutions/articles/103000008850-what-is-the-trade-for-a-minimum-number-of-days-objective-", accessed: "2026-10-23" }
  - { field: min_trading_days (none), url: "https://tradeday.freshdesk.com/en/support/solutions/articles/103000405229-what-is-fast-pass-", accessed: "2026-10-23" }
  - { field: min_trading_days (none), url: "https://www.tradeday.com/how-it-works", accessed: "2026-10-23" }
  - { field: min_trading_days (7-day, generic), url: "https://www.tradeday.com/terms-and-conditions", accessed: "2026-10-23" }
  - { field: daily_loss_limit, url: "https://www.tradeday.com/how-it-works", accessed: "2026-10-23" }
  - { field: max_position.micros (50 cap), url: "https://tradeday.freshdesk.com/en/support/solutions/articles/103000008891-what-are-the-position-limits-guidelines-and-contracts-vs-micros-", accessed: "2026-10-23" }
  - { field: automated_trading_allowed/automation_rule_text, url: "https://tradeday.freshdesk.com/en/support/solutions/articles/103000085101-automated-algo-and-bot-trading", accessed: "2026-10-23" }
  - { field: automation (prohibited practices), url: "https://tradeday.freshdesk.com/en/support/solutions/articles/103000121031-prohibited-trade-practices", accessed: "2026-10-23" }
  - { field: copy_trading_allowed, url: "https://tradeday.freshdesk.com/en/support/solutions/articles/103000008833-can-i-have-multiple-accounts-can-i-use-copy-trader-", accessed: "2026-10-23" }
  - { field: max accounts / one broker, url: "https://tradeday.freshdesk.com/en/support/solutions/articles/103000354796-multiple-accounts", accessed: "2026-10-23" }
  - { field: vps_allowed, url: "https://tradeday.freshdesk.com/en/support/solutions/articles/103000295384-using-a-vpn-vps-or-ip-masking-add-on", accessed: "2026-10-23" }
  - { field: trading_day_reset, url: "https://tradeday.freshdesk.com/en/support/solutions/articles/103000008864-what-is-the-definition-of-a-trading-day-", accessed: "2026-10-23" }
  - { field: trading_day_reset (auto-flat), url: "https://tradeday.freshdesk.com/en/support/solutions/articles/103000018874-what-is-the-permitted-trading-times-guideline-", accessed: "2026-10-23" }
```

#### Fast Pass — funded — 25K
```yaml
firm: tradeday
plan: "Fast Pass"
phase: funded
size_label: "25K"
size_usd: "25000.00"
instruments: [MNQ, NQ]
platforms: [Tradovate, Rithmic]   # one broker per trader; NinjaTrader Desktop only via Tradovate/CQG (see §2)
drawdown: { amount_usd: "1000.00", type: trailing_eod, stops_trailing_at: "Trails until it reaches the starting account balance (WDD). \"On the first payout request, if the Trailing Maximum Drawdown (TMD) has not yet reached the starting account balance, it will automatically move up to the starting account balance and remain fixed there moving forward.\"" }
daily_loss_limit: { amount_usd: null, kind: none }   # firm: "ONE RULE. ONE WAY TO FAIL" (max drawdown); optional self-set Tradovate DLL only
profit_target_usd: null
consistency_rule: { text: "For accounts opened on and after July 26 2026, accounts must satisfy the consistency objective: no single day can make up more than 45% of your total gross profits (gross profits being current profits + payouts)", threshold_pct: "45" }   # payout condition; accounts opened before July 26 2026: none (S-FPPAY)
max_position: { minis: "1", micros: "10", scaling: "scales +1 contract per $2k profit at EOD, max 2 minis" }
min_trading_days: null
trading_day_reset: { time: "17:00", tz_as_published: "CT" }   # session 5 pm CT - 4 pm CT; EOD drawdown at 16.00 CT; auto-flat 3:50 pm CT
copy_trading_allowed: conditional   # same person only; eval+Funded Sim 2-10 accts; Funded Live up to 5; no Live<->Eval/Sim copying
automated_trading_allowed: conditional   # own ATS on supported platforms; third-party-purchased bots forbidden; >200 trades/day forbidden
automation_rule_text: "To use an automated trading system, you must use one of the platforms we support: NinjaTrader, Tradovate, TradingView, Jigsaw and Quantower. TradeDay does not expose the Tradovate API to allow traders to connect automated trading systems directly. Algos and Bots purchased from a third party and employed by the trader are forbidden. If multiple users are making the same trades, then all the accounts will be shut down, and the users suspended."
vps_allowed: "no"   # "TradeDay does not allow the use of virtual private servers (VPS)"
personal_device_required: unknown   # device not mentioned; firm requires trading from the registered location/address (see §7)
status: partial
not_confirmed_fields: []
# Payout requires 5 profitable days of at least $100 (FPPAY). Do not trade between payout request and receipt (FPPAY).
# GAP: scaling ceiling not published except 25K ('max 2 minis'); micros after scaling not published.
# Moved to Funded Live on the 5th payout request.
sources:
  - { field: drawdown/consistency/max_position/scaling, url: "https://www.tradeday.com/", accessed: "2026-10-23" }
  - { field: max_position/scaling, url: "https://tradeday.freshdesk.com/en/support/solutions/articles/103000405229-what-is-fast-pass-", accessed: "2026-10-23" }
  - { field: drawdown.type (EOD), url: "https://tradeday.freshdesk.com/en/support/solutions/articles/103000008889-what-are-the-rules-for-funded-funded-sim-and-funded-live-traders-", accessed: "2026-10-23" }
  - { field: stops_trailing_at/payout conditions, url: "https://tradeday.freshdesk.com/en/support/solutions/articles/103000404096-fast-pass-funded-sim-payout-policy", accessed: "2026-10-23" }
  - { field: stops_trailing_at, url: "https://tradeday.freshdesk.com/en/support/solutions/articles/103000250682-funded-accounts-withdrawals-and-the-impact-of-drawdowns", accessed: "2026-10-23" }
  - { field: daily_loss_limit, url: "https://www.tradeday.com/how-it-works", accessed: "2026-10-23" }
  - { field: automated_trading_allowed/automation_rule_text, url: "https://tradeday.freshdesk.com/en/support/solutions/articles/103000085101-automated-algo-and-bot-trading", accessed: "2026-10-23" }
  - { field: automation (prohibited practices), url: "https://tradeday.freshdesk.com/en/support/solutions/articles/103000121031-prohibited-trade-practices", accessed: "2026-10-23" }
  - { field: copy_trading_allowed, url: "https://tradeday.freshdesk.com/en/support/solutions/articles/103000008833-can-i-have-multiple-accounts-can-i-use-copy-trader-", accessed: "2026-10-23" }
  - { field: max accounts / one broker, url: "https://tradeday.freshdesk.com/en/support/solutions/articles/103000354796-multiple-accounts", accessed: "2026-10-23" }
  - { field: vps_allowed, url: "https://tradeday.freshdesk.com/en/support/solutions/articles/103000295384-using-a-vpn-vps-or-ip-masking-add-on", accessed: "2026-10-23" }
  - { field: trading_day_reset, url: "https://tradeday.freshdesk.com/en/support/solutions/articles/103000008864-what-is-the-definition-of-a-trading-day-", accessed: "2026-10-23" }
  - { field: trading_day_reset (auto-flat), url: "https://tradeday.freshdesk.com/en/support/solutions/articles/103000018874-what-is-the-permitted-trading-times-guideline-", accessed: "2026-10-23" }
```

#### Fast Pass — funded — 50K
```yaml
firm: tradeday
plan: "Fast Pass"
phase: funded
size_label: "50K"
size_usd: "50000.00"
instruments: [MNQ, NQ]
platforms: [Tradovate, Rithmic]   # one broker per trader; NinjaTrader Desktop only via Tradovate/CQG (see §2)
drawdown: { amount_usd: "2000.00", type: trailing_eod, stops_trailing_at: "Trails until it reaches the starting account balance (WDD). \"On the first payout request, if the Trailing Maximum Drawdown (TMD) has not yet reached the starting account balance, it will automatically move up to the starting account balance and remain fixed there moving forward.\"" }
daily_loss_limit: { amount_usd: null, kind: none }   # firm: "ONE RULE. ONE WAY TO FAIL" (max drawdown); optional self-set Tradovate DLL only
profit_target_usd: null
consistency_rule: { text: "For accounts opened on and after July 26 2026, accounts must satisfy the consistency objective: no single day can make up more than 45% of your total gross profits (gross profits being current profits + payouts)", threshold_pct: "45" }   # payout condition; accounts opened before July 26 2026: none (S-FPPAY)
max_position: { minis: "2", micros: "20", scaling: "scales +1 contract per $2k profit at EOD" }
min_trading_days: null
trading_day_reset: { time: "17:00", tz_as_published: "CT" }   # session 5 pm CT - 4 pm CT; EOD drawdown at 16.00 CT; auto-flat 3:50 pm CT
copy_trading_allowed: conditional   # same person only; eval+Funded Sim 2-10 accts; Funded Live up to 5; no Live<->Eval/Sim copying
automated_trading_allowed: conditional   # own ATS on supported platforms; third-party-purchased bots forbidden; >200 trades/day forbidden
automation_rule_text: "To use an automated trading system, you must use one of the platforms we support: NinjaTrader, Tradovate, TradingView, Jigsaw and Quantower. TradeDay does not expose the Tradovate API to allow traders to connect automated trading systems directly. Algos and Bots purchased from a third party and employed by the trader are forbidden. If multiple users are making the same trades, then all the accounts will be shut down, and the users suspended."
vps_allowed: "no"   # "TradeDay does not allow the use of virtual private servers (VPS)"
personal_device_required: unknown   # device not mentioned; firm requires trading from the registered location/address (see §7)
status: partial
not_confirmed_fields: []
# Payout requires 5 profitable days of at least $150 (FPPAY). Do not trade between payout request and receipt (FPPAY).
# GAP: scaling ceiling not published except 25K ('max 2 minis'); micros after scaling not published.
# Moved to Funded Live on the 5th payout request.
sources:
  - { field: drawdown/consistency/max_position/scaling, url: "https://www.tradeday.com/", accessed: "2026-10-23" }
  - { field: max_position/scaling, url: "https://tradeday.freshdesk.com/en/support/solutions/articles/103000405229-what-is-fast-pass-", accessed: "2026-10-23" }
  - { field: drawdown.type (EOD), url: "https://tradeday.freshdesk.com/en/support/solutions/articles/103000008889-what-are-the-rules-for-funded-funded-sim-and-funded-live-traders-", accessed: "2026-10-23" }
  - { field: stops_trailing_at/payout conditions, url: "https://tradeday.freshdesk.com/en/support/solutions/articles/103000404096-fast-pass-funded-sim-payout-policy", accessed: "2026-10-23" }
  - { field: stops_trailing_at, url: "https://tradeday.freshdesk.com/en/support/solutions/articles/103000250682-funded-accounts-withdrawals-and-the-impact-of-drawdowns", accessed: "2026-10-23" }
  - { field: daily_loss_limit, url: "https://www.tradeday.com/how-it-works", accessed: "2026-10-23" }
  - { field: automated_trading_allowed/automation_rule_text, url: "https://tradeday.freshdesk.com/en/support/solutions/articles/103000085101-automated-algo-and-bot-trading", accessed: "2026-10-23" }
  - { field: automation (prohibited practices), url: "https://tradeday.freshdesk.com/en/support/solutions/articles/103000121031-prohibited-trade-practices", accessed: "2026-10-23" }
  - { field: copy_trading_allowed, url: "https://tradeday.freshdesk.com/en/support/solutions/articles/103000008833-can-i-have-multiple-accounts-can-i-use-copy-trader-", accessed: "2026-10-23" }
  - { field: max accounts / one broker, url: "https://tradeday.freshdesk.com/en/support/solutions/articles/103000354796-multiple-accounts", accessed: "2026-10-23" }
  - { field: vps_allowed, url: "https://tradeday.freshdesk.com/en/support/solutions/articles/103000295384-using-a-vpn-vps-or-ip-masking-add-on", accessed: "2026-10-23" }
  - { field: trading_day_reset, url: "https://tradeday.freshdesk.com/en/support/solutions/articles/103000008864-what-is-the-definition-of-a-trading-day-", accessed: "2026-10-23" }
  - { field: trading_day_reset (auto-flat), url: "https://tradeday.freshdesk.com/en/support/solutions/articles/103000018874-what-is-the-permitted-trading-times-guideline-", accessed: "2026-10-23" }
```

#### Fast Pass — funded — 100K
```yaml
firm: tradeday
plan: "Fast Pass"
phase: funded
size_label: "100K"
size_usd: "100000.00"
instruments: [MNQ, NQ]
platforms: [Tradovate, Rithmic]   # one broker per trader; NinjaTrader Desktop only via Tradovate/CQG (see §2)
drawdown: { amount_usd: "3000.00", type: trailing_eod, stops_trailing_at: "Trails until it reaches the starting account balance (WDD). \"On the first payout request, if the Trailing Maximum Drawdown (TMD) has not yet reached the starting account balance, it will automatically move up to the starting account balance and remain fixed there moving forward.\"" }
daily_loss_limit: { amount_usd: null, kind: none }   # firm: "ONE RULE. ONE WAY TO FAIL" (max drawdown); optional self-set Tradovate DLL only
profit_target_usd: null
consistency_rule: { text: "For accounts opened on and after July 26 2026, accounts must satisfy the consistency objective: no single day can make up more than 45% of your total gross profits (gross profits being current profits + payouts)", threshold_pct: "45" }   # payout condition; accounts opened before July 26 2026: none (S-FPPAY)
max_position: { minis: "3", micros: "30", scaling: "scales +1 contract per $2k profit at EOD" }
min_trading_days: null
trading_day_reset: { time: "17:00", tz_as_published: "CT" }   # session 5 pm CT - 4 pm CT; EOD drawdown at 16.00 CT; auto-flat 3:50 pm CT
copy_trading_allowed: conditional   # same person only; eval+Funded Sim 2-10 accts; Funded Live up to 5; no Live<->Eval/Sim copying
automated_trading_allowed: conditional   # own ATS on supported platforms; third-party-purchased bots forbidden; >200 trades/day forbidden
automation_rule_text: "To use an automated trading system, you must use one of the platforms we support: NinjaTrader, Tradovate, TradingView, Jigsaw and Quantower. TradeDay does not expose the Tradovate API to allow traders to connect automated trading systems directly. Algos and Bots purchased from a third party and employed by the trader are forbidden. If multiple users are making the same trades, then all the accounts will be shut down, and the users suspended."
vps_allowed: "no"   # "TradeDay does not allow the use of virtual private servers (VPS)"
personal_device_required: unknown   # device not mentioned; firm requires trading from the registered location/address (see §7)
status: partial
not_confirmed_fields: []
# Payout requires 5 profitable days of at least $200 (FPPAY). Do not trade between payout request and receipt (FPPAY).
# GAP: scaling ceiling not published except 25K ('max 2 minis'); micros after scaling not published.
# Moved to Funded Live on the 5th payout request.
sources:
  - { field: drawdown/consistency/max_position/scaling, url: "https://www.tradeday.com/", accessed: "2026-10-23" }
  - { field: max_position/scaling, url: "https://tradeday.freshdesk.com/en/support/solutions/articles/103000405229-what-is-fast-pass-", accessed: "2026-10-23" }
  - { field: drawdown.type (EOD), url: "https://tradeday.freshdesk.com/en/support/solutions/articles/103000008889-what-are-the-rules-for-funded-funded-sim-and-funded-live-traders-", accessed: "2026-10-23" }
  - { field: stops_trailing_at/payout conditions, url: "https://tradeday.freshdesk.com/en/support/solutions/articles/103000404096-fast-pass-funded-sim-payout-policy", accessed: "2026-10-23" }
  - { field: stops_trailing_at, url: "https://tradeday.freshdesk.com/en/support/solutions/articles/103000250682-funded-accounts-withdrawals-and-the-impact-of-drawdowns", accessed: "2026-10-23" }
  - { field: daily_loss_limit, url: "https://www.tradeday.com/how-it-works", accessed: "2026-10-23" }
  - { field: automated_trading_allowed/automation_rule_text, url: "https://tradeday.freshdesk.com/en/support/solutions/articles/103000085101-automated-algo-and-bot-trading", accessed: "2026-10-23" }
  - { field: automation (prohibited practices), url: "https://tradeday.freshdesk.com/en/support/solutions/articles/103000121031-prohibited-trade-practices", accessed: "2026-10-23" }
  - { field: copy_trading_allowed, url: "https://tradeday.freshdesk.com/en/support/solutions/articles/103000008833-can-i-have-multiple-accounts-can-i-use-copy-trader-", accessed: "2026-10-23" }
  - { field: max accounts / one broker, url: "https://tradeday.freshdesk.com/en/support/solutions/articles/103000354796-multiple-accounts", accessed: "2026-10-23" }
  - { field: vps_allowed, url: "https://tradeday.freshdesk.com/en/support/solutions/articles/103000295384-using-a-vpn-vps-or-ip-masking-add-on", accessed: "2026-10-23" }
  - { field: trading_day_reset, url: "https://tradeday.freshdesk.com/en/support/solutions/articles/103000008864-what-is-the-definition-of-a-trading-day-", accessed: "2026-10-23" }
  - { field: trading_day_reset (auto-flat), url: "https://tradeday.freshdesk.com/en/support/solutions/articles/103000018874-what-is-the-permitted-trading-times-guideline-", accessed: "2026-10-23" }
```

#### Fast Pass — funded — 150K
```yaml
firm: tradeday
plan: "Fast Pass"
phase: funded
size_label: "150K"
size_usd: "150000.00"
instruments: [MNQ, NQ]
platforms: [Tradovate, Rithmic]   # one broker per trader; NinjaTrader Desktop only via Tradovate/CQG (see §2)
drawdown: { amount_usd: "4500.00", type: trailing_eod, stops_trailing_at: "Trails until it reaches the starting account balance (WDD). \"On the first payout request, if the Trailing Maximum Drawdown (TMD) has not yet reached the starting account balance, it will automatically move up to the starting account balance and remain fixed there moving forward.\"" }
daily_loss_limit: { amount_usd: null, kind: none }   # firm: "ONE RULE. ONE WAY TO FAIL" (max drawdown); optional self-set Tradovate DLL only
profit_target_usd: null
consistency_rule: { text: "For accounts opened on and after July 26 2026, accounts must satisfy the consistency objective: no single day can make up more than 45% of your total gross profits (gross profits being current profits + payouts)", threshold_pct: "45" }   # payout condition; accounts opened before July 26 2026: none (S-FPPAY)
max_position: { minis: "4", micros: "40", scaling: "scales + 1 contract per $2k profit at EOD" }
min_trading_days: null
trading_day_reset: { time: "17:00", tz_as_published: "CT" }   # session 5 pm CT - 4 pm CT; EOD drawdown at 16.00 CT; auto-flat 3:50 pm CT
copy_trading_allowed: conditional   # same person only; eval+Funded Sim 2-10 accts; Funded Live up to 5; no Live<->Eval/Sim copying
automated_trading_allowed: conditional   # own ATS on supported platforms; third-party-purchased bots forbidden; >200 trades/day forbidden
automation_rule_text: "To use an automated trading system, you must use one of the platforms we support: NinjaTrader, Tradovate, TradingView, Jigsaw and Quantower. TradeDay does not expose the Tradovate API to allow traders to connect automated trading systems directly. Algos and Bots purchased from a third party and employed by the trader are forbidden. If multiple users are making the same trades, then all the accounts will be shut down, and the users suspended."
vps_allowed: "no"   # "TradeDay does not allow the use of virtual private servers (VPS)"
personal_device_required: unknown   # device not mentioned; firm requires trading from the registered location/address (see §7)
status: partial
not_confirmed_fields: []
# Payout requires 5 profitable days of at least $250 (FPPAY). Do not trade between payout request and receipt (FPPAY).
# GAP: scaling ceiling not published except 25K ('max 2 minis'); micros after scaling not published.
# Moved to Funded Live on the 5th payout request.
sources:
  - { field: drawdown/consistency/max_position/scaling, url: "https://www.tradeday.com/", accessed: "2026-10-23" }
  - { field: max_position/scaling, url: "https://tradeday.freshdesk.com/en/support/solutions/articles/103000405229-what-is-fast-pass-", accessed: "2026-10-23" }
  - { field: drawdown.type (EOD), url: "https://tradeday.freshdesk.com/en/support/solutions/articles/103000008889-what-are-the-rules-for-funded-funded-sim-and-funded-live-traders-", accessed: "2026-10-23" }
  - { field: stops_trailing_at/payout conditions, url: "https://tradeday.freshdesk.com/en/support/solutions/articles/103000404096-fast-pass-funded-sim-payout-policy", accessed: "2026-10-23" }
  - { field: stops_trailing_at, url: "https://tradeday.freshdesk.com/en/support/solutions/articles/103000250682-funded-accounts-withdrawals-and-the-impact-of-drawdowns", accessed: "2026-10-23" }
  - { field: daily_loss_limit, url: "https://www.tradeday.com/how-it-works", accessed: "2026-10-23" }
  - { field: automated_trading_allowed/automation_rule_text, url: "https://tradeday.freshdesk.com/en/support/solutions/articles/103000085101-automated-algo-and-bot-trading", accessed: "2026-10-23" }
  - { field: automation (prohibited practices), url: "https://tradeday.freshdesk.com/en/support/solutions/articles/103000121031-prohibited-trade-practices", accessed: "2026-10-23" }
  - { field: copy_trading_allowed, url: "https://tradeday.freshdesk.com/en/support/solutions/articles/103000008833-can-i-have-multiple-accounts-can-i-use-copy-trader-", accessed: "2026-10-23" }
  - { field: max accounts / one broker, url: "https://tradeday.freshdesk.com/en/support/solutions/articles/103000354796-multiple-accounts", accessed: "2026-10-23" }
  - { field: vps_allowed, url: "https://tradeday.freshdesk.com/en/support/solutions/articles/103000295384-using-a-vpn-vps-or-ip-masking-add-on", accessed: "2026-10-23" }
  - { field: trading_day_reset, url: "https://tradeday.freshdesk.com/en/support/solutions/articles/103000008864-what-is-the-definition-of-a-trading-day-", accessed: "2026-10-23" }
  - { field: trading_day_reset (auto-flat), url: "https://tradeday.freshdesk.com/en/support/solutions/articles/103000018874-what-is-the-permitted-trading-times-guideline-", accessed: "2026-10-23" }
```

#### Fast Pass — live — 25K
```yaml
firm: tradeday
plan: "Fast Pass"
phase: live
size_label: "25K"
size_usd: "25000.00"
instruments: [MNQ, NQ]
platforms: [Tradovate, Rithmic]   # one broker per trader; NinjaTrader Desktop only via Tradovate/CQG (see §2)
drawdown: { amount_usd: null, type: trailing_eod, stops_trailing_at: "\"Every Funded Live account starts at $0\"; floor trails up dollar-for-dollar; after clearing the buffer the balance can never touch or drop below the protected slippage buffer." }
daily_loss_limit: { amount_usd: null, kind: none }   # firm: "ONE RULE. ONE WAY TO FAIL" (max drawdown); optional self-set Tradovate DLL only
profit_target_usd: null
consistency_rule: { text: null, threshold_pct: null }
max_position: { minis: "1", micros: null, scaling: "Any future increases to position limits will be granted solely at the discretion of the TradeDay trading team." }
min_trading_days: null
trading_day_reset: { time: "17:00", tz_as_published: "CT" }   # session 5 pm CT - 4 pm CT; EOD drawdown at 16.00 CT; auto-flat 3:50 pm CT
copy_trading_allowed: conditional   # same person only; eval+Funded Sim 2-10 accts; Funded Live up to 5; no Live<->Eval/Sim copying
automated_trading_allowed: conditional   # own ATS on supported platforms; third-party-purchased bots forbidden; >200 trades/day forbidden
automation_rule_text: "To use an automated trading system, you must use one of the platforms we support: NinjaTrader, Tradovate, TradingView, Jigsaw and Quantower. TradeDay does not expose the Tradovate API to allow traders to connect automated trading systems directly. Algos and Bots purchased from a third party and employed by the trader are forbidden. If multiple users are making the same trades, then all the accounts will be shut down, and the users suspended."
vps_allowed: "no"   # "TradeDay does not allow the use of virtual private servers (VPS)"
personal_device_required: unknown   # device not mentioned; firm requires trading from the registered location/address (see §7)
status: conflict
not_confirmed_fields: [drawdown.amount_usd]
# CONFLICT drawdown: 'same End-of-Day (EOD) drawdown ... from the original evaluation account tier' (tier 25K = $1000.00) vs worked example 'Using a $50K account as an example, the TMD is $1500'.
# Slippage buffer 25K: $500.00 (SLIP). Micros not published for Fast Pass Funded Live.
sources:
  - { field: drawdown ('same EOD drawdown ... original evaluation account tier'), url: "https://tradeday.freshdesk.com/en/support/solutions/articles/103000404097-fast-pass-funded-sim-route-to-funded-live", accessed: "2026-10-23" }
  - { field: drawdown ($1500 example for 50K), url: "https://tradeday.freshdesk.com/en/support/solutions/articles/103000411652-quick-pay-and-fast-pass-funded-live-trading-rules", accessed: "2026-10-23" }
  - { field: max_position, url: "https://tradeday.freshdesk.com/en/support/solutions/articles/103000404097-fast-pass-funded-sim-route-to-funded-live", accessed: "2026-10-23" }
  - { field: slippage buffer by size, url: "https://tradeday.freshdesk.com/en/support/solutions/articles/103000089243-slippage-rule-for-funded-live-traders", accessed: "2026-10-23" }
  - { field: daily_loss_limit, url: "https://www.tradeday.com/how-it-works", accessed: "2026-10-23" }
  - { field: automated_trading_allowed/automation_rule_text, url: "https://tradeday.freshdesk.com/en/support/solutions/articles/103000085101-automated-algo-and-bot-trading", accessed: "2026-10-23" }
  - { field: automation (prohibited practices), url: "https://tradeday.freshdesk.com/en/support/solutions/articles/103000121031-prohibited-trade-practices", accessed: "2026-10-23" }
  - { field: copy_trading_allowed, url: "https://tradeday.freshdesk.com/en/support/solutions/articles/103000008833-can-i-have-multiple-accounts-can-i-use-copy-trader-", accessed: "2026-10-23" }
  - { field: max accounts / one broker, url: "https://tradeday.freshdesk.com/en/support/solutions/articles/103000354796-multiple-accounts", accessed: "2026-10-23" }
  - { field: vps_allowed, url: "https://tradeday.freshdesk.com/en/support/solutions/articles/103000295384-using-a-vpn-vps-or-ip-masking-add-on", accessed: "2026-10-23" }
  - { field: trading_day_reset, url: "https://tradeday.freshdesk.com/en/support/solutions/articles/103000008864-what-is-the-definition-of-a-trading-day-", accessed: "2026-10-23" }
  - { field: trading_day_reset (auto-flat), url: "https://tradeday.freshdesk.com/en/support/solutions/articles/103000018874-what-is-the-permitted-trading-times-guideline-", accessed: "2026-10-23" }
```

#### Fast Pass — live — 50K
```yaml
firm: tradeday
plan: "Fast Pass"
phase: live
size_label: "50K"
size_usd: "50000.00"
instruments: [MNQ, NQ]
platforms: [Tradovate, Rithmic]   # one broker per trader; NinjaTrader Desktop only via Tradovate/CQG (see §2)
drawdown: { amount_usd: null, type: trailing_eod, stops_trailing_at: "\"Every Funded Live account starts at $0\"; floor trails up dollar-for-dollar; after clearing the buffer the balance can never touch or drop below the protected slippage buffer." }
daily_loss_limit: { amount_usd: null, kind: none }   # firm: "ONE RULE. ONE WAY TO FAIL" (max drawdown); optional self-set Tradovate DLL only
profit_target_usd: null
consistency_rule: { text: null, threshold_pct: null }
max_position: { minis: "2", micros: null, scaling: "Any future increases to position limits will be granted solely at the discretion of the TradeDay trading team." }
min_trading_days: null
trading_day_reset: { time: "17:00", tz_as_published: "CT" }   # session 5 pm CT - 4 pm CT; EOD drawdown at 16.00 CT; auto-flat 3:50 pm CT
copy_trading_allowed: conditional   # same person only; eval+Funded Sim 2-10 accts; Funded Live up to 5; no Live<->Eval/Sim copying
automated_trading_allowed: conditional   # own ATS on supported platforms; third-party-purchased bots forbidden; >200 trades/day forbidden
automation_rule_text: "To use an automated trading system, you must use one of the platforms we support: NinjaTrader, Tradovate, TradingView, Jigsaw and Quantower. TradeDay does not expose the Tradovate API to allow traders to connect automated trading systems directly. Algos and Bots purchased from a third party and employed by the trader are forbidden. If multiple users are making the same trades, then all the accounts will be shut down, and the users suspended."
vps_allowed: "no"   # "TradeDay does not allow the use of virtual private servers (VPS)"
personal_device_required: unknown   # device not mentioned; firm requires trading from the registered location/address (see §7)
status: conflict
not_confirmed_fields: [drawdown.amount_usd]
# CONFLICT drawdown: 'same End-of-Day (EOD) drawdown ... from the original evaluation account tier' (tier 50K = $2000.00) vs worked example 'Using a $50K account as an example, the TMD is $1500'.
# Slippage buffer 50K: $500.00 (SLIP). Micros not published for Fast Pass Funded Live.
sources:
  - { field: drawdown ('same EOD drawdown ... original evaluation account tier'), url: "https://tradeday.freshdesk.com/en/support/solutions/articles/103000404097-fast-pass-funded-sim-route-to-funded-live", accessed: "2026-10-23" }
  - { field: drawdown ($1500 example for 50K), url: "https://tradeday.freshdesk.com/en/support/solutions/articles/103000411652-quick-pay-and-fast-pass-funded-live-trading-rules", accessed: "2026-10-23" }
  - { field: max_position, url: "https://tradeday.freshdesk.com/en/support/solutions/articles/103000404097-fast-pass-funded-sim-route-to-funded-live", accessed: "2026-10-23" }
  - { field: slippage buffer by size, url: "https://tradeday.freshdesk.com/en/support/solutions/articles/103000089243-slippage-rule-for-funded-live-traders", accessed: "2026-10-23" }
  - { field: daily_loss_limit, url: "https://www.tradeday.com/how-it-works", accessed: "2026-10-23" }
  - { field: automated_trading_allowed/automation_rule_text, url: "https://tradeday.freshdesk.com/en/support/solutions/articles/103000085101-automated-algo-and-bot-trading", accessed: "2026-10-23" }
  - { field: automation (prohibited practices), url: "https://tradeday.freshdesk.com/en/support/solutions/articles/103000121031-prohibited-trade-practices", accessed: "2026-10-23" }
  - { field: copy_trading_allowed, url: "https://tradeday.freshdesk.com/en/support/solutions/articles/103000008833-can-i-have-multiple-accounts-can-i-use-copy-trader-", accessed: "2026-10-23" }
  - { field: max accounts / one broker, url: "https://tradeday.freshdesk.com/en/support/solutions/articles/103000354796-multiple-accounts", accessed: "2026-10-23" }
  - { field: vps_allowed, url: "https://tradeday.freshdesk.com/en/support/solutions/articles/103000295384-using-a-vpn-vps-or-ip-masking-add-on", accessed: "2026-10-23" }
  - { field: trading_day_reset, url: "https://tradeday.freshdesk.com/en/support/solutions/articles/103000008864-what-is-the-definition-of-a-trading-day-", accessed: "2026-10-23" }
  - { field: trading_day_reset (auto-flat), url: "https://tradeday.freshdesk.com/en/support/solutions/articles/103000018874-what-is-the-permitted-trading-times-guideline-", accessed: "2026-10-23" }
```

#### Fast Pass — live — 100K
```yaml
firm: tradeday
plan: "Fast Pass"
phase: live
size_label: "100K"
size_usd: "100000.00"
instruments: [MNQ, NQ]
platforms: [Tradovate, Rithmic]   # one broker per trader; NinjaTrader Desktop only via Tradovate/CQG (see §2)
drawdown: { amount_usd: null, type: trailing_eod, stops_trailing_at: "\"Every Funded Live account starts at $0\"; floor trails up dollar-for-dollar; after clearing the buffer the balance can never touch or drop below the protected slippage buffer." }
daily_loss_limit: { amount_usd: null, kind: none }   # firm: "ONE RULE. ONE WAY TO FAIL" (max drawdown); optional self-set Tradovate DLL only
profit_target_usd: null
consistency_rule: { text: null, threshold_pct: null }
max_position: { minis: "3", micros: null, scaling: "Any future increases to position limits will be granted solely at the discretion of the TradeDay trading team." }
min_trading_days: null
trading_day_reset: { time: "17:00", tz_as_published: "CT" }   # session 5 pm CT - 4 pm CT; EOD drawdown at 16.00 CT; auto-flat 3:50 pm CT
copy_trading_allowed: conditional   # same person only; eval+Funded Sim 2-10 accts; Funded Live up to 5; no Live<->Eval/Sim copying
automated_trading_allowed: conditional   # own ATS on supported platforms; third-party-purchased bots forbidden; >200 trades/day forbidden
automation_rule_text: "To use an automated trading system, you must use one of the platforms we support: NinjaTrader, Tradovate, TradingView, Jigsaw and Quantower. TradeDay does not expose the Tradovate API to allow traders to connect automated trading systems directly. Algos and Bots purchased from a third party and employed by the trader are forbidden. If multiple users are making the same trades, then all the accounts will be shut down, and the users suspended."
vps_allowed: "no"   # "TradeDay does not allow the use of virtual private servers (VPS)"
personal_device_required: unknown   # device not mentioned; firm requires trading from the registered location/address (see §7)
status: conflict
not_confirmed_fields: [drawdown.amount_usd]
# CONFLICT drawdown: 'same End-of-Day (EOD) drawdown ... from the original evaluation account tier' (tier 100K = $3000.00) vs worked example 'Using a $50K account as an example, the TMD is $1500'.
# Slippage buffer 100K: $750.00 (SLIP). Micros not published for Fast Pass Funded Live.
sources:
  - { field: drawdown ('same EOD drawdown ... original evaluation account tier'), url: "https://tradeday.freshdesk.com/en/support/solutions/articles/103000404097-fast-pass-funded-sim-route-to-funded-live", accessed: "2026-10-23" }
  - { field: drawdown ($1500 example for 50K), url: "https://tradeday.freshdesk.com/en/support/solutions/articles/103000411652-quick-pay-and-fast-pass-funded-live-trading-rules", accessed: "2026-10-23" }
  - { field: max_position, url: "https://tradeday.freshdesk.com/en/support/solutions/articles/103000404097-fast-pass-funded-sim-route-to-funded-live", accessed: "2026-10-23" }
  - { field: slippage buffer by size, url: "https://tradeday.freshdesk.com/en/support/solutions/articles/103000089243-slippage-rule-for-funded-live-traders", accessed: "2026-10-23" }
  - { field: daily_loss_limit, url: "https://www.tradeday.com/how-it-works", accessed: "2026-10-23" }
  - { field: automated_trading_allowed/automation_rule_text, url: "https://tradeday.freshdesk.com/en/support/solutions/articles/103000085101-automated-algo-and-bot-trading", accessed: "2026-10-23" }
  - { field: automation (prohibited practices), url: "https://tradeday.freshdesk.com/en/support/solutions/articles/103000121031-prohibited-trade-practices", accessed: "2026-10-23" }
  - { field: copy_trading_allowed, url: "https://tradeday.freshdesk.com/en/support/solutions/articles/103000008833-can-i-have-multiple-accounts-can-i-use-copy-trader-", accessed: "2026-10-23" }
  - { field: max accounts / one broker, url: "https://tradeday.freshdesk.com/en/support/solutions/articles/103000354796-multiple-accounts", accessed: "2026-10-23" }
  - { field: vps_allowed, url: "https://tradeday.freshdesk.com/en/support/solutions/articles/103000295384-using-a-vpn-vps-or-ip-masking-add-on", accessed: "2026-10-23" }
  - { field: trading_day_reset, url: "https://tradeday.freshdesk.com/en/support/solutions/articles/103000008864-what-is-the-definition-of-a-trading-day-", accessed: "2026-10-23" }
  - { field: trading_day_reset (auto-flat), url: "https://tradeday.freshdesk.com/en/support/solutions/articles/103000018874-what-is-the-permitted-trading-times-guideline-", accessed: "2026-10-23" }
```

#### Fast Pass — live — 150K
```yaml
firm: tradeday
plan: "Fast Pass"
phase: live
size_label: "150K"
size_usd: "150000.00"
instruments: [MNQ, NQ]
platforms: [Tradovate, Rithmic]   # one broker per trader; NinjaTrader Desktop only via Tradovate/CQG (see §2)
drawdown: { amount_usd: null, type: trailing_eod, stops_trailing_at: "\"Every Funded Live account starts at $0\"; floor trails up dollar-for-dollar; after clearing the buffer the balance can never touch or drop below the protected slippage buffer." }
daily_loss_limit: { amount_usd: null, kind: none }   # firm: "ONE RULE. ONE WAY TO FAIL" (max drawdown); optional self-set Tradovate DLL only
profit_target_usd: null
consistency_rule: { text: null, threshold_pct: null }
max_position: { minis: "4", micros: null, scaling: "Any future increases to position limits will be granted solely at the discretion of the TradeDay trading team." }
min_trading_days: null
trading_day_reset: { time: "17:00", tz_as_published: "CT" }   # session 5 pm CT - 4 pm CT; EOD drawdown at 16.00 CT; auto-flat 3:50 pm CT
copy_trading_allowed: conditional   # same person only; eval+Funded Sim 2-10 accts; Funded Live up to 5; no Live<->Eval/Sim copying
automated_trading_allowed: conditional   # own ATS on supported platforms; third-party-purchased bots forbidden; >200 trades/day forbidden
automation_rule_text: "To use an automated trading system, you must use one of the platforms we support: NinjaTrader, Tradovate, TradingView, Jigsaw and Quantower. TradeDay does not expose the Tradovate API to allow traders to connect automated trading systems directly. Algos and Bots purchased from a third party and employed by the trader are forbidden. If multiple users are making the same trades, then all the accounts will be shut down, and the users suspended."
vps_allowed: "no"   # "TradeDay does not allow the use of virtual private servers (VPS)"
personal_device_required: unknown   # device not mentioned; firm requires trading from the registered location/address (see §7)
status: conflict
not_confirmed_fields: [drawdown.amount_usd]
# CONFLICT drawdown: 'same End-of-Day (EOD) drawdown ... from the original evaluation account tier' (tier 150K = $4500.00) vs worked example 'Using a $50K account as an example, the TMD is $1500'.
# Slippage buffer 150K: $1000.00 (SLIP). Micros not published for Fast Pass Funded Live.
sources:
  - { field: drawdown ('same EOD drawdown ... original evaluation account tier'), url: "https://tradeday.freshdesk.com/en/support/solutions/articles/103000404097-fast-pass-funded-sim-route-to-funded-live", accessed: "2026-10-23" }
  - { field: drawdown ($1500 example for 50K), url: "https://tradeday.freshdesk.com/en/support/solutions/articles/103000411652-quick-pay-and-fast-pass-funded-live-trading-rules", accessed: "2026-10-23" }
  - { field: max_position, url: "https://tradeday.freshdesk.com/en/support/solutions/articles/103000404097-fast-pass-funded-sim-route-to-funded-live", accessed: "2026-10-23" }
  - { field: slippage buffer by size, url: "https://tradeday.freshdesk.com/en/support/solutions/articles/103000089243-slippage-rule-for-funded-live-traders", accessed: "2026-10-23" }
  - { field: daily_loss_limit, url: "https://www.tradeday.com/how-it-works", accessed: "2026-10-23" }
  - { field: automated_trading_allowed/automation_rule_text, url: "https://tradeday.freshdesk.com/en/support/solutions/articles/103000085101-automated-algo-and-bot-trading", accessed: "2026-10-23" }
  - { field: automation (prohibited practices), url: "https://tradeday.freshdesk.com/en/support/solutions/articles/103000121031-prohibited-trade-practices", accessed: "2026-10-23" }
  - { field: copy_trading_allowed, url: "https://tradeday.freshdesk.com/en/support/solutions/articles/103000008833-can-i-have-multiple-accounts-can-i-use-copy-trader-", accessed: "2026-10-23" }
  - { field: max accounts / one broker, url: "https://tradeday.freshdesk.com/en/support/solutions/articles/103000354796-multiple-accounts", accessed: "2026-10-23" }
  - { field: vps_allowed, url: "https://tradeday.freshdesk.com/en/support/solutions/articles/103000295384-using-a-vpn-vps-or-ip-masking-add-on", accessed: "2026-10-23" }
  - { field: trading_day_reset, url: "https://tradeday.freshdesk.com/en/support/solutions/articles/103000008864-what-is-the-definition-of-a-trading-day-", accessed: "2026-10-23" }
  - { field: trading_day_reset (auto-flat), url: "https://tradeday.freshdesk.com/en/support/solutions/articles/103000018874-what-is-the-permitted-trading-times-guideline-", accessed: "2026-10-23" }
```

---

## 9. Unverified / gaps / conflicts

| # | Field | Status | What is missing or conflicting | Where searched |
|---|---|---|---|---|
| C1 | Evaluation `min_trading_days` (Fast Pass and Quick Pay) | `conflict` | **Fast Pass:** 3 days (S-OBJ, S-DAYS, S-HOME cards) vs "no minimum trading days" (S-FP, S-HOW, S-HOME FAQ, S-BLOG) vs S-TC "reach profit targets set for you over a minimum 7-day period" (generic, all evaluations). **Quick Pay:** 5 days on every help-center page, how-it-works and the homepage vs the same S-TC 7-day wording. The YAML keeps 5 for Quick Pay because 5 is what every rules page publishes, but the field is marked not confirmed. | All help-center "objectives" articles, homepage, how-it-works, T&C |
| C2 | Quick Pay Funded Live `max_position` | `conflict` | S-LIVE sets limits by balance brought over (2/20, 3/30, 4/40). S-MULTI says they equal the Funded Sim limits, possibly reduced. | S-LIVE, S-MULTI, S-SLIP, S-WDD |
| C3 | Funded Live slippage buffer | `conflict` | S-SLIP: by size ($500 for 25K/50K, $750 for 100K, $1,000 for 150K). S-LIVE (Quick Pay): by balance brought over ($500/$750/$1,000). | S-SLIP, S-LIVE |
| C4 | Fast Pass Funded Live `drawdown.amount_usd` | `conflict` | "same End-of-Day (EOD) drawdown … from the original evaluation account tier" (50K tier = $2,000) vs worked example "Using a $50K account as an example, the TMD is $1500". | S-FPLIVE, S-LIVE, S-FP |
| C5 | Fast Pass Funded Sim 100K payout cap (low priority) | `conflict` | $1,850 (S-FP) vs $1,875 (S-FPPAY, S-HOME). | Same pages |
| G1 | Quick Pay Funded Live drawdown | partial | "The maximum drawdown limit will be set to $0" (S-WDD) plus the slippage-buffer liquidation. Whether the $0 floor and the slippage buffer are one rule or two is not explained. | S-WDD, S-LIVE, S-SLIP |
| G2 | Fast Pass Funded Live micros | not published | Only contracts (1/2/3/4) are given. | S-FPLIVE, S-LIVE, S-FP |
| G3 | Fast Pass Funded Sim scaling ceiling | not published | "+1 contract per $2k profit at EOD". A ceiling ("max 2 minis") appears only for 25K. No ceiling and no micro count after scaling for 50K/100K/150K. | S-HOME, S-FP |
| G4 | Funded Sim / Funded Live `min_trading_days` | n/a | Not a pass requirement in funded phases. Payout preconditions recorded instead: Quick Pay "Min Days to Payout: 1"; Fast Pass 5 profitable days. | S-HOME, S-FPPAY |
| G5 | Holiday schedule | not published by firm | S-TIMES points to the CME holiday calendar and says it is the trader's responsibility. | S-TIMES, S-TDAY |
| G6 | Copying **from TradeDay to other firms** | not mentioned | Only copying **into** TradeDay from "your personal accounts or other evaluations" is addressed. | S-COPY, S-MULTI, S-PROHIB, S-TC |
| G7 | Daily loss limit `kind: none` | verified by statement, not literal | The firm says max drawdown is the only rule ("ONE RULE. ONE WAY TO FAIL", "TradeDay has just one rule"), and no DLL appears on any page. The words "no daily loss limit" do not appear. The PO should confirm that this supports `none`. | All 85 help-center articles (grep "daily loss"), homepage, how-it-works, T&C |
| G8 | `personal_device_required` | `unknown` | Device is not mentioned. The firm requires trading "from the address under which the account is registered" / "from the location registered with TradeDay" (S-VPS). | All help-center articles, T&C, homepage |
| G9 | Automation: semi-automated, webhooks | not mentioned | Only "automated trading system", "Algos and Bots", "trading bots", "ATS" are used. Whether a user-owned copier or router counts as an "automated trading system" or as a "trade copier" is not stated. | S-AUTO, S-PROHIB, S-TC, S-SCALP, S-SPLIT, S-PLAT |
| G10 | Funded Sim/Live agreements | not public | S-TC says Funded SIM and Funded LIVE accounts "are governed by a separate agreement between the User and TradeDay". That agreement is not on the public site (presumably behind the members area, `unverified (behind login)`). | S-TC; members.tradeday.com not accessed |

Pages not reachable: none. Every public page tried returned HTTP 200 with `curl`.

---

## 10. Changelog

- 2026-10-23 — initial research (Lupa). Covered:
  - Quick Pay (Intraday and End of Day) and Fast Pass (End of Day), 25K/50K/100K/150K;
  - Evaluation, Funded Sim and Funded Live phases;
  - platforms (format rev. 2) and device/hosting rules (PO rev. 3).

  Sources: tradeday.com homepage, how-it-works, T&C, the TradeDay 2.0 blog post, and all 85 public articles of tradeday.freshdesk.com.
- 2026-10-23 — structure (Lupa): converted the single YAML list in §8 to one `yaml` block per plan/phase/size (28 blocks, each under its own `####` heading). Values, comments and status unchanged. Checked by parsing the old list and the new blocks and comparing them: identical. Only the section-divider comments were dropped, because each heading now carries that information.
