# TradingView alerts for All Win — quick guide

> S17 draft · instruction contract v1 (`schema_version` "1.0") · [Português](GUIDE.pt-BR.md)

## 1. Daily routine

1. Paste the day's levels into the indicator.
2. **Delete yesterday's alert** (an alert keeps the levels and indicator version from when it was created).
3. Create **one alert per chart**: Condition = the indicator → **"Any alert() function call"**.
4. Under *Notifications*, tick **Webhook URL** and paste your All Win source address. **Do not edit the message**: the indicator builds the JSON.
5. Recreate the alert whenever levels, inputs or the indicator version change.

**Do not change the indicator's "Source name" during the trading session.** It is part of every signal's identifier: under a new name, a signal already sent gets a new identifier and **would execute again**. The real risk is two alerts alive at once under different names. If you must change it: outside trading hours, **delete the old alert first**, change the name, and only then create the new alert, with no alert from the old name still active.

**Secret:** the secret token lives **only in the webhook URL**. Never put it in the message, screenshots or chats. If the body contains anything shaped like a token, All Win rejects the alert.

According to [TradingView's official page](https://www.tradingview.com/support/solutions/43000529348-how-to-configure-webhook-alerts/), webhooks may occasionally fail to reach the URL (see the *Webhook status* column in TradingView's alert log). Check All Win's **audit log** to see whether each alert was received or rejected, and why.

## 2. Signal types

| Indicator signal | `signal_type` |
|---|---|
| Triggers (Setups 1/2/3/4/6) | `trigger` |
| Trade Plan | `trade_plan` |
| NQ Triggers | `nq_trigger` |

Each account chooses which types it takes (routing screen).

**When each signal is sent:**

| Signal | Sent |
|---|---|
| Triggers (Setups) by engulfing | at **bar close** (no repainting, up to one bar of delay) |
| NQ Triggers by engulfing | at **bar close** |
| Trade Plan | **intrabar**, on the touch |
| Triggers in touch mode | **intrabar**, on the touch |

**Touch signal risk:** the signal is sent the moment the price touches. If the condition is no longer true at bar close, the chart may no longer show the signal, but the order was already sent and executed; All Win does not undo it.

**Direction:** Bull buys (`buy`), Bear sells (`sell`). The Trade Plan follows its target's direction (target above the price = buy). One signal per bar, setup and direction: the same signal repeated on the same bar counts once.

**The indicator's take and stop alerts never become orders:** stop and target come from each account's profile. The indicator only writes those alerts to Pine Logs and sends nothing to All Win.

**Signal opposite to the indicator's last opening signal:** you choose it in the indicator input **"Opposite signal sends"**: `reverse` (flips the position), `close` (only flattens) or `do not send` (nothing sent; Pine Logs only). The default is `do not send`. The JSON action decides, not All Win. The indicator compares with **its own** last direction **of the same signal type** (triggers, Trade Plan and NQ Triggers each have their own, because each account chooses the types it takes), not with the account's real position: if the account holds the side opposite to a `buy`/`sell`, All Win **refuses** the order and warns you.

Known limits:
- With `do not send`, after an ignored opposite signal the indicator keeps ignoring signals in that new direction until a signal in the old direction comes again.
- When the indicator reloads, the direction is rebuilt from historical bars, where a touch is evaluated at bar close. A touch that fired live and was gone at the close is not part of that rebuild (the same touch-signal risk as above). What executes is still decided by the account's real position in All Win.

