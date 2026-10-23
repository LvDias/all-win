# ADR-0005 — NinjaTrader 8 plugin implementation decisions

- **Status:** Proposed (2026-10-23). These are technical decisions inside ADR-0001 §7 and ADR-0004. They are confirmed or adjusted by the NT8 spike report before V1-S2. N11 (U15) and its decision table (U19 a, U19.1 b, and the accepted residual risk of a failed flatten) are user decisions of 2026-10-23.
- **Owner:** Software Architect
- **Inputs:** [NT8 platform profile](../platforms/ninjatrader8-profile.md) (Trilha, 2026-10-23), §17 item 1; [ADR-0001 §7](0001-technology-stack.md); [ADR-0004](0004-adapter-protocol-v1.md).

## Decisions

| # | Topic | Decision | Why / profile ref |
|---|---|---|---|
| N1 | Brackets | **Account-API brackets** (stop-market + limit sharing an OCO id, submitted after the entry fill, resized per partial fill). **Not ATM strategies** | Distances come from the Core per account; individual/shared mode, offset moves, capping and orphan cleanup live in one place (`AllWin.Adapter.Core`). ATM templates live on the Windows machine and cannot run on Playback with server-side ATMs (§7) |
| N2 | Core library targets | `AllWin.Adapter.Core` **multi-targets `netstandard2.0;net48`**. NT8 loads the `net48` build; Linux CI tests the `netstandard2.0` build (and can build `net48` with `Microsoft.NETFramework.ReferenceAssemblies`) | Avoids the `netstandard.dll` facade issue inside NT8 (§4). **Refines ADR-0001 §7**, which named `netstandard2.0` only; the boundary (no NinjaTrader references, Linux-testable) is unchanged |
| N3 | JSON | Use **NT8's own Newtonsoft.Json 13**: compile-time reference to Newtonsoft.Json 13.0.x with the package excluded from our output (`PrivateAssets=all`, not copied to `bin\Custom`, not in the zip). Decimal fields are (de)serialized as strings by our converters; `FloatParseHandling.Decimal` is mandatory | NT8 ships Newtonsoft 13; shipping another copy collides with NT or other vendors (§4, §16). All 13.x builds share assembly version 13.0.0.0 |
| N4 | Simulation guard | Non-release builds act only on accounts where `Connection.Options.Mode == Mode.Simulation` **and** the provider is local simulation, Simulated Data Feed or Playback. Anything else, including errors reading these properties, means **refuse** (fail closed). Account names are never trusted. Release builds report the account `kind` in `hello`, obey `welcome.simulation_only`, **and** need the local opt-in of N11 for any non-simulation account | §6, §14. Prop evaluation accounts can report `Mode.Live` |
| N11 | Local live-account opt-in (U15, user decision 2026-10-23) | **Release builds only.** Executing on an account whose `kind` is `live` needs **both** gates: the Core's `welcome.simulation_only = false` **and** the plugin's local setting **"Permitir contas reais" / "Allow live accounts"**.<br>• **Default off**, on install and when the setting is missing, unreadable or invalid (fail closed). Kept across plugin upgrades.<br>• **Who changes it:** only the user, in the plugin window, through a confirmation dialog that names the connected live accounts. Never the Core, a message, a file shipped in the package, or an agent.<br>• **Scope (Vigia L1):** the opt-in covers exactly the live accounts confirmed in the dialog, stored as a set of (connection, provider, `platform_account`) keys. A confirmation is invalidated when the account reports a **known** `kind` other than `live` (simulation or playback); a transient `unknown` never invalidates it (§N11, Vigia I1/L1). The account then becomes state A of the §N11 table: All Win's own orders are only undone or shrunk, everything else stays with the user, and the plugin window and the alert say so explicitly (Pêndulo info). **A live account that is not confirmed**, including every live account while the opt-in has never been turned on, gets **no platform write at all**: entries, `flatten`, `reduce`, cancels and protective orders are all refused with `command.account_not_simulation` (Vigia M1). The one exception is undoing or shrinking All Win's own orders: state A of the §N11 table. Without local consent, a misconfigured Core can never act on a real account, not even to close.<br>• **Storage:** a per-Windows-user file readable only by the owner (same ACL as the device token). The value is protected with DPAPI `CurrentUser` plus the plugin's own entropy, so a file copied from another machine or user does not count. A value that is not exactly the one written by the dialog means off.<br>• **Threat model (Vigia L2):** the barrier protects against a wrong or remote configuration in the Core. It does not protect against code that runs as the same Windows user (another NinjaScript in the same process could write the file); the Core-side gate and the user's own machine security cover that.<br>• **Effect:** turning it on applies to commands received afterwards.<br>• **Turning it off, kind changes, and what the plugin may still write:** the decision table in **§N11 below** (user decisions U19 a and U19.1 b, 2026-10-23).<br>• A refused command is `rejected` + `command.account_not_simulation`, and the Core marks the account `out_of_sync` as for any rejection.<br>• `kind: unknown` is always state A of the table, opt-in or not.<br>• **Visibility:** the plugin window always shows the state ("Contas reais: bloqueadas neste computador" / "permitidas"), and every change is written to the plugin's local log with the time.<br>• **Development builds** never execute on live accounts and do not show the setting (N4).<br>• The protocol does not carry the setting: a 1.2 plugin cannot add a `hello` field without breaking a rolled-back Core, so the Core learns it from the rejection | Two independent barriers, one of them on the user's own PC: a Core misconfiguration alone can never route to a live account. Agents test only with simulation (CLAUDE.md §2) |

