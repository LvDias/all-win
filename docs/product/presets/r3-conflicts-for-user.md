# R3 Presets — Conflicts, Gaps and Scope Questions for the User

> **Owner:** Product Owner · **Started:** 2026-10-23 · **Status:** scope questions S-R3-1 to S-R3-5 DECIDED (a) by the user on 2026-10-24 [Decision 2026-10-24]; the ASK rows in section 2 stay open · **Sources:** [research/](research/) R3 files.
> Codes as in [r1-conflicts-for-user.md](r1-conflicts-for-user.md): **NC** not confirmed (user picks per account) · **ASK** user asks the firm · **PLAN** plan-specific value · **SKIP** out of V1 presets · **INFO** information only.
> Firms (V1-R3): Elite Trader Funding, Earn2Trade, Funded Futures Family, Phoenix Trader Funding, OneUp Trader, BluSky, Leeloo Trading.
> Progress: Elite, FFF and Phoenix filled from Lupa's consolidation (2026-10-23); BluSky, Earn2Trade, OneUp and Leeloo rechecked against their research files (2026-10-24, rows 20–50). Leeloo ships only what is published (PO night decision 2026-10-24, **PROVISIONAL — confirm with the user**; reversible with a later release).

## 1. Scope questions (decide first)

One row per firm that raises a scope question. Two kinds are expected, using the same treatment as R2 (S-1, S-8):

| Kind | Trigger | Standard options | Default recommendation |
|---|---|---|---|
| **Policy block** | The firm forbids automation, copying or a server/VPS/remote origin of orders (device rule, R2 S-5), so the product's automation or copy stance would be "forbidden" | (a) manual-tracking-only preset, no automated source or copy offered (as Topstep); (b) conditional preset with the verbatim warning and explicit opt-in (Q24/Q25); (c) exclude the firm from V1 | (a) when the firm forbids it outright; (b) when the text is conditional |
| **Platform unreachable** | None of the firm's platforms can be reached by NinjaTrader 8 through the adapter (no NinjaTrader/Rithmic/Tradovate route) | (a) manual-tracking-only preset; (b) exclude; (c) wait for a later adapter | (a), as Alpha (R2 S-8) |

Source: Lupa's consolidation (`research/consolidation-vps-platforms-gaps.md`, worktree docs-presets-research, §1–§4, D1–D10 are Lupa's proposals). Per-firm research files for R3 are not in this worktree yet; findings below are the consolidation's and are re-checked against each file before data entry.

