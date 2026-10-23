# ADR-0006 — Derived state across modules: versioned events and a staleness gate

- **Status:** Accepted (2026-10-23), Architect decision within ADR-0001 §4. First use: account rule values → risk limits (S16 → S13).
- **Owner:** Software Architect
- **Asked by:** Compasso (HTTP contract 2.5.0, S16 §5) and Lastro (data model rev. 10 §5.7).
- **Depends on:** [ADR-0001](0001-technology-stack.md) §4 (modules talk only through events and query ports), ADR-0003 §4.1 (account gates, P1), outbox dispatcher rules (data model §6.4, decision A).

## 1. Context

The `accounts` module owns an account and every input of its limits: the maximum position, the preset version and firm platform, and the account's own rule values. The `risk` module owns `risk.account_limits`, which it derives from those values and enforces (S13), and which it sends to the plugin (`limits`, ADR-0004 §7). An account create, update or preset apply must never let routing act on limits that do not match the account.

## 2. Options

| Option | How | Trade-off |
|---|---|---|
| (A) Command port | `accounts` calls a `risk` command port inside its own DB transaction | Always consistent, but it is a cross-module **command** dependency, which ADR-0001 §4 excludes. It also couples the two modules' schemas in one transaction |
| **(B) Versioned event + staleness gate** | `accounts` bumps `account.limits_version` and writes `account.limits_changed` to `app.outbox` in the same transaction. `risk` consumes it and upserts `risk.account_limits` with `derived_from_version`, idempotently (only a newer version wins). Routing reads both versions through query ports and gates entries while they differ | One writer per schema; crash-safe through the outbox; staleness is a checkable fact, not a timing assumption |

## 3. Decision

**(B).** Rules:

1. **Producer:** **any write that changes an input of the derived limits** increments `account.limits_version`, sets `limits_changed_at`, and emits `account.limits_changed { account_id, limits_version }` in the **same transaction** as the change. Inputs include at least:
   - `max_position_qty`;
   - the account's own rule values (`updateAccount.rule_values`, a conflict resolution `chosen_from_conflict`, `restorePresetValue`);
   - the preset version or firm platform (create, firm/plan/phase/size/platform change, preset apply, a new rule version adopted).
2. **Consumer:** `risk` reads the inputs **together with their `limits_version` in one consistent read** from the `accounts` query port, recomputes the limits, and upserts them with `derived_from_version` = **that read's version**, never the event's (only a newer one wins). An event whose version is not newer than the stored one is ignored, and a redelivery is harmless. Dispatcher rule (A) applies: never marked published without being consumed, and `risk` starts `from_now` with a startup reconciliation. The `accounts` query port `list_limits_versions() → [(account_id, limits_version)]` is compared with risk's own `derived_from_version`, and every account that differs is derived through the same path as the consumer. No cross-schema SQL.
3. **Gate (routing):** before routing a **position-increasing** action, routing compares `account.limits_version` (accounts query port) with `derived_from_version` (risk query port). If no limits row exists, or the row is older, the action is skipped with **`routing.limits_pending`**, audited, with no alert. **Exits continue** (ADR-0003 P1: refusing an exit never reduces exposure).
   - An action that cannot be clearly classified as entry or exit counts as **position-increasing** (fail closed).
   - The versions are read once per instruction, at the decision. The two reads are separate query-port reads, not atomic. That is acceptable: an instruction routed just before an account change commits is ordered before that change.
   - **The gate ships together with the `risk` consumer (S13), never before.** Otherwise "no limits row" would block every entry.