**Base quantity:** 1 for every type (before each account's multiplier).

## 3. Fields per action

Always: `schema_version` = `"1.0"`, `message_id` (built by the indicator: `tv-<Source name>-MNQ-<kind>-<setup>-<action[_side]>-<chart ticker>-<timeframe>-<bar time ms>`), `sent_at` (UTC, ends with `Z`), `source.type` = `"tradingview"`, `signal_type`, `action`, `instrument.symbol` = `MNQ` (the **traded future** in V1; `QQQ`/`SPY` are references only).

**Do not send `contract_month`:** All Win uses the catalog's active contract and rolls it on the date the admin configures. A month typed in the Pine goes stale at the roll and would trade the wrong contract.

| Action | Also send | Do not send | Effect on each account |
|---|---|---|---|
| `buy` / `sell` | `quantity` (integer 1–1000, before the account multiplier) | `side`, `order`, `position_effect` | opens/adds; **refused** if the account holds the opposite position |
| `close` | — | `quantity`, `side`, `order` | flattens the **whole** position and cancels stop/target |
| `reverse` | `quantity` and `side` (`long`/`short` = final side) | `position_effect` | flattens and opens the requested side; flat → opens |

**Refused by All Win (V1):** `modify`, `cancel`, `position_effect: "reduce"`, non-market orders (`order.type` limit/stop), `signal_type: "master_account"`, a `schema_version` All Win does not support (today only `1.0`). **Stop and target never go in the message:** they come from each account's profile.

Quantity is **unquoted**: `1`. Do not send `reference_price` while the chart is QQQ/SPY (it would be another instrument's price).

## 4. Valid examples

Buy (trigger):
```json
{"schema_version":"1.0","message_id":"tv-nq-es-strikes-MNQ-trigger-s6-buy-QQQ-5-1792679400000","sent_at":"2026-10-22T14:31:05.123Z","source":{"type":"tradingview","name":"nq-es-strikes","version":"4.0"},"signal_type":"trigger","action":"buy","instrument":{"symbol":"MNQ"},"quantity":1,"details":{"setup":"Setup 6","zone":"Defense","trigger":"engulfing"}}
```
Sell (Trade Plan):
```json
{"schema_version":"1.0","message_id":"tv-nq-es-strikes-MNQ-trade_plan-tp-sell-QQQ-5-1792681200000","sent_at":"2026-10-22T15:00:00.120Z","source":{"type":"tradingview","name":"nq-es-strikes","version":"4.0"},"signal_type":"trade_plan","action":"sell","instrument":{"symbol":"MNQ"},"quantity":1}
```
Sell (NQ Trigger):
```json
{"schema_version":"1.0","message_id":"tv-nq-es-strikes-MNQ-nq_trigger-nq-sell-NQ1-1-1792682000000","sent_at":"2026-10-22T15:13:20.004Z","source":{"type":"tradingview","name":"nq-es-strikes","version":"4.0"},"signal_type":"nq_trigger","action":"sell","instrument":{"symbol":"MNQ"},"quantity":1}
```
Flatten:
```json
{"schema_version":"1.0","message_id":"tv-nq-es-strikes-MNQ-trigger-s6-close-QQQ-5-1792683000000","sent_at":"2026-10-22T15:30:00.000Z","source":{"type":"tradingview","name":"nq-es-strikes","version":"4.0"},"signal_type":"trigger","action":"close","instrument":{"symbol":"MNQ"}}
```
Reverse to short:
```json
{"schema_version":"1.0","message_id":"tv-nq-es-strikes-MNQ-trigger-s3-reverse_short-QQQ-5-1792684000000","sent_at":"2026-10-22T15:46:40.250Z","source":{"type":"tradingview","name":"nq-es-strikes","version":"4.0"},"signal_type":"trigger","action":"reverse","side":"short","instrument":{"symbol":"MNQ"},"quantity":1}
```
The same messages are in `examples/accepted/`.

## 5. Validating an alert

- **In normal use:** open All Win's **audit log**. Each alert shows as received, duplicate ignored or rejected, with the reason.
- **During development:** the indicator writes every message to **Pine Logs**. Paste **only the JSON message, never the URL**, into the conversation with the team; the team tests it on the local development stack (nothing exposed) and answers accepted / duplicate / rejected + code. Re-sending the same JSON gives "duplicate": that is expected.
