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
| R1 rev. 3 (device/VPS/platform fields) | Lupa @ Dev2 (Sonnet) | implemented; commit + ff pending (Forja) | — | — | — |
| R3 — Presets research: 7 firms | Lupa @ Dev2 (Sonnet) | in progress | — | — | — |
| R2 — Presets research: 8 firms | Lupa @ Dev2 | implemented (8 files, PARTIAL; VPS field added) | — | User: Alpha (no NT8) scope; TPT/TradeDay questions to firms | PO review + consolidated conflicts |
| DB0 — Q13 audit retention + Wave 1 data model | Database Architect (Lastro @ Dev3) | rev. 3 integrated (Code Review approved) | 010e586 | Q-A3 from preset research | Backend migrations in §8 order, reviewed by Lastro |
| D1 — Design foundations + S3 rev. 3 / S4 / S5 rev. 2 specs | Product Designer (Aquarela @ Dev3) | integrated (d6a0e1e) | d6a0e1e | Architect: Q24/Q25 contract coverage, 'no choice yet' state | Frontend |
| API-SPEC — Core-generated OpenAPI (utoipa) + drift check vs contract (before S3 core merge) | Backend Rust (Ferro @ Dev3); rules by Esquadro; CI by Forja | implemented (72 tests; drift vs contract with Architect allowlist) | 3690528 + follow-up pending commit | S1 merge first | QA [Backend] Prisma + Code Review after commit |
| D2 — Design system upkeep + S3 passwordless screens | Product Designer (Aquarela @ Dev1, Sonnet; sole design-system owner) | in progress | — | — | S3 spec rewrite, then components for wave 2 |
| D3 — Screen specs wave 2 (S5 deltas D1–D16, S7, S6, S10, S16, S13) | Product Designer (Traço @ Dev1, Sonnet; specs only) | in progress | — | — | S5 first |
| C2 — HTTP contracts wave 2 (S5, S7, S6, S10, S16) | Software Architect (Compasso @ Dev1, Opus) | in progress | — | openapi.yaml single-writer: split vs serial deltas (with Esquadro) | S5 contract, then Code Review |
| P1 — S3 passwordless spec (slice-plan rev. 7, AC-1..15, S3b WhatsApp) | Product Owner (Rumo @ Dev3, Sonnet) | implemented (not committed); AC numbering collision being fixed | — | Q28–Q36, U16, U17 | Commit (Forja) |
| V1-S1 Webhook ingest — core | Backend Rust (Ferro @ Dev3) | deltas done; final candidate | 976e4f9 | — | QA [Security] CHANGES REQUESTED (0 crit, 0 high, 1 medium availability of per-IP limit, 1 low, 3 info) → contract decision (Esquadro) then fix; QA [Backend] APPROVED 976e4f9 (64 tests); QA [Backend], QA [Security], Code Review APPROVED 976e4f9; E2E APPROVED 976e4f9 + infra 5329015 (118/118, zero leaks, p95 10.5 ms); integrated (main 7e22e43; E2E 118/118 on main) |
| V1-S1 — infra (compose, Caddy redaction) | DevOps (Forja @ Dev2, Sonnet) | implemented (redaction canary test passes via fake upstream) | branch s/v1-s1-webhook-ingest--infra | Core not yet built | Full M1–M10 with real Core |
| V1-S1 — E2E + masking gate M1–M10 | QA [Integration & E2E] (Sentinela @ Dev2) | APPROVED core 43ee9ce + infra 5329015 (Core publishes no host port): 115/115 (masking M1–M10 zero leaks incl. Caddy debug, Core trace, pg_dump, metrics; idempotency; p95 12 ms) | e2e 88922fc | — | Re-run on final SHAs after S1 deltas |
| V1-S1 — HTTP contract (OpenAPI S1+S3, opt-ins Q24–Q26) | Software Architect (Prumo, dismissed) | integrated | main 984a8aa | — | Done; S5 design deltas D1–D16 queued for the S5 designer |
| V1-S3 Auth/isolation — core | Backend Rust (Brasa @ Dev2) | REPLANNING (passwordless requirement 2026-10-23); password/TOTP parts paused; was: domain, 35 use cases, crypto, store, migrations #2/#3/#8 (Lastro-approved); HTTP router next | uncommitted | merge order S1 → API-SPEC → S3; app wiring after API-SPEC | Router, then app wiring |
| V1-S4 i18n foundation + web scaffold | Frontend (Vitral @ Dev2, Sonnet) | integrated (main = 7d348f1) | 7d348f1 (types regenerated from 1.1.0) | QA [Frontend] APPROVED 7d348f1; QA [Security] APPROVED 7d348f1; Code Review (Crivo) APPROVED 7d348f1. Earlier: QA [Security]: APPROVED 3845cbf (0 crit/high/medium). QA [Frontend]: APPROVED 3845cbf (check, build, e2e 24/24). Code Review: APPROVED 3845cbf (precondition: contract on main + gen:api + drift check); merge after contract lands on main + type regen |
| V1-S3 web (login, invite, reset, TOTP, sources, connections) | Frontend (Vitral @ Dev2, Sonnet) | REPLANNING (passwordless); auth screens paused, sources/connections fixes continue; implemented (MSW) | dc9f10f | — | QA [Frontend] CHANGES REQUESTED (2 medium: focus return, Getting started card; 4 low); QA [Security] CHANGES REQUESTED (0 crit, 0 high, 1 medium, 3 low, 2 info); Code Review CHANGES REQUESTED (1 medium, 1 flaky e2e, 3 low); fix cycle with Vitral |
| V1-S2 — adapter protocol follow-ups: reason-codes registry, ADR-0004 clarifications, protocol 1.1 account_state (needed before S11) | Software Architect (Esquadro @ Dev3) | Code Review 4 CHANGES REQUESTED (1 high: re-execution after >7 days offline; 3 low) | uncommitted | U13 | Fix H1, re-review |
| V1-S2 hardening (Vigia follow-ups: NuGet lock files, token 0600, ws only in Development, store corruption typing) | Plugin Engineer (Elo @ Dev3) | integrated (main a9a95a7; log 2026-10-23-v1-s2-plugin-hardening) | 9a2ece9 | — | Remove row at next cleanup |
| V1-S2 prep — AllWin.Adapter.Core + fake-Core harness (Linux) | Plugin Engineer (Elo @ Dev3) | final candidate 1009999 (421 tests); Code Review APPROVED 1009999; QA [Plugins] APPROVED 1009999 (23 probes); QA [Security] APPROVED 1009999 (0 crit/high/medium, 2 low, 3 info); integrated (main 6579023); reason-codes/ADR-0004 committed (4fe3957, bb6b8ba, 46ffa80, d10c9a0) | 78305dd base | Windows VPS for NT8 parts | — |

