# Leeloo Trading — Prop Firm Rules Research

| | |
|---|---|
| **Firm** | Leeloo Trading (Natural Trading, LLC d/b/a Leeloo Trading, Montana USA). Futures simulation ("Performance Based Trading and Contests"), Rithmic data and fills |
| **Official domains** | `leelootrading.com` (main site, plain curl works; `www.leelootrading.com` also resolves) · `support.leelootrading.com` (SmarterTrack knowledge base, **articles require a login**; linked from the main site's Support page) · `members.leelootrading.com` (aMember login, not used) |
| **Research date** | 2026-10-23 (all URLs below accessed 2026-10-23) |
| **Researcher** | Lupa (Prop Firm Rules Researcher) |
| **File status** | `partial`. The public site gives plan sizes, profit targets, drawdown amounts, contract limits, platforms, the 30% rule, the copy-trading prohibition across users and the closing-time rule. The detailed rules are in the knowledge base (behind login, `unverified (behind login)`): trailing-drawdown lock, trading-day reset, instruments, commissions, automation, VPS/device rules. See §10 |
| **Instruments covered** | `unknown`. MNQ and NQ are **not named** on any public page; the list is in KB article a52 ("Instruments", empty without login). Plan cards say "minis | micros" (10 micros per mini) |
| **Platforms (for routing)** | Rithmic-connected. Quote: "Ninjatrader 7 / 8" is listed first among Supported Trading Platforms (https://leelootrading.com/support). **NinjaTrader 8 is an offered platform.** See §2 |

Evidence (stripped text of each cited page): `/tmp/claude-1003/-srv-www-allwin--maestri-roles-6bf77a80-a1ed-43dc-bc00-08a480f178a5/c3ed71e7-3dd4-4f1a-8a03-9bc3a3fa7b49/scratchpad/r3/leeloo-trading/` (`txt/`, `raw/`). Temporary scratch storage; capture again at re-verification.

**Programs found.** Not an evaluation + funded account model. Flow quoted from the home page: "Step 1. Purchase a Leeloo™ Practice Account. Achieve the profit target (without meeting your loss limit) with 10 traded days." → "Step 2. Move into a Leeloo Performance Account™ ... Meet set benchmarks and receive the potential opportunity to earn real payouts." → "Step 3. Prove Consistency and Discipline: For the potential for your account to be copied via proprietary software at Leeloo's discretion." In this file the **Practice Account** is recorded as phase `evaluation` and the **Performance Account (PA)** as phase `funded`. Everything is simulation: "All trade performance is reflective of real Leeloo customers placing trades in simulation." Account families on the store: Foundation (LL) accounts (six sizes), LB Leeloo Bundle (same sizes, "Only one account within the bundle can qualify"), plus Entry, Weekly and Ignite specialty accounts (KB, behind login, not recorded).

### Source index (all accessed 2026-10-23)

| Key | URL |
|---|---|
| S1 | https://leelootrading.com/ (home: plan cards, FAQ, PA rules, "Trade with Flexibility") |
| S2 | https://leelootrading.com/support (FAQ: platforms, PA fees, multiple accounts, PA rules; same text as S1) |
| S3 | https://leelootrading.com/terms-of-service ("Last Updated: March 31, 2026") |
| S4 | https://leelootrading.com/merchant-products (Leeloo Store, third-party merchants) |
| S5 | https://leelootrading.com/risk-disclosure and https://leelootrading.com/performance-disclosure (no rule values) |
| S6 | https://support.leelootrading.com/kb/... (a44 drawdown, a48 min days, a52 instruments, a103 holding positions, a190 30% rule, a5, a71, a122, a171, a180, a181, a182): all `unverified (behind login)`; "You do not have permission to access this area." |

No sitemap exists (`/sitemap.xml` returns the 404 page) and the KB index is not listable without login. Pages `leelootrading.com/blog`, `/contact`, `/privacy-policy`, `/contest` were read and contain no trading rules.

---

## 2. Platforms

All accounts connect through Rithmic. Quote (S1/S2): "Choose your trading platform (see list below) ... Downloaded R|TraderPro ... Complete and sign all the Rithmic Documents . Connect your platform to Rithmic". Supported Trading Platforms as published: Ninjatrader 7 / 8; Rithmic Pro; EdgeProX (works on Mac, Windows, Linux); Motive Wave (Mac, Windows, Linux); Multicharts; ATAS; Finamark Systems; Jigsaw Daytradr; Trade Navigator; VolFix; Bookmap X-Ray; Investor/RT (Linnsoft); Sierra Charts; OptimusFLOW. Same list for Practice and Performance Accounts as far as published (no per-phase list). One Rithmic ID per trader: "Traders are only allowed one Rithmic ID." There is also a Leeloo mobile app ("to close trades, participate in contests and view trading progress").

## 3. Plans

### 3.1 Foundation (LL) Practice Account (phase `evaluation`). Source S1

| Size | Name | Minis / micros | Profit target | Trailing drawdown | Price | Reset |
|---|---|---|---|---|---|---|
| 25K | Aspire | 3 / 30 | 1500.00 | 1500.00 | $250 / month recurring | $85 |
| 50K | Launch | 8 / 80 | 3000.00 | 2500.00 | $280 / month recurring | $85 |
| 100K | Climb | 12 / 120 | 6000.00 | 3000.00 | $320 / month recurring | $85 |
| 150K | Cruise | 15 / 150 | 9000.00 | 5000.00 | $405 / month recurring | $85 |
| 250K | Burst | 25 / 250 | 15000.00 | 6500.00 | $725 / month recurring | $85 |
| 300K | Explode | 30 / 300 | 20000.00 | 7500.00 | $850 / month recurring | $85 |

Quotes (S1): "Aspire $25,000 3 minis | 30 micros $1,500 profit target $1,500 Trailing Drawdown $250 month / recurring"; "Explode $300,000 30 minis | 300 micros $20,000 profit target $7,500 Trailing Drawdown $850 month / recurring"; "This account is eligible for a reset $85". Qualification: "Meet your Profit Target while Managing Risk (do not let your account meet the drawdown). Trade for a minimum of 10 active U.S. Traded days." Flexibility list (S1): "Swing Trade · No Scaling · No Daily Drawdown · Trade Holidays & News · Trade Full Contracts · Multiple Accounts ONE login · No maximum trading period".

Drawdown definition (S1/S2, in the PA rules, referring to the Practice Account): "The Limited Trailing Minimum Account Balance (LTMAB) or Auto Liquidate Threshold Value (ALTV) is a rising trailing max drawdown that tracks your open equity intra-trade, based on the unrealized highest profit point in your Practice Account." and "This limit trails your highest unrealized profit throughout the Practice Account's life." Recorded as `trailing_intraday`. "Throughout the Practice Account's life" suggests it never locks, but the lock rule is in KB a44 (behind login), so `stops_trailing_at` stays `null` and is listed as a gap. On breach: "Upon reaching the LTMAB/ALTV, you'll see a message indicating restricted trading, with only administrators able to place orders." and "If this threshold is met, your position will automatically liquidate, resulting in the loss of the Performance Account."

Daily loss limit: none ("No Daily Drawdown"). Hard-pause behavior not applicable.

Profit-target caution (preset Q46): the plan card publishes one figure per size, labelled "profit target". The home page header lines "$25,000 | 3 contracts" give contract counts only (minis).

**LB Leeloo Bundle (same sizes, non-recurring, "Only one account within the bundle can qualify").** Same targets/drawdowns as above; prices LB Aspire $520, Launch $635, Climb $799, Cruise $850, Burst $950, Explode $1,250 ("month / non-recurring"). Low priority, no separate blocks.

### 3.2 Performance Account (PA) (phase `funded`)

Not a purchase: obtained by qualifying a Practice Account. Quote (S1/S2): "Once you qualify for a Performance Account, there are two options available. The monthly subscription is $88 per month per Performance Account. The one-time fee is $250 per Performance Account". "The one-time fee is designed for active traders that will trade their Performance Account a minimum of 12 trading days each calendar month. Deliberately placing minimal trades only to count as a trading day or keep a near-depleted account active, is not permissible."

PA rules as published (S1/S2): "1. 30% Rule: Daily profits in a PA should not exceed 30% of the net profit from the initial starting balance, promoting responsible risk management and consistent trading growth." "2. No Flipping Policy: Engaging in trades just to meet minimum activity requirements is discouraged. In PAs, less than 5% of trading days should be 'flipping' days." "3. Don't reach the Max drawdown." "4. Don't hold more than 3 micros at the close of the market. Up to 3 micros can be held through close without permission. For standard lots or more than 3 micros, admin approval is required, with sufficient profit to cover Maintenance Margin. Unapproved holding of more than 3 micros can result in account closure and loss of balance. Trades must be flat 15 minutes before market close, unless admin permission is granted." "5. Do not copy trade another user, to not have another user trade your account." "6. No SIM Abuse allowed."

PA size-specific drawdown, contract limits, payout split and cap are **not published** on public pages. The Terms say: "Unless Leeloo consents, in its sole and absolute discretion, Leeloo is not obligated to pay the Trader more than the maximum payout amount (“Account Payout Cap”) per user. Account Payout Caps are subject to change at any time." PA nature (S1 Payout Disclosure): "All trades presented for Payouts to customers should be considered hypothetical and should not be expected to be replicated in a live trading account. PA, also known at Leeloo Performance Account or Performance Account, may represent simulated accounts or copied accounts." The "Elite" level (KB a71) is behind login.

Phase `live`: not offered (no live phase published). Step 3 says an account may be copied "via proprietary software at Leeloo's discretion".

## 4. Trading-day reset

`unknown`. No public page states the session/trading-day reset time or timezone. The only time-related text is "Trades must be flat 15 minutes before market close, unless admin permission is granted." (S1/S2; KB a103 "Holding positions during the close" behind login). "Trade Holidays & News" is published, so no holiday blackout is stated. Trading days count as "active U.S. Traded days" (definition in KB a48, behind login).

## 5. Trade copying stance

- Copying between users is prohibited. Terms (S3): "Copying trades across users is expressly prohibited and your accounts will be closed." and "Coordinated trading, account farming, signal mirroring is prohibited and your accounts and user will be closed and disabled." PA rule (S1/S2): "Do not copy trade another user, to not have another user trade your account."
- Same-person copying across own accounts: **not stated** on any public page. Multiple accounts are allowed: "Traders are allowed to maintain multiple practice and Performance Accounts simultaneously." "Traders are only allowed one Rithmic ID." The Leeloo Store lists a "Copiers" category: "Copy your trades from one account to another" (S4), sold by "3rd party merchants. We are not responsible for, nor do we sponsor or otherwise endorse" them, so this is not a firm endorsement of copier use on Leeloo accounts.
- Copying with other firms: not mentioned.
- Leeloo itself may copy a PA: "For the potential for your account to be copied via proprietary software at Leeloo's discretion." (S1)
- Account-count limit: none published ("Can I have multiple accounts at the same time? Yes.").

## 6. Automated trading stance

(i) Classification: `unknown`. No public page states a rule for or against automated trading, bots, algos or EAs on Leeloo accounts. `automation_rule_text: null`. The Leeloo Store lists "Automated Strategies: Strategy that trades automatically for you based on a fixed set of rules" (S4, third-party merchants, disclaimer above). The Terms prohibit software on the website itself (S3): "Introduce software or automated agents to the Services, or access Leeloo or the Services so as to produce multiple accounts, generate automated messages, or to scrape, strip or mine data from Leeloo or the Services without our express written permission" (this reads as a website-conduct clause, not a trading rule; not used for the classification). Also (S3): "Trading activity is reviewed to identify behaviors that would not translate to real market conditions, including but not limited to toxic flow, latency arbitrage, and price feed exploitation. Accounts using these practices will result in account closure and users will be banned permanently." and "Reckless trading, gambling behavior are prohibited".

(ii) Named mechanisms:

| Mechanism | Status |
|---|---|
| Semi-automated trading | not mentioned |
| Trade copiers | mentioned (quote): "Copiers: Copy your trades from one account to another" (store listing S4); "Copying trades across users is expressly prohibited" (S3) |
| Webhooks / alerts / TradingView execution | not mentioned (TradingView appears only as a market-ticker widget) |
| ATM / auto brackets | not mentioned |
| Automated strategies | mentioned (quote): "Automated Strategies: Strategy that trades automatically for you based on a fixed set of rules" (store listing S4, third party) |

(iii) Differs by phase: unknown, nothing published.

## 7. Device / hosting rules and other rules

**Device / hosting.** `unknown`. No public page mentions VPS, VPN, remote server, or a requirement that orders come from the trader's own device (gap). Only identity/connection rules: "Make sure you sign up with your full name exactly as it reads on your government issued identification" and "Traders are only allowed one Rithmic ID." The user must run their own platform ("Connect your platform to Rithmic"). Nothing forbids a server-side relay, but nothing allows it either.

**Other rules (public).**

| Topic | Rule (quote) | Source |
|---|---|---|
| News and holidays | "Trade Holidays & News" | S1 |
| Overnight / swing | "Swing Trade" (Practice); PA: "Trades must be flat 15 minutes before market close, unless admin permission is granted." and no more than 3 micros held at the close without approval | S1 |
| Scaling | "No Scaling" | S1 |
| Mandatory stop | not mentioned | |
| Hedging | not mentioned | |
| Max accounts | none published; one Rithmic ID | S1 |
| Inactivity | PA one-time fee: "a minimum of 12 trading days each calendar month"; "keep a near-depleted account active, is not permissible" | S1 |
| Min / max trade duration | not mentioned | |
| Min trading days | "Trade for a minimum of 10 active U.S. Traded days" (no maximum: "No maximum trading period") | S1 |
| Resets | $85, "does NOT reset your Account" on renewal | S1, S3 |

## 8. Commissions

Not published on any public page (`null`, gap). Round-turn or per-side commission and exchange/NFA/platform fees may be in the KB or in the Rithmic/Leeloo fee schedule, behind login. The only fees published are subscription, reset and PA fees (§3).

## 9. Machine-readable blocks

Common to all blocks: platforms list is the one published for the firm (no per-plan list); `trading_day_reset`, `commission_per_contract`, automation, VPS and device fields are null/unknown (gaps). `instruments` is a placeholder as requested by the format; MNQ/NQ not confirmed (gap).

### 9.1 Foundation (LL) Practice Account — evaluation

#### Foundation (LL) Practice Account — Aspire — evaluation — 25K

```yaml
firm: leeloo-trading
plan: "Foundation (LL) Practice Account — Aspire"
phase: evaluation
size_label: "25K"
size_usd: "25000.00"
instruments: [MNQ, NQ]
platforms: ["NinjaTrader 7 / 8", "Rithmic Pro (R|Trader Pro)", "EdgeProX", "Motive Wave", "Multicharts", "ATAS", "Finamark Systems", "Jigsaw Daytradr", "Trade Navigator", "VolFix", "Bookmap X-Ray", "Investor/RT (Linnsoft)", "Sierra Charts", "OptimusFLOW"]
drawdown: { amount_usd: "1500.00", type: trailing_intraday, stops_trailing_at: null }
daily_loss_limit: { amount_usd: null, kind: none }
profit_target_usd: "1500.00"
consistency_rule: { text: "Not stated for the Practice Account. The 30% rule is published for Performance Accounts only.", threshold_pct: null }
max_position: { minis: "3", micros: "30", scaling: "No Scaling (published)" }
min_trading_days: "10"
trading_day_reset: { time: null, tz_as_published: null }
commission_per_contract: { mnq_usd: null, nq_usd: null, basis: null, includes: null }
copy_trading_allowed: unknown
automated_trading_allowed: unknown
automation_rule_text: null
vps_allowed: unknown
personal_device_required: unknown
status: partial
not_confirmed_fields: ["trading_day_reset", "commission_per_contract", "vps_allowed", "personal_device_required", "automated_trading_allowed", "automation_rule_text", "instruments_mnq_nq", "drawdown.stops_trailing_at", "copy_trading_allowed"]
notes: "Practice (simulated) account; price $250 per month recurring; reset $85. Drawdown amount is the plan-card figure; the rule text says it tracks open equity intra-trade. Whether it stops trailing is not published publicly (KB a44 behind login). MNQ/NQ are not named on public pages (KB a52 behind login); micros and minis are 10:1 on the plan card."
sources:
  - { field: "plan card (size, drawdown, target, contracts, price)", url: "https://leelootrading.com/", accessed: "2026-10-23" }
  - { field: "drawdown rule", url: "https://leelootrading.com/", accessed: "2026-10-23" }
  - { field: "no daily drawdown, no scaling", url: "https://leelootrading.com/", accessed: "2026-10-23" }
  - { field: "min trading days", url: "https://leelootrading.com/support", accessed: "2026-10-23" }
  - { field: "platforms", url: "https://leelootrading.com/support", accessed: "2026-10-23" }
```

#### Foundation (LL) Practice Account — Launch — evaluation — 50K

```yaml
firm: leeloo-trading
plan: "Foundation (LL) Practice Account — Launch"
phase: evaluation
size_label: "50K"
size_usd: "50000.00"
instruments: [MNQ, NQ]
platforms: ["NinjaTrader 7 / 8", "Rithmic Pro (R|Trader Pro)", "EdgeProX", "Motive Wave", "Multicharts", "ATAS", "Finamark Systems", "Jigsaw Daytradr", "Trade Navigator", "VolFix", "Bookmap X-Ray", "Investor/RT (Linnsoft)", "Sierra Charts", "OptimusFLOW"]
drawdown: { amount_usd: "2500.00", type: trailing_intraday, stops_trailing_at: null }
daily_loss_limit: { amount_usd: null, kind: none }
profit_target_usd: "3000.00"
consistency_rule: { text: "Not stated for the Practice Account. The 30% rule is published for Performance Accounts only.", threshold_pct: null }
max_position: { minis: "8", micros: "80", scaling: "No Scaling (published)" }
min_trading_days: "10"
trading_day_reset: { time: null, tz_as_published: null }
commission_per_contract: { mnq_usd: null, nq_usd: null, basis: null, includes: null }
copy_trading_allowed: unknown
automated_trading_allowed: unknown
automation_rule_text: null
vps_allowed: unknown
personal_device_required: unknown
status: partial
not_confirmed_fields: ["trading_day_reset", "commission_per_contract", "vps_allowed", "personal_device_required", "automated_trading_allowed", "automation_rule_text", "instruments_mnq_nq", "drawdown.stops_trailing_at", "copy_trading_allowed"]
notes: "Practice (simulated) account; price $280 per month recurring; reset $85. Drawdown amount is the plan-card figure; the rule text says it tracks open equity intra-trade. Whether it stops trailing is not published publicly (KB a44 behind login). MNQ/NQ are not named on public pages (KB a52 behind login); micros and minis are 10:1 on the plan card."
sources:
  - { field: "plan card (size, drawdown, target, contracts, price)", url: "https://leelootrading.com/", accessed: "2026-10-23" }
  - { field: "drawdown rule", url: "https://leelootrading.com/", accessed: "2026-10-23" }
  - { field: "no daily drawdown, no scaling", url: "https://leelootrading.com/", accessed: "2026-10-23" }
  - { field: "min trading days", url: "https://leelootrading.com/support", accessed: "2026-10-23" }
  - { field: "platforms", url: "https://leelootrading.com/support", accessed: "2026-10-23" }
```

#### Foundation (LL) Practice Account — Climb — evaluation — 100K

```yaml
firm: leeloo-trading
plan: "Foundation (LL) Practice Account — Climb"
phase: evaluation
size_label: "100K"
size_usd: "100000.00"
instruments: [MNQ, NQ]
platforms: ["NinjaTrader 7 / 8", "Rithmic Pro (R|Trader Pro)", "EdgeProX", "Motive Wave", "Multicharts", "ATAS", "Finamark Systems", "Jigsaw Daytradr", "Trade Navigator", "VolFix", "Bookmap X-Ray", "Investor/RT (Linnsoft)", "Sierra Charts", "OptimusFLOW"]
drawdown: { amount_usd: "3000.00", type: trailing_intraday, stops_trailing_at: null }
daily_loss_limit: { amount_usd: null, kind: none }
profit_target_usd: "6000.00"
consistency_rule: { text: "Not stated for the Practice Account. The 30% rule is published for Performance Accounts only.", threshold_pct: null }
max_position: { minis: "12", micros: "120", scaling: "No Scaling (published)" }
min_trading_days: "10"
trading_day_reset: { time: null, tz_as_published: null }
commission_per_contract: { mnq_usd: null, nq_usd: null, basis: null, includes: null }
copy_trading_allowed: unknown
automated_trading_allowed: unknown
automation_rule_text: null
vps_allowed: unknown
personal_device_required: unknown
status: partial
not_confirmed_fields: ["trading_day_reset", "commission_per_contract", "vps_allowed", "personal_device_required", "automated_trading_allowed", "automation_rule_text", "instruments_mnq_nq", "drawdown.stops_trailing_at", "copy_trading_allowed"]
notes: "Practice (simulated) account; price $320 per month recurring; reset $85. Drawdown amount is the plan-card figure; the rule text says it tracks open equity intra-trade. Whether it stops trailing is not published publicly (KB a44 behind login). MNQ/NQ are not named on public pages (KB a52 behind login); micros and minis are 10:1 on the plan card."
sources:
  - { field: "plan card (size, drawdown, target, contracts, price)", url: "https://leelootrading.com/", accessed: "2026-10-23" }
  - { field: "drawdown rule", url: "https://leelootrading.com/", accessed: "2026-10-23" }
  - { field: "no daily drawdown, no scaling", url: "https://leelootrading.com/", accessed: "2026-10-23" }
  - { field: "min trading days", url: "https://leelootrading.com/support", accessed: "2026-10-23" }
  - { field: "platforms", url: "https://leelootrading.com/support", accessed: "2026-10-23" }
```

#### Foundation (LL) Practice Account — Cruise — evaluation — 150K

```yaml
firm: leeloo-trading
plan: "Foundation (LL) Practice Account — Cruise"
phase: evaluation
size_label: "150K"
size_usd: "150000.00"
instruments: [MNQ, NQ]
platforms: ["NinjaTrader 7 / 8", "Rithmic Pro (R|Trader Pro)", "EdgeProX", "Motive Wave", "Multicharts", "ATAS", "Finamark Systems", "Jigsaw Daytradr", "Trade Navigator", "VolFix", "Bookmap X-Ray", "Investor/RT (Linnsoft)", "Sierra Charts", "OptimusFLOW"]
drawdown: { amount_usd: "5000.00", type: trailing_intraday, stops_trailing_at: null }
daily_loss_limit: { amount_usd: null, kind: none }
profit_target_usd: "9000.00"
consistency_rule: { text: "Not stated for the Practice Account. The 30% rule is published for Performance Accounts only.", threshold_pct: null }
max_position: { minis: "15", micros: "150", scaling: "No Scaling (published)" }
min_trading_days: "10"
trading_day_reset: { time: null, tz_as_published: null }
commission_per_contract: { mnq_usd: null, nq_usd: null, basis: null, includes: null }
copy_trading_allowed: unknown
automated_trading_allowed: unknown
automation_rule_text: null
vps_allowed: unknown
personal_device_required: unknown
status: partial
not_confirmed_fields: ["trading_day_reset", "commission_per_contract", "vps_allowed", "personal_device_required", "automated_trading_allowed", "automation_rule_text", "instruments_mnq_nq", "drawdown.stops_trailing_at", "copy_trading_allowed"]
notes: "Practice (simulated) account; price $405 per month recurring; reset $85. Drawdown amount is the plan-card figure; the rule text says it tracks open equity intra-trade. Whether it stops trailing is not published publicly (KB a44 behind login). MNQ/NQ are not named on public pages (KB a52 behind login); micros and minis are 10:1 on the plan card."
sources:
  - { field: "plan card (size, drawdown, target, contracts, price)", url: "https://leelootrading.com/", accessed: "2026-10-23" }
  - { field: "drawdown rule", url: "https://leelootrading.com/", accessed: "2026-10-23" }
  - { field: "no daily drawdown, no scaling", url: "https://leelootrading.com/", accessed: "2026-10-23" }
  - { field: "min trading days", url: "https://leelootrading.com/support", accessed: "2026-10-23" }
  - { field: "platforms", url: "https://leelootrading.com/support", accessed: "2026-10-23" }
```

#### Foundation (LL) Practice Account — Burst — evaluation — 250K

```yaml
firm: leeloo-trading
plan: "Foundation (LL) Practice Account — Burst"
phase: evaluation
size_label: "250K"
size_usd: "250000.00"
instruments: [MNQ, NQ]
platforms: ["NinjaTrader 7 / 8", "Rithmic Pro (R|Trader Pro)", "EdgeProX", "Motive Wave", "Multicharts", "ATAS", "Finamark Systems", "Jigsaw Daytradr", "Trade Navigator", "VolFix", "Bookmap X-Ray", "Investor/RT (Linnsoft)", "Sierra Charts", "OptimusFLOW"]
drawdown: { amount_usd: "6500.00", type: trailing_intraday, stops_trailing_at: null }
daily_loss_limit: { amount_usd: null, kind: none }
profit_target_usd: "15000.00"
consistency_rule: { text: "Not stated for the Practice Account. The 30% rule is published for Performance Accounts only.", threshold_pct: null }
max_position: { minis: "25", micros: "250", scaling: "No Scaling (published)" }
min_trading_days: "10"
trading_day_reset: { time: null, tz_as_published: null }
commission_per_contract: { mnq_usd: null, nq_usd: null, basis: null, includes: null }
copy_trading_allowed: unknown
automated_trading_allowed: unknown
automation_rule_text: null
vps_allowed: unknown
personal_device_required: unknown
status: partial
not_confirmed_fields: ["trading_day_reset", "commission_per_contract", "vps_allowed", "personal_device_required", "automated_trading_allowed", "automation_rule_text", "instruments_mnq_nq", "drawdown.stops_trailing_at", "copy_trading_allowed"]
notes: "Practice (simulated) account; price $725 per month recurring; reset $85. Drawdown amount is the plan-card figure; the rule text says it tracks open equity intra-trade. Whether it stops trailing is not published publicly (KB a44 behind login). MNQ/NQ are not named on public pages (KB a52 behind login); micros and minis are 10:1 on the plan card."
sources:
  - { field: "plan card (size, drawdown, target, contracts, price)", url: "https://leelootrading.com/", accessed: "2026-10-23" }
  - { field: "drawdown rule", url: "https://leelootrading.com/", accessed: "2026-10-23" }
  - { field: "no daily drawdown, no scaling", url: "https://leelootrading.com/", accessed: "2026-10-23" }
  - { field: "min trading days", url: "https://leelootrading.com/support", accessed: "2026-10-23" }
  - { field: "platforms", url: "https://leelootrading.com/support", accessed: "2026-10-23" }
```

#### Foundation (LL) Practice Account — Explode — evaluation — 300K

```yaml
firm: leeloo-trading
plan: "Foundation (LL) Practice Account — Explode"
phase: evaluation
size_label: "300K"
size_usd: "300000.00"
instruments: [MNQ, NQ]
platforms: ["NinjaTrader 7 / 8", "Rithmic Pro (R|Trader Pro)", "EdgeProX", "Motive Wave", "Multicharts", "ATAS", "Finamark Systems", "Jigsaw Daytradr", "Trade Navigator", "VolFix", "Bookmap X-Ray", "Investor/RT (Linnsoft)", "Sierra Charts", "OptimusFLOW"]
drawdown: { amount_usd: "7500.00", type: trailing_intraday, stops_trailing_at: null }
daily_loss_limit: { amount_usd: null, kind: none }
profit_target_usd: "20000.00"
consistency_rule: { text: "Not stated for the Practice Account. The 30% rule is published for Performance Accounts only.", threshold_pct: null }
max_position: { minis: "30", micros: "300", scaling: "No Scaling (published)" }
min_trading_days: "10"
trading_day_reset: { time: null, tz_as_published: null }
commission_per_contract: { mnq_usd: null, nq_usd: null, basis: null, includes: null }
copy_trading_allowed: unknown
automated_trading_allowed: unknown
automation_rule_text: null
vps_allowed: unknown
personal_device_required: unknown
status: partial
not_confirmed_fields: ["trading_day_reset", "commission_per_contract", "vps_allowed", "personal_device_required", "automated_trading_allowed", "automation_rule_text", "instruments_mnq_nq", "drawdown.stops_trailing_at", "copy_trading_allowed"]
notes: "Practice (simulated) account; price $850 per month recurring; reset $85. Drawdown amount is the plan-card figure; the rule text says it tracks open equity intra-trade. Whether it stops trailing is not published publicly (KB a44 behind login). MNQ/NQ are not named on public pages (KB a52 behind login); micros and minis are 10:1 on the plan card."
sources:
  - { field: "plan card (size, drawdown, target, contracts, price)", url: "https://leelootrading.com/", accessed: "2026-10-23" }
  - { field: "drawdown rule", url: "https://leelootrading.com/", accessed: "2026-10-23" }
  - { field: "no daily drawdown, no scaling", url: "https://leelootrading.com/", accessed: "2026-10-23" }
  - { field: "min trading days", url: "https://leelootrading.com/support", accessed: "2026-10-23" }
  - { field: "platforms", url: "https://leelootrading.com/support", accessed: "2026-10-23" }
```

### 9.2 Performance Account — funded

For the PA, amounts that are not published are `null`. Generic PA rules from §3.2 are in `consistency_rule`.

#### Performance Account (PA) — Aspire Practice size — funded — 25K

```yaml
firm: leeloo-trading
plan: "Performance Account (PA) — Aspire Practice size"
phase: funded
size_label: "25K"
size_usd: "25000.00"
instruments: [MNQ, NQ]
platforms: ["NinjaTrader 7 / 8", "Rithmic Pro (R|Trader Pro)", "EdgeProX", "Motive Wave", "Multicharts", "ATAS", "Finamark Systems", "Jigsaw Daytradr", "Trade Navigator", "VolFix", "Bookmap X-Ray", "Investor/RT (Linnsoft)", "Sierra Charts", "OptimusFLOW"]
drawdown: { amount_usd: null, type: trailing_intraday, stops_trailing_at: null }
daily_loss_limit: { amount_usd: null, kind: none }
profit_target_usd: none
consistency_rule: { text: "30% Rule: Daily profits in a PA should not exceed 30% of the net profit from the initial starting balance. No Flipping Policy: in PAs, less than 5% of trading days should be flipping days.", threshold_pct: "30" }
max_position: { minis: null, micros: null, scaling: null }
min_trading_days: null
trading_day_reset: { time: null, tz_as_published: null }
commission_per_contract: { mnq_usd: null, nq_usd: null, basis: null, includes: null }
copy_trading_allowed: unknown
automated_trading_allowed: unknown
automation_rule_text: null
vps_allowed: unknown
personal_device_required: unknown
status: partial
not_confirmed_fields: ["trading_day_reset", "commission_per_contract", "vps_allowed", "personal_device_required", "automated_trading_allowed", "automation_rule_text", "instruments_mnq_nq", "drawdown.stops_trailing_at", "copy_trading_allowed", "drawdown.amount_usd", "max_position", "daily_loss_limit", "min_trading_days"]
notes: "PA-specific size, drawdown amount, contract limits and payout terms are not published on public pages. A PA may be a simulated or a copied account (Payout Disclosure). Close rule: no more than 3 micros held at the close without admin approval; flat 15 minutes before market close unless permission granted. PA fee: $88 per month or $250 one-time."
sources:
  - { field: "PA rules (30% rule, flipping, close, no copying)", url: "https://leelootrading.com/", accessed: "2026-10-23" }
  - { field: "PA fees", url: "https://leelootrading.com/support", accessed: "2026-10-23" }
  - { field: "PA may be simulated or copied", url: "https://leelootrading.com/", accessed: "2026-10-23" }
```

#### Performance Account (PA) — Launch Practice size — funded — 50K

```yaml
firm: leeloo-trading
plan: "Performance Account (PA) — Launch Practice size"
phase: funded
size_label: "50K"
size_usd: "50000.00"
instruments: [MNQ, NQ]
platforms: ["NinjaTrader 7 / 8", "Rithmic Pro (R|Trader Pro)", "EdgeProX", "Motive Wave", "Multicharts", "ATAS", "Finamark Systems", "Jigsaw Daytradr", "Trade Navigator", "VolFix", "Bookmap X-Ray", "Investor/RT (Linnsoft)", "Sierra Charts", "OptimusFLOW"]
drawdown: { amount_usd: null, type: trailing_intraday, stops_trailing_at: null }
daily_loss_limit: { amount_usd: null, kind: none }
profit_target_usd: none
consistency_rule: { text: "30% Rule: Daily profits in a PA should not exceed 30% of the net profit from the initial starting balance. No Flipping Policy: in PAs, less than 5% of trading days should be flipping days.", threshold_pct: "30" }
max_position: { minis: null, micros: null, scaling: null }
min_trading_days: null
trading_day_reset: { time: null, tz_as_published: null }
commission_per_contract: { mnq_usd: null, nq_usd: null, basis: null, includes: null }
copy_trading_allowed: unknown
automated_trading_allowed: unknown
automation_rule_text: null
vps_allowed: unknown
personal_device_required: unknown
status: partial
not_confirmed_fields: ["trading_day_reset", "commission_per_contract", "vps_allowed", "personal_device_required", "automated_trading_allowed", "automation_rule_text", "instruments_mnq_nq", "drawdown.stops_trailing_at", "copy_trading_allowed", "drawdown.amount_usd", "max_position", "daily_loss_limit", "min_trading_days"]
notes: "PA-specific size, drawdown amount, contract limits and payout terms are not published on public pages. A PA may be a simulated or a copied account (Payout Disclosure). Close rule: no more than 3 micros held at the close without admin approval; flat 15 minutes before market close unless permission granted. PA fee: $88 per month or $250 one-time."
sources:
  - { field: "PA rules (30% rule, flipping, close, no copying)", url: "https://leelootrading.com/", accessed: "2026-10-23" }
  - { field: "PA fees", url: "https://leelootrading.com/support", accessed: "2026-10-23" }
  - { field: "PA may be simulated or copied", url: "https://leelootrading.com/", accessed: "2026-10-23" }
```

#### Performance Account (PA) — Climb Practice size — funded — 100K

```yaml
firm: leeloo-trading
plan: "Performance Account (PA) — Climb Practice size"
phase: funded
size_label: "100K"
size_usd: "100000.00"
instruments: [MNQ, NQ]
platforms: ["NinjaTrader 7 / 8", "Rithmic Pro (R|Trader Pro)", "EdgeProX", "Motive Wave", "Multicharts", "ATAS", "Finamark Systems", "Jigsaw Daytradr", "Trade Navigator", "VolFix", "Bookmap X-Ray", "Investor/RT (Linnsoft)", "Sierra Charts", "OptimusFLOW"]
drawdown: { amount_usd: null, type: trailing_intraday, stops_trailing_at: null }
daily_loss_limit: { amount_usd: null, kind: none }
profit_target_usd: none
consistency_rule: { text: "30% Rule: Daily profits in a PA should not exceed 30% of the net profit from the initial starting balance. No Flipping Policy: in PAs, less than 5% of trading days should be flipping days.", threshold_pct: "30" }
max_position: { minis: null, micros: null, scaling: null }
min_trading_days: null
trading_day_reset: { time: null, tz_as_published: null }
commission_per_contract: { mnq_usd: null, nq_usd: null, basis: null, includes: null }
copy_trading_allowed: unknown
automated_trading_allowed: unknown
automation_rule_text: null
vps_allowed: unknown
personal_device_required: unknown
status: partial
not_confirmed_fields: ["trading_day_reset", "commission_per_contract", "vps_allowed", "personal_device_required", "automated_trading_allowed", "automation_rule_text", "instruments_mnq_nq", "drawdown.stops_trailing_at", "copy_trading_allowed", "drawdown.amount_usd", "max_position", "daily_loss_limit", "min_trading_days"]
notes: "PA-specific size, drawdown amount, contract limits and payout terms are not published on public pages. A PA may be a simulated or a copied account (Payout Disclosure). Close rule: no more than 3 micros held at the close without admin approval; flat 15 minutes before market close unless permission granted. PA fee: $88 per month or $250 one-time."
sources:
  - { field: "PA rules (30% rule, flipping, close, no copying)", url: "https://leelootrading.com/", accessed: "2026-10-23" }
  - { field: "PA fees", url: "https://leelootrading.com/support", accessed: "2026-10-23" }
  - { field: "PA may be simulated or copied", url: "https://leelootrading.com/", accessed: "2026-10-23" }
```

#### Performance Account (PA) — Cruise Practice size — funded — 150K

```yaml
firm: leeloo-trading
plan: "Performance Account (PA) — Cruise Practice size"
phase: funded
size_label: "150K"
size_usd: "150000.00"
instruments: [MNQ, NQ]
platforms: ["NinjaTrader 7 / 8", "Rithmic Pro (R|Trader Pro)", "EdgeProX", "Motive Wave", "Multicharts", "ATAS", "Finamark Systems", "Jigsaw Daytradr", "Trade Navigator", "VolFix", "Bookmap X-Ray", "Investor/RT (Linnsoft)", "Sierra Charts", "OptimusFLOW"]
drawdown: { amount_usd: null, type: trailing_intraday, stops_trailing_at: null }
daily_loss_limit: { amount_usd: null, kind: none }
profit_target_usd: none
consistency_rule: { text: "30% Rule: Daily profits in a PA should not exceed 30% of the net profit from the initial starting balance. No Flipping Policy: in PAs, less than 5% of trading days should be flipping days.", threshold_pct: "30" }
max_position: { minis: null, micros: null, scaling: null }
min_trading_days: null
trading_day_reset: { time: null, tz_as_published: null }
commission_per_contract: { mnq_usd: null, nq_usd: null, basis: null, includes: null }
copy_trading_allowed: unknown
automated_trading_allowed: unknown
automation_rule_text: null
vps_allowed: unknown
personal_device_required: unknown
status: partial
not_confirmed_fields: ["trading_day_reset", "commission_per_contract", "vps_allowed", "personal_device_required", "automated_trading_allowed", "automation_rule_text", "instruments_mnq_nq", "drawdown.stops_trailing_at", "copy_trading_allowed", "drawdown.amount_usd", "max_position", "daily_loss_limit", "min_trading_days"]
notes: "PA-specific size, drawdown amount, contract limits and payout terms are not published on public pages. A PA may be a simulated or a copied account (Payout Disclosure). Close rule: no more than 3 micros held at the close without admin approval; flat 15 minutes before market close unless permission granted. PA fee: $88 per month or $250 one-time."
sources:
  - { field: "PA rules (30% rule, flipping, close, no copying)", url: "https://leelootrading.com/", accessed: "2026-10-23" }
  - { field: "PA fees", url: "https://leelootrading.com/support", accessed: "2026-10-23" }
  - { field: "PA may be simulated or copied", url: "https://leelootrading.com/", accessed: "2026-10-23" }
```

#### Performance Account (PA) — Burst Practice size — funded — 250K

```yaml
firm: leeloo-trading
plan: "Performance Account (PA) — Burst Practice size"
phase: funded
size_label: "250K"
size_usd: "250000.00"
instruments: [MNQ, NQ]
platforms: ["NinjaTrader 7 / 8", "Rithmic Pro (R|Trader Pro)", "EdgeProX", "Motive Wave", "Multicharts", "ATAS", "Finamark Systems", "Jigsaw Daytradr", "Trade Navigator", "VolFix", "Bookmap X-Ray", "Investor/RT (Linnsoft)", "Sierra Charts", "OptimusFLOW"]
drawdown: { amount_usd: null, type: trailing_intraday, stops_trailing_at: null }
daily_loss_limit: { amount_usd: null, kind: none }
profit_target_usd: none
consistency_rule: { text: "30% Rule: Daily profits in a PA should not exceed 30% of the net profit from the initial starting balance. No Flipping Policy: in PAs, less than 5% of trading days should be flipping days.", threshold_pct: "30" }
max_position: { minis: null, micros: null, scaling: null }
min_trading_days: null
trading_day_reset: { time: null, tz_as_published: null }
commission_per_contract: { mnq_usd: null, nq_usd: null, basis: null, includes: null }
copy_trading_allowed: unknown
automated_trading_allowed: unknown
automation_rule_text: null
vps_allowed: unknown
personal_device_required: unknown
status: partial
not_confirmed_fields: ["trading_day_reset", "commission_per_contract", "vps_allowed", "personal_device_required", "automated_trading_allowed", "automation_rule_text", "instruments_mnq_nq", "drawdown.stops_trailing_at", "copy_trading_allowed", "drawdown.amount_usd", "max_position", "daily_loss_limit", "min_trading_days"]
notes: "PA-specific size, drawdown amount, contract limits and payout terms are not published on public pages. A PA may be a simulated or a copied account (Payout Disclosure). Close rule: no more than 3 micros held at the close without admin approval; flat 15 minutes before market close unless permission granted. PA fee: $88 per month or $250 one-time."
sources:
  - { field: "PA rules (30% rule, flipping, close, no copying)", url: "https://leelootrading.com/", accessed: "2026-10-23" }
  - { field: "PA fees", url: "https://leelootrading.com/support", accessed: "2026-10-23" }
  - { field: "PA may be simulated or copied", url: "https://leelootrading.com/", accessed: "2026-10-23" }
```

#### Performance Account (PA) — Explode Practice size — funded — 300K

```yaml
firm: leeloo-trading
plan: "Performance Account (PA) — Explode Practice size"
phase: funded
size_label: "300K"
size_usd: "300000.00"
instruments: [MNQ, NQ]
platforms: ["NinjaTrader 7 / 8", "Rithmic Pro (R|Trader Pro)", "EdgeProX", "Motive Wave", "Multicharts", "ATAS", "Finamark Systems", "Jigsaw Daytradr", "Trade Navigator", "VolFix", "Bookmap X-Ray", "Investor/RT (Linnsoft)", "Sierra Charts", "OptimusFLOW"]
drawdown: { amount_usd: null, type: trailing_intraday, stops_trailing_at: null }
daily_loss_limit: { amount_usd: null, kind: none }
profit_target_usd: none
consistency_rule: { text: "30% Rule: Daily profits in a PA should not exceed 30% of the net profit from the initial starting balance. No Flipping Policy: in PAs, less than 5% of trading days should be flipping days.", threshold_pct: "30" }
max_position: { minis: null, micros: null, scaling: null }
min_trading_days: null
trading_day_reset: { time: null, tz_as_published: null }
commission_per_contract: { mnq_usd: null, nq_usd: null, basis: null, includes: null }
copy_trading_allowed: unknown
automated_trading_allowed: unknown
automation_rule_text: null
vps_allowed: unknown
personal_device_required: unknown
status: partial
not_confirmed_fields: ["trading_day_reset", "commission_per_contract", "vps_allowed", "personal_device_required", "automated_trading_allowed", "automation_rule_text", "instruments_mnq_nq", "drawdown.stops_trailing_at", "copy_trading_allowed", "drawdown.amount_usd", "max_position", "daily_loss_limit", "min_trading_days"]
notes: "PA-specific size, drawdown amount, contract limits and payout terms are not published on public pages. A PA may be a simulated or a copied account (Payout Disclosure). Close rule: no more than 3 micros held at the close without admin approval; flat 15 minutes before market close unless permission granted. PA fee: $88 per month or $250 one-time."
sources:
  - { field: "PA rules (30% rule, flipping, close, no copying)", url: "https://leelootrading.com/", accessed: "2026-10-23" }
  - { field: "PA fees", url: "https://leelootrading.com/support", accessed: "2026-10-23" }
  - { field: "PA may be simulated or copied", url: "https://leelootrading.com/", accessed: "2026-10-23" }
```

## 10. Unverified / gaps / conflicts

| Field | State | Detail |
|---|---|---|
| Conflicts | none | No two official pages disagree on a value. The home page header says "3 contracts" for Aspire while the cards say "3 minis | 30 micros" (same limit, wording) |
| Knowledge base (support.leelootrading.com/kb) | unverified (behind login) | a44 drawdown, a48 min days, a52 instruments, a103 holding at close, a190 30% rule, a71 Elite, a171 Ignite, a180/a181 Entry/Weekly, a182 Bundle, a5, a122: "You do not have permission to access this area." Not read, not guessed |
| Drawdown lock (`stops_trailing_at`) | unknown | Public text says it trails "throughout the Practice Account's life"; lock rule not published |
| Trading-day reset time and timezone | unknown | not published |
| Instruments (MNQ, NQ) | unverified | not named publicly (KB a52) |
| Commission per contract (MNQ, NQ) | not published | basis and included fees unknown |
| Automation / bots | unknown | no stated rule; only third-party store listing |
| Same-person copying across own accounts | unknown | only cross-user copying is prohibited |
| VPS / VPN / remote server / personal device | unknown | not mentioned publicly |
| PA size-specific drawdown, contracts, payout split/cap, activation | not published | public text covers generic rules only |
| Phase `live` | not offered | none published |
| Entry / Weekly / Ignite / Elite programs | unverified (behind login) | |

## 11. Changelog

- 2026-10-23 — initial research (Lupa)
