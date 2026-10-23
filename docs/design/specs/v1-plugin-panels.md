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
3. **The plugin shows, the web decides.** The plugin has **no control that changes limits, turns enforcement on or off, clears an out-of-sync flag, or unblocks an account** (S15 AC-3; S11 and S13 controls live in the web). Its controls are: pair this computer, **disconnect/reconnect this computer** (§4.5; revoking a pairing is done in the web), allow live accounts on this computer (N11), read and mark notifications.
4. **Honest about freshness.** Risk values are computed in the plugin from live account events and the cached limits, so they stay current when the Core is unreachable; everything that comes from the Core (account sync state, notifications from other sources, the connection name) shows `Sem dados recentes` / `Desatualizado` when the Core is not reachable. Nothing stale is shown as current.
5. **Exact numbers.** Money and prices are `decimal` strings from `AllWin.Adapter.Core` view models; the WPF layer formats them and never converts to `double` (X-3).
6. **No secrets.** The device token and the pairing code history are never displayed or logged (ADR-0004 §8; X-4). A pairing code is shown only while the user types it.

---

## 2. Window architecture

### 2.1 Host

- One AddOn window per NinjaTrader instance (`NTWindow` with `IWorkspacePersistence`), opened from the Control Center **New** menu, item `All Win` (the documented host; Elo, profile §3). Opening it again with the window open activates it (the user clicked, so focus is expected). If NinjaTrader restores a duplicate window from a second workspace, the duplicate closes itself without activating anything [EXP].
- The AddOn keeps running (pairing, WebSocket, enforcement, warnings) when the window is closed (profile §3: "does not require its own window"). Closing the window never stops enforcement; the window says so in its footer: "Fechar esta janela não desliga o All Win." / "Closing this window doesn't turn All Win off."
- The window position and size are saved and restored with the NinjaTrader workspace (`WorkspaceOptions`, including the off-screen check). Minimum size 640 × 440 px; resizable; content scrolls vertically, never horizontally. DPI scaling follows WPF; per-monitor behavior depends on NinjaTrader's own manifest, which the plugin does not control [EXP]. The unread count is shown in the **window title** (`All Win (3)`, visible in the taskbar), not in the menu item.

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

Tab order: header buttons → critical strip → tabs → content → footer. Keyboard: `Ctrl+Tab` switches tabs (NTTabControl default); `F6` cycles regions via a `KeyBinding` on our window [EXP: conflict with NinjaTrader hot keys].

---

## 3. Visual language in WPF

### 3.1 Skin

- Backgrounds, text, borders and controls use the **NinjaTrader theme resources** (`DynamicResource`), so the window follows the user's skin. The exact resource keys are the Plugin Engineer's choice; no hard-coded window/background colors.
- Typography: NinjaTrader's effective UI font (read at load; Segoe UI digits are already fixed-width). Body 12 px, section titles 14 px semibold, key figures 18 px semibold. Numbers use `Typography.NumeralAlignment=Tabular` (harmless if the font lacks it); fallback **Consolas, only in numeric columns** [EXP: test that `1111` and `0000` have the same width].

### 3.2 Status colors (fixed sRGB values of the web tokens)

Converted from the OKLCH tokens of F§3.2. The plugin chooses between the **light** and the **dark** set **by measured contrast, not by a luminance threshold**: at load it reads the window's resolved `Background` (for a non-solid brush, the average of its stops), computes the contrast of **both** sets against that real color, and uses the set whose lowest contrast is higher; it recomputes if the resource changes (whether a skin change needs a NinjaTrader restart is [EXP]). Reference contrast below is against `#FFFFFF` (light) and `#0A0A0A` (dark); **QA re-checks on the real skins**.

Standard controls inside the `NTWindow` inherit the skin's implicit styles (background, text, border need no resource keys); resource keys are only needed for our own elements (badge, meter, critical strip). They live in the skin's `BluePrint.xaml` and are undocumented, so Elo lists the keys on the Windows machine and fixes a list with `TryFindResource` → `SystemColors` fallback [EXP].

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
- **Time:** 24-hour clock, always with the time zone name next to reset and block times (`17:00 (GMT-3)`). From protocol 1.3 the plugin receives the user's profile time zone (`welcome.time_zone`, applied at the next reconnection); before 1.3 it shows the **time zone configured in NinjaTrader** (Tools > Options > General; the same one the Log tab uses, [EXP: `Globals.GeneralOptions`]) and, as a last resort, the Windows zone, always labeled; IANA names are mapped to Windows ids with a generated CLDR table in `AllWin.Adapter.Core` (already needed for `trading_day.time_zone`); the firm's own reset zone comes from `trading_day.time_zone` (IANA) and is shown in a tooltip like the web (`17:00 CT (19:00 GMT-3)`). **In Playback**, reset and daily P&L use the connection's simulated clock and the UI says so (S15 AC-7, §5.5).

