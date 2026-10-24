# R2 Presets — Conflicts, Gaps and Scope Questions for the User

> **Owner:** Product Owner (Bússola @ Dev2) · **Started:** 2026-10-23 · **Updated as Lupa delivers each firm** · **Sources:** [research/](research/) R2 files.
> Codes as in [r1-conflicts-for-user.md](r1-conflicts-for-user.md): **NC** not confirmed (user picks per account) · **ASK** user asks the firm · **PLAN** plan-specific value · **SKIP** out of V1 presets · **INFO** information only.
> Progress: Topstep ✔ (rev. 3) · Take Profit Trader ✔ · MyFundedFutures ✔ · Tradeify ✔ · Bulenox ✔ · TradeDay ✔ · Alpha Futures ✔ · Top One Futures ✔ — **R2 complete** (8 files, all `partial`, 255 YAML blocks).

## 1. Scope questions (decide first)

**Decided 2026-10-24:** Take Profit Trader page titles that cannot be retrieved (the site blocks automated access) stay blank with a note and the URL, also in quotes; row 8 below corrected to NC. Q94 (a): Topstep presets cover the Trading Combine only.
> **Decided 2026-10-23:** S-1 (a) manual-tracking-only Topstep preset, (c) for future evaluation; S-2 (a); S-3 and S-4 excluded; S-5 adopted (VPS/personal-device rule recorded per firm). S-6 and section 2 remain open.

| # | Firm | Question | Finding | Options | Recommendation |
|---|---|---|---|---|---|
| S-1 | Topstep | **Can All Win serve Topstep accounts at all?** | "TopstepX™ is the only available trading platform" (Combine/XFA also Quantower via ProjectX and the ProjectX API; LFA only TopstepX). No NinjaTrader, Tradovate or Rithmic. Orders must "originate from your personal device"; "routing or relaying orders on your behalf" from a private server is not allowed; VPS/VPN prohibited | (a) Ship the Topstep preset in V1 for **manual tracking only** (risk rules, warnings, manual trades; no routing), with the platform limit explained; (b) leave Topstep out of V1 presets; (c) plan a future adapter that runs on the user's own device via the ProjectX API (feasibility asked to the Architect) | **(a)** now — the vision ships all 18 firms; routing is impossible in V1 and must be shown clearly. (c): Architect confirms it fits the architecture (new on-device adapter, no Core change) but **not** the firm's rule — the decision comes from our cloud Core, which Topstep may read as "relaying orders on your behalf"; only Topstep can say (user asks in writing); the ProjectX API may need its own subscription and terms |
| S-2 | Take Profit Trader | Does All Win count as an **"approved" trade copier**? | Copying is allowed only between the trader's own accounts and only with approved copiers ("solely for managing accounts owned and controlled by you"); unapproved copiers "may result in enforcement action". Automation is forbidden in every phase ("No Trading Bots or Algos") | (a) Copy stance = conditional, show the text; the user asks the firm; (b) treat as forbidden until approved | **(a)** + **ASK** — the automation opt-in (option A) already applies because automation is forbidden |
| S-5 | All firms | **VPS / "personal device" rules** | Topstep (rev. 3, all plans): "All trading activity must originate from your personal device. The use of VPS, VPNs, and remote servers is prohibited"; "your server can watch and record, but it cannot trade". MyFundedFutures: not addressed. Tradeify: conditional ("You cannot log in with VPN/VPS. After logging in normally, you may use VPS for trading at your own risk"). Take Profit Trader: not addressed (VPN only banned at sign-up/restricted countries). **TradeDay: forbidden** ("TradeDay does not allow the use of virtual private servers (VPS)"). Bulenox: not addressed. Alpha Futures: allowed ("The use of VPS is permitted in your trading"; VPN forbidden; neither during KYC). Top One Futures: conditional (fixed IP, notify support with IP and invoice; VPN not allowed). Topstep forbids VPS/VPN and requires orders to originate from the personal device. If other firms do the same, running NinjaTrader + plugin on a VPS (e.g., the Windows VPS near Chicago) could breach their rules | (a) Research records, per firm, whether VPS/servers are allowed and whether a personal device is required, and the account screen warns when the instance runs where the firm forbids; (b) ignore | **(a)** — Lupa asked to add the field (R2 onward; R1 at its rev. 2) |
| S-6 | MyFundedFutures | Studio plans 5K/10K/20K (micros only, Hyperprop only) | Below the 25K–300K range; platform not supported in V1 | (a) SKIP in V1 presets; (b) include | **(a)** |
| S-7 | TradeDay, Bulenox (and all firms) | **Is All Win an "automated trading system" or a "trade copier"?** (TradeDay asks it implicitly; Bulenox allows only user-built tools, or third-party tools approved by its management) | Product view (Q24, decided): the TradingView webhook is an **automated source** (automation opt-in) and master replication is **copying** (copy opt-in). How each **firm** classifies All Win is only for the firm to say | (a) Keep the product view; show both stances and let the user ask the firm; (b) treat All Win as third-party automation everywhere | **(a)** + **ASK** for Bulenox (third-party approval) and TradeDay |
| S-8 | Alpha Futures | **No V1 platform reaches Alpha** (AlphaTrader, TradingView via Plus500, WealthCharts, DeepChart, Quantower; no NinjaTrader/Tradovate/Rithmic). Automation forbidden on all accounts | Same situation as Topstep (S-1) | (a) Manual-tracking-only preset in V1 (same treatment as Topstep); (b) exclude Alpha | **(a)** — consistent with S-1 |
| S-3 | Topstep | Topstep Labs Challenges and Pro Account (text only in research) | Not standard plans | (a) SKIP in V1 presets; (b) include | **(a)** |
| S-4 | Take Profit Trader | PRO+ (live, invite-only) and PRO+ Development | Max position and platforms not published | (a) SKIP; user registers manually; (b) partial preset | **(a)** |

