# R1 Presets — Conflicts and Unverified Items for the User

> **Owner:** Product Owner (Bússola @ Dev2) · **Date:** 2026-10-23 · **Sources:** Lupa's R1 files in [research/](research/) (Apex §8, Phidias §8, Lucid §11), all status `partial`.
> **Decided 2026-10-23:** recommendations below accepted by the user, Apex Legacy and Phidias LIVE excluded; item-by-item review later. **Default:** conflicting items ship as **"not confirmed"** without blocking the rest of the preset; the user sees each published value with its source and picks one or types their own on the account (Designer O7). This list is for the user to decide **later**; nothing here blocks development or the other preset fields.
> Recommendation codes: **NC** = keep "not confirmed" (user picks per account) · **ASK** = user asks the firm's support and the answer is recorded with date · **PLAN** = use the plan-specific value under the firm's own precedence clause · **SKIP** = leave out of V1 presets · **INFO** = shown as information only, no preset field.

## 1. Items that affect risk limits or routing (decide first)

| # | Firm | Item | What conflicts / is missing | Recommendation |
|---|---|---|---|---|
| 1 | Phidias | Trading-day reset (Fundamental/Premium) | Three official times: 10:00 PM UTC+2 · 9:59 PM UTC · 20:59 UTC; no daylight-saving rule | **ASK + NC.** Reset drives the daily loss and the block; until confirmed, the user sets it on the account |
| 2 | Phidias | E2L reset | Not published at all | **ASK**; until then the user sets it on the account (field required) |
| 3 | Lucid | Trading-day reset | Not published as a reset (close-out 4:45 PM ET, reopen 6:00 PM ET) | **ASK**; meanwhile NC with the published times shown as hints |
| 4 | Apex | Intraday PA: level where trailing stops | "Starting Balance + $100" vs "Max Trailing Drawdown Amount + $100" | **NC + ASK** — changes the drawdown line |
| 5 | Apex | Intraday eval (Rithmic/WealthCharts): trailing stop point | "Target Profit Balance" vs "Profit Target Balance + $2,000" (examples agree on $53,000 for 50K) | **NC**, show the example value as a hint |
| 6 | Apex | 50K PA Level 4 threshold | $6,000 & Up vs $5,999 & Up | **NC** ($1 difference; low risk) |
| 7 | Phidias | Funded EOD trailing stop level, 150K | Published only for 50K/100K | **NC** (no value); user types it; **ASK** |
| 8 | Lucid | LucidPro 25K daily loss limit | "None" (help center) vs "$600" (homepage, funded overview) | **NC + ASK** — a DLL vs none changes enforcement |
| 9 | Lucid | LucidFlex DLL amounts | Homepage only ($600/$1,200/$1,800/$2,700); DLL above Initial Trail not published | Accept the homepage values as **single-source, not confirmed**; above-trail **NC** |
| 10 | Lucid | LucidDaily eval consistency | 50% vs "40% standard / 50% add-on" | **NC** — consistency is only tracked/warned in V1, so low risk |
| 11 | Lucid | LucidFlex eval consistency | 40% (50% with add-on) vs 50% | **NC** (same reason) |
| 12 | Apex | 50% consistency boundary | "below 50%" vs example showing exactly 50% as met | **NC**; warning-only in V1 |
| 13 | Phidias | Min trading days to pass | General "3" vs plan-specific 1 / 0; the firm says a specific rule prevails | **PLAN** (Premium 1, E2L 0, Fundamental 3) |
| 14 | Phidias | Daily-loss wording in the EOD FAQ | "daily loss limit … account will fail" vs every grid "DLL None" | **PLAN** (grids: none); the FAQ is read as the EOD liquidation level |
| 15 | Lucid | LucidDirect 25K Initial Trail Balance | "None" vs $26,100 → $25,100 | **NC** |
| 16 | Apex | PA max position in micros | Only the ratio 10 micros = 1 mini is published | **PLAN**: show minis and the published ratio; user confirms micros on the account (max position is mandatory) |
| 17 | Phidias | E2L LIVE micros per size | Only the 25K example (20 micros) | **NC**; user types it |
| 18 | Phidias | LIVE (Elite/Master) drawdown, DLL, reset, max position | Behind login / contract | **SKIP** in V1 presets; user registers these accounts manually |

