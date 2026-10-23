# infra/

Dev host and local stack (owner: DevOps).

- `scripts/provision-dev-host.sh`, `scripts/worktree.sh` — host provisioning and worktrees.
- `scripts/dev-stack.sh` — `up|down|logs|ps|config` for `compose/docker-compose.dev.yml`.
- `compose/` — PostgreSQL 16, Core (profile `core`, enabled once the Core image builds) and Caddy. All ports on `127.0.0.1`. Copy `.env.example` to `.env` (never committed).
- `caddy/Caddyfile.dev` — TLS-internal dev origin on `https://localhost:8443`. Masking rule R2 (contract `s1-webhook-ingest.md` §4, ADR-0001 §5): the access log never records `request>uri` or the Referer; ingress-like paths (`/ingress`, case-insensitive, decoded, `//` collapsed) are also `log_skip`ped; error/debug/reverse_proxy loggers delete `request>uri` and `request>headers`. `log_credentials` stays off. `CADDY_LOG_LEVEL=DEBUG` turns on debug output. HSTS is set for the whole origin. Re-run the M1–M10 masking test whenever this file changes.
- `docker/core.Dockerfile` — package `app` builds the binary `allwin-core`.
- `compose/postgres-init/01-roles.sh` — creates `allwin_owner/app/worker/maint` and the `allwin` database on first start (passwords from `.env`).

Log capture for QA: `scripts/dev-stack.sh logs caddy core` (container stdout; no secret reaches it).
