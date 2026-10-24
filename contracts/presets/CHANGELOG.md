# Preset release — changelog

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
