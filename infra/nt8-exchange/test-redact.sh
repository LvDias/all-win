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
2026-10-24 10:00:08 -----BEGIN OPENSSH PRIVATE KEY-----
LOG
out="$tmp/out.log"
perl "$here/redact.pl" "$tmp/in.log" > "$out" 2> "$tmp/counts"
fail=0
for leak in APEX-1234567 abcDEF1234567890xyz eyJhbGciOiJIUzI1NiJ9 AbCdEfGhIjKl_ tskey-auth 203.0.113.7 2001:db8 vm-a.tail1234 user@example.com JohnDoe 0123456789abcdef0123 hunter2 'PRIVATE KEY'; do
  if grep -qF -- "$leak" "$out"; then echo "FAIL: leaked '$leak'"; fail=1; fi
done
for keep in 'Account: Sim101' 'Playback101 ok' 127.0.0.1 'C:\Users\<user>'; do
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
