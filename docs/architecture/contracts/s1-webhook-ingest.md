# V1-S1 contract — TradingView webhook ingest

- **Status:** Ready for implementation (2026-10-23). Owner: Software Architect.
- **Contract sources:**
  - [`contracts/http/openapi.yaml`](../../../contracts/http/openapi.yaml), operation `ingressTradingView`;
  - [`contracts/instruction/v1`](../../../contracts/instruction/v1/README.md) (schema + fixtures);
  - [ADR-0003](../adr/0003-instruction-contract-v1.md) §6–7;
  - [ADR-0001 §5](../adr/0001-technology-stack.md) (masking).
- **Owners:**
  - Backend Rust (`core/adapters/tradingview`, `routing` intake, `identity` source-key lookup);
  - DevOps (Caddy log configuration);
  - QA (masking test and AC verification);
  - Database Architect (tables in `wave1-data-model.md`).

## 1. Endpoint

`POST /ingress/v1/tradingview/{token}`, HTTPS on port 443, IPv4 reachable.

- **Token:** `awk_<key_id>_<secret>`, matching `^awk_[A-Za-z0-9]{12}_[A-Za-z0-9_-]{43}$`.
- **Body:** the instruction JSON (schema v1), `Content-Type` `application/json` or `text/plain`, at most 16 KiB, UTF-8.

## 2. Processing order and outcomes

| Step | Check | Failure response | Audit entry (user-visible) |
|---|---|---|---|
| 1 | Body size ≤ 16 KiB; content type allowed | 413 `ingress.payload_too_large` / 415 `ingress.unsupported_media_type` | Only if the token is valid: "rejected" + code |
| 2 | Token format, then `key_id` lookup, then constant-time comparison of SHA-256(secret), then key and source active | **404, empty body.** The same response for malformed, unknown, revoked or mismatched tokens (S1 AC-2). Logged by `key_id` only when the format parses | None, since there is no trusted user. An operational counter is labelled by result, never by key |
| 3 | Rate limits: per key and per source IP | 429 `ingress.rate_limited` + `Retry-After` | "rejected" + code |
| 4 | Parse JSON (decimal fields are strings; no `f64` anywhere) | 400 `ingress.invalid_json` | "rejected" + code |
| 5 | Validate against instruction schema v1. Unsupported `schema_version` → `ingress.unsupported_version` | 400 `ingress.schema_violation` + JSON `pointer` | "rejected" + code + pointer |
| 6 | Source policy (ADR-0003 §6):<br>• `source.type` must equal `tradingview` → `ingress.source_mismatch`;<br>• actions `modify`/`cancel` or `position_effect: reduce` → `ingress.action_not_allowed_for_source`;<br>• non-market `order.type` → `ingress.order_type_not_allowed_for_source`;<br>• `instrument.symbol` not enabled in the instrument catalog → `ingress.unsupported_instrument` (AC-3, AC-7: catalog configuration, no code change to add NQ/ES/MES) | 422 + code | "rejected" + code |
| 7 | Insert the instruction with `UNIQUE (user_id, message_id)`, `received_at`, `sent_at`, `expiry_base = min(sent_at, received_at)` | Conflict → **200 `duplicate`** with the original `instruction_id`; nothing is routed (AC-4) | "duplicate ignored" |
| 8 | Commit, then respond **200 `accepted`**. Routing starts asynchronously after commit | — | "received" (AC-1) |

- **Scoping (AC-5):** `message_id` uniqueness is per user, so the same `message_id` from two users is processed independently. **Confirmed by the Architect** (ADR-0003 §5).
- **Latency budget:** p95 < 300 ms from the first byte at the Core to the response. TradingView aborts at about 3 s.
- **M1 seed:** until S3/S5 exist, the user, source, key and destination account come from a seed. The seed must create the key the same way S3 will (hash only), and print the token once on stdout. The token is never written into a committed file.

## 3. Audit entry shape (minimum)

`{ user_id, at, kind: "ingress.received" | "ingress.duplicate_ignored" | "ingress.rejected", source_id, key_id, message_id?, instruction_id?, code?, pointer? }`

**Never stored:** the token, the secret, or the raw request URI.

## 4. Masking test specification (S1 AC-6, gate for S1 acceptance)

**Goal:** prove that the secret part of a source token never appears in any log, trace, metric, error or audit record, on any code path.

**Canary:** the test creates a source whose secret is a recognizable random string (43 base64url characters, freshly generated per run). Every assertion below searches for the **secret** (and for its first 16 characters, to catch truncation). The `key_id` is allowed to appear.

| # | Scenario | Requests | Where to search | Pass condition |
|---|---|---|---|---|
| M1 | Accepted instruction | Valid token + valid body | Caddy access log, Caddy error/debug log, Core stdout/stderr at log level `trace` (all targets), Core panic hook output, audit rows, metrics endpoint output | Secret absent everywhere; audit has `key_id` |
| M2 | Duplicate | Same body again | Same as M1 | Absent |
| M3 | Schema rejection | Valid token, body with `quantity: 0` | Same, plus the HTTP response body | Absent; the response contains only `status/error/code/pointer` |
| M4 | Policy rejection | Valid token, `action: modify` | Same | Absent |
| M5 | Revoked token | Revoke the source, then post | Same; the response is a 404 with an empty body | Absent |
| M6 | Malformed token | `awk_<key_id>_<secret>X` and a truncated variant | Same | Absent (also for the near-miss strings) |
| M7 | Oversized / wrong content type | 20 KiB body; `Content-Type: application/xml` | Caddy + Core logs | Absent |
| M8 | Rate limit | Burst until 429 | Caddy + Core logs | Absent |
| M9 | Core error path | Force a database error, e.g. by stopping PostgreSQL in the test environment | Core logs, panic output, 5xx response | Absent; the 5xx body has no path |
| M10 | Tracing spans | Any request with tracing at `trace` | Span fields (`http.route` must be `/ingress/v1/tradingview/{token}`, never the raw URI) | Absent |

**Implementation requirements this test enforces:**

- **Caddy:** a `filter` log encoder with a `regexp` replace on `request>uri`:
  `(/ingress/v1/tradingview/awk_[A-Za-z0-9]+)_[A-Za-z0-9_-]+` → `${1}_REDACTED`.
  If the installed Caddy cannot filter that field, use `log_skip` for ingress paths plus a Core-side request log. Error logs must not echo the URI.
- **Core:**
  - The extractor wraps the token in a `Secret` type whose `Debug`/`Display` print `awk_<key_id>_REDACTED`.
  - HTTP tracing uses `MatchedPath`.
  - No middleware logs the raw URI.
  - The panic hook does not print request data.
- **The test runs in CI** against the composed stack (Caddy + Core + PostgreSQL), and again whenever logging configuration changes. QA owns it; it lives in `tests/e2e/`. DevOps provides log capture from the containers.

## 5. Other acceptance checks (beyond masking)

- AC-1–AC-5 and AC-7 are each covered by an integration test using the fixtures in `contracts/instruction/v1/examples/`:
  - every valid fixture is accepted (non-TradingView sources excepted, which get `ingress.source_mismatch` because the token is bound to `tradingview`);
  - every invalid fixture gets 400;
  - policy cases get 422.
- **Concurrency:** 20 parallel posts of the same body give exactly 1 `accepted` and 19 `duplicate`, and exactly one instruction row.
- **Cross-user:** user A's token can never create an instruction for user B. The same `message_id` used by A and B gives two instructions.
