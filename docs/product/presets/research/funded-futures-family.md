# Funded Futures Family — Prop Firm Rules Research

| | |
|---|---|
| **Firm** | Funded Futures Family LLC ("FFF"), futures only, Temecula CA. **All Evaluation and Funded Accounts are simulated** (live capital only in the Professional Stage / Live Stage) |
| **Official domains** | `www.fundedfuturesfamily.com` (main site, WordPress; plain curl works) · `intercom.help/funded-futures-family/en/` (official help center, linked from the site footer as `https://intercom.help/funded-futures-family/en/`; crawled through its `sitemap.xml`, 84 URLs, 68 articles plus 16 collection pages) · `app.fundedfuturesfamily.com` (dashboard login, not used) |
| **Research date** | 2026-10-23 (all URLs below accessed 2026-10-23) |
| **Researcher** | Lupa (Prop Firm Rules Researcher) |
| **File status** | `partial`. Plan tables (profit target, drawdown, DLL, consistency, contract limits) are verified from the help center for 9 plan families. **Not published anywhere found:** commissions, trading-day reset time. **Conflicts:** VPS permission, S2F consistency (25% vs 45%), Pro drawdown type, and several contract-limit figures. All 52 YAML blocks therefore carry `status: conflict`. See §10 |
| **Instruments covered** | MNQ, NQ. The Prohibited Trading Conduct article lists "NQ and MNQ" as markets, and the FAQ says traders can trade CME-listed futures "including the S&P 500, Nasdaq" and that "the full list of supported contracts" is in the Help Center. No product-list article exists in the 68 crawled articles, so the formal contract list is a gap |
| **Platforms (for routing)** | Purchase selector offers **Tradovate** and **WealthCharts**. The connection guide also lists NinjaTrader (sign in with FFF credentials, Simulation) and TradingView (via the Tradovate add-on). Live Stage runs on **Rithmic**. See §2 |

Evidence (raw HTML in `raw/`, stripped text in `txt/`, cleaned text in `c/`, URL on line 1): `/tmp/claude-1003/-srv-www-allwin--maestri-roles-6bf77a80-a1ed-43dc-bc00-08a480f178a5/c3ed71e7-3dd4-4f1a-8a03-9bc3a3fa7b49/scratchpad/r3/funded-futures-family/`. Temporary scratch storage, so the evidence must be captured again at re-verification.

**Programs found.** Evaluation plans: **Prime** (EOD), **Premier+** (EOD or Intraday drawdown, chosen at purchase), **Velocity** (intraday trailing, optional Daily Payout Add-On), **Base** ($2K account). Straight-to-funded (no evaluation): **S2F Standard**, **S2F Payday** (50K), **S2F Accelerate** (25K/50K, invite-only per home page), **S2F Sniper** (25K, invite-only per home page). **Prestige** evaluation is strictly invite-only and leads to the live **Professional Stage**; the **Live Stage** (Rithmic) follows. Sizes: 25K, 50K, 100K, 150K (2K for Base; 50K-150K for Pro/Live). There is no 250K or 300K size.

**Terms.** "DLL" = daily loss limit. "EOD" = end-of-day. "Minis/micros" = contracts. "Qualifying trading day" = day with at least $200 profit (payout rules). The help center says marketing material loses to the rules: "the current Terms & Conditions and Help Center articles govern in the event of any conflict with this disclosure or with marketing materials" (S54).

### Main source list (all accessed 2026-10-23)

| Key | URL |
|---|---|
| S1 | https://intercom.help/funded-futures-family/en/articles/15656536-connecting-your-trading-platform (Connecting Your Trading Platform) |
| S2 | https://intercom.help/funded-futures-family/en/articles/15892413-bots-algorithmic-trading-policy (Bots/Algorithmic Trading Policy) |
| S3 | https://intercom.help/funded-futures-family/en/articles/17708800-prohibited-trading-conduct (Prohibited Trading Conduct (page 1)) |
| S3b | https://intercom.help/funded-futures-family/en/articles/17708968-prohibited-trading-conduct (Prohibited Trading Conduct (duplicate page, same text)) |
| S4 | https://intercom.help/funded-futures-family/en/articles/17155892-device-sharing (Device Sharing) |
| S5 | https://intercom.help/funded-futures-family/en/articles/15892350-permitted-times-to-trade (Permitted Times to Trade) |
| S6 | https://intercom.help/funded-futures-family/en/articles/15892353-news-trading-policy (News Trading Policy) |
| S7 | https://intercom.help/funded-futures-family/en/articles/15892427-maximum-account-idle-time (Maximum Account Idle Time) |
| S8 | https://intercom.help/funded-futures-family/en/articles/15892419-micro-scalping-policy (Micro-scalping Policy) |
| S9 | https://intercom.help/funded-futures-family/en/articles/17709039-all-or-nothing-trading-and-account-rolling (All-or-Nothing Trading and Account Rolling) |
| S10 | https://intercom.help/funded-futures-family/en/articles/15892376-fair-play-and-integrity-policy (Fair Play and Integrity Policy) |
| S11 | https://intercom.help/funded-futures-family/en/articles/15695739-how-many-accounts-per-household (How Many Accounts Per Household?) |
| S12 | https://intercom.help/funded-futures-family/en/articles/15650785-account-fees (Account Fees) |
| S13 | https://intercom.help/funded-futures-family/en/articles/15657092-prime-evaluation-account (Prime Evaluation Account) |
| S14 | https://intercom.help/funded-futures-family/en/articles/15705476-prime-funded-account (Prime Funded Account) |
| S15 | https://intercom.help/funded-futures-family/en/articles/15808767-prime-drawdown (Prime Drawdown) |
| S16 | https://intercom.help/funded-futures-family/en/articles/15808673-prime-consistency (Prime Consistency) |
| S17 | https://intercom.help/funded-futures-family/en/articles/15715050-prime-payout-requirements (Prime Payout Requirements) |
| S18 | https://intercom.help/funded-futures-family/en/articles/16302640-prime-scaling-plan (Prime Scaling Plan) |
| S19 | https://intercom.help/funded-futures-family/en/articles/15809232-premier-evaluation (Premier+ Evaluation) |
| S20 | https://intercom.help/funded-futures-family/en/articles/15811095-premier-funded-account (Premier+ Funded Account) |
| S21 | https://intercom.help/funded-futures-family/en/articles/15850031-premier-drawdown (Premier+ Drawdown) |
| S22 | https://intercom.help/funded-futures-family/en/articles/15811464-premier-payout-requirements (Premier+ Payout Requirements) |
| S23 | https://intercom.help/funded-futures-family/en/articles/16964667-premier-consistency (Premier+ Consistency) |
| S24 | https://intercom.help/funded-futures-family/en/articles/16302826-premier-scaling-plan (Premier+ Scaling Plan) |
| S25 | https://intercom.help/funded-futures-family/en/articles/15850710-velocity-evaluation (Velocity Evaluation) |
| S26 | https://intercom.help/funded-futures-family/en/articles/15851015-velocity-funded-account (Velocity Funded Account) |
| S27 | https://intercom.help/funded-futures-family/en/articles/15878902-velocity-payout-requirements (Velocity Payout Requirements) |
| S28 | https://intercom.help/funded-futures-family/en/articles/15879485-velocity-drawdown (Velocity Drawdown) |
| S29 | https://intercom.help/funded-futures-family/en/articles/15879043-velocity-consistency (Velocity Consistency) |
| S30 | https://intercom.help/funded-futures-family/en/articles/16310772-velocity-scaling-plan (Velocity Scaling Plan) |
| S31 | https://intercom.help/funded-futures-family/en/articles/15879855-straight-to-funded-s2f-funded-account (Straight to Funded (S2F) Funded Account) |
| S32 | https://intercom.help/funded-futures-family/en/articles/15879981-straight-to-funded-s2f-payout-requirements (S2F Payout Requirements) |
| S33 | https://intercom.help/funded-futures-family/en/articles/15880555-straight-to-funded-s2f-consistency-percentage (S2F Consistency Percentage) |
| S34 | https://intercom.help/funded-futures-family/en/articles/15880602-straight-to-funded-s2f-drawdown (S2F Drawdown) |
| S35 | https://intercom.help/funded-futures-family/en/articles/16310780-straight-to-funded-s2f-scaling-plan (S2F Scaling Plan) |
| S36 | https://intercom.help/funded-futures-family/en/articles/16949878-accelerate-plan-funded-account (Accelerate Plan Funded Account) |
| S37 | https://intercom.help/funded-futures-family/en/articles/16950249-accelerate-plan-payout-requirements (Accelerate Plan Payout Requirements) |
| S38 | https://intercom.help/funded-futures-family/en/articles/16964735-s2f-accelerate-plan-consistency-percentage (S2F Accelerate Plan Consistency Percentage) |
| S39 | https://intercom.help/funded-futures-family/en/articles/16964753-s2f-accelerate-plan-drawdown (S2F Accelerate Plan Drawdown) |
| S40 | https://intercom.help/funded-futures-family/en/articles/17333029-25k-accelerate-plan (25k Accelerate Plan) |
| S41 | https://intercom.help/funded-futures-family/en/articles/17075322-base-plan-evaluation (Base Plan Evaluation) |
| S42 | https://intercom.help/funded-futures-family/en/articles/17076299-base-plan-funded-account (Base Plan Funded Account) |
| S43 | https://intercom.help/funded-futures-family/en/articles/17079566-base-plan-payout-requirements (Base Plan Payout Requirements) |
| S44 | https://intercom.help/funded-futures-family/en/articles/17082450-base-plan-drawdown (Base Plan Drawdown) |
| S45 | https://intercom.help/funded-futures-family/en/articles/17083162-base-plan-scaling-plan (Base Plan Scaling Plan) |
| S46 | https://intercom.help/funded-futures-family/en/articles/17332937-payday-50k (Payday 50K) |
| S47 | https://intercom.help/funded-futures-family/en/articles/17333010-25k-sniper-plan (25K Sniper Plan) |
| S48 | https://intercom.help/funded-futures-family/en/articles/16911595-prestige-evaluation (Prestige Evaluation) |
| S49 | https://intercom.help/funded-futures-family/en/articles/15880840-professional-stage-structure (Professional Stage Structure) |
| S50 | https://intercom.help/funded-futures-family/en/articles/15891902-professional-stage-operational-policies (Professional Stage Operational Policies) |
| S51 | https://intercom.help/funded-futures-family/en/articles/15892228-live-stage-structure (Live Stage Structure) |
| S55 | https://intercom.help/funded-futures-family/en/articles/15892316-rules-restricted-countries-regions (Rules: Restricted Countries / Regions) |
| S56 | https://intercom.help/funded-futures-family/en/articles/17709079-how-we-review-payouts (How We Review Payouts) |
| S57 | https://intercom.help/funded-futures-family/en/articles/15650632-what-is-funded-futures-family (What is Funded Futures Family?) |
| S52 | https://www.fundedfuturesfamily.com/terms-and-conditions/ (Terms and Conditions (site-terms-and-conditions.txt)) |
| S53 | https://www.fundedfuturesfamily.com/faq/ (FAQ) |
| S54 | https://www.fundedfuturesfamily.com/ (Home page (plan cards, challenge selector, disclosures)) |

Source-key note: S3 and S3b are two live copies of the same article with identical rule text (S3 carries an extra breadcrumb "Evaluation & Rules"); both read "Updated today" on 2026-10-23, so the policy was very recently rewritten. S2 (Bots policy) is dated September 8, 2026.

---

## 1. Header notes

- Every help center article (68) listed in `sitemap.xml` was captured and read; every article reachable from the 16 collections is in that set.
- Marketing home page plan cards were read for plan availability (invite-only flags) and prices; the rule values come from the help center.
- **Commission / fee schedule: no official page found.** The word "commission" appears only in the legal disclaimer ("not a ... futures commission merchant"). No per-side, round-turn, exchange, NFA or platform fee page was found. See §8.
- Prop-firm review sites linked from the home page (Trustpilot, propfirmmatch) were not used.

## 2. Platforms

| Plan / phase | Platforms / connections as published | Source |
|---|---|---|
| All evaluation and simulated funded plans | Challenge selector: "Tradovate", "WealthCharts". Connection guide: NinjaTrader, Tradovate (web, mobile, desktop), TradingView (through Tradovate), WealthCharts (web, mobile) | S54, S1 |
| Professional Stage (Pro execution environment) | not published | S49 |
| Live Stage | Rithmic infrastructure; market data not included | S51 |

Quotes:
- NinjaTrader: "Log in to the NinjaTrader website using your FFF credentials." (S1) / "Open NinjaTrader and sign in with the same FFF username and password." (S1) / "Select Simulation and click Launch." (S1)
- Tradovate: "Sign in using the username and password provided in your FFF Dashboard." (S1) / "Select the correct server or workspace for your account type (Evaluation or Funded)." (S1)
- TradingView: "Enable the TradingView Add On. In Tradovate, navigate to Settings → Add Ons and enable the TradingView Add On." (S1)
- WealthCharts: "When prompted to connect a broker, select Funded Futures Family to access your trading accounts." (S1)
- Live: "An FFF trader will transition into the Rithmic Trading Stage when:" (S51) / "Market Data: Not included; traders must select and subscribe to data services independently" (S51)
- Third-party disclosure: "Trading platforms and data services made available through FFF, including NinjaTrader, Tradovate, TradingView, WealthCharts, and Rithmic, are owned and operated by independent third parties." (S52)


**Product impact (for the PO/Architect, not a rule value).** NinjaTrader is a published connection for FFF simulated accounts (via the NinjaTrader/Tradovate sign-in with FFF credentials), so the NinjaTrader 8 plugin can in principle reach these accounts. The blockers are policy, not technology: FFF forbids bots and "any automated order entry" (§6) and has a conflict on VPS (§7). The Live Stage uses Rithmic, which NinjaTrader also supports, but no connection guide for it is published.

## 3. Plans

All plans share these properties unless a row says otherwise: simulated account with live market data; MNQ and NQ tradable; micros count 10:1 with minis in every published limit; no DLL except Payday and the live stages; Evaluation fee is a recurring subscription, S2F is a one-time fee (S12); 90/10 profit split on sim funded payouts, 80/20 in the Professional Stage (S49); no activation fee (S53, S54).

### 3.1 Prime (EOD drawdown, S13-S18)

| Size | Phase | Profit target | Drawdown | DLL | Consistency | Max position (std / max) | Notes |
|---|---|---|---|---|---|---|---|
| 25K | evaluation | $1,250 | $1,000 trailing EOD | none | none | 2 / 3 minis | |
| 50K | evaluation | $3,000 | $2,000 trailing EOD | none | none | 4 / 5 | |
| 100K | evaluation | $6,000 | $3,000 trailing EOD | none | none | 6 / 10 | |
| 150K | evaluation | $9,000 | $4,500 trailing EOD | none | none | 10 / 15 | |
| 25K-150K | funded | none | same amounts | none | 40% (payout) | same as evaluation, plus scaling plan | payout every 3 trading days, $200/day |

- Drawdown (S15): "The Maximum Loss Limit trails upward as your account grows." (S15) / "After exceeding the Initial Trail Balance, the Max Drawdown locks and no longer moves." (S15) Initial Trailing Balance: $26,000 / $52,000 / $103,000 / $154,500; Locked Drawdown Balance equals the starting balance.
- Consistency (S16): "All Prime Funded Accounts are subject to a 40% Consistency Rule." (S16) Evaluation: "No consistency or DLL" (S13)
- Scaling plan (S18): "The Scaling Plan does not apply during the Evaluation phase." (S18) Funded limits start at 1/3/4/5 minis (25K/50K/100K/150K) and reach 3/5/10/15 minis at $2,000 / $2,000 / $3,000 / $4,500 simulated profit.

### 3.2 Premier+ (EOD or Intraday drawdown, accounts bought on or after 2026-09-09, S19-S24)

| Size | Profit target | Max DD EOD (evaluation and funded) | Max DD Intraday | Max contract (evaluation table) | Funded table / scaling plan max |
|---|---|---|---|---|---|
| 25K | $1,500 | $750 | $1,000 | 2 minis / 20 micros | 2 / 3 minis |
| 50K | $3,000 | $1,500 | $2,000 | 4 / 40 | 4 / 5 |
| 100K | $6,000 | $2,500 | $3,000 | 6 / 60 | 6 / 10 |
| 150K | $9,000 | $4,000 | $4,500 | 10 / 100 (EOD table prints "10 Micros/100 Minis") | 10 / 15 |

- Evaluation: no consistency, no DLL; "pass in as little as one trading day with the Fast Pass add on or two days with the standard option" (S19). Funded: 40% consistency for purchases on or after 2026-09-09 (S20, S22, S23), no DLL.
- Accounts purchased before 2026-09-09 (legacy tables on the same page): evaluation limits 3/5/10/15 minis. Funded consistency for them is only "original funded consistency terms", not published.
- Drawdown (S21): EOD "After exceeding the account base size, the Max Drawdown locks and no longer moves." (S21) Intraday "Monitored continuously in real time throughout the trading day." (S21) Intraday lock is not stated. After the first payout (S22): "once you request your first payout, your drawdown gets brought up to base account size." (S22)
- Giveaway accounts: minimum 7 trading days and 40% consistency (S19).

### 3.3 Velocity (intraday trailing, S25-S30)

| Size | Profit target | Max DD (intraday trailing) | Consistency | Max position | Min days |
|---|---|---|---|---|---|
| 25K | $2,500 | $1,250 | 40% lifetime | 3 minis / 30 micros | 3 |
| 50K | $4,000 | $2,250 | 40% | 5 / 50 | 3 |
| 100K | $7,000 | $3,250 | 40% | 10 / 100 | 3 |
| 150K | $10,000 | $4,750 | 40% | 15 / 150 | 3 |

- "40% consistency rule applies to lifetime of account" (S25) / "Requires a minimum of 3 trading days to pass" (S25) Funded: no DLL, same intraday trailing drawdown. Daily Payout Add-On: "No trading day requirement" (S25) and "No consistency rule" (S25).

### 3.4 Straight to Funded (S2F), no evaluation (S31-S35)

| Size | Max DD (EOD trailing) | DLL | Consistency | Max contract (table) | Payout needs |
|---|---|---|---|---|---|
| 25K | $1,000 | none | 25% (help) / 45% (home) | 1 mini / 10 micros (table) vs 3 minis scaling max | 7 days of $200+ |
| 50K | $2,000 | none | 25% / 45% | 5 / 50 | same |
| 100K | $3,000 | none | 25% / 45% | 10 / 100 | same |
| 150K | $4,500 | none | 25% / 45% | 15 / 150 | same |

- "All Straight to Funded(S2F) accounts are subject to a 25% Consistency Rule." (S33) The home page plan card reads "45% consistency" (S54, text `45% consistency`). Conflict recorded in §10.

### 3.5 Accelerate, Payday, Sniper (single-size S2F variants)

| Plan | Size | DD | DLL | Consistency | Max position | Payout target / notes |
|---|---|---|---|---|---|---|
| S2F Accelerate (invite-only per home card) | 50K | $2,000 intraday trailing | none | 25% lifetime (before 2026-10-22) or 20/25/30% (on/after) | 5 minis / 50 micros, no scaling | $4,000 per payout cycle (on/after 2026-10-22); 5 winning days of $200+ |
| S2F Accelerate | 25K | $1,000 intraday, "Never locks" | none | same | 3 minis / 30 micros | $2,000 per cycle; buffer $1,100; max payout $500 |
| S2F Payday | 50K | $2,000 EOD trailing, locks at $50,000 after $52,000 EOD | $1,000 hard | 40% | 5 / 50, scaling yes | $3,000 between payouts; 7 qualifying days |
| S2F Sniper (invite-only per home card) | 25K | $750 EOD, never locks | none | none | 1 mini / 10 micros, no scaling | $2,500 between payouts; daily payouts; news restricted |

- Sources S36-S40 (Accelerate), S46 (Payday), S47 (Sniper). Sniper news rule: "You must be flat from 1 minute before through 1 minute after the news event." (S47)

### 3.6 Base ($2K account, S41-S45)

- Evaluation: $2,000 account, profit target $3,000, loss limit $2,000 EOD trailing, 40% consistency, no DLL, 5 minis / 50 micros with balance-dependent limits (S41).
- Funded: "The account uses intraday trailing drawdown, which stops trailing once you reach $2,000 in profit and remains fixed from that point forward." (S42) No consistency, no DLL, payouts $500-$1,000 daily, buffer $4,000, max 15 minis / 150 micros with balance scaling (S42, S43, S45).

### 3.7 Prestige, Professional Stage and Live Stage (S48-S51)

| Stage | Sizes | Targets / drawdown | Notes |
|---|---|---|---|
| Prestige evaluation (invite-only) | 25K $2,000 / 50K $4,000 / 100K $6,000 / 150K $9,000 targets | DD $1,000 / $2,000 / $3,000 / $4,500; 40% consistency; min 5 days; limits 3/5/10/15 minis | drawdown type and DLL not published |
| Professional Stage (Pro) | 50K, 100K, 150K | End-of-Position drawdown, locks at $50,100 / $100,100 / $150,100; "Personalized Max Drawdown, assigned per account"; DLL "governed by the Scaling Plan" | contract limits case by case; no consistency; 80/20 |
| Live Stage (Rithmic) | 50K / 100K / 150K | DD $2,500 / $3,500 / $4,500; DLL $1,000 / $1,000 / $1,250; 1 mini or 10 micros | live market via Rithmic; data not included |

- "PSA uses an End-of-Position (EOP) drawdown model." (S49) Prestige page: "End of Day Drawdown model" (S48) for the Pro funded phase (conflict in §10).
- Pro/Live entry is not automatic: "Qualification is not automatic." (S49)

## 4. Trading-day reset

- **No explicit trading-day reset time or timezone is published** on any page found. EOD drawdown is calculated on "closing balance at the end of each trading session" (S15) without stating when a session ends. `time`/`tz_as_published` are `null` in all blocks (§10).
- **Market window actually published (S5):** "All trades must close by 4:45 pm EST, and you can reopen trades at 6:00 pm EST." (S5) Also "The trades will automatically close for you at 4:45 pm EST." (S5) The article says "EST" (not "ET"); the firm does not say whether it follows daylight saving time. This is a flatten/reopen window, not a trading-day definition.
- Holidays: no holiday rule published.
- Payout review cut-off (S56, unrelated to trading day): "Requests made before 5:00 p.m. ET on a business day are reviewed the same day." (S56)

## 5. Trade copying stance

Classification: **conditional** (own accounts only; ambiguity with the automation ban).

- Allowed: "Allowed: copying trades between your own accounts." (S3)
- Prohibited: "This includes signal groups that place the same trades at the same second, and trade copiers connected to someone else's account." (S3) Penalty: "Accounts closed, not paid".
- Hedging across own accounts is prohibited, including mini vs micro: "Holding a long position in one account and a short position in another at the same time, in the same market." (S3) / "a mini against its micro, for example long NQ in one account and short MNQ in another" (S3)
- Account-count limits: "5 Sim Funded Accounts per household." (S11) / "1 Live Funded Account per household." (S11) S2F and Accelerate pages: "Max Accounts Per User: 5 active funded accounts allowed simultaneously" (S32, S37). Evaluation account count is not limited by any page found. One user profile per person (S53): "Each individual is permitted to maintain only one user profile on our platform." (S53)
- Copying with other firms: not mentioned on any page found (§10).
- **Ambiguity:** the same article that allows copying between own accounts also lists "Bots, algorithms, expert advisors and any automated order entry are not allowed." (S3) A copier is automated order entry in the ordinary sense, and no page says whether a copier is exempt. No FFF-provided copier is mentioned either. Recorded as a gap.

## 6. Automated trading stance

**(i) Classification: forbidden** (all plans and phases; no page distinguishes by plan).

`automation_rule_text` (verbatim, shown in the opt-in warning):
- "At Funded Futures Family, we don’t permit bot or algorithmic trading." (S2)
- "Bots, algorithms, expert advisors and any automated order entry are not allowed." (S3) Consequence: "What happens: the account is closed." (S3)
- "Violating this policy may lead to account suspension or bans to protect the integrity of our trading environment." (S2)
- Micro-scalping policy: "We do not permit trading activity that relies on bots, algorithms, or any strategy that cannot be reliably executed in a true live trading setting." (S8)
- Detection indicators (S2): "We monitor for unusually high trade volumes or repetitive trading patterns that occur faster than human capability." (S2) / "We track the consistency of order timing." (S2)

**(ii) Named mechanisms:**
- **Semi-automated trading:** not mentioned (no page says what is allowed between manual and bot).
- **Trade copiers:** mentioned (S3): allowed between own accounts, prohibited when "connected to someone else's account". See ambiguity in §5.
- **Webhooks / alerts:** not mentioned. TradingView appears only as a chart front end connected through Tradovate (S1).
- **ATM / auto brackets:** not mentioned. Stop loss is mentioned only as irrelevant to the all-or-nothing measure: "Contract size, a stop loss, or how long the trade was open do not change it." (S9)
- **Other limits that bite automation:** high-frequency trading ("Example: 10 or more separate orders filled in the same second." (S3)); micro-scalping ("over 50% of your trades and over 50% of your total profits must come from positions held longer than 10 seconds." (S8)); and platform-delay trading.

**(iii) Differs by phase:** no page differentiates. Evaluation, simulated funded, Pro and Live are all under "any FFF account" (S3). The Live Stage (Rithmic) has no separate automation article, which is recorded as unknown for Live (§10).

## 7. Device / hosting rules and other rules

### Device / hosting rules (format rev. 3 §3 item 7)

