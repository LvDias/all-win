# NinjaTrader 8 spike plan and Windows build/test access (Q15)

- **Status:** rev. 3 (2026-10-23).
  - User decisions: W1 reverse tunnel accepted, W3 accepted, W2 changed to a Contabo Windows VPS without GPU, and remote access to go over Tailscale with RDP never exposed to the internet.
  - rev. 3 revises W1 for Tailscale (§6.2, Proposed: the user approves T1 and the dev-server join) and gives the setup checklist (§6.5).
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
  - rev. 2 merged my option O1 with Trilha's option B (reverse SSH tunnel); the user accepted B. **rev. 3 recommends T1 (Tailscale for RDP and agent SSH)**, after the user's Tailscale proposal for RDP (§6.2). B remains the fallback.
  - Spike questions mapped to experiments E1–E10.

## 1. Goal and timebox

Retire the plugin risks before V1-S2 is implemented, and leave a **repeatable build → deploy → test loop** on the Windows machine.

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
| §2 | Requirements: Windows 11 or **Windows Server 2016+** (officially supported); DirectX 10 recommended. Trilha confirmed a GPU is **not indispensable** for Sim/Playback build and test; software rendering (WARP) is acceptable, though staff report crashes on some GPU-less VPS | **W2 = Contabo Windows Server 2022 VPS without GPU.** Plan B is a vGPU instance if E1/E3/E9 fail for graphics reasons (§6.1) |
| F5 | On NinjaTrader/Rithmic connections, **OCO is simulated locally** | Bracket-integrity tests in the spike (ADR-0004 §6.1) |

## 3. Questions the spike answers (mapped to the profile's experiments)

| # | Question | Experiment | Done when |
|---|---|---|---|
| K1 | Start NT8 remotely in the interactive session; behavior of the 8.1.8.4 login screen | **E1** | Window appears after `schtasks /run`; the login need is documented |
| K2 | Hot source loop: auto-compile, AddOn re-instantiation, where compile errors appear (logs readable over SSH?), does `AllWin.Adapter.Core` source compile under NT's compiler | **E2** | Edit → save → compiled and reloaded within a minute, without a restart; errors visible from the shell |
| K3 | Packaged build: `net48` multi-target of `AllWin.Adapter.Core` + host DLL, copy and restart, then zip + `AdditionalReferences.txt` import on a clean profile | **E3** | Both load; import on a clean profile works |
| K4 | Accounts with Mode/Provider (simulation guard, `oco_location`), market entry → OCO bracket from the fill, event sequence including partial fills and `Operation.Remove`, `Order.Name` reference surviving a restart, `OrderEntry.Automated` | **E4** | Scripted scenario passes 10/10; the restart keeps the `Name` association |
| K5 | Playback clock: `PlaybackConnection.Now` vs `Core.Globals.Now`; `SimulationAccountReset` on rewind | **E5** | Clock source chosen for S15; rewind handling documented |
| K6 | Access path works and nothing is exposed: T-checks (a)–(e) for T1. E6 (reverse tunnel with the inbound rule disabled) applies only if the fallback B is used | **T-checks / E6** | All T-checks pass |
| K7 | Maestri app with a Windows host (only if the user wants option C later) | **E7** | Optional |
| K8 | Reactive block timing | **E8** | p50/p95 over 20 trials |
| K9 | WebSocket under load beside order traffic: UI freeze or broker disconnect? | **E9** | 1 h soak, no freeze or drop |
| K10 | Does Global Simulation Mode block NinjaScript `Account.Submit` to non-sim accounts? | **E10** | Only if safely testable; otherwise skipped. Never create a live account |
| K11 | **Bracket integrity (new):** close NT8 with a working bracket on Sim101, move price through the stop with the Simulated Data Feed on restart, and observe the orphan leg, `Unknown` states and reconnect detection | new **E11** | ADR-0004 §6.1 cases reproduced, or documented as not reproducible on Sim |
| K12 | Durable local store (outbox, processed `command_id`s, cached limits) survives a crash; DPAPI token storage | new **E12** | Kill NT8 mid-command → restart → no duplicate execution |
| K13 | Latency: Core send → submit → fill report on Sim101, plus the Core relay round trip (A4) | new **E13** | p50/p95 table |

