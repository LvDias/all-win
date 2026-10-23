# V1-S3 — Access, signal sources and NinjaTrader pairing — Screen spec

> **Owner:** Product Designer (Aquarela @ Dev3) · **Status:** proposed, pending PO review · **Revision:** 1 · **Date:** 2026-10-23
> **Slice:** [V1-S3](../../product/release-1/slice-plan.md) (ready) · AC-1, AC-1b, AC-2, AC-3, AC-4, AC-5 · cross-cutting X-2, X-4, X-5, X-6, X-8.
> **Foundations:** [foundations.md](../foundations.md). Section numbers like "F§8" point there.
> **Decided by PO (2026-10-23):** D1 view-once URL, D2 immediate revocation, D4 revoked stays visible, D6 user names the connection, D13 dismissible TOTP banner, D15 USD, D18 states.
> **Pending user decisions (slice-plan §7):** **Q17** sources per user · **Q18** unpair with destinations/open position · **Q19** who issues invites · **Q20** transactional e-mail · **Q21** session length · **Q22** TOTP recovery. Each screen marks where an answer changes it; the recommended option is drawn as the default.
> **Contract:** no OpenAPI exists yet. Operation names and error codes below are **design proposals** for the Architect to adopt or rename in `contracts/` (spec-first). The Frontend implements against the contract, not this table.
> **Surfaces:** Web (Frontend), NinjaTrader plugin pairing panel (Plugin Engineer, §S3-11). **QA:** QA owner for `tests/e2e/`.

---

## 0. Information architecture for this slice

```
Public (no session)                     App (session) — shadcn Sidebar
/[locale]/login                          Conexões / Connections
/[locale]/login/verify      (TOTP)         ├─ Fontes de sinal      /[locale]/connections/sources
/[locale]/invite/[token]    (sign-up)      └─ NinjaTrader          /[locale]/connections/ninjatrader
/[locale]/forgot-password   (Q20)        Configurações / Settings
/[locale]/reset-password/[token]           ├─ Segurança            /[locale]/settings/security
                                           └─ Preferências         /[locale]/settings/preferences  (S4)
                                         Administração (admin role only, Q19)
                                           └─ Convites             /[locale]/admin/invites
                                         User menu: Idioma · Tema · Segurança · Sair
```

- `[locale]` ∈ `pt-BR`, `en` (S4 spec).
- The app home (`/[locale]`) is a placeholder dashboard until S5/S13. After sign-in, it shows the TOTP banner (S3-07) and a "Primeiros passos" / "Getting started" card: 1 create a signal source, 2 pair NinjaTrader. Each step links to its screen and shows ✓ when done (derived from list counts the API returns; no new rule).
- Every app route requires a session; Next.js middleware only redirects to login (ADR-0001 §9). Authorization is the Core's.

## 1. Global rules for this slice

- **Isolation (AC-2, X-2):** every list and detail shows only the signed-in user's objects. An id of another user's object (in the URL or a request) yields the same "not found" screen as a non-existent id (`ErrorState`, code `common.not_found`). The UI never distinguishes "exists but not yours".
- **Secrets (AC-3, X-4):** F§8. No secret, token or code appears in a URL, query string, toast, page title or error message. Invite and reset tokens arrive in the URL path by necessity: those pages call `history.replaceState` to drop the token from the visible URL after reading it, set `Referrer-Policy: no-referrer`, and load no third-party resources.
- **Audit (X-5):** the user actions below (create/rotate/revoke source, create/pair/unpair connection, enable/disable TOTP) are expected to produce audit entries. The audit view is S10; this slice only needs the events emitted (Core).
- **Duplicate submits:** F§10 "Submitting" applies to every form and confirm button.
- **Text:** all strings shown are examples in pt-BR / en. Frontend creates the keys (S4 spec §5 naming); final copy review is with the PO.

---

## S3-01 Login — `/[locale]/login`

