# Phidias Propfirm — Preset research

| | |
|---|---|
| **Firm** | Phidias Propfirm (operated by "Phidias Propfirm LTD, registered in Gibraltar") |
| **Official domains** | https://phidiaspropfirm.com/ (site, rules, terms) · https://helpcenter.phidiaspropfirm.com/ (official help center; article bodies are served by its help-desk API `app.tickly.help/api/portal/phidias-prop-firm/…`) · https://member.phidiaspropfirm.com/ (member area, login — not accessed) |
| **Research date** | 2026-10-23 (all URLs below accessed 2026-10-23) |
| **Researcher** | Lupa (Prop Firm Rules Researcher) |
| **File status** | `partial` — most fields verified; conflicts and gaps listed in §8 |
| **Instruments covered** | MNQ, NQ (both listed under "Authorized Products — All Accounts": "E-mini NASDAQ 100 (NQ)", "Micro E-Mini Nasdaq-100 (MNQ)") |

**Primary sources**

| Key | URL | Notes |
|---|---|---|
| RULES | https://phidiaspropfirm.com/rules | "All Our Rules" — full rule set, per-plan grids |
| TOU | https://phidiaspropfirm.com/tou | "General Terms of Use Phidias Propfirm" (§3 rules for CASH accounts) |
| ACC | https://phidiaspropfirm.com/accounts | Account configurator; per-size Evaluation / "After You Pass" values come from the page's own script `https://phidiaspropfirm.com/wp-content/cache/min/1/wp-content/custom-scripts/shared-scripts-3.js?ver=1786542922` |
| TRDV | https://phidiaspropfirm.com/tradovate | Tradovate / NinjaTrader / TradingView page |
| HC-x | https://helpcenter.phidiaspropfirm.com/help/<slug> | Help-center articles (slug given at each use). Several FAQ articles still use legacy plan names ("Swing", "Static"); see §8 |

Evidence (raw captured text, URL on first line) is kept outside the repo in the researcher's scratchpad: `rules.txt`, `tou.txt`, `accounts.txt`, `accounts-configurator-data.txt`, `tradovate.txt`, `helpcenter-help.txt`.

Programs found (futures only): **Express to Live (E2L)** 25K/50K/100K/150K; **Fundamental** 50K/100K/150K; **Premium** 50K/100K/150K; **10K Drawdown Challenge** (monthly competition, 10K, outside priority range). Each plan has an evaluation phase and a funded "Phidias-CASH" phase (simulated funded), then a LIVE account (automatic for E2L via "Switch to Live"; discretionary for Fundamental/Premium, incl. ELITE LIVE / MASTER LIVE). RULES (General Rules): "Phidias offers four account categories". No 25K except E2L; no sizes above 150K.

Platforms (relevant to All Win): RULES "Download and install Rtrader Pro, or connect via NinjaTrader / Tradovate". The platform (Rithmic, NinjaTrader/Tradovate, DeepCharts/dxFeed) is chosen per account at purchase and cannot be changed (HC `puis-je-changer-ma-souscription-rithmic-vers-dxfeed-ninjatrader-tradovate-ou-inv`: "The choice between Rithmic, Ninjatrader / tradovate and dxFeed is final for the account concerned.").

---

## 2. Plans

Rules that apply to all sizes of a plan are stated once under the plan; per-size tables follow. "Mini:micro" ratio — HC `comment-fonctionnent-les-positions-maximales-du-compte`: "10 micro-lots are equivalent to 1 mini-lot and 1 mini-lot = 1 “position”, 10 micro-lots = 1 “position”. For example, an account can have a maximum of 10 open positions in mini-lots or 100 positions in micro-lots." Exceeding it: "If you exceed this limit, your valuation will fail or your Phidias CASH account will be closed."

### 2.1 Express to Live (E2L) — evaluation and funded (CASH)

Common rules, RULES §15 "Common Rules — All E2L Sizes" (identical in HC `20-regles-compte-express-to-live-etl-e2l` and TOU §3.2.3):
- "Drawdown Type: Static (never trails)" · "Consistency Rules: None" · "Min. Trading Days: None" · "Daily Loss Limit: None" · "News Trading: Authorized (Eval, CASH, LIVE)" · "Overnight / Over-Week: Not permitted" · "Max E2L Accounts: 5 per trader (all sizes combined)".
- Published formula: "Liquidation formula: Liquidation Threshold = Initial Capital − Static Drawdown. The account is liquidated if the balance reaches or falls below this threshold. The static drawdown never trails."
- Funded phase: "Once the evaluation is passed, your account is converted to a funded CASH account with the same rules. Reach the profit target again on the CASH account." Then "click the "Switch to Live" button. The fixed Bonus Payout is automatically credited to your Phidias Wallet (subject to the 80/20 profit split) … No withdrawal request needed, no minimum trading days, no balance threshold." The ACC configurator data confirms identical funded values (`fundedDrawdown` = `drawdown`, `fundedTarget` = `target`, `fundedMinDays: '0'`).
- TOU §3.2.3: "0 minimum trading days for Evaluation, CASH and LIVE accounts." "Available assets for trading: CME, CBOT, NYMEX and COMEX." "No reset can occur as part of 10K Drawdown or Express to Live (E2L) account evaluations." (TOU §3.1.2)
- Scaling: none published.

| Field | E2L 25K | E2L 50K | E2L 100K | E2L 150K | Source |
|---|---|---|---|---|---|
| Account size | 25K ($25,000) | 50K ($50,000) | 100K ($100,000) | 150K ($150,000) | RULES §15, TOU §3.2.3 |
| Drawdown amount | $500 | $650 | $800 | $1,000 | RULES §15 ("Static Drawdown"), TOU table, ACC |
| Drawdown type | `static` — "never trails" | same | same | same | RULES §15 |
| Liquidation (as published) | $24,500 | $49,350 | $99,200 | $149,000 | RULES §15, TOU table |
| Daily loss limit | none (stated) | none | none | none | RULES §15 |
| Profit target (eval and CASH) | $1,500 | $2,500 | $3,500 | $4,500 | RULES §15, TOU, ACC |
| Consistency rule | none | none | none | none | RULES §10/§11 notes, §15 |
| Max position | 2 E-mini / 20 micro | 5 E-mini / 50 micro | 7 E-mini / 70 micro | 9 E-mini / 90 micro | RULES §15; ACC "2 / 20" etc. ("Max Minis / Micros") |
| Min trading days | 0 | 0 | 0 | 0 | RULES §15 "None"; TOU; ACC "0" |
| Bonus at "Switch to Live" | $1,000 (80/20 split) | $2,000 (80/20 split) | $3,000 (80/20 split) | $4,500 (80/20 split) | RULES §15 |
| LIVE credit | $500 | $1,000 | $1,500 | $2,000 | RULES §15 |
| Max accounts | 5 E2L per trader (all sizes) | | | **conflict** for 150K (see §6, §8) | RULES §20, TOU §3.1.1, HC |

### 2.2 Express to Live (E2L) — LIVE

RULES §15 "LIVE Account — From E2L": "LIVE from E2L 25K — Max Contracts 2 lots", "50K — 3 lots", "100K — 4 lots", "150K — 5 lots"; "Static Loss (LIVE): Extensible — grows with cumulative credits"; "Exchanges: CME, CBOT, NYMEX, COMEX"; "Minimum Balance: Equals total cumulative LIVE credit"; "Profit Split: 80% Trader / 20% Phidias"; "LIVE Payout: Every day, no max limit ($500 min)"; "Overnight / Over-Week: Not permitted (unless agreed)". TOU §3.2.3 "E2L LIVE Account Details": "Max Contracts (E-mini) 2 / 3 / 4 / 5" and "Static Drawdown $500 / $650 / $800 / $1,000"; example: "A User validates 5 CASH E2L 25K accounts … their LIVE account will be credited with a cumulative $2,500 (5 x $500) in live credit, with 2 lots (or 20 micro-lots) per validated account and an authorized loss of $2,500 in total." Micros per size for LIVE are not published except the 25K example. Real-money LIVE accounts are with Dorman Trading (ACC FAQ: "LIVE accounts trade real capital through Dorman Trading, a registered FCM").

| Field | LIVE from 25K | from 50K | from 100K | from 150K | Source |
|---|---|---|---|---|---|
| Drawdown | $500 static, "Extensible" | $650 | $800 | $1,000 | TOU §3.2.3, RULES §15 |
| Drawdown type | `other` — static loss that grows with cumulative LIVE credits; min balance = cumulative LIVE credit | same | same | same | RULES §15 |
| Daily loss limit | none (E2L "No daily loss limit" — TOU §3.2.3 lists it under E2L specific rules) | same | same | same | TOU §3.2.3 |
| Max position | 2 lots (TOU: E-mini) | 3 | 4 | 5 | RULES §15, TOU |
| Min trading days | 0 | 0 | 0 | 0 | TOU §3.2.3 |

### 2.3 Fundamental — evaluation and funded (CASH)

Common rules, RULES §16 "Common Rules — All Fundamental Sizes": "Drawdown Type: EOD trailing" · "Consistency (Eval): None" · "Consistency (Funded): 30% max per day" · "Min. Trading Days (Eval): 3" · "Min. Trading Days (Funded): 10" · "Daily Loss Limit: None" · "News Trading: Authorized (Eval, CASH, LIVE)" · "Overnight / Over-Week: Not permitted" · "Reset (Eval failure): Unlimited".

EOD mechanics (RULES §4, also TOU §3.1.4 and HC `4-regles-du-end-of-day-eod`; tagged "Fundamental, Premium"):
- "Your authorized drawdown is calculated at the end of each trading day, and this calculation does not change during the day. However, if your balance reaches and exceeds the value of your drawdown limit, you will be liquidated. When your account balance reaches a new profit peak at the end of the trading day, the drawdown is calculated taking into account only the profits realized at that time, excluding unrealized profits."
- "The daily stop-loss level is adjusted at the end of each trading session, which ends at 10:00 PM UTC+2."
- "On evaluation accounts: The EOD Drawdown always persists until the end of the evaluation."
- "On funded accounts: Your EOD Drawdown stops at $50,100 for a 50K account and at $100,100 for a 100K account. Once the EOD Drawdown reaches these limits, it will no longer trail." — **no value published for 150K** (gap).
- "Important: Your liquidation price remains fixed for the day. … even if you make $1,000 in the morning, your EOD stop loss remains at $47,500 — allowing a maximum loss of $3,500 for the rest of that day."

Trading hours (Fundamental only), RULES §6: "CME Globex: Sun–Fri, 11:00 PM – 9:59 PM (UTC)" · "Daily Break: 60 min from 9:59 PM UTC" · "Close All Positions By: 9:59 PM UTC" · "Failure to close a position before the deadline results in account disqualification." — **conflicts** with HC FAQ (see §3, §8).

30% consistency rule (funded CASH only), RULES §11: "Applies To: CASH accounts only" · "Rule: No single day can exceed 30% of total profit" · "Min. Calculation Period: 4 trading days". Example: "If your total profit is $1,000 on your account, one of the previous days cannot exceed $300 … you just need to increase your total profit". TOU §3.1.3 words it as: "Under no circumstances should the profit balance of a trading day exceed 30% of the total balance."

Funded (CASH) rules that change what the trader may do (RULES §12, TOU §3.1.5): "Trading Days (Fundamental): 10 days between payouts"; "A trading day is counted only if it shows a positive PnL of at least: 50K $150 / 100K $200 / 150K $250"; HC `pour-qu-une-journee-soit-comptabilisee-…`: "at least one position must be opened AND closed during the day"; "Account Minimum Thresholds 50K $52,600 / 100K $103,700 / 150K $154,500"; "Balance After Withdrawal Cannot Fall Below 50K $50,100 / 100K $100,100 / 150K $150,100"; "Maximum Withdrawal per Period 50K Up to $2,000 / 100K Up to $2,500 / 150K Up to $2,750". Scaling: none published.

| Field | Fundamental 50K | Fundamental 100K | Fundamental 150K | Source |
|---|---|---|---|---|
| Account size | 50K ($50,000) | 100K ($100,000) | 150K ($150,000) | RULES §16 |
| Drawdown amount (eval and funded) | $2,500 | $3,000 | $4,500 | RULES §16 "EOD Trailing Drawdown"; ACC (`fundedDrawdown` same) |
| Drawdown type | `trailing_eod` | `trailing_eod` | `trailing_eod` | RULES §4, §16 |
| Stops trailing | Eval: never; Funded: at $50,100 | Eval: never; Funded: at $100,100 | Eval: never; Funded: **not published** | RULES §4, TOU §3.1.4 |
| Daily loss limit | none (stated) | none | none | RULES §16 |
| Profit target (eval) | $4,000 | $6,000 | $9,000 | RULES §16 |
| Consistency | Eval none; Funded 30% | same | same | RULES §11, §16 |
| Max position | 10 E-mini / 100 micro | 14 E-mini / 140 micro | 17 E-mini / 170 micro | RULES §16; ACC |
| Min trading days | Eval 3; Funded 10 (between payouts) | same | same | RULES §16; TOU §2 "minimum of 3 trading days (Fundamental)" |

### 2.4 Premium — evaluation and funded (CASH)

