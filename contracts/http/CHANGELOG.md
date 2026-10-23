# HTTP API — changelog

## 1.0.0 — 2026-10-23 (ready for Wave 1 implementation)
- S1: `POST /ingress/v1/tradingview/{token}`.
- S3: auth (login, TOTP/recovery verify, logout, step-up, invitations lookup/accept, password reset), me (profile with time_zone, security, TOTP setup/confirm/disable, recovery codes, password), admin (invitations, users, reset link, TOTP reset), sources, connections (create, rename, pairing code / re-pair, unpair), plugin pairing `POST /adapter/v1/pairings`, SSE `/api/v1/events`.
- S5 (requested by design; Q24/Q25/Q26): `PUT /api/v1/accounts/{account_id}/opt-ins/{kind}` (`automation` | `copy`) with `rule_version_id`, `accounts.rule_version_changed`, `accounts.opt_in_not_applicable`; schemas `OptInChoices` (atomic create-with-choice for S5, `accounts.opt_in_required`), `OptInState` (`not_required` | `chosen` | `pending_review`), `FirmStance`.