**Purpose:** a registered user signs in (AC-1, AC-5).
**Layout:** centered `Card` (max 400 px) on `--muted` background. Top: product name "All Win". Top-right of the page: language switcher + theme switcher (S4). Below the card: `Acesso somente por convite.` / `Access by invitation only.`. No sign-up link (Q6 invite-only).

| Field | Component | Attributes | Validation (client) | API |
|---|---|---|---|---|
| E-mail | `Input type=email` | `autocomplete=username`, `autoFocus` | required; e-mail format | `auth.login` → `email` |
| Senha / Password | `Input type=password` + show/hide toggle button (`aria-pressed`, label "Mostrar senha") | `autocomplete=current-password` | required | `auth.login` → `password` |

**Actions:** `Entrar` / `Sign in` (primary, submits on Enter). Link `Esqueci minha senha` / `Forgot password` → S3-04 (Q20).

**Behavior and states**

| State | UI |
|---|---|
| Default | As above. If `?reason=session_expired`: info `Alert` "Sua sessão expirou. Entre de novo." / "Your session expired. Sign in again." If `?reason=signed_out`: "Você saiu da sua conta." If `?reason=password_reset`: success `Alert` "Senha redefinida. Entre com a nova senha." |
| Submitting | Button spinner "Entrando…", duplicate submits ignored |
| Success, no TOTP | Redirect to `returnTo` if it is a **same-origin relative path** (else app home) in the user's profile locale (S4) |
| Success, TOTP enabled | Go to S3-02 (the Core returns a pending-2FA state; no full session yet) |
| Invalid credentials | Destructive `Alert`: "E-mail ou senha incorretos." / "Incorrect e-mail or password." **Same message and timing for unknown e-mail, wrong password and disabled user (AC-5).** Password field cleared and focused; e-mail kept |
| Rate limited | "Muitas tentativas. Tente de novo em {minutes} min." from `Retry-After` if provided; button disabled until then (countdown) |
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
| Recovery code (alternate mode, **Q22 (a)**) | `Input` mono | `autocomplete=off`, accepts with/without hyphen, case-insensitive | non-empty | `auth.verify_recovery_code` → `code` |

**Actions:** submits automatically when the 6th digit is entered (and on Enter); explicit `Verificar` / `Verify` button for clarity. Link `Usar um código de recuperação` / `Use a recovery code` toggles mode (**hidden if Q22 = (b)**). `Voltar` / `Back` → login (cancels the pending state).

