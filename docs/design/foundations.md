# Design Foundations — All Win Web

> **Owner:** Product Designer (Aquarela, design-system owner) · **Status:** proposed, pending PO review of rev. 3 · **Revision:** 4 · **Date:** 2026-10-23
> **Inputs:** [product-vision.md](../product/product-vision.md) rev. 3, [slice-plan.md](../product/release-1/slice-plan.md) (X-1..X-8, S3, S4, S5, S16), [ADR-0001](../architecture/adr/0001-technology-stack.md) §8–§9, [ADR-0004](../architecture/adr/0004-adapter-protocol-v1.md) §8–§9, [auth-and-session-policy.md](../architecture/security/auth-and-session-policy.md), PO answers of 2026-10-23 (Bússola), user decisions of 2026-10-23 in [decisions.md](../delivery/decisions.md) (Q17–Q23, firms that forbid automation).
> **Rev. 4 change (2026-10-23, user UX rule in decisions.md):** long forms become step by step; the limit, the edit rule and the per-step behavior are in §10.1; glossary rows for step, review and skip (§5).
> **Rev. 3 changes (2026-10-23, slice-plan rev. 7, passwordless S3):** `StepUpDialog` now asks a **new code** instead of a password (§8.2); `SecretField` no longer serves TOTP (§8.1); glossary drops TOTP/recovery codes and adds code, channel and replication terms (§5); AC references renumbered (S3 AC-3 → AC-14); keyboard/autocomplete and accessible-authentication notes updated (§9, §11); new shared components live in [components.md](components.md).
> **Rev. 2 changes:** Q23 decided (§6.4); step-up pattern (§8.2); `outdated` connection state and automation states (§7.2); preset provenance: verified / not confirmed / edited, and the official rule quote (§7.4).
> **Scope:** shared, cross-screen decisions for the web application. Screen behavior lives in [specs/](specs/). Business rules are only cited, never introduced here; anything not decided is marked **[Open]** (user decision, tracked by the PO) or **[Proposal]** (design recommendation for review).
> **Consumers:** Frontend (implements in `web/src/shared/ui` and the theme), QA (checks), Plugin Engineer (status semantics and terminology, §7 and §10).

---

## 1. Principles

1. **State is never ambiguous.** Trading state (risk, connection, account) is shown with **text + icon + color**, never color alone (WCAG 1.4.1). The label must make sense on its own when read by a screen reader.
2. **The number on screen is the number in the database.** Values are displayed exactly as stored (X-3); formatting changes separators, never digits. The browser does no money arithmetic (ADR-0001 §8).
3. **Secrets are shown once, deliberately.** Masked by default, revealed by an explicit action, never rendered into logs, URLs, toasts or analytics (X-4, §8).
4. **Every automatic action has a visible reason** (X-5). UI components that display a state always have a slot for its reason.
5. **Calm by default, loud when it matters.** Neutral UI; color saturation is reserved for warnings, limits and failures, so they stand out.
6. **Bilingual from the first line** (X-6). No hard-coded text; layouts tolerate ~35 % text expansion between pt-BR and en.

## 2. Component foundation (shadcn/ui)

- **Base:** shadcn/ui on Tailwind CSS with CSS-variable theming (ADR-0001). Base color **neutral**. Components are copied into `web/src/shared/ui/` and only extended there (features never fork a component).
- **Icons:** `lucide-react` (shadcn default). Icons beside text are `aria-hidden`; icon-only buttons have an accessible name (translated).
- **Components used by S3/S4** (add others per slice): `Button`, `Input`, `InputOTP` (base of `CodeInput`), `Label`, `Form` (react-hook-form + zod, shadcn pattern), `Card`, `Table`, `Badge`, `Alert`, `AlertDialog`, `Dialog`, `Drawer` (mobile variant of Dialog, see §9), `DropdownMenu`, `Select`, `Switch`, `Checkbox`, `Tooltip`, `Skeleton`, `Separator`, `Sidebar`, `Sheet`, `Sonner` (toasts), `Progress` (only for non-semantic progress; risk usage uses the Risk meter in §7.3).
- **Shared composites** to build once in `src/shared/ui` (specified here and in the specs):
  - `SecretField`: §8.1. `StepUpDialog`: §8.2.
  - `CodeInput`, `CodeChallenge`, `Stepper`, `DataTable`, `RiskBanner`, `NotificationBell`, `NotificationItem`, `CriticalBanner` and the `EmptyState` variants: [components.md](components.md).
  - `ProvenanceMark`, `FirmRuleQuote`: §7.4.
  - `StatusBadge`: §7.
  - `RiskMeter`: §7.3 (first used in S13; defined now so tokens are final).
  - `ResponsiveDialog`: Dialog ≥ `md`, Drawer below `md` (§9).
  - `MoneyText`, `PriceText`, `QuantityText`, `DateTimeText`: display-only formatters (§6).
  - `EmptyState`, `ErrorState`, `PageHeader`: §10.
- **Dependencies:** nothing here requires a package outside ADR-0001 except a theme switcher. shadcn documents `next-themes` for dark mode; adding it is a Frontend/Architect call. A ~20-line cookie-based switcher is an acceptable alternative.

## 3. Design tokens

