#!/usr/bin/env bash
# Functional test of firewall-open.sh / firewall-close.sh in throwaway network namespaces (the host firewall is not touched). Needs sudo and iproute2.
#   infra/exposure/test-firewall.sh
# Topology: [cl: 10.77.0.2 (allowed) and 10.77.0.3 (other)] -- s-ext/c-ext -- [srv: DNAT 8443 -> cont, FORWARD -> DOCKER-USER] -- [cont: 172.30.0.2:8443 web]
set -uo pipefail
here="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
S=aw-srv; C=aw-cont; C2=aw-cont2; L=aw-cl
cleanup() { for n in "$C" "$L"; do sudo ip netns pids "$n" 2>/dev/null | xargs -r sudo kill 2>/dev/null; done; for n in $S $C $C2 $L; do sudo ip netns del $n 2>/dev/null; done; }
trap cleanup EXIT; cleanup
sudo ip netns add $S && sudo ip netns add $C && sudo ip netns add $C2 && sudo ip netns add $L || { echo "cannot create namespaces"; exit 1; }
sudo ip link add s-ext type veth peer name c-ext && sudo ip link set s-ext netns $S && sudo ip link set c-ext netns $L
sudo ip link add s-br type veth peer name c-br && sudo ip link set s-br netns $S && sudo ip link set c-br netns $C
sudo ip link add s-br2 type veth peer name c-br2 && sudo ip link set s-br2 netns $S && sudo ip link set c-br2 netns $C2   # another Docker network
x() { local ns=$1; shift; sudo ip netns exec "$ns" "$@"; }
x $S ip addr add 10.77.0.1/24 dev s-ext; x $S ip addr add 172.30.0.1/24 dev s-br; x $S ip addr add 172.31.0.1/24 dev s-br2
x $L ip addr add 10.77.0.2/24 dev c-ext; x $L ip addr add 10.77.0.3/24 dev c-ext
x $C ip addr add 172.30.0.2/24 dev c-br; x $C2 ip addr add 172.31.0.2/24 dev c-br2
for n in $S $C $C2 $L; do x $n ip link set lo up; done
x $S ip link set s-ext up; x $S ip link set s-br up; x $L ip link set c-ext up; x $C ip link set c-br up
x $C ip route add default via 172.30.0.1
x $S ip link set s-br2 up; x $C2 ip link set c-br2 up; x $C2 ip route add default via 172.31.0.1
x $S sysctl -qw net.ipv4.ip_forward=1
# Docker-like plumbing: DNAT of the published port, FORWARD -> DOCKER-USER first, then an accept (Docker's own accept rules).
x $S iptables -N DOCKER-USER; x $S iptables -A DOCKER-USER -j RETURN
x $S iptables -A FORWARD -j DOCKER-USER; x $S iptables -A FORWARD -j ACCEPT
x $S iptables -t nat -A PREROUTING ! -i s-br -p tcp --dport 8443 -j DNAT --to-destination 172.30.0.2:8443   # published port; Docker skips the bridge itself
x $S iptables -t nat -A POSTROUTING -s 172.30.0.0/24 -o s-ext -j MASQUERADE   # containers' outbound traffic, as Docker does
mkdir -p "${TMPDIR:-/tmp}/aw-fwtest-web"; echo hello-from-container > "${TMPDIR:-/tmp}/aw-fwtest-web/index.html"
( cd "${TMPDIR:-/tmp}/aw-fwtest-web" && sudo ip netns exec $C python3 -m http.server 8443 --bind 172.30.0.2 >/dev/null 2>&1 & )
mkdir -p "${TMPDIR:-/tmp}/aw-fwtest-out"; echo hello-from-outside > "${TMPDIR:-/tmp}/aw-fwtest-out/index.html"
( cd "${TMPDIR:-/tmp}/aw-fwtest-out" && sudo ip netns exec $L python3 -m http.server 8443 --bind 10.77.0.2 >/dev/null 2>&1 & )
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

