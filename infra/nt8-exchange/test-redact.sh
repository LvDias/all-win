#!/usr/bin/env bash
# Self-test of redact.pl with FAKE secrets. Run: infra/nt8-exchange/test-redact.sh
set -euo pipefail
here="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
tmp="$(mktemp -d)"; trap 'rm -rf "$tmp"' EXIT
cat > "$tmp/in.log" <<'LOG'
2026-10-24 10:00:00 Account: Sim101 connected, Playback101 ok
2026-10-24 10:00:01 Account: APEX-1234567 connected
2026-10-24 10:00:02 token=abcDEF1234567890xyz Authorization: Bearer eyJhbGciOiJIUzI1NiJ9.payloadpart.sigpart
2026-10-24 10:00:03 key awk_AbCdEfGhIjKl_0123456789012345678901234567890123456789ABC and tskey-auth-abcdefghij12345
2026-10-24 10:00:04 from 203.0.113.7 to 127.0.0.1 and 2001:db8:0:0:0:0:0:1 host vm-a.tail1234.ts.net mail user@example.com
2026-10-24 10:00:05 path C:\Users\JohnDoe\Documents\NinjaTrader 8\log\log.txt
2026-10-24 10:00:06 blob 0123456789abcdef0123456789abcdef0123456789abcdef and AbCdEfGhIjKlMnOpQrStUvWxYz0123456789AbCdEf
2026-10-24 10:00:07 url https://user:hunter2@example.com/path?x=1
2026-10-24 10:00:09 {"password": "FakePass123", "token": "faketok999", "note": "ok"}
2026-10-24 10:00:10 password="two words here" and secret='single quoted words' end
2026-10-24 10:00:11 <Password>FakeXmlPass1</Password><ApiKey>FakeXmlKey2</ApiKey><Account>Sim101</Account>
2026-10-24 10:00:12 peers 2001:db8::1 fe80::abcd:1 and ::1 and ::ffff:1234:5678
2026-10-24 10:00:13 path C:/Users/JaneRoe/Documents/NinjaTrader 8/log
2026-10-24 10:00:14 user=johndoe99 User: maria.silva username jsmith42 login=trader77
2026-10-24 10:00:15 call +55 11 91234-5678 or (11) 91234-5678 or 555-123-4567 mac aa:bb:cc:dd:ee:ff machine DESKTOP-AB12CD
2026-10-24 10:00:16 -----BEGIN OPENSSH PRIVATE KEY-----
b3BlbnNzaC1rZXktdjEAAAAABG5vbmUAAAAEbm9uZQAAAAAAAAABAAAAMwAAAAtzc2gtZW
Qy+NTUxOQAAACBfakefakefakeFAKEFAKEfakefakeFAKEFAKE/fakefake+FAKE0123
-----END OPENSSH PRIVATE KEY-----
2026-10-24 10:00:17 after the key, Sim101 still here
LOG
out="$tmp/out.log"
perl "$here/redact.pl" "$tmp/in.log" > "$out" 2> "$tmp/counts"
fail=0
for leak in FakePass123 faketok999 'two words' 'single quoted' FakeXmlPass1 FakeXmlKey2 2001:db8::1 fe80::abcd JaneRoe johndoe99 maria.silva jsmith42 trader77 91234-5678 555-123-4567 aa:bb:cc:dd:ee:ff DESKTOP-AB12CD b3BlbnNzaC1rZXktdjEAAAAABG5vbmUAAAAEbm9uZQ 'FAKEFAKEfakefake' APEX-1234567 abcDEF1234567890xyz eyJhbGciOiJIUzI1NiJ9 AbCdEfGhIjKl_ tskey-auth 203.0.113.7 2001:db8 vm-a.tail1234 user@example.com JohnDoe 0123456789abcdef0123 hunter2 'PRIVATE KEY'; do
  if grep -qF -- "$leak" "$out"; then echo "FAIL: leaked '$leak'"; fail=1; fi
done
for keep in '10:00:12 peers' '::1 and' '<Account>Sim101</Account>' 'after the key, Sim101 still here' 'Account: Sim101' 'Playback101 ok' 127.0.0.1 'C:\Users\<user>'; do
  grep -qF -- "$keep" "$out" || { echo "FAIL: lost '$keep'"; fail=1; }
done
# the redacted output is stable, and --check agrees
if ! perl "$here/redact.pl" --check "$out" 2>/dev/null; then echo "FAIL: output not stable under --check"; fail=1; fi
if perl "$here/redact.pl" --check "$tmp/in.log" 2>/dev/null; then echo "FAIL: --check missed secrets"; fail=1; fi
# plugin journal line (crc + body) keeps its shape; only the secret goes
printf '%s\n' 'a1b2c3d4 {"t":"fill","instruction_id":"0f1e2d3c-1111-2222-3333-444455556666","account":"Sim101","token":"abcDEF1234567890xyz"}' > "$tmp/journal.log"
perl "$here/redact.pl" "$tmp/journal.log" 2>/dev/null > "$tmp/journal.out"
if grep -q 'a1b2c3d4 {"t":"fill"' "$tmp/journal.out" && grep -q Sim101 "$tmp/journal.out" && ! grep -q abcDEF1234567890xyz "$tmp/journal.out"; then :; else echo "FAIL: journal line"; fail=1; fi
[ "$fail" = 0 ] && echo "redact tests OK" || exit 1
