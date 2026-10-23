# MyFundedFutures (MFFU) — Prop Firm Rules Research

| | |
|---|---|
| **Firm** | MyFundedFutures ("MFFU"; legal name in the site footer: "MyFunded Futures, LLC") |
| **Official domains** | `myfundedfutures.com` (main site; plain `curl` works, no Cloudflare block) · `help.myfundedfutures.com` (official help center, Intercom-hosted; linked from the main site's footer "FAQ" link `https://help.myfundedfutures.com/en/`, and the help center links back with "Go to website") · `myfundedperpetuals.com` is a separate company ("Funding Perpetuals Holdings, Inc.") and is out of scope |
| **Research date** | 2026-10-23 (all URLs below accessed 2026-10-23) |
| **Researcher** | Lupa (Prop Firm Rules Researcher) |
| **File status** | `partial`: every current plan, size and phase has drawdown, DLL, target, consistency, contracts and minimum days captured. Still open: the conflicts and gaps in §9, the lack of an explicit trading-day reset time, and the lack of an explicit same-person copy sentence (see §5) |
| **Instruments covered** | MNQ, NQ. Both are on the official instrument list (`NQ` "E-mini NASDAQ 100", `MNQ` "Micro E-mini Nasdaq-100", https://help.myfundedfutures.com/en/articles/9735811-futures-instrument-list). Studio plans allow micros only ("no minis") |

Evidence (raw text of every cited page, with the URL on line 1): `/tmp/claude-1003/-srv-www-allwin--maestri-roles-6bf77a80-a1ed-43dc-bc00-08a480f178a5/33652d13-4dae-46b6-ae06-44a72b375517/scratchpad/r2/myfundedfutures/`. Files are `mffu-*.txt` (main site), `hc-<article-id>-*.txt` (help center) and `portal-plans_rapid-150k-all.txt`, which holds the rendered plan-page table read in the Maestri portal "Lupa MFFU" to confirm the curl capture. Raw HTML is in `raw/`. This is temporary scratch storage, so the evidence must be re-captured at re-verification.

**Programs found.** The homepage says "Four plans, each with clear rules" and shows two tabs: "Standard Plans" (Builder, Rapid, Rapid EOD, Pro) and "Studio — Static Plans" (Studio #001, #002, #003; "Limited-time plans", sold only on `/studio` with "a daily purchase cap"). The help center has one collection each for Rapid Plans, Builder Plan, Pro Plan, Studio Plans and Live Accounts, plus a "Legacy Plans" collection. Legacy plans are out of scope: Flex ("discontinued"), Builder 25k "Legacy No DLL" and the legacy Pro plan. The Rapid plan page FAQ still mentions "Flex is a separate plan available in $25K and $50K sizes", but no current plan page exists for it, and the Flex help articles say "Effective August 5th, at 10 pm est the Flex plan will be discontinued." Payment model: one-time payment for purchases on or after August 25, 2026, and "Legacy Subscriptions" for earlier purchases (Terms §6; https://help.myfundedfutures.com/en/articles/16596524-inactivity-rule-one-time-payment-model).

**Phases.** Every plan has Evaluation → "Sim Funded" (simulated, payouts are real) → "Live" ("a real live brokerage account at Blue Row Capital"). All MFFU evaluation and Sim Funded accounts are simulated ("All MFFU accounts operate in a simulated environment", site footer). **Live accounts are real money, so they are outside agent scope (user-only).** Their parameters are recorded for completeness.

**Sizes outside 25K–300K:** Studio #003 $5K, #001 $10K, #002 $20K (low priority; recorded in §3.5). No size above $150K exists.

**Mini:micro ratio.** The help center uses 1:10, for example: "You may trade up to 5 minis and/or 50 micros total. Exceeding a total of 5 minis or its equivalent in micros (50 micros) can result in a breach" (Rapid 50k). The cross-instrument policy says that "Orders that combine different instruments (e.g., 2 minis and 20 micros) may technically execute even if they exceed the overall limit … but this is not permitted!" (https://help.myfundedfutures.com/en/articles/10244682-cross-instrument-policy). The one exception is Pro Sim Funded, which is published as "3 mini / 3 micro" etc. (see §3.4).

---

## 1. Platforms

Source: https://help.myfundedfutures.com/en/articles/8528335-overview-of-supported-platforms-at-mffu, plus the login articles in the "Trading Platforms" collection, https://help.myfundedfutures.com/en/collections/5786939-trading-platforms.

| Platform | Connection / data path (as published) | Source |
|---|---|---|
| Tradovate | Tradovate login with MFFU credentials, "Click Continue below Simulation" | https://help.myfundedfutures.com/en/articles/8445591-tradovate-login-instructions |
| NinjaTrader | "find and select NinjaTrader … set the account type to 'Simulation'" (MFFU credentials) | https://help.myfundedfutures.com/en/articles/10082979-ninjatrader-login-instructions |
| TradingView | through Tradovate ("Login to Tradovate via TradingView's Trading Panel"); dashboard: "Traders who select the Volumetric platform (dxFeed) will have access to Tradingview directly from your dashboard" | https://help.myfundedfutures.com/en/articles/9006753-how-to-trade-with-tradingview · https://help.myfundedfutures.com/en/articles/8229980-navigating-your-dashboard |
| Quantower | "click dxFeed … select MyFundedFutures" | https://help.myfundedfutures.com/en/articles/10083873-quantower-login-instructions |
| ATAS | "select dxFeed prop" | https://help.myfundedfutures.com/en/articles/10088299-atas-login-instructions |
| DeepChart / DeepDom (Volsys/Volbook) | "Select dxFeed as the Feed source and Propfirm as the server" | https://help.myfundedfutures.com/en/articles/10083143-deepchart-login-instructions |
| Volumetrica | web platform, "connected to dxFeed" | https://help.myfundedfutures.com/en/articles/9709179-volumetrica-getting-started |
| Hyperprop | "Hyperprop is live on MyFundedFutures. Trade every account you hold from one screen, with live licensed CME data" | https://myfundedfutures.com/hyperprop |

- In short, there are two account back ends, **Tradovate** (Tradovate, NinjaTrader, TradingView) and **dxFeed** (Quantower, ATAS, DeepChart/DeepDom, Volumetrica), plus **Hyperprop**. Rithmic, CQG and ProjectX are not mentioned on any official page.
- **Per plan:** no official page restricts Builder, Rapid, Rapid EOD or Pro to particular platforms, so the full list is recorded for them. **Studio: "Hyperprop only. More platforms coming soon."** (https://help.myfundedfutures.com/en/articles/17589665-studio-plans-a-comprehensive-guide).
- **Live:** the platform is not published. The live article lists "Platform License or Subscription" as a cost the trader pays (https://help.myfundedfutures.com/en/articles/10101257-understanding-live-funded-account-at-myfunded-futures). The YAML uses `platforms: []` for live blocks.
- No rule found differs by platform, so there is one YAML block per plan/phase/size.
- For the NinjaTrader 8 plugin, MFFU accounts appear under the NinjaTrader connection with account type "Simulation". Sim Funded accounts have "'SIM' in the account name" (https://help.myfundedfutures.com/en/articles/16498635-moving-from-evaluation-to-sim-funded-account).

---

## 2. Account limits (max accounts per person)

Source: https://help.myfundedfutures.com/en/articles/16498635-moving-from-evaluation-to-sim-funded-account (also https://help.myfundedfutures.com/en/articles/11802636-traders-evaluation-simplified).

> "Traders may hold a maximum of 10 active Evaluation Accounts at any given time."
> "Traders may maintain up to 5 active Sim-Funded Accounts when holding only $25K and/or $50K account sizes."
> "Plan-Specific Limits — Rapid EOD $25K: Maximum of 3 Sim-Funded Accounts · Rapid EOD $50K: Maximum of 3 Sim-Funded Accounts · Builder $25K: Maximum of 2 Sim-Funded Account · Builder $50K: Maximum of 1 Sim-Funded Accounts"
> "If a trader holds any $100K or $150K Sim-Funded Account, the maximum number of active Sim-Funded Accounts is 3. The 3-account limit applies across all Sim-Funded account sizes and plan types."

- Builder 100K/150K: "Traders can hold one active Builder Sim Funded account at a time" (help center).
- Studio: "One active Studio account at a time — either Evaluation or Sim Funded."
- **Conflict (Builder 25K):** the plan-page rules table says "2 Builder 25K per user" and the help center says "up to 2 Sim Funded accounts, per user", but the same plan page's FAQ says "No. Builder is limited to one active Builder account per user." (https://myfundedfutures.com/plans/builder).
- Live: "Sim Funded Accounts will be merged into one Live Funded Account"; "Multiple live accounts … are reserved solely for high-performing traders". Builder: "Max Live Accounts 1".
- Terms 23.1.3: "While Users may lawfully purchase and maintain multiple evaluation or funded accounts within the Account Limits published by the Company, all such accounts must belong to the same verified individual." (https://myfundedfutures.com/terms)

---

## 3. Plans

Abbreviations: **MLL** = Max Loss Limit (the drawdown). **DLL** = Daily Loss Limit. **SF** = Sim Funded.

General mechanics, quoted:
- **EOD drawdown** (https://help.myfundedfutures.com/en/articles/8348565-end-of-day-eod-drawdown-explained): "We do not have a daily drawdown but we will have a Max EOD (End of Day) Drawdown. Also to be noted the Max EOD trailing locks in at $100 plus the initial starting balance." / "open equity losses are taken into consideration when calculating whether or not the account failed on this rule." Examples are given verbatim: "50k is 52k +$100 which is 52.1k … 100k is 103k +$100 which is 103.1k · 150k is 154.5k +100 which is 154.6k".
- **Intraday drawdown** (https://help.myfundedfutures.com/en/articles/12802721-intraday-drawdown-explained): "At the funded stage, the trailing drawdown is calculated intraday based on your peak balance, which includes both realized and unrealized gains." / "if your equity, including open trades, falls below the most recently adjusted drawdown limit, your account will breach the maximum loss limit."
- **Consistency** (https://help.myfundedfutures.com/en/articles/11994562-consistency-rule-at-my-fundedfutures): "The consistency rule at MyFundedFutures is 30% on our Rapid EOD plans and 50% for Rapid Intraday and Pro plans. (Pro Plan One day pass has no consistency rule) Our Builder plans have no consistency rules in the evaluation stage … *** Builder plans have a 50% consistency rule in the simulated funded stage. ***" / "Exceeding the consistency targets does not breach the account. Traders … simply need to trade additional days until consistency is met."
- **Overnight:** "All positions must be closed before the end of the trading session" (Builder); "Holding overnight will breach the account." (Rapid plan page).
- **Inactivity:** "If a trader does not place a trade for 7 consecutive calendar days, the account could be closed. This includes simulated funded accounts." Live: "There is no inactivity rule on the live account."

### 3.1 Builder (sizes 25K / 50K / 100K / 150K; 50K also has an "Add-On" MLL option)

Sources: plan page https://myfundedfutures.com/plans/builder (rules table "All Builder plan rules by account size") · help center: 25K https://help.myfundedfutures.com/en/articles/17036130-builder-plan-25k-a-comprehensive-look · 50K https://help.myfundedfutures.com/en/articles/14290805-builder-plan-50k-a-comprehensive-guide · 100K https://help.myfundedfutures.com/en/articles/17005296-builder-plan-100k-comprehensive-guide · 150K https://help.myfundedfutures.com/en/articles/17035331-builder-plan-150k-comprehensive-guide

**Evaluation**

| Size | Target | MLL / type | DLL | Consistency | Max contracts | Min days |
|---|---|---|---|---|---|---|
| 25K ($25,000) | $1,500 | $1,000 EOD trailing (floor $24,000) | $600 soft pause | None | 2 minis / 20 micros | 1 day (**conflict**, see below) |
| 50K ($50,000) | $3,000 | $2,000 EOD trailing (floor $48,000); Add-On $1,500 (floor $48,500) | $1,000 soft pause | None | 4 minis / 40 micros | 1 day (**conflict**) |
| 100K ($100,000) | $6,000 | $3,000 EOD trailing (floor $97,000) | $1,750 soft pause | None | 6 minis / 60 micros | 1 day (**conflict**) |
| 150K ($150,000) | $9,000 | $4,500 EOD trailing (floor $145,500) | $2,500 soft pause | None | 9 minis / 90 micros | 1 day (**conflict**) |

**Sim Funded**

| Size | MLL / type | DLL | Consistency (payout) | Max contracts | Buffer | Payout cap / cycle | Active SF accounts |
|---|---|---|---|---|---|---|---|
| 25K | $1,000 EOD; starts at $0 (min balance -$1,000) | $600 soft pause | 50% | 2 / 20 | $1,100 | $1,000 | 2 (**conflict**: plan FAQ "one") |
| 50K | $2,000 EOD ($1,500 Add-On) | $1,000 soft pause | 50% | 4 / 40 | $2,100 ($1,600 Add-On) | $2,000 | 1 |
| 100K | $3,000 EOD | $1,750 soft pause | 50% | 6 / 60 | $3,100 | $3,000 | 1 |
| 150K | $4,500 EOD | $2,500 soft pause | 50% | 9 / 90 | $4,600 | **conflict**: $3,500 (plan page) vs $4,500 (help center) | 1 |

**Live** (after the 5th approved sim payout; Blue Row Capital; user-only)

| Size | MLL | Lock | DLL (live) | Max contracts | Max live accts |
|---|---|---|---|---|---|
| 25K | $1,000 EOD trailing | "Static once MLL reaches $0" | **conflict**: $600 (plan page) vs "None" (help center) | not published | 1 |
| 50K | $2,000 EOD trailing ($1,500 Add-On) | "Static once MLL reaches $0" | $1,000 (both; help-center FAQ: "The Evaluation, Sim Funded and Live stage have a $1,000 soft pause DLL.") | not published | 1 |
| 100K | $3,000 EOD trailing | **conflict**: "Static once MLL reaches $0" (plan page) vs "Locks and remains static once it reaches $100 above the starting balance" (help center) | **conflict**: $1,750 vs "None" | 3 Minis / 30 Micros (help center only) | 1 |
| 150K | $4,500 EOD trailing | **conflict**, as for 100K | **conflict**: $2,500 vs "None" | 4 Minis / 40 Micros (help center only) | 1 |

- **Drawdown, quoted** (help center, Account Rules): "The Maximum Loss Limit (MLL) is an End of Day (EOD) trailing drawdown. It does not move intraday. It only adjusts after the market closes each session, based on where your account closed. … It never moves down. … The MLL locks permanently once it reaches $100 above the starting balance." The plan-page SF row says "MLL Lock Level $100". The payout-policy article says "After your first payout, the Max Loss Limit resets to $100" (https://help.myfundedfutures.com/en/articles/13745661-payout-policy-overview-best-and-fastest-prop-firm-payouts). These are different lock conditions, so `stops_trailing_at` is marked not confirmed for SF.
- **DLL, quoted:** "Soft DLL does not breach your account. Hitting the $600 Soft DLL soft‑pauses trading for the rest of the trading session. The account remains open … Only breaching the Max Loss Limit (MLL) closes the account." / "It applies during both the Evaluation and Sim Funded stages." **Conflict:** the payout-policy article says "No daily loss limits on Builder, Rapid or Pro plans".
- **Minimum days conflict:** the plan-page table and the help center say "Min Trading Days 1 Day" ("Builder evaluations have a one-day minimum"), but the plan-page stage cards say "Evaluation Stage — One phase · min 2 days · no activation fee".
- **Payout consistency, quoted:** "your single largest profit day cannot represent more than 50% of your total profits in the cycle. This calculation resets after each approved payout." Each cycle also needs a minimum of "2 Days" (qualifying days), and payouts can be requested "48 hours after your first trade". Required net profit per payout: 25K $250, 50K $500, 100K $1,000, 150K $1,500.
- **News:** "News trading is fully unrestricted during the evaluation and sim funded stage."
- **Live post-breach:** "21-Day Cooldown Period … All Sim Funded account trading is prohibited."
- The plan-page hero says "$25K & $50K Builder", but the same page's tables and the help center have four sizes.

### 3.2 Rapid (intraday SF drawdown; sizes 25K / 50K / 100K / 150K)

Sources: plan page https://myfundedfutures.com/plans/rapid (rules table; the rendered $150K/All tab was confirmed in the portal) · help center: 25K https://help.myfundedfutures.com/en/articles/14116402-rapid-plan-25k-a-comprehensive-look · 50K https://help.myfundedfutures.com/en/articles/13134709-rapid-plan-50k-a-comprehensive-look · 100K https://help.myfundedfutures.com/en/articles/13286542-rapid-plan-100k-a-comprehensive-look · 150K https://help.myfundedfutures.com/en/articles/13286582-rapid-plan-150k-a-comprehensive-look · Live https://help.myfundedfutures.com/en/articles/13134718-understanding-rapid-live

| Size | Eval target | Eval MLL (EOD) | Eval DLL | Eval consistency | Eval max | Min days | SF MLL (intraday trailing) | SF max | SF buffer | Live MLL (EOD, stops at $0) | Live max |
|---|---|---|---|---|---|---|---|---|---|---|---|
| 25K | $1,500 | $1,000 | None | 50% (Eval Only) | 3 / 30 | 2 | $1,000, locks at $100 | 3 / 30 | $1,100 | $1,000 | 2 / 20 |
| 50K | $3,000 | $2,000 | None | 50% | 5 / 50 | 2 | $2,000, locks at $100 | 5 / 50 (**conflict** with the scaling example, see below) | $2,100 | $2,000 | 3 / 30 |
| 100K | $6,000 | $3,000 | None | 50% | 8 / 80 | 2 | $3,000, locks at $100 | 8 / 80 | $3,100 | $3,000 | 6 / 60 |
| 150K | $9,000 | $4,500 | None | 50% | 10 / 100 | 2 | $4,500, locks at $100 | 10 / 100 | $4,600 | $4,500 | 8 / 80 |

- **DLL:** "No. Rapid has no daily loss limit in either the evaluation or the funded stage." (plan page FAQ)
- **SF drawdown, quoted** (help center): "Initial Balance: $0 · Max Loss Distance: $2,000 from your equity high-water mark (HWM) · Drawdown Type: Intra-day trailing … Once your trailing Max Loss reaches $100, it locks there. From that point on, it does not trail anymore." **Conflict:** the plan-page FAQ says "On Rapid Sim Funded, once your balance is +$100 above your starting balance after your first payout, the floor locks and stops trailing upward permanently." That adds an "after your first payout" condition.
- **Eval lock:** the 25K article says "Once your trailing Max Loss reaches $25,100, it locks there." No size-specific lock is published for 50K/100K/150K beyond the general EOD article.
- **Scaling conflict (50K SF):** the cross-instrument policy example says "if you are trading a Rapid 50k sim funded plan, you start off with 2 contracts (2 minis OR 20 micros) due to the scaling plan." The plan page and the 50K article say "Max Contracts: 5 minis / 50 micros", and no scaling plan is published elsewhere.
- **News:** evaluation "T1 News Trading Yes"; SF "Tier 1 News Trading Not allowed".
- **Live transition:** "$10,000 net profit achieved in a single trading day (automatic transition), or Consistent payout performance reviewed and approved by our Risk Management Team." The live DLL is not stated (null).

### 3.3 Rapid EOD (EOD SF drawdown; sizes 25K / 50K)

Sources: plan page https://myfundedfutures.com/plans/rapid-eod · help center: 25K https://help.myfundedfutures.com/en/articles/16727601-rapid-eod-25k-a-comprehensive-look · 50K https://help.myfundedfutures.com/en/articles/16158363-rapid-eod-50k-a-comprehensive-look

| Size | Eval target | Eval MLL (EOD) | DLL | Eval consistency | Eval max | Min days | SF MLL (EOD, locks +$100) | SF max | SF buffer / then per payout | Max SF accts | Live max |
|---|---|---|---|---|---|---|---|---|---|---|---|
| 25K | $1,500 | $1,000 | None (eval and SF) | 30% (Eval Only) | 2 / 20 | 4 | $1,000 | 2 / 20 | $1,100 / $250 | 3 | 2 / 20 |
| 50K | $3,000 | $2,000 | None | 30% (Eval Only) | 3 / 30 | 4 | $2,000 | 3 / 30 | $2,100 / $500 | 3 | **conflict**: 4 / 40 (plan page) vs 3 / 30 (Rapid 50k Live, which the help center says applies) |

- SF drawdown, quoted: "Your Max Loss trails upward each time your account closes at a new end-of-day high. It does not move intraday – only at the close of each trading session. … Once your trailing Max Loss reaches +$100, it locks there."
- SF news: "News trading is not allowed on this Sim Funded account." (25K) / "No T1 news trading is not allowed on this Sim Funded account." (50K, wording as published)
- Live: "Live transition on Rapid EOD 50k follows the same rules as standard Rapid 50k." The plan page lists Live "Max Loss threshold stops at $0".

### 3.4 Pro (sizes 50K / 100K / 150K; plus "Pro 50k - One Day" add-on)

Sources: plan page https://myfundedfutures.com/plans/pro · help center https://help.myfundedfutures.com/en/articles/17573707-comprehensive-look-into-redesigned-pro-plan · consistency policy https://help.myfundedfutures.com/en/articles/8694840-balancing-freedom-and-responsibility-consistency-for-sim-funded-pro-accounts-at-mffu

| Size | Eval target | Eval MLL (EOD) | DLL (eval/SF) | Eval consistency | Eval max | Min days | SF MLL (EOD) | SF lock | SF max (as published) | Buffer | Live MLL | Live max |
|---|---|---|---|---|---|---|---|---|---|---|---|---|
| 50K | $3,000 | $2,000 | None | 50% | 3 / 30 | 2 | $2,000 | "After first payout, MLL moves to $50,100 and remains static." | "3 mini / 3 micro" | $2,100 | -$2,000 (**type conflict**) | "2" (**conflict**) |
| 100K | $6,000 | $3,000 | None | 50% | 6 / 60 | 2 | $3,000 | $100,100 | "6 mini / 6 micro" | $3,100 | -$3,000 | "3" |
| 150K | $9,000 | $4,500 | None | 50% | 9 / 90 ("9 mimi / 90 micro" in the help center) | 2 | $4,500 | $150,100 | "9 mini / 9 micro" | $4,600 | -$4,500 | "4" |
| 50K One Day | $4,000 | $2,000 | None | None | 3 / 30 | 1 | then "follow the same rules as the Pro 50k" | | | | | |

- **Drawdown, quoted** (plan page FAQ): "Pro uses end-of-day trailing drawdown in both eval and sim funded. … After your first approved payout, the MLL locks permanently at your starting balance plus $100 and stops trailing entirely."
- **SF micros:** both official pages publish "3 mini / 3 micro", "6 / 6" and "9 / 9" for SF, and the plan-page size selector shows "3 mini / 3 micro contracts". This breaks the 1:10 ratio used everywhere else. It is recorded as published and marked not confirmed (it may be a typo) for the PO/user.
- **Funded consistency:** "None" as a hard rule. The plan page also says: "the Pro plan does carry a Consistent Trading Policy: MFFU monitors sim funded accounts for trading behavior that falls outside professional norms — things like maxing contracts on every trade, contract flipping, inconsistent position sizing, or high-frequency strategies not aligned with your stated approach. Violations can result in restrictions or account termination."
- **Payouts:** "14 calendar days from day of first trade. Buffer must be cleared"; min $1,000. Max "$100,000" is "(per user)" in the help center, and the plan page says both "Up to $100,000 per payout cycle" and "up to $100,000 in total Sim Funded payouts per user" (conflict, payout only).
- **Live conflicts:** the help center Live table says "Max Loss Limit (EOD Trailing)", "Daily Loss Limit None" and "Max Contracts 2 / 3 / 4" (unit not stated). The plan page FAQ says "live account parameters differ from sim funded, including a daily loss limit, reduced contract limits, and a static drawdown floor."
- **One Day add-on:** listed in the current Pro article and in the payout policy ("Pro Plan Payouts / Including 1Day Add-On"), but its own article sits in "Legacy Plans" (https://help.myfundedfutures.com/en/articles/12879226-pro-plan-1day-addon, "For a limited time"). Current availability is to be confirmed.
- **News:** evaluation T1 allowed; SF "Not allowed".

### 3.5 Studio #001 / #002 / #003 (static MLL; $10K / $20K / $5K, outside priority range)

Sources: https://myfundedfutures.com/studio · https://help.myfundedfutures.com/en/articles/17589665-studio-plans-a-comprehensive-guide

| Plan | Size | Target | MLL (static) | DLL | Consistency | Min days | Max | SF daily profit cap | Live |
|---|---|---|---|---|---|---|---|---|---|
| #001 | $10,000 | $1,000 | $500 | None | None | None | 10 micros, no minis | $5,000 | start 40% of total requested (cap $2,000), EOD trailing, locks at $100; max "Same as the plan" |
| #002 | $20,000 | $2,000 | $1,000 | None | None | None | 20 micros, no minis | $10,000 | same; max same as plan |
| #003 | $5,000 | $500 | $250 | None | None | None | 5 micros, no minis | $2,500 | same; max "1 mini / 10 micro" |

- Quoted: "Static Max Loss Limit — Your MLL is set when the account starts and never moves. There is no trailing drawdown and no end-of-day reset of the floor during Evaluation or Sim Funded." / "No Daily Loss Limit — There is no soft pause or hard daily loss limit on Studio." / platform "Hyperprop only."
- The Studio page FAQ still describes only #001/#002 in places ("Two evaluations"), while the help center adds #003 "Effective October 14th, 2026".

---

## 4. Trading-day reset

**No explicit "trading day resets at …" statement was found.** The published session times are:
- "Trades may be placed beginning at 6:00 PM EST when the Globex session opens and can remain open until the New York session closes at 4:10 PM EST." (https://help.myfundedfutures.com/en/articles/9558251-permitted-times-to-trade)
- "any open positions will be automatically closed at 4:10 PM EST on regular (non-holiday) trading days." / "Make sure that you don't force a trade multiple times on the market after 4:10 PM EST, as this will sometimes cause the order to go through and disqualify your trading account."
- Instrument list: "MyFundedFutures trading hours are between 6:00pm EST through 4:10pm EST." (https://help.myfundedfutures.com/en/articles/9735811-futures-instrument-list)
- Live FAQ: "I forgot to close my position at 3:10 pm CST or 4:10 pm EST" (https://help.myfundedfutures.com/en/articles/12109396-comprehensive-faq-live-accounts-at-myfunded-futures).
- Dashboard: "Dashboard metrics refresh daily after market close between 5:01 PM EST and 11:59 PM EST" (https://help.myfundedfutures.com/en/articles/8229980-navigating-your-dashboard).
- **Holidays:** "Holiday trading hours will be communicated to traders including closure of open positions." / "During holiday trading hours, auto-liquidation will not occur at the half-time market close, and the trader is responsible for closing the positions. … Failure to close the positions before the market closes will result in breaching of the account unless otherwise notified."
- The time zone is published as "EST" (sometimes "ET", e.g. Studio "00:00 ET" for the purchase cap). The DLL soft pause lasts "for the rest of the trading session" / "until the next session".
- YAML: `trading_day_reset: { time: null, tz_as_published: null }`, with a comment naming the 6:00 PM EST open and the 4:10 PM EST close-out. The user decides whether the preset uses 18:00 EST.

---

## 5. Trade copying stance — `conditional`

Copy article (https://help.myfundedfutures.com/en/articles/10771500-copy-trading-at-myfundedfutures), verbatim:
> "MyFunded Futures allows copy trading across all account types."
> "Tradesyncer is our preferred trade copier partner and is fully compatible with all MFFU accounts. … Millisecond-precision trade synchronization across multiple accounts · Seamless connectivity with dxFeed and Tradovate accounts"
> "Tradovate users can utilize the built-in group copier feature"
> "External copy trading solutions can also be used with your accounts"
> "By engaging in any copy trading activities, the user explicitly and irrevocably assumes full responsibility for all consequences thereof. … MyFunded Futures is not obligated to provide technical support, troubleshooting assistance, or account resets for any issues stemming from copy trading activities. … Users remain solely responsible for ensuring their copy trading activities comply with all applicable terms of service."

Fair Play, Section 4 (https://help.myfundedfutures.com/en/articles/8444599-fair-play-and-prohibited-trading-practices), verbatim:
> "Each individual trader is required to maintain their own individual trading activity. Meaning, entering, exiting and cancelling their own trade executions. Traders are not permitted to copy trade one another by entering, exiting or cancelling trade positions. Each individual trader may not use the same device (tablet, phone or computer) as used by another trader."
> Section 2: "Collaborative Trading: Collaborating with others to execute identical or opposite strategies across unconnected accounts is prohibited."

Terms 23.1.3 (https://myfundedfutures.com/terms), verbatim:
> "While Users may lawfully purchase and maintain multiple evaluation or funded accounts within the Account Limits published by the Company, all such accounts must belong to the same verified individual. It is prohibited to: … 23.1.3.3. use multiple accounts to hedge, mirror, copy, or coordinate trades in a manner that provides an unfair advantage or manipulates simulated results; 23.1.3.4. exceed the maximum number or size of accounts authorized for a User".

Hedging (https://help.myfundedfutures.com/en/articles/12011241-hedging-what-you-should-know): "MyFundedFutures prohibits hedging of any kind. Hedging is defined as a trader entering both buy and sell positions on the same underlying asset, at the same time. For example: E-Mini and Micro NQ have the same underlying asset which is NQ (Nasdaq)."

Summary:
- **Same person's accounts:** copiers are allowed "across all account types" and "across multiple accounts", and all of a user's accounts must belong to one verified individual. No sentence literally says "you may copy between your own accounts", but that is the use case the copier article describes. Copying is prohibited when it is used "to hedge, mirror, copy, or coordinate trades in a manner that provides an unfair advantage or manipulates simulated results". Opposite positions on the same underlying (e.g. NQ and MNQ) are hedging and are banned. This matters for routing: All Win must never send opposite directions on NQ/MNQ across the user's MFFU accounts.
- **Account-count limits:** see §2 (10 evals; 5 SF if only 25K/50K, 3 if any 100K/150K; plan-specific caps).
- **Other people:** copying between different traders is prohibited, as are shared devices.
- **Copying to/from other firms:** **not mentioned** on any official page.

---

## 6. Automated trading stance — `conditional` (YAML `automated_trading_allowed: conditional`)

**`automation_rule_text`** (verbatim from Fair Play "Section 1: Automated Trading Protocols", https://help.myfundedfutures.com/en/articles/8444599-fair-play-and-prohibited-trading-practices, accessed 2026-10-23; the page's invisible zero-width line-break characters are omitted):
> "High-frequency Trading is not allowed on our plans. Traders may make use of automated trading strategies tailored to their own specific settings so long as these automated tools do not aim to exploit the favorable fills offered in the Simulated Environment. Traders trading live accounts with automated trading must abide by CME guidelines."

(i) **Classification:** `conditional`. Automation is allowed, but HFT is banned and tools must not aim to exploit simulated fills.

(ii) **Named mechanisms:**
- Semi-automated trading: **not mentioned**.
- Trade copiers: **mentioned, allowed**. "Tradesyncer is our preferred trade copier partner"; "Tradovate users can utilize the built-in group copier feature"; "External copy trading solutions can also be used with your accounts" (copy article, §5).
- Webhooks / alerts: **not mentioned**.
- ATM / automatic brackets: no ATM rule. Brackets are named in the same Fair Play page: "Slippage and Bracket Usage: Exploiting the absence of slippage and utilizing tight brackets to gain from favorable fills are not permitted." Hyperprop page: "brackets link your stop and target". Volumetrica how-to: "configure Multi OCO or Single OCO settings for order brackets". Brackets are allowed as order tools, and only tight-bracket fill exploitation is banned.
- Related prohibited practices on the same page: "Simultaneously placing multiple limit orders at the same price to manipulate order fills is prohibited."; "Gapped or Illiquid Market Trading: Initiating trades to profit from isolated fills in these markets is not allowed."
- MyFundedClub (rewards only, not trading): points may be revoked if "earned through fraudulent activity, automation, abuse" (https://help.myfundedfutures.com/en/articles/16130843-myfundedclub-membership-perks-and-rewards).

(iii) **By phase:**
- The rule text applies to "our plans" (evaluation and Sim Funded).
- Live adds: "Traders trading live accounts with automated trading must abide by CME guidelines." The live FAQ says "Yes, microscalping is permitted on the live accounts" and "there is nothing else we prohibit, but you must abide by the CME Rules."
- Pro Sim Funded's discretionary "Consistent Trading Policy" names "high-frequency strategies not aligned with your stated approach".
- No other phase difference is published.

---

## 7. Other rules that affect routing or risk

- **News (Tier 1)** (https://help.myfundedfutures.com/en/articles/8230009-news-trading-policy):
  - "No positions or orders (including limit orders) can be open 2 minutes before and after the release. All positions must be flattened at least two minutes prior to the scheduled T1 News event."
  - "Restricted Accounts: … Rapid Sim Funded · Pro Sim Funded. Unrestricted Accounts: … All evaluations · Builder Plans."
  - Rapid EOD Sim Funded: "Not allowed" (plan page and help center).
  - T1 events: "FOMC Meetings, FOMC Minutes, Employment Report, CPI" (+ EIA for energy, agricultural reports).
  - All accounts: "Ensuring no open positions or orders are active in the order book 2 minutes before and after any data release" under "Standard Trading Protocols … These protocols apply to all news releases." That wording is broader than the T1 list (flag for the PO). Live: "Yes, you can trade news."
- **2% price-limit rule** (Sim Funded and Live) (https://help.myfundedfutures.com/en/articles/9698984-2-price-limit-rule): "we do not allow market participation when a product is trading within 2% of a price limit." The firm publishes its formula: "Stop trading when market priced above: 18556 x (1 + 0.07 - 0.02) = 19483".
- **Cross-instrument:** "Orders exceeding the allowed contract size for a single instrument … will be automatically rejected"; combining minis and micros over the limit "is not permitted" and "can lead to an account breach".
- **Hedging:** prohibited "of any kind", sim and live (§5).
- **Overnight:** not allowed; auto-close at 4:10 PM EST (sim breach risk on Rapid: "Holding overnight will breach the account."). Live: positions are auto-closed and "We do not breach our live traders for this."
- **Inactivity:** 7 consecutive calendar days, evaluation and SF (one-time-payment accounts); none on live.
- **Account owner only:** "all accounts participating in our challenges are exclusively traded by the account owner" (https://help.myfundedfutures.com/en/articles/8230018-can-someone-else-trade-my-account).
- **Payout request freezes trading (live):** "your available contracts will be set to 0 until the payout is approved and paid."
- **Sim and live at once:** "once you have a live account, you cannot trade on the sim and live environment simultaneously."
- **Mandatory stop loss / min-max trade duration / trade count:** none published. Live FAQ: "there are no limits to the maximum or minimum number of trades".
- **Generic live article** (older wording, plan pages override it): "Static Drawdown", "Customizable Daily Loss Limit", "Scaling Opportunities". Live FAQ: "The max drawdown is static and is set at $140 as the minimum balance." These conflict with the plan-specific EOD-trailing live parameters and are recorded as a gap.

### 7a. VPS / server / VPN and personal device (format rev. 3)

- **VPS / server / VPN: not mentioned.** Searched: Fair Play and Prohibited Trading Practices, Can someone else trade my account?, Copy Trading, Terms, Privacy Policy, Restricted Countries, KYC/AML, Live FAQs, the platform login articles, and a keyword search ("VPS", "VPN", "virtual", "proxy", "server", "IP address") over every captured page. `vps_allowed: unknown`.
- **Personal device:** no rule says orders must come from the trader's own personal device. The rule found bans **sharing** a device with another trader:
  > "Each individual trader may not use the same device (tablet, phone or computer) as used by another trader. If this is not respected, then it could lead to permanent restrictions from using our services" (https://help.myfundedfutures.com/en/articles/8444599-fair-play-and-prohibited-trading-practices, accessed 2026-10-23)
  > "The use of shared devices is strictly forbidden and may result in a breach and/or account failure. E.g.: using the same PC or Phone as your partner to make use of our services and trading accounts." (https://help.myfundedfutures.com/en/articles/8230018-can-someone-else-trade-my-account, accessed 2026-10-23)
  
  **R4 re-check (2026-10-23).** The help-center sitemap (77 article pages) and the website Terms (https://myfundedfutures.com/terms) and Privacy Policy were re-fetched with curl and grepped for VPS, VPN, virtual private, remote desktop/access, IP address, proxy, cloud, server, same/own/personal/shared device, on behalf. No text mentions VPS, VPN, IP or remote access, and no text requires the trader's own device or permits other devices. Related text, not a device rule: Terms §23.1.3.2 lists as a violation "operate or trade on behalf of another person's account, or permit another person to operate or trade on your account;" (https://myfundedfutures.com/terms, accessed 2026-10-23), and "Each individual trader may not use the same device (tablet, phone or computer) as used by another trader." (https://help.myfundedfutures.com/en/articles/8444599-fair-play-and-prohibited-trading-practices, accessed 2026-10-23). Both fields stay `unknown`.

  `personal_device_required: unknown`. A shared VPS/server used by more than one trader would plausibly fall under "shared devices", but that is an inference, so the user/PO decides. The copy article also endorses "Tradesyncer … the #1 cloud-based futures trading copier", a server-side tool.

---

## 8. Machine-readable blocks

Common to all blocks: `copy_trading_allowed: conditional`, `automated_trading_allowed: conditional`, `vps_allowed: unknown`, `personal_device_required: unknown`, the same `automation_rule_text`, and `trading_day_reset` null (§4). `max_position` uses 0 minis where the firm says "no minis". `daily_loss_limit.kind: null` means not confirmed or not found.

#### Builder — evaluation — 25K

```yaml
firm: myfundedfutures
plan: "Builder"
phase: evaluation
size_label: "25K"
size_usd: "25000.00"
instruments: [MNQ, NQ]
platforms: [Tradovate, NinjaTrader, TradingView, Quantower, ATAS, DeepChart/DeepDom, Volumetrica, Hyperprop]
drawdown: { amount_usd: "1000.00", type: trailing_eod, stops_trailing_at: "MLL locks permanently once it reaches $100 above the starting balance (help-center Account Rules)" }
daily_loss_limit: { amount_usd: "600.00", kind: soft_pause }
profit_target_usd: "1500.00"
consistency_rule: { text: "None in evaluation", threshold_pct: null }
max_position: { minis: 2, micros: 20, scaling: "No scaling plan published; fixed max contracts" }
min_trading_days: 1
trading_day_reset: { time: null, tz_as_published: null }   # no explicit reset published; session "6:00 PM EST" open, positions auto-closed "4:10 PM EST" (see §4)
copy_trading_allowed: conditional   # copiers allowed on all account types; copying other traders and multi-account hedge/mirror/copy for unfair advantage prohibited (see §5)
automated_trading_allowed: conditional   # allowed unless HFT or aimed at exploiting sim fills; live must follow CME guidelines (see §6)
automation_rule_text: "High-frequency Trading is not allowed on our plans. Traders may make use of automated trading strategies tailored to their own specific settings so long as these automated tools do not aim to exploit the favorable fills offered in the Simulated Environment. Traders trading live accounts with automated trading must abide by CME guidelines."
vps_allowed: unknown   # VPS/server/VPN not mentioned on any official page (see §7a)
personal_device_required: unknown   # not required in words; shared devices with ANOTHER trader are forbidden (see §7a)
notes: "DLL conflict: plan page + help center say soft pause (trading stops for the rest of the session, account not breached); payout-policy article says 'No daily loss limits on Builder, Rapid or Pro plans'. Min-days conflict: plan-page table '1 Day' vs plan-page stage card 'One phase · min 2 days'. News trading allowed. Inactivity 7 calendar days."
status: conflict
not_confirmed_fields: [daily_loss_limit, min_trading_days]
sources:
  - { field: drawdown, url: "https://help.myfundedfutures.com/en/articles/17036130-builder-plan-25k-a-comprehensive-look", accessed: "2026-10-23" }
  - { field: drawdown, url: "https://myfundedfutures.com/plans/builder", accessed: "2026-10-23" }
  - { field: daily_loss_limit, url: "https://help.myfundedfutures.com/en/articles/17036130-builder-plan-25k-a-comprehensive-look", accessed: "2026-10-23" }
  - { field: daily_loss_limit, url: "https://myfundedfutures.com/plans/builder", accessed: "2026-10-23" }
  - { field: daily_loss_limit, url: "https://help.myfundedfutures.com/en/articles/13745661-payout-policy-overview-best-and-fastest-prop-firm-payouts", accessed: "2026-10-23" }
  - { field: profit_target_usd, url: "https://help.myfundedfutures.com/en/articles/17036130-builder-plan-25k-a-comprehensive-look", accessed: "2026-10-23" }
  - { field: consistency_rule, url: "https://help.myfundedfutures.com/en/articles/11994562-consistency-rule-at-my-fundedfutures", accessed: "2026-10-23" }
  - { field: max_position, url: "https://help.myfundedfutures.com/en/articles/17036130-builder-plan-25k-a-comprehensive-look", accessed: "2026-10-23" }
  - { field: max_position, url: "https://myfundedfutures.com/plans/builder", accessed: "2026-10-23" }
  - { field: min_trading_days, url: "https://myfundedfutures.com/plans/builder", accessed: "2026-10-23" }
  - { field: min_trading_days, url: "https://help.myfundedfutures.com/en/articles/17036130-builder-plan-25k-a-comprehensive-look", accessed: "2026-10-23" }
  - { field: copy_trading_allowed, url: "https://help.myfundedfutures.com/en/articles/10771500-copy-trading-at-myfundedfutures", accessed: "2026-10-23" }
  - { field: copy_trading_allowed, url: "https://myfundedfutures.com/terms", accessed: "2026-10-23" }
  - { field: automated_trading_allowed, url: "https://help.myfundedfutures.com/en/articles/8444599-fair-play-and-prohibited-trading-practices", accessed: "2026-10-23" }
  - { field: trading_day_reset, url: "https://help.myfundedfutures.com/en/articles/9558251-permitted-times-to-trade", accessed: "2026-10-23" }
  - { field: platforms, url: "https://help.myfundedfutures.com/en/articles/8528335-overview-of-supported-platforms-at-mffu", accessed: "2026-10-23" }
```

#### Builder — funded — 25K

```yaml
firm: myfundedfutures
plan: "Builder"
phase: funded
size_label: "25K"
size_usd: "25000.00"
instruments: [MNQ, NQ]
platforms: [Tradovate, NinjaTrader, TradingView, Quantower, ATAS, DeepChart/DeepDom, Volumetrica, Hyperprop]
drawdown: { amount_usd: "1000.00", type: trailing_eod, stops_trailing_at: "Sim Funded starts at $0; plan page 'MLL Lock Level $100'; help center 'locks permanently once it reaches $100 above the starting balance'; payout-policy article 'After your first payout, the Max Loss Limit resets to $100'" }
daily_loss_limit: { amount_usd: "600.00", kind: soft_pause }
profit_target_usd: null
consistency_rule: { text: "Payout stage: single largest profit day cannot represent more than 50% of total profits in the cycle; resets after each approved payout", threshold_pct: "50" }
max_position: { minis: 2, micros: 20, scaling: "No scaling plan published; fixed max contracts" }
min_trading_days: null
trading_day_reset: { time: null, tz_as_published: null }   # no explicit reset published; session "6:00 PM EST" open, positions auto-closed "4:10 PM EST" (see §4)
copy_trading_allowed: conditional   # copiers allowed on all account types; copying other traders and multi-account hedge/mirror/copy for unfair advantage prohibited (see §5)
automated_trading_allowed: conditional   # allowed unless HFT or aimed at exploiting sim fills; live must follow CME guidelines (see §6)
automation_rule_text: "High-frequency Trading is not allowed on our plans. Traders may make use of automated trading strategies tailored to their own specific settings so long as these automated tools do not aim to exploit the favorable fills offered in the Simulated Environment. Traders trading live accounts with automated trading must abide by CME guidelines."
vps_allowed: unknown   # VPS/server/VPN not mentioned on any official page (see §7a)
personal_device_required: unknown   # not required in words; shared devices with ANOTHER trader are forbidden (see §7a)
notes: "Buffer 1100.00 must be cleared before any payout; min 2 qualifying days per payout cycle; max 5 sim payouts then Live. Active accounts: conflict - plan-page table/help center/limits article '2 Builder 25K per user' vs plan-page FAQ 'one active Builder account per user'. DLL conflict as in evaluation. News allowed. Inactivity 7 calendar days."
status: conflict
not_confirmed_fields: [daily_loss_limit, drawdown.stops_trailing_at, max_accounts]
sources:
  - { field: drawdown, url: "https://help.myfundedfutures.com/en/articles/17036130-builder-plan-25k-a-comprehensive-look", accessed: "2026-10-23" }
  - { field: drawdown, url: "https://myfundedfutures.com/plans/builder", accessed: "2026-10-23" }
  - { field: drawdown, url: "https://help.myfundedfutures.com/en/articles/13745661-payout-policy-overview-best-and-fastest-prop-firm-payouts", accessed: "2026-10-23" }
  - { field: daily_loss_limit, url: "https://help.myfundedfutures.com/en/articles/17036130-builder-plan-25k-a-comprehensive-look", accessed: "2026-10-23" }
  - { field: daily_loss_limit, url: "https://help.myfundedfutures.com/en/articles/13745661-payout-policy-overview-best-and-fastest-prop-firm-payouts", accessed: "2026-10-23" }
  - { field: consistency_rule, url: "https://help.myfundedfutures.com/en/articles/17036130-builder-plan-25k-a-comprehensive-look", accessed: "2026-10-23" }
  - { field: max_position, url: "https://help.myfundedfutures.com/en/articles/17036130-builder-plan-25k-a-comprehensive-look", accessed: "2026-10-23" }
  - { field: max_position, url: "https://myfundedfutures.com/plans/builder", accessed: "2026-10-23" }
  - { field: max_accounts, url: "https://myfundedfutures.com/plans/builder", accessed: "2026-10-23" }
  - { field: max_accounts, url: "https://help.myfundedfutures.com/en/articles/16498635-moving-from-evaluation-to-sim-funded-account", accessed: "2026-10-23" }
  - { field: copy_trading_allowed, url: "https://help.myfundedfutures.com/en/articles/10771500-copy-trading-at-myfundedfutures", accessed: "2026-10-23" }
  - { field: copy_trading_allowed, url: "https://myfundedfutures.com/terms", accessed: "2026-10-23" }
  - { field: automated_trading_allowed, url: "https://help.myfundedfutures.com/en/articles/8444599-fair-play-and-prohibited-trading-practices", accessed: "2026-10-23" }
  - { field: trading_day_reset, url: "https://help.myfundedfutures.com/en/articles/9558251-permitted-times-to-trade", accessed: "2026-10-23" }
  - { field: platforms, url: "https://help.myfundedfutures.com/en/articles/8528335-overview-of-supported-platforms-at-mffu", accessed: "2026-10-23" }
```

#### Builder — live — 25K

```yaml
firm: myfundedfutures
plan: "Builder"
phase: live
size_label: "25K"
size_usd: "25000.00"
instruments: [MNQ, NQ]
platforms: []   # live platform not published
drawdown: { amount_usd: "1000.00", type: trailing_eod, stops_trailing_at: "Static once MLL reaches $0 (plan page and help center)" }
daily_loss_limit: { amount_usd: null, kind: null }
profit_target_usd: null
consistency_rule: { text: "None (Live)", threshold_pct: null }
max_position: { minis: null, micros: null, scaling: "Live: no set scaling plan; limits reviewed with risk manager (live FAQ)" }
min_trading_days: null
trading_day_reset: { time: null, tz_as_published: null }   # no explicit reset published; session "6:00 PM EST" open, positions auto-closed "4:10 PM EST" (see §4)
copy_trading_allowed: conditional   # copiers allowed on all account types; copying other traders and multi-account hedge/mirror/copy for unfair advantage prohibited (see §5)
automated_trading_allowed: conditional   # allowed unless HFT or aimed at exploiting sim fills; live must follow CME guidelines (see §6)
automation_rule_text: "High-frequency Trading is not allowed on our plans. Traders may make use of automated trading strategies tailored to their own specific settings so long as these automated tools do not aim to exploit the favorable fills offered in the Simulated Environment. Traders trading live accounts with automated trading must abide by CME guidelines."
vps_allowed: unknown   # VPS/server/VPN not mentioned on any official page (see §7a)
personal_device_required: unknown   # not required in words; shared devices with ANOTHER trader are forbidden (see §7a)
notes: "Live DLL conflict: plan page '$600' vs help center 'None'. Live max contracts not published. Promotion after 5th approved sim payout; Live at Blue Row Capital; max 1 live account; 21-day cooldown after live MLL breach. Live trading is outside All Win agent scope (user-only)."
status: conflict
not_confirmed_fields: [daily_loss_limit]
sources:
  - { field: drawdown, url: "https://help.myfundedfutures.com/en/articles/17036130-builder-plan-25k-a-comprehensive-look", accessed: "2026-10-23" }
  - { field: drawdown, url: "https://myfundedfutures.com/plans/builder", accessed: "2026-10-23" }
  - { field: daily_loss_limit, url: "https://help.myfundedfutures.com/en/articles/17036130-builder-plan-25k-a-comprehensive-look", accessed: "2026-10-23" }
  - { field: daily_loss_limit, url: "https://myfundedfutures.com/plans/builder", accessed: "2026-10-23" }
  - { field: max_position, url: "https://help.myfundedfutures.com/en/articles/17036130-builder-plan-25k-a-comprehensive-look", accessed: "2026-10-23" }
  - { field: copy_trading_allowed, url: "https://help.myfundedfutures.com/en/articles/10771500-copy-trading-at-myfundedfutures", accessed: "2026-10-23" }
  - { field: copy_trading_allowed, url: "https://myfundedfutures.com/terms", accessed: "2026-10-23" }
  - { field: automated_trading_allowed, url: "https://help.myfundedfutures.com/en/articles/8444599-fair-play-and-prohibited-trading-practices", accessed: "2026-10-23" }
  - { field: trading_day_reset, url: "https://help.myfundedfutures.com/en/articles/9558251-permitted-times-to-trade", accessed: "2026-10-23" }
  - { field: platforms, url: "https://help.myfundedfutures.com/en/articles/8528335-overview-of-supported-platforms-at-mffu", accessed: "2026-10-23" }
```

#### Builder — evaluation — 50K

```yaml
firm: myfundedfutures
plan: "Builder"
phase: evaluation
size_label: "50K"
size_usd: "50000.00"
instruments: [MNQ, NQ]
platforms: [Tradovate, NinjaTrader, TradingView, Quantower, ATAS, DeepChart/DeepDom, Volumetrica, Hyperprop]
drawdown: { amount_usd: "2000.00", type: trailing_eod, stops_trailing_at: "MLL locks permanently once it reaches $100 above the starting balance (help-center Account Rules)" }
daily_loss_limit: { amount_usd: "1000.00", kind: soft_pause }
profit_target_usd: "3000.00"
consistency_rule: { text: "None in evaluation", threshold_pct: null }
max_position: { minis: 4, micros: 40, scaling: "No scaling plan published; fixed max contracts" }
min_trading_days: 1
trading_day_reset: { time: null, tz_as_published: null }   # no explicit reset published; session "6:00 PM EST" open, positions auto-closed "4:10 PM EST" (see §4)
copy_trading_allowed: conditional   # copiers allowed on all account types; copying other traders and multi-account hedge/mirror/copy for unfair advantage prohibited (see §5)
automated_trading_allowed: conditional   # allowed unless HFT or aimed at exploiting sim fills; live must follow CME guidelines (see §6)
automation_rule_text: "High-frequency Trading is not allowed on our plans. Traders may make use of automated trading strategies tailored to their own specific settings so long as these automated tools do not aim to exploit the favorable fills offered in the Simulated Environment. Traders trading live accounts with automated trading must abide by CME guidelines."
vps_allowed: unknown   # VPS/server/VPN not mentioned on any official page (see §7a)
personal_device_required: unknown   # not required in words; shared devices with ANOTHER trader are forbidden (see §7a)
notes: "DLL conflict: plan page + help center say soft pause (trading stops for the rest of the session, account not breached); payout-policy article says 'No daily loss limits on Builder, Rapid or Pro plans'. Min-days conflict: plan-page table '1 Day' vs plan-page stage card 'One phase · min 2 days'. News trading allowed. Inactivity 7 calendar days. Add-on variant exists for 50K (see separate block)."
status: conflict
not_confirmed_fields: [daily_loss_limit, min_trading_days]
sources:
  - { field: drawdown, url: "https://help.myfundedfutures.com/en/articles/14290805-builder-plan-50k-a-comprehensive-guide", accessed: "2026-10-23" }
  - { field: drawdown, url: "https://myfundedfutures.com/plans/builder", accessed: "2026-10-23" }
  - { field: daily_loss_limit, url: "https://help.myfundedfutures.com/en/articles/14290805-builder-plan-50k-a-comprehensive-guide", accessed: "2026-10-23" }
  - { field: daily_loss_limit, url: "https://myfundedfutures.com/plans/builder", accessed: "2026-10-23" }
  - { field: daily_loss_limit, url: "https://help.myfundedfutures.com/en/articles/13745661-payout-policy-overview-best-and-fastest-prop-firm-payouts", accessed: "2026-10-23" }
  - { field: profit_target_usd, url: "https://help.myfundedfutures.com/en/articles/14290805-builder-plan-50k-a-comprehensive-guide", accessed: "2026-10-23" }
  - { field: consistency_rule, url: "https://help.myfundedfutures.com/en/articles/11994562-consistency-rule-at-my-fundedfutures", accessed: "2026-10-23" }
  - { field: max_position, url: "https://help.myfundedfutures.com/en/articles/14290805-builder-plan-50k-a-comprehensive-guide", accessed: "2026-10-23" }
  - { field: max_position, url: "https://myfundedfutures.com/plans/builder", accessed: "2026-10-23" }
  - { field: min_trading_days, url: "https://myfundedfutures.com/plans/builder", accessed: "2026-10-23" }
  - { field: min_trading_days, url: "https://help.myfundedfutures.com/en/articles/14290805-builder-plan-50k-a-comprehensive-guide", accessed: "2026-10-23" }
  - { field: copy_trading_allowed, url: "https://help.myfundedfutures.com/en/articles/10771500-copy-trading-at-myfundedfutures", accessed: "2026-10-23" }
  - { field: copy_trading_allowed, url: "https://myfundedfutures.com/terms", accessed: "2026-10-23" }
  - { field: automated_trading_allowed, url: "https://help.myfundedfutures.com/en/articles/8444599-fair-play-and-prohibited-trading-practices", accessed: "2026-10-23" }
  - { field: trading_day_reset, url: "https://help.myfundedfutures.com/en/articles/9558251-permitted-times-to-trade", accessed: "2026-10-23" }
  - { field: platforms, url: "https://help.myfundedfutures.com/en/articles/8528335-overview-of-supported-platforms-at-mffu", accessed: "2026-10-23" }
```

#### Builder — funded — 50K

```yaml
firm: myfundedfutures
plan: "Builder"
phase: funded
size_label: "50K"
size_usd: "50000.00"
instruments: [MNQ, NQ]
platforms: [Tradovate, NinjaTrader, TradingView, Quantower, ATAS, DeepChart/DeepDom, Volumetrica, Hyperprop]
drawdown: { amount_usd: "2000.00", type: trailing_eod, stops_trailing_at: "Sim Funded starts at $0; plan page 'MLL Lock Level $100'; help center 'locks permanently once it reaches $100 above the starting balance'; payout-policy article 'After your first payout, the Max Loss Limit resets to $100'" }
daily_loss_limit: { amount_usd: "1000.00", kind: soft_pause }
profit_target_usd: null
consistency_rule: { text: "Payout stage: single largest profit day cannot represent more than 50% of total profits in the cycle; resets after each approved payout", threshold_pct: "50" }
max_position: { minis: 4, micros: 40, scaling: "No scaling plan published; fixed max contracts" }
min_trading_days: null
trading_day_reset: { time: null, tz_as_published: null }   # no explicit reset published; session "6:00 PM EST" open, positions auto-closed "4:10 PM EST" (see §4)
copy_trading_allowed: conditional   # copiers allowed on all account types; copying other traders and multi-account hedge/mirror/copy for unfair advantage prohibited (see §5)
automated_trading_allowed: conditional   # allowed unless HFT or aimed at exploiting sim fills; live must follow CME guidelines (see §6)
automation_rule_text: "High-frequency Trading is not allowed on our plans. Traders may make use of automated trading strategies tailored to their own specific settings so long as these automated tools do not aim to exploit the favorable fills offered in the Simulated Environment. Traders trading live accounts with automated trading must abide by CME guidelines."
vps_allowed: unknown   # VPS/server/VPN not mentioned on any official page (see §7a)
personal_device_required: unknown   # not required in words; shared devices with ANOTHER trader are forbidden (see §7a)
notes: "Buffer 2100.00 must be cleared before any payout; min 2 qualifying days per payout cycle; max 5 sim payouts then Live. Active accounts: 1 per user. DLL conflict as in evaluation. News allowed. Inactivity 7 calendar days."
status: conflict
not_confirmed_fields: [daily_loss_limit, drawdown.stops_trailing_at]
sources:
  - { field: drawdown, url: "https://help.myfundedfutures.com/en/articles/14290805-builder-plan-50k-a-comprehensive-guide", accessed: "2026-10-23" }
  - { field: drawdown, url: "https://myfundedfutures.com/plans/builder", accessed: "2026-10-23" }
  - { field: drawdown, url: "https://help.myfundedfutures.com/en/articles/13745661-payout-policy-overview-best-and-fastest-prop-firm-payouts", accessed: "2026-10-23" }
  - { field: daily_loss_limit, url: "https://help.myfundedfutures.com/en/articles/14290805-builder-plan-50k-a-comprehensive-guide", accessed: "2026-10-23" }
  - { field: daily_loss_limit, url: "https://help.myfundedfutures.com/en/articles/13745661-payout-policy-overview-best-and-fastest-prop-firm-payouts", accessed: "2026-10-23" }
  - { field: consistency_rule, url: "https://help.myfundedfutures.com/en/articles/14290805-builder-plan-50k-a-comprehensive-guide", accessed: "2026-10-23" }
  - { field: max_position, url: "https://help.myfundedfutures.com/en/articles/14290805-builder-plan-50k-a-comprehensive-guide", accessed: "2026-10-23" }
  - { field: max_position, url: "https://myfundedfutures.com/plans/builder", accessed: "2026-10-23" }
  - { field: max_accounts, url: "https://myfundedfutures.com/plans/builder", accessed: "2026-10-23" }
  - { field: max_accounts, url: "https://help.myfundedfutures.com/en/articles/16498635-moving-from-evaluation-to-sim-funded-account", accessed: "2026-10-23" }
  - { field: copy_trading_allowed, url: "https://help.myfundedfutures.com/en/articles/10771500-copy-trading-at-myfundedfutures", accessed: "2026-10-23" }
  - { field: copy_trading_allowed, url: "https://myfundedfutures.com/terms", accessed: "2026-10-23" }
  - { field: automated_trading_allowed, url: "https://help.myfundedfutures.com/en/articles/8444599-fair-play-and-prohibited-trading-practices", accessed: "2026-10-23" }
  - { field: trading_day_reset, url: "https://help.myfundedfutures.com/en/articles/9558251-permitted-times-to-trade", accessed: "2026-10-23" }
  - { field: platforms, url: "https://help.myfundedfutures.com/en/articles/8528335-overview-of-supported-platforms-at-mffu", accessed: "2026-10-23" }
```

#### Builder — live — 50K

```yaml
firm: myfundedfutures
plan: "Builder"
phase: live
size_label: "50K"
size_usd: "50000.00"
instruments: [MNQ, NQ]
platforms: []   # live platform not published
drawdown: { amount_usd: "2000.00", type: trailing_eod, stops_trailing_at: "Static once MLL reaches $0 (plan page and help center)" }
daily_loss_limit: { amount_usd: "1000.00", kind: soft_pause }
profit_target_usd: null
consistency_rule: { text: "None (Live)", threshold_pct: null }
max_position: { minis: null, micros: null, scaling: "Live: no set scaling plan; limits reviewed with risk manager (live FAQ)" }
min_trading_days: null
trading_day_reset: { time: null, tz_as_published: null }   # no explicit reset published; session "6:00 PM EST" open, positions auto-closed "4:10 PM EST" (see §4)
copy_trading_allowed: conditional   # copiers allowed on all account types; copying other traders and multi-account hedge/mirror/copy for unfair advantage prohibited (see §5)
automated_trading_allowed: conditional   # allowed unless HFT or aimed at exploiting sim fills; live must follow CME guidelines (see §6)
automation_rule_text: "High-frequency Trading is not allowed on our plans. Traders may make use of automated trading strategies tailored to their own specific settings so long as these automated tools do not aim to exploit the favorable fills offered in the Simulated Environment. Traders trading live accounts with automated trading must abide by CME guidelines."
vps_allowed: unknown   # VPS/server/VPN not mentioned on any official page (see §7a)
personal_device_required: unknown   # not required in words; shared devices with ANOTHER trader are forbidden (see §7a)
notes: "Live DLL $1,000; help-center FAQ says the Evaluation, Sim Funded and Live stage have a $1,000 soft pause DLL. Live max contracts not published. Promotion after 5th approved sim payout; Live at Blue Row Capital; max 1 live account; 21-day cooldown after live MLL breach. Live trading is outside All Win agent scope (user-only)."
status: partial
not_confirmed_fields: []
sources:
  - { field: drawdown, url: "https://help.myfundedfutures.com/en/articles/14290805-builder-plan-50k-a-comprehensive-guide", accessed: "2026-10-23" }
  - { field: drawdown, url: "https://myfundedfutures.com/plans/builder", accessed: "2026-10-23" }
  - { field: daily_loss_limit, url: "https://help.myfundedfutures.com/en/articles/14290805-builder-plan-50k-a-comprehensive-guide", accessed: "2026-10-23" }
  - { field: daily_loss_limit, url: "https://myfundedfutures.com/plans/builder", accessed: "2026-10-23" }
  - { field: max_position, url: "https://help.myfundedfutures.com/en/articles/14290805-builder-plan-50k-a-comprehensive-guide", accessed: "2026-10-23" }
  - { field: copy_trading_allowed, url: "https://help.myfundedfutures.com/en/articles/10771500-copy-trading-at-myfundedfutures", accessed: "2026-10-23" }
  - { field: copy_trading_allowed, url: "https://myfundedfutures.com/terms", accessed: "2026-10-23" }
  - { field: automated_trading_allowed, url: "https://help.myfundedfutures.com/en/articles/8444599-fair-play-and-prohibited-trading-practices", accessed: "2026-10-23" }
  - { field: trading_day_reset, url: "https://help.myfundedfutures.com/en/articles/9558251-permitted-times-to-trade", accessed: "2026-10-23" }
  - { field: platforms, url: "https://help.myfundedfutures.com/en/articles/8528335-overview-of-supported-platforms-at-mffu", accessed: "2026-10-23" }
```

#### Builder — evaluation — 100K

```yaml
firm: myfundedfutures
plan: "Builder"
phase: evaluation
size_label: "100K"
size_usd: "100000.00"
instruments: [MNQ, NQ]
platforms: [Tradovate, NinjaTrader, TradingView, Quantower, ATAS, DeepChart/DeepDom, Volumetrica, Hyperprop]
drawdown: { amount_usd: "3000.00", type: trailing_eod, stops_trailing_at: "MLL locks permanently once it reaches $100 above the starting balance (help-center Account Rules)" }
daily_loss_limit: { amount_usd: "1750.00", kind: soft_pause }
profit_target_usd: "6000.00"
consistency_rule: { text: "None in evaluation", threshold_pct: null }
max_position: { minis: 6, micros: 60, scaling: "No scaling plan published; fixed max contracts" }
min_trading_days: 1
trading_day_reset: { time: null, tz_as_published: null }   # no explicit reset published; session "6:00 PM EST" open, positions auto-closed "4:10 PM EST" (see §4)
copy_trading_allowed: conditional   # copiers allowed on all account types; copying other traders and multi-account hedge/mirror/copy for unfair advantage prohibited (see §5)
automated_trading_allowed: conditional   # allowed unless HFT or aimed at exploiting sim fills; live must follow CME guidelines (see §6)
automation_rule_text: "High-frequency Trading is not allowed on our plans. Traders may make use of automated trading strategies tailored to their own specific settings so long as these automated tools do not aim to exploit the favorable fills offered in the Simulated Environment. Traders trading live accounts with automated trading must abide by CME guidelines."
vps_allowed: unknown   # VPS/server/VPN not mentioned on any official page (see §7a)
personal_device_required: unknown   # not required in words; shared devices with ANOTHER trader are forbidden (see §7a)
notes: "DLL conflict: plan page + help center say soft pause (trading stops for the rest of the session, account not breached); payout-policy article says 'No daily loss limits on Builder, Rapid or Pro plans'. Min-days conflict: plan-page table '1 Day' vs plan-page stage card 'One phase · min 2 days'. News trading allowed. Inactivity 7 calendar days."
status: conflict
not_confirmed_fields: [daily_loss_limit, min_trading_days]
sources:
  - { field: drawdown, url: "https://help.myfundedfutures.com/en/articles/17005296-builder-plan-100k-comprehensive-guide", accessed: "2026-10-23" }
  - { field: drawdown, url: "https://myfundedfutures.com/plans/builder", accessed: "2026-10-23" }
  - { field: daily_loss_limit, url: "https://help.myfundedfutures.com/en/articles/17005296-builder-plan-100k-comprehensive-guide", accessed: "2026-10-23" }
  - { field: daily_loss_limit, url: "https://myfundedfutures.com/plans/builder", accessed: "2026-10-23" }
  - { field: daily_loss_limit, url: "https://help.myfundedfutures.com/en/articles/13745661-payout-policy-overview-best-and-fastest-prop-firm-payouts", accessed: "2026-10-23" }
  - { field: profit_target_usd, url: "https://help.myfundedfutures.com/en/articles/17005296-builder-plan-100k-comprehensive-guide", accessed: "2026-10-23" }
  - { field: consistency_rule, url: "https://help.myfundedfutures.com/en/articles/11994562-consistency-rule-at-my-fundedfutures", accessed: "2026-10-23" }
  - { field: max_position, url: "https://help.myfundedfutures.com/en/articles/17005296-builder-plan-100k-comprehensive-guide", accessed: "2026-10-23" }
  - { field: max_position, url: "https://myfundedfutures.com/plans/builder", accessed: "2026-10-23" }
  - { field: min_trading_days, url: "https://myfundedfutures.com/plans/builder", accessed: "2026-10-23" }
  - { field: min_trading_days, url: "https://help.myfundedfutures.com/en/articles/17005296-builder-plan-100k-comprehensive-guide", accessed: "2026-10-23" }
  - { field: copy_trading_allowed, url: "https://help.myfundedfutures.com/en/articles/10771500-copy-trading-at-myfundedfutures", accessed: "2026-10-23" }
  - { field: copy_trading_allowed, url: "https://myfundedfutures.com/terms", accessed: "2026-10-23" }
  - { field: automated_trading_allowed, url: "https://help.myfundedfutures.com/en/articles/8444599-fair-play-and-prohibited-trading-practices", accessed: "2026-10-23" }
  - { field: trading_day_reset, url: "https://help.myfundedfutures.com/en/articles/9558251-permitted-times-to-trade", accessed: "2026-10-23" }
  - { field: platforms, url: "https://help.myfundedfutures.com/en/articles/8528335-overview-of-supported-platforms-at-mffu", accessed: "2026-10-23" }
```

#### Builder — funded — 100K

```yaml
firm: myfundedfutures
plan: "Builder"
phase: funded
size_label: "100K"
size_usd: "100000.00"
instruments: [MNQ, NQ]
platforms: [Tradovate, NinjaTrader, TradingView, Quantower, ATAS, DeepChart/DeepDom, Volumetrica, Hyperprop]
drawdown: { amount_usd: "3000.00", type: trailing_eod, stops_trailing_at: "Sim Funded starts at $0; plan page 'MLL Lock Level $100'; help center 'locks permanently once it reaches $100 above the starting balance'; payout-policy article 'After your first payout, the Max Loss Limit resets to $100'" }
daily_loss_limit: { amount_usd: "1750.00", kind: soft_pause }
profit_target_usd: null
consistency_rule: { text: "Payout stage: single largest profit day cannot represent more than 50% of total profits in the cycle; resets after each approved payout", threshold_pct: "50" }
max_position: { minis: 6, micros: 60, scaling: "No scaling plan published; fixed max contracts" }
min_trading_days: null
trading_day_reset: { time: null, tz_as_published: null }   # no explicit reset published; session "6:00 PM EST" open, positions auto-closed "4:10 PM EST" (see §4)
copy_trading_allowed: conditional   # copiers allowed on all account types; copying other traders and multi-account hedge/mirror/copy for unfair advantage prohibited (see §5)
automated_trading_allowed: conditional   # allowed unless HFT or aimed at exploiting sim fills; live must follow CME guidelines (see §6)
automation_rule_text: "High-frequency Trading is not allowed on our plans. Traders may make use of automated trading strategies tailored to their own specific settings so long as these automated tools do not aim to exploit the favorable fills offered in the Simulated Environment. Traders trading live accounts with automated trading must abide by CME guidelines."
vps_allowed: unknown   # VPS/server/VPN not mentioned on any official page (see §7a)
personal_device_required: unknown   # not required in words; shared devices with ANOTHER trader are forbidden (see §7a)
notes: "Buffer 3100.00 must be cleared before any payout; min 2 qualifying days per payout cycle; max 5 sim payouts then Live. Active accounts: 1 per user. DLL conflict as in evaluation. News allowed. Inactivity 7 calendar days."
status: conflict
not_confirmed_fields: [daily_loss_limit, drawdown.stops_trailing_at]
sources:
  - { field: drawdown, url: "https://help.myfundedfutures.com/en/articles/17005296-builder-plan-100k-comprehensive-guide", accessed: "2026-10-23" }
  - { field: drawdown, url: "https://myfundedfutures.com/plans/builder", accessed: "2026-10-23" }
  - { field: drawdown, url: "https://help.myfundedfutures.com/en/articles/13745661-payout-policy-overview-best-and-fastest-prop-firm-payouts", accessed: "2026-10-23" }
  - { field: daily_loss_limit, url: "https://help.myfundedfutures.com/en/articles/17005296-builder-plan-100k-comprehensive-guide", accessed: "2026-10-23" }
  - { field: daily_loss_limit, url: "https://help.myfundedfutures.com/en/articles/13745661-payout-policy-overview-best-and-fastest-prop-firm-payouts", accessed: "2026-10-23" }
  - { field: consistency_rule, url: "https://help.myfundedfutures.com/en/articles/17005296-builder-plan-100k-comprehensive-guide", accessed: "2026-10-23" }
  - { field: max_position, url: "https://help.myfundedfutures.com/en/articles/17005296-builder-plan-100k-comprehensive-guide", accessed: "2026-10-23" }
  - { field: max_position, url: "https://myfundedfutures.com/plans/builder", accessed: "2026-10-23" }
  - { field: max_accounts, url: "https://myfundedfutures.com/plans/builder", accessed: "2026-10-23" }
  - { field: max_accounts, url: "https://help.myfundedfutures.com/en/articles/16498635-moving-from-evaluation-to-sim-funded-account", accessed: "2026-10-23" }
  - { field: copy_trading_allowed, url: "https://help.myfundedfutures.com/en/articles/10771500-copy-trading-at-myfundedfutures", accessed: "2026-10-23" }
  - { field: copy_trading_allowed, url: "https://myfundedfutures.com/terms", accessed: "2026-10-23" }
  - { field: automated_trading_allowed, url: "https://help.myfundedfutures.com/en/articles/8444599-fair-play-and-prohibited-trading-practices", accessed: "2026-10-23" }
  - { field: trading_day_reset, url: "https://help.myfundedfutures.com/en/articles/9558251-permitted-times-to-trade", accessed: "2026-10-23" }
  - { field: platforms, url: "https://help.myfundedfutures.com/en/articles/8528335-overview-of-supported-platforms-at-mffu", accessed: "2026-10-23" }
```

#### Builder — live — 100K

```yaml
firm: myfundedfutures
plan: "Builder"
phase: live
size_label: "100K"
size_usd: "100000.00"
instruments: [MNQ, NQ]
platforms: []   # live platform not published
drawdown: { amount_usd: "3000.00", type: trailing_eod, stops_trailing_at: "conflict: plan page 'Static once MLL reaches $0' vs help center 'Locks and remains static once it reaches $100 above the starting balance'" }
daily_loss_limit: { amount_usd: null, kind: null }
profit_target_usd: null
consistency_rule: { text: "None (Live)", threshold_pct: null }
max_position: { minis: 3, micros: 30, scaling: "Live: no set scaling plan; limits reviewed with risk manager (live FAQ)" }
min_trading_days: null
trading_day_reset: { time: null, tz_as_published: null }   # no explicit reset published; session "6:00 PM EST" open, positions auto-closed "4:10 PM EST" (see §4)
copy_trading_allowed: conditional   # copiers allowed on all account types; copying other traders and multi-account hedge/mirror/copy for unfair advantage prohibited (see §5)
automated_trading_allowed: conditional   # allowed unless HFT or aimed at exploiting sim fills; live must follow CME guidelines (see §6)
automation_rule_text: "High-frequency Trading is not allowed on our plans. Traders may make use of automated trading strategies tailored to their own specific settings so long as these automated tools do not aim to exploit the favorable fills offered in the Simulated Environment. Traders trading live accounts with automated trading must abide by CME guidelines."
vps_allowed: unknown   # VPS/server/VPN not mentioned on any official page (see §7a)
personal_device_required: unknown   # not required in words; shared devices with ANOTHER trader are forbidden (see §7a)
notes: "Live DLL conflict: plan page '$1,750' vs help center 'None'. Promotion after 5th approved sim payout; Live at Blue Row Capital; max 1 live account; 21-day cooldown after live MLL breach. Live trading is outside All Win agent scope (user-only)."
status: conflict
not_confirmed_fields: [drawdown.stops_trailing_at, daily_loss_limit]
sources:
  - { field: drawdown, url: "https://help.myfundedfutures.com/en/articles/17005296-builder-plan-100k-comprehensive-guide", accessed: "2026-10-23" }
  - { field: drawdown, url: "https://myfundedfutures.com/plans/builder", accessed: "2026-10-23" }
  - { field: daily_loss_limit, url: "https://help.myfundedfutures.com/en/articles/17005296-builder-plan-100k-comprehensive-guide", accessed: "2026-10-23" }
  - { field: daily_loss_limit, url: "https://myfundedfutures.com/plans/builder", accessed: "2026-10-23" }
  - { field: max_position, url: "https://help.myfundedfutures.com/en/articles/17005296-builder-plan-100k-comprehensive-guide", accessed: "2026-10-23" }
  - { field: copy_trading_allowed, url: "https://help.myfundedfutures.com/en/articles/10771500-copy-trading-at-myfundedfutures", accessed: "2026-10-23" }
  - { field: copy_trading_allowed, url: "https://myfundedfutures.com/terms", accessed: "2026-10-23" }
  - { field: automated_trading_allowed, url: "https://help.myfundedfutures.com/en/articles/8444599-fair-play-and-prohibited-trading-practices", accessed: "2026-10-23" }
  - { field: trading_day_reset, url: "https://help.myfundedfutures.com/en/articles/9558251-permitted-times-to-trade", accessed: "2026-10-23" }
  - { field: platforms, url: "https://help.myfundedfutures.com/en/articles/8528335-overview-of-supported-platforms-at-mffu", accessed: "2026-10-23" }
```

#### Builder — evaluation — 150K

```yaml
firm: myfundedfutures
plan: "Builder"
phase: evaluation
size_label: "150K"
size_usd: "150000.00"
instruments: [MNQ, NQ]
platforms: [Tradovate, NinjaTrader, TradingView, Quantower, ATAS, DeepChart/DeepDom, Volumetrica, Hyperprop]
drawdown: { amount_usd: "4500.00", type: trailing_eod, stops_trailing_at: "MLL locks permanently once it reaches $100 above the starting balance (help-center Account Rules)" }
daily_loss_limit: { amount_usd: "2500.00", kind: soft_pause }
profit_target_usd: "9000.00"
consistency_rule: { text: "None in evaluation", threshold_pct: null }
max_position: { minis: 9, micros: 90, scaling: "No scaling plan published; fixed max contracts" }
min_trading_days: 1
trading_day_reset: { time: null, tz_as_published: null }   # no explicit reset published; session "6:00 PM EST" open, positions auto-closed "4:10 PM EST" (see §4)
copy_trading_allowed: conditional   # copiers allowed on all account types; copying other traders and multi-account hedge/mirror/copy for unfair advantage prohibited (see §5)
automated_trading_allowed: conditional   # allowed unless HFT or aimed at exploiting sim fills; live must follow CME guidelines (see §6)
automation_rule_text: "High-frequency Trading is not allowed on our plans. Traders may make use of automated trading strategies tailored to their own specific settings so long as these automated tools do not aim to exploit the favorable fills offered in the Simulated Environment. Traders trading live accounts with automated trading must abide by CME guidelines."
vps_allowed: unknown   # VPS/server/VPN not mentioned on any official page (see §7a)
personal_device_required: unknown   # not required in words; shared devices with ANOTHER trader are forbidden (see §7a)
notes: "DLL conflict: plan page + help center say soft pause (trading stops for the rest of the session, account not breached); payout-policy article says 'No daily loss limits on Builder, Rapid or Pro plans'. Min-days conflict: plan-page table '1 Day' vs plan-page stage card 'One phase · min 2 days'. News trading allowed. Inactivity 7 calendar days."
status: conflict
not_confirmed_fields: [daily_loss_limit, min_trading_days]
sources:
  - { field: drawdown, url: "https://help.myfundedfutures.com/en/articles/17035331-builder-plan-150k-comprehensive-guide", accessed: "2026-10-23" }
  - { field: drawdown, url: "https://myfundedfutures.com/plans/builder", accessed: "2026-10-23" }
  - { field: daily_loss_limit, url: "https://help.myfundedfutures.com/en/articles/17035331-builder-plan-150k-comprehensive-guide", accessed: "2026-10-23" }
  - { field: daily_loss_limit, url: "https://myfundedfutures.com/plans/builder", accessed: "2026-10-23" }
  - { field: daily_loss_limit, url: "https://help.myfundedfutures.com/en/articles/13745661-payout-policy-overview-best-and-fastest-prop-firm-payouts", accessed: "2026-10-23" }
  - { field: profit_target_usd, url: "https://help.myfundedfutures.com/en/articles/17035331-builder-plan-150k-comprehensive-guide", accessed: "2026-10-23" }
  - { field: consistency_rule, url: "https://help.myfundedfutures.com/en/articles/11994562-consistency-rule-at-my-fundedfutures", accessed: "2026-10-23" }
  - { field: max_position, url: "https://help.myfundedfutures.com/en/articles/17035331-builder-plan-150k-comprehensive-guide", accessed: "2026-10-23" }
  - { field: max_position, url: "https://myfundedfutures.com/plans/builder", accessed: "2026-10-23" }
  - { field: min_trading_days, url: "https://myfundedfutures.com/plans/builder", accessed: "2026-10-23" }
  - { field: min_trading_days, url: "https://help.myfundedfutures.com/en/articles/17035331-builder-plan-150k-comprehensive-guide", accessed: "2026-10-23" }
  - { field: copy_trading_allowed, url: "https://help.myfundedfutures.com/en/articles/10771500-copy-trading-at-myfundedfutures", accessed: "2026-10-23" }
  - { field: copy_trading_allowed, url: "https://myfundedfutures.com/terms", accessed: "2026-10-23" }
  - { field: automated_trading_allowed, url: "https://help.myfundedfutures.com/en/articles/8444599-fair-play-and-prohibited-trading-practices", accessed: "2026-10-23" }
  - { field: trading_day_reset, url: "https://help.myfundedfutures.com/en/articles/9558251-permitted-times-to-trade", accessed: "2026-10-23" }
  - { field: platforms, url: "https://help.myfundedfutures.com/en/articles/8528335-overview-of-supported-platforms-at-mffu", accessed: "2026-10-23" }
```

#### Builder — funded — 150K

```yaml
firm: myfundedfutures
plan: "Builder"
phase: funded
size_label: "150K"
size_usd: "150000.00"
instruments: [MNQ, NQ]
platforms: [Tradovate, NinjaTrader, TradingView, Quantower, ATAS, DeepChart/DeepDom, Volumetrica, Hyperprop]
drawdown: { amount_usd: "4500.00", type: trailing_eod, stops_trailing_at: "Sim Funded starts at $0; plan page 'MLL Lock Level $100'; help center 'locks permanently once it reaches $100 above the starting balance'; payout-policy article 'After your first payout, the Max Loss Limit resets to $100'" }
daily_loss_limit: { amount_usd: "2500.00", kind: soft_pause }
profit_target_usd: null
consistency_rule: { text: "Payout stage: single largest profit day cannot represent more than 50% of total profits in the cycle; resets after each approved payout", threshold_pct: "50" }
max_position: { minis: 9, micros: 90, scaling: "No scaling plan published; fixed max contracts" }
min_trading_days: null
trading_day_reset: { time: null, tz_as_published: null }   # no explicit reset published; session "6:00 PM EST" open, positions auto-closed "4:10 PM EST" (see §4)
copy_trading_allowed: conditional   # copiers allowed on all account types; copying other traders and multi-account hedge/mirror/copy for unfair advantage prohibited (see §5)
automated_trading_allowed: conditional   # allowed unless HFT or aimed at exploiting sim fills; live must follow CME guidelines (see §6)
automation_rule_text: "High-frequency Trading is not allowed on our plans. Traders may make use of automated trading strategies tailored to their own specific settings so long as these automated tools do not aim to exploit the favorable fills offered in the Simulated Environment. Traders trading live accounts with automated trading must abide by CME guidelines."
vps_allowed: unknown   # VPS/server/VPN not mentioned on any official page (see §7a)
personal_device_required: unknown   # not required in words; shared devices with ANOTHER trader are forbidden (see §7a)
notes: "Buffer 4600.00 must be cleared before any payout; min 2 qualifying days per payout cycle; max 5 sim payouts then Live. Active accounts: 1 per user. Max payout per cycle conflict: plan page $3,500 vs help center $4,500. DLL conflict as in evaluation. News allowed. Inactivity 7 calendar days."
status: conflict
not_confirmed_fields: [daily_loss_limit, drawdown.stops_trailing_at, payout_max_per_cycle]
sources:
  - { field: drawdown, url: "https://help.myfundedfutures.com/en/articles/17035331-builder-plan-150k-comprehensive-guide", accessed: "2026-10-23" }
  - { field: drawdown, url: "https://myfundedfutures.com/plans/builder", accessed: "2026-10-23" }
  - { field: drawdown, url: "https://help.myfundedfutures.com/en/articles/13745661-payout-policy-overview-best-and-fastest-prop-firm-payouts", accessed: "2026-10-23" }
  - { field: daily_loss_limit, url: "https://help.myfundedfutures.com/en/articles/17035331-builder-plan-150k-comprehensive-guide", accessed: "2026-10-23" }
  - { field: daily_loss_limit, url: "https://help.myfundedfutures.com/en/articles/13745661-payout-policy-overview-best-and-fastest-prop-firm-payouts", accessed: "2026-10-23" }
  - { field: consistency_rule, url: "https://help.myfundedfutures.com/en/articles/17035331-builder-plan-150k-comprehensive-guide", accessed: "2026-10-23" }
  - { field: max_position, url: "https://help.myfundedfutures.com/en/articles/17035331-builder-plan-150k-comprehensive-guide", accessed: "2026-10-23" }
  - { field: max_position, url: "https://myfundedfutures.com/plans/builder", accessed: "2026-10-23" }
  - { field: max_accounts, url: "https://myfundedfutures.com/plans/builder", accessed: "2026-10-23" }
  - { field: max_accounts, url: "https://help.myfundedfutures.com/en/articles/16498635-moving-from-evaluation-to-sim-funded-account", accessed: "2026-10-23" }
  - { field: copy_trading_allowed, url: "https://help.myfundedfutures.com/en/articles/10771500-copy-trading-at-myfundedfutures", accessed: "2026-10-23" }
  - { field: copy_trading_allowed, url: "https://myfundedfutures.com/terms", accessed: "2026-10-23" }
  - { field: automated_trading_allowed, url: "https://help.myfundedfutures.com/en/articles/8444599-fair-play-and-prohibited-trading-practices", accessed: "2026-10-23" }
  - { field: trading_day_reset, url: "https://help.myfundedfutures.com/en/articles/9558251-permitted-times-to-trade", accessed: "2026-10-23" }
  - { field: platforms, url: "https://help.myfundedfutures.com/en/articles/8528335-overview-of-supported-platforms-at-mffu", accessed: "2026-10-23" }
```

#### Builder — live — 150K

```yaml
firm: myfundedfutures
plan: "Builder"
phase: live
size_label: "150K"
size_usd: "150000.00"
instruments: [MNQ, NQ]
platforms: []   # live platform not published
drawdown: { amount_usd: "4500.00", type: trailing_eod, stops_trailing_at: "conflict: plan page 'Static once MLL reaches $0' vs help center 'Locks and remains static once it reaches $100 above the starting balance'" }
daily_loss_limit: { amount_usd: null, kind: null }
profit_target_usd: null
consistency_rule: { text: "None (Live)", threshold_pct: null }
max_position: { minis: 4, micros: 40, scaling: "Live: no set scaling plan; limits reviewed with risk manager (live FAQ)" }
min_trading_days: null
trading_day_reset: { time: null, tz_as_published: null }   # no explicit reset published; session "6:00 PM EST" open, positions auto-closed "4:10 PM EST" (see §4)
copy_trading_allowed: conditional   # copiers allowed on all account types; copying other traders and multi-account hedge/mirror/copy for unfair advantage prohibited (see §5)
automated_trading_allowed: conditional   # allowed unless HFT or aimed at exploiting sim fills; live must follow CME guidelines (see §6)
automation_rule_text: "High-frequency Trading is not allowed on our plans. Traders may make use of automated trading strategies tailored to their own specific settings so long as these automated tools do not aim to exploit the favorable fills offered in the Simulated Environment. Traders trading live accounts with automated trading must abide by CME guidelines."
vps_allowed: unknown   # VPS/server/VPN not mentioned on any official page (see §7a)
personal_device_required: unknown   # not required in words; shared devices with ANOTHER trader are forbidden (see §7a)
notes: "Live DLL conflict: plan page '$2,500' vs help center 'None'. Promotion after 5th approved sim payout; Live at Blue Row Capital; max 1 live account; 21-day cooldown after live MLL breach. Live trading is outside All Win agent scope (user-only)."
status: conflict
not_confirmed_fields: [drawdown.stops_trailing_at, daily_loss_limit]
sources:
  - { field: drawdown, url: "https://help.myfundedfutures.com/en/articles/17035331-builder-plan-150k-comprehensive-guide", accessed: "2026-10-23" }
  - { field: drawdown, url: "https://myfundedfutures.com/plans/builder", accessed: "2026-10-23" }
  - { field: daily_loss_limit, url: "https://help.myfundedfutures.com/en/articles/17035331-builder-plan-150k-comprehensive-guide", accessed: "2026-10-23" }
  - { field: daily_loss_limit, url: "https://myfundedfutures.com/plans/builder", accessed: "2026-10-23" }
  - { field: max_position, url: "https://help.myfundedfutures.com/en/articles/17035331-builder-plan-150k-comprehensive-guide", accessed: "2026-10-23" }
  - { field: copy_trading_allowed, url: "https://help.myfundedfutures.com/en/articles/10771500-copy-trading-at-myfundedfutures", accessed: "2026-10-23" }
  - { field: copy_trading_allowed, url: "https://myfundedfutures.com/terms", accessed: "2026-10-23" }
  - { field: automated_trading_allowed, url: "https://help.myfundedfutures.com/en/articles/8444599-fair-play-and-prohibited-trading-practices", accessed: "2026-10-23" }
  - { field: trading_day_reset, url: "https://help.myfundedfutures.com/en/articles/9558251-permitted-times-to-trade", accessed: "2026-10-23" }
  - { field: platforms, url: "https://help.myfundedfutures.com/en/articles/8528335-overview-of-supported-platforms-at-mffu", accessed: "2026-10-23" }
```

#### Builder (Add-On MLL option) — evaluation — 50K

```yaml
firm: myfundedfutures
plan: "Builder (Add-On MLL option)"
phase: evaluation
size_label: "50K"
size_usd: "50000.00"
instruments: [MNQ, NQ]
platforms: [Tradovate, NinjaTrader, TradingView, Quantower, ATAS, DeepChart/DeepDom, Volumetrica, Hyperprop]
drawdown: { amount_usd: "1500.00", type: trailing_eod, stops_trailing_at: "MLL locks permanently once it reaches $100 above the starting balance" }
daily_loss_limit: { amount_usd: "1000.00", kind: soft_pause }
profit_target_usd: "3000.00"
consistency_rule: { text: "None in evaluation", threshold_pct: null }
max_position: { minis: 4, micros: 40, scaling: "No scaling plan published" }
min_trading_days: 1
trading_day_reset: { time: null, tz_as_published: null }   # no explicit reset published; session "6:00 PM EST" open, positions auto-closed "4:10 PM EST" (see §4)
copy_trading_allowed: conditional   # copiers allowed on all account types; copying other traders and multi-account hedge/mirror/copy for unfair advantage prohibited (see §5)
automated_trading_allowed: conditional   # allowed unless HFT or aimed at exploiting sim fills; live must follow CME guidelines (see §6)
automation_rule_text: "High-frequency Trading is not allowed on our plans. Traders may make use of automated trading strategies tailored to their own specific settings so long as these automated tools do not aim to exploit the favorable fills offered in the Simulated Environment. Traders trading live accounts with automated trading must abide by CME guidelines."
vps_allowed: unknown   # VPS/server/VPN not mentioned on any official page (see §7a)
personal_device_required: unknown   # not required in words; shared devices with ANOTHER trader are forbidden (see §7a)
notes: "Checkout option of Builder 50K; starting minimum balance $48,500. Same conflicts as Builder 50K evaluation."
status: conflict
not_confirmed_fields: [daily_loss_limit, min_trading_days]
sources:
  - { field: drawdown, url: "https://help.myfundedfutures.com/en/articles/14290805-builder-plan-50k-a-comprehensive-guide", accessed: "2026-10-23" }
  - { field: drawdown, url: "https://myfundedfutures.com/plans/builder", accessed: "2026-10-23" }
  - { field: daily_loss_limit, url: "https://help.myfundedfutures.com/en/articles/14290805-builder-plan-50k-a-comprehensive-guide", accessed: "2026-10-23" }
  - { field: copy_trading_allowed, url: "https://help.myfundedfutures.com/en/articles/10771500-copy-trading-at-myfundedfutures", accessed: "2026-10-23" }
  - { field: copy_trading_allowed, url: "https://myfundedfutures.com/terms", accessed: "2026-10-23" }
  - { field: automated_trading_allowed, url: "https://help.myfundedfutures.com/en/articles/8444599-fair-play-and-prohibited-trading-practices", accessed: "2026-10-23" }
  - { field: trading_day_reset, url: "https://help.myfundedfutures.com/en/articles/9558251-permitted-times-to-trade", accessed: "2026-10-23" }
  - { field: platforms, url: "https://help.myfundedfutures.com/en/articles/8528335-overview-of-supported-platforms-at-mffu", accessed: "2026-10-23" }
```

#### Builder (Add-On MLL option) — funded — 50K

```yaml
firm: myfundedfutures
plan: "Builder (Add-On MLL option)"
phase: funded
size_label: "50K"
size_usd: "50000.00"
instruments: [MNQ, NQ]
platforms: [Tradovate, NinjaTrader, TradingView, Quantower, ATAS, DeepChart/DeepDom, Volumetrica, Hyperprop]
drawdown: { amount_usd: "1500.00", type: trailing_eod, stops_trailing_at: "Sim Funded starts at $0, starting minimum balance -$1,500; lock wording as Builder 50K funded" }
daily_loss_limit: { amount_usd: "1000.00", kind: soft_pause }
profit_target_usd: null
consistency_rule: { text: "Payout stage 50% (as Builder 50K)", threshold_pct: "50" }
max_position: { minis: 4, micros: 40, scaling: "No scaling plan published" }
min_trading_days: null
trading_day_reset: { time: null, tz_as_published: null }   # no explicit reset published; session "6:00 PM EST" open, positions auto-closed "4:10 PM EST" (see §4)
copy_trading_allowed: conditional   # copiers allowed on all account types; copying other traders and multi-account hedge/mirror/copy for unfair advantage prohibited (see §5)
automated_trading_allowed: conditional   # allowed unless HFT or aimed at exploiting sim fills; live must follow CME guidelines (see §6)
automation_rule_text: "High-frequency Trading is not allowed on our plans. Traders may make use of automated trading strategies tailored to their own specific settings so long as these automated tools do not aim to exploit the favorable fills offered in the Simulated Environment. Traders trading live accounts with automated trading must abide by CME guidelines."
vps_allowed: unknown   # VPS/server/VPN not mentioned on any official page (see §7a)
personal_device_required: unknown   # not required in words; shared devices with ANOTHER trader are forbidden (see §7a)
notes: "Buffer $1,600. 1 Builder account per user."
status: conflict
not_confirmed_fields: [daily_loss_limit, drawdown.stops_trailing_at]
sources:
  - { field: drawdown, url: "https://help.myfundedfutures.com/en/articles/14290805-builder-plan-50k-a-comprehensive-guide", accessed: "2026-10-23" }
  - { field: daily_loss_limit, url: "https://help.myfundedfutures.com/en/articles/14290805-builder-plan-50k-a-comprehensive-guide", accessed: "2026-10-23" }
  - { field: copy_trading_allowed, url: "https://help.myfundedfutures.com/en/articles/10771500-copy-trading-at-myfundedfutures", accessed: "2026-10-23" }
  - { field: copy_trading_allowed, url: "https://myfundedfutures.com/terms", accessed: "2026-10-23" }
  - { field: automated_trading_allowed, url: "https://help.myfundedfutures.com/en/articles/8444599-fair-play-and-prohibited-trading-practices", accessed: "2026-10-23" }
  - { field: trading_day_reset, url: "https://help.myfundedfutures.com/en/articles/9558251-permitted-times-to-trade", accessed: "2026-10-23" }
  - { field: platforms, url: "https://help.myfundedfutures.com/en/articles/8528335-overview-of-supported-platforms-at-mffu", accessed: "2026-10-23" }
```

#### Builder (Add-On MLL option) — live — 50K

```yaml
firm: myfundedfutures
plan: "Builder (Add-On MLL option)"
phase: live
size_label: "50K"
size_usd: "50000.00"
instruments: [MNQ, NQ]
platforms: []   # live platform not published
drawdown: { amount_usd: "1500.00", type: trailing_eod, stops_trailing_at: "Static once MLL reaches $0" }
daily_loss_limit: { amount_usd: "1000.00", kind: soft_pause }
profit_target_usd: null
consistency_rule: { text: "None (Live)", threshold_pct: null }
max_position: { minis: null, micros: null, scaling: "Live: no set scaling plan" }
min_trading_days: null
trading_day_reset: { time: null, tz_as_published: null }   # no explicit reset published; session "6:00 PM EST" open, positions auto-closed "4:10 PM EST" (see §4)
copy_trading_allowed: conditional   # copiers allowed on all account types; copying other traders and multi-account hedge/mirror/copy for unfair advantage prohibited (see §5)
automated_trading_allowed: conditional   # allowed unless HFT or aimed at exploiting sim fills; live must follow CME guidelines (see §6)
automation_rule_text: "High-frequency Trading is not allowed on our plans. Traders may make use of automated trading strategies tailored to their own specific settings so long as these automated tools do not aim to exploit the favorable fills offered in the Simulated Environment. Traders trading live accounts with automated trading must abide by CME guidelines."
vps_allowed: unknown   # VPS/server/VPN not mentioned on any official page (see §7a)
personal_device_required: unknown   # not required in words; shared devices with ANOTHER trader are forbidden (see §7a)
notes: "Live max contracts not published."
status: partial
not_confirmed_fields: []
sources:
  - { field: drawdown, url: "https://help.myfundedfutures.com/en/articles/14290805-builder-plan-50k-a-comprehensive-guide", accessed: "2026-10-23" }
  - { field: drawdown, url: "https://myfundedfutures.com/plans/builder", accessed: "2026-10-23" }
  - { field: daily_loss_limit, url: "https://help.myfundedfutures.com/en/articles/14290805-builder-plan-50k-a-comprehensive-guide", accessed: "2026-10-23" }
  - { field: copy_trading_allowed, url: "https://help.myfundedfutures.com/en/articles/10771500-copy-trading-at-myfundedfutures", accessed: "2026-10-23" }
  - { field: copy_trading_allowed, url: "https://myfundedfutures.com/terms", accessed: "2026-10-23" }
  - { field: automated_trading_allowed, url: "https://help.myfundedfutures.com/en/articles/8444599-fair-play-and-prohibited-trading-practices", accessed: "2026-10-23" }
  - { field: trading_day_reset, url: "https://help.myfundedfutures.com/en/articles/9558251-permitted-times-to-trade", accessed: "2026-10-23" }
  - { field: platforms, url: "https://help.myfundedfutures.com/en/articles/8528335-overview-of-supported-platforms-at-mffu", accessed: "2026-10-23" }
```

#### Rapid — evaluation — 25K

```yaml
firm: myfundedfutures
plan: "Rapid"
phase: evaluation
size_label: "25K"
size_usd: "25000.00"
instruments: [MNQ, NQ]
platforms: [Tradovate, NinjaTrader, TradingView, Quantower, ATAS, DeepChart/DeepDom, Volumetrica, Hyperprop]
drawdown: { amount_usd: "1000.00", type: trailing_eod, stops_trailing_at: "Help center (25K): 'Once your trailing Max Loss reaches $25,100, it locks there.'" }
daily_loss_limit: { amount_usd: null, kind: none }
profit_target_usd: "1500.00"
consistency_rule: { text: "No single day's profit may exceed 50% of total evaluation profits; exceeding it does not breach, trader must trade more days", threshold_pct: "50" }
max_position: { minis: 3, micros: 30, scaling: "No scaling plan published" }
min_trading_days: 2
trading_day_reset: { time: null, tz_as_published: null }   # no explicit reset published; session "6:00 PM EST" open, positions auto-closed "4:10 PM EST" (see §4)
copy_trading_allowed: conditional   # copiers allowed on all account types; copying other traders and multi-account hedge/mirror/copy for unfair advantage prohibited (see §5)
automated_trading_allowed: conditional   # allowed unless HFT or aimed at exploiting sim fills; live must follow CME guidelines (see §6)
automation_rule_text: "High-frequency Trading is not allowed on our plans. Traders may make use of automated trading strategies tailored to their own specific settings so long as these automated tools do not aim to exploit the favorable fills offered in the Simulated Environment. Traders trading live accounts with automated trading must abide by CME guidelines."
vps_allowed: unknown   # VPS/server/VPN not mentioned on any official page (see §7a)
personal_device_required: unknown   # not required in words; shared devices with ANOTHER trader are forbidden (see §7a)
notes: "Max contracts in evaluation published only in the help center / Traders Evaluation Simplified (plan-page table has no eval contract row; the size selector shows the same numbers). T1 news allowed in evaluation."
status: verified
not_confirmed_fields: []
sources:
  - { field: drawdown, url: "https://help.myfundedfutures.com/en/articles/14116402-rapid-plan-25k-a-comprehensive-look", accessed: "2026-10-23" }
  - { field: drawdown, url: "https://myfundedfutures.com/plans/rapid", accessed: "2026-10-23" }
  - { field: drawdown, url: "https://help.myfundedfutures.com/en/articles/8348565-end-of-day-eod-drawdown-explained", accessed: "2026-10-23" }
  - { field: daily_loss_limit, url: "https://help.myfundedfutures.com/en/articles/14116402-rapid-plan-25k-a-comprehensive-look", accessed: "2026-10-23" }
  - { field: profit_target_usd, url: "https://help.myfundedfutures.com/en/articles/14116402-rapid-plan-25k-a-comprehensive-look", accessed: "2026-10-23" }
  - { field: consistency_rule, url: "https://help.myfundedfutures.com/en/articles/11994562-consistency-rule-at-my-fundedfutures", accessed: "2026-10-23" }
  - { field: max_position, url: "https://help.myfundedfutures.com/en/articles/14116402-rapid-plan-25k-a-comprehensive-look", accessed: "2026-10-23" }
  - { field: max_position, url: "https://help.myfundedfutures.com/en/articles/11802636-traders-evaluation-simplified", accessed: "2026-10-23" }
  - { field: min_trading_days, url: "https://help.myfundedfutures.com/en/articles/14116402-rapid-plan-25k-a-comprehensive-look", accessed: "2026-10-23" }
  - { field: copy_trading_allowed, url: "https://help.myfundedfutures.com/en/articles/10771500-copy-trading-at-myfundedfutures", accessed: "2026-10-23" }
  - { field: copy_trading_allowed, url: "https://myfundedfutures.com/terms", accessed: "2026-10-23" }
  - { field: automated_trading_allowed, url: "https://help.myfundedfutures.com/en/articles/8444599-fair-play-and-prohibited-trading-practices", accessed: "2026-10-23" }
  - { field: trading_day_reset, url: "https://help.myfundedfutures.com/en/articles/9558251-permitted-times-to-trade", accessed: "2026-10-23" }
  - { field: platforms, url: "https://help.myfundedfutures.com/en/articles/8528335-overview-of-supported-platforms-at-mffu", accessed: "2026-10-23" }
```

#### Rapid — funded — 25K

```yaml
firm: myfundedfutures
plan: "Rapid"
phase: funded
size_label: "25K"
size_usd: "25000.00"
instruments: [MNQ, NQ]
platforms: [Tradovate, NinjaTrader, TradingView, Quantower, ATAS, DeepChart/DeepDom, Volumetrica, Hyperprop]
drawdown: { amount_usd: "1000.00", type: trailing_intraday, stops_trailing_at: "conflict: help center 'Once your trailing Max Loss reaches $100, it locks there' vs plan-page FAQ 'once your balance is +$100 above your starting balance after your first payout, the floor locks'" }
daily_loss_limit: { amount_usd: null, kind: none }
profit_target_usd: null
consistency_rule: { text: "None in Sim Funded", threshold_pct: null }
max_position: { minis: 3, micros: 30, scaling: "No scaling plan published" }
min_trading_days: null
trading_day_reset: { time: null, tz_as_published: null }   # no explicit reset published; session "6:00 PM EST" open, positions auto-closed "4:10 PM EST" (see §4)
copy_trading_allowed: conditional   # copiers allowed on all account types; copying other traders and multi-account hedge/mirror/copy for unfair advantage prohibited (see §5)
automated_trading_allowed: conditional   # allowed unless HFT or aimed at exploiting sim fills; live must follow CME guidelines (see §6)
automation_rule_text: "High-frequency Trading is not allowed on our plans. Traders may make use of automated trading strategies tailored to their own specific settings so long as these automated tools do not aim to exploit the favorable fills offered in the Simulated Environment. Traders trading live accounts with automated trading must abide by CME guidelines."
vps_allowed: unknown   # VPS/server/VPN not mentioned on any official page (see §7a)
personal_device_required: unknown   # not required in words; shared devices with ANOTHER trader are forbidden (see §7a)
notes: "Sim Funded starts at $0; Max Loss Distance 1000.00 from equity high-water mark, trails intraday incl. unrealized. Buffer 1100.00; payouts daily; T1 news NOT allowed (2 minutes before/after). Holding overnight breaches. Account limits: up to 5 active Sim Funded if only 25K/50K; 3 total if any 100K/150K."
status: conflict
not_confirmed_fields: [drawdown.stops_trailing_at]
sources:
  - { field: drawdown, url: "https://help.myfundedfutures.com/en/articles/14116402-rapid-plan-25k-a-comprehensive-look", accessed: "2026-10-23" }
  - { field: drawdown, url: "https://myfundedfutures.com/plans/rapid", accessed: "2026-10-23" }
  - { field: drawdown, url: "https://help.myfundedfutures.com/en/articles/12802721-intraday-drawdown-explained", accessed: "2026-10-23" }
  - { field: daily_loss_limit, url: "https://help.myfundedfutures.com/en/articles/14116402-rapid-plan-25k-a-comprehensive-look", accessed: "2026-10-23" }
  - { field: consistency_rule, url: "https://help.myfundedfutures.com/en/articles/14116402-rapid-plan-25k-a-comprehensive-look", accessed: "2026-10-23" }
  - { field: max_position, url: "https://help.myfundedfutures.com/en/articles/14116402-rapid-plan-25k-a-comprehensive-look", accessed: "2026-10-23" }
  - { field: max_position, url: "https://myfundedfutures.com/plans/rapid", accessed: "2026-10-23" }
  - { field: max_position, url: "https://help.myfundedfutures.com/en/articles/10244682-cross-instrument-policy", accessed: "2026-10-23" }
  - { field: max_accounts, url: "https://help.myfundedfutures.com/en/articles/16498635-moving-from-evaluation-to-sim-funded-account", accessed: "2026-10-23" }
  - { field: copy_trading_allowed, url: "https://help.myfundedfutures.com/en/articles/10771500-copy-trading-at-myfundedfutures", accessed: "2026-10-23" }
  - { field: copy_trading_allowed, url: "https://myfundedfutures.com/terms", accessed: "2026-10-23" }
  - { field: automated_trading_allowed, url: "https://help.myfundedfutures.com/en/articles/8444599-fair-play-and-prohibited-trading-practices", accessed: "2026-10-23" }
  - { field: trading_day_reset, url: "https://help.myfundedfutures.com/en/articles/9558251-permitted-times-to-trade", accessed: "2026-10-23" }
  - { field: platforms, url: "https://help.myfundedfutures.com/en/articles/8528335-overview-of-supported-platforms-at-mffu", accessed: "2026-10-23" }
```

#### Rapid — live — 25K

```yaml
firm: myfundedfutures
plan: "Rapid"
phase: live
size_label: "25K"
size_usd: "25000.00"
instruments: [MNQ, NQ]
platforms: []   # live platform not published
drawdown: { amount_usd: "1000.00", type: trailing_eod, stops_trailing_at: "Max Loss threshold stops at $0 (Live starts at $0)" }
daily_loss_limit: { amount_usd: null, kind: null }
profit_target_usd: null
consistency_rule: { text: null, threshold_pct: null }
max_position: { minis: 2, micros: 20, scaling: "Multiple accounts moved to Live are combined into one Live account; max contracts then discussed with the Live team" }
min_trading_days: null
trading_day_reset: { time: null, tz_as_published: null }   # no explicit reset published; session "6:00 PM EST" open, positions auto-closed "4:10 PM EST" (see §4)
copy_trading_allowed: conditional   # copiers allowed on all account types; copying other traders and multi-account hedge/mirror/copy for unfair advantage prohibited (see §5)
automated_trading_allowed: conditional   # allowed unless HFT or aimed at exploiting sim fills; live must follow CME guidelines (see §6)
automation_rule_text: "High-frequency Trading is not allowed on our plans. Traders may make use of automated trading strategies tailored to their own specific settings so long as these automated tools do not aim to exploit the favorable fills offered in the Simulated Environment. Traders trading live accounts with automated trading must abide by CME guidelines."
vps_allowed: unknown   # VPS/server/VPN not mentioned on any official page (see §7a)
personal_device_required: unknown   # not required in words; shared devices with ANOTHER trader are forbidden (see §7a)
notes: "Live DLL not stated on the Rapid Live page (null = not found; generic live article mentions a 'Customizable Daily Loss Limit'). Transition: $10,000 net profit in a single day (automatic) or Risk Team review. Live trading is user-only."
status: partial
not_confirmed_fields: []
sources:
  - { field: drawdown, url: "https://help.myfundedfutures.com/en/articles/13134718-understanding-rapid-live", accessed: "2026-10-23" }
  - { field: drawdown, url: "https://myfundedfutures.com/plans/rapid", accessed: "2026-10-23" }
  - { field: max_position, url: "https://help.myfundedfutures.com/en/articles/13134718-understanding-rapid-live", accessed: "2026-10-23" }
  - { field: max_position, url: "https://myfundedfutures.com/plans/rapid", accessed: "2026-10-23" }
  - { field: copy_trading_allowed, url: "https://help.myfundedfutures.com/en/articles/10771500-copy-trading-at-myfundedfutures", accessed: "2026-10-23" }
  - { field: copy_trading_allowed, url: "https://myfundedfutures.com/terms", accessed: "2026-10-23" }
  - { field: automated_trading_allowed, url: "https://help.myfundedfutures.com/en/articles/8444599-fair-play-and-prohibited-trading-practices", accessed: "2026-10-23" }
  - { field: trading_day_reset, url: "https://help.myfundedfutures.com/en/articles/9558251-permitted-times-to-trade", accessed: "2026-10-23" }
  - { field: platforms, url: "https://help.myfundedfutures.com/en/articles/8528335-overview-of-supported-platforms-at-mffu", accessed: "2026-10-23" }
```

#### Rapid — evaluation — 50K

```yaml
firm: myfundedfutures
plan: "Rapid"
phase: evaluation
size_label: "50K"
size_usd: "50000.00"
instruments: [MNQ, NQ]
platforms: [Tradovate, NinjaTrader, TradingView, Quantower, ATAS, DeepChart/DeepDom, Volumetrica, Hyperprop]
drawdown: { amount_usd: "2000.00", type: trailing_eod, stops_trailing_at: "EOD article: 'the Max EOD trailing locks in at $100 plus the initial starting balance' (no size-specific lock published for this Rapid size)" }
daily_loss_limit: { amount_usd: null, kind: none }
profit_target_usd: "3000.00"
consistency_rule: { text: "No single day's profit may exceed 50% of total evaluation profits; exceeding it does not breach, trader must trade more days", threshold_pct: "50" }
max_position: { minis: 5, micros: 50, scaling: "No scaling plan published" }
min_trading_days: 2
trading_day_reset: { time: null, tz_as_published: null }   # no explicit reset published; session "6:00 PM EST" open, positions auto-closed "4:10 PM EST" (see §4)
copy_trading_allowed: conditional   # copiers allowed on all account types; copying other traders and multi-account hedge/mirror/copy for unfair advantage prohibited (see §5)
automated_trading_allowed: conditional   # allowed unless HFT or aimed at exploiting sim fills; live must follow CME guidelines (see §6)
automation_rule_text: "High-frequency Trading is not allowed on our plans. Traders may make use of automated trading strategies tailored to their own specific settings so long as these automated tools do not aim to exploit the favorable fills offered in the Simulated Environment. Traders trading live accounts with automated trading must abide by CME guidelines."
vps_allowed: unknown   # VPS/server/VPN not mentioned on any official page (see §7a)
personal_device_required: unknown   # not required in words; shared devices with ANOTHER trader are forbidden (see §7a)
notes: "Max contracts in evaluation published only in the help center / Traders Evaluation Simplified (plan-page table has no eval contract row; the size selector shows the same numbers). T1 news allowed in evaluation."
status: verified
not_confirmed_fields: []
sources:
  - { field: drawdown, url: "https://help.myfundedfutures.com/en/articles/13134709-rapid-plan-50k-a-comprehensive-look", accessed: "2026-10-23" }
  - { field: drawdown, url: "https://myfundedfutures.com/plans/rapid", accessed: "2026-10-23" }
  - { field: drawdown, url: "https://help.myfundedfutures.com/en/articles/8348565-end-of-day-eod-drawdown-explained", accessed: "2026-10-23" }
  - { field: daily_loss_limit, url: "https://help.myfundedfutures.com/en/articles/13134709-rapid-plan-50k-a-comprehensive-look", accessed: "2026-10-23" }
  - { field: profit_target_usd, url: "https://help.myfundedfutures.com/en/articles/13134709-rapid-plan-50k-a-comprehensive-look", accessed: "2026-10-23" }
  - { field: consistency_rule, url: "https://help.myfundedfutures.com/en/articles/11994562-consistency-rule-at-my-fundedfutures", accessed: "2026-10-23" }
  - { field: max_position, url: "https://help.myfundedfutures.com/en/articles/13134709-rapid-plan-50k-a-comprehensive-look", accessed: "2026-10-23" }
  - { field: max_position, url: "https://help.myfundedfutures.com/en/articles/11802636-traders-evaluation-simplified", accessed: "2026-10-23" }
  - { field: min_trading_days, url: "https://help.myfundedfutures.com/en/articles/13134709-rapid-plan-50k-a-comprehensive-look", accessed: "2026-10-23" }
  - { field: copy_trading_allowed, url: "https://help.myfundedfutures.com/en/articles/10771500-copy-trading-at-myfundedfutures", accessed: "2026-10-23" }
  - { field: copy_trading_allowed, url: "https://myfundedfutures.com/terms", accessed: "2026-10-23" }
  - { field: automated_trading_allowed, url: "https://help.myfundedfutures.com/en/articles/8444599-fair-play-and-prohibited-trading-practices", accessed: "2026-10-23" }
  - { field: trading_day_reset, url: "https://help.myfundedfutures.com/en/articles/9558251-permitted-times-to-trade", accessed: "2026-10-23" }
  - { field: platforms, url: "https://help.myfundedfutures.com/en/articles/8528335-overview-of-supported-platforms-at-mffu", accessed: "2026-10-23" }
```

#### Rapid — funded — 50K

```yaml
firm: myfundedfutures
plan: "Rapid"
phase: funded
size_label: "50K"
size_usd: "50000.00"
instruments: [MNQ, NQ]
platforms: [Tradovate, NinjaTrader, TradingView, Quantower, ATAS, DeepChart/DeepDom, Volumetrica, Hyperprop]
drawdown: { amount_usd: "2000.00", type: trailing_intraday, stops_trailing_at: "conflict: help center 'Once your trailing Max Loss reaches $100, it locks there' vs plan-page FAQ 'once your balance is +$100 above your starting balance after your first payout, the floor locks'" }
daily_loss_limit: { amount_usd: null, kind: none }
profit_target_usd: null
consistency_rule: { text: "None in Sim Funded", threshold_pct: null }
max_position: { minis: 5, micros: 50, scaling: "conflict: Cross Instrument Policy example says a Rapid 50k sim funded account starts with 2 contracts 'due to the scaling plan'; plan page and help center say 5 minis / 50 micros" }
min_trading_days: null
trading_day_reset: { time: null, tz_as_published: null }   # no explicit reset published; session "6:00 PM EST" open, positions auto-closed "4:10 PM EST" (see §4)
copy_trading_allowed: conditional   # copiers allowed on all account types; copying other traders and multi-account hedge/mirror/copy for unfair advantage prohibited (see §5)
automated_trading_allowed: conditional   # allowed unless HFT or aimed at exploiting sim fills; live must follow CME guidelines (see §6)
automation_rule_text: "High-frequency Trading is not allowed on our plans. Traders may make use of automated trading strategies tailored to their own specific settings so long as these automated tools do not aim to exploit the favorable fills offered in the Simulated Environment. Traders trading live accounts with automated trading must abide by CME guidelines."
vps_allowed: unknown   # VPS/server/VPN not mentioned on any official page (see §7a)
personal_device_required: unknown   # not required in words; shared devices with ANOTHER trader are forbidden (see §7a)
notes: "Sim Funded starts at $0; Max Loss Distance 2000.00 from equity high-water mark, trails intraday incl. unrealized. Buffer 2100.00; payouts daily; T1 news NOT allowed (2 minutes before/after). Holding overnight breaches. Account limits: up to 5 active Sim Funded if only 25K/50K; 3 total if any 100K/150K."
status: conflict
not_confirmed_fields: [drawdown.stops_trailing_at, max_position.scaling]
sources:
  - { field: drawdown, url: "https://help.myfundedfutures.com/en/articles/13134709-rapid-plan-50k-a-comprehensive-look", accessed: "2026-10-23" }
  - { field: drawdown, url: "https://myfundedfutures.com/plans/rapid", accessed: "2026-10-23" }
  - { field: drawdown, url: "https://help.myfundedfutures.com/en/articles/12802721-intraday-drawdown-explained", accessed: "2026-10-23" }
  - { field: daily_loss_limit, url: "https://help.myfundedfutures.com/en/articles/13134709-rapid-plan-50k-a-comprehensive-look", accessed: "2026-10-23" }
  - { field: consistency_rule, url: "https://help.myfundedfutures.com/en/articles/13134709-rapid-plan-50k-a-comprehensive-look", accessed: "2026-10-23" }
  - { field: max_position, url: "https://help.myfundedfutures.com/en/articles/13134709-rapid-plan-50k-a-comprehensive-look", accessed: "2026-10-23" }
  - { field: max_position, url: "https://myfundedfutures.com/plans/rapid", accessed: "2026-10-23" }
  - { field: max_position, url: "https://help.myfundedfutures.com/en/articles/10244682-cross-instrument-policy", accessed: "2026-10-23" }
  - { field: max_accounts, url: "https://help.myfundedfutures.com/en/articles/16498635-moving-from-evaluation-to-sim-funded-account", accessed: "2026-10-23" }
  - { field: copy_trading_allowed, url: "https://help.myfundedfutures.com/en/articles/10771500-copy-trading-at-myfundedfutures", accessed: "2026-10-23" }
  - { field: copy_trading_allowed, url: "https://myfundedfutures.com/terms", accessed: "2026-10-23" }
  - { field: automated_trading_allowed, url: "https://help.myfundedfutures.com/en/articles/8444599-fair-play-and-prohibited-trading-practices", accessed: "2026-10-23" }
  - { field: trading_day_reset, url: "https://help.myfundedfutures.com/en/articles/9558251-permitted-times-to-trade", accessed: "2026-10-23" }
  - { field: platforms, url: "https://help.myfundedfutures.com/en/articles/8528335-overview-of-supported-platforms-at-mffu", accessed: "2026-10-23" }
```

#### Rapid — live — 50K

```yaml
firm: myfundedfutures
plan: "Rapid"
phase: live
size_label: "50K"
size_usd: "50000.00"
instruments: [MNQ, NQ]
platforms: []   # live platform not published
drawdown: { amount_usd: "2000.00", type: trailing_eod, stops_trailing_at: "Max Loss threshold stops at $0 (Live starts at $0)" }
daily_loss_limit: { amount_usd: null, kind: null }
profit_target_usd: null
consistency_rule: { text: null, threshold_pct: null }
max_position: { minis: 3, micros: 30, scaling: "Multiple accounts moved to Live are combined into one Live account; max contracts then discussed with the Live team" }
min_trading_days: null
trading_day_reset: { time: null, tz_as_published: null }   # no explicit reset published; session "6:00 PM EST" open, positions auto-closed "4:10 PM EST" (see §4)
copy_trading_allowed: conditional   # copiers allowed on all account types; copying other traders and multi-account hedge/mirror/copy for unfair advantage prohibited (see §5)
automated_trading_allowed: conditional   # allowed unless HFT or aimed at exploiting sim fills; live must follow CME guidelines (see §6)
automation_rule_text: "High-frequency Trading is not allowed on our plans. Traders may make use of automated trading strategies tailored to their own specific settings so long as these automated tools do not aim to exploit the favorable fills offered in the Simulated Environment. Traders trading live accounts with automated trading must abide by CME guidelines."
vps_allowed: unknown   # VPS/server/VPN not mentioned on any official page (see §7a)
personal_device_required: unknown   # not required in words; shared devices with ANOTHER trader are forbidden (see §7a)
notes: "Live DLL not stated on the Rapid Live page (null = not found; generic live article mentions a 'Customizable Daily Loss Limit'). Transition: $10,000 net profit in a single day (automatic) or Risk Team review. Live trading is user-only."
status: partial
not_confirmed_fields: []
sources:
  - { field: drawdown, url: "https://help.myfundedfutures.com/en/articles/13134718-understanding-rapid-live", accessed: "2026-10-23" }
  - { field: drawdown, url: "https://myfundedfutures.com/plans/rapid", accessed: "2026-10-23" }
  - { field: max_position, url: "https://help.myfundedfutures.com/en/articles/13134718-understanding-rapid-live", accessed: "2026-10-23" }
  - { field: max_position, url: "https://myfundedfutures.com/plans/rapid", accessed: "2026-10-23" }
  - { field: copy_trading_allowed, url: "https://help.myfundedfutures.com/en/articles/10771500-copy-trading-at-myfundedfutures", accessed: "2026-10-23" }
  - { field: copy_trading_allowed, url: "https://myfundedfutures.com/terms", accessed: "2026-10-23" }
  - { field: automated_trading_allowed, url: "https://help.myfundedfutures.com/en/articles/8444599-fair-play-and-prohibited-trading-practices", accessed: "2026-10-23" }
  - { field: trading_day_reset, url: "https://help.myfundedfutures.com/en/articles/9558251-permitted-times-to-trade", accessed: "2026-10-23" }
  - { field: platforms, url: "https://help.myfundedfutures.com/en/articles/8528335-overview-of-supported-platforms-at-mffu", accessed: "2026-10-23" }
```

#### Rapid — evaluation — 100K

```yaml
firm: myfundedfutures
plan: "Rapid"
phase: evaluation
size_label: "100K"
size_usd: "100000.00"
instruments: [MNQ, NQ]
platforms: [Tradovate, NinjaTrader, TradingView, Quantower, ATAS, DeepChart/DeepDom, Volumetrica, Hyperprop]
drawdown: { amount_usd: "3000.00", type: trailing_eod, stops_trailing_at: "EOD article: 'the Max EOD trailing locks in at $100 plus the initial starting balance' (no size-specific lock published for this Rapid size)" }
daily_loss_limit: { amount_usd: null, kind: none }
profit_target_usd: "6000.00"
consistency_rule: { text: "No single day's profit may exceed 50% of total evaluation profits; exceeding it does not breach, trader must trade more days", threshold_pct: "50" }
max_position: { minis: 8, micros: 80, scaling: "No scaling plan published" }
min_trading_days: 2
trading_day_reset: { time: null, tz_as_published: null }   # no explicit reset published; session "6:00 PM EST" open, positions auto-closed "4:10 PM EST" (see §4)
copy_trading_allowed: conditional   # copiers allowed on all account types; copying other traders and multi-account hedge/mirror/copy for unfair advantage prohibited (see §5)
automated_trading_allowed: conditional   # allowed unless HFT or aimed at exploiting sim fills; live must follow CME guidelines (see §6)
automation_rule_text: "High-frequency Trading is not allowed on our plans. Traders may make use of automated trading strategies tailored to their own specific settings so long as these automated tools do not aim to exploit the favorable fills offered in the Simulated Environment. Traders trading live accounts with automated trading must abide by CME guidelines."
vps_allowed: unknown   # VPS/server/VPN not mentioned on any official page (see §7a)
personal_device_required: unknown   # not required in words; shared devices with ANOTHER trader are forbidden (see §7a)
notes: "Max contracts in evaluation published only in the help center / Traders Evaluation Simplified (plan-page table has no eval contract row; the size selector shows the same numbers). T1 news allowed in evaluation."
status: verified
not_confirmed_fields: []
sources:
  - { field: drawdown, url: "https://help.myfundedfutures.com/en/articles/13286542-rapid-plan-100k-a-comprehensive-look", accessed: "2026-10-23" }
  - { field: drawdown, url: "https://myfundedfutures.com/plans/rapid", accessed: "2026-10-23" }
  - { field: drawdown, url: "https://help.myfundedfutures.com/en/articles/8348565-end-of-day-eod-drawdown-explained", accessed: "2026-10-23" }
  - { field: daily_loss_limit, url: "https://help.myfundedfutures.com/en/articles/13286542-rapid-plan-100k-a-comprehensive-look", accessed: "2026-10-23" }
  - { field: profit_target_usd, url: "https://help.myfundedfutures.com/en/articles/13286542-rapid-plan-100k-a-comprehensive-look", accessed: "2026-10-23" }
  - { field: consistency_rule, url: "https://help.myfundedfutures.com/en/articles/11994562-consistency-rule-at-my-fundedfutures", accessed: "2026-10-23" }
  - { field: max_position, url: "https://help.myfundedfutures.com/en/articles/13286542-rapid-plan-100k-a-comprehensive-look", accessed: "2026-10-23" }
  - { field: max_position, url: "https://help.myfundedfutures.com/en/articles/11802636-traders-evaluation-simplified", accessed: "2026-10-23" }
  - { field: min_trading_days, url: "https://help.myfundedfutures.com/en/articles/13286542-rapid-plan-100k-a-comprehensive-look", accessed: "2026-10-23" }
  - { field: copy_trading_allowed, url: "https://help.myfundedfutures.com/en/articles/10771500-copy-trading-at-myfundedfutures", accessed: "2026-10-23" }
  - { field: copy_trading_allowed, url: "https://myfundedfutures.com/terms", accessed: "2026-10-23" }
  - { field: automated_trading_allowed, url: "https://help.myfundedfutures.com/en/articles/8444599-fair-play-and-prohibited-trading-practices", accessed: "2026-10-23" }
  - { field: trading_day_reset, url: "https://help.myfundedfutures.com/en/articles/9558251-permitted-times-to-trade", accessed: "2026-10-23" }
  - { field: platforms, url: "https://help.myfundedfutures.com/en/articles/8528335-overview-of-supported-platforms-at-mffu", accessed: "2026-10-23" }
```

#### Rapid — funded — 100K

```yaml
firm: myfundedfutures
plan: "Rapid"
phase: funded
size_label: "100K"
size_usd: "100000.00"
instruments: [MNQ, NQ]
platforms: [Tradovate, NinjaTrader, TradingView, Quantower, ATAS, DeepChart/DeepDom, Volumetrica, Hyperprop]
drawdown: { amount_usd: "3000.00", type: trailing_intraday, stops_trailing_at: "conflict: help center 'Once your trailing Max Loss reaches $100, it locks there' vs plan-page FAQ 'once your balance is +$100 above your starting balance after your first payout, the floor locks'" }
daily_loss_limit: { amount_usd: null, kind: none }
profit_target_usd: null
consistency_rule: { text: "None in Sim Funded", threshold_pct: null }
max_position: { minis: 8, micros: 80, scaling: "No scaling plan published" }
min_trading_days: null
trading_day_reset: { time: null, tz_as_published: null }   # no explicit reset published; session "6:00 PM EST" open, positions auto-closed "4:10 PM EST" (see §4)
copy_trading_allowed: conditional   # copiers allowed on all account types; copying other traders and multi-account hedge/mirror/copy for unfair advantage prohibited (see §5)
automated_trading_allowed: conditional   # allowed unless HFT or aimed at exploiting sim fills; live must follow CME guidelines (see §6)
automation_rule_text: "High-frequency Trading is not allowed on our plans. Traders may make use of automated trading strategies tailored to their own specific settings so long as these automated tools do not aim to exploit the favorable fills offered in the Simulated Environment. Traders trading live accounts with automated trading must abide by CME guidelines."
vps_allowed: unknown   # VPS/server/VPN not mentioned on any official page (see §7a)
personal_device_required: unknown   # not required in words; shared devices with ANOTHER trader are forbidden (see §7a)
notes: "Sim Funded starts at $0; Max Loss Distance 3000.00 from equity high-water mark, trails intraday incl. unrealized. Buffer 3100.00; payouts daily; T1 news NOT allowed (2 minutes before/after). Holding overnight breaches. Account limits: up to 5 active Sim Funded if only 25K/50K; 3 total if any 100K/150K."
status: conflict
not_confirmed_fields: [drawdown.stops_trailing_at]
sources:
  - { field: drawdown, url: "https://help.myfundedfutures.com/en/articles/13286542-rapid-plan-100k-a-comprehensive-look", accessed: "2026-10-23" }
  - { field: drawdown, url: "https://myfundedfutures.com/plans/rapid", accessed: "2026-10-23" }
  - { field: drawdown, url: "https://help.myfundedfutures.com/en/articles/12802721-intraday-drawdown-explained", accessed: "2026-10-23" }
  - { field: daily_loss_limit, url: "https://help.myfundedfutures.com/en/articles/13286542-rapid-plan-100k-a-comprehensive-look", accessed: "2026-10-23" }
  - { field: consistency_rule, url: "https://help.myfundedfutures.com/en/articles/13286542-rapid-plan-100k-a-comprehensive-look", accessed: "2026-10-23" }
  - { field: max_position, url: "https://help.myfundedfutures.com/en/articles/13286542-rapid-plan-100k-a-comprehensive-look", accessed: "2026-10-23" }
  - { field: max_position, url: "https://myfundedfutures.com/plans/rapid", accessed: "2026-10-23" }
  - { field: max_position, url: "https://help.myfundedfutures.com/en/articles/10244682-cross-instrument-policy", accessed: "2026-10-23" }
  - { field: max_accounts, url: "https://help.myfundedfutures.com/en/articles/16498635-moving-from-evaluation-to-sim-funded-account", accessed: "2026-10-23" }
  - { field: copy_trading_allowed, url: "https://help.myfundedfutures.com/en/articles/10771500-copy-trading-at-myfundedfutures", accessed: "2026-10-23" }
  - { field: copy_trading_allowed, url: "https://myfundedfutures.com/terms", accessed: "2026-10-23" }
  - { field: automated_trading_allowed, url: "https://help.myfundedfutures.com/en/articles/8444599-fair-play-and-prohibited-trading-practices", accessed: "2026-10-23" }
  - { field: trading_day_reset, url: "https://help.myfundedfutures.com/en/articles/9558251-permitted-times-to-trade", accessed: "2026-10-23" }
  - { field: platforms, url: "https://help.myfundedfutures.com/en/articles/8528335-overview-of-supported-platforms-at-mffu", accessed: "2026-10-23" }
```

#### Rapid — live — 100K

```yaml
firm: myfundedfutures
plan: "Rapid"
phase: live
size_label: "100K"
size_usd: "100000.00"
instruments: [MNQ, NQ]
platforms: []   # live platform not published
drawdown: { amount_usd: "3000.00", type: trailing_eod, stops_trailing_at: "Max Loss threshold stops at $0 (Live starts at $0)" }
daily_loss_limit: { amount_usd: null, kind: null }
profit_target_usd: null
consistency_rule: { text: null, threshold_pct: null }
max_position: { minis: 6, micros: 60, scaling: "Multiple accounts moved to Live are combined into one Live account; max contracts then discussed with the Live team" }
min_trading_days: null
trading_day_reset: { time: null, tz_as_published: null }   # no explicit reset published; session "6:00 PM EST" open, positions auto-closed "4:10 PM EST" (see §4)
copy_trading_allowed: conditional   # copiers allowed on all account types; copying other traders and multi-account hedge/mirror/copy for unfair advantage prohibited (see §5)
automated_trading_allowed: conditional   # allowed unless HFT or aimed at exploiting sim fills; live must follow CME guidelines (see §6)
automation_rule_text: "High-frequency Trading is not allowed on our plans. Traders may make use of automated trading strategies tailored to their own specific settings so long as these automated tools do not aim to exploit the favorable fills offered in the Simulated Environment. Traders trading live accounts with automated trading must abide by CME guidelines."
vps_allowed: unknown   # VPS/server/VPN not mentioned on any official page (see §7a)
personal_device_required: unknown   # not required in words; shared devices with ANOTHER trader are forbidden (see §7a)
notes: "Live DLL not stated on the Rapid Live page (null = not found; generic live article mentions a 'Customizable Daily Loss Limit'). Transition: $10,000 net profit in a single day (automatic) or Risk Team review. Live trading is user-only."
status: partial
not_confirmed_fields: []
sources:
  - { field: drawdown, url: "https://help.myfundedfutures.com/en/articles/13134718-understanding-rapid-live", accessed: "2026-10-23" }
  - { field: drawdown, url: "https://myfundedfutures.com/plans/rapid", accessed: "2026-10-23" }
  - { field: max_position, url: "https://help.myfundedfutures.com/en/articles/13134718-understanding-rapid-live", accessed: "2026-10-23" }
  - { field: max_position, url: "https://myfundedfutures.com/plans/rapid", accessed: "2026-10-23" }
  - { field: copy_trading_allowed, url: "https://help.myfundedfutures.com/en/articles/10771500-copy-trading-at-myfundedfutures", accessed: "2026-10-23" }
  - { field: copy_trading_allowed, url: "https://myfundedfutures.com/terms", accessed: "2026-10-23" }
  - { field: automated_trading_allowed, url: "https://help.myfundedfutures.com/en/articles/8444599-fair-play-and-prohibited-trading-practices", accessed: "2026-10-23" }
  - { field: trading_day_reset, url: "https://help.myfundedfutures.com/en/articles/9558251-permitted-times-to-trade", accessed: "2026-10-23" }
  - { field: platforms, url: "https://help.myfundedfutures.com/en/articles/8528335-overview-of-supported-platforms-at-mffu", accessed: "2026-10-23" }
```

#### Rapid — evaluation — 150K

```yaml
firm: myfundedfutures
plan: "Rapid"
phase: evaluation
size_label: "150K"
size_usd: "150000.00"
instruments: [MNQ, NQ]
platforms: [Tradovate, NinjaTrader, TradingView, Quantower, ATAS, DeepChart/DeepDom, Volumetrica, Hyperprop]
drawdown: { amount_usd: "4500.00", type: trailing_eod, stops_trailing_at: "EOD article: 'the Max EOD trailing locks in at $100 plus the initial starting balance' (no size-specific lock published for this Rapid size)" }
daily_loss_limit: { amount_usd: null, kind: none }
profit_target_usd: "9000.00"
consistency_rule: { text: "No single day's profit may exceed 50% of total evaluation profits; exceeding it does not breach, trader must trade more days", threshold_pct: "50" }
max_position: { minis: 10, micros: 100, scaling: "No scaling plan published" }
min_trading_days: 2
trading_day_reset: { time: null, tz_as_published: null }   # no explicit reset published; session "6:00 PM EST" open, positions auto-closed "4:10 PM EST" (see §4)
copy_trading_allowed: conditional   # copiers allowed on all account types; copying other traders and multi-account hedge/mirror/copy for unfair advantage prohibited (see §5)
automated_trading_allowed: conditional   # allowed unless HFT or aimed at exploiting sim fills; live must follow CME guidelines (see §6)
automation_rule_text: "High-frequency Trading is not allowed on our plans. Traders may make use of automated trading strategies tailored to their own specific settings so long as these automated tools do not aim to exploit the favorable fills offered in the Simulated Environment. Traders trading live accounts with automated trading must abide by CME guidelines."
vps_allowed: unknown   # VPS/server/VPN not mentioned on any official page (see §7a)
personal_device_required: unknown   # not required in words; shared devices with ANOTHER trader are forbidden (see §7a)
notes: "Max contracts in evaluation published only in the help center / Traders Evaluation Simplified (plan-page table has no eval contract row; the size selector shows the same numbers). T1 news allowed in evaluation."
status: verified
not_confirmed_fields: []
sources:
  - { field: drawdown, url: "https://help.myfundedfutures.com/en/articles/13286582-rapid-plan-150k-a-comprehensive-look", accessed: "2026-10-23" }
  - { field: drawdown, url: "https://myfundedfutures.com/plans/rapid", accessed: "2026-10-23" }
  - { field: drawdown, url: "https://help.myfundedfutures.com/en/articles/8348565-end-of-day-eod-drawdown-explained", accessed: "2026-10-23" }
  - { field: daily_loss_limit, url: "https://help.myfundedfutures.com/en/articles/13286582-rapid-plan-150k-a-comprehensive-look", accessed: "2026-10-23" }
  - { field: profit_target_usd, url: "https://help.myfundedfutures.com/en/articles/13286582-rapid-plan-150k-a-comprehensive-look", accessed: "2026-10-23" }
  - { field: consistency_rule, url: "https://help.myfundedfutures.com/en/articles/11994562-consistency-rule-at-my-fundedfutures", accessed: "2026-10-23" }
  - { field: max_position, url: "https://help.myfundedfutures.com/en/articles/13286582-rapid-plan-150k-a-comprehensive-look", accessed: "2026-10-23" }
  - { field: max_position, url: "https://help.myfundedfutures.com/en/articles/11802636-traders-evaluation-simplified", accessed: "2026-10-23" }
  - { field: min_trading_days, url: "https://help.myfundedfutures.com/en/articles/13286582-rapid-plan-150k-a-comprehensive-look", accessed: "2026-10-23" }
  - { field: copy_trading_allowed, url: "https://help.myfundedfutures.com/en/articles/10771500-copy-trading-at-myfundedfutures", accessed: "2026-10-23" }
  - { field: copy_trading_allowed, url: "https://myfundedfutures.com/terms", accessed: "2026-10-23" }
  - { field: automated_trading_allowed, url: "https://help.myfundedfutures.com/en/articles/8444599-fair-play-and-prohibited-trading-practices", accessed: "2026-10-23" }
  - { field: trading_day_reset, url: "https://help.myfundedfutures.com/en/articles/9558251-permitted-times-to-trade", accessed: "2026-10-23" }
  - { field: platforms, url: "https://help.myfundedfutures.com/en/articles/8528335-overview-of-supported-platforms-at-mffu", accessed: "2026-10-23" }
```

#### Rapid — funded — 150K

```yaml
firm: myfundedfutures
plan: "Rapid"
phase: funded
size_label: "150K"
size_usd: "150000.00"
instruments: [MNQ, NQ]
platforms: [Tradovate, NinjaTrader, TradingView, Quantower, ATAS, DeepChart/DeepDom, Volumetrica, Hyperprop]
drawdown: { amount_usd: "4500.00", type: trailing_intraday, stops_trailing_at: "conflict: help center 'Once your trailing Max Loss reaches $100, it locks there' vs plan-page FAQ 'once your balance is +$100 above your starting balance after your first payout, the floor locks'" }
daily_loss_limit: { amount_usd: null, kind: none }
profit_target_usd: null
consistency_rule: { text: "None in Sim Funded", threshold_pct: null }
max_position: { minis: 10, micros: 100, scaling: "No scaling plan published" }
min_trading_days: null
trading_day_reset: { time: null, tz_as_published: null }   # no explicit reset published; session "6:00 PM EST" open, positions auto-closed "4:10 PM EST" (see §4)
copy_trading_allowed: conditional   # copiers allowed on all account types; copying other traders and multi-account hedge/mirror/copy for unfair advantage prohibited (see §5)
automated_trading_allowed: conditional   # allowed unless HFT or aimed at exploiting sim fills; live must follow CME guidelines (see §6)
automation_rule_text: "High-frequency Trading is not allowed on our plans. Traders may make use of automated trading strategies tailored to their own specific settings so long as these automated tools do not aim to exploit the favorable fills offered in the Simulated Environment. Traders trading live accounts with automated trading must abide by CME guidelines."
vps_allowed: unknown   # VPS/server/VPN not mentioned on any official page (see §7a)
personal_device_required: unknown   # not required in words; shared devices with ANOTHER trader are forbidden (see §7a)
notes: "Sim Funded starts at $0; Max Loss Distance 4500.00 from equity high-water mark, trails intraday incl. unrealized. Buffer 4600.00; payouts daily; T1 news NOT allowed (2 minutes before/after). Holding overnight breaches. Account limits: up to 5 active Sim Funded if only 25K/50K; 3 total if any 100K/150K."
status: conflict
not_confirmed_fields: [drawdown.stops_trailing_at]
sources:
  - { field: drawdown, url: "https://help.myfundedfutures.com/en/articles/13286582-rapid-plan-150k-a-comprehensive-look", accessed: "2026-10-23" }
  - { field: drawdown, url: "https://myfundedfutures.com/plans/rapid", accessed: "2026-10-23" }
  - { field: drawdown, url: "https://help.myfundedfutures.com/en/articles/12802721-intraday-drawdown-explained", accessed: "2026-10-23" }
  - { field: daily_loss_limit, url: "https://help.myfundedfutures.com/en/articles/13286582-rapid-plan-150k-a-comprehensive-look", accessed: "2026-10-23" }
  - { field: consistency_rule, url: "https://help.myfundedfutures.com/en/articles/13286582-rapid-plan-150k-a-comprehensive-look", accessed: "2026-10-23" }
  - { field: max_position, url: "https://help.myfundedfutures.com/en/articles/13286582-rapid-plan-150k-a-comprehensive-look", accessed: "2026-10-23" }
  - { field: max_position, url: "https://myfundedfutures.com/plans/rapid", accessed: "2026-10-23" }
  - { field: max_position, url: "https://help.myfundedfutures.com/en/articles/10244682-cross-instrument-policy", accessed: "2026-10-23" }
  - { field: max_accounts, url: "https://help.myfundedfutures.com/en/articles/16498635-moving-from-evaluation-to-sim-funded-account", accessed: "2026-10-23" }
  - { field: copy_trading_allowed, url: "https://help.myfundedfutures.com/en/articles/10771500-copy-trading-at-myfundedfutures", accessed: "2026-10-23" }
  - { field: copy_trading_allowed, url: "https://myfundedfutures.com/terms", accessed: "2026-10-23" }
  - { field: automated_trading_allowed, url: "https://help.myfundedfutures.com/en/articles/8444599-fair-play-and-prohibited-trading-practices", accessed: "2026-10-23" }
  - { field: trading_day_reset, url: "https://help.myfundedfutures.com/en/articles/9558251-permitted-times-to-trade", accessed: "2026-10-23" }
  - { field: platforms, url: "https://help.myfundedfutures.com/en/articles/8528335-overview-of-supported-platforms-at-mffu", accessed: "2026-10-23" }
```

#### Rapid — live — 150K

```yaml
firm: myfundedfutures
plan: "Rapid"
phase: live
size_label: "150K"
size_usd: "150000.00"
instruments: [MNQ, NQ]
platforms: []   # live platform not published
drawdown: { amount_usd: "4500.00", type: trailing_eod, stops_trailing_at: "Max Loss threshold stops at $0 (Live starts at $0)" }
daily_loss_limit: { amount_usd: null, kind: null }
profit_target_usd: null
consistency_rule: { text: null, threshold_pct: null }
max_position: { minis: 8, micros: 80, scaling: "Multiple accounts moved to Live are combined into one Live account; max contracts then discussed with the Live team" }
min_trading_days: null
trading_day_reset: { time: null, tz_as_published: null }   # no explicit reset published; session "6:00 PM EST" open, positions auto-closed "4:10 PM EST" (see §4)
copy_trading_allowed: conditional   # copiers allowed on all account types; copying other traders and multi-account hedge/mirror/copy for unfair advantage prohibited (see §5)
automated_trading_allowed: conditional   # allowed unless HFT or aimed at exploiting sim fills; live must follow CME guidelines (see §6)
automation_rule_text: "High-frequency Trading is not allowed on our plans. Traders may make use of automated trading strategies tailored to their own specific settings so long as these automated tools do not aim to exploit the favorable fills offered in the Simulated Environment. Traders trading live accounts with automated trading must abide by CME guidelines."
vps_allowed: unknown   # VPS/server/VPN not mentioned on any official page (see §7a)
personal_device_required: unknown   # not required in words; shared devices with ANOTHER trader are forbidden (see §7a)
notes: "Live DLL not stated on the Rapid Live page (null = not found; generic live article mentions a 'Customizable Daily Loss Limit'). Transition: $10,000 net profit in a single day (automatic) or Risk Team review. Live trading is user-only."
status: partial
not_confirmed_fields: []
sources:
  - { field: drawdown, url: "https://help.myfundedfutures.com/en/articles/13134718-understanding-rapid-live", accessed: "2026-10-23" }
  - { field: drawdown, url: "https://myfundedfutures.com/plans/rapid", accessed: "2026-10-23" }
  - { field: max_position, url: "https://help.myfundedfutures.com/en/articles/13134718-understanding-rapid-live", accessed: "2026-10-23" }
  - { field: max_position, url: "https://myfundedfutures.com/plans/rapid", accessed: "2026-10-23" }
  - { field: copy_trading_allowed, url: "https://help.myfundedfutures.com/en/articles/10771500-copy-trading-at-myfundedfutures", accessed: "2026-10-23" }
  - { field: copy_trading_allowed, url: "https://myfundedfutures.com/terms", accessed: "2026-10-23" }
  - { field: automated_trading_allowed, url: "https://help.myfundedfutures.com/en/articles/8444599-fair-play-and-prohibited-trading-practices", accessed: "2026-10-23" }
  - { field: trading_day_reset, url: "https://help.myfundedfutures.com/en/articles/9558251-permitted-times-to-trade", accessed: "2026-10-23" }
  - { field: platforms, url: "https://help.myfundedfutures.com/en/articles/8528335-overview-of-supported-platforms-at-mffu", accessed: "2026-10-23" }
```

#### Rapid EOD — evaluation — 25K

```yaml
firm: myfundedfutures
plan: "Rapid EOD"
phase: evaluation
size_label: "25K"
size_usd: "25000.00"
instruments: [MNQ, NQ]
platforms: [Tradovate, NinjaTrader, TradingView, Quantower, ATAS, DeepChart/DeepDom, Volumetrica, Hyperprop]
drawdown: { amount_usd: "1000.00", type: trailing_eod, stops_trailing_at: "EOD article: 'the Max EOD trailing locks in at $100 plus the initial starting balance'" }
daily_loss_limit: { amount_usd: null, kind: none }
profit_target_usd: "1500.00"
consistency_rule: { text: "No single day's profit may exceed 30% of total evaluation profits", threshold_pct: "30" }
max_position: { minis: 2, micros: 20, scaling: "No scaling plan published" }
min_trading_days: 4
trading_day_reset: { time: null, tz_as_published: null }   # no explicit reset published; session "6:00 PM EST" open, positions auto-closed "4:10 PM EST" (see §4)
copy_trading_allowed: conditional   # copiers allowed on all account types; copying other traders and multi-account hedge/mirror/copy for unfair advantage prohibited (see §5)
automated_trading_allowed: conditional   # allowed unless HFT or aimed at exploiting sim fills; live must follow CME guidelines (see §6)
automation_rule_text: "High-frequency Trading is not allowed on our plans. Traders may make use of automated trading strategies tailored to their own specific settings so long as these automated tools do not aim to exploit the favorable fills offered in the Simulated Environment. Traders trading live accounts with automated trading must abide by CME guidelines."
vps_allowed: unknown   # VPS/server/VPN not mentioned on any official page (see §7a)
personal_device_required: unknown   # not required in words; shared devices with ANOTHER trader are forbidden (see §7a)
notes: "News allowed in evaluation. Inactivity 7 days."
status: verified
not_confirmed_fields: []
sources:
  - { field: drawdown, url: "https://help.myfundedfutures.com/en/articles/16727601-rapid-eod-25k-a-comprehensive-look", accessed: "2026-10-23" }
  - { field: drawdown, url: "https://myfundedfutures.com/plans/rapid-eod", accessed: "2026-10-23" }
  - { field: drawdown, url: "https://help.myfundedfutures.com/en/articles/8348565-end-of-day-eod-drawdown-explained", accessed: "2026-10-23" }
  - { field: daily_loss_limit, url: "https://help.myfundedfutures.com/en/articles/16727601-rapid-eod-25k-a-comprehensive-look", accessed: "2026-10-23" }
  - { field: profit_target_usd, url: "https://help.myfundedfutures.com/en/articles/16727601-rapid-eod-25k-a-comprehensive-look", accessed: "2026-10-23" }
  - { field: consistency_rule, url: "https://help.myfundedfutures.com/en/articles/16727601-rapid-eod-25k-a-comprehensive-look", accessed: "2026-10-23" }
  - { field: consistency_rule, url: "https://help.myfundedfutures.com/en/articles/11994562-consistency-rule-at-my-fundedfutures", accessed: "2026-10-23" }
  - { field: max_position, url: "https://help.myfundedfutures.com/en/articles/16727601-rapid-eod-25k-a-comprehensive-look", accessed: "2026-10-23" }
  - { field: min_trading_days, url: "https://help.myfundedfutures.com/en/articles/16727601-rapid-eod-25k-a-comprehensive-look", accessed: "2026-10-23" }
  - { field: copy_trading_allowed, url: "https://help.myfundedfutures.com/en/articles/10771500-copy-trading-at-myfundedfutures", accessed: "2026-10-23" }
  - { field: copy_trading_allowed, url: "https://myfundedfutures.com/terms", accessed: "2026-10-23" }
  - { field: automated_trading_allowed, url: "https://help.myfundedfutures.com/en/articles/8444599-fair-play-and-prohibited-trading-practices", accessed: "2026-10-23" }
  - { field: trading_day_reset, url: "https://help.myfundedfutures.com/en/articles/9558251-permitted-times-to-trade", accessed: "2026-10-23" }
  - { field: platforms, url: "https://help.myfundedfutures.com/en/articles/8528335-overview-of-supported-platforms-at-mffu", accessed: "2026-10-23" }
```

#### Rapid EOD — funded — 25K

```yaml
firm: myfundedfutures
plan: "Rapid EOD"
phase: funded
size_label: "25K"
size_usd: "25000.00"
instruments: [MNQ, NQ]
platforms: [Tradovate, NinjaTrader, TradingView, Quantower, ATAS, DeepChart/DeepDom, Volumetrica, Hyperprop]
drawdown: { amount_usd: "1000.00", type: trailing_eod, stops_trailing_at: "Sim Funded starts at $0; once the trailing Max Loss reaches +$100 it locks there" }
daily_loss_limit: { amount_usd: null, kind: none }
profit_target_usd: null
consistency_rule: { text: "None", threshold_pct: null }
max_position: { minis: 2, micros: 20, scaling: "No scaling plan published" }
min_trading_days: null
trading_day_reset: { time: null, tz_as_published: null }   # no explicit reset published; session "6:00 PM EST" open, positions auto-closed "4:10 PM EST" (see §4)
copy_trading_allowed: conditional   # copiers allowed on all account types; copying other traders and multi-account hedge/mirror/copy for unfair advantage prohibited (see §5)
automated_trading_allowed: conditional   # allowed unless HFT or aimed at exploiting sim fills; live must follow CME guidelines (see §6)
automation_rule_text: "High-frequency Trading is not allowed on our plans. Traders may make use of automated trading strategies tailored to their own specific settings so long as these automated tools do not aim to exploit the favorable fills offered in the Simulated Environment. Traders trading live accounts with automated trading must abide by CME guidelines."
vps_allowed: unknown   # VPS/server/VPN not mentioned on any official page (see §7a)
personal_device_required: unknown   # not required in words; shared devices with ANOTHER trader are forbidden (see §7a)
notes: "Buffer 1100.00; then each payout unlocks at $250 net profit since last payout; no cap per cycle. News/T1 news NOT allowed in Sim Funded. Max 3 Sim Funded accounts (Rapid EOD)."
status: verified
not_confirmed_fields: []
sources:
  - { field: drawdown, url: "https://help.myfundedfutures.com/en/articles/16727601-rapid-eod-25k-a-comprehensive-look", accessed: "2026-10-23" }
  - { field: drawdown, url: "https://myfundedfutures.com/plans/rapid-eod", accessed: "2026-10-23" }
  - { field: daily_loss_limit, url: "https://help.myfundedfutures.com/en/articles/16727601-rapid-eod-25k-a-comprehensive-look", accessed: "2026-10-23" }
  - { field: max_position, url: "https://help.myfundedfutures.com/en/articles/16727601-rapid-eod-25k-a-comprehensive-look", accessed: "2026-10-23" }
  - { field: max_position, url: "https://myfundedfutures.com/plans/rapid-eod", accessed: "2026-10-23" }
  - { field: max_accounts, url: "https://help.myfundedfutures.com/en/articles/16498635-moving-from-evaluation-to-sim-funded-account", accessed: "2026-10-23" }
  - { field: copy_trading_allowed, url: "https://help.myfundedfutures.com/en/articles/10771500-copy-trading-at-myfundedfutures", accessed: "2026-10-23" }
  - { field: copy_trading_allowed, url: "https://myfundedfutures.com/terms", accessed: "2026-10-23" }
  - { field: automated_trading_allowed, url: "https://help.myfundedfutures.com/en/articles/8444599-fair-play-and-prohibited-trading-practices", accessed: "2026-10-23" }
  - { field: trading_day_reset, url: "https://help.myfundedfutures.com/en/articles/9558251-permitted-times-to-trade", accessed: "2026-10-23" }
  - { field: platforms, url: "https://help.myfundedfutures.com/en/articles/8528335-overview-of-supported-platforms-at-mffu", accessed: "2026-10-23" }
```

#### Rapid EOD — live — 25K

```yaml
firm: myfundedfutures
plan: "Rapid EOD"
phase: live
size_label: "25K"
size_usd: "25000.00"
instruments: [MNQ, NQ]
platforms: []   # live platform not published
drawdown: { amount_usd: "1000.00", type: trailing_eod, stops_trailing_at: "Max Loss threshold stops at $0" }
daily_loss_limit: { amount_usd: null, kind: null }
profit_target_usd: null
consistency_rule: { text: null, threshold_pct: null }
max_position: { minis: 2, micros: 20, scaling: "See Rapid live" }
min_trading_days: null
trading_day_reset: { time: null, tz_as_published: null }   # no explicit reset published; session "6:00 PM EST" open, positions auto-closed "4:10 PM EST" (see §4)
copy_trading_allowed: conditional   # copiers allowed on all account types; copying other traders and multi-account hedge/mirror/copy for unfair advantage prohibited (see §5)
automated_trading_allowed: conditional   # allowed unless HFT or aimed at exploiting sim fills; live must follow CME guidelines (see §6)
automation_rule_text: "High-frequency Trading is not allowed on our plans. Traders may make use of automated trading strategies tailored to their own specific settings so long as these automated tools do not aim to exploit the favorable fills offered in the Simulated Environment. Traders trading live accounts with automated trading must abide by CME guidelines."
vps_allowed: unknown   # VPS/server/VPN not mentioned on any official page (see §7a)
personal_device_required: unknown   # not required in words; shared devices with ANOTHER trader are forbidden (see §7a)
notes: "Live follows standard Rapid 25K (2 mini / 20 micro), plan page also 2 / 20. Live DLL not stated. Live trading is user-only."
status: partial
not_confirmed_fields: []
sources:
  - { field: drawdown, url: "https://myfundedfutures.com/plans/rapid-eod", accessed: "2026-10-23" }
  - { field: drawdown, url: "https://help.myfundedfutures.com/en/articles/16727601-rapid-eod-25k-a-comprehensive-look", accessed: "2026-10-23" }
  - { field: max_position, url: "https://myfundedfutures.com/plans/rapid-eod", accessed: "2026-10-23" }
  - { field: max_position, url: "https://help.myfundedfutures.com/en/articles/13134718-understanding-rapid-live", accessed: "2026-10-23" }
  - { field: copy_trading_allowed, url: "https://help.myfundedfutures.com/en/articles/10771500-copy-trading-at-myfundedfutures", accessed: "2026-10-23" }
  - { field: copy_trading_allowed, url: "https://myfundedfutures.com/terms", accessed: "2026-10-23" }
  - { field: automated_trading_allowed, url: "https://help.myfundedfutures.com/en/articles/8444599-fair-play-and-prohibited-trading-practices", accessed: "2026-10-23" }
  - { field: trading_day_reset, url: "https://help.myfundedfutures.com/en/articles/9558251-permitted-times-to-trade", accessed: "2026-10-23" }
  - { field: platforms, url: "https://help.myfundedfutures.com/en/articles/8528335-overview-of-supported-platforms-at-mffu", accessed: "2026-10-23" }
```

#### Rapid EOD — evaluation — 50K

```yaml
firm: myfundedfutures
plan: "Rapid EOD"
phase: evaluation
size_label: "50K"
size_usd: "50000.00"
instruments: [MNQ, NQ]
platforms: [Tradovate, NinjaTrader, TradingView, Quantower, ATAS, DeepChart/DeepDom, Volumetrica, Hyperprop]
drawdown: { amount_usd: "2000.00", type: trailing_eod, stops_trailing_at: "EOD article: 'the Max EOD trailing locks in at $100 plus the initial starting balance'" }
daily_loss_limit: { amount_usd: null, kind: none }
profit_target_usd: "3000.00"
consistency_rule: { text: "No single day's profit may exceed 30% of total evaluation profits", threshold_pct: "30" }
max_position: { minis: 3, micros: 30, scaling: "No scaling plan published" }
min_trading_days: 4
trading_day_reset: { time: null, tz_as_published: null }   # no explicit reset published; session "6:00 PM EST" open, positions auto-closed "4:10 PM EST" (see §4)
copy_trading_allowed: conditional   # copiers allowed on all account types; copying other traders and multi-account hedge/mirror/copy for unfair advantage prohibited (see §5)
automated_trading_allowed: conditional   # allowed unless HFT or aimed at exploiting sim fills; live must follow CME guidelines (see §6)
automation_rule_text: "High-frequency Trading is not allowed on our plans. Traders may make use of automated trading strategies tailored to their own specific settings so long as these automated tools do not aim to exploit the favorable fills offered in the Simulated Environment. Traders trading live accounts with automated trading must abide by CME guidelines."
vps_allowed: unknown   # VPS/server/VPN not mentioned on any official page (see §7a)
personal_device_required: unknown   # not required in words; shared devices with ANOTHER trader are forbidden (see §7a)
notes: "News allowed in evaluation. Inactivity 7 days."
status: verified
not_confirmed_fields: []
sources:
  - { field: drawdown, url: "https://help.myfundedfutures.com/en/articles/16158363-rapid-eod-50k-a-comprehensive-look", accessed: "2026-10-23" }
  - { field: drawdown, url: "https://myfundedfutures.com/plans/rapid-eod", accessed: "2026-10-23" }
  - { field: drawdown, url: "https://help.myfundedfutures.com/en/articles/8348565-end-of-day-eod-drawdown-explained", accessed: "2026-10-23" }
  - { field: daily_loss_limit, url: "https://help.myfundedfutures.com/en/articles/16158363-rapid-eod-50k-a-comprehensive-look", accessed: "2026-10-23" }
  - { field: profit_target_usd, url: "https://help.myfundedfutures.com/en/articles/16158363-rapid-eod-50k-a-comprehensive-look", accessed: "2026-10-23" }
  - { field: consistency_rule, url: "https://help.myfundedfutures.com/en/articles/16158363-rapid-eod-50k-a-comprehensive-look", accessed: "2026-10-23" }
  - { field: consistency_rule, url: "https://help.myfundedfutures.com/en/articles/11994562-consistency-rule-at-my-fundedfutures", accessed: "2026-10-23" }
  - { field: max_position, url: "https://help.myfundedfutures.com/en/articles/16158363-rapid-eod-50k-a-comprehensive-look", accessed: "2026-10-23" }
  - { field: min_trading_days, url: "https://help.myfundedfutures.com/en/articles/16158363-rapid-eod-50k-a-comprehensive-look", accessed: "2026-10-23" }
  - { field: copy_trading_allowed, url: "https://help.myfundedfutures.com/en/articles/10771500-copy-trading-at-myfundedfutures", accessed: "2026-10-23" }
  - { field: copy_trading_allowed, url: "https://myfundedfutures.com/terms", accessed: "2026-10-23" }
  - { field: automated_trading_allowed, url: "https://help.myfundedfutures.com/en/articles/8444599-fair-play-and-prohibited-trading-practices", accessed: "2026-10-23" }
  - { field: trading_day_reset, url: "https://help.myfundedfutures.com/en/articles/9558251-permitted-times-to-trade", accessed: "2026-10-23" }
  - { field: platforms, url: "https://help.myfundedfutures.com/en/articles/8528335-overview-of-supported-platforms-at-mffu", accessed: "2026-10-23" }
```

#### Rapid EOD — funded — 50K

```yaml
firm: myfundedfutures
plan: "Rapid EOD"
phase: funded
size_label: "50K"
size_usd: "50000.00"
instruments: [MNQ, NQ]
platforms: [Tradovate, NinjaTrader, TradingView, Quantower, ATAS, DeepChart/DeepDom, Volumetrica, Hyperprop]
drawdown: { amount_usd: "2000.00", type: trailing_eod, stops_trailing_at: "Sim Funded starts at $0; once the trailing Max Loss reaches +$100 it locks there" }
daily_loss_limit: { amount_usd: null, kind: none }
profit_target_usd: null
consistency_rule: { text: "None", threshold_pct: null }
max_position: { minis: 3, micros: 30, scaling: "No scaling plan published" }
min_trading_days: null
trading_day_reset: { time: null, tz_as_published: null }   # no explicit reset published; session "6:00 PM EST" open, positions auto-closed "4:10 PM EST" (see §4)
copy_trading_allowed: conditional   # copiers allowed on all account types; copying other traders and multi-account hedge/mirror/copy for unfair advantage prohibited (see §5)
automated_trading_allowed: conditional   # allowed unless HFT or aimed at exploiting sim fills; live must follow CME guidelines (see §6)
automation_rule_text: "High-frequency Trading is not allowed on our plans. Traders may make use of automated trading strategies tailored to their own specific settings so long as these automated tools do not aim to exploit the favorable fills offered in the Simulated Environment. Traders trading live accounts with automated trading must abide by CME guidelines."
vps_allowed: unknown   # VPS/server/VPN not mentioned on any official page (see §7a)
personal_device_required: unknown   # not required in words; shared devices with ANOTHER trader are forbidden (see §7a)
notes: "Buffer 2100.00; then each payout unlocks at $500 net profit since last payout; no cap per cycle. News/T1 news NOT allowed in Sim Funded. Max 3 Sim Funded accounts (Rapid EOD)."
status: verified
not_confirmed_fields: []
sources:
  - { field: drawdown, url: "https://help.myfundedfutures.com/en/articles/16158363-rapid-eod-50k-a-comprehensive-look", accessed: "2026-10-23" }
  - { field: drawdown, url: "https://myfundedfutures.com/plans/rapid-eod", accessed: "2026-10-23" }
  - { field: daily_loss_limit, url: "https://help.myfundedfutures.com/en/articles/16158363-rapid-eod-50k-a-comprehensive-look", accessed: "2026-10-23" }
  - { field: max_position, url: "https://help.myfundedfutures.com/en/articles/16158363-rapid-eod-50k-a-comprehensive-look", accessed: "2026-10-23" }
  - { field: max_position, url: "https://myfundedfutures.com/plans/rapid-eod", accessed: "2026-10-23" }
  - { field: max_accounts, url: "https://help.myfundedfutures.com/en/articles/16498635-moving-from-evaluation-to-sim-funded-account", accessed: "2026-10-23" }
  - { field: copy_trading_allowed, url: "https://help.myfundedfutures.com/en/articles/10771500-copy-trading-at-myfundedfutures", accessed: "2026-10-23" }
  - { field: copy_trading_allowed, url: "https://myfundedfutures.com/terms", accessed: "2026-10-23" }
  - { field: automated_trading_allowed, url: "https://help.myfundedfutures.com/en/articles/8444599-fair-play-and-prohibited-trading-practices", accessed: "2026-10-23" }
  - { field: trading_day_reset, url: "https://help.myfundedfutures.com/en/articles/9558251-permitted-times-to-trade", accessed: "2026-10-23" }
  - { field: platforms, url: "https://help.myfundedfutures.com/en/articles/8528335-overview-of-supported-platforms-at-mffu", accessed: "2026-10-23" }
```

#### Rapid EOD — live — 50K

```yaml
firm: myfundedfutures
plan: "Rapid EOD"
phase: live
size_label: "50K"
size_usd: "50000.00"
instruments: [MNQ, NQ]
platforms: []   # live platform not published
drawdown: { amount_usd: "2000.00", type: trailing_eod, stops_trailing_at: "Max Loss threshold stops at $0" }
daily_loss_limit: { amount_usd: null, kind: null }
profit_target_usd: null
consistency_rule: { text: null, threshold_pct: null }
max_position: { minis: null, micros: null, scaling: "See Rapid live" }
min_trading_days: null
trading_day_reset: { time: null, tz_as_published: null }   # no explicit reset published; session "6:00 PM EST" open, positions auto-closed "4:10 PM EST" (see §4)
copy_trading_allowed: conditional   # copiers allowed on all account types; copying other traders and multi-account hedge/mirror/copy for unfair advantage prohibited (see §5)
automated_trading_allowed: conditional   # allowed unless HFT or aimed at exploiting sim fills; live must follow CME guidelines (see §6)
automation_rule_text: "High-frequency Trading is not allowed on our plans. Traders may make use of automated trading strategies tailored to their own specific settings so long as these automated tools do not aim to exploit the favorable fills offered in the Simulated Environment. Traders trading live accounts with automated trading must abide by CME guidelines."
vps_allowed: unknown   # VPS/server/VPN not mentioned on any official page (see §7a)
personal_device_required: unknown   # not required in words; shared devices with ANOTHER trader are forbidden (see §7a)
notes: "Live max contracts conflict: Rapid EOD plan page '4 / 40' vs help center (Rapid EOD 50k follows standard Rapid 50k Live: '3 mini / 30 micro'). Live DLL not stated. Live trading is user-only."
status: conflict
not_confirmed_fields: [max_position]
sources:
  - { field: drawdown, url: "https://myfundedfutures.com/plans/rapid-eod", accessed: "2026-10-23" }
  - { field: drawdown, url: "https://help.myfundedfutures.com/en/articles/16158363-rapid-eod-50k-a-comprehensive-look", accessed: "2026-10-23" }
  - { field: max_position, url: "https://myfundedfutures.com/plans/rapid-eod", accessed: "2026-10-23" }
  - { field: max_position, url: "https://help.myfundedfutures.com/en/articles/13134718-understanding-rapid-live", accessed: "2026-10-23" }
  - { field: copy_trading_allowed, url: "https://help.myfundedfutures.com/en/articles/10771500-copy-trading-at-myfundedfutures", accessed: "2026-10-23" }
  - { field: copy_trading_allowed, url: "https://myfundedfutures.com/terms", accessed: "2026-10-23" }
  - { field: automated_trading_allowed, url: "https://help.myfundedfutures.com/en/articles/8444599-fair-play-and-prohibited-trading-practices", accessed: "2026-10-23" }
  - { field: trading_day_reset, url: "https://help.myfundedfutures.com/en/articles/9558251-permitted-times-to-trade", accessed: "2026-10-23" }
  - { field: platforms, url: "https://help.myfundedfutures.com/en/articles/8528335-overview-of-supported-platforms-at-mffu", accessed: "2026-10-23" }
```

#### Pro — evaluation — 50K

```yaml
firm: myfundedfutures
plan: "Pro"
phase: evaluation
size_label: "50K"
size_usd: "50000.00"
instruments: [MNQ, NQ]
platforms: [Tradovate, NinjaTrader, TradingView, Quantower, ATAS, DeepChart/DeepDom, Volumetrica, Hyperprop]
drawdown: { amount_usd: "2000.00", type: trailing_eod, stops_trailing_at: "EOD article: 'the Max EOD trailing locks in at $100 plus the initial starting balance'" }
daily_loss_limit: { amount_usd: null, kind: none }
profit_target_usd: "3000.00"
consistency_rule: { text: "No single day's profit may exceed 50% of total evaluation profits", threshold_pct: "50" }
max_position: { minis: 3, micros: 30, scaling: "No build-up; full allocation from day one (plan page)" }
min_trading_days: 2
trading_day_reset: { time: null, tz_as_published: null }   # no explicit reset published; session "6:00 PM EST" open, positions auto-closed "4:10 PM EST" (see §4)
copy_trading_allowed: conditional   # copiers allowed on all account types; copying other traders and multi-account hedge/mirror/copy for unfair advantage prohibited (see §5)
automated_trading_allowed: conditional   # allowed unless HFT or aimed at exploiting sim fills; live must follow CME guidelines (see §6)
automation_rule_text: "High-frequency Trading is not allowed on our plans. Traders may make use of automated trading strategies tailored to their own specific settings so long as these automated tools do not aim to exploit the favorable fills offered in the Simulated Environment. Traders trading live accounts with automated trading must abide by CME guidelines."
vps_allowed: unknown   # VPS/server/VPN not mentioned on any official page (see §7a)
personal_device_required: unknown   # not required in words; shared devices with ANOTHER trader are forbidden (see §7a)
notes: "T1 news allowed in evaluation."
status: verified
not_confirmed_fields: []
sources:
  - { field: drawdown, url: "https://help.myfundedfutures.com/en/articles/17573707-comprehensive-look-into-redesigned-pro-plan", accessed: "2026-10-23" }
  - { field: drawdown, url: "https://myfundedfutures.com/plans/pro", accessed: "2026-10-23" }
  - { field: drawdown, url: "https://help.myfundedfutures.com/en/articles/8348565-end-of-day-eod-drawdown-explained", accessed: "2026-10-23" }
  - { field: daily_loss_limit, url: "https://help.myfundedfutures.com/en/articles/17573707-comprehensive-look-into-redesigned-pro-plan", accessed: "2026-10-23" }
  - { field: profit_target_usd, url: "https://help.myfundedfutures.com/en/articles/17573707-comprehensive-look-into-redesigned-pro-plan", accessed: "2026-10-23" }
  - { field: consistency_rule, url: "https://help.myfundedfutures.com/en/articles/17573707-comprehensive-look-into-redesigned-pro-plan", accessed: "2026-10-23" }
  - { field: max_position, url: "https://help.myfundedfutures.com/en/articles/17573707-comprehensive-look-into-redesigned-pro-plan", accessed: "2026-10-23" }
  - { field: max_position, url: "https://myfundedfutures.com/plans/pro", accessed: "2026-10-23" }
  - { field: min_trading_days, url: "https://help.myfundedfutures.com/en/articles/17573707-comprehensive-look-into-redesigned-pro-plan", accessed: "2026-10-23" }
  - { field: copy_trading_allowed, url: "https://help.myfundedfutures.com/en/articles/10771500-copy-trading-at-myfundedfutures", accessed: "2026-10-23" }
  - { field: copy_trading_allowed, url: "https://myfundedfutures.com/terms", accessed: "2026-10-23" }
  - { field: automated_trading_allowed, url: "https://help.myfundedfutures.com/en/articles/8444599-fair-play-and-prohibited-trading-practices", accessed: "2026-10-23" }
  - { field: trading_day_reset, url: "https://help.myfundedfutures.com/en/articles/9558251-permitted-times-to-trade", accessed: "2026-10-23" }
  - { field: platforms, url: "https://help.myfundedfutures.com/en/articles/8528335-overview-of-supported-platforms-at-mffu", accessed: "2026-10-23" }
```

#### Pro — funded — 50K

```yaml
firm: myfundedfutures
plan: "Pro"
phase: funded
size_label: "50K"
size_usd: "50000.00"
instruments: [MNQ, NQ]
platforms: [Tradovate, NinjaTrader, TradingView, Quantower, ATAS, DeepChart/DeepDom, Volumetrica, Hyperprop]
drawdown: { amount_usd: "2000.00", type: trailing_eod, stops_trailing_at: "After first payout, MLL moves to $50,100 and remains static (help center); plan-page FAQ: 'After your first approved payout, the MLL locks permanently at your starting balance plus $100'" }
daily_loss_limit: { amount_usd: null, kind: none }
profit_target_usd: null
consistency_rule: { text: "None (but discretionary 'Consistent Trading Policy' monitors behaviour, e.g. maxing contracts on every trade, contract flipping, HFT)", threshold_pct: null }
max_position: { minis: 3, micros: 3, scaling: "Published as '3 mini / 3 micro' on both plan page and help center (mini:micro ratio differs from evaluation 1:10; confirm with firm)" }
min_trading_days: null
trading_day_reset: { time: null, tz_as_published: null }   # no explicit reset published; session "6:00 PM EST" open, positions auto-closed "4:10 PM EST" (see §4)
copy_trading_allowed: conditional   # copiers allowed on all account types; copying other traders and multi-account hedge/mirror/copy for unfair advantage prohibited (see §5)
automated_trading_allowed: conditional   # allowed unless HFT or aimed at exploiting sim fills; live must follow CME guidelines (see §6)
automation_rule_text: "High-frequency Trading is not allowed on our plans. Traders may make use of automated trading strategies tailored to their own specific settings so long as these automated tools do not aim to exploit the favorable fills offered in the Simulated Environment. Traders trading live accounts with automated trading must abide by CME guidelines."
vps_allowed: unknown   # VPS/server/VPN not mentioned on any official page (see §7a)
personal_device_required: unknown   # not required in words; shared devices with ANOTHER trader are forbidden (see §7a)
notes: "Buffer 2100.00; payouts 14 calendar days from first trade + buffer cleared; min $1,000; max $100,000 (help center 'per user'; plan page 'per payout cycle' - conflict, payout only). T1 news NOT allowed in Sim Funded."
status: partial
not_confirmed_fields: [max_position.micros]
sources:
  - { field: drawdown, url: "https://help.myfundedfutures.com/en/articles/17573707-comprehensive-look-into-redesigned-pro-plan", accessed: "2026-10-23" }
  - { field: drawdown, url: "https://myfundedfutures.com/plans/pro", accessed: "2026-10-23" }
  - { field: daily_loss_limit, url: "https://help.myfundedfutures.com/en/articles/17573707-comprehensive-look-into-redesigned-pro-plan", accessed: "2026-10-23" }
  - { field: consistency_rule, url: "https://myfundedfutures.com/plans/pro", accessed: "2026-10-23" }
  - { field: max_position, url: "https://help.myfundedfutures.com/en/articles/17573707-comprehensive-look-into-redesigned-pro-plan", accessed: "2026-10-23" }
  - { field: max_position, url: "https://myfundedfutures.com/plans/pro", accessed: "2026-10-23" }
  - { field: copy_trading_allowed, url: "https://help.myfundedfutures.com/en/articles/10771500-copy-trading-at-myfundedfutures", accessed: "2026-10-23" }
  - { field: copy_trading_allowed, url: "https://myfundedfutures.com/terms", accessed: "2026-10-23" }
  - { field: automated_trading_allowed, url: "https://help.myfundedfutures.com/en/articles/8444599-fair-play-and-prohibited-trading-practices", accessed: "2026-10-23" }
  - { field: trading_day_reset, url: "https://help.myfundedfutures.com/en/articles/9558251-permitted-times-to-trade", accessed: "2026-10-23" }
  - { field: platforms, url: "https://help.myfundedfutures.com/en/articles/8528335-overview-of-supported-platforms-at-mffu", accessed: "2026-10-23" }
```

#### Pro — live — 50K

```yaml
firm: myfundedfutures
plan: "Pro"
phase: live
size_label: "50K"
size_usd: "50000.00"
instruments: [MNQ, NQ]
platforms: []   # live platform not published
drawdown: { amount_usd: "2000.00", type: other, stops_trailing_at: "conflict: help center 'Max Loss Limit (EOD Trailing)' vs plan-page FAQ 'a static drawdown floor'" }
daily_loss_limit: { amount_usd: null, kind: null }
profit_target_usd: null
consistency_rule: { text: null, threshold_pct: null }
max_position: { minis: null, micros: null, scaling: "Help center 'Max Contracts 2' (unit not stated); plan page 'reduced contract limits'" }
min_trading_days: null
trading_day_reset: { time: null, tz_as_published: null }   # no explicit reset published; session "6:00 PM EST" open, positions auto-closed "4:10 PM EST" (see §4)
copy_trading_allowed: conditional   # copiers allowed on all account types; copying other traders and multi-account hedge/mirror/copy for unfair advantage prohibited (see §5)
automated_trading_allowed: conditional   # allowed unless HFT or aimed at exploiting sim fills; live must follow CME guidelines (see §6)
automation_rule_text: "High-frequency Trading is not allowed on our plans. Traders may make use of automated trading strategies tailored to their own specific settings so long as these automated tools do not aim to exploit the favorable fills offered in the Simulated Environment. Traders trading live accounts with automated trading must abide by CME guidelines."
vps_allowed: unknown   # VPS/server/VPN not mentioned on any official page (see §7a)
personal_device_required: unknown   # not required in words; shared devices with ANOTHER trader are forbidden (see §7a)
notes: "Live DLL conflict: help center 'None' vs plan-page FAQ 'live account parameters differ from sim funded, including a daily loss limit'. Live starts at $0. Live trading is user-only."
status: conflict
not_confirmed_fields: [drawdown.type, daily_loss_limit, max_position]
sources:
  - { field: drawdown, url: "https://help.myfundedfutures.com/en/articles/17573707-comprehensive-look-into-redesigned-pro-plan", accessed: "2026-10-23" }
  - { field: drawdown, url: "https://myfundedfutures.com/plans/pro", accessed: "2026-10-23" }
  - { field: daily_loss_limit, url: "https://help.myfundedfutures.com/en/articles/17573707-comprehensive-look-into-redesigned-pro-plan", accessed: "2026-10-23" }
  - { field: daily_loss_limit, url: "https://myfundedfutures.com/plans/pro", accessed: "2026-10-23" }
  - { field: max_position, url: "https://help.myfundedfutures.com/en/articles/17573707-comprehensive-look-into-redesigned-pro-plan", accessed: "2026-10-23" }
  - { field: copy_trading_allowed, url: "https://help.myfundedfutures.com/en/articles/10771500-copy-trading-at-myfundedfutures", accessed: "2026-10-23" }
  - { field: copy_trading_allowed, url: "https://myfundedfutures.com/terms", accessed: "2026-10-23" }
  - { field: automated_trading_allowed, url: "https://help.myfundedfutures.com/en/articles/8444599-fair-play-and-prohibited-trading-practices", accessed: "2026-10-23" }
  - { field: trading_day_reset, url: "https://help.myfundedfutures.com/en/articles/9558251-permitted-times-to-trade", accessed: "2026-10-23" }
  - { field: platforms, url: "https://help.myfundedfutures.com/en/articles/8528335-overview-of-supported-platforms-at-mffu", accessed: "2026-10-23" }
```

#### Pro — evaluation — 100K

```yaml
firm: myfundedfutures
plan: "Pro"
phase: evaluation
size_label: "100K"
size_usd: "100000.00"
instruments: [MNQ, NQ]
platforms: [Tradovate, NinjaTrader, TradingView, Quantower, ATAS, DeepChart/DeepDom, Volumetrica, Hyperprop]
drawdown: { amount_usd: "3000.00", type: trailing_eod, stops_trailing_at: "EOD article: 'the Max EOD trailing locks in at $100 plus the initial starting balance'" }
daily_loss_limit: { amount_usd: null, kind: none }
profit_target_usd: "6000.00"
consistency_rule: { text: "No single day's profit may exceed 50% of total evaluation profits", threshold_pct: "50" }
max_position: { minis: 6, micros: 60, scaling: "No build-up; full allocation from day one (plan page)" }
min_trading_days: 2
trading_day_reset: { time: null, tz_as_published: null }   # no explicit reset published; session "6:00 PM EST" open, positions auto-closed "4:10 PM EST" (see §4)
copy_trading_allowed: conditional   # copiers allowed on all account types; copying other traders and multi-account hedge/mirror/copy for unfair advantage prohibited (see §5)
automated_trading_allowed: conditional   # allowed unless HFT or aimed at exploiting sim fills; live must follow CME guidelines (see §6)
automation_rule_text: "High-frequency Trading is not allowed on our plans. Traders may make use of automated trading strategies tailored to their own specific settings so long as these automated tools do not aim to exploit the favorable fills offered in the Simulated Environment. Traders trading live accounts with automated trading must abide by CME guidelines."
vps_allowed: unknown   # VPS/server/VPN not mentioned on any official page (see §7a)
personal_device_required: unknown   # not required in words; shared devices with ANOTHER trader are forbidden (see §7a)
notes: "T1 news allowed in evaluation."
status: verified
not_confirmed_fields: []
sources:
  - { field: drawdown, url: "https://help.myfundedfutures.com/en/articles/17573707-comprehensive-look-into-redesigned-pro-plan", accessed: "2026-10-23" }
  - { field: drawdown, url: "https://myfundedfutures.com/plans/pro", accessed: "2026-10-23" }
  - { field: drawdown, url: "https://help.myfundedfutures.com/en/articles/8348565-end-of-day-eod-drawdown-explained", accessed: "2026-10-23" }
  - { field: daily_loss_limit, url: "https://help.myfundedfutures.com/en/articles/17573707-comprehensive-look-into-redesigned-pro-plan", accessed: "2026-10-23" }
  - { field: profit_target_usd, url: "https://help.myfundedfutures.com/en/articles/17573707-comprehensive-look-into-redesigned-pro-plan", accessed: "2026-10-23" }
  - { field: consistency_rule, url: "https://help.myfundedfutures.com/en/articles/17573707-comprehensive-look-into-redesigned-pro-plan", accessed: "2026-10-23" }
  - { field: max_position, url: "https://help.myfundedfutures.com/en/articles/17573707-comprehensive-look-into-redesigned-pro-plan", accessed: "2026-10-23" }
  - { field: max_position, url: "https://myfundedfutures.com/plans/pro", accessed: "2026-10-23" }
  - { field: min_trading_days, url: "https://help.myfundedfutures.com/en/articles/17573707-comprehensive-look-into-redesigned-pro-plan", accessed: "2026-10-23" }
  - { field: copy_trading_allowed, url: "https://help.myfundedfutures.com/en/articles/10771500-copy-trading-at-myfundedfutures", accessed: "2026-10-23" }
  - { field: copy_trading_allowed, url: "https://myfundedfutures.com/terms", accessed: "2026-10-23" }
  - { field: automated_trading_allowed, url: "https://help.myfundedfutures.com/en/articles/8444599-fair-play-and-prohibited-trading-practices", accessed: "2026-10-23" }
  - { field: trading_day_reset, url: "https://help.myfundedfutures.com/en/articles/9558251-permitted-times-to-trade", accessed: "2026-10-23" }
  - { field: platforms, url: "https://help.myfundedfutures.com/en/articles/8528335-overview-of-supported-platforms-at-mffu", accessed: "2026-10-23" }
```

#### Pro — funded — 100K

```yaml
firm: myfundedfutures
plan: "Pro"
phase: funded
size_label: "100K"
size_usd: "100000.00"
instruments: [MNQ, NQ]
platforms: [Tradovate, NinjaTrader, TradingView, Quantower, ATAS, DeepChart/DeepDom, Volumetrica, Hyperprop]
drawdown: { amount_usd: "3000.00", type: trailing_eod, stops_trailing_at: "After first payout, MLL moves to $100,100 and remains static (help center); plan-page FAQ: 'After your first approved payout, the MLL locks permanently at your starting balance plus $100'" }
daily_loss_limit: { amount_usd: null, kind: none }
profit_target_usd: null
consistency_rule: { text: "None (but discretionary 'Consistent Trading Policy' monitors behaviour, e.g. maxing contracts on every trade, contract flipping, HFT)", threshold_pct: null }
max_position: { minis: 6, micros: 6, scaling: "Published as '6 mini / 6 micro' on both plan page and help center (mini:micro ratio differs from evaluation 1:10; confirm with firm)" }
min_trading_days: null
trading_day_reset: { time: null, tz_as_published: null }   # no explicit reset published; session "6:00 PM EST" open, positions auto-closed "4:10 PM EST" (see §4)
copy_trading_allowed: conditional   # copiers allowed on all account types; copying other traders and multi-account hedge/mirror/copy for unfair advantage prohibited (see §5)
automated_trading_allowed: conditional   # allowed unless HFT or aimed at exploiting sim fills; live must follow CME guidelines (see §6)
automation_rule_text: "High-frequency Trading is not allowed on our plans. Traders may make use of automated trading strategies tailored to their own specific settings so long as these automated tools do not aim to exploit the favorable fills offered in the Simulated Environment. Traders trading live accounts with automated trading must abide by CME guidelines."
vps_allowed: unknown   # VPS/server/VPN not mentioned on any official page (see §7a)
personal_device_required: unknown   # not required in words; shared devices with ANOTHER trader are forbidden (see §7a)
notes: "Buffer 3100.00; payouts 14 calendar days from first trade + buffer cleared; min $1,000; max $100,000 (help center 'per user'; plan page 'per payout cycle' - conflict, payout only). T1 news NOT allowed in Sim Funded."
status: partial
not_confirmed_fields: [max_position.micros]
sources:
  - { field: drawdown, url: "https://help.myfundedfutures.com/en/articles/17573707-comprehensive-look-into-redesigned-pro-plan", accessed: "2026-10-23" }
  - { field: drawdown, url: "https://myfundedfutures.com/plans/pro", accessed: "2026-10-23" }
  - { field: daily_loss_limit, url: "https://help.myfundedfutures.com/en/articles/17573707-comprehensive-look-into-redesigned-pro-plan", accessed: "2026-10-23" }
  - { field: consistency_rule, url: "https://myfundedfutures.com/plans/pro", accessed: "2026-10-23" }
  - { field: max_position, url: "https://help.myfundedfutures.com/en/articles/17573707-comprehensive-look-into-redesigned-pro-plan", accessed: "2026-10-23" }
  - { field: max_position, url: "https://myfundedfutures.com/plans/pro", accessed: "2026-10-23" }
  - { field: copy_trading_allowed, url: "https://help.myfundedfutures.com/en/articles/10771500-copy-trading-at-myfundedfutures", accessed: "2026-10-23" }
  - { field: copy_trading_allowed, url: "https://myfundedfutures.com/terms", accessed: "2026-10-23" }
  - { field: automated_trading_allowed, url: "https://help.myfundedfutures.com/en/articles/8444599-fair-play-and-prohibited-trading-practices", accessed: "2026-10-23" }
  - { field: trading_day_reset, url: "https://help.myfundedfutures.com/en/articles/9558251-permitted-times-to-trade", accessed: "2026-10-23" }
  - { field: platforms, url: "https://help.myfundedfutures.com/en/articles/8528335-overview-of-supported-platforms-at-mffu", accessed: "2026-10-23" }
```

#### Pro — live — 100K

```yaml
firm: myfundedfutures
plan: "Pro"
phase: live
size_label: "100K"
size_usd: "100000.00"
instruments: [MNQ, NQ]
platforms: []   # live platform not published
drawdown: { amount_usd: "3000.00", type: other, stops_trailing_at: "conflict: help center 'Max Loss Limit (EOD Trailing)' vs plan-page FAQ 'a static drawdown floor'" }
daily_loss_limit: { amount_usd: null, kind: null }
profit_target_usd: null
consistency_rule: { text: null, threshold_pct: null }
max_position: { minis: null, micros: null, scaling: "Help center 'Max Contracts 3' (unit not stated); plan page 'reduced contract limits'" }
min_trading_days: null
trading_day_reset: { time: null, tz_as_published: null }   # no explicit reset published; session "6:00 PM EST" open, positions auto-closed "4:10 PM EST" (see §4)
copy_trading_allowed: conditional   # copiers allowed on all account types; copying other traders and multi-account hedge/mirror/copy for unfair advantage prohibited (see §5)
automated_trading_allowed: conditional   # allowed unless HFT or aimed at exploiting sim fills; live must follow CME guidelines (see §6)
automation_rule_text: "High-frequency Trading is not allowed on our plans. Traders may make use of automated trading strategies tailored to their own specific settings so long as these automated tools do not aim to exploit the favorable fills offered in the Simulated Environment. Traders trading live accounts with automated trading must abide by CME guidelines."
vps_allowed: unknown   # VPS/server/VPN not mentioned on any official page (see §7a)
personal_device_required: unknown   # not required in words; shared devices with ANOTHER trader are forbidden (see §7a)
notes: "Live DLL conflict: help center 'None' vs plan-page FAQ 'live account parameters differ from sim funded, including a daily loss limit'. Live starts at $0. Live trading is user-only."
status: conflict
not_confirmed_fields: [drawdown.type, daily_loss_limit, max_position]
sources:
  - { field: drawdown, url: "https://help.myfundedfutures.com/en/articles/17573707-comprehensive-look-into-redesigned-pro-plan", accessed: "2026-10-23" }
  - { field: drawdown, url: "https://myfundedfutures.com/plans/pro", accessed: "2026-10-23" }
  - { field: daily_loss_limit, url: "https://help.myfundedfutures.com/en/articles/17573707-comprehensive-look-into-redesigned-pro-plan", accessed: "2026-10-23" }
  - { field: daily_loss_limit, url: "https://myfundedfutures.com/plans/pro", accessed: "2026-10-23" }
  - { field: max_position, url: "https://help.myfundedfutures.com/en/articles/17573707-comprehensive-look-into-redesigned-pro-plan", accessed: "2026-10-23" }
  - { field: copy_trading_allowed, url: "https://help.myfundedfutures.com/en/articles/10771500-copy-trading-at-myfundedfutures", accessed: "2026-10-23" }
  - { field: copy_trading_allowed, url: "https://myfundedfutures.com/terms", accessed: "2026-10-23" }
  - { field: automated_trading_allowed, url: "https://help.myfundedfutures.com/en/articles/8444599-fair-play-and-prohibited-trading-practices", accessed: "2026-10-23" }
  - { field: trading_day_reset, url: "https://help.myfundedfutures.com/en/articles/9558251-permitted-times-to-trade", accessed: "2026-10-23" }
  - { field: platforms, url: "https://help.myfundedfutures.com/en/articles/8528335-overview-of-supported-platforms-at-mffu", accessed: "2026-10-23" }
```

#### Pro — evaluation — 150K

```yaml
firm: myfundedfutures
plan: "Pro"
phase: evaluation
size_label: "150K"
size_usd: "150000.00"
instruments: [MNQ, NQ]
platforms: [Tradovate, NinjaTrader, TradingView, Quantower, ATAS, DeepChart/DeepDom, Volumetrica, Hyperprop]
drawdown: { amount_usd: "4500.00", type: trailing_eod, stops_trailing_at: "EOD article: 'the Max EOD trailing locks in at $100 plus the initial starting balance'" }
daily_loss_limit: { amount_usd: null, kind: none }
profit_target_usd: "9000.00"
consistency_rule: { text: "No single day's profit may exceed 50% of total evaluation profits", threshold_pct: "50" }
max_position: { minis: 9, micros: 90, scaling: "No build-up; full allocation from day one (plan page)" }
min_trading_days: 2
trading_day_reset: { time: null, tz_as_published: null }   # no explicit reset published; session "6:00 PM EST" open, positions auto-closed "4:10 PM EST" (see §4)
copy_trading_allowed: conditional   # copiers allowed on all account types; copying other traders and multi-account hedge/mirror/copy for unfair advantage prohibited (see §5)
automated_trading_allowed: conditional   # allowed unless HFT or aimed at exploiting sim fills; live must follow CME guidelines (see §6)
automation_rule_text: "High-frequency Trading is not allowed on our plans. Traders may make use of automated trading strategies tailored to their own specific settings so long as these automated tools do not aim to exploit the favorable fills offered in the Simulated Environment. Traders trading live accounts with automated trading must abide by CME guidelines."
vps_allowed: unknown   # VPS/server/VPN not mentioned on any official page (see §7a)
personal_device_required: unknown   # not required in words; shared devices with ANOTHER trader are forbidden (see §7a)
notes: "T1 news allowed in evaluation."
status: verified
not_confirmed_fields: []
sources:
  - { field: drawdown, url: "https://help.myfundedfutures.com/en/articles/17573707-comprehensive-look-into-redesigned-pro-plan", accessed: "2026-10-23" }
  - { field: drawdown, url: "https://myfundedfutures.com/plans/pro", accessed: "2026-10-23" }
  - { field: drawdown, url: "https://help.myfundedfutures.com/en/articles/8348565-end-of-day-eod-drawdown-explained", accessed: "2026-10-23" }
  - { field: daily_loss_limit, url: "https://help.myfundedfutures.com/en/articles/17573707-comprehensive-look-into-redesigned-pro-plan", accessed: "2026-10-23" }
  - { field: profit_target_usd, url: "https://help.myfundedfutures.com/en/articles/17573707-comprehensive-look-into-redesigned-pro-plan", accessed: "2026-10-23" }
  - { field: consistency_rule, url: "https://help.myfundedfutures.com/en/articles/17573707-comprehensive-look-into-redesigned-pro-plan", accessed: "2026-10-23" }
  - { field: max_position, url: "https://help.myfundedfutures.com/en/articles/17573707-comprehensive-look-into-redesigned-pro-plan", accessed: "2026-10-23" }
  - { field: max_position, url: "https://myfundedfutures.com/plans/pro", accessed: "2026-10-23" }
  - { field: min_trading_days, url: "https://help.myfundedfutures.com/en/articles/17573707-comprehensive-look-into-redesigned-pro-plan", accessed: "2026-10-23" }
  - { field: copy_trading_allowed, url: "https://help.myfundedfutures.com/en/articles/10771500-copy-trading-at-myfundedfutures", accessed: "2026-10-23" }
  - { field: copy_trading_allowed, url: "https://myfundedfutures.com/terms", accessed: "2026-10-23" }
  - { field: automated_trading_allowed, url: "https://help.myfundedfutures.com/en/articles/8444599-fair-play-and-prohibited-trading-practices", accessed: "2026-10-23" }
  - { field: trading_day_reset, url: "https://help.myfundedfutures.com/en/articles/9558251-permitted-times-to-trade", accessed: "2026-10-23" }
  - { field: platforms, url: "https://help.myfundedfutures.com/en/articles/8528335-overview-of-supported-platforms-at-mffu", accessed: "2026-10-23" }
```

#### Pro — funded — 150K

```yaml
firm: myfundedfutures
plan: "Pro"
phase: funded
size_label: "150K"
size_usd: "150000.00"
instruments: [MNQ, NQ]
platforms: [Tradovate, NinjaTrader, TradingView, Quantower, ATAS, DeepChart/DeepDom, Volumetrica, Hyperprop]
drawdown: { amount_usd: "4500.00", type: trailing_eod, stops_trailing_at: "After first payout, MLL moves to $150,100 and remains static (help center); plan-page FAQ: 'After your first approved payout, the MLL locks permanently at your starting balance plus $100'" }
daily_loss_limit: { amount_usd: null, kind: none }
profit_target_usd: null
consistency_rule: { text: "None (but discretionary 'Consistent Trading Policy' monitors behaviour, e.g. maxing contracts on every trade, contract flipping, HFT)", threshold_pct: null }
max_position: { minis: 9, micros: 9, scaling: "Published as '9 mini / 9 micro' on both plan page and help center (mini:micro ratio differs from evaluation 1:10; confirm with firm)" }
min_trading_days: null
trading_day_reset: { time: null, tz_as_published: null }   # no explicit reset published; session "6:00 PM EST" open, positions auto-closed "4:10 PM EST" (see §4)
copy_trading_allowed: conditional   # copiers allowed on all account types; copying other traders and multi-account hedge/mirror/copy for unfair advantage prohibited (see §5)
automated_trading_allowed: conditional   # allowed unless HFT or aimed at exploiting sim fills; live must follow CME guidelines (see §6)
automation_rule_text: "High-frequency Trading is not allowed on our plans. Traders may make use of automated trading strategies tailored to their own specific settings so long as these automated tools do not aim to exploit the favorable fills offered in the Simulated Environment. Traders trading live accounts with automated trading must abide by CME guidelines."
vps_allowed: unknown   # VPS/server/VPN not mentioned on any official page (see §7a)
personal_device_required: unknown   # not required in words; shared devices with ANOTHER trader are forbidden (see §7a)
notes: "Buffer 4600.00; payouts 14 calendar days from first trade + buffer cleared; min $1,000; max $100,000 (help center 'per user'; plan page 'per payout cycle' - conflict, payout only). T1 news NOT allowed in Sim Funded."
status: partial
not_confirmed_fields: [max_position.micros]
sources:
  - { field: drawdown, url: "https://help.myfundedfutures.com/en/articles/17573707-comprehensive-look-into-redesigned-pro-plan", accessed: "2026-10-23" }
  - { field: drawdown, url: "https://myfundedfutures.com/plans/pro", accessed: "2026-10-23" }
  - { field: daily_loss_limit, url: "https://help.myfundedfutures.com/en/articles/17573707-comprehensive-look-into-redesigned-pro-plan", accessed: "2026-10-23" }
  - { field: consistency_rule, url: "https://myfundedfutures.com/plans/pro", accessed: "2026-10-23" }
  - { field: max_position, url: "https://help.myfundedfutures.com/en/articles/17573707-comprehensive-look-into-redesigned-pro-plan", accessed: "2026-10-23" }
  - { field: max_position, url: "https://myfundedfutures.com/plans/pro", accessed: "2026-10-23" }
  - { field: copy_trading_allowed, url: "https://help.myfundedfutures.com/en/articles/10771500-copy-trading-at-myfundedfutures", accessed: "2026-10-23" }
  - { field: copy_trading_allowed, url: "https://myfundedfutures.com/terms", accessed: "2026-10-23" }
  - { field: automated_trading_allowed, url: "https://help.myfundedfutures.com/en/articles/8444599-fair-play-and-prohibited-trading-practices", accessed: "2026-10-23" }
  - { field: trading_day_reset, url: "https://help.myfundedfutures.com/en/articles/9558251-permitted-times-to-trade", accessed: "2026-10-23" }
  - { field: platforms, url: "https://help.myfundedfutures.com/en/articles/8528335-overview-of-supported-platforms-at-mffu", accessed: "2026-10-23" }
```

#### Pro — live — 150K

```yaml
firm: myfundedfutures
plan: "Pro"
phase: live
size_label: "150K"
size_usd: "150000.00"
instruments: [MNQ, NQ]
platforms: []   # live platform not published
drawdown: { amount_usd: "4500.00", type: other, stops_trailing_at: "conflict: help center 'Max Loss Limit (EOD Trailing)' vs plan-page FAQ 'a static drawdown floor'" }
daily_loss_limit: { amount_usd: null, kind: null }
profit_target_usd: null
consistency_rule: { text: null, threshold_pct: null }
max_position: { minis: null, micros: null, scaling: "Help center 'Max Contracts 4' (unit not stated); plan page 'reduced contract limits'" }
min_trading_days: null
trading_day_reset: { time: null, tz_as_published: null }   # no explicit reset published; session "6:00 PM EST" open, positions auto-closed "4:10 PM EST" (see §4)
copy_trading_allowed: conditional   # copiers allowed on all account types; copying other traders and multi-account hedge/mirror/copy for unfair advantage prohibited (see §5)
automated_trading_allowed: conditional   # allowed unless HFT or aimed at exploiting sim fills; live must follow CME guidelines (see §6)
automation_rule_text: "High-frequency Trading is not allowed on our plans. Traders may make use of automated trading strategies tailored to their own specific settings so long as these automated tools do not aim to exploit the favorable fills offered in the Simulated Environment. Traders trading live accounts with automated trading must abide by CME guidelines."
vps_allowed: unknown   # VPS/server/VPN not mentioned on any official page (see §7a)
personal_device_required: unknown   # not required in words; shared devices with ANOTHER trader are forbidden (see §7a)
notes: "Live DLL conflict: help center 'None' vs plan-page FAQ 'live account parameters differ from sim funded, including a daily loss limit'. Live starts at $0. Live trading is user-only."
status: conflict
not_confirmed_fields: [drawdown.type, daily_loss_limit, max_position]
sources:
  - { field: drawdown, url: "https://help.myfundedfutures.com/en/articles/17573707-comprehensive-look-into-redesigned-pro-plan", accessed: "2026-10-23" }
  - { field: drawdown, url: "https://myfundedfutures.com/plans/pro", accessed: "2026-10-23" }
  - { field: daily_loss_limit, url: "https://help.myfundedfutures.com/en/articles/17573707-comprehensive-look-into-redesigned-pro-plan", accessed: "2026-10-23" }
  - { field: daily_loss_limit, url: "https://myfundedfutures.com/plans/pro", accessed: "2026-10-23" }
  - { field: max_position, url: "https://help.myfundedfutures.com/en/articles/17573707-comprehensive-look-into-redesigned-pro-plan", accessed: "2026-10-23" }
  - { field: copy_trading_allowed, url: "https://help.myfundedfutures.com/en/articles/10771500-copy-trading-at-myfundedfutures", accessed: "2026-10-23" }
  - { field: copy_trading_allowed, url: "https://myfundedfutures.com/terms", accessed: "2026-10-23" }
  - { field: automated_trading_allowed, url: "https://help.myfundedfutures.com/en/articles/8444599-fair-play-and-prohibited-trading-practices", accessed: "2026-10-23" }
  - { field: trading_day_reset, url: "https://help.myfundedfutures.com/en/articles/9558251-permitted-times-to-trade", accessed: "2026-10-23" }
  - { field: platforms, url: "https://help.myfundedfutures.com/en/articles/8528335-overview-of-supported-platforms-at-mffu", accessed: "2026-10-23" }
```

#### Pro (50K One Day add-on) — evaluation — 50K

```yaml
firm: myfundedfutures
plan: "Pro (50K One Day add-on)"
phase: evaluation
size_label: "50K"
size_usd: "50000.00"
instruments: [MNQ, NQ]
platforms: [Tradovate, NinjaTrader, TradingView, Quantower, ATAS, DeepChart/DeepDom, Volumetrica, Hyperprop]
drawdown: { amount_usd: "2000.00", type: trailing_eod, stops_trailing_at: "EOD article: 'locks in at $100 plus the initial starting balance'" }
daily_loss_limit: { amount_usd: null, kind: none }
profit_target_usd: "4000.00"
consistency_rule: { text: "None", threshold_pct: null }
max_position: { minis: 3, micros: 30, scaling: "No build-up" }
min_trading_days: 1
trading_day_reset: { time: null, tz_as_published: null }   # no explicit reset published; session "6:00 PM EST" open, positions auto-closed "4:10 PM EST" (see §4)
copy_trading_allowed: conditional   # copiers allowed on all account types; copying other traders and multi-account hedge/mirror/copy for unfair advantage prohibited (see §5)
automated_trading_allowed: conditional   # allowed unless HFT or aimed at exploiting sim fills; live must follow CME guidelines (see §6)
automation_rule_text: "High-frequency Trading is not allowed on our plans. Traders may make use of automated trading strategies tailored to their own specific settings so long as these automated tools do not aim to exploit the favorable fills offered in the Simulated Environment. Traders trading live accounts with automated trading must abide by CME guidelines."
vps_allowed: unknown   # VPS/server/VPN not mentioned on any official page (see §7a)
personal_device_required: unknown   # not required in words; shared devices with ANOTHER trader are forbidden (see §7a)
notes: "'Pro 50k One Day account follow the same rules as the Pro 50k' after evaluation. The add-on article sits in the help center's 'Legacy Plans' collection but the current Pro and payout articles still list it - availability to confirm."
status: verified
not_confirmed_fields: []
sources:
  - { field: profit_target_usd, url: "https://help.myfundedfutures.com/en/articles/17573707-comprehensive-look-into-redesigned-pro-plan", accessed: "2026-10-23" }
  - { field: profit_target_usd, url: "https://help.myfundedfutures.com/en/articles/11802636-traders-evaluation-simplified", accessed: "2026-10-23" }
  - { field: consistency_rule, url: "https://help.myfundedfutures.com/en/articles/11994562-consistency-rule-at-my-fundedfutures", accessed: "2026-10-23" }
  - { field: max_position, url: "https://help.myfundedfutures.com/en/articles/17573707-comprehensive-look-into-redesigned-pro-plan", accessed: "2026-10-23" }
  - { field: copy_trading_allowed, url: "https://help.myfundedfutures.com/en/articles/10771500-copy-trading-at-myfundedfutures", accessed: "2026-10-23" }
  - { field: copy_trading_allowed, url: "https://myfundedfutures.com/terms", accessed: "2026-10-23" }
  - { field: automated_trading_allowed, url: "https://help.myfundedfutures.com/en/articles/8444599-fair-play-and-prohibited-trading-practices", accessed: "2026-10-23" }
  - { field: trading_day_reset, url: "https://help.myfundedfutures.com/en/articles/9558251-permitted-times-to-trade", accessed: "2026-10-23" }
  - { field: platforms, url: "https://help.myfundedfutures.com/en/articles/8528335-overview-of-supported-platforms-at-mffu", accessed: "2026-10-23" }
```

#### Studio #001 — evaluation — 10K

```yaml
firm: myfundedfutures
plan: "Studio #001"
phase: evaluation
size_label: "10K"
size_usd: "10000.00"
instruments: [MNQ]
platforms: [Hyperprop]
drawdown: { amount_usd: "500.00", type: static, stops_trailing_at: "Static - set at start, never moves" }
daily_loss_limit: { amount_usd: null, kind: none }
profit_target_usd: "1000.00"
consistency_rule: { text: "None", threshold_pct: null }
max_position: { minis: 0, micros: 10, scaling: "Micros only, no minis" }
min_trading_days: null
trading_day_reset: { time: null, tz_as_published: null }   # no explicit reset published; session "6:00 PM EST" open, positions auto-closed "4:10 PM EST" (see §4)
copy_trading_allowed: conditional   # copiers allowed on all account types; copying other traders and multi-account hedge/mirror/copy for unfair advantage prohibited (see §5)
automated_trading_allowed: conditional   # allowed unless HFT or aimed at exploiting sim fills; live must follow CME guidelines (see §6)
automation_rule_text: "High-frequency Trading is not allowed on our plans. Traders may make use of automated trading strategies tailored to their own specific settings so long as these automated tools do not aim to exploit the favorable fills offered in the Simulated Environment. Traders trading live accounts with automated trading must abide by CME guidelines."
vps_allowed: unknown   # VPS/server/VPN not mentioned on any official page (see §7a)
personal_device_required: unknown   # not required in words; shared devices with ANOTHER trader are forbidden (see §7a)
notes: "Below 25K priority range. Min trading days: none. 1 active Studio account (eval or Sim Funded)."
status: verified
not_confirmed_fields: []
sources:
  - { field: drawdown, url: "https://help.myfundedfutures.com/en/articles/17589665-studio-plans-a-comprehensive-guide", accessed: "2026-10-23" }
  - { field: drawdown, url: "https://myfundedfutures.com/studio", accessed: "2026-10-23" }
  - { field: max_position, url: "https://help.myfundedfutures.com/en/articles/17589665-studio-plans-a-comprehensive-guide", accessed: "2026-10-23" }
  - { field: copy_trading_allowed, url: "https://help.myfundedfutures.com/en/articles/10771500-copy-trading-at-myfundedfutures", accessed: "2026-10-23" }
  - { field: copy_trading_allowed, url: "https://myfundedfutures.com/terms", accessed: "2026-10-23" }
  - { field: automated_trading_allowed, url: "https://help.myfundedfutures.com/en/articles/8444599-fair-play-and-prohibited-trading-practices", accessed: "2026-10-23" }
  - { field: trading_day_reset, url: "https://help.myfundedfutures.com/en/articles/9558251-permitted-times-to-trade", accessed: "2026-10-23" }
  - { field: platforms, url: "https://help.myfundedfutures.com/en/articles/8528335-overview-of-supported-platforms-at-mffu", accessed: "2026-10-23" }
```

#### Studio #001 — funded — 10K

```yaml
firm: myfundedfutures
plan: "Studio #001"
phase: funded
size_label: "10K"
size_usd: "10000.00"
instruments: [MNQ]
platforms: [Hyperprop]
drawdown: { amount_usd: "500.00", type: static, stops_trailing_at: "Static; Sim Funded starts at $0" }
daily_loss_limit: { amount_usd: null, kind: none }
profit_target_usd: null
consistency_rule: { text: "None", threshold_pct: null }
max_position: { minis: 0, micros: 10, scaling: "Micros only, no minis" }
min_trading_days: null
trading_day_reset: { time: null, tz_as_published: null }   # no explicit reset published; session "6:00 PM EST" open, positions auto-closed "4:10 PM EST" (see §4)
copy_trading_allowed: conditional   # copiers allowed on all account types; copying other traders and multi-account hedge/mirror/copy for unfair advantage prohibited (see §5)
automated_trading_allowed: conditional   # allowed unless HFT or aimed at exploiting sim fills; live must follow CME guidelines (see §6)
automation_rule_text: "High-frequency Trading is not allowed on our plans. Traders may make use of automated trading strategies tailored to their own specific settings so long as these automated tools do not aim to exploit the favorable fills offered in the Simulated Environment. Traders trading live accounts with automated trading must abide by CME guidelines."
vps_allowed: unknown   # VPS/server/VPN not mentioned on any official page (see §7a)
personal_device_required: unknown   # not required in words; shared devices with ANOTHER trader are forbidden (see §7a)
notes: "Daily profit cap $5000 (excess forfeited). Payouts anytime."
status: verified
not_confirmed_fields: []
sources:
  - { field: drawdown, url: "https://help.myfundedfutures.com/en/articles/17589665-studio-plans-a-comprehensive-guide", accessed: "2026-10-23" }
  - { field: max_position, url: "https://help.myfundedfutures.com/en/articles/17589665-studio-plans-a-comprehensive-guide", accessed: "2026-10-23" }
  - { field: copy_trading_allowed, url: "https://help.myfundedfutures.com/en/articles/10771500-copy-trading-at-myfundedfutures", accessed: "2026-10-23" }
  - { field: copy_trading_allowed, url: "https://myfundedfutures.com/terms", accessed: "2026-10-23" }
  - { field: automated_trading_allowed, url: "https://help.myfundedfutures.com/en/articles/8444599-fair-play-and-prohibited-trading-practices", accessed: "2026-10-23" }
  - { field: trading_day_reset, url: "https://help.myfundedfutures.com/en/articles/9558251-permitted-times-to-trade", accessed: "2026-10-23" }
  - { field: platforms, url: "https://help.myfundedfutures.com/en/articles/8528335-overview-of-supported-platforms-at-mffu", accessed: "2026-10-23" }
```

#### Studio #001 — live — 10K

```yaml
firm: myfundedfutures
plan: "Studio #001"
phase: live
size_label: "10K"
size_usd: null
instruments: [MNQ]
platforms: []   # live platform not published
drawdown: { amount_usd: null, type: trailing_eod, stops_trailing_at: "Live MLL equal to Live starting balance (40% of total requested, capped at $2,000); trails EOD and locks at $100" }
daily_loss_limit: { amount_usd: null, kind: null }
profit_target_usd: null
consistency_rule: { text: null, threshold_pct: null }
max_position: { minis: 0, micros: 10, scaling: "#001/#002: same as plan (micros, no minis); #003: 1 mini / 10 micro" }
min_trading_days: null
trading_day_reset: { time: null, tz_as_published: null }   # no explicit reset published; session "6:00 PM EST" open, positions auto-closed "4:10 PM EST" (see §4)
copy_trading_allowed: conditional   # copiers allowed on all account types; copying other traders and multi-account hedge/mirror/copy for unfair advantage prohibited (see §5)
automated_trading_allowed: conditional   # allowed unless HFT or aimed at exploiting sim fills; live must follow CME guidelines (see §6)
automation_rule_text: "High-frequency Trading is not allowed on our plans. Traders may make use of automated trading strategies tailored to their own specific settings so long as these automated tools do not aim to exploit the favorable fills offered in the Simulated Environment. Traders trading live accounts with automated trading must abide by CME guidelines."
vps_allowed: unknown   # VPS/server/VPN not mentioned on any official page (see §7a)
personal_device_required: unknown   # not required in words; shared devices with ANOTHER trader are forbidden (see §7a)
notes: "Live DLL not stated. Live trading is user-only."
status: partial
not_confirmed_fields: []
sources:
  - { field: drawdown, url: "https://help.myfundedfutures.com/en/articles/17589665-studio-plans-a-comprehensive-guide", accessed: "2026-10-23" }
  - { field: max_position, url: "https://help.myfundedfutures.com/en/articles/17589665-studio-plans-a-comprehensive-guide", accessed: "2026-10-23" }
  - { field: copy_trading_allowed, url: "https://help.myfundedfutures.com/en/articles/10771500-copy-trading-at-myfundedfutures", accessed: "2026-10-23" }
  - { field: copy_trading_allowed, url: "https://myfundedfutures.com/terms", accessed: "2026-10-23" }
  - { field: automated_trading_allowed, url: "https://help.myfundedfutures.com/en/articles/8444599-fair-play-and-prohibited-trading-practices", accessed: "2026-10-23" }
  - { field: trading_day_reset, url: "https://help.myfundedfutures.com/en/articles/9558251-permitted-times-to-trade", accessed: "2026-10-23" }
  - { field: platforms, url: "https://help.myfundedfutures.com/en/articles/8528335-overview-of-supported-platforms-at-mffu", accessed: "2026-10-23" }
```

#### Studio #002 — evaluation — 20K

```yaml
firm: myfundedfutures
plan: "Studio #002"
phase: evaluation
size_label: "20K"
size_usd: "20000.00"
instruments: [MNQ]
platforms: [Hyperprop]
drawdown: { amount_usd: "1000.00", type: static, stops_trailing_at: "Static - set at start, never moves" }
daily_loss_limit: { amount_usd: null, kind: none }
profit_target_usd: "2000.00"
consistency_rule: { text: "None", threshold_pct: null }
max_position: { minis: 0, micros: 20, scaling: "Micros only, no minis" }
min_trading_days: null
trading_day_reset: { time: null, tz_as_published: null }   # no explicit reset published; session "6:00 PM EST" open, positions auto-closed "4:10 PM EST" (see §4)
copy_trading_allowed: conditional   # copiers allowed on all account types; copying other traders and multi-account hedge/mirror/copy for unfair advantage prohibited (see §5)
automated_trading_allowed: conditional   # allowed unless HFT or aimed at exploiting sim fills; live must follow CME guidelines (see §6)
automation_rule_text: "High-frequency Trading is not allowed on our plans. Traders may make use of automated trading strategies tailored to their own specific settings so long as these automated tools do not aim to exploit the favorable fills offered in the Simulated Environment. Traders trading live accounts with automated trading must abide by CME guidelines."
vps_allowed: unknown   # VPS/server/VPN not mentioned on any official page (see §7a)
personal_device_required: unknown   # not required in words; shared devices with ANOTHER trader are forbidden (see §7a)
notes: "Below 25K priority range. Min trading days: none. 1 active Studio account (eval or Sim Funded)."
status: verified
not_confirmed_fields: []
sources:
  - { field: drawdown, url: "https://help.myfundedfutures.com/en/articles/17589665-studio-plans-a-comprehensive-guide", accessed: "2026-10-23" }
  - { field: drawdown, url: "https://myfundedfutures.com/studio", accessed: "2026-10-23" }
  - { field: max_position, url: "https://help.myfundedfutures.com/en/articles/17589665-studio-plans-a-comprehensive-guide", accessed: "2026-10-23" }
  - { field: copy_trading_allowed, url: "https://help.myfundedfutures.com/en/articles/10771500-copy-trading-at-myfundedfutures", accessed: "2026-10-23" }
  - { field: copy_trading_allowed, url: "https://myfundedfutures.com/terms", accessed: "2026-10-23" }
  - { field: automated_trading_allowed, url: "https://help.myfundedfutures.com/en/articles/8444599-fair-play-and-prohibited-trading-practices", accessed: "2026-10-23" }
  - { field: trading_day_reset, url: "https://help.myfundedfutures.com/en/articles/9558251-permitted-times-to-trade", accessed: "2026-10-23" }
  - { field: platforms, url: "https://help.myfundedfutures.com/en/articles/8528335-overview-of-supported-platforms-at-mffu", accessed: "2026-10-23" }
```

#### Studio #002 — funded — 20K

```yaml
firm: myfundedfutures
plan: "Studio #002"
phase: funded
size_label: "20K"
size_usd: "20000.00"
instruments: [MNQ]
platforms: [Hyperprop]
drawdown: { amount_usd: "1000.00", type: static, stops_trailing_at: "Static; Sim Funded starts at $0" }
daily_loss_limit: { amount_usd: null, kind: none }
profit_target_usd: null
consistency_rule: { text: "None", threshold_pct: null }
max_position: { minis: 0, micros: 20, scaling: "Micros only, no minis" }
min_trading_days: null
trading_day_reset: { time: null, tz_as_published: null }   # no explicit reset published; session "6:00 PM EST" open, positions auto-closed "4:10 PM EST" (see §4)
copy_trading_allowed: conditional   # copiers allowed on all account types; copying other traders and multi-account hedge/mirror/copy for unfair advantage prohibited (see §5)
automated_trading_allowed: conditional   # allowed unless HFT or aimed at exploiting sim fills; live must follow CME guidelines (see §6)
automation_rule_text: "High-frequency Trading is not allowed on our plans. Traders may make use of automated trading strategies tailored to their own specific settings so long as these automated tools do not aim to exploit the favorable fills offered in the Simulated Environment. Traders trading live accounts with automated trading must abide by CME guidelines."
vps_allowed: unknown   # VPS/server/VPN not mentioned on any official page (see §7a)
personal_device_required: unknown   # not required in words; shared devices with ANOTHER trader are forbidden (see §7a)
notes: "Daily profit cap $10000 (excess forfeited). Payouts anytime."
status: verified
not_confirmed_fields: []
sources:
  - { field: drawdown, url: "https://help.myfundedfutures.com/en/articles/17589665-studio-plans-a-comprehensive-guide", accessed: "2026-10-23" }
  - { field: max_position, url: "https://help.myfundedfutures.com/en/articles/17589665-studio-plans-a-comprehensive-guide", accessed: "2026-10-23" }
  - { field: copy_trading_allowed, url: "https://help.myfundedfutures.com/en/articles/10771500-copy-trading-at-myfundedfutures", accessed: "2026-10-23" }
  - { field: copy_trading_allowed, url: "https://myfundedfutures.com/terms", accessed: "2026-10-23" }
  - { field: automated_trading_allowed, url: "https://help.myfundedfutures.com/en/articles/8444599-fair-play-and-prohibited-trading-practices", accessed: "2026-10-23" }
  - { field: trading_day_reset, url: "https://help.myfundedfutures.com/en/articles/9558251-permitted-times-to-trade", accessed: "2026-10-23" }
  - { field: platforms, url: "https://help.myfundedfutures.com/en/articles/8528335-overview-of-supported-platforms-at-mffu", accessed: "2026-10-23" }
```

#### Studio #002 — live — 20K

```yaml
firm: myfundedfutures
plan: "Studio #002"
phase: live
size_label: "20K"
size_usd: null
instruments: [MNQ]
platforms: []   # live platform not published
drawdown: { amount_usd: null, type: trailing_eod, stops_trailing_at: "Live MLL equal to Live starting balance (40% of total requested, capped at $2,000); trails EOD and locks at $100" }
daily_loss_limit: { amount_usd: null, kind: null }
profit_target_usd: null
consistency_rule: { text: null, threshold_pct: null }
max_position: { minis: 0, micros: 20, scaling: "#001/#002: same as plan (micros, no minis); #003: 1 mini / 10 micro" }
min_trading_days: null
trading_day_reset: { time: null, tz_as_published: null }   # no explicit reset published; session "6:00 PM EST" open, positions auto-closed "4:10 PM EST" (see §4)
copy_trading_allowed: conditional   # copiers allowed on all account types; copying other traders and multi-account hedge/mirror/copy for unfair advantage prohibited (see §5)
automated_trading_allowed: conditional   # allowed unless HFT or aimed at exploiting sim fills; live must follow CME guidelines (see §6)
automation_rule_text: "High-frequency Trading is not allowed on our plans. Traders may make use of automated trading strategies tailored to their own specific settings so long as these automated tools do not aim to exploit the favorable fills offered in the Simulated Environment. Traders trading live accounts with automated trading must abide by CME guidelines."
vps_allowed: unknown   # VPS/server/VPN not mentioned on any official page (see §7a)
personal_device_required: unknown   # not required in words; shared devices with ANOTHER trader are forbidden (see §7a)
notes: "Live DLL not stated. Live trading is user-only."
status: partial
not_confirmed_fields: []
sources:
  - { field: drawdown, url: "https://help.myfundedfutures.com/en/articles/17589665-studio-plans-a-comprehensive-guide", accessed: "2026-10-23" }
  - { field: max_position, url: "https://help.myfundedfutures.com/en/articles/17589665-studio-plans-a-comprehensive-guide", accessed: "2026-10-23" }
  - { field: copy_trading_allowed, url: "https://help.myfundedfutures.com/en/articles/10771500-copy-trading-at-myfundedfutures", accessed: "2026-10-23" }
  - { field: copy_trading_allowed, url: "https://myfundedfutures.com/terms", accessed: "2026-10-23" }
  - { field: automated_trading_allowed, url: "https://help.myfundedfutures.com/en/articles/8444599-fair-play-and-prohibited-trading-practices", accessed: "2026-10-23" }
  - { field: trading_day_reset, url: "https://help.myfundedfutures.com/en/articles/9558251-permitted-times-to-trade", accessed: "2026-10-23" }
  - { field: platforms, url: "https://help.myfundedfutures.com/en/articles/8528335-overview-of-supported-platforms-at-mffu", accessed: "2026-10-23" }
```

#### Studio #003 — evaluation — 5K

```yaml
firm: myfundedfutures
plan: "Studio #003"
phase: evaluation
size_label: "5K"
size_usd: "5000.00"
instruments: [MNQ]
platforms: [Hyperprop]
drawdown: { amount_usd: "250.00", type: static, stops_trailing_at: "Static - set at start, never moves" }
daily_loss_limit: { amount_usd: null, kind: none }
profit_target_usd: "500.00"
consistency_rule: { text: "None", threshold_pct: null }
max_position: { minis: 0, micros: 5, scaling: "Micros only, no minis" }
min_trading_days: null
trading_day_reset: { time: null, tz_as_published: null }   # no explicit reset published; session "6:00 PM EST" open, positions auto-closed "4:10 PM EST" (see §4)
copy_trading_allowed: conditional   # copiers allowed on all account types; copying other traders and multi-account hedge/mirror/copy for unfair advantage prohibited (see §5)
automated_trading_allowed: conditional   # allowed unless HFT or aimed at exploiting sim fills; live must follow CME guidelines (see §6)
automation_rule_text: "High-frequency Trading is not allowed on our plans. Traders may make use of automated trading strategies tailored to their own specific settings so long as these automated tools do not aim to exploit the favorable fills offered in the Simulated Environment. Traders trading live accounts with automated trading must abide by CME guidelines."
vps_allowed: unknown   # VPS/server/VPN not mentioned on any official page (see §7a)
personal_device_required: unknown   # not required in words; shared devices with ANOTHER trader are forbidden (see §7a)
notes: "Below 25K priority range. Min trading days: none. 1 active Studio account (eval or Sim Funded)."
status: verified
not_confirmed_fields: []
sources:
  - { field: drawdown, url: "https://help.myfundedfutures.com/en/articles/17589665-studio-plans-a-comprehensive-guide", accessed: "2026-10-23" }
  - { field: drawdown, url: "https://myfundedfutures.com/studio", accessed: "2026-10-23" }
  - { field: max_position, url: "https://help.myfundedfutures.com/en/articles/17589665-studio-plans-a-comprehensive-guide", accessed: "2026-10-23" }
  - { field: copy_trading_allowed, url: "https://help.myfundedfutures.com/en/articles/10771500-copy-trading-at-myfundedfutures", accessed: "2026-10-23" }
  - { field: copy_trading_allowed, url: "https://myfundedfutures.com/terms", accessed: "2026-10-23" }
  - { field: automated_trading_allowed, url: "https://help.myfundedfutures.com/en/articles/8444599-fair-play-and-prohibited-trading-practices", accessed: "2026-10-23" }
  - { field: trading_day_reset, url: "https://help.myfundedfutures.com/en/articles/9558251-permitted-times-to-trade", accessed: "2026-10-23" }
  - { field: platforms, url: "https://help.myfundedfutures.com/en/articles/8528335-overview-of-supported-platforms-at-mffu", accessed: "2026-10-23" }
```

#### Studio #003 — funded — 5K

```yaml
firm: myfundedfutures
plan: "Studio #003"
phase: funded
size_label: "5K"
size_usd: "5000.00"
instruments: [MNQ]
platforms: [Hyperprop]
drawdown: { amount_usd: "250.00", type: static, stops_trailing_at: "Static; Sim Funded starts at $0" }
daily_loss_limit: { amount_usd: null, kind: none }
profit_target_usd: null
consistency_rule: { text: "None", threshold_pct: null }
max_position: { minis: 0, micros: 5, scaling: "Micros only, no minis" }
min_trading_days: null
trading_day_reset: { time: null, tz_as_published: null }   # no explicit reset published; session "6:00 PM EST" open, positions auto-closed "4:10 PM EST" (see §4)
copy_trading_allowed: conditional   # copiers allowed on all account types; copying other traders and multi-account hedge/mirror/copy for unfair advantage prohibited (see §5)
automated_trading_allowed: conditional   # allowed unless HFT or aimed at exploiting sim fills; live must follow CME guidelines (see §6)
automation_rule_text: "High-frequency Trading is not allowed on our plans. Traders may make use of automated trading strategies tailored to their own specific settings so long as these automated tools do not aim to exploit the favorable fills offered in the Simulated Environment. Traders trading live accounts with automated trading must abide by CME guidelines."
vps_allowed: unknown   # VPS/server/VPN not mentioned on any official page (see §7a)
personal_device_required: unknown   # not required in words; shared devices with ANOTHER trader are forbidden (see §7a)
notes: "Daily profit cap $2500 (excess forfeited). Payouts anytime."
status: verified
not_confirmed_fields: []
sources:
  - { field: drawdown, url: "https://help.myfundedfutures.com/en/articles/17589665-studio-plans-a-comprehensive-guide", accessed: "2026-10-23" }
  - { field: max_position, url: "https://help.myfundedfutures.com/en/articles/17589665-studio-plans-a-comprehensive-guide", accessed: "2026-10-23" }
  - { field: copy_trading_allowed, url: "https://help.myfundedfutures.com/en/articles/10771500-copy-trading-at-myfundedfutures", accessed: "2026-10-23" }
  - { field: copy_trading_allowed, url: "https://myfundedfutures.com/terms", accessed: "2026-10-23" }
  - { field: automated_trading_allowed, url: "https://help.myfundedfutures.com/en/articles/8444599-fair-play-and-prohibited-trading-practices", accessed: "2026-10-23" }
  - { field: trading_day_reset, url: "https://help.myfundedfutures.com/en/articles/9558251-permitted-times-to-trade", accessed: "2026-10-23" }
  - { field: platforms, url: "https://help.myfundedfutures.com/en/articles/8528335-overview-of-supported-platforms-at-mffu", accessed: "2026-10-23" }
```

#### Studio #003 — live — 5K

```yaml
firm: myfundedfutures
plan: "Studio #003"
phase: live
size_label: "5K"
size_usd: null
instruments: [MNQ, NQ]
platforms: []   # live platform not published
drawdown: { amount_usd: null, type: trailing_eod, stops_trailing_at: "Live MLL equal to Live starting balance (40% of total requested, capped at $2,000); trails EOD and locks at $100" }
daily_loss_limit: { amount_usd: null, kind: null }
profit_target_usd: null
consistency_rule: { text: null, threshold_pct: null }
max_position: { minis: 1, micros: 10, scaling: "#001/#002: same as plan (micros, no minis); #003: 1 mini / 10 micro" }
min_trading_days: null
trading_day_reset: { time: null, tz_as_published: null }   # no explicit reset published; session "6:00 PM EST" open, positions auto-closed "4:10 PM EST" (see §4)
copy_trading_allowed: conditional   # copiers allowed on all account types; copying other traders and multi-account hedge/mirror/copy for unfair advantage prohibited (see §5)
automated_trading_allowed: conditional   # allowed unless HFT or aimed at exploiting sim fills; live must follow CME guidelines (see §6)
automation_rule_text: "High-frequency Trading is not allowed on our plans. Traders may make use of automated trading strategies tailored to their own specific settings so long as these automated tools do not aim to exploit the favorable fills offered in the Simulated Environment. Traders trading live accounts with automated trading must abide by CME guidelines."
vps_allowed: unknown   # VPS/server/VPN not mentioned on any official page (see §7a)
personal_device_required: unknown   # not required in words; shared devices with ANOTHER trader are forbidden (see §7a)
notes: "Live DLL not stated. Live trading is user-only."
status: partial
not_confirmed_fields: []
sources:
  - { field: drawdown, url: "https://help.myfundedfutures.com/en/articles/17589665-studio-plans-a-comprehensive-guide", accessed: "2026-10-23" }
  - { field: max_position, url: "https://help.myfundedfutures.com/en/articles/17589665-studio-plans-a-comprehensive-guide", accessed: "2026-10-23" }
  - { field: copy_trading_allowed, url: "https://help.myfundedfutures.com/en/articles/10771500-copy-trading-at-myfundedfutures", accessed: "2026-10-23" }
  - { field: copy_trading_allowed, url: "https://myfundedfutures.com/terms", accessed: "2026-10-23" }
  - { field: automated_trading_allowed, url: "https://help.myfundedfutures.com/en/articles/8444599-fair-play-and-prohibited-trading-practices", accessed: "2026-10-23" }
  - { field: trading_day_reset, url: "https://help.myfundedfutures.com/en/articles/9558251-permitted-times-to-trade", accessed: "2026-10-23" }
  - { field: platforms, url: "https://help.myfundedfutures.com/en/articles/8528335-overview-of-supported-platforms-at-mffu", accessed: "2026-10-23" }
```

---

## 9. Unverified / gaps / conflicts (for the user report)

| # | Field | Plan / phase / size | Status | Detail and sources searched |
|---|---|---|---|---|
| 1 | daily_loss_limit | Builder eval + SF (all sizes) | conflict | Soft pause $600 / $1,000 / $1,750 / $2,500 (plan page + help center) vs "No daily loss limits on Builder, Rapid or Pro plans" (payout-policy article 13745661) |
| 2 | min_trading_days | Builder eval (all sizes) | conflict | "1 Day" (plan-page table, help center) vs "One phase · min 2 days" (plan-page stage card) |
| 3 | max_accounts | Builder 25K SF | conflict | "2 Builder 25K per user" (plan-page table, help center, limits article) vs "Builder is limited to one active Builder account per user" (plan-page FAQ) |
| 4 | payout max per cycle | Builder 150K SF | conflict | $3,500 (plan page) vs $4,500 (help center 17035331). Payout only, does not affect routing |
| 5 | drawdown.stops_trailing_at | Builder SF (all sizes) | conflict | "locks permanently once it reaches $100 above the starting balance" / "MLL Lock Level $100" vs "After your first payout, the Max Loss Limit resets to $100" (payout policy) |
| 6 | daily_loss_limit (live) | Builder Live 25K / 100K / 150K | conflict | plan page $600 / $1,750 / $2,500 vs help center "None" |
| 7 | drawdown lock (live) | Builder Live 100K / 150K | conflict | "Static once MLL reaches $0" (plan page) vs "Locks and remains static once it reaches $100 above the starting balance" (help center) |
| 8 | max_position (live) | Builder Live 25K / 50K | not found | Only 100K (3/30) and 150K (4/40) are published |
| 9 | drawdown.stops_trailing_at | Rapid SF (all sizes) | conflict | "Once your trailing Max Loss reaches $100, it locks there" (help center) vs "+$100 above your starting balance after your first payout" (plan-page FAQ) |
| 10 | max_position.scaling | Rapid 50K SF | conflict | Cross-instrument policy: "you start off with 2 contracts … due to the scaling plan" vs 5 minis / 50 micros (plan page, help center); no scaling plan published elsewhere |
| 11 | max_position (live) | Rapid EOD 50K Live | conflict | plan page 4 / 40 vs Rapid 50k Live 3 / 30 (the help center says Rapid EOD 50k follows it) |
| 12 | max_position.micros | Pro SF 50K / 100K / 150K | anomaly (both sources agree) | Published as "3 mini / 3 micro", "6 / 6", "9 / 9"; evaluation uses 1:10. May be a typo; ask the firm/user |
| 13 | drawdown type, DLL, contracts (live) | Pro Live | conflict | Help center: EOD trailing, DLL None, Max Contracts 2/3/4 (no unit) vs plan-page FAQ: "including a daily loss limit, reduced contract limits, and a static drawdown floor" |
| 14 | payout max | Pro SF | conflict | "$100,000 (per user)" (help center) vs "Up to $100,000 per payout cycle" (plan page); payout only |
| 15 | availability | Pro 50K One Day add-on | unclear | Current Pro and payout articles list it; its own article is in "Legacy Plans" and says "For a limited time" |
| 16 | daily_loss_limit (live) | Rapid, Rapid EOD, Studio Live | not found | Not stated on the live parameter tables; the generic live article mentions a "Customizable Daily Loss Limit" |
| 17 | live drawdown (generic) | All Live | conflict (generic vs plan) | Generic pages: "Static Drawdown" / "static and is set at $140 as the minimum balance" vs plan-specific EOD trailing tables |
| 18 | trading_day_reset | all | not published | No explicit reset statement. Searched: Permitted Times to Trade, instrument list, EOD drawdown article, plan pages, live FAQ, dashboard article. Published: 6:00 PM EST open, 4:10 PM EST auto-close |
| 19 | copy_trading (same person) | all | implicit | Allowed "across all account types" / "across multiple accounts", all accounts must be the same verified individual; no literal sentence about copying between one's own accounts |
| 20 | copy to/from other firms | all | not mentioned | Searched: copy article, Fair Play, Terms, hedging article, live FAQ |
| 21 | webhooks / alerts, semi-automated | all | not mentioned | Searched the same pages plus all help-center articles (keyword search) |
| 22 | news scope | all | wording | "Standard Trading Protocols" say no positions or orders 2 minutes before/after "any data release" and "apply to all news releases", while only T1 is listed as restricted for Rapid/Pro SF; Builder and evaluations are "unrestricted" |
| 23 | platforms (live) | all Live | not published | Only "Platform License or Subscription" as a live cost |
| 24 | vps_allowed | all | not mentioned | No VPS/server/VPN statement; searched as listed in §7a |
| 25 | personal_device_required | all | not stated | Only a ban on sharing a device with another trader (Fair Play §4; article 8230018). Whether a shared VPS counts is not stated |

## 10. Pages not reached

None. All pages were read with `curl` (no Cloudflare block). The Rapid plan page was also read rendered in the Maestri portal "Lupa MFFU" to confirm the tab content ($150K / All). Legacy-plan articles (Flex, Builder 25k Legacy No DLL, legacy Pro) were captured but are not part of the presets.

## 11. Changelog

- 2026-10-23 — initial research (Lupa)
- 2026-10-23 — added VPS/VPN and personal-device fields per format rev. 3 (§7a, YAML `vps_allowed`, `personal_device_required`, gaps 24–25) (Lupa)
- 2026-10-23 — rev bump R4: VPS and personal-device re-checked on all 77 help-center articles, Terms and Privacy Policy; no explicit text either way, `vps_allowed` and `personal_device_required` stay `unknown`, gaps 24-25 stay open (Lupa)
