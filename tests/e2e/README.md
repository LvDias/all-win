# tests/e2e — cross-module E2E and contract tests

Owner: QA. API-level suite (Playwright runner, raw HTTP client) against the composed stack
`infra/compose/docker-compose.dev.yml` (Caddy + Core + PostgreSQL). Simulation and disposable data only.

## S1 — webhook ingest (`specs/s1/`)

Source: `docs/architecture/contracts/s1-webhook-ingest.md` (contract 1.1.0), S1 AC-1..AC-7.

| File | Covers |
|---|---|
| `00-selftest` | Masking scanner: 28 sixteen-character windows, M6-o pieces of 8+ characters, masked reports |
| `10-ingest` | AC-1 valid fixtures + audit "received" + `received_at`; audit details per HTTP contract 4.3.0 (`src/audit.ts`: received = action/instrument_symbol/sent_at, duplicate = original_instruction_id, rejected = error_code/pointer/body_size; rows found by `instruction_id`/key_id, since details carry no `message_id`); source policy 422; AC-7 unsupported symbol; AC-3 invalid fixtures (400 + pointer, no receipt); invalid JSON; AC-2 empty 404 matrix (unknown/wrong/short/long/malformed tokens, other `/ingress` paths, other methods) and indistinguishable responses; AC-4 duplicate (same `instruction_id`, one row, audit), 20 parallel posts → 1 accepted + 19 duplicate; AC-5 same `message_id` for two users; per-user scoping |
| `20-latency` | p95 < 300 ms smoke (client-side through Caddy: an upper bound of the contract measure) |
| `30-masking` | Capture controls; M1–M4, M6 a–o, M7, M10, R2/R3 Caddy log structure, M5, whole-run scan |
| `40-disruptive` | M8 rate limit (valid and invalid tokens; a spoofed `X-Forwarded-For` must not reset the per-IP limit); M9 PostgreSQL stopped (500 `common.internal`); M9 Core stopped (Caddy 502); final whole-run scan |

Files run in name order with one worker: later files revoke a token, exhaust rate limits and stop services.

**Search terms** (every masking scenario): the full secret of each canary token, its 28 sixteen-character
windows, and the M6-o pieces of at least 8 characters. Searched as raw text in Caddy output (access + error
loggers), Core stdout/stderr at `RUST_LOG=trace`, a full `pg_dump --data-only` (audit rows included), the
metrics endpoint (when `E2E_METRICS_PATH` is set) and every HTTP response (status, headers, body).
Reports name the term (`a:window[5..21)`) and mask every occurrence; no secret is printed.

## Running

```bash
cd tests/e2e && pnpm install
# Caddy only, with a stub upstream (before the Core exists):
E2E_MODE=edge-only pnpm -s test:s1 --reporter=dot
# Full stack:
pnpm -s test:s1 --reporter=dot   # seed/revoke via the Core's dev commands
```

The suite starts its own compose project (`allwin-e2e-<id>`) with a generated env file (random
passwords, free loopback ports, `RUST_LOG=trace`), builds the Core image, and removes containers, volumes and
its run directory at the end. QA verdicts use a clean worktree at the exact SHA (`E2E_STACK_ROOT`).
Docker calls go through `sg docker -c` when the user is not in the `docker` group.

| Variable | Default | Meaning |
|---|---|---|
| `E2E_MODE` | `full` | `edge-only`: stub upstream (`stack/compose.e2e.yml`), Core-dependent tests skipped with a reason |
| `E2E_STACK_ROOT` | this checkout | Checkout holding `infra/` (and `core/`) to test |
| `E2E_STACK` | `managed` | `external`: use a running stack described by `E2E_ENV_FILE` |
| `E2E_ENV_FILE` | generated | Compose env file |
| `E2E_KEEP_STACK` | — | `1` keeps the stack and run directory (may contain canary tokens) |
| `E2E_SEED_CMD` | `allwin-core seed` via `compose exec` | Shell command printing one token `awk_<key_id>_<secret>` on stdout. Gets `SEED_LABEL` (`a`, `b`: two distinct users) and `E2E_COMPOSE` (compose command prefix, e.g. `$E2E_COMPOSE exec -T core allwin-core seed …`). Its stdout is never printed |
| `E2E_REVOKE_CMD` | `allwin-core revoke-key` via `compose exec` | Revokes a key for M5. Gets `KEY_ID` and `E2E_COMPOSE` |
| `E2E_METRICS_PATH` | `/metrics` | Metrics path on the Core, fetched inside the compose network (the Core publishes no host port). Empty → metrics reported untested |
| `E2E_ENABLED_SYMBOLS` | `MNQ` | Symbols enabled by the seed catalog (AC-7) |
| `E2E_CADDY_LOG_LEVEL` | `DEBUG` | Caddy server/error log level in the generated env file |
| `E2E_RUST_LOG` | `trace` | Core log filter in the generated env file |
| `E2E_EXTRA_ROUTES` | — | Extra `http.route` templates allowed in M10 |
| `E2E_SQL_INSTRUCTION` / `_RECEIPT` / `_AUDIT` | `routing.instruction` / `routing.message_receipt` / `audit.audit_entry` | Relations (wave1-data-model §6) |
| `E2E_LATENCY_SAMPLES` / `_PAUSE_MS` / `_P95_MS` | 60 / 50 / 300 | Latency smoke |
| `E2E_RATE_MAX_BURST` | 600 | M8: requests before declaring "no 429" |

## Known limits

- Caddy debug output is searched when the infra has the `CADDY_LOG_LEVEL` switch (infra 7a35afb+); the suite sets it to `DEBUG` and asserts that debug lines were captured.
- The Core panic hook cannot be triggered black-box; panic output is covered only as part of Core stderr.
- AC-7 "adding a symbol needs no code change" is a configuration review item; the suite checks rejection of a disabled symbol.
- Functional tests retry a 429 after `Retry-After`; the concurrency, latency and M8 tests do not.