## N11 — Live-account decision table (U15; U19 a and U19.1 b decided by the user, 2026-10-23)

Closed with QA [Plugins] and QA [Security] on 2026-10-23. Release builds only (development builds never write to a live account, N4).

**Rules the table is derived from:**
- **I1:** never increase or flip a real position.
- **I2:** never leave a real open position without protection.
- **C (consent):** the plugin acts on a live account only with local consent. Consent yields in two cases only:
  - undoing or shrinking orders that All Win itself placed (an `aw:` Name, N5) when keeping them would break I1;
  - protecting the fill of an entry that All Win submitted with consent, capped per row 6, because leaving it unprotected would break I2 (Crivo F1, option a).

**States:**
- **A:** live account not confirmed, its confirmation invalidated (a known non-live `kind` reported), or `kind: unknown` (with the transient-unknown exception below).
- **B:** confirmed, opt-in on and `welcome.simulation_only = false`. Normal rules apply (every row allowed, subject to the normal checks below).
- **C0:** confirmed, after "off" (or after `simulation_only = true`), position flat.
- **C1:** like C0, but non-flat with an All Win identity (ADR-0004 §5).
- **C2:** like C0, but non-flat with an `unknown` or `external` identity.

| # | Operation | A | C0 | C1 | C2 | Why |
|---|---|---|---|---|---|---|
| 1 | Submit an entry, or modify a non-protective All Win order (entry or exit) | N | N | N | N | I1: a modify can add exposure |
| 2 | Cancel a working All Win entry, or a working non-protective All Win exit (e.g. a resting guarded-reduce limit) | Y | Y | Y | Y | I1: if the position changes by hand, a working exit could open or flip a position. Reported as `order.live_consent_cancelled` when the plugin initiates it |
| 3 | Cancel an orphan All Win leg (position flat) | Y | Y | — | — | I1: a leg on a flat account can open a position |
| 4 | Shrink an All Win leg to ≤ \|position\| (down only) | Y | — | Y | Y | I1, keeps I2 |
| 5 | Cancel an All Win leg on the wrong side (would add to or flip the position) | Y | — | Y | Y | I1 |
| 6 | Initial bracket of a consented All Win entry, including its growth per partial fill of that same entry, capped to the quantity that entry has filled and ≤ \|position\| | Y | — | Y | Y | I2; capped, so it never breaks I1. "Consented" = submitted while the account was in state B |
| 7 | Any other upsize of a leg (capped ≤ \|position\|) | N | — | Y | N | I2 against stale state; without an All Win identity the user manages it |
| 8 | Re-price a leg | N | — | Y | N | keeps protection; risky on stale state |
| 9 | `flatten` | N | Y | Y | N | C: an unknown/external position may be the user's own manual trade |
| 10 | Guarded `reduce` (U13) | N | Y | Y | N | U13; on C0 it gives `command.nothing_to_reduce` |
| 11 | Cancel a stop/target that still protects an open position, outside a `flatten`/`reduce` of the same command | N | — | N | N | I2 (**U19.1 option b**). In B the normal rules allow it |

