# S17 — TradingView alerts: analysis (draft, not committed)

- **Owner:** Plugin Engineer (Vela) · **Status:** proposed · **Date:** 2026-10-23
- **Inputs:** ADR-0003 §3, §6, §7; `contracts/instruction/v1` (schema + fixtures); `docs/architecture/contracts/s1-webhook-ingest.md` §2; Core code `core/adapters/tradingview/src/policy.rs`, `core/crates/routing/infra/src/ingest_store.rs`; slice plan V1-S17; TradingView Pine alerts docs.
- **Rule:** the Pine adapts to contract 1.0. No contract change is proposed.

## 1. Signal type mapping

| Indicator signal | `signal_type` | Contract value exists? |
|---|---|---|
| Gatilhos / triggers (Setups 1, 2, 3, 4, 6) | `trigger` | yes |
| Trade Plan activation | `trade_plan` | yes |
| Triggers do NQ | `nq_trigger` | yes |

No gap. `master_account` is refused from TradingView (422 `ingress.signal_type_not_allowed_for_source`).

## 2. Actions from TradingView in V1

| Action | Required fields beyond the envelope | Forbidden fields | Core result |
|---|---|---|---|
| `buy` / `sell` | `quantity` (integer 1–1000) | `side`, `changes`, `position_effect: reduce`, non-market `order` | open/add; opposite position → `routing.position_conflict` |
| `close` | — | `quantity` (ignored, audited `quantity_ignored`; do not send), `side`, `order`, `position_effect`, `changes` | flattens the whole position + cancels stop/target |
| `reverse` | `quantity`, `side` (`long`/`short` = final side) | `position_effect`, `changes` | flatten then open `side`; flat → opens; already `side` → `routing.position_conflict` |
| `modify`, `cancel` | — | — | **422** `ingress.action_not_allowed_for_source` |
| `buy`/`sell` + `position_effect: reduce` | — | — | **422** `ingress.action_not_allowed_for_source` |
| `order.type` ≠ `market` | — | — | **422** `ingress.order_type_not_allowed_for_source` |

Envelope (always): `schema_version: "1.0"` (the Core supports only minor 0 → `1.1` = 400 `ingress.unsupported_version`), `message_id`, `sent_at`, `source.type: "tradingview"`, `signal_type`, `action`, `instrument.symbol`.

Other ingress rules the Pine must respect:
- **No stop/target** fields (they come from the account profile; unknown fields → 400 `ingress.schema_violation`).
- **Decimals are strings** (`reference_price`), quantities are JSON integers.
- `instrument.symbol` is the **traded** future root, not the chart ticker (`QQQ`). V1 trades **MNQ only** (vision §4, PO 2026-10-23); other symbols → 422 `ingress.unsupported_instrument`.
- **Do not send `instrument.contract_month`** (PO 2026-10-23): the Core takes the active contract from the admin rollover catalog and switches it on the configured date (P5, S20 AC-4); a month typed in the Pine goes stale at the roll.
- **Credential guard:** any string anywhere in the body shaped like a token (`awk_…`/`awd_…`) → 400 with an empty pointer. The secret goes only in the webhook URL (A3, X-4).
- Body ≤ 16 KiB; TradingView sends `application/json` when the message parses, otherwise `text/plain` — both accepted.
- `sent_at` must be the **emission instant** (`timenow`), never the bar time: the Core's expiry base is `min(sent_at, received_at)`, so an old bar time would make entries expire (`routing.expired`).
- `reference_price` is informational. The indicator reads QQQ/SPY; a QQQ price in an MNQ instruction would corrupt slippage analysis → **omitted** (PO agreed 2026-10-23).

## 3. message_id uniqueness (S1 idempotency)

**Core semantics (verified in code):** `INSERT … ON CONFLICT (user_id, message_id) DO NOTHING`. Scope is the **user** (not the source/token), forever. A repeated id gets 200 `duplicate` with the first `instruction_id`; **the body is not compared** — a different instruction that reuses an id is dropped (first wins). Allowed: `[A-Za-z0-9._:-]`, 1–128 chars.

So the id must be (1) identical when the *same event* re-fires (protection against double execution), and (2) different for any *distinct* signal, across all charts/alerts of the user, forever.

