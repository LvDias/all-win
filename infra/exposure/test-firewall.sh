#!/usr/bin/env bash
# Functional test of firewall-open.sh / firewall-close.sh in throwaway network namespaces (the host firewall is not touched). Needs sudo and iproute2.
#   infra/exposure/test-firewall.sh
# Topology: [cl: 10.77.0.2 (allowed) and 10.77.0.3 (other)] -- s-ext/c-ext -- [srv: DNAT 8443 -> cont, FORWARD -> DOCKER-USER] -- [cont: 172.30.0.2:8443 web]
set -uo pipefail
here="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
S=aw-srv; C=aw-cont; L=aw-cl
cleanup() { sudo ip netns pids "$C" 2>/dev/null | xargs -r sudo kill 2>/dev/null; for n in $S $C $L; do sudo ip netns del $n 2>/dev/null; done; }
trap cleanup EXIT; cleanup
sudo ip netns add $S && sudo ip netns add $C && sudo ip netns add $L || { echo "cannot create namespaces"; exit 1; }
sudo ip link add s-ext type veth peer name c-ext && sudo ip link set s-ext netns $S && sudo ip link set c-ext netns $L
sudo ip link add s-br type veth peer name c-br && sudo ip link set s-br netns $S && sudo ip link set c-br netns $C
x() { local ns=$1; shift; sudo ip netns exec "$ns" "$@"; }
x $S ip addr add 10.77.0.1/24 dev s-ext; x $S ip addr add 172.30.0.1/24 dev s-br
x $L ip addr add 10.77.0.2/24 dev c-ext; x $L ip addr add 10.77.0.3/24 dev c-ext
x $C ip addr add 172.30.0.2/24 dev c-br
for n in $S $C $L; do x $n ip link set lo up; done
x $S ip link set s-ext up; x $S ip link set s-br up; x $L ip link set c-ext up; x $C ip link set c-br up
x $C ip route add default via 172.30.0.1
x $S sysctl -qw net.ipv4.ip_forward=1
# Docker-like plumbing: DNAT of the published port, FORWARD -> DOCKER-USER first, then an accept (Docker's own accept rules).
x $S iptables -N DOCKER-USER; x $S iptables -A DOCKER-USER -j RETURN
x $S iptables -A FORWARD -j DOCKER-USER; x $S iptables -A FORWARD -j ACCEPT
x $S iptables -t nat -A PREROUTING -p tcp --dport 8443 -j DNAT --to-destination 172.30.0.2:8443
mkdir -p "${TMPDIR:-/tmp}/aw-fwtest-web"; echo hello-from-container > "${TMPDIR:-/tmp}/aw-fwtest-web/index.html"
( cd "${TMPDIR:-/tmp}/aw-fwtest-web" && sudo ip netns exec $C python3 -m http.server 8443 --bind 172.30.0.2 >/dev/null 2>&1 & )
sleep 1
get() { # source -> prints OK if the response body arrives
  x $L timeout 5 python3 -c "
import socket,sys
s=socket.socket(); s.bind(('$1',0)); s.settimeout(3)
try:
    s.connect(('10.77.0.1',8443)); s.sendall(b'GET / HTTP/1.0\r\n\r\n'); d=b''
    while True:
        c=s.recv(4096)
        if not c: break
        d+=c
    print('OK' if b'hello-from-container' in d else 'NODATA')
except Exception as e: print('FAIL')" 2>/dev/null
}
fail=0; check() { if [ "$2" = "$3" ]; then echo "ok   : $1 ($3)"; else echo "FAIL : $1 (got $3, wanted $2)"; fail=1; fi; }

check "before the rules: allowed source reaches the container" OK "$(get 10.77.0.2)"
check "before the rules: other source ALSO reaches it (the ufw/Docker bypass)" OK "$(get 10.77.0.3)"
sudo ip netns exec $S env ALLWIN_EXPOSE_EXT_IF=s-ext "$here/firewall-open.sh" 10.77.0.2 --test-source >/dev/null 2>&1; check "firewall-open exits 0" 0 $?
check "after the rules: allowed source connects AND receives data (replies pass)" OK "$(get 10.77.0.2)"
check "after the rules: other source times out" FAIL "$(get 10.77.0.3)"
# the container's own outbound connection to some host:8443 is not caught by the hook
x $L python3 -c "
import socket,threading
s=socket.socket(); s.bind(('10.77.0.2',9443)); s.listen(1); s.settimeout(6)
" 2>/dev/null
x $S bash -c "iptables -S DOCKER-USER | grep -c 'ctdir ORIGINAL'" | { read n; check "hook is restricted to the original direction and the external interface" 1 "$n"; }
x $S bash -c "iptables -S DOCKER-USER | grep -c -- '-i s-ext'" | { read n; check "hook names the external interface" 1 "$n"; }
# connection limit: 5 simultaneous connections from the allowed source, the 5th is refused
x $L python3 -c "
import socket,time
socks=[]; ok=0
for i in range(6):
    s=socket.socket(); s.bind(('10.77.0.2',0)); s.settimeout(2)
    try: s.connect(('10.77.0.1',8443)); socks.append(s); ok+=1
    except Exception: pass
print(ok)" 2>/dev/null | { read n; if [ "$n" -le 5 ] && [ "$n" -ge 3 ]; then echo "ok   : connection limit holds ($n of 6 simultaneous connections admitted)"; else echo "FAIL : connection limit ($n of 6)"; fail=1; fi; }
sudo ip netns exec $S env ALLWIN_EXPOSE_EXT_IF=s-ext "$here/firewall-open.sh" 10.77.0.2 --test-source >/dev/null 2>&1
x $S bash -c "iptables -S DOCKER-USER | grep -c ALLWIN-ADAPTER" | { read n; check "second open is idempotent (one hook)" 1 "$n"; }
sudo ip netns exec $S "$here/firewall-close.sh" >/dev/null 2>&1; check "firewall-close exits 0 and verifies" 0 $?
x $S bash -c "iptables -S DOCKER-USER | grep -c ALLWIN-ADAPTER; iptables -S ALLWIN-ADAPTER >/dev/null 2>&1 && echo chain-left" | tr '\n' ' ' | { read a; check "after close: no hook, no chain" "0" "${a% }"; }
check "after close: the other source is back to the unfiltered state (the container itself is stopped by expose-close)" OK "$(get 10.77.0.3)"
[ "$fail" = 0 ] && echo "firewall tests OK" || { echo "firewall tests FAILED"; exit 1; }
