# All Win — pairs this NinjaTrader machine with the REAL Core (development build {version}, M1 round).
# Reads %LOCALAPPDATA%\AllWin\dev-core.json (endpoint + certificate pin), asks for the pairing code (prompt only, never an
# argument), calls POST https://<host>:<port>/adapter/v1/pairings over TLS pinned to that certificate SHA-256, and writes the
# device token to %LOCALAPPDATA%\AllWin\device.token with an owner-only ACL. It shows only the connection id and the key id, never the token.
# The pin applies to THIS PowerShell process only (NinjaTrader is not touched). Then restart NinjaTrader.
$ErrorActionPreference = 'Stop'
$aw = Join-Path $env:LOCALAPPDATA 'AllWin'
$configPath = Join-Path $aw 'dev-core.json'
$tokenPath = Join-Path $aw 'device.token'
$idPath = Join-Path $aw 'instance-id'

function Fail($pt, $en) { Write-Host "ERRO / ERROR: $pt / $en"; exit 1 }

if (-not (Test-Path $configPath)) { Fail "dev-core.json não encontrado em $aw" "dev-core.json not found in $aw" }
# Same rules as the plugin (DevCoreConfig.Parse, Crivo L2): only "endpoint" and "cert_sha256", both strings; endpoint absolute
# wss:// without user info or query; pin = 64 hex, ':' and spaces ignored. Anything else: nothing is sent.
try { $config = Get-Content -Raw $configPath | ConvertFrom-Json } catch { Fail 'dev-core.json não é JSON válido' 'dev-core.json is not valid JSON' }
if ($config -isnot [PSCustomObject]) { Fail 'dev-core.json precisa ser um objeto JSON' 'dev-core.json must be a JSON object' }
foreach ($p in $config.PSObject.Properties) {
    if ($p.Name -cne 'endpoint' -and $p.Name -cne 'cert_sha256') { Fail ('campo desconhecido no dev-core.json: ' + $p.Name) ('unknown field in dev-core.json: ' + $p.Name) }
}
if ($config.endpoint -isnot [string] -or $config.cert_sha256 -isnot [string]) { Fail 'endpoint e cert_sha256 precisam ser texto' 'endpoint and cert_sha256 must be strings' }
$endpoint = $null
if (-not [Uri]::TryCreate($config.endpoint, [UriKind]::Absolute, [ref]$endpoint) -or $endpoint.Scheme -ne 'wss' -or $endpoint.UserInfo -or $endpoint.Query) {
    Fail 'o endpoint precisa ser wss://host:porta/caminho' 'the endpoint must be wss://host:port/path'
}
$pin = ($config.cert_sha256 -replace '[: ]', '').ToLowerInvariant()
if ($pin -notmatch '^[0-9a-f]{64}$') { Fail 'cert_sha256 precisa ter 64 hex (dois-pontos permitidos)' 'cert_sha256 must be 64 hex characters (colons allowed)' }
$pairUri = 'https://' + $endpoint.Host + ':' + $endpoint.Port + '/adapter/v1/pairings'

# Same instance id as the plugin (the plugin reads this file when it starts).
New-Item -ItemType Directory -Force -Path $aw | Out-Null
# Same rule as the plugin (Guid.TryParse, Crivo L1): a file the plugin would accept is reused as is, anything else is replaced.
$instanceId = $null
$parsed = [guid]::Empty
if ((Test-Path $idPath) -and [guid]::TryParse((Get-Content -Raw $idPath).Trim(), [ref]$parsed)) { $instanceId = $parsed.ToString() }
else { $instanceId = [guid]::NewGuid().ToString(); Set-Content -NoNewline -Path $idPath -Value $instanceId }

# Certificate pin: accept ONLY the configured SHA-256 (no CA chain, no name check, no "accept invalid").
Add-Type -TypeDefinition @"
using System;
using System.Net.Security;
using System.Security.Cryptography;
using System.Security.Cryptography.X509Certificates;
public static class AllWinPin
{
    public static string Expected;
    public static bool Check(object sender, X509Certificate certificate, X509Chain chain, SslPolicyErrors errors)
    {
        if (certificate == null || Expected == null) return false;
        using (var sha = SHA256.Create())
        {
            var hash = sha.ComputeHash(certificate.GetRawCertData());
            var hex = BitConverter.ToString(hash).Replace("-", "").ToLowerInvariant();
            return hex == Expected;
        }
    }
}
"@
[AllWinPin]::Expected = $pin
[Net.ServicePointManager]::SecurityProtocol = [Net.SecurityProtocolType]::Tls12
[Net.ServicePointManager]::ServerCertificateValidationCallback = [Net.Security.RemoteCertificateValidationCallback] [AllWinPin].GetMethod('Check').CreateDelegate([Net.Security.RemoteCertificateValidationCallback])

$secure = Read-Host 'Código de pareamento (o que o time enviou) / Pairing code (the one the team sent)' -AsSecureString
$bstr = [Runtime.InteropServices.Marshal]::SecureStringToBSTR($secure)
try { $code = [Runtime.InteropServices.Marshal]::PtrToStringBSTR($bstr) } finally { [Runtime.InteropServices.Marshal]::ZeroFreeBSTR($bstr) }
if ([string]::IsNullOrWhiteSpace($code)) { Fail 'nenhum código digitado' 'no code typed' }
$name = $env:COMPUTERNAME; if ($name.Length -gt 64) { $name = $name.Substring(0, 64) }

