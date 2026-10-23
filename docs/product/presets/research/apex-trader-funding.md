# Apex Trader Funding — Preset Research

| | |
|---|---|
| **Firm** | Apex Trader Funding (Apex Trader Funding Inc., Austin, TX) |
| **Official domains** | `apextraderfunding.com` (site, help center at `/help-center/`, news at `/news/`). `support.apextraderfunding.com` redirects to a login and was **not** used. |
| **Research date** | 2026-10-23 (all URLs below accessed 2026-10-23) |
| **Researcher** | Lupa (Prop Firm Rules Researcher) |
| **Format** | preset-research-format.md rev. 2 (platforms, automation classification, "not confirmed") |
| **File status** | `partial` — every numeric field for current products is sourced, but several official pages conflict (see §9), and the "User Agreement" that the help center refers to for automation details is not public. |
| **Instruments covered** | MNQ, NQ (rules below apply to all Apex instruments; NQ/MNQ are listed as tradable, closing 5 PM ET — [Futures Trading Times](https://apextraderfunding.com/help-center/getting-started/futures-trading-times/)) |
| **Platforms** | Tradovate, Rithmic, WealthCharts for every current plan and size (see §2). The evaluation trailing-stop rule differs by platform, so evaluation YAML blocks are split by platform. |

**Product context.** Apex retired its earlier products on 2026-03-01 and now sells two evaluation paths, each leading to a Performance Account (PA, "Simulated Funded"): **EOD Trailing Drawdown** and **Intraday Trailing Drawdown**, in sizes 25K, 50K, 100K, 150K. Each is sold as "Standard" or "No Activation Fee"; both have the same trading rules. Older products are called **Legacy**. Whether Legacy products are still for sale is a **conflict** (see §3.5 and §9).

> "Apex Trader Funding has retired the previous versions of the Evaluation and Performance accounts. As of March 1st, 2026, these prior versions are no longer available for purchase." — https://apextraderfunding.com/help-center/legacy-products/legacy-products-overview/

> "No Activation Fee Evaluations follow the same rules, drawdown structure, access period, and qualification process as standard Evaluations." — https://apextraderfunding.com/help-center/uncategorized/no-activation-fee-evaluations/

I checked this on the homepage product picker: the account rules are the same for every type × fee mode × size combination (evidence: `homepage-product-picker.txt`).

---

## 1. Source index (all accessed 2026-10-23)

| Key | URL |
|---|---|
| S-EODEVAL | https://apextraderfunding.com/help-center/eod-trailing-drawdown-accounts/eod-evaluations/ |
| S-EODPA | https://apextraderfunding.com/help-center/eod-trailing-drawdown-accounts/eod-performance-accounts-pa/ |
| S-EODDD | https://apextraderfunding.com/help-center/eod-trailing-drawdown-accounts/eod-drawdown-explained/ |
| S-EODPAY | https://apextraderfunding.com/help-center/eod-trailing-drawdown-accounts/eod-payouts/ |
| S-INTEVAL | https://apextraderfunding.com/help-center/evaluation-accounts-ea/intraday-trailing-drawdown-evaluations/ |
| S-INTPA | https://apextraderfunding.com/help-center/intraday-trailing-drawdown-accounts/intraday-trailing-drawdown-performance-accounts-pa/ |
| S-INTDD | https://apextraderfunding.com/help-center/intraday-trailing-drawdown-accounts/intraday-trailing-drawdown-explained/ |
| S-INTPAY | https://apextraderfunding.com/help-center/intraday-trailing-drawdown-accounts/intraday-trailing-drawdown-payouts/ |
| S-DLL | https://apextraderfunding.com/help-center/additional-helpful-items/daily-loss-limit-explained/ |
| S-SCALE | https://apextraderfunding.com/help-center/additional-helpful-items/scaling-levels-pa-explained/ |
| S-CONS | https://apextraderfunding.com/help-center/additional-helpful-items/50-consistency-requirement/ |
| S-HOME | https://apextraderfunding.com/ (homepage product picker: "ACCOUNT RULES" card per type/size) |
| S-TIMES | https://apextraderfunding.com/help-center/getting-started/futures-trading-times/ |
| S-HOLIDAY | https://apextraderfunding.com/help-center/helpful-items/holiday-trading/ |
| S-PROHIB | https://apextraderfunding.com/help-center/getting-started/prohibited-activities/ |
| S-MAXPA | https://apextraderfunding.com/help-center/performance-accounts-pa/how-many-paid-funded-accounts-am-i-allowed-to-have/ |
| S-HEDGE | https://apextraderfunding.com/help-center/legacy-helpful-items/hedging-and-correlated-instruments-rule/ |
| S-GROUP | https://apextraderfunding.com/help-center/tradovate/tradovate-group-copier/ |
| S-TVFAQ | https://apextraderfunding.com/help-center/tradovate/tradovate-frequently-asked-questions/ |
| S-TVAPI | https://apextraderfunding.com/help-center/tradovate/tradovate-api-request-limit/ |
| S-PLAT | https://apextraderfunding.com/help-center/getting-started/choosing-the-right-platform/ |
| S-LEGCOMP | https://apextraderfunding.com/help-center/performance-accounts-pa/legacy-performance-account-pa-compliance/ |
| S-USERACC | https://apextraderfunding.com/help-center/getting-started/apex-trader-funding-user-accounts/ |
| S-INACT | https://apextraderfunding.com/help-center/billing/inactivity-policy-on-performance-accounts-pa/ |
| S-ACCESS | https://apextraderfunding.com/help-center/billing/evaluation-plan-fees-and-access-explained/ |
| S-ACTIV | https://apextraderfunding.com/help-center/billing/pa-activation-process-deadline-explained/ |
| S-NOFEE | https://apextraderfunding.com/help-center/uncategorized/no-activation-fee-evaluations/ |
| S-LEGOV | https://apextraderfunding.com/help-center/legacy-products/legacy-products-overview/ |
| S-LEGPROD | https://apextraderfunding.com/legacy-products |
| S-LEGEVAL | https://apextraderfunding.com/help-center/evaluation-accounts-ea/legacy-evaluation-rules/ |
| S-LEGPA | https://apextraderfunding.com/help-center/legacy-products/legacy-performance-account-pa-trading-rules/ |
| S-LIVE | https://apextraderfunding.com/help-center/getting-started/apex-live-prop-trading-program-faq/ |
| S-TOU | https://apextraderfunding.com/terms-of-use/ (Effective Date: October 22, 2025) |
| S-TVCONN | https://apextraderfunding.com/help-center/tradovate/tradingview-connection-and-possible-issues/ |
| S-NTRITH | https://apextraderfunding.com/help-center/rithmic/connection-guide-for-ninjatrader-using-rithmic/ (title from site search; page not captured) |
| S-NTTV | https://apextraderfunding.com/help-center/tradovate/tradovate-x-ninjatrader8-connection-guide/ (title from site search; page not captured) |

The raw text of each page is kept as an audit trail in the researcher's scratchpad evidence folder (one `<slug>.txt` / `<slug>.full.txt` per page, with the URL on line 1). The `.full.txt` captures include the collapsed FAQ answers.

---

## 2. Platforms

| Plan | Platforms (trading account vendor) as published | Source |
|---|---|---|
| EOD Trailing Drawdown: Evaluation and PA, 25K / 50K / 100K / 150K, Standard and No Activation Fee | Tradovate, Rithmic, Wealthcharts | S-HOME product picker (vendor list read for each of the 16 type × fee × size combinations; evidence `homepage-vendors.txt`) |
| Intraday Trailing Drawdown: Evaluation and PA, all sizes, both fee modes | Tradovate, Rithmic, Wealthcharts | same |
| Legacy (§3.5) | Tradovate, WealthCharts, Rithmic | S-LEGPROD ("TRADOVATE / WEALTHCHARTS / RITHMIC" tabs) |

- "Trading accounts are what you purchase to place your trades in on your preferred trading platform. … They may be Rithmic, Tradovate, or WealthCharts accounts." — S-USERACC
- A PA keeps the evaluation's platform: "Can I Convert a Paid/Performance Account Between Tradovate and Rithmic? — No, accounts cannot be transferred." — S-TVFAQ
- **NinjaTrader** is a front end on top of Rithmic or Tradovate, not a separate vendor. Apex publishes NinjaTrader connection guides for both (S-NTRITH, S-NTTV; titles from the site search, pages not captured). License: "Ensure you have a paid NinjaTrader license key (purchased directly from NinjaTrader, or included in Apex accounts subscriptions)." — S-TVFAQ
- **TradingView** connects through Tradovate: "To connect Tradovate to TradingView, you must have an active, tradable account." — S-TVCONN
- Instruments: "**Exception: Certain exchanges only available on Tradovate Platform" — S-HOME (this concerns EUREX, not NQ/MNQ).
- **The only rule found that differs by platform** is when the evaluation drawdown stops trailing (S-EODDD, S-INTDD): "Evaluations Rithmic and Wealthcharts" stop trailing at the profit-target level, and "Evaluations Tradovate … trails indefinitely". Evaluation YAML blocks are therefore split into `[Rithmic, WealthCharts]` and `[Tradovate]`. No platform-specific PA rule was published, so each PA size is one block with all three platforms.
- For All Win (NinjaTrader 8 plugin): an Apex account appears in NinjaTrader as a Rithmic or Tradovate connection. The user must pick the matching preset variant for evaluations.

---

## 3. Plans

Both current plans use the same sizes, profit targets, drawdown amounts and evaluation contract limits. They differ in **how the drawdown trails** and in the **evaluation Daily Loss Limit (DLL)**: the EOD evaluation has a DLL, and the Intraday evaluation has none.

### 3.1 EOD Trailing Drawdown — Evaluation

Common rules (S-EODEVAL):
- "EOD Drawdown calculated once per day at market close and enforced following trading session"
- "No minimum trading days required, may pass in one trading day". The homepage card shows "Min. Days To Pass 1" (S-HOME).
- 30-day access period. "the Evaluation expires automatically at 6:00 PM ET on Day 30" (S-ACCESS).
- "Does hitting the Daily Loss Limit fail my evaluation? — No. Reaching DLL pauses trading for the remainder of the session. Your account remains active." (S-EODEVAL FAQ) → DLL kind **soft_pause**.
- Contract limits are fixed, with no scaling ("Scaling: Not Applied"). Consistency: "Not Applied".
- **Drawdown type `trailing_eod`.** "The EOD Threshold is recalculated once per trading day at 4:59:59 PM ET, based on the account's closing balance. It always trails the highest achieved EOD balance and never moves downward." (S-EODDD). It is enforced in real time against the balance including open positions: "If at any moment during the trading session the account balance touches or falls below the EOD Threshold, all open positions are automatically liquidated, and the evaluation is immediately failed." (S-EODEVAL)
- **When it stops trailing (evaluation) depends on the platform** (S-EODDD):
  - "Evaluations Rithmic and Wealthcharts — EOD Threshold stops trailing and becomes fixed when it reaches an amount equal to the Target Profit balance." Example: "50K … Threshold Stop Level = $53.000 … This occurs when the Highest EOD Balance closes above $55,000 ($53,000 + $2,000)."
  - "Evaluations Tradovate — EOD Drawdown trails indefinitely with the peak EOD account balance."

| Field | 25K | 50K | 100K | 150K | Source |
|---|---|---|---|---|---|
| Size (USD) | 25,000 | 50,000 | 100,000 | 150,000 | S-EODEVAL, S-HOME |
| Profit target | $1,500 | $3,000 | $6,000 | $9,000 | S-EODEVAL, S-HOME |
| Max drawdown (EOD) | $1,000 | $2,000 | $3,000 | $4,000 | S-EODEVAL, S-HOME |
| Daily loss limit (soft pause) | $500 | $1,000 | $1,500 | $2,000 | S-EODEVAL, S-DLL, S-HOME |
| Max contracts | 4 mini / 40 micro | 6 mini / 60 micro | 8 mini / 80 micro | 12 mini / 120 micro | S-HOME (S-EODEVAL: 4 / 6 / 8 / 12) |
| Consistency | Not Applied | Not Applied | Not Applied | Not Applied | S-EODEVAL |
| Min trading days | none required (homepage: "Min. Days To Pass 1") | same | same | same | S-EODEVAL, S-HOME |
| After passing | 7 calendar days to activate the PA | same | same | same | S-EODEVAL, S-ACTIV |

### 3.2 EOD Trailing Drawdown — Performance Account (PA)

- "The End-of-Day (EOD) Performance Account is a Simulated Funded (Sim Funded) account awarded after you pass the EOD Evaluation." (S-EODPA)
- **Drawdown `trailing_eod`; it stops trailing at Starting Balance + $100.** "Once the EOD Threshold reaches Starting Balance + $100, it stops increasing. Example for a 50K EOD PA: Threshold Stop Level = $50,100 … This occurs when the Highest EOD Balance reaches $52,100 (Starting Balance + Max Drawdown + $100)." (S-EODDD). A breach closes the PA permanently.
- **DLL is tiered** and its kind is **soft_pause**: "If it is reached → positions close automatically → trading pauses for the day … Your account remains active … DLL resets daily at 6PM ET" (S-DLL).
- **Scaling (tiers).** Max contracts and DLL depend on a tier. The tier is set once per day from the closing balance at 4:59:59 PM ET and applies to the next session (S-SCALE). "Level 1 Floor: If your ending account balance drops below the Level 1 threshold, you remain at Level 1." Orders above the limit are rejected without penalty.
- **Micros:** "ten (10) micro contracts equal one (1) standard contract. All exposure is calculated using this contract equivalency." The limit is combined across all instruments (S-SCALE FAQ).
- **Consistency (payout eligibility only):** 50%. See §3.6. The account is not failed for breaking it.
- **Payout prerequisites that affect trading** (S-EODPAY): 5 qualifying trading days, each with at least the minimum daily profit (25K $100 / 50K $250 / 100K $300 / 150K $350). "The Safety Net is calculated as your account's drawdown limit plus $100. This threshold must be maintained for the lifetime of the Performance Account." Safety Net: $26,100 / $52,100 / $103,100 / $154,100. Max 6 payouts per PA, after which "the PA is closed".
- **Inactivity:** "you must record at least 2 trading days with $50 or more in net profit within every rolling 30-day period" or the PA is closed (S-INACT).

| Field | 25K | 50K | 100K | 150K | Source |
|---|---|---|---|---|---|
| Max drawdown (EOD) | $1,000 | $2,000 | $3,000 | $4,000 | S-EODPA |
| Trailing stops at | Starting Balance + $100 | ($50,100 example) | Starting Balance + $100 | Starting Balance + $100 | S-EODDD |
| Max contracts (top tier) | 2 | 4 | 6 | 10 | S-EODPA, S-SCALE |
| DLL | tier-based (below) | tier-based | tier-based | tier-based | S-EODPA, S-DLL |
| Profit target | none | none | none | none | S-EODPA (not listed) |
| Consistency | 50% (payout) | 50% | 50% | 50% | S-CONS, S-EODPAY |
| Min days | 5 qualifying days per payout request | same | same | same | S-EODPAY |

**PA tier tables** (S-DLL, and the same tables on S-SCALE; "Profit Range" as published):

| Size | Level | Profit range | Max contracts | DLL |
|---|---|---|---|---|
| 25K | 1 | $0 – $999 | 1 | $500 |
| 25K | 2 | $1,000 – $1,999 | 2 | $500 |
| 25K | 3 | $2,000 & Up | 2 | $1,250 |
| 50K | 1 | $0 – $1,499 | 2 | $1,000 |
| 50K | 2 | $1,500 – $2,999 | 3 | $1,000 |
| 50K | 3 | $3,000 – $5,999 | 4 | $2,000 |
| 50K | 4 | **$6,000 & Up** (S-DLL) / **$5,999 & Up** (S-SCALE) → `conflict` | 4 | $3,000 |
| 100K | 1 | $0 – $1,999 | 3 | $1,750 |
| 100K | 2 | $2,000 – $2,999 | 4 | $1,750 |
| 100K | 3 | $3,000 – $4,999 | 5 | $1,750 |
| 100K | 4 | $5,000 – $9,999 | 6 | $2,500 |
| 100K | 5 | $10,000 & Up | 6 | $3,500 |
| 150K | 1 | $0 – $1,999 | 4 | $2,500 |
| 150K | 2 | $2,000 – $2,999 | 5 | $2,500 |
| 150K | 3 | $3,000 – $4,999 | 7 | $2,500 |
| 150K | 4 | $5,000 – $9,999 | 10 | $3,000 |
| 150K | 5 | $10,000 & Up | 10 | $4,000 |

The same tier tables apply to the Intraday PA. S-DLL presents them under "DLL During PA" without separating plans, and S-INTPA marks "Scaling Tier Based: Yes" and "DLL Tier Based: Yes".

### 3.3 Intraday Trailing Drawdown — Evaluation

- "Real-time intraday trailing drawdown follows Peak Balance and never moves down" and "No Daily Loss Limit" (S-INTEVAL). FAQ: "No. The Intraday Evaluation does not have a Daily Loss Limit."
- **Drawdown type `trailing_intraday`.** Unrealized gains count: "The Peak Balance includes both realized and unrealized gains. If an open trade pushes your account to a new high, the Trailing Threshold adjusts upward immediately" (S-INTDD FAQ). It does not reset daily.
- **When it stops trailing (evaluation) depends on the platform** (S-INTDD):
  - Body text: "Evaluations Rithmic and Wealthcharts — Once the Intraday Threshold reaches Target Profit Balance, it stops increasing. Example for a 50K … Threshold Stop Level = $53.000. This occurs when the highest balance … reaches $55,000 (Profit Target Balance + Max Drawdown)."
  - FAQ text: "In Rithmic and Wealthcharts Evaluations, trailing stops once the Intraday Threshold reaches Profit Target Balance + $2,000." It gives the same example (stop level $53,000). The wording differs from the body text → `conflict` (wording; the example values agree).
  - "Evaluations Tradovate — Intraday Drawdown trails indefinitely with the peak account balance."
- Fixed contracts: "Contract limits remain fixed throughout the Evaluation." No consistency rule. No minimum days. 30-day access. 7 days to activate the PA.

| Field | 25K | 50K | 100K | 150K | Source |
|---|---|---|---|---|---|
| Profit target | $1,500 | $3,000 | $6,000 | $9,000 | S-INTEVAL, S-HOME |
| Max drawdown (intraday trailing) | $1,000 | $2,000 | $3,000 | $4,000 | S-INTEVAL, S-INTDD, S-HOME |
| Daily loss limit | none | none | none | none | S-INTEVAL, S-DLL |
| Max contracts | 4 mini / 40 micro | 6 mini / 60 micro | 8 mini / 80 micro | 12 mini / 120 micro | S-HOME (S-INTEVAL: 4/6/8/12) |
| Consistency | Not Applied | Not Applied | Not Applied | Not Applied | S-INTEVAL |
| Min trading days | none required (homepage "1") | same | same | same | S-INTEVAL, S-HOME |

### 3.4 Intraday Trailing Drawdown — Performance Account (PA)

- Max drawdown $1,000 / $2,000 / $3,000 / $4,000, intraday trailing, real time, including unrealized PnL. A breach closes the PA permanently (S-INTPA).
- **When it stops trailing — `conflict`:**
  - S-INTDD: "In Performance Accounts, trailing stops once Starting Balance + $100 is reached" and "Example for a 50K Intraday PA: Threshold Stop Level = $50,100."
  - S-INTPA ("at a Glance"): "Trailing stops once Max Trailing Drawdown Amount + $100 is reached".
- Max contracts (top tier) 2 / 4 / 6 / 10. DLL and scaling are tier-based (tables in §3.2) (S-INTPA, S-DLL, S-SCALE). The DLL is soft_pause: "Hitting the DLL pauses trading for the remainder of the session. The account remains active" (S-INTPA FAQ).
- Inactivity: "If you do not record at least two $50 net profit days within 30 consecutive calendar days, the account will be closed." (S-INTPA FAQ, S-INACT)
- Payout prerequisites (S-INTPAY): 5 qualifying days with minimum daily profit of 25K $100 / 50K $200 / 100K $250 / 150K $300. The Safety Net is the same as for the EOD PA ($26,100 / $52,100 / $103,100 / $154,100). 50% consistency. Max 6 payouts.

### 3.5 Legacy products (noted only; no presets drafted)

Legacy accounts follow different rules: a full trailing drawdown, a "Static" 100K account, 7 minimum trading days, the Legacy PA 30% MAE rule, a 5:1 risk-reward rule, and half-size contracts until the threshold stops trailing (S-LEGEVAL, S-LEGPA). The official pages **conflict** on whether they are still sold:
- S-LEGOV: "Can I buy more Legacy evaluations …? No, as of March 1st, 2026, Legacy evaluation accounts are no longer available for purchase to anyone."
- S-LEGPROD: "Legacy Accounts – Available Now – Limited Time. Legacy evaluation accounts across 6 balance tiers and 3 platforms, offered for a limited time." S-HOME: "Legacy Accounts Are Back For a Limited Time".
- S-INACT also mentions "promotional Legacy accounts purchased after March 1st, 2026".

The Legacy evaluation parameters published on S-LEGPROD are the same for Tradovate, WealthCharts and Rithmic. They are recorded here for reference only:

| Legacy eval "FULL" | 25K | 50K | 100K | 150K | 250K | 300K |
|---|---|---|---|---|---|---|
| Contracts | 4 (40 Micros) | 10 (100 Micros) | 14 (140 Micros) | 17 (170 Micros) | 27 (270 Micros) | 35 (350 Micros) |
| Profit Goal | $1,500 | $3,000 | $6,000 | $9,000 | $15,000 | $20,000 |
| Trailing Threshold | $1,500 | $2,500 | $3,000 | $5,000 | $6,500 | $7,500 |
| Daily Drawdown | None | None | None | None | None | None |

S-LEGEVAL also lists 75K Full (12 minis, $2,750) and 100K Static (2 minis, $625), which are not on the current Legacy sales page. Legacy PA rules were **not** researched in depth (gap §9). The PO must decide whether Legacy presets are needed.

### 3.6 50% consistency (PA, both plans)

> "The 50% Consistency ensures that no single trading day accounts for more than 50% of your total accumulated profit at the time of a payout request. … Your largest profitable trading day must represent less than 50% of your net profit since your last payout request, or since account inception if no payout has been made. If this condition is not met, the payout request option will not be available. The account remains active" — S-CONS

Published formula: "Highest Profit Day ÷ Total Profit = Consistency Percentage. The result must be below 50%." Under this rule a breach blocks payouts only and does not fail the account. **Conflict (wording):** the page's own example table marks "$2,000 … 50% … ✓ Consistency Met", which contradicts "must be below 50%". S-EODPAY uses "50% or more" as the failing condition, which agrees with "below 50%".

---

## 4. Trading-day reset

- **"A trading day is defined from 6 PM ET one day to 4:59 PM ET the next day."** — S-TIMES
- "The trading day resets at 6:00 PM ET." — S-EODEVAL FAQ. "The Daily Loss Limit resets at the start of the next trading day (6PM ET)" — S-DLL.
- Drawdown and tier calculation at **4:59:59 PM ET** (S-EODDD, S-SCALE).
- Flat requirement: **"All trades must be closed before 4:59 PM ET."** — S-TIMES (NQ/MNQ close "5 PM ET"; "Starting at 6:00 PM ET, you can start placing trades again.")
- Holidays (S-HOLIDAY): trading is allowed when the market is open. "trading activity during the holiday or half-day session will be combined with the next full trading day and counted as a single trading day." "the Daily Loss Limit will not reset after a holiday or half-day session. The Daily Loss Limit will reset at 6:00 PM ET following the next full trading day." Positions must be closed at early closes.
- Legacy: "Sundays: Trading on Sundays counts as part of Monday's trading day." (S-LEGEVAL)

For the preset: `trading_day_reset: { time: "18:00", tz_as_published: "ET" }`, with a holiday exception: there is no DLL reset after a holiday or half-day session.

---

## 5. Trade copying stance

**Same person's accounts — allowed (with a conflicting Legacy-labelled page; see below):**
> "Copy trading across PA accounts under your personal name and business name is allowed. However, the hedging rule still applies; all PAs must trade in the same direction and not hedge against correlated assets." — S-MAXPA (https://apextraderfunding.com/help-center/performance-accounts-pa/how-many-paid-funded-accounts-am-i-allowed-to-have/)

> "Tradovate has a limited trade copier built into it, known as Group Trade. Group Trading is automatically enabled for any Apex user with more than one active Tradovate account" … "Tradovate Groups (Trade Copier) forbids the use of Bracket orders (ATMs). If you want to use bracket orders (ATMs), you must use NinjaTrader." — S-GROUP

> "Can I Use the Apex Trade Copier to Trade Simultaneously on Tradovate, Rithmic, and My Personal Broker Accounts? — Yes, as long as the trade copier is connected to NinjaTrader." and "Does the Apex Investing Institute Trade Copier Work with Tradovate and Rithmic? — Yes, the trade copier works across Tradovate, Rithmic, and personal broker accounts when connected via NinjaTrader." — S-TVFAQ

> WealthCharts: "Built-in Trade Copier: Features a built-in trade copier allowing traders to execute trades across multiple accounts simultaneously." — S-PLAT

**Copying with other people — forbidden:**
> "Account and Resource Sharing: Sharing MAC addresses, computers, IPs, credit cards, or trade copying with other traders is strictly forbidden. Violations will result in account closure, forfeiture of funds, and potential additional verification or audits to ensure compliance." — S-PROHIB

**Conflicting text (on a page titled "Legacy Performance Account (PA) Compliance"):**
> "Copy or Trading Services Operations or Participation — Performance Accounts (PA) and Live Prop Accounts must be traded exclusively by the individual listed on the account. Under no circumstances can these accounts be managed or influenced by any other party, person, system, automated trading bot, copy trading service, or trade mirroring software." — S-LEGCOMP

> "Please note that the results from the use of any trade copiers or external programs are the sole responsibility of the trader listed on the account." — S-LEGCOMP

**Cross-account hedging:** "You may not hold opposing long and short positions at the same time in correlated instruments, contract sizes, or across multiple accounts. … Trading opposite positions with other traders in the same household or other related parties." (S-HEDGE). Evaluations: "You may not pass Evaluation Accounts by hedging them against each other." (S-EODEVAL, S-INTEVAL)

**Account-count limits:**
> "The total limit is 20 active PA accounts across all people in the same house, companies, and connections, whether the accounts are with Rithmic, Tradovate, or WealthCharts. … There are no limits to Evaluation accounts." — S-MAXPA

The same limit appears in S-EODPA/S-INTPA ("You may hold up to 20 PAs active at the same time") and in S-LEGOV, which counts Legacy, Intraday and EOD PAs together: "must not exceed 20".

Other limits: one personal and one business user account per person (S-USERACC). Tradovate's maximum is 100 contracts per order, including across a group, and "placing an order across 20 accounts may generate 20 separate requests" (S-TVAPI; this is a platform limit, not an Apex rule).

**Copying to/from accounts at other firms:** no official page names other prop firms. S-TVFAQ allows the trade copier across Apex accounts and the trader's "personal broker accounts" via NinjaTrader. → recorded as `unverified` for other prop firms (gap §9).

**Summary for preset:** the official pages disagree on same-person copying (C1), so the YAML records `copy_trading_allowed: unknown` with `not_confirmed_fields: [copy_trading_allowed]` and does not pick a side. Rev. 1 of this file recorded `conditional`. What the pages say: copying across the same person's (and their business's) PAs is allowed per S-MAXPA, all in the same direction, within the 20 active PA per household limit. Copying with other traders is forbidden on every page. S-LEGCOMP forbids "copy trading service, or trade mirroring software" (`conflict`, §9).

---

## 6. Automated trading stance

**Not allowed:**
> "No Automation or Algorithm Usage allowed: Rewards are intended to recognize human traders actively participating in the learning process, not to reward automated systems executing preprogrammed logic. Please see the user agreement for more details, and we encourage you to reach out if you have further questions." — S-PROHIB (https://apextraderfunding.com/help-center/getting-started/prohibited-activities/)

> "Manipulation of the simulated trading environment: Apex Trader Funding strictly prohibits manipulation or exploitation of the simulation environment in any way, including High Frequency Trading (HFT) or any other exploitative strategies. … this includes non-directional bracket trading, where orders are left open on both sides of the market" — S-PROHIB

> "At Apex Trader Funding, the use of automation is strictly prohibited on all account types. This includes any form of AI (Artificial Intelligence), Autobots, algorithms, fully automated trading systems, and high-frequency trading (HFTs). These types of systems that trade without active human intervention are not allowed. Any type of hands-off, set-and-forget, or set-and-walk-away trading, including systems that run continuously 24 hours a day, is strictly prohibited. … All software used must follow all guidelines of the User Agreement. We strongly encourage the submission of your software for approval before use." — S-LEGCOMP

> Live program: "Automation and algorithm usage are not allowed." — S-LIVE

- **Trade copiers** are not called automation. Apex documents Tradovate Group Trade, the WealthCharts copier and a NinjaTrader-connected copier (§5).
- **ATM brackets:** "Apex encourages the use of ATM (Advanced Trade Management) strategies, which automate stop-loss and profit-target levels." (S-LEGCOMP, Legacy-labelled page)
- **Webhooks / TradingView-driven order entry:** no official page mentions webhooks (the site search for "webhook" returned no results). Automated order entry from signals would fall under "No Automation or Algorithm Usage allowed". The help center only covers TradingView as a manual trading front end for Tradovate.
- The **Terms of Use** (S-TOU) have no trading-automation section. They defer to the "Evaluation and Performance Account User Agreement", which is not publicly available → `unverified (behind login / not public)`.

**Summary for preset:** `automated_trading_allowed: no`. Bots, algorithms, AI, HFT and set-and-forget systems are prohibited. Same-person copiers are documented positively but their status is not confirmed (C1), and ATM brackets are encouraged on the Legacy page. **Product note:** All Win's TradingView-webhook → automatic routing (product-vision §7) is automated order entry. On Apex this appears to conflict with "No Automation or Algorithm Usage allowed". Per product-vision §9 the preset ships with this stance clearly shown.

### 6.1 PO classification (rev. 2)

**(i) Classification: `forbidden`** (YAML `automated_trading_allowed: "no"`).

`automation_rule_text` (VERBATIM, for the end-user opt-in warning). Source: https://apextraderfunding.com/help-center/getting-started/prohibited-activities/, accessed 2026-10-23. This is the current-products Prohibited Activities page, chosen because it is not Legacy-labelled:

> No Automation or Algorithm Usage allowed: Rewards are intended to recognize human traders actively participating in the learning process, not to reward automated systems executing preprogrammed logic. Please see the user agreement for more details, and we encourage you to reach out if you have further questions.

(Copied from the captured page text; only a trailing non-breaking space was dropped.)

**(ii) Mentions of related techniques:**

| Topic | Mentioned? | Verbatim quote | Source |
|---|---|---|---|
| Semi-automated trading | not mentioned | The site search and captured pages contain no "semi-automated" wording. The closest text is "These types of systems that trade without active human intervention are not allowed." and "Any type of hands-off, set-and-forget, or set-and-walk-away trading … is strictly prohibited." | S-LEGCOMP |
| Trade copiers | yes | "Tradovate has a limited trade copier built into it, known as Group Trade. Group Trading is automatically enabled for any Apex user with more than one active Tradovate account" / "Copy trading across PA accounts under your personal name and business name is allowed." / "Please note that the results from the use of any trade copiers or external programs are the sole responsibility of the trader listed on the account." / (conflicting) "Under no circumstances can these accounts be managed or influenced by any other party, person, system, automated trading bot, copy trading service, or trade mirroring software." | S-GROUP, S-MAXPA, S-LEGCOMP |
| Webhooks / alerts | not mentioned (webhooks); alerts only as platform features | Webhooks: no result for "webhook" in the site search. Alerts (charting feature, not order entry): "Scanner and Price Alerts make trading stress-free, so set up your own conditions and get away from the computer screen" (WealthCharts FAQ). | site search; https://apextraderfunding.com/help-center/wealthcharts/common-questions-and-troubleshooting-for-wealthcharts/ |
| ATM / automatic brackets | yes | "ATM Strategies for Automation: Apex encourages the use of ATM (Advanced Trade Management) strategies, which automate stop-loss and profit-target levels." (Legacy-labelled page) / "Tradovate Groups (Trade Copier) forbids the use of Bracket orders (ATMs). If you want to use bracket orders (ATMs), you must use NinjaTrader." / Prohibited: "non-directional bracket trading, where orders are left open on both sides of the market" | S-LEGCOMP, S-GROUP, S-PROHIB |

**(iii) Differs by phase?** No difference found. S-PROHIB states the ban without naming a phase. S-LEGCOMP: "the use of automation is strictly prohibited on all account types" and "Using these types of automation will result in the immediate closure of your PA or Live account". Live program (S-LIVE): "Automation and algorithm usage are not allowed." The same `automation_rule_text` is therefore used for evaluation and funded blocks.

---

## 7. Other rules that affect routing or risk

| Rule | Text (verbatim excerpt) | Source |
|---|---|---|
| No overnight holding | "Holding open trade positions through the market close. Traders are responsible for ensuring that all open trade positions are closed prior to the market close." (prohibited) / "All trades must be closed before 4:59 PM ET." | S-PROHIB, S-TIMES |
| Hedging / one direction | "No Hedging of Any Kind – Directional Trading only. Holding both long and short positions simultaneously on the same or correlated instrument is strictly prohibited." Includes micro vs mini and across accounts. | S-PROHIB, S-HEDGE |
| News trading | "Trading during news is allowed for your normal trading strategy. Using "news trading" strategies that "chase the market" or place orders on both sides but to be lucky or gamble the outcome of news is not allowed." | S-PROHIB |
| Stop loss | "All trades must have either pending or mental stop losses and a well-defined risk management strategy." | S-PROHIB |
| Risk/reward | "setting a five-tick profit target with a 150-tick stop loss demonstrates unacceptable risk management." / "Using the Trailing Threshold as a Stop Loss … prohibited" | S-PROHIB |
| Max contracts enforcement | "If an order would cause your total exposure to exceed the maximum allowed size, it is automatically rejected." Limit is combined across instruments; 10 micros = 1 standard. | S-SCALE |
| Max accounts | 20 active PAs per household (all plans and platforms combined); evaluations unlimited; 1 personal + 1 business user account | S-MAXPA, S-USERACC |
| Inactivity (PA) | 2 days with ≥ $50 net profit per rolling 30 days, or the PA is closed | S-INACT |
| Evaluation access | 30 days; expires 6:00 PM ET on day 30; no resets ("There are no reset fees. If an Evaluation fails, the only way to continue is by purchasing a new one.") | S-ACCESS |
| PA activation | 7 calendar days after the evaluation is marked Passed; "As soon as an Evaluation from the new products is marked as Passed, it is automatically locked to prevent any additional trading." | S-ACTIV |
| Payout cap | 6 payouts per PA, after which the PA is closed | S-EODPAY, S-INTPAY |
| Trader probation | Failing the guidelines "may result in placement in Trader Probation." | S-PROHIB |
| VPN / shared devices | VPNs/proxies/cloud servers prohibited when used to disguise identity or location. Sharing MAC/IP/computers with other traders is forbidden. | S-PROHIB |
| Live program (out of scope) | Invitation only; sim accounts closed on selection; max 5 live accounts; "All trades must be closed by 4:50 PM ET." | S-LIVE |

---

## 8. Machine-readable blocks

24 blocks: per evaluation size, one block for `[Rithmic, WealthCharts]` and one for `[Tradovate]` (16 in all), plus one PA block per size with all three platforms (8). Every block carries `automated_trading_allowed`, `automation_rule_text` and `not_confirmed_fields`. All blocks have `status: conflict` because C1 (copy trading) applies to every block. Their numeric fields are verified. The tier structure is given under `max_position.scaling` for PAs. `minis` holds the top-tier maximum as published in the PA parameter table. `daily_loss_limit.amount_usd` holds the **Level 1** DLL, and the other tiers are in `scaling`. For PA micros the firm publishes only the ratio ("ten (10) micro contracts equal one (1) standard contract"), so `micros` is null for PAs.

```yaml
# ---------- EOD Trailing Drawdown (EOD Evaluation) — Evaluation (split by platform: trailing stop differs) ----------

- firm: apex-trader-funding
  plan: "EOD Trailing Drawdown (EOD Evaluation)"
  phase: evaluation
  size_label: "25K"
  size_usd: "25000.00"
  instruments: [MNQ, NQ]
  platforms: [Rithmic, WealthCharts]
  drawdown: { amount_usd: "1000.00", type: trailing_eod, stops_trailing_at: "EOD Threshold stops trailing and becomes fixed when it reaches an amount equal to the Target Profit balance." }
  daily_loss_limit: { amount_usd: "500.00", kind: soft_pause }
  profit_target_usd: "1500.00"
  consistency_rule: { text: "Not Applied", threshold_pct: null }
  max_position: { minis: "4", micros: "40", scaling: null }
  min_trading_days: 1   # "No minimum trading days required, may pass in one trading day"; homepage "Min. Days To Pass 1"
  trading_day_reset: { time: "18:00", tz_as_published: "ET" }
  copy_trading_allowed: unknown   # C1 conflict: same-person PA copying allowed (S-MAXPA) vs Legacy PA Compliance ban; other-trader copying forbidden
  automated_trading_allowed: "no"
  automation_rule_text: "No Automation or Algorithm Usage allowed: Rewards are intended to recognize human traders actively participating in the learning process, not to reward automated systems executing preprogrammed logic. Please see the user agreement for more details, and we encourage you to reach out if you have further questions."
  status: conflict   # numeric fields verified; see not_confirmed_fields
  not_confirmed_fields: [copy_trading_allowed]
  sources: 
    - { field: profit_target/drawdown/dll/max_contracts, url: "https://apextraderfunding.com/help-center/eod-trailing-drawdown-accounts/eod-evaluations/", accessed: "2026-10-23" }
    - { field: max_position.micros, url: "https://apextraderfunding.com/", accessed: "2026-10-23" }
    - { field: drawdown.type/stops_trailing_at, url: "https://apextraderfunding.com/help-center/eod-trailing-drawdown-accounts/eod-drawdown-explained/", accessed: "2026-10-23" }
    - { field: daily_loss_limit.kind, url: "https://apextraderfunding.com/help-center/additional-helpful-items/daily-loss-limit-explained/", accessed: "2026-10-23" }
    - { field: trading_day_reset, url: "https://apextraderfunding.com/help-center/getting-started/futures-trading-times/", accessed: "2026-10-23" }
    - { field: copy_trading_allowed (A), url: "https://apextraderfunding.com/help-center/performance-accounts-pa/how-many-paid-funded-accounts-am-i-allowed-to-have/", accessed: "2026-10-23" }
    - { field: copy_trading_allowed (B), url: "https://apextraderfunding.com/help-center/performance-accounts-pa/legacy-performance-account-pa-compliance/", accessed: "2026-10-23" }
    - { field: automation_rule_text, url: "https://apextraderfunding.com/help-center/getting-started/prohibited-activities/", accessed: "2026-10-23" }

- firm: apex-trader-funding
  plan: "EOD Trailing Drawdown (EOD Evaluation)"
  phase: evaluation
  size_label: "25K"
  size_usd: "25000.00"
  instruments: [MNQ, NQ]
  platforms: [Tradovate]
  drawdown: { amount_usd: "1000.00", type: trailing_eod, stops_trailing_at: "EOD Drawdown trails indefinitely with the peak EOD account balance." }
  daily_loss_limit: { amount_usd: "500.00", kind: soft_pause }
  profit_target_usd: "1500.00"
  consistency_rule: { text: "Not Applied", threshold_pct: null }
  max_position: { minis: "4", micros: "40", scaling: null }
  min_trading_days: 1   # "No minimum trading days required, may pass in one trading day"; homepage "Min. Days To Pass 1"
  trading_day_reset: { time: "18:00", tz_as_published: "ET" }
  copy_trading_allowed: unknown   # C1 conflict: same-person PA copying allowed (S-MAXPA) vs Legacy PA Compliance ban; other-trader copying forbidden
  automated_trading_allowed: "no"
  automation_rule_text: "No Automation or Algorithm Usage allowed: Rewards are intended to recognize human traders actively participating in the learning process, not to reward automated systems executing preprogrammed logic. Please see the user agreement for more details, and we encourage you to reach out if you have further questions."
  status: conflict   # numeric fields verified; see not_confirmed_fields
  not_confirmed_fields: [copy_trading_allowed]
  sources: 
    - { field: profit_target/drawdown/dll/max_contracts, url: "https://apextraderfunding.com/help-center/eod-trailing-drawdown-accounts/eod-evaluations/", accessed: "2026-10-23" }
    - { field: max_position.micros, url: "https://apextraderfunding.com/", accessed: "2026-10-23" }
    - { field: drawdown.type/stops_trailing_at, url: "https://apextraderfunding.com/help-center/eod-trailing-drawdown-accounts/eod-drawdown-explained/", accessed: "2026-10-23" }
    - { field: daily_loss_limit.kind, url: "https://apextraderfunding.com/help-center/additional-helpful-items/daily-loss-limit-explained/", accessed: "2026-10-23" }
    - { field: trading_day_reset, url: "https://apextraderfunding.com/help-center/getting-started/futures-trading-times/", accessed: "2026-10-23" }
    - { field: copy_trading_allowed (A), url: "https://apextraderfunding.com/help-center/performance-accounts-pa/how-many-paid-funded-accounts-am-i-allowed-to-have/", accessed: "2026-10-23" }
    - { field: copy_trading_allowed (B), url: "https://apextraderfunding.com/help-center/performance-accounts-pa/legacy-performance-account-pa-compliance/", accessed: "2026-10-23" }
    - { field: automation_rule_text, url: "https://apextraderfunding.com/help-center/getting-started/prohibited-activities/", accessed: "2026-10-23" }

- firm: apex-trader-funding
  plan: "EOD Trailing Drawdown (EOD Evaluation)"
  phase: evaluation
  size_label: "50K"
  size_usd: "50000.00"
  instruments: [MNQ, NQ]
  platforms: [Rithmic, WealthCharts]
  drawdown: { amount_usd: "2000.00", type: trailing_eod, stops_trailing_at: "EOD Threshold stops trailing and becomes fixed when it reaches an amount equal to the Target Profit balance. (published 50K example: $53,000)" }
  daily_loss_limit: { amount_usd: "1000.00", kind: soft_pause }
  profit_target_usd: "3000.00"
  consistency_rule: { text: "Not Applied", threshold_pct: null }
  max_position: { minis: "6", micros: "60", scaling: null }
  min_trading_days: 1   # "No minimum trading days required, may pass in one trading day"; homepage "Min. Days To Pass 1"
  trading_day_reset: { time: "18:00", tz_as_published: "ET" }
  copy_trading_allowed: unknown   # C1 conflict: same-person PA copying allowed (S-MAXPA) vs Legacy PA Compliance ban; other-trader copying forbidden
  automated_trading_allowed: "no"
  automation_rule_text: "No Automation or Algorithm Usage allowed: Rewards are intended to recognize human traders actively participating in the learning process, not to reward automated systems executing preprogrammed logic. Please see the user agreement for more details, and we encourage you to reach out if you have further questions."
  status: conflict   # numeric fields verified; see not_confirmed_fields
  not_confirmed_fields: [copy_trading_allowed]
  sources: 
    - { field: profit_target/drawdown/dll/max_contracts, url: "https://apextraderfunding.com/help-center/eod-trailing-drawdown-accounts/eod-evaluations/", accessed: "2026-10-23" }
    - { field: max_position.micros, url: "https://apextraderfunding.com/", accessed: "2026-10-23" }
    - { field: drawdown.type/stops_trailing_at, url: "https://apextraderfunding.com/help-center/eod-trailing-drawdown-accounts/eod-drawdown-explained/", accessed: "2026-10-23" }
    - { field: daily_loss_limit.kind, url: "https://apextraderfunding.com/help-center/additional-helpful-items/daily-loss-limit-explained/", accessed: "2026-10-23" }
    - { field: trading_day_reset, url: "https://apextraderfunding.com/help-center/getting-started/futures-trading-times/", accessed: "2026-10-23" }
    - { field: copy_trading_allowed (A), url: "https://apextraderfunding.com/help-center/performance-accounts-pa/how-many-paid-funded-accounts-am-i-allowed-to-have/", accessed: "2026-10-23" }
    - { field: copy_trading_allowed (B), url: "https://apextraderfunding.com/help-center/performance-accounts-pa/legacy-performance-account-pa-compliance/", accessed: "2026-10-23" }
    - { field: automation_rule_text, url: "https://apextraderfunding.com/help-center/getting-started/prohibited-activities/", accessed: "2026-10-23" }

- firm: apex-trader-funding
  plan: "EOD Trailing Drawdown (EOD Evaluation)"
  phase: evaluation
  size_label: "50K"
  size_usd: "50000.00"
  instruments: [MNQ, NQ]
  platforms: [Tradovate]
  drawdown: { amount_usd: "2000.00", type: trailing_eod, stops_trailing_at: "EOD Drawdown trails indefinitely with the peak EOD account balance." }
  daily_loss_limit: { amount_usd: "1000.00", kind: soft_pause }
  profit_target_usd: "3000.00"
  consistency_rule: { text: "Not Applied", threshold_pct: null }
  max_position: { minis: "6", micros: "60", scaling: null }
  min_trading_days: 1   # "No minimum trading days required, may pass in one trading day"; homepage "Min. Days To Pass 1"
  trading_day_reset: { time: "18:00", tz_as_published: "ET" }
  copy_trading_allowed: unknown   # C1 conflict: same-person PA copying allowed (S-MAXPA) vs Legacy PA Compliance ban; other-trader copying forbidden
  automated_trading_allowed: "no"
  automation_rule_text: "No Automation or Algorithm Usage allowed: Rewards are intended to recognize human traders actively participating in the learning process, not to reward automated systems executing preprogrammed logic. Please see the user agreement for more details, and we encourage you to reach out if you have further questions."
  status: conflict   # numeric fields verified; see not_confirmed_fields
  not_confirmed_fields: [copy_trading_allowed]
  sources: 
    - { field: profit_target/drawdown/dll/max_contracts, url: "https://apextraderfunding.com/help-center/eod-trailing-drawdown-accounts/eod-evaluations/", accessed: "2026-10-23" }
    - { field: max_position.micros, url: "https://apextraderfunding.com/", accessed: "2026-10-23" }
    - { field: drawdown.type/stops_trailing_at, url: "https://apextraderfunding.com/help-center/eod-trailing-drawdown-accounts/eod-drawdown-explained/", accessed: "2026-10-23" }
    - { field: daily_loss_limit.kind, url: "https://apextraderfunding.com/help-center/additional-helpful-items/daily-loss-limit-explained/", accessed: "2026-10-23" }
    - { field: trading_day_reset, url: "https://apextraderfunding.com/help-center/getting-started/futures-trading-times/", accessed: "2026-10-23" }
    - { field: copy_trading_allowed (A), url: "https://apextraderfunding.com/help-center/performance-accounts-pa/how-many-paid-funded-accounts-am-i-allowed-to-have/", accessed: "2026-10-23" }
    - { field: copy_trading_allowed (B), url: "https://apextraderfunding.com/help-center/performance-accounts-pa/legacy-performance-account-pa-compliance/", accessed: "2026-10-23" }
    - { field: automation_rule_text, url: "https://apextraderfunding.com/help-center/getting-started/prohibited-activities/", accessed: "2026-10-23" }

- firm: apex-trader-funding
  plan: "EOD Trailing Drawdown (EOD Evaluation)"
  phase: evaluation
  size_label: "100K"
  size_usd: "100000.00"
  instruments: [MNQ, NQ]
  platforms: [Rithmic, WealthCharts]
  drawdown: { amount_usd: "3000.00", type: trailing_eod, stops_trailing_at: "EOD Threshold stops trailing and becomes fixed when it reaches an amount equal to the Target Profit balance." }
  daily_loss_limit: { amount_usd: "1500.00", kind: soft_pause }
  profit_target_usd: "6000.00"
  consistency_rule: { text: "Not Applied", threshold_pct: null }
  max_position: { minis: "8", micros: "80", scaling: null }
  min_trading_days: 1   # "No minimum trading days required, may pass in one trading day"; homepage "Min. Days To Pass 1"
  trading_day_reset: { time: "18:00", tz_as_published: "ET" }
  copy_trading_allowed: unknown   # C1 conflict: same-person PA copying allowed (S-MAXPA) vs Legacy PA Compliance ban; other-trader copying forbidden
  automated_trading_allowed: "no"
  automation_rule_text: "No Automation or Algorithm Usage allowed: Rewards are intended to recognize human traders actively participating in the learning process, not to reward automated systems executing preprogrammed logic. Please see the user agreement for more details, and we encourage you to reach out if you have further questions."
  status: conflict   # numeric fields verified; see not_confirmed_fields
  not_confirmed_fields: [copy_trading_allowed]
  sources: 
    - { field: profit_target/drawdown/dll/max_contracts, url: "https://apextraderfunding.com/help-center/eod-trailing-drawdown-accounts/eod-evaluations/", accessed: "2026-10-23" }
    - { field: max_position.micros, url: "https://apextraderfunding.com/", accessed: "2026-10-23" }
    - { field: drawdown.type/stops_trailing_at, url: "https://apextraderfunding.com/help-center/eod-trailing-drawdown-accounts/eod-drawdown-explained/", accessed: "2026-10-23" }
    - { field: daily_loss_limit.kind, url: "https://apextraderfunding.com/help-center/additional-helpful-items/daily-loss-limit-explained/", accessed: "2026-10-23" }
    - { field: trading_day_reset, url: "https://apextraderfunding.com/help-center/getting-started/futures-trading-times/", accessed: "2026-10-23" }
    - { field: copy_trading_allowed (A), url: "https://apextraderfunding.com/help-center/performance-accounts-pa/how-many-paid-funded-accounts-am-i-allowed-to-have/", accessed: "2026-10-23" }
    - { field: copy_trading_allowed (B), url: "https://apextraderfunding.com/help-center/performance-accounts-pa/legacy-performance-account-pa-compliance/", accessed: "2026-10-23" }
    - { field: automation_rule_text, url: "https://apextraderfunding.com/help-center/getting-started/prohibited-activities/", accessed: "2026-10-23" }

- firm: apex-trader-funding
  plan: "EOD Trailing Drawdown (EOD Evaluation)"
  phase: evaluation
  size_label: "100K"
  size_usd: "100000.00"
  instruments: [MNQ, NQ]
  platforms: [Tradovate]
  drawdown: { amount_usd: "3000.00", type: trailing_eod, stops_trailing_at: "EOD Drawdown trails indefinitely with the peak EOD account balance." }
  daily_loss_limit: { amount_usd: "1500.00", kind: soft_pause }
  profit_target_usd: "6000.00"
  consistency_rule: { text: "Not Applied", threshold_pct: null }
  max_position: { minis: "8", micros: "80", scaling: null }
  min_trading_days: 1   # "No minimum trading days required, may pass in one trading day"; homepage "Min. Days To Pass 1"
  trading_day_reset: { time: "18:00", tz_as_published: "ET" }
  copy_trading_allowed: unknown   # C1 conflict: same-person PA copying allowed (S-MAXPA) vs Legacy PA Compliance ban; other-trader copying forbidden
  automated_trading_allowed: "no"
  automation_rule_text: "No Automation or Algorithm Usage allowed: Rewards are intended to recognize human traders actively participating in the learning process, not to reward automated systems executing preprogrammed logic. Please see the user agreement for more details, and we encourage you to reach out if you have further questions."
  status: conflict   # numeric fields verified; see not_confirmed_fields
  not_confirmed_fields: [copy_trading_allowed]
  sources: 
    - { field: profit_target/drawdown/dll/max_contracts, url: "https://apextraderfunding.com/help-center/eod-trailing-drawdown-accounts/eod-evaluations/", accessed: "2026-10-23" }
    - { field: max_position.micros, url: "https://apextraderfunding.com/", accessed: "2026-10-23" }
    - { field: drawdown.type/stops_trailing_at, url: "https://apextraderfunding.com/help-center/eod-trailing-drawdown-accounts/eod-drawdown-explained/", accessed: "2026-10-23" }
    - { field: daily_loss_limit.kind, url: "https://apextraderfunding.com/help-center/additional-helpful-items/daily-loss-limit-explained/", accessed: "2026-10-23" }
    - { field: trading_day_reset, url: "https://apextraderfunding.com/help-center/getting-started/futures-trading-times/", accessed: "2026-10-23" }
    - { field: copy_trading_allowed (A), url: "https://apextraderfunding.com/help-center/performance-accounts-pa/how-many-paid-funded-accounts-am-i-allowed-to-have/", accessed: "2026-10-23" }
    - { field: copy_trading_allowed (B), url: "https://apextraderfunding.com/help-center/performance-accounts-pa/legacy-performance-account-pa-compliance/", accessed: "2026-10-23" }
    - { field: automation_rule_text, url: "https://apextraderfunding.com/help-center/getting-started/prohibited-activities/", accessed: "2026-10-23" }

- firm: apex-trader-funding
  plan: "EOD Trailing Drawdown (EOD Evaluation)"
  phase: evaluation
  size_label: "150K"
  size_usd: "150000.00"
  instruments: [MNQ, NQ]
  platforms: [Rithmic, WealthCharts]
  drawdown: { amount_usd: "4000.00", type: trailing_eod, stops_trailing_at: "EOD Threshold stops trailing and becomes fixed when it reaches an amount equal to the Target Profit balance." }
  daily_loss_limit: { amount_usd: "2000.00", kind: soft_pause }
  profit_target_usd: "9000.00"
  consistency_rule: { text: "Not Applied", threshold_pct: null }
  max_position: { minis: "12", micros: "120", scaling: null }
  min_trading_days: 1   # "No minimum trading days required, may pass in one trading day"; homepage "Min. Days To Pass 1"
  trading_day_reset: { time: "18:00", tz_as_published: "ET" }
  copy_trading_allowed: unknown   # C1 conflict: same-person PA copying allowed (S-MAXPA) vs Legacy PA Compliance ban; other-trader copying forbidden
  automated_trading_allowed: "no"
  automation_rule_text: "No Automation or Algorithm Usage allowed: Rewards are intended to recognize human traders actively participating in the learning process, not to reward automated systems executing preprogrammed logic. Please see the user agreement for more details, and we encourage you to reach out if you have further questions."
  status: conflict   # numeric fields verified; see not_confirmed_fields
  not_confirmed_fields: [copy_trading_allowed]
  sources: 
    - { field: profit_target/drawdown/dll/max_contracts, url: "https://apextraderfunding.com/help-center/eod-trailing-drawdown-accounts/eod-evaluations/", accessed: "2026-10-23" }
    - { field: max_position.micros, url: "https://apextraderfunding.com/", accessed: "2026-10-23" }
    - { field: drawdown.type/stops_trailing_at, url: "https://apextraderfunding.com/help-center/eod-trailing-drawdown-accounts/eod-drawdown-explained/", accessed: "2026-10-23" }
    - { field: daily_loss_limit.kind, url: "https://apextraderfunding.com/help-center/additional-helpful-items/daily-loss-limit-explained/", accessed: "2026-10-23" }
    - { field: trading_day_reset, url: "https://apextraderfunding.com/help-center/getting-started/futures-trading-times/", accessed: "2026-10-23" }
    - { field: copy_trading_allowed (A), url: "https://apextraderfunding.com/help-center/performance-accounts-pa/how-many-paid-funded-accounts-am-i-allowed-to-have/", accessed: "2026-10-23" }
    - { field: copy_trading_allowed (B), url: "https://apextraderfunding.com/help-center/performance-accounts-pa/legacy-performance-account-pa-compliance/", accessed: "2026-10-23" }
    - { field: automation_rule_text, url: "https://apextraderfunding.com/help-center/getting-started/prohibited-activities/", accessed: "2026-10-23" }

- firm: apex-trader-funding
  plan: "EOD Trailing Drawdown (EOD Evaluation)"
  phase: evaluation
  size_label: "150K"
  size_usd: "150000.00"
  instruments: [MNQ, NQ]
  platforms: [Tradovate]
  drawdown: { amount_usd: "4000.00", type: trailing_eod, stops_trailing_at: "EOD Drawdown trails indefinitely with the peak EOD account balance." }
  daily_loss_limit: { amount_usd: "2000.00", kind: soft_pause }
  profit_target_usd: "9000.00"
  consistency_rule: { text: "Not Applied", threshold_pct: null }
  max_position: { minis: "12", micros: "120", scaling: null }
  min_trading_days: 1   # "No minimum trading days required, may pass in one trading day"; homepage "Min. Days To Pass 1"
  trading_day_reset: { time: "18:00", tz_as_published: "ET" }
  copy_trading_allowed: unknown   # C1 conflict: same-person PA copying allowed (S-MAXPA) vs Legacy PA Compliance ban; other-trader copying forbidden
  automated_trading_allowed: "no"
  automation_rule_text: "No Automation or Algorithm Usage allowed: Rewards are intended to recognize human traders actively participating in the learning process, not to reward automated systems executing preprogrammed logic. Please see the user agreement for more details, and we encourage you to reach out if you have further questions."
  status: conflict   # numeric fields verified; see not_confirmed_fields
  not_confirmed_fields: [copy_trading_allowed]
  sources: 
    - { field: profit_target/drawdown/dll/max_contracts, url: "https://apextraderfunding.com/help-center/eod-trailing-drawdown-accounts/eod-evaluations/", accessed: "2026-10-23" }
    - { field: max_position.micros, url: "https://apextraderfunding.com/", accessed: "2026-10-23" }
    - { field: drawdown.type/stops_trailing_at, url: "https://apextraderfunding.com/help-center/eod-trailing-drawdown-accounts/eod-drawdown-explained/", accessed: "2026-10-23" }
    - { field: daily_loss_limit.kind, url: "https://apextraderfunding.com/help-center/additional-helpful-items/daily-loss-limit-explained/", accessed: "2026-10-23" }
    - { field: trading_day_reset, url: "https://apextraderfunding.com/help-center/getting-started/futures-trading-times/", accessed: "2026-10-23" }
    - { field: copy_trading_allowed (A), url: "https://apextraderfunding.com/help-center/performance-accounts-pa/how-many-paid-funded-accounts-am-i-allowed-to-have/", accessed: "2026-10-23" }
    - { field: copy_trading_allowed (B), url: "https://apextraderfunding.com/help-center/performance-accounts-pa/legacy-performance-account-pa-compliance/", accessed: "2026-10-23" }
    - { field: automation_rule_text, url: "https://apextraderfunding.com/help-center/getting-started/prohibited-activities/", accessed: "2026-10-23" }

# ---------- Intraday Trailing Drawdown (Intraday Evaluation) — Evaluation (split by platform: trailing stop differs) ----------

- firm: apex-trader-funding
  plan: "Intraday Trailing Drawdown (Intraday Evaluation)"
  phase: evaluation
  size_label: "25K"
  size_usd: "25000.00"
  instruments: [MNQ, NQ]
  platforms: [Rithmic, WealthCharts]
  drawdown: { amount_usd: "1000.00", type: trailing_intraday, stops_trailing_at: "CONFLICT (C5): 'Once the Intraday Threshold reaches Target Profit Balance, it stops increasing.' (body) vs 'trailing stops once the Intraday Threshold reaches Profit Target Balance + $2,000' (FAQ)" }
  daily_loss_limit: { amount_usd: null, kind: none }
  profit_target_usd: "1500.00"
  consistency_rule: { text: "Not Applied", threshold_pct: null }
  max_position: { minis: "4", micros: "40", scaling: null }
  min_trading_days: 1   # "No minimum trading days required, may pass in one trading day"; homepage "Min. Days To Pass 1"
  trading_day_reset: { time: "18:00", tz_as_published: "ET" }
  copy_trading_allowed: unknown   # C1 conflict: same-person PA copying allowed (S-MAXPA) vs Legacy PA Compliance ban; other-trader copying forbidden
  automated_trading_allowed: "no"
  automation_rule_text: "No Automation or Algorithm Usage allowed: Rewards are intended to recognize human traders actively participating in the learning process, not to reward automated systems executing preprogrammed logic. Please see the user agreement for more details, and we encourage you to reach out if you have further questions."
  status: conflict   # numeric fields verified; see not_confirmed_fields
  not_confirmed_fields: [copy_trading_allowed, drawdown.stops_trailing_at]
  sources: 
    - { field: profit_target/drawdown/dll/max_contracts, url: "https://apextraderfunding.com/help-center/evaluation-accounts-ea/intraday-trailing-drawdown-evaluations/", accessed: "2026-10-23" }
    - { field: max_position.micros, url: "https://apextraderfunding.com/", accessed: "2026-10-23" }
    - { field: drawdown.type/stops_trailing_at, url: "https://apextraderfunding.com/help-center/intraday-trailing-drawdown-accounts/intraday-trailing-drawdown-explained/", accessed: "2026-10-23" }
    - { field: daily_loss_limit.kind, url: "https://apextraderfunding.com/help-center/additional-helpful-items/daily-loss-limit-explained/", accessed: "2026-10-23" }
    - { field: trading_day_reset, url: "https://apextraderfunding.com/help-center/getting-started/futures-trading-times/", accessed: "2026-10-23" }
    - { field: copy_trading_allowed (A), url: "https://apextraderfunding.com/help-center/performance-accounts-pa/how-many-paid-funded-accounts-am-i-allowed-to-have/", accessed: "2026-10-23" }
    - { field: copy_trading_allowed (B), url: "https://apextraderfunding.com/help-center/performance-accounts-pa/legacy-performance-account-pa-compliance/", accessed: "2026-10-23" }
    - { field: automation_rule_text, url: "https://apextraderfunding.com/help-center/getting-started/prohibited-activities/", accessed: "2026-10-23" }

- firm: apex-trader-funding
  plan: "Intraday Trailing Drawdown (Intraday Evaluation)"
  phase: evaluation
  size_label: "25K"
  size_usd: "25000.00"
  instruments: [MNQ, NQ]
  platforms: [Tradovate]
  drawdown: { amount_usd: "1000.00", type: trailing_intraday, stops_trailing_at: "Intraday Drawdown trails indefinitely with the peak account balance." }
  daily_loss_limit: { amount_usd: null, kind: none }
  profit_target_usd: "1500.00"
  consistency_rule: { text: "Not Applied", threshold_pct: null }
  max_position: { minis: "4", micros: "40", scaling: null }
  min_trading_days: 1   # "No minimum trading days required, may pass in one trading day"; homepage "Min. Days To Pass 1"
  trading_day_reset: { time: "18:00", tz_as_published: "ET" }
  copy_trading_allowed: unknown   # C1 conflict: same-person PA copying allowed (S-MAXPA) vs Legacy PA Compliance ban; other-trader copying forbidden
  automated_trading_allowed: "no"
  automation_rule_text: "No Automation or Algorithm Usage allowed: Rewards are intended to recognize human traders actively participating in the learning process, not to reward automated systems executing preprogrammed logic. Please see the user agreement for more details, and we encourage you to reach out if you have further questions."
  status: conflict   # numeric fields verified; see not_confirmed_fields
  not_confirmed_fields: [copy_trading_allowed]
  sources: 
    - { field: profit_target/drawdown/dll/max_contracts, url: "https://apextraderfunding.com/help-center/evaluation-accounts-ea/intraday-trailing-drawdown-evaluations/", accessed: "2026-10-23" }
    - { field: max_position.micros, url: "https://apextraderfunding.com/", accessed: "2026-10-23" }
    - { field: drawdown.type/stops_trailing_at, url: "https://apextraderfunding.com/help-center/intraday-trailing-drawdown-accounts/intraday-trailing-drawdown-explained/", accessed: "2026-10-23" }
    - { field: daily_loss_limit.kind, url: "https://apextraderfunding.com/help-center/additional-helpful-items/daily-loss-limit-explained/", accessed: "2026-10-23" }
    - { field: trading_day_reset, url: "https://apextraderfunding.com/help-center/getting-started/futures-trading-times/", accessed: "2026-10-23" }
    - { field: copy_trading_allowed (A), url: "https://apextraderfunding.com/help-center/performance-accounts-pa/how-many-paid-funded-accounts-am-i-allowed-to-have/", accessed: "2026-10-23" }
    - { field: copy_trading_allowed (B), url: "https://apextraderfunding.com/help-center/performance-accounts-pa/legacy-performance-account-pa-compliance/", accessed: "2026-10-23" }
    - { field: automation_rule_text, url: "https://apextraderfunding.com/help-center/getting-started/prohibited-activities/", accessed: "2026-10-23" }

- firm: apex-trader-funding
  plan: "Intraday Trailing Drawdown (Intraday Evaluation)"
  phase: evaluation
  size_label: "50K"
  size_usd: "50000.00"
  instruments: [MNQ, NQ]
  platforms: [Rithmic, WealthCharts]
  drawdown: { amount_usd: "2000.00", type: trailing_intraday, stops_trailing_at: "CONFLICT (C5): 'Once the Intraday Threshold reaches Target Profit Balance, it stops increasing.' (body) vs 'trailing stops once the Intraday Threshold reaches Profit Target Balance + $2,000' (FAQ); both give 50K stop level $53,000" }
  daily_loss_limit: { amount_usd: null, kind: none }
  profit_target_usd: "3000.00"
  consistency_rule: { text: "Not Applied", threshold_pct: null }
  max_position: { minis: "6", micros: "60", scaling: null }
  min_trading_days: 1   # "No minimum trading days required, may pass in one trading day"; homepage "Min. Days To Pass 1"
  trading_day_reset: { time: "18:00", tz_as_published: "ET" }
  copy_trading_allowed: unknown   # C1 conflict: same-person PA copying allowed (S-MAXPA) vs Legacy PA Compliance ban; other-trader copying forbidden
  automated_trading_allowed: "no"
  automation_rule_text: "No Automation or Algorithm Usage allowed: Rewards are intended to recognize human traders actively participating in the learning process, not to reward automated systems executing preprogrammed logic. Please see the user agreement for more details, and we encourage you to reach out if you have further questions."
  status: conflict   # numeric fields verified; see not_confirmed_fields
  not_confirmed_fields: [copy_trading_allowed, drawdown.stops_trailing_at]
  sources: 
    - { field: profit_target/drawdown/dll/max_contracts, url: "https://apextraderfunding.com/help-center/evaluation-accounts-ea/intraday-trailing-drawdown-evaluations/", accessed: "2026-10-23" }
    - { field: max_position.micros, url: "https://apextraderfunding.com/", accessed: "2026-10-23" }
    - { field: drawdown.type/stops_trailing_at, url: "https://apextraderfunding.com/help-center/intraday-trailing-drawdown-accounts/intraday-trailing-drawdown-explained/", accessed: "2026-10-23" }
    - { field: daily_loss_limit.kind, url: "https://apextraderfunding.com/help-center/additional-helpful-items/daily-loss-limit-explained/", accessed: "2026-10-23" }
    - { field: trading_day_reset, url: "https://apextraderfunding.com/help-center/getting-started/futures-trading-times/", accessed: "2026-10-23" }
    - { field: copy_trading_allowed (A), url: "https://apextraderfunding.com/help-center/performance-accounts-pa/how-many-paid-funded-accounts-am-i-allowed-to-have/", accessed: "2026-10-23" }
    - { field: copy_trading_allowed (B), url: "https://apextraderfunding.com/help-center/performance-accounts-pa/legacy-performance-account-pa-compliance/", accessed: "2026-10-23" }
    - { field: automation_rule_text, url: "https://apextraderfunding.com/help-center/getting-started/prohibited-activities/", accessed: "2026-10-23" }

- firm: apex-trader-funding
  plan: "Intraday Trailing Drawdown (Intraday Evaluation)"
  phase: evaluation
  size_label: "50K"
  size_usd: "50000.00"
  instruments: [MNQ, NQ]
  platforms: [Tradovate]
  drawdown: { amount_usd: "2000.00", type: trailing_intraday, stops_trailing_at: "Intraday Drawdown trails indefinitely with the peak account balance." }
  daily_loss_limit: { amount_usd: null, kind: none }
  profit_target_usd: "3000.00"
  consistency_rule: { text: "Not Applied", threshold_pct: null }
  max_position: { minis: "6", micros: "60", scaling: null }
  min_trading_days: 1   # "No minimum trading days required, may pass in one trading day"; homepage "Min. Days To Pass 1"
  trading_day_reset: { time: "18:00", tz_as_published: "ET" }
  copy_trading_allowed: unknown   # C1 conflict: same-person PA copying allowed (S-MAXPA) vs Legacy PA Compliance ban; other-trader copying forbidden
  automated_trading_allowed: "no"
  automation_rule_text: "No Automation or Algorithm Usage allowed: Rewards are intended to recognize human traders actively participating in the learning process, not to reward automated systems executing preprogrammed logic. Please see the user agreement for more details, and we encourage you to reach out if you have further questions."
  status: conflict   # numeric fields verified; see not_confirmed_fields
  not_confirmed_fields: [copy_trading_allowed]
  sources: 
    - { field: profit_target/drawdown/dll/max_contracts, url: "https://apextraderfunding.com/help-center/evaluation-accounts-ea/intraday-trailing-drawdown-evaluations/", accessed: "2026-10-23" }
    - { field: max_position.micros, url: "https://apextraderfunding.com/", accessed: "2026-10-23" }
    - { field: drawdown.type/stops_trailing_at, url: "https://apextraderfunding.com/help-center/intraday-trailing-drawdown-accounts/intraday-trailing-drawdown-explained/", accessed: "2026-10-23" }
    - { field: daily_loss_limit.kind, url: "https://apextraderfunding.com/help-center/additional-helpful-items/daily-loss-limit-explained/", accessed: "2026-10-23" }
    - { field: trading_day_reset, url: "https://apextraderfunding.com/help-center/getting-started/futures-trading-times/", accessed: "2026-10-23" }
    - { field: copy_trading_allowed (A), url: "https://apextraderfunding.com/help-center/performance-accounts-pa/how-many-paid-funded-accounts-am-i-allowed-to-have/", accessed: "2026-10-23" }
    - { field: copy_trading_allowed (B), url: "https://apextraderfunding.com/help-center/performance-accounts-pa/legacy-performance-account-pa-compliance/", accessed: "2026-10-23" }
    - { field: automation_rule_text, url: "https://apextraderfunding.com/help-center/getting-started/prohibited-activities/", accessed: "2026-10-23" }

- firm: apex-trader-funding
  plan: "Intraday Trailing Drawdown (Intraday Evaluation)"
  phase: evaluation
  size_label: "100K"
  size_usd: "100000.00"
  instruments: [MNQ, NQ]
  platforms: [Rithmic, WealthCharts]
  drawdown: { amount_usd: "3000.00", type: trailing_intraday, stops_trailing_at: "CONFLICT (C5): 'Once the Intraday Threshold reaches Target Profit Balance, it stops increasing.' (body) vs 'trailing stops once the Intraday Threshold reaches Profit Target Balance + $2,000' (FAQ)" }
  daily_loss_limit: { amount_usd: null, kind: none }
  profit_target_usd: "6000.00"
  consistency_rule: { text: "Not Applied", threshold_pct: null }
  max_position: { minis: "8", micros: "80", scaling: null }
  min_trading_days: 1   # "No minimum trading days required, may pass in one trading day"; homepage "Min. Days To Pass 1"
  trading_day_reset: { time: "18:00", tz_as_published: "ET" }
  copy_trading_allowed: unknown   # C1 conflict: same-person PA copying allowed (S-MAXPA) vs Legacy PA Compliance ban; other-trader copying forbidden
  automated_trading_allowed: "no"
  automation_rule_text: "No Automation or Algorithm Usage allowed: Rewards are intended to recognize human traders actively participating in the learning process, not to reward automated systems executing preprogrammed logic. Please see the user agreement for more details, and we encourage you to reach out if you have further questions."
  status: conflict   # numeric fields verified; see not_confirmed_fields
  not_confirmed_fields: [copy_trading_allowed, drawdown.stops_trailing_at]
  sources: 
    - { field: profit_target/drawdown/dll/max_contracts, url: "https://apextraderfunding.com/help-center/evaluation-accounts-ea/intraday-trailing-drawdown-evaluations/", accessed: "2026-10-23" }
    - { field: max_position.micros, url: "https://apextraderfunding.com/", accessed: "2026-10-23" }
    - { field: drawdown.type/stops_trailing_at, url: "https://apextraderfunding.com/help-center/intraday-trailing-drawdown-accounts/intraday-trailing-drawdown-explained/", accessed: "2026-10-23" }
    - { field: daily_loss_limit.kind, url: "https://apextraderfunding.com/help-center/additional-helpful-items/daily-loss-limit-explained/", accessed: "2026-10-23" }
    - { field: trading_day_reset, url: "https://apextraderfunding.com/help-center/getting-started/futures-trading-times/", accessed: "2026-10-23" }
    - { field: copy_trading_allowed (A), url: "https://apextraderfunding.com/help-center/performance-accounts-pa/how-many-paid-funded-accounts-am-i-allowed-to-have/", accessed: "2026-10-23" }
    - { field: copy_trading_allowed (B), url: "https://apextraderfunding.com/help-center/performance-accounts-pa/legacy-performance-account-pa-compliance/", accessed: "2026-10-23" }
    - { field: automation_rule_text, url: "https://apextraderfunding.com/help-center/getting-started/prohibited-activities/", accessed: "2026-10-23" }

- firm: apex-trader-funding
  plan: "Intraday Trailing Drawdown (Intraday Evaluation)"
  phase: evaluation
  size_label: "100K"
  size_usd: "100000.00"
  instruments: [MNQ, NQ]
  platforms: [Tradovate]
  drawdown: { amount_usd: "3000.00", type: trailing_intraday, stops_trailing_at: "Intraday Drawdown trails indefinitely with the peak account balance." }
  daily_loss_limit: { amount_usd: null, kind: none }
  profit_target_usd: "6000.00"
  consistency_rule: { text: "Not Applied", threshold_pct: null }
  max_position: { minis: "8", micros: "80", scaling: null }
  min_trading_days: 1   # "No minimum trading days required, may pass in one trading day"; homepage "Min. Days To Pass 1"
  trading_day_reset: { time: "18:00", tz_as_published: "ET" }
  copy_trading_allowed: unknown   # C1 conflict: same-person PA copying allowed (S-MAXPA) vs Legacy PA Compliance ban; other-trader copying forbidden
  automated_trading_allowed: "no"
  automation_rule_text: "No Automation or Algorithm Usage allowed: Rewards are intended to recognize human traders actively participating in the learning process, not to reward automated systems executing preprogrammed logic. Please see the user agreement for more details, and we encourage you to reach out if you have further questions."
  status: conflict   # numeric fields verified; see not_confirmed_fields
  not_confirmed_fields: [copy_trading_allowed]
  sources: 
    - { field: profit_target/drawdown/dll/max_contracts, url: "https://apextraderfunding.com/help-center/evaluation-accounts-ea/intraday-trailing-drawdown-evaluations/", accessed: "2026-10-23" }
    - { field: max_position.micros, url: "https://apextraderfunding.com/", accessed: "2026-10-23" }
    - { field: drawdown.type/stops_trailing_at, url: "https://apextraderfunding.com/help-center/intraday-trailing-drawdown-accounts/intraday-trailing-drawdown-explained/", accessed: "2026-10-23" }
    - { field: daily_loss_limit.kind, url: "https://apextraderfunding.com/help-center/additional-helpful-items/daily-loss-limit-explained/", accessed: "2026-10-23" }
    - { field: trading_day_reset, url: "https://apextraderfunding.com/help-center/getting-started/futures-trading-times/", accessed: "2026-10-23" }
    - { field: copy_trading_allowed (A), url: "https://apextraderfunding.com/help-center/performance-accounts-pa/how-many-paid-funded-accounts-am-i-allowed-to-have/", accessed: "2026-10-23" }
    - { field: copy_trading_allowed (B), url: "https://apextraderfunding.com/help-center/performance-accounts-pa/legacy-performance-account-pa-compliance/", accessed: "2026-10-23" }
    - { field: automation_rule_text, url: "https://apextraderfunding.com/help-center/getting-started/prohibited-activities/", accessed: "2026-10-23" }

- firm: apex-trader-funding
  plan: "Intraday Trailing Drawdown (Intraday Evaluation)"
  phase: evaluation
  size_label: "150K"
  size_usd: "150000.00"
  instruments: [MNQ, NQ]
  platforms: [Rithmic, WealthCharts]
  drawdown: { amount_usd: "4000.00", type: trailing_intraday, stops_trailing_at: "CONFLICT (C5): 'Once the Intraday Threshold reaches Target Profit Balance, it stops increasing.' (body) vs 'trailing stops once the Intraday Threshold reaches Profit Target Balance + $2,000' (FAQ)" }
  daily_loss_limit: { amount_usd: null, kind: none }
  profit_target_usd: "9000.00"
  consistency_rule: { text: "Not Applied", threshold_pct: null }
  max_position: { minis: "12", micros: "120", scaling: null }
  min_trading_days: 1   # "No minimum trading days required, may pass in one trading day"; homepage "Min. Days To Pass 1"
  trading_day_reset: { time: "18:00", tz_as_published: "ET" }
  copy_trading_allowed: unknown   # C1 conflict: same-person PA copying allowed (S-MAXPA) vs Legacy PA Compliance ban; other-trader copying forbidden
  automated_trading_allowed: "no"
  automation_rule_text: "No Automation or Algorithm Usage allowed: Rewards are intended to recognize human traders actively participating in the learning process, not to reward automated systems executing preprogrammed logic. Please see the user agreement for more details, and we encourage you to reach out if you have further questions."
  status: conflict   # numeric fields verified; see not_confirmed_fields
  not_confirmed_fields: [copy_trading_allowed, drawdown.stops_trailing_at]
  sources: 
    - { field: profit_target/drawdown/dll/max_contracts, url: "https://apextraderfunding.com/help-center/evaluation-accounts-ea/intraday-trailing-drawdown-evaluations/", accessed: "2026-10-23" }
    - { field: max_position.micros, url: "https://apextraderfunding.com/", accessed: "2026-10-23" }
    - { field: drawdown.type/stops_trailing_at, url: "https://apextraderfunding.com/help-center/intraday-trailing-drawdown-accounts/intraday-trailing-drawdown-explained/", accessed: "2026-10-23" }
    - { field: daily_loss_limit.kind, url: "https://apextraderfunding.com/help-center/additional-helpful-items/daily-loss-limit-explained/", accessed: "2026-10-23" }
    - { field: trading_day_reset, url: "https://apextraderfunding.com/help-center/getting-started/futures-trading-times/", accessed: "2026-10-23" }
    - { field: copy_trading_allowed (A), url: "https://apextraderfunding.com/help-center/performance-accounts-pa/how-many-paid-funded-accounts-am-i-allowed-to-have/", accessed: "2026-10-23" }
    - { field: copy_trading_allowed (B), url: "https://apextraderfunding.com/help-center/performance-accounts-pa/legacy-performance-account-pa-compliance/", accessed: "2026-10-23" }
    - { field: automation_rule_text, url: "https://apextraderfunding.com/help-center/getting-started/prohibited-activities/", accessed: "2026-10-23" }

- firm: apex-trader-funding
  plan: "Intraday Trailing Drawdown (Intraday Evaluation)"
  phase: evaluation
  size_label: "150K"
  size_usd: "150000.00"
  instruments: [MNQ, NQ]
  platforms: [Tradovate]
  drawdown: { amount_usd: "4000.00", type: trailing_intraday, stops_trailing_at: "Intraday Drawdown trails indefinitely with the peak account balance." }
  daily_loss_limit: { amount_usd: null, kind: none }
  profit_target_usd: "9000.00"
  consistency_rule: { text: "Not Applied", threshold_pct: null }
  max_position: { minis: "12", micros: "120", scaling: null }
  min_trading_days: 1   # "No minimum trading days required, may pass in one trading day"; homepage "Min. Days To Pass 1"
  trading_day_reset: { time: "18:00", tz_as_published: "ET" }
  copy_trading_allowed: unknown   # C1 conflict: same-person PA copying allowed (S-MAXPA) vs Legacy PA Compliance ban; other-trader copying forbidden
  automated_trading_allowed: "no"
  automation_rule_text: "No Automation or Algorithm Usage allowed: Rewards are intended to recognize human traders actively participating in the learning process, not to reward automated systems executing preprogrammed logic. Please see the user agreement for more details, and we encourage you to reach out if you have further questions."
  status: conflict   # numeric fields verified; see not_confirmed_fields
  not_confirmed_fields: [copy_trading_allowed]
  sources: 
    - { field: profit_target/drawdown/dll/max_contracts, url: "https://apextraderfunding.com/help-center/evaluation-accounts-ea/intraday-trailing-drawdown-evaluations/", accessed: "2026-10-23" }
    - { field: max_position.micros, url: "https://apextraderfunding.com/", accessed: "2026-10-23" }
    - { field: drawdown.type/stops_trailing_at, url: "https://apextraderfunding.com/help-center/intraday-trailing-drawdown-accounts/intraday-trailing-drawdown-explained/", accessed: "2026-10-23" }
    - { field: daily_loss_limit.kind, url: "https://apextraderfunding.com/help-center/additional-helpful-items/daily-loss-limit-explained/", accessed: "2026-10-23" }
    - { field: trading_day_reset, url: "https://apextraderfunding.com/help-center/getting-started/futures-trading-times/", accessed: "2026-10-23" }
    - { field: copy_trading_allowed (A), url: "https://apextraderfunding.com/help-center/performance-accounts-pa/how-many-paid-funded-accounts-am-i-allowed-to-have/", accessed: "2026-10-23" }
    - { field: copy_trading_allowed (B), url: "https://apextraderfunding.com/help-center/performance-accounts-pa/legacy-performance-account-pa-compliance/", accessed: "2026-10-23" }
    - { field: automation_rule_text, url: "https://apextraderfunding.com/help-center/getting-started/prohibited-activities/", accessed: "2026-10-23" }

# ---------- EOD Trailing Drawdown (EOD Performance Account) (no platform-specific rule published) ----------

- firm: apex-trader-funding
  plan: "EOD Trailing Drawdown (EOD Performance Account)"
  phase: funded
  size_label: "25K"
  size_usd: "25000.00"
  instruments: [MNQ, NQ]
  platforms: [Tradovate, Rithmic, WealthCharts]
  drawdown: { amount_usd: "1000.00", type: trailing_eod, stops_trailing_at: "Once the EOD Threshold reaches Starting Balance + $100, it stops increasing." }
  daily_loss_limit: { amount_usd: "500.00", kind: soft_pause, tiered: true }   # Level 1 DLL; tiers below
  profit_target_usd: null
  consistency_rule: { text: "Your largest profitable trading day must represent less than 50% of your net profit since your last payout request, or since account inception if no payout has been made. (payout eligibility only; account not failed)", threshold_pct: "50" }
  max_position:
    minis: "2"   # top-tier max as published
    micros: null   # firm publishes ratio only: "ten (10) micro contracts equal one (1) standard contract"
    scaling:
      - { level: 1, profit_range: "$0 \u2013 $999", max_contracts: "1", dll_usd: "500.00" }
      - { level: 2, profit_range: "$1,000 \u2013 $1,999", max_contracts: "2", dll_usd: "500.00" }
      - { level: 3, profit_range: "$2,000 & Up", max_contracts: "2", dll_usd: "1250.00" }
  min_trading_days: 5   # per payout request only; each qualifying day >= $100 net profit
  trading_day_reset: { time: "18:00", tz_as_published: "ET" }
  copy_trading_allowed: unknown   # C1 conflict: same-person PA copying allowed (S-MAXPA) vs Legacy PA Compliance ban; other-trader copying forbidden
  automated_trading_allowed: "no"
  automation_rule_text: "No Automation or Algorithm Usage allowed: Rewards are intended to recognize human traders actively participating in the learning process, not to reward automated systems executing preprogrammed logic. Please see the user agreement for more details, and we encourage you to reach out if you have further questions."
  status: conflict   # numeric fields verified; see not_confirmed_fields
  not_confirmed_fields: [copy_trading_allowed, "consistency_rule.threshold_pct (boundary: <50% vs example 50% met)"]
  sources: 
    - { field: drawdown/max_contracts, url: "https://apextraderfunding.com/help-center/eod-trailing-drawdown-accounts/eod-performance-accounts-pa/", accessed: "2026-10-23" }
    - { field: drawdown.stops_trailing_at, url: "https://apextraderfunding.com/help-center/eod-trailing-drawdown-accounts/eod-drawdown-explained/", accessed: "2026-10-23" }
    - { field: dll/scaling (A), url: "https://apextraderfunding.com/help-center/additional-helpful-items/daily-loss-limit-explained/", accessed: "2026-10-23" }
    - { field: scaling (B), url: "https://apextraderfunding.com/help-center/additional-helpful-items/scaling-levels-pa-explained/", accessed: "2026-10-23" }
    - { field: consistency_rule, url: "https://apextraderfunding.com/help-center/additional-helpful-items/50-consistency-requirement/", accessed: "2026-10-23" }
    - { field: min_trading_days, url: "https://apextraderfunding.com/help-center/eod-trailing-drawdown-accounts/eod-payouts/", accessed: "2026-10-23" }
    - { field: trading_day_reset, url: "https://apextraderfunding.com/help-center/getting-started/futures-trading-times/", accessed: "2026-10-23" }
    - { field: copy_trading_allowed (A), url: "https://apextraderfunding.com/help-center/performance-accounts-pa/how-many-paid-funded-accounts-am-i-allowed-to-have/", accessed: "2026-10-23" }
    - { field: copy_trading_allowed (B), url: "https://apextraderfunding.com/help-center/performance-accounts-pa/legacy-performance-account-pa-compliance/", accessed: "2026-10-23" }
    - { field: automation_rule_text, url: "https://apextraderfunding.com/help-center/getting-started/prohibited-activities/", accessed: "2026-10-23" }

- firm: apex-trader-funding
  plan: "EOD Trailing Drawdown (EOD Performance Account)"
  phase: funded
  size_label: "50K"
  size_usd: "50000.00"
  instruments: [MNQ, NQ]
  platforms: [Tradovate, Rithmic, WealthCharts]
  drawdown: { amount_usd: "2000.00", type: trailing_eod, stops_trailing_at: "Once the EOD Threshold reaches Starting Balance + $100, it stops increasing. (published 50K example: $50,100)" }
  daily_loss_limit: { amount_usd: "1000.00", kind: soft_pause, tiered: true }   # Level 1 DLL; tiers below
  profit_target_usd: null
  consistency_rule: { text: "Your largest profitable trading day must represent less than 50% of your net profit since your last payout request, or since account inception if no payout has been made. (payout eligibility only; account not failed)", threshold_pct: "50" }
  max_position:
    minis: "4"   # top-tier max as published
    micros: null   # firm publishes ratio only: "ten (10) micro contracts equal one (1) standard contract"
    scaling:
      - { level: 1, profit_range: "$0 \u2013 $1,499", max_contracts: "2", dll_usd: "1000.00" }
      - { level: 2, profit_range: "$1,500 \u2013 $2,999", max_contracts: "3", dll_usd: "1000.00" }
      - { level: 3, profit_range: "$3,000 \u2013 $5,999", max_contracts: "4", dll_usd: "2000.00" }
      - { level: 4, profit_range: "CONFLICT: $6,000 & Up (DLL page) vs $5,999 & Up (Scaling page)", max_contracts: "4", dll_usd: "3000.00" }
  min_trading_days: 5   # per payout request only; each qualifying day >= $250 net profit
  trading_day_reset: { time: "18:00", tz_as_published: "ET" }
  copy_trading_allowed: unknown   # C1 conflict: same-person PA copying allowed (S-MAXPA) vs Legacy PA Compliance ban; other-trader copying forbidden
  automated_trading_allowed: "no"
  automation_rule_text: "No Automation or Algorithm Usage allowed: Rewards are intended to recognize human traders actively participating in the learning process, not to reward automated systems executing preprogrammed logic. Please see the user agreement for more details, and we encourage you to reach out if you have further questions."
  status: conflict   # numeric fields verified; see not_confirmed_fields
  not_confirmed_fields: [copy_trading_allowed, "consistency_rule.threshold_pct (boundary: <50% vs example 50% met)", "max_position.scaling[level 4].profit_range"]
  sources: 
    - { field: drawdown/max_contracts, url: "https://apextraderfunding.com/help-center/eod-trailing-drawdown-accounts/eod-performance-accounts-pa/", accessed: "2026-10-23" }
    - { field: drawdown.stops_trailing_at, url: "https://apextraderfunding.com/help-center/eod-trailing-drawdown-accounts/eod-drawdown-explained/", accessed: "2026-10-23" }
    - { field: dll/scaling (A), url: "https://apextraderfunding.com/help-center/additional-helpful-items/daily-loss-limit-explained/", accessed: "2026-10-23" }
    - { field: scaling (B), url: "https://apextraderfunding.com/help-center/additional-helpful-items/scaling-levels-pa-explained/", accessed: "2026-10-23" }
    - { field: consistency_rule, url: "https://apextraderfunding.com/help-center/additional-helpful-items/50-consistency-requirement/", accessed: "2026-10-23" }
    - { field: min_trading_days, url: "https://apextraderfunding.com/help-center/eod-trailing-drawdown-accounts/eod-payouts/", accessed: "2026-10-23" }
    - { field: trading_day_reset, url: "https://apextraderfunding.com/help-center/getting-started/futures-trading-times/", accessed: "2026-10-23" }
    - { field: copy_trading_allowed (A), url: "https://apextraderfunding.com/help-center/performance-accounts-pa/how-many-paid-funded-accounts-am-i-allowed-to-have/", accessed: "2026-10-23" }
    - { field: copy_trading_allowed (B), url: "https://apextraderfunding.com/help-center/performance-accounts-pa/legacy-performance-account-pa-compliance/", accessed: "2026-10-23" }
    - { field: automation_rule_text, url: "https://apextraderfunding.com/help-center/getting-started/prohibited-activities/", accessed: "2026-10-23" }

- firm: apex-trader-funding
  plan: "EOD Trailing Drawdown (EOD Performance Account)"
  phase: funded
  size_label: "100K"
  size_usd: "100000.00"
  instruments: [MNQ, NQ]
  platforms: [Tradovate, Rithmic, WealthCharts]
  drawdown: { amount_usd: "3000.00", type: trailing_eod, stops_trailing_at: "Once the EOD Threshold reaches Starting Balance + $100, it stops increasing." }
  daily_loss_limit: { amount_usd: "1750.00", kind: soft_pause, tiered: true }   # Level 1 DLL; tiers below
  profit_target_usd: null
  consistency_rule: { text: "Your largest profitable trading day must represent less than 50% of your net profit since your last payout request, or since account inception if no payout has been made. (payout eligibility only; account not failed)", threshold_pct: "50" }
  max_position:
    minis: "6"   # top-tier max as published
    micros: null   # firm publishes ratio only: "ten (10) micro contracts equal one (1) standard contract"
    scaling:
      - { level: 1, profit_range: "$0 \u2013 $1,999", max_contracts: "3", dll_usd: "1750.00" }
      - { level: 2, profit_range: "$2,000 \u2013 $2,999", max_contracts: "4", dll_usd: "1750.00" }
      - { level: 3, profit_range: "$3,000 \u2013 $4,999", max_contracts: "5", dll_usd: "1750.00" }
      - { level: 4, profit_range: "$5,000 \u2013 $9,999", max_contracts: "6", dll_usd: "2500.00" }
      - { level: 5, profit_range: "$10,000 & Up", max_contracts: "6", dll_usd: "3500.00" }
  min_trading_days: 5   # per payout request only; each qualifying day >= $300 net profit
  trading_day_reset: { time: "18:00", tz_as_published: "ET" }
  copy_trading_allowed: unknown   # C1 conflict: same-person PA copying allowed (S-MAXPA) vs Legacy PA Compliance ban; other-trader copying forbidden
  automated_trading_allowed: "no"
  automation_rule_text: "No Automation or Algorithm Usage allowed: Rewards are intended to recognize human traders actively participating in the learning process, not to reward automated systems executing preprogrammed logic. Please see the user agreement for more details, and we encourage you to reach out if you have further questions."
  status: conflict   # numeric fields verified; see not_confirmed_fields
  not_confirmed_fields: [copy_trading_allowed, "consistency_rule.threshold_pct (boundary: <50% vs example 50% met)"]
  sources: 
    - { field: drawdown/max_contracts, url: "https://apextraderfunding.com/help-center/eod-trailing-drawdown-accounts/eod-performance-accounts-pa/", accessed: "2026-10-23" }
    - { field: drawdown.stops_trailing_at, url: "https://apextraderfunding.com/help-center/eod-trailing-drawdown-accounts/eod-drawdown-explained/", accessed: "2026-10-23" }
    - { field: dll/scaling (A), url: "https://apextraderfunding.com/help-center/additional-helpful-items/daily-loss-limit-explained/", accessed: "2026-10-23" }
    - { field: scaling (B), url: "https://apextraderfunding.com/help-center/additional-helpful-items/scaling-levels-pa-explained/", accessed: "2026-10-23" }
    - { field: consistency_rule, url: "https://apextraderfunding.com/help-center/additional-helpful-items/50-consistency-requirement/", accessed: "2026-10-23" }
    - { field: min_trading_days, url: "https://apextraderfunding.com/help-center/eod-trailing-drawdown-accounts/eod-payouts/", accessed: "2026-10-23" }
    - { field: trading_day_reset, url: "https://apextraderfunding.com/help-center/getting-started/futures-trading-times/", accessed: "2026-10-23" }
    - { field: copy_trading_allowed (A), url: "https://apextraderfunding.com/help-center/performance-accounts-pa/how-many-paid-funded-accounts-am-i-allowed-to-have/", accessed: "2026-10-23" }
    - { field: copy_trading_allowed (B), url: "https://apextraderfunding.com/help-center/performance-accounts-pa/legacy-performance-account-pa-compliance/", accessed: "2026-10-23" }
    - { field: automation_rule_text, url: "https://apextraderfunding.com/help-center/getting-started/prohibited-activities/", accessed: "2026-10-23" }

- firm: apex-trader-funding
  plan: "EOD Trailing Drawdown (EOD Performance Account)"
  phase: funded
  size_label: "150K"
  size_usd: "150000.00"
  instruments: [MNQ, NQ]
  platforms: [Tradovate, Rithmic, WealthCharts]
  drawdown: { amount_usd: "4000.00", type: trailing_eod, stops_trailing_at: "Once the EOD Threshold reaches Starting Balance + $100, it stops increasing." }
  daily_loss_limit: { amount_usd: "2500.00", kind: soft_pause, tiered: true }   # Level 1 DLL; tiers below
  profit_target_usd: null
  consistency_rule: { text: "Your largest profitable trading day must represent less than 50% of your net profit since your last payout request, or since account inception if no payout has been made. (payout eligibility only; account not failed)", threshold_pct: "50" }
  max_position:
    minis: "10"   # top-tier max as published
    micros: null   # firm publishes ratio only: "ten (10) micro contracts equal one (1) standard contract"
    scaling:
      - { level: 1, profit_range: "$0 \u2013 $1,999", max_contracts: "4", dll_usd: "2500.00" }
      - { level: 2, profit_range: "$2,000 \u2013 $2,999", max_contracts: "5", dll_usd: "2500.00" }
      - { level: 3, profit_range: "$3,000 \u2013 $4,999", max_contracts: "7", dll_usd: "2500.00" }
      - { level: 4, profit_range: "$5,000 \u2013 $9,999", max_contracts: "10", dll_usd: "3000.00" }
      - { level: 5, profit_range: "$10,000 & Up", max_contracts: "10", dll_usd: "4000.00" }
  min_trading_days: 5   # per payout request only; each qualifying day >= $350 net profit
  trading_day_reset: { time: "18:00", tz_as_published: "ET" }
  copy_trading_allowed: unknown   # C1 conflict: same-person PA copying allowed (S-MAXPA) vs Legacy PA Compliance ban; other-trader copying forbidden
  automated_trading_allowed: "no"
  automation_rule_text: "No Automation or Algorithm Usage allowed: Rewards are intended to recognize human traders actively participating in the learning process, not to reward automated systems executing preprogrammed logic. Please see the user agreement for more details, and we encourage you to reach out if you have further questions."
  status: conflict   # numeric fields verified; see not_confirmed_fields
  not_confirmed_fields: [copy_trading_allowed, "consistency_rule.threshold_pct (boundary: <50% vs example 50% met)"]
  sources: 
    - { field: drawdown/max_contracts, url: "https://apextraderfunding.com/help-center/eod-trailing-drawdown-accounts/eod-performance-accounts-pa/", accessed: "2026-10-23" }
    - { field: drawdown.stops_trailing_at, url: "https://apextraderfunding.com/help-center/eod-trailing-drawdown-accounts/eod-drawdown-explained/", accessed: "2026-10-23" }
    - { field: dll/scaling (A), url: "https://apextraderfunding.com/help-center/additional-helpful-items/daily-loss-limit-explained/", accessed: "2026-10-23" }
    - { field: scaling (B), url: "https://apextraderfunding.com/help-center/additional-helpful-items/scaling-levels-pa-explained/", accessed: "2026-10-23" }
    - { field: consistency_rule, url: "https://apextraderfunding.com/help-center/additional-helpful-items/50-consistency-requirement/", accessed: "2026-10-23" }
    - { field: min_trading_days, url: "https://apextraderfunding.com/help-center/eod-trailing-drawdown-accounts/eod-payouts/", accessed: "2026-10-23" }
    - { field: trading_day_reset, url: "https://apextraderfunding.com/help-center/getting-started/futures-trading-times/", accessed: "2026-10-23" }
    - { field: copy_trading_allowed (A), url: "https://apextraderfunding.com/help-center/performance-accounts-pa/how-many-paid-funded-accounts-am-i-allowed-to-have/", accessed: "2026-10-23" }
    - { field: copy_trading_allowed (B), url: "https://apextraderfunding.com/help-center/performance-accounts-pa/legacy-performance-account-pa-compliance/", accessed: "2026-10-23" }
    - { field: automation_rule_text, url: "https://apextraderfunding.com/help-center/getting-started/prohibited-activities/", accessed: "2026-10-23" }

# ---------- Intraday Trailing Drawdown (Intraday Performance Account) (no platform-specific rule published) ----------

- firm: apex-trader-funding
  plan: "Intraday Trailing Drawdown (Intraday Performance Account)"
  phase: funded
  size_label: "25K"
  size_usd: "25000.00"
  instruments: [MNQ, NQ]
  platforms: [Tradovate, Rithmic, WealthCharts]
  drawdown: { amount_usd: "1000.00", type: trailing_intraday, stops_trailing_at: "CONFLICT (C2): 'In Performance Accounts, trailing stops once the Intraday Threshold reaches Starting Balance + $100.' (Intraday Drawdown Explained) vs 'Trailing stops once Max Trailing Drawdown Amount + $100 is reached' (Intraday PA page)" }
  daily_loss_limit: { amount_usd: "500.00", kind: soft_pause, tiered: true }   # Level 1 DLL; tiers below
  profit_target_usd: null
  consistency_rule: { text: "Your largest profitable trading day must represent less than 50% of your net profit since your last payout request, or since account inception if no payout has been made. (payout eligibility only; account not failed)", threshold_pct: "50" }
  max_position:
    minis: "2"   # top-tier max as published
    micros: null   # firm publishes ratio only: "ten (10) micro contracts equal one (1) standard contract"
    scaling:
      - { level: 1, profit_range: "$0 \u2013 $999", max_contracts: "1", dll_usd: "500.00" }
      - { level: 2, profit_range: "$1,000 \u2013 $1,999", max_contracts: "2", dll_usd: "500.00" }
      - { level: 3, profit_range: "$2,000 & Up", max_contracts: "2", dll_usd: "1250.00" }
  min_trading_days: 5   # per payout request only; each qualifying day >= $100 net profit
  trading_day_reset: { time: "18:00", tz_as_published: "ET" }
  copy_trading_allowed: unknown   # C1 conflict: same-person PA copying allowed (S-MAXPA) vs Legacy PA Compliance ban; other-trader copying forbidden
  automated_trading_allowed: "no"
  automation_rule_text: "No Automation or Algorithm Usage allowed: Rewards are intended to recognize human traders actively participating in the learning process, not to reward automated systems executing preprogrammed logic. Please see the user agreement for more details, and we encourage you to reach out if you have further questions."
  status: conflict   # numeric fields verified; see not_confirmed_fields
  not_confirmed_fields: [copy_trading_allowed, "consistency_rule.threshold_pct (boundary: <50% vs example 50% met)", drawdown.stops_trailing_at]
  sources: 
    - { field: drawdown/max_contracts, url: "https://apextraderfunding.com/help-center/intraday-trailing-drawdown-accounts/intraday-trailing-drawdown-performance-accounts-pa/", accessed: "2026-10-23" }
    - { field: drawdown.stops_trailing_at, url: "https://apextraderfunding.com/help-center/intraday-trailing-drawdown-accounts/intraday-trailing-drawdown-explained/", accessed: "2026-10-23" }
    - { field: dll/scaling (A), url: "https://apextraderfunding.com/help-center/additional-helpful-items/daily-loss-limit-explained/", accessed: "2026-10-23" }
    - { field: scaling (B), url: "https://apextraderfunding.com/help-center/additional-helpful-items/scaling-levels-pa-explained/", accessed: "2026-10-23" }
    - { field: consistency_rule, url: "https://apextraderfunding.com/help-center/additional-helpful-items/50-consistency-requirement/", accessed: "2026-10-23" }
    - { field: min_trading_days, url: "https://apextraderfunding.com/help-center/intraday-trailing-drawdown-accounts/intraday-trailing-drawdown-payouts/", accessed: "2026-10-23" }
    - { field: trading_day_reset, url: "https://apextraderfunding.com/help-center/getting-started/futures-trading-times/", accessed: "2026-10-23" }
    - { field: copy_trading_allowed (A), url: "https://apextraderfunding.com/help-center/performance-accounts-pa/how-many-paid-funded-accounts-am-i-allowed-to-have/", accessed: "2026-10-23" }
    - { field: copy_trading_allowed (B), url: "https://apextraderfunding.com/help-center/performance-accounts-pa/legacy-performance-account-pa-compliance/", accessed: "2026-10-23" }
    - { field: automation_rule_text, url: "https://apextraderfunding.com/help-center/getting-started/prohibited-activities/", accessed: "2026-10-23" }

- firm: apex-trader-funding
  plan: "Intraday Trailing Drawdown (Intraday Performance Account)"
  phase: funded
  size_label: "50K"
  size_usd: "50000.00"
  instruments: [MNQ, NQ]
  platforms: [Tradovate, Rithmic, WealthCharts]
  drawdown: { amount_usd: "2000.00", type: trailing_intraday, stops_trailing_at: "CONFLICT (C2): 'In Performance Accounts, trailing stops once the Intraday Threshold reaches Starting Balance + $100.' (Intraday Drawdown Explained) vs 'Trailing stops once Max Trailing Drawdown Amount + $100 is reached' (Intraday PA page)" }
  daily_loss_limit: { amount_usd: "1000.00", kind: soft_pause, tiered: true }   # Level 1 DLL; tiers below
  profit_target_usd: null
  consistency_rule: { text: "Your largest profitable trading day must represent less than 50% of your net profit since your last payout request, or since account inception if no payout has been made. (payout eligibility only; account not failed)", threshold_pct: "50" }
  max_position:
    minis: "4"   # top-tier max as published
    micros: null   # firm publishes ratio only: "ten (10) micro contracts equal one (1) standard contract"
    scaling:
      - { level: 1, profit_range: "$0 \u2013 $1,499", max_contracts: "2", dll_usd: "1000.00" }
      - { level: 2, profit_range: "$1,500 \u2013 $2,999", max_contracts: "3", dll_usd: "1000.00" }
      - { level: 3, profit_range: "$3,000 \u2013 $5,999", max_contracts: "4", dll_usd: "2000.00" }
      - { level: 4, profit_range: "CONFLICT: $6,000 & Up (DLL page) vs $5,999 & Up (Scaling page)", max_contracts: "4", dll_usd: "3000.00" }
  min_trading_days: 5   # per payout request only; each qualifying day >= $200 net profit
  trading_day_reset: { time: "18:00", tz_as_published: "ET" }
  copy_trading_allowed: unknown   # C1 conflict: same-person PA copying allowed (S-MAXPA) vs Legacy PA Compliance ban; other-trader copying forbidden
  automated_trading_allowed: "no"
  automation_rule_text: "No Automation or Algorithm Usage allowed: Rewards are intended to recognize human traders actively participating in the learning process, not to reward automated systems executing preprogrammed logic. Please see the user agreement for more details, and we encourage you to reach out if you have further questions."
  status: conflict   # numeric fields verified; see not_confirmed_fields
  not_confirmed_fields: [copy_trading_allowed, "consistency_rule.threshold_pct (boundary: <50% vs example 50% met)", drawdown.stops_trailing_at, "max_position.scaling[level 4].profit_range"]
  sources: 
    - { field: drawdown/max_contracts, url: "https://apextraderfunding.com/help-center/intraday-trailing-drawdown-accounts/intraday-trailing-drawdown-performance-accounts-pa/", accessed: "2026-10-23" }
    - { field: drawdown.stops_trailing_at, url: "https://apextraderfunding.com/help-center/intraday-trailing-drawdown-accounts/intraday-trailing-drawdown-explained/", accessed: "2026-10-23" }
    - { field: dll/scaling (A), url: "https://apextraderfunding.com/help-center/additional-helpful-items/daily-loss-limit-explained/", accessed: "2026-10-23" }
    - { field: scaling (B), url: "https://apextraderfunding.com/help-center/additional-helpful-items/scaling-levels-pa-explained/", accessed: "2026-10-23" }
    - { field: consistency_rule, url: "https://apextraderfunding.com/help-center/additional-helpful-items/50-consistency-requirement/", accessed: "2026-10-23" }
    - { field: min_trading_days, url: "https://apextraderfunding.com/help-center/intraday-trailing-drawdown-accounts/intraday-trailing-drawdown-payouts/", accessed: "2026-10-23" }
    - { field: trading_day_reset, url: "https://apextraderfunding.com/help-center/getting-started/futures-trading-times/", accessed: "2026-10-23" }
    - { field: copy_trading_allowed (A), url: "https://apextraderfunding.com/help-center/performance-accounts-pa/how-many-paid-funded-accounts-am-i-allowed-to-have/", accessed: "2026-10-23" }
    - { field: copy_trading_allowed (B), url: "https://apextraderfunding.com/help-center/performance-accounts-pa/legacy-performance-account-pa-compliance/", accessed: "2026-10-23" }
    - { field: automation_rule_text, url: "https://apextraderfunding.com/help-center/getting-started/prohibited-activities/", accessed: "2026-10-23" }

- firm: apex-trader-funding
  plan: "Intraday Trailing Drawdown (Intraday Performance Account)"
  phase: funded
  size_label: "100K"
  size_usd: "100000.00"
  instruments: [MNQ, NQ]
  platforms: [Tradovate, Rithmic, WealthCharts]
  drawdown: { amount_usd: "3000.00", type: trailing_intraday, stops_trailing_at: "CONFLICT (C2): 'In Performance Accounts, trailing stops once the Intraday Threshold reaches Starting Balance + $100.' (Intraday Drawdown Explained) vs 'Trailing stops once Max Trailing Drawdown Amount + $100 is reached' (Intraday PA page)" }
  daily_loss_limit: { amount_usd: "1750.00", kind: soft_pause, tiered: true }   # Level 1 DLL; tiers below
  profit_target_usd: null
  consistency_rule: { text: "Your largest profitable trading day must represent less than 50% of your net profit since your last payout request, or since account inception if no payout has been made. (payout eligibility only; account not failed)", threshold_pct: "50" }
  max_position:
    minis: "6"   # top-tier max as published
    micros: null   # firm publishes ratio only: "ten (10) micro contracts equal one (1) standard contract"
    scaling:
      - { level: 1, profit_range: "$0 \u2013 $1,999", max_contracts: "3", dll_usd: "1750.00" }
      - { level: 2, profit_range: "$2,000 \u2013 $2,999", max_contracts: "4", dll_usd: "1750.00" }
      - { level: 3, profit_range: "$3,000 \u2013 $4,999", max_contracts: "5", dll_usd: "1750.00" }
      - { level: 4, profit_range: "$5,000 \u2013 $9,999", max_contracts: "6", dll_usd: "2500.00" }
      - { level: 5, profit_range: "$10,000 & Up", max_contracts: "6", dll_usd: "3500.00" }
  min_trading_days: 5   # per payout request only; each qualifying day >= $250 net profit
  trading_day_reset: { time: "18:00", tz_as_published: "ET" }
  copy_trading_allowed: unknown   # C1 conflict: same-person PA copying allowed (S-MAXPA) vs Legacy PA Compliance ban; other-trader copying forbidden
  automated_trading_allowed: "no"
  automation_rule_text: "No Automation or Algorithm Usage allowed: Rewards are intended to recognize human traders actively participating in the learning process, not to reward automated systems executing preprogrammed logic. Please see the user agreement for more details, and we encourage you to reach out if you have further questions."
  status: conflict   # numeric fields verified; see not_confirmed_fields
  not_confirmed_fields: [copy_trading_allowed, "consistency_rule.threshold_pct (boundary: <50% vs example 50% met)", drawdown.stops_trailing_at]
  sources: 
    - { field: drawdown/max_contracts, url: "https://apextraderfunding.com/help-center/intraday-trailing-drawdown-accounts/intraday-trailing-drawdown-performance-accounts-pa/", accessed: "2026-10-23" }
    - { field: drawdown.stops_trailing_at, url: "https://apextraderfunding.com/help-center/intraday-trailing-drawdown-accounts/intraday-trailing-drawdown-explained/", accessed: "2026-10-23" }
    - { field: dll/scaling (A), url: "https://apextraderfunding.com/help-center/additional-helpful-items/daily-loss-limit-explained/", accessed: "2026-10-23" }
    - { field: scaling (B), url: "https://apextraderfunding.com/help-center/additional-helpful-items/scaling-levels-pa-explained/", accessed: "2026-10-23" }
    - { field: consistency_rule, url: "https://apextraderfunding.com/help-center/additional-helpful-items/50-consistency-requirement/", accessed: "2026-10-23" }
    - { field: min_trading_days, url: "https://apextraderfunding.com/help-center/intraday-trailing-drawdown-accounts/intraday-trailing-drawdown-payouts/", accessed: "2026-10-23" }
    - { field: trading_day_reset, url: "https://apextraderfunding.com/help-center/getting-started/futures-trading-times/", accessed: "2026-10-23" }
    - { field: copy_trading_allowed (A), url: "https://apextraderfunding.com/help-center/performance-accounts-pa/how-many-paid-funded-accounts-am-i-allowed-to-have/", accessed: "2026-10-23" }
    - { field: copy_trading_allowed (B), url: "https://apextraderfunding.com/help-center/performance-accounts-pa/legacy-performance-account-pa-compliance/", accessed: "2026-10-23" }
    - { field: automation_rule_text, url: "https://apextraderfunding.com/help-center/getting-started/prohibited-activities/", accessed: "2026-10-23" }

- firm: apex-trader-funding
  plan: "Intraday Trailing Drawdown (Intraday Performance Account)"
  phase: funded
  size_label: "150K"
  size_usd: "150000.00"
  instruments: [MNQ, NQ]
  platforms: [Tradovate, Rithmic, WealthCharts]
  drawdown: { amount_usd: "4000.00", type: trailing_intraday, stops_trailing_at: "CONFLICT (C2): 'In Performance Accounts, trailing stops once the Intraday Threshold reaches Starting Balance + $100.' (Intraday Drawdown Explained) vs 'Trailing stops once Max Trailing Drawdown Amount + $100 is reached' (Intraday PA page)" }
  daily_loss_limit: { amount_usd: "2500.00", kind: soft_pause, tiered: true }   # Level 1 DLL; tiers below
  profit_target_usd: null
  consistency_rule: { text: "Your largest profitable trading day must represent less than 50% of your net profit since your last payout request, or since account inception if no payout has been made. (payout eligibility only; account not failed)", threshold_pct: "50" }
  max_position:
    minis: "10"   # top-tier max as published
    micros: null   # firm publishes ratio only: "ten (10) micro contracts equal one (1) standard contract"
    scaling:
      - { level: 1, profit_range: "$0 \u2013 $1,999", max_contracts: "4", dll_usd: "2500.00" }
      - { level: 2, profit_range: "$2,000 \u2013 $2,999", max_contracts: "5", dll_usd: "2500.00" }
      - { level: 3, profit_range: "$3,000 \u2013 $4,999", max_contracts: "7", dll_usd: "2500.00" }
      - { level: 4, profit_range: "$5,000 \u2013 $9,999", max_contracts: "10", dll_usd: "3000.00" }
      - { level: 5, profit_range: "$10,000 & Up", max_contracts: "10", dll_usd: "4000.00" }
  min_trading_days: 5   # per payout request only; each qualifying day >= $300 net profit
  trading_day_reset: { time: "18:00", tz_as_published: "ET" }
  copy_trading_allowed: unknown   # C1 conflict: same-person PA copying allowed (S-MAXPA) vs Legacy PA Compliance ban; other-trader copying forbidden
  automated_trading_allowed: "no"
  automation_rule_text: "No Automation or Algorithm Usage allowed: Rewards are intended to recognize human traders actively participating in the learning process, not to reward automated systems executing preprogrammed logic. Please see the user agreement for more details, and we encourage you to reach out if you have further questions."
  status: conflict   # numeric fields verified; see not_confirmed_fields
  not_confirmed_fields: [copy_trading_allowed, "consistency_rule.threshold_pct (boundary: <50% vs example 50% met)", drawdown.stops_trailing_at]
  sources: 
    - { field: drawdown/max_contracts, url: "https://apextraderfunding.com/help-center/intraday-trailing-drawdown-accounts/intraday-trailing-drawdown-performance-accounts-pa/", accessed: "2026-10-23" }
    - { field: drawdown.stops_trailing_at, url: "https://apextraderfunding.com/help-center/intraday-trailing-drawdown-accounts/intraday-trailing-drawdown-explained/", accessed: "2026-10-23" }
    - { field: dll/scaling (A), url: "https://apextraderfunding.com/help-center/additional-helpful-items/daily-loss-limit-explained/", accessed: "2026-10-23" }
    - { field: scaling (B), url: "https://apextraderfunding.com/help-center/additional-helpful-items/scaling-levels-pa-explained/", accessed: "2026-10-23" }
    - { field: consistency_rule, url: "https://apextraderfunding.com/help-center/additional-helpful-items/50-consistency-requirement/", accessed: "2026-10-23" }
    - { field: min_trading_days, url: "https://apextraderfunding.com/help-center/intraday-trailing-drawdown-accounts/intraday-trailing-drawdown-payouts/", accessed: "2026-10-23" }
    - { field: trading_day_reset, url: "https://apextraderfunding.com/help-center/getting-started/futures-trading-times/", accessed: "2026-10-23" }
    - { field: copy_trading_allowed (A), url: "https://apextraderfunding.com/help-center/performance-accounts-pa/how-many-paid-funded-accounts-am-i-allowed-to-have/", accessed: "2026-10-23" }
    - { field: copy_trading_allowed (B), url: "https://apextraderfunding.com/help-center/performance-accounts-pa/legacy-performance-account-pa-compliance/", accessed: "2026-10-23" }
    - { field: automation_rule_text, url: "https://apextraderfunding.com/help-center/getting-started/prohibited-activities/", accessed: "2026-10-23" }
```

`not_confirmed_fields` mapping: C1 → `copy_trading_allowed` (all blocks); C2 → `drawdown.stops_trailing_at` (Intraday PA blocks); C3 → `max_position.scaling[level 4].profit_range` (50K PA blocks); C4 → consistency boundary (all PA blocks); C5 → `drawdown.stops_trailing_at` (Intraday evaluation, Rithmic/WealthCharts blocks); C6 (Legacy availability) applies to no block, because no Legacy blocks were written.

Schema notes for the PO: the blocks are a YAML list. They add `daily_loss_limit.tiered` and a structured `max_position.scaling` list for tier-based PAs. `automated_trading_allowed` is quoted as `"no"` so YAML does not read it as a boolean.

---

## 9. Unverified / gaps / conflicts

| # | Field | Issue | Where searched |
|---|---|---|---|
| C1 | Copy trading stance | `conflict`. S-MAXPA: "Copy trading across PA accounts under your personal name and business name is allowed." S-LEGCOMP (Legacy PA Compliance page): PAs and Live accounts may not be "managed or influenced by any other party, person, system, automated trading bot, copy trading service, or trade mirroring software." S-GROUP/S-TVFAQ/S-PLAT document copiers positively. | Help center search for "copy", "copier", "trade copier", "multiple accounts" |
| C2 | Intraday PA: when trailing stops | `conflict`. "Starting Balance + $100" (S-INTDD) vs "Max Trailing Drawdown Amount + $100" (S-INTPA). | S-INTDD, S-INTPA |
| C3 | 50K PA Level 4 threshold | `conflict`. "$6,000 & Up" (S-DLL) vs "$5,999 & Up" (S-SCALE). | S-DLL, S-SCALE |
| C4 | 50% consistency boundary | `conflict` (wording). "must be below 50%" / "less than 50%" vs the example table showing 50% as "✓ Consistency Met" (S-CONS). | S-CONS, S-EODPAY |
| C5 | Intraday eval (Rithmic/WealthCharts) trailing stop | `conflict` (wording). Body text "reaches Target Profit Balance" vs FAQ "reaches Profit Target Balance + $2,000". The examples agree on $53,000 for 50K. | S-INTDD |
| C6 | Legacy availability | `conflict`. S-LEGOV "no longer available for purchase to anyone" vs S-LEGPROD/S-HOME "Legacy Accounts Are Back For a Limited Time" and S-INACT "promotional Legacy accounts purchased after March 1st, 2026". | Legacy pages, homepage |
| G1 | Automation details in User Agreement | `unverified (behind login / not public)`. S-PROHIB says "Please see the user agreement for more details". The Evaluation and Performance Account User Agreement is not published, and the Terms of Use have no automation section. | Terms of Use, Risk Disclosure, site search "user agreement" |
| G2 | Webhook / signal-driven automation explicitly named | Not found. The site search for "webhook" returned nothing. The general prohibition (S-PROHIB) applies. | Site search "webhook", "bot", "automation", "algorithm", "tradingview" |
| G3 | Copying to/from accounts at **other prop firms** | `unverified`. Not addressed. S-TVFAQ only mentions copying to "personal broker accounts" via NinjaTrader. | Site search "copy", "copier" |
| G4 | PA max position in micros per size | The firm publishes only the ratio "ten (10) micro contracts equal one (1) standard contract". Micro counts per PA size/tier were not derived. | S-SCALE |
| G5 | Legacy PA presets (and Legacy eval sizes 250K/300K, 75K, 100K Static) | Not researched in depth. Only noted (§3.5). Waiting on the PO decision (depends on C6). | S-LEGPROD, S-LEGEVAL, S-LEGPA |
| G6 | Sizes other than 25K–150K for current products | None offered: the homepage picker and help-center tables list only 25K, 50K, 100K and 150K. No 200K/250K/300K current products. | S-HOME, S-EODEVAL, S-INTEVAL |
| N1 | Platform-dependent evaluation rule | Resolved in rev. 2: evaluation blocks are split by platform (`[Rithmic, WealthCharts]` / `[Tradovate]`). | S-EODDD, S-INTDD |
| G7 | NinjaTrader connection guides | Not captured. They are cited only by title from the official site search (S-NTRITH, S-NTTV). This does not affect rule values. | site search "ninjatrader" |

---

## 10. Changelog

- 2026-10-23 — initial research (Lupa)
- 2026-10-23 — retrofit to format rev. 2 (Lupa). Added §2 Platforms (vendor list read from the homepage picker for all 16 combinations) and a `platforms:` field. Split evaluation YAML by platform (24 blocks, up from 16). Added `automation_rule_text` and `not_confirmed_fields` (C1–C5) to every block. Added the PO automation classification §6.1 (i)/(ii)/(iii). `copy_trading_allowed` changed from `conditional` to `unknown` (not confirmed, C1). Renumbered sections to the rev. 2 order.
