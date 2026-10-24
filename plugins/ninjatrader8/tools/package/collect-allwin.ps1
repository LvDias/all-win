# All Win — fallback diagnostics collection (when the AddOn did not compile or load).
# Allowlist (ADR-0005 N12): only %LOCALAPPDATA%\AllWin\logs and \journal, plus an inventory of the installed All Win files.
# Never collects token or consent files, NT8's own log/trace, or the NT8 database.
$ErrorActionPreference = 'Continue'
$stamp = (Get-Date).ToUniversalTime().ToString('yyyyMMddTHHmmssZ')
$aw = Join-Path $env:LOCALAPPDATA 'AllWin'
$nt = Join-Path ([Environment]::GetFolderPath('MyDocuments')) 'NinjaTrader 8'
$out = Join-Path ([Environment]::GetFolderPath('Desktop')) "AllWin-diag-collect-$stamp"
New-Item -ItemType Directory -Force -Path $out | Out-Null

function Copy-Safe($from, $to, $filter) {
    if (-not (Test-Path $from)) { return }
    New-Item -ItemType Directory -Force -Path $to | Out-Null
    Get-ChildItem -Path $from -File -Filter $filter -ErrorAction SilentlyContinue |
        Where-Object { $_.Name -notlike '*.token' -and $_.Name -notlike '*consent*' } |
        ForEach-Object {
            try {
                $src = [IO.File]::Open($_.FullName, 'Open', 'Read', 'ReadWrite, Delete')
                $dst = [IO.File]::Create((Join-Path $to $_.Name))
                $src.CopyTo($dst); $dst.Close(); $src.Close()
            } catch { "copy failed: $($_.Exception.Message)" | Add-Content (Join-Path $out 'collect-errors.txt') }
        }
}

Copy-Safe (Join-Path $aw 'logs') (Join-Path $out 'allwin-logs') '*'
Copy-Safe (Join-Path $aw 'journal') (Join-Path $out 'allwin-journal') '*'
# Never NT8's own log/trace (N12): they name other connections and account numbers. In test rounds they are a
# separate script step (the user copies today's files, filtered to Sim101/Playback).

$inv = Join-Path $out 'inventory.txt'
"collected_utc=$stamp" | Set-Content $inv
"os=$([Environment]::OSVersion.VersionString)" | Add-Content $inv
foreach ($p in @((Join-Path $nt 'bin\Custom\AddOns\AllWin'), (Join-Path $nt 'bin\Custom\AddOns\AllWinCoreSource'))) {
    if (Test-Path $p) { Get-ChildItem -Recurse -File $p | ForEach-Object { "$($_.FullName.Substring($nt.Length)) $($_.Length) $((Get-FileHash $_.FullName -Algorithm SHA256).Hash)" } | Add-Content $inv }
}
$dll = Join-Path $nt 'bin\Custom\AllWin.Adapter.Core.dll'
if (Test-Path $dll) { "bin\Custom\AllWin.Adapter.Core.dll $((Get-Item $dll).Length) $((Get-FileHash $dll -Algorithm SHA256).Hash)" | Add-Content $inv }
$config = Join-Path $nt 'Config.xml'
if (Test-Path $config) { "Config.xml lines naming AllWin:" | Add-Content $inv; Select-String -Path $config -Pattern 'AllWin' | ForEach-Object { $_.Line.Trim() } | Add-Content $inv }

# Second redaction pass: token-shaped strings never leave the machine.
Get-ChildItem -Recurse -File $out -Include *.log, *.txt | ForEach-Object {
    $text = Get-Content -Raw $_.FullName -ErrorAction SilentlyContinue
    if ($text -and $text -match 'aw[dk]_[A-Za-z0-9]+_[A-Za-z0-9_-]+') {
        ($text -replace 'aw([dk])_([A-Za-z0-9]+)_[A-Za-z0-9_-]+', 'aw$1_$2_[REDACTED]') | Set-Content -NoNewline $_.FullName
    }
}

# Inventory of every collected file (QA checks that no token/consent file name appears).
Get-ChildItem -Recurse -File $out | ForEach-Object { "$($_.FullName.Substring($out.Length + 1)) $($_.Length)" } | Set-Content (Join-Path $out 'collected-files.txt')

$zip = "$out.zip"
Compress-Archive -Path (Join-Path $out '*') -DestinationPath $zip -Force
Write-Host "Diagnóstico salvo / diagnostics saved: $zip"
