# Adapter protocol v1 (Core ↔ connected-client adapters)

WebSocket protocol between the Core and adapters that run inside trading platforms (first: NinjaTrader 8 AddOn). Decision record and normative semantics (delivery, ordering, expiry, reconnect, out-of-sync, pairing): [ADR-0004](../../../docs/architecture/adr/0004-adapter-protocol-v1.md). Owner: Software Architect.

- `messages.schema.json`: one JSON object per text frame; envelope + payload by `type`. The `instruction` payload embeds `../../instruction/v1/instruction.schema.json` by `$id`, so validators must load both schemas.
- `reason-codes.md`: registry of adapter-emitted reason codes (status, terminal, `depends_on` success) and report attribution rules.
- `examples/valid/*.json`, `examples/invalid/*.json`: conformance fixtures for Rust (`core/crates/contracts`, `core/tools/adapter-sim`) and C# (`AllWin.Adapter.Core`). Invalid file names state the reason.
- Self-check: `python3 contracts/scripts/validate_fixtures.py contracts -v`.

Decimals are strings; quantities are integers; timestamps are UTC with `Z`; ids are lowercase UUIDs. Tokens never appear in messages (the device token is only in the upgrade request header).
