# Top One Futures — Prop Firm Rules Research

| | |
|---|---|
| **Firm** | Top One Futures (Top One Futures, LLC, Cheyenne WY). Futures only. Not the same as "Top One Trader" (`toponetrader.com`), a separate brand linked from the homepage. That brand was not researched |
| **Official domains** | `www.toponefutures.com` (main site; plain curl works) · `help.toponefutures.com` (official help center, Intercom-hosted; linked from the homepage, alias `intercom.help/top-one-futures`) · `portal.toponefutures.com` (trader dashboard, login, not used) · `checkout.toponefutures.com` (checkout, not used) |
| **Research date** | 2026-10-23 (all URLs below accessed 2026-10-23) |
| **Researcher** | Lupa (Prop Firm Rules Researcher) |
| **File status** | `partial`. Every core field is verified on official pages for the six plans currently sold (Elite Daily, Elite ACCESS, Elite ACCESS Intraday, Instant Sim Funded, Ignite, LaunchPad X-Daily Spark) in all sizes, plus Pre-Live/Live. Still open: 7 conflicts (two 150K DLL values, copying to/from other firms, same-IP rule vs copying, Ignite flatten time, account limits) and gaps (per-plan platform list, Elite Daily lock trigger). The "personal device" rule was closed in R4 (`no`). See §9 |
| **Instruments covered** | MNQ, NQ. Both are listed: "/NQ NASDAQ-100" and "/MNQ Micro NASDAQ-100" (Q3). Micros are 1/10 of a mini: "Minis are standard futures contracts, while micros are one-tenth the size of minis. You can trade minis, micros, or a combination of both, as long as you stay within your limit." (G1) |
| **Platforms (for routing)** | **Tradovate/NinjaTrader** (Tradovate-backed accounts, usable in NinjaTrader Desktop/Web, Tradovate Web/Desktop and TradingView), **BlackArrow**, **WealthCharts**. The platform is chosen at checkout and is permanent per account. No Rithmic, CQG or ProjectX found. See §2 |
| **Automation** | **Forbidden** on all accounts (§6) |
| **Copying own accounts** | **Conditional**: only between accounts of the same program and the same size (§5) |
| **VPS** | **Conditional**: allowed with a static IP after notifying support. VPN not allowed (§7) |

Evidence (raw text of every cited page, URL on line 1; raw HTML in `raw/`): `/tmp/claude-1003/-srv-www-allwin--maestri-roles-6bf77a80-a1ed-43dc-bc00-08a480f178a5/33652d13-4dae-46b6-ae06-44a72b375517/scratchpad/r2/top-one-futures/` (help-center articles in `hc/`, site pages as `home.txt` and `site-*.txt`). This is temporary scratch storage, so the evidence must be captured again at re-verification. Help-center pages say "Updated today / this week / over 3 weeks ago" (relative dates only).

**Programs found.** The homepage ("Choose Your Account") sells four product cards on 2026-10-23: **Elite Daily**, **Elite Access** (with a "Drawdown mode EOD / Intraday" toggle, that is, **Elite ACCESS** and **Elite ACCESS Intraday**), **INSTANT Sim Funded** and **IGNITE Instant Funding**. It also links the **LaunchPad**, which sells **X-Daily Spark** (limited release of 1,500 accounts, $50K only). All other products have sizes $25K, $50K, $100K and $150K. There is no size above $150K. Every sim funded path can lead to **Pre-Live / Live** (§3.7).

Not on sale on 2026-10-23, low priority (§3.8): **Elite** (1-Step Elite Evaluation / Elite Sim Funded) and **S2F Sim Pro**. Both still have help-center articles but no homepage card. **X-Ultra** is marked "no longer available for new purchases". The **Instant Sim Funded legacy** generation applies to accounts bought before 7/27/2026. **Last Trader Standing** is a free competition, not a funding plan.

**Top One Futures' terms.** "MLL" = Max Loss Limit = Max (trailing) drawdown. "DLL" = Daily Loss Limit, also called "daily drawdown". "Soft breach" = paused for the rest of the day. "Hard breach" = account closed.

### Main source list (all accessed 2026-10-23)

Help-center base: `https://help.toponefutures.com/en/articles/`

| Key | URL |
|---|---|
| HOME | https://www.toponefutures.com/ (product cards) |
| SITE-EA | https://www.toponefutures.com/elite-access |
| SITE-LP | https://www.toponefutures.com/launchpad |
| SITE-CP | https://www.toponefutures.com/coupon-code-pricing (product cards, same layout as home) |
| TC | https://www.toponefutures.com/terms-and-conditions ("Last updated: June 10, 2026") |
| G1 | https://help.toponefutures.com/en/articles/10906950-maximum-contracts-explained |
| G2 | https://help.toponefutures.com/en/articles/10907081-daily-drawdown-what-you-need-to-know |
| G3 | https://help.toponefutures.com/en/articles/10907150-understanding-the-drawdown-lock |
| G4 | https://help.toponefutures.com/en/articles/10907463-what-is-the-maximum-number-of-accounts-i-can-trade |
| G5 | https://help.toponefutures.com/en/articles/10907901-can-i-hold-trades-overnight |
| G6 | https://help.toponefutures.com/en/articles/10907920-can-i-use-ea-s-bots |
| G7 | https://help.toponefutures.com/en/articles/10907951-can-i-trade-news |
| G8 | https://help.toponefutures.com/en/articles/10907989-inactivity-rule-explained |
| G9 | https://help.toponefutures.com/en/articles/10908016-minimum-trade-time-rule-explained |
| G11 | https://help.toponefutures.com/en/articles/11021584-prohibited-trading-practices |
| G12 | https://help.toponefutures.com/en/articles/11165893-trading-hours-at-top-one-futures |
| G13 | https://help.toponefutures.com/en/articles/11187841-are-there-ip-address-restrictions |
| G14 | https://help.toponefutures.com/en/articles/11532971-what-is-the-limit-for-funded-accounts-per-household |
| G15 | https://help.toponefutures.com/en/articles/11680891-copy-trading-policy |
| G16 | https://help.toponefutures.com/en/articles/14847123-account-ownership-and-registration-policies |
| L1 | https://help.toponefutures.com/en/articles/11003130-path-to-trading-live |
| PY1 | https://help.toponefutures.com/en/articles/11024436-understanding-lock-upon-payout |
| Q1 | https://help.toponefutures.com/en/articles/13449146-top-one-futures-account-comparison-elite-vs-instant-vs-s2f-sim-pro-vs-ignite-accounts |
| Q2 | https://help.toponefutures.com/en/articles/14617600-differences-between-elite-elite-access-and-elite-daily-accounts |
| Q3 | https://help.toponefutures.com/en/articles/11021492-trading-instruments-available-at-top-one-futures |
| Q4 | https://help.toponefutures.com/en/articles/17149341-can-i-switch-my-trading-platform-tradovate-ninjatrader-or-blackarrow |
| Q5 | https://help.toponefutures.com/en/articles/17635087-no-daily-loss-limit-add-on-trade-without-daily-limits |
| Q6 | https://help.toponefutures.com/en/articles/11510286-why-is-my-account-paused |
| ED1–ED10 | Elite Daily V2: 15056075 (overview), 15056388 (DLL evaluation), 13904706 (DLL funded), 13904722 (MLL), 13904734 (profit target), 15056475 (40% consistency), 13904824 (scaling), 13904852 (news), 13904864 (max accounts), 15056446 (payouts) |
| EA1–EA11 | Elite ACCESS: 14595016 (overview), 14595053 (scaling), 14595073 (profit target), 14595108 (buffer), 14595138 (DLL), 14595145 (MLL EOD), 14595151 (consistency 40%), 14595155 (min profitable days), 14595161 (payouts), 14595166 (progression 50%), 14595174 (news) |
| EI1–EI10 | Elite ACCESS Intraday: 17059267 (overview), 17059431 (scaling), 17059492 (profit target), 17059548 (buffer), 17059623 (DLL), 17059654 (MLL evaluation EOD), 17059688 (MLL funded intraday), 17059698 (consistency), 17059750 (min profitable days), 17059862 (news) |
| IS1–IS5 | Instant Sim Funded: 11020805 (overview), 10907027 (scaling), 10908119 (trailing drawdown), 10908767 (consistency 20%), 12718038 (payouts) |
| IG1–IG3 | Ignite: 12671168 (overview), 12707415 (payouts), 12718011 (15% consistency) |
| XS1–XS8 | X-Daily Spark: 17678693 (overview), 17678737 (evaluation), 17678749 (DLL), 17678752 (MLL), 17678755 (activation/limits), 17678757 (payouts), 17678769 (contracts/scaling), 17678771 (other rules) |
| XU1–XU2 | X-Ultra: 17244414 (overview), 17244951 (other rules) |
| EL1–EL6 | Elite (legacy): 11020730 (overview), 10907134 (drawdown), 10907168 (no eval consistency), 10907263 (25% consistency), 10996306 (scaling), 10907192 (one-day pass) |
| P1–P9 | Platforms: 12828004 (Tradovate first account), 12828473 (Tradovate → NinjaTrader 8), 12853687 (Tradovate → TradingView), 12847147 (ATM strategies in NinjaTrader), 16847893 (BlackArrow first account), 17512965 (BlackArrow copy trading), 17320795 (WealthCharts first account), 17333170 (WealthCharts copy trading), 12840505 (NinjaTrader maintenance) |

For ED/EA/EI/IS/IG/XS/XU/EL/P keys, the full URL is the base plus the article's full slug. Every slug is listed in the evidence file `arts.txt`, and every YAML block below cites full URLs.

---

## 1. Header notes

- The help-center home lists 16 collections and 131 articles. All of them were downloaded with curl and saved. Every rule article was read.
- **Hierarchy used.** Product-specific help-center articles are the main source. General help-center articles, the homepage cards and the T&C are cross-checks. If two official pages disagree, both values are recorded and the field is marked `conflict` (§9).
- **Homepage "strike-through" pairs.** Some homepage/coupon cards show two values in one cell. Examples: Ignite 100K "Trailing max drawdown `$3,000 $4,000`", Ignite 150K "`$4,500 $6,000`", and Max Accounts "`6 | 10`" / "`10 | 20`" / "`3 | 5`". In the HTML the first value is wrapped in `<em>`, which looks like an old or crossed-out value. These were not used as rule values. Where the second value differs from the help center, it is noted in §9.
- **Promotional account limits.** "Note: the limits above are our standard account limits. During certain promotional periods, we may allow traders to hold more accounts than these standard limits. This applies only to accounts purchased during that promotion. Please check our homepage for the account limits currently in effect, or ask our support team for the current promotional limits." (G4). The homepage currently advertises "Copy trade up to 20 accounts" (Elite Access promo) and "Copy Trade 10 Accounts" (Ignite).

## 2. Platforms

| Plan / phase | Platforms / connections as published | Source |
|---|---|---|
| All products (firm-wide) | The platform is chosen at checkout: "you can select your preferred trading platform, such as Tradovate / NinjaTrader or BlackArrow". The choice is permanent: "Once an account is generated on a specific platform, platform switches are strictly not permitted." A funded account "is automatically issued on the exact same platform used during the evaluation" | Q4 |
| Tradovate/NinjaTrader accounts | Tradovate Web/Desktop ("Tradovate Prop - Simulation"). NinjaTrader Desktop (Windows; "MacOS is not supported") or NinjaTrader Web, logged in with the Tradovate credentials ("NinjaTrader Prop - Simulation"). TradingView through the Tradovate broker panel ("Demo" environment) | P1, P2, P3 |
| BlackArrow accounts | BlackArrow platform. Credentials come from BlackArrow by email | P5 |
| WealthCharts accounts | WealthCharts ("WealthCharts Prop" tab → Top One Futures). Credentials come from WealthCharts | P7 |
| Elite ACCESS (site page) | The `/elite-access` configurator shows "Choose a trading platform: Tradovate/NinjaTrader" (only option shown) | SITE-EA |
| Pre-Live / Live | Not published. Live accounts are "set up with the broker in 2 to 3 weeks". Pro data fees are listed for CME/NYMEX/COMEX/CBOT | L1 |

- Per-plan availability (which of the three platforms each plan offers) is **not published** on any public page. It is only visible at checkout, which was not used. Every YAML block lists the firm-wide set and §9 records the gap.
- The homepage shows Tradovate, NinjaTrader and TradingView logos. No Rithmic, CQG, DXFeed, ProjectX or Quantower connection appears on any page read.
- "All Tradovate-based accounts (including Top One Futures accounts) depend on NinjaTrader's backend systems." (P9)

**Product impact (for the PO/Architect, not a rule value).** Top One Futures accounts on "Tradovate/NinjaTrader" connect to NinjaTrader 8 Desktop with the Tradovate credentials from the dashboard. NT8 is therefore a supported route for these accounts. Automated order entry (bots/EAs/scripts) is forbidden (§6), and copying is limited to same-program, same-size accounts (§5).

## 3. Plans

Money values are as published. "Minis / micros" use the firm's 1:10 ratio (G1). Unless noted, all sizes are $25K / $50K / $100K / $150K.

### 3.1 Elite Daily (V2) — evaluation and funded (ED1)

"The Elite Daily V2 Plan is a one phase evaluation model … Available account sizes: $25,000, $50,000, $100,000 and $150,000." (ED1)

| Field | $25K | $50K | $100K | $150K | Source |
|---|---|---|---|---|---|
| **Evaluation** | | | | | |
| Profit target | $1,500 (6%) | $3,000 (6%) | $6,000 (6%) | $9,000 (6%) | ED1, ED5 |
| Max drawdown (MLL), EOD trailing | $1,000 | $2,000 | $3,000 | $4,500 | ED1, ED4 |
| DLL | $500 | $1,000 | $1,250 | **$1,850** (conflict, §9 C2) | ED1, ED2 |
| DLL kind (eval) | "Violating the DLL during evaluation results in a pause; your account will automatically unpause the next trading day." | | | | ED2 |
| Consistency | 40%: "No single trading day may account for more than 40% of your total profit at the moment the profit target is reached." | | | | ED6 |
| Max contracts | 1 / 10 micros | 3 / 30 | 5 / 50 | 7 / 70 | ED1, ED7 ("no scaling restrictions during the evaluation phase") |
| Min trading days | "although there is no minimum day requirement, the 40% Consistency Rule must still be respected" | | | | ED5 |
| **Funded** | | | | | |
| Trailing drawdown (EOD) | $1,000 | $2,000 | $2,500 | $3,500 | ED1, ED4 |
| Drawdown locks at | $25,100 | $50,100 | $100,100 | $150,100 | ED1 |
| DLL (soft breach) | $500 | $1,000 | $1,250 | **$1,850** (conflict, §9 C2) | ED1, ED3 |
| Consistency | None: "There is no consistency rule once funded." | | | | ED6 |
| Max contracts (scaling) | 1 | 1, 2, 3 | 2, 3, 4, 5 | 3, 4, 5, 6, 7 | ED1; tiers in ED7 (§8.1) |
| Min trading days (funded) | 1 | 1 | 1 | 1 | ED1 |
| Payout buffer / max per request | $1,500 / $750 | $2,500 / $1,000 | $3,500 / $1,500 | $4,500 / $2,250 | ED1, ED10 |
| News (funded only) | "No trades may be executed or closed 2 minutes before and 2 minutes after major high-impact news events." | | | | ED1, ED8 |
| Max accounts | "a trader's household may trade a maximum of 5 Elite Daily accounts at any time. This includes evaluation accounts, funded accounts, or a combination of both." | | | | ED9 |

- **Drawdown mechanics (ED4).** "the MLL does not trail intraday. It updates after the trading session closes, based on your end of day balance, and locks in at your starting balance + $100 once your account has grown enough. There are no warnings when the MLL is breached." The balance that triggers the lock is **not stated** for Elite Daily. ED4's example is "$100K Funded … You close the day at $108,000 … New breach level $100,100". Recorded as a gap (§9 G3).
- **DLL calculation (ED2/ED3).** It is based on the previous day's closing balance. "Realized losses / Unrealized/floating losses / Commissions and fees … Both balance and equity are monitored." When the DLL is hit, "the system will automatically execute market orders to liquidate all open positions" (ED3).
- **No-DLL add-on.** "You can select the No Daily Loss Limit add-on at checkout for about 30% more than the base account price. The add-on can only be chosen during the initial checkout" (ED2).
- **Payout rules.** There is a 50% Daily Progression Rule: "After your first payout, at least 50% of every future payout request must come from new profits generated since your previous payout." The minimum payout is $500 (ED10). The path to Live needs "5 successful payouts" (ED10, L1).
- **Purchase limits (ED1/ED9).** "up to 10 evaluation accounts within any rolling 30-day period", "reset a maximum of 12 times within a 30-day window".

### 3.2 Elite ACCESS — evaluation and funded, EOD drawdown (EA1)

"The Elite ACCESS account is a fast-track evaluation … no daily loss limit during the evaluation … You have 30 days to qualify for a funded account." (EA1)

| Field | $25K | $50K | $100K | $150K | Source |
|---|---|---|---|---|---|
| **Evaluation** | | | | | |
| Profit target (6%) | $1,500 | $3,000 | $6,000 | $9,000 | EA1, EA3 |
| Max drawdown (EOD trailing) | $1,000 | $2,000 | $3,000 | $4,000 | EA1, EA6 |
| DLL | None | None | None | None | EA1, EA5 |
| Consistency | "There are no consistency requirements." | | | | EA3 |
| Max contracts | 1 mini / 10 micros | 3 / 30 | 5 / 50 | 7 / 70 | EA1, G1 (no scaling in evaluation) |
| Min days / time limit | "Minimum of 1 trading day … 30 days to pass." | | | | EA1 |
| **Funded** | | | | | |
| Max drawdown (EOD trailing) | $1,000 | $2,000 | $3,000 | $3,500 | EA1, EA6 |
| Drawdown lock level | $25,100 | $50,100 | $100,100 | $150,100 | EA6 |
| DLL (soft breach) | $500 | $1,000 | $1,250 | **$1,750** (conflict, §9 C1) | EA1, EA5, G2 |
| Consistency (funded) | 40%: "Your largest single trading day must not exceed 40% of your total profits in your current payout cycle." | | | | EA7 |
| Max contracts (fully scaled) | 1 | 3 | 5 | 7 | EA1, EA2 (tiers in §8.2) |
| Min profitable days per payout | 5 days of ≥ $200 | ≥ $250 | ≥ $300 | ≥ $350 | EA8 |
| Payout buffer / max per request | $1,500 / $1,000 | $2,000 / $1,500 | $3,000 / $2,000 | $4,500 / $2,500 | EA1, EA9 |
| News (funded only) | 2 minutes before and after high-impact news | | | | EA11 |
| Max accounts | "A trader may maintain no more than 10 active funded accounts at any given time." Elite ACCESS and Intraday share one limit of 10 (G4) | | | | EA1, G4 |

- **Drawdown (EA6).** "The drawdown is calculated based on your end-of-day (EOD) balance … It trails upward as your account reaches new highs. It does not move down if your balance decreases." "Once your account reaches a certain level, the trailing drawdown locks at breakeven + $100". "Intraday drops below the level do not count, unless the day closes below it." "Hard breach: if your account closes below the allowed drawdown level, the account is terminated". The lock trigger is given in EI7: "the regular Elite ACCESS account, where the drawdown locks as soon as the balance reaches starting balance + drawdown + $100."
- **DLL (EA5).** "Soft breach: if the Daily Loss Limit is reached or exceeded, your account is paused for the rest of the day. Open positions may be closed, and the account becomes active again after the rollover period." "The DLL is calculated based on your starting balance for the day." Closed losses, open (floating) losses, commissions and fees all count. A No-DLL add-on exists: "The evaluation price stays the same, and the activation fee is about 30% higher."
- **Payout rules.** The balance must be above the buffer + $500 (EA4). The 50% progression rule applies from the 2nd payout (EA10). The minimum payout is $500 and the path to Live needs 3 payouts. "Only one account transitions to live … your remaining accounts are closed at that point." (EA1)

### 3.3 Elite ACCESS Intraday — evaluation EOD, funded intraday drawdown (EI1)

"The difference with the regular Elite ACCESS account: the funded phase uses an Intraday Trailing Drawdown (the evaluation keeps the End-of-Day drawdown) and the activation fees are lower." (EI1)

| Field | $25K | $50K | $100K | $150K | Source |
|---|---|---|---|---|---|
| **Evaluation** | | | | | |
| Profit target (6%) | $1,500 | $3,000 | $6,000 | $9,000 | EI1, EI3 |
| MLL (EOD trailing) | $1,000 | $2,000 | $3,000 | $4,000 | EI1, EI6 |
| Lock threshold → locks at | $26,100 → $25,100 | $52,100 → $50,100 | $103,100 → $100,100 | $154,100 → $150,100 | EI6 |
| DLL | None | None | None | None | EI5 |
| Consistency | none in evaluation | | | | EI3, EI8 |
| Max contracts | 1 / 10 | 3 / 30 | 5 / 50 | 7 / 70 | EI1, EI2 |
| **Funded** | | | | | |
| Max drawdown (intraday trailing, TIDD) | $1,000 | $2,000 | $3,000 | $3,500 | EI1, EI7 |
| DLL (soft breach) | $500 | $1,000 | $1,250 | **$1,750** (conflict, §9 C1) | EI1, EI5, G2 |
| Consistency (funded) | 40% (same text as Elite ACCESS) | | | | EI8 |
| Max contracts (fully scaled) | 1 | 3 | 5 | 7 | EI2 (tiers identical to Elite ACCESS, §8.3) |
| Min profitable days per payout | 5 × $200 | 5 × $250 | 5 × $300 | 5 × $350 | EI9 |
| Payout buffer / max per request | $1,500 / $1,000 | $2,000 / $1,500 | $3,000 / $2,000 | $4,500 / $2,500 | EI1 |
| News (funded only) | 2 minutes before/after high-impact news | | | | EI10 |

- **Funded TIDD (EI7).** "Your maximum allowable loss trails your highest intraday equity, including open (unrealized) trades. It is tracked live, tick by tick, in real time." "It does not reset at the end of the day". **When it stops trailing:** "On funded Elite ACCESS Intraday accounts, the TIDD keeps trailing until you request your first payout. After your first payout request, it locks at starting balance + $100 and stops trailing." "Hard breach: if your equity drops at or below the Maximum Loss Limit threshold during the funded phase, the account is permanently breached".
- **DLL (EI5).** "Open positions are automatically closed, and the account becomes active again after the market rollover." "It resets once per day at the end-of-day rollover".

### 3.4 Instant Sim Funded — funded only, no evaluation (IS1)

"Purchased before July 27, 2026: the original Instant Sim Funded parameters. Purchased on or after July 27, 2026: the updated Instant Sim Funded 2.0 rules." (IS1). The YAML (§8.4) covers **2.0**, which is the version sold now. Legacy values are shown here for existing accounts.

| Field | $25K | $50K | $100K | $150K | Source |
|---|---|---|---|---|---|
| Max trailing drawdown (EOD), 2.0 | $1,000 | $2,000 | $4,000 | $5,250 | IS1, IS3 |
| Max trailing drawdown (EOD), legacy | $1,000 | $2,000 | $3,000 | $4,500 | IS1, IS3 |
| Balance to lock → locks at (2.0) | $26,100 → $25,100 | $52,100 → $50,100 | $104,100 → $100,100 | $155,350 → $150,100 | IS3 |
| Balance to lock → locks at (legacy) | $26,100 → $25,100 | $52,100 → $50,100 | $103,100 → $100,100 | $154,600 → $150,100 | IS3 |
| DLL (soft breach), 2.0 | $625 | $1,250 | $2,500 | $3,000 | IS1, G2 |
| DLL (soft breach), legacy | $625 | $1,250 | $2,500 | $3,750 | IS1, G2 |
| Consistency | 20%: "no single day's profit may exceed 20% of your total profits" (for payouts) | | | | IS1, IS4 |
| Max contracts | 1 / 10 | 3 / 30 | 5 / 50 | 7 / 70 | G1 (no scaling: IS1, IS2) |
| Max per payout (2.0 / legacy) | $1,000 / $1,500 | $2,000 / $2,500 | $2,500 / $3,000 | $3,000 / $3,500 | IS1, IS5 |
| Max accounts | "3 (100K or 150K sizes) or 5 (25K or 50K sizes)", all the same size | | | | G4, G14 (conflict with Q1 for 25K/50K, §9 C7) |

