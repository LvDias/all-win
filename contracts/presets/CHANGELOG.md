# Preset release — changelog

## 1.1 — 2026-10-24 (additive minor; on top of 1.0.1)
- `drawdown_value.trailing_lock` (Q90 a, user; Esquadro, Torno, Malha) {`mode` `floor_reaches_start` | `never`, `offset` ≥ 0 only with `floor_reaches_start`}. Trailing types only (schema), the same as HTTP 5.4.0 `DrawdownValue.trailing_lock`. The publisher maps the research text without inventing: ambiguous → `not_confirmed`, silent → absent. `stops_trailing_at` stays the published text. The diff (check 9) sees `trailing_lock` as part of the value. The Core accepts `schema_version` 1.0 and 1.1.
- **Page titles (user decision 2026-10-24):** a firm page that blocks automated access (e.g. Apex, TPT) is cited with NO title and a short `title_note`, plus the required `url` and `accessed_at`; nothing is derived.
  - `quote.page_title` becomes optional; `title_note` (≤ 200, English, from the publisher) is required when it is absent and forbidden when it is present (Torno).
  - On a source, `title_note` is optional and allowed only without `page_title`.
  - `page_title`, when present, has minLength 1, so it is never an empty string (Guarita).
  - Old 1.0 files stay valid.
- Fixtures: valid `release-1-1-trailing-lock.json`, `release-1-1-quote-without-title.json`; invalid `trailing-lock-on-static.json`, `trailing-lock-never-with-offset.json`, `quote-without-title-or-note.json`, `quote-title-note-with-page-title.json`.

## 1.0.1 — 2026-10-23 (documentation and fixture only; no schema change)
- README check 9 states the diff semantics agreed with Torno: field comparison without `sources`; one entry per (preset, field, `firm_platform`); stance precedence; VPS and device text-only changes are `stance_text_changed` with `material: false`; a rule set appearing or disappearing counts as a change of all its fields; a change of the automation/copy platform split is refused in V1.
- Check 10 also covers `name`, `label`, `published_text` and `description`: every text that is displayed (Esquadro's low note).
- Fixture `valid/release-v2-stance-changed.json`: the v2 drawdown amount is 2500.00, so its `rule_value_changed` is real (Torno; the Core's check 9 refused the old fixture).

## 1.0 — 2026-10-23 (DRAFT, Compasso; Esquadro approves)
- `release.schema.json`: one firm release with firm, release (version, verification date, research file, changes), platforms, stances (versioned automation/copy rule texts, VPS, personal device) and plans/phases/sizes with per-platform rule sets. Rule fields follow HTTP `PresetRules`, including the commission per contract side and the evaluation target, with `verified` / `not_confirmed` provenance (`published_values`).
- Fixtures: 4 valid (new firm, not confirmed per platform, v2 stance change, manual only) and 11 invalid.
- Import checks the Core runs beyond the schema: README §"Core import checks".
- Code Review (Esmeril) fixes:
  - `rule_sets[].stored_only`: `min_trading_days` and `payout_rules`, stored and not exposed (S16 AC-10), evaluation and funded phases only;
  - `stance_info`: `not_confirmed` requires ≥ 2 quotes, and `research` only with `unknown`;
  - `personal_device` gains `not_confirmed` (S16 AC-5; HTTP follows in 5.4.0);
  - `rule_text.change` is the transition into its `version`, ignored when the version is unchanged;
  - README checks 3 (unique `size_amount` per phase) and 11 (at least one verified field per offered preset).
- Torno:
  - `platforms[].manual_only` (S-R3-2/4);
  - idempotent re-import keyed by the file's sha256 (Core behaviour);
  - the Core assigns rule version ids keyed by (firm slug, kind, firm_platform, version).
- Fixtures: 5 valid, 16 invalid.
- Owners to notify: Backend Rust (Torno, S16 import), the preset publisher (Lupa / PO), Database Architect (keys → ids).
