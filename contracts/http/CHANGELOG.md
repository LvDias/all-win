# HTTP API — changelog

## 1.1.2 — 2026-10-23 (S1 implementation deltas)
- Ingress credential guard (s1 notes §2 step 5a, after parsing and before schema validation): a body string shaped like an All Win token (`aw[kd]_…`) → 400 `ingress.schema_violation` with an empty `pointer`, audited with the empty pointer; the body is not stored or logged.
- Ingress rate limiting (QA Security M1): the per-IP limit before auth becomes a flood cap, with a much higher cap for configured TradingView source ranges; token failures charge a per-IP failure budget (429 on failures only, valid tokens never refused); the per-key limit stays. The over-budget 429 is identical for every failure kind; unknown key_ids go into a 60-s negative cache; trusting X-Forwarded-For is a deployment requirement. Defaults in s1 notes §2 (resolves U14). IPv6 is keyed by /64.

## 1.1.1 — 2026-10-23 (clarifications for S1 implementation)
- Ingress 422 adds `ingress.signal_type_not_allowed_for_source` (ADR-0003 §6 policy had no code). Ingress 400 `ingress.unsupported_version` documented with pointer `/schema_version` for a minor newer than the Core supports.

## 1.1.0 — 2026-10-23 (Code Review 1 fixes; additive except where noted)
- Root `security: [{session: []}]`; public operations keep `security: []`; `logout` accepts any session state; 401 `auth.unauthenticated` on every session operation.
- Common errors: 400 `common.invalid_request` (+ `pointer`) on every `/api/v1` and `/adapter/v1` operation with input; 500 `common.internal` everywhere. Ingress path/method violations stay the empty 404.
- `Origin` check scoped to state-changing `/api/v1` requests; 403 `common.forbidden_origin` documented.
- Check precedence defined (`auth.forbidden` before `auth.step_up_required` on admin operations).
- `Cache-Control: no-store` declared on one-time-secret responses; `nosniff` + `no-store` on every Core response (ADR-0001 §9).
- `x-step-up: conditional` + `x-step-up-when` replaces the free-text value on `setAccountOptIn`.
- 429 on `changePassword`, `regenerateRecoveryCodes`, `confirmTotpSetup`. `stepUp` without `code` (TOTP user) → 401 `auth.invalid_code`.
- `renameSource` 409 `sources.revoked`; `renameConnection` 409 `connections.name_taken` / `connections.unpaired`.
- New response fields (required): `Source.is_automated` (derived), `Connection.status_changed_at`, `at` on every SSE event; optional `Connection.instance_id`.
- `OptInState` conditional requirements (`chosen` / `pending_review` / `not_required`); `OptInChoices` embedded as `opt_ins`. Q26 text-change trigger marked provisional.
- SSE: no event ids, no `Last-Event-ID`; refetch after reconnect.
- Ingress processing order: per-IP rate limit first; token check before reading the body. Masking rule replaced (ADR-0001 §5, S1 notes §4).

## 1.0.0 — 2026-10-23 (ready for Wave 1 implementation)
- S1: `POST /ingress/v1/tradingview/{token}`.
- S3: auth (login, TOTP/recovery verify, logout, step-up, invitations lookup/accept, password reset), me (profile with time_zone, security, TOTP setup/confirm/disable, recovery codes, password), admin (invitations, users, reset link, TOTP reset), sources, connections (create, rename, pairing code / re-pair, unpair), plugin pairing `POST /adapter/v1/pairings`, SSE `/api/v1/events`.
- S5 (requested by design; Q24/Q25/Q26): `PUT /api/v1/accounts/{account_id}/opt-ins/{kind}` (`automation` | `copy`) with `rule_version_id`, `accounts.rule_version_changed`, `accounts.opt_in_not_applicable`; schemas `OptInChoices` (atomic create-with-choice for S5, `accounts.opt_in_required`), `OptInState` (`not_required` | `chosen` | `pending_review`), `FirmStance`.
