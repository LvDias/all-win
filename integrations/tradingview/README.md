# integrations/tradingview

TradingView alert integration for All Win (slice V1-S17). Owner: Plugin Engineer.

| Path | Content |
|---|---|
| [GUIDE.pt-BR.md](GUIDE.pt-BR.md), [GUIDE.en.md](GUIDE.en.md) | User guide: daily routine, fields per action, valid examples |
| [ANALYSIS.md](ANALYSIS.md) | Contract mapping, `message_id` design, validation options, open questions |
| [pine/allwin_alert.pine](pine/allwin_alert.pine) | Pine v6 block that builds the instruction JSON and calls `alert()` |
| `examples/accepted/` | Messages the Core accepts (schema + TradingView source policy) |
| `examples/rejected/` | `400-*`: fail the schema; `422-*`: schema-valid, refused by the TradingView source policy |

The Pine adapts to the instruction contract ([ADR-0003](../../docs/architecture/adr/0003-instruction-contract-v1.md), [`contracts/instruction/v1`](../../contracts/instruction/v1/)), never the other way round.

## Pine block

### Install

1. Open the indicator in the Pine Editor (Pine Script v6).
2. Paste the whole content of `pine/allwin_alert.pine` right after the `indicator(...)` line.
3. At each place where the indicator detects an opening signal, call `aw_open(...)`; at each take/stop alert, call `aw_note(...)` (see *Usage* at the end of the block). Remove the old `alert()` calls for those signals.
4. Save, add to the chart, then create **one alert per chart** with "Any alert() function call" and the All Win webhook URL (guide §1).

### Inputs (group "All Win")

| Input | Default | Meaning |
|---|---|---|
| Source name | `nq-es-strikes` | `source.name`, shown in the audit log and part of every `message_id`: 1–24 characters `A-Z a-z 0-9 . _ -`. Do not change it during the trading day (an event re-fired under a new name gets a new id) |
| Indicator version | `4.0` | `source.version` (1–32 characters) |
| Quantity: triggers / Trade Plan / NQ Triggers | 1 | Base quantity per signal type (1–1000), before each account's multiplier. User decision S17-d: 1 for every type |
| Opposite signal sends | `do not send` | What a signal opposite to the indicator's last opening direction sends (user decision S17-c: the JSON action decides, not All Win): `reverse` (flips), `close` (only flattens) or `do not send` (Pine Logs only) |
| Send alerts | on | Off: messages go only to Pine Logs; nothing is sent |

Fixed in code: `instrument.symbol` = `MNQ` (V1); no `contract_month` and no `reference_price` are sent; `schema_version` = `1.0`; market orders only.


### When each signal is sent (user decision S17-a)

| Signal | `timing` | Sent |
|---|---|---|
| Triggers (Setups) by engulfing | `"close"` | on the bar's last tick: no repainting, up to one bar of delay |
| NQ Triggers by engulfing | `"close"` | same |
| Trade Plan | `"touch"` | intrabar, as soon as the condition is true |
| Triggers in touch mode | `"touch"` | same |

**Vanish risk (touch):** an intrabar signal is sent the moment the price touches. If the condition is no longer true at bar close, the chart may no longer show the signal, but the order was already sent and executed; All Win does not undo it. Use touch only for the signals the user chose (Trade Plan, touch-mode triggers).

### `aw_open(kind, timing, dir, setup, details)` (opening signals)

| Argument | Values |
|---|---|
| `kind`, `timing`, `setup`, `details` | as for `aw_send` below |
| `dir` | `"long"` (Bull: buys) or `"short"` (Bear: sells). Trade Plan: the direction of its target (target above the price = `"long"`) |

Uses the kind's base quantity. Flat or same direction → `buy`/`sell` (same direction adds). Opposite to the indicator's last opening direction → the *Opposite signal sends* input: `reverse` (to `dir`), `close`, or nothing. Returns the `message_id` sent, or `""`.

"Opposite" is judged from the **indicator's own** last opening direction **of the same signal type** (`aw_dir`: one entry each for triggers, Trade Plan and NQ Triggers, because accounts subscribe per signal type), rebuilt from the historical bars after a reload; it is not the account's position. All Win always decides with the account's real position (ADR-0003, no contract change): a `buy`/`sell` that conflicts with it is refused and the user is warned (`routing.position_conflict`), `reverse` flips or opens, `close` flattens. Each opening event (kind, setup, direction, bar) is decided once, so a touch signal re-evaluated on later ticks of the same bar cannot turn a `reverse` into an extra `buy`.

