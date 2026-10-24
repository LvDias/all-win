# NinjaTrader 8 spike plan and Windows build/test access (Q15)

- **Status:** rev. 7.1 (2026-10-24).
  - **rev. 7.1, review notes on rev. 7 (Crivo, Vigia, Pêndulo):** development-only switches compiled out of every other build, with a packaging check (§8, E12); `net48` and `netstandard2.0` from the same source (§8); package and fake-Core hashes checked by the user, explicit loopback bind (§8); "Export diagnostics" by allowlist with a token scan (§8, ADR-0005 N12); NT8's own logs checked for live connections before return; optional user-only round RX for Rithmic plug-in mode; K1/K2 rows and §6.4 updated for manual rounds.
  - **rev. 7, user decision 2026-10-24: tests inside NinjaTrader 8 are MANUAL, run by the user over RDP.** The VPS stays as it is: Tailscale, the tailnet policy, Tailnet Lock and RDP restricted to the tailnet. Agents get **no access** to the VPS: `sshd` is not started, `Install-AllWinTestHost.ps1` is not run, and `Lock-Firewall.ps1` is set aside. The dev server stays in the tailnet; only the dev → VPS:22 rule goes. What this cancels and what stays is in §8.
  - rev. 6, from the actual state: the user already installed Tailscale on the VPS and the Mac, and the VPS joined **untagged** through the user's login. The checklist now:
    - applies the policy first;
    - tags the VPS `tag:allwin-nt8` and renames it from the admin console, with **no auth key at all**;
    - then turns on Tailnet Lock and signs the existing nodes (§6.3, §6.5 steps 3–4).
    Review notes added:
    - QA Security I1: the login URL goes to the user only;
    - QA Security I2: the VPS host key is pinned;
    - QA Security N1: `sshd` is never started before step 10;
    - Code Review n1: the firewall check covers UDP 3389, Any-port rules, every profile and the interface;
    - Code Review n2: the user verifies a node before approving or signing it.
  - rev. 5 (QA Security, Phase 2 review):
    - H1: OpenSSH Server is installed **before** `Lock-Firewall.ps1`, and `sshd` starts only after the firewall is verified (§6.5 steps 5–6, 10).
    - M1: the dev server joins by interactive login, and the user signs its node key on the Mac. No `tag:allwin-dev` auth key exists, so none passes through an agent (§6.3, §6.5 step 8).
  - User decisions: W1 reverse tunnel accepted, W3 accepted, W2 changed to a Contabo Windows VPS without GPU, and remote access to go over Tailscale with RDP never exposed to the internet.
  - rev. 3 revised W1 for Tailscale (§6.2) and gave the setup checklist (§6.5).
  - rev. 4: the user approved **T1 with Tailnet Lock on**, including the dev server joining the tailnet (U1, 2026-10-23). §6.1 gives the final machine configuration (U2), checked against official sources on 2026-10-23.
- **Owner:** Software Architect (plan). Execution:
  - Plugin Engineer runs the spike.
  - Plugin Researcher (Trilha) supplies platform facts.
  - DevOps provides the Windows host setup script and the dev-host tunnel account.
- **Inputs:**
  - [NT8 platform profile](../platforms/ninjatrader8-profile.md): facts, §13 Q15 options, §15 experiments E1–E10. **Facts below cite it; this plan does not repeat its sources.**
  - [ADR-0001 §7](../adr/0001-technology-stack.md) and [ADR-0004](../adr/0004-adapter-protocol-v1.md) (including §6.1 bracket integrity).
  - [ADR-0005](../adr/0005-nt8-plugin-implementation.md) (plugin implementation decisions).
  - User conditions of 2026-10-23: no port opened without authorization; agents use simulation only.
- **rev. 2 changes:**
  - Facts reconciled with the profile.
  - rev. 2 merged my option O1 with Trilha's option B (reverse SSH tunnel); the user accepted B. rev. 3 recommended T1 (Tailscale for RDP and agent SSH), after the user's Tailscale proposal for RDP; **the user approved T1 with Tailnet Lock (U1, rev. 4, §6.2)**. B stays documented only as the fallback if Tailscale is ever dropped.
  - Spike questions mapped to experiments E1–E10.

## 1. Goal and timebox

Retire the plugin risks before V1-S2 is implemented, through **manual test rounds** that the user runs over RDP (§8): the team delivers an import package, a numbered script and the list of logs to return.

- **Spike code is throwaway.** Branch `s/v1-spike-nt8--plugin`, under `plugins/ninjatrader8/spike/`; never merged as product code.
- **Timebox:** 3 agent-days after the machine is ready. Going over is escalated to the Maestro.

## 2. Facts that shape the plan (from the profile)

