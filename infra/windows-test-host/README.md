# Windows test host (NinjaTrader 8) — setup, "Fase 2"

Implements `docs/architecture/spikes/nt8-spike-plan.md` §6.3–§6.5. **Status: DRAFT** — the scripts have never run on Windows (first run = experiment E1).
**Do not run any script here until Code Review (Crivo) and QA Security (Vigia) approved the exact commit** (they run as Administrator).

| Item | Value |
|---|---|
| Hostname in the tailnet | `allwin-nt8-01` (tag `tag:allwin-nt8`) |
| Dev server (agents) | tag `tag:allwin-dev`, outbound only |
| Public IP | `<VPS_PUBLIC_IP>` (never written in the repo) |
| Break-glass | Contabo panel → VNC console |
| Agents' SSH public key (below) | fingerprint `SHA256:YW0/0bcG352ZlHrfOJeRx6v6h8GcxduANjeRFRaD99w` |

Agents' **public** key (the private half stays on the dev server, mode 600):

```
ssh-ed25519 AAAAC3NzaC1lZDI1NTE5AAAAIAciPbXwf2lko6zYMDdDKlO+hpJFWiw9GtJRlrUks02k allwin-agents-nt8@dev2
```

Rules: no live accounts on this host; no auth key is ever saved in a file, a log or a chat. Placeholders below look like `<THIS>`.

---

## 0. Before you start (Contabo)

1. Enable 2FA on the Contabo account. Save the Administrator password in your password manager.
2. Contabo panel → your VPS → **VNC console**. Sign in as Administrator. Do **not** use RDP over the public IP.

## 1. Tailscale admin console (browser, on the Mac)

1. Enable 2FA on the Tailscale login.
2. **Access controls** → paste the whole content of `infra/tailscale/policy.hujson` → Save. The built-in `tests` block must pass (the console refuses the save otherwise).
3. **Settings → Keys** → *Generate auth key*, twice. For each: **Reusable off**, **Ephemeral off**, **Pre-approved on**, expiration 1 day, **Tags**:
   - key 1: `tag:allwin-nt8`
   - key 2: `tag:allwin-dev`

   Keep both open only until step 2 below. Never paste them anywhere except the commands shown here.

## 2. Mac — Tailnet Lock and pre-signing (Terminal)

Install Tailscale on the Mac and sign in first (App Store or tailscale.com/download). Then:

```bash
# Enable Tailnet Lock with this Mac as the trusted signer. Save the printed disablement secrets in the password manager.
tailscale lock init --gen-disablements 2
tailscale lock status          # must say: Tailnet lock is ENABLED

# Pre-sign the two auth keys. Use the keys that these commands PRINT, not the originals.
tailscale lock sign <AUTH_KEY_1_NT8>
tailscale lock sign <AUTH_KEY_2_DEV>
```

If `lock init` asks for the trusted key list, follow the command's own prompt and include this Mac's key (shown by `tailscale lock status`). Keep one disablement secret offline so losing the Mac does not lock the tailnet.

Send **only** the signed dev key (key 2) to Forja through the channel the Maestro gives you; never the nt8 one.

## 3. VPS — get the scripts onto the machine

The repository is private. Pick one:

- **A (preferred)**: in the VNC console open PowerShell **as Administrator**, run `notepad C:\AllWin-setup.ps1`, paste the file content (VNC clipboard support is unverified for Contabo), save, and repeat for each script. Save them as `C:\AllWin-setup\Lock-Firewall.ps1` and `C:\AllWin-setup\Install-AllWinTestHost.ps1` (create the folder with `mkdir C:\AllWin-setup`).
- **B**: after step 4 (joined to the tailnet), RDP from the Mac to `allwin-nt8-01` with *Devices & Audio → Drives* shared and copy the two files. Do this only before step 5 and keep the session short.

Check the file hash against the one the Maestro gives you before running:

```powershell
Get-FileHash C:\AllWin-setup\*.ps1 -Algorithm SHA256 | Format-Table Hash, Path
```

## 4. VPS (VNC console, PowerShell as Administrator) — join the tailnet

```powershell
# Download and install Tailscale.
$msi = "$env:TEMP\tailscale.msi"
Invoke-WebRequest https://pkgs.tailscale.com/stable/tailscale-setup-latest-amd64.msi -OutFile $msi
Start-Process msiexec.exe -ArgumentList "/i `"$msi`" /quiet" -Wait