## 2. Conflicts and gaps

| # | Firm | Item | Finding | Recommendation |
|---|---|---|---|---|
| 1 | Topstep | Day close for "best day / winning day" counting | 3:10 PM CT vs 4:00 PM CT | **NC** (affects consistency tracking only, warning-only in V1) |
| 2 | Topstep | LFA max micros | Not published | **NC**; user types it |
| 3 | Topstep | Scaling plan tiers | Published only as an image | **NC** + Lupa to transcribe with the image source cited, if legible |
| 4 | Topstep | Combine minimum trading days | Gap | **SKIP** (not used by the product) |
| 5 | Topstep | Third-party copiers, webhooks, semi-automation | Not named | **INFO**; stance conditional with text shown |
| 6 | Topstep | Labs plan details | Partial | **NC** where missing |
| 7 | Take Profit Trader | Consistency boundary | "below 50%" vs "cannot be more than 50%" | **NC** (warning-only in V1) |
| 8 | Take Profit Trader | Reset time zone | "Eastern" vs "EST" (5 PM; auto-flatten 4:55 PM) | **NC** (corrected 2026-10-24 from PLAN: a conflicting required field is never picked by the product, S16 AC-5/AC-6); two published values, the user picks; **ASK** if the user wants certainty |
| 9 | Take Profit Trader | Whether minis and micros share one limit | Gap | **NC**; one max position per account in V1 (Q-A1) |
| 10 | Take Profit Trader | Copying to/from other firms | Not addressed; opposite positions across own accounts lead to liquidation | **INFO**; show the opposite-positions rule in the account warning |

| 11 | MyFundedFutures | Trading-day reset | Not published as a reset (open 6:00 PM EST, auto-close 4:10 PM EST) | **NC** with published times as hints; **ASK** |
| 12 | MyFundedFutures | Builder daily loss | Soft pause vs "No daily loss limits" | **NC + ASK** — affects enforcement |
| 13 | MyFundedFutures | Builder minimum days | 1 vs 2 | **SKIP** (not used by the product) |
| 14 | MyFundedFutures | Builder 25K account count | 2 vs 1 | **INFO** (account counts are not enforced) |
| 15 | MyFundedFutures | Rapid 50K and Rapid EOD Live max contracts | Conflicting values | **NC** (max position is required; user picks) |
| 16 | MyFundedFutures | Pro Live drawdown type, daily loss, contracts | Conflicting | **NC + ASK** |
| 17 | MyFundedFutures | Pro micros published as "3/3" | Likely typo | **NC** (never corrected by the product); **ASK** |
| 18 | MyFundedFutures | Live platform | Not published | **NC** (user picks platform on the account) |
| 19 | MyFundedFutures | Copying / automation | Copying allowed across own accounts, but not "to hedge, mirror, copy, or coordinate trades … unfair advantage"; NQ vs MNQ hedge forbidden. Automation allowed if it does not "exploit the favorable fills"; HFT forbidden; webhooks not named | Both stances **conditional** → explicit opt-ins (Q25), texts shown |
| 20 | MyFundedFutures | Remaining items | 25 gaps/conflicts listed in the research file | Default **NC**; reviewed with the user later |