## 2. Copying and automation stances (shown in the account opt-in, decision 2026-10-23 option A)

| # | Firm | Item | Finding | Recommendation |
|---|---|---|---|---|
| 19 | Apex | Copy trading | Allowed across the same person's PAs (current page) vs Legacy PA Compliance page forbidding copy/mirroring software | **NC on the stance; show both texts.** The current page applies to current products; Legacy is item 25 |
| 20 | Apex | Automation | Forbidden; details point to a user agreement that is not public; webhooks not named | Stance = **forbidden** (opt-in warning with the quoted text); **ASK** only if the user wants certainty about webhooks |
| 21 | Phidias | Automation | "Fully automated" forbidden, "semi-automated" allowed with active monitoring; not defined; scope for evaluations unclear | Stance = **forbidden/conditional** → explicit opt-in; **ASK** whether webhook routing counts as semi-automated |
| 22 | Lucid | Copying across the same person's accounts | "Trade copiers are permitted", but no page addresses one person's own accounts; Terms ban "same strategy between unconnected accounts and third parties" | Stance = **conditional**, both texts shown; **ASK** |
| 23 | Lucid | Webhook automation | Automation allowed with conditions; webhooks not named | Stance = **allowed (conditional)**, text shown |
| 24 | All three | Copying to/from accounts at other firms | Not addressed (Lucid forbids cross-firm hedging only) | **INFO**: not a V1 feature (only same-user accounts); show "not addressed by the firm" |

## 3. Scope and low-priority items

| # | Firm | Item | Finding | Recommendation |
|---|---|---|---|---|
| 25 | Apex | Legacy products | "No longer available" vs "Legacy accounts are back for a limited time" | **SKIP** Legacy presets in V1; users with Legacy accounts register manually |
| 26 | Apex | Sizes 200K–300K | Not offered in current products (25K–150K only) | **INFO** — nothing to build |
| 27 | Phidias | Max E2L 150K accounts; 150K count rules | 1 vs 5 accounts; interaction of caps unclear | **INFO** (account-count limits are not enforced by the product); show the texts |
| 28 | Phidias | Consistency wording | "30% of total profit" vs "30% of total balance" | **NC** on the wording; threshold 30% in both |
| 29 | Phidias | Minimum withdrawal | $500 vs $1,000 | **SKIP** (payouts are not in the product) |
| 30 | Lucid | LucidPro funded DLL wording | "× 60% of highest EOD profit" vs "60% of Peak EOD Balance" | **NC** + ASK |
| 31 | Lucid | Days to payout, min trading days, HFT threshold | Single source or not published | **SKIP** (not used by the product) |

## 4. Product notes from R1 (for the slice plan, not user decisions)

- Apex's evaluation drawdown behaves differently on Rithmic/WealthCharts vs Tradovate → presets need a **platform dimension**, and the user picks the platform on the account (decision 2026-10-23; S16).
- **Apex rev. 2 (Lupa, 2026-10-23):** evaluations split by platform ([Rithmic, WealthCharts] stops trailing at the target; [Tradovate] always trails); platforms Tradovate, Rithmic, WealthCharts. C1 (copying) marks every Apex block `conflict`, but per the decision only the copy stance is "not confirmed"; the numeric fields are verified and the preset stays selectable (S16 AC-5).
- Apex also prohibits **non-directional brackets** (entry orders placed on both sides) and ATM use on Tradovate Groups. All Win's protective stop/target after an entry is directional, so it is **INFO** for now; to confirm with the firm in the ASK batch if the user wants certainty.
- Recommended **ASK** batch for the user (one message per firm): Phidias reset times + semi-automation, Lucid reset + LucidPro 25K DLL + same-person copying, Apex intraday trailing stop level.
