# Data architecture — All Win

Owner: Database Architect (Lastro @ Dev3). Assigned by the Maestro on 2026-10-23 (board item DB0). Architecture decisions remain in the ADRs; the documents here hold the data model, retention and migration conventions that implement them.

| Document | Status | Content |
|---|---|---|
| [q13-audit-retention.md](q13-audit-retention.md) | Proposed — awaiting user decision on Q13 / Q13-b | Audit/activity retention options, volume and cost estimates, recommendation, secret-masking rule for stored data |
| [wave1-data-model.md](wave1-data-model.md) | Proposed — awaiting Architect alignment (ALIGN-1…7) and PO answers (Q-A1…A5, Q-N1) | Schemas and tables for S1, S3, S5–S7 (+ previews for S2/M1, S13, S19), numeric types, RLS isolation, migration conventions and Wave 1 order |

Rules that apply to every migration are in [wave1-data-model.md §8](wave1-data-model.md#8-migration-conventions-and-wave-1-order) and [§3](wave1-data-model.md#3-per-user-isolation-application-scoping--rls).
