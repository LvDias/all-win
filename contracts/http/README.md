# HTTP API contract

`openapi.yaml` (OpenAPI 3.1, schemas are JSON Schema 2020-12) is the canonical web ↔ Core and webhook/pairing HTTP contract (ADR-0002 §4). Owner: Software Architect. Spec-first: change this file before implementing; the Core's generated spec (utoipa) must not drift from it (CI check), and the web generates its TypeScript client from it (`openapi-typescript`).

Slices covered in 1.0: V1-S1 (ingress), V1-S3 (auth, invitations, sessions, step-up, sources, connections, pairing, SSE events), and the S5 automation opt-in operation. Slice notes: [S1](../../docs/architecture/contracts/s1-webhook-ingest.md), [S3](../../docs/architecture/contracts/s3-access-sources-pairing.md), [S5 design deltas](../../docs/architecture/contracts/s5-design-deltas.md).

Self-check: `python3 contracts/scripts/check_openapi.py contracts/http/openapi.yaml` (refs, operationIds, all schemas incl. inline, examples, and the conventions: root session security, 401/400/403/500 coverage, `x-step-up` values). DevOps wires it into `make check`.
