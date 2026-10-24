# allwin-heavy — heavy jobs on the shared dev host

> User directive (2026-10-24): the host is saturated. At most **3** heavy jobs run at once, host-wide, and every one of them
> goes through this wrapper.

The dev host is shared by every agent account (`dev2`, `dev3`, ...). Cold builds and full suites saturated it.

**Rule: heavy jobs run only through `allwin-heavy`:** `cargo build/test/clippy`, `dotnet build/test`, `pnpm build`,
full `vitest`/`pnpm test`, `playwright`, the e2e stack. Small targeted checks (`cargo fmt --check`, one small unit-test
file, `tsc` on one file) may run directly. While you wait for a slot, work on code, specs or review; do not poll.
No performance or latency measurement until the SRE says the host is quiet.

```bash
infra/scripts/allwin-heavy cargo test --workspace
infra/scripts/allwin-heavy cargo clippy --all-targets -- -D warnings
infra/scripts/allwin-heavy pnpm test
```

## What it does

1. **Semaphore:** 3 slots (`ALLWIN_HEAVY_SLOTS`) (`flock` on `slot.1`, `slot.2`) in `/srv/www/allwin.worktrees/.heavy-locks`
   (group `devs`, setgid, mode 2770, files 0660), shared by all accounts. It waits (polling every 5 s, up to
   30 min, then exit 75) when both are taken. The slot is held until the command ends, even if the wrapper is killed.
2. **systemd scope** (`systemd-run --user --scope`, no root): `CPUQuota=200%`, `CPUWeight=20`,
   `MemoryHigh=8G` (throttles, never kills), `--nice=10`, and `ionice -c2 -n7`. The command runs in the foreground:
   cwd, environment, tty and exit code pass through. Without a user bus it falls back to `nice -n 10` only
   and says so.
3. **Defaults** (override by exporting first): `CARGO_BUILD_JOBS=2`, `RUST_TEST_THREADS=2`,
   `CARGO_TARGET_DIR=$HOME/.cache/allwin/cargo-target/wt-<worktree>-<hash>` (one per worktree), `PLAYWRIGHT_PORT` = a free port in 3200-3899 chosen per
   worktree, and `PLAYWRIGHT_BASE_URL=http://localhost:$PLAYWRIGHT_PORT` (see "Playwright ports").
4. **Shared toolchain and compile cache** (see below): `RUSTUP_HOME`/`CARGO_HOME` under `/opt/rust` and `RUSTC_WRAPPER=sccache`.
5. **dotnet:** `MSBUILDDISABLENODEREUSE=1` and no MSBuild server, so nothing stays resident.
6. **`--fresh-target`** (or `ALLWIN_HEAVY_FRESH_TARGET=1`): see "QA verdict runs" below.

Do not leave background processes behind (a dev server, a container client): they inherit the slot and hold it
until they exit. To see who holds a slot: `fuser -v /srv/www/allwin.worktrees/.heavy-locks/slot.*`.

Settings: `ALLWIN_HEAVY_SLOTS`, `ALLWIN_HEAVY_WAIT_S`, `ALLWIN_HEAVY_CPU_QUOTA`, `ALLWIN_HEAVY_MEMORY_HIGH`,
`ALLWIN_HEAVY_LOCK_DIR`, `ALLWIN_HEAVY_TARGET_ROOT`, `ALLWIN_HEAVY_KEEP_TARGET`.

Limit: Docker containers (PostgreSQL, the e2e stack) run under `dockerd`, so the quota caps the client and build
side only.

A test that times out only while the host is loaded is not a finding until it passes or fails alone.

## One cargo target directory per worktree

Our own crates have the same relative layout in every worktree, hence the same metadata hash: **a target directory shared
by two worktrees mixes their artifacts** (a clippy run compiled against another worktree's crate; a test can even pass
against the wrong code). Never share a target between worktrees.

`allwin-heavy` therefore defaults to `CARGO_TARGET_DIR=$HOME/.cache/allwin/cargo-target/wt-<worktree>-<hash of its path>`:
one per worktree and account, outside the repository, no sudo. An exported `CARGO_TARGET_DIR` wins (for example
`CARGO_TARGET_DIR=$PWD/target`). For plain `cargo` outside the wrapper, `core/target` (the default) is already per worktree.

- Third-party crates are rebuilt once per worktree; that is the price of isolation until the compile cache is enabled.
- Cargo's build-directory lock serializes concurrent builds inside one worktree's target.
- Nothing in the repo depends on `core/target`. Delete a worktree's target with its worktree
  (`rm -rf "$HOME"/.cache/allwin/cargo-target/wt-<name>-*`), and everything when the toolchain changes.
- The uplifted binaries (`target/debug/allwin-core`, examples) belong to that worktree's last build only.

## QA verdict runs: fresh target per SHA

Verification runs in a disposable worktree at an exact SHA must not share artifacts with anything else:

```bash
cd /srv/www/allwin.worktrees/qa-<slice>-<sha7>-<agent>
infra/scripts/allwin-heavy --fresh-target cargo test --workspace
```

`--fresh-target` sets `CARGO_TARGET_DIR=$HOME/.cache/allwin/cargo-target/verify-<sha12>[-dirty]-<pid>`, runs the
command (the slot stays held), and removes that directory when the command ends, whatever its exit code.
Several commands on the same SHA that should reuse the build: set `ALLWIN_HEAVY_KEEP_TARGET=1` and delete
`verify-*` yourself afterwards (`rm -rf "$HOME"/.cache/allwin/cargo-target/verify-<sha12>*`). Without
`allwin-heavy`, use `CARGO_TARGET_DIR=$PWD/core/target` in the verification worktree.

## Playwright ports

`web/playwright.config.ts` starts the app on `PLAYWRIGHT_PORT` (default 3100) and, outside CI, **reuses a server that
is already listening there** (`reuseExistingServer`). Two worktrees on the same port can therefore test each other's
build (false green or false red). `allwin-heavy` exports a free `PLAYWRIGHT_PORT` per worktree (start from a hash of the
worktree path, first port not listening) and a matching `PLAYWRIGHT_BASE_URL`; no change to the config is needed.
Outside the wrapper: `PLAYWRIGHT_PORT=<a free port> pnpm exec playwright test`, and stop the server afterwards.
The API-level suite in `tests/e2e` uses a fresh compose project per run and is not affected.

## Parallelism flags for the other tools

The wrapper limits how many heavy jobs run, not how wide each one is. Keep each job narrow:
`dotnet test -m:1`, `pnpm exec vitest run --maxWorkers=2`, `pnpm exec playwright test --workers=2`,
`cargo test -- --test-threads=2` (`CARGO_BUILD_JOBS=2` and `RUST_TEST_THREADS=2` are already the defaults).

## Shared compile cache (sccache) — OFF by default

The cache is **disabled by default** (a first version let one account's server run another account's compiles). It
is enabled with `ALLWIN_HEAVY_SCCACHE_ENABLE=1` only after QA has verified the per-job server as `dev2` and `dev3`.
The description below is how it works when enabled.

`allwin-heavy` sets `RUSTC_WRAPPER=/opt/rust/cargo/bin/sccache` (v0.18.0, prebuilt release, checksum verified) with the
cache in `/srv/www/allwin.worktrees/.sccache` (group `devs`, setgid, 30 GB cap) and `CARGO_INCREMENTAL=0` (sccache cannot
cache incremental builds). **One sccache server per job**, on a unix socket in the account's runtime dir
(`/run/user/<uid>/allwin-sccache-<pid>.sock`, mode 700 per account): it runs inside the job's systemd scope, so its CPU
counts against the job's quota, and the wrapper stops it (and removes the socket) when the job ends. Never rely on the
default TCP port 4226: a client uses whichever server answers there, from any account, with that server's account and
cache directory. The wrapper also
points every account at the same toolchain (`RUSTUP_HOME=/opt/rust/rustup`, `CARGO_HOME=/opt/rust/cargo`, rustc 1.99.1)
and runs with `umask 002`, so cache keys match across `dev2` and `dev3` and files stay group-writable.

- **Known limit (SRE, sccache 0.18):** the cache key includes the absolute target-directory path. A build hits only when
  the same path built the same input before, so with per-worktree or per-SHA (`--fresh-target`, unique path) targets
  it does not hit across worktrees or accounts. It stays off until the targets have stable paths (a proposal: one
  target path per slot, emptied at the start of the job); do not count on it for speed.
- `ALLWIN_HEAVY_NO_SCCACHE=1` forces the cache off even when enabled;
  `allwin-heavy bash -c 'cargo build && sccache --show-stats'` shows hits and misses of that job's server (the stats
  die with it).
- Outside the wrapper nothing changes: plain `cargo` keeps your own `CARGO_HOME` and no cache.

## Quiet window (PAUSE)

For a planned performance measurement the SRE creates the file `/srv/www/allwin.worktrees/.heavy-locks/PAUSE` (its first line
is shown to waiting jobs, for example the end time). While it exists, **new** jobs wait with `paused for SRE quiet window` and
the wait does not count against `ALLWIN_HEAVY_WAIT_S`; running jobs are never killed and finish normally. A PAUSE **expires by itself**: it is ignored once the end time in its first line (`until HH:MM`) has passed, or once the file is older
than 2 hours; waiting jobs say so once. While it is in force, waiting jobs repeat the message every 5 minutes with the file's age and owner.
The 2 hours count from the file's modification time, so `touch PAUSE` (or rewriting it) restarts them. Deleting the file releases the queue. `ALLWIN_HEAVY_IGNORE_PAUSE=1` makes a job ignore the pause; only the account doing the measurement uses it.