## Environment notes

- Workspaces Dev1, Dev2, Dev3 share `/srv/www/allwin`, one Claude Max account each. Dev1 back in use since 2026-10-23 (user: use it if its limit returned).
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
| U1 | Windows VPS access (spike plan rev. 3 §6): T1 Tailscale for everything (user RDP + agents' SSH over the tailnet, no reverse tunnel; this dev server joins the tailnet), and Tailnet Lock? | T1 · T2 (Tailscale Mac↔VPS + reverse tunnel for agents) · T3 (own WireGuard, needs an inbound UDP port) | T1, Tailnet Lock on | APPROVED (T1 + Tailnet Lock) 2026-10-23 |
| U2 | Contract the Contabo Windows VPS (Windows Server 2022 Desktop Experience, 6–8 vCPU, 16–24 GB, 150–200 GB NVMe, St. Louis) and follow the checklist in the spike plan §6 | — | Contract | Final config delivered (nt8-spike-plan rev. 4 §6.1): Cloud VPS 8 (8 vCPU, 24 GB, 300 GB) or VPS Plus 8, Windows Server 2025 Desktop Experience (fallback 2022), St. Louis, no GPU, Contabo license add-on only — user to contract |
| U3 | S-8 Alpha Futures has no V1 platform (no NT8/Tradovate/Rithmic) | (a) manual-tracking-only preset, like Topstep · (b) exclude | (a) | CONFIRMED (a) |
| U4 | S-6 MyFundedFutures "Studio" plan | exclude · include | exclude | CONFIRMED (exclude) |
| U5 | S-7 How All Win classifies itself in presets: automated trading (TradingView sources) and trade copier (master replication) | as stated · other | as stated; user asks Bulenox/TradeDay/TPT whether All Win is allowed/"approved" | CONFIRMED |
| U6 | R1 (31 + 7 new rev. 3 gaps: Phidias/Lucid VPS unknown, Apex VPS conditional, Phidias/Lucid platform matrix, Phidias automation scope) and R2 (items 1–50) preset conflicts: ship as "not confirmed" until reviewed | — | as already decided | With the user (non-blocking) |
| U7 | Q26 scope: does a change of the firm's official rule TEXT (not only the stance) put an opt-in back to pending review? | stance only · stance or material text change | stance or material text change (audit stores the exact text shown) | CONFIRMED (stance or text) |
| U8 | Ownership of security headers: page headers (CSP nonce, nosniff, Referrer-Policy, Permissions-Policy) in Next proxy; HSTS at Caddy | as stated · all at Caddy | as stated | CONFIRMED |
| U9 | Supported browser floor for exact number formatting: Chrome 106+, Safari 15.4+, Firefox 116+ (Intl string input) | as stated · wider support with a decimal library | as stated | CONFIRMED |
| U10 | Q2 login lockout without account enumeration: per-identifier counter keyed by HMAC(pepper, normalized email) for ANY identifier, 20/h, 15-min lock, identical responses and timing; known-IP exemption only after a prior successful login | adopt · keep current policy §2 | adopt (QA Security and Architect agree) | SUPERSEDED by passwordless (2026-10-23); intent mapped by the Architect |
| U11 | Q3 pairing: remove the global pairing lock (cheap DoS for all users); keep 10/min per IP, add 30/min per /24 or /64 prefix, global threshold becomes an alert; optional 10-char codes for more margin | adopt · keep global lock | adopt (QA Security and Architect agree) | APPROVED (adopt) |
| U12 | C1: users who lost the phone and signed in with a recovery code have no self-service way out. Accept a recovery code in disableTotp only (password + TOTP code OR recovery code; code consumed, audited, other sessions ended, throttled)? stepUp and regenerateRecoveryCodes stay TOTP-only. Optional: warning e-mail on 2FA removal (new transactional e-mail type beyond Q20) | adopt · admin reset only · also in stepUp | adopt in disableTotp only; e-mail warning yes | SUPERSEDED by passwordless (2026-10-23) |
| U13 | Late partial exit (T1): exits never expire, but a `reduce` delivered late after the position changed (e.g., the user opened a new position by hand) could cut the wrong position. Add a position-identity guard on `reduce` (refs.position_ref: if the referenced position no longer exists, reject + notify)? `flatten` keeps not expiring | (A) position-identity guard: Core sends refs.position_ref (command_id that opened the position); adapter executes only if the current position is the same, else rejected + command.position_changed, account out_of_sync, user notified, audited; flatten unchanged · (B) reduce gets expires_at like entries (a late reduce of the same position is dropped, exposure stays higher) · (C) keep T1 | (A) | APPROVED (A) |
| U14 | Ingress rate limits defaults: 300/min per IP and 60/min per source key (TradingView sends from a few shared IPs) | as stated · other values | superseded by contract 1.1.2: flood cap 3000/min (30000/min for TradingView ranges), per-IP token-failure budget 30/min that never refuses a valid token, per-key 60/min | RESOLVED technically (QA Security finding); values remain tunable before opening sign-up |
| U15 | Live trading safety (later, before any live use): besides the Core's `simulation_only` flag, require an explicit local "allow live accounts" opt-in in the NT8 plugin UI (release builds) as defense in depth? | add local opt-in · rely on Core flag only | add local opt-in | APPROVED (add local opt-in) |
| U16 | Login code alphabet (passwordless) | (a) 8 uppercase alphanumerics without ambiguous characters · (b) mixed case + digits + specials | (a) — typed from a phone; with 5 attempts and 10-min expiry the entropy is ample; Architect to confirm | OPEN (security) |
| Q28–Q36 | Passwordless S3 open questions (slice-plan rev. 7 §7): Q28 loss of both channels, Q29 session/remember device, Q30 step-up per action vs window, Q31 WhatsApp mandatory?, Q32 one channel per account, Q33 send limits, Q34 new-device notice content, Q35 step-up list, Q36 invite e-mail = account e-mail | see slice-plan §7 | PO recommendations; Architect reviews Q33–Q35 | OPEN (user) |
| U17 | WhatsApp code provider | Meta Cloud API · Twilio · Zenvia · other | Meta Cloud API direct (est. US$4–110/month for 100–1,000 users); 2nd Twilio WhatsApp; critical path = Meta business verification (weeks) + dedicated number (user) — docs/product/research/whatsapp-otp-providers.md | OPEN (cost) — WhatsApp may ship after e-mail |


## Incidents
- 2026-10-23 ~04:20–05:50: Wave 1 briefs to Ferro (S1 core), Elo (S2 prep) and Sentinela (S1 E2E) were never delivered (Maestro's dispatch command hung before sending). Detected when Brasa reported the missing core scaffold; briefs resent. Impact: ~1.5 h idle on S1 core and S2 prep; S3 core blocked on the scaffold.
