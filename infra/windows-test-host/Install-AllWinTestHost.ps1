<#
.SYNOPSIS
    Sets up the All Win Windows test host for the agents: hardened sshd, folders, logs, and on-demand NT8 tasks.

.DESCRIPTION
    nt8-spike-plan §6.3 / §6.5 step 10. Run as Administrator on the VPS, after Lock-Firewall.ps1 (run
    AFTER the OpenSSH Server install) and after the standard user `allwin-agent`, Git for Windows and NinjaTrader 8
    are installed. Lock-Firewall.ps1 must sit in the same folder: its -CheckOnly audit runs twice (before the sshd
    config is touched and right before sshd starts) and the script stops if 22/3389 is exposed.
    sshd must still be STOPPED when this script starts (it is never started before the lockdown, QA Security H1);
    the script creates sshd_config (from sshd_config_default) and the host keys (ssh-keygen -A) without starting it.

    It does the following and nothing else:
      1. Checks the preconditions (allwin-agent exists, is enabled and is NOT an administrator; OpenSSH Server,
         Git Bash and the NT8 executable exist). It stops at the first failure.
      2. sshd (backup of sshd_config first; validated with `sshd -t`; restored if the check fails; started only after the firewall audit passes):
         AllowUsers allwin-agent, PasswordAuthentication no, KbdInteractiveAuthentication no, PubkeyAuthentication yes,
         StrictModes yes, DisableForwarding yes, and a dedicated AuthorizedKeysFile owned by SYSTEM/Administrators.
         DefaultShell = Git Bash. The agents' public key is restricted to the tailnet range (from="100.64.0.0/10").
      3. Folders C:\AllWin\{bin,work,logs,screenshots,admin-logs}. admin-logs (this script's and Lock-Firewall's transcripts) is administrators-only. bin is writable by administrators only (the scheduled
         scripts live there, so the agent cannot change what the tasks run); the agent can modify work, logs and
         screenshots.
      4. Scheduled Tasks that run in allwin-agent's INTERACTIVE session (/it equivalent: LogonType Interactive, no
         stored password): AllWin-NT8-Start, AllWin-NT8-Stop, AllWin-Screenshot. The agent may run them
         (schtasks /run /tn <name>) but cannot edit them.

    Only a PUBLIC key is given to this script. No password, token or key is stored, logged or printed
    (the log shows the key fingerprint). Run with -WhatIf first.

    STATUS: DRAFT. Do not run before Code Review (Crivo) and QA Security (Vigia) approved this exact commit.
    Not yet run on Windows: the first run is the spike's experiment E1.

.PARAMETER PublicKey
    One line, the agents' OpenSSH public key (`ssh-ed25519 AAAA... comment`). Either this or -PublicKeyFile.

.PARAMETER PublicKeyFile
    A file holding that single line.

.PARAMETER AgentUser
    The standard Windows user the agents use. Default: allwin-agent.

.PARAMETER GitBashPath
    Default: C:\Program Files\Git\bin\bash.exe.

.PARAMETER Nt8Path
    Default: C:\Program Files\NinjaTrader 8\bin64\NinjaTrader.exe.

.PARAMETER Root
    Default: C:\AllWin.
#>
#Requires -Version 5.1
#Requires -RunAsAdministrator
[CmdletBinding(SupportsShouldProcess = $true, ConfirmImpact = 'High')]
param(
    [string]$PublicKey,
    [string]$PublicKeyFile,
    [string]$AgentUser = 'allwin-agent',
    [string]$GitBashPath = 'C:\Program Files\Git\bin\bash.exe',
    [string]$Nt8Path = 'C:\Program Files\NinjaTrader 8\bin64\NinjaTrader.exe',
    [string]$Root = 'C:\AllWin'
)

Set-StrictMode -Version Latest
$ErrorActionPreference = 'Stop'

$sidSystem = '*S-1-5-18'
$sidAdmins = '*S-1-5-32-544'
$tailnetRange = '100.64.0.0/10'
$sshDir = Join-Path $env:ProgramData 'ssh'
$sshdConfig = Join-Path $sshDir 'sshd_config'
$keysFile = Join-Path $sshDir 'allwin_authorized_keys'
$sshdExe = Join-Path $env:WINDIR 'System32\OpenSSH\sshd.exe'
$logDir = Join-Path $Root 'admin-logs'

