# Phoenix Trader Funding — Prop Firm Rules Research

| | |
|---|---|
| **Firm** | Phoenix Trader Funding (futures only; simulated accounts, live capital via EdgeClear after funding) |
| **Official domains** | `phoenixtraderfunding.com` (main site and Help Center `/faq`; verified: it is the site linked from its own pages, serves the sitemap, terms and billing terms, and its Help Center is a React app) · `server.phoenixtraderfunding.com/public/api/` (the site's own backend; the `/faqs/fetch` and `/packages/fetch` JSON endpoints feed the Help Center and the pricing pages, found in the site's JS bundle) · `thortradecopier.com` (Thor copier, linked from the site, not read) · `phxtf.com` (press, not read) |
| **Research date** | 2026-10-23 (all URLs below accessed 2026-10-23) |
| **Researcher** | Lupa (Prop Firm Rules Researcher) |
| **File status** | `partial`. Verified from the firm's own Help Center: profit targets, drawdown amounts and types, consistency, position limits, commissions, copy trading, automation, VPS, platforms, overnight, news. Not published anywhere found: trading-day reset time and time zone, live drawdown amounts and live starting size, exchange-fee inclusion in commissions. One conflict: Merit tiers and ES commission on the `/merit-accounts` marketing page vs the Help Center (section 10). |
| **Instruments covered** | MNQ and NQ are in the commission table (CME). Other futures also listed. |
| **Platforms (for routing)** | Depends on the data feed. **TFeed**: Odin (Phoenix web platform) and Quantower connect directly; NinjaTrader, Tradovate, TradingView and Rithmic-leader platforms connect through Phoenix's copier Thor (a separate "leader" account with market data is required). **dxFeed**: Odin, Atas, Quantower, DeepChart, DeepDom. **Live funded**: Rithmic via EdgeClear. |

**How the text was captured.** The Help Center pages are JavaScript-rendered, so the page text was captured from the site's own backend endpoint that renders them (`https://server.phoenixtraderfunding.com/public/api/faqs/fetch`, 95 articles) and converted to plain text. Every quote below was matched character by character against that captured text. The public URL of each article is `https://phoenixtraderfunding.com/faq/detail/<category-slug>/<article-slug>`; those are the "URL" values used below. The price/size package data comes from `https://server.phoenixtraderfunding.com/public/api/packages/fetch` (cross-check only). The Terms of Use text was taken from the site's JS bundle for `https://phoenixtraderfunding.com/terms`. Evidence in scratch (temporary): `/tmp/claude-1003/-srv-www-allwin--maestri-roles-6bf77a80-a1ed-43dc-bc00-08a480f178a5/c3ed71e7-3dd4-4f1a-8a03-9bc3a3fa7b49/scratchpad/r3/phoenix-trader-funding/` (`faqs.txt`, `faqs_fetch.json`, `packages_fetch.json`, `terms.txt`).

**Programs found (current, purchasable).** Four lines: **Classic** ($25K Starter, $50K Growth, $100K Scale; TFeed or dxFeed), **Daily** ($10K Seed, $25K Starter, $50K Growth; TFeed only), **Spark** ($10K Seed, $25K Starter; TFeed only; Spark Growth $50K is closed to new purchases), **Merit** (single $2,000 size, 30-day review, one attempt). Phases: evaluation ("challenge") → pre-funded (simulated funded, paid) → Live Funded (EdgeClear, Rithmic); Merit goes straight to live. No 150K, 200K or 300K size exists in the current lineup. Classic Starter/Growth/Scale "V3" names appear in the package data; older Phoenix 2.0 products are not described on the live Help Center.

**Firm terminology.** "TFeed" = Phoenix's own execution feed (no market data included). "dxFeed" = a feed with included Level 1 data. "Thor" = Phoenix's server-based trade copier. "Odin" = Phoenix's web trading platform. "Pre-funded" = the paid simulated funded stage.

---

## 1. Source index (all accessed 2026-10-23)

| Key | Source |
|---|---|
| F-classic-rules | https://phoenixtraderfunding.com/faq/detail/classic-accounts/what-are-the-rules-of-the-classic-accounts |
| F-classic-payout | https://phoenixtraderfunding.com/faq/detail/classic-accounts/classic-accounts-payout-rules |
| F-classic-scaling | https://phoenixtraderfunding.com/faq/detail/classic-accounts/scaling-plan-classic-accounts |
| F-daily-rules | https://phoenixtraderfunding.com/faq/detail/daily-accounts/what-are-the-rules-of-the-daily-accounts |
| F-daily-payout | https://phoenixtraderfunding.com/faq/detail/daily-accounts/daily-accounts-payout-rules |
| F-daily-scaling | https://phoenixtraderfunding.com/faq/detail/daily-accounts/scaling-plan-daily-accounts |
| F-spark-rules | https://phoenixtraderfunding.com/faq/detail/spark-accounts/what-are-the-rules-of-the-spark-accounts |
| F-spark-payout | https://phoenixtraderfunding.com/faq/detail/spark-accounts/spark-accounts-payout-rules |
| F-spark-scaling | https://phoenixtraderfunding.com/faq/detail/spark-accounts/scaling-plan-spark-accounts |
| F-merit-rules | https://phoenixtraderfunding.com/faq/detail/merit-accounts/what-is-a-merit-account-how-do-they-work-what-are-the-rules-for-a-merit-account |
| F-merit-payout | https://phoenixtraderfunding.com/faq/detail/merit-accounts/merit-accounts-payout-rules |
| F-merit-live | https://phoenixtraderfunding.com/faq/detail/merit-accounts/how-much-money-is-in-the-live-funded-accounts-merit-accounts |
| F-commissions | https://phoenixtraderfunding.com/faq/detail/risk-management/commissionsfees |
| F-copy | https://phoenixtraderfunding.com/faq/detail/risk-management/is-copy-trading-allowed |
| F-prohibited | https://phoenixtraderfunding.com/faq/detail/most-asked-questions/are-there-any-forbidden-or-prohibited-strategies |
| F-scalping | https://phoenixtraderfunding.com/faq/detail/most-asked-questions/do-you-allow-scalping |
| F-ninjatrader | https://phoenixtraderfunding.com/faq/detail/most-asked-questions/do-you-support-ninjatrader |
| F-platforms | https://phoenixtraderfunding.com/faq/detail/dont-know-where-to-start/on-which-platforms-can-i-trade |
| F-vpn-vps | https://phoenixtraderfunding.com/faq/detail/most-asked-questions/do-you-allow-vpn-vps |
| F-max-accounts | https://phoenixtraderfunding.com/faq/detail/most-asked-questions/how-many-accounts-can-i-trade-maximum-number-of-accounts |
| F-overnight | https://phoenixtraderfunding.com/faq/detail/risk-management/do-you-allow-overnightoverweek-positions |
| F-hedging | https://phoenixtraderfunding.com/faq/detail/most-asked-questions/do-you-allow-hedging-on-your-accounts |
| F-inactivity | https://phoenixtraderfunding.com/faq/detail/most-asked-questions/is-there-an-inactivity-rule |
| F-news | https://phoenixtraderfunding.com/faq/detail/most-asked-questions/can-i-trade-the-news |
| F-consistency | https://phoenixtraderfunding.com/faq/detail/most-asked-questions/whats-a-consistency-rule |
| F-trailing | https://phoenixtraderfunding.com/faq/detail/dont-know-where-to-start/what-is-a-trailing-drawdown-and-how-is-it-calculated |
| F-live-rules | https://phoenixtraderfunding.com/faq/detail/most-asked-questions/what-are-the-rules-for-being-live-funded |
| F-broker | https://phoenixtraderfunding.com/faq/detail/most-asked-questions/which-broker-will-host-your-live-account |
| F-funded-rules | https://phoenixtraderfunding.com/faq/detail/most-asked-questions/is-there-any-specific-rule-for-funded-accounts |
| F-payout-rules | https://phoenixtraderfunding.com/faq/detail/payout/what-are-our-payout-rules |
| T-terms | https://phoenixtraderfunding.com/terms (text from the JS bundle `assets/terms-45e6c76c.js`) |
| P-packages | https://server.phoenixtraderfunding.com/public/api/packages/fetch (site backend, JSON) |
| P-merit-page | https://phoenixtraderfunding.com/merit-accounts (marketing text from `assets/AccountsMerit-6a9f734d.js`) |

---

## 2. Platforms

Per line and phase, as published:

| Line / phase | Feed | Platforms as published |
|---|---|---|
| Classic, evaluation and pre-funded | TFeed (checkout choice) | Direct: Odin, Quantower (needs an addon). Through Thor (leader account needed): NinjaTrader, Tradovate, TradingView; Rithmic leader: Sierra Chart, MultiCharts, Jigsaw, Bookmap, MotiveWave, Investor/RT, Tiger.Trade, and others. |
| Classic, evaluation and pre-funded | dxFeed (checkout choice) | Direct, no copier: Odin, Atas, Quantower, DeepChart, DeepDom. |
| Daily, Spark | TFeed only | Same as the TFeed row. |
| Merit | Not stated in the Help Center (package data lists both feed IDs) | Unknown. |
| Live Funded (all lines) | Rithmic EdgeClear | "Anything that speaks Rithmic will connect to it". |

