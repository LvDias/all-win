# infra/

Dev host and local stack (owner: DevOps).

- `scripts/provision-dev-host.sh`, `scripts/worktree.sh` — host provisioning and worktrees.
- `scripts/dev-stack.sh` — `up|down|logs|ps|config` for `compose/docker-compose.dev.yml`.
- `compose/` — PostgreSQL 16, Core (profile `core`, enabled once the Core image builds) and Caddy. All ports on `127.0.0.1`. Copy `.env.example` to `.env` (never committed).
- `caddy/Caddyfile.dev` — TLS-internal dev origin on `https://localhost:8443`. Every logger (access and error) redacts the secret of `/ingress/v1/tradingview/awk_<key_id>_<secret>` to `awk_<key_id>_REDACTED` (contract `s1-webhook-ingest.md` §4, ADR-0001 §5). Re-run the M1–M10 masking test whenever this file changes.
- `docker/core.Dockerfile` — unverified until `core/` exists.

Log capture for QA: `scripts/dev-stack.sh logs caddy core` (container stdout; no secret reaches it).
