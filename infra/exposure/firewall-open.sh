#!/usr/bin/env bash
# Applies and VERIFIES the firewall rules for TCP 8443: one allowed source IP (the user's Windows VPS), connection and rate limits, everything
# else dropped. Needs sudo; normally called by expose-open.sh. DOES NOT RUN without the explicit "open" authorization of the user (README).
# Docker-published ports bypass ufw, so the rules live in the DOCKER-USER chain (evaluated for published container ports).
#   firewall-open.sh <VPS_PUBLIC_IP> [--dry-run] [--test-source]
# --test-source (tests only) allows private and documentation addresses; the external interface can be forced with ALLWIN_EXPOSE_EXT_IF.
# The rules are in memory only: a reboot removes them (and expose-open.sh is the only way to start the gateway again).
set -euo pipefail
ip="${1:-}"; dry=0; test_source=0
shift || true
for a in "$@"; do case "$a" in --dry-run) dry=1 ;; --test-source) test_source=1 ;; *) echo "unknown option $a" >&2; exit 2 ;; esac; done

oct='(0|[1-9][0-9]{0,2})'
[[ "$ip" =~ ^$oct\.$oct\.$oct\.$oct$ ]] || { echo "usage: $0 <VPS public IPv4, no leading zeros> [--dry-run]" >&2; exit 2; }
IFS=. read -r o1 o2 o3 o4 <<<"$ip"
for o in $o1 $o2 $o3 $o4; do [ "$o" -le 255 ] || { echo "invalid IPv4" >&2; exit 2; }; done
# Refused: this-network, private, loopback, CGNAT (100.64/10), link-local, multicast and reserved (224/3), documentation ranges.
bad=""
[ "$o1" -eq 0 ] || [ "$o1" -eq 127 ] || [ "$o1" -ge 224 ] && bad=1
[ "$o1" -eq 10 ] && bad=1
[ "$o1" -eq 100 ] && [ "$o2" -ge 64 ] && [ "$o2" -le 127 ] && bad=1
[ "$o1" -eq 169 ] && [ "$o2" -eq 254 ] && bad=1
[ "$o1" -eq 172 ] && [ "$o2" -ge 16 ] && [ "$o2" -le 31 ] && bad=1
[ "$o1" -eq 192 ] && [ "$o2" -eq 168 ] && bad=1
if [ "$test_source" != 1 ]; then
  [ "$o1" -eq 192 ] && [ "$o2" -eq 0 ] && [ "$o3" -eq 2 ] && bad=1
  [ "$o1" -eq 198 ] && [ "$o2" -eq 51 ] && [ "$o3" -eq 100 ] && bad=1
  [ "$o1" -eq 203 ] && [ "$o2" -eq 0 ] && [ "$o3" -eq 113 ] && bad=1
else
  bad=""
fi
[ -z "$bad" ] || { echo "refusing a private, loopback, CGNAT, multicast/reserved or documentation address" >&2; exit 2; }
# The source must not be this server.
if hostname -I 2>/dev/null | tr ' ' '\n' | grep -qx "$ip"; then echo "refusing: $ip is an address of this host" >&2; exit 2; fi

# External interface = the one of the default route (the hook must not catch traffic coming from containers).
ext_if="${ALLWIN_EXPOSE_EXT_IF:-$(ip -4 route show default 2>/dev/null | awk '/default/ {for (i=1;i<NF;i++) if ($i=="dev") {print $(i+1); exit}}')}"
[[ "$ext_if" =~ ^[A-Za-z0-9_.:-]{1,15}$ ]] || { echo "cannot determine the external interface (set ALLWIN_EXPOSE_EXT_IF)" >&2; exit 1; }

PORT=8443; CHAIN=ALLWIN-ADAPTER
SUDO=""; [ "$dry" = 1 ] || [ "$(id -u)" = 0 ] || SUDO="sudo"
run() { if [ "$dry" = 1 ]; then echo "+ $*"; else "$@"; fi; }
ipt() { run $SUDO iptables "$@"; }

ipt -N "$CHAIN" 2>/dev/null || true
ipt -F "$CHAIN"
# 1. at most 4 concurrent connections from the allowed source (one plugin session plus reconnects)
ipt -A "$CHAIN" -s "$ip" -p tcp -m connlimit --connlimit-above 4 --connlimit-mask 32 -j DROP
# 2. at most 12 new connections per minute (burst 6) from the allowed source: bounds connect/auth attempts
ipt -A "$CHAIN" -s "$ip" -p tcp -m conntrack --ctstate NEW -m hashlimit \
    --hashlimit-above 12/min --hashlimit-burst 6 --hashlimit-mode srcip --hashlimit-name allwin-adapter -j DROP
ipt -A "$CHAIN" -s "$ip" -p tcp -j ACCEPT
# 3. every other source: dropped
ipt -A "$CHAIN" -j DROP

# Hook: only packets that ENTER from the external interface, in the ORIGINAL direction (client to server), to a published port 8443.
# Replies (container to client) and a container's own outbound connections to some host:8443 never match.
HOOK=(-i "$ext_if" -p tcp -m conntrack --ctorigdstport "$PORT" --ctdir ORIGINAL -j "$CHAIN")
if [ "$dry" = 1 ]; then echo "+ $SUDO iptables -I DOCKER-USER 1 ${HOOK[*]}"; else
  $SUDO iptables -C DOCKER-USER "${HOOK[@]}" 2>/dev/null || $SUDO iptables -I DOCKER-USER 1 "${HOOK[@]}"
fi
# IPv6: Docker does not publish on IPv6 here; drop it on the external interface anyway.
HOOK6=(-i "$ext_if" -p tcp --dport "$PORT" -j DROP)
if [ "$dry" = 1 ]; then echo "+ $SUDO ip6tables -I DOCKER-USER 1 ${HOOK6[*]}"; else
  $SUDO ip6tables -C DOCKER-USER "${HOOK6[@]}" 2>/dev/null || $SUDO ip6tables -I DOCKER-USER 1 "${HOOK6[@]}" 2>/dev/null || true
fi

# Verify before reporting success (iptables -C): the hook, the accept for the source, and the final drop.
if [ "$dry" != 1 ]; then
  ok=1
  $SUDO iptables -C DOCKER-USER "${HOOK[@]}" 2>/dev/null || { echo "verify: hook missing" >&2; ok=0; }
  $SUDO iptables -C "$CHAIN" -s "$ip" -p tcp -j ACCEPT 2>/dev/null || { echo "verify: accept rule missing" >&2; ok=0; }
  $SUDO iptables -C "$CHAIN" -j DROP 2>/dev/null || { echo "verify: final drop missing" >&2; ok=0; }
  [ "$ok" = 1 ] || { echo "FIREWALL NOT APPLIED CORRECTLY" >&2; exit 1; }
fi
echo "firewall ready: $PORT from the allowed source only via $ext_if (connlimit 4, 12 new/min)."
