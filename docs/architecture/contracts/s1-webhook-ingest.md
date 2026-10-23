# V1-S1 contract — TradingView webhook ingest

- **Status:** Ready for implementation (2026-10-23); revised after Code Review 1 (2026-10-23: processing order §2, masking rule §4). Owner: Software Architect.
- **Contract sources:**
  - [`contracts/http/openapi.yaml`](../../../contracts/http/openapi.yaml), operation `ingressTradingView`;
  - [`contracts/instruction/v1`](../../../contracts/instruction/v1/README.md) (schema + fixtures);
  - [ADR-0003](../adr/0003-instruction-contract-v1.md) §6–7;
  - [ADR-0001 §5](../adr/0001-technology-stack.md) (masking; the normative rule is §4.1 below).
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
| 1 | **Per-source-IP rate limit**, in memory, before any parsing or database access, so floods of invalid tokens are throttled too | 429 `ingress.rate_limited` + `Retry-After` | None (no trusted user yet). Operational counter by result, never by key |
| 2 | Route and method: only `POST` on the exact path `/ingress/v1/tradingview/{token}`. Token format, then `key_id` lookup, then constant-time comparison of SHA-256(secret), then key and source active. Done from the path alone, **before the body is read** | **404, empty body.** The same response for malformed, over-long, unknown, revoked or mismatched tokens, for any other path or method under `/ingress`, and for path-parameter violations; never 400/401/405 (S1 AC-2). The Core logs the `key_id` only when the token is canonical | None, since there is no trusted user. Operational counter by result, never by key |
| 3 | Per-key rate limit | 429 `ingress.rate_limited` + `Retry-After` | "rejected" + code |
| 4 | Content type allowed (`application/json`, `text/plain`); body ≤ 16 KiB (`Content-Length` checked first, then enforced while streaming). The token is already known, so these rejections are audited without a second lookup | 415 `ingress.unsupported_media_type` / 413 `ingress.payload_too_large` | "rejected" + code |
| 5 | Parse JSON (decimal fields are strings; no `f64` anywhere) | 400 `ingress.invalid_json` | "rejected" + code |
| 6 | Validate against instruction schema v1. A `schema_version` whose minor is newer than the Core supports (S1 supports `1.0`), or an unknown major → `ingress.unsupported_version` with `pointer` `/schema_version` (ADR-0003 §8) | 400 `ingress.schema_violation` / `ingress.unsupported_version` + JSON `pointer` | "rejected" + code + pointer |
| 7 | Source policy (ADR-0003 §6):<br>• `source.type` must equal `tradingview` → `ingress.source_mismatch`;<br>• actions `modify`/`cancel` or `position_effect: reduce` → `ingress.action_not_allowed_for_source`;<br>• non-market `order.type` → `ingress.order_type_not_allowed_for_source`;<br>• `signal_type` not in `trigger`/`trade_plan`/`nq_trigger` (e.g. `master_account`) → `ingress.signal_type_not_allowed_for_source`, `pointer` `/signal_type`;<br>• `instrument.symbol` not enabled in the instrument catalog → `ingress.unsupported_instrument` (AC-3, AC-7: catalog configuration, no code change to add NQ/ES/MES) | 422 + code | "rejected" + code |
| 8 | Insert the instruction with `UNIQUE (user_id, message_id)`, `received_at`, `sent_at`, `expiry_base = min(sent_at, received_at)` | Conflict → **200 `duplicate`** with the original `instruction_id`; nothing is routed (AC-4) | "duplicate ignored" |
| 9 | Commit, then respond **200 `accepted`**. Routing starts asynchronously after commit | — | "received" (AC-1) |

- **Unexpected errors:** 500 `{"status":"rejected","error":{"code":"common.internal"}}`, never with the path, token or SQL (M9).
- **Caddy body limit:** if Caddy enforces a request body limit, it must be a backstop **above** 16 KiB (e.g. 64 KiB), so the Core is the one that answers 413 and audits it.
- **No `Origin` check** on ingress (the `/api/v1` CSRF rule does not apply).