| # | Fact | Consequence |
|---|---|---|
| F2 | NT8 **8.1.8.4**, .NET Framework 4.8, Roslyn with **C# 13**, ships **Newtonsoft.Json 13** | `net48` host confirmed; never bundle our own Newtonsoft (ADR-0005) |
| F3 / C3 | A DLL built outside NT8 loads from `Documents\NinjaTrader 8\bin\Custom` **only after an NT8 restart**. There is no headless compile | Restarts are expensive (next row) |
| C2 | **No command-line login.** The 8.1 login screen does not save the password | Every NT8 restart needs the user, or an auto-login tool that stores the password (W3) |
| C4 | **Source files saved while the NinjaScript Editor is open are auto-compiled, with no restart** | Development uses a **hot source loop**; restarts only when the packaged DLL is tested |
| C1 | NT8 is a WPF/DirectX desktop app. It must run in a **logged-on interactive session**; processes started by `sshd` run in session 0 | NT8 is started by a pre-created Scheduled Task with `/it`, triggered with `schtasks /run` (E1) |
| F6 | **Simulated Data Feed** = fully offline testing on Sim101. Playback needs all other connections closed | Two test sessions: Sim + Simulated Data Feed, and Playback |
| F7 | **One NT8 instance per computer** | Cross-instance tests (S19) need **two Windows machines/VMs** |
| §2 | Requirements: Windows 11 or **Windows Server 2016+** (officially supported); DirectX 10 recommended. Trilha confirmed a GPU is **not indispensable** for Sim/Playback build and test; software rendering (WARP) is acceptable, though staff report crashes on some GPU-less VPS | **W2 = Contabo Windows Server 2025 VPS without GPU (§6.1).** Plan B is a vGPU instance if E1/E3/E9 fail for graphics reasons (§6.1) |
| F5 | On NinjaTrader/Rithmic connections, **OCO is simulated locally** | Bracket-integrity tests in the spike (ADR-0004 §6.1) |

## 3. Questions the spike answers (mapped to the profile's experiments)

| # | Question | Experiment | Done when |
|---|---|---|---|
| K1 | ~~Start NT8 remotely~~ **Cancelled (rev. 7):** the user starts NT8 and logs in | ~~E1~~ | — (cancelled) |
| K2 | Source import (rev. 7.1, manual): does a source-form package compile and load on import without a restart, where do compile errors appear, does the host source compile under NT's compiler | **E2** | The user imports the round's source package: it compiles and loads without a restart, or the compile errors are captured from the NinjaScript output and the NT8 log the user returns |
| K3 | Packaged build: `net48` multi-target of `AllWin.Adapter.Core` + host DLL, copy and restart, then zip + `AdditionalReferences.txt` import on a clean profile | **E3** | Both load; import on a clean profile works |
| K4 | Accounts with Mode/Provider (simulation guard, `oco_location`), market entry → OCO bracket from the fill, event sequence including partial fills and `Operation.Remove`, `Order.Name` reference surviving a restart, `OrderEntry.Automated` | **E4** | Scripted scenario passes 10/10; the restart keeps the `Name` association |
| K5 | Playback clock: `PlaybackConnection.Now` vs `Core.Globals.Now`; `SimulationAccountReset` on rewind | **E5** | Clock source chosen for S15; rewind handling documented |
| K6 | ~~Agent access path~~ **Cancelled (rev. 7):** no agent access. Only the user's own check stays: RDP is reachable only over the tailnet (§6.5) | — | RDP not reachable from the internet |
| K7 | ~~Maestri app with a Windows host~~ **Cancelled (rev. 7)** | — | — |
| K8 | Reactive block timing | **E8** | p50/p95 over 20 trials |
| K9 | WebSocket under load beside order traffic: UI freeze or broker disconnect? | **E9** | 1 h soak, no freeze or drop |
| K10 | Does Global Simulation Mode block NinjaScript `Account.Submit` to non-sim accounts? | **E10** | Only if safely testable; otherwise skipped. Never create a live account |
| K11 | **Bracket integrity (new):** close NT8 with a working bracket on Sim101, move price through the stop with the Simulated Data Feed on restart, and observe the orphan leg, `Unknown` states and reconnect detection | new **E11** | ADR-0004 §6.1 cases reproduced, or documented as not reproducible on Sim |
| K12 | Durable local store (outbox, processed `command_id`s, cached limits) survives a crash; DPAPI token storage | new **E12** | Kill NT8 mid-command → restart → no duplicate execution |
| K13 | Latency: Core send → submit → fill report on Sim101, plus the Core relay round trip (A4) | new **E13** | p50/p95 table |
| K14 | **Commissions (U20, net P&L):**<br>• `Execution.Commission` with and without a commission template on Sim101 and on Playback;<br>• whether it is filled at the execution event or later;<br>• what `AccountItem.Commission`, `Fee`, `RealizedProfitLoss` and `GrossRealizedProfitLoss` contain (net or gross; fees included?) | new **E14** | Table per connection type. If commission arrives late, measure the delay and the size: the protocol reports a late commission as absent and the floor applies (ADR-0004 §7). Note the effect (QA Security I1): if the real commission exceeds the floor, the day's net realized P&L is understated by the difference; the drawdown stays conservative. If the difference is relevant, the Core reconciles the day with the platform balance at the close |
| K15 | Executions after a restart (also E4): are the day's earlier executions readable again, so the plugin can rebuild the day's commission adjustment? | **E4** (extended) | Documented; if not readable, the plugin keeps the day's adjustment in its journal (ADR-0004 §7) |

