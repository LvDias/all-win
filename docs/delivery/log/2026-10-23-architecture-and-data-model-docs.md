# 2026-10-23 — Architecture (spike plan rev. 6.1, ADR-0003 S7 gates) and data model rev. 12–16.3

- **Merges (local, then pushed):**
  - `d/architecture` → main 266f1d9: 2ca46b4 and fe1f673 (cumulative 1eac5a9ab79e).
  - `d/data` → main f7e9a4e: data model rev. 12 to 16.3 (cumulative 0483e3472235).
  - Documentation only.
- **Scope:**
  - **NT8 spike plan rev. 6/6.1:** the VPS already joined the tailnet; it is tagged from the admin console, with no auth keys; Tailnet Lock is enabled after tagging.
  - **ADR-0003 S7:**
    - numbered gate order;
    - `routing.platform_unbound` and `routing.no_bracket_profile`;
    - Q74 by position (opposite → close only; flat → gate code; already on the requested side → position conflict, never a flatten);
    - exits never expire at routing.
  - **Data model:**
    - rev. 12: audit actor;
    - rev. 13: S13 risk;
    - rev. 14: S21 trades;
    - rev. 15–15.2: net P&L, commission floor, evaluation target, shared-fill commission;
    - rev. 16–16.3: binding in accounts per ADR-0008; Wave 1 migration versions; outbox no-consumer allowlist and per-consumer completion with `from_now` start_seq.

## Gate verdicts (exact diff hashes, sha256[:12])

| Commit | Content | Verdicts |
|---|---|---|
| 2ca46b4 | Spike plan rev. 6/6.1 | Code Review, QA Security, Architect APPROVED 095fa227a617 |
| fe1f673 | ADR-0003 S7 gates | Code Review, QA Security APPROVED e96b194dd57a |
| 1647af5 … e39d83b | Data model rev. 12–15.2 | Code Review APPROVED on each exact hash (author: Database Architect) |
| c509981 | Data model rev. 16 | Code Review APPROVED 1741556b905a (cumulative over WIP 9dcc772) |
| 1a634da, 7acd73c, 4cc8085 | Data model rev. 16.1–16.3 | Code Review APPROVED ff6a5085a7d5, 0775df1aef8d, c1148f6ad77d |

- QA Security was not requested for the data model document. Schema code and migrations get QA Security in their slices.
- Docs link check on the changed files: 0 broken (DevOps).

## Not in this merge

- `d/product` (Q73/Q74 slice-plan and vision updates, 799783a and 33f24e2) is waiting for a review verdict on the exact hash.

## Follow-ups

- Outbox purge job and per-consumer tracking: assigned to Backend (Brasa), after S3 core.
- Infra lows L1–L3 from the Phase 2 Code Review: tracked by DevOps for the next infra change.
