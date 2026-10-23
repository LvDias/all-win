# Tradeify — Prop Firm Rules Research

| | |
|---|---|
| **Firm** | Tradeify (futures; "Tradeify Futures") |
| **Official domains** | `tradeify.co` (main site; plain curl works, but the plan cards are filled in by JavaScript, so they were read through a browser portal) · `help.tradeify.co` (official help center, Intercom, linked from the main site's "Help Center" menu as `https://help.tradeify.co/en/`; blocks curl with Cloudflare 403, so it was read through the portal) · `app-f.tradeify.co` (dashboard login, not used). Sister brands `tradeify247.co` (crypto) and `tradeifyfx.co` (forex) are separate and out of scope ("completely separate from Tradeify Futures", Common FAQs) |
| **Research date** | 2026-10-23 (all URLs below accessed 2026-10-23) |
| **Researcher** | Lupa (Prop Firm Rules Researcher) |
| **File status** | `partial`. The main fields are verified for every current plan and size. Still open: the conflicts and gaps in §9 (Select 25K evaluation max contracts, DLL kind on the 300K V2, Elite Live micro limits and platforms, copying to/from other firms, personal-device rule) |
| **Instruments covered** | MNQ, NQ. Both are supported products: "E-Mini NASDAQ (NQ) on CME … Micro E-Mini NASDAQ (MNQ) on CME" (https://help.tradeify.co/en/articles/10468222-rules-supported-trading-products-assets) |

Evidence (raw text of every cited page, URL on line 1): `/tmp/claude-1003/-srv-www-allwin--maestri-roles-6bf77a80-a1ed-43dc-bc00-08a480f178a5/33652d13-4dae-46b6-ae06-44a72b375517/scratchpad/r2/tradeify/`. This is temporary scratch storage, so the evidence must be captured again at re-verification.

**Programs found (names as published).** The homepage plan selector has three plans: **Growth** ("Pass in 1 day"), **Select** ("Pass in 3 days") and **Lightning** ("Instant"). The help center calls them "GROWTH™ Evaluation", "Select Evaluation" (after passing you choose **Select Flex** or **Select Daily** for the funded account) and "Lightning Funded" (straight to sim funded). Sizes are 25K, 50K, 100K and 150K. Select also has a **300K** size, sold as a "limited release": V2 is the current version, and V1 is legacy and no longer sold. The live stage is the **Tradeify Elite** program. The **Legacy Live** program (accounts bought before Dec 3, 2025) is recorded in §7 only. The Funded Trader Agreement §4.1 still names an "Advanced Challenge". The homepage statistics also mention "Advanced", but no Advanced plan is sold or documented in the help center today, so it is not covered (§9).

**Terms used below.**
- "EOD" means end of day.
- "DLL" means Daily Loss Limit.
- "Sim Funded" is Tradeify's name for the simulated funded phase (`phase: funded`).
- "Elite Live" is the real-capital phase (`phase: live`).

---

## 1. Platforms

Source: https://help.tradeify.co/en/articles/10468221-supported-platforms (cross-checked with the Common FAQs and the Pricing Reference).

> "Tradeify supports three broker connections: Tradovate, Rithmic, and WealthCharts. You select your broker at checkout. Each broker has its own credentials and gives you access to different trading platforms."

| Broker connection (chosen at checkout) | Platforms |
|---|---|
| **Tradovate** | Tradovate, **NinjaTrader** ("connects using your Tradovate credentials"), TradingView ("connects via Tradovate") |
| **Rithmic** | Tradesea, Quantower, Sierra Chart (paid subscription), R\|Trader |
| **WealthCharts** | WealthCharts only |

- NinjaTrader is Tradovate-only. Quoted: "Can I use NinjaTrader with a Rithmic account? No. NinjaTrader connects through Tradovate only."
- You cannot switch broker after purchase: "you cannot switch between brokers (e.g., from Tradovate to Rithmic) without purchasing a new account."
- ProjectX is no longer offered: "We offered ProjectX as a platform option for a short time. … so we no longer offer that platform." (Common FAQs)
- **Per plan.** The homepage selector offers the same three connections (Tradovate / WealthCharts / Rithmic / Tradesea) for Growth, Select and Lightning. The 300K page says: "The 300K size is offered on Tradovate, Rithmic and WealthCharts." I captured the homepage cards for every plan × connection. The rule values (drawdown, DLL, profit target, consistency, max contracts) are **identical across connections**. The only text difference is the Select reset tooltip ("Allowed up to 10 resets one time" on Rithmic vs "… per month" on Tradovate), which is a pricing detail. So there is one YAML block per plan/phase/size, valid for all connections.
- **Rule difference by platform (Rithmic only).** Holiday half-days are recorded late: "On the Rithmic broker, a holiday half-day (shortened session) is not recorded until the next full market close. The day still counts as a trading day, but the credit (and any payout based on it) may not post until the following full session closes." (https://help.tradeify.co/en/articles/10468225-what-is-a-trading-day)
- **Elite Live:** platforms are not stated on the Elite page (§9).

## 2. Plans

Common mechanics (all plans). Source: https://help.tradeify.co/en/articles/10495897-rules-trailing-max-drawdowns
- **Drawdown type: `trailing_eod`.** Quoted:
  - "All Tradeify accounts use EOD Trailing Drawdown. Your drawdown limit only updates at the end of each trading day, not during the day."
  - "it is ENFORCED in real-time against your net liquidation value (your balance plus unrealized P&L on any open positions)."
  - "Hitting this limit FAILS your account permanently."
- **When it stops trailing.** Quoted:
  - "On Sim Funded accounts (not Evaluations), your drawdown locks in place once you profit beyond the drawdown amount by $100, or immediately when you request a payout - whichever comes first."
  - "Once locked, the drawdown floor becomes fixed at $100 above starting balance"
  - "Drawdown never locks on evaluation accounts." (Essential Rules)
- **DLL is a soft pause.** Source: https://help.tradeify.co/en/articles/10468321-rules-daily-loss-limit. Quoted:
  - "The Daily Loss Limit is a "soft breach" - it pauses trading for the day but does NOT fail your account".
  - "You can resume trading the NEXT market session (after 6:00 PM ET)".
  - "Applies to: Growth, Lightning, and Select Daily Funded accounts. Select Flex accounts do not have a DLL."
  - Warning: "The DLL is not a hard stop - losses may exceed the limit before the system triggers protection."
- **Contract ratio.** Quoted: "10 Micro contracts = 1 Mini contract / You can hold minis and micros at the same time. Your combined position must stay within your account's contract limit, counted at this 10:1 ratio." (https://help.tradeify.co/en/articles/12268167-essential-trading-rules-overview)
- **Payout rule that affects trading (funded).** The 10-second rule, quoted: "To request a payout on a funded account, you must meet BOTH of the below two criteria: / Over 50% of your trades are longer than 10 seconds / Over 50% of your profit must come from trades held longer than 10 seconds. … It does not fail your account." (Guidelines for Traders). The FTA §6.8 wording differs (§9).

### 2.1 Growth — Evaluation ("GROWTH™ Evaluation")

Source: https://help.tradeify.co/en/articles/10495915-growth-evaluation-accounts. Values match the homepage Growth cards (https://tradeify.co/, rendered).

| Size | Profit target | Trailing max drawdown (EOD) | DLL (soft breach) | Consistency | Max position | Min days |
|---|---|---|---|---|---|---|
| 25K ($25,000) | $1,500 | $1,000 | $600 | none | 1 contract (10 micros) | 1 |
| 50K ($50,000) | $3,000 | $2,000 | $1,250 | none | 4 contracts (40 micros) | 1 |
| 100K ($100,000) | $6,000 | $3,500 | $2,500 | none | 8 contracts (80 micros) | 1 |
| 150K ($150,000) | $9,000 | $5,000 | $3,750 | none | 12 contracts (120 micros) | 1 |

- Quoted: "GROWTH™ evaluation has a minimum of 1 trading day, so you can pass immediately. It has no consistency rule. It has a Daily Loss Limit (soft breach)."
- Legacy max position (bought before Sep 12, 2025 8:00 AM ET): 50K 5 (50 micros), 100K 10 (100), 150K 15 (150).
- No time limit. Weekly inactivity rule: "you must place at least one trade per week (Monday through Friday) on each evaluation account".
- Daily activation limit: "You can activate a maximum of 5 GROWTH™ Funded accounts per day. The limit resets every 24 hours (UTC)."

### 2.2 Growth — Sim Funded

Sources:
- Homepage Growth funded cards (https://tradeify.co/, rendered): "Max Drawdown (EOD) $1,000 / $2,000 / $3,500 / $5,000"; DLL "$600 / $1,250 / $2,500 / $3,750"; "Consistency 35%"; "Max Contracts" as in the evaluation.
- DLL article (initial DLL, and the increase at 6%).
- Drawdown article (lock values).
- Payout: https://help.tradeify.co/en/articles/11083796-growth-funded-account-payout-policy

| Size | Drawdown (EOD) | Lock trigger (EOD balance) → floor | Initial DLL | DLL after 6% profit (from next session) | Consistency (payout) | Max position |
|---|---|---|---|---|---|---|
| 25K | $1,000 | $26,100 → $25,100 | $600 | $1,000 (at $26,500) | 35% | 1 / 10 micros |
| 50K | $2,000 | $52,100 → $50,100 | $1,250 | $2,000 (at $53,000) | 35% | 4 / 40 |
| 100K | $3,500 | $103,600 → $100,100 | $2,500 | $3,500 (at $106,000) | 35% | 8 / 80 |
| 150K | $5,000 | $155,100 → $150,100 | $3,750 | $5,000 (at $159,000) | 35% | 12 / 120 |

- DLL increase, quoted: "When your account reaches 6% profit (balance reaches the threshold below), your DLL increases to match your drawdown amount. This larger DLL takes effect the NEXT trading session". Legacy accounts (before Sep 12, 2025): "your DLL is completely REMOVED".
- Scaling, quoted: "Growth and Lightning use their maximum position size from day one".
- Payout rules that change trading:
  - Quoted: "5 or more of the Trading Days must show a profit greater than: $100 … 25k / $150 … 50k / $200 … 100k / $250 … 150k".
  - Minimum balance: $26,500 / $53,000 / $104,500 / $156,500.
  - Quoted: "This amount must be maintained until the payout request is approved."
- The Sim Funded account cannot be reset.

### 2.3 Select — Evaluation

Sources: https://help.tradeify.co/en/articles/12853921-select-evaluation-accounts and the homepage Select cards.

| Size | Profit target | Max drawdown (EOD) | DLL | Consistency | Max contracts | Min days |
|---|---|---|---|---|---|---|
| 25K | $1,500 | $1,000 | None | 40% (50% add-on) | **conflict**: "1 mini / 10 micro" vs "2/20" (§9 #1) | 3 |
| 50K | $3,000 | $2,000 | None | 40% (50% add-on) | 4 mini / 40 micro | 3 |
| 100K | $6,000 | $3,000 | None | 40% (50% add-on) | 8 mini / 80 micro | 3 |
| 150K | $9,000 | $4,500 | None | 40% (50% add-on) | 12 mini / 120 micro | 3 |
| 300K V2 (limited) | $14,000 | $8,000 (EOD) | $4,000 (kind not stated, §9 #2) | 40% | 16 mini / 160 micro | 3 |

- Consistency, quoted: "No single trading day can represent more than 40% of your total profit." The "50% Consistency Add-on raises your consistency limit from 40% to 50% … pass your evaluation in as little as 2 trading days" (Pricing Reference). The add-on is not available on the 300K.
- Min days, quoted: "SELECT™ evaluation has a minimum of 3 trading days because of its consistency rule."
- The 50K bought before the new dashboard launch keeps a $2,500 target.
- 300K (https://help.tradeify.co/en/articles/16497699-300k-select-account):
  - V2 = "purchased on or after" Aug 27, 2026.
  - V1 legacy: $6,000 max loss, no DLL.
  - Quoted: "cannot be reset"; "KYC must be completed and approved before your purchase"; "A maximum of 3 Select 300K purchases per user for the lifetime".
  - The general Select articles say "No daily loss limits during evaluation". That is stated for the 25K–150K sizes. The 300K page states the V2 $4,000 DLL explicitly, so it is recorded as a size-specific rule.
- Purchase limits: "Maximum 15 evaluations can be purchased within any 30-day period … Each evaluation can be reset up to 10 times within a 30-day period".

### 2.4 Select Flex — Sim Funded (5-Day Payout Policy)

Source: https://help.tradeify.co/en/articles/12853966-select-flex-and-select-daily-payout-policies (plus the 300K page). The payout policy is chosen after passing, and the choice is permanent for that account.

| Size | Max drawdown (EOD) | Lock trigger → floor | DLL | Consistency | Starting → max contracts (scaled) |
|---|---|---|---|---|---|
| 25K | $1,000 | $26,100 → $25,100 | None | None | 1/10 → 2/20 |
| 50K | $2,000 | $52,100 → $50,100 | None | None | 2/20 → 4/40 |
| 100K | $3,000 | $103,100 → $100,100 | None | None | 3/30 → 8/80 |
| 150K | $4,500 | $154,600 → $150,100 | None | None | 3/30 → 12/120 |
| 300K V2 | $8,000 | $308,100 → $300,100 | $4,000 (kind not stated) | None | 3/30 → 16/160 |

- **Scaling plan (Select funded, both policies).** Quoted: "Scaling is calibrated at End of Day (EOD) … When your EOD equity reaches a scaling trigger threshold, your limits automatically increase the next trading day / Scaling triggers are cumulative".
  - 25K: $1,500 → 2/20.
  - 50K: $1,500 → 3/30; $2,000 → 4/40.
  - 100K: $1,500 → 4/40; $2,000 → 5/50; $3,000 → 8/80.
  - 150K: the same as 100K, then $4,500 → 12/120.
  - 300K, by EOD profit: $0–1,500 3/30; $1,500–3,000 4/40; $3,000–5,000 6/60; $5,000–7,000 10/100; $7,000+ 16/160.
- Payout:
  - "Every 5 winning days". Winning day = 25K $100 / 50K $150 / 100K $200 / 150K $250 / 300K $750.
  - Quoted: "You must have positive net profit during each payout cycle".
- 300K V1 legacy Flex: $6,000 drawdown, no DLL, lock at $306,100.

### 2.5 Select Daily — Sim Funded (Daily Payout Policy)

| Size | Max drawdown (EOD) | Lock trigger → floor | DLL (soft) | Buffer | Starting → max contracts |
|---|---|---|---|---|---|
| 25K | $1,000 | $26,100 → $25,100 | $500 | $1,100 | 1/10 → 2/20 |
| 50K | $2,000 | $52,100 → $50,100 | $1,000 | $2,100 | 2/20 → 4/40 |
| 100K | $2,500 | $102,600 → $100,100 | $1,250 | $2,600 | 3/30 → 8/80 |
| 150K | $3,500 | $153,600 → $150,100 | $1,750 | $3,600 | 3/30 → 12/120 |
| 300K V2 | $6,000 | $306,100 → $300,100 | $3,000 | $6,100 | 3/30 → 16/160 |

- No consistency rule on funded. Quoted: "the 40% consistency requirement only applies during the evaluation phase."
- Buffer, quoted: "You cannot withdraw profits that would bring your account balance below the buffer amount."
- 300K V1 legacy Daily: $5,000 drawdown, $2,500 DLL, buffer $5,100.
- The values match the homepage Select funded cards ("Daily / Flex" toggle).

### 2.6 Lightning Funded (no evaluation)

Sources:
- https://help.tradeify.co/en/articles/10495938-lightning-funded-accounts
- Payout: https://help.tradeify.co/en/articles/10495932-lightning-funded-account-payout-policy
- Homepage Lightning cards (same values).

| Size | Trailing max drawdown (EOD) | Lock trigger → floor | DLL (soft) | DLL after 6% | Consistency (payouts) | Max position |
|---|---|---|---|---|---|---|
| 25K | $1,000 | $26,100 → $25,100 | None | N/A | 20% → 25% → 30% | 1 / 10 micros |
| 50K | $2,000 | $52,100 → $50,100 | $1,250 | $2,000 (at $53,000) | 20% → 25% → 30% | 4 / 40 |
| 100K | $4,000 | $104,100 → $100,100 | $2,500 | $4,000 (at $106,000) | 20% → 25% → 30% | 8 / 80 |
| 150K | $5,250 | $155,350 → $150,100 | $3,000 | $5,250 (at $159,000) | 20% → 25% → 30% | 12 / 120 |

- Consistency, quoted: "First payout: 20% / Second payout: 25% / Third and subsequent payouts: 30%" (accounts bought after Sep 12, 2025 8:00 AM ET; older accounts: 20% for all). Formula as published: "Biggest End of Day PnL / Consistency % = Total Balance Needed".
- Minimum days, quoted: "No Minimum Trading Day Count."
- Payout profit goal (Payout 1 / Payout 2+): $1,500/$1,000 · $3,000/$2,000 · $6,000/$3,500 · $9,000/$4,500.
- Legacy accounts:
  - 150K bought before March 31: DLL $3,750.
  - 150K bought before the dashboard launch: DLL $3,750 and drawdown $6,000.
  - Legacy position limits (before Sep 12, 2025): 5/10/15 minis.
- No resets ("replacement only").

### 2.7 Tradeify Elite — Live

Source: https://help.tradeify.co/en/articles/12969284-tradeify-elite-program

Live is by invitation only. Quoted: "3 payouts on a single account, or 10 total payouts … These thresholds represent the minimum requirements for consideration, not automatic qualification." Accounts start at a $0 balance. There is no DLL: "Elite Live accounts include fixed drawdowns, expanded contract limits, and no Daily Loss Limit (DLL)."

| Live size (from sim size) | EOD drawdown | Lock trigger (EOD balance) | Contracts ($0 → DD) | Contracts (above DD) | DLL |
|---|---|---|---|---|---|
| 25K Elite | $1,500 | $1,600 | 1 mini / 10 micro | 2 mini / 20 micro | No DLL |
| 50K Elite | $2,000 | $2,100 | 2 mini / 20 micro | 4 mini / 40 micro | No DLL |
| 100K Elite | $3,000 | $3,100 | 4 mini / 40 micro | 8 mini / 80 micro | No DLL |
| 150K Elite | $4,500 | $4,600 | **6 mini / 40 micro** (as published) | 12 mini / 120 micro | No DLL |
| 300K Elite | $6,000 | $6,100 | **8 mini / 60 micro** (as published) | 10 mini / 100 micro | No DLL |

- The values in bold do not follow the firm's own 10:1 ratio (§9 #3).
- Payouts are daily with an 80/20 split. Quoted: "If a payout brings the balance to $0, the Live Account will be closed."
- Inactivity: "at least one trade every 30 days".
- Quoted: "You cannot have Sim Funded and Live Funded accounts active at the same time." This applies household-wide.
- **Legacy Live** (accounts bought before Dec 3, 2025, may be chosen over Elite): a single combined live account with a balance-tiered DLL ($2,000 under $10,000 … $100,000) and max contracts "Set by Risk" below $60,000. It is not seeded as a preset; see https://help.tradeify.co/en/articles/12999764-legacy-live-program-complete-guide.

## 3. Trading-day reset

Sources:
- https://help.tradeify.co/en/articles/10468225-what-is-a-trading-day
- https://help.tradeify.co/en/articles/10495876-rules-permitted-times-to-trade
- DLL article

Quoted:
- "A trading day is counted as at least one trade placed from 6:00 P.M. ET to 5:00 P.M. ET the next day."
- DLL: "The DLL resets at the start of each trading session (6:00 PM ET)."
- Time zone: "Eastern Time (ET). This is Eastern Standard Time (EST) in winter and Eastern Daylight Time (EDT) in summer."

Close-out rules:
- Quoted: "All positions must be closed by 4:45 PM Eastern Time each trading day. … This applies to ALL account types including Live."
- Holidays: "Holiday shortened days: Close all positions by 12:59 PM ET". "Holiday market hours are announced in our Discord server."
- Open positions are auto-closed at the close: "This automatic closure will NOT result in account failure".
- No overnight or weekend holding.
- Reconciliation: "Daily account reconciliations occur between 5:00 p.m. and 6:00 p.m. ET." (Growth payout policy)
- Rithmic holiday half-days post at the next full close (§1).

YAML uses `time: "18:00", tz_as_published: "ET"` (the published session/DLL reset). The session ends at 17:00 ET, and positions must be flat by 16:45 ET.

## 4. Trade copying stance — `conditional` (own accounts only, same direction)

Copying across the **same person's accounts is explicitly allowed**. Quotes:

- WealthCharts Order Entry & Trade Copier (https://help.tradeify.co/en/articles/13825156-wealthcharts-order-entry-trade-copier):
  > "Copy trading (using Trade Copier or any method) is allowed on Tradeify, but you may only copy trades between accounts that you personally own and manage. Copying trades from or to accounts owned by other people is strictly prohibited and may result in account termination. All copied accounts must trade in the same direction — a copier configured to invert direction, or a lead account flattened while followers stay open, creates opposing positions across accounts and is a hedging violation."
- Tradovate Order Entry & Copy Trading (https://help.tradeify.co/en/articles/13196343-tradovate-order-entry-copy-trading):
  > "You may only group trade between accounts you own and manage. Copying other strategies or engaging in copy trading with others is not permitted. If you choose to use third-party trade copiers, you do so at your own risk. We are not responsible for any errors they may cause, and support for such issues is beyond our scope."
  > "Bracket orders (ATM Strategies) are NOT supported while Group Trading."
- Tradesea Setup Guide (https://help.tradeify.co/en/articles/14068317-tradesea-setup-guide):
  > "Note: Copy trading is explicitly allowed by Tradeify. Using a trade copier across your own accounts does not violate any Tradeify rules, provided every account trades in the same direction."
- Hedging (https://help.tradeify.co/en/articles/10495868-rules-hedging-correlated-products):
  > "Can I trade the same instrument on more than one account? A: Yes, as long as the positions are in the same direction. Copy trading is permitted."
- Funded Trader Agreement §6.7(b) (https://tradeify.co/funded-trader-agreement):
  > "Team trading, collaborative trading, group trading, the management of a Trader's account by or on behalf of any third party, and the copying or mirroring of trades between the Trader and any other person are strictly prohibited. … the copying or mirroring of the Trader's own trades across the Trader's own Trader-Controlled Accounts is permitted to the extent expressly allowed by the rules published at the Help Center referenced in Section 4.1, provided always that such activity does not result in Opposing Positions prohibited by Section 6.7(c)."
- Risk & Compliance FAQ (https://help.tradeify.co/en/articles/17069653-risk-compliance-guidelines-faq) lists "prohibited trade-copying" and "synchronized trading" among prohibited coordination "with others".

**Account-count limits.** Source: https://help.tradeify.co/en/articles/10468251-how-many-simulated-funded-accounts-can-i-have-at-once. Quoted:
- "An individual user may have up to 5 Simulated Funded Accounts at any time"
- "A household is also restricted to 5 … (exception: V2 Select 300K funded accounts sit on top of the 5, for up to 8 in total)"
- "up to 15 select evaluations per 30 days and as many Growth Evaluation accounts as you want".
- Elite: "up to 5 live accounts".
- One user account: "Only one Tradeify account is permitted per user" (Common FAQs).

**Other firms.**
- No rule was found that states whether trades may be *copied* to or from accounts at other firms.
- Opposing positions against accounts at other firms are prohibited. FTA §6.7(c)/(i), quoted: "Holding a position in a Trader-Controlled Account opposite a position in an External Account constitutes hedging". The firm "may rely on information provided to it by NinjaTrader … as evidence of Opposing Positions held on a Cross-Platform Account".
- Bots may not be used across firms (§5).
- So same-direction copying to other firms is **not stated** (§9 #4).

## 5. Automated trading stance — `conditional`

**(i) Classification:** `conditional` (YAML `automated_trading_allowed: conditional`).

Verbatim rule (this text is used as `automation_rule_text`). Source: Guidelines for Traders, https://help.tradeify.co/en/articles/10468318-guidelines-for-traders, accessed 2026-10-23:

> At Tradeify, we allow the use of bots and algorithms under certain conditions:
> Ownership: You must be able to prove that you are the sole owner of the bot or strategy, and that no one else has access to or is using it. This ensures that the bot/algorithm is not being shared with other traders or firms.
> We scan to ensure there are no similar orders on other accounts. We will also require a live video of you enabling the code on your own PC.
> Exclusive Use: While you may use the bot on your personal accounts, using it across multiple firms is against Tradeify's policy. The bot should be solely for your own use within Tradeify.
> No High-Frequency Trading (HFT) Bots: Personal bots are allowed as long as they are not high-frequency trading (HFT) bots. Tradeify has specific risk measures in place to detect and monitor such activities.
> Compliance and Verification: Tradeify reserves the right to request information or documentation if our risk measures flag your account for potential violations of these guidelines.

The same conditions appear in the Funded Trader Agreement §6.6 "Automated Bots". That version has no "live video" sentence and says "Compliance & Verification". Other quotes:
- Essential Rules: "Algorithmic Trading" section: "Allowed if:" "You own the strategy exclusively" / "Not shared with other traders/firms" / "Not high-frequency trading" / "Can prove ownership if requested"; "HFT Bots: High-frequency trading bots not allowed".
- Risk & Compliance FAQ, "Can I use bots, algorithms, or trade copiers?": "Automated trading is permitted only as expressly allowed, and traders remain fully responsible for its activity. Review may follow: use of prohibited high-frequency systems / unauthorized or unowned bots/strategies / sharing of proprietary strategies / coordinated automation across unrelated accounts / automation designed to exploit conditions or circumvent Risk controls / No trading technology exempts a trader from any Tradeify rule."

**(ii) Specific mentions:**
- **Semi-automated trading:** not mentioned.
- **Trade copiers:** mentioned and allowed between your own accounts (§4). Third-party copiers: "you do so at your own risk" (Tradovate article). Quoted: "Tradeify cannot resolve technical issues originating from third-party platforms (TradingView, Trade copiers, etc.)" (https://help.tradeify.co/en/articles/12270208-platform-connection-troubleshooting).
- **Webhooks / alerts:** not mentioned. A help-center search for "webhook" returned no articles. TradingView is supported only as a platform via Tradovate, and no alert-automation rule was found.
- **ATM / automatic brackets:** mentioned as platform features with no restriction:
  - Tradovate article: "ATM (Advanced Trade Management) Strategies enable the automatic placement of predefined Stop Loss and Profit Target orders once an Entry order is executed." Quoted: "Bracket orders (ATM Strategies) are NOT supported while Group Trading."
  - Tradesea: "You can also enable Auto Position Brackets to automatically attach stop-loss and take-profit to every new position."
  - Connection troubleshooting warns that ATM strategies across many accounts with group trading can hit Tradovate API rate limits.

**(iii) By phase:**
- The bot rule shows no phase distinction. The Guidelines and Essential Rules state it once for all accounts, and FTA §6.6 applies to the Trader.
- Related limits that do depend on phase:
  - The microscalping / 10-second payout rule "applies to funded accounts only. It does not apply during an evaluation."
  - The hedging rule "applies at all times and to every account type" (FTA §6.7(e)).

## 6. VPS / server / VPN and personal device

- Common FAQs (https://help.tradeify.co/en/articles/12268494-common-faqs):
  > "Can I use a VPN or VPS? You cannot log in with VPN/VPS. After logging in normally, you may use VPS for trading at your own risk."
- Terms of Use (https://tradeify.co/terms-of-use), "You agree not to: …":
  > "use a Virtual Private Server (“VPS”), virtual machine, virtualized environment, remote computing environment, or similar technology to access or use Tradeify’s Site, platform, accounts, or services where such technology obscures, alters, masks, or prevents Tradeify from reliably verifying the user’s device, network, location, identity, or account activity"
- Risk & Compliance FAQ: "Shared households, networks, workplaces, or devices do not, by themselves, establish a violation. … Review may follow where access methods appear designed to conceal true ownership … including through prohibited virtualization, remote-access, anonymization, or circumvention technologies."
- Platform guides (Quantower, Tradesea, Sierra Chart, R|Trader) recommend disabling a VPN for connection reasons ("VPNs add latency and can cause disconnections").

**Classification:** `vps_allowed: conditional`. A VPS for trading is allowed "at your own risk" after a normal (non-VPN/VPS) login. A VPS that hides the device or identity is forbidden.

**Personal device:** `personal_device_required: unknown`. No rule says that manual orders must come from the trader's personal device. For bots only, the Guidelines say: "We will also require a live video of you enabling the code on your own PC." It is not stated whether a bot running on a VPS meets "your own PC" (§9 #5). This matters for NinjaTrader 8 + All Win on a VPS.

## 7. Other rules that affect routing or risk

- **Hedging (prohibited, cross-account).** Quoted:
  - "You cannot hold opposing positions — one long, one short — on the same instrument or on two products from the same Product Group, whether those positions sit in a single account or are spread across several of your accounts."
  - Example: "Long ES and Short NQ. Both are Equity Index products".
  - Automated check: it breaches only if "Opposing positions exist … The hedge duration exceeds 10 seconds … Profit generated from the hedge exceeds $150". Cross-account hedging is reviewed separately.
  - Consequences include "All involved accounts set to violation status". A copier that inverts direction is a hedging violation.
  - Source: https://help.tradeify.co/en/articles/10495868-rules-hedging-correlated-products
- **News:** allowed. Quoted: "We do not have any rules against or guidelines around trading news events." (https://help.tradeify.co/en/articles/10495874-rules-news-trading). Select pages say "Tier 1 News Trading: You're allowed to trade during major news events."
- **Overnight / weekend holding:** not allowed. Positions must be flat by 4:45 PM ET (12:59 PM ET on holiday early closes), with auto-close at the close (§3).
- **Microscalping / 10-second rule** (funded, payout gate only): see §2. The FTA wording differs (§9 #6).
- **DCA:** Guidelines: "DCA is allowed, but it should always be part of a structured strategy." This conflicts with FTA §6.5 (§9 #7).
- **Martingale:** "Yes, as long as you're not hedging." (Common FAQs)
- **Other FTA §6.5 rules:** the FTA also asks for a "consistent trading plan in terms of size, stops, and targets, avoiding maximum or larger contract sizes on single trades, refraining from flipping contracts just to register a “trading day,”". This is not reflected in the help center (§9 #7).
- **Data feed:** FTA §6.3: "Trading based on an external or delayed data feed is not allowed."
- **Inactivity:** "at least one trade per week (Monday through Friday). This is per account" for evaluation and funded accounts. Elite Live: one filled trade every 30 days.
- **Max accounts:** see §4. Sim and Live cannot be held at the same time, household-wide.
- **Scaling plans:**
  - Select funded and Select 300K: see §2.4.
  - Growth and Lightning: full size from day one.
  - Elite: two tiers (below / above the drawdown amount).
- **Restricted countries:** see https://help.tradeify.co/en/articles/10495888-rules-restricted-countries (not captured; not needed for presets).
- **Auto-liquidation** (Common FAQs): "Auto-liquidation is a protective mechanism that closes your positions when you hit your daily loss limit or trailing drawdown threshold."

## 8. Machine-readable blocks

There are 32 blocks:
- Growth evaluation and Growth funded, 25K–150K (8).
- Select evaluation 25K–150K + 300K V2 (5).
- Select Flex funded 25K–150K + 300K V2 (5).
- Select Daily funded 25K–150K + 300K V2 (5).
- Lightning funded 25K–150K (4).
- Tradeify Elite live 25K–300K (5).

All broker connections share the same values (§1), so `platforms` lists every connection. For Select funded, `max_position` is the scaled maximum, and the starting tier and triggers are in `scaling`. Legacy versions (Select 300K V1, pre-Sep-2025 and pre-dashboard accounts, Legacy Live) are in the text and notes only.

#### Growth — evaluation — 25K
```yaml
firm: tradeify
plan: "GROWTH™ Evaluation"
phase: evaluation
size_label: "25K"
size_usd: "25000.00"
instruments: [MNQ, NQ]
platforms: [Tradovate, NinjaTrader (via Tradovate), TradingView (via Tradovate), Rithmic, Tradesea (Rithmic), Quantower (Rithmic), Sierra Chart (Rithmic), R|Trader (Rithmic), WealthCharts]
drawdown: { amount_usd: "1000.00", type: trailing_eod, stops_trailing_at: "Never locks on evaluation accounts ('Drawdown only locks on Sim Funded accounts')" }
daily_loss_limit: { amount_usd: "600.00", kind: soft_pause }   # 'Daily Loss Limit (soft breach)'; trading paused until next session
profit_target_usd: "1500.00"
consistency_rule: { text: null, threshold_pct: null }
max_position: { minis: 1, micros: 10, scaling: "No scaling; 10 micros = 1 mini; minis and micros may be combined within the limit" }
min_trading_days: 1   # 'GROWTH™ evaluation has a minimum of 1 trading day'
trading_day_reset: { time: "18:00", tz_as_published: "ET" }   # trading day "6:00 PM ET to 5:00 PM ET"; DLL "resets at the start of each trading session (6:00 PM ET)"; flat by 4:45 PM ET
copy_trading_allowed: conditional   # only between accounts you personally own and manage, all in the same direction; copying to/from other people prohibited; copying to/from other firms not stated (bots: not across multiple firms)
automated_trading_allowed: conditional   # bots/algos allowed if solely owned, not shared with other traders/firms, not HFT, not used across multiple firms; ownership may be verified
automation_rule_text: |
  At Tradeify, we allow the use of bots and algorithms under certain conditions:
  Ownership: You must be able to prove that you are the sole owner of the bot or strategy, and that no one else has access to or is using it. This ensures that the bot/algorithm is not being shared with other traders or firms.
  We scan to ensure there are no similar orders on other accounts. We will also require a live video of you enabling the code on your own PC.
  Exclusive Use: While you may use the bot on your personal accounts, using it across multiple firms is against Tradeify's policy. The bot should be solely for your own use within Tradeify.
  No High-Frequency Trading (HFT) Bots: Personal bots are allowed as long as they are not high-frequency trading (HFT) bots. Tradeify has specific risk measures in place to detect and monitor such activities.
  Compliance and Verification: Tradeify reserves the right to request information or documentation if our risk measures flag your account for potential violations of these guidelines.
vps_allowed: conditional   # FAQ: no VPN/VPS for login; VPS for trading 'at your own risk' after a normal login; Terms forbid VPS that obscures device/identity verification
personal_device_required: unknown   # not stated for manual orders; for bots the firm may 'require a live video of you enabling the code on your own PC'
notes: "No consistency rule on the evaluation. Weekly inactivity rule: at least one trade per week (Mon-Fri). Legacy max position (bought before Sep 12, 2025 8:00 AM ET): 50K 5/50, 100K 10/100, 150K 15/150."
status: verified
not_confirmed_fields: []
sources:
  - { field: drawdown, url: "https://help.tradeify.co/en/articles/10495915-growth-evaluation-accounts", accessed: "2026-10-23" }
  - { field: drawdown, url: "https://help.tradeify.co/en/articles/10495897-rules-trailing-max-drawdowns", accessed: "2026-10-23" }
  - { field: daily_loss_limit, url: "https://help.tradeify.co/en/articles/10495915-growth-evaluation-accounts", accessed: "2026-10-23" }
  - { field: daily_loss_limit, url: "https://help.tradeify.co/en/articles/10468321-rules-daily-loss-limit", accessed: "2026-10-23" }
  - { field: profit_target_usd, url: "https://help.tradeify.co/en/articles/10495915-growth-evaluation-accounts", accessed: "2026-10-23" }
  - { field: max_position, url: "https://help.tradeify.co/en/articles/10495915-growth-evaluation-accounts", accessed: "2026-10-23" }
  - { field: min_trading_days, url: "https://help.tradeify.co/en/articles/10495915-growth-evaluation-accounts", accessed: "2026-10-23" }
  - { field: drawdown, url: "https://tradeify.co/", accessed: "2026-10-23" }
  - { field: platforms, url: "https://help.tradeify.co/en/articles/10468221-supported-platforms", accessed: "2026-10-23" }
  - { field: trading_day_reset, url: "https://help.tradeify.co/en/articles/10468225-what-is-a-trading-day", accessed: "2026-10-23" }
  - { field: trading_day_reset, url: "https://help.tradeify.co/en/articles/10468321-rules-daily-loss-limit", accessed: "2026-10-23" }
  - { field: copy_trading_allowed, url: "https://help.tradeify.co/en/articles/10495868-rules-hedging-correlated-products", accessed: "2026-10-23" }
  - { field: copy_trading_allowed, url: "https://help.tradeify.co/en/articles/13196343-tradovate-order-entry-copy-trading", accessed: "2026-10-23" }
  - { field: copy_trading_allowed, url: "https://help.tradeify.co/en/articles/13825156-wealthcharts-order-entry-trade-copier", accessed: "2026-10-23" }
  - { field: copy_trading_allowed, url: "https://tradeify.co/funded-trader-agreement", accessed: "2026-10-23" }
  - { field: automated_trading_allowed, url: "https://help.tradeify.co/en/articles/10468318-guidelines-for-traders", accessed: "2026-10-23" }
  - { field: automated_trading_allowed, url: "https://tradeify.co/funded-trader-agreement", accessed: "2026-10-23" }
  - { field: vps_allowed, url: "https://help.tradeify.co/en/articles/12268494-common-faqs", accessed: "2026-10-23" }
  - { field: vps_allowed, url: "https://tradeify.co/terms-of-use", accessed: "2026-10-23" }
```

#### Growth — evaluation — 50K
```yaml
firm: tradeify
plan: "GROWTH™ Evaluation"
phase: evaluation
size_label: "50K"
size_usd: "50000.00"
instruments: [MNQ, NQ]
platforms: [Tradovate, NinjaTrader (via Tradovate), TradingView (via Tradovate), Rithmic, Tradesea (Rithmic), Quantower (Rithmic), Sierra Chart (Rithmic), R|Trader (Rithmic), WealthCharts]
drawdown: { amount_usd: "2000.00", type: trailing_eod, stops_trailing_at: "Never locks on evaluation accounts ('Drawdown only locks on Sim Funded accounts')" }
daily_loss_limit: { amount_usd: "1250.00", kind: soft_pause }   # 'Daily Loss Limit (soft breach)'; trading paused until next session
profit_target_usd: "3000.00"
consistency_rule: { text: null, threshold_pct: null }
max_position: { minis: 4, micros: 40, scaling: "No scaling; 10 micros = 1 mini; minis and micros may be combined within the limit" }
min_trading_days: 1   # 'GROWTH™ evaluation has a minimum of 1 trading day'
trading_day_reset: { time: "18:00", tz_as_published: "ET" }   # trading day "6:00 PM ET to 5:00 PM ET"; DLL "resets at the start of each trading session (6:00 PM ET)"; flat by 4:45 PM ET
copy_trading_allowed: conditional   # only between accounts you personally own and manage, all in the same direction; copying to/from other people prohibited; copying to/from other firms not stated (bots: not across multiple firms)
automated_trading_allowed: conditional   # bots/algos allowed if solely owned, not shared with other traders/firms, not HFT, not used across multiple firms; ownership may be verified
automation_rule_text: |
  At Tradeify, we allow the use of bots and algorithms under certain conditions:
  Ownership: You must be able to prove that you are the sole owner of the bot or strategy, and that no one else has access to or is using it. This ensures that the bot/algorithm is not being shared with other traders or firms.
  We scan to ensure there are no similar orders on other accounts. We will also require a live video of you enabling the code on your own PC.
  Exclusive Use: While you may use the bot on your personal accounts, using it across multiple firms is against Tradeify's policy. The bot should be solely for your own use within Tradeify.
  No High-Frequency Trading (HFT) Bots: Personal bots are allowed as long as they are not high-frequency trading (HFT) bots. Tradeify has specific risk measures in place to detect and monitor such activities.
  Compliance and Verification: Tradeify reserves the right to request information or documentation if our risk measures flag your account for potential violations of these guidelines.
vps_allowed: conditional   # FAQ: no VPN/VPS for login; VPS for trading 'at your own risk' after a normal login; Terms forbid VPS that obscures device/identity verification
personal_device_required: unknown   # not stated for manual orders; for bots the firm may 'require a live video of you enabling the code on your own PC'
notes: "No consistency rule on the evaluation. Weekly inactivity rule: at least one trade per week (Mon-Fri). Legacy max position (bought before Sep 12, 2025 8:00 AM ET): 50K 5/50, 100K 10/100, 150K 15/150."
status: verified
not_confirmed_fields: []
sources:
  - { field: drawdown, url: "https://help.tradeify.co/en/articles/10495915-growth-evaluation-accounts", accessed: "2026-10-23" }
  - { field: drawdown, url: "https://help.tradeify.co/en/articles/10495897-rules-trailing-max-drawdowns", accessed: "2026-10-23" }
  - { field: daily_loss_limit, url: "https://help.tradeify.co/en/articles/10495915-growth-evaluation-accounts", accessed: "2026-10-23" }
  - { field: daily_loss_limit, url: "https://help.tradeify.co/en/articles/10468321-rules-daily-loss-limit", accessed: "2026-10-23" }
  - { field: profit_target_usd, url: "https://help.tradeify.co/en/articles/10495915-growth-evaluation-accounts", accessed: "2026-10-23" }
  - { field: max_position, url: "https://help.tradeify.co/en/articles/10495915-growth-evaluation-accounts", accessed: "2026-10-23" }
  - { field: min_trading_days, url: "https://help.tradeify.co/en/articles/10495915-growth-evaluation-accounts", accessed: "2026-10-23" }
  - { field: drawdown, url: "https://tradeify.co/", accessed: "2026-10-23" }
  - { field: platforms, url: "https://help.tradeify.co/en/articles/10468221-supported-platforms", accessed: "2026-10-23" }
  - { field: trading_day_reset, url: "https://help.tradeify.co/en/articles/10468225-what-is-a-trading-day", accessed: "2026-10-23" }
  - { field: trading_day_reset, url: "https://help.tradeify.co/en/articles/10468321-rules-daily-loss-limit", accessed: "2026-10-23" }
  - { field: copy_trading_allowed, url: "https://help.tradeify.co/en/articles/10495868-rules-hedging-correlated-products", accessed: "2026-10-23" }
  - { field: copy_trading_allowed, url: "https://help.tradeify.co/en/articles/13196343-tradovate-order-entry-copy-trading", accessed: "2026-10-23" }
  - { field: copy_trading_allowed, url: "https://help.tradeify.co/en/articles/13825156-wealthcharts-order-entry-trade-copier", accessed: "2026-10-23" }
  - { field: copy_trading_allowed, url: "https://tradeify.co/funded-trader-agreement", accessed: "2026-10-23" }
  - { field: automated_trading_allowed, url: "https://help.tradeify.co/en/articles/10468318-guidelines-for-traders", accessed: "2026-10-23" }
  - { field: automated_trading_allowed, url: "https://tradeify.co/funded-trader-agreement", accessed: "2026-10-23" }
  - { field: vps_allowed, url: "https://help.tradeify.co/en/articles/12268494-common-faqs", accessed: "2026-10-23" }
  - { field: vps_allowed, url: "https://tradeify.co/terms-of-use", accessed: "2026-10-23" }
```

#### Growth — evaluation — 100K
```yaml
firm: tradeify
plan: "GROWTH™ Evaluation"
phase: evaluation
size_label: "100K"
size_usd: "100000.00"
instruments: [MNQ, NQ]
platforms: [Tradovate, NinjaTrader (via Tradovate), TradingView (via Tradovate), Rithmic, Tradesea (Rithmic), Quantower (Rithmic), Sierra Chart (Rithmic), R|Trader (Rithmic), WealthCharts]
drawdown: { amount_usd: "3500.00", type: trailing_eod, stops_trailing_at: "Never locks on evaluation accounts ('Drawdown only locks on Sim Funded accounts')" }
daily_loss_limit: { amount_usd: "2500.00", kind: soft_pause }   # 'Daily Loss Limit (soft breach)'; trading paused until next session
profit_target_usd: "6000.00"
consistency_rule: { text: null, threshold_pct: null }
max_position: { minis: 8, micros: 80, scaling: "No scaling; 10 micros = 1 mini; minis and micros may be combined within the limit" }
min_trading_days: 1   # 'GROWTH™ evaluation has a minimum of 1 trading day'
trading_day_reset: { time: "18:00", tz_as_published: "ET" }   # trading day "6:00 PM ET to 5:00 PM ET"; DLL "resets at the start of each trading session (6:00 PM ET)"; flat by 4:45 PM ET
copy_trading_allowed: conditional   # only between accounts you personally own and manage, all in the same direction; copying to/from other people prohibited; copying to/from other firms not stated (bots: not across multiple firms)
automated_trading_allowed: conditional   # bots/algos allowed if solely owned, not shared with other traders/firms, not HFT, not used across multiple firms; ownership may be verified
automation_rule_text: |
  At Tradeify, we allow the use of bots and algorithms under certain conditions:
  Ownership: You must be able to prove that you are the sole owner of the bot or strategy, and that no one else has access to or is using it. This ensures that the bot/algorithm is not being shared with other traders or firms.
  We scan to ensure there are no similar orders on other accounts. We will also require a live video of you enabling the code on your own PC.
  Exclusive Use: While you may use the bot on your personal accounts, using it across multiple firms is against Tradeify's policy. The bot should be solely for your own use within Tradeify.
  No High-Frequency Trading (HFT) Bots: Personal bots are allowed as long as they are not high-frequency trading (HFT) bots. Tradeify has specific risk measures in place to detect and monitor such activities.
  Compliance and Verification: Tradeify reserves the right to request information or documentation if our risk measures flag your account for potential violations of these guidelines.
vps_allowed: conditional   # FAQ: no VPN/VPS for login; VPS for trading 'at your own risk' after a normal login; Terms forbid VPS that obscures device/identity verification
personal_device_required: unknown   # not stated for manual orders; for bots the firm may 'require a live video of you enabling the code on your own PC'
notes: "No consistency rule on the evaluation. Weekly inactivity rule: at least one trade per week (Mon-Fri). Legacy max position (bought before Sep 12, 2025 8:00 AM ET): 50K 5/50, 100K 10/100, 150K 15/150."
status: verified
not_confirmed_fields: []
sources:
  - { field: drawdown, url: "https://help.tradeify.co/en/articles/10495915-growth-evaluation-accounts", accessed: "2026-10-23" }
  - { field: drawdown, url: "https://help.tradeify.co/en/articles/10495897-rules-trailing-max-drawdowns", accessed: "2026-10-23" }
  - { field: daily_loss_limit, url: "https://help.tradeify.co/en/articles/10495915-growth-evaluation-accounts", accessed: "2026-10-23" }
  - { field: daily_loss_limit, url: "https://help.tradeify.co/en/articles/10468321-rules-daily-loss-limit", accessed: "2026-10-23" }
  - { field: profit_target_usd, url: "https://help.tradeify.co/en/articles/10495915-growth-evaluation-accounts", accessed: "2026-10-23" }
  - { field: max_position, url: "https://help.tradeify.co/en/articles/10495915-growth-evaluation-accounts", accessed: "2026-10-23" }
  - { field: min_trading_days, url: "https://help.tradeify.co/en/articles/10495915-growth-evaluation-accounts", accessed: "2026-10-23" }
  - { field: drawdown, url: "https://tradeify.co/", accessed: "2026-10-23" }
  - { field: platforms, url: "https://help.tradeify.co/en/articles/10468221-supported-platforms", accessed: "2026-10-23" }
  - { field: trading_day_reset, url: "https://help.tradeify.co/en/articles/10468225-what-is-a-trading-day", accessed: "2026-10-23" }
  - { field: trading_day_reset, url: "https://help.tradeify.co/en/articles/10468321-rules-daily-loss-limit", accessed: "2026-10-23" }
  - { field: copy_trading_allowed, url: "https://help.tradeify.co/en/articles/10495868-rules-hedging-correlated-products", accessed: "2026-10-23" }
  - { field: copy_trading_allowed, url: "https://help.tradeify.co/en/articles/13196343-tradovate-order-entry-copy-trading", accessed: "2026-10-23" }
  - { field: copy_trading_allowed, url: "https://help.tradeify.co/en/articles/13825156-wealthcharts-order-entry-trade-copier", accessed: "2026-10-23" }
  - { field: copy_trading_allowed, url: "https://tradeify.co/funded-trader-agreement", accessed: "2026-10-23" }
  - { field: automated_trading_allowed, url: "https://help.tradeify.co/en/articles/10468318-guidelines-for-traders", accessed: "2026-10-23" }
  - { field: automated_trading_allowed, url: "https://tradeify.co/funded-trader-agreement", accessed: "2026-10-23" }
  - { field: vps_allowed, url: "https://help.tradeify.co/en/articles/12268494-common-faqs", accessed: "2026-10-23" }
  - { field: vps_allowed, url: "https://tradeify.co/terms-of-use", accessed: "2026-10-23" }
```

#### Growth — evaluation — 150K
```yaml
firm: tradeify
plan: "GROWTH™ Evaluation"
phase: evaluation
size_label: "150K"
size_usd: "150000.00"
instruments: [MNQ, NQ]
platforms: [Tradovate, NinjaTrader (via Tradovate), TradingView (via Tradovate), Rithmic, Tradesea (Rithmic), Quantower (Rithmic), Sierra Chart (Rithmic), R|Trader (Rithmic), WealthCharts]
drawdown: { amount_usd: "5000.00", type: trailing_eod, stops_trailing_at: "Never locks on evaluation accounts ('Drawdown only locks on Sim Funded accounts')" }
daily_loss_limit: { amount_usd: "3750.00", kind: soft_pause }   # 'Daily Loss Limit (soft breach)'; trading paused until next session
profit_target_usd: "9000.00"
consistency_rule: { text: null, threshold_pct: null }
max_position: { minis: 12, micros: 120, scaling: "No scaling; 10 micros = 1 mini; minis and micros may be combined within the limit" }
min_trading_days: 1   # 'GROWTH™ evaluation has a minimum of 1 trading day'
trading_day_reset: { time: "18:00", tz_as_published: "ET" }   # trading day "6:00 PM ET to 5:00 PM ET"; DLL "resets at the start of each trading session (6:00 PM ET)"; flat by 4:45 PM ET
copy_trading_allowed: conditional   # only between accounts you personally own and manage, all in the same direction; copying to/from other people prohibited; copying to/from other firms not stated (bots: not across multiple firms)
automated_trading_allowed: conditional   # bots/algos allowed if solely owned, not shared with other traders/firms, not HFT, not used across multiple firms; ownership may be verified
automation_rule_text: |
  At Tradeify, we allow the use of bots and algorithms under certain conditions:
  Ownership: You must be able to prove that you are the sole owner of the bot or strategy, and that no one else has access to or is using it. This ensures that the bot/algorithm is not being shared with other traders or firms.
  We scan to ensure there are no similar orders on other accounts. We will also require a live video of you enabling the code on your own PC.
  Exclusive Use: While you may use the bot on your personal accounts, using it across multiple firms is against Tradeify's policy. The bot should be solely for your own use within Tradeify.
  No High-Frequency Trading (HFT) Bots: Personal bots are allowed as long as they are not high-frequency trading (HFT) bots. Tradeify has specific risk measures in place to detect and monitor such activities.
  Compliance and Verification: Tradeify reserves the right to request information or documentation if our risk measures flag your account for potential violations of these guidelines.
vps_allowed: conditional   # FAQ: no VPN/VPS for login; VPS for trading 'at your own risk' after a normal login; Terms forbid VPS that obscures device/identity verification
personal_device_required: unknown   # not stated for manual orders; for bots the firm may 'require a live video of you enabling the code on your own PC'
notes: "No consistency rule on the evaluation. Weekly inactivity rule: at least one trade per week (Mon-Fri). Legacy max position (bought before Sep 12, 2025 8:00 AM ET): 50K 5/50, 100K 10/100, 150K 15/150."
status: verified
not_confirmed_fields: []
sources:
  - { field: drawdown, url: "https://help.tradeify.co/en/articles/10495915-growth-evaluation-accounts", accessed: "2026-10-23" }
  - { field: drawdown, url: "https://help.tradeify.co/en/articles/10495897-rules-trailing-max-drawdowns", accessed: "2026-10-23" }
  - { field: daily_loss_limit, url: "https://help.tradeify.co/en/articles/10495915-growth-evaluation-accounts", accessed: "2026-10-23" }
  - { field: daily_loss_limit, url: "https://help.tradeify.co/en/articles/10468321-rules-daily-loss-limit", accessed: "2026-10-23" }
  - { field: profit_target_usd, url: "https://help.tradeify.co/en/articles/10495915-growth-evaluation-accounts", accessed: "2026-10-23" }
  - { field: max_position, url: "https://help.tradeify.co/en/articles/10495915-growth-evaluation-accounts", accessed: "2026-10-23" }
  - { field: min_trading_days, url: "https://help.tradeify.co/en/articles/10495915-growth-evaluation-accounts", accessed: "2026-10-23" }
  - { field: drawdown, url: "https://tradeify.co/", accessed: "2026-10-23" }
  - { field: platforms, url: "https://help.tradeify.co/en/articles/10468221-supported-platforms", accessed: "2026-10-23" }
  - { field: trading_day_reset, url: "https://help.tradeify.co/en/articles/10468225-what-is-a-trading-day", accessed: "2026-10-23" }
  - { field: trading_day_reset, url: "https://help.tradeify.co/en/articles/10468321-rules-daily-loss-limit", accessed: "2026-10-23" }
  - { field: copy_trading_allowed, url: "https://help.tradeify.co/en/articles/10495868-rules-hedging-correlated-products", accessed: "2026-10-23" }
  - { field: copy_trading_allowed, url: "https://help.tradeify.co/en/articles/13196343-tradovate-order-entry-copy-trading", accessed: "2026-10-23" }
  - { field: copy_trading_allowed, url: "https://help.tradeify.co/en/articles/13825156-wealthcharts-order-entry-trade-copier", accessed: "2026-10-23" }
  - { field: copy_trading_allowed, url: "https://tradeify.co/funded-trader-agreement", accessed: "2026-10-23" }
  - { field: automated_trading_allowed, url: "https://help.tradeify.co/en/articles/10468318-guidelines-for-traders", accessed: "2026-10-23" }
  - { field: automated_trading_allowed, url: "https://tradeify.co/funded-trader-agreement", accessed: "2026-10-23" }
  - { field: vps_allowed, url: "https://help.tradeify.co/en/articles/12268494-common-faqs", accessed: "2026-10-23" }
  - { field: vps_allowed, url: "https://tradeify.co/terms-of-use", accessed: "2026-10-23" }
```

#### Growth — funded — 25K
```yaml
firm: tradeify
plan: "GROWTH™ Sim Funded"
phase: funded
size_label: "25K"
size_usd: "25000.00"
instruments: [MNQ, NQ]
platforms: [Tradovate, NinjaTrader (via Tradovate), TradingView (via Tradovate), Rithmic, Tradesea (Rithmic), Quantower (Rithmic), Sierra Chart (Rithmic), R|Trader (Rithmic), WealthCharts]
drawdown: { amount_usd: "1000.00", type: trailing_eod, stops_trailing_at: "Locks when EOD balance reaches $26,100, or immediately when a payout is requested (whichever first); floor then fixed at starting balance + $100" }
daily_loss_limit: { amount_usd: "600.00", kind: soft_pause }   # initial DLL; increases to $1,000 from the next session once balance reaches $26,500 (6% profit)
profit_target_usd: null
consistency_rule: { text: "Growth Sim Funded Accounts must follow a 35% Consistency Rule (largest day ≤ 35% of total profit, per payout request; resets after an approved payout)", threshold_pct: "35" }
max_position: { minis: 1, micros: 10, scaling: "'Growth and Lightning use their maximum position size from day one'" }
min_trading_days: null   # no activation minimum; payout needs 5+ trading days each with profit > $100 (resets after each payout)
trading_day_reset: { time: "18:00", tz_as_published: "ET" }   # trading day "6:00 PM ET to 5:00 PM ET"; DLL "resets at the start of each trading session (6:00 PM ET)"; flat by 4:45 PM ET
copy_trading_allowed: conditional   # only between accounts you personally own and manage, all in the same direction; copying to/from other people prohibited; copying to/from other firms not stated (bots: not across multiple firms)
automated_trading_allowed: conditional   # bots/algos allowed if solely owned, not shared with other traders/firms, not HFT, not used across multiple firms; ownership may be verified
automation_rule_text: |
  At Tradeify, we allow the use of bots and algorithms under certain conditions:
  Ownership: You must be able to prove that you are the sole owner of the bot or strategy, and that no one else has access to or is using it. This ensures that the bot/algorithm is not being shared with other traders or firms.
  We scan to ensure there are no similar orders on other accounts. We will also require a live video of you enabling the code on your own PC.
  Exclusive Use: While you may use the bot on your personal accounts, using it across multiple firms is against Tradeify's policy. The bot should be solely for your own use within Tradeify.
  No High-Frequency Trading (HFT) Bots: Personal bots are allowed as long as they are not high-frequency trading (HFT) bots. Tradeify has specific risk measures in place to detect and monitor such activities.
  Compliance and Verification: Tradeify reserves the right to request information or documentation if our risk measures flag your account for potential violations of these guidelines.
vps_allowed: conditional   # FAQ: no VPN/VPS for login; VPS for trading 'at your own risk' after a normal login; Terms forbid VPS that obscures device/identity verification
personal_device_required: unknown   # not stated for manual orders; for bots the firm may 'require a live video of you enabling the code on your own PC'
notes: "Payout min balance $26,500; 10-second rule on funded accounts (over 50% of trades and over 50% of profit from trades held > 10 s) gates payouts only. Drawdown amount from the homepage Growth funded card ('Max Drawdown (EOD)') and the drawdown lock table. Legacy accounts (before Sep 12, 2025): DLL removed at 6% profit."
status: verified
not_confirmed_fields: []
sources:
  - { field: drawdown, url: "https://tradeify.co/", accessed: "2026-10-23" }
  - { field: drawdown, url: "https://help.tradeify.co/en/articles/10495897-rules-trailing-max-drawdowns", accessed: "2026-10-23" }
  - { field: daily_loss_limit, url: "https://help.tradeify.co/en/articles/10468321-rules-daily-loss-limit", accessed: "2026-10-23" }
  - { field: daily_loss_limit, url: "https://help.tradeify.co/en/articles/12268167-essential-trading-rules-overview", accessed: "2026-10-23" }
  - { field: daily_loss_limit, url: "https://tradeify.co/", accessed: "2026-10-23" }
  - { field: consistency_rule, url: "https://help.tradeify.co/en/articles/10468320-rules-consistency-rule", accessed: "2026-10-23" }
  - { field: consistency_rule, url: "https://help.tradeify.co/en/articles/11083796-growth-funded-account-payout-policy", accessed: "2026-10-23" }
  - { field: max_position, url: "https://help.tradeify.co/en/articles/12268167-essential-trading-rules-overview", accessed: "2026-10-23" }
  - { field: max_position, url: "https://tradeify.co/", accessed: "2026-10-23" }
  - { field: payout, url: "https://help.tradeify.co/en/articles/11083796-growth-funded-account-payout-policy", accessed: "2026-10-23" }
  - { field: platforms, url: "https://help.tradeify.co/en/articles/10468221-supported-platforms", accessed: "2026-10-23" }
  - { field: trading_day_reset, url: "https://help.tradeify.co/en/articles/10468225-what-is-a-trading-day", accessed: "2026-10-23" }
  - { field: trading_day_reset, url: "https://help.tradeify.co/en/articles/10468321-rules-daily-loss-limit", accessed: "2026-10-23" }
  - { field: copy_trading_allowed, url: "https://help.tradeify.co/en/articles/10495868-rules-hedging-correlated-products", accessed: "2026-10-23" }
  - { field: copy_trading_allowed, url: "https://help.tradeify.co/en/articles/13196343-tradovate-order-entry-copy-trading", accessed: "2026-10-23" }
  - { field: copy_trading_allowed, url: "https://help.tradeify.co/en/articles/13825156-wealthcharts-order-entry-trade-copier", accessed: "2026-10-23" }
  - { field: copy_trading_allowed, url: "https://tradeify.co/funded-trader-agreement", accessed: "2026-10-23" }
  - { field: automated_trading_allowed, url: "https://help.tradeify.co/en/articles/10468318-guidelines-for-traders", accessed: "2026-10-23" }
  - { field: automated_trading_allowed, url: "https://tradeify.co/funded-trader-agreement", accessed: "2026-10-23" }
  - { field: vps_allowed, url: "https://help.tradeify.co/en/articles/12268494-common-faqs", accessed: "2026-10-23" }
  - { field: vps_allowed, url: "https://tradeify.co/terms-of-use", accessed: "2026-10-23" }
```

#### Growth — funded — 50K
```yaml
firm: tradeify
plan: "GROWTH™ Sim Funded"
phase: funded
size_label: "50K"
size_usd: "50000.00"
instruments: [MNQ, NQ]
platforms: [Tradovate, NinjaTrader (via Tradovate), TradingView (via Tradovate), Rithmic, Tradesea (Rithmic), Quantower (Rithmic), Sierra Chart (Rithmic), R|Trader (Rithmic), WealthCharts]
drawdown: { amount_usd: "2000.00", type: trailing_eod, stops_trailing_at: "Locks when EOD balance reaches $52,100, or immediately when a payout is requested (whichever first); floor then fixed at starting balance + $100" }
daily_loss_limit: { amount_usd: "1250.00", kind: soft_pause }   # initial DLL; increases to $2,000 from the next session once balance reaches $53,000 (6% profit)
profit_target_usd: null
consistency_rule: { text: "Growth Sim Funded Accounts must follow a 35% Consistency Rule (largest day ≤ 35% of total profit, per payout request; resets after an approved payout)", threshold_pct: "35" }
max_position: { minis: 4, micros: 40, scaling: "'Growth and Lightning use their maximum position size from day one'" }
min_trading_days: null   # no activation minimum; payout needs 5+ trading days each with profit > $150 (resets after each payout)
trading_day_reset: { time: "18:00", tz_as_published: "ET" }   # trading day "6:00 PM ET to 5:00 PM ET"; DLL "resets at the start of each trading session (6:00 PM ET)"; flat by 4:45 PM ET
copy_trading_allowed: conditional   # only between accounts you personally own and manage, all in the same direction; copying to/from other people prohibited; copying to/from other firms not stated (bots: not across multiple firms)
automated_trading_allowed: conditional   # bots/algos allowed if solely owned, not shared with other traders/firms, not HFT, not used across multiple firms; ownership may be verified
automation_rule_text: |
  At Tradeify, we allow the use of bots and algorithms under certain conditions:
  Ownership: You must be able to prove that you are the sole owner of the bot or strategy, and that no one else has access to or is using it. This ensures that the bot/algorithm is not being shared with other traders or firms.
  We scan to ensure there are no similar orders on other accounts. We will also require a live video of you enabling the code on your own PC.
  Exclusive Use: While you may use the bot on your personal accounts, using it across multiple firms is against Tradeify's policy. The bot should be solely for your own use within Tradeify.
  No High-Frequency Trading (HFT) Bots: Personal bots are allowed as long as they are not high-frequency trading (HFT) bots. Tradeify has specific risk measures in place to detect and monitor such activities.
  Compliance and Verification: Tradeify reserves the right to request information or documentation if our risk measures flag your account for potential violations of these guidelines.
vps_allowed: conditional   # FAQ: no VPN/VPS for login; VPS for trading 'at your own risk' after a normal login; Terms forbid VPS that obscures device/identity verification
personal_device_required: unknown   # not stated for manual orders; for bots the firm may 'require a live video of you enabling the code on your own PC'
notes: "Payout min balance $53,000; 10-second rule on funded accounts (over 50% of trades and over 50% of profit from trades held > 10 s) gates payouts only. Drawdown amount from the homepage Growth funded card ('Max Drawdown (EOD)') and the drawdown lock table. Legacy accounts (before Sep 12, 2025): DLL removed at 6% profit."
status: verified
not_confirmed_fields: []
sources:
  - { field: drawdown, url: "https://tradeify.co/", accessed: "2026-10-23" }
  - { field: drawdown, url: "https://help.tradeify.co/en/articles/10495897-rules-trailing-max-drawdowns", accessed: "2026-10-23" }
  - { field: daily_loss_limit, url: "https://help.tradeify.co/en/articles/10468321-rules-daily-loss-limit", accessed: "2026-10-23" }
  - { field: daily_loss_limit, url: "https://help.tradeify.co/en/articles/12268167-essential-trading-rules-overview", accessed: "2026-10-23" }
  - { field: daily_loss_limit, url: "https://tradeify.co/", accessed: "2026-10-23" }
  - { field: consistency_rule, url: "https://help.tradeify.co/en/articles/10468320-rules-consistency-rule", accessed: "2026-10-23" }
  - { field: consistency_rule, url: "https://help.tradeify.co/en/articles/11083796-growth-funded-account-payout-policy", accessed: "2026-10-23" }
  - { field: max_position, url: "https://help.tradeify.co/en/articles/12268167-essential-trading-rules-overview", accessed: "2026-10-23" }
  - { field: max_position, url: "https://tradeify.co/", accessed: "2026-10-23" }
  - { field: payout, url: "https://help.tradeify.co/en/articles/11083796-growth-funded-account-payout-policy", accessed: "2026-10-23" }
  - { field: platforms, url: "https://help.tradeify.co/en/articles/10468221-supported-platforms", accessed: "2026-10-23" }
  - { field: trading_day_reset, url: "https://help.tradeify.co/en/articles/10468225-what-is-a-trading-day", accessed: "2026-10-23" }
  - { field: trading_day_reset, url: "https://help.tradeify.co/en/articles/10468321-rules-daily-loss-limit", accessed: "2026-10-23" }
  - { field: copy_trading_allowed, url: "https://help.tradeify.co/en/articles/10495868-rules-hedging-correlated-products", accessed: "2026-10-23" }
  - { field: copy_trading_allowed, url: "https://help.tradeify.co/en/articles/13196343-tradovate-order-entry-copy-trading", accessed: "2026-10-23" }
  - { field: copy_trading_allowed, url: "https://help.tradeify.co/en/articles/13825156-wealthcharts-order-entry-trade-copier", accessed: "2026-10-23" }
  - { field: copy_trading_allowed, url: "https://tradeify.co/funded-trader-agreement", accessed: "2026-10-23" }
  - { field: automated_trading_allowed, url: "https://help.tradeify.co/en/articles/10468318-guidelines-for-traders", accessed: "2026-10-23" }
  - { field: automated_trading_allowed, url: "https://tradeify.co/funded-trader-agreement", accessed: "2026-10-23" }
  - { field: vps_allowed, url: "https://help.tradeify.co/en/articles/12268494-common-faqs", accessed: "2026-10-23" }
  - { field: vps_allowed, url: "https://tradeify.co/terms-of-use", accessed: "2026-10-23" }
```

#### Growth — funded — 100K
```yaml
firm: tradeify
plan: "GROWTH™ Sim Funded"
phase: funded
size_label: "100K"
size_usd: "100000.00"
instruments: [MNQ, NQ]
platforms: [Tradovate, NinjaTrader (via Tradovate), TradingView (via Tradovate), Rithmic, Tradesea (Rithmic), Quantower (Rithmic), Sierra Chart (Rithmic), R|Trader (Rithmic), WealthCharts]
drawdown: { amount_usd: "3500.00", type: trailing_eod, stops_trailing_at: "Locks when EOD balance reaches $103,600, or immediately when a payout is requested (whichever first); floor then fixed at starting balance + $100" }
daily_loss_limit: { amount_usd: "2500.00", kind: soft_pause }   # initial DLL; increases to $3,500 from the next session once balance reaches $106,000 (6% profit)
profit_target_usd: null
consistency_rule: { text: "Growth Sim Funded Accounts must follow a 35% Consistency Rule (largest day ≤ 35% of total profit, per payout request; resets after an approved payout)", threshold_pct: "35" }
max_position: { minis: 8, micros: 80, scaling: "'Growth and Lightning use their maximum position size from day one'" }
min_trading_days: null   # no activation minimum; payout needs 5+ trading days each with profit > $200 (resets after each payout)
trading_day_reset: { time: "18:00", tz_as_published: "ET" }   # trading day "6:00 PM ET to 5:00 PM ET"; DLL "resets at the start of each trading session (6:00 PM ET)"; flat by 4:45 PM ET
copy_trading_allowed: conditional   # only between accounts you personally own and manage, all in the same direction; copying to/from other people prohibited; copying to/from other firms not stated (bots: not across multiple firms)
automated_trading_allowed: conditional   # bots/algos allowed if solely owned, not shared with other traders/firms, not HFT, not used across multiple firms; ownership may be verified
automation_rule_text: |
  At Tradeify, we allow the use of bots and algorithms under certain conditions:
  Ownership: You must be able to prove that you are the sole owner of the bot or strategy, and that no one else has access to or is using it. This ensures that the bot/algorithm is not being shared with other traders or firms.
  We scan to ensure there are no similar orders on other accounts. We will also require a live video of you enabling the code on your own PC.
  Exclusive Use: While you may use the bot on your personal accounts, using it across multiple firms is against Tradeify's policy. The bot should be solely for your own use within Tradeify.
  No High-Frequency Trading (HFT) Bots: Personal bots are allowed as long as they are not high-frequency trading (HFT) bots. Tradeify has specific risk measures in place to detect and monitor such activities.
  Compliance and Verification: Tradeify reserves the right to request information or documentation if our risk measures flag your account for potential violations of these guidelines.
vps_allowed: conditional   # FAQ: no VPN/VPS for login; VPS for trading 'at your own risk' after a normal login; Terms forbid VPS that obscures device/identity verification
personal_device_required: unknown   # not stated for manual orders; for bots the firm may 'require a live video of you enabling the code on your own PC'
notes: "Payout min balance $104,500; 10-second rule on funded accounts (over 50% of trades and over 50% of profit from trades held > 10 s) gates payouts only. Drawdown amount from the homepage Growth funded card ('Max Drawdown (EOD)') and the drawdown lock table. Legacy accounts (before Sep 12, 2025): DLL removed at 6% profit."
status: verified
not_confirmed_fields: []
sources:
  - { field: drawdown, url: "https://tradeify.co/", accessed: "2026-10-23" }
  - { field: drawdown, url: "https://help.tradeify.co/en/articles/10495897-rules-trailing-max-drawdowns", accessed: "2026-10-23" }
  - { field: daily_loss_limit, url: "https://help.tradeify.co/en/articles/10468321-rules-daily-loss-limit", accessed: "2026-10-23" }
  - { field: daily_loss_limit, url: "https://help.tradeify.co/en/articles/12268167-essential-trading-rules-overview", accessed: "2026-10-23" }
  - { field: daily_loss_limit, url: "https://tradeify.co/", accessed: "2026-10-23" }
  - { field: consistency_rule, url: "https://help.tradeify.co/en/articles/10468320-rules-consistency-rule", accessed: "2026-10-23" }
  - { field: consistency_rule, url: "https://help.tradeify.co/en/articles/11083796-growth-funded-account-payout-policy", accessed: "2026-10-23" }
  - { field: max_position, url: "https://help.tradeify.co/en/articles/12268167-essential-trading-rules-overview", accessed: "2026-10-23" }
  - { field: max_position, url: "https://tradeify.co/", accessed: "2026-10-23" }
  - { field: payout, url: "https://help.tradeify.co/en/articles/11083796-growth-funded-account-payout-policy", accessed: "2026-10-23" }
  - { field: platforms, url: "https://help.tradeify.co/en/articles/10468221-supported-platforms", accessed: "2026-10-23" }
  - { field: trading_day_reset, url: "https://help.tradeify.co/en/articles/10468225-what-is-a-trading-day", accessed: "2026-10-23" }
  - { field: trading_day_reset, url: "https://help.tradeify.co/en/articles/10468321-rules-daily-loss-limit", accessed: "2026-10-23" }
  - { field: copy_trading_allowed, url: "https://help.tradeify.co/en/articles/10495868-rules-hedging-correlated-products", accessed: "2026-10-23" }
  - { field: copy_trading_allowed, url: "https://help.tradeify.co/en/articles/13196343-tradovate-order-entry-copy-trading", accessed: "2026-10-23" }
  - { field: copy_trading_allowed, url: "https://help.tradeify.co/en/articles/13825156-wealthcharts-order-entry-trade-copier", accessed: "2026-10-23" }
  - { field: copy_trading_allowed, url: "https://tradeify.co/funded-trader-agreement", accessed: "2026-10-23" }
  - { field: automated_trading_allowed, url: "https://help.tradeify.co/en/articles/10468318-guidelines-for-traders", accessed: "2026-10-23" }
  - { field: automated_trading_allowed, url: "https://tradeify.co/funded-trader-agreement", accessed: "2026-10-23" }
  - { field: vps_allowed, url: "https://help.tradeify.co/en/articles/12268494-common-faqs", accessed: "2026-10-23" }
  - { field: vps_allowed, url: "https://tradeify.co/terms-of-use", accessed: "2026-10-23" }
```

#### Growth — funded — 150K
```yaml
firm: tradeify
plan: "GROWTH™ Sim Funded"
phase: funded
size_label: "150K"
size_usd: "150000.00"
instruments: [MNQ, NQ]
platforms: [Tradovate, NinjaTrader (via Tradovate), TradingView (via Tradovate), Rithmic, Tradesea (Rithmic), Quantower (Rithmic), Sierra Chart (Rithmic), R|Trader (Rithmic), WealthCharts]
drawdown: { amount_usd: "5000.00", type: trailing_eod, stops_trailing_at: "Locks when EOD balance reaches $155,100, or immediately when a payout is requested (whichever first); floor then fixed at starting balance + $100" }
daily_loss_limit: { amount_usd: "3750.00", kind: soft_pause }   # initial DLL; increases to $5,000 from the next session once balance reaches $159,000 (6% profit)
profit_target_usd: null
consistency_rule: { text: "Growth Sim Funded Accounts must follow a 35% Consistency Rule (largest day ≤ 35% of total profit, per payout request; resets after an approved payout)", threshold_pct: "35" }
max_position: { minis: 12, micros: 120, scaling: "'Growth and Lightning use their maximum position size from day one'" }
min_trading_days: null   # no activation minimum; payout needs 5+ trading days each with profit > $250 (resets after each payout)
trading_day_reset: { time: "18:00", tz_as_published: "ET" }   # trading day "6:00 PM ET to 5:00 PM ET"; DLL "resets at the start of each trading session (6:00 PM ET)"; flat by 4:45 PM ET
copy_trading_allowed: conditional   # only between accounts you personally own and manage, all in the same direction; copying to/from other people prohibited; copying to/from other firms not stated (bots: not across multiple firms)
automated_trading_allowed: conditional   # bots/algos allowed if solely owned, not shared with other traders/firms, not HFT, not used across multiple firms; ownership may be verified
automation_rule_text: |
  At Tradeify, we allow the use of bots and algorithms under certain conditions:
  Ownership: You must be able to prove that you are the sole owner of the bot or strategy, and that no one else has access to or is using it. This ensures that the bot/algorithm is not being shared with other traders or firms.
  We scan to ensure there are no similar orders on other accounts. We will also require a live video of you enabling the code on your own PC.
  Exclusive Use: While you may use the bot on your personal accounts, using it across multiple firms is against Tradeify's policy. The bot should be solely for your own use within Tradeify.
  No High-Frequency Trading (HFT) Bots: Personal bots are allowed as long as they are not high-frequency trading (HFT) bots. Tradeify has specific risk measures in place to detect and monitor such activities.
  Compliance and Verification: Tradeify reserves the right to request information or documentation if our risk measures flag your account for potential violations of these guidelines.
vps_allowed: conditional   # FAQ: no VPN/VPS for login; VPS for trading 'at your own risk' after a normal login; Terms forbid VPS that obscures device/identity verification
personal_device_required: unknown   # not stated for manual orders; for bots the firm may 'require a live video of you enabling the code on your own PC'
notes: "Payout min balance $156,500; 10-second rule on funded accounts (over 50% of trades and over 50% of profit from trades held > 10 s) gates payouts only. Drawdown amount from the homepage Growth funded card ('Max Drawdown (EOD)') and the drawdown lock table. Legacy accounts (before Sep 12, 2025): DLL removed at 6% profit."
status: verified
not_confirmed_fields: []
sources:
  - { field: drawdown, url: "https://tradeify.co/", accessed: "2026-10-23" }
  - { field: drawdown, url: "https://help.tradeify.co/en/articles/10495897-rules-trailing-max-drawdowns", accessed: "2026-10-23" }
  - { field: daily_loss_limit, url: "https://help.tradeify.co/en/articles/10468321-rules-daily-loss-limit", accessed: "2026-10-23" }
  - { field: daily_loss_limit, url: "https://help.tradeify.co/en/articles/12268167-essential-trading-rules-overview", accessed: "2026-10-23" }
  - { field: daily_loss_limit, url: "https://tradeify.co/", accessed: "2026-10-23" }
  - { field: consistency_rule, url: "https://help.tradeify.co/en/articles/10468320-rules-consistency-rule", accessed: "2026-10-23" }
  - { field: consistency_rule, url: "https://help.tradeify.co/en/articles/11083796-growth-funded-account-payout-policy", accessed: "2026-10-23" }
  - { field: max_position, url: "https://help.tradeify.co/en/articles/12268167-essential-trading-rules-overview", accessed: "2026-10-23" }
  - { field: max_position, url: "https://tradeify.co/", accessed: "2026-10-23" }
  - { field: payout, url: "https://help.tradeify.co/en/articles/11083796-growth-funded-account-payout-policy", accessed: "2026-10-23" }
  - { field: platforms, url: "https://help.tradeify.co/en/articles/10468221-supported-platforms", accessed: "2026-10-23" }
  - { field: trading_day_reset, url: "https://help.tradeify.co/en/articles/10468225-what-is-a-trading-day", accessed: "2026-10-23" }
  - { field: trading_day_reset, url: "https://help.tradeify.co/en/articles/10468321-rules-daily-loss-limit", accessed: "2026-10-23" }
  - { field: copy_trading_allowed, url: "https://help.tradeify.co/en/articles/10495868-rules-hedging-correlated-products", accessed: "2026-10-23" }
  - { field: copy_trading_allowed, url: "https://help.tradeify.co/en/articles/13196343-tradovate-order-entry-copy-trading", accessed: "2026-10-23" }
  - { field: copy_trading_allowed, url: "https://help.tradeify.co/en/articles/13825156-wealthcharts-order-entry-trade-copier", accessed: "2026-10-23" }
  - { field: copy_trading_allowed, url: "https://tradeify.co/funded-trader-agreement", accessed: "2026-10-23" }
  - { field: automated_trading_allowed, url: "https://help.tradeify.co/en/articles/10468318-guidelines-for-traders", accessed: "2026-10-23" }
  - { field: automated_trading_allowed, url: "https://tradeify.co/funded-trader-agreement", accessed: "2026-10-23" }
  - { field: vps_allowed, url: "https://help.tradeify.co/en/articles/12268494-common-faqs", accessed: "2026-10-23" }
  - { field: vps_allowed, url: "https://tradeify.co/terms-of-use", accessed: "2026-10-23" }
```

#### Select — evaluation — 25K
```yaml
firm: tradeify
plan: "Select Evaluation"
phase: evaluation
size_label: "25K"
size_usd: "25000.00"
instruments: [MNQ, NQ]
platforms: [Tradovate, NinjaTrader (via Tradovate), TradingView (via Tradovate), Rithmic, Tradesea (Rithmic), Quantower (Rithmic), Sierra Chart (Rithmic), R|Trader (Rithmic), WealthCharts]
drawdown: { amount_usd: "1000.00", type: trailing_eod, stops_trailing_at: "Never locks on evaluation accounts" }
daily_loss_limit: { amount_usd: null, kind: none }   # 'Daily Loss Limit: None'
profit_target_usd: "1500.00"
consistency_rule: { text: "No single trading day can represent more than 40% of your total profit (evaluation phase only)", threshold_pct: "40" }
max_position: { minis: null, micros: null, scaling: "Evaluation: 1/10 (Select Evaluation article, homepage) vs 2/20 (payout policy article) - conflict" }
min_trading_days: 3   # 'minimum of 3 trading days because of its consistency rule' (2 with the 50% add-on)
trading_day_reset: { time: "18:00", tz_as_published: "ET" }   # trading day "6:00 PM ET to 5:00 PM ET"; DLL "resets at the start of each trading session (6:00 PM ET)"; flat by 4:45 PM ET
copy_trading_allowed: conditional   # only between accounts you personally own and manage, all in the same direction; copying to/from other people prohibited; copying to/from other firms not stated (bots: not across multiple firms)
automated_trading_allowed: conditional   # bots/algos allowed if solely owned, not shared with other traders/firms, not HFT, not used across multiple firms; ownership may be verified
automation_rule_text: |
  At Tradeify, we allow the use of bots and algorithms under certain conditions:
  Ownership: You must be able to prove that you are the sole owner of the bot or strategy, and that no one else has access to or is using it. This ensures that the bot/algorithm is not being shared with other traders or firms.
  We scan to ensure there are no similar orders on other accounts. We will also require a live video of you enabling the code on your own PC.
  Exclusive Use: While you may use the bot on your personal accounts, using it across multiple firms is against Tradeify's policy. The bot should be solely for your own use within Tradeify.
  No High-Frequency Trading (HFT) Bots: Personal bots are allowed as long as they are not high-frequency trading (HFT) bots. Tradeify has specific risk measures in place to detect and monitor such activities.
  Compliance and Verification: Tradeify reserves the right to request information or documentation if our risk measures flag your account for potential violations of these guidelines.
vps_allowed: conditional   # FAQ: no VPN/VPS for login; VPS for trading 'at your own risk' after a normal login; Terms forbid VPS that obscures device/identity verification
personal_device_required: unknown   # not stated for manual orders; for bots the firm may 'require a live video of you enabling the code on your own PC'
notes: "CONFLICT max_position: Select Evaluation article and homepage card say '1 mini / 10 micro'; Select payout policy article says 'During evaluation, you have access to the full contract limits (2/20, 4/40, 8/80, or 12/120 depending on account size)'. Optional 50% Consistency Add-on at purchase (min. 2 days). Select 50K bought before the new dashboard launch keeps a $2,500 profit target. 'Tier 1 News Trading' allowed."
status: conflict
not_confirmed_fields: [max_position]
sources:
  - { field: drawdown, url: "https://help.tradeify.co/en/articles/12853921-select-evaluation-accounts", accessed: "2026-10-23" }
  - { field: daily_loss_limit, url: "https://help.tradeify.co/en/articles/12853921-select-evaluation-accounts", accessed: "2026-10-23" }
  - { field: profit_target_usd, url: "https://help.tradeify.co/en/articles/12853921-select-evaluation-accounts", accessed: "2026-10-23" }
  - { field: consistency_rule, url: "https://help.tradeify.co/en/articles/12853921-select-evaluation-accounts", accessed: "2026-10-23" }
  - { field: max_position, url: "https://help.tradeify.co/en/articles/12853921-select-evaluation-accounts", accessed: "2026-10-23" }
  - { field: max_position, url: "https://help.tradeify.co/en/articles/12853966-select-flex-and-select-daily-payout-policies", accessed: "2026-10-23" }
  - { field: max_position, url: "https://tradeify.co/", accessed: "2026-10-23" }
  - { field: min_trading_days, url: "https://help.tradeify.co/en/articles/12853921-select-evaluation-accounts", accessed: "2026-10-23" }
  - { field: platforms, url: "https://help.tradeify.co/en/articles/10468221-supported-platforms", accessed: "2026-10-23" }
  - { field: trading_day_reset, url: "https://help.tradeify.co/en/articles/10468225-what-is-a-trading-day", accessed: "2026-10-23" }
  - { field: trading_day_reset, url: "https://help.tradeify.co/en/articles/10468321-rules-daily-loss-limit", accessed: "2026-10-23" }
  - { field: copy_trading_allowed, url: "https://help.tradeify.co/en/articles/10495868-rules-hedging-correlated-products", accessed: "2026-10-23" }
  - { field: copy_trading_allowed, url: "https://help.tradeify.co/en/articles/13196343-tradovate-order-entry-copy-trading", accessed: "2026-10-23" }
  - { field: copy_trading_allowed, url: "https://help.tradeify.co/en/articles/13825156-wealthcharts-order-entry-trade-copier", accessed: "2026-10-23" }
  - { field: copy_trading_allowed, url: "https://tradeify.co/funded-trader-agreement", accessed: "2026-10-23" }
  - { field: automated_trading_allowed, url: "https://help.tradeify.co/en/articles/10468318-guidelines-for-traders", accessed: "2026-10-23" }
  - { field: automated_trading_allowed, url: "https://tradeify.co/funded-trader-agreement", accessed: "2026-10-23" }
  - { field: vps_allowed, url: "https://help.tradeify.co/en/articles/12268494-common-faqs", accessed: "2026-10-23" }
  - { field: vps_allowed, url: "https://tradeify.co/terms-of-use", accessed: "2026-10-23" }
```

#### Select — evaluation — 50K
```yaml
firm: tradeify
plan: "Select Evaluation"
phase: evaluation
size_label: "50K"
size_usd: "50000.00"
instruments: [MNQ, NQ]
platforms: [Tradovate, NinjaTrader (via Tradovate), TradingView (via Tradovate), Rithmic, Tradesea (Rithmic), Quantower (Rithmic), Sierra Chart (Rithmic), R|Trader (Rithmic), WealthCharts]
drawdown: { amount_usd: "2000.00", type: trailing_eod, stops_trailing_at: "Never locks on evaluation accounts" }
daily_loss_limit: { amount_usd: null, kind: none }   # 'Daily Loss Limit: None'
profit_target_usd: "3000.00"
consistency_rule: { text: "No single trading day can represent more than 40% of your total profit (evaluation phase only)", threshold_pct: "40" }
max_position: { minis: 4, micros: 40, scaling: "Full limit during evaluation; scaling applies only to funded" }
min_trading_days: 3   # 'minimum of 3 trading days because of its consistency rule' (2 with the 50% add-on)
trading_day_reset: { time: "18:00", tz_as_published: "ET" }   # trading day "6:00 PM ET to 5:00 PM ET"; DLL "resets at the start of each trading session (6:00 PM ET)"; flat by 4:45 PM ET
copy_trading_allowed: conditional   # only between accounts you personally own and manage, all in the same direction; copying to/from other people prohibited; copying to/from other firms not stated (bots: not across multiple firms)
automated_trading_allowed: conditional   # bots/algos allowed if solely owned, not shared with other traders/firms, not HFT, not used across multiple firms; ownership may be verified
automation_rule_text: |
  At Tradeify, we allow the use of bots and algorithms under certain conditions:
  Ownership: You must be able to prove that you are the sole owner of the bot or strategy, and that no one else has access to or is using it. This ensures that the bot/algorithm is not being shared with other traders or firms.
  We scan to ensure there are no similar orders on other accounts. We will also require a live video of you enabling the code on your own PC.
  Exclusive Use: While you may use the bot on your personal accounts, using it across multiple firms is against Tradeify's policy. The bot should be solely for your own use within Tradeify.
  No High-Frequency Trading (HFT) Bots: Personal bots are allowed as long as they are not high-frequency trading (HFT) bots. Tradeify has specific risk measures in place to detect and monitor such activities.
  Compliance and Verification: Tradeify reserves the right to request information or documentation if our risk measures flag your account for potential violations of these guidelines.
vps_allowed: conditional   # FAQ: no VPN/VPS for login; VPS for trading 'at your own risk' after a normal login; Terms forbid VPS that obscures device/identity verification
personal_device_required: unknown   # not stated for manual orders; for bots the firm may 'require a live video of you enabling the code on your own PC'
notes: "Optional 50% Consistency Add-on at purchase (min. 2 days). Select 50K bought before the new dashboard launch keeps a $2,500 profit target. 'Tier 1 News Trading' allowed."
status: verified
not_confirmed_fields: []
sources:
  - { field: drawdown, url: "https://help.tradeify.co/en/articles/12853921-select-evaluation-accounts", accessed: "2026-10-23" }
  - { field: daily_loss_limit, url: "https://help.tradeify.co/en/articles/12853921-select-evaluation-accounts", accessed: "2026-10-23" }
  - { field: profit_target_usd, url: "https://help.tradeify.co/en/articles/12853921-select-evaluation-accounts", accessed: "2026-10-23" }
  - { field: consistency_rule, url: "https://help.tradeify.co/en/articles/12853921-select-evaluation-accounts", accessed: "2026-10-23" }
  - { field: max_position, url: "https://help.tradeify.co/en/articles/12853921-select-evaluation-accounts", accessed: "2026-10-23" }
  - { field: max_position, url: "https://help.tradeify.co/en/articles/12853966-select-flex-and-select-daily-payout-policies", accessed: "2026-10-23" }
  - { field: max_position, url: "https://tradeify.co/", accessed: "2026-10-23" }
  - { field: min_trading_days, url: "https://help.tradeify.co/en/articles/12853921-select-evaluation-accounts", accessed: "2026-10-23" }
  - { field: platforms, url: "https://help.tradeify.co/en/articles/10468221-supported-platforms", accessed: "2026-10-23" }
  - { field: trading_day_reset, url: "https://help.tradeify.co/en/articles/10468225-what-is-a-trading-day", accessed: "2026-10-23" }
  - { field: trading_day_reset, url: "https://help.tradeify.co/en/articles/10468321-rules-daily-loss-limit", accessed: "2026-10-23" }
  - { field: copy_trading_allowed, url: "https://help.tradeify.co/en/articles/10495868-rules-hedging-correlated-products", accessed: "2026-10-23" }
  - { field: copy_trading_allowed, url: "https://help.tradeify.co/en/articles/13196343-tradovate-order-entry-copy-trading", accessed: "2026-10-23" }
  - { field: copy_trading_allowed, url: "https://help.tradeify.co/en/articles/13825156-wealthcharts-order-entry-trade-copier", accessed: "2026-10-23" }
  - { field: copy_trading_allowed, url: "https://tradeify.co/funded-trader-agreement", accessed: "2026-10-23" }
  - { field: automated_trading_allowed, url: "https://help.tradeify.co/en/articles/10468318-guidelines-for-traders", accessed: "2026-10-23" }
  - { field: automated_trading_allowed, url: "https://tradeify.co/funded-trader-agreement", accessed: "2026-10-23" }
  - { field: vps_allowed, url: "https://help.tradeify.co/en/articles/12268494-common-faqs", accessed: "2026-10-23" }
  - { field: vps_allowed, url: "https://tradeify.co/terms-of-use", accessed: "2026-10-23" }
```

#### Select — evaluation — 100K
```yaml
firm: tradeify
plan: "Select Evaluation"
phase: evaluation
size_label: "100K"
size_usd: "100000.00"
instruments: [MNQ, NQ]
platforms: [Tradovate, NinjaTrader (via Tradovate), TradingView (via Tradovate), Rithmic, Tradesea (Rithmic), Quantower (Rithmic), Sierra Chart (Rithmic), R|Trader (Rithmic), WealthCharts]
drawdown: { amount_usd: "3000.00", type: trailing_eod, stops_trailing_at: "Never locks on evaluation accounts" }
daily_loss_limit: { amount_usd: null, kind: none }   # 'Daily Loss Limit: None'
profit_target_usd: "6000.00"
consistency_rule: { text: "No single trading day can represent more than 40% of your total profit (evaluation phase only)", threshold_pct: "40" }
max_position: { minis: 8, micros: 80, scaling: "Full limit during evaluation; scaling applies only to funded" }
min_trading_days: 3   # 'minimum of 3 trading days because of its consistency rule' (2 with the 50% add-on)
trading_day_reset: { time: "18:00", tz_as_published: "ET" }   # trading day "6:00 PM ET to 5:00 PM ET"; DLL "resets at the start of each trading session (6:00 PM ET)"; flat by 4:45 PM ET
copy_trading_allowed: conditional   # only between accounts you personally own and manage, all in the same direction; copying to/from other people prohibited; copying to/from other firms not stated (bots: not across multiple firms)
automated_trading_allowed: conditional   # bots/algos allowed if solely owned, not shared with other traders/firms, not HFT, not used across multiple firms; ownership may be verified
automation_rule_text: |
  At Tradeify, we allow the use of bots and algorithms under certain conditions:
  Ownership: You must be able to prove that you are the sole owner of the bot or strategy, and that no one else has access to or is using it. This ensures that the bot/algorithm is not being shared with other traders or firms.
  We scan to ensure there are no similar orders on other accounts. We will also require a live video of you enabling the code on your own PC.
  Exclusive Use: While you may use the bot on your personal accounts, using it across multiple firms is against Tradeify's policy. The bot should be solely for your own use within Tradeify.
  No High-Frequency Trading (HFT) Bots: Personal bots are allowed as long as they are not high-frequency trading (HFT) bots. Tradeify has specific risk measures in place to detect and monitor such activities.
  Compliance and Verification: Tradeify reserves the right to request information or documentation if our risk measures flag your account for potential violations of these guidelines.
vps_allowed: conditional   # FAQ: no VPN/VPS for login; VPS for trading 'at your own risk' after a normal login; Terms forbid VPS that obscures device/identity verification
personal_device_required: unknown   # not stated for manual orders; for bots the firm may 'require a live video of you enabling the code on your own PC'
notes: "Optional 50% Consistency Add-on at purchase (min. 2 days). Select 50K bought before the new dashboard launch keeps a $2,500 profit target. 'Tier 1 News Trading' allowed."
status: verified
not_confirmed_fields: []
sources:
  - { field: drawdown, url: "https://help.tradeify.co/en/articles/12853921-select-evaluation-accounts", accessed: "2026-10-23" }
  - { field: daily_loss_limit, url: "https://help.tradeify.co/en/articles/12853921-select-evaluation-accounts", accessed: "2026-10-23" }
  - { field: profit_target_usd, url: "https://help.tradeify.co/en/articles/12853921-select-evaluation-accounts", accessed: "2026-10-23" }
  - { field: consistency_rule, url: "https://help.tradeify.co/en/articles/12853921-select-evaluation-accounts", accessed: "2026-10-23" }
  - { field: max_position, url: "https://help.tradeify.co/en/articles/12853921-select-evaluation-accounts", accessed: "2026-10-23" }
  - { field: max_position, url: "https://help.tradeify.co/en/articles/12853966-select-flex-and-select-daily-payout-policies", accessed: "2026-10-23" }
  - { field: max_position, url: "https://tradeify.co/", accessed: "2026-10-23" }
  - { field: min_trading_days, url: "https://help.tradeify.co/en/articles/12853921-select-evaluation-accounts", accessed: "2026-10-23" }
  - { field: platforms, url: "https://help.tradeify.co/en/articles/10468221-supported-platforms", accessed: "2026-10-23" }
  - { field: trading_day_reset, url: "https://help.tradeify.co/en/articles/10468225-what-is-a-trading-day", accessed: "2026-10-23" }
  - { field: trading_day_reset, url: "https://help.tradeify.co/en/articles/10468321-rules-daily-loss-limit", accessed: "2026-10-23" }
  - { field: copy_trading_allowed, url: "https://help.tradeify.co/en/articles/10495868-rules-hedging-correlated-products", accessed: "2026-10-23" }
  - { field: copy_trading_allowed, url: "https://help.tradeify.co/en/articles/13196343-tradovate-order-entry-copy-trading", accessed: "2026-10-23" }
  - { field: copy_trading_allowed, url: "https://help.tradeify.co/en/articles/13825156-wealthcharts-order-entry-trade-copier", accessed: "2026-10-23" }
  - { field: copy_trading_allowed, url: "https://tradeify.co/funded-trader-agreement", accessed: "2026-10-23" }
  - { field: automated_trading_allowed, url: "https://help.tradeify.co/en/articles/10468318-guidelines-for-traders", accessed: "2026-10-23" }
  - { field: automated_trading_allowed, url: "https://tradeify.co/funded-trader-agreement", accessed: "2026-10-23" }
  - { field: vps_allowed, url: "https://help.tradeify.co/en/articles/12268494-common-faqs", accessed: "2026-10-23" }
  - { field: vps_allowed, url: "https://tradeify.co/terms-of-use", accessed: "2026-10-23" }
```

#### Select — evaluation — 150K
```yaml
firm: tradeify
plan: "Select Evaluation"
phase: evaluation
size_label: "150K"
size_usd: "150000.00"
instruments: [MNQ, NQ]
platforms: [Tradovate, NinjaTrader (via Tradovate), TradingView (via Tradovate), Rithmic, Tradesea (Rithmic), Quantower (Rithmic), Sierra Chart (Rithmic), R|Trader (Rithmic), WealthCharts]
drawdown: { amount_usd: "4500.00", type: trailing_eod, stops_trailing_at: "Never locks on evaluation accounts" }
daily_loss_limit: { amount_usd: null, kind: none }   # 'Daily Loss Limit: None'
profit_target_usd: "9000.00"
consistency_rule: { text: "No single trading day can represent more than 40% of your total profit (evaluation phase only)", threshold_pct: "40" }
max_position: { minis: 12, micros: 120, scaling: "Full limit during evaluation; scaling applies only to funded" }
min_trading_days: 3   # 'minimum of 3 trading days because of its consistency rule' (2 with the 50% add-on)
trading_day_reset: { time: "18:00", tz_as_published: "ET" }   # trading day "6:00 PM ET to 5:00 PM ET"; DLL "resets at the start of each trading session (6:00 PM ET)"; flat by 4:45 PM ET
copy_trading_allowed: conditional   # only between accounts you personally own and manage, all in the same direction; copying to/from other people prohibited; copying to/from other firms not stated (bots: not across multiple firms)
automated_trading_allowed: conditional   # bots/algos allowed if solely owned, not shared with other traders/firms, not HFT, not used across multiple firms; ownership may be verified
automation_rule_text: |
  At Tradeify, we allow the use of bots and algorithms under certain conditions:
  Ownership: You must be able to prove that you are the sole owner of the bot or strategy, and that no one else has access to or is using it. This ensures that the bot/algorithm is not being shared with other traders or firms.
  We scan to ensure there are no similar orders on other accounts. We will also require a live video of you enabling the code on your own PC.
  Exclusive Use: While you may use the bot on your personal accounts, using it across multiple firms is against Tradeify's policy. The bot should be solely for your own use within Tradeify.
  No High-Frequency Trading (HFT) Bots: Personal bots are allowed as long as they are not high-frequency trading (HFT) bots. Tradeify has specific risk measures in place to detect and monitor such activities.
  Compliance and Verification: Tradeify reserves the right to request information or documentation if our risk measures flag your account for potential violations of these guidelines.
vps_allowed: conditional   # FAQ: no VPN/VPS for login; VPS for trading 'at your own risk' after a normal login; Terms forbid VPS that obscures device/identity verification
personal_device_required: unknown   # not stated for manual orders; for bots the firm may 'require a live video of you enabling the code on your own PC'
notes: "Optional 50% Consistency Add-on at purchase (min. 2 days). Select 50K bought before the new dashboard launch keeps a $2,500 profit target. 'Tier 1 News Trading' allowed."
status: verified
not_confirmed_fields: []
sources:
  - { field: drawdown, url: "https://help.tradeify.co/en/articles/12853921-select-evaluation-accounts", accessed: "2026-10-23" }
  - { field: daily_loss_limit, url: "https://help.tradeify.co/en/articles/12853921-select-evaluation-accounts", accessed: "2026-10-23" }
  - { field: profit_target_usd, url: "https://help.tradeify.co/en/articles/12853921-select-evaluation-accounts", accessed: "2026-10-23" }
  - { field: consistency_rule, url: "https://help.tradeify.co/en/articles/12853921-select-evaluation-accounts", accessed: "2026-10-23" }
  - { field: max_position, url: "https://help.tradeify.co/en/articles/12853921-select-evaluation-accounts", accessed: "2026-10-23" }
  - { field: max_position, url: "https://help.tradeify.co/en/articles/12853966-select-flex-and-select-daily-payout-policies", accessed: "2026-10-23" }
  - { field: max_position, url: "https://tradeify.co/", accessed: "2026-10-23" }
  - { field: min_trading_days, url: "https://help.tradeify.co/en/articles/12853921-select-evaluation-accounts", accessed: "2026-10-23" }
  - { field: platforms, url: "https://help.tradeify.co/en/articles/10468221-supported-platforms", accessed: "2026-10-23" }
  - { field: trading_day_reset, url: "https://help.tradeify.co/en/articles/10468225-what-is-a-trading-day", accessed: "2026-10-23" }
  - { field: trading_day_reset, url: "https://help.tradeify.co/en/articles/10468321-rules-daily-loss-limit", accessed: "2026-10-23" }
  - { field: copy_trading_allowed, url: "https://help.tradeify.co/en/articles/10495868-rules-hedging-correlated-products", accessed: "2026-10-23" }
  - { field: copy_trading_allowed, url: "https://help.tradeify.co/en/articles/13196343-tradovate-order-entry-copy-trading", accessed: "2026-10-23" }
  - { field: copy_trading_allowed, url: "https://help.tradeify.co/en/articles/13825156-wealthcharts-order-entry-trade-copier", accessed: "2026-10-23" }
  - { field: copy_trading_allowed, url: "https://tradeify.co/funded-trader-agreement", accessed: "2026-10-23" }
  - { field: automated_trading_allowed, url: "https://help.tradeify.co/en/articles/10468318-guidelines-for-traders", accessed: "2026-10-23" }
  - { field: automated_trading_allowed, url: "https://tradeify.co/funded-trader-agreement", accessed: "2026-10-23" }
  - { field: vps_allowed, url: "https://help.tradeify.co/en/articles/12268494-common-faqs", accessed: "2026-10-23" }
  - { field: vps_allowed, url: "https://tradeify.co/terms-of-use", accessed: "2026-10-23" }
```

#### Select — evaluation — 300K
```yaml
firm: tradeify
plan: "Select Evaluation (300K V2, limited release)"
phase: evaluation
size_label: "300K"
size_usd: "300000.00"
instruments: [MNQ, NQ]
platforms: [Tradovate, NinjaTrader (via Tradovate), TradingView (via Tradovate), Rithmic, Tradesea (Rithmic), Quantower (Rithmic), Sierra Chart (Rithmic), R|Trader (Rithmic), WealthCharts]
drawdown: { amount_usd: "8000.00", type: trailing_eod, stops_trailing_at: "No lock on evaluation" }
daily_loss_limit: { amount_usd: "4000.00", kind: null }   # kind not stated for the 300K; DLL article lists only Growth, Lightning, Select Daily
profit_target_usd: "14000.00"
consistency_rule: { text: "No single trading day can represent more than 40% of your total profit", threshold_pct: "40" }
max_position: { minis: 16, micros: 160, scaling: "Full 16 mini / 160 micro during evaluation" }
min_trading_days: 3   # 'a minimum of 3 trading days'
trading_day_reset: { time: "18:00", tz_as_published: "ET" }   # trading day "6:00 PM ET to 5:00 PM ET"; DLL "resets at the start of each trading session (6:00 PM ET)"; flat by 4:45 PM ET
copy_trading_allowed: conditional   # only between accounts you personally own and manage, all in the same direction; copying to/from other people prohibited; copying to/from other firms not stated (bots: not across multiple firms)
automated_trading_allowed: conditional   # bots/algos allowed if solely owned, not shared with other traders/firms, not HFT, not used across multiple firms; ownership may be verified
automation_rule_text: |
  At Tradeify, we allow the use of bots and algorithms under certain conditions:
  Ownership: You must be able to prove that you are the sole owner of the bot or strategy, and that no one else has access to or is using it. This ensures that the bot/algorithm is not being shared with other traders or firms.
  We scan to ensure there are no similar orders on other accounts. We will also require a live video of you enabling the code on your own PC.
  Exclusive Use: While you may use the bot on your personal accounts, using it across multiple firms is against Tradeify's policy. The bot should be solely for your own use within Tradeify.
  No High-Frequency Trading (HFT) Bots: Personal bots are allowed as long as they are not high-frequency trading (HFT) bots. Tradeify has specific risk measures in place to detect and monitor such activities.
  Compliance and Verification: Tradeify reserves the right to request information or documentation if our risk measures flag your account for potential violations of these guidelines.
vps_allowed: conditional   # FAQ: no VPN/VPS for login; VPS for trading 'at your own risk' after a normal login; Terms forbid VPS that obscures device/identity verification
personal_device_required: unknown   # not stated for manual orders; for bots the firm may 'require a live video of you enabling the code on your own PC'
notes: "V2 = purchases on or after Aug 27, 2026. V1 (legacy): $6,000 EOD max loss, no DLL. No resets, KYC before purchase, max 3 lifetime purchases, 50% add-on not available."
status: partial
not_confirmed_fields: [daily_loss_limit.kind]
sources:
  - { field: drawdown, url: "https://help.tradeify.co/en/articles/16497699-300k-select-account", accessed: "2026-10-23" }
  - { field: daily_loss_limit, url: "https://help.tradeify.co/en/articles/16497699-300k-select-account", accessed: "2026-10-23" }
  - { field: profit_target_usd, url: "https://help.tradeify.co/en/articles/16497699-300k-select-account", accessed: "2026-10-23" }
  - { field: consistency_rule, url: "https://help.tradeify.co/en/articles/16497699-300k-select-account", accessed: "2026-10-23" }
  - { field: max_position, url: "https://help.tradeify.co/en/articles/16497699-300k-select-account", accessed: "2026-10-23" }
  - { field: platforms, url: "https://help.tradeify.co/en/articles/10468221-supported-platforms", accessed: "2026-10-23" }
  - { field: trading_day_reset, url: "https://help.tradeify.co/en/articles/10468225-what-is-a-trading-day", accessed: "2026-10-23" }
  - { field: trading_day_reset, url: "https://help.tradeify.co/en/articles/10468321-rules-daily-loss-limit", accessed: "2026-10-23" }
  - { field: copy_trading_allowed, url: "https://help.tradeify.co/en/articles/10495868-rules-hedging-correlated-products", accessed: "2026-10-23" }
  - { field: copy_trading_allowed, url: "https://help.tradeify.co/en/articles/13196343-tradovate-order-entry-copy-trading", accessed: "2026-10-23" }
  - { field: copy_trading_allowed, url: "https://help.tradeify.co/en/articles/13825156-wealthcharts-order-entry-trade-copier", accessed: "2026-10-23" }
  - { field: copy_trading_allowed, url: "https://tradeify.co/funded-trader-agreement", accessed: "2026-10-23" }
  - { field: automated_trading_allowed, url: "https://help.tradeify.co/en/articles/10468318-guidelines-for-traders", accessed: "2026-10-23" }
  - { field: automated_trading_allowed, url: "https://tradeify.co/funded-trader-agreement", accessed: "2026-10-23" }
  - { field: vps_allowed, url: "https://help.tradeify.co/en/articles/12268494-common-faqs", accessed: "2026-10-23" }
  - { field: vps_allowed, url: "https://tradeify.co/terms-of-use", accessed: "2026-10-23" }
```

#### Select Flex — funded — 25K
```yaml
firm: tradeify
plan: "Select Flex (5-Day Payout Policy)"
phase: funded
size_label: "25K"
size_usd: "25000.00"
instruments: [MNQ, NQ]
platforms: [Tradovate, NinjaTrader (via Tradovate), TradingView (via Tradovate), Rithmic, Tradesea (Rithmic), Quantower (Rithmic), Sierra Chart (Rithmic), R|Trader (Rithmic), WealthCharts]
drawdown: { amount_usd: "1000.00", type: trailing_eod, stops_trailing_at: "Locks when EOD balance reaches $26,100 or on payout request (whichever first); floor fixed at starting balance + $100" }
daily_loss_limit: { amount_usd: null, kind: none }   # 'Select Flex accounts do not have a DLL'
profit_target_usd: null
consistency_rule: { text: null, threshold_pct: null }
max_position: { minis: 2, micros: 20, scaling: "Start 1 mini / 10 micros; $1,500 EOD equity -> 2/20 (max) (calibrated at EOD, effective next day, cumulative)" }
min_trading_days: null   # payout every 5 winning days (25K $100 / 50K $150 / 100K $200 / 150K $250 per day)
trading_day_reset: { time: "18:00", tz_as_published: "ET" }   # trading day "6:00 PM ET to 5:00 PM ET"; DLL "resets at the start of each trading session (6:00 PM ET)"; flat by 4:45 PM ET
copy_trading_allowed: conditional   # only between accounts you personally own and manage, all in the same direction; copying to/from other people prohibited; copying to/from other firms not stated (bots: not across multiple firms)
automated_trading_allowed: conditional   # bots/algos allowed if solely owned, not shared with other traders/firms, not HFT, not used across multiple firms; ownership may be verified
automation_rule_text: |
  At Tradeify, we allow the use of bots and algorithms under certain conditions:
  Ownership: You must be able to prove that you are the sole owner of the bot or strategy, and that no one else has access to or is using it. This ensures that the bot/algorithm is not being shared with other traders or firms.
  We scan to ensure there are no similar orders on other accounts. We will also require a live video of you enabling the code on your own PC.
  Exclusive Use: While you may use the bot on your personal accounts, using it across multiple firms is against Tradeify's policy. The bot should be solely for your own use within Tradeify.
  No High-Frequency Trading (HFT) Bots: Personal bots are allowed as long as they are not high-frequency trading (HFT) bots. Tradeify has specific risk measures in place to detect and monitor such activities.
  Compliance and Verification: Tradeify reserves the right to request information or documentation if our risk measures flag your account for potential violations of these guidelines.
vps_allowed: conditional   # FAQ: no VPN/VPS for login; VPS for trading 'at your own risk' after a normal login; Terms forbid VPS that obscures device/identity verification
personal_device_required: unknown   # not stated for manual orders; for bots the firm may 'require a live video of you enabling the code on your own PC'
notes: "max_position shows the scaled maximum; the starting limit is lower (see scaling). No consistency rule on funded. 10-second rule gates payouts."
status: verified
not_confirmed_fields: []
sources:
  - { field: drawdown, url: "https://help.tradeify.co/en/articles/12853966-select-flex-and-select-daily-payout-policies", accessed: "2026-10-23" }
  - { field: drawdown, url: "https://help.tradeify.co/en/articles/10495897-rules-trailing-max-drawdowns", accessed: "2026-10-23" }
  - { field: daily_loss_limit, url: "https://help.tradeify.co/en/articles/12853966-select-flex-and-select-daily-payout-policies", accessed: "2026-10-23" }
  - { field: daily_loss_limit, url: "https://help.tradeify.co/en/articles/10468321-rules-daily-loss-limit", accessed: "2026-10-23" }
  - { field: consistency_rule, url: "https://help.tradeify.co/en/articles/12853966-select-flex-and-select-daily-payout-policies", accessed: "2026-10-23" }
  - { field: max_position, url: "https://help.tradeify.co/en/articles/12853966-select-flex-and-select-daily-payout-policies", accessed: "2026-10-23" }
  - { field: drawdown, url: "https://tradeify.co/", accessed: "2026-10-23" }
  - { field: platforms, url: "https://help.tradeify.co/en/articles/10468221-supported-platforms", accessed: "2026-10-23" }
  - { field: trading_day_reset, url: "https://help.tradeify.co/en/articles/10468225-what-is-a-trading-day", accessed: "2026-10-23" }
  - { field: trading_day_reset, url: "https://help.tradeify.co/en/articles/10468321-rules-daily-loss-limit", accessed: "2026-10-23" }
  - { field: copy_trading_allowed, url: "https://help.tradeify.co/en/articles/10495868-rules-hedging-correlated-products", accessed: "2026-10-23" }
  - { field: copy_trading_allowed, url: "https://help.tradeify.co/en/articles/13196343-tradovate-order-entry-copy-trading", accessed: "2026-10-23" }
  - { field: copy_trading_allowed, url: "https://help.tradeify.co/en/articles/13825156-wealthcharts-order-entry-trade-copier", accessed: "2026-10-23" }
  - { field: copy_trading_allowed, url: "https://tradeify.co/funded-trader-agreement", accessed: "2026-10-23" }
  - { field: automated_trading_allowed, url: "https://help.tradeify.co/en/articles/10468318-guidelines-for-traders", accessed: "2026-10-23" }
  - { field: automated_trading_allowed, url: "https://tradeify.co/funded-trader-agreement", accessed: "2026-10-23" }
  - { field: vps_allowed, url: "https://help.tradeify.co/en/articles/12268494-common-faqs", accessed: "2026-10-23" }
  - { field: vps_allowed, url: "https://tradeify.co/terms-of-use", accessed: "2026-10-23" }
```

#### Select Flex — funded — 50K
```yaml
firm: tradeify
plan: "Select Flex (5-Day Payout Policy)"
phase: funded
size_label: "50K"
size_usd: "50000.00"
instruments: [MNQ, NQ]
platforms: [Tradovate, NinjaTrader (via Tradovate), TradingView (via Tradovate), Rithmic, Tradesea (Rithmic), Quantower (Rithmic), Sierra Chart (Rithmic), R|Trader (Rithmic), WealthCharts]
drawdown: { amount_usd: "2000.00", type: trailing_eod, stops_trailing_at: "Locks when EOD balance reaches $52,100 or on payout request (whichever first); floor fixed at starting balance + $100" }
daily_loss_limit: { amount_usd: null, kind: none }   # 'Select Flex accounts do not have a DLL'
profit_target_usd: null
consistency_rule: { text: null, threshold_pct: null }
max_position: { minis: 4, micros: 40, scaling: "Start 2/20; $1,500 -> 3/30; $2,000 -> 4/40 (max) (calibrated at EOD, effective next day, cumulative)" }
min_trading_days: null   # payout every 5 winning days (25K $100 / 50K $150 / 100K $200 / 150K $250 per day)
trading_day_reset: { time: "18:00", tz_as_published: "ET" }   # trading day "6:00 PM ET to 5:00 PM ET"; DLL "resets at the start of each trading session (6:00 PM ET)"; flat by 4:45 PM ET
copy_trading_allowed: conditional   # only between accounts you personally own and manage, all in the same direction; copying to/from other people prohibited; copying to/from other firms not stated (bots: not across multiple firms)
automated_trading_allowed: conditional   # bots/algos allowed if solely owned, not shared with other traders/firms, not HFT, not used across multiple firms; ownership may be verified
automation_rule_text: |
  At Tradeify, we allow the use of bots and algorithms under certain conditions:
  Ownership: You must be able to prove that you are the sole owner of the bot or strategy, and that no one else has access to or is using it. This ensures that the bot/algorithm is not being shared with other traders or firms.
  We scan to ensure there are no similar orders on other accounts. We will also require a live video of you enabling the code on your own PC.
  Exclusive Use: While you may use the bot on your personal accounts, using it across multiple firms is against Tradeify's policy. The bot should be solely for your own use within Tradeify.
  No High-Frequency Trading (HFT) Bots: Personal bots are allowed as long as they are not high-frequency trading (HFT) bots. Tradeify has specific risk measures in place to detect and monitor such activities.
  Compliance and Verification: Tradeify reserves the right to request information or documentation if our risk measures flag your account for potential violations of these guidelines.
vps_allowed: conditional   # FAQ: no VPN/VPS for login; VPS for trading 'at your own risk' after a normal login; Terms forbid VPS that obscures device/identity verification
personal_device_required: unknown   # not stated for manual orders; for bots the firm may 'require a live video of you enabling the code on your own PC'
notes: "max_position shows the scaled maximum; the starting limit is lower (see scaling). No consistency rule on funded. 10-second rule gates payouts."
status: verified
not_confirmed_fields: []
sources:
  - { field: drawdown, url: "https://help.tradeify.co/en/articles/12853966-select-flex-and-select-daily-payout-policies", accessed: "2026-10-23" }
  - { field: drawdown, url: "https://help.tradeify.co/en/articles/10495897-rules-trailing-max-drawdowns", accessed: "2026-10-23" }
  - { field: daily_loss_limit, url: "https://help.tradeify.co/en/articles/12853966-select-flex-and-select-daily-payout-policies", accessed: "2026-10-23" }
  - { field: daily_loss_limit, url: "https://help.tradeify.co/en/articles/10468321-rules-daily-loss-limit", accessed: "2026-10-23" }
  - { field: consistency_rule, url: "https://help.tradeify.co/en/articles/12853966-select-flex-and-select-daily-payout-policies", accessed: "2026-10-23" }
  - { field: max_position, url: "https://help.tradeify.co/en/articles/12853966-select-flex-and-select-daily-payout-policies", accessed: "2026-10-23" }
  - { field: drawdown, url: "https://tradeify.co/", accessed: "2026-10-23" }
  - { field: platforms, url: "https://help.tradeify.co/en/articles/10468221-supported-platforms", accessed: "2026-10-23" }
  - { field: trading_day_reset, url: "https://help.tradeify.co/en/articles/10468225-what-is-a-trading-day", accessed: "2026-10-23" }
  - { field: trading_day_reset, url: "https://help.tradeify.co/en/articles/10468321-rules-daily-loss-limit", accessed: "2026-10-23" }
  - { field: copy_trading_allowed, url: "https://help.tradeify.co/en/articles/10495868-rules-hedging-correlated-products", accessed: "2026-10-23" }
  - { field: copy_trading_allowed, url: "https://help.tradeify.co/en/articles/13196343-tradovate-order-entry-copy-trading", accessed: "2026-10-23" }
  - { field: copy_trading_allowed, url: "https://help.tradeify.co/en/articles/13825156-wealthcharts-order-entry-trade-copier", accessed: "2026-10-23" }
  - { field: copy_trading_allowed, url: "https://tradeify.co/funded-trader-agreement", accessed: "2026-10-23" }
  - { field: automated_trading_allowed, url: "https://help.tradeify.co/en/articles/10468318-guidelines-for-traders", accessed: "2026-10-23" }
  - { field: automated_trading_allowed, url: "https://tradeify.co/funded-trader-agreement", accessed: "2026-10-23" }
  - { field: vps_allowed, url: "https://help.tradeify.co/en/articles/12268494-common-faqs", accessed: "2026-10-23" }
  - { field: vps_allowed, url: "https://tradeify.co/terms-of-use", accessed: "2026-10-23" }
```

#### Select Flex — funded — 100K
```yaml
firm: tradeify
plan: "Select Flex (5-Day Payout Policy)"
phase: funded
size_label: "100K"
size_usd: "100000.00"
instruments: [MNQ, NQ]
platforms: [Tradovate, NinjaTrader (via Tradovate), TradingView (via Tradovate), Rithmic, Tradesea (Rithmic), Quantower (Rithmic), Sierra Chart (Rithmic), R|Trader (Rithmic), WealthCharts]
drawdown: { amount_usd: "3000.00", type: trailing_eod, stops_trailing_at: "Locks when EOD balance reaches $103,100 or on payout request (whichever first); floor fixed at starting balance + $100" }
daily_loss_limit: { amount_usd: null, kind: none }   # 'Select Flex accounts do not have a DLL'
profit_target_usd: null
consistency_rule: { text: null, threshold_pct: null }
max_position: { minis: 8, micros: 80, scaling: "Start 3/30; $1,500 -> 4/40; $2,000 -> 5/50; $3,000 -> 8/80 (max) (calibrated at EOD, effective next day, cumulative)" }
min_trading_days: null   # payout every 5 winning days (25K $100 / 50K $150 / 100K $200 / 150K $250 per day)
trading_day_reset: { time: "18:00", tz_as_published: "ET" }   # trading day "6:00 PM ET to 5:00 PM ET"; DLL "resets at the start of each trading session (6:00 PM ET)"; flat by 4:45 PM ET
copy_trading_allowed: conditional   # only between accounts you personally own and manage, all in the same direction; copying to/from other people prohibited; copying to/from other firms not stated (bots: not across multiple firms)
automated_trading_allowed: conditional   # bots/algos allowed if solely owned, not shared with other traders/firms, not HFT, not used across multiple firms; ownership may be verified
automation_rule_text: |
  At Tradeify, we allow the use of bots and algorithms under certain conditions:
  Ownership: You must be able to prove that you are the sole owner of the bot or strategy, and that no one else has access to or is using it. This ensures that the bot/algorithm is not being shared with other traders or firms.
  We scan to ensure there are no similar orders on other accounts. We will also require a live video of you enabling the code on your own PC.
  Exclusive Use: While you may use the bot on your personal accounts, using it across multiple firms is against Tradeify's policy. The bot should be solely for your own use within Tradeify.
  No High-Frequency Trading (HFT) Bots: Personal bots are allowed as long as they are not high-frequency trading (HFT) bots. Tradeify has specific risk measures in place to detect and monitor such activities.
  Compliance and Verification: Tradeify reserves the right to request information or documentation if our risk measures flag your account for potential violations of these guidelines.
vps_allowed: conditional   # FAQ: no VPN/VPS for login; VPS for trading 'at your own risk' after a normal login; Terms forbid VPS that obscures device/identity verification
personal_device_required: unknown   # not stated for manual orders; for bots the firm may 'require a live video of you enabling the code on your own PC'
notes: "max_position shows the scaled maximum; the starting limit is lower (see scaling). No consistency rule on funded. 10-second rule gates payouts."
status: verified
not_confirmed_fields: []
sources:
  - { field: drawdown, url: "https://help.tradeify.co/en/articles/12853966-select-flex-and-select-daily-payout-policies", accessed: "2026-10-23" }
  - { field: drawdown, url: "https://help.tradeify.co/en/articles/10495897-rules-trailing-max-drawdowns", accessed: "2026-10-23" }
  - { field: daily_loss_limit, url: "https://help.tradeify.co/en/articles/12853966-select-flex-and-select-daily-payout-policies", accessed: "2026-10-23" }
  - { field: daily_loss_limit, url: "https://help.tradeify.co/en/articles/10468321-rules-daily-loss-limit", accessed: "2026-10-23" }
  - { field: consistency_rule, url: "https://help.tradeify.co/en/articles/12853966-select-flex-and-select-daily-payout-policies", accessed: "2026-10-23" }
  - { field: max_position, url: "https://help.tradeify.co/en/articles/12853966-select-flex-and-select-daily-payout-policies", accessed: "2026-10-23" }
  - { field: drawdown, url: "https://tradeify.co/", accessed: "2026-10-23" }
  - { field: platforms, url: "https://help.tradeify.co/en/articles/10468221-supported-platforms", accessed: "2026-10-23" }
  - { field: trading_day_reset, url: "https://help.tradeify.co/en/articles/10468225-what-is-a-trading-day", accessed: "2026-10-23" }
  - { field: trading_day_reset, url: "https://help.tradeify.co/en/articles/10468321-rules-daily-loss-limit", accessed: "2026-10-23" }
  - { field: copy_trading_allowed, url: "https://help.tradeify.co/en/articles/10495868-rules-hedging-correlated-products", accessed: "2026-10-23" }
  - { field: copy_trading_allowed, url: "https://help.tradeify.co/en/articles/13196343-tradovate-order-entry-copy-trading", accessed: "2026-10-23" }
  - { field: copy_trading_allowed, url: "https://help.tradeify.co/en/articles/13825156-wealthcharts-order-entry-trade-copier", accessed: "2026-10-23" }
  - { field: copy_trading_allowed, url: "https://tradeify.co/funded-trader-agreement", accessed: "2026-10-23" }
  - { field: automated_trading_allowed, url: "https://help.tradeify.co/en/articles/10468318-guidelines-for-traders", accessed: "2026-10-23" }
  - { field: automated_trading_allowed, url: "https://tradeify.co/funded-trader-agreement", accessed: "2026-10-23" }
  - { field: vps_allowed, url: "https://help.tradeify.co/en/articles/12268494-common-faqs", accessed: "2026-10-23" }
  - { field: vps_allowed, url: "https://tradeify.co/terms-of-use", accessed: "2026-10-23" }
```

#### Select Flex — funded — 150K
```yaml
firm: tradeify
plan: "Select Flex (5-Day Payout Policy)"
phase: funded
size_label: "150K"
size_usd: "150000.00"
instruments: [MNQ, NQ]
platforms: [Tradovate, NinjaTrader (via Tradovate), TradingView (via Tradovate), Rithmic, Tradesea (Rithmic), Quantower (Rithmic), Sierra Chart (Rithmic), R|Trader (Rithmic), WealthCharts]
drawdown: { amount_usd: "4500.00", type: trailing_eod, stops_trailing_at: "Locks when EOD balance reaches $154,600 or on payout request (whichever first); floor fixed at starting balance + $100" }
daily_loss_limit: { amount_usd: null, kind: none }   # 'Select Flex accounts do not have a DLL'
profit_target_usd: null
consistency_rule: { text: null, threshold_pct: null }
max_position: { minis: 12, micros: 120, scaling: "Start 3/30; $1,500 -> 4/40; $2,000 -> 5/50; $3,000 -> 8/80; $4,500 -> 12/120 (max) (calibrated at EOD, effective next day, cumulative)" }
min_trading_days: null   # payout every 5 winning days (25K $100 / 50K $150 / 100K $200 / 150K $250 per day)
trading_day_reset: { time: "18:00", tz_as_published: "ET" }   # trading day "6:00 PM ET to 5:00 PM ET"; DLL "resets at the start of each trading session (6:00 PM ET)"; flat by 4:45 PM ET
copy_trading_allowed: conditional   # only between accounts you personally own and manage, all in the same direction; copying to/from other people prohibited; copying to/from other firms not stated (bots: not across multiple firms)
automated_trading_allowed: conditional   # bots/algos allowed if solely owned, not shared with other traders/firms, not HFT, not used across multiple firms; ownership may be verified
automation_rule_text: |
  At Tradeify, we allow the use of bots and algorithms under certain conditions:
  Ownership: You must be able to prove that you are the sole owner of the bot or strategy, and that no one else has access to or is using it. This ensures that the bot/algorithm is not being shared with other traders or firms.
  We scan to ensure there are no similar orders on other accounts. We will also require a live video of you enabling the code on your own PC.
  Exclusive Use: While you may use the bot on your personal accounts, using it across multiple firms is against Tradeify's policy. The bot should be solely for your own use within Tradeify.
  No High-Frequency Trading (HFT) Bots: Personal bots are allowed as long as they are not high-frequency trading (HFT) bots. Tradeify has specific risk measures in place to detect and monitor such activities.
  Compliance and Verification: Tradeify reserves the right to request information or documentation if our risk measures flag your account for potential violations of these guidelines.
vps_allowed: conditional   # FAQ: no VPN/VPS for login; VPS for trading 'at your own risk' after a normal login; Terms forbid VPS that obscures device/identity verification
personal_device_required: unknown   # not stated for manual orders; for bots the firm may 'require a live video of you enabling the code on your own PC'
notes: "max_position shows the scaled maximum; the starting limit is lower (see scaling). No consistency rule on funded. 10-second rule gates payouts."
status: verified
not_confirmed_fields: []
sources:
  - { field: drawdown, url: "https://help.tradeify.co/en/articles/12853966-select-flex-and-select-daily-payout-policies", accessed: "2026-10-23" }
  - { field: drawdown, url: "https://help.tradeify.co/en/articles/10495897-rules-trailing-max-drawdowns", accessed: "2026-10-23" }
  - { field: daily_loss_limit, url: "https://help.tradeify.co/en/articles/12853966-select-flex-and-select-daily-payout-policies", accessed: "2026-10-23" }
  - { field: daily_loss_limit, url: "https://help.tradeify.co/en/articles/10468321-rules-daily-loss-limit", accessed: "2026-10-23" }
  - { field: consistency_rule, url: "https://help.tradeify.co/en/articles/12853966-select-flex-and-select-daily-payout-policies", accessed: "2026-10-23" }
  - { field: max_position, url: "https://help.tradeify.co/en/articles/12853966-select-flex-and-select-daily-payout-policies", accessed: "2026-10-23" }
  - { field: drawdown, url: "https://tradeify.co/", accessed: "2026-10-23" }
  - { field: platforms, url: "https://help.tradeify.co/en/articles/10468221-supported-platforms", accessed: "2026-10-23" }
  - { field: trading_day_reset, url: "https://help.tradeify.co/en/articles/10468225-what-is-a-trading-day", accessed: "2026-10-23" }
  - { field: trading_day_reset, url: "https://help.tradeify.co/en/articles/10468321-rules-daily-loss-limit", accessed: "2026-10-23" }
  - { field: copy_trading_allowed, url: "https://help.tradeify.co/en/articles/10495868-rules-hedging-correlated-products", accessed: "2026-10-23" }
  - { field: copy_trading_allowed, url: "https://help.tradeify.co/en/articles/13196343-tradovate-order-entry-copy-trading", accessed: "2026-10-23" }
  - { field: copy_trading_allowed, url: "https://help.tradeify.co/en/articles/13825156-wealthcharts-order-entry-trade-copier", accessed: "2026-10-23" }
  - { field: copy_trading_allowed, url: "https://tradeify.co/funded-trader-agreement", accessed: "2026-10-23" }
  - { field: automated_trading_allowed, url: "https://help.tradeify.co/en/articles/10468318-guidelines-for-traders", accessed: "2026-10-23" }
  - { field: automated_trading_allowed, url: "https://tradeify.co/funded-trader-agreement", accessed: "2026-10-23" }
  - { field: vps_allowed, url: "https://help.tradeify.co/en/articles/12268494-common-faqs", accessed: "2026-10-23" }
  - { field: vps_allowed, url: "https://tradeify.co/terms-of-use", accessed: "2026-10-23" }
```

#### Select Flex — funded — 300K
```yaml
firm: tradeify
plan: "Select Flex (5-Day Payout Policy) — 300K V2"
phase: funded
size_label: "300K"
size_usd: "300000.00"
instruments: [MNQ, NQ]
platforms: [Tradovate, NinjaTrader (via Tradovate), TradingView (via Tradovate), Rithmic, Tradesea (Rithmic), Quantower (Rithmic), Sierra Chart (Rithmic), R|Trader (Rithmic), WealthCharts]
drawdown: { amount_usd: "8000.00", type: trailing_eod, stops_trailing_at: "Locks once EOD balance reaches $308,100 (V2 Flex) or on payout approval; fixed floor $300,100" }
daily_loss_limit: { amount_usd: "4000.00", kind: null }   # kind not stated for the 300K
profit_target_usd: null
consistency_rule: { text: null, threshold_pct: null }
max_position: { minis: 16, micros: 160, scaling: "EOD profit $0-1,500: 3/30; $1,500-3,000: 4/40; $3,000-5,000: 6/60; $5,000-7,000: 10/100; $7,000+: 16/160 (next day, cumulative)" }
min_trading_days: null   # payout every 5 winning days ($750 minimum per day)
trading_day_reset: { time: "18:00", tz_as_published: "ET" }   # trading day "6:00 PM ET to 5:00 PM ET"; DLL "resets at the start of each trading session (6:00 PM ET)"; flat by 4:45 PM ET
copy_trading_allowed: conditional   # only between accounts you personally own and manage, all in the same direction; copying to/from other people prohibited; copying to/from other firms not stated (bots: not across multiple firms)
automated_trading_allowed: conditional   # bots/algos allowed if solely owned, not shared with other traders/firms, not HFT, not used across multiple firms; ownership may be verified
automation_rule_text: |
  At Tradeify, we allow the use of bots and algorithms under certain conditions:
  Ownership: You must be able to prove that you are the sole owner of the bot or strategy, and that no one else has access to or is using it. This ensures that the bot/algorithm is not being shared with other traders or firms.
  We scan to ensure there are no similar orders on other accounts. We will also require a live video of you enabling the code on your own PC.
  Exclusive Use: While you may use the bot on your personal accounts, using it across multiple firms is against Tradeify's policy. The bot should be solely for your own use within Tradeify.
  No High-Frequency Trading (HFT) Bots: Personal bots are allowed as long as they are not high-frequency trading (HFT) bots. Tradeify has specific risk measures in place to detect and monitor such activities.
  Compliance and Verification: Tradeify reserves the right to request information or documentation if our risk measures flag your account for potential violations of these guidelines.
vps_allowed: conditional   # FAQ: no VPN/VPS for login; VPS for trading 'at your own risk' after a normal login; Terms forbid VPS that obscures device/identity verification
personal_device_required: unknown   # not stated for manual orders; for bots the firm may 'require a live video of you enabling the code on your own PC'
notes: "V1 legacy Flex: $6,000 drawdown, no DLL, lock at $306,100. V2 300K funded accounts sit on top of the 5-account limit (up to 8)."
status: partial
not_confirmed_fields: [daily_loss_limit.kind]
sources:
  - { field: drawdown, url: "https://help.tradeify.co/en/articles/16497699-300k-select-account", accessed: "2026-10-23" }
  - { field: daily_loss_limit, url: "https://help.tradeify.co/en/articles/16497699-300k-select-account", accessed: "2026-10-23" }
  - { field: max_position, url: "https://help.tradeify.co/en/articles/16497699-300k-select-account", accessed: "2026-10-23" }
  - { field: platforms, url: "https://help.tradeify.co/en/articles/10468221-supported-platforms", accessed: "2026-10-23" }
  - { field: trading_day_reset, url: "https://help.tradeify.co/en/articles/10468225-what-is-a-trading-day", accessed: "2026-10-23" }
  - { field: trading_day_reset, url: "https://help.tradeify.co/en/articles/10468321-rules-daily-loss-limit", accessed: "2026-10-23" }
  - { field: copy_trading_allowed, url: "https://help.tradeify.co/en/articles/10495868-rules-hedging-correlated-products", accessed: "2026-10-23" }
  - { field: copy_trading_allowed, url: "https://help.tradeify.co/en/articles/13196343-tradovate-order-entry-copy-trading", accessed: "2026-10-23" }
  - { field: copy_trading_allowed, url: "https://help.tradeify.co/en/articles/13825156-wealthcharts-order-entry-trade-copier", accessed: "2026-10-23" }
  - { field: copy_trading_allowed, url: "https://tradeify.co/funded-trader-agreement", accessed: "2026-10-23" }
  - { field: automated_trading_allowed, url: "https://help.tradeify.co/en/articles/10468318-guidelines-for-traders", accessed: "2026-10-23" }
  - { field: automated_trading_allowed, url: "https://tradeify.co/funded-trader-agreement", accessed: "2026-10-23" }
  - { field: vps_allowed, url: "https://help.tradeify.co/en/articles/12268494-common-faqs", accessed: "2026-10-23" }
  - { field: vps_allowed, url: "https://tradeify.co/terms-of-use", accessed: "2026-10-23" }
```

#### Select Daily — funded — 25K
```yaml
firm: tradeify
plan: "Select Daily (Daily Payout Policy)"
phase: funded
size_label: "25K"
size_usd: "25000.00"
instruments: [MNQ, NQ]
platforms: [Tradovate, NinjaTrader (via Tradovate), TradingView (via Tradovate), Rithmic, Tradesea (Rithmic), Quantower (Rithmic), Sierra Chart (Rithmic), R|Trader (Rithmic), WealthCharts]
drawdown: { amount_usd: "1000.00", type: trailing_eod, stops_trailing_at: "Locks when EOD balance reaches $26,100 or on payout request (whichever first); floor fixed at starting balance + $100" }
daily_loss_limit: { amount_usd: "500.00", kind: soft_pause }   # soft breach, resume next session (6:00 PM ET)
profit_target_usd: null
consistency_rule: { text: null, threshold_pct: null }
max_position: { minis: 2, micros: 20, scaling: "Start 1 mini / 10 micros; $1,500 EOD equity -> 2/20 (max) (calibrated at EOD, effective next day, cumulative)" }
min_trading_days: null   # daily payout eligibility once above the buffer
trading_day_reset: { time: "18:00", tz_as_published: "ET" }   # trading day "6:00 PM ET to 5:00 PM ET"; DLL "resets at the start of each trading session (6:00 PM ET)"; flat by 4:45 PM ET
copy_trading_allowed: conditional   # only between accounts you personally own and manage, all in the same direction; copying to/from other people prohibited; copying to/from other firms not stated (bots: not across multiple firms)
automated_trading_allowed: conditional   # bots/algos allowed if solely owned, not shared with other traders/firms, not HFT, not used across multiple firms; ownership may be verified
automation_rule_text: |
  At Tradeify, we allow the use of bots and algorithms under certain conditions:
  Ownership: You must be able to prove that you are the sole owner of the bot or strategy, and that no one else has access to or is using it. This ensures that the bot/algorithm is not being shared with other traders or firms.
  We scan to ensure there are no similar orders on other accounts. We will also require a live video of you enabling the code on your own PC.
  Exclusive Use: While you may use the bot on your personal accounts, using it across multiple firms is against Tradeify's policy. The bot should be solely for your own use within Tradeify.
  No High-Frequency Trading (HFT) Bots: Personal bots are allowed as long as they are not high-frequency trading (HFT) bots. Tradeify has specific risk measures in place to detect and monitor such activities.
  Compliance and Verification: Tradeify reserves the right to request information or documentation if our risk measures flag your account for potential violations of these guidelines.
vps_allowed: conditional   # FAQ: no VPN/VPS for login; VPS for trading 'at your own risk' after a normal login; Terms forbid VPS that obscures device/identity verification
personal_device_required: unknown   # not stated for manual orders; for bots the firm may 'require a live video of you enabling the code on your own PC'
notes: "Payout buffer $1,100 (cannot withdraw below it). max_position shows the scaled maximum. Essential Rules overview lists Select Daily DLL only for 50K/100K/150K; the 25K $500 is from the DLL and payout-policy articles and the homepage."
status: verified
not_confirmed_fields: []
sources:
  - { field: drawdown, url: "https://help.tradeify.co/en/articles/12853966-select-flex-and-select-daily-payout-policies", accessed: "2026-10-23" }
  - { field: drawdown, url: "https://help.tradeify.co/en/articles/10495897-rules-trailing-max-drawdowns", accessed: "2026-10-23" }
  - { field: daily_loss_limit, url: "https://help.tradeify.co/en/articles/12853966-select-flex-and-select-daily-payout-policies", accessed: "2026-10-23" }
  - { field: daily_loss_limit, url: "https://help.tradeify.co/en/articles/10468321-rules-daily-loss-limit", accessed: "2026-10-23" }
  - { field: max_position, url: "https://help.tradeify.co/en/articles/12853966-select-flex-and-select-daily-payout-policies", accessed: "2026-10-23" }
  - { field: drawdown, url: "https://tradeify.co/", accessed: "2026-10-23" }
  - { field: platforms, url: "https://help.tradeify.co/en/articles/10468221-supported-platforms", accessed: "2026-10-23" }
  - { field: trading_day_reset, url: "https://help.tradeify.co/en/articles/10468225-what-is-a-trading-day", accessed: "2026-10-23" }
  - { field: trading_day_reset, url: "https://help.tradeify.co/en/articles/10468321-rules-daily-loss-limit", accessed: "2026-10-23" }
  - { field: copy_trading_allowed, url: "https://help.tradeify.co/en/articles/10495868-rules-hedging-correlated-products", accessed: "2026-10-23" }
  - { field: copy_trading_allowed, url: "https://help.tradeify.co/en/articles/13196343-tradovate-order-entry-copy-trading", accessed: "2026-10-23" }
  - { field: copy_trading_allowed, url: "https://help.tradeify.co/en/articles/13825156-wealthcharts-order-entry-trade-copier", accessed: "2026-10-23" }
  - { field: copy_trading_allowed, url: "https://tradeify.co/funded-trader-agreement", accessed: "2026-10-23" }
  - { field: automated_trading_allowed, url: "https://help.tradeify.co/en/articles/10468318-guidelines-for-traders", accessed: "2026-10-23" }
  - { field: automated_trading_allowed, url: "https://tradeify.co/funded-trader-agreement", accessed: "2026-10-23" }
  - { field: vps_allowed, url: "https://help.tradeify.co/en/articles/12268494-common-faqs", accessed: "2026-10-23" }
  - { field: vps_allowed, url: "https://tradeify.co/terms-of-use", accessed: "2026-10-23" }
```

#### Select Daily — funded — 50K
```yaml
firm: tradeify
plan: "Select Daily (Daily Payout Policy)"
phase: funded
size_label: "50K"
size_usd: "50000.00"
instruments: [MNQ, NQ]
platforms: [Tradovate, NinjaTrader (via Tradovate), TradingView (via Tradovate), Rithmic, Tradesea (Rithmic), Quantower (Rithmic), Sierra Chart (Rithmic), R|Trader (Rithmic), WealthCharts]
drawdown: { amount_usd: "2000.00", type: trailing_eod, stops_trailing_at: "Locks when EOD balance reaches $52,100 or on payout request (whichever first); floor fixed at starting balance + $100" }
daily_loss_limit: { amount_usd: "1000.00", kind: soft_pause }   # soft breach, resume next session (6:00 PM ET)
profit_target_usd: null
consistency_rule: { text: null, threshold_pct: null }
max_position: { minis: 4, micros: 40, scaling: "Start 2/20; $1,500 -> 3/30; $2,000 -> 4/40 (max) (calibrated at EOD, effective next day, cumulative)" }
min_trading_days: null   # daily payout eligibility once above the buffer
trading_day_reset: { time: "18:00", tz_as_published: "ET" }   # trading day "6:00 PM ET to 5:00 PM ET"; DLL "resets at the start of each trading session (6:00 PM ET)"; flat by 4:45 PM ET
copy_trading_allowed: conditional   # only between accounts you personally own and manage, all in the same direction; copying to/from other people prohibited; copying to/from other firms not stated (bots: not across multiple firms)
automated_trading_allowed: conditional   # bots/algos allowed if solely owned, not shared with other traders/firms, not HFT, not used across multiple firms; ownership may be verified
automation_rule_text: |
  At Tradeify, we allow the use of bots and algorithms under certain conditions:
  Ownership: You must be able to prove that you are the sole owner of the bot or strategy, and that no one else has access to or is using it. This ensures that the bot/algorithm is not being shared with other traders or firms.
  We scan to ensure there are no similar orders on other accounts. We will also require a live video of you enabling the code on your own PC.
  Exclusive Use: While you may use the bot on your personal accounts, using it across multiple firms is against Tradeify's policy. The bot should be solely for your own use within Tradeify.
  No High-Frequency Trading (HFT) Bots: Personal bots are allowed as long as they are not high-frequency trading (HFT) bots. Tradeify has specific risk measures in place to detect and monitor such activities.
  Compliance and Verification: Tradeify reserves the right to request information or documentation if our risk measures flag your account for potential violations of these guidelines.
vps_allowed: conditional   # FAQ: no VPN/VPS for login; VPS for trading 'at your own risk' after a normal login; Terms forbid VPS that obscures device/identity verification
personal_device_required: unknown   # not stated for manual orders; for bots the firm may 'require a live video of you enabling the code on your own PC'
notes: "Payout buffer $2,100 (cannot withdraw below it). max_position shows the scaled maximum. Essential Rules overview lists Select Daily DLL only for 50K/100K/150K; the 25K $500 is from the DLL and payout-policy articles and the homepage."
status: verified
not_confirmed_fields: []
sources:
  - { field: drawdown, url: "https://help.tradeify.co/en/articles/12853966-select-flex-and-select-daily-payout-policies", accessed: "2026-10-23" }
  - { field: drawdown, url: "https://help.tradeify.co/en/articles/10495897-rules-trailing-max-drawdowns", accessed: "2026-10-23" }
  - { field: daily_loss_limit, url: "https://help.tradeify.co/en/articles/12853966-select-flex-and-select-daily-payout-policies", accessed: "2026-10-23" }
  - { field: daily_loss_limit, url: "https://help.tradeify.co/en/articles/10468321-rules-daily-loss-limit", accessed: "2026-10-23" }
  - { field: max_position, url: "https://help.tradeify.co/en/articles/12853966-select-flex-and-select-daily-payout-policies", accessed: "2026-10-23" }
  - { field: drawdown, url: "https://tradeify.co/", accessed: "2026-10-23" }
  - { field: platforms, url: "https://help.tradeify.co/en/articles/10468221-supported-platforms", accessed: "2026-10-23" }
  - { field: trading_day_reset, url: "https://help.tradeify.co/en/articles/10468225-what-is-a-trading-day", accessed: "2026-10-23" }
  - { field: trading_day_reset, url: "https://help.tradeify.co/en/articles/10468321-rules-daily-loss-limit", accessed: "2026-10-23" }
  - { field: copy_trading_allowed, url: "https://help.tradeify.co/en/articles/10495868-rules-hedging-correlated-products", accessed: "2026-10-23" }
  - { field: copy_trading_allowed, url: "https://help.tradeify.co/en/articles/13196343-tradovate-order-entry-copy-trading", accessed: "2026-10-23" }
  - { field: copy_trading_allowed, url: "https://help.tradeify.co/en/articles/13825156-wealthcharts-order-entry-trade-copier", accessed: "2026-10-23" }
  - { field: copy_trading_allowed, url: "https://tradeify.co/funded-trader-agreement", accessed: "2026-10-23" }
  - { field: automated_trading_allowed, url: "https://help.tradeify.co/en/articles/10468318-guidelines-for-traders", accessed: "2026-10-23" }
  - { field: automated_trading_allowed, url: "https://tradeify.co/funded-trader-agreement", accessed: "2026-10-23" }
  - { field: vps_allowed, url: "https://help.tradeify.co/en/articles/12268494-common-faqs", accessed: "2026-10-23" }
  - { field: vps_allowed, url: "https://tradeify.co/terms-of-use", accessed: "2026-10-23" }
```

#### Select Daily — funded — 100K
```yaml
firm: tradeify
plan: "Select Daily (Daily Payout Policy)"
phase: funded
size_label: "100K"
size_usd: "100000.00"
instruments: [MNQ, NQ]
platforms: [Tradovate, NinjaTrader (via Tradovate), TradingView (via Tradovate), Rithmic, Tradesea (Rithmic), Quantower (Rithmic), Sierra Chart (Rithmic), R|Trader (Rithmic), WealthCharts]
drawdown: { amount_usd: "2500.00", type: trailing_eod, stops_trailing_at: "Locks when EOD balance reaches $102,600 or on payout request (whichever first); floor fixed at starting balance + $100" }
daily_loss_limit: { amount_usd: "1250.00", kind: soft_pause }   # soft breach, resume next session (6:00 PM ET)
profit_target_usd: null
consistency_rule: { text: null, threshold_pct: null }
max_position: { minis: 8, micros: 80, scaling: "Start 3/30; $1,500 -> 4/40; $2,000 -> 5/50; $3,000 -> 8/80 (max) (calibrated at EOD, effective next day, cumulative)" }
min_trading_days: null   # daily payout eligibility once above the buffer
trading_day_reset: { time: "18:00", tz_as_published: "ET" }   # trading day "6:00 PM ET to 5:00 PM ET"; DLL "resets at the start of each trading session (6:00 PM ET)"; flat by 4:45 PM ET
copy_trading_allowed: conditional   # only between accounts you personally own and manage, all in the same direction; copying to/from other people prohibited; copying to/from other firms not stated (bots: not across multiple firms)
automated_trading_allowed: conditional   # bots/algos allowed if solely owned, not shared with other traders/firms, not HFT, not used across multiple firms; ownership may be verified
automation_rule_text: |
  At Tradeify, we allow the use of bots and algorithms under certain conditions:
  Ownership: You must be able to prove that you are the sole owner of the bot or strategy, and that no one else has access to or is using it. This ensures that the bot/algorithm is not being shared with other traders or firms.
  We scan to ensure there are no similar orders on other accounts. We will also require a live video of you enabling the code on your own PC.
  Exclusive Use: While you may use the bot on your personal accounts, using it across multiple firms is against Tradeify's policy. The bot should be solely for your own use within Tradeify.
  No High-Frequency Trading (HFT) Bots: Personal bots are allowed as long as they are not high-frequency trading (HFT) bots. Tradeify has specific risk measures in place to detect and monitor such activities.
  Compliance and Verification: Tradeify reserves the right to request information or documentation if our risk measures flag your account for potential violations of these guidelines.
vps_allowed: conditional   # FAQ: no VPN/VPS for login; VPS for trading 'at your own risk' after a normal login; Terms forbid VPS that obscures device/identity verification
personal_device_required: unknown   # not stated for manual orders; for bots the firm may 'require a live video of you enabling the code on your own PC'
notes: "Payout buffer $2,600 (cannot withdraw below it). max_position shows the scaled maximum. Essential Rules overview lists Select Daily DLL only for 50K/100K/150K; the 25K $500 is from the DLL and payout-policy articles and the homepage."
status: verified
not_confirmed_fields: []
sources:
  - { field: drawdown, url: "https://help.tradeify.co/en/articles/12853966-select-flex-and-select-daily-payout-policies", accessed: "2026-10-23" }
  - { field: drawdown, url: "https://help.tradeify.co/en/articles/10495897-rules-trailing-max-drawdowns", accessed: "2026-10-23" }
  - { field: daily_loss_limit, url: "https://help.tradeify.co/en/articles/12853966-select-flex-and-select-daily-payout-policies", accessed: "2026-10-23" }
  - { field: daily_loss_limit, url: "https://help.tradeify.co/en/articles/10468321-rules-daily-loss-limit", accessed: "2026-10-23" }
  - { field: max_position, url: "https://help.tradeify.co/en/articles/12853966-select-flex-and-select-daily-payout-policies", accessed: "2026-10-23" }
  - { field: drawdown, url: "https://tradeify.co/", accessed: "2026-10-23" }
  - { field: platforms, url: "https://help.tradeify.co/en/articles/10468221-supported-platforms", accessed: "2026-10-23" }
  - { field: trading_day_reset, url: "https://help.tradeify.co/en/articles/10468225-what-is-a-trading-day", accessed: "2026-10-23" }
  - { field: trading_day_reset, url: "https://help.tradeify.co/en/articles/10468321-rules-daily-loss-limit", accessed: "2026-10-23" }
  - { field: copy_trading_allowed, url: "https://help.tradeify.co/en/articles/10495868-rules-hedging-correlated-products", accessed: "2026-10-23" }
  - { field: copy_trading_allowed, url: "https://help.tradeify.co/en/articles/13196343-tradovate-order-entry-copy-trading", accessed: "2026-10-23" }
  - { field: copy_trading_allowed, url: "https://help.tradeify.co/en/articles/13825156-wealthcharts-order-entry-trade-copier", accessed: "2026-10-23" }
  - { field: copy_trading_allowed, url: "https://tradeify.co/funded-trader-agreement", accessed: "2026-10-23" }
  - { field: automated_trading_allowed, url: "https://help.tradeify.co/en/articles/10468318-guidelines-for-traders", accessed: "2026-10-23" }
  - { field: automated_trading_allowed, url: "https://tradeify.co/funded-trader-agreement", accessed: "2026-10-23" }
  - { field: vps_allowed, url: "https://help.tradeify.co/en/articles/12268494-common-faqs", accessed: "2026-10-23" }
  - { field: vps_allowed, url: "https://tradeify.co/terms-of-use", accessed: "2026-10-23" }
```

#### Select Daily — funded — 150K
```yaml
firm: tradeify
plan: "Select Daily (Daily Payout Policy)"
phase: funded
size_label: "150K"
size_usd: "150000.00"
instruments: [MNQ, NQ]
platforms: [Tradovate, NinjaTrader (via Tradovate), TradingView (via Tradovate), Rithmic, Tradesea (Rithmic), Quantower (Rithmic), Sierra Chart (Rithmic), R|Trader (Rithmic), WealthCharts]
drawdown: { amount_usd: "3500.00", type: trailing_eod, stops_trailing_at: "Locks when EOD balance reaches $153,600 or on payout request (whichever first); floor fixed at starting balance + $100" }
daily_loss_limit: { amount_usd: "1750.00", kind: soft_pause }   # soft breach, resume next session (6:00 PM ET)
profit_target_usd: null
consistency_rule: { text: null, threshold_pct: null }
max_position: { minis: 12, micros: 120, scaling: "Start 3/30; $1,500 -> 4/40; $2,000 -> 5/50; $3,000 -> 8/80; $4,500 -> 12/120 (max) (calibrated at EOD, effective next day, cumulative)" }
min_trading_days: null   # daily payout eligibility once above the buffer
trading_day_reset: { time: "18:00", tz_as_published: "ET" }   # trading day "6:00 PM ET to 5:00 PM ET"; DLL "resets at the start of each trading session (6:00 PM ET)"; flat by 4:45 PM ET
copy_trading_allowed: conditional   # only between accounts you personally own and manage, all in the same direction; copying to/from other people prohibited; copying to/from other firms not stated (bots: not across multiple firms)
automated_trading_allowed: conditional   # bots/algos allowed if solely owned, not shared with other traders/firms, not HFT, not used across multiple firms; ownership may be verified
automation_rule_text: |
  At Tradeify, we allow the use of bots and algorithms under certain conditions:
  Ownership: You must be able to prove that you are the sole owner of the bot or strategy, and that no one else has access to or is using it. This ensures that the bot/algorithm is not being shared with other traders or firms.
  We scan to ensure there are no similar orders on other accounts. We will also require a live video of you enabling the code on your own PC.
  Exclusive Use: While you may use the bot on your personal accounts, using it across multiple firms is against Tradeify's policy. The bot should be solely for your own use within Tradeify.
  No High-Frequency Trading (HFT) Bots: Personal bots are allowed as long as they are not high-frequency trading (HFT) bots. Tradeify has specific risk measures in place to detect and monitor such activities.
  Compliance and Verification: Tradeify reserves the right to request information or documentation if our risk measures flag your account for potential violations of these guidelines.
vps_allowed: conditional   # FAQ: no VPN/VPS for login; VPS for trading 'at your own risk' after a normal login; Terms forbid VPS that obscures device/identity verification
personal_device_required: unknown   # not stated for manual orders; for bots the firm may 'require a live video of you enabling the code on your own PC'
notes: "Payout buffer $3,600 (cannot withdraw below it). max_position shows the scaled maximum. Essential Rules overview lists Select Daily DLL only for 50K/100K/150K; the 25K $500 is from the DLL and payout-policy articles and the homepage."
status: verified
not_confirmed_fields: []
sources:
  - { field: drawdown, url: "https://help.tradeify.co/en/articles/12853966-select-flex-and-select-daily-payout-policies", accessed: "2026-10-23" }
  - { field: drawdown, url: "https://help.tradeify.co/en/articles/10495897-rules-trailing-max-drawdowns", accessed: "2026-10-23" }
  - { field: daily_loss_limit, url: "https://help.tradeify.co/en/articles/12853966-select-flex-and-select-daily-payout-policies", accessed: "2026-10-23" }
  - { field: daily_loss_limit, url: "https://help.tradeify.co/en/articles/10468321-rules-daily-loss-limit", accessed: "2026-10-23" }
  - { field: max_position, url: "https://help.tradeify.co/en/articles/12853966-select-flex-and-select-daily-payout-policies", accessed: "2026-10-23" }
  - { field: drawdown, url: "https://tradeify.co/", accessed: "2026-10-23" }
  - { field: platforms, url: "https://help.tradeify.co/en/articles/10468221-supported-platforms", accessed: "2026-10-23" }
  - { field: trading_day_reset, url: "https://help.tradeify.co/en/articles/10468225-what-is-a-trading-day", accessed: "2026-10-23" }
  - { field: trading_day_reset, url: "https://help.tradeify.co/en/articles/10468321-rules-daily-loss-limit", accessed: "2026-10-23" }
  - { field: copy_trading_allowed, url: "https://help.tradeify.co/en/articles/10495868-rules-hedging-correlated-products", accessed: "2026-10-23" }
  - { field: copy_trading_allowed, url: "https://help.tradeify.co/en/articles/13196343-tradovate-order-entry-copy-trading", accessed: "2026-10-23" }
  - { field: copy_trading_allowed, url: "https://help.tradeify.co/en/articles/13825156-wealthcharts-order-entry-trade-copier", accessed: "2026-10-23" }
  - { field: copy_trading_allowed, url: "https://tradeify.co/funded-trader-agreement", accessed: "2026-10-23" }
  - { field: automated_trading_allowed, url: "https://help.tradeify.co/en/articles/10468318-guidelines-for-traders", accessed: "2026-10-23" }
  - { field: automated_trading_allowed, url: "https://tradeify.co/funded-trader-agreement", accessed: "2026-10-23" }
  - { field: vps_allowed, url: "https://help.tradeify.co/en/articles/12268494-common-faqs", accessed: "2026-10-23" }
  - { field: vps_allowed, url: "https://tradeify.co/terms-of-use", accessed: "2026-10-23" }
```

#### Select Daily — funded — 300K
```yaml
firm: tradeify
plan: "Select Daily (Daily Payout Policy) — 300K V2"
phase: funded
size_label: "300K"
size_usd: "300000.00"
instruments: [MNQ, NQ]
platforms: [Tradovate, NinjaTrader (via Tradovate), TradingView (via Tradovate), Rithmic, Tradesea (Rithmic), Quantower (Rithmic), Sierra Chart (Rithmic), R|Trader (Rithmic), WealthCharts]
drawdown: { amount_usd: "6000.00", type: trailing_eod, stops_trailing_at: "Locks once EOD balance reaches $306,100 (V2 Daily); fixed floor $300,100" }
daily_loss_limit: { amount_usd: "3000.00", kind: soft_pause }   # Select Daily DLL is a soft breach per the DLL article (article tables cover 25K-150K)
profit_target_usd: null
consistency_rule: { text: null, threshold_pct: null }
max_position: { minis: 16, micros: 160, scaling: "EOD profit $0-1,500: 3/30; $1,500-3,000: 4/40; $3,000-5,000: 6/60; $5,000-7,000: 10/100; $7,000+: 16/160 (next day, cumulative)" }
min_trading_days: null   # daily eligibility above the buffer
trading_day_reset: { time: "18:00", tz_as_published: "ET" }   # trading day "6:00 PM ET to 5:00 PM ET"; DLL "resets at the start of each trading session (6:00 PM ET)"; flat by 4:45 PM ET
copy_trading_allowed: conditional   # only between accounts you personally own and manage, all in the same direction; copying to/from other people prohibited; copying to/from other firms not stated (bots: not across multiple firms)
automated_trading_allowed: conditional   # bots/algos allowed if solely owned, not shared with other traders/firms, not HFT, not used across multiple firms; ownership may be verified
automation_rule_text: |
  At Tradeify, we allow the use of bots and algorithms under certain conditions:
  Ownership: You must be able to prove that you are the sole owner of the bot or strategy, and that no one else has access to or is using it. This ensures that the bot/algorithm is not being shared with other traders or firms.
  We scan to ensure there are no similar orders on other accounts. We will also require a live video of you enabling the code on your own PC.
  Exclusive Use: While you may use the bot on your personal accounts, using it across multiple firms is against Tradeify's policy. The bot should be solely for your own use within Tradeify.
  No High-Frequency Trading (HFT) Bots: Personal bots are allowed as long as they are not high-frequency trading (HFT) bots. Tradeify has specific risk measures in place to detect and monitor such activities.
  Compliance and Verification: Tradeify reserves the right to request information or documentation if our risk measures flag your account for potential violations of these guidelines.
vps_allowed: conditional   # FAQ: no VPN/VPS for login; VPS for trading 'at your own risk' after a normal login; Terms forbid VPS that obscures device/identity verification
personal_device_required: unknown   # not stated for manual orders; for bots the firm may 'require a live video of you enabling the code on your own PC'
notes: "Buffer $6,100 (balance $306,100); max $4,000/day; 50% of each payout must be new profit. V1 legacy Daily: $5,000 drawdown, $2,500 DLL, buffer $5,100."
status: verified
not_confirmed_fields: []
sources:
  - { field: drawdown, url: "https://help.tradeify.co/en/articles/16497699-300k-select-account", accessed: "2026-10-23" }
  - { field: daily_loss_limit, url: "https://help.tradeify.co/en/articles/16497699-300k-select-account", accessed: "2026-10-23" }
  - { field: daily_loss_limit, url: "https://help.tradeify.co/en/articles/10468321-rules-daily-loss-limit", accessed: "2026-10-23" }
  - { field: max_position, url: "https://help.tradeify.co/en/articles/16497699-300k-select-account", accessed: "2026-10-23" }
  - { field: platforms, url: "https://help.tradeify.co/en/articles/10468221-supported-platforms", accessed: "2026-10-23" }
  - { field: trading_day_reset, url: "https://help.tradeify.co/en/articles/10468225-what-is-a-trading-day", accessed: "2026-10-23" }
  - { field: trading_day_reset, url: "https://help.tradeify.co/en/articles/10468321-rules-daily-loss-limit", accessed: "2026-10-23" }
  - { field: copy_trading_allowed, url: "https://help.tradeify.co/en/articles/10495868-rules-hedging-correlated-products", accessed: "2026-10-23" }
  - { field: copy_trading_allowed, url: "https://help.tradeify.co/en/articles/13196343-tradovate-order-entry-copy-trading", accessed: "2026-10-23" }
  - { field: copy_trading_allowed, url: "https://help.tradeify.co/en/articles/13825156-wealthcharts-order-entry-trade-copier", accessed: "2026-10-23" }
  - { field: copy_trading_allowed, url: "https://tradeify.co/funded-trader-agreement", accessed: "2026-10-23" }
  - { field: automated_trading_allowed, url: "https://help.tradeify.co/en/articles/10468318-guidelines-for-traders", accessed: "2026-10-23" }
  - { field: automated_trading_allowed, url: "https://tradeify.co/funded-trader-agreement", accessed: "2026-10-23" }
  - { field: vps_allowed, url: "https://help.tradeify.co/en/articles/12268494-common-faqs", accessed: "2026-10-23" }
  - { field: vps_allowed, url: "https://tradeify.co/terms-of-use", accessed: "2026-10-23" }
```

#### Lightning — funded — 25K
```yaml
firm: tradeify
plan: "Lightning Funded"
phase: funded
size_label: "25K"
size_usd: "25000.00"
instruments: [MNQ, NQ]
platforms: [Tradovate, NinjaTrader (via Tradovate), TradingView (via Tradovate), Rithmic, Tradesea (Rithmic), Quantower (Rithmic), Sierra Chart (Rithmic), R|Trader (Rithmic), WealthCharts]
drawdown: { amount_usd: "1000.00", type: trailing_eod, stops_trailing_at: "Locks when EOD balance reaches $26,100 or on payout request (whichever first); floor fixed at starting balance + $100" }
daily_loss_limit: { amount_usd: null, kind: none }   # '25k Lightning funded does not have Daily Loss Limit protection'
profit_target_usd: null
consistency_rule: { text: "Gradual Consistency Rule: First payout 20%, second payout 25%, third and subsequent payouts 30% (accounts bought after Sep 12, 2025 8:00 AM ET; older: 20% for all)", threshold_pct: "20/25/30" }
max_position: { minis: 1, micros: 10, scaling: "Maximum from day one; legacy (before Sep 12, 2025): 50K 5/50, 100K 10/100, 150K 15/150" }
min_trading_days: null   # 'No Minimum Trading Day Count'
trading_day_reset: { time: "18:00", tz_as_published: "ET" }   # trading day "6:00 PM ET to 5:00 PM ET"; DLL "resets at the start of each trading session (6:00 PM ET)"; flat by 4:45 PM ET
copy_trading_allowed: conditional   # only between accounts you personally own and manage, all in the same direction; copying to/from other people prohibited; copying to/from other firms not stated (bots: not across multiple firms)
automated_trading_allowed: conditional   # bots/algos allowed if solely owned, not shared with other traders/firms, not HFT, not used across multiple firms; ownership may be verified
automation_rule_text: |
  At Tradeify, we allow the use of bots and algorithms under certain conditions:
  Ownership: You must be able to prove that you are the sole owner of the bot or strategy, and that no one else has access to or is using it. This ensures that the bot/algorithm is not being shared with other traders or firms.
  We scan to ensure there are no similar orders on other accounts. We will also require a live video of you enabling the code on your own PC.
  Exclusive Use: While you may use the bot on your personal accounts, using it across multiple firms is against Tradeify's policy. The bot should be solely for your own use within Tradeify.
  No High-Frequency Trading (HFT) Bots: Personal bots are allowed as long as they are not high-frequency trading (HFT) bots. Tradeify has specific risk measures in place to detect and monitor such activities.
  Compliance and Verification: Tradeify reserves the right to request information or documentation if our risk measures flag your account for potential violations of these guidelines.
vps_allowed: conditional   # FAQ: no VPN/VPS for login; VPS for trading 'at your own risk' after a normal login; Terms forbid VPS that obscures device/identity verification
personal_device_required: unknown   # not stated for manual orders; for bots the firm may 'require a live video of you enabling the code on your own PC'
notes: "Straight to sim funded, no evaluation. Payout profit goal (Payout 1 / Payout 2+): $1,500 / $1,000; no minimum trading days. 10-second rule gates payouts."
status: verified
not_confirmed_fields: []
sources:
  - { field: drawdown, url: "https://help.tradeify.co/en/articles/10495938-lightning-funded-accounts", accessed: "2026-10-23" }
  - { field: drawdown, url: "https://help.tradeify.co/en/articles/10495897-rules-trailing-max-drawdowns", accessed: "2026-10-23" }
  - { field: daily_loss_limit, url: "https://help.tradeify.co/en/articles/10495938-lightning-funded-accounts", accessed: "2026-10-23" }
  - { field: daily_loss_limit, url: "https://help.tradeify.co/en/articles/10468321-rules-daily-loss-limit", accessed: "2026-10-23" }
  - { field: daily_loss_limit, url: "https://help.tradeify.co/en/articles/12268167-essential-trading-rules-overview", accessed: "2026-10-23" }
  - { field: consistency_rule, url: "https://help.tradeify.co/en/articles/10468320-rules-consistency-rule", accessed: "2026-10-23" }
  - { field: consistency_rule, url: "https://help.tradeify.co/en/articles/10495932-lightning-funded-account-payout-policy", accessed: "2026-10-23" }
  - { field: max_position, url: "https://help.tradeify.co/en/articles/10495938-lightning-funded-accounts", accessed: "2026-10-23" }
  - { field: payout, url: "https://help.tradeify.co/en/articles/10495932-lightning-funded-account-payout-policy", accessed: "2026-10-23" }
  - { field: drawdown, url: "https://tradeify.co/", accessed: "2026-10-23" }
  - { field: platforms, url: "https://help.tradeify.co/en/articles/10468221-supported-platforms", accessed: "2026-10-23" }
  - { field: trading_day_reset, url: "https://help.tradeify.co/en/articles/10468225-what-is-a-trading-day", accessed: "2026-10-23" }
  - { field: trading_day_reset, url: "https://help.tradeify.co/en/articles/10468321-rules-daily-loss-limit", accessed: "2026-10-23" }
  - { field: copy_trading_allowed, url: "https://help.tradeify.co/en/articles/10495868-rules-hedging-correlated-products", accessed: "2026-10-23" }
  - { field: copy_trading_allowed, url: "https://help.tradeify.co/en/articles/13196343-tradovate-order-entry-copy-trading", accessed: "2026-10-23" }
  - { field: copy_trading_allowed, url: "https://help.tradeify.co/en/articles/13825156-wealthcharts-order-entry-trade-copier", accessed: "2026-10-23" }
  - { field: copy_trading_allowed, url: "https://tradeify.co/funded-trader-agreement", accessed: "2026-10-23" }
  - { field: automated_trading_allowed, url: "https://help.tradeify.co/en/articles/10468318-guidelines-for-traders", accessed: "2026-10-23" }
  - { field: automated_trading_allowed, url: "https://tradeify.co/funded-trader-agreement", accessed: "2026-10-23" }
  - { field: vps_allowed, url: "https://help.tradeify.co/en/articles/12268494-common-faqs", accessed: "2026-10-23" }
  - { field: vps_allowed, url: "https://tradeify.co/terms-of-use", accessed: "2026-10-23" }
```

#### Lightning — funded — 50K
```yaml
firm: tradeify
plan: "Lightning Funded"
phase: funded
size_label: "50K"
size_usd: "50000.00"
instruments: [MNQ, NQ]
platforms: [Tradovate, NinjaTrader (via Tradovate), TradingView (via Tradovate), Rithmic, Tradesea (Rithmic), Quantower (Rithmic), Sierra Chart (Rithmic), R|Trader (Rithmic), WealthCharts]
drawdown: { amount_usd: "2000.00", type: trailing_eod, stops_trailing_at: "Locks when EOD balance reaches $52,100 or on payout request (whichever first); floor fixed at starting balance + $100" }
daily_loss_limit: { amount_usd: "1250.00", kind: soft_pause }   # soft breach; increases to $2,000 from the next session once balance reaches $53,000 (6% profit)
profit_target_usd: null
consistency_rule: { text: "Gradual Consistency Rule: First payout 20%, second payout 25%, third and subsequent payouts 30% (accounts bought after Sep 12, 2025 8:00 AM ET; older: 20% for all)", threshold_pct: "20/25/30" }
max_position: { minis: 4, micros: 40, scaling: "Maximum from day one; legacy (before Sep 12, 2025): 50K 5/50, 100K 10/100, 150K 15/150" }
min_trading_days: null   # 'No Minimum Trading Day Count'
trading_day_reset: { time: "18:00", tz_as_published: "ET" }   # trading day "6:00 PM ET to 5:00 PM ET"; DLL "resets at the start of each trading session (6:00 PM ET)"; flat by 4:45 PM ET
copy_trading_allowed: conditional   # only between accounts you personally own and manage, all in the same direction; copying to/from other people prohibited; copying to/from other firms not stated (bots: not across multiple firms)
automated_trading_allowed: conditional   # bots/algos allowed if solely owned, not shared with other traders/firms, not HFT, not used across multiple firms; ownership may be verified
automation_rule_text: |
  At Tradeify, we allow the use of bots and algorithms under certain conditions:
  Ownership: You must be able to prove that you are the sole owner of the bot or strategy, and that no one else has access to or is using it. This ensures that the bot/algorithm is not being shared with other traders or firms.
  We scan to ensure there are no similar orders on other accounts. We will also require a live video of you enabling the code on your own PC.
  Exclusive Use: While you may use the bot on your personal accounts, using it across multiple firms is against Tradeify's policy. The bot should be solely for your own use within Tradeify.
  No High-Frequency Trading (HFT) Bots: Personal bots are allowed as long as they are not high-frequency trading (HFT) bots. Tradeify has specific risk measures in place to detect and monitor such activities.
  Compliance and Verification: Tradeify reserves the right to request information or documentation if our risk measures flag your account for potential violations of these guidelines.
vps_allowed: conditional   # FAQ: no VPN/VPS for login; VPS for trading 'at your own risk' after a normal login; Terms forbid VPS that obscures device/identity verification
personal_device_required: unknown   # not stated for manual orders; for bots the firm may 'require a live video of you enabling the code on your own PC'
notes: "Straight to sim funded, no evaluation. Payout profit goal (Payout 1 / Payout 2+): $3,000 / $2,000; no minimum trading days. 10-second rule gates payouts."
status: verified
not_confirmed_fields: []
sources:
  - { field: drawdown, url: "https://help.tradeify.co/en/articles/10495938-lightning-funded-accounts", accessed: "2026-10-23" }
  - { field: drawdown, url: "https://help.tradeify.co/en/articles/10495897-rules-trailing-max-drawdowns", accessed: "2026-10-23" }
  - { field: daily_loss_limit, url: "https://help.tradeify.co/en/articles/10495938-lightning-funded-accounts", accessed: "2026-10-23" }
  - { field: daily_loss_limit, url: "https://help.tradeify.co/en/articles/10468321-rules-daily-loss-limit", accessed: "2026-10-23" }
  - { field: daily_loss_limit, url: "https://help.tradeify.co/en/articles/12268167-essential-trading-rules-overview", accessed: "2026-10-23" }
  - { field: consistency_rule, url: "https://help.tradeify.co/en/articles/10468320-rules-consistency-rule", accessed: "2026-10-23" }
  - { field: consistency_rule, url: "https://help.tradeify.co/en/articles/10495932-lightning-funded-account-payout-policy", accessed: "2026-10-23" }
  - { field: max_position, url: "https://help.tradeify.co/en/articles/10495938-lightning-funded-accounts", accessed: "2026-10-23" }
  - { field: payout, url: "https://help.tradeify.co/en/articles/10495932-lightning-funded-account-payout-policy", accessed: "2026-10-23" }
  - { field: drawdown, url: "https://tradeify.co/", accessed: "2026-10-23" }
  - { field: platforms, url: "https://help.tradeify.co/en/articles/10468221-supported-platforms", accessed: "2026-10-23" }
  - { field: trading_day_reset, url: "https://help.tradeify.co/en/articles/10468225-what-is-a-trading-day", accessed: "2026-10-23" }
  - { field: trading_day_reset, url: "https://help.tradeify.co/en/articles/10468321-rules-daily-loss-limit", accessed: "2026-10-23" }
  - { field: copy_trading_allowed, url: "https://help.tradeify.co/en/articles/10495868-rules-hedging-correlated-products", accessed: "2026-10-23" }
  - { field: copy_trading_allowed, url: "https://help.tradeify.co/en/articles/13196343-tradovate-order-entry-copy-trading", accessed: "2026-10-23" }
  - { field: copy_trading_allowed, url: "https://help.tradeify.co/en/articles/13825156-wealthcharts-order-entry-trade-copier", accessed: "2026-10-23" }
  - { field: copy_trading_allowed, url: "https://tradeify.co/funded-trader-agreement", accessed: "2026-10-23" }
  - { field: automated_trading_allowed, url: "https://help.tradeify.co/en/articles/10468318-guidelines-for-traders", accessed: "2026-10-23" }
  - { field: automated_trading_allowed, url: "https://tradeify.co/funded-trader-agreement", accessed: "2026-10-23" }
  - { field: vps_allowed, url: "https://help.tradeify.co/en/articles/12268494-common-faqs", accessed: "2026-10-23" }
  - { field: vps_allowed, url: "https://tradeify.co/terms-of-use", accessed: "2026-10-23" }
```

#### Lightning — funded — 100K
```yaml
firm: tradeify
plan: "Lightning Funded"
phase: funded
size_label: "100K"
size_usd: "100000.00"
instruments: [MNQ, NQ]
platforms: [Tradovate, NinjaTrader (via Tradovate), TradingView (via Tradovate), Rithmic, Tradesea (Rithmic), Quantower (Rithmic), Sierra Chart (Rithmic), R|Trader (Rithmic), WealthCharts]
drawdown: { amount_usd: "4000.00", type: trailing_eod, stops_trailing_at: "Locks when EOD balance reaches $104,100 or on payout request (whichever first); floor fixed at starting balance + $100" }
daily_loss_limit: { amount_usd: "2500.00", kind: soft_pause }   # soft breach; increases to $4,000 from the next session once balance reaches $106,000 (6% profit)
profit_target_usd: null
consistency_rule: { text: "Gradual Consistency Rule: First payout 20%, second payout 25%, third and subsequent payouts 30% (accounts bought after Sep 12, 2025 8:00 AM ET; older: 20% for all)", threshold_pct: "20/25/30" }
max_position: { minis: 8, micros: 80, scaling: "Maximum from day one; legacy (before Sep 12, 2025): 50K 5/50, 100K 10/100, 150K 15/150" }
min_trading_days: null   # 'No Minimum Trading Day Count'
trading_day_reset: { time: "18:00", tz_as_published: "ET" }   # trading day "6:00 PM ET to 5:00 PM ET"; DLL "resets at the start of each trading session (6:00 PM ET)"; flat by 4:45 PM ET
copy_trading_allowed: conditional   # only between accounts you personally own and manage, all in the same direction; copying to/from other people prohibited; copying to/from other firms not stated (bots: not across multiple firms)
automated_trading_allowed: conditional   # bots/algos allowed if solely owned, not shared with other traders/firms, not HFT, not used across multiple firms; ownership may be verified
automation_rule_text: |
  At Tradeify, we allow the use of bots and algorithms under certain conditions:
  Ownership: You must be able to prove that you are the sole owner of the bot or strategy, and that no one else has access to or is using it. This ensures that the bot/algorithm is not being shared with other traders or firms.
  We scan to ensure there are no similar orders on other accounts. We will also require a live video of you enabling the code on your own PC.
  Exclusive Use: While you may use the bot on your personal accounts, using it across multiple firms is against Tradeify's policy. The bot should be solely for your own use within Tradeify.
  No High-Frequency Trading (HFT) Bots: Personal bots are allowed as long as they are not high-frequency trading (HFT) bots. Tradeify has specific risk measures in place to detect and monitor such activities.
  Compliance and Verification: Tradeify reserves the right to request information or documentation if our risk measures flag your account for potential violations of these guidelines.
vps_allowed: conditional   # FAQ: no VPN/VPS for login; VPS for trading 'at your own risk' after a normal login; Terms forbid VPS that obscures device/identity verification
personal_device_required: unknown   # not stated for manual orders; for bots the firm may 'require a live video of you enabling the code on your own PC'
notes: "Straight to sim funded, no evaluation. Payout profit goal (Payout 1 / Payout 2+): $6,000 / $3,500; no minimum trading days. 10-second rule gates payouts."
status: verified
not_confirmed_fields: []
sources:
  - { field: drawdown, url: "https://help.tradeify.co/en/articles/10495938-lightning-funded-accounts", accessed: "2026-10-23" }
  - { field: drawdown, url: "https://help.tradeify.co/en/articles/10495897-rules-trailing-max-drawdowns", accessed: "2026-10-23" }
  - { field: daily_loss_limit, url: "https://help.tradeify.co/en/articles/10495938-lightning-funded-accounts", accessed: "2026-10-23" }
  - { field: daily_loss_limit, url: "https://help.tradeify.co/en/articles/10468321-rules-daily-loss-limit", accessed: "2026-10-23" }
  - { field: daily_loss_limit, url: "https://help.tradeify.co/en/articles/12268167-essential-trading-rules-overview", accessed: "2026-10-23" }
  - { field: consistency_rule, url: "https://help.tradeify.co/en/articles/10468320-rules-consistency-rule", accessed: "2026-10-23" }
  - { field: consistency_rule, url: "https://help.tradeify.co/en/articles/10495932-lightning-funded-account-payout-policy", accessed: "2026-10-23" }
  - { field: max_position, url: "https://help.tradeify.co/en/articles/10495938-lightning-funded-accounts", accessed: "2026-10-23" }
  - { field: payout, url: "https://help.tradeify.co/en/articles/10495932-lightning-funded-account-payout-policy", accessed: "2026-10-23" }
  - { field: drawdown, url: "https://tradeify.co/", accessed: "2026-10-23" }
  - { field: platforms, url: "https://help.tradeify.co/en/articles/10468221-supported-platforms", accessed: "2026-10-23" }
  - { field: trading_day_reset, url: "https://help.tradeify.co/en/articles/10468225-what-is-a-trading-day", accessed: "2026-10-23" }
  - { field: trading_day_reset, url: "https://help.tradeify.co/en/articles/10468321-rules-daily-loss-limit", accessed: "2026-10-23" }
  - { field: copy_trading_allowed, url: "https://help.tradeify.co/en/articles/10495868-rules-hedging-correlated-products", accessed: "2026-10-23" }
  - { field: copy_trading_allowed, url: "https://help.tradeify.co/en/articles/13196343-tradovate-order-entry-copy-trading", accessed: "2026-10-23" }
  - { field: copy_trading_allowed, url: "https://help.tradeify.co/en/articles/13825156-wealthcharts-order-entry-trade-copier", accessed: "2026-10-23" }
  - { field: copy_trading_allowed, url: "https://tradeify.co/funded-trader-agreement", accessed: "2026-10-23" }
  - { field: automated_trading_allowed, url: "https://help.tradeify.co/en/articles/10468318-guidelines-for-traders", accessed: "2026-10-23" }
  - { field: automated_trading_allowed, url: "https://tradeify.co/funded-trader-agreement", accessed: "2026-10-23" }
  - { field: vps_allowed, url: "https://help.tradeify.co/en/articles/12268494-common-faqs", accessed: "2026-10-23" }
  - { field: vps_allowed, url: "https://tradeify.co/terms-of-use", accessed: "2026-10-23" }
```

#### Lightning — funded — 150K
```yaml
firm: tradeify
plan: "Lightning Funded"
phase: funded
size_label: "150K"
size_usd: "150000.00"
instruments: [MNQ, NQ]
platforms: [Tradovate, NinjaTrader (via Tradovate), TradingView (via Tradovate), Rithmic, Tradesea (Rithmic), Quantower (Rithmic), Sierra Chart (Rithmic), R|Trader (Rithmic), WealthCharts]
drawdown: { amount_usd: "5250.00", type: trailing_eod, stops_trailing_at: "Locks when EOD balance reaches $155,350 or on payout request (whichever first); floor fixed at starting balance + $100" }
daily_loss_limit: { amount_usd: "3000.00", kind: soft_pause }   # soft breach; increases to $5,250 from the next session once balance reaches $159,000 (6% profit)
profit_target_usd: null
consistency_rule: { text: "Gradual Consistency Rule: First payout 20%, second payout 25%, third and subsequent payouts 30% (accounts bought after Sep 12, 2025 8:00 AM ET; older: 20% for all)", threshold_pct: "20/25/30" }
max_position: { minis: 12, micros: 120, scaling: "Maximum from day one; legacy (before Sep 12, 2025): 50K 5/50, 100K 10/100, 150K 15/150" }
min_trading_days: null   # 'No Minimum Trading Day Count'
trading_day_reset: { time: "18:00", tz_as_published: "ET" }   # trading day "6:00 PM ET to 5:00 PM ET"; DLL "resets at the start of each trading session (6:00 PM ET)"; flat by 4:45 PM ET
copy_trading_allowed: conditional   # only between accounts you personally own and manage, all in the same direction; copying to/from other people prohibited; copying to/from other firms not stated (bots: not across multiple firms)
automated_trading_allowed: conditional   # bots/algos allowed if solely owned, not shared with other traders/firms, not HFT, not used across multiple firms; ownership may be verified
automation_rule_text: |
  At Tradeify, we allow the use of bots and algorithms under certain conditions:
  Ownership: You must be able to prove that you are the sole owner of the bot or strategy, and that no one else has access to or is using it. This ensures that the bot/algorithm is not being shared with other traders or firms.
  We scan to ensure there are no similar orders on other accounts. We will also require a live video of you enabling the code on your own PC.
  Exclusive Use: While you may use the bot on your personal accounts, using it across multiple firms is against Tradeify's policy. The bot should be solely for your own use within Tradeify.
  No High-Frequency Trading (HFT) Bots: Personal bots are allowed as long as they are not high-frequency trading (HFT) bots. Tradeify has specific risk measures in place to detect and monitor such activities.
  Compliance and Verification: Tradeify reserves the right to request information or documentation if our risk measures flag your account for potential violations of these guidelines.
vps_allowed: conditional   # FAQ: no VPN/VPS for login; VPS for trading 'at your own risk' after a normal login; Terms forbid VPS that obscures device/identity verification
personal_device_required: unknown   # not stated for manual orders; for bots the firm may 'require a live video of you enabling the code on your own PC'
notes: "Straight to sim funded, no evaluation. Payout profit goal (Payout 1 / Payout 2+): $9,000 / $4,500; no minimum trading days. 10-second rule gates payouts. 150K bought before March 31: DLL $3,750; 150K bought before the new dashboard launch: DLL $3,750 and drawdown $6,000."
status: verified
not_confirmed_fields: []
sources:
  - { field: drawdown, url: "https://help.tradeify.co/en/articles/10495938-lightning-funded-accounts", accessed: "2026-10-23" }
  - { field: drawdown, url: "https://help.tradeify.co/en/articles/10495897-rules-trailing-max-drawdowns", accessed: "2026-10-23" }
  - { field: daily_loss_limit, url: "https://help.tradeify.co/en/articles/10495938-lightning-funded-accounts", accessed: "2026-10-23" }
  - { field: daily_loss_limit, url: "https://help.tradeify.co/en/articles/10468321-rules-daily-loss-limit", accessed: "2026-10-23" }
  - { field: daily_loss_limit, url: "https://help.tradeify.co/en/articles/12268167-essential-trading-rules-overview", accessed: "2026-10-23" }
  - { field: consistency_rule, url: "https://help.tradeify.co/en/articles/10468320-rules-consistency-rule", accessed: "2026-10-23" }
  - { field: consistency_rule, url: "https://help.tradeify.co/en/articles/10495932-lightning-funded-account-payout-policy", accessed: "2026-10-23" }
  - { field: max_position, url: "https://help.tradeify.co/en/articles/10495938-lightning-funded-accounts", accessed: "2026-10-23" }
  - { field: payout, url: "https://help.tradeify.co/en/articles/10495932-lightning-funded-account-payout-policy", accessed: "2026-10-23" }
  - { field: drawdown, url: "https://tradeify.co/", accessed: "2026-10-23" }
  - { field: platforms, url: "https://help.tradeify.co/en/articles/10468221-supported-platforms", accessed: "2026-10-23" }
  - { field: trading_day_reset, url: "https://help.tradeify.co/en/articles/10468225-what-is-a-trading-day", accessed: "2026-10-23" }
  - { field: trading_day_reset, url: "https://help.tradeify.co/en/articles/10468321-rules-daily-loss-limit", accessed: "2026-10-23" }
  - { field: copy_trading_allowed, url: "https://help.tradeify.co/en/articles/10495868-rules-hedging-correlated-products", accessed: "2026-10-23" }
  - { field: copy_trading_allowed, url: "https://help.tradeify.co/en/articles/13196343-tradovate-order-entry-copy-trading", accessed: "2026-10-23" }
  - { field: copy_trading_allowed, url: "https://help.tradeify.co/en/articles/13825156-wealthcharts-order-entry-trade-copier", accessed: "2026-10-23" }
  - { field: copy_trading_allowed, url: "https://tradeify.co/funded-trader-agreement", accessed: "2026-10-23" }
  - { field: automated_trading_allowed, url: "https://help.tradeify.co/en/articles/10468318-guidelines-for-traders", accessed: "2026-10-23" }
  - { field: automated_trading_allowed, url: "https://tradeify.co/funded-trader-agreement", accessed: "2026-10-23" }
  - { field: vps_allowed, url: "https://help.tradeify.co/en/articles/12268494-common-faqs", accessed: "2026-10-23" }
  - { field: vps_allowed, url: "https://tradeify.co/terms-of-use", accessed: "2026-10-23" }
```

#### Tradeify Elite — live — 25K
```yaml
firm: tradeify
plan: "Tradeify Elite Live"
phase: live
size_label: "25K"
size_usd: "25000.00"
instruments: [MNQ, NQ]
platforms: null   # not stated for Elite Live accounts
drawdown: { amount_usd: "1500.00", type: trailing_eod, stops_trailing_at: "Starts at $0 balance; locks when EOD balance reaches $1,600 (drawdown + $100)" }
daily_loss_limit: { amount_usd: null, kind: none }   # 'No DLL'
profit_target_usd: null
consistency_rule: { text: null, threshold_pct: null }
max_position: { minis: 2, micros: 20, scaling: "$0 -> DD: 1 mini / 10 micro; above DD: 2 mini / 20 micro" }
min_trading_days: null   # no minimum; at least one filled trade every 30 days or the account closes
trading_day_reset: { time: "18:00", tz_as_published: "ET" }   # trading day "6:00 PM ET to 5:00 PM ET"; DLL "resets at the start of each trading session (6:00 PM ET)"; flat by 4:45 PM ET
copy_trading_allowed: conditional   # only between accounts you personally own and manage, all in the same direction; copying to/from other people prohibited; copying to/from other firms not stated (bots: not across multiple firms)
automated_trading_allowed: conditional   # bots/algos allowed if solely owned, not shared with other traders/firms, not HFT, not used across multiple firms; ownership may be verified
automation_rule_text: |
  At Tradeify, we allow the use of bots and algorithms under certain conditions:
  Ownership: You must be able to prove that you are the sole owner of the bot or strategy, and that no one else has access to or is using it. This ensures that the bot/algorithm is not being shared with other traders or firms.
  We scan to ensure there are no similar orders on other accounts. We will also require a live video of you enabling the code on your own PC.
  Exclusive Use: While you may use the bot on your personal accounts, using it across multiple firms is against Tradeify's policy. The bot should be solely for your own use within Tradeify.
  No High-Frequency Trading (HFT) Bots: Personal bots are allowed as long as they are not high-frequency trading (HFT) bots. Tradeify has specific risk measures in place to detect and monitor such activities.
  Compliance and Verification: Tradeify reserves the right to request information or documentation if our risk measures flag your account for potential violations of these guidelines.
vps_allowed: conditional   # FAQ: no VPN/VPS for login; VPS for trading 'at your own risk' after a normal login; Terms forbid VPS that obscures device/identity verification
personal_device_required: unknown   # not stated for manual orders; for bots the firm may 'require a live video of you enabling the code on your own PC'
notes: "Invitation only (3 payouts on one account or 10 total payouts are minimum thresholds for consideration). Size label = the sim size that transitioned. 80/20 split; daily payouts. Accounts purchased before Dec 3, 2025 may instead be on the Legacy Live program."
status: partial
not_confirmed_fields: [platforms]
sources:
  - { field: drawdown, url: "https://help.tradeify.co/en/articles/12969284-tradeify-elite-program", accessed: "2026-10-23" }
  - { field: daily_loss_limit, url: "https://help.tradeify.co/en/articles/12969284-tradeify-elite-program", accessed: "2026-10-23" }
  - { field: max_position, url: "https://help.tradeify.co/en/articles/12969284-tradeify-elite-program", accessed: "2026-10-23" }
  - { field: trading_day_reset, url: "https://help.tradeify.co/en/articles/10495876-rules-permitted-times-to-trade", accessed: "2026-10-23" }
  - { field: platforms, url: "https://help.tradeify.co/en/articles/10468221-supported-platforms", accessed: "2026-10-23" }
  - { field: trading_day_reset, url: "https://help.tradeify.co/en/articles/10468225-what-is-a-trading-day", accessed: "2026-10-23" }
  - { field: trading_day_reset, url: "https://help.tradeify.co/en/articles/10468321-rules-daily-loss-limit", accessed: "2026-10-23" }
  - { field: copy_trading_allowed, url: "https://help.tradeify.co/en/articles/10495868-rules-hedging-correlated-products", accessed: "2026-10-23" }
  - { field: copy_trading_allowed, url: "https://help.tradeify.co/en/articles/13196343-tradovate-order-entry-copy-trading", accessed: "2026-10-23" }
  - { field: copy_trading_allowed, url: "https://help.tradeify.co/en/articles/13825156-wealthcharts-order-entry-trade-copier", accessed: "2026-10-23" }
  - { field: copy_trading_allowed, url: "https://tradeify.co/funded-trader-agreement", accessed: "2026-10-23" }
  - { field: automated_trading_allowed, url: "https://help.tradeify.co/en/articles/10468318-guidelines-for-traders", accessed: "2026-10-23" }
  - { field: automated_trading_allowed, url: "https://tradeify.co/funded-trader-agreement", accessed: "2026-10-23" }
  - { field: vps_allowed, url: "https://help.tradeify.co/en/articles/12268494-common-faqs", accessed: "2026-10-23" }
  - { field: vps_allowed, url: "https://tradeify.co/terms-of-use", accessed: "2026-10-23" }
```

#### Tradeify Elite — live — 50K
```yaml
firm: tradeify
plan: "Tradeify Elite Live"
phase: live
size_label: "50K"
size_usd: "50000.00"
instruments: [MNQ, NQ]
platforms: null   # not stated for Elite Live accounts
drawdown: { amount_usd: "2000.00", type: trailing_eod, stops_trailing_at: "Starts at $0 balance; locks when EOD balance reaches $2,100 (drawdown + $100)" }
daily_loss_limit: { amount_usd: null, kind: none }   # 'No DLL'
profit_target_usd: null
consistency_rule: { text: null, threshold_pct: null }
max_position: { minis: 4, micros: 40, scaling: "$0 -> DD: 2 mini / 20 micro; above DD: 4 mini / 40 micro" }
min_trading_days: null   # no minimum; at least one filled trade every 30 days or the account closes
trading_day_reset: { time: "18:00", tz_as_published: "ET" }   # trading day "6:00 PM ET to 5:00 PM ET"; DLL "resets at the start of each trading session (6:00 PM ET)"; flat by 4:45 PM ET
copy_trading_allowed: conditional   # only between accounts you personally own and manage, all in the same direction; copying to/from other people prohibited; copying to/from other firms not stated (bots: not across multiple firms)
automated_trading_allowed: conditional   # bots/algos allowed if solely owned, not shared with other traders/firms, not HFT, not used across multiple firms; ownership may be verified
automation_rule_text: |
  At Tradeify, we allow the use of bots and algorithms under certain conditions:
  Ownership: You must be able to prove that you are the sole owner of the bot or strategy, and that no one else has access to or is using it. This ensures that the bot/algorithm is not being shared with other traders or firms.
  We scan to ensure there are no similar orders on other accounts. We will also require a live video of you enabling the code on your own PC.
  Exclusive Use: While you may use the bot on your personal accounts, using it across multiple firms is against Tradeify's policy. The bot should be solely for your own use within Tradeify.
  No High-Frequency Trading (HFT) Bots: Personal bots are allowed as long as they are not high-frequency trading (HFT) bots. Tradeify has specific risk measures in place to detect and monitor such activities.
  Compliance and Verification: Tradeify reserves the right to request information or documentation if our risk measures flag your account for potential violations of these guidelines.
vps_allowed: conditional   # FAQ: no VPN/VPS for login; VPS for trading 'at your own risk' after a normal login; Terms forbid VPS that obscures device/identity verification
personal_device_required: unknown   # not stated for manual orders; for bots the firm may 'require a live video of you enabling the code on your own PC'
notes: "Invitation only (3 payouts on one account or 10 total payouts are minimum thresholds for consideration). Size label = the sim size that transitioned. 80/20 split; daily payouts. Accounts purchased before Dec 3, 2025 may instead be on the Legacy Live program."
status: partial
not_confirmed_fields: [platforms]
sources:
  - { field: drawdown, url: "https://help.tradeify.co/en/articles/12969284-tradeify-elite-program", accessed: "2026-10-23" }
  - { field: daily_loss_limit, url: "https://help.tradeify.co/en/articles/12969284-tradeify-elite-program", accessed: "2026-10-23" }
  - { field: max_position, url: "https://help.tradeify.co/en/articles/12969284-tradeify-elite-program", accessed: "2026-10-23" }
  - { field: trading_day_reset, url: "https://help.tradeify.co/en/articles/10495876-rules-permitted-times-to-trade", accessed: "2026-10-23" }
  - { field: platforms, url: "https://help.tradeify.co/en/articles/10468221-supported-platforms", accessed: "2026-10-23" }
  - { field: trading_day_reset, url: "https://help.tradeify.co/en/articles/10468225-what-is-a-trading-day", accessed: "2026-10-23" }
  - { field: trading_day_reset, url: "https://help.tradeify.co/en/articles/10468321-rules-daily-loss-limit", accessed: "2026-10-23" }
  - { field: copy_trading_allowed, url: "https://help.tradeify.co/en/articles/10495868-rules-hedging-correlated-products", accessed: "2026-10-23" }
  - { field: copy_trading_allowed, url: "https://help.tradeify.co/en/articles/13196343-tradovate-order-entry-copy-trading", accessed: "2026-10-23" }
  - { field: copy_trading_allowed, url: "https://help.tradeify.co/en/articles/13825156-wealthcharts-order-entry-trade-copier", accessed: "2026-10-23" }
  - { field: copy_trading_allowed, url: "https://tradeify.co/funded-trader-agreement", accessed: "2026-10-23" }
  - { field: automated_trading_allowed, url: "https://help.tradeify.co/en/articles/10468318-guidelines-for-traders", accessed: "2026-10-23" }
  - { field: automated_trading_allowed, url: "https://tradeify.co/funded-trader-agreement", accessed: "2026-10-23" }
  - { field: vps_allowed, url: "https://help.tradeify.co/en/articles/12268494-common-faqs", accessed: "2026-10-23" }
  - { field: vps_allowed, url: "https://tradeify.co/terms-of-use", accessed: "2026-10-23" }
```

#### Tradeify Elite — live — 100K
```yaml
firm: tradeify
plan: "Tradeify Elite Live"
phase: live
size_label: "100K"
size_usd: "100000.00"
instruments: [MNQ, NQ]
platforms: null   # not stated for Elite Live accounts
drawdown: { amount_usd: "3000.00", type: trailing_eod, stops_trailing_at: "Starts at $0 balance; locks when EOD balance reaches $3,100 (drawdown + $100)" }
daily_loss_limit: { amount_usd: null, kind: none }   # 'No DLL'
profit_target_usd: null
consistency_rule: { text: null, threshold_pct: null }
max_position: { minis: 8, micros: 80, scaling: "$0 -> DD: 4 mini / 40 micro; above DD: 8 mini / 80 micro" }
min_trading_days: null   # no minimum; at least one filled trade every 30 days or the account closes
trading_day_reset: { time: "18:00", tz_as_published: "ET" }   # trading day "6:00 PM ET to 5:00 PM ET"; DLL "resets at the start of each trading session (6:00 PM ET)"; flat by 4:45 PM ET
copy_trading_allowed: conditional   # only between accounts you personally own and manage, all in the same direction; copying to/from other people prohibited; copying to/from other firms not stated (bots: not across multiple firms)
automated_trading_allowed: conditional   # bots/algos allowed if solely owned, not shared with other traders/firms, not HFT, not used across multiple firms; ownership may be verified
automation_rule_text: |
  At Tradeify, we allow the use of bots and algorithms under certain conditions:
  Ownership: You must be able to prove that you are the sole owner of the bot or strategy, and that no one else has access to or is using it. This ensures that the bot/algorithm is not being shared with other traders or firms.
  We scan to ensure there are no similar orders on other accounts. We will also require a live video of you enabling the code on your own PC.
  Exclusive Use: While you may use the bot on your personal accounts, using it across multiple firms is against Tradeify's policy. The bot should be solely for your own use within Tradeify.
  No High-Frequency Trading (HFT) Bots: Personal bots are allowed as long as they are not high-frequency trading (HFT) bots. Tradeify has specific risk measures in place to detect and monitor such activities.
  Compliance and Verification: Tradeify reserves the right to request information or documentation if our risk measures flag your account for potential violations of these guidelines.
vps_allowed: conditional   # FAQ: no VPN/VPS for login; VPS for trading 'at your own risk' after a normal login; Terms forbid VPS that obscures device/identity verification
personal_device_required: unknown   # not stated for manual orders; for bots the firm may 'require a live video of you enabling the code on your own PC'
notes: "Invitation only (3 payouts on one account or 10 total payouts are minimum thresholds for consideration). Size label = the sim size that transitioned. 80/20 split; daily payouts. Accounts purchased before Dec 3, 2025 may instead be on the Legacy Live program."
status: partial
not_confirmed_fields: [platforms]
sources:
  - { field: drawdown, url: "https://help.tradeify.co/en/articles/12969284-tradeify-elite-program", accessed: "2026-10-23" }
  - { field: daily_loss_limit, url: "https://help.tradeify.co/en/articles/12969284-tradeify-elite-program", accessed: "2026-10-23" }
  - { field: max_position, url: "https://help.tradeify.co/en/articles/12969284-tradeify-elite-program", accessed: "2026-10-23" }
  - { field: trading_day_reset, url: "https://help.tradeify.co/en/articles/10495876-rules-permitted-times-to-trade", accessed: "2026-10-23" }
  - { field: platforms, url: "https://help.tradeify.co/en/articles/10468221-supported-platforms", accessed: "2026-10-23" }
  - { field: trading_day_reset, url: "https://help.tradeify.co/en/articles/10468225-what-is-a-trading-day", accessed: "2026-10-23" }
  - { field: trading_day_reset, url: "https://help.tradeify.co/en/articles/10468321-rules-daily-loss-limit", accessed: "2026-10-23" }
  - { field: copy_trading_allowed, url: "https://help.tradeify.co/en/articles/10495868-rules-hedging-correlated-products", accessed: "2026-10-23" }
  - { field: copy_trading_allowed, url: "https://help.tradeify.co/en/articles/13196343-tradovate-order-entry-copy-trading", accessed: "2026-10-23" }
  - { field: copy_trading_allowed, url: "https://help.tradeify.co/en/articles/13825156-wealthcharts-order-entry-trade-copier", accessed: "2026-10-23" }
  - { field: copy_trading_allowed, url: "https://tradeify.co/funded-trader-agreement", accessed: "2026-10-23" }
  - { field: automated_trading_allowed, url: "https://help.tradeify.co/en/articles/10468318-guidelines-for-traders", accessed: "2026-10-23" }
  - { field: automated_trading_allowed, url: "https://tradeify.co/funded-trader-agreement", accessed: "2026-10-23" }
  - { field: vps_allowed, url: "https://help.tradeify.co/en/articles/12268494-common-faqs", accessed: "2026-10-23" }
  - { field: vps_allowed, url: "https://tradeify.co/terms-of-use", accessed: "2026-10-23" }
```

#### Tradeify Elite — live — 150K
```yaml
firm: tradeify
plan: "Tradeify Elite Live"
phase: live
size_label: "150K"
size_usd: "150000.00"
instruments: [MNQ, NQ]
platforms: null   # not stated for Elite Live accounts
drawdown: { amount_usd: "4500.00", type: trailing_eod, stops_trailing_at: "Starts at $0 balance; locks when EOD balance reaches $4,600 (drawdown + $100)" }
daily_loss_limit: { amount_usd: null, kind: none }   # 'No DLL'
profit_target_usd: null
consistency_rule: { text: null, threshold_pct: null }
max_position: { minis: 12, micros: 120, scaling: "$0 -> DD: 6 mini / 40 micro (as published); above DD: 12 mini / 120 micro" }
min_trading_days: null   # no minimum; at least one filled trade every 30 days or the account closes
trading_day_reset: { time: "18:00", tz_as_published: "ET" }   # trading day "6:00 PM ET to 5:00 PM ET"; DLL "resets at the start of each trading session (6:00 PM ET)"; flat by 4:45 PM ET
copy_trading_allowed: conditional   # only between accounts you personally own and manage, all in the same direction; copying to/from other people prohibited; copying to/from other firms not stated (bots: not across multiple firms)
automated_trading_allowed: conditional   # bots/algos allowed if solely owned, not shared with other traders/firms, not HFT, not used across multiple firms; ownership may be verified
automation_rule_text: |
  At Tradeify, we allow the use of bots and algorithms under certain conditions:
  Ownership: You must be able to prove that you are the sole owner of the bot or strategy, and that no one else has access to or is using it. This ensures that the bot/algorithm is not being shared with other traders or firms.
  We scan to ensure there are no similar orders on other accounts. We will also require a live video of you enabling the code on your own PC.
  Exclusive Use: While you may use the bot on your personal accounts, using it across multiple firms is against Tradeify's policy. The bot should be solely for your own use within Tradeify.
  No High-Frequency Trading (HFT) Bots: Personal bots are allowed as long as they are not high-frequency trading (HFT) bots. Tradeify has specific risk measures in place to detect and monitor such activities.
  Compliance and Verification: Tradeify reserves the right to request information or documentation if our risk measures flag your account for potential violations of these guidelines.
vps_allowed: conditional   # FAQ: no VPN/VPS for login; VPS for trading 'at your own risk' after a normal login; Terms forbid VPS that obscures device/identity verification
personal_device_required: unknown   # not stated for manual orders; for bots the firm may 'require a live video of you enabling the code on your own PC'
notes: "The starting tier '$0 -> DD: 6 mini / 40 micro (as published)' does not follow the 10:1 mini:micro ratio the firm uses elsewhere; recorded as published, needs confirmation. Invitation only (3 payouts on one account or 10 total payouts are minimum thresholds for consideration). Size label = the sim size that transitioned. 80/20 split; daily payouts. Accounts purchased before Dec 3, 2025 may instead be on the Legacy Live program."
status: partial
not_confirmed_fields: [max_position.starting_tier_micros, platforms]
sources:
  - { field: drawdown, url: "https://help.tradeify.co/en/articles/12969284-tradeify-elite-program", accessed: "2026-10-23" }
  - { field: daily_loss_limit, url: "https://help.tradeify.co/en/articles/12969284-tradeify-elite-program", accessed: "2026-10-23" }
  - { field: max_position, url: "https://help.tradeify.co/en/articles/12969284-tradeify-elite-program", accessed: "2026-10-23" }
  - { field: trading_day_reset, url: "https://help.tradeify.co/en/articles/10495876-rules-permitted-times-to-trade", accessed: "2026-10-23" }
  - { field: platforms, url: "https://help.tradeify.co/en/articles/10468221-supported-platforms", accessed: "2026-10-23" }
  - { field: trading_day_reset, url: "https://help.tradeify.co/en/articles/10468225-what-is-a-trading-day", accessed: "2026-10-23" }
  - { field: trading_day_reset, url: "https://help.tradeify.co/en/articles/10468321-rules-daily-loss-limit", accessed: "2026-10-23" }
  - { field: copy_trading_allowed, url: "https://help.tradeify.co/en/articles/10495868-rules-hedging-correlated-products", accessed: "2026-10-23" }
  - { field: copy_trading_allowed, url: "https://help.tradeify.co/en/articles/13196343-tradovate-order-entry-copy-trading", accessed: "2026-10-23" }
  - { field: copy_trading_allowed, url: "https://help.tradeify.co/en/articles/13825156-wealthcharts-order-entry-trade-copier", accessed: "2026-10-23" }
  - { field: copy_trading_allowed, url: "https://tradeify.co/funded-trader-agreement", accessed: "2026-10-23" }
  - { field: automated_trading_allowed, url: "https://help.tradeify.co/en/articles/10468318-guidelines-for-traders", accessed: "2026-10-23" }
  - { field: automated_trading_allowed, url: "https://tradeify.co/funded-trader-agreement", accessed: "2026-10-23" }
  - { field: vps_allowed, url: "https://help.tradeify.co/en/articles/12268494-common-faqs", accessed: "2026-10-23" }
  - { field: vps_allowed, url: "https://tradeify.co/terms-of-use", accessed: "2026-10-23" }
```

#### Tradeify Elite — live — 300K
```yaml
firm: tradeify
plan: "Tradeify Elite Live"
phase: live
size_label: "300K"
size_usd: "300000.00"
instruments: [MNQ, NQ]
platforms: null   # not stated for Elite Live accounts
drawdown: { amount_usd: "6000.00", type: trailing_eod, stops_trailing_at: "Starts at $0 balance; locks when EOD balance reaches $6,100 (drawdown + $100)" }
daily_loss_limit: { amount_usd: null, kind: none }   # 'No DLL'
profit_target_usd: null
consistency_rule: { text: null, threshold_pct: null }
max_position: { minis: 10, micros: 100, scaling: "$0 -> DD: 8 mini / 60 micro (as published); above DD: 10 mini / 100 micro" }
min_trading_days: null   # no minimum; at least one filled trade every 30 days or the account closes
trading_day_reset: { time: "18:00", tz_as_published: "ET" }   # trading day "6:00 PM ET to 5:00 PM ET"; DLL "resets at the start of each trading session (6:00 PM ET)"; flat by 4:45 PM ET
copy_trading_allowed: conditional   # only between accounts you personally own and manage, all in the same direction; copying to/from other people prohibited; copying to/from other firms not stated (bots: not across multiple firms)
automated_trading_allowed: conditional   # bots/algos allowed if solely owned, not shared with other traders/firms, not HFT, not used across multiple firms; ownership may be verified
automation_rule_text: |
  At Tradeify, we allow the use of bots and algorithms under certain conditions:
  Ownership: You must be able to prove that you are the sole owner of the bot or strategy, and that no one else has access to or is using it. This ensures that the bot/algorithm is not being shared with other traders or firms.
  We scan to ensure there are no similar orders on other accounts. We will also require a live video of you enabling the code on your own PC.
  Exclusive Use: While you may use the bot on your personal accounts, using it across multiple firms is against Tradeify's policy. The bot should be solely for your own use within Tradeify.
  No High-Frequency Trading (HFT) Bots: Personal bots are allowed as long as they are not high-frequency trading (HFT) bots. Tradeify has specific risk measures in place to detect and monitor such activities.
  Compliance and Verification: Tradeify reserves the right to request information or documentation if our risk measures flag your account for potential violations of these guidelines.
vps_allowed: conditional   # FAQ: no VPN/VPS for login; VPS for trading 'at your own risk' after a normal login; Terms forbid VPS that obscures device/identity verification
personal_device_required: unknown   # not stated for manual orders; for bots the firm may 'require a live video of you enabling the code on your own PC'
notes: "The starting tier '$0 -> DD: 8 mini / 60 micro (as published)' does not follow the 10:1 mini:micro ratio the firm uses elsewhere; recorded as published, needs confirmation. Invitation only (3 payouts on one account or 10 total payouts are minimum thresholds for consideration). Size label = the sim size that transitioned. 80/20 split; daily payouts. Accounts purchased before Dec 3, 2025 may instead be on the Legacy Live program."
status: partial
not_confirmed_fields: [max_position.starting_tier_micros, platforms]
sources:
  - { field: drawdown, url: "https://help.tradeify.co/en/articles/12969284-tradeify-elite-program", accessed: "2026-10-23" }
  - { field: daily_loss_limit, url: "https://help.tradeify.co/en/articles/12969284-tradeify-elite-program", accessed: "2026-10-23" }
  - { field: max_position, url: "https://help.tradeify.co/en/articles/12969284-tradeify-elite-program", accessed: "2026-10-23" }
  - { field: trading_day_reset, url: "https://help.tradeify.co/en/articles/10495876-rules-permitted-times-to-trade", accessed: "2026-10-23" }
  - { field: platforms, url: "https://help.tradeify.co/en/articles/10468221-supported-platforms", accessed: "2026-10-23" }
  - { field: trading_day_reset, url: "https://help.tradeify.co/en/articles/10468225-what-is-a-trading-day", accessed: "2026-10-23" }
  - { field: trading_day_reset, url: "https://help.tradeify.co/en/articles/10468321-rules-daily-loss-limit", accessed: "2026-10-23" }
  - { field: copy_trading_allowed, url: "https://help.tradeify.co/en/articles/10495868-rules-hedging-correlated-products", accessed: "2026-10-23" }
  - { field: copy_trading_allowed, url: "https://help.tradeify.co/en/articles/13196343-tradovate-order-entry-copy-trading", accessed: "2026-10-23" }
  - { field: copy_trading_allowed, url: "https://help.tradeify.co/en/articles/13825156-wealthcharts-order-entry-trade-copier", accessed: "2026-10-23" }
  - { field: copy_trading_allowed, url: "https://tradeify.co/funded-trader-agreement", accessed: "2026-10-23" }
  - { field: automated_trading_allowed, url: "https://help.tradeify.co/en/articles/10468318-guidelines-for-traders", accessed: "2026-10-23" }
  - { field: automated_trading_allowed, url: "https://tradeify.co/funded-trader-agreement", accessed: "2026-10-23" }
  - { field: vps_allowed, url: "https://help.tradeify.co/en/articles/12268494-common-faqs", accessed: "2026-10-23" }
  - { field: vps_allowed, url: "https://tradeify.co/terms-of-use", accessed: "2026-10-23" }
```
## 9. Unverified / gaps / conflicts (for the user report)

| # | Field | Status | What is missing or conflicting | Where searched |
|---|---|---|---|---|
| 1 | Select Evaluation 25K `max_position` | **conflict** | Select Evaluation article: "Maximum Contracts: 1 mini / 10 micro". The homepage 25K card says "1 mini / 10 micros". The Select payout policy article says "During evaluation, you have access to the full contract limits (2/20, 4/40, 8/80, or 12/120 depending on account size)". The 50K–150K values agree | select-evaluation-accounts; select-flex-and-select-daily-payout-policies; tradeify.co cards |
| 2 | Select 300K V2 evaluation and Flex DLL ($4,000) `daily_loss_limit.kind` | not confirmed | The amount is stated, but no official text says whether this DLL is a soft or hard breach. The DLL article's "Applies to" lists only Growth, Lightning and Select Daily funded accounts | 300k-select-account; rules-daily-loss-limit |
| 3 | Tradeify Elite 150K / 300K starting contracts | not confirmed | Published as "6 mini / 40 micro" (150K) and "8 mini / 60 micro" (300K). These do not match the firm's stated 10:1 ratio. The 300K "above DD" tier (10/100) is also below the 150K tier. Recorded as published | tradeify-elite-program |
| 4 | Copying to/from accounts at other firms | unverified (not stated) | Same-person copying is allowed and copying with other people is forbidden. No text says whether same-direction copying to or from another prop firm is allowed. Only bots ("using it across multiple firms is against Tradeify's policy") and opposing positions vs External Accounts (FTA §6.7(c)/(i)) are addressed | copier articles (Tradovate, WealthCharts, Tradesea), hedging, Risk & Compliance FAQ, FTA, Terms |
| 5 | `personal_device_required` | unknown | Not stated for manual orders. For bots: "a live video of you enabling the code on your own PC". It is not stated whether a bot on a VPS qualifies. VPS trading is allowed "at your own risk" after a normal login (Common FAQs), but the Terms forbid a VPS that prevents device or identity verification | common-faqs; terms-of-use; guidelines-for-traders; risk-compliance FAQ |
| 6 | Microscalping / 10-second payout rule | conflict (wording) | Help center: both "Over 50% of your trades are longer than 10 seconds" AND "Over 50% of your profit…". FTA §6.8: only "50% or more of the Trader’s account profit must come from trades held longer than ten (10) seconds". This is a payout gate, not a preset field | guidelines-for-traders; what-is-a-trading-day; funded-trader-agreement |
| 7 | DCA / size consistency | conflict | Guidelines: "DCA is allowed, but it should always be part of a structured strategy". FTA §6.5: "avoiding dollar-cost averaging and high-frequency trading" and "avoiding maximum or larger contract sizes on single trades". This may affect copy-sizing presets | guidelines-for-traders; funded-trader-agreement |
| 8 | Elite Live `platforms` | not stated | The Elite page does not name platforms or connections for live accounts | tradeify-elite-program; legacy-live guide |
| 9 | Webhooks / alert automation | not mentioned | A help-center search for "webhook" returned nothing. TradingView is listed only as a platform via Tradovate | help-center search, supported-platforms |
| 10 | "Advanced Challenge" | not covered | Named in FTA §4.1 and in the homepage statistics ("Growth, Advanced and Select evaluation products"), but no current help-center article or homepage plan exists. Treated as legacy | help-center collections; tradeify.co |
| 11 | Help-center collections | partial listing | The "Trading Platforms & Products" (21) and "Accounts & Rules" (19) collections showed fewer article links than their counts. Missing articles were found by search for the key terms (copier, copy, automated, ATM, bracket, webhook, bot, 300K, WealthCharts). Some unrelated platform or competition articles may not have been read | collection pages, search |
| 12 | Lightning DLL wording | note (not a conflict) | "Which Plan is Right for You?" says "Daily loss limits apply" for Lightning in general. The size tables and Essential Rules say the 25K has none | which-plan-is-right-for-you; lightning-funded-accounts; essential-rules |

Pages not reached: none. All cited pages were read on 2026-10-23. The help center was read through the "Lupa Tradeify" browser portal (curl gets 403). tradeify.co was read by curl (FTA, Terms) and by the portal (rendered plan cards for Growth / Select / Lightning × Tradovate / WealthCharts / Rithmic). Restricted Countries was not captured, since it is not needed for presets.

## 10. Changelog

- 2026-10-23 — initial research (Lupa). Format rev. 2 + PO additions (automation detail) + rev. 3 (VPS / personal device).