## 4. Out of scope

- Product-quality code, UI polish, localization.
- Core implementation: the spike uses a **scripted fake Core** (profile §13.3), the mirror of `adapter-sim`.
- Any live account, live connection, Market Replay download, or NinjaTrader login. Those are done by the user only (profile §14).

## 5. Deliverables

1. `plugins/ninjatrader8/docs/spike-report.md` (Plugin Engineer): K1–K13 with evidence and measured numbers, plus proposed ADR deltas. If deltas are needed, the Architect issues them.
2. Repeatable loop scripts:
   - `plugins/ninjatrader8/tools/` (Plugin Engineer): build, deploy-source, deploy-package, start/stop NT8, collect logs and screenshots, run scenarios.
   - `infra/windows-test-host/` (DevOps): host setup and tunnel task.
3. Fake-Core scenario harness: entry → fill → bracket, partial fills, reject, disconnect/reconnect, expiry, flatten/block, orphan cleanup, bracket integrity.
4. Draft install guide for the user (S3 "Como instalar o plugin").

## 6. Windows test machine and access (W1–W4)

**Fixed in every option:**

- A dedicated test VPS with a dedicated **standard (non-admin) Windows user** `allwin-agent`.
- A **dedicated NinjaTrader development login**, ideally on the free license (Global Simulation Mode stays locked on).
- **No live broker connections** and no personal data on the VPS.
- The plugin's own simulation guard, which fails closed (ADR-0005 N4).

### 6.1 W2 — Machine (decided: Contabo, no GPU)

- **Spec:** Contabo Windows VPS, **Windows Server 2022 with Desktop Experience**, 6–8 vCPU, 16–24 GB RAM, 150–200 GB NVMe, region **St. Louis** (US central, consistent with A2).
- **Server, not Windows 11:** NT8 officially supports Windows Server 2016+ (profile §2).
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

**Recommendation: T1.**

There is no reason to avoid Tailscale here. Its one structural trade-off is trusting Tailscale's coordination server (control plane) and the user's tailnet admin login. Traffic between nodes is end-to-end encrypted with WireGuard. The control-plane trust can be reduced with **Tailnet Lock**, so new nodes must be signed by a trusted node. If the user does not want any third-party control plane, T3 (or Headscale, self-hosted) is the alternative.

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
- **Auth keys:** two **single-use, pre-authorized, tagged** auth keys, generated by the user. They are used once at install and then expire; they are never stored in the repo or in logs.
- **DNS:**
  - **MagicDNS** on, for names such as `allwin-nt8-01`.
  - The dev server runs Tailscale with `--accept-dns=false`, so its system resolver is not changed; it reaches the VPS by its Tailscale IP or a static hosts entry managed by DevOps.
  - The VPS and the Mac may use MagicDNS.
- **Dev server specifics:** `tailscale up --auth-key=… --advertise-tags=tag:allwin-dev --accept-dns=false --accept-routes=false`. No ufw change is needed: the dev server only initiates connections, and the ACL blocks inbound tailnet traffic to it. Installing Tailscale on the dev server is a network change, so it **needs the user's explicit approval** (decision of 2026-10-23: SSH/firewall/exposure changes are not covered by the standing authorization).
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
- **Initial exposure window:** a new Contabo Windows VPS usually starts with RDP reachable on its public IP. To keep "never open to the internet" true, the first setup is done through the **VNC console**. If VNC is unavailable, the user does steps 3–5 of the checklist in one short first RDP session with the strong initial password, and then confirms with the T-checks below that 3389 is closed publicly.
- **Production stays out:** the future production server must **not** join this tailnet without a separate decision.

### 6.4 W3 — NT8 restarts (decided)

