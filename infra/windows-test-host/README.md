# Windows test host (NinjaTrader 8) — setup, "Fase 2"

Implements `docs/architecture/spikes/nt8-spike-plan.md` §6.3–§6.5 (rev. 5). **Status: DRAFT** — the scripts have never run on Windows (first run = experiment E1).
**Do not run any script here until Code Review (Crivo) and QA Security (Vigia) approved the exact commit** (they run as Administrator).

| Item | Value |
|---|---|
| Hostname in the tailnet | `allwin-nt8-01` (tag `tag:allwin-nt8`) |
| Dev server (agents) | tag `tag:allwin-dev`, outbound only, joins **without an auth key** |
| Public IP | `<VPS_PUBLIC_IP>` (never written in the repo) |
| Break-glass | Contabo panel → VNC console |

Agents' **public** key (the private half stays on the dev server, mode 600; fingerprint `SHA256:YW0/0bcG352ZlHrfOJeRx6v6h8GcxduANjeRFRaD99w`):

```
ssh-ed25519 AAAAC3NzaC1lZDI1NTE5AAAAIAciPbXwf2lko6zYMDdDKlO+hpJFWiw9GtJRlrUks02k allwin-agents-nt8@dev2
```

Rules: no live accounts on this host; no auth key or login URL is ever pasted in a chat, a file or an agent terminal. Placeholders look like `<THIS>`.

**Current state (user, before this README):** Tailscale is installed on the VPS and on the Mac; the VPS joined **without a tag and without an auth key**. So there is no auth key to create or pre-sign: the VPS is tagged in the admin console and the existing nodes are signed once Tailnet Lock is on.

---

## 0. Before you start (Contabo)

1. Enable 2FA on the Contabo account. Save the Administrator password in your password manager.
2. Contabo panel → your VPS → **VNC console**. Sign in as Administrator. Do **not** use RDP over the public IP.

## 1. Tailscale admin console (browser, on the Mac)

1. Enable 2FA on the Tailscale login.
2. **Access controls** → paste the whole content of `infra/tailscale/policy.hujson` → Save. The built-in `tests` block must pass (the console refuses the save otherwise).
3. **Machines** → the VPS → *⋯ → Edit ACL tags* → `tag:allwin-nt8`. Then *⋯ → Edit machine name* → `allwin-nt8-01`. (Do this while signed in as the admin who owns the tag.)

## 2. Mac — Tailnet Lock and signing the existing nodes (Terminal)

```bash
# Enable Tailnet Lock with this Mac as the trusted signer. Save the printed disablement secrets in the password manager.
tailscale lock init --gen-disablements 2
tailscale lock status
```

If `lock init` asks for the trusted key list, follow its own prompt and include this Mac's key (shown by `tailscale lock status`). Keep one disablement secret offline so losing the Mac does not lock the tailnet.

`tailscale lock status` lists nodes that are not signed ("locked out"). The Mac signs itself on init. For the VPS, read its node key **on the VPS** (a node key is a public value, not a secret) in the VNC console and sign it on the Mac:

```powershell
# VPS, VNC console:
& "C:\Program Files\Tailscale\tailscale.exe" lock status
```
```bash
# Mac: check the node key printed above matches the pending VPS in the admin console (hostname allwin-nt8-01, OS Windows), then:
tailscale lock sign nodekey:<VPS_NODE_KEY>
```

The VPS also needs to stay joined as a tagged, non-expiring node: confirm in the console that `allwin-nt8-01` shows the tag and no key-expiry date.

## 3. VPS — get the scripts onto the machine

The repository is private. Pick one:

- **A (preferred)**: in the VNC console open PowerShell **as Administrator**, create an admin-only folder, then paste each script into Notepad (VNC clipboard support is unverified for Contabo):

  ```powershell
  mkdir C:\AllWin-setup
  icacls C:\AllWin-setup /inheritance:r /grant:r '*S-1-5-18:(OI)(CI)F' '*S-1-5-32-544:(OI)(CI)F'
  notepad C:\AllWin-setup\Lock-Firewall.ps1
  notepad C:\AllWin-setup\Install-AllWinTestHost.ps1
  ```
