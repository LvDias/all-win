# allwin-heavy — heavy jobs on the shared dev host

The dev host is shared by every agent account (`dev2`, `dev3`, ...). Cold builds and full suites saturated it.

**Rule: full suites run only through `allwin-heavy`:** `cargo test --workspace`, `cargo clippy --all-targets`,
the e2e stack, full `vitest`/`pnpm test`, full `dotnet test`. Targeted runs (`cargo test -p <crate>`,
`vitest <path>`) may run directly, with `nice -n 10`.

```bash
infra/scripts/allwin-heavy cargo test --workspace
infra/scripts/allwin-heavy cargo clippy --all-targets -- -D warnings
infra/scripts/allwin-heavy pnpm test
```

## What it does

1. **Semaphore:** 2 slots (`flock` on `slot.1`, `slot.2`) in `/srv/www/allwin.worktrees/.heavy-locks`
   (group `devs`, setgid, mode 2770, files 0660), shared by all accounts. It waits (polling every 5 s, up to
   30 min, then exit 75) when both are taken. The slot is held until the command ends, even if the wrapper is killed.
2. **systemd scope** (`systemd-run --user --scope`, no root): `CPUQuota=200%`, `CPUWeight=20`,
   `MemoryHigh=8G` (throttles, never kills), `--nice=10`, and `ionice -c2 -n7`. The command runs in the foreground:
   cwd, environment, tty and exit code pass through. Without a user bus it falls back to `nice -n 10` only
   and says so.
3. **Defaults** (override by exporting first): `CARGO_BUILD_JOBS=2`, `RUST_TEST_THREADS=2`,
   `CARGO_TARGET_DIR=$HOME/.cache/allwin/cargo-target`.
4. **`--fresh-target`** (or `ALLWIN_HEAVY_FRESH_TARGET=1`): see "QA verdict runs" below.

Do not leave background processes behind (a dev server, a container client): they inherit the slot and hold it
until they exit. To see who holds a slot: `fuser -v /srv/www/allwin.worktrees/.heavy-locks/slot.*`.

Settings: `ALLWIN_HEAVY_SLOTS`, `ALLWIN_HEAVY_WAIT_S`, `ALLWIN_HEAVY_CPU_QUOTA`, `ALLWIN_HEAVY_MEMORY_HIGH`,
`ALLWIN_HEAVY_LOCK_DIR`, `ALLWIN_HEAVY_TARGET_ROOT`, `ALLWIN_HEAVY_KEEP_TARGET`.

Limit: Docker containers (PostgreSQL, the e2e stack) run under `dockerd`, so the quota caps the client and build
side only.

A test that times out only while the host is loaded is not a finding until it passes or fails alone.

## One cargo target directory per account

Each worktree used to build its own `core/target` (63 GB in total, every dependency cold). Use one directory per
account instead; it needs no sudo and `allwin-heavy` sets it by default. For plain `cargo` runs, put this in `~/.profile`:

```bash
export CARGO_TARGET_DIR="$HOME/.cache/allwin/cargo-target"
```

- **One target per account is enough, not one per branch family.** Third-party crates (the expensive part) are keyed
  by crate, version, features and toolchain, so every worktree on the same `rust-toolchain.toml` reuses them. Our own
  crates are keyed by path and are rebuilt per worktree (cheap); artifacts from different worktrees coexist.
- The shared target is for day-to-day builds only. Our own crates have the same relative layout in every
  worktree, hence the same metadata hash, so artifacts of two snapshots can mix and produce a **false compile
  failure** (seen by QA). The uplifted top-level binaries (`target/debug/allwin-core`, examples) are also
  overwritten by the last worktree that built them.
- Cargo's build-directory lock serializes concurrent builds in the same directory ("Blocking waiting for file lock"),
  which is what we want on this host.
- Accounts do not share a target directory (separate home, no cross-account write access).
- Nothing in the repo depends on `core/target`. To trim the directory: `cargo clean -p <our-crate>`; delete the
  whole directory when the toolchain changes (`rm -rf "$HOME/.cache/allwin/cargo-target"`).
- Old per-worktree `core/target` directories can be removed once the owner no longer needs them.

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
