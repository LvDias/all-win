# S13 — Risk rules and live risk state HTTP contract notes

- **Status:** proposed (2026-10-23), for Code Review. Owner: Software Architect (Compasso). Contract version **3.1.0**, on top of 3.0.0 (wave-2 review fixes).
- **Contract:** [`contracts/http/modules/risk.yaml`](../../../contracts/http/modules/risk.yaml), plus the `risk.*` audit codes in `modules/audit.yaml`.
- **Inputs:** slice-plan rev. 7, V1-S13 AC-1…AC-6 (and S14 AC-1 for the gate text) · design spec `v1-s13-risk.md` rev. 1 §9/§10 · [S16 notes](s16-presets.md) (rule value types, provenance) · [wave1-data-model.md](../data/wave1-data-model.md) §5.6.

## 1. Operations

| Operation | Method and path | Notes |
|---|---|---|
| `getAccountRisk` | `GET /api/v1/accounts/{account_id}/risk` | Trading day, meters (used / limit / truncated percent / state), unset and untracked rules, block, staleness, pending changes |
| `listRiskSummaries` | `GET /api/v1/risk-summaries` | Worst state per account for the list column (one call) |
| `getAccountRiskRules` | `GET /api/v1/accounts/{account_id}/risk-rules` | Rules + enforcement flag (read-only) + preset provenance + limits `version` |
| `putAccountRiskRules` | `PUT /api/v1/accounts/{account_id}/risk-rules` | Full replacement; `version`; `acknowledge_reached` |
| `previewAccountRiskRules` | `POST /api/v1/accounts/{account_id}/risk-rules/preview` | Validate only: effect per change and `would_be_reached` |
| (S16) `restorePresetValue` | — | For edited fields |

SSE: `risk.state_changed` (state, stale, block, trading-day reset, rules changed; not on every tick). The client refetches.

## 2. Decisions

1. **The Core computes everything.**
   - `used`, `limit`, `floor` are decimal strings;
   - `percent` is an integer, truncated (F§6.2), so "80 %" never appears while the state is `normal`;
   - `state` ∈ `normal` / `warning` / `reached` / `target_reached` (F§7.1, D18: no 90 % band);
   - consistency never goes beyond `warning` (S13 AC-6).
2. **Rule value types are the S16 ones** (`DrawdownValue`, `DailyLossValue`, `DailyProfitTargetValue`, `TradingDayResetValue`), so preset and account values never drift apart.
   - A drawdown of type `other` cannot be tracked and is listed in `untracked_rules`, as is an unresolved not-confirmed value.
   - Absent rules are `unset_rules` (UI "sem limite definido").
3. **Concurrency token = the account's limits version** (`account.limits_version`, ADR-0006). Each save bumps it; the plugin receives the derived limits after the risk module catches up (§3).
4. **Q48 :**
   - a stricter value applies at once, a looser one from the next trading day (`pending_changes`). Looser (3.2.0, QA Security L1) also covers removing a rule, `flatten_and_block` → `notify`, and `daily_loss` `hard` → `soft_pause` / `none`;
   - the preview reports `effective` / `effective_from` per change and `would_be_reached`;
   - the save needs `acknowledge_reached: true` when it would already be reached, else 409 `risk.reached_not_acknowledged`. This guards against a race between the dialog and the save.
5. **Q47 :** prop accounts need a daily loss limit unless the preset says `none` (422 `risk.daily_loss_required`). Real accounts may leave every rule unset.
6. **Q49 / Q63 :** `flatten_and_block` needs enforcement on (422 `risk.flatten_requires_enforcement`). If enforcement is turned off later, the Core treats it as `notify` (S15 owns the switch).
7. **Q-S13-2 (PO):** prop accounts may edit their copy of the reset time. Real accounts may not (422 `risk.reset_not_editable`; they use the CME session close, `trading_day.source = cme_session_close`).
8. **Q-S13-5:** for trailing drawdowns, `floor` is the account balance at which the drawdown is breached, as it stands now.
9. **Q-S13-7:** the consistency threshold has at most one decimal, 0–100 (`^(100(\.0)?|[0-9]{1,2}(\.[0-9])?)$`, fixed in 3.2.0).
10. **Playback (S15 AC-7):** `trading_day.playback_clock` carries the simulated clock the day and P&L follow.
11. **Manual trades (PO rule: S13 AC-2 + S21 AC-3, PO clarification, slice-plan rev. 7; QA Security I1, 3.4.0):** trades entered by hand (S21, origin `manual`) do NOT feed daily loss, drawdown, the profit target, consistency or blocks, and never unblock an account; they feed only the trade list and the metrics (S21/S22). Risk derives only from what the platform reports.
12. **Staleness:** `stale` + `stale_reason` (`connection_down` / `no_recent_data`). The UI never shows stale values as current.
13. **No step-up in V1** (Q35 list). If the wider list is approved later, the body gains `step_up` (`x-step-up` on the PUT) with no other change.

