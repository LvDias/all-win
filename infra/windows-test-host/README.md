# Windows test host (NinjaTrader 8) — setup, "Fase 2"

> **Phase 2 parked (2026-10-24).** NinjaTrader 8 tests are manual, run by the user over RDP. The VPS stays as it is (Tailscale,
> policy, Tailnet Lock, RDP restricted to the tailnet). **Steps 8-12 are not used:** sshd is not started,
> `Install-AllWinTestHost.ps1` is not run, agents get no access to the VPS, and `Lock-Firewall.ps1` is kept only for reference.
> Files reach the VPS through the user (RDP drive redirection); see `infra/nt8-exchange/README.md`.

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

Rules: no live accounts on this host; no auth key is ever pasted in a chat, a file or an agent terminal. The dev-server login URL (step 8) is relayed by Forja only to you, who opens it at once. Placeholders look like `<THIS>`.

**Current state (reported by the user):**

- Tailscale is installed on the VPS (through the GUI) and on the Mac. The VPS joined **without a tag and without an auth key**: it gets its tag in the admin console and the existing nodes are signed once Tailnet Lock is on. There is no auth key to create.
- RDP works **only through the tailnet**: the user restricted the three Remote Desktop rules to `100.64.0.0/10` by hand, and the public IP refuses the connection.
- The **VNC console does not accept paste**. All steps below therefore run in an **RDP session over the tailnet from the Mac** (clipboard and drive redirection work there). VNC stays break-glass only (typing, no paste).
- `allwin-agent` already exists (Remote Desktop Users, not an administrator). NinjaTrader 8 is installed, not logged in. Windows Server 2025 ships the OpenSSH Server stopped with manual start.
- **Still to do:** policy + VPS tag and name (step 1), Tailnet Lock and signing (step 2), the scripts (steps 3, 5, 6, 10), the dev server (step 8), the NT8 login (step 11).

---

## 0. Before you start (Contabo)

1. Enable 2FA on the Contabo account. Save the Administrator password in your password manager.
2. Work from an **RDP session over the tailnet** (Mac → `allwin-nt8-01` or its `100.x` address), as Administrator. The public IP does not accept RDP. The Contabo **VNC console** is break-glass only (no paste).

## 1. Tailscale admin console (browser, on the Mac)

1. Enable 2FA on the Tailscale login.
2. **Access controls** → paste the whole content of `infra/tailscale/policy.hujson` → Save. The built-in `tests` block must pass (the console refuses the save otherwise).
3. **Machines** → the VPS → *⋯ → Edit ACL tags* → `tag:allwin-nt8`. Then *⋯ → Edit machine name* → `allwin-nt8-01`. (Do this while signed in as the admin who owns the tag.)
4. **Do 2 and 3 back to back.** Once the policy is saved, an untagged VPS is not covered by any rule (the rules name only the tags), so RDP over the tailnet stops until the tag is applied. Keep the VNC console open as the way back in.

## 2. Mac — Tailnet Lock and signing the existing nodes (Terminal)

```bash
# Enable Tailnet Lock with this Mac as the trusted signer. Save the printed disablement secrets in the password manager.
tailscale lock init --gen-disablements 2
tailscale lock status
```

If `lock init` asks for the trusted key list, follow its own prompt and include this Mac's key (shown by `tailscale lock status`). Keep one disablement secret offline so losing the Mac does not lock the tailnet.

`tailscale lock status` lists nodes that are not signed ("locked out"). The Mac signs itself on init. For the VPS, read its node key **on the VPS** (a node key is a public value, not a secret) in the RDP session and sign it on the Mac:

```powershell
# VPS, RDP session:
& "C:\Program Files\Tailscale\tailscale.exe" lock status
```
```bash
# Mac: check the node key printed above matches the pending VPS in the admin console (hostname allwin-nt8-01, OS Windows), then:
tailscale lock sign nodekey:<VPS_NODE_KEY>
```

The VPS also needs to stay joined as a tagged, non-expiring node: confirm in the console that `allwin-nt8-01` shows the tag and no key-expiry date.

## 3. VPS - get the scripts onto the machine

The repository is private and the VNC console cannot paste, so use **option B only**: RDP from the Mac to the VPS (over the tailnet) with **one folder that holds only the two scripts** shared (*Devices & Audio -> Drives*, or Clipboard), not the whole Mac disk: the Administrator session can read whatever is shared. Copy the two scripts from the Mac checkout of the approved commit (`infra/windows-test-host/Lock-Firewall.ps1` and `Install-AllWinTestHost.ps1`), make the folder admin-only, and only then turn the sharing off. In PowerShell as Administrator:

