# S6 — Stop/target profiles HTTP contract notes

- **Status:** proposed (2026-10-23), for Code Review. Owner: Software Architect (Compasso). Contract version **2.3.0**, on top of 2.2.0 (S7).
- **Contract:** [`contracts/http/modules/profiles.yaml`](../../../contracts/http/modules/profiles.yaml), plus one reason code added to `modules/routing.yaml`.
- **Inputs:** slice-plan rev. 7, V1-S6 AC-1…AC-5 · design spec `v1-s6-stop-target-profiles.md` rev. 1 §9/§10 · ADR-0004 §5 (normalization to whole ticks, Q-N1) · [wave1-data-model.md](../data/wave1-data-model.md) §5.1, §5.4.

## 1. Operations

| Operation | Method and path | Notes |
|---|---|---|
| `listInstruments` | `GET /api/v1/instruments` | Global catalog: `tick_size`, `tick_value`, `point_value` as strings, `price_decimals` |
| `listDefaultProfiles` | `GET /api/v1/default-profiles?instrument_id=` | Always the 6 standard sizes; an undefined size has no `profile` |
| `putDefaultProfile` | `PUT /api/v1/default-profiles/{instrument_id}/{size_amount}` | Idempotent upsert |
| `listAccountProfiles` | `GET /api/v1/accounts/{account_id}/profiles` | One entry per enabled instrument |
| `putAccountProfile` | `PUT /api/v1/accounts/{account_id}/profiles/{instrument_id}` | `row_version` when a profile exists, absent when it does not |
| `restoreAccountProfile` | `POST /api/v1/accounts/{account_id}/profiles/{instrument_id}/restore` | Copies the current default (Q-S6-4) |
| `previewProfile` | `GET /api/v1/profile-preview?instrument_id=&distance_unit=&stop_distance=&target_distance=` | Stateless; no storage, no audit |

No step-up (Q35 list). No SSE: the tab refetches on focus (spec §9).

## 2. Decisions

1. **All arithmetic is the Core's.** The preview returns, per side, `ticks` (integer), `points` and `usd` (decimal strings) and `rounded`. The browser only inserts these values into the copy. The preview uses the same conversion as execution, so what the user sees is what is sent in `protection`.
2. **Off-tick values are rounded, not rejected.** Points and dollars that are not a whole number of ticks are rounded **toward the entry** (down to the tighter distance) and flagged `rounded: true`. This is ADR-0004 §5 and Q-N1, applied to stop and target. The stored profile keeps the value the user typed; the adjustment appears in each execution's audit entry (S6 AC-4). So the spec's `profiles.not_tick_multiple` / `profiles.not_whole_ticks` are not 422 codes:
   - fractional **ticks** are a schema error (400 `common.invalid_request` with the pointer);
   - off-tick points or dollars are accepted and rounded.
3. **Remaining 422 codes:**
   - `profiles.distance_below_one_tick` covers zero and anything below one tick after conversion. It replaces `profiles.not_positive`.
   - `profiles.distance_too_large` applies above 10 000 ticks (the structural cap of command quantities and distances).
   - `profiles.default_missing` applies on restore when the size has no default.
4. **Q-S6-6:** dollars accept at most **2 decimals** in the contract. The data model's 4 decimals stay as storage headroom.
5. **Q-S6-7 → Q60 (nearest size), option (c), decided** (3.0.0: `restoreAccountProfile` reuses `copied_from_size`, so the user's tie choice is kept; with no stored size and a tie it requires `profile_size`):
   - Distance is the absolute difference between the account size and each standard size. Below 25k the Core uses 25k; above 300k it uses 300k (PO premises).
   - `getNearestProfileSize` (`GET /api/v1/profile-sizes/nearest?size_amount=&instrument_id=`) returns 1 candidate, or 2 on an exact tie. Each candidate has `has_default`.
   - On a tie the user picks one, and `createAccount.profile_size` carries the choice. The Core never chooses.
   - If the user decides (a) "smaller" or (b) "larger", the list has one entry and nothing else changes.
   - The copy is reported as `origin: nearest_size_default` with `copied_from_size`.
6. **Account without a profile (Q43):** an account created while its size had no default has no profile (`AccountProfileList` entry without `profile`). Its entries are skipped and audited with the new routing reason **`routing.no_bracket_profile`** (`blocks: entries_only`; exits continue, P1). The code is added to `RouteReasonCode`, so the S7 status cell shows it too. The user fixes it with `putAccountProfile` (no `row_version`, `origin: set_by_user`).
7. **Provenance** (`origin`, `copied_from_size`, `copied_at`, `customized`, `default_now`) covers the S5 detail need "profile provenance", which was deferred from the S5 contract.
8. The **shared/individual mode** is not here: it is a per-account setting of S8 AC-5. S6 shows `Individual` read-only.

## 3. Audit

- `profiles.default_changed`: instrument, size, and previous and new unit/stop/target, as strings.
- `profiles.account_changed` with `action` ∈ `initial_copy` / `edited` / `restored`: the same fields plus `copied_from_size`.
- `routing.no_bracket_profile` (skip; Q43).

Rendering belongs to S10.

## 4. Data-model deltas (for the Database Architect, through the Maestro)

- `accounts.account_bracket_profile`: add `row_version integer NOT NULL DEFAULT 1`, `origin text CHECK IN ('size_default','nearest_size_default','set_by_user')`, `copied_at timestamptz` and `customized boolean NOT NULL DEFAULT false`. `copied_from_size` already exists.
- Rules:
  - `set_by_user` has no `copied_from_size` / `copied_at`, and `customized` is false: `CHECK (origin <> 'set_by_user' OR NOT customized)`.
  - A copy has both columns.
  - A restore sets `copied_at = now()` and `customized = false`.

  Applied in data model rev. 8 (Lastro, 5dd2d45); the `set_by_user` `CHECK` in rev. 9.
- `distance_unit = 'usd'`: the contract limits it to 2 decimals; the `CHECK` at 4 can stay.
