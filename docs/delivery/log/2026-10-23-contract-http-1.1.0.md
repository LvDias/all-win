# 2026-10-23 — HTTP contract 1.1.0 (S1 + S3 + S5 opt-ins) and ADR-0001 masking/headers

- **Merge (local, no push), by DevOps:** `main` = `984a8aa`. Commits: `407ab71` docs(architecture) ADR-0001 §5 masking, §9 security headers ownership, `uqr` dependency · `b6c8cf5` feat(contracts) S1 OpenAPI (rebased from `45069cc`) · `984a8aa` feat(contracts) HTTP 1.1.0 (rebased from `35a211b`).
- **Scope:** `contracts/http/openapi.yaml` 1.1.0 (37 operations), `contracts/scripts/check_openapi.py`, `docs/architecture/contracts/{s1-webhook-ingest,s3-access-sources-pairing,s5-design-deltas}.md`, ADR-0001. Inputs for V1-S1, V1-S3, V1-S4, V1-S5.
- **Gates:** Code Reviewer — Code Review 1 CHANGES REQUESTED (2 high, 5 medium, 8 low) → all 15 resolved → APPROVED on exact content hashes (`6a9cb65c2503` contracts, `39bcacb09dc5` docs-architecture), commits re-checked. Software Architect is the owner. QA [Security] input on Q2/Q3 incorporated as proposals. `check_openapi` OK (37 ops, 161 schemas); `validate_fixtures` ALL OK.
- **Security summary:** secret-masking leak variants found in review were fixed at contract level (ingress-like scope, no URI in Caddy access logs, M6 a–o test spec). Implementation verification happens in the S1 gate (M1–M10).
- **Known limitations:** Q26 text-change trigger provisional (board U7). Q2/Q3 security proposals await the user (U10, U11); contract unchanged for them.
- **Accepted risks:** none.
- **Follow-ups:** S4 web regenerates `schema.gen.ts` from 1.1.0 and wires the drift check; S5 design applies D1–D16; Caddy rule implemented in S1 infra with M6 as gate.
