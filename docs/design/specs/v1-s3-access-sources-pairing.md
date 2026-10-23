# V1-S3 — Access, signal sources and NinjaTrader pairing — Screen spec

> **Owner:** Product Designer (Aquarela @ Dev3) · **Status:** final for implementation · **Revision:** 3 · **Date:** 2026-10-23
> **Slice:** [V1-S3](../../product/release-1/slice-plan.md) (ready) · AC-1, AC-1b, AC-2, AC-3, AC-4, AC-5 · cross-cutting X-2, X-4, X-5, X-6, X-8.
> **Foundations:** [foundations.md](../foundations.md). Section numbers like "F§8" point there.
> **Decided:**
> - **By the PO** ([design-answers-s3-s4.md](../../product/release-1/design-answers-s3-s4.md)): D1 view-once URL, D2 immediate revocation, D4 revoked stays visible, D6 user names the connection, D13 dismissible TOTP banner, D15 USD, D18 states.
> - **By the user on 2026-10-23** ([decisions.md](../../delivery/decisions.md)), all as recommended: **Q17** several named sources · **Q18** unpair allowed after confirmation, accounts become out of sync, nothing closed · **Q19** admin-only invites, single-use, bound to an e-mail, 7 days, manual delivery · **Q20** transactional e-mail for invite and reset only, admin-generated reset link as fallback · **Q21** 12 h idle / 24 h absolute; "keep me signed in" 7 d / 30 d; step-up after 15 min · **Q22** 10 recovery codes, password + code to toggle TOTP, admin-assisted reset.
> **Architecture inputs:**
> - [auth-and-session-policy.md](../../architecture/security/auth-and-session-policy.md): passwords, throttling, step-up list, link lifetimes, session invalidation.
> - [ADR-0004 §8](../../architecture/adr/0004-adapter-protocol-v1.md): pairing code format, lifetime and re-pairing.
>
> **Contract:** no OpenAPI exists yet. Operation names and error codes in §3 are **design proposals** for the Architect to adopt or rename in `contracts/` (spec-first). The Frontend implements against the contract, not this table.
> **Contract:** [contracts/http/openapi.yaml](../../../contracts/http/openapi.yaml) and [s3 contract notes](../../architecture/contracts/s3-access-sources-pairing.md) (Architect). Where this spec and the contract differ, **the contract wins**; tell the Designer.
> **Rev. 3 changes (contract alignment, 2026-10-23):**
> - Invitation and reset links now carry the token in the URL **fragment** (`#t=`) and send it in the POST body.
> - The pairing state "invalidated after 5 wrong attempts" is removed. `pairing_code_invalidated` now has reason `superseded` or `pairing_locked`, and the plugin maps `pairing.temporarily_locked` to its rate-limited state.
> - TOTP enrolment step 1 is replaced by step-up.
> - The time zone is sent at sign-up, and `session.ended` reasons are mapped.
> - Account bindings follow re-pairing by account name.
> - Operation names now come from the OpenAPI.
> **Rev. 2 changes:** Q17–Q22 applied (alternative branches removed). Added "keep me signed in", step-up (F§8.2), admin Users tab (reset link and TOTP reset), reset-success page, re-pairing, and the `outdated` state. Final password, recovery-code and pairing-code formats.
> **Surfaces:** Web (Frontend), NinjaTrader plugin pairing panel (Plugin Engineer, §S3-11). **QA:** QA owner for `tests/e2e/`.

---

## 0. Information architecture for this slice

```
Public (no session)                     App (session) — shadcn Sidebar
/[locale]/login                          Conexões / Connections
/[locale]/login/verify      (TOTP)         ├─ Fontes de sinal      /[locale]/connections/sources
/[locale]/invite#t=…       (sign-up)      └─ NinjaTrader          /[locale]/connections/ninjatrader
/[locale]/forgot-password                Configurações / Settings
/[locale]/reset-password#t=…               ├─ Segurança            /[locale]/settings/security
                                           └─ Preferências         /[locale]/settings/preferences  (S4)
                                         Administração (platform admin only, Q19)
                                           ├─ Convites             /[locale]/admin/invites
                                           └─ Usuários             /[locale]/admin/users
                                         User menu: Idioma · Tema · Segurança · Sair
```

- `[locale]` ∈ `pt-BR`, `en` (S4 spec).
- The app home (`/[locale]`) is a placeholder dashboard until S5/S13. After sign-in, it shows the TOTP banner (S3-07) and a "Primeiros passos" / "Getting started" card: 1 create a signal source, 2 pair NinjaTrader. Each step links to its screen and shows ✓ when done (derived from list counts the API returns; no new rule).
- Every app route requires a session; Next.js middleware only redirects to login (ADR-0001 §9). Authorization is the Core's.

## 1. Global rules for this slice

