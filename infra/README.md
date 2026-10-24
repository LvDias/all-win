# infra/

Dev host and local stack (owner: DevOps).

- `scripts/provision-dev-host.sh`, `scripts/worktree.sh` — host provisioning and worktrees.
- `scripts/dev-stack.sh` — `up|down|logs|ps|config` for `compose/docker-compose.dev.yml`.
- `compose/` — PostgreSQL 16, Core (profile `core`, enabled once the Core image builds) and Caddy. All ports on `127.0.0.1`. Copy `.env.example` to `.env` (never committed).
- `caddy/Caddyfile.dev` — TLS-internal dev origin on `https://localhost:8443`. Masking rule R2 (contract `s1-webhook-ingest.md` §4, ADR-0001 §5): the access log never records `request>uri` or the Referer; ingress-like paths (`/ingress`, case-insensitive, decoded, `//` collapsed) are also `log_skip`ped; error/debug/reverse_proxy loggers delete `request>uri` and `request>headers`. `log_credentials` stays off. `CADDY_LOG_LEVEL=DEBUG` turns on debug output. HSTS is set for the whole origin. Re-run the M1–M10 masking test whenever this file changes.
- `docker/core.Dockerfile` — package `app` builds the binary `allwin-core`.
- `compose/postgres-init/01-roles.sh` — creates `allwin_owner/app/worker/maint/publisher` and the `allwin` database on first start (passwords from `.env`).

Log capture for QA: `scripts/dev-stack.sh logs caddy core` (container stdout; no secret reaches it).

## Deployment requirements (Core behind Caddy)

- The Core port is never published (dev compose, E2E, production). The Core trusts the last `X-Forwarded-For` hop (`INGRESS_TRUST_FORWARDED_FOR=true`), so it must be reachable only through Caddy. A debug overlay that publishes it must set `INGRESS_TRUST_FORWARDED_FOR=false` and leave `tradingview_source_ranges` empty.
- Caddy proxies only `/ingress*` (ingress-like paths) and `/api/*` (and `/adapter` when it exists) to the Core. `/metrics` and `/healthz` live on the same Core listener and must never be proxied or exposed publicly.
- Caddy never records the request URI or the Referer; `log_credentials` stays off; `resp_headers>Location` is deleted from the logs.

## Gates to wire in `make check` / CI (no Makefile or CI exists yet)

- Core: `cargo fmt --check`, `cargo clippy --workspace --all-targets --all-features -D warnings`, `ALLWIN_REQUIRE_DB_TESTS=1 cargo test --workspace` (includes the OpenAPI drift test, no database needed for it).
- sqlx offline data: against a freshly migrated PostgreSQL 16, `.sqlx` must match the queries. With sqlx-cli 0.9.0 `cargo sqlx prepare --workspace --check` fails under `SQLX_OFFLINE=true` (set in `core/.cargo/config.toml`), so run `cargo sqlx prepare --workspace -- --all-targets` and then `git diff --exit-code core/.sqlx`.
- Plugin: `dotnet test plugins/ninjatrader8 --nologo -v q`.
- Web: `pnpm check` (lint, typecheck, i18n parity, API drift, Vitest).


## Operator recovery (break-glass)

Run the Core's operator command **on the host**, through the wrapper, never directly in the container:

```bash
infra/scripts/allwin-admin-recovery [args of `allwin-core admin recovery`]
```

The wrapper forwards three variables, replacing anything inherited: `ALLWIN_OPERATOR_LOGIN` (`logname`, the login user from
utmp, not `$USER`), `ALLWIN_OPERATOR_SSH_CONNECTION` (`$SSH_CONNECTION`) and `ALLWIN_OPERATOR_SSH_KEY` (the SHA256
fingerprint of the key that authenticated, read from the file `$SSH_USER_AUTH`). The Core writes them **only to the operator
log** (tracing target `operator`, server side), never to the user-visible audit, and refuses `--break-glass` without
`ALLWIN_OPERATOR_LOGIN`. The compose file and env file default to the dev stack; for another host set
`ALLWIN_COMPOSE_FILE` and `ALLWIN_COMPOSE_ENV_FILE`.

The wrapper writes two lines to the host's syslog/journald (tag `allwin-admin-recovery`, priority `authpriv.notice`): `start` (login, SSH connection, key
fingerprint, subcommand and flag names without values) and `end` (exit code). It does **not** copy the command's output: on a terminal the container's
echo of what the operator types would travel on that stream, so a typed code could reach the log. It fails closed: without a working `logger` the
command does not run. This is a second record next to the Core's operator event, not independent proof (it exists only when the operator uses the
wrapper); keep the host's syslog retention and access under the operator's control.

**Production host (operator/user decision, documented here, not changed by this repository):** set `ExposeAuthInfo yes` in
`sshd_config` so `$SSH_USER_AUTH` exists, and `LogLevel VERBOSE`, which makes sshd log the fingerprint of the key used. That
log line, correlated by time and `change_id`, is the strong evidence; the variables above are the convenience copy.

**Runbook line:** break-glass trusts whoever has a shell on the host. It raises an alert and an e-mail notice, and the 72-hour
wait and the notices to the user still apply.

## Database roles

| Role | Login | `BYPASSRLS` | Used by |
|---|---|---|---|
| `allwin_owner` | yes | no | migrations only |
| `allwin_app` | yes | no | Core runtime (RLS enforced) |
| `allwin_worker` | yes | yes | queue loops that find work across users |
| `allwin_maint` | yes | yes | maintenance jobs (purges) |
| `allwin_publisher` | yes | **no** | the publish-release CLI only (S16, maintenance path); grants come from migrations (reviewed by the Database Architect) |

Roles are created outside the migrations; migrations only grant. Dev compose: `postgres-init/01-roles.sh` on the first
start of an empty volume. **An existing dev volume** keeps its old roles: add the new one once, as the superuser:

```bash
psql -U postgres -c "CREATE ROLE allwin_publisher LOGIN NOBYPASSRLS PASSWORD '<value of ALLWIN_PUBLISHER_PASSWORD>'" 
psql -U postgres -d allwin -c "GRANT CONNECT ON DATABASE allwin TO allwin_publisher"
```

(or recreate the volume: `infra/scripts/dev-stack.sh down && docker volume rm allwin-dev_pgdata`).

**CI and QA:** there is no role setup to change. The Core test harness (`core/crates/*/tests/support`) connects as the
superuser from `TEST_DATABASE_ADMIN_URL` (CI service container or `infra/qa-db`), creates the roles itself with
test-only passwords and one database per test binary. The fifth role is added there (Backend owns that code).

**Production** (none exists yet): create the five roles before the first migration, each with its own random password
from the secret store; `allwin_publisher` gets no `BYPASSRLS`, no superuser, and no `CREATEROLE/CREATEDB`.
