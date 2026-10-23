<#
.SYNOPSIS
    Locks RDP (3389) and SSH (22) on the All Win Windows test host to the Tailscale network only.

.DESCRIPTION
    nt8-spike-plan §6.3 / §6.5 step 5. Run as Administrator on the VPS, AFTER the host has joined the tailnet
    (the "Tailscale" network adapter must be up and hold a 100.64.0.0/10 address).

    What it does, in this order (it never removes access before the replacement exists):
      1. Checks that the Tailscale adapter is up and has a tailnet address. It stops otherwise.
      2. Creates two inbound allow rules, scoped to RemoteAddress 100.64.0.0/10 on the Tailscale interface only:
         AllWin-Tailnet-RDP-TCP-3389 and AllWin-Tailnet-SSH-TCP-22.
      3. Disables every other enabled inbound allow rule that opens TCP/UDP 3389 or 22 to a remote address outside the
         tailnet range (the default Remote Desktop rules, OpenSSH-Server-In-TCP, and anything else found).
      4. Makes sure the firewall is on and the default inbound action is Block for all profiles.
      5. Requires Network Level Authentication (NLA) for RDP.

    It then audits the ACTIVE policy store and FAILS LOUDLY if any enabled inbound allow rule still exposes 3389 or 22
    outside the tailnet range, including rules with port Any or a port range that covers them (those are reported, not
    auto-disabled; a rule scoped to a specific program or service is only a warning).

    Run it AGAIN after installing the OpenSSH Server capability (that install creates OpenSSH-Server-In-TCP for any remote
    address) and before sshd is ever started. It is idempotent: existing AllWin rules are updated in place.
    -CheckOnly runs just the audit (no changes, no Tailscale check); Install-AllWinTestHost.ps1 uses it before starting sshd.

    It does not touch Tailscale, sshd, users, or any other rule. It stores and prints no secret.
    Run with -WhatIf first to see every change without making it.

    Break-glass if everyone is locked out: the Contabo panel VNC console (nt8-spike-plan §6.3).

    STATUS: DRAFT. Do not run before Code Review (Crivo) and QA Security (Vigia) approved this exact commit.

.PARAMETER TailscaleInterfaceAlias
    Alias of the Tailscale network adapter. Default: Tailscale.

.PARAMETER TailnetRange
    Remote address range allowed in. Default: 100.64.0.0/10 (the Tailscale CGNAT range).

.PARAMETER LogDir
    Admin-only folder for the transcript log (SYSTEM and Administrators; the agent user has no access).
    Default: C:\AllWin\admin-logs.

.PARAMETER CheckOnly
    Only audit the active firewall policy and fail if 3389 or 22 is exposed outside the tailnet range.
#>
#Requires -Version 5.1
#Requires -RunAsAdministrator
[CmdletBinding(SupportsShouldProcess = $true, ConfirmImpact = 'High')]
param(
    [string]$TailscaleInterfaceAlias = 'Tailscale',
    [string]$TailnetRange = '100.64.0.0/10',
    [string]$LogDir = 'C:\AllWin\admin-logs',
    [switch]$CheckOnly
)

Set-StrictMode -Version Latest
$ErrorActionPreference = 'Stop'

$ruleRdp = 'AllWin-Tailnet-RDP-TCP-3389'
$ruleSsh = 'AllWin-Tailnet-SSH-TCP-22'
$managedPorts = @(22, 3389)

function ConvertTo-UInt32([string]$Ip) {
    $b = ([System.Net.IPAddress]::Parse($Ip)).GetAddressBytes()
    if ($b.Length -ne 4) { throw "Not an IPv4 address: $Ip" }
    return ([uint32]$b[0] * 16777216) + ([uint32]$b[1] * 65536) + ([uint32]$b[2] * 256) + [uint32]$b[3]
}