- **Isolation (AC-2, X-2):** every list and detail shows only the signed-in user's objects. An id of another user's object (in the URL or a request) yields the same "not found" screen as a non-existent id (`ErrorState`, code `common.not_found`). The UI never distinguishes "exists but not yours".
- **Secrets (AC-3, X-4):** F§8. No secret, token or code appears in a URL path, query string, toast, page title or error message. **Invite and reset tokens travel only in the URL fragment** (`#t=<token>`), which browsers never send to the server or in `Referer`. Those pages:
  - read `location.hash` on the client;
  - immediately remove the fragment with `history.replaceState` (so it is not in history, bookmarks or screenshots of the address bar);
  - send the token **in the POST body** (`lookupInvitation`/`acceptInvitation`, `confirmPasswordReset`);
  - set `Referrer-Policy: no-referrer` and load no third-party resources.

  A page opened without a `t` fragment shows the generic invalid state.
- **Audit (X-5):** the user actions below (create/rotate/revoke source, create/pair/unpair connection, enable/disable TOTP) are expected to produce audit entries. The audit view is S10; this slice only needs the events emitted (Core).
- **Duplicate submits:** F§10 "Submitting" applies to every form and confirm button.
- **Session ended elsewhere:** the SSE stream sends `session.ended` with a reason before closing. Open tabs redirect to login with the mapped message: `expired` → `session_expired`; `password_reset`, `password_changed`, `totp_changed`, `admin_totp_reset` → `sessions_ended`; `logout` → `signed_out` (S3-01).
- **Step-up (Q21, auth policy §1):** these actions open the step-up dialog (F§8.2) first when the last authentication is older than 15 minutes: create or rotate a source URL, create a pairing code or unpair, enable/disable TOTP or regenerate recovery codes, and change the password. The Core is the authority. The web opens the dialog when the Core answers `auth.step_up_required`, then retries the original action once.
- **Text:** all strings shown are examples in pt-BR / en. Frontend creates the keys (S4 spec §5 naming); final copy review is with the PO.

---

## S3-01 Login — `/[locale]/login`

**Purpose:** a registered user signs in (AC-1, AC-5).
**Layout:** centered `Card` (max 400 px) on `--muted` background. Top: product name "All Win". Top-right of the page: language switcher + theme switcher (S4). Below the card: `Acesso somente por convite.` / `Access by invitation only.`. No sign-up link (Q6 invite-only).

| Field | Component | Attributes | Validation (client) | API |
|---|---|---|---|---|
| E-mail | `Input type=email` | `autocomplete=username`, `autoFocus` | required; e-mail format | `auth.login` → `email` |
| Senha / Password | `Input type=password` + show/hide toggle button (`aria-pressed`, label "Mostrar senha") | `autocomplete=current-password` | required | `auth.login` → `password` |
| Manter conectado / Keep me signed in | `Checkbox`, **unchecked by default** | Help text under it: "Por até 30 dias neste dispositivo. Não use em computador compartilhado." / "For up to 30 days on this device. Don't use on a shared computer." | — | `auth.login` → `remember_me` (Q21: idle 7 d / absolute 30 d; without it, idle 12 h / absolute 24 h) |

**Actions:** `Entrar` / `Sign in` (primary, submits on Enter). Link `Esqueci minha senha` / `Forgot password` → S3-04.

**Behavior and states**

| State | UI |
|---|---|
| Default | As above. If `?reason=session_expired`: info `Alert` "Sua sessão expirou. Entre de novo." / "Your session expired. Sign in again." If `?reason=signed_out`: "Você saiu da sua conta." If `?reason=sessions_ended` (after TOTP was toggled or the password changed on another device): "Sua sessão foi encerrada por uma mudança de segurança na conta. Entre de novo." |
| Submitting | Button spinner "Entrando…", duplicate submits ignored |
| Success, no TOTP | Redirect to `returnTo` if it is a **same-origin relative path** (else app home) in the user's profile locale (S4) |
| Success, TOTP enabled | Go to S3-02 (the Core returns a pending-2FA state; no full session yet) |
| Invalid credentials | Destructive `Alert`: "E-mail ou senha incorretos." / "Incorrect e-mail or password." **Same message and timing for unknown e-mail, wrong password and disabled user (AC-5).** Password field cleared and focused; e-mail kept |
| Rate limited (`auth.rate_limited`) | "Muitas tentativas. Aguarde {time} e tente de novo." using `Retry-After` (seconds → "30 s" / "2 min"); button disabled with a countdown. The policy's progressive delays (1 s … 60 s) are usually shorter than the time it takes to retype, so short delays show only the spinner. The soft lock for unknown IPs (15 min) shows the same message; it never says the account is locked (AC-5) |
| Network/server error | "Não foi possível conectar. Tente de novo." + code; values kept |
| Already signed in | Visiting `/login` redirects to app home |

**Accessibility:** form has `aria-labelledby` the card title "Entrar"; errors announced by focusing the `Alert` (it has `tabIndex=-1`, `role=alert`). Paste allowed, password managers supported (WCAG 3.3.8).
**AC mapping:** AC-1 (sign in), AC-5 (non-revealing failures), S4 AC-1 (switcher on public pages).

## S3-02 Two-step verification at login — `/[locale]/login/verify`

**Purpose:** second factor when TOTP is enabled (AC-1b). Reachable only in the pending-2FA state; otherwise redirects to login.
**Layout:** same card. Title "Verificação em duas etapas" / "Two-step verification". Text: "Digite o código de 6 dígitos do seu app autenticador." / "Enter the 6-digit code from your authenticator app."

