#!/usr/bin/env bash
# Removes the firewall rules of the E2E exposure and VERIFIES it. Exits non-zero with "NOT CLOSED" if anything remains.
# Normally called by expose-close.sh.   firewall-close.sh [--dry-run]
set -uo pipefail
dry=0; [ "${1:-}" = "--dry-run" ] && dry=1
PORT=8443; CHAIN=ALLWIN-ADAPTER
SUDO=""; [ "$dry" = 1 ] || [ "$(id -u)" = 0 ] || SUDO="sudo"
run() { if [ "$dry" = 1 ]; then echo "+ $*"; else "$@"; fi; }

# Delete every hook that points at the chain (any interface), then the chain itself, then the IPv6 drops.
if [ "$dry" = 1 ]; then echo "+ (delete all DOCKER-USER rules with -j $CHAIN or --dport $PORT -j DROP, flush and delete $CHAIN)"; exit 0; fi
while rule="$($SUDO iptables -S DOCKER-USER 2>/dev/null | grep -- "-j $CHAIN" | head -n 1)"; [ -n "$rule" ]; do
  $SUDO iptables ${rule/-A /-D } 2>/dev/null || break
done
$SUDO iptables -F "$CHAIN" 2>/dev/null
$SUDO iptables -X "$CHAIN" 2>/dev/null
while rule="$($SUDO ip6tables -S DOCKER-USER 2>/dev/null | grep -- "--dport $PORT" | head -n 1)"; [ -n "$rule" ]; do
  $SUDO ip6tables ${rule/-A /-D } 2>/dev/null || break
done

fail=0
$SUDO iptables -S DOCKER-USER 2>/dev/null | grep -q -- "-j $CHAIN" && { echo "still present: DOCKER-USER hook" >&2; fail=1; }
$SUDO iptables -S "$CHAIN" >/dev/null 2>&1 && { echo "still present: chain $CHAIN" >&2; fail=1; }
$SUDO ip6tables -S DOCKER-USER 2>/dev/null | grep -q -- "--dport $PORT" && { echo "still present: IPv6 drop" >&2; fail=1; }
[ "$fail" = 0 ] || { echo "NOT CLOSED (firewall)" >&2; exit 1; }
echo "firewall rules removed and verified."
