# plugins/ninjatrader8 — rules for agents

Owner: Plugin Engineer. The repository rules in [/CLAUDE.md](../../CLAUDE.md) apply. This file adds the component rules.

1. **Simulation only.** Test with `SimulatedPlatform`, NT8 Sim101/Simulated Data Feed or Playback. Never a live account.
   - The engine's guard (`AdapterEngine.IsExecutionAllowed` + `EnsureWritable`) is checked on **every** platform write. Never bypass it.
   - Development builds stay simulation-only whatever the Core says.
   - `AdapterBuild.Release` may only come from a compile-time constant of the release build (net48 host slice), never from configuration or UI.
   - Live accounts (release builds only, ADR-0005 N11): both gates (`welcome.simulation_only = false` and the local opt-in `ILiveAccountConsent`) or the account is outside state B. Every platform write names its `LiveRow` and passes `EnsureWritable`; `LiveAccountTable` is the only place the table lives. U19/U19.1 are `EngineOptions` set by build/server configuration, never a UI toggle; missing or invalid = the recommended option. Only orders with an `aw:` Name are ever touched on the plugin's own initiative.
   - Release builds connect only to `wss://`; `ws://` on loopback is a Development-only harness path.
2. **Logic lives in `AllWin.Adapter.Core`.**
   - No NinjaTrader references there.
   - C# ≤ 12, no `record`/`init` (NT8 compiles the hot-loop source itself; ADR-0005 N9).
   - Newtonsoft.Json 13 is compile-time only (`PrivateAssets=all`; never shipped, ADR-0005 N3).
3. **Exact decimals.** Prices and money are `decimal` and strings on the wire. NT `double` prices become decimals only through `TickMath.FromPlatformDouble`.
4. **Write-ahead.**
   - Persist the intent (`Save` + `Flush`) before any platform call that could execute something.
   - A command that reached the platform is never submitted again, even after a crash.
   - Every engine input runs on one thread (`EngineLoop`).
5. **Contracts first.**
   - `contracts/**` belongs to the Architect.
   - After a contract change, run `tools/gen-contract-schemas.py`. The drift test enforces it.
   - Every frame is validated against the schema both ways. An invalid outbound frame throws.
   - Reason codes: `Protocol/ReasonCodes.cs`, mirrored from `contracts/adapter-protocol/v1/reason-codes.md`.
   - A `reduce` runs only through the 1.2 position-identity guard (ADR-0004 §5). Never strip or invent `refs`, and never execute a reduce from a pre-1.2 envelope.
6. **No secrets in logs.**
   - The device token is printed only as `TokenMask.Mask` (key_id).
   - Log ids and codes, never payload bodies.
7. **UI text.**
   - Every user-visible string goes into `Localization/AdapterTexts.cs` in en and pt-BR.
   - Parity tests enforce it.
8. **Tests.**
   - Run `dotnet test plugins/ninjatrader8 --nologo -v q` before handing over.
   - New behavior needs an engine test (`tests/.../Engine`). A harness scenario is needed when the behavior crosses the transport.
