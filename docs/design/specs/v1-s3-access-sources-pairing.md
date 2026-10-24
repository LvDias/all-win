# V1-S3 — Passwordless access, signal sources and NinjaTrader pairing — Screen spec

> **Owner:** Product Designer (Aquarela, design-system owner) · **Status:** proposed (rev. 4), pending the open questions in §4 · **Revision:** 4 · **Date:** 2026-10-23
> **Slice:** [V1-S3](../../product/release-1/slice-plan.md) rev. 7 (passwordless; AC-1..AC-16; AC-12 blocked by Q28) · V1-S3b (WhatsApp, blocked by U17/Q31: §6 only) · cross-cutting X-2, X-4, X-5, X-6, X-8.
> **Policy:** [auth-and-session-policy.md](../../architecture/security/auth-and-session-policy.md) rev. 2 (passwordless, approved in Code Review). Where it is silent, ask the Architect or the PO.
> **Foundations:** [foundations.md](../foundations.md) ("F§8" points there); **components:** [components.md](../components.md) ("C§2" points there).
> **Decided:**
> - **By the PO** ([design-answers-s3-s4.md](../../product/release-1/design-answers-s3-s4.md)): D1 view-once URL, D2 immediate revocation, D4 revoked stays visible, D6 user names the connection, D15 USD, D18 states.
> - **By the user on 2026-10-23** ([decisions.md](../../delivery/decisions.md)): Q17 several named sources · Q18 unpair allowed after confirmation (now slice AC-15) · Q19 admin-only invites, single-use, bound to an e-mail, 7 days, manual delivery · **new requirement: no password, codes by e-mail (WhatsApp later)**.
> - **Pending with the user** (marked *provisional* where designed; see §4): Q28 recovery · Q29 session lengths · Q30 step-up window · Q33 limits · Q34 new-device notice and "sign out of all devices" · Q35 step-up list · Q36 · **U16 code format** (the policy recommends Crockford Base32, 8 characters, shown `XXXX-XXXX`).
> - **Superseded, removed from this spec:** password and its policy (D10), password reset (Q20 reset part, D9), optional TOTP and its banner (A5, D13), recovery codes and admin-assisted TOTP reset (Q22, D12), admin Users page (its only actions were reset link and TOTP reset).
>
> **Contract:** HTTP API 3.4.1 (auth/me operations published at 2.0.0): `contracts/http/openapi.yaml` and the [S3 contract notes](../../architecture/contracts/s3-access-sources-pairing.md). Operation names and error codes below are the **published** ones. Where this spec and the contract differ, **the contract wins**; tell the Designer. Not yet in the contract: `Connection.disconnect_reason` (S3-10, pending Q66).
> **Surfaces:** Web (Frontend), NinjaTrader plugin pairing panel (Plugin Engineer, S3-11, unchanged). **QA:** QA owner for `tests/e2e/`.
>
> **Rev. 4 changes (2026-10-23, slice-plan rev. 7):**
> - Login, invite sign-up, step-up and channel change are code based. New: code entry (`CodeInput` / `CodeChallenge`, C§1), resend with cooldown, channel choice slot (S3b), new-device notice e-mail, "sign out of all devices" and devices list **[provisional, Q34]**.
> - Removed: password fields, forgot/reset password, TOTP screens and banner, recovery codes, admin Users page, password-related `session.ended` reasons.
> - Screen ids S3-08a and S3-09..S3-11 are **kept** so that other specs' references stay valid; S3-04, S3-05, S3-07 and S3-08b were reused or removed (see §0).
> - AC numbers follow slice-plan rev. 7 (sources = AC-14, pairing = AC-15).

---

## 0. Information architecture for this slice

```
Public (no session)                      App (session) — shadcn Sidebar
/[locale]/login           (e-mail)         Conexões / Connections
/[locale]/login/code      (enter code)       ├─ Fontes de sinal      /[locale]/connections/sources
/[locale]/invite#t=…      (sign-up)          └─ NinjaTrader          /[locale]/connections/ninjatrader
                                           Configurações / Settings
                                             ├─ Segurança            /[locale]/settings/security
                                             └─ Preferências         /[locale]/settings/preferences  (S4)
                                           Administração (platform admin only, Q19)
                                             └─ Convites             /[locale]/admin/invites
                                           User menu: Idioma · Tema · Segurança · Sair
```

| Id | Screen | Id | Screen |
|---|---|---|---|
| S3-01 | Login: e-mail | S3-08a | Admin: invitations |
| S3-02 | Login: enter code | S3-09 | Signal sources |
| S3-03 | Invitation sign-up (details + code) | S3-10 | NinjaTrader connections |
| S3-04 | Step-up by code (dialog) | S3-11 | Plugin pairing panel |
| S3-05 | Change e-mail (dialog flow) | | |
| S3-06 | Security settings (channels, sessions, devices) | | |
| S3-07 | New-device notice (e-mail) | | |

- `[locale]` ∈ `pt-BR`, `en` (S4 spec).
- The app home (`/[locale]`) is a placeholder dashboard until S5/S13. It shows a "Primeiros passos" / "Getting started" card: 1 create a signal source, 2 pair NinjaTrader. Each step links to its screen and shows ✓ when done (derived from list counts the API returns; no new rule).
- Every app route requires a session; Next.js middleware only redirects to login (ADR-0001 §9). Authorization is the Core's.

## 1. Global rules for this slice

- **Isolation (AC-13, X-2):** every list and detail shows only the signed-in user's objects. An id of another user's object (in the URL or a request) yields the same "not found" screen as a non-existent id (`ErrorState`, code `common.not_found`). The UI never distinguishes "exists but not yours".
- **No password anywhere (AC-3).** No field, link, hint or e-mail mentions a password for All Win. Password managers must not offer to save anything on these pages: code fields use `autocomplete="one-time-code"`, the e-mail field `autocomplete="email"`.
- **Codes are secrets (X-4).** A one-time code:
  - is never placed in a URL, page title, toast, error message, `console`, analytics or a persisted cache;
  - lives only in the input's state, and the field is cleared after every attempt (success or failure);
  - is never echoed back by the UI ("Código incorreto", not "O código ABCD1234 está incorreto").
- **Tokens in links:** the invitation link carries its token only in the URL fragment (`#t=<token>`). The page reads `location.hash`, removes it at once with `history.replaceState`, keeps the token in memory, sends it in the **POST body**, sets `Referrer-Policy: no-referrer` and loads no third-party resources. A reload loses the token (see S3-03).
- **Challenge model (policy §2.2, §3).** Every code belongs to a *challenge* with an opaque `challenge_id` (a sealed token) returned only to the requester and kept **in the page's memory** (never URL, storage or logs). A reload or a language switch drops it, so the user starts again from S3-01 (the switcher is disabled while a challenge is pending, S4 §3). A **resend** replaces the code of the same challenge and invalidates the previous one. A challenge that is closed (used, 5 wrong attempts, expired) is never reopened: the UI asks for a **new challenge** instead (`Pedir novo código`). A new challenge never invalidates another one.
- **Code rules shown to the user (AC-5).** Every value below comes from the Core response or config (variables in the message, never hard-coded in the catalogs): code length, lifetime (10 min), maximum wrong attempts (5), resend cooldown (60 s today, Q33), rate-limit waits. The UI states the rules in plain words and does not compute them.
- **Non-enumeration (AC-7).** On the public pages the visible response, wording, states and their order are **identical** for a registered and an unregistered e-mail, for a disabled user, and for a failed delivery. Concretely:
  - the e-mail field is never checked inline for existence;
  - S3-01 always moves to S3-02, and S3-02 always says "Se {email} tiver uma conta…";
  - wrong, expired, already-used, exhausted and never-issued codes show **one** message (`auth.code_invalid`): "Código inválido ou expirado. Confira ou peça um novo.";
  - the UI never shows "attempts left", a different cooldown, or a delivery failure on a public page. It counts **its own** failed attempts per challenge (a resend does not reset them) only to suggest a new code after the 5th;
  - the channel choice (S3b) is offered from the deployment's available channels, never from the account's, and a chosen channel the account lacks still gets the normal answer (S3-01);
  - a channel change to a value owned by another account looks exactly like a normal one until the final confirmation fails with the generic message (S3-05).
  Timing equality is the Core's job; the UI adds no artificial delay and no behavior that depends on the account.
