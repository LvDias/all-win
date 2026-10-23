# 2026-10-23 — OpenAPI multi-file split, step 1 (tooling)

- **Scope:** HTTP contract tooling only, no contract change (still 1.1.3): `check_openapi.py` gains a bundler and a drift check; `contracts/http/openapi.bundled.yaml` is generated and committed; README and CHANGELOG describe the root + `modules/<module>.yaml` layout with one owner per module (root: Esquadro; accounts/routing/profiles/audit/presets: Compasso).
- **Revision:** `561b965` (branch `s/v1-contracts-wave2--contracts`, owner Compasso). Integrated into `main` as merge commit `0cd75d7` (local, no push).
- **Gates (exact SHA `561b965`):** Software Architect (Esquadro): APPROVED. Code Review (Crivo): APPROVED — check OK (37 ops / 161 schemas / 5 examples), bundle parses equal to the root, synthetic module tests and negatives caught.
- **Known limitations:** the drift check compares YAML bytes and PyYAML is not pinned yet (follow-up for DevOps); root vs bundle generated TypeScript equality was verified by the author, not by the reviewer.
- **Accepted risks:** none.
- **Follow-ups:** pin PyYAML where `make check` and CI run; web `gen:api` and the Core drift check read the bundle; contract 2.0.0 rebased on this merge.
