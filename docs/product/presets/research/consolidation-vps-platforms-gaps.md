# Consolidation — automation, copy, VPS, device, platforms, gaps (R1 + R2 + R3)

| | |
|---|---|
| **Prepared for** | Rumo (Product Owner), via the Maestro |
| **Prepared by** | Lupa (Prop Firm Rules Researcher) |
| **Date** | 2026-10-23 |
| **Basis** | The YAML blocks of the 18 files in this folder (aggregated by script) plus each file's gaps table. No new web research was done for this file. The per-firm files hold the quotes and URLs; this file does not repeat them. |
| **State** | Proposed. The recommendations in §4 are Lupa's proposals, not decisions. |
| **Format caveat** | The rev. 3 format doc is not in this worktree. All files use the R2 format plus the rev. 3 extras. |

Reading notes: `no` / `yes` below are the YAML values (PyYAML reads bare `no`/`yes` as booleans; the files mean the strings). "unk" = `unknown`. Counts are blocks, excluded blocks left out. Every file is `partial`; none is `verified`.

## 1. One row per firm: stances

| Firm (file) | Copy | Automation | VPS | Personal device | Platforms offered (as published) | NT8 plugin |
|---|---|---|---|---|---|---|
| Alpha Futures | conditional (scope conflict) | no (22) / unk (4) | conditional | unk | AlphaTrader, DeepChart, Quantower, TradingView, WealthCharts | No NinjaTrader listed: unreachable per blocks |
| Apex | unk (conflict) | no | conditional (VPN/proxy banned only to disguise identity/device/location) | unk | Rithmic, Tradovate, WealthCharts | NinjaTrader not named in blocks: confirm in file |
| BluSky | **conflict** | unk | unk | unk | Rithmic apps, Tradovate/NT/TV, Volumetrica, Tradesea, Tickblaze, BlackArrow | Partial: Rithmic and Tradovate paths only |
| Bulenox | conditional | conditional (conflict: commercial tools) | unk | unk | Rithmic apps incl. NinjaTrader | Reachable |
| Earn2Trade | no | conditional (one clause) | unk | unk | Rithmic, BlackArrow, NT via Tradovate, DeepCharts | Partial: not BlackArrow; Rithmic path needs own NT licence |
| Elite Trader Funding | conditional (approved copiers) | conditional (written authorisation) | no (conflict with copier text) | unk | Tradovate, NinjaTrader, TradingView, Rithmic apps, Tickblaze | Reachable, but policy blocks VPS and bots |
| Funded Futures Family | conditional (own accounts; ambiguity vs automation ban) | **no, all plans** | conflict | unk | Tradovate, WealthCharts, NinjaTrader, TradingView; Pro = Rithmic | Reachable, policy blocks automated entry |
| Leeloo | unk (cross-user banned) | unk | unk | unk | Rithmic apps incl. NT7/8 | Reachable; rules behind login |
| Lucid | conditional | conditional (HFT/microscalping banned) | unk | unk | NinjaTrader, Tradovate, TradingView, Quantower, Tradesea, others | Reachable |
| MyFundedFutures | conditional | conditional | unk | unk | NinjaTrader, Tradovate, TradingView, Quantower, Volumetrica, others | Reachable |
| OneUp | conditional | unk | unk | unk | Rithmic only; NT 8.1 preferred | Reachable on desktop; not NT Web/Mobile |
| Phidias | conditional | conditional (semi-auto allowed if monitored) | unk | unk | Rithmic, NinjaTrader/Tradovate, DeepCharts | Reachable |
| Phoenix | yes (own accounts, Thor) | no on eval/pre-funded (17), conditional on live (9) | yes ("Connections are monitored.") | unk | Tradovate, Rithmic EdgeClear, dxFeed apps, Odin, Quantower | Partial: TFeed via Tradovate or Thor only; not dxFeed |
| Take Profit Trader | conditional (10 blocks) | no | unk | unk | NinjaTrader, Tradovate, Rithmic, CQG, TradingView, others | Reachable |
| Top One Futures | conditional | no | conditional | unk | Tradovate/NinjaTrader, BlackArrow, WealthCharts | Via Tradovate only |
| Topstep | conditional (12) / no (3) | conditional (12) / no (3) | no | **yes** | TopstepX, ProjectX API, Quantower | No NinjaTrader listed: unreachable per blocks |
| TradeDay | conditional | conditional | no | unk | Rithmic, Tradovate | NinjaTrader not named in blocks: confirm in file |
| Tradeify | conditional | conditional | conditional | unk | NinjaTrader (Tradovate), Quantower, Rithmic apps, Tradesea, TradingView, WealthCharts | Reachable via Tradovate |