Tokens follow the shadcn CSS-variable names so generated components work unchanged. Values are OKLCH (Tailwind v4 / shadcn current). **All pairs below were checked for contrast** (script results in the table; WCAG 2.2 AA: text ≥ 4.5:1, UI boundaries/focus/graphics ≥ 3:1).

### 3.1 Base tokens

Values that differ from shadcn's neutral defaults are **bold**, changed to pass AA.

| Token | Light | Dark | Notes |
|---|---|---|---|
| `--background` | `oklch(1 0 0)` | `oklch(0.145 0 0)` | |
| `--foreground` | `oklch(0.145 0 0)` | `oklch(0.985 0 0)` | 19.8 : 1 / 19.0 : 1 |
| `--card`, `--popover` | `oklch(1 0 0)` | `oklch(0.205 0 0)` | |
| `--card-foreground`, `--popover-foreground` | `oklch(0.145 0 0)` | `oklch(0.985 0 0)` | |
| `--primary` | `oklch(0.205 0 0)` | `oklch(0.922 0 0)` | No brand color yet; brand identity is **[Open]** (no product input). Primary stays neutral until then. |
| `--primary-foreground` | `oklch(0.985 0 0)` | `oklch(0.205 0 0)` | |
| `--secondary`, `--accent` | `oklch(0.97 0 0)` | `oklch(0.269 0 0)` | |
| `--muted` | `oklch(0.97 0 0)` | `oklch(0.269 0 0)` | |
| `--muted-foreground` | **`oklch(0.53 0 0)`** | `oklch(0.708 0 0)` | Default 0.556 failed on `--muted` (4.34). Now 4.84 on muted, 5.28 on background; dark 5.83 / 7.63 |
| `--border` | `oklch(0.922 0 0)` | `oklch(1 0 0 / 10%)` | Decorative separators only |
| `--input` | **`oklch(0.64 0 0)`** | **`oklch(0.52 0 0)`** | Field boundary is required to identify the control (1.4.11): 3.36 : 1 light, 3.25 : 1 on dark card |
| `--ring` | **`oklch(0.556 0 0)`** | `oklch(0.708 0 0)` | Focus ring 4.73 : 1 light, 6.9 : 1 dark. Ring width 2 px + 2 px offset |
| `--destructive` | `oklch(0.577 0.245 27.325)` | `oklch(0.704 0.191 22.216)` | Destructive *buttons*. Frontend verifies label contrast on the final button style (≥ 4.5 : 1) |
| `--radius` | `0.5rem` | | Cards `lg`, inputs/buttons `md`, badges `full` |

### 3.2 Status tokens (new, added to the theme)

Each status has a **strong** color (text, icon, meter fill) and a **soft** background (badges, alerts). Strong color is AA on background, card and its own soft background in both themes.

| Token | Light strong / soft | Dark strong / soft | Contrast (strong on bg · card · soft) |
|---|---|---|---|
| `--status-success` / `-soft` | `oklch(0.527 0.154 150.069)` / `oklch(0.982 0.018 155.826)` | `oklch(0.792 0.209 151.711)` / `oklch(0.266 0.065 152.934)` | L 4.94 · 4.94 · 4.72 — D 11.2 · 10.1 · 8.4 |
| `--status-warning` / `-soft` | `oklch(0.555 0.163 48.998)` / `oklch(0.987 0.022 95.277)` | `oklch(0.828 0.189 84.429)` / `oklch(0.279 0.077 45.635)` | L 5.05 · 5.05 · 4.87 — D 11.5 · 10.4 · 8.7 |
| `--status-danger` / `-soft` | `oklch(0.505 0.213 27.518)` / `oklch(0.971 0.013 17.38)` | `oklch(0.704 0.191 22.216)` / `oklch(0.258 0.092 26.042)` | L 6.42 · 6.42 · 5.88 — D 6.84 · 6.19 · 5.58 |
| `--status-info` / `-soft` | `oklch(0.488 0.243 264.376)` / `oklch(0.97 0.014 254.604)` | `oklch(0.707 0.165 254.624)` / `oklch(0.282 0.091 267.935)` | L 6.82 · 6.82 · 6.26 — D 7.51 · 6.79 · 5.58 |
| `--status-neutral` / `-soft` | = `--muted-foreground` / `--muted` | = `--muted-foreground` / `--muted` | L 5.28 · 5.28 · 4.84 — D 7.63 · 6.91 · 5.83 |

Additional semantic aliases (no new values):

- `--pnl-positive` → `--status-success`; `--pnl-negative` → `--status-danger`. P&L is **never** color-only: it always carries an explicit sign (§6.2).
- Badges use soft background + strong text + a 1 px border in strong color at 30 % opacity (decorative; the text carries the meaning).

### 3.3 Typography

- **Families:** Geist Sans (UI) and Geist Mono (identifiers, secrets, pairing codes), self-hosted through `next/font` (no runtime third-party requests). Both cover Portuguese diacritics.
- **Numerals:** every numeric display (prices, money, quantities, percentages, times, counters) uses `font-variant-numeric: tabular-nums` (Tailwind `tabular-nums`) so columns align and live values do not jitter.
- **Scale** (Tailwind): `text-xs` 12/16 (meta only, never for essential info), `text-sm` 14/20 (default UI and table text), `text-base` 16/24 (body copy, **all form inputs**), `text-lg` 18/28 (card titles), `text-xl` 20/28 (page titles on mobile), `text-2xl` 24/32 (page titles ≥ `md`), `text-3xl` 30/36 (key figures such as balance or pairing code).
- **Weights:** 400 body, 500 labels/table headers, 600 titles and key figures. No weights below 400.
- **Inputs are 16 px minimum** so mobile Safari does not zoom on focus.
- Text resizes to 200 % without loss (1.4.4); no text in images.

