# infra/

Dev host and local stack (owner: DevOps).

- `scripts/provision-dev-host.sh`, `scripts/worktree.sh` — host provisioning and worktrees.
- `scripts/dev-stack.sh` — `up|down|logs|ps|config` for `compose/docker-compose.dev.yml`.
- `compose/` — PostgreSQL 16, Core (profile `core`, enabled once the Core image builds) and Caddy. All ports on `127.0.0.1`. Copy `.env.example` to `.env` (never committed).
- `caddy/Caddyfile.dev` — TLS-internal dev origin on `https://localhost:8443`. Masking rule R2 (contract `s1-webhook-ingest.md` §4, ADR-0001 §5): the access log never records `request>uri` or the Referer; ingress-like paths (`/ingress`, case-insensitive, decoded, `//` collapsed) are also `log_skip`ped; error/debug/reverse_proxy loggers delete `request>uri` and `request>headers`. `log_credentials` stays off. `CADDY_LOG_LEVEL=DEBUG` turns on debug output. HSTS is set for the whole origin. Re-run the M1–M10 masking test whenever this file changes.
- `docker/core.Dockerfile` — package `app` builds the binary `allwin-core`.
- `compose/postgres-init/01-roles.sh` — creates `allwin_owner/app/worker/maint` and the `allwin` database on first start (passwords from `.env`).

Log capture for QA: `scripts/dev-stack.sh logs caddy core` (container stdout; no secret reaches it).

## Deployment requirements (Core behind Caddy)

- The Core port is never published (dev compose, E2E, production). The Core trusts the last `X-Forwarded-For` hop (`INGRESS_TRUST_FORWARDED_FOR=true`), so it must be reachable only through Caddy. A debug overlay that publishes it must set `INGRESS_TRUST_FORWARDED_FOR=false` and leave `tradingview_source_ranges` empty.
- Caddy proxies only `/ingress*` (ingress-like paths) and `/api/*` (and `/adapter` when it exists) to the Core. `/metrics` and `/healthz` live on the same Core listener and must never be proxied or exposed publicly.
- `/metrics` is never routed on the public edge (only reachable inside the compose network, e.g. `docker compose exec caddy wget -qO- http://core:<port>/metrics`).
- Configuration review rejects (or at least raises an alert for) overly broad TradingView source ranges, for example `0.0.0.0/0` or `::/0` in `INGRESS_TRADINGVIEW_SOURCE_RANGES`: the list must be the published TradingView addresses only. The Core refuses to start with a non-empty list while `INGRESS_TRUST_FORWARDED_FOR=false`.
- Caddy never records the request URI or the Referer; `log_credentials` stays off; `resp_headers>Location` is deleted from the logs.

## Gates to wire in `make check` / CI (no Makefile or CI exists yet)

- Core: `cargo fmt --check`, `cargo clippy --workspace --all-targets --all-features -D warnings`, `ALLWIN_REQUIRE_DB_TESTS=1 cargo test --workspace` (includes the OpenAPI drift test, no database needed for it).
- sqlx offline data: against a freshly migrated PostgreSQL 16, `.sqlx` must match the queries. With sqlx-cli 0.9.0 `cargo sqlx prepare --workspace --check` fails under `SQLX_OFFLINE=true` (set in `core/.cargo/config.toml`), so run `cargo sqlx prepare --workspace -- --all-targets` and then `git diff --exit-code core/.sqlx`.
- Plugin: with `CI=true`, `dotnet restore plugins/ninjatrader8 --locked-mode` before build and test (the `packages.lock.json` files are committed; NU1900 is not suppressed when `CI=true`), then `dotnet test plugins/ninjatrader8 --nologo -v q`.
- Web: `pnpm check` (lint, typecheck, i18n parity, API drift, Vitest).

## Migrations and local databases

- Migration `20261023060010` (outbox delivery attempts, "1b") sorts before `060100`-`060600`. A database that already applied the S1/S3 branch versions without it has an out-of-order history: drop and recreate it (the dev and QA PostgreSQL containers are disposable). Do not hand-edit `_sqlx_migrations` to "re-stamp" it, and do not add a compensating migration for a migration that was never released.
- The same applies whenever a not-yet-released migration changes content (its checksum is recorded): recreate the database.
- Integration tests use the harness, which creates and alters the `allwin_*` roles in the cluster it is pointed at. Point `TEST_DATABASE_ADMIN_URL` only at a throwaway instance, never at the compose PostgreSQL: running the tests there changes the role attributes and the compose logins stop working (observed on 2026-10-23).
- sqlx offline data is checked against a freshly migrated database: run the migrations as `allwin_owner`, then `SQLX_OFFLINE=false cargo sqlx prepare --workspace -- --all-targets` and `git diff --exit-code core/.sqlx`.