## 4. Out of scope

- Product-quality code, UI polish, localization.
- Core implementation: the spike uses a **scripted fake Core** (profile §13.3), the mirror of `adapter-sim`.
- Any live account, live connection, Market Replay download, or NinjaTrader login. Those are done by the user only (profile §14).

## 5. Deliverables

1. `plugins/ninjatrader8/docs/spike-report.md` (Plugin Engineer): K1–K13 with evidence and measured numbers, plus proposed ADR deltas. If deltas are needed, the Architect issues them.
2. **Per test round (rev. 7, §8):** the import package, the numbered script and the log-collection script. The former agent-side loop scripts (`plugins/ninjatrader8/tools/` start/stop/deploy over SSH, `infra/windows-test-host/`) no longer apply.
3. Fake-Core scenario harness: entry → fill → bracket, partial fills, reject, disconnect/reconnect, expiry, flatten/block, orphan cleanup, bracket integrity.
4. Draft install guide for the user (S3 "Como instalar o plugin").

## 6. Windows test machine and access (W1–W4)

**Fixed in every option:**

- A dedicated test VPS with a dedicated **standard (non-admin) Windows user** `allwin-agent`.
- A **dedicated NinjaTrader development login**, ideally on the free license (Global Simulation Mode stays locked on).
- **No live broker connections** and no personal data on the VPS.
- The plugin's own simulation guard, which fails closed (ADR-0005 N4).

### 6.1 W2 — Machine (decided: Contabo, no GPU; final configuration U2)

**Final configuration (to contract):**

| Item | Value |
|---|---|
| Product | Contabo **Cloud VPS 8**: 8 vCPU, 24 GB RAM, 300 GB SSD. Optional upgrade: Cloud VPS **Plus** 8 (same CPU/RAM, 450 GB NVMe, AMD EPYC) if faster builds are worth the price |
| OS | **Windows Server 2025** (Contabo license add-on), installed **with Desktop Experience** (NT8 is a WPF desktop app; Server Core cannot run it). Edition (Standard or Datacenter) is whatever Contabo supplies; either works. Fallback: Windows Server 2022 if the 2025 image is not offered at checkout |
| Region | **US Central (St. Louis)**, consistent with A2 |
| GPU | None (software rendering, WARP; plan B below) |
| License | Only the Contabo Windows Server add-on. Bring-your-own-license is not allowed on Contabo VPS, and Windows 10/11 images are not offered |
| RDP | **No RDS CALs and no RD Session Host role.** Windows Server allows two concurrent Remote Desktop for Administration sessions without CALs, which covers our use: one single-person test host, Administrator plus `allwin-agent`, never more than two sessions. Agents use SSH, not RDP |
| NT8 prerequisites | .NET Framework 4.8 (built into Server 2022/2025); NT8 8.1.8.4 or later; RDP resolution at least 1024×768 (use 1920×1080 so screenshots are readable) |

**Why these choices:**
- **Server, not Windows 10/11:** the official NT8 minimum requirements list "Windows 11, Windows Server 2016 or later 64-bit", ".NET Framework 4.8", "DirectX10 compatible graphics card highly recommended", 1024×768, 2 GB RAM (8 GB and a quad core recommended). Windows 10 is no longer listed. Contabo offers only Windows Server on VPS, so Windows 11 is not an option there anyway.
- **2025 rather than 2022:** both are within "Server 2016 or later". Windows Server 2022 left Microsoft mainstream support on 2026-10-13 (security updates continue until 2031-10-14). Windows Server 2025 is in mainstream support until 2029-11-13. Microsoft lists each end as 06:59:59 PT of the following day; the dates here are the last supported day (Patch Tuesday). Experiment E1 still verifies that NT8 starts and renders in the session; if it fails on 2025 and not for graphics reasons, reinstall with the 2022 image (a Contabo reinstall, same license add-on) before considering plan B.
- **Sizing:** NT8 uses all cores and memory grows with them. The host also runs MSBuild, the .NET SDK test suite and Market Replay data, so 8 vCPU / 24 GB leaves headroom; Cloud VPS 6 (12 GB) is below the plan's 16 GB floor.

