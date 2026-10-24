# Authentication, session and credential policy (V1)

- **Status:** rev. 3.4 (2026-10-24): §7 Q88 — "sign out everywhere" cancels any pending user-requested change, awaiting codes or on hold (Crivo n1); §9.1 operator recovery event `identity.operator_recovery`, its ops consumer, operator log collection and host tee (Brasa N-1, Cinzel); the break-glass notice to other admins reaches none today (Crivo nit). Rev. 3.3 (2026-10-24): §6.2 Q95/Q96 step-up for clearing out of sync and restarting the trading day, Q87 (a) every route switch-on; §5.2 Q91 (a) shared-browser device cookie risk accepted; §7/§9 Q88 (a) — "sign out everywhere" and "forget the requesting device" cancel a user-requested hold; §9.1 the admin recovery operator command (S3 AC-12). Rev. 3.1 (2026-10-23): §6.1 step-up consumption across modules ([ADR-0007](../adr/0007-access-port.md), user approved); a moot M2/M3 clause removed from §6.2. Rev. 3 **Accepted** (2026-10-23). The user accepted every recommendation of rev. 2 (decisions.md, "User decisions on all open recommendations"): U16 format 1, Q28–Q36, M2, M3, proposals (i) the other-channel code on a channel change and (ii) the 24-h freeze after a change, and the Q35 set with the setting on. Rev. 2 (2026-10-23): passwordless sign-in by one-time code replaces password + TOTP (user requirement change of 2026-10-23, [decisions.md](../../delivery/decisions.md); supersedes Q21's password parts, Q22, D10, A5). Decided by the user and restated here: invite-only sign-up, e-mail and WhatsApp confirmed by code, login by a code on a channel the user chooses, code single use / 10 min / 5 wrong attempts, send limits per user and per IP, new-device e-mail, fresh code for sensitive actions, channel change confirmed on old and new channel, WhatsApp may ship after e-mail. Still open: U17 (WhatsApp supplier; the user started the Meta verification).
- **Rev. 1** (Accepted 2026-10-23): password + optional TOTP model. Its session values (Q21 a) and link/token rules carry over unless stated.
- **Owner:** Software Architect.
- **Product source:** [slice-plan](../../product/release-1/slice-plan.md) V1-S3 rev. 7 (AC-1…AC-16) and V1-S3b (WhatsApp).
- **Context:** [ADR-0001 §9](../adr/0001-technology-stack.md). The Core is the only auth authority, with server-side sessions in PostgreSQL. Data model: [wave1-data-model.md](../data/wave1-data-model.md) rev. 6 (Database Architect).

## 1. Model

- **No password exists.** Nothing is requested, stored or reset. Possession of a confirmed channel (e-mail, later WhatsApp) is the only authentication factor.
- **Channels:** e-mail is mandatory and confirmed at sign-up. WhatsApp (S3b) is a second channel, confirmed by code before it is ever used. A channel value belongs to one account only (Q32 a, Q-I1); a clash is refused without saying who owns it.
- **Flows that send a code** (each is a *challenge* with an opaque `challenge_id`):

| Purpose | Who requests | Sent to | Bound to |
|---|---|---|---|
| `signup` | invitee, with a valid invitation | the invited e-mail. WhatsApp at sign-up: **Q31 decided (a)**: sign-up is e-mail only and WhatsApp is added later as a channel change (§7, S3 AC-3, S3b AC-1) | invitation, flow |
| `login` | anyone, unauthenticated | the chosen confirmed channel of the account, if it exists | identifier, flow |
| `step_up` | signed-in user | a confirmed channel the user chooses | session, action, target |
| `channel_change` | signed-in user | the current value **and** the new value of the channel (two codes, one flow) | session, flow |

## 2. Codes

### 2.1 Format (U16) — decided: format 1

| | Format 1 (decided) | Format 2 |
|---|---|---|
| Alphabet | **Crockford Base32**: `0–9 A–Z` without `I L O U` (32 symbols) | upper + lower + digits + specials (≈94 printable ASCII) |
| Length / entropy | 8 characters → 32⁸ = 2⁴⁰ ≈ 1.1 × 10¹² | 8 characters → 94⁸ ≈ 2⁵² ≈ 6.1 × 10¹⁵ |
| Online guessing, one code (5 attempts) | 5 / 2⁴⁰ ≈ 4.5 × 10⁻¹² | ≈ 8 × 10⁻¹⁶ |
| Targeted attacker for one year at the §4 limits (20 codes/day × 5 attempts = 100 guesses/day, 36 500/year) | ≈ **3.3 × 10⁻⁸** | ≈ 6 × 10⁻¹² |
| Typing | case-insensitive, no look-alikes, works on a phone keyboard and in a WhatsApp preview | case-sensitive; specials differ across keyboard layouts and are mangled by some mail/WhatsApp clients; frequent typing errors would burn the 5 attempts |

- **Why format 1:** the code is protected **online** by three limits (5 attempts, 10 minutes, send limits), not by its entropy. At those limits format 1 already gives an attacker about 1 chance in 30 million per targeted account per year. Format 2 lowers a risk that is already negligible, and the usability cost is real: wrong attempts lock the code and push users to resend. Offline guessing does not apply: codes are stored as HMAC with a pepper kept outside the database (§2.3) and die in 10 minutes.
- **Rules (format 1):**
  - Generated with the OS CSPRNG by rejection-free index sampling over the 32 symbols.
  - Shown as `XXXX-XXXX`.
  - Input is checked and normalized before comparison:
    - more than 32 characters is rejected by the contract schema (400 `common.invalid_request`, no attempt counted, nothing revealed); within 32, anything that is not printable ASCII counts as a wrong attempt with the generic error;
    - trim, remove spaces and `-`, uppercase, then map `O→0`, `I→1`, `L→1` (Crockford decoding), so the "ambiguous" characters never cause a wrong attempt;
    - the result must be exactly 8 characters of the alphabet, otherwise it is a wrong attempt.
- **Normalization is format-specific.** The rules above (case folding, `-` and space removal, `O/I/L` mapping) apply only to format 1, which is then exactly 8 characters. Format 2 would be case-sensitive and may contain `-`, so it would only trim surrounding spaces. The contract carries `code` as a plain string (max 32 characters) and the Core normalizes it per the chosen format, so the schema and the contract do not change with U16. Tests must not depend on the format until U16 closes.

### 2.2 Lifecycle

- **Single use.** Valid for **10 minutes** from issue. After **5 wrong attempts** the code is invalidated and the user must request a new one (user decision).
- **A challenge, not a free-floating code.** Verification always takes `challenge_id` + `code`. The `challenge_id` (the sealed token of §3, at most 256 characters in the contract) is returned only to the requester, so a code is accepted only together with the challenge that produced it.
- **Resend** on the same challenge replaces that challenge's code and invalidates the previous one.
  - Wrong attempts count **per challenge, across resends**: a resend does not reset them, or each resend would grant 5 more guesses.
  - A resend on a challenge that is no longer open (consumed, invalidated or expired) does not reopen it. The answer is the same `202`, nothing is sent, and the client starts a new challenge.
- **A new challenge never invalidates or supersedes another challenge.** There is no "at most N open" rule that drops the oldest: anyone who knows a user's e-mail could then kill the code the user is about to type with a few login requests. The number of open challenges is bounded by the send limits instead (§4: at most 5 per 15 min per identifier and device class, each living 10 min). The guessing bound of §2.1 already counts every code sent, so this adds no risk.
  - **Deviation from S3 AC-5 [Assumption]** ("a new code request invalidates the previous code of the same purpose"): rejected for this reason. The PO updates AC-5.
- **Step-up and channel-change codes** are bound to the requesting session (and, for step-up, to the action and target, §6). Any other session that presents them gets the generic failure.
- **Destination check before accepting a code, per purpose:**
  - `login`, `step_up`: the destination is still a current confirmed value of the user (a code sent to an e-mail that was changed meanwhile is dead).
  - `signup`: the destination is still the invited e-mail of an open invitation.
  - `channel_change`: the code for the current value requires that value to still be current; the code for the new value requires it to still be the pending new value of the same flow.
- **Verification is serialized** per code row (`SELECT … FOR UPDATE`). A failed attempt is committed before answering, so concurrent guesses cannot exceed 5.

### 2.3 Storage, delivery, logging

- **Stored only as HMAC-SHA-256(k_code, challenge_id ‖ normalized code)**, with `pepper_version` (ALIGN-4). Comparison is constant-time.
- **Keys:** the pepper is an environment secret, never in the database, versioned. Each use gets its own key derived with HKDF-SHA-256 from the pepper: `k_code` (info `allwin/otp-code/v1`) for codes, `k_ident` (info `allwin/identifier/v1`) for the identifier counters of §3, and the pairing-code key of ADR-0004 §8. A leak of one derived value never helps with another use.
- **Rotation** never resets the limits. A code verifies with the `pepper_version` stored on its row. For identifier counters, during a rotation the Core reads both versions' counters for the current 24-h window and uses their sum, and writes only the new version. The old version is dropped after 24 h.
- **The plaintext code exists only in memory** between generation and delivery. It is never written to the outbox, the audit log, the application logs, traces or error reports.
- **Delivery** is a direct call to the e-mail (or WhatsApp) provider, made **after the commit and after the HTTP response** (a detached task). The request's response time therefore never depends on whether a code was sent (§3). If the process dies before sending, the user simply requests a resend; no retry queue holds the code.
- **Delivery failure** (provider error or rejection), without an enumeration oracle (S3 AC-7, S3b AC-5):
  - The `login` screen **always** shows the same text, whatever happened: "Se o e-mail estiver cadastrado, enviamos um código. Não chegou? Confira o spam ou peça outro em 60 s." A failed login delivery is **never** reported to the unauthenticated requester.
  - Every failed send of a registered user writes an audit row `auth.code_delivery_failed` (channel, purpose, provider error class; never the code). After the next successful sign-in the user sees it in the audit log and as a notice ("Um código de acesso não pôde ser enviado em …"). Repeated provider failures raise an admin alert.
  - `signup`, `step_up` and `channel_change` run in a context that already proves the destination (invitation-bound or signed in). There the client may read the challenge's delivery state (`pending`, `sent`, `failed`) and offer another channel: "Não foi possível enviar pelo WhatsApp. Enviar por e-mail?" The Core never switches channel on its own (S3b AC-5).
- **Delivery providers by environment** (decided by the Architect, 2026-10-23):
  - A `CodeSender` port. By default it is `DisabledSender`, which reports delivery `failed` and sends nothing. That is correct while no provider exists (e-mail Q8, WhatsApp U17): nobody can sign in.
  - **Development/test only:** an SMTP sender to a local Mailpit (dev compose). It is enabled only when `APP_ENV` is development or test **and** an explicit flag is set. The Core refuses to start with it in any other environment.
  - E2E reads codes through the Mailpit API.
  - The Core logs which `CodeSender` is active at startup (Disabled, Mailpit or the provider), so an operator sees a Mailpit enabled by mistake at once (Vigia I2).
- **Message content:**
  - the code;
  - what it is for, in the user's language ("para entrar", "para ativar a automação na conta …", "para trocar o e-mail");
  - "the All Win team will never ask you for this code";
  - for `login`, the coarse context of the request (browser/OS family, time).
  - It never contains a link that signs in by itself (no magic links: links are opened by mail scanners and previews).
- The provider necessarily sees the plaintext code. This is accepted and recorded with the supplier choice (U17 for WhatsApp; the transactional e-mail provider of Q8).

## 3. Anti-enumeration (intent of U10, now for sending and verifying codes)

- **Login request** (`identifier` + chosen channel) **always** answers `202` with a fresh `challenge_id`, whether or not the account exists or has that channel. Nothing is sent when it does not.
  - The visible response and timing are identical: delivery is detached (§2.3), and an unknown identifier still runs the same database statements up to the commit.
- **Unknown identifiers carry the same challenge state** (R10). The `challenge_id` returned to the client is a **sealed token**: `uuid ‖ identifier HMAC ‖ device class ‖ issued_at`, authenticated with an HKDF-derived key `k_challenge` (info `allwin/challenge/v1`). A registered identifier also has a code row under that `uuid`; an unknown one has none. Every resend and verification first opens the token, so the per-identifier cooldown and counters (§4) apply to resends of unknown identifiers exactly as to registered ones. A forged or altered token gets the same answer as any other failure. With no row, a resend to an unknown identifier is counted and then sends nothing. **A resend's response and counting depend only on the token and the counters, never on the code row** (M1): a closed challenge (expired, used, 5 wrong attempts) and an unknown one both count, start the cooldown and get the same body (`expires_at` = now + 10 min, `resend_after_s` = 60).
- **Verification** of a code that is wrong, expired, already used, invalidated by attempts, bound to another session, or whose challenge never had an account returns **one** error: `auth.code_invalid`. The UI always says "Código inválido ou expirado. Confira ou peça um novo." Telling "attempts exhausted" apart from "wrong" would let an attacker probe whether an account exists (an unknown challenge never reaches 5).
  - The UI tracks the attempts it made itself to suggest a resend after the 5th failure.
- **Send limits are keyed by HMAC(k_ident, normalized identifier) (§2.3) for every identifier**, existing or not (U10's mechanism). A limit therefore triggers on an unregistered e-mail exactly as on a registered one. Counting only the rows of existing users would reveal who is registered: they would hit `429` and unknown addresses never would.
- **Channel clash** (new e-mail or number already used by another account): the flow proceeds without revealing the clash. The code to the new value is not sent, and the final confirmation fails with `auth.code_invalid`. A suppressed send reports the delivery state **`sent`**, exactly like a real one, and writes no `auth.code_delivery_failed` (N1): otherwise the delivery state of §2.3 would reveal the clash. The user is never told "this address belongs to someone".
- **WhatsApp number clash** (S3b, I4): same rule as the channel clash above. Adding a number that belongs to another account never says so.
- **Sign-up** needs a valid invitation token bound to one e-mail (Q36 a), so it does not enumerate accounts.

## 4. Limits (Q33) — decided values

"Known device" = the request carries a valid device cookie of that user (§5.2). Unknown devices and known devices have **separate** per-identifier budgets. An attacker who knows the e-mail can therefore exhaust only the unknown-device budget, and the real user keeps signing in from their own browser.

| Limit | Value | When hit |
|---|---|---|
| Resend cooldown per (identifier HMAC, device class) | 60 s | `429 auth.rate_limited` + `Retry-After` |
| Sends per identifier, unknown devices (all purposes) | 5 per 15 min, 20 per 24 h | `429`; identical for registered and unregistered identifiers |
| Sends per identifier, known devices | 5 per 15 min, 20 per 24 h (own budget) | `429` |
| Sends per signed-in user (step-up, channel change) | 10 per hour | `429` |
| Sends per IP (IPv4 /32, IPv6 /64) | 10 per 15 min, 50 per 24 h | `429` |
| Sends per prefix (IPv4 /24, IPv6 /48) | 100 per hour | `429` for that prefix + alert (U11 approach: no global lock) |
| Global sends | above 300 per hour | **alert** to the admin, no lock |
| Global WhatsApp sends (S3b) | daily spend cap set with U17 | further WhatsApp sends refused, e-mail still offered + alert (§8) |
| Wrong attempts per code | 5 (user decision) | code invalidated; generic `auth.code_invalid` |
| Wrong attempts per IP | 30 per 15 min, across codes | `429` |

- Per-IP and per-prefix counters live in memory (single Core instance, ADR-0001). The per-identifier and per-user counters are durable, so a restart does not reset them. The identifier counter is keyed by the HMAC and holds no address.
- **Probability check:** 20 codes × 5 attempts = at most 100 guesses per targeted identifier per day from unknown devices, plus 100 per day from the user's own known devices (an attacker cannot use those). That gives the §2.1 figure, ≈ 3.3 × 10⁻⁸ per year.
- The limits are configuration; the values above are the V1 defaults for the user to approve.
- **Identical for unknown identifiers:** every limit that a `login` request can hit behaves the same whether or not the identifier exists. The resend cooldown is therefore keyed on the identifier HMAC and device class (one send per 60 s), not on the code row, because an unknown identifier has none. A `login` that hits the global WhatsApp cap, or chooses a channel the account does not have, still gets the normal `202` and nothing is sent. Only a registered user gets an `auth.code_delivery_failed` audit row (§2.3). These signals are reported only in authenticated or invitation-bound flows.
- **Every send counts**, including resends, in every row of the table.
- **Known residual (accepted by design, for the user to see):** an attacker who knows the e-mail can exhaust the unknown-device budget. The real user then cannot sign in from a **new** browser for up to 24 h; known devices are unaffected.

## 5. Sessions and devices

### 5.1 Session (Q29)

- **Values (Q21 a, carried over; Q29 a decided):** idle 12 h / absolute 24 h; "keep me signed in" idle 7 days / absolute 30 days.
- **Mechanics (rev. 1, unchanged):**
  - `__Host-aw_session` cookie, `HttpOnly; Secure; SameSite=Lax; Path=/`, 256-bit random id stored as SHA-256;
  - the idle timeout is refreshed only by user-initiated requests (SSE and polling do not extend it);
  - on expiry, redirect to sign-in with `?reason=session_expired`.
- **Rotation:** a new session id is issued at login and after a channel change.
- There is no "pending 2FA" state any more.

### 5.2 Device and "new device"

- **Device cookie:** `__Host-aw_device`, `HttpOnly; Secure; SameSite=Lax; Path=/`, 256-bit random, stored as SHA-256 in `known_device`.
  - It is renewed on use.
  - Max-Age is 400 days (the browser cap). The row is deleted when unseen for 13 months (Lastro's proposal, accepted).
- **The device cookie is set when sign-up completes** (the sign-up creates the first session and its device) and at every sign-in from a browser without one.
- **Shared browser** (C4): only an unknown or revoked cookie value is replaced. A value valid for **another** user is kept, and this user gets their own `known_device` row with the same value, so each user of a shared browser keeps it as a known device.
- **A new device** is a successful sign-in whose request carries no valid device cookie **of that user**. Only the sign-up session itself sends no notice (AC-9); any later sign-in from another browser, including the first one after sign-up, is a new device.
- **"Forget this device"** (user action in Sessions/Devices) revokes the device and ends the sessions bound to it (`end_reason = device_revoked`).
- **Shared browser, forget device** (Crivo N2): if user A forgets the device while user B still has a valid row with the same cookie value, A's next sign-in sets a new value. Then B's next sign-in from that browser counts as a new device, and B gets a notice. This is accepted: it errs towards notifying.
- **Accepted risk, shared browser (Vigia I11, Q91 a, user decision 2026-10-24):** a previous user of the same browser who also obtains a sign-in code for this account signs in as a known device, so without the new-device notice and without the M3 new-device rules. Accepted because the attack already requires access to the user's e-mail (the code); "forget this device" on a shared computer removes it.
- **A revoked device's cookie counts as no cookie** (R11). The next sign-in from that browser is a new device: it gets a **new** random device value and a new `known_device` row; the revoked row is never reactivated. The same applies after "sign out everywhere". The response clears or replaces the stale cookie.

### 5.3 New-device notice (Q34) — decided content

- Sent **by e-mail** to the account e-mail (also when the sign-in used WhatsApp, S3b AC-4), in the user's language and time zone, through the outbox (it holds no secret). Contents:
  - date and time;
  - browser and OS family, from the User-Agent (coarse, e.g. "Chrome on Windows");
  - the channel used ("código enviado por e-mail");
  - the IP address. **No geolocation in V1**: it needs a GeoIP database, a new dependency with its own license and updates, and it is often wrong for mobile networks. It can be added later by decision.
  - **"Não foi você?"** with what to do: sign in, open *Sessões*, use **"Sair de todos os dispositivos"**, then review and revoke **source tokens** and **NT8 pairings** you do not recognize (an attacker's persistent access survives the end of their session), and check the e-mail account's own security.
- **No one-click action link** in the e-mail. An unauthenticated "end all sessions" link would let anyone holding the e-mail sign the user out (the e-mail can be forwarded or previewed). An attacker who controls the inbox can sign in anyway, so the link adds nothing against them.
- **"Sair de todos os dispositivos"** therefore **enters S3** (Q34 a): it ends every session of the user, including the current one, and revokes all devices. It needs no step-up: it only reduces access. Its confirmation screen lists the source tokens and paired instances and links to their revoke/unpair actions (L3).

## 6. Step-up

### 6.1 Mechanics (Q30)

- Before a sensitive action, the client obtains a **fresh code** for that action. Q30 (a) is decided: every action asks a new code, and there is no reuse window.
  1. `POST` request step-up (channel, action, target): the Core sends a `step_up` code bound to the session, the action kind and the target id, and returns its `challenge_id`.
  2. The client sends the action request with the `challenge_id` and the `code` (HTTP contract 2.0.0). The Core verifies the code against its bound session, action and target, and consumes it atomically **before** the action is performed. When the action belongs to the `identity` module, consumption and action share one transaction. For an action of another module, the code is consumed in identity's own commit and the action runs afterwards ([ADR-0007](../adr/0007-access-port.md) §3.2). A code therefore authorizes at most one action, and nothing other than its bound action and target. An action refused after the step-up (409/422) has spent the code, and the user requests a new one.
- Missing or failed step-up → `403 auth.step_up_required` (missing) or `auth.code_invalid` (wrong). Every attempt is audited (AC-10, AC-16).
- With two confirmed channels the user may pick either (S3b AC-3), except for channel change (§7).

### 6.2 Which actions (Q35) — decided: the user's three plus the security musts

| Action | Step-up | Why |
|---|---|---|
| Accept an opt-in: automation on an account, or copy (S5, S7/S19) | **yes** (user) | Starts automated trading on an account |
| Unpair an NT8 instance | **yes** (user) | Disconnects enforcement and routing |
| Change e-mail or WhatsApp | **yes** (user), on both channels (§7) | Account takeover path |
| Create or rotate a **source token** | **must** | Gives a holder of a stolen session a persistent way to send trades into the user's accounts. It survives sign-out and session expiry, because the token is a separate credential |
| Create a **pairing code** | **must** | Pairing an attacker's NT8 instance gives persistent access to commands and account data, also surviving the session |
| **Admin** actions: issue an invitation, recovery for a lost channel (§9) | **must** | An admin session controls other users' accounts |
| Switch any route on (`route_enable`; Q87 a, 2026-10-24); change the platform binding of an account that receives automation (`account_binding_change`) | **must** (QA Security M1/L1 on HTTP 2.4.0; Q87) | Either one points trading at an account |
| Clear an account's out-of-sync state (`account_sync_clear`, target = account; HTTP 5.3.0) | **yes** (user decision Q95, 2026-10-24) | The clear rebases the expected state and resumes automated entries on that account |
| Restart today's count after an unknown trading day (`trading_day_restart`, target = account; HTTP 5.4.0, ADR-0004 §7.1) | **yes** (user decision Q96, 2026-10-24) | It re-anchors the day's risk measure and can leave unseen losses out |
| Turn enforcement **off** on an account; raise maximum position size | no (not in the accepted Q35 set) | Increases loss exposure but grants no persistent access |
| Decline an opt-in; revoke a source token; sign out (everywhere) | no | Reduce access only |

**Q35 decided (2026-10-23): the musts are enforced, setting on.** They run through a Core setting (`step_up.q35_persistence_actions`, default on; contract `x-step-up: conditional`), so a later user decision changes the setting, not the contract. The setting is **server configuration only** (environment or config file), never an admin, UI or database toggle; missing or invalid means **on** (fail-closed); its value is logged at startup.

The three musts are what separates "someone used my open browser for a while" from "someone keeps access to my trading after I sign out". Without them, step-up on the three user actions can be bypassed in practice.

**M3 — persistence step-ups from a new device** — **decided** (2026-10-23): when the session's device was first seen less than 7 days ago, the persistence actions (source token create/rotate, pairing code, opt-in accept) require the step-up code **by e-mail**. After S3b, they require codes on **both** channels. Reason: after a SIM swap, a WhatsApp-only step-up from the attacker's new browser would hand over persistent trading access. With e-mail required, the attacker also needs the inbox, and the new-device e-mail notice has already been sent. Cost: none in the e-mail-only phase (e-mail is the only channel); after S3b, one extra code on new devices only.

**Residual, accepted by the user (2026-10-23):** in the e-mail-only phase M3 adds no barrier, and during an M2 hold whoever controls the inbox can still create source tokens and pairing codes. The hold protects **ownership** of the account, not against **persistence**. Against persistence the defence is the new-device notice and the §5.3 guidance to revoke unknown tokens and pairings.

## 7. Channel change (AC-11)

- **Default:** the user enters the new value. Codes go to the **current** value and to the **new** value (one flow, two codes). The change happens only when both are confirmed within 10 minutes. Until then the current value stays in force.
- **With two confirmed channels** (after S3b) — decided (proposal i): changing one channel also requires a code from the **other** channel. Otherwise an attacker who takes over the e-mail inbox alone can confirm "old e-mail + new e-mail" and own the account permanently. The other channel turns a single compromised channel into a recoverable incident.
- **On success:**
  - every other session of the user ends (`end_reason = channel_changed`), and the current session is rotated;
  - a notice goes to the **old** value ("seu e-mail foi trocado para j***@…") and to the other channel, if any;
  - the change is audited with both masked values;
  - **no** further channel change is accepted for **24 hours** (decided, proposal ii), so a takeover cannot chain changes before the user reacts to the notice.
- **Old channel unreachable** — decided with **Q28**: the old value is **never** replaced without its own code, even when another channel is confirmed. Otherwise a single taken-over channel (a SIM swap on WhatsApp) could replace the e-mail. The change goes through the §9 hold: a code to the new value and a code from the other confirmed channel (if any) open the request, then 72 h with notices to every old value, cancellable with **"Cancelar troca"** (§9).
- **M2 — hold for changes from a new device** — **decided** (2026-10-23): in the e-mail-only phase, a channel change requested from a device first seen less than 7 days ago also takes the 72-h hold of §9, with notices and "Cancelar troca", even when both codes are confirmed. Reason: with e-mail as the only channel, someone who controls the inbox confirms both codes and owns the account permanently; the hold gives the real user 72 h to react to the new-device and change notices. Cost: a user who really changes e-mail from a new phone waits 3 days. When the hold takes effect (at its end, "on effect", as in §9), every session ends and every device is revoked: the requester may be the attacker.
- Adding WhatsApp the first time is a channel change with "current" = e-mail.
- **Other cancellations of a user-requested change (Q88 a, 2026-10-24; rev. 3.4):** this applies to every pending change the user requested, not to the admin recovery of §9.
  - **"Sign out everywhere"** cancels any pending user-requested change: one still awaiting its codes, or one on hold (the M2 hold or the "old channel unreachable" hold). Otherwise an attacker's change still awaiting codes would survive the user's reaction and could be confirmed later (Crivo n1).
  - **"Forget this device"** cancels it only when the forgotten device is the one that requested the change.
  - Both use the same path as "Cancelar troca", in the same transaction as the sign-out or the forget: the same audit row and notices to every channel.
  - A new request after such a cancellation raises an admin alert and an audit row (§9).
  - No 24-h freeze follows a cancellation; that freeze follows only a change that took effect.
  - Signing in alone still never cancels.

## 8. Channel risks and mitigations

| Risk | Effect | Mitigation |
|---|---|---|
| **E-mail account takeover** | With e-mail as the only channel, it is a full account takeover. This is not new: the old password reset by e-mail gave the same power; optional TOTP was the only extra barrier, and it is gone | New-device notice; step-up on persistence actions (§6.2); the 24-h channel-change freeze; advice in the UI to protect the e-mail with 2FA. After S3b, §7 requires the other channel for a change. **Option for the user (not required):** "sign in needs codes from both channels" for users with WhatsApp confirmed |
| **SIM swap / WhatsApp re-registration** (attacker moves the number) | Codes by WhatsApp reach the attacker | The new-device notice always goes by e-mail. With two channels, a WhatsApp takeover cannot change the e-mail (§7). The supplier's "number changed / re-registered" signal, if U17's supplier offers one, blocks WhatsApp codes until the user confirms by e-mail |
| **Number recycling** (the operator reassigns an abandoned number) | A stranger receives codes | Remind users to update the number; codes go only to confirmed numbers; the e-mail notice on new devices |
| **Real-time phishing / relay** (attacker triggers a login and talks the user into reading the code) | Takeover | Message text says what the code is for and that All Win never asks for it; the `login` message names the requesting browser. Codes are phishable by nature. Passkeys (WebAuthn) would close this and can be a later decision |
| **Lock-screen previews** of e-mail or WhatsApp | Someone near the phone reads a code | The code alone is useless without the `challenge_id`, which stays in the requesting browser (§2.2). For login the attacker must also request it from their own browser: that is the relay case above |
| **WhatsApp cost abuse** (pumping: mass code requests to numbers) | Supplier bill | Codes only to confirmed numbers or to the number typed in an invited sign-up; the §4 limits; a country allow-list (Brazil +55, extended by decision); the global daily WhatsApp cap with an alert |
| **Provider sees codes / provider outage** | Code exposure in the provider's logs; no login | Accepted with the supplier choice. If WhatsApp delivery fails, the user is told only in signed-in or invitation-bound flows (§2.3; never on the login screen) and may request an e-mail code; the Core never silently resends through another channel (S3b AC-5) |
| **Mail scanners** fetching links | Would consume magic links | There are no magic links; codes are typed |

## 9. Loss of every channel (Q28) — decided: (a) admin-assisted recovery

- **Decided (Q28 a, 2026-10-23):** admin-assisted recovery (AC-12 unblocked).
  - The admin verifies identity out of band (an agreed procedure, not just a message from the new address). The admin action requires step-up.
  - The admin sets a new e-mail; a code to it confirms it.
  - The change takes effect after a **72-hour hold**, during which notices go to every old channel.
  - **Cancelling** (N1) is an explicit action **"Cancelar troca"**, available from any session of the user, audited and followed by a notice to every channel. Merely signing in does not cancel, because the requester can sign in too. A new request after a cancellation raises an admin alert.
  - On effect: all sessions end and all devices are revoked. Source tokens and pairings stay, as in rev. 1 for reset, and the user is pointed to review them.
  - Everything is audited.
- **Not cancelled by "sign out everywhere" or "forget device"** (Q88): only the explicit "Cancelar troca", or the admin's cancel (§9.1), cancels an admin recovery.

### 9.1 Operator command (S3 AC-12)

There is no HTTP endpoint for admin recovery. It is a subcommand of the Core binary, `allwin-core admin recovery <start|confirm|status|cancel>`, run on the host over SSH. The use cases live in `identity`; the command is a thin adapter in `crates/app`. It runs as `allwin_app` with `app.user_id` set to the target user (the same RLS as the user's own flows), and the audit records `actor = platform_admin` with the admin who acted.

- **Admin factors.** Normal mode: SSH access to the host, plus a fresh step-up code (`admin_recovery`) sent to the admin's own confirmed channel. The code follows the normal send limits and the 5-attempt rule. It is read at an interactive prompt from a TTY with echo off, never as an argument; without a TTY the command refuses.
- **Break-glass (I3) is only for a platform admin recovering their own account when no other step-up factor is available.** `--break-glass` against any other target is refused, and nothing is created.
  - **Now:** break-glass is allowed only when no other active platform admin exists. If one exists, that admin runs the normal recovery with step-up.
  - **After S3b (WhatsApp):** an admin with a confirmed WhatsApp gets the step-up code there instead of break-glass. The path for an admin who lost both channels is **pending a user decision at S3b**.
  - It is SSH only. The user-visible audit records `break_glass = true` on the `security.channel_change_*` entries and nothing about the host. In break-glass the admin is the target, so the admin's trail is the trail the account owner sees, and the operator's SSH address must not appear there.
  - **Who ran it** is recorded only in the operator log (server tracing). A host wrapper (DevOps) passes `logname`, `SSH_CONNECTION` and the SSH key fingerprint to the command; the fingerprint is available only when sshd exposes it (`ExposeAuthInfo`). These values are marked host-reported and validated against their expected format. The strong evidence is the sshd log at `LogLevel VERBOSE`, correlated by time and change id. Any sshd change is made by DevOps, with the user's authorization.
  - Every break-glass `start`, `confirm` and `cancel` that commits raises an operational alert. At `start`, in the same transaction, a notice (pt-BR/en, no links) goes to every other active platform admin (none today, since break-glass runs only when no other active admin exists) and to the target's current channels: "Uma recuperação de emergência (break-glass) da conta All Win de <e-mail mascarado> foi iniciada no servidor em <data>. Se não foi você, fale com o suporte e use 'Cancelar troca' em qualquer sessão."
  - The 72-h hold and the notices are the protection.
- **Operational event and alert (rev. 3.4; Brasa N-1, Cinzel).** Every operator action writes one outbox event `identity.operator_recovery` in the same transaction as its audit event, for `start`, `confirm` and `cancel`, in normal and break-glass mode. Refusals, failed step-ups and confirms that ran out of attempts write it too; when no change was created they write it in a transaction of their own. `user_id` is the target. A refused `start` whose target user does not exist has no target: the event then carries the acting admin's own `user_id` and `target_found = false`, never the value typed for `--user` (it may be contact data); the operator log and the host tee keep the details. The host-reported fields are marked as such and validated against their format; no code, token or full contact value is in the payload.
  - **Consumer:** a registered ops consumer, shipped with AC-12 (never on the `FromNow` allowlist). It logs normal actions at WARN, and break-glass actions at ERROR with `security_alert`, plus an e-mail to the operator. The audit consumer does not record this event type; the user-visible audit stays as above.
  - **Operator log:** the operator log is collected with the server logs and readable only by the operator; never at a debug-only level; kept at least as long as the audit entries it explains (Vigia I2).
  - **Host tee (backup, DevOps):** the host wrapper also copies the command's operator-log lines to the host syslog, so a failure of the outbox or its consumer still leaves a trace.
  - SSH access to production is held only by the operator (the user), never by agents or CI.
- **Step-up binding.** At `start` the code is bound to the admin, the action and the target user; the change id is created in the transaction that consumes it. `confirm` and `cancel` each need a new step-up bound to the change id. One `admin_recovery` grant can serve either `confirm` or `cancel` on that change; this is accepted, because cancel only reduces access and confirm also needs the user's code.
- **`start`:** `--user`, `--new-email`, `--verified-by <video_call|phone_known_number|in_person|other>` and `--verification-ref` (plain text, at most 200 characters, required and non-empty with `--verified-by other`). The reference is shown to the target user, so it never carries contact data, document numbers or anyone's personal data. The Core refuses `@`, a `+` followed by a digit, more than 6 consecutive digits and token-shaped parts; the rest is an operator rule. It creates the pending change in the same state as §7 (kind `admin_recovery`) and sends a code to the new e-mail. Output shows masked values only.
- **`confirm`:** the user reads the code from the new e-mail to the admin out of band; the admin types it at the prompt. Wrong codes count against the usual 5 attempts; the 5th gives `security.code_rate_limited` (purpose `channel_change`) and the change ends `not_completed`, with notices. Confirmation starts the 72-h hold with notices to every old channel. The user's code proves control of the new e-mail, not identity; identity rests on `--verified-by`, the 72-h hold and the notices.
- **Effect** (worker job at the end of the hold): the e-mail changes, every session ends and every device is revoked; source tokens and pairings stay. The **new** e-mail gets the full notice: recovery done, sessions ended, devices removed, review your source URLs and pairings. The **old** channels get only the plain notice ("o e-mail da conta foi substituído por n***@… por uma recuperação administrativa em <data>; não foi você? fale com o suporte"), with no count or list of tokens or pairings. No one-click links.
- **Audit:** start, confirm, cancel and effect are visible to the target user, with masked values and `verified_by`/`verification_ref`. In the target's trail the only failure is the user's code (the 5th wrong attempt: `security.code_rate_limited` and `not_completed`). The admin's step-up failures go to the admin's trail (`security.step_up_failed`). No code ever appears in logs or audit.

- (b) "start over with a new invitation" loses the trading history and configuration. (c) a third recovery contact adds a channel to secure.
- **The admin's own recovery** (I3) — decided with Q28: no admin can recover themselves through the admin flow. Either a second admin account held by the user, or a documented break-glass command run on the server by the operator (the user, over SSH), with the same 72-h hold, notices and audit. Decided: the break-glass command; a second admin would be a second credential to protect.

## 10. Invitations, tokens and logging

| Item | Decision |
|---|---|
| Invitation links | Unchanged from rev. 1: admin only, bound to one e-mail (Q36 a: the sign-up e-mail must equal it), 256-bit token in the **URL fragment**, stored as SHA-256, single use, valid 7 days, revocable. The account is created and the invitation consumed only when the e-mail code is confirmed (AC-2). An abandoned sign-up leaves the invitation usable until it expires |
| Password reset links | **Removed** (no password) |
| Pairing codes | HMAC-SHA-256 with pepper; limits in ADR-0004 §8 (U11: per-IP and per-prefix, no global lock) |
| Logging | Session ids, device ids, invitation tokens, one-time codes and pairing codes are never logged. Channel values are masked in logs (`j***@d***.com`, `+55******1234`); the user's own audit view shows them masked too. Same masking rule as ADR-0001 §5 |

## 11. Validation expectations (S3)

- Authorization matrix and cross-user tests for every auth, session, device and code endpoint (a challenge id of user A is useless to user B).
- **Code tests:**
  - 5th wrong attempt invalidates; concurrent guesses never exceed 5;
  - expiry at 10 min with a fake clock;
  - single use;
  - a resend invalidates only its own challenge;
  - step-up code bound to session + action + target;
  - a code sent to a replaced destination is dead;
  - normalization (`o`→`0`, `l`→`1`, spaces, hyphen); more than 32 characters is 400 with no attempt counted; non-ASCII and fewer than 8 after normalization are wrong attempts.
- **Enumeration tests:**
  - a resend within 60 s gets the same answer (`429` + `Retry-After`) for a registered and an unregistered identifier, and so do the per-identifier counters (R10);
  - 5 wrong attempts then a resend, and an expired challenge then a resend, give identical responses and counting for a registered and an unregistered identifier (M1);
  - `requestStepUp` on another user's target gets 404 and sends nothing;
  - a forged or altered `challenge_id` gets the generic failure;
  - a channel change (or WhatsApp add) to a value owned by another account and one to a free value show the same delivery state (`sent`) and the same responses until the final confirmation;
  - identical status, body shape and timing (statistical tolerance) for login requests on registered and unregistered identifiers;
  - identical `429` behavior at the per-identifier limit for both;
  - a single error code for every verification failure.
- **Limit tests:** each §4 row, including the separate known-device budget.
- **Session tests:**
  - idle/absolute expiry (fake clock); SSE does not extend idle;
  - rotation at login and channel change;
  - "forget device" and "sign out everywhere" end the right sessions;
  - channel change ends other sessions and freezes further changes for 24 h.
- **Hold tests (§9, §7/M2):** "Cancelar troca" from any session cancels the pending change, with an audit row and notices; signing in alone does not cancel; a new request after a cancellation raises an admin alert and an audit row.
  - Q88: "sign out everywhere" cancels a user-requested change awaiting codes and one on hold; "forget device" cancels it only for the requesting device (both cases tested); neither cancels an admin recovery.
- **Admin recovery tests (§9.1):** no admin code → nothing; a wrong admin code → nothing; break-glass → the same hold; break-glass against a target that is not a platform admin → refused, nothing created; no TTY → refused; `--verified-by other` without a reference → refused; a reference with `@`, `+digit`, more than 6 consecutive digits or a token shape → refused; a wrong or expired user code at `confirm`; cancel during the hold; a repeated request → admin alert; the effect ends sessions and revokes devices; the notice split between new and old channels; the command touches only the target user; no HTTP route exists for it. Rev. 3.4: each action, refusal and failed step-up writes exactly one `identity.operator_recovery` event; a break-glass action is logged at ERROR with `security_alert` and e-mails the operator; the audit consumer ignores the event type; the payload holds no code, token or full contact value.
- **Challenge tests:** three login requests from an unknown browser do not invalidate the user's open challenge; a resend keeps the attempt count; a resend on a closed challenge sends nothing.
- **New-device tests:** the notice fires once per new device, for the first sign-in after sign-up from another browser too, and not for the sign-up session.
- **Delivery-failure tests:** a failed login send changes nothing in the login response; it produces an `auth.code_delivery_failed` audit row and a notice after the next sign-in; on step-up the delivery state reads `failed`.
- **Audit tests (AC-16):** a user-visible audit row with its reason for each sign-in, failed verification, rate-limited request, new-device notice, delivery failure, step-up success and failure, channel change, "forget device" and "sign out everywhere". No audit row contains a code or an unmasked channel value.
- **Log-redaction test:** no code, token, session/device id or unmasked channel value in Core logs, audit rows or the outbox. It also covers the tracing spans of the detached delivery task, the provider HTTP client's debug logs (request bodies are never logged), and panic and error reports. The delivery function takes the code as a redacting wrapper type whose `Debug`/`Display` print `***`.

## Superseded by rev. 2

- §2 Passwords (D10), password throttling and the dummy argon2id check.
- TOTP (A5), recovery codes (Q22), admin TOTP reset, the pending-2FA state.
- Password-reset links and "reset ends all sessions".
- Step-up by password + TOTP with a 15-minute window (rev. 1 §1), replaced by §6.
