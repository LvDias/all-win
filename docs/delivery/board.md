# Delivery Board — All Win

> Current state only. Maintained by Maestro (single writer). Status vocabulary: specified, in progress, implemented, QA verified, reviewed, integrated, deployed, business accepted.
> Product baseline: [product-vision.md](../product/product-vision.md) (rev. 3).

| Slice | Owner | Status | Revision | Blockers | Next step |
|---|---|---|---|---|---|
| S0 — Stack, repo structure, root CLAUDE.md | Software Architect (Prumo @ Dev3) | approved; ADR-0001/0002 Accepted, CLAUDE.md approved | — | — | Initial local commit (DevOps) |
| P0 — V1 slice plan (rev. 4) | Product Owner (Bússola @ Dev2) | approved; S1–S18, S21, S22, R1–R3 ready; S19/S20 blocked (ADR-0003/0004) | — | Q17–Q23 (screen specs), Q13 | User answers |
| W0-a — ADR-0003 instruction contract v1 + fixtures | Prumo @ Dev3 | Accepted (contract deltas, fixtures ALL OK) | — | — | S1 implementation |
| W0-b — ADR-0004 Core↔plugin protocol v1 + fixtures | Prumo @ Dev3 | Accepted (§6.1 OCO location; fixtures ALL OK) | — | — | S2 after spike |
| W0-c — NT8 spike plan + Q15 build/test options | Prumo @ Dev3, with Trilha @ Dev3 (NT8 profile) | spike plan rev. 2 + ADR-0005 (Proposed) | — | User W1–W3; Windows machine | User decision |
| W0-d — Host tooling, shared-repo hygiene, .gitignore, initial local commit | DevOps (Forja @ Dev2) | in progress | — | Firewall changes need user authorization | Report versions; commit after S0 final |
| R1 — Presets research: Apex, Phidias, Lucid | Lupa @ Dev2 (format by Bússola) | PO reviewed; 31 conflicts in docs/product/presets/r1-conflicts-for-user.md | — | User: conflict list, Q24–Q27 | User decision (non-blocking) |
| R2 — Presets research: 8 firms | Lupa @ Dev2 | in progress (3/8); user decided S-1..S-4 | — | — | PO review; S-5 VPS rule per firm |
| DB0 — Q13 audit retention + Wave 1 data model | Database Architect (Lastro @ Dev3) | Accepted rev. 2 (DDL not executed yet) | — | Q-A3 from preset research | Backend migrations in §8 order, reviewed by Lastro |
| D1 — Design foundations rev. 2 + S3/S4 FINAL + S5 opt-in spec | Product Designer (Aquarela @ Dev3) | specified | — | Architect contract items; user Q24–Q26 | PO review, then Frontend |
| S1, S3, S4 | — | ready; contracts in worktree s-v1-s1-webhook-ingest--contracts (OpenAPI 37 ops) | — | Tooling re-run (DevOps) | Recruit implementers |

## Environment notes

- Workspaces Dev1, Dev2, Dev3 share `/srv/www/allwin`, one Claude Max account each. Dev1 is paused by the user until further notice; use Dev2 and Dev3.
- Repository has no commits and no remote yet.
- Host has only git: Rust, Node, .NET SDK, Docker, PostgreSQL must be provisioned (DevOps) before scaffolding.
- safe.directory fixed by the user; git verified for dev2 and dev3.
- Maestri link outage resolved after the user reconnected Dev2/Dev3 (Lastro @ Dev3 still reconnecting).
- Tooling: first provision run failed at Node (npm shebang, PATH under sudo); DevOps making the script idempotent for a re-run; Wave 1 implementation (S1, S3, S4) starts once tooling is verified and worktrees exist.