- The user types the NT8 password when NT8 must restart; the hot source loop keeps restarts rare.
- Windows auto-logon stays off.
- The `allwin-agent` desktop session stays alive by **disconnecting** RDP rather than signing out, so `schtasks /it` can start NT8 in it (E1 verifies on Server 2022).

### 6.5 W4 — Setup checklist

**User** (about 1–2 h; DevOps scripts are reviewed by the user before running):

1. **Contract** the Contabo VPS (§6.1). Enable 2FA on the Contabo account. Save the Administrator password in the password manager.
2. **Open the VNC console** in the Contabo panel and sign in as Administrator. Use the first RDP session only if VNC is unavailable, and keep it short.
3. **Tailscale admin** (create the account if needed; enable 2FA on its login):
   - apply `infra/tailscale/policy.hujson` (prepared by DevOps);
   - optionally enable Tailnet Lock;
   - generate **two** single-use, pre-authorized auth keys, one tagged `tag:allwin-nt8` and one tagged `tag:allwin-dev`.
4. **VPS:** install Tailscale and join with the `tag:allwin-nt8` key (hostname `allwin-nt8-01`).
5. **VPS:** run `infra/windows-test-host/Lock-Firewall.ps1` (DevOps). It disables the public RDP/SSH rules, adds the tailnet-scoped rules, and enforces NLA.
6. **Mac:** install Tailscale, sign in, and from now on RDP only to `allwin-nt8-01`.
7. **Approve** DevOps joining the **dev server** to the tailnet with the `tag:allwin-dev` key (flags in §6.3). Approving this is the user's explicit network approval.
8. **VPS, as Administrator:** create the standard user `allwin-agent` and add it to *Remote Desktop Users*. Install:
   - NinjaTrader 8 (8.1.8.4 or later);
   - .NET SDK 8+;
   - Build Tools for Visual Studio (MSBuild + .NET Framework 4.8 targeting pack);
   - Git for Windows;
   - OpenSSH Server.
9. **VPS:** run `infra/windows-test-host/Install-AllWinTestHost.ps1` (DevOps). It sets up:
   - the `sshd` config and the agents' public key;
   - folders and logs;
   - the `/it` Scheduled Tasks for starting/stopping NT8 and taking screenshots.
10. **VPS, RDP as `allwin-agent`:**
    - sign in to NT8 with the **dedicated development login**;
    - keep only Sim101/Playback and the Simulated Data Feed connections;
    - download MNQ Market Replay data;
    - then **disconnect** the RDP session (do not sign out).

**Team (DevOps), after step 10:**

- **T-checks** (in the spike report):
  - (a) from outside the tailnet, ports 3389 and 22 on the public IP are closed;
  - (b) the dev server reaches `ssh allwin-agent@allwin-nt8-01`;
  - (c) from the VPS, connections to the dev server's Tailscale IP are refused by the ACL;
  - (d) the Mac reaches RDP;
  - (e) no auth key appears in any file or log.
- Then the spike starts (§7).

**Never on the VPS:** live brokerage or prop-firm credentials, the user's personal NT8 profile, or admin rights for `allwin-agent`.

### 6.6 Complements

- **User-run scripts (E):** remain available for a first session if setup is delayed.
- **CI runner (D):** can be added later on the same VPS as a regression gate, once the GitHub remote and push authorization exist.
- **The reverse tunnel (B):** stays documented as the fallback if Tailscale is rejected (T2).

## 7. Spike sequence (after the machine is ready)

1. **Day 1:** T-checks (§6.5), then E1 (access and start), E2 (hot loop), E3 (packaging), K12 store basics.
2. **Day 2:** E4 (orders, events, names, guard), E9 (socket soak), E11 (bracket integrity), E12 (crash).
3. **Day 3:** E5 (Playback clock), E8 (block timing), E13 (latency), E10 if safe; then the report.

**Exit gate:** the report is reviewed by the Architect and QA; ADR deltas, if any, are issued; V1-S2 starts.
