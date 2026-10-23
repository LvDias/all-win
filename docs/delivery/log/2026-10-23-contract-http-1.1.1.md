# 2026-10-23 — HTTP contract 1.1.1 (S1 ingress versioning and signal-type rule)

- **Merge (local, no push), by DevOps:** `main` = `17a8c2f`. Commits: `c24e1e5` docs(architecture) ADR-0003 §6 signal_type per source · `17a8c2f` feat(contracts) HTTP 1.1.1.
- **Scope:** future minor `schema_version` → 400 `ingress.unsupported_version` (pointer `/schema_version`); `signal_type: master_account` from a TradingView source → 422 `ingress.signal_type_not_allowed_for_source` (pointer `/signal_type`). OpenAPI, CHANGELOG, S1 contract notes, ADR-0003 §6.
- **Gates:** Code Reviewer APPROVED; committed content matches the approved content hashes (`fe62c418df39` contracts, `511857b53a7c` docs-architecture). Software Architect is the owner.
- **Known limitations:** web catalogs need the new error key (in the S3 web fix cycle).
- **Accepted risks:** none.
- **Follow-ups:** wave1-data-model rev. 3 integrated afterwards as `010e586` (content changed after the first approval, incl. 30-day `pairing_code` retention; Code Reviewer re-approved hash `81350bbf2eaa`). Main checkout restored clean.