function Get-Cidr([string]$Text) {
    # Returns @{ Lo; Hi } for "a.b.c.d", "a.b.c.d/nn" or "a.b.c.d/m.m.m.m"; $null for anything else (Any, LocalSubnet, ranges...).
    if ($Text -notmatch '^(\d{1,3}(\.\d{1,3}){3})(/(\d{1,2}|\d{1,3}(\.\d{1,3}){3}))?$') { return $null }
    $ip = ConvertTo-UInt32 $Matches[1]
    $mask = [uint32]::MaxValue
    if ($Matches[4]) {
        if ($Matches[4] -match '^\d{1,2}$') {
            $n = [int]$Matches[4]
            if ($n -gt 32) { return $null }
            $mask = if ($n -eq 0) { [uint32]0 } else { [uint32]([uint32]::MaxValue -shl (32 - $n)) }
        } else { $mask = ConvertTo-UInt32 $Matches[4] }
    }
    $lo = $ip -band $mask
    return @{ Lo = $lo; Hi = ($lo -bor ([uint32]::MaxValue -bxor $mask)) }
}

function Test-RemoteInsideTailnet($Addresses) {
    # True only if EVERY remote address entry lies inside $TailnetRange. Any / * / LocalSubnet / names => false.
    $tail = Get-Cidr $TailnetRange
    foreach ($a in @($Addresses)) {
        $c = Get-Cidr ([string]$a)
        if (-not $c -or $c.Lo -lt $tail.Lo -or $c.Hi -gt $tail.Hi) { return $false }
    }
    return $true
}

function Get-PortCoverage($Ports) {
    # 'literal' = lists 22/3389 explicitly, 'broad' = Any or a range that covers one of them, $null = does not cover.
    $kind = $null
    foreach ($p in @($Ports)) {
        $t = ([string]$p).Trim()
        if ($t -eq 'Any' -or $t -eq '*') { return 'broad' }
        if ($t -match '^(\d+)-(\d+)$') {
            foreach ($m in $managedPorts) { if ([int]$Matches[1] -le $m -and $m -le [int]$Matches[2]) { return 'broad' } }
        } elseif ($t -match '^\d+$') {
            if ($managedPorts -contains [int]$t) { $kind = 'literal' }
        }
    }
    return $kind
}

function Get-ExposedRule {
    # Enabled inbound allow rules in the ACTIVE store (local + group policy) that expose 22/3389 outside the tailnet.
    Get-NetFirewallRule -PolicyStore ActiveStore -Direction Inbound -Action Allow -Enabled True |
        Where-Object { $_.Name -ne $ruleRdp -and $_.Name -ne $ruleSsh } |
        ForEach-Object {
            $rule = $_
            $pf = @(Get-NetFirewallPortFilter -AssociatedNetFirewallRule $rule)
            $af = @(Get-NetFirewallAddressFilter -AssociatedNetFirewallRule $rule)
            $app = @(Get-NetFirewallApplicationFilter -AssociatedNetFirewallRule $rule)
            $svc = @(Get-NetFirewallServiceFilter -AssociatedNetFirewallRule $rule)
            $protocols = @($pf | ForEach-Object { [string]$_.Protocol })
            $tcpUdp = ($protocols.Count -eq 0) -or ($protocols | Where-Object { $_ -in 'TCP', 'UDP', 'Any', '6', '17' })
            $ports = @($pf | ForEach-Object { $_.LocalPort } | ForEach-Object { $_ })
            $kind = if ($tcpUdp) { Get-PortCoverage $ports } else { $null }
            if ($rule.Name -like 'RemoteDesktop-*' -or $rule.Name -like 'OpenSSH-Server-*') { if (-not $kind) { $kind = 'literal' } }
            if (-not $kind) { return }
            $remote = @($af | ForEach-Object { $_.RemoteAddress } | ForEach-Object { $_ })
            if (Test-RemoteInsideTailnet $remote) { return }
            $scoped = ($app | Where-Object { [string]$_.Program -ne 'Any' }) -or ($svc | Where-Object { [string]$_.Service -ne 'Any' })
            [pscustomobject]@{ Name = $rule.Name; Kind = $kind; Scoped = [bool]$scoped; Ports = ($ports -join ','); Remote = ($remote -join ',') }
        }
}