# Guarita I1/I3: the code and the token only ever pass through .NET methods, never as a cmdlet parameter or pipeline input,
# so PowerShell module/pipeline logging cannot record them. Plain HttpWebRequest with redirects OFF: any 3xx is an error.
Add-Type -AssemblyName System.Web.Extensions
$json = New-Object System.Web.Script.Serialization.JavaScriptSerializer
$adapter = New-Object 'System.Collections.Generic.Dictionary[string,object]'
$adapter['type'] = 'ninjatrader8'; $adapter['version'] = '{version}'; $adapter['build'] = 'development'
$request = New-Object 'System.Collections.Generic.Dictionary[string,object]'
$request['pairing_code'] = $code; $request['instance_id'] = $instanceId; $request['instance_name'] = $name; $request['adapter'] = $adapter
$bodyBytes = [Text.Encoding]::UTF8.GetBytes($json.Serialize($request))
$request.Clear(); $request = $null; $code = $null

$status = $null; $content = $null
try {
    $http = [Net.HttpWebRequest]::Create($pairUri)
    $http.Method = 'POST'
    $http.ContentType = 'application/json'
    $http.AllowAutoRedirect = $false
    $http.Timeout = 30000
    $http.ContentLength = $bodyBytes.Length
    $upload = $http.GetRequestStream()
    try { $upload.Write($bodyBytes, 0, $bodyBytes.Length) } finally { $upload.Close() }
    $response = $http.GetResponse()
    try {
        $status = [int]$response.StatusCode
        $reader = New-Object IO.StreamReader($response.GetResponseStream(), [Text.Encoding]::UTF8)
        try { $content = $reader.ReadToEnd() } finally { $reader.Close() }
    } finally { $response.Close() }
} catch [Net.WebException] {
    $failed = $_.Exception.Response
    $netStatus = $_.Exception.Status
    if ($failed) { $status = [int]$failed.StatusCode; $retryAfter = $failed.Headers['Retry-After']; $failed.Close() }
    switch ($status) {
        400 { Fail 'código inválido, expirado ou já usado; peça um código novo ao time' 'invalid, expired or used code; ask the team for a new code' }
        426 { Fail 'versão do plugin não suportada pelo Core' 'plugin version not supported by the Core' }
        429 { Fail ('muitas tentativas; espere ' + $retryAfter + ' s') ('too many attempts; wait ' + $retryAfter + ' s') }
        $null { Fail ('sem resposta do Core (certificado diferente do pin, ou rede): ' + $netStatus) ('no answer from the Core (certificate does not match the pin, or network): ' + $netStatus) }
        default { Fail "HTTP $status" "HTTP $status" }
    }
} finally {
    [Array]::Clear($bodyBytes, 0, $bodyBytes.Length)
    [Net.ServicePointManager]::ServerCertificateValidationCallback = $null
}

if ($status -ge 300 -and $status -lt 400) { Fail ('o Core respondeu com redirecionamento HTTP ' + $status + '; nada gravado') ('the Core answered with a redirect HTTP ' + $status + '; nothing written') }
if ($status -ne 201) { Fail ('resposta inesperada HTTP ' + $status) ('unexpected response HTTP ' + $status) }
try { $result = $json.DeserializeObject($content) } catch { $result = $null }
$content = $null
if ($result -isnot [Collections.Generic.IDictionary[string,object]]) { Fail 'resposta do Core em formato inesperado; nada gravado' 'unexpected Core response; nothing written' }
$token = [string]$result['device_token']
if ($token -cnotmatch '^awd_[A-Za-z0-9]{12}_[A-Za-z0-9_-]{43}\z') { Fail 'token em formato inesperado; nada gravado' 'unexpected token format; nothing written' }
$connectionId = [string]$result['connection_id']
$parsedConnection = [guid]::Empty
if (-not [guid]::TryParseExact($connectionId, 'D', [ref]$parsedConnection)) { Fail 'connection_id em formato inesperado; nada gravado' 'unexpected connection_id format; nothing written' }
$result = $null

# Owner-only file first, then the content: the token is never readable by another account, not even for a moment.
if (Test-Path $tokenPath) { Remove-Item -Force $tokenPath }
New-Item -ItemType File -Path $tokenPath | Out-Null
# By SID (Guarita L1 / Crivo n1): no ambiguity between a local and a domain account with the same name.
$sid = [Security.Principal.WindowsIdentity]::GetCurrent().User.Value
& icacls $tokenPath /inheritance:r /grant:r "*$($sid):(R,W)" | Out-Null
if ($LASTEXITCODE -ne 0) { Remove-Item -Force $tokenPath; Fail 'não consegui restringir o arquivo do token; nada gravado' 'could not restrict the token file; nothing written' }
[IO.File]::WriteAllText($tokenPath, $token)
$keyId = $token.Split('_')[1]
$token = $null

Write-Host "Pareado / Paired: connection $($parsedConnection.ToString()), key_id awd_$keyId`_*** — reinicie o NinjaTrader / restart NinjaTrader."
