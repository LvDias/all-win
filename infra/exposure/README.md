# E2E exposure of the plugin gateway (port 8443) — plan

**Purpose.** For the Sunday end-to-end test, the NinjaTrader plugin on the user's Windows VPS must reach the dev Core. The user
authorized exposing **only TCP 8443** of the plugin gateway to the internet: TLS plus the device token, reachable from **one source IP**
(the Windows VPS), rate-limited, **closed right after the test** until a staging environment exists.
Nothing here is applied by merging this folder. **Do not run `expose-open.sh` before the gates below, and never start `caddy-adapter` by hand.**

The real source IP lives only on the host (command-line argument, never in the repository); this document uses `<VPS_PUBLIC_IP>`
and `<DEV_SERVER_PUBLIC_IP>`.

## Gates (all required before opening)

1. This plan approved by QA Security (Vigia and Escudo) on its exact hash.
2. The dev stack is up and healthy (postgres, core, caddy) and the Core has only **Sim** accounts.
3. The test moment agreed with the user (the Maestro tells DevOps: "open").
4. The user's authorization for this exact change is on record (decision of 2026-10-24).

## What is exposed

| Layer | Setting |
|---|---|
| Listener | one extra container, `caddy-adapter` (`docker-compose.expose.yml`), publishes `8443:8443`; runs as the key owner's uid:gid, no restart policy, gate marker required to start, read-only root, **all capabilities dropped**, 256 MB, 200 pids |
| Routes | `Caddyfile.adapter`: **only** `/adapter/v1` and `/adapter/v1/*` (plugin HTTP and the WebSocket) go to the Core; every other path (web, `/api`, `/ingress`, `/metrics`, scanners) gets the connection closed with no response (`abort`) |
| Logging | URI and all request headers deleted from the access log (the Bearer device token is in a header); no request body is logged |
| Body cap | 64 KB at the edge |
| Auth | the Core's own device-token check on `/adapter/v1` (token hashed at rest, per-token and per-IP limits in the Core) |
| Not reachable | the main dev Caddy (stays on `127.0.0.1`, moved to 8444 while the gateway uses 8443), postgres, the Core's own port, the web |

## TLS: how the plugin trusts the certificate without a public domain

The edge uses a **static self-signed leaf certificate**: EC P-256, **`basicConstraints=critical,CA:FALSE`**, `keyUsage=critical,digitalSignature`,
EKU `serverAuth`, SAN = the server's public IP, valid **3 days** (the test window; at most 7). It is generated on the server by
`gen-e2e-cert.sh <DEV_SERVER_PUBLIC_IP>`; the private key stays in `~/.allwin-e2e-cert/` (mode 600, owner = the account that runs the gateway), is
mounted read-only into a container that runs as that same uid:gid, and is never committed. It must not be a CA: a trusted CA key would let whoever
reads it mint certificates for any site the Windows machine trusts. Caddy's internal CA is not used because its leaf certificates are short-lived.

* **Plan A (Plugin Engineer): pin the certificate in the dev plugin.** A dev-only setting with the SHA-256 fingerprint of this certificate, checked in
  the `ServerCertificateValidationCallback`; no change to the Windows machine, and only the plugin trusts this exact certificate. The fingerprint is
  not secret; it reaches the user and the plugin owner in the Maestro's message, a different channel from the file.
* **Fallback (fastest, if the pin is not ready): per-user trust.** The user, as the account that runs NinjaTrader, copies `e2e.crt` through the
  exchange channel and runs `certutil -user -addstore Root e2e.crt` (CurrentUser store, not the machine store), checks the thumbprint against the
  fingerprint from the Maestro's message, and removes it right after the test with `certutil -user -delstore Root <thumbprint>`. Because the leaf is
  `CA:FALSE` it can only authenticate that server name/IP.

Never turn certificate validation off in the plugin, even for the test.

## Firewall and the only way to start the gateway

Docker-published ports **bypass ufw**, so the control is the `DOCKER-USER` chain (`firewall-open.sh <VPS_PUBLIC_IP>`, normally called by `expose-open.sh`):

