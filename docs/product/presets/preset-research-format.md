# Prop Firm Preset Research — Format

> **Owner:** Product Owner (Bússola @ Dev2) · **Applies to:** V1-R1, R2, R3 ([slice-plan.md](../release-1/slice-plan.md)) · **Date:** 2026-10-23 · **Rev.** 2 (platforms, automation classification, "not confirmed")
> Research files live in `docs/product/presets/research/`, owned and written only by the researcher ("Lupa"). The PO reviews each file before any preset is built from it.
> Canonical rules: [product-vision.md §9, §12](../product-vision.md) — presets are never invented; each firm's rules are verified on official pages; anything unverifiable is reported to the user, who decides to delay that preset or release without it.

## 1. One file per firm

`docs/product/presets/research/<firm-slug>.md` (e.g. `apex-trader-funding.md`, `phidias-propfirm.md`, `lucid-trading.md`).

## 2. Sources

- **Official only:** the firm's own website, rules/help center, FAQ, and terms. Third-party summaries, reviews, affiliate sites, and social posts may point to where to look but are never the source of a value.
- Every value cites the **exact URL** and the **access date**. If a page is dynamic, also save a short verbatim quote of the sentence that states the value.
- If two official pages disagree, record both values with both sources and mark the field `conflict`. Do not pick one. A conflicting field ships in the preset as **"not confirmed"** and does not block the rest of the preset; the PO collects conflicts for the user (decision 2026-10-23).
- No sign-up, login, payment, or form submission to reach rules. If a rule is only visible after login, mark it `unverified (behind login)`.

## 3. File structure

1. **Header:** firm name, official domains, research date, researcher, file status (`verified` = every required field verified · `partial` · `unverified`), and the instruments covered (at least MNQ/NQ).
2. **Platforms:** the trading platforms/data connections the firm offers for each plan (e.g., Rithmic, Tradovate, NinjaTrader license) — stored in the preset (decision 2026-10-23).
3. **Plans:** one section per plan (program) and phase (evaluation, funded/PA, live). For each **account size** offered, a table with the required fields (§4). Note sizes outside 25k–300k if the firm has them; record them, but they are low priority.
4. **Trading-day reset:** time and **time zone as published** (e.g., "5:00 PM ET"), plus any holiday/early-close rule.
5. **Trade copying stance:** verbatim quote + URL — specifically whether copying across the **same person's** accounts is allowed, any account-count limits, and any rule about copying from or to accounts at other firms.
6. **Automated trading stance:** verbatim quote + URL — bots/algos/automated order entry, and whether trade copiers or webhook automation are named. Classify as `allowed | forbidden | conditional | unknown`; the verbatim text is shown to the user in the opt-in warning for firms that forbid automation (decision 2026-10-23, option A).
7. **Device / hosting rules:** whether the firm allows trading from a VPS/server/VPN, and whether orders must originate from the trader's personal device — verbatim quote + URL (YAML `vps_allowed: yes|no|conditional|unknown`, `personal_device_required: yes|no|unknown`).
7b. **Other rules that affect routing or risk:** e.g., news trading restrictions, mandatory stop loss, scaling plans (contracts allowed vs balance), max accounts per person, overnight/weekend holding, minimum/maximum trade duration, inactivity rules.
8. **Machine-readable block:** one YAML block per plan/phase/size (§5).
9. **Unverified / gaps / conflicts:** a list for the user report — field, what is missing, where it was searched.
10. **Changelog:** dated entries for each change to the file.

## 4. Required fields per plan / phase / size

| Field | Notes |
|---|---|
| Account size | As published (e.g. "50K"); also the nominal amount in USD |
| Drawdown amount | Exact USD as published |
| Drawdown type | One of: `trailing_intraday`, `trailing_eod`, `static`, `other` (describe). Include when/if it stops trailing (e.g., locks at starting balance + X) |
| Daily loss limit | Exact USD, or `none` if the firm states there is none; also whether it is a hard breach or a soft "pause for the day" |
| Profit target | Exact USD (evaluation phases); `none` where not applicable |
| Consistency rule | Exact wording and threshold (e.g., "no day > 30% of total profit"), and the phase it applies to |
| Max position size | Maximum contracts in **minis and micros** as published (state the mini:micro ratio if given); scaling rules if size depends on balance |
| Minimum trading days | Number and the phase it applies to |
| Payout / activation rules | Only if they change what the trader may do (e.g., buffer that changes drawdown); fees and prices are **not** needed |
| Source | URL + access date for each value (a footnote per row is fine) |

Money values are written exactly as published, as strings, with no rounding (e.g. `"2,500.00"` → `"2500.00"` in YAML). Never derive a value by arithmetic unless the firm publishes the formula; if you do apply the firm's formula, show it.

## 5. YAML block (one per plan/phase/size)

```yaml
firm: apex-trader-funding
plan: "<plan name as published>"
phase: evaluation        # evaluation | funded | live
size_label: "50K"
size_usd: "50000.00"
instruments: [MNQ, NQ]
platforms: [Rithmic, Tradovate]   # as published for this plan
drawdown: { amount_usd: "0.00", type: trailing_intraday, stops_trailing_at: "<text or null>" }
daily_loss_limit: { amount_usd: null, kind: none }   # kind: hard | soft_pause | none
profit_target_usd: null
consistency_rule: { text: null, threshold_pct: null }
max_position: { minis: null, micros: null, scaling: null }
min_trading_days: null
trading_day_reset: { time: "17:00", tz_as_published: "ET" }
copy_trading_allowed: unknown   # yes | no | conditional | unknown
automated_trading_allowed: unknown   # yes | no | conditional | unknown
automation_rule_text: null          # verbatim, shown in the opt-in warning
vps_allowed: unknown                # yes | no | conditional | unknown
personal_device_required: unknown   # yes | no | unknown
status: unverified       # verified | partial | unverified | conflict (per field: not_confirmed)
not_confirmed_fields: []
sources:
  - { field: drawdown, url: "https://...", accessed: "2026-10-23" }
```

`null` means not found; `unknown` is allowed only together with an entry in the gaps list. The YAML is input for the preset seed (V1-S16); the Markdown text and sources are the evidence.

## 6. Done criteria for one firm (maps to V1-R AC-1…AC-5)

- Every required field is `verified` with URL + date, or listed in gaps.
- Copy-trading and automation stances are quoted verbatim.
- The researcher sends the PO a short summary: file path, status, number of plans/sizes, and gaps. The PO reviews and, if there are gaps, reports to the user through the Maestro.
- Re-verification is repeated shortly before release (R AC-5).
