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
3. At each place where the indicator detects a signal, call `aw_send(...)` (see *Usage* at the end of the block). Remove the old `alert()` calls for those signals.
4. Save, add to the chart, then create **one alert per chart** with "Any alert() function call" and the All Win webhook URL (guide §1).

### Inputs (group "All Win")

| Input | Default | Meaning |
|---|---|---|
| Source name | `nq-es-strikes` | `source.name`, shown in the audit log and part of every `message_id`: 1–24 characters `A-Z a-z 0-9 . _ -`. Do not change it during the trading day (an event re-fired under a new name gets a new id) |
| Indicator version | `4.0` | `source.version` (1–32 characters) |
| Quantity: triggers / Trade Plan / NQ Triggers | 1 | Base quantity per signal type (1–1000), before each account's multiplier |
| Send only at bar close | on | On: send on the bar's last tick (no repainting, up to one bar of delay). Off: send intrabar as soon as the condition is true |
| Send alerts | on | Off: messages go only to Pine Logs; nothing is sent |

Fixed in code: `instrument.symbol` = `MNQ` (V1); no `contract_month` and no `reference_price` are sent; `schema_version` = `1.0`; market orders only.

### `aw_send(kind, action, side, setup, qty, details)`

| Argument | Values |
|---|---|
| `kind` | `"trigger"`, `"trade_plan"`, `"nq_trigger"` |
| `action` | `"buy"`, `"sell"`, `"close"`, `"reverse"` |
| `side` | `"long"`/`"short"` for `reverse` (final side), `""` otherwise |
| `setup` | Short code that identifies the signal (`"s6"`, `"tp"`, `"nq"`); part of `message_id`; 1–16 characters `A-Z a-z 0-9 . _`, anything else is refused (never stripped) |
| `qty` | Base quantity; ignored for `close` |
| `details` | `""` or pairs from `aw_kv(key, value)` joined with `","`; hard limit 32 pairs; display only. Values must be deterministic for the event (setup, zone, trigger names; no `timenow`, live price or counters): a re-fire with different details is reported by the Core as a duplicate conflict |

Returns the `message_id` sent, or `""`.

Behaviour:
- `message_id` = `tv-<source name>-MNQ-<kind>-<setup>-<action[_side]>-<chart ticker>-<timeframe>-<bar open time ms>` (unique per user across all sources): the same event always has the same id, so the Core answers `duplicate` instead of executing twice (alert restarted, two alert instances on the chart). Two identical signals on the same bar and chart are one signal.
- `sent_at` = `timenow` in UTC with milliseconds (emission time; the Core's expiry is based on it).
- Sends only on realtime bars. Each id is sent once per script run (`varip` guard), with `alert.freq_all` so that distinct signals on the same bar are never swallowed.
- Text values are JSON-escaped (backslash, double quote, newline).
- Every message is written to Pine Logs (`log.info`). Invalid arguments (kind, action, side, setup, more than 32 details), an id over 128 characters, any other control character in the body, or a token-shaped string in the body are logged with `log.error` and **nothing is sent** (fail safe).
- Invalid inputs stop the indicator at load with a visible error.

### Pine Editor checklist (not verified outside TradingView)

The block has not been compiled here. On the first paste, confirm in the Pine Editor, with "Send alerts" off:

1. It compiles in v6 with no error, including: the global `varip array<string>` changed inside `aw_send`; `alert()` and `log.info()` called inside a user function.
2. `str.match(j, "[\\x00-\\x1F]")` is accepted as written. If Pine rejects the `\x` escape, replace it with explicit checks for a tab and a carriage return.
3. A forced test signal writes one line to Pine Logs whose `message_id` ends with a plain bar time (digits only, no thousands separator) and whose `sent_at` looks like `2026-10-22T14:31:05.123Z`.
4. That JSON, pasted into the conversation with the team (never the URL), is accepted by the development stack.
