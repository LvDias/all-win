# Topstep — Prop Firm Rules Research

| | |
|---|---|
| **Firm** | Topstep (futures only) |
| **Official domains** | `www.topstep.com` (main site; plain curl works) · `help.topstep.com` (official help center, Intercom-hosted; linked from the main site's header "Help Center" and footer → `https://help.topstep.com/`) · `topstepx.com` (TopstepX platform, not read) · `dashboard.topstep.com` (login, not used) |
| **Research date** | 2026-10-23 (all URLs below accessed 2026-10-23) |
| **Researcher** | Lupa (Prop Firm Rules Researcher) |
| **File status** | `partial`. Every core field (drawdown, DLL, profit target, consistency, max position, trading day, copy and automation stance) is verified for the standard Trading Combine, Express Funded Account and Live Funded Account in 50K / 100K / 150K. Still open: one conflict (the time a "day" locks for consistency/payout counting), the LFA micro limit, copying to/from other firms, webhooks, and some Topstep Labs details. See §9 |
| **Instruments covered** | MNQ, NQ. Both are listed as permitted CME Equity Futures. MNQ is marked with "*" ("See Prohibited Conduct for restrictions on specific products marked with an asterisk") (https://help.topstep.com/en/articles/8284206-when-and-what-products-can-i-trade) |
| **Platforms (for routing)** | **TopstepX only.** Quote: "TopstepX™ is the only available trading platform." (https://help.topstep.com/en/articles/8284199-new-to-topstep-start-here). Also available: Quantower through the ProjectX/TopstepX connection (TC and XFA only), and the TopstepX/ProjectX API (TC and XFA only, not Live). **NinjaTrader, Tradovate and Rithmic are not offered.** See §2 |

Evidence (raw text of every cited page, URL on line 1; raw HTML in `raw/`; scaling-plan image in `img/`): `/tmp/claude-1003/-srv-www-allwin--maestri-roles-6bf77a80-a1ed-43dc-bc00-08a480f178a5/33652d13-4dae-46b6-ae06-44a72b375517/scratchpad/r2/topstep/`. This is temporary scratch storage, so the evidence must be captured again at re-verification.

**Programs found.** The standard path is Trading Combine® (TC, evaluation) → Express Funded Account® (XFA, simulated funded) → Live Funded Account® (LFA, live capital, by call-up only). Standard sizes are **$50K, $100K and $150K**. There is no standard 25K, 250K or 300K size. **Topstep Labs** sells experimental products: a $25K Static TC, a $50K Static TC and a $250K Freedom TC (all in the 25K–300K range, recorded in §3.4), plus fixed-payout "Challenges" ($1.5K, $3K, $6K, low priority, §3.5). A **Pro Account** replaces the LFA in jurisdictions without live market access (§3.6). A free 150K **Practice Account** exists (sim practice only, not a funded path).

**Topstep's terms.** "MLL" = Maximum Loss Limit (the drawdown). "DLL" = Daily Loss Limit. "PDLL" = Personal Daily Loss Limit (optional, set by the trader in TopstepX). "Lots" = mini-equivalent contracts. In the TC and XFA, micros and minis count 10:1.

### Main source list (all accessed 2026-10-23)

| Key | URL |
|---|---|
| S1 | https://help.topstep.com/en/articles/8284197-trading-combine-parameters |
| S2 | https://help.topstep.com/en/articles/8284204-what-is-the-maximum-loss-limit |
| S3 | https://help.topstep.com/en/articles/10490293-daily-loss-limit-in-the-trading-combine-and-express-funded-account |
| S4 | https://help.topstep.com/en/articles/8284208-consistency-at-topstep |
| S5 | https://help.topstep.com/en/articles/8284215-express-funded-account-parameters |
| S6 | https://help.topstep.com/en/articles/8284223-what-is-the-scaling-plan (the table is an image, "XFA charts - hc.png") |
| S7 | https://help.topstep.com/en/articles/10657969-live-funded-account-parameters |
| S8 | https://help.topstep.com/en/articles/11748475-dynamic-live-risk-expansion |
| S9 | https://help.topstep.com/en/articles/8284206-when-and-what-products-can-i-trade |
| S10 | https://help.topstep.com/en/articles/13350348-topstep-holiday-trading-hours |
| S11 | https://help.topstep.com/en/articles/14434175-topstepx (Trade Copier section; the old URL `/8284139-platform-connection-instructions` redirects here) |
| S12 | https://help.topstep.com/en/articles/11187768-topstepx-api-access |
| S13 | https://help.topstep.com/en/articles/10296582-prohibited-conduct |
| S14 | https://help.topstep.com/en/articles/10305426-prohibited-trading-strategies-at-topstep |
| S15 | https://help.topstep.com/en/articles/13747047-understanding-hedging |
| S16 | https://help.topstep.com/en/articles/13613539-risk-adjustments-high-risk-high-volatility |
| S17 | https://help.topstep.com/en/articles/8284199-new-to-topstep-start-here |
| S18 | https://help.topstep.com/en/articles/8284179-quantower-connection-instructions |
| S19 | https://help.topstep.com/en/articles/15520357-topstep-labs |
| S20 | https://help.topstep.com/en/articles/14645398-what-is-a-pro-account |
| S21 | https://help.topstep.com/en/articles/8284233-topstep-payout-policy |
| S22 | https://www.topstep.com/topstep-prop (plan cards) |
| S23 | https://www.topstep.com/express-funded-account-rules ("Last updated: June 26, 2025") |
| S24 | https://www.topstep.com/live-funded-account-rules ("Last updated: July 1, 2026") |
| S25 | https://www.topstep.com/terms-of-use ("Last updated: September 21, 2026") |
| S26 | https://help.topstep.com/en/articles/8284121-trading-combine-subscriptions |
| S27 | https://help.topstep.com/en/articles/8284211-economic-releases |
| S28 | https://help.topstep.com/en/articles/10551016-what-is-the-focused-trader-program |
| S29 | https://help.topstep.com/en/articles/8284134-practice-account |

---

## 1. Header notes

- The help center collections index (`/en/collections/…`, 10 collections, 58 articles) was crawled in full. Every article was saved to the evidence directory.
- The main site's rule pages (S22–S25) agree with the help center on every value checked, with one stale exception noted in §9 (S23 mentions "platforms other than TopstepX").

## 2. Platforms

| Plan / phase | Platforms / connections as published | Source |
|---|---|---|
| Trading Combine | TopstepX (web, also mobile browser). Quantower via ProjectX → server "TopstepX". TopstepX/ProjectX API (paid add-on, billed by ProjectX) | S17, S18, S12 |
| Express Funded Account | TopstepX. Quantower via ProjectX/TopstepX. TopstepX/ProjectX API | S17, S18, S12 |
| Live Funded Account | TopstepX only. The API and Quantower are not available on Live | S17, S10, S12, S18 |

Quotes:
- "TopstepX™ is the only available trading platform." (S17, FAQ "What trading platform does Topstep use?")
- "Quantower via TopstepX is available for the Trading Combine® and Express Funded Account® (XFA) only." (S18)
- "TopstepX™ API Access lets advanced Traders and developers build automated strategies, connect third-party tools, and execute trades directly through TopstepX. It's powered by ProjectX and billed separately from your Topstep subscription." (S12)
- "No, Live funded accounts are not allowed to trade through the ProjectX API. The API Gateway is built for the simulated environment and isn't available on Live." (S12)
- "If you trade a Live Funded Account on TopstepX, you are subject to blended trade dates during CME holidays" (S10)
- TopstepX charts: "Cannot connect to external platforms like TradingView; custom indicators are not available due to TradingView's commercial licensing terms." (S11)
- Data: "TopstepX pulls data straight from the exchange(s)— no third-party feed." (S11)
- No official page captured on 2026-10-23 offers NinjaTrader, Tradovate, Rithmic, CQG or T4. S23 still says "Accounts trading on platforms other than TopstepX are also subject to a Daily Loss Limit as outlined below", but the page has no such section. Treated as stale (§9).

**Product impact (for the PO/Architect, not a rule value).** The NinjaTrader 8 plugin route cannot reach Topstep accounts. The only programmatic route is the TopstepX/ProjectX API, and only for TC/XFA. The API page also restricts *where* orders may originate (quoted in §6). A server-side relay that places orders is listed as "Not allowed on a private server".

## 3. Plans

### 3.1 Trading Combine (evaluation). Standard sizes

| Size | Profit target | MLL (drawdown) | Drawdown type / lock | DLL | Consistency | Max position | Min days |
|---|---|---|---|---|---|---|---|
| $50K ($50,000) | $3,000 | $2,000 | trailing EOD; locks at starting balance | optional at checkout: $1,000 (soft pause) | 55% (see below) | 5 minis / 50 micros | no explicit minimum; "as few as two days" |
| $100K ($100,000) | $6,000 | $3,000 | trailing EOD; locks at starting balance | optional: $2,000 (soft pause) | 55% | 10 / 100 | same |
| $150K ($150,000) | $9,000 | $4,500 | trailing EOD; locks at starting balance | optional: $3,000 (soft pause) | 55% | 15 / 150 | same |

Sources: profit targets S4 table and S22 cards. MLL amounts S2 and S22. Max position S1 table ("Micros and minis count at a 10:1 ratio") and S22 ("5 mini / 50 micro", "10 mini / 100 micro", "15 mini / 150 micro"). DLL S3 and S22.

- **Drawdown, quoted (S2):** "The MLL is a trailing limit. It rises as your end-of-day balance grows, but never moves down. Once it reaches your starting balance, it locks permanently." / "The MLL updates at the end of each trading day but is monitored in real time throughout the session. Both realized and unrealized P&L count toward it. If your Net P&L hits the limit at any point during the day, your account is liquidated immediately." S22 adds: "Account balance high - maximum drawdown = Maximum Loss Limit (which is set at the end of each day)."
- **Breach in the TC (S2):** "Your account is liquidated for the rest of the trading day and becomes ineligible for funding until you Reset."
- **DLL, quoted (S3):** "The Daily Loss Limit (DLL) is optional in the Trading Combine® and Express Funded Account® (XFA). It's automatic in the Live Funded Account® (LFA). Triggering it is not a rule violation — it's a forced break for the rest of that session." / "Daily Loss Limit at checkout is fixed (no changes later)" / "Applies to your Express Funded Account after you pass". The kind is **soft_pause**: "Open positions are flattened · Pending orders are canceled · No new trades until 5 PM CT next session". The terms of use agree (S25 §27): "the account will hit a soft breach and will be auto-liquidated for the remainder of the then current trading session."
- **Consistency, quoted (S4):** "Your single best day of profit must stay at or below 55% of your Profit Target. If it exceeds that, your Profit Target increases." / "55% is a hard line. It is not rounded, and there is no buffer." The firm publishes the formula: "Best Day Profit ÷ Total Profit = Best Day %" and "Best Day ÷ 0.55 = Total Profit Needed". Best-day recommendations: 50K "Less than $1,650", 100K "Less than $3,300", 150K "Less than $4,950". S1 words it as: "your best single day should stay below 55% of your Profit Target to avoid increasing your Consistency Target."
- **Minimum days (S1):** "Can I pass the Trading Combine in one day? No. … You can pass in as few as two days, but keep your best day below 55% of your Profit Target." No separate minimum-days rule is published.
- **Accounts (S1, S26):** "There's no limit to the number of Trading Combines you can have." / "Single Profile Policy: All Trading Combines, Express Funded Accounts, and Live Funded Accounts must be under 1 Topstep profile."

### 3.2 Express Funded Account (XFA, simulated funded). Standard sizes

The balance starts at **$0**. The size label means buying power. At activation the trader picks a payout path: **Standard** or **Consistency** (S5).

| Size | MLL | Drawdown type / lock | DLL | Profit target | Consistency | Max position (Scaling Plan) | Days for payout |
|---|---|---|---|---|---|---|---|
| $50K | $2,000 (starts at −$2,000) | trailing EOD; locks at $0; set to $0 after the first payout | optional $1,000 (soft pause), if chosen at TC checkout or at XFA activation | none | Standard: none · Consistency path: 40% | below $1,500: 2 lots · $1,500–$2,000: 3 lots · above $2,000: 5 lots | Standard: 5 winning days of $150+ · Consistency: 3 days with ≥1 trade |
| $100K | $3,000 (starts at −$3,000) | same | optional $2,000 | none | same | below $1,500: 3 · $1,500–$2,000: 4 · $2,000–$3,000: 5 · above $3,000: 10 | same |
| $150K | $4,500 (starts at −$4,500) | same | optional $3,000 | none | same | below $1,500: 3 · $1,500–$2,000: 4 · $2,000–$3,000: 5 · $3,000–$4,500: 10 · above $4,500: 15 | same |

- **MLL, quoted (S2):** "For a 50K XFA, your MLL starts at -$2,000 and trails upward. Once your balance reaches $2,000, the MLL locks at $0 permanently." / "After your first Payout: Your MLL is set to $0 regardless of where it was before." S23 agrees: "$2,000 for $50K accounts, $3,000 for $100K, and $4,500 for $150K … If your Max Loss Limit balance increases to $0, your account will lock at $0." Breach: "Express Funded Account: Your account is permanently closed." (S2)
- **Scaling plan.** Read from the image in S6 (`img/xfa-scaling-hc.png` in the evidence directory). The S6 text says: "Your max contracts do not increase mid-session." / "1 Mini = 10 Micro contracts · Limits are based on mini-contract equivalents · Example — $50K XFA, 2-lot Scaling Plan: 2 Minis, OR 20 Micros". S16's XFA table uses the same balance bands. The image labels overlap at the edges ("$1,500 - $2,000"), so the lot count at exactly $1,500 / $2,000 / $3,000 / $4,500 is not stated. The site version (S23, image `XFA-Rules_Scaling-plan.avif`) is AVIF and could not be read (§9).
- **Consistency (XFA Consistency path only; S4, S5):** "Consistency = Largest Winning Day ÷ Total Net Profit" / "Your Consistency % must be 40% or below to be Payout eligible. This is not rounded." It resets after each payout.
- **Accounts (S5):** "You can hold up to 5 active XFAs at a time." / "Can I trade across multiple XFAs at once? Yes, up to $750K buying power using a trade copier." "Shoulder Tap account holders are limited to 1 XFA."
- **Inactivity (S5):** "XFAs with no trading activity for more than 30 days may be closed."

### 3.3 Live Funded Account (LFA, live capital). Call-up only

| LFA size | Drawdown / loss floor | DLL (starting, automatic) | Max position (starting) | Profit target | Consistency |
|---|---|---|---|---|---|
| $50K | balance must stay above $1,000 (type `other`) | $2,000 (soft pause) | 5 | none to pass. $3,000 net profit unlocks each 25% reserve tranche | none (payout: 5 winning days of $150+) |
| $100K | same | $3,000 | 10 | $6,000 per tranche | same |
| $150K | same | $4,500 | 15 | $9,000 per tranche | same |

- **Size and balance (S7):** the LFA size is the average of eligible XFA sizes, "rounded up to the nearest tier ($50K, $100K, or $150K)". "20% available to trade immediately — minimum $10,000 · 80% held in Reserve — released in 4 increments of 25% as you hit profit thresholds".
- **Loss floor, quoted.** S24: "Currently, your Maximum Loss Limit is $1,000. For the remainder of your Live Funded Account, you will need to make sure your account balance stays above $1,000 (positive)." S7: "If your LFA balance drops below $1,000, the account may be immediately liquidated and closed at end of the trading day." No trailing drawdown is published for the LFA.
- **DLL, quoted (S24):** "If your Net P&L reaches or exceeds this limit during the trading day (defined as 5:00 PM CT to 3:10 PM CT), your account will enter a soft breach." / "This lockout is not a rule violation." The DLL is **dynamic** (S8). It rises +$500 at the $15K / $20K / $50K profit tiers after 10 active trading days per tier, up to "Up to $100,000" at 1M profit. There is also a safeguard: "Tradable balance at or below $10,000 → DLL drops to $2,000, Max Position Size = 5 · Tradable balance at or below $5,000 → DLL drops to $1,000, Max Position Size = 3".
- **Max position (S8):** "Position Limits remain at the max for each account size (5 lots for $50Ks, 10 lots for $100Ks, 15 lots for $150Ks) until the account reaches Tier 4 with with $100K in profit." (typo as published). Micros: "The Micro to Mini ratio functionality is available for the Trading Combine and Express Funded Account. It is not currently available for the Live Funded Account." (S1, S6). **No LFA micro limit is published → unverified.**
- **Accounts (S7):** "You can only have one (1) Live Funded Account active. When you receive a Live Funded Account, all Express Funded Accounts are closed."
- **Inactivity (S7):** "Live Funded Accounts with no trading activity for more than 30 days may be closed."

### 3.4 Topstep Labs evaluations in the 25K–300K range (S19)

S19 says they are "now available for purchase any time. You can purchase up to 5 of each account per month!" They are Labs experiments: "Some Labs ideas graduate into the full Topstep experience. Others won't."

| Product / phase | Profit target | MLL / type | DLL | Max contracts | Consistency |
|---|---|---|---|---|---|
| $25K Static TC | $2,000 | $1,000 "static, non-trailing" | "Mandatory $500" | 2 mini / 20 micro | 55% |
| $25K Static XFA | none | $1,000 static; "After first Payout, MLL sets to $0" | Mandatory $500 | "2 mini / 20 micro (no scaling)" | path: Consistency or Standard |
| $25K Static LFA | — | not stated | Mandatory $500 | "2 mini / 2 micro" (as published) | — |
| $50K Static TC | $4,000 | $2,000 static | Mandatory $1,000 | 4 mini / 40 micro | 55% |
| $50K Static XFA | none | $2,000 static until first payout, then $0 | Mandatory $1,000 | 4 mini / 40 micro; "Scaling Plan 2/3/4" (thresholds not published) | Standard (none) or Consistency (40%) |
| $50K Static LFA | — | per the standard LFA | Mandatory $1,000 | "5 mini / 50 micros and reduces down to a minimum of 3 mini / 30 micros" | — |
| $250K Freedom TC | $15,000 | $10,000 "Trailing End of Day" | Mandatory $5,000 | 25 mini / 250 micro | 55% |
| $250K Freedom XFA | none | $10,000; "After first Payout, MLL sets to $0" (type not stated) | not stated in the XFA table | 25 mini / 250 micro; scaling below $1,500: 3 · $1,500–$2,000: 4 · $2,000–$3,000: 5 · $3,000–$4,500: 10 · $4,500–$6,000: 15 · $6,000–$8,000: 20 · above $8,000: 25 lots | Standard only |
| $250K Freedom LFA | — | standard LFA, "treated as a $150K" | Mandatory $5,000 | 15 mini | — |

### 3.5 Topstep Labs Challenges (outside the size range, low priority)

These are fixed-payout, two-round products with a static MLL and no path to Live (S19). $1.5K Challenge: target $1,500, MLL $500, 2 micros. $1.5K Powered Up: target $1,500, MLL $500, 2 minis / 20 micros. $3K Challenge: target $3,000, MLL $1,000, 10 micros / 1 mini. $3K Undead: target $3,000, MLL $1,000, 5 minis / 50 micros. $3K Slasher: target $3,000, MLL $500, 2 minis / 20 micros. $6K Challenge: target $6,000, MLL $2,000, 10 micros / 1 mini. None has a DLL or consistency rule. They are not modeled in YAML.

### 3.6 Pro Account (S20). Replaces the LFA where live access is unavailable

"It's a simulated account with parameters similar to an LFA." One per trader. Starting balance: $10,000 / $20,000 / $30,000 (50K / 100K / 150K). DLL: $1,000 / $2,000 / $3,000. "The Maximum Loss Limit in a Pro Account mirrors the Express Funded Account® (XFA) Maximum Loss Limit." "Pro Accounts are subject to the contract size Scaling Plan." Not modeled in YAML (pending a PO decision, §9).

## 4. Trading-day reset

- **Trading day: 5:00 PM CT to 3:10 PM CT.** Quotes: "The trading day runs from 5:00 PM CT through 3:10 PM CT the next calendar day. Trades placed after 5:00 PM CT (e.g., 6:30 PM CT Tuesday) count toward Wednesday's activity." (S1). Also: "the trading day (5:00 PM CT-3:10 PM CT)" (S25 §27).
- **Must be flat:** "All positions must be closed by 3:10 PM CT every weekday. You can resume trading at 5:00 PM CT. No swing trading." / "Avoid opening new positions after 3:08 PM CT — Risk Managers begin flattening at that time." (S9; the same article also writes "3:10 PM CST" once). The terms say positions are auto-flattened "about ten seconds prior to the closing bell at 3:10 PM CT" (S25). Products with an earlier close must be exited before that close.
- **Week:** Sunday open 5:00 PM CT. Friday close 3:10 PM CT, "closed till Sunday at 5:00 PM" (S9).
- **Holidays (S10):** "All account types must close positions 15 minutes before any early close." The 2026 schedule is listed (e.g., Thanksgiving Thu Nov 26, 2026: close by 11:45 CT). CME "blended" trade dates apply to the **LFA only** (TC: No, XFA: No).
- **Day-lock time for best-day/winning-day counting: conflict.** S4 says "At 3:10 PM CT, that day's value locks into your trading history". S21 says "A day locks in at 4:00 PM CT" (Standard) and "Days lock in at 4:00 PM CT" (Consistency). The trading-day reset itself (5:00 PM CT) does not conflict.

## 5. Trade copying stance

Classification: TC and XFA **conditional** (allowed with the built-in TopstepX Trade Copier, across the same trader's accounts). LFA **no**.

- "You can also use a trade copier to duplicate trades across multiple accounts. See What is a Trade Copier?" (S1, link → S11)
- "Can I trade across multiple XFAs at once? Yes, up to $750K buying power using a trade copier. See What is a Trade Copier. Hedging across accounts and coordinated trading with others is prohibited." (S5)
- "The Trade Copier lets you mirror trades from a Lead account to 1 or more Follower accounts automatically — so you can run the same strategy across multiple accounts without placing every order manually." / "The Lead account must have the lowest Maximum Position Size (MPS) of all accounts in the group." / "Trade copying is only available for Trading Combine® and Express Funded Account® accounts. Live Funded Account® accounts cannot use the Trade Copier." / "If the Lead hits its Daily Loss Limit or is auto-liquidated, all Follower accounts are flattened with it" / "When a Payout request is submitted, Follower accounts are automatically unlinked." (S11)
- "Your copy-trading connection is automatically disabled while a Payout processes." (S21)
- Hedging (S15): "Technical glitches and copy trading software can create temporary opposite positions — our system accounts for that. But you're still fully responsible for all activity across your accounts, including anything created by automated systems or third-party tools." / "Can I trade the same instrument across multiple accounts? Yes! … What's prohibited is holding opposite positions simultaneously". Cross-account hedging includes "MES/ES, MNQ/NQ".
- **Copying with other people is prohibited.** S25 §27: "In concert with others, including between unconnected accounts and third parties, performing the same trading strategy (i.e., performing the same trades in the same time increments), opposite strategy, or other activity meant to pool or hedge risk of the aggregate, unconnected accounts". Also "Trading on behalf of others including but not limited to sharing any incentives as part of any business arrangement".
- **Account limits:** TC unlimited. XFA 5 active. LFA 1. One profile per person (S1, S25: "you are only permitted to have one Account").
- **Copying to/from accounts at other firms:** not mentioned on any official page → `unknown` (§9).

## 6. Automated trading stance

**(i) Classification:** TC `conditional` · XFA `conditional` · LFA `no` (not possible).

Verbatim rule texts (the opt-in warning text):
- TC (S1): "Yes, with conditions. Topstep won't help set up or troubleshoot automated strategies, and no exceptions are made for errant trades or malfunctions. Test on your Practice Account first and review Prohibited Conduct and Prohibited Trading Strategies before going live."
- XFA (S5): "Yes, with conditions. Topstep won't help set them up or cover malfunctions. Review Prohibited Conduct and Prohibited Trading Strategies first."
- LFA (S7): "The API Gateway is built for the simulated environment and isn't available on Live, so automated strategies are not possible at this time in the Live Funded Account."
- Bots via API (S12): "Yes. Custom automated strategies and bots are allowed via the TopstepX / ProjectX API, subject to standard platform rules and our prohibition on highfrequency trading (HFT). You're free to develop your own system without additional restrictions beyond those standard rules. You remain the owner of your bot and are solely responsible for its design, performance, and behavior — including making sure it operates within all applicable rules."
- **Where orders may originate (S12), critical for All Win's server-side routing:** "All trading activity must originate from your personal device. The use of VPS, VPNs, and remote servers is prohibited by Topstep's Terms of Use. Running automation on a VPS can result in account suspension or removal from the program." The table "Not allowed on a private server" lists: "Placing, modifying, or cancelling orders · Any automated trigger that can reach the order endpoints · Routing or relaying orders on your behalf". "The line is order transmission: your server can watch and record, but it cannot trade." S25 §28: "Using any VPN or VPS on Accounts is strictly prohibited and may result in account termination and forfeiture of profits."
- Limits (S13, S25): "Unfair technology — using software, AI, ultra-high speed systems, or mass data entry to gain an unfair advantage". S14: "Running scalping algorithms designed to exploit unrealistic SIM fills". S12: "A rule violation is treated the same whether it originates from manual trading or third-party automation."

**(ii) Named mechanisms:**
- **Semi-automated trading:** not mentioned.
- **Trade copiers:** mentioned (built-in TopstepX copier, TC/XFA only; see §5).
- **Webhooks:** not mentioned. **Alerts:** TopstepX has price alerts (notifications only), and TopstepX "Cannot connect to external platforms like TradingView" (S11).
- **ATM / automatic brackets:** mentioned. TopstepX offers "Position Risk / Profit Brackets" ("Attach predefined stop-loss and take-profit levels to every position automatically") and "Auto-OCO Brackets" (S11). S14 prohibits misuse: "Using tight brackets or auto-breakeven to take advantage of favorable SIM fills". NinjaTrader ATM is not mentioned.
- **Third-party tools via API:** "Some traders connect third-party applications to the ProjectX API. If you do, it's at your own risk — Topstep and ProjectX don't affiliate with, endorse, or support any external vendor or platform offering API integrations." (S12)

**(iii) Differs by phase:** yes. TC and XFA allow automation with conditions (API, copier). The LFA has no API and no copier, so automation is not possible there.

## 7. Other rules that affect routing or risk

### Device / hosting rules (format rev. 3 §3 item 7)

YAML for all 15 blocks: `vps_allowed: no` and `personal_device_required: yes`. **The rule is the same for every plan** (Trading Combine, XFA, LFA, Labs). The terms-of-use clause covers "Accounts" without naming a plan. The personal-device sentence says "All trading activity" and is not limited to one phase. It is published on the API Access page (TC/XFA, since the API is not available on Live). No page gives a different rule for the LFA or Labs.

- "All trading activity must originate from your personal device. The use of VPS, VPNs, and remote servers is prohibited by Topstep's Terms of Use. Running automation on a VPS can result in account suspension or removal from the program." (https://help.topstep.com/en/articles/11187768-topstepx-api-access, accessed 2026-10-23)
- Same page, table "Not allowed on a private server": "Placing, modifying, or cancelling orders" · "Any automated trigger that can reach the order endpoints" · "Routing or relaying orders on your behalf". Followed by: "The line is order transmission: your server can watch and record, but it cannot trade." Allowed on a private server: "Historical data storage", "Research and backtesting", "Logging and analytics", "A read-only dashboard", "Receiving copies of your own fills, positions, and P&L". (same URL)
- "Using any VPN or VPS on Accounts is strictly prohibited and may result in account termination and forfeiture of profits." (https://www.topstep.com/terms-of-use §28, "Last updated: September 21, 2026", accessed 2026-10-23)
- "Do not use a VPN. VPNs, proxy services, TOR, geo-location obfuscation, and other identity-masking services are not permitted at Topstep." (https://help.topstep.com/en/articles/10296582-prohibited-conduct, accessed 2026-10-23)
- Related: "Exchange rules limit you to 1 device/browser at a time" (https://help.topstep.com/en/articles/14434175-topstepx, accessed 2026-10-23)

### Other rules

- **News (S16, S27):** "Ahead of Tier 1 News Events (for example, CPI, FOMC, NFP, PPI), Topstep temporarily restricts new opening positions on equity index products and metals in SIM". **NQ:** "No new opening transactions permitted during the 10-minute window surrounding the release (5 minutes before, 5 minutes after)." **MNQ:** "Opening transactions limited to 1, 3, 6, 9, and 15 contracts for the $25K, $50K, $100K, $150K, and $250K account sizes". XFA caps during the window also depend on the scaling band (e.g., $50K XFA below $1,500: "0/1"). LFA: "Live traders with a balance below $3,000 will be limited to 1 contract during Tier 1 News Events". S14 prohibits "Purposefully trading your full Maximum Position Size directly into a scheduled major news event." Windows (CT): CPI 7:25–7:35 AM, FOMC 12:55–1:05 PM and 1:25–1:35 PM, NFP 7:25–7:35 AM, PPI 7:25–7:35 AM.
- **Volatility limits (S16):** temporary product limits on energies and metals, not on NQ/MNQ.
- **Hedging (S15):** opposite positions across one trader's accounts are prohibited (escalating liquidation and violations), for TC, XFA and Live. Practice accounts are excluded.
- **Overnight / weekend:** not allowed; flat by 3:10 PM CT (S9).
- **Price limit:** "Holding a position within 2% of a product's price lock limit" is prohibited (S13).
- **Scaling-plan overage (S6):** "Errors corrected in under 10 seconds are ignored. Leave too many contracts on for 10+ seconds and your account may be reviewed."
- **Max accounts:** TC no limit · XFA 5 active · LFA 1 · Pro 1 · Practice 1 (S29). The Focused Trader Program limits a trader to "1 active $50K account at a time" (S28).
- **Inactivity:** XFA and LFA "more than 30 days may be closed". Labs products have a 30-day inactivity rule and a 90-day lifespan.
- **Single device / VPN:** "Exchange rules limit you to 1 device/browser at a time" (S11). VPN/VPS prohibited (S12, S13, S25).
- **Mandatory stop loss / minimum trade duration:** none published.

## 8. Machine-readable blocks

Common to all blocks: `trading_day_reset` is 17:00 CT (the session runs 5:00 PM CT to 3:10 PM CT, flat by 3:10 PM CT). Where a DLL is optional, the block records the amount that applies **if the trader selects it**. With no DLL selected, the kind is `none`.

```yaml
firm: topstep
plan: "Trading Combine"
phase: evaluation
size_label: "$50K"
size_usd: "50000.00"
instruments: [MNQ, NQ]
platforms: [TopstepX, "Quantower (ProjectX connection)", "TopstepX/ProjectX API"]
drawdown: { amount_usd: "2000.00", type: trailing_eod, stops_trailing_at: "Once it reaches your starting balance, it locks permanently ($50,000)" }
daily_loss_limit: { amount_usd: "1000.00", kind: soft_pause }   # optional at checkout; kind none if not selected
profit_target_usd: "3000.00"
consistency_rule: { text: "Your single best day of profit must stay at or below 55% of your Profit Target. If it exceeds that, your Profit Target increases.", threshold_pct: "55" }
max_position: { minis: 5, micros: 50, scaling: null }
min_trading_days: null   # no explicit rule; "You can pass in as few as two days"
trading_day_reset: { time: "17:00", tz_as_published: "CT" }
copy_trading_allowed: conditional
automated_trading_allowed: conditional
automation_rule_text: "Yes, with conditions. Topstep won't help set up or troubleshoot automated strategies, and no exceptions are made for errant trades or malfunctions. Test on your Practice Account first and review Prohibited Conduct and Prohibited Trading Strategies before going live."
vps_allowed: no                     # ToU §28 + API Access page; applies to all account types
personal_device_required: yes       # "All trading activity must originate from your personal device."
status: partial
not_confirmed_fields: [day_lock_time_for_best_day]
sources:
  - { field: vps_allowed, url: "https://www.topstep.com/terms-of-use", accessed: "2026-10-23" }
  - { field: personal_device_required, url: "https://help.topstep.com/en/articles/11187768-topstepx-api-access", accessed: "2026-10-23" }
  - { field: drawdown, url: "https://help.topstep.com/en/articles/8284204-what-is-the-maximum-loss-limit", accessed: "2026-10-23" }
  - { field: daily_loss_limit, url: "https://help.topstep.com/en/articles/10490293-daily-loss-limit-in-the-trading-combine-and-express-funded-account", accessed: "2026-10-23" }
  - { field: profit_target_usd, url: "https://help.topstep.com/en/articles/8284208-consistency-at-topstep", accessed: "2026-10-23" }
  - { field: consistency_rule, url: "https://help.topstep.com/en/articles/8284208-consistency-at-topstep", accessed: "2026-10-23" }
  - { field: max_position, url: "https://help.topstep.com/en/articles/8284197-trading-combine-parameters", accessed: "2026-10-23" }
  - { field: trading_day_reset, url: "https://help.topstep.com/en/articles/8284197-trading-combine-parameters", accessed: "2026-10-23" }
  - { field: platforms, url: "https://help.topstep.com/en/articles/8284199-new-to-topstep-start-here", accessed: "2026-10-23" }
  - { field: copy_trading_allowed, url: "https://help.topstep.com/en/articles/14434175-topstepx", accessed: "2026-10-23" }
  - { field: automated_trading_allowed, url: "https://help.topstep.com/en/articles/8284197-trading-combine-parameters", accessed: "2026-10-23" }
  - { field: plan_cards_crosscheck, url: "https://www.topstep.com/topstep-prop", accessed: "2026-10-23" }
```

```yaml
firm: topstep
plan: "Trading Combine"
phase: evaluation
size_label: "$100K"
size_usd: "100000.00"
instruments: [MNQ, NQ]
platforms: [TopstepX, "Quantower (ProjectX connection)", "TopstepX/ProjectX API"]
drawdown: { amount_usd: "3000.00", type: trailing_eod, stops_trailing_at: "Once it reaches your starting balance, it locks permanently ($100,000)" }
daily_loss_limit: { amount_usd: "2000.00", kind: soft_pause }   # optional at checkout; kind none if not selected
profit_target_usd: "6000.00"
consistency_rule: { text: "Your single best day of profit must stay at or below 55% of your Profit Target. If it exceeds that, your Profit Target increases.", threshold_pct: "55" }
max_position: { minis: 10, micros: 100, scaling: null }
min_trading_days: null   # no explicit rule; "You can pass in as few as two days"
trading_day_reset: { time: "17:00", tz_as_published: "CT" }
copy_trading_allowed: conditional
automated_trading_allowed: conditional
automation_rule_text: "Yes, with conditions. Topstep won't help set up or troubleshoot automated strategies, and no exceptions are made for errant trades or malfunctions. Test on your Practice Account first and review Prohibited Conduct and Prohibited Trading Strategies before going live."
vps_allowed: no                     # ToU §28 + API Access page; applies to all account types
personal_device_required: yes       # "All trading activity must originate from your personal device."
status: partial
not_confirmed_fields: [day_lock_time_for_best_day]
sources:
  - { field: vps_allowed, url: "https://www.topstep.com/terms-of-use", accessed: "2026-10-23" }
  - { field: personal_device_required, url: "https://help.topstep.com/en/articles/11187768-topstepx-api-access", accessed: "2026-10-23" }
  - { field: drawdown, url: "https://help.topstep.com/en/articles/8284204-what-is-the-maximum-loss-limit", accessed: "2026-10-23" }
  - { field: daily_loss_limit, url: "https://help.topstep.com/en/articles/10490293-daily-loss-limit-in-the-trading-combine-and-express-funded-account", accessed: "2026-10-23" }
  - { field: profit_target_usd, url: "https://help.topstep.com/en/articles/8284208-consistency-at-topstep", accessed: "2026-10-23" }
  - { field: consistency_rule, url: "https://help.topstep.com/en/articles/8284208-consistency-at-topstep", accessed: "2026-10-23" }
  - { field: max_position, url: "https://help.topstep.com/en/articles/8284197-trading-combine-parameters", accessed: "2026-10-23" }
  - { field: trading_day_reset, url: "https://help.topstep.com/en/articles/8284197-trading-combine-parameters", accessed: "2026-10-23" }
  - { field: platforms, url: "https://help.topstep.com/en/articles/8284199-new-to-topstep-start-here", accessed: "2026-10-23" }
  - { field: copy_trading_allowed, url: "https://help.topstep.com/en/articles/14434175-topstepx", accessed: "2026-10-23" }
  - { field: automated_trading_allowed, url: "https://help.topstep.com/en/articles/8284197-trading-combine-parameters", accessed: "2026-10-23" }
  - { field: plan_cards_crosscheck, url: "https://www.topstep.com/topstep-prop", accessed: "2026-10-23" }
```

```yaml
firm: topstep
plan: "Trading Combine"
phase: evaluation
size_label: "$150K"
size_usd: "150000.00"
instruments: [MNQ, NQ]
platforms: [TopstepX, "Quantower (ProjectX connection)", "TopstepX/ProjectX API"]
drawdown: { amount_usd: "4500.00", type: trailing_eod, stops_trailing_at: "Once it reaches your starting balance, it locks permanently ($150,000)" }
daily_loss_limit: { amount_usd: "3000.00", kind: soft_pause }   # optional at checkout; kind none if not selected
profit_target_usd: "9000.00"
consistency_rule: { text: "Your single best day of profit must stay at or below 55% of your Profit Target. If it exceeds that, your Profit Target increases.", threshold_pct: "55" }
max_position: { minis: 15, micros: 150, scaling: null }
min_trading_days: null   # no explicit rule; "You can pass in as few as two days"
trading_day_reset: { time: "17:00", tz_as_published: "CT" }
copy_trading_allowed: conditional
automated_trading_allowed: conditional
automation_rule_text: "Yes, with conditions. Topstep won't help set up or troubleshoot automated strategies, and no exceptions are made for errant trades or malfunctions. Test on your Practice Account first and review Prohibited Conduct and Prohibited Trading Strategies before going live."
vps_allowed: no                     # ToU §28 + API Access page; applies to all account types
personal_device_required: yes       # "All trading activity must originate from your personal device."
status: partial
not_confirmed_fields: [day_lock_time_for_best_day]
sources:
  - { field: vps_allowed, url: "https://www.topstep.com/terms-of-use", accessed: "2026-10-23" }
  - { field: personal_device_required, url: "https://help.topstep.com/en/articles/11187768-topstepx-api-access", accessed: "2026-10-23" }
  - { field: drawdown, url: "https://help.topstep.com/en/articles/8284204-what-is-the-maximum-loss-limit", accessed: "2026-10-23" }
  - { field: daily_loss_limit, url: "https://help.topstep.com/en/articles/10490293-daily-loss-limit-in-the-trading-combine-and-express-funded-account", accessed: "2026-10-23" }
  - { field: profit_target_usd, url: "https://help.topstep.com/en/articles/8284208-consistency-at-topstep", accessed: "2026-10-23" }
  - { field: consistency_rule, url: "https://help.topstep.com/en/articles/8284208-consistency-at-topstep", accessed: "2026-10-23" }
  - { field: max_position, url: "https://help.topstep.com/en/articles/8284197-trading-combine-parameters", accessed: "2026-10-23" }
  - { field: trading_day_reset, url: "https://help.topstep.com/en/articles/8284197-trading-combine-parameters", accessed: "2026-10-23" }
  - { field: platforms, url: "https://help.topstep.com/en/articles/8284199-new-to-topstep-start-here", accessed: "2026-10-23" }
  - { field: copy_trading_allowed, url: "https://help.topstep.com/en/articles/14434175-topstepx", accessed: "2026-10-23" }
  - { field: automated_trading_allowed, url: "https://help.topstep.com/en/articles/8284197-trading-combine-parameters", accessed: "2026-10-23" }
  - { field: plan_cards_crosscheck, url: "https://www.topstep.com/topstep-prop", accessed: "2026-10-23" }
```

```yaml
firm: topstep
plan: "Express Funded Account"
phase: funded
size_label: "$50K"
size_usd: "50000.00"   # buying power; balance starts at $0
instruments: [MNQ, NQ]
platforms: [TopstepX, "Quantower (ProjectX connection)", "TopstepX/ProjectX API"]
drawdown: { amount_usd: "2000.00", type: trailing_eod, stops_trailing_at: "MLL starts at -$2,000; once balance reaches $2,000 the MLL locks at $0 permanently; set to $0 after first Payout" }
daily_loss_limit: { amount_usd: "1000.00", kind: soft_pause }   # optional; kind none if not selected
profit_target_usd: null
consistency_rule: { text: "XFA Consistency path only: Consistency = Largest Winning Day ÷ Total Net Profit; must be 40% or below to be Payout eligible. Standard path: none.", threshold_pct: "40" }
max_position: { minis: 5, micros: 50, scaling: "Scaling Plan (balance): Below $1,500 = 2 Lots; $1,500 - $2,000 = 3 Lots; Above $2,000 = 5 Lots; 1 Mini = 10 Micro; max contracts do not increase mid-session" }
min_trading_days: null   # payout eligibility only: Standard 5 winning days of $150+; Consistency 3 days with >=1 trade
trading_day_reset: { time: "17:00", tz_as_published: "CT" }
copy_trading_allowed: conditional
automated_trading_allowed: conditional
automation_rule_text: "Yes, with conditions. Topstep won't help set them up or cover malfunctions. Review Prohibited Conduct and Prohibited Trading Strategies first."
vps_allowed: no                     # ToU §28 + API Access page; applies to all account types
personal_device_required: yes       # "All trading activity must originate from your personal device."
status: partial
not_confirmed_fields: [day_lock_time_for_payout_days, scaling_band_boundaries]
sources:
  - { field: vps_allowed, url: "https://www.topstep.com/terms-of-use", accessed: "2026-10-23" }
  - { field: personal_device_required, url: "https://help.topstep.com/en/articles/11187768-topstepx-api-access", accessed: "2026-10-23" }
  - { field: drawdown, url: "https://help.topstep.com/en/articles/8284204-what-is-the-maximum-loss-limit", accessed: "2026-10-23" }
  - { field: drawdown_crosscheck, url: "https://www.topstep.com/express-funded-account-rules", accessed: "2026-10-23" }
  - { field: daily_loss_limit, url: "https://help.topstep.com/en/articles/10490293-daily-loss-limit-in-the-trading-combine-and-express-funded-account", accessed: "2026-10-23" }
  - { field: consistency_rule, url: "https://help.topstep.com/en/articles/8284215-express-funded-account-parameters", accessed: "2026-10-23" }
  - { field: max_position, url: "https://help.topstep.com/en/articles/8284223-what-is-the-scaling-plan", accessed: "2026-10-23" }
  - { field: min_trading_days, url: "https://help.topstep.com/en/articles/8284233-topstep-payout-policy", accessed: "2026-10-23" }
  - { field: copy_trading_allowed, url: "https://help.topstep.com/en/articles/8284215-express-funded-account-parameters", accessed: "2026-10-23" }
  - { field: automated_trading_allowed, url: "https://help.topstep.com/en/articles/8284215-express-funded-account-parameters", accessed: "2026-10-23" }
  - { field: platforms, url: "https://help.topstep.com/en/articles/8284179-quantower-connection-instructions", accessed: "2026-10-23" }
```

```yaml
firm: topstep
plan: "Express Funded Account"
phase: funded
size_label: "$100K"
size_usd: "100000.00"   # buying power; balance starts at $0
instruments: [MNQ, NQ]
platforms: [TopstepX, "Quantower (ProjectX connection)", "TopstepX/ProjectX API"]
drawdown: { amount_usd: "3000.00", type: trailing_eod, stops_trailing_at: "MLL starts at -$3,000, trails upward and locks at $0 once reached; set to $0 after first Payout" }
daily_loss_limit: { amount_usd: "2000.00", kind: soft_pause }   # optional; kind none if not selected
profit_target_usd: null
consistency_rule: { text: "XFA Consistency path only: Consistency = Largest Winning Day ÷ Total Net Profit; must be 40% or below to be Payout eligible. Standard path: none.", threshold_pct: "40" }
max_position: { minis: 10, micros: 100, scaling: "Scaling Plan (balance): Below $1,500 = 3 Lots; $1,500 - $2,000 = 4 Lots; $2,000 - $3,000 = 5 Lots; Above $3,000 = 10 Lots; 1 Mini = 10 Micro" }
min_trading_days: null   # payout eligibility only: Standard 5 winning days of $150+; Consistency 3 days with >=1 trade
trading_day_reset: { time: "17:00", tz_as_published: "CT" }
copy_trading_allowed: conditional
automated_trading_allowed: conditional
automation_rule_text: "Yes, with conditions. Topstep won't help set them up or cover malfunctions. Review Prohibited Conduct and Prohibited Trading Strategies first."
vps_allowed: no                     # ToU §28 + API Access page; applies to all account types
personal_device_required: yes       # "All trading activity must originate from your personal device."
status: partial
not_confirmed_fields: [day_lock_time_for_payout_days, scaling_band_boundaries]
sources:
  - { field: vps_allowed, url: "https://www.topstep.com/terms-of-use", accessed: "2026-10-23" }
  - { field: personal_device_required, url: "https://help.topstep.com/en/articles/11187768-topstepx-api-access", accessed: "2026-10-23" }
  - { field: drawdown, url: "https://help.topstep.com/en/articles/8284204-what-is-the-maximum-loss-limit", accessed: "2026-10-23" }
  - { field: daily_loss_limit, url: "https://help.topstep.com/en/articles/10490293-daily-loss-limit-in-the-trading-combine-and-express-funded-account", accessed: "2026-10-23" }
  - { field: consistency_rule, url: "https://help.topstep.com/en/articles/8284215-express-funded-account-parameters", accessed: "2026-10-23" }
  - { field: max_position, url: "https://help.topstep.com/en/articles/8284223-what-is-the-scaling-plan", accessed: "2026-10-23" }
  - { field: min_trading_days, url: "https://help.topstep.com/en/articles/8284233-topstep-payout-policy", accessed: "2026-10-23" }
  - { field: copy_trading_allowed, url: "https://help.topstep.com/en/articles/8284215-express-funded-account-parameters", accessed: "2026-10-23" }
  - { field: automated_trading_allowed, url: "https://help.topstep.com/en/articles/8284215-express-funded-account-parameters", accessed: "2026-10-23" }
```

```yaml
firm: topstep
plan: "Express Funded Account"
phase: funded
size_label: "$150K"
size_usd: "150000.00"   # buying power; balance starts at $0
instruments: [MNQ, NQ]
platforms: [TopstepX, "Quantower (ProjectX connection)", "TopstepX/ProjectX API"]
drawdown: { amount_usd: "4500.00", type: trailing_eod, stops_trailing_at: "MLL starts at -$4,500, trails upward and locks at $0 once reached; set to $0 after first Payout" }
daily_loss_limit: { amount_usd: "3000.00", kind: soft_pause }   # optional; kind none if not selected
profit_target_usd: null
consistency_rule: { text: "XFA Consistency path only: Consistency = Largest Winning Day ÷ Total Net Profit; must be 40% or below to be Payout eligible. Standard path: none.", threshold_pct: "40" }
max_position: { minis: 15, micros: 150, scaling: "Scaling Plan (balance): Below $1,500 = 3 Lots; $1,500 - $2,000 = 4 Lots; $2,000 - $3,000 = 5 Lots; $3,000 - $4,500 = 10 Lots; Above $4,500 = 15 Lots; 1 Mini = 10 Micro" }
min_trading_days: null   # payout eligibility only: Standard 5 winning days of $150+; Consistency 3 days with >=1 trade
trading_day_reset: { time: "17:00", tz_as_published: "CT" }
copy_trading_allowed: conditional
automated_trading_allowed: conditional
automation_rule_text: "Yes, with conditions. Topstep won't help set them up or cover malfunctions. Review Prohibited Conduct and Prohibited Trading Strategies first."
vps_allowed: no                     # ToU §28 + API Access page; applies to all account types
personal_device_required: yes       # "All trading activity must originate from your personal device."
status: partial
not_confirmed_fields: [day_lock_time_for_payout_days, scaling_band_boundaries]
sources:
  - { field: vps_allowed, url: "https://www.topstep.com/terms-of-use", accessed: "2026-10-23" }
  - { field: personal_device_required, url: "https://help.topstep.com/en/articles/11187768-topstepx-api-access", accessed: "2026-10-23" }
  - { field: drawdown, url: "https://help.topstep.com/en/articles/8284204-what-is-the-maximum-loss-limit", accessed: "2026-10-23" }
  - { field: daily_loss_limit, url: "https://help.topstep.com/en/articles/10490293-daily-loss-limit-in-the-trading-combine-and-express-funded-account", accessed: "2026-10-23" }
  - { field: consistency_rule, url: "https://help.topstep.com/en/articles/8284215-express-funded-account-parameters", accessed: "2026-10-23" }
  - { field: max_position, url: "https://help.topstep.com/en/articles/8284223-what-is-the-scaling-plan", accessed: "2026-10-23" }
  - { field: min_trading_days, url: "https://help.topstep.com/en/articles/8284233-topstep-payout-policy", accessed: "2026-10-23" }
  - { field: copy_trading_allowed, url: "https://help.topstep.com/en/articles/8284215-express-funded-account-parameters", accessed: "2026-10-23" }
  - { field: automated_trading_allowed, url: "https://help.topstep.com/en/articles/8284215-express-funded-account-parameters", accessed: "2026-10-23" }
```

```yaml
firm: topstep
plan: "Live Funded Account"
phase: live
size_label: "$50K"
size_usd: "50000.00"   # tier; 20% of transferred balance available at start, minimum $10,000
instruments: [MNQ, NQ]
platforms: [TopstepX]
drawdown: { amount_usd: null, type: other, stops_trailing_at: "No trailing MLL published. Balance must stay above $1,000; below $1,000 the account is liquidated and closed at end of day" }
daily_loss_limit: { amount_usd: "2000.00", kind: soft_pause }   # automatic; dynamic (Dynamic Live Risk Expansion, safeguards at $10,000 / $5,000 tradable balance)
profit_target_usd: null   # $3,000 net profit unlocks each 25% reserve tranche (not a pass target)
consistency_rule: { text: null, threshold_pct: null }
max_position: { minis: 5, micros: null, scaling: "Dynamic Live Risk Expansion: 5 lots until Tier 4 ($100K profit), then up to 30/50/70/100 lots; safeguard: tradable balance <= $10,000 → 5, <= $5,000 → 3. Micro:mini ratio not available in LFA" }
min_trading_days: null   # payouts: 5 winning days of $150+ per cycle
trading_day_reset: { time: "17:00", tz_as_published: "CT" }
copy_trading_allowed: no
automated_trading_allowed: no
automation_rule_text: "The API Gateway is built for the simulated environment and isn't available on Live, so automated strategies are not possible at this time in the Live Funded Account."
vps_allowed: no                     # ToU §28 + API Access page; applies to all account types
personal_device_required: yes       # "All trading activity must originate from your personal device."
status: partial
not_confirmed_fields: [max_position.micros]
sources:
  - { field: drawdown, url: "https://www.topstep.com/live-funded-account-rules", accessed: "2026-10-23" }
  - { field: drawdown_crosscheck, url: "https://help.topstep.com/en/articles/10657969-live-funded-account-parameters", accessed: "2026-10-23" }
  - { field: daily_loss_limit, url: "https://help.topstep.com/en/articles/11748475-dynamic-live-risk-expansion", accessed: "2026-10-23" }
  - { field: max_position, url: "https://help.topstep.com/en/articles/11748475-dynamic-live-risk-expansion", accessed: "2026-10-23" }
  - { field: trading_day_reset, url: "https://www.topstep.com/live-funded-account-rules", accessed: "2026-10-23" }
  - { field: copy_trading_allowed, url: "https://help.topstep.com/en/articles/14434175-topstepx", accessed: "2026-10-23" }
  - { field: automated_trading_allowed, url: "https://help.topstep.com/en/articles/10657969-live-funded-account-parameters", accessed: "2026-10-23" }
  - { field: vps_allowed, url: "https://www.topstep.com/terms-of-use", accessed: "2026-10-23" }
  - { field: personal_device_required, url: "https://help.topstep.com/en/articles/11187768-topstepx-api-access", accessed: "2026-10-23" }
```

```yaml
firm: topstep
plan: "Live Funded Account"
phase: live
size_label: "$100K"
size_usd: "100000.00"   # tier; 20% of transferred balance available at start, minimum $10,000
instruments: [MNQ, NQ]
platforms: [TopstepX]
drawdown: { amount_usd: null, type: other, stops_trailing_at: "No trailing MLL published. Balance must stay above $1,000; below $1,000 the account is liquidated and closed at end of day" }
daily_loss_limit: { amount_usd: "3000.00", kind: soft_pause }   # automatic; dynamic
profit_target_usd: null   # $6,000 net profit unlocks each 25% reserve tranche
consistency_rule: { text: null, threshold_pct: null }
max_position: { minis: 10, micros: null, scaling: "Dynamic Live Risk Expansion: 10 lots until Tier 4 ($100K profit), then up to 30/50/70/100 lots; safeguard: tradable balance <= $10,000 → 5, <= $5,000 → 3. Micro:mini ratio not available in LFA" }
min_trading_days: null
trading_day_reset: { time: "17:00", tz_as_published: "CT" }
copy_trading_allowed: no
automated_trading_allowed: no
automation_rule_text: "The API Gateway is built for the simulated environment and isn't available on Live, so automated strategies are not possible at this time in the Live Funded Account."
vps_allowed: no                     # ToU §28 + API Access page; applies to all account types
personal_device_required: yes       # "All trading activity must originate from your personal device."
status: partial
not_confirmed_fields: [max_position.micros]
sources:
  - { field: drawdown, url: "https://www.topstep.com/live-funded-account-rules", accessed: "2026-10-23" }
  - { field: daily_loss_limit, url: "https://help.topstep.com/en/articles/11748475-dynamic-live-risk-expansion", accessed: "2026-10-23" }
  - { field: max_position, url: "https://help.topstep.com/en/articles/11748475-dynamic-live-risk-expansion", accessed: "2026-10-23" }
  - { field: copy_trading_allowed, url: "https://help.topstep.com/en/articles/14434175-topstepx", accessed: "2026-10-23" }
  - { field: automated_trading_allowed, url: "https://help.topstep.com/en/articles/10657969-live-funded-account-parameters", accessed: "2026-10-23" }
  - { field: vps_allowed, url: "https://www.topstep.com/terms-of-use", accessed: "2026-10-23" }
  - { field: personal_device_required, url: "https://help.topstep.com/en/articles/11187768-topstepx-api-access", accessed: "2026-10-23" }
```

```yaml
firm: topstep
plan: "Live Funded Account"
phase: live
size_label: "$150K"
size_usd: "150000.00"   # tier; 20% of transferred balance available at start, minimum $10,000
instruments: [MNQ, NQ]
platforms: [TopstepX]
drawdown: { amount_usd: null, type: other, stops_trailing_at: "No trailing MLL published. Balance must stay above $1,000; below $1,000 the account is liquidated and closed at end of day" }
daily_loss_limit: { amount_usd: "4500.00", kind: soft_pause }   # automatic; dynamic
profit_target_usd: null   # $9,000 net profit unlocks each 25% reserve tranche
consistency_rule: { text: null, threshold_pct: null }
max_position: { minis: 15, micros: null, scaling: "Dynamic Live Risk Expansion: 15 lots until Tier 4 ($100K profit), then up to 30/50/70/100 lots; safeguard: tradable balance <= $10,000 → 5, <= $5,000 → 3. Micro:mini ratio not available in LFA" }
min_trading_days: null
trading_day_reset: { time: "17:00", tz_as_published: "CT" }
copy_trading_allowed: no
automated_trading_allowed: no
automation_rule_text: "The API Gateway is built for the simulated environment and isn't available on Live, so automated strategies are not possible at this time in the Live Funded Account."
vps_allowed: no                     # ToU §28 + API Access page; applies to all account types
personal_device_required: yes       # "All trading activity must originate from your personal device."
status: partial
not_confirmed_fields: [max_position.micros]
sources:
  - { field: drawdown, url: "https://www.topstep.com/live-funded-account-rules", accessed: "2026-10-23" }
  - { field: daily_loss_limit, url: "https://help.topstep.com/en/articles/11748475-dynamic-live-risk-expansion", accessed: "2026-10-23" }
  - { field: max_position, url: "https://help.topstep.com/en/articles/11748475-dynamic-live-risk-expansion", accessed: "2026-10-23" }
  - { field: copy_trading_allowed, url: "https://help.topstep.com/en/articles/14434175-topstepx", accessed: "2026-10-23" }
  - { field: automated_trading_allowed, url: "https://help.topstep.com/en/articles/10657969-live-funded-account-parameters", accessed: "2026-10-23" }
  - { field: vps_allowed, url: "https://www.topstep.com/terms-of-use", accessed: "2026-10-23" }
  - { field: personal_device_required, url: "https://help.topstep.com/en/articles/11187768-topstepx-api-access", accessed: "2026-10-23" }
```

```yaml
firm: topstep
plan: "Topstep Labs $25K Static Trading Combine"
phase: evaluation
size_label: "$25K"
size_usd: "25000.00"
instruments: [MNQ, NQ]
platforms: [TopstepX, "Quantower (ProjectX connection)", "TopstepX/ProjectX API"]   # general platform statement; not stated per Labs product
drawdown: { amount_usd: "1000.00", type: static, stops_trailing_at: "static, non-trailing" }
daily_loss_limit: { amount_usd: "500.00", kind: soft_pause }   # "Mandatory $500"
profit_target_usd: "2000.00"
consistency_rule: { text: "Consistency Target 55%", threshold_pct: "55" }
max_position: { minis: 2, micros: 20, scaling: null }
min_trading_days: null
trading_day_reset: { time: "17:00", tz_as_published: "CT" }
copy_trading_allowed: conditional
automated_trading_allowed: conditional
automation_rule_text: "Yes, with conditions. Topstep won't help set up or troubleshoot automated strategies, and no exceptions are made for errant trades or malfunctions. Test on your Practice Account first and review Prohibited Conduct and Prohibited Trading Strategies before going live."
vps_allowed: no                     # ToU §28 + API Access page; applies to all account types
personal_device_required: yes       # "All trading activity must originate from your personal device."
status: partial
not_confirmed_fields: [platforms, daily_loss_limit.kind]   # Labs page does not restate platform or DLL breach kind
sources:
  - { field: vps_allowed, url: "https://www.topstep.com/terms-of-use", accessed: "2026-10-23" }
  - { field: personal_device_required, url: "https://help.topstep.com/en/articles/11187768-topstepx-api-access", accessed: "2026-10-23" }
  - { field: all_values, url: "https://help.topstep.com/en/articles/15520357-topstep-labs", accessed: "2026-10-23" }
  - { field: automated_trading_allowed, url: "https://help.topstep.com/en/articles/8284197-trading-combine-parameters", accessed: "2026-10-23" }
```

```yaml
firm: topstep
plan: "Topstep Labs $25K Static Trading Combine"
phase: funded
size_label: "$25K"
size_usd: "25000.00"
instruments: [MNQ, NQ]
platforms: [TopstepX, "Quantower (ProjectX connection)", "TopstepX/ProjectX API"]
drawdown: { amount_usd: "1000.00", type: static, stops_trailing_at: "static, non-trailing; After first Payout, MLL sets to $0" }
daily_loss_limit: { amount_usd: "500.00", kind: soft_pause }   # "Mandatory $500"
profit_target_usd: null
consistency_rule: { text: "Path: Choose Consistency or Standard (Consistency path 40%)", threshold_pct: "40" }
max_position: { minis: 2, micros: 20, scaling: "no scaling" }
min_trading_days: null
trading_day_reset: { time: "17:00", tz_as_published: "CT" }
copy_trading_allowed: conditional
automated_trading_allowed: conditional
automation_rule_text: "Yes, with conditions. Topstep won't help set them up or cover malfunctions. Review Prohibited Conduct and Prohibited Trading Strategies first."
vps_allowed: no                     # ToU §28 + API Access page; applies to all account types
personal_device_required: yes       # "All trading activity must originate from your personal device."
status: partial
not_confirmed_fields: [platforms, daily_loss_limit.kind]
sources:
  - { field: vps_allowed, url: "https://www.topstep.com/terms-of-use", accessed: "2026-10-23" }
  - { field: personal_device_required, url: "https://help.topstep.com/en/articles/11187768-topstepx-api-access", accessed: "2026-10-23" }
  - { field: all_values, url: "https://help.topstep.com/en/articles/15520357-topstep-labs", accessed: "2026-10-23" }
```

```yaml
firm: topstep
plan: "Topstep Labs $50K Static Drawdown Trading Combine"
phase: evaluation
size_label: "$50K"
size_usd: "50000.00"
instruments: [MNQ, NQ]
platforms: [TopstepX, "Quantower (ProjectX connection)", "TopstepX/ProjectX API"]
drawdown: { amount_usd: "2000.00", type: static, stops_trailing_at: "static, non-trailing" }
daily_loss_limit: { amount_usd: "1000.00", kind: soft_pause }   # "Mandatory $1,000"
profit_target_usd: "4000.00"
consistency_rule: { text: "Consistency Target 55%", threshold_pct: "55" }
max_position: { minis: 4, micros: 40, scaling: null }
min_trading_days: null
trading_day_reset: { time: "17:00", tz_as_published: "CT" }
copy_trading_allowed: conditional
automated_trading_allowed: conditional
automation_rule_text: "Yes, with conditions. Topstep won't help set up or troubleshoot automated strategies, and no exceptions are made for errant trades or malfunctions. Test on your Practice Account first and review Prohibited Conduct and Prohibited Trading Strategies before going live."
vps_allowed: no                     # ToU §28 + API Access page; applies to all account types
personal_device_required: yes       # "All trading activity must originate from your personal device."
status: partial
not_confirmed_fields: [platforms, daily_loss_limit.kind]
sources:
  - { field: vps_allowed, url: "https://www.topstep.com/terms-of-use", accessed: "2026-10-23" }
  - { field: personal_device_required, url: "https://help.topstep.com/en/articles/11187768-topstepx-api-access", accessed: "2026-10-23" }
  - { field: all_values, url: "https://help.topstep.com/en/articles/15520357-topstep-labs", accessed: "2026-10-23" }
```

```yaml
firm: topstep
plan: "Topstep Labs $50K Static Drawdown Trading Combine"
phase: funded
size_label: "$50K"
size_usd: "50000.00"
instruments: [MNQ, NQ]
platforms: [TopstepX, "Quantower (ProjectX connection)", "TopstepX/ProjectX API"]
drawdown: { amount_usd: "2000.00", type: static, stops_trailing_at: "static, non-trailing. After first Payout, MLL sets to $0" }
daily_loss_limit: { amount_usd: "1000.00", kind: soft_pause }   # "Mandatory $1,000"
profit_target_usd: null
consistency_rule: { text: "Choose Standard (no consistency) or Consistency (40%)", threshold_pct: "40" }
max_position: { minis: 4, micros: 40, scaling: "Scaling Plan 2/3/4 (balance thresholds not published)" }
min_trading_days: null
trading_day_reset: { time: "17:00", tz_as_published: "CT" }
copy_trading_allowed: conditional
automated_trading_allowed: conditional
automation_rule_text: "Yes, with conditions. Topstep won't help set them up or cover malfunctions. Review Prohibited Conduct and Prohibited Trading Strategies first."
vps_allowed: no                     # ToU §28 + API Access page; applies to all account types
personal_device_required: yes       # "All trading activity must originate from your personal device."
status: partial
not_confirmed_fields: [platforms, daily_loss_limit.kind, max_position.scaling]
sources:
  - { field: vps_allowed, url: "https://www.topstep.com/terms-of-use", accessed: "2026-10-23" }
  - { field: personal_device_required, url: "https://help.topstep.com/en/articles/11187768-topstepx-api-access", accessed: "2026-10-23" }
  - { field: all_values, url: "https://help.topstep.com/en/articles/15520357-topstep-labs", accessed: "2026-10-23" }
```

```yaml
firm: topstep
plan: "Topstep Labs $250K Freedom Funded Trading Combine"
phase: evaluation
size_label: "$250K"
size_usd: "250000.00"
instruments: [MNQ, NQ]
platforms: [TopstepX, "Quantower (ProjectX connection)", "TopstepX/ProjectX API"]
drawdown: { amount_usd: "10000.00", type: trailing_eod, stops_trailing_at: null }
daily_loss_limit: { amount_usd: "5000.00", kind: soft_pause }   # "Mandatory $5,000"
profit_target_usd: "15000.00"
consistency_rule: { text: "Consistency Target 55%", threshold_pct: "55" }
max_position: { minis: 25, micros: 250, scaling: null }
min_trading_days: null
trading_day_reset: { time: "17:00", tz_as_published: "CT" }
copy_trading_allowed: conditional
automated_trading_allowed: conditional
automation_rule_text: "Yes, with conditions. Topstep won't help set up or troubleshoot automated strategies, and no exceptions are made for errant trades or malfunctions. Test on your Practice Account first and review Prohibited Conduct and Prohibited Trading Strategies before going live."
vps_allowed: no                     # ToU §28 + API Access page; applies to all account types
personal_device_required: yes       # "All trading activity must originate from your personal device."
status: partial
not_confirmed_fields: [platforms, daily_loss_limit.kind, drawdown.stops_trailing_at]
sources:
  - { field: vps_allowed, url: "https://www.topstep.com/terms-of-use", accessed: "2026-10-23" }
  - { field: personal_device_required, url: "https://help.topstep.com/en/articles/11187768-topstepx-api-access", accessed: "2026-10-23" }
  - { field: all_values, url: "https://help.topstep.com/en/articles/15520357-topstep-labs", accessed: "2026-10-23" }
```

```yaml
firm: topstep
plan: "Topstep Labs $250K Freedom Funded Trading Combine"
phase: funded
size_label: "$250K"
size_usd: "250000.00"
instruments: [MNQ, NQ]
platforms: [TopstepX, "Quantower (ProjectX connection)", "TopstepX/ProjectX API"]
drawdown: { amount_usd: "10000.00", type: other, stops_trailing_at: "type not stated; After first Payout, MLL sets to $0" }
daily_loss_limit: { amount_usd: null, kind: none }   # not stated in the XFA table
profit_target_usd: null
consistency_rule: { text: "Path: Standard", threshold_pct: null }
max_position: { minis: 25, micros: 250, scaling: "Below $1,500 = 3 Lots; $1,500 - $2,000 = 4; $2,000 - $3,000 = 5; $3,000 - $4,500 = 10; $4,500 - $6,000 = 15; $6,000 - $8,000 = 20; Above $8,000 = 25 Lots" }
min_trading_days: null
trading_day_reset: { time: "17:00", tz_as_published: "CT" }
copy_trading_allowed: conditional
automated_trading_allowed: conditional
automation_rule_text: "Yes, with conditions. Topstep won't help set them up or cover malfunctions. Review Prohibited Conduct and Prohibited Trading Strategies first."
vps_allowed: no                     # ToU §28 + API Access page; applies to all account types
personal_device_required: yes       # "All trading activity must originate from your personal device."
status: partial
not_confirmed_fields: [platforms, drawdown.type, daily_loss_limit]
sources:
  - { field: vps_allowed, url: "https://www.topstep.com/terms-of-use", accessed: "2026-10-23" }
  - { field: personal_device_required, url: "https://help.topstep.com/en/articles/11187768-topstepx-api-access", accessed: "2026-10-23" }
  - { field: all_values, url: "https://help.topstep.com/en/articles/15520357-topstep-labs", accessed: "2026-10-23" }
```

## 9. Unverified / gaps / conflicts

| # | Field | Status | What is missing / found | Where searched |
|---|---|---|---|---|
| 1 | Day-lock time for best day (TC consistency) and winning/qualifying days (XFA payout) | **conflict** | S4: "At 3:10 PM CT, that day's value locks". S21: "A day locks in at 4:00 PM CT" / "Days lock in at 4:00 PM CT". The trading-day reset (5:00 PM CT) is not affected | S4, S21 |
| 2 | LFA max micros | unverified | "The Micro to Mini ratio functionality … is not currently available for the Live Funded Account." No LFA micro limit is published (S16 lists LFA MCL "3/6/9", but nothing for MNQ) | S1, S6, S7, S8, S16, S24 |
| 3 | Copying to/from accounts at **other firms** | unknown | Not mentioned. Copying with other people is prohibited (S25 §27), and hedging includes "accounts held with different Topstep Parties" | S1, S5, S11, S13, S15, S25 |
| 4 | Third-party / external trade copiers | unknown | Only the built-in TopstepX Trade Copier is documented. Third-party tools may connect through the API "at your own risk" (S12) | S11, S12, S15 |
| 5 | Webhooks / TradingView alerts to orders | not mentioned | TopstepX "Cannot connect to external platforms like TradingView". Webhooks are not named | S11, S12, help-center crawl |
| 6 | Semi-automated trading | not mentioned | — | full help-center crawl |
| 7 | NinjaTrader / Tradovate / Rithmic | not offered | S17: "TopstepX™ is the only available trading platform." S23 (dated June 26, 2025) still says "Accounts trading on platforms other than TopstepX are also subject to a Daily Loss Limit as outlined below" with no such section. Stale text, flagged for the PO | S17, S23, help-center crawl, official-domain web search |
| 8 | **Server-side order routing** | finding (not a gap) | S12: orders must "originate from your personal device". "Routing or relaying orders on your behalf" is listed as "Not allowed on a private server". VPS/VPN prohibited (S25 §28). This affects any All Win server-side adapter for Topstep | S12, S25 |
| 9 | XFA scaling band boundaries | not confirmed | The S6 image shows overlapping labels ("$1,500 - $2,000", "$2,000 - $3,000"), so the lot count at exactly $1,500 / $2,000 / $3,000 / $4,500 is not stated. The values come from an image, not text. The site version (S23, `XFA-Rules_Scaling-plan.avif`) could not be read (AVIF; no decoder available) | S6, S16, S23 |
| 10 | TC minimum trading days | no explicit rule | Only "You can pass in as few as two days" (S1), which follows from the 55% consistency rule | S1, S4 |
| 11 | Labs products: platform, DLL breach kind, $250K XFA MLL type and DLL, $50K Static XFA scaling thresholds ("2/3/4"), Labs LFA MLL | partial | The Labs page does not restate these | S19 |
| 12 | Pro Account | not modeled in YAML | MLL "mirrors the Express Funded Account" (no amount table). DLL $1,000 / $2,000 / $3,000. PO to decide whether a preset is needed | S20 |
| 13 | Time zone wording | note | Pages use "CT". S9 also writes "3:10 PM CST" once, and S11 writes "4:00 PM – 5:00 PM CST" for the contract roll | S9, S11 |
| 14 | Sizes 25K / 300K (standard) | not offered | Standard sizes are 50K / 100K / 150K only. 25K and 250K exist only as Topstep Labs products | S22, S19 |

## 10. Changelog

- 2026-10-23 — initial research (Lupa)
- 2026-10-23 — format rev. 3: added `vps_allowed: no` and `personal_device_required: yes` (with sources) to all 15 YAML blocks; added the §7 "Device / hosting rules" subsection with verbatim quotes; the rule does not differ by plan (Lupa)
