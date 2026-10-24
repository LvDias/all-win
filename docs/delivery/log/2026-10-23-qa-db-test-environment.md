# 2026-10-23 — QA test database environment (infra/qa-db)

- **Scope:** user decision (2026-10-23): isolated Docker PostgreSQL 16 test environment for QA, reused by every slice. `infra/qa-db/qa-db.sh create|reset|destroy|status <name> --env-file PATH [--owner USER]` + README. One container per QA/run, port published on 127.0.0.1 only, data on tmpfs, random password written only to a mode-600 env file (never argv/logs), no firewall change.
- **Revision:** `s/v1-qa-db--infra` 3c37f2b (first review 6c5520c, diff 4b61bed6bf52; fix delta ae77dc21ac72); integrated in `main` 33c57b8.
- **Gates:**
  - Code Review (Esmeril): CHANGES REQUIRED on 6c5520c (1 high, 2 medium, 3 low) → APPROVED on 3c37f2b.
  - QA [Security] (Guarita): APPROVED 6c5520c (2 low) → re-check APPROVED 3c37f2b (0 critical/high/medium, 1 low, 1 info).
- **Known limitations:** the password is visible to docker-group members via `docker inspect` (accepted: docker group is root-equivalent and all its members are sudoers); a narrow temp-file race remains in group-writable directories (all devs members are sudoers).
- **Accepted risks:** none beyond the documented limitations.
- **Follow-ups:** `26e64c3` (relative --env-file resolved to absolute; new dirs 711) awaiting Code Review + QA Security; optional fd-based write.
- **First uses:** S1 Core PostgreSQL integration tests on 196a30a (79/0/1); Core baseline on main (64/0/1).