### 3.4 Spacing, layout and elevation

- Spacing on the Tailwind 4 px scale. Standard gaps: 4 (inline icon-text), 8 (field internals), 16 (between fields, card padding on mobile), 24 (card padding ≥ `md`, between cards), 32 (page sections).
- **Breakpoints:** Tailwind defaults (`sm` 640, `md` 768, `lg` 1024, `xl` 1280). **Minimum supported width: 320 CSS px** (WCAG 1.4.10 reflow).
- **App shell:** shadcn `Sidebar` (collapsible to icons ≥ `lg`, off-canvas `Sheet` below `lg`) + top bar (page title, live connection indicator, user menu). Content max width 1280 px for forms/lists; wide data tables may use full width.
- **Sidebar order (confirmed 2026-10-23, waves 1–3; items appear when their slice ships):** *Operação:* Início · Contas · Roteamento · Atividade · Trades · Desempenho. *Configuração:* Perfis de stop e alvo · Presets. Then Conexões (Fontes de sinal, NinjaTrader) · Configurações (Segurança, Preferências) · Administração (admin only). Notifications are the top-bar bell only (C§7), never a sidebar item. Group labels are `SidebarGroupLabel`s; collapsed-to-icons mode keeps one icon per item with a tooltip name.
- **Elevation:** flat. Cards use a border; shadows only for overlays (popover, dialog, toast).
- **Motion:** 150–200 ms ease-out for overlays only. No motion conveys state. Under `prefers-reduced-motion: reduce`, transitions become opacity-only or instant, and spinners become a static "Carregando…" label.

## 4. Color modes (light / dark)

- **Modes:** Light, Dark, System. **Default: System** (`prefers-color-scheme`). Design decision, no product rule involved.
- **Switch location:** user menu → "Tema" (Claro / Escuro / Sistema · Light / Dark / System), and on public pages (login) next to the language switcher.
- **Persistence:** per device (cookie read during server rendering, so the first paint has the right theme and does not flash). Not stored in the user profile; nothing requires it.
- Both themes are first-class: every new component is reviewed in both. Tokens in §3 are the only colors allowed in components (no raw Tailwind palette classes in features).
- Charts (S22) will get their own palette, defined in that slice.

## 5. Terminology (glossary, pt-BR ↔ en)

The same term is used everywhere (web, plugin, errors, audit). The Plugin Engineer uses this table for the NinjaTrader UI.

| Concept | pt-BR | en |
|---|---|---|
| Signal source (e.g. a TradingView webhook) | Fonte de sinal / Fonte | Signal source / Source |
| Secret webhook URL | URL secreta do webhook | Secret webhook URL |
| Rotate secret | Gerar nova URL (rotacionar) | Rotate URL |
| Revoke | Revogar | Revoke |
| NinjaTrader instance connection | Conexão NinjaTrader / Instância | NinjaTrader connection / Instance |
| Pair / Unpair | Parear / Desparear | Pair / Unpair |
| Pairing code | Código de pareamento | Pairing code |
| Trading account | Conta | Account |
| Limit | Limite | Limit |
| Warning (80 %) | Alerta | Warning |
| Limit reached | Limite atingido | Limit reached |
| Blocked until reset | Bloqueada até o reset | Blocked until reset |
| Trading-day reset | Reset do dia de trading | Trading-day reset |
| Out of sync | Fora de sincronia | Out of sync |
| Paused / Disabled | Pausada / Desativada | Paused / Disabled |
| Verification code (one-time, sent to e-mail/WhatsApp) | Código de verificação / Código | Verification code / Code |
| Notification / unread / critical | Notificação / Não lida / Crítica | Notification / Unread / Critical |
| Sign-in channel | Canal de entrada | Sign-in channel |
| Resend code | Reenviar código | Resend code |
| New device | Novo dispositivo | New device |
| Sign out of all devices | Sair de todos os dispositivos | Sign out of all devices |
| Forget this device | Esquecer dispositivo | Forget this device |
| Cancel pending change | Cancelar troca | Cancel change |
| Invitation | Convite | Invitation |
| Audit log | Registro de auditoria | Audit log |
| Automated orders / automated source | Ordens automatizadas / Fonte automatizada | Automated orders / Automated source |
| Accepts / doesn't accept automation | Aceita automação / Sem automação | Accepts automation / No automation |
| Choice pending | Escolha pendente | Choice pending |
| Official rule | Regra oficial | Official rule |
| Not confirmed (conflicting official pages) | Não confirmado | Not confirmed |
| Firm account platform (e.g. Rithmic, Tradovate) | Plataforma da conta na firma | Firm account platform |
| Execution platform (e.g. NinjaTrader) | Plataforma de execução | Execution platform |
| Step-up re-authentication | Confirme que é você | Confirm it's you |
| Accepts / doesn't accept replication (copy opt-in) | Aceita replicação / Sem replicação | Accepts replication / No replication |
| Step of a multi-step form | Etapa | Step |
| Final check before saving a multi-step form | Revisar / Revisão | Review |
| Skip an optional step | Pular etapa | Skip step |
| Go to the previous / next step | Voltar / Continuar | Back / Continue |