function Assert-NoPublicExposure {
    # The AllWin allow rules, if present, must be scoped to the Tailscale interface and the tailnet range.
    foreach ($n in @($ruleRdp, $ruleSsh)) {
        $r = Get-NetFirewallRule -PolicyStore ActiveStore -Name $n -ErrorAction SilentlyContinue
        if (-not $r) { continue }
        $ifc = @(Get-NetFirewallInterfaceFilter -AssociatedNetFirewallRule $r | ForEach-Object { $_.InterfaceAlias } | ForEach-Object { $_ })
        $rem = @(Get-NetFirewallAddressFilter -AssociatedNetFirewallRule $r | ForEach-Object { $_.RemoteAddress } | ForEach-Object { $_ })
        if (($ifc -join ',') -ne $TailscaleInterfaceAlias -or -not (Test-RemoteInsideTailnet $rem)) {
            throw "Rule $n is not scoped to interface '$TailscaleInterfaceAlias' and $TailnetRange (interface=$($ifc -join ','), remote=$($rem -join ','))."
        }
    }
    $exposed = @(Get-ExposedRule)
    foreach ($e in ($exposed | Where-Object { $_.Scoped })) {
        Write-Warning ("Program/service-scoped rule '{0}' (ports={1}, remote={2}) may cover 22/3389. Review it by hand." -f $e.Name, $e.Ports, $e.Remote)
    }
    $bad = @($exposed | Where-Object { -not $_.Scoped })
    if ($bad.Count -gt 0) {
        $bad | ForEach-Object { Write-Host ("  EXPOSED: {0}  ports={1}  remote={2}  ({3})" -f $_.Name, $_.Ports, $_.Remote, $_.Kind) }
        throw 'Inbound allow rules still expose TCP/UDP 22 or 3389 outside the tailnet range (listed above). Fix them, then rerun.'
    }
    Write-Host 'Audit OK: no enabled inbound allow rule exposes 22 or 3389 outside the tailnet range (active policy store).'
}

if ($CheckOnly) {
    Assert-NoPublicExposure
    return
}

# Admin-only log folder: the agent user must not be able to alter or delete the admin audit trail.
if (-not (Test-Path -LiteralPath $LogDir)) { New-Item -ItemType Directory -Path $LogDir -Force | Out-Null }
& icacls.exe $LogDir /inheritance:r /grant:r '*S-1-5-18:(OI)(CI)F' '*S-1-5-32-544:(OI)(CI)F' | Out-Null
if ($LASTEXITCODE -ne 0) { throw "icacls failed on $LogDir" }
$logFile = Join-Path $LogDir ('Lock-Firewall-{0:yyyyMMdd-HHmmss}.log' -f (Get-Date))
Start-Transcript -Path $logFile | Out-Null