Personal-device rule: only Topstep publishes one. The other 17 files carry `unknown`, so All Win cannot tell the user whether orders from a server-side relay are acceptable. The NT8 plugin runs on the user's own PC, which satisfies a personal-device rule by construction.

## 2. Gaps and conflicts per firm

Each table lists only what changes a preset or an opt-in warning. Full lists: §9–§11 of each file.

### Alpha Futures
| Field | Gap or conflict | Kind |
|---|---|---|
| copy trading scope | help center vs Terms and Conditions | conflict |
| max accounts | 3 vs 5, and 5 total vs 5 per tier | conflict |
| daily loss limit, drawdown type, min days | some sizes: help vs T&C | conflict |
| automation, VPS (4 blocks) | unknown | gap |
| personal device | not mentioned | gap |

### Apex
| Field | Gap or conflict | Kind |
|---|---|---|
| copy trading | official pages conflict (24/24 blocks) | conflict |
| consistency threshold | "<50%" vs example "50% met" | conflict |
| trailing stop point (8 blocks), scaling level 4 range (2) | unpublished | gap |
| personal device | unknown | gap |
| automation details | the "User Agreement" is not public | gap |

### BluSky
| Field | Gap or conflict | Kind |
|---|---|---|
| copy trading | "Trade Copier Allowed: Yes" vs "Account Mirroring" forbidden | conflict |
| NinjaTrader on Rithmic | phase scope differs between articles | conflict |
| Propel 150K/200K consistency | 30% vs 34% | conflict |
| Propel 300K daily loss limit | $2,500 vs none | conflict |
| Sim Funded contract limits, Propel start balances, Orbit 200K contracts and Buffer phase | articles disagree | conflict |
| automation, VPS, personal device | no official text found | gap |
| trading-day reset | only "6PM", no timezone | gap |
| Tradovate commission table | basis not stated | gap |

### Bulenox
| Field | Gap or conflict | Kind |
|---|---|---|
| automation, third-party tools | FAQ "only user-built tools" vs Terms "must be reviewed and approved" | conflict |
| Momentum and Momentum Master rules | stage tables unlabeled or unpublished | gap |
| Qualification consistency and lock rules | not published | gap |
| VPS, personal device | not mentioned | gap |

### Earn2Trade
| Field | Gap or conflict | Kind |
|---|---|---|
| funded daily loss limit | behaviour not published; `kind: unknown` is outside the format set | gap, **PO decision** |
| MNQ and NQ commission | MNQ not published (images); `null` in 25/25 blocks | gap |
| automation | one clause on AI / ultra-fast data entry, reading unconfirmed | gap |
| VPS, personal device | none published | gap |

### Elite Trader Funding
| Field | Gap or conflict | Kind |
|---|---|---|
| news rule, funded | help: no restriction vs ToS: flat 5 min before/after | conflict |
| VPS wording | ToS bans vs copier text "legitimate VPS" | conflict |
| 1-Step 250K | listed vs "no longer sold" | conflict |
| Static 25K floor | $24,100 vs $24,000 | conflict |
| Direct to Funded target label, DTF 25K existence | tables disagree | conflict |
| 150K drawdown, target, daily loss limit | unpublished | gap |
| trading-day reset, personal device | unpublished | gap |