## 6. Numbers, money, prices, dates

### 6.1 Rules for all numeric display

1. The Core sends decimals as **strings** (ADR-0001 §8). The web **never** converts them to JS `number`. Formatting uses `Intl.NumberFormat.prototype.format(string)`, which treats a string as an exact decimal (ECMA-402 NumberFormat v3; Frontend verifies support in the target browsers, else formats via the single `decimal.js` helper allowed by ADR-0001).
2. **No silent rounding.** Display precision is the **larger** of the type's precision (below) and the precision of the stored value. If a stored money value had 4 decimals, all 4 are shown. Formatting never drops significant digits (X-3).
3. Locale controls only separators, currency symbol position and grouping. **Locale switch never changes a value.**
4. Grouping separators are on for money and prices (`21.234,50` / `21,234.50`). Identifiers (ids, codes, versions) are never number-formatted.
5. **Every derived figure** (percent used, distance to limit, converted price) comes from the Core. The UI only formats it.

### 6.2 Types

| Type | Precision | pt-BR | en | Notes |
|---|---|---|---|---|
| Money (USD, always: PO 2026-10-23, no conversion) | min 2 | `US$ 1.234,50` | `$1,234.50` | `style: currency, currency: USD`. Same currency in both locales; only the symbol style differs |
| P&L / signed money | min 2 | `+US$ 1.234,50` · `-US$ 320,00` · `US$ 0,00` | `+$1,234.50` · `-$320.00` · `$0.00` | `signDisplay: exceptZero`. The sign is mandatory, because color alone is not enough. Use the engine's sign characters as they are; do not substitute the typographic minus, which some screen readers skip |
| Price | instrument tick decimals (MNQ: 2) | `21.234,50` | `21,234.50` | No currency symbol (it is an index-point price) |
| Points / ticks distance | points: tick decimals; ticks: 0 | `12,50 pts` · `50 ticks` | `12.50 pts` · `50 ticks` | Units shown always (vision §8: points, ticks or dollars) |
| Quantity (contracts) | 0 | `3 contratos` | `3 contracts` | ICU plural (`1 contrato` / `1 contract`) |
| Multiplier | 0 | `2×` | `2×` | Whole numbers only (vision §8) |
| Percent of limit used | 0, **truncated** (`roundingMode: trunc`) | `79 %`* | `79%` | Truncation guarantees that `80 %` is never displayed while the Core state is still *normal*. State and color always come from the Core's state field, never from the displayed number |
| Counts | 0 | `1.204` | `1,204` | |

\* pt-BR `Intl` output is `79%` without a space in current engines; use the engine's output and do not hand-insert spaces.

### 6.3 Number input (parsing)

- `inputMode="decimal"` (or `numeric` for integers), `autocomplete="off"`, text input (never `type="number"`, which accepts floats and locale quirks).
- Accepted: optional sign where the field allows it, digits, and **one** decimal separator. In pt-BR both `,` and `.` are accepted as the decimal separator. In en only `.`. **Grouping separators are rejected** in input ("Digite sem separador de milhar" / "Enter without thousands separators").
- Fields validate scale: prices must be a multiple of the instrument tick ("Use múltiplos de 0,25" / "Use multiples of 0.25"); money ≤ 2 decimals unless the field says otherwise. This rejects ambiguous entries such as `1.234` in pt-BR.
- On blur, the field re-renders the parsed value in the locale format, so the user sees how it was understood. The value sent to the Core is a canonical decimal string (`"1234.50"`). Parsing goes through the single exact parser required by ADR-0001 §8.

### 6.4 Dates, times, time zones

**Q23 decided by the user (2026-10-23).**

- **Storage/wire:** UTC instants (X-8). **Display time zone:** the user profile setting, defaulting to the browser's zone.
- **24-hour clock in both languages** (`hourCycle: h23`).
- Formats: date `23/10/2026` · `Oct 23, 2026`; date-time `23/10/2026 14:31:05` · `Oct 23, 2026, 14:31:05`; time `14:31:05` (seconds shown on audit/execution data, hidden elsewhere).
- **The time zone is always visible next to reset and session times**, e.g. `Reset às 17:00 (GMT-3)` / `Resets at 17:00 (GMT-3)`, using `timeZoneName: short`. Where the firm's own reset zone differs from the display zone, show both in a tooltip: `17:00 CT (19:00 GMT-3)`.
- Relative times ("há 2 min" / "2 min ago") only for recency indicators (last seen, last received), always with the absolute date-time in a `<time datetime>` element and tooltip.

## 7. Status semantics

### 7.1 Risk state per limit (vision §9, PO answer D18)

Exactly three risk levels exist. **There is no 90 % band.** The state is computed by the Core and sent as a field; the UI maps it.