## 3. Interplay with S16 and ADR-0006

- The rule values belong to the account (`accounts` module; `accounts.account_rule_value`, data model §5.7). `putAccountRiskRules` writes them, bumps `account.limits_version` and emits `account.limits_changed`, all in one transaction (outbox).
- The risk module derives `risk.account_limits` from the accounts query port, idempotently, with `derived_from_version` (ADR-0006, option B). Only then does it send `limits` to the plugin.
- While the derivation lags:
  - `AccountRisk.limits_pending` is true, and the meters show the previous limits;
  - routing skips new entries with `routing.limits_pending` (exits continue, P1).
- `updateAccount` no longer resets rule values (3.0.0: `rule_values` only with a changed preset selection).

## 4. Error codes

| Code | Status | Pointer |
|---|---|---|
| `common.version_conflict` | 409 | — |
| `risk.reached_not_acknowledged` | 409 | — |
| `risk.required_value_unresolved` | 422 | `/rules/<field>` |
| `risk.daily_loss_required` | 422 | `/rules/daily_loss_limit` |
| `risk.flatten_requires_enforcement` | 422 | `/rules/profit_target_on_reach` |
| `risk.reset_not_editable` | 422 | `/rules/trading_day_reset` |
| `risk.consistency_not_applicable` | 422 | `/rules/consistency_threshold_pct` |
| `risk.amount_invalid` | 422 | the field |

`risk.drawdown_incomplete` from the spec is not needed: type and amount are one object (`DrawdownValue`), so a half-filled drawdown is a schema error (400 with the pointer).

## 5. Audit codes (added to `KnownAuditCode`)

- `risk.rules_changed` (category `config`): changes, limits version, the effect of each change.
- `risk.warning`, `risk.limit_reached`, `risk.target_reached` (+ `action`), `risk.consistency_warning` (category `risk`): kind, limit, used, percent.
- S15 adds `risk.flatten` and `risk.blocked`.

## 6. Data ownership (ADR-0006 §3.7, data model rev. 13)

- **Inputs (accounts):** the user's value of each rule, plus its own `saved_at` per kind. The latest save of a kind wins, even over an earlier pending value. This includes `profit_target_on_reach` and `consistency_threshold_pct`. Every save bumps `account.limits_version`, a pending one included, so a rules form opened earlier gets 409 (A2).
- **Derived (risk):** the effective limits (stricter → now) and `risk.scheduled_limit` (looser → the trading day after `saved_at`). Only the derivation writes them. `AccountRisk.pending_changes` reads `risk.scheduled_limit`.
- **Reset step:** clear blocks → apply due schedules → re-derive (`effective_as_of`), with no version bump; `limits` go to the plugin afterwards. A looser value never clears a block early ("no early unlock").
- **Other tables (Lastro):**
  - `risk.trading_day` per account and day;
  - `risk.drawdown_tracker` (high water, floor);
  - `risk.account_block` (cause, until; one active);
  - `warning_threshold_pct` is dropped (80 % is fixed, D18).

## 7. 4.0.0 (written as 3.5.0) — net basis and the evaluation target

- **Net (S13 AC-2 rev. 8; firms measure on net):** every meter is net. `AccountRisk` adds `pnl_basis: net`, `commission_source` and `commissions_today`. `commission_source: none` (platform 0 and no floor) shows the notice "comissão não informada; os limites podem estar folgados demais". It is not a block. The account floor `commission_per_contract_side` (U20 a) is an input in accounts; changing it bumps the limits version (ADR-0006).
- **S13b — evaluation target (Q46; Q69 a, Q70 a):**
  - `RiskRules.evaluation_profit_target` is prefilled from the preset (`PresetRules.evaluation_profit_target`, evaluation phase only) and editable.
  - The account has `evaluation_start` (default: the registration date) and an optional `evaluation_earned_before`. They change only through `POST /api/v1/accounts/{id}/evaluation/restart`, audited as `accounts.evaluation_restarted`. A Sim / Playback reset never restarts the count.
  - `AccountRisk.evaluation` carries `start`, `profit_target`, `earned_before`, `achieved` (earned_before + the Core's net sum of the account's fills since `start`; manual trades never count), `remaining`, `percent` (truncated), `state` (`in_progress` / `target_reached`), `reached_at` and `as_of`.
  - Reaching the target raises one notification, `risk.evaluation_target_reached` (`success`), and does nothing automatic. Minimum trading days and payout are not in V1.