**Sources (checked 2026-10-23):**
- NinjaTrader, [minimum system requirements](https://static.ninjatrader.com/support/helpGuides/nt8/minimum_system_requirements.htm) (official help guide; the page says nothing about VPS or RDP).
- Contabo docs: [VPS plans and add-ons](https://docs.contabo.com/docs/servers-hosting/vps/) ("Windows Server" add-on, "Contabo-provided license"; "bring-your-own-license is not permitted on VPS"); [images](https://docs.contabo.com/docs/servers-hosting/images/) (Windows Server "2016, 2019, 2022, 2025"; no Windows 10/11); [own Windows license](https://help.contabo.com/en/support/solutions/articles/103000270398-can-i-use-my-own-windows-license-on-my-contabo-server-) (license only via the add-on; adding it needs a reinstall).
- Microsoft lifecycle: [Windows Server 2022](https://learn.microsoft.com/en-us/lifecycle/products/windows-server-2022), [Windows Server 2025](https://learn.microsoft.com/en-us/lifecycle/products/windows-server-2025).
- Microsoft, [Remote Desktop for Administration](https://learn.microsoft.com/en-us/previous-versions/windows/it-pro/windows-server-2008-R2-and-2008/cc742816(v=ws.11)): two concurrent connections, no RDS CALs (the current docs for 2022/2025 keep the same rule but I found no newer page; this is the only secondary-strength point).
- Not verified: the monthly prices (the Contabo order pages block automated access). The user sees them at checkout.
- **No GPU:** NT8 renders with software (WARP). Charts may be slower, which does not matter for an AddOn tested through events and logs.
- **Plan B (vGPU):** move to a GPU-capable Windows instance with another provider, in the same region if possible. Trigger: E1 (NT8 window does not start or render in the session), E3 (package fails to load for graphics reasons) or E9 (UI freezes or crashes) fail because of graphics.
- **S19 cross-instance tests** will need a second VPS later (one NT8 per computer).

### 6.2 W1 — Access: Tailscale for RDP and agent SSH (revised recommendation)

The user's rule: **RDP is never open to the internet.** The user's home IP changes, so IP allow-lists do not work. The user proposed a Tailscale tailnet joining the user's Mac, the Linux dev server and the Windows VPS. Since the VPS must be on the tailnet for RDP anyway, the question is how agents reach its `sshd`.

| Option | Agent SSH path | Pros | Cons / risks |
|---|---|---|---|
| **T1. Everything over the tailnet (recommended)** | Linux dev server → VPS `sshd` over Tailscale (`ssh allwin-agent@allwin-nt8-01`). No reverse tunnel | One mechanism for RDP and SSH. **The VPS holds no credential for the dev server**: in the reverse-tunnel design it held a forwarding key to it. The ACL denies every connection from the VPS to the dev server. Survives IP changes. No public port anywhere | Adds the Tailscale daemon to the **dev server** (a new network interface on a host where agents have sudo). Mitigated by tags, a default-deny ACL and an outbound-only role (below). The tailnet admin account becomes a root of trust |
| T2. RDP over Tailscale (Mac ↔ VPS only); agents keep the reverse SSH tunnel | Windows → dev host `ssh -R` (W1 as accepted) | The dev server stays out of the tailnet | Two mechanisms to operate. The VPS keeps a key to the dev server, restricted to forwarding but still a path toward the dev host |
| T3. No Tailscale: self-hosted WireGuard hub on the dev server | WireGuard | No third-party control plane | The dev server needs an inbound UDP port (requires user approval) and manual key management on the Mac. More operational work for no security gain at this scale |

**Decision: T1, with Tailnet Lock on** (approved by the user on 2026-10-23, U1, including the dev server joining the tailnet).

There is no reason to avoid Tailscale here. Its one structural trade-off is trusting Tailscale's coordination server (control plane) and the user's tailnet admin login. Traffic between nodes is end-to-end encrypted with WireGuard. The control-plane trust is reduced with **Tailnet Lock** (required by the user's decision): new nodes must be signed by a trusted node. If the user does not want any third-party control plane, T3 (or Headscale, self-hosted) is the alternative.

### 6.3 Tailnet design (for T1)

- **Nodes and tags:**
  - the user's Mac (owned by the user, untagged);
  - the dev server: `tag:allwin-dev`;
  - the VPS: `tag:allwin-nt8`.
  - Tagged nodes do not expire with a user key, so node-key expiry does not cut agents off. Each tag is owned by the admin.
- **ACL policy** (default deny; DevOps keeps it versioned in `infra/tailscale/policy.hujson`, the user applies it in the admin console):
  - `autogroup:member` (the user's devices) → `tag:allwin-nt8`: TCP 3389 (RDP) and 22.
  - `tag:allwin-dev` → `tag:allwin-nt8`: TCP 22 only.
  - `tag:allwin-nt8` → anything: **nothing**. The VPS cannot open connections to the dev server or the Mac over the tailnet.
  - Nothing → `tag:allwin-dev`. The dev server is outbound-only in the tailnet; the user keeps using the normal SSH path to it.
  - No exit nodes, no subnet routes, and Tailscale SSH stays off: OpenSSH with keys remains the audited path.
- **No auth keys (rev. 6; M1):**
  - The VPS already joined through the user's interactive login, untagged. The user tags it from the admin console (*Machines → Edit ACL tags*, `tag:allwin-nt8`) and renames it `allwin-nt8-01`. The user then checks there that key expiry is disabled for it, and disables it if not.
  - The dev server also joins by **interactive login** (§6.5 step 8), so no reusable credential passes through an agent or its terminal:
    - DevOps runs `tailscale up` without a key;
    - the printed login URL works as a one-time capability, so DevOps sends it **only to the user** (QA Security I1);
    - the user approves it as the tag owner, and signs the new node from the Mac.
- **Tailnet Lock (on):**
  - The user's Mac is the trusted signing node; add a second signing node or keep the disablement secrets offline (password manager), as Tailscale requires, so losing the Mac does not lock the tailnet.
  - Tailnet Lock is turned on **after** the policy is applied and the VPS is tagged (§6.5 step 4).
  - Any node that `tailscale lock status` then reports as unsigned (the VPS, and later the dev server) is signed from the Mac with `tailscale lock sign <node-key>`. A node key is public, not a secret.
  - **Before approving a login URL or signing a node key,** the user checks in the admin console that the pending node's hostname, OS and node key are the expected machine's (Code Review n2). Otherwise an agent could relay another machine's URL or key.
  - A node that is not signed cannot talk to the others: the T-checks below include (f).
- **DNS:**
  - **MagicDNS** on, for names such as `allwin-nt8-01`.
  - The dev server runs Tailscale with `--accept-dns=false`, so its system resolver is not changed; it reaches the VPS by its Tailscale IP or a static hosts entry managed by DevOps.
  - The VPS and the Mac may use MagicDNS.
- **Dev server specifics:** `tailscale up --advertise-tags=tag:allwin-dev --accept-dns=false --accept-routes=false`, with no `--auth-key`. The user approves the login URL while signed in as the tag owner (admin), so the tag applies, and then confirms the tailnet, the tag and the signature in `tailscale status` / `tailscale lock status` (QA Security I1). No ufw change is needed: the dev server only initiates connections, and the ACL blocks inbound tailnet traffic to it. Installing Tailscale on the dev server is a network change; the user **approved** it on 2026-10-23 (U1). Any other SSH/firewall/exposure change still needs its own approval.
- **Windows firewall on the VPS:**
  - Disable the default public inbound rules for Remote Desktop and `OpenSSH-Server-In-TCP`.
  - Add inbound allow rules for TCP 3389 and 22, scoped to remote addresses **100.64.0.0/10** (the Tailscale range) on the Tailscale interface only.
  - Require NLA for RDP.
  - `sshd`: `AllowUsers allwin-agent`, `PasswordAuthentication no`, `DefaultShell` = Git Bash.
  - `sshd` keeps listening on all addresses; the firewall enforces the scope. Binding it to the Tailscale IP is fragile at boot because Tailscale may come up after `sshd`.
- **Contabo control panel:**
  - Enable 2FA on the Contabo account.
  - Store the Administrator password in the user's password manager.
  - The panel's **VNC console** is the break-glass path if Tailscale or the firewall locks everyone out. DevOps confirms that Contabo offers it for Windows VPS (unverified).
  - Contabo's network-level firewall, if offered for this product, can additionally block all inbound except Tailscale's UDP (unverified, optional).
- **Initial exposure window:** a new Contabo Windows VPS usually starts with RDP reachable on its public IP. To keep "never open to the internet" true, the first setup is done through the **VNC console**. If VNC is unavailable, the user does steps 3–6 of the checklist in one short first RDP session with the strong initial password, and then confirms with the T-checks below that 3389 is closed publicly. (rev. 6) The VPS is already running, so this window is open **now**: steps 3–6 are done in one sitting, through VNC if available.
- **Production stays out:** the future production server must **not** join this tailnet without a separate decision.

### 6.4 W3 — NT8 restarts (decided)

- The user types the NT8 password when NT8 must restart. From rev. 7 **every** start and restart of NT8 is done by the user; there is no remote start.
- Windows auto-logon stays off.
- ~~The `allwin-agent` desktop session stays alive by **disconnecting** RDP rather than signing out, so `schtasks /it` can start NT8 in it (E1 verifies on Server 2025).~~ Obsolete from rev. 7: there is no remote start (§8).

### 6.5 W4 — Setup checklist

**User** (about 1–2 h; DevOps scripts are reviewed by the user before running):

1. **Contract** the Contabo VPS (§6.1). Enable 2FA on the Contabo account. Save the Administrator password in the password manager.
2. **Open the VNC console** in the Contabo panel and sign in as Administrator. Use the first RDP session only if VNC is unavailable, and keep it short.
3. **Tailscale admin** (create the account if needed; enable 2FA on its login):
   1. apply `infra/tailscale/policy.hujson` (prepared by DevOps), which makes the admin the owner of both tags;
   2. the VPS is already in the tailnet, untagged (rev. 6): in *Machines*, set its ACL tag to `tag:allwin-nt8`, rename it `allwin-nt8-01`, and check that key expiry is disabled for it. If that login was done in a browser on the VPS, sign out of the identity provider there and clear its session (Google, GitHub, etc.), so it does not stay in the Administrator profile;
   3. the Mac stays the user's own, untagged device.
   **No auth key is generated** (rev. 6; M1).
4. **Mac: turn on Tailnet Lock**, with the Mac as the trusted signing node:
   - store the disablement secrets in the password manager, and add a second signing node or keep the secrets offline (§6.3);
   - run `tailscale lock status`, and sign every node it reports as unsigned (today the VPS) with `tailscale lock sign <node-key>`, after the admin-console check of §6.3 (Code Review n2);
   - confirm that the VPS shows as signed.
5. **VPS:** install the **OpenSSH Server** capability, and **never start `sshd` before step 10**, not even once to generate its files (QA Security N1). The install creates the rule `OpenSSH-Server-In-TCP`, enabled for any remote address, and the default `sshd_config` accepts passwords. So the capability must exist before step 6 disables that rule (QA Security H1).
6. **VPS:** run `infra/windows-test-host/Lock-Firewall.ps1` (DevOps). It:
   - disables the public RDP rules and `OpenSSH-Server-In-TCP`;
   - adds the tailnet-scoped rules;
   - enforces NLA.
   It is idempotent and ends with a check that fails loudly when an **enabled ALLOW** inbound rule, in any profile, meets both conditions (Code Review n1):
   - it covers TCP or UDP 3389 (Windows ships "Remote Desktop - User Mode (UDP-In)") or TCP 22, including rules whose local port is Any or a range;
   - it allows a remote address outside 100.64.0.0/10 or an interface other than Tailscale's.
7. **Mac:** Tailscale is already installed and signed in. From now on, RDP only to `allwin-nt8-01`.
8. **Dev server**, approved by the user on 2026-10-23 (U1):
   - DevOps runs `tailscale up` with the §6.3 flags and **no auth key**.
   - The user opens the printed login URL on the Mac and approves it as the admin, after the admin-console check of §6.3 (Code Review n2).
   - DevOps reads the node key from `tailscale lock status` and sends it to the user.
   - The user runs `tailscale lock sign <node-key>` on the Mac (M1).
9. **VPS, as Administrator:** create the standard user `allwin-agent` and add it to *Remote Desktop Users*. Install:
   - NinjaTrader 8 (8.1.8.4 or later);
   - .NET SDK 8+;
   - Build Tools for Visual Studio (MSBuild + .NET Framework 4.8 targeting pack);
   - Git for Windows.
10. **VPS:** run `infra/windows-test-host/Install-AllWinTestHost.ps1` (DevOps). In this order, it:
    - creates `sshd_config` from `sshd_config_default` and generates the host keys (`ssh-keygen -A`) **without starting the service**. The keys go to `%ProgramData%\ssh` with the ACLs `sshd` requires (SYSTEM and Administrators only), and the script checks those ACLs. Otherwise `sshd` refuses to start, which fails closed but stops the setup (Code Review note b, to be confirmed by DevOps). It then writes the settings (`PasswordAuthentication no`, `AllowUsers allwin-agent`, `DefaultShell`) and the agents' public key, and validates them with `sshd -t` (QA Security N1);
    - prints the `ssh_host_ed25519` fingerprint. The user reads it on the VNC console and sends it to DevOps, and DevOps pins it in the agents' `known_hosts` with `StrictHostKeyChecking yes`. Trust on first use is not allowed (QA Security I2);
    - re-runs the `Lock-Firewall.ps1` check and **stops** if it fails;
    - only then sets `sshd` to automatic start and starts it;
    - sets up folders and logs;
    - sets up the `/it` Scheduled Tasks for starting/stopping NT8 and taking screenshots.
11. **VPS, RDP as `allwin-agent`:**
    - sign in to NT8 with the **dedicated development login**;
    - keep only Sim101/Playback and the Simulated Data Feed connections;
    - download MNQ Market Replay data;
    - then **disconnect** the RDP session (do not sign out).

**Team (DevOps), after step 11:**

- **T-checks** (in the spike report):
  - (a) from outside the tailnet, ports 3389 and 22 on the public IP are closed;
  - (b) the dev server reaches `ssh allwin-agent@allwin-nt8-01` with the pinned host key and `StrictHostKeyChecking yes`;
  - (c) from the VPS, connections to the dev server's Tailscale IP are refused by the ACL;
  - (d) the Mac reaches RDP;
  - (e) no auth key was generated. The dev server's login URL went from DevOps only to the user, never to another agent, a file or a log. Only public node keys and the host-key fingerprint were exchanged;
  - (f) `tailscale lock status` shows Tailnet Lock enabled and every node signed (Mac, VPS, dev server), with the VPS tagged `tag:allwin-nt8` and the dev server tagged `tag:allwin-dev`;
  - (g) `sshd -T` reports `passwordauthentication no`, and `OpenSSH-Server-In-TCP` is disabled. Repeat (a) and (g) after each Windows feature update, because an update may re-enable default rules.
- Then the spike starts (§7).

**Never on the VPS:** live brokerage or prop-firm credentials, the user's personal NT8 profile, or admin rights for `allwin-agent`.

### 6.6 Complements

- **User-run scripts (E):** remain available for a first session if setup is delayed.
- **CI runner (D):** can be added later on the same VPS as a regression gate, once the GitHub remote and push authorization exist.
- **The reverse tunnel (B):** stays documented as the fallback if Tailscale is rejected (T2).

## 7. Spike sequence (superseded by §8 in rev. 7)

1. **Day 1:** T-checks (§6.5), then E1 (access and start), E2 (hot loop), E3 (packaging), K12 store basics.
2. **Day 2:** E4 (orders, events, names, guard), E9 (socket soak), E11 (bracket integrity), E12 (crash).
3. **Day 3:** E5 (Playback clock), E8 (block timing), E13 (latency), E14 (commissions), E10 if safe; then the report.

**Exit gate:** the report is reviewed by the Architect and QA; ADR deltas, if any, are issued; V1-S2 starts.

## 8. Manual test rounds (rev. 7, user decision 2026-10-24)

**What changes.** Nothing in the protocol, the Core or the plugin architecture. Only the way the plugin host is tested changes: the user runs every NT8 test by hand over RDP, always on Sim101 or Playback with the Simulated Data Feed. Agents never touch the VPS.

**Cancelled:** K1/E1 (remote start with `schtasks`), K6 T-checks for agent access and E6, K7/E7, the agent-side loop scripts (`tools/` start, stop and deploy over SSH; `infra/windows-test-host/`), `sshd` on the VPS, the tailnet rule dev → VPS:22, and the agent-driven hot source loop. The rule that only the user restarts NT8 (§6.4) becomes the general rule.

**Still run, now by the user, one round at a time:**

| Experiment | What the user does | What answers it |
|---|---|---|
| E2/E3 (K2/K3) | Import the round's package (Tools → Import → NinjaScript Add-On); note whether it compiles and loads without a restart, and whether the packaged form loads after a restart | Import result + NT8 log |
| E4 + K15 | Start the looping scenario of the local fake Core with one action (10 repetitions); restart NT8 once and check that the `Order.Name` association and the day's executions survive | Plugin journal and log |
| E5 | Playback at 1× and Max, Go To, rewind | Plugin log (clocks, `SimulationAccountReset`) |
| E8, E13 | Start the looping scenario (≥ 20 trials) with one action. The plugin logs every stage with a monotonic high-resolution time and UTC: for E13, frame received → journaled → Submit → OrderUpdate accepted → ExecutionUpdate → report written → ack; for E8, the execution that crosses the limit → block set → cancel/flatten Submit. The platform leg comes from the NT8 callback times, not `Execution.Time`. The fake Core logs its send/receive on the same host clock. Note whether an RDP session was active | Plugin and fake-Core logs |
| E9 | Start the self-repeating soak scenario and leave it for 1 h | Plugin log, NT8 log |
| E11 | NT8 cannot move prices while closed, so the case is reproduced with the plugin unobservable instead: (a) stop the plugin while NT8 runs, move the price through the stop with the Simulated Data Feed controls, start the plugin; (b) in Playback, move past the stop while the plugin is stopped | Plugin journal and log (ADR-0004 §6.1 cases) |
| E12 | A random Task Manager kill cannot hit the windows that matter, so development builds get fault-injection switches ("crash at step X", set by the fake-Core scenario or a dev-only menu): after the journal write and before Submit; after Submit and before "submitted" is journaled; before the ack. The user starts each case with one action and restarts NT8. **Development builds only (rev. 7.1, Crivo L1, Vigia L1):** see "Development-only capabilities" below | Journal: no duplicate execution, per window |
| RX | **Optional, user-only (rev. 7.1, Pêndulo n3):** a TPT account in Rithmic plug-in mode. The user connects it and the plugin only observes: the connection kind and provider it reports, `oco_location`, and the account classification. Orders only if the account is classified as simulation; never on a live account | Plugin log (`[INF]` lines for the connection) |
| E14 | Run with and without a commission template | Plugin log (commission at the event and later) |
| E10 | Skipped unless safely testable; never with a live account | — |
| D1/D2, K12 token storage | Run the exact PowerShell commands in the script (`icacls` on the plugin folders, a search of the logs for the token) | Command output in the round's zip |
| D3 | Optional and user-only: needs a second Windows user on the VPS | — |

**What the team delivers for each round:**
1. **An import package:** a NinjaScript Add-On import file named with its version (for example `AllWin-NT8-0.3.1-round2.zip`). Development rounds prefer the **source** form, which NT8 compiles on import without a restart. Milestone rounds use the packaged DLL form, which needs one restart by the user. The Plugin Engineer chooses the form and states it.
2. **A short numbered script** from QA Plugins: step → expected result, with as many checks per round as fit, Sim101/Playback only. The script also says when a restart is needed.
3. **The logs to return and how:** the plugin window's "Export diagnostics" (ADR-0005 N12) zips the plugin logs and a journal copy (never the token or consent files). The user adds the NT8 `log`/`trace` files of the day (`Documents\NinjaTrader 8\log` and `\trace`) and the fake-Core transcript. The user returns them through the RDP-redirected drive; the Maestro names the drop location on the dev side.
   - **NT8's own logs (rev. 7.1, Vigia I1):** they can name the user's other connections and account numbers, possibly live ones. The script asks the user to check them, or the collection step keeps only the Sim101/Playback lines, before returning. Nothing from a live connection goes into the drop.
   - **The drop on the dev side (Vigia I2):** group-restricted (like the review folders: group only, no world access) with a retention rule, because it holds journals and transcripts of simulated trading.
4. **Hashes (rev. 7.1, Vigia L2; Crivo nit):** the SHA-256 of the package and of the fake-Core exe do **not** travel in the script or anywhere on the RDP-redirected drive. The Maestro sends them to the user in the round message, a separate channel, so the check catches substitution on the drive as well as corruption. The user checks them with `Get-FileHash` before the import and before starting the exe, and stops on a mismatch.

**Development-only capabilities (rev. 7.1; Crivo L1, Vigia L1, Pêndulo n2).** The fault-injection switches, the dev-only menu and the fake-Core scenario hooks exist only in development builds. They are removed by a **compile-time constant**, never a runtime flag: the same constant that separates release and development builds (ADR-0005 N11). Milestone and release packages are built without it. A packaging check fails the build when a non-development package contains any such entry point (type, menu item or scenario handler). A crash switch in a user's plugin could cause duplicated or unknown orders.

**One source for both targets (rev. 7.1, Crivo L2).** `AllWin.Adapter.Core` builds for `netstandard2.0` (the Linux tests) and `net48` (the package, ADR-0005 N9) from the same source. No `#if` changes logic between the two targets; a target-specific shim, if ever needed, is listed and reviewed. The round scripts exercise the `net48` critical paths inside NT8: journal write and replay, dedupe of `command_id`, staleness guard, and the flatten proof report.

**User decisions (2026-10-24):** no agents on the VPS; focus on finishing the plugin. **M1 waits for the staging host (option B)**: no port is opened now, and the dev server stays in the tailnet with no rule towards the VPS.

**Rounds (from the Plugin Engineer's plan):** R0 probe (import format, platform facts: E3, E4, K15, `Order.Name` across a restart, E14; one Market order on Sim101 is allowed, simulation only); R1 (S2 AC-1, AC-4, AC-5, AC-6, window, strings, token configuration); R2 (AC-2, AC-3, flatten with its proof report, restart recovery and reconnect catch-up, duplicates and stale commands); 1–2 repair rounds. M1 end to end comes later, on staging. Every round is a new package and an NT8 restart by the user. The package layout is ADR-0005 N9 and the local files are N12.

**Protocol version for the rounds:** the fake Core negotiates 1.2, which is the Core's minimum (ADR-0004 §9), so R0–R2 run at 1.2. The plugin must reach **1.4 before M1**: with a session below 1.4 the Core skips the entries of every account with a drawdown rule (ADR-0004 §7.1), and the rollout order is plugin first. The plugin's 1.3 contract tests that are red on `main` are fixed in the S2 branch before it merges.

**Local fake Core.** Without network changes the plugin cannot reach the dev server. It is shipped as a self-contained `win-x64` exe that the user starts on the VPS, on loopback `127.0.0.1` only, with no port opened. It binds `127.0.0.1` explicitly, never `0.0.0.0` or any address, so Windows does not offer to open a firewall rule; if a firewall prompt ever appears, the user cancels it (Vigia L2); it writes a transcript the user returns. The scenarios therefore run against the scripted fake Core shipped in the package, on loopback: development builds only, ADR-0005, with the owner-only token file.
- Its scenarios **loop by themselves** (E4 ×10, E8/E13 ≥ 20 trials, the E9 soak), so each run is one user action.
- Two things cannot be measured on the VPS: the Core relay round trip (A4) and Core → plugin over the real network. They, and the M1 end-to-end test with the real Core, wait for the staging host (user decision B). That needs no VPS change.

**Exit gate:** the same as §7. The spike report is written by the Plugin Engineer from the returned logs and reviewed by the Architect and QA, and ADR deltas are issued if needed.

