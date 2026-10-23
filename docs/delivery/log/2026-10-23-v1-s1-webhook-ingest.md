# 2026-10-23 — V1-S1 Webhook ingest (TradingView → Core)

- **Merge (local, no push), by DevOps, under the standing authorization:** merge commits preserving the reviewed SHAs — `020c02f` core at `976e4f9`, `4a660ff` infra at `5329015`, `7e22e43` e2e suite at `942739d`. `main` = `7e22e43`.
- **Scope:** Core Cargo workspace scaffold; TradingView ingress `/ingress/v1/tradingview/{token}` per contract HTTP 1.1.2 (flood cap, per-IP token-failure budget that never refuses a valid token, per-key limit, credential guard, uniform empty 404); idempotent receipt by (user_id, message_id); instruction storage with expiry base; audit via transactional outbox with retries/dead-letter; RLS fail-closed; Core-side masking; dev compose (PostgreSQL 16, Caddy with no URI in access logs, Core with no host port); E2E suite. Covers S1 AC-1…AC-7.
- **Gate verdicts (exact SHAs):**
  - QA [Backend] (Prisma): APPROVED `976e4f9` — 64 tests incl. DB, RLS, idempotency.
  - QA [Security] (Vigia): APPROVED `976e4f9` — summary: earlier 1 medium (per-IP limit availability) + 1 low fixed; 0 open critical/high/medium/low; 3 info tracked.
  - Code Reviewer (Crivo): APPROVED `976e4f9` — earlier medium (contract 1.1.2 limits) and low (outbox retries) fixed.
  - QA [Integration & E2E] (Sentinela): APPROVED core `976e4f9` + infra `5329015` — 118/118, zero secret occurrences across Caddy (incl. debug), Core trace, DB dump, metrics, responses; p95 10.5 ms through Caddy (budget 300 ms).
  - Database Architect (Lastro): migrations approved.
  - Integration on `main` `7e22e43`: E2E 118/118 again, trees identical to the reviewed SHAs, p95 14.8 ms.
- **Known limitations:** p95 measured with a loaded host (upper bound); IP flood cap of 3000/min and the unknown-key cache not observable black-box; no CI pipeline yet; `/metrics` must never be routed at the public edge; HSTS to be set at Caddy before any HTTPS exposure. Web types on `main` are from contract 1.1.0 while the contract is 1.1.3 (drift fix in progress).
- **Accepted risks:** none.
- **Follow-ups:** API-SPEC (Core-generated OpenAPI + drift check) before S3 core merge; M1 needs per-consumer delivery tracking for routing.
