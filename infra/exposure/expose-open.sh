#!/usr/bin/env bash
# The ONLY way to bring the E2E gateway up: firewall rules first (verified), then the gate marker, then the container.
#   expose-open.sh <VPS_PUBLIC_IP>        (exactly one argument)
# Precondition: gen-e2e-cert.sh already ran (certificate and key in ALLWIN_E2E_CERT_DIR, default ~/.allwin-e2e-cert) and the README gates are met.
set -euo pipefail
here="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
# EXACTLY one argument, the allowed source IP. No other option is accepted (no --dry-run, no --test-source): a dry run is expose-dry-run.sh.
[ "$#" -eq 1 ] || { echo "usage: $0 <VPS_PUBLIC_IP>   (exactly one argument; for a dry run use expose-dry-run.sh)" >&2; exit 2; }
ip="$1"
# The test-only variable never reaches a real opening.
unset ALLWIN_FIREWALL_TEST_MODE
compose="$here/../compose"
cert="${ALLWIN_E2E_CERT_DIR:-$HOME/.allwin-e2e-cert}"; gate="${ALLWIN_EXPOSE_GATE_DIR:-/run/allwin-expose}"
[ -f "$cert/e2e.crt" ] && [ -f "$cert/e2e.key" ] || { echo "run gen-e2e-cert.sh first" >&2; exit 1; }
[ "$(stat -c %a "$cert/e2e.key")" = 600 ] && [ "$(stat -c %u "$cert/e2e.key")" = "$(id -u)" ] || { echo "the key must be mode 600 and owned by $(id -un)" >&2; exit 1; }
[ -f "$compose/.env" ] || { echo "missing $compose/.env" >&2; exit 1; }

dk() { if docker info >/dev/null 2>&1; then docker "$@"; else sg docker -c "$(printf '%q ' docker "$@")"; fi; }

cleanup_on_error() { echo "open failed: closing" >&2; "$here/expose-close.sh" || true; }
trap cleanup_on_error ERR

# 0. the gateway's own Docker network bridge (the dev stack must be up): the firewall also drops 8443 from every OTHER Docker bridge
core_id="$(dk ps -q --filter label=com.docker.compose.service=core | head -n 1)"
[ -n "$core_id" ] || { echo "the dev stack is not up (no core container)" >&2; exit 1; }
net_id="$(dk inspect -f '{{range .NetworkSettings.Networks}}{{.NetworkID}} {{end}}' "$core_id" | awk '{print $1}')"
gw_br="$(dk network inspect -f '{{index .Options "com.docker.network.bridge.name"}}' "$net_id" 2>/dev/null)"
[ -n "$gw_br" ] && [ "$gw_br" != "<no value>" ] || gw_br="br-${net_id:0:12}"
ip link show "$gw_br" >/dev/null 2>&1 || { echo "cannot find the gateway network bridge ($gw_br)" >&2; exit 1; }
# 1. firewall: applied AND verified before anything listens
"$here/firewall-open.sh" "$ip" --gw-bridge "$gw_br"
# 2. gate marker (tmpfs): written only after the firewall was verified; removed by expose-close.sh and by a reboot
sudo install -d -o "$(id -u)" -g "$(id -g)" -m 755 "$gate"
: > "$gate/OPEN"
# 3. gateway container (the dev edge moves to 8444 so 8443 is free)
export ALLWIN_E2E_CERT_DIR="$cert" ALLWIN_E2E_UID="$(id -u)" ALLWIN_E2E_GID="$(id -g)" ALLWIN_EXPOSE_GATE_DIR="$gate" CADDY_HTTPS_PORT=8444
dk compose -f "$compose/docker-compose.dev.yml" -f "$here/docker-compose.expose.yml" --env-file "$compose/.env" --profile core up -d caddy caddy-adapter
# 4. verify: container running, port listening, hook still present
sleep 3
[ "$(dk ps -q --filter name=caddy-adapter --filter status=running | wc -l)" -ge 1 ] || { echo "caddy-adapter is not running" >&2; false; }
ss -ltn | awk '{print $4}' | grep -qE ':8443$' || { echo "nothing listens on 8443" >&2; false; }
sudo iptables -S DOCKER-USER | grep -q -- "-j ALLWIN-ADAPTER" || { echo "firewall hook missing" >&2; false; }
sudo iptables -S DOCKER-USER | grep -q "allwin-adapter-bridge" || { echo "bridge drop missing" >&2; false; }
echo "gateway open on 8443 for the allowed source. Close with expose-close.sh."