- **Audit (X-5, AC-16):** sign-ins, failed or rate-limited attempts, new-device notices, step-up results and channel changes are written by the Core to the user's audit log. The audit view is S10; this slice only needs the events emitted.
- **Duplicate submits:** F§10 "Submitting" applies to every form and confirm button. `Enviar código` and `Reenviar` are also protected by the cooldown (C§1.4).
- **Session ended elsewhere:** the SSE stream sends `session.ended` with a reason before closing. Open tabs redirect to login: `expired` → `?reason=session_expired`; `logout` → `signed_out`; `signed_out_everywhere`, `device_revoked` and `channel_changed` → `sessions_ended`. Password/TOTP reasons are removed; the Frontend maps whatever reasons the final contract defines to these three messages.
- **Step-up (AC-10, policy §6):** a **new code for that action**. Decided by the user: (i) accepting automation on an account (S5), (ii) unpairing a NinjaTrader instance (S3-10), (iii) changing the e-mail (S3-05, whose two codes are the step-up). **Provisional (the Architect's "musts", Q35):** creating or rotating a source URL (S3-09), creating a pairing code or moving an instance (S3-10), and issuing an invitation (S3-08a). S3-04 opens for every operation the contract marks `x-step-up`, and the original request is sent **once, with the code**. Adding or removing an action from the list therefore changes no screen.
- **Text:** all strings shown are examples in pt-BR / en. Frontend creates the keys (S4 spec §5 naming); final copy review is with the PO.

---

## S3-01 Login: e-mail — `/[locale]/login` (AC-4, AC-7)

**Purpose:** a registered user starts sign-in by asking for a code.
**Layout:** centered `Card` (max 400 px) on `--muted` background. Top: product name "All Win". Top-right of the page: language switcher + theme switcher (S4). Below the card: `Acesso somente por convite.` / `Access by invitation only.` No sign-up link (Q6).
**Title / text:** "Entrar" / "Sign in" · "Digite seu e-mail. Enviaremos um código para você entrar. Não há senha." / "Enter your e-mail. We'll send you a code to sign in. There is no password."

| Field | Component | Attributes | Validation (client) | API (operationId) |
|---|---|---|---|---|
| E-mail | `Input type=email` | `autocomplete=email`, `inputMode=email`, `autoCapitalize=none`, `spellCheck=false`, `autoFocus` | required; e-mail format only (never existence) | `requestLoginCode` → `email` |
| Receber código por / Send code by | `RadioGroup` (2 options: E-mail, WhatsApp) | **Only rendered when the Core reports more than one channel available in the deployment** (S3b). Default: the channel last used **on this device** (local storage in try/catch), else e-mail. Never depends on the typed e-mail | — | → `channel` (**required** by the contract; sent as `email` when the selector is not rendered) |
| Manter conectado / Keep me signed in | `Checkbox`, **unchecked by default** | Help text: "Por até {days} dias neste dispositivo. Não use em computador compartilhado." / "For up to {days} days on this device. Don't use on a shared computer." (values from the Core: 7 d idle / 30 d absolute today, AC-8, Q29 open) | — | kept in memory and sent with `verifyLoginCode` → `remember_me` (not with the request) |

**Actions:** `Enviar código` / `Send code` (primary, submits on Enter).

| State | UI |
|---|---|
| Default | As above. `?reason=session_expired`: info `Alert` "Sua sessão expirou. Entre de novo." / "Your session expired. Sign in again." · `signed_out`: "Você saiu da sua conta." · `sessions_ended`: "Suas sessões foram encerradas. Entre de novo." |
| Submitting | Button spinner "Enviando…"; duplicate submits ignored |
| Success | **Always** go to S3-02, whether or not the e-mail has an account (AC-7). The typed e-mail is carried in memory (no URL, no query) |
| Rate limited (`auth.rate_limited`, AC-6) | Destructive-soft `Alert`: "Muitos pedidos de código. Aguarde {time} e tente de novo." / "Too many code requests. Wait {time} and try again." from `Retry-After`; button disabled with a countdown (C§1.4). Same text for per-user and per-IP limits; it never says which one |
| Delivery problems | **Never shown on this page** (policy §2.3): the user sees the same text whatever happened. A failed send for a registered user is audited and shown after the next sign-in (S3-06 notice) |
| Network/server error | "Não foi possível conectar. Tente de novo." + code; value kept |
| Already signed in | Visiting `/login` redirects to app home |

**Accessibility:** form `aria-labelledby` the title; errors announced by focusing the `Alert` (`tabIndex=-1`, `role=alert`). Autofill and paste allowed.
**AC mapping:** AC-4, AC-6 (rate-limit message), AC-7 (same response), AC-8 (remember me).

## S3-02 Login: enter code — `/[locale]/login/code` (AC-4, AC-5, AC-6, AC-7)

**Purpose:** the user types the code received and gets a session.
**Entry:** only in the pending state (after S3-01). Otherwise redirect to `/login`. The page holds the e-mail in memory; it is shown as typed, so nothing is revealed. Reloading the page keeps the pending state only if the Core's pending challenge (cookie or token, the Architect's choice) is still valid; otherwise it goes to S3-01 with the e-mail empty.
**Layout:** same card. Title "Digite o código" / "Enter the code". Text (policy §2.3, fixed wording): "Se {email} estiver cadastrado, enviamos um código. Ele vale por {minutes} minutos e só funciona uma vez. Não chegou? Confira o spam ou peça outro em {seconds} s." / "If {email} is registered, we sent a code. It's valid for {minutes} minutes and works once. Didn't arrive? Check spam or request another in {seconds} s."

**Body:** `CodeChallenge` (C§1.3) with `purpose=login`. API: `verifyLoginCode { challenge_id, code, remember_me }`; `resendCode { challenge_id }`; the first cooldown is `resend_after_s` from the `requestLoginCode` answer (`ChallengeIssued`). Login never reads the delivery state (`getChallengeDelivery` answers `not_reported`).

| Element | Behavior |
|---|---|
| `CodeInput` | `autoFocus`; accepts paste with spaces, hyphens and lowercase; auto-submits when the code is complete (C§1.1) |
| `Verificar` / `Verify` | Explicit button, for clarity and for users who turn auto-submit off with assistive tech |
| `Reenviar código` / `Resend code` | Disabled with "Reenviar em 0:45" until the cooldown ends (60 s, C§1.4). A resend replaces the code of this challenge: info text "Enviamos um novo código. O anterior deixou de valer." Wrong attempts are **not** reset by a resend. When the challenge is closed (5 local failures, or 10 min since issue) the button becomes `Pedir novo código` / `Request a new code` and starts a **new challenge** (same cooldown and limits) |
| `Usar outro e-mail` / `Use another e-mail` | Link button → S3-01 with the field focused. Cancels the pending state |
| `Receber por outro canal` / `Get the code another way` | Only when more than one channel is available (S3b); returns to S3-01 with the e-mail kept |

| State | UI |
|---|---|
| Awaiting | Focus in `CodeInput`; polite announcement on arrival: "Código solicitado. Digite o código recebido." |
| Submitting | Input read-only, spinner; no second auto-submit |
| Wrong, expired, used, exhausted or never issued (`auth.code_invalid`) | **One** message in the field's error slot: "Código inválido ou expirado. Confira ou peça um novo." / "Invalid or expired code. Check it or request a new one." Input cleared and focused. No attempts counter |
| After the 5th failed attempt **made in this challenge** (counted by the UI, policy §3) | Input disabled; info `Alert`: "Peça um novo código para continuar." / "Request a new code to continue." Same for a registered and an unregistered e-mail; the server never tells them apart |
| Challenge lost (reload, forged or altered token) | The generic message above, or S3-01 with the e-mail empty after a reload. No separate "expired" message |
| Rate limited (`auth.rate_limited`) | As S3-01, on the resend button or the submit; the countdown replaces the label |
| Success | `LoginResult.new_device = true` → one-time info toast "Este é um dispositivo novo. Enviamos um aviso ao seu e-mail." / "This is a new device. We sent a notice to your e-mail." (never on sign-up). Redirect to `returnTo` if it is a **same-origin relative path** (else app home), in the profile locale (S4). Nothing about a new device is shown here: the notice is an e-mail (S3-07) |
| Lost access to the e-mail | **No link until Q28 is decided** (AC-12 blocked). A slot below the form is reserved for it. The design does not invent a recovery path |
| Network error | "Não foi possível conectar. Tente de novo." + code; value kept |

**Accessibility:** WCAG 3.3.7 (no redundant entry: the e-mail is not asked again) and 3.3.8 (paste and autofill supported, no memorization; copying the code from the e-mail is the accepted method). Live regions follow C§1.3.
**AC mapping:** AC-4, AC-5 (one-time, 10 min, 5 attempts, resend replaces the code of its challenge), AC-6, AC-7.

