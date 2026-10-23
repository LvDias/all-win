# 2026-10-23 — U13 position-identity guard (adapter protocol 1.2) and ADR-0005 N11 live-account consent table (plugin)

- **Scope:** user decision U13 option (A): a late `reduce` executes only if the referenced position is still the same (`refs.position_ref`), else `command.position_changed`, account out_of_sync, user notified, audited; `flatten` unchanged. User decision U15: local "Permitir contas reais" opt-in in the NT8 plugin, implemented as the ADR-0005 §N11 decision table (state × operation) with U19 (a) / U19.1 (b) as configurable defaults (decided by the user, residual risk accepted).
- **Revisions and merges (local, then pushed to the private repository):**
  - adapter protocol 1.2 contract `53a86ad` → merge `1acd752`; reason code `order.live_consent_cancelled` `be6c238` → merge `6c3e966`;
  - architecture docs (ADR-0003/0004 1.2 guard, ADR-0005 N11 table, ADR-0006) → merge `092514d`;
  - plugin `92fa116` (delta over `b576867`, owner Elo) → merge `1ef4e24`.
- **Gates:**
  - Contract 1.2 (patch `1c17ef076d5d`; commit content verified by Code Review): Code Review (Crivo), QA [Plugins] (Pêndulo), QA [Security] (Vigia): APPROVED.
  - ADR-0005 N11 table: Code Review, QA [Plugins], QA [Security]: APPROVED (after three rounds; the decision table replaced free text).
  - Plugin `b576867` → `92fa116`: QA [Plugins] (Pêndulo, Linux scope), Code Review (Bisturi), QA [Security] (Vigia): CHANGES REQUESTED on `b576867`, APPROVED on `92fa116`. QA Security summary: 2 low + 1 info fixed, 0 open.
  - Validation: `dotnet test` 673/673, 144 cell tests (36 cells × 4 U19/U19.1 combinations), 22 guard tests, mutation runs proving the tests catch each defect; Release build 0 warnings.
- **Known limitations:** UNTESTED until the Windows VPS: net48 host, consent dialog, DPAPI/ACL of the consent file, kind detection on NT8, restart behaviour (E4/E9), commissions (E11).
- **Accepted risks:** residual I2 gap — a flatten that fails after cancelling the legs leaves the position unprotected; mitigated by a top-severity alert + out_of_sync. Accepted by the user (decisions.md, 2026-10-23).
- **Follow-ups:** plugin follow-up `b4295b3` (shared-mode row-6 cap) CHANGES REQUESTED by QA [Plugins] (M10: a resting add received before the owner but filled after it), fix pending with Elo; protocol 1.3 implementation after its approval.
