# HTTP API contract

`openapi.yaml` (OpenAPI 3.1, schemas are JSON Schema 2020-12) is the canonical web ↔ Core and webhook/pairing HTTP contract (ADR-0002 §4). Owner: Software Architect. Spec-first: change this file before implementing; the Core's generated spec (utoipa) must not drift from it (CI check), and the web generates its TypeScript client from it (`openapi-typescript`).

Slices covered in 1.0: V1-S1 (ingress), V1-S3 (auth, invitations, sessions, step-up, sources, connections, pairing, SSE events), and the S5 automation opt-in operation. Slice notes: [S1](../../docs/architecture/contracts/s1-webhook-ingest.md), [S3](../../docs/architecture/contracts/s3-access-sources-pairing.md), [S5 design deltas](../../docs/architecture/contracts/s5-design-deltas.md).

## Layout (multi-file, agreed between the two Software Architects, 2026-10-23)

- `openapi.yaml` — the **root**: `info`, `servers`, root `security`, `securitySchemes`, common components (`Error`, common responses 400/401/403/404/409/429/500, `NoStore`, common parameters) and one path-item `$ref` per module path. Owner: Esquadro (Software Architect). Auth/me/admin/sources/connections/pairing/ingress stay inline in the root for now.
- `modules/<module>.yaml` — one file per business module with only `paths` and `components` (its own schemas/responses/parameters). One owner per module: `accounts`, `routing`, `profiles`, `audit`, `presets` → Compasso (Software Architect).
  - Root path entry: `/api/v1/accounts: { $ref: 'modules/accounts.yaml#/paths/~1api~1v1~1accounts' }` (same path key on both sides).
  - In a module: root components are `'../openapi.yaml#/components/...'`, the module's own are `'#/components/...'`, another module's are `'<module>.yaml#/components/...'`.
  - Component names are unique across the root and all modules (the bundler fails on a duplicate).
- `openapi.bundled.yaml` — **generated, committed, never edited**: module path items inlined and module components merged into the root namespace. It is the single file for the Core's drift check (utoipa, `drift-allowlist.yaml`) and the one every check below runs on. Regenerate with `--write` after any change to the root or a module.
- Web types: generate from `openapi.bundled.yaml` (`ALLWIN_OPENAPI=../contracts/http/openapi.bundled.yaml`). Today, with no module files, the root and the bundle produce a byte-identical `schema.gen.ts`; once a module exists, `openapi-typescript` on the multi-file root drops schemas that are not reachable from an operation and reorders the rest, so the bundle is the only stable input.

Self-check: `python3 contracts/scripts/check_openapi.py contracts/http/openapi.yaml` bundles the root and modules, fails if the committed `openapi.bundled.yaml` differs from the generated one, then checks the bundle (module shape, refs, operationIds, all schemas incl. inline, examples, and the conventions: root session security, 401/400/403/500 coverage, `x-step-up` values). `--write` regenerates the bundle. DevOps wires it into `make check`.