**Conflict (two official pages).**
- Bots policy (S2, dated September 8, 2026): "VPNs and VPSs are permitted and may be used at the trader’s discretion." (S2)
- Prohibited Trading Conduct (S3, S3b, "Updated today"): "VPS, proxy and bot hosts are not allowed. A personal VPN is allowed." (S3) Consequence in its table: "Balance reverted; account restrictions or closure may also apply without prior warning." (S3)
- R4 (2026-10-23): both articles re-captured (S2 bots policy and S3 Prohibited Trading Conduct, a second copy of which exists at https://intercom.help/funded-futures-family/en/articles/17708968-prohibited-trading-conduct with the same sentence); texts unchanged. No choice is made between them: `vps_allowed: conflict` and a conflict entry in `not_confirmed_fields` in all blocks. The FFF terms page (https://www.fundedfuturesfamily.com/terms-and-conditions/) contains no VPS, IP or device text.
- **Personal device (R4, 2026-10-23): `personal_device_required: yes`.** Device Sharing article (S4, https://intercom.help/funded-futures-family/en/articles/17155892-device-sharing): "Device sharing is not permitted at Funded Futures Family. Each trader must use their own device to access and trade on their accounts." Prohibited Trading Conduct (S3): "Account sharing Anyone other than you trading your account or logging in to your dashboard or platform. This includes paid passing services, sharing a trading device with another trader, and buying in someone else's name." Scope of `yes`: the trader's own device, not shared with another trader; no page says the device must be a local (non-VPS) machine, and VPS permission is a separate conflict (above). "We use device fingerprints to identify devices associated with user profiles." (S4)
- Not an issue for a single machine: "Sharing Wi-Fi, an office network, or an IP address alone does not establish device sharing." (S4)
- WealthCharts: "For the best experience, avoid logging in on multiple devices at the same time." (S1)

### Other rules

- **News (S6, S3):** "News trading is permitted unless the specific rules for your account or plan state otherwise." (S6) S3 adds: "News trading: allowed, but straddles, strangles and news-only gambles are not." (S3) Sniper: restricted (hard breach).
- **Overnight:** "You are free to trade overnight or at any time during the trading day" (S5) but not during 4:45-6:00 pm EST (S5).
- **Hedging:** prohibited across accounts (§5).
- **Inactivity (S7):** "To keep your account active, you must place at least one trade per week (Monday–Friday) for a minimum of 10 seconds." (S7)
- **Micro-scalping (S8):** "over 50% of your trades and over 50% of your total profits must come from positions held longer than 10 seconds." (S8)
- **CME price limits:** "CME price limits: no trading within 2% of a contract's CME price limit." (S3)
- **All-or-nothing / account rolling (S9):** "All-or-nothing trade: one trade that wins or loses 80% or more of the account's loss limit." (S9) Repeated pattern may lead to closure of affected accounts.
- **Scaling plan:** updated at session end, "will not adjust in real time" (S18). Mandatory stop loss: none published. Minimum/maximum trade duration: only the 10-second thresholds above.
- **Resets (S12):** "Users are allowed unlimited resets for evaluation accounts. On funded accounts, users are allowed up to three resets max." (S12)
- **Restricted countries (S55):** a long list including Turkey, Russia, Vietnam and others; see the page.

## 8. Commissions

**Not published.** No official FFF page found states a commission per contract, exchange/NFA/platform fee, or fee schedule for MNQ or NQ. The simulated accounts publish only the evaluation subscription, reset fees and the S2F one-time fee (S12, S54 selector shows "$69.99 One time fee" for the 25K Sniper selection, "+$39" for Velocity add-ons). In every YAML block `commission_per_contract` is all `null`. Gap logged in §10. Live Stage commissions would be set by the Rithmic broker and are not published (S51).

## 9. Machine-readable blocks

One block per plan, phase and size. Common values in every block: `copy_trading_allowed: conditional`, `automated_trading_allowed: no` with `automation_rule_text` quoted in §6, `vps_allowed: conflict` (§7), `personal_device_required: unknown`, `trading_day_reset: null` (unpublished), `commission_per_contract: null` (unpublished), `status: conflict`.

#### Prime — evaluation — 25K

```yaml
firm: funded-futures-family
plan: "Prime"
phase: evaluation
size_label: "25K"
size_usd: "25000.00"
instruments: [MNQ, NQ]
platforms: ["Tradovate", "WealthCharts", "NinjaTrader (sign in with FFF credentials, Simulation)", "TradingView (via Tradovate add-on)"]
drawdown: { amount_usd: "1000.00", type: "trailing_eod", stops_trailing_at: "Locks when the balance exceeds the Initial Trail Balance (drawdown threshold reaches the starting balance). Published on the funded-account drawdown page; the evaluation page only says \"end-of-day drawdown\"" }
daily_loss_limit: { amount_usd: null, kind: "none" }
profit_target_usd: "1250.00"
consistency_rule: { text: "none (no consistency rule on the evaluation)", threshold_pct: null }
max_position: { minis: "2 standard / 3 max", micros: "20 standard / 30 max", scaling: "Scaling plan does not apply in the evaluation" }
min_trading_days: null
trading_day_reset: { time: null, tz_as_published: null }
commission_per_contract: { mnq_usd: null, nq_usd: null, basis: null, includes: null }
copy_trading_allowed: conditional
automated_trading_allowed: no
automation_rule_text: "At Funded Futures Family, we don’t permit bot or algorithmic trading. / Bots, algorithms, expert advisors and any automated order entry are not allowed."
vps_allowed: conflict
personal_device_required: yes
status: conflict
not_confirmed_fields: ["commission_per_contract", "trading_day_reset", "vps_allowed (conflict)", "copy_trading_allowed (ambiguity vs automation ban)", "drawdown.stops_trailing_at (evaluation lock not stated separately)", "max_position (Standard vs Max undefined)", "min_trading_days"]
notes: "Max position published as \"Standard/Max\" columns, meaning of the two figures not defined. Passing in \"as little as one trading day\" (no minimum-days rule stated). Seven days to activate the funded account after passing. No activation fee."
sources:
  - { field: platforms, url: "https://intercom.help/funded-futures-family/en/articles/15656536-connecting-your-trading-platform", accessed: "2026-10-23" }
  - { field: platforms, url: "https://www.fundedfuturesfamily.com/", accessed: "2026-10-23" }
  - { field: instruments, url: "https://intercom.help/funded-futures-family/en/articles/17708800-prohibited-trading-conduct", accessed: "2026-10-23" }
  - { field: automation, url: "https://intercom.help/funded-futures-family/en/articles/15892413-bots-algorithmic-trading-policy", accessed: "2026-10-23" }
  - { field: automation, url: "https://intercom.help/funded-futures-family/en/articles/17708800-prohibited-trading-conduct", accessed: "2026-10-23" }
  - { field: copy_trading, url: "https://intercom.help/funded-futures-family/en/articles/17708800-prohibited-trading-conduct", accessed: "2026-10-23" }
  - { field: vps_allowed, url: "https://intercom.help/funded-futures-family/en/articles/15892413-bots-algorithmic-trading-policy", accessed: "2026-10-23" }
  - { field: vps_allowed, url: "https://intercom.help/funded-futures-family/en/articles/17708800-prohibited-trading-conduct", accessed: "2026-10-23" }
  - { field: personal_device_required, url: "https://intercom.help/funded-futures-family/en/articles/17155892-device-sharing", accessed: "2026-10-23" }
  - { field: trading_day_reset, url: "https://intercom.help/funded-futures-family/en/articles/15892350-permitted-times-to-trade", accessed: "2026-10-23" }
  - { field: drawdown, url: "https://intercom.help/funded-futures-family/en/articles/15657092-prime-evaluation-account", accessed: "2026-10-23" }
  - { field: drawdown, url: "https://intercom.help/funded-futures-family/en/articles/15808767-prime-drawdown", accessed: "2026-10-23" }
  - { field: daily_loss_limit, url: "https://intercom.help/funded-futures-family/en/articles/15657092-prime-evaluation-account", accessed: "2026-10-23" }
  - { field: profit_target_usd, url: "https://intercom.help/funded-futures-family/en/articles/15657092-prime-evaluation-account", accessed: "2026-10-23" }
  - { field: consistency_rule, url: "https://intercom.help/funded-futures-family/en/articles/15657092-prime-evaluation-account", accessed: "2026-10-23" }
  - { field: max_position, url: "https://intercom.help/funded-futures-family/en/articles/15657092-prime-evaluation-account", accessed: "2026-10-23" }
  - { field: max_position, url: "https://intercom.help/funded-futures-family/en/articles/16302640-prime-scaling-plan", accessed: "2026-10-23" }
```

#### Prime — evaluation — 50K

```yaml
firm: funded-futures-family
plan: "Prime"
phase: evaluation
size_label: "50K"
size_usd: "50000.00"
instruments: [MNQ, NQ]
platforms: ["Tradovate", "WealthCharts", "NinjaTrader (sign in with FFF credentials, Simulation)", "TradingView (via Tradovate add-on)"]
drawdown: { amount_usd: "2000.00", type: "trailing_eod", stops_trailing_at: "Locks when the balance exceeds the Initial Trail Balance (drawdown threshold reaches the starting balance). Published on the funded-account drawdown page; the evaluation page only says \"end-of-day drawdown\"" }
daily_loss_limit: { amount_usd: null, kind: "none" }
profit_target_usd: "3000.00"
consistency_rule: { text: "none (no consistency rule on the evaluation)", threshold_pct: null }
max_position: { minis: "4 standard / 5 max", micros: "40 standard / 50 max", scaling: "Scaling plan does not apply in the evaluation" }
min_trading_days: null
trading_day_reset: { time: null, tz_as_published: null }
commission_per_contract: { mnq_usd: null, nq_usd: null, basis: null, includes: null }
copy_trading_allowed: conditional
automated_trading_allowed: no
automation_rule_text: "At Funded Futures Family, we don’t permit bot or algorithmic trading. / Bots, algorithms, expert advisors and any automated order entry are not allowed."
vps_allowed: conflict
personal_device_required: yes
status: conflict
not_confirmed_fields: ["commission_per_contract", "trading_day_reset", "vps_allowed (conflict)", "copy_trading_allowed (ambiguity vs automation ban)", "drawdown.stops_trailing_at (evaluation lock not stated separately)", "max_position (Standard vs Max undefined)", "min_trading_days"]
notes: "Max position published as \"Standard/Max\" columns, meaning of the two figures not defined. Passing in \"as little as one trading day\" (no minimum-days rule stated). Seven days to activate the funded account after passing. No activation fee."
sources:
  - { field: platforms, url: "https://intercom.help/funded-futures-family/en/articles/15656536-connecting-your-trading-platform", accessed: "2026-10-23" }
  - { field: platforms, url: "https://www.fundedfuturesfamily.com/", accessed: "2026-10-23" }
  - { field: instruments, url: "https://intercom.help/funded-futures-family/en/articles/17708800-prohibited-trading-conduct", accessed: "2026-10-23" }
  - { field: automation, url: "https://intercom.help/funded-futures-family/en/articles/15892413-bots-algorithmic-trading-policy", accessed: "2026-10-23" }
  - { field: automation, url: "https://intercom.help/funded-futures-family/en/articles/17708800-prohibited-trading-conduct", accessed: "2026-10-23" }
  - { field: copy_trading, url: "https://intercom.help/funded-futures-family/en/articles/17708800-prohibited-trading-conduct", accessed: "2026-10-23" }
  - { field: vps_allowed, url: "https://intercom.help/funded-futures-family/en/articles/15892413-bots-algorithmic-trading-policy", accessed: "2026-10-23" }
  - { field: vps_allowed, url: "https://intercom.help/funded-futures-family/en/articles/17708800-prohibited-trading-conduct", accessed: "2026-10-23" }
  - { field: personal_device_required, url: "https://intercom.help/funded-futures-family/en/articles/17155892-device-sharing", accessed: "2026-10-23" }
  - { field: trading_day_reset, url: "https://intercom.help/funded-futures-family/en/articles/15892350-permitted-times-to-trade", accessed: "2026-10-23" }
  - { field: drawdown, url: "https://intercom.help/funded-futures-family/en/articles/15657092-prime-evaluation-account", accessed: "2026-10-23" }
  - { field: drawdown, url: "https://intercom.help/funded-futures-family/en/articles/15808767-prime-drawdown", accessed: "2026-10-23" }
  - { field: daily_loss_limit, url: "https://intercom.help/funded-futures-family/en/articles/15657092-prime-evaluation-account", accessed: "2026-10-23" }
  - { field: profit_target_usd, url: "https://intercom.help/funded-futures-family/en/articles/15657092-prime-evaluation-account", accessed: "2026-10-23" }
  - { field: consistency_rule, url: "https://intercom.help/funded-futures-family/en/articles/15657092-prime-evaluation-account", accessed: "2026-10-23" }
  - { field: max_position, url: "https://intercom.help/funded-futures-family/en/articles/15657092-prime-evaluation-account", accessed: "2026-10-23" }
  - { field: max_position, url: "https://intercom.help/funded-futures-family/en/articles/16302640-prime-scaling-plan", accessed: "2026-10-23" }
```

#### Prime — evaluation — 100K

```yaml
firm: funded-futures-family
plan: "Prime"
phase: evaluation
size_label: "100K"
size_usd: "100000.00"
instruments: [MNQ, NQ]
platforms: ["Tradovate", "WealthCharts", "NinjaTrader (sign in with FFF credentials, Simulation)", "TradingView (via Tradovate add-on)"]
drawdown: { amount_usd: "3000.00", type: "trailing_eod", stops_trailing_at: "Locks when the balance exceeds the Initial Trail Balance (drawdown threshold reaches the starting balance). Published on the funded-account drawdown page; the evaluation page only says \"end-of-day drawdown\"" }
daily_loss_limit: { amount_usd: null, kind: "none" }
profit_target_usd: "6000.00"
consistency_rule: { text: "none (no consistency rule on the evaluation)", threshold_pct: null }
max_position: { minis: "6 standard / 10 max", micros: "60 standard / 100 max", scaling: "Scaling plan does not apply in the evaluation" }
min_trading_days: null
trading_day_reset: { time: null, tz_as_published: null }
commission_per_contract: { mnq_usd: null, nq_usd: null, basis: null, includes: null }
copy_trading_allowed: conditional
automated_trading_allowed: no
automation_rule_text: "At Funded Futures Family, we don’t permit bot or algorithmic trading. / Bots, algorithms, expert advisors and any automated order entry are not allowed."
vps_allowed: conflict
personal_device_required: yes
status: conflict
not_confirmed_fields: ["commission_per_contract", "trading_day_reset", "vps_allowed (conflict)", "copy_trading_allowed (ambiguity vs automation ban)", "drawdown.stops_trailing_at (evaluation lock not stated separately)", "max_position (Standard vs Max undefined)", "min_trading_days"]
notes: "Max position published as \"Standard/Max\" columns, meaning of the two figures not defined. Passing in \"as little as one trading day\" (no minimum-days rule stated). Seven days to activate the funded account after passing. No activation fee."
sources:
  - { field: platforms, url: "https://intercom.help/funded-futures-family/en/articles/15656536-connecting-your-trading-platform", accessed: "2026-10-23" }
  - { field: platforms, url: "https://www.fundedfuturesfamily.com/", accessed: "2026-10-23" }
  - { field: instruments, url: "https://intercom.help/funded-futures-family/en/articles/17708800-prohibited-trading-conduct", accessed: "2026-10-23" }
  - { field: automation, url: "https://intercom.help/funded-futures-family/en/articles/15892413-bots-algorithmic-trading-policy", accessed: "2026-10-23" }
  - { field: automation, url: "https://intercom.help/funded-futures-family/en/articles/17708800-prohibited-trading-conduct", accessed: "2026-10-23" }
  - { field: copy_trading, url: "https://intercom.help/funded-futures-family/en/articles/17708800-prohibited-trading-conduct", accessed: "2026-10-23" }
  - { field: vps_allowed, url: "https://intercom.help/funded-futures-family/en/articles/15892413-bots-algorithmic-trading-policy", accessed: "2026-10-23" }
  - { field: vps_allowed, url: "https://intercom.help/funded-futures-family/en/articles/17708800-prohibited-trading-conduct", accessed: "2026-10-23" }
  - { field: personal_device_required, url: "https://intercom.help/funded-futures-family/en/articles/17155892-device-sharing", accessed: "2026-10-23" }
  - { field: trading_day_reset, url: "https://intercom.help/funded-futures-family/en/articles/15892350-permitted-times-to-trade", accessed: "2026-10-23" }
  - { field: drawdown, url: "https://intercom.help/funded-futures-family/en/articles/15657092-prime-evaluation-account", accessed: "2026-10-23" }
  - { field: drawdown, url: "https://intercom.help/funded-futures-family/en/articles/15808767-prime-drawdown", accessed: "2026-10-23" }
  - { field: daily_loss_limit, url: "https://intercom.help/funded-futures-family/en/articles/15657092-prime-evaluation-account", accessed: "2026-10-23" }
  - { field: profit_target_usd, url: "https://intercom.help/funded-futures-family/en/articles/15657092-prime-evaluation-account", accessed: "2026-10-23" }
  - { field: consistency_rule, url: "https://intercom.help/funded-futures-family/en/articles/15657092-prime-evaluation-account", accessed: "2026-10-23" }
  - { field: max_position, url: "https://intercom.help/funded-futures-family/en/articles/15657092-prime-evaluation-account", accessed: "2026-10-23" }
  - { field: max_position, url: "https://intercom.help/funded-futures-family/en/articles/16302640-prime-scaling-plan", accessed: "2026-10-23" }
```

#### Prime — evaluation — 150K

```yaml
firm: funded-futures-family
plan: "Prime"
phase: evaluation
size_label: "150K"
size_usd: "150000.00"
instruments: [MNQ, NQ]
platforms: ["Tradovate", "WealthCharts", "NinjaTrader (sign in with FFF credentials, Simulation)", "TradingView (via Tradovate add-on)"]
drawdown: { amount_usd: "4500.00", type: "trailing_eod", stops_trailing_at: "Locks when the balance exceeds the Initial Trail Balance (drawdown threshold reaches the starting balance). Published on the funded-account drawdown page; the evaluation page only says \"end-of-day drawdown\"" }
daily_loss_limit: { amount_usd: null, kind: "none" }
profit_target_usd: "9000.00"
consistency_rule: { text: "none (no consistency rule on the evaluation)", threshold_pct: null }
max_position: { minis: "10 standard / 15 max", micros: "100 standard / 150 max", scaling: "Scaling plan does not apply in the evaluation" }
min_trading_days: null
trading_day_reset: { time: null, tz_as_published: null }
commission_per_contract: { mnq_usd: null, nq_usd: null, basis: null, includes: null }
copy_trading_allowed: conditional
automated_trading_allowed: no
automation_rule_text: "At Funded Futures Family, we don’t permit bot or algorithmic trading. / Bots, algorithms, expert advisors and any automated order entry are not allowed."
vps_allowed: conflict
personal_device_required: yes
status: conflict
not_confirmed_fields: ["commission_per_contract", "trading_day_reset", "vps_allowed (conflict)", "copy_trading_allowed (ambiguity vs automation ban)", "drawdown.stops_trailing_at (evaluation lock not stated separately)", "max_position (Standard vs Max undefined)", "min_trading_days"]
notes: "Max position published as \"Standard/Max\" columns, meaning of the two figures not defined. Passing in \"as little as one trading day\" (no minimum-days rule stated). Seven days to activate the funded account after passing. No activation fee."
sources:
  - { field: platforms, url: "https://intercom.help/funded-futures-family/en/articles/15656536-connecting-your-trading-platform", accessed: "2026-10-23" }
  - { field: platforms, url: "https://www.fundedfuturesfamily.com/", accessed: "2026-10-23" }
  - { field: instruments, url: "https://intercom.help/funded-futures-family/en/articles/17708800-prohibited-trading-conduct", accessed: "2026-10-23" }
  - { field: automation, url: "https://intercom.help/funded-futures-family/en/articles/15892413-bots-algorithmic-trading-policy", accessed: "2026-10-23" }
  - { field: automation, url: "https://intercom.help/funded-futures-family/en/articles/17708800-prohibited-trading-conduct", accessed: "2026-10-23" }
  - { field: copy_trading, url: "https://intercom.help/funded-futures-family/en/articles/17708800-prohibited-trading-conduct", accessed: "2026-10-23" }
  - { field: vps_allowed, url: "https://intercom.help/funded-futures-family/en/articles/15892413-bots-algorithmic-trading-policy", accessed: "2026-10-23" }
  - { field: vps_allowed, url: "https://intercom.help/funded-futures-family/en/articles/17708800-prohibited-trading-conduct", accessed: "2026-10-23" }
  - { field: personal_device_required, url: "https://intercom.help/funded-futures-family/en/articles/17155892-device-sharing", accessed: "2026-10-23" }
  - { field: trading_day_reset, url: "https://intercom.help/funded-futures-family/en/articles/15892350-permitted-times-to-trade", accessed: "2026-10-23" }
  - { field: drawdown, url: "https://intercom.help/funded-futures-family/en/articles/15657092-prime-evaluation-account", accessed: "2026-10-23" }
  - { field: drawdown, url: "https://intercom.help/funded-futures-family/en/articles/15808767-prime-drawdown", accessed: "2026-10-23" }
  - { field: daily_loss_limit, url: "https://intercom.help/funded-futures-family/en/articles/15657092-prime-evaluation-account", accessed: "2026-10-23" }
  - { field: profit_target_usd, url: "https://intercom.help/funded-futures-family/en/articles/15657092-prime-evaluation-account", accessed: "2026-10-23" }
  - { field: consistency_rule, url: "https://intercom.help/funded-futures-family/en/articles/15657092-prime-evaluation-account", accessed: "2026-10-23" }
  - { field: max_position, url: "https://intercom.help/funded-futures-family/en/articles/15657092-prime-evaluation-account", accessed: "2026-10-23" }
  - { field: max_position, url: "https://intercom.help/funded-futures-family/en/articles/16302640-prime-scaling-plan", accessed: "2026-10-23" }
```

#### Prime — funded — 25K

```yaml
firm: funded-futures-family
plan: "Prime"
phase: funded
size_label: "25K"
size_usd: "25000.00"
instruments: [MNQ, NQ]
platforms: ["Tradovate", "WealthCharts", "NinjaTrader (sign in with FFF credentials, Simulation)", "TradingView (via Tradovate add-on)"]
drawdown: { amount_usd: "1000.00", type: "trailing_eod", stops_trailing_at: "Locks at the starting balance once the balance exceeds the Initial Trail Balance (Locked Drawdown Balance = starting balance)" }
daily_loss_limit: { amount_usd: null, kind: "none" }
profit_target_usd: "none"
consistency_rule: { text: "Largest single trading day profit cannot exceed 40% of total profit when requesting a payout; resets after each approved payout", threshold_pct: 40 }
max_position: { minis: "3 max (2 standard)", micros: "30 max (20 standard)", scaling: "Funded: scaling plan, 1 mini/10 micros at $0-$999 profit rising to 3 minis/30 micros at $2,000+" }
min_trading_days: null
trading_day_reset: { time: null, tz_as_published: null }
commission_per_contract: { mnq_usd: null, nq_usd: null, basis: null, includes: null }
copy_trading_allowed: conditional
automated_trading_allowed: no
automation_rule_text: "At Funded Futures Family, we don’t permit bot or algorithmic trading. / Bots, algorithms, expert advisors and any automated order entry are not allowed."
vps_allowed: conflict
personal_device_required: yes
status: conflict
not_confirmed_fields: ["commission_per_contract", "trading_day_reset", "vps_allowed (conflict)", "copy_trading_allowed (ambiguity vs automation ban)", "min_trading_days"]
notes: "Payouts every 3 trading days, minimum $200 profit per trading day, 90/10 split, buffer = drawdown + $100. Drawdown does not reset after a payout. Max position published as Standard/Max; scaling plan limits are updated at the end of each session."
sources:
  - { field: platforms, url: "https://intercom.help/funded-futures-family/en/articles/15656536-connecting-your-trading-platform", accessed: "2026-10-23" }
  - { field: platforms, url: "https://www.fundedfuturesfamily.com/", accessed: "2026-10-23" }
  - { field: instruments, url: "https://intercom.help/funded-futures-family/en/articles/17708800-prohibited-trading-conduct", accessed: "2026-10-23" }
  - { field: automation, url: "https://intercom.help/funded-futures-family/en/articles/15892413-bots-algorithmic-trading-policy", accessed: "2026-10-23" }
  - { field: automation, url: "https://intercom.help/funded-futures-family/en/articles/17708800-prohibited-trading-conduct", accessed: "2026-10-23" }
  - { field: copy_trading, url: "https://intercom.help/funded-futures-family/en/articles/17708800-prohibited-trading-conduct", accessed: "2026-10-23" }
  - { field: vps_allowed, url: "https://intercom.help/funded-futures-family/en/articles/15892413-bots-algorithmic-trading-policy", accessed: "2026-10-23" }
  - { field: vps_allowed, url: "https://intercom.help/funded-futures-family/en/articles/17708800-prohibited-trading-conduct", accessed: "2026-10-23" }
  - { field: personal_device_required, url: "https://intercom.help/funded-futures-family/en/articles/17155892-device-sharing", accessed: "2026-10-23" }
  - { field: trading_day_reset, url: "https://intercom.help/funded-futures-family/en/articles/15892350-permitted-times-to-trade", accessed: "2026-10-23" }
  - { field: drawdown, url: "https://intercom.help/funded-futures-family/en/articles/15705476-prime-funded-account", accessed: "2026-10-23" }
  - { field: drawdown, url: "https://intercom.help/funded-futures-family/en/articles/15808767-prime-drawdown", accessed: "2026-10-23" }
  - { field: daily_loss_limit, url: "https://intercom.help/funded-futures-family/en/articles/15705476-prime-funded-account", accessed: "2026-10-23" }
  - { field: consistency_rule, url: "https://intercom.help/funded-futures-family/en/articles/15808673-prime-consistency", accessed: "2026-10-23" }
  - { field: consistency_rule, url: "https://intercom.help/funded-futures-family/en/articles/15715050-prime-payout-requirements", accessed: "2026-10-23" }
  - { field: max_position, url: "https://intercom.help/funded-futures-family/en/articles/15705476-prime-funded-account", accessed: "2026-10-23" }
  - { field: max_position, url: "https://intercom.help/funded-futures-family/en/articles/16302640-prime-scaling-plan", accessed: "2026-10-23" }
```

#### Prime — funded — 50K

```yaml
firm: funded-futures-family
plan: "Prime"
phase: funded
size_label: "50K"
size_usd: "50000.00"
instruments: [MNQ, NQ]
platforms: ["Tradovate", "WealthCharts", "NinjaTrader (sign in with FFF credentials, Simulation)", "TradingView (via Tradovate add-on)"]
drawdown: { amount_usd: "2000.00", type: "trailing_eod", stops_trailing_at: "Locks at the starting balance once the balance exceeds the Initial Trail Balance (Locked Drawdown Balance = starting balance)" }
daily_loss_limit: { amount_usd: null, kind: "none" }
profit_target_usd: "none"
consistency_rule: { text: "Largest single trading day profit cannot exceed 40% of total profit when requesting a payout; resets after each approved payout", threshold_pct: 40 }
max_position: { minis: "5 max (4 standard)", micros: "50 max (40 standard)", scaling: "Funded: scaling plan, 3 minis/30 micros at $0-$999 rising to 5 minis/50 micros at $2,000+" }
min_trading_days: null
trading_day_reset: { time: null, tz_as_published: null }
commission_per_contract: { mnq_usd: null, nq_usd: null, basis: null, includes: null }
copy_trading_allowed: conditional
automated_trading_allowed: no
automation_rule_text: "At Funded Futures Family, we don’t permit bot or algorithmic trading. / Bots, algorithms, expert advisors and any automated order entry are not allowed."
vps_allowed: conflict
personal_device_required: yes
status: conflict
not_confirmed_fields: ["commission_per_contract", "trading_day_reset", "vps_allowed (conflict)", "copy_trading_allowed (ambiguity vs automation ban)", "min_trading_days"]
notes: "Payouts every 3 trading days, minimum $200 profit per trading day, 90/10 split, buffer = drawdown + $100. Drawdown does not reset after a payout. Max position published as Standard/Max; scaling plan limits are updated at the end of each session."
sources:
  - { field: platforms, url: "https://intercom.help/funded-futures-family/en/articles/15656536-connecting-your-trading-platform", accessed: "2026-10-23" }
  - { field: platforms, url: "https://www.fundedfuturesfamily.com/", accessed: "2026-10-23" }
  - { field: instruments, url: "https://intercom.help/funded-futures-family/en/articles/17708800-prohibited-trading-conduct", accessed: "2026-10-23" }
  - { field: automation, url: "https://intercom.help/funded-futures-family/en/articles/15892413-bots-algorithmic-trading-policy", accessed: "2026-10-23" }
  - { field: automation, url: "https://intercom.help/funded-futures-family/en/articles/17708800-prohibited-trading-conduct", accessed: "2026-10-23" }
  - { field: copy_trading, url: "https://intercom.help/funded-futures-family/en/articles/17708800-prohibited-trading-conduct", accessed: "2026-10-23" }
  - { field: vps_allowed, url: "https://intercom.help/funded-futures-family/en/articles/15892413-bots-algorithmic-trading-policy", accessed: "2026-10-23" }
  - { field: vps_allowed, url: "https://intercom.help/funded-futures-family/en/articles/17708800-prohibited-trading-conduct", accessed: "2026-10-23" }
  - { field: personal_device_required, url: "https://intercom.help/funded-futures-family/en/articles/17155892-device-sharing", accessed: "2026-10-23" }
  - { field: trading_day_reset, url: "https://intercom.help/funded-futures-family/en/articles/15892350-permitted-times-to-trade", accessed: "2026-10-23" }
  - { field: drawdown, url: "https://intercom.help/funded-futures-family/en/articles/15705476-prime-funded-account", accessed: "2026-10-23" }
  - { field: drawdown, url: "https://intercom.help/funded-futures-family/en/articles/15808767-prime-drawdown", accessed: "2026-10-23" }
  - { field: daily_loss_limit, url: "https://intercom.help/funded-futures-family/en/articles/15705476-prime-funded-account", accessed: "2026-10-23" }
  - { field: consistency_rule, url: "https://intercom.help/funded-futures-family/en/articles/15808673-prime-consistency", accessed: "2026-10-23" }
  - { field: consistency_rule, url: "https://intercom.help/funded-futures-family/en/articles/15715050-prime-payout-requirements", accessed: "2026-10-23" }
  - { field: max_position, url: "https://intercom.help/funded-futures-family/en/articles/15705476-prime-funded-account", accessed: "2026-10-23" }
  - { field: max_position, url: "https://intercom.help/funded-futures-family/en/articles/16302640-prime-scaling-plan", accessed: "2026-10-23" }
```

#### Prime — funded — 100K

```yaml
firm: funded-futures-family
plan: "Prime"
phase: funded
size_label: "100K"
size_usd: "100000.00"
instruments: [MNQ, NQ]
platforms: ["Tradovate", "WealthCharts", "NinjaTrader (sign in with FFF credentials, Simulation)", "TradingView (via Tradovate add-on)"]
drawdown: { amount_usd: "3000.00", type: "trailing_eod", stops_trailing_at: "Locks at the starting balance once the balance exceeds the Initial Trail Balance (Locked Drawdown Balance = starting balance)" }
daily_loss_limit: { amount_usd: null, kind: "none" }
profit_target_usd: "none"
consistency_rule: { text: "Largest single trading day profit cannot exceed 40% of total profit when requesting a payout; resets after each approved payout", threshold_pct: 40 }
max_position: { minis: "10 max (6 standard)", micros: "100 max (60 standard)", scaling: "Funded: scaling plan, 4 minis/40 micros at $0-$999 rising to 10 minis/100 micros at $3,000+" }
min_trading_days: null
trading_day_reset: { time: null, tz_as_published: null }
commission_per_contract: { mnq_usd: null, nq_usd: null, basis: null, includes: null }
copy_trading_allowed: conditional
automated_trading_allowed: no
automation_rule_text: "At Funded Futures Family, we don’t permit bot or algorithmic trading. / Bots, algorithms, expert advisors and any automated order entry are not allowed."
vps_allowed: conflict
personal_device_required: yes
status: conflict
not_confirmed_fields: ["commission_per_contract", "trading_day_reset", "vps_allowed (conflict)", "copy_trading_allowed (ambiguity vs automation ban)", "min_trading_days"]
notes: "Payouts every 3 trading days, minimum $200 profit per trading day, 90/10 split, buffer = drawdown + $100. Drawdown does not reset after a payout. Max position published as Standard/Max; scaling plan limits are updated at the end of each session."
sources:
  - { field: platforms, url: "https://intercom.help/funded-futures-family/en/articles/15656536-connecting-your-trading-platform", accessed: "2026-10-23" }
  - { field: platforms, url: "https://www.fundedfuturesfamily.com/", accessed: "2026-10-23" }
  - { field: instruments, url: "https://intercom.help/funded-futures-family/en/articles/17708800-prohibited-trading-conduct", accessed: "2026-10-23" }
  - { field: automation, url: "https://intercom.help/funded-futures-family/en/articles/15892413-bots-algorithmic-trading-policy", accessed: "2026-10-23" }
  - { field: automation, url: "https://intercom.help/funded-futures-family/en/articles/17708800-prohibited-trading-conduct", accessed: "2026-10-23" }
  - { field: copy_trading, url: "https://intercom.help/funded-futures-family/en/articles/17708800-prohibited-trading-conduct", accessed: "2026-10-23" }
  - { field: vps_allowed, url: "https://intercom.help/funded-futures-family/en/articles/15892413-bots-algorithmic-trading-policy", accessed: "2026-10-23" }
  - { field: vps_allowed, url: "https://intercom.help/funded-futures-family/en/articles/17708800-prohibited-trading-conduct", accessed: "2026-10-23" }
  - { field: personal_device_required, url: "https://intercom.help/funded-futures-family/en/articles/17155892-device-sharing", accessed: "2026-10-23" }
  - { field: trading_day_reset, url: "https://intercom.help/funded-futures-family/en/articles/15892350-permitted-times-to-trade", accessed: "2026-10-23" }
  - { field: drawdown, url: "https://intercom.help/funded-futures-family/en/articles/15705476-prime-funded-account", accessed: "2026-10-23" }
  - { field: drawdown, url: "https://intercom.help/funded-futures-family/en/articles/15808767-prime-drawdown", accessed: "2026-10-23" }
  - { field: daily_loss_limit, url: "https://intercom.help/funded-futures-family/en/articles/15705476-prime-funded-account", accessed: "2026-10-23" }
  - { field: consistency_rule, url: "https://intercom.help/funded-futures-family/en/articles/15808673-prime-consistency", accessed: "2026-10-23" }
  - { field: consistency_rule, url: "https://intercom.help/funded-futures-family/en/articles/15715050-prime-payout-requirements", accessed: "2026-10-23" }
  - { field: max_position, url: "https://intercom.help/funded-futures-family/en/articles/15705476-prime-funded-account", accessed: "2026-10-23" }
  - { field: max_position, url: "https://intercom.help/funded-futures-family/en/articles/16302640-prime-scaling-plan", accessed: "2026-10-23" }
```

#### Prime — funded — 150K

```yaml
firm: funded-futures-family
plan: "Prime"
phase: funded
size_label: "150K"
size_usd: "150000.00"
instruments: [MNQ, NQ]
platforms: ["Tradovate", "WealthCharts", "NinjaTrader (sign in with FFF credentials, Simulation)", "TradingView (via Tradovate add-on)"]
drawdown: { amount_usd: "4500.00", type: "trailing_eod", stops_trailing_at: "Locks at the starting balance once the balance exceeds the Initial Trail Balance (Locked Drawdown Balance = starting balance)" }
daily_loss_limit: { amount_usd: null, kind: "none" }
profit_target_usd: "none"
consistency_rule: { text: "Largest single trading day profit cannot exceed 40% of total profit when requesting a payout; resets after each approved payout", threshold_pct: 40 }
max_position: { minis: "15 max (10 standard)", micros: "150 max (100 standard)", scaling: "Funded: scaling plan, 5 minis/50 micros at $0-$999 rising to 15 minis/150 micros at $4,500+" }
min_trading_days: null
trading_day_reset: { time: null, tz_as_published: null }
commission_per_contract: { mnq_usd: null, nq_usd: null, basis: null, includes: null }
copy_trading_allowed: conditional
automated_trading_allowed: no
automation_rule_text: "At Funded Futures Family, we don’t permit bot or algorithmic trading. / Bots, algorithms, expert advisors and any automated order entry are not allowed."
vps_allowed: conflict
personal_device_required: yes
status: conflict
not_confirmed_fields: ["commission_per_contract", "trading_day_reset", "vps_allowed (conflict)", "copy_trading_allowed (ambiguity vs automation ban)", "min_trading_days"]
notes: "Payouts every 3 trading days, minimum $200 profit per trading day, 90/10 split, buffer = drawdown + $100. Drawdown does not reset after a payout. Max position published as Standard/Max; scaling plan limits are updated at the end of each session."
sources:
  - { field: platforms, url: "https://intercom.help/funded-futures-family/en/articles/15656536-connecting-your-trading-platform", accessed: "2026-10-23" }
  - { field: platforms, url: "https://www.fundedfuturesfamily.com/", accessed: "2026-10-23" }
  - { field: instruments, url: "https://intercom.help/funded-futures-family/en/articles/17708800-prohibited-trading-conduct", accessed: "2026-10-23" }
  - { field: automation, url: "https://intercom.help/funded-futures-family/en/articles/15892413-bots-algorithmic-trading-policy", accessed: "2026-10-23" }
  - { field: automation, url: "https://intercom.help/funded-futures-family/en/articles/17708800-prohibited-trading-conduct", accessed: "2026-10-23" }
  - { field: copy_trading, url: "https://intercom.help/funded-futures-family/en/articles/17708800-prohibited-trading-conduct", accessed: "2026-10-23" }
  - { field: vps_allowed, url: "https://intercom.help/funded-futures-family/en/articles/15892413-bots-algorithmic-trading-policy", accessed: "2026-10-23" }
  - { field: vps_allowed, url: "https://intercom.help/funded-futures-family/en/articles/17708800-prohibited-trading-conduct", accessed: "2026-10-23" }
  - { field: personal_device_required, url: "https://intercom.help/funded-futures-family/en/articles/17155892-device-sharing", accessed: "2026-10-23" }
  - { field: trading_day_reset, url: "https://intercom.help/funded-futures-family/en/articles/15892350-permitted-times-to-trade", accessed: "2026-10-23" }
  - { field: drawdown, url: "https://intercom.help/funded-futures-family/en/articles/15705476-prime-funded-account", accessed: "2026-10-23" }
  - { field: drawdown, url: "https://intercom.help/funded-futures-family/en/articles/15808767-prime-drawdown", accessed: "2026-10-23" }
  - { field: daily_loss_limit, url: "https://intercom.help/funded-futures-family/en/articles/15705476-prime-funded-account", accessed: "2026-10-23" }
  - { field: consistency_rule, url: "https://intercom.help/funded-futures-family/en/articles/15808673-prime-consistency", accessed: "2026-10-23" }
  - { field: consistency_rule, url: "https://intercom.help/funded-futures-family/en/articles/15715050-prime-payout-requirements", accessed: "2026-10-23" }
  - { field: max_position, url: "https://intercom.help/funded-futures-family/en/articles/15705476-prime-funded-account", accessed: "2026-10-23" }
  - { field: max_position, url: "https://intercom.help/funded-futures-family/en/articles/16302640-prime-scaling-plan", accessed: "2026-10-23" }
```

#### Premier+ (EOD drawdown) — evaluation — 25K

```yaml
firm: funded-futures-family
plan: "Premier+ (EOD drawdown)"
phase: evaluation
size_label: "25K"
size_usd: "25000.00"
instruments: [MNQ, NQ]
platforms: ["Tradovate", "WealthCharts", "NinjaTrader (sign in with FFF credentials, Simulation)", "TradingView (via Tradovate add-on)"]
drawdown: { amount_usd: "750.00", type: "trailing_eod", stops_trailing_at: "Locks when it reaches the base account balance" }
daily_loss_limit: { amount_usd: null, kind: "none" }
profit_target_usd: "1500.00"
consistency_rule: { text: "none (no consistency rule on the evaluation)", threshold_pct: null }
max_position: { minis: "2", micros: "20", scaling: "Scaling plan does not apply in the evaluation" }
min_trading_days: "2"
trading_day_reset: { time: null, tz_as_published: null }
commission_per_contract: { mnq_usd: null, nq_usd: null, basis: null, includes: null }
copy_trading_allowed: conditional
automated_trading_allowed: no
automation_rule_text: "At Funded Futures Family, we don’t permit bot or algorithmic trading. / Bots, algorithms, expert advisors and any automated order entry are not allowed."
vps_allowed: conflict
personal_device_required: yes
status: conflict
not_confirmed_fields: ["commission_per_contract", "trading_day_reset", "vps_allowed (conflict)", "copy_trading_allowed (ambiguity vs automation ban)"]
notes: "Accounts purchased on or after 2026-09-09 (current table). Standard option passes in two days, Fast Pass add-on in one day. Seven days to activate the funded account. No activation fee."
sources:
  - { field: platforms, url: "https://intercom.help/funded-futures-family/en/articles/15656536-connecting-your-trading-platform", accessed: "2026-10-23" }
  - { field: platforms, url: "https://www.fundedfuturesfamily.com/", accessed: "2026-10-23" }
  - { field: instruments, url: "https://intercom.help/funded-futures-family/en/articles/17708800-prohibited-trading-conduct", accessed: "2026-10-23" }
  - { field: automation, url: "https://intercom.help/funded-futures-family/en/articles/15892413-bots-algorithmic-trading-policy", accessed: "2026-10-23" }
  - { field: automation, url: "https://intercom.help/funded-futures-family/en/articles/17708800-prohibited-trading-conduct", accessed: "2026-10-23" }
  - { field: copy_trading, url: "https://intercom.help/funded-futures-family/en/articles/17708800-prohibited-trading-conduct", accessed: "2026-10-23" }
  - { field: vps_allowed, url: "https://intercom.help/funded-futures-family/en/articles/15892413-bots-algorithmic-trading-policy", accessed: "2026-10-23" }
  - { field: vps_allowed, url: "https://intercom.help/funded-futures-family/en/articles/17708800-prohibited-trading-conduct", accessed: "2026-10-23" }
  - { field: personal_device_required, url: "https://intercom.help/funded-futures-family/en/articles/17155892-device-sharing", accessed: "2026-10-23" }
  - { field: trading_day_reset, url: "https://intercom.help/funded-futures-family/en/articles/15892350-permitted-times-to-trade", accessed: "2026-10-23" }
  - { field: drawdown, url: "https://intercom.help/funded-futures-family/en/articles/15809232-premier-evaluation", accessed: "2026-10-23" }
  - { field: drawdown, url: "https://intercom.help/funded-futures-family/en/articles/15850031-premier-drawdown", accessed: "2026-10-23" }
  - { field: daily_loss_limit, url: "https://intercom.help/funded-futures-family/en/articles/15809232-premier-evaluation", accessed: "2026-10-23" }
  - { field: profit_target_usd, url: "https://intercom.help/funded-futures-family/en/articles/15809232-premier-evaluation", accessed: "2026-10-23" }
  - { field: consistency_rule, url: "https://intercom.help/funded-futures-family/en/articles/15809232-premier-evaluation", accessed: "2026-10-23" }
  - { field: max_position, url: "https://intercom.help/funded-futures-family/en/articles/15809232-premier-evaluation", accessed: "2026-10-23" }
  - { field: min_trading_days, url: "https://intercom.help/funded-futures-family/en/articles/15809232-premier-evaluation", accessed: "2026-10-23" }
```

#### Premier+ (EOD drawdown) — evaluation — 50K

```yaml
firm: funded-futures-family
plan: "Premier+ (EOD drawdown)"
phase: evaluation
size_label: "50K"
size_usd: "50000.00"
instruments: [MNQ, NQ]
platforms: ["Tradovate", "WealthCharts", "NinjaTrader (sign in with FFF credentials, Simulation)", "TradingView (via Tradovate add-on)"]
drawdown: { amount_usd: "1500.00", type: "trailing_eod", stops_trailing_at: "Locks when it reaches the base account balance" }
daily_loss_limit: { amount_usd: null, kind: "none" }
profit_target_usd: "3000.00"
consistency_rule: { text: "none (no consistency rule on the evaluation)", threshold_pct: null }
max_position: { minis: "4", micros: "40", scaling: "Scaling plan does not apply in the evaluation" }
min_trading_days: "2"
trading_day_reset: { time: null, tz_as_published: null }
commission_per_contract: { mnq_usd: null, nq_usd: null, basis: null, includes: null }
copy_trading_allowed: conditional
automated_trading_allowed: no
automation_rule_text: "At Funded Futures Family, we don’t permit bot or algorithmic trading. / Bots, algorithms, expert advisors and any automated order entry are not allowed."
vps_allowed: conflict
personal_device_required: yes
status: conflict
not_confirmed_fields: ["commission_per_contract", "trading_day_reset", "vps_allowed (conflict)", "copy_trading_allowed (ambiguity vs automation ban)"]
notes: "Accounts purchased on or after 2026-09-09 (current table). Standard option passes in two days, Fast Pass add-on in one day. Seven days to activate the funded account. No activation fee."
sources:
  - { field: platforms, url: "https://intercom.help/funded-futures-family/en/articles/15656536-connecting-your-trading-platform", accessed: "2026-10-23" }
  - { field: platforms, url: "https://www.fundedfuturesfamily.com/", accessed: "2026-10-23" }
  - { field: instruments, url: "https://intercom.help/funded-futures-family/en/articles/17708800-prohibited-trading-conduct", accessed: "2026-10-23" }
  - { field: automation, url: "https://intercom.help/funded-futures-family/en/articles/15892413-bots-algorithmic-trading-policy", accessed: "2026-10-23" }
  - { field: automation, url: "https://intercom.help/funded-futures-family/en/articles/17708800-prohibited-trading-conduct", accessed: "2026-10-23" }
  - { field: copy_trading, url: "https://intercom.help/funded-futures-family/en/articles/17708800-prohibited-trading-conduct", accessed: "2026-10-23" }
  - { field: vps_allowed, url: "https://intercom.help/funded-futures-family/en/articles/15892413-bots-algorithmic-trading-policy", accessed: "2026-10-23" }
  - { field: vps_allowed, url: "https://intercom.help/funded-futures-family/en/articles/17708800-prohibited-trading-conduct", accessed: "2026-10-23" }
  - { field: personal_device_required, url: "https://intercom.help/funded-futures-family/en/articles/17155892-device-sharing", accessed: "2026-10-23" }
  - { field: trading_day_reset, url: "https://intercom.help/funded-futures-family/en/articles/15892350-permitted-times-to-trade", accessed: "2026-10-23" }
  - { field: drawdown, url: "https://intercom.help/funded-futures-family/en/articles/15809232-premier-evaluation", accessed: "2026-10-23" }
  - { field: drawdown, url: "https://intercom.help/funded-futures-family/en/articles/15850031-premier-drawdown", accessed: "2026-10-23" }
  - { field: daily_loss_limit, url: "https://intercom.help/funded-futures-family/en/articles/15809232-premier-evaluation", accessed: "2026-10-23" }
  - { field: profit_target_usd, url: "https://intercom.help/funded-futures-family/en/articles/15809232-premier-evaluation", accessed: "2026-10-23" }
  - { field: consistency_rule, url: "https://intercom.help/funded-futures-family/en/articles/15809232-premier-evaluation", accessed: "2026-10-23" }
  - { field: max_position, url: "https://intercom.help/funded-futures-family/en/articles/15809232-premier-evaluation", accessed: "2026-10-23" }
  - { field: min_trading_days, url: "https://intercom.help/funded-futures-family/en/articles/15809232-premier-evaluation", accessed: "2026-10-23" }
```

#### Premier+ (EOD drawdown) — evaluation — 100K

```yaml
firm: funded-futures-family
plan: "Premier+ (EOD drawdown)"
phase: evaluation
size_label: "100K"
size_usd: "100000.00"
instruments: [MNQ, NQ]
platforms: ["Tradovate", "WealthCharts", "NinjaTrader (sign in with FFF credentials, Simulation)", "TradingView (via Tradovate add-on)"]
drawdown: { amount_usd: "2500.00", type: "trailing_eod", stops_trailing_at: "Locks when it reaches the base account balance" }
daily_loss_limit: { amount_usd: null, kind: "none" }
profit_target_usd: "6000.00"
consistency_rule: { text: "none (no consistency rule on the evaluation)", threshold_pct: null }
max_position: { minis: "6", micros: "60", scaling: "Scaling plan does not apply in the evaluation" }
min_trading_days: "2"
trading_day_reset: { time: null, tz_as_published: null }
commission_per_contract: { mnq_usd: null, nq_usd: null, basis: null, includes: null }
copy_trading_allowed: conditional
automated_trading_allowed: no
automation_rule_text: "At Funded Futures Family, we don’t permit bot or algorithmic trading. / Bots, algorithms, expert advisors and any automated order entry are not allowed."
vps_allowed: conflict
personal_device_required: yes
status: conflict
not_confirmed_fields: ["commission_per_contract", "trading_day_reset", "vps_allowed (conflict)", "copy_trading_allowed (ambiguity vs automation ban)"]
notes: "Accounts purchased on or after 2026-09-09 (current table). Standard option passes in two days, Fast Pass add-on in one day. Seven days to activate the funded account. No activation fee."
sources:
  - { field: platforms, url: "https://intercom.help/funded-futures-family/en/articles/15656536-connecting-your-trading-platform", accessed: "2026-10-23" }
  - { field: platforms, url: "https://www.fundedfuturesfamily.com/", accessed: "2026-10-23" }
  - { field: instruments, url: "https://intercom.help/funded-futures-family/en/articles/17708800-prohibited-trading-conduct", accessed: "2026-10-23" }
  - { field: automation, url: "https://intercom.help/funded-futures-family/en/articles/15892413-bots-algorithmic-trading-policy", accessed: "2026-10-23" }
  - { field: automation, url: "https://intercom.help/funded-futures-family/en/articles/17708800-prohibited-trading-conduct", accessed: "2026-10-23" }
  - { field: copy_trading, url: "https://intercom.help/funded-futures-family/en/articles/17708800-prohibited-trading-conduct", accessed: "2026-10-23" }
  - { field: vps_allowed, url: "https://intercom.help/funded-futures-family/en/articles/15892413-bots-algorithmic-trading-policy", accessed: "2026-10-23" }
  - { field: vps_allowed, url: "https://intercom.help/funded-futures-family/en/articles/17708800-prohibited-trading-conduct", accessed: "2026-10-23" }
  - { field: personal_device_required, url: "https://intercom.help/funded-futures-family/en/articles/17155892-device-sharing", accessed: "2026-10-23" }
  - { field: trading_day_reset, url: "https://intercom.help/funded-futures-family/en/articles/15892350-permitted-times-to-trade", accessed: "2026-10-23" }
  - { field: drawdown, url: "https://intercom.help/funded-futures-family/en/articles/15809232-premier-evaluation", accessed: "2026-10-23" }
  - { field: drawdown, url: "https://intercom.help/funded-futures-family/en/articles/15850031-premier-drawdown", accessed: "2026-10-23" }
  - { field: daily_loss_limit, url: "https://intercom.help/funded-futures-family/en/articles/15809232-premier-evaluation", accessed: "2026-10-23" }
  - { field: profit_target_usd, url: "https://intercom.help/funded-futures-family/en/articles/15809232-premier-evaluation", accessed: "2026-10-23" }
  - { field: consistency_rule, url: "https://intercom.help/funded-futures-family/en/articles/15809232-premier-evaluation", accessed: "2026-10-23" }
  - { field: max_position, url: "https://intercom.help/funded-futures-family/en/articles/15809232-premier-evaluation", accessed: "2026-10-23" }
  - { field: min_trading_days, url: "https://intercom.help/funded-futures-family/en/articles/15809232-premier-evaluation", accessed: "2026-10-23" }
```

#### Premier+ (EOD drawdown) — evaluation — 150K

```yaml
firm: funded-futures-family
plan: "Premier+ (EOD drawdown)"
phase: evaluation
size_label: "150K"
size_usd: "150000.00"
instruments: [MNQ, NQ]
platforms: ["Tradovate", "WealthCharts", "NinjaTrader (sign in with FFF credentials, Simulation)", "TradingView (via Tradovate add-on)"]
drawdown: { amount_usd: "4000.00", type: "trailing_eod", stops_trailing_at: "Locks when it reaches the base account balance" }
daily_loss_limit: { amount_usd: null, kind: "none" }
profit_target_usd: "9000.00"
consistency_rule: { text: "none (no consistency rule on the evaluation)", threshold_pct: null }
max_position: { minis: "10", micros: "100", scaling: "Scaling plan does not apply in the evaluation" }
min_trading_days: "2"
trading_day_reset: { time: null, tz_as_published: null }
commission_per_contract: { mnq_usd: null, nq_usd: null, basis: null, includes: null }
copy_trading_allowed: conditional
automated_trading_allowed: no
automation_rule_text: "At Funded Futures Family, we don’t permit bot or algorithmic trading. / Bots, algorithms, expert advisors and any automated order entry are not allowed."
vps_allowed: conflict
personal_device_required: yes
status: conflict
not_confirmed_fields: ["commission_per_contract", "trading_day_reset", "vps_allowed (conflict)", "copy_trading_allowed (ambiguity vs automation ban)", "max_position (150K EOD table inverted)"]
notes: "Accounts purchased on or after 2026-09-09 (current table). Standard option passes in two days, Fast Pass add-on in one day. Seven days to activate the funded account. No activation fee. The EOD table prints \"10 Micros/100 Minis\" for 150K (order inverted); the Intraday table and the funded page say 10 minis/100 micros. Treated as a typo but not confirmed."
sources:
  - { field: platforms, url: "https://intercom.help/funded-futures-family/en/articles/15656536-connecting-your-trading-platform", accessed: "2026-10-23" }
  - { field: platforms, url: "https://www.fundedfuturesfamily.com/", accessed: "2026-10-23" }
  - { field: instruments, url: "https://intercom.help/funded-futures-family/en/articles/17708800-prohibited-trading-conduct", accessed: "2026-10-23" }
  - { field: automation, url: "https://intercom.help/funded-futures-family/en/articles/15892413-bots-algorithmic-trading-policy", accessed: "2026-10-23" }
  - { field: automation, url: "https://intercom.help/funded-futures-family/en/articles/17708800-prohibited-trading-conduct", accessed: "2026-10-23" }
  - { field: copy_trading, url: "https://intercom.help/funded-futures-family/en/articles/17708800-prohibited-trading-conduct", accessed: "2026-10-23" }
  - { field: vps_allowed, url: "https://intercom.help/funded-futures-family/en/articles/15892413-bots-algorithmic-trading-policy", accessed: "2026-10-23" }
  - { field: vps_allowed, url: "https://intercom.help/funded-futures-family/en/articles/17708800-prohibited-trading-conduct", accessed: "2026-10-23" }
  - { field: personal_device_required, url: "https://intercom.help/funded-futures-family/en/articles/17155892-device-sharing", accessed: "2026-10-23" }
  - { field: trading_day_reset, url: "https://intercom.help/funded-futures-family/en/articles/15892350-permitted-times-to-trade", accessed: "2026-10-23" }
  - { field: drawdown, url: "https://intercom.help/funded-futures-family/en/articles/15809232-premier-evaluation", accessed: "2026-10-23" }
  - { field: drawdown, url: "https://intercom.help/funded-futures-family/en/articles/15850031-premier-drawdown", accessed: "2026-10-23" }
  - { field: daily_loss_limit, url: "https://intercom.help/funded-futures-family/en/articles/15809232-premier-evaluation", accessed: "2026-10-23" }
  - { field: profit_target_usd, url: "https://intercom.help/funded-futures-family/en/articles/15809232-premier-evaluation", accessed: "2026-10-23" }
  - { field: consistency_rule, url: "https://intercom.help/funded-futures-family/en/articles/15809232-premier-evaluation", accessed: "2026-10-23" }
  - { field: max_position, url: "https://intercom.help/funded-futures-family/en/articles/15809232-premier-evaluation", accessed: "2026-10-23" }
  - { field: min_trading_days, url: "https://intercom.help/funded-futures-family/en/articles/15809232-premier-evaluation", accessed: "2026-10-23" }
```

#### Premier+ (Intraday drawdown) — evaluation — 25K

```yaml
firm: funded-futures-family
plan: "Premier+ (Intraday drawdown)"
phase: evaluation
size_label: "25K"
size_usd: "25000.00"
instruments: [MNQ, NQ]
platforms: ["Tradovate", "WealthCharts", "NinjaTrader (sign in with FFF credentials, Simulation)", "TradingView (via Tradovate add-on)"]
drawdown: { amount_usd: "1000.00", type: "trailing_intraday", stops_trailing_at: "Not stated for the evaluation; page says for Intraday \"the drawdown continues to trail the account balance as the account grows\"" }
daily_loss_limit: { amount_usd: null, kind: "none" }
profit_target_usd: "1500.00"
consistency_rule: { text: "none (no consistency rule on the evaluation)", threshold_pct: null }
max_position: { minis: "2", micros: "20", scaling: "Scaling plan does not apply in the evaluation" }
min_trading_days: "2"
trading_day_reset: { time: null, tz_as_published: null }
commission_per_contract: { mnq_usd: null, nq_usd: null, basis: null, includes: null }
copy_trading_allowed: conditional
automated_trading_allowed: no
automation_rule_text: "At Funded Futures Family, we don’t permit bot or algorithmic trading. / Bots, algorithms, expert advisors and any automated order entry are not allowed."
vps_allowed: conflict
personal_device_required: yes
status: conflict
not_confirmed_fields: ["commission_per_contract", "trading_day_reset", "vps_allowed (conflict)", "copy_trading_allowed (ambiguity vs automation ban)", "drawdown.stops_trailing_at"]
notes: "Accounts purchased on or after 2026-09-09 (current table). Standard option passes in two days, Fast Pass add-on in one day. Seven days to activate the funded account. No activation fee."
sources:
  - { field: platforms, url: "https://intercom.help/funded-futures-family/en/articles/15656536-connecting-your-trading-platform", accessed: "2026-10-23" }
  - { field: platforms, url: "https://www.fundedfuturesfamily.com/", accessed: "2026-10-23" }
  - { field: instruments, url: "https://intercom.help/funded-futures-family/en/articles/17708800-prohibited-trading-conduct", accessed: "2026-10-23" }
  - { field: automation, url: "https://intercom.help/funded-futures-family/en/articles/15892413-bots-algorithmic-trading-policy", accessed: "2026-10-23" }
  - { field: automation, url: "https://intercom.help/funded-futures-family/en/articles/17708800-prohibited-trading-conduct", accessed: "2026-10-23" }
  - { field: copy_trading, url: "https://intercom.help/funded-futures-family/en/articles/17708800-prohibited-trading-conduct", accessed: "2026-10-23" }
  - { field: vps_allowed, url: "https://intercom.help/funded-futures-family/en/articles/15892413-bots-algorithmic-trading-policy", accessed: "2026-10-23" }
  - { field: vps_allowed, url: "https://intercom.help/funded-futures-family/en/articles/17708800-prohibited-trading-conduct", accessed: "2026-10-23" }
  - { field: personal_device_required, url: "https://intercom.help/funded-futures-family/en/articles/17155892-device-sharing", accessed: "2026-10-23" }
  - { field: trading_day_reset, url: "https://intercom.help/funded-futures-family/en/articles/15892350-permitted-times-to-trade", accessed: "2026-10-23" }
  - { field: drawdown, url: "https://intercom.help/funded-futures-family/en/articles/15809232-premier-evaluation", accessed: "2026-10-23" }
  - { field: drawdown, url: "https://intercom.help/funded-futures-family/en/articles/15850031-premier-drawdown", accessed: "2026-10-23" }
  - { field: daily_loss_limit, url: "https://intercom.help/funded-futures-family/en/articles/15809232-premier-evaluation", accessed: "2026-10-23" }
  - { field: profit_target_usd, url: "https://intercom.help/funded-futures-family/en/articles/15809232-premier-evaluation", accessed: "2026-10-23" }
  - { field: consistency_rule, url: "https://intercom.help/funded-futures-family/en/articles/15809232-premier-evaluation", accessed: "2026-10-23" }
  - { field: max_position, url: "https://intercom.help/funded-futures-family/en/articles/15809232-premier-evaluation", accessed: "2026-10-23" }
  - { field: min_trading_days, url: "https://intercom.help/funded-futures-family/en/articles/15809232-premier-evaluation", accessed: "2026-10-23" }
```

#### Premier+ (Intraday drawdown) — evaluation — 50K

```yaml
firm: funded-futures-family
plan: "Premier+ (Intraday drawdown)"
phase: evaluation
size_label: "50K"
size_usd: "50000.00"
instruments: [MNQ, NQ]
platforms: ["Tradovate", "WealthCharts", "NinjaTrader (sign in with FFF credentials, Simulation)", "TradingView (via Tradovate add-on)"]
drawdown: { amount_usd: "2000.00", type: "trailing_intraday", stops_trailing_at: "Not stated for the evaluation; page says for Intraday \"the drawdown continues to trail the account balance as the account grows\"" }
daily_loss_limit: { amount_usd: null, kind: "none" }
profit_target_usd: "3000.00"
consistency_rule: { text: "none (no consistency rule on the evaluation)", threshold_pct: null }
max_position: { minis: "4", micros: "40", scaling: "Scaling plan does not apply in the evaluation" }
min_trading_days: "2"
trading_day_reset: { time: null, tz_as_published: null }
commission_per_contract: { mnq_usd: null, nq_usd: null, basis: null, includes: null }
copy_trading_allowed: conditional
automated_trading_allowed: no
automation_rule_text: "At Funded Futures Family, we don’t permit bot or algorithmic trading. / Bots, algorithms, expert advisors and any automated order entry are not allowed."
vps_allowed: conflict
personal_device_required: yes
status: conflict
not_confirmed_fields: ["commission_per_contract", "trading_day_reset", "vps_allowed (conflict)", "copy_trading_allowed (ambiguity vs automation ban)", "drawdown.stops_trailing_at"]
notes: "Accounts purchased on or after 2026-09-09 (current table). Standard option passes in two days, Fast Pass add-on in one day. Seven days to activate the funded account. No activation fee."
sources:
  - { field: platforms, url: "https://intercom.help/funded-futures-family/en/articles/15656536-connecting-your-trading-platform", accessed: "2026-10-23" }
  - { field: platforms, url: "https://www.fundedfuturesfamily.com/", accessed: "2026-10-23" }
  - { field: instruments, url: "https://intercom.help/funded-futures-family/en/articles/17708800-prohibited-trading-conduct", accessed: "2026-10-23" }
  - { field: automation, url: "https://intercom.help/funded-futures-family/en/articles/15892413-bots-algorithmic-trading-policy", accessed: "2026-10-23" }
  - { field: automation, url: "https://intercom.help/funded-futures-family/en/articles/17708800-prohibited-trading-conduct", accessed: "2026-10-23" }
  - { field: copy_trading, url: "https://intercom.help/funded-futures-family/en/articles/17708800-prohibited-trading-conduct", accessed: "2026-10-23" }
  - { field: vps_allowed, url: "https://intercom.help/funded-futures-family/en/articles/15892413-bots-algorithmic-trading-policy", accessed: "2026-10-23" }
  - { field: vps_allowed, url: "https://intercom.help/funded-futures-family/en/articles/17708800-prohibited-trading-conduct", accessed: "2026-10-23" }
  - { field: personal_device_required, url: "https://intercom.help/funded-futures-family/en/articles/17155892-device-sharing", accessed: "2026-10-23" }
  - { field: trading_day_reset, url: "https://intercom.help/funded-futures-family/en/articles/15892350-permitted-times-to-trade", accessed: "2026-10-23" }
  - { field: drawdown, url: "https://intercom.help/funded-futures-family/en/articles/15809232-premier-evaluation", accessed: "2026-10-23" }
  - { field: drawdown, url: "https://intercom.help/funded-futures-family/en/articles/15850031-premier-drawdown", accessed: "2026-10-23" }
  - { field: daily_loss_limit, url: "https://intercom.help/funded-futures-family/en/articles/15809232-premier-evaluation", accessed: "2026-10-23" }
  - { field: profit_target_usd, url: "https://intercom.help/funded-futures-family/en/articles/15809232-premier-evaluation", accessed: "2026-10-23" }
  - { field: consistency_rule, url: "https://intercom.help/funded-futures-family/en/articles/15809232-premier-evaluation", accessed: "2026-10-23" }
  - { field: max_position, url: "https://intercom.help/funded-futures-family/en/articles/15809232-premier-evaluation", accessed: "2026-10-23" }
  - { field: min_trading_days, url: "https://intercom.help/funded-futures-family/en/articles/15809232-premier-evaluation", accessed: "2026-10-23" }
```

#### Premier+ (Intraday drawdown) — evaluation — 100K

```yaml
firm: funded-futures-family
plan: "Premier+ (Intraday drawdown)"
phase: evaluation
size_label: "100K"
size_usd: "100000.00"
instruments: [MNQ, NQ]
platforms: ["Tradovate", "WealthCharts", "NinjaTrader (sign in with FFF credentials, Simulation)", "TradingView (via Tradovate add-on)"]
drawdown: { amount_usd: "3000.00", type: "trailing_intraday", stops_trailing_at: "Not stated for the evaluation; page says for Intraday \"the drawdown continues to trail the account balance as the account grows\"" }
daily_loss_limit: { amount_usd: null, kind: "none" }
profit_target_usd: "6000.00"
consistency_rule: { text: "none (no consistency rule on the evaluation)", threshold_pct: null }
max_position: { minis: "6", micros: "60", scaling: "Scaling plan does not apply in the evaluation" }
min_trading_days: "2"
trading_day_reset: { time: null, tz_as_published: null }
commission_per_contract: { mnq_usd: null, nq_usd: null, basis: null, includes: null }
copy_trading_allowed: conditional
automated_trading_allowed: no
automation_rule_text: "At Funded Futures Family, we don’t permit bot or algorithmic trading. / Bots, algorithms, expert advisors and any automated order entry are not allowed."
vps_allowed: conflict
personal_device_required: yes
status: conflict
not_confirmed_fields: ["commission_per_contract", "trading_day_reset", "vps_allowed (conflict)", "copy_trading_allowed (ambiguity vs automation ban)", "drawdown.stops_trailing_at"]
notes: "Accounts purchased on or after 2026-09-09 (current table). Standard option passes in two days, Fast Pass add-on in one day. Seven days to activate the funded account. No activation fee."
sources:
  - { field: platforms, url: "https://intercom.help/funded-futures-family/en/articles/15656536-connecting-your-trading-platform", accessed: "2026-10-23" }
  - { field: platforms, url: "https://www.fundedfuturesfamily.com/", accessed: "2026-10-23" }
  - { field: instruments, url: "https://intercom.help/funded-futures-family/en/articles/17708800-prohibited-trading-conduct", accessed: "2026-10-23" }
  - { field: automation, url: "https://intercom.help/funded-futures-family/en/articles/15892413-bots-algorithmic-trading-policy", accessed: "2026-10-23" }
  - { field: automation, url: "https://intercom.help/funded-futures-family/en/articles/17708800-prohibited-trading-conduct", accessed: "2026-10-23" }
  - { field: copy_trading, url: "https://intercom.help/funded-futures-family/en/articles/17708800-prohibited-trading-conduct", accessed: "2026-10-23" }
  - { field: vps_allowed, url: "https://intercom.help/funded-futures-family/en/articles/15892413-bots-algorithmic-trading-policy", accessed: "2026-10-23" }
  - { field: vps_allowed, url: "https://intercom.help/funded-futures-family/en/articles/17708800-prohibited-trading-conduct", accessed: "2026-10-23" }
  - { field: personal_device_required, url: "https://intercom.help/funded-futures-family/en/articles/17155892-device-sharing", accessed: "2026-10-23" }
  - { field: trading_day_reset, url: "https://intercom.help/funded-futures-family/en/articles/15892350-permitted-times-to-trade", accessed: "2026-10-23" }
  - { field: drawdown, url: "https://intercom.help/funded-futures-family/en/articles/15809232-premier-evaluation", accessed: "2026-10-23" }
  - { field: drawdown, url: "https://intercom.help/funded-futures-family/en/articles/15850031-premier-drawdown", accessed: "2026-10-23" }
  - { field: daily_loss_limit, url: "https://intercom.help/funded-futures-family/en/articles/15809232-premier-evaluation", accessed: "2026-10-23" }
  - { field: profit_target_usd, url: "https://intercom.help/funded-futures-family/en/articles/15809232-premier-evaluation", accessed: "2026-10-23" }
  - { field: consistency_rule, url: "https://intercom.help/funded-futures-family/en/articles/15809232-premier-evaluation", accessed: "2026-10-23" }
  - { field: max_position, url: "https://intercom.help/funded-futures-family/en/articles/15809232-premier-evaluation", accessed: "2026-10-23" }
  - { field: min_trading_days, url: "https://intercom.help/funded-futures-family/en/articles/15809232-premier-evaluation", accessed: "2026-10-23" }
```

#### Premier+ (Intraday drawdown) — evaluation — 150K

```yaml
firm: funded-futures-family
plan: "Premier+ (Intraday drawdown)"
phase: evaluation
size_label: "150K"
size_usd: "150000.00"
instruments: [MNQ, NQ]
platforms: ["Tradovate", "WealthCharts", "NinjaTrader (sign in with FFF credentials, Simulation)", "TradingView (via Tradovate add-on)"]
drawdown: { amount_usd: "4500.00", type: "trailing_intraday", stops_trailing_at: "Not stated for the evaluation; page says for Intraday \"the drawdown continues to trail the account balance as the account grows\"" }
daily_loss_limit: { amount_usd: null, kind: "none" }
profit_target_usd: "9000.00"
consistency_rule: { text: "none (no consistency rule on the evaluation)", threshold_pct: null }
max_position: { minis: "10", micros: "100", scaling: "Scaling plan does not apply in the evaluation" }
min_trading_days: "2"
trading_day_reset: { time: null, tz_as_published: null }
commission_per_contract: { mnq_usd: null, nq_usd: null, basis: null, includes: null }
copy_trading_allowed: conditional
automated_trading_allowed: no
automation_rule_text: "At Funded Futures Family, we don’t permit bot or algorithmic trading. / Bots, algorithms, expert advisors and any automated order entry are not allowed."
vps_allowed: conflict
personal_device_required: yes
status: conflict
not_confirmed_fields: ["commission_per_contract", "trading_day_reset", "vps_allowed (conflict)", "copy_trading_allowed (ambiguity vs automation ban)", "drawdown.stops_trailing_at"]
notes: "Accounts purchased on or after 2026-09-09 (current table). Standard option passes in two days, Fast Pass add-on in one day. Seven days to activate the funded account. No activation fee. The EOD table prints \"10 Micros/100 Minis\" for 150K (order inverted); the Intraday table and the funded page say 10 minis/100 micros. Treated as a typo but not confirmed."
sources:
  - { field: platforms, url: "https://intercom.help/funded-futures-family/en/articles/15656536-connecting-your-trading-platform", accessed: "2026-10-23" }
  - { field: platforms, url: "https://www.fundedfuturesfamily.com/", accessed: "2026-10-23" }
  - { field: instruments, url: "https://intercom.help/funded-futures-family/en/articles/17708800-prohibited-trading-conduct", accessed: "2026-10-23" }
  - { field: automation, url: "https://intercom.help/funded-futures-family/en/articles/15892413-bots-algorithmic-trading-policy", accessed: "2026-10-23" }
  - { field: automation, url: "https://intercom.help/funded-futures-family/en/articles/17708800-prohibited-trading-conduct", accessed: "2026-10-23" }
  - { field: copy_trading, url: "https://intercom.help/funded-futures-family/en/articles/17708800-prohibited-trading-conduct", accessed: "2026-10-23" }
  - { field: vps_allowed, url: "https://intercom.help/funded-futures-family/en/articles/15892413-bots-algorithmic-trading-policy", accessed: "2026-10-23" }
  - { field: vps_allowed, url: "https://intercom.help/funded-futures-family/en/articles/17708800-prohibited-trading-conduct", accessed: "2026-10-23" }
  - { field: personal_device_required, url: "https://intercom.help/funded-futures-family/en/articles/17155892-device-sharing", accessed: "2026-10-23" }
  - { field: trading_day_reset, url: "https://intercom.help/funded-futures-family/en/articles/15892350-permitted-times-to-trade", accessed: "2026-10-23" }
  - { field: drawdown, url: "https://intercom.help/funded-futures-family/en/articles/15809232-premier-evaluation", accessed: "2026-10-23" }
  - { field: drawdown, url: "https://intercom.help/funded-futures-family/en/articles/15850031-premier-drawdown", accessed: "2026-10-23" }
  - { field: daily_loss_limit, url: "https://intercom.help/funded-futures-family/en/articles/15809232-premier-evaluation", accessed: "2026-10-23" }
  - { field: profit_target_usd, url: "https://intercom.help/funded-futures-family/en/articles/15809232-premier-evaluation", accessed: "2026-10-23" }
  - { field: consistency_rule, url: "https://intercom.help/funded-futures-family/en/articles/15809232-premier-evaluation", accessed: "2026-10-23" }
  - { field: max_position, url: "https://intercom.help/funded-futures-family/en/articles/15809232-premier-evaluation", accessed: "2026-10-23" }
  - { field: min_trading_days, url: "https://intercom.help/funded-futures-family/en/articles/15809232-premier-evaluation", accessed: "2026-10-23" }
```

#### Premier+ (EOD drawdown) — funded — 25K

```yaml
firm: funded-futures-family
plan: "Premier+ (EOD drawdown)"
phase: funded
size_label: "25K"
size_usd: "25000.00"
instruments: [MNQ, NQ]
platforms: ["Tradovate", "WealthCharts", "NinjaTrader (sign in with FFF credentials, Simulation)", "TradingView (via Tradovate add-on)"]
drawdown: { amount_usd: "750.00", type: "trailing_eod", stops_trailing_at: "Locks when it reaches the base account balance" }
daily_loss_limit: { amount_usd: null, kind: "none" }
profit_target_usd: "none"
consistency_rule: { text: "Accounts purchased on or after 2026-09-09: no single trading day above 40% of total realized profit at the time of a payout request; resets after each approved payout. Earlier purchases \"retain their original funded consistency terms\" (not published)", threshold_pct: 40 }
max_position: { minis: "2 (funded table) vs 3 (scaling plan max)", micros: "20 (funded table) vs 30 (scaling plan max)", scaling: "Scaling plan applies; see scaling plan table" }
min_trading_days: null
trading_day_reset: { time: null, tz_as_published: null }
commission_per_contract: { mnq_usd: null, nq_usd: null, basis: null, includes: null }
copy_trading_allowed: conditional
automated_trading_allowed: no
automation_rule_text: "At Funded Futures Family, we don’t permit bot or algorithmic trading. / Bots, algorithms, expert advisors and any automated order entry are not allowed."
vps_allowed: conflict
personal_device_required: yes
status: conflict
not_confirmed_fields: ["commission_per_contract", "trading_day_reset", "vps_allowed (conflict)", "copy_trading_allowed (ambiguity vs automation ban)", "max_position (conflict: funded table vs scaling plan)", "min_trading_days"]
notes: "Payouts every 5 qualifying trading days, minimum $200 profit per trading day, 90/10 split, max payout 50% of profit up to $1,000 / $2,000 / $2,500 / $3,000. The funded page prints the Intraday amounts as the Max Drawdown column although it also describes an EOD variant; the EOD amounts come from the Premier+ Drawdown page."
sources:
  - { field: platforms, url: "https://intercom.help/funded-futures-family/en/articles/15656536-connecting-your-trading-platform", accessed: "2026-10-23" }
  - { field: platforms, url: "https://www.fundedfuturesfamily.com/", accessed: "2026-10-23" }
  - { field: instruments, url: "https://intercom.help/funded-futures-family/en/articles/17708800-prohibited-trading-conduct", accessed: "2026-10-23" }
  - { field: automation, url: "https://intercom.help/funded-futures-family/en/articles/15892413-bots-algorithmic-trading-policy", accessed: "2026-10-23" }
  - { field: automation, url: "https://intercom.help/funded-futures-family/en/articles/17708800-prohibited-trading-conduct", accessed: "2026-10-23" }
  - { field: copy_trading, url: "https://intercom.help/funded-futures-family/en/articles/17708800-prohibited-trading-conduct", accessed: "2026-10-23" }
  - { field: vps_allowed, url: "https://intercom.help/funded-futures-family/en/articles/15892413-bots-algorithmic-trading-policy", accessed: "2026-10-23" }
  - { field: vps_allowed, url: "https://intercom.help/funded-futures-family/en/articles/17708800-prohibited-trading-conduct", accessed: "2026-10-23" }
  - { field: personal_device_required, url: "https://intercom.help/funded-futures-family/en/articles/17155892-device-sharing", accessed: "2026-10-23" }
  - { field: trading_day_reset, url: "https://intercom.help/funded-futures-family/en/articles/15892350-permitted-times-to-trade", accessed: "2026-10-23" }
  - { field: drawdown, url: "https://intercom.help/funded-futures-family/en/articles/15811095-premier-funded-account", accessed: "2026-10-23" }
  - { field: drawdown, url: "https://intercom.help/funded-futures-family/en/articles/15850031-premier-drawdown", accessed: "2026-10-23" }
  - { field: daily_loss_limit, url: "https://intercom.help/funded-futures-family/en/articles/15811095-premier-funded-account", accessed: "2026-10-23" }
  - { field: consistency_rule, url: "https://intercom.help/funded-futures-family/en/articles/15811095-premier-funded-account", accessed: "2026-10-23" }
  - { field: consistency_rule, url: "https://intercom.help/funded-futures-family/en/articles/15811464-premier-payout-requirements", accessed: "2026-10-23" }
  - { field: consistency_rule, url: "https://intercom.help/funded-futures-family/en/articles/16964667-premier-consistency", accessed: "2026-10-23" }
  - { field: max_position, url: "https://intercom.help/funded-futures-family/en/articles/15811095-premier-funded-account", accessed: "2026-10-23" }
  - { field: max_position, url: "https://intercom.help/funded-futures-family/en/articles/16302826-premier-scaling-plan", accessed: "2026-10-23" }
```

#### Premier+ (EOD drawdown) — funded — 50K

```yaml
firm: funded-futures-family
plan: "Premier+ (EOD drawdown)"
phase: funded
size_label: "50K"
size_usd: "50000.00"
instruments: [MNQ, NQ]
platforms: ["Tradovate", "WealthCharts", "NinjaTrader (sign in with FFF credentials, Simulation)", "TradingView (via Tradovate add-on)"]
drawdown: { amount_usd: "1500.00", type: "trailing_eod", stops_trailing_at: "Locks when it reaches the base account balance" }
daily_loss_limit: { amount_usd: null, kind: "none" }
profit_target_usd: "none"
consistency_rule: { text: "Accounts purchased on or after 2026-09-09: no single trading day above 40% of total realized profit at the time of a payout request; resets after each approved payout. Earlier purchases \"retain their original funded consistency terms\" (not published)", threshold_pct: 40 }
max_position: { minis: "4 (funded table) vs 5 (scaling plan max)", micros: "40 (funded table) vs 50 (scaling plan max)", scaling: "Scaling plan applies; see scaling plan table" }
min_trading_days: null
trading_day_reset: { time: null, tz_as_published: null }
commission_per_contract: { mnq_usd: null, nq_usd: null, basis: null, includes: null }
copy_trading_allowed: conditional
automated_trading_allowed: no
automation_rule_text: "At Funded Futures Family, we don’t permit bot or algorithmic trading. / Bots, algorithms, expert advisors and any automated order entry are not allowed."
vps_allowed: conflict
personal_device_required: yes
status: conflict
not_confirmed_fields: ["commission_per_contract", "trading_day_reset", "vps_allowed (conflict)", "copy_trading_allowed (ambiguity vs automation ban)", "max_position (conflict: funded table vs scaling plan)", "min_trading_days"]
notes: "Payouts every 5 qualifying trading days, minimum $200 profit per trading day, 90/10 split, max payout 50% of profit up to $1,000 / $2,000 / $2,500 / $3,000. The funded page prints the Intraday amounts as the Max Drawdown column although it also describes an EOD variant; the EOD amounts come from the Premier+ Drawdown page."
sources:
  - { field: platforms, url: "https://intercom.help/funded-futures-family/en/articles/15656536-connecting-your-trading-platform", accessed: "2026-10-23" }
  - { field: platforms, url: "https://www.fundedfuturesfamily.com/", accessed: "2026-10-23" }
  - { field: instruments, url: "https://intercom.help/funded-futures-family/en/articles/17708800-prohibited-trading-conduct", accessed: "2026-10-23" }
  - { field: automation, url: "https://intercom.help/funded-futures-family/en/articles/15892413-bots-algorithmic-trading-policy", accessed: "2026-10-23" }
  - { field: automation, url: "https://intercom.help/funded-futures-family/en/articles/17708800-prohibited-trading-conduct", accessed: "2026-10-23" }
  - { field: copy_trading, url: "https://intercom.help/funded-futures-family/en/articles/17708800-prohibited-trading-conduct", accessed: "2026-10-23" }
  - { field: vps_allowed, url: "https://intercom.help/funded-futures-family/en/articles/15892413-bots-algorithmic-trading-policy", accessed: "2026-10-23" }
  - { field: vps_allowed, url: "https://intercom.help/funded-futures-family/en/articles/17708800-prohibited-trading-conduct", accessed: "2026-10-23" }
  - { field: personal_device_required, url: "https://intercom.help/funded-futures-family/en/articles/17155892-device-sharing", accessed: "2026-10-23" }
  - { field: trading_day_reset, url: "https://intercom.help/funded-futures-family/en/articles/15892350-permitted-times-to-trade", accessed: "2026-10-23" }
  - { field: drawdown, url: "https://intercom.help/funded-futures-family/en/articles/15811095-premier-funded-account", accessed: "2026-10-23" }
  - { field: drawdown, url: "https://intercom.help/funded-futures-family/en/articles/15850031-premier-drawdown", accessed: "2026-10-23" }
  - { field: daily_loss_limit, url: "https://intercom.help/funded-futures-family/en/articles/15811095-premier-funded-account", accessed: "2026-10-23" }
  - { field: consistency_rule, url: "https://intercom.help/funded-futures-family/en/articles/15811095-premier-funded-account", accessed: "2026-10-23" }
  - { field: consistency_rule, url: "https://intercom.help/funded-futures-family/en/articles/15811464-premier-payout-requirements", accessed: "2026-10-23" }
  - { field: consistency_rule, url: "https://intercom.help/funded-futures-family/en/articles/16964667-premier-consistency", accessed: "2026-10-23" }
  - { field: max_position, url: "https://intercom.help/funded-futures-family/en/articles/15811095-premier-funded-account", accessed: "2026-10-23" }
  - { field: max_position, url: "https://intercom.help/funded-futures-family/en/articles/16302826-premier-scaling-plan", accessed: "2026-10-23" }
```

#### Premier+ (EOD drawdown) — funded — 100K

```yaml
firm: funded-futures-family
plan: "Premier+ (EOD drawdown)"
phase: funded
size_label: "100K"
size_usd: "100000.00"
instruments: [MNQ, NQ]
platforms: ["Tradovate", "WealthCharts", "NinjaTrader (sign in with FFF credentials, Simulation)", "TradingView (via Tradovate add-on)"]
drawdown: { amount_usd: "2500.00", type: "trailing_eod", stops_trailing_at: "Locks when it reaches the base account balance" }
daily_loss_limit: { amount_usd: null, kind: "none" }
profit_target_usd: "none"
consistency_rule: { text: "Accounts purchased on or after 2026-09-09: no single trading day above 40% of total realized profit at the time of a payout request; resets after each approved payout. Earlier purchases \"retain their original funded consistency terms\" (not published)", threshold_pct: 40 }
max_position: { minis: "6 (funded table) vs 10 (scaling plan max)", micros: "60 (funded table) vs 100 (scaling plan max)", scaling: "Scaling plan applies; see scaling plan table" }
min_trading_days: null
trading_day_reset: { time: null, tz_as_published: null }
commission_per_contract: { mnq_usd: null, nq_usd: null, basis: null, includes: null }
copy_trading_allowed: conditional
automated_trading_allowed: no
automation_rule_text: "At Funded Futures Family, we don’t permit bot or algorithmic trading. / Bots, algorithms, expert advisors and any automated order entry are not allowed."
vps_allowed: conflict
personal_device_required: yes
status: conflict
not_confirmed_fields: ["commission_per_contract", "trading_day_reset", "vps_allowed (conflict)", "copy_trading_allowed (ambiguity vs automation ban)", "max_position (conflict: funded table vs scaling plan)", "min_trading_days"]
notes: "Payouts every 5 qualifying trading days, minimum $200 profit per trading day, 90/10 split, max payout 50% of profit up to $1,000 / $2,000 / $2,500 / $3,000. The funded page prints the Intraday amounts as the Max Drawdown column although it also describes an EOD variant; the EOD amounts come from the Premier+ Drawdown page."
sources:
  - { field: platforms, url: "https://intercom.help/funded-futures-family/en/articles/15656536-connecting-your-trading-platform", accessed: "2026-10-23" }
  - { field: platforms, url: "https://www.fundedfuturesfamily.com/", accessed: "2026-10-23" }
  - { field: instruments, url: "https://intercom.help/funded-futures-family/en/articles/17708800-prohibited-trading-conduct", accessed: "2026-10-23" }
  - { field: automation, url: "https://intercom.help/funded-futures-family/en/articles/15892413-bots-algorithmic-trading-policy", accessed: "2026-10-23" }
  - { field: automation, url: "https://intercom.help/funded-futures-family/en/articles/17708800-prohibited-trading-conduct", accessed: "2026-10-23" }
  - { field: copy_trading, url: "https://intercom.help/funded-futures-family/en/articles/17708800-prohibited-trading-conduct", accessed: "2026-10-23" }
  - { field: vps_allowed, url: "https://intercom.help/funded-futures-family/en/articles/15892413-bots-algorithmic-trading-policy", accessed: "2026-10-23" }
  - { field: vps_allowed, url: "https://intercom.help/funded-futures-family/en/articles/17708800-prohibited-trading-conduct", accessed: "2026-10-23" }
  - { field: personal_device_required, url: "https://intercom.help/funded-futures-family/en/articles/17155892-device-sharing", accessed: "2026-10-23" }
  - { field: trading_day_reset, url: "https://intercom.help/funded-futures-family/en/articles/15892350-permitted-times-to-trade", accessed: "2026-10-23" }
  - { field: drawdown, url: "https://intercom.help/funded-futures-family/en/articles/15811095-premier-funded-account", accessed: "2026-10-23" }
  - { field: drawdown, url: "https://intercom.help/funded-futures-family/en/articles/15850031-premier-drawdown", accessed: "2026-10-23" }
  - { field: daily_loss_limit, url: "https://intercom.help/funded-futures-family/en/articles/15811095-premier-funded-account", accessed: "2026-10-23" }
  - { field: consistency_rule, url: "https://intercom.help/funded-futures-family/en/articles/15811095-premier-funded-account", accessed: "2026-10-23" }
  - { field: consistency_rule, url: "https://intercom.help/funded-futures-family/en/articles/15811464-premier-payout-requirements", accessed: "2026-10-23" }
  - { field: consistency_rule, url: "https://intercom.help/funded-futures-family/en/articles/16964667-premier-consistency", accessed: "2026-10-23" }
  - { field: max_position, url: "https://intercom.help/funded-futures-family/en/articles/15811095-premier-funded-account", accessed: "2026-10-23" }
  - { field: max_position, url: "https://intercom.help/funded-futures-family/en/articles/16302826-premier-scaling-plan", accessed: "2026-10-23" }
```

#### Premier+ (EOD drawdown) — funded — 150K

```yaml
firm: funded-futures-family
plan: "Premier+ (EOD drawdown)"
phase: funded
size_label: "150K"
size_usd: "150000.00"
instruments: [MNQ, NQ]
platforms: ["Tradovate", "WealthCharts", "NinjaTrader (sign in with FFF credentials, Simulation)", "TradingView (via Tradovate add-on)"]
drawdown: { amount_usd: "4000.00", type: "trailing_eod", stops_trailing_at: "Locks when it reaches the base account balance" }
daily_loss_limit: { amount_usd: null, kind: "none" }
profit_target_usd: "none"
consistency_rule: { text: "Accounts purchased on or after 2026-09-09: no single trading day above 40% of total realized profit at the time of a payout request; resets after each approved payout. Earlier purchases \"retain their original funded consistency terms\" (not published)", threshold_pct: 40 }
max_position: { minis: "10 (funded table) vs 15 (scaling plan max)", micros: "100 (funded table) vs 150 (scaling plan max)", scaling: "Scaling plan applies; see scaling plan table" }
min_trading_days: null
trading_day_reset: { time: null, tz_as_published: null }
commission_per_contract: { mnq_usd: null, nq_usd: null, basis: null, includes: null }
copy_trading_allowed: conditional
automated_trading_allowed: no
automation_rule_text: "At Funded Futures Family, we don’t permit bot or algorithmic trading. / Bots, algorithms, expert advisors and any automated order entry are not allowed."
vps_allowed: conflict
personal_device_required: yes
status: conflict
not_confirmed_fields: ["commission_per_contract", "trading_day_reset", "vps_allowed (conflict)", "copy_trading_allowed (ambiguity vs automation ban)", "max_position (conflict: funded table vs scaling plan)", "min_trading_days"]
notes: "Payouts every 5 qualifying trading days, minimum $200 profit per trading day, 90/10 split, max payout 50% of profit up to $1,000 / $2,000 / $2,500 / $3,000. The funded page prints the Intraday amounts as the Max Drawdown column although it also describes an EOD variant; the EOD amounts come from the Premier+ Drawdown page."
sources:
  - { field: platforms, url: "https://intercom.help/funded-futures-family/en/articles/15656536-connecting-your-trading-platform", accessed: "2026-10-23" }
  - { field: platforms, url: "https://www.fundedfuturesfamily.com/", accessed: "2026-10-23" }
  - { field: instruments, url: "https://intercom.help/funded-futures-family/en/articles/17708800-prohibited-trading-conduct", accessed: "2026-10-23" }
  - { field: automation, url: "https://intercom.help/funded-futures-family/en/articles/15892413-bots-algorithmic-trading-policy", accessed: "2026-10-23" }
  - { field: automation, url: "https://intercom.help/funded-futures-family/en/articles/17708800-prohibited-trading-conduct", accessed: "2026-10-23" }
  - { field: copy_trading, url: "https://intercom.help/funded-futures-family/en/articles/17708800-prohibited-trading-conduct", accessed: "2026-10-23" }
  - { field: vps_allowed, url: "https://intercom.help/funded-futures-family/en/articles/15892413-bots-algorithmic-trading-policy", accessed: "2026-10-23" }
  - { field: vps_allowed, url: "https://intercom.help/funded-futures-family/en/articles/17708800-prohibited-trading-conduct", accessed: "2026-10-23" }
  - { field: personal_device_required, url: "https://intercom.help/funded-futures-family/en/articles/17155892-device-sharing", accessed: "2026-10-23" }
  - { field: trading_day_reset, url: "https://intercom.help/funded-futures-family/en/articles/15892350-permitted-times-to-trade", accessed: "2026-10-23" }
  - { field: drawdown, url: "https://intercom.help/funded-futures-family/en/articles/15811095-premier-funded-account", accessed: "2026-10-23" }
  - { field: drawdown, url: "https://intercom.help/funded-futures-family/en/articles/15850031-premier-drawdown", accessed: "2026-10-23" }
  - { field: daily_loss_limit, url: "https://intercom.help/funded-futures-family/en/articles/15811095-premier-funded-account", accessed: "2026-10-23" }
  - { field: consistency_rule, url: "https://intercom.help/funded-futures-family/en/articles/15811095-premier-funded-account", accessed: "2026-10-23" }
  - { field: consistency_rule, url: "https://intercom.help/funded-futures-family/en/articles/15811464-premier-payout-requirements", accessed: "2026-10-23" }
  - { field: consistency_rule, url: "https://intercom.help/funded-futures-family/en/articles/16964667-premier-consistency", accessed: "2026-10-23" }
  - { field: max_position, url: "https://intercom.help/funded-futures-family/en/articles/15811095-premier-funded-account", accessed: "2026-10-23" }
  - { field: max_position, url: "https://intercom.help/funded-futures-family/en/articles/16302826-premier-scaling-plan", accessed: "2026-10-23" }
```

#### Premier+ (Intraday drawdown) — funded — 25K

```yaml
firm: funded-futures-family
plan: "Premier+ (Intraday drawdown)"
phase: funded
size_label: "25K"
size_usd: "25000.00"
instruments: [MNQ, NQ]
platforms: ["Tradovate", "WealthCharts", "NinjaTrader (sign in with FFF credentials, Simulation)", "TradingView (via Tradovate add-on)"]
drawdown: { amount_usd: "1000.00", type: "trailing_intraday", stops_trailing_at: "Not stated; Intraday trails \"as the account grows\". After the first payout \"your drawdown gets brought up to base account size\"" }
daily_loss_limit: { amount_usd: null, kind: "none" }
profit_target_usd: "none"
consistency_rule: { text: "Accounts purchased on or after 2026-09-09: no single trading day above 40% of total realized profit at the time of a payout request; resets after each approved payout. Earlier purchases \"retain their original funded consistency terms\" (not published)", threshold_pct: 40 }
max_position: { minis: "2 (funded table) vs 3 (scaling plan max)", micros: "20 (funded table) vs 30 (scaling plan max)", scaling: "Scaling plan applies; see scaling plan table" }
min_trading_days: null
trading_day_reset: { time: null, tz_as_published: null }
commission_per_contract: { mnq_usd: null, nq_usd: null, basis: null, includes: null }
copy_trading_allowed: conditional
automated_trading_allowed: no
automation_rule_text: "At Funded Futures Family, we don’t permit bot or algorithmic trading. / Bots, algorithms, expert advisors and any automated order entry are not allowed."
vps_allowed: conflict
personal_device_required: yes
status: conflict
not_confirmed_fields: ["commission_per_contract", "trading_day_reset", "vps_allowed (conflict)", "copy_trading_allowed (ambiguity vs automation ban)", "max_position (conflict: funded table vs scaling plan)", "min_trading_days", "drawdown.stops_trailing_at"]
notes: "Payouts every 5 qualifying trading days, minimum $200 profit per trading day, 90/10 split, max payout 50% of profit up to $1,000 / $2,000 / $2,500 / $3,000. The funded page prints the Intraday amounts as the Max Drawdown column although it also describes an EOD variant; the EOD amounts come from the Premier+ Drawdown page."
sources:
  - { field: platforms, url: "https://intercom.help/funded-futures-family/en/articles/15656536-connecting-your-trading-platform", accessed: "2026-10-23" }
  - { field: platforms, url: "https://www.fundedfuturesfamily.com/", accessed: "2026-10-23" }
  - { field: instruments, url: "https://intercom.help/funded-futures-family/en/articles/17708800-prohibited-trading-conduct", accessed: "2026-10-23" }
  - { field: automation, url: "https://intercom.help/funded-futures-family/en/articles/15892413-bots-algorithmic-trading-policy", accessed: "2026-10-23" }
  - { field: automation, url: "https://intercom.help/funded-futures-family/en/articles/17708800-prohibited-trading-conduct", accessed: "2026-10-23" }
  - { field: copy_trading, url: "https://intercom.help/funded-futures-family/en/articles/17708800-prohibited-trading-conduct", accessed: "2026-10-23" }
  - { field: vps_allowed, url: "https://intercom.help/funded-futures-family/en/articles/15892413-bots-algorithmic-trading-policy", accessed: "2026-10-23" }
  - { field: vps_allowed, url: "https://intercom.help/funded-futures-family/en/articles/17708800-prohibited-trading-conduct", accessed: "2026-10-23" }
  - { field: personal_device_required, url: "https://intercom.help/funded-futures-family/en/articles/17155892-device-sharing", accessed: "2026-10-23" }
  - { field: trading_day_reset, url: "https://intercom.help/funded-futures-family/en/articles/15892350-permitted-times-to-trade", accessed: "2026-10-23" }
  - { field: drawdown, url: "https://intercom.help/funded-futures-family/en/articles/15811095-premier-funded-account", accessed: "2026-10-23" }
  - { field: drawdown, url: "https://intercom.help/funded-futures-family/en/articles/15850031-premier-drawdown", accessed: "2026-10-23" }
  - { field: daily_loss_limit, url: "https://intercom.help/funded-futures-family/en/articles/15811095-premier-funded-account", accessed: "2026-10-23" }
  - { field: consistency_rule, url: "https://intercom.help/funded-futures-family/en/articles/15811095-premier-funded-account", accessed: "2026-10-23" }
  - { field: consistency_rule, url: "https://intercom.help/funded-futures-family/en/articles/15811464-premier-payout-requirements", accessed: "2026-10-23" }
  - { field: consistency_rule, url: "https://intercom.help/funded-futures-family/en/articles/16964667-premier-consistency", accessed: "2026-10-23" }
  - { field: max_position, url: "https://intercom.help/funded-futures-family/en/articles/15811095-premier-funded-account", accessed: "2026-10-23" }
  - { field: max_position, url: "https://intercom.help/funded-futures-family/en/articles/16302826-premier-scaling-plan", accessed: "2026-10-23" }
```

#### Premier+ (Intraday drawdown) — funded — 50K

```yaml
firm: funded-futures-family
plan: "Premier+ (Intraday drawdown)"
phase: funded
size_label: "50K"
size_usd: "50000.00"
instruments: [MNQ, NQ]
platforms: ["Tradovate", "WealthCharts", "NinjaTrader (sign in with FFF credentials, Simulation)", "TradingView (via Tradovate add-on)"]
drawdown: { amount_usd: "2000.00", type: "trailing_intraday", stops_trailing_at: "Not stated; Intraday trails \"as the account grows\". After the first payout \"your drawdown gets brought up to base account size\"" }
daily_loss_limit: { amount_usd: null, kind: "none" }
profit_target_usd: "none"
consistency_rule: { text: "Accounts purchased on or after 2026-09-09: no single trading day above 40% of total realized profit at the time of a payout request; resets after each approved payout. Earlier purchases \"retain their original funded consistency terms\" (not published)", threshold_pct: 40 }
max_position: { minis: "4 (funded table) vs 5 (scaling plan max)", micros: "40 (funded table) vs 50 (scaling plan max)", scaling: "Scaling plan applies; see scaling plan table" }
min_trading_days: null
trading_day_reset: { time: null, tz_as_published: null }
commission_per_contract: { mnq_usd: null, nq_usd: null, basis: null, includes: null }
copy_trading_allowed: conditional
automated_trading_allowed: no
automation_rule_text: "At Funded Futures Family, we don’t permit bot or algorithmic trading. / Bots, algorithms, expert advisors and any automated order entry are not allowed."
vps_allowed: conflict
personal_device_required: yes
status: conflict
not_confirmed_fields: ["commission_per_contract", "trading_day_reset", "vps_allowed (conflict)", "copy_trading_allowed (ambiguity vs automation ban)", "max_position (conflict: funded table vs scaling plan)", "min_trading_days", "drawdown.stops_trailing_at"]
notes: "Payouts every 5 qualifying trading days, minimum $200 profit per trading day, 90/10 split, max payout 50% of profit up to $1,000 / $2,000 / $2,500 / $3,000. The funded page prints the Intraday amounts as the Max Drawdown column although it also describes an EOD variant; the EOD amounts come from the Premier+ Drawdown page."
sources:
  - { field: platforms, url: "https://intercom.help/funded-futures-family/en/articles/15656536-connecting-your-trading-platform", accessed: "2026-10-23" }
  - { field: platforms, url: "https://www.fundedfuturesfamily.com/", accessed: "2026-10-23" }
  - { field: instruments, url: "https://intercom.help/funded-futures-family/en/articles/17708800-prohibited-trading-conduct", accessed: "2026-10-23" }
  - { field: automation, url: "https://intercom.help/funded-futures-family/en/articles/15892413-bots-algorithmic-trading-policy", accessed: "2026-10-23" }
  - { field: automation, url: "https://intercom.help/funded-futures-family/en/articles/17708800-prohibited-trading-conduct", accessed: "2026-10-23" }
  - { field: copy_trading, url: "https://intercom.help/funded-futures-family/en/articles/17708800-prohibited-trading-conduct", accessed: "2026-10-23" }
  - { field: vps_allowed, url: "https://intercom.help/funded-futures-family/en/articles/15892413-bots-algorithmic-trading-policy", accessed: "2026-10-23" }
  - { field: vps_allowed, url: "https://intercom.help/funded-futures-family/en/articles/17708800-prohibited-trading-conduct", accessed: "2026-10-23" }
  - { field: personal_device_required, url: "https://intercom.help/funded-futures-family/en/articles/17155892-device-sharing", accessed: "2026-10-23" }
  - { field: trading_day_reset, url: "https://intercom.help/funded-futures-family/en/articles/15892350-permitted-times-to-trade", accessed: "2026-10-23" }
  - { field: drawdown, url: "https://intercom.help/funded-futures-family/en/articles/15811095-premier-funded-account", accessed: "2026-10-23" }
  - { field: drawdown, url: "https://intercom.help/funded-futures-family/en/articles/15850031-premier-drawdown", accessed: "2026-10-23" }
  - { field: daily_loss_limit, url: "https://intercom.help/funded-futures-family/en/articles/15811095-premier-funded-account", accessed: "2026-10-23" }
  - { field: consistency_rule, url: "https://intercom.help/funded-futures-family/en/articles/15811095-premier-funded-account", accessed: "2026-10-23" }
  - { field: consistency_rule, url: "https://intercom.help/funded-futures-family/en/articles/15811464-premier-payout-requirements", accessed: "2026-10-23" }
  - { field: consistency_rule, url: "https://intercom.help/funded-futures-family/en/articles/16964667-premier-consistency", accessed: "2026-10-23" }
  - { field: max_position, url: "https://intercom.help/funded-futures-family/en/articles/15811095-premier-funded-account", accessed: "2026-10-23" }
  - { field: max_position, url: "https://intercom.help/funded-futures-family/en/articles/16302826-premier-scaling-plan", accessed: "2026-10-23" }
```

#### Premier+ (Intraday drawdown) — funded — 100K

```yaml
firm: funded-futures-family
plan: "Premier+ (Intraday drawdown)"
phase: funded
size_label: "100K"
size_usd: "100000.00"
instruments: [MNQ, NQ]
platforms: ["Tradovate", "WealthCharts", "NinjaTrader (sign in with FFF credentials, Simulation)", "TradingView (via Tradovate add-on)"]
drawdown: { amount_usd: "3000.00", type: "trailing_intraday", stops_trailing_at: "Not stated; Intraday trails \"as the account grows\". After the first payout \"your drawdown gets brought up to base account size\"" }
daily_loss_limit: { amount_usd: null, kind: "none" }
profit_target_usd: "none"
consistency_rule: { text: "Accounts purchased on or after 2026-09-09: no single trading day above 40% of total realized profit at the time of a payout request; resets after each approved payout. Earlier purchases \"retain their original funded consistency terms\" (not published)", threshold_pct: 40 }
max_position: { minis: "6 (funded table) vs 10 (scaling plan max)", micros: "60 (funded table) vs 100 (scaling plan max)", scaling: "Scaling plan applies; see scaling plan table" }
min_trading_days: null
trading_day_reset: { time: null, tz_as_published: null }
commission_per_contract: { mnq_usd: null, nq_usd: null, basis: null, includes: null }
copy_trading_allowed: conditional
automated_trading_allowed: no
automation_rule_text: "At Funded Futures Family, we don’t permit bot or algorithmic trading. / Bots, algorithms, expert advisors and any automated order entry are not allowed."
vps_allowed: conflict
personal_device_required: yes
status: conflict
not_confirmed_fields: ["commission_per_contract", "trading_day_reset", "vps_allowed (conflict)", "copy_trading_allowed (ambiguity vs automation ban)", "max_position (conflict: funded table vs scaling plan)", "min_trading_days", "drawdown.stops_trailing_at"]
notes: "Payouts every 5 qualifying trading days, minimum $200 profit per trading day, 90/10 split, max payout 50% of profit up to $1,000 / $2,000 / $2,500 / $3,000. The funded page prints the Intraday amounts as the Max Drawdown column although it also describes an EOD variant; the EOD amounts come from the Premier+ Drawdown page."
sources:
  - { field: platforms, url: "https://intercom.help/funded-futures-family/en/articles/15656536-connecting-your-trading-platform", accessed: "2026-10-23" }
  - { field: platforms, url: "https://www.fundedfuturesfamily.com/", accessed: "2026-10-23" }
  - { field: instruments, url: "https://intercom.help/funded-futures-family/en/articles/17708800-prohibited-trading-conduct", accessed: "2026-10-23" }
  - { field: automation, url: "https://intercom.help/funded-futures-family/en/articles/15892413-bots-algorithmic-trading-policy", accessed: "2026-10-23" }
  - { field: automation, url: "https://intercom.help/funded-futures-family/en/articles/17708800-prohibited-trading-conduct", accessed: "2026-10-23" }
  - { field: copy_trading, url: "https://intercom.help/funded-futures-family/en/articles/17708800-prohibited-trading-conduct", accessed: "2026-10-23" }
  - { field: vps_allowed, url: "https://intercom.help/funded-futures-family/en/articles/15892413-bots-algorithmic-trading-policy", accessed: "2026-10-23" }
  - { field: vps_allowed, url: "https://intercom.help/funded-futures-family/en/articles/17708800-prohibited-trading-conduct", accessed: "2026-10-23" }
  - { field: personal_device_required, url: "https://intercom.help/funded-futures-family/en/articles/17155892-device-sharing", accessed: "2026-10-23" }
  - { field: trading_day_reset, url: "https://intercom.help/funded-futures-family/en/articles/15892350-permitted-times-to-trade", accessed: "2026-10-23" }
  - { field: drawdown, url: "https://intercom.help/funded-futures-family/en/articles/15811095-premier-funded-account", accessed: "2026-10-23" }
  - { field: drawdown, url: "https://intercom.help/funded-futures-family/en/articles/15850031-premier-drawdown", accessed: "2026-10-23" }
  - { field: daily_loss_limit, url: "https://intercom.help/funded-futures-family/en/articles/15811095-premier-funded-account", accessed: "2026-10-23" }
  - { field: consistency_rule, url: "https://intercom.help/funded-futures-family/en/articles/15811095-premier-funded-account", accessed: "2026-10-23" }
  - { field: consistency_rule, url: "https://intercom.help/funded-futures-family/en/articles/15811464-premier-payout-requirements", accessed: "2026-10-23" }
  - { field: consistency_rule, url: "https://intercom.help/funded-futures-family/en/articles/16964667-premier-consistency", accessed: "2026-10-23" }
  - { field: max_position, url: "https://intercom.help/funded-futures-family/en/articles/15811095-premier-funded-account", accessed: "2026-10-23" }
  - { field: max_position, url: "https://intercom.help/funded-futures-family/en/articles/16302826-premier-scaling-plan", accessed: "2026-10-23" }
```

#### Premier+ (Intraday drawdown) — funded — 150K

```yaml
firm: funded-futures-family
plan: "Premier+ (Intraday drawdown)"
phase: funded
size_label: "150K"
size_usd: "150000.00"
instruments: [MNQ, NQ]
platforms: ["Tradovate", "WealthCharts", "NinjaTrader (sign in with FFF credentials, Simulation)", "TradingView (via Tradovate add-on)"]
drawdown: { amount_usd: "4500.00", type: "trailing_intraday", stops_trailing_at: "Not stated; Intraday trails \"as the account grows\". After the first payout \"your drawdown gets brought up to base account size\"" }
daily_loss_limit: { amount_usd: null, kind: "none" }
profit_target_usd: "none"
consistency_rule: { text: "Accounts purchased on or after 2026-09-09: no single trading day above 40% of total realized profit at the time of a payout request; resets after each approved payout. Earlier purchases \"retain their original funded consistency terms\" (not published)", threshold_pct: 40 }
max_position: { minis: "10 (funded table) vs 15 (scaling plan max)", micros: "100 (funded table) vs 150 (scaling plan max)", scaling: "Scaling plan applies; see scaling plan table" }
min_trading_days: null
trading_day_reset: { time: null, tz_as_published: null }
commission_per_contract: { mnq_usd: null, nq_usd: null, basis: null, includes: null }
copy_trading_allowed: conditional
automated_trading_allowed: no
automation_rule_text: "At Funded Futures Family, we don’t permit bot or algorithmic trading. / Bots, algorithms, expert advisors and any automated order entry are not allowed."
vps_allowed: conflict
personal_device_required: yes
status: conflict
not_confirmed_fields: ["commission_per_contract", "trading_day_reset", "vps_allowed (conflict)", "copy_trading_allowed (ambiguity vs automation ban)", "max_position (conflict: funded table vs scaling plan)", "min_trading_days", "drawdown.stops_trailing_at"]
notes: "Payouts every 5 qualifying trading days, minimum $200 profit per trading day, 90/10 split, max payout 50% of profit up to $1,000 / $2,000 / $2,500 / $3,000. The funded page prints the Intraday amounts as the Max Drawdown column although it also describes an EOD variant; the EOD amounts come from the Premier+ Drawdown page."
sources:
  - { field: platforms, url: "https://intercom.help/funded-futures-family/en/articles/15656536-connecting-your-trading-platform", accessed: "2026-10-23" }
  - { field: platforms, url: "https://www.fundedfuturesfamily.com/", accessed: "2026-10-23" }
  - { field: instruments, url: "https://intercom.help/funded-futures-family/en/articles/17708800-prohibited-trading-conduct", accessed: "2026-10-23" }
  - { field: automation, url: "https://intercom.help/funded-futures-family/en/articles/15892413-bots-algorithmic-trading-policy", accessed: "2026-10-23" }
  - { field: automation, url: "https://intercom.help/funded-futures-family/en/articles/17708800-prohibited-trading-conduct", accessed: "2026-10-23" }
  - { field: copy_trading, url: "https://intercom.help/funded-futures-family/en/articles/17708800-prohibited-trading-conduct", accessed: "2026-10-23" }
  - { field: vps_allowed, url: "https://intercom.help/funded-futures-family/en/articles/15892413-bots-algorithmic-trading-policy", accessed: "2026-10-23" }
  - { field: vps_allowed, url: "https://intercom.help/funded-futures-family/en/articles/17708800-prohibited-trading-conduct", accessed: "2026-10-23" }
  - { field: personal_device_required, url: "https://intercom.help/funded-futures-family/en/articles/17155892-device-sharing", accessed: "2026-10-23" }
  - { field: trading_day_reset, url: "https://intercom.help/funded-futures-family/en/articles/15892350-permitted-times-to-trade", accessed: "2026-10-23" }
  - { field: drawdown, url: "https://intercom.help/funded-futures-family/en/articles/15811095-premier-funded-account", accessed: "2026-10-23" }
  - { field: drawdown, url: "https://intercom.help/funded-futures-family/en/articles/15850031-premier-drawdown", accessed: "2026-10-23" }
  - { field: daily_loss_limit, url: "https://intercom.help/funded-futures-family/en/articles/15811095-premier-funded-account", accessed: "2026-10-23" }
  - { field: consistency_rule, url: "https://intercom.help/funded-futures-family/en/articles/15811095-premier-funded-account", accessed: "2026-10-23" }
  - { field: consistency_rule, url: "https://intercom.help/funded-futures-family/en/articles/15811464-premier-payout-requirements", accessed: "2026-10-23" }
  - { field: consistency_rule, url: "https://intercom.help/funded-futures-family/en/articles/16964667-premier-consistency", accessed: "2026-10-23" }
  - { field: max_position, url: "https://intercom.help/funded-futures-family/en/articles/15811095-premier-funded-account", accessed: "2026-10-23" }
  - { field: max_position, url: "https://intercom.help/funded-futures-family/en/articles/16302826-premier-scaling-plan", accessed: "2026-10-23" }
```

#### Velocity — evaluation — 25K

```yaml
firm: funded-futures-family
plan: "Velocity"
phase: evaluation
size_label: "25K"
size_usd: "25000.00"
instruments: [MNQ, NQ]
platforms: ["Tradovate", "WealthCharts", "NinjaTrader (sign in with FFF credentials, Simulation)", "TradingView (via Tradovate add-on)"]
drawdown: { amount_usd: "1250.00", type: "trailing_intraday", stops_trailing_at: "Not stated (page: threshold \"does not move back down\")" }
daily_loss_limit: { amount_usd: null, kind: "none" }
profit_target_usd: "2500.00"
consistency_rule: { text: "40% consistency rule applies to lifetime of account", threshold_pct: 40 }
max_position: { minis: "3", micros: "30", scaling: "Scaling plan does not apply in the evaluation" }
min_trading_days: "3"
trading_day_reset: { time: null, tz_as_published: null }
commission_per_contract: { mnq_usd: null, nq_usd: null, basis: null, includes: null }
copy_trading_allowed: conditional
automated_trading_allowed: no
automation_rule_text: "At Funded Futures Family, we don’t permit bot or algorithmic trading. / Bots, algorithms, expert advisors and any automated order entry are not allowed."
vps_allowed: conflict
personal_device_required: yes
status: conflict
not_confirmed_fields: ["commission_per_contract", "trading_day_reset", "vps_allowed (conflict)", "copy_trading_allowed (ambiguity vs automation ban)", "drawdown.stops_trailing_at"]
notes: "Daily Payout Add-On (+$39 per the home page selector) removes the trading-day requirement and consistency rule for payouts. Seven days to activate the funded account."
sources:
  - { field: platforms, url: "https://intercom.help/funded-futures-family/en/articles/15656536-connecting-your-trading-platform", accessed: "2026-10-23" }
  - { field: platforms, url: "https://www.fundedfuturesfamily.com/", accessed: "2026-10-23" }
  - { field: instruments, url: "https://intercom.help/funded-futures-family/en/articles/17708800-prohibited-trading-conduct", accessed: "2026-10-23" }
  - { field: automation, url: "https://intercom.help/funded-futures-family/en/articles/15892413-bots-algorithmic-trading-policy", accessed: "2026-10-23" }
  - { field: automation, url: "https://intercom.help/funded-futures-family/en/articles/17708800-prohibited-trading-conduct", accessed: "2026-10-23" }
  - { field: copy_trading, url: "https://intercom.help/funded-futures-family/en/articles/17708800-prohibited-trading-conduct", accessed: "2026-10-23" }
  - { field: vps_allowed, url: "https://intercom.help/funded-futures-family/en/articles/15892413-bots-algorithmic-trading-policy", accessed: "2026-10-23" }
  - { field: vps_allowed, url: "https://intercom.help/funded-futures-family/en/articles/17708800-prohibited-trading-conduct", accessed: "2026-10-23" }
  - { field: personal_device_required, url: "https://intercom.help/funded-futures-family/en/articles/17155892-device-sharing", accessed: "2026-10-23" }
  - { field: trading_day_reset, url: "https://intercom.help/funded-futures-family/en/articles/15892350-permitted-times-to-trade", accessed: "2026-10-23" }
  - { field: drawdown, url: "https://intercom.help/funded-futures-family/en/articles/15850710-velocity-evaluation", accessed: "2026-10-23" }
  - { field: drawdown, url: "https://intercom.help/funded-futures-family/en/articles/15879485-velocity-drawdown", accessed: "2026-10-23" }
  - { field: daily_loss_limit, url: "https://intercom.help/funded-futures-family/en/articles/15850710-velocity-evaluation", accessed: "2026-10-23" }
  - { field: profit_target_usd, url: "https://intercom.help/funded-futures-family/en/articles/15850710-velocity-evaluation", accessed: "2026-10-23" }
  - { field: consistency_rule, url: "https://intercom.help/funded-futures-family/en/articles/15850710-velocity-evaluation", accessed: "2026-10-23" }
  - { field: max_position, url: "https://intercom.help/funded-futures-family/en/articles/15850710-velocity-evaluation", accessed: "2026-10-23" }
  - { field: min_trading_days, url: "https://intercom.help/funded-futures-family/en/articles/15850710-velocity-evaluation", accessed: "2026-10-23" }
```

#### Velocity — evaluation — 50K

```yaml
firm: funded-futures-family
plan: "Velocity"
phase: evaluation
size_label: "50K"
size_usd: "50000.00"
instruments: [MNQ, NQ]
platforms: ["Tradovate", "WealthCharts", "NinjaTrader (sign in with FFF credentials, Simulation)", "TradingView (via Tradovate add-on)"]
drawdown: { amount_usd: "2250.00", type: "trailing_intraday", stops_trailing_at: "Not stated (page: threshold \"does not move back down\")" }
daily_loss_limit: { amount_usd: null, kind: "none" }
profit_target_usd: "4000.00"
consistency_rule: { text: "40% consistency rule applies to lifetime of account", threshold_pct: 40 }
max_position: { minis: "5", micros: "50", scaling: "Scaling plan does not apply in the evaluation" }
min_trading_days: "3"
trading_day_reset: { time: null, tz_as_published: null }
commission_per_contract: { mnq_usd: null, nq_usd: null, basis: null, includes: null }
copy_trading_allowed: conditional
automated_trading_allowed: no
automation_rule_text: "At Funded Futures Family, we don’t permit bot or algorithmic trading. / Bots, algorithms, expert advisors and any automated order entry are not allowed."
vps_allowed: conflict
personal_device_required: yes
status: conflict
not_confirmed_fields: ["commission_per_contract", "trading_day_reset", "vps_allowed (conflict)", "copy_trading_allowed (ambiguity vs automation ban)", "drawdown.stops_trailing_at"]
notes: "Daily Payout Add-On (+$39 per the home page selector) removes the trading-day requirement and consistency rule for payouts. Seven days to activate the funded account."
sources:
  - { field: platforms, url: "https://intercom.help/funded-futures-family/en/articles/15656536-connecting-your-trading-platform", accessed: "2026-10-23" }
  - { field: platforms, url: "https://www.fundedfuturesfamily.com/", accessed: "2026-10-23" }
  - { field: instruments, url: "https://intercom.help/funded-futures-family/en/articles/17708800-prohibited-trading-conduct", accessed: "2026-10-23" }
  - { field: automation, url: "https://intercom.help/funded-futures-family/en/articles/15892413-bots-algorithmic-trading-policy", accessed: "2026-10-23" }
  - { field: automation, url: "https://intercom.help/funded-futures-family/en/articles/17708800-prohibited-trading-conduct", accessed: "2026-10-23" }
  - { field: copy_trading, url: "https://intercom.help/funded-futures-family/en/articles/17708800-prohibited-trading-conduct", accessed: "2026-10-23" }
  - { field: vps_allowed, url: "https://intercom.help/funded-futures-family/en/articles/15892413-bots-algorithmic-trading-policy", accessed: "2026-10-23" }
  - { field: vps_allowed, url: "https://intercom.help/funded-futures-family/en/articles/17708800-prohibited-trading-conduct", accessed: "2026-10-23" }
  - { field: personal_device_required, url: "https://intercom.help/funded-futures-family/en/articles/17155892-device-sharing", accessed: "2026-10-23" }
  - { field: trading_day_reset, url: "https://intercom.help/funded-futures-family/en/articles/15892350-permitted-times-to-trade", accessed: "2026-10-23" }
  - { field: drawdown, url: "https://intercom.help/funded-futures-family/en/articles/15850710-velocity-evaluation", accessed: "2026-10-23" }
  - { field: drawdown, url: "https://intercom.help/funded-futures-family/en/articles/15879485-velocity-drawdown", accessed: "2026-10-23" }
  - { field: daily_loss_limit, url: "https://intercom.help/funded-futures-family/en/articles/15850710-velocity-evaluation", accessed: "2026-10-23" }
  - { field: profit_target_usd, url: "https://intercom.help/funded-futures-family/en/articles/15850710-velocity-evaluation", accessed: "2026-10-23" }
  - { field: consistency_rule, url: "https://intercom.help/funded-futures-family/en/articles/15850710-velocity-evaluation", accessed: "2026-10-23" }
  - { field: max_position, url: "https://intercom.help/funded-futures-family/en/articles/15850710-velocity-evaluation", accessed: "2026-10-23" }
  - { field: min_trading_days, url: "https://intercom.help/funded-futures-family/en/articles/15850710-velocity-evaluation", accessed: "2026-10-23" }
```

#### Velocity — evaluation — 100K

```yaml
firm: funded-futures-family
plan: "Velocity"
phase: evaluation
size_label: "100K"
size_usd: "100000.00"
instruments: [MNQ, NQ]
platforms: ["Tradovate", "WealthCharts", "NinjaTrader (sign in with FFF credentials, Simulation)", "TradingView (via Tradovate add-on)"]
drawdown: { amount_usd: "3250.00", type: "trailing_intraday", stops_trailing_at: "Not stated (page: threshold \"does not move back down\")" }
daily_loss_limit: { amount_usd: null, kind: "none" }
profit_target_usd: "7000.00"
consistency_rule: { text: "40% consistency rule applies to lifetime of account", threshold_pct: 40 }
max_position: { minis: "10", micros: "100", scaling: "Scaling plan does not apply in the evaluation" }
min_trading_days: "3"
trading_day_reset: { time: null, tz_as_published: null }
commission_per_contract: { mnq_usd: null, nq_usd: null, basis: null, includes: null }
copy_trading_allowed: conditional
automated_trading_allowed: no
automation_rule_text: "At Funded Futures Family, we don’t permit bot or algorithmic trading. / Bots, algorithms, expert advisors and any automated order entry are not allowed."
vps_allowed: conflict
personal_device_required: yes
status: conflict
not_confirmed_fields: ["commission_per_contract", "trading_day_reset", "vps_allowed (conflict)", "copy_trading_allowed (ambiguity vs automation ban)", "drawdown.stops_trailing_at"]
notes: "Daily Payout Add-On (+$39 per the home page selector) removes the trading-day requirement and consistency rule for payouts. Seven days to activate the funded account."
sources:
  - { field: platforms, url: "https://intercom.help/funded-futures-family/en/articles/15656536-connecting-your-trading-platform", accessed: "2026-10-23" }
  - { field: platforms, url: "https://www.fundedfuturesfamily.com/", accessed: "2026-10-23" }
  - { field: instruments, url: "https://intercom.help/funded-futures-family/en/articles/17708800-prohibited-trading-conduct", accessed: "2026-10-23" }
  - { field: automation, url: "https://intercom.help/funded-futures-family/en/articles/15892413-bots-algorithmic-trading-policy", accessed: "2026-10-23" }
  - { field: automation, url: "https://intercom.help/funded-futures-family/en/articles/17708800-prohibited-trading-conduct", accessed: "2026-10-23" }
  - { field: copy_trading, url: "https://intercom.help/funded-futures-family/en/articles/17708800-prohibited-trading-conduct", accessed: "2026-10-23" }
  - { field: vps_allowed, url: "https://intercom.help/funded-futures-family/en/articles/15892413-bots-algorithmic-trading-policy", accessed: "2026-10-23" }
  - { field: vps_allowed, url: "https://intercom.help/funded-futures-family/en/articles/17708800-prohibited-trading-conduct", accessed: "2026-10-23" }
  - { field: personal_device_required, url: "https://intercom.help/funded-futures-family/en/articles/17155892-device-sharing", accessed: "2026-10-23" }
  - { field: trading_day_reset, url: "https://intercom.help/funded-futures-family/en/articles/15892350-permitted-times-to-trade", accessed: "2026-10-23" }
  - { field: drawdown, url: "https://intercom.help/funded-futures-family/en/articles/15850710-velocity-evaluation", accessed: "2026-10-23" }
  - { field: drawdown, url: "https://intercom.help/funded-futures-family/en/articles/15879485-velocity-drawdown", accessed: "2026-10-23" }
  - { field: daily_loss_limit, url: "https://intercom.help/funded-futures-family/en/articles/15850710-velocity-evaluation", accessed: "2026-10-23" }
  - { field: profit_target_usd, url: "https://intercom.help/funded-futures-family/en/articles/15850710-velocity-evaluation", accessed: "2026-10-23" }
  - { field: consistency_rule, url: "https://intercom.help/funded-futures-family/en/articles/15850710-velocity-evaluation", accessed: "2026-10-23" }
  - { field: max_position, url: "https://intercom.help/funded-futures-family/en/articles/15850710-velocity-evaluation", accessed: "2026-10-23" }
  - { field: min_trading_days, url: "https://intercom.help/funded-futures-family/en/articles/15850710-velocity-evaluation", accessed: "2026-10-23" }
```

#### Velocity — evaluation — 150K

```yaml
firm: funded-futures-family
plan: "Velocity"
phase: evaluation
size_label: "150K"
size_usd: "150000.00"
instruments: [MNQ, NQ]
platforms: ["Tradovate", "WealthCharts", "NinjaTrader (sign in with FFF credentials, Simulation)", "TradingView (via Tradovate add-on)"]
drawdown: { amount_usd: "4750.00", type: "trailing_intraday", stops_trailing_at: "Not stated (page: threshold \"does not move back down\")" }
daily_loss_limit: { amount_usd: null, kind: "none" }
profit_target_usd: "10000.00"
consistency_rule: { text: "40% consistency rule applies to lifetime of account", threshold_pct: 40 }
max_position: { minis: "15", micros: "150", scaling: "Scaling plan does not apply in the evaluation" }
min_trading_days: "3"
trading_day_reset: { time: null, tz_as_published: null }
commission_per_contract: { mnq_usd: null, nq_usd: null, basis: null, includes: null }
copy_trading_allowed: conditional
automated_trading_allowed: no
automation_rule_text: "At Funded Futures Family, we don’t permit bot or algorithmic trading. / Bots, algorithms, expert advisors and any automated order entry are not allowed."
vps_allowed: conflict
personal_device_required: yes
status: conflict
not_confirmed_fields: ["commission_per_contract", "trading_day_reset", "vps_allowed (conflict)", "copy_trading_allowed (ambiguity vs automation ban)", "drawdown.stops_trailing_at"]
notes: "Daily Payout Add-On (+$39 per the home page selector) removes the trading-day requirement and consistency rule for payouts. Seven days to activate the funded account."
sources:
  - { field: platforms, url: "https://intercom.help/funded-futures-family/en/articles/15656536-connecting-your-trading-platform", accessed: "2026-10-23" }
  - { field: platforms, url: "https://www.fundedfuturesfamily.com/", accessed: "2026-10-23" }
  - { field: instruments, url: "https://intercom.help/funded-futures-family/en/articles/17708800-prohibited-trading-conduct", accessed: "2026-10-23" }
  - { field: automation, url: "https://intercom.help/funded-futures-family/en/articles/15892413-bots-algorithmic-trading-policy", accessed: "2026-10-23" }
  - { field: automation, url: "https://intercom.help/funded-futures-family/en/articles/17708800-prohibited-trading-conduct", accessed: "2026-10-23" }
  - { field: copy_trading, url: "https://intercom.help/funded-futures-family/en/articles/17708800-prohibited-trading-conduct", accessed: "2026-10-23" }
  - { field: vps_allowed, url: "https://intercom.help/funded-futures-family/en/articles/15892413-bots-algorithmic-trading-policy", accessed: "2026-10-23" }
  - { field: vps_allowed, url: "https://intercom.help/funded-futures-family/en/articles/17708800-prohibited-trading-conduct", accessed: "2026-10-23" }
  - { field: personal_device_required, url: "https://intercom.help/funded-futures-family/en/articles/17155892-device-sharing", accessed: "2026-10-23" }
  - { field: trading_day_reset, url: "https://intercom.help/funded-futures-family/en/articles/15892350-permitted-times-to-trade", accessed: "2026-10-23" }
  - { field: drawdown, url: "https://intercom.help/funded-futures-family/en/articles/15850710-velocity-evaluation", accessed: "2026-10-23" }
  - { field: drawdown, url: "https://intercom.help/funded-futures-family/en/articles/15879485-velocity-drawdown", accessed: "2026-10-23" }
  - { field: daily_loss_limit, url: "https://intercom.help/funded-futures-family/en/articles/15850710-velocity-evaluation", accessed: "2026-10-23" }
  - { field: profit_target_usd, url: "https://intercom.help/funded-futures-family/en/articles/15850710-velocity-evaluation", accessed: "2026-10-23" }
  - { field: consistency_rule, url: "https://intercom.help/funded-futures-family/en/articles/15850710-velocity-evaluation", accessed: "2026-10-23" }
  - { field: max_position, url: "https://intercom.help/funded-futures-family/en/articles/15850710-velocity-evaluation", accessed: "2026-10-23" }
  - { field: min_trading_days, url: "https://intercom.help/funded-futures-family/en/articles/15850710-velocity-evaluation", accessed: "2026-10-23" }
```

#### Velocity — funded — 25K

```yaml
firm: funded-futures-family
plan: "Velocity"
phase: funded
size_label: "25K"
size_usd: "25000.00"
instruments: [MNQ, NQ]
platforms: ["Tradovate", "WealthCharts", "NinjaTrader (sign in with FFF credentials, Simulation)", "TradingView (via Tradovate add-on)"]
drawdown: { amount_usd: "1250.00", type: "trailing_intraday", stops_trailing_at: "Not stated (page: threshold \"remains locked at that level\" once moved higher, ambiguous)" }
daily_loss_limit: { amount_usd: null, kind: "none" }
profit_target_usd: "none"
consistency_rule: { text: "40% Consistency Rule: largest single trading day cannot exceed 40% of total profit when requesting a payout; resets after each approved payout. Daily Payout Add-On: no consistency rule", threshold_pct: 40 }
max_position: { minis: "3 max", micros: "30 max", scaling: "Scaling plan applies (same table as Prime/Premier+/S2F)" }
min_trading_days: null
trading_day_reset: { time: null, tz_as_published: null }
commission_per_contract: { mnq_usd: null, nq_usd: null, basis: null, includes: null }
copy_trading_allowed: conditional
automated_trading_allowed: no
automation_rule_text: "At Funded Futures Family, we don’t permit bot or algorithmic trading. / Bots, algorithms, expert advisors and any automated order entry are not allowed."
vps_allowed: conflict
personal_device_required: yes
status: conflict
not_confirmed_fields: ["commission_per_contract", "trading_day_reset", "vps_allowed (conflict)", "copy_trading_allowed (ambiguity vs automation ban)", "drawdown.stops_trailing_at", "consistency_rule (Velocity card on home page says none in sim funded; help center says 40%; add-on removes it)", "min_trading_days"]
notes: "Minimum 3 trading days to request a payout (none with the Daily Payout Add-On), minimum $200 profit per trading day, profit requirement per payout $1,500 / $3,000 / $6,000 / $9,000, max payout per request (standard) $750 / $1,250 / $2,250 / $3,250, 90/10 split. Funded uses the same intraday trailing drawdown as the evaluation. The home page card says \"No consistency in sim funded\" (with add-on unlocks)."
sources:
  - { field: platforms, url: "https://intercom.help/funded-futures-family/en/articles/15656536-connecting-your-trading-platform", accessed: "2026-10-23" }
  - { field: platforms, url: "https://www.fundedfuturesfamily.com/", accessed: "2026-10-23" }
  - { field: instruments, url: "https://intercom.help/funded-futures-family/en/articles/17708800-prohibited-trading-conduct", accessed: "2026-10-23" }
  - { field: automation, url: "https://intercom.help/funded-futures-family/en/articles/15892413-bots-algorithmic-trading-policy", accessed: "2026-10-23" }
  - { field: automation, url: "https://intercom.help/funded-futures-family/en/articles/17708800-prohibited-trading-conduct", accessed: "2026-10-23" }
  - { field: copy_trading, url: "https://intercom.help/funded-futures-family/en/articles/17708800-prohibited-trading-conduct", accessed: "2026-10-23" }
  - { field: vps_allowed, url: "https://intercom.help/funded-futures-family/en/articles/15892413-bots-algorithmic-trading-policy", accessed: "2026-10-23" }
  - { field: vps_allowed, url: "https://intercom.help/funded-futures-family/en/articles/17708800-prohibited-trading-conduct", accessed: "2026-10-23" }
  - { field: personal_device_required, url: "https://intercom.help/funded-futures-family/en/articles/17155892-device-sharing", accessed: "2026-10-23" }
  - { field: trading_day_reset, url: "https://intercom.help/funded-futures-family/en/articles/15892350-permitted-times-to-trade", accessed: "2026-10-23" }
  - { field: drawdown, url: "https://intercom.help/funded-futures-family/en/articles/15851015-velocity-funded-account", accessed: "2026-10-23" }
  - { field: drawdown, url: "https://intercom.help/funded-futures-family/en/articles/15879485-velocity-drawdown", accessed: "2026-10-23" }
  - { field: daily_loss_limit, url: "https://intercom.help/funded-futures-family/en/articles/15851015-velocity-funded-account", accessed: "2026-10-23" }
  - { field: consistency_rule, url: "https://intercom.help/funded-futures-family/en/articles/15879043-velocity-consistency", accessed: "2026-10-23" }
  - { field: consistency_rule, url: "https://intercom.help/funded-futures-family/en/articles/15878902-velocity-payout-requirements", accessed: "2026-10-23" }
  - { field: consistency_rule, url: "https://www.fundedfuturesfamily.com/", accessed: "2026-10-23" }
  - { field: max_position, url: "https://intercom.help/funded-futures-family/en/articles/15851015-velocity-funded-account", accessed: "2026-10-23" }
  - { field: max_position, url: "https://intercom.help/funded-futures-family/en/articles/16310772-velocity-scaling-plan", accessed: "2026-10-23" }
```

#### Velocity — funded — 50K

```yaml
firm: funded-futures-family
plan: "Velocity"
phase: funded
size_label: "50K"
size_usd: "50000.00"
instruments: [MNQ, NQ]
platforms: ["Tradovate", "WealthCharts", "NinjaTrader (sign in with FFF credentials, Simulation)", "TradingView (via Tradovate add-on)"]
drawdown: { amount_usd: "2250.00", type: "trailing_intraday", stops_trailing_at: "Not stated (page: threshold \"remains locked at that level\" once moved higher, ambiguous)" }
daily_loss_limit: { amount_usd: null, kind: "none" }
profit_target_usd: "none"
consistency_rule: { text: "40% Consistency Rule: largest single trading day cannot exceed 40% of total profit when requesting a payout; resets after each approved payout. Daily Payout Add-On: no consistency rule", threshold_pct: 40 }
max_position: { minis: "5 max", micros: "50 max", scaling: "Scaling plan applies (same table as Prime/Premier+/S2F)" }
min_trading_days: null
trading_day_reset: { time: null, tz_as_published: null }
commission_per_contract: { mnq_usd: null, nq_usd: null, basis: null, includes: null }
copy_trading_allowed: conditional
automated_trading_allowed: no
automation_rule_text: "At Funded Futures Family, we don’t permit bot or algorithmic trading. / Bots, algorithms, expert advisors and any automated order entry are not allowed."
vps_allowed: conflict
personal_device_required: yes
status: conflict
not_confirmed_fields: ["commission_per_contract", "trading_day_reset", "vps_allowed (conflict)", "copy_trading_allowed (ambiguity vs automation ban)", "drawdown.stops_trailing_at", "consistency_rule (Velocity card on home page says none in sim funded; help center says 40%; add-on removes it)", "min_trading_days"]
notes: "Minimum 3 trading days to request a payout (none with the Daily Payout Add-On), minimum $200 profit per trading day, profit requirement per payout $1,500 / $3,000 / $6,000 / $9,000, max payout per request (standard) $750 / $1,250 / $2,250 / $3,250, 90/10 split. Funded uses the same intraday trailing drawdown as the evaluation. The home page card says \"No consistency in sim funded\" (with add-on unlocks)."
sources:
  - { field: platforms, url: "https://intercom.help/funded-futures-family/en/articles/15656536-connecting-your-trading-platform", accessed: "2026-10-23" }
  - { field: platforms, url: "https://www.fundedfuturesfamily.com/", accessed: "2026-10-23" }
  - { field: instruments, url: "https://intercom.help/funded-futures-family/en/articles/17708800-prohibited-trading-conduct", accessed: "2026-10-23" }
  - { field: automation, url: "https://intercom.help/funded-futures-family/en/articles/15892413-bots-algorithmic-trading-policy", accessed: "2026-10-23" }
  - { field: automation, url: "https://intercom.help/funded-futures-family/en/articles/17708800-prohibited-trading-conduct", accessed: "2026-10-23" }
  - { field: copy_trading, url: "https://intercom.help/funded-futures-family/en/articles/17708800-prohibited-trading-conduct", accessed: "2026-10-23" }
  - { field: vps_allowed, url: "https://intercom.help/funded-futures-family/en/articles/15892413-bots-algorithmic-trading-policy", accessed: "2026-10-23" }
  - { field: vps_allowed, url: "https://intercom.help/funded-futures-family/en/articles/17708800-prohibited-trading-conduct", accessed: "2026-10-23" }
  - { field: personal_device_required, url: "https://intercom.help/funded-futures-family/en/articles/17155892-device-sharing", accessed: "2026-10-23" }
  - { field: trading_day_reset, url: "https://intercom.help/funded-futures-family/en/articles/15892350-permitted-times-to-trade", accessed: "2026-10-23" }
  - { field: drawdown, url: "https://intercom.help/funded-futures-family/en/articles/15851015-velocity-funded-account", accessed: "2026-10-23" }
  - { field: drawdown, url: "https://intercom.help/funded-futures-family/en/articles/15879485-velocity-drawdown", accessed: "2026-10-23" }
  - { field: daily_loss_limit, url: "https://intercom.help/funded-futures-family/en/articles/15851015-velocity-funded-account", accessed: "2026-10-23" }
  - { field: consistency_rule, url: "https://intercom.help/funded-futures-family/en/articles/15879043-velocity-consistency", accessed: "2026-10-23" }
  - { field: consistency_rule, url: "https://intercom.help/funded-futures-family/en/articles/15878902-velocity-payout-requirements", accessed: "2026-10-23" }
  - { field: consistency_rule, url: "https://www.fundedfuturesfamily.com/", accessed: "2026-10-23" }
  - { field: max_position, url: "https://intercom.help/funded-futures-family/en/articles/15851015-velocity-funded-account", accessed: "2026-10-23" }
  - { field: max_position, url: "https://intercom.help/funded-futures-family/en/articles/16310772-velocity-scaling-plan", accessed: "2026-10-23" }
```

#### Velocity — funded — 100K

```yaml
firm: funded-futures-family
plan: "Velocity"
phase: funded
size_label: "100K"
size_usd: "100000.00"
instruments: [MNQ, NQ]
platforms: ["Tradovate", "WealthCharts", "NinjaTrader (sign in with FFF credentials, Simulation)", "TradingView (via Tradovate add-on)"]
drawdown: { amount_usd: "3250.00", type: "trailing_intraday", stops_trailing_at: "Not stated (page: threshold \"remains locked at that level\" once moved higher, ambiguous)" }
daily_loss_limit: { amount_usd: null, kind: "none" }
profit_target_usd: "none"
consistency_rule: { text: "40% Consistency Rule: largest single trading day cannot exceed 40% of total profit when requesting a payout; resets after each approved payout. Daily Payout Add-On: no consistency rule", threshold_pct: 40 }
max_position: { minis: "10 max", micros: "100 max", scaling: "Scaling plan applies (same table as Prime/Premier+/S2F)" }
min_trading_days: null
trading_day_reset: { time: null, tz_as_published: null }
commission_per_contract: { mnq_usd: null, nq_usd: null, basis: null, includes: null }
copy_trading_allowed: conditional
automated_trading_allowed: no
automation_rule_text: "At Funded Futures Family, we don’t permit bot or algorithmic trading. / Bots, algorithms, expert advisors and any automated order entry are not allowed."
vps_allowed: conflict
personal_device_required: yes
status: conflict
not_confirmed_fields: ["commission_per_contract", "trading_day_reset", "vps_allowed (conflict)", "copy_trading_allowed (ambiguity vs automation ban)", "drawdown.stops_trailing_at", "consistency_rule (Velocity card on home page says none in sim funded; help center says 40%; add-on removes it)", "min_trading_days"]
notes: "Minimum 3 trading days to request a payout (none with the Daily Payout Add-On), minimum $200 profit per trading day, profit requirement per payout $1,500 / $3,000 / $6,000 / $9,000, max payout per request (standard) $750 / $1,250 / $2,250 / $3,250, 90/10 split. Funded uses the same intraday trailing drawdown as the evaluation. The home page card says \"No consistency in sim funded\" (with add-on unlocks)."
sources:
  - { field: platforms, url: "https://intercom.help/funded-futures-family/en/articles/15656536-connecting-your-trading-platform", accessed: "2026-10-23" }
  - { field: platforms, url: "https://www.fundedfuturesfamily.com/", accessed: "2026-10-23" }
  - { field: instruments, url: "https://intercom.help/funded-futures-family/en/articles/17708800-prohibited-trading-conduct", accessed: "2026-10-23" }
  - { field: automation, url: "https://intercom.help/funded-futures-family/en/articles/15892413-bots-algorithmic-trading-policy", accessed: "2026-10-23" }
  - { field: automation, url: "https://intercom.help/funded-futures-family/en/articles/17708800-prohibited-trading-conduct", accessed: "2026-10-23" }
  - { field: copy_trading, url: "https://intercom.help/funded-futures-family/en/articles/17708800-prohibited-trading-conduct", accessed: "2026-10-23" }
  - { field: vps_allowed, url: "https://intercom.help/funded-futures-family/en/articles/15892413-bots-algorithmic-trading-policy", accessed: "2026-10-23" }
  - { field: vps_allowed, url: "https://intercom.help/funded-futures-family/en/articles/17708800-prohibited-trading-conduct", accessed: "2026-10-23" }
  - { field: personal_device_required, url: "https://intercom.help/funded-futures-family/en/articles/17155892-device-sharing", accessed: "2026-10-23" }
  - { field: trading_day_reset, url: "https://intercom.help/funded-futures-family/en/articles/15892350-permitted-times-to-trade", accessed: "2026-10-23" }
  - { field: drawdown, url: "https://intercom.help/funded-futures-family/en/articles/15851015-velocity-funded-account", accessed: "2026-10-23" }
  - { field: drawdown, url: "https://intercom.help/funded-futures-family/en/articles/15879485-velocity-drawdown", accessed: "2026-10-23" }
  - { field: daily_loss_limit, url: "https://intercom.help/funded-futures-family/en/articles/15851015-velocity-funded-account", accessed: "2026-10-23" }
  - { field: consistency_rule, url: "https://intercom.help/funded-futures-family/en/articles/15879043-velocity-consistency", accessed: "2026-10-23" }
  - { field: consistency_rule, url: "https://intercom.help/funded-futures-family/en/articles/15878902-velocity-payout-requirements", accessed: "2026-10-23" }
  - { field: consistency_rule, url: "https://www.fundedfuturesfamily.com/", accessed: "2026-10-23" }
  - { field: max_position, url: "https://intercom.help/funded-futures-family/en/articles/15851015-velocity-funded-account", accessed: "2026-10-23" }
  - { field: max_position, url: "https://intercom.help/funded-futures-family/en/articles/16310772-velocity-scaling-plan", accessed: "2026-10-23" }
```

#### Velocity — funded — 150K

```yaml
firm: funded-futures-family
plan: "Velocity"
phase: funded
size_label: "150K"
size_usd: "150000.00"
instruments: [MNQ, NQ]
platforms: ["Tradovate", "WealthCharts", "NinjaTrader (sign in with FFF credentials, Simulation)", "TradingView (via Tradovate add-on)"]
drawdown: { amount_usd: "4750.00", type: "trailing_intraday", stops_trailing_at: "Not stated (page: threshold \"remains locked at that level\" once moved higher, ambiguous)" }
daily_loss_limit: { amount_usd: null, kind: "none" }
profit_target_usd: "none"
consistency_rule: { text: "40% Consistency Rule: largest single trading day cannot exceed 40% of total profit when requesting a payout; resets after each approved payout. Daily Payout Add-On: no consistency rule", threshold_pct: 40 }
max_position: { minis: "15 max", micros: "150 max", scaling: "Scaling plan applies (same table as Prime/Premier+/S2F)" }
min_trading_days: null
trading_day_reset: { time: null, tz_as_published: null }
commission_per_contract: { mnq_usd: null, nq_usd: null, basis: null, includes: null }
copy_trading_allowed: conditional
automated_trading_allowed: no
automation_rule_text: "At Funded Futures Family, we don’t permit bot or algorithmic trading. / Bots, algorithms, expert advisors and any automated order entry are not allowed."
vps_allowed: conflict
personal_device_required: yes
status: conflict
not_confirmed_fields: ["commission_per_contract", "trading_day_reset", "vps_allowed (conflict)", "copy_trading_allowed (ambiguity vs automation ban)", "drawdown.stops_trailing_at", "consistency_rule (Velocity card on home page says none in sim funded; help center says 40%; add-on removes it)", "min_trading_days"]
notes: "Minimum 3 trading days to request a payout (none with the Daily Payout Add-On), minimum $200 profit per trading day, profit requirement per payout $1,500 / $3,000 / $6,000 / $9,000, max payout per request (standard) $750 / $1,250 / $2,250 / $3,250, 90/10 split. Funded uses the same intraday trailing drawdown as the evaluation. The home page card says \"No consistency in sim funded\" (with add-on unlocks)."
sources:
  - { field: platforms, url: "https://intercom.help/funded-futures-family/en/articles/15656536-connecting-your-trading-platform", accessed: "2026-10-23" }
  - { field: platforms, url: "https://www.fundedfuturesfamily.com/", accessed: "2026-10-23" }
  - { field: instruments, url: "https://intercom.help/funded-futures-family/en/articles/17708800-prohibited-trading-conduct", accessed: "2026-10-23" }
  - { field: automation, url: "https://intercom.help/funded-futures-family/en/articles/15892413-bots-algorithmic-trading-policy", accessed: "2026-10-23" }
  - { field: automation, url: "https://intercom.help/funded-futures-family/en/articles/17708800-prohibited-trading-conduct", accessed: "2026-10-23" }
  - { field: copy_trading, url: "https://intercom.help/funded-futures-family/en/articles/17708800-prohibited-trading-conduct", accessed: "2026-10-23" }
  - { field: vps_allowed, url: "https://intercom.help/funded-futures-family/en/articles/15892413-bots-algorithmic-trading-policy", accessed: "2026-10-23" }
  - { field: vps_allowed, url: "https://intercom.help/funded-futures-family/en/articles/17708800-prohibited-trading-conduct", accessed: "2026-10-23" }
  - { field: personal_device_required, url: "https://intercom.help/funded-futures-family/en/articles/17155892-device-sharing", accessed: "2026-10-23" }
  - { field: trading_day_reset, url: "https://intercom.help/funded-futures-family/en/articles/15892350-permitted-times-to-trade", accessed: "2026-10-23" }
  - { field: drawdown, url: "https://intercom.help/funded-futures-family/en/articles/15851015-velocity-funded-account", accessed: "2026-10-23" }
  - { field: drawdown, url: "https://intercom.help/funded-futures-family/en/articles/15879485-velocity-drawdown", accessed: "2026-10-23" }
  - { field: daily_loss_limit, url: "https://intercom.help/funded-futures-family/en/articles/15851015-velocity-funded-account", accessed: "2026-10-23" }
  - { field: consistency_rule, url: "https://intercom.help/funded-futures-family/en/articles/15879043-velocity-consistency", accessed: "2026-10-23" }
  - { field: consistency_rule, url: "https://intercom.help/funded-futures-family/en/articles/15878902-velocity-payout-requirements", accessed: "2026-10-23" }
  - { field: consistency_rule, url: "https://www.fundedfuturesfamily.com/", accessed: "2026-10-23" }
  - { field: max_position, url: "https://intercom.help/funded-futures-family/en/articles/15851015-velocity-funded-account", accessed: "2026-10-23" }
  - { field: max_position, url: "https://intercom.help/funded-futures-family/en/articles/16310772-velocity-scaling-plan", accessed: "2026-10-23" }
```

#### Straight to Funded (S2F) — funded — 25K

```yaml
firm: funded-futures-family
plan: "Straight to Funded (S2F)"
phase: funded
size_label: "25K"
size_usd: "25000.00"
instruments: [MNQ, NQ]
platforms: ["Tradovate", "WealthCharts", "NinjaTrader (sign in with FFF credentials, Simulation)", "TradingView (via Tradovate add-on)"]
drawdown: { amount_usd: "1000.00", type: "trailing_eod", stops_trailing_at: "Locks at the starting balance after exceeding the Initial Trailing Balance ($26,000 / $52,000 / $103,000 / $154,500)" }
daily_loss_limit: { amount_usd: null, kind: "none" }
profit_target_usd: "none"
consistency_rule: { text: "25% consistency rule: largest single trading day cannot exceed 25% of total profit when requesting a payout; resets after each approved payout. The home page plan card says \"45% consistency\"", threshold_pct: 25 }
max_position: { minis: "1 (table) vs 3 (scaling max)", micros: "10 (table) vs 30 (scaling max)", scaling: "Scaling plan applies to all funded accounts" }
min_trading_days: "7"
trading_day_reset: { time: null, tz_as_published: null }
commission_per_contract: { mnq_usd: null, nq_usd: null, basis: null, includes: null }
copy_trading_allowed: conditional
automated_trading_allowed: no
automation_rule_text: "At Funded Futures Family, we don’t permit bot or algorithmic trading. / Bots, algorithms, expert advisors and any automated order entry are not allowed."
vps_allowed: conflict
personal_device_required: yes
status: conflict
not_confirmed_fields: ["commission_per_contract", "trading_day_reset", "vps_allowed (conflict)", "copy_trading_allowed (ambiguity vs automation ban)", "consistency_rule (conflict: 25% help center vs 45% home page)", "max_position (25K: table 1 mini vs scaling plan max 3 minis)"]
notes: "No evaluation phase. Payout needs 7 trading days of at least $200 profit, profit goal between payouts $1,500 / $3,000 / $6,000 / $9,000 (payout 1) and $1,000 / $2,000 / $3,000 / $4,500 (payout 2+), max payout per payout $1,000-$3,500 by size, 90/10 split. One-time fee, no resets on S2F accounts. Max 5 active funded accounts."
sources:
  - { field: platforms, url: "https://intercom.help/funded-futures-family/en/articles/15656536-connecting-your-trading-platform", accessed: "2026-10-23" }
  - { field: platforms, url: "https://www.fundedfuturesfamily.com/", accessed: "2026-10-23" }
  - { field: instruments, url: "https://intercom.help/funded-futures-family/en/articles/17708800-prohibited-trading-conduct", accessed: "2026-10-23" }
  - { field: automation, url: "https://intercom.help/funded-futures-family/en/articles/15892413-bots-algorithmic-trading-policy", accessed: "2026-10-23" }
  - { field: automation, url: "https://intercom.help/funded-futures-family/en/articles/17708800-prohibited-trading-conduct", accessed: "2026-10-23" }
  - { field: copy_trading, url: "https://intercom.help/funded-futures-family/en/articles/17708800-prohibited-trading-conduct", accessed: "2026-10-23" }
  - { field: vps_allowed, url: "https://intercom.help/funded-futures-family/en/articles/15892413-bots-algorithmic-trading-policy", accessed: "2026-10-23" }
  - { field: vps_allowed, url: "https://intercom.help/funded-futures-family/en/articles/17708800-prohibited-trading-conduct", accessed: "2026-10-23" }
  - { field: personal_device_required, url: "https://intercom.help/funded-futures-family/en/articles/17155892-device-sharing", accessed: "2026-10-23" }
  - { field: trading_day_reset, url: "https://intercom.help/funded-futures-family/en/articles/15892350-permitted-times-to-trade", accessed: "2026-10-23" }
  - { field: drawdown, url: "https://intercom.help/funded-futures-family/en/articles/15879855-straight-to-funded-s2f-funded-account", accessed: "2026-10-23" }
  - { field: drawdown, url: "https://intercom.help/funded-futures-family/en/articles/15880602-straight-to-funded-s2f-drawdown", accessed: "2026-10-23" }
  - { field: daily_loss_limit, url: "https://intercom.help/funded-futures-family/en/articles/15879855-straight-to-funded-s2f-funded-account", accessed: "2026-10-23" }
  - { field: consistency_rule, url: "https://intercom.help/funded-futures-family/en/articles/15879855-straight-to-funded-s2f-funded-account", accessed: "2026-10-23" }
  - { field: consistency_rule, url: "https://intercom.help/funded-futures-family/en/articles/15879981-straight-to-funded-s2f-payout-requirements", accessed: "2026-10-23" }
  - { field: consistency_rule, url: "https://intercom.help/funded-futures-family/en/articles/15880555-straight-to-funded-s2f-consistency-percentage", accessed: "2026-10-23" }
  - { field: consistency_rule, url: "https://www.fundedfuturesfamily.com/", accessed: "2026-10-23" }
  - { field: max_position, url: "https://intercom.help/funded-futures-family/en/articles/15879855-straight-to-funded-s2f-funded-account", accessed: "2026-10-23" }
  - { field: max_position, url: "https://intercom.help/funded-futures-family/en/articles/16310780-straight-to-funded-s2f-scaling-plan", accessed: "2026-10-23" }
  - { field: min_trading_days, url: "https://intercom.help/funded-futures-family/en/articles/15879855-straight-to-funded-s2f-funded-account", accessed: "2026-10-23" }
  - { field: min_trading_days, url: "https://intercom.help/funded-futures-family/en/articles/15879981-straight-to-funded-s2f-payout-requirements", accessed: "2026-10-23" }
```

#### Straight to Funded (S2F) — funded — 50K

```yaml
firm: funded-futures-family
plan: "Straight to Funded (S2F)"
phase: funded
size_label: "50K"
size_usd: "50000.00"
instruments: [MNQ, NQ]
platforms: ["Tradovate", "WealthCharts", "NinjaTrader (sign in with FFF credentials, Simulation)", "TradingView (via Tradovate add-on)"]
drawdown: { amount_usd: "2000.00", type: "trailing_eod", stops_trailing_at: "Locks at the starting balance after exceeding the Initial Trailing Balance ($26,000 / $52,000 / $103,000 / $154,500)" }
daily_loss_limit: { amount_usd: null, kind: "none" }
profit_target_usd: "none"
consistency_rule: { text: "25% consistency rule: largest single trading day cannot exceed 25% of total profit when requesting a payout; resets after each approved payout. The home page plan card says \"45% consistency\"", threshold_pct: 25 }
max_position: { minis: "5", micros: "5", scaling: "Scaling plan applies to all funded accounts" }
min_trading_days: "7"
trading_day_reset: { time: null, tz_as_published: null }
commission_per_contract: { mnq_usd: null, nq_usd: null, basis: null, includes: null }
copy_trading_allowed: conditional
automated_trading_allowed: no
automation_rule_text: "At Funded Futures Family, we don’t permit bot or algorithmic trading. / Bots, algorithms, expert advisors and any automated order entry are not allowed."
vps_allowed: conflict
personal_device_required: yes
status: conflict
not_confirmed_fields: ["commission_per_contract", "trading_day_reset", "vps_allowed (conflict)", "copy_trading_allowed (ambiguity vs automation ban)", "consistency_rule (conflict: 25% help center vs 45% home page)"]
notes: "No evaluation phase. Payout needs 7 trading days of at least $200 profit, profit goal between payouts $1,500 / $3,000 / $6,000 / $9,000 (payout 1) and $1,000 / $2,000 / $3,000 / $4,500 (payout 2+), max payout per payout $1,000-$3,500 by size, 90/10 split. One-time fee, no resets on S2F accounts. Max 5 active funded accounts."
sources:
  - { field: platforms, url: "https://intercom.help/funded-futures-family/en/articles/15656536-connecting-your-trading-platform", accessed: "2026-10-23" }
  - { field: platforms, url: "https://www.fundedfuturesfamily.com/", accessed: "2026-10-23" }
  - { field: instruments, url: "https://intercom.help/funded-futures-family/en/articles/17708800-prohibited-trading-conduct", accessed: "2026-10-23" }
  - { field: automation, url: "https://intercom.help/funded-futures-family/en/articles/15892413-bots-algorithmic-trading-policy", accessed: "2026-10-23" }
  - { field: automation, url: "https://intercom.help/funded-futures-family/en/articles/17708800-prohibited-trading-conduct", accessed: "2026-10-23" }
  - { field: copy_trading, url: "https://intercom.help/funded-futures-family/en/articles/17708800-prohibited-trading-conduct", accessed: "2026-10-23" }
  - { field: vps_allowed, url: "https://intercom.help/funded-futures-family/en/articles/15892413-bots-algorithmic-trading-policy", accessed: "2026-10-23" }
  - { field: vps_allowed, url: "https://intercom.help/funded-futures-family/en/articles/17708800-prohibited-trading-conduct", accessed: "2026-10-23" }
  - { field: personal_device_required, url: "https://intercom.help/funded-futures-family/en/articles/17155892-device-sharing", accessed: "2026-10-23" }
  - { field: trading_day_reset, url: "https://intercom.help/funded-futures-family/en/articles/15892350-permitted-times-to-trade", accessed: "2026-10-23" }
  - { field: drawdown, url: "https://intercom.help/funded-futures-family/en/articles/15879855-straight-to-funded-s2f-funded-account", accessed: "2026-10-23" }
  - { field: drawdown, url: "https://intercom.help/funded-futures-family/en/articles/15880602-straight-to-funded-s2f-drawdown", accessed: "2026-10-23" }
  - { field: daily_loss_limit, url: "https://intercom.help/funded-futures-family/en/articles/15879855-straight-to-funded-s2f-funded-account", accessed: "2026-10-23" }
  - { field: consistency_rule, url: "https://intercom.help/funded-futures-family/en/articles/15879855-straight-to-funded-s2f-funded-account", accessed: "2026-10-23" }
  - { field: consistency_rule, url: "https://intercom.help/funded-futures-family/en/articles/15879981-straight-to-funded-s2f-payout-requirements", accessed: "2026-10-23" }
  - { field: consistency_rule, url: "https://intercom.help/funded-futures-family/en/articles/15880555-straight-to-funded-s2f-consistency-percentage", accessed: "2026-10-23" }
  - { field: consistency_rule, url: "https://www.fundedfuturesfamily.com/", accessed: "2026-10-23" }
  - { field: max_position, url: "https://intercom.help/funded-futures-family/en/articles/15879855-straight-to-funded-s2f-funded-account", accessed: "2026-10-23" }
  - { field: max_position, url: "https://intercom.help/funded-futures-family/en/articles/16310780-straight-to-funded-s2f-scaling-plan", accessed: "2026-10-23" }
  - { field: min_trading_days, url: "https://intercom.help/funded-futures-family/en/articles/15879855-straight-to-funded-s2f-funded-account", accessed: "2026-10-23" }
  - { field: min_trading_days, url: "https://intercom.help/funded-futures-family/en/articles/15879981-straight-to-funded-s2f-payout-requirements", accessed: "2026-10-23" }
```

#### Straight to Funded (S2F) — funded — 100K

```yaml
firm: funded-futures-family
plan: "Straight to Funded (S2F)"
phase: funded
size_label: "100K"
size_usd: "100000.00"
instruments: [MNQ, NQ]
platforms: ["Tradovate", "WealthCharts", "NinjaTrader (sign in with FFF credentials, Simulation)", "TradingView (via Tradovate add-on)"]
drawdown: { amount_usd: "3000.00", type: "trailing_eod", stops_trailing_at: "Locks at the starting balance after exceeding the Initial Trailing Balance ($26,000 / $52,000 / $103,000 / $154,500)" }
daily_loss_limit: { amount_usd: null, kind: "none" }
profit_target_usd: "none"
consistency_rule: { text: "25% consistency rule: largest single trading day cannot exceed 25% of total profit when requesting a payout; resets after each approved payout. The home page plan card says \"45% consistency\"", threshold_pct: 25 }
max_position: { minis: "10", micros: "10", scaling: "Scaling plan applies to all funded accounts" }
min_trading_days: "7"
trading_day_reset: { time: null, tz_as_published: null }
commission_per_contract: { mnq_usd: null, nq_usd: null, basis: null, includes: null }
copy_trading_allowed: conditional
automated_trading_allowed: no
automation_rule_text: "At Funded Futures Family, we don’t permit bot or algorithmic trading. / Bots, algorithms, expert advisors and any automated order entry are not allowed."
vps_allowed: conflict
personal_device_required: yes
status: conflict
not_confirmed_fields: ["commission_per_contract", "trading_day_reset", "vps_allowed (conflict)", "copy_trading_allowed (ambiguity vs automation ban)", "consistency_rule (conflict: 25% help center vs 45% home page)"]
notes: "No evaluation phase. Payout needs 7 trading days of at least $200 profit, profit goal between payouts $1,500 / $3,000 / $6,000 / $9,000 (payout 1) and $1,000 / $2,000 / $3,000 / $4,500 (payout 2+), max payout per payout $1,000-$3,500 by size, 90/10 split. One-time fee, no resets on S2F accounts. Max 5 active funded accounts."
sources:
  - { field: platforms, url: "https://intercom.help/funded-futures-family/en/articles/15656536-connecting-your-trading-platform", accessed: "2026-10-23" }
  - { field: platforms, url: "https://www.fundedfuturesfamily.com/", accessed: "2026-10-23" }
  - { field: instruments, url: "https://intercom.help/funded-futures-family/en/articles/17708800-prohibited-trading-conduct", accessed: "2026-10-23" }
  - { field: automation, url: "https://intercom.help/funded-futures-family/en/articles/15892413-bots-algorithmic-trading-policy", accessed: "2026-10-23" }
  - { field: automation, url: "https://intercom.help/funded-futures-family/en/articles/17708800-prohibited-trading-conduct", accessed: "2026-10-23" }
  - { field: copy_trading, url: "https://intercom.help/funded-futures-family/en/articles/17708800-prohibited-trading-conduct", accessed: "2026-10-23" }
  - { field: vps_allowed, url: "https://intercom.help/funded-futures-family/en/articles/15892413-bots-algorithmic-trading-policy", accessed: "2026-10-23" }
  - { field: vps_allowed, url: "https://intercom.help/funded-futures-family/en/articles/17708800-prohibited-trading-conduct", accessed: "2026-10-23" }
  - { field: personal_device_required, url: "https://intercom.help/funded-futures-family/en/articles/17155892-device-sharing", accessed: "2026-10-23" }
  - { field: trading_day_reset, url: "https://intercom.help/funded-futures-family/en/articles/15892350-permitted-times-to-trade", accessed: "2026-10-23" }
  - { field: drawdown, url: "https://intercom.help/funded-futures-family/en/articles/15879855-straight-to-funded-s2f-funded-account", accessed: "2026-10-23" }
  - { field: drawdown, url: "https://intercom.help/funded-futures-family/en/articles/15880602-straight-to-funded-s2f-drawdown", accessed: "2026-10-23" }
  - { field: daily_loss_limit, url: "https://intercom.help/funded-futures-family/en/articles/15879855-straight-to-funded-s2f-funded-account", accessed: "2026-10-23" }
  - { field: consistency_rule, url: "https://intercom.help/funded-futures-family/en/articles/15879855-straight-to-funded-s2f-funded-account", accessed: "2026-10-23" }
  - { field: consistency_rule, url: "https://intercom.help/funded-futures-family/en/articles/15879981-straight-to-funded-s2f-payout-requirements", accessed: "2026-10-23" }
  - { field: consistency_rule, url: "https://intercom.help/funded-futures-family/en/articles/15880555-straight-to-funded-s2f-consistency-percentage", accessed: "2026-10-23" }
  - { field: consistency_rule, url: "https://www.fundedfuturesfamily.com/", accessed: "2026-10-23" }
  - { field: max_position, url: "https://intercom.help/funded-futures-family/en/articles/15879855-straight-to-funded-s2f-funded-account", accessed: "2026-10-23" }
  - { field: max_position, url: "https://intercom.help/funded-futures-family/en/articles/16310780-straight-to-funded-s2f-scaling-plan", accessed: "2026-10-23" }
  - { field: min_trading_days, url: "https://intercom.help/funded-futures-family/en/articles/15879855-straight-to-funded-s2f-funded-account", accessed: "2026-10-23" }
  - { field: min_trading_days, url: "https://intercom.help/funded-futures-family/en/articles/15879981-straight-to-funded-s2f-payout-requirements", accessed: "2026-10-23" }
```

#### Straight to Funded (S2F) — funded — 150K

```yaml
firm: funded-futures-family
plan: "Straight to Funded (S2F)"
phase: funded
size_label: "150K"
size_usd: "150000.00"
instruments: [MNQ, NQ]
platforms: ["Tradovate", "WealthCharts", "NinjaTrader (sign in with FFF credentials, Simulation)", "TradingView (via Tradovate add-on)"]
drawdown: { amount_usd: "4500.00", type: "trailing_eod", stops_trailing_at: "Locks at the starting balance after exceeding the Initial Trailing Balance ($26,000 / $52,000 / $103,000 / $154,500)" }
daily_loss_limit: { amount_usd: null, kind: "none" }
profit_target_usd: "none"
consistency_rule: { text: "25% consistency rule: largest single trading day cannot exceed 25% of total profit when requesting a payout; resets after each approved payout. The home page plan card says \"45% consistency\"", threshold_pct: 25 }
max_position: { minis: "15", micros: "15", scaling: "Scaling plan applies to all funded accounts" }
min_trading_days: "7"
trading_day_reset: { time: null, tz_as_published: null }
commission_per_contract: { mnq_usd: null, nq_usd: null, basis: null, includes: null }
copy_trading_allowed: conditional
automated_trading_allowed: no
automation_rule_text: "At Funded Futures Family, we don’t permit bot or algorithmic trading. / Bots, algorithms, expert advisors and any automated order entry are not allowed."
vps_allowed: conflict
personal_device_required: yes
status: conflict
not_confirmed_fields: ["commission_per_contract", "trading_day_reset", "vps_allowed (conflict)", "copy_trading_allowed (ambiguity vs automation ban)", "consistency_rule (conflict: 25% help center vs 45% home page)"]
notes: "No evaluation phase. Payout needs 7 trading days of at least $200 profit, profit goal between payouts $1,500 / $3,000 / $6,000 / $9,000 (payout 1) and $1,000 / $2,000 / $3,000 / $4,500 (payout 2+), max payout per payout $1,000-$3,500 by size, 90/10 split. One-time fee, no resets on S2F accounts. Max 5 active funded accounts."
sources:
  - { field: platforms, url: "https://intercom.help/funded-futures-family/en/articles/15656536-connecting-your-trading-platform", accessed: "2026-10-23" }
  - { field: platforms, url: "https://www.fundedfuturesfamily.com/", accessed: "2026-10-23" }
  - { field: instruments, url: "https://intercom.help/funded-futures-family/en/articles/17708800-prohibited-trading-conduct", accessed: "2026-10-23" }
  - { field: automation, url: "https://intercom.help/funded-futures-family/en/articles/15892413-bots-algorithmic-trading-policy", accessed: "2026-10-23" }
  - { field: automation, url: "https://intercom.help/funded-futures-family/en/articles/17708800-prohibited-trading-conduct", accessed: "2026-10-23" }
  - { field: copy_trading, url: "https://intercom.help/funded-futures-family/en/articles/17708800-prohibited-trading-conduct", accessed: "2026-10-23" }
  - { field: vps_allowed, url: "https://intercom.help/funded-futures-family/en/articles/15892413-bots-algorithmic-trading-policy", accessed: "2026-10-23" }
  - { field: vps_allowed, url: "https://intercom.help/funded-futures-family/en/articles/17708800-prohibited-trading-conduct", accessed: "2026-10-23" }
  - { field: personal_device_required, url: "https://intercom.help/funded-futures-family/en/articles/17155892-device-sharing", accessed: "2026-10-23" }
  - { field: trading_day_reset, url: "https://intercom.help/funded-futures-family/en/articles/15892350-permitted-times-to-trade", accessed: "2026-10-23" }
  - { field: drawdown, url: "https://intercom.help/funded-futures-family/en/articles/15879855-straight-to-funded-s2f-funded-account", accessed: "2026-10-23" }
  - { field: drawdown, url: "https://intercom.help/funded-futures-family/en/articles/15880602-straight-to-funded-s2f-drawdown", accessed: "2026-10-23" }
  - { field: daily_loss_limit, url: "https://intercom.help/funded-futures-family/en/articles/15879855-straight-to-funded-s2f-funded-account", accessed: "2026-10-23" }
  - { field: consistency_rule, url: "https://intercom.help/funded-futures-family/en/articles/15879855-straight-to-funded-s2f-funded-account", accessed: "2026-10-23" }
  - { field: consistency_rule, url: "https://intercom.help/funded-futures-family/en/articles/15879981-straight-to-funded-s2f-payout-requirements", accessed: "2026-10-23" }
  - { field: consistency_rule, url: "https://intercom.help/funded-futures-family/en/articles/15880555-straight-to-funded-s2f-consistency-percentage", accessed: "2026-10-23" }
  - { field: consistency_rule, url: "https://www.fundedfuturesfamily.com/", accessed: "2026-10-23" }
  - { field: max_position, url: "https://intercom.help/funded-futures-family/en/articles/15879855-straight-to-funded-s2f-funded-account", accessed: "2026-10-23" }
  - { field: max_position, url: "https://intercom.help/funded-futures-family/en/articles/16310780-straight-to-funded-s2f-scaling-plan", accessed: "2026-10-23" }
  - { field: min_trading_days, url: "https://intercom.help/funded-futures-family/en/articles/15879855-straight-to-funded-s2f-funded-account", accessed: "2026-10-23" }
  - { field: min_trading_days, url: "https://intercom.help/funded-futures-family/en/articles/15879981-straight-to-funded-s2f-payout-requirements", accessed: "2026-10-23" }
```

#### S2F Accelerate — funded — 50K

```yaml
firm: funded-futures-family
plan: "S2F Accelerate"
phase: funded
size_label: "50K"
size_usd: "50000.00"
instruments: [MNQ, NQ]
platforms: ["Tradovate", "WealthCharts", "NinjaTrader (sign in with FFF credentials, Simulation)", "TradingView (via Tradovate add-on)"]
drawdown: { amount_usd: "2000.00", type: "trailing_intraday", stops_trailing_at: "Not stated for 50K (the 25K page says \"Never locks\"; the 50K drawdown page says only that the threshold \"never moves back down\")" }
daily_loss_limit: { amount_usd: null, kind: "none" }
profit_target_usd: "none"
consistency_rule: { text: "Accounts purchased before 2026-10-22: 25% for the lifetime of the account. On or after 2026-10-22: 20% (payout 1) / 25% (payout 2) / 30% (payout 3 and beyond) of total profit", threshold_pct: null }
max_position: { minis: "5", micros: "50", scaling: "No scaling plan" }
min_trading_days: "5"
trading_day_reset: { time: null, tz_as_published: null }
commission_per_contract: { mnq_usd: null, nq_usd: null, basis: null, includes: null }
copy_trading_allowed: conditional
automated_trading_allowed: no
automation_rule_text: "At Funded Futures Family, we don’t permit bot or algorithmic trading. / Bots, algorithms, expert advisors and any automated order entry are not allowed."
vps_allowed: conflict
personal_device_required: yes
status: conflict
not_confirmed_fields: ["commission_per_contract", "trading_day_reset", "vps_allowed (conflict)", "copy_trading_allowed (ambiguity vs automation ban)", "drawdown.stops_trailing_at"]
notes: "No evaluation. Invite-only per home page card. 5 winning days of $200+ before a payout. Payout profit target per cycle for purchases on or after 2026-10-22: $4,000 (50K), resets after each payout (before that date: none). Payout buffer $52,100, max payouts $1,250 / $1,250 / $1,500, minimum payout $500, 90/10 split. Dates in the rules are 2026-10-22; today is 2026-10-23 so the new terms apply to current purchases."
sources:
  - { field: platforms, url: "https://intercom.help/funded-futures-family/en/articles/15656536-connecting-your-trading-platform", accessed: "2026-10-23" }
  - { field: platforms, url: "https://www.fundedfuturesfamily.com/", accessed: "2026-10-23" }
  - { field: instruments, url: "https://intercom.help/funded-futures-family/en/articles/17708800-prohibited-trading-conduct", accessed: "2026-10-23" }
  - { field: automation, url: "https://intercom.help/funded-futures-family/en/articles/15892413-bots-algorithmic-trading-policy", accessed: "2026-10-23" }
  - { field: automation, url: "https://intercom.help/funded-futures-family/en/articles/17708800-prohibited-trading-conduct", accessed: "2026-10-23" }
  - { field: copy_trading, url: "https://intercom.help/funded-futures-family/en/articles/17708800-prohibited-trading-conduct", accessed: "2026-10-23" }
  - { field: vps_allowed, url: "https://intercom.help/funded-futures-family/en/articles/15892413-bots-algorithmic-trading-policy", accessed: "2026-10-23" }
  - { field: vps_allowed, url: "https://intercom.help/funded-futures-family/en/articles/17708800-prohibited-trading-conduct", accessed: "2026-10-23" }
  - { field: personal_device_required, url: "https://intercom.help/funded-futures-family/en/articles/17155892-device-sharing", accessed: "2026-10-23" }
  - { field: trading_day_reset, url: "https://intercom.help/funded-futures-family/en/articles/15892350-permitted-times-to-trade", accessed: "2026-10-23" }
  - { field: drawdown, url: "https://intercom.help/funded-futures-family/en/articles/16949878-accelerate-plan-funded-account", accessed: "2026-10-23" }
  - { field: drawdown, url: "https://intercom.help/funded-futures-family/en/articles/16964753-s2f-accelerate-plan-drawdown", accessed: "2026-10-23" }
  - { field: daily_loss_limit, url: "https://intercom.help/funded-futures-family/en/articles/16949878-accelerate-plan-funded-account", accessed: "2026-10-23" }
  - { field: consistency_rule, url: "https://intercom.help/funded-futures-family/en/articles/16949878-accelerate-plan-funded-account", accessed: "2026-10-23" }
  - { field: consistency_rule, url: "https://intercom.help/funded-futures-family/en/articles/16950249-accelerate-plan-payout-requirements", accessed: "2026-10-23" }
  - { field: consistency_rule, url: "https://intercom.help/funded-futures-family/en/articles/16964735-s2f-accelerate-plan-consistency-percentage", accessed: "2026-10-23" }
  - { field: max_position, url: "https://intercom.help/funded-futures-family/en/articles/16949878-accelerate-plan-funded-account", accessed: "2026-10-23" }
  - { field: min_trading_days, url: "https://intercom.help/funded-futures-family/en/articles/16950249-accelerate-plan-payout-requirements", accessed: "2026-10-23" }
```

#### S2F Accelerate — funded — 25K

```yaml
firm: funded-futures-family
plan: "S2F Accelerate"
phase: funded
size_label: "25K"
size_usd: "25000.00"
instruments: [MNQ, NQ]
platforms: ["Tradovate", "WealthCharts", "NinjaTrader (sign in with FFF credentials, Simulation)", "TradingView (via Tradovate add-on)"]
drawdown: { amount_usd: "1000.00", type: "trailing_intraday", stops_trailing_at: "Never locks (\"Drawdown Lock: Never locks\")" }
daily_loss_limit: { amount_usd: null, kind: "none" }
profit_target_usd: "none"
consistency_rule: { text: "Accounts purchased before 2026-10-22: 25% lifetime. On or after 2026-10-22: 20% / 25% / 30% for payouts 1 / 2 / 3 and beyond", threshold_pct: null }
max_position: { minis: "3", micros: "30", scaling: "No scaling plan" }
min_trading_days: "5"
trading_day_reset: { time: null, tz_as_published: null }
commission_per_contract: { mnq_usd: null, nq_usd: null, basis: null, includes: null }
copy_trading_allowed: conditional
automated_trading_allowed: no
automation_rule_text: "At Funded Futures Family, we don’t permit bot or algorithmic trading. / Bots, algorithms, expert advisors and any automated order entry are not allowed."
vps_allowed: conflict
personal_device_required: yes
status: conflict
not_confirmed_fields: ["commission_per_contract", "trading_day_reset", "vps_allowed (conflict)", "copy_trading_allowed (ambiguity vs automation ban)"]
notes: "Invite-only per home page card. 5 qualifying trading days of $200+. Payout profit target per cycle for purchases on or after 2026-10-22: $2,000. Payout buffer $1,100, maximum payout $500."
sources:
  - { field: platforms, url: "https://intercom.help/funded-futures-family/en/articles/15656536-connecting-your-trading-platform", accessed: "2026-10-23" }
  - { field: platforms, url: "https://www.fundedfuturesfamily.com/", accessed: "2026-10-23" }
  - { field: instruments, url: "https://intercom.help/funded-futures-family/en/articles/17708800-prohibited-trading-conduct", accessed: "2026-10-23" }
  - { field: automation, url: "https://intercom.help/funded-futures-family/en/articles/15892413-bots-algorithmic-trading-policy", accessed: "2026-10-23" }
  - { field: automation, url: "https://intercom.help/funded-futures-family/en/articles/17708800-prohibited-trading-conduct", accessed: "2026-10-23" }
  - { field: copy_trading, url: "https://intercom.help/funded-futures-family/en/articles/17708800-prohibited-trading-conduct", accessed: "2026-10-23" }
  - { field: vps_allowed, url: "https://intercom.help/funded-futures-family/en/articles/15892413-bots-algorithmic-trading-policy", accessed: "2026-10-23" }
  - { field: vps_allowed, url: "https://intercom.help/funded-futures-family/en/articles/17708800-prohibited-trading-conduct", accessed: "2026-10-23" }
  - { field: personal_device_required, url: "https://intercom.help/funded-futures-family/en/articles/17155892-device-sharing", accessed: "2026-10-23" }
  - { field: trading_day_reset, url: "https://intercom.help/funded-futures-family/en/articles/15892350-permitted-times-to-trade", accessed: "2026-10-23" }
  - { field: drawdown, url: "https://intercom.help/funded-futures-family/en/articles/17333029-25k-accelerate-plan", accessed: "2026-10-23" }
  - { field: daily_loss_limit, url: "https://intercom.help/funded-futures-family/en/articles/17333029-25k-accelerate-plan", accessed: "2026-10-23" }
  - { field: consistency_rule, url: "https://intercom.help/funded-futures-family/en/articles/17333029-25k-accelerate-plan", accessed: "2026-10-23" }
  - { field: consistency_rule, url: "https://intercom.help/funded-futures-family/en/articles/16964735-s2f-accelerate-plan-consistency-percentage", accessed: "2026-10-23" }
  - { field: max_position, url: "https://intercom.help/funded-futures-family/en/articles/17333029-25k-accelerate-plan", accessed: "2026-10-23" }
  - { field: min_trading_days, url: "https://intercom.help/funded-futures-family/en/articles/17333029-25k-accelerate-plan", accessed: "2026-10-23" }
```

#### S2F Payday — funded — 50K

```yaml
firm: funded-futures-family
plan: "S2F Payday"
phase: funded
size_label: "50K"
size_usd: "50000.00"
instruments: [MNQ, NQ]
platforms: ["Tradovate", "WealthCharts", "NinjaTrader (sign in with FFF credentials, Simulation)", "TradingView (via Tradovate add-on)"]
drawdown: { amount_usd: "2000.00", type: "trailing_eod", stops_trailing_at: "Locks at $50,000 after reaching $52,000 EOD" }
daily_loss_limit: { amount_usd: "1000.00", kind: "hard" }
profit_target_usd: "none"
consistency_rule: { text: "Consistency rule 40%", threshold_pct: 40 }
max_position: { minis: "5", micros: "50", scaling: "Scaling plan: yes (table not published on the page)" }
min_trading_days: "7"
trading_day_reset: { time: null, tz_as_published: null }
commission_per_contract: { mnq_usd: null, nq_usd: null, basis: null, includes: null }
copy_trading_allowed: conditional
automated_trading_allowed: no
automation_rule_text: "At Funded Futures Family, we don’t permit bot or algorithmic trading. / Bots, algorithms, expert advisors and any automated order entry are not allowed."
vps_allowed: conflict
personal_device_required: yes
status: conflict
not_confirmed_fields: ["commission_per_contract", "trading_day_reset", "vps_allowed (conflict)", "copy_trading_allowed (ambiguity vs automation ban)", "max_position.scaling (Payday scaling table not published)"]
notes: "7 qualifying trading days of $200+. Profit required between payouts $3,000. Payouts up to $1,000 / $1,000 / $1,250 (#3 and beyond). DLL type published as \"Hard Breach\"."
sources:
  - { field: platforms, url: "https://intercom.help/funded-futures-family/en/articles/15656536-connecting-your-trading-platform", accessed: "2026-10-23" }
  - { field: platforms, url: "https://www.fundedfuturesfamily.com/", accessed: "2026-10-23" }
  - { field: instruments, url: "https://intercom.help/funded-futures-family/en/articles/17708800-prohibited-trading-conduct", accessed: "2026-10-23" }
  - { field: automation, url: "https://intercom.help/funded-futures-family/en/articles/15892413-bots-algorithmic-trading-policy", accessed: "2026-10-23" }
  - { field: automation, url: "https://intercom.help/funded-futures-family/en/articles/17708800-prohibited-trading-conduct", accessed: "2026-10-23" }
  - { field: copy_trading, url: "https://intercom.help/funded-futures-family/en/articles/17708800-prohibited-trading-conduct", accessed: "2026-10-23" }
  - { field: vps_allowed, url: "https://intercom.help/funded-futures-family/en/articles/15892413-bots-algorithmic-trading-policy", accessed: "2026-10-23" }
  - { field: vps_allowed, url: "https://intercom.help/funded-futures-family/en/articles/17708800-prohibited-trading-conduct", accessed: "2026-10-23" }
  - { field: personal_device_required, url: "https://intercom.help/funded-futures-family/en/articles/17155892-device-sharing", accessed: "2026-10-23" }
  - { field: trading_day_reset, url: "https://intercom.help/funded-futures-family/en/articles/15892350-permitted-times-to-trade", accessed: "2026-10-23" }
  - { field: drawdown, url: "https://intercom.help/funded-futures-family/en/articles/17332937-payday-50k", accessed: "2026-10-23" }
  - { field: daily_loss_limit, url: "https://intercom.help/funded-futures-family/en/articles/17332937-payday-50k", accessed: "2026-10-23" }
  - { field: consistency_rule, url: "https://intercom.help/funded-futures-family/en/articles/17332937-payday-50k", accessed: "2026-10-23" }
  - { field: max_position, url: "https://intercom.help/funded-futures-family/en/articles/17332937-payday-50k", accessed: "2026-10-23" }
  - { field: min_trading_days, url: "https://intercom.help/funded-futures-family/en/articles/17332937-payday-50k", accessed: "2026-10-23" }
```

#### S2F Sniper — funded — 25K

```yaml
firm: funded-futures-family
plan: "S2F Sniper (\"Sniper Gang\")"
phase: funded
size_label: "25K"
size_usd: "25000.00"
instruments: [MNQ, NQ]
platforms: ["Tradovate", "WealthCharts", "NinjaTrader (sign in with FFF credentials, Simulation)", "TradingView (via Tradovate add-on)"]
drawdown: { amount_usd: "750.00", type: "trailing_eod", stops_trailing_at: "Never locks" }
daily_loss_limit: { amount_usd: null, kind: "none" }
profit_target_usd: "none"
consistency_rule: { text: "None", threshold_pct: null }
max_position: { minis: "1", micros: "10", scaling: "No scaling plan" }
min_trading_days: null
trading_day_reset: { time: null, tz_as_published: null }
commission_per_contract: { mnq_usd: null, nq_usd: null, basis: null, includes: null }
copy_trading_allowed: conditional
automated_trading_allowed: no
automation_rule_text: "At Funded Futures Family, we don’t permit bot or algorithmic trading. / Bots, algorithms, expert advisors and any automated order entry are not allowed."
vps_allowed: conflict
personal_device_required: yes
status: conflict
not_confirmed_fields: ["commission_per_contract", "trading_day_reset", "vps_allowed (conflict)", "copy_trading_allowed (ambiguity vs automation ban)", "min_trading_days"]
notes: "Invite-only per home page card. Daily payouts after a $2,500 profit target between payout requests; payouts #1-2 up to $500, #3+ up to $1,000. News restriction: high-impact USD news, must be flat from 1 minute before to 1 minute after, breach otherwise (hard breach). No traditional minimum trading days."
sources:
  - { field: platforms, url: "https://intercom.help/funded-futures-family/en/articles/15656536-connecting-your-trading-platform", accessed: "2026-10-23" }
  - { field: platforms, url: "https://www.fundedfuturesfamily.com/", accessed: "2026-10-23" }
  - { field: instruments, url: "https://intercom.help/funded-futures-family/en/articles/17708800-prohibited-trading-conduct", accessed: "2026-10-23" }
  - { field: automation, url: "https://intercom.help/funded-futures-family/en/articles/15892413-bots-algorithmic-trading-policy", accessed: "2026-10-23" }
  - { field: automation, url: "https://intercom.help/funded-futures-family/en/articles/17708800-prohibited-trading-conduct", accessed: "2026-10-23" }
  - { field: copy_trading, url: "https://intercom.help/funded-futures-family/en/articles/17708800-prohibited-trading-conduct", accessed: "2026-10-23" }
  - { field: vps_allowed, url: "https://intercom.help/funded-futures-family/en/articles/15892413-bots-algorithmic-trading-policy", accessed: "2026-10-23" }
  - { field: vps_allowed, url: "https://intercom.help/funded-futures-family/en/articles/17708800-prohibited-trading-conduct", accessed: "2026-10-23" }
  - { field: personal_device_required, url: "https://intercom.help/funded-futures-family/en/articles/17155892-device-sharing", accessed: "2026-10-23" }
  - { field: trading_day_reset, url: "https://intercom.help/funded-futures-family/en/articles/15892350-permitted-times-to-trade", accessed: "2026-10-23" }
  - { field: drawdown, url: "https://intercom.help/funded-futures-family/en/articles/17333010-25k-sniper-plan", accessed: "2026-10-23" }
  - { field: daily_loss_limit, url: "https://intercom.help/funded-futures-family/en/articles/17333010-25k-sniper-plan", accessed: "2026-10-23" }
  - { field: consistency_rule, url: "https://intercom.help/funded-futures-family/en/articles/17333010-25k-sniper-plan", accessed: "2026-10-23" }
  - { field: max_position, url: "https://intercom.help/funded-futures-family/en/articles/17333010-25k-sniper-plan", accessed: "2026-10-23" }
```

#### Base — evaluation — 2K

```yaml
firm: funded-futures-family
plan: "Base ($2,000 Base Plan)"
phase: evaluation
size_label: "2K"
size_usd: "2000.00"
instruments: [MNQ, NQ]
platforms: ["Tradovate", "WealthCharts", "NinjaTrader (sign in with FFF credentials, Simulation)", "TradingView (via Tradovate add-on)"]
drawdown: { amount_usd: "2000.00", type: "trailing_eod", stops_trailing_at: "Not stated (page: \"only moves upward\")" }
daily_loss_limit: { amount_usd: null, kind: "none" }
profit_target_usd: "3000.00"
consistency_rule: { text: "40% consistency rule", threshold_pct: 40 }
max_position: { minis: "5", micros: "50", scaling: "Position limits adjust with the current account balance after each closed trade" }
min_trading_days: null
trading_day_reset: { time: null, tz_as_published: null }
commission_per_contract: { mnq_usd: null, nq_usd: null, basis: null, includes: null }
copy_trading_allowed: conditional
automated_trading_allowed: no
automation_rule_text: "At Funded Futures Family, we don’t permit bot or algorithmic trading. / Bots, algorithms, expert advisors and any automated order entry are not allowed."
vps_allowed: conflict
personal_device_required: yes
status: conflict
not_confirmed_fields: ["commission_per_contract", "trading_day_reset", "vps_allowed (conflict)", "copy_trading_allowed (ambiguity vs automation ban)", "drawdown.stops_trailing_at", "min_trading_days"]
notes: "Account size is published as $2,000 with a $3,000 profit target and $2,000 loss limit (so the loss limit equals the account size). Equity must stay above the threshold throughout the day including open trades."
sources:
  - { field: platforms, url: "https://intercom.help/funded-futures-family/en/articles/15656536-connecting-your-trading-platform", accessed: "2026-10-23" }
  - { field: platforms, url: "https://www.fundedfuturesfamily.com/", accessed: "2026-10-23" }
  - { field: instruments, url: "https://intercom.help/funded-futures-family/en/articles/17708800-prohibited-trading-conduct", accessed: "2026-10-23" }
  - { field: automation, url: "https://intercom.help/funded-futures-family/en/articles/15892413-bots-algorithmic-trading-policy", accessed: "2026-10-23" }
  - { field: automation, url: "https://intercom.help/funded-futures-family/en/articles/17708800-prohibited-trading-conduct", accessed: "2026-10-23" }
  - { field: copy_trading, url: "https://intercom.help/funded-futures-family/en/articles/17708800-prohibited-trading-conduct", accessed: "2026-10-23" }
  - { field: vps_allowed, url: "https://intercom.help/funded-futures-family/en/articles/15892413-bots-algorithmic-trading-policy", accessed: "2026-10-23" }
  - { field: vps_allowed, url: "https://intercom.help/funded-futures-family/en/articles/17708800-prohibited-trading-conduct", accessed: "2026-10-23" }
  - { field: personal_device_required, url: "https://intercom.help/funded-futures-family/en/articles/17155892-device-sharing", accessed: "2026-10-23" }
  - { field: trading_day_reset, url: "https://intercom.help/funded-futures-family/en/articles/15892350-permitted-times-to-trade", accessed: "2026-10-23" }
  - { field: drawdown, url: "https://intercom.help/funded-futures-family/en/articles/17075322-base-plan-evaluation", accessed: "2026-10-23" }
  - { field: drawdown, url: "https://intercom.help/funded-futures-family/en/articles/17082450-base-plan-drawdown", accessed: "2026-10-23" }
  - { field: daily_loss_limit, url: "https://intercom.help/funded-futures-family/en/articles/17075322-base-plan-evaluation", accessed: "2026-10-23" }
  - { field: profit_target_usd, url: "https://intercom.help/funded-futures-family/en/articles/17075322-base-plan-evaluation", accessed: "2026-10-23" }
  - { field: consistency_rule, url: "https://intercom.help/funded-futures-family/en/articles/17075322-base-plan-evaluation", accessed: "2026-10-23" }
  - { field: max_position, url: "https://intercom.help/funded-futures-family/en/articles/17075322-base-plan-evaluation", accessed: "2026-10-23" }
  - { field: max_position, url: "https://intercom.help/funded-futures-family/en/articles/17083162-base-plan-scaling-plan", accessed: "2026-10-23" }
```

#### Base — funded — 2K

```yaml
firm: funded-futures-family
plan: "Base ($2,000 Base Plan)"
phase: funded
size_label: "2K"
size_usd: "2000.00"
instruments: [MNQ, NQ]
platforms: ["Tradovate", "WealthCharts", "NinjaTrader (sign in with FFF credentials, Simulation)", "TradingView (via Tradovate add-on)"]
drawdown: { amount_usd: "2000.00", type: "trailing_intraday", stops_trailing_at: "Stops trailing once profit reaches $2,000 and remains fixed" }
daily_loss_limit: { amount_usd: null, kind: "none" }
profit_target_usd: "none"
consistency_rule: { text: "No consistency rule", threshold_pct: null }
max_position: { minis: "15", micros: "150", scaling: "Scales with closed balance: $1,000 allows 1 mini/10 micros, $2,000 allows 2 minis/20 micros, $4,000 allows 4 minis/40 micros, up to 15 minis/150 micros" }
min_trading_days: "0"
trading_day_reset: { time: null, tz_as_published: null }
commission_per_contract: { mnq_usd: null, nq_usd: null, basis: null, includes: null }
copy_trading_allowed: conditional
automated_trading_allowed: no
automation_rule_text: "At Funded Futures Family, we don’t permit bot or algorithmic trading. / Bots, algorithms, expert advisors and any automated order entry are not allowed."
vps_allowed: conflict
personal_device_required: yes
status: conflict
not_confirmed_fields: ["commission_per_contract", "trading_day_reset", "vps_allowed (conflict)", "copy_trading_allowed (ambiguity vs automation ban)"]
notes: "Payouts every trading day once eligible, $500 to $1,000 per payout, balance must stay at or above the $4,000 buffer, at least $1 new net profit between payouts, 90% split. \"No qualifying trading days required.\""
sources:
  - { field: platforms, url: "https://intercom.help/funded-futures-family/en/articles/15656536-connecting-your-trading-platform", accessed: "2026-10-23" }
  - { field: platforms, url: "https://www.fundedfuturesfamily.com/", accessed: "2026-10-23" }
  - { field: instruments, url: "https://intercom.help/funded-futures-family/en/articles/17708800-prohibited-trading-conduct", accessed: "2026-10-23" }
  - { field: automation, url: "https://intercom.help/funded-futures-family/en/articles/15892413-bots-algorithmic-trading-policy", accessed: "2026-10-23" }
  - { field: automation, url: "https://intercom.help/funded-futures-family/en/articles/17708800-prohibited-trading-conduct", accessed: "2026-10-23" }
  - { field: copy_trading, url: "https://intercom.help/funded-futures-family/en/articles/17708800-prohibited-trading-conduct", accessed: "2026-10-23" }
  - { field: vps_allowed, url: "https://intercom.help/funded-futures-family/en/articles/15892413-bots-algorithmic-trading-policy", accessed: "2026-10-23" }
  - { field: vps_allowed, url: "https://intercom.help/funded-futures-family/en/articles/17708800-prohibited-trading-conduct", accessed: "2026-10-23" }
  - { field: personal_device_required, url: "https://intercom.help/funded-futures-family/en/articles/17155892-device-sharing", accessed: "2026-10-23" }
  - { field: trading_day_reset, url: "https://intercom.help/funded-futures-family/en/articles/15892350-permitted-times-to-trade", accessed: "2026-10-23" }
  - { field: drawdown, url: "https://intercom.help/funded-futures-family/en/articles/17076299-base-plan-funded-account", accessed: "2026-10-23" }
  - { field: drawdown, url: "https://intercom.help/funded-futures-family/en/articles/17082450-base-plan-drawdown", accessed: "2026-10-23" }
  - { field: daily_loss_limit, url: "https://intercom.help/funded-futures-family/en/articles/17076299-base-plan-funded-account", accessed: "2026-10-23" }
  - { field: consistency_rule, url: "https://intercom.help/funded-futures-family/en/articles/17076299-base-plan-funded-account", accessed: "2026-10-23" }
  - { field: max_position, url: "https://intercom.help/funded-futures-family/en/articles/17076299-base-plan-funded-account", accessed: "2026-10-23" }
  - { field: max_position, url: "https://intercom.help/funded-futures-family/en/articles/17083162-base-plan-scaling-plan", accessed: "2026-10-23" }
  - { field: min_trading_days, url: "https://intercom.help/funded-futures-family/en/articles/17076299-base-plan-funded-account", accessed: "2026-10-23" }
  - { field: min_trading_days, url: "https://intercom.help/funded-futures-family/en/articles/17079566-base-plan-payout-requirements", accessed: "2026-10-23" }
```

#### Prestige — evaluation — 25K

```yaml
firm: funded-futures-family
plan: "Prestige (invite-only)"
phase: evaluation
size_label: "25K"
size_usd: "25000.00"
instruments: [MNQ, NQ]
platforms: ["Tradovate", "WealthCharts", "NinjaTrader (sign in with FFF credentials, Simulation)", "TradingView (via Tradovate add-on)"]
drawdown: { amount_usd: "1000.00", type: null, stops_trailing_at: null }
daily_loss_limit: { amount_usd: null, kind: null }
profit_target_usd: "2000.00"
consistency_rule: { text: "40% consistency rule", threshold_pct: 40 }
max_position: { minis: "3", micros: "30", scaling: null }
min_trading_days: "5"
trading_day_reset: { time: null, tz_as_published: null }
commission_per_contract: { mnq_usd: null, nq_usd: null, basis: null, includes: null }
copy_trading_allowed: conditional
automated_trading_allowed: no
automation_rule_text: "At Funded Futures Family, we don’t permit bot or algorithmic trading. / Bots, algorithms, expert advisors and any automated order entry are not allowed."
vps_allowed: conflict
personal_device_required: yes
status: conflict
not_confirmed_fields: ["commission_per_contract", "trading_day_reset", "vps_allowed (conflict)", "copy_trading_allowed (ambiguity vs automation ban)", "drawdown.type", "drawdown.stops_trailing_at", "daily_loss_limit"]
notes: "Strictly invite-only; traders cannot apply. Passing leads to the Professional Stage (see Pro blocks), no simulated funded payouts. Drawdown type and daily loss limit of the Prestige evaluation are not published."
sources:
  - { field: platforms, url: "https://intercom.help/funded-futures-family/en/articles/15656536-connecting-your-trading-platform", accessed: "2026-10-23" }
  - { field: platforms, url: "https://www.fundedfuturesfamily.com/", accessed: "2026-10-23" }
  - { field: instruments, url: "https://intercom.help/funded-futures-family/en/articles/17708800-prohibited-trading-conduct", accessed: "2026-10-23" }
  - { field: automation, url: "https://intercom.help/funded-futures-family/en/articles/15892413-bots-algorithmic-trading-policy", accessed: "2026-10-23" }
  - { field: automation, url: "https://intercom.help/funded-futures-family/en/articles/17708800-prohibited-trading-conduct", accessed: "2026-10-23" }
  - { field: copy_trading, url: "https://intercom.help/funded-futures-family/en/articles/17708800-prohibited-trading-conduct", accessed: "2026-10-23" }
  - { field: vps_allowed, url: "https://intercom.help/funded-futures-family/en/articles/15892413-bots-algorithmic-trading-policy", accessed: "2026-10-23" }
  - { field: vps_allowed, url: "https://intercom.help/funded-futures-family/en/articles/17708800-prohibited-trading-conduct", accessed: "2026-10-23" }
  - { field: personal_device_required, url: "https://intercom.help/funded-futures-family/en/articles/17155892-device-sharing", accessed: "2026-10-23" }
  - { field: trading_day_reset, url: "https://intercom.help/funded-futures-family/en/articles/15892350-permitted-times-to-trade", accessed: "2026-10-23" }
  - { field: drawdown, url: "https://intercom.help/funded-futures-family/en/articles/16911595-prestige-evaluation", accessed: "2026-10-23" }
  - { field: profit_target_usd, url: "https://intercom.help/funded-futures-family/en/articles/16911595-prestige-evaluation", accessed: "2026-10-23" }
  - { field: consistency_rule, url: "https://intercom.help/funded-futures-family/en/articles/16911595-prestige-evaluation", accessed: "2026-10-23" }
  - { field: max_position, url: "https://intercom.help/funded-futures-family/en/articles/16911595-prestige-evaluation", accessed: "2026-10-23" }
  - { field: min_trading_days, url: "https://intercom.help/funded-futures-family/en/articles/16911595-prestige-evaluation", accessed: "2026-10-23" }
```

#### Prestige — evaluation — 50K

```yaml
firm: funded-futures-family
plan: "Prestige (invite-only)"
phase: evaluation
size_label: "50K"
size_usd: "50000.00"
instruments: [MNQ, NQ]
platforms: ["Tradovate", "WealthCharts", "NinjaTrader (sign in with FFF credentials, Simulation)", "TradingView (via Tradovate add-on)"]
drawdown: { amount_usd: "2000.00", type: null, stops_trailing_at: null }
daily_loss_limit: { amount_usd: null, kind: null }
profit_target_usd: "4000.00"
consistency_rule: { text: "40% consistency rule", threshold_pct: 40 }
max_position: { minis: "5", micros: "50", scaling: null }
min_trading_days: "5"
trading_day_reset: { time: null, tz_as_published: null }
commission_per_contract: { mnq_usd: null, nq_usd: null, basis: null, includes: null }
copy_trading_allowed: conditional
automated_trading_allowed: no
automation_rule_text: "At Funded Futures Family, we don’t permit bot or algorithmic trading. / Bots, algorithms, expert advisors and any automated order entry are not allowed."
vps_allowed: conflict
personal_device_required: yes
status: conflict
not_confirmed_fields: ["commission_per_contract", "trading_day_reset", "vps_allowed (conflict)", "copy_trading_allowed (ambiguity vs automation ban)", "drawdown.type", "drawdown.stops_trailing_at", "daily_loss_limit"]
notes: "Strictly invite-only; traders cannot apply. Passing leads to the Professional Stage (see Pro blocks), no simulated funded payouts. Drawdown type and daily loss limit of the Prestige evaluation are not published."
sources:
  - { field: platforms, url: "https://intercom.help/funded-futures-family/en/articles/15656536-connecting-your-trading-platform", accessed: "2026-10-23" }
  - { field: platforms, url: "https://www.fundedfuturesfamily.com/", accessed: "2026-10-23" }
  - { field: instruments, url: "https://intercom.help/funded-futures-family/en/articles/17708800-prohibited-trading-conduct", accessed: "2026-10-23" }
  - { field: automation, url: "https://intercom.help/funded-futures-family/en/articles/15892413-bots-algorithmic-trading-policy", accessed: "2026-10-23" }
  - { field: automation, url: "https://intercom.help/funded-futures-family/en/articles/17708800-prohibited-trading-conduct", accessed: "2026-10-23" }
  - { field: copy_trading, url: "https://intercom.help/funded-futures-family/en/articles/17708800-prohibited-trading-conduct", accessed: "2026-10-23" }
  - { field: vps_allowed, url: "https://intercom.help/funded-futures-family/en/articles/15892413-bots-algorithmic-trading-policy", accessed: "2026-10-23" }
  - { field: vps_allowed, url: "https://intercom.help/funded-futures-family/en/articles/17708800-prohibited-trading-conduct", accessed: "2026-10-23" }
  - { field: personal_device_required, url: "https://intercom.help/funded-futures-family/en/articles/17155892-device-sharing", accessed: "2026-10-23" }
  - { field: trading_day_reset, url: "https://intercom.help/funded-futures-family/en/articles/15892350-permitted-times-to-trade", accessed: "2026-10-23" }
  - { field: drawdown, url: "https://intercom.help/funded-futures-family/en/articles/16911595-prestige-evaluation", accessed: "2026-10-23" }
  - { field: profit_target_usd, url: "https://intercom.help/funded-futures-family/en/articles/16911595-prestige-evaluation", accessed: "2026-10-23" }
  - { field: consistency_rule, url: "https://intercom.help/funded-futures-family/en/articles/16911595-prestige-evaluation", accessed: "2026-10-23" }
  - { field: max_position, url: "https://intercom.help/funded-futures-family/en/articles/16911595-prestige-evaluation", accessed: "2026-10-23" }
  - { field: min_trading_days, url: "https://intercom.help/funded-futures-family/en/articles/16911595-prestige-evaluation", accessed: "2026-10-23" }
```

#### Prestige — evaluation — 100K

```yaml
firm: funded-futures-family
plan: "Prestige (invite-only)"
phase: evaluation
size_label: "100K"
size_usd: "100000.00"
instruments: [MNQ, NQ]
platforms: ["Tradovate", "WealthCharts", "NinjaTrader (sign in with FFF credentials, Simulation)", "TradingView (via Tradovate add-on)"]
drawdown: { amount_usd: "3000.00", type: null, stops_trailing_at: null }
daily_loss_limit: { amount_usd: null, kind: null }
profit_target_usd: "6000.00"
consistency_rule: { text: "40% consistency rule", threshold_pct: 40 }
max_position: { minis: "10", micros: "100", scaling: null }
min_trading_days: "5"
trading_day_reset: { time: null, tz_as_published: null }
commission_per_contract: { mnq_usd: null, nq_usd: null, basis: null, includes: null }
copy_trading_allowed: conditional
automated_trading_allowed: no
automation_rule_text: "At Funded Futures Family, we don’t permit bot or algorithmic trading. / Bots, algorithms, expert advisors and any automated order entry are not allowed."
vps_allowed: conflict
personal_device_required: yes
status: conflict
not_confirmed_fields: ["commission_per_contract", "trading_day_reset", "vps_allowed (conflict)", "copy_trading_allowed (ambiguity vs automation ban)", "drawdown.type", "drawdown.stops_trailing_at", "daily_loss_limit"]
notes: "Strictly invite-only; traders cannot apply. Passing leads to the Professional Stage (see Pro blocks), no simulated funded payouts. Drawdown type and daily loss limit of the Prestige evaluation are not published."
sources:
  - { field: platforms, url: "https://intercom.help/funded-futures-family/en/articles/15656536-connecting-your-trading-platform", accessed: "2026-10-23" }
  - { field: platforms, url: "https://www.fundedfuturesfamily.com/", accessed: "2026-10-23" }
  - { field: instruments, url: "https://intercom.help/funded-futures-family/en/articles/17708800-prohibited-trading-conduct", accessed: "2026-10-23" }
  - { field: automation, url: "https://intercom.help/funded-futures-family/en/articles/15892413-bots-algorithmic-trading-policy", accessed: "2026-10-23" }
  - { field: automation, url: "https://intercom.help/funded-futures-family/en/articles/17708800-prohibited-trading-conduct", accessed: "2026-10-23" }
  - { field: copy_trading, url: "https://intercom.help/funded-futures-family/en/articles/17708800-prohibited-trading-conduct", accessed: "2026-10-23" }
  - { field: vps_allowed, url: "https://intercom.help/funded-futures-family/en/articles/15892413-bots-algorithmic-trading-policy", accessed: "2026-10-23" }
  - { field: vps_allowed, url: "https://intercom.help/funded-futures-family/en/articles/17708800-prohibited-trading-conduct", accessed: "2026-10-23" }
  - { field: personal_device_required, url: "https://intercom.help/funded-futures-family/en/articles/17155892-device-sharing", accessed: "2026-10-23" }
  - { field: trading_day_reset, url: "https://intercom.help/funded-futures-family/en/articles/15892350-permitted-times-to-trade", accessed: "2026-10-23" }
  - { field: drawdown, url: "https://intercom.help/funded-futures-family/en/articles/16911595-prestige-evaluation", accessed: "2026-10-23" }
  - { field: profit_target_usd, url: "https://intercom.help/funded-futures-family/en/articles/16911595-prestige-evaluation", accessed: "2026-10-23" }
  - { field: consistency_rule, url: "https://intercom.help/funded-futures-family/en/articles/16911595-prestige-evaluation", accessed: "2026-10-23" }
  - { field: max_position, url: "https://intercom.help/funded-futures-family/en/articles/16911595-prestige-evaluation", accessed: "2026-10-23" }
  - { field: min_trading_days, url: "https://intercom.help/funded-futures-family/en/articles/16911595-prestige-evaluation", accessed: "2026-10-23" }
```

#### Prestige — evaluation — 150K

```yaml
firm: funded-futures-family
plan: "Prestige (invite-only)"
phase: evaluation
size_label: "150K"
size_usd: "150000.00"
instruments: [MNQ, NQ]
platforms: ["Tradovate", "WealthCharts", "NinjaTrader (sign in with FFF credentials, Simulation)", "TradingView (via Tradovate add-on)"]
drawdown: { amount_usd: "4500.00", type: null, stops_trailing_at: null }
daily_loss_limit: { amount_usd: null, kind: null }
profit_target_usd: "9000.00"
consistency_rule: { text: "40% consistency rule", threshold_pct: 40 }
max_position: { minis: "15", micros: "150", scaling: null }
min_trading_days: "5"
trading_day_reset: { time: null, tz_as_published: null }
commission_per_contract: { mnq_usd: null, nq_usd: null, basis: null, includes: null }
copy_trading_allowed: conditional
automated_trading_allowed: no
automation_rule_text: "At Funded Futures Family, we don’t permit bot or algorithmic trading. / Bots, algorithms, expert advisors and any automated order entry are not allowed."
vps_allowed: conflict
personal_device_required: yes
status: conflict
not_confirmed_fields: ["commission_per_contract", "trading_day_reset", "vps_allowed (conflict)", "copy_trading_allowed (ambiguity vs automation ban)", "drawdown.type", "drawdown.stops_trailing_at", "daily_loss_limit"]
notes: "Strictly invite-only; traders cannot apply. Passing leads to the Professional Stage (see Pro blocks), no simulated funded payouts. Drawdown type and daily loss limit of the Prestige evaluation are not published."
sources:
  - { field: platforms, url: "https://intercom.help/funded-futures-family/en/articles/15656536-connecting-your-trading-platform", accessed: "2026-10-23" }
  - { field: platforms, url: "https://www.fundedfuturesfamily.com/", accessed: "2026-10-23" }
  - { field: instruments, url: "https://intercom.help/funded-futures-family/en/articles/17708800-prohibited-trading-conduct", accessed: "2026-10-23" }
  - { field: automation, url: "https://intercom.help/funded-futures-family/en/articles/15892413-bots-algorithmic-trading-policy", accessed: "2026-10-23" }
  - { field: automation, url: "https://intercom.help/funded-futures-family/en/articles/17708800-prohibited-trading-conduct", accessed: "2026-10-23" }
  - { field: copy_trading, url: "https://intercom.help/funded-futures-family/en/articles/17708800-prohibited-trading-conduct", accessed: "2026-10-23" }
  - { field: vps_allowed, url: "https://intercom.help/funded-futures-family/en/articles/15892413-bots-algorithmic-trading-policy", accessed: "2026-10-23" }
  - { field: vps_allowed, url: "https://intercom.help/funded-futures-family/en/articles/17708800-prohibited-trading-conduct", accessed: "2026-10-23" }
  - { field: personal_device_required, url: "https://intercom.help/funded-futures-family/en/articles/17155892-device-sharing", accessed: "2026-10-23" }
  - { field: trading_day_reset, url: "https://intercom.help/funded-futures-family/en/articles/15892350-permitted-times-to-trade", accessed: "2026-10-23" }
  - { field: drawdown, url: "https://intercom.help/funded-futures-family/en/articles/16911595-prestige-evaluation", accessed: "2026-10-23" }
  - { field: profit_target_usd, url: "https://intercom.help/funded-futures-family/en/articles/16911595-prestige-evaluation", accessed: "2026-10-23" }
  - { field: consistency_rule, url: "https://intercom.help/funded-futures-family/en/articles/16911595-prestige-evaluation", accessed: "2026-10-23" }
  - { field: max_position, url: "https://intercom.help/funded-futures-family/en/articles/16911595-prestige-evaluation", accessed: "2026-10-23" }
  - { field: min_trading_days, url: "https://intercom.help/funded-futures-family/en/articles/16911595-prestige-evaluation", accessed: "2026-10-23" }
```

#### Professional Stage (Pro) — funded — 50K

```yaml
firm: funded-futures-family
plan: "Professional Stage (Pro execution environment)"
phase: funded
size_label: "50K"
size_usd: "50000.00"
instruments: [MNQ, NQ]
platforms: ["not published for the Pro execution environment"]
drawdown: { amount_usd: null, type: "other", stops_trailing_at: "Locks at $50100 (End-of-Position drawdown: updates only when a position is fully closed)" }
daily_loss_limit: { amount_usd: null, kind: null }
profit_target_usd: "none"
consistency_rule: { text: "There are no consistency rules in the Professional Stage", threshold_pct: null }
max_position: { minis: "case by case (Live Desk review)", micros: "case by case (Live Desk review)", scaling: "Scaling by EOD realized profit, applied only after Live Desk review and broker-side configuration" }
min_trading_days: null
trading_day_reset: { time: null, tz_as_published: null }
commission_per_contract: { mnq_usd: null, nq_usd: null, basis: null, includes: null }
copy_trading_allowed: conditional
automated_trading_allowed: no
automation_rule_text: "At Funded Futures Family, we don’t permit bot or algorithmic trading. / Bots, algorithms, expert advisors and any automated order entry are not allowed."
vps_allowed: conflict
personal_device_required: yes
status: conflict
not_confirmed_fields: ["commission_per_contract", "trading_day_reset", "vps_allowed (conflict)", "copy_trading_allowed (ambiguity vs automation ban)", "drawdown.amount_usd", "drawdown.type (conflict: EOP vs EOD)", "daily_loss_limit", "max_position", "min_trading_days"]
notes: "Live-market program reached after 3 approved payouts or $10,000 in approved simulated payouts, then risk review. Personalized max drawdown assigned per account (amount not published). Daily Loss Limit governed by the scaling plan (amount not published). 80% trader / 20% FFF. The Prestige page calls the Pro funded phase an \"End of Day Drawdown model\", the structure page \"End-of-Position (EOP)\"."
sources:
  - { field: platforms, url: "https://intercom.help/funded-futures-family/en/articles/15656536-connecting-your-trading-platform", accessed: "2026-10-23" }
  - { field: platforms, url: "https://www.fundedfuturesfamily.com/", accessed: "2026-10-23" }
  - { field: instruments, url: "https://intercom.help/funded-futures-family/en/articles/17708800-prohibited-trading-conduct", accessed: "2026-10-23" }
  - { field: automation, url: "https://intercom.help/funded-futures-family/en/articles/15892413-bots-algorithmic-trading-policy", accessed: "2026-10-23" }
  - { field: automation, url: "https://intercom.help/funded-futures-family/en/articles/17708800-prohibited-trading-conduct", accessed: "2026-10-23" }
  - { field: copy_trading, url: "https://intercom.help/funded-futures-family/en/articles/17708800-prohibited-trading-conduct", accessed: "2026-10-23" }
  - { field: vps_allowed, url: "https://intercom.help/funded-futures-family/en/articles/15892413-bots-algorithmic-trading-policy", accessed: "2026-10-23" }
  - { field: vps_allowed, url: "https://intercom.help/funded-futures-family/en/articles/17708800-prohibited-trading-conduct", accessed: "2026-10-23" }
  - { field: personal_device_required, url: "https://intercom.help/funded-futures-family/en/articles/17155892-device-sharing", accessed: "2026-10-23" }
  - { field: trading_day_reset, url: "https://intercom.help/funded-futures-family/en/articles/15892350-permitted-times-to-trade", accessed: "2026-10-23" }
  - { field: drawdown, url: "https://intercom.help/funded-futures-family/en/articles/15880840-professional-stage-structure", accessed: "2026-10-23" }
  - { field: drawdown, url: "https://intercom.help/funded-futures-family/en/articles/16911595-prestige-evaluation", accessed: "2026-10-23" }
  - { field: daily_loss_limit, url: "https://intercom.help/funded-futures-family/en/articles/15880840-professional-stage-structure", accessed: "2026-10-23" }
  - { field: consistency_rule, url: "https://intercom.help/funded-futures-family/en/articles/15880840-professional-stage-structure", accessed: "2026-10-23" }
  - { field: max_position, url: "https://intercom.help/funded-futures-family/en/articles/15880840-professional-stage-structure", accessed: "2026-10-23" }
  - { field: max_position, url: "https://intercom.help/funded-futures-family/en/articles/15891902-professional-stage-operational-policies", accessed: "2026-10-23" }
```

#### Professional Stage (Pro) — funded — 100K

```yaml
firm: funded-futures-family
plan: "Professional Stage (Pro execution environment)"
phase: funded
size_label: "100K"
size_usd: "100000.00"
instruments: [MNQ, NQ]
platforms: ["not published for the Pro execution environment"]
drawdown: { amount_usd: null, type: "other", stops_trailing_at: "Locks at $100100 (End-of-Position drawdown: updates only when a position is fully closed)" }
daily_loss_limit: { amount_usd: null, kind: null }
profit_target_usd: "none"
consistency_rule: { text: "There are no consistency rules in the Professional Stage", threshold_pct: null }
max_position: { minis: "case by case (Live Desk review)", micros: "case by case (Live Desk review)", scaling: "Scaling by EOD realized profit, applied only after Live Desk review and broker-side configuration" }
min_trading_days: null
trading_day_reset: { time: null, tz_as_published: null }
commission_per_contract: { mnq_usd: null, nq_usd: null, basis: null, includes: null }
copy_trading_allowed: conditional
automated_trading_allowed: no
automation_rule_text: "At Funded Futures Family, we don’t permit bot or algorithmic trading. / Bots, algorithms, expert advisors and any automated order entry are not allowed."
vps_allowed: conflict
personal_device_required: yes
status: conflict
not_confirmed_fields: ["commission_per_contract", "trading_day_reset", "vps_allowed (conflict)", "copy_trading_allowed (ambiguity vs automation ban)", "drawdown.amount_usd", "drawdown.type (conflict: EOP vs EOD)", "daily_loss_limit", "max_position", "min_trading_days"]
notes: "Live-market program reached after 3 approved payouts or $10,000 in approved simulated payouts, then risk review. Personalized max drawdown assigned per account (amount not published). Daily Loss Limit governed by the scaling plan (amount not published). 80% trader / 20% FFF. The Prestige page calls the Pro funded phase an \"End of Day Drawdown model\", the structure page \"End-of-Position (EOP)\"."
sources:
  - { field: platforms, url: "https://intercom.help/funded-futures-family/en/articles/15656536-connecting-your-trading-platform", accessed: "2026-10-23" }
  - { field: platforms, url: "https://www.fundedfuturesfamily.com/", accessed: "2026-10-23" }
  - { field: instruments, url: "https://intercom.help/funded-futures-family/en/articles/17708800-prohibited-trading-conduct", accessed: "2026-10-23" }
  - { field: automation, url: "https://intercom.help/funded-futures-family/en/articles/15892413-bots-algorithmic-trading-policy", accessed: "2026-10-23" }
  - { field: automation, url: "https://intercom.help/funded-futures-family/en/articles/17708800-prohibited-trading-conduct", accessed: "2026-10-23" }
  - { field: copy_trading, url: "https://intercom.help/funded-futures-family/en/articles/17708800-prohibited-trading-conduct", accessed: "2026-10-23" }
  - { field: vps_allowed, url: "https://intercom.help/funded-futures-family/en/articles/15892413-bots-algorithmic-trading-policy", accessed: "2026-10-23" }
  - { field: vps_allowed, url: "https://intercom.help/funded-futures-family/en/articles/17708800-prohibited-trading-conduct", accessed: "2026-10-23" }
  - { field: personal_device_required, url: "https://intercom.help/funded-futures-family/en/articles/17155892-device-sharing", accessed: "2026-10-23" }
  - { field: trading_day_reset, url: "https://intercom.help/funded-futures-family/en/articles/15892350-permitted-times-to-trade", accessed: "2026-10-23" }
  - { field: drawdown, url: "https://intercom.help/funded-futures-family/en/articles/15880840-professional-stage-structure", accessed: "2026-10-23" }
  - { field: drawdown, url: "https://intercom.help/funded-futures-family/en/articles/16911595-prestige-evaluation", accessed: "2026-10-23" }
  - { field: daily_loss_limit, url: "https://intercom.help/funded-futures-family/en/articles/15880840-professional-stage-structure", accessed: "2026-10-23" }
  - { field: consistency_rule, url: "https://intercom.help/funded-futures-family/en/articles/15880840-professional-stage-structure", accessed: "2026-10-23" }
  - { field: max_position, url: "https://intercom.help/funded-futures-family/en/articles/15880840-professional-stage-structure", accessed: "2026-10-23" }
  - { field: max_position, url: "https://intercom.help/funded-futures-family/en/articles/15891902-professional-stage-operational-policies", accessed: "2026-10-23" }
```

#### Professional Stage (Pro) — funded — 150K

```yaml
firm: funded-futures-family
plan: "Professional Stage (Pro execution environment)"
phase: funded
size_label: "150K"
size_usd: "150000.00"
instruments: [MNQ, NQ]
platforms: ["not published for the Pro execution environment"]
drawdown: { amount_usd: null, type: "other", stops_trailing_at: "Locks at $150100 (End-of-Position drawdown: updates only when a position is fully closed)" }
daily_loss_limit: { amount_usd: null, kind: null }
profit_target_usd: "none"
consistency_rule: { text: "There are no consistency rules in the Professional Stage", threshold_pct: null }
max_position: { minis: "case by case (Live Desk review)", micros: "case by case (Live Desk review)", scaling: "Scaling by EOD realized profit, applied only after Live Desk review and broker-side configuration" }
min_trading_days: null
trading_day_reset: { time: null, tz_as_published: null }
commission_per_contract: { mnq_usd: null, nq_usd: null, basis: null, includes: null }
copy_trading_allowed: conditional
automated_trading_allowed: no
automation_rule_text: "At Funded Futures Family, we don’t permit bot or algorithmic trading. / Bots, algorithms, expert advisors and any automated order entry are not allowed."
vps_allowed: conflict
personal_device_required: yes
status: conflict
not_confirmed_fields: ["commission_per_contract", "trading_day_reset", "vps_allowed (conflict)", "copy_trading_allowed (ambiguity vs automation ban)", "drawdown.amount_usd", "drawdown.type (conflict: EOP vs EOD)", "daily_loss_limit", "max_position", "min_trading_days"]
notes: "Live-market program reached after 3 approved payouts or $10,000 in approved simulated payouts, then risk review. Personalized max drawdown assigned per account (amount not published). Daily Loss Limit governed by the scaling plan (amount not published). 80% trader / 20% FFF. The Prestige page calls the Pro funded phase an \"End of Day Drawdown model\", the structure page \"End-of-Position (EOP)\"."
sources:
  - { field: platforms, url: "https://intercom.help/funded-futures-family/en/articles/15656536-connecting-your-trading-platform", accessed: "2026-10-23" }
  - { field: platforms, url: "https://www.fundedfuturesfamily.com/", accessed: "2026-10-23" }
  - { field: instruments, url: "https://intercom.help/funded-futures-family/en/articles/17708800-prohibited-trading-conduct", accessed: "2026-10-23" }
  - { field: automation, url: "https://intercom.help/funded-futures-family/en/articles/15892413-bots-algorithmic-trading-policy", accessed: "2026-10-23" }
  - { field: automation, url: "https://intercom.help/funded-futures-family/en/articles/17708800-prohibited-trading-conduct", accessed: "2026-10-23" }
  - { field: copy_trading, url: "https://intercom.help/funded-futures-family/en/articles/17708800-prohibited-trading-conduct", accessed: "2026-10-23" }
  - { field: vps_allowed, url: "https://intercom.help/funded-futures-family/en/articles/15892413-bots-algorithmic-trading-policy", accessed: "2026-10-23" }
  - { field: vps_allowed, url: "https://intercom.help/funded-futures-family/en/articles/17708800-prohibited-trading-conduct", accessed: "2026-10-23" }
  - { field: personal_device_required, url: "https://intercom.help/funded-futures-family/en/articles/17155892-device-sharing", accessed: "2026-10-23" }
  - { field: trading_day_reset, url: "https://intercom.help/funded-futures-family/en/articles/15892350-permitted-times-to-trade", accessed: "2026-10-23" }
  - { field: drawdown, url: "https://intercom.help/funded-futures-family/en/articles/15880840-professional-stage-structure", accessed: "2026-10-23" }
  - { field: drawdown, url: "https://intercom.help/funded-futures-family/en/articles/16911595-prestige-evaluation", accessed: "2026-10-23" }
  - { field: daily_loss_limit, url: "https://intercom.help/funded-futures-family/en/articles/15880840-professional-stage-structure", accessed: "2026-10-23" }
  - { field: consistency_rule, url: "https://intercom.help/funded-futures-family/en/articles/15880840-professional-stage-structure", accessed: "2026-10-23" }
  - { field: max_position, url: "https://intercom.help/funded-futures-family/en/articles/15880840-professional-stage-structure", accessed: "2026-10-23" }
  - { field: max_position, url: "https://intercom.help/funded-futures-family/en/articles/15891902-professional-stage-operational-policies", accessed: "2026-10-23" }
```

#### Live Stage (Rithmic) — live — 50K

```yaml
firm: funded-futures-family
plan: "Live Stage (Live Market Execution Environment)"
phase: live
size_label: "50K"
size_usd: "50000.00"
instruments: [MNQ, NQ]
platforms: ["Rithmic (execution infrastructure; connection client not named)"]
drawdown: { amount_usd: "2500.00", type: null, stops_trailing_at: null }
daily_loss_limit: { amount_usd: "1000.00", kind: null }
profit_target_usd: "none"
consistency_rule: { text: "There are no consistency rules in the Professional Stage (same lifecycle)", threshold_pct: null }
max_position: { minis: "1", micros: "10", scaling: "Limits may increase over time at Live Team discretion" }
min_trading_days: null
trading_day_reset: { time: null, tz_as_published: null }
commission_per_contract: { mnq_usd: null, nq_usd: null, basis: null, includes: null }
copy_trading_allowed: conditional
automated_trading_allowed: no
automation_rule_text: "At Funded Futures Family, we don’t permit bot or algorithmic trading. / Bots, algorithms, expert advisors and any automated order entry are not allowed."
vps_allowed: conflict
personal_device_required: yes
status: conflict
not_confirmed_fields: ["commission_per_contract", "trading_day_reset", "vps_allowed (conflict)", "copy_trading_allowed (ambiguity vs automation ban)", "drawdown.type", "drawdown.stops_trailing_at", "daily_loss_limit.kind", "min_trading_days"]
notes: "Execution infrastructure: Rithmic. \"Market Data: Not included; traders must select and subscribe to data services independently\". Reached after $5,000 in approved Professional Stage payouts or an initial Pro payout plus review; documentation and broker processing required. Live is the only phase in which a real-money account exists; the DLL kind and drawdown type are not published."
sources:
  - { field: platforms, url: "https://intercom.help/funded-futures-family/en/articles/15656536-connecting-your-trading-platform", accessed: "2026-10-23" }
  - { field: platforms, url: "https://www.fundedfuturesfamily.com/", accessed: "2026-10-23" }
  - { field: instruments, url: "https://intercom.help/funded-futures-family/en/articles/17708800-prohibited-trading-conduct", accessed: "2026-10-23" }
  - { field: automation, url: "https://intercom.help/funded-futures-family/en/articles/15892413-bots-algorithmic-trading-policy", accessed: "2026-10-23" }
  - { field: automation, url: "https://intercom.help/funded-futures-family/en/articles/17708800-prohibited-trading-conduct", accessed: "2026-10-23" }
  - { field: copy_trading, url: "https://intercom.help/funded-futures-family/en/articles/17708800-prohibited-trading-conduct", accessed: "2026-10-23" }
  - { field: vps_allowed, url: "https://intercom.help/funded-futures-family/en/articles/15892413-bots-algorithmic-trading-policy", accessed: "2026-10-23" }
  - { field: vps_allowed, url: "https://intercom.help/funded-futures-family/en/articles/17708800-prohibited-trading-conduct", accessed: "2026-10-23" }
  - { field: personal_device_required, url: "https://intercom.help/funded-futures-family/en/articles/17155892-device-sharing", accessed: "2026-10-23" }
  - { field: trading_day_reset, url: "https://intercom.help/funded-futures-family/en/articles/15892350-permitted-times-to-trade", accessed: "2026-10-23" }
  - { field: drawdown, url: "https://intercom.help/funded-futures-family/en/articles/15892228-live-stage-structure", accessed: "2026-10-23" }
  - { field: daily_loss_limit, url: "https://intercom.help/funded-futures-family/en/articles/15892228-live-stage-structure", accessed: "2026-10-23" }
  - { field: max_position, url: "https://intercom.help/funded-futures-family/en/articles/15892228-live-stage-structure", accessed: "2026-10-23" }
  - { field: platforms, url: "https://intercom.help/funded-futures-family/en/articles/15892228-live-stage-structure", accessed: "2026-10-23" }
```

#### Live Stage (Rithmic) — live — 100K

```yaml
firm: funded-futures-family
plan: "Live Stage (Live Market Execution Environment)"
phase: live
size_label: "100K"
size_usd: "100000.00"
instruments: [MNQ, NQ]
platforms: ["Rithmic (execution infrastructure; connection client not named)"]
drawdown: { amount_usd: "3500.00", type: null, stops_trailing_at: null }
daily_loss_limit: { amount_usd: "1000.00", kind: null }
profit_target_usd: "none"
consistency_rule: { text: "There are no consistency rules in the Professional Stage (same lifecycle)", threshold_pct: null }
max_position: { minis: "1", micros: "10", scaling: "Limits may increase over time at Live Team discretion" }
min_trading_days: null
trading_day_reset: { time: null, tz_as_published: null }
commission_per_contract: { mnq_usd: null, nq_usd: null, basis: null, includes: null }
copy_trading_allowed: conditional
automated_trading_allowed: no
automation_rule_text: "At Funded Futures Family, we don’t permit bot or algorithmic trading. / Bots, algorithms, expert advisors and any automated order entry are not allowed."
vps_allowed: conflict
personal_device_required: yes
status: conflict
not_confirmed_fields: ["commission_per_contract", "trading_day_reset", "vps_allowed (conflict)", "copy_trading_allowed (ambiguity vs automation ban)", "drawdown.type", "drawdown.stops_trailing_at", "daily_loss_limit.kind", "min_trading_days"]
notes: "Execution infrastructure: Rithmic. \"Market Data: Not included; traders must select and subscribe to data services independently\". Reached after $5,000 in approved Professional Stage payouts or an initial Pro payout plus review; documentation and broker processing required. Live is the only phase in which a real-money account exists; the DLL kind and drawdown type are not published."
sources:
  - { field: platforms, url: "https://intercom.help/funded-futures-family/en/articles/15656536-connecting-your-trading-platform", accessed: "2026-10-23" }
  - { field: platforms, url: "https://www.fundedfuturesfamily.com/", accessed: "2026-10-23" }
  - { field: instruments, url: "https://intercom.help/funded-futures-family/en/articles/17708800-prohibited-trading-conduct", accessed: "2026-10-23" }
  - { field: automation, url: "https://intercom.help/funded-futures-family/en/articles/15892413-bots-algorithmic-trading-policy", accessed: "2026-10-23" }
  - { field: automation, url: "https://intercom.help/funded-futures-family/en/articles/17708800-prohibited-trading-conduct", accessed: "2026-10-23" }
  - { field: copy_trading, url: "https://intercom.help/funded-futures-family/en/articles/17708800-prohibited-trading-conduct", accessed: "2026-10-23" }
  - { field: vps_allowed, url: "https://intercom.help/funded-futures-family/en/articles/15892413-bots-algorithmic-trading-policy", accessed: "2026-10-23" }
  - { field: vps_allowed, url: "https://intercom.help/funded-futures-family/en/articles/17708800-prohibited-trading-conduct", accessed: "2026-10-23" }
  - { field: personal_device_required, url: "https://intercom.help/funded-futures-family/en/articles/17155892-device-sharing", accessed: "2026-10-23" }
  - { field: trading_day_reset, url: "https://intercom.help/funded-futures-family/en/articles/15892350-permitted-times-to-trade", accessed: "2026-10-23" }
  - { field: drawdown, url: "https://intercom.help/funded-futures-family/en/articles/15892228-live-stage-structure", accessed: "2026-10-23" }
  - { field: daily_loss_limit, url: "https://intercom.help/funded-futures-family/en/articles/15892228-live-stage-structure", accessed: "2026-10-23" }
  - { field: max_position, url: "https://intercom.help/funded-futures-family/en/articles/15892228-live-stage-structure", accessed: "2026-10-23" }
  - { field: platforms, url: "https://intercom.help/funded-futures-family/en/articles/15892228-live-stage-structure", accessed: "2026-10-23" }
```

#### Live Stage (Rithmic) — live — 150K

```yaml
firm: funded-futures-family
plan: "Live Stage (Live Market Execution Environment)"
phase: live
size_label: "150K"
size_usd: "150000.00"
instruments: [MNQ, NQ]
platforms: ["Rithmic (execution infrastructure; connection client not named)"]
drawdown: { amount_usd: "4500.00", type: null, stops_trailing_at: null }
daily_loss_limit: { amount_usd: "1250.00", kind: null }
profit_target_usd: "none"
consistency_rule: { text: "There are no consistency rules in the Professional Stage (same lifecycle)", threshold_pct: null }
max_position: { minis: "1", micros: "10", scaling: "Limits may increase over time at Live Team discretion" }
min_trading_days: null
trading_day_reset: { time: null, tz_as_published: null }
commission_per_contract: { mnq_usd: null, nq_usd: null, basis: null, includes: null }
copy_trading_allowed: conditional
automated_trading_allowed: no
automation_rule_text: "At Funded Futures Family, we don’t permit bot or algorithmic trading. / Bots, algorithms, expert advisors and any automated order entry are not allowed."
vps_allowed: conflict
personal_device_required: yes
status: conflict
not_confirmed_fields: ["commission_per_contract", "trading_day_reset", "vps_allowed (conflict)", "copy_trading_allowed (ambiguity vs automation ban)", "drawdown.type", "drawdown.stops_trailing_at", "daily_loss_limit.kind", "min_trading_days"]
notes: "Execution infrastructure: Rithmic. \"Market Data: Not included; traders must select and subscribe to data services independently\". Reached after $5,000 in approved Professional Stage payouts or an initial Pro payout plus review; documentation and broker processing required. Live is the only phase in which a real-money account exists; the DLL kind and drawdown type are not published."
sources:
  - { field: platforms, url: "https://intercom.help/funded-futures-family/en/articles/15656536-connecting-your-trading-platform", accessed: "2026-10-23" }
  - { field: platforms, url: "https://www.fundedfuturesfamily.com/", accessed: "2026-10-23" }
  - { field: instruments, url: "https://intercom.help/funded-futures-family/en/articles/17708800-prohibited-trading-conduct", accessed: "2026-10-23" }
  - { field: automation, url: "https://intercom.help/funded-futures-family/en/articles/15892413-bots-algorithmic-trading-policy", accessed: "2026-10-23" }
  - { field: automation, url: "https://intercom.help/funded-futures-family/en/articles/17708800-prohibited-trading-conduct", accessed: "2026-10-23" }
  - { field: copy_trading, url: "https://intercom.help/funded-futures-family/en/articles/17708800-prohibited-trading-conduct", accessed: "2026-10-23" }
  - { field: vps_allowed, url: "https://intercom.help/funded-futures-family/en/articles/15892413-bots-algorithmic-trading-policy", accessed: "2026-10-23" }
  - { field: vps_allowed, url: "https://intercom.help/funded-futures-family/en/articles/17708800-prohibited-trading-conduct", accessed: "2026-10-23" }
  - { field: personal_device_required, url: "https://intercom.help/funded-futures-family/en/articles/17155892-device-sharing", accessed: "2026-10-23" }
  - { field: trading_day_reset, url: "https://intercom.help/funded-futures-family/en/articles/15892350-permitted-times-to-trade", accessed: "2026-10-23" }
  - { field: drawdown, url: "https://intercom.help/funded-futures-family/en/articles/15892228-live-stage-structure", accessed: "2026-10-23" }
  - { field: daily_loss_limit, url: "https://intercom.help/funded-futures-family/en/articles/15892228-live-stage-structure", accessed: "2026-10-23" }
  - { field: max_position, url: "https://intercom.help/funded-futures-family/en/articles/15892228-live-stage-structure", accessed: "2026-10-23" }
  - { field: platforms, url: "https://intercom.help/funded-futures-family/en/articles/15892228-live-stage-structure", accessed: "2026-10-23" }
```

## 10. Unverified / gaps / conflicts

| Field | Plans | Status | Detail |
|---|---|---|---|
| `vps_allowed` | all | **conflict** | S2 (Bots policy, Sept 8, 2026): "VPNs and VPSs are permitted and may be used at the trader’s discretion." vs S3/S3b (Prohibited Trading Conduct, updated today): "VPS, proxy and bot hosts are not allowed. A personal VPN is allowed." |
| `consistency_rule` | S2F Standard | **conflict** | S31/S32/S33 (help center): 25%. S54 (home page plan card): "45% consistency". Help center declared to govern over marketing (S54 disclosure), but not decided here |
| `consistency_rule` | Velocity funded | conflict (soft) | S29/S27: 40% on funded; S54 card: "No consistency in sim funded" (valid with Daily Payout Add-On per S27) |
| `drawdown.type` | Pro funded | **conflict** | S49: End-of-Position (EOP); S48 (Prestige): "End of Day Drawdown model" for the Pro funded phase |
| `max_position` | Premier+ funded (all sizes) | **conflict** | S20 funded table: 2/4/6/10 minis; S24 scaling plan max: 3/5/10/15 minis |
| `max_position` | S2F 25K | **conflict** | S31 table: 1 mini / 10 micros; S35 scaling plan: up to 3 minis / 30 micros |
| `max_position` | Premier+ 150K EOD evaluation | typo | S19 EOD table prints "10 Micros/100 Minis" (inverted); Intraday table and S20 print 10 minis / 100 micros |
| `max_position` | Prime, Premier+ evaluation | unclear | "Standard/Max" figures; no page defines the difference |
| `trading_day_reset` | all | unpublished | Only a 4:45 pm EST flatten / 6:00 pm EST reopen window (S5) found. Holiday rule unpublished |
| `commission_per_contract` | all | unpublished | No fee schedule found (§8) |
| `copy_trading_allowed` | all | ambiguity | S3 allows copying between own accounts but bans "any automated order entry"; no page says whether a copier is exempt |
| `personal_device_required` | all | closed R4: yes | S4: "Each trader must use their own device to access and trade on their accounts." Residual: nothing says whether a remote machine counts as the trader's own device |
| `automated_trading_allowed` | Live Stage / Pro | unknown | No Pro/Live-specific automation article; the general ban is in S2/S3 |
| `drawdown.stops_trailing_at` | Velocity, Accelerate 50K, Premier+ Intraday, Prestige, Base evaluation | unpublished | Lock behaviour not stated (S28/S39 say only "remains locked at that level" after a payout / never moves back down) |
| `drawdown.amount_usd`, `daily_loss_limit` | Pro funded | unpublished | "Personalized Max Drawdown, assigned per account"; DLL "governed by the Scaling Plan" |
| `daily_loss_limit.kind`, `drawdown.type` | Live Stage, Prestige | unpublished | Amounts published for Live, type and kind not |
| `min_trading_days` | Prime, Base, Premier+/S2F variants | partial | Evaluation minimums only where a page states them (Premier+ 2 days, Velocity 3, Prestige 5); payout-day requirements in notes |
| `instruments` | all | partial | MNQ/NQ named only in the Prohibited Trading Conduct article; no formal contract list article found |
| Copying to/from other firms | all | not mentioned | |
| Live Stage platform detail | Live | partial | Rithmic named; no connection guide |
| Pre-2026-09-09 Premier+ funded consistency | Premier+ legacy | unpublished | "retain their original funded consistency terms" |

## 11. Changelog

- 2026-10-23 — initial research (Lupa)
- 2026-10-23 — rev bump R4: `personal_device_required` closed as `yes` (Device Sharing article, verbatim in §7); `vps_allowed` changed from `unknown` to `conflict` (both official texts re-captured, unchanged, none withdrawn). `trading_day_reset` stays unpublished: crawled the help center collections (68 articles) and the terms page; only "All trades must close by 4:45 pm EST, and you can reopen trades at 6:00 pm EST." (https://intercom.help/funded-futures-family/en/articles/15892350-permitted-times-to-trade); no holiday rule (Lupa)
