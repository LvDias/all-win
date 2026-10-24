# Preset release v1

Machine-readable release of one prop firm's presets (V1-S16). The publisher builds it from a research file that the PO has reviewed (`docs/product/presets/research/<firm>.md`, format §5). The Core imports it as the firm's next release.

- **Schema:** [`release.schema.json`](release.schema.json) (JSON Schema 2020-12). Fixtures are in `examples/valid` and `examples/invalid`, checked by `contracts/scripts/validate_fixtures.py`.
- **Owner:** Software Architect (Compasso). Esquadro approves changes.
- **Status:** draft (2026-10-23).

## Shape

- **`firm`:** `slug`, `name`, official `domains`, and optionally `manual_only` (S16 AC-7).
- **`release`:** `version_number`, which is the previous release + 1; `verified_on`; the `research_file` used as evidence; and `changes`. A change is the HTTP `ReleaseChange` with keys instead of ids: `preset` is `{plan, phase, size}`.
- **`platforms`:** the firm-side platforms (HTTP `FirmPlatform` codes). A platform may carry its own `manual_only` (S-R3-2/4): accounts on that platform are manual only. Two more cases need no field of their own: a firm-level `manual_only` (S-R3-1 b), and excluding a whole firm, which means publishing no file for it.
- **`stances`:**
  - `automation` and `copy` are opt-in rule texts. There is one text per kind, or one per firm platform when the stance differs by platform. Each has a publisher `version`, which increases on every stance or text change. A `version` > 1 requires `change.cause`, and a `text_changed` cause also requires a `reason` (Q26, Q42).
  - `vps` and `personal_device` come with verbatim quotes, or with `research` when the stance is `unknown`. A `not_confirmed` value needs at least 2 conflicting quotes; `personal_device` also allows `not_confirmed` (S16 AC-5).
  - `change` describes the transition into this `version`. The Core reads it only when the version is new, and ignores it when the version equals the stored one.
- **`plans` → `phases` → `sizes` → `rule_sets[]`:** the rules are the HTTP `PresetRules` fields. Each field is `verified` (`value` + `sources`) or `not_confirmed` (≥ 2 `published_values`, each with its sources, and no `value`; S16 AC-5). Commission per contract and side and the evaluation target are rule fields.
- **`rule_sets[].stored_only`** (evaluation and funded phases only): `min_trading_days` (with provenance) and `payout_rules` (verbatim quotes). The Core stores them with the preset; no HTTP operation exposes them in V1 (S16 AC-10, Q46).
- **`excluded_plans`:** plans that are not offered as presets (S16 AC-9).

Money is always a string, exactly as published. A file never carries database ids: the Core maps keys to ids and assigns the rule version ids.

## Core import checks (not expressible in the schema)

The import is all or nothing. Any failure rejects the whole file, and nothing is published.

1. `firm.slug` matches the stored firm. A new slug is a new firm with `version_number` 1.
2. `version_number` is exactly the stored one + 1.
3. These are unique: plan keys; phases within a plan; and, within a phase, size keys and `size_amount` (one size per amount, so the Q73 nearest-size choice is never ambiguous).
4. Every `firm_platform` used is in `platforms`.
5. `rule_sets` either has one entry with no `firm_platform`, or one entry per platform, each with `firm_platform`. The same rule holds for `stances.automation` and `stances.copy`.
6. Every quote and source URL is on one of `firm.domains`, or one of their subdomains.
7. `evaluation_profit_target` appears only in `evaluation` phases.
8. For each (kind, platform), a rule text's `version` is the stored one when the stance and text are unchanged, and the stored one + 1 when either changed. It never goes back.
9. `changes` equal the Core's own diff against the previous release:
   - stance and text changes;
   - every `rule_value_changed`;
   - every preset added or removed;
   - `material` ⇔ `change.cause = text_changed`.
10. Texts are plain text and contain no token-shaped part (`aw[kd]_…`).
11. Every offered preset (size × rule set) has at least one `verified` rule field. A firm with no verified data is not offered (S16 AC-5), so its file is refused.

## Idempotent publishing

The Core stores the sha256 of each imported file's canonical JSON with the release.
- Importing the same file again for an already-published `version_number` is a successful no-op.
- Other content with the same `version_number` is an error, and nothing changes.

This needs a release column (Lastro), not a schema change.

## Versioning

`schema_version` is `1.x`. A minor version adds optional fields only, and a Core that knows `1.n` refuses a file with a newer minor. A breaking change is `v2`.