Common rules, RULES §17 "Common Rules — All Premium Sizes": "Drawdown Type: EOD trailing" · "Consistency (Eval): None" · "Consistency (Funded CASH): 30% max per day" · "Min. Trading Days (Eval): 1" · "Min. Days Between Payouts (Funded): 5" · "CASH Account Funded: 1 trading day after eval pass" · "Daily Loss Limit: None" · "News Trading: Authorized (Eval, CASH, LIVE)" · "Overnight / Over-Week: Authorized" · "Cash Account Reset: Available (1x per CASH account)".
- EOD mechanics, funded floors and the 30% rule: same sections as Fundamental (RULES §4, §11 are tagged "Fundamental, Premium").
- Overnight (RULES §5): "Positions on Premium accounts can remain open overnight and over-week, with a lot size corresponding to the size of your account." Premium is excluded from the Fundamental trading-hours rule (RULES §6).
- Cash Account Reset (RULES §17): "If a funded Premium CASH account is liquidated due to a drawdown breach, the trader can reset it once at a flat fee." "The Cash Account Reset is only available within 24 hours of the account being liquidated."
- Funded withdrawal thresholds, min daily PnL and per-period caps are the same values as Fundamental (RULES §12; ACC). Scaling: none published.

| Field | Premium 50K | Premium 100K | Premium 150K | Source |
|---|---|---|---|---|
| Account size | 50K ($50,000) | 100K ($100,000) | 150K ($150,000) | RULES §17 |
| Drawdown amount (eval and funded) | $2,500 | $3,000 | $4,500 | RULES §17; ACC |
| Drawdown type | `trailing_eod` | `trailing_eod` | `trailing_eod` | RULES §4, §17 |
| Stops trailing | Eval: never; Funded: at $50,100 | Eval: never; Funded: at $100,100 | Eval: never; Funded: **not published** | RULES §4, TOU §3.1.4 |
| Daily loss limit | none (stated) | none | none | RULES §17 |
| Profit target (eval) | $4,000 | $6,000 | $9,000 | RULES §17 |
| Consistency | Eval none; Funded 30% | same | same | RULES §11, §17 |
| Max position | 10 E-mini / 100 micro | 14 E-mini / 140 micro | 17 E-mini / 170 micro | RULES §17; ACC |
| Min trading days | Eval 1; Funded 5 (between payouts) | same | same | RULES §17; TOU §2 "1 trading day (Premium)" |

### 2.5 LIVE via Fundamental / Premium (ELITE LIVE, MASTER LIVE) — discretionary

RULES §12: "For Fundamental and Premium accounts, the live transition is strictly discretionary. A trader becomes eligible for consideration upon reaching 5 successful payouts or $100,000 in cumulative profits across all accounts." TOU §3.1.5: "The User may not refuse the transition, nor elect to remain on their CASH account." RULES §14 (Wallet-funded LIVE): "Default Position Size: 1 mini / 10 micro contracts"; "The maximum loss will always remain fixed at the amount sent ($500)" (example); "Size Extension: To increase the number of authorized contracts, the trader must make a written request by email to support." ELITE LIVE: "60 days live + $125,000 balance"; MASTER LIVE: "120 days live + $250,000 balance". LIVE drawdown type, daily loss limit and reset are **not published** on public pages (gap).

### 2.6 10K Drawdown Challenge (low priority — 10K, competition)

RULES §18: "Account Size: $10,000 USD" · "Max Volume: 25 E-mini contracts" · "Drawdown: Trailing $10,000 EOD" · "Instruments: All CME CBOT NYMEX COMEX" · "Overnight: Allowed (limit 25 contracts / 250 micros)" · "Weekend: Allowed" · "Multiple Registrations: Unlimited, simultaneous multi-account" · "Ranked by total profit" · "Multi-accounts: best account counted". TOU §3.2.2: "The challenge has a duration of 1 month." No profit target / pass criterion (ranking competition). TOU §3.1.1 lists "1 10k drawdown account" among funded-account limits.

---

## 3. Trading-day reset

- **Published:** RULES §4 (EOD Rules; tagged Fundamental, Premium): "The daily stop-loss level is adjusted at the end of each trading session, which ends at 10:00 PM UTC+2." · "Session End: 10:00 PM UTC+2". Same in HC `4-regles-du-end-of-day-eod` and the help-center copy of the rules (HC `regles`: "22h00 UTC+2").
- **Fundamental close-out deadline:** RULES §6 / TOU §3.2.1 / HC `10-horaires-de-negociations-concerne-uniquement-les-comptes-fundamental`: "Close All Positions By: 9:59 PM UTC"; "CME Globex: Sun–Fri, 11:00 PM – 9:59 PM (UTC)"; "60 min from 9:59 PM UTC".
- **Conflict:** HC FAQ `puis-je-effectuer-des-transactions-pendant-la-nuit-ou-sur-plusieurs-seances-de-t`: "For FUNDAMENTAL accounts, ALL positions opened by our traders MUST be closed before 20:59 (UTC). Traders are allowed to trade DURING the night session if they wish, from 22:00 (UTC) until 20:59 (UTC) the following day." — differs from "9:59 PM UTC" / "11:00 PM" on RULES and TOU.
- The session end is published as a fixed offset ("UTC+2") and the close-out as "UTC"; the firm does not publish how these relate to each other or to daylight-saving changes. The researcher does not convert them; the user should confirm the reset instant to store in the preset.
- **E2L:** no E2L-specific day-end time is published; the 10:00 PM UTC+2 statement is in the EOD section, which says "The EOD rules described above do not apply to E2L accounts." E2L must still close before the end of the trading day (RULES §5).
- **Holidays/Sundays:** HC `les-dimanches-et-jours-feries-sont-ils-consideres-comme-des-jours-de-bourse`: "Sundays are not included in the trading week. As for public holidays, you can indeed trade if the market is open on that day. However, public holidays are not counted as trading days." RULES §6 refers to "Holiday Calendar: cmegroup.com/holiday-calendar". No early-close rule beyond that.

## 4. Trade copying stance

**Allowed, within account-count limits; third-party copiers permitted.** Verbatim:

