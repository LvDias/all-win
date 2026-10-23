# QA test database

Disposable PostgreSQL 16 (`postgres:16-alpine`) in Docker for QA runs of the Core PostgreSQL integration tests.
One container per QA agent and run. Nothing is kept: data lives in a tmpfs and `destroy` removes the container.

```bash
infra/qa-db/qa-db.sh create  <name> --env-file <worktree>/core/.env.test [--owner <unix-user>]
infra/qa-db/qa-db.sh reset   <name> --env-file <worktree>/core/.env.test [--owner <unix-user>]
infra/qa-db/qa-db.sh destroy <name>
infra/qa-db/qa-db.sh status
```

`<name>` is lowercase letters, digits and `-` (suggested: `<agent>-<sha7>`). Container name: `allwin-qa-db-<name>`.

## What it does

- Publishes the port on `127.0.0.1` only (random free port). No firewall or `ufw` change.
- Generates a random superuser password per environment and writes only
  `TEST_DATABASE_ADMIN_URL=...` to `--env-file` (mode 600; `--owner` hands the file to the QA agent's
  account via `sudo -n chown`). The path must be git-ignored (`core/.env.test` is). The password is never
  printed and never passed on a command line.
- The Core test harness (`core/crates/app/tests/support`) reads that URL, creates the five roles (`allwin_owner/app/worker/maint/publisher`) with
  test-only passwords, and one database per test binary from the repo migrations, run as `allwin_owner`.
  Run the tests with `ALLWIN_REQUIRE_DB_TESTS=1 cargo test` so a missing database fails instead of skipping.
- The env file is written atomically: a new mode-600 file in the target directory, ownership handed over
  on that temp file (`sudo -n chown -h --`), then renamed over the target. A symlink, a non-regular file or a
  symlinked directory is refused; inside a git worktree the path must be git-ignored and untracked.
  If `create` fails, the container and the partial file are removed.
- `reset` = `destroy` + `create` (new password, new port, new env file).

## Limits

- The password is visible to members of the `docker` group (`docker inspect`); it is a throwaway credential
  for a loopback-only database, not a secret worth rotating after use.
- Memory 1 GiB, 512 pids, data tmpfs 1 GiB, `fsync=off`: for tests only.