| Field | Component | Attributes | Validation | API |
|---|---|---|---|---|
| Código / Code | `InputOTP` 6 slots (single underlying input) | `inputMode=numeric`, `autocomplete=one-time-code`, `pattern=\d{6}`, `autoFocus` | 6 digits | `auth.verify_totp` → `code` |
| Recovery code (alternate mode, Q22) | `Input` mono, placeholder `XXXXX-XXXXX` | `autocomplete=off`, `autocapitalize=characters`, `spellCheck=false`; accepts with or without the hyphen and spaces, case-insensitive | 10 Crockford Base32 characters after removing separators (auth policy §2) | `auth.verify_recovery_code` → `code` |

**Actions:** submits automatically when the 6th digit is entered (and on Enter); explicit `Verificar` / `Verify` button for clarity. Link `Usar um código de recuperação` / `Use a recovery code` toggles mode. `Voltar` / `Back` → login (cancels the pending state).

| State | UI |
|---|---|
| Submitting | Inputs read-only, spinner; no second auto-submit |
| Wrong code | "Código inválido. Tente de novo." / "Invalid code. Try again."; slots cleared and focused |
| Recovery code used | Success → app home, with a one-time warning `Alert` in the app shell: "Você entrou com um código de recuperação. Restam {n}." (count from API). With 3 or fewer left, it adds `Gerar novos códigos` → S3-06. The Security page always shows the remaining count |
| Code already used in this time window (replay) | Same message as a wrong code |
| Pending state expired (5 min, auth policy §1) | "A verificação expirou. Entre de novo." → back to login, e-mail kept |
| Rate limited | As S3-01 |
| Lost device, no codes | Help text under the form: "Perdeu o acesso ao app e aos códigos? Fale com o administrador, que vai confirmar sua identidade e desativar a verificação." / "Lost your app and codes? Contact the administrator, who will confirm your identity and turn verification off." (Q22: admin-assisted reset, see S3-08b) |

**AC mapping:** AC-1b (sign-in requires the code once enabled).

## S3-03 Accept invitation / create account — `/[locale]/invite#t=<token>`

**Purpose:** an invited person creates their account (AC-1, Q6). The locale of the link is the one chosen when the invite was created (S3-08).
**Entry:** link from the invitation (copied by the admin or e-mailed, Q20), `https://<host>/<locale>/invite#t=<token>`. On load the page reads the token from the fragment, removes the fragment (§1), and validates it with `lookupInvitation` (token in the body). The token stays in memory only until `acceptInvitation`.

| State | UI |
|---|---|
| Validating | Card skeleton |
| Invalid / used / expired / revoked | `ErrorState`: "Este convite não é válido. Ele pode ter expirado ou já ter sido usado. Peça um novo convite." / "This invitation isn't valid. It may have expired or already been used. Ask for a new one." **One message for all causes** (does not reveal which). No form |
| Valid | Form below |
| Already signed in | `Alert`: "Você está conectado como {email}. Saia para aceitar este convite." with `Sair` button |

| Field | Component | Validation | API |
|---|---|---|---|
| E-mail | `Input` read-only, prefilled from the invite (Q19: bound to one e-mail) | — | from `lookupInvitation` |
| Senha / Password | `Input type=password` + show/hide, `autocomplete=new-password` | Auth policy §2: 12 to 128 characters (any characters, spaces allowed), no composition rules. The Core rejects common/breached passwords and passwords containing the e-mail name or "allwin". Live hint under the field: "De 12 a 128 caracteres. Uma frase longa é mais fácil de lembrar." Length counter, no strength meter. Client checks only length; the rest is the Core's | `auth.accept_invitation` → `password` |
| Idioma / Language | `Select` pt-BR / English, prefilled with the current locale | required | → `locale` (profile, S4) |
| Fuso horário / Time zone | Read-only line "Fuso horário: {zone} (GMT-03:00) · `Alterar`", prefilled from the browser (`Intl.DateTimeFormat().resolvedOptions().timeZone`). `Alterar` expands the S4 §4 time-zone `Select` | required by the contract; `me.invalid_time_zone` → inline error | → `time_zone` |

No password-confirmation field: the show/hide toggle is the typo check (NIST 800-63B practice), and password reset exists (AC-5). No display-name field: no requirement asks for one. **Terms of use / privacy acceptance:** **[Open]**, vision §12: required before launch, not before dev. A checkbox slot is reserved above the button.

**Actions:** `Criar conta` / `Create account`.

| Result | UI |
|---|---|
| Success | Signed in automatically → app home (`Primeiros passos` + TOTP banner S3-07), in the chosen language |
| `auth.password_too_short` / `auth.password_too_long` / `auth.password_breached` | Inline field error: "Use pelo menos 12 caracteres." / "Use no máximo 128 caracteres." / "Esta senha é comum, aparece em vazamentos conhecidos ou contém seu e-mail. Escolha outra." |
| Invite became invalid meanwhile | Switch to the invalid state above |

**AC mapping:** AC-1 (invite-only: refused without a valid, unused, unexpired invite, with a clear message).