- **Scoping (AC-5):** `message_id` uniqueness is per user, so the same `message_id` from two users is processed independently. **Confirmed by the Architect** (ADR-0003 §5).
- **Latency budget:** p95 < 300 ms from the first byte at the Core to the response. TradingView aborts at about 3 s.
- **M1 seed:** until S3/S5 exist, the user, source, key and destination account come from a seed. The seed must create the key the same way S3 will (hash only), and print the token once on stdout. The token is never written into a committed file.

## 3. Audit entry shape (minimum)

`{ user_id, at, kind: "ingress.received" | "ingress.duplicate_ignored" | "ingress.rejected", source_id, key_id, message_id?, instruction_id?, code?, pointer? }`

**Never stored:** the token, the secret, or the raw request URI.

## 4. Masking rule and test specification (S1 AC-6, gate for S1 acceptance)

**Goal:** prove that no part of a source secret ever appears in any log, trace, metric, error or audit record, on any code path, including requests that only *look like* ingress requests.

### 4.1 Normative rule (ADR-0001 §5, revised 2026-10-23 after Code Review 1)

- **Ingress-like request:** any request whose path, compared case-insensitively after percent-decoding and collapsing repeated `/`, starts with `/ingress`. Examples: `//ingress/…`, `/Ingress/…`, `/ingress/v1/TradingView/…`, `/ingress/v1/tradingview//awk_…`, `/%69ngress/…`, tokens with `%5F` or `%2B` inside, a bare secret without `awk_`, a token in the query string.
- **R1.** For an ingress-like request, nothing after `/ingress` (path remainder and query string) is written to any log, trace, metric label or error. The only identifying value allowed is the `key_id`, and only when the Core parsed it from a canonical token (`^awk_[A-Za-z0-9]{12}_[A-Za-z0-9_-]{43}$`).
- **R2 (every request).** Caddy access logs never contain the request URI (path or query) of **any** request, so a token pasted anywhere, even split by separators into short pieces, cannot be logged. Method, host, status, size and duration are kept. The URI-bearing request logs are written by the Core with route templates only (`MatchedPath`, `<unmatched>`). The Next.js server does not log request URLs. (A length-based catch-all regexp was rejected in review: separators can split a secret into pieces it does not catch.)
- **R3.** Caddy error, debug and reverse-proxy log entries never contain the request URI or request headers, for any path.

### 4.2 Implementation requirements

- **Caddy** (DevOps; the exact directives are verified against the installed Caddy version, and M6 below is the gate, not the config text):
  - access log: `log_skip` for every ingress-like request. Recommended matcher: `@ingress path_regexp (?i)^/+ingress` (Caddy matches it against the decoded path), combined with `path /ingress*`; DevOps confirms both decode and case behavior with the M6 variants;
  - access log, all requests: a `filter` encoder that deletes `request>uri` and `request>headers>Referer` (R2). `log_skip` for ingress stays as a second layer;
  - error/debug/reverse-proxy loggers: a `filter` encoder that deletes `request>uri` and `request>headers` (R3).
  - The earlier rule (regexp keeping `awk_<id>` on the canonical path only) is withdrawn: it leaked the secret on near-miss URIs.
- **Core** (Backend):
  - The ingress route is the only route under `/ingress`. Every other ingress-like request (other case, double slash, other method, extra segments) falls to a handler that answers the empty 404 and logs only `route="<unmatched-ingress>"`.
  - The Core writes the ingress request log that replaces Caddy's: method, route template, status, latency, client IP and, for a canonical token only, `key_id`.
  - The extractor wraps the token in a `Secret` type whose `Debug`/`Display` print `awk_<key_id>_REDACTED` when the token is canonical and `REDACTED` otherwise.
  - HTTP tracing uses `MatchedPath`; unmatched requests record `<unmatched>`, never the raw URI. No middleware logs the raw URI or query.
  - The panic hook does not print request data.

### 4.3 Test

**Canary:** the test creates a source whose secret is a recognizable random string (43 base64url characters, freshly generated per run). The `key_id` is allowed to appear.