* the hook matches only packets that **enter from the external interface** (the one of the default route), in the **original direction**
  (`--ctdir ORIGINAL`), to a published port 8443; replies from the container and a container's own outbound connections to some host:8443 never
  match;
* chain `ALLWIN-ADAPTER`: from the allowed IP at most **4 concurrent** connections (`connlimit`), at most **12 new connections per minute, burst 6**
  (`hashlimit`), then ACCEPT; **every other source is dropped**; IPv6 on 8443 is dropped on the external interface;
* the script refuses private, loopback, CGNAT, link-local, multicast/reserved (224/3) and documentation addresses, octets with leading zeros, and any
  address of this host; it has `--dry-run`; it **verifies** the hook, the accept and the final drop with `iptables -C` and fails otherwise;
* the rules are in memory only, so a reboot removes them. `restart: "no"` keeps the container down, and the container **refuses to start** unless the
  gate marker `/run/allwin-expose/OPEN` exists (written by `expose-open.sh` only after the firewall was applied and verified, removed by
  `expose-close.sh`; `/run` is a tmpfs, so a reboot removes it too). **Never start `caddy-adapter` with `docker compose` by hand.**

`ufw` itself is unchanged (only OpenSSH is allowed in). The requested `ufw allow from <VPS_PUBLIC_IP> to any port 8443 proto tcp` alone would not
restrict a Docker-published port, which is why the restriction is in `DOCKER-USER`.

Tested functionally with `test-firewall.sh` in throwaway network namespaces (client with an allowed and another source, a DNAT/FORWARD/`DOCKER-USER`
setup like Docker's, a web server as the "container"; the host firewall is not touched): before the rules both sources reach the container (the bypass);
after `firewall-open.sh` the allowed source connects **and receives data**, the other source times out, the connection limit holds, a second open is
idempotent, and `firewall-close.sh` removes everything.

## Open (when the gates are met; needs sudo)

```bash
infra/exposure/gen-e2e-cert.sh <DEV_SERVER_PUBLIC_IP>   # prints the fingerprint; send it to the user and the plugin owner via the Maestro
infra/exposure/expose-open.sh <VPS_PUBLIC_IP>           # firewall (verified) -> gate marker -> caddy-adapter -> checks
```

`expose-open.sh` moves the dev edge from 8443 to 8444 (`CADDY_HTTPS_PORT=8444`) so 8443 is free for the gateway, and closes everything again if any step
fails. The WebSocket frame size limit after the upgrade is the Core's (the 64 KB cap above applies to HTTP request bodies, not to WebSocket frames).

## Verify (from outside the server, not from the server)

* From the Windows VPS: `TCP 8443` connects; `https://<DEV_SERVER_PUBLIC_IP>:8443/adapter/v1/...` answers; any other path closes the connection.
* From any other host: the connection to 8443 times out (dropped).
* On the server: `sudo iptables -S ALLWIN-ADAPTER`, `docker ps` shows only `caddy-adapter` publishing 8443, `ss -ltn | grep 8443`.

## Close (right after the test, or if anything looks wrong)

```bash
infra/exposure/expose-close.sh
```

It removes the gate marker first (a container that restarts then refuses to start), removes `caddy-adapter`, runs `firewall-close.sh` (hook, chain, IPv6
drop), deletes the certificate and key, and then **verifies** each of them (container, marker, certificate and key, hook, chain, nothing listening
on 8443). Any leftover prints what remains and exits non-zero with **`NOT CLOSED`**. The user also removes the certificate from the Windows store
(`certutil -user -delstore Root <thumbprint>` if the fallback was used), and the device token used in the test is revoked in the Core.

## Residual risks (accepted for a short, supervised window)

* The device token is a bearer secret over TLS from one IP; the IP restriction is the main barrier before token validation; an account that can run Docker on this host (and agents running as the key owner) can read the key, so keep the window short. IP spoofing for a TCP
  session is not practical; a compromised VPS could use the token, which is why the test uses Sim accounts only and the token is revoked after.
* `connlimit`/`hashlimit` bound connection and attempt rates but are not a full DDoS defense.
* The certificate is a `CA:FALSE` leaf valid 3 days, so even if it is trusted on the Windows machine it cannot sign anything; still remove it after the test.
