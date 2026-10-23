# V1 — NinjaTrader 8 plugin panels (AddOn window) — Screen spec

> **Owner:** Product Designer (Traço @ Dev3) · **Status:** proposed, pending PO review and Plugin Engineer feasibility check · **Revision:** 1 · **Date:** 2026-10-23
> **Surface:** the All Win AddOn window inside NinjaTrader 8 (WPF, `net48`, host project `AllWin.NinjaTrader`). Implemented by the **Plugin Engineer (Elo)**; QA checks on the Windows machine with Sim/Playback only (X-7).
> **Slices covered:** S3 (pairing and connection, plugin side of AC-4), S13 (live risk state and 80 % warnings, AC-3), S15 (enforcement and block until the reset, AC-2/3/4/6/7), S18 (notification list in the plugin, AC-1/2). Web counterparts: [S3](v1-s3-access-sources-pairing.md) (S3-10, S3-11), [S13](v1-s13-risk.md), [S15](v1-s15-enforcement.md), [S18](v1-s18-notifications.md).
> **Inputs read:** [NT8 platform profile](../../architecture/platforms/ninjatrader8-profile.md) (§3 AddOn window, §5 threading, §8 Sim/Playback, §12 lockout, §16 pitfalls), [ADR-0005](../../architecture/adr/0005-nt8-plugin-implementation.md) (N4, N6, N10, N11), [ADR-0004](../../architecture/adr/0004-adapter-protocol-v1.md) (§3 messages, §7 local enforcement, §8 pairing), [messages.schema.json](../../../contracts/adapter-protocol/v1/messages.schema.json), [foundations.md](../foundations.md).
> **Not shadcn.** The web components do not exist here. This spec reuses the **tokens' meaning, the terminology (F§5), the state vocabulary (F§7), the number/time rules (F§6) and the texts**; the controls are standard WPF/NinjaTrader controls. Whatever is not technically feasible is a question for the Plugin Engineer (§11), never an assumption.
> **Language:** pt-BR and en for every string, following the web profile language (`welcome.locale`, Q11, S4 AC-3). Code, identifiers and this document are in English.

---

## 1. Principles for the plugin UI

1. **The trader is trading.** The window never steals focus, never opens a modal dialog on its own, and never covers NinjaTrader's order or chart windows. The only dialogs are the ones the user starts (pair, unpair, allow live accounts).
2. **State is never ambiguous** (F§1): every state is text + icon + color; color alone never carries meaning, including in NinjaTrader's dark and light skins.
3. **The plugin shows, the web decides.** The plugin has **no control that changes limits, turns enforcement on or off, clears an out-of-sync flag, or unblocks an account** (S15 AC-3; S11 and S13 controls live in the web). Its controls are: pair/unpair this computer, allow live accounts on this computer (N11), read and mark notifications.
4. **Honest about freshness.** Risk values are computed in the plugin from live account events and the cached limits, so they stay current when the Core is unreachable; everything that comes from the Core (account sync state, notifications from other sources, the connection name) shows `Sem dados recentes` / `Desatualizado` when the Core is not reachable. Nothing stale is shown as current.
5. **Exact numbers.** Money and prices are `decimal` strings from `AllWin.Adapter.Core` view models; the WPF layer formats them and never converts to `double` (X-3).
6. **No secrets.** The device token and the pairing code history are never displayed or logged (ADR-0004 §8; X-4). A pairing code is shown only while the user types it.

---

## 2. Window architecture

### 2.1 Host

- One AddOn window per NinjaTrader instance (`NTWindow`), opened from a Control Center menu item named `All Win` **[Elo: which menu — New or Tools — and singleton behavior, profile §3]**. Opening it again focuses the existing window.
- The AddOn keeps running (pairing, WebSocket, enforcement, warnings) when the window is closed (profile §3: "does not require its own window"). Closing the window never stops enforcement; the window says so in its footer: "Fechar esta janela não desliga o All Win." / "Closing this window doesn't turn All Win off."
- Window restores its position and size with the NinjaTrader workspace **[Elo]**. Minimum size 640 × 440 px; resizable; content scrolls vertically, never horizontally. DPI scaling follows WPF (device-independent units).

