#!/usr/bin/env bash
# Closes the E2E gateway and VERIFIES it: container, firewall rules, gate marker, certificate and key. Exits non-zero with "NOT CLOSED"
# if anything is left.   expose-close.sh
set -uo pipefail
here="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"; project="${ALLWIN_STACK_PROJECT:-allwin-sunday}"
cert="${ALLWIN_E2E_CERT_DIR:-$HOME/.allwin-e2e-cert}"; gate="${ALLWIN_EXPOSE_GATE_DIR:-/run/allwin-expose}"
dk() { if docker info >/dev/null 2>&1; then docker "$@"; else sg docker -c "$(printf '%q ' docker "$@")"; fi; }

rm -f -- "$gate/OPEN"                                   # first: a container that restarts now refuses to start
ids="$(dk ps -aq --filter "label=com.docker.compose.project=$project" --filter label=com.docker.compose.service=caddy-adapter 2>/dev/null)"
[ -z "$ids" ] || dk rm -f $ids >/dev/null 2>&1
"$here/firewall-close.sh" || true
rm -f -- "$cert/e2e.key" "$cert/e2e.crt"

fail=0
[ -z "$(dk ps -aq --filter "label=com.docker.compose.project=$project" --filter label=com.docker.compose.service=caddy-adapter 2>/dev/null)" ] || { echo "still present: caddy-adapter container" >&2; fail=1; }
[ ! -e "$gate/OPEN" ] || { echo "still present: gate marker" >&2; fail=1; }
{ [ -e "$cert/e2e.key" ] || [ -e "$cert/e2e.crt" ]; } && { echo "still present: certificate or key" >&2; fail=1; }
sudo iptables -S DOCKER-USER 2>/dev/null | grep -qE -- "-j ALLWIN-ADAPTER|allwin-adapter-bridge" && { echo "still present: firewall hook or bridge drop" >&2; fail=1; }
sudo iptables -S ALLWIN-ADAPTER >/dev/null 2>&1 && { echo "still present: firewall chain" >&2; fail=1; }
ss -ltn 2>/dev/null | awk '{print $4}' | grep -qE ':8443$' && { echo "still listening on 8443" >&2; fail=1; }
[ "$fail" = 0 ] || { echo "NOT CLOSED" >&2; exit 1; }
echo "closed and verified: no container, no rules, no marker, no certificate, nothing on 8443."