## S3-04 Forgot password — `/[locale]/forgot-password` (AC-5, Q20)

Q20 decided: transactional e-mail for reset (and invitations) only; an admin-generated reset link is the fallback.
- Field: E-mail (`autocomplete=username`). Action `Enviar link` / `Send link`.
- Result is **always the same** (AC-5; the Core always answers 202): "Se houver uma conta com este e-mail, você receberá um link em alguns minutos. O link vale por 30 minutos. Não chegou? Confira o spam ou peça um link ao administrador." / "If an account exists for this e-mail, you'll receive a link in a few minutes. The link is valid for 30 minutes. Didn't get it? Check spam or ask the administrator for a link."
- A new request invalidates earlier links (auth policy §3). The page says nothing about it, to avoid confusing users who click an older e-mail. The older link simply shows the generic invalid state.
- Rate limited: same generic message (no different response).
- **E-mail template** (Frontend/Core, both languages, in the profile language): subject "Redefinir sua senha do All Win" / "Reset your All Win password"; body: one sentence, the button link, validity "30 minutos", and "Se você não pediu, ignore este e-mail. Sua senha continua a mesma." No other links, no tracking pixels.

## S3-05 Reset password — `/[locale]/reset-password#t=<token>` (AC-5)

- Same fragment handling as S3-03 (§1). There is no lookup call: the form is shown directly, and an invalid, used, expired or superseded token is reported on submit with one generic message (`auth.reset_token_invalid`): "Este link de redefinição não é válido. Ele pode ter expirado (vale 30 minutos) ou já ter sido usado. Peça um novo link." + `Pedir novo link` → S3-04. Validity: 30 minutes, single use.
- Field: Nova senha / New password (rules as S3-03). Action `Redefinir senha` / `Reset password`.
- **Success page** (not a redirect: auth policy §3 asks for review links). Success `Card`:
  - "Senha redefinida. Por segurança, todas as suas sessões foram encerradas, inclusive neste navegador." / "Password reset. For security, all your sessions were ended, including this browser."
  - Primary `Entrar` → login.
  - Info `Alert`: "Suspeita de acesso indevido? Depois de entrar, revise suas Fontes de sinal e Conexões NinjaTrader e revogue ou despareie o que não reconhecer. A redefinição de senha não desconecta o NinjaTrader nem invalida as URLs dos alertas." with links to S3-09 and S3-10 (they pass through login with `returnTo`).
- TOTP, if enabled, is still required at the next sign-in.

## S3-06 Security settings — `/[locale]/settings/security` (AC-1b)

**Layout:** `PageHeader` "Segurança" / "Security"; stacked `Card`s.

### Card A — Two-step verification
- Status line: `Badge` `Ativada` (success, `ShieldCheck`) / `Desativada` (warning, `ShieldAlert`) + text "Recomendado: protege suas contas de trading mesmo que sua senha vaze." / "Recommended: protects your trading accounts even if your password leaks."
- When enabled: "Códigos de recuperação restantes: {n} de 10" + `Gerar novos códigos` / `Generate new codes`. With 3 or fewer left, the line uses `--status-warning`.
- Action: `Ativar` / `Enable` or `Desativar` / `Disable`.
- Note under the actions: "Ativar ou desativar encerra suas sessões em outros dispositivos." / "Turning this on or off signs you out on other devices." (auth policy §3).

**Enable flow:** `ResponsiveDialog`, 3 steps with a step indicator ("Etapa 2 de 3"). `startTotpSetup` is a step-up operation, so re-authentication is the F§8.2 step-up dialog, shown before step 1 when the last authentication is older than 15 minutes. Closing at any step before step 2 succeeds = nothing changes. The enrolment secret expires after 10 minutes (`auth.totp_setup_expired` → "A configuração expirou. Comece de novo." + `Recomeçar`).

| Step | Content | Validation / API |
|---|---|---|
| 1 Scan | QR code (`img` with `alt` "QR code para o app autenticador") + "Não consegue escanear? Digite a chave:" + setup key in a `SecretField` (F§8; grouped in blocks of 4 when revealed) | `startTotpSetup` returns `otpauth_uri`, `secret`, `expires_at`; never cached |
| 2 Confirm code | `InputOTP` 6 digits | `confirmTotpSetup` → `code`. Wrong: "Código inválido. Confira se o relógio do celular está certo e tente de novo." |
| 3 Recovery codes (Q22) | 10 codes, each shown `XXXXX-XXXXX` (10 Crockford Base32 characters), in a 2-column mono list; actions `Copiar todos` / `Baixar .txt` (client-side file from the response, filename `all-win-recovery-codes.txt`, one code per line plus the generation date); warning `Alert`: "Guarde estes códigos em um lugar seguro. Cada um funciona uma vez. Eles não serão mostrados de novo." Required checkbox "Guardei meus códigos" enables `Concluir` / `Done` | Codes shown once; cleared from state on close. Closing (Esc, overlay, back) before the checkbox opens the guard "Você ainda não confirmou que guardou os códigos. Fechar mesmo assim?" (TOTP is already on at this point) |

Success toast: "Verificação em duas etapas ativada." The TOTP banner (S3-07) disappears.

