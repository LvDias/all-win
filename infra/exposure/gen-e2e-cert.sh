#!/usr/bin/env bash
# Generates the self-signed certificate of the E2E exposure on the server. The key never leaves the server and is never committed.
#   gen-e2e-cert.sh <DEV_SERVER_PUBLIC_IP> [days]       (default 3 days, max 7)
# Prints the SHA-256 fingerprint of the certificate: give it to the user for the Windows trust step (README, step 5).
set -euo pipefail
ip="${1:-}"; days="${2:-3}"
[[ "$ip" =~ ^([0-9]{1,3}\.){3}[0-9]{1,3}$ ]] || { echo "usage: $0 <server public IPv4> [days]" >&2; exit 2; }
[[ "$days" =~ ^[0-9]{1,3}$ ]] && [ "$days" -ge 1 ] && [ "$days" -le 7 ] || { echo "days must be 1..7" >&2; exit 2; }
dir="${ALLWIN_E2E_CERT_DIR:-$HOME/.allwin-e2e-cert}"
umask 077
mkdir -p -- "$dir"; chmod 700 -- "$dir"
openssl req -x509 -newkey ec -pkeyopt ec_paramgen_curve:prime256v1 -nodes -days "$days" \
  -subj "/CN=allwin-e2e-gateway" -addext "basicConstraints=critical,CA:FALSE" -addext "keyUsage=critical,digitalSignature" \
  -addext "extendedKeyUsage=serverAuth" -addext "subjectAltName=IP:$ip" \
  -keyout "$dir/e2e.key" -out "$dir/e2e.crt" 2>/dev/null
chmod 600 "$dir/e2e.key"; chmod 644 "$dir/e2e.crt"   # the container runs as the key owner (uid:gid of this account): the key stays 600
echo "certificate: $dir/e2e.crt (valid $days days)"
echo "sha256 fingerprint: $(openssl x509 -in "$dir/e2e.crt" -noout -fingerprint -sha256 | sed 's/^.*=//')"
