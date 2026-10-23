# 2026-10-23 — HTTP contract 1.1.3

- **Scope:** fix release of the HTTP contract. Two descriptions that YAML parsed as null keys are quoted; `check_openapi.py` gains a null-value lint (`find_nulls`). No operation or schema change.
- **Revision:** `d37149d` (branch `s/v1-s1-webhook-ingest--contracts`, owner Software Architect). Integrated into `main` as merge commit `533c191` (local, no push).
- **Gates (exact SHA `d37149d`):** Code Review (Crivo): APPROVED — commit diff hash `b50fafa1de51`, `check_openapi.py` at the exact SHA: OPENAPI CHECK OK (37 operations, 161 schemas, 5 examples).
- **Integration check on main:** `check_openapi.py` + fixture validation (DevOps).
- **Known limitations:** web generated types on `main` still come from 1.1.0; regeneration on `s/v1-web-types-1.1.3--web` (Vitral) pending QA [Frontend] + Code Review.
- **Accepted risks:** none.
- **Follow-ups:** OpenAPI split by module (Compasso, step 1) rebases on this merge; contract 2.0.0 (passwordless) builds on the split.