> "Exactly two platforms connect straight to a TFeed account. Everything else reaches it through Thor, which is the next section."
> (F-platforms, https://phoenixtraderfunding.com/faq/detail/dont-know-where-to-start/on-which-platforms-can-i-trade)

> "Connect NinjaTrader through the Tradovate connection, using your TFeed credentials"
> "Thor ships a native NinjaTrader addon. You trade NinjaTrader's own Sim101 account exactly as you normally would, and the addon copies every execution into your Phoenix TFeed account."
> "TFeed carries no market data of its own, so Sim101 is where your charts and prices come from."
> (F-ninjatrader, https://phoenixtraderfunding.com/faq/detail/most-asked-questions/do-you-support-ninjatrader)

> "TradingView has two routes, and both work. Connect it through Tradovate, or use Thor's own native TradingView integration with webhooks."
> (F-platforms)

> "Live Funded accounts run on Rithmic through our broker partner EdgeClear, so a Rithmic setup carries across when you graduate."
> (F-platforms)

> "EdgeClear. Every Live Funded account is hosted there, on a Rithmic EdgeClear connection, whichever feed you evaluated on."
> (F-broker, https://phoenixtraderfunding.com/faq/detail/most-asked-questions/which-broker-will-host-your-live-account)

Notes: NinjaTrader 8 is supported only on TFeed accounts, either through the Tradovate connection (log in to NT8's Tradovate connection with the TFeed credentials), or through Phoenix's native Thor addon (the trader's orders go to NT8's own Sim101 account and the addon copies every execution into the Phoenix account). The Thor addon route needs the trader's own NinjaTrader subscription with market data. NinjaTrader is not in the dxFeed platform list. On the live account the platform is any Rithmic-compatible platform (NinjaTrader supports Rithmic).

---

## 3. Plans

Standard facts for all lines: no daily loss limit except the Merit challenge's "25% Dynamic (Soft)" daily drawdown; overnight and over-week positions prohibited on every line and phase; news trading allowed; scalping with no minimum trade duration; no mandatory stop requirement found; Classic and Daily profit splits 90% and 80%, Spark 90%, Merit adaptive 50/50 then 80%.

### 3.1 Classic

| Field | Starter 25K | Growth 50K | Scale 100K |
|---|---|---|---|
| Profit target (evaluation) | 1500.00 | 3000.00 | 6000.00 |
| Maximum drawdown | 1500.00 | 2000.00 | 3000.00 |
| Drawdown type | end-of-day trailing; stops trailing at the starting balance (static) | same | same |
| Daily loss limit | none | none | none |
| Consistency | evaluation 50%, funded none | same | same |
| Min trading days (evaluation) | 2 | 2 | 2 |
| Scaling (applies to challenge and funded) | 1 mini/10 micros below $1,000 profit; 1/15 from $1,000 to $2,000 (top) | 1/10 below $1,000; 2/20 to $2,000; 3/30 to $4,000 (top) | 2/20 below $1,000; 3/30 to $2,000; 4/40 to $4,000; 6/60 to $6,000 (top) |
| Max position ceiling | 1 mini / 15 micros | 3 / 30 | 6 / 60 |
| Funded: locked buffer / max per payout | 1500 / 600 | 2000 / 1000 | 3000 / 2000 |
| Live after | 4 payouts; live amount 1250.00 | 4 payouts; 2000.00 | 4 payouts; 3500.00 |

Sources: F-classic-rules, F-classic-payout, F-classic-scaling. Minis and micros are interchangeable 1:10 ("Minis and micros are interchangeable at the stated ratio"). Quote for the target and consistency:
> "Your best trading day must not represent more than 50% of your total profits when you reach the profit target."
> (F-classic-rules / F-consistency)

Drawdown type quote:
> "Classic trails on an end-of-day basis, and like the challenges, the drawdown stops trailing when it reaches the starting balance."
> (F-classic-payout)

### 3.2 Daily (TFeed only)

| Field | Seed 10K (low priority) | Starter 25K | Growth 50K |
|---|---|---|---|
| Profit target (evaluation) | 600.00 | 1500.00 | 3000.00 |
| Maximum drawdown | 600.00 | 1500.00 | 2000.00 |
| Drawdown type | end-of-day trailing, evaluation and funded | same | same |
| Daily loss limit | none | none | none |
| Consistency | evaluation 50%, funded none | same | same |
| Min trading days (evaluation) | 2 | 2 | 2 |
| Max position (alternatives, not a total) | 6 micros (no minis) | 1 mini or 15 micros | 3 minis or 30 micros |
| Scaling | none | none | none |
| Payout | daily, cap 200 | daily, cap 500 | daily, cap 1000 (aggregate cap 1000 per day per trader) |
| Locked buffer | 600 | 1500 | 2000 |

Sources: F-daily-rules, F-daily-payout, F-daily-scaling.
> "Your maximum drawdown is an end of day trailing drawdown. It follows your balance upwards at the close of each trading day, never intraday, and it stops trailing once your account has cleared its locked buffer."
> (F-daily-rules)

### 3.3 Spark (TFeed only)

| Field | Seed 10K (low priority) | Starter 25K |
|---|---|---|
| Profit target (evaluation) | 600.00 | 1500.00 |
| Maximum drawdown | 400.00 | 1000.00 |
| Drawdown type | evaluation: end-of-day trailing; funded: real-time (intraday) trailing; stops at starting balance | same |
| Daily loss limit | none | none |
| Consistency | evaluation none; funded 30% | same |
| Min trading days (evaluation) | 1 | 1 |
| Max position (alternatives) | 1 mini or 10 micros | 2 minis or 20 micros |
| Scaling | none | none |
| Payout | bi-weekly, max 300, min 30 | bi-weekly, max 400, min 75 |
| Live after / amount | 4 payouts / 400 | 4 payouts / 1000 |

Sources: F-spark-rules, F-spark-payout, F-spark-scaling.
> "During the challenge the drawdown trails on an end-of-day basis. Once the account is funded it trails in real time. In both stages it stops trailing when it reaches the starting balance."
> (F-spark-payout)

Spark Growth ($50K) is closed to new purchases ("Spark Growth is closed to new purchases", F-spark-payout). The evaluation is closed after 90 days without a trade.

### 3.4 Merit (single size 2K, low priority)

30 calendar days, no profit target, no minimum trading days, daily drawdown 25% Dynamic (Soft) during the challenge (none when funded), maximum drawdown listed as the full account balance (table cells in the article), one attempt (cannot be reset), decided by a risk review.
> "Merit is not built like our other accounts. It has one rule: 30 calendar days. No profit target, no minimum days."
> (F-merit-rules)
> "$2,000. Merit comes in a single size, and an approved Merit challenge is funded with $2,000 on the live account."
> (F-merit-live)
> "Your maximum end of day drawdown on the live account is identical to the one you traded during the challenge."
> (F-merit-live)
Live Merit: no daily drawdown, no buffer, no withdrawal limits, no consistency rule, position size listed as "Same as the challenge" (table cell). See the conflict with the marketing page in section 10.

### 3.5 Pre-funded to live rules (all lines)
> "The drawdown becomes static, the scaling changes, and every rule except overnight and over-week positions is waived."
> (F-live-rules)
> "Live scales with your results: one extra mini for every $2,000 of additional profit made on the Live account."
> (F-live-rules / F-broker)

### 3.6 Per-plan-per-size YAML

The blocks are in section 9.

---

## 4. Trading-day reset

**Not published.** No article or terms page found states a daily trading-day reset time or time zone. What is published: end-of-day drawdown "is recalculated once a day, after the close"; positions "must be closed before the session ends"; "Check the closing time of each product you trade, because they are not all the same"; Tier 1 news times are stated in "US Central, the exchange's own zone"; CME price limits are published around 4:05 PM CT. Holiday rule: only for the Daily payout clock ("Only trading days are counted, weekends and market holidays do not move the clock"). No holiday trading-hours article found.

## 5. Trade copying stance

> "Yes, on every Phoenix account and at every stage: Classic, Daily, Spark and Merit, challenge and funded alike."
> "Running the same direction across several of your own accounts is fine. That is what copy trading is for and it is why we build a copier."
> "What is prohibited is hedging: opposing positions on the same asset across two accounts, so the pair is flat whichever way price goes."
> (F-copy, https://phoenixtraderfunding.com/faq/detail/risk-management/is-copy-trading-allowed)

Same-person copying across own accounts: allowed. Copier: Phoenix's own, "Thor is our own trade copier. It is what powers TFeed, and it is included with every TFeed purchase rather than sold separately." (F-copy). Account-count limits: challenges unlimited; pre-funded 3 per trader on dxFeed, 5 per trader on TFeed (per-line sub-limits: Spark 3, Daily 3, Classic Growth/Scale 3, Classic Starter 5); live funded theoretical maximum 5, discretionary; other funded accounts go dormant when one transfers to live:
> "Unlimited challenges. Pre-funded accounts are capped by data feed. Live funded accounts are capped and discretionary."
> (F-max-accounts)

Copying with other firms: not prohibited as copying. Cross-firm limits found: hedging "yours or anyone else's" is prohibited ("No. Opening opposing positions on the same asset across two accounts is prohibited, on funded accounts and during challenges alike." F-hedging), and:
> "This covers your other Phoenix accounts and accounts at other firms."
> (F-prohibited / F-classic-rules, about martingale or loss recovery across accounts, applied from 15 September 2026)

Terms of Use, section 7: "hedging or offsetting positions across several accounts, including accounts held by other people;" (T-terms).

## 6. Automated trading stance

**(i) Classification: forbidden** on evaluation and pre-funded accounts (conditional/ambiguous on Live Funded).

`automation_rule_text` (verbatim):
> "HFT and EA bots. Automated systems that trade without you. Scalp as fast as you like, what we cannot fund is software trading in your place."
> (F-prohibited, https://phoenixtraderfunding.com/faq/detail/most-asked-questions/are-there-any-forbidden-or-prohibited-strategies; the same sentence appears in F-payout-rules as "No HFT or EA bots. Automated systems that trade without you.")

Additional official text:
> "Automated scalping is prohibited. HFT systems and EA bots that place trades without you are not permitted, and using one fails the challenge."
> (F-scalping, https://phoenixtraderfunding.com/faq/detail/most-asked-questions/do-you-allow-scalping)
> "The distinction is not speed, it is who is deciding."
> "Tools that assist you are fine. Hotkeys, a DOM, a ladder, alerts, anything that makes you faster at your own decisions. The rule is about execution without a human, not about software."
> (F-scalping)

Live Funded:
> "On a Live Funded account, every rule except overnight and over-week is waived. Those two stay."
> (F-prohibited). The "Prohibited everywhere" list, which includes HFT and EA bots, is introduced with "These apply during the challenge and on funded accounts", and the live waiver sentence above waives every rule except overnight and over-week. The Terms of Use section 7 prohibited-practices list does not name automation. So whether automation is allowed on a Live Funded account is not explicit; treated as `conditional` and flagged in section 10.

**(ii) Named mechanisms**

| Mechanism | Stance |
|---|---|
| Semi-automated / trade management tools | Mentioned (indirectly): "Tools that assist you are fine. Hotkeys, a DOM, a ladder, alerts, anything that makes you faster at your own decisions. The rule is about execution without a human, not about software." (F-scalping). No explicit text on semi-automated strategies; "execution without a human" is the line. |
| Trade copiers | Mentioned (quote): "Running the same direction across several of your own accounts is fine. That is what copy trading is for and it is why we build a copier." (F-copy). The copier is Phoenix's own (Thor). |
| Webhooks / alerts | Mentioned (quote): "TradingView connects through Tradovate and webhooks rather than directly." and "TradingView has two routes, and both work. Connect it through Tradovate, or use Thor's own native TradingView integration with webhooks." (F-copy, F-platforms). Alerts as a helping tool: see the "Tools that assist you" quote. A webhook that places orders without a human decision falls under "execution without a human"; the firm does not address this combination. |
| ATM / auto brackets | Not mentioned as a rule. Odin's own bracket orders are described as platform features (stop-loss and take-profit held server-side). |

**(iii) Differs by phase:** the prohibition list is headed "Prohibited everywhere" and introduced with "These apply during the challenge and on funded accounts", and on Live Funded "every rule except overnight and over-week is waived". The bot prohibition is also in the cross-line list of F-payout-rules ("Trading rules that apply to every line"). Consequence: "using one fails the challenge" (F-scalping). For funded accounts the consequence for bots is not stated separately (general text: "On a funded account it can mean a payout denial, removal of the associated profits, or closure").

---

## 7. Device / hosting rules and other rules

### Device / hosting

> "Yes, you may trade through a VPN, VPS or proxy. Connections are monitored, and that is worth understanding before you rely on one."
> "All sessions are logged and reviewed, so that a single trader stays a single identifiable trader. That is the entire purpose."
> "Deliberately hiding or changing your identity results in account termination."
> (F-vpn-vps, https://phoenixtraderfunding.com/faq/detail/most-asked-questions/do-you-allow-vpn-vps)

Terms of Use: "Your account is personal and cannot be transferred, sold or shared." and "letting another person trade, manage or access your account, or trading on behalf of someone else;" (T-terms). R4 (2026-10-23): `personal_device_required: no`. The same FAQ (F-vpn-vps) explicitly permits remote and proxied connections: "There is no rule against connecting through a VPN, a VPS or a proxy, and plenty of traders have good reasons to: a VPS for uptime, a VPN on public networks, a proxy at work." No text requires a personal device or own IP; the only identity rule is "One trader, one identity, is the principle it protects." Sharing the account with another person remains prohibited (T-terms).

### Other rules

| Rule | Published value | Source |
|---|---|---|
| News | Allowed on every account and tier, hold through and open into Tier 1: "Yes. News trading is allowed on every account and every tier, Tier 1 events included. You may hold through them and open into them." | F-news |
| Mandatory stop | Not found | none |
| Overnight / weekend | Prohibited on all lines and phases including Live: "No. Overnight and over-week positions are prohibited on every account we offer: Classic, Daily, Spark and Merit, at every stage." | F-overnight |
| Hedging | Prohibited across two accounts: "No. Opening opposing positions on the same asset across two accounts is prohibited, on funded accounts and during challenges alike." | F-hedging |
| Scaling | Classic only (3.1); none on Daily and Spark; Live: one extra mini for every $2,000 of additional profit | F-*-scaling, F-live-rules |
| Max accounts | See section 5 | F-max-accounts |
| Inactivity | Pre-funded: "Place at least one trade every 7 calendar days. This is the same on dxFeed and on TFeed."; Live: 30 calendar days "not applied mechanically"; challenges none except Spark: "A Spark challenge is closed after 90 days without a trade." | F-inactivity |
| Min / max trade duration | No minimum; scalping allowed | F-scalping |
| Funded-only | "Flipping. Token trades placed only to validate a trading day rather than to trade. A day identified as flipping is not counted." and "One tick trading on bonds such as ZB, ZN or ZS." (prohibited on funded accounts) | F-prohibited |
| Maximum time for an evaluation | Merit 30 calendar days; others none stated here | F-merit-rules |
| 2% price-limit rule | "Stop trading a contract once it comes within 2 percent of its daily price limit." | Help Center article "What is the 2% price limit rule?" (https://phoenixtraderfunding.com/faq/detail/risk-management/what-is-the-2-price-limit-rule) |

---

## 8. Commissions

> "Each cell shows one side first, then the round turn in lighter type."
> "Commissions are charged per contract per side, so a 3 lot round turn on ES costs three times the round turn figure above."
> "TFeed is half the dxFeed rate on every single contract, and the same rates apply to challenges and funded accounts alike."
> "They come off your account balance as you trade, which means they count against your drawdown exactly like a loss does."
> (F-commissions, https://phoenixtraderfunding.com/faq/detail/risk-management/commissionsfees)

| Product | dxFeed (one side / round turn) | TFeed (one side / round turn) |
|---|---|---|
| MNQ (Micro E-mini NASDAQ-100) | 0.71 / 1.42 | 0.35 / 0.71 |
| NQ (E-mini NASDAQ-100) | 2.18 / 4.36 | 1.09 / 2.18 |

USD, per contract; commissions are charged per contract per side and "come off your account balance as you trade", counting against the drawdown. The page does not say whether exchange, NFA or clearing fees are included in these figures (gap). The same rates apply in challenge and funded. Merit pays "The same as every other Phoenix account" (F-merit-payout/merit commissions article). The live account's commission schedule is not published separately.

---

## 9. Machine-readable blocks

#### Classic — evaluation — 25K (Starter)

```yaml
firm: phoenix-trader-funding
plan: "Classic Starter"
phase: evaluation
size_label: "25K"
size_usd: "25000.00"
instruments: [MNQ, NQ]
platforms: ["Odin (Phoenix web platform, direct on TFeed)", "Quantower (direct on TFeed, needs addon)", "NinjaTrader 8 via Tradovate connection with TFeed credentials", "NinjaTrader via Thor addon (Sim101 copied)", "Tradovate", "TradingView via Tradovate or Thor webhooks", "Rithmic-leader platforms via Thor (Sierra Chart, MultiCharts, Bookmap, etc.)", "dxFeed feed: Odin", "dxFeed feed: Atas", "dxFeed feed: Quantower", "dxFeed feed: DeepChart", "dxFeed feed: DeepDom"]
drawdown: { amount_usd: "1500.00", type: trailing_eod, stops_trailing_at: "Locks at the starting balance (static from then on). Source: \"the drawdown stops trailing when it reaches the starting balance.\"" }
daily_loss_limit: { amount_usd: null, kind: none }
profit_target_usd: "1500.00"
consistency_rule: { text: "Best trading day must not exceed 50% of total profit when the target is reached (challenge only)", threshold_pct: "50" }
max_position: { minis: "1", micros: "10", scaling: "1 mini/10 micros below $1,000 profit; 1/15 at $1,000-$2,000 (Starter tops out)" }
min_trading_days: "2"
trading_day_reset: { time: null, tz_as_published: null }
commission_per_contract: { mnq_usd: "0.35", nq_usd: "1.09", basis: per_side, includes: "TFeed rates, per contract per side as published (round turn: MNQ 0.71, NQ 2.18). dxFeed rates (Classic only): MNQ 0.71, NQ 2.18 per side (round turn 1.42 / 4.36). Inclusion of exchange/NFA/clearing fees is not stated." }
copy_trading_allowed: yes
automated_trading_allowed: no
automation_rule_text: "HFT and EA bots. Automated systems that trade without you. Scalp as fast as you like, what we cannot fund is software trading in your place."
vps_allowed: yes
personal_device_required: no
status: partial
not_confirmed_fields: ["trading_day_reset", "commission_per_contract.includes"]
notes: "Eval profit target and drawdown from the Classic rules article. Drawdown is end-of-day trailing per the Classic payout article. Starting size is the lowest scaling band; the ceiling (1 minis / 15 micros) is the top band. Classic is offered on both TFeed and dxFeed with different commission rates. No daily loss limit."
sources:
  - { field: profit_target_usd, url: "https://phoenixtraderfunding.com/faq/detail/classic-accounts/what-are-the-rules-of-the-classic-accounts", accessed: "2026-10-23" }
  - { field: drawdown, url: "https://phoenixtraderfunding.com/faq/detail/classic-accounts/classic-accounts-payout-rules", accessed: "2026-10-23" }
  - { field: drawdown, url: "https://phoenixtraderfunding.com/faq/detail/classic-accounts/what-are-the-rules-of-the-classic-accounts", accessed: "2026-10-23" }
  - { field: consistency_rule, url: "https://phoenixtraderfunding.com/faq/detail/classic-accounts/what-are-the-rules-of-the-classic-accounts", accessed: "2026-10-23" }
  - { field: max_position, url: "https://phoenixtraderfunding.com/faq/detail/classic-accounts/scaling-plan-classic-accounts", accessed: "2026-10-23" }
  - { field: commission_per_contract, url: "https://phoenixtraderfunding.com/faq/detail/risk-management/commissionsfees", accessed: "2026-10-23" }
  - { field: copy_trading_allowed, url: "https://phoenixtraderfunding.com/faq/detail/risk-management/is-copy-trading-allowed", accessed: "2026-10-23" }
  - { field: automation_rule_text, url: "https://phoenixtraderfunding.com/faq/detail/most-asked-questions/are-there-any-forbidden-or-prohibited-strategies", accessed: "2026-10-23" }
  - { field: vps_allowed, url: "https://phoenixtraderfunding.com/faq/detail/most-asked-questions/do-you-allow-vpn-vps", accessed: "2026-10-23" }
  - { field: personal_device_required, url: "https://phoenixtraderfunding.com/faq/detail/most-asked-questions/do-you-allow-vpn-vps", accessed: "2026-10-23" }
  - { field: platforms, url: "https://phoenixtraderfunding.com/faq/detail/dont-know-where-to-start/on-which-platforms-can-i-trade", accessed: "2026-10-23" }
  - { field: platforms, url: "https://phoenixtraderfunding.com/faq/detail/most-asked-questions/do-you-support-ninjatrader", accessed: "2026-10-23" }
  - { field: packages_cross_check, url: "https://server.phoenixtraderfunding.com/public/api/packages/fetch", accessed: "2026-10-23" }
```

#### Classic — evaluation — 50K (Growth)

```yaml
firm: phoenix-trader-funding
plan: "Classic Growth"
phase: evaluation
size_label: "50K"
size_usd: "50000.00"
instruments: [MNQ, NQ]
platforms: ["Odin (Phoenix web platform, direct on TFeed)", "Quantower (direct on TFeed, needs addon)", "NinjaTrader 8 via Tradovate connection with TFeed credentials", "NinjaTrader via Thor addon (Sim101 copied)", "Tradovate", "TradingView via Tradovate or Thor webhooks", "Rithmic-leader platforms via Thor (Sierra Chart, MultiCharts, Bookmap, etc.)", "dxFeed feed: Odin", "dxFeed feed: Atas", "dxFeed feed: Quantower", "dxFeed feed: DeepChart", "dxFeed feed: DeepDom"]
drawdown: { amount_usd: "2000.00", type: trailing_eod, stops_trailing_at: "Locks at the starting balance (static from then on). Source: \"the drawdown stops trailing when it reaches the starting balance.\"" }
daily_loss_limit: { amount_usd: null, kind: none }
profit_target_usd: "3000.00"
consistency_rule: { text: "Best trading day must not exceed 50% of total profit when the target is reached (challenge only)", threshold_pct: "50" }
max_position: { minis: "1", micros: "10", scaling: "1/10 below $1,000; 2/20 at $1,000-$2,000; 3/30 at $2,000-$4,000 (tops out at $4,000)" }
min_trading_days: "2"
trading_day_reset: { time: null, tz_as_published: null }
commission_per_contract: { mnq_usd: "0.35", nq_usd: "1.09", basis: per_side, includes: "TFeed rates, per contract per side as published (round turn: MNQ 0.71, NQ 2.18). dxFeed rates (Classic only): MNQ 0.71, NQ 2.18 per side (round turn 1.42 / 4.36). Inclusion of exchange/NFA/clearing fees is not stated." }
copy_trading_allowed: yes
automated_trading_allowed: no
automation_rule_text: "HFT and EA bots. Automated systems that trade without you. Scalp as fast as you like, what we cannot fund is software trading in your place."
vps_allowed: yes
personal_device_required: no
status: partial
not_confirmed_fields: ["trading_day_reset", "commission_per_contract.includes"]
notes: "Eval profit target and drawdown from the Classic rules article. Drawdown is end-of-day trailing per the Classic payout article. Starting size is the lowest scaling band; the ceiling (3 minis / 30 micros) is the top band. Classic is offered on both TFeed and dxFeed with different commission rates. No daily loss limit."
sources:
  - { field: profit_target_usd, url: "https://phoenixtraderfunding.com/faq/detail/classic-accounts/what-are-the-rules-of-the-classic-accounts", accessed: "2026-10-23" }
  - { field: drawdown, url: "https://phoenixtraderfunding.com/faq/detail/classic-accounts/classic-accounts-payout-rules", accessed: "2026-10-23" }
  - { field: drawdown, url: "https://phoenixtraderfunding.com/faq/detail/classic-accounts/what-are-the-rules-of-the-classic-accounts", accessed: "2026-10-23" }
  - { field: consistency_rule, url: "https://phoenixtraderfunding.com/faq/detail/classic-accounts/what-are-the-rules-of-the-classic-accounts", accessed: "2026-10-23" }
  - { field: max_position, url: "https://phoenixtraderfunding.com/faq/detail/classic-accounts/scaling-plan-classic-accounts", accessed: "2026-10-23" }
  - { field: commission_per_contract, url: "https://phoenixtraderfunding.com/faq/detail/risk-management/commissionsfees", accessed: "2026-10-23" }
  - { field: copy_trading_allowed, url: "https://phoenixtraderfunding.com/faq/detail/risk-management/is-copy-trading-allowed", accessed: "2026-10-23" }
  - { field: automation_rule_text, url: "https://phoenixtraderfunding.com/faq/detail/most-asked-questions/are-there-any-forbidden-or-prohibited-strategies", accessed: "2026-10-23" }
  - { field: vps_allowed, url: "https://phoenixtraderfunding.com/faq/detail/most-asked-questions/do-you-allow-vpn-vps", accessed: "2026-10-23" }
  - { field: personal_device_required, url: "https://phoenixtraderfunding.com/faq/detail/most-asked-questions/do-you-allow-vpn-vps", accessed: "2026-10-23" }
  - { field: platforms, url: "https://phoenixtraderfunding.com/faq/detail/dont-know-where-to-start/on-which-platforms-can-i-trade", accessed: "2026-10-23" }
  - { field: platforms, url: "https://phoenixtraderfunding.com/faq/detail/most-asked-questions/do-you-support-ninjatrader", accessed: "2026-10-23" }
  - { field: packages_cross_check, url: "https://server.phoenixtraderfunding.com/public/api/packages/fetch", accessed: "2026-10-23" }
```

#### Classic — evaluation — 100K (Scale)

```yaml
firm: phoenix-trader-funding
plan: "Classic Scale"
phase: evaluation
size_label: "100K"
size_usd: "100000.00"
instruments: [MNQ, NQ]
platforms: ["Odin (Phoenix web platform, direct on TFeed)", "Quantower (direct on TFeed, needs addon)", "NinjaTrader 8 via Tradovate connection with TFeed credentials", "NinjaTrader via Thor addon (Sim101 copied)", "Tradovate", "TradingView via Tradovate or Thor webhooks", "Rithmic-leader platforms via Thor (Sierra Chart, MultiCharts, Bookmap, etc.)", "dxFeed feed: Odin", "dxFeed feed: Atas", "dxFeed feed: Quantower", "dxFeed feed: DeepChart", "dxFeed feed: DeepDom"]
drawdown: { amount_usd: "3000.00", type: trailing_eod, stops_trailing_at: "Locks at the starting balance (static from then on). Source: \"the drawdown stops trailing when it reaches the starting balance.\"" }
daily_loss_limit: { amount_usd: null, kind: none }
profit_target_usd: "6000.00"
consistency_rule: { text: "Best trading day must not exceed 50% of total profit when the target is reached (challenge only)", threshold_pct: "50" }
max_position: { minis: "2", micros: "20", scaling: "2/20 below $1,000; 3/30 at $1,000-$2,000; 4/40 at $2,000-$4,000; 6/60 at $4,000-$6,000 (tops out at $6,000)" }
min_trading_days: "2"
trading_day_reset: { time: null, tz_as_published: null }
commission_per_contract: { mnq_usd: "0.35", nq_usd: "1.09", basis: per_side, includes: "TFeed rates, per contract per side as published (round turn: MNQ 0.71, NQ 2.18). dxFeed rates (Classic only): MNQ 0.71, NQ 2.18 per side (round turn 1.42 / 4.36). Inclusion of exchange/NFA/clearing fees is not stated." }
copy_trading_allowed: yes
automated_trading_allowed: no
automation_rule_text: "HFT and EA bots. Automated systems that trade without you. Scalp as fast as you like, what we cannot fund is software trading in your place."
vps_allowed: yes
personal_device_required: no
status: partial
not_confirmed_fields: ["trading_day_reset", "commission_per_contract.includes"]
notes: "Eval profit target and drawdown from the Classic rules article. Drawdown is end-of-day trailing per the Classic payout article. Starting size is the lowest scaling band; the ceiling (6 minis / 60 micros) is the top band. Classic is offered on both TFeed and dxFeed with different commission rates. No daily loss limit."
sources:
  - { field: profit_target_usd, url: "https://phoenixtraderfunding.com/faq/detail/classic-accounts/what-are-the-rules-of-the-classic-accounts", accessed: "2026-10-23" }
  - { field: drawdown, url: "https://phoenixtraderfunding.com/faq/detail/classic-accounts/classic-accounts-payout-rules", accessed: "2026-10-23" }
  - { field: drawdown, url: "https://phoenixtraderfunding.com/faq/detail/classic-accounts/what-are-the-rules-of-the-classic-accounts", accessed: "2026-10-23" }
  - { field: consistency_rule, url: "https://phoenixtraderfunding.com/faq/detail/classic-accounts/what-are-the-rules-of-the-classic-accounts", accessed: "2026-10-23" }
  - { field: max_position, url: "https://phoenixtraderfunding.com/faq/detail/classic-accounts/scaling-plan-classic-accounts", accessed: "2026-10-23" }
  - { field: commission_per_contract, url: "https://phoenixtraderfunding.com/faq/detail/risk-management/commissionsfees", accessed: "2026-10-23" }
  - { field: copy_trading_allowed, url: "https://phoenixtraderfunding.com/faq/detail/risk-management/is-copy-trading-allowed", accessed: "2026-10-23" }
  - { field: automation_rule_text, url: "https://phoenixtraderfunding.com/faq/detail/most-asked-questions/are-there-any-forbidden-or-prohibited-strategies", accessed: "2026-10-23" }
  - { field: vps_allowed, url: "https://phoenixtraderfunding.com/faq/detail/most-asked-questions/do-you-allow-vpn-vps", accessed: "2026-10-23" }
  - { field: personal_device_required, url: "https://phoenixtraderfunding.com/faq/detail/most-asked-questions/do-you-allow-vpn-vps", accessed: "2026-10-23" }
  - { field: platforms, url: "https://phoenixtraderfunding.com/faq/detail/dont-know-where-to-start/on-which-platforms-can-i-trade", accessed: "2026-10-23" }
  - { field: platforms, url: "https://phoenixtraderfunding.com/faq/detail/most-asked-questions/do-you-support-ninjatrader", accessed: "2026-10-23" }
  - { field: packages_cross_check, url: "https://server.phoenixtraderfunding.com/public/api/packages/fetch", accessed: "2026-10-23" }
```

#### Classic — funded (pre-funded) — 25K (Starter)

```yaml
firm: phoenix-trader-funding
plan: "Classic Starter"
phase: funded
size_label: "25K"
size_usd: "25000.00"
instruments: [MNQ, NQ]
platforms: ["Odin (Phoenix web platform, direct on TFeed)", "Quantower (direct on TFeed, needs addon)", "NinjaTrader 8 via Tradovate connection with TFeed credentials", "NinjaTrader via Thor addon (Sim101 copied)", "Tradovate", "TradingView via Tradovate or Thor webhooks", "Rithmic-leader platforms via Thor (Sierra Chart, MultiCharts, Bookmap, etc.)", "dxFeed feed: Odin", "dxFeed feed: Atas", "dxFeed feed: Quantower", "dxFeed feed: DeepChart", "dxFeed feed: DeepDom"]
drawdown: { amount_usd: "1500.00", type: trailing_eod, stops_trailing_at: "Stops trailing at the starting balance; at first withdrawal the limit is set to the starting balance if not already reached, and stays static" }
daily_loss_limit: { amount_usd: null, kind: none }
profit_target_usd: null
consistency_rule: { text: "None on the funded account", threshold_pct: null }
max_position: { minis: "1", micros: "10", scaling: "1 mini/10 micros below $1,000 profit; 1/15 at $1,000-$2,000 (Starter tops out)" }
min_trading_days: null
trading_day_reset: { time: null, tz_as_published: null }
commission_per_contract: { mnq_usd: "0.35", nq_usd: "1.09", basis: per_side, includes: "TFeed rates, per contract per side as published (round turn: MNQ 0.71, NQ 2.18). dxFeed rates (Classic only): MNQ 0.71, NQ 2.18 per side (round turn 1.42 / 4.36). Inclusion of exchange/NFA/clearing fees is not stated." }
copy_trading_allowed: yes
automated_trading_allowed: no
automation_rule_text: "HFT and EA bots. Automated systems that trade without you. Scalp as fast as you like, what we cannot fund is software trading in your place."
vps_allowed: yes
personal_device_required: no
status: partial
not_confirmed_fields: ["trading_day_reset", "commission_per_contract.includes", "min_trading_days"]
notes: "Pre-funded Classic. Pays weekly, 90% split, first payout after 5 trading days, locked buffer 1500.00, max per payout ($600). Live funding after 4 payouts. Drawdown amount is the package value; funded article restates it only as the locked buffer."
sources:
  - { field: drawdown, url: "https://phoenixtraderfunding.com/faq/detail/classic-accounts/classic-accounts-payout-rules", accessed: "2026-10-23" }
  - { field: consistency_rule, url: "https://phoenixtraderfunding.com/faq/detail/classic-accounts/what-are-the-rules-of-the-classic-accounts", accessed: "2026-10-23" }
  - { field: max_position, url: "https://phoenixtraderfunding.com/faq/detail/classic-accounts/scaling-plan-classic-accounts", accessed: "2026-10-23" }
  - { field: automation_rule_text, url: "https://phoenixtraderfunding.com/faq/detail/most-asked-questions/are-there-any-forbidden-or-prohibited-strategies", accessed: "2026-10-23" }
  - { field: copy_trading_allowed, url: "https://phoenixtraderfunding.com/faq/detail/risk-management/is-copy-trading-allowed", accessed: "2026-10-23" }
  - { field: vps_allowed, url: "https://phoenixtraderfunding.com/faq/detail/most-asked-questions/do-you-allow-vpn-vps", accessed: "2026-10-23" }
  - { field: personal_device_required, url: "https://phoenixtraderfunding.com/faq/detail/most-asked-questions/do-you-allow-vpn-vps", accessed: "2026-10-23" }
```

#### Classic — funded (pre-funded) — 50K (Growth)

```yaml
firm: phoenix-trader-funding
plan: "Classic Growth"
phase: funded
size_label: "50K"
size_usd: "50000.00"
instruments: [MNQ, NQ]
platforms: ["Odin (Phoenix web platform, direct on TFeed)", "Quantower (direct on TFeed, needs addon)", "NinjaTrader 8 via Tradovate connection with TFeed credentials", "NinjaTrader via Thor addon (Sim101 copied)", "Tradovate", "TradingView via Tradovate or Thor webhooks", "Rithmic-leader platforms via Thor (Sierra Chart, MultiCharts, Bookmap, etc.)", "dxFeed feed: Odin", "dxFeed feed: Atas", "dxFeed feed: Quantower", "dxFeed feed: DeepChart", "dxFeed feed: DeepDom"]
drawdown: { amount_usd: "2000.00", type: trailing_eod, stops_trailing_at: "Stops trailing at the starting balance; at first withdrawal the limit is set to the starting balance if not already reached, and stays static" }
daily_loss_limit: { amount_usd: null, kind: none }
profit_target_usd: null
consistency_rule: { text: "None on the funded account", threshold_pct: null }
max_position: { minis: "1", micros: "10", scaling: "1/10 below $1,000; 2/20 at $1,000-$2,000; 3/30 at $2,000-$4,000 (tops out at $4,000)" }
min_trading_days: null
trading_day_reset: { time: null, tz_as_published: null }
commission_per_contract: { mnq_usd: "0.35", nq_usd: "1.09", basis: per_side, includes: "TFeed rates, per contract per side as published (round turn: MNQ 0.71, NQ 2.18). dxFeed rates (Classic only): MNQ 0.71, NQ 2.18 per side (round turn 1.42 / 4.36). Inclusion of exchange/NFA/clearing fees is not stated." }
copy_trading_allowed: yes
automated_trading_allowed: no
automation_rule_text: "HFT and EA bots. Automated systems that trade without you. Scalp as fast as you like, what we cannot fund is software trading in your place."
vps_allowed: yes
personal_device_required: no
status: partial
not_confirmed_fields: ["trading_day_reset", "commission_per_contract.includes", "min_trading_days"]
notes: "Pre-funded Classic. Pays weekly, 90% split, first payout after 5 trading days, locked buffer 2000.00, max per payout ($1,000). Live funding after 4 payouts. Drawdown amount is the package value; funded article restates it only as the locked buffer."
sources:
  - { field: drawdown, url: "https://phoenixtraderfunding.com/faq/detail/classic-accounts/classic-accounts-payout-rules", accessed: "2026-10-23" }
  - { field: consistency_rule, url: "https://phoenixtraderfunding.com/faq/detail/classic-accounts/what-are-the-rules-of-the-classic-accounts", accessed: "2026-10-23" }
  - { field: max_position, url: "https://phoenixtraderfunding.com/faq/detail/classic-accounts/scaling-plan-classic-accounts", accessed: "2026-10-23" }
  - { field: automation_rule_text, url: "https://phoenixtraderfunding.com/faq/detail/most-asked-questions/are-there-any-forbidden-or-prohibited-strategies", accessed: "2026-10-23" }
  - { field: copy_trading_allowed, url: "https://phoenixtraderfunding.com/faq/detail/risk-management/is-copy-trading-allowed", accessed: "2026-10-23" }
  - { field: vps_allowed, url: "https://phoenixtraderfunding.com/faq/detail/most-asked-questions/do-you-allow-vpn-vps", accessed: "2026-10-23" }
  - { field: personal_device_required, url: "https://phoenixtraderfunding.com/faq/detail/most-asked-questions/do-you-allow-vpn-vps", accessed: "2026-10-23" }
```

#### Classic — funded (pre-funded) — 100K (Scale)

```yaml
firm: phoenix-trader-funding
plan: "Classic Scale"
phase: funded
size_label: "100K"
size_usd: "100000.00"
instruments: [MNQ, NQ]
platforms: ["Odin (Phoenix web platform, direct on TFeed)", "Quantower (direct on TFeed, needs addon)", "NinjaTrader 8 via Tradovate connection with TFeed credentials", "NinjaTrader via Thor addon (Sim101 copied)", "Tradovate", "TradingView via Tradovate or Thor webhooks", "Rithmic-leader platforms via Thor (Sierra Chart, MultiCharts, Bookmap, etc.)", "dxFeed feed: Odin", "dxFeed feed: Atas", "dxFeed feed: Quantower", "dxFeed feed: DeepChart", "dxFeed feed: DeepDom"]
drawdown: { amount_usd: "3000.00", type: trailing_eod, stops_trailing_at: "Stops trailing at the starting balance; at first withdrawal the limit is set to the starting balance if not already reached, and stays static" }
daily_loss_limit: { amount_usd: null, kind: none }
profit_target_usd: null
consistency_rule: { text: "None on the funded account", threshold_pct: null }
max_position: { minis: "2", micros: "20", scaling: "2/20 below $1,000; 3/30 at $1,000-$2,000; 4/40 at $2,000-$4,000; 6/60 at $4,000-$6,000 (tops out at $6,000)" }
min_trading_days: null
trading_day_reset: { time: null, tz_as_published: null }
commission_per_contract: { mnq_usd: "0.35", nq_usd: "1.09", basis: per_side, includes: "TFeed rates, per contract per side as published (round turn: MNQ 0.71, NQ 2.18). dxFeed rates (Classic only): MNQ 0.71, NQ 2.18 per side (round turn 1.42 / 4.36). Inclusion of exchange/NFA/clearing fees is not stated." }
copy_trading_allowed: yes
automated_trading_allowed: no
automation_rule_text: "HFT and EA bots. Automated systems that trade without you. Scalp as fast as you like, what we cannot fund is software trading in your place."
vps_allowed: yes
personal_device_required: no
status: partial
not_confirmed_fields: ["trading_day_reset", "commission_per_contract.includes", "min_trading_days"]
notes: "Pre-funded Classic. Pays weekly, 90% split, first payout after 5 trading days, locked buffer 3000.00, max per payout ($2,000). Live funding after 4 payouts. Drawdown amount is the package value; funded article restates it only as the locked buffer."
sources:
  - { field: drawdown, url: "https://phoenixtraderfunding.com/faq/detail/classic-accounts/classic-accounts-payout-rules", accessed: "2026-10-23" }
  - { field: consistency_rule, url: "https://phoenixtraderfunding.com/faq/detail/classic-accounts/what-are-the-rules-of-the-classic-accounts", accessed: "2026-10-23" }
  - { field: max_position, url: "https://phoenixtraderfunding.com/faq/detail/classic-accounts/scaling-plan-classic-accounts", accessed: "2026-10-23" }
  - { field: automation_rule_text, url: "https://phoenixtraderfunding.com/faq/detail/most-asked-questions/are-there-any-forbidden-or-prohibited-strategies", accessed: "2026-10-23" }
  - { field: copy_trading_allowed, url: "https://phoenixtraderfunding.com/faq/detail/risk-management/is-copy-trading-allowed", accessed: "2026-10-23" }
  - { field: vps_allowed, url: "https://phoenixtraderfunding.com/faq/detail/most-asked-questions/do-you-allow-vpn-vps", accessed: "2026-10-23" }
  - { field: personal_device_required, url: "https://phoenixtraderfunding.com/faq/detail/most-asked-questions/do-you-allow-vpn-vps", accessed: "2026-10-23" }
```

#### Classic — live — 25K (Starter)

```yaml
firm: phoenix-trader-funding
plan: "Classic Starter"
phase: live
size_label: "25K"
size_usd: "25000.00"
instruments: [MNQ, NQ]
platforms: ["Rithmic EdgeClear connection (any Rithmic-capable platform)"]
drawdown: { amount_usd: null, type: static, stops_trailing_at: "Static from the start on the live account" }
daily_loss_limit: { amount_usd: null, kind: none }
profit_target_usd: null
consistency_rule: { text: null, threshold_pct: null }
max_position: { minis: null, micros: null, scaling: "Live scaling: one extra mini for every $2,000 of additional profit on the Live account (starting size not published)" }
min_trading_days: null
trading_day_reset: { time: null, tz_as_published: null }
commission_per_contract: { mnq_usd: null, nq_usd: null, basis: null, includes: "Live commission schedule not published separately; FAQ says the same rates apply to challenges and funded accounts alike (TFeed/dxFeed table)." }
copy_trading_allowed: yes
automated_trading_allowed: conditional
automation_rule_text: "On a Live Funded account, every rule except overnight and over-week is waived. Those two stay. HFT and EA bots. Automated systems that trade without you. Scalp as fast as you like, what we cannot fund is software trading in your place."
vps_allowed: yes
personal_device_required: no
status: partial
not_confirmed_fields: ["trading_day_reset", "drawdown.amount_usd", "max_position", "automated_trading_allowed", "commission_per_contract"]
notes: "Live funded account with EdgeClear (Rithmic). Live funding amount 1250.00. CME professional market data fees apply and are deducted from the account. Only the overnight/over-week rule survives; the automation prohibition is listed as prohibited on challenges and funded accounts and the live waiver text says every rule except overnight/over-week is waived (ambiguous, see section 10)."
sources:
  - { field: drawdown, url: "https://phoenixtraderfunding.com/faq/detail/classic-accounts/classic-accounts-payout-rules", accessed: "2026-10-23" }
  - { field: max_position, url: "https://phoenixtraderfunding.com/faq/detail/most-asked-questions/what-are-the-rules-for-being-live-funded", accessed: "2026-10-23" }
  - { field: automation_rule_text, url: "https://phoenixtraderfunding.com/faq/detail/most-asked-questions/are-there-any-forbidden-or-prohibited-strategies", accessed: "2026-10-23" }
  - { field: platforms, url: "https://phoenixtraderfunding.com/faq/detail/most-asked-questions/which-broker-will-host-your-live-account", accessed: "2026-10-23" }
```

#### Classic — live — 50K (Growth)

```yaml
firm: phoenix-trader-funding
plan: "Classic Growth"
phase: live
size_label: "50K"
size_usd: "50000.00"
instruments: [MNQ, NQ]
platforms: ["Rithmic EdgeClear connection (any Rithmic-capable platform)"]
drawdown: { amount_usd: null, type: static, stops_trailing_at: "Static from the start on the live account" }
daily_loss_limit: { amount_usd: null, kind: none }
profit_target_usd: null
consistency_rule: { text: null, threshold_pct: null }
max_position: { minis: null, micros: null, scaling: "Live scaling: one extra mini for every $2,000 of additional profit on the Live account (starting size not published)" }
min_trading_days: null
trading_day_reset: { time: null, tz_as_published: null }
commission_per_contract: { mnq_usd: null, nq_usd: null, basis: null, includes: "Live commission schedule not published separately; FAQ says the same rates apply to challenges and funded accounts alike (TFeed/dxFeed table)." }
copy_trading_allowed: yes
automated_trading_allowed: conditional
automation_rule_text: "On a Live Funded account, every rule except overnight and over-week is waived. Those two stay. HFT and EA bots. Automated systems that trade without you. Scalp as fast as you like, what we cannot fund is software trading in your place."
vps_allowed: yes
personal_device_required: no
status: partial
not_confirmed_fields: ["trading_day_reset", "drawdown.amount_usd", "max_position", "automated_trading_allowed", "commission_per_contract"]
notes: "Live funded account with EdgeClear (Rithmic). Live funding amount 2000.00. CME professional market data fees apply and are deducted from the account. Only the overnight/over-week rule survives; the automation prohibition is listed as prohibited on challenges and funded accounts and the live waiver text says every rule except overnight/over-week is waived (ambiguous, see section 10)."
sources:
  - { field: drawdown, url: "https://phoenixtraderfunding.com/faq/detail/classic-accounts/classic-accounts-payout-rules", accessed: "2026-10-23" }
  - { field: max_position, url: "https://phoenixtraderfunding.com/faq/detail/most-asked-questions/what-are-the-rules-for-being-live-funded", accessed: "2026-10-23" }
  - { field: automation_rule_text, url: "https://phoenixtraderfunding.com/faq/detail/most-asked-questions/are-there-any-forbidden-or-prohibited-strategies", accessed: "2026-10-23" }
  - { field: platforms, url: "https://phoenixtraderfunding.com/faq/detail/most-asked-questions/which-broker-will-host-your-live-account", accessed: "2026-10-23" }
```

#### Classic — live — 100K (Scale)

```yaml
firm: phoenix-trader-funding
plan: "Classic Scale"
phase: live
size_label: "100K"
size_usd: "100000.00"
instruments: [MNQ, NQ]
platforms: ["Rithmic EdgeClear connection (any Rithmic-capable platform)"]
drawdown: { amount_usd: null, type: static, stops_trailing_at: "Static from the start on the live account" }
daily_loss_limit: { amount_usd: null, kind: none }
profit_target_usd: null
consistency_rule: { text: null, threshold_pct: null }
max_position: { minis: null, micros: null, scaling: "Live scaling: one extra mini for every $2,000 of additional profit on the Live account (starting size not published)" }
min_trading_days: null
trading_day_reset: { time: null, tz_as_published: null }
commission_per_contract: { mnq_usd: null, nq_usd: null, basis: null, includes: "Live commission schedule not published separately; FAQ says the same rates apply to challenges and funded accounts alike (TFeed/dxFeed table)." }
copy_trading_allowed: yes
automated_trading_allowed: conditional
automation_rule_text: "On a Live Funded account, every rule except overnight and over-week is waived. Those two stay. HFT and EA bots. Automated systems that trade without you. Scalp as fast as you like, what we cannot fund is software trading in your place."
vps_allowed: yes
personal_device_required: no
status: partial
not_confirmed_fields: ["trading_day_reset", "drawdown.amount_usd", "max_position", "automated_trading_allowed", "commission_per_contract"]
notes: "Live funded account with EdgeClear (Rithmic). Live funding amount 3500.00. CME professional market data fees apply and are deducted from the account. Only the overnight/over-week rule survives; the automation prohibition is listed as prohibited on challenges and funded accounts and the live waiver text says every rule except overnight/over-week is waived (ambiguous, see section 10)."
sources:
  - { field: drawdown, url: "https://phoenixtraderfunding.com/faq/detail/classic-accounts/classic-accounts-payout-rules", accessed: "2026-10-23" }
  - { field: max_position, url: "https://phoenixtraderfunding.com/faq/detail/most-asked-questions/what-are-the-rules-for-being-live-funded", accessed: "2026-10-23" }
  - { field: automation_rule_text, url: "https://phoenixtraderfunding.com/faq/detail/most-asked-questions/are-there-any-forbidden-or-prohibited-strategies", accessed: "2026-10-23" }
  - { field: platforms, url: "https://phoenixtraderfunding.com/faq/detail/most-asked-questions/which-broker-will-host-your-live-account", accessed: "2026-10-23" }
```

#### Daily — evaluation — 10K (Seed)

```yaml
firm: phoenix-trader-funding
plan: "Daily Seed"
phase: evaluation
size_label: "10K"
size_usd: "10000.00"
instruments: [MNQ, NQ]
platforms: ["Odin (Phoenix web platform, direct on TFeed)", "Quantower (direct on TFeed, needs addon)", "NinjaTrader 8 via Tradovate connection with TFeed credentials", "NinjaTrader via Thor addon (Sim101 copied)", "Tradovate", "TradingView via Tradovate or Thor webhooks", "Rithmic-leader platforms via Thor (Sierra Chart, MultiCharts, Bookmap, etc.)"]
drawdown: { amount_usd: "600.00", type: trailing_eod, stops_trailing_at: "Stops trailing once the account has cleared its locked buffer (= starting balance level per the trailing-drawdown article)" }
daily_loss_limit: { amount_usd: null, kind: none }
profit_target_usd: "600.00"
consistency_rule: { text: "Best trading day must not exceed 50% of total profit when the target is reached (challenge only)", threshold_pct: "50" }
max_position: { minis: null, micros: "6", scaling: "None: no scaling plan, full size from day one" }
min_trading_days: "2"
trading_day_reset: { time: null, tz_as_published: null }
commission_per_contract: { mnq_usd: "0.35", nq_usd: "1.09", basis: per_side, includes: "TFeed rates, per contract per side as published (round turn: MNQ 0.71, NQ 2.18). dxFeed rates (Classic only): MNQ 0.71, NQ 2.18 per side (round turn 1.42 / 4.36). Inclusion of exchange/NFA/clearing fees is not stated." }
copy_trading_allowed: yes
automated_trading_allowed: no
automation_rule_text: "HFT and EA bots. Automated systems that trade without you. Scalp as fast as you like, what we cannot fund is software trading in your place."
vps_allowed: yes
personal_device_required: no
status: partial
not_confirmed_fields: ["trading_day_reset", "commission_per_contract.includes"]
notes: "TFeed only (not available on dxFeed). Max contracts are alternatives, not a total. LOW PRIORITY (size below 25K)."
sources:
  - { field: profit_target_usd, url: "https://phoenixtraderfunding.com/faq/detail/daily-accounts/what-are-the-rules-of-the-daily-accounts", accessed: "2026-10-23" }
  - { field: drawdown, url: "https://phoenixtraderfunding.com/faq/detail/daily-accounts/what-are-the-rules-of-the-daily-accounts", accessed: "2026-10-23" }
  - { field: consistency_rule, url: "https://phoenixtraderfunding.com/faq/detail/daily-accounts/what-are-the-rules-of-the-daily-accounts", accessed: "2026-10-23" }
  - { field: max_position, url: "https://phoenixtraderfunding.com/faq/detail/daily-accounts/scaling-plan-daily-accounts", accessed: "2026-10-23" }
  - { field: commission_per_contract, url: "https://phoenixtraderfunding.com/faq/detail/risk-management/commissionsfees", accessed: "2026-10-23" }
  - { field: copy_trading_allowed, url: "https://phoenixtraderfunding.com/faq/detail/risk-management/is-copy-trading-allowed", accessed: "2026-10-23" }
  - { field: automation_rule_text, url: "https://phoenixtraderfunding.com/faq/detail/most-asked-questions/are-there-any-forbidden-or-prohibited-strategies", accessed: "2026-10-23" }
  - { field: vps_allowed, url: "https://phoenixtraderfunding.com/faq/detail/most-asked-questions/do-you-allow-vpn-vps", accessed: "2026-10-23" }
  - { field: personal_device_required, url: "https://phoenixtraderfunding.com/faq/detail/most-asked-questions/do-you-allow-vpn-vps", accessed: "2026-10-23" }
```

#### Daily — evaluation — 25K (Starter)

```yaml
firm: phoenix-trader-funding
plan: "Daily Starter"
phase: evaluation
size_label: "25K"
size_usd: "25000.00"
instruments: [MNQ, NQ]
platforms: ["Odin (Phoenix web platform, direct on TFeed)", "Quantower (direct on TFeed, needs addon)", "NinjaTrader 8 via Tradovate connection with TFeed credentials", "NinjaTrader via Thor addon (Sim101 copied)", "Tradovate", "TradingView via Tradovate or Thor webhooks", "Rithmic-leader platforms via Thor (Sierra Chart, MultiCharts, Bookmap, etc.)"]
drawdown: { amount_usd: "1500.00", type: trailing_eod, stops_trailing_at: "Stops trailing once the account has cleared its locked buffer (= starting balance level per the trailing-drawdown article)" }
daily_loss_limit: { amount_usd: null, kind: none }
profit_target_usd: "1500.00"
consistency_rule: { text: "Best trading day must not exceed 50% of total profit when the target is reached (challenge only)", threshold_pct: "50" }
max_position: { minis: "1", micros: "15", scaling: "None: no scaling plan, full size from day one" }
min_trading_days: "2"
trading_day_reset: { time: null, tz_as_published: null }
commission_per_contract: { mnq_usd: "0.35", nq_usd: "1.09", basis: per_side, includes: "TFeed rates, per contract per side as published (round turn: MNQ 0.71, NQ 2.18). dxFeed rates (Classic only): MNQ 0.71, NQ 2.18 per side (round turn 1.42 / 4.36). Inclusion of exchange/NFA/clearing fees is not stated." }
copy_trading_allowed: yes
automated_trading_allowed: no
automation_rule_text: "HFT and EA bots. Automated systems that trade without you. Scalp as fast as you like, what we cannot fund is software trading in your place."
vps_allowed: yes
personal_device_required: no
status: partial
not_confirmed_fields: ["trading_day_reset", "commission_per_contract.includes"]
notes: "TFeed only (not available on dxFeed). Max contracts are alternatives, not a total."
sources:
  - { field: profit_target_usd, url: "https://phoenixtraderfunding.com/faq/detail/daily-accounts/what-are-the-rules-of-the-daily-accounts", accessed: "2026-10-23" }
  - { field: drawdown, url: "https://phoenixtraderfunding.com/faq/detail/daily-accounts/what-are-the-rules-of-the-daily-accounts", accessed: "2026-10-23" }
  - { field: consistency_rule, url: "https://phoenixtraderfunding.com/faq/detail/daily-accounts/what-are-the-rules-of-the-daily-accounts", accessed: "2026-10-23" }
  - { field: max_position, url: "https://phoenixtraderfunding.com/faq/detail/daily-accounts/scaling-plan-daily-accounts", accessed: "2026-10-23" }
  - { field: commission_per_contract, url: "https://phoenixtraderfunding.com/faq/detail/risk-management/commissionsfees", accessed: "2026-10-23" }
  - { field: copy_trading_allowed, url: "https://phoenixtraderfunding.com/faq/detail/risk-management/is-copy-trading-allowed", accessed: "2026-10-23" }
  - { field: automation_rule_text, url: "https://phoenixtraderfunding.com/faq/detail/most-asked-questions/are-there-any-forbidden-or-prohibited-strategies", accessed: "2026-10-23" }
  - { field: vps_allowed, url: "https://phoenixtraderfunding.com/faq/detail/most-asked-questions/do-you-allow-vpn-vps", accessed: "2026-10-23" }
  - { field: personal_device_required, url: "https://phoenixtraderfunding.com/faq/detail/most-asked-questions/do-you-allow-vpn-vps", accessed: "2026-10-23" }
```

#### Daily — evaluation — 50K (Growth)

```yaml
firm: phoenix-trader-funding
plan: "Daily Growth"
phase: evaluation
size_label: "50K"
size_usd: "50000.00"
instruments: [MNQ, NQ]
platforms: ["Odin (Phoenix web platform, direct on TFeed)", "Quantower (direct on TFeed, needs addon)", "NinjaTrader 8 via Tradovate connection with TFeed credentials", "NinjaTrader via Thor addon (Sim101 copied)", "Tradovate", "TradingView via Tradovate or Thor webhooks", "Rithmic-leader platforms via Thor (Sierra Chart, MultiCharts, Bookmap, etc.)"]
drawdown: { amount_usd: "2000.00", type: trailing_eod, stops_trailing_at: "Stops trailing once the account has cleared its locked buffer (= starting balance level per the trailing-drawdown article)" }
daily_loss_limit: { amount_usd: null, kind: none }
profit_target_usd: "3000.00"
consistency_rule: { text: "Best trading day must not exceed 50% of total profit when the target is reached (challenge only)", threshold_pct: "50" }
max_position: { minis: "3", micros: "30", scaling: "None: no scaling plan, full size from day one" }
min_trading_days: "2"
trading_day_reset: { time: null, tz_as_published: null }
commission_per_contract: { mnq_usd: "0.35", nq_usd: "1.09", basis: per_side, includes: "TFeed rates, per contract per side as published (round turn: MNQ 0.71, NQ 2.18). dxFeed rates (Classic only): MNQ 0.71, NQ 2.18 per side (round turn 1.42 / 4.36). Inclusion of exchange/NFA/clearing fees is not stated." }
copy_trading_allowed: yes
automated_trading_allowed: no
automation_rule_text: "HFT and EA bots. Automated systems that trade without you. Scalp as fast as you like, what we cannot fund is software trading in your place."
vps_allowed: yes
personal_device_required: no
status: partial
not_confirmed_fields: ["trading_day_reset", "commission_per_contract.includes"]
notes: "TFeed only (not available on dxFeed). Max contracts are alternatives, not a total."
sources:
  - { field: profit_target_usd, url: "https://phoenixtraderfunding.com/faq/detail/daily-accounts/what-are-the-rules-of-the-daily-accounts", accessed: "2026-10-23" }
  - { field: drawdown, url: "https://phoenixtraderfunding.com/faq/detail/daily-accounts/what-are-the-rules-of-the-daily-accounts", accessed: "2026-10-23" }
  - { field: consistency_rule, url: "https://phoenixtraderfunding.com/faq/detail/daily-accounts/what-are-the-rules-of-the-daily-accounts", accessed: "2026-10-23" }
  - { field: max_position, url: "https://phoenixtraderfunding.com/faq/detail/daily-accounts/scaling-plan-daily-accounts", accessed: "2026-10-23" }
  - { field: commission_per_contract, url: "https://phoenixtraderfunding.com/faq/detail/risk-management/commissionsfees", accessed: "2026-10-23" }
  - { field: copy_trading_allowed, url: "https://phoenixtraderfunding.com/faq/detail/risk-management/is-copy-trading-allowed", accessed: "2026-10-23" }
  - { field: automation_rule_text, url: "https://phoenixtraderfunding.com/faq/detail/most-asked-questions/are-there-any-forbidden-or-prohibited-strategies", accessed: "2026-10-23" }
  - { field: vps_allowed, url: "https://phoenixtraderfunding.com/faq/detail/most-asked-questions/do-you-allow-vpn-vps", accessed: "2026-10-23" }
  - { field: personal_device_required, url: "https://phoenixtraderfunding.com/faq/detail/most-asked-questions/do-you-allow-vpn-vps", accessed: "2026-10-23" }
```

#### Daily — funded (pre-funded) — 10K (Seed)

```yaml
firm: phoenix-trader-funding
plan: "Daily Seed"
phase: funded
size_label: "10K"
size_usd: "10000.00"
instruments: [MNQ, NQ]
platforms: ["Odin (Phoenix web platform, direct on TFeed)", "Quantower (direct on TFeed, needs addon)", "NinjaTrader 8 via Tradovate connection with TFeed credentials", "NinjaTrader via Thor addon (Sim101 copied)", "Tradovate", "TradingView via Tradovate or Thor webhooks", "Rithmic-leader platforms via Thor (Sierra Chart, MultiCharts, Bookmap, etc.)"]
drawdown: { amount_usd: "600.00", type: trailing_eod, stops_trailing_at: "Stops trailing once the account has cleared its locked buffer ($600)" }
daily_loss_limit: { amount_usd: null, kind: none }
profit_target_usd: null
consistency_rule: { text: "None on the funded account", threshold_pct: null }
max_position: { minis: null, micros: "6", scaling: "None: no scaling plan" }
min_trading_days: null
trading_day_reset: { time: null, tz_as_published: null }
commission_per_contract: { mnq_usd: "0.35", nq_usd: "1.09", basis: per_side, includes: "TFeed rates, per contract per side as published (round turn: MNQ 0.71, NQ 2.18). dxFeed rates (Classic only): MNQ 0.71, NQ 2.18 per side (round turn 1.42 / 4.36). Inclusion of exchange/NFA/clearing fees is not stated." }
copy_trading_allowed: yes
automated_trading_allowed: no
automation_rule_text: "HFT and EA bots. Automated systems that trade without you. Scalp as fast as you like, what we cannot fund is software trading in your place."
vps_allowed: yes
personal_device_required: no
status: partial
not_confirmed_fields: ["trading_day_reset", "commission_per_contract.includes", "min_trading_days"]
notes: "Pays daily (cap $200 per day per account, $1,000 per day per trader across all Daily accounts), 80% split, min payout $75, profits mature 3 trading days (5 for a windfall day). Funded drawdown type EOD trailing. LOW PRIORITY (size below 25K)."
sources:
  - { field: drawdown, url: "https://phoenixtraderfunding.com/faq/detail/daily-accounts/daily-accounts-payout-rules", accessed: "2026-10-23" }
  - { field: max_position, url: "https://phoenixtraderfunding.com/faq/detail/daily-accounts/scaling-plan-daily-accounts", accessed: "2026-10-23" }
  - { field: automation_rule_text, url: "https://phoenixtraderfunding.com/faq/detail/most-asked-questions/are-there-any-forbidden-or-prohibited-strategies", accessed: "2026-10-23" }
  - { field: copy_trading_allowed, url: "https://phoenixtraderfunding.com/faq/detail/risk-management/is-copy-trading-allowed", accessed: "2026-10-23" }
```

#### Daily — funded (pre-funded) — 25K (Starter)

```yaml
firm: phoenix-trader-funding
plan: "Daily Starter"
phase: funded
size_label: "25K"
size_usd: "25000.00"
instruments: [MNQ, NQ]
platforms: ["Odin (Phoenix web platform, direct on TFeed)", "Quantower (direct on TFeed, needs addon)", "NinjaTrader 8 via Tradovate connection with TFeed credentials", "NinjaTrader via Thor addon (Sim101 copied)", "Tradovate", "TradingView via Tradovate or Thor webhooks", "Rithmic-leader platforms via Thor (Sierra Chart, MultiCharts, Bookmap, etc.)"]
drawdown: { amount_usd: "1500.00", type: trailing_eod, stops_trailing_at: "Stops trailing once the account has cleared its locked buffer ($1,500)" }
daily_loss_limit: { amount_usd: null, kind: none }
profit_target_usd: null
consistency_rule: { text: "None on the funded account", threshold_pct: null }
max_position: { minis: "1", micros: "15", scaling: "None: no scaling plan" }
min_trading_days: null
trading_day_reset: { time: null, tz_as_published: null }
commission_per_contract: { mnq_usd: "0.35", nq_usd: "1.09", basis: per_side, includes: "TFeed rates, per contract per side as published (round turn: MNQ 0.71, NQ 2.18). dxFeed rates (Classic only): MNQ 0.71, NQ 2.18 per side (round turn 1.42 / 4.36). Inclusion of exchange/NFA/clearing fees is not stated." }
copy_trading_allowed: yes
automated_trading_allowed: no
automation_rule_text: "HFT and EA bots. Automated systems that trade without you. Scalp as fast as you like, what we cannot fund is software trading in your place."
vps_allowed: yes
personal_device_required: no
status: partial
not_confirmed_fields: ["trading_day_reset", "commission_per_contract.includes", "min_trading_days"]
notes: "Pays daily (cap $500 per day per account, $1,000 per day per trader across all Daily accounts), 80% split, min payout $75, profits mature 3 trading days (5 for a windfall day). Funded drawdown type EOD trailing."
sources:
  - { field: drawdown, url: "https://phoenixtraderfunding.com/faq/detail/daily-accounts/daily-accounts-payout-rules", accessed: "2026-10-23" }
  - { field: max_position, url: "https://phoenixtraderfunding.com/faq/detail/daily-accounts/scaling-plan-daily-accounts", accessed: "2026-10-23" }
  - { field: automation_rule_text, url: "https://phoenixtraderfunding.com/faq/detail/most-asked-questions/are-there-any-forbidden-or-prohibited-strategies", accessed: "2026-10-23" }
  - { field: copy_trading_allowed, url: "https://phoenixtraderfunding.com/faq/detail/risk-management/is-copy-trading-allowed", accessed: "2026-10-23" }
```

#### Daily — funded (pre-funded) — 50K (Growth)

```yaml
firm: phoenix-trader-funding
plan: "Daily Growth"
phase: funded
size_label: "50K"
size_usd: "50000.00"
instruments: [MNQ, NQ]
platforms: ["Odin (Phoenix web platform, direct on TFeed)", "Quantower (direct on TFeed, needs addon)", "NinjaTrader 8 via Tradovate connection with TFeed credentials", "NinjaTrader via Thor addon (Sim101 copied)", "Tradovate", "TradingView via Tradovate or Thor webhooks", "Rithmic-leader platforms via Thor (Sierra Chart, MultiCharts, Bookmap, etc.)"]
drawdown: { amount_usd: "2000.00", type: trailing_eod, stops_trailing_at: "Stops trailing once the account has cleared its locked buffer ($2,000)" }
daily_loss_limit: { amount_usd: null, kind: none }
profit_target_usd: null
consistency_rule: { text: "None on the funded account", threshold_pct: null }
max_position: { minis: "3", micros: "30", scaling: "None: no scaling plan" }
min_trading_days: null
trading_day_reset: { time: null, tz_as_published: null }
commission_per_contract: { mnq_usd: "0.35", nq_usd: "1.09", basis: per_side, includes: "TFeed rates, per contract per side as published (round turn: MNQ 0.71, NQ 2.18). dxFeed rates (Classic only): MNQ 0.71, NQ 2.18 per side (round turn 1.42 / 4.36). Inclusion of exchange/NFA/clearing fees is not stated." }
copy_trading_allowed: yes
automated_trading_allowed: no
automation_rule_text: "HFT and EA bots. Automated systems that trade without you. Scalp as fast as you like, what we cannot fund is software trading in your place."
vps_allowed: yes
personal_device_required: no
status: partial
not_confirmed_fields: ["trading_day_reset", "commission_per_contract.includes", "min_trading_days"]
notes: "Pays daily (cap $1,000 per day per account, $1,000 per day per trader across all Daily accounts), 80% split, min payout $75, profits mature 3 trading days (5 for a windfall day). Funded drawdown type EOD trailing."
sources:
  - { field: drawdown, url: "https://phoenixtraderfunding.com/faq/detail/daily-accounts/daily-accounts-payout-rules", accessed: "2026-10-23" }
  - { field: max_position, url: "https://phoenixtraderfunding.com/faq/detail/daily-accounts/scaling-plan-daily-accounts", accessed: "2026-10-23" }
  - { field: automation_rule_text, url: "https://phoenixtraderfunding.com/faq/detail/most-asked-questions/are-there-any-forbidden-or-prohibited-strategies", accessed: "2026-10-23" }
  - { field: copy_trading_allowed, url: "https://phoenixtraderfunding.com/faq/detail/risk-management/is-copy-trading-allowed", accessed: "2026-10-23" }
```

#### Daily — live — 10K (Seed)

```yaml
firm: phoenix-trader-funding
plan: "Daily Seed"
phase: live
size_label: "10K"
size_usd: "10000.00"
instruments: [MNQ, NQ]
platforms: ["Rithmic EdgeClear connection (any Rithmic-capable platform)"]
drawdown: { amount_usd: null, type: static, stops_trailing_at: "Static on live" }
daily_loss_limit: { amount_usd: null, kind: none }
profit_target_usd: null
consistency_rule: { text: null, threshold_pct: null }
max_position: { minis: null, micros: null, scaling: "Live scaling: one extra mini for every $2,000 of additional profit on the Live account" }
min_trading_days: null
trading_day_reset: { time: null, tz_as_published: null }
commission_per_contract: { mnq_usd: null, nq_usd: null, basis: null, includes: "Live commission schedule not published separately." }
copy_trading_allowed: yes
automated_trading_allowed: conditional
automation_rule_text: "On a Live Funded account, every rule except overnight and over-week is waived. Those two stay. HFT and EA bots. Automated systems that trade without you. Scalp as fast as you like, what we cannot fund is software trading in your place."
vps_allowed: yes
personal_device_required: no
status: partial
not_confirmed_fields: ["trading_day_reset", "drawdown.amount_usd", "max_position", "automated_trading_allowed", "commission_per_contract", "live threshold"]
notes: "Moves to live when lifetime payouts reach a size-specific threshold shown on the dashboard (threshold amount not published in the FAQ) plus risk review; hosted by EdgeClear (Rithmic). LOW PRIORITY (size below 25K)."
sources:
  - { field: live, url: "https://phoenixtraderfunding.com/faq/detail/most-asked-questions/what-are-the-rules-for-being-live-funded", accessed: "2026-10-23" }
  - { field: platforms, url: "https://phoenixtraderfunding.com/faq/detail/most-asked-questions/which-broker-will-host-your-live-account", accessed: "2026-10-23" }
```

#### Daily — live — 25K (Starter)

```yaml
firm: phoenix-trader-funding
plan: "Daily Starter"
phase: live
size_label: "25K"
size_usd: "25000.00"
instruments: [MNQ, NQ]
platforms: ["Rithmic EdgeClear connection (any Rithmic-capable platform)"]
drawdown: { amount_usd: null, type: static, stops_trailing_at: "Static on live" }
daily_loss_limit: { amount_usd: null, kind: none }
profit_target_usd: null
consistency_rule: { text: null, threshold_pct: null }
max_position: { minis: null, micros: null, scaling: "Live scaling: one extra mini for every $2,000 of additional profit on the Live account" }
min_trading_days: null
trading_day_reset: { time: null, tz_as_published: null }
commission_per_contract: { mnq_usd: null, nq_usd: null, basis: null, includes: "Live commission schedule not published separately." }
copy_trading_allowed: yes
automated_trading_allowed: conditional
automation_rule_text: "On a Live Funded account, every rule except overnight and over-week is waived. Those two stay. HFT and EA bots. Automated systems that trade without you. Scalp as fast as you like, what we cannot fund is software trading in your place."
vps_allowed: yes
personal_device_required: no
status: partial
not_confirmed_fields: ["trading_day_reset", "drawdown.amount_usd", "max_position", "automated_trading_allowed", "commission_per_contract", "live threshold"]
notes: "Moves to live when lifetime payouts reach a size-specific threshold shown on the dashboard (threshold amount not published in the FAQ) plus risk review; hosted by EdgeClear (Rithmic)."
sources:
  - { field: live, url: "https://phoenixtraderfunding.com/faq/detail/most-asked-questions/what-are-the-rules-for-being-live-funded", accessed: "2026-10-23" }
  - { field: platforms, url: "https://phoenixtraderfunding.com/faq/detail/most-asked-questions/which-broker-will-host-your-live-account", accessed: "2026-10-23" }
```

#### Daily — live — 50K (Growth)

```yaml
firm: phoenix-trader-funding
plan: "Daily Growth"
phase: live
size_label: "50K"
size_usd: "50000.00"
instruments: [MNQ, NQ]
platforms: ["Rithmic EdgeClear connection (any Rithmic-capable platform)"]
drawdown: { amount_usd: null, type: static, stops_trailing_at: "Static on live" }
daily_loss_limit: { amount_usd: null, kind: none }
profit_target_usd: null
consistency_rule: { text: null, threshold_pct: null }
max_position: { minis: null, micros: null, scaling: "Live scaling: one extra mini for every $2,000 of additional profit on the Live account" }
min_trading_days: null
trading_day_reset: { time: null, tz_as_published: null }
commission_per_contract: { mnq_usd: null, nq_usd: null, basis: null, includes: "Live commission schedule not published separately." }
copy_trading_allowed: yes
automated_trading_allowed: conditional
automation_rule_text: "On a Live Funded account, every rule except overnight and over-week is waived. Those two stay. HFT and EA bots. Automated systems that trade without you. Scalp as fast as you like, what we cannot fund is software trading in your place."
vps_allowed: yes
personal_device_required: no
status: partial
not_confirmed_fields: ["trading_day_reset", "drawdown.amount_usd", "max_position", "automated_trading_allowed", "commission_per_contract", "live threshold"]
notes: "Moves to live when lifetime payouts reach a size-specific threshold shown on the dashboard (threshold amount not published in the FAQ) plus risk review; hosted by EdgeClear (Rithmic)."
sources:
  - { field: live, url: "https://phoenixtraderfunding.com/faq/detail/most-asked-questions/what-are-the-rules-for-being-live-funded", accessed: "2026-10-23" }
  - { field: platforms, url: "https://phoenixtraderfunding.com/faq/detail/most-asked-questions/which-broker-will-host-your-live-account", accessed: "2026-10-23" }
```

#### Spark — evaluation — 10K (Seed)

```yaml
firm: phoenix-trader-funding
plan: "Spark Seed"
phase: evaluation
size_label: "10K"
size_usd: "10000.00"
instruments: [MNQ, NQ]
platforms: ["Odin (Phoenix web platform, direct on TFeed)", "Quantower (direct on TFeed, needs addon)", "NinjaTrader 8 via Tradovate connection with TFeed credentials", "NinjaTrader via Thor addon (Sim101 copied)", "Tradovate", "TradingView via Tradovate or Thor webhooks", "Rithmic-leader platforms via Thor (Sierra Chart, MultiCharts, Bookmap, etc.)"]
drawdown: { amount_usd: "400.00", type: trailing_eod, stops_trailing_at: "Stops trailing when it reaches the starting balance" }
daily_loss_limit: { amount_usd: null, kind: none }
profit_target_usd: "600.00"
consistency_rule: { text: "No consistency rule during the challenge (funded account has 30%)", threshold_pct: null }
max_position: { minis: "1", micros: "10", scaling: "None: no scaling plan" }
min_trading_days: "1"
trading_day_reset: { time: null, tz_as_published: null }
commission_per_contract: { mnq_usd: "0.35", nq_usd: "1.09", basis: per_side, includes: "TFeed rates, per contract per side as published (round turn: MNQ 0.71, NQ 2.18). dxFeed rates (Classic only): MNQ 0.71, NQ 2.18 per side (round turn 1.42 / 4.36). Inclusion of exchange/NFA/clearing fees is not stated." }
copy_trading_allowed: yes
automated_trading_allowed: no
automation_rule_text: "HFT and EA bots. Automated systems that trade without you. Scalp as fast as you like, what we cannot fund is software trading in your place."
vps_allowed: yes
personal_device_required: no
status: partial
not_confirmed_fields: ["trading_day_reset", "commission_per_contract.includes"]
notes: "TFeed only. 1 minimum trading day. Challenge closed after 90 days without a trade. Spark Growth ($50K) is closed to new purchases. LOW PRIORITY (size below 25K)."
sources:
  - { field: profit_target_usd, url: "https://phoenixtraderfunding.com/faq/detail/spark-accounts/what-are-the-rules-of-the-spark-accounts", accessed: "2026-10-23" }
  - { field: drawdown, url: "https://phoenixtraderfunding.com/faq/detail/spark-accounts/spark-accounts-payout-rules", accessed: "2026-10-23" }
  - { field: max_position, url: "https://phoenixtraderfunding.com/faq/detail/spark-accounts/scaling-plan-spark-accounts", accessed: "2026-10-23" }
  - { field: commission_per_contract, url: "https://phoenixtraderfunding.com/faq/detail/risk-management/commissionsfees", accessed: "2026-10-23" }
  - { field: copy_trading_allowed, url: "https://phoenixtraderfunding.com/faq/detail/risk-management/is-copy-trading-allowed", accessed: "2026-10-23" }
  - { field: automation_rule_text, url: "https://phoenixtraderfunding.com/faq/detail/most-asked-questions/are-there-any-forbidden-or-prohibited-strategies", accessed: "2026-10-23" }
  - { field: inactivity, url: "https://phoenixtraderfunding.com/faq/detail/most-asked-questions/is-there-an-inactivity-rule", accessed: "2026-10-23" }
```

#### Spark — evaluation — 25K (Starter)

```yaml
firm: phoenix-trader-funding
plan: "Spark Starter"
phase: evaluation
size_label: "25K"
size_usd: "25000.00"
instruments: [MNQ, NQ]
platforms: ["Odin (Phoenix web platform, direct on TFeed)", "Quantower (direct on TFeed, needs addon)", "NinjaTrader 8 via Tradovate connection with TFeed credentials", "NinjaTrader via Thor addon (Sim101 copied)", "Tradovate", "TradingView via Tradovate or Thor webhooks", "Rithmic-leader platforms via Thor (Sierra Chart, MultiCharts, Bookmap, etc.)"]
drawdown: { amount_usd: "1000.00", type: trailing_eod, stops_trailing_at: "Stops trailing when it reaches the starting balance" }
daily_loss_limit: { amount_usd: null, kind: none }
profit_target_usd: "1500.00"
consistency_rule: { text: "No consistency rule during the challenge (funded account has 30%)", threshold_pct: null }
max_position: { minis: "2", micros: "20", scaling: "None: no scaling plan" }
min_trading_days: "1"
trading_day_reset: { time: null, tz_as_published: null }
commission_per_contract: { mnq_usd: "0.35", nq_usd: "1.09", basis: per_side, includes: "TFeed rates, per contract per side as published (round turn: MNQ 0.71, NQ 2.18). dxFeed rates (Classic only): MNQ 0.71, NQ 2.18 per side (round turn 1.42 / 4.36). Inclusion of exchange/NFA/clearing fees is not stated." }
copy_trading_allowed: yes
automated_trading_allowed: no
automation_rule_text: "HFT and EA bots. Automated systems that trade without you. Scalp as fast as you like, what we cannot fund is software trading in your place."
vps_allowed: yes
personal_device_required: no
status: partial
not_confirmed_fields: ["trading_day_reset", "commission_per_contract.includes"]
notes: "TFeed only. 1 minimum trading day. Challenge closed after 90 days without a trade. Spark Growth ($50K) is closed to new purchases."
sources:
  - { field: profit_target_usd, url: "https://phoenixtraderfunding.com/faq/detail/spark-accounts/what-are-the-rules-of-the-spark-accounts", accessed: "2026-10-23" }
  - { field: drawdown, url: "https://phoenixtraderfunding.com/faq/detail/spark-accounts/spark-accounts-payout-rules", accessed: "2026-10-23" }
  - { field: max_position, url: "https://phoenixtraderfunding.com/faq/detail/spark-accounts/scaling-plan-spark-accounts", accessed: "2026-10-23" }
  - { field: commission_per_contract, url: "https://phoenixtraderfunding.com/faq/detail/risk-management/commissionsfees", accessed: "2026-10-23" }
  - { field: copy_trading_allowed, url: "https://phoenixtraderfunding.com/faq/detail/risk-management/is-copy-trading-allowed", accessed: "2026-10-23" }
  - { field: automation_rule_text, url: "https://phoenixtraderfunding.com/faq/detail/most-asked-questions/are-there-any-forbidden-or-prohibited-strategies", accessed: "2026-10-23" }
  - { field: inactivity, url: "https://phoenixtraderfunding.com/faq/detail/most-asked-questions/is-there-an-inactivity-rule", accessed: "2026-10-23" }
```

#### Spark — funded (pre-funded) — 10K (Seed)

```yaml
firm: phoenix-trader-funding
plan: "Spark Seed"
phase: funded
size_label: "10K"
size_usd: "10000.00"
instruments: [MNQ, NQ]
platforms: ["Odin (Phoenix web platform, direct on TFeed)", "Quantower (direct on TFeed, needs addon)", "NinjaTrader 8 via Tradovate connection with TFeed credentials", "NinjaTrader via Thor addon (Sim101 copied)", "Tradovate", "TradingView via Tradovate or Thor webhooks", "Rithmic-leader platforms via Thor (Sierra Chart, MultiCharts, Bookmap, etc.)"]
drawdown: { amount_usd: "400.00", type: trailing_intraday, stops_trailing_at: "Stops trailing when it reaches the starting balance; at first withdrawal set to the starting balance and static" }
daily_loss_limit: { amount_usd: null, kind: none }
profit_target_usd: null
consistency_rule: { text: "Funded consistency rule 30% (text: 'the funded account does carry a consistency rule of 30%')", threshold_pct: "30" }
max_position: { minis: "1", micros: "10", scaling: "None: no scaling plan" }
min_trading_days: null
trading_day_reset: { time: null, tz_as_published: null }
commission_per_contract: { mnq_usd: "0.35", nq_usd: "1.09", basis: per_side, includes: "TFeed rates, per contract per side as published (round turn: MNQ 0.71, NQ 2.18). dxFeed rates (Classic only): MNQ 0.71, NQ 2.18 per side (round turn 1.42 / 4.36). Inclusion of exchange/NFA/clearing fees is not stated." }
copy_trading_allowed: yes
automated_trading_allowed: no
automation_rule_text: "HFT and EA bots. Automated systems that trade without you. Scalp as fast as you like, what we cannot fund is software trading in your place."
vps_allowed: yes
personal_device_required: no
status: partial
not_confirmed_fields: ["trading_day_reset", "commission_per_contract.includes", "min_trading_days", "drawdown.amount_usd (funded restated?)"]
notes: "Funded drawdown trails in real time (funded drawdown amount not restated; package value used). Pays bi-weekly, 90% split, max per payout $300, min payout $30, buffer none. Activation fee applies (only Spark has one; amount $29 in package data). LOW PRIORITY (size below 25K)."
sources:
  - { field: drawdown, url: "https://phoenixtraderfunding.com/faq/detail/spark-accounts/spark-accounts-payout-rules", accessed: "2026-10-23" }
  - { field: consistency_rule, url: "https://phoenixtraderfunding.com/faq/detail/spark-accounts/what-are-the-rules-of-the-spark-accounts", accessed: "2026-10-23" }
  - { field: max_position, url: "https://phoenixtraderfunding.com/faq/detail/spark-accounts/scaling-plan-spark-accounts", accessed: "2026-10-23" }
  - { field: automation_rule_text, url: "https://phoenixtraderfunding.com/faq/detail/most-asked-questions/are-there-any-forbidden-or-prohibited-strategies", accessed: "2026-10-23" }
```

#### Spark — funded (pre-funded) — 25K (Starter)

```yaml
firm: phoenix-trader-funding
plan: "Spark Starter"
phase: funded
size_label: "25K"
size_usd: "25000.00"
instruments: [MNQ, NQ]
platforms: ["Odin (Phoenix web platform, direct on TFeed)", "Quantower (direct on TFeed, needs addon)", "NinjaTrader 8 via Tradovate connection with TFeed credentials", "NinjaTrader via Thor addon (Sim101 copied)", "Tradovate", "TradingView via Tradovate or Thor webhooks", "Rithmic-leader platforms via Thor (Sierra Chart, MultiCharts, Bookmap, etc.)"]
drawdown: { amount_usd: "1000.00", type: trailing_intraday, stops_trailing_at: "Stops trailing when it reaches the starting balance; at first withdrawal set to the starting balance and static" }
daily_loss_limit: { amount_usd: null, kind: none }
profit_target_usd: null
consistency_rule: { text: "Funded consistency rule 30% (text: 'the funded account does carry a consistency rule of 30%')", threshold_pct: "30" }
max_position: { minis: "2", micros: "20", scaling: "None: no scaling plan" }
min_trading_days: null
trading_day_reset: { time: null, tz_as_published: null }
commission_per_contract: { mnq_usd: "0.35", nq_usd: "1.09", basis: per_side, includes: "TFeed rates, per contract per side as published (round turn: MNQ 0.71, NQ 2.18). dxFeed rates (Classic only): MNQ 0.71, NQ 2.18 per side (round turn 1.42 / 4.36). Inclusion of exchange/NFA/clearing fees is not stated." }
copy_trading_allowed: yes
automated_trading_allowed: no
automation_rule_text: "HFT and EA bots. Automated systems that trade without you. Scalp as fast as you like, what we cannot fund is software trading in your place."
vps_allowed: yes
personal_device_required: no
status: partial
not_confirmed_fields: ["trading_day_reset", "commission_per_contract.includes", "min_trading_days", "drawdown.amount_usd (funded restated?)"]
notes: "Funded drawdown trails in real time (funded drawdown amount not restated; package value used). Pays bi-weekly, 90% split, max per payout $400, min payout $75, buffer none. Activation fee applies (only Spark has one; amount $29 in package data)."
sources:
  - { field: drawdown, url: "https://phoenixtraderfunding.com/faq/detail/spark-accounts/spark-accounts-payout-rules", accessed: "2026-10-23" }
  - { field: consistency_rule, url: "https://phoenixtraderfunding.com/faq/detail/spark-accounts/what-are-the-rules-of-the-spark-accounts", accessed: "2026-10-23" }
  - { field: max_position, url: "https://phoenixtraderfunding.com/faq/detail/spark-accounts/scaling-plan-spark-accounts", accessed: "2026-10-23" }
  - { field: automation_rule_text, url: "https://phoenixtraderfunding.com/faq/detail/most-asked-questions/are-there-any-forbidden-or-prohibited-strategies", accessed: "2026-10-23" }
```

#### Spark — live — 10K (Seed)

```yaml
firm: phoenix-trader-funding
plan: "Spark Seed"
phase: live
size_label: "10K"
size_usd: "10000.00"
instruments: [MNQ, NQ]
platforms: ["Rithmic EdgeClear connection (any Rithmic-capable platform)"]
drawdown: { amount_usd: null, type: static, stops_trailing_at: "Static on live" }
daily_loss_limit: { amount_usd: null, kind: none }
profit_target_usd: null
consistency_rule: { text: null, threshold_pct: null }
max_position: { minis: null, micros: null, scaling: "Live scaling: one extra mini for every $2,000 of additional profit on the Live account" }
min_trading_days: null
trading_day_reset: { time: null, tz_as_published: null }
commission_per_contract: { mnq_usd: null, nq_usd: null, basis: null, includes: "Live commission schedule not published separately." }
copy_trading_allowed: yes
automated_trading_allowed: conditional
automation_rule_text: "On a Live Funded account, every rule except overnight and over-week is waived. Those two stay. HFT and EA bots. Automated systems that trade without you. Scalp as fast as you like, what we cannot fund is software trading in your place."
vps_allowed: yes
personal_device_required: no
status: partial
not_confirmed_fields: ["trading_day_reset", "drawdown.amount_usd", "max_position", "automated_trading_allowed", "commission_per_contract"]
notes: "Live after four payouts; live funding amount $400; EdgeClear (Rithmic). LOW PRIORITY (size below 25K)."
sources:
  - { field: live, url: "https://phoenixtraderfunding.com/faq/detail/spark-accounts/spark-accounts-payout-rules", accessed: "2026-10-23" }
  - { field: platforms, url: "https://phoenixtraderfunding.com/faq/detail/most-asked-questions/which-broker-will-host-your-live-account", accessed: "2026-10-23" }
```

#### Spark — live — 25K (Starter)

```yaml
firm: phoenix-trader-funding
plan: "Spark Starter"
phase: live
size_label: "25K"
size_usd: "25000.00"
instruments: [MNQ, NQ]
platforms: ["Rithmic EdgeClear connection (any Rithmic-capable platform)"]
drawdown: { amount_usd: null, type: static, stops_trailing_at: "Static on live" }
daily_loss_limit: { amount_usd: null, kind: none }
profit_target_usd: null
consistency_rule: { text: null, threshold_pct: null }
max_position: { minis: null, micros: null, scaling: "Live scaling: one extra mini for every $2,000 of additional profit on the Live account" }
min_trading_days: null
trading_day_reset: { time: null, tz_as_published: null }
commission_per_contract: { mnq_usd: null, nq_usd: null, basis: null, includes: "Live commission schedule not published separately." }
copy_trading_allowed: yes
automated_trading_allowed: conditional
automation_rule_text: "On a Live Funded account, every rule except overnight and over-week is waived. Those two stay. HFT and EA bots. Automated systems that trade without you. Scalp as fast as you like, what we cannot fund is software trading in your place."
vps_allowed: yes
personal_device_required: no
status: partial
not_confirmed_fields: ["trading_day_reset", "drawdown.amount_usd", "max_position", "automated_trading_allowed", "commission_per_contract"]
notes: "Live after four payouts; live funding amount $1,000; EdgeClear (Rithmic)."
sources:
  - { field: live, url: "https://phoenixtraderfunding.com/faq/detail/spark-accounts/spark-accounts-payout-rules", accessed: "2026-10-23" }
  - { field: platforms, url: "https://phoenixtraderfunding.com/faq/detail/most-asked-questions/which-broker-will-host-your-live-account", accessed: "2026-10-23" }
```

#### Merit — evaluation — $2K (single size)

```yaml
firm: phoenix-trader-funding
plan: "Merit"
phase: evaluation
size_label: "2K"
size_usd: "2000.00"
instruments: [MNQ, NQ]
platforms: ["Odin (Phoenix web platform, direct on TFeed)", "Quantower (direct on TFeed, needs addon)", "NinjaTrader 8 via Tradovate connection with TFeed credentials", "NinjaTrader via Thor addon (Sim101 copied)", "Tradovate", "TradingView via Tradovate or Thor webhooks", "Rithmic-leader platforms via Thor (Sierra Chart, MultiCharts, Bookmap, etc.)", "dxFeed: feed availability for Merit not stated in the FAQ"]
drawdown: { amount_usd: "2000.00", type: trailing_eod, stops_trailing_at: "Maximum drawdown: 'The full account balance'; live article: 'stops trailing once your equity returns to the initial balance'" }
daily_loss_limit: { amount_usd: null, kind: soft_pause }
profit_target_usd: null
consistency_rule: { text: null, threshold_pct: null }
max_position: { minis: null, micros: null, scaling: "Position size 'Set by the account size' (package data max_contracts 2, unit not stated)" }
min_trading_days: null
trading_day_reset: { time: null, tz_as_published: null }
commission_per_contract: { mnq_usd: "0.35", nq_usd: "1.09", basis: per_side, includes: "TFeed rates, per contract per side as published (round turn: MNQ 0.71, NQ 2.18). dxFeed rates (Classic only): MNQ 0.71, NQ 2.18 per side (round turn 1.42 / 4.36). Inclusion of exchange/NFA/clearing fees is not stated." }
copy_trading_allowed: yes
automated_trading_allowed: no
automation_rule_text: "HFT and EA bots. Automated systems that trade without you. Scalp as fast as you like, what we cannot fund is software trading in your place."
vps_allowed: yes
personal_device_required: no
status: conflict
not_confirmed_fields: ["trading_day_reset", "max_position", "drawdown.type (challenge not restated)", "conflict: tiers/commission vs /merit-accounts page"]
notes: "30 calendar days, no profit target, no minimum days, daily drawdown '25% Dynamic (Soft)', no activation fee, one attempt (cannot be reset), decided by risk review. The /merit-accounts marketing page conflicts with the FAQ on tiers and commission (section 10). LOW PRIORITY (size $2K, below 25K)."
sources:
  - { field: drawdown, url: "https://phoenixtraderfunding.com/faq/detail/merit-accounts/what-is-a-merit-account-how-do-they-work-what-are-the-rules-for-a-merit-account", accessed: "2026-10-23" }
  - { field: drawdown, url: "https://phoenixtraderfunding.com/faq/detail/merit-accounts/how-much-money-is-in-the-live-funded-accounts-merit-accounts", accessed: "2026-10-23" }
  - { field: daily_loss_limit, url: "https://phoenixtraderfunding.com/faq/detail/merit-accounts/what-is-a-merit-account-how-do-they-work-what-are-the-rules-for-a-merit-account", accessed: "2026-10-23" }
  - { field: commission_per_contract, url: "https://phoenixtraderfunding.com/faq/detail/risk-management/commissionsfees", accessed: "2026-10-23" }
  - { field: sizes, url: "https://phoenixtraderfunding.com/merit-accounts", accessed: "2026-10-23" }
  - { field: packages_cross_check, url: "https://server.phoenixtraderfunding.com/public/api/packages/fetch", accessed: "2026-10-23" }
```

#### Merit — live — $2K (single size)

```yaml
firm: phoenix-trader-funding
plan: "Merit"
phase: live
size_label: "2K"
size_usd: "2000.00"
instruments: [MNQ, NQ]
platforms: ["Rithmic EdgeClear connection (any Rithmic-capable platform)"]
drawdown: { amount_usd: "2000.00", type: trailing_eod, stops_trailing_at: "Stops trailing once equity returns to the initial balance, then fixed" }
daily_loss_limit: { amount_usd: null, kind: none }
profit_target_usd: null
consistency_rule: { text: null, threshold_pct: null }
max_position: { minis: null, micros: null, scaling: "Same as the challenge ('Position size: Same as the challenge'); Live scales one extra mini per $2,000 additional profit" }
min_trading_days: null
trading_day_reset: { time: null, tz_as_published: null }
commission_per_contract: { mnq_usd: null, nq_usd: null, basis: null, includes: "Merit pays the same commissions as every other Phoenix account (FAQ)." }
copy_trading_allowed: yes
automated_trading_allowed: conditional
automation_rule_text: "On a Live Funded account, every rule except overnight and over-week is waived. Those two stay. HFT and EA bots. Automated systems that trade without you. Scalp as fast as you like, what we cannot fund is software trading in your place."
vps_allowed: yes
personal_device_required: no
status: conflict
not_confirmed_fields: ["trading_day_reset", "max_position", "automated_trading_allowed", "commission_per_contract"]
notes: "Straight to live on approval (EdgeClear, Rithmic). No daily drawdown, buffer, withdrawal limits or consistency rule. Adaptive split 50/50 then 80%. Drawdown amount for live not restated (package value used). LOW PRIORITY (size $2K)."
sources:
  - { field: drawdown, url: "https://phoenixtraderfunding.com/faq/detail/merit-accounts/how-much-money-is-in-the-live-funded-accounts-merit-accounts", accessed: "2026-10-23" }
  - { field: payout, url: "https://phoenixtraderfunding.com/faq/detail/merit-accounts/merit-accounts-payout-rules", accessed: "2026-10-23" }
```

---

## 10. Unverified / gaps / conflicts

| Item | Status | Detail |
|---|---|---|
| Trading-day reset time and time zone | unknown | Not published (section 4). All blocks `trading_day_reset: null`. |
| Personal device requirement | closed R4: no | F-vpn-vps: "There is no rule against connecting through a VPN, a VPS or a proxy"; connections are monitored (one trader, one identity). |
| Conflict: Merit tiers and live amounts | `conflict` | Help Center (https://phoenixtraderfunding.com/faq/detail/merit-accounts/how-much-money-is-in-the-live-funded-accounts-merit-accounts): "$2,000. Merit comes in a single size, and an approved Merit challenge is funded with $2,000 on the live account." and package data (one $2,000 package, price 69). Marketing page `/merit-accounts` (https://phoenixtraderfunding.com/merit-accounts): the page lists "Merit $10K Tier => $1000", "Merit $50K Tier => $2000" and "Merit $100K Tier => $3500" as live amounts (list items in the page's JS). Both recorded; not resolved. |
| Conflict: ES commission on the Merit page | `conflict` | Marketing page (HTML link text omitted): "are the same as on other accounts, at 1.99 for an ES contract"; Help Center commission table: ES dxFeed 2.18 per side / 4.36 round turn, TFeed 1.09 / 2.18. MNQ/NQ are not stated on the Merit page, so only ES is in conflict. |
| Merit profit split (info) | note | Marketing page: "50:50 until we get our money back, then 80:20"; Help Center: 50/50 then 80%. Consistent. |
| Live Funded automation | ambiguous | Section 6. Cannot tell whether "HFT and EA bots" stay prohibited on Live. |
| Live drawdown amount and live starting size | unknown | Live drawdown type is static; amount and live position limits other than one extra mini for every $2,000 of additional profit are not published. |
| Merit position size | unknown | "Set by the account size"; package data `max_contracts` 2 has no unit. |
| Merit feed / platform | unknown | Not stated in the Help Center. |
| Daily live-transfer threshold | unknown | "shown on your dashboard" (behind login). |
| Funded drawdown amounts (Classic, Daily, Spark, Merit) | partial | The articles state types, buffers and the starting-balance lock; the dollar amount is taken from the package data and the locked buffer equal to it (Classic, Daily). Spark has no buffer, so the funded amount is the package value only. |
| Exchange/NFA/clearing fees in commissions | unknown | Not stated. |
| Live commissions | unknown | No separate live schedule. |
| Page freshness | note | Help Center articles say "these rules are subject to change"; some rules apply "from 15 September 2026". |

---

## 11. Changelog

- 2026-10-23 — initial research (Lupa)
- 2026-10-23 — rev bump R4: `personal_device_required` closed as `no` (F-vpn-vps explicitly permits VPN/VPS/proxy, re-captured from https://server.phoenixtraderfunding.com/public/api/faqs/fetch); `vps_allowed: yes` re-confirmed unchanged. `trading_day_reset` stays unpublished: searched all FAQ articles via the faqs API and the terms bundle for reset, session, time zone and midnight; only "the limit is recalculated once a day, after the close" and "Every position must be closed before the session ends." and "Only trading days are counted, weekends and market holidays do not move the clock" (payout clock only) (Lupa)