4. **Plugin barrier:** `risk` sends the new `limits` to the adapter only after the upsert. Until then the plugin keeps enforcing the previous limits. That is safe, because the Core gate already stops new entries.
5. **Monitoring:** a periodic worker job (every 30 s) runs the same comparison as the reconciliation and raises an operational alert when a gap is older than 60 s (measured from `limits_changed_at`). It **only alerts** and never derives outside the consumer path. It is never a silent state.
7. **Saved-but-not-yet-effective values (S13 Q48, provisional; Crivo A1):**
   - **Input (accounts):** a save of a limit value writes only the user's value and its own `saved_at`, per kind. The **latest save of a kind wins** and supersedes every earlier value of that kind, pending or not. accounts does not compute trading days or effective dates.
   - **Derivation (risk):** from the consistent read (§3.2) plus the limits in force, risk decides per kind:
     - stricter than in force → effective now;
     - looser → scheduled for the start of the trading day after its `saved_at`. "Looser" follows the contract's definition (s13-risk.md, QA Security L1): removing a rule (absent = no limit), `profit_target_on_reach` `flatten_and_block` → `notify`, and `daily_loss` `hard` → `soft_pause`/`none` are all loosening. A change of one kind that is **not comparable** (for example a drawdown type change, static 2000 → trailing_eod 2500, or a type change with a smaller amount) is treated as **looser**: today's limit stays in force and the new one applies from the next trading day (fail closed; test). A change of a prop account's **`trading_day_reset`** (Q-S13-2) is always scheduled, never applied at once: it takes effect from the start of the trading day after its `saved_at`, computed with the reset in force. Moving the reset mid-day would re-cut the current day and clear its block early (an early unlock).
     It writes the effective values to `risk.account_limits` and the scheduled ones to the derived `risk.scheduled_limit`. Only the derivation writes either table, never an API use case. Both can be rebuilt from the accounts read.
     - Because the latest save wins at the input, a looser (pending) value followed by a stricter one leaves only the stricter one. After the reset the stricter value is still in force (test).
   - **Reset step:** at each trading-day start, one risk step clears the previous day's blocks, then applies the due scheduled values and re-derives. It sets `effective_as_of` = that trading-day start and sends `limits` to the plugin afterwards. A looser value never clears a block before the reset ("no early unlock", vision §9). The reset bumps no `limits_version`, because no input changed.
   - **Monitoring:** besides §3.5, an alert fires when `effective_as_of` is older than the current trading-day start for more than 60 s (a stuck reset). Fail-closed meanwhile: the previous limits, stricter or equal, stay in force.
   - **Pending saves also bump `limits_version`:** the gate then briefly skips entries (`routing.limits_pending`) even though the effective limits did not change. This is intended. It keeps `pending_changes` visible in the UI and the plugin, so it must not be optimized away.
   - **Ports:** only risk → accounts (query port for the inputs). The trait is defined in the consuming module's application crate (dependency inversion) and wired in `crates/app`, so the crates form no compile-time cycle.
   - **HTTP:** `/accounts/{id}/risk-rules` is grouped under the `risk` HTTP module for clients. In the Core its handler calls the accounts use case that writes the inputs, and `AccountRisk.pending_changes` is read from `risk.scheduled_limit`.
6. **General rule:** any future state that one module derives from another's data follows the same pattern (versioned event, idempotent upsert with `derived_from_*`, a staleness gate at the point of action). A command port across modules needs its own ADR.

## 4. Consequences

- **Positive:** the module boundaries of ADR-0001 hold; no distributed transaction; staleness can be tested and audited; crash-safe.
- **Negative:** a short window, normally milliseconds, in which entries for a just-changed account are skipped as `routing.limits_pending`. This is visible in the audit, and an instruction arriving in that window is not executed (never execute late).
- **Tests:**
  - an entry routed between the change and the derivation is skipped with `routing.limits_pending`, while an exit in the same window executes;
  - a redelivered or out-of-order event never downgrades the limits;
  - after a crash between the commit and consumption, startup reconciliation closes the gap;
  - the plugin receives `limits` only after the upsert;
  - an update that commits during a derivation never leaves `derived_from_version == limits_version` with stale inputs.
- **Owners:** Database Architect (columns `limits_version`, `derived_from_version`; data model §5.7), Backend (producer, consumer, gate, reconciliation), Software Architect (ADR-0003 code `routing.limits_pending` in the next ADR-0003 delta; HTTP audit code via Compasso).
