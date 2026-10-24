#!/usr/bin/env bash
# The ONLY way to bring the E2E gateway up: firewall rules first (verified), then the gate marker, then the container.
#   expose-open.sh <VPS_PUBLIC_IP>        (exactly one argument)
# It works on the RUNNING Sunday stack only: compose project `allwin-sunday` (ALLWIN_STACK_PROJECT), the same files as the stack
# (docker-compose.dev.yml + docker-compose.stack.yml from ALLWIN_STACK_DIR, default ../compose, plus docker-compose.expose.yml) and that
# stack's .env. It never creates another stack: caddy-adapter starts with --no-deps, and the Core and the bridge are found by the compose
# project label, failing unless there is exactly one.
# Precondition: gen-e2e-cert.sh already ran (certificate and key in ALLWIN_E2E_CERT_DIR, default ~/.allwin-e2e-cert) and the README gates are met.
set -euo pipefail
here="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
# EXACTLY one argument, the allowed source IP. No other option is accepted (no --dry-run, no --test-source): a dry run is expose-dry-run.sh.
[ "$#" -eq 1 ] || { echo "usage: $0 <VPS_PUBLIC_IP>   (exactly one argument; for a dry run use expose-dry-run.sh)" >&2; exit 2; }
ip="$1"
# The test-only variable never reaches a real opening.
unset ALLWIN_FIREWALL_TEST_MODE
stack="${ALLWIN_STACK_DIR:-$here/../compose}"; project="${ALLWIN_STACK_PROJECT:-allwin-sunday}"
[[ "$project" =~ ^[a-z0-9][a-z0-9_-]*$ ]] || { echo "invalid compose project name" >&2; exit 2; }
cert="${ALLWIN_E2E_CERT_DIR:-$HOME/.allwin-e2e-cert}"; gate="${ALLWIN_EXPOSE_GATE_DIR:-/run/allwin-expose}"
[ -f "$cert/e2e.crt" ] && [ -f "$cert/e2e.key" ] || { echo "run gen-e2e-cert.sh first" >&2; exit 1; }
[ "$(stat -c %a "$cert/e2e.key")" = 600 ] && [ "$(stat -c %u "$cert/e2e.key")" = "$(id -u)" ] || { echo "the key must be mode 600 and owned by $(id -un)" >&2; exit 1; }
for f in docker-compose.dev.yml docker-compose.stack.yml .env; do [ -f "$stack/$f" ] || { echo "missing $stack/$f" >&2; exit 1; }; done

dk() { if docker info >/dev/null 2>&1; then docker "$@"; else sg docker -c "$(printf '%q ' docker "$@")"; fi; }
dc() { dk compose -p "$project" -f "$stack/docker-compose.dev.yml" -f "$stack/docker-compose.stack.yml" -f "$here/docker-compose.expose.yml" \
        --env-file "$stack/.env" --profile core "$@"; }
exactly_one() { # <what> <ids...>: the ids must be exactly one non-empty line
  local what="$1" ids="$2" n; n="$(printf '%s\n' "$ids" | grep -c . || true)"
  [ "$n" -eq 1 ] || { echo "expected exactly one $what of project $project, found $n" >&2; return 1; }
}

cleanup_on_error() { echo "open failed: closing" >&2; "$here/expose-close.sh" || true; }
trap cleanup_on_error ERR

# 0. the running stack: exactly one Core container and exactly one compose network of THIS project (other projects on the host are ignored)
core_id="$(dk ps -q --filter "label=com.docker.compose.project=$project" --filter label=com.docker.compose.service=core --filter status=running)"
exactly_one "running core container" "$core_id"
net_id="$(dk network ls -q --filter "label=com.docker.compose.project=$project")"
exactly_one "compose network" "$net_id"
gw_br="$(dk network inspect -f '{{index .Options "com.docker.network.bridge.name"}}' "$net_id" 2>/dev/null)"
[ -n "$gw_br" ] && [ "$gw_br" != "<no value>" ] || gw_br="br-${net_id:0:12}"
ip link show "$gw_br" >/dev/null 2>&1 || { echo "cannot find the gateway network bridge ($gw_br)" >&2; exit 1; }
# 0b. nothing may already listen on 8443 (for example a stack edge left on the default 8443): otherwise the final listening check would pass by accident
if ss -ltn | awk '{print $4}' | grep -qE ':8443$'; then echo "refusing: something already listens on 8443 (set the stack's CADDY_HTTPS_PORT to another port, e.g. 18443)" >&2; exit 1; fi
# 1. firewall: applied AND verified before anything listens
"$here/firewall-open.sh" "$ip" --gw-bridge "$gw_br"
# 2. gate marker (tmpfs): written only after the firewall was verified; removed by expose-close.sh and by a reboot
sudo install -d -o "$(id -u)" -g "$(id -g)" -m 755 "$gate"
: > "$gate/OPEN"
# 3. gateway container ONLY (--no-deps: nothing else is created or recreated; the stack's edge keeps its own loopback port)
export ALLWIN_E2E_CERT_DIR="$cert" ALLWIN_E2E_UID="$(id -u)" ALLWIN_E2E_GID="$(id -g)" ALLWIN_EXPOSE_GATE_DIR="$gate"
dc up -d --no-deps caddy-adapter
# 4. verify: exactly one caddy-adapter of this project running, port listening, hooks still present
sleep 3
adapter_id="$(dk ps -q --filter "label=com.docker.compose.project=$project" --filter label=com.docker.compose.service=caddy-adapter --filter status=running)"
exactly_one "running caddy-adapter" "$adapter_id"
ss -ltn | awk '{print $4}' | grep -qE ':8443$' || { echo "nothing listens on 8443" >&2; false; }
sudo iptables -S DOCKER-USER | grep -q -- "-j ALLWIN-ADAPTER" || { echo "firewall hook missing" >&2; false; }
sudo iptables -S DOCKER-USER | grep -q "allwin-adapter-bridge" || { echo "bridge drop missing" >&2; false; }
echo "gateway open on 8443 for the allowed source (project $project). Close with expose-close.sh."
