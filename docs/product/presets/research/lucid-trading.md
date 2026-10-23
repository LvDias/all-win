# Lucid Trading — Prop Firm Rules Research

| | |
|---|---|
| **Firm** | Lucid Trading |
| **Official domains** | `lucidtrading.com` (main site; blocks plain curl with Cloudflare 403, read through a browser portal) · `support.lucidtrading.com` (official help center, Intercom-hosted, linked from the main site's header "FAQ"/support link `https://support.lucidtrading.com/en/` and from the homepage link "For a full list of supported trading platforms see our FAQ" → `https://support.lucidtrading.com/en/articles/11404614-what-platforms-are-supported`) · `store.lucidtrading.com` (store, not used) |
| **Research date** | 2026-10-23 (all URLs below accessed 2026-10-23) |
| **Researcher** | Lupa (Prop Firm Rules Researcher) |
| **File status** | `partial` — the main fields are verified for every current plan. Still open: the conflicts in §11, an explicit trading-day reset time, an explicit same-person copy statement, and webhooks (see §11) |
| **Instruments covered** | MNQ, NQ. Both are listed as approved products on the CME (https://support.lucidtrading.com/en/articles/11508978-approved-products-and-commissions) |
| **Platforms (for routing)** | "All listed platforms work with all Lucid account types." CQG: NinjaTrader, Tradovate, TradingView. Rithmic: Quantower, Tradesea, MotiveWave, Sierra Chart, Jigsaw, Bookmap, ATAS, R\|Trader Pro, MultiCharts. Source: https://support.lucidtrading.com/en/articles/11404614-lucid-trading-supported-platforms |

Evidence (raw text of every cited page, URL on line 1): `/tmp/claude-1003/-srv-www-allwin--maestri-roles-6bf77a80-a1ed-43dc-bc00-08a480f178a5/33652d13-4dae-46b6-ae06-44a72b375517/scratchpad/lucid/`. This is temporary scratch storage, so the evidence must be re-captured at re-verification.

**Programs found.** The homepage tabs are "Pro / Flex / Daily / Direct". The help center has one collection each for LucidPro, LucidFlex, LucidDaily, LucidDirect and Live Trading. **LucidMaxx** is invite-only ("not available to the general public"). **LucidBlack** is a legacy collection ("LucidBlack (Legacy)"), and so are the "… Live (Legacy)" pages, which apply to "accounts purchased/reset on 2/27/26 and prior". All plans are offered in 25K, 50K, 100K and 150K. No size outside 25K–300K was found.

**Help-center convention used below.** "MLL" = Max Loss Limit (the drawdown). "Initial Trail Balance" = the balance at which the MLL stops trailing. "Locked MLL Balance" = where it locks.

---

## 1. LucidPro

Sources:
- Evaluation: https://support.lucidtrading.com/en/articles/12890029-lucidpro-evaluation-account
- Funded: https://support.lucidtrading.com/en/articles/12890069-lucidpro-funded-account
- Drawdown: https://support.lucidtrading.com/en/articles/12890136-lucidpro-drawdown
- DLL: https://support.lucidtrading.com/en/articles/12890122-lucidpro-daily-loss-limit
- Consistency: https://support.lucidtrading.com/en/articles/12890109-lucidpro-consistency-percentage
- Payouts: https://support.lucidtrading.com/en/articles/12890092-lucidpro-payouts
- Customization: https://support.lucidtrading.com/en/articles/16226068-lucidpro-customization
- Homepage cards: https://lucidtrading.com/

### 1.1 LucidPro — Evaluation

| Size | Profit target | MLL (drawdown) | Drawdown type / lock | DLL (fixed) | Consistency | Max size | Min days |
|---|---|---|---|---|---|---|---|
| 25K ($25,000) | $1,250 | $1,000 | EOD; locks once above $26,100 → Locked MLL $25,100 | **conflict**: "None" (help) vs "$600" (homepage 25K PRO EVAL card, DLL toggle "$600 / OFF") | none published | 2 mini or 20 micros | none published |
| 50K ($50,000) | $3,000 | $2,000 | EOD; $52,100 → $50,100 | $1,200 | none published | 4 mini or 40 micros | none published |
| 100K ($100,000) | $6000 (written this way) | $3,000 | EOD; $103,100 → $100,100 | $1,800 | none published | 6 mini or 60 micros | none published |
| 150K ($150,000) | $9,000 | $4,500 | EOD; $154,600 → $150,100 | $2,700 | none published | 10 mini or 100 micros | none published |

- Drawdown, quoted: "LucidPro evaluation and funded accounts use an End-of-Day Drawdown (EOD Drawdown) system … Once your account exceeds the Initial Trail Balance, the MLL locks and no longer moves … the MLL locks at the initial balance plus $100 … If your account balance reached the MLL, your account will be breached."
- DLL is a soft pause, quoted: "All DLLs at Lucid Trading are considered soft breaches, meaning you do not lose your account for hitting DLL as long as the Max Loss Limit (MLL) has not been reached." / "you will be restricted from placing additional trades until the next trading session".
- The DLL is chosen at checkout (customization page), quoted: "Daily Loss Limit: On or Off … Daily Loss Limit Off — No Daily Loss Limit is applied during either the evaluation or funded phase." The amounts above apply only when DLL is On.
- Minimum days: the page states "Pass the evaluation in one trading day". No minimum trading-day count is published.
- Homepage cross-check (Pro tab, captured 2026-10-23, https://lucidtrading.com/): the eval cards show "Daily Loss Limit $600 / $1,200 / $1,800 / $2,700" with an "OFF" toggle. Max Size is written "2 Mini OR 20 Micro" etc. The other values match the help center.

### 1.2 LucidPro — Funded

| Size | MLL | Drawdown type / lock | Fixed DLL (optional) | Scaling DLL | Consistency (payout) | Max size | Scaling plan |
|---|---|---|---|---|---|---|---|
| 25K | $1,000 | EOD; $26,100 → $25,100 | **conflict**: "$600" (funded overview) vs "None" (DLL article) | 60% of Peak EOD Balance | 40% | 2 mini / 20 micros | "No scaling plan" |
| 50K | $2,000 | EOD; $52,100 → $50,100 | $1,200 | 60% of Peak EOD Balance | 40% | 4 / 40 | none |
| 100K | $3,000 | EOD; $103,100 → $100,100 | $1,800 | 60% of Peak EOD Balance | 40% | 6 / 60 | none |
| 150K | $4,500 | EOD; $154,600 → $150,100 | $2,700 | 60% of Peak EOD Balance | 40% | 10 / 100 | none |

- DLL scaling, quoted from the DLL article: "Once the account closes above the Initial Trail Balance, the Fixed DLL is replaced by the LucidScale DLL. LucidScale DLL Formula: Highest end-of-day account profit × 60% = LucidScale DLL … It does not decrease, even if the account draws down."
  - The funded overview words the same rule as "60% of Peak EOD Balance" (balance, not profit). The two wordings differ; see §11.
- Consistency, quoted: "Your largest single-day profit must be no more than 40% of your total profit during the payout cycle." Formula: "Largest Single Day Profit / Account Profit = Consistency Percentage". The rule is 35% for accounts "purchased or reset on or before" 11/28/2025 3:00 PM EST.
- Rules that change what the trader may do:
  - Payout buffer, quoted: "The buffer is equal to: Initial Max Loss Limit + $100". A trade that drops the balance into the buffer before the payout is processed may get the request denied.
  - Minimum profit goal per cycle: 25K $250 / 50K $500 / 100K $750 / 150K $1,000.
  - With DLL On, the account has On Demand Payouts. Quoted: "After requesting an intraday payout, trading on your funded LucidPro account is paused until the end of the trading day."
- Homepage "Funded Rules" card backs (Pro tab, https://lucidtrading.com/):
  - "Payout Profit Target": $250 / $500 / $750 / $1,000.
  - "DLL (Below Initial Trail)": $600 / $1,200 / $1,800 / $2,700. The 25K value $600 conflicts with the DLL article "None" (§11 #1).
  - "LucidScale DLL (Above Initial Trail) 60% of Peak EOD Balance"; "Consistency 40%"; "Scaling Plan —"; "Payouts to Live: 5".
  - "Days to Payout 3". This appears only on the homepage; the LucidPro payouts article has no minimum-days rule (§11 #13).

## 2. LucidFlex

Sources:
- Evaluation: https://support.lucidtrading.com/en/articles/12945790-lucidflex-evaluation-account
- Funded: https://support.lucidtrading.com/en/articles/12945795-lucidflex-funded-account
- Drawdown: https://support.lucidtrading.com/en/articles/12945815-lucidflex-drawdown
- Scaling plan: https://support.lucidtrading.com/en/articles/12945808-lucidflex-scaling-plan
- Consistency: https://support.lucidtrading.com/en/articles/12945805-lucidflex-consistency-percentage
- Payouts: https://support.lucidtrading.com/en/articles/12945796-lucidflex-payouts
- Customization: https://support.lucidtrading.com/en/articles/16226050-lucidflex-customization

### 2.1 LucidFlex — Evaluation

| Size | Profit target | MLL | Drawdown type / lock | DLL | Consistency | Max size |
|---|---|---|---|---|---|---|
| 25K | $1,250 | $1,000 | EOD; $26,100 → $25,100 | $600 (homepage; On/Off at checkout) | **conflict**: help "40% (50% with add-on)" vs homepage "50%" | 2 mini or 20 micros |
| 50K | $3,000 | $2,000 | EOD; $52,100 → $50,100 | $1,200 (homepage) | conflict (same) | 4 / 40 |
| 100K | $6,000 | $3,000 | EOD; $103,100 → $100,100 | $1,800 (homepage) | conflict (same) | 6 / 60 |
| 150K | $9,000 | $4,500 | EOD; $154,600 → $150,100 | $2,700 (homepage) | conflict (same) | 10 / 100 |

- Consistency, quoted: "In LucidFlex evaluation accounts your Consistency Percentage must be 40% or less to be eligible to upgrade … If you added the 50% consistency add-on at checkout, it must be 50% or less."
  - The firm says the 50% add-on has a "cushion". It publishes an example table: 25K $650.00 / 50K $1,560.00 / 100K $3,120.00 / 150K $4,680.00, with the note "NOT a fixed dollar amount".
- Minimum days: no minimum is published. Quoted: "Pass in as few as 3 days with 40% consistency, or 2 days with the 50% consistency add-on".
- Scaling, quoted: "There is no scaling plan in the evaluation phase."
- DLL amounts come from the homepage only (Flex tab, https://lucidtrading.com/). The eval cards show "Daily Loss Limit $600 [toggle] OFF … DLL $600" (25K), and $1,200 / $1,800 / $2,700 for the larger sizes. The help center gives no Flex amounts, so these are single-source.
- Consistency on the homepage Flex eval cards: "Consistency 50%", with no 40% option visible on the card. The help center says 40% is standard and 50% is an add-on. Recorded as a conflict (§11 #4).

### 2.2 LucidFlex — Funded

| Size | MLL | Drawdown | DLL | Consistency | Max size | Scaling plan (simulated profit → contracts, updated end of session) |
|---|---|---|---|---|---|---|
| 25K | $1,000 | EOD; $26,100 → $25,100; on payout the MLL moves to the Locked MLL Balance | Optional; $600 "DLL (Below Initial Trail)" (homepage) | none | 2 mini / 20 micros | $0–$999: 1 mini or 10 micros · $1,000–$1,999: 2 or 20 |
| 50K | $2,000 | EOD; $52,100 → $50,100 | Optional; $1,200 (homepage) | none | 4 / 40 | $0–$999: 2/20 · $1,000–$1,999: 3/30 · $2,000–$2,999: 4/40 |
| 100K | $3,000 | EOD; $103,100 → $100,100 | Optional; $1,800 (homepage) | none | 6 / 60 | $0–$999: 3/30 · $1,000–$1,999: 4/40 · $2,000–$2,999: 5/50 · $3,000–$4,499: 6/60 |
| 150K | $4,500 | EOD; $154,600 → $150,100 | Optional; $2,700 (homepage) | none | 10 / 100 | $0–$999: 4/40 · $1,000–$1,999: 5/50 · $2,000–$2,999: 6/60 · $3,000–$4,499: 8/80 · $4,500+: 10/100 |

- Drawdown, quoted: "Once you request a payout from LucidFlex, your MLL automatically adjusts to the Locked MLL Balance".
- Consistency, quoted: "There is no Consistency Percentage on LucidFlex funded accounts".
- Scaling, quoted: "The scaling plan updates at the end of each session, so your limits will not update in real-time throughout the day." If a trader circumvents the limits, the result "may result in having profit wiped for the day".
- Payout rules:
  - Minimum profit on 5 separate days per cycle: 25K $100 / 50K $150 / 100K $200 / 150K $250.
  - Positive net profit in the cycle.
  - "There is no buffer balance".
  - "Traders may take up to 5 payouts from each LucidFlex account after which they will be moved live."
  - With DLL On, trading is paused until the end of the trading day after an intraday payout. Accounts with a hedging violation are not eligible for intraday payouts.
- Homepage "Funded Rules" card backs (Flex tab):
  - "DLL (Below Initial Trail)": $600 / $1,200 / $1,800 / $2,700. No "LucidScale" row is shown for Flex, and the help center publishes no Flex scaling DLL, so the DLL above the Initial Trail is unverified.
  - "Min Days of Profit 5 of $100 / $150 / $200 / $250"; "Days to Payout 5"; "Payouts to Live: 5".
  - The scaling-plan popup matches the help-center table.

## 3. LucidDaily

Sources:
- Evaluation: https://support.lucidtrading.com/en/articles/15996664-luciddaily-evaluation
- Funded: https://support.lucidtrading.com/en/articles/15997244-luciddaily-funded-account
- Drawdown: https://support.lucidtrading.com/en/articles/15998425-luciddaily-drawdown
- DLL: https://support.lucidtrading.com/en/articles/16085900-luciddaily-daily-loss-limit
- Consistency: https://support.lucidtrading.com/en/articles/15998336-luciddaily-consistency
- Customization: https://support.lucidtrading.com/en/articles/16033858-luciddaily-customization
- Payouts: https://support.lucidtrading.com/en/articles/15997266-luciddaily-payouts
- Live: https://support.lucidtrading.com/en/articles/16010520-luciddaily-live

### 3.1 LucidDaily — Evaluation

| Size | Profit target | MLL | Drawdown type / lock | DLL (if On) | Consistency | Max size |
|---|---|---|---|---|---|---|
| 25K | $1,250 | $1,000 | **EOD or Intraday, chosen at checkout**; $26,100 → $25,100 | $600 | **conflict** 50% vs 40%/50% | 2 mini / 20 micros |
| 50K | $3,000 | $2,000 | same; $52,100 → $50,100 | $1,200 | conflict | 4 / 40 |
| 100K | $6,000 | $3,000 | same; $103,100 → $100,100 | $1,800 | conflict | 6 / 60 |
| 150K | $9,000 | $4,500 | same; $154,600 → $150,100 | $2,700 | conflict | 10 / 100 |

- Drawdown, quoted: "LucidDaily evaluation accounts may have an End-of-Day drawdown or an Intraday drawdown depending on which option you select at check out." / "Intraday drawdown brings your MLL up with your real-time P&L".
- Consistency conflict:
  - The evaluation and consistency articles say "50%" ("you must have Consistency Percentage of 50% or less").
  - The customization article says "Consistency: 40% or 50% … 40% (standard) … 50% (add-on)".
  - The customization article also contradicts itself on the number of configurations: "Two settings" then lists three, "eight unique … configurations" then "four possible combinations".
- Homepage (Daily tab): the eval cards show "Drawdown In Eval EOD / Intraday", "Daily Loss Limit $600 [toggle] OFF" (25K; $1,200 / $1,800 / $2,700 for the larger sizes) and "Consistency 50%". These match the evaluation article. The 40% option from the customization article is not shown.
- DLL, quoted: "The DLL remains fixed throughout the trading session and does not trail upward with the Max Loss Limit (MLL). If the MLL trails above the DLL during the session, the MLL becomes the more restrictive threshold. Reaching the MLL at that point will result in a hard breach."

### 3.2 LucidDaily — Funded

| Size | MLL | Drawdown | DLL (if On) | Consistency | Max size |
|---|---|---|---|---|---|
| 25K | $1,000 | Intraday (always); $26,100 → $25,100 | $600 | none | 2 mini / 20 micros |
| 50K | $2,000 | Intraday; $52,100 → $50,100 | $1,200 | none | 4 / 40 |
| 100K | $3,000 | Intraday; $103,100 → $100,100 | $1,800 | none | 6 / 60 |
| 150K | $4,500 | Intraday; $154,600 → $150,100 | $2,700 | none | 10 / 100 |

- Drawdown, quoted: "The LucidDaily funded accounts always use an Intraday drawdown."
- News is a hard breach. Verbatim (separate lines on the page):
  > Trading U.S high impact red folder news in a LucidDaily funded account is not permitted and is a hard breach. This restriction applies only when BOTH of the following are true:
  > Impact = High
  > Currency = USD
  > You must be flat from 1 minute before through 1 minute after the news event. You may not hold or open new positions through that window. Your funded account will be breached.
- Homepage "Funded Rules" (Daily tab): "DLL $600 / $1,200 / $1,800 / $2,700", "Drawdown Type Intraday", "Consistency —", "Daily Payouts ✓". These match the help center.
- Payout rules:
  - Buffer = "Initial Max Loss Limit + $100": 25K $26,100 / 50K $52,100 / 100K $103,100 / 150K $154,600.
  - "Maximum Daily Profit": 25K $6,000 / 50K $8,000 / 100K $10,000 / 150K $12,000. Quoted: "If you meet or exceed the daily amount you are automatically moved live".

## 4. LucidDirect (straight-to-funded, no evaluation)

Sources:
- Funded: https://support.lucidtrading.com/en/articles/12890148-luciddirect-funded-account
- Drawdown: https://support.lucidtrading.com/en/articles/12890192-luciddirect-drawdown
- DLL: https://support.lucidtrading.com/en/articles/12890185-luciddirect-daily-loss-limit
- Consistency: https://support.lucidtrading.com/en/articles/12890178-luciddirect-consistency-percentage
- Payouts: https://support.lucidtrading.com/en/articles/12890164-luciddirect-payout-objectives

| Size | MLL | Drawdown / lock | Fixed DLL | Scaling DLL | Consistency (payout) | Max size |
|---|---|---|---|---|---|---|
| 25K | $1,000 | EOD; $26,100 → $25,100 | None | None | 20% | 2 mini / 20 micros |
| 50K | $2,000 | EOD; $52,100 → $50,100 | $1,200 | 60% of Peak EOD Balance | 20% | 4 / 40 |
| 100K | $3,500 | EOD; $103,600 → $100,100 | $2,100 | 60% of Peak EOD Balance | 20% | 6 / 60 |
| 150K | $5,000 | EOD; $155,100 → $150,100 | $3,000 | 60% of Peak EOD Balance | 20% | 10 / 100 |

- DLL, quoted: "LucidDirect has no add-ons; the Daily Loss Limit is set by account size."
- Scaling plan, quoted: "No scaling plan, access to max contract size immediately".
- Payout profit goal (Goal 1 / Goal 2+): 25K $1,500/$1,250 · 50K $3,000/$2,500 · 100K $6,000/$3,500 · 150K $9,000/$4,500.
- Homepage (Direct tab, https://lucidtrading.com/):
  - "DLL (Below Initial Trail)": NONE / $1,200 / $2,100 / $3,000.
  - "LucidScale DLL (Above Initial Trail)": NONE (25K) / "60% of Peak EOD Balance".
  - "Consistency Rule 20%"; "Max Size 2 Mini OR 20 Micro" … "10 Mini OR 100 Micro". These match the help center.
  - "Min Day to Payout 5" and "Max Accounts 5". The minimum-days rule appears only on the homepage; the LucidDirect payout article has no minimum-days rule (§11 #13).

## 5. Live (current "New Live Structure") and invite-only / legacy programs

Sources:
- New live structure: https://support.lucidtrading.com/en/articles/16558826-new-live-structure
- Live scaling plan: https://support.lucidtrading.com/en/articles/15245873-live-scaling-plan
- Allowed trading times: https://support.lucidtrading.com/en/articles/11404729-allowed-trading-times

**Live is a real account. Agents must never connect to it.**

**How traders move live**
- Traders enter "the live review pool" after Payout 5 (Flex/Pro/Direct), after reaching the Maximum Daily Profit (Daily), or at the risk team's discretion.
- Quoted: "All live accounts begin with: $0 starting balance / Daily payouts / End of Day (EOD) drawdown / No daily loss limit".

**Live accounts by funded-account size**

| Funded size | Starting live drawdown | Max live contracts |
|---|---|---|
| 25K | $1,000 | 2 minis or 20 micros |
| 50K | $2,000 | 4 / 40 |
| 100K | $3,000 | 6 / 60 |
| 150K | $4,500 | 10 / 100 |

**Drawdown lock**, quoted: "Once a trader generates live profits equal to their starting live drawdown, the Max Loss Limit locks at $100 and no longer moves. If a trader requests a live payout before generating profits over the live drawdown, the Max Loss Limit locks to $100."

**Live scaling plan.** It is published per exchange (CME/CBOT/NYMEX/COMEX) and updated "at the end of each trading day". The CME column (NQ/MNQ) is in the YAML below.

**Holding rule.** Verbatim (separate lines on the page):
> Swing trading is not allowed in the new LucidLive accounts
> Traders may hold positions until 4:45 PM EST on Tradovate Live accounts, or 4:15 PM EST on Rithmic Live accounts

**LucidMaxx (invite-only, low priority; no YAML)**

Source: https://support.lucidtrading.com/en/articles/17327076-new-lucidmaxx-program

| Size | Step 1 evaluation (target / consistency / EOD drawdown) | Step 2 pre-live (target / EOD drawdown) | Live |
|---|---|---|---|
| 25K | $1,250 / 40% / $1,000 | $1,000 / $1,000 | static MLL $100, starting balance $1,000 |
| 50K | $3,000 / 40% / $2,000 | $2,000 / $2,000 | static MLL $100, starting balance $2,000 |
| 100K | $6,000 / 40% / $3,000 | $3,000 / $3,000 | static MLL $100, starting balance $3,000 |
| 150K | $9,000 / 40% / $4,500 | $4,500 / $4,500 | static MLL $100, starting balance $4,500 |

- "No DLL (No Daily Loss Limit)"; "Up to 5 accounts"; "Standard Live Scaling Plan applies" in Step 2.
- The program cannot be bought publicly, so no preset is proposed.

**Legacy (not recorded in detail).** These cover legacy accounts only and are not proposed as presets:
- LucidBlack: 25K–100K. Evaluation consistency 60%. Quoted: "There is no DLL on LucidBlack evaluation accounts". Source: https://support.lucidtrading.com/en/articles/13424894-lucidblack-evaluation-account
- Legacy LucidMaxx and the "… Live (Legacy)" pages: they apply to accounts "purchased/reset on 2/27/26 and prior" or earlier dates.

---

## 6. Trading-day reset

**Not published as an explicit "reset time".** The official pages define the session as follows (quoted):
- Verbatim, under "LucidPro, LucidFlex and LucidDirect Accounts" (separate lines on the page):
  > All positions must be closed by 4:45 PM EST, Monday through Friday
  > Any open positions will be automatically closed by Lucid Trading at 4:45 PM EST
  > Important: Holding a position past this time does not result in a failed account
- "Trading resumes at 6:00 PM EST, Sunday through Thursday"
- Holidays: "On market holidays with altered closing times, all positions must be closed before the market closes, regardless of the usual 4:45 PM cutoff"
- Source for the three quotes above: https://support.lucidtrading.com/en/articles/11404729-allowed-trading-times
- The on-demand payout window is "during the trading session, from profit made that session (6:00 PM to 3:45 PM ET)". Sources: https://support.lucidtrading.com/en/articles/16226068-lucidpro-customization, https://support.lucidtrading.com/en/articles/12890185-luciddirect-daily-loss-limit
- "We move traders live every evening after the final session report comes in at 6 PM est". Source: https://support.lucidtrading.com/en/articles/16010520-luciddaily-live
- A DLL lock lasts "until the next trading session" (DLL articles).

The Terms of Use, Privacy Policy, Refund/Chargeback Policy and Risk Disclosure (read 2026-10-23) contain no reset or session times.

Note: the session end is published as "3:45 PM ET" (payout window) in one place and as the "4:45 PM EST" close-out in another. The YAML leaves `trading_day_reset` as `null`. The PO/user must decide whether the session start "6:00 PM EST/ET" is the reset for the preset.

## 7. Trade copying stance

Verbatim quote, from https://support.lucidtrading.com/en/articles/11404728-other-trading-activities:

> **Automated Strategies**
> Automated trading systems and trade copiers are permitted
> All automated activity must comply with Lucid Trading rules
> Traders are fully responsible for any software errors, malfunctions, or unintended outcomes

**Hedging (limits what copying may do).** Verbatim, from https://support.lucidtrading.com/en/articles/11404734-prohibited-hedging:

> Lucid Trading strictly prohibits all forms of hedging, including but not limited to:
> Hedging across multiple accounts held by the same user
> Hedging between different users' accounts
> Hedging between different firms or funded platforms
> Any other form of hedging designed to bypass risk management rules

> you cannot go long / short the same contract but one is minis or micros in separate accounts. This is strictly prohibited. … you cannot go long / short different contracts in separate accounts with correlated assets.

**Live hedging and copiers.** Verbatim, from https://support.lucidtrading.com/en/articles/16558826-new-live-structure:

> Hedging live accounts is strictly prohibited by both Lucid and the CME. This includes any copytrader issues that may arise. Violations will result in a permanent ban.

**Account-count limits.** Verbatim, from https://support.lucidtrading.com/en/articles/11404617-maximum-number-of-accounts:

> Account Limit: Up to 10 active evaluation accounts per household or family.
> Shared Limit: Traders may hold a maximum of 10 accounts in total across evaluation accounts and funded accounts.
> Account Limit: Up to 5 active funded accounts per household or family.
> Account Limit: Up to 5 active LucidLive accounts per household or family.

**Household rule.** Quoted: "If one household member is trading live, others may not trade simulated accounts." (live FAQ)

**Terms of Use** (https://lucidtrading.com/terms-of-use/, "Last Modified: March 20, 2025"). Verbatim items from "Prohibited Uses":

> Performing, alone or in concert with any other persons, including connected accounts, or accounts held with different Users, trades, or combination of trades, the purpose of which is to manipulate, abuse, or give any User an unfair advantage while using the Website or Services.

> In concert with others, including between unconnected accounts and third parties, performing the same trading strategy, opposite strategy, or other activity mean to pool or hedge risk of the aggregate, unconnected accounts.

> Trading on behalf of others including but not limited to sharing any incentives as part of any business arrangement.

**Refund/Chargeback Policy** (https://lucidtrading.com/disclaimer-of-liability-refunds-and-chargeback-policy/). It names copiers, as a liability disclaimer only:

> Individual malfunctions, failures, or errors in trade copier systems;,

**Homepage FAQ** (https://lucidtrading.com/, collapsed answer). Verbatim: "We allow a max of 5 funded accounts per household." This matches the help center. The Direct cards also show "Max Accounts 5".

**Reading:**
- Trade copiers are permitted (help center). No official text explicitly says "copying across the same person's accounts is allowed", so this is recorded as `conditional`; see §11.
- The Terms prohibit, between "unconnected accounts and third parties", performing "the same trading strategy" to "pool or hedge risk". This would cover copying into or from other people's accounts. They also prohibit "Trading on behalf of others". Copying between one person's own accounts is not addressed by those clauses.
- Copying must never create opposite or correlated positions across accounts (own, other users', or other firms').
- No rule on copying from or to accounts at other firms was found, beyond the hedging ban between firms.

## 8. Automated trading stance

**(i) Classification:** `conditional` (permitted; HFT, microscalping and exploiting the simulation are prohibited). All phases and plans share one rule text.

Rule text stored in `automation_rule_text` (three separate bullets under "Automated Strategies", checked against the captured page; https://support.lucidtrading.com/en/articles/11404728-other-trading-activities, accessed 2026-10-23): "Automated trading systems and trade copiers are permitted" / "All automated activity must comply with Lucid Trading rules" / "Traders are fully responsible for any software errors, malfunctions, or unintended outcomes".

**(ii) Named mechanisms:**
- **Semi-automated trading:** not mentioned.
- **Trade copiers:** named and permitted (§7).
- **Webhooks / TradingView alerts:** not named. TradingView is a supported platform (CQG route), but no page addresses alert-to-order automation.
- **Automatic brackets / ATM:** not mentioned. NinjaTrader is a supported platform.
- **Entry methods:** "Lucid does not impose limits on entry methods" (same page, under "Scaling Into Trades (DCA)"). It refers to scaling, not to automation.

**(iii) Differs by phase:** no. No page limits automation by plan or phase. The only phase-specific item near it is the LucidLive hedging line, which mentions "copytrader issues" (§7).

Detail:

Verbatim, from https://support.lucidtrading.com/en/articles/11404728-other-trading-activities. These are separate bullets under "Automated Strategies":
- "Automated trading systems and trade copiers are permitted"
- "All automated activity must comply with Lucid Trading rules"
- "Traders are fully responsible for any software errors, malfunctions, or unintended outcomes"

**Terms of Use** (https://lucidtrading.com/terms-of-use/), verbatim items from "Prohibited Uses":
- "Using any software, artificial intelligence, ultra-high speed, or mass data entry which might manipulate, abuse, or give any User an unfair advantage while using the Website, the Software and/or the Services."
- "Using any trading strategy that includes disruptive practices (as determined in Lucid’s sole discretion, including but not limited to spoofing strategies, microscalping, hedging, and high frequency trading."
- "Performing trades using an external or slow data feed."

**Refund/Chargeback Policy**, verbatim (liability disclaimer): "Individual issues, errors, or malfunctions relating to any third-party applications, including, without limitation, automated trading systems, custom indicators, or other third-party software;,"

The Terms clause on software is conditional ("which might manipulate, abuse, or give any User an unfair advantage"). It does not contradict the help-center permission, so it is not marked as a conflict.

**Limits on automation:**
- **HFT is prohibited.** Source: https://support.lucidtrading.com/en/articles/11404736-prohibited-high-frequency-trading
  - Quoted: "High-frequency trading (HFT) is a type of automated trading strategy that involves submitting and executing a high volume of trades within extremely short time frames … HFT is strictly prohibited."
  - Penalties: a first offense gets a written warning. Repeated offenses lead to profits removed, accounts closed and a permanent restriction.
  - No numeric threshold is published.
- **Microscalping is prohibited.** Source: https://support.lucidtrading.com/en/articles/11404742-prohibited-microscalping
  - Quoted: "You will be flagged if: More than 50% of your profits are generated from trades held for 5 seconds or less".
- **Exploiting the simulation is prohibited.** Quoted: "Taking advantage of system errors or update delays / Exploiting discrepancies in simulated prices". Source: https://support.lucidtrading.com/en/articles/11404732-trade-with-integrity

**Webhooks.** Webhook automation (e.g. TradingView alerts) is not named on any official page found, including the Terms of Use, Privacy Policy, Refund Policy and Risk Disclosure. TradingView is listed as a supported platform.

## 9. Other rules that affect routing or risk

### Device / hosting rules (format rev. 3 §3 item 7)

YAML for all 32 blocks: `vps_allowed: unknown` and `personal_device_required: unknown`. **No official page states a rule on VPS, VPN, remote servers or the originating device.** Checked on 2026-10-23:
- Help-center search (https://support.lucidtrading.com/en/?q=vps, read through the portal "Lupa Lucid"): `vps` → "0 search results found"; `vpn` → "0 search results found". The searches "remote desktop", "personal device" and "server" return only unrelated articles (Inactivity Policy, HFT, Payouts, Registering as a Business, Our Mission, Other Trading Activities).
- The 61 help-center articles captured the same day contain none of the words vps, vpn, ip address, proxy or virtual private.
- Terms of Use, Privacy Policy, Refund/Chargeback Policy, Risk Disclosure and the homepage FAQ: no hosting or device rule. The Terms ban using "any robot, spider, or other automatic device … to access the Website" (website scraping, not trading) and "Performing trades using an external or slow data feed".
- Reported as `unknown`, not `yes`. The user decides whether to ask the firm.

All from https://support.lucidtrading.com/en/articles/11404728-other-trading-activities unless another source is given.

| Rule | Official text / value | Source |
|---|---|---|
| News trading | "Not allowed on Daily — Trading red folder news on the LucidDaily is a hard breach". "Allowed on Flex, Pro and Direct". The Daily funded window is ±1 minute around USD high-impact news (§3.2). | this page; 15997244 |
| Overnight / weekend | Pro, Flex and Direct: flat by 4:45 PM EST, auto-closed at 4:45 PM EST; holding past that time is not a failure. Live: no swing trading (§5, §6). | 11404729 |
| Scaling in / DCA | "Traders are free to scale into positions or use Dollar Cost Averaging (DCA)". Martingaling is "strongly discouraged". | this page |
| Scalping / flipping | "Genuine scalping is allowed". "Flipping is allowed". | this page |
| Min trade duration | Microscalping flag: >50% of profits from trades ≤5 seconds. | 11404742 |
| Mandatory stop loss | none found | — |
| Inactivity | "Accounts that have not been traded resulting in at least a net profit or loss of 1$ within 30 calendar days will be deemed abandoned and will be permanently deleted". Breached accounts are deleted after 30 days if not reset. | https://support.lucidtrading.com/en/articles/11404632-inactivity-policy |
| Max accounts | 10 evaluation / 5 funded / 5 live per household or family (§7). | 11404617 |
| Hedging penalty | "the associated accounts will be reset to prior days balance … this does not reset the total remaining drawdown". Repeated offenses breach all involved accounts. | 11404734 |
| Volatile contracts (live) | "Certain extremely volatile contracts have reduced position sizing". | 16558826 |

---

## 10. Machine-readable blocks

There are 32 blocks: LucidPro evaluation/funded, LucidFlex evaluation/funded, LucidDaily evaluation/funded, LucidDirect funded and LucidLive, each in 25K, 50K, 100K and 150K. Max-position numbers are as published ("N mini or M micros", ratio 1:10). DLL amounts apply only when DLL is On (Pro, Flex, Daily are checkout options). `kind: soft_pause` follows the firm's "All DLLs at Lucid Trading are considered soft breaches … restricted from placing additional trades until the next trading session".

#### LucidPro — evaluation — 25K
```yaml
firm: lucid-trading
plan: "LucidPro"
phase: evaluation
size_label: "25K"
size_usd: "25000.00"
instruments: [MNQ, NQ]
platforms: [NinjaTrader, Tradovate, TradingView, Quantower, Tradesea, MotiveWave, "Sierra Chart", Jigsaw, Bookmap, ATAS, "R|Trader Pro", MultiCharts]   # "All listed platforms work with all Lucid account types"
drawdown: { amount_usd: "1000.00", type: trailing_eod, stops_trailing_at: "MLL locks once balance exceeds Initial Trail Balance $26,100; Locked MLL Balance $25,100 (initial balance + $100)" }
daily_loss_limit: { amount_usd: null, kind: none }
profit_target_usd: "1250.00"
consistency_rule: { text: null, threshold_pct: null }
max_position: { minis: 2, micros: 20, scaling: null }
min_trading_days: null
trading_day_reset: { time: null, tz_as_published: null }   # not published as a reset; see §6 (session 6:00 PM ET/EST, close-out 4:45 PM EST)
copy_trading_allowed: conditional   # copiers permitted; hedging across accounts/users/firms prohibited; Terms ban same-strategy pooling between unconnected accounts/third parties and trading for others; same-person copy not stated explicitly
automated_trading_allowed: conditional   # automated systems permitted; HFT and microscalping prohibited
automation_rule_text: "Automated trading systems and trade copiers are permitted / All automated activity must comply with Lucid Trading rules / Traders are fully responsible for any software errors, malfunctions, or unintended outcomes"
vps_allowed: unknown                # not published on any official page found; see gaps
personal_device_required: unknown   # not published on any official page found; see gaps
notes: "25K DLL conflict: help eval/DLL articles say None; homepage 25K PRO EVAL card and funded overview show $600. DLL is a checkout option (On/Off) per LucidPro Customization; amount shown applies to DLL On. No consistency rule published for the evaluation. Min trading days not published ('Pass the evaluation in one trading day')."
status: conflict
not_confirmed_fields: [daily_loss_limit, trading_day_reset, copy_trading_allowed, vps_allowed, personal_device_required]
sources:
  - { field: drawdown, url: "https://support.lucidtrading.com/en/articles/12890136-lucidpro-drawdown", accessed: "2026-10-23" }
  - { field: daily_loss_limit, url: "https://support.lucidtrading.com/en/articles/12890122-lucidpro-daily-loss-limit", accessed: "2026-10-23" }
  - { field: profit_target_usd, url: "https://support.lucidtrading.com/en/articles/12890029-lucidpro-evaluation-account", accessed: "2026-10-23" }
  - { field: max_position, url: "https://support.lucidtrading.com/en/articles/12890029-lucidpro-evaluation-account", accessed: "2026-10-23" }
  - { field: daily_loss_limit, url: "https://support.lucidtrading.com/en/articles/16226068-lucidpro-customization", accessed: "2026-10-23" }
  - { field: daily_loss_limit, url: "https://lucidtrading.com/", accessed: "2026-10-23" }
  - { field: copy_trading_allowed, url: "https://support.lucidtrading.com/en/articles/11404728-other-trading-activities", accessed: "2026-10-23" }
  - { field: automated_trading_allowed, url: "https://support.lucidtrading.com/en/articles/11404728-other-trading-activities", accessed: "2026-10-23" }
  - { field: trading_day_reset, url: "https://support.lucidtrading.com/en/articles/11404729-allowed-trading-times", accessed: "2026-10-23" }
  - { field: copy_trading_allowed, url: "https://lucidtrading.com/terms-of-use/", accessed: "2026-10-23" }
  - { field: platforms, url: "https://support.lucidtrading.com/en/articles/11404614-lucid-trading-supported-platforms", accessed: "2026-10-23" }
  - { field: automation_rule_text, url: "https://support.lucidtrading.com/en/articles/11404728-other-trading-activities", accessed: "2026-10-23" }
  - { field: vps_allowed, url: "https://support.lucidtrading.com/en/?q=vps", accessed: "2026-10-23" }   # help-center search "vps": 0 results; no rule found
```

#### LucidPro — funded — 25K
```yaml
firm: lucid-trading
plan: "LucidPro"
phase: funded
size_label: "25K"
size_usd: "25000.00"
instruments: [MNQ, NQ]
platforms: [NinjaTrader, Tradovate, TradingView, Quantower, Tradesea, MotiveWave, "Sierra Chart", Jigsaw, Bookmap, ATAS, "R|Trader Pro", MultiCharts]   # "All listed platforms work with all Lucid account types"
drawdown: { amount_usd: "1000.00", type: trailing_eod, stops_trailing_at: "MLL locks once balance exceeds Initial Trail Balance $26,100; Locked MLL Balance $25,100 (initial balance + $100)" }
daily_loss_limit: { amount_usd: null, kind: none }
profit_target_usd: null
consistency_rule: { text: "Largest Single Day Profit / Account Profit = Consistency Percentage; must be 40% or less to request a payout (35% for accounts purchased/reset before 11/28/2025 3:00 PM EST); resets after each payout", threshold_pct: "40" }
max_position: { minis: 2, micros: 20, scaling: "No scaling plan, access to max contract size immediately" }
min_trading_days: null
trading_day_reset: { time: null, tz_as_published: null }   # not published as a reset; see §6 (session 6:00 PM ET/EST, close-out 4:45 PM EST)
copy_trading_allowed: conditional   # copiers permitted; hedging across accounts/users/firms prohibited; Terms ban same-strategy pooling between unconnected accounts/third parties and trading for others; same-person copy not stated explicitly
automated_trading_allowed: conditional   # automated systems permitted; HFT and microscalping prohibited
automation_rule_text: "Automated trading systems and trade copiers are permitted / All automated activity must comply with Lucid Trading rules / Traders are fully responsible for any software errors, malfunctions, or unintended outcomes"
vps_allowed: unknown                # not published on any official page found; see gaps
personal_device_required: unknown   # not published on any official page found; see gaps
notes: "25K DLL conflict: help eval/DLL articles say None; homepage 25K PRO EVAL card and funded overview show $600. DLL optional (checkout). Fixed DLL until the account closes above the Initial Trail Balance, then LucidScale DLL = 'Highest end-of-day account profit × 60%' (DLL article) / '60% of Peak EOD Balance' (funded overview) - wording differs. Payout buffer = Initial Max Loss Limit + $100; min profit goal per cycle 25K $250 / 50K $500 / 100K $750 / 150K $1,000."
status: conflict
not_confirmed_fields: [daily_loss_limit, trading_day_reset, copy_trading_allowed, vps_allowed, personal_device_required]
sources:
  - { field: drawdown, url: "https://support.lucidtrading.com/en/articles/12890136-lucidpro-drawdown", accessed: "2026-10-23" }
  - { field: daily_loss_limit, url: "https://support.lucidtrading.com/en/articles/12890122-lucidpro-daily-loss-limit", accessed: "2026-10-23" }
  - { field: daily_loss_limit, url: "https://support.lucidtrading.com/en/articles/12890069-lucidpro-funded-account", accessed: "2026-10-23" }
  - { field: consistency_rule, url: "https://support.lucidtrading.com/en/articles/12890109-lucidpro-consistency-percentage", accessed: "2026-10-23" }
  - { field: max_position, url: "https://support.lucidtrading.com/en/articles/12890069-lucidpro-funded-account", accessed: "2026-10-23" }
  - { field: payout, url: "https://support.lucidtrading.com/en/articles/12890092-lucidpro-payouts", accessed: "2026-10-23" }
  - { field: copy_trading_allowed, url: "https://support.lucidtrading.com/en/articles/11404728-other-trading-activities", accessed: "2026-10-23" }
  - { field: automated_trading_allowed, url: "https://support.lucidtrading.com/en/articles/11404728-other-trading-activities", accessed: "2026-10-23" }
  - { field: trading_day_reset, url: "https://support.lucidtrading.com/en/articles/11404729-allowed-trading-times", accessed: "2026-10-23" }
  - { field: copy_trading_allowed, url: "https://lucidtrading.com/terms-of-use/", accessed: "2026-10-23" }
  - { field: platforms, url: "https://support.lucidtrading.com/en/articles/11404614-lucid-trading-supported-platforms", accessed: "2026-10-23" }
  - { field: automation_rule_text, url: "https://support.lucidtrading.com/en/articles/11404728-other-trading-activities", accessed: "2026-10-23" }
  - { field: vps_allowed, url: "https://support.lucidtrading.com/en/?q=vps", accessed: "2026-10-23" }   # help-center search "vps": 0 results; no rule found
```

#### LucidPro — evaluation — 50K
```yaml
firm: lucid-trading
plan: "LucidPro"
phase: evaluation
size_label: "50K"
size_usd: "50000.00"
instruments: [MNQ, NQ]
platforms: [NinjaTrader, Tradovate, TradingView, Quantower, Tradesea, MotiveWave, "Sierra Chart", Jigsaw, Bookmap, ATAS, "R|Trader Pro", MultiCharts]   # "All listed platforms work with all Lucid account types"
drawdown: { amount_usd: "2000.00", type: trailing_eod, stops_trailing_at: "MLL locks once balance exceeds Initial Trail Balance $52,100; Locked MLL Balance $50,100 (initial balance + $100)" }
daily_loss_limit: { amount_usd: "1200.00", kind: soft_pause }
profit_target_usd: "3000.00"
consistency_rule: { text: null, threshold_pct: null }
max_position: { minis: 4, micros: 40, scaling: null }
min_trading_days: null
trading_day_reset: { time: null, tz_as_published: null }   # not published as a reset; see §6 (session 6:00 PM ET/EST, close-out 4:45 PM EST)
copy_trading_allowed: conditional   # copiers permitted; hedging across accounts/users/firms prohibited; Terms ban same-strategy pooling between unconnected accounts/third parties and trading for others; same-person copy not stated explicitly
automated_trading_allowed: conditional   # automated systems permitted; HFT and microscalping prohibited
automation_rule_text: "Automated trading systems and trade copiers are permitted / All automated activity must comply with Lucid Trading rules / Traders are fully responsible for any software errors, malfunctions, or unintended outcomes"
vps_allowed: unknown                # not published on any official page found; see gaps
personal_device_required: unknown   # not published on any official page found; see gaps
notes: "DLL is a checkout option (On/Off) per LucidPro Customization; amount shown applies to DLL On. No consistency rule published for the evaluation. Min trading days not published ('Pass the evaluation in one trading day')."
status: partial
not_confirmed_fields: [trading_day_reset, copy_trading_allowed, vps_allowed, personal_device_required]
sources:
  - { field: drawdown, url: "https://support.lucidtrading.com/en/articles/12890136-lucidpro-drawdown", accessed: "2026-10-23" }
  - { field: daily_loss_limit, url: "https://support.lucidtrading.com/en/articles/12890122-lucidpro-daily-loss-limit", accessed: "2026-10-23" }
  - { field: profit_target_usd, url: "https://support.lucidtrading.com/en/articles/12890029-lucidpro-evaluation-account", accessed: "2026-10-23" }
  - { field: max_position, url: "https://support.lucidtrading.com/en/articles/12890029-lucidpro-evaluation-account", accessed: "2026-10-23" }
  - { field: daily_loss_limit, url: "https://support.lucidtrading.com/en/articles/16226068-lucidpro-customization", accessed: "2026-10-23" }
  - { field: daily_loss_limit, url: "https://lucidtrading.com/", accessed: "2026-10-23" }
  - { field: copy_trading_allowed, url: "https://support.lucidtrading.com/en/articles/11404728-other-trading-activities", accessed: "2026-10-23" }
  - { field: automated_trading_allowed, url: "https://support.lucidtrading.com/en/articles/11404728-other-trading-activities", accessed: "2026-10-23" }
  - { field: trading_day_reset, url: "https://support.lucidtrading.com/en/articles/11404729-allowed-trading-times", accessed: "2026-10-23" }
  - { field: copy_trading_allowed, url: "https://lucidtrading.com/terms-of-use/", accessed: "2026-10-23" }
  - { field: platforms, url: "https://support.lucidtrading.com/en/articles/11404614-lucid-trading-supported-platforms", accessed: "2026-10-23" }
  - { field: automation_rule_text, url: "https://support.lucidtrading.com/en/articles/11404728-other-trading-activities", accessed: "2026-10-23" }
  - { field: vps_allowed, url: "https://support.lucidtrading.com/en/?q=vps", accessed: "2026-10-23" }   # help-center search "vps": 0 results; no rule found
```

#### LucidPro — funded — 50K
```yaml
firm: lucid-trading
plan: "LucidPro"
phase: funded
size_label: "50K"
size_usd: "50000.00"
instruments: [MNQ, NQ]
platforms: [NinjaTrader, Tradovate, TradingView, Quantower, Tradesea, MotiveWave, "Sierra Chart", Jigsaw, Bookmap, ATAS, "R|Trader Pro", MultiCharts]   # "All listed platforms work with all Lucid account types"
drawdown: { amount_usd: "2000.00", type: trailing_eod, stops_trailing_at: "MLL locks once balance exceeds Initial Trail Balance $52,100; Locked MLL Balance $50,100 (initial balance + $100)" }
daily_loss_limit: { amount_usd: "1200.00", kind: soft_pause }
profit_target_usd: null
consistency_rule: { text: "Largest Single Day Profit / Account Profit = Consistency Percentage; must be 40% or less to request a payout (35% for accounts purchased/reset before 11/28/2025 3:00 PM EST); resets after each payout", threshold_pct: "40" }
max_position: { minis: 4, micros: 40, scaling: "No scaling plan, access to max contract size immediately" }
min_trading_days: null
trading_day_reset: { time: null, tz_as_published: null }   # not published as a reset; see §6 (session 6:00 PM ET/EST, close-out 4:45 PM EST)
copy_trading_allowed: conditional   # copiers permitted; hedging across accounts/users/firms prohibited; Terms ban same-strategy pooling between unconnected accounts/third parties and trading for others; same-person copy not stated explicitly
automated_trading_allowed: conditional   # automated systems permitted; HFT and microscalping prohibited
automation_rule_text: "Automated trading systems and trade copiers are permitted / All automated activity must comply with Lucid Trading rules / Traders are fully responsible for any software errors, malfunctions, or unintended outcomes"
vps_allowed: unknown                # not published on any official page found; see gaps
personal_device_required: unknown   # not published on any official page found; see gaps
notes: "DLL optional (checkout). Fixed DLL until the account closes above the Initial Trail Balance, then LucidScale DLL = 'Highest end-of-day account profit × 60%' (DLL article) / '60% of Peak EOD Balance' (funded overview) - wording differs. Payout buffer = Initial Max Loss Limit + $100; min profit goal per cycle 25K $250 / 50K $500 / 100K $750 / 150K $1,000."
status: partial
not_confirmed_fields: [daily_loss_limit, trading_day_reset, copy_trading_allowed, vps_allowed, personal_device_required]
sources:
  - { field: drawdown, url: "https://support.lucidtrading.com/en/articles/12890136-lucidpro-drawdown", accessed: "2026-10-23" }
  - { field: daily_loss_limit, url: "https://support.lucidtrading.com/en/articles/12890122-lucidpro-daily-loss-limit", accessed: "2026-10-23" }
  - { field: daily_loss_limit, url: "https://support.lucidtrading.com/en/articles/12890069-lucidpro-funded-account", accessed: "2026-10-23" }
  - { field: consistency_rule, url: "https://support.lucidtrading.com/en/articles/12890109-lucidpro-consistency-percentage", accessed: "2026-10-23" }
  - { field: max_position, url: "https://support.lucidtrading.com/en/articles/12890069-lucidpro-funded-account", accessed: "2026-10-23" }
  - { field: payout, url: "https://support.lucidtrading.com/en/articles/12890092-lucidpro-payouts", accessed: "2026-10-23" }
  - { field: copy_trading_allowed, url: "https://support.lucidtrading.com/en/articles/11404728-other-trading-activities", accessed: "2026-10-23" }
  - { field: automated_trading_allowed, url: "https://support.lucidtrading.com/en/articles/11404728-other-trading-activities", accessed: "2026-10-23" }
  - { field: trading_day_reset, url: "https://support.lucidtrading.com/en/articles/11404729-allowed-trading-times", accessed: "2026-10-23" }
  - { field: copy_trading_allowed, url: "https://lucidtrading.com/terms-of-use/", accessed: "2026-10-23" }
  - { field: platforms, url: "https://support.lucidtrading.com/en/articles/11404614-lucid-trading-supported-platforms", accessed: "2026-10-23" }
  - { field: automation_rule_text, url: "https://support.lucidtrading.com/en/articles/11404728-other-trading-activities", accessed: "2026-10-23" }
  - { field: vps_allowed, url: "https://support.lucidtrading.com/en/?q=vps", accessed: "2026-10-23" }   # help-center search "vps": 0 results; no rule found
```

#### LucidPro — evaluation — 100K
```yaml
firm: lucid-trading
plan: "LucidPro"
phase: evaluation
size_label: "100K"
size_usd: "100000.00"
instruments: [MNQ, NQ]
platforms: [NinjaTrader, Tradovate, TradingView, Quantower, Tradesea, MotiveWave, "Sierra Chart", Jigsaw, Bookmap, ATAS, "R|Trader Pro", MultiCharts]   # "All listed platforms work with all Lucid account types"
drawdown: { amount_usd: "3000.00", type: trailing_eod, stops_trailing_at: "MLL locks once balance exceeds Initial Trail Balance $103,100; Locked MLL Balance $100,100 (initial balance + $100)" }
daily_loss_limit: { amount_usd: "1800.00", kind: soft_pause }
profit_target_usd: "6000.00"
consistency_rule: { text: null, threshold_pct: null }
max_position: { minis: 6, micros: 60, scaling: null }
min_trading_days: null
trading_day_reset: { time: null, tz_as_published: null }   # not published as a reset; see §6 (session 6:00 PM ET/EST, close-out 4:45 PM EST)
copy_trading_allowed: conditional   # copiers permitted; hedging across accounts/users/firms prohibited; Terms ban same-strategy pooling between unconnected accounts/third parties and trading for others; same-person copy not stated explicitly
automated_trading_allowed: conditional   # automated systems permitted; HFT and microscalping prohibited
automation_rule_text: "Automated trading systems and trade copiers are permitted / All automated activity must comply with Lucid Trading rules / Traders are fully responsible for any software errors, malfunctions, or unintended outcomes"
vps_allowed: unknown                # not published on any official page found; see gaps
personal_device_required: unknown   # not published on any official page found; see gaps
notes: "DLL is a checkout option (On/Off) per LucidPro Customization; amount shown applies to DLL On. No consistency rule published for the evaluation. Min trading days not published ('Pass the evaluation in one trading day')."
status: partial
not_confirmed_fields: [trading_day_reset, copy_trading_allowed, vps_allowed, personal_device_required]
sources:
  - { field: drawdown, url: "https://support.lucidtrading.com/en/articles/12890136-lucidpro-drawdown", accessed: "2026-10-23" }
  - { field: daily_loss_limit, url: "https://support.lucidtrading.com/en/articles/12890122-lucidpro-daily-loss-limit", accessed: "2026-10-23" }
  - { field: profit_target_usd, url: "https://support.lucidtrading.com/en/articles/12890029-lucidpro-evaluation-account", accessed: "2026-10-23" }
  - { field: max_position, url: "https://support.lucidtrading.com/en/articles/12890029-lucidpro-evaluation-account", accessed: "2026-10-23" }
  - { field: daily_loss_limit, url: "https://support.lucidtrading.com/en/articles/16226068-lucidpro-customization", accessed: "2026-10-23" }
  - { field: daily_loss_limit, url: "https://lucidtrading.com/", accessed: "2026-10-23" }
  - { field: copy_trading_allowed, url: "https://support.lucidtrading.com/en/articles/11404728-other-trading-activities", accessed: "2026-10-23" }
  - { field: automated_trading_allowed, url: "https://support.lucidtrading.com/en/articles/11404728-other-trading-activities", accessed: "2026-10-23" }
  - { field: trading_day_reset, url: "https://support.lucidtrading.com/en/articles/11404729-allowed-trading-times", accessed: "2026-10-23" }
  - { field: copy_trading_allowed, url: "https://lucidtrading.com/terms-of-use/", accessed: "2026-10-23" }
  - { field: platforms, url: "https://support.lucidtrading.com/en/articles/11404614-lucid-trading-supported-platforms", accessed: "2026-10-23" }
  - { field: automation_rule_text, url: "https://support.lucidtrading.com/en/articles/11404728-other-trading-activities", accessed: "2026-10-23" }
  - { field: vps_allowed, url: "https://support.lucidtrading.com/en/?q=vps", accessed: "2026-10-23" }   # help-center search "vps": 0 results; no rule found
```

#### LucidPro — funded — 100K
```yaml
firm: lucid-trading
plan: "LucidPro"
phase: funded
size_label: "100K"
size_usd: "100000.00"
instruments: [MNQ, NQ]
platforms: [NinjaTrader, Tradovate, TradingView, Quantower, Tradesea, MotiveWave, "Sierra Chart", Jigsaw, Bookmap, ATAS, "R|Trader Pro", MultiCharts]   # "All listed platforms work with all Lucid account types"
drawdown: { amount_usd: "3000.00", type: trailing_eod, stops_trailing_at: "MLL locks once balance exceeds Initial Trail Balance $103,100; Locked MLL Balance $100,100 (initial balance + $100)" }
daily_loss_limit: { amount_usd: "1800.00", kind: soft_pause }
profit_target_usd: null
consistency_rule: { text: "Largest Single Day Profit / Account Profit = Consistency Percentage; must be 40% or less to request a payout (35% for accounts purchased/reset before 11/28/2025 3:00 PM EST); resets after each payout", threshold_pct: "40" }
max_position: { minis: 6, micros: 60, scaling: "No scaling plan, access to max contract size immediately" }
min_trading_days: null
trading_day_reset: { time: null, tz_as_published: null }   # not published as a reset; see §6 (session 6:00 PM ET/EST, close-out 4:45 PM EST)
copy_trading_allowed: conditional   # copiers permitted; hedging across accounts/users/firms prohibited; Terms ban same-strategy pooling between unconnected accounts/third parties and trading for others; same-person copy not stated explicitly
automated_trading_allowed: conditional   # automated systems permitted; HFT and microscalping prohibited
automation_rule_text: "Automated trading systems and trade copiers are permitted / All automated activity must comply with Lucid Trading rules / Traders are fully responsible for any software errors, malfunctions, or unintended outcomes"
vps_allowed: unknown                # not published on any official page found; see gaps
personal_device_required: unknown   # not published on any official page found; see gaps
notes: "DLL optional (checkout). Fixed DLL until the account closes above the Initial Trail Balance, then LucidScale DLL = 'Highest end-of-day account profit × 60%' (DLL article) / '60% of Peak EOD Balance' (funded overview) - wording differs. Payout buffer = Initial Max Loss Limit + $100; min profit goal per cycle 25K $250 / 50K $500 / 100K $750 / 150K $1,000."
status: partial
not_confirmed_fields: [daily_loss_limit, trading_day_reset, copy_trading_allowed, vps_allowed, personal_device_required]
sources:
  - { field: drawdown, url: "https://support.lucidtrading.com/en/articles/12890136-lucidpro-drawdown", accessed: "2026-10-23" }
  - { field: daily_loss_limit, url: "https://support.lucidtrading.com/en/articles/12890122-lucidpro-daily-loss-limit", accessed: "2026-10-23" }
  - { field: daily_loss_limit, url: "https://support.lucidtrading.com/en/articles/12890069-lucidpro-funded-account", accessed: "2026-10-23" }
  - { field: consistency_rule, url: "https://support.lucidtrading.com/en/articles/12890109-lucidpro-consistency-percentage", accessed: "2026-10-23" }
  - { field: max_position, url: "https://support.lucidtrading.com/en/articles/12890069-lucidpro-funded-account", accessed: "2026-10-23" }
  - { field: payout, url: "https://support.lucidtrading.com/en/articles/12890092-lucidpro-payouts", accessed: "2026-10-23" }
  - { field: copy_trading_allowed, url: "https://support.lucidtrading.com/en/articles/11404728-other-trading-activities", accessed: "2026-10-23" }
  - { field: automated_trading_allowed, url: "https://support.lucidtrading.com/en/articles/11404728-other-trading-activities", accessed: "2026-10-23" }
  - { field: trading_day_reset, url: "https://support.lucidtrading.com/en/articles/11404729-allowed-trading-times", accessed: "2026-10-23" }
  - { field: copy_trading_allowed, url: "https://lucidtrading.com/terms-of-use/", accessed: "2026-10-23" }
  - { field: platforms, url: "https://support.lucidtrading.com/en/articles/11404614-lucid-trading-supported-platforms", accessed: "2026-10-23" }
  - { field: automation_rule_text, url: "https://support.lucidtrading.com/en/articles/11404728-other-trading-activities", accessed: "2026-10-23" }
  - { field: vps_allowed, url: "https://support.lucidtrading.com/en/?q=vps", accessed: "2026-10-23" }   # help-center search "vps": 0 results; no rule found
```

#### LucidPro — evaluation — 150K
```yaml
firm: lucid-trading
plan: "LucidPro"
phase: evaluation
size_label: "150K"
size_usd: "150000.00"
instruments: [MNQ, NQ]
platforms: [NinjaTrader, Tradovate, TradingView, Quantower, Tradesea, MotiveWave, "Sierra Chart", Jigsaw, Bookmap, ATAS, "R|Trader Pro", MultiCharts]   # "All listed platforms work with all Lucid account types"
drawdown: { amount_usd: "4500.00", type: trailing_eod, stops_trailing_at: "MLL locks once balance exceeds Initial Trail Balance $154,600; Locked MLL Balance $150,100 (initial balance + $100)" }
daily_loss_limit: { amount_usd: "2700.00", kind: soft_pause }
profit_target_usd: "9000.00"
consistency_rule: { text: null, threshold_pct: null }
max_position: { minis: 10, micros: 100, scaling: null }
min_trading_days: null
trading_day_reset: { time: null, tz_as_published: null }   # not published as a reset; see §6 (session 6:00 PM ET/EST, close-out 4:45 PM EST)
copy_trading_allowed: conditional   # copiers permitted; hedging across accounts/users/firms prohibited; Terms ban same-strategy pooling between unconnected accounts/third parties and trading for others; same-person copy not stated explicitly
automated_trading_allowed: conditional   # automated systems permitted; HFT and microscalping prohibited
automation_rule_text: "Automated trading systems and trade copiers are permitted / All automated activity must comply with Lucid Trading rules / Traders are fully responsible for any software errors, malfunctions, or unintended outcomes"
vps_allowed: unknown                # not published on any official page found; see gaps
personal_device_required: unknown   # not published on any official page found; see gaps
notes: "DLL is a checkout option (On/Off) per LucidPro Customization; amount shown applies to DLL On. No consistency rule published for the evaluation. Min trading days not published ('Pass the evaluation in one trading day')."
status: partial
not_confirmed_fields: [trading_day_reset, copy_trading_allowed, vps_allowed, personal_device_required]
sources:
  - { field: drawdown, url: "https://support.lucidtrading.com/en/articles/12890136-lucidpro-drawdown", accessed: "2026-10-23" }
  - { field: daily_loss_limit, url: "https://support.lucidtrading.com/en/articles/12890122-lucidpro-daily-loss-limit", accessed: "2026-10-23" }
  - { field: profit_target_usd, url: "https://support.lucidtrading.com/en/articles/12890029-lucidpro-evaluation-account", accessed: "2026-10-23" }
  - { field: max_position, url: "https://support.lucidtrading.com/en/articles/12890029-lucidpro-evaluation-account", accessed: "2026-10-23" }
  - { field: daily_loss_limit, url: "https://support.lucidtrading.com/en/articles/16226068-lucidpro-customization", accessed: "2026-10-23" }
  - { field: daily_loss_limit, url: "https://lucidtrading.com/", accessed: "2026-10-23" }
  - { field: copy_trading_allowed, url: "https://support.lucidtrading.com/en/articles/11404728-other-trading-activities", accessed: "2026-10-23" }
  - { field: automated_trading_allowed, url: "https://support.lucidtrading.com/en/articles/11404728-other-trading-activities", accessed: "2026-10-23" }
  - { field: trading_day_reset, url: "https://support.lucidtrading.com/en/articles/11404729-allowed-trading-times", accessed: "2026-10-23" }
  - { field: copy_trading_allowed, url: "https://lucidtrading.com/terms-of-use/", accessed: "2026-10-23" }
  - { field: platforms, url: "https://support.lucidtrading.com/en/articles/11404614-lucid-trading-supported-platforms", accessed: "2026-10-23" }
  - { field: automation_rule_text, url: "https://support.lucidtrading.com/en/articles/11404728-other-trading-activities", accessed: "2026-10-23" }
  - { field: vps_allowed, url: "https://support.lucidtrading.com/en/?q=vps", accessed: "2026-10-23" }   # help-center search "vps": 0 results; no rule found
```

#### LucidPro — funded — 150K
```yaml
firm: lucid-trading
plan: "LucidPro"
phase: funded
size_label: "150K"
size_usd: "150000.00"
instruments: [MNQ, NQ]
platforms: [NinjaTrader, Tradovate, TradingView, Quantower, Tradesea, MotiveWave, "Sierra Chart", Jigsaw, Bookmap, ATAS, "R|Trader Pro", MultiCharts]   # "All listed platforms work with all Lucid account types"
drawdown: { amount_usd: "4500.00", type: trailing_eod, stops_trailing_at: "MLL locks once balance exceeds Initial Trail Balance $154,600; Locked MLL Balance $150,100 (initial balance + $100)" }
daily_loss_limit: { amount_usd: "2700.00", kind: soft_pause }
profit_target_usd: null
consistency_rule: { text: "Largest Single Day Profit / Account Profit = Consistency Percentage; must be 40% or less to request a payout (35% for accounts purchased/reset before 11/28/2025 3:00 PM EST); resets after each payout", threshold_pct: "40" }
max_position: { minis: 10, micros: 100, scaling: "No scaling plan, access to max contract size immediately" }
min_trading_days: null
trading_day_reset: { time: null, tz_as_published: null }   # not published as a reset; see §6 (session 6:00 PM ET/EST, close-out 4:45 PM EST)
copy_trading_allowed: conditional   # copiers permitted; hedging across accounts/users/firms prohibited; Terms ban same-strategy pooling between unconnected accounts/third parties and trading for others; same-person copy not stated explicitly
automated_trading_allowed: conditional   # automated systems permitted; HFT and microscalping prohibited
automation_rule_text: "Automated trading systems and trade copiers are permitted / All automated activity must comply with Lucid Trading rules / Traders are fully responsible for any software errors, malfunctions, or unintended outcomes"
vps_allowed: unknown                # not published on any official page found; see gaps
personal_device_required: unknown   # not published on any official page found; see gaps
notes: "DLL optional (checkout). Fixed DLL until the account closes above the Initial Trail Balance, then LucidScale DLL = 'Highest end-of-day account profit × 60%' (DLL article) / '60% of Peak EOD Balance' (funded overview) - wording differs. Payout buffer = Initial Max Loss Limit + $100; min profit goal per cycle 25K $250 / 50K $500 / 100K $750 / 150K $1,000."
status: partial
not_confirmed_fields: [daily_loss_limit, trading_day_reset, copy_trading_allowed, vps_allowed, personal_device_required]
sources:
  - { field: drawdown, url: "https://support.lucidtrading.com/en/articles/12890136-lucidpro-drawdown", accessed: "2026-10-23" }
  - { field: daily_loss_limit, url: "https://support.lucidtrading.com/en/articles/12890122-lucidpro-daily-loss-limit", accessed: "2026-10-23" }
  - { field: daily_loss_limit, url: "https://support.lucidtrading.com/en/articles/12890069-lucidpro-funded-account", accessed: "2026-10-23" }
  - { field: consistency_rule, url: "https://support.lucidtrading.com/en/articles/12890109-lucidpro-consistency-percentage", accessed: "2026-10-23" }
  - { field: max_position, url: "https://support.lucidtrading.com/en/articles/12890069-lucidpro-funded-account", accessed: "2026-10-23" }
  - { field: payout, url: "https://support.lucidtrading.com/en/articles/12890092-lucidpro-payouts", accessed: "2026-10-23" }
  - { field: copy_trading_allowed, url: "https://support.lucidtrading.com/en/articles/11404728-other-trading-activities", accessed: "2026-10-23" }
  - { field: automated_trading_allowed, url: "https://support.lucidtrading.com/en/articles/11404728-other-trading-activities", accessed: "2026-10-23" }
  - { field: trading_day_reset, url: "https://support.lucidtrading.com/en/articles/11404729-allowed-trading-times", accessed: "2026-10-23" }
  - { field: copy_trading_allowed, url: "https://lucidtrading.com/terms-of-use/", accessed: "2026-10-23" }
  - { field: platforms, url: "https://support.lucidtrading.com/en/articles/11404614-lucid-trading-supported-platforms", accessed: "2026-10-23" }
  - { field: automation_rule_text, url: "https://support.lucidtrading.com/en/articles/11404728-other-trading-activities", accessed: "2026-10-23" }
  - { field: vps_allowed, url: "https://support.lucidtrading.com/en/?q=vps", accessed: "2026-10-23" }   # help-center search "vps": 0 results; no rule found
```

#### LucidFlex — evaluation — 25K
```yaml
firm: lucid-trading
plan: "LucidFlex"
phase: evaluation
size_label: "25K"
size_usd: "25000.00"
instruments: [MNQ, NQ]
platforms: [NinjaTrader, Tradovate, TradingView, Quantower, Tradesea, MotiveWave, "Sierra Chart", Jigsaw, Bookmap, ATAS, "R|Trader Pro", MultiCharts]   # "All listed platforms work with all Lucid account types"
drawdown: { amount_usd: "1000.00", type: trailing_eod, stops_trailing_at: "MLL locks once balance exceeds Initial Trail Balance $26,100; Locked MLL Balance $25,100 (initial balance + $100)" }
daily_loss_limit: { amount_usd: "600.00", kind: soft_pause }
profit_target_usd: "1250.00"
consistency_rule: { text: "Largest Single Day Profit / Account Profit = Consistency Percentage; 40% or less to upgrade (50% with add-on)", threshold_pct: "40" }
max_position: { minis: 2, micros: 20, scaling: "No scaling plan in the evaluation phase" }
min_trading_days: null
trading_day_reset: { time: null, tz_as_published: null }   # not published as a reset; see §6 (session 6:00 PM ET/EST, close-out 4:45 PM EST)
copy_trading_allowed: conditional   # copiers permitted; hedging across accounts/users/firms prohibited; Terms ban same-strategy pooling between unconnected accounts/third parties and trading for others; same-person copy not stated explicitly
automated_trading_allowed: conditional   # automated systems permitted; HFT and microscalping prohibited
automation_rule_text: "Automated trading systems and trade copiers are permitted / All automated activity must comply with Lucid Trading rules / Traders are fully responsible for any software errors, malfunctions, or unintended outcomes"
vps_allowed: unknown                # not published on any official page found; see gaps
personal_device_required: unknown   # not published on any official page found; see gaps
notes: "DLL On/Off chosen at checkout; DLL amount is from the homepage only (help center gives none); amount applies to DLL On. Consistency conflict: help center 40% standard / 50% add-on vs homepage eval card 50%."
status: conflict
not_confirmed_fields: [consistency_rule, trading_day_reset, copy_trading_allowed, vps_allowed, personal_device_required]
sources:
  - { field: drawdown, url: "https://support.lucidtrading.com/en/articles/12945815-lucidflex-drawdown", accessed: "2026-10-23" }
  - { field: profit_target_usd, url: "https://support.lucidtrading.com/en/articles/12945790-lucidflex-evaluation-account", accessed: "2026-10-23" }
  - { field: consistency_rule, url: "https://support.lucidtrading.com/en/articles/12945805-lucidflex-consistency-percentage", accessed: "2026-10-23" }
  - { field: daily_loss_limit, url: "https://support.lucidtrading.com/en/articles/16226050-lucidflex-customization", accessed: "2026-10-23" }
  - { field: max_position, url: "https://support.lucidtrading.com/en/articles/12945790-lucidflex-evaluation-account", accessed: "2026-10-23" }
  - { field: daily_loss_limit, url: "https://lucidtrading.com/", accessed: "2026-10-23" }
  - { field: copy_trading_allowed, url: "https://support.lucidtrading.com/en/articles/11404728-other-trading-activities", accessed: "2026-10-23" }
  - { field: automated_trading_allowed, url: "https://support.lucidtrading.com/en/articles/11404728-other-trading-activities", accessed: "2026-10-23" }
  - { field: trading_day_reset, url: "https://support.lucidtrading.com/en/articles/11404729-allowed-trading-times", accessed: "2026-10-23" }
  - { field: copy_trading_allowed, url: "https://lucidtrading.com/terms-of-use/", accessed: "2026-10-23" }
  - { field: platforms, url: "https://support.lucidtrading.com/en/articles/11404614-lucid-trading-supported-platforms", accessed: "2026-10-23" }
  - { field: automation_rule_text, url: "https://support.lucidtrading.com/en/articles/11404728-other-trading-activities", accessed: "2026-10-23" }
  - { field: vps_allowed, url: "https://support.lucidtrading.com/en/?q=vps", accessed: "2026-10-23" }   # help-center search "vps": 0 results; no rule found
```

#### LucidFlex — funded — 25K
```yaml
firm: lucid-trading
plan: "LucidFlex"
phase: funded
size_label: "25K"
size_usd: "25000.00"
instruments: [MNQ, NQ]
platforms: [NinjaTrader, Tradovate, TradingView, Quantower, Tradesea, MotiveWave, "Sierra Chart", Jigsaw, Bookmap, ATAS, "R|Trader Pro", MultiCharts]   # "All listed platforms work with all Lucid account types"
drawdown: { amount_usd: "1000.00", type: trailing_eod, stops_trailing_at: "MLL locks once balance exceeds Initial Trail Balance $26,100; Locked MLL Balance $25,100 (initial balance + $100); on payout request MLL adjusts to the Locked MLL Balance" }
daily_loss_limit: { amount_usd: "600.00", kind: soft_pause }
profit_target_usd: null
consistency_rule: { text: "There is no Consistency Percentage on LucidFlex funded accounts", threshold_pct: null }
max_position: { minis: 2, micros: 20, scaling: "$0-$999: 1 mini/10 micros; $1,000-$1,999: 2/20 (end-of-session update)" }
min_trading_days: null
trading_day_reset: { time: null, tz_as_published: null }   # not published as a reset; see §6 (session 6:00 PM ET/EST, close-out 4:45 PM EST)
copy_trading_allowed: conditional   # copiers permitted; hedging across accounts/users/firms prohibited; Terms ban same-strategy pooling between unconnected accounts/third parties and trading for others; same-person copy not stated explicitly
automated_trading_allowed: conditional   # automated systems permitted; HFT and microscalping prohibited
automation_rule_text: "Automated trading systems and trade copiers are permitted / All automated activity must comply with Lucid Trading rules / Traders are fully responsible for any software errors, malfunctions, or unintended outcomes"
vps_allowed: unknown                # not published on any official page found; see gaps
personal_device_required: unknown   # not published on any official page found; see gaps
notes: "DLL 'Optional' (checkout); amount = homepage 'DLL (Below Initial Trail)' (homepage only); DLL above Initial Trail not published (unverified). Payout: minimum profit of $100 on 5 separate days per cycle + positive net profit; no buffer; up to 5 payouts then moved live."
status: partial
not_confirmed_fields: [daily_loss_limit, trading_day_reset, copy_trading_allowed, vps_allowed, personal_device_required]
sources:
  - { field: drawdown, url: "https://support.lucidtrading.com/en/articles/12945815-lucidflex-drawdown", accessed: "2026-10-23" }
  - { field: max_position, url: "https://support.lucidtrading.com/en/articles/12945808-lucidflex-scaling-plan", accessed: "2026-10-23" }
  - { field: max_position, url: "https://support.lucidtrading.com/en/articles/12945795-lucidflex-funded-account", accessed: "2026-10-23" }
  - { field: consistency_rule, url: "https://support.lucidtrading.com/en/articles/12945795-lucidflex-funded-account", accessed: "2026-10-23" }
  - { field: payout, url: "https://support.lucidtrading.com/en/articles/12945796-lucidflex-payouts", accessed: "2026-10-23" }
  - { field: daily_loss_limit, url: "https://lucidtrading.com/", accessed: "2026-10-23" }
  - { field: copy_trading_allowed, url: "https://support.lucidtrading.com/en/articles/11404728-other-trading-activities", accessed: "2026-10-23" }
  - { field: automated_trading_allowed, url: "https://support.lucidtrading.com/en/articles/11404728-other-trading-activities", accessed: "2026-10-23" }
  - { field: trading_day_reset, url: "https://support.lucidtrading.com/en/articles/11404729-allowed-trading-times", accessed: "2026-10-23" }
  - { field: copy_trading_allowed, url: "https://lucidtrading.com/terms-of-use/", accessed: "2026-10-23" }
  - { field: platforms, url: "https://support.lucidtrading.com/en/articles/11404614-lucid-trading-supported-platforms", accessed: "2026-10-23" }
  - { field: automation_rule_text, url: "https://support.lucidtrading.com/en/articles/11404728-other-trading-activities", accessed: "2026-10-23" }
  - { field: vps_allowed, url: "https://support.lucidtrading.com/en/?q=vps", accessed: "2026-10-23" }   # help-center search "vps": 0 results; no rule found
```

#### LucidFlex — evaluation — 50K
```yaml
firm: lucid-trading
plan: "LucidFlex"
phase: evaluation
size_label: "50K"
size_usd: "50000.00"
instruments: [MNQ, NQ]
platforms: [NinjaTrader, Tradovate, TradingView, Quantower, Tradesea, MotiveWave, "Sierra Chart", Jigsaw, Bookmap, ATAS, "R|Trader Pro", MultiCharts]   # "All listed platforms work with all Lucid account types"
drawdown: { amount_usd: "2000.00", type: trailing_eod, stops_trailing_at: "MLL locks once balance exceeds Initial Trail Balance $52,100; Locked MLL Balance $50,100 (initial balance + $100)" }
daily_loss_limit: { amount_usd: "1200.00", kind: soft_pause }
profit_target_usd: "3000.00"
consistency_rule: { text: "Largest Single Day Profit / Account Profit = Consistency Percentage; 40% or less to upgrade (50% with add-on)", threshold_pct: "40" }
max_position: { minis: 4, micros: 40, scaling: "No scaling plan in the evaluation phase" }
min_trading_days: null
trading_day_reset: { time: null, tz_as_published: null }   # not published as a reset; see §6 (session 6:00 PM ET/EST, close-out 4:45 PM EST)
copy_trading_allowed: conditional   # copiers permitted; hedging across accounts/users/firms prohibited; Terms ban same-strategy pooling between unconnected accounts/third parties and trading for others; same-person copy not stated explicitly
automated_trading_allowed: conditional   # automated systems permitted; HFT and microscalping prohibited
automation_rule_text: "Automated trading systems and trade copiers are permitted / All automated activity must comply with Lucid Trading rules / Traders are fully responsible for any software errors, malfunctions, or unintended outcomes"
vps_allowed: unknown                # not published on any official page found; see gaps
personal_device_required: unknown   # not published on any official page found; see gaps
notes: "DLL On/Off chosen at checkout; DLL amount is from the homepage only (help center gives none); amount applies to DLL On. Consistency conflict: help center 40% standard / 50% add-on vs homepage eval card 50%."
status: conflict
not_confirmed_fields: [consistency_rule, trading_day_reset, copy_trading_allowed, vps_allowed, personal_device_required]
sources:
  - { field: drawdown, url: "https://support.lucidtrading.com/en/articles/12945815-lucidflex-drawdown", accessed: "2026-10-23" }
  - { field: profit_target_usd, url: "https://support.lucidtrading.com/en/articles/12945790-lucidflex-evaluation-account", accessed: "2026-10-23" }
  - { field: consistency_rule, url: "https://support.lucidtrading.com/en/articles/12945805-lucidflex-consistency-percentage", accessed: "2026-10-23" }
  - { field: daily_loss_limit, url: "https://support.lucidtrading.com/en/articles/16226050-lucidflex-customization", accessed: "2026-10-23" }
  - { field: max_position, url: "https://support.lucidtrading.com/en/articles/12945790-lucidflex-evaluation-account", accessed: "2026-10-23" }
  - { field: daily_loss_limit, url: "https://lucidtrading.com/", accessed: "2026-10-23" }
  - { field: copy_trading_allowed, url: "https://support.lucidtrading.com/en/articles/11404728-other-trading-activities", accessed: "2026-10-23" }
  - { field: automated_trading_allowed, url: "https://support.lucidtrading.com/en/articles/11404728-other-trading-activities", accessed: "2026-10-23" }
  - { field: trading_day_reset, url: "https://support.lucidtrading.com/en/articles/11404729-allowed-trading-times", accessed: "2026-10-23" }
  - { field: copy_trading_allowed, url: "https://lucidtrading.com/terms-of-use/", accessed: "2026-10-23" }
  - { field: platforms, url: "https://support.lucidtrading.com/en/articles/11404614-lucid-trading-supported-platforms", accessed: "2026-10-23" }
  - { field: automation_rule_text, url: "https://support.lucidtrading.com/en/articles/11404728-other-trading-activities", accessed: "2026-10-23" }
  - { field: vps_allowed, url: "https://support.lucidtrading.com/en/?q=vps", accessed: "2026-10-23" }   # help-center search "vps": 0 results; no rule found
```

#### LucidFlex — funded — 50K
```yaml
firm: lucid-trading
plan: "LucidFlex"
phase: funded
size_label: "50K"
size_usd: "50000.00"
instruments: [MNQ, NQ]
platforms: [NinjaTrader, Tradovate, TradingView, Quantower, Tradesea, MotiveWave, "Sierra Chart", Jigsaw, Bookmap, ATAS, "R|Trader Pro", MultiCharts]   # "All listed platforms work with all Lucid account types"
drawdown: { amount_usd: "2000.00", type: trailing_eod, stops_trailing_at: "MLL locks once balance exceeds Initial Trail Balance $52,100; Locked MLL Balance $50,100 (initial balance + $100); on payout request MLL adjusts to the Locked MLL Balance" }
daily_loss_limit: { amount_usd: "1200.00", kind: soft_pause }
profit_target_usd: null
consistency_rule: { text: "There is no Consistency Percentage on LucidFlex funded accounts", threshold_pct: null }
max_position: { minis: 4, micros: 40, scaling: "$0-$999: 2/20; $1,000-$1,999: 3/30; $2,000-$2,999: 4/40 (end-of-session update)" }
min_trading_days: null
trading_day_reset: { time: null, tz_as_published: null }   # not published as a reset; see §6 (session 6:00 PM ET/EST, close-out 4:45 PM EST)
copy_trading_allowed: conditional   # copiers permitted; hedging across accounts/users/firms prohibited; Terms ban same-strategy pooling between unconnected accounts/third parties and trading for others; same-person copy not stated explicitly
automated_trading_allowed: conditional   # automated systems permitted; HFT and microscalping prohibited
automation_rule_text: "Automated trading systems and trade copiers are permitted / All automated activity must comply with Lucid Trading rules / Traders are fully responsible for any software errors, malfunctions, or unintended outcomes"
vps_allowed: unknown                # not published on any official page found; see gaps
personal_device_required: unknown   # not published on any official page found; see gaps
notes: "DLL 'Optional' (checkout); amount = homepage 'DLL (Below Initial Trail)' (homepage only); DLL above Initial Trail not published (unverified). Payout: minimum profit of $150 on 5 separate days per cycle + positive net profit; no buffer; up to 5 payouts then moved live."
status: partial
not_confirmed_fields: [daily_loss_limit, trading_day_reset, copy_trading_allowed, vps_allowed, personal_device_required]
sources:
  - { field: drawdown, url: "https://support.lucidtrading.com/en/articles/12945815-lucidflex-drawdown", accessed: "2026-10-23" }
  - { field: max_position, url: "https://support.lucidtrading.com/en/articles/12945808-lucidflex-scaling-plan", accessed: "2026-10-23" }
  - { field: max_position, url: "https://support.lucidtrading.com/en/articles/12945795-lucidflex-funded-account", accessed: "2026-10-23" }
  - { field: consistency_rule, url: "https://support.lucidtrading.com/en/articles/12945795-lucidflex-funded-account", accessed: "2026-10-23" }
  - { field: payout, url: "https://support.lucidtrading.com/en/articles/12945796-lucidflex-payouts", accessed: "2026-10-23" }
  - { field: daily_loss_limit, url: "https://lucidtrading.com/", accessed: "2026-10-23" }
  - { field: copy_trading_allowed, url: "https://support.lucidtrading.com/en/articles/11404728-other-trading-activities", accessed: "2026-10-23" }
  - { field: automated_trading_allowed, url: "https://support.lucidtrading.com/en/articles/11404728-other-trading-activities", accessed: "2026-10-23" }
  - { field: trading_day_reset, url: "https://support.lucidtrading.com/en/articles/11404729-allowed-trading-times", accessed: "2026-10-23" }
  - { field: copy_trading_allowed, url: "https://lucidtrading.com/terms-of-use/", accessed: "2026-10-23" }
  - { field: platforms, url: "https://support.lucidtrading.com/en/articles/11404614-lucid-trading-supported-platforms", accessed: "2026-10-23" }
  - { field: automation_rule_text, url: "https://support.lucidtrading.com/en/articles/11404728-other-trading-activities", accessed: "2026-10-23" }
  - { field: vps_allowed, url: "https://support.lucidtrading.com/en/?q=vps", accessed: "2026-10-23" }   # help-center search "vps": 0 results; no rule found
```

#### LucidFlex — evaluation — 100K
```yaml
firm: lucid-trading
plan: "LucidFlex"
phase: evaluation
size_label: "100K"
size_usd: "100000.00"
instruments: [MNQ, NQ]
platforms: [NinjaTrader, Tradovate, TradingView, Quantower, Tradesea, MotiveWave, "Sierra Chart", Jigsaw, Bookmap, ATAS, "R|Trader Pro", MultiCharts]   # "All listed platforms work with all Lucid account types"
drawdown: { amount_usd: "3000.00", type: trailing_eod, stops_trailing_at: "MLL locks once balance exceeds Initial Trail Balance $103,100; Locked MLL Balance $100,100 (initial balance + $100)" }
daily_loss_limit: { amount_usd: "1800.00", kind: soft_pause }
profit_target_usd: "6000.00"
consistency_rule: { text: "Largest Single Day Profit / Account Profit = Consistency Percentage; 40% or less to upgrade (50% with add-on)", threshold_pct: "40" }
max_position: { minis: 6, micros: 60, scaling: "No scaling plan in the evaluation phase" }
min_trading_days: null
trading_day_reset: { time: null, tz_as_published: null }   # not published as a reset; see §6 (session 6:00 PM ET/EST, close-out 4:45 PM EST)
copy_trading_allowed: conditional   # copiers permitted; hedging across accounts/users/firms prohibited; Terms ban same-strategy pooling between unconnected accounts/third parties and trading for others; same-person copy not stated explicitly
automated_trading_allowed: conditional   # automated systems permitted; HFT and microscalping prohibited
automation_rule_text: "Automated trading systems and trade copiers are permitted / All automated activity must comply with Lucid Trading rules / Traders are fully responsible for any software errors, malfunctions, or unintended outcomes"
vps_allowed: unknown                # not published on any official page found; see gaps
personal_device_required: unknown   # not published on any official page found; see gaps
notes: "DLL On/Off chosen at checkout; DLL amount is from the homepage only (help center gives none); amount applies to DLL On. Consistency conflict: help center 40% standard / 50% add-on vs homepage eval card 50%."
status: conflict
not_confirmed_fields: [consistency_rule, trading_day_reset, copy_trading_allowed, vps_allowed, personal_device_required]
sources:
  - { field: drawdown, url: "https://support.lucidtrading.com/en/articles/12945815-lucidflex-drawdown", accessed: "2026-10-23" }
  - { field: profit_target_usd, url: "https://support.lucidtrading.com/en/articles/12945790-lucidflex-evaluation-account", accessed: "2026-10-23" }
  - { field: consistency_rule, url: "https://support.lucidtrading.com/en/articles/12945805-lucidflex-consistency-percentage", accessed: "2026-10-23" }
  - { field: daily_loss_limit, url: "https://support.lucidtrading.com/en/articles/16226050-lucidflex-customization", accessed: "2026-10-23" }
  - { field: max_position, url: "https://support.lucidtrading.com/en/articles/12945790-lucidflex-evaluation-account", accessed: "2026-10-23" }
  - { field: daily_loss_limit, url: "https://lucidtrading.com/", accessed: "2026-10-23" }
  - { field: copy_trading_allowed, url: "https://support.lucidtrading.com/en/articles/11404728-other-trading-activities", accessed: "2026-10-23" }
  - { field: automated_trading_allowed, url: "https://support.lucidtrading.com/en/articles/11404728-other-trading-activities", accessed: "2026-10-23" }
  - { field: trading_day_reset, url: "https://support.lucidtrading.com/en/articles/11404729-allowed-trading-times", accessed: "2026-10-23" }
  - { field: copy_trading_allowed, url: "https://lucidtrading.com/terms-of-use/", accessed: "2026-10-23" }
  - { field: platforms, url: "https://support.lucidtrading.com/en/articles/11404614-lucid-trading-supported-platforms", accessed: "2026-10-23" }
  - { field: automation_rule_text, url: "https://support.lucidtrading.com/en/articles/11404728-other-trading-activities", accessed: "2026-10-23" }
  - { field: vps_allowed, url: "https://support.lucidtrading.com/en/?q=vps", accessed: "2026-10-23" }   # help-center search "vps": 0 results; no rule found
```

#### LucidFlex — funded — 100K
```yaml
firm: lucid-trading
plan: "LucidFlex"
phase: funded
size_label: "100K"
size_usd: "100000.00"
instruments: [MNQ, NQ]
platforms: [NinjaTrader, Tradovate, TradingView, Quantower, Tradesea, MotiveWave, "Sierra Chart", Jigsaw, Bookmap, ATAS, "R|Trader Pro", MultiCharts]   # "All listed platforms work with all Lucid account types"
drawdown: { amount_usd: "3000.00", type: trailing_eod, stops_trailing_at: "MLL locks once balance exceeds Initial Trail Balance $103,100; Locked MLL Balance $100,100 (initial balance + $100); on payout request MLL adjusts to the Locked MLL Balance" }
daily_loss_limit: { amount_usd: "1800.00", kind: soft_pause }
profit_target_usd: null
consistency_rule: { text: "There is no Consistency Percentage on LucidFlex funded accounts", threshold_pct: null }
max_position: { minis: 6, micros: 60, scaling: "$0-$999: 3/30; $1,000-$1,999: 4/40; $2,000-$2,999: 5/50; $3,000-$4,499: 6/60 (end-of-session update)" }
min_trading_days: null
trading_day_reset: { time: null, tz_as_published: null }   # not published as a reset; see §6 (session 6:00 PM ET/EST, close-out 4:45 PM EST)
copy_trading_allowed: conditional   # copiers permitted; hedging across accounts/users/firms prohibited; Terms ban same-strategy pooling between unconnected accounts/third parties and trading for others; same-person copy not stated explicitly
automated_trading_allowed: conditional   # automated systems permitted; HFT and microscalping prohibited
automation_rule_text: "Automated trading systems and trade copiers are permitted / All automated activity must comply with Lucid Trading rules / Traders are fully responsible for any software errors, malfunctions, or unintended outcomes"
vps_allowed: unknown                # not published on any official page found; see gaps
personal_device_required: unknown   # not published on any official page found; see gaps
notes: "DLL 'Optional' (checkout); amount = homepage 'DLL (Below Initial Trail)' (homepage only); DLL above Initial Trail not published (unverified). Payout: minimum profit of $200 on 5 separate days per cycle + positive net profit; no buffer; up to 5 payouts then moved live."
status: partial
not_confirmed_fields: [daily_loss_limit, trading_day_reset, copy_trading_allowed, vps_allowed, personal_device_required]
sources:
  - { field: drawdown, url: "https://support.lucidtrading.com/en/articles/12945815-lucidflex-drawdown", accessed: "2026-10-23" }
  - { field: max_position, url: "https://support.lucidtrading.com/en/articles/12945808-lucidflex-scaling-plan", accessed: "2026-10-23" }
  - { field: max_position, url: "https://support.lucidtrading.com/en/articles/12945795-lucidflex-funded-account", accessed: "2026-10-23" }
  - { field: consistency_rule, url: "https://support.lucidtrading.com/en/articles/12945795-lucidflex-funded-account", accessed: "2026-10-23" }
  - { field: payout, url: "https://support.lucidtrading.com/en/articles/12945796-lucidflex-payouts", accessed: "2026-10-23" }
  - { field: daily_loss_limit, url: "https://lucidtrading.com/", accessed: "2026-10-23" }
  - { field: copy_trading_allowed, url: "https://support.lucidtrading.com/en/articles/11404728-other-trading-activities", accessed: "2026-10-23" }
  - { field: automated_trading_allowed, url: "https://support.lucidtrading.com/en/articles/11404728-other-trading-activities", accessed: "2026-10-23" }
  - { field: trading_day_reset, url: "https://support.lucidtrading.com/en/articles/11404729-allowed-trading-times", accessed: "2026-10-23" }
  - { field: copy_trading_allowed, url: "https://lucidtrading.com/terms-of-use/", accessed: "2026-10-23" }
  - { field: platforms, url: "https://support.lucidtrading.com/en/articles/11404614-lucid-trading-supported-platforms", accessed: "2026-10-23" }
  - { field: automation_rule_text, url: "https://support.lucidtrading.com/en/articles/11404728-other-trading-activities", accessed: "2026-10-23" }
  - { field: vps_allowed, url: "https://support.lucidtrading.com/en/?q=vps", accessed: "2026-10-23" }   # help-center search "vps": 0 results; no rule found
```

#### LucidFlex — evaluation — 150K
```yaml
firm: lucid-trading
plan: "LucidFlex"
phase: evaluation
size_label: "150K"
size_usd: "150000.00"
instruments: [MNQ, NQ]
platforms: [NinjaTrader, Tradovate, TradingView, Quantower, Tradesea, MotiveWave, "Sierra Chart", Jigsaw, Bookmap, ATAS, "R|Trader Pro", MultiCharts]   # "All listed platforms work with all Lucid account types"
drawdown: { amount_usd: "4500.00", type: trailing_eod, stops_trailing_at: "MLL locks once balance exceeds Initial Trail Balance $154,600; Locked MLL Balance $150,100 (initial balance + $100)" }
daily_loss_limit: { amount_usd: "2700.00", kind: soft_pause }
profit_target_usd: "9000.00"
consistency_rule: { text: "Largest Single Day Profit / Account Profit = Consistency Percentage; 40% or less to upgrade (50% with add-on)", threshold_pct: "40" }
max_position: { minis: 10, micros: 100, scaling: "No scaling plan in the evaluation phase" }
min_trading_days: null
trading_day_reset: { time: null, tz_as_published: null }   # not published as a reset; see §6 (session 6:00 PM ET/EST, close-out 4:45 PM EST)
copy_trading_allowed: conditional   # copiers permitted; hedging across accounts/users/firms prohibited; Terms ban same-strategy pooling between unconnected accounts/third parties and trading for others; same-person copy not stated explicitly
automated_trading_allowed: conditional   # automated systems permitted; HFT and microscalping prohibited
automation_rule_text: "Automated trading systems and trade copiers are permitted / All automated activity must comply with Lucid Trading rules / Traders are fully responsible for any software errors, malfunctions, or unintended outcomes"
vps_allowed: unknown                # not published on any official page found; see gaps
personal_device_required: unknown   # not published on any official page found; see gaps
notes: "DLL On/Off chosen at checkout; DLL amount is from the homepage only (help center gives none); amount applies to DLL On. Consistency conflict: help center 40% standard / 50% add-on vs homepage eval card 50%."
status: conflict
not_confirmed_fields: [consistency_rule, trading_day_reset, copy_trading_allowed, vps_allowed, personal_device_required]
sources:
  - { field: drawdown, url: "https://support.lucidtrading.com/en/articles/12945815-lucidflex-drawdown", accessed: "2026-10-23" }
  - { field: profit_target_usd, url: "https://support.lucidtrading.com/en/articles/12945790-lucidflex-evaluation-account", accessed: "2026-10-23" }
  - { field: consistency_rule, url: "https://support.lucidtrading.com/en/articles/12945805-lucidflex-consistency-percentage", accessed: "2026-10-23" }
  - { field: daily_loss_limit, url: "https://support.lucidtrading.com/en/articles/16226050-lucidflex-customization", accessed: "2026-10-23" }
  - { field: max_position, url: "https://support.lucidtrading.com/en/articles/12945790-lucidflex-evaluation-account", accessed: "2026-10-23" }
  - { field: daily_loss_limit, url: "https://lucidtrading.com/", accessed: "2026-10-23" }
  - { field: copy_trading_allowed, url: "https://support.lucidtrading.com/en/articles/11404728-other-trading-activities", accessed: "2026-10-23" }
  - { field: automated_trading_allowed, url: "https://support.lucidtrading.com/en/articles/11404728-other-trading-activities", accessed: "2026-10-23" }
  - { field: trading_day_reset, url: "https://support.lucidtrading.com/en/articles/11404729-allowed-trading-times", accessed: "2026-10-23" }
  - { field: copy_trading_allowed, url: "https://lucidtrading.com/terms-of-use/", accessed: "2026-10-23" }
  - { field: platforms, url: "https://support.lucidtrading.com/en/articles/11404614-lucid-trading-supported-platforms", accessed: "2026-10-23" }
  - { field: automation_rule_text, url: "https://support.lucidtrading.com/en/articles/11404728-other-trading-activities", accessed: "2026-10-23" }
  - { field: vps_allowed, url: "https://support.lucidtrading.com/en/?q=vps", accessed: "2026-10-23" }   # help-center search "vps": 0 results; no rule found
```

#### LucidFlex — funded — 150K
```yaml
firm: lucid-trading
plan: "LucidFlex"
phase: funded
size_label: "150K"
size_usd: "150000.00"
instruments: [MNQ, NQ]
platforms: [NinjaTrader, Tradovate, TradingView, Quantower, Tradesea, MotiveWave, "Sierra Chart", Jigsaw, Bookmap, ATAS, "R|Trader Pro", MultiCharts]   # "All listed platforms work with all Lucid account types"
drawdown: { amount_usd: "4500.00", type: trailing_eod, stops_trailing_at: "MLL locks once balance exceeds Initial Trail Balance $154,600; Locked MLL Balance $150,100 (initial balance + $100); on payout request MLL adjusts to the Locked MLL Balance" }
daily_loss_limit: { amount_usd: "2700.00", kind: soft_pause }
profit_target_usd: null
consistency_rule: { text: "There is no Consistency Percentage on LucidFlex funded accounts", threshold_pct: null }
max_position: { minis: 10, micros: 100, scaling: "$0-$999: 4/40; $1,000-$1,999: 5/50; $2,000-$2,999: 6/60; $3,000-$4,499: 8/80; $4,500+: 10/100 (end-of-session update)" }
min_trading_days: null
trading_day_reset: { time: null, tz_as_published: null }   # not published as a reset; see §6 (session 6:00 PM ET/EST, close-out 4:45 PM EST)
copy_trading_allowed: conditional   # copiers permitted; hedging across accounts/users/firms prohibited; Terms ban same-strategy pooling between unconnected accounts/third parties and trading for others; same-person copy not stated explicitly
automated_trading_allowed: conditional   # automated systems permitted; HFT and microscalping prohibited
automation_rule_text: "Automated trading systems and trade copiers are permitted / All automated activity must comply with Lucid Trading rules / Traders are fully responsible for any software errors, malfunctions, or unintended outcomes"
vps_allowed: unknown                # not published on any official page found; see gaps
personal_device_required: unknown   # not published on any official page found; see gaps
notes: "DLL 'Optional' (checkout); amount = homepage 'DLL (Below Initial Trail)' (homepage only); DLL above Initial Trail not published (unverified). Payout: minimum profit of $250 on 5 separate days per cycle + positive net profit; no buffer; up to 5 payouts then moved live."
status: partial
not_confirmed_fields: [daily_loss_limit, trading_day_reset, copy_trading_allowed, vps_allowed, personal_device_required]
sources:
  - { field: drawdown, url: "https://support.lucidtrading.com/en/articles/12945815-lucidflex-drawdown", accessed: "2026-10-23" }
  - { field: max_position, url: "https://support.lucidtrading.com/en/articles/12945808-lucidflex-scaling-plan", accessed: "2026-10-23" }
  - { field: max_position, url: "https://support.lucidtrading.com/en/articles/12945795-lucidflex-funded-account", accessed: "2026-10-23" }
  - { field: consistency_rule, url: "https://support.lucidtrading.com/en/articles/12945795-lucidflex-funded-account", accessed: "2026-10-23" }
  - { field: payout, url: "https://support.lucidtrading.com/en/articles/12945796-lucidflex-payouts", accessed: "2026-10-23" }
  - { field: daily_loss_limit, url: "https://lucidtrading.com/", accessed: "2026-10-23" }
  - { field: copy_trading_allowed, url: "https://support.lucidtrading.com/en/articles/11404728-other-trading-activities", accessed: "2026-10-23" }
  - { field: automated_trading_allowed, url: "https://support.lucidtrading.com/en/articles/11404728-other-trading-activities", accessed: "2026-10-23" }
  - { field: trading_day_reset, url: "https://support.lucidtrading.com/en/articles/11404729-allowed-trading-times", accessed: "2026-10-23" }
  - { field: copy_trading_allowed, url: "https://lucidtrading.com/terms-of-use/", accessed: "2026-10-23" }
  - { field: platforms, url: "https://support.lucidtrading.com/en/articles/11404614-lucid-trading-supported-platforms", accessed: "2026-10-23" }
  - { field: automation_rule_text, url: "https://support.lucidtrading.com/en/articles/11404728-other-trading-activities", accessed: "2026-10-23" }
  - { field: vps_allowed, url: "https://support.lucidtrading.com/en/?q=vps", accessed: "2026-10-23" }   # help-center search "vps": 0 results; no rule found
```

#### LucidDaily — evaluation — 25K
```yaml
firm: lucid-trading
plan: "LucidDaily"
phase: evaluation
size_label: "25K"
size_usd: "25000.00"
instruments: [MNQ, NQ]
platforms: [NinjaTrader, Tradovate, TradingView, Quantower, Tradesea, MotiveWave, "Sierra Chart", Jigsaw, Bookmap, ATAS, "R|Trader Pro", MultiCharts]   # "All listed platforms work with all Lucid account types"
drawdown: { amount_usd: "1000.00", type: other, stops_trailing_at: "MLL locks once balance exceeds Initial Trail Balance $26,100; Locked MLL Balance $25,100 (initial balance + $100); type chosen at checkout: End-of-Day or Intraday" }
daily_loss_limit: { amount_usd: "600.00", kind: soft_pause }
profit_target_usd: "1250.00"
consistency_rule: { text: "Largest Single Day Profit / Account Profit = Consistency Percentage; 50% or less to upgrade (evaluation article) vs '40% (standard) ... 50% (add-on)' (customization article)", threshold_pct: "50" }
max_position: { minis: 2, micros: 20, scaling: null }
min_trading_days: null
trading_day_reset: { time: null, tz_as_published: null }   # not published as a reset; see §6 (session 6:00 PM ET/EST, close-out 4:45 PM EST)
copy_trading_allowed: conditional   # copiers permitted; hedging across accounts/users/firms prohibited; Terms ban same-strategy pooling between unconnected accounts/third parties and trading for others; same-person copy not stated explicitly
automated_trading_allowed: conditional   # automated systems permitted; HFT and microscalping prohibited
automation_rule_text: "Automated trading systems and trade copiers are permitted / All automated activity must comply with Lucid Trading rules / Traders are fully responsible for any software errors, malfunctions, or unintended outcomes"
vps_allowed: unknown                # not published on any official page found; see gaps
personal_device_required: unknown   # not published on any official page found; see gaps
notes: "drawdown.type 'other' = trader picks trailing_eod or trailing_intraday at checkout. DLL On/Off at checkout (amount applies to DLL On). Red-folder USD high-impact news: hard breach on LucidDaily (eval per Other Trading Activities)."
status: conflict
not_confirmed_fields: [consistency_rule, trading_day_reset, copy_trading_allowed, vps_allowed, personal_device_required]
sources:
  - { field: drawdown, url: "https://support.lucidtrading.com/en/articles/15998425-luciddaily-drawdown", accessed: "2026-10-23" }
  - { field: daily_loss_limit, url: "https://support.lucidtrading.com/en/articles/16085900-luciddaily-daily-loss-limit", accessed: "2026-10-23" }
  - { field: profit_target_usd, url: "https://support.lucidtrading.com/en/articles/15996664-luciddaily-evaluation", accessed: "2026-10-23" }
  - { field: consistency_rule, url: "https://support.lucidtrading.com/en/articles/15998336-luciddaily-consistency", accessed: "2026-10-23" }
  - { field: consistency_rule, url: "https://support.lucidtrading.com/en/articles/16033858-luciddaily-customization", accessed: "2026-10-23" }
  - { field: max_position, url: "https://support.lucidtrading.com/en/articles/15996664-luciddaily-evaluation", accessed: "2026-10-23" }
  - { field: copy_trading_allowed, url: "https://support.lucidtrading.com/en/articles/11404728-other-trading-activities", accessed: "2026-10-23" }
  - { field: automated_trading_allowed, url: "https://support.lucidtrading.com/en/articles/11404728-other-trading-activities", accessed: "2026-10-23" }
  - { field: trading_day_reset, url: "https://support.lucidtrading.com/en/articles/11404729-allowed-trading-times", accessed: "2026-10-23" }
  - { field: copy_trading_allowed, url: "https://lucidtrading.com/terms-of-use/", accessed: "2026-10-23" }
  - { field: platforms, url: "https://support.lucidtrading.com/en/articles/11404614-lucid-trading-supported-platforms", accessed: "2026-10-23" }
  - { field: automation_rule_text, url: "https://support.lucidtrading.com/en/articles/11404728-other-trading-activities", accessed: "2026-10-23" }
  - { field: vps_allowed, url: "https://support.lucidtrading.com/en/?q=vps", accessed: "2026-10-23" }   # help-center search "vps": 0 results; no rule found
```

#### LucidDaily — funded — 25K
```yaml
firm: lucid-trading
plan: "LucidDaily"
phase: funded
size_label: "25K"
size_usd: "25000.00"
instruments: [MNQ, NQ]
platforms: [NinjaTrader, Tradovate, TradingView, Quantower, Tradesea, MotiveWave, "Sierra Chart", Jigsaw, Bookmap, ATAS, "R|Trader Pro", MultiCharts]   # "All listed platforms work with all Lucid account types"
drawdown: { amount_usd: "1000.00", type: trailing_intraday, stops_trailing_at: "MLL locks once balance exceeds Initial Trail Balance $26,100; Locked MLL Balance $25,100 (initial balance + $100)" }
daily_loss_limit: { amount_usd: "600.00", kind: soft_pause }
profit_target_usd: null
consistency_rule: { text: "There is no Consistency Percentage on LucidDaily funded accounts", threshold_pct: null }
max_position: { minis: 2, micros: 20, scaling: null }
min_trading_days: null
trading_day_reset: { time: null, tz_as_published: null }   # not published as a reset; see §6 (session 6:00 PM ET/EST, close-out 4:45 PM EST)
copy_trading_allowed: conditional   # copiers permitted; hedging across accounts/users/firms prohibited; Terms ban same-strategy pooling between unconnected accounts/third parties and trading for others; same-person copy not stated explicitly
automated_trading_allowed: conditional   # automated systems permitted; HFT and microscalping prohibited
automation_rule_text: "Automated trading systems and trade copiers are permitted / All automated activity must comply with Lucid Trading rules / Traders are fully responsible for any software errors, malfunctions, or unintended outcomes"
vps_allowed: unknown                # not published on any official page found; see gaps
personal_device_required: unknown   # not published on any official page found; see gaps
notes: "Hard breach: must be flat from 1 minute before through 1 minute after USD high-impact (red folder) news. DLL fixed, does not trail; if MLL trails above DLL, MLL is the binding hard limit. Buffer = Initial MLL + $100. Maximum Daily Profit (moves live): 25K $6,000 / 50K $8,000 / 100K $10,000 / 150K $12,000."
status: partial
not_confirmed_fields: [trading_day_reset, copy_trading_allowed, vps_allowed, personal_device_required]
sources:
  - { field: drawdown, url: "https://support.lucidtrading.com/en/articles/15998425-luciddaily-drawdown", accessed: "2026-10-23" }
  - { field: drawdown, url: "https://support.lucidtrading.com/en/articles/15997244-luciddaily-funded-account", accessed: "2026-10-23" }
  - { field: daily_loss_limit, url: "https://support.lucidtrading.com/en/articles/16085900-luciddaily-daily-loss-limit", accessed: "2026-10-23" }
  - { field: max_position, url: "https://support.lucidtrading.com/en/articles/15997244-luciddaily-funded-account", accessed: "2026-10-23" }
  - { field: payout, url: "https://support.lucidtrading.com/en/articles/15997266-luciddaily-payouts", accessed: "2026-10-23" }
  - { field: copy_trading_allowed, url: "https://support.lucidtrading.com/en/articles/11404728-other-trading-activities", accessed: "2026-10-23" }
  - { field: automated_trading_allowed, url: "https://support.lucidtrading.com/en/articles/11404728-other-trading-activities", accessed: "2026-10-23" }
  - { field: trading_day_reset, url: "https://support.lucidtrading.com/en/articles/11404729-allowed-trading-times", accessed: "2026-10-23" }
  - { field: copy_trading_allowed, url: "https://lucidtrading.com/terms-of-use/", accessed: "2026-10-23" }
  - { field: platforms, url: "https://support.lucidtrading.com/en/articles/11404614-lucid-trading-supported-platforms", accessed: "2026-10-23" }
  - { field: automation_rule_text, url: "https://support.lucidtrading.com/en/articles/11404728-other-trading-activities", accessed: "2026-10-23" }
  - { field: vps_allowed, url: "https://support.lucidtrading.com/en/?q=vps", accessed: "2026-10-23" }   # help-center search "vps": 0 results; no rule found
```

#### LucidDaily — evaluation — 50K
```yaml
firm: lucid-trading
plan: "LucidDaily"
phase: evaluation
size_label: "50K"
size_usd: "50000.00"
instruments: [MNQ, NQ]
platforms: [NinjaTrader, Tradovate, TradingView, Quantower, Tradesea, MotiveWave, "Sierra Chart", Jigsaw, Bookmap, ATAS, "R|Trader Pro", MultiCharts]   # "All listed platforms work with all Lucid account types"
drawdown: { amount_usd: "2000.00", type: other, stops_trailing_at: "MLL locks once balance exceeds Initial Trail Balance $52,100; Locked MLL Balance $50,100 (initial balance + $100); type chosen at checkout: End-of-Day or Intraday" }
daily_loss_limit: { amount_usd: "1200.00", kind: soft_pause }
profit_target_usd: "3000.00"
consistency_rule: { text: "Largest Single Day Profit / Account Profit = Consistency Percentage; 50% or less to upgrade (evaluation article) vs '40% (standard) ... 50% (add-on)' (customization article)", threshold_pct: "50" }
max_position: { minis: 4, micros: 40, scaling: null }
min_trading_days: null
trading_day_reset: { time: null, tz_as_published: null }   # not published as a reset; see §6 (session 6:00 PM ET/EST, close-out 4:45 PM EST)
copy_trading_allowed: conditional   # copiers permitted; hedging across accounts/users/firms prohibited; Terms ban same-strategy pooling between unconnected accounts/third parties and trading for others; same-person copy not stated explicitly
automated_trading_allowed: conditional   # automated systems permitted; HFT and microscalping prohibited
automation_rule_text: "Automated trading systems and trade copiers are permitted / All automated activity must comply with Lucid Trading rules / Traders are fully responsible for any software errors, malfunctions, or unintended outcomes"
vps_allowed: unknown                # not published on any official page found; see gaps
personal_device_required: unknown   # not published on any official page found; see gaps
notes: "drawdown.type 'other' = trader picks trailing_eod or trailing_intraday at checkout. DLL On/Off at checkout (amount applies to DLL On). Red-folder USD high-impact news: hard breach on LucidDaily (eval per Other Trading Activities)."
status: conflict
not_confirmed_fields: [consistency_rule, trading_day_reset, copy_trading_allowed, vps_allowed, personal_device_required]
sources:
  - { field: drawdown, url: "https://support.lucidtrading.com/en/articles/15998425-luciddaily-drawdown", accessed: "2026-10-23" }
  - { field: daily_loss_limit, url: "https://support.lucidtrading.com/en/articles/16085900-luciddaily-daily-loss-limit", accessed: "2026-10-23" }
  - { field: profit_target_usd, url: "https://support.lucidtrading.com/en/articles/15996664-luciddaily-evaluation", accessed: "2026-10-23" }
  - { field: consistency_rule, url: "https://support.lucidtrading.com/en/articles/15998336-luciddaily-consistency", accessed: "2026-10-23" }
  - { field: consistency_rule, url: "https://support.lucidtrading.com/en/articles/16033858-luciddaily-customization", accessed: "2026-10-23" }
  - { field: max_position, url: "https://support.lucidtrading.com/en/articles/15996664-luciddaily-evaluation", accessed: "2026-10-23" }
  - { field: copy_trading_allowed, url: "https://support.lucidtrading.com/en/articles/11404728-other-trading-activities", accessed: "2026-10-23" }
  - { field: automated_trading_allowed, url: "https://support.lucidtrading.com/en/articles/11404728-other-trading-activities", accessed: "2026-10-23" }
  - { field: trading_day_reset, url: "https://support.lucidtrading.com/en/articles/11404729-allowed-trading-times", accessed: "2026-10-23" }
  - { field: copy_trading_allowed, url: "https://lucidtrading.com/terms-of-use/", accessed: "2026-10-23" }
  - { field: platforms, url: "https://support.lucidtrading.com/en/articles/11404614-lucid-trading-supported-platforms", accessed: "2026-10-23" }
  - { field: automation_rule_text, url: "https://support.lucidtrading.com/en/articles/11404728-other-trading-activities", accessed: "2026-10-23" }
  - { field: vps_allowed, url: "https://support.lucidtrading.com/en/?q=vps", accessed: "2026-10-23" }   # help-center search "vps": 0 results; no rule found
```

#### LucidDaily — funded — 50K
```yaml
firm: lucid-trading
plan: "LucidDaily"
phase: funded
size_label: "50K"
size_usd: "50000.00"
instruments: [MNQ, NQ]
platforms: [NinjaTrader, Tradovate, TradingView, Quantower, Tradesea, MotiveWave, "Sierra Chart", Jigsaw, Bookmap, ATAS, "R|Trader Pro", MultiCharts]   # "All listed platforms work with all Lucid account types"
drawdown: { amount_usd: "2000.00", type: trailing_intraday, stops_trailing_at: "MLL locks once balance exceeds Initial Trail Balance $52,100; Locked MLL Balance $50,100 (initial balance + $100)" }
daily_loss_limit: { amount_usd: "1200.00", kind: soft_pause }
profit_target_usd: null
consistency_rule: { text: "There is no Consistency Percentage on LucidDaily funded accounts", threshold_pct: null }
max_position: { minis: 4, micros: 40, scaling: null }
min_trading_days: null
trading_day_reset: { time: null, tz_as_published: null }   # not published as a reset; see §6 (session 6:00 PM ET/EST, close-out 4:45 PM EST)
copy_trading_allowed: conditional   # copiers permitted; hedging across accounts/users/firms prohibited; Terms ban same-strategy pooling between unconnected accounts/third parties and trading for others; same-person copy not stated explicitly
automated_trading_allowed: conditional   # automated systems permitted; HFT and microscalping prohibited
automation_rule_text: "Automated trading systems and trade copiers are permitted / All automated activity must comply with Lucid Trading rules / Traders are fully responsible for any software errors, malfunctions, or unintended outcomes"
vps_allowed: unknown                # not published on any official page found; see gaps
personal_device_required: unknown   # not published on any official page found; see gaps
notes: "Hard breach: must be flat from 1 minute before through 1 minute after USD high-impact (red folder) news. DLL fixed, does not trail; if MLL trails above DLL, MLL is the binding hard limit. Buffer = Initial MLL + $100. Maximum Daily Profit (moves live): 25K $6,000 / 50K $8,000 / 100K $10,000 / 150K $12,000."
status: partial
not_confirmed_fields: [trading_day_reset, copy_trading_allowed, vps_allowed, personal_device_required]
sources:
  - { field: drawdown, url: "https://support.lucidtrading.com/en/articles/15998425-luciddaily-drawdown", accessed: "2026-10-23" }
  - { field: drawdown, url: "https://support.lucidtrading.com/en/articles/15997244-luciddaily-funded-account", accessed: "2026-10-23" }
  - { field: daily_loss_limit, url: "https://support.lucidtrading.com/en/articles/16085900-luciddaily-daily-loss-limit", accessed: "2026-10-23" }
  - { field: max_position, url: "https://support.lucidtrading.com/en/articles/15997244-luciddaily-funded-account", accessed: "2026-10-23" }
  - { field: payout, url: "https://support.lucidtrading.com/en/articles/15997266-luciddaily-payouts", accessed: "2026-10-23" }
  - { field: copy_trading_allowed, url: "https://support.lucidtrading.com/en/articles/11404728-other-trading-activities", accessed: "2026-10-23" }
  - { field: automated_trading_allowed, url: "https://support.lucidtrading.com/en/articles/11404728-other-trading-activities", accessed: "2026-10-23" }
  - { field: trading_day_reset, url: "https://support.lucidtrading.com/en/articles/11404729-allowed-trading-times", accessed: "2026-10-23" }
  - { field: copy_trading_allowed, url: "https://lucidtrading.com/terms-of-use/", accessed: "2026-10-23" }
  - { field: platforms, url: "https://support.lucidtrading.com/en/articles/11404614-lucid-trading-supported-platforms", accessed: "2026-10-23" }
  - { field: automation_rule_text, url: "https://support.lucidtrading.com/en/articles/11404728-other-trading-activities", accessed: "2026-10-23" }
  - { field: vps_allowed, url: "https://support.lucidtrading.com/en/?q=vps", accessed: "2026-10-23" }   # help-center search "vps": 0 results; no rule found
```

#### LucidDaily — evaluation — 100K
```yaml
firm: lucid-trading
plan: "LucidDaily"
phase: evaluation
size_label: "100K"
size_usd: "100000.00"
instruments: [MNQ, NQ]
platforms: [NinjaTrader, Tradovate, TradingView, Quantower, Tradesea, MotiveWave, "Sierra Chart", Jigsaw, Bookmap, ATAS, "R|Trader Pro", MultiCharts]   # "All listed platforms work with all Lucid account types"
drawdown: { amount_usd: "3000.00", type: other, stops_trailing_at: "MLL locks once balance exceeds Initial Trail Balance $103,100; Locked MLL Balance $100,100 (initial balance + $100); type chosen at checkout: End-of-Day or Intraday" }
daily_loss_limit: { amount_usd: "1800.00", kind: soft_pause }
profit_target_usd: "6000.00"
consistency_rule: { text: "Largest Single Day Profit / Account Profit = Consistency Percentage; 50% or less to upgrade (evaluation article) vs '40% (standard) ... 50% (add-on)' (customization article)", threshold_pct: "50" }
max_position: { minis: 6, micros: 60, scaling: null }
min_trading_days: null
trading_day_reset: { time: null, tz_as_published: null }   # not published as a reset; see §6 (session 6:00 PM ET/EST, close-out 4:45 PM EST)
copy_trading_allowed: conditional   # copiers permitted; hedging across accounts/users/firms prohibited; Terms ban same-strategy pooling between unconnected accounts/third parties and trading for others; same-person copy not stated explicitly
automated_trading_allowed: conditional   # automated systems permitted; HFT and microscalping prohibited
automation_rule_text: "Automated trading systems and trade copiers are permitted / All automated activity must comply with Lucid Trading rules / Traders are fully responsible for any software errors, malfunctions, or unintended outcomes"
vps_allowed: unknown                # not published on any official page found; see gaps
personal_device_required: unknown   # not published on any official page found; see gaps
notes: "drawdown.type 'other' = trader picks trailing_eod or trailing_intraday at checkout. DLL On/Off at checkout (amount applies to DLL On). Red-folder USD high-impact news: hard breach on LucidDaily (eval per Other Trading Activities)."
status: conflict
not_confirmed_fields: [consistency_rule, trading_day_reset, copy_trading_allowed, vps_allowed, personal_device_required]
sources:
  - { field: drawdown, url: "https://support.lucidtrading.com/en/articles/15998425-luciddaily-drawdown", accessed: "2026-10-23" }
  - { field: daily_loss_limit, url: "https://support.lucidtrading.com/en/articles/16085900-luciddaily-daily-loss-limit", accessed: "2026-10-23" }
  - { field: profit_target_usd, url: "https://support.lucidtrading.com/en/articles/15996664-luciddaily-evaluation", accessed: "2026-10-23" }
  - { field: consistency_rule, url: "https://support.lucidtrading.com/en/articles/15998336-luciddaily-consistency", accessed: "2026-10-23" }
  - { field: consistency_rule, url: "https://support.lucidtrading.com/en/articles/16033858-luciddaily-customization", accessed: "2026-10-23" }
  - { field: max_position, url: "https://support.lucidtrading.com/en/articles/15996664-luciddaily-evaluation", accessed: "2026-10-23" }
  - { field: copy_trading_allowed, url: "https://support.lucidtrading.com/en/articles/11404728-other-trading-activities", accessed: "2026-10-23" }
  - { field: automated_trading_allowed, url: "https://support.lucidtrading.com/en/articles/11404728-other-trading-activities", accessed: "2026-10-23" }
  - { field: trading_day_reset, url: "https://support.lucidtrading.com/en/articles/11404729-allowed-trading-times", accessed: "2026-10-23" }
  - { field: copy_trading_allowed, url: "https://lucidtrading.com/terms-of-use/", accessed: "2026-10-23" }
  - { field: platforms, url: "https://support.lucidtrading.com/en/articles/11404614-lucid-trading-supported-platforms", accessed: "2026-10-23" }
  - { field: automation_rule_text, url: "https://support.lucidtrading.com/en/articles/11404728-other-trading-activities", accessed: "2026-10-23" }
  - { field: vps_allowed, url: "https://support.lucidtrading.com/en/?q=vps", accessed: "2026-10-23" }   # help-center search "vps": 0 results; no rule found
```

#### LucidDaily — funded — 100K
```yaml
firm: lucid-trading
plan: "LucidDaily"
phase: funded
size_label: "100K"
size_usd: "100000.00"
instruments: [MNQ, NQ]
platforms: [NinjaTrader, Tradovate, TradingView, Quantower, Tradesea, MotiveWave, "Sierra Chart", Jigsaw, Bookmap, ATAS, "R|Trader Pro", MultiCharts]   # "All listed platforms work with all Lucid account types"
drawdown: { amount_usd: "3000.00", type: trailing_intraday, stops_trailing_at: "MLL locks once balance exceeds Initial Trail Balance $103,100; Locked MLL Balance $100,100 (initial balance + $100)" }
daily_loss_limit: { amount_usd: "1800.00", kind: soft_pause }
profit_target_usd: null
consistency_rule: { text: "There is no Consistency Percentage on LucidDaily funded accounts", threshold_pct: null }
max_position: { minis: 6, micros: 60, scaling: null }
min_trading_days: null
trading_day_reset: { time: null, tz_as_published: null }   # not published as a reset; see §6 (session 6:00 PM ET/EST, close-out 4:45 PM EST)
copy_trading_allowed: conditional   # copiers permitted; hedging across accounts/users/firms prohibited; Terms ban same-strategy pooling between unconnected accounts/third parties and trading for others; same-person copy not stated explicitly
automated_trading_allowed: conditional   # automated systems permitted; HFT and microscalping prohibited
automation_rule_text: "Automated trading systems and trade copiers are permitted / All automated activity must comply with Lucid Trading rules / Traders are fully responsible for any software errors, malfunctions, or unintended outcomes"
vps_allowed: unknown                # not published on any official page found; see gaps
personal_device_required: unknown   # not published on any official page found; see gaps
notes: "Hard breach: must be flat from 1 minute before through 1 minute after USD high-impact (red folder) news. DLL fixed, does not trail; if MLL trails above DLL, MLL is the binding hard limit. Buffer = Initial MLL + $100. Maximum Daily Profit (moves live): 25K $6,000 / 50K $8,000 / 100K $10,000 / 150K $12,000."
status: partial
not_confirmed_fields: [trading_day_reset, copy_trading_allowed, vps_allowed, personal_device_required]
sources:
  - { field: drawdown, url: "https://support.lucidtrading.com/en/articles/15998425-luciddaily-drawdown", accessed: "2026-10-23" }
  - { field: drawdown, url: "https://support.lucidtrading.com/en/articles/15997244-luciddaily-funded-account", accessed: "2026-10-23" }
  - { field: daily_loss_limit, url: "https://support.lucidtrading.com/en/articles/16085900-luciddaily-daily-loss-limit", accessed: "2026-10-23" }
  - { field: max_position, url: "https://support.lucidtrading.com/en/articles/15997244-luciddaily-funded-account", accessed: "2026-10-23" }
  - { field: payout, url: "https://support.lucidtrading.com/en/articles/15997266-luciddaily-payouts", accessed: "2026-10-23" }
  - { field: copy_trading_allowed, url: "https://support.lucidtrading.com/en/articles/11404728-other-trading-activities", accessed: "2026-10-23" }
  - { field: automated_trading_allowed, url: "https://support.lucidtrading.com/en/articles/11404728-other-trading-activities", accessed: "2026-10-23" }
  - { field: trading_day_reset, url: "https://support.lucidtrading.com/en/articles/11404729-allowed-trading-times", accessed: "2026-10-23" }
  - { field: copy_trading_allowed, url: "https://lucidtrading.com/terms-of-use/", accessed: "2026-10-23" }
  - { field: platforms, url: "https://support.lucidtrading.com/en/articles/11404614-lucid-trading-supported-platforms", accessed: "2026-10-23" }
  - { field: automation_rule_text, url: "https://support.lucidtrading.com/en/articles/11404728-other-trading-activities", accessed: "2026-10-23" }
  - { field: vps_allowed, url: "https://support.lucidtrading.com/en/?q=vps", accessed: "2026-10-23" }   # help-center search "vps": 0 results; no rule found
```

#### LucidDaily — evaluation — 150K
```yaml
firm: lucid-trading
plan: "LucidDaily"
phase: evaluation
size_label: "150K"
size_usd: "150000.00"
instruments: [MNQ, NQ]
platforms: [NinjaTrader, Tradovate, TradingView, Quantower, Tradesea, MotiveWave, "Sierra Chart", Jigsaw, Bookmap, ATAS, "R|Trader Pro", MultiCharts]   # "All listed platforms work with all Lucid account types"
drawdown: { amount_usd: "4500.00", type: other, stops_trailing_at: "MLL locks once balance exceeds Initial Trail Balance $154,600; Locked MLL Balance $150,100 (initial balance + $100); type chosen at checkout: End-of-Day or Intraday" }
daily_loss_limit: { amount_usd: "2700.00", kind: soft_pause }
profit_target_usd: "9000.00"
consistency_rule: { text: "Largest Single Day Profit / Account Profit = Consistency Percentage; 50% or less to upgrade (evaluation article) vs '40% (standard) ... 50% (add-on)' (customization article)", threshold_pct: "50" }
max_position: { minis: 10, micros: 100, scaling: null }
min_trading_days: null
trading_day_reset: { time: null, tz_as_published: null }   # not published as a reset; see §6 (session 6:00 PM ET/EST, close-out 4:45 PM EST)
copy_trading_allowed: conditional   # copiers permitted; hedging across accounts/users/firms prohibited; Terms ban same-strategy pooling between unconnected accounts/third parties and trading for others; same-person copy not stated explicitly
automated_trading_allowed: conditional   # automated systems permitted; HFT and microscalping prohibited
automation_rule_text: "Automated trading systems and trade copiers are permitted / All automated activity must comply with Lucid Trading rules / Traders are fully responsible for any software errors, malfunctions, or unintended outcomes"
vps_allowed: unknown                # not published on any official page found; see gaps
personal_device_required: unknown   # not published on any official page found; see gaps
notes: "drawdown.type 'other' = trader picks trailing_eod or trailing_intraday at checkout. DLL On/Off at checkout (amount applies to DLL On). Red-folder USD high-impact news: hard breach on LucidDaily (eval per Other Trading Activities)."
status: conflict
not_confirmed_fields: [consistency_rule, trading_day_reset, copy_trading_allowed, vps_allowed, personal_device_required]
sources:
  - { field: drawdown, url: "https://support.lucidtrading.com/en/articles/15998425-luciddaily-drawdown", accessed: "2026-10-23" }
  - { field: daily_loss_limit, url: "https://support.lucidtrading.com/en/articles/16085900-luciddaily-daily-loss-limit", accessed: "2026-10-23" }
  - { field: profit_target_usd, url: "https://support.lucidtrading.com/en/articles/15996664-luciddaily-evaluation", accessed: "2026-10-23" }
  - { field: consistency_rule, url: "https://support.lucidtrading.com/en/articles/15998336-luciddaily-consistency", accessed: "2026-10-23" }
  - { field: consistency_rule, url: "https://support.lucidtrading.com/en/articles/16033858-luciddaily-customization", accessed: "2026-10-23" }
  - { field: max_position, url: "https://support.lucidtrading.com/en/articles/15996664-luciddaily-evaluation", accessed: "2026-10-23" }
  - { field: copy_trading_allowed, url: "https://support.lucidtrading.com/en/articles/11404728-other-trading-activities", accessed: "2026-10-23" }
  - { field: automated_trading_allowed, url: "https://support.lucidtrading.com/en/articles/11404728-other-trading-activities", accessed: "2026-10-23" }
  - { field: trading_day_reset, url: "https://support.lucidtrading.com/en/articles/11404729-allowed-trading-times", accessed: "2026-10-23" }
  - { field: copy_trading_allowed, url: "https://lucidtrading.com/terms-of-use/", accessed: "2026-10-23" }
  - { field: platforms, url: "https://support.lucidtrading.com/en/articles/11404614-lucid-trading-supported-platforms", accessed: "2026-10-23" }
  - { field: automation_rule_text, url: "https://support.lucidtrading.com/en/articles/11404728-other-trading-activities", accessed: "2026-10-23" }
  - { field: vps_allowed, url: "https://support.lucidtrading.com/en/?q=vps", accessed: "2026-10-23" }   # help-center search "vps": 0 results; no rule found
```

#### LucidDaily — funded — 150K
```yaml
firm: lucid-trading
plan: "LucidDaily"
phase: funded
size_label: "150K"
size_usd: "150000.00"
instruments: [MNQ, NQ]
platforms: [NinjaTrader, Tradovate, TradingView, Quantower, Tradesea, MotiveWave, "Sierra Chart", Jigsaw, Bookmap, ATAS, "R|Trader Pro", MultiCharts]   # "All listed platforms work with all Lucid account types"
drawdown: { amount_usd: "4500.00", type: trailing_intraday, stops_trailing_at: "MLL locks once balance exceeds Initial Trail Balance $154,600; Locked MLL Balance $150,100 (initial balance + $100)" }
daily_loss_limit: { amount_usd: "2700.00", kind: soft_pause }
profit_target_usd: null
consistency_rule: { text: "There is no Consistency Percentage on LucidDaily funded accounts", threshold_pct: null }
max_position: { minis: 10, micros: 100, scaling: null }
min_trading_days: null
trading_day_reset: { time: null, tz_as_published: null }   # not published as a reset; see §6 (session 6:00 PM ET/EST, close-out 4:45 PM EST)
copy_trading_allowed: conditional   # copiers permitted; hedging across accounts/users/firms prohibited; Terms ban same-strategy pooling between unconnected accounts/third parties and trading for others; same-person copy not stated explicitly
automated_trading_allowed: conditional   # automated systems permitted; HFT and microscalping prohibited
automation_rule_text: "Automated trading systems and trade copiers are permitted / All automated activity must comply with Lucid Trading rules / Traders are fully responsible for any software errors, malfunctions, or unintended outcomes"
vps_allowed: unknown                # not published on any official page found; see gaps
personal_device_required: unknown   # not published on any official page found; see gaps
notes: "Hard breach: must be flat from 1 minute before through 1 minute after USD high-impact (red folder) news. DLL fixed, does not trail; if MLL trails above DLL, MLL is the binding hard limit. Buffer = Initial MLL + $100. Maximum Daily Profit (moves live): 25K $6,000 / 50K $8,000 / 100K $10,000 / 150K $12,000."
status: partial
not_confirmed_fields: [trading_day_reset, copy_trading_allowed, vps_allowed, personal_device_required]
sources:
  - { field: drawdown, url: "https://support.lucidtrading.com/en/articles/15998425-luciddaily-drawdown", accessed: "2026-10-23" }
  - { field: drawdown, url: "https://support.lucidtrading.com/en/articles/15997244-luciddaily-funded-account", accessed: "2026-10-23" }
  - { field: daily_loss_limit, url: "https://support.lucidtrading.com/en/articles/16085900-luciddaily-daily-loss-limit", accessed: "2026-10-23" }
  - { field: max_position, url: "https://support.lucidtrading.com/en/articles/15997244-luciddaily-funded-account", accessed: "2026-10-23" }
  - { field: payout, url: "https://support.lucidtrading.com/en/articles/15997266-luciddaily-payouts", accessed: "2026-10-23" }
  - { field: copy_trading_allowed, url: "https://support.lucidtrading.com/en/articles/11404728-other-trading-activities", accessed: "2026-10-23" }
  - { field: automated_trading_allowed, url: "https://support.lucidtrading.com/en/articles/11404728-other-trading-activities", accessed: "2026-10-23" }
  - { field: trading_day_reset, url: "https://support.lucidtrading.com/en/articles/11404729-allowed-trading-times", accessed: "2026-10-23" }
  - { field: copy_trading_allowed, url: "https://lucidtrading.com/terms-of-use/", accessed: "2026-10-23" }
  - { field: platforms, url: "https://support.lucidtrading.com/en/articles/11404614-lucid-trading-supported-platforms", accessed: "2026-10-23" }
  - { field: automation_rule_text, url: "https://support.lucidtrading.com/en/articles/11404728-other-trading-activities", accessed: "2026-10-23" }
  - { field: vps_allowed, url: "https://support.lucidtrading.com/en/?q=vps", accessed: "2026-10-23" }   # help-center search "vps": 0 results; no rule found
```

#### LucidDirect — funded (straight-to-funded) — 25K
```yaml
firm: lucid-trading
plan: "LucidDirect"
phase: funded
size_label: "25K"
size_usd: "25000.00"
instruments: [MNQ, NQ]
platforms: [NinjaTrader, Tradovate, TradingView, Quantower, Tradesea, MotiveWave, "Sierra Chart", Jigsaw, Bookmap, ATAS, "R|Trader Pro", MultiCharts]   # "All listed platforms work with all Lucid account types"
drawdown: { amount_usd: "1000.00", type: trailing_eod, stops_trailing_at: "MLL locks once balance exceeds Initial Trail Balance $26,100; Locked MLL Balance $25,100" }
daily_loss_limit: { amount_usd: null, kind: none }
profit_target_usd: null
consistency_rule: { text: "Largest Single Day Profit / Account Profit = Consistency Percentage; 20% or less to request a payout; resets after each payout", threshold_pct: "20" }
max_position: { minis: 2, micros: 20, scaling: "No scaling plan, access to max contract size immediately" }
min_trading_days: null
trading_day_reset: { time: null, tz_as_published: null }   # not published as a reset; see §6 (session 6:00 PM ET/EST, close-out 4:45 PM EST)
copy_trading_allowed: conditional   # copiers permitted; hedging across accounts/users/firms prohibited; Terms ban same-strategy pooling between unconnected accounts/third parties and trading for others; same-person copy not stated explicitly
automated_trading_allowed: conditional   # automated systems permitted; HFT and microscalping prohibited
automation_rule_text: "Automated trading systems and trade copiers are permitted / All automated activity must comply with Lucid Trading rules / Traders are fully responsible for any software errors, malfunctions, or unintended outcomes"
vps_allowed: unknown                # not published on any official page found; see gaps
personal_device_required: unknown   # not published on any official page found; see gaps
notes: "25K: no DLL ('None'). Note DLL table lists Initial Trail Balance 'None' for 25K while drawdown table lists $26,100. No add-ons; DLL set by account size. No evaluation phase."
status: partial
not_confirmed_fields: [drawdown, trading_day_reset, copy_trading_allowed, vps_allowed, personal_device_required]
sources:
  - { field: drawdown, url: "https://support.lucidtrading.com/en/articles/12890192-luciddirect-drawdown", accessed: "2026-10-23" }
  - { field: daily_loss_limit, url: "https://support.lucidtrading.com/en/articles/12890185-luciddirect-daily-loss-limit", accessed: "2026-10-23" }
  - { field: max_position, url: "https://support.lucidtrading.com/en/articles/12890148-luciddirect-funded-account", accessed: "2026-10-23" }
  - { field: consistency_rule, url: "https://support.lucidtrading.com/en/articles/12890178-luciddirect-consistency-percentage", accessed: "2026-10-23" }
  - { field: payout, url: "https://support.lucidtrading.com/en/articles/12890164-luciddirect-payout-objectives", accessed: "2026-10-23" }
  - { field: copy_trading_allowed, url: "https://support.lucidtrading.com/en/articles/11404728-other-trading-activities", accessed: "2026-10-23" }
  - { field: automated_trading_allowed, url: "https://support.lucidtrading.com/en/articles/11404728-other-trading-activities", accessed: "2026-10-23" }
  - { field: trading_day_reset, url: "https://support.lucidtrading.com/en/articles/11404729-allowed-trading-times", accessed: "2026-10-23" }
  - { field: copy_trading_allowed, url: "https://lucidtrading.com/terms-of-use/", accessed: "2026-10-23" }
  - { field: platforms, url: "https://support.lucidtrading.com/en/articles/11404614-lucid-trading-supported-platforms", accessed: "2026-10-23" }
  - { field: automation_rule_text, url: "https://support.lucidtrading.com/en/articles/11404728-other-trading-activities", accessed: "2026-10-23" }
  - { field: vps_allowed, url: "https://support.lucidtrading.com/en/?q=vps", accessed: "2026-10-23" }   # help-center search "vps": 0 results; no rule found
```

#### LucidDirect — funded (straight-to-funded) — 50K
```yaml
firm: lucid-trading
plan: "LucidDirect"
phase: funded
size_label: "50K"
size_usd: "50000.00"
instruments: [MNQ, NQ]
platforms: [NinjaTrader, Tradovate, TradingView, Quantower, Tradesea, MotiveWave, "Sierra Chart", Jigsaw, Bookmap, ATAS, "R|Trader Pro", MultiCharts]   # "All listed platforms work with all Lucid account types"
drawdown: { amount_usd: "2000.00", type: trailing_eod, stops_trailing_at: "MLL locks once balance exceeds Initial Trail Balance $52,100; Locked MLL Balance $50,100" }
daily_loss_limit: { amount_usd: "1200.00", kind: soft_pause }
profit_target_usd: null
consistency_rule: { text: "Largest Single Day Profit / Account Profit = Consistency Percentage; 20% or less to request a payout; resets after each payout", threshold_pct: "20" }
max_position: { minis: 4, micros: 40, scaling: "No scaling plan, access to max contract size immediately" }
min_trading_days: null
trading_day_reset: { time: null, tz_as_published: null }   # not published as a reset; see §6 (session 6:00 PM ET/EST, close-out 4:45 PM EST)
copy_trading_allowed: conditional   # copiers permitted; hedging across accounts/users/firms prohibited; Terms ban same-strategy pooling between unconnected accounts/third parties and trading for others; same-person copy not stated explicitly
automated_trading_allowed: conditional   # automated systems permitted; HFT and microscalping prohibited
automation_rule_text: "Automated trading systems and trade copiers are permitted / All automated activity must comply with Lucid Trading rules / Traders are fully responsible for any software errors, malfunctions, or unintended outcomes"
vps_allowed: unknown                # not published on any official page found; see gaps
personal_device_required: unknown   # not published on any official page found; see gaps
notes: "Fixed DLL until close above Initial Trail Balance, then LucidScale DLL (highest EOD profit × 60%). No add-ons; DLL set by account size. No evaluation phase."
status: partial
not_confirmed_fields: [trading_day_reset, copy_trading_allowed, vps_allowed, personal_device_required]
sources:
  - { field: drawdown, url: "https://support.lucidtrading.com/en/articles/12890192-luciddirect-drawdown", accessed: "2026-10-23" }
  - { field: daily_loss_limit, url: "https://support.lucidtrading.com/en/articles/12890185-luciddirect-daily-loss-limit", accessed: "2026-10-23" }
  - { field: max_position, url: "https://support.lucidtrading.com/en/articles/12890148-luciddirect-funded-account", accessed: "2026-10-23" }
  - { field: consistency_rule, url: "https://support.lucidtrading.com/en/articles/12890178-luciddirect-consistency-percentage", accessed: "2026-10-23" }
  - { field: payout, url: "https://support.lucidtrading.com/en/articles/12890164-luciddirect-payout-objectives", accessed: "2026-10-23" }
  - { field: copy_trading_allowed, url: "https://support.lucidtrading.com/en/articles/11404728-other-trading-activities", accessed: "2026-10-23" }
  - { field: automated_trading_allowed, url: "https://support.lucidtrading.com/en/articles/11404728-other-trading-activities", accessed: "2026-10-23" }
  - { field: trading_day_reset, url: "https://support.lucidtrading.com/en/articles/11404729-allowed-trading-times", accessed: "2026-10-23" }
  - { field: copy_trading_allowed, url: "https://lucidtrading.com/terms-of-use/", accessed: "2026-10-23" }
  - { field: platforms, url: "https://support.lucidtrading.com/en/articles/11404614-lucid-trading-supported-platforms", accessed: "2026-10-23" }
  - { field: automation_rule_text, url: "https://support.lucidtrading.com/en/articles/11404728-other-trading-activities", accessed: "2026-10-23" }
  - { field: vps_allowed, url: "https://support.lucidtrading.com/en/?q=vps", accessed: "2026-10-23" }   # help-center search "vps": 0 results; no rule found
```

#### LucidDirect — funded (straight-to-funded) — 100K
```yaml
firm: lucid-trading
plan: "LucidDirect"
phase: funded
size_label: "100K"
size_usd: "100000.00"
instruments: [MNQ, NQ]
platforms: [NinjaTrader, Tradovate, TradingView, Quantower, Tradesea, MotiveWave, "Sierra Chart", Jigsaw, Bookmap, ATAS, "R|Trader Pro", MultiCharts]   # "All listed platforms work with all Lucid account types"
drawdown: { amount_usd: "3500.00", type: trailing_eod, stops_trailing_at: "MLL locks once balance exceeds Initial Trail Balance $103,600; Locked MLL Balance $100,100" }
daily_loss_limit: { amount_usd: "2100.00", kind: soft_pause }
profit_target_usd: null
consistency_rule: { text: "Largest Single Day Profit / Account Profit = Consistency Percentage; 20% or less to request a payout; resets after each payout", threshold_pct: "20" }
max_position: { minis: 6, micros: 60, scaling: "No scaling plan, access to max contract size immediately" }
min_trading_days: null
trading_day_reset: { time: null, tz_as_published: null }   # not published as a reset; see §6 (session 6:00 PM ET/EST, close-out 4:45 PM EST)
copy_trading_allowed: conditional   # copiers permitted; hedging across accounts/users/firms prohibited; Terms ban same-strategy pooling between unconnected accounts/third parties and trading for others; same-person copy not stated explicitly
automated_trading_allowed: conditional   # automated systems permitted; HFT and microscalping prohibited
automation_rule_text: "Automated trading systems and trade copiers are permitted / All automated activity must comply with Lucid Trading rules / Traders are fully responsible for any software errors, malfunctions, or unintended outcomes"
vps_allowed: unknown                # not published on any official page found; see gaps
personal_device_required: unknown   # not published on any official page found; see gaps
notes: "Fixed DLL until close above Initial Trail Balance, then LucidScale DLL (highest EOD profit × 60%). No add-ons; DLL set by account size. No evaluation phase."
status: partial
not_confirmed_fields: [trading_day_reset, copy_trading_allowed, vps_allowed, personal_device_required]
sources:
  - { field: drawdown, url: "https://support.lucidtrading.com/en/articles/12890192-luciddirect-drawdown", accessed: "2026-10-23" }
  - { field: daily_loss_limit, url: "https://support.lucidtrading.com/en/articles/12890185-luciddirect-daily-loss-limit", accessed: "2026-10-23" }
  - { field: max_position, url: "https://support.lucidtrading.com/en/articles/12890148-luciddirect-funded-account", accessed: "2026-10-23" }
  - { field: consistency_rule, url: "https://support.lucidtrading.com/en/articles/12890178-luciddirect-consistency-percentage", accessed: "2026-10-23" }
  - { field: payout, url: "https://support.lucidtrading.com/en/articles/12890164-luciddirect-payout-objectives", accessed: "2026-10-23" }
  - { field: copy_trading_allowed, url: "https://support.lucidtrading.com/en/articles/11404728-other-trading-activities", accessed: "2026-10-23" }
  - { field: automated_trading_allowed, url: "https://support.lucidtrading.com/en/articles/11404728-other-trading-activities", accessed: "2026-10-23" }
  - { field: trading_day_reset, url: "https://support.lucidtrading.com/en/articles/11404729-allowed-trading-times", accessed: "2026-10-23" }
  - { field: copy_trading_allowed, url: "https://lucidtrading.com/terms-of-use/", accessed: "2026-10-23" }
  - { field: platforms, url: "https://support.lucidtrading.com/en/articles/11404614-lucid-trading-supported-platforms", accessed: "2026-10-23" }
  - { field: automation_rule_text, url: "https://support.lucidtrading.com/en/articles/11404728-other-trading-activities", accessed: "2026-10-23" }
  - { field: vps_allowed, url: "https://support.lucidtrading.com/en/?q=vps", accessed: "2026-10-23" }   # help-center search "vps": 0 results; no rule found
```

#### LucidDirect — funded (straight-to-funded) — 150K
```yaml
firm: lucid-trading
plan: "LucidDirect"
phase: funded
size_label: "150K"
size_usd: "150000.00"
instruments: [MNQ, NQ]
platforms: [NinjaTrader, Tradovate, TradingView, Quantower, Tradesea, MotiveWave, "Sierra Chart", Jigsaw, Bookmap, ATAS, "R|Trader Pro", MultiCharts]   # "All listed platforms work with all Lucid account types"
drawdown: { amount_usd: "5000.00", type: trailing_eod, stops_trailing_at: "MLL locks once balance exceeds Initial Trail Balance $155,100; Locked MLL Balance $150,100" }
daily_loss_limit: { amount_usd: "3000.00", kind: soft_pause }
profit_target_usd: null
consistency_rule: { text: "Largest Single Day Profit / Account Profit = Consistency Percentage; 20% or less to request a payout; resets after each payout", threshold_pct: "20" }
max_position: { minis: 10, micros: 100, scaling: "No scaling plan, access to max contract size immediately" }
min_trading_days: null
trading_day_reset: { time: null, tz_as_published: null }   # not published as a reset; see §6 (session 6:00 PM ET/EST, close-out 4:45 PM EST)
copy_trading_allowed: conditional   # copiers permitted; hedging across accounts/users/firms prohibited; Terms ban same-strategy pooling between unconnected accounts/third parties and trading for others; same-person copy not stated explicitly
automated_trading_allowed: conditional   # automated systems permitted; HFT and microscalping prohibited
automation_rule_text: "Automated trading systems and trade copiers are permitted / All automated activity must comply with Lucid Trading rules / Traders are fully responsible for any software errors, malfunctions, or unintended outcomes"
vps_allowed: unknown                # not published on any official page found; see gaps
personal_device_required: unknown   # not published on any official page found; see gaps
notes: "Fixed DLL until close above Initial Trail Balance, then LucidScale DLL (highest EOD profit × 60%). No add-ons; DLL set by account size. No evaluation phase."
status: partial
not_confirmed_fields: [trading_day_reset, copy_trading_allowed, vps_allowed, personal_device_required]
sources:
  - { field: drawdown, url: "https://support.lucidtrading.com/en/articles/12890192-luciddirect-drawdown", accessed: "2026-10-23" }
  - { field: daily_loss_limit, url: "https://support.lucidtrading.com/en/articles/12890185-luciddirect-daily-loss-limit", accessed: "2026-10-23" }
  - { field: max_position, url: "https://support.lucidtrading.com/en/articles/12890148-luciddirect-funded-account", accessed: "2026-10-23" }
  - { field: consistency_rule, url: "https://support.lucidtrading.com/en/articles/12890178-luciddirect-consistency-percentage", accessed: "2026-10-23" }
  - { field: payout, url: "https://support.lucidtrading.com/en/articles/12890164-luciddirect-payout-objectives", accessed: "2026-10-23" }
  - { field: copy_trading_allowed, url: "https://support.lucidtrading.com/en/articles/11404728-other-trading-activities", accessed: "2026-10-23" }
  - { field: automated_trading_allowed, url: "https://support.lucidtrading.com/en/articles/11404728-other-trading-activities", accessed: "2026-10-23" }
  - { field: trading_day_reset, url: "https://support.lucidtrading.com/en/articles/11404729-allowed-trading-times", accessed: "2026-10-23" }
  - { field: copy_trading_allowed, url: "https://lucidtrading.com/terms-of-use/", accessed: "2026-10-23" }
  - { field: platforms, url: "https://support.lucidtrading.com/en/articles/11404614-lucid-trading-supported-platforms", accessed: "2026-10-23" }
  - { field: automation_rule_text, url: "https://support.lucidtrading.com/en/articles/11404728-other-trading-activities", accessed: "2026-10-23" }
  - { field: vps_allowed, url: "https://support.lucidtrading.com/en/?q=vps", accessed: "2026-10-23" }   # help-center search "vps": 0 results; no rule found
```

#### LucidLive (new live structure) — live — 25K funded-account size
```yaml
firm: lucid-trading
plan: "LucidLive"
phase: live
size_label: "25K"
size_usd: "25000.00"
instruments: [MNQ, NQ]
platforms: [Tradovate, Rithmic]   # Live article names "Tradovate Live accounts" and "Rithmic Live accounts"; supported-platforms page: all listed platforms work with all account types
drawdown: { amount_usd: "1000.00", type: trailing_eod, stops_trailing_at: "Locks at $100 once live profits equal the starting live drawdown, or on a live payout request before that" }
daily_loss_limit: { amount_usd: null, kind: none }
profit_target_usd: null
consistency_rule: { text: "There are no consistency requirements for live accounts", threshold_pct: null }
max_position: { minis: 2, micros: 20, scaling: "CME (NQ/MNQ): $0-$1999.99: 1 mini/10 micros; $2000+: 2/20 (end-of-day update; other exchanges differ)" }
min_trading_days: null
trading_day_reset: { time: null, tz_as_published: null }   # not published as a reset; see §6 (session 6:00 PM ET/EST, close-out 4:45 PM EST)
copy_trading_allowed: conditional   # copiers permitted; hedging across accounts/users/firms prohibited; Terms ban same-strategy pooling between unconnected accounts/third parties and trading for others; same-person copy not stated explicitly
automated_trading_allowed: conditional   # automated systems permitted; HFT and microscalping prohibited
automation_rule_text: "Automated trading systems and trade copiers are permitted / All automated activity must comply with Lucid Trading rules / Traders are fully responsible for any software errors, malfunctions, or unintended outcomes"
vps_allowed: unknown                # not published on any official page found; see gaps
personal_device_required: unknown   # not published on any official page found; see gaps
notes: "Live accounts start at $0 balance; amount = 'Starting Live Drawdown'. Swing trading not allowed; hold until 4:45 PM EST (Tradovate Live) or 4:15 PM EST (Rithmic Live). Live is real capital: agents must never connect to it."
status: partial
not_confirmed_fields: [trading_day_reset, copy_trading_allowed, vps_allowed, personal_device_required]
sources:
  - { field: drawdown, url: "https://support.lucidtrading.com/en/articles/16558826-new-live-structure", accessed: "2026-10-23" }
  - { field: daily_loss_limit, url: "https://support.lucidtrading.com/en/articles/16558826-new-live-structure", accessed: "2026-10-23" }
  - { field: max_position, url: "https://support.lucidtrading.com/en/articles/15245873-live-scaling-plan", accessed: "2026-10-23" }
  - { field: consistency_rule, url: "https://support.lucidtrading.com/en/articles/16558826-new-live-structure", accessed: "2026-10-23" }
  - { field: copy_trading_allowed, url: "https://support.lucidtrading.com/en/articles/11404728-other-trading-activities", accessed: "2026-10-23" }
  - { field: automated_trading_allowed, url: "https://support.lucidtrading.com/en/articles/11404728-other-trading-activities", accessed: "2026-10-23" }
  - { field: trading_day_reset, url: "https://support.lucidtrading.com/en/articles/11404729-allowed-trading-times", accessed: "2026-10-23" }
  - { field: copy_trading_allowed, url: "https://lucidtrading.com/terms-of-use/", accessed: "2026-10-23" }
  - { field: platforms, url: "https://support.lucidtrading.com/en/articles/11404614-lucid-trading-supported-platforms", accessed: "2026-10-23" }
  - { field: automation_rule_text, url: "https://support.lucidtrading.com/en/articles/11404728-other-trading-activities", accessed: "2026-10-23" }
  - { field: vps_allowed, url: "https://support.lucidtrading.com/en/?q=vps", accessed: "2026-10-23" }   # help-center search "vps": 0 results; no rule found
```

#### LucidLive (new live structure) — live — 50K funded-account size
```yaml
firm: lucid-trading
plan: "LucidLive"
phase: live
size_label: "50K"
size_usd: "50000.00"
instruments: [MNQ, NQ]
platforms: [Tradovate, Rithmic]   # Live article names "Tradovate Live accounts" and "Rithmic Live accounts"; supported-platforms page: all listed platforms work with all account types
drawdown: { amount_usd: "2000.00", type: trailing_eod, stops_trailing_at: "Locks at $100 once live profits equal the starting live drawdown, or on a live payout request before that" }
daily_loss_limit: { amount_usd: null, kind: none }
profit_target_usd: null
consistency_rule: { text: "There are no consistency requirements for live accounts", threshold_pct: null }
max_position: { minis: 4, micros: 40, scaling: "CME: $0-$1999.99: 2/20; $2000-$3999: 3/30; $4000+: 4/40" }
min_trading_days: null
trading_day_reset: { time: null, tz_as_published: null }   # not published as a reset; see §6 (session 6:00 PM ET/EST, close-out 4:45 PM EST)
copy_trading_allowed: conditional   # copiers permitted; hedging across accounts/users/firms prohibited; Terms ban same-strategy pooling between unconnected accounts/third parties and trading for others; same-person copy not stated explicitly
automated_trading_allowed: conditional   # automated systems permitted; HFT and microscalping prohibited
automation_rule_text: "Automated trading systems and trade copiers are permitted / All automated activity must comply with Lucid Trading rules / Traders are fully responsible for any software errors, malfunctions, or unintended outcomes"
vps_allowed: unknown                # not published on any official page found; see gaps
personal_device_required: unknown   # not published on any official page found; see gaps
notes: "Live accounts start at $0 balance; amount = 'Starting Live Drawdown'. Swing trading not allowed; hold until 4:45 PM EST (Tradovate Live) or 4:15 PM EST (Rithmic Live). Live is real capital: agents must never connect to it."
status: partial
not_confirmed_fields: [trading_day_reset, copy_trading_allowed, vps_allowed, personal_device_required]
sources:
  - { field: drawdown, url: "https://support.lucidtrading.com/en/articles/16558826-new-live-structure", accessed: "2026-10-23" }
  - { field: daily_loss_limit, url: "https://support.lucidtrading.com/en/articles/16558826-new-live-structure", accessed: "2026-10-23" }
  - { field: max_position, url: "https://support.lucidtrading.com/en/articles/15245873-live-scaling-plan", accessed: "2026-10-23" }
  - { field: consistency_rule, url: "https://support.lucidtrading.com/en/articles/16558826-new-live-structure", accessed: "2026-10-23" }
  - { field: copy_trading_allowed, url: "https://support.lucidtrading.com/en/articles/11404728-other-trading-activities", accessed: "2026-10-23" }
  - { field: automated_trading_allowed, url: "https://support.lucidtrading.com/en/articles/11404728-other-trading-activities", accessed: "2026-10-23" }
  - { field: trading_day_reset, url: "https://support.lucidtrading.com/en/articles/11404729-allowed-trading-times", accessed: "2026-10-23" }
  - { field: copy_trading_allowed, url: "https://lucidtrading.com/terms-of-use/", accessed: "2026-10-23" }
  - { field: platforms, url: "https://support.lucidtrading.com/en/articles/11404614-lucid-trading-supported-platforms", accessed: "2026-10-23" }
  - { field: automation_rule_text, url: "https://support.lucidtrading.com/en/articles/11404728-other-trading-activities", accessed: "2026-10-23" }
  - { field: vps_allowed, url: "https://support.lucidtrading.com/en/?q=vps", accessed: "2026-10-23" }   # help-center search "vps": 0 results; no rule found
```

#### LucidLive (new live structure) — live — 100K funded-account size
```yaml
firm: lucid-trading
plan: "LucidLive"
phase: live
size_label: "100K"
size_usd: "100000.00"
instruments: [MNQ, NQ]
platforms: [Tradovate, Rithmic]   # Live article names "Tradovate Live accounts" and "Rithmic Live accounts"; supported-platforms page: all listed platforms work with all account types
drawdown: { amount_usd: "3000.00", type: trailing_eod, stops_trailing_at: "Locks at $100 once live profits equal the starting live drawdown, or on a live payout request before that" }
daily_loss_limit: { amount_usd: null, kind: none }
profit_target_usd: null
consistency_rule: { text: "There are no consistency requirements for live accounts", threshold_pct: null }
max_position: { minis: 6, micros: 60, scaling: "CME: $0-$1999.99: 3/30; $2000-$3999: 4/40; $4000-$5999: 5/50; $6000+: 6/60" }
min_trading_days: null
trading_day_reset: { time: null, tz_as_published: null }   # not published as a reset; see §6 (session 6:00 PM ET/EST, close-out 4:45 PM EST)
copy_trading_allowed: conditional   # copiers permitted; hedging across accounts/users/firms prohibited; Terms ban same-strategy pooling between unconnected accounts/third parties and trading for others; same-person copy not stated explicitly
automated_trading_allowed: conditional   # automated systems permitted; HFT and microscalping prohibited
automation_rule_text: "Automated trading systems and trade copiers are permitted / All automated activity must comply with Lucid Trading rules / Traders are fully responsible for any software errors, malfunctions, or unintended outcomes"
vps_allowed: unknown                # not published on any official page found; see gaps
personal_device_required: unknown   # not published on any official page found; see gaps
notes: "Live accounts start at $0 balance; amount = 'Starting Live Drawdown'. Swing trading not allowed; hold until 4:45 PM EST (Tradovate Live) or 4:15 PM EST (Rithmic Live). Live is real capital: agents must never connect to it."
status: partial
not_confirmed_fields: [trading_day_reset, copy_trading_allowed, vps_allowed, personal_device_required]
sources:
  - { field: drawdown, url: "https://support.lucidtrading.com/en/articles/16558826-new-live-structure", accessed: "2026-10-23" }
  - { field: daily_loss_limit, url: "https://support.lucidtrading.com/en/articles/16558826-new-live-structure", accessed: "2026-10-23" }
  - { field: max_position, url: "https://support.lucidtrading.com/en/articles/15245873-live-scaling-plan", accessed: "2026-10-23" }
  - { field: consistency_rule, url: "https://support.lucidtrading.com/en/articles/16558826-new-live-structure", accessed: "2026-10-23" }
  - { field: copy_trading_allowed, url: "https://support.lucidtrading.com/en/articles/11404728-other-trading-activities", accessed: "2026-10-23" }
  - { field: automated_trading_allowed, url: "https://support.lucidtrading.com/en/articles/11404728-other-trading-activities", accessed: "2026-10-23" }
  - { field: trading_day_reset, url: "https://support.lucidtrading.com/en/articles/11404729-allowed-trading-times", accessed: "2026-10-23" }
  - { field: copy_trading_allowed, url: "https://lucidtrading.com/terms-of-use/", accessed: "2026-10-23" }
  - { field: platforms, url: "https://support.lucidtrading.com/en/articles/11404614-lucid-trading-supported-platforms", accessed: "2026-10-23" }
  - { field: automation_rule_text, url: "https://support.lucidtrading.com/en/articles/11404728-other-trading-activities", accessed: "2026-10-23" }
  - { field: vps_allowed, url: "https://support.lucidtrading.com/en/?q=vps", accessed: "2026-10-23" }   # help-center search "vps": 0 results; no rule found
```

#### LucidLive (new live structure) — live — 150K funded-account size
```yaml
firm: lucid-trading
plan: "LucidLive"
phase: live
size_label: "150K"
size_usd: "150000.00"
instruments: [MNQ, NQ]
platforms: [Tradovate, Rithmic]   # Live article names "Tradovate Live accounts" and "Rithmic Live accounts"; supported-platforms page: all listed platforms work with all account types
drawdown: { amount_usd: "4500.00", type: trailing_eod, stops_trailing_at: "Locks at $100 once live profits equal the starting live drawdown, or on a live payout request before that" }
daily_loss_limit: { amount_usd: null, kind: none }
profit_target_usd: null
consistency_rule: { text: "There are no consistency requirements for live accounts", threshold_pct: null }
max_position: { minis: 10, micros: 100, scaling: "CME: $0-$1999.99: 4/40; $2000-$3999: 5/50; $4000-$5999: 6/60; $6000-$8999: 8/80; $9000+: 10/100" }
min_trading_days: null
trading_day_reset: { time: null, tz_as_published: null }   # not published as a reset; see §6 (session 6:00 PM ET/EST, close-out 4:45 PM EST)
copy_trading_allowed: conditional   # copiers permitted; hedging across accounts/users/firms prohibited; Terms ban same-strategy pooling between unconnected accounts/third parties and trading for others; same-person copy not stated explicitly
automated_trading_allowed: conditional   # automated systems permitted; HFT and microscalping prohibited
automation_rule_text: "Automated trading systems and trade copiers are permitted / All automated activity must comply with Lucid Trading rules / Traders are fully responsible for any software errors, malfunctions, or unintended outcomes"
vps_allowed: unknown                # not published on any official page found; see gaps
personal_device_required: unknown   # not published on any official page found; see gaps
notes: "Live accounts start at $0 balance; amount = 'Starting Live Drawdown'. Swing trading not allowed; hold until 4:45 PM EST (Tradovate Live) or 4:15 PM EST (Rithmic Live). Live is real capital: agents must never connect to it."
status: partial
not_confirmed_fields: [trading_day_reset, copy_trading_allowed, vps_allowed, personal_device_required]
sources:
  - { field: drawdown, url: "https://support.lucidtrading.com/en/articles/16558826-new-live-structure", accessed: "2026-10-23" }
  - { field: daily_loss_limit, url: "https://support.lucidtrading.com/en/articles/16558826-new-live-structure", accessed: "2026-10-23" }
  - { field: max_position, url: "https://support.lucidtrading.com/en/articles/15245873-live-scaling-plan", accessed: "2026-10-23" }
  - { field: consistency_rule, url: "https://support.lucidtrading.com/en/articles/16558826-new-live-structure", accessed: "2026-10-23" }
  - { field: copy_trading_allowed, url: "https://support.lucidtrading.com/en/articles/11404728-other-trading-activities", accessed: "2026-10-23" }
  - { field: automated_trading_allowed, url: "https://support.lucidtrading.com/en/articles/11404728-other-trading-activities", accessed: "2026-10-23" }
  - { field: trading_day_reset, url: "https://support.lucidtrading.com/en/articles/11404729-allowed-trading-times", accessed: "2026-10-23" }
  - { field: copy_trading_allowed, url: "https://lucidtrading.com/terms-of-use/", accessed: "2026-10-23" }
  - { field: platforms, url: "https://support.lucidtrading.com/en/articles/11404614-lucid-trading-supported-platforms", accessed: "2026-10-23" }
  - { field: automation_rule_text, url: "https://support.lucidtrading.com/en/articles/11404728-other-trading-activities", accessed: "2026-10-23" }
  - { field: vps_allowed, url: "https://support.lucidtrading.com/en/?q=vps", accessed: "2026-10-23" }   # help-center search "vps": 0 results; no rule found
```

---

## 11. Unverified / gaps / conflicts (for the user report)

| # | Field | Status | Details / where searched |
|---|---|---|---|
| 1 | LucidPro 25K DLL (evaluation and funded) | `conflict` | The help evaluation table and the DLL article say "None". The homepage Pro tab (25K eval card "Daily Loss Limit $600"; funded card "DLL (Below Initial Trail) $600") and the funded overview (12890069) say "$600". |
| 2 | LucidPro funded scaling DLL wording | `conflict` (wording) | DLL article: "Highest end-of-day account profit × 60%". Funded overview and homepage: "60% of Peak EOD Balance". |
| 3 | LucidDaily evaluation consistency | `conflict` | Evaluation and consistency articles, and the homepage Daily cards: 50%. Customization article: "40% (standard) … 50% (add-on)". The customization page also contradicts itself on the number of settings and configurations. |
| 4 | LucidFlex evaluation consistency | `conflict` | Help center: "40% (50% with add-on)". Homepage Flex eval cards: "Consistency 50%" with no 40% option visible. |
| 5 | LucidFlex DLL USD amounts | single source (homepage only) | Homepage: $600 / $1,200 / $1,800 / $2,700 for both evaluation and funded ("DLL (Below Initial Trail)"). The help center gives no amounts. The Flex DLL above the Initial Trail is `unverified` (not published anywhere). |
| 6 | Trading-day reset time | `unverified` (not published as a reset) | Published: close-out 4:45 PM EST, trading resumes 6:00 PM EST, payout session "6:00 PM to 3:45 PM ET". No sentence states "the trading day resets at …". Not in the Terms or the other legal pages either. |
| 7 | Copying across the same person's accounts | `unverified` (not explicit) | "trade copiers are permitted" (help center). The Terms ban same-strategy activity "between unconnected accounts and third parties" to pool or hedge risk, and "Trading on behalf of others". No page explicitly addresses copying between one person's own accounts. Searched the help center (61 articles), the homepage and its FAQ, and all 4 footer legal pages. |
| 8 | Copying to/from accounts at other firms | `unverified` | Only "Hedging between different firms or funded platforms" is prohibited. Nothing else was found. |
| 9 | Webhook automation | `unverified` | Not named on any official page (help center, homepage, Terms, Privacy, Refund, Risk Disclosure). |
| 10 | Minimum trading days | `none published` for evaluations | Flex funded payout needs 5 profitable days (help center and homepage). The live bonus needs 20 trading days (bonus only). |
| 11 | HFT numeric threshold | `none published` | Only a qualitative definition (help center and Terms). |
| 12 | LucidDirect 25K Initial Trail Balance | minor `conflict` | The DLL table says "None". The drawdown table says $26,100 → $25,100. |
| 13 | Days to payout (Pro 3, Direct 5) | single source (homepage only) | Homepage cards: Pro "Days to Payout 3"; Direct "Min Day to Payout 5". The LucidPro and LucidDirect payout articles state no minimum-days rule. Not contradictory, but unconfirmed by the help center. |
| 14 | VPS / VPN / remote server and originating device | `unknown` (no rule published) | No rule on any official page (see §9 "Device / hosting rules"). Searched the help-center search ("vps", "vpn" return 0 results), 61 captured help articles, Terms, Privacy, Refund, Risk Disclosure and the homepage FAQ. |
| 15 | Platforms for LucidLive | note | The supported-platforms page says "All listed platforms work with all Lucid account types". The live article names only "Tradovate Live accounts" and "Rithmic Live accounts" (close-out times). The YAML lists Tradovate and Rithmic for the live blocks. |

## 12. Pages not reached

- None. On the second pass (2026-10-23) these were read through the portal: Terms of Use, Privacy Policy, Refund/Chargeback Policy, Risk Disclosure, the homepage Pro/Flex/Daily/Direct tabs (eval cards and "Funded Rules" card backs) and the homepage FAQ.
- The help center was captured with curl. Its pages are server-rendered, so the text is the published article HTML.

## 13. Changelog

- 2026-10-23 — initial research (Lupa)
- 2026-10-23 — second pass (Lupa):
  - Read the Terms of Use, the other footer legal pages, the homepage Flex/Daily/Direct tabs, the "Funded Rules" card backs and the homepage FAQ.
  - Added the Flex DLL amounts (homepage), a new conflict on Flex evaluation consistency, the homepage-only days-to-payout values (Pro 3, Direct 5), and the Terms clauses on copying, automation and trading for others.
  - Fixed verbatim quotes to match the captured line structure (no added punctuation).
  - Updated the gaps list and the YAML for Flex.
- 2026-10-23 — format rev. 3 (Lupa): added `platforms`, `automation_rule_text`, `vps_allowed: unknown`, `personal_device_required: unknown` and `not_confirmed_fields` (with sources) to all 32 YAML blocks; added the (i)/(ii)/(iii) automation classification, the §9 "Device / hosting rules" subsection and gaps 14–15. Existing values and `status` untouched. Fields already conflicting or unpublished (DLL, consistency, trading-day reset, same-person copy) are listed in `not_confirmed_fields`. The `automation_rule_text` quote was checked against the captured page text; the help-center searches ran through the portal "Lupa Lucid".
