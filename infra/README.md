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

## Sunday stack (loopback only, simulation only)

`compose/docker-compose.stack.yml` is an overlay on the dev compose: the Core with identity configured (`APP_ENV=development`,
`APP_PUBLIC_ORIGIN=https://localhost:<CADDY_HTTPS_PORT>`, `IDENTITY_PEPPERS`, `CODE_SENDER=dev_smtp` to the local Mailpit), Mailpit (UI
`127.0.0.1:8025`, SMTP `127.0.0.1:1025`) and the web behind the same Caddy (`caddy/Caddyfile.stack`: `/api/*` and `/ingress*` to the Core,
`/adapter/*` to the Core with the WebSocket upgrade, everything else to the web). Nothing is published beyond `127.0.0.1`. The images are built
from a host-compiled binary or a host-made Next.js build through `allwin-heavy` (`scripts/build-core-image.sh`, `scripts/build-web-image.sh`);
an image built with a test feature (`dev-stubs`) gets its own tag and never goes to staging or a release.

**Deployment note (X-Forwarded-For, L-W2):** the Core's gateway trusts the **last** hop of `X-Forwarded-For`. That is correct only with
exactly **one** trusted proxy in front of the Core (this Caddy), and that proxy must **overwrite** the header with the real peer address, never
append to a value sent by the client. Caddy's `reverse_proxy` does that by default when no `trusted_proxies` is configured (checked with a real
Caddy: a forged `X-Forwarded-For: 1.2.3.4, 5.6.7.8` reached the backend as the peer address only), and the `/adapter/*` route sets it explicitly
with `header_up X-Forwarded-For {remote_host}`. Do not add a second proxy or CDN in front, and do not set `trusted_proxies`, without revisiting this.

**Mailpit authentication.** The host is shared, so Mailpit's UI and API (`127.0.0.1:8025`) need basic auth: an htpasswd file on the host (mode 600, outside
the repository, path in `ALLWIN_MAILPIT_AUTH_FILE`), mounted read-only as `MP_UI_AUTH_FILE`. SMTP (`127.0.0.1:1025`) stays unauthenticated and loopback only.
Readers use the credential through a file, never on a command line or in a message: `curl -K <credential.curlrc> http://127.0.0.1:8025/api/v1/messages`.

**Before any staging use:** pin the base images by digest (`ubuntu:24.04`, `node:24-alpine`, `caddy:2.10-alpine`, `postgres:16-alpine`, `axllent/mailpit:v1.27`); the tags are
mutable. The runtime Dockerfiles and the overlay use tags only for the loopback Sunday stack.
