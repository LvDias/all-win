# S16 — Prop firm presets HTTP contract notes

- **Status:** proposed (2026-10-23), for Code Review. Owner: Software Architect (Compasso). Contract version **2.5.0**, on top of 2.4.0.
- **Contract:** [`contracts/http/modules/presets.yaml`](../../../contracts/http/modules/presets.yaml) (extended; the S5 rule versions are unchanged), plus `rule_values` in `createAccount` / `updateAccount` ([`modules/accounts.yaml`](../../../contracts/http/modules/accounts.yaml)).
- **Inputs:** slice-plan rev. 7, V1-S16 AC-1…AC-9 · design spec `v1-s16-presets.md` rev. 1 §9/§10 · `docs/product/presets/preset-research-format.md` rev. 2 (field list) · [S5 notes](s5-accounts.md) (rule versions, Q40–Q42).

## 1. Operations

| Operation | Method and path | Notes |
|---|---|---|
| `listFirms` | `GET /api/v1/firms` | Catalog list: stances per kind, `manual_only` with the preset's reason, `not_confirmed_count`, release |
| `getFirm` | `GET /api/v1/firms/{firm_id}` | Stance blocks with quotes; firm platforms; plans → phases → sizes (each with its current `preset_version_id`); excluded plans |
| `listFirmReleases` | `GET /api/v1/firms/{firm_id}/releases` | Version history with structured changes, the material flag and reason |
| `getPresetVersion` | `GET /api/v1/preset-versions/{preset_version_id}` | The rules preview: every rule value with its provenance, per firm platform when the rules differ |
| `getAccountPreset` | `GET /api/v1/accounts/{account_id}/preset` | The account's version, per-field origin, newer version |
| `getAccountPresetDiff` | `GET /api/v1/accounts/{account_id}/preset-diff?target_preset_version_id=` | Comparison dialog |
| `applyPresetVersion` | `POST /api/v1/accounts/{account_id}/preset-version` | Per-field choices; `opt_ins` + step-up when the rule version changes |
| `restorePresetValue` | `POST /api/v1/accounts/{account_id}/preset-values/{field}/restore` | `row_version` |
| (S5) `createAccount` / `updateAccount` | — | New `rule_values` (prop with a preset only) |

SSE: `presets.version_available` (`firm_id`, `account_ids`).

## 2. Decisions

1. **Versioning unit = the firm release.** Each firm is researched as a whole (one research file per firm) and published together. A release creates a new version of each of the firm's presets and new opt-in rule versions where the stance or its text changed. Releases and versions are immutable. `PresetVersion.superseded_by` points to the newer one.
2. **Rule fields** (from the research format §4, limited to S16 AC-1):
   - `max_position_qty`, `drawdown`, `daily_loss_limit`, `daily_profit_target`, `trading_day_reset`, `consistency_rule`;
   - the first four are required to save (S16 AC-6);
   - evaluation profit target, minimum trading days and payout rules are not in the contract until the user decides Q46.
3. **Provenance per value:** `verified` → `value` + `sources`; `not_confirmed` → at least two `published_values`, each with its sources, and never a `value` (S16 AC-5/AC-6: never prefilled).
4. **The account's values** travel as `rule_values` (only where they differ from the preset or resolve a not-confirmed field). They are sent on create and on an update that changes the preset selection; with an unchanged selection `updateAccount` refuses `rule_values` (422 `accounts.rule_values_without_selection_change`), so saving the account form never resets risk edits. Later edits go through `putAccountRiskRules` (S13). `max_position_qty` stays the account's top-level field (S5).
   - `getAccountPreset` reports each field's `origin` ∈ `preset` / `edited` / `chosen_from_conflict` / `unresolved`.
   - **Q-S16-3:** a resolved value is stored with `chose` ∈ `published_value` / `other_value` and `chosen_at` (F§7.4 popover "Escolhido por você entre valores divergentes em {date}").
5. **Applying a version (S16 AC-3).** `applyPresetVersion` is the only way to move to a newer version of the same preset; `updateAccount` refuses it with 422 `presets.use_apply_version`. `updateAccount` with another preset is a re-selection that re-fills the values. Every `requires_choice` row needs one of:
   - `choices.<field>` (`keep_mine` / `use_preset`; nothing preselected, Q-S16-4);
   - `rule_values.<field>` (a not-confirmed field of the target).

   If the new cap is below the account's maximum, the body must lower `max_position_qty`. The S5 opt-in rules apply when the opt-in rule version changes: `opt_ins`, step-up `account_opt_in_accept`.
