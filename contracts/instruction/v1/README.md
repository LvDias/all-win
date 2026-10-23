# Instruction contract v1

Canonical standard instruction accepted by the Core from every Input adapter. Decision record: [ADR-0003](../../../docs/architecture/adr/0003-instruction-contract-v1.md). Owner: Software Architect (changes via versioned delta, see `../CHANGELOG.md`).

- `instruction.schema.json`: JSON Schema draft 2020-12. Structural rules only; source policies are in ADR-0003 §6.
- `examples/valid/*.json`: every implementation must accept these and round-trip them without losing decimal precision.
- `examples/invalid/*.json`: every implementation must reject these. The file name states the reason.

Conformance rule (ADR-0002 §4): Rust (`core/crates/contracts`), C# (`plugins/ninjatrader8/src/AllWin.Adapter.Core`) and the generated TypeScript types are tested against these fixtures in CI. Decimals are JSON strings; quantities are integers; credentials never appear in the body (the TradingView token is in the URL path).