if (-not (Test-Path -LiteralPath $logDir)) { New-Item -ItemType Directory -Path $logDir -Force | Out-Null }
# Admin-only: the agent user must not be able to alter or delete the admin audit trail.
& icacls.exe $logDir /inheritance:r /grant:r "${sidSystem}:(OI)(CI)F" "${sidAdmins}:(OI)(CI)F" | Out-Null
if ($LASTEXITCODE -ne 0) { throw "icacls failed on $logDir" }
$logFile = Join-Path $logDir ('Install-AllWinTestHost-{0:yyyyMMdd-HHmmss}.log' -f (Get-Date))
Start-Transcript -Path $logFile | Out-Null

function Test-PublicKeyLine([string]$Line) {
    # A bare ed25519 key plus an optional comment. No options prefix (command=, from=, ...): this script adds its own.
    return $Line -match '^ssh-ed25519 AAAA[0-9A-Za-z+/]{40,}={0,3}( [^\r\n"]*)?$'
}

try {
    # ---- 1. Preconditions ---------------------------------------------------------------------------------------
    if ($PublicKeyFile) {
        if (-not (Test-Path -LiteralPath $PublicKeyFile)) { throw "Public key file not found: $PublicKeyFile" }
        $PublicKey = (Get-Content -LiteralPath $PublicKeyFile -Raw).Trim()
    }
    if (-not $PublicKey) { throw 'Give -PublicKey or -PublicKeyFile (the agents public key, one line).' }
    $PublicKey = $PublicKey.Trim()
    if (-not (Test-PublicKeyLine $PublicKey)) { throw 'The public key must be a single bare "ssh-ed25519 AAAA... comment" line (a public key, not a private one).' }

    $user = Get-LocalUser -Name $AgentUser -ErrorAction SilentlyContinue
    if (-not $user) { throw "Local user '$AgentUser' does not exist (checklist step 9)." }
    if (-not $user.Enabled) { throw "Local user '$AgentUser' is disabled." }
    $isAdmin = Get-LocalGroupMember -SID 'S-1-5-32-544' -ErrorAction SilentlyContinue | Where-Object { $_.SID -eq $user.SID }
    if ($isAdmin) { throw "'$AgentUser' is an administrator. It must be a standard user (nt8-spike-plan §6)." }
    $rdpMember = Get-LocalGroupMember -SID 'S-1-5-32-555' -ErrorAction SilentlyContinue | Where-Object { $_.SID -eq $user.SID }
    if (-not $rdpMember) { Write-Warning "'$AgentUser' is not in 'Remote Desktop Users' (needed for the one-time NT8 login in step 11)." }

    if (-not (Test-Path -LiteralPath $sshdExe)) { throw 'OpenSSH Server is not installed (checklist step 5).' }
    if (-not (Get-Service -Name sshd -ErrorAction SilentlyContinue)) { throw 'The sshd service is missing (checklist step 5).' }
    # sshd must NOT have been started yet (QA Security H1): it would accept password logins on a public port 22.
    $sshdSvc = Get-Service -Name sshd
    if ($sshdSvc.Status -ne 'Stopped') { throw "The sshd service is $($sshdSvc.Status). It must stay stopped until this script has locked it down: stop it, rerun Lock-Firewall.ps1, then rerun this script." }
    # Firewall audit BEFORE any sshd change or start (same audit as Lock-Firewall.ps1 -CheckOnly).
    $lockScript = Join-Path $PSScriptRoot 'Lock-Firewall.ps1'
    if (-not (Test-Path -LiteralPath $lockScript)) { throw "Lock-Firewall.ps1 must sit next to this script ($lockScript)." }
    & $lockScript -CheckOnly
    if (-not (Test-Path -LiteralPath $GitBashPath)) { throw "Git Bash not found at $GitBashPath (checklist step 9)." }
    if (-not (Test-Path -LiteralPath $Nt8Path)) { Write-Warning "NinjaTrader executable not found at $Nt8Path. The tasks are created anyway; fix -Nt8Path if NT8 lives elsewhere." }

    $fingerprint = '(ssh-keygen not found)'
    $keygen = Join-Path $env:WINDIR 'System32\OpenSSH\ssh-keygen.exe'
    if (Test-Path -LiteralPath $keygen) {
        $tmp = New-TemporaryFile
        try { Set-Content -LiteralPath $tmp -Value $PublicKey -Encoding ascii; $fingerprint = (& $keygen -lf $tmp) -join ' ' } finally { Remove-Item -LiteralPath $tmp -Force }
    }
    Write-Host "Agent public key fingerprint: $fingerprint"

    # ---- 2. Folders ---------------------------------------------------------------------------------------------
    $folders = @{ bin = 'RX'; work = 'M'; logs = 'M'; screenshots = 'M' }
    foreach ($name in $folders.Keys) {
        $path = Join-Path $Root $name
        if ($PSCmdlet.ShouldProcess($path, "Create and set ACL (SYSTEM/Administrators full; $AgentUser $($folders[$name]))")) {
            if (-not (Test-Path -LiteralPath $path)) { New-Item -ItemType Directory -Path $path -Force | Out-Null }
            & icacls.exe $path /setowner $sidAdmins /T /C | Out-Null
            & icacls.exe $path /reset /T /C | Out-Null
            & icacls.exe $path /inheritance:r /grant:r "${sidSystem}:(OI)(CI)F" "${sidAdmins}:(OI)(CI)F" "${AgentUser}:(OI)(CI)$($folders[$name])" | Out-Null
            if ($LASTEXITCODE -ne 0) { throw "icacls failed on $path" }
        }
    }
    if ($PSCmdlet.ShouldProcess($Root, 'Set ACL on the root folder')) {
        & icacls.exe $Root /setowner $sidAdmins | Out-Null
        & icacls.exe $Root /reset | Out-Null
        & icacls.exe $Root /inheritance:r /grant:r "${sidSystem}:(OI)(CI)F" "${sidAdmins}:(OI)(CI)F" "${AgentUser}:(RX)" | Out-Null
        if ($LASTEXITCODE -ne 0) { throw "icacls failed on $Root" }
    }

    # ---- 3. Scripts run by the tasks (in bin, administrators only can edit) --------------------------------------
    $startScript = @"
`$ErrorActionPreference = 'Stop'
`$exe = '$($Nt8Path.Replace("'", "''"))'
if (Get-Process -Name NinjaTrader -ErrorAction SilentlyContinue) { 'NinjaTrader is already running.'; exit 0 }
Start-Process -FilePath `$exe
'NinjaTrader started.'
"@
    $stopScript = @"
`$ErrorActionPreference = 'Stop'
`$procs = Get-Process -Name NinjaTrader -ErrorAction SilentlyContinue
if (-not `$procs) { 'NinjaTrader is not running.'; exit 0 }
`$procs | ForEach-Object { `$null = `$_.CloseMainWindow() }
`$deadline = (Get-Date).AddSeconds(30)
while ((Get-Process -Name NinjaTrader -ErrorAction SilentlyContinue) -and (Get-Date) -lt `$deadline) { Start-Sleep -Seconds 1 }
if (Get-Process -Name NinjaTrader -ErrorAction SilentlyContinue) {
    Get-Process -Name NinjaTrader -ErrorAction SilentlyContinue | Stop-Process -Force
    'NinjaTrader did not close in 30 s and was stopped.'
} else { 'NinjaTrader closed.' }
"@
    $shotScript = @"
`$ErrorActionPreference = 'Stop'
Add-Type -AssemblyName System.Windows.Forms, System.Drawing
`$b = [System.Windows.Forms.SystemInformation]::VirtualScreen
`$bmp = New-Object System.Drawing.Bitmap `$b.Width, `$b.Height
try {
    `$g = [System.Drawing.Graphics]::FromImage(`$bmp)
    `$g.CopyFromScreen(`$b.Left, `$b.Top, 0, 0, `$bmp.Size)
    `$out = Join-Path '$((Join-Path $Root 'screenshots').Replace("'", "''"))' ('screen-{0:yyyyMMdd-HHmmss}.png' -f (Get-Date))
    `$bmp.Save(`$out, [System.Drawing.Imaging.ImageFormat]::Png)
    `$out
} finally { `$bmp.Dispose() }
"@
    $scripts = @{ 'Start-NT8.ps1' = $startScript; 'Stop-NT8.ps1' = $stopScript; 'Take-Screenshot.ps1' = $shotScript }
    foreach ($name in $scripts.Keys) {
        $path = Join-Path (Join-Path $Root 'bin') $name
        if ($PSCmdlet.ShouldProcess($path, 'Write task script')) { Set-Content -LiteralPath $path -Value $scripts[$name] -Encoding UTF8 }
    }

    # ---- 4. sshd ------------------------------------------------------------------------------------------------
    if ($PSCmdlet.ShouldProcess($keysFile, "Write the agents public key (from=$tailnetRange) with owner-only ACL")) {
        Set-Content -LiteralPath $keysFile -Value ('from="{0}",no-agent-forwarding,no-X11-forwarding {1}' -f $tailnetRange, $PublicKey) -Encoding ascii
        & icacls.exe $keysFile /inheritance:r /grant:r "${sidSystem}:F" "${sidAdmins}:F" "${AgentUser}:R" | Out-Null
        if ($LASTEXITCODE -ne 0) { throw "icacls failed on $keysFile" }
    }

    $backup = $null
    $begin = '# BEGIN AllWin'
    $end = '# END AllWin'
    $block = @(
        $begin,
        "AllowUsers $AgentUser",
        'PasswordAuthentication no',
        'KbdInteractiveAuthentication no',
        'PubkeyAuthentication yes',
        'AuthorizedKeysFile __PROGRAMDATA__/ssh/allwin_authorized_keys',
        'StrictModes yes',
        'MaxAuthTries 3',
        'LoginGraceTime 30',
        'AllowTcpForwarding no',
        'AllowAgentForwarding no',
        'X11Forwarding no',
        'PermitTunnel no',
        'GatewayPorts no',
        'DisableForwarding yes',
        $end
    )
    if ($PSCmdlet.ShouldProcess($sshdConfig, 'Create sshd_config and host keys if missing (without starting sshd)')) {
        if (-not (Test-Path -LiteralPath $sshDir)) { New-Item -ItemType Directory -Path $sshDir -Force | Out-Null }
        if (-not (Test-Path -LiteralPath $sshdConfig)) {
            $default = Join-Path $env:WINDIR 'System32\OpenSSH\sshd_config_default'
            if (-not (Test-Path -LiteralPath $default)) { throw "$default not found; cannot create $sshdConfig." }
            Copy-Item -LiteralPath $default -Destination $sshdConfig
        }
        & (Join-Path $env:WINDIR 'System32\OpenSSH\ssh-keygen.exe') -A
        if ($LASTEXITCODE -ne 0) { throw 'ssh-keygen -A failed.' }
    }
    if ($PSCmdlet.ShouldProcess($sshdConfig, 'Apply the AllWin sshd block (backup, validate with sshd -t; sshd is started only after the firewall audit)')) {
        $backup = "$sshdConfig.allwin-backup-{0:yyyyMMdd-HHmmss}" -f (Get-Date)
        Copy-Item -LiteralPath $sshdConfig -Destination $backup
        $lines = New-Object System.Collections.Generic.List[string]
        $skip = $false
        foreach ($l in (Get-Content -LiteralPath $sshdConfig)) {
            if ($l -eq $begin) { $skip = $true; continue }
            if ($l -eq $end) { $skip = $false; continue }
            if (-not $skip) { $lines.Add($l) }
        }
        # Directives placed after a "Match" line would be conditional, so the block goes before the first Match.
        $at = $lines.Count
        for ($i = 0; $i -lt $lines.Count; $i++) { if ($lines[$i] -match '^\s*Match\s') { $at = $i; break } }
        # sshd uses the first value it sees for a keyword, so comment any active line for the same keys above the block.
        $keys = 'AllowUsers', 'PasswordAuthentication', 'KbdInteractiveAuthentication', 'PubkeyAuthentication', 'AuthorizedKeysFile', 'StrictModes', 'MaxAuthTries', 'LoginGraceTime', 'AllowTcpForwarding', 'AllowAgentForwarding', 'X11Forwarding', 'PermitTunnel', 'GatewayPorts', 'DisableForwarding'
        for ($i = 0; $i -lt $at; $i++) {
            foreach ($k in $keys) { if ($lines[$i] -match "^\s*$k\s") { $lines[$i] = "#AllWin-disabled# $($lines[$i])" } }
        }
        $lines.InsertRange($at, [string[]]$block)
        Set-Content -LiteralPath $sshdConfig -Value $lines -Encoding ascii
        & $sshdExe -t
        if ($LASTEXITCODE -ne 0) {
            Copy-Item -LiteralPath $backup -Destination $sshdConfig -Force
            throw "sshd -t rejected the new configuration; the original was restored from $backup."
        }
        $shellKey = 'HKLM:\SOFTWARE\OpenSSH'
        if (-not (Test-Path $shellKey)) { New-Item -Path $shellKey -Force | Out-Null }
        New-ItemProperty -Path $shellKey -Name DefaultShell -Value $GitBashPath -PropertyType String -Force | Out-Null
        New-ItemProperty -Path $shellKey -Name DefaultShellCommandOption -Value '-c' -PropertyType String -Force | Out-Null
        Write-Host "sshd_config written and validated (sshd not started yet). Previous config: $backup"
    }

    # Start sshd only now: the firewall audit must pass first, and again right before the service starts.
    if ($PSCmdlet.ShouldProcess('sshd', 'Re-run the firewall audit, then set Automatic start and start the service')) {
        & $lockScript -CheckOnly
        Set-Service -Name sshd -StartupType Automatic
        try { Start-Service -Name sshd }
        catch {
            Set-Service -Name sshd -StartupType Disabled
            if ($backup -and (Test-Path -LiteralPath $backup)) { Copy-Item -LiteralPath $backup -Destination $sshdConfig -Force }
            throw "sshd failed to start; it is disabled again and the previous config was restored. $($_.Exception.Message)"
        }
        $hostPub = Join-Path $sshDir 'ssh_host_ed25519_key.pub'
        Write-Host ''
        Write-Host 'SSH host key for pinning on the dev server known_hosts (public values only; T-check b):'
        if (Test-Path -LiteralPath $hostPub) {
            Write-Host ("  fingerprint: " + ((& (Join-Path $env:WINDIR 'System32\OpenSSH\ssh-keygen.exe') -lf $hostPub) -join ' '))
            Write-Host ("  known_hosts line: allwin-nt8-01 " + ((Get-Content -LiteralPath $hostPub -Raw).Trim() -replace ' [^ ]*$', ''))
        }
    }

    # ---- 5. Scheduled Tasks in the agent's interactive session ----------------------------------------------------
    $principal = New-ScheduledTaskPrincipal -UserId ("{0}\{1}" -f $env:COMPUTERNAME, $AgentUser) -LogonType Interactive -RunLevel Limited
    $settings = New-ScheduledTaskSettingsSet -ExecutionTimeLimit (New-TimeSpan -Minutes 5) -MultipleInstances IgnoreNew -StartWhenAvailable
    $tasks = @(
        @{ Name = 'AllWin-NT8-Start'; Script = 'Start-NT8.ps1' },
        @{ Name = 'AllWin-NT8-Stop'; Script = 'Stop-NT8.ps1' },
        @{ Name = 'AllWin-Screenshot'; Script = 'Take-Screenshot.ps1' }
    )
    foreach ($t in $tasks) {
        if ($PSCmdlet.ShouldProcess($t.Name, "Register the task for $AgentUser (interactive session only)")) {
            $scriptPath = Join-Path (Join-Path $Root 'bin') $t.Script
            $action = New-ScheduledTaskAction -Execute 'powershell.exe' -Argument ('-NoProfile -ExecutionPolicy Bypass -File "{0}"' -f $scriptPath)
            Register-ScheduledTask -TaskName $t.Name -Action $action -Principal $principal -Settings $settings -Force | Out-Null
            # Let the agent run (not edit) the task: add read+execute for its SID to the task's security descriptor.
            $svc = New-Object -ComObject 'Schedule.Service'
            $svc.Connect()
            $task = $svc.GetFolder('\').GetTask($t.Name)
            # DACL only (4): the full descriptor (0xF) would include the SACL. Insert the ACE through the object model.
            $rsd = New-Object System.Security.AccessControl.RawSecurityDescriptor($task.GetSecurityDescriptor(4))
            $sidObj = $user.SID
            $has = @($rsd.DiscretionaryAcl | Where-Object { $_.SecurityIdentifier -eq $sidObj -and $_.AceQualifier -eq 'AccessAllowed' }).Count -gt 0
            if (-not $has) {
                # 0x1200A9 = SDDL FRFX (read + execute): the agent can run the task, not change it.
                $ace = New-Object System.Security.AccessControl.CommonAce([System.Security.AccessControl.AceFlags]::None, [System.Security.AccessControl.AceQualifier]::AccessAllowed, 0x1200A9, $sidObj, $false, $null)
                $rsd.DiscretionaryAcl.InsertAce($rsd.DiscretionaryAcl.Count, $ace)
                $task.SetSecurityDescriptor($rsd.GetSddlForm([System.Security.AccessControl.AccessControlSections]::Access), 0)
            }
            $check = New-Object System.Security.AccessControl.RawSecurityDescriptor($svc.GetFolder('\').GetTask($t.Name).GetSecurityDescriptor(4))
            if (-not (@($check.DiscretionaryAcl | Where-Object { $_.SecurityIdentifier -eq $sidObj -and $_.AceQualifier -eq 'AccessAllowed' }).Count)) { throw "Task ACL for $($t.Name) does not contain $AgentUser after the update." }
        }
    }

    Write-Host ''
    Write-Host 'Done. Still to do by hand (checklist step 11): sign in to NT8 as allwin-agent over RDP, keep only Sim101/Playback and'
    Write-Host 'the Simulated Data Feed connections, then DISCONNECT (do not sign out) so the interactive tasks have a session.'
    Write-Host "Verify from the dev server: ssh $AgentUser@allwin-nt8-01 'echo ok'   and, as $AgentUser: schtasks /run /tn AllWin-Screenshot"
    Write-Host "Log: $logFile"
}
finally {
    Stop-Transcript | Out-Null
}