try {
    # 1. The tailnet must already be reachable, otherwise locking the public rules would cut the user off.
    $adapter = Get-NetAdapter -InterfaceAlias $TailscaleInterfaceAlias -ErrorAction SilentlyContinue
    if (-not $adapter -or $adapter.Status -ne 'Up') {
        throw "The '$TailscaleInterfaceAlias' network adapter is missing or not up. Join the tailnet first (checklist step 4). Nothing was changed."
    }
    $tailnetAddr = Get-NetIPAddress -InterfaceAlias $TailscaleInterfaceAlias -AddressFamily IPv4 -ErrorAction SilentlyContinue |
        Where-Object { Test-RemoteInsideTailnet $_.IPAddress } | Select-Object -First 1
    if (-not $tailnetAddr) {
        throw "The '$TailscaleInterfaceAlias' adapter has no address in $TailnetRange. Check 'tailscale status'. Nothing was changed."
    }
    Write-Host "Tailscale adapter is up with address $($tailnetAddr.IPAddress)."

    # 2. Create or update (in place, never remove first) the tailnet-scoped allow rules.
    foreach ($spec in @(@{ Name = $ruleRdp; Port = 3389; Label = 'RDP' }, @{ Name = $ruleSsh; Port = 22; Label = 'SSH' })) {
        if ($PSCmdlet.ShouldProcess($spec.Name, "Create or update inbound allow TCP $($spec.Port) from $TailnetRange on $TailscaleInterfaceAlias")) {
            $display = "All Win tailnet $($spec.Label) (TCP $($spec.Port))"
            if (Get-NetFirewallRule -Name $spec.Name -ErrorAction SilentlyContinue) {
                Set-NetFirewallRule -Name $spec.Name -NewDisplayName $display -Action Allow -Protocol TCP -LocalPort $spec.Port `
                    -RemoteAddress $TailnetRange -InterfaceAlias $TailscaleInterfaceAlias -Profile Any -Enabled True
            } else {
                New-NetFirewallRule -Name $spec.Name -DisplayName $display `
                    -Direction Inbound -Action Allow -Protocol TCP -LocalPort $spec.Port `
                    -RemoteAddress $TailnetRange -InterfaceAlias $TailscaleInterfaceAlias -Profile Any -Enabled True | Out-Null
            }
        }
    }
    if (-not $WhatIfPreference) {
        foreach ($n in @($ruleRdp, $ruleSsh)) {
            $r = Get-NetFirewallRule -Name $n -ErrorAction SilentlyContinue
            if (-not $r -or $r.Enabled -ne 'True') { throw "Rule $n was not created or is not enabled. The public rules were NOT touched." }
        }
    }

    # 3. Disable every enabled inbound allow rule that lists 22/3389 explicitly outside the tailnet scope.
    #    Rules with port Any or a range are NOT auto-disabled (they may be unrelated); the audit below fails on them.
    $disabled = @()
    foreach ($e in @(Get-ExposedRule | Where-Object { $_.Kind -eq 'literal' })) {
        if ($PSCmdlet.ShouldProcess($e.Name, 'Disable (opens RDP/SSH outside the tailnet scope)')) {
            try { Disable-NetFirewallRule -Name $e.Name; $disabled += $e.Name }
            catch { Write-Warning "Could not disable $($e.Name) (group policy rule?): $($_.Exception.Message)" }
        }
    }

    # 4. Firewall on, default inbound Block, for every profile.
    if ($PSCmdlet.ShouldProcess('All firewall profiles', 'Enable and set DefaultInboundAction Block')) {
        Set-NetFirewallProfile -All -Enabled True -DefaultInboundAction Block
    }

    # 5. NLA for RDP (UserAuthentication = 1). Leaves RDP itself enabled.
    $rdpTcp = 'HKLM:\SYSTEM\CurrentControlSet\Control\Terminal Server\WinStations\RDP-Tcp'
    if ($PSCmdlet.ShouldProcess($rdpTcp, 'Set UserAuthentication = 1 (NLA required)')) {
        Set-ItemProperty -Path $rdpTcp -Name UserAuthentication -Value 1 -Type DWord
    }

    # 6. Audit the active store; fails loudly (after the changes, so the summary shows what is left).
    Write-Host ''
    Write-Host 'Disabled rules:' ($(if ($disabled) { $disabled -join ', ' } else { '(none found)' }))
    if (-not $WhatIfPreference) { Assert-NoPublicExposure }
    Write-Host ''
    Write-Host 'Next: from OUTSIDE the tailnet, confirm ports 3389 and 22 on the public IP are closed (T-check a).'
    Write-Host "Log: $logFile"
}
finally {
    Stop-Transcript | Out-Null
}
