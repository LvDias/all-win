# Fake Core scenario harness

`AllWin.FakeCore` plays the Core side of adapter protocol v1. It is the mirror of `core/tools/adapter-sim` (spike plan §5 item 3, profile §13.3). Results are asserted from the `execution_report` / `position_snapshot` stream, so no screen is needed.

- It listens on **loopback only** (`127.0.0.1`). No firewall port is opened.
- It validates every frame in both directions against `contracts/adapter-protocol/v1`. It fails a scenario on any protocol violation by the adapter.
- It handles the handshake (`welcome` + `limits`), acks every reliable adapter message (unless told not to), and keeps an outbox. Its retransmissions are byte-identical, and on reconnect it sends its own unacked messages again after the adapter's snapshot.

## Run on Linux (adapter + simulated platform in-process)

```bash
dotnet run --project plugins/ninjatrader8/tools/AllWin.FakeCore -- -with-sim            # all scenarios
dotnet run --project plugins/ninjatrader8/tools/AllWin.FakeCore -- -with-sim -v -scenario plugins/ninjatrader8/tools/AllWin.FakeCore/scenarios/08-bracket-integrity.json
```

The same scenarios run in `dotnet test` (`FakeCoreScenarioTests`).

## Run against NinjaTrader 8 (Windows test VPS, Sim101 + Simulated Data Feed)

This is not yet possible: the plugin host (`AllWin.NinjaTrader8`) is the next step. It will connect to `ws://127.0.0.1:8765/adapter/v1` in development builds.

1. On the VPS, as the `allwin-agent` user, start NT8 with the Simulated Data Feed connected. Only Sim101 is used.
2. Start the fake Core:

   ```
   dotnet run --project plugins\ninjatrader8\tools\AllWin.FakeCore -- -port 8765 -token-file %USERPROFILE%\.allwin-fakecore-token -transcript fakecore.log
   ```

3. Configure the plugin with the same test token file. The harness prints the key id only, never the token.
4. Scenarios without `"requires_sim": true` run against NT8: entry-bracket, disconnect-reconnect, expiry, flatten-block, duplicates, stale, partial-exit. The others need price control and run with `-with-sim`. E11 covers the NT8 bracket-integrity case manually.

## Scenario format

```json
{ "name": "…", "description": "…", "requires_sim": false,
  "accounts": [{ "alias": "A", "platform_account": "Sim101" }],
  "sim": { "tick_size": { "MNQ": "0.25" }, "price": { "MNQ": "21000" } },
  "steps": [ { "do": "…" } ] }
```

**Account ids.** They are stable per platform account across runs, so a real plugin keeps its mapping.

**Instrument.** The default is MNQ 2026-12. Override it with `"instrument"` at the scenario or step level.

### Steps (`do`)

| Step | Fields | Meaning |
|---|---|---|
| `wait_ready` | `timeout_ms` | Waits for the adapter's reconnect snapshot |
| `expect_session` | `count` | Waits for session #n to be ready, after a reconnect |
| `command` | `id`, `kind`, `place`/`modify`/`cancel`/`flatten`, `depends_on` (alias), `position_ref` (alias), `ttl_ms`, `sent_at_offset_ms`, `msg` (alias of the msg_id) | Sends a command in the negotiated protocol version. Ids are generated. Entries get `expires_at = now + ttl_ms` (default 5000). `targets[].command` takes an alias. A reduce needs `position_ref`: the alias of the command that opened the position (protocol 1.2, U13) |
| `repeat_command` | `id` | Same `command_id`, new `msg_id` (business duplicate) |
| `resend` | `msg` | The same frame, byte-identical (transport duplicate) |
| `expect_report` | `command` or `external: true`, `match` (subset of the payload), `count`, `exact`, `timeout_ms` | Waits for matching execution reports. Duplicates by `msg_id` are ignored |
| `expect_no_report` | `command`, `match`, `within_ms` | No matching report after the wait |
| `expect_snapshot` | `trigger`, `positions` `{alias:{symbol:qty}}`, `connected`, `working_orders` `{alias:n}`, `session` (`current` or `any`) | Waits for a matching snapshot |
| `expect_ack` | `msg` | The adapter acked our message |
| `limits` | `version`, `blocked_for_ms`, `enforcement_enabled`, `msg` | Sends `limits` |
| `close` | `code` | The Core closes the session (e.g. 4000) |
| `drop_acks` | `value` | Stops or resumes acking adapter messages |
| `sleep` | `ms` | |
| `sim` | `set_price`, `oco`, `reject_next`, `market_fill_slices`, `connected` `{alias:bool}`, `reset` (alias), `external_fill`, `stop_adapter`, `start_adapter` | Simulator controls; needs `-with-sim`. `stop_adapter` + `start_adapter` model an NT8 crash and restart: events in between are lost and order handles change |