## S3-03 Invitation sign-up — `/[locale]/invite#t=<token>` (AC-1, AC-2, AC-3)

**Purpose:** an invited person creates their account. The account is created, and the invitation consumed, **only after the code is confirmed** (AC-2). The locale of the link is the one chosen when the invite was created (S3-08a).
**Entry:** link from the invitation (copied by the admin or e-mailed). On load the page reads the token from the fragment, removes it (§1), and validates it with `lookupInvitation` (token in the body). The token stays in memory until the account is created.

| Load state | UI |
|---|---|
| Validating | Card skeleton |
| Invalid / used / expired / revoked (`auth.invitation_invalid`) | `ErrorState`: "Este convite não é válido. Ele pode ter expirado ou já ter sido usado. Peça um novo convite." / "This invitation isn't valid. It may have expired or already been used. Ask for a new one." **One message for all causes.** No form |
| No `t` in the URL (for example after a reload) | Same `ErrorState` plus, below it: "Se você recarregou esta página, abra o link do convite de novo." / "If you reloaded this page, open the invitation link again." The invitation is still usable |
| Already signed in | `Alert`: "Você está conectado como {email}. Saia para aceitar este convite." with `Sair` |
| Valid | Two-step flow below, with `Stepper` (C§2, compact): "Etapa 1 de 2 · Seus dados" / "Step 1 of 2 · Your details" |

### Step 1: your details

Text: "Você entra sempre com um código enviado ao seu e-mail. Não é preciso criar senha." / "You always sign in with a code sent to your e-mail. No password needed."

| Field | Component | Validation | API (operationId) |
|---|---|---|---|
| E-mail | `Input` read-only, prefilled from the invite, with `Lock` icon and help "Este é o e-mail do convite." (Q36 recommends equal; AC-2 assumption) | — | from `lookupInvitation` |
| Idioma / Language | `Select` pt-BR / English, prefilled with the current locale | required | → `locale` (profile, S4) |
| Fuso horário / Time zone | Read-only line "Fuso horário: {zone} (GMT-03:00) · `Alterar`", prefilled from the browser. `Alterar` expands the S4 §4 time-zone `Select` | required; `me.invalid_time_zone` → inline error | → `time_zone` |
| Termos / Terms | Checkbox slot reserved above the button. **[Open]** vision §12: required before launch, not before dev | — | — |

**Action:** `Enviar código` / `Send code` → `startSignup { token }` (e-mail only). `locale`, `time_zone` and `remember_me` are held in memory and sent later with `completeSignup`. The invitation is unaffected until confirmation.
**If Q36 is decided as (b)** (invitee may use another e-mail): the E-mail field becomes editable, with the same non-enumeration behavior, and nothing else changes.

### Step 2: confirm your e-mail

"Etapa 2 de 2 · Confirmar e-mail". Text: "Enviamos um código para {email}. Digite-o para criar sua conta. Ele vale por {minutes} minutos. Não chegou? Confira o spam ou peça outro em {seconds} s." (the invitee is bound to that address, so it can be shown plainly.) The page may read the challenge's delivery state (`pending`, `sent`, `failed`, policy §2.3): on `failed` it shows "Não foi possível enviar o código. Tente de novo em instantes." + `Reenviar código`. `CodeChallenge` (C§1.3) with `purpose=signup` and a "Manter conectado" `Checkbox` (unchecked; `completeSignup` requires `remember_me`). API: `completeSignup { challenge_id, code, locale, time_zone, remember_me }`; every failure is 401 `auth.code_invalid`. Delivery state: `getChallengeDelivery { challenge_id }`. Back link `Voltar` / `Back` → step 1 (a new request later invalidates the earlier code, AC-5).