- **Drawdown (IS3).** "It tracks the highest account balance achieved by the end of the day (5:00 pm EST)". "the drawdown moves up as your account balance reaches new highs, but never moves down". "Once your profit reaches the drawdown amount plus $100, the drawdown locks at your starting balance + $100". "The lock also activates after your first payout." (IS1)
- **Payout target (IS5).** "First payout: 6% of your initial account balance. Second payout: 5% on top of your new balance … Third payout and all future payouts: 4%".
- "For the consistency rule, a trading day ends at 5:00 pm EST." (IS1)

### 3.5 Ignite Instant Funding — funded only, no evaluation (IG1)

| Field | $25K | $50K | $100K | $150K | Source |
|---|---|---|---|---|---|
| Max loss limit (EOD trailing) | $1,000 | $2,000 | $4,000 | $6,000 | IG1 |
| Lock triggers at → locks at | $26,100 → $25,100 | $52,100 → $50,100 | $104,100 → $100,100 | $156,100 → $150,100 | IG1 |
| DLL (soft breach) | $500 | $1,000 | $2,000 | $3,000 | IG1, G2 |
| Consistency | 15%: "Your single best trading day cannot account for more than 15% of your total accumulated profit at the time of your payout request." | | | | IG2, IG3 |
| Max contracts | 1 mini (10 micros) | 3 (30) | 5 (50) | 7 (70) | IG1, G1 (fixed, no scaling) |
| Profit target per payout (5%) | $1,250 | $2,500 | $5,000 | $7,500 | IG1, IG2 |
| Max per payout / min payout | $500 / $250 | $1,000 / $250 | $1,500 / $250 | $2,000 / $250 | IG2 |
| Min trading days | None | None | None | None | IG2 |
| Max accounts | "You may hold and copy trade up to 10 Ignite accounts of the same size." (conflict with Q1 "6", §9 C6) | | | | IG1, G4 |

- "If your DLL is hit, your account is paused for the rest of the trading day and resumes when markets reopen at 6:00 pm ET." (IG1). A No-DLL add-on is available at checkout (IG1, Q5).
- "Overnight trading and expert advisors or bots are not allowed. All trades must be closed by 4:00 pm ET." (IG1). This conflicts with the general 4:10 PM EST (§9 C5).
- The 15% rule "applies to new Ignite accounts opened under the updated terms. Existing accounts that use the ESS rule remain unchanged" (IG3). ESS is not defined on any page read.

### 3.6 LaunchPad X-Daily Spark — $50K only, limited release (XS1)

"X-Daily Spark is a one-step $50,000 evaluation … There is no consistency rule on the evaluation or on the funded account." "Only 1,500 X-Daily Spark accounts are available." (XS1). SITE-LP showed "1,500 of 1,500 left" on 2026-10-23.

| Field | Evaluation | Funded | Source |
|---|---|---|---|
| Profit target | $3,000 (6%). Add-ons: $2,500 (+$15) or $2,000 (+$30) | none | XS1, XS2 |
| Max drawdown | $2,000 EOD trailing ("static with add-on": "Your breach level stays at $48,000") | $2,000 EOD trailing, "locks at $50,100" ("You close the day at $52,100 or higher (funded) → Breach level locks at $50,100") | XS4 |
| DLL | None | $1,000. "Soft breach: Trading is paused for the rest of the trading day." Removable for +$30 at activation | XS3 |
| Consistency | None | None | XS1 |
| Max contracts | 3 minis (30 micros), no scaling | $0–$1,499: 1 mini (10 micros); $1,500–$1,999: 2 (20); $2,000+: 3 (30) | XS7 |
| Min trading days | "Min trading days to pass: None", 30-day limit | "Min trading days before a payout: None" | XS1, XS2 |
| Payout | — | Daily; min $500, max $1,000, buffer $3,000; 80/20 split; 50% progression rule | XS6 |
| News | no news rule | "no trades may be opened or closed 2 minutes before and 2 minutes after major high-impact news events" | XS8 |
| Max accounts | "3 in total (evaluation and funded combined)" | same | XS5 |

"EAs and bots: Not allowed." "Overnight holding: Not allowed. All positions must be closed before the end of the trading day at 4:10 PM ET." (XS8)

### 3.7 Pre-Live / Live — all sim funded plans (L1)

"After 3 successful payouts from one sim funded account, a risk manager will review your account and either keep you in the sim funded environment for further review, or transition you to a live account." Elite Daily needs 5 payouts (L1, ED10).

| Live acct size | Starting balance | MLL (EOD drawdown) | EOD balance to trigger DD lock | Drawdown lock | DLL | Max contracts |
|---|---|---|---|---|---|---|
| $25K | $0 | $1,000 | $1,100 | $100 | None | 1 |
| $50K | $0 | $2,000 | $2,100 | $100 | None | 3 |
| $100K | $0 | $3,000 | $3,100 | $100 | None | 5 |
| $150K | $0 | $4,000 | $4,100 | $100 | None | 7 |

(L1, table copied as published.) Pre-Live has "the same drawdown levels and contract sizing as live", "No Consistency Rule", and "There is no DLL on Pre-Live". The scaling table is in §8.7. Account limits: "if you have an active Top One Futures Live Funded account, you are not permitted to hold any additional Sim Funded accounts" (G4). In Pre-Live, "Trading is only allowed on ONE Evaluation account (no sim funded)" (L1). Live scales "up to 5" accounts (L1).

### 3.8 Products not on sale on 2026-10-23 (low priority, no YAML)

- **Elite (1-Step Elite Evaluation / Elite Sim Funded)** (EL1–EL6, G1, G2). Still described in Q1/Q2 but no homepage card. The profit target is 6% ($1,500 / $3,000 / $6,000 / $9,000). The max trailing drawdown (EOD) is $1,000 / $2,000 / $3,000 / $4,500 and locks at starting balance + $100 (balance to lock $26,100 / $52,100 / $103,100 / $154,600). The DLL is 2.5% ($625 / $1,250 / $2,500 / $3,750) and is a soft pause. There is no evaluation consistency rule; the funded account has a 25% rule. Max contracts are fixed at 1/10, 3/30, 5/50 and 7/70. EL2 adds: "while the Max Trailing Drawdown is calculated based on your end-of-day balance, you cannot drop below the drawdown threshold at any time intraday." Account limits disagree: Q1 says "Unlimited" evaluation and 3 funded, while G4 says "3 (100K or 150K sizes) or 5 (25K or 50K sizes)".
- **S2F Sim Pro** (Q1, G1, G2). No collection of its own. Q1 gives "Intraday Trailing" 3% drawdown, 2% daily drawdown, "20% ESS" and "10 days (7 days profitable)". The DLL is $500 / $1,000 / $2,000 / $3,000 (G2). Max contracts are 1/10, 4/40, 8/80 and 12/120 (G1).
- **X-Ultra** (XU1): "X-Ultra accounts are no longer available for new purchases." Sizes $50K / $100K / $150K. The DLL is a **hard breach** ($1,000 / $1,500 / $2,000): "On X-Ultra accounts, reaching the DLL is a hard breach and the account fails immediately" (G2).
- **Last Trader Standing** (17678782, 17678785): a free 10-day competition on a simulated $100K account with "Accounts per person 1". It is not a funding plan.

## 4. Trading-day reset

| Item | Published text | Source |
|---|---|---|
| Session | "You can place trades between 6:00 PM EST and 4:10 PM EST the next day, Monday through Friday. … There is a daily 1-hour break from 5:00 PM to 6:00 PM EST". "Final session of the week: 6:00 PM EST on Thursday – 4:00 PM EST on Friday". "Markets reopen Sunday at 6:00 PM EST" | G12 |
| Forced flat | "All open trades are automatically closed by 4:10 PM EST every trading day. … the system will forcibly liquidate all open positions at that time." "From 4:10 PM to 6:00 PM EST, trading is paused." | G12, G5 |
| Trading day (LaunchPad) | "A trading day runs from 6:00 PM ET to 4:10 PM ET. The daily loss limit, end of day balances and contract scaling are all based on this trading day." | XS8 (same text for X-Ultra: XU2) |
| EOD / consistency day (Instant, Elite) | "a trading day ends at 5:00 pm EST" (IS1, IS4, EL4). The drawdown uses the "highest closed balance at the end of the day (5:00 pm EST)" (IS3) | IS1, IS3, IS4, EL4 |
| DLL reactivation | "When the new trading session starts, account reactivation is not instantaneous. System processing may take up to 5 to 10 minutes before trading is re-enabled" (G2). Ignite: "resumes when markets reopen at 6:00 pm ET" (IG1). Paused for DLL → reactivated "At the next trading session" (Q6) | G2, IG1, Q6 |
| Ignite close | "All trades must be closed by 4:00 pm ET." Conflicts with 4:10 PM EST (§9 C5) | IG1 |
| Holidays / early close | No holiday-schedule article was found. Friday's session closes at 4:00 PM EST (G12) | §9 G6 |

The firm writes both "EST" and "ET". The text is recorded as published; no DST conversion was applied. In the YAML, the reset is `18:00 EST` (session open) for the Elite family, `18:00 ET` for Ignite and X-Daily Spark, and `17:00 EST` for Instant Sim Funded (the published day end for EOD/consistency).

## 5. Trade copying stance

**Same person's accounts: allowed only between accounts of the same program and the same size.**

- "Top One Futures allows you to copy trades to manage multiple accounts efficiently. However, to maintain risk integrity, copy trading is strictly limited to accounts of the exact same type and exact same size. Mixing different account programs, sizes, or trying to bypass these rules through manual mirroring or correlated assets is a strict violation of our terms." (G15)
- "You can only copy trades between accounts that belong to the identical program and share the same size." Allowed examples: "Elite ACCESS $50K to another Elite ACCESS $50K", "Ignite Instant Funding $100K to another Ignite Instant Funding $100K". Prohibited: "Elite ACCESS with Elite Daily", "Instant Sim Funded with Ignite", "Instant Sim Funded $50K with Instant Sim Funded $100K". (G15)
- Manual mirroring and correlated assets also count: "If you buy or sell the same instrument across mismatched accounts, even if it is minutes or an hour later, it is still classified as copy trading." "NQ and MNQ in the same direction across mismatched accounts" is listed as prohibited correlated trading. (G15)
- Copiers named by the firm: "We provide a trade copier feature built directly into our trading platform, Tradovate." (G15). BlackArrow: "Order Replicator, enabling you to mirror trades across up to 10 accounts simultaneously" (P6). WealthCharts: "Trade Copier" with the note "Copy trading is only allowed between accounts of the same program and size." (P8)
- Responsibility: "Each trader is solely responsible for the configuration, supervision, and final outcomes of all trades executed via third-party copy trading software." "We cannot void losses, reset accounts, or provide refunds for trades originated by external software" (G15)
- Between people: "Copy trading between your accounts and another person's accounts, regardless of your relationship to that trader (even a family member or roommate)" is "strictly prohibited". Also prohibited: "Following signal services, Telegram groups, Discord calls, or any shared trade alert system". (G4, G15, G11)

**Account-count limits (G4, standard, per category, all the same size):**

| Category | Max active funded |
|---|---|
| Elite Daily Funded | 5 ("All 5 must be the exact same account size"). ED9 says 5 eval + funded combined and "You may mix account sizes" (conflict, §9 C8) |
| Elite ACCESS Funded + Elite ACCESS Intraday Funded (combined) | 10 |
| Instant Sim Funded | 3 (100K or 150K sizes) or 5 (25K or 50K sizes) |
| Elite Sim Funded | 3 (100K or 150K sizes) or 5 (25K or 50K sizes) |
| Ignite Funded | 10 |
| LaunchPad (X-Ultra, X-Daily Spark, future) | 3 in total |

These are also **household** limits: "These limits apply to the entire household combined, that is, across every person living at the same address" (G14). "If you trade or activate more accounts than your limit, the oldest of your existing active accounts will be automatically breached" (G11). Promotions may raise the limits (§1).

**Copying to/from other firms: conflict (§9 C3).**

- Help center: "While Top One Futures does not prohibit copy trading between your own accounts at different firms, all trade decisions must originate from activity on our platform. As long as your trades begin from your Top One Futures account and reflect your own strategy and discretion, this is acceptable." (G15, also G11 §13)
- Terms & Conditions §4 lists as "strictly prohibited": "Copy trading, signal-following, or group trading with external accounts." (TC)
- Related: "Front-running: Placing trades with the intent to benefit from anticipated price movement based on trades executed elsewhere (for example in another account or at another firm) is not allowed." (G11)

**Same-IP rule vs copying: conflict (§9 C4).** The IP policy's "Not allowed" column lists "Trading multiple accounts from the same IP". It also says: "Multiple accounts from similar IP addresses: Trading from multiple accounts originating from the same or closely related IP addresses is not allowed" (G13). The copy policy allows copying between the same person's same-type, same-size accounts, which normally share one IP. The same article also says "Identical trades placed across multiple accounts under different ownership will be flagged" (G13).

## 6. Automated trading stance

**Classification: `forbidden`** (YAML `automated_trading_allowed: "no"`). It applies to all plans and phases: "strictly forbidden on all Top One Futures accounts, regardless of account type or stage" (G11 intro).

Verbatim rule texts (all accessed 2026-10-23):

- G11 §12 "Automated trading / Expert Advisors (EAs)" (used as `automation_rule_text`): "Use of any automated trading system, script, bot, Expert Advisor (EA), or high-frequency execution tool is strictly forbidden. All trades must be entered manually or semi-manually by the trader." — https://help.toponefutures.com/en/articles/11021584-prohibited-trading-practices
- G6: "No, the use of Expert Advisors (EAs) or Bots is not allowed on your accounts. We require all traders to engage in manual, skill-based trading to ensure fairness and integrity in the trading environment." Consequences: "Account suspension or termination … Loss of funding privileges". Page summary: "automated trading is not allowed on any account". — https://help.toponefutures.com/en/articles/10907920-can-i-use-ea-s-bots
- TC §4: "Using bots, Expert Advisors (EAs), or automated trading systems." — https://www.toponefutures.com/terms-and-conditions
- Plan pages repeat it: Ignite "Overnight trading and expert advisors or bots are not allowed." (IG1). X-Daily Spark "EAs and bots: Not allowed." (XS1, XS8)

PO checklist:

| Item | Finding |
|---|---|
| (i) Classification | `no` (forbidden), all plans |
| (ii) Semi-automated trading | Mentioned: "All trades must be entered manually or semi-manually by the trader." (G11). "Semi-manually" is not defined |
| (ii) Trade copiers | Allowed for own same-program, same-size accounts (§5). The firm names Tradovate's built-in copier, BlackArrow Replicator and WealthCharts Trade Copier. BlackArrow note: "Automations and execution strategies cannot be used as the master account." (P6) |
| (ii) Webhooks / alerts | Webhook automation: not mentioned. Alerts are mentioned only as signal sources to avoid: "Following signal services, Telegram groups, Discord calls, or any shared trade alert system" (G11, G15). The TradingView connection is documented for manual trading through the Tradovate panel (P3). Nothing says TradingView alerts or strategies may place orders |
| (ii) ATM / automatic brackets | Documented and described as optional. "ATM (Advanced Trade Management) Strategies in NinjaTrader Desktop allow you to automatically place Stop Loss and Take Profit orders and add optional automation such as breakeven and trailing stops." "They are completely optional. You can trade manually or use ATMs for convenience and consistency." "ATM Strategies are 100% client-side." (P4). No page says ATMs fall under the bot ban |
| (iii) Differs by phase? | No. The ban covers "all Top One Futures accounts, regardless of account type or stage" (G11) |
| Other | "Using any publicly available or purchased system advertised to help traders 'pass Evaluation' is not allowed." (G11 §9). Minimum hold is 10 seconds: "10.00 seconds = violation, 10.01 seconds = acceptable." (G11 §6, G9) |

## 7. Other rules that affect routing or risk

### Device / hosting rules (format rev. 3 §3 item 7)

