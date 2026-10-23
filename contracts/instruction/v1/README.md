# Instruction contract v1

Canonical standard instruction accepted by the Core from every Input adapter. Decision record: [ADR-0003](../../../docs/architecture/adr/0003-instruction-contract-v1.md). Owner: Software Architect (changes via versioned delta, see `../CHANGELOG.md`).

- `instruction.schema.json`: JSON Schema draft 2020-12. Structural rules only; source policies are in ADR-0003 §6.
- `examples/valid/*.json`: every implementation must accept these and round-trip them without losing decimal precision.
- `examples/invalid/*.json`: every implementation must reject these. The file name states the reason.

Conformance rule (ADR-0002 §4): Rust (`core/crates/contracts`), C# (`plugins/ninjatrader8/src/AllWin.Adapter.Core`) and the generated TypeScript types are tested against these fixtures in CI. Decimals are JSON strings; quantities are integers; credentials never appear in the body (the TradingView token is in the URL path).

## Duplicate comparison (ingress, HTTP 5.4.0)

A repeated `message_id` is compared with its first receipt over this **closed list** of fields: `source`, `signal_type`, `action`, `position_effect`, `side`, `instrument`, `quantity`, `order`, `changes`, `refs`. The Core keeps the sha256 of the canonical parsed JSON of those fields only (absent ≠ null), and only while the content is kept. The purge (Q13-b) clears it in the same batch. After the purge, a repeat is a plain duplicate (`ingress.duplicate_ignored` with `content_not_retained: true`), never a conflict.

- Same fields → normal duplicate, 200.
- A different field → 200 and nothing executes; audited and notified as `ingress.duplicate_conflict` with the names of the differing fields.

`message_id`, `sent_at`, `reference_price`, `details` and `schema_version` are not compared. Expiry and everything else come from the first receipt. A new instruction field joins the list only by an explicit contract change (Esquadro, 2026-10-24).