- **B**: RDP from the Mac to `allwin-nt8-01` (tailnet) with *Devices & Audio → Drives* shared and copy the two files into the same admin-only folder.

**Before EVERY run** (the scripts run as Administrator), compare the hashes with the ones the Maestro gives you for the approved commit:

```powershell
Get-FileHash C:\AllWin-setup\*.ps1 -Algorithm SHA256 | Format-Table Hash, Path
```

Both scripts must stay in the same folder (the installer calls `Lock-Firewall.ps1 -CheckOnly`).

## 4. VPS — tailnet check (already joined)

```powershell
& "C:\Program Files\Tailscale\tailscale.exe" status
& "C:\Program Files\Tailscale\tailscale.exe" ip -4
```

Expect a `100.x` address and the host `allwin-nt8-01`. Do not run `tailscale up` again with any key.

## 5. VPS — install the OpenSSH Server capability and do NOT start sshd

The capability creates `OpenSSH-Server-In-TCP`, enabled for any remote address, and the default `sshd_config` accepts passwords. So: install it, keep `sshd` stopped, and lock the firewall right after (step 6).

```powershell
Add-WindowsCapability -Online -Name OpenSSH.Server~~~~0.0.1.0
Get-Service sshd          # must say Stopped. If it is Running: Stop-Service sshd
```

Never run `Start-Service sshd` by hand. The installer starts it, only after the firewall audit passes.

## 6. VPS — lock RDP and SSH to the tailnet (PowerShell as Administrator)

```powershell
# Dry run first: lists every change, changes nothing.
C:\AllWin-setup\Lock-Firewall.ps1 -WhatIf
# Real run (refuses to change anything unless the Tailscale adapter is up with a 100.x address).
C:\AllWin-setup\Lock-Firewall.ps1
```

It ends with an audit of the active firewall policy and **fails** if any enabled inbound allow rule still exposes TCP/UDP 3389 or 22 outside `100.64.0.0/10` (including rules with port Any or a port range). Fix what it lists and rerun; it is idempotent. Transcripts go to the admin-only `C:\AllWin\admin-logs`.

## 7. Mac — RDP over the tailnet

Windows App / Microsoft Remote Desktop → PC name `allwin-nt8-01` → sign in as Administrator (NLA required). From now on use only this.

## 8. Dev server — join without an auth key (Forja, user already approved U1)

Forja runs, on the dev server (no key anywhere):

```bash
sudo tailscale up --advertise-tags=tag:allwin-dev --accept-dns=false --accept-routes=false
```

The command prints a **login URL**. Treat it as a one-use capability: it goes only to you (not to any agent, chat or file).

1. Open the URL on the Mac, signed in as the admin who owns the tag, and **before approving** check in the admin console that the pending node is the dev server (hostname and OS Linux) and that its node key matches the one Forja reads from `tailscale lock status` on the dev server.
2. Approve it. Forja sends you only the **node key** (public value).
3. On the Mac: `tailscale lock sign nodekey:<DEV_NODE_KEY>`.
4. Confirm with `tailscale status` (tailnet, tag `tag:allwin-dev`) and `tailscale lock status` (signed) on the Mac.

## 9. VPS — users and software (as Administrator)

```powershell
# Standard user for the agents (NOT an administrator). Store the password in the password manager.
$pw = Read-Host -AsSecureString 'Password for allwin-agent'
New-LocalUser -Name allwin-agent -Password $pw -PasswordNeverExpires -AccountNeverExpires -FullName 'All Win agents'
Add-LocalGroupMember -Group 'Remote Desktop Users' -Member allwin-agent
```