| Core state | Meaning | Token | Icon (lucide) | Label pt-BR / en |
|---|---|---|---|---|
| `normal` | < 80 % of the limit | `--status-neutral` (meter fill: `--foreground` at 70 %) | — | *(no badge; meter only)* |
| `warning` | ≥ 80 % of the limit | `--status-warning` | `TriangleAlert` | `Alerta (80 %)` / `Warning (80%)` |
| `reached` | Limit reached; account blocked until the reset if enforcement is on | `--status-danger` | `OctagonX` | `Limite atingido` / `Limit reached` |

**Daily profit target** (Q3, decided): progress toward the target uses `--status-success`.

- With "notify only" (default), reaching it shows `Meta atingida` / `Target reached` (`CircleCheck`, success).
- When the account chose "flatten and block", the account becomes blocked (§7.2), and **the reason must say it was the target**: `Bloqueada até 17:00 (GMT-3) · motivo: meta de lucro diária atingida`. The blocked badge uses `--status-success` for this reason and `--status-danger` for loss/drawdown reasons, so a good outcome never looks like a loss.

**Consistency rules** (Q4): track and warn only, never blocked. Use `warning` at most; never `reached`.

### 7.2 Account and connection states (separate from risk; they can coexist)

Owned by S5/S11/S3. Listed here so the visual language is fixed now. Final copy belongs to each slice spec.

| State | Token | Icon | pt-BR / en |
|---|---|---|---|
| Blocked until reset | by reason (§7.1) | `Lock` | `Bloqueada até {time}` / `Blocked until {time}` |
| Paused | neutral | `Pause` | `Pausada` / `Paused` |
| Disabled | neutral | `CircleOff` | `Desativada` / `Disabled` |
| Out of sync | danger | `RefreshCcwDot` | `Fora de sincronia` / `Out of sync` |
| Instance connected | success | `PlugZap` | `Conectada` / `Connected` |
| Instance disconnected | danger | `Unplug` | `Desconectada` / `Disconnected` |
| Instance awaiting pairing | info | `Hourglass` | `Aguardando pareamento` / `Awaiting pairing` |
| Instance outdated (plugin below the minimum version, ADR-0004 §9) | warning | `CircleArrowUp` | `Desatualizada` / `Outdated` |
| Instance unpaired / source revoked | neutral, strikethrough-free (use text) | `Ban` | `Despareada` / `Unpaired` · `Revogada` / `Revoked` |
| Source active | success | `CircleCheck` | `Ativa` / `Active` |
| Data stale / unknown (no recent update) | neutral | `CircleDashed` | `Sem dados recentes` / `No recent data` |
| Account accepts automated sources (S5 opt-in) | info | `Bot` | `Aceita automação` / `Accepts automation` |
| Account does not accept automated sources | neutral | `BotOff` | `Sem automação` / `No automation` |
| Automation choice pending (firm forbids/conditions automation, no choice yet, or suspended after a rule change) | warning | `CircleHelp` | `Escolha pendente` / `Choice pending` |

When several states apply to one account, show them in this priority order (most severe first): out of sync → disconnected → blocked → risk warning → automation choice pending → paused/disabled. On narrow cards show the top one plus `+N` with the full list in a popover.

### 7.3 `RiskMeter` (pattern, first used by S13)

- Structure: label (limit name) · formatted `used / limit` (`US$ 820,00 de US$ 1.000,00`) · percent · horizontal bar with a fixed **80 % tick mark** · status badge when not `normal`.
- Semantics: `role="meter"` with `aria-valuemin=0`, `aria-valuemax=100`, `aria-valuenow`, and `aria-valuetext` carrying the full sentence (`Perda diária: US$ 820,00 de US$ 1.000,00, 82 %, alerta`).
- Bar: track `--muted`, fill by state (§7.1); fill vs track ≥ 3 : 1 in both themes.
- Live updates (SSE) do **not** announce every tick. Only a **state change** (normal → warning → reached) is announced via a polite live region, and it also becomes a notification (S18).

### 7.4 Preset provenance and official rule text (S5, S16)

Preset values come from official firm pages (vision §9). The user must always be able to tell **where a value came from** and **whether it is confirmed**. Each preset-derived field carries one of three provenance marks:

| Provenance | When | Visual | Accessible text |
|---|---|---|---|
| **Verified** | Value verified on an official page | No badge, to keep forms calm. An `Info` icon-button after the label opens a popover: "Fonte oficial: {firm page title} · verificado em {date}" + link (opens in a new tab, `rel="noopener noreferrer"`) | "Fonte da regra" |
| **Not confirmed** (decision 2026-10-23: official pages conflict; shipped without blocking the rest) | The firm's own pages disagree | Field **not prefilled** (O7). `Badge` warning `Não confirmado` / `Not confirmed` (`CircleHelp`) next to the label; field border uses `--status-warning`; under the field a `ConflictPicker`: "As páginas oficiais da firma divergem. Escolha um valor ou digite outro:" followed by one `RadioGroup` option per published value, each with its source link and access date, plus "Outro valor". A required field cannot be saved until resolved: "Resolva este valor para salvar." | Badge text + "valores publicados divergentes" |
| **Edited by you** | The user changed a preset value on their account (S16 AC-2) | Neutral `Badge` `Editado` / `Edited` (`PencilLine`); popover shows the preset value and source: "Valor do preset: US$ 2.500,00 · fonte…" + `Restaurar valor do preset` | "Editado por você; valor do preset {value}" |