### Funded Futures Family
| Field | Gap or conflict | Kind |
|---|---|---|
| VPS | "permitted" vs "not allowed" | conflict |
| Standard S2F consistency | 25% vs 45% | conflict |
| Velocity funded consistency | 40% vs none | conflict |
| Pro funded drawdown | EOP vs EOD | conflict |
| Premier+/S2F 25K max position | funded table vs scaling plan | conflict |
| commission, trading-day reset, personal device | not published | gap |
| copy vs automation ban | copy of own accounts allowed, automated entry banned: ambiguous | conflict |

### Leeloo
| Field | Gap or conflict | Kind |
|---|---|---|
| everything in the knowledge base | behind login: reset, instruments, commission, automation, VPS, device, trailing lock | gap |
| own-account copy | not stated (cross-user banned) | gap |

### Lucid
| Field | Gap or conflict | Kind |
|---|---|---|
| daily loss limit (9), consistency (8), drawdown (1) | official pages disagree | conflict |
| trading-day reset, copy scope | no explicit statement | gap |
| VPS, personal device | help-center search returned 0 results | gap |
| webhooks | not found | gap |

### MyFundedFutures
| Field | Gap or conflict | Kind |
|---|---|---|
| daily loss limit (16 blocks), drawdown type (3), min days (5), max position (4), max accounts | articles disagree | conflict |
| trading-day reset, same-person copy sentence | not explicit | gap |
| VPS, personal device | not found | gap |

### OneUp
| Field | Gap or conflict | Kind |
|---|---|---|
| balance resets | "unlimited" vs $50 / $100 | conflict |
| Express min days | 3 vs 5 (old note) | conflict |
| automation, VPS, personal device | no rule published anywhere found | gap |
| funded scaling and commission tables | images only, not quotable | gap |

### Phidias
| Field | Gap or conflict | Kind |
|---|---|---|
| per-plan platform matrix | not published | gap |
| trading-day reset (20 blocks) | unpublished | gap |
| VPS, personal device | unknown | gap |
| Premium blocks | `verified` for rev-2 fields; the two device fields are not confirmed | note |

### Phoenix
| Field | Gap or conflict | Kind |
|---|---|---|
| Merit tiers | marketing page $2K only vs help center $10K/50K/100K | conflict |
| Merit ES commission | 1.99 vs 2.18 / 1.09 | conflict |
| bot ban on Live | ambiguous | conflict |
| trading-day reset and timezone, personal device | not published | gap |
| live drawdown amounts and starting size | not published | gap |

### Take Profit Trader
| Field | Gap or conflict | Kind |
|---|---|---|
| PRO+ max position and platform list | not published | gap |
| PRO+ Development trailing stop point, daily loss limit pause end | not published | gap |
| timezone label of the trading day | not stated (20 blocks) | gap |
| VPS, personal device | unknown | gap |

### Top One Futures
| Field | Gap or conflict | Kind |
|---|---|---|
| copying to/from other firms; same-IP rule vs copying | pages disagree (38 blocks) | conflict |
| max accounts and size mixing | pages disagree | conflict |
| 150K daily loss limit | two values | conflict |
| Ignite flatten time | two values | conflict |
| per-plan platform list, personal device | not published | gap |

### Topstep
| Field | Gap or conflict | Kind |
|---|---|---|
| time a day locks for consistency and payout counting | pages disagree | conflict |
| LFA micro limit, scaling band boundaries | unpublished or inconsistent | gap |
| copying to/from other firms, webhooks | not stated | gap |
| daily loss limit kind (5 blocks) | unclear | gap |
| platforms per plan (6 blocks) | partly unpublished | gap |

### TradeDay
| Field | Gap or conflict | Kind |
|---|---|---|
| Fast Pass min days (12 blocks) | pages disagree | conflict |
| Funded Live drawdown and position limits | pages disagree | conflict |
| slippage buffer basis | pages disagree | conflict |
| personal device | unknown | gap |

### Tradeify
| Field | Gap or conflict | Kind |
|---|---|---|
| Select 25K evaluation max contracts | gap | gap |
| 300K V2 daily loss limit kind | gap | gap |
| Elite Live micro limits and platforms | gap | gap |
| copying to/from other firms, personal device | not stated | gap |

## 3. NinjaTrader 8 plugin: what it cannot reach, and policy blocks