**Regenerate recovery codes** (auth policy §2, step-up): dialog with Senha + Código atual (`InputOTP`) → `auth.recovery_codes_regenerate`. The warning says "Os códigos anteriores deixam de funcionar." Then the same step-3 view. *(Regeneration comes from the Architect's policy; PO to confirm that it is in the V1 scope, §4.)*

**Disable flow:** `AlertDialog`: title "Desativar a verificação em duas etapas?"; body: "Sua conta ficará protegida só pela senha, e suas sessões em outros dispositivos serão encerradas." Fields: Senha + Código atual (`InputOTP`, with a link to use a recovery code instead). Confirm `Desativar` (destructive). Errors as above. AC-1b: requires re-authentication (Q22: password + code).

### Card B — Session
- `Sair` / `Sign out` button (same as the user menu).
- Text (Q21): "Sua sessão termina após 12 horas sem uso ou 24 horas depois de entrar. Com “Manter conectado”: 7 dias sem uso ou 30 dias." / "Your session ends after 12 hours of inactivity or 24 hours after sign-in. With “Keep me signed in”: 7 days of inactivity or 30 days." Values come from the Core config, not hard-coded in the catalogs (variables in the message).
- Only actions you take keep the session alive. An open dashboard that only receives live updates still expires (auth policy §1). The text says: "Apenas deixar a tela aberta não mantém a sessão ativa." / "Just leaving the page open doesn't keep the session active."

### Card C — Password
- `Alterar senha` / `Change password` (step-up per auth policy §1): dialog with Senha atual + Nova senha (S3-03 rules). Success: "Senha alterada. Suas sessões em outros dispositivos foram encerradas." *(The policy lists this action; PO to confirm V1 scope, §4.)*
- "Sign out of all devices" is not offered (auth policy §1: PO item, not in S3).

## S3-07 TOTP suggestion banner (AC-1b "strongly suggested", D13)

- Where: app shell, below the top bar, on every app page, while TOTP is disabled.
- Component: `Alert` (info, `ShieldAlert` icon): "Proteja sua conta com a verificação em duas etapas." / "Protect your account with two-step verification." Actions: `Ativar agora` / `Enable now` (→ S3-06, opens the enable dialog) and `Agora não` / `Not now` (icon-button `X` with name "Dispensar aviso").
- Dismissal persists **per device** (local storage, wrapped in try/catch; failure = shows again). It reappears on a new device or browser. The Security card keeps the permanent recommendation. No API change needed.
- Not shown on public pages or inside dialogs. On mobile it is a single line + one action, with the dismiss button ≥ 44 px.

## S3-08 Administration (platform admin) — AC-1, AC-5, Q19, Q20, Q22

Q19 decided: only a platform admin (`is_platform_admin`, held by the user) issues invitations. They are single use, bound to one e-mail, valid 7 days and delivered manually; Q20 adds a transactional e-mail.
**Visibility:** the sidebar group "Administração" and its routes exist only for admins. A non-admin gets the generic not-found screen (does not reveal the route). Admin screens show identity data only (e-mail, dates, TOTP on/off). **Never** another user's accounts, sources, connections or trades (X-2).

### S3-08a Invitations — `/[locale]/admin/invites`

**List** (`Table` ≥ `md`, cards below): E-mail · Status (`Pendente` info / `Usado` success / `Expirado` neutral / `Revogado` neutral) · Criado em · Expira em (date-time with zone, F§6.4) · Actions (`Revogar` for pending).
**Empty:** "Nenhum convite ainda. Convide alguém para usar o All Win." + `Convidar`.

**Create** (`ResponsiveDialog` "Novo convite"):
| Field | Validation | API |
|---|---|---|
| E-mail | required, e-mail format; if an account or pending invite exists for it: "Já existe uma conta ou um convite pendente para este e-mail." (admin-only screen, so this disclosure is acceptable; PO to confirm, §4) | `invites.create` → `email` |
| Idioma do convite / Invitation language | pt-BR / English, default = admin's locale | → `locale` (link, e-mail and landing-page language, S4) |

Result step:
- "Enviamos o convite para {email}." (Q20). If sending fails: warning `Alert` "Não foi possível enviar o e-mail. Copie o link e envie você mesmo." The invite stays valid.
- The invite link in a `SecretField` with `Copiar`, for manual delivery (Q19). It is a credential: **view-once**, since the Core stores only its SHA-256 (auth policy §3). Plus "Expira em {date time zone}".
- Closing without copying shows no guard when the e-mail was sent. When sending failed, it shows the same guard as S3-09 create.

**Revoke:** `AlertDialog` "Revogar o convite para {email}? O link deixará de funcionar." → `invites.revoke`.

### S3-08b Users — `/[locale]/admin/users` (fallbacks for Q20 and Q22)

**List:** E-mail · Criado em · Verificação em duas etapas (`Ativada` / `Desativada`) · Actions. Search by e-mail. Empty is impossible (the admin is listed).

**Action `Gerar link de redefinição de senha`** (Q20 fallback): `AlertDialog` "Gerar um link de redefinição para {email}? Links anteriores deixam de funcionar." → `admin.users.create_reset_link`. Result: link in a `SecretField` (view-once), "Vale por 30 minutos e funciona uma vez." + "Envie por um canal em que você confirmou a identidade da pessoa."

**Action `Desativar verificação em duas etapas`** (Q22 admin-assisted reset after an identity check): `AlertDialog` (destructive):
- Body: "Use isto só depois de confirmar a identidade da pessoa por um canal independente (por exemplo, uma chamada de vídeo). A conta ficará protegida só pela senha, e as sessões dela serão encerradas."
- Required checkbox "Confirmei a identidade de {email}". The check itself happens outside the product; the UI only records that the admin attests to it.
- Confirm → `admin.users.reset_totp`. The action is audited (X-5) and visible to the affected user in their own audit log.
- The admin's own row hides both actions; the admin uses their own Security page.

Both admin actions require step-up (F§8.2). **[Architect: add them to the policy's step-up list, §4.]**

## S3-09 Signal sources — `/[locale]/connections/sources` (AC-3, S1 AC-2/AC-6, Q17)

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

### Create source (`ResponsiveDialog`, 2 steps; step-up first if needed, §1)
1. **Name:** `Input` "Nome da fonte" / "Source name", required, 1–60 chars, trimmed; example placeholder "Ex.: Strikes MNQ". Duplicate name among the user's active sources: inline "Você já tem uma fonte com este nome." **[Proposal: names unique among active sources; PO to confirm, §4]**. Action `Criar` → `sources.create { name }`.
2. **Copy the URL (view-once):**
   - Warning `Alert`: "Copie a URL agora. Por segurança, ela não será mostrada de novo. Se perdê-la, gere uma nova." / "Copy the URL now. For security, it won't be shown again. If you lose it, rotate to get a new one."
   - `SecretField` (F§8) with the full `webhook_url` from the response: masked by default, `Revelar`, `Copiar`.
   - Steps text: "1. No TradingView, abra o alerta → Notificações → Webhook URL. 2. Cole a URL. 3. Salve o alerta." **[Copy to validate with PO/S17 against TradingView's current UI]**
   - Automation note (info `Alert`, O1): "Ordens desta fonte são automatizadas. Contas de firmas que proíbem automação só as recebem se você aceitar isso na conta." with a link to Accounts (S5).
   - `Concluir` / `Done`. If the user tries to close (button, Esc, overlay, back gesture) **without having used Copiar**, an `AlertDialog` asks: "Você ainda não copiou a URL. Se fechar agora, precisará gerar uma nova. Fechar mesmo assim?" (`Voltar e copiar` focused / `Fechar`).
   - On close: secret wiped from state; list refetched; toast "Fonte criada." (no URL in the toast).
   - Errors at step 1: inline/`Alert`; nothing created. Network failure **after** submit with unknown outcome: "Não sabemos se a fonte foi criada. Atualize a lista antes de tentar de novo." + `Atualizar lista` (prevents duplicate sources; the Core's create is not assumed idempotent).

### Rotate (`Gerar nova URL`): D2, immediate
- `AlertDialog`: title "Gerar nova URL para “{name}”?"; body: "A URL atual para de funcionar **imediatamente**. Alertas que chegarem com ela serão rejeitados e registrados na auditoria. Depois, cole a nova URL no alerta do TradingView." Confirm `Gerar nova URL` (destructive). Step-up if needed.
- Then the same step-2 view-once dialog as create. Toast "URL da fonte “{name}” trocada."
- `sources.rotate { id }` → returns the new `webhook_url` once.

### Revoke: D4, stays visible
- `AlertDialog`: "Revogar a fonte “{name}”?"; body: "Alertas com esta URL serão rejeitados. A fonte continua no histórico como revogada. Isso não pode ser desfeito; para voltar a receber sinais, crie uma nova fonte." Confirm `Revogar fonte` (destructive) → `sources.revoke { id }`.
- Row becomes `Revogada`, hidden unless "Mostrar revogadas" is on. Actions on a revoked row: none (name kept for history).
- **[Dependency S7]:** if routing rules reference the source, the dialog lists them ("Rotas que usam esta fonte deixarão de receber sinais: …"). Until S7 exists, no list.

### Rename
- Dialog with the name field (same validation) → `sources.update { id, name }`. Does not change the URL; no step-up.

**Accessibility:** the masked URL cell has `aria-label` "URL secreta, oculta, identificador {key_id}" (it reads the key id, not the bullets). Row actions button: "Ações para {name}".
**AC mapping:** AC-3 (generate, view once, revoke; old secret rejected: surfaced by copy and audit), AC-2 (own sources only), X-4 (masking, no secret in toasts/URL/logs), X-5 (actions audited).

## S3-10 NinjaTrader connections — `/[locale]/connections/ninjatrader` (AC-4, Q18)

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
**Disconnected instance:** row shows danger badge + text "O plugin não está conectado. Ordens para as contas desta instância não serão executadas." (consequence wording; out-of-sync handling is S11).
**Outdated instance:** warning badge + "Atualize o plugin para voltar a conectar." + link to the download page **[Dependency]**.

### Create connection + pairing (`ResponsiveDialog`, stays open while waiting; step-up first if needed, §1)
1. **Name:** `Input` "Nome da conexão" / "Connection name", required, 1–60 chars, placeholder "Ex.: PC de casa, VPS Chicago". → `connections.create { name }`, which returns the connection, the `pairing_code` and `expires_at`.
2. **Pairing code** (ADR-0004 §8: 8 Crockford Base32 characters shown `XXXX-XXXX`, 10 minutes, single use, one active code per connection):
   - Shown **visibly** (it must be typed into another app): `text-3xl` mono, letter-spaced, e.g. `K7QM-4D2P`, with a `Copiar` button. It is short-lived and single use, so it is not masked; the dialog says "Não compartilhe este código." / "Don't share this code."
   - Countdown: "Expira em 9:41" from the server's `expires_at` (`aria-live=off`; a polite announcement only at 1 min left and at expiry).
   - Instructions: "No NinjaTrader: abra a janela All Win, clique em Parear e digite o código. Letras maiúsculas ou minúsculas, com ou sem hífen." **[Menu path to validate with the Plugin Engineer after the NT8 spike]**
   - Waiting indicator: "Aguardando o plugin…" with a spinner (static text under reduced motion).
   - **Paired** (SSE `connection.paired`): success state with `CircleCheck`: "“{name}” conectada. {n} contas encontradas: Sim101, …" + `Vincular contas` (→ S5) and `Concluir`. Focus moves to the success heading.
   - **Expired:** "O código expirou." + `Gerar novo código` (→ `connections.new_pairing_code { id }`).
   - **Code invalidated** (SSE `connection.pairing_code_invalidated`, by `reason`):
     - `superseded`: a newer code was generated for this connection (e.g. in another tab or by "Mover para outra máquina"). Info `Alert`: "Um código mais novo foi gerado para esta conexão. Este não vale mais." + `Gerar novo código`. The displayed code is replaced by "—" and the countdown stops.
     - `pairing_locked`: pairing is temporarily locked for everyone after too many wrong codes (global protection, contract notes §2). Warning `Alert`: "O pareamento está temporariamente bloqueado por excesso de tentativas erradas. Aguarde alguns minutos e gere um novo código." + `Gerar novo código` (if the Core still refuses, the button shows the returned error).
   - Wrong codes typed in the plugin are **not** reported to the web: the Core cannot tie a wrong code to a connection. The plugin shows them (S3-11).
   - **Closing while waiting:** allowed without a guard (no secret is lost: a new code can be generated). The connection stays in the list as `Aguardando pareamento`.

### Move to another machine (re-pairing, ADR-0004 §8)
- `AlertDialog`: "Mover “{name}” para outra máquina?"; body: "Você vai receber um novo código. Quando ele for usado, a instância atual será desconectada e não receberá mais ordens." Confirm `Gerar código` → step-up if needed → same pairing step 2 as above.
- Until the new code is used, the current instance keeps working. On success: toast "“{name}” agora está em outra máquina." Account bindings **follow the connection by platform account name** (contract notes §2). Bound accounts the new instance does not report become out of sync (`sync.account_not_found_after_repair`). The connection row then lists them as "Não encontrada na nova instância" (danger) with `Vincular de novo` (→ S5) and a link to the out-of-sync view (S11).

### Unpair (`Desparear`): Q18 decided
Allowed after a confirmation that lists the affected accounts; the accounts become out of sync; nothing is closed automatically.
- Step-up if needed, then `AlertDialog` title: "Desparear “{name}”?"
- Body (always): "O plugin desta instância será desconectado imediatamente e não receberá mais ordens. Para reconectar, crie uma nova conexão."
- If any of the instance's accounts is bound to an All Win account **[Dependency S5 data]**: list "Estas contas ficarão fora de sincronia e deixarão de receber ordens: APEX-12345-01, Sim101."
- If any has an open position **[Dependency S11 position data]**: danger `Alert` inside the dialog: "Há posição aberta em: APEX-12345-01 (2 contratos). **A posição continua aberta no NinjaTrader**, sem monitoramento nem proteção de risco do All Win. Nada será fechado automaticamente." Confirmation requires a checkbox "Entendo que a posição continuará aberta" before `Desparear` enables.
- Confirm `Desparear` (destructive) → `connections.unpair { id }`. Row → `Despareada`. Other connections are unaffected (AC-4); the dialog says nothing about them.
- Unpaired connections stay listed (history) under "Mostrar despareadas", like revoked sources. No actions on them.

**AC mapping:** AC-4 (pair per instance, own credential, unpair; unpaired receives nothing; revoking one does not affect others), AC-2, X-4 (device token never displayed), X-5.

## S3-11 NinjaTrader plugin: pairing panel (Plugin Engineer)

The plugin UI follows NinjaTrader 8 conventions (NTWindow/WPF, NinjaTrader skin colors, not shadcn). A detailed window spec is owed after the NT8 spike. This section fixes **states and copy** so web and plugin agree. Language follows the web profile (Q11; received in `welcome`). Before the first pairing, the plugin uses the Windows UI language if pt or en, else pt-BR (S4 §6).

| State | Plugin shows | Action |
|---|---|---|
| Not paired | "Não pareado. Gere um código em All Win → Conexões → NinjaTrader." | Code field (accepts 8 characters, case-insensitive, ignores spaces and hyphens, displays as `XXXX-XXXX`) + `Parear` |
| Pairing | "Pareando…" | Disabled |
| Invalid / expired / used / superseded code (`pairing.invalid_code`) | "Código inválido ou expirado. Confira o código ou gere um novo no All Win." (one message for all causes) | Field kept for correction |
| Rate limited (`pairing.rate_limited`) or pairing temporarily locked (`pairing.temporarily_locked`) | "Muitas tentativas. Aguarde {time} e tente de novo." (`Retry-After` when present; otherwise "alguns minutos") | Disabled with countdown |
| Plugin version unsupported at pairing (`pairing.adapter_version_unsupported`) | Same as "Update required" below | — |
| Paired + connected | Connection name (from Core), "Conectado", account count | `Desparear` (local: forgets the token after confirmation) |
| Disconnected / reconnecting | "Sem conexão com o All Win. Tentando reconectar…" + last connected time | — |
| Revoked (unpaired in the web, or re-paired on another machine; close code 4002) | "Esta instância foi despareada ou movida para outra máquina no All Win. Gere um novo código para conectar." | Back to Not paired |
| Update required (4003) | "Versão do plugin não suportada. Atualize o plugin." | Link to download page (**[Dependency]**) |

The device token is never displayed or logged by the plugin (DPAPI storage, ADR-0004 §8).

---

## 2. AC traceability

| AC | Screens |
|---|---|
| AC-1 invite-only sign-up, sign in/out | S3-01, S3-03, S3-08a, user menu "Sair" |
| AC-1b optional TOTP, strongly suggested, re-auth to disable | S3-02, S3-06, S3-07, S3-08b (admin reset) |
| AC-2 isolation | §1 global rule; every list/detail; admin screens show identity data only; QA two-user tests per screen |
| AC-3 generate / view once / revoke source secret | S3-09 |
| AC-4 pair / unpair per instance | S3-10, S3-11 |
| AC-5 password reset, session expiry, non-revealing failures | S3-01, S3-04, S3-05, S3-06 Card B, S3-08b, F§10 session-expired pattern |
| X-4 secrets | F§8, S3-03/05 token handling, S3-06, S3-08, S3-09, S3-10 |
| X-6 bilingual | all strings keyed; e-mails in the recipient's language; S4 spec |

## 3. Operations and error codes (contract)

The design operation names of rev. 2 map 1:1 to OpenAPI `operationId`s; the table is in [contract notes §1](../../architecture/contracts/s3-access-sources-pairing.md). The Frontend uses the generated client and the `operationId`s only.

**How the web shows error codes that the contract added:**

| Code | Where | UI |
|---|---|---|
| `auth.unauthenticated` | any call | Session-expired pattern (F§10) |
| `auth.forbidden` | admin routes for a non-admin | Generic not-found screen (does not reveal the route) |
| `auth.totp_already_enabled` | S3-06 enable | Refetch security state; toast "A verificação já está ativada." |
| `auth.totp_setup_expired` | S3-06 step 2 | "A configuração expirou. Comece de novo." + `Recomeçar` |
| `auth.reset_token_invalid` | S3-05 submit | Generic invalid-link message + `Pedir novo link` |
| `connections.name_taken` | S3-10 create/rename | Inline "Você já tem uma conexão com este nome." |
| `connections.unpaired` | S3-10 actions on an unpaired row (stale list) | Refetch; toast "Esta conexão já foi despareada." |
| `connections.open_positions_not_acknowledged` | S3-10 unpair | Show the open-position `Alert` + required checkbox (the list may be newer than the dialog); keep the dialog open |
| `sources.revoked` | S3-09 rotate/rename a revoked source (stale list) | Refetch; toast "Esta fonte já foi revogada." |
| `pairing.invalid_code`, `pairing.rate_limited`, `pairing.temporarily_locked`, `pairing.adapter_version_unsupported` | plugin only | S3-11 |
| `me.invalid_time_zone` | S3-03, S4 preferences | Inline "Fuso horário inválido. Escolha outro." |
| `common.forbidden_origin` | any write | Generic error with the code (indicates a misconfiguration, not a user error) |

`connections.pairing_code_expired` is not returned; the web derives expiry from `expires_at` (S3-10).

## 4. Open questions (owner)

| Question | Owner | Spec default |
|---|---|---|
| Is "regenerate recovery codes" (auth policy §2) in V1 scope? | PO | Included |
| Is "change password" (auth policy §1 step-up list) in V1 scope? | PO | Included |
| Are source names unique among active sources? | PO | Yes |
| Is showing "an account or pending invite exists" OK on the admin-only invite screen? | PO | Yes |
| Terms/privacy acceptance at sign-up | User (vision §12) | Slot reserved |

Architect items from rev. 2 are resolved in [contract notes §2](../../architecture/contracts/s3-access-sources-pairing.md): the step-up list now includes accepting automation and the admin actions, and account bindings follow re-pairing by name.
