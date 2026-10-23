# NinjaTrader 8 plugin

Owner: Plugin Engineer. Rules: [CLAUDE.md](CLAUDE.md). Design: [docs/adapter-core-design.md](docs/adapter-core-design.md).

The plugin connects a trader's NinjaTrader 8 to the All Win Core through adapter protocol v1 ([ADR-0004](../../docs/architecture/adr/0004-adapter-protocol-v1.md)). It executes Core commands on mapped simulation accounts and reports every execution. It also keeps brackets and orphan protection alive locally. It implements protocol versions 1.0–1.2; 1.2 adds the position-identity guard on `reduce` (U13). Release builds act on a live account only with the local "Allow live accounts" opt-in and the decision table of [ADR-0005 §N11](../../docs/architecture/adr/0005-nt8-plugin-implementation.md).

## Layout

| Path | What | Targets |
|---|---|---|
| `src/AllWin.Adapter.Core/` | All plugin logic: protocol v1 client state machine, durable store, brackets, orphan protection, simulation guard, WebSocket transport, pt-BR/en texts. No NinjaTrader references | `netstandard2.0;net48` (ADR-0005 N2) |
| `src/AllWin.Adapter.Sim/` | Deterministic simulated broker implementing `ITradingPlatform`. Used for tests and the harness, never deployed | `netstandard2.0` |
| `tools/AllWin.FakeCore/` | Scripted fake Core with scenario files (spike plan §5 item 3). With `-with-sim`, it also hosts the real adapter against the simulator | `net10.0` |
| `tests/AllWin.Adapter.Core.Tests/` | Unit tests, contract conformance (`contracts/adapter-protocol/v1` and `contracts/instruction/v1` fixtures), end-to-end scenarios | `net10.0` |
| *(not yet)* `src/AllWin.NinjaTrader8/` | `net48` AddOn host: NT8 API mapping and UI only. It needs the NT8 DLLs and the Windows machine | `net48` |

## Build and test (Linux, .NET 10 SDK)

```bash
dotnet test plugins/ninjatrader8 --nologo -v q            # 676 tests, includes the fake-Core scenarios
dotnet run --project plugins/ninjatrader8/tools/AllWin.FakeCore -- -with-sim   # scenarios with a readable summary
python3 plugins/ninjatrader8/tools/gen-contract-schemas.py # after a contract change (a drift test fails otherwise)
```

**Dependencies.** Restores are pinned by `packages.lock.json`, one per project, committed. After changing a package reference, run `dotnet restore plugins/ninjatrader8 --force-evaluate` and commit the updated lock files. CI sets `CI=true`, which turns on `RestoreLockedMode` and stops tolerating NU1900, and runs `dotnet restore --locked-mode`. NuGet audit covers transitive packages (`NuGetAuditMode=all`).

## Status (U13 reduce guard + N11 live-account table, 2026-10-23)

- **Verified on Linux:**
  - `AllWin.Adapter.Core` builds for `netstandard2.0` and `net48`, with warnings as errors.
  - All protocol fixtures pass: our in-process validator and an independent JSON Schema implementation agree on every fixture.
  - Engine behavior is unit-tested, including the position-identity guard and one test per cell of the §N11 table in each of the four U19 × U19.1 combinations.
  - The 12 harness scenarios pass over a real loopback WebSocket against the simulated platform, plus a half-open connection test.
- **Not verified (needs the Windows VPS, which does not exist yet):**
  - Anything inside NinjaTrader 8: the AddOn host, Account API mapping, event ordering, the `Order.Name` round-trip after a restart (E4), and the provider names used by the simulation guard.
  - Durable store behavior on NTFS and under NT8 (E12).
  - DPAPI token storage, and the live-account opt-in: the confirmation dialog, its DPAPI-protected file and its ACL (ADR-0005 N11).
  - NT8 execution retention and ordering across a restart, which the identity record relies on (E4/E9).
  - Packaging (E3).
  - See [docs/adapter-core-design.md §8](docs/adapter-core-design.md#8-open-items-for-the-nt8-spike-windows).