**How the rows are applied:**
- **Transient `kind: unknown`** (Bisturi, plugin review b576867): while the platform reports `unknown` (for example during NT8 start-up or a reconnect), the account is treated as state A for every **Core** write, fail closed. It does **not** invalidate the stored confirmation. The rule is stateless (it also holds after a restart): the confirmation is invalidated only when the account reports a **known** `kind` other than `live` (simulation or playback) (Vigia L1). "Off" does **not** erase the confirmations: it ends state B, so the C states apply, and the next confirmation dialog replaces the confirmed set (Pêndulo M8, Vigia nit). Tests: live → unknown → live keeps the confirmation; live → unknown → simulation invalidates it. While `unknown`, the plugin still performs rows 3–6 on its own (I1/I2 reactions to its own legs and fills). It does **not** self-cancel row-2 orders, because those were submitted with consent. When the kind is known again and still `live`, the account returns to its previous state. If `unknown` lasts more than 10 s, the plugin window alerts the user: "Tipo da conta desconhecido; o All Win não consegue agir nesta conta (nem zerar)" (Vigia I1).
- **Who initiates on A:**
  - The plugin performs rows **2, 3, 4, 5 and 6 on its own**: at the transition to A (kind change, `kind: unknown`), and again whenever the condition appears (a fill, flat, a flip). It acts only on orders with an `aw:` Name, never on another order (Pêndulo M7, Vigia).
  - The row-6 bracket uses the consented command's own protection distances; no Core command is needed.
  - The Core does not know the local consent state, so the plugin cannot wait for it.
  - On A, a Core command is accepted for row 2, and for rows 3 and 5 after the plugin's own local check.
- **Local truth:** flat, side and \|position\| are checked by the plugin from the platform's own state, never taken from the Core's word.
- **On A, every action** produces an alert in the plugin window ("Conta real sem consentimento: ordens All Win canceladas/reduzidas; o restante fica com você no NinjaTrader"), a local log line, and an `execution_report`.
- **A `flatten` that fails after its cancelling step** (protective legs already cancelled, close then `rejected`, e.g. `order.platform_rejected`) leaves a live position open without protection. This is the one known I2 gap, **accepted by the user** (2026-10-23). The plugin raises a top-severity alert, "Posição aberta sem proteção; gerencie no NinjaTrader", and the Core sets `out_of_sync` (Pêndulo L5). It applies in any state where `flatten` is allowed.
- **Gate order:** consent first, then the table, then the command's own checks. On A or C2 a reduce therefore gets `command.account_not_simulation`, not `command.position_changed`; Elo tests this order.
- **The table only adds refusals:** it never overrides the stricter rules, which still apply on top: the step-0 crash-window lookup and a failed lookup (ADR-0004 §5), the no-late-bracket rule (R1, `order.protection_not_submitted`), and catch-up shrinking only.
- **At "off"** (**U19 option a**), the plugin cancels working All Win entries on live accounts at once (row 2, with report and audit). The dialog lists what continues and what stops. It says that a confirmed account in C2 cannot be flattened through All Win: its entries are cancelled, its protection stays, and the user manages it in NinjaTrader.
- **The same table applies** when a new `welcome` brings `simulation_only = true`.
- **Tests:** one EngineHarness test per Y/N cell (`AllWin.Adapter.Core`, Linux), plus the NT8 Sim checklist for the dialog and DPAPI (untested on Windows).
- **Implementation:** every platform write is classified by row before submission. The release/development decision comes from one compile-time constant shared by the connector and the engine.
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
