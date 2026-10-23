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
      3. Disables every other enabled inbound allow rule that opens TCP/UDP 3389 or 22 (the default Remote Desktop
         rules, OpenSSH-Server-In-TCP, and anything else found).
      4. Makes sure the firewall is on and the default inbound action is Block for all profiles.
      5. Requires Network Level Authentication (NLA) for RDP.

    It does not touch Tailscale, sshd, users, or any other rule. It stores and prints no secret.
    Run with -WhatIf first to see every change without making it.

    Break-glass if everyone is locked out: the Contabo panel VNC console (nt8-spike-plan §6.3).

    STATUS: DRAFT. Do not run before Code Review (Crivo) and QA Security (Vigia) approved this exact commit.

.PARAMETER TailscaleInterfaceAlias
    Alias of the Tailscale network adapter. Default: Tailscale.

.PARAMETER TailnetRange
    Remote address range allowed in. Default: 100.64.0.0/10 (the Tailscale CGNAT range).

.PARAMETER LogDir
    Folder for the transcript log. Default: C:\AllWin\logs.
#>
#Requires -Version 5.1
#Requires -RunAsAdministrator
[CmdletBinding(SupportsShouldProcess = $true, ConfirmImpact = 'High')]
param(
    [string]$TailscaleInterfaceAlias = 'Tailscale',
    [string]$TailnetRange = '100.64.0.0/10',
    [string]$LogDir = 'C:\AllWin\logs'
)

Set-StrictMode -Version Latest
$ErrorActionPreference = 'Stop'

$ruleRdp = 'AllWin-Tailnet-RDP-TCP-3389'
$ruleSsh = 'AllWin-Tailnet-SSH-TCP-22'
$managedPorts = @('3389', '22')

if (-not (Test-Path -LiteralPath $LogDir)) { New-Item -ItemType Directory -Path $LogDir -Force | Out-Null }
$logFile = Join-Path $LogDir ('Lock-Firewall-{0:yyyyMMdd-HHmmss}.log' -f (Get-Date))
Start-Transcript -Path $logFile | Out-Null

try {
    # 1. The tailnet must already be reachable, otherwise locking the public rules would cut the user off.
    $adapter = Get-NetAdapter -InterfaceAlias $TailscaleInterfaceAlias -ErrorAction SilentlyContinue
    if (-not $adapter -or $adapter.Status -ne 'Up') {
        throw "The '$TailscaleInterfaceAlias' network adapter is missing or not up. Join the tailnet first (checklist step 4). Nothing was changed."
    }
    $tailnetAddr = Get-NetIPAddress -InterfaceAlias $TailscaleInterfaceAlias -AddressFamily IPv4 -ErrorAction SilentlyContinue |
        Where-Object { $_.IPAddress -match '^100\.(6[4-9]|[7-9][0-9]|1[01][0-9]|12[0-7])\.' } | Select-Object -First 1
    if (-not $tailnetAddr) {
        throw "The '$TailscaleInterfaceAlias' adapter has no address in $TailnetRange. Check 'tailscale status'. Nothing was changed."
    }
    Write-Host "Tailscale adapter is up with address $($tailnetAddr.IPAddress)."

    # 2. Create (or replace) the tailnet-scoped allow rules first.
    foreach ($spec in @(@{ Name = $ruleRdp; Port = 3389; Label = 'RDP' }, @{ Name = $ruleSsh; Port = 22; Label = 'SSH' })) {
        if ($PSCmdlet.ShouldProcess($spec.Name, "Create inbound allow TCP $($spec.Port) from $TailnetRange on $TailscaleInterfaceAlias")) {
            Get-NetFirewallRule -Name $spec.Name -ErrorAction SilentlyContinue | Remove-NetFirewallRule
            New-NetFirewallRule -Name $spec.Name -DisplayName "All Win tailnet $($spec.Label) (TCP $($spec.Port))" `
                -Direction Inbound -Action Allow -Protocol TCP -LocalPort $spec.Port `
                -RemoteAddress $TailnetRange -InterfaceAlias $TailscaleInterfaceAlias -Profile Any -Enabled True | Out-Null
        }
    }
    if (-not $WhatIfPreference) {
        foreach ($n in @($ruleRdp, $ruleSsh)) {
            $r = Get-NetFirewallRule -Name $n -ErrorAction SilentlyContinue
            if (-not $r -or $r.Enabled -ne 'True') { throw "Rule $n was not created or is not enabled. The public rules were NOT touched." }
        }
    }

    # 3. Disable every other enabled inbound allow rule that opens 3389 or 22.
    $disabled = @()
    $candidates = Get-NetFirewallRule -Direction Inbound -Action Allow -Enabled True |
        Where-Object { $_.Name -ne $ruleRdp -and $_.Name -ne $ruleSsh }
    foreach ($rule in $candidates) {
        $ports = @(Get-NetFirewallPortFilter -AssociatedNetFirewallRule $rule |
            ForEach-Object { $_.LocalPort } | ForEach-Object { $_ })
        $opens = $false
        foreach ($p in $ports) { if ($managedPorts -contains [string]$p) { $opens = $true } }
        if ($rule.Name -like 'RemoteDesktop-*' -or $rule.Name -like 'OpenSSH-Server-*') { $opens = $true }
        if ($opens -and $PSCmdlet.ShouldProcess($rule.Name, 'Disable (opens RDP/SSH outside the tailnet scope)')) {
            Disable-NetFirewallRule -Name $rule.Name
            $disabled += $rule.Name
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

    # Summary (no secrets).
    Write-Host ''
    Write-Host 'Disabled rules:' ($(if ($disabled) { $disabled -join ', ' } else { '(none found)' }))
    Write-Host 'Enabled inbound allow rules that open 3389/22 now:'
    Get-NetFirewallRule -Direction Inbound -Action Allow -Enabled True | ForEach-Object {
        $ports = @(Get-NetFirewallPortFilter -AssociatedNetFirewallRule $_ | ForEach-Object { $_.LocalPort } | ForEach-Object { $_ })
        if ($ports | Where-Object { $managedPorts -contains [string]$_ }) {
            $addr = (Get-NetFirewallAddressFilter -AssociatedNetFirewallRule $_).RemoteAddress
            Write-Host ("  {0}  ports={1}  remote={2}" -f $_.Name, ($ports -join ','), ($addr -join ','))
        }
    }
    Write-Host ''
    Write-Host 'Next: from OUTSIDE the tailnet, confirm ports 3389 and 22 on the public IP are closed (T-check a).'
    Write-Host "Log: $logFile"
}
finally {
    Stop-Transcript | Out-Null
}