- RULES §20 "Account Limits & Copy Trading" (https://phidiaspropfirm.com/rules), identical in HC `14-nombre-de-comptes-autorises-et-copy-trading`:
  > "Copy trading is allowed within our guidelines regarding the number of authorized accounts. Each person, company or Internet connection can own up to 5 CASH Premium + 5 CASH Fundamental + 5 CASH/LIVE Express to Live accounts."
  > "You may use third-party copy trading software, but Phidias assumes no responsibility for failures or technical problems on these platforms. If we discover identical strategies exceeding the maximum allowed capital, we reserve the right to suspend these accounts."
- TOU §3.1.1 "COPY TRADING" (https://phidiaspropfirm.com/tou):
  > "The Company authorizes the use of copy trading, as long as the use of this method respects the Company’s guidelines regarding the number of authorized accounts."
  > "This means that Users have the freedom to use third-party software or platforms that offer copy trading functionalities. However, any use of these tools is the responsibility of the User. The Company assumes no responsibility in case of any failure or technical problem on these third-party platforms."
  > "It is each User’s responsibility to ensure that the use of these copy trading software or platforms complies with the Company’s rules regarding the number of funded accounts. Any exceeding of this limit will be considered a violation of the Company’s policy and no payment will be made to the User for accounts in violation."
- TRDV (https://phidiaspropfirm.com/tradovate): "Built-in Trade Copier — Copy trades across multiple funded accounts in real time. Running 5 accounts? One click copies to all of them."
- **Same person's accounts:** allowed (copy trading across the user's own accounts is explicitly marketed — TRDV — and permitted up to the limits).
- **Opposite positions across accounts are forbidden** (TOU §3.1.3): "The User can never be both buyer in one account and seller in another account at the same time. This applies to the User’s personal accounts or in collaboration with other Users." RULES §10 lists as a "lottery transaction": "inverse operation (buy vs sell) via multiple accounts on the same asset."
- **Account-count limits:** see §6.
- **Copying to/from accounts at other firms:** not addressed on any official page found (gap). Related: HC `quel-capital-vais-je-negocier-en-tant-que-trader-phidias`: "If we discover strategies traded identically on different accounts and whose allocated capital value exceeds 15 accounts, we reserve the right to suspend these accounts".

## 5. Automated trading stance

**Fully automated trading is not permitted; semi-automated software is permitted only if the trader actively monitors and manually adjusts all trades.** Verbatim:

- RULES §10 "Consistency Policy", item 3 (tagged Fundamental, Premium):
  > "3. No Automated / HFT Trading — The use of robots, fully automated trading algorithms, or any other form of automated trading is not permitted. Only semi-automated software is permitted, provided the trader actively monitors and manually adjusts all trades."
- TOU §3.1.3 (the TOU states §3 rules "apply in principle to all CASH accounts, namely Fundamental, Premium, Express to Live (E2L) and 10K Drawdown accounts"):
  > "Prohibition of automated trading and High frequency trading: the use of robots, fully automated trading algorithms or any form of automated trading is not authorized, except for semi-automated software, provided the User actively monitors and manually adjusts all operations;"
- HC `15-regularite-politique-de-coherence-chez-phidias-propfirm`: same text ("No Automated Trading and High Frequency Trading (HFT)").
- **Trade copiers:** explicitly allowed (§4). **Webhooks / TradingView alerts:** not named. TRDV markets TradingView order entry ("execute trades directly from TradingView on your funded account") but does not address webhook/alert automation (gap). For All Win, routing TradingView signals automatically to accounts would fall under "automated trading" unless the user's workflow qualifies as "semi-automated … actively monitors and manually adjusts all trades" — the firm does not define this; the user should confirm with the firm.
- Scope note: the RULES consistency policy section is tagged Fundamental/Premium; the TOU applies the prohibition to all CASH accounts including E2L. Evaluation accounts are not separately addressed in the automation clause.

## 6. Other rules that affect routing or risk

- **News trading:** allowed on all accounts — RULES §7: "Yes, you are authorized to trade during news events on all accounts." RULES §10 also lists as discouraged: "Full margin trading during high volatility economic announcements."
- **Overnight / weekend:** Fundamental — "All positions must be CLOSED before end of day"; Premium — "Overnight & over-week positions ALLOWED"; E2L — "Overnight & over-week positions NOT allowed" (RULES §5). "Failure to close a position before market close will result in account disqualification." E2L LIVE: "Not permitted (unless agreed)". 10K Challenge: allowed.
- **Max accounts per person (funded):** RULES §20: "Max CASH Premium: 5 accounts" · "Max CASH Fundamental: 5 accounts" · "Max E2L (all sizes incl. 150K): 5 CASH / LIVE accounts" · "Max 150K CASH (Fund / Premium): 4 accounts (2 Fund / 2 Premium)" · "Evaluation Accounts: No limit". "150K CASH accounts (Fundamental or Premium) are considered equivalent to two standard accounts (50K or 100K). They count double in the total authorized accounts per category." "The limits set above apply across all of the user's login credentials, any user account or person residing at the same address, same IP address, etc." "Any CASH account beyond this threshold will be considered lost." TOU §3.1.1: "The total limit for the number of simultaneous funded accounts is 15 accounts" and "the same tax household, the same company or persons benefiting from the same internet connection are limited to 15 funded accounts". ACC FAQ: "You can have up to 15 funded accounts at the same time". **Conflict on E2L 150K** (TOU "E2L 150K is limited to 1 account" vs RULES "All E2L sizes — including the 150K — count toward this 5-account total and can be cumulated freely within it" vs HC "Max E2L 150K 5 accounts" — see §8).
- **Only one LIVE account per trader** (RULES §12: "A trader can only have one LIVE account."), with plan-eligibility consequences listed there.
- **Contract month:** "You are required to trade the highest volume and most recent contract … Spread trading between two maturities on the same product is forbidden." (RULES §10)
- **Treasury one-tick arbitrage** prohibited (RULES §9/§10) — not relevant to MNQ/NQ.
- **Gambling / risk measures:** RULES §1: "In case of intentional or repetitive non-conforming behavior … we reserve the right to take measures … reducing contracts or closing the account completely".
- **Mandatory stop loss, minimum/maximum trade duration:** none published.
- **Inactivity / evaluation duration:** no maximum days (HC `y-a-t-il-un-nombre-maximum-de-jours-pour-atteindre-mon-objectif-de-profit`: "No, you can take as many days as you need to reach your profit target. There is no maximum."). Monthly subscription evaluations auto-reset 30 days after purchase on failure (RULES §3). No inactivity rule found.
- **Firm may copy positions:** TOU §2: "all accounts are accounts on which the Company may decide, at its sole discretion, whether or not to copy positions using its own software."
- **Scaling plans:** none for evaluation/CASH (fixed max contracts per size). LIVE size extension on written request (RULES §14).

## 7. Machine-readable blocks

One block per plan / phase / size (26 blocks). Money as published, normalized to strings without separators. `notes` is an extra free-text key with published context. `copy_trading_allowed: conditional` = allowed within account-count limits; `automated_trading_allowed: conditional` = fully automated prohibited, semi-automated with active monitoring permitted (§5).

#### Express to Live 25K — evaluation
```yaml
firm: phidias-propfirm
plan: "Express to Live (E2L)"
phase: evaluation
size_label: "25K"
size_usd: "25000.00"
instruments: [MNQ, NQ]
drawdown: { amount_usd: "500.00", type: static, stops_trailing_at: "never trails (static)" }
daily_loss_limit: { amount_usd: null, kind: none }
profit_target_usd: "1500.00"
consistency_rule: { text: null, threshold_pct: null }
max_position: { minis: 2, micros: 20, scaling: "none published" }
min_trading_days: 0
trading_day_reset: { time: null, tz_as_published: null }
copy_trading_allowed: conditional
automated_trading_allowed: conditional
status: partial
notes: "Liquidation threshold published as $24,500 (formula published: Initial Capital - Static Drawdown). Overnight/over-week not permitted. Max 5 E2L accounts per trader. trading_day_reset null: no E2L-specific day-end time published (the 10:00 PM UTC+2 session end is stated in the EOD section, which says it does not apply to E2L) — see gaps."
sources:
  - { field: drawdown, url: "https://phidiaspropfirm.com/rules", accessed: "2026-10-23" }
  - { field: drawdown, url: "https://phidiaspropfirm.com/tou", accessed: "2026-10-23" }
  - { field: drawdown, url: "https://phidiaspropfirm.com/accounts", accessed: "2026-10-23" }
  - { field: profit_target, url: "https://phidiaspropfirm.com/rules", accessed: "2026-10-23" }
  - { field: max_position, url: "https://phidiaspropfirm.com/rules", accessed: "2026-10-23" }
  - { field: max_position, url: "https://phidiaspropfirm.com/accounts", accessed: "2026-10-23" }
  - { field: daily_loss_limit, url: "https://phidiaspropfirm.com/rules", accessed: "2026-10-23" }
  - { field: consistency_rule, url: "https://phidiaspropfirm.com/rules", accessed: "2026-10-23" }
  - { field: min_trading_days, url: "https://phidiaspropfirm.com/rules", accessed: "2026-10-23" }
  - { field: min_trading_days, url: "https://phidiaspropfirm.com/tou", accessed: "2026-10-23" }
  - { field: trading_day_reset, url: "https://phidiaspropfirm.com/rules", accessed: "2026-10-23" }
  - { field: copy_trading, url: "https://phidiaspropfirm.com/rules", accessed: "2026-10-23" }
  - { field: copy_trading, url: "https://phidiaspropfirm.com/tou", accessed: "2026-10-23" }
  - { field: automated_trading, url: "https://phidiaspropfirm.com/rules", accessed: "2026-10-23" }
  - { field: automated_trading, url: "https://phidiaspropfirm.com/tou", accessed: "2026-10-23" }
```

#### Express to Live 25K — funded (CASH)
```yaml
firm: phidias-propfirm
plan: "Express to Live (E2L)"
phase: funded
size_label: "25K"
size_usd: "25000.00"
instruments: [MNQ, NQ]
drawdown: { amount_usd: "500.00", type: static, stops_trailing_at: "never trails (static)" }
daily_loss_limit: { amount_usd: null, kind: none }
profit_target_usd: "1500.00"
consistency_rule: { text: null, threshold_pct: null }
max_position: { minis: 2, micros: 20, scaling: "none published" }
min_trading_days: 0
trading_day_reset: { time: null, tz_as_published: null }
copy_trading_allowed: conditional
automated_trading_allowed: conditional
status: partial
notes: "Funded CASH account 'with the same rules'; reach the profit target again, then 'Switch to Live' credits a fixed $1,000 bonus (80/20) and converts to LIVE. Standard withdrawal rules do not apply. Liquidation $24,500."
sources:
  - { field: drawdown, url: "https://phidiaspropfirm.com/rules", accessed: "2026-10-23" }
  - { field: drawdown, url: "https://phidiaspropfirm.com/tou", accessed: "2026-10-23" }
  - { field: drawdown, url: "https://phidiaspropfirm.com/accounts", accessed: "2026-10-23" }
  - { field: profit_target, url: "https://phidiaspropfirm.com/rules", accessed: "2026-10-23" }
  - { field: max_position, url: "https://phidiaspropfirm.com/rules", accessed: "2026-10-23" }
  - { field: max_position, url: "https://phidiaspropfirm.com/accounts", accessed: "2026-10-23" }
  - { field: daily_loss_limit, url: "https://phidiaspropfirm.com/rules", accessed: "2026-10-23" }
  - { field: consistency_rule, url: "https://phidiaspropfirm.com/rules", accessed: "2026-10-23" }
  - { field: min_trading_days, url: "https://phidiaspropfirm.com/rules", accessed: "2026-10-23" }
  - { field: min_trading_days, url: "https://phidiaspropfirm.com/tou", accessed: "2026-10-23" }
  - { field: trading_day_reset, url: "https://phidiaspropfirm.com/rules", accessed: "2026-10-23" }
  - { field: copy_trading, url: "https://phidiaspropfirm.com/rules", accessed: "2026-10-23" }
  - { field: copy_trading, url: "https://phidiaspropfirm.com/tou", accessed: "2026-10-23" }
  - { field: automated_trading, url: "https://phidiaspropfirm.com/rules", accessed: "2026-10-23" }
  - { field: automated_trading, url: "https://phidiaspropfirm.com/tou", accessed: "2026-10-23" }
  - { field: funded_rules, url: "https://phidiaspropfirm.com/accounts (configurator: fundedDrawdown/fundedTarget/fundedMinDays)", accessed: "2026-10-23" }
```

#### Express to Live 25K — live
```yaml
firm: phidias-propfirm
plan: "Express to Live (E2L)"
phase: live
size_label: "25K"
size_usd: "25000.00"
instruments: [MNQ, NQ]
drawdown: { amount_usd: "500.00", type: other, stops_trailing_at: "static; LIVE static loss is 'Extensible — grows with cumulative credits'" }
daily_loss_limit: { amount_usd: null, kind: none }
profit_target_usd: null
consistency_rule: { text: null, threshold_pct: null }
max_position: { minis: 2, micros: null, scaling: "Size extension only on written request to support (Risk Management decides)" }
min_trading_days: 0
trading_day_reset: { time: null, tz_as_published: null }
copy_trading_allowed: conditional
automated_trading_allowed: conditional
status: partial
notes: "Static Loss (LIVE) published as 'Extensible — grows with cumulative credits'; minimum balance 'Equals total cumulative LIVE credit' (LIVE credit for this size: $500). Overnight/over-week 'Not permitted (unless agreed)'. Max contracts published as 2 'lots' (rules page) / 'Max Contracts (E-mini)' 2 (TOU). Micros not published per size; help center states 10 micro-lots = 1 mini-lot. TOU example: '2 lots (or 20 micro-lots) per validated account'."
sources:
  - { field: drawdown, url: "https://phidiaspropfirm.com/tou", accessed: "2026-10-23" }
  - { field: max_position, url: "https://phidiaspropfirm.com/rules", accessed: "2026-10-23" }
  - { field: max_position, url: "https://phidiaspropfirm.com/tou", accessed: "2026-10-23" }
  - { field: overnight, url: "https://phidiaspropfirm.com/rules", accessed: "2026-10-23" }
  - { field: min_trading_days, url: "https://phidiaspropfirm.com/tou", accessed: "2026-10-23" }
  - { field: copy_trading, url: "https://phidiaspropfirm.com/rules", accessed: "2026-10-23" }
  - { field: copy_trading, url: "https://phidiaspropfirm.com/tou", accessed: "2026-10-23" }
  - { field: automated_trading, url: "https://phidiaspropfirm.com/rules", accessed: "2026-10-23" }
  - { field: automated_trading, url: "https://phidiaspropfirm.com/tou", accessed: "2026-10-23" }
```

#### Express to Live 50K — evaluation
```yaml
firm: phidias-propfirm
plan: "Express to Live (E2L)"
phase: evaluation
size_label: "50K"
size_usd: "50000.00"
instruments: [MNQ, NQ]
drawdown: { amount_usd: "650.00", type: static, stops_trailing_at: "never trails (static)" }
daily_loss_limit: { amount_usd: null, kind: none }
profit_target_usd: "2500.00"
consistency_rule: { text: null, threshold_pct: null }
max_position: { minis: 5, micros: 50, scaling: "none published" }
min_trading_days: 0
trading_day_reset: { time: null, tz_as_published: null }
copy_trading_allowed: conditional
automated_trading_allowed: conditional
status: partial
notes: "Liquidation threshold published as $49,350 (formula published: Initial Capital - Static Drawdown). Overnight/over-week not permitted. Max 5 E2L accounts per trader. trading_day_reset null: no E2L-specific day-end time published (the 10:00 PM UTC+2 session end is stated in the EOD section, which says it does not apply to E2L) — see gaps."
sources:
  - { field: drawdown, url: "https://phidiaspropfirm.com/rules", accessed: "2026-10-23" }
  - { field: drawdown, url: "https://phidiaspropfirm.com/tou", accessed: "2026-10-23" }
  - { field: drawdown, url: "https://phidiaspropfirm.com/accounts", accessed: "2026-10-23" }
  - { field: profit_target, url: "https://phidiaspropfirm.com/rules", accessed: "2026-10-23" }
  - { field: max_position, url: "https://phidiaspropfirm.com/rules", accessed: "2026-10-23" }
  - { field: max_position, url: "https://phidiaspropfirm.com/accounts", accessed: "2026-10-23" }
  - { field: daily_loss_limit, url: "https://phidiaspropfirm.com/rules", accessed: "2026-10-23" }
  - { field: consistency_rule, url: "https://phidiaspropfirm.com/rules", accessed: "2026-10-23" }
  - { field: min_trading_days, url: "https://phidiaspropfirm.com/rules", accessed: "2026-10-23" }
  - { field: min_trading_days, url: "https://phidiaspropfirm.com/tou", accessed: "2026-10-23" }
  - { field: trading_day_reset, url: "https://phidiaspropfirm.com/rules", accessed: "2026-10-23" }
  - { field: copy_trading, url: "https://phidiaspropfirm.com/rules", accessed: "2026-10-23" }
  - { field: copy_trading, url: "https://phidiaspropfirm.com/tou", accessed: "2026-10-23" }
  - { field: automated_trading, url: "https://phidiaspropfirm.com/rules", accessed: "2026-10-23" }
  - { field: automated_trading, url: "https://phidiaspropfirm.com/tou", accessed: "2026-10-23" }
```

#### Express to Live 50K — funded (CASH)
```yaml
firm: phidias-propfirm
plan: "Express to Live (E2L)"
phase: funded
size_label: "50K"
size_usd: "50000.00"
instruments: [MNQ, NQ]
drawdown: { amount_usd: "650.00", type: static, stops_trailing_at: "never trails (static)" }
daily_loss_limit: { amount_usd: null, kind: none }
profit_target_usd: "2500.00"
consistency_rule: { text: null, threshold_pct: null }
max_position: { minis: 5, micros: 50, scaling: "none published" }
min_trading_days: 0
trading_day_reset: { time: null, tz_as_published: null }
copy_trading_allowed: conditional
automated_trading_allowed: conditional
status: partial
notes: "Funded CASH account 'with the same rules'; reach the profit target again, then 'Switch to Live' credits a fixed $2,000 bonus (80/20) and converts to LIVE. Standard withdrawal rules do not apply. Liquidation $49,350."
sources:
  - { field: drawdown, url: "https://phidiaspropfirm.com/rules", accessed: "2026-10-23" }
  - { field: drawdown, url: "https://phidiaspropfirm.com/tou", accessed: "2026-10-23" }
  - { field: drawdown, url: "https://phidiaspropfirm.com/accounts", accessed: "2026-10-23" }
  - { field: profit_target, url: "https://phidiaspropfirm.com/rules", accessed: "2026-10-23" }
  - { field: max_position, url: "https://phidiaspropfirm.com/rules", accessed: "2026-10-23" }
  - { field: max_position, url: "https://phidiaspropfirm.com/accounts", accessed: "2026-10-23" }
  - { field: daily_loss_limit, url: "https://phidiaspropfirm.com/rules", accessed: "2026-10-23" }
  - { field: consistency_rule, url: "https://phidiaspropfirm.com/rules", accessed: "2026-10-23" }
  - { field: min_trading_days, url: "https://phidiaspropfirm.com/rules", accessed: "2026-10-23" }
  - { field: min_trading_days, url: "https://phidiaspropfirm.com/tou", accessed: "2026-10-23" }
  - { field: trading_day_reset, url: "https://phidiaspropfirm.com/rules", accessed: "2026-10-23" }
  - { field: copy_trading, url: "https://phidiaspropfirm.com/rules", accessed: "2026-10-23" }
  - { field: copy_trading, url: "https://phidiaspropfirm.com/tou", accessed: "2026-10-23" }
  - { field: automated_trading, url: "https://phidiaspropfirm.com/rules", accessed: "2026-10-23" }
  - { field: automated_trading, url: "https://phidiaspropfirm.com/tou", accessed: "2026-10-23" }
  - { field: funded_rules, url: "https://phidiaspropfirm.com/accounts (configurator: fundedDrawdown/fundedTarget/fundedMinDays)", accessed: "2026-10-23" }
```

#### Express to Live 50K — live
```yaml
firm: phidias-propfirm
plan: "Express to Live (E2L)"
phase: live
size_label: "50K"
size_usd: "50000.00"
instruments: [MNQ, NQ]
drawdown: { amount_usd: "650.00", type: other, stops_trailing_at: "static; LIVE static loss is 'Extensible — grows with cumulative credits'" }
daily_loss_limit: { amount_usd: null, kind: none }
profit_target_usd: null
consistency_rule: { text: null, threshold_pct: null }
max_position: { minis: 3, micros: null, scaling: "Size extension only on written request to support (Risk Management decides)" }
min_trading_days: 0
trading_day_reset: { time: null, tz_as_published: null }
copy_trading_allowed: conditional
automated_trading_allowed: conditional
status: partial
notes: "Static Loss (LIVE) published as 'Extensible — grows with cumulative credits'; minimum balance 'Equals total cumulative LIVE credit' (LIVE credit for this size: $1,000). Overnight/over-week 'Not permitted (unless agreed)'. Max contracts published as 3 'lots' (rules page) / 'Max Contracts (E-mini)' 3 (TOU). Micros not published per size; help center states 10 micro-lots = 1 mini-lot."
sources:
  - { field: drawdown, url: "https://phidiaspropfirm.com/tou", accessed: "2026-10-23" }
  - { field: max_position, url: "https://phidiaspropfirm.com/rules", accessed: "2026-10-23" }
  - { field: max_position, url: "https://phidiaspropfirm.com/tou", accessed: "2026-10-23" }
  - { field: overnight, url: "https://phidiaspropfirm.com/rules", accessed: "2026-10-23" }
  - { field: min_trading_days, url: "https://phidiaspropfirm.com/tou", accessed: "2026-10-23" }
  - { field: copy_trading, url: "https://phidiaspropfirm.com/rules", accessed: "2026-10-23" }
  - { field: copy_trading, url: "https://phidiaspropfirm.com/tou", accessed: "2026-10-23" }
  - { field: automated_trading, url: "https://phidiaspropfirm.com/rules", accessed: "2026-10-23" }
  - { field: automated_trading, url: "https://phidiaspropfirm.com/tou", accessed: "2026-10-23" }
```

#### Express to Live 100K — evaluation
```yaml
firm: phidias-propfirm
plan: "Express to Live (E2L)"
phase: evaluation
size_label: "100K"
size_usd: "100000.00"
instruments: [MNQ, NQ]
drawdown: { amount_usd: "800.00", type: static, stops_trailing_at: "never trails (static)" }
daily_loss_limit: { amount_usd: null, kind: none }
profit_target_usd: "3500.00"
consistency_rule: { text: null, threshold_pct: null }
max_position: { minis: 7, micros: 70, scaling: "none published" }
min_trading_days: 0
trading_day_reset: { time: null, tz_as_published: null }
copy_trading_allowed: conditional
automated_trading_allowed: conditional
status: partial
notes: "Liquidation threshold published as $99,200 (formula published: Initial Capital - Static Drawdown). Overnight/over-week not permitted. Max 5 E2L accounts per trader. trading_day_reset null: no E2L-specific day-end time published (the 10:00 PM UTC+2 session end is stated in the EOD section, which says it does not apply to E2L) — see gaps."
sources:
  - { field: drawdown, url: "https://phidiaspropfirm.com/rules", accessed: "2026-10-23" }
  - { field: drawdown, url: "https://phidiaspropfirm.com/tou", accessed: "2026-10-23" }
  - { field: drawdown, url: "https://phidiaspropfirm.com/accounts", accessed: "2026-10-23" }
  - { field: profit_target, url: "https://phidiaspropfirm.com/rules", accessed: "2026-10-23" }
  - { field: max_position, url: "https://phidiaspropfirm.com/rules", accessed: "2026-10-23" }
  - { field: max_position, url: "https://phidiaspropfirm.com/accounts", accessed: "2026-10-23" }
  - { field: daily_loss_limit, url: "https://phidiaspropfirm.com/rules", accessed: "2026-10-23" }
  - { field: consistency_rule, url: "https://phidiaspropfirm.com/rules", accessed: "2026-10-23" }
  - { field: min_trading_days, url: "https://phidiaspropfirm.com/rules", accessed: "2026-10-23" }
  - { field: min_trading_days, url: "https://phidiaspropfirm.com/tou", accessed: "2026-10-23" }
  - { field: trading_day_reset, url: "https://phidiaspropfirm.com/rules", accessed: "2026-10-23" }
  - { field: copy_trading, url: "https://phidiaspropfirm.com/rules", accessed: "2026-10-23" }
  - { field: copy_trading, url: "https://phidiaspropfirm.com/tou", accessed: "2026-10-23" }
  - { field: automated_trading, url: "https://phidiaspropfirm.com/rules", accessed: "2026-10-23" }
  - { field: automated_trading, url: "https://phidiaspropfirm.com/tou", accessed: "2026-10-23" }
```

#### Express to Live 100K — funded (CASH)
```yaml
firm: phidias-propfirm
plan: "Express to Live (E2L)"
phase: funded
size_label: "100K"
size_usd: "100000.00"
instruments: [MNQ, NQ]
drawdown: { amount_usd: "800.00", type: static, stops_trailing_at: "never trails (static)" }
daily_loss_limit: { amount_usd: null, kind: none }
profit_target_usd: "3500.00"
consistency_rule: { text: null, threshold_pct: null }
max_position: { minis: 7, micros: 70, scaling: "none published" }
min_trading_days: 0
trading_day_reset: { time: null, tz_as_published: null }
copy_trading_allowed: conditional
automated_trading_allowed: conditional
status: partial
notes: "Funded CASH account 'with the same rules'; reach the profit target again, then 'Switch to Live' credits a fixed $3,000 bonus (80/20) and converts to LIVE. Standard withdrawal rules do not apply. Liquidation $99,200."
sources:
  - { field: drawdown, url: "https://phidiaspropfirm.com/rules", accessed: "2026-10-23" }
  - { field: drawdown, url: "https://phidiaspropfirm.com/tou", accessed: "2026-10-23" }
  - { field: drawdown, url: "https://phidiaspropfirm.com/accounts", accessed: "2026-10-23" }
  - { field: profit_target, url: "https://phidiaspropfirm.com/rules", accessed: "2026-10-23" }
  - { field: max_position, url: "https://phidiaspropfirm.com/rules", accessed: "2026-10-23" }
  - { field: max_position, url: "https://phidiaspropfirm.com/accounts", accessed: "2026-10-23" }
  - { field: daily_loss_limit, url: "https://phidiaspropfirm.com/rules", accessed: "2026-10-23" }
  - { field: consistency_rule, url: "https://phidiaspropfirm.com/rules", accessed: "2026-10-23" }
  - { field: min_trading_days, url: "https://phidiaspropfirm.com/rules", accessed: "2026-10-23" }
  - { field: min_trading_days, url: "https://phidiaspropfirm.com/tou", accessed: "2026-10-23" }
  - { field: trading_day_reset, url: "https://phidiaspropfirm.com/rules", accessed: "2026-10-23" }
  - { field: copy_trading, url: "https://phidiaspropfirm.com/rules", accessed: "2026-10-23" }
  - { field: copy_trading, url: "https://phidiaspropfirm.com/tou", accessed: "2026-10-23" }
  - { field: automated_trading, url: "https://phidiaspropfirm.com/rules", accessed: "2026-10-23" }
  - { field: automated_trading, url: "https://phidiaspropfirm.com/tou", accessed: "2026-10-23" }
  - { field: funded_rules, url: "https://phidiaspropfirm.com/accounts (configurator: fundedDrawdown/fundedTarget/fundedMinDays)", accessed: "2026-10-23" }
```

#### Express to Live 100K — live
```yaml
firm: phidias-propfirm
plan: "Express to Live (E2L)"
phase: live
size_label: "100K"
size_usd: "100000.00"
instruments: [MNQ, NQ]
drawdown: { amount_usd: "800.00", type: other, stops_trailing_at: "static; LIVE static loss is 'Extensible — grows with cumulative credits'" }
daily_loss_limit: { amount_usd: null, kind: none }
profit_target_usd: null
consistency_rule: { text: null, threshold_pct: null }
max_position: { minis: 4, micros: null, scaling: "Size extension only on written request to support (Risk Management decides)" }
min_trading_days: 0
trading_day_reset: { time: null, tz_as_published: null }
copy_trading_allowed: conditional
automated_trading_allowed: conditional
status: partial
notes: "Static Loss (LIVE) published as 'Extensible — grows with cumulative credits'; minimum balance 'Equals total cumulative LIVE credit' (LIVE credit for this size: $1,500). Overnight/over-week 'Not permitted (unless agreed)'. Max contracts published as 4 'lots' (rules page) / 'Max Contracts (E-mini)' 4 (TOU). Micros not published per size; help center states 10 micro-lots = 1 mini-lot."
sources:
  - { field: drawdown, url: "https://phidiaspropfirm.com/tou", accessed: "2026-10-23" }
  - { field: max_position, url: "https://phidiaspropfirm.com/rules", accessed: "2026-10-23" }
  - { field: max_position, url: "https://phidiaspropfirm.com/tou", accessed: "2026-10-23" }
  - { field: overnight, url: "https://phidiaspropfirm.com/rules", accessed: "2026-10-23" }
  - { field: min_trading_days, url: "https://phidiaspropfirm.com/tou", accessed: "2026-10-23" }
  - { field: copy_trading, url: "https://phidiaspropfirm.com/rules", accessed: "2026-10-23" }
  - { field: copy_trading, url: "https://phidiaspropfirm.com/tou", accessed: "2026-10-23" }
  - { field: automated_trading, url: "https://phidiaspropfirm.com/rules", accessed: "2026-10-23" }
  - { field: automated_trading, url: "https://phidiaspropfirm.com/tou", accessed: "2026-10-23" }
```

#### Express to Live 150K — evaluation
```yaml
firm: phidias-propfirm
plan: "Express to Live (E2L)"
phase: evaluation
size_label: "150K"
size_usd: "150000.00"
instruments: [MNQ, NQ]
drawdown: { amount_usd: "1000.00", type: static, stops_trailing_at: "never trails (static)" }
daily_loss_limit: { amount_usd: null, kind: none }
profit_target_usd: "4500.00"
consistency_rule: { text: null, threshold_pct: null }
max_position: { minis: 9, micros: 90, scaling: "none published" }
min_trading_days: 0
trading_day_reset: { time: null, tz_as_published: null }
copy_trading_allowed: conditional
automated_trading_allowed: conditional
status: conflict
notes: "Liquidation threshold published as $149,000 (formula published: Initial Capital - Static Drawdown). Overnight/over-week not permitted. Max 5 E2L accounts per trader. E2L 150K account-count limit is a conflict (TOU: 1 account; rules page/help center: within the 5-account cap). trading_day_reset null: no E2L-specific day-end time published (the 10:00 PM UTC+2 session end is stated in the EOD section, which says it does not apply to E2L) — see gaps."
sources:
  - { field: drawdown, url: "https://phidiaspropfirm.com/rules", accessed: "2026-10-23" }
  - { field: drawdown, url: "https://phidiaspropfirm.com/tou", accessed: "2026-10-23" }
  - { field: drawdown, url: "https://phidiaspropfirm.com/accounts", accessed: "2026-10-23" }
  - { field: profit_target, url: "https://phidiaspropfirm.com/rules", accessed: "2026-10-23" }
  - { field: max_position, url: "https://phidiaspropfirm.com/rules", accessed: "2026-10-23" }
  - { field: max_position, url: "https://phidiaspropfirm.com/accounts", accessed: "2026-10-23" }
  - { field: daily_loss_limit, url: "https://phidiaspropfirm.com/rules", accessed: "2026-10-23" }
  - { field: consistency_rule, url: "https://phidiaspropfirm.com/rules", accessed: "2026-10-23" }
  - { field: min_trading_days, url: "https://phidiaspropfirm.com/rules", accessed: "2026-10-23" }
  - { field: min_trading_days, url: "https://phidiaspropfirm.com/tou", accessed: "2026-10-23" }
  - { field: trading_day_reset, url: "https://phidiaspropfirm.com/rules", accessed: "2026-10-23" }
  - { field: copy_trading, url: "https://phidiaspropfirm.com/rules", accessed: "2026-10-23" }
  - { field: copy_trading, url: "https://phidiaspropfirm.com/tou", accessed: "2026-10-23" }
  - { field: automated_trading, url: "https://phidiaspropfirm.com/rules", accessed: "2026-10-23" }
  - { field: automated_trading, url: "https://phidiaspropfirm.com/tou", accessed: "2026-10-23" }
```

#### Express to Live 150K — funded (CASH)
```yaml
firm: phidias-propfirm
plan: "Express to Live (E2L)"
phase: funded
size_label: "150K"
size_usd: "150000.00"
instruments: [MNQ, NQ]
drawdown: { amount_usd: "1000.00", type: static, stops_trailing_at: "never trails (static)" }
daily_loss_limit: { amount_usd: null, kind: none }
profit_target_usd: "4500.00"
consistency_rule: { text: null, threshold_pct: null }
max_position: { minis: 9, micros: 90, scaling: "none published" }
min_trading_days: 0
trading_day_reset: { time: null, tz_as_published: null }
copy_trading_allowed: conditional
automated_trading_allowed: conditional
status: conflict
notes: "Funded CASH account 'with the same rules'; reach the profit target again, then 'Switch to Live' credits a fixed $4,500 bonus (80/20) and converts to LIVE. Standard withdrawal rules do not apply. Liquidation $149,000. E2L 150K account-count limit conflict (see gaps)."
sources:
  - { field: drawdown, url: "https://phidiaspropfirm.com/rules", accessed: "2026-10-23" }
  - { field: drawdown, url: "https://phidiaspropfirm.com/tou", accessed: "2026-10-23" }
  - { field: drawdown, url: "https://phidiaspropfirm.com/accounts", accessed: "2026-10-23" }
  - { field: profit_target, url: "https://phidiaspropfirm.com/rules", accessed: "2026-10-23" }
  - { field: max_position, url: "https://phidiaspropfirm.com/rules", accessed: "2026-10-23" }
  - { field: max_position, url: "https://phidiaspropfirm.com/accounts", accessed: "2026-10-23" }
  - { field: daily_loss_limit, url: "https://phidiaspropfirm.com/rules", accessed: "2026-10-23" }
  - { field: consistency_rule, url: "https://phidiaspropfirm.com/rules", accessed: "2026-10-23" }
  - { field: min_trading_days, url: "https://phidiaspropfirm.com/rules", accessed: "2026-10-23" }
  - { field: min_trading_days, url: "https://phidiaspropfirm.com/tou", accessed: "2026-10-23" }
  - { field: trading_day_reset, url: "https://phidiaspropfirm.com/rules", accessed: "2026-10-23" }
  - { field: copy_trading, url: "https://phidiaspropfirm.com/rules", accessed: "2026-10-23" }
  - { field: copy_trading, url: "https://phidiaspropfirm.com/tou", accessed: "2026-10-23" }
  - { field: automated_trading, url: "https://phidiaspropfirm.com/rules", accessed: "2026-10-23" }
  - { field: automated_trading, url: "https://phidiaspropfirm.com/tou", accessed: "2026-10-23" }
  - { field: funded_rules, url: "https://phidiaspropfirm.com/accounts (configurator: fundedDrawdown/fundedTarget/fundedMinDays)", accessed: "2026-10-23" }
```

#### Express to Live 150K — live
```yaml
firm: phidias-propfirm
plan: "Express to Live (E2L)"
phase: live
size_label: "150K"
size_usd: "150000.00"
instruments: [MNQ, NQ]
drawdown: { amount_usd: "1000.00", type: other, stops_trailing_at: "static; LIVE static loss is 'Extensible — grows with cumulative credits'" }
daily_loss_limit: { amount_usd: null, kind: none }
profit_target_usd: null
consistency_rule: { text: null, threshold_pct: null }
max_position: { minis: 5, micros: null, scaling: "Size extension only on written request to support (Risk Management decides)" }
min_trading_days: 0
trading_day_reset: { time: null, tz_as_published: null }
copy_trading_allowed: conditional
automated_trading_allowed: conditional
status: partial
notes: "Static Loss (LIVE) published as 'Extensible — grows with cumulative credits'; minimum balance 'Equals total cumulative LIVE credit' (LIVE credit for this size: $2,000). Overnight/over-week 'Not permitted (unless agreed)'. Max contracts published as 5 'lots' (rules page) / 'Max Contracts (E-mini)' 5 (TOU). Micros not published per size; help center states 10 micro-lots = 1 mini-lot."
sources:
  - { field: drawdown, url: "https://phidiaspropfirm.com/tou", accessed: "2026-10-23" }
  - { field: max_position, url: "https://phidiaspropfirm.com/rules", accessed: "2026-10-23" }
  - { field: max_position, url: "https://phidiaspropfirm.com/tou", accessed: "2026-10-23" }
  - { field: overnight, url: "https://phidiaspropfirm.com/rules", accessed: "2026-10-23" }
  - { field: min_trading_days, url: "https://phidiaspropfirm.com/tou", accessed: "2026-10-23" }
  - { field: copy_trading, url: "https://phidiaspropfirm.com/rules", accessed: "2026-10-23" }
  - { field: copy_trading, url: "https://phidiaspropfirm.com/tou", accessed: "2026-10-23" }
  - { field: automated_trading, url: "https://phidiaspropfirm.com/rules", accessed: "2026-10-23" }
  - { field: automated_trading, url: "https://phidiaspropfirm.com/tou", accessed: "2026-10-23" }
```

#### Fundamental 50K — evaluation
```yaml
firm: phidias-propfirm
plan: "Fundamental"
phase: evaluation
size_label: "50K"
size_usd: "50000.00"
instruments: [MNQ, NQ]
drawdown: { amount_usd: "2500.00", type: trailing_eod, stops_trailing_at: "Evaluation: 'The EOD Drawdown always persists until the end of the evaluation' (never stops trailing)" }
daily_loss_limit: { amount_usd: null, kind: none }
profit_target_usd: "4000.00"
consistency_rule: { text: "None (evaluation)", threshold_pct: null }
max_position: { minis: 10, micros: 100, scaling: "none published; 10 micro-lots = 1 mini-lot" }
min_trading_days: 3
trading_day_reset: { time: "22:00", tz_as_published: "UTC+2" }
copy_trading_allowed: conditional
automated_trading_allowed: conditional
status: conflict
notes: "Overnight/over-week not permitted; all positions closed before end of day (Fundamental trading hours: close all positions by 9:59 PM UTC — conflict with help-center FAQ 20:59 UTC)."
sources:
  - { field: drawdown, url: "https://phidiaspropfirm.com/rules", accessed: "2026-10-23" }
  - { field: drawdown, url: "https://phidiaspropfirm.com/tou", accessed: "2026-10-23" }
  - { field: drawdown, url: "https://phidiaspropfirm.com/accounts", accessed: "2026-10-23" }
  - { field: profit_target, url: "https://phidiaspropfirm.com/rules", accessed: "2026-10-23" }
  - { field: max_position, url: "https://phidiaspropfirm.com/rules", accessed: "2026-10-23" }
  - { field: max_position, url: "https://phidiaspropfirm.com/accounts", accessed: "2026-10-23" }
  - { field: max_position, url: "https://helpcenter.phidiaspropfirm.com/help/comment-fonctionnent-les-positions-maximales-du-compte", accessed: "2026-10-23" }
  - { field: daily_loss_limit, url: "https://phidiaspropfirm.com/rules", accessed: "2026-10-23" }
  - { field: consistency_rule, url: "https://phidiaspropfirm.com/rules", accessed: "2026-10-23" }
  - { field: min_trading_days, url: "https://phidiaspropfirm.com/rules", accessed: "2026-10-23" }
  - { field: min_trading_days, url: "https://phidiaspropfirm.com/tou", accessed: "2026-10-23" }
  - { field: trading_day_reset, url: "https://phidiaspropfirm.com/rules", accessed: "2026-10-23" }
  - { field: trading_day_reset, url: "https://helpcenter.phidiaspropfirm.com/help/4-regles-du-end-of-day-eod", accessed: "2026-10-23" }
  - { field: copy_trading, url: "https://phidiaspropfirm.com/rules", accessed: "2026-10-23" }
  - { field: copy_trading, url: "https://phidiaspropfirm.com/tou", accessed: "2026-10-23" }
  - { field: automated_trading, url: "https://phidiaspropfirm.com/rules", accessed: "2026-10-23" }
  - { field: automated_trading, url: "https://phidiaspropfirm.com/tou", accessed: "2026-10-23" }
  - { field: trading_hours, url: "https://helpcenter.phidiaspropfirm.com/help/puis-je-effectuer-des-transactions-pendant-la-nuit-ou-sur-plusieurs-seances-de-t", accessed: "2026-10-23" }
```

#### Fundamental 50K — funded (CASH)
```yaml
firm: phidias-propfirm
plan: "Fundamental"
phase: funded
size_label: "50K"
size_usd: "50000.00"
instruments: [MNQ, NQ]
drawdown: { amount_usd: "2500.00", type: trailing_eod, stops_trailing_at: "$50,100 (funded floor, 'trailing stops here')" }
daily_loss_limit: { amount_usd: null, kind: none }
profit_target_usd: null
consistency_rule: { text: "No single day can exceed 30% of total profit (CASH accounts only; min. calculation period 4 trading days)", threshold_pct: "30" }
max_position: { minis: 10, micros: 100, scaling: "none published; 10 micro-lots = 1 mini-lot" }
min_trading_days: 10
trading_day_reset: { time: "22:00", tz_as_published: "UTC+2" }
copy_trading_allowed: conditional
automated_trading_allowed: conditional
status: conflict
notes: "Overnight/over-week not permitted; all positions closed before end of day (Fundamental trading hours: close all positions by 9:59 PM UTC — conflict with help-center FAQ 20:59 UTC). Funded: 10 min trading days between payouts; a trading day counts only with positive PnL >= $150; withdrawal threshold $52,600; max withdrawal per period $2,000; balance after withdrawal cannot fall below start + 100."
sources:
  - { field: drawdown, url: "https://phidiaspropfirm.com/rules", accessed: "2026-10-23" }
  - { field: drawdown, url: "https://phidiaspropfirm.com/tou", accessed: "2026-10-23" }
  - { field: drawdown, url: "https://phidiaspropfirm.com/accounts", accessed: "2026-10-23" }
  - { field: profit_target, url: "https://phidiaspropfirm.com/rules", accessed: "2026-10-23" }
  - { field: max_position, url: "https://phidiaspropfirm.com/rules", accessed: "2026-10-23" }
  - { field: max_position, url: "https://phidiaspropfirm.com/accounts", accessed: "2026-10-23" }
  - { field: max_position, url: "https://helpcenter.phidiaspropfirm.com/help/comment-fonctionnent-les-positions-maximales-du-compte", accessed: "2026-10-23" }
  - { field: daily_loss_limit, url: "https://phidiaspropfirm.com/rules", accessed: "2026-10-23" }
  - { field: consistency_rule, url: "https://phidiaspropfirm.com/rules", accessed: "2026-10-23" }
  - { field: min_trading_days, url: "https://phidiaspropfirm.com/rules", accessed: "2026-10-23" }
  - { field: min_trading_days, url: "https://phidiaspropfirm.com/tou", accessed: "2026-10-23" }
  - { field: trading_day_reset, url: "https://phidiaspropfirm.com/rules", accessed: "2026-10-23" }
  - { field: trading_day_reset, url: "https://helpcenter.phidiaspropfirm.com/help/4-regles-du-end-of-day-eod", accessed: "2026-10-23" }
  - { field: copy_trading, url: "https://phidiaspropfirm.com/rules", accessed: "2026-10-23" }
  - { field: copy_trading, url: "https://phidiaspropfirm.com/tou", accessed: "2026-10-23" }
  - { field: automated_trading, url: "https://phidiaspropfirm.com/rules", accessed: "2026-10-23" }
  - { field: automated_trading, url: "https://phidiaspropfirm.com/tou", accessed: "2026-10-23" }
  - { field: trading_hours, url: "https://helpcenter.phidiaspropfirm.com/help/puis-je-effectuer-des-transactions-pendant-la-nuit-ou-sur-plusieurs-seances-de-t", accessed: "2026-10-23" }
  - { field: funded_rules, url: "https://phidiaspropfirm.com/rules", accessed: "2026-10-23" }
  - { field: funded_rules, url: "https://phidiaspropfirm.com/tou", accessed: "2026-10-23" }
```

#### Fundamental 100K — evaluation
```yaml
firm: phidias-propfirm
plan: "Fundamental"
phase: evaluation
size_label: "100K"
size_usd: "100000.00"
instruments: [MNQ, NQ]
drawdown: { amount_usd: "3000.00", type: trailing_eod, stops_trailing_at: "Evaluation: 'The EOD Drawdown always persists until the end of the evaluation' (never stops trailing)" }
daily_loss_limit: { amount_usd: null, kind: none }
profit_target_usd: "6000.00"
consistency_rule: { text: "None (evaluation)", threshold_pct: null }
max_position: { minis: 14, micros: 140, scaling: "none published; 10 micro-lots = 1 mini-lot" }
min_trading_days: 3
trading_day_reset: { time: "22:00", tz_as_published: "UTC+2" }
copy_trading_allowed: conditional
automated_trading_allowed: conditional
status: conflict
notes: "Overnight/over-week not permitted; all positions closed before end of day (Fundamental trading hours: close all positions by 9:59 PM UTC — conflict with help-center FAQ 20:59 UTC)."
sources:
  - { field: drawdown, url: "https://phidiaspropfirm.com/rules", accessed: "2026-10-23" }
  - { field: drawdown, url: "https://phidiaspropfirm.com/tou", accessed: "2026-10-23" }
  - { field: drawdown, url: "https://phidiaspropfirm.com/accounts", accessed: "2026-10-23" }
  - { field: profit_target, url: "https://phidiaspropfirm.com/rules", accessed: "2026-10-23" }
  - { field: max_position, url: "https://phidiaspropfirm.com/rules", accessed: "2026-10-23" }
  - { field: max_position, url: "https://phidiaspropfirm.com/accounts", accessed: "2026-10-23" }
  - { field: max_position, url: "https://helpcenter.phidiaspropfirm.com/help/comment-fonctionnent-les-positions-maximales-du-compte", accessed: "2026-10-23" }
  - { field: daily_loss_limit, url: "https://phidiaspropfirm.com/rules", accessed: "2026-10-23" }
  - { field: consistency_rule, url: "https://phidiaspropfirm.com/rules", accessed: "2026-10-23" }
  - { field: min_trading_days, url: "https://phidiaspropfirm.com/rules", accessed: "2026-10-23" }
  - { field: min_trading_days, url: "https://phidiaspropfirm.com/tou", accessed: "2026-10-23" }
  - { field: trading_day_reset, url: "https://phidiaspropfirm.com/rules", accessed: "2026-10-23" }
  - { field: trading_day_reset, url: "https://helpcenter.phidiaspropfirm.com/help/4-regles-du-end-of-day-eod", accessed: "2026-10-23" }
  - { field: copy_trading, url: "https://phidiaspropfirm.com/rules", accessed: "2026-10-23" }
  - { field: copy_trading, url: "https://phidiaspropfirm.com/tou", accessed: "2026-10-23" }
  - { field: automated_trading, url: "https://phidiaspropfirm.com/rules", accessed: "2026-10-23" }
  - { field: automated_trading, url: "https://phidiaspropfirm.com/tou", accessed: "2026-10-23" }
  - { field: trading_hours, url: "https://helpcenter.phidiaspropfirm.com/help/puis-je-effectuer-des-transactions-pendant-la-nuit-ou-sur-plusieurs-seances-de-t", accessed: "2026-10-23" }
```

#### Fundamental 100K — funded (CASH)
```yaml
firm: phidias-propfirm
plan: "Fundamental"
phase: funded
size_label: "100K"
size_usd: "100000.00"
instruments: [MNQ, NQ]
drawdown: { amount_usd: "3000.00", type: trailing_eod, stops_trailing_at: "$100,100 (funded floor, 'trailing stops here')" }
daily_loss_limit: { amount_usd: null, kind: none }
profit_target_usd: null
consistency_rule: { text: "No single day can exceed 30% of total profit (CASH accounts only; min. calculation period 4 trading days)", threshold_pct: "30" }
max_position: { minis: 14, micros: 140, scaling: "none published; 10 micro-lots = 1 mini-lot" }
min_trading_days: 10
trading_day_reset: { time: "22:00", tz_as_published: "UTC+2" }
copy_trading_allowed: conditional
automated_trading_allowed: conditional
status: conflict
notes: "Overnight/over-week not permitted; all positions closed before end of day (Fundamental trading hours: close all positions by 9:59 PM UTC — conflict with help-center FAQ 20:59 UTC). Funded: 10 min trading days between payouts; a trading day counts only with positive PnL >= $200; withdrawal threshold $103,700; max withdrawal per period $2,500; balance after withdrawal cannot fall below start + 100."
sources:
  - { field: drawdown, url: "https://phidiaspropfirm.com/rules", accessed: "2026-10-23" }
  - { field: drawdown, url: "https://phidiaspropfirm.com/tou", accessed: "2026-10-23" }
  - { field: drawdown, url: "https://phidiaspropfirm.com/accounts", accessed: "2026-10-23" }
  - { field: profit_target, url: "https://phidiaspropfirm.com/rules", accessed: "2026-10-23" }
  - { field: max_position, url: "https://phidiaspropfirm.com/rules", accessed: "2026-10-23" }
  - { field: max_position, url: "https://phidiaspropfirm.com/accounts", accessed: "2026-10-23" }
  - { field: max_position, url: "https://helpcenter.phidiaspropfirm.com/help/comment-fonctionnent-les-positions-maximales-du-compte", accessed: "2026-10-23" }
  - { field: daily_loss_limit, url: "https://phidiaspropfirm.com/rules", accessed: "2026-10-23" }
  - { field: consistency_rule, url: "https://phidiaspropfirm.com/rules", accessed: "2026-10-23" }
  - { field: min_trading_days, url: "https://phidiaspropfirm.com/rules", accessed: "2026-10-23" }
  - { field: min_trading_days, url: "https://phidiaspropfirm.com/tou", accessed: "2026-10-23" }
  - { field: trading_day_reset, url: "https://phidiaspropfirm.com/rules", accessed: "2026-10-23" }
  - { field: trading_day_reset, url: "https://helpcenter.phidiaspropfirm.com/help/4-regles-du-end-of-day-eod", accessed: "2026-10-23" }
  - { field: copy_trading, url: "https://phidiaspropfirm.com/rules", accessed: "2026-10-23" }
  - { field: copy_trading, url: "https://phidiaspropfirm.com/tou", accessed: "2026-10-23" }
  - { field: automated_trading, url: "https://phidiaspropfirm.com/rules", accessed: "2026-10-23" }
  - { field: automated_trading, url: "https://phidiaspropfirm.com/tou", accessed: "2026-10-23" }
  - { field: trading_hours, url: "https://helpcenter.phidiaspropfirm.com/help/puis-je-effectuer-des-transactions-pendant-la-nuit-ou-sur-plusieurs-seances-de-t", accessed: "2026-10-23" }
  - { field: funded_rules, url: "https://phidiaspropfirm.com/rules", accessed: "2026-10-23" }
  - { field: funded_rules, url: "https://phidiaspropfirm.com/tou", accessed: "2026-10-23" }
```

#### Fundamental 150K — evaluation
```yaml
firm: phidias-propfirm
plan: "Fundamental"
phase: evaluation
size_label: "150K"
size_usd: "150000.00"
instruments: [MNQ, NQ]
drawdown: { amount_usd: "4500.00", type: trailing_eod, stops_trailing_at: "Evaluation: 'The EOD Drawdown always persists until the end of the evaluation' (never stops trailing)" }
daily_loss_limit: { amount_usd: null, kind: none }
profit_target_usd: "9000.00"
consistency_rule: { text: "None (evaluation)", threshold_pct: null }
max_position: { minis: 17, micros: 170, scaling: "none published; 10 micro-lots = 1 mini-lot" }
min_trading_days: 3
trading_day_reset: { time: "22:00", tz_as_published: "UTC+2" }
copy_trading_allowed: conditional
automated_trading_allowed: conditional
status: conflict
notes: "Overnight/over-week not permitted; all positions closed before end of day (Fundamental trading hours: close all positions by 9:59 PM UTC — conflict with help-center FAQ 20:59 UTC)."
sources:
  - { field: drawdown, url: "https://phidiaspropfirm.com/rules", accessed: "2026-10-23" }
  - { field: drawdown, url: "https://phidiaspropfirm.com/tou", accessed: "2026-10-23" }
  - { field: drawdown, url: "https://phidiaspropfirm.com/accounts", accessed: "2026-10-23" }
  - { field: profit_target, url: "https://phidiaspropfirm.com/rules", accessed: "2026-10-23" }
  - { field: max_position, url: "https://phidiaspropfirm.com/rules", accessed: "2026-10-23" }
  - { field: max_position, url: "https://phidiaspropfirm.com/accounts", accessed: "2026-10-23" }
  - { field: max_position, url: "https://helpcenter.phidiaspropfirm.com/help/comment-fonctionnent-les-positions-maximales-du-compte", accessed: "2026-10-23" }
  - { field: daily_loss_limit, url: "https://phidiaspropfirm.com/rules", accessed: "2026-10-23" }
  - { field: consistency_rule, url: "https://phidiaspropfirm.com/rules", accessed: "2026-10-23" }
  - { field: min_trading_days, url: "https://phidiaspropfirm.com/rules", accessed: "2026-10-23" }
  - { field: min_trading_days, url: "https://phidiaspropfirm.com/tou", accessed: "2026-10-23" }
  - { field: trading_day_reset, url: "https://phidiaspropfirm.com/rules", accessed: "2026-10-23" }
  - { field: trading_day_reset, url: "https://helpcenter.phidiaspropfirm.com/help/4-regles-du-end-of-day-eod", accessed: "2026-10-23" }
  - { field: copy_trading, url: "https://phidiaspropfirm.com/rules", accessed: "2026-10-23" }
  - { field: copy_trading, url: "https://phidiaspropfirm.com/tou", accessed: "2026-10-23" }
  - { field: automated_trading, url: "https://phidiaspropfirm.com/rules", accessed: "2026-10-23" }
  - { field: automated_trading, url: "https://phidiaspropfirm.com/tou", accessed: "2026-10-23" }
  - { field: trading_hours, url: "https://helpcenter.phidiaspropfirm.com/help/puis-je-effectuer-des-transactions-pendant-la-nuit-ou-sur-plusieurs-seances-de-t", accessed: "2026-10-23" }
```

#### Fundamental 150K — funded (CASH)
```yaml
firm: phidias-propfirm
plan: "Fundamental"
phase: funded
size_label: "150K"
size_usd: "150000.00"
instruments: [MNQ, NQ]
drawdown: { amount_usd: "4500.00", type: trailing_eod, stops_trailing_at: null }
daily_loss_limit: { amount_usd: null, kind: none }
profit_target_usd: null
consistency_rule: { text: "No single day can exceed 30% of total profit (CASH accounts only; min. calculation period 4 trading days)", threshold_pct: "30" }
max_position: { minis: 17, micros: 170, scaling: "none published; 10 micro-lots = 1 mini-lot" }
min_trading_days: 10
trading_day_reset: { time: "22:00", tz_as_published: "UTC+2" }
copy_trading_allowed: conditional
automated_trading_allowed: conditional
status: conflict
notes: "Overnight/over-week not permitted; all positions closed before end of day (Fundamental trading hours: close all positions by 9:59 PM UTC — conflict with help-center FAQ 20:59 UTC). Funded: 10 min trading days between payouts; a trading day counts only with positive PnL >= $250; withdrawal threshold $154,500; max withdrawal per period $2,750; balance after withdrawal cannot fall below start + 100. Funded trailing-stop level for 150K NOT published (gap)."
sources:
  - { field: drawdown, url: "https://phidiaspropfirm.com/rules", accessed: "2026-10-23" }
  - { field: drawdown, url: "https://phidiaspropfirm.com/tou", accessed: "2026-10-23" }
  - { field: drawdown, url: "https://phidiaspropfirm.com/accounts", accessed: "2026-10-23" }
  - { field: profit_target, url: "https://phidiaspropfirm.com/rules", accessed: "2026-10-23" }
  - { field: max_position, url: "https://phidiaspropfirm.com/rules", accessed: "2026-10-23" }
  - { field: max_position, url: "https://phidiaspropfirm.com/accounts", accessed: "2026-10-23" }
  - { field: max_position, url: "https://helpcenter.phidiaspropfirm.com/help/comment-fonctionnent-les-positions-maximales-du-compte", accessed: "2026-10-23" }
  - { field: daily_loss_limit, url: "https://phidiaspropfirm.com/rules", accessed: "2026-10-23" }
  - { field: consistency_rule, url: "https://phidiaspropfirm.com/rules", accessed: "2026-10-23" }
  - { field: min_trading_days, url: "https://phidiaspropfirm.com/rules", accessed: "2026-10-23" }
  - { field: min_trading_days, url: "https://phidiaspropfirm.com/tou", accessed: "2026-10-23" }
  - { field: trading_day_reset, url: "https://phidiaspropfirm.com/rules", accessed: "2026-10-23" }
  - { field: trading_day_reset, url: "https://helpcenter.phidiaspropfirm.com/help/4-regles-du-end-of-day-eod", accessed: "2026-10-23" }
  - { field: copy_trading, url: "https://phidiaspropfirm.com/rules", accessed: "2026-10-23" }
  - { field: copy_trading, url: "https://phidiaspropfirm.com/tou", accessed: "2026-10-23" }
  - { field: automated_trading, url: "https://phidiaspropfirm.com/rules", accessed: "2026-10-23" }
  - { field: automated_trading, url: "https://phidiaspropfirm.com/tou", accessed: "2026-10-23" }
  - { field: trading_hours, url: "https://helpcenter.phidiaspropfirm.com/help/puis-je-effectuer-des-transactions-pendant-la-nuit-ou-sur-plusieurs-seances-de-t", accessed: "2026-10-23" }
  - { field: funded_rules, url: "https://phidiaspropfirm.com/rules", accessed: "2026-10-23" }
  - { field: funded_rules, url: "https://phidiaspropfirm.com/tou", accessed: "2026-10-23" }
```

#### Premium 50K — evaluation
```yaml
firm: phidias-propfirm
plan: "Premium"
phase: evaluation
size_label: "50K"
size_usd: "50000.00"
instruments: [MNQ, NQ]
drawdown: { amount_usd: "2500.00", type: trailing_eod, stops_trailing_at: "Evaluation: 'The EOD Drawdown always persists until the end of the evaluation' (never stops trailing)" }
daily_loss_limit: { amount_usd: null, kind: none }
profit_target_usd: "4000.00"
consistency_rule: { text: "None (evaluation)", threshold_pct: null }
max_position: { minis: 10, micros: 100, scaling: "none published; 10 micro-lots = 1 mini-lot" }
min_trading_days: 1
trading_day_reset: { time: "22:00", tz_as_published: "UTC+2" }
copy_trading_allowed: conditional
automated_trading_allowed: conditional
status: verified
notes: "Overnight and over-week positions allowed, 'with a lot size corresponding to the size of your account'."
sources:
  - { field: drawdown, url: "https://phidiaspropfirm.com/rules", accessed: "2026-10-23" }
  - { field: drawdown, url: "https://phidiaspropfirm.com/tou", accessed: "2026-10-23" }
  - { field: drawdown, url: "https://phidiaspropfirm.com/accounts", accessed: "2026-10-23" }
  - { field: profit_target, url: "https://phidiaspropfirm.com/rules", accessed: "2026-10-23" }
  - { field: max_position, url: "https://phidiaspropfirm.com/rules", accessed: "2026-10-23" }
  - { field: max_position, url: "https://phidiaspropfirm.com/accounts", accessed: "2026-10-23" }
  - { field: max_position, url: "https://helpcenter.phidiaspropfirm.com/help/comment-fonctionnent-les-positions-maximales-du-compte", accessed: "2026-10-23" }
  - { field: daily_loss_limit, url: "https://phidiaspropfirm.com/rules", accessed: "2026-10-23" }
  - { field: consistency_rule, url: "https://phidiaspropfirm.com/rules", accessed: "2026-10-23" }
  - { field: min_trading_days, url: "https://phidiaspropfirm.com/rules", accessed: "2026-10-23" }
  - { field: min_trading_days, url: "https://phidiaspropfirm.com/tou", accessed: "2026-10-23" }
  - { field: trading_day_reset, url: "https://phidiaspropfirm.com/rules", accessed: "2026-10-23" }
  - { field: trading_day_reset, url: "https://helpcenter.phidiaspropfirm.com/help/4-regles-du-end-of-day-eod", accessed: "2026-10-23" }
  - { field: copy_trading, url: "https://phidiaspropfirm.com/rules", accessed: "2026-10-23" }
  - { field: copy_trading, url: "https://phidiaspropfirm.com/tou", accessed: "2026-10-23" }
  - { field: automated_trading, url: "https://phidiaspropfirm.com/rules", accessed: "2026-10-23" }
  - { field: automated_trading, url: "https://phidiaspropfirm.com/tou", accessed: "2026-10-23" }
```

#### Premium 50K — funded (CASH)
```yaml
firm: phidias-propfirm
plan: "Premium"
phase: funded
size_label: "50K"
size_usd: "50000.00"
instruments: [MNQ, NQ]
drawdown: { amount_usd: "2500.00", type: trailing_eod, stops_trailing_at: "$50,100 (funded floor, 'trailing stops here')" }
daily_loss_limit: { amount_usd: null, kind: none }
profit_target_usd: null
consistency_rule: { text: "No single day can exceed 30% of total profit (CASH accounts only; min. calculation period 4 trading days)", threshold_pct: "30" }
max_position: { minis: 10, micros: 100, scaling: "none published; 10 micro-lots = 1 mini-lot" }
min_trading_days: 5
trading_day_reset: { time: "22:00", tz_as_published: "UTC+2" }
copy_trading_allowed: conditional
automated_trading_allowed: conditional
status: verified
notes: "Overnight and over-week positions allowed, 'with a lot size corresponding to the size of your account'. Funded: 5 min trading days between payouts; a trading day counts only with positive PnL >= $150; withdrawal threshold $52,600; max withdrawal per period $2,000; balance after withdrawal cannot fall below start + 100. Premium CASH Account Reset available 1x within 24h after liquidation."
sources:
  - { field: drawdown, url: "https://phidiaspropfirm.com/rules", accessed: "2026-10-23" }
  - { field: drawdown, url: "https://phidiaspropfirm.com/tou", accessed: "2026-10-23" }
  - { field: drawdown, url: "https://phidiaspropfirm.com/accounts", accessed: "2026-10-23" }
  - { field: profit_target, url: "https://phidiaspropfirm.com/rules", accessed: "2026-10-23" }
  - { field: max_position, url: "https://phidiaspropfirm.com/rules", accessed: "2026-10-23" }
  - { field: max_position, url: "https://phidiaspropfirm.com/accounts", accessed: "2026-10-23" }
  - { field: max_position, url: "https://helpcenter.phidiaspropfirm.com/help/comment-fonctionnent-les-positions-maximales-du-compte", accessed: "2026-10-23" }
  - { field: daily_loss_limit, url: "https://phidiaspropfirm.com/rules", accessed: "2026-10-23" }
  - { field: consistency_rule, url: "https://phidiaspropfirm.com/rules", accessed: "2026-10-23" }
  - { field: min_trading_days, url: "https://phidiaspropfirm.com/rules", accessed: "2026-10-23" }
  - { field: min_trading_days, url: "https://phidiaspropfirm.com/tou", accessed: "2026-10-23" }
  - { field: trading_day_reset, url: "https://phidiaspropfirm.com/rules", accessed: "2026-10-23" }
  - { field: trading_day_reset, url: "https://helpcenter.phidiaspropfirm.com/help/4-regles-du-end-of-day-eod", accessed: "2026-10-23" }
  - { field: copy_trading, url: "https://phidiaspropfirm.com/rules", accessed: "2026-10-23" }
  - { field: copy_trading, url: "https://phidiaspropfirm.com/tou", accessed: "2026-10-23" }
  - { field: automated_trading, url: "https://phidiaspropfirm.com/rules", accessed: "2026-10-23" }
  - { field: automated_trading, url: "https://phidiaspropfirm.com/tou", accessed: "2026-10-23" }
  - { field: funded_rules, url: "https://phidiaspropfirm.com/rules", accessed: "2026-10-23" }
  - { field: funded_rules, url: "https://phidiaspropfirm.com/tou", accessed: "2026-10-23" }
```

#### Premium 100K — evaluation
```yaml
firm: phidias-propfirm
plan: "Premium"
phase: evaluation
size_label: "100K"
size_usd: "100000.00"
instruments: [MNQ, NQ]
drawdown: { amount_usd: "3000.00", type: trailing_eod, stops_trailing_at: "Evaluation: 'The EOD Drawdown always persists until the end of the evaluation' (never stops trailing)" }
daily_loss_limit: { amount_usd: null, kind: none }
profit_target_usd: "6000.00"
consistency_rule: { text: "None (evaluation)", threshold_pct: null }
max_position: { minis: 14, micros: 140, scaling: "none published; 10 micro-lots = 1 mini-lot" }
min_trading_days: 1
trading_day_reset: { time: "22:00", tz_as_published: "UTC+2" }
copy_trading_allowed: conditional
automated_trading_allowed: conditional
status: verified
notes: "Overnight and over-week positions allowed, 'with a lot size corresponding to the size of your account'."
sources:
  - { field: drawdown, url: "https://phidiaspropfirm.com/rules", accessed: "2026-10-23" }
  - { field: drawdown, url: "https://phidiaspropfirm.com/tou", accessed: "2026-10-23" }
  - { field: drawdown, url: "https://phidiaspropfirm.com/accounts", accessed: "2026-10-23" }
  - { field: profit_target, url: "https://phidiaspropfirm.com/rules", accessed: "2026-10-23" }
  - { field: max_position, url: "https://phidiaspropfirm.com/rules", accessed: "2026-10-23" }
  - { field: max_position, url: "https://phidiaspropfirm.com/accounts", accessed: "2026-10-23" }
  - { field: max_position, url: "https://helpcenter.phidiaspropfirm.com/help/comment-fonctionnent-les-positions-maximales-du-compte", accessed: "2026-10-23" }
  - { field: daily_loss_limit, url: "https://phidiaspropfirm.com/rules", accessed: "2026-10-23" }
  - { field: consistency_rule, url: "https://phidiaspropfirm.com/rules", accessed: "2026-10-23" }
  - { field: min_trading_days, url: "https://phidiaspropfirm.com/rules", accessed: "2026-10-23" }
  - { field: min_trading_days, url: "https://phidiaspropfirm.com/tou", accessed: "2026-10-23" }
  - { field: trading_day_reset, url: "https://phidiaspropfirm.com/rules", accessed: "2026-10-23" }
  - { field: trading_day_reset, url: "https://helpcenter.phidiaspropfirm.com/help/4-regles-du-end-of-day-eod", accessed: "2026-10-23" }
  - { field: copy_trading, url: "https://phidiaspropfirm.com/rules", accessed: "2026-10-23" }
  - { field: copy_trading, url: "https://phidiaspropfirm.com/tou", accessed: "2026-10-23" }
  - { field: automated_trading, url: "https://phidiaspropfirm.com/rules", accessed: "2026-10-23" }
  - { field: automated_trading, url: "https://phidiaspropfirm.com/tou", accessed: "2026-10-23" }
```

#### Premium 100K — funded (CASH)
```yaml
firm: phidias-propfirm
plan: "Premium"
phase: funded
size_label: "100K"
size_usd: "100000.00"
instruments: [MNQ, NQ]
drawdown: { amount_usd: "3000.00", type: trailing_eod, stops_trailing_at: "$100,100 (funded floor, 'trailing stops here')" }
daily_loss_limit: { amount_usd: null, kind: none }
profit_target_usd: null
consistency_rule: { text: "No single day can exceed 30% of total profit (CASH accounts only; min. calculation period 4 trading days)", threshold_pct: "30" }
max_position: { minis: 14, micros: 140, scaling: "none published; 10 micro-lots = 1 mini-lot" }
min_trading_days: 5
trading_day_reset: { time: "22:00", tz_as_published: "UTC+2" }
copy_trading_allowed: conditional
automated_trading_allowed: conditional
status: verified
notes: "Overnight and over-week positions allowed, 'with a lot size corresponding to the size of your account'. Funded: 5 min trading days between payouts; a trading day counts only with positive PnL >= $200; withdrawal threshold $103,700; max withdrawal per period $2,500; balance after withdrawal cannot fall below start + 100. Premium CASH Account Reset available 1x within 24h after liquidation."
sources:
  - { field: drawdown, url: "https://phidiaspropfirm.com/rules", accessed: "2026-10-23" }
  - { field: drawdown, url: "https://phidiaspropfirm.com/tou", accessed: "2026-10-23" }
  - { field: drawdown, url: "https://phidiaspropfirm.com/accounts", accessed: "2026-10-23" }
  - { field: profit_target, url: "https://phidiaspropfirm.com/rules", accessed: "2026-10-23" }
  - { field: max_position, url: "https://phidiaspropfirm.com/rules", accessed: "2026-10-23" }
  - { field: max_position, url: "https://phidiaspropfirm.com/accounts", accessed: "2026-10-23" }
  - { field: max_position, url: "https://helpcenter.phidiaspropfirm.com/help/comment-fonctionnent-les-positions-maximales-du-compte", accessed: "2026-10-23" }
  - { field: daily_loss_limit, url: "https://phidiaspropfirm.com/rules", accessed: "2026-10-23" }
  - { field: consistency_rule, url: "https://phidiaspropfirm.com/rules", accessed: "2026-10-23" }
  - { field: min_trading_days, url: "https://phidiaspropfirm.com/rules", accessed: "2026-10-23" }
  - { field: min_trading_days, url: "https://phidiaspropfirm.com/tou", accessed: "2026-10-23" }
  - { field: trading_day_reset, url: "https://phidiaspropfirm.com/rules", accessed: "2026-10-23" }
  - { field: trading_day_reset, url: "https://helpcenter.phidiaspropfirm.com/help/4-regles-du-end-of-day-eod", accessed: "2026-10-23" }
  - { field: copy_trading, url: "https://phidiaspropfirm.com/rules", accessed: "2026-10-23" }
  - { field: copy_trading, url: "https://phidiaspropfirm.com/tou", accessed: "2026-10-23" }
  - { field: automated_trading, url: "https://phidiaspropfirm.com/rules", accessed: "2026-10-23" }
  - { field: automated_trading, url: "https://phidiaspropfirm.com/tou", accessed: "2026-10-23" }
  - { field: funded_rules, url: "https://phidiaspropfirm.com/rules", accessed: "2026-10-23" }
  - { field: funded_rules, url: "https://phidiaspropfirm.com/tou", accessed: "2026-10-23" }
```

#### Premium 150K — evaluation
```yaml
firm: phidias-propfirm
plan: "Premium"
phase: evaluation
size_label: "150K"
size_usd: "150000.00"
instruments: [MNQ, NQ]
drawdown: { amount_usd: "4500.00", type: trailing_eod, stops_trailing_at: "Evaluation: 'The EOD Drawdown always persists until the end of the evaluation' (never stops trailing)" }
daily_loss_limit: { amount_usd: null, kind: none }
profit_target_usd: "9000.00"
consistency_rule: { text: "None (evaluation)", threshold_pct: null }
max_position: { minis: 17, micros: 170, scaling: "none published; 10 micro-lots = 1 mini-lot" }
min_trading_days: 1
trading_day_reset: { time: "22:00", tz_as_published: "UTC+2" }
copy_trading_allowed: conditional
automated_trading_allowed: conditional
status: verified
notes: "Overnight and over-week positions allowed, 'with a lot size corresponding to the size of your account'."
sources:
  - { field: drawdown, url: "https://phidiaspropfirm.com/rules", accessed: "2026-10-23" }
  - { field: drawdown, url: "https://phidiaspropfirm.com/tou", accessed: "2026-10-23" }
  - { field: drawdown, url: "https://phidiaspropfirm.com/accounts", accessed: "2026-10-23" }
  - { field: profit_target, url: "https://phidiaspropfirm.com/rules", accessed: "2026-10-23" }
  - { field: max_position, url: "https://phidiaspropfirm.com/rules", accessed: "2026-10-23" }
  - { field: max_position, url: "https://phidiaspropfirm.com/accounts", accessed: "2026-10-23" }
  - { field: max_position, url: "https://helpcenter.phidiaspropfirm.com/help/comment-fonctionnent-les-positions-maximales-du-compte", accessed: "2026-10-23" }
  - { field: daily_loss_limit, url: "https://phidiaspropfirm.com/rules", accessed: "2026-10-23" }
  - { field: consistency_rule, url: "https://phidiaspropfirm.com/rules", accessed: "2026-10-23" }
  - { field: min_trading_days, url: "https://phidiaspropfirm.com/rules", accessed: "2026-10-23" }
  - { field: min_trading_days, url: "https://phidiaspropfirm.com/tou", accessed: "2026-10-23" }
  - { field: trading_day_reset, url: "https://phidiaspropfirm.com/rules", accessed: "2026-10-23" }
  - { field: trading_day_reset, url: "https://helpcenter.phidiaspropfirm.com/help/4-regles-du-end-of-day-eod", accessed: "2026-10-23" }
  - { field: copy_trading, url: "https://phidiaspropfirm.com/rules", accessed: "2026-10-23" }
  - { field: copy_trading, url: "https://phidiaspropfirm.com/tou", accessed: "2026-10-23" }
  - { field: automated_trading, url: "https://phidiaspropfirm.com/rules", accessed: "2026-10-23" }
  - { field: automated_trading, url: "https://phidiaspropfirm.com/tou", accessed: "2026-10-23" }
```

#### Premium 150K — funded (CASH)
```yaml
firm: phidias-propfirm
plan: "Premium"
phase: funded
size_label: "150K"
size_usd: "150000.00"
instruments: [MNQ, NQ]
drawdown: { amount_usd: "4500.00", type: trailing_eod, stops_trailing_at: null }
daily_loss_limit: { amount_usd: null, kind: none }
profit_target_usd: null
consistency_rule: { text: "No single day can exceed 30% of total profit (CASH accounts only; min. calculation period 4 trading days)", threshold_pct: "30" }
max_position: { minis: 17, micros: 170, scaling: "none published; 10 micro-lots = 1 mini-lot" }
min_trading_days: 5
trading_day_reset: { time: "22:00", tz_as_published: "UTC+2" }
copy_trading_allowed: conditional
automated_trading_allowed: conditional
status: partial
notes: "Overnight and over-week positions allowed, 'with a lot size corresponding to the size of your account'. Funded: 5 min trading days between payouts; a trading day counts only with positive PnL >= $250; withdrawal threshold $154,500; max withdrawal per period $2,750; balance after withdrawal cannot fall below start + 100. Funded trailing-stop level for 150K NOT published (gap). Premium CASH Account Reset available 1x within 24h after liquidation."
sources:
  - { field: drawdown, url: "https://phidiaspropfirm.com/rules", accessed: "2026-10-23" }
  - { field: drawdown, url: "https://phidiaspropfirm.com/tou", accessed: "2026-10-23" }
  - { field: drawdown, url: "https://phidiaspropfirm.com/accounts", accessed: "2026-10-23" }
  - { field: profit_target, url: "https://phidiaspropfirm.com/rules", accessed: "2026-10-23" }
  - { field: max_position, url: "https://phidiaspropfirm.com/rules", accessed: "2026-10-23" }
  - { field: max_position, url: "https://phidiaspropfirm.com/accounts", accessed: "2026-10-23" }
  - { field: max_position, url: "https://helpcenter.phidiaspropfirm.com/help/comment-fonctionnent-les-positions-maximales-du-compte", accessed: "2026-10-23" }
  - { field: daily_loss_limit, url: "https://phidiaspropfirm.com/rules", accessed: "2026-10-23" }
  - { field: consistency_rule, url: "https://phidiaspropfirm.com/rules", accessed: "2026-10-23" }
  - { field: min_trading_days, url: "https://phidiaspropfirm.com/rules", accessed: "2026-10-23" }
  - { field: min_trading_days, url: "https://phidiaspropfirm.com/tou", accessed: "2026-10-23" }
  - { field: trading_day_reset, url: "https://phidiaspropfirm.com/rules", accessed: "2026-10-23" }
  - { field: trading_day_reset, url: "https://helpcenter.phidiaspropfirm.com/help/4-regles-du-end-of-day-eod", accessed: "2026-10-23" }
  - { field: copy_trading, url: "https://phidiaspropfirm.com/rules", accessed: "2026-10-23" }
  - { field: copy_trading, url: "https://phidiaspropfirm.com/tou", accessed: "2026-10-23" }
  - { field: automated_trading, url: "https://phidiaspropfirm.com/rules", accessed: "2026-10-23" }
  - { field: automated_trading, url: "https://phidiaspropfirm.com/tou", accessed: "2026-10-23" }
  - { field: funded_rules, url: "https://phidiaspropfirm.com/rules", accessed: "2026-10-23" }
  - { field: funded_rules, url: "https://phidiaspropfirm.com/tou", accessed: "2026-10-23" }
```

#### Fundamental / Premium — live (discretionary)
```yaml
firm: phidias-propfirm
plan: "Fundamental / Premium LIVE (incl. ELITE LIVE / MASTER LIVE)"
phase: live
size_label: null
size_usd: null
instruments: [MNQ, NQ]
drawdown: { amount_usd: null, type: other, stops_trailing_at: "not published" }
daily_loss_limit: { amount_usd: null, kind: null }
profit_target_usd: null
consistency_rule: { text: null, threshold_pct: null }
max_position: { minis: 1, micros: 10, scaling: "'Default Position Size 1 mini / 10 micro contracts' (Wallet-funded LIVE); size extension on written request" }
min_trading_days: null
trading_day_reset: { time: null, tz_as_published: null }
copy_trading_allowed: conditional
automated_trading_allowed: conditional
status: unverified
notes: "LIVE transition is discretionary (5 payouts or $100,000 cumulative profits). LIVE drawdown, daily loss and reset are not published publicly; daily_loss_limit not published for LIVE (kind null — see gaps). Max loss equals the amount transferred from the Wallet (example $500)."
sources:
  - { field: max_position, url: "https://phidiaspropfirm.com/rules", accessed: "2026-10-23" }
  - { field: live_transition, url: "https://phidiaspropfirm.com/rules", accessed: "2026-10-23" }
  - { field: live_transition, url: "https://phidiaspropfirm.com/tou", accessed: "2026-10-23" }
  - { field: copy_trading, url: "https://phidiaspropfirm.com/rules", accessed: "2026-10-23" }
  - { field: copy_trading, url: "https://phidiaspropfirm.com/tou", accessed: "2026-10-23" }
  - { field: automated_trading, url: "https://phidiaspropfirm.com/rules", accessed: "2026-10-23" }
  - { field: automated_trading, url: "https://phidiaspropfirm.com/tou", accessed: "2026-10-23" }
```

#### 10K Drawdown Challenge — competition (low priority, outside 25K–300K)
```yaml
firm: phidias-propfirm
plan: "10K Drawdown Challenge"
phase: evaluation
size_label: "10K"
size_usd: "10000.00"
instruments: [MNQ, NQ]
drawdown: { amount_usd: "10000.00", type: trailing_eod, stops_trailing_at: "not published" }
daily_loss_limit: { amount_usd: null, kind: null }
profit_target_usd: null
consistency_rule: { text: null, threshold_pct: null }
max_position: { minis: 25, micros: null, scaling: "Overnight 'Allowed (limit 25 contracts / 250 micros)'" }
min_trading_days: null
trading_day_reset: { time: null, tz_as_published: null }
copy_trading_allowed: conditional
automated_trading_allowed: conditional
status: partial
notes: "Monthly competition ranked by total profit, not a pass/fail evaluation. 'Max Volume 25 E-mini contracts'; drawdown 'Trailing $10,000 EOD'; weekend allowed; unlimited simultaneous registrations. Daily loss limit not stated on the challenge (kind null)."
sources:
  - { field: drawdown, url: "https://phidiaspropfirm.com/rules", accessed: "2026-10-23" }
  - { field: max_position, url: "https://phidiaspropfirm.com/rules", accessed: "2026-10-23" }
  - { field: rules, url: "https://phidiaspropfirm.com/tou", accessed: "2026-10-23" }
```

## 8. Unverified / gaps / conflicts

| # | Field | Status | What is missing or conflicting | Where searched |
|---|---|---|---|---|
| 1 | Trading-day reset / Fundamental close-out time | `conflict` | RULES §4 / HC `4-regles-du-end-of-day-eod`: session "ends at 10:00 PM UTC+2". RULES §6, TOU §3.2.1, HC `10-horaires-…`: "Close All Positions By: 9:59 PM UTC", Globex "11:00 PM – 9:59 PM (UTC)". HC FAQ `puis-je-effectuer-des-transactions-pendant-la-nuit-…`: "closed before 20:59 (UTC)", night session "from 22:00 (UTC)". Fixed offsets are given with no daylight-saving rule. User to confirm the reset instant. | RULES, TOU, HC (75 articles) |
| 2 | E2L trading-day end / reset | gap | No E2L-specific day-end time; the only session-end statement is in the EOD section, which says it does not apply to E2L. YAML `trading_day_reset` is null for E2L. | RULES §4, §5, §15; TOU §3.2.3; HC E2L article |
| 3 | Funded EOD trailing stop level, 150K (Fundamental and Premium) | gap | Published only for 50K ($50,100) and 100K ($100,100). | RULES §4, TOU §3.1.4, HC `4-regles-du-end-of-day-eod`, ACC configurator |
| 4 | Max E2L 150K accounts | `conflict` | TOU §3.1.1 and §3.2.3: "E2L 150K is limited to 1 account" / "E2L 150K is limited to 1 account per User". RULES §20: "Max E2L (all sizes incl. 150K): 5 CASH / LIVE accounts" and "All E2L sizes — including the 150K — count toward this 5-account total and can be cumulated freely within it". HC `14-nombre-de-comptes-autorises-et-copy-trading`: "Max E2L 150K 5 accounts" but also, in the same article, "5 E2L (any mix of 25K/50K/100K + optionally 1x 150K) = OK". | RULES, TOU, HC |
| 5 | 150K Fundamental/Premium count vs per-type caps | ambiguity | RULES: 150K CASH "count double" and max "4 accounts (2 Fund / 2 Premium)"; TOU lists "5 Premium accounts (50k or 100k)", "5 Fundamental accounts (50k or 100k)", "2 150k Fundamental accounts", "2 150k Premium accounts" and also "1 10k drawdown account" (not on RULES). Not contradictory in values, but the interaction is not fully specified. | RULES §20, TOU §3.1.1 |
| 6 | Min trading days to pass — general vs plan sections | `conflict` (resolved by firm's precedence clause, for PO review) | RULES §3 General Rules (all accounts): "Min. Trading Days to Pass 3 days". Plan sections: Premium "1", E2L "None"/0; TOU §2: "3 trading days (Fundamental), 1 trading day (Premium), or 0 trading days (Express to Live)". TOU §3.2: "the existence of a specific rule will always prevail over a general rule" — YAML uses the plan-specific values. Legacy HC FAQ `quel-est-le-nombre-minimum-de-jours-…`: "1 day for Statics accounts and 3 days for Fundamental and Swing accounts". | RULES, TOU, HC |
| 7 | Copy trading to/from accounts at other firms | gap | Not addressed on any official page. | RULES §20, TOU §3.1.1, HC article 14, TRDV |
| 8 | Webhook / TradingView-alert automation | gap | Not named. Rule is "fully automated" prohibited, "semi-automated" allowed with active monitoring and manual adjustment; "semi-automated" is not defined. Needs the user to confirm with the firm whether All Win's signal routing qualifies. | RULES §10, TOU §3.1.3, HC article 15, TRDV |
| 9 | Automation clause scope for evaluation accounts | gap | RULES section tagged Fundamental/Premium; TOU applies to all CASH accounts incl. E2L; evaluation phase not explicitly addressed. | RULES §10, TOU §3.1.3 |
| 10 | "Daily loss limit" wording in HC EOD FAQ | ambiguity | HC `cest-quoi-le-drawdown-eod-end-of-day`: "If your account reaches the daily loss limit at any time during the day, your account will fail" while every plan grid states "Daily Loss Limit: None". Read as the EOD liquidation level, but not stated. YAML uses `none` per the plan grids. | RULES §15–17, HC |
| 11 | LIVE (Fundamental/Premium, ELITE/MASTER) drawdown, daily loss, reset, max position per size | unverified | Only "Default Position Size 1 mini / 10 micro contracts" (Wallet-funded) and max loss = amount transferred are public. Likely behind login / contract. | RULES §12, §14, §19; TOU §3.1.5, §3.2.4 |
| 12 | E2L LIVE micros per size | gap | Only 25K example "2 lots (or 20 micro-lots) per validated account" (TOU); other sizes publish lots/E-mini only. | RULES §15, TOU §3.2.3 |
| 13 | Minimum withdrawal (payout; low priority) | `conflict` | RULES §12: "Min. Withdrawal $500"; TOU §3.1.5: "The minimum withdrawal amount can never be less than $1,000." Legacy HC FAQ also says 10 days "for your first 3 withdrawals" then none, vs RULES "10 days between payouts". Does not change trading limits. | RULES, TOU, HC |
| 14 | Consistency rule wording | ambiguity | RULES: "No single day can exceed 30% of total profit"; TOU §3.1.3: "the profit balance of a trading day exceed 30% of the total balance". YAML uses the RULES wording; threshold 30 in both. | RULES §10–11, TOU §3.1.3, §3.1.8 |
| 15 | Legacy help-center FAQ content | note | Several HC FAQ articles still refer to "Swing" and "25K Static" accounts and old limits ("5 CASH 25K Static accounts … 5 CASH Swing accounts"); current pages name Express to Live / Fundamental / Premium. Treated as outdated, not used for values except where noted. | HC FAQ articles |
| 16 | Member area / contract | not accessed | https://member.phidiaspropfirm.com/ requires login; the CASH contract is not public. | — |

## 9. Changelog

- 2026-10-23 — initial research (Lupa)