### 2.2 Layout

```
┌ All Win ─────────────────────────────────────────────────────────────────┐
│ ● Conectado   Contas reais: bloqueadas   ▲ 1 conta em alerta   🔔 3       │ header (always visible)
│ ┌ ⚠ Alerta crítico: APEX-1 desconectou com posição aberta. [Ver] ───────┐ │ critical strip (only when a critical item is unread)
│ [ Contas ] [ Avisos (3) ] [ Conexão ]                                    │ NTTabControl
│  … tab content …                                                         │
│ Fechar esta janela não desliga o All Win.                                │ footer
└──────────────────────────────────────────────────────────────────────────┘
```

- **Header strip** (36 px): connection badge (§4.1) · live-accounts state (N11, release builds only; hidden in development builds) · worst risk state across mapped accounts (`Alerta (80 %)`, `Limite atingido`, `Bloqueada`, or nothing) · bell button with unread count → `Avisos` tab.
- **Critical strip:** appears below the header while a `critical` notification is unread (S18 `CriticalBanner` behavior in WPF): no close button; leaves when the item is marked read in the list; `Ver` opens the item in `Avisos`.
- **Tabs:** `Contas` (default when paired), `Avisos`, `Conexão`. When not paired, only `Conexão` is available and is shown full-window (the other tabs are hidden, not disabled).
- **Footer:** the line above and the plugin version.

Tab order: header buttons → critical strip → tabs → content → footer. Keyboard: `Ctrl+Tab` switches tabs (NTTabControl default), `F6` cycles regions **[Elo]**.

---

## 3. Visual language in WPF

### 3.1 Skin

- Backgrounds, text, borders and controls use the **NinjaTrader theme resources** (`DynamicResource`), so the window follows the user's skin. The exact resource keys are the Plugin Engineer's choice; no hard-coded window/background colors.
- Typography: NinjaTrader's default UI font. Body 12 px, section titles 14 px semibold, key figures 18 px semibold. Numbers use a tabular-figure setting (`Typography.NumeralAlignment=Tabular`) if the font supports it, else a monospaced font for numeric columns **[Elo]**.

### 3.2 Status colors (fixed sRGB values of the web tokens)

Converted from the OKLCH tokens of F§3.2. The plugin picks the **light** set when the skin's window background is light and the **dark** set when it is dark (by relative luminance). Contrast measured against `#FFFFFF` (light) and `#0A0A0A` (dark); **QA re-checks against the real NinjaTrader skin backgrounds** (the skin may not be pure white or near-black).

| Status | Light set (text/icon) | Contrast | Dark set (text/icon) | Contrast |
|---|---|---|---|---|
| Success | `#008236` | 4.95 | `#05DF72` | 11.13 |
| Warning | `#BB4D00` | 5.03 | `#FFB900` | 11.49 |
| Danger | `#C10007` | 6.42 | `#FF6467` | 6.85 |
| Info | `#1447E6` | 6.83 | `#51A2FF` | 7.51 |
| Neutral (secondary text) | `#6C6C6C` | 5.25 | `#A1A1A1` | 7.66 |

Badges: a 1 px border in the status color and a transparent or slightly tinted background taken from the skin; the text carries the meaning. P&L: sign always explicit (`+US$ 120,00`, `-US$ 45,50`) with `--pnl` colors as secondary cue (F§3.2).

### 3.3 Icons

Same icon names as the web (lucide: `Plug`, `PlugZap`, `Unplug`, `TriangleAlert`, `OctagonX`, `Lock`, `Bell`, `CircleCheck`, `CircleHelp`, `RefreshCcwDot`, `Bot`...) embedded as **vector `Path` resources** (lucide is ISC-licensed), 16 px, `AutomationProperties.AccessibilityView=Raw`: icons are decorative and the text is the name. No icon font dependency **[Elo]**.

### 3.4 Language, numbers, time

