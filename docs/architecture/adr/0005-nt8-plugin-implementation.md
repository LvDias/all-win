# ADR-0005 — NinjaTrader 8 plugin implementation decisions

- **Status:** Proposed (2026-10-23). These are technical decisions inside ADR-0001 §7 and ADR-0004. They are confirmed or adjusted by the NT8 spike report before V1-S2.
- **Owner:** Software Architect
- **Inputs:** [NT8 platform profile](../platforms/ninjatrader8-profile.md) (Trilha, 2026-10-23), §17 item 1; [ADR-0001 §7](0001-technology-stack.md); [ADR-0004](0004-adapter-protocol-v1.md).

## Decisions

| # | Topic | Decision | Why / profile ref |
|---|---|---|---|
| N1 | Brackets | **Account-API brackets** (stop-market + limit sharing an OCO id, submitted after the entry fill, resized per partial fill). **Not ATM strategies** | Distances come from the Core per account; individual/shared mode, offset moves, capping and orphan cleanup live in one place (`AllWin.Adapter.Core`). ATM templates live on the Windows machine and cannot run on Playback with server-side ATMs (§7) |
| N2 | Core library targets | `AllWin.Adapter.Core` **multi-targets `netstandard2.0;net48`**. NT8 loads the `net48` build; Linux CI tests the `netstandard2.0` build (and can build `net48` with `Microsoft.NETFramework.ReferenceAssemblies`) | Avoids the `netstandard.dll` facade issue inside NT8 (§4). **Refines ADR-0001 §7**, which named `netstandard2.0` only; the boundary (no NinjaTrader references, Linux-testable) is unchanged |
| N3 | JSON | Use **NT8's own Newtonsoft.Json 13**: compile-time reference to Newtonsoft.Json 13.0.x with the package excluded from our output (`PrivateAssets=all`, not copied to `bin\Custom`, not in the zip). Decimal fields are (de)serialized as strings by our converters; `FloatParseHandling.Decimal` is mandatory | NT8 ships Newtonsoft 13; shipping another copy collides with NT or other vendors (§4, §16). All 13.x builds share assembly version 13.0.0.0 |
| N4 | Simulation guard | Non-release builds act only on accounts where `Connection.Options.Mode == Mode.Simulation` **and** the provider is local simulation, Simulated Data Feed or Playback. Anything else, including errors reading these properties, means **refuse** (fail closed). Account names are never trusted. Release builds report the account `kind` in `hello` and obey `welcome.simulation_only` | §6, §14. Prop evaluation accounts can report `Mode.Live` |
| N5 | Order tagging | Every routed order uses **`OrderEntry.Automated`** (CME tag 1028) and `Name = "aw:" + command_id + ":" + role`, where role is `e` (entry), `s` (stop), `t` (target) or `x` (exit/flatten). 41 characters, within the 50-character limit | §6. The `Name` is the reconciliation key after a restart; `Order.OrderId` changes over time |
| N6 | Threading | NT event handlers copy event args into a single-consumer queue and return. All state changes happen on our worker. WebSocket I/O runs on its own task with serialized sends. UI updates use `Dispatcher.InvokeAsync` only; `Dispatcher.Invoke` is forbidden. Shared NT collections are locked as documented | §5, §16 |
| N7 | Positions and fills | Flat is detected by `PositionUpdate` with `Operation.Remove`, which triggers orphan cancellation. `ExecutionUpdate` amend/remove maps to `fill.operation`. `SimulationAccountReset` maps to a snapshot with `trigger: account_reset` | §6; ADR-0004 §6 |
| N8 | OCO location | `oco_location` is derived from the connection provider: NinjaTrader and Rithmic → `local`; documented server-side providers → `server`; otherwise `unknown` | §7.3; ADR-0004 §6.1 |
| N9 | Development loop | Host code is deployed as **source** under `bin\Custom\AddOns\AllWin\` for the hot loop (no restart). The packaged DLL + zip is tested at milestones and before every release | §13.2 C4; spike E2/E3 |
| N10 | Packaging | NinjaScript Archive (.zip) with our DLLs and `AdditionalReferences.txt`. No `.cs` files in releases, no Newtonsoft, `AllWin` prefix on every type and file. The minimum supported NT8 version is pinned and checked at load | §10 |

## Consequences

- One implementation of bracket logic, testable on Linux.
- A plugin package free of dependency conflicts.
- A guard that cannot be bypassed by account naming.
- The spike must verify N2 (E3), N5 (E4: `Name` survives a restart) and N9 (E2). If any fails, the Architect issues a delta.