---

## 4. Tab `Conexão` (S3 AC-4; states of S3-11)

### 4.1 Connection badge (header and tab)

| State | Badge pt-BR / en | Token, icon | Notes |
|---|---|---|---|
| Not paired | `Não pareado` / `Not paired` | neutral, `Unplug` | |
| Pairing | `Pareando…` / `Pairing…` | info, `Hourglass` | |
| Connected | `Conectado` / `Connected` | success, `PlugZap` | |
| Disconnected / reconnecting | `Sem conexão` / `Disconnected` | danger, `Unplug` | "Tentando reconectar…" |
| Disconnected by you | `Desconectado por você` / `Disconnected by you` | neutral, `Unplug` | after `Desconectar este computador` (§4.5); stays until `Reconectar` |
| Revoked | `Despareado` / `Unpaired` | neutral, `Ban` | unpaired in the web or moved to another machine (close 4002) |
| Update required | `Atualização necessária` / `Update required` | warning, `CircleArrowUp` | close 4003 |

### 4.2 Content by state

The texts in the S3-11 table are the contract between web and plugin and are used **verbatim**; this section adds the layout.

- **Not paired:** title "Parear este computador" / "Pair this computer"; steps: "1. No All Win, abra Conexões → NinjaTrader e crie uma conexão. 2. Digite aqui o código mostrado." (and en). Code field: single `TextBox`, `MaxLength` tolerant of paste (spaces and hyphens ignored), case-insensitive, shown as `XXXX-XXXX` while typing; maps O→0 and I/L→1 as ADR-0004 §8; button `Parear` enabled when 8 valid characters are present, default button (Enter). No code is ever written to a log.
- **Pairing:** field and button disabled, "Pareando…" with an indeterminate `ProgressBar` (static "Pareando…" text under reduced animation / high contrast).
- **Invalid code** (`pairing.invalid_code`): one message for every cause: "Código inválido ou expirado. Confira o código ou gere um novo no All Win." The field keeps its text and takes focus. **Rate limited / temporarily locked:** "Muitas tentativas. Aguarde {time} e tente de novo." with a visible countdown (text updates every second but is not announced each tick) and the field disabled.
- **Connected:** section **Estado** (badge, connection name as defined in the web (`welcome.connection_name`, protocol 1.3), plugin version and build, last connected time with zone), section **Contas do NinjaTrader** (§4.3), section **Segurança deste computador** (§4.4), and the `Desconectar este computador` button (§4.5).
- **Disconnected:** "Sem conexão com o All Win. Tentando reconectar…" + "Última conexão: {time}". Below, a fixed note: "Se a aplicação automática estiver ligada, o NinjaTrader continua aplicando os últimos limites recebidos." / "If automatic enforcement is on, NinjaTrader keeps applying the last limits received." (S15 AC-4). No other action.
- **Revoked:** "Esta instância foi despareada ou movida para outra máquina no All Win. Gere um novo código para conectar." and "A aplicação automática de risco deixou de funcionar neste computador para as contas desvinculadas." (G7). The pairing form appears below. State and notifications cache cleared (the unread count shows `—`).
- **Update required:** "Versão do plugin não suportada. Atualize o plugin." + a link to `/<locale>/download` on the build's origin constant (G8).

### 4.3 NinjaTrader accounts list

A read-only table (`DataGrid` or `ListView`): **Conta no NinjaTrader** (`Account.Name`), **Tipo** (`Simulação` / `Playback` / `Live` / `Desconhecida` badge, from `kind`), **Estado** (`Mapeada` with the role `Destino`/`Master` and the sync badge, or `Não mapeada: ignorada pelo All Win`). Unmapped accounts are listed collapsed under "Contas não vinculadas ({n})" with the hint "Vincule no All Win para o All Win usar esta conta." The plugin never sends commands, reports or enforcement for unmapped accounts (ADR-0004 §3).