**Technically unreachable (the firm's connection is not one NT8 supports):**
| Firm | Unreachable |
|---|---|
| Phoenix | dxFeed-based platforms; NT8 only via TFeed through the Tradovate connection or the Thor addon (orders hit Sim101) |
| BluSky | Volumetrica, Tradesea, Tickblaze, BlackArrow |
| Earn2Trade | BlackArrow; Rithmic without an own NinjaTrader licence (new subscriptions must use the NT-Tradovate option; legacy NT-Rithmic ended 2026-06-30) |
| OneUp | NinjaTrader Web and Mobile |
| Topstep | TopstepX / ProjectX only in the blocks (no NinjaTrader listed) |
| Alpha Futures | AlphaTrader, WealthCharts, Quantower, DeepChart, TradingView only in the blocks |

**Reachable but blocked by firm policy:**
| Firm | Policy | Source in |
|---|---|---|
| Funded Futures Family | automated order entry forbidden on all plans | funded-futures-family.md §6 |
| Phoenix | bots forbidden on evaluation and pre-funded | phoenix-trader-funding.md §6 |
| Elite Trader Funding | bots only with written authorisation; VPN/VPS/proxy prohibited; approved copiers only | elite-trader-funding.md §6–§7 |
| Earn2Trade | copy trading not allowed | earn2trade.md §5 |
| Topstep | VPS not allowed; personal device required | topstep.md §7 |
| TradeDay | VPS not allowed | tradeday.md §7 |

**No rule published (ask the firm in writing):** OneUp, BluSky, Leeloo, Earn2Trade (automation); 17 of 18 firms (personal device).

## 4. Decision list for the user (Lupa's proposals, not approved)

| # | Decision | Options | Recommendation |
|---|---|---|---|
| D1 | Firms with an automation ban (FFF all plans; Phoenix eval/pre-funded; Topstep 3 blocks; Apex, Alpha, Take Profit Trader, Top One `no`) | block All Win routing to those accounts / warn and let the user proceed / warn only | Warn with the verbatim `automation_rule_text` and require an explicit opt-in per account. Default off. Never route silently. |
| D2 | Elite Trader Funding: bots need written authorisation, VPS banned | treat as `conditional` / treat as banned | Treat as `conditional`, default off. The opt-in must tell the user to get the firm's written authorisation first. The plugin runs on the user's PC, so the VPS ban is not triggered by the plugin. |
| D3 | `unknown` automation (OneUp, BluSky, Leeloo, Earn2Trade's single clause) | allow with warning / block | Allow with a generic "firm rule not published" warning and a suggestion to ask the firm. Do not state a rule the firm has not published. |
| D4 | VPS-hosting of any All Win component | allow / forbid where `no` or `conflict` / forbid everywhere | Keep the user-PC plugin as the only order path. Show the VPS stance as informational. For `no`/`conflict` firms (Topstep, TradeDay, Elite, FFF) say that the user's PC must not be a VPS. |
| D5 | Personal-device rule unknown for 17 firms | assume allowed / assume required | Assume nothing; show "not published". The NT8 plugin on the user's own PC is the lowest-risk design under either reading. |
| D6 | Platforms NT8 cannot reach | hide the preset / show with a "not supported by the NT8 plugin" flag | Show, flagged. The preset data stays useful for limits and risk even when the platform is not routable. |
| D7 | Conflicting fields (`status: conflict`) | use the stricter value / use none | Use the stricter published value in the preset only if the user opts in, and show "unconfirmed". Otherwise leave the field empty. |
| D8 | Earn2Trade funded daily loss limit `kind: unknown` | add `unknown` to the format set / map to `none` | Add `unknown` to the format set. Mapping to `none` would invent a rule. |
| D9 | Preset commission floor (U20) for firms without a published figure (FFF, Leeloo, Earn2Trade MNQ) | block preset / allow user-entered | Allow a user-entered value, marked "not published by the firm". |
| D10 | Publish the rev. 3 format doc | PO action | Needed so the extras (platforms, automation text, device fields, commission) are formalised and `status` values are reviewed. |

## 5. Changelog

- 2026-10-23 — initial consolidation (Lupa).
