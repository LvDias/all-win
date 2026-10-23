#!/usr/bin/env bash
# Provision the All Win dev host (Ubuntu 24.04) with the ADR-0001 toolchains,
# shared by the users in group `devs`. Run once as root: sudo bash provision-dev-host.sh
#
# Firewall: this script does not touch ufw. Docker manages its own iptables chains
# (NAT/FORWARD) for container networking; daemon.json binds published ports to
# 127.0.0.1 by default, so no container port is reachable from outside the host.
set -euo pipefail

NODE_VERSION="${NODE_VERSION:-24.22.1}"      # Node.js Active LTS (Krypton)
PNPM_VERSION="${PNPM_VERSION:-12.15.0}"
RUST_TOOLCHAIN="${RUST_TOOLCHAIN:-1.99.1}"   # projects pin their own via rust-toolchain.toml
SQLX_CLI_VERSION="${SQLX_CLI_VERSION:-0.9.0}"
DEV_GROUP=devs
DEV_USERS=(dev1 dev2 dev3)

[[ $EUID -eq 0 ]] || { echo "run as root (sudo)" >&2; exit 1; }
getent group "$DEV_GROUP" >/dev/null || { echo "group $DEV_GROUP missing" >&2; exit 1; }

echo "== ufw status before (read-only) =="; ufw status verbose || true

export DEBIAN_FRONTEND=noninteractive
apt-get update
apt-get install -y ca-certificates curl gnupg build-essential pkg-config libssl-dev \
  xz-utils unzip jq postgresql-client dotnet-sdk-10.0

shared_dir() {  # group-writable, setgid directory owned by root:devs
  install -d -o root -g "$DEV_GROUP" -m 2775 "$1"
}
fix_shared() {
  chgrp -R "$DEV_GROUP" "$1"
  chmod -R g+rwX "$1"
  find "$1" -type d -exec chmod g+s {} +
}

# --- Docker Engine + compose plugin (official repository) ---
install -m 0755 -d /etc/apt/keyrings
curl -fsSL https://download.docker.com/linux/ubuntu/gpg -o /etc/apt/keyrings/docker.asc
chmod a+r /etc/apt/keyrings/docker.asc
. /etc/os-release
echo "deb [arch=$(dpkg --print-architecture) signed-by=/etc/apt/keyrings/docker.asc] https://download.docker.com/linux/ubuntu ${VERSION_CODENAME} stable" \
  > /etc/apt/sources.list.d/docker.list
install -d -m 0755 /etc/docker
if [[ ! -f /etc/docker/daemon.json ]]; then
  cat > /etc/docker/daemon.json <<'JSON'
{
  "ip": "127.0.0.1",
  "log-driver": "local"
}
JSON
fi
apt-get update
apt-get install -y docker-ce docker-ce-cli containerd.io docker-buildx-plugin docker-compose-plugin
systemctl enable --now docker
for u in "${DEV_USERS[@]}"; do usermod -aG docker "$u"; done

# --- Node.js LTS + pnpm (shared prefix /opt/node) ---
shared_dir /opt/node
tmp=$(mktemp -d)
curl -fsSL "https://nodejs.org/dist/v${NODE_VERSION}/node-v${NODE_VERSION}-linux-x64.tar.xz" -o "$tmp/node.tar.xz"
curl -fsSL "https://nodejs.org/dist/v${NODE_VERSION}/SHASUMS256.txt" -o "$tmp/SHASUMS256.txt"
(cd "$tmp" && grep " node-v${NODE_VERSION}-linux-x64.tar.xz\$" SHASUMS256.txt | sed 's/ node-v.*/ node.tar.xz/' | sha256sum -c -)
tar -xJf "$tmp/node.tar.xz" -C /opt/node --strip-components=1
rm -rf "$tmp"
/opt/node/bin/npm install -g --prefix /opt/node "pnpm@${PNPM_VERSION}"
fix_shared /opt/node

# --- Rust (shared rustup at /opt/rust) ---
shared_dir /opt/rust
export RUSTUP_HOME=/opt/rust/rustup CARGO_HOME=/opt/rust/cargo
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs \
  | sh -s -- -y --no-modify-path --profile minimal --default-toolchain "$RUST_TOOLCHAIN" -c rustfmt,clippy
/opt/rust/cargo/bin/rustup default "$RUST_TOOLCHAIN"
/opt/rust/cargo/bin/cargo install sqlx-cli --version "$SQLX_CLI_VERSION" --locked \
  --no-default-features --features rustls,postgres
fix_shared /opt/rust

# --- Environment for every session (login, non-login, non-interactive) ---
cat > /etc/profile.d/allwin-dev.sh <<'SH'
export RUSTUP_HOME=/opt/rust/rustup
export CARGO_HOME=/opt/rust/cargo
export DOTNET_CLI_TELEMETRY_OPTOUT=1
case ":$PATH:" in *:/opt/rust/cargo/bin:*) ;; *) PATH="/opt/rust/cargo/bin:$PATH" ;; esac
case ":$PATH:" in *:/opt/node/bin:*) ;; *) PATH="/opt/node/bin:$PATH" ;; esac
export PATH
SH
for kv in RUSTUP_HOME=/opt/rust/rustup CARGO_HOME=/opt/rust/cargo DOTNET_CLI_TELEMETRY_OPTOUT=1; do
  grep -q "^${kv%%=*}=" /etc/environment || echo "$kv" >> /etc/environment
done
for b in /opt/rust/cargo/bin/* /opt/node/bin/node /opt/node/bin/npm /opt/node/bin/npx /opt/node/bin/pnpm /opt/node/bin/pnpx; do
  [[ -e $b ]] && ln -sf "$b" "/usr/local/bin/$(basename "$b")"
done

echo "== ufw status after (read-only) =="; ufw status verbose || true
echo "== versions =="
RUSTUP_HOME=/opt/rust/rustup CARGO_HOME=/opt/rust/cargo bash -c \
  'rustc --version; cargo --version; cargo clippy --version; rustfmt --version; sqlx --version'
node --version; pnpm --version; dotnet --version; docker --version; docker compose version; psql --version
echo "Done. Users must log out and back in (or restart their agent sessions) to get the docker group and env."
