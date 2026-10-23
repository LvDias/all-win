#!/usr/bin/env bash
# Provision the All Win dev host (Ubuntu 24.04) with the ADR-0001 toolchains,
# shared by the users in group `devs`. Run as root: sudo bash provision-dev-host.sh
# Idempotent: safe to re-run; finished steps are detected and skipped.
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
NODE_DIR=/opt/node
RUST_DIR=/opt/rust
PLAYWRIGHT_DIR=/opt/ms-playwright   # shared browsers, filled by `playwright install`
WORKTREES_DIR=/srv/www/allwin.worktrees

[[ $EUID -eq 0 ]] || { echo "run as root (sudo)" >&2; exit 1; }
getent group "$DEV_GROUP" >/dev/null || { echo "group $DEV_GROUP missing" >&2; exit 1; }

# sudo resets PATH; npm/pnpm shebangs and install scripts need `node` on it.
export RUSTUP_HOME="$RUST_DIR/rustup" CARGO_HOME="$RUST_DIR/cargo"
export PATH="$NODE_DIR/bin:$CARGO_HOME/bin:/usr/local/sbin:/usr/local/bin:/usr/sbin:/usr/bin:/sbin:/bin"

step() { echo; echo "== $* =="; }

shared_dir() {  # group-writable, setgid directory owned by root:devs
  install -d -o root -g "$DEV_GROUP" -m 2775 "$1"
}
fix_shared() {
  chown -R root:"$DEV_GROUP" "$1"
  chmod -R g+rwX "$1"
  find "$1" -type d -exec chmod g+s {} +
}

step "ufw status before (read-only)"; ufw status verbose || true

step "apt packages"
export DEBIAN_FRONTEND=noninteractive
apt-get update
apt-get install -y ca-certificates curl gnupg build-essential pkg-config libssl-dev \
  xz-utils unzip jq postgresql-client dotnet-sdk-10.0

step "Docker Engine + compose plugin"
install -m 0755 -d /etc/apt/keyrings
if [[ ! -s /etc/apt/keyrings/docker.asc ]]; then
  curl -fsSL https://download.docker.com/linux/ubuntu/gpg -o /etc/apt/keyrings/docker.asc
fi
chmod a+r /etc/apt/keyrings/docker.asc
. /etc/os-release
docker_list="deb [arch=$(dpkg --print-architecture) signed-by=/etc/apt/keyrings/docker.asc] https://download.docker.com/linux/ubuntu ${VERSION_CODENAME} stable"
if [[ "$(cat /etc/apt/sources.list.d/docker.list 2>/dev/null)" != "$docker_list" ]]; then
  echo "$docker_list" > /etc/apt/sources.list.d/docker.list
  apt-get update
fi
install -d -m 0755 /etc/docker
if [[ ! -f /etc/docker/daemon.json ]]; then
  cat > /etc/docker/daemon.json <<'JSON'
{
  "ip": "127.0.0.1",
  "log-driver": "local"
}
JSON
  restart_docker=1
fi
jq -e '.ip == "127.0.0.1"' /etc/docker/daemon.json >/dev/null \
  || echo "WARNING: /etc/docker/daemon.json has no \"ip\": \"127.0.0.1\"; published ports may bind to all interfaces" >&2
apt-get install -y docker-ce docker-ce-cli containerd.io docker-buildx-plugin docker-compose-plugin
systemctl enable --now docker
if [[ ${restart_docker:-0} -eq 1 ]]; then systemctl restart docker; fi
for u in "${DEV_USERS[@]}"; do
  id -nG "$u" | tr ' ' '\n' | grep -qx docker || usermod -aG docker "$u"
done

step "Node.js $NODE_VERSION + pnpm $PNPM_VERSION ($NODE_DIR)"
have_node=$("$NODE_DIR/bin/node" --version 2>/dev/null || true)
have_pnpm=$("$NODE_DIR/bin/pnpm" --version 2>/dev/null || true)
if [[ $have_node == "v$NODE_VERSION" && $have_pnpm == "$PNPM_VERSION" ]]; then
  echo "already installed"