Install by hand: NinjaTrader 8 (8.1.8.4+), .NET SDK 8+, Build Tools for Visual Studio (MSBuild + .NET Framework 4.8 targeting pack), Git for Windows.

## 10. VPS — configure sshd, folders and tasks

```powershell
$key = 'ssh-ed25519 AAAAC3NzaC1lZDI1NTE5AAAAIAciPbXwf2lko6zYMDdDKlO+hpJFWiw9GtJRlrUks02k allwin-agents-nt8@dev2'
C:\AllWin-setup\Install-AllWinTestHost.ps1 -PublicKey $key -WhatIf
C:\AllWin-setup\Install-AllWinTestHost.ps1 -PublicKey $key
```

In this order it: checks that `sshd` is still stopped and the firewall audit passes; creates `sshd_config` (from `sshd_config_default`) and the host keys without starting `sshd`; writes the hardened config (`PasswordAuthentication no`, `AllowUsers allwin-agent`, `DisableForwarding yes`, `DefaultShell` Git Bash) and the agents' key (`from="100.64.0.0/10"`); re-runs the audit; only then sets `sshd` to automatic start and starts it; creates the folders and the `/it` Scheduled Tasks. At the end it prints the **SSH host key fingerprint and a `known_hosts` line**: note both (T-check b).

## 11. VPS — NinjaTrader first login (RDP as `allwin-agent`)

Sign in to NT8 with the **dedicated development login**; keep only Sim101/Playback and the Simulated Data Feed connections; download MNQ Market Replay data; then **disconnect** the RDP session (do not sign out).

## 12. T-checks (Forja records them in the spike report)

| | Check | How |
|---|---|---|
| a | public 3389 and 22 closed | from a host outside the tailnet: `nc -zv -w5 <VPS_PUBLIC_IP> 3389`, same for 22 → both fail |
| b | dev server reaches the VPS, host key pinned | you give Forja the `known_hosts` line printed in step 10 (public value) and compare the fingerprint with the VNC console (`ssh-keygen -lf C:\ProgramData\ssh\ssh_host_ed25519_key.pub`). Forja adds it to the agents' `known_hosts` and runs `ssh -o StrictHostKeyChecking=yes -i ~/.ssh/allwin_agents_nt8_ed25519 allwin-agent@<VPS_TAILSCALE_IP> 'echo ok'` (by IP: the dev server has `--accept-dns=false`, so the MagicDNS name does not resolve there) |
| c | VPS cannot reach the dev server | on the VPS: `Test-NetConnection <DEV_TAILSCALE_IP> -Port 22` → fails |
| d | Mac reaches RDP | step 7 |
| e | no auth key anywhere, none handed to an agent | on the VPS: `Select-String -Path C:\AllWin\admin-logs\*,C:\AllWin-setup\* -Pattern 'tskey-'`; on the dev server: `grep -rl 'tskey-' ~/.bash_history ~/.zsh_history /var/log 2>/dev/null` → no match |
| f | Tailnet Lock | on the Mac: `tailscale lock status` → enabled, all three nodes (Mac, VPS, dev server) signed |
| g | sshd locked | on the VPS: `& "$env:WINDIR\System32\OpenSSH\sshd.exe" -T \| Select-String passwordauthentication` → `no`; `Get-NetFirewallRule -Name OpenSSH-Server-In-TCP` → `Enabled False`. **Repeat (a) and (g) after each Windows feature update** (an update may re-enable default rules). |

## Break-glass and rollback

- Locked out: use the Contabo VNC console. Check `Get-NetFirewallRule -Name AllWin-Tailnet-*` and `tailscale status`; fix Tailscale first. Re-enabling a public RDP rule is a firewall change that needs the user's explicit decision, and must be undone by re-running `Lock-Firewall.ps1`.
- `sshd_config` is backed up next to itself (`sshd_config.allwin-backup-<timestamp>`) before each change and restored automatically if `sshd -t` fails or if `sshd` fails to start (the service is then set to Disabled).
