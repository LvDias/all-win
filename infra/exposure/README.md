# E2E exposure of the plugin gateway (port 8443) — plan

**Purpose.** For the Sunday end-to-end test, the NinjaTrader plugin on the user's Windows VPS must reach the dev Core. The user
authorized exposing **only TCP 8443** of the plugin gateway to the internet: TLS plus the device token, reachable from **one source IP**
(the Windows VPS), rate-limited, **closed right after the test** until a staging environment exists.
Nothing here is applied by merging this folder. **Do not run `firewall-open.sh` or start `caddy-adapter` before the gates below.**

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
| Listener | one extra container, `caddy-adapter` (`docker-compose.expose.yml`), publishes `8443:8443`; no restart policy, read-only root, all capabilities dropped but `NET_BIND_SERVICE`, 256 MB, 200 pids |
| Routes | `Caddyfile.adapter`: **only** `/adapter/v1` and `/adapter/v1/*` (plugin HTTP and the WebSocket) go to the Core; every other path (web, `/api`, `/ingress`, `/metrics`, scanners) gets the connection closed with no response (`abort`) |
| Logging | URI and all request headers deleted from the access log (the Bearer device token is in a header); no request body is logged |
| Body cap | 64 KB at the edge |
| Auth | the Core's own device-token check on `/adapter/v1` (token hashed at rest, per-token and per-IP limits in the Core) |
| Not reachable | the main dev Caddy (stays on `127.0.0.1`, moved to 8444 while the gateway uses 8443), postgres, the Core's own port, the web |

## TLS: how the plugin trusts the certificate without a public domain

The edge uses a **static self-signed certificate** (EC P-256, valid 14 days, SAN = the server's public IP), generated on the server by
`gen-e2e-cert.sh <DEV_SERVER_PUBLIC_IP>`. The private key stays in `~/.allwin-e2e-cert/` (mode 600 on the host), is mounted read-only into the
container, and is never committed. Caddy's internal CA is not used here because it issues short-lived leaf certificates (a pinned leaf would break).

Trust on the Windows VPS, two ways (the first needs no plugin change and is the plan for Sunday):

1. **Import the certificate as trusted** (the user, over RDP, as Administrator): copy `e2e.crt` through the exchange channel, then
   `certutil -addstore -f Root e2e.crt`, and check that the thumbprint equals the fingerprint printed by `gen-e2e-cert.sh` (given to the user in the
   Maestro's message, a different channel from the file). The plugin then uses its normal TLS validation. Remove it afterwards with
   `certutil -delstore Root <thumbprint>`.
2. **Pin the fingerprint in the plugin** (needs a small Plugin Engineer change: a dev-only setting `GatewayCertSha256` compared in the
   `ServerCertificateValidationCallback`, with no bypass of hostname/IP or expiry checks beyond the pin). Not required for Sunday.

Never turn certificate validation off in the plugin, even for the test.

## Firewall

Docker-published ports **bypass ufw**, so the control is the `DOCKER-USER` chain (`firewall-open.sh <VPS_PUBLIC_IP>`):

* the hook only matches traffic whose original destination port is 8443;
* chain `ALLWIN-ADAPTER`: from the allowed IP at most **4 concurrent** connections (`connlimit`), at most **12 new connections per minute, burst 6**
  (`hashlimit`), then ACCEPT; **every other source is dropped**;
* IPv6 on 8443 is dropped;
* the script refuses private, loopback and CGNAT addresses and any address of this host, and has `--dry-run`;
* the rules are in memory only, so a reboot closes the port; `restart: "no"` keeps the container down too.

`ufw` itself is unchanged (only OpenSSH is allowed in). The user-requested `ufw allow from <VPS_PUBLIC_IP> to any port 8443 proto tcp` is **not**
sufficient for a Docker-published port, which is why the equivalent restriction is in `DOCKER-USER`. Tested in a throwaway network namespace:
the rules load, a second `open` is idempotent, `close` removes the hook and the chain; the host firewall was not touched.

## Open (when the gates are met; needs sudo)

```bash
cd infra/exposure && ./gen-e2e-cert.sh <DEV_SERVER_PUBLIC_IP>        # prints the fingerprint; send it to the user via the Maestro
cd ../compose
# The dev edge normally publishes 127.0.0.1:8443; move it to 8444 so 8443 is free for the gateway (set CADDY_HTTPS_PORT=8444 in .env).
CADDY_HTTPS_PORT=8444 ALLWIN_E2E_CERT_DIR="$HOME/.allwin-e2e-cert" docker compose -f docker-compose.dev.yml \
  -f ../exposure/docker-compose.expose.yml --env-file .env --profile core up -d caddy caddy-adapter
../exposure/firewall-open.sh <VPS_PUBLIC_IP>
```

Container files: the key must be readable by the container user; if Caddy cannot read it, run `chmod 640` and `chgrp` it to the container's group
(see the log of `caddy-adapter`), never `644`.

## Verify (from outside the server, not from the server)

* From the Windows VPS: `TCP 8443` connects; `https://<DEV_SERVER_PUBLIC_IP>:8443/adapter/v1/...` answers; any other path closes the connection.
* From any other host: the connection to 8443 times out (dropped).
* On the server: `sudo iptables -S ALLWIN-ADAPTER`, `docker ps` shows only `caddy-adapter` publishing 8443, `ss -ltn | grep 8443`.

## Close (right after the test, or if anything looks wrong)

```bash
infra/exposure/firewall-close.sh
```

It stops and removes `caddy-adapter`, deletes the DOCKER-USER hook and the `ALLWIN-ADAPTER` chain, and deletes the certificate and key; then it
checks that nothing listens on 8443. The user also removes the certificate from the Windows Root store (`certutil -delstore Root <thumbprint>`).
The device token used in the test is revoked in the Core.

## Residual risks (accepted for a short, supervised window)

* The device token is a bearer secret over TLS from one IP; the IP restriction is the main barrier before token validation. IP spoofing for a TCP
  session is not practical; a compromised VPS could use the token, which is why the test uses Sim accounts only and the token is revoked after.
* `connlimit`/`hashlimit` bound connection and attempt rates but are not a full DDoS defense.
* A self-signed certificate imported into the Windows Root store is trusted for any site on that machine until removed: remove it after the test.