**Already decided, so not asked again** (the PO maps Lupa's proposals to the decisions log):
- **D1 / D2 / D3** are covered by Q24/Q25 and S5 AC-7: a firm that forbids, conditions or does not publish its automation rule gives a **required choice, nothing preselected, off by default**, with the verbatim quote and the risk warning; All Win never blocks silently and never states a rule the firm did not publish.
- **D4 / D5** are covered by S-5 (VPS / personal-device rule recorded per firm, shown as information; "not published" when unknown). The NT8 plugin runs on the user's own PC.
- **D7 conflicts with a decided rule:** a conflicting field ships as **NC**, is not pre-filled, and the product **never picks the stricter value** (S16 AC-5, AC-6; decision 2026-10-23b). Recommend **rejecting D7**.
- **D9** is covered by S16 AC-12 / S13 AC-11: field starts empty, warning "comissão não informada".

| # | Firm | Kind | Finding (from the consolidation) | Options | Recommendation |
|---|---|---|---|---|---|
| S-R3-1 | Funded Futures Family | Policy block | Automated order entry forbidden on **all plans**; platforms reachable (NinjaTrader, Tradovate; Pro = Rithmic). Copy of own accounts allowed but ambiguous against the automation ban | (a) Keep the decided rule: preset offered, required opt-in with the verbatim ban and risk warning (Q24); (b) manual-tracking-only preset for this firm (no automated source, no copy); (c) exclude | **(a)** — the user decides for their own account with the firm's text in front of them; (b) is the safer product stance if the user wants no exposure. **Needs the user** because the ban covers every plan, unlike conditional firms **DECIDED (a), 2026-10-24** |
| S-R3-2 | Phoenix Trader Funding | Policy block + partial platform | Bots forbidden on evaluation and pre-funded (17 blocks), conditional on live (9); bot ban on live "ambiguous" (NC); VPS "yes, connections are monitored". NT8 reaches it only through the Tradovate TFeed path or the Thor add-on (orders hit Sim101); dxFeed platforms are not reachable | (a) Preset per platform: routable on the NT8-reachable path with the opt-in, **manual-tracking-only** on dxFeed platforms; (b) manual-tracking-only for all Phoenix; (c) exclude | **(a)** — S16 AC-1 already stores rules per platform. **Ask:** is the Thor path (orders on Sim101) acceptable as a supported route, or only the Tradovate path? **DECIDED (a), 2026-10-24** |
| S-R3-3 | Elite Trader Funding | Policy block | Bots only with the firm's **written authorisation**; VPN/VPS/proxy prohibited vs copier text "legitimate VPS" (conflict, NC); approved copiers only. Platforms reachable | (a) Treat as conditional, opt-in off by default, the warning tells the user to obtain the firm's written authorisation first (D2); (b) treat as forbidden (as S-R3-1) | **(a)** **DECIDED (a), 2026-10-24** |
| S-R3-4 | BluSky, Earn2Trade, OneUp, Top One Futures (and Phoenix) | Platform partly unreachable | BluSky: Volumetrica, Tradesea, Tickblaze, BlackArrow unreachable (Rithmic and Tradovate paths reachable). Earn2Trade: BlackArrow unreachable; the Rithmic path needs the user's own NinjaTrader licence. OneUp: NinjaTrader Web/Mobile unreachable. Top One: Tradovate path only | (a) Show the preset with the firm's platform on the account; an account on an unreachable platform is **manual-tracking-only** with the reason (S16 AC-7); (b) hide those platform variants; (c) hide the whole firm | **(a)** (Lupa's D6, aligned with S16 AC-7). The preset data stays useful for limits and risk **DECIDED (a), 2026-10-24** |
| S-R3-5 | Earn2Trade | Policy | Copy trading not allowed (`copy: no`) | (a) Copy stance `forbidden`: follower opt-in with the verbatim text (Q25); (b) the account cannot be a follower | **(a)** — consistent with Q25 **DECIDED (a), 2026-10-24** |

Lupa's other proposals: **D6** is S-R3-4; **D8** (add `unknown` to the research format's set for a funded daily-loss kind, instead of mapping it to `none`, which would invent a rule) — a research-format rule, not a business rule: **done** in format rev. 3; **D10** (publish the rev. 3 format doc: platforms, automation text, device fields, commission, `status` values) — **done**, [preset-research-format.md](preset-research-format.md) rev. 3 (2026-10-24).

## 2. Conflicts and gaps

Same table as R2 section 2 (firm, item, finding, recommendation code). Each firm's own "Unverified / gaps / conflicts" list (research format §3.9) is copied here with one code per row. Conflicting or missing fields ship as **NC** and never block the rest of the preset.

| # | Firm | Item | Finding | Recommendation |
|---|---|---|---|---|
| 1 | Elite | News rule on funded accounts | Help: no restriction vs ToS: flat 5 min before/after | **INFO** (not used by the product) |
| 2 | Elite | VPS wording | ToS bans vs copier text "legitimate VPS" | **NC**; shown as information (S-5) |
| 3 | Elite | 1-Step 250K | Listed vs "no longer sold" | **NC + ASK** (is the plan offered?) |
| 4 | Elite | Static 25K floor | $24,100 vs $24,000 | **NC** (drawdown is required; user picks) |
| 5 | Elite | Direct to Funded target label; DTF 25K existence | Tables disagree | **NC** |
| 6 | Elite | 150K drawdown, target, daily loss limit | Unpublished | **NC + ASK** (required fields) |
| 7 | Elite | Trading-day reset; personal device | Unpublished | **NC + ASK** (reset); **INFO** (device) |
| 8 | FFF | VPS | "permitted" vs "not allowed" | **NC**; information |
| 9 | FFF | Standard S2F consistency | 25% vs 45% | **NC** (warning-only, Q4) |
| 10 | FFF | Velocity funded consistency | 40% vs none | **NC** (warning-only) |
| 11 | FFF | Pro funded drawdown type | EOP vs EOD | **NC + ASK** — affects enforcement |
| 12 | FFF | Premier+/S2F 25K max position | Funded table vs scaling plan | **NC** (max position is required; user picks) |
| 13 | FFF | Commission, trading-day reset, personal device | Not published | commission: empty + warning (S16 AC-12); reset **NC + ASK**; device **INFO** |
| 14 | FFF | Copy vs automation ban | Copy of own accounts allowed, automated entry banned | see S-R3-1 |
| 15 | Phoenix | Merit tiers | Marketing page $2K only vs help center $10K/50K/100K | **NC** (plan existence) + **ASK** |
| 16 | Phoenix | Merit ES commission | 1.99 vs 2.18 / 1.09 | **NC** (floor starts empty, S16 AC-12) |
| 17 | Phoenix | Bot ban on live | Ambiguous | **NC**; stance `conditional`, text shown (S-R3-2) |
| 18 | Phoenix | Trading-day reset and time zone; personal device | Not published | **NC + ASK** (reset); **INFO** (device) |
| 19 | Phoenix | Live drawdown amounts and starting size | Not published | **NC + ASK** (required fields) |
| 20 | BluSky | Copy stance | Copier allowed (help "Trade Copier Allowed: Yes") vs conduct policy forbids "Account Mirroring" | **NC**; verbatim texts shown (Q25) |
| 21 | BluSky | NinjaTrader on Rithmic: phase scope | "Evaluation, BluLive and Sim Funded only" vs "Evaluation Only" | **NC** (platform availability per phase); Tradovate route unaffected |
| 22 | BluSky | Propel 150K/200K consistency | 30% vs 34% | **NC** (warning-only, Q4) |
| 23 | BluSky | Propel 300K evaluation daily loss | "Capped at $2,500" vs "None" | **NC** (required) |
| 24 | BluSky | Orbit 200K evaluation contracts; Orbit Buffer phase | 2 mini/20 micro vs 3/30; Buffer phase listed on one page only | **NC** (max position is required); Buffer phase **NC** (existence) |
| 25 | BluSky | Sim Funded contract limits; starting balance 150K/300K; minimum balance | Three sources disagree; $2,000/$3,500 vs $1,500/$3,000; $100 above/below vs older table | **NC**; Launch/Propel max position stays empty |
| 26 | BluSky | Consistency method wording; Orbit Sim Funded start | Largest day vs sum of profitable days vs target increase; $50,000 start with lock vs $0 start | **NC** |
| 27 | BluSky | Trading-day reset | Only "6PM"/"6PM EST"; no table, no holiday rule | **NC + ASK** (reset) |
| 28 | BluSky | Daily loss kind (Instant/Direct); Sim Funded daily loss; trailing lock point | Hard vs soft not stated; none stated (Risk Team may impose); lock unclear | **NC** (kind `unknown`, not `none`; format rev. 3) |
| 29 | BluSky | Commission Tradovate/NinjaTrader, funded, Buffer, Tickblaze, BlackArrow | No basis, or "set by brokerage" | empty + warning (S16 AC-12) |
| 30 | BluSky | Automation / VPS / device / copying with other firms | Not published; only "automated software" clause in the conduct policy | automation `unknown`: required choice, generic warning plus the verbatim clause (S5 §5.1); VPS/device **INFO** "not published" |
| 31 | BluSky | Plan x platform availability; Live parameters; Project X | Dynamic or behind login; not published | **NC**; platform chosen on the account (S-R3-4 a) |
| 32 | Earn2Trade | Commission | MNQ not published (range text only); NQ per platform; funded none | empty + warning |
| 33 | Earn2Trade | Funded daily loss behaviour (hard vs soft) | Not mapped on the page | **NC** (kind `unknown`) |
| 34 | Earn2Trade | Fixed drawdown 200K (via TCP25) and 400K | Published as a minimum balance, not an amount | **NC** (never derived) |
| 35 | Earn2Trade | Automation stance | `conditional` is the researcher's reading of one clause ("software, artificial intelligence, or ultra-fast data entry ... unfair advantage") | required choice, nothing preselected; verbatim clause shown (S5 §5.1) |
| 36 | Earn2Trade | NinjaTrader route | New subscriptions: NinjaTrader via Tradovate only; legacy NinjaTrader on Rithmic honored until 2026-06-30 (passed) | update S-R3-4: NT8 reachable via the Tradovate option; Rithmic route needs the user's own license (not routable by plan) |
| 37 | Earn2Trade | VPS / device / copying with other firms | Not mentioned | **INFO** "not published" |
| 38 | Earn2Trade | Partner-firm funded rules (Helios/Appius/Kronos) | Not covered by E2T pages | **INFO**; funded presets carry only what E2T publishes |
| 39 | OneUp | Funded drawdown amount | From homepage card, not phase-specific | **NC** for the funded phase |
| 40 | OneUp | Drawdown unrealized treatment | Not stated | **NC** (type as published, shown unconfirmed) |
| 41 | OneUp | Profit targets | Homepage only | verified (single official source) |
| 42 | OneUp | Balance resets "Unlimited" vs $50/$100; Express minimum days 3 vs 5 | Conflicting; no routing or risk effect | **INFO** (not used by the product) |
| 43 | OneUp | Reset time | "5 PM CT" only as start of day; no daily limit reset event | **NC + ASK** (reset) |
| 44 | OneUp | Automation / VPS / device / copying with other firms | Not published | automation `unknown` (required choice, generic warning); **INFO** |
| 45 | OneUp | Funded scaling and commission (images) | Read visually | **PLAN**; re-check before release |
| 46 | OneUp | Platform | Rithmic only; NinjaTrader 8.1 desktop reachable; Web/Mobile not | S-R3-4 (a) |
| 47 | Leeloo | Scope of public data | KB behind login: drawdown, min days, instruments, Elite/Entry/Weekly/Ignite not read; PA size-specific drawdown, contracts, split not published | preset ships only what is published; every other required field **NC** (principle of Q94/Q97); programs behind login not covered |
| 48 | Leeloo | Trading-day reset, drawdown lock, instruments (MNQ/NQ), commission | Not published | **NC + ASK** (reset, instruments); commission empty + warning |
| 49 | Leeloo | Copying | Cross-user forbidden; same-person unknown; a public support news item on copier limits (research §7) | **NC**; re-read the news item before data entry |
| 50 | Leeloo | Automation / VPS / device | Not published | automation `unknown`; **INFO** |

## 3. How the user answers

One message per batch: scope questions first (a/b/c per row), then only the **ASK** rows. Everything coded NC, PLAN, SKIP or INFO needs no answer.
