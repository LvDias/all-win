# Alpha Futures — Prop Firm Rules Research

| | |
|---|---|
| **Firm** | Alpha Futures (Alpha Futures Limited, England and Wales, company number 15655643; futures only). Part of the "Alpha Group" together with Alpha Capital Group and ACG Markets (S27 FAQ). **Alpha Capital Group (forex) is a separate firm and was not researched.** |
| **Official domains** | `alpha-futures.com` (main site, Webflow; plain curl works; `www.` redirects here) · `help.alpha-futures.com` (official help center, Intercom-hosted, "Alpha Futures Limited Help Center"; linked from the main site header/footer "Help Center" → `https://help.alpha-futures.com/en/`) · `app.alpha-futures.com` (dashboard login, not used) · `consistencycalculator.alpha-futures.com` (calculator, not used) · `futures.alphatrader.com` / `alphatrader.com` (AlphaTrader platform, not read) |
| **Research date** | 2026-10-23 (all URLs below accessed 2026-10-23) |
| **Researcher** | Lupa (Prop Firm Rules Researcher) |
| **File status** | `partial`. Drawdown, daily loss guard, profit target, consistency, max position/scaling, trading day, copy and automation stances are verified on official pages for every current plan and size (Zero, Standard, Advanced, Direct). Open: several conflicts between the help center and the Terms and Conditions (copy-trading scope, max accounts, some MLL percentages, one min-days value, one micro limit), the personal-device rule (not mentioned), and most live-account details. See §9 |
| **Instruments covered** | MNQ, NQ. "The CME Group Products are available to trade here at Alpha Futures." (S10). NQ and MNQ are named explicitly in the price-limit article (S19: "Equity Products ES, MES, NQ, MNQ, RTY, M2K, YM, and MYM overnight price limits are 7%.") |
| **Platforms (for routing)** | AlphaTrader, TradingView (via Plus500), WealthCharts, DeepChart (Volumetrica Trading), Quantower (dxFeed, Volumetrica Trading). **NinjaTrader, Tradovate, Rithmic and CQG are not offered** (no mention on any official page). See §2 |

Evidence (raw text of every cited page, URL on line 1; raw HTML in `raw/`): `/tmp/claude-1003/-srv-www-allwin--maestri-roles-6bf77a80-a1ed-43dc-bc00-08a480f178a5/33652d13-4dae-46b6-ae06-44a72b375517/scratchpad/r2/alpha-futures/`. This is temporary scratch storage, so the evidence must be captured again at re-verification.

**Programs found.** Four current account types, named on the site and help center: **Zero**, **Standard**, **Advanced** (each: one-phase Evaluation → Qualified Account) and **Direct** (no evaluation; starts as a Qualified Account, one-time fee). "You choose an account type (Zero, Standard, Advanced, or Direct), pick your account size, and start trading inside our simulated trading environment." (S27). Sizes: Zero 25K / 50K / 100K; Standard 50K / 100K / 150K; Advanced 50K / 100K / 150K; Direct 25K / 50K / 100K / 150K. No 200K, 250K or 300K size exists. Qualified Accounts are simulated ("All Alpha Futures accounts operate inside a simulated trading environment.", S27). **Live** accounts (real capital, call-up only) exist through the "Path To Live" (Alpha Futures Live Program or Alpha Prime Program) (S15, S16).

**Alpha Futures terms.** "MLL" = Maximum Loss Limit (the drawdown). "Daily Loss Guard" (DLG) = their daily loss limit ("The Daily Loss Limit also referred to as the Daily Loss Guard", S6). "Qualified Account" = simulated funded stage (`phase: funded` in the YAML). The T&C call traders "Analysts" ("Analyst Account" = evaluation, "Qualified Analyst Account" = qualified).

### Main source list (all accessed 2026-10-23)

| Key | URL |
|---|---|
| S1 | https://help.alpha-futures.com/en/articles/11632512-standard-account-overview (dated July 27, 2026) |
| S2 | https://help.alpha-futures.com/en/articles/11634907-advanced-account-overview (July 22, 2026) |
| S3 | https://help.alpha-futures.com/en/articles/11771813-zero-account-overview (August 12, 2026) |
| S4 | https://help.alpha-futures.com/en/articles/15838742-direct-account-overview (July 9, 2026) |
| S5 | https://help.alpha-futures.com/en/articles/9491999-maximum-loss-limit-mll (July 15, 2026) |
| S6 | https://help.alpha-futures.com/en/articles/9492014-daily-loss-guard (July 15, 2026) |
| S7 | https://help.alpha-futures.com/en/articles/9492025-scaling-plan (July 24, 2026) |
| S8 | https://help.alpha-futures.com/en/articles/9492048-consistency-rule (July 27, 2026) |
| S9 | https://help.alpha-futures.com/en/articles/9492063-news-trading-policy (July 27, 2026) |
| S10 | https://help.alpha-futures.com/en/articles/9492096-what-and-when-you-can-trade (July 17, 2026) |
| S11 | https://help.alpha-futures.com/en/articles/9508585-prohibited-trading-practices (October 30, 2025) |
| S12 | https://help.alpha-futures.com/en/articles/10573989-copy-trading (July 17, 2026) |
| S13 | https://help.alpha-futures.com/en/articles/9492088-maximum-allocation (July 27, 2026) |
| S14 | https://help.alpha-futures.com/en/articles/10525305-trading-platforms ("Updated over 3 weeks ago") |
| S15 | https://help.alpha-futures.com/en/articles/10743344-path-to-live-structure (August 21, 2026) |
| S16 | https://help.alpha-futures.com/en/articles/11023753-live-account-rules-and-parameters (June 20, 2026) |
| S17 | https://help.alpha-futures.com/en/articles/9492051-payout-policy (July 27, 2026) |
| S18 | https://help.alpha-futures.com/en/articles/12757982-inactivity-rule |
| S19 | https://help.alpha-futures.com/en/articles/9806068-price-limit-trading |
| S20 | https://help.alpha-futures.com/en/articles/9867976-completing-kyc |
| S21 | https://alpha-futures.com/product/standard (plan cards + FAQ) |
| S22 | https://alpha-futures.com/product/advanced (plan cards + FAQ) |
| S23 | https://alpha-futures.com/product/zero (plan cards + FAQ) |
| S24 | https://alpha-futures.com/product/direct (plan cards + FAQ) |
| S25 | https://alpha-futures.com/platforms |
| S26 | https://alpha-futures.com/terms-and-conditions ("Effective from July 27, 2026"; §8 Virtual Trading Rules, Schedules 1–5) |
| S27 | https://alpha-futures.com/how-it-works (FAQ) and https://alpha-futures.com/ (home FAQ) |
| S28 | https://help.alpha-futures.com/en/articles/9492077-reset |
| S29 | https://help.alpha-futures.com/en/articles/10525620-how-to-trade-with-tradingview · …/14135839-getting-started-with-quantower · …/14112469-getting-started-with-deepchart · …/15018330-getting-started-with-wealthcharts · …/15271201-getting-started-with-alphatrader |
| S30 | https://help.alpha-futures.com/en/articles/15855255-live-cool-down-period |
| S31 | https://alpha-futures.com/terms-of-use |

---

## 1. Header notes

- The help center's 5 collections (Alpha Prime, Getting Started, Monthly Subscription, Trading Rules and Parameters, Payout Information) list 36 articles; all 36 were downloaded with curl and saved to the evidence directory. The help center's search page is JS-rendered and was not used.
- The four product pages (S21–S24), the platforms page (S25), How It Works and home (S27), the Terms and Conditions (S26), the Terms of Use (S31) and the General Service Agreement were also saved.
- The product pages agree with the help center on every USD value checked. The **Terms and Conditions (S26) disagree with the help center in several places** (§9). Several T&C values are written as percentages ("4%", "3-4%"). They were compared with the USD values only to detect conflicts; no value in this file was derived from a percentage.
- No login, form or chat was used. The browser portal was not needed (curl returned 200 for every page).

## 2. Platforms

Quote (S14, bullet list flattened with " — " and " · "): "At Alpha Futures, our traders have their choice of using: AlphaTrader — Our in house developed platform, available on web and mobile app · TradingView — Connection powered by Plus500 · WealthCharts — A powerful, web based platform designed by live capital trading champions · Deepchart by Volumetrica Trading — Web based or desktop download available, Complimentary access to "OrderFlow" package · Quantower — Desktop and PC only, dXFeed data feed, utilizing Volumetrica Trading clearing house" and "Please note, platform choices are not interchangeable after purchase."

| Platform | Connection / data as published | Plans |
|---|---|---|
| AlphaTrader | in-house; "available on web and mobile app" (S14); has a "Built in trade copier" (S25) | all plans (no per-plan restriction published) |
| TradingView | "Connection powered by Plus500" (S14); "available at checkout on every plan" (S21–S25); connect via broker "Plus500US", "demo", firm "AlphaTrader" (S29) | every plan (stated) |
| WealthCharts | web based; "Built-In Trade Copier & OmniProp Support" (S25) | all plans (no restriction published) |
| DeepChart | "by Volumetrica Trading" (S14) | all plans (no restriction published) |
| Quantower | "dXFeed data feed, utilizing Volumetrica Trading clearing house" (S14); connection "dxFeed Prop" (S29) | "Choose Quantower when you buy an Alpha Futures evaluation — platform choice is locked at purchase." (S25). Whether Direct can choose Quantower is not stated |

- **Not offered: NinjaTrader, Tradovate, Rithmic, CQG.** None of these names appears on any page read (36 help articles, site pages, T&C). **Consequence for All Win:** the NinjaTrader 8 plugin path cannot connect to an Alpha Futures account; any preset would need a different adapter (e.g. TradingView) — PO/Architect decision.
- No rule published differs by platform, so the YAML has one block per plan/phase/size (not per platform).
- Live accounts: platforms not published (S15/S16 mention only "our partnered brokerage").

## 3. Plans

Common to all Evaluation and Qualified accounts (S5): "The Maximum Loss Limit (MLL) is EOD (end of day) trailing on all of our accounts here at Alpha Futures. None of our accounts utilize intraday trailing drawdown you will see elsewhere in industry. Maximum Loss Limit stops trailing at the account starting balance on all of our accounts." / "The Maximum Loss Limit is calculated from your account balance high at the end of each trading day, not intraday equity high" / "If you break MLL at any point, floating equity or closed balance of account, your account will be liquidated." / "Hitting MLL on an Alpha Futures Qualified Account means the account has been breached and will be closed." (T&C S26 Schedule 1: "…will be closed at the end of that trading day.")

Daily Loss Guard (S6): "The Daily Loss Limit also referred to as the Daily Loss Guard is a soft breach rule on Alpha Futures Accounts. If you hit the Daily Loss Guard, you do not lose your Account, but your account will be locked until the next trading day (6PM ET)." / "If your open or closed (unrealized or realized) P&L at any point reaches -2% during the trading day, all open positions will be flattened, any pending orders will be canceled, and your account will be prevented from placing any new trades until the start of the next trading day." / "The Daily Loss Guard is based on each trading day's P&L, which includes simulated commissions, fees, and both unrealized and realized trade P&L." → kind **soft_pause** wherever a DLG exists.

Ratio: every published limit pairs minis and micros as "N minis, 10×N micros" (e.g. "5 minis, 50 micros"); no separate ratio sentence is published. "MBT and MET … are treated as full mini contracts" (S10).

### 3.1 Zero — Evaluation (S3, S23)

| Size | Profit target | MLL | Drawdown type | Daily Loss Guard | Consistency | Max position | Min days |
|---|---|---|---|---|---|---|---|
| 25K ($25,000) | $1,500 | $1,000 | trailing EOD; stops at starting balance | $500 (soft pause) | none | 1 mini / 10 micros | 1 |
| 50K ($50,000) | $3,000 | $2,000 | same | $1,000 (soft pause) | none | 3 minis / 30 micros | 1 |
| 100K ($100,000) | $6,000 | $3,000 | same | $2,000 (soft pause) | none | 6 minis / 60 micros | 1 |

- S3: "ONE DAY PASS! No Consistency Rule on Zero Evaluations means you may pass in one day." / "There is only one way to fail on Evaluation Accounts, do not allow the Account Balance to hit or exceed the Maximum Loss Limit."
- DLG on Zero Evaluations (S6): "There is no Daily Loss Guard on Advanced Evaluations, but there is on Zero Evaluations." / "The Daily Loss Guard is 2% of starting account balance on Zero Accounts." / "25k Zero Account = $500 · 50K Zero Account = -$1,000 · 100K Zero Account = -$2,000". **Conflict note:** T&C Schedule 2 notes say "The Daily Loss Guard is a soft breach rule on Qualified Analyst Accounts only", while T&C Schedule 1 lists Zero Evaluation "Daily Loss Guard: 2% of starting account balance" (§9 G6).
- Min days: S23 cards "Minimum Trading Days 1"; T&C 8.1.9: "A zero account assessment may be passed in a single trading day."
- **MLL 100K:** help/product "$3,000"; T&C Schedule 1 Zero Evaluation "Maximum Loss Limit 4%" and notes "a trailing drawdown set at 4% of all account sizes" → conflict for 100K (§9 G4). The T&C tables do not list a Zero 25K size at all.

### 3.2 Zero — Qualified (S3, S7, S8, S23)

| Size | MLL | Daily Loss Guard | Consistency | Max position (scaling) | Min days |
|---|---|---|---|---|---|
| 25K | $1,000 | $500 | 40% | 1 mini / 10 micros; "Scaling Rule (Qualified only): No" | 5 |
| 50K | $2,000 | $1,000 | 40% | 3 minis / 30 micros with scaling plan | 5 |
| 100K | $3,000 | $2,000 | 40% | 6 minis / 60 micros with scaling plan | 5 |

- Scaling plan, Zero Qualified (S7; same table in T&C Schedule 3), minis/micros by profit:

| Profit | 50k | 100k |
|---|---|---|
| Less than $1,500 profit | 1 / 10 | 2 / 20 |
| $1,500-2,000 profit | 2 / 20 | 3 / 30 |
| $2,000-3,000 profit | 3 / 30 | 4 / 40 |
| $3,000-4,500 profit | - | 5 / 50 |
| $4,500+ profit | - | 6 / 60 |

- Consistency (S8): "There is a 40% Consistency Rule on Zero and Standard Qualified Accounts. The 40% Consistency Rule states during the Qualified stage, profits from any single trading day cannot be greater than or equal to 40% of net profits accumulated since last withdrawal request." / "If you make more than 40% in a single day, your account will NOT be breached, but you will need to continue to trade and make more profit until the rule is satisfied." / formula published: "biggest day/total net profit (since last request) = 40%". It is a payout-eligibility rule, not a breach.
- Min days: S23 card "Minimum Trading Days 5"; T&C Schedule 2 "5".
- Payout gating (S17): "Request every 5 winning trading days of $200 profit, or more" and "You may request up to 50% of the profit in your account each withdrawal request". S3: "No "MLL slides to $0 at first withdrawal" rule. … If MLL was at $49,200 before payout, it will be after as well". S26 Sch. 4 1.2.5: "All virtual trades must be closed within your Qualified Analyst Account at the time of making a withdrawal request, and all virtual trading will be paused and can only resume once the withdrawal amount has been withdrawn".
- MLL 100K conflict with T&C "4%" (Schedule 2, Zero Qualified) — §9 G4.

### 3.3 Standard — Evaluation (S1, S21)

| Size | Profit target | MLL | Drawdown type | Daily Loss Guard | Consistency | Max position | Min days |
|---|---|---|---|---|---|---|---|
| 50K | $3,000 | $2,000 | trailing EOD; stops at starting balance | none ("NONE on Evaluation!") | 50% | 5 minis / 50 micros | 2 |
| 100K | $6,000 | $3,000 | same | none | 50% | 10 minis / 100 micros | 2 |
| 150K | $9,000 | $4,500 | same | none | 50% | 15 minis / 150 micros | 2 |