| Ingredient | Assessment |
|---|---|
| `timenow` (ms) | Unique-ish but **not idempotent**: a re-fire of the same event (tick re-evaluation, alert restarted on TradingView's servers, two alert instances on the same chart) gets a new id → **double execution**. Reject as the key. Use it only for `sent_at`. |
| `time` (bar open, ms) | Deterministic per bar, survives recalculation/reload. Alone it collides between signals on the same bar → combine. |
| Ticker / timeframe | Two charts (QQQ 5 min and QQQ 1 min, or QQQ and SPY both mapped to MNQ) can fire the same setup on the same bar time → must be in the id, or the second is dropped as "duplicate". `!` (e.g. `NQ1!`) is not allowed → strip. |
| Signal type, setup, action/side | Distinguish distinct signals on the same bar (a `close` and a `reverse` on the same bar must not collide). |
| `var` counter | Rolled back on every realtime tick and reset when the script recalculates (alert recreated, server restart) → not unique across reloads. Reject. |
| `varip` counter | Persists across ticks but resets on reload → collides with ids already sent earlier that day. Reject as a key; fine as a **local send guard**. |
| Repainting | With `alert.freq_once_per_bar` / `freq_all`, a condition can be true mid-bar and false at close: the signal was still sent and executed. The id does not fix this; only bar-close evaluation does (latency cost). User decision (Q-S17-a). |
| Re-alerts after recreating the alert (daily routine) | With the deterministic id, an event already sent today has the same id after recreation → Core answers `duplicate` (safe). If yesterday's alert was not deleted, both fire the same event → second is `duplicate` (safe), but signals that differ because of the old levels still execute → deleting yesterday's alert remains mandatory. |

**Recommendation (deterministic event key):**

```
tv-<sourceName>-<instrument>-<signal_type>-<setup>-<action[_side]>-<chartTicker>-<timeframe>-<barTimeMs>
e.g. tv-nq-es-strikes-MNQ-trigger-s6-buy-QQQ-5-1792679400000      (≈60 chars, limit 128)
```

Extends the ADR-0003 §7 guidance (`tv-<symbol>-<kind>-<setup>-<bar time ms>`) with source name (ids are unique per user, across sources), action, chart ticker and timeframe; no contract change. Source name (1–24) and setup (1–16) are validated, never stripped, so two distinct values can never collapse into one id (review Esmeril LOW-2/3).

Plus, in Pine: `alert()` with `freq_all` guarded by a `varip` set of ids already sent in this script run, so each event is posted once per run (no per-tick spam against the per-key limit of 60/min and no audit noise), while the Core dedupes across reloads and duplicate alert instances. TradingView's `freq_once_per_bar` is tracked per `alert()` call site; a shared helper would have one call site and could suppress a *second distinct* signal on the same bar — the explicit guard avoids depending on that.

**Accepted trade-off:** two identical signals (same setup, same direction) on the same bar of the same chart collapse into one. Under "repeated entries add to the position" that is a business choice → Q-S17-b.

## 4. How the user validates an alert (no port opened)

The Pine function writes every message to **Pine Logs** (`log.info`), so the user can copy the exact JSON without waiting for a live webhook.

| Option | How | Fidelity | Cost / risk |
|---|---|---|---|
| **(a) Paste into the Maestro session → local dev stack** (approved default, development phase only; the user's product path is the audit log, S10) | User pastes the JSON (never the URL). The team posts it to `https://127.0.0.1:8443/ingress/v1/tradingview/<dev seed token>` (dev stack, ports bound to 127.0.0.1) and returns HTTP + `accepted`/`duplicate`/`rejected` + code + pointer | **Exact** (real Core: schema, credential guard, policy, catalog, dedupe) | Needs the dev stack up (DevOps; docker access) and a dev seed source key. The instruction is routed in dev (simulation only). Re-sending the same JSON returns `duplicate`, which also proves the idempotency |
| **(b) Offline CLI `validate-alert`** | `python3 integrations/tradingview/tools/validate_alert.py alert.json` — contract schema + static ingress rules (version 1.0, credential guard, TV policy, symbol list) | High for structure; **can drift** from the Core (re-implements policy; catalog is a static list); no dedupe | Cheap, offline, the user can run it locally. Mark as pre-check, Core is authoritative |
| (b′) Core dry-run binary | `allwin-core check-instruction <file>` reusing the Core's parse + policy (no DB insert) | Exact minus catalog/dedupe | Needs Backend Rust (Ferro) work, not in S17 scope |
| **(c) Real TradingView → All Win** | Alert with the webhook URL pointing to a reachable All Win host | End-to-end (S17 AC-3) | **Needs a public HTTPS endpoint (port 443) or a tunnel → explicit user authorization.** Not proposed by me |

**Recommendation:** (a) for S17 now (exact, zero new code, nothing exposed), (b) as a later convenience, (c) only when the user authorizes a public endpoint (needed anyway for S17 AC-3 with real TradingView).

## 5. Open questions (for the user, via Maestro)

- **Q-S17-a** Evaluate signals at **bar close** (`barstate.isconfirmed`, no repainting, ≤1 bar latency) or **intrabar** (faster, may send signals that vanish at close)? What does the current indicator do?
- **Q-S17-b** Two identical signals (same setup and direction) on the same bar of the same chart: one signal (recommended) or two entries?
- **Q-S17-c** For each signal, which action does the indicator mean: opposite signal while positioned = `reverse`, `close`, or a plain `buy`/`sell` (which the Core rejects with `position_conflict` when opposite)? Is there an exit signal (→ `close`)? Business rule — the team does not decide it.
- **Q-S17-d** Base quantity per signal type (one input per type, or one global)?
- ~~Q-S17-e~~ decided by PO: omit `reference_price`.
- ~~Q-S17-f~~ already decided: V1 trades MNQ only (vision §4).
- **Q-S17-g** Pine source: to integrate the function into the indicator (and keep it in `integrations/tradingview/`), the user must hand over the source (Q7). Until then the function is paste-in.
- **Q-S17-b** PO recommends one signal (consistent with "never execute twice"); user to confirm.
- **Same `message_id`, different body** — Architect decision (Esquadro, 2026-10-23), implementation pending: still 200 and nothing executed, but the Core writes the audit entry `ingress.duplicate_conflict` (user-visible warning, grouped by source; details: source name, `message_id`, both `received_at`, names of the differing fields, never the raw body). Comparison uses a sha256 of the canonical parsed JSON. Owners: Lastro (column in `routing.message_receipt`), Compasso (audit/notification code), Ferro (Core). With the deterministic Pine id, such a conflict means a broken template, which is what the user needs to see. No change to the Pine or the guides.
  - **`sent_at` is excluded from the compared hash** (Esquadro, 2026-10-23): a legitimate re-fire of the same event (alert restarted, two alert instances) carries the same id with a new `sent_at` (`timenow`) and stays a plain `duplicate` with no warning. Every other field is compared; expiry keeps the first receipt's `sent_at`/`received_at`. Consequence for the Pine: `details` values must be deterministic for the event (no `timenow`, live price or tick counter), otherwise a re-fire shows as a conflict.
