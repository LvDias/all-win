# Platform profile — NinjaTrader 8 (Desktop)

- **Status:** Research baseline v1. Facts checked **2026-10-23**, nothing verified on a real NinjaTrader machine yet.
- **Owner:** Plugin Researcher (Trilha). Decisions that build on it belong to the Software Architect (Prumo) through the NT8 spike and ADRs.
- **Scope:** what All Win needs from NinjaTrader 8 for V1 slices S2, S11, S12, S15, S19–S21, plus the **Q15 question**: how agents build and test the plugin on the user's Windows machine.
- **Inputs:** [product-vision.md](../../product/product-vision.md), [ADR-0001](../adr/0001-technology-stack.md), [ADR-0002](../adr/0002-repository-structure.md), [slice-plan.md](../../product/release-1/slice-plan.md), [decisions.md](../../delivery/decisions.md).

## Source legend

Each fact carries one of these labels:

| Label | Meaning |
|---|---|
| **[OFF]** | Official source, fetched and read on 2026-10-23 |
| **[OFF-s]** | Official page that could not be opened from our host; the claim comes from a search-engine summary of it |
| **[SEC]** | Secondary source (user forum post, vendor, blog). Never the only basis for a design decision |
| **[INF]** | Our inference from the sources above. Not a documented fact |
| **[EXP]** | Needs a minimal experiment on a Windows + NT8 machine (see §15) |

**How to reach the sources from our host.** The dev host egresses from France.

- `ninjatrader.com/support/helpGuides/nt8/*` redirects to an EU marketing page. The same official pages are served unredirected at `https://static.ninjatrader.com/support/helpGuides/nt8/<page>.htm`; the help guide citations below use that host.
- NinjaScript docs moved to `https://docs.ninjatrader.com/ninjascript/<page>`. Appending `.md` gives clean Markdown, the index is at `/llms.txt`, and an MCP server is available at `https://docs.ninjatrader.com/_mcp/server` [OFF]. The site puts bots behind a Vercel challenge after a burst of requests. The same content is served at `https://ninjatrader-public.docs.buildwithfern.com/ninjascript/<page>.md`.
- The legacy staff forum `forum.ninjatrader.com` now redirects to `discourse.ninjatrader.com`, where old threads return 404. Staff quotes from it are therefore **[SEC]**, taken from search summaries.

---

## 1. Summary for decisions