else
  tmp=$(mktemp -d)
  curl -fsSL "https://nodejs.org/dist/v${NODE_VERSION}/node-v${NODE_VERSION}-linux-x64.tar.xz" -o "$tmp/node.tar.xz"
  curl -fsSL "https://nodejs.org/dist/v${NODE_VERSION}/SHASUMS256.txt" -o "$tmp/SHASUMS256.txt"
  (cd "$tmp" && grep " node-v${NODE_VERSION}-linux-x64.tar.xz\$" SHASUMS256.txt | sed 's/ node-v.*/ node.tar.xz/' | sha256sum -c -)
  rm -rf "$NODE_DIR"   # re-extract clean (also drops a partial earlier run)
  shared_dir "$NODE_DIR"
  tar -xJf "$tmp/node.tar.xz" -C "$NODE_DIR" --strip-components=1 --no-same-owner
  rm -rf "$tmp"
  npm install -g --prefix "$NODE_DIR" "pnpm@${PNPM_VERSION}"
fi
fix_shared "$NODE_DIR"

step "Rust $RUST_TOOLCHAIN ($RUST_DIR)"
shared_dir "$RUST_DIR"
if [[ -x $CARGO_HOME/bin/rustup ]]; then
  rustup toolchain install "$RUST_TOOLCHAIN" --profile minimal -c rustfmt,clippy
else
  curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs \
    | sh -s -- -y --no-modify-path --profile minimal --default-toolchain "$RUST_TOOLCHAIN" -c rustfmt,clippy
fi
rustup default "$RUST_TOOLCHAIN"
if sqlx --version 2>/dev/null | grep -qF " $SQLX_CLI_VERSION"; then
  echo "sqlx-cli $SQLX_CLI_VERSION already installed"
else
  cargo install sqlx-cli --version "$SQLX_CLI_VERSION" --locked --force \
    --no-default-features --features rustls,postgres
fi
fix_shared "$RUST_DIR"

step "Environment for every session"
cat > /etc/profile.d/allwin-dev.sh <<'SH'
export RUSTUP_HOME=/opt/rust/rustup
export CARGO_HOME=/opt/rust/cargo
export DOTNET_CLI_TELEMETRY_OPTOUT=1
case ":$PATH:" in *:/opt/rust/cargo/bin:*) ;; *) PATH="/opt/rust/cargo/bin:$PATH" ;; esac
case ":$PATH:" in *:/opt/node/bin:*) ;; *) PATH="/opt/node/bin:$PATH" ;; esac
export PATH
export PLAYWRIGHT_BROWSERS_PATH=/opt/ms-playwright
# Shared host (12 CPUs, many parallel worktrees): cap cargo parallelism.
export CARGO_BUILD_JOBS=4
# Group-writable files for agents in group devs (pnpm/cargo/git outputs, shared worktrees).
case " $(id -nG 2>/dev/null) " in *" devs "*) umask 002 ;; esac
SH
for kv in RUSTUP_HOME=/opt/rust/rustup CARGO_HOME=/opt/rust/cargo DOTNET_CLI_TELEMETRY_OPTOUT=1 PLAYWRIGHT_BROWSERS_PATH=/opt/ms-playwright CARGO_BUILD_JOBS=4; do
  grep -q "^${kv%%=*}=" /etc/environment || echo "$kv" >> /etc/environment
done
shared_dir "$PLAYWRIGHT_DIR"
for b in "$CARGO_HOME"/bin/* "$NODE_DIR"/bin/{node,npm,npx,pnpm,pnpx}; do
  if [[ -e $b ]]; then ln -sfn "$b" "/usr/local/bin/$(basename "$b")"; fi
done

step "git safe.directory for slice worktrees"
# git 2.43 ignores the "/srv/www/*" pattern; each worktree path needs its own entry.
for wt in "$WORKTREES_DIR"/*/; do
  [[ -d $wt ]] || continue
  wt=${wt%/}
  git config --system --get-all safe.directory | grep -qxF "$wt" \
    || git config --system --add safe.directory "$wt"
done

step "ufw status after (read-only)"; ufw status verbose || true

step "versions"
rustc --version; cargo --version; cargo clippy --version; rustfmt --version; sqlx --version
node --version; pnpm --version; dotnet --version; docker --version; docker compose version; psql --version
echo
echo "Done. Users must log out and back in (or restart their agent sessions) to get the docker group and env."