| 21 | Tradeify | Select eval 25K max contracts | 1/10 vs 2/20 | **NC** |
| 22 | Tradeify | 300K V2 daily loss | Soft pause vs hard breach | **NC + ASK** — affects enforcement |
| 23 | Tradeify | Elite Live 150K/300K initial contracts | Break the 10:1 micro ratio | **NC** |
| 24 | Tradeify | **DCA / adding to positions** | Help center allows; FTA §6.5 discourages | **NC + ASK** — All Win adds to the position on repeated entries (vision §6), so this matters for the opt-in text |
| 25 | Tradeify | "10-second" rule wording | Ambiguous | **INFO** + ASK (may concern minimum hold time; product has no hold-time control) |
| 26 | Tradeify | Elite Live platform; "Advanced Challenge" | Not published / undocumented | **NC**; Advanced Challenge **SKIP** until documented |
| 27 | Tradeify | Copying / automation | Copy only between own accounts, same direction; up to 5 Sim Funded per person/household (8 with 300K V2). Bots allowed under conditions (sole owner, not "across multiple firms", no HFT); copiers and ATM allowed | Both stances **conditional** → explicit opt-ins (Q25), texts shown. "Not across multiple firms": **INFO** in the warning (All Win routes one source to several firms' accounts) — **ASK** |
| 28 | Tradeify | Legacy versions | Text only | **SKIP** (consistent with Apex Legacy) |

| 29 | TradeDay | Platform for NinjaTrader | NT8 only via Tradovate: "we do not support NinjaTrader Desktop with Rithmic accounts" | **INFO** — the platform chosen on the account (S5 AC-1) must be Tradovate for routing via the plugin; warn when Rithmic is chosen |
| 30 | TradeDay | Daily loss | None by the firm ("ONE RULE"); whether "none" is a value | **PLAN**: `none` is a valid value (no firm DLL); the user may still set a personal one |
| 31 | TradeDay | Copying | "Trade copiers are allowed from your personal accounts or other evaluations in to your TradeDay account"; up to 10 accounts, Live up to 5; never between Live and other accounts | **Conditional** → copy opt-in; Live↔non-Live is **INFO** in the warning |
| 32 | TradeDay | Automation | Only on supported platforms; third-party purchased algos/bots forbidden; > 200 trades/day forbidden | **Conditional** → opt-in; see S-7 |
| 33 | TradeDay | C1–C5 (min days, Live limits, slippage buffer, Fast Pass Live drawdown, 100K payout cap) | Conflicting | **Live limits, Fast Pass Live drawdown: NC + ASK**; min days/payout **SKIP** |
| 34 | Bulenox | Third-party tools (C1) | FAQ forbids third-party tools; Terms allow those "reviewed and approved by the Bulenox management team before use" | **NC**, both texts in the opt-in warning; **ASK** (S-7) |
| 35 | Bulenox | Funded (live) rules | Not published | **NC**; user types them |
| 36 | Bulenox | Platform | Rithmic only (NinjaTrader with free licence) | **INFO** — reachable by the V1 plugin |
| 37 | Bulenox | Remaining N1–N4, G1–G10 | In research file | Default **NC**; later review |
| 38 | Alpha Futures | Copying (conflict) | Help center: "permitted as long as it is a single user executing the trades"; T&C §8.1.5: "the only copy trading allowed is that from your own external account to your Alpha Futures Account" | **NC**, both texts (relevant only if S-8 changes) |
| 39 | Alpha Futures | MLL Zero 100K and Advanced Qualified | 4% (T&C) vs USD values (help center) | **NC + ASK** |
| 40 | Alpha Futures | Advanced Eval min days; Zero Eval DLG; Standard 100K micros; max accounts (3 vs 5) | Conflicting | DLG/micros **NC**; min days and account count **SKIP/INFO** |
| 41 | Alpha Futures | Live rules, Direct min days, Zero 25K in T&C, holidays | Gaps | **NC**; Live by invitation → **SKIP** like other invite-only live programs (pending S-8) |

| 42 | Top One Futures | Automation | Forbidden in every phase: "Use of any automated trading system, script, bot, Expert Advisor (EA)… is strictly forbidden. All trades must be entered manually or semi-manually"; ATM documented as optional | **Forbidden** → automation opt-in with this text |
| 43 | Top One Futures | Copying | "copy trading is strictly limited to accounts of the exact same type and exact same size" | **Conditional** → copy opt-in; the warning must state same type **and size** (a master → follower of another size would breach). Product does not enforce it in V1 — **INFO**; candidate rule for later |
| 44 | Top One Futures | Same-IP rule vs allowed copying | "Trading multiple accounts from the same IP" restricted vs copying allowed | **NC + ASK** — relevant because one NinjaTrader instance holds several accounts |
| 45 | Top One Futures | 150K daily loss | $1,750 vs $1,850 vs n/a | **NC + ASK** |
| 46 | Top One Futures | Copying with other firms | Help center allows; T&C forbid | **INFO** (not a V1 feature); show T&C text |
| 47 | Top One Futures | Session/reset | Session 6:00 PM–4:10 PM EST, forced close 4:10 PM; Instant/Elite: "a trading day ends at 5:00 pm EST"; Ignite close time conflicting | **NC** (user picks) |
| 48 | Top One Futures | Max accounts (Ignite, Instant); VPN wording; news on X-Daily Spark | Conflicting/ambiguous | **INFO/SKIP** (not enforced by the product) |
| 49 | Top One Futures | Plans not for sale (Elite, S2F Sim Pro, X-Ultra) | Text only | **SKIP** |
| 50 | Top One Futures | Platform per plan | Firm-wide list chosen at checkout (Tradovate/NinjaTrader, BlackArrow, WealthCharts), no change afterwards | **INFO** — routing needs Tradovate/NinjaTrader chosen on the account |

## 3. Product notes

- **Opposite positions across the same person's accounts** (Take Profit Trader liquidates them) — All Win routes the same instruction to all subscribed accounts, so opposite positions only arise from manual trades or rejected/partial routing; the out-of-sync flag (S11) already alerts. **INFO** for the preset text.
- **No overnight positions** (Take Profit Trader, auto-close 4:55 PM ET; Topstep flat by 3:10 PM CT): candidate for a preset "must be flat by" time with a warning — to propose after R2 if more firms have it.