- After the user resolves a not-confirmed value, it shows **both** marks: `Editado` and, in the popover, "Escolhido por você entre valores divergentes em {date}". The conflict is never presented as resolved by the firm.
- A preset with not-confirmed items stays selectable (O8). In the preset picker it shows `{n} itens não confirmados` (warning text, not a badge, so the list stays scannable).
- Provenance marks are never color-only: badge text and accessible text always carry the meaning.

**`FirmRuleQuote`**: shows an official firm rule verbatim (used by the S5 automation opt-in and S16 stance display).
- `figure` containing a `blockquote` (with `cite` = source URL) in the firm's original language (`lang="en"` when the page is English, so screen readers switch pronunciation). Left border `--status-info`, `--muted` background, `text-sm`.
- `figcaption`: "{Firm} · {page title} · acessado em {date}" + link `Ver página oficial` / `View official page`, new tab.
- **The text is never translated or summarized inside the quote**, because a paraphrase would be our interpretation. A one-line plain-language summary may appear *above* the quote, labeled "Resumo do All Win (não é aconselhamento jurídico)" / "All Win summary (not legal advice)".
- Long quotes: first 6 lines visible, then `Mostrar texto completo` (`Collapsible`). The full text is in the DOM, so screen readers and find-in-page work.

## 8. Secrets and sensitive actions

### 8.1 `SecretField` pattern (X-4, S3)

Used for the secret webhook URL, the invitation link (S3) and one-time admin-issued links. Sign-in codes are typed by the user and use `CodeInput` instead ([components.md](components.md) C§1). Pairing codes use a different, visible pattern (see S3 spec §S3-10).

- **Masked by default.** The masked rendering keeps only the non-secret part: for the webhook URL, host + path + `awk_<key_id>_` followed by `••••••••` (the `key_id` is the only part allowed to appear anywhere, per root CLAUDE.md §2). **The secret is not in the DOM while masked** (protects against screenshots, extensions and DOM-reading tools).
- **Actions:** `Revelar` / `Reveal` (toggle, `aria-pressed`), `Copiar` / `Copy` (copies the full value without revealing it). After copying, the button shows `Copiado` + check icon for 2 s, and a polite live region announces it. A copy failure (clipboard permission) automatically reveals the value and selects it for manual copy, with an inline note.
- **Auto re-mask:** when the dialog closes, the window loses visibility (`visibilitychange`), or after 60 s revealed.
- **Visibility is limited to one moment.** Values exist only in the creation/rotation response (view-once, S3 AC-14). They are held in component state, never in TanStack Query caches persisted anywhere, URL, `localStorage`, toasts, analytics, error reports or `console`. The state is cleared on dialog close.
- Mono font, `break-all`, `spellCheck=false`, `autoComplete="off"`, `data-sentry-mask`-style exclusion if any client telemetry is added later.

### 8.2 `StepUpDialog` (S3 AC-10, policy §6, no password)

A sensitive action needs a **fresh one-time code for that action**, sent to a channel the user owns. The Core decides which actions (S3-04 lists them; Q30 whether a code covers several; Q35 whether more are added). The web reacts to the contract's `x-step-up` marker and does not hard-code the list.
- **Mechanics (policy §6.1):** 1) the dialog asks the Core for a step-up code (`channel`, action kind, target) and gets a `challenge_id`; 2) the user types the code; 3) the **original request is sent once with `challenge_id` + `code`**, and the Core verifies and performs the action in one transaction. There is no separate "verify then retry". A request sent without a code answers `auth.step_up_required`, which also opens the dialog. The challenge lives in memory only.
- **Content:** title "Confirme que é você" / "Confirm it's you"; text "Enviamos um código para {masked_channel} para {action}. Ele vale por {minutes} minutos." / "We sent a code to {masked_channel} to {action}. It's valid for {minutes} minutes." Body: `CodeChallenge` with `purpose=step_up` ([components.md](components.md) C§1.3). Actions `Confirmar` / `Cancelar`. `{masked_channel}` comes from the Core (for example `j***@d***.com`); the web never masks by itself.
- **Sending:** with one confirmed channel the code is sent when the dialog opens (the user has just confirmed the action). With several channels (S3b) the dialog first asks "Enviar código para" (`RadioGroup`, last used preselected) and a `Enviar código` button. For persistence actions on a new device the Core may require e-mail, or codes on both channels in turn (S3-04, provisional).
- **Cancel / Esc:** the action is **not** performed; the user returns to where they were with nothing changed.
- **Errors:** every verification failure is `auth.code_invalid` → "Código inválido ou expirado. Confira ou peça um novo." (one message). The dialog stays open, the action is not performed and the Core audits the attempt (AC-10, AC-16). The UI counts its own failures per challenge; after the 5th it disables the input and offers `Pedir novo código`. Rate limited → countdown (C§1.4).
- **Stacking:** if the action already has a confirmation `AlertDialog`, the step-up dialog replaces it, and the result step follows. Two modals are never shown at once.
- **View-once dialogs:** a step-up never opens over a dialog that holds a secret. Actions that show a secret ask for the code first, then show the secret.

## 9. Mobile browsers

