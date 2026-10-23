# NinjaTrader 8 spike plan and Windows build/test access (Q15)

- **Status:** Proposed (2026-10-23). The user chooses the Windows access option (§5) before providing the machine.
- **Owner:** Software Architect (plan). Execution: Plugin Engineer, with the Plugin Researcher for platform facts and DevOps for the Windows host setup script.
- **Inputs:**
  - [ADR-0001 §7](../adr/0001-technology-stack.md) (plugin architecture);
  - [ADR-0004](../adr/0004-adapter-protocol-v1.md) (protocol, enforcement, pairing);
  - slice-plan V1-S2/S15/M1;
  - the user's conditions of 2026-10-23: firewall ports need explicit authorization; the build/test approach must be defined before the machine is provided; Maestri over SSH needs bash/curl and may not work on Windows.
- **Fact status:** platform facts marked **(unverified)** come from general knowledge and must be confirmed by the Plugin Researcher's platform profile (requested from "Trilha" on 2026-10-23, still pending) or by the spike itself. This plan will be revised when the profile arrives.

## 1. Goal and timebox

Retire the plugin risks before V1-S2 is implemented, and leave a working, repeatable build-deploy-test loop on the Windows machine. The spike code is throwaway, kept on its own branch (`s/v1-spike-nt8--plugin`) under `plugins/ninjatrader8/spike/` for reference, and is never merged as product code. Timebox: **3 agent-days after the machine is ready**. Running over the timebox is escalated to the Maestro.

## 2. Questions the spike must answer

Each question is answered **verified** (with evidence: log, screenshot or measured number) or **not feasible** (with the alternative).

| # | Question | Why it matters | Done when |
|---|---|---|---|
| K1 | Can an AddOn be built **outside** the NinjaScript Editor (MSBuild/`dotnet build`, `net48`, referencing `NinjaTrader.Core.dll`, `NinjaTrader.Gui.dll`, `NinjaTrader.Custom.dll` from the local install) and loaded from `Documents\NinjaTrader 8\bin\Custom`? Does reloading require an NinjaTrader restart? **(unverified: believed yes, restart required)** | The whole automated build/deploy loop depends on it | Scripted build → deploy → NinjaTrader start → AddOn menu visible, logged |
| K2 | Does NinjaTrader load our `netstandard2.0` library (`AllWin.Adapter.Core`) next to the AddOn without facade or binding errors? | ADR-0001 §7: Linux-testable core | AddOn calls into the library and logs its version |
| K3 | Which JSON library and version does NinjaTrader already load (Newtonsoft.Json?), and can we use it without a version conflict? **(unverified)** | Avoid assembly conflicts inside the host | Decision recorded: use the host's copy, or a dependency-free serializer |
| K4 | `ClientWebSocket` over `wss` (TLS 1.2+) from inside NinjaTrader to the dev Core or a test echo endpoint, with reconnect and backoff, without blocking NinjaTrader threads | ADR-0004 transport | 1 h soak with forced disconnects, no UI freeze, reconnects logged |
| K5 | Enumerate accounts (`Account.All`), detect the account kind (Sim101 / Playback101 / live) reliably, and subscribe to order, execution, position and account (P&L) events | Simulation guard, mapping, reports | Event log for a manual trade on Sim101 matches the expected sequence |
| K6 | Submit a market order, attach stop and target computed from the fill price (OCO), modify them (price delta), cancel, and flatten on Sim101. Orphan cancellation when flat | V1-S2, S9 | Scripted scenario passes 10/10 runs |
| K7 | Order `Name`/tag length and characters, to embed a short `command_id` reference. After an NinjaTrader restart, can working orders be re-associated with their `command_id`s? | ADR-0004 idempotency and reconciliation | Restart test with working orders recovers the associations, or a documented alternative |
| K8 | Reactive block: time from a manual order on a blocked account to its cancellation; behavior if it fills first | S15 AC-6 | Measured p50/p95 over 20 trials |
| K9 | Playback101 with downloaded Market Replay data for MNQ: platform time access, and trading-day reset driven by playback time | S15 AC-7, testability without a live feed | Reset observed at the simulated reset time |
| K10 | Durable local store (outbox, processed `command_id`s, cached limits) in the NinjaTrader user folder: survives crash and restart; DPAPI token storage | ADR-0004 §4, §8 | Kill NinjaTrader mid-command → restart → no duplicate execution |
| K11 | Threading model: which thread delivers events, how to marshal to the UI (WPF dispatcher), and how to keep socket I/O off NinjaTrader's critical threads | Stability inside the host | Design note plus no deadlock under the K4 soak |
| K12 | Latency: Core send → plugin receipt → order submitted → fill reported (Sim101), and Core relay round trip for master replication (A4) | A4 "measure latency"; baseline for TTL (T2) | Table of p50/p95 from this host to the dev Core |
| K13 | Packaging: NinjaScript "Export → compiled assembly" vs. a zip with DLLs; install/upgrade steps for the user; NinjaTrader version compatibility check at load | Distribution, "update required" path | Install guide draft; fresh install on a clean user profile works |
| K14 | One NinjaTrader instance per Windows user, or several? **(unverified: believed one per user)** | Multi-instance topology (vision §8) | Answer recorded |