- S1: "In order to pass a Standard Evaluation: there are two conditions to be met, the trader must reach (+ maintain) the profit target, and satisfy the 50% consistency rule." / "2.) No Daily Loss Guard on Evaluations."
- Consistency (S8): "The Consistency Rule on Standard Evaluations is 50%. This means that during your Evaluation, profits from one single trading day cannot be larger than 50% of your net profits made" / "You will not pass your Evaluation until this rule is satisfied." Recommended best day: 50k "$1,500 or less", 100k "$3,000 or less", 150k "$4,500 or less".
- Min days: S21 "Minimum Trading Days 2"; T&C 8.1.9 and Schedule 1 "2".
- Max position 100K: T&C 8.1.10(a) says "100k = 10 contracts/10 micros" while S1, S21 and T&C Schedule 1 say 100 micros → conflict (§9 G7).

### 3.4 Standard — Qualified (S1, S7, S8, S21)

| Size | MLL | Daily Loss Guard | Consistency | Max position (scaling) | Min days |
|---|---|---|---|---|---|
| 50K | $2,000 | $1,000 (soft pause) | 40% | 5 minis / 50 micros with scaling plan | 5 |
| 100K | $3,000 | $2,000 (soft pause) | 40% | 10 minis / 100 micros with scaling plan | 5 |
| 150K | $4,500 | $3,000 (soft pause) | 40% | 15 minis / 150 micros with scaling plan | 5 |

- Scaling plan, Standard Qualified (S7; same in T&C Schedule 3):

| Profit | 50k | 100k | 150k |
|---|---|---|---|
| Less than $1,500 profit | 2 / 20 | 3 / 30 | 3 / 30 |
| $1,500-2,000 profit | 3 / 30 | 4 / 40 | 4 / 40 |
| $2,000-3,000 profit | 5 / 50 | 5 / 50 | 5 / 50 |
| $3,000-4,500 profit | 5 / 50 | 10 / 100 | 10 / 100 |
| $4,500+ profit | 5 / 50 | 10 / 100 | 15 / 150 |

- DLG: S1 table "Daily Loss Guard (NONE on Evaluation!) $1,000 / $2,000 / $3,000"; S21 Qualified cards "$1000", "$2000", "$3000". T&C Schedule 2 Standard Qualified: "2% of starting account balance. This is NOT a fail, it just locks your account for the day".
- Consistency 40% (same text as §3.2). News restrictions apply (§7).
- Max accounts: conflict (§7, §9 G8).

### 3.5 Advanced — Evaluation (S2, S22)

| Size | Profit target | MLL | Drawdown type | Daily Loss Guard | Consistency | Max position | Min days |
|---|---|---|---|---|---|---|---|
| 50K | $4,000 | $1,750 | trailing EOD; stops at starting balance | NONE | 40% | 5 minis / 50 micros | 3 (conflict: 2) |
| 100K | $8,000 | $3,500 | same | NONE | 40% | 10 minis / 100 micros | 3 (conflict: 2) |
| 150K | $12,000 | $5,250 | same | NONE | 40% | 15 minis / 150 micros | 3 (conflict: 2) |

- S2: "In order to pass an Advanced Evaluation: there are two conditions to be met, the trader must reach (+ maintain) the profit target, and satisfy the 40% consistency rule." / "3.) No Daily Loss Guard on Evaluation or Qualified Accounts".
- S6: "Advanced Accounts, there is no preset Daily Loss Guard, but traders may set one themselves at any time in their trading platform." → `kind: none`.
- Consistency (S8): "The Consistency Rule on Advanced Evaluations is 40%. This means that during your Evaluation, profits from one single trading day cannot be larger than 40% of your net profits made." Recommended best day: 50k "$1,600 or less", 100k "$3,200 or less", 150k "$4,800 or less".
- **Min days conflict:** S22 cards "Minimum Trading Days 3" and FAQ "hit the profit target in at least 3 trading days"; T&C 8.1.9 "The standard and advanced account assessments consist of 2 and 3 minimum trading days"; but T&C Schedule 1 "Advanced Evaluation | Minimum Virtual Trading Days | … 2" (§9 G5).

### 3.6 Advanced — Qualified (S2, S22)

| Size | MLL | Daily Loss Guard | Consistency | Max position | Min days |
|---|---|---|---|---|---|
| 50K | $1,750 | NONE | none | 5 minis / 50 micros, no scaling | 5 |
| 100K | $3,500 | NONE | none | 10 minis / 100 micros, no scaling | 5 |
| 150K | $5,250 | NONE | none | 15 minis / 150 micros, no scaling | 5 |

- S2: "4.) No Consistency Rule on Advanced Qualified Accounts" / "5.) No News Trading Rule or restrictions" / "6.) No Scaling Plan on Advanced Accounts. Get access to full contract limit at all times." S22 cards: "5 minis / 50 micros (No Scaling Plan!)", "Daily Loss Guard None", "Consistency Rule None".
- **MLL conflict:** S2/S22 $1,750 / $3,500 / $5,250; T&C Schedule 2 "Advanced Qualified | 5 | None | 4%" (§9 G4).
- Max accounts: conflict 3 vs 5 (§9 G8).

### 3.7 Direct — Qualified only (S4, S17, S24)

| Size | MLL | Daily Loss Guard | Consistency | Max position | 1st / 2nd+ Withdrawal Profit Target | Min days |
|---|---|---|---|---|---|---|
| 25K | $1,000 | $500 (soft pause) | 20% | 2 minis / 20 micros, no scaling | $1,500 / $1,000 | 5 (T&C only) |
| 50K | $2,000 | $1,000 | 20% | 4 minis / 40 micros | $3,000 / $2,000 | 5 (T&C only) |
| 100K | $3,000 | $2,000 | 20% | 8 minis / 80 micros | $6,000 / $4,000 | 5 (T&C only) |
| 150K | $4,500 | $3,000 | 20% | 10 minis / 100 micros | $9,000 / $6,000 | 5 (T&C only) |

- S4: "No Evaluation, jump straight into a Qualified Account from day one" / "5.) No Scaling Plan on Direct Qualified Accounts" / "in order to eligible for a withdrawal request, user must satisfy the Withdrawal Profit Target, and 20% Consistency Rule." / "Withdrawal Profit Targets are the profit needed to earn in between withdrawal requests. After each withdrawal, your profit target resets and you must earn fresh profit from new trades to reach the next target. Leftover profit from the previous cycle does not carry over." The withdrawal profit target is a payout gate, not a pass target, so `profit_target_usd: null` in the YAML (values in the YAML comment).
- S24 cards call it "EOD MLL" and "Daily Loss Limit" ($500 / $1,000 / $2,000 / $3,000). S8: "There is a 20% Consistency Rule on Direct Qualified Accounts. The 20% Consistency Rule states profits from any single trading day cannot be greater than or equal to 20% of net profits accumulated since last withdrawal request."
- Min days: only T&C Schedule 2 gives "Direct Qualified | 5". S17: "Direct Accounts have the same profit split, but no winning day requirement". Not confirmed (§9 G9).

### 3.8 Live accounts (Path To Live; real capital, by call-up only) (S15, S16, S30)

- Call-up (S15): "A trader may be reviewed or called to live immediately if they have met one or more of the following conditions: 5 performance fees on a single Qualified Account · A significant amount of lifetime performance fees · Outstanding performance in a Qualified Account · Former member of the Generic Live Program or Alpha Prime Program". "When a trader is called to live, all Qualified Accounts are closed" / "Traders receive one live account per eligible Qualified Account". (S27 FAQ words the trigger differently: "+$40,000 in payable balance, or after 5 payout cycles".)
- Live summary (S15): "Daily Payouts · No Consistency Rule · No news trading restrictions · No maximum withdrawal request · EOD Style Drawdown · Scaling Daily Loss Limit (30% of account)".

| Eligible Qualified Account | Live Start Balance | Live EOD Drawdown | Contracts (before MLL reaches $0 balance) | Contracts (after MLL reaches $0 balance) |
|---|---|---|---|---|
| 25K | $0 | $1,000 | 1 mini / 10 micro | 2 mini / 20 micro |
| 50K | $0 | $2,000 | 2 mini / 20 micro | 4 mini / 40 micro |
| 100K | $0 | $3,000 | 4 mini / 40 micro | 8 mini / 80 micro |
| 150K | $0 | $4,500 | 6 mini / 60 micro | 12 mini / 120 micro |

- Legacy live (S16): "Our Legacy Live Traders before June 21 start with a $10,000 static, non trailing, drawdown account." / "Daily Loss Limit: starts at $3,000 and continues to scale at 30% of account balance as you gain. … This is not a breach of the account, simply a pause until next trading day." / "Trades do not need to be closed by 4:20pm EST (like they do on sim accounts). Swing trading/holding overnight is indeed permitted". Prime option: "a singular static account up to $10,000" (S15). Legacy accounts are not modeled in YAML.
- Not published for live: whether the EOD drawdown trails or when it stops, the DLL as a USD amount per size, platforms, trading-day reset, copy/automation/VPS stance. Live YAML blocks are `partial` with these in `not_confirmed_fields`.
- Losing a live account (S30): "All live traders face a "cool down" period if their live account is lost." / "The baseline target for a live account is 3 months."

## 4. Trading-day reset

- **New trading day: 6PM ET.** Quote (S6): "your account will be locked until the next trading day (6PM ET)". T&C Schedule 2 (S26): "locked until the next trading day (6PM ET)".
- **Session (S10):** "Trading hours are 6PM EST to 5PM EST the next day in Futures." / "They are not open from Friday 5PM EST until Sunday at 6PM EST."
- **Must be flat (S10):** "You may begin trading at 6PM EST to start a new trading day, but all trades must be closed before 4:20PM EST every day. We will close the trades for you if you forget". T&C Schedule 2: "All trades must be closed before 4:20pm ET each day on both Analyst Accounts and Qualified Analyst Accounts." Live accounts are exempt (S16).
- Time-zone wording: pages use both "ET" and "EST" for the same times. Recorded as published; YAML uses `"ET"` (the DLG reset sentence).
- Holidays / early closes: **not mentioned** on any official page read (§9 G13).
- MLL/DLG day boundary: MLL is set from "account balance high at the end of each trading day" (S5); the exact EOD snapshot time is not published separately.

## 5. Trade copying stance

Classification: Evaluation and Qualified **conditional**, with a **conflict** on scope. Live **unknown**.

- Help center (S12, title "Can I copy across my Alpha Futures Accounts?"): "Many platforms have built in trade copiers. Copy trading in general is permitted as long as it is a single user executing the trades, and it is the single individual listed on the Alpha Futures Accounts." / "Automated, Group, and Reverse Trading are strictly prohibited." / "To avoid complications with copy trading, use your smallest account/one with the least amount of profit or leverage as your leader account. If the accounts have different leverage or buying power trades may not copy." / "Please be aware copy traders are not perfect and impose risk, even if they are available directly on your trading platform. Alpha Futures is not liable or able to fix any issues you may face while using a trade copying software."
- **Terms and Conditions (S26 §8.1.5):** "the only copy trading allowed is that from your own external account to your Alpha Futures Account. Any Analyst found copy trading other analyst's trades will be subject to review and if found to be in breach of the Virtual Trading Rules, will result in instant termination of the Services and the closure of your Alpha Futures Account;"
- **Conflict:** S12 allows copying across the same person's Alpha Futures accounts; T&C §8.1.5 says the "only" copy trading allowed is from the trader's own external account to the Alpha Futures account. Both agree copying another person's trades is not allowed. Not resolved here (§9 G1).
- **Copying from other firms / external accounts:** T&C §8.1.5 allows "from your own external account to your Alpha Futures Account". Copying **from** Alpha Futures **to** other firms: not mentioned.
- Platform copiers: AlphaTrader "Built in trade copier"; WealthCharts "Built-In Trade Copier & OmniProp Support" (S25).
- Group trading (S11): "Collaborating with others to execute identical or opposite strategies across unconnected accounts is prohibited. Also referred to as "Group Trading"."
- Hedging (S11): ""Reverse Trading" or Hedging, meaning going short on one account, and long on the other is strictly prohibited at Evaluation and Qualified Phase. Long NQ and short MNQ on the same account is another example, and strictly prohibited."
- Account limits: see §7 (conflict).

## 6. Automated trading stance

