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
- **`plans` → `phases` → `sizes` → `rule_sets[]`:** the rules are the HTTP `PresetRules` fields. Each field is `verified` (`value` + `sources`) or `not_confirmed` (no `value`; S16 AC-5). From 1.2 a `not_confirmed` field says why in `not_confirmed_reason` (Torno, Rumo's R3 recheck): `conflicting` (absent means this; ≥ 2 `published_values`, each with its sources), `partial` (the firm published only part of the value: the verbatim text in `quotes`, plus `research` for where the rest was searched; never a structured or derived value, e.g. "6PM" stays text with no time zone added, and a daily loss amount with no hard/soft kind stays a quote) or `not_published` (nothing found, e.g. a knowledge base behind a login: `research` only). The Core handles the three alike: never prefilled; a required field needs the user's own value. Commission per contract and side and the evaluation target are rule fields.
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
9. `changes` equal the Core's own diff against the previous release. Diff semantics (1.0.1, with Torno):
   - **`rule_value_changed`** compares a field without `sources`. A new access date or page is not a value change, but `status`, `value`, `published_text` and the values in `published_values` count; those values are also compared without their `sources`. From 1.2, `not_confirmed_reason` (absent = `conflicting`) and the `text` of the `quotes` of a `partial` field also count; `research` does not.
   - **Platforms.** With per-platform `rule_sets` there is one entry per (preset, field, `firm_platform`), with `firm_platform` set. Without platforms the entry has no `firm_platform`. A rule set that appears or disappears for a platform in an existing preset makes every field of it a `rule_value_changed` with that platform.
   - **Automation and copy texts.** A stance change gives `stance_changed`, and only that entry when the text also changed. A text-only change gives `stance_text_changed` with `material` ⇔ `change.cause = text_changed`.
   - **`vps` / `personal_device`.** A value change gives `stance_changed`. A change of quotes or research alone gives `stance_text_changed` with `material: false`; these stances have no `change.cause` and never suspend anything.
   - **Presets.** Every preset added or removed gives `preset_added` / `preset_removed`.
   - **Split change.** A change in how `automation` / `copy` texts split by platform between releases (one text ↔ per-platform texts) is **refused** in V1 (fail-closed). It would need its own suspension rule.
10. Texts are plain text and contain no token-shaped part (`aw[kd]_…`). This covers every displayed text: quotes, `name`, `label`, `published_text`, `description` and localized texts.
11. Every offered preset (size × rule set) has at least one `verified` rule field. A firm with no verified data is not offered (S16 AC-5), so its file is refused.
12. A file uses only the fields of the minor it declares (1.1.1, Esmeril). A `schema_version` "1.0" file is refused when it has any 1.1 field: `trailing_lock` in a drawdown value, `title_note` on a quote or a source, or a quote without `page_title`. A "1.0" or "1.1" file is refused when a rule field has `not_confirmed_reason`, `quotes` or `research` (1.2). The schema cannot see `schema_version` from those nested objects, so the Core checks it. A Core test covers each of the three cases.

## Idempotent publishing

The Core stores the sha256 of each imported file's canonical JSON with the release.
- Importing the same file again for an already-published `version_number` is a successful no-op.
- Other content with the same `version_number` is an error, and nothing changes.

This needs a release column (Lastro), not a schema change.

## Versioning

`schema_version` is `1.x`. A minor version adds optional fields only, and a Core that knows `1.n` refuses a file with a newer minor. A file is also refused when it uses a field of a minor newer than the one it declares (check 12). A breaking change is `v2`.
