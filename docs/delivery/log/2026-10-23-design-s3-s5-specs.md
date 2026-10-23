# 2026-10-23 — Design: S3 spec rev. 3, S5 opt-in spec rev. 2, frontend handoff

- **Merge:** `d6a0e1e` fast-forwarded into `main` (local, no push), by DevOps (Forja).
- **Scope:** docs only — `docs/design/specs/v1-s3-access-sources-pairing.md` (rev. 3: invite/reset tokens in URL fragment, pairing lock states per ADR-0004 §8, TOTP flow, time_zone on invite acceptance), `docs/design/specs/v1-s5-automation-opt-in.md` (rev. 2), `docs/design/README.md`, `docs/design/handoff-frontend-s3-s4.md`. Covers design inputs for V1-S3, V1-S4, V1-S5 ACs.
- **Gates:** documentation change; owner review (Product Designer) and consistency with the S3 contract (Software Architect requests applied). Secret/PII scan clean (DevOps). No code, so area QA / Code Reviewer not applicable.
- **Known limitations:** 6 links to `contracts/http/openapi.yaml` and `docs/architecture/contracts/` resolve only after the S1 contracts worktree is integrated. Contract follow-up pending: Q25/Q24 coverage of the automation/copy opt-in and the "prop account without a choice" state (Architect).
- **Accepted risks:** none.
- **Follow-ups:** Frontend implements S4 then S3 web against the contract with MSW.