getc2() { x $C2 timeout 6 python3 -c "
import socket
s=socket.socket(); s.settimeout(3)
try:
    s.connect(('10.77.0.1',8443)); s.sendall(b'GET / HTTP/1.0\r\n\r\n'); d=b''
    while True:
        c=s.recv(4096)
        if not c: break
        d+=c
    print('OK' if b'hello-from-container' in d else 'NODATA')
except Exception: print('FAIL')" 2>/dev/null; }
check "before the rules: allowed source reaches the container" OK "$(get 10.77.0.2)"
check "before the rules: a container on ANOTHER Docker network reaches the gateway via <host>:8443" OK "$(getc2)"
check "before the rules: other source ALSO reaches it (the ufw/Docker bypass)" OK "$(get 10.77.0.3)"
sudo ip netns exec $S env ALLWIN_FIREWALL_TEST_MODE=1 ALLWIN_EXPOSE_EXT_IF=s-ext "$here/firewall-open.sh" 10.77.0.2 --gw-bridge s-br --test-source >/dev/null 2>&1; check "firewall-open exits 0" 0 $?
check "after the rules: allowed source connects AND receives data (replies pass)" OK "$(get 10.77.0.2)"
check "after the rules: other source times out" FAIL "$(get 10.77.0.3)"
check "after the rules: a container on ANOTHER Docker network is dropped too (only the VPS)" FAIL "$(getc2)"
# the container's own OUTBOUND connection to some host:8443 (and its replies) is not caught by the hook
out="$(x $C timeout 8 python3 -c "
import socket
s=socket.socket(); s.settimeout(4)
try:
    s.connect(('10.77.0.2',8443)); s.sendall(b'GET / HTTP/1.0\r\n\r\n'); d=b''
    while True:
        c=s.recv(4096)
        if not c: break
        d+=c
    print('OK' if b'hello-from-outside' in d else 'NODATA')
except Exception: print('FAIL')" 2>/dev/null)"
check "after the rules: a container's outbound connection to host:8443 still works (hook is inbound-only)" OK "$out"
hook="$(x $S iptables -S DOCKER-USER | grep -- '-j ALLWIN-ADAPTER')"
case "$hook" in *"--ctdir ORIGINAL"*) h1=1 ;; *) h1=0 ;; esac; check "hook is restricted to the original direction" 1 "$h1"
case "$hook" in *"-i s-ext"*) h2=1 ;; *) h2=0 ;; esac; check "hook names the external interface" 1 "$h2"
case "$(x $S iptables -S DOCKER-USER)" in *"allwin-adapter-bridge"*"allwin-adapter-bridge"*) h3=2 ;; *) h3=0 ;; esac; check "bridge drop and gateway-bridge keep rules are present" 2 "$h3"
# connection limit: 5 simultaneous connections from the allowed source, the 5th is refused
n="$(x $L python3 -c "
import socket,time
socks=[]; ok=0
for i in range(6):
    s=socket.socket(); s.bind(('10.77.0.2',0)); s.settimeout(2)
    try: s.connect(('10.77.0.1',8443)); socks.append(s); ok+=1
    except Exception: pass
print(ok)" 2>/dev/null)"
if [ "${n:-0}" -le 5 ] && [ "${n:-0}" -ge 3 ]; then echo "ok   : connection limit holds ($n of 6 simultaneous connections admitted)"; else echo "FAIL : connection limit (${n:-?} of 6)"; fail=1; fi
sudo ip netns exec $S env ALLWIN_FIREWALL_TEST_MODE=1 ALLWIN_EXPOSE_EXT_IF=s-ext "$here/firewall-open.sh" 10.77.0.2 --gw-bridge s-br --test-source >/dev/null 2>&1
n="$(x $S iptables -S DOCKER-USER | grep -c -- "-j ALLWIN-ADAPTER")"; check "second open is idempotent (one hook)" 1 "$n"
n="$(x $S iptables -S DOCKER-USER | grep -c allwin-adapter-bridge)"; check "second open is idempotent (two bridge rules)" 2 "$n"
sudo ip netns exec $S "$here/firewall-close.sh" >/dev/null 2>&1; check "firewall-close exits 0 and verifies" 0 $?
left="$(x $S iptables -S DOCKER-USER | grep -c -E -- "ALLWIN-ADAPTER|allwin-adapter-bridge")"; x $S iptables -S ALLWIN-ADAPTER >/dev/null 2>&1 && left=$((left+1)); check "after close: no hook, no bridge rules, no chain" 0 "$left"
check "after close: the other source is back to the unfiltered state (the container itself is stopped by expose-close)" OK "$(get 10.77.0.3)"
[ "$fail" = 0 ] && echo "firewall tests OK" || { echo "firewall tests FAILED"; exit 1; }
