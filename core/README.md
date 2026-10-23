# All Win Core

Rust modular monolith (ADR-0001, ADR-0002). Toolchain pinned by `rust-toolchain.toml` (1.99.1).

## Layout

- `crates/app` — binary `allwin-core`: configuration, telemetry, HTTP server, wiring of module ports.
- `crates/shared-kernel` — `UserId` (UUIDv7), `Price`/`Money` (exact decimals, strings on the wire), `Clock`.
- `crates/contracts` — DTOs of `../contracts/*`, validated against the canonical JSON Schemas and fixtures.
- `crates/<module>/{enterprise,application,infra}` — business modules, created by the slice that needs them.
- `.sqlx/` — committed sqlx offline metadata (`SQLX_OFFLINE=true` via `.cargo/config.toml`).

Shared dependency versions live in `[workspace.dependencies]`; members inherit them with `<dep>.workspace = true`.
Workspace lints deny `unwrap`/`expect`/`panic`/float arithmetic outside tests.

## Configuration

| Variable | Default | Meaning |
|---|---|---|
| `DATABASE_URL` | required | PostgreSQL connection string |
| `CORE_BIND` | `127.0.0.1:8080` | listen address |
| `DB_MAX_CONNECTIONS` | `10` | pool size |
| `RUST_LOG` | `info` | tracing filter (JSON lines on stdout) |

## Checks

```sh
cargo fmt --all --check
cargo clippy -q --workspace --all-targets --all-features -- -D warnings
cargo test -q --workspace
```

If rustup reports no default toolchain on the dev host, export `RUSTUP_HOME=/opt/rust/rustup CARGO_HOME=/opt/rust/cargo`.
