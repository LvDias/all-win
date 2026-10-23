# Delivery Board — All Win

> Current state only. Maintained by Maestro (single writer). Status vocabulary: specified, in progress, implemented, QA verified, reviewed, integrated, deployed, business accepted.
> Product baseline: [product-vision.md](../product/product-vision.md) (rev. 3).

| Slice | Owner | Status | Revision | Blockers | Next step |
|---|---|---|---|---|---|
| S0 — Stack, repo structure, root CLAUDE.md | Software Architect (Prumo @ Dev3) | approved; ADR-0001/0002 Accepted, CLAUDE.md approved | — | — | Initial local commit (DevOps) |
| P0 — V1 slice plan (rev. 4) | Product Owner (Bússola @ Dev2) | approved; S1–S18, S21, S22, R1–R3 ready; S19/S20 blocked (ADR-0003/0004) | — | Q17–Q23 (screen specs), Q13 | User answers |
| W0-a — ADR-0003 instruction contract v1 + fixtures | Prumo @ Dev3 | Accepted (contract deltas, fixtures ALL OK) | — | — | S1 implementation |
| W0-b — ADR-0004 Core↔plugin protocol v1 + fixtures | Prumo @ Dev3 | Accepted (§6.1 OCO location; fixtures ALL OK) | — | — | S2 after spike |
| W0-c — NT8 spike plan + Windows VPS access | Prumo @ Dev3 (Trilha dismissed) | spike plan rev. 3 (Tailscale T1 recommended) | 78305dd | User: approve T1 + Tailnet Lock; contract VPS | User decision |
| W0-d — Host tooling, shared-repo hygiene, .gitignore, initial local commit | DevOps (Forja @ Dev2) | in progress | — | Firewall changes need user authorization | Report versions; commit after S0 final |
| R1 — Presets research: Apex, Phidias, Lucid | Lupa @ Dev2 (format by Bússola) | PO reviewed; 31 conflicts in docs/product/presets/r1-conflicts-for-user.md | — | User: conflict list, Q24–Q27 | User decision (non-blocking) |
| R2 — Presets research: 8 firms | Lupa @ Dev2 | implemented (8 files, PARTIAL; VPS field added) | — | User: Alpha (no NT8) scope; TPT/TradeDay questions to firms | PO review + consolidated conflicts |
| DB0 — Q13 audit retention + Wave 1 data model | Database Architect (Lastro @ Dev3) | Accepted rev. 2 (DDL not executed yet) | — | Q-A3 from preset research | Backend migrations in §8 order, reviewed by Lastro |
| D1 — Design foundations + S3 rev. 3 / S4 / S5 rev. 2 specs | Product Designer (Aquarela @ Dev3) | integrated (d6a0e1e) | d6a0e1e | Architect: Q24/Q25 contract coverage, 'no choice yet' state | Frontend |
| V1-S1 Webhook ingest — core | Backend Rust (Ferro @ Dev3) | in progress (owns core workspace skeleton) | 78305dd base | — | Skeleton first, then S1 |
| V1-S1 — infra (compose, Caddy redaction) | DevOps (Forja @ Dev2) | in progress | 78305dd base | — | Compose on 127.0.0.1 |
| V1-S1 — E2E + masking gate M1–M10 | QA [Integration & E2E] (Sentinela @ Dev2) | in progress (tests from spec) | 78305dd base | — | Verdict on implemented revision |
| V1-S1 — HTTP contract (OpenAPI S1+S3) | Software Architect (Prumo @ Dev3) | specified (uncommitted in contracts worktree) | — | — | Commit with S1 integration |
| V1-S3 Auth/isolation — core | Backend Rust (Brasa @ Dev2) | in progress (waits for core skeleton) | 78305dd base | Ferro's skeleton | Identity module |
| V1-S4 i18n foundation + web scaffold | Frontend (Vitral @ Dev2) | in progress | 78305dd base | — | Then S3 web |
| V1-S2 prep — AllWin.Adapter.Core + fake-Core harness (Linux) | Plugin Engineer (Elo @ Dev3) | in progress | 78305dd base | Windows VPS for NT8 parts | — |

## Environment notes

- Workspaces Dev1, Dev2, Dev3 share `/srv/www/allwin`, one Claude Max account each. Dev1 is paused by the user until further notice; use Dev2 and Dev3.
- Repository has no commits and no remote yet.
- Host has only git: Rust, Node, .NET SDK, Docker, PostgreSQL must be provisioned (DevOps) before scaffolding.
- safe.directory fixed by the user; git verified for dev2 and dev3.
- Maestri link outage resolved after the user reconnected Dev2/Dev3 (Lastro @ Dev3 still reconnecting).
- Tooling provisioned and verified (Rust 1.99.1, Node 24.22.1, pnpm 12.15.0, .NET 10, Docker 29.9, psql 16).

## Working rules (2026-10-23)
- No agent writes in the main checkout; docs live in per-owner worktrees under /srv/www/allwin.worktrees/docs-*; design uses s-v1-s3-auth-isolation--design.
- Sonnet preset for routine work (research, DevOps, routine QA, frontend with ready spec, PO/design doc upkeep); Opus for architecture, risk/order logic, security, code review. Switching Lupa, Forja, Vitral, Bússola, Aquarela to Sonnet after handoff.

## Open questions for the user (overnight protocol)

| # | Question | Options | Recommendation | Status |
|---|---|---|---|---|
| U1 | Windows VPS access (spike plan rev. 3 §6): T1 Tailscale for everything (user RDP + agents' SSH over the tailnet, no reverse tunnel; this dev server joins the tailnet), and Tailnet Lock? | T1 · T2 (Tailscale Mac↔VPS + reverse tunnel for agents) · T3 (own WireGuard, needs an inbound UDP port) | T1, Tailnet Lock on | OPEN (security/network) |
| U2 | Contract the Contabo Windows VPS (Windows Server 2022 Desktop Experience, 6–8 vCPU, 16–24 GB, 150–200 GB NVMe, St. Louis) and follow the checklist in the spike plan §6 | — | Contract | OPEN (cost) |
| U3 | S-8 Alpha Futures has no V1 platform (no NT8/Tradovate/Rithmic) | (a) manual-tracking-only preset, like Topstep · (b) exclude | (a) | DECIDED PROVISIONALLY (a), awaiting confirmation |
| U4 | S-6 MyFundedFutures "Studio" plan | exclude · include | exclude | DECIDED PROVISIONALLY (exclude), awaiting confirmation |
| U5 | S-7 How All Win classifies itself in presets: automated trading (TradingView sources) and trade copier (master replication) | as stated · other | as stated; user asks Bulenox/TradeDay/TPT whether All Win is allowed/"approved" | DECIDED PROVISIONALLY, awaiting confirmation |
| U6 | R1 (31) and R2 (items 1–50) preset conflicts: ship as "not confirmed" until reviewed | — | as already decided | Waiting for user review (non-blocking) |