```powershell
mkdir C:\AllWin-setup
icacls C:\AllWin-setup /inheritance:r /grant:r '*S-1-5-18:(OI)(CI)F' '*S-1-5-32-544:(OI)(CI)F'
# then copy the two .ps1 files into C:\AllWin-setup (Explorer drag and drop from the shared drive, or paste)
```

**Before EVERY run** (the scripts run as Administrator), compare the hash with the one the Maestro gives you for the approved commit. The scripts are ASCII-only and the check ignores line endings, so Notepad (LF or CRLF) does not change the result:

```powershell
Get-ChildItem C:\AllWin-setup\*.ps1 | ForEach-Object {
  $t = [IO.File]::ReadAllText($_.FullName) -replace "`r`n", "`n"
  $h = [BitConverter]::ToString([Security.Cryptography.SHA256]::Create().ComputeHash([Text.Encoding]::UTF8.GetBytes($t))).Replace('-', '').ToLower()
  '{0}  {1}' -f $h.Substring(0, 16), $_.Name
}
```

If a hash does not match, stop and tell the Maestro. Do not run the script.

Both scripts must stay in the same folder (the installer calls `Lock-Firewall.ps1 -CheckOnly`).

## 4. VPS — tailnet check (already joined)

```powershell
& "C:\Program Files\Tailscale\tailscale.exe" status
& "C:\Program Files\Tailscale\tailscale.exe" ip -4
```

Expect a `100.x` address. The host name becomes `allwin-nt8-01` after step 1. Do not run `tailscale up` again with any key.

## 5. VPS - check the OpenSSH Server and do NOT start sshd

Windows Server 2025 ships the OpenSSH Server stopped with manual start. Its rule `OpenSSH-Server-In-TCP` may already exist for any remote address, and the default `sshd_config` accepts passwords. So: keep `sshd` stopped and lock the firewall right after (step 6).

```powershell
Get-WindowsCapability -Online -Name 'OpenSSH.Server*' | Select-Object Name, State   # expect Installed
Get-Service sshd                                                                  # must say Stopped
```

If the state is `NotPresent`, install it with `Add-WindowsCapability -Online -Name OpenSSH.Server~~~~0.0.1.0`. If `sshd` is Running, run `Stop-Service sshd` first. Never run `Start-Service sshd` by hand: the installer starts it, only after the firewall audit passes.

## 6. VPS — lock RDP and SSH to the tailnet (PowerShell as Administrator)

```powershell
# Dry run first: lists every change, changes nothing.
C:\AllWin-setup\Lock-Firewall.ps1 -WhatIf
# Real run (refuses to change anything unless the Tailscale adapter is up with a 100.x address).
C:\AllWin-setup\Lock-Firewall.ps1
```

**Rules you edited by hand:** the three Remote Desktop rules you already restricted to `100.64.0.0/10` pass the audit and stay enabled (the script does not touch rules whose remote address is inside the tailnet range). Any Remote Desktop rule still open to other addresses, and a pre-existing `OpenSSH-Server-In-TCP` open to any address, are **disabled** by the script. The script adds its own two interface-scoped rules next to yours.

It ends with an audit of the active firewall policy and **fails** if any enabled inbound allow rule still exposes TCP/UDP 3389 or 22 outside `100.64.0.0/10` (including rules with port Any or a port range). Fix what it lists and rerun; it is idempotent. Transcripts go to the admin-only `C:\AllWin\admin-logs`.

**After step 6 succeeds**, scope your hand-edited Remote Desktop rules to the Tailscale interface too (the audit checks the interface only for the two AllWin rules, so this is on you). Confirm first that RDP over the tailnet still works (it now also passes through `AllWin-Tailnet-RDP-TCP-3389`), then:

```powershell
Get-NetFirewallRule -Name 'RemoteDesktop-*' | Where-Object Enabled -eq 'True' | Set-NetFirewallRule -InterfaceAlias Tailscale   # every enabled Remote Desktop rule; keeps RDP over UDP
C:\AllWin-setup\Lock-Firewall.ps1 -CheckOnly                                         # must end with "Audit OK"
```

If RDP breaks, use the VNC console to run `Get-NetFirewallRule -Name 'RemoteDesktop-*' | Where-Object Enabled -eq 'True' | Set-NetFirewallRule -InterfaceAlias Any` and fix Tailscale first.

## 7. Mac — RDP over the tailnet

Windows App / Microsoft Remote Desktop → PC name `allwin-nt8-01` → sign in as Administrator (NLA required). From now on use only this.

## 8. Dev server — join without an auth key (Forja, user already approved U1)

Forja runs, on the dev server (no key anywhere):

```bash
sudo tailscale up --advertise-tags=tag:allwin-dev --accept-dns=false --accept-routes=false
```

The command prints a **login URL**. Forja sees it in the terminal, so treat it as a one-use capability: Forja relays it only to you, and you open it at once (or run the command yourself in your own session).

1. Open the URL on the Mac, signed in as the admin who owns the tag, and **before approving** check in the admin console that the pending node is the dev server (hostname and OS Linux) and that its node key matches the one Forja reads from `tailscale lock status` on the dev server.
2. Approve it. Forja sends you only the **node key** (public value).
3. On the Mac: `tailscale lock sign nodekey:<DEV_NODE_KEY>`.
4. Confirm with `tailscale status` (tailnet, tag `tag:allwin-dev`) and `tailscale lock status` (signed) on the Mac.

## 9. VPS - users and software (as Administrator)

Already done: user `allwin-agent` (Remote Desktop Users, not an administrator) and NinjaTrader 8. Check what is still missing (Git for Windows is required by the installer: it needs Git Bash):

```powershell
Get-LocalGroupMember -Group Administrators | Select-Object Name        # allwin-agent must NOT be listed
Test-Path 'C:\Program Files\Git\bin\bash.exe'                         # True
dotnet --version                                                        # .NET SDK 8+
```

Install by hand what is missing: .NET SDK 8+, Build Tools for Visual Studio (MSBuild + .NET Framework 4.8 targeting pack), Git for Windows.

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
| b | dev server reaches the VPS, host key pinned | you give Forja the `known_hosts` line printed in step 10 (public value) and compare the fingerprint with the RDP session (`ssh-keygen -lf C:\ProgramData\ssh\ssh_host_ed25519_key.pub`). Forja adds it to the agents' `known_hosts` and runs `ssh -o StrictHostKeyChecking=yes -i ~/.ssh/allwin_agents_nt8_ed25519 allwin-agent@<VPS_TAILSCALE_IP> 'echo ok'` (by IP: the dev server has `--accept-dns=false`, so the MagicDNS name does not resolve there) |
| c | VPS cannot reach the dev server | on the VPS: `Test-NetConnection <DEV_TAILSCALE_IP> -Port 22` → fails |
| d | Mac reaches RDP | step 7 |
| e | no auth key anywhere, none handed to an agent | on the VPS: `Select-String -Path C:\AllWin\admin-logs\*,C:\AllWin-setup\* -Pattern 'tskey-'`; on the dev server: `grep -rl 'tskey-' ~/.bash_history ~/.zsh_history /var/log 2>/dev/null` → no match |
| f | Tailnet Lock | on the Mac: `tailscale lock status` → enabled, all three nodes (Mac, VPS, dev server) signed |
| g | sshd locked | on the VPS: `& "$env:WINDIR\System32\OpenSSH\sshd.exe" -T \| Select-String passwordauthentication` → `no`; `Get-NetFirewallRule -Name OpenSSH-Server-In-TCP` → `Enabled False`. **Repeat (a) and (g) after each Windows feature update** (an update may re-enable default rules). |

## Break-glass and rollback

- Locked out: use the Contabo VNC console (typing only; it does not accept paste). Check `Get-NetFirewallRule -Name AllWin-Tailnet-*` and `tailscale status`; fix Tailscale first. Re-enabling a public RDP rule is a firewall change that needs the user's explicit decision, and must be undone by re-running `Lock-Firewall.ps1`.
- `sshd_config` is backed up next to itself (`sshd_config.allwin-backup-<timestamp>`) before each change and restored automatically if `sshd -t` fails or if `sshd` fails to start (the service is then set to Disabled and the config goes back to the default, which accepts passwords). After such a failure never start `sshd` by hand: fix the cause and rerun `Install-AllWinTestHost.ps1`.
- The first run of the scripts on Windows is experiment E1. Check that `Lock-Firewall.ps1` runs its audit without an exception (an error there stops the setup, which is the safe direction).
