# S5 opt-in — deltas the design spec must apply

- **Status:** Ready for Design (2026-10-23). Owner: Software Architect. Requested by Code Review 1 (finding 6).
- **Applies to:** [v1-s5 spec](../../design/specs/v1-s5-automation-opt-in.md) rev. 2 (Product Designer), via the Maestro.
- **Contract:** [`contracts/http/openapi.yaml`](../../../contracts/http/openapi.yaml) 1.1.0: operation `setAccountOptIn`, schemas `OptInChoiceRequest`, `OptInChoices`, `OptInState`, `FirmStance`. Context: [S3 contract notes §2](s3-access-sources-pairing.md). Where the spec and the contract differ, the contract wins.
- **Decisions behind it:** option A, Q24, Q25 and Q26 approved by the user on 2026-10-23 ([decisions.md](../../delivery/decisions.md)).

## 1. Names and fields

| # | Spec rev. 2 says | Contract (apply this) |
|---|---|---|
| D1 | `setAccountAutomationChoice`, `PUT /api/v1/accounts/{account_id}/automation` | `setAccountOptIn`, `PUT /api/v1/accounts/{account_id}/opt-ins/{kind}`, `kind` = `automation` or `copy` (Q24) |
| D2 | Request `automation_choice`, `automation_rule_version_id` | `choice` (`accepted` / `declined`), `rule_version_id` |
| D3 | Response `AutomationState`: `automation_state`, `automation_choice`, `automation_chosen_at`, `automation_rule_version_id` | `OptInState`: `kind`, `state` (`not_required` / `chosen` / `pending_review`), `firm_stance`, `choice`, `chosen_at`, `rule_version_id`, `current_rule_version_id`. With `pending_review`, `choice` is the **previous** choice (shown for comparison, never preselected) |
| D4 | 409 body carries `automation_rule_version_id` | 409 `accounts.rule_version_changed`, body field `error.current_rule_version_id` |
| D5 | 422 `accounts.automation_choice_not_applicable` | 422 `accounts.opt_in_not_applicable` (state is `not_required`). Same UI: refetch, section disappears, no error shown |
| D6 | Stance `automated_trading_allowed` ∈ `yes / no / conditional / unknown` + provenance (verified / not confirmed) | `FirmStance` ∈ `allowed / forbidden / conditional / unknown / not_confirmed`. Mapping: `yes` verified → `allowed`; `no` → `forbidden`; `conditional` → `conditional`; `unknown` or missing → `unknown`; any value whose provenance is not confirmed → `not_confirmed`. The summary-line copy of spec §3 keys on these five values |
| D7 | Routing skip reason (text only) | Codes `routing.automation_not_accepted` / `routing.copy_not_accepted`; the audit text of spec §7 translates them |

## 2. Behavior

| # | Spec rev. 2 says | Contract (apply this) |
|---|---|---|
| D8 | Create flow: create the account, then step-up, then `setAccountAutomationChoice`; the account may be "saved without a choice" (alert + `Escolher agora`) | **Atomic create-with-choice.** The S5 create and update requests carry the choices in the `opt_ins` property (`OptInChoices`: `{ "automation": { "choice", "rule_version_id" }, "copy": {…} }`). One request, one save. Remove the "saved without a choice" alert, its sequencing and the related Architect question: that state does not exist |
| D9 | Step-up only for `setAccountAutomationChoice` with `accepted` | The S5 create/update operations are `x-step-up: conditional` when an `opt_ins` choice is `accepted` and changes the stored state; `setAccountOptIn` is `x-step-up: conditional` with `choice == accepted`. Step-up runs **before** the single save; cancelling it saves nothing and keeps the form values (already spec §9). Applies to `copy` as well as `automation` |
| D10 | Save with no choice → inline error (client side) | Unchanged in the UI. The server answers 422 `accounts.opt_in_required` with `pointer` `/opt_ins/automation` or `/opt_ins/copy`; map the pointer to the radio group of that kind and focus it |
| D11 | 409 stale rule version on save | Unchanged in the UI (reload the section, clear the choice, keep other fields). It can now come from create/update too, not only from `setAccountOptIn` |
| D12 | "The contract applies the choice only when the firm forbids automation; Q25 cases need a contract change" | Q25 approved and already in the contract: a choice is required when the stance is `forbidden`, `conditional`, `unknown` or `not_confirmed`. Delete the note |
| D13 | Q24 not drawn; `copy` component "reusable" | Q24 approved. Draw the `copy` variant of the section (same structure, copy wording) for the moment an account is added as a follower (S7/S19), where the `copy` choice is made atomically. In the automation help text use the variant "A replicação da conta master tem uma escolha própria." |
| D14 | Q26 drawn as option (b), triggered by a change of stance **or** official text | Q26 approved as (b) for a **stance change**. A material change of the official text with the same stance also gives `pending_review`: **provisional Architect decision, awaiting user confirmation**. Keep the drawing; label the text-change case as provisional |
| D15 | Prop accounts created before S16: `Escolha pendente` | Confirmed as `state = pending_review` (PO still confirms the migration behavior). It is the only way an account reaches the pending state without a prior choice |
| D16 | Changing firm/plan/phase/size/firm platform clears the choice | Unchanged in the UI. If the change selects a different rule version, the update request must carry new `opt_ins` for the affected kinds, or the Core answers 422 `accounts.opt_in_required` |

## 3. Defined later in the S5 contract

The rest of the S5 account create/update operations (paths, other fields, errors) is defined in the S5 HTTP contract, which follows the S5 spec. The points above (`opt_ins` property, pointers, conditional step-up, 409/422 codes) are fixed now and will not change there.