6. **Stances (Q-S16-2):**
   - automation and copy use the `RuleVersion` of the opt-ins (one source of truth);
   - `vps` ∈ `allowed` / `forbidden` / `conditional` / `unknown` / `not_confirmed`;
   - `personal_device` ∈ `required` / `not_required` / `unknown`. The research format's `yes`/`no` are mapped to these names so the wire never has a bare "yes".
7. **Manual-only (S16 AC-7, Q-S16-6):** `manual_only.reason` is a `RuleQuote` (verbatim, source, date). The frame around it is UI text.
8. **Material change (Q42, provisional):** `ReleaseChange` `stance_text_changed` carries `material` (and a bilingual `reason` when material). This matches `RuleVersion.change` (S5).
9. **Suspension before applying (Q-S5-3, PO yes):** `AccountPresetState.newer_version.suspends` tells the UI the opt-in is already `pending_review`.
10. **3.0.0 review fixes (Crivo, 2.5.0):**
   - P2: `applyPresetVersion` takes `firm_platform`; the diff has `firm_platform_required` / `firm_platforms`.
   - P3: `RuleVersion.firm_platform`. `FirmStances.automation` / `copy` are lists, one per platform when the stance varies. `FirmSummary` shows `varies_by_platform`. `FirmRelease.rule_versions` are labelled `{id, kind, firm_platform}`, and each release carries its VPS and device stances.
   - P4: the manual-only reason is a `RuleQuote` everywhere (routing candidates too).
   - P5: Q41 is modelled only as the reserved `pending_reason: preset_attached`. A user who attaches a preset through `updateAccount` sends the new choice inline (D16), and nothing attaches presets automatically.
   - Lows:
     - verified rules need at least one source;
     - `ReleaseChange` rules are enforced by the schema;
     - a field is in `choices` or `rule_values`, never both;
     - `chose` only with `chosen_from_conflict`;
     - `Account.preset_update_available` is added;
     - a not-confirmed maximum position is resolved by `max_position_qty` (cap = the highest published value);
     - apply audits `presets.version_applied` with both version numbers, plus `accounts.opt_in_chosen` for inline choices;
     - `FieldChange` serializes object values as compact JSON.
11. **Q1 — publishing path:** releases are published outside the HTTP contract, by the catalog seed/CLI that Backend owns, run by the admin from the reviewed research files.
   - The tool writes the release, its `changes` (with the Q42 material flag and the bilingual reason) and the rule versions.
   - In the same transaction it moves the affected `chosen` opt-ins to `pending_review` and writes one `accounts.opt_in_suspended` audit entry per account. SSE `presets.version_available` follows.
   - No HTTP publish or edit operation exists.

## 3. Error codes

| Code | Status | Pointer | Operations |
|---|---|---|---|
| `presets.version_not_newer` | 422 | `/target_preset_version_id` (`/query/…` on the diff) | diff, apply |
| `presets.version_superseded` | 409 | — | apply |
| `presets.choice_required` | 422 | `/choices/<field>` | apply |
| `presets.required_field_unresolved` | 422 | `/rule_values/<field>` or `/max_position_qty` | create, update, apply |
| `presets.use_apply_version` | 422 | `/preset_version_id` | update |
| `accounts.rule_values_without_selection_change` | 422 | `/rule_values` | update |
| `presets.value_not_confirmed` | 422 | — | restore |
| `accounts.max_position_exceeds_preset` | 422 | `/max_position_qty` | create, update, apply |
| `common.version_conflict` | 409 | — | apply, restore |

## 4. Audit (category `config`)

- `presets.version_applied`: old and new version, every changed field with old and new values, choices.
- `presets.value_restored`: field, old and new value.
- `accounts.created` / `.updated` carry `rule_values` changes as `FieldChange`s.
- The rendering and the `details` schemas go into `modules/audit.yaml` (`KnownAuditCode`) in this same version.

## 5. For the Database Architect and Backend (through the Maestro)

- Catalog tables: `catalog.prop_firm`, `catalog.firm_release`, `catalog.firm_plan`, `catalog.preset` (firm, plan, phase, size), `catalog.preset_version` (preset, release, rule sets per firm platform as typed columns or `jsonb` with provenance), `catalog.stance` (vps / personal device per release). `catalog.opt_in_rule_version` already exists (rev. 8) and gets `release_id`.
- The account's rule-field origins (`edited` / `chosen_from_conflict` + `chose` + `chosen_at`) need a home: `accounts.account_rule_value` (account, field, value, origin, chose, chosen_at).
- **Backend design point (not a contract matter):** the rule values feed `risk.account_limits` (S13). Create, update and apply must save account and limits atomically. Either the accounts service writes both in one transaction through a risk port, or risk derives its limits from the `account.*` event with an "awaiting limits" state. ADR-0001 allows only events and query ports between modules, so the second fits the rules and needs a gate (no routing before limits exist). To be decided by the Software Architects with Backend.