| State | UI |
|---|---|
| Submitting | Inputs read-only, spinner; no second auto-submit |
| Wrong code | "Código inválido. Tente de novo." / "Invalid code. Try again."; slots cleared and focused |
| Recovery code used | Success → app home, with a one-time warning `Alert` in the app shell: "Você entrou com um código de recuperação. Restam {n}." (count from API). The Security page always shows the remaining count. Whether the user can **regenerate** codes is **[Open → Q22]** (not in the recommendation; flagged to PO) |
| Pending state expired | "A verificação expirou. Entre de novo." → back to login |
| Rate limited | As S3-01 |
| Lost device, no codes | Help text under the form: "Perdeu o acesso ao app e aos códigos? Fale com o administrador." / "Lost your app and codes? Contact the administrator." (**Q22 (a)**: admin-assisted reset; the admin path is out of this slice's UI unless Q19/Q22 add it) |

**AC mapping:** AC-1b (sign-in requires the code once enabled).

## S3-03 Accept invitation / create account — `/[locale]/invite/[token]`

**Purpose:** an invited person creates their account (AC-1, Q6). The locale of the link is the one chosen when the invite was created (S3-08).
**Entry:** link from the invitation (copied by the admin or e-mailed, Q20). The page validates the token on load.

| State | UI |
|---|---|
| Validating | Card skeleton |
| Invalid / used / expired / revoked | `ErrorState`: "Este convite não é válido. Ele pode ter expirado ou já ter sido usado. Peça um novo convite." / "This invitation isn't valid. It may have expired or already been used. Ask for a new one." **One message for all causes** (does not reveal which). No form |
| Valid | Form below |
| Already signed in | `Alert`: "Você está conectado como {email}. Saia para aceitar este convite." with `Sair` button |

| Field | Component | Validation | API |
|---|---|---|---|
| E-mail | `Input` read-only, prefilled from the invite (**Q19 (a)**: bound to one e-mail) | — | from `auth.get_invitation` |
| Senha / Password | `Input type=password` + show/hide, `autocomplete=new-password` | Min length 12, max 128, no composition rules, rejected if in a breached/common list (Core check) **[Proposal D10, Architect to confirm]**. Live hint under the field: "Mínimo de 12 caracteres. Uma frase longa é mais fácil de lembrar." Length counter, no strength meter | `auth.accept_invitation` → `password` |
| Idioma / Language | `Select` pt-BR / English, prefilled with the current locale | required | → `locale` (profile, S4) |

No password-confirmation field: the show/hide toggle is the typo check (NIST 800-63B practice), and password reset exists (AC-5). No display-name field: no requirement asks for one. **Terms of use / privacy acceptance:** **[Open]**, vision §12: required before launch, not before dev. A checkbox slot is reserved above the button.

**Actions:** `Criar conta` / `Create account`.

| Result | UI |
|---|---|
| Success | Signed in automatically → app home (`Primeiros passos` + TOTP banner S3-07), in the chosen language |
| `auth.password_too_short` / `auth.password_breached` | Inline field error: "Use pelo menos 12 caracteres." / "Esta senha aparece em vazamentos conhecidos. Escolha outra." |
| Invite became invalid meanwhile | Switch to the invalid state above |

**AC mapping:** AC-1 (invite-only: refused without a valid, unused, unexpired invite, with a clear message).

## S3-04 Forgot password — `/[locale]/forgot-password` (AC-5, **Q20**)

**Q20 (a), recommended: transactional e-mail.**
- Field: E-mail (`autocomplete=username`). Action `Enviar link` / `Send link`.
- Result is **always the same** (AC-5): "Se houver uma conta com este e-mail, você receberá um link em alguns minutos. O link vale por {duration}." / "If an account exists for this e-mail, you'll receive a link in a few minutes…". `{duration}` comes from the Core config (**[Open: Architect]**).
- Rate limited: same generic message (no different response).

**Q20 (b), fallback: no e-mail.** The page shows only text: "Para redefinir sua senha, peça um link ao administrador." / "To reset your password, ask the administrator for a link." No form. The admin generates the link from S3-08 (an extra row action "Gerar link de redefinição"; spec added only if (b) is chosen).

## S3-05 Reset password — `/[locale]/reset-password/[token]` (AC-5)

- Same token handling as S3-03 (validating / invalid-generic / valid).
- Field: Nova senha / New password (rules as S3-03). Action `Redefinir senha` / `Reset password`.
- Success → login with `?reason=password_reset`. TOTP, if enabled, is still required at the next sign-in.
- **[Open: Architect]** whether a reset ends all existing sessions (recommended by design: yes, and the success message would say "Você foi desconectado de todos os dispositivos").

## S3-06 Security settings — `/[locale]/settings/security` (AC-1b)

**Layout:** `PageHeader` "Segurança" / "Security"; stacked `Card`s.

### Card A — Two-step verification
- Status line: `Badge` `Ativada` (success, `ShieldCheck`) / `Desativada` (warning, `ShieldAlert`) + text "Recomendado: protege suas contas de trading mesmo que sua senha vaze." / "Recommended: protects your trading accounts even if your password leaks."
- When enabled and **Q22 (a)**: "Códigos de recuperação restantes: {n}".
- Action: `Ativar` / `Enable` or `Desativar` / `Disable`.

**Enable flow:** `ResponsiveDialog`, 4 steps with a step indicator ("Etapa 2 de 4"). Closing at any step before step 3 succeeds = nothing changes.

| Step | Content | Validation / API |
|---|---|---|
| 1 Confirm it's you | Senha / Password (`current-password`) | `auth.totp_setup_start` → `password` (re-auth, AC-1b). Wrong password: "Senha incorreta." |
| 2 Scan | QR code (`img` with `alt` "QR code para o app autenticador") + "Não consegue escanear? Digite a chave:" + setup key in a `SecretField` (F§8; grouped in blocks of 4 when revealed) | Response carries the otpauth URI / key; never cached |
| 3 Confirm code | `InputOTP` 6 digits | `auth.totp_setup_confirm` → `code`. Wrong: "Código inválido. Confira o relógio do celular e tente de novo." |
| 4 Recovery codes (**Q22 (a)**) | 10 codes in a 2-column mono list; actions `Copiar todos` / `Baixar .txt` (client-side file from the response, filename `all-win-recovery-codes.txt`); warning `Alert`: "Guarde estes códigos em um lugar seguro. Cada um funciona uma vez. Eles não serão mostrados de novo." Checkbox (required) "Guardei meus códigos" → enables `Concluir` / `Done` | Codes shown once; cleared from state on close |

If Q22 = (b), step 4 is removed. Success toast: "Verificação em duas etapas ativada." The TOTP banner (S3-07) disappears.

**Disable flow:** `AlertDialog`: title "Desativar a verificação em duas etapas?"; body: "Sua conta ficará protegida só pela senha." Fields: Senha + Código atual (`InputOTP`; link to use a recovery code instead under Q22 (a)). Confirm `Desativar` (destructive). Errors as above. AC-1b: requires re-authentication.

### Card B — Session
- `Sair` / `Sign out` button (same as the user menu).
- Session duration text ("Sessões inativas expiram após {idle}.") only once **Q21** sets values. Until then, hidden.
- **[Proposal, not required by an AC]**: change password and "sign out of all devices". Listed for PO; not part of this slice unless approved.

## S3-07 TOTP suggestion banner (AC-1b "strongly suggested", D13)

- Where: app shell, below the top bar, on every app page, while TOTP is disabled.
- Component: `Alert` (info, `ShieldAlert` icon): "Proteja sua conta com a verificação em duas etapas." / "Protect your account with two-step verification." Actions: `Ativar agora` / `Enable now` (→ S3-06, opens the enable dialog) and `Agora não` / `Not now` (icon-button `X` with name "Dispensar aviso").
- Dismissal persists **per device** (local storage, wrapped in try/catch; failure = shows again). It reappears on a new device or browser. The Security card keeps the permanent recommendation. No API change needed.
- Not shown on public pages or inside dialogs. On mobile it is a single line + one action, with the dismiss button ≥ 44 px.

## S3-08 Invitations (admin) — `/[locale]/admin/invites` (AC-1, **Q19**)

Drawn for **Q19 (a)**: a platform-admin role held by the user; single-use link bound to one e-mail; valid 7 days; manual delivery (plus e-mail if **Q20 (a)**). If Q19 = (b) "any user can invite", the same screen moves to Settings and is available to all users.

**Visibility:** sidebar group "Administração" and this route exist only for admins. A non-admin gets the generic not-found screen (does not reveal the route).

**List** (`Table` ≥ `md`, cards below): E-mail · Status (`Pendente` info / `Usado` success / `Expirado` neutral / `Revogado` neutral) · Criado em · Expira em (date-time with zone, F§6.4) · Actions (`Revogar` for pending).
**Empty:** "Nenhum convite ainda. Convide alguém para usar o All Win." + `Convidar`.

**Create** (`ResponsiveDialog` "Novo convite"):
| Field | Validation | API |
|---|---|---|
| E-mail | required, e-mail format; if an account or pending invite exists for it: "Já existe uma conta ou um convite pendente para este e-mail." (**admin-only screen, so this disclosure is acceptable**; PO to confirm) | `invites.create` → `email` |
| Idioma do convite / Invitation language | pt-BR / English, default = admin's locale | → `locale` (link and e-mail language, S4) |

Result step: the invite link in a `SecretField` (it is a credential: view-once like source URLs; **[Assumption]** the Core stores only its hash, Architect to confirm) with `Copiar`, plus "Expira em {date time zone}". With Q20 (a): also "Enviamos o convite para {email}." Closing without copying → same guard as S3-09 create.
**Revoke:** `AlertDialog` "Revogar o convite para {email}? O link deixará de funcionar." → `invites.revoke`.

## S3-09 Signal sources — `/[locale]/connections/sources` (AC-3, S1 AC-2/AC-6, Q17)

**Purpose:** create, rotate and revoke the secret webhook URLs that TradingView alerts post to. The URL contains the secret (A3); it is **view-once** (D1).
**Layout:** `PageHeader` "Fontes de sinal" / "Signal sources" + description "Cada fonte tem uma URL secreta. Cole-a no campo Webhook URL do alerta no TradingView." + primary `Nova fonte` / `New source`. Toggle (`Switch`) "Mostrar revogadas" / "Show revoked" (default off).

Drawn for **Q17 (a)**: several named sources. If Q17 = (b) one per user: hide `Nova fonte` while an active source exists, and the name field becomes optional (default "TradingView").

**List** — `Table` ≥ `md`, `Card` list below (F§9):

| Column | Content | Source |
|---|---|---|
| Nome / Name | text | `sources.list[].name` |
| Tipo / Type | `TradingView` | `.type` |
| URL | **Masked only** (never revealable after creation): `https://{host}/ingress/v1/tradingview/awk_{key_id}_••••••••` in mono, truncated in the middle on narrow widths; full masked value in a tooltip | `.webhook_url_masked` or `.key_id` + base URL (Core provides; the web never builds secrets) |
| Status | `Ativa` (success) / `Revogada` (neutral, `Ban`) + revoked date | `.status`, `.revoked_at` |
| Criada em / Created | date-time | `.created_at` |
| Último sinal / Last signal | relative time + absolute tooltip, or "Nunca" / "Never" | `.last_received_at`, **[Dependency: S1/S10 must expose it; hide the column if absent]** |
| Ações | `DropdownMenu`: `Renomear`, `Gerar nova URL`, `Revogar` (active only) | |

**Empty state:** title "Nenhuma fonte de sinal" / "No signal sources"; text "Crie uma fonte para receber os alertas do TradingView com segurança."; action `Nova fonte`. Below, a link to the alert setup guide (**[Dependency: S17 guide]**; hidden until it exists).
**Loading:** 3 skeleton rows. **Error:** `ErrorState` with retry.

### Create source (`ResponsiveDialog`, 2 steps)
1. **Name:** `Input` "Nome da fonte" / "Source name", required, 1–60 chars, trimmed; example placeholder "Ex.: Strikes MNQ". Duplicate name for the same user (active ones): inline "Você já tem uma fonte com este nome." **[Proposal: names unique among active sources; PO to confirm]**. Action `Criar` → `sources.create { name }`.
2. **Copy the URL (view-once):**
   - Warning `Alert`: "Copie a URL agora. Por segurança, ela não será mostrada de novo. Se perdê-la, gere uma nova." / "Copy the URL now. For security, it won't be shown again. If you lose it, rotate to get a new one."
   - `SecretField` (F§8) with the full `webhook_url` from the response: masked by default, `Revelar`, `Copiar`.
   - Steps text: "1. No TradingView, abra o alerta → Notificações → Webhook URL. 2. Cole a URL. 3. Salve o alerta." **[Copy to validate with PO/S17 against TradingView's current UI]**
   - `Concluir` / `Done`. If the user tries to close (button, Esc, overlay, back gesture) **without having used Copiar**, an `AlertDialog` asks: "Você ainda não copiou a URL. Se fechar agora, precisará gerar uma nova. Fechar mesmo assim?" (`Voltar e copiar` focused / `Fechar`).
   - On close: secret wiped from state; list refetched; toast "Fonte criada." (no URL in the toast).
   - Errors at step 1: inline/`Alert`; nothing created. Network failure **after** submit with unknown outcome: "Não sabemos se a fonte foi criada. Atualize a lista antes de tentar de novo." + `Atualizar lista` (prevents duplicate sources; the Core's create is not assumed idempotent).

### Rotate (`Gerar nova URL`) — D2 immediate
- `AlertDialog`: title "Gerar nova URL para “{name}”?"; body: "A URL atual para de funcionar **imediatamente**. Alertas que chegarem com ela serão rejeitados e registrados na auditoria. Depois, cole a nova URL no alerta do TradingView." Confirm `Gerar nova URL` (destructive).
- Then the same step-2 view-once dialog as create. Toast "URL da fonte “{name}” trocada."
- `sources.rotate { id }` → returns the new `webhook_url` once.

### Revoke — D4 stays visible
- `AlertDialog`: "Revogar a fonte “{name}”?"; body: "Alertas com esta URL serão rejeitados. A fonte continua no histórico como revogada. Isso não pode ser desfeito; para voltar a receber sinais, crie uma nova fonte." Confirm `Revogar fonte` (destructive) → `sources.revoke { id }`.
- Row becomes `Revogada`, hidden unless "Mostrar revogadas" is on. Actions on a revoked row: none (name kept for history).
- **[Dependency S7]:** if routing rules reference the source, the dialog lists them ("Rotas que usam esta fonte deixarão de receber sinais: …"). Until S7 exists, no list.

### Rename
- Inline dialog with the name field (same validation) → `sources.update { id, name }`. Does not change the URL.

**Accessibility:** the masked URL cell has `aria-label` "URL secreta, oculta, termina em identificador {key_id}" (it reads the key id, not the bullets). Row actions button: "Ações para {name}".
**AC mapping:** AC-3 (generate, view once, revoke; old secret rejected: surfaced by copy and audit), AC-2 (own sources only), X-4 (masking, no secret in toasts/URL/logs), X-5 (actions audited).

## S3-10 NinjaTrader connections — `/[locale]/connections/ninjatrader` (AC-4, Q18)

**Purpose:** pair each NinjaTrader 8 instance to the user (each has its own credential: device token, never shown in the web) and unpair it. Live status via SSE (ADR-0001 §9).
**Layout:** `PageHeader` "Conexões NinjaTrader" / "NinjaTrader connections" + `Nova conexão` / `New connection`.

**List** (`Table` ≥ `md`; cards below, one per instance):

| Column | Content | Source |
|---|---|---|
| Nome / Name | user-given (D6), e.g. "VPS Chicago" | `.name` |
| Status | F§7.2: `Aguardando pareamento` / `Conectada` / `Desconectada` / `Despareada` + "há 3 min" since last change | `.status`, `.status_changed_at` (live) |
| Contas / Accounts | count of NinjaTrader accounts the plugin reports (from `hello`); expandable row / `Collapsible` on card listing account names (e.g. `Sim101`, `APEX-12345-01`) | `.accounts[]` **[Dependency ADR-0004]** |
| Plugin | version; `Badge` warning "Atualização necessária" when the Core rejected the version (`unsupported_version`) | `.adapter_version`, `.compatibility` |
| Última conexão / Last seen | date-time | `.last_seen_at` |
| Detalhes | instance id (mono, from plugin; D6) shown in the expanded row | `.instance_id` |
| Ações | `Renomear`, `Gerar novo código` (awaiting pairing only), `Desparear` | |

**Empty state:** title "Nenhum NinjaTrader conectado"; three numbered steps: "1. Instale o plugin All Win no NinjaTrader 8. 2. Crie uma conexão aqui. 3. Digite o código de pareamento no plugin." Link "Como instalar o plugin" **[Dependency: install guide by Plugin Engineer; hidden until it exists]**; action `Nova conexão`.
**Disconnected instance:** row shows danger badge + text "O plugin não está conectado. Ordens para as contas desta instância não serão executadas." (consequence wording; the out-of-sync handling is S11).

### Create connection + pairing (`ResponsiveDialog`, stays open while waiting)
1. **Name:** `Input` "Nome da conexão" / "Connection name", required, 1–60 chars, placeholder "Ex.: PC de casa, VPS Chicago". → `connections.create { name }`, which returns the connection and a pairing code.
2. **Pairing code:**
   - Code displayed **visibly** (it must be typed into another app): `text-3xl` mono, grouped `K7QM-4D2P`, `Copiar` button. It is short-lived and single-use, so it is not masked; the dialog says "Não compartilhe este código." / "Don't share this code." Format and TTL are the Architect's (D5); this layout fits up to 12 characters. **[Proposal to Architect: 8 chars from an unambiguous alphabet (no 0/O/1/I), 10 min, single use]**.
   - Countdown: "Expira em 9:41" (`<span aria-live=off>`; a polite announcement only at 1 min left and at expiry). The countdown uses the server's `expires_at`.
   - Instructions: "No NinjaTrader: abra a janela All Win (menu New → All Win), clique em Parear e digite o código." **[Copy to validate with the Plugin Engineer after the NT8 spike]**
   - Waiting indicator: "Aguardando o plugin…" with a spinner (static text under reduced motion).
   - **Paired** (SSE event): success state with `CircleCheck`: "“{name}” conectada. {n} contas encontradas: Sim101, …" + `Concluir`. Focus moves to the success heading.
   - **Expired:** "O código expirou." + `Gerar novo código` (→ `connections.new_pairing_code { id }`, invalidates any previous code).
   - **Closing while waiting:** allowed without a guard (no secret is lost: a new code can be generated). The connection stays in the list as `Aguardando pareamento`.
   - **Invalid attempts** happen in the plugin, not here (S3-11).

### Unpair (`Desparear`): Q18
Drawn for **Q18 (a)**: allowed after a confirmation that lists the affected accounts. Nothing is closed automatically.
- `AlertDialog` title: "Desparear “{name}”?"
- Body (always): "O plugin desta instância será desconectado e não receberá mais ordens. Para reconectar, crie uma nova conexão."
- If any of the instance's accounts is a routing destination **[Dependency S5/S7 data]**: list "Estas contas deixarão de receber ordens: APEX-12345-01, Sim101."
- If any has an open position **[Dependency S11 position data]**: danger `Alert` inside the dialog: "Há posição aberta em: APEX-12345-01 (2 contratos). **A posição continua aberta no NinjaTrader**, sem monitoramento nem proteção de risco do All Win, e a conta será marcada como fora de sincronia. Nada será fechado automaticamente." Confirmation requires a checkbox "Entendo que a posição continuará aberta" before `Desparear` enables.
- Confirm `Desparear` (destructive) → `connections.unpair { id }`. Row → `Despareada`. Other connections are unaffected (AC-4); the dialog says nothing about them.
- **If Q18 = (b)**: with an open position, the confirm button is replaced by an explanation "Feche a posição no NinjaTrader antes de desparear." and only `Fechar`.
- Unpaired connections stay listed (history) under a "Mostrar despareadas" toggle, like revoked sources.

**AC mapping:** AC-4 (pair per instance, own credential, unpair; unpaired receives nothing; revoking one does not affect others), AC-2, X-4 (device token never displayed), X-5.

## S3-11 NinjaTrader plugin: pairing panel (Plugin Engineer)

The plugin UI follows NinjaTrader 8 conventions (NTWindow/WPF, NinjaTrader skin colors, not shadcn). Detailed window spec is owed after the NT8 spike (ADR-0001 §7). This section fixes **states and copy** so web and plugin agree. Language follows the web profile (Q11, S4 AC-3); before the first pairing, the plugin uses the Windows UI language if pt or en, else pt-BR.

| State | Plugin shows | Action |
|---|---|---|
| Not paired | "Não pareado. Gere um código em All Win → Conexões → NinjaTrader." | Code field (8 chars, auto-uppercase, hyphen optional) + `Parear` |
| Pairing | "Pareando…" | Disabled |
| Invalid / expired / used code | "Código inválido ou expirado. Gere um novo código no All Win." (one message for all causes) | Field cleared |
| Paired + connected | Name of the connection (from Core), "Conectado", account count | `Desparear` (local: forgets the token after confirmation) |
| Disconnected / reconnecting | "Sem conexão com o All Win. Tentando reconectar…" + last connected time | — |
| Revoked (unpaired in web) | "Esta instância foi despareada no All Win. Gere um novo código para conectar." | Back to Not paired |
| Update required | "Versão do plugin não suportada. Atualize o plugin." | Link to download page (**[Dependency]**) |

The device token is never displayed or logged by the plugin (DPAPI storage, ADR-0001 §6).

---

## 2. AC traceability

| AC | Screens |
|---|---|
| AC-1 invite-only sign-up, sign in/out | S3-01, S3-03, S3-08, user menu "Sair" |
| AC-1b optional TOTP, strongly suggested, re-auth to disable | S3-02, S3-06, S3-07 |
| AC-2 isolation | §1 global rule; every list/detail; QA two-user tests per screen |
| AC-3 generate / view once / revoke source secret | S3-09 |
| AC-4 pair / unpair per instance | S3-10, S3-11 |
| AC-5 password reset, session expiry, non-revealing failures | S3-01, S3-04, S3-05, F§10 session-expired pattern |
| X-4 secrets | F§8, S3-03/05 token handling, S3-06, S3-08, S3-09, S3-10 |
| X-6 bilingual | all strings keyed; S4 spec |

## 3. Proposed operations and error codes (for the Architect / OpenAPI)

| Operation (proposal) | Used by | Notes |
|---|---|---|
| `auth.login`, `auth.verify_totp`, `auth.verify_recovery_code`, `auth.logout` | S3-01/02 | Login returns `session` or `pending_2fa` |
| `auth.get_invitation`, `auth.accept_invitation` | S3-03 | Get returns only `email`, `locale`, validity |
| `auth.request_password_reset`, `auth.reset_password` | S3-04/05 | Request always 202 |
| `auth.totp_setup_start`, `auth.totp_setup_confirm`, `auth.totp_disable`, `me.security` | S3-06 | `me.security` returns `totp_enabled`, `recovery_codes_remaining` |
| `invites.list/create/revoke` | S3-08 | Admin-only |
| `sources.list/create/rotate/revoke/update` | S3-09 | `create`/`rotate` return `webhook_url` **once**; `list` returns `key_id` + masked URL only |
| `connections.list/create/new_pairing_code/unpair/update` + SSE `connection.status_changed`, `connection.paired` | S3-10 | `create` returns `pairing_code`, `expires_at` |

Error codes the web will translate (proposal): `auth.invalid_credentials`, `auth.rate_limited`, `auth.invalid_code`, `auth.pending_expired`, `auth.password_too_short`, `auth.password_breached`, `auth.invitation_invalid`, `auth.reset_token_invalid`, `auth.reauth_failed`, `invites.email_taken`, `sources.name_taken`, `connections.pairing_code_expired`, `common.not_found`, `common.rate_limited`, `common.unexpected`.

## 4. Open questions (owner)

| ID | Question | Owner | Spec default |
|---|---|---|---|
| Q17 | Sources per user | User | Several named (a) |
| Q18 | Unpair with destinations / open position | User | Allow with listed consequences + checkbox (a) |
| Q19 | Who issues invites | User | Admin role, 7 days, bound to e-mail (a) |
| Q20 | Transactional e-mail | User | E-mail for invite + reset (a) |
| Q21 | Session idle/absolute timeout | Architect → User | Text hidden until set |
| Q22 | TOTP recovery | User | 10 codes (a). New: may the user **regenerate** codes? |
| D5 | Pairing code format/TTL | Architect | Layout fits ≤ 12 chars |
| D10 | Password policy | Architect | ≥ 12, breached list |
| — | Reset ends all sessions? Invite links stored hashed? Reset link validity? | Architect | Assumed yes / yes / config |
| — | Source names unique among active sources? Invite e-mail disclosure on admin screen ok? | PO | Proposed yes / yes |
| — | Change password / sign out everywhere in V1? | PO | Not in this slice |
| — | Terms/privacy acceptance at sign-up | User (vision §12) | Slot reserved |