The web must "work well in mobile browsers" (vision §3). Target: iOS Safari and Android Chrome, current and previous major version.

- **Reflow:** every screen works at 320 px wide without horizontal page scroll (1.4.10). Exception: dense data tables may scroll horizontally *inside their container*, with a sticky first column.
- **Tables → cards:** below `md`, list screens render each row as a `Card` with the primary field as title, status badge top-right, secondary fields as a definition list, and actions in a `DropdownMenu` (kebab). Same data, same order, same actions.
- **Overlays:** `ResponsiveDialog`: centered `Dialog` ≥ `md`, bottom `Drawer` below `md`. Destructive confirmations stay `AlertDialog` (centered) at all sizes.
- **Navigation:** the sidebar becomes an off-canvas `Sheet` opened from a top-bar menu button. Focus moves into the sheet and returns to the button on close.
- **Touch targets:** **≥ 44 × 44 px** for all interactive elements on touch layouts (exceeds the 24 px WCAG 2.2 minimum, 2.5.8). Spacing between adjacent targets ≥ 8 px.
- **Viewport:** use `100dvh` (not `100vh`), respect `env(safe-area-inset-*)` for fixed bars, and never disable zoom (`user-scalable` stays enabled).
- **Keyboard:** the correct `inputMode`/`type`/`autocomplete` per field (`email`, `one-time-code`, `tel`) so mobile keyboards and autofill work. There are no password fields in All Win. When the keyboard opens, the field in focus stays visible (sticky footers must not cover it).
- **No hover-only information.** Tooltips also open on tap/focus, and nothing essential lives only in a tooltip.
- **Background tabs:** when the tab returns to the foreground, live data is refetched and the SSE stream reconnects. A stale indicator (`Sem dados recentes`) is shown until fresh data arrives. Mobile browsers suspend background tabs, so the UI must never present stale risk/connection data as current.
- Orientation is not locked (1.3.4).

## 10. Feedback patterns (loading, empty, error, success, confirmation)

| Situation | Pattern |
|---|---|
| Initial page load | `Skeleton` matching the final layout (table rows / cards). No full-page spinners. If loading takes > 10 s, show "Ainda carregando…" with a retry link |
| Background refetch | Keep current data; subtle spinner in the page header; no layout shift |
| Empty list | `EmptyState`: icon, one-sentence title, one-sentence explanation of what the thing is for, one primary action. No illustrations |
| Field validation | Inline under the field (`aria-describedby`, `aria-invalid`), shown on blur and on submit; on submit, focus moves to the first invalid field, and a summary `Alert` lists errors when there are 3 or more |
| Request error (form) | `Alert` (destructive) above the form actions, with the translated error code; inputs keep their values |
| Request error (page) | `ErrorState` in place of the content: what failed, `Tentar novamente` / `Try again`, and the stable error code in small mono text (for support), never stack traces |
| Network offline | Top-bar banner `Sem conexão com o servidor. Os dados podem estar desatualizados.` until online; live states switch to "no recent data" |
| Success (non-destructive) | `Sonner` toast, 4 s, polite. Never contains secrets |
| Destructive / irreversible action | `AlertDialog`: title states the action and the object; body states consequences; cancel button focused by default; confirm button uses `destructive` variant and repeats the verb (`Revogar fonte`, not `OK`) |
| Submitting | Primary button shows spinner + "Salvando…", is disabled, and the form ignores further submits until the response (duplicate-submit prevention). Write endpoints are expected to be idempotent server-side as the real guarantee |
| Session expired (401 on any call) | Redirect to login with `?reason=session_expired&returnTo=<path>`; login shows an info `Alert`. Unsaved form data is lost; the message says so |
| Unknown error code from the Core | Generic translated message + the code (`Erro inesperado (código: x.y)`) |

### 10.1 Long forms: stepper or sections (user UX rule, decisions.md 2026-10-23)

The user wants a long "add something" form to be step by step, so filling many fields in one go is not tiring. A form is **long** when the create flow meets **any** of these (count the controls the user must fill or choose; read-only lines, fixed values and untouched defaults do not count; a `RadioGroup`, a checkbox group or a date + time pair counts as one):

| Test | Limit |
|---|---|
| Controls in the create form | **more than 6** |
| Sections (a heading with its own purpose) | **3 or more** |
| One section gates another (a choice changes which fields, texts or options appear next) | **any** |
| A section must be read before it can be answered (rule text, warning, risk) | **any** |

**Long = `Stepper` (C§2) in "form flow" mode.** Otherwise a single page or dialog with `Card` sections is enough (no stepper for a form of 6 controls or fewer, and never for a one-field dialog).

**Per step**
- At most **6 controls** per step; if a step passes that, split it. Flows have **2 to 6 steps** (C§2). If more would be needed, the form is too big: move optional fields to the edit page.
- A step groups fields that answer one question ("Que conta é esta?"), and its title is that question or a noun ("Dados da conta"). The first step is the one that decides which later steps exist (for example the account type).
- Steps that do not apply to the user's choice are **left out of the list** (the count says "Etapa 2 de 5" for that user), never shown disabled.
- The last step is always **`Revisar`**: a read-only summary of every step (definition list, grouped by step) with `Editar` next to each group and the single `Criar …` / `Salvar …` button. The only write happens here. Steps before it never save.
- **Optional steps** show `Pular etapa` next to `Continuar`; skipped means "nothing set", and the review says so.

