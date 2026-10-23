# Bulenox — Preset Research

| | |
|---|---|
| **Firm** | Bulenox (Bulenox LLC, 1201 N Orange St, Ste 7149, Wilmington, DE 19801), futures only |
| **Official domains** | `bulenox.com`: site, Accounts & Pricing, FAQ (`/faq`), Help Center (`/help-center/<category>`), legal PDFs (`/legal/Terms_of_Use.pdf`, `/legal/Risk_Disclosure.pdf`). The FAQ and Help Center are JavaScript pages. Their text comes from the site's own public CMS (`https://bulenox.com/cms/items/faq_items`, `/cms/items/help_items`, `/cms/items/help_categories`). I read the CMS JSON with curl, and I checked one rendered answer in a browser portal: the FAQ answer "Can I use bots, algos or trade copiers?" matches the CMS text word for word. `bulenox.com/member/*` (login, checkout) was **not** opened. |
| **Research date** | 2026-10-23 (all URLs below accessed 2026-10-23) |
| **Researcher** | Lupa (Prop Firm Rules Researcher) |
| **Format** | preset-research-format.md rev. 2, plus the PO rev. 3 addition (VPS / personal device) |
| **File status** | `partial`. Drawdown, daily loss limit, profit target, contract limits, scaling, trading day, copy stance and automation stance are sourced for every Qualification, Master, Fast Track and Momentum size. Still open: one automation conflict (C1). The rules for the Momentum Master stage and the live Funded Account are mostly not published. VPS and personal device are not mentioned. Lock rules for Qualification and Momentum are not stated. See §9. |
| **Instruments covered** | MNQ, NQ. Both are listed: "E-mini NASDAQ 100 (NQ)" and "Micro E-mini Nasdaq-100 (MNQ)" ([Help Center › Qualification › Instruments](https://bulenox.com/help-center/qualification#instruments)). "One standard contract is equivalent to ten micro contracts." |
| **Sizes** | $25,000, $50,000, $100,000 and $150,000 in every program. There is nothing above $150K ("Get funded up to $150,000", homepage), so 200K and 300K do not exist. |

## 1. Header notes

Evidence: raw text of every cited page, with the URL on line 1. Raw HTML, JS bundles and CMS JSON are in `raw/`. Location: `/tmp/claude-1003/-srv-www-allwin--maestri-roles-6bf77a80-a1ed-43dc-bc00-08a480f178a5/33652d13-4dae-46b6-ae06-44a72b375517/scratchpad/r2/bulenox/` (files: `help-center.txt`, `faq.txt`, `accounts-pricing.txt`, `home.txt`, `terms-of-use.txt`, `risk-disclosure.txt`, `raw/faq-portal-bots-item.html`). This is temporary scratch storage; copy it if a permanent audit trail is needed.

**Programs found** (names as published):

- **Qualification** ("The classic route. Pass a Qualification Account, activate a Master Account and trade your way to a Funded Account.") → **Master Account** (simulated) → **Funded Account** (live, "a real brokerage account funded by Bulenox and traded with real money").
- **Fast Track** ("a simulated live account bought without a qualification stage") → after at least 3 payouts it may be reviewed for a **Funded Account**.
- **Momentum** ("a qualification that includes the Master stage at no extra cost") → **Momentum Master** → Funded Account.

Every program is sold in two **risk models**, chosen at purchase: **Option 1 — No Scaling / Trailing Drawdown** and **Option 2 — EOD (End-of-Day) Drawdown**. "This choice is locked for the life of the account." ([accounts-pricing](https://bulenox.com/accounts-pricing))

### Source index (all accessed 2026-10-23)

| Key | URL | Content |
|---|---|---|
| S-HOME | https://bulenox.com/ | Program overview, risk-model chooser, platform logos (alt text), "Trade until 15:59 CT" |
| S-PRICE | https://bulenox.com/accounts-pricing | Plan cards per program × risk model × size (tier data in the page bundle `/assets/index-BLUF_KPG.js`) |
| S-FAQ | https://bulenox.com/faq | Bots/algos/copiers, news, number of accounts, platforms |
| S-HQ | https://bulenox.com/help-center/qualification (`#rules`, `#instruments`, `#option-1`, `#option-2`, `#multiple-accounts`) | Qualification rules, trading day, Option 1/2 mechanics, DLL, scaling |
| S-HFT | https://bulenox.com/help-center/fast-track (`#ft-parameters`, `#ft-consistency`, `#ft-first-payout`, `#ft-payout-limits`, `#ft-rules-sessions`) | Fast Track parameters |
| S-HMO | https://bulenox.com/help-center/momentum (`#mo-parameters`, `#mo-rules`, `#mo-consistency`, `#mo-qualifying-days`, `#mo-minimum-balance`) | Momentum parameters |
| S-HMA | https://bulenox.com/help-center/master (`#master-trading-rules`, `#withdrawal-rules`, `#multiple-master-accounts`, Inactivity Rule) | Master rules, lock, DLL, scaling, payouts |
| S-HFU | https://bulenox.com/help-center/funded (`#moving-to-funded`, `#consolidation`, `#funded-setup`) | Funded Account transition |
| S-HCO | https://bulenox.com/help-center/connection (`#rithmic-connection`, `#ninjatrader-connection`, `#other-platforms`) | Platforms, OCO note |
| S-TOU | https://bulenox.com/legal/Terms_of_Use.pdf ("Updated October 2026") | "ALGORITHMS AND AUTO TRADING", household rule |
| S-RISK | https://bulenox.com/legal/Risk_Disclosure.pdf | Generic disclosure only (no trading rules) |

---

## 2. Platforms

All plans use **Rithmic** (data and order routing; "Rithmic Paper Trading" system), with **R | Trader Pro** as the base application. The firm publishes no per-plan platform restriction for Qualification, Master, Fast Track or Momentum ("Three account types. Same markets, same platforms — different paths to a payout." — S-PRICE). **No platform is stated for the live Funded Account** (gap G7).

> "Download and install R | Trader Pro … Download the preferred additional Trading Platform … Connect and Setup Bulenox Trader’s Account" — S-HOME (How it works)

> "Under System, select Rithmic Paper Trading. Under Gateway, select Chicago. … Make sure that Market Data is set to On and Allow Plugins is set to On." — [S-HCO #rithmic-connection](https://bulenox.com/help-center/connection#rithmic-connection)

> "More than 20, including NinjaTrader, R|Trader Pro, Quantower, Sierra Chart, MultiCharts and ATAS. NinjaTrader 8 comes with a free licence on the Master Account." — S-FAQ ("What platforms do you support?")

> "Our partners — Our data provider and our platform partner." (logos: Rithmic, NinjaTrader) — S-HOME

- **NinjaTrader 8** connects through "My Rithmic for NinjaTrader Brokerage" with "System: Rithmic Paper Trading (Chicago)" and "Connection Name: Bulenox" ([S-HCO #ninjatrader-connection](https://bulenox.com/help-center/connection#ninjatrader-connection), "NinjaTrader Connection (Free License)").
- **Full platform list.** This list comes from the logo alt texts on the homepage "Our Available Platforms" band. The Help Center "Other Supported Platforms" item shows the same logos without text. The list is: R | Trader Pro, NinjaTrader, Quantower, Sierra Chart, ATAS, Bookmap, Tiger.Trade, MotiveWave, Investor/RT, Photon Trader, Trade Navigator, TSLab, ScalpTool, QST, Medved Trader, OverCharts, QScalp, VolFix, Inside Edge, Optimus Flow, MultiCharts.
- **Fee note.** A $100/month fee applies when connecting "through a Third-Party API or Trader Composite Software" (S-HQ #rules, S-FAQ, S-TOU). This matters for automation; see §6.
- **Not offered.** No Tradovate, CQG, DXFeed, ProjectX or TopstepX connection is mentioned anywhere.
- **Operating system.** "Rithmic runs on Windows and is not supported on macOS or ChromeOS." (S-HCO)

There is **no platform-dependent rule**, so there is one YAML block per plan/phase/size (no per-platform split).

---

## 3. Plans

Common to every plan and phase (S-HQ #rules, S-HMA #master-trading-rules, S-FAQ):

- **Trading day and holding.** The trading day runs 5:00 PM – 4:00 PM CT (see §4). "All positions must be closed by 3:59 PM (CT)." "Holding positions overnight is strictly prohibited."
- **Contracts.** "Standard and micro contracts can be traded simultaneously. One standard contract is equivalent to ten micro contracts." "Traders may hold several positions at the same time."
- **Commissions.** "Commissions are included in balance, P&L and risk calculations." (Master; the Option 1 and Option 2 definitions also include commissions)
- **Option 1 trailing drawdown** (S-HQ #option-1): "The trailing drawdown follows your account’s highest value throughout the trading day, including both realized and unrealized (open) profits. It is calculated in real time and includes commissions. … If your account balance falls below the drawdown threshold, the account is suspended." → `trailing_intraday`.
- **Option 2 EOD drawdown** (S-HQ #option-2): "The End-of-Day (EOD) drawdown is updated once per trading day, after the market closes. Unlike a trailing drawdown, it is based only on your closing account balance. Intraday fluctuations do not affect your threshold." → `trailing_eod`.
- **Option 2 daily loss limit** (S-HQ #option-2): "It is calculated using your realized and unrealized P&L, including commissions, between 5:00 PM and 4:00 PM CT. … If your Daily Loss Limit is reached, trading is disabled for the rest of the trading day. This is not considered a rule violation, and trading automatically resumes at the start of the next trading session." → `soft_pause`.
- **Option 1 has no daily loss limit.** The pricing card shows "No daily loss limit" on every Trailing card (S-PRICE).

### 3.1 Qualification Account (evaluation)

Source: S-PRICE, plan cards (Qualification tab). Option 2 values are from S-HQ #option-2. Lock wording is from S-HMA.

| Size | Drawdown (Opt 1 "Drawdown up to" / Opt 2 "EOD Drawdown") | Opt 1 contracts | Opt 2 contracts ("Contracts EOD" = top of scaling plan) | Opt 2 daily loss limit | Profit target | Min. trading days |
|---|---|---|---|---|---|---|
| $25,000 | $1,500 | 3 | 3 (scaling: up to $1,500 CoH: 2 • $1,501+: 3) | $500 | $1,500 | None |
| $50,000 | $2,500 | 7 | 7 (up to $1,500: 2 • $1,501–4,000: 4 • $4,001+: 7) | $1,100 | $3,000 | None |
| $100,000 | $3,000 | 12 | 12 (up to $2,000: 3 • $2,001–3,000: 5 • $3,001–5,000: 8 • $5,001+: 12) | $2,200 | $6,000 | None |
| $150,000 | $4,500 | 15 | 15 (up to $4,000: 5 • $4,001–8,000: 8 • $8,001–12,000: 10 • $12,001+: 15) | $3,300 | $9,000 | None |

- **Option 1 limit.** "Full contract size from day one — no scaling steps." (S-PRICE)
- **Option 2 scaling.** "Your maximum position size is based on your Cash on Hand. As your account grows, you can trade more contracts. If your balance decreases, your contract limit is reduced accordingly." (S-HQ #option-2). Cash on Hand is "the amount by which your balance exceeds the starting balance" (S-HMA).
- **Minimum days.** "There is no minimum trading day requirement to pass the Qualification Account." "A trading day is counted only when a trader opens at least one trade." (S-HQ #rules)
- **Lock.** No lock is stated for the Qualification stage. The lock at "starting balance + $100" is published for the Master Account (and Fast Track) only. Gap G3.
- **Consistency.** No consistency rule is published for the Qualification Account (none stated, and no "no consistency" statement either, unlike Momentum). Gap G4.
- **Access.** "Access valid for 30 days" (S-PRICE). Reset after a drawdown breach: "Your previous profits, losses and trading days are cleared." (S-HQ #reset)
- **Micros.** No per-size micros figure is published for this plan; only the ratio "1 mini = 10 micros". The YAML leaves `micros: null`, with no derived value.
- **Pricing card wording.** The Option 2 card shows "Contracts EOD: 3/7/12/15". These equal the **top** tier of the scaling plan, not the starting limit. See §9 N2.

### 3.2 Master Account (funded, simulated)

Source: [S-HMA #master-trading-rules](https://bulenox.com/help-center/master#master-trading-rules). "Your Master Account uses the same account size and Option selected during Qualification … The main trading conditions of your Qualification Account continue to apply. The key difference is that the drawdown on a Master Account eventually stops moving and becomes permanently fixed."

| Size | Drawdown | Initial threshold | Permanent threshold | Account value required to lock | Opt 1 contracts | Opt 2 scaling (max) | Opt 2 DLL |
|---|---|---|---|---|---|---|---|
| $25,000 | $1,500 | $23,500 | $25,100 | $26,600 | 3 | 2 → 3 | $500 |
| $50,000 | $2,500 | $47,500 | $50,100 | $52,600 | 7 | 2 → 4 → 7 | $1,100 |
| $100,000 | $3,000 | $97,000 | $100,100 | $103,100 | 12 | 3 → 5 → 8 → 12 | $2,200 |
| $150,000 | $4,500 | $145,500 | $150,100 | $154,600 | 15 | 5 → 8 → 10 → 15 | $3,300 |

- **Lock.** "The drawdown becomes permanently fixed when it reaches starting balance + $100."
- **Option 2 DLL.** "The Daily Loss Limit is permanently removed once the EOD Drawdown locks at the starting balance + $100." Hitting it means "trading is disabled for the remainder of the session; the event is not considered an account violation".
- **Breach.** "exceeding the drawdown threshold closes the Master Account." A Master reset is available for an individually calculated fee.
- **Option 1 contract limits** are the Qualification values: "Open positions must remain within the contract limit for your account". The table above repeats the Option 1 limits from S-PRICE because S-HMA states that Qualification conditions continue.
- **Consistency (payout condition, not a breach).** "Your most profitable trading day must not represent more than 40% of your Total P&L for the applicable payout period. … Consistency Percentage = Best Day P&L ÷ Total P&L × 100. The result must be 40% or less." "If the Consistency Rule is not met: the account is not closed; no trading rule is considered violated". "For the first payout, the Safety Threshold Reserve is included in the Consistency Rule calculation." (S-HMA #withdrawal-rules)
- **Payout rules that affect trading.** "complete at least 10 individual trading days; request at least $1,000; maintain the required Safety Threshold Reserve; meet the 40% Consistency Rule". The Safety Threshold Reserve is $1,600 / $2,600 / $3,100 / $4,600 for 25K / 50K / 100K / 150K, "the minimum amount that must remain above the starting balance after a payout". Maximum payout for payouts 1–3 is $1,000 / $1,500 / $1,750 / $2,000. From the 4th payout, the request "must be less than 50% of the total profit on the Master Account".
- **Inactivity.** "The Master Account requires executing at least one trade per week and should not be inactive for more than a week." (S-HMA, Inactivity Rule)

### 3.3 Fast Track Account (funded, simulated; no qualification)

Source: [S-HFT #ft-parameters](https://bulenox.com/help-center/fast-track#ft-parameters). The same values appear on the S-PRICE Fast Track cards.

| Size | Opt 1 trailing DD | Opt 1 contract limit | Opt 2 EOD DD | Opt 2 DLL | Opt 2 contract limit | First payout profit goal |
|---|---|---|---|---|---|---|
| $25,000 | $1,000 | 3 minis (30 micros) | $1,000 | no daily loss limit | 2 minis (20 micros) | $1,500 |
| $50,000 | $2,000 | 7 minis (70 micros) | $2,000 | $1,200 | 4 minis (40 micros) | $3,000 |
| $100,000 | $4,000 | 12 minis (120 micros) | $4,000 | $2,500 | 8 minis (80 micros) | $6,000 |
| $150,000 | $5,500 | 15 minis (150 micros) | $5,500 | $3,300 | 12 minis (120 micros) | $9,000 |

- **Option definitions.** "Option 1 uses the trailing drawdown. Option 2 uses the end-of-day drawdown together with a daily loss limit and has no scaling plan."
- **Lock.** "On both options the drawdown follows the balance upward and then stops for good once it reaches the starting balance plus $100. From that point it is a fixed floor and no longer moves."
- **Option 2 DLL.** "On Option 2 the daily loss limit stays active for the whole life of the account and is never lifted. The $25,000 Option 2 account is the exception: it has no daily loss limit." The Fast Track page does not restate whether hitting the DLL pauses the day or closes the account. The generic Option 2 text on S-PRICE says "The daily loss limit pauses your trading for the day instead of closing the account." The YAML records `soft_pause` from that generic text and lists the kind as not confirmed (N1).
- **Consistency (payout condition).** "Your highest profitable day divided by the net profit of the payout cycle. The maximum is 20% for the first payout, 25% for the second and 30% from the third onwards. Missing it is not a breach — keep trading until the account comes into line." (#ft-consistency)
- **Minimum days.** "There is no minimum number of trading days." Subsequent payout goals are $1,000 / $2,000 / $3,000 / $4,500 in new profit. "A payout may not bring the balance to or below the drawdown threshold."
- **Path to live.** "After at least three payouts you may be reviewed for transition to a live Funded Account. The transition is not automatic."

### 3.4 Momentum Account (evaluation) and Momentum Master (funded, simulated)

Source: [S-HMO #mo-parameters](https://bulenox.com/help-center/momentum#mo-parameters). The table header is "Account Parameters". It is not labelled by stage; see N3.

| Size | Profit target | Opt 1 trailing DD | Opt 1 contract limit | Opt 2 EOD DD | Opt 2 DLL | Opt 2 contract limit |
|---|---|---|---|---|---|---|
| $25,000 | $1,500 | $1,000 | 3 minis (30 micros) | $1,000 | $600 | 2 minis (20 micros) |
| $50,000 | $3,000 | $2,000 | 7 minis (70 micros) | $2,000 | $1,200 | 4 minis (40 micros) |
| $100,000 | $6,000 | $3,000 | 12 minis (120 micros) | $3,000 | $2,500 | 8 minis (80 micros) |
| $150,000 | $9,000 | $4,500 | 15 minis (150 micros) | $4,500 | $3,300 | 12 minis (120 micros) |

- **Qualification-stage rules** (#mo-rules): "There is no minimum number of trading days … There is no consistency requirement at this stage. News trading is allowed within all risk limits. All positions must be closed before the session close; overnight and weekend positions are prohibited. Qualification profits do not carry over — your Momentum Master starts at its stated balance." Also: "Momentum qualification is valid for 30 calendar days from activation."
- **Momentum Master.** "Activated free of charge as soon as your Momentum account is approved."
  - Payout cycle: "at least five qualifying profitable days, the 35% consistency rule, the minimum payout balance …".
  - Qualifying day: net profit ≥ $100 / $150 / $200 / $250 for 25K / 50K / 100K / 150K.
  - Consistency: "Your best trading day may not exceed 35% of the net profit of the payout cycle. Missing it is not a breach".
  - Minimum balance for payout: $26,500 / $53,000 / $104,500 / $156,500. Safety Net Reserve (STR): $1,000 / $2,000 / $3,000 / $4,500.
  - **Not published for Momentum Master:** whether the drawdown locks (or at what level), whether the DLL stays, and the contract limit. The YAML uses the "Account Parameters" values and marks them not confirmed (gap G5).

### 3.5 Funded Account (live)

Source: [S-HFU](https://bulenox.com/help-center/funded#consolidation).

- **Eligibility.** "After completing three successful payouts on your active Master Accounts, your accounts may be considered for transition to a Funded Account. The transition is completed at Bulenox’s discretion and is not automatic after the third payout."
- **Per-account transition.** "Each eligible Master-level Account transitions into its own separate Funded Account." Additional active Master-level accounts with at least 1 successful Reward may also transition, subject to Risk Management approval.
- **Initial drawdown.** "each Funded Account receives its own Initial End-of-Day (EOD) Drawdown": $1,500 / $2,000 / $3,000 / $4,500 for Master size 25K / 50K / 100K / 150K.
- **Pending accounts.** "Pending Master-level Account — remains pending and cannot be activated while Trader maintains an active Funded Account."
- **Not published:** the Funded Account's DLL, contract limits, drawdown behaviour after the initial value, consistency, platforms and trading day. The Funded Account Agreement is not public. Gap G6.

---

## 4. Trading-day reset

> "The trading day starts at 5:00 PM and ends at 4:00 PM the following day, North American Central Time (CT). Weekends and public holidays are not counted as trading days. All positions must be closed by 3:59 PM (CT)." — [S-HQ #rules](https://bulenox.com/help-center/qualification#rules)

> "The trading session runs from 5:00 PM CT to 4:00 PM CT the following day. All positions must be closed by 3:59 PM CT." — [S-HMA](https://bulenox.com/help-center/master#master-trading-rules)

The FAQ says the same: "The trading day runs from 5:00 pm to 4:00 pm CT, and all positions must be closed by 3:59 pm CT." The homepage says "Trade until 15:59 CT — later than most."

- **Reset:** **5:00 PM CT** (`17:00`, as published "North American Central Time (CT)"). The daily loss limit window is "between 5:00 PM and 4:00 PM CT". The EOD drawdown updates "once per trading day, after the market closes".
- **Flat by:** **3:59 PM CT** every day. There is no overnight or weekend holding.
- **Holidays / early close:** only "Weekends and public holidays are not counted as trading days." No holiday calendar or early-close rule is published (gap G8). Rithmic weekend maintenance: "During the maintenance period, you may be unable to log in." (S-HCO)

---

## 5. Trade copying stance

Classification: **conditional**. Trade copiers are permitted, but only as user-built tools for the account owner's personal use. A $100/month fee applies when connecting through a third-party API or composite software.

> "Can I use bots, algos or trade copiers?
> Yes. Bots, algorithms and trade copiers are permitted. If you connect to Rithmic through a third-party API or composite software, an additional $100 per month applies.
> Only user-built tools intended solely for the account owner’s personal use are permitted. Commercial, shared, rented or publicly distributed automation tools are not allowed." — https://bulenox.com/faq (accessed 2026-10-23; rendered text checked in the browser, evidence `raw/faq-portal-bots-item.html`)

> "Trade Copiers & Third-Party Software — If a trader chooses to connect to Rithmic through a Third-Party API or Trader Composite Software, a $100 monthly fee applies. Third-Party API and Trader Composite Software are provided by independent companies and are not operated or controlled by Bulenox." — https://bulenox.com/help-center/qualification#rules

**Copying across the same person's accounts.** No sentence names copying between one's own Bulenox accounts explicitly. Copiers are "permitted", and all of a trader's accounts sit under one Rithmic User ID:

> "There is no limit to the number of Qualification Accounts you can own. You may hold up to five active Master-level accounts in total — Master, Fast Track and Momentum Master in any combination. Every account is linked to the same Rithmic User ID" — https://bulenox.com/help-center/qualification#multiple-accounts

> "Balances, profits and drawdown limits cannot be combined between accounts." — https://bulenox.com/help-center/master#multiple-master-accounts

**Account-count limits**
- Qualification accounts: unlimited.
- Master-level accounts (Master, Fast Track and Momentum Master combined): **5 active**. "All five can be activated right away — there is no unlocking sequence."
- One profile per person: "Each trader may have only one Bulenox profile and one Rithmic User ID. Creating multiple profiles or using multiple User IDs is not permitted and may result in account suspension." (S-HQ #multiple-accounts)
- One profile per household: "you are the sole authorized Bulenox trader profile holder within your household, and no other person residing at the same physical address or sharing the same residential location operates or maintains a Bulenox trader profile, unless such person has obtained prior written approval from Bulenox Compliance." (S-TOU, User Representations F)

**Copying to or from accounts at other firms:** **not mentioned** on any official page (gap G2).

**Third-party copier products:** see conflict C1 in §9. The FAQ excludes commercial tools. The Terms of Use allow third-party trading technology after review and approval.

---

## 6. Automated trading stance

Classification: **conditional** (YAML `automated_trading_allowed: conditional`).

Quote 1 — FAQ (https://bulenox.com/faq, "Can I use bots, algos or trade copiers?", accessed 2026-10-23):

> "Yes. Bots, algorithms and trade copiers are permitted. If you connect to Rithmic through a third-party API or composite software, an additional $100 per month applies.
> Only user-built tools intended solely for the account owner’s personal use are permitted. Commercial, shared, rented or publicly distributed automation tools are not allowed.
> These tools are developed and maintained independently by their users, so their uptime, reliability and performance are outside our control, and we cannot provide support or accept responsibility for any issues arising from their use."

Quote 2 — Terms of Use (https://bulenox.com/legal/Terms_of_Use.pdf, "Updated October 2026", section "ALGORITHMS AND AUTO TRADING", accessed 2026-10-23):

> "You agree to not abuse the company provided service programs. It includes but is not limited to using “scalping” algorithms, DTC Protocol Bridge API or automated discretional trading in any of the trading environments. Open and close hundreds or thousand of rapid trades by using any trading algorithms. Implementation of reckless transactions in rough markets to benefit from lack of execution. If abuse is suspected, we reserve the right to refuse to claim any profit. To avoid performance discrepancies and/or meet applicable compliance requirements, any third-party algorithm, automated trading software, or other third-party trading technology must be reviewed and approved by the Bulenox management team before use. Approval may be subject to additional requirements or conditions established by Bulenox. If you elect to use Trader Composite Software, you shall be required to pay a third-party API/Trader Composite Software fee of one hundred dollars ($100) per month. Payment of this fee permits direct connectivity to the Rithmic platform through an approved third-party API or Trader Composite interface and does not constitute approval of any specific algorithm, strategy, or software. Any such software remains subject to Bulenox's review and approval requirements."

`automation_rule_text` in every YAML block holds Quote 1 (all three paragraphs) and, after a blank line, the first part of Quote 2, up to "established by Bulenox." Both are exact captured text. The fee sentences of Quote 2 are left out of the warning text, but they are quoted above. The PO may decide to show only one quote; because of C1, I did not choose.

PO checklist:

- **(i) Classification:** `conditional`. Automation is allowed. Per the FAQ, it must be a user-built tool for personal use only. Per the ToU, third-party software needs management review and approval, and "abuse" patterns are excluded.
- **(ii) Specific mentions:**
  - **Trade copiers:** "Bots, algorithms and trade copiers are permitted." (FAQ)
  - **Semi-automated trading:** not mentioned by that name. The ToU lists "automated discretional trading" among abuses.
  - **Webhooks / alerts:** not mentioned.
  - **Third-party API / composite software:** quoted above ($100/month; the ToU requires approval).
  - **"DTC Protocol Bridge API":** named as abuse in the ToU.
  - **ATM / automatic brackets:** no ATM rule. The only related text is the OCO note: "OCO — One Cancels Other — functionality is simulated locally on your computer. If NinjaTrader is offline or disconnected from Rithmic, the second part of an OCO order — the target or stop order — may not be canceled. … Bulenox is not responsible for losses caused by additional positions opened as a result of uncanceled OCO orders." ([S-HCO #ninjatrader-connection](https://bulenox.com/help-center/connection#ninjatrader-connection))
  - **NinjaTrader setting:** "Allow Plugins is set to On" is part of the Rithmic login setup (S-HCO).
- **(iii) Difference by phase:** none published. The FAQ and ToU are general. The ToU says "in any of the trading environments". The copier/third-party fee text sits under the Qualification rules. Nothing separate is published for Master, Fast Track, Momentum or the live Funded Account, so the live stance is not confirmed (gap G6).

### 6.1 VPS / server / VPN and personal device (PO rev. 3)

**Not mentioned.** There is no text about VPS, virtual/remote servers, VPN, IP or location restrictions on trading, or a requirement that orders come from the trader's personal device.

- **Where I searched:** all 9 Help Center categories (CMS JSON, every language), the FAQ, the Terms of Use, the Risk Disclosure, the homepage and accounts-pricing. Keywords: VPS, VPN, virtual, device, server, IP address, remote, cloud, computer.
- **Only nearby texts found:**
  - The Rithmic gateway choice ("You may also select another server based on your location: Europe or Frankfurt for Europe; Tokyo or Singapore for Asia").
  - The OCO note ("simulated locally on your computer").
  - The ToU right to "block certain IP addresses from accessing the Sites and Company Services". This is about the website, not trading.

YAML: `vps_allowed: unknown`, `personal_device_required: unknown`. Gap G1.

---

## 7. Other rules that affect routing or risk

- **News trading: allowed.** "News trading is allowed within all risk limits. Trade straight through the economic calendar. CPI, FOMC, NFP — no blackout windows, no forced position closing, no penalties. The only limits are your drawdown and your position size." ([S-HFT #ft-rules-sessions](https://bulenox.com/help-center/fast-track#ft-rules-sessions); same text on S-HOME and S-FAQ)
- **Overnight / weekend: prohibited.** All positions must be closed by 3:59 PM CT (all programs).
- **Mandatory stop loss:** not mentioned.
- **Hedging / correlated positions:** not mentioned.
- **Min/max trade duration:** not mentioned. The ToU "abuse" list includes "Open and close hundreds or thousand of rapid trades by using any trading algorithms" and "“scalping” algorithms".
- **Scaling:**
  - Qualification and Master **Option 2**: Dynamic Scaling Plan on Cash on Hand, in both directions (§3.1, §3.2).
  - Option 1: no scaling.
  - Fast Track and Momentum: no scaling in either option ("Option 2 — no scaling").
- **Max accounts:** unlimited Qualification accounts. Up to **5** active Master-level accounts (Master, Fast Track and Momentum Master combined). One profile and one Rithmic User ID per person; one profile per household unless Compliance approves (§5).
- **Inactivity:** Master Account needs at least one trade per week, otherwise "it could be put on hold or disabled".
- **Breach behaviour:**
  - Qualification: drawdown breach → "the account is suspended"; reset or buy a new account.
  - Master: "exceeding the drawdown threshold closes the Master Account".
  - DLL (Option 2): pause for the session.
- **Market data status:** Rithmic professional vs non-professional, "locked for the life of the account — Qualification, Master and Funded" (S-PRICE). Fees are not needed, but see N4.
- **Restricted countries:** a list is published in S-FAQ ("Is my country supported?"). It does not affect routing.

---

## 8. Machine-readable blocks

44 blocks: Qualification (2 options × 4 sizes), Master (8), Fast Track (8), Momentum qualification stage (8), Momentum Master (8), Funded live (4). `null` = not found. `not_confirmed_fields` lists both conflicting fields and fields that are unverified for that phase; both ship as "not confirmed". All blocks parse with PyYAML `safe_load`.

```yaml
# Qualification · Option 1 · 25K
firm: bulenox
plan: "Qualification Account — Option 1 (No Scaling, Trailing Drawdown)"
phase: evaluation
size_label: "25K"
size_usd: "25000.00"
instruments: [MNQ, NQ]
platforms: [Rithmic, "R | Trader Pro", NinjaTrader, Quantower, "Sierra Chart", MultiCharts, ATAS, Bookmap, MotiveWave, "Investor/RT", "Photon Trader", "Trade Navigator", TSLab, ScalpTool, QST, "Medved Trader", OverCharts, QScalp, VolFix, "Inside Edge", "Optimus Flow", Tiger.Trade]
drawdown: { amount_usd: "1500.00", type: trailing_intraday, stops_trailing_at: "not stated for the Qualification Account (the starting balance + $100 lock is published for the Master Account only)" }
daily_loss_limit: { amount_usd: null, kind: none }
profit_target_usd: "1500.00"
consistency_rule: { text: null, threshold_pct: null }
max_position: { minis: "3", micros: null, scaling: "none — Option 1 'Full contract size from day one — no scaling steps'; 1 standard contract = 10 micro contracts (micros total not published per size for this plan)" }
min_trading_days: 0
trading_day_reset: { time: "17:00", tz_as_published: "North American Central Time (CT)" }   # "starts at 5:00 PM and ends at 4:00 PM the following day"; positions closed by 3:59 PM CT
copy_trading_allowed: conditional   # copiers permitted; only user-built tools for the account owner's personal use
automated_trading_allowed: conditional
automation_rule_text: |-
    Yes. Bots, algorithms and trade copiers are permitted. If you connect to Rithmic through a third-party API or composite software, an additional $100 per month applies.
    Only user-built tools intended solely for the account owner’s personal use are permitted. Commercial, shared, rented or publicly distributed automation tools are not allowed.
    These tools are developed and maintained independently by their users, so their uptime, reliability and performance are outside our control, and we cannot provide support or accept responsibility for any issues arising from their use.

    You agree to not abuse the company provided service programs. It includes but is not limited to using “scalping” algorithms, DTC Protocol Bridge API or automated discretional trading in any of the trading environments. Open and close hundreds or thousand of rapid trades by using any trading algorithms. Implementation of reckless transactions in rough markets to benefit from lack of execution. If abuse is suspected, we reserve the right to refuse to claim any profit. To avoid performance discrepancies and/or meet applicable compliance requirements, any third-party algorithm, automated trading software, or other third-party trading technology must be reviewed and approved by the Bulenox management team before use. Approval may be subject to additional requirements or conditions established by Bulenox.
vps_allowed: unknown               # not mentioned on any official page (see §9 G1)
personal_device_required: unknown  # not mentioned on any official page (see §9 G1)
status: partial
not_confirmed_fields:
  - "consistency_rule (none published for the Qualification Account)"
  - "drawdown.stops_trailing_at (not stated for Qualification)"
  - "automation_rule_text (third-party / commercial tools: FAQ 'Only user-built tools ... Commercial ... not allowed' vs Terms of Use 'any third-party algorithm ... must be reviewed and approved' — see §9 C1)"
sources:
  - { field: drawdown, url: "https://bulenox.com/accounts-pricing", accessed: "2026-10-23" }
  - { field: drawdown, url: "https://bulenox.com/help-center/master#master-trading-rules", accessed: "2026-10-23" }
  - { field: drawdown.type, url: "https://bulenox.com/help-center/qualification#option-1", accessed: "2026-10-23" }
  - { field: daily_loss_limit, url: "https://bulenox.com/accounts-pricing", accessed: "2026-10-23" }
  - { field: profit_target_usd, url: "https://bulenox.com/accounts-pricing", accessed: "2026-10-23" }
  - { field: max_position, url: "https://bulenox.com/accounts-pricing", accessed: "2026-10-23" }
  - { field: max_position, url: "https://bulenox.com/help-center/qualification#rules", accessed: "2026-10-23" }
  - { field: min_trading_days, url: "https://bulenox.com/help-center/qualification#rules", accessed: "2026-10-23" }
  - { field: trading_day_reset, url: "https://bulenox.com/help-center/qualification#rules", accessed: "2026-10-23" }
  - { field: copy_trading_allowed, url: "https://bulenox.com/faq", accessed: "2026-10-23" }
  - { field: automated_trading_allowed, url: "https://bulenox.com/faq", accessed: "2026-10-23" }
  - { field: automated_trading_allowed, url: "https://bulenox.com/legal/Terms_of_Use.pdf", accessed: "2026-10-23" }
  - { field: platforms, url: "https://bulenox.com/", accessed: "2026-10-23" }
  - { field: platforms, url: "https://bulenox.com/faq", accessed: "2026-10-23" }
  - { field: vps_allowed, url: "not mentioned — searched help center (all 9 categories), FAQ, Terms of Use, Risk Disclosure, homepage, accounts-pricing", accessed: "2026-10-23" }
  - { field: personal_device_required, url: "not mentioned — same search", accessed: "2026-10-23" }
```

```yaml
# Qualification · Option 1 · 50K
firm: bulenox
plan: "Qualification Account — Option 1 (No Scaling, Trailing Drawdown)"
phase: evaluation
size_label: "50K"
size_usd: "50000.00"
instruments: [MNQ, NQ]
platforms: [Rithmic, "R | Trader Pro", NinjaTrader, Quantower, "Sierra Chart", MultiCharts, ATAS, Bookmap, MotiveWave, "Investor/RT", "Photon Trader", "Trade Navigator", TSLab, ScalpTool, QST, "Medved Trader", OverCharts, QScalp, VolFix, "Inside Edge", "Optimus Flow", Tiger.Trade]
drawdown: { amount_usd: "2500.00", type: trailing_intraday, stops_trailing_at: "not stated for the Qualification Account (the starting balance + $100 lock is published for the Master Account only)" }
daily_loss_limit: { amount_usd: null, kind: none }
profit_target_usd: "3000.00"
consistency_rule: { text: null, threshold_pct: null }
max_position: { minis: "7", micros: null, scaling: "none — Option 1 'Full contract size from day one — no scaling steps'; 1 standard contract = 10 micro contracts (micros total not published per size for this plan)" }
min_trading_days: 0
trading_day_reset: { time: "17:00", tz_as_published: "North American Central Time (CT)" }   # "starts at 5:00 PM and ends at 4:00 PM the following day"; positions closed by 3:59 PM CT
copy_trading_allowed: conditional   # copiers permitted; only user-built tools for the account owner's personal use
automated_trading_allowed: conditional
automation_rule_text: |-
    Yes. Bots, algorithms and trade copiers are permitted. If you connect to Rithmic through a third-party API or composite software, an additional $100 per month applies.
    Only user-built tools intended solely for the account owner’s personal use are permitted. Commercial, shared, rented or publicly distributed automation tools are not allowed.
    These tools are developed and maintained independently by their users, so their uptime, reliability and performance are outside our control, and we cannot provide support or accept responsibility for any issues arising from their use.

    You agree to not abuse the company provided service programs. It includes but is not limited to using “scalping” algorithms, DTC Protocol Bridge API or automated discretional trading in any of the trading environments. Open and close hundreds or thousand of rapid trades by using any trading algorithms. Implementation of reckless transactions in rough markets to benefit from lack of execution. If abuse is suspected, we reserve the right to refuse to claim any profit. To avoid performance discrepancies and/or meet applicable compliance requirements, any third-party algorithm, automated trading software, or other third-party trading technology must be reviewed and approved by the Bulenox management team before use. Approval may be subject to additional requirements or conditions established by Bulenox.
vps_allowed: unknown               # not mentioned on any official page (see §9 G1)
personal_device_required: unknown  # not mentioned on any official page (see §9 G1)
status: partial
not_confirmed_fields:
  - "consistency_rule (none published for the Qualification Account)"
  - "drawdown.stops_trailing_at (not stated for Qualification)"
  - "automation_rule_text (third-party / commercial tools: FAQ 'Only user-built tools ... Commercial ... not allowed' vs Terms of Use 'any third-party algorithm ... must be reviewed and approved' — see §9 C1)"
sources:
  - { field: drawdown, url: "https://bulenox.com/accounts-pricing", accessed: "2026-10-23" }
  - { field: drawdown, url: "https://bulenox.com/help-center/master#master-trading-rules", accessed: "2026-10-23" }
  - { field: drawdown.type, url: "https://bulenox.com/help-center/qualification#option-1", accessed: "2026-10-23" }
  - { field: daily_loss_limit, url: "https://bulenox.com/accounts-pricing", accessed: "2026-10-23" }
  - { field: profit_target_usd, url: "https://bulenox.com/accounts-pricing", accessed: "2026-10-23" }
  - { field: max_position, url: "https://bulenox.com/accounts-pricing", accessed: "2026-10-23" }
  - { field: max_position, url: "https://bulenox.com/help-center/qualification#rules", accessed: "2026-10-23" }
  - { field: min_trading_days, url: "https://bulenox.com/help-center/qualification#rules", accessed: "2026-10-23" }
  - { field: trading_day_reset, url: "https://bulenox.com/help-center/qualification#rules", accessed: "2026-10-23" }
  - { field: copy_trading_allowed, url: "https://bulenox.com/faq", accessed: "2026-10-23" }
  - { field: automated_trading_allowed, url: "https://bulenox.com/faq", accessed: "2026-10-23" }
  - { field: automated_trading_allowed, url: "https://bulenox.com/legal/Terms_of_Use.pdf", accessed: "2026-10-23" }
  - { field: platforms, url: "https://bulenox.com/", accessed: "2026-10-23" }
  - { field: platforms, url: "https://bulenox.com/faq", accessed: "2026-10-23" }
  - { field: vps_allowed, url: "not mentioned — searched help center (all 9 categories), FAQ, Terms of Use, Risk Disclosure, homepage, accounts-pricing", accessed: "2026-10-23" }
  - { field: personal_device_required, url: "not mentioned — same search", accessed: "2026-10-23" }
```

```yaml
# Qualification · Option 1 · 100K
firm: bulenox
plan: "Qualification Account — Option 1 (No Scaling, Trailing Drawdown)"
phase: evaluation
size_label: "100K"
size_usd: "100000.00"
instruments: [MNQ, NQ]
platforms: [Rithmic, "R | Trader Pro", NinjaTrader, Quantower, "Sierra Chart", MultiCharts, ATAS, Bookmap, MotiveWave, "Investor/RT", "Photon Trader", "Trade Navigator", TSLab, ScalpTool, QST, "Medved Trader", OverCharts, QScalp, VolFix, "Inside Edge", "Optimus Flow", Tiger.Trade]
drawdown: { amount_usd: "3000.00", type: trailing_intraday, stops_trailing_at: "not stated for the Qualification Account (the starting balance + $100 lock is published for the Master Account only)" }
daily_loss_limit: { amount_usd: null, kind: none }
profit_target_usd: "6000.00"
consistency_rule: { text: null, threshold_pct: null }
max_position: { minis: "12", micros: null, scaling: "none — Option 1 'Full contract size from day one — no scaling steps'; 1 standard contract = 10 micro contracts (micros total not published per size for this plan)" }
min_trading_days: 0
trading_day_reset: { time: "17:00", tz_as_published: "North American Central Time (CT)" }   # "starts at 5:00 PM and ends at 4:00 PM the following day"; positions closed by 3:59 PM CT
copy_trading_allowed: conditional   # copiers permitted; only user-built tools for the account owner's personal use
automated_trading_allowed: conditional
automation_rule_text: |-
    Yes. Bots, algorithms and trade copiers are permitted. If you connect to Rithmic through a third-party API or composite software, an additional $100 per month applies.
    Only user-built tools intended solely for the account owner’s personal use are permitted. Commercial, shared, rented or publicly distributed automation tools are not allowed.
    These tools are developed and maintained independently by their users, so their uptime, reliability and performance are outside our control, and we cannot provide support or accept responsibility for any issues arising from their use.

    You agree to not abuse the company provided service programs. It includes but is not limited to using “scalping” algorithms, DTC Protocol Bridge API or automated discretional trading in any of the trading environments. Open and close hundreds or thousand of rapid trades by using any trading algorithms. Implementation of reckless transactions in rough markets to benefit from lack of execution. If abuse is suspected, we reserve the right to refuse to claim any profit. To avoid performance discrepancies and/or meet applicable compliance requirements, any third-party algorithm, automated trading software, or other third-party trading technology must be reviewed and approved by the Bulenox management team before use. Approval may be subject to additional requirements or conditions established by Bulenox.
vps_allowed: unknown               # not mentioned on any official page (see §9 G1)
personal_device_required: unknown  # not mentioned on any official page (see §9 G1)
status: partial
not_confirmed_fields:
  - "consistency_rule (none published for the Qualification Account)"
  - "drawdown.stops_trailing_at (not stated for Qualification)"
  - "automation_rule_text (third-party / commercial tools: FAQ 'Only user-built tools ... Commercial ... not allowed' vs Terms of Use 'any third-party algorithm ... must be reviewed and approved' — see §9 C1)"
sources:
  - { field: drawdown, url: "https://bulenox.com/accounts-pricing", accessed: "2026-10-23" }
  - { field: drawdown, url: "https://bulenox.com/help-center/master#master-trading-rules", accessed: "2026-10-23" }
  - { field: drawdown.type, url: "https://bulenox.com/help-center/qualification#option-1", accessed: "2026-10-23" }
  - { field: daily_loss_limit, url: "https://bulenox.com/accounts-pricing", accessed: "2026-10-23" }
  - { field: profit_target_usd, url: "https://bulenox.com/accounts-pricing", accessed: "2026-10-23" }
  - { field: max_position, url: "https://bulenox.com/accounts-pricing", accessed: "2026-10-23" }
  - { field: max_position, url: "https://bulenox.com/help-center/qualification#rules", accessed: "2026-10-23" }
  - { field: min_trading_days, url: "https://bulenox.com/help-center/qualification#rules", accessed: "2026-10-23" }
  - { field: trading_day_reset, url: "https://bulenox.com/help-center/qualification#rules", accessed: "2026-10-23" }
  - { field: copy_trading_allowed, url: "https://bulenox.com/faq", accessed: "2026-10-23" }
  - { field: automated_trading_allowed, url: "https://bulenox.com/faq", accessed: "2026-10-23" }
  - { field: automated_trading_allowed, url: "https://bulenox.com/legal/Terms_of_Use.pdf", accessed: "2026-10-23" }
  - { field: platforms, url: "https://bulenox.com/", accessed: "2026-10-23" }
  - { field: platforms, url: "https://bulenox.com/faq", accessed: "2026-10-23" }
  - { field: vps_allowed, url: "not mentioned — searched help center (all 9 categories), FAQ, Terms of Use, Risk Disclosure, homepage, accounts-pricing", accessed: "2026-10-23" }
  - { field: personal_device_required, url: "not mentioned — same search", accessed: "2026-10-23" }
```

```yaml
# Qualification · Option 1 · 150K
firm: bulenox
plan: "Qualification Account — Option 1 (No Scaling, Trailing Drawdown)"
phase: evaluation
size_label: "150K"
size_usd: "150000.00"
instruments: [MNQ, NQ]
platforms: [Rithmic, "R | Trader Pro", NinjaTrader, Quantower, "Sierra Chart", MultiCharts, ATAS, Bookmap, MotiveWave, "Investor/RT", "Photon Trader", "Trade Navigator", TSLab, ScalpTool, QST, "Medved Trader", OverCharts, QScalp, VolFix, "Inside Edge", "Optimus Flow", Tiger.Trade]
drawdown: { amount_usd: "4500.00", type: trailing_intraday, stops_trailing_at: "not stated for the Qualification Account (the starting balance + $100 lock is published for the Master Account only)" }
daily_loss_limit: { amount_usd: null, kind: none }
profit_target_usd: "9000.00"
consistency_rule: { text: null, threshold_pct: null }
max_position: { minis: "15", micros: null, scaling: "none — Option 1 'Full contract size from day one — no scaling steps'; 1 standard contract = 10 micro contracts (micros total not published per size for this plan)" }
min_trading_days: 0
trading_day_reset: { time: "17:00", tz_as_published: "North American Central Time (CT)" }   # "starts at 5:00 PM and ends at 4:00 PM the following day"; positions closed by 3:59 PM CT
copy_trading_allowed: conditional   # copiers permitted; only user-built tools for the account owner's personal use
automated_trading_allowed: conditional
automation_rule_text: |-
    Yes. Bots, algorithms and trade copiers are permitted. If you connect to Rithmic through a third-party API or composite software, an additional $100 per month applies.
    Only user-built tools intended solely for the account owner’s personal use are permitted. Commercial, shared, rented or publicly distributed automation tools are not allowed.
    These tools are developed and maintained independently by their users, so their uptime, reliability and performance are outside our control, and we cannot provide support or accept responsibility for any issues arising from their use.

    You agree to not abuse the company provided service programs. It includes but is not limited to using “scalping” algorithms, DTC Protocol Bridge API or automated discretional trading in any of the trading environments. Open and close hundreds or thousand of rapid trades by using any trading algorithms. Implementation of reckless transactions in rough markets to benefit from lack of execution. If abuse is suspected, we reserve the right to refuse to claim any profit. To avoid performance discrepancies and/or meet applicable compliance requirements, any third-party algorithm, automated trading software, or other third-party trading technology must be reviewed and approved by the Bulenox management team before use. Approval may be subject to additional requirements or conditions established by Bulenox.
vps_allowed: unknown               # not mentioned on any official page (see §9 G1)
personal_device_required: unknown  # not mentioned on any official page (see §9 G1)
status: partial
not_confirmed_fields:
  - "consistency_rule (none published for the Qualification Account)"
  - "drawdown.stops_trailing_at (not stated for Qualification)"
  - "automation_rule_text (third-party / commercial tools: FAQ 'Only user-built tools ... Commercial ... not allowed' vs Terms of Use 'any third-party algorithm ... must be reviewed and approved' — see §9 C1)"
sources:
  - { field: drawdown, url: "https://bulenox.com/accounts-pricing", accessed: "2026-10-23" }
  - { field: drawdown, url: "https://bulenox.com/help-center/master#master-trading-rules", accessed: "2026-10-23" }
  - { field: drawdown.type, url: "https://bulenox.com/help-center/qualification#option-1", accessed: "2026-10-23" }
  - { field: daily_loss_limit, url: "https://bulenox.com/accounts-pricing", accessed: "2026-10-23" }
  - { field: profit_target_usd, url: "https://bulenox.com/accounts-pricing", accessed: "2026-10-23" }
  - { field: max_position, url: "https://bulenox.com/accounts-pricing", accessed: "2026-10-23" }
  - { field: max_position, url: "https://bulenox.com/help-center/qualification#rules", accessed: "2026-10-23" }
  - { field: min_trading_days, url: "https://bulenox.com/help-center/qualification#rules", accessed: "2026-10-23" }
  - { field: trading_day_reset, url: "https://bulenox.com/help-center/qualification#rules", accessed: "2026-10-23" }
  - { field: copy_trading_allowed, url: "https://bulenox.com/faq", accessed: "2026-10-23" }
  - { field: automated_trading_allowed, url: "https://bulenox.com/faq", accessed: "2026-10-23" }
  - { field: automated_trading_allowed, url: "https://bulenox.com/legal/Terms_of_Use.pdf", accessed: "2026-10-23" }
  - { field: platforms, url: "https://bulenox.com/", accessed: "2026-10-23" }
  - { field: platforms, url: "https://bulenox.com/faq", accessed: "2026-10-23" }
  - { field: vps_allowed, url: "not mentioned — searched help center (all 9 categories), FAQ, Terms of Use, Risk Disclosure, homepage, accounts-pricing", accessed: "2026-10-23" }
  - { field: personal_device_required, url: "not mentioned — same search", accessed: "2026-10-23" }
```

```yaml
# Qualification · Option 2 · 25K
firm: bulenox
plan: "Qualification Account — Option 2 (EOD Account)"
phase: evaluation
size_label: "25K"
size_usd: "25000.00"
instruments: [MNQ, NQ]
platforms: [Rithmic, "R | Trader Pro", NinjaTrader, Quantower, "Sierra Chart", MultiCharts, ATAS, Bookmap, MotiveWave, "Investor/RT", "Photon Trader", "Trade Navigator", TSLab, ScalpTool, QST, "Medved Trader", OverCharts, QScalp, VolFix, "Inside Edge", "Optimus Flow", Tiger.Trade]
drawdown: { amount_usd: "1500.00", type: trailing_eod, stops_trailing_at: "not stated for the Qualification Account (lock at starting balance + $100 is published for the Master Account)" }
daily_loss_limit: { amount_usd: "500.00", kind: soft_pause }   # 'trading is disabled for the rest of the trading day. This is not considered a rule violation'
profit_target_usd: "1500.00"
consistency_rule: { text: null, threshold_pct: null }
max_position: { minis: "3", micros: null, scaling: "Dynamic Scaling Plan (both directions): Cash on Hand up to $1,500: 2 • $1,501+: 3; minis value = top tier ('Contracts EOD' on pricing card); 1 standard contract = 10 micro contracts (micros total not published per size for this plan)" }
min_trading_days: 0
trading_day_reset: { time: "17:00", tz_as_published: "North American Central Time (CT)" }   # "starts at 5:00 PM and ends at 4:00 PM the following day"; positions closed by 3:59 PM CT
copy_trading_allowed: conditional   # copiers permitted; only user-built tools for the account owner's personal use
automated_trading_allowed: conditional
automation_rule_text: |-
    Yes. Bots, algorithms and trade copiers are permitted. If you connect to Rithmic through a third-party API or composite software, an additional $100 per month applies.
    Only user-built tools intended solely for the account owner’s personal use are permitted. Commercial, shared, rented or publicly distributed automation tools are not allowed.
    These tools are developed and maintained independently by their users, so their uptime, reliability and performance are outside our control, and we cannot provide support or accept responsibility for any issues arising from their use.

    You agree to not abuse the company provided service programs. It includes but is not limited to using “scalping” algorithms, DTC Protocol Bridge API or automated discretional trading in any of the trading environments. Open and close hundreds or thousand of rapid trades by using any trading algorithms. Implementation of reckless transactions in rough markets to benefit from lack of execution. If abuse is suspected, we reserve the right to refuse to claim any profit. To avoid performance discrepancies and/or meet applicable compliance requirements, any third-party algorithm, automated trading software, or other third-party trading technology must be reviewed and approved by the Bulenox management team before use. Approval may be subject to additional requirements or conditions established by Bulenox.
vps_allowed: unknown               # not mentioned on any official page (see §9 G1)
personal_device_required: unknown  # not mentioned on any official page (see §9 G1)
status: partial
not_confirmed_fields:
  - "consistency_rule (none published for the Qualification Account)"
  - "drawdown.stops_trailing_at (not stated for Qualification)"
  - "automation_rule_text (third-party / commercial tools: FAQ 'Only user-built tools ... Commercial ... not allowed' vs Terms of Use 'any third-party algorithm ... must be reviewed and approved' — see §9 C1)"
sources:
  - { field: drawdown, url: "https://bulenox.com/accounts-pricing", accessed: "2026-10-23" }
  - { field: drawdown.type, url: "https://bulenox.com/help-center/qualification#option-2", accessed: "2026-10-23" }
  - { field: daily_loss_limit, url: "https://bulenox.com/help-center/qualification#option-2", accessed: "2026-10-23" }
  - { field: daily_loss_limit, url: "https://bulenox.com/accounts-pricing", accessed: "2026-10-23" }
  - { field: profit_target_usd, url: "https://bulenox.com/accounts-pricing", accessed: "2026-10-23" }
  - { field: max_position, url: "https://bulenox.com/help-center/qualification#option-2", accessed: "2026-10-23" }
  - { field: max_position, url: "https://bulenox.com/accounts-pricing", accessed: "2026-10-23" }
  - { field: min_trading_days, url: "https://bulenox.com/help-center/qualification#rules", accessed: "2026-10-23" }
  - { field: trading_day_reset, url: "https://bulenox.com/help-center/qualification#rules", accessed: "2026-10-23" }
  - { field: copy_trading_allowed, url: "https://bulenox.com/faq", accessed: "2026-10-23" }
  - { field: automated_trading_allowed, url: "https://bulenox.com/faq", accessed: "2026-10-23" }
  - { field: automated_trading_allowed, url: "https://bulenox.com/legal/Terms_of_Use.pdf", accessed: "2026-10-23" }
  - { field: platforms, url: "https://bulenox.com/", accessed: "2026-10-23" }
  - { field: platforms, url: "https://bulenox.com/faq", accessed: "2026-10-23" }
  - { field: vps_allowed, url: "not mentioned — searched help center (all 9 categories), FAQ, Terms of Use, Risk Disclosure, homepage, accounts-pricing", accessed: "2026-10-23" }
  - { field: personal_device_required, url: "not mentioned — same search", accessed: "2026-10-23" }
```

```yaml
# Qualification · Option 2 · 50K
firm: bulenox
plan: "Qualification Account — Option 2 (EOD Account)"
phase: evaluation
size_label: "50K"
size_usd: "50000.00"
instruments: [MNQ, NQ]
platforms: [Rithmic, "R | Trader Pro", NinjaTrader, Quantower, "Sierra Chart", MultiCharts, ATAS, Bookmap, MotiveWave, "Investor/RT", "Photon Trader", "Trade Navigator", TSLab, ScalpTool, QST, "Medved Trader", OverCharts, QScalp, VolFix, "Inside Edge", "Optimus Flow", Tiger.Trade]
drawdown: { amount_usd: "2500.00", type: trailing_eod, stops_trailing_at: "not stated for the Qualification Account (lock at starting balance + $100 is published for the Master Account)" }
daily_loss_limit: { amount_usd: "1100.00", kind: soft_pause }   # 'trading is disabled for the rest of the trading day. This is not considered a rule violation'
profit_target_usd: "3000.00"
consistency_rule: { text: null, threshold_pct: null }
max_position: { minis: "7", micros: null, scaling: "Dynamic Scaling Plan (both directions): Cash on Hand up to $1,500: 2 • $1,501–4,000: 4 • $4,001+: 7; minis value = top tier ('Contracts EOD' on pricing card); 1 standard contract = 10 micro contracts (micros total not published per size for this plan)" }
min_trading_days: 0
trading_day_reset: { time: "17:00", tz_as_published: "North American Central Time (CT)" }   # "starts at 5:00 PM and ends at 4:00 PM the following day"; positions closed by 3:59 PM CT
copy_trading_allowed: conditional   # copiers permitted; only user-built tools for the account owner's personal use
automated_trading_allowed: conditional
automation_rule_text: |-
    Yes. Bots, algorithms and trade copiers are permitted. If you connect to Rithmic through a third-party API or composite software, an additional $100 per month applies.
    Only user-built tools intended solely for the account owner’s personal use are permitted. Commercial, shared, rented or publicly distributed automation tools are not allowed.
    These tools are developed and maintained independently by their users, so their uptime, reliability and performance are outside our control, and we cannot provide support or accept responsibility for any issues arising from their use.

    You agree to not abuse the company provided service programs. It includes but is not limited to using “scalping” algorithms, DTC Protocol Bridge API or automated discretional trading in any of the trading environments. Open and close hundreds or thousand of rapid trades by using any trading algorithms. Implementation of reckless transactions in rough markets to benefit from lack of execution. If abuse is suspected, we reserve the right to refuse to claim any profit. To avoid performance discrepancies and/or meet applicable compliance requirements, any third-party algorithm, automated trading software, or other third-party trading technology must be reviewed and approved by the Bulenox management team before use. Approval may be subject to additional requirements or conditions established by Bulenox.
vps_allowed: unknown               # not mentioned on any official page (see §9 G1)
personal_device_required: unknown  # not mentioned on any official page (see §9 G1)
status: partial
not_confirmed_fields:
  - "consistency_rule (none published for the Qualification Account)"
  - "drawdown.stops_trailing_at (not stated for Qualification)"
  - "automation_rule_text (third-party / commercial tools: FAQ 'Only user-built tools ... Commercial ... not allowed' vs Terms of Use 'any third-party algorithm ... must be reviewed and approved' — see §9 C1)"
sources:
  - { field: drawdown, url: "https://bulenox.com/accounts-pricing", accessed: "2026-10-23" }
  - { field: drawdown.type, url: "https://bulenox.com/help-center/qualification#option-2", accessed: "2026-10-23" }
  - { field: daily_loss_limit, url: "https://bulenox.com/help-center/qualification#option-2", accessed: "2026-10-23" }
  - { field: daily_loss_limit, url: "https://bulenox.com/accounts-pricing", accessed: "2026-10-23" }
  - { field: profit_target_usd, url: "https://bulenox.com/accounts-pricing", accessed: "2026-10-23" }
  - { field: max_position, url: "https://bulenox.com/help-center/qualification#option-2", accessed: "2026-10-23" }
  - { field: max_position, url: "https://bulenox.com/accounts-pricing", accessed: "2026-10-23" }
  - { field: min_trading_days, url: "https://bulenox.com/help-center/qualification#rules", accessed: "2026-10-23" }
  - { field: trading_day_reset, url: "https://bulenox.com/help-center/qualification#rules", accessed: "2026-10-23" }
  - { field: copy_trading_allowed, url: "https://bulenox.com/faq", accessed: "2026-10-23" }
  - { field: automated_trading_allowed, url: "https://bulenox.com/faq", accessed: "2026-10-23" }
  - { field: automated_trading_allowed, url: "https://bulenox.com/legal/Terms_of_Use.pdf", accessed: "2026-10-23" }
  - { field: platforms, url: "https://bulenox.com/", accessed: "2026-10-23" }
  - { field: platforms, url: "https://bulenox.com/faq", accessed: "2026-10-23" }
  - { field: vps_allowed, url: "not mentioned — searched help center (all 9 categories), FAQ, Terms of Use, Risk Disclosure, homepage, accounts-pricing", accessed: "2026-10-23" }
  - { field: personal_device_required, url: "not mentioned — same search", accessed: "2026-10-23" }
```

```yaml
# Qualification · Option 2 · 100K
firm: bulenox
plan: "Qualification Account — Option 2 (EOD Account)"
phase: evaluation
size_label: "100K"
size_usd: "100000.00"
instruments: [MNQ, NQ]
platforms: [Rithmic, "R | Trader Pro", NinjaTrader, Quantower, "Sierra Chart", MultiCharts, ATAS, Bookmap, MotiveWave, "Investor/RT", "Photon Trader", "Trade Navigator", TSLab, ScalpTool, QST, "Medved Trader", OverCharts, QScalp, VolFix, "Inside Edge", "Optimus Flow", Tiger.Trade]
drawdown: { amount_usd: "3000.00", type: trailing_eod, stops_trailing_at: "not stated for the Qualification Account (lock at starting balance + $100 is published for the Master Account)" }
daily_loss_limit: { amount_usd: "2200.00", kind: soft_pause }   # 'trading is disabled for the rest of the trading day. This is not considered a rule violation'
profit_target_usd: "6000.00"
consistency_rule: { text: null, threshold_pct: null }
max_position: { minis: "12", micros: null, scaling: "Dynamic Scaling Plan (both directions): Cash on Hand up to $2,000: 3 • $2,001–3,000: 5 • $3,001–5,000: 8 • $5,001+: 12; minis value = top tier ('Contracts EOD' on pricing card); 1 standard contract = 10 micro contracts (micros total not published per size for this plan)" }
min_trading_days: 0
trading_day_reset: { time: "17:00", tz_as_published: "North American Central Time (CT)" }   # "starts at 5:00 PM and ends at 4:00 PM the following day"; positions closed by 3:59 PM CT
copy_trading_allowed: conditional   # copiers permitted; only user-built tools for the account owner's personal use
automated_trading_allowed: conditional
automation_rule_text: |-
    Yes. Bots, algorithms and trade copiers are permitted. If you connect to Rithmic through a third-party API or composite software, an additional $100 per month applies.
    Only user-built tools intended solely for the account owner’s personal use are permitted. Commercial, shared, rented or publicly distributed automation tools are not allowed.
    These tools are developed and maintained independently by their users, so their uptime, reliability and performance are outside our control, and we cannot provide support or accept responsibility for any issues arising from their use.

    You agree to not abuse the company provided service programs. It includes but is not limited to using “scalping” algorithms, DTC Protocol Bridge API or automated discretional trading in any of the trading environments. Open and close hundreds or thousand of rapid trades by using any trading algorithms. Implementation of reckless transactions in rough markets to benefit from lack of execution. If abuse is suspected, we reserve the right to refuse to claim any profit. To avoid performance discrepancies and/or meet applicable compliance requirements, any third-party algorithm, automated trading software, or other third-party trading technology must be reviewed and approved by the Bulenox management team before use. Approval may be subject to additional requirements or conditions established by Bulenox.
vps_allowed: unknown               # not mentioned on any official page (see §9 G1)
personal_device_required: unknown  # not mentioned on any official page (see §9 G1)
status: partial
not_confirmed_fields:
  - "consistency_rule (none published for the Qualification Account)"
  - "drawdown.stops_trailing_at (not stated for Qualification)"
  - "automation_rule_text (third-party / commercial tools: FAQ 'Only user-built tools ... Commercial ... not allowed' vs Terms of Use 'any third-party algorithm ... must be reviewed and approved' — see §9 C1)"
sources:
  - { field: drawdown, url: "https://bulenox.com/accounts-pricing", accessed: "2026-10-23" }
  - { field: drawdown.type, url: "https://bulenox.com/help-center/qualification#option-2", accessed: "2026-10-23" }
  - { field: daily_loss_limit, url: "https://bulenox.com/help-center/qualification#option-2", accessed: "2026-10-23" }
  - { field: daily_loss_limit, url: "https://bulenox.com/accounts-pricing", accessed: "2026-10-23" }
  - { field: profit_target_usd, url: "https://bulenox.com/accounts-pricing", accessed: "2026-10-23" }
  - { field: max_position, url: "https://bulenox.com/help-center/qualification#option-2", accessed: "2026-10-23" }
  - { field: max_position, url: "https://bulenox.com/accounts-pricing", accessed: "2026-10-23" }
  - { field: min_trading_days, url: "https://bulenox.com/help-center/qualification#rules", accessed: "2026-10-23" }
  - { field: trading_day_reset, url: "https://bulenox.com/help-center/qualification#rules", accessed: "2026-10-23" }
  - { field: copy_trading_allowed, url: "https://bulenox.com/faq", accessed: "2026-10-23" }
  - { field: automated_trading_allowed, url: "https://bulenox.com/faq", accessed: "2026-10-23" }
  - { field: automated_trading_allowed, url: "https://bulenox.com/legal/Terms_of_Use.pdf", accessed: "2026-10-23" }
  - { field: platforms, url: "https://bulenox.com/", accessed: "2026-10-23" }
  - { field: platforms, url: "https://bulenox.com/faq", accessed: "2026-10-23" }
  - { field: vps_allowed, url: "not mentioned — searched help center (all 9 categories), FAQ, Terms of Use, Risk Disclosure, homepage, accounts-pricing", accessed: "2026-10-23" }
  - { field: personal_device_required, url: "not mentioned — same search", accessed: "2026-10-23" }
```

```yaml
# Qualification · Option 2 · 150K
firm: bulenox
plan: "Qualification Account — Option 2 (EOD Account)"
phase: evaluation
size_label: "150K"
size_usd: "150000.00"
instruments: [MNQ, NQ]
platforms: [Rithmic, "R | Trader Pro", NinjaTrader, Quantower, "Sierra Chart", MultiCharts, ATAS, Bookmap, MotiveWave, "Investor/RT", "Photon Trader", "Trade Navigator", TSLab, ScalpTool, QST, "Medved Trader", OverCharts, QScalp, VolFix, "Inside Edge", "Optimus Flow", Tiger.Trade]
drawdown: { amount_usd: "4500.00", type: trailing_eod, stops_trailing_at: "not stated for the Qualification Account (lock at starting balance + $100 is published for the Master Account)" }
daily_loss_limit: { amount_usd: "3300.00", kind: soft_pause }   # 'trading is disabled for the rest of the trading day. This is not considered a rule violation'
profit_target_usd: "9000.00"
consistency_rule: { text: null, threshold_pct: null }
max_position: { minis: "15", micros: null, scaling: "Dynamic Scaling Plan (both directions): Cash on Hand up to $4,000: 5 • $4,001–8,000: 8 • $8,001–12,000: 10 • $12,001+: 15; minis value = top tier ('Contracts EOD' on pricing card); 1 standard contract = 10 micro contracts (micros total not published per size for this plan)" }
min_trading_days: 0
trading_day_reset: { time: "17:00", tz_as_published: "North American Central Time (CT)" }   # "starts at 5:00 PM and ends at 4:00 PM the following day"; positions closed by 3:59 PM CT
copy_trading_allowed: conditional   # copiers permitted; only user-built tools for the account owner's personal use
automated_trading_allowed: conditional
automation_rule_text: |-
    Yes. Bots, algorithms and trade copiers are permitted. If you connect to Rithmic through a third-party API or composite software, an additional $100 per month applies.
    Only user-built tools intended solely for the account owner’s personal use are permitted. Commercial, shared, rented or publicly distributed automation tools are not allowed.
    These tools are developed and maintained independently by their users, so their uptime, reliability and performance are outside our control, and we cannot provide support or accept responsibility for any issues arising from their use.

    You agree to not abuse the company provided service programs. It includes but is not limited to using “scalping” algorithms, DTC Protocol Bridge API or automated discretional trading in any of the trading environments. Open and close hundreds or thousand of rapid trades by using any trading algorithms. Implementation of reckless transactions in rough markets to benefit from lack of execution. If abuse is suspected, we reserve the right to refuse to claim any profit. To avoid performance discrepancies and/or meet applicable compliance requirements, any third-party algorithm, automated trading software, or other third-party trading technology must be reviewed and approved by the Bulenox management team before use. Approval may be subject to additional requirements or conditions established by Bulenox.
vps_allowed: unknown               # not mentioned on any official page (see §9 G1)
personal_device_required: unknown  # not mentioned on any official page (see §9 G1)
status: partial
not_confirmed_fields:
  - "consistency_rule (none published for the Qualification Account)"
  - "drawdown.stops_trailing_at (not stated for Qualification)"
  - "automation_rule_text (third-party / commercial tools: FAQ 'Only user-built tools ... Commercial ... not allowed' vs Terms of Use 'any third-party algorithm ... must be reviewed and approved' — see §9 C1)"
sources:
  - { field: drawdown, url: "https://bulenox.com/accounts-pricing", accessed: "2026-10-23" }
  - { field: drawdown.type, url: "https://bulenox.com/help-center/qualification#option-2", accessed: "2026-10-23" }
  - { field: daily_loss_limit, url: "https://bulenox.com/help-center/qualification#option-2", accessed: "2026-10-23" }
  - { field: daily_loss_limit, url: "https://bulenox.com/accounts-pricing", accessed: "2026-10-23" }
  - { field: profit_target_usd, url: "https://bulenox.com/accounts-pricing", accessed: "2026-10-23" }
  - { field: max_position, url: "https://bulenox.com/help-center/qualification#option-2", accessed: "2026-10-23" }
  - { field: max_position, url: "https://bulenox.com/accounts-pricing", accessed: "2026-10-23" }
  - { field: min_trading_days, url: "https://bulenox.com/help-center/qualification#rules", accessed: "2026-10-23" }
  - { field: trading_day_reset, url: "https://bulenox.com/help-center/qualification#rules", accessed: "2026-10-23" }
  - { field: copy_trading_allowed, url: "https://bulenox.com/faq", accessed: "2026-10-23" }
  - { field: automated_trading_allowed, url: "https://bulenox.com/faq", accessed: "2026-10-23" }
  - { field: automated_trading_allowed, url: "https://bulenox.com/legal/Terms_of_Use.pdf", accessed: "2026-10-23" }
  - { field: platforms, url: "https://bulenox.com/", accessed: "2026-10-23" }
  - { field: platforms, url: "https://bulenox.com/faq", accessed: "2026-10-23" }
  - { field: vps_allowed, url: "not mentioned — searched help center (all 9 categories), FAQ, Terms of Use, Risk Disclosure, homepage, accounts-pricing", accessed: "2026-10-23" }
  - { field: personal_device_required, url: "not mentioned — same search", accessed: "2026-10-23" }
```

```yaml
# Master · Option 1 · 25K
firm: bulenox
plan: "Master Account — Option 1 (No Scaling, Trailing Drawdown)"
phase: funded
size_label: "25K"
size_usd: "25000.00"
instruments: [MNQ, NQ]
platforms: [Rithmic, "R | Trader Pro", NinjaTrader, Quantower, "Sierra Chart", MultiCharts, ATAS, Bookmap, MotiveWave, "Investor/RT", "Photon Trader", "Trade Navigator", TSLab, ScalpTool, QST, "Medved Trader", OverCharts, QScalp, VolFix, "Inside Edge", "Optimus Flow", Tiger.Trade]
drawdown: { amount_usd: "1500.00", type: trailing_intraday, stops_trailing_at: "permanently fixed at starting balance + $100 (threshold $25,100, reached at account value $26,600)" }
daily_loss_limit: { amount_usd: null, kind: none }
profit_target_usd: null
consistency_rule: { text: "Your most profitable trading day must not represent more than 40% of your Total P&L for the applicable payout period. (payout condition, not a breach)", threshold_pct: "40" }
max_position: { minis: "3", micros: null, scaling: "none (Option 1); 1 standard contract = 10 micro contracts (micros total not published per size for this plan)" }
min_trading_days: null
trading_day_reset: { time: "17:00", tz_as_published: "North American Central Time (CT)" }   # "starts at 5:00 PM and ends at 4:00 PM the following day"; positions closed by 3:59 PM CT
copy_trading_allowed: conditional   # copiers permitted; only user-built tools for the account owner's personal use
automated_trading_allowed: conditional
automation_rule_text: |-
    Yes. Bots, algorithms and trade copiers are permitted. If you connect to Rithmic through a third-party API or composite software, an additional $100 per month applies.
    Only user-built tools intended solely for the account owner’s personal use are permitted. Commercial, shared, rented or publicly distributed automation tools are not allowed.
    These tools are developed and maintained independently by their users, so their uptime, reliability and performance are outside our control, and we cannot provide support or accept responsibility for any issues arising from their use.

    You agree to not abuse the company provided service programs. It includes but is not limited to using “scalping” algorithms, DTC Protocol Bridge API or automated discretional trading in any of the trading environments. Open and close hundreds or thousand of rapid trades by using any trading algorithms. Implementation of reckless transactions in rough markets to benefit from lack of execution. If abuse is suspected, we reserve the right to refuse to claim any profit. To avoid performance discrepancies and/or meet applicable compliance requirements, any third-party algorithm, automated trading software, or other third-party trading technology must be reviewed and approved by the Bulenox management team before use. Approval may be subject to additional requirements or conditions established by Bulenox.
vps_allowed: unknown               # not mentioned on any official page (see §9 G1)
personal_device_required: unknown  # not mentioned on any official page (see §9 G1)
payout_rules: "10 individual trading days per payout; min $1,000; Safety Threshold Reserve $1,600 above starting balance; inactivity: at least one trade per week"
status: partial
not_confirmed_fields:
  - "automation_rule_text (third-party / commercial tools: FAQ 'Only user-built tools ... Commercial ... not allowed' vs Terms of Use 'any third-party algorithm ... must be reviewed and approved' — see §9 C1)"
sources:
  - { field: drawdown, url: "https://bulenox.com/help-center/master#master-trading-rules", accessed: "2026-10-23" }
  - { field: daily_loss_limit, url: "https://bulenox.com/accounts-pricing", accessed: "2026-10-23" }
  - { field: consistency_rule, url: "https://bulenox.com/help-center/master#withdrawal-rules", accessed: "2026-10-23" }
  - { field: max_position, url: "https://bulenox.com/help-center/master#master-trading-rules", accessed: "2026-10-23" }
  - { field: max_position, url: "https://bulenox.com/accounts-pricing", accessed: "2026-10-23" }
  - { field: trading_day_reset, url: "https://bulenox.com/help-center/qualification#rules", accessed: "2026-10-23" }
  - { field: copy_trading_allowed, url: "https://bulenox.com/faq", accessed: "2026-10-23" }
  - { field: automated_trading_allowed, url: "https://bulenox.com/faq", accessed: "2026-10-23" }
  - { field: automated_trading_allowed, url: "https://bulenox.com/legal/Terms_of_Use.pdf", accessed: "2026-10-23" }
  - { field: platforms, url: "https://bulenox.com/", accessed: "2026-10-23" }
  - { field: platforms, url: "https://bulenox.com/faq", accessed: "2026-10-23" }
  - { field: vps_allowed, url: "not mentioned — searched help center (all 9 categories), FAQ, Terms of Use, Risk Disclosure, homepage, accounts-pricing", accessed: "2026-10-23" }
  - { field: personal_device_required, url: "not mentioned — same search", accessed: "2026-10-23" }
```

```yaml
# Master · Option 1 · 50K
firm: bulenox
plan: "Master Account — Option 1 (No Scaling, Trailing Drawdown)"
phase: funded
size_label: "50K"
size_usd: "50000.00"
instruments: [MNQ, NQ]
platforms: [Rithmic, "R | Trader Pro", NinjaTrader, Quantower, "Sierra Chart", MultiCharts, ATAS, Bookmap, MotiveWave, "Investor/RT", "Photon Trader", "Trade Navigator", TSLab, ScalpTool, QST, "Medved Trader", OverCharts, QScalp, VolFix, "Inside Edge", "Optimus Flow", Tiger.Trade]
drawdown: { amount_usd: "2500.00", type: trailing_intraday, stops_trailing_at: "permanently fixed at starting balance + $100 (threshold $50,100, reached at account value $52,600)" }
daily_loss_limit: { amount_usd: null, kind: none }
profit_target_usd: null
consistency_rule: { text: "Your most profitable trading day must not represent more than 40% of your Total P&L for the applicable payout period. (payout condition, not a breach)", threshold_pct: "40" }
max_position: { minis: "7", micros: null, scaling: "none (Option 1); 1 standard contract = 10 micro contracts (micros total not published per size for this plan)" }
min_trading_days: null
trading_day_reset: { time: "17:00", tz_as_published: "North American Central Time (CT)" }   # "starts at 5:00 PM and ends at 4:00 PM the following day"; positions closed by 3:59 PM CT
copy_trading_allowed: conditional   # copiers permitted; only user-built tools for the account owner's personal use
automated_trading_allowed: conditional
automation_rule_text: |-
    Yes. Bots, algorithms and trade copiers are permitted. If you connect to Rithmic through a third-party API or composite software, an additional $100 per month applies.
    Only user-built tools intended solely for the account owner’s personal use are permitted. Commercial, shared, rented or publicly distributed automation tools are not allowed.
    These tools are developed and maintained independently by their users, so their uptime, reliability and performance are outside our control, and we cannot provide support or accept responsibility for any issues arising from their use.

    You agree to not abuse the company provided service programs. It includes but is not limited to using “scalping” algorithms, DTC Protocol Bridge API or automated discretional trading in any of the trading environments. Open and close hundreds or thousand of rapid trades by using any trading algorithms. Implementation of reckless transactions in rough markets to benefit from lack of execution. If abuse is suspected, we reserve the right to refuse to claim any profit. To avoid performance discrepancies and/or meet applicable compliance requirements, any third-party algorithm, automated trading software, or other third-party trading technology must be reviewed and approved by the Bulenox management team before use. Approval may be subject to additional requirements or conditions established by Bulenox.
vps_allowed: unknown               # not mentioned on any official page (see §9 G1)
personal_device_required: unknown  # not mentioned on any official page (see §9 G1)
payout_rules: "10 individual trading days per payout; min $1,000; Safety Threshold Reserve $2,600 above starting balance; inactivity: at least one trade per week"
status: partial
not_confirmed_fields:
  - "automation_rule_text (third-party / commercial tools: FAQ 'Only user-built tools ... Commercial ... not allowed' vs Terms of Use 'any third-party algorithm ... must be reviewed and approved' — see §9 C1)"
sources:
  - { field: drawdown, url: "https://bulenox.com/help-center/master#master-trading-rules", accessed: "2026-10-23" }
  - { field: daily_loss_limit, url: "https://bulenox.com/accounts-pricing", accessed: "2026-10-23" }
  - { field: consistency_rule, url: "https://bulenox.com/help-center/master#withdrawal-rules", accessed: "2026-10-23" }
  - { field: max_position, url: "https://bulenox.com/help-center/master#master-trading-rules", accessed: "2026-10-23" }
  - { field: max_position, url: "https://bulenox.com/accounts-pricing", accessed: "2026-10-23" }
  - { field: trading_day_reset, url: "https://bulenox.com/help-center/qualification#rules", accessed: "2026-10-23" }
  - { field: copy_trading_allowed, url: "https://bulenox.com/faq", accessed: "2026-10-23" }
  - { field: automated_trading_allowed, url: "https://bulenox.com/faq", accessed: "2026-10-23" }
  - { field: automated_trading_allowed, url: "https://bulenox.com/legal/Terms_of_Use.pdf", accessed: "2026-10-23" }
  - { field: platforms, url: "https://bulenox.com/", accessed: "2026-10-23" }
  - { field: platforms, url: "https://bulenox.com/faq", accessed: "2026-10-23" }
  - { field: vps_allowed, url: "not mentioned — searched help center (all 9 categories), FAQ, Terms of Use, Risk Disclosure, homepage, accounts-pricing", accessed: "2026-10-23" }
  - { field: personal_device_required, url: "not mentioned — same search", accessed: "2026-10-23" }
```

```yaml
# Master · Option 1 · 100K
firm: bulenox
plan: "Master Account — Option 1 (No Scaling, Trailing Drawdown)"
phase: funded
size_label: "100K"
size_usd: "100000.00"
instruments: [MNQ, NQ]
platforms: [Rithmic, "R | Trader Pro", NinjaTrader, Quantower, "Sierra Chart", MultiCharts, ATAS, Bookmap, MotiveWave, "Investor/RT", "Photon Trader", "Trade Navigator", TSLab, ScalpTool, QST, "Medved Trader", OverCharts, QScalp, VolFix, "Inside Edge", "Optimus Flow", Tiger.Trade]
drawdown: { amount_usd: "3000.00", type: trailing_intraday, stops_trailing_at: "permanently fixed at starting balance + $100 (threshold $100,100, reached at account value $103,100)" }
daily_loss_limit: { amount_usd: null, kind: none }
profit_target_usd: null
consistency_rule: { text: "Your most profitable trading day must not represent more than 40% of your Total P&L for the applicable payout period. (payout condition, not a breach)", threshold_pct: "40" }
max_position: { minis: "12", micros: null, scaling: "none (Option 1); 1 standard contract = 10 micro contracts (micros total not published per size for this plan)" }
min_trading_days: null
trading_day_reset: { time: "17:00", tz_as_published: "North American Central Time (CT)" }   # "starts at 5:00 PM and ends at 4:00 PM the following day"; positions closed by 3:59 PM CT
copy_trading_allowed: conditional   # copiers permitted; only user-built tools for the account owner's personal use
automated_trading_allowed: conditional
automation_rule_text: |-
    Yes. Bots, algorithms and trade copiers are permitted. If you connect to Rithmic through a third-party API or composite software, an additional $100 per month applies.
    Only user-built tools intended solely for the account owner’s personal use are permitted. Commercial, shared, rented or publicly distributed automation tools are not allowed.
    These tools are developed and maintained independently by their users, so their uptime, reliability and performance are outside our control, and we cannot provide support or accept responsibility for any issues arising from their use.

    You agree to not abuse the company provided service programs. It includes but is not limited to using “scalping” algorithms, DTC Protocol Bridge API or automated discretional trading in any of the trading environments. Open and close hundreds or thousand of rapid trades by using any trading algorithms. Implementation of reckless transactions in rough markets to benefit from lack of execution. If abuse is suspected, we reserve the right to refuse to claim any profit. To avoid performance discrepancies and/or meet applicable compliance requirements, any third-party algorithm, automated trading software, or other third-party trading technology must be reviewed and approved by the Bulenox management team before use. Approval may be subject to additional requirements or conditions established by Bulenox.
vps_allowed: unknown               # not mentioned on any official page (see §9 G1)
personal_device_required: unknown  # not mentioned on any official page (see §9 G1)
payout_rules: "10 individual trading days per payout; min $1,000; Safety Threshold Reserve $3,100 above starting balance; inactivity: at least one trade per week"
status: partial
not_confirmed_fields:
  - "automation_rule_text (third-party / commercial tools: FAQ 'Only user-built tools ... Commercial ... not allowed' vs Terms of Use 'any third-party algorithm ... must be reviewed and approved' — see §9 C1)"
sources:
  - { field: drawdown, url: "https://bulenox.com/help-center/master#master-trading-rules", accessed: "2026-10-23" }
  - { field: daily_loss_limit, url: "https://bulenox.com/accounts-pricing", accessed: "2026-10-23" }
  - { field: consistency_rule, url: "https://bulenox.com/help-center/master#withdrawal-rules", accessed: "2026-10-23" }
  - { field: max_position, url: "https://bulenox.com/help-center/master#master-trading-rules", accessed: "2026-10-23" }
  - { field: max_position, url: "https://bulenox.com/accounts-pricing", accessed: "2026-10-23" }
  - { field: trading_day_reset, url: "https://bulenox.com/help-center/qualification#rules", accessed: "2026-10-23" }
  - { field: copy_trading_allowed, url: "https://bulenox.com/faq", accessed: "2026-10-23" }
  - { field: automated_trading_allowed, url: "https://bulenox.com/faq", accessed: "2026-10-23" }
  - { field: automated_trading_allowed, url: "https://bulenox.com/legal/Terms_of_Use.pdf", accessed: "2026-10-23" }
  - { field: platforms, url: "https://bulenox.com/", accessed: "2026-10-23" }
  - { field: platforms, url: "https://bulenox.com/faq", accessed: "2026-10-23" }
  - { field: vps_allowed, url: "not mentioned — searched help center (all 9 categories), FAQ, Terms of Use, Risk Disclosure, homepage, accounts-pricing", accessed: "2026-10-23" }
  - { field: personal_device_required, url: "not mentioned — same search", accessed: "2026-10-23" }
```

```yaml
# Master · Option 1 · 150K
firm: bulenox
plan: "Master Account — Option 1 (No Scaling, Trailing Drawdown)"
phase: funded
size_label: "150K"
size_usd: "150000.00"
instruments: [MNQ, NQ]
platforms: [Rithmic, "R | Trader Pro", NinjaTrader, Quantower, "Sierra Chart", MultiCharts, ATAS, Bookmap, MotiveWave, "Investor/RT", "Photon Trader", "Trade Navigator", TSLab, ScalpTool, QST, "Medved Trader", OverCharts, QScalp, VolFix, "Inside Edge", "Optimus Flow", Tiger.Trade]
drawdown: { amount_usd: "4500.00", type: trailing_intraday, stops_trailing_at: "permanently fixed at starting balance + $100 (threshold $150,100, reached at account value $154,600)" }
daily_loss_limit: { amount_usd: null, kind: none }
profit_target_usd: null
consistency_rule: { text: "Your most profitable trading day must not represent more than 40% of your Total P&L for the applicable payout period. (payout condition, not a breach)", threshold_pct: "40" }
max_position: { minis: "15", micros: null, scaling: "none (Option 1); 1 standard contract = 10 micro contracts (micros total not published per size for this plan)" }
min_trading_days: null
trading_day_reset: { time: "17:00", tz_as_published: "North American Central Time (CT)" }   # "starts at 5:00 PM and ends at 4:00 PM the following day"; positions closed by 3:59 PM CT
copy_trading_allowed: conditional   # copiers permitted; only user-built tools for the account owner's personal use
automated_trading_allowed: conditional
automation_rule_text: |-
    Yes. Bots, algorithms and trade copiers are permitted. If you connect to Rithmic through a third-party API or composite software, an additional $100 per month applies.
    Only user-built tools intended solely for the account owner’s personal use are permitted. Commercial, shared, rented or publicly distributed automation tools are not allowed.
    These tools are developed and maintained independently by their users, so their uptime, reliability and performance are outside our control, and we cannot provide support or accept responsibility for any issues arising from their use.

    You agree to not abuse the company provided service programs. It includes but is not limited to using “scalping” algorithms, DTC Protocol Bridge API or automated discretional trading in any of the trading environments. Open and close hundreds or thousand of rapid trades by using any trading algorithms. Implementation of reckless transactions in rough markets to benefit from lack of execution. If abuse is suspected, we reserve the right to refuse to claim any profit. To avoid performance discrepancies and/or meet applicable compliance requirements, any third-party algorithm, automated trading software, or other third-party trading technology must be reviewed and approved by the Bulenox management team before use. Approval may be subject to additional requirements or conditions established by Bulenox.
vps_allowed: unknown               # not mentioned on any official page (see §9 G1)
personal_device_required: unknown  # not mentioned on any official page (see §9 G1)
payout_rules: "10 individual trading days per payout; min $1,000; Safety Threshold Reserve $4,600 above starting balance; inactivity: at least one trade per week"
status: partial
not_confirmed_fields:
  - "automation_rule_text (third-party / commercial tools: FAQ 'Only user-built tools ... Commercial ... not allowed' vs Terms of Use 'any third-party algorithm ... must be reviewed and approved' — see §9 C1)"
sources:
  - { field: drawdown, url: "https://bulenox.com/help-center/master#master-trading-rules", accessed: "2026-10-23" }
  - { field: daily_loss_limit, url: "https://bulenox.com/accounts-pricing", accessed: "2026-10-23" }
  - { field: consistency_rule, url: "https://bulenox.com/help-center/master#withdrawal-rules", accessed: "2026-10-23" }
  - { field: max_position, url: "https://bulenox.com/help-center/master#master-trading-rules", accessed: "2026-10-23" }
  - { field: max_position, url: "https://bulenox.com/accounts-pricing", accessed: "2026-10-23" }
  - { field: trading_day_reset, url: "https://bulenox.com/help-center/qualification#rules", accessed: "2026-10-23" }
  - { field: copy_trading_allowed, url: "https://bulenox.com/faq", accessed: "2026-10-23" }
  - { field: automated_trading_allowed, url: "https://bulenox.com/faq", accessed: "2026-10-23" }
  - { field: automated_trading_allowed, url: "https://bulenox.com/legal/Terms_of_Use.pdf", accessed: "2026-10-23" }
  - { field: platforms, url: "https://bulenox.com/", accessed: "2026-10-23" }
  - { field: platforms, url: "https://bulenox.com/faq", accessed: "2026-10-23" }
  - { field: vps_allowed, url: "not mentioned — searched help center (all 9 categories), FAQ, Terms of Use, Risk Disclosure, homepage, accounts-pricing", accessed: "2026-10-23" }
  - { field: personal_device_required, url: "not mentioned — same search", accessed: "2026-10-23" }
```

```yaml
# Master · Option 2 · 25K
firm: bulenox
plan: "Master Account — Option 2 (EOD Account)"
phase: funded
size_label: "25K"
size_usd: "25000.00"
instruments: [MNQ, NQ]
platforms: [Rithmic, "R | Trader Pro", NinjaTrader, Quantower, "Sierra Chart", MultiCharts, ATAS, Bookmap, MotiveWave, "Investor/RT", "Photon Trader", "Trade Navigator", TSLab, ScalpTool, QST, "Medved Trader", OverCharts, QScalp, VolFix, "Inside Edge", "Optimus Flow", Tiger.Trade]
drawdown: { amount_usd: "1500.00", type: trailing_eod, stops_trailing_at: "permanently fixed at starting balance + $100 (threshold $25,100, after the account closes at $26,600)" }
daily_loss_limit: { amount_usd: "500.00", kind: soft_pause }   # disabled for the remainder of the session, not a violation; permanently removed once the EOD Drawdown locks at starting balance + $100
profit_target_usd: null
consistency_rule: { text: "Your most profitable trading day must not represent more than 40% of your Total P&L for the applicable payout period. (payout condition, not a breach)", threshold_pct: "40" }
max_position: { minis: "3", micros: null, scaling: "Dynamic Scaling Plan (both directions): Cash on Hand up to $1,500: 2 • $1,501+: 3; 1 standard contract = 10 micro contracts (micros total not published per size for this plan)" }
min_trading_days: null
trading_day_reset: { time: "17:00", tz_as_published: "North American Central Time (CT)" }   # "starts at 5:00 PM and ends at 4:00 PM the following day"; positions closed by 3:59 PM CT
copy_trading_allowed: conditional   # copiers permitted; only user-built tools for the account owner's personal use
automated_trading_allowed: conditional
automation_rule_text: |-
    Yes. Bots, algorithms and trade copiers are permitted. If you connect to Rithmic through a third-party API or composite software, an additional $100 per month applies.
    Only user-built tools intended solely for the account owner’s personal use are permitted. Commercial, shared, rented or publicly distributed automation tools are not allowed.
    These tools are developed and maintained independently by their users, so their uptime, reliability and performance are outside our control, and we cannot provide support or accept responsibility for any issues arising from their use.

    You agree to not abuse the company provided service programs. It includes but is not limited to using “scalping” algorithms, DTC Protocol Bridge API or automated discretional trading in any of the trading environments. Open and close hundreds or thousand of rapid trades by using any trading algorithms. Implementation of reckless transactions in rough markets to benefit from lack of execution. If abuse is suspected, we reserve the right to refuse to claim any profit. To avoid performance discrepancies and/or meet applicable compliance requirements, any third-party algorithm, automated trading software, or other third-party trading technology must be reviewed and approved by the Bulenox management team before use. Approval may be subject to additional requirements or conditions established by Bulenox.
vps_allowed: unknown               # not mentioned on any official page (see §9 G1)
personal_device_required: unknown  # not mentioned on any official page (see §9 G1)
payout_rules: "10 individual trading days per payout; min $1,000; Safety Threshold Reserve $1,600 above starting balance; inactivity: at least one trade per week"
status: partial
not_confirmed_fields:
  - "automation_rule_text (third-party / commercial tools: FAQ 'Only user-built tools ... Commercial ... not allowed' vs Terms of Use 'any third-party algorithm ... must be reviewed and approved' — see §9 C1)"
sources:
  - { field: drawdown, url: "https://bulenox.com/help-center/master#master-trading-rules", accessed: "2026-10-23" }
  - { field: daily_loss_limit, url: "https://bulenox.com/help-center/master#master-trading-rules", accessed: "2026-10-23" }
  - { field: consistency_rule, url: "https://bulenox.com/help-center/master#withdrawal-rules", accessed: "2026-10-23" }
  - { field: max_position, url: "https://bulenox.com/help-center/master#master-trading-rules", accessed: "2026-10-23" }
  - { field: trading_day_reset, url: "https://bulenox.com/help-center/qualification#rules", accessed: "2026-10-23" }
  - { field: copy_trading_allowed, url: "https://bulenox.com/faq", accessed: "2026-10-23" }
  - { field: automated_trading_allowed, url: "https://bulenox.com/faq", accessed: "2026-10-23" }
  - { field: automated_trading_allowed, url: "https://bulenox.com/legal/Terms_of_Use.pdf", accessed: "2026-10-23" }
  - { field: platforms, url: "https://bulenox.com/", accessed: "2026-10-23" }
  - { field: platforms, url: "https://bulenox.com/faq", accessed: "2026-10-23" }
  - { field: vps_allowed, url: "not mentioned — searched help center (all 9 categories), FAQ, Terms of Use, Risk Disclosure, homepage, accounts-pricing", accessed: "2026-10-23" }
  - { field: personal_device_required, url: "not mentioned — same search", accessed: "2026-10-23" }
```

```yaml
# Master · Option 2 · 50K
firm: bulenox
plan: "Master Account — Option 2 (EOD Account)"
phase: funded
size_label: "50K"
size_usd: "50000.00"
instruments: [MNQ, NQ]
platforms: [Rithmic, "R | Trader Pro", NinjaTrader, Quantower, "Sierra Chart", MultiCharts, ATAS, Bookmap, MotiveWave, "Investor/RT", "Photon Trader", "Trade Navigator", TSLab, ScalpTool, QST, "Medved Trader", OverCharts, QScalp, VolFix, "Inside Edge", "Optimus Flow", Tiger.Trade]
drawdown: { amount_usd: "2500.00", type: trailing_eod, stops_trailing_at: "permanently fixed at starting balance + $100 (threshold $50,100, after the account closes at $52,600)" }
daily_loss_limit: { amount_usd: "1100.00", kind: soft_pause }   # disabled for the remainder of the session, not a violation; permanently removed once the EOD Drawdown locks at starting balance + $100
profit_target_usd: null
consistency_rule: { text: "Your most profitable trading day must not represent more than 40% of your Total P&L for the applicable payout period. (payout condition, not a breach)", threshold_pct: "40" }
max_position: { minis: "7", micros: null, scaling: "Dynamic Scaling Plan (both directions): Cash on Hand up to $1,500: 2 • $1,501–4,000: 4 • $4,001+: 7; 1 standard contract = 10 micro contracts (micros total not published per size for this plan)" }
min_trading_days: null
trading_day_reset: { time: "17:00", tz_as_published: "North American Central Time (CT)" }   # "starts at 5:00 PM and ends at 4:00 PM the following day"; positions closed by 3:59 PM CT
copy_trading_allowed: conditional   # copiers permitted; only user-built tools for the account owner's personal use
automated_trading_allowed: conditional
automation_rule_text: |-
    Yes. Bots, algorithms and trade copiers are permitted. If you connect to Rithmic through a third-party API or composite software, an additional $100 per month applies.
    Only user-built tools intended solely for the account owner’s personal use are permitted. Commercial, shared, rented or publicly distributed automation tools are not allowed.
    These tools are developed and maintained independently by their users, so their uptime, reliability and performance are outside our control, and we cannot provide support or accept responsibility for any issues arising from their use.

    You agree to not abuse the company provided service programs. It includes but is not limited to using “scalping” algorithms, DTC Protocol Bridge API or automated discretional trading in any of the trading environments. Open and close hundreds or thousand of rapid trades by using any trading algorithms. Implementation of reckless transactions in rough markets to benefit from lack of execution. If abuse is suspected, we reserve the right to refuse to claim any profit. To avoid performance discrepancies and/or meet applicable compliance requirements, any third-party algorithm, automated trading software, or other third-party trading technology must be reviewed and approved by the Bulenox management team before use. Approval may be subject to additional requirements or conditions established by Bulenox.
vps_allowed: unknown               # not mentioned on any official page (see §9 G1)
personal_device_required: unknown  # not mentioned on any official page (see §9 G1)
payout_rules: "10 individual trading days per payout; min $1,000; Safety Threshold Reserve $2,600 above starting balance; inactivity: at least one trade per week"
status: partial
not_confirmed_fields:
  - "automation_rule_text (third-party / commercial tools: FAQ 'Only user-built tools ... Commercial ... not allowed' vs Terms of Use 'any third-party algorithm ... must be reviewed and approved' — see §9 C1)"
sources:
  - { field: drawdown, url: "https://bulenox.com/help-center/master#master-trading-rules", accessed: "2026-10-23" }
  - { field: daily_loss_limit, url: "https://bulenox.com/help-center/master#master-trading-rules", accessed: "2026-10-23" }
  - { field: consistency_rule, url: "https://bulenox.com/help-center/master#withdrawal-rules", accessed: "2026-10-23" }
  - { field: max_position, url: "https://bulenox.com/help-center/master#master-trading-rules", accessed: "2026-10-23" }
  - { field: trading_day_reset, url: "https://bulenox.com/help-center/qualification#rules", accessed: "2026-10-23" }
  - { field: copy_trading_allowed, url: "https://bulenox.com/faq", accessed: "2026-10-23" }
  - { field: automated_trading_allowed, url: "https://bulenox.com/faq", accessed: "2026-10-23" }
  - { field: automated_trading_allowed, url: "https://bulenox.com/legal/Terms_of_Use.pdf", accessed: "2026-10-23" }
  - { field: platforms, url: "https://bulenox.com/", accessed: "2026-10-23" }
  - { field: platforms, url: "https://bulenox.com/faq", accessed: "2026-10-23" }
  - { field: vps_allowed, url: "not mentioned — searched help center (all 9 categories), FAQ, Terms of Use, Risk Disclosure, homepage, accounts-pricing", accessed: "2026-10-23" }
  - { field: personal_device_required, url: "not mentioned — same search", accessed: "2026-10-23" }
```

```yaml
# Master · Option 2 · 100K
firm: bulenox
plan: "Master Account — Option 2 (EOD Account)"
phase: funded
size_label: "100K"
size_usd: "100000.00"
instruments: [MNQ, NQ]
platforms: [Rithmic, "R | Trader Pro", NinjaTrader, Quantower, "Sierra Chart", MultiCharts, ATAS, Bookmap, MotiveWave, "Investor/RT", "Photon Trader", "Trade Navigator", TSLab, ScalpTool, QST, "Medved Trader", OverCharts, QScalp, VolFix, "Inside Edge", "Optimus Flow", Tiger.Trade]
drawdown: { amount_usd: "3000.00", type: trailing_eod, stops_trailing_at: "permanently fixed at starting balance + $100 (threshold $100,100, after the account closes at $103,100)" }
daily_loss_limit: { amount_usd: "2200.00", kind: soft_pause }   # disabled for the remainder of the session, not a violation; permanently removed once the EOD Drawdown locks at starting balance + $100
profit_target_usd: null
consistency_rule: { text: "Your most profitable trading day must not represent more than 40% of your Total P&L for the applicable payout period. (payout condition, not a breach)", threshold_pct: "40" }
max_position: { minis: "12", micros: null, scaling: "Dynamic Scaling Plan (both directions): Cash on Hand up to $2,000: 3 • $2,001–3,000: 5 • $3,001–5,000: 8 • $5,001+: 12; 1 standard contract = 10 micro contracts (micros total not published per size for this plan)" }
min_trading_days: null
trading_day_reset: { time: "17:00", tz_as_published: "North American Central Time (CT)" }   # "starts at 5:00 PM and ends at 4:00 PM the following day"; positions closed by 3:59 PM CT
copy_trading_allowed: conditional   # copiers permitted; only user-built tools for the account owner's personal use
automated_trading_allowed: conditional
automation_rule_text: |-
    Yes. Bots, algorithms and trade copiers are permitted. If you connect to Rithmic through a third-party API or composite software, an additional $100 per month applies.
    Only user-built tools intended solely for the account owner’s personal use are permitted. Commercial, shared, rented or publicly distributed automation tools are not allowed.
    These tools are developed and maintained independently by their users, so their uptime, reliability and performance are outside our control, and we cannot provide support or accept responsibility for any issues arising from their use.

    You agree to not abuse the company provided service programs. It includes but is not limited to using “scalping” algorithms, DTC Protocol Bridge API or automated discretional trading in any of the trading environments. Open and close hundreds or thousand of rapid trades by using any trading algorithms. Implementation of reckless transactions in rough markets to benefit from lack of execution. If abuse is suspected, we reserve the right to refuse to claim any profit. To avoid performance discrepancies and/or meet applicable compliance requirements, any third-party algorithm, automated trading software, or other third-party trading technology must be reviewed and approved by the Bulenox management team before use. Approval may be subject to additional requirements or conditions established by Bulenox.
vps_allowed: unknown               # not mentioned on any official page (see §9 G1)
personal_device_required: unknown  # not mentioned on any official page (see §9 G1)
payout_rules: "10 individual trading days per payout; min $1,000; Safety Threshold Reserve $3,100 above starting balance; inactivity: at least one trade per week"
status: partial
not_confirmed_fields:
  - "automation_rule_text (third-party / commercial tools: FAQ 'Only user-built tools ... Commercial ... not allowed' vs Terms of Use 'any third-party algorithm ... must be reviewed and approved' — see §9 C1)"
sources:
  - { field: drawdown, url: "https://bulenox.com/help-center/master#master-trading-rules", accessed: "2026-10-23" }
  - { field: daily_loss_limit, url: "https://bulenox.com/help-center/master#master-trading-rules", accessed: "2026-10-23" }
  - { field: consistency_rule, url: "https://bulenox.com/help-center/master#withdrawal-rules", accessed: "2026-10-23" }
  - { field: max_position, url: "https://bulenox.com/help-center/master#master-trading-rules", accessed: "2026-10-23" }
  - { field: trading_day_reset, url: "https://bulenox.com/help-center/qualification#rules", accessed: "2026-10-23" }
  - { field: copy_trading_allowed, url: "https://bulenox.com/faq", accessed: "2026-10-23" }
  - { field: automated_trading_allowed, url: "https://bulenox.com/faq", accessed: "2026-10-23" }
  - { field: automated_trading_allowed, url: "https://bulenox.com/legal/Terms_of_Use.pdf", accessed: "2026-10-23" }
  - { field: platforms, url: "https://bulenox.com/", accessed: "2026-10-23" }
  - { field: platforms, url: "https://bulenox.com/faq", accessed: "2026-10-23" }
  - { field: vps_allowed, url: "not mentioned — searched help center (all 9 categories), FAQ, Terms of Use, Risk Disclosure, homepage, accounts-pricing", accessed: "2026-10-23" }
  - { field: personal_device_required, url: "not mentioned — same search", accessed: "2026-10-23" }
```

```yaml
# Master · Option 2 · 150K
firm: bulenox
plan: "Master Account — Option 2 (EOD Account)"
phase: funded
size_label: "150K"
size_usd: "150000.00"
instruments: [MNQ, NQ]
platforms: [Rithmic, "R | Trader Pro", NinjaTrader, Quantower, "Sierra Chart", MultiCharts, ATAS, Bookmap, MotiveWave, "Investor/RT", "Photon Trader", "Trade Navigator", TSLab, ScalpTool, QST, "Medved Trader", OverCharts, QScalp, VolFix, "Inside Edge", "Optimus Flow", Tiger.Trade]
drawdown: { amount_usd: "4500.00", type: trailing_eod, stops_trailing_at: "permanently fixed at starting balance + $100 (threshold $150,100, after the account closes at $154,600)" }
daily_loss_limit: { amount_usd: "3300.00", kind: soft_pause }   # disabled for the remainder of the session, not a violation; permanently removed once the EOD Drawdown locks at starting balance + $100
profit_target_usd: null
consistency_rule: { text: "Your most profitable trading day must not represent more than 40% of your Total P&L for the applicable payout period. (payout condition, not a breach)", threshold_pct: "40" }
max_position: { minis: "15", micros: null, scaling: "Dynamic Scaling Plan (both directions): Cash on Hand up to $4,000: 5 • $4,001–8,000: 8 • $8,001–12,000: 10 • $12,001+: 15; 1 standard contract = 10 micro contracts (micros total not published per size for this plan)" }
min_trading_days: null
trading_day_reset: { time: "17:00", tz_as_published: "North American Central Time (CT)" }   # "starts at 5:00 PM and ends at 4:00 PM the following day"; positions closed by 3:59 PM CT
copy_trading_allowed: conditional   # copiers permitted; only user-built tools for the account owner's personal use
automated_trading_allowed: conditional
automation_rule_text: |-
    Yes. Bots, algorithms and trade copiers are permitted. If you connect to Rithmic through a third-party API or composite software, an additional $100 per month applies.
    Only user-built tools intended solely for the account owner’s personal use are permitted. Commercial, shared, rented or publicly distributed automation tools are not allowed.
    These tools are developed and maintained independently by their users, so their uptime, reliability and performance are outside our control, and we cannot provide support or accept responsibility for any issues arising from their use.

    You agree to not abuse the company provided service programs. It includes but is not limited to using “scalping” algorithms, DTC Protocol Bridge API or automated discretional trading in any of the trading environments. Open and close hundreds or thousand of rapid trades by using any trading algorithms. Implementation of reckless transactions in rough markets to benefit from lack of execution. If abuse is suspected, we reserve the right to refuse to claim any profit. To avoid performance discrepancies and/or meet applicable compliance requirements, any third-party algorithm, automated trading software, or other third-party trading technology must be reviewed and approved by the Bulenox management team before use. Approval may be subject to additional requirements or conditions established by Bulenox.
vps_allowed: unknown               # not mentioned on any official page (see §9 G1)
personal_device_required: unknown  # not mentioned on any official page (see §9 G1)
payout_rules: "10 individual trading days per payout; min $1,000; Safety Threshold Reserve $4,600 above starting balance; inactivity: at least one trade per week"
status: partial
not_confirmed_fields:
  - "automation_rule_text (third-party / commercial tools: FAQ 'Only user-built tools ... Commercial ... not allowed' vs Terms of Use 'any third-party algorithm ... must be reviewed and approved' — see §9 C1)"
sources:
  - { field: drawdown, url: "https://bulenox.com/help-center/master#master-trading-rules", accessed: "2026-10-23" }
  - { field: daily_loss_limit, url: "https://bulenox.com/help-center/master#master-trading-rules", accessed: "2026-10-23" }
  - { field: consistency_rule, url: "https://bulenox.com/help-center/master#withdrawal-rules", accessed: "2026-10-23" }
  - { field: max_position, url: "https://bulenox.com/help-center/master#master-trading-rules", accessed: "2026-10-23" }
  - { field: trading_day_reset, url: "https://bulenox.com/help-center/qualification#rules", accessed: "2026-10-23" }
  - { field: copy_trading_allowed, url: "https://bulenox.com/faq", accessed: "2026-10-23" }
  - { field: automated_trading_allowed, url: "https://bulenox.com/faq", accessed: "2026-10-23" }
  - { field: automated_trading_allowed, url: "https://bulenox.com/legal/Terms_of_Use.pdf", accessed: "2026-10-23" }
  - { field: platforms, url: "https://bulenox.com/", accessed: "2026-10-23" }
  - { field: platforms, url: "https://bulenox.com/faq", accessed: "2026-10-23" }
  - { field: vps_allowed, url: "not mentioned — searched help center (all 9 categories), FAQ, Terms of Use, Risk Disclosure, homepage, accounts-pricing", accessed: "2026-10-23" }
  - { field: personal_device_required, url: "not mentioned — same search", accessed: "2026-10-23" }
```

```yaml
# Fast Track · Option 1 · 25K
firm: bulenox
plan: "Fast Track Account — Option 1 (Trailing Drawdown)"
phase: funded
size_label: "25K"
size_usd: "25000.00"
instruments: [MNQ, NQ]
platforms: [Rithmic, "R | Trader Pro", NinjaTrader, Quantower, "Sierra Chart", MultiCharts, ATAS, Bookmap, MotiveWave, "Investor/RT", "Photon Trader", "Trade Navigator", TSLab, ScalpTool, QST, "Medved Trader", OverCharts, QScalp, VolFix, "Inside Edge", "Optimus Flow", Tiger.Trade]
drawdown: { amount_usd: "1000.00", type: trailing_intraday, stops_trailing_at: "stops for good once it reaches the starting balance plus $100 (fixed floor)" }
daily_loss_limit: { amount_usd: null, kind: none }
profit_target_usd: null
consistency_rule: { text: "Your highest profitable day divided by the net profit of the payout cycle. The maximum is 20% for the first payout, 25% for the second and 30% from the third onwards. Missing it is not a breach", threshold_pct: "20/25/30" }
max_position: { minis: "3", micros: "30", scaling: "none; 1 mini = 10 micros" }
min_trading_days: 0
trading_day_reset: { time: "17:00", tz_as_published: "North American Central Time (CT)" }   # "starts at 5:00 PM and ends at 4:00 PM the following day"; positions closed by 3:59 PM CT
copy_trading_allowed: conditional   # copiers permitted; only user-built tools for the account owner's personal use
automated_trading_allowed: conditional
automation_rule_text: |-
    Yes. Bots, algorithms and trade copiers are permitted. If you connect to Rithmic through a third-party API or composite software, an additional $100 per month applies.
    Only user-built tools intended solely for the account owner’s personal use are permitted. Commercial, shared, rented or publicly distributed automation tools are not allowed.
    These tools are developed and maintained independently by their users, so their uptime, reliability and performance are outside our control, and we cannot provide support or accept responsibility for any issues arising from their use.

    You agree to not abuse the company provided service programs. It includes but is not limited to using “scalping” algorithms, DTC Protocol Bridge API or automated discretional trading in any of the trading environments. Open and close hundreds or thousand of rapid trades by using any trading algorithms. Implementation of reckless transactions in rough markets to benefit from lack of execution. If abuse is suspected, we reserve the right to refuse to claim any profit. To avoid performance discrepancies and/or meet applicable compliance requirements, any third-party algorithm, automated trading software, or other third-party trading technology must be reviewed and approved by the Bulenox management team before use. Approval may be subject to additional requirements or conditions established by Bulenox.
vps_allowed: unknown               # not mentioned on any official page (see §9 G1)
personal_device_required: unknown  # not mentioned on any official page (see §9 G1)
payout_rules: "first payout profit goal $1,500; no minimum trading days; payout may not bring the balance to or below the drawdown threshold"
status: partial
not_confirmed_fields:
  - "automation_rule_text (third-party / commercial tools: FAQ 'Only user-built tools ... Commercial ... not allowed' vs Terms of Use 'any third-party algorithm ... must be reviewed and approved' — see §9 C1)"
sources:
  - { field: drawdown, url: "https://bulenox.com/help-center/fast-track#ft-parameters", accessed: "2026-10-23" }
  - { field: max_position, url: "https://bulenox.com/help-center/fast-track#ft-parameters", accessed: "2026-10-23" }
  - { field: daily_loss_limit, url: "https://bulenox.com/accounts-pricing", accessed: "2026-10-23" }
  - { field: consistency_rule, url: "https://bulenox.com/help-center/fast-track#ft-consistency", accessed: "2026-10-23" }
  - { field: min_trading_days, url: "https://bulenox.com/help-center/fast-track#ft-first-payout", accessed: "2026-10-23" }
  - { field: trading_day_reset, url: "https://bulenox.com/help-center/qualification#rules", accessed: "2026-10-23" }
  - { field: copy_trading_allowed, url: "https://bulenox.com/faq", accessed: "2026-10-23" }
  - { field: automated_trading_allowed, url: "https://bulenox.com/faq", accessed: "2026-10-23" }
  - { field: automated_trading_allowed, url: "https://bulenox.com/legal/Terms_of_Use.pdf", accessed: "2026-10-23" }
  - { field: platforms, url: "https://bulenox.com/", accessed: "2026-10-23" }
  - { field: platforms, url: "https://bulenox.com/faq", accessed: "2026-10-23" }
  - { field: vps_allowed, url: "not mentioned — searched help center (all 9 categories), FAQ, Terms of Use, Risk Disclosure, homepage, accounts-pricing", accessed: "2026-10-23" }
  - { field: personal_device_required, url: "not mentioned — same search", accessed: "2026-10-23" }
```

```yaml
# Fast Track · Option 1 · 50K
firm: bulenox
plan: "Fast Track Account — Option 1 (Trailing Drawdown)"
phase: funded
size_label: "50K"
size_usd: "50000.00"
instruments: [MNQ, NQ]
platforms: [Rithmic, "R | Trader Pro", NinjaTrader, Quantower, "Sierra Chart", MultiCharts, ATAS, Bookmap, MotiveWave, "Investor/RT", "Photon Trader", "Trade Navigator", TSLab, ScalpTool, QST, "Medved Trader", OverCharts, QScalp, VolFix, "Inside Edge", "Optimus Flow", Tiger.Trade]
drawdown: { amount_usd: "2000.00", type: trailing_intraday, stops_trailing_at: "stops for good once it reaches the starting balance plus $100 (fixed floor)" }
daily_loss_limit: { amount_usd: null, kind: none }
profit_target_usd: null
consistency_rule: { text: "Your highest profitable day divided by the net profit of the payout cycle. The maximum is 20% for the first payout, 25% for the second and 30% from the third onwards. Missing it is not a breach", threshold_pct: "20/25/30" }
max_position: { minis: "7", micros: "70", scaling: "none; 1 mini = 10 micros" }
min_trading_days: 0
trading_day_reset: { time: "17:00", tz_as_published: "North American Central Time (CT)" }   # "starts at 5:00 PM and ends at 4:00 PM the following day"; positions closed by 3:59 PM CT
copy_trading_allowed: conditional   # copiers permitted; only user-built tools for the account owner's personal use
automated_trading_allowed: conditional
automation_rule_text: |-
    Yes. Bots, algorithms and trade copiers are permitted. If you connect to Rithmic through a third-party API or composite software, an additional $100 per month applies.
    Only user-built tools intended solely for the account owner’s personal use are permitted. Commercial, shared, rented or publicly distributed automation tools are not allowed.
    These tools are developed and maintained independently by their users, so their uptime, reliability and performance are outside our control, and we cannot provide support or accept responsibility for any issues arising from their use.

    You agree to not abuse the company provided service programs. It includes but is not limited to using “scalping” algorithms, DTC Protocol Bridge API or automated discretional trading in any of the trading environments. Open and close hundreds or thousand of rapid trades by using any trading algorithms. Implementation of reckless transactions in rough markets to benefit from lack of execution. If abuse is suspected, we reserve the right to refuse to claim any profit. To avoid performance discrepancies and/or meet applicable compliance requirements, any third-party algorithm, automated trading software, or other third-party trading technology must be reviewed and approved by the Bulenox management team before use. Approval may be subject to additional requirements or conditions established by Bulenox.
vps_allowed: unknown               # not mentioned on any official page (see §9 G1)
personal_device_required: unknown  # not mentioned on any official page (see §9 G1)
payout_rules: "first payout profit goal $3,000; no minimum trading days; payout may not bring the balance to or below the drawdown threshold"
status: partial
not_confirmed_fields:
  - "automation_rule_text (third-party / commercial tools: FAQ 'Only user-built tools ... Commercial ... not allowed' vs Terms of Use 'any third-party algorithm ... must be reviewed and approved' — see §9 C1)"
sources:
  - { field: drawdown, url: "https://bulenox.com/help-center/fast-track#ft-parameters", accessed: "2026-10-23" }
  - { field: max_position, url: "https://bulenox.com/help-center/fast-track#ft-parameters", accessed: "2026-10-23" }
  - { field: daily_loss_limit, url: "https://bulenox.com/accounts-pricing", accessed: "2026-10-23" }
  - { field: consistency_rule, url: "https://bulenox.com/help-center/fast-track#ft-consistency", accessed: "2026-10-23" }
  - { field: min_trading_days, url: "https://bulenox.com/help-center/fast-track#ft-first-payout", accessed: "2026-10-23" }
  - { field: trading_day_reset, url: "https://bulenox.com/help-center/qualification#rules", accessed: "2026-10-23" }
  - { field: copy_trading_allowed, url: "https://bulenox.com/faq", accessed: "2026-10-23" }
  - { field: automated_trading_allowed, url: "https://bulenox.com/faq", accessed: "2026-10-23" }
  - { field: automated_trading_allowed, url: "https://bulenox.com/legal/Terms_of_Use.pdf", accessed: "2026-10-23" }
  - { field: platforms, url: "https://bulenox.com/", accessed: "2026-10-23" }
  - { field: platforms, url: "https://bulenox.com/faq", accessed: "2026-10-23" }
  - { field: vps_allowed, url: "not mentioned — searched help center (all 9 categories), FAQ, Terms of Use, Risk Disclosure, homepage, accounts-pricing", accessed: "2026-10-23" }
  - { field: personal_device_required, url: "not mentioned — same search", accessed: "2026-10-23" }
```

```yaml
# Fast Track · Option 1 · 100K
firm: bulenox
plan: "Fast Track Account — Option 1 (Trailing Drawdown)"
phase: funded
size_label: "100K"
size_usd: "100000.00"
instruments: [MNQ, NQ]
platforms: [Rithmic, "R | Trader Pro", NinjaTrader, Quantower, "Sierra Chart", MultiCharts, ATAS, Bookmap, MotiveWave, "Investor/RT", "Photon Trader", "Trade Navigator", TSLab, ScalpTool, QST, "Medved Trader", OverCharts, QScalp, VolFix, "Inside Edge", "Optimus Flow", Tiger.Trade]
drawdown: { amount_usd: "4000.00", type: trailing_intraday, stops_trailing_at: "stops for good once it reaches the starting balance plus $100 (fixed floor)" }
daily_loss_limit: { amount_usd: null, kind: none }
profit_target_usd: null
consistency_rule: { text: "Your highest profitable day divided by the net profit of the payout cycle. The maximum is 20% for the first payout, 25% for the second and 30% from the third onwards. Missing it is not a breach", threshold_pct: "20/25/30" }
max_position: { minis: "12", micros: "120", scaling: "none; 1 mini = 10 micros" }
min_trading_days: 0
trading_day_reset: { time: "17:00", tz_as_published: "North American Central Time (CT)" }   # "starts at 5:00 PM and ends at 4:00 PM the following day"; positions closed by 3:59 PM CT
copy_trading_allowed: conditional   # copiers permitted; only user-built tools for the account owner's personal use
automated_trading_allowed: conditional
automation_rule_text: |-
    Yes. Bots, algorithms and trade copiers are permitted. If you connect to Rithmic through a third-party API or composite software, an additional $100 per month applies.
    Only user-built tools intended solely for the account owner’s personal use are permitted. Commercial, shared, rented or publicly distributed automation tools are not allowed.
    These tools are developed and maintained independently by their users, so their uptime, reliability and performance are outside our control, and we cannot provide support or accept responsibility for any issues arising from their use.

    You agree to not abuse the company provided service programs. It includes but is not limited to using “scalping” algorithms, DTC Protocol Bridge API or automated discretional trading in any of the trading environments. Open and close hundreds or thousand of rapid trades by using any trading algorithms. Implementation of reckless transactions in rough markets to benefit from lack of execution. If abuse is suspected, we reserve the right to refuse to claim any profit. To avoid performance discrepancies and/or meet applicable compliance requirements, any third-party algorithm, automated trading software, or other third-party trading technology must be reviewed and approved by the Bulenox management team before use. Approval may be subject to additional requirements or conditions established by Bulenox.
vps_allowed: unknown               # not mentioned on any official page (see §9 G1)
personal_device_required: unknown  # not mentioned on any official page (see §9 G1)
payout_rules: "first payout profit goal $6,000; no minimum trading days; payout may not bring the balance to or below the drawdown threshold"
status: partial
not_confirmed_fields:
  - "automation_rule_text (third-party / commercial tools: FAQ 'Only user-built tools ... Commercial ... not allowed' vs Terms of Use 'any third-party algorithm ... must be reviewed and approved' — see §9 C1)"
sources:
  - { field: drawdown, url: "https://bulenox.com/help-center/fast-track#ft-parameters", accessed: "2026-10-23" }
  - { field: max_position, url: "https://bulenox.com/help-center/fast-track#ft-parameters", accessed: "2026-10-23" }
  - { field: daily_loss_limit, url: "https://bulenox.com/accounts-pricing", accessed: "2026-10-23" }
  - { field: consistency_rule, url: "https://bulenox.com/help-center/fast-track#ft-consistency", accessed: "2026-10-23" }
  - { field: min_trading_days, url: "https://bulenox.com/help-center/fast-track#ft-first-payout", accessed: "2026-10-23" }
  - { field: trading_day_reset, url: "https://bulenox.com/help-center/qualification#rules", accessed: "2026-10-23" }
  - { field: copy_trading_allowed, url: "https://bulenox.com/faq", accessed: "2026-10-23" }
  - { field: automated_trading_allowed, url: "https://bulenox.com/faq", accessed: "2026-10-23" }
  - { field: automated_trading_allowed, url: "https://bulenox.com/legal/Terms_of_Use.pdf", accessed: "2026-10-23" }
  - { field: platforms, url: "https://bulenox.com/", accessed: "2026-10-23" }
  - { field: platforms, url: "https://bulenox.com/faq", accessed: "2026-10-23" }
  - { field: vps_allowed, url: "not mentioned — searched help center (all 9 categories), FAQ, Terms of Use, Risk Disclosure, homepage, accounts-pricing", accessed: "2026-10-23" }
  - { field: personal_device_required, url: "not mentioned — same search", accessed: "2026-10-23" }
```

```yaml
# Fast Track · Option 1 · 150K
firm: bulenox
plan: "Fast Track Account — Option 1 (Trailing Drawdown)"
phase: funded
size_label: "150K"
size_usd: "150000.00"
instruments: [MNQ, NQ]
platforms: [Rithmic, "R | Trader Pro", NinjaTrader, Quantower, "Sierra Chart", MultiCharts, ATAS, Bookmap, MotiveWave, "Investor/RT", "Photon Trader", "Trade Navigator", TSLab, ScalpTool, QST, "Medved Trader", OverCharts, QScalp, VolFix, "Inside Edge", "Optimus Flow", Tiger.Trade]
drawdown: { amount_usd: "5500.00", type: trailing_intraday, stops_trailing_at: "stops for good once it reaches the starting balance plus $100 (fixed floor)" }
daily_loss_limit: { amount_usd: null, kind: none }
profit_target_usd: null
consistency_rule: { text: "Your highest profitable day divided by the net profit of the payout cycle. The maximum is 20% for the first payout, 25% for the second and 30% from the third onwards. Missing it is not a breach", threshold_pct: "20/25/30" }
max_position: { minis: "15", micros: "150", scaling: "none; 1 mini = 10 micros" }
min_trading_days: 0
trading_day_reset: { time: "17:00", tz_as_published: "North American Central Time (CT)" }   # "starts at 5:00 PM and ends at 4:00 PM the following day"; positions closed by 3:59 PM CT
copy_trading_allowed: conditional   # copiers permitted; only user-built tools for the account owner's personal use
automated_trading_allowed: conditional
automation_rule_text: |-
    Yes. Bots, algorithms and trade copiers are permitted. If you connect to Rithmic through a third-party API or composite software, an additional $100 per month applies.
    Only user-built tools intended solely for the account owner’s personal use are permitted. Commercial, shared, rented or publicly distributed automation tools are not allowed.
    These tools are developed and maintained independently by their users, so their uptime, reliability and performance are outside our control, and we cannot provide support or accept responsibility for any issues arising from their use.

    You agree to not abuse the company provided service programs. It includes but is not limited to using “scalping” algorithms, DTC Protocol Bridge API or automated discretional trading in any of the trading environments. Open and close hundreds or thousand of rapid trades by using any trading algorithms. Implementation of reckless transactions in rough markets to benefit from lack of execution. If abuse is suspected, we reserve the right to refuse to claim any profit. To avoid performance discrepancies and/or meet applicable compliance requirements, any third-party algorithm, automated trading software, or other third-party trading technology must be reviewed and approved by the Bulenox management team before use. Approval may be subject to additional requirements or conditions established by Bulenox.
vps_allowed: unknown               # not mentioned on any official page (see §9 G1)
personal_device_required: unknown  # not mentioned on any official page (see §9 G1)
payout_rules: "first payout profit goal $9,000; no minimum trading days; payout may not bring the balance to or below the drawdown threshold"
status: partial
not_confirmed_fields:
  - "automation_rule_text (third-party / commercial tools: FAQ 'Only user-built tools ... Commercial ... not allowed' vs Terms of Use 'any third-party algorithm ... must be reviewed and approved' — see §9 C1)"
sources:
  - { field: drawdown, url: "https://bulenox.com/help-center/fast-track#ft-parameters", accessed: "2026-10-23" }
  - { field: max_position, url: "https://bulenox.com/help-center/fast-track#ft-parameters", accessed: "2026-10-23" }
  - { field: daily_loss_limit, url: "https://bulenox.com/accounts-pricing", accessed: "2026-10-23" }
  - { field: consistency_rule, url: "https://bulenox.com/help-center/fast-track#ft-consistency", accessed: "2026-10-23" }
  - { field: min_trading_days, url: "https://bulenox.com/help-center/fast-track#ft-first-payout", accessed: "2026-10-23" }
  - { field: trading_day_reset, url: "https://bulenox.com/help-center/qualification#rules", accessed: "2026-10-23" }
  - { field: copy_trading_allowed, url: "https://bulenox.com/faq", accessed: "2026-10-23" }
  - { field: automated_trading_allowed, url: "https://bulenox.com/faq", accessed: "2026-10-23" }
  - { field: automated_trading_allowed, url: "https://bulenox.com/legal/Terms_of_Use.pdf", accessed: "2026-10-23" }
  - { field: platforms, url: "https://bulenox.com/", accessed: "2026-10-23" }
  - { field: platforms, url: "https://bulenox.com/faq", accessed: "2026-10-23" }
  - { field: vps_allowed, url: "not mentioned — searched help center (all 9 categories), FAQ, Terms of Use, Risk Disclosure, homepage, accounts-pricing", accessed: "2026-10-23" }
  - { field: personal_device_required, url: "not mentioned — same search", accessed: "2026-10-23" }
```

```yaml
# Fast Track · Option 2 · 25K
firm: bulenox
plan: "Fast Track Account — Option 2 (EOD Drawdown / No scaling)"
phase: funded
size_label: "25K"
size_usd: "25000.00"
instruments: [MNQ, NQ]
platforms: [Rithmic, "R | Trader Pro", NinjaTrader, Quantower, "Sierra Chart", MultiCharts, ATAS, Bookmap, MotiveWave, "Investor/RT", "Photon Trader", "Trade Navigator", TSLab, ScalpTool, QST, "Medved Trader", OverCharts, QScalp, VolFix, "Inside Edge", "Optimus Flow", Tiger.Trade]
drawdown: { amount_usd: "1000.00", type: trailing_eod, stops_trailing_at: "stops for good once it reaches the starting balance plus $100 (fixed floor)" }
daily_loss_limit: { amount_usd: null, kind: none }   # 'The $25,000 Option 2 account is the exception: it has no daily loss limit.'
profit_target_usd: null
consistency_rule: { text: "Your highest profitable day divided by the net profit of the payout cycle. The maximum is 20% for the first payout, 25% for the second and 30% from the third onwards. Missing it is not a breach", threshold_pct: "20/25/30" }
max_position: { minis: "2", micros: "20", scaling: "none (Option 2 has no scaling plan); 1 mini = 10 micros" }
min_trading_days: 0
trading_day_reset: { time: "17:00", tz_as_published: "North American Central Time (CT)" }   # "starts at 5:00 PM and ends at 4:00 PM the following day"; positions closed by 3:59 PM CT
copy_trading_allowed: conditional   # copiers permitted; only user-built tools for the account owner's personal use
automated_trading_allowed: conditional
automation_rule_text: |-
    Yes. Bots, algorithms and trade copiers are permitted. If you connect to Rithmic through a third-party API or composite software, an additional $100 per month applies.
    Only user-built tools intended solely for the account owner’s personal use are permitted. Commercial, shared, rented or publicly distributed automation tools are not allowed.
    These tools are developed and maintained independently by their users, so their uptime, reliability and performance are outside our control, and we cannot provide support or accept responsibility for any issues arising from their use.

    You agree to not abuse the company provided service programs. It includes but is not limited to using “scalping” algorithms, DTC Protocol Bridge API or automated discretional trading in any of the trading environments. Open and close hundreds or thousand of rapid trades by using any trading algorithms. Implementation of reckless transactions in rough markets to benefit from lack of execution. If abuse is suspected, we reserve the right to refuse to claim any profit. To avoid performance discrepancies and/or meet applicable compliance requirements, any third-party algorithm, automated trading software, or other third-party trading technology must be reviewed and approved by the Bulenox management team before use. Approval may be subject to additional requirements or conditions established by Bulenox.
vps_allowed: unknown               # not mentioned on any official page (see §9 G1)
personal_device_required: unknown  # not mentioned on any official page (see §9 G1)
payout_rules: "first payout profit goal $1,500; no minimum trading days; payout may not bring the balance to or below the drawdown threshold"
status: partial
not_confirmed_fields:
  - "automation_rule_text (third-party / commercial tools: FAQ 'Only user-built tools ... Commercial ... not allowed' vs Terms of Use 'any third-party algorithm ... must be reviewed and approved' — see §9 C1)"
sources:
  - { field: drawdown, url: "https://bulenox.com/help-center/fast-track#ft-parameters", accessed: "2026-10-23" }
  - { field: max_position, url: "https://bulenox.com/help-center/fast-track#ft-parameters", accessed: "2026-10-23" }
  - { field: daily_loss_limit, url: "https://bulenox.com/help-center/fast-track#ft-parameters", accessed: "2026-10-23" }
  - { field: daily_loss_limit.kind, url: "https://bulenox.com/accounts-pricing", accessed: "2026-10-23" }
  - { field: consistency_rule, url: "https://bulenox.com/help-center/fast-track#ft-consistency", accessed: "2026-10-23" }
  - { field: min_trading_days, url: "https://bulenox.com/help-center/fast-track#ft-first-payout", accessed: "2026-10-23" }
  - { field: trading_day_reset, url: "https://bulenox.com/help-center/qualification#rules", accessed: "2026-10-23" }
  - { field: copy_trading_allowed, url: "https://bulenox.com/faq", accessed: "2026-10-23" }
  - { field: automated_trading_allowed, url: "https://bulenox.com/faq", accessed: "2026-10-23" }
  - { field: automated_trading_allowed, url: "https://bulenox.com/legal/Terms_of_Use.pdf", accessed: "2026-10-23" }
  - { field: platforms, url: "https://bulenox.com/", accessed: "2026-10-23" }
  - { field: platforms, url: "https://bulenox.com/faq", accessed: "2026-10-23" }
  - { field: vps_allowed, url: "not mentioned — searched help center (all 9 categories), FAQ, Terms of Use, Risk Disclosure, homepage, accounts-pricing", accessed: "2026-10-23" }
  - { field: personal_device_required, url: "not mentioned — same search", accessed: "2026-10-23" }
```

```yaml
# Fast Track · Option 2 · 50K
firm: bulenox
plan: "Fast Track Account — Option 2 (EOD Drawdown / No scaling)"
phase: funded
size_label: "50K"
size_usd: "50000.00"
instruments: [MNQ, NQ]
platforms: [Rithmic, "R | Trader Pro", NinjaTrader, Quantower, "Sierra Chart", MultiCharts, ATAS, Bookmap, MotiveWave, "Investor/RT", "Photon Trader", "Trade Navigator", TSLab, ScalpTool, QST, "Medved Trader", OverCharts, QScalp, VolFix, "Inside Edge", "Optimus Flow", Tiger.Trade]
drawdown: { amount_usd: "2000.00", type: trailing_eod, stops_trailing_at: "stops for good once it reaches the starting balance plus $100 (fixed floor)" }
daily_loss_limit: { amount_usd: "1200.00", kind: soft_pause }   # 'stays active for the whole life of the account and is never lifted'
profit_target_usd: null
consistency_rule: { text: "Your highest profitable day divided by the net profit of the payout cycle. The maximum is 20% for the first payout, 25% for the second and 30% from the third onwards. Missing it is not a breach", threshold_pct: "20/25/30" }
max_position: { minis: "4", micros: "40", scaling: "none (Option 2 has no scaling plan); 1 mini = 10 micros" }
min_trading_days: 0
trading_day_reset: { time: "17:00", tz_as_published: "North American Central Time (CT)" }   # "starts at 5:00 PM and ends at 4:00 PM the following day"; positions closed by 3:59 PM CT
copy_trading_allowed: conditional   # copiers permitted; only user-built tools for the account owner's personal use
automated_trading_allowed: conditional
automation_rule_text: |-
    Yes. Bots, algorithms and trade copiers are permitted. If you connect to Rithmic through a third-party API or composite software, an additional $100 per month applies.
    Only user-built tools intended solely for the account owner’s personal use are permitted. Commercial, shared, rented or publicly distributed automation tools are not allowed.
    These tools are developed and maintained independently by their users, so their uptime, reliability and performance are outside our control, and we cannot provide support or accept responsibility for any issues arising from their use.

    You agree to not abuse the company provided service programs. It includes but is not limited to using “scalping” algorithms, DTC Protocol Bridge API or automated discretional trading in any of the trading environments. Open and close hundreds or thousand of rapid trades by using any trading algorithms. Implementation of reckless transactions in rough markets to benefit from lack of execution. If abuse is suspected, we reserve the right to refuse to claim any profit. To avoid performance discrepancies and/or meet applicable compliance requirements, any third-party algorithm, automated trading software, or other third-party trading technology must be reviewed and approved by the Bulenox management team before use. Approval may be subject to additional requirements or conditions established by Bulenox.
vps_allowed: unknown               # not mentioned on any official page (see §9 G1)
personal_device_required: unknown  # not mentioned on any official page (see §9 G1)
payout_rules: "first payout profit goal $3,000; no minimum trading days; payout may not bring the balance to or below the drawdown threshold"
status: partial
not_confirmed_fields:
  - "daily_loss_limit.kind (pause-for-the-day behaviour is stated generically for Option 2 on the pricing page, not restated on the Fast Track page)"
  - "automation_rule_text (third-party / commercial tools: FAQ 'Only user-built tools ... Commercial ... not allowed' vs Terms of Use 'any third-party algorithm ... must be reviewed and approved' — see §9 C1)"
sources:
  - { field: drawdown, url: "https://bulenox.com/help-center/fast-track#ft-parameters", accessed: "2026-10-23" }
  - { field: max_position, url: "https://bulenox.com/help-center/fast-track#ft-parameters", accessed: "2026-10-23" }
  - { field: daily_loss_limit, url: "https://bulenox.com/help-center/fast-track#ft-parameters", accessed: "2026-10-23" }
  - { field: daily_loss_limit.kind, url: "https://bulenox.com/accounts-pricing", accessed: "2026-10-23" }
  - { field: consistency_rule, url: "https://bulenox.com/help-center/fast-track#ft-consistency", accessed: "2026-10-23" }
  - { field: min_trading_days, url: "https://bulenox.com/help-center/fast-track#ft-first-payout", accessed: "2026-10-23" }
  - { field: trading_day_reset, url: "https://bulenox.com/help-center/qualification#rules", accessed: "2026-10-23" }
  - { field: copy_trading_allowed, url: "https://bulenox.com/faq", accessed: "2026-10-23" }
  - { field: automated_trading_allowed, url: "https://bulenox.com/faq", accessed: "2026-10-23" }
  - { field: automated_trading_allowed, url: "https://bulenox.com/legal/Terms_of_Use.pdf", accessed: "2026-10-23" }
  - { field: platforms, url: "https://bulenox.com/", accessed: "2026-10-23" }
  - { field: platforms, url: "https://bulenox.com/faq", accessed: "2026-10-23" }
  - { field: vps_allowed, url: "not mentioned — searched help center (all 9 categories), FAQ, Terms of Use, Risk Disclosure, homepage, accounts-pricing", accessed: "2026-10-23" }
  - { field: personal_device_required, url: "not mentioned — same search", accessed: "2026-10-23" }
```

```yaml
# Fast Track · Option 2 · 100K
firm: bulenox
plan: "Fast Track Account — Option 2 (EOD Drawdown / No scaling)"
phase: funded
size_label: "100K"
size_usd: "100000.00"
instruments: [MNQ, NQ]
platforms: [Rithmic, "R | Trader Pro", NinjaTrader, Quantower, "Sierra Chart", MultiCharts, ATAS, Bookmap, MotiveWave, "Investor/RT", "Photon Trader", "Trade Navigator", TSLab, ScalpTool, QST, "Medved Trader", OverCharts, QScalp, VolFix, "Inside Edge", "Optimus Flow", Tiger.Trade]
drawdown: { amount_usd: "4000.00", type: trailing_eod, stops_trailing_at: "stops for good once it reaches the starting balance plus $100 (fixed floor)" }
daily_loss_limit: { amount_usd: "2500.00", kind: soft_pause }   # 'stays active for the whole life of the account and is never lifted'
profit_target_usd: null
consistency_rule: { text: "Your highest profitable day divided by the net profit of the payout cycle. The maximum is 20% for the first payout, 25% for the second and 30% from the third onwards. Missing it is not a breach", threshold_pct: "20/25/30" }
max_position: { minis: "8", micros: "80", scaling: "none (Option 2 has no scaling plan); 1 mini = 10 micros" }
min_trading_days: 0
trading_day_reset: { time: "17:00", tz_as_published: "North American Central Time (CT)" }   # "starts at 5:00 PM and ends at 4:00 PM the following day"; positions closed by 3:59 PM CT
copy_trading_allowed: conditional   # copiers permitted; only user-built tools for the account owner's personal use
automated_trading_allowed: conditional
automation_rule_text: |-
    Yes. Bots, algorithms and trade copiers are permitted. If you connect to Rithmic through a third-party API or composite software, an additional $100 per month applies.
    Only user-built tools intended solely for the account owner’s personal use are permitted. Commercial, shared, rented or publicly distributed automation tools are not allowed.
    These tools are developed and maintained independently by their users, so their uptime, reliability and performance are outside our control, and we cannot provide support or accept responsibility for any issues arising from their use.

    You agree to not abuse the company provided service programs. It includes but is not limited to using “scalping” algorithms, DTC Protocol Bridge API or automated discretional trading in any of the trading environments. Open and close hundreds or thousand of rapid trades by using any trading algorithms. Implementation of reckless transactions in rough markets to benefit from lack of execution. If abuse is suspected, we reserve the right to refuse to claim any profit. To avoid performance discrepancies and/or meet applicable compliance requirements, any third-party algorithm, automated trading software, or other third-party trading technology must be reviewed and approved by the Bulenox management team before use. Approval may be subject to additional requirements or conditions established by Bulenox.
vps_allowed: unknown               # not mentioned on any official page (see §9 G1)
personal_device_required: unknown  # not mentioned on any official page (see §9 G1)
payout_rules: "first payout profit goal $6,000; no minimum trading days; payout may not bring the balance to or below the drawdown threshold"
status: partial
not_confirmed_fields:
  - "daily_loss_limit.kind (pause-for-the-day behaviour is stated generically for Option 2 on the pricing page, not restated on the Fast Track page)"
  - "automation_rule_text (third-party / commercial tools: FAQ 'Only user-built tools ... Commercial ... not allowed' vs Terms of Use 'any third-party algorithm ... must be reviewed and approved' — see §9 C1)"
sources:
  - { field: drawdown, url: "https://bulenox.com/help-center/fast-track#ft-parameters", accessed: "2026-10-23" }
  - { field: max_position, url: "https://bulenox.com/help-center/fast-track#ft-parameters", accessed: "2026-10-23" }
  - { field: daily_loss_limit, url: "https://bulenox.com/help-center/fast-track#ft-parameters", accessed: "2026-10-23" }
  - { field: daily_loss_limit.kind, url: "https://bulenox.com/accounts-pricing", accessed: "2026-10-23" }
  - { field: consistency_rule, url: "https://bulenox.com/help-center/fast-track#ft-consistency", accessed: "2026-10-23" }
  - { field: min_trading_days, url: "https://bulenox.com/help-center/fast-track#ft-first-payout", accessed: "2026-10-23" }
  - { field: trading_day_reset, url: "https://bulenox.com/help-center/qualification#rules", accessed: "2026-10-23" }
  - { field: copy_trading_allowed, url: "https://bulenox.com/faq", accessed: "2026-10-23" }
  - { field: automated_trading_allowed, url: "https://bulenox.com/faq", accessed: "2026-10-23" }
  - { field: automated_trading_allowed, url: "https://bulenox.com/legal/Terms_of_Use.pdf", accessed: "2026-10-23" }
  - { field: platforms, url: "https://bulenox.com/", accessed: "2026-10-23" }
  - { field: platforms, url: "https://bulenox.com/faq", accessed: "2026-10-23" }
  - { field: vps_allowed, url: "not mentioned — searched help center (all 9 categories), FAQ, Terms of Use, Risk Disclosure, homepage, accounts-pricing", accessed: "2026-10-23" }
  - { field: personal_device_required, url: "not mentioned — same search", accessed: "2026-10-23" }
```

```yaml
# Fast Track · Option 2 · 150K
firm: bulenox
plan: "Fast Track Account — Option 2 (EOD Drawdown / No scaling)"
phase: funded
size_label: "150K"
size_usd: "150000.00"
instruments: [MNQ, NQ]
platforms: [Rithmic, "R | Trader Pro", NinjaTrader, Quantower, "Sierra Chart", MultiCharts, ATAS, Bookmap, MotiveWave, "Investor/RT", "Photon Trader", "Trade Navigator", TSLab, ScalpTool, QST, "Medved Trader", OverCharts, QScalp, VolFix, "Inside Edge", "Optimus Flow", Tiger.Trade]
drawdown: { amount_usd: "5500.00", type: trailing_eod, stops_trailing_at: "stops for good once it reaches the starting balance plus $100 (fixed floor)" }
daily_loss_limit: { amount_usd: "3300.00", kind: soft_pause }   # 'stays active for the whole life of the account and is never lifted'
profit_target_usd: null
consistency_rule: { text: "Your highest profitable day divided by the net profit of the payout cycle. The maximum is 20% for the first payout, 25% for the second and 30% from the third onwards. Missing it is not a breach", threshold_pct: "20/25/30" }
max_position: { minis: "12", micros: "120", scaling: "none (Option 2 has no scaling plan); 1 mini = 10 micros" }
min_trading_days: 0
trading_day_reset: { time: "17:00", tz_as_published: "North American Central Time (CT)" }   # "starts at 5:00 PM and ends at 4:00 PM the following day"; positions closed by 3:59 PM CT
copy_trading_allowed: conditional   # copiers permitted; only user-built tools for the account owner's personal use
automated_trading_allowed: conditional
automation_rule_text: |-
    Yes. Bots, algorithms and trade copiers are permitted. If you connect to Rithmic through a third-party API or composite software, an additional $100 per month applies.
    Only user-built tools intended solely for the account owner’s personal use are permitted. Commercial, shared, rented or publicly distributed automation tools are not allowed.
    These tools are developed and maintained independently by their users, so their uptime, reliability and performance are outside our control, and we cannot provide support or accept responsibility for any issues arising from their use.

    You agree to not abuse the company provided service programs. It includes but is not limited to using “scalping” algorithms, DTC Protocol Bridge API or automated discretional trading in any of the trading environments. Open and close hundreds or thousand of rapid trades by using any trading algorithms. Implementation of reckless transactions in rough markets to benefit from lack of execution. If abuse is suspected, we reserve the right to refuse to claim any profit. To avoid performance discrepancies and/or meet applicable compliance requirements, any third-party algorithm, automated trading software, or other third-party trading technology must be reviewed and approved by the Bulenox management team before use. Approval may be subject to additional requirements or conditions established by Bulenox.
vps_allowed: unknown               # not mentioned on any official page (see §9 G1)
personal_device_required: unknown  # not mentioned on any official page (see §9 G1)
payout_rules: "first payout profit goal $9,000; no minimum trading days; payout may not bring the balance to or below the drawdown threshold"
status: partial
not_confirmed_fields:
  - "daily_loss_limit.kind (pause-for-the-day behaviour is stated generically for Option 2 on the pricing page, not restated on the Fast Track page)"
  - "automation_rule_text (third-party / commercial tools: FAQ 'Only user-built tools ... Commercial ... not allowed' vs Terms of Use 'any third-party algorithm ... must be reviewed and approved' — see §9 C1)"
sources:
  - { field: drawdown, url: "https://bulenox.com/help-center/fast-track#ft-parameters", accessed: "2026-10-23" }
  - { field: max_position, url: "https://bulenox.com/help-center/fast-track#ft-parameters", accessed: "2026-10-23" }
  - { field: daily_loss_limit, url: "https://bulenox.com/help-center/fast-track#ft-parameters", accessed: "2026-10-23" }
  - { field: daily_loss_limit.kind, url: "https://bulenox.com/accounts-pricing", accessed: "2026-10-23" }
  - { field: consistency_rule, url: "https://bulenox.com/help-center/fast-track#ft-consistency", accessed: "2026-10-23" }
  - { field: min_trading_days, url: "https://bulenox.com/help-center/fast-track#ft-first-payout", accessed: "2026-10-23" }
  - { field: trading_day_reset, url: "https://bulenox.com/help-center/qualification#rules", accessed: "2026-10-23" }
  - { field: copy_trading_allowed, url: "https://bulenox.com/faq", accessed: "2026-10-23" }
  - { field: automated_trading_allowed, url: "https://bulenox.com/faq", accessed: "2026-10-23" }
  - { field: automated_trading_allowed, url: "https://bulenox.com/legal/Terms_of_Use.pdf", accessed: "2026-10-23" }
  - { field: platforms, url: "https://bulenox.com/", accessed: "2026-10-23" }
  - { field: platforms, url: "https://bulenox.com/faq", accessed: "2026-10-23" }
  - { field: vps_allowed, url: "not mentioned — searched help center (all 9 categories), FAQ, Terms of Use, Risk Disclosure, homepage, accounts-pricing", accessed: "2026-10-23" }
  - { field: personal_device_required, url: "not mentioned — same search", accessed: "2026-10-23" }
```

```yaml
# Momentum (qualification stage) · Option 1 · 25K
firm: bulenox
plan: "Momentum Account — Option 1 (Trailing Drawdown)"
phase: evaluation
size_label: "25K"
size_usd: "25000.00"
instruments: [MNQ, NQ]
platforms: [Rithmic, "R | Trader Pro", NinjaTrader, Quantower, "Sierra Chart", MultiCharts, ATAS, Bookmap, MotiveWave, "Investor/RT", "Photon Trader", "Trade Navigator", TSLab, ScalpTool, QST, "Medved Trader", OverCharts, QScalp, VolFix, "Inside Edge", "Optimus Flow", Tiger.Trade]
drawdown: { amount_usd: "1000.00", type: trailing_intraday, stops_trailing_at: null }
daily_loss_limit: { amount_usd: null, kind: none }
profit_target_usd: "1500.00"
consistency_rule: { text: "There is no consistency requirement at this stage.", threshold_pct: null }
max_position: { minis: "3", micros: "30", scaling: "none; 1 mini = 10 micros" }
min_trading_days: 0
trading_day_reset: { time: "17:00", tz_as_published: "North American Central Time (CT)" }   # "starts at 5:00 PM and ends at 4:00 PM the following day"; positions closed by 3:59 PM CT
copy_trading_allowed: conditional   # copiers permitted; only user-built tools for the account owner's personal use
automated_trading_allowed: conditional
automation_rule_text: |-
    Yes. Bots, algorithms and trade copiers are permitted. If you connect to Rithmic through a third-party API or composite software, an additional $100 per month applies.
    Only user-built tools intended solely for the account owner’s personal use are permitted. Commercial, shared, rented or publicly distributed automation tools are not allowed.
    These tools are developed and maintained independently by their users, so their uptime, reliability and performance are outside our control, and we cannot provide support or accept responsibility for any issues arising from their use.

    You agree to not abuse the company provided service programs. It includes but is not limited to using “scalping” algorithms, DTC Protocol Bridge API or automated discretional trading in any of the trading environments. Open and close hundreds or thousand of rapid trades by using any trading algorithms. Implementation of reckless transactions in rough markets to benefit from lack of execution. If abuse is suspected, we reserve the right to refuse to claim any profit. To avoid performance discrepancies and/or meet applicable compliance requirements, any third-party algorithm, automated trading software, or other third-party trading technology must be reviewed and approved by the Bulenox management team before use. Approval may be subject to additional requirements or conditions established by Bulenox.
vps_allowed: unknown               # not mentioned on any official page (see §9 G1)
personal_device_required: unknown  # not mentioned on any official page (see §9 G1)
status: partial
not_confirmed_fields:
  - "drawdown.stops_trailing_at (not published for Momentum)"
  - "automation_rule_text (third-party / commercial tools: FAQ 'Only user-built tools ... Commercial ... not allowed' vs Terms of Use 'any third-party algorithm ... must be reviewed and approved' — see §9 C1)"
sources:
  - { field: drawdown, url: "https://bulenox.com/help-center/momentum#mo-parameters", accessed: "2026-10-23" }
  - { field: profit_target_usd, url: "https://bulenox.com/help-center/momentum#mo-parameters", accessed: "2026-10-23" }
  - { field: max_position, url: "https://bulenox.com/help-center/momentum#mo-parameters", accessed: "2026-10-23" }
  - { field: daily_loss_limit, url: "https://bulenox.com/accounts-pricing", accessed: "2026-10-23" }
  - { field: consistency_rule, url: "https://bulenox.com/help-center/momentum#mo-rules", accessed: "2026-10-23" }
  - { field: min_trading_days, url: "https://bulenox.com/help-center/momentum#mo-rules", accessed: "2026-10-23" }
  - { field: trading_day_reset, url: "https://bulenox.com/help-center/qualification#rules", accessed: "2026-10-23" }
  - { field: copy_trading_allowed, url: "https://bulenox.com/faq", accessed: "2026-10-23" }
  - { field: automated_trading_allowed, url: "https://bulenox.com/faq", accessed: "2026-10-23" }
  - { field: automated_trading_allowed, url: "https://bulenox.com/legal/Terms_of_Use.pdf", accessed: "2026-10-23" }
  - { field: platforms, url: "https://bulenox.com/", accessed: "2026-10-23" }
  - { field: platforms, url: "https://bulenox.com/faq", accessed: "2026-10-23" }
  - { field: vps_allowed, url: "not mentioned — searched help center (all 9 categories), FAQ, Terms of Use, Risk Disclosure, homepage, accounts-pricing", accessed: "2026-10-23" }
  - { field: personal_device_required, url: "not mentioned — same search", accessed: "2026-10-23" }
```

```yaml
# Momentum (qualification stage) · Option 1 · 50K
firm: bulenox
plan: "Momentum Account — Option 1 (Trailing Drawdown)"
phase: evaluation
size_label: "50K"
size_usd: "50000.00"
instruments: [MNQ, NQ]
platforms: [Rithmic, "R | Trader Pro", NinjaTrader, Quantower, "Sierra Chart", MultiCharts, ATAS, Bookmap, MotiveWave, "Investor/RT", "Photon Trader", "Trade Navigator", TSLab, ScalpTool, QST, "Medved Trader", OverCharts, QScalp, VolFix, "Inside Edge", "Optimus Flow", Tiger.Trade]
drawdown: { amount_usd: "2000.00", type: trailing_intraday, stops_trailing_at: null }
daily_loss_limit: { amount_usd: null, kind: none }
profit_target_usd: "3000.00"
consistency_rule: { text: "There is no consistency requirement at this stage.", threshold_pct: null }
max_position: { minis: "7", micros: "70", scaling: "none; 1 mini = 10 micros" }
min_trading_days: 0
trading_day_reset: { time: "17:00", tz_as_published: "North American Central Time (CT)" }   # "starts at 5:00 PM and ends at 4:00 PM the following day"; positions closed by 3:59 PM CT
copy_trading_allowed: conditional   # copiers permitted; only user-built tools for the account owner's personal use
automated_trading_allowed: conditional
automation_rule_text: |-
    Yes. Bots, algorithms and trade copiers are permitted. If you connect to Rithmic through a third-party API or composite software, an additional $100 per month applies.
    Only user-built tools intended solely for the account owner’s personal use are permitted. Commercial, shared, rented or publicly distributed automation tools are not allowed.
    These tools are developed and maintained independently by their users, so their uptime, reliability and performance are outside our control, and we cannot provide support or accept responsibility for any issues arising from their use.

    You agree to not abuse the company provided service programs. It includes but is not limited to using “scalping” algorithms, DTC Protocol Bridge API or automated discretional trading in any of the trading environments. Open and close hundreds or thousand of rapid trades by using any trading algorithms. Implementation of reckless transactions in rough markets to benefit from lack of execution. If abuse is suspected, we reserve the right to refuse to claim any profit. To avoid performance discrepancies and/or meet applicable compliance requirements, any third-party algorithm, automated trading software, or other third-party trading technology must be reviewed and approved by the Bulenox management team before use. Approval may be subject to additional requirements or conditions established by Bulenox.
vps_allowed: unknown               # not mentioned on any official page (see §9 G1)
personal_device_required: unknown  # not mentioned on any official page (see §9 G1)
status: partial
not_confirmed_fields:
  - "drawdown.stops_trailing_at (not published for Momentum)"
  - "automation_rule_text (third-party / commercial tools: FAQ 'Only user-built tools ... Commercial ... not allowed' vs Terms of Use 'any third-party algorithm ... must be reviewed and approved' — see §9 C1)"
sources:
  - { field: drawdown, url: "https://bulenox.com/help-center/momentum#mo-parameters", accessed: "2026-10-23" }
  - { field: profit_target_usd, url: "https://bulenox.com/help-center/momentum#mo-parameters", accessed: "2026-10-23" }
  - { field: max_position, url: "https://bulenox.com/help-center/momentum#mo-parameters", accessed: "2026-10-23" }
  - { field: daily_loss_limit, url: "https://bulenox.com/accounts-pricing", accessed: "2026-10-23" }
  - { field: consistency_rule, url: "https://bulenox.com/help-center/momentum#mo-rules", accessed: "2026-10-23" }
  - { field: min_trading_days, url: "https://bulenox.com/help-center/momentum#mo-rules", accessed: "2026-10-23" }
  - { field: trading_day_reset, url: "https://bulenox.com/help-center/qualification#rules", accessed: "2026-10-23" }
  - { field: copy_trading_allowed, url: "https://bulenox.com/faq", accessed: "2026-10-23" }
  - { field: automated_trading_allowed, url: "https://bulenox.com/faq", accessed: "2026-10-23" }
  - { field: automated_trading_allowed, url: "https://bulenox.com/legal/Terms_of_Use.pdf", accessed: "2026-10-23" }
  - { field: platforms, url: "https://bulenox.com/", accessed: "2026-10-23" }
  - { field: platforms, url: "https://bulenox.com/faq", accessed: "2026-10-23" }
  - { field: vps_allowed, url: "not mentioned — searched help center (all 9 categories), FAQ, Terms of Use, Risk Disclosure, homepage, accounts-pricing", accessed: "2026-10-23" }
  - { field: personal_device_required, url: "not mentioned — same search", accessed: "2026-10-23" }
```

```yaml
# Momentum (qualification stage) · Option 1 · 100K
firm: bulenox
plan: "Momentum Account — Option 1 (Trailing Drawdown)"
phase: evaluation
size_label: "100K"
size_usd: "100000.00"
instruments: [MNQ, NQ]
platforms: [Rithmic, "R | Trader Pro", NinjaTrader, Quantower, "Sierra Chart", MultiCharts, ATAS, Bookmap, MotiveWave, "Investor/RT", "Photon Trader", "Trade Navigator", TSLab, ScalpTool, QST, "Medved Trader", OverCharts, QScalp, VolFix, "Inside Edge", "Optimus Flow", Tiger.Trade]
drawdown: { amount_usd: "3000.00", type: trailing_intraday, stops_trailing_at: null }
daily_loss_limit: { amount_usd: null, kind: none }
profit_target_usd: "6000.00"
consistency_rule: { text: "There is no consistency requirement at this stage.", threshold_pct: null }
max_position: { minis: "12", micros: "120", scaling: "none; 1 mini = 10 micros" }
min_trading_days: 0
trading_day_reset: { time: "17:00", tz_as_published: "North American Central Time (CT)" }   # "starts at 5:00 PM and ends at 4:00 PM the following day"; positions closed by 3:59 PM CT
copy_trading_allowed: conditional   # copiers permitted; only user-built tools for the account owner's personal use
automated_trading_allowed: conditional
automation_rule_text: |-
    Yes. Bots, algorithms and trade copiers are permitted. If you connect to Rithmic through a third-party API or composite software, an additional $100 per month applies.
    Only user-built tools intended solely for the account owner’s personal use are permitted. Commercial, shared, rented or publicly distributed automation tools are not allowed.
    These tools are developed and maintained independently by their users, so their uptime, reliability and performance are outside our control, and we cannot provide support or accept responsibility for any issues arising from their use.

    You agree to not abuse the company provided service programs. It includes but is not limited to using “scalping” algorithms, DTC Protocol Bridge API or automated discretional trading in any of the trading environments. Open and close hundreds or thousand of rapid trades by using any trading algorithms. Implementation of reckless transactions in rough markets to benefit from lack of execution. If abuse is suspected, we reserve the right to refuse to claim any profit. To avoid performance discrepancies and/or meet applicable compliance requirements, any third-party algorithm, automated trading software, or other third-party trading technology must be reviewed and approved by the Bulenox management team before use. Approval may be subject to additional requirements or conditions established by Bulenox.
vps_allowed: unknown               # not mentioned on any official page (see §9 G1)
personal_device_required: unknown  # not mentioned on any official page (see §9 G1)
status: partial
not_confirmed_fields:
  - "drawdown.stops_trailing_at (not published for Momentum)"
  - "automation_rule_text (third-party / commercial tools: FAQ 'Only user-built tools ... Commercial ... not allowed' vs Terms of Use 'any third-party algorithm ... must be reviewed and approved' — see §9 C1)"
sources:
  - { field: drawdown, url: "https://bulenox.com/help-center/momentum#mo-parameters", accessed: "2026-10-23" }
  - { field: profit_target_usd, url: "https://bulenox.com/help-center/momentum#mo-parameters", accessed: "2026-10-23" }
  - { field: max_position, url: "https://bulenox.com/help-center/momentum#mo-parameters", accessed: "2026-10-23" }
  - { field: daily_loss_limit, url: "https://bulenox.com/accounts-pricing", accessed: "2026-10-23" }
  - { field: consistency_rule, url: "https://bulenox.com/help-center/momentum#mo-rules", accessed: "2026-10-23" }
  - { field: min_trading_days, url: "https://bulenox.com/help-center/momentum#mo-rules", accessed: "2026-10-23" }
  - { field: trading_day_reset, url: "https://bulenox.com/help-center/qualification#rules", accessed: "2026-10-23" }
  - { field: copy_trading_allowed, url: "https://bulenox.com/faq", accessed: "2026-10-23" }
  - { field: automated_trading_allowed, url: "https://bulenox.com/faq", accessed: "2026-10-23" }
  - { field: automated_trading_allowed, url: "https://bulenox.com/legal/Terms_of_Use.pdf", accessed: "2026-10-23" }
  - { field: platforms, url: "https://bulenox.com/", accessed: "2026-10-23" }
  - { field: platforms, url: "https://bulenox.com/faq", accessed: "2026-10-23" }
  - { field: vps_allowed, url: "not mentioned — searched help center (all 9 categories), FAQ, Terms of Use, Risk Disclosure, homepage, accounts-pricing", accessed: "2026-10-23" }
  - { field: personal_device_required, url: "not mentioned — same search", accessed: "2026-10-23" }
```

```yaml
# Momentum (qualification stage) · Option 1 · 150K
firm: bulenox
plan: "Momentum Account — Option 1 (Trailing Drawdown)"
phase: evaluation
size_label: "150K"
size_usd: "150000.00"
instruments: [MNQ, NQ]
platforms: [Rithmic, "R | Trader Pro", NinjaTrader, Quantower, "Sierra Chart", MultiCharts, ATAS, Bookmap, MotiveWave, "Investor/RT", "Photon Trader", "Trade Navigator", TSLab, ScalpTool, QST, "Medved Trader", OverCharts, QScalp, VolFix, "Inside Edge", "Optimus Flow", Tiger.Trade]
drawdown: { amount_usd: "4500.00", type: trailing_intraday, stops_trailing_at: null }
daily_loss_limit: { amount_usd: null, kind: none }
profit_target_usd: "9000.00"
consistency_rule: { text: "There is no consistency requirement at this stage.", threshold_pct: null }
max_position: { minis: "15", micros: "150", scaling: "none; 1 mini = 10 micros" }
min_trading_days: 0
trading_day_reset: { time: "17:00", tz_as_published: "North American Central Time (CT)" }   # "starts at 5:00 PM and ends at 4:00 PM the following day"; positions closed by 3:59 PM CT
copy_trading_allowed: conditional   # copiers permitted; only user-built tools for the account owner's personal use
automated_trading_allowed: conditional
automation_rule_text: |-
    Yes. Bots, algorithms and trade copiers are permitted. If you connect to Rithmic through a third-party API or composite software, an additional $100 per month applies.
    Only user-built tools intended solely for the account owner’s personal use are permitted. Commercial, shared, rented or publicly distributed automation tools are not allowed.
    These tools are developed and maintained independently by their users, so their uptime, reliability and performance are outside our control, and we cannot provide support or accept responsibility for any issues arising from their use.

    You agree to not abuse the company provided service programs. It includes but is not limited to using “scalping” algorithms, DTC Protocol Bridge API or automated discretional trading in any of the trading environments. Open and close hundreds or thousand of rapid trades by using any trading algorithms. Implementation of reckless transactions in rough markets to benefit from lack of execution. If abuse is suspected, we reserve the right to refuse to claim any profit. To avoid performance discrepancies and/or meet applicable compliance requirements, any third-party algorithm, automated trading software, or other third-party trading technology must be reviewed and approved by the Bulenox management team before use. Approval may be subject to additional requirements or conditions established by Bulenox.
vps_allowed: unknown               # not mentioned on any official page (see §9 G1)
personal_device_required: unknown  # not mentioned on any official page (see §9 G1)
status: partial
not_confirmed_fields:
  - "drawdown.stops_trailing_at (not published for Momentum)"
  - "automation_rule_text (third-party / commercial tools: FAQ 'Only user-built tools ... Commercial ... not allowed' vs Terms of Use 'any third-party algorithm ... must be reviewed and approved' — see §9 C1)"
sources:
  - { field: drawdown, url: "https://bulenox.com/help-center/momentum#mo-parameters", accessed: "2026-10-23" }
  - { field: profit_target_usd, url: "https://bulenox.com/help-center/momentum#mo-parameters", accessed: "2026-10-23" }
  - { field: max_position, url: "https://bulenox.com/help-center/momentum#mo-parameters", accessed: "2026-10-23" }
  - { field: daily_loss_limit, url: "https://bulenox.com/accounts-pricing", accessed: "2026-10-23" }
  - { field: consistency_rule, url: "https://bulenox.com/help-center/momentum#mo-rules", accessed: "2026-10-23" }
  - { field: min_trading_days, url: "https://bulenox.com/help-center/momentum#mo-rules", accessed: "2026-10-23" }
  - { field: trading_day_reset, url: "https://bulenox.com/help-center/qualification#rules", accessed: "2026-10-23" }
  - { field: copy_trading_allowed, url: "https://bulenox.com/faq", accessed: "2026-10-23" }
  - { field: automated_trading_allowed, url: "https://bulenox.com/faq", accessed: "2026-10-23" }
  - { field: automated_trading_allowed, url: "https://bulenox.com/legal/Terms_of_Use.pdf", accessed: "2026-10-23" }
  - { field: platforms, url: "https://bulenox.com/", accessed: "2026-10-23" }
  - { field: platforms, url: "https://bulenox.com/faq", accessed: "2026-10-23" }
  - { field: vps_allowed, url: "not mentioned — searched help center (all 9 categories), FAQ, Terms of Use, Risk Disclosure, homepage, accounts-pricing", accessed: "2026-10-23" }
  - { field: personal_device_required, url: "not mentioned — same search", accessed: "2026-10-23" }
```

```yaml
# Momentum (qualification stage) · Option 2 · 25K
firm: bulenox
plan: "Momentum Account — Option 2 (EOD Drawdown / No scaling)"
phase: evaluation
size_label: "25K"
size_usd: "25000.00"
instruments: [MNQ, NQ]
platforms: [Rithmic, "R | Trader Pro", NinjaTrader, Quantower, "Sierra Chart", MultiCharts, ATAS, Bookmap, MotiveWave, "Investor/RT", "Photon Trader", "Trade Navigator", TSLab, ScalpTool, QST, "Medved Trader", OverCharts, QScalp, VolFix, "Inside Edge", "Optimus Flow", Tiger.Trade]
drawdown: { amount_usd: "1000.00", type: trailing_eod, stops_trailing_at: null }
daily_loss_limit: { amount_usd: "600.00", kind: soft_pause }
profit_target_usd: "1500.00"
consistency_rule: { text: "There is no consistency requirement at this stage.", threshold_pct: null }
max_position: { minis: "2", micros: "20", scaling: "none (Option 2 — no scaling); 1 mini = 10 micros" }
min_trading_days: 0
trading_day_reset: { time: "17:00", tz_as_published: "North American Central Time (CT)" }   # "starts at 5:00 PM and ends at 4:00 PM the following day"; positions closed by 3:59 PM CT
copy_trading_allowed: conditional   # copiers permitted; only user-built tools for the account owner's personal use
automated_trading_allowed: conditional
automation_rule_text: |-
    Yes. Bots, algorithms and trade copiers are permitted. If you connect to Rithmic through a third-party API or composite software, an additional $100 per month applies.
    Only user-built tools intended solely for the account owner’s personal use are permitted. Commercial, shared, rented or publicly distributed automation tools are not allowed.
    These tools are developed and maintained independently by their users, so their uptime, reliability and performance are outside our control, and we cannot provide support or accept responsibility for any issues arising from their use.

    You agree to not abuse the company provided service programs. It includes but is not limited to using “scalping” algorithms, DTC Protocol Bridge API or automated discretional trading in any of the trading environments. Open and close hundreds or thousand of rapid trades by using any trading algorithms. Implementation of reckless transactions in rough markets to benefit from lack of execution. If abuse is suspected, we reserve the right to refuse to claim any profit. To avoid performance discrepancies and/or meet applicable compliance requirements, any third-party algorithm, automated trading software, or other third-party trading technology must be reviewed and approved by the Bulenox management team before use. Approval may be subject to additional requirements or conditions established by Bulenox.
vps_allowed: unknown               # not mentioned on any official page (see §9 G1)
personal_device_required: unknown  # not mentioned on any official page (see §9 G1)
status: partial
not_confirmed_fields:
  - "drawdown.stops_trailing_at (not published for Momentum)"
  - "daily_loss_limit.kind (pause behaviour stated generically for Option 2 on the pricing page, not on the Momentum page)"
  - "automation_rule_text (third-party / commercial tools: FAQ 'Only user-built tools ... Commercial ... not allowed' vs Terms of Use 'any third-party algorithm ... must be reviewed and approved' — see §9 C1)"
sources:
  - { field: drawdown, url: "https://bulenox.com/help-center/momentum#mo-parameters", accessed: "2026-10-23" }
  - { field: profit_target_usd, url: "https://bulenox.com/help-center/momentum#mo-parameters", accessed: "2026-10-23" }
  - { field: max_position, url: "https://bulenox.com/help-center/momentum#mo-parameters", accessed: "2026-10-23" }
  - { field: daily_loss_limit, url: "https://bulenox.com/help-center/momentum#mo-parameters", accessed: "2026-10-23" }
  - { field: daily_loss_limit.kind, url: "https://bulenox.com/accounts-pricing", accessed: "2026-10-23" }
  - { field: consistency_rule, url: "https://bulenox.com/help-center/momentum#mo-rules", accessed: "2026-10-23" }
  - { field: min_trading_days, url: "https://bulenox.com/help-center/momentum#mo-rules", accessed: "2026-10-23" }
  - { field: trading_day_reset, url: "https://bulenox.com/help-center/qualification#rules", accessed: "2026-10-23" }
  - { field: copy_trading_allowed, url: "https://bulenox.com/faq", accessed: "2026-10-23" }
  - { field: automated_trading_allowed, url: "https://bulenox.com/faq", accessed: "2026-10-23" }
  - { field: automated_trading_allowed, url: "https://bulenox.com/legal/Terms_of_Use.pdf", accessed: "2026-10-23" }
  - { field: platforms, url: "https://bulenox.com/", accessed: "2026-10-23" }
  - { field: platforms, url: "https://bulenox.com/faq", accessed: "2026-10-23" }
  - { field: vps_allowed, url: "not mentioned — searched help center (all 9 categories), FAQ, Terms of Use, Risk Disclosure, homepage, accounts-pricing", accessed: "2026-10-23" }
  - { field: personal_device_required, url: "not mentioned — same search", accessed: "2026-10-23" }
```

```yaml
# Momentum (qualification stage) · Option 2 · 50K
firm: bulenox
plan: "Momentum Account — Option 2 (EOD Drawdown / No scaling)"
phase: evaluation
size_label: "50K"
size_usd: "50000.00"
instruments: [MNQ, NQ]
platforms: [Rithmic, "R | Trader Pro", NinjaTrader, Quantower, "Sierra Chart", MultiCharts, ATAS, Bookmap, MotiveWave, "Investor/RT", "Photon Trader", "Trade Navigator", TSLab, ScalpTool, QST, "Medved Trader", OverCharts, QScalp, VolFix, "Inside Edge", "Optimus Flow", Tiger.Trade]
drawdown: { amount_usd: "2000.00", type: trailing_eod, stops_trailing_at: null }
daily_loss_limit: { amount_usd: "1200.00", kind: soft_pause }
profit_target_usd: "3000.00"
consistency_rule: { text: "There is no consistency requirement at this stage.", threshold_pct: null }
max_position: { minis: "4", micros: "40", scaling: "none (Option 2 — no scaling); 1 mini = 10 micros" }
min_trading_days: 0
trading_day_reset: { time: "17:00", tz_as_published: "North American Central Time (CT)" }   # "starts at 5:00 PM and ends at 4:00 PM the following day"; positions closed by 3:59 PM CT
copy_trading_allowed: conditional   # copiers permitted; only user-built tools for the account owner's personal use
automated_trading_allowed: conditional
automation_rule_text: |-
    Yes. Bots, algorithms and trade copiers are permitted. If you connect to Rithmic through a third-party API or composite software, an additional $100 per month applies.
    Only user-built tools intended solely for the account owner’s personal use are permitted. Commercial, shared, rented or publicly distributed automation tools are not allowed.
    These tools are developed and maintained independently by their users, so their uptime, reliability and performance are outside our control, and we cannot provide support or accept responsibility for any issues arising from their use.

    You agree to not abuse the company provided service programs. It includes but is not limited to using “scalping” algorithms, DTC Protocol Bridge API or automated discretional trading in any of the trading environments. Open and close hundreds or thousand of rapid trades by using any trading algorithms. Implementation of reckless transactions in rough markets to benefit from lack of execution. If abuse is suspected, we reserve the right to refuse to claim any profit. To avoid performance discrepancies and/or meet applicable compliance requirements, any third-party algorithm, automated trading software, or other third-party trading technology must be reviewed and approved by the Bulenox management team before use. Approval may be subject to additional requirements or conditions established by Bulenox.
vps_allowed: unknown               # not mentioned on any official page (see §9 G1)
personal_device_required: unknown  # not mentioned on any official page (see §9 G1)
status: partial
not_confirmed_fields:
  - "drawdown.stops_trailing_at (not published for Momentum)"
  - "daily_loss_limit.kind (pause behaviour stated generically for Option 2 on the pricing page, not on the Momentum page)"
  - "automation_rule_text (third-party / commercial tools: FAQ 'Only user-built tools ... Commercial ... not allowed' vs Terms of Use 'any third-party algorithm ... must be reviewed and approved' — see §9 C1)"
sources:
  - { field: drawdown, url: "https://bulenox.com/help-center/momentum#mo-parameters", accessed: "2026-10-23" }
  - { field: profit_target_usd, url: "https://bulenox.com/help-center/momentum#mo-parameters", accessed: "2026-10-23" }
  - { field: max_position, url: "https://bulenox.com/help-center/momentum#mo-parameters", accessed: "2026-10-23" }
  - { field: daily_loss_limit, url: "https://bulenox.com/help-center/momentum#mo-parameters", accessed: "2026-10-23" }
  - { field: daily_loss_limit.kind, url: "https://bulenox.com/accounts-pricing", accessed: "2026-10-23" }
  - { field: consistency_rule, url: "https://bulenox.com/help-center/momentum#mo-rules", accessed: "2026-10-23" }
  - { field: min_trading_days, url: "https://bulenox.com/help-center/momentum#mo-rules", accessed: "2026-10-23" }
  - { field: trading_day_reset, url: "https://bulenox.com/help-center/qualification#rules", accessed: "2026-10-23" }
  - { field: copy_trading_allowed, url: "https://bulenox.com/faq", accessed: "2026-10-23" }
  - { field: automated_trading_allowed, url: "https://bulenox.com/faq", accessed: "2026-10-23" }
  - { field: automated_trading_allowed, url: "https://bulenox.com/legal/Terms_of_Use.pdf", accessed: "2026-10-23" }
  - { field: platforms, url: "https://bulenox.com/", accessed: "2026-10-23" }
  - { field: platforms, url: "https://bulenox.com/faq", accessed: "2026-10-23" }
  - { field: vps_allowed, url: "not mentioned — searched help center (all 9 categories), FAQ, Terms of Use, Risk Disclosure, homepage, accounts-pricing", accessed: "2026-10-23" }
  - { field: personal_device_required, url: "not mentioned — same search", accessed: "2026-10-23" }
```

```yaml
# Momentum (qualification stage) · Option 2 · 100K
firm: bulenox
plan: "Momentum Account — Option 2 (EOD Drawdown / No scaling)"
phase: evaluation
size_label: "100K"
size_usd: "100000.00"
instruments: [MNQ, NQ]
platforms: [Rithmic, "R | Trader Pro", NinjaTrader, Quantower, "Sierra Chart", MultiCharts, ATAS, Bookmap, MotiveWave, "Investor/RT", "Photon Trader", "Trade Navigator", TSLab, ScalpTool, QST, "Medved Trader", OverCharts, QScalp, VolFix, "Inside Edge", "Optimus Flow", Tiger.Trade]
drawdown: { amount_usd: "3000.00", type: trailing_eod, stops_trailing_at: null }
daily_loss_limit: { amount_usd: "2500.00", kind: soft_pause }
profit_target_usd: "6000.00"
consistency_rule: { text: "There is no consistency requirement at this stage.", threshold_pct: null }
max_position: { minis: "8", micros: "80", scaling: "none (Option 2 — no scaling); 1 mini = 10 micros" }
min_trading_days: 0
trading_day_reset: { time: "17:00", tz_as_published: "North American Central Time (CT)" }   # "starts at 5:00 PM and ends at 4:00 PM the following day"; positions closed by 3:59 PM CT
copy_trading_allowed: conditional   # copiers permitted; only user-built tools for the account owner's personal use
automated_trading_allowed: conditional
automation_rule_text: |-
    Yes. Bots, algorithms and trade copiers are permitted. If you connect to Rithmic through a third-party API or composite software, an additional $100 per month applies.
    Only user-built tools intended solely for the account owner’s personal use are permitted. Commercial, shared, rented or publicly distributed automation tools are not allowed.
    These tools are developed and maintained independently by their users, so their uptime, reliability and performance are outside our control, and we cannot provide support or accept responsibility for any issues arising from their use.

    You agree to not abuse the company provided service programs. It includes but is not limited to using “scalping” algorithms, DTC Protocol Bridge API or automated discretional trading in any of the trading environments. Open and close hundreds or thousand of rapid trades by using any trading algorithms. Implementation of reckless transactions in rough markets to benefit from lack of execution. If abuse is suspected, we reserve the right to refuse to claim any profit. To avoid performance discrepancies and/or meet applicable compliance requirements, any third-party algorithm, automated trading software, or other third-party trading technology must be reviewed and approved by the Bulenox management team before use. Approval may be subject to additional requirements or conditions established by Bulenox.
vps_allowed: unknown               # not mentioned on any official page (see §9 G1)
personal_device_required: unknown  # not mentioned on any official page (see §9 G1)
status: partial
not_confirmed_fields:
  - "drawdown.stops_trailing_at (not published for Momentum)"
  - "daily_loss_limit.kind (pause behaviour stated generically for Option 2 on the pricing page, not on the Momentum page)"
  - "automation_rule_text (third-party / commercial tools: FAQ 'Only user-built tools ... Commercial ... not allowed' vs Terms of Use 'any third-party algorithm ... must be reviewed and approved' — see §9 C1)"
sources:
  - { field: drawdown, url: "https://bulenox.com/help-center/momentum#mo-parameters", accessed: "2026-10-23" }
  - { field: profit_target_usd, url: "https://bulenox.com/help-center/momentum#mo-parameters", accessed: "2026-10-23" }
  - { field: max_position, url: "https://bulenox.com/help-center/momentum#mo-parameters", accessed: "2026-10-23" }
  - { field: daily_loss_limit, url: "https://bulenox.com/help-center/momentum#mo-parameters", accessed: "2026-10-23" }
  - { field: daily_loss_limit.kind, url: "https://bulenox.com/accounts-pricing", accessed: "2026-10-23" }
  - { field: consistency_rule, url: "https://bulenox.com/help-center/momentum#mo-rules", accessed: "2026-10-23" }
  - { field: min_trading_days, url: "https://bulenox.com/help-center/momentum#mo-rules", accessed: "2026-10-23" }
  - { field: trading_day_reset, url: "https://bulenox.com/help-center/qualification#rules", accessed: "2026-10-23" }
  - { field: copy_trading_allowed, url: "https://bulenox.com/faq", accessed: "2026-10-23" }
  - { field: automated_trading_allowed, url: "https://bulenox.com/faq", accessed: "2026-10-23" }
  - { field: automated_trading_allowed, url: "https://bulenox.com/legal/Terms_of_Use.pdf", accessed: "2026-10-23" }
  - { field: platforms, url: "https://bulenox.com/", accessed: "2026-10-23" }
  - { field: platforms, url: "https://bulenox.com/faq", accessed: "2026-10-23" }
  - { field: vps_allowed, url: "not mentioned — searched help center (all 9 categories), FAQ, Terms of Use, Risk Disclosure, homepage, accounts-pricing", accessed: "2026-10-23" }
  - { field: personal_device_required, url: "not mentioned — same search", accessed: "2026-10-23" }
```

```yaml
# Momentum (qualification stage) · Option 2 · 150K
firm: bulenox
plan: "Momentum Account — Option 2 (EOD Drawdown / No scaling)"
phase: evaluation
size_label: "150K"
size_usd: "150000.00"
instruments: [MNQ, NQ]
platforms: [Rithmic, "R | Trader Pro", NinjaTrader, Quantower, "Sierra Chart", MultiCharts, ATAS, Bookmap, MotiveWave, "Investor/RT", "Photon Trader", "Trade Navigator", TSLab, ScalpTool, QST, "Medved Trader", OverCharts, QScalp, VolFix, "Inside Edge", "Optimus Flow", Tiger.Trade]
drawdown: { amount_usd: "4500.00", type: trailing_eod, stops_trailing_at: null }
daily_loss_limit: { amount_usd: "3300.00", kind: soft_pause }
profit_target_usd: "9000.00"
consistency_rule: { text: "There is no consistency requirement at this stage.", threshold_pct: null }
max_position: { minis: "12", micros: "120", scaling: "none (Option 2 — no scaling); 1 mini = 10 micros" }
min_trading_days: 0
trading_day_reset: { time: "17:00", tz_as_published: "North American Central Time (CT)" }   # "starts at 5:00 PM and ends at 4:00 PM the following day"; positions closed by 3:59 PM CT
copy_trading_allowed: conditional   # copiers permitted; only user-built tools for the account owner's personal use
automated_trading_allowed: conditional
automation_rule_text: |-
    Yes. Bots, algorithms and trade copiers are permitted. If you connect to Rithmic through a third-party API or composite software, an additional $100 per month applies.
    Only user-built tools intended solely for the account owner’s personal use are permitted. Commercial, shared, rented or publicly distributed automation tools are not allowed.
    These tools are developed and maintained independently by their users, so their uptime, reliability and performance are outside our control, and we cannot provide support or accept responsibility for any issues arising from their use.

    You agree to not abuse the company provided service programs. It includes but is not limited to using “scalping” algorithms, DTC Protocol Bridge API or automated discretional trading in any of the trading environments. Open and close hundreds or thousand of rapid trades by using any trading algorithms. Implementation of reckless transactions in rough markets to benefit from lack of execution. If abuse is suspected, we reserve the right to refuse to claim any profit. To avoid performance discrepancies and/or meet applicable compliance requirements, any third-party algorithm, automated trading software, or other third-party trading technology must be reviewed and approved by the Bulenox management team before use. Approval may be subject to additional requirements or conditions established by Bulenox.
vps_allowed: unknown               # not mentioned on any official page (see §9 G1)
personal_device_required: unknown  # not mentioned on any official page (see §9 G1)
status: partial
not_confirmed_fields:
  - "drawdown.stops_trailing_at (not published for Momentum)"
  - "daily_loss_limit.kind (pause behaviour stated generically for Option 2 on the pricing page, not on the Momentum page)"
  - "automation_rule_text (third-party / commercial tools: FAQ 'Only user-built tools ... Commercial ... not allowed' vs Terms of Use 'any third-party algorithm ... must be reviewed and approved' — see §9 C1)"
sources:
  - { field: drawdown, url: "https://bulenox.com/help-center/momentum#mo-parameters", accessed: "2026-10-23" }
  - { field: profit_target_usd, url: "https://bulenox.com/help-center/momentum#mo-parameters", accessed: "2026-10-23" }
  - { field: max_position, url: "https://bulenox.com/help-center/momentum#mo-parameters", accessed: "2026-10-23" }
  - { field: daily_loss_limit, url: "https://bulenox.com/help-center/momentum#mo-parameters", accessed: "2026-10-23" }
  - { field: daily_loss_limit.kind, url: "https://bulenox.com/accounts-pricing", accessed: "2026-10-23" }
  - { field: consistency_rule, url: "https://bulenox.com/help-center/momentum#mo-rules", accessed: "2026-10-23" }
  - { field: min_trading_days, url: "https://bulenox.com/help-center/momentum#mo-rules", accessed: "2026-10-23" }
  - { field: trading_day_reset, url: "https://bulenox.com/help-center/qualification#rules", accessed: "2026-10-23" }
  - { field: copy_trading_allowed, url: "https://bulenox.com/faq", accessed: "2026-10-23" }
  - { field: automated_trading_allowed, url: "https://bulenox.com/faq", accessed: "2026-10-23" }
  - { field: automated_trading_allowed, url: "https://bulenox.com/legal/Terms_of_Use.pdf", accessed: "2026-10-23" }
  - { field: platforms, url: "https://bulenox.com/", accessed: "2026-10-23" }
  - { field: platforms, url: "https://bulenox.com/faq", accessed: "2026-10-23" }
  - { field: vps_allowed, url: "not mentioned — searched help center (all 9 categories), FAQ, Terms of Use, Risk Disclosure, homepage, accounts-pricing", accessed: "2026-10-23" }
  - { field: personal_device_required, url: "not mentioned — same search", accessed: "2026-10-23" }
```

```yaml
# Momentum Master · Option 1 · 25K
firm: bulenox
plan: "Momentum Master — Option 1 (Trailing Drawdown)"
phase: funded
size_label: "25K"
size_usd: "25000.00"
instruments: [MNQ, NQ]
platforms: [Rithmic, "R | Trader Pro", NinjaTrader, Quantower, "Sierra Chart", MultiCharts, ATAS, Bookmap, MotiveWave, "Investor/RT", "Photon Trader", "Trade Navigator", TSLab, ScalpTool, QST, "Medved Trader", OverCharts, QScalp, VolFix, "Inside Edge", "Optimus Flow", Tiger.Trade]
drawdown: { amount_usd: "1000.00", type: trailing_intraday, stops_trailing_at: null }
daily_loss_limit: { amount_usd: null, kind: none }
profit_target_usd: null
consistency_rule: { text: "Your best trading day may not exceed 35% of the net profit of the payout cycle. Missing it is not a breach", threshold_pct: "35" }
max_position: { minis: "3", micros: "30", scaling: "none; 1 mini = 10 micros" }
min_trading_days: null
trading_day_reset: { time: "17:00", tz_as_published: "North American Central Time (CT)" }   # "starts at 5:00 PM and ends at 4:00 PM the following day"; positions closed by 3:59 PM CT
copy_trading_allowed: conditional   # copiers permitted; only user-built tools for the account owner's personal use
automated_trading_allowed: conditional
automation_rule_text: |-
    Yes. Bots, algorithms and trade copiers are permitted. If you connect to Rithmic through a third-party API or composite software, an additional $100 per month applies.
    Only user-built tools intended solely for the account owner’s personal use are permitted. Commercial, shared, rented or publicly distributed automation tools are not allowed.
    These tools are developed and maintained independently by their users, so their uptime, reliability and performance are outside our control, and we cannot provide support or accept responsibility for any issues arising from their use.

    You agree to not abuse the company provided service programs. It includes but is not limited to using “scalping” algorithms, DTC Protocol Bridge API or automated discretional trading in any of the trading environments. Open and close hundreds or thousand of rapid trades by using any trading algorithms. Implementation of reckless transactions in rough markets to benefit from lack of execution. If abuse is suspected, we reserve the right to refuse to claim any profit. To avoid performance discrepancies and/or meet applicable compliance requirements, any third-party algorithm, automated trading software, or other third-party trading technology must be reviewed and approved by the Bulenox management team before use. Approval may be subject to additional requirements or conditions established by Bulenox.
vps_allowed: unknown               # not mentioned on any official page (see §9 G1)
personal_device_required: unknown  # not mentioned on any official page (see §9 G1)
payout_rules: "5 qualifying profitable days (>= $100 net) per cycle; min balance for payout $26,500; Safety Net Reserve $1,000"
status: partial
not_confirmed_fields:
  - "drawdown.amount_usd (the Momentum 'Account Parameters' table is not labelled by stage)"
  - "drawdown.stops_trailing_at (not published for Momentum Master)"
  - "daily_loss_limit (not published separately for Momentum Master)"
  - "max_position (not published separately for Momentum Master)"
  - "automation_rule_text (third-party / commercial tools: FAQ 'Only user-built tools ... Commercial ... not allowed' vs Terms of Use 'any third-party algorithm ... must be reviewed and approved' — see §9 C1)"
sources:
  - { field: drawdown, url: "https://bulenox.com/help-center/momentum#mo-parameters", accessed: "2026-10-23" }
  - { field: consistency_rule, url: "https://bulenox.com/help-center/momentum#mo-consistency", accessed: "2026-10-23" }
  - { field: max_position, url: "https://bulenox.com/help-center/momentum#mo-parameters", accessed: "2026-10-23" }
  - { field: trading_day_reset, url: "https://bulenox.com/help-center/qualification#rules", accessed: "2026-10-23" }
  - { field: copy_trading_allowed, url: "https://bulenox.com/faq", accessed: "2026-10-23" }
  - { field: automated_trading_allowed, url: "https://bulenox.com/faq", accessed: "2026-10-23" }
  - { field: automated_trading_allowed, url: "https://bulenox.com/legal/Terms_of_Use.pdf", accessed: "2026-10-23" }
  - { field: platforms, url: "https://bulenox.com/", accessed: "2026-10-23" }
  - { field: platforms, url: "https://bulenox.com/faq", accessed: "2026-10-23" }
  - { field: vps_allowed, url: "not mentioned — searched help center (all 9 categories), FAQ, Terms of Use, Risk Disclosure, homepage, accounts-pricing", accessed: "2026-10-23" }
  - { field: personal_device_required, url: "not mentioned — same search", accessed: "2026-10-23" }
```

```yaml
# Momentum Master · Option 1 · 50K
firm: bulenox
plan: "Momentum Master — Option 1 (Trailing Drawdown)"
phase: funded
size_label: "50K"
size_usd: "50000.00"
instruments: [MNQ, NQ]
platforms: [Rithmic, "R | Trader Pro", NinjaTrader, Quantower, "Sierra Chart", MultiCharts, ATAS, Bookmap, MotiveWave, "Investor/RT", "Photon Trader", "Trade Navigator", TSLab, ScalpTool, QST, "Medved Trader", OverCharts, QScalp, VolFix, "Inside Edge", "Optimus Flow", Tiger.Trade]
drawdown: { amount_usd: "2000.00", type: trailing_intraday, stops_trailing_at: null }
daily_loss_limit: { amount_usd: null, kind: none }
profit_target_usd: null
consistency_rule: { text: "Your best trading day may not exceed 35% of the net profit of the payout cycle. Missing it is not a breach", threshold_pct: "35" }
max_position: { minis: "7", micros: "70", scaling: "none; 1 mini = 10 micros" }
min_trading_days: null
trading_day_reset: { time: "17:00", tz_as_published: "North American Central Time (CT)" }   # "starts at 5:00 PM and ends at 4:00 PM the following day"; positions closed by 3:59 PM CT
copy_trading_allowed: conditional   # copiers permitted; only user-built tools for the account owner's personal use
automated_trading_allowed: conditional
automation_rule_text: |-
    Yes. Bots, algorithms and trade copiers are permitted. If you connect to Rithmic through a third-party API or composite software, an additional $100 per month applies.
    Only user-built tools intended solely for the account owner’s personal use are permitted. Commercial, shared, rented or publicly distributed automation tools are not allowed.
    These tools are developed and maintained independently by their users, so their uptime, reliability and performance are outside our control, and we cannot provide support or accept responsibility for any issues arising from their use.

    You agree to not abuse the company provided service programs. It includes but is not limited to using “scalping” algorithms, DTC Protocol Bridge API or automated discretional trading in any of the trading environments. Open and close hundreds or thousand of rapid trades by using any trading algorithms. Implementation of reckless transactions in rough markets to benefit from lack of execution. If abuse is suspected, we reserve the right to refuse to claim any profit. To avoid performance discrepancies and/or meet applicable compliance requirements, any third-party algorithm, automated trading software, or other third-party trading technology must be reviewed and approved by the Bulenox management team before use. Approval may be subject to additional requirements or conditions established by Bulenox.
vps_allowed: unknown               # not mentioned on any official page (see §9 G1)
personal_device_required: unknown  # not mentioned on any official page (see §9 G1)
payout_rules: "5 qualifying profitable days (>= $150 net) per cycle; min balance for payout $53,000; Safety Net Reserve $2,000"
status: partial
not_confirmed_fields:
  - "drawdown.amount_usd (the Momentum 'Account Parameters' table is not labelled by stage)"
  - "drawdown.stops_trailing_at (not published for Momentum Master)"
  - "daily_loss_limit (not published separately for Momentum Master)"
  - "max_position (not published separately for Momentum Master)"
  - "automation_rule_text (third-party / commercial tools: FAQ 'Only user-built tools ... Commercial ... not allowed' vs Terms of Use 'any third-party algorithm ... must be reviewed and approved' — see §9 C1)"
sources:
  - { field: drawdown, url: "https://bulenox.com/help-center/momentum#mo-parameters", accessed: "2026-10-23" }
  - { field: consistency_rule, url: "https://bulenox.com/help-center/momentum#mo-consistency", accessed: "2026-10-23" }
  - { field: max_position, url: "https://bulenox.com/help-center/momentum#mo-parameters", accessed: "2026-10-23" }
  - { field: trading_day_reset, url: "https://bulenox.com/help-center/qualification#rules", accessed: "2026-10-23" }
  - { field: copy_trading_allowed, url: "https://bulenox.com/faq", accessed: "2026-10-23" }
  - { field: automated_trading_allowed, url: "https://bulenox.com/faq", accessed: "2026-10-23" }
  - { field: automated_trading_allowed, url: "https://bulenox.com/legal/Terms_of_Use.pdf", accessed: "2026-10-23" }
  - { field: platforms, url: "https://bulenox.com/", accessed: "2026-10-23" }
  - { field: platforms, url: "https://bulenox.com/faq", accessed: "2026-10-23" }
  - { field: vps_allowed, url: "not mentioned — searched help center (all 9 categories), FAQ, Terms of Use, Risk Disclosure, homepage, accounts-pricing", accessed: "2026-10-23" }
  - { field: personal_device_required, url: "not mentioned — same search", accessed: "2026-10-23" }
```

```yaml
# Momentum Master · Option 1 · 100K
firm: bulenox
plan: "Momentum Master — Option 1 (Trailing Drawdown)"
phase: funded
size_label: "100K"
size_usd: "100000.00"
instruments: [MNQ, NQ]
platforms: [Rithmic, "R | Trader Pro", NinjaTrader, Quantower, "Sierra Chart", MultiCharts, ATAS, Bookmap, MotiveWave, "Investor/RT", "Photon Trader", "Trade Navigator", TSLab, ScalpTool, QST, "Medved Trader", OverCharts, QScalp, VolFix, "Inside Edge", "Optimus Flow", Tiger.Trade]
drawdown: { amount_usd: "3000.00", type: trailing_intraday, stops_trailing_at: null }
daily_loss_limit: { amount_usd: null, kind: none }
profit_target_usd: null
consistency_rule: { text: "Your best trading day may not exceed 35% of the net profit of the payout cycle. Missing it is not a breach", threshold_pct: "35" }
max_position: { minis: "12", micros: "120", scaling: "none; 1 mini = 10 micros" }
min_trading_days: null
trading_day_reset: { time: "17:00", tz_as_published: "North American Central Time (CT)" }   # "starts at 5:00 PM and ends at 4:00 PM the following day"; positions closed by 3:59 PM CT
copy_trading_allowed: conditional   # copiers permitted; only user-built tools for the account owner's personal use
automated_trading_allowed: conditional
automation_rule_text: |-
    Yes. Bots, algorithms and trade copiers are permitted. If you connect to Rithmic through a third-party API or composite software, an additional $100 per month applies.
    Only user-built tools intended solely for the account owner’s personal use are permitted. Commercial, shared, rented or publicly distributed automation tools are not allowed.
    These tools are developed and maintained independently by their users, so their uptime, reliability and performance are outside our control, and we cannot provide support or accept responsibility for any issues arising from their use.

    You agree to not abuse the company provided service programs. It includes but is not limited to using “scalping” algorithms, DTC Protocol Bridge API or automated discretional trading in any of the trading environments. Open and close hundreds or thousand of rapid trades by using any trading algorithms. Implementation of reckless transactions in rough markets to benefit from lack of execution. If abuse is suspected, we reserve the right to refuse to claim any profit. To avoid performance discrepancies and/or meet applicable compliance requirements, any third-party algorithm, automated trading software, or other third-party trading technology must be reviewed and approved by the Bulenox management team before use. Approval may be subject to additional requirements or conditions established by Bulenox.
vps_allowed: unknown               # not mentioned on any official page (see §9 G1)
personal_device_required: unknown  # not mentioned on any official page (see §9 G1)
payout_rules: "5 qualifying profitable days (>= $200 net) per cycle; min balance for payout $104,500; Safety Net Reserve $3,000"
status: partial
not_confirmed_fields:
  - "drawdown.amount_usd (the Momentum 'Account Parameters' table is not labelled by stage)"
  - "drawdown.stops_trailing_at (not published for Momentum Master)"
  - "daily_loss_limit (not published separately for Momentum Master)"
  - "max_position (not published separately for Momentum Master)"
  - "automation_rule_text (third-party / commercial tools: FAQ 'Only user-built tools ... Commercial ... not allowed' vs Terms of Use 'any third-party algorithm ... must be reviewed and approved' — see §9 C1)"
sources:
  - { field: drawdown, url: "https://bulenox.com/help-center/momentum#mo-parameters", accessed: "2026-10-23" }
  - { field: consistency_rule, url: "https://bulenox.com/help-center/momentum#mo-consistency", accessed: "2026-10-23" }
  - { field: max_position, url: "https://bulenox.com/help-center/momentum#mo-parameters", accessed: "2026-10-23" }
  - { field: trading_day_reset, url: "https://bulenox.com/help-center/qualification#rules", accessed: "2026-10-23" }
  - { field: copy_trading_allowed, url: "https://bulenox.com/faq", accessed: "2026-10-23" }
  - { field: automated_trading_allowed, url: "https://bulenox.com/faq", accessed: "2026-10-23" }
  - { field: automated_trading_allowed, url: "https://bulenox.com/legal/Terms_of_Use.pdf", accessed: "2026-10-23" }
  - { field: platforms, url: "https://bulenox.com/", accessed: "2026-10-23" }
  - { field: platforms, url: "https://bulenox.com/faq", accessed: "2026-10-23" }
  - { field: vps_allowed, url: "not mentioned — searched help center (all 9 categories), FAQ, Terms of Use, Risk Disclosure, homepage, accounts-pricing", accessed: "2026-10-23" }
  - { field: personal_device_required, url: "not mentioned — same search", accessed: "2026-10-23" }
```

```yaml
# Momentum Master · Option 1 · 150K
firm: bulenox
plan: "Momentum Master — Option 1 (Trailing Drawdown)"
phase: funded
size_label: "150K"
size_usd: "150000.00"
instruments: [MNQ, NQ]
platforms: [Rithmic, "R | Trader Pro", NinjaTrader, Quantower, "Sierra Chart", MultiCharts, ATAS, Bookmap, MotiveWave, "Investor/RT", "Photon Trader", "Trade Navigator", TSLab, ScalpTool, QST, "Medved Trader", OverCharts, QScalp, VolFix, "Inside Edge", "Optimus Flow", Tiger.Trade]
drawdown: { amount_usd: "4500.00", type: trailing_intraday, stops_trailing_at: null }
daily_loss_limit: { amount_usd: null, kind: none }
profit_target_usd: null
consistency_rule: { text: "Your best trading day may not exceed 35% of the net profit of the payout cycle. Missing it is not a breach", threshold_pct: "35" }
max_position: { minis: "15", micros: "150", scaling: "none; 1 mini = 10 micros" }
min_trading_days: null
trading_day_reset: { time: "17:00", tz_as_published: "North American Central Time (CT)" }   # "starts at 5:00 PM and ends at 4:00 PM the following day"; positions closed by 3:59 PM CT
copy_trading_allowed: conditional   # copiers permitted; only user-built tools for the account owner's personal use
automated_trading_allowed: conditional
automation_rule_text: |-
    Yes. Bots, algorithms and trade copiers are permitted. If you connect to Rithmic through a third-party API or composite software, an additional $100 per month applies.
    Only user-built tools intended solely for the account owner’s personal use are permitted. Commercial, shared, rented or publicly distributed automation tools are not allowed.
    These tools are developed and maintained independently by their users, so their uptime, reliability and performance are outside our control, and we cannot provide support or accept responsibility for any issues arising from their use.

    You agree to not abuse the company provided service programs. It includes but is not limited to using “scalping” algorithms, DTC Protocol Bridge API or automated discretional trading in any of the trading environments. Open and close hundreds or thousand of rapid trades by using any trading algorithms. Implementation of reckless transactions in rough markets to benefit from lack of execution. If abuse is suspected, we reserve the right to refuse to claim any profit. To avoid performance discrepancies and/or meet applicable compliance requirements, any third-party algorithm, automated trading software, or other third-party trading technology must be reviewed and approved by the Bulenox management team before use. Approval may be subject to additional requirements or conditions established by Bulenox.
vps_allowed: unknown               # not mentioned on any official page (see §9 G1)
personal_device_required: unknown  # not mentioned on any official page (see §9 G1)
payout_rules: "5 qualifying profitable days (>= $250 net) per cycle; min balance for payout $156,500; Safety Net Reserve $4,500"
status: partial
not_confirmed_fields:
  - "drawdown.amount_usd (the Momentum 'Account Parameters' table is not labelled by stage)"
  - "drawdown.stops_trailing_at (not published for Momentum Master)"
  - "daily_loss_limit (not published separately for Momentum Master)"
  - "max_position (not published separately for Momentum Master)"
  - "automation_rule_text (third-party / commercial tools: FAQ 'Only user-built tools ... Commercial ... not allowed' vs Terms of Use 'any third-party algorithm ... must be reviewed and approved' — see §9 C1)"
sources:
  - { field: drawdown, url: "https://bulenox.com/help-center/momentum#mo-parameters", accessed: "2026-10-23" }
  - { field: consistency_rule, url: "https://bulenox.com/help-center/momentum#mo-consistency", accessed: "2026-10-23" }
  - { field: max_position, url: "https://bulenox.com/help-center/momentum#mo-parameters", accessed: "2026-10-23" }
  - { field: trading_day_reset, url: "https://bulenox.com/help-center/qualification#rules", accessed: "2026-10-23" }
  - { field: copy_trading_allowed, url: "https://bulenox.com/faq", accessed: "2026-10-23" }
  - { field: automated_trading_allowed, url: "https://bulenox.com/faq", accessed: "2026-10-23" }
  - { field: automated_trading_allowed, url: "https://bulenox.com/legal/Terms_of_Use.pdf", accessed: "2026-10-23" }
  - { field: platforms, url: "https://bulenox.com/", accessed: "2026-10-23" }
  - { field: platforms, url: "https://bulenox.com/faq", accessed: "2026-10-23" }
  - { field: vps_allowed, url: "not mentioned — searched help center (all 9 categories), FAQ, Terms of Use, Risk Disclosure, homepage, accounts-pricing", accessed: "2026-10-23" }
  - { field: personal_device_required, url: "not mentioned — same search", accessed: "2026-10-23" }
```

```yaml
# Momentum Master · Option 2 · 25K
firm: bulenox
plan: "Momentum Master — Option 2 (EOD Drawdown / No scaling)"
phase: funded
size_label: "25K"
size_usd: "25000.00"
instruments: [MNQ, NQ]
platforms: [Rithmic, "R | Trader Pro", NinjaTrader, Quantower, "Sierra Chart", MultiCharts, ATAS, Bookmap, MotiveWave, "Investor/RT", "Photon Trader", "Trade Navigator", TSLab, ScalpTool, QST, "Medved Trader", OverCharts, QScalp, VolFix, "Inside Edge", "Optimus Flow", Tiger.Trade]
drawdown: { amount_usd: "1000.00", type: trailing_eod, stops_trailing_at: null }
daily_loss_limit: { amount_usd: "600.00", kind: soft_pause }
profit_target_usd: null
consistency_rule: { text: "Your best trading day may not exceed 35% of the net profit of the payout cycle. Missing it is not a breach", threshold_pct: "35" }
max_position: { minis: "2", micros: "20", scaling: "none; 1 mini = 10 micros" }
min_trading_days: null
trading_day_reset: { time: "17:00", tz_as_published: "North American Central Time (CT)" }   # "starts at 5:00 PM and ends at 4:00 PM the following day"; positions closed by 3:59 PM CT
copy_trading_allowed: conditional   # copiers permitted; only user-built tools for the account owner's personal use
automated_trading_allowed: conditional
automation_rule_text: |-
    Yes. Bots, algorithms and trade copiers are permitted. If you connect to Rithmic through a third-party API or composite software, an additional $100 per month applies.
    Only user-built tools intended solely for the account owner’s personal use are permitted. Commercial, shared, rented or publicly distributed automation tools are not allowed.
    These tools are developed and maintained independently by their users, so their uptime, reliability and performance are outside our control, and we cannot provide support or accept responsibility for any issues arising from their use.

    You agree to not abuse the company provided service programs. It includes but is not limited to using “scalping” algorithms, DTC Protocol Bridge API or automated discretional trading in any of the trading environments. Open and close hundreds or thousand of rapid trades by using any trading algorithms. Implementation of reckless transactions in rough markets to benefit from lack of execution. If abuse is suspected, we reserve the right to refuse to claim any profit. To avoid performance discrepancies and/or meet applicable compliance requirements, any third-party algorithm, automated trading software, or other third-party trading technology must be reviewed and approved by the Bulenox management team before use. Approval may be subject to additional requirements or conditions established by Bulenox.
vps_allowed: unknown               # not mentioned on any official page (see §9 G1)
personal_device_required: unknown  # not mentioned on any official page (see §9 G1)
payout_rules: "5 qualifying profitable days (>= $100 net) per cycle; min balance for payout $26,500; Safety Net Reserve $1,000"
status: partial
not_confirmed_fields:
  - "drawdown.amount_usd (the Momentum 'Account Parameters' table is not labelled by stage)"
  - "drawdown.stops_trailing_at (not published for Momentum Master)"
  - "daily_loss_limit (not published separately for Momentum Master)"
  - "max_position (not published separately for Momentum Master)"
  - "automation_rule_text (third-party / commercial tools: FAQ 'Only user-built tools ... Commercial ... not allowed' vs Terms of Use 'any third-party algorithm ... must be reviewed and approved' — see §9 C1)"
sources:
  - { field: drawdown, url: "https://bulenox.com/help-center/momentum#mo-parameters", accessed: "2026-10-23" }
  - { field: consistency_rule, url: "https://bulenox.com/help-center/momentum#mo-consistency", accessed: "2026-10-23" }
  - { field: max_position, url: "https://bulenox.com/help-center/momentum#mo-parameters", accessed: "2026-10-23" }
  - { field: trading_day_reset, url: "https://bulenox.com/help-center/qualification#rules", accessed: "2026-10-23" }
  - { field: copy_trading_allowed, url: "https://bulenox.com/faq", accessed: "2026-10-23" }
  - { field: automated_trading_allowed, url: "https://bulenox.com/faq", accessed: "2026-10-23" }
  - { field: automated_trading_allowed, url: "https://bulenox.com/legal/Terms_of_Use.pdf", accessed: "2026-10-23" }
  - { field: platforms, url: "https://bulenox.com/", accessed: "2026-10-23" }
  - { field: platforms, url: "https://bulenox.com/faq", accessed: "2026-10-23" }
  - { field: vps_allowed, url: "not mentioned — searched help center (all 9 categories), FAQ, Terms of Use, Risk Disclosure, homepage, accounts-pricing", accessed: "2026-10-23" }
  - { field: personal_device_required, url: "not mentioned — same search", accessed: "2026-10-23" }
```

```yaml
# Momentum Master · Option 2 · 50K
firm: bulenox
plan: "Momentum Master — Option 2 (EOD Drawdown / No scaling)"
phase: funded
size_label: "50K"
size_usd: "50000.00"
instruments: [MNQ, NQ]
platforms: [Rithmic, "R | Trader Pro", NinjaTrader, Quantower, "Sierra Chart", MultiCharts, ATAS, Bookmap, MotiveWave, "Investor/RT", "Photon Trader", "Trade Navigator", TSLab, ScalpTool, QST, "Medved Trader", OverCharts, QScalp, VolFix, "Inside Edge", "Optimus Flow", Tiger.Trade]
drawdown: { amount_usd: "2000.00", type: trailing_eod, stops_trailing_at: null }
daily_loss_limit: { amount_usd: "1200.00", kind: soft_pause }
profit_target_usd: null
consistency_rule: { text: "Your best trading day may not exceed 35% of the net profit of the payout cycle. Missing it is not a breach", threshold_pct: "35" }
max_position: { minis: "4", micros: "40", scaling: "none; 1 mini = 10 micros" }
min_trading_days: null
trading_day_reset: { time: "17:00", tz_as_published: "North American Central Time (CT)" }   # "starts at 5:00 PM and ends at 4:00 PM the following day"; positions closed by 3:59 PM CT
copy_trading_allowed: conditional   # copiers permitted; only user-built tools for the account owner's personal use
automated_trading_allowed: conditional
automation_rule_text: |-
    Yes. Bots, algorithms and trade copiers are permitted. If you connect to Rithmic through a third-party API or composite software, an additional $100 per month applies.
    Only user-built tools intended solely for the account owner’s personal use are permitted. Commercial, shared, rented or publicly distributed automation tools are not allowed.
    These tools are developed and maintained independently by their users, so their uptime, reliability and performance are outside our control, and we cannot provide support or accept responsibility for any issues arising from their use.

    You agree to not abuse the company provided service programs. It includes but is not limited to using “scalping” algorithms, DTC Protocol Bridge API or automated discretional trading in any of the trading environments. Open and close hundreds or thousand of rapid trades by using any trading algorithms. Implementation of reckless transactions in rough markets to benefit from lack of execution. If abuse is suspected, we reserve the right to refuse to claim any profit. To avoid performance discrepancies and/or meet applicable compliance requirements, any third-party algorithm, automated trading software, or other third-party trading technology must be reviewed and approved by the Bulenox management team before use. Approval may be subject to additional requirements or conditions established by Bulenox.
vps_allowed: unknown               # not mentioned on any official page (see §9 G1)
personal_device_required: unknown  # not mentioned on any official page (see §9 G1)
payout_rules: "5 qualifying profitable days (>= $150 net) per cycle; min balance for payout $53,000; Safety Net Reserve $2,000"
status: partial
not_confirmed_fields:
  - "drawdown.amount_usd (the Momentum 'Account Parameters' table is not labelled by stage)"
  - "drawdown.stops_trailing_at (not published for Momentum Master)"
  - "daily_loss_limit (not published separately for Momentum Master)"
  - "max_position (not published separately for Momentum Master)"
  - "automation_rule_text (third-party / commercial tools: FAQ 'Only user-built tools ... Commercial ... not allowed' vs Terms of Use 'any third-party algorithm ... must be reviewed and approved' — see §9 C1)"
sources:
  - { field: drawdown, url: "https://bulenox.com/help-center/momentum#mo-parameters", accessed: "2026-10-23" }
  - { field: consistency_rule, url: "https://bulenox.com/help-center/momentum#mo-consistency", accessed: "2026-10-23" }
  - { field: max_position, url: "https://bulenox.com/help-center/momentum#mo-parameters", accessed: "2026-10-23" }
  - { field: trading_day_reset, url: "https://bulenox.com/help-center/qualification#rules", accessed: "2026-10-23" }
  - { field: copy_trading_allowed, url: "https://bulenox.com/faq", accessed: "2026-10-23" }
  - { field: automated_trading_allowed, url: "https://bulenox.com/faq", accessed: "2026-10-23" }
  - { field: automated_trading_allowed, url: "https://bulenox.com/legal/Terms_of_Use.pdf", accessed: "2026-10-23" }
  - { field: platforms, url: "https://bulenox.com/", accessed: "2026-10-23" }
  - { field: platforms, url: "https://bulenox.com/faq", accessed: "2026-10-23" }
  - { field: vps_allowed, url: "not mentioned — searched help center (all 9 categories), FAQ, Terms of Use, Risk Disclosure, homepage, accounts-pricing", accessed: "2026-10-23" }
  - { field: personal_device_required, url: "not mentioned — same search", accessed: "2026-10-23" }
```

```yaml
# Momentum Master · Option 2 · 100K
firm: bulenox
plan: "Momentum Master — Option 2 (EOD Drawdown / No scaling)"
phase: funded
size_label: "100K"
size_usd: "100000.00"
instruments: [MNQ, NQ]
platforms: [Rithmic, "R | Trader Pro", NinjaTrader, Quantower, "Sierra Chart", MultiCharts, ATAS, Bookmap, MotiveWave, "Investor/RT", "Photon Trader", "Trade Navigator", TSLab, ScalpTool, QST, "Medved Trader", OverCharts, QScalp, VolFix, "Inside Edge", "Optimus Flow", Tiger.Trade]
drawdown: { amount_usd: "3000.00", type: trailing_eod, stops_trailing_at: null }
daily_loss_limit: { amount_usd: "2500.00", kind: soft_pause }
profit_target_usd: null
consistency_rule: { text: "Your best trading day may not exceed 35% of the net profit of the payout cycle. Missing it is not a breach", threshold_pct: "35" }
max_position: { minis: "8", micros: "80", scaling: "none; 1 mini = 10 micros" }
min_trading_days: null
trading_day_reset: { time: "17:00", tz_as_published: "North American Central Time (CT)" }   # "starts at 5:00 PM and ends at 4:00 PM the following day"; positions closed by 3:59 PM CT
copy_trading_allowed: conditional   # copiers permitted; only user-built tools for the account owner's personal use
automated_trading_allowed: conditional
automation_rule_text: |-
    Yes. Bots, algorithms and trade copiers are permitted. If you connect to Rithmic through a third-party API or composite software, an additional $100 per month applies.
    Only user-built tools intended solely for the account owner’s personal use are permitted. Commercial, shared, rented or publicly distributed automation tools are not allowed.
    These tools are developed and maintained independently by their users, so their uptime, reliability and performance are outside our control, and we cannot provide support or accept responsibility for any issues arising from their use.

    You agree to not abuse the company provided service programs. It includes but is not limited to using “scalping” algorithms, DTC Protocol Bridge API or automated discretional trading in any of the trading environments. Open and close hundreds or thousand of rapid trades by using any trading algorithms. Implementation of reckless transactions in rough markets to benefit from lack of execution. If abuse is suspected, we reserve the right to refuse to claim any profit. To avoid performance discrepancies and/or meet applicable compliance requirements, any third-party algorithm, automated trading software, or other third-party trading technology must be reviewed and approved by the Bulenox management team before use. Approval may be subject to additional requirements or conditions established by Bulenox.
vps_allowed: unknown               # not mentioned on any official page (see §9 G1)
personal_device_required: unknown  # not mentioned on any official page (see §9 G1)
payout_rules: "5 qualifying profitable days (>= $200 net) per cycle; min balance for payout $104,500; Safety Net Reserve $3,000"
status: partial
not_confirmed_fields:
  - "drawdown.amount_usd (the Momentum 'Account Parameters' table is not labelled by stage)"
  - "drawdown.stops_trailing_at (not published for Momentum Master)"
  - "daily_loss_limit (not published separately for Momentum Master)"
  - "max_position (not published separately for Momentum Master)"
  - "automation_rule_text (third-party / commercial tools: FAQ 'Only user-built tools ... Commercial ... not allowed' vs Terms of Use 'any third-party algorithm ... must be reviewed and approved' — see §9 C1)"
sources:
  - { field: drawdown, url: "https://bulenox.com/help-center/momentum#mo-parameters", accessed: "2026-10-23" }
  - { field: consistency_rule, url: "https://bulenox.com/help-center/momentum#mo-consistency", accessed: "2026-10-23" }
  - { field: max_position, url: "https://bulenox.com/help-center/momentum#mo-parameters", accessed: "2026-10-23" }
  - { field: trading_day_reset, url: "https://bulenox.com/help-center/qualification#rules", accessed: "2026-10-23" }
  - { field: copy_trading_allowed, url: "https://bulenox.com/faq", accessed: "2026-10-23" }
  - { field: automated_trading_allowed, url: "https://bulenox.com/faq", accessed: "2026-10-23" }
  - { field: automated_trading_allowed, url: "https://bulenox.com/legal/Terms_of_Use.pdf", accessed: "2026-10-23" }
  - { field: platforms, url: "https://bulenox.com/", accessed: "2026-10-23" }
  - { field: platforms, url: "https://bulenox.com/faq", accessed: "2026-10-23" }
  - { field: vps_allowed, url: "not mentioned — searched help center (all 9 categories), FAQ, Terms of Use, Risk Disclosure, homepage, accounts-pricing", accessed: "2026-10-23" }
  - { field: personal_device_required, url: "not mentioned — same search", accessed: "2026-10-23" }
```

```yaml
# Momentum Master · Option 2 · 150K
firm: bulenox
plan: "Momentum Master — Option 2 (EOD Drawdown / No scaling)"
phase: funded
size_label: "150K"
size_usd: "150000.00"
instruments: [MNQ, NQ]
platforms: [Rithmic, "R | Trader Pro", NinjaTrader, Quantower, "Sierra Chart", MultiCharts, ATAS, Bookmap, MotiveWave, "Investor/RT", "Photon Trader", "Trade Navigator", TSLab, ScalpTool, QST, "Medved Trader", OverCharts, QScalp, VolFix, "Inside Edge", "Optimus Flow", Tiger.Trade]
drawdown: { amount_usd: "4500.00", type: trailing_eod, stops_trailing_at: null }
daily_loss_limit: { amount_usd: "3300.00", kind: soft_pause }
profit_target_usd: null
consistency_rule: { text: "Your best trading day may not exceed 35% of the net profit of the payout cycle. Missing it is not a breach", threshold_pct: "35" }
max_position: { minis: "12", micros: "120", scaling: "none; 1 mini = 10 micros" }
min_trading_days: null
trading_day_reset: { time: "17:00", tz_as_published: "North American Central Time (CT)" }   # "starts at 5:00 PM and ends at 4:00 PM the following day"; positions closed by 3:59 PM CT
copy_trading_allowed: conditional   # copiers permitted; only user-built tools for the account owner's personal use
automated_trading_allowed: conditional
automation_rule_text: |-
    Yes. Bots, algorithms and trade copiers are permitted. If you connect to Rithmic through a third-party API or composite software, an additional $100 per month applies.
    Only user-built tools intended solely for the account owner’s personal use are permitted. Commercial, shared, rented or publicly distributed automation tools are not allowed.
    These tools are developed and maintained independently by their users, so their uptime, reliability and performance are outside our control, and we cannot provide support or accept responsibility for any issues arising from their use.

    You agree to not abuse the company provided service programs. It includes but is not limited to using “scalping” algorithms, DTC Protocol Bridge API or automated discretional trading in any of the trading environments. Open and close hundreds or thousand of rapid trades by using any trading algorithms. Implementation of reckless transactions in rough markets to benefit from lack of execution. If abuse is suspected, we reserve the right to refuse to claim any profit. To avoid performance discrepancies and/or meet applicable compliance requirements, any third-party algorithm, automated trading software, or other third-party trading technology must be reviewed and approved by the Bulenox management team before use. Approval may be subject to additional requirements or conditions established by Bulenox.
vps_allowed: unknown               # not mentioned on any official page (see §9 G1)
personal_device_required: unknown  # not mentioned on any official page (see §9 G1)
payout_rules: "5 qualifying profitable days (>= $250 net) per cycle; min balance for payout $156,500; Safety Net Reserve $4,500"
status: partial
not_confirmed_fields:
  - "drawdown.amount_usd (the Momentum 'Account Parameters' table is not labelled by stage)"
  - "drawdown.stops_trailing_at (not published for Momentum Master)"
  - "daily_loss_limit (not published separately for Momentum Master)"
  - "max_position (not published separately for Momentum Master)"
  - "automation_rule_text (third-party / commercial tools: FAQ 'Only user-built tools ... Commercial ... not allowed' vs Terms of Use 'any third-party algorithm ... must be reviewed and approved' — see §9 C1)"
sources:
  - { field: drawdown, url: "https://bulenox.com/help-center/momentum#mo-parameters", accessed: "2026-10-23" }
  - { field: consistency_rule, url: "https://bulenox.com/help-center/momentum#mo-consistency", accessed: "2026-10-23" }
  - { field: max_position, url: "https://bulenox.com/help-center/momentum#mo-parameters", accessed: "2026-10-23" }
  - { field: trading_day_reset, url: "https://bulenox.com/help-center/qualification#rules", accessed: "2026-10-23" }
  - { field: copy_trading_allowed, url: "https://bulenox.com/faq", accessed: "2026-10-23" }
  - { field: automated_trading_allowed, url: "https://bulenox.com/faq", accessed: "2026-10-23" }
  - { field: automated_trading_allowed, url: "https://bulenox.com/legal/Terms_of_Use.pdf", accessed: "2026-10-23" }
  - { field: platforms, url: "https://bulenox.com/", accessed: "2026-10-23" }
  - { field: platforms, url: "https://bulenox.com/faq", accessed: "2026-10-23" }
  - { field: vps_allowed, url: "not mentioned — searched help center (all 9 categories), FAQ, Terms of Use, Risk Disclosure, homepage, accounts-pricing", accessed: "2026-10-23" }
  - { field: personal_device_required, url: "not mentioned — same search", accessed: "2026-10-23" }
```

```yaml
# Funded (live) · Master size 25K  (size_label = Master Account size)
firm: bulenox
plan: "Funded Account (live; transition from Master-level accounts)"
phase: live
size_label: "25K"
size_usd: "25000.00"
instruments: [MNQ, NQ]
platforms: []   # not stated for Funded Accounts
drawdown: { amount_usd: "1500.00", type: other, stops_trailing_at: "not published — 'Initial End-of-Day (EOD) Drawdown'; whether/how it trails is not stated" }
daily_loss_limit: { amount_usd: null, kind: null }   # not published for Funded Accounts (null = not found; see §9)
profit_target_usd: null
consistency_rule: { text: null, threshold_pct: null }
max_position: { minis: null, micros: null, scaling: null }
min_trading_days: null
trading_day_reset: { time: "17:00", tz_as_published: "North American Central Time (CT)" }   # "starts at 5:00 PM and ends at 4:00 PM the following day"; positions closed by 3:59 PM CT
copy_trading_allowed: conditional   # copiers permitted; only user-built tools for the account owner's personal use
automated_trading_allowed: conditional
automation_rule_text: |-
    Yes. Bots, algorithms and trade copiers are permitted. If you connect to Rithmic through a third-party API or composite software, an additional $100 per month applies.
    Only user-built tools intended solely for the account owner’s personal use are permitted. Commercial, shared, rented or publicly distributed automation tools are not allowed.
    These tools are developed and maintained independently by their users, so their uptime, reliability and performance are outside our control, and we cannot provide support or accept responsibility for any issues arising from their use.

    You agree to not abuse the company provided service programs. It includes but is not limited to using “scalping” algorithms, DTC Protocol Bridge API or automated discretional trading in any of the trading environments. Open and close hundreds or thousand of rapid trades by using any trading algorithms. Implementation of reckless transactions in rough markets to benefit from lack of execution. If abuse is suspected, we reserve the right to refuse to claim any profit. To avoid performance discrepancies and/or meet applicable compliance requirements, any third-party algorithm, automated trading software, or other third-party trading technology must be reviewed and approved by the Bulenox management team before use. Approval may be subject to additional requirements or conditions established by Bulenox.
vps_allowed: unknown               # not mentioned on any official page (see §9 G1)
personal_device_required: unknown  # not mentioned on any official page (see §9 G1)
status: unverified
not_confirmed_fields:
  - "drawdown.type (only 'Initial End-of-Day (EOD) Drawdown' is published)"
  - "daily_loss_limit (not published)"
  - "max_position (not published)"
  - "consistency_rule (not published)"
  - "platforms (not stated for Funded Accounts)"
  - "trading_day_reset (not restated for Funded Accounts)"
  - "copy_trading_allowed / automated_trading_allowed (not restated for live Funded Accounts)"
  - "automation_rule_text (third-party / commercial tools: FAQ 'Only user-built tools ... Commercial ... not allowed' vs Terms of Use 'any third-party algorithm ... must be reviewed and approved' — see §9 C1)"
sources:
  - { field: drawdown, url: "https://bulenox.com/help-center/funded#consolidation", accessed: "2026-10-23" }
  - { field: trading_day_reset, url: "https://bulenox.com/help-center/qualification#rules", accessed: "2026-10-23" }
  - { field: copy_trading_allowed, url: "https://bulenox.com/faq", accessed: "2026-10-23" }
  - { field: automated_trading_allowed, url: "https://bulenox.com/faq", accessed: "2026-10-23" }
  - { field: automated_trading_allowed, url: "https://bulenox.com/legal/Terms_of_Use.pdf", accessed: "2026-10-23" }
  - { field: platforms, url: "https://bulenox.com/", accessed: "2026-10-23" }
  - { field: platforms, url: "https://bulenox.com/faq", accessed: "2026-10-23" }
  - { field: vps_allowed, url: "not mentioned — searched help center (all 9 categories), FAQ, Terms of Use, Risk Disclosure, homepage, accounts-pricing", accessed: "2026-10-23" }
  - { field: personal_device_required, url: "not mentioned — same search", accessed: "2026-10-23" }
```

```yaml
# Funded (live) · Master size 50K  (size_label = Master Account size)
firm: bulenox
plan: "Funded Account (live; transition from Master-level accounts)"
phase: live
size_label: "50K"
size_usd: "50000.00"
instruments: [MNQ, NQ]
platforms: []   # not stated for Funded Accounts
drawdown: { amount_usd: "2000.00", type: other, stops_trailing_at: "not published — 'Initial End-of-Day (EOD) Drawdown'; whether/how it trails is not stated" }
daily_loss_limit: { amount_usd: null, kind: null }   # not published for Funded Accounts (null = not found; see §9)
profit_target_usd: null
consistency_rule: { text: null, threshold_pct: null }
max_position: { minis: null, micros: null, scaling: null }
min_trading_days: null
trading_day_reset: { time: "17:00", tz_as_published: "North American Central Time (CT)" }   # "starts at 5:00 PM and ends at 4:00 PM the following day"; positions closed by 3:59 PM CT
copy_trading_allowed: conditional   # copiers permitted; only user-built tools for the account owner's personal use
automated_trading_allowed: conditional
automation_rule_text: |-
    Yes. Bots, algorithms and trade copiers are permitted. If you connect to Rithmic through a third-party API or composite software, an additional $100 per month applies.
    Only user-built tools intended solely for the account owner’s personal use are permitted. Commercial, shared, rented or publicly distributed automation tools are not allowed.
    These tools are developed and maintained independently by their users, so their uptime, reliability and performance are outside our control, and we cannot provide support or accept responsibility for any issues arising from their use.

    You agree to not abuse the company provided service programs. It includes but is not limited to using “scalping” algorithms, DTC Protocol Bridge API or automated discretional trading in any of the trading environments. Open and close hundreds or thousand of rapid trades by using any trading algorithms. Implementation of reckless transactions in rough markets to benefit from lack of execution. If abuse is suspected, we reserve the right to refuse to claim any profit. To avoid performance discrepancies and/or meet applicable compliance requirements, any third-party algorithm, automated trading software, or other third-party trading technology must be reviewed and approved by the Bulenox management team before use. Approval may be subject to additional requirements or conditions established by Bulenox.
vps_allowed: unknown               # not mentioned on any official page (see §9 G1)
personal_device_required: unknown  # not mentioned on any official page (see §9 G1)
status: unverified
not_confirmed_fields:
  - "drawdown.type (only 'Initial End-of-Day (EOD) Drawdown' is published)"
  - "daily_loss_limit (not published)"
  - "max_position (not published)"
  - "consistency_rule (not published)"
  - "platforms (not stated for Funded Accounts)"
  - "trading_day_reset (not restated for Funded Accounts)"
  - "copy_trading_allowed / automated_trading_allowed (not restated for live Funded Accounts)"
  - "automation_rule_text (third-party / commercial tools: FAQ 'Only user-built tools ... Commercial ... not allowed' vs Terms of Use 'any third-party algorithm ... must be reviewed and approved' — see §9 C1)"
sources:
  - { field: drawdown, url: "https://bulenox.com/help-center/funded#consolidation", accessed: "2026-10-23" }
  - { field: trading_day_reset, url: "https://bulenox.com/help-center/qualification#rules", accessed: "2026-10-23" }
  - { field: copy_trading_allowed, url: "https://bulenox.com/faq", accessed: "2026-10-23" }
  - { field: automated_trading_allowed, url: "https://bulenox.com/faq", accessed: "2026-10-23" }
  - { field: automated_trading_allowed, url: "https://bulenox.com/legal/Terms_of_Use.pdf", accessed: "2026-10-23" }
  - { field: platforms, url: "https://bulenox.com/", accessed: "2026-10-23" }
  - { field: platforms, url: "https://bulenox.com/faq", accessed: "2026-10-23" }
  - { field: vps_allowed, url: "not mentioned — searched help center (all 9 categories), FAQ, Terms of Use, Risk Disclosure, homepage, accounts-pricing", accessed: "2026-10-23" }
  - { field: personal_device_required, url: "not mentioned — same search", accessed: "2026-10-23" }
```

```yaml
# Funded (live) · Master size 100K  (size_label = Master Account size)
firm: bulenox
plan: "Funded Account (live; transition from Master-level accounts)"
phase: live
size_label: "100K"
size_usd: "100000.00"
instruments: [MNQ, NQ]
platforms: []   # not stated for Funded Accounts
drawdown: { amount_usd: "3000.00", type: other, stops_trailing_at: "not published — 'Initial End-of-Day (EOD) Drawdown'; whether/how it trails is not stated" }
daily_loss_limit: { amount_usd: null, kind: null }   # not published for Funded Accounts (null = not found; see §9)
profit_target_usd: null
consistency_rule: { text: null, threshold_pct: null }
max_position: { minis: null, micros: null, scaling: null }
min_trading_days: null
trading_day_reset: { time: "17:00", tz_as_published: "North American Central Time (CT)" }   # "starts at 5:00 PM and ends at 4:00 PM the following day"; positions closed by 3:59 PM CT
copy_trading_allowed: conditional   # copiers permitted; only user-built tools for the account owner's personal use
automated_trading_allowed: conditional
automation_rule_text: |-
    Yes. Bots, algorithms and trade copiers are permitted. If you connect to Rithmic through a third-party API or composite software, an additional $100 per month applies.
    Only user-built tools intended solely for the account owner’s personal use are permitted. Commercial, shared, rented or publicly distributed automation tools are not allowed.
    These tools are developed and maintained independently by their users, so their uptime, reliability and performance are outside our control, and we cannot provide support or accept responsibility for any issues arising from their use.

    You agree to not abuse the company provided service programs. It includes but is not limited to using “scalping” algorithms, DTC Protocol Bridge API or automated discretional trading in any of the trading environments. Open and close hundreds or thousand of rapid trades by using any trading algorithms. Implementation of reckless transactions in rough markets to benefit from lack of execution. If abuse is suspected, we reserve the right to refuse to claim any profit. To avoid performance discrepancies and/or meet applicable compliance requirements, any third-party algorithm, automated trading software, or other third-party trading technology must be reviewed and approved by the Bulenox management team before use. Approval may be subject to additional requirements or conditions established by Bulenox.
vps_allowed: unknown               # not mentioned on any official page (see §9 G1)
personal_device_required: unknown  # not mentioned on any official page (see §9 G1)
status: unverified
not_confirmed_fields:
  - "drawdown.type (only 'Initial End-of-Day (EOD) Drawdown' is published)"
  - "daily_loss_limit (not published)"
  - "max_position (not published)"
  - "consistency_rule (not published)"
  - "platforms (not stated for Funded Accounts)"
  - "trading_day_reset (not restated for Funded Accounts)"
  - "copy_trading_allowed / automated_trading_allowed (not restated for live Funded Accounts)"
  - "automation_rule_text (third-party / commercial tools: FAQ 'Only user-built tools ... Commercial ... not allowed' vs Terms of Use 'any third-party algorithm ... must be reviewed and approved' — see §9 C1)"
sources:
  - { field: drawdown, url: "https://bulenox.com/help-center/funded#consolidation", accessed: "2026-10-23" }
  - { field: trading_day_reset, url: "https://bulenox.com/help-center/qualification#rules", accessed: "2026-10-23" }
  - { field: copy_trading_allowed, url: "https://bulenox.com/faq", accessed: "2026-10-23" }
  - { field: automated_trading_allowed, url: "https://bulenox.com/faq", accessed: "2026-10-23" }
  - { field: automated_trading_allowed, url: "https://bulenox.com/legal/Terms_of_Use.pdf", accessed: "2026-10-23" }
  - { field: platforms, url: "https://bulenox.com/", accessed: "2026-10-23" }
  - { field: platforms, url: "https://bulenox.com/faq", accessed: "2026-10-23" }
  - { field: vps_allowed, url: "not mentioned — searched help center (all 9 categories), FAQ, Terms of Use, Risk Disclosure, homepage, accounts-pricing", accessed: "2026-10-23" }
  - { field: personal_device_required, url: "not mentioned — same search", accessed: "2026-10-23" }
```

```yaml
# Funded (live) · Master size 150K  (size_label = Master Account size)
firm: bulenox
plan: "Funded Account (live; transition from Master-level accounts)"
phase: live
size_label: "150K"
size_usd: "150000.00"
instruments: [MNQ, NQ]
platforms: []   # not stated for Funded Accounts
drawdown: { amount_usd: "4500.00", type: other, stops_trailing_at: "not published — 'Initial End-of-Day (EOD) Drawdown'; whether/how it trails is not stated" }
daily_loss_limit: { amount_usd: null, kind: null }   # not published for Funded Accounts (null = not found; see §9)
profit_target_usd: null
consistency_rule: { text: null, threshold_pct: null }
max_position: { minis: null, micros: null, scaling: null }
min_trading_days: null
trading_day_reset: { time: "17:00", tz_as_published: "North American Central Time (CT)" }   # "starts at 5:00 PM and ends at 4:00 PM the following day"; positions closed by 3:59 PM CT
copy_trading_allowed: conditional   # copiers permitted; only user-built tools for the account owner's personal use
automated_trading_allowed: conditional
automation_rule_text: |-
    Yes. Bots, algorithms and trade copiers are permitted. If you connect to Rithmic through a third-party API or composite software, an additional $100 per month applies.
    Only user-built tools intended solely for the account owner’s personal use are permitted. Commercial, shared, rented or publicly distributed automation tools are not allowed.
    These tools are developed and maintained independently by their users, so their uptime, reliability and performance are outside our control, and we cannot provide support or accept responsibility for any issues arising from their use.

    You agree to not abuse the company provided service programs. It includes but is not limited to using “scalping” algorithms, DTC Protocol Bridge API or automated discretional trading in any of the trading environments. Open and close hundreds or thousand of rapid trades by using any trading algorithms. Implementation of reckless transactions in rough markets to benefit from lack of execution. If abuse is suspected, we reserve the right to refuse to claim any profit. To avoid performance discrepancies and/or meet applicable compliance requirements, any third-party algorithm, automated trading software, or other third-party trading technology must be reviewed and approved by the Bulenox management team before use. Approval may be subject to additional requirements or conditions established by Bulenox.
vps_allowed: unknown               # not mentioned on any official page (see §9 G1)
personal_device_required: unknown  # not mentioned on any official page (see §9 G1)
status: unverified
not_confirmed_fields:
  - "drawdown.type (only 'Initial End-of-Day (EOD) Drawdown' is published)"
  - "daily_loss_limit (not published)"
  - "max_position (not published)"
  - "consistency_rule (not published)"
  - "platforms (not stated for Funded Accounts)"
  - "trading_day_reset (not restated for Funded Accounts)"
  - "copy_trading_allowed / automated_trading_allowed (not restated for live Funded Accounts)"
  - "automation_rule_text (third-party / commercial tools: FAQ 'Only user-built tools ... Commercial ... not allowed' vs Terms of Use 'any third-party algorithm ... must be reviewed and approved' — see §9 C1)"
sources:
  - { field: drawdown, url: "https://bulenox.com/help-center/funded#consolidation", accessed: "2026-10-23" }
  - { field: trading_day_reset, url: "https://bulenox.com/help-center/qualification#rules", accessed: "2026-10-23" }
  - { field: copy_trading_allowed, url: "https://bulenox.com/faq", accessed: "2026-10-23" }
  - { field: automated_trading_allowed, url: "https://bulenox.com/faq", accessed: "2026-10-23" }
  - { field: automated_trading_allowed, url: "https://bulenox.com/legal/Terms_of_Use.pdf", accessed: "2026-10-23" }
  - { field: platforms, url: "https://bulenox.com/", accessed: "2026-10-23" }
  - { field: platforms, url: "https://bulenox.com/faq", accessed: "2026-10-23" }
  - { field: vps_allowed, url: "not mentioned — searched help center (all 9 categories), FAQ, Terms of Use, Risk Disclosure, homepage, accounts-pricing", accessed: "2026-10-23" }
  - { field: personal_device_required, url: "not mentioned — same search", accessed: "2026-10-23" }
```

---

## 9. Unverified / gaps / conflicts

### Conflicts (official pages disagree; ship as "not confirmed")

| ID | Field | Source A | Source B | Where it applies |
|---|---|---|---|---|
| C1 | `automation_rule_text`: whether third-party or commercial tools (for example a bought trade copier) may be used | FAQ: "Only user-built tools intended solely for the account owner’s personal use are permitted. Commercial, shared, rented or publicly distributed automation tools are not allowed." (https://bulenox.com/faq) | ToU: "any third-party algorithm, automated trading software, or other third-party trading technology must be reviewed and approved by the Bulenox management team before use." This reads as allowed after approval (https://bulenox.com/legal/Terms_of_Use.pdf) | All 44 blocks. The classification is `conditional` under both texts. Only the condition differs. |

### Items for PO review (possible inconsistency in wording; values were not changed)

| ID | Item |
|---|---|
| N1 | **Fast Track and Momentum Option 2 DLL kind.** The plan pages give the amounts but do not restate the consequence. `soft_pause` comes from the generic Option 2 text on S-PRICE ("pauses your trading for the day instead of closing the account"). That generic block also says "Option 2 · Scaling plan" and "Same account sizes, same targets, same platforms. Only the risk model changes." The Fast Track and Momentum pages and cards say "Option 2 — no scaling", and Fast Track 25K Option 2 has "no daily loss limit". I recorded the plan-specific values and marked the DLL kind not confirmed for those blocks. |
| N2 | **Qualification Option 2 "Contracts EOD".** The pricing card shows 3/7/12/15. The Help Center scaling plan starts lower (2/2/3/5) and reaches those values only at the top tier. YAML `minis` = top tier, with the full scaling text. |
| N3 | **Momentum "Account Parameters" table** is not labelled Qualification vs Momentum Master. It is applied to the qualification stage, and to Momentum Master as not confirmed. |
| N4 | **Rithmic professional data fee** (fees are not needed for presets): "$116" monthly (S-HQ #status, S-PRICE) vs "$112 per exchange" (S-HCO). |

### Gaps (not found on any official public page)

| ID | Field | What is missing | Where searched |
|---|---|---|---|
| G1 | `vps_allowed`, `personal_device_required` | No text on VPS, server, VPN or personal device | Help Center (all categories, CMS JSON in all languages), FAQ, ToU, Risk Disclosure, homepage, accounts-pricing |
| G2 | Copy to/from other firms; explicit same-person cross-account copying | Not mentioned. Copiers are "permitted" in general. | FAQ, Help Center (Qualification rules, Multiple Accounts, Master), ToU |
| G3 | `drawdown.stops_trailing_at` for the Qualification Account (both options) and the Momentum qualification stage | Lock rule published only for Master and Fast Track | S-HQ, S-HMO, S-PRICE |
| G4 | `consistency_rule` for the Qualification Account | No rule and no "no consistency" statement (Momentum has one: "no consistency requirement at this stage") | S-HQ, S-PRICE, S-FAQ |
| G5 | Momentum Master drawdown amount, lock, DLL, contract limit | Only payout rules are stage-specific | S-HMO |
| G6 | Funded (live) Account: DLL, contract limits, drawdown behaviour after the "Initial EOD Drawdown", consistency, reset, platforms, automation/copy stance | Funded Account Agreement and Master Account Agreement are not public (sent by email after approval) | S-HFU, S-HMA, S-FAQ, ToU |
| G7 | Platforms for the Funded (live) Account | Not stated | S-HFU, S-HCO |
| G8 | Holiday calendar / early-close rules | Only "Weekends and public holidays are not counted as trading days." | S-HQ, S-HMA, S-FAQ, S-HOME |
| G9 | Micros per size for Qualification and Master | Only the ratio "1 standard = 10 micros" is published (Fast Track and Momentum publish micros explicitly) | S-HQ, S-HMA, S-PRICE |
| G10 | Webhooks/alerts, semi-automated trading, ATM strategies, mandatory stop loss, hedging | Not mentioned (OCO note only; see §6) | FAQ, Help Center, ToU |

Not reached: none. Every public page loaded with curl or the portal. `bulenox.com/member/*` (login, checkout) was deliberately not opened.

---

## 10. Changelog

- 2026-10-23 — initial research (Lupa). Format rev. 2 plus PO rev. 3 (VPS / personal device). Sources: bulenox.com homepage, accounts-pricing, FAQ, Help Center (via the site's public CMS JSON, with the rendered FAQ answer checked in a browser portal), Terms of Use (Updated October 2026), Risk Disclosure.
