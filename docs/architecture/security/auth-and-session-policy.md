# Authentication, session and credential policy (V1)

- **Status:** Accepted (2026-10-23): the user accepted Q21 option (a), D10, and the related Q19/Q20/Q22 recommendations. Delta at acceptance: low-entropy codes stored with HMAC-SHA-256 + pepper (ALIGN-4). Later delta (2026-10-23, design request): step-up list extended (automation opt-in, admin actions); invitation and reset links carry the token in the URL fragment (§3).
- **Owner:** Software Architect
- **Answers:** slice-plan §7 **Q21** (session length), design follow-ups **D10** (password policy) and the open Architect items of [v1-s3 spec](../../design/specs/v1-s3-access-sources-pairing.md) (does reset end sessions, invite/reset link storage and validity). **D5** (pairing code) is answered in [ADR-0004 §8](../adr/0004-adapter-protocol-v1.md).
- **Context:** [ADR-0001 §9](../adr/0001-technology-stack.md). The Core is the only auth authority, with server-side sessions in PostgreSQL. Registration is invite-only (Q6), TOTP is optional (A5), and recovery codes follow Q22 (a).

## 1. Sessions (Q21, user decision)

**Mechanics (decided by the architecture):**

- **Cookie:** `__Host-aw_session` with `HttpOnly; Secure; SameSite=Lax; Path=/`, and no `Domain`. The value is a 256-bit random id; PostgreSQL stores only its SHA-256.
- **Rotation:** a new session id is issued at login, after TOTP verification, and after any password change. The pending-2FA state (password accepted, TOTP not yet verified) lasts 5 minutes and grants nothing else.
- **Idle timeout:** sliding, refreshed only by **user-initiated requests**. The SSE event stream and background polling **do not** extend it, so an unattended open tab does expire.
- **Absolute timeout:** counted from login and never extended.
- **On expiry:** redirect to login with `?reason=session_expired` (design S3-01).
- **Step-up re-authentication:** password, plus TOTP when enabled, is required if the last authentication is older than **15 minutes**, before any of these sensitive actions:
  - creating or rotating a source token;
  - creating a pairing code or unpairing;
  - enabling or disabling TOTP and regenerating recovery codes;
  - changing the password;
  - turning automatic enforcement **off** on an account;
  - raising an account's maximum position size;
  - accepting an opt-in on an account (`automation` or `copy`, choice `accepted`; S5 and S7/S19). Declining needs no step-up;
  - admin actions: generating a password-reset link and resetting a user's TOTP.
- **"Sign out" vs. "sign out everywhere":** logout deletes the current session server-side. Ending every session is triggered automatically by the events in §3. A user-facing "sign out of all devices" button is a PO item, not part of S3.

**Values: options for the user**

| Option | Without "keep me signed in" | With "keep me signed in" | Trade-off |
|---|---|---|---|
| **(a) Trading-day** (recommended) | idle **12 h**, absolute **24 h** | idle **7 days**, absolute **30 days** | One login per trading day; a lost or stolen unlocked device is the main risk, mitigated by optional TOTP, step-up for sensitive actions, and remote session end via password reset |
| (b) Strict | idle 30 min, absolute 12 h | not offered | Safest, but a user who only watches the dashboard during a session gets logged out mid-trade (SSE does not count as activity) |
| (c) Long | idle 7 days, absolute 30 days | idle 30 days, absolute 90 days | Convenient; weak for an app that controls trading accounts |

**Decided: (a)** (Q21, 2026-10-23). The design text "Sessões inativas expiram após {idle}" reads "12 horas" or "7 dias".

## 2. Passwords (D10)

