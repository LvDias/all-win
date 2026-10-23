# Delivery Board — All Win

> Current state only. Maintained by Maestro (single writer). Status vocabulary: specified, in progress, implemented, QA verified, reviewed, integrated, deployed, business accepted.
> Product baseline: [product-vision.md](../product/product-vision.md) (rev. 3).

| Slice | Owner | Status | Revision | Blockers | Next step |
|---|---|---|---|---|---|
| S0 — Stack, repo structure, root CLAUDE.md | Software Architect (Prumo @ Dev3) | approved; ADR-0001/0002 Accepted, CLAUDE.md approved | — | — | Initial local commit (DevOps) |
| P0 — V1 slice plan (rev. 3) | Product Owner (Bússola @ Dev2) | approved; S1–S18, S21, S22, R1–R3 ready; S19/S20 blocked (ADR-0003/0004) | — | Q17–Q23 (screen specs), Q13 | User answers |
| W0-a — ADR-0003 instruction contract v1 + fixtures | Prumo @ Dev3 | accepted by user (P1–P5 answered) | — | — | Owner marks Accepted; S1 can start |
| W0-b — ADR-0004 Core↔plugin protocol v1 + fixtures | Prumo @ Dev3 | accepted by user (T1–T3 answered, TTL 5 s) | — | — | Owner marks Accepted; S2 after spike |
| W0-c — NT8 spike plan + Q15 build/test options | Prumo @ Dev3, with Trilha @ Dev3 (NT8 profile) | spike plan + NT8 profile specified; Q15 options to reconcile (Prumo O1 overlay vs Trilha B reverse tunnel) | — | Maestri link down; user W1–W3, NT8 ToS reading; Windows machine | Prumo consolidates Q15 options |
| W0-d — Host tooling, shared-repo hygiene, .gitignore, initial local commit | DevOps (Forja @ Dev2) | in progress | — | Firewall changes need user authorization | Report versions; commit after S0 final |
| R1 — Presets research: Apex, Phidias, Lucid | Lupa @ Dev2 (format by Bússola) | implemented (3 files, PARTIAL); user chose option A | — | — | PO review + consolidated conflict list |
| R2 — Presets research: 8 firms | Lupa @ Dev2 | in progress | — | — | PO review |
| DB0 — Q13 audit retention + initial data model (S1, S3, S5–S7) | Database Architect (Lastro @ Dev3) | Q13 + Wave 1 data model proposed (DDL not executed: no PostgreSQL yet) | — | User Q13, Q13-b, Q-A1–A5, Q-N1; Prumo ALIGN-1..7 | Architect alignment, then Backend migrations |
| D1 — Design foundations + S3/S4 specs | Product Designer (Aquarela @ Dev3) | in progress | — | — | PO review |
| S1, S3, S4 | — | specified / waiting | — | ADR-0003 (S1), scaffold + tooling (all) | Start after W0-a/W0-d |

## Environment notes

- Workspaces Dev1, Dev2, Dev3 share `/srv/www/allwin`, one Claude Max account each. Dev1 is paused by the user until further notice; use Dev2 and Dev3.
- Repository has no commits and no remote yet.
- Host has only git: Rust, Node, .NET SDK, Docker, PostgreSQL must be provisioned (DevOps) before scaffolding.
- safe.directory fixed by the user; git verified for dev2 and dev3.
- Maestri link outage resolved after the user reconnected Dev2/Dev3 (Lastro @ Dev3 still reconnecting).
- Tooling: user running provision-dev-host.sh; Wave 1 implementation (S1, S3, S4) starts once tooling is verified and worktrees exist.
