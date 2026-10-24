#!/usr/bin/env bash
# Opens TCP 8443 for ONE source IP (the user's Windows VPS) with a connection and rate limit. Needs sudo; DOES NOT RUN without
# the explicit "open" authorization of the user (see README). Docker-published ports bypass ufw, so the rules live in the
# DOCKER-USER chain (evaluated for published container ports before Docker's own accept rules).
#   firewall-open.sh <VPS_PUBLIC_IP> [--dry-run]
# The rules are in memory only: a reboot closes the port. firewall-close.sh removes them.
set -euo pipefail
ip="${1:-}"; dry=0; [ "${2:-}" = "--dry-run" ] && dry=1
[[ "$ip" =~ ^([0-9]{1,3}\.){3}[0-9]{1,3}$ ]] || { echo "usage: $0 <VPS public IPv4> [--dry-run]" >&2; exit 2; }
for o in ${ip//./ }; do [ "$o" -le 255 ] || { echo "invalid IPv4" >&2; exit 2; }; done
case "$ip" in 0.*|10.*|127.*|169.254.*|192.168.*|255.*|100.6[4-9].*|100.[7-9][0-9].*|100.1[01][0-9].*|100.12[0-7].*|172.1[6-9].*|172.2[0-9].*|172.3[01].*)
  echo "refusing a private, loopback or CGNAT address" >&2; exit 2 ;; esac
# The source must not be this server (a typo that would open the port to ourselves, or to the wrong machine).
if hostname -I 2>/dev/null | tr ' ' '\n' | grep -qx "$ip"; then echo "refusing: $ip is an address of this host" >&2; exit 2; fi

PORT=8443; CHAIN=ALLWIN-ADAPTER
run() { if [ "$dry" = 1 ]; then echo "+ $*"; else "$@"; fi; }
SUDO=""; [ "$dry" = 1 ] || [ "$(id -u)" = 0 ] || SUDO="sudo"

run $SUDO iptables -N "$CHAIN" 2>/dev/null || true
run $SUDO iptables -F "$CHAIN"
# 1. at most 4 concurrent connections from the allowed source (one plugin session + reconnects)
run $SUDO iptables -A "$CHAIN" -s "$ip" -p tcp -m connlimit --connlimit-above 4 --connlimit-mask 32 -j DROP
# 2. at most 12 new connections per minute (burst 6) from the allowed source: bounds connect/auth attempts
run $SUDO iptables -A "$CHAIN" -s "$ip" -p tcp -m conntrack --ctstate NEW -m hashlimit \
      --hashlimit-above 12/min --hashlimit-burst 6 --hashlimit-mode srcip --hashlimit-name allwin-adapter -j DROP
run $SUDO iptables -A "$CHAIN" -s "$ip" -p tcp -j ACCEPT
# 3. every other source: dropped
run $SUDO iptables -A "$CHAIN" -j DROP
# Hook: only traffic whose ORIGINAL destination port is 8443 (a published container port) enters the chain.
ensure() { # <iptables|ip6tables> <rule...>: insert at the top of DOCKER-USER unless it is already there
  local bin="$1"; shift
  if [ "$dry" = 1 ]; then echo "+ $SUDO $bin -I DOCKER-USER 1 $*"; return 0; fi
  $SUDO "$bin" -C DOCKER-USER "$@" 2>/dev/null || $SUDO "$bin" -I DOCKER-USER 1 "$@"
}
# Hook: only traffic whose ORIGINAL destination port is 8443 (a published container port) enters the chain.
ensure iptables -p tcp -m conntrack --ctorigdstport "$PORT" -j "$CHAIN"
# IPv6: Docker does not publish on IPv6 here; drop it anyway.
ensure ip6tables -p tcp -m conntrack --ctorigdstport "$PORT" -j DROP || true
echo "port $PORT open for the allowed source only (connlimit 4, 12 new/min). Close with firewall-close.sh."