| Rule | Decision | Rationale |
|---|---|---|
| Length | **Minimum 12, maximum 128 characters**, counted after Unicode NFKC normalization; all characters allowed, including spaces | NIST SP 800-63B; supports passphrases (design hint) |
| Composition rules | **None** (no forced digits or symbols) | NIST 800-63B: composition rules reduce real strength |
| Breached/common check | **Rejected** if the password is in a bundled list of the most common breached passwords (top 100 000, refreshed at release time), or if it equals or contains the user's e-mail local part or "allwin" (case-insensitive) → `auth.password_breached` | No outbound call or dependency at login; an online HIBP k-anonymity check can be added later if wanted |
| Rotation | **No periodic expiry**. A change is forced only on evidence of compromise | NIST 800-63B |
| Hashing | **argon2id**, at minimum m = 19 MiB, t = 2, p = 1 (OWASP baseline). Tuned on the production VM to about 200–300 ms per hash; parameters stored in the PHC string so they can be upgraded on next login | ADR-0001 §8 |
| Confirmation field | None. A show/hide toggle instead (design S3-03) | Design decision, compatible with the policy |
| Paste / password managers | Allowed (`autocomplete` attributes as designed) | WCAG 3.3.8, NIST |

**Login abuse controls:**

- An unknown e-mail runs a dummy argon2id verification, so timing and message are identical for every failure (design AC-5).
- Throttling:
  - per IP: 10 attempts per minute;
  - per (account, IP): after 5 consecutive failures, a progressive delay (1 s, 2 s, 4 s … capped at 60 s);
  - per account: after 20 failures per hour, a soft lock of that account **for unknown IPs** for 15 minutes, so an attacker cannot lock the user out from their usual IP.
- Response: `auth.rate_limited`.
- The TOTP and recovery-code endpoints get the same throttling. A TOTP time-step that has already been used is rejected (replay protection).

**TOTP details (A5, Q22 a):**

- RFC 6238, SHA-1, 6 digits, 30 s step, ±1 step tolerance.
- The secret is encrypted at rest (AES-256-GCM, key from the environment).
- 10 recovery codes of 10 Crockford Base32 characters each (50 bits), stored as **HMAC-SHA-256 with a server-side pepper** (ALIGN-4: plain SHA-256 over 50 bits can be brute-forced offline after a leak), single use, shown once. A lost device without codes goes to an admin-assisted reset after an identity check (Q22).
- Enabling, disabling or regenerating codes requires password plus a current code (step-up).

## 3. Links, tokens and session invalidation (open Architect items from the design spec)

| Item | Decision |
|---|---|
| Password reset ends all sessions? | **Yes.** All sessions of the user end, including the one that requested the reset. The success message says so (design S3-05 proposal accepted). The same applies to enabling or disabling TOTP and to changing the password: all *other* sessions end |
| Reset also revokes plugin device tokens and source tokens? | **No, not automatically.** Revoking the device token would disconnect enforcement and routing mid-session. Instead, the reset-success page links to Connections and Sources with an "unpair" / "revoke" action |
| Token transport in links | Invitation and reset links put the token in the **URL fragment** (`/{locale}/invite#t=<token>`, `/{locale}/reset-password#t=<token>`). Browsers never send fragments to servers, so tokens cannot reach Caddy, Next.js or Core logs, nor `Referer` headers. The web app reads the fragment and sends the token in a POST body (OpenAPI `lookupInvitation`, `acceptInvitation`, `confirmPasswordReset`) |
| Invitation links | Issued **by the admin only**, bound to one e-mail address, delivered by transactional e-mail or manually by the admin (Q19/Q20). 256-bit token in the URL, **stored as SHA-256**, single use, valid **7 days**, revocable by the admin. Accepting it creates the account and starts a session |
| Password reset links | Sent by transactional e-mail; when e-mail is unavailable, the admin generates the link as a fallback (Q20). 256-bit token, **stored as SHA-256**, single use, valid **30 minutes**. A new request invalidates earlier tokens. The request endpoint always answers 202 (no account enumeration) |
| Pairing codes | HMAC-SHA-256 with pepper; limits in ADR-0004 §8 |
| Logging | Session ids, invite/reset tokens, TOTP secrets, recovery codes and pairing codes are never logged. Only internal ids are logged (same masking rule as ADR-0001 §5) |

## 4. Validation expectations (S3)

- Authorization matrix and cross-user tests for every auth/session endpoint.
- Session tests:
  - idle and absolute expiry, using a fake clock;
  - SSE does not extend the idle timeout;
  - rotation at login and after TOTP verification;
  - every invalidation event ends the right sessions.
- Timing-equivalence test for login failures (statistical tolerance) and throttling tests.
- Log-redaction test: none of the secrets above appear in Core logs or audit rows.