| # | Finding | Impact |
|---|---|---|
| F1 | An **AddOn** sees every account of every connection in the instance (`Account.All`). It can subscribe to order, execution, position and account events and can submit, change, cancel and flatten orders. | Confirms ADR-0001 §7 (AddOn host, not Strategy). |
| F2 | Current version **8.1.8.4** (2026-10-19). It runs on **.NET Framework 4.8** with a Roslyn compiler (C# 13 in the NinjaScript Editor since 8.1.6) and ships **Newtonsoft.Json 13**. | `net48` host is correct. Do not bundle another Newtonsoft. |
| F3 | A DLL built outside NinjaTrader goes into `Documents\NinjaTrader 8\bin\Custom` and needs an **NT8 restart** to load. Source files compiled by the NinjaScript Editor reload **without** a restart. | Shapes the dev loop and Q15 (§13). |
| F4 | No official headless compiler, command-line login, service mode or test framework exists. The 8.1 login screen does **not** store the password [SEC]. | An agent cannot restart NT8 unattended unless the user accepts an auto-login trade-off. |
| F5 | On the **NinjaTrader** and **Rithmic** connections, **OCO is simulated on the local PC** [OFF]. While NT8 is offline, a filled stop does not cancel its target (or the reverse). | Safety gap for brackets and orphan-order protection (vision §8). Product/Architect attention (§7.3). |
| F6 | **Playback** requires all other connections to be closed. Sim101 needs streaming data to fill; the built-in **Simulated Data Feed** enables fully offline tests. | Test plan needs separate Sim and Playback runs (§8). |
| F7 | One NT8 instance per computer (ToS "single computer"; staff say multiple instances are unsupported) [OFF]/[SEC]. | Multi-instance tests (S19) need two machines or VMs. |
| F8 | **ToS 5(a)(viii)** forbids developing "an interface between Platform to Broker APIs without the express written consent from the Company" [OFF]. | Legal reading is a user decision (§11). |
| F9 | 8.1.8 added a **manual lockout for prop accounts on the NinjaTrader connection**. It has no NinjaScript API. | S15's block stays reactive in our plugin, as ADR-0001 assumes (§12). |
| F10 | For Q15, an option that needs **no new inbound port** exists: a reverse SSH tunnel from Windows to the dev host. With Git Bash as the SSH shell, the **Maestri CLI** (bash + curl + sed; jq and tmux optional) should run. Whether the Maestri *app* supports a Windows host is unknown. | Options for Prumo (§13). |

---

## 2. Versions, OS and runtime

| Fact | Source |
|---|---|
| Latest release is 8.1.8 (2026-07-21), with patches 8.1.8.1 (Jul 24), 8.1.8.2 (Aug 11), 8.1.8.3 (Sep 29) and **8.1.8.4 (Oct 19, 2026)**. | [OFF] [release notes](https://static.ninjatrader.com/support/helpGuides/nt8/release_notes.htm), [8.1.8](https://static.ninjatrader.com/support/helpGuides/nt8/8_1_8.htm) |
| Minimum requirements: "Windows 11, Windows Server 2016 or later 64-bit", "Microsoft .NET Framework 4.8", "DirectX10 compatible graphics card highly recommended". Windows 10 is no longer listed. | [OFF] [minimum_system_requirements](https://static.ninjatrader.com/support/helpGuides/nt8/minimum_system_requirements.htm) |
| .NET 4.8 since R23 (8.0.23): the Visual Studio AddOn sample "targets .NET 4.8 with NinjaTrader Release R23 or higher". | [OFF] [addon_development_overview](https://docs.ninjatrader.com/ninjascript/addon_development_overview) |
| C# level by release:<br>• 8.1.2: "Updated compiler to use Roslyn".<br>• 8.1.4: C# 9.<br>• 8.1.6: "NinjaScript Editor with C# 13 support". | [OFF] [8.1.2](https://static.ninjatrader.com/support/helpGuides/nt8/8_1_2.htm), [8.1.6](https://static.ninjatrader.com/support/helpGuides/nt8/8_1_6.htm) |
| Language features that need .NET Core runtime support may still fail on .NET Framework 4.8. | [INF] |
| 8.1.2: "Updated Newtonsoft to version 13 … Some custom scripts that reference old versions of Newtonsoft will need the references updated". The exact 13.0.x build is not documented. | [OFF] [8.1.2](https://static.ninjatrader.com/support/helpGuides/nt8/8_1_2.htm), [EXP] |
| Since 8.1.1 the platform signs in with a NinjaTrader user login instead of license keys. | [OFF-s] 8.1.1 notes, as summarized by the subagent research |
| A GPU is "highly recommended", not required. Without one, Windows renders DirectX 10 in software through WARP ([Microsoft WARP](https://learn.microsoft.com/en-us/windows/win32/direct3darticles/directx-warp)). Staff acknowledged "some DirectX errors" on VMs. A GPU-less Windows Server 2022 Desktop Experience VPS is acceptable for compile and Sim/Playback tests; E1/E3 must confirm stability, with a vGPU plan as the fallback. Recommended spec (official): quad core, 8 GB RAM, SSD. Our dev box should have 6–8 vCPU, 16–24 GB RAM and 150–200 GB NVMe [INF]. | [OFF] requirements (checked 2026-10-23); [SEC] forum; [EXP] |

## 3. Extension types: AddOn vs Strategy vs Indicator

| Type | Fits All Win? | Why |
|---|---|---|
| **AddOn** (`AddOnBase`, optional `NTWindow`/`NTTabPage`) | **Yes — host** | "AddOns can also subscribe to live market data, access account information, and more". An AddOn "does not require its own window to function" and can do "non-UI-driven functionality across the platform, such as monitoring market data or accessing account, position, and order information" [OFF] ([overview](https://docs.ninjatrader.com/ninjascript/addon_development_overview), [other uses](https://docs.ninjatrader.com/ninjascript/other_uses_for_an_addon)). Our plugin window (status, pairing, 80 % warnings) is an `NTWindow` opened from the Control Center menu (`NTMenuItem` in `OnWindowCreated`). |
| **Strategy** (managed/unmanaged) | No | A strategy runs on one instrument and one account, has its own virtual "Strategy Position" separate from the "Account Position", and can diverge from it [OFF] ([strategy vs account position](https://docs.ninjatrader.com/ninjascript/strategy_position_vs_account_position)). Replication and risk need every account (ADR-0001 §13). |
| **Indicator** | No | Chart-bound; no order API. |

**Lifecycle.** AddOns use the "Active" state model (`SetDefaults` → `Active` → `Terminated`), not the data-processing states of indicators and strategies [OFF-s] ([OnStateChange](https://docs.ninjatrader.com/ninjascript/onstatechange)). Clean up event subscriptions and sockets in `State.Terminated` [OFF] ([best practices](https://docs.ninjatrader.com/ninjascript/best_practices)).

**Recompile behavior to verify [EXP]:**
- Does an AddOn instance get `Terminated` and recreated when NinjaScript is recompiled?
- `OnWindowCreated()` is called "in the thread of that window" [OFF].

## 4. Project structure, references, compilation and loading

There are two official development paths [OFF] ([addon_development_overview](https://docs.ninjatrader.com/ninjascript/addon_development_overview)):

| Path | How it builds | Reload | Notes |
|---|---|---|---|
| **NinjaScript Editor** (source `.cs` under `Documents\NinjaTrader 8\bin\Custom\…`) | NT8 compiles **all** NinjaScript files into one `NinjaTrader.Custom.dll` (F5 or right-click Compile) [OFF] ([compile_errors](https://docs.ninjatrader.com/ninjascript/compile_errors), [visual_studio_debugging](https://docs.ninjatrader.com/ninjascript/visual_studio_debugging)) | "Changes … are reflected immediately upon NS Editor Compile and does not require restart" | When an external editor saves a file "preferably with the NinjaScript editor opened still at the same time, so changes would be auto compiled in then" [OFF] ([visual_studio_debugging](https://docs.ninjatrader.com/ninjascript/visual_studio_debugging)). A compile error in **any** script blocks the whole build. |
| **Visual Studio / SDK project** that builds its own DLL | References the NT DLLs; a post-build step copies the DLL "into the appropriate folder (NinjaTrader 8/bin/Custom)" | "NinjaTrader needs to be restarted in order to re-load the compiled DLL after changes" | Can bundle XAML and resources [OFF]. The official sample is `NinjaTraderAddOnProject.zip` (not downloadable from our host; get it on Windows [EXP]). |

More facts and constraints:

- **NinjaTrader DLLs.** `NinjaTrader.Core.dll`, `NinjaTrader.Gui.dll` and others live in `C:\Program Files\NinjaTrader 8\bin`, and `NinjaTrader.Custom.dll` in `Documents\NinjaTrader 8\bin\Custom` [OFF] (sample start action path; Custom folder per export docs). The exact references and HintPaths of the official sample are unverified [EXP].
  - Per ADR-0002 these DLLs are never committed.
  - Copying them to Linux CI is **not** cleared by any license text we found. Treat the `net48` host as **Windows-only to build** [INF].
- **Do not edit `NinjaTrader.Custom.csproj`.** NT regenerates it [SEC] (forum staff, via search summaries).
- **Third-party DLL references** are added in the NinjaScript Editor under right-click > References… [OFF] ([editor components](https://docs.ninjatrader.com/ninjascript_editor_components)).
- **netstandard2.0 inside NT8.** One user reports CS0012 ("reference netstandard 2.0.0.0"). Their fix: copy the .NET 4.8 facade `netstandard.dll` and all dependencies into `bin\Custom` and reference them, or add them to `Config.xml` `<References>` while NT is closed [SEC] ([discourse t/7226](https://discourse.ninjatrader.com/t/7226)).
  - **Recommendation:** multi-target `AllWin.Adapter.Core` as `netstandard2.0;net48`, ship the `net48` build to NT8, and keep `netstandard2.0` for Linux tests. This avoids the facade question. ADR-0001 §7 lists this load question for the spike [INF][EXP].
- **Dependency conflicts.** Two add-ons that ship different copies of a common library (e.g. Newtonsoft) can collide; vendors merge or obfuscate to avoid it [SEC] ([discourse t/2722](https://discourse.ninjatrader.com/t/2722)). **Recommendation:** no Newtonsoft in our package. Either reference NT's Newtonsoft 13 or use a dependency-free JSON layer in `AllWin.Adapter.Core`; the Architect/Plugin Engineer decide.
- **Assembly unloading.** .NET Framework cannot unload an assembly from a running AppDomain. "Remove NinjaScript Assembly will not unload existing assemblies until restart" [OFF] ([remove assembly](https://docs.ninjatrader.com/ninjascript/remove_ninjascript_assembly)).
- **Debugging.** Visual Studio 2019/2022 "Attach to Process" on NinjaTrader, using the debug DLL produced in "Debug Mode" [OFF] ([visual_studio_debugging](https://docs.ninjatrader.com/ninjascript/visual_studio_debugging)).

## 5. Threading and dispatcher rules

All [OFF], from [multi_threading_consideration_for_ninjascript](https://docs.ninjatrader.com/ninjascript/multi_threading_consideration_for_ninjascript) and the API examples:

- NT8 has several UI threads plus background worker threads. Events arrive on threads that are not your object's thread.
- Use `Dispatcher.InvokeAsync()` to touch UI. "Calling the synchronous Dispatcher.Invoke() method can potentially result in a deadlock". Check `Dispatcher.CheckAccess()` first.
- **Lock shared collections:** `lock (Account.All)`, `lock (account.Executions)`, `lock (Connection.Connections)`, `lock (Core.Globals.ConnectOptions)` (all in the official examples).
- **Event sequence vs current state.** `OrderUpdate` delivers each state change **in sequence**. The `Order` object passed in is the *current* core object and can already be ahead (e.g. the event says Working while `order.OrderState` is PartFilled). Use the event-args values for the sequence and the order object for the latest state.
- **Connection events:** "work with a copy of the ConnectionStatusEventArgs".
- **Our design implications [INF]:**
  - Event handlers must do no blocking I/O. Copy the event data into the `AllWin.Adapter.Core` state machine through a single-consumer queue, and do WebSocket I/O on our own task.
  - `ClientWebSocket` allows only one concurrent send and one concurrent receive [OFF] ([Microsoft](https://learn.microsoft.com/en-us/dotnet/api/system.net.websockets.clientwebsocket?view=netframework-4.8)).
  - One user reports broker disconnects while an AddOn held a Socket.IO connection, suspected to be dispatcher contention [SEC] ([discourse t/6605](https://discourse.ninjatrader.com/t/6605)). We need a load experiment [EXP].

## 6. Accounts, orders, executions and positions (all accounts in an instance)

All [OFF] from [Account class](https://docs.ninjatrader.com/ninjascript/account_class), [Order](https://docs.ninjatrader.com/ninjascript/order), [CreateOrder](https://docs.ninjatrader.com/ninjascript/createorder), [Connection](https://docs.ninjatrader.com/ninjascript/connection_class) and [ConnectOptions](https://docs.ninjatrader.com/ninjascript/connectoptions), unless marked.

| Need | API | Notes |
|---|---|---|
| Enumerate accounts | `Account.All` (static) | Each `Account` has `.Name` and `.Connection`. `Connection.Connections` lists the connected connections, and `Connection.Accounts` the accounts of each. That `Account.All` spans **all simultaneous connections** in Multi-provider mode is implied, not stated [EXP]. |
| Account added/removed/status | `Account.AccountStatusUpdate` (static, all accounts) | Fires "on any status change of any account". |
| Connection health | `Connection.ConnectionStatusUpdate` (static); `Connection.Status` (order feed) and `PriceStatus` (price feed): Connected / Connecting / ConnectionLost / Disconnecting / Disconnected | Feeds S2 AC-4 and S11 (instance/account disconnect). |
| Live vs simulation | `connection.Options.Mode` is `Mode.Live` or `Mode.Simulation`. `Provider` identifies the provider; `Provider.Playback` appears in the official example. | Basis of the **simulation guard** (ADR-0001 §7). The guard must not trust account names alone (§14). |
| Orders | `account.OrderUpdate`; `account.Orders` | `OrderState`: Initialized, Submitted, Accepted, TriggerPending, Working, ChangePending, ChangeSubmitted, CancelPending, CancelSubmitted, Cancelled, Rejected, PartFilled, Filled, Unknown. `Order.IsTerminalState()`. "In real-time, some stop orders may only reach 'Accepted' state if they are simulated/held on a brokers server." |
| Fills | `account.ExecutionUpdate`; `account.Executions` | Fires when "new executions come in, an existing execution is amended … or an execution is removed (e.g. by the broker's back office)". **Reports must handle amend/remove.** There is "not a supported method to retrieve historical executions from the local database" [OFF-s]. |
| Positions | `account.PositionUpdate`; `account.Positions` | "The core MarketPosition e.Position is considered flat when Operation.Remove is seen … Operation.Update would be seen if there was no flat state in between, i.e. on a reverse." This is the trigger for **orphan-order cancellation** (vision §8). |
| Account values / P&L | `account.AccountItemUpdate`; `account.Get(AccountItem.X, Currency)` | Items include RealizedProfitLoss, UnrealizedProfitLoss, GrossRealizedProfitLoss, CashValue, NetLiquidation, Commission, Fee, BuyingPower and others. There are **no** items for loss limit, drawdown or lockout. Values are `double`; convert to `decimal` at the boundary (ADR-0001). |
| Submit | `account.CreateOrder(instrument, action, type, orderEntry, tif, qty, limit, stop, oco, name, gtd, customOrder)` then `account.Submit(orders)` | Order types: Market, Limit, StopMarket, StopLimit, MIT. TIF: Day, Gtc, Gtd, Ioc, Opg. **`name` max 50 characters**, usable to carry a short `command_id` reference (ADR-0001 §7 spike item). `oco` is a string group id. **`OrderEntry.Automated` sets CME tag 1028** (manual-order indicator): our routed orders must use `Automated` [INF]. |
| Change / cancel | `account.Change(orders)` with `StopPriceChanged` / `LimitPriceChanged` / `QuantityChanged`; `account.Cancel(orders)`; `account.CancelAllOrders(instrument)`; `Connection.CancelAllOrders()` | Basis of stop/target moves (S20 AC-2) and blocks (S15). |
| Flatten | `account.Flatten(ICollection<Instrument>)` | "A collection of Instruments for orders to be cancelled and positions closed". |
| Identity | `Order.OrderId` "is NOT a unique value … it can change throughout an order's lifetime". Compare `Order` objects by reference. An undocumented `Id` field exists [SEC]. | For reconciliation after a plugin restart, rely on `Order.Name` (our reference) plus account and instrument [INF][EXP]. |
| Sim reset / Playback rewind | `Account.SimulationAccountReset` | Fires on manual reset **and** on Playback rewind/fast-forward. The plugin must treat it as "state discontinuity → flag out of sync", not as fills [INF]. |
| Connect programmatically | `Connection.Connect(ConnectOptions)` from `Core.Globals.ConnectOptions` | ⚠ **Can connect live accounts** (§14). Agents use it only for Sim/Playback test automation, if at all. |

## 7. Stop/target: ATM vs Account-API brackets ("unmanaged")

### 7.1 Options

| Option | How | Pros | Cons for All Win |
|---|---|---|---|
| **ATM strategy** | `AtmStrategy.StartAtmStrategy(templateName, entryOrder)`; the entry order **must** be named `"Entry"` [OFF] ([StartAtmStrategy](https://docs.ninjatrader.com/ninjascript/startatmstrategy)). Child orders are tracked via ATM methods, e.g. `GetAtmStrategyStopTargetOrderStatus("Stop1", id)` [OFF-s]. | Native NT UI and bracket handling; stop strategies (breakeven, trailing). | Distances come from a **template** on the Windows machine, not the Core's per-account profile (vision §6 and §8). Individual vs shared mode, "move by the same X" (S20), max-position capping and orphan cleanup would be split between NT templates and our logic. ATM strategies "operate in real-time only" [OFF] ([using ATM strategies](https://docs.ninjatrader.com/ninjascript/using_atm_strategies)). Local ATMs rely on the local simulation infrastructure; server-side ATMs exist only with Multi-provider **disabled** and "cannot be used … connected to Playback" [OFF] ([multi-provider](https://static.ninjatrader.com/support/helpGuides/nt8/enabling_disabling-multi-provi.htm)). |
| **Unmanaged strategy orders** (`SubmitOrderUnmanaged`, `IsUnmanaged`) | Strategy-only API [OFF] ([unmanaged approach](https://docs.ninjatrader.com/ninjascript/unmanaged_approach)) | — | Not available to an AddOn. Strategies don't fit (§3). |
| **Account-API brackets** (recommended) | After the entry fills (`OrderUpdate` Filled / `ExecutionUpdate`), submit a StopMarket and a Limit with a **shared OCO id**. This is the official AddOn example [OFF] ([OrderUpdate](https://docs.ninjatrader.com/ninjascript/orderupdate)). | Distances come from the Core per account. One implementation of individual/shared mode, offset moves, capping and orphan cleanup in `AllWin.Adapter.Core`. Works the same on Sim101 and Playback. | We own partial-fill handling (resize brackets per fill), OCO races and reconnect recovery. |

### 7.2 Recommendation

Use **Account-API brackets**, i.e. the AddOn equivalent of "unmanaged". This is consistent with ADR-0001 (Core-owned rules, plugin-side conversions). ATM remains out of scope unless the user asks for NT stop strategies.

### 7.3 Safety gap: where OCO and stops live [OFF]

Source: [where do your orders reside](https://static.ninjatrader.com/support/helpGuides/nt8/where_do_your_orders_reside_.htm) and [risks of electronic trading](https://static.ninjatrader.com/support/helpGuides/nt8/risks_of_electronic_trading_wi.htm).

| Connection | Where OCO lives |
|---|---|
| **NinjaTrader** | "Most OCO (One Cancels Other) functionality is simulated on your local PC." |
| **Rithmic** (common for prop firms) | "OCO … functionality is simulated on your local PC." |
| Continuum/CQG, Interactive Brokers | Mostly native on the server. |

Stop and limit orders in Accepted/Working state rest "at the brokerage or exchange". If the exchange does not support an order type, it stays on the provider's servers.

Consequences:
- **What happens when NT8 is down:** "If NinjaTrader is offline (internet connection is down or PC crashed) then the simulated OCO functionality will not be operational." A filled stop then leaves its target working, and vice versa.
- **Recovery:** "MIT orders and Simulated Orders … in TriggerPending state could not be recovered … The order state after connection recovery would then be 'Unknown'."
- **For All Win [INF]:** our orphan-order protection (vision §8) and S11 disconnect handling cannot assume the bracket is atomic at the broker on the NinjaTrader and Rithmic connections. Reconnect logic must:
  - detect the *both-legs* and *orphan-leg* cases from `position_snapshot` plus `Orders`;
  - **alert and wait** (never auto-correct, per vision §8).
- The PO/Architect should decide whether this needs user-facing text or an acceptance criterion.

## 8. Simulation, Playback and multiple instances

### Sim101 and local simulation accounts

- "A default account that represents your own simulated account … behaves identical to a live account". It "cannot be removed" [OFF] ([Sim101](https://static.ninjatrader.com/support/helpGuides/nt8/the_sim101_account.htm), [accounts tab](https://static.ninjatrader.com/support/helpGuides/nt8/accounts_tab.htm)).
- **Reset:** Accounts tab → Edit Account → Initial cash → reset. Realized P&L returns to 0, and `Account.SimulationAccountReset` fires [OFF].
- **Extra sim accounts:** "unlimited number of simulation accounts" (Accounts tab → New simulation account; "active the next time you connect to a data provider") [OFF] ([multiple sim accounts](https://static.ninjatrader.com/support/helpGuides/nt8/multiple_simulation_accounts.htm)).
  - Since 8.1.1, local sim accounts exist **only with Multi-provider mode enabled** [OFF] ([multi-provider](https://static.ninjatrader.com/support/helpGuides/nt8/enabling_disabling-multi-provi.htm)).
  - A mandatory "Sim" name prefix is not documented [SEC]/[EXP].
- **Fill engine:** a fill-probability model ("ask/bid volume, trade volume, time … random time delays"). TIF expiry is not supported on Sim101, and the simulator rolls to the next day at 4:15 EST [OFF] ([simulation](https://static.ninjatrader.com/support/helpGuides/nt8/simulation.htm), [trading in simulation](https://static.ninjatrader.com/support/helpGuides/nt8/trading_in_simulation.htm)).
- **Data source:** realistic fills need streaming market data. The built-in **Simulated Data Feed** connection generates data internally for "Offline testing of trade automation using NinjaScript". It has no historical data and no relation to real prices [OFF] ([simulated data feed](https://static.ninjatrader.com/support/helpGuides/nt8/simulated_data_feed_connection.htm)). **This is the agent-friendly test source: no external connection at all.**
- **Global Simulation Mode:** when on, "all order entry interfaces (SuperDOM, Chart Trader, etc.) will only allow selection of a simulation account". "Free license users are not able to disable Global Simulation Mode" [OFF] ([global sim mode](https://static.ninjatrader.com/support/helpGuides/nt8/global_simulation_mode.htm)).
  - It is documented for UI order entry only. Whether it blocks `Account.Submit` from NinjaScript is unknown [EXP]. **Do not rely on it.** Our own guard is mandatory.

### Playback (Market Replay)

- "You must close all other connections before connecting to Playback" [OFF] ([playback connection](https://static.ninjatrader.com/support/helpGuides/nt8/playback_connection.htm)). **Playback and Sim101 + data feed tests run in separate sessions.**
- **Controls:** play/pause, speed up to "Max", Go To date/time. The caption shows the play-head time. Orders "are processed immediately and synchronously"; there is no latency simulation. Sliding resets the Playback account's trade history [OFF] ([playback](https://static.ninjatrader.com/support/helpGuides/nt8/playback.htm)).
- **Playback101:** settings derive from Sim101 (help page), but 8.1.2 notes say they can now be changed independently. 8.1.7 adds multiple Playback accounts (Multi-provider required) [OFF]/[EXP].
- **Replay data:** NinjaTrader offers limited free Market Replay data (Tools > Historical Data > Get Market Replay data, while disconnected from Playback). Users can also record their own [OFF] ([set up](https://static.ninjatrader.com/support/helpGuides/nt8/set_up12.htm)).
  - Downloading needs a data connection [SEC]. **The user does this, not agents** (it means connecting a data feed with the user's login).
- **Simulated clock (S15 AC-7):** no official reference page for a "playback time" API was found.
  - Staff mentioned `NinjaTrader.Cbi.Connection.PlaybackConnection.Now` (null when not on Playback) and `Core.Globals.Now` in an NT8-beta thread [SEC].
  - Detecting Playback via `Provider.Playback` is official [OFF].
  - **[EXP] E5** is required before S15.

### Multiple connections and instances

- **Multiple connections:** "NinjaTrader supports multiple simultaneous connections to different connectivity providers" (Multi-provider mode) [OFF] ([multiple connections](https://static.ninjatrader.com/support/helpGuides/nt8/multiple_connections.htm)). Trading third-party brokers such as Rithmic or Schwab may need the paid **Multiple Broker add-on** [OFF] for Schwab ([8.1.4](https://static.ninjatrader.com/support/helpGuides/nt8/8_1_4.htm)), [SEC] for Rithmic. This is a user cost question for live use, not for agents.
- **Multiple instances:**
  - ToS 1(f): "license to use the Platform on a single computer at any one time" [OFF] ([ToS](https://static.ninjatrader.com/support/helpGuides/nt8/tos.htm)).
  - Staff: running two instances on one machine "is not recommended and is not a supported operation"; use a VM/VPS [SEC].
  - User-based logins allow "two simultaneous market data active sessions" [OFF] ([licensing FAQ](https://docs.ninjatrader.com/ninjascript/user_based_licensing_faq)).
  - **Implication:** the vision's "more than one NinjaTrader instance" means separate machines or VMs. Testing S19 cross-instance needs **two Windows machines/VMs** (or one VM plus the user's PC) [INF].

## 9. Outbound WebSocket from NT8

- `System.Net.WebSockets.ClientWebSocket` is in `System.dll` on .NET Framework 4.8 and available to `netstandard2.0`.
  - "The only public implementations of client and server WebSockets are supported on Windows 8 and Windows Server 2012" — fine for Windows 11 / Server 2016+ [OFF] ([Microsoft Learn](https://learn.microsoft.com/en-us/dotnet/api/system.net.websockets.clientwebsocket?view=netframework-4.8)).
  - **One send and one receive at a time:** serialize sends.
- **No extra library is needed**, which avoids the dependency conflicts of §4. Third-party clients (e.g. websocket-sharp) are unnecessary [INF].
- **TLS:** .NET 4.8 uses the OS TLS defaults (TLS 1.2/1.3 on Windows 11) [INF; verify with the production Caddy endpoint during the spike].
- **No inbound port** is needed on the trader's machine (ADR-0001 §6).

## 10. Packaging, distribution and install flow

All [OFF] ([export](https://docs.ninjatrader.com/ninjascript/export), [import](https://docs.ninjatrader.com/ninjascript/import), [distribution procedure](https://docs.ninjatrader.com/ninjascript/distribution_procedure), [compiled assemblies](https://docs.ninjatrader.com/ninjascript/considerations_for_compiled_assemblies), [best practices](https://docs.ninjatrader.com/ninjascript/best_practices), [3rd-party add-ons](https://static.ninjatrader.com/support/helpGuides/nt8/using_3rd_party_add-ons.htm)):

- **Format:** a "NinjaScript Archive File" (.zip). It is created in-app via Tools > Export > NinjaScript… (source or "Export as compiled assembly", optionally "Protect compiled assembly" with Agile.NET).
  - The exporter "can't add resource files"; build in Visual Studio instead.
  - A VS-built DLL plus extra DLLs go into the zip root, with an `AdditionalReferences.txt` listing the assembly names (distribution procedure page).
  - **The zip layout VS → import must be validated [EXP] E3.**
- **User install:** Control Center → Tools > Import > NinjaScript Add-On… → select zip → Import (option to replace), then restart NinjaTrader. Uninstall: Tools > Remove NinjaScript Assembly, then restart. Only import archives "from a trusted source".
- **Compatibility:** "NinjaScript exports might not be backwards compatible" with older NT8 releases. Pin and test against the minimum NT version we support [OFF].
- **Vendor practice:**
  - Do not ship `.cs` files.
  - Prefix names (`AllWin…`).
  - A custom installer must not overwrite NT files.
  - Clean up resources in `State.Terminated`.
- **Vendor licensing** (`VendorLicense(productId)`, NT ≥ 8.1.5) is **optional**; the free service is for "qualified 3rd party developers". It is not needed for a private tool; the user decides if All Win is ever sold [OFF] ([licensing](https://docs.ninjatrader.com/ninjascript/licensing_user_authentication)).
- **Update flow [INF]:** import the new zip with "replace", restart NT8, and the plugin sends its version in `hello` (ADR-0001 §6). An unsupported version shows "update required".

## 11. Licensing and terms constraints (user decides)

| Item | Text | Source |
|---|---|---|
| Free license | Sim and charting. "Free license users are not able to disable Global Simulation Mode". Plans: Free / Monthly / Lifetime, "free simulated trading" on all. | [OFF] global sim mode; [OFF-s] [pricing](https://ninjatrader.com/pricing/) |
| Third-party apps | ToS 1(c): vendors "may … develop plugins including … indicators, strategies and other various utilities ('Apps')"; use is at the user's sole risk. | [OFF] [ToS](https://static.ninjatrader.com/support/helpGuides/nt8/tos.htm) |
| **Broker-API interfaces** | ToS 5(a)(viii) forbids using the Platform to "develop an interface between Platform to Broker APIs without the express written consent from the Company". | [OFF] [ToS](https://static.ninjatrader.com/support/helpGuides/nt8/tos.htm) |
| Single computer | ToS 1(f): "on a single computer at any one time". | [OFF] |
| EULA (NinjaTrader LLC, updated 2026-06-12, hosted on tradovate.com) | 2.1(b) no use "for the direct or indirect benefit of any person or entity, other than yourself"; 2.1(d) no "combine or incorporate the Software … with or into any other materials, programs or systems"; 2.3 "Commercial Use Prohibited". It is unclear whether this EULA covers the desktop platform or only web/mobile. | [OFF-s] [EULA](https://www.tradovate.com/nt-terms-and-eula/) (read by the subagent) |
| Automated trading | No prohibition found. NinjaScript officially supports automated order submission, and `OrderEntry.Automated` exists for the CME 1028 tag. | [OFF] [CreateOrder](https://docs.ninjatrader.com/ninjascript/createorder) |
| Trade copiers | No official NinjaTrader policy found. Prop-firm rules govern (vision §9 presets). | [SEC] |
| Order rate limits | None published for desktop. Users report "A rate limit for too many order requests has been hit" on NinjaTrader/Tradovate prop accounts while copying. The NinjaTrader Trade API (REST) is rate-limited (HTTP 429). | [SEC] [discourse t/3843](https://discourse.ninjatrader.com/t/3843); [OFF-s] [API auth](https://docs.ninjatrader.com/api/authentication) |

**Reading [INF], not legal advice:**
- All Win's plugin sends orders through NinjaTrader's own Account API to the user's own connections. It is an "App" in the ToS 1(c) sense, not a new Broker-API integration.
- The broadly worded clauses still deserve a user decision:
  - 5(a)(viii);
  - the EULA's "other than yourself" and "commercial use", if All Win is ever offered to other traders.
- Options: ask NinjaTrader Vendor Support for written confirmation, or seek legal review (vision §12).
- These are **not** blockers for sim-only development.

## 12. Built-in risk and lockout features (relevant to S15)

- **Risk templates** apply to **local simulation accounts** only: margins, Max Order Size, Max Position Size. "To define your Live or Simulation NinjaTrader risk, you can go to the Client Dashboard" [OFF] ([risk window](https://static.ninjatrader.com/support/helpGuides/nt8/using_the_risk_window.htm)).
- The Accounts tab shows optional columns: Daily loss limit, Daily profit trigger, Trailing max drawdown, Weekly loss limit, Weekly profit trigger (8.1.3) [OFF].
- **Manual lockout (8.1.8):** "Manual lockout for Prop accounts on the NinjaTrader connection … Starting a lockout automatically cancels working orders, flattens open positions, and prevents new orders until the lockout expires" [OFF] ([8.1.8](https://static.ninjatrader.com/support/helpGuides/nt8/8_1_8.htm)). The maximum is 24 h and it cannot be ended early [SEC] ([NinjaTrader prop blog](https://prop.ninjatrader.com/blogs/manual-lockout-ninjatrader-prop/)).
- **No NinjaScript API** to read or set risk limits or lockouts was found. `AccountItem` has none [OFF]. The REST Trade API has lockout and risk endpoints for NinjaTrader/Tradovate accounts only [OFF-s].
- **Consequence:**
  - S15 AC-6 ("block is reactive") stands for a generic plugin.
  - A future **server-side Tradovate/NinjaTrader adapter** (release 2) could use real lockouts. This is a note for the Architect, not V1 scope.

## 13. Remote build and test on the Windows machine (Q15)

This section gives facts and options; Prumo decides the spike and presents the options to the user.

### 13.1 Constraints that drive the options

| # | Constraint | Source |
|---|---|---|
| C1 | NT8 is a WPF desktop app (DirectX 10). It must run in a **logged-in interactive desktop session**. Processes started by `sshd` (a service) run in session 0 and cannot show UI on the user's desktop. | [OFF] [MS session 0](https://msdn.microsoft.com/en-us/library/bb203962); [OFF-repo] [Win32-OpenSSH #998](https://github.com/PowerShell/Win32-OpenSSH/issues/998) |
| C2 | **No command-line login.** The 8.1 login screen saves the user name but not the password ("Product Management has specifically chosen not to save the password"). Every NT8 start needs the user, or a third-party auto-login tool that stores the password. | [SEC] [forum 1237868](https://forum.ninjatrader.com/forum/ninjatrader-8/platform-technical-support-aa/1237868-starting-up-ninjatrader-desktop-8-1-without-typing-username-and-password-every-time) — recheck on 8.1.8.4 [EXP] |
| C3 | **No headless NinjaScript compile.** `NinjaTrader.Custom.dll` is built only by the running NT8. A DLL we build ourselves loads only after an NT8 restart (→ C2). | [OFF] §4 |
| C4 | Source files saved while the NinjaScript Editor is open are **auto-compiled with no restart**. | [OFF] [visual_studio_debugging](https://docs.ninjatrader.com/ninjascript/visual_studio_debugging) |
| C5 | **No official test framework.** Unit tests run on our `AllWin.Adapter.Core` (Linux). Host checks run against Sim101 + Simulated Data Feed, and Playback. | [SEC] staff; [OFF] §8 |
| C6 | **Opening an inbound port** on the Windows machine needs user authorization (decision log 2026-10-23). Installing OpenSSH Server creates and enables the `OpenSSH-Server-In-TCP` rule (port 22). | [OFF] [OpenSSH install](https://learn.microsoft.com/en-us/windows-server/administration/openssh/openssh_install_firstuse) |
| C7 | The **Maestri CLI** (`~/.maestri/bin/maestri`, 57 lines, read 2026-10-23):<br>• is a bash script using `curl` (required), `jq` (optional; sed fallback) and `tmux` (optional, for token refresh);<br>• posts to `http://$MAESTRI_HOST/cli`. On this host `MAESTRI_HOST=localhost:7433` is a loopback listener, provided through the Maestri app's SSH connection [INF from `ss -ltn`]. | Local inspection |
| C8 | **Git for Windows** ships bash, curl (mingw), sed, awk, grep and coreutils. **jq and tmux are not bundled** by default. Windows also ships `curl.exe` (since build 17063). | [OFF] [git-for-windows file list](https://raw.githubusercontent.com/git-for-windows/build-extra/main/make-file-list.sh); [OFF-s] Microsoft |
| C9 | Windows OpenSSH's default shell is `cmd.exe`. It can be changed via `HKLM\SOFTWARE\OpenSSH\DefaultShell`; Git Bash `C:\Program Files\Git\bin\bash.exe` works. Admin keys go in `%ProgramData%\ssh\administrators_authorized_keys`. | [OFF] [OpenSSH config](https://learn.microsoft.com/en-us/windows-server/administration/openssh/openssh-server-configuration), [DefaultShell wiki](https://github.com/PowerShell/Win32-OpenSSH/wiki/DefaultShell) |
| C10 | **Claude Code** runs natively on Windows 10 1809+ / Server 2019+. It uses Git Bash for its Bash tool when Git for Windows is installed. Sandboxing is unsupported on native Windows and supported in WSL 2. **Remote Control** is outbound-only HTTPS (Pro/Max/Team/Enterprise login; not API keys). Claude Desktop SSH sessions support Windows OpenSSH targets ("get less testing"). | [OFF] [setup](https://code.claude.com/docs/en/setup), [remote control](https://code.claude.com/docs/en/remote-control), [desktop](https://code.claude.com/docs/en/desktop) |

### 13.2 Building

| Item | Facts / recommendation |
|---|---|
| `AllWin.Adapter.Core` | `dotnet test` on Linux (already in ADR-0001). It can also be built for `net48` on Linux with the `Microsoft.NETFramework.ReferenceAssemblies` package: "enable building .NET Framework projects on any machine with at least MSBuild or the .NET Core SDK" [OFF] ([microsoft/dotnet README](https://github.com/microsoft/dotnet/blob/main/releases/reference-assemblies/README.md)). |
| `AllWin.NinjaTrader` (net48 host) | Build **on Windows**, where the NT DLLs are installed (§4). Use the .NET SDK (`dotnet build`) or Build Tools for Visual Studio; neither needs the Visual Studio IDE. WPF/XAML in an SDK-style `net48` project needs a Windows build [INF]. |
| Two dev loops | **(a) Restart loop:** copy built DLLs to `bin\Custom`, restart NT8. Each restart hits C2. **(b) Hot loop:** deploy the host project as **source** under `bin\Custom\AddOns\AllWin\` with the NinjaScript Editor open; NT auto-compiles (C4). The `AllWin.Adapter.Core` DLL still needs a restart when it changes, unless it is also deployed as source during development. Whether NT's compiler builds it unchanged is [EXP] E2. |
| Compile results | Compile errors show in the NinjaScript Editor UI. Whether they are also in `Documents\NinjaTrader 8\log\*.txt` / `trace\*.txt` (readable over SSH) is [EXP] E2. |
| Starting NT8 from a remote shell | Create once, in the user's session, a Task Scheduler task with `/it` ("run only when the user is logged on" → `TASK_LOGON_INTERACTIVE_TOKEN`). Trigger it with `schtasks /run /tn AllWinStartNT8` from SSH [OFF] ([schtasks](https://learn.microsoft.com/en-us/windows-server/administration/windows-commands/schtasks-create), [logon types](https://learn.microsoft.com/en-us/windows/win32/api/taskschd/ne-taskschd-task_logon_type)). Whether the window then appears on the desktop is [EXP] E1. The login screen still needs C2. |

### 13.3 Testing and observability

- **Unit and contract tests:** Linux CI (`AllWin.Adapter.Core`, fixtures from `contracts/`).
- **Host tests on Windows:** Sim101 or additional local sim accounts + **Simulated Data Feed** (no external connection), and Playback (separate session, user-downloaded or recorded data).
  - A **scripted scenario harness** is recommended [INF]: the plugin connects to a scripted **fake Core**, the mirror of `adapter-sim`. Scenarios: entry → fill → bracket, partial fills, reject, disconnect/reconnect, expiry, flatten/block, orphan cleanup.
  - Results are asserted from the `execution_report` / `position_snapshot` stream, so most checks need **no screen**.
- **Observability over a shell:** the plugin writes a redacted structured log file (e.g. `Documents\AllWin\logs\`). NT8's own logs and traces are under `Documents\NinjaTrader 8\log` and `\trace` [INF, EXP E2]. UI checks (window, pt-BR/en texts, warnings) need a screenshot or the user.
- **Two instances** (S19 cross-instance) need two Windows machines/VMs (§8).

### 13.4 Remote-access options (for Prumo and the user)

| Option | How it works | New inbound port? | Maestri | Can (re)start NT8 / see UI | Main risks / costs |
|---|---|---|---|---|---|
| **A. Inbound SSH** | OpenSSH Server on Windows, DefaultShell = Git Bash. Linux agents `ssh`/`scp` in to build, deploy, run `schtasks`, read logs. | **Yes**, port 22 (can be scoped to the dev host IP). Needs user authorization. | The Maestri app could open an SSH terminal on Windows; the CLI should work in Git Bash (C7–C9). **App support for a Windows host is unknown [EXP] E7.** | Via `schtasks /it` (E1); login per C2. | Exposed port. Admin key handling. |
| **B. Reverse SSH tunnel** (outbound from Windows) | Windows runs `ssh -N -R <port>:localhost:22 tunnel@devhost` as a scheduled task with retry. Linux agents connect to `localhost:<port>`. On the dev host, the `-R` listener binds to loopback by default (`GatewayPorts`) [OFF] ([ssh](https://man.openbsd.org/ssh), [sshd_config](https://man.openbsd.org/sshd_config)). | **No** new port on Windows if the `OpenSSH-Server-In-TCP` rule can stay disabled for loopback [EXP] E6. The dev host's existing SSH port is reused. | Same as A, through the tunnel. | Same as A. | The Windows box holds a dev-host key: restrict it to port-forwarding only (`restrict,port-forwarding,permitlisten=…` in `authorized_keys`). |
| **C. Agent on Windows** | Claude Code native on Windows, running **in the user's interactive session**. Coordination through Maestri (if the app supports Windows, E7) or Claude Code Remote Control (outbound HTTPS). | **No** | Maestri unknown (E7); Remote Control is not Maestri. | **Yes**: same session as NT8; can start NT8 directly. | Claude login on that machine. Unsandboxed on native Windows (WSL 2 is sandbox-capable but cannot drive the Windows desktop session) [OFF]. |
| **D. Self-hosted CI runner** | GitHub Actions runner on Windows, long-polling over outbound HTTPS 443; "no need for an inbound connection" [OFF] ([runners](https://docs.github.com/en/actions/reference/runners/self-hosted-runners), [GHES comms](https://docs.github.com/en/enterprise-server@3.13/actions/concepts/runners/communicating-with-self-hosted-runners)). Run interactively, not as a service, for UI work [OFF analog: [Azure agents](https://learn.microsoft.com/en-us/azure/devops/pipelines/agents/agents)]. | **No** | n/a (CI, not an interactive agent) | Yes if run interactively. | Needs the GitHub remote (A6) and pushes, which need user authorization. Slow feedback for a dev loop. GitHub warns on self-hosted runner security, even for private repos [OFF] ([secure use](https://docs.github.com/en/actions/reference/security/secure-use)). Good for **regression later**, not the inner loop. |
| **E. User runs scripts** | The team ships a script plus checklist; the user runs it and pastes the results. | No | n/a | User | Slows every plugin slice (PO Q15 option b). |

**Cross-cutting choices for the user** (all are security trade-offs; Prumo frames them):
1. Open port 22 (A), or outbound-only (B/C/D).
2. Windows **Autologon**: Sysinternals stores the password "as an LSA secret, [but] a user with administrative rights can easily retrieve and decrypt it" [OFF] ([Autologon](https://learn.microsoft.com/en-us/sysinternals/downloads/autologon)). It is needed only if the machine must survive reboots without the user.
3. NT8 auto-login via a third-party tool (stores the NinjaTrader password; conflicts with the "no secrets" rule unless the user accepts it), or the user types the password at each NT8 restart (C2).
4. Which NinjaTrader login the test machine uses. **Recommendation [INF]:**
   - a NinjaTrader login dedicated to development, ideally on the free license, which keeps Global Simulation Mode locked on;
   - with **no live broker connections configured** on that machine;
   - plus our plugin's own simulation guard (§14).

**WSL** can serve as the Linux-like shell for A/B/C: jq and tmux are installable, and it can call Windows tools such as `msbuild.exe`, `dotnet.exe` or `NinjaTrader.exe` as the active user [OFF] ([WSL filesystems](https://learn.microsoft.com/en-us/windows/wsl/filesystems)). In mirrored mode, WSL reaches Windows `127.0.0.1` listeners [OFF] ([WSL networking](https://learn.microsoft.com/en-us/windows/wsl/networking)). It does **not** solve C1: a GUI started from an SSH-originated session still lands in the wrong session [INF].

**Researcher's recommendation [INF], for Prumo's decision:**
- **B (reverse tunnel) + Git Bash shell** for agents: build, deploy, logs and scenario runs from Linux. It needs no new inbound port.
- **A one-time interactive `schtasks` task** to (re)start NT8.
- **The hot source loop (C4)** to avoid NT8 restarts and logins during development.
- **The fake-Core scenario harness** so most checks are screenless.
- Try **C** in parallel as a fallback if E1/E7 fail.
- Add **D** later for regression once the GitHub remote is authorized.

## 14. APIs and settings that can affect a live account or real money ⚠

| API / setting | Risk | Rule for agents |
|---|---|---|
| `Account.CreateOrder/Submit/Change/Cancel/CancelAllOrders/Flatten`, `Connection.CancelAllOrders()` | Act on any account in `Account.All`, live included | The plugin's non-release builds must refuse unless `account.Connection.Options.Mode == Mode.Simulation` **and** the provider is local simulation or Playback. Prop evaluation accounts on live connections may report `Mode.Live` [EXP]. The guard must fail closed. |
| `AtmStrategy.StartAtmStrategy` | Same | Not used (§7). |
| `Connection.Connect(ConnectOptions)` / `Disconnect()` | Could connect a configured live connection | Agents never configure live connections. Test code connects only `Playback` / `Simulated Data Feed` by name and then verifies `Mode`. |
| Global Simulation Mode | UI only; NinjaScript coverage unknown | Keep on. A free license keeps it locked [OFF]. Never rely on it alone. |
| Accounts tab > Lockout (8.1.8) | Real flatten/cancel on prop accounts | Not touched by agents. |
| NT8 login / Market Replay download | Uses the user's NinjaTrader credentials and data feed | Done by the user only. |

## 15. Minimal experiments for the NT8 spike (Plugin Engineer, Sim/Playback only)

| ID | Experiment | Answers |
|---|---|---|
| E1 | From an SSH session, `schtasks /run` a pre-created `/it` task that starts `NinjaTrader.exe`. Does the window appear on the logged-in desktop? Check the 8.1.8.4 login screen (password saved?). | C1, C2 |
| E2 | With the NinjaScript Editor open, copy a modified AddOn `.cs` into `bin\Custom\AddOns\AllWin\`.<br>• Does it auto-compile?<br>• Is the AddOn re-instantiated?<br>• Where do compile errors appear (logs readable over SSH?)<br>• Does the `AllWin.Adapter.Core` source compile under NT's Roslyn/.NET 4.8? | C3, C4, §4 |
| E3 | Build `AllWin.Adapter.Core` (`net48` target) + host DLL with `dotnet build`, then:<br>• copy to `bin\Custom` and restart → loads?<br>• package zip + `AdditionalReferences.txt` → Import on a clean NT8 → loads?<br>• repeat with the `netstandard2.0` build (facade needed?). | ADR-0001 §7 spike items, §10 |
| E4 | AddOn on Sim101 + Simulated Data Feed:<br>• enumerate `Account.All` with Mode/Provider;<br>• submit Market entry → OCO bracket;<br>• log the `OrderUpdate`/`ExecutionUpdate`/`PositionUpdate` sequence, including partial fills (larger qty) and `Operation.Remove`;<br>• `Order.Name` 50-char reference survives restart?<br>• check the `OrderEntry.Automated` flag. | §6, §7 |
| E5 | Playback at 1× and Max, after Go To:<br>• compare `Connection.PlaybackConnection.Now`, `Core.Globals.Now`, execution times;<br>• observe `SimulationAccountReset` on rewind. | S15 AC-7 |
| E6 | Reverse tunnel: is Windows `sshd` reachable on loopback with the `OpenSSH-Server-In-TCP` rule disabled? | Option B |
| E7 | Maestri app with a Windows OpenSSH host (Git Bash shell): can it open a terminal, forward `MAESTRI_HOST`, and run `maestri list`? | Options A–C |
| E8 | Reactive block timing (S15 AC-6): manual order on a "blocked" sim account → time to cancel/flatten. | S15 |
| E9 | WebSocket under load: keep a `ClientWebSocket` busy (heartbeats + bursts) while placing orders. Any UI freeze or connection drop? | §5 |
| E10 | Global Simulation Mode on, free license: does `Account.Submit` to a non-sim account get blocked from NinjaScript? (Only if a non-sim account is visible without trading. Otherwise skip; never create one.) | §14 |

## 16. Known pitfalls

- A compile error in **any** script under `bin\Custom` blocks compiling everything (`NinjaTrader.Custom.dll`) [OFF].
- `Order.OrderId` changes over an order's life; use object identity and our own `Name` reference [OFF].
- Event-args vs `Order` object state can differ within one callback (§5) [OFF].
- `ExecutionUpdate` can amend or remove executions [OFF].
- Locally simulated OCO and simulated stops need NT8 running (§7.3) [OFF].
- Playback rewinds reset the Playback account and fire `SimulationAccountReset` [OFF].
- Exports may not import into older NT8 versions [OFF].
- Do not edit the generated `NinjaTrader.Custom.csproj` [SEC].
- Shipping our own Newtonsoft can clash with NT's or other vendors' copies [SEC].
- Use `Dispatcher.InvokeAsync`, never `Invoke`, from event threads [OFF].

## 17. Open questions to route (via Maestro)

1. **Architect (Prumo):** pick the Q15 option set (§13.4) for the spike and the options to present to the user. Decide the JSON library policy (§4), the `netstandard2.0` vs multi-target choice (§4), and whether ADR-0004 should carry a "bracket integrity unknown" reconcile state (§7.3).
2. **PO:** S11/S15 wording for locally simulated OCO on NinjaTrader and Rithmic connections (§7.3); S19 needs two Windows machines/VMs for cross-instance tests (§8).
3. **User (through Maestro/Prumo):**
   - inbound port vs outbound-only access;
   - Autologon and NT8 auto-login trade-offs;
   - the dedicated NinjaTrader dev login;
   - ToS 5(a)(viii) and EULA reading (§11);
   - who downloads Market Replay data.
