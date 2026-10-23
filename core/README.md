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
| `DATABASE_URL` | required | PostgreSQL as `allwin_app` (RLS enforced) |
| `WORKER_DATABASE_URL` | `DATABASE_URL` | `allwin_worker` (BYPASSRLS), outbox dispatcher |
| `MIGRATION_DATABASE_URL` | unset | `allwin_owner`; when set, `serve` applies migrations first |
| `CORE_BIND` | `127.0.0.1:8080` | listen address |
| `DB_MAX_CONNECTIONS` | `10` | pool size |
| `INGRESS_TRUST_FORWARDED_FOR` | `false` | client IP from the last `X-Forwarded-For` hop; `true` only when Caddy sets it and the Core is reachable only through Caddy |
| `INGRESS_IP_FLOOD_PER_MIN` | `3000` | every ingress request, per IPv4 address / IPv6 /64 |
| `INGRESS_TRADINGVIEW_FLOOD_PER_MIN` | `30000` | the same, for addresses in the TradingView ranges |
| `INGRESS_TRADINGVIEW_SOURCE_RANGES` | empty | comma-separated CIDRs of TradingView's webhook IPs (IPv4 /16 or narrower, IPv6 /32 or narrower); non-empty requires `INGRESS_TRUST_FORWARDED_FOR=true` (the Core refuses to start otherwise) |
| `INGRESS_IP_TOKEN_FAILURES_PER_MIN` | `30` | token failures per IP before the identical 429; valid tokens are never refused |
| `INGRESS_RATE_PER_KEY_PER_MIN` | `60` | per authenticated key |
| `INGRESS_UNKNOWN_KEY_CACHE_TTL_S` | `60` | negative cache of unknown `key_id`s only (0 disables) |
| `RUST_LOG` | `info` | tracing filter (JSON lines on stdout) |
| `ALLWIN_ENABLE_DEV_COMMANDS` | unset | `true` enables `seed` / `revoke-key` (dev and test only) |

Migrations must run connected as `allwin_owner` (never a superuser): default privileges only apply to objects it creates.

## Commands

```sh
allwin-core                    # serve (HTTP, outbox dispatcher)
allwin-core migrate            # apply migrations (MIGRATION_DATABASE_URL)
allwin-core seed [LABEL]       # dev: user seed-<LABEL>@example.invalid + TradingView source; prints ONE token on stdout
allwin-core revoke-key KEY_ID  # dev: revoke a source key
```

Endpoints on the Core bind: `POST /ingress/v1/tradingview/{token}`, `GET /healthz` (204), `GET /metrics`
(counters by result only). Deployment requirement: Caddy proxies only `/ingress/*` and `/api/*`, never
`/metrics` or `/healthz`, and the Core port is not published outside the Docker network. Request logs carry `http.route` (template, `<unmatched>` or
`<unmatched-ingress>`), status, latency, `client.ip` and, for a canonical token only, `key_id`.

## Checks

```sh
cargo fmt --all --check
cargo clippy -q --workspace --all-targets --all-features -- -D warnings
cargo test -q --workspace
```

Integration tests need PostgreSQL 16: set `TEST_DATABASE_ADMIN_URL` (superuser; or put it in the git-ignored
`core/.env.test`). The harness creates the four roles with test-only passwords and one database per test binary.
Without it the DB tests are skipped; `ALLWIN_REQUIRE_DB_TESTS=1` (CI) makes that a failure.
After changing a query, regenerate `.sqlx/` with `cargo sqlx prepare --workspace -- --all-targets` against a migrated database.

If rustup reports no default toolchain on the dev host, export `RUSTUP_HOME=/opt/rust/rustup CARGO_HOME=/opt/rust/cargo`.