- **Language:** `welcome.locale` (`pt-BR` or `en`); before the first pairing, the Windows UI language if pt or en, else pt-BR (S4 §6). A change in the web applies on the next connection (S4 AC-3). All strings in resource dictionaries; no hard-coded text; a missing key is a test failure, never shown as the key.
- **Money / prices / quantities:** F§6.2 formats using the active locale's culture over `decimal` values: `US$ 1.234,50` / `$1,234.50`, `3 contratos` / `3 contracts`, percent truncated (`79 %`/`79%`). **No `double` anywhere** (X-3; ADR-0001 §8).
- **Time:** 24-hour clock, always with the time zone name next to reset and block times (`17:00 (GMT-3)`). The plugin does not receive the user's profile time zone today (§10 gap), so by default it shows the **computer's Windows time zone**, always labeled; the firm's own reset zone comes from `trading_day.time_zone` (IANA) and is shown in a tooltip like the web (`17:00 CT (19:00 GMT-3)`). **In Playback**, reset and daily P&L use the connection's simulated clock and the UI says so (S15 AC-7, §5.5).

---

## 4. Tab `Conexão` (S3 AC-4; states of S3-11)

### 4.1 Connection badge (header and tab)

| State | Badge pt-BR / en | Token, icon | Notes |
|---|---|---|---|
| Not paired | `Não pareado` / `Not paired` | neutral, `Unplug` | |
| Pairing | `Pareando…` / `Pairing…` | info, `Hourglass` | |
| Connected | `Conectado` / `Connected` | success, `PlugZap` | |
| Disconnected / reconnecting | `Sem conexão` / `Disconnected` | danger, `Unplug` | "Tentando reconectar…" |
| Revoked | `Despareado` / `Unpaired` | neutral, `Ban` | unpaired in the web or moved to another machine (close 4002) |
| Update required | `Atualização necessária` / `Update required` | warning, `CircleArrowUp` | close 4003 |

### 4.2 Content by state

The texts in the S3-11 table are the contract between web and plugin and are used **verbatim**; this section adds the layout.

- **Not paired:** title "Parear este computador" / "Pair this computer"; steps: "1. No All Win, abra Conexões → NinjaTrader e crie uma conexão. 2. Digite aqui o código mostrado." (and en). Code field: single `TextBox`, `MaxLength` tolerant of paste (spaces and hyphens ignored), case-insensitive, shown as `XXXX-XXXX` while typing; maps O→0 and I/L→1 as ADR-0004 §8; button `Parear` enabled when 8 valid characters are present, default button (Enter). No code is ever written to a log.
- **Pairing:** field and button disabled, "Pareando…" with an indeterminate `ProgressBar` (static "Pareando…" text under reduced animation / high contrast).
- **Invalid code** (`pairing.invalid_code`): one message for every cause: "Código inválido ou expirado. Confira o código ou gere um novo no All Win." The field keeps its text and takes focus. **Rate limited / temporarily locked:** "Muitas tentativas. Aguarde {time} e tente de novo." with a visible countdown (text updates every second but is not announced each tick) and the field disabled.
- **Connected:** section **Estado** (badge, connection name as defined in the web [§10 gap], plugin version and build, last connected time with zone), section **Contas do NinjaTrader** (§4.3), section **Segurança deste computador** (§4.4), and the `Desparear este computador` button (§4.5).
- **Disconnected:** "Sem conexão com o All Win. Tentando reconectar…" + "Última conexão: {time}". Below, a fixed note: "Se a aplicação automática estiver ligada, o NinjaTrader continua aplicando os últimos limites recebidos." / "If automatic enforcement is on, NinjaTrader keeps applying the last limits received." (S15 AC-4). No other action.
- **Revoked:** "Esta instância foi despareada ou movida para outra máquina no All Win. Gere um novo código para conectar." The pairing form appears below. State and notifications cache cleared (the unread count shows `—`).
- **Update required:** "Versão do plugin não suportada. Atualize o plugin." + a link to the download page **[dependency: download page URL, S3 spec]**.

### 4.3 NinjaTrader accounts list

A read-only table (`DataGrid` or `ListView`): **Conta no NinjaTrader** (`Account.Name`), **Tipo** (`Simulação` / `Playback` / `Live` / `Desconhecida` badge, from `kind`), **Estado** (`Mapeada` with the role `Destino`/`Master` and the sync badge, or `Não mapeada: ignorada pelo All Win`). Unmapped accounts are listed collapsed under "Contas não vinculadas ({n})" with the hint "Vincule no All Win para o All Win usar esta conta." The plugin never sends commands, reports or enforcement for unmapped accounts (ADR-0004 §3).