**Search terms, for every scenario:** the full secret **and every 16-character window of it** (28 sliding windows), searched as raw text. Splits introduced by the test (M6-g) are placed so both pieces are at least 16 characters long; for M6-o the pieces themselves are added as search terms.

| # | Scenario | Requests | Where to search | Pass condition |
|---|---|---|---|---|
| M1 | Accepted instruction | Valid token + valid body | Caddy access log, Caddy error/debug log, Core stdout/stderr at log level `trace` (all targets), Core panic hook output, audit rows, metrics endpoint output | No search term anywhere; audit has `key_id` |
| M2 | Duplicate | Same body again | Same as M1 | Absent |
| M3 | Schema rejection | Valid token, body with `quantity: 0` | Same, plus the HTTP response body | Absent; the response contains only `status/error/code/pointer` |
| M4 | Policy rejection | Valid token, `action: modify` | Same | Absent |
| M5 | Revoked token | Revoke the source, then post | Same; the response is a 404 with an empty body | Absent |
| M6 | Malformed and near-miss requests (each gets the empty 404) | a. `awk_<key_id>_<secret>X`; b. truncated secret (first 30 characters); c. `/ingress/v1/tradingview//awk_…`; d. `//ingress/v1/tradingview/awk_…`; e. `/Ingress/v1/TradingView/awk_…` and `/INGRESS/V1/TRADINGVIEW/awk_…`; f. `awk_<key_id>%5F<secret>`; g. `awk_<key_id>_<secret[0:20]>%2B<secret[20:]>`; h. one secret character percent-encoded (`%41` for `A`, etc.); i. bare secret `/ingress/v1/tradingview/<secret>`; j. token in the query `/ingress/v1/tradingview?t=awk_…`; k. `/%69ngress/v1/tradingview/awk_…`; l. trailing slash `awk_…/`; m. `GET` on the canonical URL; n. token in a non-ingress path `/api/v1/awk_<key_id>_<secret>` (R2); o. secret split by separators in non-ingress paths: `/x/awk_<key_id>/<secret[0:20]>/<secret[20:]>`, `/pt-BR/<secret[0:10]>.<secret[10:20]>/<secret[20:]>?q=<secret>` (R2; here each piece of ≥ 8 characters is also searched) | Same as M1 | Absent for every variant |
| M7 | Oversized / wrong content type | 20 KiB body; `Content-Type: application/xml` | Caddy + Core logs | Absent |
| M8 | Rate limit | Burst until 429, with a valid token and with invalid tokens | Caddy + Core logs | Absent |
| M9 | Core error path | Force a database error, e.g. by stopping PostgreSQL in the test environment; also stop the Core so Caddy answers 502 | Core logs, panic output, Caddy error log, 5xx response | Absent; the 5xx body has no path |
| M10 | Tracing spans | Any request with tracing at `trace` | Span fields (`http.route` must be `/ingress/v1/tradingview/{token}` or `<unmatched>`, never the raw URI) | Absent |

**The test runs in CI** against the composed stack (Caddy + Core + PostgreSQL), and again whenever logging configuration changes. QA owns it; it lives in `tests/e2e/`. DevOps provides log capture from the containers.

## 5. Other acceptance checks (beyond masking)

- AC-1–AC-5 and AC-7 are each covered by an integration test using the fixtures in `contracts/instruction/v1/examples/`:
  - every valid fixture is accepted, with two exceptions: non-TradingView sources get 422 `ingress.source_mismatch` (the token is bound to `tradingview`), and `future-minor-version.json` (`1.3`) gets 400 `ingress.unsupported_version` while the Core supports only `1.0`. The schema-conformance tests in the contracts crate still accept it, because it is valid against the schema;
  - every invalid fixture gets 400;
  - policy cases get 422.
- **Concurrency:** 20 parallel posts of the same body give exactly 1 `accepted` and 19 `duplicate`, and exactly one instruction row.
- **Cross-user:** user A's token can never create an instruction for user B. The same `message_id` used by A and B gives two instructions.