# Join with the SIGNED nt8 key printed by `tailscale lock sign` on the Mac.
& "C:\Program Files\Tailscale\tailscale.exe" up --auth-key=<SIGNED_AUTH_KEY_1_NT8> --hostname=allwin-nt8-01 --advertise-tags=tag:allwin-nt8 --unattended --ssh=false

# Confirm: expect a 100.x address, the node tagged, and (after the Mac signs or the key was pre-signed) not "locked out".
& "C:\Program Files\Tailscale\tailscale.exe" status
& "C:\Program Files\Tailscale\tailscale.exe" ip -4
```

Then clear the key from the console history:

```powershell
Clear-History; Remove-Item (Get-PSReadLineOption).HistorySavePath -ErrorAction SilentlyContinue
```

## 5. VPS — lock RDP and SSH to the tailnet

```powershell
# Dry run first: lists every change, changes nothing.
C:\AllWin-setup\Lock-Firewall.ps1 -WhatIf
# Real run (refuses to change anything unless the Tailscale adapter is up with a 100.x address).
C:\AllWin-setup\Lock-Firewall.ps1
```

From the Mac, RDP only to `allwin-nt8-01` from now on. If you lock yourself out, use the VNC console.

## 6. Mac — check RDP over the tailnet

Open Windows App / Microsoft Remote Desktop → PC name `allwin-nt8-01` → sign in as Administrator (NLA required).

## 7. VPS — users and software (as Administrator, RDP or VNC)

```powershell
# Standard user for the agents (NOT an administrator). You will be asked for a password: store it in the password manager.
$pw = Read-Host -AsSecureString 'Password for allwin-agent'
New-LocalUser -Name allwin-agent -Password $pw -PasswordNeverExpires -AccountNeverExpires -FullName 'All Win agents'
Add-LocalGroupMember -Group 'Remote Desktop Users' -Member allwin-agent
Add-WindowsCapability -Online -Name OpenSSH.Server~~~~0.0.1.0
```

Install by hand: NinjaTrader 8 (8.1.8.4+), .NET SDK 8+, Build Tools for Visual Studio (MSBuild + .NET Framework 4.8 targeting pack), Git for Windows.

## 8. VPS — configure sshd, folders and tasks

```powershell
$key = 'ssh-ed25519 AAAAC3NzaC1lZDI1NTE5AAAAIAciPbXwf2lko6zYMDdDKlO+hpJFWiw9GtJRlrUks02k allwin-agents-nt8@dev2'
C:\AllWin-setup\Install-AllWinTestHost.ps1 -PublicKey $key -WhatIf
C:\AllWin-setup\Install-AllWinTestHost.ps1 -PublicKey $key
```

Logs go to `C:\AllWin\logs`.

## 9. VPS — NinjaTrader first login (RDP as `allwin-agent`)

Sign in to NT8 with the **dedicated development login**; keep only Sim101/Playback and the Simulated Data Feed connections; download MNQ Market Replay data; then **disconnect** the RDP session (do not sign out).

## 10. Dev server (Forja, user already approved U1)

```bash
sudo tailscale up --auth-key=<SIGNED_AUTH_KEY_2_DEV> --advertise-tags=tag:allwin-dev --accept-dns=false --accept-routes=false
```

## 11. T-checks (Forja records them in the spike report)

| | Check | How |
|---|---|---|
| a | public 3389 and 22 closed | from a host outside the tailnet: `nc -zv -w5 <VPS_PUBLIC_IP> 3389`, same for 22 → both fail |
| b | dev server reaches the VPS | `ssh -i ~/.ssh/allwin_agents_nt8_ed25519 allwin-agent@allwin-nt8-01 'echo ok'` |
| c | VPS cannot reach the dev server | on the VPS: `Test-NetConnection <DEV_TAILSCALE_IP> -Port 22` → fails |
| d | Mac reaches RDP | step 6 |
| e | no auth key in any file or log | `Select-String -Path C:\AllWin\logs\*,C:\AllWin-setup\* -Pattern 'tskey-'` → no match |
| f | Tailnet Lock | on the Mac: `tailscale lock status` → enabled, both nodes signed |

## Break-glass and rollback

- Locked out: use the Contabo VNC console. To regain access, run `Get-NetFirewallRule -Name AllWin-Tailnet-*` to check the rules and `tailscale status` to check the node; fix Tailscale first. Re-enabling a public RDP rule is a firewall change that needs the user's explicit decision, and must be undone by re-running `Lock-Firewall.ps1`.
- `sshd_config` is backed up next to itself (`sshd_config.allwin-backup-<timestamp>`) before each change and restored automatically if `sshd -t` fails.