| State | UI |
|---|---|
| Wrong / expired / attempts exhausted / rate limited | As S3-02 (same messages and components) |
| Success | Account created, invitation consumed, signed in automatically → app home with "Primeiros passos", in the chosen language. **No new-device e-mail** for this first session (AC-9 assumption) |
| Abandoned or failed | Nothing is created; the invitation stays usable until it expires (AC-2) |
| Invitation became invalid meanwhile | Switch to the invalid state above |
| Network failure after submitting the code, outcome unknown | "Não sabemos se sua conta foi criada. Tente entrar com seu e-mail." + link to S3-01 (the Core's confirm is not assumed idempotent) |

**AC mapping:** AC-1 (refused without a valid invitation, with a clear message), AC-2 (confirm before creating), AC-3 (no password asked).

## S3-04 Step-up by code (dialog) (AC-10, AC-6)

**Purpose:** prove, with a **new code for that action**, that the person at the keyboard owns the account, immediately before a sensitive action. The pattern is defined once in [F§8.2](../foundations.md); this section lists the S3 usages. Mechanics follow policy §6.1: the client asks for a code bound to the session, the action kind and the target; then sends the **action request once, with `challenge_id` and `code`**; the Core verifies and performs the action in one transaction.

| Action | Status | Where it starts | `{action}` text in the dialog |
|---|---|---|---|
| (i) Accept automation (opt-in) on an account | decided (AC-10) | S5 opt-in dialog | "ativar ordens automatizadas na conta {name}" / "turn on automated orders on account {name}" |
| (ii) Unpair an instance | decided (AC-10) | S3-10 Unpair | "desparear a conexão “{name}”" / "unpair connection “{name}”" |
| (iii) Change e-mail | decided (AC-10, AC-11) | S3-05 | its two codes **are** the step-up (answered by policy §7); no extra dialog |
| Create or rotate a source URL (`source_create`, `source_rotate`) | **provisional** (Q35, Core setting `step_up.q35_persistence_actions`, default on) | S3-09 | "criar ou trocar a URL da fonte “{name}”" / "create or rotate the URL of source “{name}”" |
| Create a connection or a pairing code, move an instance (`connection_create`, `pairing_code_new`) | **provisional** (same setting) | S3-10 | "gerar um código de pareamento para “{name}”" / "generate a pairing code for “{name}”" |
| Issue an invitation (`admin_invitation_create`) | **provisional** (same setting) | S3-08a | "convidar {email}" / "invite {email}" |
| Enable a route; change an account binding (`route_enable`, `account_binding_change`) | **provisional** (same setting; S7/S5 screens) | S7 / S5 | text supplied by those specs |

API: `requestStepUp { action, channel, target_id? }` → `ChallengeIssued`; the action request then carries `step_up: { challenge_id, code }` (`StepUpProof`). `target_id` is required for source_rotate, pairing_code_new, connection_unpair, account_opt_in_accept, route_enable and account_binding_change, and forbidden for the others. Errors: `auth.channel_unavailable` (no confirmed value on that channel) and `auth.step_up_channel_not_allowed` (the Core requires another channel on this device: the dialog switches to e-mail and says so, M3). Delivery: `getChallengeDelivery`.

Not asked: declining an opt-in, revoking a source, renaming, signing out (they only reduce access, policy §6.2). Enforcement off and max position are a recommendation awaiting Q35; the dialog covers them with no new screen if adopted.

**Flow:** the user confirms the action in its own dialog → S3-04 replaces it (never two modals) → with one confirmed channel the code is sent on open → `CodeChallenge` `purpose=step_up` → the action request goes out once with the code → its result step follows. Cancel or Esc: nothing is performed. Wrong code: the action response is `auth.code_invalid`; the dialog stays open with the generic message, the action is not performed and the attempt is audited. After 5 local failures the dialog asks `Pedir novo código`.
**Persistence actions from a new device (M3, provisional, user decision):** when the Core says the device is new (first seen < 7 days), the dialog offers e-mail only; after S3b it asks a code on **both** channels, one after the other, with a `Stepper` ("Código 1 de 2"). Text: "Neste dispositivo novo, o código precisa ser confirmado no e-mail." / "On this new device the code must be confirmed by e-mail."
**Q30 does not change the UI** (the dialog opens whenever the Core requires a code). Channel change excludes the "pick any channel" rule (§S3-05).
**AC mapping:** AC-10, AC-6, AC-16.

## S3-05 Change e-mail (dialog flow) (AC-11, AC-6)

**Purpose:** replace the sign-in e-mail. One flow, two codes: one to the **current** address and one to the **new** one. The change happens only when both are confirmed within 10 minutes; until then the current value stays in force (policy §7).
**Entry:** S3-06 Card A, `Alterar e-mail` / `Change e-mail`. `ResponsiveDialog` (Drawer below `md`) with the `Stepper` (C§2): 4 steps.

| Step | Content | API (operationId) |
|---|---|---|
| 1 Novo e-mail / New e-mail | `Input type=email` "Novo e-mail" (`autocomplete=email`). Text: "Vamos enviar um código para o e-mail atual ({masked_old}) e outro para o novo. Os dois são necessários." Action `Enviar códigos` | `startChannelChange { channel: "email", new_value }` → `ChannelChange` (`id`, `codes[]` with `role` and `challenge_id`, `expires_at`, `masked_new_value`) |
| 2 E-mail atual / Current e-mail | `CodeChallenge` `purpose=channel_old`: "Digite o código enviado para {masked_old}." | `verifyChannelChangeCode { challenge_id, code }` on `/me/channel-changes/{change_id}` for the `current_value` code |
| 3 Novo e-mail / New e-mail | `CodeChallenge` `purpose=channel_new`: "Digite o código enviado para {new_email}." The delivery state comes from `getChallengeDelivery` (a suppressed send reads `sent`) | same operation for the `new_value` code |
| 4 Pronto / Done | `CircleCheck`: "E-mail alterado para {new_email}. Suas sessões em outros dispositivos foram encerradas. Avisamos o e-mail anterior." Focus on the heading; `Concluir` | — |

- **Always proceeds (policy §3).** Step 1 never refuses a new e-mail because it belongs to another account: the screen shows step 3 and its delivery state reads "sent" exactly as for a free address. The only refusals are format (`me.invalid_channel_value`, pointer `/new_value`) and the address that equals the user's own (422 `me.channel_unchanged`: "Este já é o seu e-mail atual." / "This is already your current e-mail."). If the address is taken, the final confirmation fails with the generic "Código inválido ou expirado. Confira ou peça um novo." The UI never says that an address is in use. Format errors are the only step-1 refusal.
- **Pending window:** a line under the stepper, "Esta alteração expira em 8:12" / "This change expires in 8:12" from the server's `expires_at` (10 min), `aria-live=off`; polite announcement at 1 min left and at expiry.
- **Order is free for the Core.** Steps 2 and 3 only collect the codes; each can be resent on its own (60 s cooldown); a resend never touches the other code.
- **Expired or failed:** the generic code message (`auth.code_invalid`); `me.channel_change_closed` (expired, cancelled or completed in another tab) → "Esta alteração não está mais aberta." and a fresh start; after 5 failures in a step, `Pedir novo código` restarts the flow.
- **Cancel / close at any step:** allowed without a guard. Nothing changes; reopening starts a new flow. Toast on close from steps 2–3: "Alteração cancelada. Seu e-mail continua {masked_old}."
- **24-hour freeze (policy §7):** `getMySecurity.channel_change_allowed_at` in the future → `Alterar e-mail` is disabled with the reason "Você trocou o e-mail há pouco. Uma nova troca fica disponível em {date-time zone}." A refused request (409 `me.channel_change_too_soon`) shows the same text. 409 `me.channel_change_pending` (another change is open or on hold) → refetch and show the pending-change banner (S3-06 Card A).
- **Hold for a new device (M2, provisional, user decision; the same hold serves Q28):** `verifyChannelChangeCode` answers `completed` or `on_hold` (with `hold_until`). On `on_hold`, step 4 reads "Alteração agendada. Seu e-mail atual continua valendo até {date-time zone}. Avisamos o e-mail atual." and the security page shows the **pending-change banner** (S3-06 Card A) with `Cancelar troca` / `Cancel change`.
- **Old address unreachable (Q28):** no link until Q28 is decided; a slot under step 2 is reserved (policy §7 and §9 describe the 72 h hold; not designed until the user decides).
- **Step-up:** the two codes are the AC-10 (iii) step-up; no extra dialog.
- **After S3b** a change of one channel also needs a code from the **other** confirmed channel (policy §7): a third `CodeChallenge` step is inserted and the stepper grows to 5 steps.
- `{masked_old}` is the masked value from the Core, for example `j***@d***.com`. The web never masks by itself.

**AC mapping:** AC-11, AC-10 (iii), AC-6, AC-16.

## S3-06 Security settings — `/[locale]/settings/security` (AC-8, AC-9, AC-11)

**Layout:** `PageHeader` "Segurança" / "Security"; stacked `Card`s. Above them, when present: **delivery notice** (info `Alert`, from the audit): "Um código de acesso não pôde ser enviado em {date-time zone}. Se você estava tentando entrar, peça outro código." + `Ver auditoria` (S10). It is driven by the audit entry `security.code_delivery_failed` (details: `channel`, `purpose`; never the code) that the Core records for a registered user (policy §2.3). **[Dependency: how the entry reaches the shell after sign-in (audit read or notification) to be confirmed with the Architect; until then the entry is visible in S10 only.]**

### Card A: Sign-in channels / Canais de entrada
- Data: `getMySecurity` (`email`, optional `whatsapp`, `session`, `channel_change_allowed_at`, `pending_channel_change`). Cancel: `cancelChannelChange { change_id }` (any session, idempotent).
- Text: "Você entra com um código enviado a um destes canais." / "You sign in with a code sent to one of these channels."
- Row **E-mail**: address in full (the user's own), `Badge` `Confirmado` (success, `CircleCheck`), action `Alterar e-mail` → S3-05 (disabled during the 24 h freeze, with the reason).
- Row **WhatsApp** (S3b only, hidden until then): see §6.
- **Pending-change banner** (only with a hold, provisional): warning `Alert` "Troca de e-mail pendente: {masked_new} passa a valer em {date-time zone}. Até lá, o e-mail atual continua valendo." + `Cancelar troca`. `AlertDialog`: "Cancelar a troca de e-mail?" "O e-mail atual continua o mesmo. Avisaremos todos os canais." Cancelling is available from any session and never requires a code (policy §9).
- Note: "Alterar um canal exige um código no canal atual e outro no novo." / "Changing a channel needs a code on the current channel and one on the new channel."
- Advice line: "Proteja também a conta do seu e-mail (por exemplo, com verificação em duas etapas do provedor): quem controla o e-mail controla o acesso." / "Also protect your e-mail account (for example with your provider's two-step verification): whoever controls the e-mail controls access." (policy §8)

### Card B: Session / Sessão
- `Sair` / `Sign out` button (same as the user menu).
- Text (AC-8): "Sua sessão termina após {idle} sem uso ou {absolute} depois de entrar. Com “Manter conectado”: {idle_r} sem uso ou {absolute_r}." Values from the Core (today 12 h / 24 h and 7 d / 30 d; Q29 recommends keeping them).
- "Apenas deixar a tela aberta não mantém a sessão ativa." / "Just leaving the page open doesn't keep the session active."

### Card C: Devices and sessions / Dispositivos e sessões **[provisional, Q34]**
- A `DataTable` (C§4, cards below `md`) of known devices: Dispositivo ("Chrome no Windows", family only) · Primeiro acesso · Último acesso · badge `Este dispositivo` (info) · Ações. Action `Esquecer dispositivo` / `Forget this device`: `AlertDialog` "Esquecer este dispositivo?" "As sessões dele serão encerradas. No próximo acesso a partir dele, você receberá o aviso de novo dispositivo." (policy §5.2). Not offered for the current device (use `Sair`). API: `listDevices` (`label`, `first_seen_at`, `last_seen_at`, `current`), `forgetDevice` (no step-up; idempotent).
- Button `Sair de todos os dispositivos` / `Sign out of all devices` → `AlertDialog` (destructive), title "Sair de todos os dispositivos?"; body: "Todas as sessões, inclusive esta, serão encerradas e os dispositivos serão esquecidos. Você precisará entrar de novo com um código." Then a block **"Antes de sair, revise o que continua funcionando"**: "Isto **não** revoga as URLs de alertas nem as conexões do NinjaTrader. Se você suspeita de acesso indevido, revogue o que não reconhece." with the counts and links "{n} fontes ativas → Fontes de sinal" and "{m} conexões pareadas → NinjaTrader" (policy §5.3, L3). Confirm `Sair de todos` → `signOutEverywhere` (no step-up) → redirect to login with `?reason=sessions_ended`. No step-up (it only reduces access).

**AC mapping:** AC-8 (Card B), AC-9 (Card C, notice), AC-11 (Card A), AC-16 (delivery notice).

## S3-07 New-device notice (e-mail) (AC-9) — template for Core/Frontend

Sent to the account e-mail through the outbox when a sign-in comes from a browser without a valid device cookie of that user (policy §5.2): the first sign-in after sign-up from another browser counts; the sign-up session itself sends none. Never as a *notification* (S18 AC-3). Also sent when the sign-in used WhatsApp (S3b). Language and time zone: the user's profile (S4). Content follows policy §5.3 **[provisional, Q34]**.

| Part | pt-BR | en |
|---|---|---|
| Subject | Novo acesso à sua conta All Win | New sign-in to your All Win account |
| Lead | Sua conta foi acessada de um dispositivo que não usávamos antes. | Your account was accessed from a device we hadn't seen before. |
| Details (list) | Quando: {date-time + zone} · Navegador e sistema: {browser} em {OS} · Canal do código: e-mail · Endereço IP: {ip}. *No location (no geolocation in V1).* | When: … · Browser and system: … · Code channel: e-mail · IP address: {ip} |
| If it was you | Foi você? Nada a fazer. | Was it you? Nothing to do. |
| If it was not | Não foi você? 1. Entre no All Win, abra **Segurança → Dispositivos e sessões** e use **Sair de todos os dispositivos**. 2. Em **Fontes de sinal** e **NinjaTrader**, revogue o que você não reconhece: URLs e conexões continuam funcionando depois que a sessão termina. 3. Confira a segurança da sua conta de e-mail. | Wasn't you? 1. Sign in to All Win, open **Security → Devices and sessions** and use **Sign out of all devices**. 2. In **Signal sources** and **NinjaTrader**, revoke anything you don't recognize: URLs and connections keep working after the session ends. 3. Check the security of your e-mail account. |

**No link of any kind that acts or signs in, and no code** (policy §5.3: an unauthenticated one-click action would let anyone holding the e-mail sign the user out). The e-mail names the path in words. No tracking pixel. The notice for an e-mail change sent to the **old** address (S3-05) uses the same layout: "Seu e-mail de entrada foi trocado para {masked_new}. Não foi você? …" with the same steps.

---

## S3-08 Administration (platform admin) — AC-1, Q19

Q19 decided: only a platform admin (`is_platform_admin`, held by the user) issues invitations. They are single use, bound to one e-mail, valid 7 days and delivered manually; the invitation can also be e-mailed (Q20 invite part).
**Visibility:** the sidebar group "Administração" and its route exist only for admins. A non-admin gets the generic not-found screen (does not reveal the route). Admin screens show identity data only (e-mail, dates). **Never** another user's accounts, sources, connections or trades (X-2). The former Users page (reset link, TOTP reset) is **removed**; if the user chooses admin-assisted recovery (Q28 option a), a Users page returns with a single audited "Recuperar acesso" action, designed then.

### S3-08a Invitations — `/[locale]/admin/invites`

**List** (`Table` ≥ `md`, cards below): E-mail · Status (`Pendente` info / `Usado` success / `Expirado` neutral / `Revogado` neutral) · Criado em · Expira em (date-time with zone, F§6.4) · Actions (`Revogar` for pending).
**Empty:** "Nenhum convite ainda. Convide alguém para usar o All Win." + `Convidar`.

**Create** (`ResponsiveDialog` "Novo convite"; submitting runs the admin step-up by code, S3-04, provisional):
| Field | Validation | API |
|---|---|---|
| E-mail | required, e-mail format; if an account or pending invite exists for it: "Já existe uma conta ou um convite pendente para este e-mail." (admin-only screen, so this disclosure is acceptable; PO to confirm, §4) | `createInvitation` → `email` |
| Idioma do convite / Invitation language | pt-BR / English, default = admin's locale | → `locale` (link, e-mail and landing-page language, S4) |

Result step:
- "Enviamos o convite para {email}." (Q20). If sending fails: warning `Alert` "Não foi possível enviar o e-mail. Copie o link e envie você mesmo." The invite stays valid.
- The invite link in a `SecretField` with `Copiar`, for manual delivery (Q19). It is a credential: **view-once**, since the Core stores only its SHA-256 (auth policy §3). Plus "Expira em {date time zone}".
- Closing without copying shows no guard when the e-mail was sent. When sending failed, it shows the same guard as S3-09 create.

**Revoke:** `AlertDialog` "Revogar o convite para {email}? O link deixará de funcionar." → `revokeInvitation`.

## S3-09 Signal sources — `/[locale]/connections/sources` (AC-14, S1 AC-2/AC-6, Q17)

**Purpose:** create, rotate and revoke the secret webhook URLs that TradingView alerts post to. The URL contains the secret (A3); it is **view-once** (D1). Q17 decided: several named sources, one URL each (e.g. one per chart/alert).
**Layout:** `PageHeader` "Fontes de sinal" / "Signal sources" + description "Cada fonte tem uma URL secreta. Cole-a no campo Webhook URL do alerta no TradingView." + primary `Nova fonte` / `New source`. Toggle (`Switch`) "Mostrar revogadas" / "Show revoked" (default off).

**List** — `Table` ≥ `md`, `Card` list below (F§9):

| Column | Content | Source |
|---|---|---|
| Nome / Name | text | `sources.list[].name` |
| Tipo / Type | `TradingView` · badge `Automatizada` / `Automated` (info, `Bot` icon). This is the classification used by the S5 automation opt-in (O1) | `.type`, `.is_automated` |
| URL | **Masked only** (never revealable after creation): `https://{host}/ingress/v1/tradingview/awk_{key_id}_••••••••` in mono, truncated in the middle on narrow widths; full masked value in a tooltip. Only the `key_id` is ever shown (root CLAUDE.md §2) | `.webhook_url_masked` (Core provides; the web never builds secrets) |
| Status | `Ativa` (success) / `Revogada` (neutral, `Ban`) + revoked date | `.status`, `.revoked_at` |
| Criada em / Created | date-time | `.created_at` |
| Último sinal / Last signal | relative time + absolute tooltip, or "Nunca" / "Never" | `.last_received_at`, **[Dependency: S1/S10 must expose it; hide the column if absent]** |
| Ações | `DropdownMenu`: `Renomear`, `Gerar nova URL`, `Revogar` (active only) | |

**Empty state:** title "Nenhuma fonte de sinal" / "No signal sources"; text "Crie uma fonte para receber os alertas do TradingView com segurança."; action `Nova fonte`. Below, a link to the alert setup guide (**[Dependency: S17 guide]**; hidden until it exists).
**Loading:** 3 skeleton rows. **Error:** `ErrorState` with retry.

### Create source (`ResponsiveDialog`, 2 steps; compact `Stepper`, C§2; **provisional step-up by code first**, S3-04, Q35)
1. **Name:** `Input` "Nome da fonte" / "Source name", required, 1–60 chars, trimmed; example placeholder "Ex.: Strikes MNQ". Duplicate name among the user's active sources: inline "Você já tem uma fonte com este nome." **[Proposal: names unique among active sources; PO to confirm, §4]**. Action `Criar` → `createSource { name }`.
2. **Copy the URL (view-once):**
   - Warning `Alert`: "Copie a URL agora. Por segurança, ela não será mostrada de novo. Se perdê-la, gere uma nova." / "Copy the URL now. For security, it won't be shown again. If you lose it, rotate to get a new one."
   - `SecretField` (F§8) with the full `webhook_url` from the response: masked by default, `Revelar`, `Copiar`.
   - Steps text: "1. No TradingView, abra o alerta → Notificações → Webhook URL. 2. Cole a URL. 3. Salve o alerta." **[Copy to validate with PO/S17 against TradingView's current UI]**
   - Automation note (info `Alert`, O1): "Ordens desta fonte são automatizadas. Contas de firmas que proíbem automação só as recebem se você aceitar isso na conta." with a link to Accounts (S5).
   - `Concluir` / `Done`. If the user tries to close (button, Esc, overlay, back gesture) **without having used Copiar**, an `AlertDialog` asks: "Você ainda não copiou a URL. Se fechar agora, precisará gerar uma nova. Fechar mesmo assim?" (`Voltar e copiar` focused / `Fechar`).
   - On close: secret wiped from state; list refetched; toast "Fonte criada." (no URL in the toast).
   - Errors at step 1: inline/`Alert`; nothing created. Network failure **after** submit with unknown outcome: "Não sabemos se a fonte foi criada. Atualize a lista antes de tentar de novo." + `Atualizar lista` (prevents duplicate sources; the Core's create is not assumed idempotent).

### Rotate (`Gerar nova URL`): D2, immediate
- `AlertDialog`: title "Gerar nova URL para “{name}”?"; body: "A URL atual para de funcionar **imediatamente**. Alertas que chegarem com ela serão rejeitados e registrados na auditoria. Depois, cole a nova URL no alerta do TradingView." Confirm `Gerar nova URL` (destructive) → step-up by code (S3-04, provisional Q35), then the new URL.
- Then the same step-2 view-once dialog as create. Toast "URL da fonte “{name}” trocada."
- `rotateSource` → returns the new `webhook_url` once.

### Revoke: D4, stays visible
- `AlertDialog`: "Revogar a fonte “{name}”?"; body: "Alertas com esta URL serão rejeitados. A fonte continua no histórico como revogada. Isso não pode ser desfeito; para voltar a receber sinais, crie uma nova fonte." Confirm `Revogar fonte` (destructive) → `revokeSource`.
- Row becomes `Revogada`, hidden unless "Mostrar revogadas" is on. Actions on a revoked row: none (name kept for history).
- **[Dependency S7]:** if routing rules reference the source, the dialog lists them ("Rotas que usam esta fonte deixarão de receber sinais: …"). Until S7 exists, no list.

### Rename
- Dialog with the name field (same validation) → `renameSource`. Does not change the URL; no step-up.

**Accessibility:** the masked URL cell has `aria-label` "URL secreta, oculta, identificador {key_id}" (it reads the key id, not the bullets). Row actions button: "Ações para {name}".
**AC mapping:** AC-14 (generate, view once, revoke; old secret rejected: surfaced by copy and audit), AC-13 (own sources only), X-4 (masking, no secret in toasts/URL/logs), X-5 (actions audited).

## S3-10 NinjaTrader connections — `/[locale]/connections/ninjatrader` (AC-15, Q18)

**Purpose:** pair each NinjaTrader 8 instance to the user and unpair it. Each instance has its own credential, a device token that is never shown in the web. Live status via SSE (ADR-0001 §9). Pairing per ADR-0004 §8.
**Layout:** `PageHeader` "Conexões NinjaTrader" / "NinjaTrader connections" + `Nova conexão` / `New connection`. Toggle "Mostrar despareadas" / "Show unpaired" (default off).

**List** (`Table` ≥ `md`; cards below, one per instance):

| Column | Content | Source |
|---|---|---|
| Nome / Name | user-given (D6), e.g. "VPS Chicago"; the plugin-reported `instance_name` appears as secondary text when different | `.name`, `.instance_name` |
| Status | F§7.2: `Aguardando pareamento` / `Conectada` / `Desconectada` / `Desatualizada` (warning, `CircleArrowUp`: plugin older than the Core's minimum, ADR-0004 §9) / `Despareada` + "há 3 min" since last change | `.status`, `.status_changed_at` (live) |
| Contas / Accounts | count of NinjaTrader accounts reported in `hello`; expandable row / `Collapsible` on the card listing each account name (e.g. `Sim101`) with its binding: "Vinculada a {All Win account}" or "Não vinculada" + link `Vincular` → S5 account form (ADR-0004 §8 step 4) | `.accounts[]`, `.accounts[].bound_account` |
| Plugin | version + build | `.adapter_version` |
| Última conexão / Last seen | date-time | `.last_seen_at` |
| Detalhes | instance id (mono, from plugin; D6) in the expanded row | `.instance_id` |
| Ações | `Renomear`; `Gerar código de pareamento` (awaiting pairing); `Mover para outra máquina` (paired: re-pairing); `Desparear` | |

**Empty state:** title "Nenhum NinjaTrader conectado"; three numbered steps: "1. Instale o plugin All Win no NinjaTrader 8. 2. Crie uma conexão aqui. 3. Digite o código de pareamento no plugin." Link "Como instalar o plugin" **[Dependency: install guide by Plugin Engineer; hidden until it exists]**; action `Nova conexão`.
**Disconnected instance:** row shows danger badge `Desconectada` + a reason line from `disconnect_reason` (the badge never carries the reason alone):
- `user` **[provisional, Q66; needs `Connection.disconnect_reason`]**: "Desconectada pelo usuário no plugin. O pareamento continua válido. Ordens para as contas desta instância não serão executadas até ele conectar de novo." / "Disconnected by the user in the plugin. The pairing is still valid. Orders for this instance's accounts won't run until they connect again."
- `network`: "O plugin perdeu a conexão. Ordens para as contas desta instância não serão executadas."
- `outdated` / `unpaired`: use the `Desatualizada` / `Despareada` states below.
- Field absent (contract not yet published) or unknown value: "O plugin não está conectado. Ordens para as contas desta instância não serão executadas."
The consequence wording stays the same in every case.
**What else appears for `user` (ADR-0004 §6, answered by the Architect, provisional with Q66):**
- **Mapped accounts flat and nothing in flight:** only the status, the reason line above and an audit entry "Desconectada pelo usuário no plugin". **No alert** (S11/S18).
- **Open position or a command in flight on a mapped account:** that account becomes `Fora de sincronia` (F§7.2) and an S11/S18 alert is raised with reason `sync.disconnected_by_user`: "Você desconectou o plugin com posição aberta; o All Win não está acompanhando esta conta." / "You disconnected the plugin with an open position; All Win is not tracking this account." Routing to it stops. The connection row lists the affected accounts with `Fora de sincronia` and a link to the S11 view; it offers no resync control here (S11 AC-2).
- In both cases local enforcement (S15) keeps working in the plugin. The alert wording is registered in the S18 catalog by the S18 owner.
**Outdated instance:** warning badge + "Atualize o plugin para voltar a conectar." + link to the download page **[Dependency]**.

### Create connection + pairing (`ResponsiveDialog`, stays open while waiting; **provisional step-up by code first**, S3-04, Q35)
1. **Name:** `Input` "Nome da conexão" / "Connection name", required, 1–60 chars, placeholder "Ex.: PC de casa, VPS Chicago". → `createConnection { name }`, which returns the connection, the `pairing_code` and `expires_at`.
2. **Pairing code** (ADR-0004 §8: 8 Crockford Base32 characters shown `XXXX-XXXX`, 10 minutes, single use, one active code per connection):
   - Shown **visibly** (it must be typed into another app): `text-3xl` mono, letter-spaced, e.g. `K7QM-4D2P`, with a `Copiar` button. It is short-lived and single use, so it is not masked; the dialog says "Não compartilhe este código." / "Don't share this code."
   - Countdown: "Expira em 9:41" from the server's `expires_at` (`aria-live=off`; a polite announcement only at 1 min left and at expiry).
   - Instructions: "No NinjaTrader: abra a janela All Win, clique em Parear e digite o código. Letras maiúsculas ou minúsculas, com ou sem hífen." **[Menu path to validate with the Plugin Engineer after the NT8 spike]**
   - Waiting indicator: "Aguardando o plugin…" with a spinner (static text under reduced motion).
   - **Paired** (SSE `connection.paired`; its `visible_accounts` is not used for the count, because accounts arrive after the plugin's `hello` and the list is rebuilt when the connection is `connected` (Core behavior reported by the Frontend, 2026-10-24; not yet stated in the contract: documentation request to the Architect, Q-S3-P1)): success state with `CircleCheck`. Focus moves to the success heading.
     - Until the connection is `connected`: heading "“{name}” pareada" / "“{name}” paired"; below it, in a separate `role="status"` line, "Buscando contas…" / "Looking for accounts…" with a spinner (static text under reduced motion). Only `Concluir`.
     - When `connected`: heading "“{name}” conectada" / "“{name}” connected"; the status line shows the real count (ICU plural): "1 conta encontrada: Sim101" / "1 account found: Sim101" · "{n} contas encontradas: Sim101, …" / "{n} accounts found: Sim101, …" · "Nenhuma conta encontrada." / "No accounts found." (only after `connected`). Only the status line is announced; focus does not move again. `Vincular contas` (→ S5) appears when n ≥ 1; `Concluir` is always there.
     - No timeout (provisional, 2026-10-24): if the plugin drops before `connected`, the line stays "Buscando contas…"; the user can close, and the list row shows the real status.
   - **Expired:** "O código expirou." + `Gerar novo código` (→ `newPairingCode`).
   - **Code invalidated** (SSE `connection.pairing_code_invalidated`, by `reason`):
     - `superseded`: a newer code was generated for this connection (e.g. in another tab or by "Mover para outra máquina"). Info `Alert`: "Um código mais novo foi gerado para esta conexão. Este não vale mais." + `Gerar novo código`. The displayed code is replaced by "—" and the countdown stops.
     - `pairing_locked`: pairing is temporarily locked for everyone after too many wrong codes (global protection, contract notes §2). Warning `Alert`: "O pareamento está temporariamente bloqueado por excesso de tentativas erradas. Aguarde alguns minutos e gere um novo código." + `Gerar novo código` (if the Core still refuses, the button shows the returned error).
   - Wrong codes typed in the plugin are **not** reported to the web: the Core cannot tie a wrong code to a connection. The plugin shows them (S3-11).
   - **Closing while waiting:** allowed without a guard (no secret is lost: a new code can be generated). The connection stays in the list as `Aguardando pareamento`.

### Move to another machine (re-pairing, ADR-0004 §8)
- `AlertDialog`: "Mover “{name}” para outra máquina?"; body: "Você vai receber um novo código. Quando ele for usado, a instância atual será desconectada e não receberá mais ordens." Confirm `Gerar código` → step-up by code (S3-04, provisional Q35) → same pairing step 2 as above.
- Until the new code is used, the current instance keeps working. On success: toast "“{name}” agora está em outra máquina." Account bindings **follow the connection by platform account name** (contract notes §2). Bound accounts the new instance does not report become out of sync (`sync.account_not_found_after_repair`). The connection row then lists them as "Não encontrada na nova instância" (danger) with `Vincular de novo` (→ S5) and a link to the out-of-sync view (S11).

### Unpair (`Desparear`): Q18 decided; **revocation happens only here (Q66, provisional)**
Allowed after a confirmation that lists the affected accounts; the accounts become out of sync; nothing is closed automatically.
- `AlertDialog` title: "Desparear “{name}”?" Confirming opens the **step-up by code (S3-04, AC-10 ii)**; the unpair request is sent only after a valid code. Cancelling the code leaves the connection paired. The open-position checkbox below is answered before the code is requested.
- Body (always): "O plugin desta instância será desconectado imediatamente e não receberá mais ordens. Para reconectar, crie uma nova conexão."
- If any of the instance's accounts is bound to an All Win account **[Dependency S5 data]**: list "Estas contas ficarão fora de sincronia e deixarão de receber ordens: APEX-12345-01, Sim101."
- If any has an open position **[Dependency S11 position data]**: danger `Alert` inside the dialog: "Há posição aberta em: APEX-12345-01 (2 contratos). **A posição continua aberta no NinjaTrader**, sem monitoramento nem proteção de risco do All Win. Nada será fechado automaticamente." Confirmation requires a checkbox "Entendo que a posição continuará aberta" before `Desparear` enables.
- Confirm `Desparear` (destructive) → `unpairConnection`. Row → `Despareada`. Other connections are unaffected (AC-4); the dialog says nothing about them.
- **Q66 (provisional):** this is the only place a pairing is revoked. If the user disconnected the plugin locally (S3-11 `Desconectar`), the row reads `Desconectada` and still offers `Desparear`; the dialog text tells them that the pairing is still valid until they confirm here.
- Unpaired connections stay listed (history) under "Mostrar despareadas", like revoked sources. No actions on them.

**AC mapping:** AC-15 (pair per instance, own credential, unpair; unpaired receives nothing; revoking one does not affect others), AC-13, AC-10 (ii), X-4 (device token never displayed), X-5.

## S3-11 NinjaTrader plugin: pairing panel (Plugin Engineer)

The plugin UI follows NinjaTrader 8 conventions (NTWindow/WPF, NinjaTrader skin colors, not shadcn). A detailed window spec is owed after the NT8 spike. This section fixes **states and copy** so web and plugin agree. Language follows the web profile (Q11; received in `welcome`). Before the first pairing, the plugin uses the Windows UI language if pt or en, else pt-BR (S4 §6).

| State | Plugin shows | Action |
|---|---|---|
| Not paired | "Não pareado. Gere um código em All Win → Conexões → NinjaTrader." | Code field (accepts 8 characters, case-insensitive, ignores spaces and hyphens, displays as `XXXX-XXXX`) + `Parear` |
| Pairing | "Pareando…" | Disabled |
| Invalid / expired / used / superseded code (`pairing.invalid_code`) | "Código inválido ou expirado. Confira o código ou gere um novo no All Win." (one message for all causes) | Field kept for correction |
| Rate limited (`pairing.rate_limited`, which also covers the temporary lock) | "Muitas tentativas. Aguarde {time} e tente de novo." (`Retry-After` when present; otherwise "alguns minutos") | Disabled with countdown |
| Plugin version unsupported at pairing (`pairing.adapter_version_unsupported`) | Same as "Update required" below | — |
| Paired + connected | Connection name (from Core), "Conectado", account count | `Desconectar` / `Disconnect` **[provisional, Q66]**: pauses the connection **on this computer only**, keeps the device token and does not revoke the pairing (protocol 1.3: close code 4100 `user_disconnect`). Confirmation (WPF dialog): "Desconectar o All Win neste computador? O pareamento continua válido; ordens deixam de chegar aqui até você conectar de novo. A proteção local de risco continua ativa. Para revogar o pareamento, use o All Win na web." |
| Disconnected by the user (after `Desconectar`) **[provisional, Q66]** | "Desconectado neste computador. O pareamento continua válido. Para revogá-lo, use All Win → Conexões → NinjaTrader na web." The plugin stores a persistent "disconnected by user" flag: it **does not reconnect by itself**, not even after NinjaTrader or the plugin restart. Local enforcement (S15) keeps working while disconnected | `Conectar` / `Connect` (uses the stored device token; no new code) |
| Disconnected / reconnecting | "Sem conexão com o All Win. Tentando reconectar…" + last connected time | — |
| Revoked (**revoked in the web with a code (step-up)**, or re-paired on another machine; close code 4002) | "Esta instância foi despareada ou movida para outra máquina no All Win. Gere um novo código para conectar." | Back to Not paired |
| Update required (4003) | "Versão do plugin não suportada. Atualize o plugin." | Link to download page (**[Dependency]**) |

**Q66 (provisional, Traço/PO):** the plugin has **no** revoke or unpair action. Revoking a pairing happens only in the web (S3-10 Unpair), after a step-up by code (S3-04). The plugin's `Desconectar` is local and reversible; the Core keeps the pairing, and the instance stays listed as paired. The word "Desparear" no longer appears in the plugin UI (glossary: F§5 "Pair / Unpair" applies to the web). Why: a plugin on a compromised or shared machine could otherwise remove protection without proving the account owner's identity.
**Answered by the Architect:** `Desconectar` keeps the device token (a local pause); the flag survives restarts; the web learns the reason through protocol 1.3 (close code 4100) and `Connection.disconnect_reason` (`user | network | outdated | unpaired`) in HTTP contract 2.x. Until that contract exists the web shows only `Desconectada` (S3-10 prepares the text).

The device token is never displayed or logged by the plugin (DPAPI storage, ADR-0004 §8).

---

## 2. AC traceability (slice-plan rev. 7)

| AC | Screens |
|---|---|
| AC-1 invite-only sign-up | S3-03, S3-08a |
| AC-2 confirm e-mail by code before the account exists | S3-03 step 2 |
| AC-3 no password | §1, S3-01, S3-03 (and S3b: §6 for WhatsApp) |
| AC-4 sign in by code, sign out | S3-01, S3-02, user menu `Sair`, S3-06 Card B |
| AC-5 code rules | §1, S3-02, C§1 |
| AC-6 sending limits, cooldown | S3-01, S3-02, S3-04, S3-05, C§1.4 |
| AC-7 non-enumeration | §1, S3-01, S3-02 |
| AC-8 sessions | S3-01 (remember me), S3-06 Card B, `session.ended` handling |
| AC-9 new-device notice | S3-07 (e-mail), S3-06 Card C and delivery notice |
| AC-10 step-up | S3-04, S3-10 (unpair), S5 (accept automation), S3-05 |
| AC-11 change channel | S3-05, S3-06 Card A |
| AC-12 recovery | **Blocked (Q28)**: slots reserved in S3-02 and S3-05, nothing designed |
| AC-13 isolation | §1; every list/detail; admin screens show identity data only; QA two-user tests per screen |
| AC-14 source secret: generate / view once / revoke | S3-09 |
| AC-15 pair / unpair per instance | S3-10, S3-11 |
| AC-16 audit | §1 (events only; view is S10) |
| X-4 secrets | §1, F§8, C§1, S3-03, S3-08a, S3-09, S3-10 |
| X-6 bilingual | all strings keyed; e-mails in the recipient's language; S4 spec |

## 3. Operations and error codes

**Published operations used by this slice** (contract 3.4.1; the Frontend uses the generated client and these `operationId`s only):

| Screen | operationId | Notes |
|---|---|---|
| S3-01 | `requestLoginCode` | `{ email, channel }`; always 202 with a sealed `challenge_id`, `expires_at`, `resend_after_s` |
| S3-02 | `verifyLoginCode`, `resendCode`, `getChallengeDelivery` | verify takes `{ challenge_id, code, remember_me }` and returns `LoginResult` (`new_device`); delivery is `not_reported` for login |
| S3-03 | `lookupInvitation`, `startSignup`, `completeSignup` | token in the body; `completeSignup` takes `locale`, `time_zone`, `remember_me` |
| S3-04 | `requestStepUp`, `getChallengeDelivery` | proof travels in the action request as `step_up` |
| S3-05 | `startChannelChange`, `verifyChannelChangeCode`, `cancelChannelChange`, `getChallengeDelivery` | |
| S3-06 | `getMySecurity`, `listDevices`, `forgetDevice`, `signOutEverywhere`, `logout` | |
| S3-08a, S3-09, S3-10 | `listInvitations`, `createInvitation`, `revokeInvitation`; sources; connections; `pairAdapter` | unchanged; `createInvitation`, `createSource`, `rotateSource`, `createConnection`, `newPairingCode` are `x-step-up: conditional` |
| profile | `getMe`, `updateMe` | |

| Error code (published) | Where | UI |
|---|---|---|
| `auth.code_invalid` | any code screen, any step-up action, `completeSignup` | One message for every verification failure: "Código inválido ou expirado. Confira ou peça um novo." |
| `auth.rate_limited` (429, `Retry-After`) | request, resend, verify | "Muitos pedidos. Aguarde {time}…" with countdown (C§1.4) |
| `auth.step_up_required` | any `x-step-up` operation sent without proof | Opens S3-04; the action is then sent once with the proof |
| `auth.channel_unavailable` | `requestStepUp`, `startChannelChange` | Step-up: "Você não tem um canal confirmado para isso." / WhatsApp not offered yet: the option stays hidden |
| `auth.step_up_channel_not_allowed` | `requestStepUp` | Dialog switches to e-mail: "Neste dispositivo novo, o código precisa ser confirmado no e-mail." |
| `auth.invitation_invalid` | `lookupInvitation`, `startSignup` | The single invalid-invitation state (S3-03) |
| `me.channel_change_too_soon` / `me.channel_change_pending` | `startChannelChange` | Freeze text / pending-change banner (S3-05) |
| `me.channel_unchanged` | `startChannelChange` | "Este já é o seu e-mail atual." |
| `me.invalid_channel_value` | `startChannelChange` | Inline format error under the field |
| `me.channel_change_closed` | `verifyChannelChangeCode`, `cancelChannelChange` | "Esta alteração não está mais aberta." |
| `me.invalid_time_zone` | S3-03, S4 preferences | Inline "Fuso horário inválido. Escolha outro." |
| `auth.unauthenticated` | any call | Session-expired pattern (F§10) |
| `auth.forbidden` | admin routes for a non-admin | Generic not-found screen |
| `common.forbidden_origin` | any write | Generic error with the code (misconfiguration, not a user error) |
| `connections.*`, `sources.*`, `pairing.*` | S3-09 / S3-10 / S3-11 | As in those screens. The plugin uses `pairing.invalid_code`, `pairing.rate_limited` (also for a temporary lock; `pairing.temporarily_locked` does not exist) and `pairing.adapter_version_unsupported` |

Not error codes: `security.code_delivery_failed` is an **audit** code (S3-06 notice). `Connection.disconnect_reason` (S3-10) is **not in the contract yet**.

Removed with the password model (no longer in the contract): `auth.password_*`, `auth.totp_*`, `auth.reset_token_invalid`, `auth.recovery_codes_regenerate`, `verifyLoginTotp`, `verifyLoginRecoveryCode`, `requestPasswordReset`, `confirmPasswordReset`, `changePassword`, `adminCreateResetLink`, `adminResetTotp`.

## 4. Open questions (owner)

| # | Question | Owner | Spec default |
|---|---|---|---|
| U16 | Code format | Architect / user | Policy recommends **Crockford Base32, 8 characters, shown `XXXX-XXXX`**; the Core does the normalization (case, `-`, space, `O→0`, `I/L→1`). `CodeInput` only strips separators, uppercases and limits length from config (C§1.1); format 2 would need a case-sensitive mode |
| Q28 | Recovery when every channel is lost / old channel unreachable | User | Nothing designed; slots reserved (S3-02, S3-05) |
| Q29 | Session lengths; does "remember device" skip the new-device notice | User | Q21 values; notice on every first use of a device |
| Q30 | Step-up per action or time window | User | UI works for both (S3-04) |
| Q33 | Limits and resend cooldown (numbers) | Architect / user | The UI shows whatever the Core sends |
| Q34 | New-device notice content; "sign out of all devices" and the devices list enter S3 (Architect recommends yes) | Architect / user | **Provisional**: S3-06 Card C, S3-07 as in policy §5.3 |
| Q35 | Wider step-up list | Architect / user | The three of AC-10 **plus** the Architect's musts (source URL, pairing code, invitation) marked provisional; enforcement off / max position not designed |
| Q36 | Invite e-mail must equal sign-up e-mail | User | Equal, read-only (S3-03) |
| Q-D | S3-01 channel choice from deployment channels (policy §2/§3): resolved | — | Resolved by the policy |
| M2 / M3 | 72 h hold for a channel change from a new device; persistence step-up by e-mail on new devices | User | Designed as provisional (S3-05, S3-04) |
| Q-G | Should the S3-06 delivery notice ("código não enviado") also be a push/notification? | PO | No: account notice only (S18 AC-3) |
| Q-S3-P1 | Document on `connection.paired` that `visible_accounts` may be empty (accounts arrive after `hello`; the list is rebuilt on `connected`) | Architect | The dialog counts accounts only after `connected` (§ pairing step 2) |
| — | Terms/privacy acceptance at sign-up | User (vision §12) | Slot reserved |

Resolved by policy rev. 2: step-up for channel change (the two codes), "e-mail in use" never revealed, notice to the old address, where the challenge lives (sealed token, kept in memory), no outage banner on public pages, "sign out of all devices" inside S3 (provisional). Resolved earlier and still valid: Architect items in [contract notes §2](../../architecture/contracts/s3-access-sources-pairing.md) for sources, pairing and re-pairing.

## 5. Components used

`CodeInput`, `CodeChallenge`, `Stepper` (new, [components.md](../components.md)); `StepUpDialog` (F§8.2, rewritten), `SecretField`, `ResponsiveDialog`, `PageHeader`, `EmptyState`, `ErrorState`, `StatusBadge`, `DataTable` (S3-08a, S3-09, S3-10 lists, C§4). shadcn: `Card`, `Input`, `Select`, `Checkbox`, `RadioGroup`, `Alert`, `AlertDialog`, `Dialog`, `Drawer`, `Badge`, `Table`, `DropdownMenu`, `Switch`, `Skeleton`, `Sonner`.

## 6. V1-S3b addendum: WhatsApp as a second channel (blocked by U17 and Q31, **not for implementation yet**)

Designed now so the S3 components are final; screens appear only when the Core reports the channel available.

- **Add a number** (S3b AC-1): S3-06 Card A gets a WhatsApp row. Without a number: "Não adicionado" + `Adicionar WhatsApp`. Dialog with `Stepper`: 1 Número (`PhoneInput`: country `Select` + national number; international format, country code required; sent as E.164 string; `autocomplete=tel`, `inputMode=tel`) → 2 Código enviado para esse número (`CodeChallenge` `purpose=channel_add`) → 3 Pronto. Unconfirmed numbers are never used and appear as `Não confirmado` (warning) with `Confirmar`. Optional, never forced by the UI (Q31 recommended option a); a dismissible reminder card in "Primeiros passos" is allowed.
- **Change number** (S3b AC-3): the S3-05 flow with a phone field; codes to the old and the new **number**, plus a code from the other confirmed channel (policy §7). A number owned by another account behaves like a free one until the final confirmation (policy §3).
- **Sign-in** (S3b AC-2): the S3-01 channel `RadioGroup` becomes visible. Cooldown, attempts and limits are identical on both channels.
- **Step-up** (S3b AC-3): S3-04 asks which confirmed channel to use.
- **Delivery failure** (S3b AC-5): shown only in signed-in or invitation-bound flows, read from the challenge's delivery state: "Não foi possível enviar pelo WhatsApp. Enviar por e-mail?" (policy §2.3) + `Receber por e-mail`. The UI never sends the same code through another channel by itself. On public pages the failure is never shown (AC-7).
- **New-device notice** stays an e-mail even after a WhatsApp sign-in (S3b AC-4).
- WhatsApp costs per message are admin-only data (S3b AC-7), not in this spec.