## 3. Out of scope

- Production-quality plugin code, UI polish and localization.
- Core implementation: the spike may use a minimal echo/test endpoint or the `adapter-sim` once it exists.
- Any live account. Only Sim101 and Playback101 connections are configured on the test machine.

## 4. Deliverables

1. `plugins/ninjatrader8/docs/spike-report.md` (Plugin Engineer): K1–K14 answers with evidence, measured numbers, and recommended changes to ADR-0001 §7 / ADR-0004. If those changes are needed, the Architect issues versioned deltas.
2. The scripts that make up the **repeatable loop**, kept for V1-S2 onward:
   - `plugins/ninjatrader8/tools/` (Plugin Engineer): build, deploy, NinjaTrader start/stop, log and screenshot collection;
   - `infra/windows-test-host/` (DevOps): host setup.
3. Draft install guide for the user (input to S3 "Como instalar o plugin").

## 5. How agents build and test on the Windows machine (Q15): options

**Constraint that shapes every option:** NinjaTrader 8 is a WPF desktop application that needs an **interactive, logged-on Windows desktop session** **(unverified, highly likely)**. Processes started directly from a remote shell (SSH) or from a Windows service run without a visible desktop. So every option needs a way to start NinjaTrader *inside* the interactive session, for example a Scheduled Task set to "Run only when user is logged on", triggered remotely, plus an always-logged-on test user (auto-logon, or a disconnected RDP session).

**Recommended isolation, whatever the option:**

- A **dedicated Windows machine or VM**, or at least a **dedicated standard (non-admin) Windows user** `allwin-agent`.
- That user has its own NinjaTrader 8 profile, with only Sim/Playback connections and **no live brokerage credentials**.
- Agents must never be able to reach the user's personal NinjaTrader profile or live accounts.

| Option | How it works | Pros | Cons | User installs | Security implications |
|---|---|---|---|---|---|
| **O1. OpenSSH Server + PowerShell, over a private network** | Agents stay on the Linux dev server (in Maestri as today) and run `ssh win-nt8 'pwsh -File tools\…'` to build (MSBuild/`dotnet`), deploy the DLLs, start or stop NinjaTrader through Scheduled Tasks in the interactive session, run scenarios against the dev Core, and fetch logs and screenshots | Fast iteration (seconds to minutes); fully scriptable; no AI agent installed on Windows; Maestri unchanged, so the bash/curl limitation does not apply; Microsoft-supported components | Needs one inbound path to Windows sshd: **requires user authorization**. Visual checks rely on scripted screenshots | OpenSSH Server (Windows optional feature); VS 2022 Build Tools (MSBuild + .NET Framework 4.8 targeting pack); .NET 8 SDK; NinjaTrader 8 + free sim login + MNQ Market Replay data; auto-logon for `allwin-agent`; Tailscale or WireGuard on Windows and the dev server; our setup script (reviewed by the user before running) | Key-only auth, `AllowUsers allwin-agent`, password login disabled. sshd bound to the **private overlay interface only**: no public port, no router port-forward. Standard user, no admin. Revocable by removing the key or logging the device out of the overlay network. Full command audit in the sshd log |
| O2. WSL2 on the Windows machine | A Linux distro inside Windows. An agent (or an SSH target for Maestri) runs in WSL with bash/curl and calls Windows tools through interop (`dotnet.exe`, NinjaTrader). Interop processes start in the user's interactive session | bash/curl available, so Maestri over SSH could attach; interop solves the GUI-session problem when WSL runs inside the logged-on session | Still needs an inbound path (sshd in WSL plus port proxy, or an overlay network). More moving parts (WSL networking, interop edge cases). If an agent seat runs there, the user's Claude account has to be set up on that machine | WSL2 + distro; the O1 build tools; optionally Claude Code inside WSL | Same network controls as O1. WSL shares the Windows user's files (`/mnt/c`), so it **must** run under the dedicated `allwin-agent` user, not the user's personal one |
| O3. Self-hosted GitHub Actions runner on Windows | The runner connects **outbound** to GitHub. Agents push a branch; a workflow builds, deploys to NinjaTrader, runs the scenarios and uploads logs and screenshots as artifacts | **No inbound port at all**; reproducible, audited runs; doubles as a regression gate later | **Every iteration needs a push**, and push currently needs user authorization each time (A6), so the user must grant a standing authorization for spike/feature branches. Slow loop (minutes). The runner must run interactively, not as a service, to launch NinjaTrader. Depends on the GitHub remote existing | GitHub runner (interactive mode, `allwin-agent` user), the O1 build tools, NinjaTrader 8 | Outbound HTTPS only. Private repository only (a public repo would let outsiders run code on the runner). The runner executes repository code, so branch protection and review matter |
| O4. Native Windows agent (Claude Code on Windows, Git Bash) | A Plugin Engineer agent runs directly on Windows under `allwin-agent` and talks to the team by files/git, or by the Maestri CLI if its server is reachable and Git Bash's bash/curl suffice **(unverified with Maestri)** | Agent sees the real desktop; no SSH needed | Maestri integration uncertain (user's own concern). Separate agent seat and login on that machine. Harder to supervise from the canvas | Claude Code for Windows, Git for Windows, the O1 build tools, NinjaTrader 8 | An autonomous agent with a shell on a Windows machine. Acceptable only on a dedicated machine or VM with no personal data or live credentials |
| O5. Manual, user-run script | The team publishes a package (DLLs + `Run-AllWinChecks.ps1`). The user runs it; it deploys, starts NinjaTrader, walks a checklist, and zips logs and screenshots for the user to hand back | **No remote access, no ports, no agent on Windows**; simplest to approve | Every iteration costs the user's time and a round trip: the critical-path bottleneck the plan warned about (Q15-b). Little exploratory debugging | NinjaTrader 8 + replay data; PowerShell execution policy allowing the signed or unblocked script | Minimal attack surface. The user reviews each package before running it |