**(i) Classification:** Evaluation and Qualified (all plans) **forbidden** (YAML `no`), with semi-automated signal tools allowed if the trader places and manages trades manually. Live: **unknown** (S11's intro limits the article to "Evaluation and Qualified stages").

Verbatim rule text (help center, S11, used as `automation_rule_text`): "The use of AI, bots, and other automated trading mechanisms is strictly prohibited across all account types."

Full section (S11, "Can I automate my trading?"):
- "Alpha Futures restricts the use of automated systems designed for high-frequency trading (HFT), particularly those resulting in over 100 trades per day. Alpha Futures & Alpha Prime team are not interested in working with strategies taking 100+ trades per day, automated or not."
- "The use of AI, bots, and other automated trading mechanisms is strictly prohibited across all account types."
- "Semi-Automated Trading, such as a custom indicator that gives buy or sell signals, is permissible under the condition that the user manually places, monitors, and manages the trade while understanding the system and semi-automation's purpose."
- "Any form of hands-off, continuous day and night trading, or any other type of full automation is strictly forbidden."

T&C Schedule 5 (S26) has the same rule with slightly different wording: "The use of AI, bots, and other automated trading mechanisms is strictly prohibited across all account types." / "Semi-Automated Trading is permissible under the condition that analysts actively monitor, manually manage, and understand the system and semi-automation's purpose." The copy-trading article repeats: "Automated, Group, and Reverse Trading are strictly prohibited." (S12)

**(ii) Named mechanisms:**
- **Semi-automated trading:** mentioned, allowed with conditions (quotes above: the user "manually places, monitors, and manages the trade").
- **Trade copiers:** mentioned (S12, S25), see §5. Copying is allowed while "Automated … Trading" is prohibited; the firm does not say whether an automatic copier counts as automation.
- **Webhooks:** not mentioned. **Alerts:** TradingView "Price and indicator alerts" and WealthCharts "Real-Time Alerts" are listed as platform features (S25); alert-to-order automation is not addressed.
- **ATM / automatic brackets:** ATM not mentioned. AlphaTrader: "One Click Trading: drag to set stop loss and take profit." (S25). Misuse prohibited (S11): "Exploiting the absence of slippage and utilizing tight brackets to gain from favorable fills are not permitted."
- Site Terms of Use (S31) ban "any automated use of the system" for the website (scrapers, bots); this concerns site use, not trading.

**(iii) Differs by phase:** no for Evaluation vs Qualified ("across all account types"; S11 intro "in Evaluation and Qualified stages"). Live: not stated. The Alpha Prime program lists "Developing automated strategies with our in house Quantitative Team" as a benefit (S15), so the live stance may differ (§9 G3).

**Consequence for All Win:** automatic routing/copying of signals into Alpha Futures accounts by All Win is fully automated order entry, which these texts forbid. The opt-in warning must show the verbatim text above.

## 7. Other rules that affect routing or risk

### Device / hosting rules (format rev. 3 §3 item 7)

YAML for all Evaluation/Qualified blocks: `vps_allowed: conditional`, `personal_device_required: unknown`. Live blocks: `vps_allowed: unknown`.

- "The use of VPS is permitted in your trading, however the use of VPN attempting to hide or change you IP address is not." (https://help.alpha-futures.com/en/articles/9508585-prohibited-trading-practices, accessed 2026-10-23)
- "The use of VPS/VPN during the KYC process is not permitted. The usage of one of these devices can lead to account closure." (https://help.alpha-futures.com/en/articles/9867976-completing-kyc, accessed 2026-10-23)
- Classified `conditional` because VPS is permitted for trading, while VPN IP masking is not, and neither may be used during KYC. The T&C (S26) do not mention VPS or VPN.
- **Personal device:** not mentioned (R4 re-check 2026-10-23: re-fetched the prohibited-practices, KYC, copy-trading, TradingView and trading-hours help articles, the T&C, the Terms of Use, the home and how-it-works pages; searched personal, own computer, device, IP address, remote, third party, shared, location). Nearest texts, none a device rule: "The use of VPS is permitted in your trading, however the use of VPN attempting to hide or change you IP address is not." (prohibited-practices); "During the KYC process, it is mandatory that the account holder personally completes the KYC requirements. … Any attempt to circumvent this requirement, or Third Party Involvement at all, will be considered a" breach (KYC article, https://help.alpha-futures.com/en/articles/9867976-completing-kyc, accessed 2026-10-23); T&C: "Only one username per individual is permitted, and one individual per household may use our services." (https://alpha-futures.com/terms-and-conditions, accessed 2026-10-23). `personal_device_required` stays `unknown`. `vps_allowed` stays `conditional` (newest help-center text unchanged).

### Other rules

- **News (S9):** "There are no news trading restrictions for any Evaluation Accounts." / "Advanced Qualified Accounts do not have News Trading Restrictions" / "Direct, Standard, and Zero Qualified Accounts: You may hold through news releases following your trading plan, but no orders may be executed within 2 minutes before or 2 minutes after high impact news events." Source calendar: ForexFactory red folder ("USD for /ES, NQ, /GC, /CL for example"). Speeches: "The 4 minute window only applies to the start time listed of the speech event". Penalty (S9): "the first time violating this rule will only be a warning … Subsequent violations after warning will lead to breach of the account." (T&C Schedule 5 words it differently: "a few times violating this rule will be treated as honest accidents"; §9 G11.) Live: "No news trading restrictions" (S15).
- **Max accounts / allocation (conflict, §9 G8):**
  - S13: "Zero Account: 5 Qualified Accounts · Direct Account: 5 Qualified Accounts · Standard Account: 5 Qualified Accounts (any standard accounts prior to July 27th, 2026 are limited to 3) · Advanced Account: 3 Qualified Accounts · Any user that has an active Advanced Qualified account will be limited to 3. Only one username per individual is permitted, and one individual per household may use our services."
  - T&C Schedule 2: "The maximum any single user can hold is 5 Qualified Accounts. … Advanced Account: 3 Qualified Accounts. Any user that has an active Standard or Advanced Qualified account will be limited to 3."
  - T&C §8.1.3: "For Analyst Accounts, there is no maximum total allocation limit. For Qualified Analyst accounts, the maximum total allocation per household is five accounts".
  - S22 FAQ (Advanced): "You can run up to 5 accounts at the same time." S24 FAQ (Direct): "You can run up to 5 accounts per size tier."
- **Overnight / weekend:** not allowed on Evaluation/Qualified; flat before 4:20PM EST/ET daily (§4). Allowed on live (S16).
- **Hedging / reverse trading:** prohibited across accounts and within one account (NQ vs MNQ) (S11).
- **HFT:** "over 100 trades per day" restricted (S11).
- **Tick / micro scalping (S11):** ""Tick or Micro Scalping" (trades less than 10 ticks AND less than 2 minutes)" — "This is only an issue when it is a clear pattern in the trader's profit." (T&C Schedule 5: "(trades less than 10 ticks)".)
- **Order book spamming, all-or-nothing, account rolling/stacking (S11):** "Using maximum leverage on a single position and using DLL as a stop loss will not be tolerated." / "excessively stacking orders (known as order book spamming) … will not be tolerated" / "Simultaneously placing multiple limit orders at the same price to manipulate order fills is prohibited." / "More examples of Account Rolling are also known as "Account Stacking", meaning excessive Evaluations and storing multiple passed accounts".
- **Price limits (S19):** "we enforce strict guidelines to prevent trading when a product is within 2% of a CME price limit." / "This prohibited conduct is enforced on ALL Alpha Futures Accounts". NQ/MNQ overnight limit 7%; the firm publishes the example "stop trading if that asset reach a 5% net gain or loss on the day".
- **Inactivity (S18):** "To keep your account active you must place a trade at least once every 10 trading days (does not matter size of trade)."
- **Qualified risk level (S26 Sch. 4 1.3.2):** "a Qualified Analyst is required to maintain the same level of risk on a Qualified Analyst Account".
- **Payout pause (S26 Sch. 4 1.2.5):** trading is paused during a withdrawal request (quote in §3.2).
- **Resets (S28):** Evaluation resets on all plans; Qualified Resets "only available on Zero and Standard Accounts", "2 times on a singular account, if the account has never reached payout request", "up to 7 days … after an account is breached".
- **Mandatory stop loss / minimum hold time:** no mandatory stop. "There is no "must hold past X minutes" rule" (S11).

## 8. Machine-readable blocks

26 blocks: Zero Evaluation ×3, Zero Qualified ×3, Standard Evaluation ×3, Standard Qualified ×3, Advanced Evaluation ×3, Advanced Qualified ×3, Direct Qualified ×4, Live ×4. "Qualified" = `phase: funded` (simulated). Common values: `trading_day_reset` 18:00 "ET" (S6; session 6PM–5PM EST, flat before 4:20PM EST/ET); `drawdown.type: trailing_eod`, stops trailing at the starting balance (S5); `automated_trading_allowed: no` with the S11 sentence as `automation_rule_text`; `copy_trading_allowed: conditional` with the scope conflict listed in `not_confirmed_fields`; `vps_allowed: conditional`; `personal_device_required: unknown`. The `platforms` list is the same for every plan (S14); only TradingView is stated "on every plan", and the Quantower choice is described for "an Alpha Futures evaluation" (S25). Max accounts has no YAML field; its conflict is listed in `not_confirmed_fields` for the affected plans and in §9.

```yaml
firm: alpha-futures
plan: "Zero"
phase: evaluation
size_label: "25K"
size_usd: "25000.00"
instruments: [MNQ, NQ]
platforms: [AlphaTrader, "TradingView (Plus500 connection)", WealthCharts, "DeepChart (Volumetrica Trading)", "Quantower (dxFeed / Volumetrica Trading)"]
drawdown: { amount_usd: "1000.00", type: trailing_eod, stops_trailing_at: "Maximum Loss Limit stops trailing at the account starting balance on all of our accounts." }
daily_loss_limit: { amount_usd: "500.00", kind: soft_pause }
profit_target_usd: "1500.00"
consistency_rule: { text: "No Consistency Rule on Zero Evaluations", threshold_pct: null }
max_position: { minis: 1, micros: 10, scaling: null }
min_trading_days: 1
trading_day_reset: { time: "18:00", tz_as_published: "ET" }
copy_trading_allowed: conditional
automated_trading_allowed: no
automation_rule_text: "The use of AI, bots, and other automated trading mechanisms is strictly prohibited across all account types."
vps_allowed: conditional
personal_device_required: unknown   # not mentioned on any official page (gap G12)
status: partial
not_confirmed_fields: ["copy_trading_allowed (scope)", "daily_loss_limit (T&C: Qualified only)", "T&C omits 25K"]
sources:
  - { field: profit_target_usd, url: "https://help.alpha-futures.com/en/articles/11771813-zero-account-overview", accessed: "2026-10-23" }
  - { field: drawdown.amount_usd, url: "https://help.alpha-futures.com/en/articles/11771813-zero-account-overview", accessed: "2026-10-23" }
  - { field: daily_loss_limit, url: "https://help.alpha-futures.com/en/articles/11771813-zero-account-overview", accessed: "2026-10-23" }
  - { field: max_position, url: "https://help.alpha-futures.com/en/articles/11771813-zero-account-overview", accessed: "2026-10-23" }
  - { field: consistency_rule, url: "https://help.alpha-futures.com/en/articles/11771813-zero-account-overview", accessed: "2026-10-23" }
  - { field: min_trading_days, url: "https://alpha-futures.com/product/zero", accessed: "2026-10-23" }
  - { field: drawdown.type, url: "https://help.alpha-futures.com/en/articles/9491999-maximum-loss-limit-mll", accessed: "2026-10-23" }
  - { field: daily_loss_limit.kind, url: "https://help.alpha-futures.com/en/articles/9492014-daily-loss-guard", accessed: "2026-10-23" }
  - { field: trading_day_reset, url: "https://help.alpha-futures.com/en/articles/9492014-daily-loss-guard", accessed: "2026-10-23" }
  - { field: copy_trading_allowed, url: "https://help.alpha-futures.com/en/articles/10573989-copy-trading", accessed: "2026-10-23" }
  - { field: copy_trading_allowed (conflict), url: "https://alpha-futures.com/terms-and-conditions", accessed: "2026-10-23" }
  - { field: automated_trading_allowed, url: "https://help.alpha-futures.com/en/articles/9508585-prohibited-trading-practices", accessed: "2026-10-23" }
  - { field: vps_allowed, url: "https://help.alpha-futures.com/en/articles/9508585-prohibited-trading-practices", accessed: "2026-10-23" }
  - { field: platforms, url: "https://help.alpha-futures.com/en/articles/10525305-trading-platforms", accessed: "2026-10-23" }
```

```yaml
firm: alpha-futures
plan: "Zero"
phase: evaluation
size_label: "50K"
size_usd: "50000.00"
instruments: [MNQ, NQ]
platforms: [AlphaTrader, "TradingView (Plus500 connection)", WealthCharts, "DeepChart (Volumetrica Trading)", "Quantower (dxFeed / Volumetrica Trading)"]
drawdown: { amount_usd: "2000.00", type: trailing_eod, stops_trailing_at: "Maximum Loss Limit stops trailing at the account starting balance on all of our accounts." }
daily_loss_limit: { amount_usd: "1000.00", kind: soft_pause }
profit_target_usd: "3000.00"
consistency_rule: { text: "No Consistency Rule on Zero Evaluations", threshold_pct: null }
max_position: { minis: 3, micros: 30, scaling: null }
min_trading_days: 1
trading_day_reset: { time: "18:00", tz_as_published: "ET" }
copy_trading_allowed: conditional
automated_trading_allowed: no
automation_rule_text: "The use of AI, bots, and other automated trading mechanisms is strictly prohibited across all account types."
vps_allowed: conditional
personal_device_required: unknown   # not mentioned on any official page (gap G12)
status: partial
not_confirmed_fields: ["copy_trading_allowed (scope)", "daily_loss_limit (T&C: Qualified only)"]
sources:
  - { field: profit_target_usd, url: "https://help.alpha-futures.com/en/articles/11771813-zero-account-overview", accessed: "2026-10-23" }
  - { field: drawdown.amount_usd, url: "https://help.alpha-futures.com/en/articles/11771813-zero-account-overview", accessed: "2026-10-23" }
  - { field: daily_loss_limit, url: "https://help.alpha-futures.com/en/articles/11771813-zero-account-overview", accessed: "2026-10-23" }
  - { field: max_position, url: "https://help.alpha-futures.com/en/articles/11771813-zero-account-overview", accessed: "2026-10-23" }
  - { field: consistency_rule, url: "https://help.alpha-futures.com/en/articles/11771813-zero-account-overview", accessed: "2026-10-23" }
  - { field: min_trading_days, url: "https://alpha-futures.com/product/zero", accessed: "2026-10-23" }
  - { field: drawdown.type, url: "https://help.alpha-futures.com/en/articles/9491999-maximum-loss-limit-mll", accessed: "2026-10-23" }
  - { field: daily_loss_limit.kind, url: "https://help.alpha-futures.com/en/articles/9492014-daily-loss-guard", accessed: "2026-10-23" }
  - { field: trading_day_reset, url: "https://help.alpha-futures.com/en/articles/9492014-daily-loss-guard", accessed: "2026-10-23" }
  - { field: copy_trading_allowed, url: "https://help.alpha-futures.com/en/articles/10573989-copy-trading", accessed: "2026-10-23" }
  - { field: copy_trading_allowed (conflict), url: "https://alpha-futures.com/terms-and-conditions", accessed: "2026-10-23" }
  - { field: automated_trading_allowed, url: "https://help.alpha-futures.com/en/articles/9508585-prohibited-trading-practices", accessed: "2026-10-23" }
  - { field: vps_allowed, url: "https://help.alpha-futures.com/en/articles/9508585-prohibited-trading-practices", accessed: "2026-10-23" }
  - { field: platforms, url: "https://help.alpha-futures.com/en/articles/10525305-trading-platforms", accessed: "2026-10-23" }
```

```yaml
firm: alpha-futures
plan: "Zero"
phase: evaluation
size_label: "100K"
size_usd: "100000.00"
instruments: [MNQ, NQ]
platforms: [AlphaTrader, "TradingView (Plus500 connection)", WealthCharts, "DeepChart (Volumetrica Trading)", "Quantower (dxFeed / Volumetrica Trading)"]
drawdown: { amount_usd: "3000.00", type: trailing_eod, stops_trailing_at: "Maximum Loss Limit stops trailing at the account starting balance on all of our accounts." }
daily_loss_limit: { amount_usd: "2000.00", kind: soft_pause }
profit_target_usd: "6000.00"
consistency_rule: { text: "No Consistency Rule on Zero Evaluations", threshold_pct: null }
max_position: { minis: 6, micros: 60, scaling: null }
min_trading_days: 1
trading_day_reset: { time: "18:00", tz_as_published: "ET" }
copy_trading_allowed: conditional
automated_trading_allowed: no
automation_rule_text: "The use of AI, bots, and other automated trading mechanisms is strictly prohibited across all account types."
vps_allowed: conditional
personal_device_required: unknown   # not mentioned on any official page (gap G12)
status: partial
not_confirmed_fields: ["drawdown.amount_usd (T&C 4% vs $3,000)", "copy_trading_allowed (scope)", "daily_loss_limit (T&C: Qualified only)"]
sources:
  - { field: profit_target_usd, url: "https://help.alpha-futures.com/en/articles/11771813-zero-account-overview", accessed: "2026-10-23" }
  - { field: drawdown.amount_usd, url: "https://help.alpha-futures.com/en/articles/11771813-zero-account-overview", accessed: "2026-10-23" }
  - { field: daily_loss_limit, url: "https://help.alpha-futures.com/en/articles/11771813-zero-account-overview", accessed: "2026-10-23" }
  - { field: max_position, url: "https://help.alpha-futures.com/en/articles/11771813-zero-account-overview", accessed: "2026-10-23" }
  - { field: consistency_rule, url: "https://help.alpha-futures.com/en/articles/11771813-zero-account-overview", accessed: "2026-10-23" }
  - { field: min_trading_days, url: "https://alpha-futures.com/product/zero", accessed: "2026-10-23" }
  - { field: drawdown.type, url: "https://help.alpha-futures.com/en/articles/9491999-maximum-loss-limit-mll", accessed: "2026-10-23" }
  - { field: daily_loss_limit.kind, url: "https://help.alpha-futures.com/en/articles/9492014-daily-loss-guard", accessed: "2026-10-23" }
  - { field: trading_day_reset, url: "https://help.alpha-futures.com/en/articles/9492014-daily-loss-guard", accessed: "2026-10-23" }
  - { field: copy_trading_allowed, url: "https://help.alpha-futures.com/en/articles/10573989-copy-trading", accessed: "2026-10-23" }
  - { field: copy_trading_allowed (conflict), url: "https://alpha-futures.com/terms-and-conditions", accessed: "2026-10-23" }
  - { field: automated_trading_allowed, url: "https://help.alpha-futures.com/en/articles/9508585-prohibited-trading-practices", accessed: "2026-10-23" }
  - { field: vps_allowed, url: "https://help.alpha-futures.com/en/articles/9508585-prohibited-trading-practices", accessed: "2026-10-23" }
  - { field: platforms, url: "https://help.alpha-futures.com/en/articles/10525305-trading-platforms", accessed: "2026-10-23" }
```

```yaml
# Qualified Account (Alpha Futures' simulated funded stage)
firm: alpha-futures
plan: "Zero"
phase: funded
size_label: "25K"
size_usd: "25000.00"
instruments: [MNQ, NQ]
platforms: [AlphaTrader, "TradingView (Plus500 connection)", WealthCharts, "DeepChart (Volumetrica Trading)", "Quantower (dxFeed / Volumetrica Trading)"]
drawdown: { amount_usd: "1000.00", type: trailing_eod, stops_trailing_at: "Maximum Loss Limit stops trailing at the account starting balance on all of our accounts." }
daily_loss_limit: { amount_usd: "500.00", kind: soft_pause }
profit_target_usd: null
consistency_rule: { text: "profits from any single trading day cannot be greater than or equal to 40% of net profits accumulated since last withdrawal request.", threshold_pct: "40" }
max_position: { minis: 1, micros: 10, scaling: null }
min_trading_days: 5
trading_day_reset: { time: "18:00", tz_as_published: "ET" }
copy_trading_allowed: conditional
automated_trading_allowed: no
automation_rule_text: "The use of AI, bots, and other automated trading mechanisms is strictly prohibited across all account types."
vps_allowed: conditional
personal_device_required: unknown   # not mentioned on any official page (gap G12)
status: partial
not_confirmed_fields: ["copy_trading_allowed (scope)", "T&C omits 25K"]
sources:
  - { field: drawdown.amount_usd, url: "https://help.alpha-futures.com/en/articles/11771813-zero-account-overview", accessed: "2026-10-23" }
  - { field: daily_loss_limit, url: "https://help.alpha-futures.com/en/articles/11771813-zero-account-overview", accessed: "2026-10-23" }
  - { field: max_position, url: "https://help.alpha-futures.com/en/articles/11771813-zero-account-overview", accessed: "2026-10-23" }
  - { field: scaling, url: "https://help.alpha-futures.com/en/articles/9492025-scaling-plan", accessed: "2026-10-23" }
  - { field: consistency_rule, url: "https://help.alpha-futures.com/en/articles/9492048-consistency-rule", accessed: "2026-10-23" }
  - { field: min_trading_days, url: "https://alpha-futures.com/product/zero", accessed: "2026-10-23" }
  - { field: max_position (T&C), url: "https://alpha-futures.com/terms-and-conditions", accessed: "2026-10-23" }
  - { field: drawdown.type, url: "https://help.alpha-futures.com/en/articles/9491999-maximum-loss-limit-mll", accessed: "2026-10-23" }
  - { field: daily_loss_limit.kind, url: "https://help.alpha-futures.com/en/articles/9492014-daily-loss-guard", accessed: "2026-10-23" }
  - { field: trading_day_reset, url: "https://help.alpha-futures.com/en/articles/9492014-daily-loss-guard", accessed: "2026-10-23" }
  - { field: copy_trading_allowed, url: "https://help.alpha-futures.com/en/articles/10573989-copy-trading", accessed: "2026-10-23" }
  - { field: copy_trading_allowed (conflict), url: "https://alpha-futures.com/terms-and-conditions", accessed: "2026-10-23" }
  - { field: automated_trading_allowed, url: "https://help.alpha-futures.com/en/articles/9508585-prohibited-trading-practices", accessed: "2026-10-23" }
  - { field: vps_allowed, url: "https://help.alpha-futures.com/en/articles/9508585-prohibited-trading-practices", accessed: "2026-10-23" }
  - { field: platforms, url: "https://help.alpha-futures.com/en/articles/10525305-trading-platforms", accessed: "2026-10-23" }
```

```yaml
# Qualified Account (Alpha Futures' simulated funded stage)
firm: alpha-futures
plan: "Zero"
phase: funded
size_label: "50K"
size_usd: "50000.00"
instruments: [MNQ, NQ]
platforms: [AlphaTrader, "TradingView (Plus500 connection)", WealthCharts, "DeepChart (Volumetrica Trading)", "Quantower (dxFeed / Volumetrica Trading)"]
drawdown: { amount_usd: "2000.00", type: trailing_eod, stops_trailing_at: "Maximum Loss Limit stops trailing at the account starting balance on all of our accounts." }
daily_loss_limit: { amount_usd: "1000.00", kind: soft_pause }
profit_target_usd: null
consistency_rule: { text: "profits from any single trading day cannot be greater than or equal to 40% of net profits accumulated since last withdrawal request.", threshold_pct: "40" }
max_position: { minis: 3, micros: 30, scaling: "Zero Qualified 50k (minis/micros by profit): Less than $1,500 profit 1/10; $1,500-2,000 profit 2/20; $2,000-3,000 profit 3/30" }
min_trading_days: 5
trading_day_reset: { time: "18:00", tz_as_published: "ET" }
copy_trading_allowed: conditional
automated_trading_allowed: no
automation_rule_text: "The use of AI, bots, and other automated trading mechanisms is strictly prohibited across all account types."
vps_allowed: conditional
personal_device_required: unknown   # not mentioned on any official page (gap G12)
status: partial
not_confirmed_fields: ["copy_trading_allowed (scope)"]
sources:
  - { field: drawdown.amount_usd, url: "https://help.alpha-futures.com/en/articles/11771813-zero-account-overview", accessed: "2026-10-23" }
  - { field: daily_loss_limit, url: "https://help.alpha-futures.com/en/articles/11771813-zero-account-overview", accessed: "2026-10-23" }
  - { field: max_position, url: "https://help.alpha-futures.com/en/articles/11771813-zero-account-overview", accessed: "2026-10-23" }
  - { field: scaling, url: "https://help.alpha-futures.com/en/articles/9492025-scaling-plan", accessed: "2026-10-23" }
  - { field: consistency_rule, url: "https://help.alpha-futures.com/en/articles/9492048-consistency-rule", accessed: "2026-10-23" }
  - { field: min_trading_days, url: "https://alpha-futures.com/product/zero", accessed: "2026-10-23" }
  - { field: max_position (T&C), url: "https://alpha-futures.com/terms-and-conditions", accessed: "2026-10-23" }
  - { field: drawdown.type, url: "https://help.alpha-futures.com/en/articles/9491999-maximum-loss-limit-mll", accessed: "2026-10-23" }
  - { field: daily_loss_limit.kind, url: "https://help.alpha-futures.com/en/articles/9492014-daily-loss-guard", accessed: "2026-10-23" }
  - { field: trading_day_reset, url: "https://help.alpha-futures.com/en/articles/9492014-daily-loss-guard", accessed: "2026-10-23" }
  - { field: copy_trading_allowed, url: "https://help.alpha-futures.com/en/articles/10573989-copy-trading", accessed: "2026-10-23" }
  - { field: copy_trading_allowed (conflict), url: "https://alpha-futures.com/terms-and-conditions", accessed: "2026-10-23" }
  - { field: automated_trading_allowed, url: "https://help.alpha-futures.com/en/articles/9508585-prohibited-trading-practices", accessed: "2026-10-23" }
  - { field: vps_allowed, url: "https://help.alpha-futures.com/en/articles/9508585-prohibited-trading-practices", accessed: "2026-10-23" }
  - { field: platforms, url: "https://help.alpha-futures.com/en/articles/10525305-trading-platforms", accessed: "2026-10-23" }
```

```yaml
# Qualified Account (Alpha Futures' simulated funded stage)
firm: alpha-futures
plan: "Zero"
phase: funded
size_label: "100K"
size_usd: "100000.00"
instruments: [MNQ, NQ]
platforms: [AlphaTrader, "TradingView (Plus500 connection)", WealthCharts, "DeepChart (Volumetrica Trading)", "Quantower (dxFeed / Volumetrica Trading)"]
drawdown: { amount_usd: "3000.00", type: trailing_eod, stops_trailing_at: "Maximum Loss Limit stops trailing at the account starting balance on all of our accounts." }
daily_loss_limit: { amount_usd: "2000.00", kind: soft_pause }
profit_target_usd: null
consistency_rule: { text: "profits from any single trading day cannot be greater than or equal to 40% of net profits accumulated since last withdrawal request.", threshold_pct: "40" }
max_position: { minis: 6, micros: 60, scaling: "Zero Qualified 100k (minis/micros by profit): Less than $1,500 profit 2/20; $1,500-2,000 profit 3/30; $2,000-3,000 profit 4/40; $3,000-4,500 profit 5/50; $4,500+ profit 6/60" }
min_trading_days: 5
trading_day_reset: { time: "18:00", tz_as_published: "ET" }
copy_trading_allowed: conditional
automated_trading_allowed: no
automation_rule_text: "The use of AI, bots, and other automated trading mechanisms is strictly prohibited across all account types."
vps_allowed: conditional
personal_device_required: unknown   # not mentioned on any official page (gap G12)
status: partial
not_confirmed_fields: ["drawdown.amount_usd (T&C 4% vs $3,000)", "copy_trading_allowed (scope)"]
sources:
  - { field: drawdown.amount_usd, url: "https://help.alpha-futures.com/en/articles/11771813-zero-account-overview", accessed: "2026-10-23" }
  - { field: daily_loss_limit, url: "https://help.alpha-futures.com/en/articles/11771813-zero-account-overview", accessed: "2026-10-23" }
  - { field: max_position, url: "https://help.alpha-futures.com/en/articles/11771813-zero-account-overview", accessed: "2026-10-23" }
  - { field: scaling, url: "https://help.alpha-futures.com/en/articles/9492025-scaling-plan", accessed: "2026-10-23" }
  - { field: consistency_rule, url: "https://help.alpha-futures.com/en/articles/9492048-consistency-rule", accessed: "2026-10-23" }
  - { field: min_trading_days, url: "https://alpha-futures.com/product/zero", accessed: "2026-10-23" }
  - { field: max_position (T&C), url: "https://alpha-futures.com/terms-and-conditions", accessed: "2026-10-23" }
  - { field: drawdown.type, url: "https://help.alpha-futures.com/en/articles/9491999-maximum-loss-limit-mll", accessed: "2026-10-23" }
  - { field: daily_loss_limit.kind, url: "https://help.alpha-futures.com/en/articles/9492014-daily-loss-guard", accessed: "2026-10-23" }
  - { field: trading_day_reset, url: "https://help.alpha-futures.com/en/articles/9492014-daily-loss-guard", accessed: "2026-10-23" }
  - { field: copy_trading_allowed, url: "https://help.alpha-futures.com/en/articles/10573989-copy-trading", accessed: "2026-10-23" }
  - { field: copy_trading_allowed (conflict), url: "https://alpha-futures.com/terms-and-conditions", accessed: "2026-10-23" }
  - { field: automated_trading_allowed, url: "https://help.alpha-futures.com/en/articles/9508585-prohibited-trading-practices", accessed: "2026-10-23" }
  - { field: vps_allowed, url: "https://help.alpha-futures.com/en/articles/9508585-prohibited-trading-practices", accessed: "2026-10-23" }
  - { field: platforms, url: "https://help.alpha-futures.com/en/articles/10525305-trading-platforms", accessed: "2026-10-23" }
```

```yaml
firm: alpha-futures
plan: "Standard"
phase: evaluation
size_label: "50K"
size_usd: "50000.00"
instruments: [MNQ, NQ]
platforms: [AlphaTrader, "TradingView (Plus500 connection)", WealthCharts, "DeepChart (Volumetrica Trading)", "Quantower (dxFeed / Volumetrica Trading)"]
drawdown: { amount_usd: "2000.00", type: trailing_eod, stops_trailing_at: "Maximum Loss Limit stops trailing at the account starting balance on all of our accounts." }
daily_loss_limit: { amount_usd: null, kind: none }
profit_target_usd: "3000.00"
consistency_rule: { text: "The Consistency Rule on Standard Evaluations is 50%. This means that during your Evaluation, profits from one single trading day cannot be larger than 50% of your net profits made", threshold_pct: "50" }
max_position: { minis: 5, micros: 50, scaling: null }
min_trading_days: 2
trading_day_reset: { time: "18:00", tz_as_published: "ET" }
copy_trading_allowed: conditional
automated_trading_allowed: no
automation_rule_text: "The use of AI, bots, and other automated trading mechanisms is strictly prohibited across all account types."
vps_allowed: conditional
personal_device_required: unknown   # not mentioned on any official page (gap G12)
status: partial
not_confirmed_fields: ["copy_trading_allowed (scope)"]
sources:
  - { field: profit_target_usd, url: "https://help.alpha-futures.com/en/articles/11632512-standard-account-overview", accessed: "2026-10-23" }
  - { field: drawdown.amount_usd, url: "https://help.alpha-futures.com/en/articles/11632512-standard-account-overview", accessed: "2026-10-23" }
  - { field: daily_loss_limit, url: "https://help.alpha-futures.com/en/articles/11632512-standard-account-overview", accessed: "2026-10-23" }
  - { field: max_position, url: "https://help.alpha-futures.com/en/articles/11632512-standard-account-overview", accessed: "2026-10-23" }
  - { field: consistency_rule, url: "https://help.alpha-futures.com/en/articles/9492048-consistency-rule", accessed: "2026-10-23" }
  - { field: min_trading_days, url: "https://alpha-futures.com/terms-and-conditions", accessed: "2026-10-23" }
  - { field: drawdown.type, url: "https://help.alpha-futures.com/en/articles/9491999-maximum-loss-limit-mll", accessed: "2026-10-23" }
  - { field: daily_loss_limit.kind, url: "https://help.alpha-futures.com/en/articles/9492014-daily-loss-guard", accessed: "2026-10-23" }
  - { field: trading_day_reset, url: "https://help.alpha-futures.com/en/articles/9492014-daily-loss-guard", accessed: "2026-10-23" }
  - { field: copy_trading_allowed, url: "https://help.alpha-futures.com/en/articles/10573989-copy-trading", accessed: "2026-10-23" }
  - { field: copy_trading_allowed (conflict), url: "https://alpha-futures.com/terms-and-conditions", accessed: "2026-10-23" }
  - { field: automated_trading_allowed, url: "https://help.alpha-futures.com/en/articles/9508585-prohibited-trading-practices", accessed: "2026-10-23" }
  - { field: vps_allowed, url: "https://help.alpha-futures.com/en/articles/9508585-prohibited-trading-practices", accessed: "2026-10-23" }
  - { field: platforms, url: "https://help.alpha-futures.com/en/articles/10525305-trading-platforms", accessed: "2026-10-23" }
```

```yaml
firm: alpha-futures
plan: "Standard"
phase: evaluation
size_label: "100K"
size_usd: "100000.00"
instruments: [MNQ, NQ]
platforms: [AlphaTrader, "TradingView (Plus500 connection)", WealthCharts, "DeepChart (Volumetrica Trading)", "Quantower (dxFeed / Volumetrica Trading)"]
drawdown: { amount_usd: "3000.00", type: trailing_eod, stops_trailing_at: "Maximum Loss Limit stops trailing at the account starting balance on all of our accounts." }
daily_loss_limit: { amount_usd: null, kind: none }
profit_target_usd: "6000.00"
consistency_rule: { text: "The Consistency Rule on Standard Evaluations is 50%. This means that during your Evaluation, profits from one single trading day cannot be larger than 50% of your net profits made", threshold_pct: "50" }
max_position: { minis: 10, micros: 100, scaling: null }
min_trading_days: 2
trading_day_reset: { time: "18:00", tz_as_published: "ET" }
copy_trading_allowed: conditional
automated_trading_allowed: no
automation_rule_text: "The use of AI, bots, and other automated trading mechanisms is strictly prohibited across all account types."
vps_allowed: conditional
personal_device_required: unknown   # not mentioned on any official page (gap G12)
status: partial
not_confirmed_fields: ["copy_trading_allowed (scope)", "max_position.micros (T&C 8.1.10: 10 micros)"]
sources:
  - { field: profit_target_usd, url: "https://help.alpha-futures.com/en/articles/11632512-standard-account-overview", accessed: "2026-10-23" }
  - { field: drawdown.amount_usd, url: "https://help.alpha-futures.com/en/articles/11632512-standard-account-overview", accessed: "2026-10-23" }
  - { field: daily_loss_limit, url: "https://help.alpha-futures.com/en/articles/11632512-standard-account-overview", accessed: "2026-10-23" }
  - { field: max_position, url: "https://help.alpha-futures.com/en/articles/11632512-standard-account-overview", accessed: "2026-10-23" }
  - { field: consistency_rule, url: "https://help.alpha-futures.com/en/articles/9492048-consistency-rule", accessed: "2026-10-23" }
  - { field: min_trading_days, url: "https://alpha-futures.com/terms-and-conditions", accessed: "2026-10-23" }
  - { field: drawdown.type, url: "https://help.alpha-futures.com/en/articles/9491999-maximum-loss-limit-mll", accessed: "2026-10-23" }
  - { field: daily_loss_limit.kind, url: "https://help.alpha-futures.com/en/articles/9492014-daily-loss-guard", accessed: "2026-10-23" }
  - { field: trading_day_reset, url: "https://help.alpha-futures.com/en/articles/9492014-daily-loss-guard", accessed: "2026-10-23" }
  - { field: copy_trading_allowed, url: "https://help.alpha-futures.com/en/articles/10573989-copy-trading", accessed: "2026-10-23" }
  - { field: copy_trading_allowed (conflict), url: "https://alpha-futures.com/terms-and-conditions", accessed: "2026-10-23" }
  - { field: automated_trading_allowed, url: "https://help.alpha-futures.com/en/articles/9508585-prohibited-trading-practices", accessed: "2026-10-23" }
  - { field: vps_allowed, url: "https://help.alpha-futures.com/en/articles/9508585-prohibited-trading-practices", accessed: "2026-10-23" }
  - { field: platforms, url: "https://help.alpha-futures.com/en/articles/10525305-trading-platforms", accessed: "2026-10-23" }
```

```yaml
firm: alpha-futures
plan: "Standard"
phase: evaluation
size_label: "150K"
size_usd: "150000.00"
instruments: [MNQ, NQ]
platforms: [AlphaTrader, "TradingView (Plus500 connection)", WealthCharts, "DeepChart (Volumetrica Trading)", "Quantower (dxFeed / Volumetrica Trading)"]
drawdown: { amount_usd: "4500.00", type: trailing_eod, stops_trailing_at: "Maximum Loss Limit stops trailing at the account starting balance on all of our accounts." }
daily_loss_limit: { amount_usd: null, kind: none }
profit_target_usd: "9000.00"
consistency_rule: { text: "The Consistency Rule on Standard Evaluations is 50%. This means that during your Evaluation, profits from one single trading day cannot be larger than 50% of your net profits made", threshold_pct: "50" }
max_position: { minis: 15, micros: 150, scaling: null }
min_trading_days: 2
trading_day_reset: { time: "18:00", tz_as_published: "ET" }
copy_trading_allowed: conditional
automated_trading_allowed: no
automation_rule_text: "The use of AI, bots, and other automated trading mechanisms is strictly prohibited across all account types."
vps_allowed: conditional
personal_device_required: unknown   # not mentioned on any official page (gap G12)
status: partial
not_confirmed_fields: ["copy_trading_allowed (scope)"]
sources:
  - { field: profit_target_usd, url: "https://help.alpha-futures.com/en/articles/11632512-standard-account-overview", accessed: "2026-10-23" }
  - { field: drawdown.amount_usd, url: "https://help.alpha-futures.com/en/articles/11632512-standard-account-overview", accessed: "2026-10-23" }
  - { field: daily_loss_limit, url: "https://help.alpha-futures.com/en/articles/11632512-standard-account-overview", accessed: "2026-10-23" }
  - { field: max_position, url: "https://help.alpha-futures.com/en/articles/11632512-standard-account-overview", accessed: "2026-10-23" }
  - { field: consistency_rule, url: "https://help.alpha-futures.com/en/articles/9492048-consistency-rule", accessed: "2026-10-23" }
  - { field: min_trading_days, url: "https://alpha-futures.com/terms-and-conditions", accessed: "2026-10-23" }
  - { field: drawdown.type, url: "https://help.alpha-futures.com/en/articles/9491999-maximum-loss-limit-mll", accessed: "2026-10-23" }
  - { field: daily_loss_limit.kind, url: "https://help.alpha-futures.com/en/articles/9492014-daily-loss-guard", accessed: "2026-10-23" }
  - { field: trading_day_reset, url: "https://help.alpha-futures.com/en/articles/9492014-daily-loss-guard", accessed: "2026-10-23" }
  - { field: copy_trading_allowed, url: "https://help.alpha-futures.com/en/articles/10573989-copy-trading", accessed: "2026-10-23" }
  - { field: copy_trading_allowed (conflict), url: "https://alpha-futures.com/terms-and-conditions", accessed: "2026-10-23" }
  - { field: automated_trading_allowed, url: "https://help.alpha-futures.com/en/articles/9508585-prohibited-trading-practices", accessed: "2026-10-23" }
  - { field: vps_allowed, url: "https://help.alpha-futures.com/en/articles/9508585-prohibited-trading-practices", accessed: "2026-10-23" }
  - { field: platforms, url: "https://help.alpha-futures.com/en/articles/10525305-trading-platforms", accessed: "2026-10-23" }
```

```yaml
# Qualified Account (simulated funded stage)
firm: alpha-futures
plan: "Standard"
phase: funded
size_label: "50K"
size_usd: "50000.00"
instruments: [MNQ, NQ]
platforms: [AlphaTrader, "TradingView (Plus500 connection)", WealthCharts, "DeepChart (Volumetrica Trading)", "Quantower (dxFeed / Volumetrica Trading)"]
drawdown: { amount_usd: "2000.00", type: trailing_eod, stops_trailing_at: "Maximum Loss Limit stops trailing at the account starting balance on all of our accounts." }
daily_loss_limit: { amount_usd: "1000.00", kind: soft_pause }
profit_target_usd: null
consistency_rule: { text: "profits from any single trading day cannot be greater than or equal to 40% of net profits accumulated since last withdrawal request.", threshold_pct: "40" }
max_position: { minis: 5, micros: 50, scaling: "Standard Qualified 50k (minis/micros by profit): Less than $1,500 profit 2/20; $1,500-2,000 profit 3/30; $2,000-3,000 profit 5/50; $3,000-4,500 profit 5/50; $4,500+ profit 5/50" }
min_trading_days: 5
trading_day_reset: { time: "18:00", tz_as_published: "ET" }
copy_trading_allowed: conditional
automated_trading_allowed: no
automation_rule_text: "The use of AI, bots, and other automated trading mechanisms is strictly prohibited across all account types."
vps_allowed: conditional
personal_device_required: unknown   # not mentioned on any official page (gap G12)
status: partial
not_confirmed_fields: ["copy_trading_allowed (scope)", "max_accounts (3 vs 5)"]
sources:
  - { field: drawdown.amount_usd, url: "https://help.alpha-futures.com/en/articles/11632512-standard-account-overview", accessed: "2026-10-23" }
  - { field: daily_loss_limit, url: "https://help.alpha-futures.com/en/articles/11632512-standard-account-overview", accessed: "2026-10-23" }
  - { field: max_position, url: "https://help.alpha-futures.com/en/articles/11632512-standard-account-overview", accessed: "2026-10-23" }
  - { field: scaling, url: "https://help.alpha-futures.com/en/articles/9492025-scaling-plan", accessed: "2026-10-23" }
  - { field: consistency_rule, url: "https://help.alpha-futures.com/en/articles/9492048-consistency-rule", accessed: "2026-10-23" }
  - { field: min_trading_days, url: "https://alpha-futures.com/product/standard", accessed: "2026-10-23" }
  - { field: drawdown.type, url: "https://help.alpha-futures.com/en/articles/9491999-maximum-loss-limit-mll", accessed: "2026-10-23" }
  - { field: daily_loss_limit.kind, url: "https://help.alpha-futures.com/en/articles/9492014-daily-loss-guard", accessed: "2026-10-23" }
  - { field: trading_day_reset, url: "https://help.alpha-futures.com/en/articles/9492014-daily-loss-guard", accessed: "2026-10-23" }
  - { field: copy_trading_allowed, url: "https://help.alpha-futures.com/en/articles/10573989-copy-trading", accessed: "2026-10-23" }
  - { field: copy_trading_allowed (conflict), url: "https://alpha-futures.com/terms-and-conditions", accessed: "2026-10-23" }
  - { field: automated_trading_allowed, url: "https://help.alpha-futures.com/en/articles/9508585-prohibited-trading-practices", accessed: "2026-10-23" }
  - { field: vps_allowed, url: "https://help.alpha-futures.com/en/articles/9508585-prohibited-trading-practices", accessed: "2026-10-23" }
  - { field: platforms, url: "https://help.alpha-futures.com/en/articles/10525305-trading-platforms", accessed: "2026-10-23" }
```

```yaml
# Qualified Account (simulated funded stage)
firm: alpha-futures
plan: "Standard"
phase: funded
size_label: "100K"
size_usd: "100000.00"
instruments: [MNQ, NQ]
platforms: [AlphaTrader, "TradingView (Plus500 connection)", WealthCharts, "DeepChart (Volumetrica Trading)", "Quantower (dxFeed / Volumetrica Trading)"]
drawdown: { amount_usd: "3000.00", type: trailing_eod, stops_trailing_at: "Maximum Loss Limit stops trailing at the account starting balance on all of our accounts." }
daily_loss_limit: { amount_usd: "2000.00", kind: soft_pause }
profit_target_usd: null
consistency_rule: { text: "profits from any single trading day cannot be greater than or equal to 40% of net profits accumulated since last withdrawal request.", threshold_pct: "40" }
max_position: { minis: 10, micros: 100, scaling: "Standard Qualified 100k (minis/micros by profit): Less than $1,500 profit 3/30; $1,500-2,000 profit 4/40; $2,000-3,000 profit 5/50; $3,000-4,500 profit 10/100; $4,500+ profit 10/100" }
min_trading_days: 5
trading_day_reset: { time: "18:00", tz_as_published: "ET" }
copy_trading_allowed: conditional
automated_trading_allowed: no
automation_rule_text: "The use of AI, bots, and other automated trading mechanisms is strictly prohibited across all account types."
vps_allowed: conditional
personal_device_required: unknown   # not mentioned on any official page (gap G12)
status: partial
not_confirmed_fields: ["copy_trading_allowed (scope)", "max_accounts (3 vs 5)", "max_position.micros (T&C 8.1.10: 10 micros)"]
sources:
  - { field: drawdown.amount_usd, url: "https://help.alpha-futures.com/en/articles/11632512-standard-account-overview", accessed: "2026-10-23" }
  - { field: daily_loss_limit, url: "https://help.alpha-futures.com/en/articles/11632512-standard-account-overview", accessed: "2026-10-23" }
  - { field: max_position, url: "https://help.alpha-futures.com/en/articles/11632512-standard-account-overview", accessed: "2026-10-23" }
  - { field: scaling, url: "https://help.alpha-futures.com/en/articles/9492025-scaling-plan", accessed: "2026-10-23" }
  - { field: consistency_rule, url: "https://help.alpha-futures.com/en/articles/9492048-consistency-rule", accessed: "2026-10-23" }
  - { field: min_trading_days, url: "https://alpha-futures.com/product/standard", accessed: "2026-10-23" }
  - { field: drawdown.type, url: "https://help.alpha-futures.com/en/articles/9491999-maximum-loss-limit-mll", accessed: "2026-10-23" }
  - { field: daily_loss_limit.kind, url: "https://help.alpha-futures.com/en/articles/9492014-daily-loss-guard", accessed: "2026-10-23" }
  - { field: trading_day_reset, url: "https://help.alpha-futures.com/en/articles/9492014-daily-loss-guard", accessed: "2026-10-23" }
  - { field: copy_trading_allowed, url: "https://help.alpha-futures.com/en/articles/10573989-copy-trading", accessed: "2026-10-23" }
  - { field: copy_trading_allowed (conflict), url: "https://alpha-futures.com/terms-and-conditions", accessed: "2026-10-23" }
  - { field: automated_trading_allowed, url: "https://help.alpha-futures.com/en/articles/9508585-prohibited-trading-practices", accessed: "2026-10-23" }
  - { field: vps_allowed, url: "https://help.alpha-futures.com/en/articles/9508585-prohibited-trading-practices", accessed: "2026-10-23" }
  - { field: platforms, url: "https://help.alpha-futures.com/en/articles/10525305-trading-platforms", accessed: "2026-10-23" }
```

```yaml
# Qualified Account (simulated funded stage)
firm: alpha-futures
plan: "Standard"
phase: funded
size_label: "150K"
size_usd: "150000.00"
instruments: [MNQ, NQ]
platforms: [AlphaTrader, "TradingView (Plus500 connection)", WealthCharts, "DeepChart (Volumetrica Trading)", "Quantower (dxFeed / Volumetrica Trading)"]
drawdown: { amount_usd: "4500.00", type: trailing_eod, stops_trailing_at: "Maximum Loss Limit stops trailing at the account starting balance on all of our accounts." }
daily_loss_limit: { amount_usd: "3000.00", kind: soft_pause }
profit_target_usd: null
consistency_rule: { text: "profits from any single trading day cannot be greater than or equal to 40% of net profits accumulated since last withdrawal request.", threshold_pct: "40" }
max_position: { minis: 15, micros: 150, scaling: "Standard Qualified 150k (minis/micros by profit): Less than $1,500 profit 3/30; $1,500-2,000 profit 4/40; $2,000-3,000 profit 5/50; $3,000-4,500 profit 10/100; $4,500+ profit 15/150" }
min_trading_days: 5
trading_day_reset: { time: "18:00", tz_as_published: "ET" }
copy_trading_allowed: conditional
automated_trading_allowed: no
automation_rule_text: "The use of AI, bots, and other automated trading mechanisms is strictly prohibited across all account types."
vps_allowed: conditional
personal_device_required: unknown   # not mentioned on any official page (gap G12)
status: partial
not_confirmed_fields: ["copy_trading_allowed (scope)", "max_accounts (3 vs 5)"]
sources:
  - { field: drawdown.amount_usd, url: "https://help.alpha-futures.com/en/articles/11632512-standard-account-overview", accessed: "2026-10-23" }
  - { field: daily_loss_limit, url: "https://help.alpha-futures.com/en/articles/11632512-standard-account-overview", accessed: "2026-10-23" }
  - { field: max_position, url: "https://help.alpha-futures.com/en/articles/11632512-standard-account-overview", accessed: "2026-10-23" }
  - { field: scaling, url: "https://help.alpha-futures.com/en/articles/9492025-scaling-plan", accessed: "2026-10-23" }
  - { field: consistency_rule, url: "https://help.alpha-futures.com/en/articles/9492048-consistency-rule", accessed: "2026-10-23" }
  - { field: min_trading_days, url: "https://alpha-futures.com/product/standard", accessed: "2026-10-23" }
  - { field: drawdown.type, url: "https://help.alpha-futures.com/en/articles/9491999-maximum-loss-limit-mll", accessed: "2026-10-23" }
  - { field: daily_loss_limit.kind, url: "https://help.alpha-futures.com/en/articles/9492014-daily-loss-guard", accessed: "2026-10-23" }
  - { field: trading_day_reset, url: "https://help.alpha-futures.com/en/articles/9492014-daily-loss-guard", accessed: "2026-10-23" }
  - { field: copy_trading_allowed, url: "https://help.alpha-futures.com/en/articles/10573989-copy-trading", accessed: "2026-10-23" }
  - { field: copy_trading_allowed (conflict), url: "https://alpha-futures.com/terms-and-conditions", accessed: "2026-10-23" }
  - { field: automated_trading_allowed, url: "https://help.alpha-futures.com/en/articles/9508585-prohibited-trading-practices", accessed: "2026-10-23" }
  - { field: vps_allowed, url: "https://help.alpha-futures.com/en/articles/9508585-prohibited-trading-practices", accessed: "2026-10-23" }
  - { field: platforms, url: "https://help.alpha-futures.com/en/articles/10525305-trading-platforms", accessed: "2026-10-23" }
```

```yaml
firm: alpha-futures
plan: "Advanced"
phase: evaluation
size_label: "50K"
size_usd: "50000.00"
instruments: [MNQ, NQ]
platforms: [AlphaTrader, "TradingView (Plus500 connection)", WealthCharts, "DeepChart (Volumetrica Trading)", "Quantower (dxFeed / Volumetrica Trading)"]
drawdown: { amount_usd: "1750.00", type: trailing_eod, stops_trailing_at: "Maximum Loss Limit stops trailing at the account starting balance on all of our accounts." }
daily_loss_limit: { amount_usd: null, kind: none }
profit_target_usd: "4000.00"
consistency_rule: { text: "The Consistency Rule on Advanced Evaluations is 40%. This means that during your Evaluation, profits from one single trading day cannot be larger than 40% of your net profits made.", threshold_pct: "40" }
max_position: { minis: 5, micros: 50, scaling: null }
min_trading_days: 3   # conflict: T&C Schedule 1 says 2
trading_day_reset: { time: "18:00", tz_as_published: "ET" }
copy_trading_allowed: conditional
automated_trading_allowed: no
automation_rule_text: "The use of AI, bots, and other automated trading mechanisms is strictly prohibited across all account types."
vps_allowed: conditional
personal_device_required: unknown   # not mentioned on any official page (gap G12)
status: partial
not_confirmed_fields: ["min_trading_days (3 vs 2)", "copy_trading_allowed (scope)"]
sources:
  - { field: profit_target_usd, url: "https://help.alpha-futures.com/en/articles/11634907-advanced-account-overview", accessed: "2026-10-23" }
  - { field: drawdown.amount_usd, url: "https://help.alpha-futures.com/en/articles/11634907-advanced-account-overview", accessed: "2026-10-23" }
  - { field: daily_loss_limit, url: "https://help.alpha-futures.com/en/articles/11634907-advanced-account-overview", accessed: "2026-10-23" }
  - { field: max_position, url: "https://help.alpha-futures.com/en/articles/11634907-advanced-account-overview", accessed: "2026-10-23" }
  - { field: consistency_rule, url: "https://help.alpha-futures.com/en/articles/9492048-consistency-rule", accessed: "2026-10-23" }
  - { field: min_trading_days, url: "https://alpha-futures.com/product/advanced", accessed: "2026-10-23" }
  - { field: min_trading_days (conflict), url: "https://alpha-futures.com/terms-and-conditions", accessed: "2026-10-23" }
  - { field: drawdown.type, url: "https://help.alpha-futures.com/en/articles/9491999-maximum-loss-limit-mll", accessed: "2026-10-23" }
  - { field: daily_loss_limit.kind, url: "https://help.alpha-futures.com/en/articles/9492014-daily-loss-guard", accessed: "2026-10-23" }
  - { field: trading_day_reset, url: "https://help.alpha-futures.com/en/articles/9492014-daily-loss-guard", accessed: "2026-10-23" }
  - { field: copy_trading_allowed, url: "https://help.alpha-futures.com/en/articles/10573989-copy-trading", accessed: "2026-10-23" }
  - { field: copy_trading_allowed (conflict), url: "https://alpha-futures.com/terms-and-conditions", accessed: "2026-10-23" }
  - { field: automated_trading_allowed, url: "https://help.alpha-futures.com/en/articles/9508585-prohibited-trading-practices", accessed: "2026-10-23" }
  - { field: vps_allowed, url: "https://help.alpha-futures.com/en/articles/9508585-prohibited-trading-practices", accessed: "2026-10-23" }
  - { field: platforms, url: "https://help.alpha-futures.com/en/articles/10525305-trading-platforms", accessed: "2026-10-23" }
```

```yaml
firm: alpha-futures
plan: "Advanced"
phase: evaluation
size_label: "100K"
size_usd: "100000.00"
instruments: [MNQ, NQ]
platforms: [AlphaTrader, "TradingView (Plus500 connection)", WealthCharts, "DeepChart (Volumetrica Trading)", "Quantower (dxFeed / Volumetrica Trading)"]
drawdown: { amount_usd: "3500.00", type: trailing_eod, stops_trailing_at: "Maximum Loss Limit stops trailing at the account starting balance on all of our accounts." }
daily_loss_limit: { amount_usd: null, kind: none }
profit_target_usd: "8000.00"
consistency_rule: { text: "The Consistency Rule on Advanced Evaluations is 40%. This means that during your Evaluation, profits from one single trading day cannot be larger than 40% of your net profits made.", threshold_pct: "40" }
max_position: { minis: 10, micros: 100, scaling: null }
min_trading_days: 3   # conflict: T&C Schedule 1 says 2
trading_day_reset: { time: "18:00", tz_as_published: "ET" }
copy_trading_allowed: conditional
automated_trading_allowed: no
automation_rule_text: "The use of AI, bots, and other automated trading mechanisms is strictly prohibited across all account types."
vps_allowed: conditional
personal_device_required: unknown   # not mentioned on any official page (gap G12)
status: partial
not_confirmed_fields: ["min_trading_days (3 vs 2)", "copy_trading_allowed (scope)"]
sources:
  - { field: profit_target_usd, url: "https://help.alpha-futures.com/en/articles/11634907-advanced-account-overview", accessed: "2026-10-23" }
  - { field: drawdown.amount_usd, url: "https://help.alpha-futures.com/en/articles/11634907-advanced-account-overview", accessed: "2026-10-23" }
  - { field: daily_loss_limit, url: "https://help.alpha-futures.com/en/articles/11634907-advanced-account-overview", accessed: "2026-10-23" }
  - { field: max_position, url: "https://help.alpha-futures.com/en/articles/11634907-advanced-account-overview", accessed: "2026-10-23" }
  - { field: consistency_rule, url: "https://help.alpha-futures.com/en/articles/9492048-consistency-rule", accessed: "2026-10-23" }
  - { field: min_trading_days, url: "https://alpha-futures.com/product/advanced", accessed: "2026-10-23" }
  - { field: min_trading_days (conflict), url: "https://alpha-futures.com/terms-and-conditions", accessed: "2026-10-23" }
  - { field: drawdown.type, url: "https://help.alpha-futures.com/en/articles/9491999-maximum-loss-limit-mll", accessed: "2026-10-23" }
  - { field: daily_loss_limit.kind, url: "https://help.alpha-futures.com/en/articles/9492014-daily-loss-guard", accessed: "2026-10-23" }
  - { field: trading_day_reset, url: "https://help.alpha-futures.com/en/articles/9492014-daily-loss-guard", accessed: "2026-10-23" }
  - { field: copy_trading_allowed, url: "https://help.alpha-futures.com/en/articles/10573989-copy-trading", accessed: "2026-10-23" }
  - { field: copy_trading_allowed (conflict), url: "https://alpha-futures.com/terms-and-conditions", accessed: "2026-10-23" }
  - { field: automated_trading_allowed, url: "https://help.alpha-futures.com/en/articles/9508585-prohibited-trading-practices", accessed: "2026-10-23" }
  - { field: vps_allowed, url: "https://help.alpha-futures.com/en/articles/9508585-prohibited-trading-practices", accessed: "2026-10-23" }
  - { field: platforms, url: "https://help.alpha-futures.com/en/articles/10525305-trading-platforms", accessed: "2026-10-23" }
```

```yaml
firm: alpha-futures
plan: "Advanced"
phase: evaluation
size_label: "150K"
size_usd: "150000.00"
instruments: [MNQ, NQ]
platforms: [AlphaTrader, "TradingView (Plus500 connection)", WealthCharts, "DeepChart (Volumetrica Trading)", "Quantower (dxFeed / Volumetrica Trading)"]
drawdown: { amount_usd: "5250.00", type: trailing_eod, stops_trailing_at: "Maximum Loss Limit stops trailing at the account starting balance on all of our accounts." }
daily_loss_limit: { amount_usd: null, kind: none }
profit_target_usd: "12000.00"
consistency_rule: { text: "The Consistency Rule on Advanced Evaluations is 40%. This means that during your Evaluation, profits from one single trading day cannot be larger than 40% of your net profits made.", threshold_pct: "40" }
max_position: { minis: 15, micros: 150, scaling: null }
min_trading_days: 3   # conflict: T&C Schedule 1 says 2
trading_day_reset: { time: "18:00", tz_as_published: "ET" }
copy_trading_allowed: conditional
automated_trading_allowed: no
automation_rule_text: "The use of AI, bots, and other automated trading mechanisms is strictly prohibited across all account types."
vps_allowed: conditional
personal_device_required: unknown   # not mentioned on any official page (gap G12)
status: partial
not_confirmed_fields: ["min_trading_days (3 vs 2)", "copy_trading_allowed (scope)"]
sources:
  - { field: profit_target_usd, url: "https://help.alpha-futures.com/en/articles/11634907-advanced-account-overview", accessed: "2026-10-23" }
  - { field: drawdown.amount_usd, url: "https://help.alpha-futures.com/en/articles/11634907-advanced-account-overview", accessed: "2026-10-23" }
  - { field: daily_loss_limit, url: "https://help.alpha-futures.com/en/articles/11634907-advanced-account-overview", accessed: "2026-10-23" }
  - { field: max_position, url: "https://help.alpha-futures.com/en/articles/11634907-advanced-account-overview", accessed: "2026-10-23" }
  - { field: consistency_rule, url: "https://help.alpha-futures.com/en/articles/9492048-consistency-rule", accessed: "2026-10-23" }
  - { field: min_trading_days, url: "https://alpha-futures.com/product/advanced", accessed: "2026-10-23" }
  - { field: min_trading_days (conflict), url: "https://alpha-futures.com/terms-and-conditions", accessed: "2026-10-23" }
  - { field: drawdown.type, url: "https://help.alpha-futures.com/en/articles/9491999-maximum-loss-limit-mll", accessed: "2026-10-23" }
  - { field: daily_loss_limit.kind, url: "https://help.alpha-futures.com/en/articles/9492014-daily-loss-guard", accessed: "2026-10-23" }
  - { field: trading_day_reset, url: "https://help.alpha-futures.com/en/articles/9492014-daily-loss-guard", accessed: "2026-10-23" }
  - { field: copy_trading_allowed, url: "https://help.alpha-futures.com/en/articles/10573989-copy-trading", accessed: "2026-10-23" }
  - { field: copy_trading_allowed (conflict), url: "https://alpha-futures.com/terms-and-conditions", accessed: "2026-10-23" }
  - { field: automated_trading_allowed, url: "https://help.alpha-futures.com/en/articles/9508585-prohibited-trading-practices", accessed: "2026-10-23" }
  - { field: vps_allowed, url: "https://help.alpha-futures.com/en/articles/9508585-prohibited-trading-practices", accessed: "2026-10-23" }
  - { field: platforms, url: "https://help.alpha-futures.com/en/articles/10525305-trading-platforms", accessed: "2026-10-23" }
```

```yaml
# Qualified Account (simulated funded stage)
firm: alpha-futures
plan: "Advanced"
phase: funded
size_label: "50K"
size_usd: "50000.00"
instruments: [MNQ, NQ]
platforms: [AlphaTrader, "TradingView (Plus500 connection)", WealthCharts, "DeepChart (Volumetrica Trading)", "Quantower (dxFeed / Volumetrica Trading)"]
drawdown: { amount_usd: "1750.00", type: trailing_eod, stops_trailing_at: "Maximum Loss Limit stops trailing at the account starting balance on all of our accounts." }
daily_loss_limit: { amount_usd: null, kind: none }
profit_target_usd: null
consistency_rule: { text: "There is NO Consistency Rule on Advanced Qualified Accounts.", threshold_pct: null }
max_position: { minis: 5, micros: 50, scaling: "There is no Scaling Plan on Advanced Accounts." }
min_trading_days: 5
trading_day_reset: { time: "18:00", tz_as_published: "ET" }
copy_trading_allowed: conditional
automated_trading_allowed: no
automation_rule_text: "The use of AI, bots, and other automated trading mechanisms is strictly prohibited across all account types."
vps_allowed: conditional
personal_device_required: unknown   # not mentioned on any official page (gap G12)
status: partial
not_confirmed_fields: ["drawdown.amount_usd (T&C 4%)", "copy_trading_allowed (scope)", "max_accounts (3 vs 5)"]
sources:
  - { field: drawdown.amount_usd, url: "https://help.alpha-futures.com/en/articles/11634907-advanced-account-overview", accessed: "2026-10-23" }
  - { field: daily_loss_limit, url: "https://help.alpha-futures.com/en/articles/11634907-advanced-account-overview", accessed: "2026-10-23" }
  - { field: max_position, url: "https://help.alpha-futures.com/en/articles/11634907-advanced-account-overview", accessed: "2026-10-23" }
  - { field: scaling, url: "https://help.alpha-futures.com/en/articles/9492025-scaling-plan", accessed: "2026-10-23" }
  - { field: consistency_rule, url: "https://help.alpha-futures.com/en/articles/9492048-consistency-rule", accessed: "2026-10-23" }
  - { field: min_trading_days, url: "https://alpha-futures.com/product/advanced", accessed: "2026-10-23" }
  - { field: drawdown (conflict), url: "https://alpha-futures.com/terms-and-conditions", accessed: "2026-10-23" }
  - { field: drawdown.type, url: "https://help.alpha-futures.com/en/articles/9491999-maximum-loss-limit-mll", accessed: "2026-10-23" }
  - { field: daily_loss_limit.kind, url: "https://help.alpha-futures.com/en/articles/9492014-daily-loss-guard", accessed: "2026-10-23" }
  - { field: trading_day_reset, url: "https://help.alpha-futures.com/en/articles/9492014-daily-loss-guard", accessed: "2026-10-23" }
  - { field: copy_trading_allowed, url: "https://help.alpha-futures.com/en/articles/10573989-copy-trading", accessed: "2026-10-23" }
  - { field: copy_trading_allowed (conflict), url: "https://alpha-futures.com/terms-and-conditions", accessed: "2026-10-23" }
  - { field: automated_trading_allowed, url: "https://help.alpha-futures.com/en/articles/9508585-prohibited-trading-practices", accessed: "2026-10-23" }
  - { field: vps_allowed, url: "https://help.alpha-futures.com/en/articles/9508585-prohibited-trading-practices", accessed: "2026-10-23" }
  - { field: platforms, url: "https://help.alpha-futures.com/en/articles/10525305-trading-platforms", accessed: "2026-10-23" }
```

```yaml
# Qualified Account (simulated funded stage)
firm: alpha-futures
plan: "Advanced"
phase: funded
size_label: "100K"
size_usd: "100000.00"
instruments: [MNQ, NQ]
platforms: [AlphaTrader, "TradingView (Plus500 connection)", WealthCharts, "DeepChart (Volumetrica Trading)", "Quantower (dxFeed / Volumetrica Trading)"]
drawdown: { amount_usd: "3500.00", type: trailing_eod, stops_trailing_at: "Maximum Loss Limit stops trailing at the account starting balance on all of our accounts." }
daily_loss_limit: { amount_usd: null, kind: none }
profit_target_usd: null
consistency_rule: { text: "There is NO Consistency Rule on Advanced Qualified Accounts.", threshold_pct: null }
max_position: { minis: 10, micros: 100, scaling: "There is no Scaling Plan on Advanced Accounts." }
min_trading_days: 5
trading_day_reset: { time: "18:00", tz_as_published: "ET" }
copy_trading_allowed: conditional
automated_trading_allowed: no
automation_rule_text: "The use of AI, bots, and other automated trading mechanisms is strictly prohibited across all account types."
vps_allowed: conditional
personal_device_required: unknown   # not mentioned on any official page (gap G12)
status: partial
not_confirmed_fields: ["drawdown.amount_usd (T&C 4%)", "copy_trading_allowed (scope)", "max_accounts (3 vs 5)"]
sources:
  - { field: drawdown.amount_usd, url: "https://help.alpha-futures.com/en/articles/11634907-advanced-account-overview", accessed: "2026-10-23" }
  - { field: daily_loss_limit, url: "https://help.alpha-futures.com/en/articles/11634907-advanced-account-overview", accessed: "2026-10-23" }
  - { field: max_position, url: "https://help.alpha-futures.com/en/articles/11634907-advanced-account-overview", accessed: "2026-10-23" }
  - { field: scaling, url: "https://help.alpha-futures.com/en/articles/9492025-scaling-plan", accessed: "2026-10-23" }
  - { field: consistency_rule, url: "https://help.alpha-futures.com/en/articles/9492048-consistency-rule", accessed: "2026-10-23" }
  - { field: min_trading_days, url: "https://alpha-futures.com/product/advanced", accessed: "2026-10-23" }
  - { field: drawdown (conflict), url: "https://alpha-futures.com/terms-and-conditions", accessed: "2026-10-23" }
  - { field: drawdown.type, url: "https://help.alpha-futures.com/en/articles/9491999-maximum-loss-limit-mll", accessed: "2026-10-23" }
  - { field: daily_loss_limit.kind, url: "https://help.alpha-futures.com/en/articles/9492014-daily-loss-guard", accessed: "2026-10-23" }
  - { field: trading_day_reset, url: "https://help.alpha-futures.com/en/articles/9492014-daily-loss-guard", accessed: "2026-10-23" }
  - { field: copy_trading_allowed, url: "https://help.alpha-futures.com/en/articles/10573989-copy-trading", accessed: "2026-10-23" }
  - { field: copy_trading_allowed (conflict), url: "https://alpha-futures.com/terms-and-conditions", accessed: "2026-10-23" }
  - { field: automated_trading_allowed, url: "https://help.alpha-futures.com/en/articles/9508585-prohibited-trading-practices", accessed: "2026-10-23" }
  - { field: vps_allowed, url: "https://help.alpha-futures.com/en/articles/9508585-prohibited-trading-practices", accessed: "2026-10-23" }
  - { field: platforms, url: "https://help.alpha-futures.com/en/articles/10525305-trading-platforms", accessed: "2026-10-23" }
```

```yaml
# Qualified Account (simulated funded stage)
firm: alpha-futures
plan: "Advanced"
phase: funded
size_label: "150K"
size_usd: "150000.00"
instruments: [MNQ, NQ]
platforms: [AlphaTrader, "TradingView (Plus500 connection)", WealthCharts, "DeepChart (Volumetrica Trading)", "Quantower (dxFeed / Volumetrica Trading)"]
drawdown: { amount_usd: "5250.00", type: trailing_eod, stops_trailing_at: "Maximum Loss Limit stops trailing at the account starting balance on all of our accounts." }
daily_loss_limit: { amount_usd: null, kind: none }
profit_target_usd: null
consistency_rule: { text: "There is NO Consistency Rule on Advanced Qualified Accounts.", threshold_pct: null }
max_position: { minis: 15, micros: 150, scaling: "There is no Scaling Plan on Advanced Accounts." }
min_trading_days: 5
trading_day_reset: { time: "18:00", tz_as_published: "ET" }
copy_trading_allowed: conditional
automated_trading_allowed: no
automation_rule_text: "The use of AI, bots, and other automated trading mechanisms is strictly prohibited across all account types."
vps_allowed: conditional
personal_device_required: unknown   # not mentioned on any official page (gap G12)
status: partial
not_confirmed_fields: ["drawdown.amount_usd (T&C 4%)", "copy_trading_allowed (scope)", "max_accounts (3 vs 5)"]
sources:
  - { field: drawdown.amount_usd, url: "https://help.alpha-futures.com/en/articles/11634907-advanced-account-overview", accessed: "2026-10-23" }
  - { field: daily_loss_limit, url: "https://help.alpha-futures.com/en/articles/11634907-advanced-account-overview", accessed: "2026-10-23" }
  - { field: max_position, url: "https://help.alpha-futures.com/en/articles/11634907-advanced-account-overview", accessed: "2026-10-23" }
  - { field: scaling, url: "https://help.alpha-futures.com/en/articles/9492025-scaling-plan", accessed: "2026-10-23" }
  - { field: consistency_rule, url: "https://help.alpha-futures.com/en/articles/9492048-consistency-rule", accessed: "2026-10-23" }
  - { field: min_trading_days, url: "https://alpha-futures.com/product/advanced", accessed: "2026-10-23" }
  - { field: drawdown (conflict), url: "https://alpha-futures.com/terms-and-conditions", accessed: "2026-10-23" }
  - { field: drawdown.type, url: "https://help.alpha-futures.com/en/articles/9491999-maximum-loss-limit-mll", accessed: "2026-10-23" }
  - { field: daily_loss_limit.kind, url: "https://help.alpha-futures.com/en/articles/9492014-daily-loss-guard", accessed: "2026-10-23" }
  - { field: trading_day_reset, url: "https://help.alpha-futures.com/en/articles/9492014-daily-loss-guard", accessed: "2026-10-23" }
  - { field: copy_trading_allowed, url: "https://help.alpha-futures.com/en/articles/10573989-copy-trading", accessed: "2026-10-23" }
  - { field: copy_trading_allowed (conflict), url: "https://alpha-futures.com/terms-and-conditions", accessed: "2026-10-23" }
  - { field: automated_trading_allowed, url: "https://help.alpha-futures.com/en/articles/9508585-prohibited-trading-practices", accessed: "2026-10-23" }
  - { field: vps_allowed, url: "https://help.alpha-futures.com/en/articles/9508585-prohibited-trading-practices", accessed: "2026-10-23" }
  - { field: platforms, url: "https://help.alpha-futures.com/en/articles/10525305-trading-platforms", accessed: "2026-10-23" }
```

```yaml
# No evaluation; starts as a Qualified Account. Withdrawal Profit Target (not a pass target): 1st $1500.00, 2nd+ $1000.00
firm: alpha-futures
plan: "Direct"
phase: funded
size_label: "25K"
size_usd: "25000.00"
instruments: [MNQ, NQ]
platforms: [AlphaTrader, "TradingView (Plus500 connection)", WealthCharts, "DeepChart (Volumetrica Trading)", "Quantower (dxFeed / Volumetrica Trading)"]
drawdown: { amount_usd: "1000.00", type: trailing_eod, stops_trailing_at: "Maximum Loss Limit stops trailing at the account starting balance on all of our accounts." }
daily_loss_limit: { amount_usd: "500.00", kind: soft_pause }
profit_target_usd: null
consistency_rule: { text: "profits from any single trading day cannot be greater than or equal to 20% of net profits accumulated since last withdrawal request.", threshold_pct: "20" }
max_position: { minis: 2, micros: 20, scaling: "No Scaling Plan on Direct Qualified Accounts" }
min_trading_days: 5   # T&C Schedule 2 only; payout policy: no winning-day requirement
trading_day_reset: { time: "18:00", tz_as_published: "ET" }
copy_trading_allowed: conditional
automated_trading_allowed: no
automation_rule_text: "The use of AI, bots, and other automated trading mechanisms is strictly prohibited across all account types."
vps_allowed: conditional
personal_device_required: unknown   # not mentioned on any official page (gap G12)
status: partial
not_confirmed_fields: ["copy_trading_allowed (scope)", "max_accounts (5 total vs 5 per size tier)", "min_trading_days"]
sources:
  - { field: drawdown.amount_usd, url: "https://help.alpha-futures.com/en/articles/15838742-direct-account-overview", accessed: "2026-10-23" }
  - { field: daily_loss_limit, url: "https://help.alpha-futures.com/en/articles/15838742-direct-account-overview", accessed: "2026-10-23" }
  - { field: max_position, url: "https://help.alpha-futures.com/en/articles/15838742-direct-account-overview", accessed: "2026-10-23" }
  - { field: consistency_rule, url: "https://help.alpha-futures.com/en/articles/9492048-consistency-rule", accessed: "2026-10-23" }
  - { field: withdrawal_profit_target, url: "https://help.alpha-futures.com/en/articles/15838742-direct-account-overview", accessed: "2026-10-23" }
  - { field: min_trading_days, url: "https://alpha-futures.com/terms-and-conditions", accessed: "2026-10-23" }
  - { field: drawdown.type, url: "https://help.alpha-futures.com/en/articles/9491999-maximum-loss-limit-mll", accessed: "2026-10-23" }
  - { field: daily_loss_limit.kind, url: "https://help.alpha-futures.com/en/articles/9492014-daily-loss-guard", accessed: "2026-10-23" }
  - { field: trading_day_reset, url: "https://help.alpha-futures.com/en/articles/9492014-daily-loss-guard", accessed: "2026-10-23" }
  - { field: copy_trading_allowed, url: "https://help.alpha-futures.com/en/articles/10573989-copy-trading", accessed: "2026-10-23" }
  - { field: copy_trading_allowed (conflict), url: "https://alpha-futures.com/terms-and-conditions", accessed: "2026-10-23" }
  - { field: automated_trading_allowed, url: "https://help.alpha-futures.com/en/articles/9508585-prohibited-trading-practices", accessed: "2026-10-23" }
  - { field: vps_allowed, url: "https://help.alpha-futures.com/en/articles/9508585-prohibited-trading-practices", accessed: "2026-10-23" }
  - { field: platforms, url: "https://help.alpha-futures.com/en/articles/10525305-trading-platforms", accessed: "2026-10-23" }
```

```yaml
# No evaluation; starts as a Qualified Account. Withdrawal Profit Target (not a pass target): 1st $3000.00, 2nd+ $2000.00
firm: alpha-futures
plan: "Direct"
phase: funded
size_label: "50K"
size_usd: "50000.00"
instruments: [MNQ, NQ]
platforms: [AlphaTrader, "TradingView (Plus500 connection)", WealthCharts, "DeepChart (Volumetrica Trading)", "Quantower (dxFeed / Volumetrica Trading)"]
drawdown: { amount_usd: "2000.00", type: trailing_eod, stops_trailing_at: "Maximum Loss Limit stops trailing at the account starting balance on all of our accounts." }
daily_loss_limit: { amount_usd: "1000.00", kind: soft_pause }
profit_target_usd: null
consistency_rule: { text: "profits from any single trading day cannot be greater than or equal to 20% of net profits accumulated since last withdrawal request.", threshold_pct: "20" }
max_position: { minis: 4, micros: 40, scaling: "No Scaling Plan on Direct Qualified Accounts" }
min_trading_days: 5   # T&C Schedule 2 only; payout policy: no winning-day requirement
trading_day_reset: { time: "18:00", tz_as_published: "ET" }
copy_trading_allowed: conditional
automated_trading_allowed: no
automation_rule_text: "The use of AI, bots, and other automated trading mechanisms is strictly prohibited across all account types."
vps_allowed: conditional
personal_device_required: unknown   # not mentioned on any official page (gap G12)
status: partial
not_confirmed_fields: ["copy_trading_allowed (scope)", "max_accounts (5 total vs 5 per size tier)", "min_trading_days"]
sources:
  - { field: drawdown.amount_usd, url: "https://help.alpha-futures.com/en/articles/15838742-direct-account-overview", accessed: "2026-10-23" }
  - { field: daily_loss_limit, url: "https://help.alpha-futures.com/en/articles/15838742-direct-account-overview", accessed: "2026-10-23" }
  - { field: max_position, url: "https://help.alpha-futures.com/en/articles/15838742-direct-account-overview", accessed: "2026-10-23" }
  - { field: consistency_rule, url: "https://help.alpha-futures.com/en/articles/9492048-consistency-rule", accessed: "2026-10-23" }
  - { field: withdrawal_profit_target, url: "https://help.alpha-futures.com/en/articles/15838742-direct-account-overview", accessed: "2026-10-23" }
  - { field: min_trading_days, url: "https://alpha-futures.com/terms-and-conditions", accessed: "2026-10-23" }
  - { field: drawdown.type, url: "https://help.alpha-futures.com/en/articles/9491999-maximum-loss-limit-mll", accessed: "2026-10-23" }
  - { field: daily_loss_limit.kind, url: "https://help.alpha-futures.com/en/articles/9492014-daily-loss-guard", accessed: "2026-10-23" }
  - { field: trading_day_reset, url: "https://help.alpha-futures.com/en/articles/9492014-daily-loss-guard", accessed: "2026-10-23" }
  - { field: copy_trading_allowed, url: "https://help.alpha-futures.com/en/articles/10573989-copy-trading", accessed: "2026-10-23" }
  - { field: copy_trading_allowed (conflict), url: "https://alpha-futures.com/terms-and-conditions", accessed: "2026-10-23" }
  - { field: automated_trading_allowed, url: "https://help.alpha-futures.com/en/articles/9508585-prohibited-trading-practices", accessed: "2026-10-23" }
  - { field: vps_allowed, url: "https://help.alpha-futures.com/en/articles/9508585-prohibited-trading-practices", accessed: "2026-10-23" }
  - { field: platforms, url: "https://help.alpha-futures.com/en/articles/10525305-trading-platforms", accessed: "2026-10-23" }
```

```yaml
# No evaluation; starts as a Qualified Account. Withdrawal Profit Target (not a pass target): 1st $6000.00, 2nd+ $4000.00
firm: alpha-futures
plan: "Direct"
phase: funded
size_label: "100K"
size_usd: "100000.00"
instruments: [MNQ, NQ]
platforms: [AlphaTrader, "TradingView (Plus500 connection)", WealthCharts, "DeepChart (Volumetrica Trading)", "Quantower (dxFeed / Volumetrica Trading)"]
drawdown: { amount_usd: "3000.00", type: trailing_eod, stops_trailing_at: "Maximum Loss Limit stops trailing at the account starting balance on all of our accounts." }
daily_loss_limit: { amount_usd: "2000.00", kind: soft_pause }
profit_target_usd: null
consistency_rule: { text: "profits from any single trading day cannot be greater than or equal to 20% of net profits accumulated since last withdrawal request.", threshold_pct: "20" }
max_position: { minis: 8, micros: 80, scaling: "No Scaling Plan on Direct Qualified Accounts" }
min_trading_days: 5   # T&C Schedule 2 only; payout policy: no winning-day requirement
trading_day_reset: { time: "18:00", tz_as_published: "ET" }
copy_trading_allowed: conditional
automated_trading_allowed: no
automation_rule_text: "The use of AI, bots, and other automated trading mechanisms is strictly prohibited across all account types."
vps_allowed: conditional
personal_device_required: unknown   # not mentioned on any official page (gap G12)
status: partial
not_confirmed_fields: ["copy_trading_allowed (scope)", "max_accounts (5 total vs 5 per size tier)", "min_trading_days"]
sources:
  - { field: drawdown.amount_usd, url: "https://help.alpha-futures.com/en/articles/15838742-direct-account-overview", accessed: "2026-10-23" }
  - { field: daily_loss_limit, url: "https://help.alpha-futures.com/en/articles/15838742-direct-account-overview", accessed: "2026-10-23" }
  - { field: max_position, url: "https://help.alpha-futures.com/en/articles/15838742-direct-account-overview", accessed: "2026-10-23" }
  - { field: consistency_rule, url: "https://help.alpha-futures.com/en/articles/9492048-consistency-rule", accessed: "2026-10-23" }
  - { field: withdrawal_profit_target, url: "https://help.alpha-futures.com/en/articles/15838742-direct-account-overview", accessed: "2026-10-23" }
  - { field: min_trading_days, url: "https://alpha-futures.com/terms-and-conditions", accessed: "2026-10-23" }
  - { field: drawdown.type, url: "https://help.alpha-futures.com/en/articles/9491999-maximum-loss-limit-mll", accessed: "2026-10-23" }
  - { field: daily_loss_limit.kind, url: "https://help.alpha-futures.com/en/articles/9492014-daily-loss-guard", accessed: "2026-10-23" }
  - { field: trading_day_reset, url: "https://help.alpha-futures.com/en/articles/9492014-daily-loss-guard", accessed: "2026-10-23" }
  - { field: copy_trading_allowed, url: "https://help.alpha-futures.com/en/articles/10573989-copy-trading", accessed: "2026-10-23" }
  - { field: copy_trading_allowed (conflict), url: "https://alpha-futures.com/terms-and-conditions", accessed: "2026-10-23" }
  - { field: automated_trading_allowed, url: "https://help.alpha-futures.com/en/articles/9508585-prohibited-trading-practices", accessed: "2026-10-23" }
  - { field: vps_allowed, url: "https://help.alpha-futures.com/en/articles/9508585-prohibited-trading-practices", accessed: "2026-10-23" }
  - { field: platforms, url: "https://help.alpha-futures.com/en/articles/10525305-trading-platforms", accessed: "2026-10-23" }
```

```yaml
# No evaluation; starts as a Qualified Account. Withdrawal Profit Target (not a pass target): 1st $9000.00, 2nd+ $6000.00
firm: alpha-futures
plan: "Direct"
phase: funded
size_label: "150K"
size_usd: "150000.00"
instruments: [MNQ, NQ]
platforms: [AlphaTrader, "TradingView (Plus500 connection)", WealthCharts, "DeepChart (Volumetrica Trading)", "Quantower (dxFeed / Volumetrica Trading)"]
drawdown: { amount_usd: "4500.00", type: trailing_eod, stops_trailing_at: "Maximum Loss Limit stops trailing at the account starting balance on all of our accounts." }
daily_loss_limit: { amount_usd: "3000.00", kind: soft_pause }
profit_target_usd: null
consistency_rule: { text: "profits from any single trading day cannot be greater than or equal to 20% of net profits accumulated since last withdrawal request.", threshold_pct: "20" }
max_position: { minis: 10, micros: 100, scaling: "No Scaling Plan on Direct Qualified Accounts" }
min_trading_days: 5   # T&C Schedule 2 only; payout policy: no winning-day requirement
trading_day_reset: { time: "18:00", tz_as_published: "ET" }
copy_trading_allowed: conditional
automated_trading_allowed: no
automation_rule_text: "The use of AI, bots, and other automated trading mechanisms is strictly prohibited across all account types."
vps_allowed: conditional
personal_device_required: unknown   # not mentioned on any official page (gap G12)
status: partial
not_confirmed_fields: ["copy_trading_allowed (scope)", "max_accounts (5 total vs 5 per size tier)", "min_trading_days"]
sources:
  - { field: drawdown.amount_usd, url: "https://help.alpha-futures.com/en/articles/15838742-direct-account-overview", accessed: "2026-10-23" }
  - { field: daily_loss_limit, url: "https://help.alpha-futures.com/en/articles/15838742-direct-account-overview", accessed: "2026-10-23" }
  - { field: max_position, url: "https://help.alpha-futures.com/en/articles/15838742-direct-account-overview", accessed: "2026-10-23" }
  - { field: consistency_rule, url: "https://help.alpha-futures.com/en/articles/9492048-consistency-rule", accessed: "2026-10-23" }
  - { field: withdrawal_profit_target, url: "https://help.alpha-futures.com/en/articles/15838742-direct-account-overview", accessed: "2026-10-23" }
  - { field: min_trading_days, url: "https://alpha-futures.com/terms-and-conditions", accessed: "2026-10-23" }
  - { field: drawdown.type, url: "https://help.alpha-futures.com/en/articles/9491999-maximum-loss-limit-mll", accessed: "2026-10-23" }
  - { field: daily_loss_limit.kind, url: "https://help.alpha-futures.com/en/articles/9492014-daily-loss-guard", accessed: "2026-10-23" }
  - { field: trading_day_reset, url: "https://help.alpha-futures.com/en/articles/9492014-daily-loss-guard", accessed: "2026-10-23" }
  - { field: copy_trading_allowed, url: "https://help.alpha-futures.com/en/articles/10573989-copy-trading", accessed: "2026-10-23" }
  - { field: copy_trading_allowed (conflict), url: "https://alpha-futures.com/terms-and-conditions", accessed: "2026-10-23" }
  - { field: automated_trading_allowed, url: "https://help.alpha-futures.com/en/articles/9508585-prohibited-trading-practices", accessed: "2026-10-23" }
  - { field: vps_allowed, url: "https://help.alpha-futures.com/en/articles/9508585-prohibited-trading-practices", accessed: "2026-10-23" }
  - { field: platforms, url: "https://help.alpha-futures.com/en/articles/10525305-trading-platforms", accessed: "2026-10-23" }
```

```yaml
# Live capital, by call-up only. Published 'Start Balance $0', 'EOD Drawdown $1000.00'. 'EOD Style Drawdown'; 'Scaling Daily Loss Limit (30% of account)'
firm: alpha-futures
plan: "Alpha Futures Live Program / Alpha Prime (Path to Live)"
phase: live
size_label: "Live from 25K Qualified"
size_usd: null
instruments: [MNQ, NQ]
platforms: []   # not published for live accounts
drawdown: { amount_usd: "1000.00", type: other, stops_trailing_at: "not stated for live accounts" }
daily_loss_limit: { amount_usd: null, kind: soft_pause }
profit_target_usd: null
consistency_rule: { text: "No Consistency Rule", threshold_pct: null }
max_position: { minis: 1, micros: 10, scaling: "before MLL reaches $0 balance: 1 mini / 10 micro; after MLL reaches $0 balance: 2 mini / 20 micro" }
min_trading_days: null
trading_day_reset: { time: null, tz_as_published: null }
copy_trading_allowed: unknown
automated_trading_allowed: unknown
automation_rule_text: null
vps_allowed: unknown
personal_device_required: unknown   # not mentioned on any official page (gap G12)
status: partial
not_confirmed_fields: ["drawdown.type", "daily_loss_limit.amount_usd (30% of account)", "daily_loss_limit.kind", "platforms", "trading_day_reset", "copy_trading_allowed", "automated_trading_allowed", "vps_allowed"]
sources:
  - { field: drawdown.amount_usd, url: "https://help.alpha-futures.com/en/articles/10743344-path-to-live-structure", accessed: "2026-10-23" }
  - { field: max_position, url: "https://help.alpha-futures.com/en/articles/10743344-path-to-live-structure", accessed: "2026-10-23" }
  - { field: daily_loss_limit, url: "https://help.alpha-futures.com/en/articles/10743344-path-to-live-structure", accessed: "2026-10-23" }
  - { field: daily_loss_limit.kind, url: "https://help.alpha-futures.com/en/articles/11023753-live-account-rules-and-parameters", accessed: "2026-10-23" }
  - { field: consistency_rule, url: "https://help.alpha-futures.com/en/articles/10743344-path-to-live-structure", accessed: "2026-10-23" }
```

```yaml
# Live capital, by call-up only. Published 'Start Balance $0', 'EOD Drawdown $2000.00'. 'EOD Style Drawdown'; 'Scaling Daily Loss Limit (30% of account)'
firm: alpha-futures
plan: "Alpha Futures Live Program / Alpha Prime (Path to Live)"
phase: live
size_label: "Live from 50K Qualified"
size_usd: null
instruments: [MNQ, NQ]
platforms: []   # not published for live accounts
drawdown: { amount_usd: "2000.00", type: other, stops_trailing_at: "not stated for live accounts" }
daily_loss_limit: { amount_usd: null, kind: soft_pause }
profit_target_usd: null
consistency_rule: { text: "No Consistency Rule", threshold_pct: null }
max_position: { minis: 2, micros: 20, scaling: "before MLL reaches $0 balance: 2 mini / 20 micro; after MLL reaches $0 balance: 4 mini / 40 micro" }
min_trading_days: null
trading_day_reset: { time: null, tz_as_published: null }
copy_trading_allowed: unknown
automated_trading_allowed: unknown
automation_rule_text: null
vps_allowed: unknown
personal_device_required: unknown   # not mentioned on any official page (gap G12)
status: partial
not_confirmed_fields: ["drawdown.type", "daily_loss_limit.amount_usd (30% of account)", "daily_loss_limit.kind", "platforms", "trading_day_reset", "copy_trading_allowed", "automated_trading_allowed", "vps_allowed"]
sources:
  - { field: drawdown.amount_usd, url: "https://help.alpha-futures.com/en/articles/10743344-path-to-live-structure", accessed: "2026-10-23" }
  - { field: max_position, url: "https://help.alpha-futures.com/en/articles/10743344-path-to-live-structure", accessed: "2026-10-23" }
  - { field: daily_loss_limit, url: "https://help.alpha-futures.com/en/articles/10743344-path-to-live-structure", accessed: "2026-10-23" }
  - { field: daily_loss_limit.kind, url: "https://help.alpha-futures.com/en/articles/11023753-live-account-rules-and-parameters", accessed: "2026-10-23" }
  - { field: consistency_rule, url: "https://help.alpha-futures.com/en/articles/10743344-path-to-live-structure", accessed: "2026-10-23" }
```

```yaml
# Live capital, by call-up only. Published 'Start Balance $0', 'EOD Drawdown $3000.00'. 'EOD Style Drawdown'; 'Scaling Daily Loss Limit (30% of account)'
firm: alpha-futures
plan: "Alpha Futures Live Program / Alpha Prime (Path to Live)"
phase: live
size_label: "Live from 100K Qualified"
size_usd: null
instruments: [MNQ, NQ]
platforms: []   # not published for live accounts
drawdown: { amount_usd: "3000.00", type: other, stops_trailing_at: "not stated for live accounts" }
daily_loss_limit: { amount_usd: null, kind: soft_pause }
profit_target_usd: null
consistency_rule: { text: "No Consistency Rule", threshold_pct: null }
max_position: { minis: 4, micros: 40, scaling: "before MLL reaches $0 balance: 4 mini / 40 micro; after MLL reaches $0 balance: 8 mini / 80 micro" }
min_trading_days: null
trading_day_reset: { time: null, tz_as_published: null }
copy_trading_allowed: unknown
automated_trading_allowed: unknown
automation_rule_text: null
vps_allowed: unknown
personal_device_required: unknown   # not mentioned on any official page (gap G12)
status: partial
not_confirmed_fields: ["drawdown.type", "daily_loss_limit.amount_usd (30% of account)", "daily_loss_limit.kind", "platforms", "trading_day_reset", "copy_trading_allowed", "automated_trading_allowed", "vps_allowed"]
sources:
  - { field: drawdown.amount_usd, url: "https://help.alpha-futures.com/en/articles/10743344-path-to-live-structure", accessed: "2026-10-23" }
  - { field: max_position, url: "https://help.alpha-futures.com/en/articles/10743344-path-to-live-structure", accessed: "2026-10-23" }
  - { field: daily_loss_limit, url: "https://help.alpha-futures.com/en/articles/10743344-path-to-live-structure", accessed: "2026-10-23" }
  - { field: daily_loss_limit.kind, url: "https://help.alpha-futures.com/en/articles/11023753-live-account-rules-and-parameters", accessed: "2026-10-23" }
  - { field: consistency_rule, url: "https://help.alpha-futures.com/en/articles/10743344-path-to-live-structure", accessed: "2026-10-23" }
```

```yaml
# Live capital, by call-up only. Published 'Start Balance $0', 'EOD Drawdown $4500.00'. 'EOD Style Drawdown'; 'Scaling Daily Loss Limit (30% of account)'
firm: alpha-futures
plan: "Alpha Futures Live Program / Alpha Prime (Path to Live)"
phase: live
size_label: "Live from 150K Qualified"
size_usd: null
instruments: [MNQ, NQ]
platforms: []   # not published for live accounts
drawdown: { amount_usd: "4500.00", type: other, stops_trailing_at: "not stated for live accounts" }
daily_loss_limit: { amount_usd: null, kind: soft_pause }
profit_target_usd: null
consistency_rule: { text: "No Consistency Rule", threshold_pct: null }
max_position: { minis: 6, micros: 60, scaling: "before MLL reaches $0 balance: 6 mini / 60 micro; after MLL reaches $0 balance: 12 mini / 120 micro" }
min_trading_days: null
trading_day_reset: { time: null, tz_as_published: null }
copy_trading_allowed: unknown
automated_trading_allowed: unknown
automation_rule_text: null
vps_allowed: unknown
personal_device_required: unknown   # not mentioned on any official page (gap G12)
status: partial
not_confirmed_fields: ["drawdown.type", "daily_loss_limit.amount_usd (30% of account)", "daily_loss_limit.kind", "platforms", "trading_day_reset", "copy_trading_allowed", "automated_trading_allowed", "vps_allowed"]
sources:
  - { field: drawdown.amount_usd, url: "https://help.alpha-futures.com/en/articles/10743344-path-to-live-structure", accessed: "2026-10-23" }
  - { field: max_position, url: "https://help.alpha-futures.com/en/articles/10743344-path-to-live-structure", accessed: "2026-10-23" }
  - { field: daily_loss_limit, url: "https://help.alpha-futures.com/en/articles/10743344-path-to-live-structure", accessed: "2026-10-23" }
  - { field: daily_loss_limit.kind, url: "https://help.alpha-futures.com/en/articles/11023753-live-account-rules-and-parameters", accessed: "2026-10-23" }
  - { field: consistency_rule, url: "https://help.alpha-futures.com/en/articles/10743344-path-to-live-structure", accessed: "2026-10-23" }
```


## 9. Unverified / gaps / conflicts

| # | Field | Status | What is missing / found | Where searched |
|---|---|---|---|---|
| G1 | Copy trading across the **same person's** Alpha Futures accounts | **conflict** | S12: "Copy trading in general is permitted as long as it is a single user executing the trades, and it is the single individual listed on the Alpha Futures Accounts." T&C §8.1.5: "the only copy trading allowed is that from your own external account to your Alpha Futures Account." | S12, S26 |
| G2 | Copying **from Alpha Futures to other firms** | not mentioned | T&C allows copying from the trader's "own external account" into Alpha Futures; the other direction is not addressed | S12, S26, S11 |
| G3 | Live accounts: automation, copy trading, VPS, platforms, trading-day reset | unknown | S11 covers "Evaluation and Qualified stages"; S15/S16 do not address these. S15 lists "Developing automated strategies with our in house Quantitative Team" for Alpha Prime | S11, S15, S16, S30 |
| G4 | MLL amount: Zero 100K (Eval + Qualified) and Advanced Qualified 50K/100K/150K | **conflict** | Help/product: Zero 100K "$3,000"; Advanced "$1,750 / $3,500 / $5,250". T&C Schedule 1/2: Zero "4%", Advanced Qualified "4%", notes "trailing drawdown set at 4% of all account sizes". Percentages were compared only to detect the conflict | S2, S3, S22, S23, S26 |
| G5 | Advanced Evaluation minimum trading days | **conflict** | S22 cards/FAQ and T&C §8.1.9: 3. T&C Schedule 1 table: 2 | S22, S26 |
| G6 | Zero Evaluation Daily Loss Guard | **conflict (minor)** | S3, S6, S23 and T&C Schedule 1 table: DLG applies on Zero Evaluations ($500 / $1,000 / $2,000). T&C Schedule 2 notes: "a soft breach rule on Qualified Analyst Accounts only" | S3, S6, S23, S26 |
| G7 | Standard 100K max micros | **conflict** | T&C §8.1.10(a): "100k = 10 contracts/10 micros". S1, S21, T&C Schedule 1 and Sch. 4 1.3.3.2: 100 micros | S1, S21, S26 |
| G8 | Max accounts (Advanced, Standard, Direct) | **conflict** | S13: Advanced 3, Standard 5, "Any user that has an active Advanced Qualified account will be limited to 3." T&C Sch. 2: "Any user that has an active Standard or Advanced Qualified account will be limited to 3." T&C §8.1.3: "maximum total allocation per household is five accounts". S22 FAQ (Advanced): "up to 5 accounts at the same time". S24 FAQ (Direct): "up to 5 accounts per size tier" | S13, S22, S24, S26 |
| G9 | Direct minimum trading days | not confirmed | Only T&C Schedule 2 ("Direct Qualified \| 5"). S17: Direct has "no winning day requirement". Product page S24 shows no min days | S4, S17, S24, S26 |
| G10 | Zero 25K in the T&C | gap | T&C §8.1.10 and Schedules 1–3 list no Zero 25K; values come from S3/S23 only | S26 |
| G11 | News-rule penalty wording | note | S9: first violation "only a warning", then breach. T&C Sch. 5: "a few times … honest accidents", then possible closure. S27 How-It-Works FAQ mentions the 2-minute rule only for Zero Qualified (S9 and home FAQ include Standard and Direct) | S9, S26, S27 |
| G12 | Orders must originate from a **personal device** | not mentioned | No official page states it. VPS is "permitted in your trading" (S11) | all 36 help articles, S21–S27, S31, General Service Agreement |
| G13 | Holiday / early-close schedule | not mentioned | — | help-center crawl, S10, S26 |
| G14 | Webhooks / alert-to-order automation; ATM | not mentioned | Platforms list alerts as features (S25) only | help-center crawl, S25, S26 |
| G15 | Whether a platform trade copier counts as "automated trading" | not stated | Copying allowed (S12) while "Automated … Trading" is prohibited in the same article | S11, S12 |
| G16 | **NinjaTrader / Tradovate / Rithmic / CQG** | not offered | Platforms: AlphaTrader, TradingView (Plus500), WealthCharts, DeepChart, Quantower (S14). The All Win NT8 plugin cannot be used with this firm | S14, S25, S29, full crawl |
| G17 | Platform per plan (Quantower / others on Direct) | partial | Only TradingView is stated "on every plan"; no per-plan restriction is published for the others | S14, S21–S25 |
| G18 | Live: drawdown trailing behaviour, DLL in USD, DLL kind for current (non-legacy) live | unverified | S15: "EOD Style Drawdown", "Scaling Daily Loss Limit (30% of account)", "Start Balance $0". The soft-pause sentence is on S16 next to the legacy-live description | S15, S16 |
| G19 | Daily Loss Guard base wording | note | S6: "2% of starting account balance"; T&C Sch. 2 notes: "2% of account balance on all of our account sizes". The USD values (S1, S3, S4) are used | S6, S26 |
| G20 | MLL breach timing on Qualified | note | S5: "breached and will be closed"; T&C Sch. 1: "closed at the end of that trading day"; both: liquidation on breach | S5, S26 |

## 10. Changelog

- 2026-10-23 — initial research (Lupa)
- 2026-10-23 — rev bump R4: personal-device and VPS re-checked on the newest help-center and T&C text; nothing new found, `personal_device_required` stays unknown (G12), `vps_allowed` stays conditional; no YAML change (Lupa)