### 4.4 Live accounts (ADR-0005 N11) — release builds only

A section "Segurança deste computador" with the always-visible state: "Contas reais: bloqueadas neste computador" / "Real accounts: blocked on this computer" (default) or "permitidas" / "allowed", and one control.
- **Allow:** button `Permitir contas reais…` opens a modal confirmation **that names the connected live accounts**: title "Permitir contas reais neste computador?"; body "O All Win poderá executar ordens roteadas nestas contas reais: {list}. Só você pode mudar isto, e só aqui. O padrão é bloqueado." Buttons `Permitir` / `Cancelar` (`Cancelar` is the default and gets focus). If no live account is connected: "Nenhuma conta real conectada agora. A permissão vale para as que conectarem."
- **Block again:** button `Bloquear contas reais` takes effect at once (including queued commands), **no confirmation** (it only reduces risk).
- Every change is written to the local log with the time (ADR-0005 N11). Development builds do not show this section and never act on live accounts (N4).
- `kind: unknown` accounts are always refused; the account row says "Tipo desconhecido: o All Win não opera nesta conta."

### 4.5 Disconnect this computer (local) **[Provisional, Q66 rec. (a); user decides]**

The plugin does **not** revoke the pairing. `Desconectar este computador` only **stops the connection from this computer**; the pairing stays in the Core as `Desconectada` with the usual out-of-sync rules (S11), and **revoking the pairing is done in the web with a code** (S3-10; Q18 stays: confirmation listing the accounts, strong warning with an open position or enforcement on, nothing is closed).
- Dialog: title "Desconectar este computador?"; body "O All Win para de enviar ordens e de receber as suas execuções neste computador. No All Win a conexão aparece como desconectada. Para encerrar o pareamento de vez, desparear no All Win → Conexões." and, when applicable (the plugin knows its mapped accounts' positions and orders): danger text "Há posição aberta em: {accounts}. Ela continua aberta no NinjaTrader e o All Win não a acompanhará; as contas ficarão fora de sincronia no All Win." with a required checkbox "Entendo" only in that case. If enforcement is on: "Com a aplicação automática ligada, o NinjaTrader continua aplicando os últimos limites recebidos, como em uma queda de conexão." **[Elo: confirm that local enforcement keeps running with the cached limits after a user disconnect, ADR-0004 §7]**
- With an open position or a command in flight, the account is flagged out of sync and an alert is raised (`sync.disconnected_by_user`: "Você desconectou o plugin com posição aberta; o All Win não está acompanhando esta conta"); without a position there is only the status and the audit entry, no alert (ADR-0004 §6).
- Buttons `Desconectar` (destructive style) / `Cancelar` (default).
- Result: badge `Desconectado por você`, the plugin does not reconnect on its own, a `Reconectar` button reconnects with the stored credential (no code needed). The credential stays in the protected store; **[Elo/PO: whether to forget it]**. The same state survives a NinjaTrader restart until the user reconnects **[Elo]**.
- To remove the credential from this computer the user revokes in the web (state `Despareado`, back to the pairing form).

---

## 5. Tab `Contas`: live risk state and enforcement (S13, S15)

### 5.1 Layout

A vertical list of **account cards**, one per mapped account, each collapsible (`Expander`). Order: blocked first, then limit reached, then warning, then out of sync, then name. Empty (no mapped account): "Nenhuma conta vinculada. Vincule contas no All Win para ver o risco aqui." Loading before the first limits/state: skeleton lines ("Carregando…").

**Card header (always visible):** NinjaTrader account name (and the All Win name, `display_name`, protocol 1.3) · kind badge · sync badge (`Fora de sincronia`, F§7.2) · enforcement badge (`Aplicação automática: ligada` / `desligada`) · the account's worst risk badge (`Alerta (80 %)`, `Limite atingido`, `Bloqueada até {time}`, `Meta atingida`) · expand chevron. Cards with a warning or worse start expanded.

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
- **List** (`ListView`, virtualized, newest first; critical unread pinned first, then unread by time, then read): each item shows severity icon + sr-only/automation name ("Crítica", "Atenção", …), title, body (1–2 lines), account, absolute time with zone, `×{n}` and "última às {time}" when grouped, a text badge `Nova` when unread, and the buttons `Marcar como lida` (read items show "Lida"). `Abrir no All Win` opens `<welcome.app_url>/<locale>/accounts/<account_id>` (absolute https origin from `welcome.app_url`, no secret, G6) in the default browser **[Elo: feasibility]**; items without an account open the notifications list in the web.
- **Source and rendering (protocol 1.3, G1):** the Core sends the snapshot (`notifications`, up to 200) after `welcome` and upserts (`notification`); the plugin renders each item from `code` + `params` (codes and values, not text) with its own pt-BR/en catalog, and marks read with `notification_read`.
- **Catalog:** the codes, severities, titles and bodies are those of the S18 catalog (§6 of the S18 spec) and S10 catalog; the plugin keeps its own pt-BR/en resource copy for the codes it can render; an unknown code renders "Aviso {code}" with the details collapsed. Grouping (same code + account + trading day) is done by the Core.
- **Window limits:** the last 200 items are a display cache (the record is the audit log, PO); below the list "Mostrando os últimos 200 avisos. Os mais antigos estão no All Win."
- **States:** loading (first sync) "Carregando…"; empty "Nenhum aviso." ; disconnected: banner "Sem conexão: a lista pode estar incompleta." and the count shows `3*` with the tooltip "Alguns avisos podem ainda não ter sido sincronizados."; Revoked/Not paired: tab hidden.
- **Offline behavior:** events the plugin itself raised while the Core was unreachable (warnings, flatten, block, reactive cancels, `while_disconnected: true`) appear locally with the tag `Ainda não sincronizado`; after reconnection the Core creates the notifications from the delivered `risk_event`s and each local copy whose adapter `event_id` is on the notification's `source_event_ids` list (protocol 1.3, a dedicated field, not in `params`) is replaced by that notification: the same list item, keeping its position. A notification without `source_event_ids` (born in the Core, e.g. consistency or Core `sync.*`) is always a new item. Never match by time + code. Marking read while offline is queued and sent on reconnect.
- **Critical items:** `critical` notifications also drive the header strip (§2.2). The strip uses `AutomationProperties.LiveSetting=Assertive` only when the item arrives while the window is open; on window load it is a labeled region and does not re-announce.

---

## 7. When the window is closed

Enforcement, warnings and notifications keep running without the window (§2.1). The user must still be told:

| Surface | Use | Status |
|---|---|---|
| NinjaTrader Control Center **Log** tab (`NinjaScript.Log`) | see the level table below | feasible (Elo) |
| **Non-modal toast**: a plain WPF `Window` (not an `NTWindow`, so it is not saved in the workspace) on its own STA thread and `Dispatcher`, `WindowStyle=None`, `ShowActivated=false`, `ShowInTaskbar=false`, `Topmost` only while visible, with `WS_EX_NOACTIVATE \| WS_EX_TOOLWINDOW` so even clicking its buttons never takes activation from the order window | warnings auto-hide after 10 s; limit reached, block, reactive cancel and critical stay until `Entendi`; `Abrir All Win` opens the window; dismissing a toast never marks the notification read | feasible [INF]; **[EXP]** on the Windows machine |
| Window title count | `All Win (3)` in the taskbar | feasible |
| Sounds | none in V1 | |

**Toast rules.** Bottom-right corner of the work area of the monitor that hosts the Control Center; at most **3 stacked**, then "+n"; small. A topmost toast can still intercept a click aimed at an order ticket below it, so it stays small and capped; avoiding the order window by detecting overlap is **phase 2** (fixed corner in V1). Because it has no keyboard focus, **every toast has a mirror in the window** (`Avisos` and the critical strip, §6) with the same `Entendi`. It raises a UI Automation notification for screen readers.

**Log levels (never `LogLevel.Alert`, which opens a pop-up window and would steal focus):**

| Level | Events |
|---|---|
| Warning | 80 % warning |
| Error | limit reached; flattened, orders cancelled, blocked; reactive cancel; failure to flatten or cancel; disconnect with an open position |
| Information | unblocked at the reset; paired, unpaired, revoked, connected; live accounts allowed or blocked (N11) |

One log line per state change (same dedupe key as the warning); reactive-cancel bursts collapse to at most one line per account every 5 s with "(+n)", plus a global per-minute cap, because each entry costs memory in the Control Center. Text follows `welcome.locale` and never contains a pairing code or token (Adapter.Core redaction). The Log timestamp follows NinjaTrader's configured time zone.

**The plugin never steals focus [Provisional, Q65 rec. (a)]**: Log + non-activating toast + the permanent status on each account in the `Contas` tab (badge and block card). The window never opens itself. The user is advised to keep the All Win window in their workspace (footer hint).

**Alternative to evaluate (Q-P7, PO):** `NinjaScript.Alert.AlertCallback` writes to NinjaTrader's **Alerts Log** window (priority, colors, re-arm, optional sound; none in V1), which traders already watch. It could complement the toast for limit reached and block. [EXP] whether the Alerts window opens by itself or takes focus; not the default until verified.

---

## 8. Strings (pt-BR / en; keys follow the plugin's own resource naming)

| Where | pt-BR | en |
|---|---|---|
| Window title | All Win | All Win |
| Tabs | Contas · Avisos · Conexão | Accounts · Notices · Connection |
| Connection states | Não pareado · Pareando… · Conectado · Sem conexão · Desconectado por você · Despareado · Atualização necessária | Not paired · Pairing… · Connected · Disconnected · Disconnected by you · Unpaired · Update required |
| Pair | Parear | Pair |
| Disconnect / reconnect | Desconectar este computador · Reconectar | Disconnect this computer · Reconnect |
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
- Full keyboard use: logical tab order (§2.2), visible focus rectangle, `Enter` activates the default button, `Esc` closes dialogs, arrow keys move in lists and tabs; no mouse-only action in the window. **The toast never takes keyboard focus** (non-activating), so `Esc`/`Tab` do not reach it: every toast is mirrored in the window (`Avisos` tab and the critical strip) with the same `Entendi` action, and that is the keyboard-accessible path.
- Contrast: §3.2; with Windows **High Contrast** (`SystemParameters.HighContrast`) our window swaps status and text colors for `SystemColors` (NinjaTrader's own chrome does not follow High Contrast; QA checks); no information in color alone (every state has text + icon). Screen readers: toasts raise `AutomationPeer.RaiseNotificationEvent` since a non-activated window is not announced by itself [EXP: Narrator/NVDA].
- Live changes: UIA live regions as in §5.4 and §6; no per-tick announcements; the countdown of the pairing lock updates its text but is not announced each second.
- Targets: buttons ≥ 28 × 28 px at 100 % DPI (desktop pointer, WCAG 2.5.8 minimum 24 px); touch is not a target of the NinjaTrader window.
- Text scaling: layouts use `*` and `Auto` sizing, no fixed-width labels, so the ~35 % growth between pt-BR and en and Windows text scaling do not clip (F§1).
- Reduced motion: no animated transitions; indeterminate progress becomes static text.
- Screen reader pass: Narrator and NVDA on the Conexão flow and on a risk state change (QA, Windows machine).

---

## 10. Data the panels need and protocol decisions (G1–G8)

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

**Gaps G1–G8: answered by the protocol owner (Esquadro), protocol 1.3, additive:**

| # | Decision | Effect on the panels |
|---|---|---|
| G1 | **In V1 (1.3):** C→A `notifications` (snapshot after `welcome`, up to 200 items, read and unread); C→A `notification` (reliable, upsert by `notification_id` with `code`, `severity`, optional `account_id`, `params` as codes and values, `count`, `created_at`, `updated_at`, optional `read_at`); A→C `notification_read` (`notification_ids[]`, `read_at`). Reading in the web arrives as a `notification` with `read_at`. One set, one count | §6 renders from `code` + `params` with the plugin's own pt-BR/en catalog (the plugin translates); read state shared both ways |
| G2 | `welcome.connection_name` (1.3) | `Conexão` tab shows the name; before 1.3 the line is omitted |
| G3 | `welcome.accounts[].display_name` (1.3); a rename applies at the next `welcome` (not in `account_state`) | account cards show it; before 1.3 only `platform_account` |
| G4 | `welcome.time_zone` (IANA, profile) (1.3). Renaming or changing the zone applies at the next `welcome` (reconnection) | §3.4 uses it; before 1.3 the Windows time zone, always labeled |
| G5 | Consistency goes as a **notification** (no meter, Q-P4) | §6 catalog row only |
| G6 | **`welcome.app_url`** (absolute https origin, no path or secret); deep link `<app_url>/<locale>/accounts/<account_id>` | `Abrir no All Win` is a real link (Elo still confirms launching the default browser) |
| G7 | No protocol change: unpairing (or an unmapped account) **stops the local enforcement of that account** (ADR-0004 §7) | `Revoked` state and the web unpair say so; see §4.2 |
| G8 | Build constant (release → production origin, development → local); link `/<locale>/download` on it | `Update required` link |

**Final decisions of the protocol owner (Esquadro) on the Plugin Engineer's preferences, protocol 1.3:**
- **G1 `revision`:** accepted. A monotonic integer per `notification_id`; the plugin applies an upsert only if the revision is greater than the one it holds, so it does not depend on delivery order.
- **G1 `source_event_ids`:** accepted as a **list** (replaces the single id): the `event_id`s of the plugin's A→C messages aggregated in that notification (`count`), at most the 50 most recent. The plugin replaces each local "not yet synced" copy whose `event_id` is on the list.
- **G1 snapshot:** the `notifications` snapshot carries the last 200 items and an **authoritative `unread_count`** that covers everything, including unread items older than the 200.
- **Catalog (accepted in part):** the codes, severities and the schema of each code's `params` are a data file in the contract (`contracts/notifications/v1/registry.json`, the protocol owner's). The pt-BR/en **texts stay in each client's catalog** (web and plugin own their UI copy), with a drift test: every code in the registry has a key in both languages in both clients and no key is orphaned. The fallback "Aviso {code}" is only for codes newer than the plugin.
- **G3 `display_name` in `account_state`:** declined for V1; a rename takes effect at the next `welcome`.
- **G6 `welcome.app_url`:** accepted, replacing the earlier "origin of the Core": an absolute `https` origin with no path or secret; deep links are built on it. The plugin accepts only `https` and logs only the host.
- **G8:** a **build constant** (the 4003 close arrives before `welcome`): release builds point at the production origin, development builds at the local one.
- **G2 / G4:** as in the table (connection name is untrusted display text, bounded and without markup; IANA zones map to Windows ids via the generated CLDR table in `AllWin.Adapter.Core`).

**Out of V1 (protocol owner):** live propagation of a changed name or time zone (next `welcome` only), history beyond 200 items and search in the plugin, a consistency meter, sound, clearing `out_of_sync` in the plugin, editing any configuration from the plugin.

**Correlation (protocol owner):** the 1.3 `notification` carries `source_event_ids` (a list, at most the 50 most recent `event_id`s of the A→C messages aggregated in it); the key is always the adapter's durable `event_id`.


---

## 11. Feasibility (answered by the Plugin Engineer, Elo)

Legend: [OFF] official NinjaScript docs, [INF] WPF/.NET 4.8 inference, [EXP] needs an experiment on the Windows machine (none of this has run in NT8 yet: the Windows VPS does not exist). Every [INF] and [EXP] item becomes a checklist item for QA [Plugins]. **Everything in this spec is feasible, with three adjustments, now applied:** the toast never has keyboard focus (so every toast is mirrored in the window, §7, §9); never `LogLevel.Alert` (§7); the light/dark set is chosen by measured contrast (§3.2).

| # | Topic | Answer |
|---|---|---|
| 1 | Menu and single window | Control Center **New** menu (documented); `NTWindow` + `IWorkspacePersistence`/`WorkspaceOptions` saves and restores position and size; duplicate restore closes itself without activating [EXP]; count in the window title instead of the menu item; `F6` via `KeyBinding` [EXP] (§2.1, §2.2) |
| 2 | Skin | Standard controls inherit the skin; keys only for our own elements, listed on the Windows machine with `SystemColors` fallback [EXP]; set chosen by measured contrast; High Contrast handled in our window only (§3.2, §9) |
| 3 | Numerals | `Tabular` setting plus a Consolas fallback in numeric columns; width test [EXP] (§3.1) |
| 4 | Toast | Plain WPF window, own STA thread, `WS_EX_NOACTIVATE`; no keyboard focus; UIA notification; max 3 stacked; fixed corner in V1 (§7) |
| 5 | Log | `NinjaScript.Log` Information/Warning/Error; never Alert; collapsing and caps (§7); `AlertCallback` is an option for the PO (Q-P7) |
| 6 | UIA | `LiveSetting` and `LiveRegionChanged` exist since .NET 4.7.1; `ProgressBar` already exposes read-only `RangeValue`, so only `Name`, `HelpText` and `ItemStatus` (state) are set [EXP: Narrator/NVDA with Accessibility Insights] |
| 7 | Browser | `Process.Start` with `UseShellExecute` only for an absolute `https` URI whose host equals the Core's app host; the path is built only from validated ids; no token or secret; the user's browser session, the plugin authenticates nothing |
| 8 | DPI and size | Minimum 640 × 440 with vertical scroll: ok; per-monitor DPI depends on NinjaTrader's manifest, not ours [EXP] |
| 9 | Threading | Each NT window has its own `Dispatcher`; the engine publishes immutable snapshots, windows and toast pull the latest via `InvokeAsync(Background)`, coalesced to **2 Hz per window** (one batch with all accounts), never a synchronous `Invoke`; no virtualization in the account list; target to measure: 20 accounts at 2 Hz under 5 % of one core without affecting Chart Trader [EXP] |
| 10 | Pairing field | Plain `TextBox` (not `PasswordBox`), `CharacterCasing=Upper`, undo and spell check off, paste normalized in `AllWin.Adapter.Core` (Linux-tested), cleared on success, revocation and window close, never bound to logged properties |
| 11 | Local record | Own file on the existing journal store (CRC journal + atomic snapshot), separate from the outbox; current and previous trading day, at most 500 `risk_event`s per account, display cache of 200 notices, under about 1 MB; derived, not authoritative; marks-as-read made offline go through the durable outbox; UI writes batched |

**Dependency:** `risk_state`, `risk_event` and `limits`/enforcement do not yet exist in `AllWin.Adapter.Core` (S13/S15 work); the panels depend on them. Today the Core has session, commands and protection.

---

## 12. Decisions and open questions

**Accepted by the PO (Rumo, 2026-10-23):** Q-P3 clearing an out-of-sync flag is web-only in V1; Q-P4 consistency in the plugin is a notification only, no meter; Q-P5 the last 200 items are a **display cache** (the record is the audit log); Q-P6 no sound in V1.

**User questions drawn as recommendation (a), provisional:**

| ID | Question | Spec default (provisional) |
|---|---|---|
| Q65 | The plugin never steals focus | Log + non-activating toast (if feasible) + permanent per-account status in the panel; the window never opens itself |
| Q66 | Unpair in the plugin | Only **disconnects** locally (warning + checkbox when a position is open); the pairing stays in the Core as disconnected with the out-of-sync rules; revoking is in the web with a code (Q18 unchanged) |

**For the PO:** Q-P7 — complement the toast with NinjaTrader's Alerts Log (`AlertCallback`, no sound) for limit reached and block? Default: no, until the [EXP] shows it does not open by itself or take focus.

**For the Plugin Engineer (Elo):** §11 answered; still open: does local enforcement keep running after a user disconnect, does the disconnect survive a NinjaTrader restart, and whether the credential is kept (§4.5).

**Consequence for other specs:** the S3 spec's S3-11 table says the plugin's `Desparear` "forgets the token after confirmation"; under Q66 it becomes `Desconectar` (credential kept). Aquarela updates S3-11 when Q66 is decided.

## 13. AC traceability

| AC | Where |
|---|---|
| S3 AC-4 / S3-11 pair, disconnect (Q66), states, one message for invalid codes | §4 |
| N11 (U15) live accounts opt-in, state always visible | §2.2 header, §4.4 |
| S13 AC-2, AC-3 live risk state, 80 % warning in the plugin | §5.2, §5.4 |
| S13 AC-4 trading-day reset (firm / CME), time zone shown | §5.2 trading-day line, §3.4 |
| S15 AC-1 off by default (shown), AC-2 flatten/cancel/block, AC-3 no early unlock, AC-4 works offline, AC-6 reactive block explained, AC-7 Playback clock | §5.3, §5.5, §5.6, §4.2 |
| S18 AC-1, AC-2 notifications visible in the plugin; mark read; unread count | §2.2, §6 |
| X-3, X-4, X-5, X-6, X-7, X-8 | §1.5, §1.6, §5.3 audit-backed events, §3.4 language, simulation only, time |