**Navigation and data**
- Buttons: `Voltar` (secondary, hidden on step 1; `Cancelar` is there instead) and `Continuar` (primary). In the review: `Voltar` and the final button. Enter in a text field presses `Continuar`.
- `Continuar` validates **that step only** (inline errors, focus on the first invalid field, F§10). It is never disabled to hide an error; the user is told what is wrong.
- Going back **never loses data** (3.3.7): every step's values live in one form state until save or cancel. Changing an earlier answer that gates later steps (for example the firm) clears only the affected later values and says so in an `Alert` on that later step: "A regra mudou com a sua seleção. Escolha de novo." The review is recomputed.
- From the review, `Editar` on a group goes to that step; after `Continuar` there it returns **straight to the review** (the stepper shows the review as current), unless the change added or removed steps in between.
- Server errors that point to a field (422 with `pointer`) send the user back to the step that owns the field, focus it and show the error there; the review shows a destructive `Alert` "Corrija a etapa {n}: …" with a link. Errors without a field stay in the review above the button; values are kept.
- Leaving with unsaved data (`Cancelar`, browser back, close of the dialog) asks `AlertDialog` "Descartar alterações?" (F§10). Nothing is persisted as a draft in V1 **[Proposal]**; a reload loses the form, and the page says it with `beforeunload` only while the form is dirty.
- A step that needs the Core to answer before continuing (for example the preset list) shows a `Skeleton` and keeps `Continuar` available only when the data is there; a failure uses `ErrorState` inside the step with retry, and `Voltar` still works.
- Idempotency key and duplicate-submit rules (F§10) are on the final button only.
- Step-up (F§8.2) is asked **at the final button** only if the saved state requires it, and cancelling it returns to the review with every value kept.

**Layout:** page forms: stepper above the card (`full` ≥ `md`, `compact` below), content max 720 px, sticky footer with `Voltar` / `Continuar` that never hides the focused field (2.4.11). Dialog forms: `compact` stepper inside the `ResponsiveDialog` (a `Drawer` below `md`) and only when the flow has ≤ 4 steps with ≤ 3 controls each; a dialog that needs more becomes a page.

**Edit** (the entity already exists)
- **Default: one page with sections** (`Card`s in the same order as the create steps), one `Salvar` in the sticky footer, `Cancelar` with the unsaved guard. The user opens an edit to change one thing and needs to reach it directly, so no stepper.
- On ≥ `lg` a sticky "Nesta página" list of anchors (`nav`, current section marked `aria-current`) helps in pages with 4 or more sections; below `lg` it collapses to a `Select`-like jump control or is omitted.
- **Exception: a change that reopens a gated decision** (for example the firm or plan changed and the rule text now differs, or a rule version changed) opens that decision as a **short `Stepper` flow** (review the new rule → choose → review) in a dialog, while the other sections stay as they are. The edit page itself never turns into a stepper.
- Frequent, repeated entry (for example manual trades, S21) still uses the stepper when it is long by the tests above, and offers `Salvar e registrar outro` on the review so repeat use is not slower than a plain form **[Proposal]**.

**Accessibility:** C§2 focus and live-region rules apply. The step title is the `h2` that receives focus; the review is a `dl` per group with `Editar {group}` links that name the group for screen readers ("Editar Dados da conta"). Per-step `fieldset`/`legend` for grouped controls. 3.3.7: nothing the user already typed is asked again; the review does not re-ask. 3.3.8: no step uses a cognitive test.

## 11. Accessibility baseline (WCAG 2.2 AA)

Applies to every screen. Spec-specific notes are in each spec.

- Landmarks: `header`, `nav` (sidebar), `main`, one `h1` per page matching the page title, and a skip link "Pular para o conteúdo" / "Skip to content".
- Every input has a visible `Label`; placeholders are examples only, never labels.
- Focus visible on every interactive element (`--ring`). Focus is not obscured by sticky bars (2.4.11). Logical tab order. Overlays trap focus and restore it to the trigger on close.
- Errors: identified in text, linked to the field, with a suggestion (3.3.1, 3.3.3). Auth flows (code entry) allow paste and autofill and never ask the user to memorize or transcribe from memory (3.3.8); the e-mail typed once is carried to the next step (3.3.7 redundant entry).
- Status messages through live regions (4.1.3): toasts polite; errors on submit announced by moving focus.
- Contrast per §3. Text spacing overrides do not break layouts (1.4.12).
- Language: `<html lang="pt-BR|en">` follows the active locale. Language-switcher options carry their own `lang`.
- Automated: axe checks in Playwright for each screen (QA), plus manual keyboard and screen-reader pass (NVDA + Firefox, VoiceOver + Safari iOS) on the auth flows.

## 12. Open items affecting foundations

| ID | Item | Owner | Effect while open |
|---|---|---|---|
| — | Brand identity (logo, primary color) | User | Neutral primary; logo = product name in text |
| — | Theme switcher dependency (`next-themes` or own) | Frontend / Architect | Either satisfies §4 |
| — | `Intl.NumberFormat` string-decimal support in target browsers | Frontend | Fallback: single `decimal.js` helper |