- **VPS: `conditional`.** "At Top One Futures, we permit the use of Virtual Private Servers (VPS) with static IP addresses for trading purposes. … If you choose to use a VPS, please ensure it has a static IP address and notify us in advance to avoid any issues." Requirements: "IP address of the VPS: This must be a static IP"; "VPS service invoice: The invoice should include the VPS's IP address"; "IP registration: Ensure the VPS IP address is shared and registered with our team". The summary table says "Using a VPS with a static IP (must notify support)" under "Allowed". (G13)
- **VPN: not allowed.** The summary table lists "Using a VPN (strictly prohibited)". The body says "The use of Virtual Private Networks (VPNs) is not advised … using a VPN may trigger location violations and lead to account suspension or breaches." (G13). The wording differs inside the same page (§9 C9).
- **Personal device: `no`** (R4, 2026-10-23). No page requires orders to come from a "personal device", and the IP page (G13, https://help.toponefutures.com/en/articles/11187841-are-there-ip-address-restrictions) lists as "Allowed": "Trading from different devices (mobile, desktop)" and "Using a VPS with a static IP (must notify support)". It also says: "At Top One Futures, we permit the use of Virtual Private Servers (VPS) with static IP addresses for trading purposes." Recommendation only: "We highly recommend that traders use a single device with a unique IP address for their trading activities". Limits that still apply: "You must be the sole individual executing trades on your account. Sharing your login credentials or allowing a third party to trade on your behalf is not allowed" (G16, https://help.toponefutures.com/en/articles/14847123-account-ownership-and-registration-policies). Searched: G13, G16, G11, G15, G6, G14 and the T&C (June 10, 2026) for device, personal, remote, third, VPS, server, cloud, shared, IP. Earlier searches: all 131 help-center articles, the homepage, /elite-access, /launchpad and /coupon-code-pricing.
- **Only the owner may trade.** "Only the registered account owner is permitted to trade. Allowing friends, family, or third parties to access or trade on your account violates this policy." (G11 §5). "You must be the sole individual executing trades on your account." (G16). One profile per person: "We operate under a strict one account per user policy." (G16)
- **IP monitoring.** "Multiple IP addresses on the same account: Using multiple IP addresses to access a single account may raise flags". See the same-IP conflict in §5. (G13)
- **ATM client-side (P4).** "If NinjaTrader Desktop is not open, running and connected, no ATM instructions can be sent." This is relevant to a VPS-hosted NT8.

### Other rules

| Rule | Text (verbatim) | Source |
|---|---|---|
| No overnight | "holding trades overnight is not allowed. All trades must be closed by 4:10 PM EST daily." Ignite says 4:00 pm ET (C5) | G5, G12, IG1 |
| News | General: "we allow news trading on your accounts". Exceptions are the funded accounts of Elite Daily, Elite ACCESS and Elite ACCESS Intraday ("2 minutes before and 2 minutes after"). G7 says "All other account types (including X-Ultra): No news trading rule", but X-Daily Spark funded has a 2-minute rule (XS8) (§9 C10). Inside the window, "Opening a trade / Closing a trade / Opening pending orders / Placing stop or limit orders / Modifying a stop loss / Modifying a take profit" are violations. "If a trade is closed by any means (manual or automatic) during the window, it is a violation." | G7, ED8, EA11, EI10, XS8 |
| Minimum trade duration | "All trades must remain open longer than 10 seconds." "Closing any portion of a trade before 10 seconds violates this rule." "Adding to a trade (for example scaling in) within 10 seconds is permitted." | G11 §6, G9 |
| Hedging | "You may not hedge one account against another, whether within Top One Futures or across other prop firms or brokers." | G11 §4, G13 |
| Inactivity | "At least one trade (entry and exit) must occur within a 14-day window … (336 consecutive hours)". Otherwise "The account will receive a hard breach and be closed permanently." | G8 |
| Contract-limit breach | "Exceeding your allowed contract limit is considered a rule violation. … Trades may be rejected or reduced … Profits generated while exceeding contract limits may be removed". "Contract limits do not combine across multiple accounts". | G1 |
| Scaling timing | "Your allowed contract size is determined by your end-of-day (EOD) closing balance and updates once per day." "Contract increases apply the following trading day once your closing balance qualifies" | G1 |
| DLL is not a stop | "the DLL can execute late, early, at a different price than you expect, or fail to trigger. … you must place hard stop loss orders directly on your trades." | G2 |
| No-DLL add-on | Since October 16, 2026, at initial checkout, for "Elite ACCESS / Elite Daily / Ignite Instant Funding" (Q5). G2 also lists Elite ACCESS Intraday, and EI5 offers it (§9 G7) | Q5, G2, EI5 |
| Micro Bitcoin | separate MBT limits (1 / 3 / 5 / 7, "depending on account type"). Not relevant to MNQ/NQ | G1 |
| Churning | "Creating multiple evaluation accounts in a short period and using a batch-style approach … is strictly prohibited." | G11 §10 |
| Live transition | "When transitioning to a live / pre-live account, all other accounts are temporarily disabled". "Should the trader breach their Pre-Live or Live account, there is a mandatory 3-week cooling off period" | L1 |
| Payout lock | "When you request a payout, your maximum trailing drawdown locks in at the starting balance plus $100." (general payouts article) | PY1, G3 |

## 8. Machine-readable blocks

One block per plan / phase / size (38 blocks). All blocks share `instruments: [MNQ, NQ]`, the firm-wide platform list (§2), `copy_trading_allowed: conditional`, `automated_trading_allowed: "no"`, `vps_allowed: conditional` and `personal_device_required: "no"`. `min_trading_days: "none"` means the firm explicitly states there is no minimum. `null` means the value was not found. Conflict field names used in `not_confirmed_fields` are defined in §9.

### 8.1 Elite Daily (V2)

```yaml
firm: top-one-futures
plan: "Elite Daily"
phase: evaluation
size_label: "$25K"
size_usd: "25000.00"
instruments: [MNQ, NQ]
platforms: ["Tradovate/NinjaTrader", BlackArrow, WealthCharts]   # firm-wide list; per-plan availability not published (see §2, §9)
drawdown: { amount_usd: "1000.00", type: trailing_eod, stops_trailing_at: "locks in at your starting balance + $100 ($25,100); trigger level not published for Elite Daily" }
daily_loss_limit: { amount_usd: "500.00", kind: soft_pause }   # eval DLL = pause, auto-unpause next trading day; No-DLL add-on at checkout
profit_target_usd: "1500.00"
consistency_rule: { text: "No single trading day may account for more than 40% of your total profit at the moment the profit target is reached.", threshold_pct: "40" }
max_position: { minis: 1, micros: 10, scaling: "none in evaluation (ED7)" }
min_trading_days: "none"   # published: "there is no minimum day requirement" (ED5)
trading_day_reset: { time: "18:00", tz_as_published: "EST" }   # sessions "6:00 PM EST to 4:10 PM EST"; forced flat 4:10 PM EST (G12)
copy_trading_allowed: conditional   # own accounts only, same program AND same size (G15)
automated_trading_allowed: "no"
automation_rule_text: "Use of any automated trading system, script, bot, Expert Advisor (EA), or high-frequency execution tool is strictly forbidden. All trades must be entered manually or semi-manually by the trader."
vps_allowed: conditional          # VPS with static IP, notify support in advance with IP + invoice (G13); VPN prohibited
personal_device_required: "no"   # explicitly permitted: other devices and a static-IP VPS (G13); account owner must be the sole trader (G16)
status: partial
not_confirmed_fields: [copy_trading_other_firms, copy_trading_same_ip, max_accounts_size_mixing]
sources:
  - { field: drawdown, url: "https://help.toponefutures.com/en/articles/13904722-elite-daily-v2-max-loss-limit-mll", accessed: "2026-10-23" }
  - { field: daily_loss_limit, url: "https://help.toponefutures.com/en/articles/15056388-elite-daily-v2-daily-loss-limit-dll-evaluation-phase", accessed: "2026-10-23" }
  - { field: profit_target_usd, url: "https://help.toponefutures.com/en/articles/13904734-elite-daily-v2-profit-target-evaluation", accessed: "2026-10-23" }
  - { field: consistency_rule, url: "https://help.toponefutures.com/en/articles/15056475-elite-daily-v2-40-consistency-target-evaluation-only", accessed: "2026-10-23" }
  - { field: max_position, url: "https://help.toponefutures.com/en/articles/13904824-elite-daily-v2-contract-scaling-plan", accessed: "2026-10-23" }
  - { field: min_trading_days, url: "https://help.toponefutures.com/en/articles/13904734-elite-daily-v2-profit-target-evaluation", accessed: "2026-10-23" }
  - { field: trading_day_reset, url: "https://help.toponefutures.com/en/articles/11165893-trading-hours-at-top-one-futures", accessed: "2026-10-23" }
  - { field: automated_trading_allowed, url: "https://help.toponefutures.com/en/articles/11021584-prohibited-trading-practices", accessed: "2026-10-23" }
  - { field: copy_trading_allowed, url: "https://help.toponefutures.com/en/articles/11680891-copy-trading-policy", accessed: "2026-10-23" }
  - { field: vps_allowed, url: "https://help.toponefutures.com/en/articles/11187841-are-there-ip-address-restrictions", accessed: "2026-10-23" }
  - { field: personal_device_required, url: "https://help.toponefutures.com/en/articles/11187841-are-there-ip-address-restrictions", accessed: "2026-10-23" }
  - { field: platforms, url: "https://help.toponefutures.com/en/articles/17149341-can-i-switch-my-trading-platform-tradovate-ninjatrader-or-blackarrow", accessed: "2026-10-23" }
```

```yaml
firm: top-one-futures
plan: "Elite Daily"
phase: evaluation
size_label: "$50K"
size_usd: "50000.00"
instruments: [MNQ, NQ]
platforms: ["Tradovate/NinjaTrader", BlackArrow, WealthCharts]   # firm-wide list; per-plan availability not published (see §2, §9)
drawdown: { amount_usd: "2000.00", type: trailing_eod, stops_trailing_at: "locks in at your starting balance + $100 ($50,100); trigger level not published for Elite Daily" }
daily_loss_limit: { amount_usd: "1000.00", kind: soft_pause }   # eval DLL = pause, auto-unpause next trading day; No-DLL add-on at checkout
profit_target_usd: "3000.00"
consistency_rule: { text: "No single trading day may account for more than 40% of your total profit at the moment the profit target is reached.", threshold_pct: "40" }
max_position: { minis: 3, micros: 30, scaling: "none in evaluation (ED7)" }
min_trading_days: "none"   # published: "there is no minimum day requirement" (ED5)
trading_day_reset: { time: "18:00", tz_as_published: "EST" }   # sessions "6:00 PM EST to 4:10 PM EST"; forced flat 4:10 PM EST (G12)
copy_trading_allowed: conditional   # own accounts only, same program AND same size (G15)
automated_trading_allowed: "no"
automation_rule_text: "Use of any automated trading system, script, bot, Expert Advisor (EA), or high-frequency execution tool is strictly forbidden. All trades must be entered manually or semi-manually by the trader."
vps_allowed: conditional          # VPS with static IP, notify support in advance with IP + invoice (G13); VPN prohibited
personal_device_required: "no"   # explicitly permitted: other devices and a static-IP VPS (G13); account owner must be the sole trader (G16)
status: partial
not_confirmed_fields: [copy_trading_other_firms, copy_trading_same_ip, max_accounts_size_mixing]
sources:
  - { field: drawdown, url: "https://help.toponefutures.com/en/articles/13904722-elite-daily-v2-max-loss-limit-mll", accessed: "2026-10-23" }
  - { field: daily_loss_limit, url: "https://help.toponefutures.com/en/articles/15056388-elite-daily-v2-daily-loss-limit-dll-evaluation-phase", accessed: "2026-10-23" }
  - { field: profit_target_usd, url: "https://help.toponefutures.com/en/articles/13904734-elite-daily-v2-profit-target-evaluation", accessed: "2026-10-23" }
  - { field: consistency_rule, url: "https://help.toponefutures.com/en/articles/15056475-elite-daily-v2-40-consistency-target-evaluation-only", accessed: "2026-10-23" }
  - { field: max_position, url: "https://help.toponefutures.com/en/articles/13904824-elite-daily-v2-contract-scaling-plan", accessed: "2026-10-23" }
  - { field: min_trading_days, url: "https://help.toponefutures.com/en/articles/13904734-elite-daily-v2-profit-target-evaluation", accessed: "2026-10-23" }
  - { field: trading_day_reset, url: "https://help.toponefutures.com/en/articles/11165893-trading-hours-at-top-one-futures", accessed: "2026-10-23" }
  - { field: automated_trading_allowed, url: "https://help.toponefutures.com/en/articles/11021584-prohibited-trading-practices", accessed: "2026-10-23" }
  - { field: copy_trading_allowed, url: "https://help.toponefutures.com/en/articles/11680891-copy-trading-policy", accessed: "2026-10-23" }
  - { field: vps_allowed, url: "https://help.toponefutures.com/en/articles/11187841-are-there-ip-address-restrictions", accessed: "2026-10-23" }
  - { field: personal_device_required, url: "https://help.toponefutures.com/en/articles/11187841-are-there-ip-address-restrictions", accessed: "2026-10-23" }
  - { field: platforms, url: "https://help.toponefutures.com/en/articles/17149341-can-i-switch-my-trading-platform-tradovate-ninjatrader-or-blackarrow", accessed: "2026-10-23" }
```

```yaml
firm: top-one-futures
plan: "Elite Daily"
phase: evaluation
size_label: "$100K"
size_usd: "100000.00"
instruments: [MNQ, NQ]
platforms: ["Tradovate/NinjaTrader", BlackArrow, WealthCharts]   # firm-wide list; per-plan availability not published (see §2, §9)
drawdown: { amount_usd: "3000.00", type: trailing_eod, stops_trailing_at: "locks in at your starting balance + $100 ($100,100); trigger level not published for Elite Daily" }
daily_loss_limit: { amount_usd: "1250.00", kind: soft_pause }   # eval DLL = pause, auto-unpause next trading day; No-DLL add-on at checkout
profit_target_usd: "6000.00"
consistency_rule: { text: "No single trading day may account for more than 40% of your total profit at the moment the profit target is reached.", threshold_pct: "40" }
max_position: { minis: 5, micros: 50, scaling: "none in evaluation (ED7)" }
min_trading_days: "none"   # published: "there is no minimum day requirement" (ED5)
trading_day_reset: { time: "18:00", tz_as_published: "EST" }   # sessions "6:00 PM EST to 4:10 PM EST"; forced flat 4:10 PM EST (G12)
copy_trading_allowed: conditional   # own accounts only, same program AND same size (G15)
automated_trading_allowed: "no"
automation_rule_text: "Use of any automated trading system, script, bot, Expert Advisor (EA), or high-frequency execution tool is strictly forbidden. All trades must be entered manually or semi-manually by the trader."
vps_allowed: conditional          # VPS with static IP, notify support in advance with IP + invoice (G13); VPN prohibited
personal_device_required: "no"   # explicitly permitted: other devices and a static-IP VPS (G13); account owner must be the sole trader (G16)
status: partial
not_confirmed_fields: [copy_trading_other_firms, copy_trading_same_ip, max_accounts_size_mixing]
sources:
  - { field: drawdown, url: "https://help.toponefutures.com/en/articles/13904722-elite-daily-v2-max-loss-limit-mll", accessed: "2026-10-23" }
  - { field: daily_loss_limit, url: "https://help.toponefutures.com/en/articles/15056388-elite-daily-v2-daily-loss-limit-dll-evaluation-phase", accessed: "2026-10-23" }
  - { field: profit_target_usd, url: "https://help.toponefutures.com/en/articles/13904734-elite-daily-v2-profit-target-evaluation", accessed: "2026-10-23" }
  - { field: consistency_rule, url: "https://help.toponefutures.com/en/articles/15056475-elite-daily-v2-40-consistency-target-evaluation-only", accessed: "2026-10-23" }
  - { field: max_position, url: "https://help.toponefutures.com/en/articles/13904824-elite-daily-v2-contract-scaling-plan", accessed: "2026-10-23" }
  - { field: min_trading_days, url: "https://help.toponefutures.com/en/articles/13904734-elite-daily-v2-profit-target-evaluation", accessed: "2026-10-23" }
  - { field: trading_day_reset, url: "https://help.toponefutures.com/en/articles/11165893-trading-hours-at-top-one-futures", accessed: "2026-10-23" }
  - { field: automated_trading_allowed, url: "https://help.toponefutures.com/en/articles/11021584-prohibited-trading-practices", accessed: "2026-10-23" }
  - { field: copy_trading_allowed, url: "https://help.toponefutures.com/en/articles/11680891-copy-trading-policy", accessed: "2026-10-23" }
  - { field: vps_allowed, url: "https://help.toponefutures.com/en/articles/11187841-are-there-ip-address-restrictions", accessed: "2026-10-23" }
  - { field: personal_device_required, url: "https://help.toponefutures.com/en/articles/11187841-are-there-ip-address-restrictions", accessed: "2026-10-23" }
  - { field: platforms, url: "https://help.toponefutures.com/en/articles/17149341-can-i-switch-my-trading-platform-tradovate-ninjatrader-or-blackarrow", accessed: "2026-10-23" }
```

```yaml
firm: top-one-futures
plan: "Elite Daily"
phase: evaluation
size_label: "$150K"
size_usd: "150000.00"
instruments: [MNQ, NQ]
platforms: ["Tradovate/NinjaTrader", BlackArrow, WealthCharts]   # firm-wide list; per-plan availability not published (see §2, §9)
drawdown: { amount_usd: "4500.00", type: trailing_eod, stops_trailing_at: "locks in at your starting balance + $100 ($150,100); trigger level not published for Elite Daily" }
daily_loss_limit: { amount_usd: "1850.00", kind: soft_pause }   # CONFLICT: $1,850 (ED1, ED2) vs $1,750 (homepage card) vs n/a (G2)
profit_target_usd: "9000.00"
consistency_rule: { text: "No single trading day may account for more than 40% of your total profit at the moment the profit target is reached.", threshold_pct: "40" }
max_position: { minis: 7, micros: 70, scaling: "none in evaluation (ED7)" }
min_trading_days: "none"   # published: "there is no minimum day requirement" (ED5)
trading_day_reset: { time: "18:00", tz_as_published: "EST" }   # sessions "6:00 PM EST to 4:10 PM EST"; forced flat 4:10 PM EST (G12)
copy_trading_allowed: conditional   # own accounts only, same program AND same size (G15)
automated_trading_allowed: "no"
automation_rule_text: "Use of any automated trading system, script, bot, Expert Advisor (EA), or high-frequency execution tool is strictly forbidden. All trades must be entered manually or semi-manually by the trader."
vps_allowed: conditional          # VPS with static IP, notify support in advance with IP + invoice (G13); VPN prohibited
personal_device_required: "no"   # explicitly permitted: other devices and a static-IP VPS (G13); account owner must be the sole trader (G16)
status: partial
not_confirmed_fields: [daily_loss_limit, copy_trading_other_firms, copy_trading_same_ip, max_accounts_size_mixing]
sources:
  - { field: drawdown, url: "https://help.toponefutures.com/en/articles/13904722-elite-daily-v2-max-loss-limit-mll", accessed: "2026-10-23" }
  - { field: daily_loss_limit, url: "https://help.toponefutures.com/en/articles/15056388-elite-daily-v2-daily-loss-limit-dll-evaluation-phase", accessed: "2026-10-23" }
  - { field: profit_target_usd, url: "https://help.toponefutures.com/en/articles/13904734-elite-daily-v2-profit-target-evaluation", accessed: "2026-10-23" }
  - { field: consistency_rule, url: "https://help.toponefutures.com/en/articles/15056475-elite-daily-v2-40-consistency-target-evaluation-only", accessed: "2026-10-23" }
  - { field: max_position, url: "https://help.toponefutures.com/en/articles/13904824-elite-daily-v2-contract-scaling-plan", accessed: "2026-10-23" }
  - { field: min_trading_days, url: "https://help.toponefutures.com/en/articles/13904734-elite-daily-v2-profit-target-evaluation", accessed: "2026-10-23" }
  - { field: trading_day_reset, url: "https://help.toponefutures.com/en/articles/11165893-trading-hours-at-top-one-futures", accessed: "2026-10-23" }
  - { field: automated_trading_allowed, url: "https://help.toponefutures.com/en/articles/11021584-prohibited-trading-practices", accessed: "2026-10-23" }
  - { field: copy_trading_allowed, url: "https://help.toponefutures.com/en/articles/11680891-copy-trading-policy", accessed: "2026-10-23" }
  - { field: vps_allowed, url: "https://help.toponefutures.com/en/articles/11187841-are-there-ip-address-restrictions", accessed: "2026-10-23" }
  - { field: personal_device_required, url: "https://help.toponefutures.com/en/articles/11187841-are-there-ip-address-restrictions", accessed: "2026-10-23" }
  - { field: platforms, url: "https://help.toponefutures.com/en/articles/17149341-can-i-switch-my-trading-platform-tradovate-ninjatrader-or-blackarrow", accessed: "2026-10-23" }
```

```yaml
firm: top-one-futures
plan: "Elite Daily"
phase: funded
size_label: "$25K"
size_usd: "25000.00"
instruments: [MNQ, NQ]
platforms: ["Tradovate/NinjaTrader", BlackArrow, WealthCharts]   # firm-wide list; per-plan availability not published (see §2, §9)
drawdown: { amount_usd: "1000.00", type: trailing_eod, stops_trailing_at: "locks in at your starting balance + $100 ($25,100); also locks on first payout (G3)" }
daily_loss_limit: { amount_usd: "500.00", kind: soft_pause }   # soft breach: paused rest of day; No-DLL add-on at checkout
profit_target_usd: null   # none; payout buffer + 50% progression rule apply (§3.1)
consistency_rule: { text: null, threshold_pct: null }
max_position: { minis: 1, micros: 10, scaling: "$0-$1,500+: 1 mini / 10 micros (full allocation)" }
min_trading_days: 1   # "Min Trading Days (Funded) 1" (ED1)
trading_day_reset: { time: "18:00", tz_as_published: "EST" }   # sessions "6:00 PM EST to 4:10 PM EST"; forced flat 4:10 PM EST (G12)
copy_trading_allowed: conditional   # own accounts only, same program AND same size (G15)
automated_trading_allowed: "no"
automation_rule_text: "Use of any automated trading system, script, bot, Expert Advisor (EA), or high-frequency execution tool is strictly forbidden. All trades must be entered manually or semi-manually by the trader."
vps_allowed: conditional          # VPS with static IP, notify support in advance with IP + invoice (G13); VPN prohibited
personal_device_required: "no"   # explicitly permitted: other devices and a static-IP VPS (G13); account owner must be the sole trader (G16)
status: partial
not_confirmed_fields: [copy_trading_other_firms, copy_trading_same_ip, max_accounts_size_mixing]
sources:
  - { field: drawdown, url: "https://help.toponefutures.com/en/articles/13904722-elite-daily-v2-max-loss-limit-mll", accessed: "2026-10-23" }
  - { field: daily_loss_limit, url: "https://help.toponefutures.com/en/articles/13904706-elite-daily-v2-daily-loss-limit-dll-funded-phase", accessed: "2026-10-23" }
  - { field: consistency_rule, url: "https://help.toponefutures.com/en/articles/15056475-elite-daily-v2-40-consistency-target-evaluation-only", accessed: "2026-10-23" }
  - { field: max_position, url: "https://help.toponefutures.com/en/articles/13904824-elite-daily-v2-contract-scaling-plan", accessed: "2026-10-23" }
  - { field: min_trading_days, url: "https://help.toponefutures.com/en/articles/15056075-elite-daily-v2-overview", accessed: "2026-10-23" }
  - { field: trading_day_reset, url: "https://help.toponefutures.com/en/articles/11165893-trading-hours-at-top-one-futures", accessed: "2026-10-23" }
  - { field: automated_trading_allowed, url: "https://help.toponefutures.com/en/articles/11021584-prohibited-trading-practices", accessed: "2026-10-23" }
  - { field: copy_trading_allowed, url: "https://help.toponefutures.com/en/articles/11680891-copy-trading-policy", accessed: "2026-10-23" }
  - { field: vps_allowed, url: "https://help.toponefutures.com/en/articles/11187841-are-there-ip-address-restrictions", accessed: "2026-10-23" }
  - { field: personal_device_required, url: "https://help.toponefutures.com/en/articles/11187841-are-there-ip-address-restrictions", accessed: "2026-10-23" }
  - { field: platforms, url: "https://help.toponefutures.com/en/articles/17149341-can-i-switch-my-trading-platform-tradovate-ninjatrader-or-blackarrow", accessed: "2026-10-23" }
```

```yaml
firm: top-one-futures
plan: "Elite Daily"
phase: funded
size_label: "$50K"
size_usd: "50000.00"
instruments: [MNQ, NQ]
platforms: ["Tradovate/NinjaTrader", BlackArrow, WealthCharts]   # firm-wide list; per-plan availability not published (see §2, §9)
drawdown: { amount_usd: "2000.00", type: trailing_eod, stops_trailing_at: "locks in at your starting balance + $100 ($50,100); also locks on first payout (G3)" }
daily_loss_limit: { amount_usd: "1000.00", kind: soft_pause }   # soft breach: paused rest of day; No-DLL add-on at checkout
profit_target_usd: null   # none; payout buffer + 50% progression rule apply (§3.1)
consistency_rule: { text: null, threshold_pct: null }
max_position: { minis: 3, micros: 30, scaling: "$0-$1,499: 1/10; $1,500-$1,999: 2/20; $2,000+: 3/30 (full); EOD balance, applies next trading day" }
min_trading_days: 1   # "Min Trading Days (Funded) 1" (ED1)
trading_day_reset: { time: "18:00", tz_as_published: "EST" }   # sessions "6:00 PM EST to 4:10 PM EST"; forced flat 4:10 PM EST (G12)
copy_trading_allowed: conditional   # own accounts only, same program AND same size (G15)
automated_trading_allowed: "no"
automation_rule_text: "Use of any automated trading system, script, bot, Expert Advisor (EA), or high-frequency execution tool is strictly forbidden. All trades must be entered manually or semi-manually by the trader."
vps_allowed: conditional          # VPS with static IP, notify support in advance with IP + invoice (G13); VPN prohibited
personal_device_required: "no"   # explicitly permitted: other devices and a static-IP VPS (G13); account owner must be the sole trader (G16)
status: partial
not_confirmed_fields: [copy_trading_other_firms, copy_trading_same_ip, max_accounts_size_mixing]
sources:
  - { field: drawdown, url: "https://help.toponefutures.com/en/articles/13904722-elite-daily-v2-max-loss-limit-mll", accessed: "2026-10-23" }
  - { field: daily_loss_limit, url: "https://help.toponefutures.com/en/articles/13904706-elite-daily-v2-daily-loss-limit-dll-funded-phase", accessed: "2026-10-23" }
  - { field: consistency_rule, url: "https://help.toponefutures.com/en/articles/15056475-elite-daily-v2-40-consistency-target-evaluation-only", accessed: "2026-10-23" }
  - { field: max_position, url: "https://help.toponefutures.com/en/articles/13904824-elite-daily-v2-contract-scaling-plan", accessed: "2026-10-23" }
  - { field: min_trading_days, url: "https://help.toponefutures.com/en/articles/15056075-elite-daily-v2-overview", accessed: "2026-10-23" }
  - { field: trading_day_reset, url: "https://help.toponefutures.com/en/articles/11165893-trading-hours-at-top-one-futures", accessed: "2026-10-23" }
  - { field: automated_trading_allowed, url: "https://help.toponefutures.com/en/articles/11021584-prohibited-trading-practices", accessed: "2026-10-23" }
  - { field: copy_trading_allowed, url: "https://help.toponefutures.com/en/articles/11680891-copy-trading-policy", accessed: "2026-10-23" }
  - { field: vps_allowed, url: "https://help.toponefutures.com/en/articles/11187841-are-there-ip-address-restrictions", accessed: "2026-10-23" }
  - { field: personal_device_required, url: "https://help.toponefutures.com/en/articles/11187841-are-there-ip-address-restrictions", accessed: "2026-10-23" }
  - { field: platforms, url: "https://help.toponefutures.com/en/articles/17149341-can-i-switch-my-trading-platform-tradovate-ninjatrader-or-blackarrow", accessed: "2026-10-23" }
```

```yaml
firm: top-one-futures
plan: "Elite Daily"
phase: funded
size_label: "$100K"
size_usd: "100000.00"
instruments: [MNQ, NQ]
platforms: ["Tradovate/NinjaTrader", BlackArrow, WealthCharts]   # firm-wide list; per-plan availability not published (see §2, §9)
drawdown: { amount_usd: "2500.00", type: trailing_eod, stops_trailing_at: "locks in at your starting balance + $100 ($100,100); also locks on first payout (G3)" }
daily_loss_limit: { amount_usd: "1250.00", kind: soft_pause }   # soft breach: paused rest of day; No-DLL add-on at checkout
profit_target_usd: null   # none; payout buffer + 50% progression rule apply (§3.1)
consistency_rule: { text: null, threshold_pct: null }
max_position: { minis: 5, micros: 50, scaling: "$0-$1,499: 2/20; $1,500-$1,999: 3/30; $2,000-$2,999: 4/40; $3,000+: 5/50 (full); EOD balance, applies next trading day" }
min_trading_days: 1   # "Min Trading Days (Funded) 1" (ED1)
trading_day_reset: { time: "18:00", tz_as_published: "EST" }   # sessions "6:00 PM EST to 4:10 PM EST"; forced flat 4:10 PM EST (G12)
copy_trading_allowed: conditional   # own accounts only, same program AND same size (G15)
automated_trading_allowed: "no"
automation_rule_text: "Use of any automated trading system, script, bot, Expert Advisor (EA), or high-frequency execution tool is strictly forbidden. All trades must be entered manually or semi-manually by the trader."
vps_allowed: conditional          # VPS with static IP, notify support in advance with IP + invoice (G13); VPN prohibited
personal_device_required: "no"   # explicitly permitted: other devices and a static-IP VPS (G13); account owner must be the sole trader (G16)
status: partial
not_confirmed_fields: [copy_trading_other_firms, copy_trading_same_ip, max_accounts_size_mixing]
sources:
  - { field: drawdown, url: "https://help.toponefutures.com/en/articles/13904722-elite-daily-v2-max-loss-limit-mll", accessed: "2026-10-23" }
  - { field: daily_loss_limit, url: "https://help.toponefutures.com/en/articles/13904706-elite-daily-v2-daily-loss-limit-dll-funded-phase", accessed: "2026-10-23" }
  - { field: consistency_rule, url: "https://help.toponefutures.com/en/articles/15056475-elite-daily-v2-40-consistency-target-evaluation-only", accessed: "2026-10-23" }
  - { field: max_position, url: "https://help.toponefutures.com/en/articles/13904824-elite-daily-v2-contract-scaling-plan", accessed: "2026-10-23" }
  - { field: min_trading_days, url: "https://help.toponefutures.com/en/articles/15056075-elite-daily-v2-overview", accessed: "2026-10-23" }
  - { field: trading_day_reset, url: "https://help.toponefutures.com/en/articles/11165893-trading-hours-at-top-one-futures", accessed: "2026-10-23" }
  - { field: automated_trading_allowed, url: "https://help.toponefutures.com/en/articles/11021584-prohibited-trading-practices", accessed: "2026-10-23" }
  - { field: copy_trading_allowed, url: "https://help.toponefutures.com/en/articles/11680891-copy-trading-policy", accessed: "2026-10-23" }
  - { field: vps_allowed, url: "https://help.toponefutures.com/en/articles/11187841-are-there-ip-address-restrictions", accessed: "2026-10-23" }
  - { field: personal_device_required, url: "https://help.toponefutures.com/en/articles/11187841-are-there-ip-address-restrictions", accessed: "2026-10-23" }
  - { field: platforms, url: "https://help.toponefutures.com/en/articles/17149341-can-i-switch-my-trading-platform-tradovate-ninjatrader-or-blackarrow", accessed: "2026-10-23" }
```

```yaml
firm: top-one-futures
plan: "Elite Daily"
phase: funded
size_label: "$150K"
size_usd: "150000.00"
instruments: [MNQ, NQ]
platforms: ["Tradovate/NinjaTrader", BlackArrow, WealthCharts]   # firm-wide list; per-plan availability not published (see §2, §9)
drawdown: { amount_usd: "3500.00", type: trailing_eod, stops_trailing_at: "locks in at your starting balance + $100 ($150,100); also locks on first payout (G3)" }
daily_loss_limit: { amount_usd: "1850.00", kind: soft_pause }   # CONFLICT: $1,850 (ED1, ED3, homepage) vs n/a (G2)
profit_target_usd: null   # none; payout buffer + 50% progression rule apply (§3.1)
consistency_rule: { text: null, threshold_pct: null }
max_position: { minis: 7, micros: 70, scaling: "$0-$1,499: 3/30; $1,500-$1,999: 4/40; $2,000-$2,999: 5/50; $3,000-$4,499: 6/60; $4,500+: 7/70 (full); EOD balance, applies next trading day" }
min_trading_days: 1   # "Min Trading Days (Funded) 1" (ED1)
trading_day_reset: { time: "18:00", tz_as_published: "EST" }   # sessions "6:00 PM EST to 4:10 PM EST"; forced flat 4:10 PM EST (G12)
copy_trading_allowed: conditional   # own accounts only, same program AND same size (G15)
automated_trading_allowed: "no"
automation_rule_text: "Use of any automated trading system, script, bot, Expert Advisor (EA), or high-frequency execution tool is strictly forbidden. All trades must be entered manually or semi-manually by the trader."
vps_allowed: conditional          # VPS with static IP, notify support in advance with IP + invoice (G13); VPN prohibited
personal_device_required: "no"   # explicitly permitted: other devices and a static-IP VPS (G13); account owner must be the sole trader (G16)
status: partial
not_confirmed_fields: [daily_loss_limit, copy_trading_other_firms, copy_trading_same_ip, max_accounts_size_mixing]
sources:
  - { field: drawdown, url: "https://help.toponefutures.com/en/articles/13904722-elite-daily-v2-max-loss-limit-mll", accessed: "2026-10-23" }
  - { field: daily_loss_limit, url: "https://help.toponefutures.com/en/articles/13904706-elite-daily-v2-daily-loss-limit-dll-funded-phase", accessed: "2026-10-23" }
  - { field: consistency_rule, url: "https://help.toponefutures.com/en/articles/15056475-elite-daily-v2-40-consistency-target-evaluation-only", accessed: "2026-10-23" }
  - { field: max_position, url: "https://help.toponefutures.com/en/articles/13904824-elite-daily-v2-contract-scaling-plan", accessed: "2026-10-23" }
  - { field: min_trading_days, url: "https://help.toponefutures.com/en/articles/15056075-elite-daily-v2-overview", accessed: "2026-10-23" }
  - { field: trading_day_reset, url: "https://help.toponefutures.com/en/articles/11165893-trading-hours-at-top-one-futures", accessed: "2026-10-23" }
  - { field: automated_trading_allowed, url: "https://help.toponefutures.com/en/articles/11021584-prohibited-trading-practices", accessed: "2026-10-23" }
  - { field: copy_trading_allowed, url: "https://help.toponefutures.com/en/articles/11680891-copy-trading-policy", accessed: "2026-10-23" }
  - { field: vps_allowed, url: "https://help.toponefutures.com/en/articles/11187841-are-there-ip-address-restrictions", accessed: "2026-10-23" }
  - { field: personal_device_required, url: "https://help.toponefutures.com/en/articles/11187841-are-there-ip-address-restrictions", accessed: "2026-10-23" }
  - { field: platforms, url: "https://help.toponefutures.com/en/articles/17149341-can-i-switch-my-trading-platform-tradovate-ninjatrader-or-blackarrow", accessed: "2026-10-23" }
```

### 8.2 Elite ACCESS (EOD funded drawdown)

```yaml
firm: top-one-futures
plan: "Elite ACCESS"
phase: evaluation
size_label: "$25K"
size_usd: "25000.00"
instruments: [MNQ, NQ]
platforms: ["Tradovate/NinjaTrader", BlackArrow, WealthCharts]   # firm-wide list; per-plan availability not published (see §2, §9)
drawdown: { amount_usd: "1000.00", type: trailing_eod, stops_trailing_at: "locks at breakeven + $100 ($25,100) once the balance reaches starting balance + drawdown + $100 (EI7 wording)" }
daily_loss_limit: { amount_usd: null, kind: none }   # no DLL in evaluation (EA5)
profit_target_usd: "1500.00"
consistency_rule: { text: null, threshold_pct: null }
max_position: { minis: 1, micros: 10, scaling: "none in evaluation (EA2)" }
min_trading_days: 1   # "Minimum of 1 trading day"; 30 days to pass (EA1)
trading_day_reset: { time: "18:00", tz_as_published: "EST" }   # sessions "6:00 PM EST to 4:10 PM EST"; forced flat 4:10 PM EST (G12)
copy_trading_allowed: conditional   # own accounts only, same program AND same size (G15)
automated_trading_allowed: "no"
automation_rule_text: "Use of any automated trading system, script, bot, Expert Advisor (EA), or high-frequency execution tool is strictly forbidden. All trades must be entered manually or semi-manually by the trader."
vps_allowed: conditional          # VPS with static IP, notify support in advance with IP + invoice (G13); VPN prohibited
personal_device_required: "no"   # explicitly permitted: other devices and a static-IP VPS (G13); account owner must be the sole trader (G16)
status: partial
not_confirmed_fields: [copy_trading_other_firms, copy_trading_same_ip]
sources:
  - { field: drawdown, url: "https://help.toponefutures.com/en/articles/14595145-elite-access-maximum-loss-limit-mll-eod", accessed: "2026-10-23" }
  - { field: daily_loss_limit, url: "https://help.toponefutures.com/en/articles/14595138-elite-access-daily-loss-limit-dll", accessed: "2026-10-23" }
  - { field: profit_target_usd, url: "https://help.toponefutures.com/en/articles/14595073-elite-access-evaluation-profit-target", accessed: "2026-10-23" }
  - { field: consistency_rule, url: "https://help.toponefutures.com/en/articles/14595073-elite-access-evaluation-profit-target", accessed: "2026-10-23" }
  - { field: max_position, url: "https://help.toponefutures.com/en/articles/10906950-maximum-contracts-explained", accessed: "2026-10-23" }
  - { field: min_trading_days, url: "https://help.toponefutures.com/en/articles/14595016-elite-access-accounts-overview", accessed: "2026-10-23" }
  - { field: trading_day_reset, url: "https://help.toponefutures.com/en/articles/11165893-trading-hours-at-top-one-futures", accessed: "2026-10-23" }
  - { field: automated_trading_allowed, url: "https://help.toponefutures.com/en/articles/11021584-prohibited-trading-practices", accessed: "2026-10-23" }
  - { field: copy_trading_allowed, url: "https://help.toponefutures.com/en/articles/11680891-copy-trading-policy", accessed: "2026-10-23" }
  - { field: vps_allowed, url: "https://help.toponefutures.com/en/articles/11187841-are-there-ip-address-restrictions", accessed: "2026-10-23" }
  - { field: personal_device_required, url: "https://help.toponefutures.com/en/articles/11187841-are-there-ip-address-restrictions", accessed: "2026-10-23" }
  - { field: platforms, url: "https://help.toponefutures.com/en/articles/17149341-can-i-switch-my-trading-platform-tradovate-ninjatrader-or-blackarrow", accessed: "2026-10-23" }
```

```yaml
firm: top-one-futures
plan: "Elite ACCESS"
phase: evaluation
size_label: "$50K"
size_usd: "50000.00"
instruments: [MNQ, NQ]
platforms: ["Tradovate/NinjaTrader", BlackArrow, WealthCharts]   # firm-wide list; per-plan availability not published (see §2, §9)
drawdown: { amount_usd: "2000.00", type: trailing_eod, stops_trailing_at: "locks at breakeven + $100 ($50,100) once the balance reaches starting balance + drawdown + $100 (EI7 wording)" }
daily_loss_limit: { amount_usd: null, kind: none }   # no DLL in evaluation (EA5)
profit_target_usd: "3000.00"
consistency_rule: { text: null, threshold_pct: null }
max_position: { minis: 3, micros: 30, scaling: "none in evaluation (EA2)" }
min_trading_days: 1   # "Minimum of 1 trading day"; 30 days to pass (EA1)
trading_day_reset: { time: "18:00", tz_as_published: "EST" }   # sessions "6:00 PM EST to 4:10 PM EST"; forced flat 4:10 PM EST (G12)
copy_trading_allowed: conditional   # own accounts only, same program AND same size (G15)
automated_trading_allowed: "no"
automation_rule_text: "Use of any automated trading system, script, bot, Expert Advisor (EA), or high-frequency execution tool is strictly forbidden. All trades must be entered manually or semi-manually by the trader."
vps_allowed: conditional          # VPS with static IP, notify support in advance with IP + invoice (G13); VPN prohibited
personal_device_required: "no"   # explicitly permitted: other devices and a static-IP VPS (G13); account owner must be the sole trader (G16)
status: partial
not_confirmed_fields: [copy_trading_other_firms, copy_trading_same_ip]
sources:
  - { field: drawdown, url: "https://help.toponefutures.com/en/articles/14595145-elite-access-maximum-loss-limit-mll-eod", accessed: "2026-10-23" }
  - { field: daily_loss_limit, url: "https://help.toponefutures.com/en/articles/14595138-elite-access-daily-loss-limit-dll", accessed: "2026-10-23" }
  - { field: profit_target_usd, url: "https://help.toponefutures.com/en/articles/14595073-elite-access-evaluation-profit-target", accessed: "2026-10-23" }
  - { field: consistency_rule, url: "https://help.toponefutures.com/en/articles/14595073-elite-access-evaluation-profit-target", accessed: "2026-10-23" }
  - { field: max_position, url: "https://help.toponefutures.com/en/articles/10906950-maximum-contracts-explained", accessed: "2026-10-23" }
  - { field: min_trading_days, url: "https://help.toponefutures.com/en/articles/14595016-elite-access-accounts-overview", accessed: "2026-10-23" }
  - { field: trading_day_reset, url: "https://help.toponefutures.com/en/articles/11165893-trading-hours-at-top-one-futures", accessed: "2026-10-23" }
  - { field: automated_trading_allowed, url: "https://help.toponefutures.com/en/articles/11021584-prohibited-trading-practices", accessed: "2026-10-23" }
  - { field: copy_trading_allowed, url: "https://help.toponefutures.com/en/articles/11680891-copy-trading-policy", accessed: "2026-10-23" }
  - { field: vps_allowed, url: "https://help.toponefutures.com/en/articles/11187841-are-there-ip-address-restrictions", accessed: "2026-10-23" }
  - { field: personal_device_required, url: "https://help.toponefutures.com/en/articles/11187841-are-there-ip-address-restrictions", accessed: "2026-10-23" }
  - { field: platforms, url: "https://help.toponefutures.com/en/articles/17149341-can-i-switch-my-trading-platform-tradovate-ninjatrader-or-blackarrow", accessed: "2026-10-23" }
```

```yaml
firm: top-one-futures
plan: "Elite ACCESS"
phase: evaluation
size_label: "$100K"
size_usd: "100000.00"
instruments: [MNQ, NQ]
platforms: ["Tradovate/NinjaTrader", BlackArrow, WealthCharts]   # firm-wide list; per-plan availability not published (see §2, §9)
drawdown: { amount_usd: "3000.00", type: trailing_eod, stops_trailing_at: "locks at breakeven + $100 ($100,100) once the balance reaches starting balance + drawdown + $100 (EI7 wording)" }
daily_loss_limit: { amount_usd: null, kind: none }   # no DLL in evaluation (EA5)
profit_target_usd: "6000.00"
consistency_rule: { text: null, threshold_pct: null }
max_position: { minis: 5, micros: 50, scaling: "none in evaluation (EA2)" }
min_trading_days: 1   # "Minimum of 1 trading day"; 30 days to pass (EA1)
trading_day_reset: { time: "18:00", tz_as_published: "EST" }   # sessions "6:00 PM EST to 4:10 PM EST"; forced flat 4:10 PM EST (G12)
copy_trading_allowed: conditional   # own accounts only, same program AND same size (G15)
automated_trading_allowed: "no"
automation_rule_text: "Use of any automated trading system, script, bot, Expert Advisor (EA), or high-frequency execution tool is strictly forbidden. All trades must be entered manually or semi-manually by the trader."
vps_allowed: conditional          # VPS with static IP, notify support in advance with IP + invoice (G13); VPN prohibited
personal_device_required: "no"   # explicitly permitted: other devices and a static-IP VPS (G13); account owner must be the sole trader (G16)
status: partial
not_confirmed_fields: [copy_trading_other_firms, copy_trading_same_ip]
sources:
  - { field: drawdown, url: "https://help.toponefutures.com/en/articles/14595145-elite-access-maximum-loss-limit-mll-eod", accessed: "2026-10-23" }
  - { field: daily_loss_limit, url: "https://help.toponefutures.com/en/articles/14595138-elite-access-daily-loss-limit-dll", accessed: "2026-10-23" }
  - { field: profit_target_usd, url: "https://help.toponefutures.com/en/articles/14595073-elite-access-evaluation-profit-target", accessed: "2026-10-23" }
  - { field: consistency_rule, url: "https://help.toponefutures.com/en/articles/14595073-elite-access-evaluation-profit-target", accessed: "2026-10-23" }
  - { field: max_position, url: "https://help.toponefutures.com/en/articles/10906950-maximum-contracts-explained", accessed: "2026-10-23" }
  - { field: min_trading_days, url: "https://help.toponefutures.com/en/articles/14595016-elite-access-accounts-overview", accessed: "2026-10-23" }
  - { field: trading_day_reset, url: "https://help.toponefutures.com/en/articles/11165893-trading-hours-at-top-one-futures", accessed: "2026-10-23" }
  - { field: automated_trading_allowed, url: "https://help.toponefutures.com/en/articles/11021584-prohibited-trading-practices", accessed: "2026-10-23" }
  - { field: copy_trading_allowed, url: "https://help.toponefutures.com/en/articles/11680891-copy-trading-policy", accessed: "2026-10-23" }
  - { field: vps_allowed, url: "https://help.toponefutures.com/en/articles/11187841-are-there-ip-address-restrictions", accessed: "2026-10-23" }
  - { field: personal_device_required, url: "https://help.toponefutures.com/en/articles/11187841-are-there-ip-address-restrictions", accessed: "2026-10-23" }
  - { field: platforms, url: "https://help.toponefutures.com/en/articles/17149341-can-i-switch-my-trading-platform-tradovate-ninjatrader-or-blackarrow", accessed: "2026-10-23" }
```

```yaml
firm: top-one-futures
plan: "Elite ACCESS"
phase: evaluation
size_label: "$150K"
size_usd: "150000.00"
instruments: [MNQ, NQ]
platforms: ["Tradovate/NinjaTrader", BlackArrow, WealthCharts]   # firm-wide list; per-plan availability not published (see §2, §9)
drawdown: { amount_usd: "4000.00", type: trailing_eod, stops_trailing_at: "locks at breakeven + $100 ($150,100) once the balance reaches starting balance + drawdown + $100 (EI7 wording)" }
daily_loss_limit: { amount_usd: null, kind: none }   # no DLL in evaluation (EA5)
profit_target_usd: "9000.00"
consistency_rule: { text: null, threshold_pct: null }
max_position: { minis: 7, micros: 70, scaling: "none in evaluation (EA2)" }
min_trading_days: 1   # "Minimum of 1 trading day"; 30 days to pass (EA1)
trading_day_reset: { time: "18:00", tz_as_published: "EST" }   # sessions "6:00 PM EST to 4:10 PM EST"; forced flat 4:10 PM EST (G12)
copy_trading_allowed: conditional   # own accounts only, same program AND same size (G15)
automated_trading_allowed: "no"
automation_rule_text: "Use of any automated trading system, script, bot, Expert Advisor (EA), or high-frequency execution tool is strictly forbidden. All trades must be entered manually or semi-manually by the trader."
vps_allowed: conditional          # VPS with static IP, notify support in advance with IP + invoice (G13); VPN prohibited
personal_device_required: "no"   # explicitly permitted: other devices and a static-IP VPS (G13); account owner must be the sole trader (G16)
status: partial
not_confirmed_fields: [copy_trading_other_firms, copy_trading_same_ip]
sources:
  - { field: drawdown, url: "https://help.toponefutures.com/en/articles/14595145-elite-access-maximum-loss-limit-mll-eod", accessed: "2026-10-23" }
  - { field: daily_loss_limit, url: "https://help.toponefutures.com/en/articles/14595138-elite-access-daily-loss-limit-dll", accessed: "2026-10-23" }
  - { field: profit_target_usd, url: "https://help.toponefutures.com/en/articles/14595073-elite-access-evaluation-profit-target", accessed: "2026-10-23" }
  - { field: consistency_rule, url: "https://help.toponefutures.com/en/articles/14595073-elite-access-evaluation-profit-target", accessed: "2026-10-23" }
  - { field: max_position, url: "https://help.toponefutures.com/en/articles/10906950-maximum-contracts-explained", accessed: "2026-10-23" }
  - { field: min_trading_days, url: "https://help.toponefutures.com/en/articles/14595016-elite-access-accounts-overview", accessed: "2026-10-23" }
  - { field: trading_day_reset, url: "https://help.toponefutures.com/en/articles/11165893-trading-hours-at-top-one-futures", accessed: "2026-10-23" }
  - { field: automated_trading_allowed, url: "https://help.toponefutures.com/en/articles/11021584-prohibited-trading-practices", accessed: "2026-10-23" }
  - { field: copy_trading_allowed, url: "https://help.toponefutures.com/en/articles/11680891-copy-trading-policy", accessed: "2026-10-23" }
  - { field: vps_allowed, url: "https://help.toponefutures.com/en/articles/11187841-are-there-ip-address-restrictions", accessed: "2026-10-23" }
  - { field: personal_device_required, url: "https://help.toponefutures.com/en/articles/11187841-are-there-ip-address-restrictions", accessed: "2026-10-23" }
  - { field: platforms, url: "https://help.toponefutures.com/en/articles/17149341-can-i-switch-my-trading-platform-tradovate-ninjatrader-or-blackarrow", accessed: "2026-10-23" }
```

```yaml
firm: top-one-futures
plan: "Elite ACCESS"
phase: funded
size_label: "$25K"
size_usd: "25000.00"
instruments: [MNQ, NQ]
platforms: ["Tradovate/NinjaTrader", BlackArrow, WealthCharts]   # firm-wide list; per-plan availability not published (see §2, §9)
drawdown: { amount_usd: "1000.00", type: trailing_eod, stops_trailing_at: "locks at breakeven + $100 ($25,100); also locks on first payout (G3)" }
daily_loss_limit: { amount_usd: "500.00", kind: soft_pause }   # soft breach: paused rest of day; No-DLL add-on (raises activation fee)
profit_target_usd: null   # none; payout needs buffer + $500 (§3.2)
consistency_rule: { text: "Your largest single trading day must not exceed 40% of your total profits in your current payout cycle.", threshold_pct: "40" }
max_position: { minis: 1, micros: 10, scaling: "Level 1: 1 mini (full allocation)" }
min_trading_days: 5   # 5 profitable days (min daily profit per size) before each payout (EA8)
trading_day_reset: { time: "18:00", tz_as_published: "EST" }   # sessions "6:00 PM EST to 4:10 PM EST"; forced flat 4:10 PM EST (G12)
copy_trading_allowed: conditional   # own accounts only, same program AND same size (G15)
automated_trading_allowed: "no"
automation_rule_text: "Use of any automated trading system, script, bot, Expert Advisor (EA), or high-frequency execution tool is strictly forbidden. All trades must be entered manually or semi-manually by the trader."
vps_allowed: conditional          # VPS with static IP, notify support in advance with IP + invoice (G13); VPN prohibited
personal_device_required: "no"   # explicitly permitted: other devices and a static-IP VPS (G13); account owner must be the sole trader (G16)
status: partial
not_confirmed_fields: [copy_trading_other_firms, copy_trading_same_ip]
sources:
  - { field: drawdown, url: "https://help.toponefutures.com/en/articles/14595145-elite-access-maximum-loss-limit-mll-eod", accessed: "2026-10-23" }
  - { field: daily_loss_limit, url: "https://help.toponefutures.com/en/articles/14595138-elite-access-daily-loss-limit-dll", accessed: "2026-10-23" }
  - { field: consistency_rule, url: "https://help.toponefutures.com/en/articles/14595151-elite-access-funded-consistency-rule-40", accessed: "2026-10-23" }
  - { field: max_position, url: "https://help.toponefutures.com/en/articles/14595053-elite-access-contract-scaling-plan", accessed: "2026-10-23" }
  - { field: min_trading_days, url: "https://help.toponefutures.com/en/articles/14595155-elite-access-minimum-profitable-trading-days-funded-only", accessed: "2026-10-23" }
  - { field: trading_day_reset, url: "https://help.toponefutures.com/en/articles/11165893-trading-hours-at-top-one-futures", accessed: "2026-10-23" }
  - { field: automated_trading_allowed, url: "https://help.toponefutures.com/en/articles/11021584-prohibited-trading-practices", accessed: "2026-10-23" }
  - { field: copy_trading_allowed, url: "https://help.toponefutures.com/en/articles/11680891-copy-trading-policy", accessed: "2026-10-23" }
  - { field: vps_allowed, url: "https://help.toponefutures.com/en/articles/11187841-are-there-ip-address-restrictions", accessed: "2026-10-23" }
  - { field: personal_device_required, url: "https://help.toponefutures.com/en/articles/11187841-are-there-ip-address-restrictions", accessed: "2026-10-23" }
  - { field: platforms, url: "https://help.toponefutures.com/en/articles/17149341-can-i-switch-my-trading-platform-tradovate-ninjatrader-or-blackarrow", accessed: "2026-10-23" }
```

```yaml
firm: top-one-futures
plan: "Elite ACCESS"
phase: funded
size_label: "$50K"
size_usd: "50000.00"
instruments: [MNQ, NQ]
platforms: ["Tradovate/NinjaTrader", BlackArrow, WealthCharts]   # firm-wide list; per-plan availability not published (see §2, §9)
drawdown: { amount_usd: "2000.00", type: trailing_eod, stops_trailing_at: "locks at breakeven + $100 ($50,100); also locks on first payout (G3)" }
daily_loss_limit: { amount_usd: "1000.00", kind: soft_pause }   # soft breach: paused rest of day; No-DLL add-on (raises activation fee)
profit_target_usd: null   # none; payout needs buffer + $500 (§3.2)
consistency_rule: { text: "Your largest single trading day must not exceed 40% of your total profits in your current payout cycle.", threshold_pct: "40" }
max_position: { minis: 3, micros: 30, scaling: "L1: 1 mini; L2: 2 minis from +$1,500; L3: 3 minis from +$2,000 (full); EOD balance, applies next trading day" }
min_trading_days: 5   # 5 profitable days (min daily profit per size) before each payout (EA8)
trading_day_reset: { time: "18:00", tz_as_published: "EST" }   # sessions "6:00 PM EST to 4:10 PM EST"; forced flat 4:10 PM EST (G12)
copy_trading_allowed: conditional   # own accounts only, same program AND same size (G15)
automated_trading_allowed: "no"
automation_rule_text: "Use of any automated trading system, script, bot, Expert Advisor (EA), or high-frequency execution tool is strictly forbidden. All trades must be entered manually or semi-manually by the trader."
vps_allowed: conditional          # VPS with static IP, notify support in advance with IP + invoice (G13); VPN prohibited
personal_device_required: "no"   # explicitly permitted: other devices and a static-IP VPS (G13); account owner must be the sole trader (G16)
status: partial
not_confirmed_fields: [copy_trading_other_firms, copy_trading_same_ip]
sources:
  - { field: drawdown, url: "https://help.toponefutures.com/en/articles/14595145-elite-access-maximum-loss-limit-mll-eod", accessed: "2026-10-23" }
  - { field: daily_loss_limit, url: "https://help.toponefutures.com/en/articles/14595138-elite-access-daily-loss-limit-dll", accessed: "2026-10-23" }
  - { field: consistency_rule, url: "https://help.toponefutures.com/en/articles/14595151-elite-access-funded-consistency-rule-40", accessed: "2026-10-23" }
  - { field: max_position, url: "https://help.toponefutures.com/en/articles/14595053-elite-access-contract-scaling-plan", accessed: "2026-10-23" }
  - { field: min_trading_days, url: "https://help.toponefutures.com/en/articles/14595155-elite-access-minimum-profitable-trading-days-funded-only", accessed: "2026-10-23" }
  - { field: trading_day_reset, url: "https://help.toponefutures.com/en/articles/11165893-trading-hours-at-top-one-futures", accessed: "2026-10-23" }
  - { field: automated_trading_allowed, url: "https://help.toponefutures.com/en/articles/11021584-prohibited-trading-practices", accessed: "2026-10-23" }
  - { field: copy_trading_allowed, url: "https://help.toponefutures.com/en/articles/11680891-copy-trading-policy", accessed: "2026-10-23" }
  - { field: vps_allowed, url: "https://help.toponefutures.com/en/articles/11187841-are-there-ip-address-restrictions", accessed: "2026-10-23" }
  - { field: personal_device_required, url: "https://help.toponefutures.com/en/articles/11187841-are-there-ip-address-restrictions", accessed: "2026-10-23" }
  - { field: platforms, url: "https://help.toponefutures.com/en/articles/17149341-can-i-switch-my-trading-platform-tradovate-ninjatrader-or-blackarrow", accessed: "2026-10-23" }
```

```yaml
firm: top-one-futures
plan: "Elite ACCESS"
phase: funded
size_label: "$100K"
size_usd: "100000.00"
instruments: [MNQ, NQ]
platforms: ["Tradovate/NinjaTrader", BlackArrow, WealthCharts]   # firm-wide list; per-plan availability not published (see §2, §9)
drawdown: { amount_usd: "3000.00", type: trailing_eod, stops_trailing_at: "locks at breakeven + $100 ($100,100); also locks on first payout (G3)" }
daily_loss_limit: { amount_usd: "1250.00", kind: soft_pause }   # soft breach: paused rest of day; No-DLL add-on (raises activation fee)
profit_target_usd: null   # none; payout needs buffer + $500 (§3.2)
consistency_rule: { text: "Your largest single trading day must not exceed 40% of your total profits in your current payout cycle.", threshold_pct: "40" }
max_position: { minis: 5, micros: 50, scaling: "L1: 2 minis; L2: 3 from +$1,500; L3: 4 from +$2,000; L4: 5 from +$3,000 (full); EOD balance, applies next trading day" }
min_trading_days: 5   # 5 profitable days (min daily profit per size) before each payout (EA8)
trading_day_reset: { time: "18:00", tz_as_published: "EST" }   # sessions "6:00 PM EST to 4:10 PM EST"; forced flat 4:10 PM EST (G12)
copy_trading_allowed: conditional   # own accounts only, same program AND same size (G15)
automated_trading_allowed: "no"
automation_rule_text: "Use of any automated trading system, script, bot, Expert Advisor (EA), or high-frequency execution tool is strictly forbidden. All trades must be entered manually or semi-manually by the trader."
vps_allowed: conditional          # VPS with static IP, notify support in advance with IP + invoice (G13); VPN prohibited
personal_device_required: "no"   # explicitly permitted: other devices and a static-IP VPS (G13); account owner must be the sole trader (G16)
status: partial
not_confirmed_fields: [copy_trading_other_firms, copy_trading_same_ip]
sources:
  - { field: drawdown, url: "https://help.toponefutures.com/en/articles/14595145-elite-access-maximum-loss-limit-mll-eod", accessed: "2026-10-23" }
  - { field: daily_loss_limit, url: "https://help.toponefutures.com/en/articles/14595138-elite-access-daily-loss-limit-dll", accessed: "2026-10-23" }
  - { field: consistency_rule, url: "https://help.toponefutures.com/en/articles/14595151-elite-access-funded-consistency-rule-40", accessed: "2026-10-23" }
  - { field: max_position, url: "https://help.toponefutures.com/en/articles/14595053-elite-access-contract-scaling-plan", accessed: "2026-10-23" }
  - { field: min_trading_days, url: "https://help.toponefutures.com/en/articles/14595155-elite-access-minimum-profitable-trading-days-funded-only", accessed: "2026-10-23" }
  - { field: trading_day_reset, url: "https://help.toponefutures.com/en/articles/11165893-trading-hours-at-top-one-futures", accessed: "2026-10-23" }
  - { field: automated_trading_allowed, url: "https://help.toponefutures.com/en/articles/11021584-prohibited-trading-practices", accessed: "2026-10-23" }
  - { field: copy_trading_allowed, url: "https://help.toponefutures.com/en/articles/11680891-copy-trading-policy", accessed: "2026-10-23" }
  - { field: vps_allowed, url: "https://help.toponefutures.com/en/articles/11187841-are-there-ip-address-restrictions", accessed: "2026-10-23" }
  - { field: personal_device_required, url: "https://help.toponefutures.com/en/articles/11187841-are-there-ip-address-restrictions", accessed: "2026-10-23" }
  - { field: platforms, url: "https://help.toponefutures.com/en/articles/17149341-can-i-switch-my-trading-platform-tradovate-ninjatrader-or-blackarrow", accessed: "2026-10-23" }
```

```yaml
firm: top-one-futures
plan: "Elite ACCESS"
phase: funded
size_label: "$150K"
size_usd: "150000.00"
instruments: [MNQ, NQ]
platforms: ["Tradovate/NinjaTrader", BlackArrow, WealthCharts]   # firm-wide list; per-plan availability not published (see §2, §9)
drawdown: { amount_usd: "3500.00", type: trailing_eod, stops_trailing_at: "locks at breakeven + $100 ($150,100); also locks on first payout (G3)" }
daily_loss_limit: { amount_usd: "1750.00", kind: soft_pause }   # CONFLICT: $1,750 (EA1, EA5, G2) vs $1,850 (homepage / elite-access / coupon cards)
profit_target_usd: null   # none; payout needs buffer + $500 (§3.2)
consistency_rule: { text: "Your largest single trading day must not exceed 40% of your total profits in your current payout cycle.", threshold_pct: "40" }
max_position: { minis: 7, micros: 70, scaling: "L1: 3 minis; L2: 4 from +$1,500; L3: 5 from +$2,500; L4: 6 from +$3,500; L5: 7 from +$4,500 (full); EOD balance, applies next trading day" }
min_trading_days: 5   # 5 profitable days (min daily profit per size) before each payout (EA8)
trading_day_reset: { time: "18:00", tz_as_published: "EST" }   # sessions "6:00 PM EST to 4:10 PM EST"; forced flat 4:10 PM EST (G12)
copy_trading_allowed: conditional   # own accounts only, same program AND same size (G15)
automated_trading_allowed: "no"
automation_rule_text: "Use of any automated trading system, script, bot, Expert Advisor (EA), or high-frequency execution tool is strictly forbidden. All trades must be entered manually or semi-manually by the trader."
vps_allowed: conditional          # VPS with static IP, notify support in advance with IP + invoice (G13); VPN prohibited
personal_device_required: "no"   # explicitly permitted: other devices and a static-IP VPS (G13); account owner must be the sole trader (G16)
status: partial
not_confirmed_fields: [daily_loss_limit, copy_trading_other_firms, copy_trading_same_ip]
sources:
  - { field: drawdown, url: "https://help.toponefutures.com/en/articles/14595145-elite-access-maximum-loss-limit-mll-eod", accessed: "2026-10-23" }
  - { field: daily_loss_limit, url: "https://help.toponefutures.com/en/articles/14595138-elite-access-daily-loss-limit-dll", accessed: "2026-10-23" }
  - { field: consistency_rule, url: "https://help.toponefutures.com/en/articles/14595151-elite-access-funded-consistency-rule-40", accessed: "2026-10-23" }
  - { field: max_position, url: "https://help.toponefutures.com/en/articles/14595053-elite-access-contract-scaling-plan", accessed: "2026-10-23" }
  - { field: min_trading_days, url: "https://help.toponefutures.com/en/articles/14595155-elite-access-minimum-profitable-trading-days-funded-only", accessed: "2026-10-23" }
  - { field: trading_day_reset, url: "https://help.toponefutures.com/en/articles/11165893-trading-hours-at-top-one-futures", accessed: "2026-10-23" }
  - { field: automated_trading_allowed, url: "https://help.toponefutures.com/en/articles/11021584-prohibited-trading-practices", accessed: "2026-10-23" }
  - { field: copy_trading_allowed, url: "https://help.toponefutures.com/en/articles/11680891-copy-trading-policy", accessed: "2026-10-23" }
  - { field: vps_allowed, url: "https://help.toponefutures.com/en/articles/11187841-are-there-ip-address-restrictions", accessed: "2026-10-23" }
  - { field: personal_device_required, url: "https://help.toponefutures.com/en/articles/11187841-are-there-ip-address-restrictions", accessed: "2026-10-23" }
  - { field: platforms, url: "https://help.toponefutures.com/en/articles/17149341-can-i-switch-my-trading-platform-tradovate-ninjatrader-or-blackarrow", accessed: "2026-10-23" }
```

### 8.3 Elite ACCESS Intraday (intraday funded drawdown)

```yaml
firm: top-one-futures
plan: "Elite ACCESS Intraday"
phase: evaluation
size_label: "$25K"
size_usd: "25000.00"
instruments: [MNQ, NQ]
platforms: ["Tradovate/NinjaTrader", BlackArrow, WealthCharts]   # firm-wide list; per-plan availability not published (see §2, §9)
drawdown: { amount_usd: "1000.00", type: trailing_eod, stops_trailing_at: "locks at starting balance + $100 ($25,100) once highest EOD balance reaches $26,100" }
daily_loss_limit: { amount_usd: null, kind: none }   # no DLL in evaluation (EI5)
profit_target_usd: "1500.00"
consistency_rule: { text: null, threshold_pct: null }
max_position: { minis: 1, micros: 10, scaling: "none in evaluation (EI2)" }
min_trading_days: 1   # "Minimum of 1 trading day"; 30 days to pass (EI1)
trading_day_reset: { time: "18:00", tz_as_published: "EST" }   # sessions "6:00 PM EST to 4:10 PM EST"; forced flat 4:10 PM EST (G12)
copy_trading_allowed: conditional   # own accounts only, same program AND same size (G15)
automated_trading_allowed: "no"
automation_rule_text: "Use of any automated trading system, script, bot, Expert Advisor (EA), or high-frequency execution tool is strictly forbidden. All trades must be entered manually or semi-manually by the trader."
vps_allowed: conditional          # VPS with static IP, notify support in advance with IP + invoice (G13); VPN prohibited
personal_device_required: "no"   # explicitly permitted: other devices and a static-IP VPS (G13); account owner must be the sole trader (G16)
status: partial
not_confirmed_fields: [copy_trading_other_firms, copy_trading_same_ip]
sources:
  - { field: drawdown, url: "https://help.toponefutures.com/en/articles/17059654-elite-access-intraday-maximum-loss-limit-evaluation-phase-eod", accessed: "2026-10-23" }
  - { field: daily_loss_limit, url: "https://help.toponefutures.com/en/articles/17059623-elite-access-intraday-daily-loss-limit-dll", accessed: "2026-10-23" }
  - { field: profit_target_usd, url: "https://help.toponefutures.com/en/articles/17059492-elite-access-intraday-evaluation-profit-target", accessed: "2026-10-23" }
  - { field: consistency_rule, url: "https://help.toponefutures.com/en/articles/17059492-elite-access-intraday-evaluation-profit-target", accessed: "2026-10-23" }
  - { field: max_position, url: "https://help.toponefutures.com/en/articles/17059431-elite-access-intraday-contract-scaling-plan", accessed: "2026-10-23" }
  - { field: min_trading_days, url: "https://help.toponefutures.com/en/articles/17059267-elite-access-intraday-accounts-overview", accessed: "2026-10-23" }
  - { field: trading_day_reset, url: "https://help.toponefutures.com/en/articles/11165893-trading-hours-at-top-one-futures", accessed: "2026-10-23" }
  - { field: automated_trading_allowed, url: "https://help.toponefutures.com/en/articles/11021584-prohibited-trading-practices", accessed: "2026-10-23" }
  - { field: copy_trading_allowed, url: "https://help.toponefutures.com/en/articles/11680891-copy-trading-policy", accessed: "2026-10-23" }
  - { field: vps_allowed, url: "https://help.toponefutures.com/en/articles/11187841-are-there-ip-address-restrictions", accessed: "2026-10-23" }
  - { field: personal_device_required, url: "https://help.toponefutures.com/en/articles/11187841-are-there-ip-address-restrictions", accessed: "2026-10-23" }
  - { field: platforms, url: "https://help.toponefutures.com/en/articles/17149341-can-i-switch-my-trading-platform-tradovate-ninjatrader-or-blackarrow", accessed: "2026-10-23" }
```

```yaml
firm: top-one-futures
plan: "Elite ACCESS Intraday"
phase: evaluation
size_label: "$50K"
size_usd: "50000.00"
instruments: [MNQ, NQ]
platforms: ["Tradovate/NinjaTrader", BlackArrow, WealthCharts]   # firm-wide list; per-plan availability not published (see §2, §9)
drawdown: { amount_usd: "2000.00", type: trailing_eod, stops_trailing_at: "locks at starting balance + $100 ($50,100) once highest EOD balance reaches $52,100" }
daily_loss_limit: { amount_usd: null, kind: none }   # no DLL in evaluation (EI5)
profit_target_usd: "3000.00"
consistency_rule: { text: null, threshold_pct: null }
max_position: { minis: 3, micros: 30, scaling: "none in evaluation (EI2)" }
min_trading_days: 1   # "Minimum of 1 trading day"; 30 days to pass (EI1)
trading_day_reset: { time: "18:00", tz_as_published: "EST" }   # sessions "6:00 PM EST to 4:10 PM EST"; forced flat 4:10 PM EST (G12)
copy_trading_allowed: conditional   # own accounts only, same program AND same size (G15)
automated_trading_allowed: "no"
automation_rule_text: "Use of any automated trading system, script, bot, Expert Advisor (EA), or high-frequency execution tool is strictly forbidden. All trades must be entered manually or semi-manually by the trader."
vps_allowed: conditional          # VPS with static IP, notify support in advance with IP + invoice (G13); VPN prohibited
personal_device_required: "no"   # explicitly permitted: other devices and a static-IP VPS (G13); account owner must be the sole trader (G16)
status: partial
not_confirmed_fields: [copy_trading_other_firms, copy_trading_same_ip]
sources:
  - { field: drawdown, url: "https://help.toponefutures.com/en/articles/17059654-elite-access-intraday-maximum-loss-limit-evaluation-phase-eod", accessed: "2026-10-23" }
  - { field: daily_loss_limit, url: "https://help.toponefutures.com/en/articles/17059623-elite-access-intraday-daily-loss-limit-dll", accessed: "2026-10-23" }
  - { field: profit_target_usd, url: "https://help.toponefutures.com/en/articles/17059492-elite-access-intraday-evaluation-profit-target", accessed: "2026-10-23" }
  - { field: consistency_rule, url: "https://help.toponefutures.com/en/articles/17059492-elite-access-intraday-evaluation-profit-target", accessed: "2026-10-23" }
  - { field: max_position, url: "https://help.toponefutures.com/en/articles/17059431-elite-access-intraday-contract-scaling-plan", accessed: "2026-10-23" }
  - { field: min_trading_days, url: "https://help.toponefutures.com/en/articles/17059267-elite-access-intraday-accounts-overview", accessed: "2026-10-23" }
  - { field: trading_day_reset, url: "https://help.toponefutures.com/en/articles/11165893-trading-hours-at-top-one-futures", accessed: "2026-10-23" }
  - { field: automated_trading_allowed, url: "https://help.toponefutures.com/en/articles/11021584-prohibited-trading-practices", accessed: "2026-10-23" }
  - { field: copy_trading_allowed, url: "https://help.toponefutures.com/en/articles/11680891-copy-trading-policy", accessed: "2026-10-23" }
  - { field: vps_allowed, url: "https://help.toponefutures.com/en/articles/11187841-are-there-ip-address-restrictions", accessed: "2026-10-23" }
  - { field: personal_device_required, url: "https://help.toponefutures.com/en/articles/11187841-are-there-ip-address-restrictions", accessed: "2026-10-23" }
  - { field: platforms, url: "https://help.toponefutures.com/en/articles/17149341-can-i-switch-my-trading-platform-tradovate-ninjatrader-or-blackarrow", accessed: "2026-10-23" }
```

```yaml
firm: top-one-futures
plan: "Elite ACCESS Intraday"
phase: evaluation
size_label: "$100K"
size_usd: "100000.00"
instruments: [MNQ, NQ]
platforms: ["Tradovate/NinjaTrader", BlackArrow, WealthCharts]   # firm-wide list; per-plan availability not published (see §2, §9)
drawdown: { amount_usd: "3000.00", type: trailing_eod, stops_trailing_at: "locks at starting balance + $100 ($100,100) once highest EOD balance reaches $103,100" }
daily_loss_limit: { amount_usd: null, kind: none }   # no DLL in evaluation (EI5)
profit_target_usd: "6000.00"
consistency_rule: { text: null, threshold_pct: null }
max_position: { minis: 5, micros: 50, scaling: "none in evaluation (EI2)" }
min_trading_days: 1   # "Minimum of 1 trading day"; 30 days to pass (EI1)
trading_day_reset: { time: "18:00", tz_as_published: "EST" }   # sessions "6:00 PM EST to 4:10 PM EST"; forced flat 4:10 PM EST (G12)
copy_trading_allowed: conditional   # own accounts only, same program AND same size (G15)
automated_trading_allowed: "no"
automation_rule_text: "Use of any automated trading system, script, bot, Expert Advisor (EA), or high-frequency execution tool is strictly forbidden. All trades must be entered manually or semi-manually by the trader."
vps_allowed: conditional          # VPS with static IP, notify support in advance with IP + invoice (G13); VPN prohibited
personal_device_required: "no"   # explicitly permitted: other devices and a static-IP VPS (G13); account owner must be the sole trader (G16)
status: partial
not_confirmed_fields: [copy_trading_other_firms, copy_trading_same_ip]
sources:
  - { field: drawdown, url: "https://help.toponefutures.com/en/articles/17059654-elite-access-intraday-maximum-loss-limit-evaluation-phase-eod", accessed: "2026-10-23" }
  - { field: daily_loss_limit, url: "https://help.toponefutures.com/en/articles/17059623-elite-access-intraday-daily-loss-limit-dll", accessed: "2026-10-23" }
  - { field: profit_target_usd, url: "https://help.toponefutures.com/en/articles/17059492-elite-access-intraday-evaluation-profit-target", accessed: "2026-10-23" }
  - { field: consistency_rule, url: "https://help.toponefutures.com/en/articles/17059492-elite-access-intraday-evaluation-profit-target", accessed: "2026-10-23" }
  - { field: max_position, url: "https://help.toponefutures.com/en/articles/17059431-elite-access-intraday-contract-scaling-plan", accessed: "2026-10-23" }
  - { field: min_trading_days, url: "https://help.toponefutures.com/en/articles/17059267-elite-access-intraday-accounts-overview", accessed: "2026-10-23" }
  - { field: trading_day_reset, url: "https://help.toponefutures.com/en/articles/11165893-trading-hours-at-top-one-futures", accessed: "2026-10-23" }
  - { field: automated_trading_allowed, url: "https://help.toponefutures.com/en/articles/11021584-prohibited-trading-practices", accessed: "2026-10-23" }
  - { field: copy_trading_allowed, url: "https://help.toponefutures.com/en/articles/11680891-copy-trading-policy", accessed: "2026-10-23" }
  - { field: vps_allowed, url: "https://help.toponefutures.com/en/articles/11187841-are-there-ip-address-restrictions", accessed: "2026-10-23" }
  - { field: personal_device_required, url: "https://help.toponefutures.com/en/articles/11187841-are-there-ip-address-restrictions", accessed: "2026-10-23" }
  - { field: platforms, url: "https://help.toponefutures.com/en/articles/17149341-can-i-switch-my-trading-platform-tradovate-ninjatrader-or-blackarrow", accessed: "2026-10-23" }
```

```yaml
firm: top-one-futures
plan: "Elite ACCESS Intraday"
phase: evaluation
size_label: "$150K"
size_usd: "150000.00"
instruments: [MNQ, NQ]
platforms: ["Tradovate/NinjaTrader", BlackArrow, WealthCharts]   # firm-wide list; per-plan availability not published (see §2, §9)
drawdown: { amount_usd: "4000.00", type: trailing_eod, stops_trailing_at: "locks at starting balance + $100 ($150,100) once highest EOD balance reaches $154,100" }
daily_loss_limit: { amount_usd: null, kind: none }   # no DLL in evaluation (EI5)
profit_target_usd: "9000.00"
consistency_rule: { text: null, threshold_pct: null }
max_position: { minis: 7, micros: 70, scaling: "none in evaluation (EI2)" }
min_trading_days: 1   # "Minimum of 1 trading day"; 30 days to pass (EI1)
trading_day_reset: { time: "18:00", tz_as_published: "EST" }   # sessions "6:00 PM EST to 4:10 PM EST"; forced flat 4:10 PM EST (G12)
copy_trading_allowed: conditional   # own accounts only, same program AND same size (G15)
automated_trading_allowed: "no"
automation_rule_text: "Use of any automated trading system, script, bot, Expert Advisor (EA), or high-frequency execution tool is strictly forbidden. All trades must be entered manually or semi-manually by the trader."
vps_allowed: conditional          # VPS with static IP, notify support in advance with IP + invoice (G13); VPN prohibited
personal_device_required: "no"   # explicitly permitted: other devices and a static-IP VPS (G13); account owner must be the sole trader (G16)
status: partial
not_confirmed_fields: [copy_trading_other_firms, copy_trading_same_ip]
sources:
  - { field: drawdown, url: "https://help.toponefutures.com/en/articles/17059654-elite-access-intraday-maximum-loss-limit-evaluation-phase-eod", accessed: "2026-10-23" }
  - { field: daily_loss_limit, url: "https://help.toponefutures.com/en/articles/17059623-elite-access-intraday-daily-loss-limit-dll", accessed: "2026-10-23" }
  - { field: profit_target_usd, url: "https://help.toponefutures.com/en/articles/17059492-elite-access-intraday-evaluation-profit-target", accessed: "2026-10-23" }
  - { field: consistency_rule, url: "https://help.toponefutures.com/en/articles/17059492-elite-access-intraday-evaluation-profit-target", accessed: "2026-10-23" }
  - { field: max_position, url: "https://help.toponefutures.com/en/articles/17059431-elite-access-intraday-contract-scaling-plan", accessed: "2026-10-23" }
  - { field: min_trading_days, url: "https://help.toponefutures.com/en/articles/17059267-elite-access-intraday-accounts-overview", accessed: "2026-10-23" }
  - { field: trading_day_reset, url: "https://help.toponefutures.com/en/articles/11165893-trading-hours-at-top-one-futures", accessed: "2026-10-23" }
  - { field: automated_trading_allowed, url: "https://help.toponefutures.com/en/articles/11021584-prohibited-trading-practices", accessed: "2026-10-23" }
  - { field: copy_trading_allowed, url: "https://help.toponefutures.com/en/articles/11680891-copy-trading-policy", accessed: "2026-10-23" }
  - { field: vps_allowed, url: "https://help.toponefutures.com/en/articles/11187841-are-there-ip-address-restrictions", accessed: "2026-10-23" }
  - { field: personal_device_required, url: "https://help.toponefutures.com/en/articles/11187841-are-there-ip-address-restrictions", accessed: "2026-10-23" }
  - { field: platforms, url: "https://help.toponefutures.com/en/articles/17149341-can-i-switch-my-trading-platform-tradovate-ninjatrader-or-blackarrow", accessed: "2026-10-23" }
```

```yaml
firm: top-one-futures
plan: "Elite ACCESS Intraday"
phase: funded
size_label: "$25K"
size_usd: "25000.00"
instruments: [MNQ, NQ]
platforms: ["Tradovate/NinjaTrader", BlackArrow, WealthCharts]   # firm-wide list; per-plan availability not published (see §2, §9)
drawdown: { amount_usd: "1000.00", type: trailing_intraday, stops_trailing_at: "keeps trailing (tick by tick, incl. unrealized) until your first payout request, then locks at starting balance + $100 ($25,100)" }
daily_loss_limit: { amount_usd: "500.00", kind: soft_pause }   # soft breach: open positions closed, paused until rollover
profit_target_usd: null   # none; payout needs buffer + $500 (§3.3)
consistency_rule: { text: "Your largest single trading day must not exceed 40% of your total profits in your current payout cycle.", threshold_pct: "40" }
max_position: { minis: 1, micros: 10, scaling: "Level 1: 1 mini (full allocation)" }
min_trading_days: 5   # 5 profitable days before each payout (EI9)
trading_day_reset: { time: "18:00", tz_as_published: "EST" }   # sessions "6:00 PM EST to 4:10 PM EST"; forced flat 4:10 PM EST (G12)
copy_trading_allowed: conditional   # own accounts only, same program AND same size (G15)
automated_trading_allowed: "no"
automation_rule_text: "Use of any automated trading system, script, bot, Expert Advisor (EA), or high-frequency execution tool is strictly forbidden. All trades must be entered manually or semi-manually by the trader."
vps_allowed: conditional          # VPS with static IP, notify support in advance with IP + invoice (G13); VPN prohibited
personal_device_required: "no"   # explicitly permitted: other devices and a static-IP VPS (G13); account owner must be the sole trader (G16)
status: partial
not_confirmed_fields: [copy_trading_other_firms, copy_trading_same_ip]
sources:
  - { field: drawdown, url: "https://help.toponefutures.com/en/articles/17059688-elite-access-intraday-maximum-loss-limit-funded-phase-intraday", accessed: "2026-10-23" }
  - { field: daily_loss_limit, url: "https://help.toponefutures.com/en/articles/17059623-elite-access-intraday-daily-loss-limit-dll", accessed: "2026-10-23" }
  - { field: consistency_rule, url: "https://help.toponefutures.com/en/articles/17059698-elite-access-intraday-consistency-rule-40-funded", accessed: "2026-10-23" }
  - { field: max_position, url: "https://help.toponefutures.com/en/articles/17059431-elite-access-intraday-contract-scaling-plan", accessed: "2026-10-23" }
  - { field: min_trading_days, url: "https://help.toponefutures.com/en/articles/17059750-elite-access-intraday-minimum-profitable-days-funded-only", accessed: "2026-10-23" }
  - { field: trading_day_reset, url: "https://help.toponefutures.com/en/articles/11165893-trading-hours-at-top-one-futures", accessed: "2026-10-23" }
  - { field: automated_trading_allowed, url: "https://help.toponefutures.com/en/articles/11021584-prohibited-trading-practices", accessed: "2026-10-23" }
  - { field: copy_trading_allowed, url: "https://help.toponefutures.com/en/articles/11680891-copy-trading-policy", accessed: "2026-10-23" }
  - { field: vps_allowed, url: "https://help.toponefutures.com/en/articles/11187841-are-there-ip-address-restrictions", accessed: "2026-10-23" }
  - { field: personal_device_required, url: "https://help.toponefutures.com/en/articles/11187841-are-there-ip-address-restrictions", accessed: "2026-10-23" }
  - { field: platforms, url: "https://help.toponefutures.com/en/articles/17149341-can-i-switch-my-trading-platform-tradovate-ninjatrader-or-blackarrow", accessed: "2026-10-23" }
```

```yaml
firm: top-one-futures
plan: "Elite ACCESS Intraday"
phase: funded
size_label: "$50K"
size_usd: "50000.00"
instruments: [MNQ, NQ]
platforms: ["Tradovate/NinjaTrader", BlackArrow, WealthCharts]   # firm-wide list; per-plan availability not published (see §2, §9)
drawdown: { amount_usd: "2000.00", type: trailing_intraday, stops_trailing_at: "keeps trailing (tick by tick, incl. unrealized) until your first payout request, then locks at starting balance + $100 ($50,100)" }
daily_loss_limit: { amount_usd: "1000.00", kind: soft_pause }   # soft breach: open positions closed, paused until rollover
profit_target_usd: null   # none; payout needs buffer + $500 (§3.3)
consistency_rule: { text: "Your largest single trading day must not exceed 40% of your total profits in your current payout cycle.", threshold_pct: "40" }
max_position: { minis: 3, micros: 30, scaling: "L1: 1 mini; L2: 2 minis from +$1,500; L3: 3 minis from +$2,000 (full); EOD balance, applies next trading day" }
min_trading_days: 5   # 5 profitable days before each payout (EI9)
trading_day_reset: { time: "18:00", tz_as_published: "EST" }   # sessions "6:00 PM EST to 4:10 PM EST"; forced flat 4:10 PM EST (G12)
copy_trading_allowed: conditional   # own accounts only, same program AND same size (G15)
automated_trading_allowed: "no"
automation_rule_text: "Use of any automated trading system, script, bot, Expert Advisor (EA), or high-frequency execution tool is strictly forbidden. All trades must be entered manually or semi-manually by the trader."
vps_allowed: conditional          # VPS with static IP, notify support in advance with IP + invoice (G13); VPN prohibited
personal_device_required: "no"   # explicitly permitted: other devices and a static-IP VPS (G13); account owner must be the sole trader (G16)
status: partial
not_confirmed_fields: [copy_trading_other_firms, copy_trading_same_ip]
sources:
  - { field: drawdown, url: "https://help.toponefutures.com/en/articles/17059688-elite-access-intraday-maximum-loss-limit-funded-phase-intraday", accessed: "2026-10-23" }
  - { field: daily_loss_limit, url: "https://help.toponefutures.com/en/articles/17059623-elite-access-intraday-daily-loss-limit-dll", accessed: "2026-10-23" }
  - { field: consistency_rule, url: "https://help.toponefutures.com/en/articles/17059698-elite-access-intraday-consistency-rule-40-funded", accessed: "2026-10-23" }
  - { field: max_position, url: "https://help.toponefutures.com/en/articles/17059431-elite-access-intraday-contract-scaling-plan", accessed: "2026-10-23" }
  - { field: min_trading_days, url: "https://help.toponefutures.com/en/articles/17059750-elite-access-intraday-minimum-profitable-days-funded-only", accessed: "2026-10-23" }
  - { field: trading_day_reset, url: "https://help.toponefutures.com/en/articles/11165893-trading-hours-at-top-one-futures", accessed: "2026-10-23" }
  - { field: automated_trading_allowed, url: "https://help.toponefutures.com/en/articles/11021584-prohibited-trading-practices", accessed: "2026-10-23" }
  - { field: copy_trading_allowed, url: "https://help.toponefutures.com/en/articles/11680891-copy-trading-policy", accessed: "2026-10-23" }
  - { field: vps_allowed, url: "https://help.toponefutures.com/en/articles/11187841-are-there-ip-address-restrictions", accessed: "2026-10-23" }
  - { field: personal_device_required, url: "https://help.toponefutures.com/en/articles/11187841-are-there-ip-address-restrictions", accessed: "2026-10-23" }
  - { field: platforms, url: "https://help.toponefutures.com/en/articles/17149341-can-i-switch-my-trading-platform-tradovate-ninjatrader-or-blackarrow", accessed: "2026-10-23" }
```

```yaml
firm: top-one-futures
plan: "Elite ACCESS Intraday"
phase: funded
size_label: "$100K"
size_usd: "100000.00"
instruments: [MNQ, NQ]
platforms: ["Tradovate/NinjaTrader", BlackArrow, WealthCharts]   # firm-wide list; per-plan availability not published (see §2, §9)
drawdown: { amount_usd: "3000.00", type: trailing_intraday, stops_trailing_at: "keeps trailing (tick by tick, incl. unrealized) until your first payout request, then locks at starting balance + $100 ($100,100)" }
daily_loss_limit: { amount_usd: "1250.00", kind: soft_pause }   # soft breach: open positions closed, paused until rollover
profit_target_usd: null   # none; payout needs buffer + $500 (§3.3)
consistency_rule: { text: "Your largest single trading day must not exceed 40% of your total profits in your current payout cycle.", threshold_pct: "40" }
max_position: { minis: 5, micros: 50, scaling: "L1: 2 minis; L2: 3 from +$1,500; L3: 4 from +$2,000; L4: 5 from +$3,000 (full); EOD balance, applies next trading day" }
min_trading_days: 5   # 5 profitable days before each payout (EI9)
trading_day_reset: { time: "18:00", tz_as_published: "EST" }   # sessions "6:00 PM EST to 4:10 PM EST"; forced flat 4:10 PM EST (G12)
copy_trading_allowed: conditional   # own accounts only, same program AND same size (G15)
automated_trading_allowed: "no"
automation_rule_text: "Use of any automated trading system, script, bot, Expert Advisor (EA), or high-frequency execution tool is strictly forbidden. All trades must be entered manually or semi-manually by the trader."
vps_allowed: conditional          # VPS with static IP, notify support in advance with IP + invoice (G13); VPN prohibited
personal_device_required: "no"   # explicitly permitted: other devices and a static-IP VPS (G13); account owner must be the sole trader (G16)
status: partial
not_confirmed_fields: [copy_trading_other_firms, copy_trading_same_ip]
sources:
  - { field: drawdown, url: "https://help.toponefutures.com/en/articles/17059688-elite-access-intraday-maximum-loss-limit-funded-phase-intraday", accessed: "2026-10-23" }
  - { field: daily_loss_limit, url: "https://help.toponefutures.com/en/articles/17059623-elite-access-intraday-daily-loss-limit-dll", accessed: "2026-10-23" }
  - { field: consistency_rule, url: "https://help.toponefutures.com/en/articles/17059698-elite-access-intraday-consistency-rule-40-funded", accessed: "2026-10-23" }
  - { field: max_position, url: "https://help.toponefutures.com/en/articles/17059431-elite-access-intraday-contract-scaling-plan", accessed: "2026-10-23" }
  - { field: min_trading_days, url: "https://help.toponefutures.com/en/articles/17059750-elite-access-intraday-minimum-profitable-days-funded-only", accessed: "2026-10-23" }
  - { field: trading_day_reset, url: "https://help.toponefutures.com/en/articles/11165893-trading-hours-at-top-one-futures", accessed: "2026-10-23" }
  - { field: automated_trading_allowed, url: "https://help.toponefutures.com/en/articles/11021584-prohibited-trading-practices", accessed: "2026-10-23" }
  - { field: copy_trading_allowed, url: "https://help.toponefutures.com/en/articles/11680891-copy-trading-policy", accessed: "2026-10-23" }
  - { field: vps_allowed, url: "https://help.toponefutures.com/en/articles/11187841-are-there-ip-address-restrictions", accessed: "2026-10-23" }
  - { field: personal_device_required, url: "https://help.toponefutures.com/en/articles/11187841-are-there-ip-address-restrictions", accessed: "2026-10-23" }
  - { field: platforms, url: "https://help.toponefutures.com/en/articles/17149341-can-i-switch-my-trading-platform-tradovate-ninjatrader-or-blackarrow", accessed: "2026-10-23" }
```

```yaml
firm: top-one-futures
plan: "Elite ACCESS Intraday"
phase: funded
size_label: "$150K"
size_usd: "150000.00"
instruments: [MNQ, NQ]
platforms: ["Tradovate/NinjaTrader", BlackArrow, WealthCharts]   # firm-wide list; per-plan availability not published (see §2, §9)
drawdown: { amount_usd: "3500.00", type: trailing_intraday, stops_trailing_at: "keeps trailing (tick by tick, incl. unrealized) until your first payout request, then locks at starting balance + $100 ($150,100)" }
daily_loss_limit: { amount_usd: "1750.00", kind: soft_pause }   # CONFLICT: $1,750 (EI1, EI5, G2) vs $1,850 (homepage / coupon Elite Access cards)
profit_target_usd: null   # none; payout needs buffer + $500 (§3.3)
consistency_rule: { text: "Your largest single trading day must not exceed 40% of your total profits in your current payout cycle.", threshold_pct: "40" }
max_position: { minis: 7, micros: 70, scaling: "L1: 3 minis; L2: 4 from +$1,500; L3: 5 from +$2,500; L4: 6 from +$3,500; L5: 7 from +$4,500 (full); EOD balance, applies next trading day" }
min_trading_days: 5   # 5 profitable days before each payout (EI9)
trading_day_reset: { time: "18:00", tz_as_published: "EST" }   # sessions "6:00 PM EST to 4:10 PM EST"; forced flat 4:10 PM EST (G12)
copy_trading_allowed: conditional   # own accounts only, same program AND same size (G15)
automated_trading_allowed: "no"
automation_rule_text: "Use of any automated trading system, script, bot, Expert Advisor (EA), or high-frequency execution tool is strictly forbidden. All trades must be entered manually or semi-manually by the trader."
vps_allowed: conditional          # VPS with static IP, notify support in advance with IP + invoice (G13); VPN prohibited
personal_device_required: "no"   # explicitly permitted: other devices and a static-IP VPS (G13); account owner must be the sole trader (G16)
status: partial
not_confirmed_fields: [daily_loss_limit, copy_trading_other_firms, copy_trading_same_ip]
sources:
  - { field: drawdown, url: "https://help.toponefutures.com/en/articles/17059688-elite-access-intraday-maximum-loss-limit-funded-phase-intraday", accessed: "2026-10-23" }
  - { field: daily_loss_limit, url: "https://help.toponefutures.com/en/articles/17059623-elite-access-intraday-daily-loss-limit-dll", accessed: "2026-10-23" }
  - { field: consistency_rule, url: "https://help.toponefutures.com/en/articles/17059698-elite-access-intraday-consistency-rule-40-funded", accessed: "2026-10-23" }
  - { field: max_position, url: "https://help.toponefutures.com/en/articles/17059431-elite-access-intraday-contract-scaling-plan", accessed: "2026-10-23" }
  - { field: min_trading_days, url: "https://help.toponefutures.com/en/articles/17059750-elite-access-intraday-minimum-profitable-days-funded-only", accessed: "2026-10-23" }
  - { field: trading_day_reset, url: "https://help.toponefutures.com/en/articles/11165893-trading-hours-at-top-one-futures", accessed: "2026-10-23" }
  - { field: automated_trading_allowed, url: "https://help.toponefutures.com/en/articles/11021584-prohibited-trading-practices", accessed: "2026-10-23" }
  - { field: copy_trading_allowed, url: "https://help.toponefutures.com/en/articles/11680891-copy-trading-policy", accessed: "2026-10-23" }
  - { field: vps_allowed, url: "https://help.toponefutures.com/en/articles/11187841-are-there-ip-address-restrictions", accessed: "2026-10-23" }
  - { field: personal_device_required, url: "https://help.toponefutures.com/en/articles/11187841-are-there-ip-address-restrictions", accessed: "2026-10-23" }
  - { field: platforms, url: "https://help.toponefutures.com/en/articles/17149341-can-i-switch-my-trading-platform-tradovate-ninjatrader-or-blackarrow", accessed: "2026-10-23" }
```

### 8.4 Instant Sim Funded 2.0 (purchased on or after 7/27/2026; no evaluation)

```yaml
firm: top-one-futures
plan: "Instant Sim Funded"
phase: funded
size_label: "$25K"
size_usd: "25000.00"
instruments: [MNQ, NQ]
platforms: ["Tradovate/NinjaTrader", BlackArrow, WealthCharts]   # firm-wide list; per-plan availability not published (see §2, §9)
drawdown: { amount_usd: "1000.00", type: trailing_eod, stops_trailing_at: "locks at starting balance + $100 ($25,100) once balance reaches $26,100; also on first payout" }
daily_loss_limit: { amount_usd: "625.00", kind: soft_pause }   # soft breach (IS1, G2)
profit_target_usd: null   # none to keep account; payout target 6% / 5% / 4% per cycle (§3.4)
consistency_rule: { text: "no single day's profit may exceed 20% of your total profits", threshold_pct: "20" }
max_position: { minis: 1, micros: 10, scaling: "none; fixed limit from day one (IS1)" }
min_trading_days: "none"   # "Minimum trading days: None" (Q1 comparison table)
trading_day_reset: { time: "17:00", tz_as_published: "EST" }   # EOD / consistency day "ends at 5:00 pm EST" (IS3, IS4); sessions 6:00 PM-4:10 PM EST (G12)
copy_trading_allowed: conditional   # own accounts only, same program AND same size (G15)
automated_trading_allowed: "no"
automation_rule_text: "Use of any automated trading system, script, bot, Expert Advisor (EA), or high-frequency execution tool is strictly forbidden. All trades must be entered manually or semi-manually by the trader."
vps_allowed: conditional          # VPS with static IP, notify support in advance with IP + invoice (G13); VPN prohibited
personal_device_required: "no"   # explicitly permitted: other devices and a static-IP VPS (G13); account owner must be the sole trader (G16)
status: partial
not_confirmed_fields: [max_accounts, copy_trading_other_firms, copy_trading_same_ip]
sources:
  - { field: drawdown, url: "https://help.toponefutures.com/en/articles/10908119-trailing-drawdown-on-instant-sim-funded-accounts-how-it-works", accessed: "2026-10-23" }
  - { field: daily_loss_limit, url: "https://help.toponefutures.com/en/articles/11020805-overview-of-the-instant-sim-funded-program", accessed: "2026-10-23" }
  - { field: consistency_rule, url: "https://help.toponefutures.com/en/articles/10908767-consistency-rule-in-instant-sim-funded-accounts-explained", accessed: "2026-10-23" }
  - { field: max_position, url: "https://help.toponefutures.com/en/articles/10906950-maximum-contracts-explained", accessed: "2026-10-23" }
  - { field: min_trading_days, url: "https://help.toponefutures.com/en/articles/11020805-overview-of-the-instant-sim-funded-program", accessed: "2026-10-23" }
  - { field: trading_day_reset, url: "https://help.toponefutures.com/en/articles/10908119-trailing-drawdown-on-instant-sim-funded-accounts-how-it-works", accessed: "2026-10-23" }
  - { field: automated_trading_allowed, url: "https://help.toponefutures.com/en/articles/11021584-prohibited-trading-practices", accessed: "2026-10-23" }
  - { field: copy_trading_allowed, url: "https://help.toponefutures.com/en/articles/11680891-copy-trading-policy", accessed: "2026-10-23" }
  - { field: vps_allowed, url: "https://help.toponefutures.com/en/articles/11187841-are-there-ip-address-restrictions", accessed: "2026-10-23" }
  - { field: personal_device_required, url: "https://help.toponefutures.com/en/articles/11187841-are-there-ip-address-restrictions", accessed: "2026-10-23" }
  - { field: platforms, url: "https://help.toponefutures.com/en/articles/17149341-can-i-switch-my-trading-platform-tradovate-ninjatrader-or-blackarrow", accessed: "2026-10-23" }
```

```yaml
firm: top-one-futures
plan: "Instant Sim Funded"
phase: funded
size_label: "$50K"
size_usd: "50000.00"
instruments: [MNQ, NQ]
platforms: ["Tradovate/NinjaTrader", BlackArrow, WealthCharts]   # firm-wide list; per-plan availability not published (see §2, §9)
drawdown: { amount_usd: "2000.00", type: trailing_eod, stops_trailing_at: "locks at starting balance + $100 ($50,100) once balance reaches $52,100; also on first payout" }
daily_loss_limit: { amount_usd: "1250.00", kind: soft_pause }   # soft breach (IS1, G2)
profit_target_usd: null   # none to keep account; payout target 6% / 5% / 4% per cycle (§3.4)
consistency_rule: { text: "no single day's profit may exceed 20% of your total profits", threshold_pct: "20" }
max_position: { minis: 3, micros: 30, scaling: "none; fixed limit from day one (IS1)" }
min_trading_days: "none"   # "Minimum trading days: None" (Q1 comparison table)
trading_day_reset: { time: "17:00", tz_as_published: "EST" }   # EOD / consistency day "ends at 5:00 pm EST" (IS3, IS4); sessions 6:00 PM-4:10 PM EST (G12)
copy_trading_allowed: conditional   # own accounts only, same program AND same size (G15)
automated_trading_allowed: "no"
automation_rule_text: "Use of any automated trading system, script, bot, Expert Advisor (EA), or high-frequency execution tool is strictly forbidden. All trades must be entered manually or semi-manually by the trader."
vps_allowed: conditional          # VPS with static IP, notify support in advance with IP + invoice (G13); VPN prohibited
personal_device_required: "no"   # explicitly permitted: other devices and a static-IP VPS (G13); account owner must be the sole trader (G16)
status: partial
not_confirmed_fields: [max_accounts, copy_trading_other_firms, copy_trading_same_ip]
sources:
  - { field: drawdown, url: "https://help.toponefutures.com/en/articles/10908119-trailing-drawdown-on-instant-sim-funded-accounts-how-it-works", accessed: "2026-10-23" }
  - { field: daily_loss_limit, url: "https://help.toponefutures.com/en/articles/11020805-overview-of-the-instant-sim-funded-program", accessed: "2026-10-23" }
  - { field: consistency_rule, url: "https://help.toponefutures.com/en/articles/10908767-consistency-rule-in-instant-sim-funded-accounts-explained", accessed: "2026-10-23" }
  - { field: max_position, url: "https://help.toponefutures.com/en/articles/10906950-maximum-contracts-explained", accessed: "2026-10-23" }
  - { field: min_trading_days, url: "https://help.toponefutures.com/en/articles/11020805-overview-of-the-instant-sim-funded-program", accessed: "2026-10-23" }
  - { field: trading_day_reset, url: "https://help.toponefutures.com/en/articles/10908119-trailing-drawdown-on-instant-sim-funded-accounts-how-it-works", accessed: "2026-10-23" }
  - { field: automated_trading_allowed, url: "https://help.toponefutures.com/en/articles/11021584-prohibited-trading-practices", accessed: "2026-10-23" }
  - { field: copy_trading_allowed, url: "https://help.toponefutures.com/en/articles/11680891-copy-trading-policy", accessed: "2026-10-23" }
  - { field: vps_allowed, url: "https://help.toponefutures.com/en/articles/11187841-are-there-ip-address-restrictions", accessed: "2026-10-23" }
  - { field: personal_device_required, url: "https://help.toponefutures.com/en/articles/11187841-are-there-ip-address-restrictions", accessed: "2026-10-23" }
  - { field: platforms, url: "https://help.toponefutures.com/en/articles/17149341-can-i-switch-my-trading-platform-tradovate-ninjatrader-or-blackarrow", accessed: "2026-10-23" }
```

```yaml
firm: top-one-futures
plan: "Instant Sim Funded"
phase: funded
size_label: "$100K"
size_usd: "100000.00"
instruments: [MNQ, NQ]
platforms: ["Tradovate/NinjaTrader", BlackArrow, WealthCharts]   # firm-wide list; per-plan availability not published (see §2, §9)
drawdown: { amount_usd: "4000.00", type: trailing_eod, stops_trailing_at: "locks at starting balance + $100 ($100,100) once balance reaches $104,100; also on first payout" }
daily_loss_limit: { amount_usd: "2500.00", kind: soft_pause }   # soft breach (IS1, G2)
profit_target_usd: null   # none to keep account; payout target 6% / 5% / 4% per cycle (§3.4)
consistency_rule: { text: "no single day's profit may exceed 20% of your total profits", threshold_pct: "20" }
max_position: { minis: 5, micros: 50, scaling: "none; fixed limit from day one (IS1)" }
min_trading_days: "none"   # "Minimum trading days: None" (Q1 comparison table)
trading_day_reset: { time: "17:00", tz_as_published: "EST" }   # EOD / consistency day "ends at 5:00 pm EST" (IS3, IS4); sessions 6:00 PM-4:10 PM EST (G12)
copy_trading_allowed: conditional   # own accounts only, same program AND same size (G15)
automated_trading_allowed: "no"
automation_rule_text: "Use of any automated trading system, script, bot, Expert Advisor (EA), or high-frequency execution tool is strictly forbidden. All trades must be entered manually or semi-manually by the trader."
vps_allowed: conditional          # VPS with static IP, notify support in advance with IP + invoice (G13); VPN prohibited
personal_device_required: "no"   # explicitly permitted: other devices and a static-IP VPS (G13); account owner must be the sole trader (G16)
status: partial
not_confirmed_fields: [copy_trading_other_firms, copy_trading_same_ip]
sources:
  - { field: drawdown, url: "https://help.toponefutures.com/en/articles/10908119-trailing-drawdown-on-instant-sim-funded-accounts-how-it-works", accessed: "2026-10-23" }
  - { field: daily_loss_limit, url: "https://help.toponefutures.com/en/articles/11020805-overview-of-the-instant-sim-funded-program", accessed: "2026-10-23" }
  - { field: consistency_rule, url: "https://help.toponefutures.com/en/articles/10908767-consistency-rule-in-instant-sim-funded-accounts-explained", accessed: "2026-10-23" }
  - { field: max_position, url: "https://help.toponefutures.com/en/articles/10906950-maximum-contracts-explained", accessed: "2026-10-23" }
  - { field: min_trading_days, url: "https://help.toponefutures.com/en/articles/11020805-overview-of-the-instant-sim-funded-program", accessed: "2026-10-23" }
  - { field: trading_day_reset, url: "https://help.toponefutures.com/en/articles/10908119-trailing-drawdown-on-instant-sim-funded-accounts-how-it-works", accessed: "2026-10-23" }
  - { field: automated_trading_allowed, url: "https://help.toponefutures.com/en/articles/11021584-prohibited-trading-practices", accessed: "2026-10-23" }
  - { field: copy_trading_allowed, url: "https://help.toponefutures.com/en/articles/11680891-copy-trading-policy", accessed: "2026-10-23" }
  - { field: vps_allowed, url: "https://help.toponefutures.com/en/articles/11187841-are-there-ip-address-restrictions", accessed: "2026-10-23" }
  - { field: personal_device_required, url: "https://help.toponefutures.com/en/articles/11187841-are-there-ip-address-restrictions", accessed: "2026-10-23" }
  - { field: platforms, url: "https://help.toponefutures.com/en/articles/17149341-can-i-switch-my-trading-platform-tradovate-ninjatrader-or-blackarrow", accessed: "2026-10-23" }
```

```yaml
firm: top-one-futures
plan: "Instant Sim Funded"
phase: funded
size_label: "$150K"
size_usd: "150000.00"
instruments: [MNQ, NQ]
platforms: ["Tradovate/NinjaTrader", BlackArrow, WealthCharts]   # firm-wide list; per-plan availability not published (see §2, §9)
drawdown: { amount_usd: "5250.00", type: trailing_eod, stops_trailing_at: "locks at starting balance + $100 ($150,100) once balance reaches $155,350; also on first payout" }
daily_loss_limit: { amount_usd: "3000.00", kind: soft_pause }   # soft breach (IS1, G2)
profit_target_usd: null   # none to keep account; payout target 6% / 5% / 4% per cycle (§3.4)
consistency_rule: { text: "no single day's profit may exceed 20% of your total profits", threshold_pct: "20" }
max_position: { minis: 7, micros: 70, scaling: "none; fixed limit from day one (IS1)" }
min_trading_days: "none"   # "Minimum trading days: None" (Q1 comparison table)
trading_day_reset: { time: "17:00", tz_as_published: "EST" }   # EOD / consistency day "ends at 5:00 pm EST" (IS3, IS4); sessions 6:00 PM-4:10 PM EST (G12)
copy_trading_allowed: conditional   # own accounts only, same program AND same size (G15)
automated_trading_allowed: "no"
automation_rule_text: "Use of any automated trading system, script, bot, Expert Advisor (EA), or high-frequency execution tool is strictly forbidden. All trades must be entered manually or semi-manually by the trader."
vps_allowed: conditional          # VPS with static IP, notify support in advance with IP + invoice (G13); VPN prohibited
personal_device_required: "no"   # explicitly permitted: other devices and a static-IP VPS (G13); account owner must be the sole trader (G16)
status: partial
not_confirmed_fields: [copy_trading_other_firms, copy_trading_same_ip]
sources:
  - { field: drawdown, url: "https://help.toponefutures.com/en/articles/10908119-trailing-drawdown-on-instant-sim-funded-accounts-how-it-works", accessed: "2026-10-23" }
  - { field: daily_loss_limit, url: "https://help.toponefutures.com/en/articles/11020805-overview-of-the-instant-sim-funded-program", accessed: "2026-10-23" }
  - { field: consistency_rule, url: "https://help.toponefutures.com/en/articles/10908767-consistency-rule-in-instant-sim-funded-accounts-explained", accessed: "2026-10-23" }
  - { field: max_position, url: "https://help.toponefutures.com/en/articles/10906950-maximum-contracts-explained", accessed: "2026-10-23" }
  - { field: min_trading_days, url: "https://help.toponefutures.com/en/articles/11020805-overview-of-the-instant-sim-funded-program", accessed: "2026-10-23" }
  - { field: trading_day_reset, url: "https://help.toponefutures.com/en/articles/10908119-trailing-drawdown-on-instant-sim-funded-accounts-how-it-works", accessed: "2026-10-23" }
  - { field: automated_trading_allowed, url: "https://help.toponefutures.com/en/articles/11021584-prohibited-trading-practices", accessed: "2026-10-23" }
  - { field: copy_trading_allowed, url: "https://help.toponefutures.com/en/articles/11680891-copy-trading-policy", accessed: "2026-10-23" }
  - { field: vps_allowed, url: "https://help.toponefutures.com/en/articles/11187841-are-there-ip-address-restrictions", accessed: "2026-10-23" }
  - { field: personal_device_required, url: "https://help.toponefutures.com/en/articles/11187841-are-there-ip-address-restrictions", accessed: "2026-10-23" }
  - { field: platforms, url: "https://help.toponefutures.com/en/articles/17149341-can-i-switch-my-trading-platform-tradovate-ninjatrader-or-blackarrow", accessed: "2026-10-23" }
```

### 8.5 Ignite Instant Funding (no evaluation)

```yaml
firm: top-one-futures
plan: "Ignite"
phase: funded
size_label: "$25K"
size_usd: "25000.00"
instruments: [MNQ, NQ]
platforms: ["Tradovate/NinjaTrader", BlackArrow, WealthCharts]   # firm-wide list; per-plan availability not published (see §2, §9)
drawdown: { amount_usd: "1000.00", type: trailing_eod, stops_trailing_at: "locks at $100 above starting balance ($25,100) once balance reaches $26,100; also on first payout" }
daily_loss_limit: { amount_usd: "500.00", kind: soft_pause }   # paused rest of trading day; No-DLL add-on at checkout
profit_target_usd: null   # none to keep account; 5% per payout cycle (§3.5)
consistency_rule: { text: "Your single best trading day cannot account for more than 15% of your total accumulated profit at the time of your payout request.", threshold_pct: "15" }
max_position: { minis: 1, micros: 10, scaling: "none; fixed limits (G1)" }
min_trading_days: "none"   # "Minimum trading days: None" (IG2)
trading_day_reset: { time: "18:00", tz_as_published: "ET" }    # DLL pause "resumes when markets reopen at 6:00 pm ET" (IG1)
copy_trading_allowed: conditional   # own accounts only, same program AND same size (G15)
automated_trading_allowed: "no"
automation_rule_text: "Use of any automated trading system, script, bot, Expert Advisor (EA), or high-frequency execution tool is strictly forbidden. All trades must be entered manually or semi-manually by the trader."
vps_allowed: conditional          # VPS with static IP, notify support in advance with IP + invoice (G13); VPN prohibited
personal_device_required: "no"   # explicitly permitted: other devices and a static-IP VPS (G13); account owner must be the sole trader (G16)
status: partial
not_confirmed_fields: [max_accounts, flatten_time, copy_trading_other_firms, copy_trading_same_ip]
sources:
  - { field: drawdown, url: "https://help.toponefutures.com/en/articles/12671168-ignite-accounts-overview", accessed: "2026-10-23" }
  - { field: daily_loss_limit, url: "https://help.toponefutures.com/en/articles/12671168-ignite-accounts-overview", accessed: "2026-10-23" }
  - { field: consistency_rule, url: "https://help.toponefutures.com/en/articles/12707415-ignite-accounts-payout-requirements", accessed: "2026-10-23" }
  - { field: max_position, url: "https://help.toponefutures.com/en/articles/12671168-ignite-accounts-overview", accessed: "2026-10-23" }
  - { field: min_trading_days, url: "https://help.toponefutures.com/en/articles/12707415-ignite-accounts-payout-requirements", accessed: "2026-10-23" }
  - { field: trading_day_reset, url: "https://help.toponefutures.com/en/articles/12671168-ignite-accounts-overview", accessed: "2026-10-23" }
  - { field: automated_trading_allowed, url: "https://help.toponefutures.com/en/articles/11021584-prohibited-trading-practices", accessed: "2026-10-23" }
  - { field: copy_trading_allowed, url: "https://help.toponefutures.com/en/articles/11680891-copy-trading-policy", accessed: "2026-10-23" }
  - { field: vps_allowed, url: "https://help.toponefutures.com/en/articles/11187841-are-there-ip-address-restrictions", accessed: "2026-10-23" }
  - { field: personal_device_required, url: "https://help.toponefutures.com/en/articles/11187841-are-there-ip-address-restrictions", accessed: "2026-10-23" }
  - { field: platforms, url: "https://help.toponefutures.com/en/articles/17149341-can-i-switch-my-trading-platform-tradovate-ninjatrader-or-blackarrow", accessed: "2026-10-23" }
```

```yaml
firm: top-one-futures
plan: "Ignite"
phase: funded
size_label: "$50K"
size_usd: "50000.00"
instruments: [MNQ, NQ]
platforms: ["Tradovate/NinjaTrader", BlackArrow, WealthCharts]   # firm-wide list; per-plan availability not published (see §2, §9)
drawdown: { amount_usd: "2000.00", type: trailing_eod, stops_trailing_at: "locks at $100 above starting balance ($50,100) once balance reaches $52,100; also on first payout" }
daily_loss_limit: { amount_usd: "1000.00", kind: soft_pause }   # paused rest of trading day; No-DLL add-on at checkout
profit_target_usd: null   # none to keep account; 5% per payout cycle (§3.5)
consistency_rule: { text: "Your single best trading day cannot account for more than 15% of your total accumulated profit at the time of your payout request.", threshold_pct: "15" }
max_position: { minis: 3, micros: 30, scaling: "none; fixed limits (G1)" }
min_trading_days: "none"   # "Minimum trading days: None" (IG2)
trading_day_reset: { time: "18:00", tz_as_published: "ET" }    # DLL pause "resumes when markets reopen at 6:00 pm ET" (IG1)
copy_trading_allowed: conditional   # own accounts only, same program AND same size (G15)
automated_trading_allowed: "no"
automation_rule_text: "Use of any automated trading system, script, bot, Expert Advisor (EA), or high-frequency execution tool is strictly forbidden. All trades must be entered manually or semi-manually by the trader."
vps_allowed: conditional          # VPS with static IP, notify support in advance with IP + invoice (G13); VPN prohibited
personal_device_required: "no"   # explicitly permitted: other devices and a static-IP VPS (G13); account owner must be the sole trader (G16)
status: partial
not_confirmed_fields: [max_accounts, flatten_time, copy_trading_other_firms, copy_trading_same_ip]
sources:
  - { field: drawdown, url: "https://help.toponefutures.com/en/articles/12671168-ignite-accounts-overview", accessed: "2026-10-23" }
  - { field: daily_loss_limit, url: "https://help.toponefutures.com/en/articles/12671168-ignite-accounts-overview", accessed: "2026-10-23" }
  - { field: consistency_rule, url: "https://help.toponefutures.com/en/articles/12707415-ignite-accounts-payout-requirements", accessed: "2026-10-23" }
  - { field: max_position, url: "https://help.toponefutures.com/en/articles/12671168-ignite-accounts-overview", accessed: "2026-10-23" }
  - { field: min_trading_days, url: "https://help.toponefutures.com/en/articles/12707415-ignite-accounts-payout-requirements", accessed: "2026-10-23" }
  - { field: trading_day_reset, url: "https://help.toponefutures.com/en/articles/12671168-ignite-accounts-overview", accessed: "2026-10-23" }
  - { field: automated_trading_allowed, url: "https://help.toponefutures.com/en/articles/11021584-prohibited-trading-practices", accessed: "2026-10-23" }
  - { field: copy_trading_allowed, url: "https://help.toponefutures.com/en/articles/11680891-copy-trading-policy", accessed: "2026-10-23" }
  - { field: vps_allowed, url: "https://help.toponefutures.com/en/articles/11187841-are-there-ip-address-restrictions", accessed: "2026-10-23" }
  - { field: personal_device_required, url: "https://help.toponefutures.com/en/articles/11187841-are-there-ip-address-restrictions", accessed: "2026-10-23" }
  - { field: platforms, url: "https://help.toponefutures.com/en/articles/17149341-can-i-switch-my-trading-platform-tradovate-ninjatrader-or-blackarrow", accessed: "2026-10-23" }
```

```yaml
firm: top-one-futures
plan: "Ignite"
phase: funded
size_label: "$100K"
size_usd: "100000.00"
instruments: [MNQ, NQ]
platforms: ["Tradovate/NinjaTrader", BlackArrow, WealthCharts]   # firm-wide list; per-plan availability not published (see §2, §9)
drawdown: { amount_usd: "4000.00", type: trailing_eod, stops_trailing_at: "locks at $100 above starting balance ($100,100) once balance reaches $104,100; also on first payout" }
daily_loss_limit: { amount_usd: "2000.00", kind: soft_pause }   # paused rest of trading day; No-DLL add-on at checkout
profit_target_usd: null   # none to keep account; 5% per payout cycle (§3.5)
consistency_rule: { text: "Your single best trading day cannot account for more than 15% of your total accumulated profit at the time of your payout request.", threshold_pct: "15" }
max_position: { minis: 5, micros: 50, scaling: "none; fixed limits (G1)" }
min_trading_days: "none"   # "Minimum trading days: None" (IG2)
trading_day_reset: { time: "18:00", tz_as_published: "ET" }    # DLL pause "resumes when markets reopen at 6:00 pm ET" (IG1)
copy_trading_allowed: conditional   # own accounts only, same program AND same size (G15)
automated_trading_allowed: "no"
automation_rule_text: "Use of any automated trading system, script, bot, Expert Advisor (EA), or high-frequency execution tool is strictly forbidden. All trades must be entered manually or semi-manually by the trader."
vps_allowed: conditional          # VPS with static IP, notify support in advance with IP + invoice (G13); VPN prohibited
personal_device_required: "no"   # explicitly permitted: other devices and a static-IP VPS (G13); account owner must be the sole trader (G16)
status: partial
not_confirmed_fields: [max_accounts, flatten_time, copy_trading_other_firms, copy_trading_same_ip]
sources:
  - { field: drawdown, url: "https://help.toponefutures.com/en/articles/12671168-ignite-accounts-overview", accessed: "2026-10-23" }
  - { field: daily_loss_limit, url: "https://help.toponefutures.com/en/articles/12671168-ignite-accounts-overview", accessed: "2026-10-23" }
  - { field: consistency_rule, url: "https://help.toponefutures.com/en/articles/12707415-ignite-accounts-payout-requirements", accessed: "2026-10-23" }
  - { field: max_position, url: "https://help.toponefutures.com/en/articles/12671168-ignite-accounts-overview", accessed: "2026-10-23" }
  - { field: min_trading_days, url: "https://help.toponefutures.com/en/articles/12707415-ignite-accounts-payout-requirements", accessed: "2026-10-23" }
  - { field: trading_day_reset, url: "https://help.toponefutures.com/en/articles/12671168-ignite-accounts-overview", accessed: "2026-10-23" }
  - { field: automated_trading_allowed, url: "https://help.toponefutures.com/en/articles/11021584-prohibited-trading-practices", accessed: "2026-10-23" }
  - { field: copy_trading_allowed, url: "https://help.toponefutures.com/en/articles/11680891-copy-trading-policy", accessed: "2026-10-23" }
  - { field: vps_allowed, url: "https://help.toponefutures.com/en/articles/11187841-are-there-ip-address-restrictions", accessed: "2026-10-23" }
  - { field: personal_device_required, url: "https://help.toponefutures.com/en/articles/11187841-are-there-ip-address-restrictions", accessed: "2026-10-23" }
  - { field: platforms, url: "https://help.toponefutures.com/en/articles/17149341-can-i-switch-my-trading-platform-tradovate-ninjatrader-or-blackarrow", accessed: "2026-10-23" }
```

```yaml
firm: top-one-futures
plan: "Ignite"
phase: funded
size_label: "$150K"
size_usd: "150000.00"
instruments: [MNQ, NQ]
platforms: ["Tradovate/NinjaTrader", BlackArrow, WealthCharts]   # firm-wide list; per-plan availability not published (see §2, §9)
drawdown: { amount_usd: "6000.00", type: trailing_eod, stops_trailing_at: "locks at $100 above starting balance ($150,100) once balance reaches $156,100; also on first payout" }
daily_loss_limit: { amount_usd: "3000.00", kind: soft_pause }   # paused rest of trading day; No-DLL add-on at checkout
profit_target_usd: null   # none to keep account; 5% per payout cycle (§3.5)
consistency_rule: { text: "Your single best trading day cannot account for more than 15% of your total accumulated profit at the time of your payout request.", threshold_pct: "15" }
max_position: { minis: 7, micros: 70, scaling: "none; fixed limits (G1)" }
min_trading_days: "none"   # "Minimum trading days: None" (IG2)
trading_day_reset: { time: "18:00", tz_as_published: "ET" }    # DLL pause "resumes when markets reopen at 6:00 pm ET" (IG1)
copy_trading_allowed: conditional   # own accounts only, same program AND same size (G15)
automated_trading_allowed: "no"
automation_rule_text: "Use of any automated trading system, script, bot, Expert Advisor (EA), or high-frequency execution tool is strictly forbidden. All trades must be entered manually or semi-manually by the trader."
vps_allowed: conditional          # VPS with static IP, notify support in advance with IP + invoice (G13); VPN prohibited
personal_device_required: "no"   # explicitly permitted: other devices and a static-IP VPS (G13); account owner must be the sole trader (G16)
status: partial
not_confirmed_fields: [max_accounts, flatten_time, copy_trading_other_firms, copy_trading_same_ip]
sources:
  - { field: drawdown, url: "https://help.toponefutures.com/en/articles/12671168-ignite-accounts-overview", accessed: "2026-10-23" }
  - { field: daily_loss_limit, url: "https://help.toponefutures.com/en/articles/12671168-ignite-accounts-overview", accessed: "2026-10-23" }
  - { field: consistency_rule, url: "https://help.toponefutures.com/en/articles/12707415-ignite-accounts-payout-requirements", accessed: "2026-10-23" }
  - { field: max_position, url: "https://help.toponefutures.com/en/articles/12671168-ignite-accounts-overview", accessed: "2026-10-23" }
  - { field: min_trading_days, url: "https://help.toponefutures.com/en/articles/12707415-ignite-accounts-payout-requirements", accessed: "2026-10-23" }
  - { field: trading_day_reset, url: "https://help.toponefutures.com/en/articles/12671168-ignite-accounts-overview", accessed: "2026-10-23" }
  - { field: automated_trading_allowed, url: "https://help.toponefutures.com/en/articles/11021584-prohibited-trading-practices", accessed: "2026-10-23" }
  - { field: copy_trading_allowed, url: "https://help.toponefutures.com/en/articles/11680891-copy-trading-policy", accessed: "2026-10-23" }
  - { field: vps_allowed, url: "https://help.toponefutures.com/en/articles/11187841-are-there-ip-address-restrictions", accessed: "2026-10-23" }
  - { field: personal_device_required, url: "https://help.toponefutures.com/en/articles/11187841-are-there-ip-address-restrictions", accessed: "2026-10-23" }
  - { field: platforms, url: "https://help.toponefutures.com/en/articles/17149341-can-i-switch-my-trading-platform-tradovate-ninjatrader-or-blackarrow", accessed: "2026-10-23" }
```

### 8.6 LaunchPad X-Daily Spark (limited release, $50K only)

```yaml
firm: top-one-futures
plan: "X-Daily Spark"
phase: evaluation
size_label: "$50K"
size_usd: "50000.00"
instruments: [MNQ, NQ]
platforms: ["Tradovate/NinjaTrader", BlackArrow, WealthCharts]   # firm-wide list; per-plan availability not published (see §2, §9)
drawdown: { amount_usd: "2000.00", type: trailing_eod, stops_trailing_at: "not published for the evaluation (static at $48,000 with the +$15 add-on)" }
daily_loss_limit: { amount_usd: null, kind: none }   # no DLL in evaluation (XS3)
profit_target_usd: "3000.00"   # $2,500 (+$15) or $2,000 (+$30) with add-ons (XS2)
consistency_rule: { text: null, threshold_pct: null }
max_position: { minis: 3, micros: 30, scaling: "none in evaluation (XS7)" }
min_trading_days: "none"   # "Min trading days to pass: None"; 30-day limit (XS2)
trading_day_reset: { time: "18:00", tz_as_published: "ET" }    # "A trading day runs from 6:00 PM ET to 4:10 PM ET" (XS8)
copy_trading_allowed: conditional   # own accounts only, same program AND same size (G15)
automated_trading_allowed: "no"
automation_rule_text: "Use of any automated trading system, script, bot, Expert Advisor (EA), or high-frequency execution tool is strictly forbidden. All trades must be entered manually or semi-manually by the trader."
vps_allowed: conditional          # VPS with static IP, notify support in advance with IP + invoice (G13); VPN prohibited
personal_device_required: "no"   # explicitly permitted: other devices and a static-IP VPS (G13); account owner must be the sole trader (G16)
status: partial
not_confirmed_fields: [copy_trading_other_firms, copy_trading_same_ip]
sources:
  - { field: drawdown, url: "https://help.toponefutures.com/en/articles/17678752-x-daily-spark-maximum-drawdown-mll-eod", accessed: "2026-10-23" }
  - { field: daily_loss_limit, url: "https://help.toponefutures.com/en/articles/17678749-x-daily-spark-daily-loss-limit-dll", accessed: "2026-10-23" }
  - { field: profit_target_usd, url: "https://help.toponefutures.com/en/articles/17678737-x-daily-spark-evaluation-profit-target-rules", accessed: "2026-10-23" }
  - { field: consistency_rule, url: "https://help.toponefutures.com/en/articles/17678737-x-daily-spark-evaluation-profit-target-rules", accessed: "2026-10-23" }
  - { field: max_position, url: "https://help.toponefutures.com/en/articles/17678769-x-daily-spark-contract-limits-scaling", accessed: "2026-10-23" }
  - { field: min_trading_days, url: "https://help.toponefutures.com/en/articles/17678737-x-daily-spark-evaluation-profit-target-rules", accessed: "2026-10-23" }
  - { field: trading_day_reset, url: "https://help.toponefutures.com/en/articles/17678771-x-daily-spark-other-trading-rules", accessed: "2026-10-23" }
  - { field: automated_trading_allowed, url: "https://help.toponefutures.com/en/articles/11021584-prohibited-trading-practices", accessed: "2026-10-23" }
  - { field: copy_trading_allowed, url: "https://help.toponefutures.com/en/articles/11680891-copy-trading-policy", accessed: "2026-10-23" }
  - { field: vps_allowed, url: "https://help.toponefutures.com/en/articles/11187841-are-there-ip-address-restrictions", accessed: "2026-10-23" }
  - { field: personal_device_required, url: "https://help.toponefutures.com/en/articles/11187841-are-there-ip-address-restrictions", accessed: "2026-10-23" }
  - { field: platforms, url: "https://help.toponefutures.com/en/articles/17149341-can-i-switch-my-trading-platform-tradovate-ninjatrader-or-blackarrow", accessed: "2026-10-23" }
```

```yaml
firm: top-one-futures
plan: "X-Daily Spark"
phase: funded
size_label: "$50K"
size_usd: "50000.00"
instruments: [MNQ, NQ]
platforms: ["Tradovate/NinjaTrader", BlackArrow, WealthCharts]   # firm-wide list; per-plan availability not published (see §2, §9)
drawdown: { amount_usd: "2000.00", type: trailing_eod, stops_trailing_at: "locks at $50,100 when you close the day at $52,100 or higher" }
daily_loss_limit: { amount_usd: "1000.00", kind: soft_pause }   # paused rest of trading day; removable for +$30 at activation (XS3)
profit_target_usd: null   # none; payout buffer $3,000 (§3.6)
consistency_rule: { text: null, threshold_pct: null }
max_position: { minis: 3, micros: 30, scaling: "$0-$1,499: 1 mini (10 micros); $1,500-$1,999: 2 (20); $2,000+: 3 (30), full; EOD balance, applies next trading day" }
min_trading_days: "none"   # "Min trading days before a payout: None" (XS1)
trading_day_reset: { time: "18:00", tz_as_published: "ET" }    # "A trading day runs from 6:00 PM ET to 4:10 PM ET" (XS8)
copy_trading_allowed: conditional   # own accounts only, same program AND same size (G15)
automated_trading_allowed: "no"
automation_rule_text: "Use of any automated trading system, script, bot, Expert Advisor (EA), or high-frequency execution tool is strictly forbidden. All trades must be entered manually or semi-manually by the trader."
vps_allowed: conditional          # VPS with static IP, notify support in advance with IP + invoice (G13); VPN prohibited
personal_device_required: "no"   # explicitly permitted: other devices and a static-IP VPS (G13); account owner must be the sole trader (G16)
status: partial
not_confirmed_fields: [copy_trading_other_firms, copy_trading_same_ip, news_rule]
sources:
  - { field: drawdown, url: "https://help.toponefutures.com/en/articles/17678752-x-daily-spark-maximum-drawdown-mll-eod", accessed: "2026-10-23" }
  - { field: daily_loss_limit, url: "https://help.toponefutures.com/en/articles/17678749-x-daily-spark-daily-loss-limit-dll", accessed: "2026-10-23" }
  - { field: consistency_rule, url: "https://help.toponefutures.com/en/articles/17678693-x-daily-spark-accounts-overview", accessed: "2026-10-23" }
  - { field: max_position, url: "https://help.toponefutures.com/en/articles/17678769-x-daily-spark-contract-limits-scaling", accessed: "2026-10-23" }
  - { field: min_trading_days, url: "https://help.toponefutures.com/en/articles/17678693-x-daily-spark-accounts-overview", accessed: "2026-10-23" }
  - { field: trading_day_reset, url: "https://help.toponefutures.com/en/articles/17678771-x-daily-spark-other-trading-rules", accessed: "2026-10-23" }
  - { field: automated_trading_allowed, url: "https://help.toponefutures.com/en/articles/11021584-prohibited-trading-practices", accessed: "2026-10-23" }
  - { field: copy_trading_allowed, url: "https://help.toponefutures.com/en/articles/11680891-copy-trading-policy", accessed: "2026-10-23" }
  - { field: vps_allowed, url: "https://help.toponefutures.com/en/articles/11187841-are-there-ip-address-restrictions", accessed: "2026-10-23" }
  - { field: personal_device_required, url: "https://help.toponefutures.com/en/articles/11187841-are-there-ip-address-restrictions", accessed: "2026-10-23" }
  - { field: platforms, url: "https://help.toponefutures.com/en/articles/17149341-can-i-switch-my-trading-platform-tradovate-ninjatrader-or-blackarrow", accessed: "2026-10-23" }
```

### 8.7 Pre-Live / Live (after payouts on a sim funded account; all plans)

```yaml
# Live accounts start at a $0 balance; size_usd is the nominal size label. Platform for live not published.
firm: top-one-futures
plan: "Pre-Live / Live"
phase: live
size_label: "$25K"
size_usd: "25000.00"
instruments: [MNQ, NQ]
platforms: null   # not published for Pre-Live/Live (L1); see §9 G1
drawdown: { amount_usd: "1000.00", type: trailing_eod, stops_trailing_at: "drawdown lock $100 once EOD balance reaches $1,100 (starting balance $0)" }
daily_loss_limit: { amount_usd: null, kind: none }   # "There is no DLL on Pre-Live"; Live "Daily Loss Limit: None" (L1)
profit_target_usd: null   # n/a
consistency_rule: { text: null, threshold_pct: null }
max_position: { minis: 1, micros: 10, scaling: "$0-$1,499: 1/10 (max 1)" }
min_trading_days: null   # not published
trading_day_reset: { time: "18:00", tz_as_published: "EST" }   # sessions "6:00 PM EST to 4:10 PM EST"; forced flat 4:10 PM EST (G12)
copy_trading_allowed: conditional   # own accounts only, same program AND same size (G15)
automated_trading_allowed: "no"
automation_rule_text: "Use of any automated trading system, script, bot, Expert Advisor (EA), or high-frequency execution tool is strictly forbidden. All trades must be entered manually or semi-manually by the trader."
vps_allowed: conditional          # VPS with static IP, notify support in advance with IP + invoice (G13); VPN prohibited
personal_device_required: "no"   # explicitly permitted: other devices and a static-IP VPS (G13); account owner must be the sole trader (G16)
status: partial
not_confirmed_fields: [copy_trading_other_firms, copy_trading_same_ip]
sources:
  - { field: drawdown, url: "https://help.toponefutures.com/en/articles/11003130-path-to-trading-live", accessed: "2026-10-23" }
  - { field: daily_loss_limit, url: "https://help.toponefutures.com/en/articles/11003130-path-to-trading-live", accessed: "2026-10-23" }
  - { field: consistency_rule, url: "https://help.toponefutures.com/en/articles/11003130-path-to-trading-live", accessed: "2026-10-23" }
  - { field: max_position, url: "https://help.toponefutures.com/en/articles/11003130-path-to-trading-live", accessed: "2026-10-23" }
  - { field: trading_day_reset, url: "https://help.toponefutures.com/en/articles/11165893-trading-hours-at-top-one-futures", accessed: "2026-10-23" }
  - { field: automated_trading_allowed, url: "https://help.toponefutures.com/en/articles/11021584-prohibited-trading-practices", accessed: "2026-10-23" }
  - { field: copy_trading_allowed, url: "https://help.toponefutures.com/en/articles/11680891-copy-trading-policy", accessed: "2026-10-23" }
  - { field: vps_allowed, url: "https://help.toponefutures.com/en/articles/11187841-are-there-ip-address-restrictions", accessed: "2026-10-23" }
  - { field: personal_device_required, url: "https://help.toponefutures.com/en/articles/11187841-are-there-ip-address-restrictions", accessed: "2026-10-23" }
```

```yaml
# Live accounts start at a $0 balance; size_usd is the nominal size label. Platform for live not published.
firm: top-one-futures
plan: "Pre-Live / Live"
phase: live
size_label: "$50K"
size_usd: "50000.00"
instruments: [MNQ, NQ]
platforms: null   # not published for Pre-Live/Live (L1); see §9 G1
drawdown: { amount_usd: "2000.00", type: trailing_eod, stops_trailing_at: "drawdown lock $100 once EOD balance reaches $2,100 (starting balance $0)" }
daily_loss_limit: { amount_usd: null, kind: none }   # "There is no DLL on Pre-Live"; Live "Daily Loss Limit: None" (L1)
profit_target_usd: null   # n/a
consistency_rule: { text: null, threshold_pct: null }
max_position: { minis: 3, micros: 30, scaling: "$0-$1,499: 1/10; $1,500-$1,999: 2/20; $2,000-$2,999: 3/30" }
min_trading_days: null   # not published
trading_day_reset: { time: "18:00", tz_as_published: "EST" }   # sessions "6:00 PM EST to 4:10 PM EST"; forced flat 4:10 PM EST (G12)
copy_trading_allowed: conditional   # own accounts only, same program AND same size (G15)
automated_trading_allowed: "no"
automation_rule_text: "Use of any automated trading system, script, bot, Expert Advisor (EA), or high-frequency execution tool is strictly forbidden. All trades must be entered manually or semi-manually by the trader."
vps_allowed: conditional          # VPS with static IP, notify support in advance with IP + invoice (G13); VPN prohibited
personal_device_required: "no"   # explicitly permitted: other devices and a static-IP VPS (G13); account owner must be the sole trader (G16)
status: partial
not_confirmed_fields: [copy_trading_other_firms, copy_trading_same_ip]
sources:
  - { field: drawdown, url: "https://help.toponefutures.com/en/articles/11003130-path-to-trading-live", accessed: "2026-10-23" }
  - { field: daily_loss_limit, url: "https://help.toponefutures.com/en/articles/11003130-path-to-trading-live", accessed: "2026-10-23" }
  - { field: consistency_rule, url: "https://help.toponefutures.com/en/articles/11003130-path-to-trading-live", accessed: "2026-10-23" }
  - { field: max_position, url: "https://help.toponefutures.com/en/articles/11003130-path-to-trading-live", accessed: "2026-10-23" }
  - { field: trading_day_reset, url: "https://help.toponefutures.com/en/articles/11165893-trading-hours-at-top-one-futures", accessed: "2026-10-23" }
  - { field: automated_trading_allowed, url: "https://help.toponefutures.com/en/articles/11021584-prohibited-trading-practices", accessed: "2026-10-23" }
  - { field: copy_trading_allowed, url: "https://help.toponefutures.com/en/articles/11680891-copy-trading-policy", accessed: "2026-10-23" }
  - { field: vps_allowed, url: "https://help.toponefutures.com/en/articles/11187841-are-there-ip-address-restrictions", accessed: "2026-10-23" }
  - { field: personal_device_required, url: "https://help.toponefutures.com/en/articles/11187841-are-there-ip-address-restrictions", accessed: "2026-10-23" }
```

```yaml
# Live accounts start at a $0 balance; size_usd is the nominal size label. Platform for live not published.
firm: top-one-futures
plan: "Pre-Live / Live"
phase: live
size_label: "$100K"
size_usd: "100000.00"
instruments: [MNQ, NQ]
platforms: null   # not published for Pre-Live/Live (L1); see §9 G1
drawdown: { amount_usd: "3000.00", type: trailing_eod, stops_trailing_at: "drawdown lock $100 once EOD balance reaches $3,100 (starting balance $0)" }
daily_loss_limit: { amount_usd: null, kind: none }   # "There is no DLL on Pre-Live"; Live "Daily Loss Limit: None" (L1)
profit_target_usd: null   # n/a
consistency_rule: { text: null, threshold_pct: null }
max_position: { minis: 5, micros: 50, scaling: "$0-$1,499: 2/20; $1,500-$1,999: 3/30; $2,000-$2,999: 4/40; $3,000-$4,499: 5/50" }
min_trading_days: null   # not published
trading_day_reset: { time: "18:00", tz_as_published: "EST" }   # sessions "6:00 PM EST to 4:10 PM EST"; forced flat 4:10 PM EST (G12)
copy_trading_allowed: conditional   # own accounts only, same program AND same size (G15)
automated_trading_allowed: "no"
automation_rule_text: "Use of any automated trading system, script, bot, Expert Advisor (EA), or high-frequency execution tool is strictly forbidden. All trades must be entered manually or semi-manually by the trader."
vps_allowed: conditional          # VPS with static IP, notify support in advance with IP + invoice (G13); VPN prohibited
personal_device_required: "no"   # explicitly permitted: other devices and a static-IP VPS (G13); account owner must be the sole trader (G16)
status: partial
not_confirmed_fields: [copy_trading_other_firms, copy_trading_same_ip]
sources:
  - { field: drawdown, url: "https://help.toponefutures.com/en/articles/11003130-path-to-trading-live", accessed: "2026-10-23" }
  - { field: daily_loss_limit, url: "https://help.toponefutures.com/en/articles/11003130-path-to-trading-live", accessed: "2026-10-23" }
  - { field: consistency_rule, url: "https://help.toponefutures.com/en/articles/11003130-path-to-trading-live", accessed: "2026-10-23" }
  - { field: max_position, url: "https://help.toponefutures.com/en/articles/11003130-path-to-trading-live", accessed: "2026-10-23" }
  - { field: trading_day_reset, url: "https://help.toponefutures.com/en/articles/11165893-trading-hours-at-top-one-futures", accessed: "2026-10-23" }
  - { field: automated_trading_allowed, url: "https://help.toponefutures.com/en/articles/11021584-prohibited-trading-practices", accessed: "2026-10-23" }
  - { field: copy_trading_allowed, url: "https://help.toponefutures.com/en/articles/11680891-copy-trading-policy", accessed: "2026-10-23" }
  - { field: vps_allowed, url: "https://help.toponefutures.com/en/articles/11187841-are-there-ip-address-restrictions", accessed: "2026-10-23" }
  - { field: personal_device_required, url: "https://help.toponefutures.com/en/articles/11187841-are-there-ip-address-restrictions", accessed: "2026-10-23" }
```

```yaml
# Live accounts start at a $0 balance; size_usd is the nominal size label. Platform for live not published.
firm: top-one-futures
plan: "Pre-Live / Live"
phase: live
size_label: "$150K"
size_usd: "150000.00"
instruments: [MNQ, NQ]
platforms: null   # not published for Pre-Live/Live (L1); see §9 G1
drawdown: { amount_usd: "4000.00", type: trailing_eod, stops_trailing_at: "drawdown lock $100 once EOD balance reaches $4,100 (starting balance $0)" }
daily_loss_limit: { amount_usd: null, kind: none }   # "There is no DLL on Pre-Live"; Live "Daily Loss Limit: None" (L1)
profit_target_usd: null   # n/a
consistency_rule: { text: null, threshold_pct: null }
max_position: { minis: 7, micros: 70, scaling: "$0-$1,499: 3/30; $1,500-$1,999: 4/40; $2,000-$2,999: 5/50; $3,000-$4,499: 6/60; $4,500+: 7/70" }
min_trading_days: null   # not published
trading_day_reset: { time: "18:00", tz_as_published: "EST" }   # sessions "6:00 PM EST to 4:10 PM EST"; forced flat 4:10 PM EST (G12)
copy_trading_allowed: conditional   # own accounts only, same program AND same size (G15)
automated_trading_allowed: "no"
automation_rule_text: "Use of any automated trading system, script, bot, Expert Advisor (EA), or high-frequency execution tool is strictly forbidden. All trades must be entered manually or semi-manually by the trader."
vps_allowed: conditional          # VPS with static IP, notify support in advance with IP + invoice (G13); VPN prohibited
personal_device_required: "no"   # explicitly permitted: other devices and a static-IP VPS (G13); account owner must be the sole trader (G16)
status: partial
not_confirmed_fields: [copy_trading_other_firms, copy_trading_same_ip]
sources:
  - { field: drawdown, url: "https://help.toponefutures.com/en/articles/11003130-path-to-trading-live", accessed: "2026-10-23" }
  - { field: daily_loss_limit, url: "https://help.toponefutures.com/en/articles/11003130-path-to-trading-live", accessed: "2026-10-23" }
  - { field: consistency_rule, url: "https://help.toponefutures.com/en/articles/11003130-path-to-trading-live", accessed: "2026-10-23" }
  - { field: max_position, url: "https://help.toponefutures.com/en/articles/11003130-path-to-trading-live", accessed: "2026-10-23" }
  - { field: trading_day_reset, url: "https://help.toponefutures.com/en/articles/11165893-trading-hours-at-top-one-futures", accessed: "2026-10-23" }
  - { field: automated_trading_allowed, url: "https://help.toponefutures.com/en/articles/11021584-prohibited-trading-practices", accessed: "2026-10-23" }
  - { field: copy_trading_allowed, url: "https://help.toponefutures.com/en/articles/11680891-copy-trading-policy", accessed: "2026-10-23" }
  - { field: vps_allowed, url: "https://help.toponefutures.com/en/articles/11187841-are-there-ip-address-restrictions", accessed: "2026-10-23" }
  - { field: personal_device_required, url: "https://help.toponefutures.com/en/articles/11187841-are-there-ip-address-restrictions", accessed: "2026-10-23" }
```

## 9. Unverified / gaps / conflicts

### Conflicts (two official pages disagree; ship as "not confirmed")

| ID | Field (YAML name) | Value A | Value B | Blocks affected |
|---|---|---|---|---|
| C1 | `daily_loss_limit`, Elite ACCESS and Elite ACCESS Intraday **funded $150K** | **$1,750**: EA1, EA5, EI1, EI5, G2 table, and the SITE-EA configurator text ("Funded Daily Loss Limit: $25k: $500 $50k: $1,000 $100k: $1,250 $150k: $1,750") | **$1,850**: Elite Access product card, Funded tab, on HOME, SITE-EA and SITE-CP | 8.2 and 8.3 funded $150K |
| C2 | `daily_loss_limit`, Elite Daily **$150K** (evaluation and funded) | **$1,850**: ED1, ED2, ED3, SITE-CP card (both tabs), HOME card funded tab | **$1,750**: HOME card evaluation tab. **"n/a"**: G2 table row "Elite Daily V2 Evaluation / Sim Funded" | 8.1 evaluation and funded $150K |
| C3 | `copy_trading_other_firms` (copying to/from own accounts at other firms) | Allowed if trades originate on Top One Futures: "Top One Futures does not prohibit copy trading between your own accounts at different firms, all trade decisions must originate from activity on our platform" (G15, G11 §13) | Prohibited: "Copy trading, signal-following, or group trading with external accounts." (TC §4, last updated June 10, 2026) | all |
| C4 | `copy_trading_same_ip` | Copying between the same person's same-program, same-size accounts is allowed (G15, IG1, P6, P8) | "Not allowed: … Trading multiple accounts from the same IP"; "Trading from multiple accounts originating from the same or closely related IP addresses is not allowed" (G13) | all |
| C5 | `flatten_time`, Ignite | "All trades must be closed by 4:00 pm ET." (IG1) | "All trades must be closed by 4:10 PM EST daily." (G5). G12 says the rule applies "to all account types" | 8.5 |
| C6 | `max_accounts`, Ignite | 10 (IG1, G4, G14, G11 §15) | 6 (Q1 comparison table). HOME/SITE-CP show "6 \| 10" | 8.5 |
| C7 | `max_accounts`, Instant Sim Funded $25K/$50K | 5 (G4, G14, G11 §15) | 3 (Q1 comparison table, no size split) | 8.4 $25K, $50K |
| C8 | `max_accounts_size_mixing`, Elite Daily | "You may mix account sizes within your 5 account limit", eval + funded combined (ED9) | "Elite Daily Funded: 5. All 5 must be the exact same account size" (G4, G14) | 8.1 |
| C9 | VPN (no YAML field) | "Using a VPN (strictly prohibited)" (G13 summary table) | "The use of Virtual Private Networks (VPNs) is not advised" (G13 body) | text only |
| C10 | `news_rule`, X-Daily Spark funded | 2-minute rule before/after major news (XS1, XS8) | G7: "All other account types (including X-Ultra): No news trading rule". G7 does not list X-Daily Spark | 8.6 funded |

### Gaps (not found or not published)

| ID | Field | What is missing | Where searched |
|---|---|---|---|
| G1 | `platforms` per plan | Which of Tradovate/NinjaTrader, BlackArrow and WealthCharts each plan offers is not published. Only the /elite-access configurator shows one option ("Tradovate/NinjaTrader"). The Live platform/broker is not named | All 131 help-center articles, HOME, SITE-EA, SITE-LP, SITE-CP. Checkout not used (rule) |
| G2 | `personal_device_required` | CLOSED in R4: `no`. The IP page explicitly allows other devices and a static-IP VPS (§7). | G13, G16, G11, G15, T&C |
| G3 | Elite Daily `drawdown.stops_trailing_at` trigger | The lock level is published ($25,100 … $150,100). The balance that triggers it is not ("once your account has grown enough") | ED1, ED4, G3 (G3 tables cover Instant and Elite only) |
| G4 | X-Daily Spark evaluation lock | Only the funded lock ($50,100 at a $52,100 close) is published | XS1, XS4 |
| G5 | Automation details | Webhooks: not mentioned. "Semi-manually" (G11) is not defined. ATM strategies are documented as optional (P4), but no page says whether they fall under the bot ban | G6, G11, TC, P1–P9 |
| G6 | Holiday / early-close schedule | No holiday article. Only "Final session of the week … 4:00 PM EST on Friday" (G12) | All collections |
| G7 | No-DLL add-on scope | Q5 lists Elite ACCESS, Elite Daily and Ignite. G2 and EI5 also offer it for Elite ACCESS Intraday. X-Daily Spark offers it at activation (XS3), while Q5 says checkout only | Q5, G2, EI5, XS3 |
| G8 | Time-zone labels | The firm uses both "EST" and "ET". The day end is 5:00 pm EST for Instant/Elite (EOD and consistency) and 6:00 PM–4:10 PM ET for the LaunchPad. Recorded as published, no conversion | G12, IS1, IS3, IS4, XS8 |
| G9 | Ignite "ESS" rule (older Ignite accounts) | Not defined on any page read | IG3, Q1 |
| G10 | Live `min_trading_days`, Live platform | Not published | L1 |
| G11 | Sale status of Elite and S2F Sim Pro | No homepage card on 2026-10-23, but Q1/Q2 still describe them. Not confirmed whether they can still be bought (checkout not used) | HOME, SITE-CP, Q1, Q2 |
| G12 | Homepage paired values (`<em>` + value) | Treated as old or crossed-out values and not used (§1) | HOME, SITE-CP |
| G13 | "Top One Trader" (`toponetrader.com`) | A separate brand linked from the homepage; not researched (out of scope) | HOME |

## 10. Changelog

- 2026-10-23 — initial research (Lupa). Six current plans (Elite Daily, Elite ACCESS, Elite ACCESS Intraday, Instant Sim Funded 2.0, Ignite, X-Daily Spark) plus Pre-Live/Live; 38 YAML blocks; legacy products summarized in §3.8. Sources: www.toponefutures.com and help.toponefutures.com, read with curl (no portal needed).
- 2026-10-23 — rev bump R4: `personal_device_required` closed as `no` in all 38 blocks (G13 allows other devices and a static-IP VPS; G16 sole-trader rule noted). Nothing else changed (Lupa).
