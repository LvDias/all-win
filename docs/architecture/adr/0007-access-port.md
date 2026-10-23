# ADR-0007 — Access port: authenticated caller and step-up across modules

- **Status:** Accepted (2026-10-23, user approved). Architect decision within ADR-0001 §4; the one cross-module command port that ADR-0006 §3.6 requires an ADR for.
- **Owner:** Software Architect. Crate owner: Backend (Brasa, S3).
- **Asked by:** Brasa (S3), for Malha (S5), Torno (S6) and Ferro (S7/S13).
- **Depends on:** [ADR-0001](0001-technology-stack.md) §4 and §9, [ADR-0006](0006-derived-state-across-modules.md) §3.6, [auth-and-session-policy.md](../security/auth-and-session-policy.md) rev. 3 §5–§6, HTTP contract 2.0.0+ (`StepUpProof`, `StepUpAction`).

## 1. Context

Sessions and step-up codes belong to the `identity` module (S3). Every other module's HTTP handlers need the authenticated caller. `accounts` (opt-in accept, binding change) and the routing module (route enable) also need a step-up check. Modules must not depend on each other (ADR-0001 §4).

Policy §6.1 said the Core verifies the code "and performs the action in the same transaction". Across modules that would mean a shared transaction, which couples two schemas. ADR-0006 rejected that kind of coupling.

A second issue is the step-up target. For `account_opt_in_accept`, `route_enable` and `account_binding_change`, `requestStepUp` must answer 404 without sending a code when the target account does not exist or belongs to someone else. Only `accounts` knows that.

## 2. Options

| Topic | Options | Decision |
|---|---|---|
| Where the port lives | (i) traits in `identity`, imported by every module; (ii) a neutral crate | **(ii)**. With (i), every module would depend on `identity` |
| Code consumption | (a) consumed atomically inside `authorize`, in identity's own commit, before the action runs; (b) `authorize` validates, the action runs, then `grant.consume()`; (c) a shared transaction passed across modules | **(a)**. (b) leaves a replay window: two parallel requests with one code would both run the action. (c) couples schemas |
| Target ownership | a query port answered by `accounts` | **Yes**, with dependency inversion (§3.4) |

## 3. Decision

1. **Crate `core/crates/access`:** a workspace member that depends only on `shared-kernel`, with no `sqlx`. It contains:
   - `Caller { user_id, session_id, device_id, is_platform_admin }`. This is the minimum other modules need; identity-only fields stay in identity.
   - `trait SessionAuthenticator`. It resolves the session cookie, and the idle/absolute expiry and activity touch of policy §5.1, into a `Caller`.
   - `StepUpAction` (mirrors the contract enum) and `StepUpProof { challenge_id, code }`.
   - `trait StepUpVerifier`, implemented by `identity`: `is_required(action)` (the Q35 setting) and `verify_and_consume(caller, action, target, proof) -> Result<(), AccessError>`. The second one also writes the audit.
   - A concrete `struct StepUpGate` that wraps a `StepUpVerifier` and offers `authorize(caller, action, target, proof) -> Result<StepUpGrant, AccessError>`. It is the only code that can create a `StepUpGrant` (§3.3).
   - `AccessError` = `Unauthenticated` | `StepUpRequired` | `CodeInvalid` | `Unavailable`.
   - A generic HTTP extractor behind the cargo feature `axum`. It applies the session check and the strict `Origin` check with the contract precedence (401 → 403 `common.forbidden_origin`).

   `identity` implements `SessionAuthenticator` and `StepUpVerifier`; `crates/app` builds the `StepUpGate` around the verifier and injects both.
2. **`authorize` consumes the code before the action (a).**
   - It checks that the challenge is a `step_up` challenge bound to the same `session_id`, `action` and `target`. It then consumes the challenge with a conditional update (`consumed_at IS NULL`), in identity's own transaction, and only one concurrent request can win.
   - It applies the setting `step_up.q35_persistence_actions`: for an action that is switched off, it returns a grant without a proof.
   - It writes `security.step_up_succeeded` / `security.step_up_failed` (action and target) to the user's audit log.
   - **Policy §6.1 now reads:** the code is consumed atomically before the action is performed. When the action belongs to `identity` (sources, connections, invitations, channel change), consumption and action share one transaction.
   - **Consequence:** an action that fails after the step-up (409/422 or a crash) has spent the code, and the user requests a new one. A code never authorizes twice, and never authorizes anything other than its bound action and target. The action audits its own outcome, including the refusal.
3. **`StepUpGrant`** is minted only by `StepUpGate::authorize`.
   - Its fields are private, and its constructor is private to the `access` crate. The verifier only answers `Ok`/`Err`, so `identity`, like any other module, cannot create a grant.
   - It carries the `action` and `target` it was issued for. It is neither `Clone` nor serializable, it is never stored, and it lives only for that request.
   - The action's use case takes it **by value** and calls `grant.redeem(expected_action, expected_target)`. When either value differs, the call fails, and the use case answers 500 `common.internal` without acting (a handler bug, fail closed). A grant for `route_enable` on account A can therefore never authorize a binding change on account B.
   - A missing step-up is a compile error, and a wrong one is a runtime refusal.
   - When the action is switched off by Q35, `authorize` still mints a grant, with no proof needed, for the same action and target.
4. **Precedence in the handler:**
   - before `authorize`: 401 → 403 origin → 403 `auth.forbidden` (admin) → 400 → 404. The 404 includes the target ownership, so another user's target never reaches a code check.
   - then step-up: 403 `auth.step_up_required` / `auth.code_invalid`.
   - then 409/422.
   - `Unavailable` maps to 500 `common.internal`. It fails closed: no grant, no action.
5. **Target ownership:** `trait AccountTargets::owned_by(user_id, account_id) -> bool` is defined in `identity`'s application crate, because identity consumes it. `crates/app` implements it with an adapter that calls an `accounts` query port, so `accounts` does not depend on `identity`.
   - Until `accounts` exists, the default answers `false`. For `requestStepUp` that means 404 `common.not_found` with nothing sent, identical to a missing account, and therefore fail closed.
   - The action itself scopes the account by `user_id` again when it executes.
6. **Provisional ports:** a module may start with its own provisional `AuthenticatedUser` port shaped like `Caller`, then switch to `access` when it merges. No module re-implements session lookup or step-up.

## 4. Consequences

- **Positive:**
  - no cross-module dependency on `identity`;
  - one implementation of session, Origin and step-up rules;
  - replay is impossible by construction.
- **Negative:**
  - a code is lost when the action is refused after step-up, so the user asks again. This is accepted: correct, never weaker;
  - `access` is a shared crate, so changes to it need Architect review.
- **Tests:**
  - two parallel requests with the same code: one runs the action, the other gets `auth.code_invalid`;
  - a code of another action, another target or another session gets `auth.code_invalid`;
  - `requestStepUp` on another user's account answers 404 and sends nothing;
  - an action refused with 409 after the step-up leaves the code unusable;
  - a grant for another action or another target passed to a use case gives 500 and no action;
  - a Q35-off action needs no proof;
  - `Unavailable` gives 500 and no action;
  - the extractor precedence 401 → 403 origin.
- **Owners:**
  - Backend S3 (Brasa): crate, `identity` implementation;
  - Malha, Torno, Ferro: consumers;
  - Software Architect: this ADR and the policy §6.1 delta.