Known limits (documented, not fixed):
- With `do not send`, an ignored flip leaves the type's direction unchanged, so every later signal in the new direction is also treated as opposite and ignored, until a signal in the old direction comes again.
- After a reload, the direction is rebuilt from historical bars, where touch signals are evaluated at bar close. A touch that fired live and was gone at the close is missing from the rebuild, so the rebuilt direction can differ from what was sent. This is the same vanish risk as above; the account's real position in All Win still decides what executes.

### `aw_note(what, text)` (take/stop alerts)

The indicator's own take and stop alerts **never become orders**: stop and target come from each account's profile (S17-c). `aw_note` writes them to Pine Logs only and sends nothing to All Win.

### `aw_send(kind, timing, action, side, setup, qty, details)` (low level)

| Argument | Values |
|---|---|
| `kind` | `"trigger"`, `"trade_plan"`, `"nq_trigger"` |
| `timing` | `"close"` or `"touch"` (table above) |
| `action` | `"buy"`, `"sell"`, `"close"`, `"reverse"` |
| `side` | `"long"`/`"short"` for `reverse` (final side), `""` otherwise |
| `setup` | Short code that identifies the signal (`"s6"`, `"tp"`, `"nq"`); part of `message_id`; 1–16 characters `A-Z a-z 0-9 . _`, anything else is refused (never stripped) |
| `qty` | Base quantity; ignored for `close` |
| `details` | `""` or pairs from `aw_kv(key, value)` joined with `","`; hard limit 32 pairs; display only. Good practice: values deterministic for the event (setup, zone, trigger names; no `timenow`, live price or counters). `details` is not part of the duplicate comparison, so different details on a re-fire never raise a conflict |

Returns the `message_id` sent, or `""`.

Behaviour:
- `message_id` = `tv-<source name>-MNQ-<kind>-<setup>-<action[_side]>-<chart ticker>-<timeframe>-<bar open time ms>` (unique per user across all sources): the same event always has the same id, so the Core answers `duplicate` instead of executing twice (alert restarted, two alert instances on the chart). One signal per bar, setup and direction (user decision S17-b): two identical signals on the same bar and chart are one signal, also when a touch condition flickers within the bar. A touch-mode trigger and an engulfing trigger of the same setup need distinct setup codes (e.g. `s3t` and `s3`) to stay two signals.
- `sent_at` = `timenow` in UTC with milliseconds (emission time; the Core's expiry is based on it).
- Sends only on realtime bars (`"close"`: only on the confirmed bar). Each id is sent once per script run (`varip` guard), with `alert.freq_all` so that distinct signals on the same bar are never swallowed.
- Text values are JSON-escaped (backslash, double quote, newline).
- Every message is written to Pine Logs (`log.info`). Invalid arguments (kind, timing, action, side, `dir`, setup, more than 32 details), an id over 128 characters, any other control character in the body, or a token-shaped string in the body are logged with `log.error` and **nothing is sent** (fail safe).
- Invalid inputs stop the indicator at load with a visible error.

### Pine Editor checklist (not verified outside TradingView)

The block has not been compiled here. On the first paste, confirm in the Pine Editor, with "Send alerts" off:

1. It compiles in v6 with no error, including: the global `varip` arrays changed inside `aw_send`/`aw_open`; `alert()` and `log.info()` called inside a user function.
2. `str.match(j, "[\\x00-\\x1F]")` is accepted as written. If Pine rejects the `\x` escape, replace it with explicit checks for a tab and a carriage return.
3. A forced test signal writes one line to Pine Logs whose `message_id` ends with a plain bar time (digits only, no thousands separator) and whose `sent_at` looks like `2026-10-22T14:31:05.123Z`.
4. That JSON, pasted into the conversation with the team (never the URL), is accepted by the development stack.
5. Opposite signal: with a long trigger earlier in the day on the chart, a short trigger logs `reverse`, `close` or "not sent" per the input; after removing and re-adding the indicator, the same happens (the direction is rebuilt from history). A Trade Plan or NQ Trigger signal is judged on its own type only.