### Recommendation

**O1** (OpenSSH + PowerShell over a private overlay network such as Tailscale), on a **dedicated Windows VM or machine with a dedicated standard user `allwin-agent`**:

- The **Linux dev server initiates** the connection through the overlay network. No public firewall port and no router port-forward are needed.
- The overlay network and the sshd rule limited to its interface **still require the user's explicit authorization**, which we request as one consolidated change.
- The bash/curl limitation of Maestri over SSH **does not apply**: no Maestri terminal runs on Windows, and Windows is just an SSH target used by the Plugin Engineer from Linux.
- NinjaTrader runs in the auto-logged-on `allwin-agent` desktop, started through Scheduled Tasks that the setup script creates. Screenshots are captured the same way.

**Fallbacks and complements:**

- **O5 for the very first spike session**, if the user prefers to start with zero remote access. It is the same scripts, run by hand.
- **O3 later** as a nightly or PR regression gate on the same machine, once the GitHub remote exists and the user grants standing push authorization for non-`main` branches.

O2 and O4 are not recommended for V1: more setup and an agent seat on the Windows machine, with no advantage over O1 for our team layout.

**Hosting note:** a small cloud Windows VM in the **US central region** (same region as production, A2) is a valid alternative to a machine at home. It gives full isolation from personal data, and its latency to the Core is representative of the production setup. It has a monthly cost. NinjaTrader's support for Windows Server editions must be checked **(unverified)**; a Windows 10/11 VM avoids the question.

### What the user does for O1 (checklist, about 1–2 h, guided by a script the user reviews first)

1. Provide a Windows 10/11 64-bit machine or VM (≥ 4 vCPU, 8 GB RAM, 60 GB disk). Create the local standard user `allwin-agent`.
2. Install, as admin once: NinjaTrader 8 (latest), VS 2022 Build Tools (MSBuild + .NET Framework 4.8 targeting pack), .NET 8 SDK, OpenSSH Server (optional feature), Tailscale or WireGuard.
3. In `allwin-agent`: log into NinjaTrader with a free account; keep only Sim101/Playback connections; download MNQ Market Replay data for the test dates; enable auto-logon for this user.
4. **Authorize** the network change: join the machine to the private overlay network, and allow sshd only on that interface, only for `allwin-agent`, with key-only auth (we provide the public key).
5. Run `infra/windows-test-host/Install-AllWinTestHost.ps1` (DevOps). It creates the folders, the Scheduled Tasks (start/stop NinjaTrader, screenshot) and the log locations, and changes no firewall rule beyond step 4.

**Never on that machine:** live brokerage or prop-firm credentials, the user's personal NinjaTrader profile, or admin rights for `allwin-agent`.

## 6. Spike sequence (after the machine is ready)

1. **Day 1:** K1, K2, K3, K11 (build/deploy loop, library load, JSON, threading), K13 basics, K14.
2. **Day 2:** K4, K5, K6, K7, K10 (socket soak, accounts and events, orders, Name limits and restart recovery, durable store).
3. **Day 3:** K8, K9, K12 (reactive block timings, Playback reset, latency), then the report.

Exit gate: the report is reviewed by the Architect and QA; ADR deltas, if any, are issued; V1-S2 starts.

## 7. Decisions needed from the user

| # | Decision | Recommendation |
|---|---|---|
| W1 | Access option for agents on the Windows machine | **O1** over a private overlay network; O5 acceptable for the first session |
| W2 | Machine: a dedicated VM or machine, or a dedicated user on an existing machine | **Dedicated VM/machine** (cloud Windows 10/11 VM in US central, or local). If it is an existing personal machine, at minimum a dedicated standard user with no live credentials |
| W3 | Authorize the overlay network + sshd restricted to it (only if O1/O2) | Authorize, as one consolidated change, when the machine is ready |
| W4 | Standing push authorization for non-`main` branches (only if O3 is adopted later) | Decide later; not needed for O1 |
