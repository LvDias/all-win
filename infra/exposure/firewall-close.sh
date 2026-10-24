#!/usr/bin/env bash
# Closes the E2E exposure: stops the extra Caddy, removes the firewall rules, deletes the certificate and key.
#   firewall-close.sh [--dry-run]
set -uo pipefail
dry=0; [ "${1:-}" = "--dry-run" ] && dry=1
PORT=8443; CHAIN=ALLWIN-ADAPTER
SUDO=""; [ "$dry" = 1 ] || [ "$(id -u)" = 0 ] || SUDO="sudo"
run() { if [ "$dry" = 1 ]; then echo "+ $*"; else "$@"; fi; }
here="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
compose="$here/../compose"
if [ -f "$compose/.env" ]; then
  run sg docker -c "docker compose -f '$compose/docker-compose.dev.yml' -f '$here/docker-compose.expose.yml' --env-file '$compose/.env' rm -sf caddy-adapter" 2>/dev/null \
    || run docker rm -f "$(docker ps -aq --filter name=caddy-adapter 2>/dev/null)" 2>/dev/null
fi
run $SUDO iptables -D DOCKER-USER -p tcp -m conntrack --ctorigdstport "$PORT" -j "$CHAIN" 2>/dev/null
run $SUDO iptables -F "$CHAIN" 2>/dev/null
run $SUDO iptables -X "$CHAIN" 2>/dev/null
run $SUDO ip6tables -D DOCKER-USER -p tcp -m conntrack --ctorigdstport "$PORT" -j DROP 2>/dev/null
dir="${ALLWIN_E2E_CERT_DIR:-$HOME/.allwin-e2e-cert}"
run rm -f -- "$dir/e2e.key" "$dir/e2e.crt"
echo "closed. Check from outside: nothing may answer on $PORT."
if [ "$dry" != 1 ]; then ss -ltn 2>/dev/null | awk '{print $4}' | grep -E ":$PORT\$" && echo "WARNING: something still listens on $PORT" || echo "no listener on $PORT"; fi