### 4.4 Live accounts (ADR-0005 N11) — release builds only

A section "Segurança deste computador" with the always-visible state: "Contas reais: bloqueadas neste computador" / "Real accounts: blocked on this computer" (default) or "permitidas" / "allowed", and one control.
- **Allow:** button `Permitir contas reais…` opens a modal confirmation **that names the connected live accounts**: title "Permitir contas reais neste computador?"; body "O All Win poderá executar ordens roteadas nestas contas reais: {list}. Só você pode mudar isto, e só aqui. O padrão é bloqueado." Buttons `Permitir` / `Cancelar` (`Cancelar` is the default and gets focus). If no live account is connected: "Nenhuma conta real conectada agora. A permissão vale para as que conectarem."
- **Block again:** button `Bloquear contas reais` takes effect at once (including queued commands), **no confirmation** (it only reduces risk).
- Every change is written to the local log with the time (ADR-0005 N11). Development builds do not show this section and never act on live accounts (N4).
- `kind: unknown` accounts are always refused; the account row says "Tipo desconhecido: o All Win não opera nesta conta."

### 4.5 Unpair (local)

`Desparear este computador` opens a confirmation dialog: "Desparear este computador? O plugin esquece a credencial e para de receber ordens." and, **when applicable (the plugin knows its mapped accounts' positions and enforcement state)**: danger text "Há posição aberta em: {accounts}. A posição continua aberta no NinjaTrader, sem monitoramento nem proteção do All Win." and "A aplicação automática de risco deixa de funcionar neste computador." (unmapped accounts are ignored, ADR-0004 §3). A checkbox "Entendo" is required only when a position is open. Buttons `Desparear` (destructive style) / `Cancelar` (default). The web is not told (the Core sees a disconnect; the user finishes in the web, S3-10). **[Rumo: is local unpair allowed with open positions? S3 Q18 covers the web side.]**

---

## 5. Tab `Contas`: live risk state and enforcement (S13, S15)

### 5.1 Layout

A vertical list of **account cards**, one per mapped account, each collapsible (`Expander`). Order: blocked first, then limit reached, then warning, then out of sync, then name. Empty (no mapped account): "Nenhuma conta vinculada. Vincule contas no All Win para ver o risco aqui." Loading before the first limits/state: skeleton lines ("Carregando…").

**Card header (always visible):** NinjaTrader account name (and the All Win name when available [§10 gap]) · kind badge · sync badge (`Fora de sincronia`, F§7.2) · enforcement badge (`Aplicação automática: ligada` / `desligada`) · the account's worst risk badge (`Alerta (80 %)`, `Limite atingido`, `Bloqueada até {time}`, `Meta atingida`) · expand chevron. Cards with a warning or worse start expanded.

### 5.2 Card body

1. **Trading-day line:** "Dia de trading termina às 17:00 (GMT-3) · reset em {date-time zone}" (from `trading_day`); Playback variant §5.5. Tooltip: firm's zone.
2. **Meters** (the WPF equivalent of `RiskMeter`, F§7.3), one per limit that exists in the cached `limits` version:

| Meter | Content |
|---|---|
| Perda diária | `US$ 820,00 de US$ 1.000,00 · 82 %` |
| Drawdown ({type}) | `used / limit` and "Piso atual: US$ …" when `floor_balance` is present |
| Meta de lucro diária | progress, success color, text `Meta atingida` when reached |
| P&L do dia | realized + unrealized as signed money (information, not a limit) |

   - Each meter is a `ProgressBar` (0–100) with a fixed **80 % tick**, a text value beside it, and the state badge when not `normal` (F§7.1: exactly three levels, **no 90 % band**). The fill color follows the state tokens; the text and badge carry the meaning.
   - `AutomationProperties.Name` = limit name; `AutomationProperties.HelpText`/value text = the full sentence ("Perda diária: US$ 820,00 de US$ 1.000,00, 82 %, alerta"); UIA `RangeValue` provider so Narrator reads it.
   - Values come from `risk_state` (`realized_pnl`, `unrealized_pnl`, `daily_loss_remaining`, `drawdown_remaining`, `balance`) and the cached `limits`; percentages are computed by `AllWin.Adapter.Core` with `decimal` and **truncated** so `80 %` is never shown while the state is `normal`. Redraws are throttled to 2 per second per account (matches `risk_state`). No announcement per tick; only state changes (§5.4).
   - A limit not set shows no meter: "Perda diária: sem limite definido."
3. **Limits footer:** "Limites versão {n}, recebidos às {time}" and, if the plugin holds a version older than the Core's, "Desatualizados" (only knowable while connected).
4. **Block card** when `blocked_until` is set (§5.3).
5. **Recent events** (last 5 risk events of this account today, from the plugin's local record): time with seconds, text, `while_disconnected` mark. Empty: "Nenhum evento de risco hoje."

### 5.3 Blocked until the reset (S15 AC-2, 3, 6)

A card section with danger border (success border when the reason is the daily profit target, F§7.1) and text:
- Title: "Conta bloqueada até {date-time (zone)}" + reason: "motivo: perda diária atingida" / "drawdown atingido" / "meta de lucro diária atingida".
- Why: limit name, limit, value that triggered it, time with seconds.
- What was done: "Posições fechadas: {n}. Ordens canceladas: {m}." (from the plugin's own actions).
- What happens now (fixed text, AC-6): "Qualquer ordem manual enviada nesta conta será cancelada e a posição resultante será fechada logo depois. O NinjaTrader não consegue travar a conta. **Não há desbloqueio antecipado.**"
- A list "Ordens canceladas desde o bloqueio" with time and order summary (each `blocked_order_cancelled`).
- **No button unblocks** (S15 AC-3). The block ends only at the reset, when the card disappears and the account shows "Liberada às {time}" for the rest of the session.
- If a flatten or cancel failed: `Alert`-style row (danger): "Não foi possível fechar tudo em {account}. A conta foi marcada fora de sincronia." (the account is flagged; the plugin waits for the user, never auto-resyncs).

### 5.4 Warnings and state changes (S13 AC-3, S15 AC-6)

| Event (`risk_event.kind`) | In the window | Outside the window (§7) | Notification |
|---|---|---|---|
| `warning` (80 %) | badge + meter state, header chip | NinjaTrader Log (warning) + non-modal toast, auto-hides after 10 s | yes (`risk.warning`, once per state change) |
| `limit_reached` | badge, meter | Log (error) + toast that stays until dismissed | yes |
| `flattened`, `orders_cancelled`, `blocked` | block card | Log + persistent toast | yes |
| `blocked_order_cancelled` | row in the block card | Log + toast "Ordem cancelada: conta {account} bloqueada até {time}" (this is how the user is told **at that moment** that the account is blocked and why, AC-6) | yes |
| `unblocked` (reset) | card disappears; "Liberada" line | Log (info) | informational (Q62 provisional) |
| daily profit target reached | `Meta atingida`; with `flatten_and_block` the block card applies | Log + toast | once |

State changes are announced once through UI Automation (`LiveSetting=Polite`; limit reached/block `Assertive`), never per tick.

### 5.5 Playback and Sim discontinuities

- For `kind: playback` the trading-day line reads "Relógio de Playback: o reset e o P&L do dia seguem o relógio simulado ({simulated date-time})", and `blocked_until`/reset times are in that clock (`platform_time`), labeled; never mixed with the computer's clock (S15 AC-7).
- A Sim reset or Playback rewind is a discontinuity (S11 AC-6): the card shows warning text "A simulação foi reiniciada; os valores do dia recomeçaram. Isto não conta como resultado." and the sync badge `Fora de sincronia` (S11 owns the flag).

### 5.6 Enforcement badge

Shown from `limits.enforcement_enabled`: `Aplicação automática: ligada` (info) or `desligada` (neutral). When off, a one-line note under the badge: "Com a função desligada, o All Win ainda recusa novas entradas ao atingir um limite, mas o NinjaTrader não fecha posições nem bloqueia a conta." The plugin has no switch (§1.3); the card says "Altere no All Win → Conta → Risco." when the user hovers or focuses the badge (tooltip with the same text in the accessible description).

---

## 6. Tab `Avisos` (S18 AC-1, AC-2)

The notification list in the plugin is the same set and the same unread count as the web (PO: one set, one count; reading on either side counts on both).

- **Toolbar:** `ToggleButton`s `Todas` / `Não lidas`; button `Marcar todas como lidas`; the unread count.
- **List** (`ListView`, virtualized, newest first; critical unread pinned first, then unread by time, then read): each item shows severity icon + sr-only/automation name ("Crítica", "Atenção", …), title, body (1–2 lines), account, absolute time with zone, `×{n}` and "última às {time}" when grouped, a text badge `Nova` when unread, and the buttons `Marcar como lida` (read items show "Lida"). A hint "Abrir no All Win" (text) names where to continue; if opening the default browser to the signed-in app is feasible it becomes a link **[Elo]**.
- **Catalog:** the codes, severities, titles and bodies are those of the S18 catalog (§6 of the S18 spec) and S10 catalog; the plugin keeps its own pt-BR/en resource copy for the codes it can render; an unknown code renders "Aviso {code}" with the details collapsed. Grouping (same code + account + trading day) is done by the Core.
- **Window limits:** the last 200 items are cached; below the list "Mostrando os últimos 200 avisos. Os mais antigos estão no All Win."
- **States:** loading (first sync) "Carregando…"; empty "Nenhum aviso." ; disconnected: banner "Sem conexão: a lista pode estar incompleta." and the count shows `3*` with the tooltip "Alguns avisos podem ainda não ter sido sincronizados."; Revoked/Not paired: tab hidden.
- **Offline behavior:** events the plugin itself raised while the Core was unreachable (warnings, flatten, block, reactive cancels, `while_disconnected: true`) appear locally with the tag `Ainda não sincronizado`; after reconnection the Core creates the notifications from the delivered `risk_event`s and the local copies are replaced (matched by `event_id`). Marking read while offline is queued and sent on reconnect.
- **Critical items:** `critical` notifications also drive the header strip (§2.2). The strip uses `AutomationProperties.LiveSetting=Assertive` only when the item arrives while the window is open; on window load it is a labeled region and does not re-announce.

---

## 7. When the window is closed

Enforcement, warnings and notifications keep running without the window (§2.1). The user must still be told:

| Surface | Use | Status |
|---|---|---|
| NinjaTrader Control Center **Log** tab | every warning, limit reached, block, reactive cancel, disconnect with open position, pairing change | **[Elo: `NinjaScript.Log` levels and wording]** |
| **Non-modal toast** (small, `ShowActivated=false`, topmost only while shown, bottom-right corner, never over the order entry window if it can be avoided) | warnings (10 s), and persistent for limit reached / block / reactive cancel / critical until `Entendi` is pressed; the toast has `Abrir All Win` which opens the window; dismissing the toast does not mark the notification read | **[Elo: feasibility and focus behavior in NT8]** |
| Window icon / menu item state | optional unread count on the Control Center menu item | **[Elo: probably not possible]** |
| Sounds | none in V1 (not in the ACs) | |

If the toast cannot be non-activating, the fallback is the Log only plus the window left open; the spec then recommends instructing the user to keep the All Win window in the workspace. That is a product question for the PO (§12).

---

## 8. Strings (pt-BR / en; keys follow the plugin's own resource naming)

| Where | pt-BR | en |
|---|---|---|
| Window title | All Win | All Win |
| Tabs | Contas · Avisos · Conexão | Accounts · Notices · Connection |
| Connection states | Não pareado · Pareando… · Conectado · Sem conexão · Despareado · Atualização necessária | Not paired · Pairing… · Connected · Disconnected · Unpaired · Update required |
| Pair | Parear | Pair |
| Unpair | Desparear este computador | Unpair this computer |
| Live accounts | Contas reais: bloqueadas neste computador / permitidas | Real accounts: blocked on this computer / allowed |
| Risk | Alerta (80 %) · Limite atingido · Meta atingida | Warning (80%) · Limit reached · Target reached |
| Blocked | Bloqueada até {time} · motivo: {reason} | Blocked until {time} · reason: {reason} |
| No unlock | Não há desbloqueio antecipado. | There is no early unlock. |
| Reactive block | O NinjaTrader não consegue travar a conta. | NinjaTrader can't lock the account. |
| No data | Sem dados recentes | No recent data |
| Notifications | Marcar como lida · Marcar todas como lidas · Nova · Lida | Mark as read · Mark all as read · New · Read |
| Unsynced | Ainda não sincronizado | Not synced yet |
| Footer | Fechar esta janela não desliga o All Win. | Closing this window doesn't turn All Win off. |

Terms follow F§5 (conta, limite, alerta, fora de sincronia, parear/desparear, plataforma de execução...). Texts must not sound like legal advice.

---

## 9. Accessibility and keyboard (WPF / UI Automation)

- Standards: WCAG 2.2 AA intent applied to a desktop app (the project baseline); native accessibility via UI Automation. Every control has `AutomationProperties.Name`; badges expose their text; icons are `Raw` (not exposed).
- Full keyboard use: logical tab order (§2.2), visible focus rectangle (high-contrast capable), `Enter` activates the default button, `Esc` closes dialogs and the toast, arrow keys move in lists and tabs; no mouse-only action.
- Contrast: §3.2; the window respects Windows **High Contrast** by falling back to system brushes **[Elo]**; no information in color alone (every state has text + icon).
- Live changes: UIA live regions as in §5.4 and §6; no per-tick announcements; the countdown of the pairing lock updates its text but is not announced each second.
- Targets: buttons ≥ 28 × 28 px at 100 % DPI (desktop pointer, WCAG 2.5.8 minimum 24 px); touch is not a target of the NinjaTrader window.
- Text scaling: layouts use `*` and `Auto` sizing, no fixed-width labels, so the ~35 % growth between pt-BR and en and Windows text scaling do not clip (F§1).
- Reduced motion: no animated transitions; indeterminate progress becomes static text.
- Screen reader pass: Narrator and NVDA on the Conexão flow and on a risk state change (QA, Windows machine).

---

## 10. Data the panels need and protocol gaps (for the Architect and the Plugin Engineer)

**Already in the protocol (ADR-0004 / `messages.schema.json`):**

| Panel need | Source |
|---|---|
| Language | `welcome.locale` |
| Mapped accounts, roles, sync state and reasons | `welcome.accounts[]` and `account_state` (1.1) |
| Live-account guard | `welcome.simulation_only`, local N11 setting |
| Limits, enforcement flag, `blocked_until`, trading day (reset time + IANA zone), warning threshold, drawdown floor | `limits` (versioned) |
| Live P&L and remaining limits | plugin-computed, sent as `risk_state` (A→C); the panel reads the plugin's own view model |
| Risk events (warning, limit reached, flattened, orders cancelled, blocked, unblocked, blocked order cancelled), `while_disconnected`, `platform_time` | `risk_event` |
| Pairing errors | S3-11 / ADR-0004 §8 codes |

**Gaps (not in the protocol today; proposals for the Architect, plugin feasibility for Elo):**

| # | Gap | Needed for | Proposal |
|---|---|---|---|
| G1 | **Notifications to the plugin** and **read-state sync**: there is no C→A notification message and no read acknowledgement | S18 in the plugin (one set, one unread count) | New reliable messages: `notification` (C→A: id, created/updated at, code, severity, details, account_id, count, read_at) and `notification_read` (both ways: ids, read_at); plus an initial sync of the last 200 on connect (a protocol 1.x minor, ADR-0004 versioning) |
| G2 | **Connection name** (as the user named it in the web) | `Conexão` tab | Add to `welcome` |
| G3 | **All Win account name** for each mapped account (the plugin knows only `platform_account`) | account cards and notices | Add `display_name` to `welcome.accounts[]` |
| G4 | **User display time zone** (profile) | times in the plugin match the web (S4 AC-1) | Add to `welcome` (otherwise the Windows zone is used, labeled) |
| G5 | **Consistency rule state** (S13 AC-6: tracked and warned by the Core, not in `limits`) | the 80 % warning for consistency in the plugin (S13 AC-3 says warnings appear in both) | Deliver as notifications (G1); no meter in the plugin |
| G6 | **App base URL** and account deep links | "Abrir no All Win" | Plugin config (not a secret); Elo to confirm acceptable |
| G7 | **Enforcement explanation for the offline/unmapped case** | §4.5 | Document in ADR-0004 §7 that unpairing stops enforcement |
| G8 | **Download page URL** for "Update required" | S3-11 dependency | Static link in the plugin |

---

## 11. Feasibility questions for the Plugin Engineer (Elo)

1. **Menu entry:** which Control Center menu (New / Tools) hosts `All Win`, and can the AddOn keep a singleton `NTWindow` restoring position/size with the workspace?
2. **Skin colors:** which NinjaTrader theme resource keys give background, text, border, and the light/dark luminance so §3.2 can pick its set? Is high-contrast handled by the theme?
3. **Tabular numerals:** does the default NinjaTrader font support `Typography.NumeralAlignment=Tabular`; else which monospaced font?
4. **Toasts:** can the AddOn open a small non-activating, non-modal topmost window (`ShowActivated=false`) without stealing focus from order entry, and does it behave on multi-monitor setups?
5. **Log:** `NinjaScript.Log` / `Print`: severity levels shown in the Control Center Log tab, and rate limits to avoid flooding.
6. **UIA:** can `AutomationProperties.LiveSetting` and a `RangeValue` provider be used inside `NTWindow` content without fighting the host?
7. **Open browser:** is launching the default browser to the app URL acceptable from the AddOn (no secrets in the URL)?
8. **DPI and size:** minimum window size 640 × 440, scrolling, per-monitor DPI inside NT8.
9. **Threading:** 2 updates per second per account for up to N accounts through `Dispatcher.InvokeAsync` with a single UI-bound view model (ADR-0005 N6); any measured limit on the number of cards?
10. **Pairing field:** paste handling and masking of the code in the field (it is short-lived, not masked) and log redaction.
11. **Local record:** the "recent events" and offline notices need a small durable local store of `risk_event`s (the plugin already keeps an outbox); acceptable size and retention (one trading day for the UI, the outbox for delivery)?

---

## 12. Open questions for the PO (Rumo)

| ID | Question | Spec default |
|---|---|---|
| Q-P1 | If a non-activating toast is not feasible, is the Log plus a window kept open acceptable for warnings and blocks? Or does the PO want the window to open itself on a block (it would take focus)? | Log + toast if feasible; never steal focus |
| Q-P2 | Local unpair with an open position or enforcement on: allowed with the warning in §4.5? | Allowed with the checkbox |
| Q-P3 | Should the plugin offer a way to clear the out-of-sync flag, or only the web (S11 AC-3)? | Web only |
| Q-P4 | Consistency warnings in the plugin as notifications only (no meter) | Yes |
| Q-P5 | Notification retention in the plugin: last 200 items | Yes |
| Q-P6 | Sound for warnings and blocks | None in V1 |

## 13. AC traceability

| AC | Where |
|---|---|
| S3 AC-4 / S3-11 pair, unpair, states, one message for invalid codes | §4 |
| N11 (U15) live accounts opt-in, state always visible | §2.2 header, §4.4 |
| S13 AC-2, AC-3 live risk state, 80 % warning in the plugin | §5.2, §5.4 |
| S13 AC-4 trading-day reset (firm / CME), time zone shown | §5.2 trading-day line, §3.4 |
| S15 AC-1 off by default (shown), AC-2 flatten/cancel/block, AC-3 no early unlock, AC-4 works offline, AC-6 reactive block explained, AC-7 Playback clock | §5.3, §5.5, §5.6, §4.2 |
| S18 AC-1, AC-2 notifications visible in the plugin; mark read; unread count | §2.2, §6 |
| X-3, X-4, X-5, X-6, X-7, X-8 | §1.5, §1.6, §5.3 audit-backed events, §3.4 language, simulation only, time |
