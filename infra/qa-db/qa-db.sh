#!/usr/bin/env bash
# Disposable PostgreSQL 16 for QA runs (one container per QA agent/run). See README.md.
#   qa-db.sh create  <name> --env-file PATH [--owner USER]
#   qa-db.sh reset   <name> --env-file PATH [--owner USER]   (destroy + create: new password, new port)
#   qa-db.sh destroy <name>
#   qa-db.sh status
set -euo pipefail
umask 077

IMAGE="postgres:16-alpine"
PREFIX="allwin-qa-db-"
LABEL="allwin.qa-db=1"

die() { echo "error: $*" >&2; exit 1; }

# Docker works for some accounts only through the docker group (sg). Re-run under it, via bash.
if ! docker info >/dev/null 2>&1; then
  if [ -z "${QA_DB_UNDER_SG:-}" ] && getent group docker | cut -d: -f4 | tr ',' '\n' | grep -qx "$(id -un)" && command -v sg >/dev/null 2>&1; then
    q() { printf "'%s'" "${1//\'/\'\\\'\'}"; }
    line="exec bash $(q "$0")"
    for a in "$@"; do line="$line $(q "$a")"; done
    export QA_DB_UNDER_SG=1
    exec sg docker -c "$line"
  fi
  die "docker is not usable by this user"
fi

cmd="${1:-}"; shift || true
name="${1:-}"; [ $# -gt 0 ] && shift || true
env_file=""
owner=""
while [ $# -gt 0 ]; do
  case "$1" in
    --env-file) env_file="${2:-}"; [ -n "$env_file" ] || die "--env-file needs a path"; shift 2 ;;
    --owner) owner="${2:-}"; [[ "$owner" =~ ^[a-z_][a-z0-9_-]{0,31}$ ]] || die "--owner needs a user name"; shift 2 ;;
    *) die "unknown argument: $1" ;;
  esac
done

valid_name() { [[ "$1" =~ ^[a-z0-9][a-z0-9-]{0,40}$ ]] || die "name must match ^[a-z0-9][a-z0-9-]{0,40}$"; }
cname() { echo "${PREFIX}$1"; }
exists() { docker inspect --format '{{index .Config.Labels "allwin.qa-db"}}' "$(cname "$1")" 2>/dev/null | grep -qx 1; }

# Refuse an unsafe env-file target before anything is created.
check_env_target() {
  local p="$1" d; d="$(dirname "$p")"
  case "$p" in */) die "--env-file must be a file path" ;; esac
  # New directories are 711, not 700 (umask 077): an --owner account must be able to reach the file.
  [ -d "$d" ] || install -d -m 711 -- "$d"
  [ ! -L "$d" ] || die "env file directory is a symlink: $d"
  if [ -L "$p" ]; then die "env file path is a symlink: $p"; fi
  if [ -e "$p" ] && [ ! -f "$p" ]; then die "env file path is not a regular file: $p"; fi
  # Absolute path from here on, so the git checks and the final rename see the same file.
  d="$(cd "$d" && pwd -P)"
  env_file="$d/$(basename "$p")"
  p="$env_file"
  if top="$(git -C "$d" rev-parse --show-toplevel 2>/dev/null)"; then
    git -C "$d" ls-files --error-unmatch -- "$p" >/dev/null 2>&1 && die "env file is tracked by git: $p"
    git -C "$d" check-ignore -q -- "$p" || die "env file inside a git worktree must be git-ignored: $p"
  fi
}

do_destroy() {
  valid_name "$1"
  if exists "$1"; then
    docker rm -f -v "$(cname "$1")" >/dev/null
    echo "destroyed $(cname "$1")"
  else
    echo "nothing to destroy: $(cname "$1")"
  fi
}

do_create() {
  valid_name "$1"
  [ -n "$env_file" ] || die "--env-file PATH is required"
  exists "$1" && die "$(cname "$1") already exists (use reset or destroy)"
  docker inspect "$(cname "$1")" >/dev/null 2>&1 && die "$(cname "$1") exists but is not a qa-db container"
  check_env_target "$env_file"

  local pw port="" i
  c=""; tmp=""; ok=0   # globals: the EXIT trap runs after this function returns
  c="$(cname "$1")"
  cleanup() {
    if [ "$ok" != 1 ]; then
      docker rm -f -v "$c" >/dev/null 2>&1 || true
      [ -z "$tmp" ] || rm -f -- "$tmp"
    fi
  }
  trap cleanup EXIT

  pw="$(openssl rand -hex 24)"
  # Password goes in by environment variable name only: never on a command line or in output.
  POSTGRES_PASSWORD="$pw" docker run -d --name "$c" --label "$LABEL" \
    --memory 1g --pids-limit 512 --restart no \
    -p 127.0.0.1::5432 -e POSTGRES_PASSWORD \
    --tmpfs /var/lib/postgresql/data:rw,size=1g \
    "$IMAGE" -c fsync=off -c synchronous_commit=off -c full_page_writes=off \
    -c max_connections=200 -c listen_addresses='*' >/dev/null

  # TCP on loopback: the entrypoint's temporary init server listens on the unix socket only,
  # so this succeeds only for the final server.
  for i in $(seq 1 60); do
    if docker exec "$c" pg_isready -h 127.0.0.1 -p 5432 -U postgres -q 2>/dev/null; then
      port="$(docker port "$c" 5432/tcp | head -1)"
      break
    fi
    sleep 1
  done
  [ -n "$port" ] || die "PostgreSQL did not become ready"
  case "$port" in 127.0.0.1:*) port="${port##*:}" ;; *) die "port is not bound to 127.0.0.1" ;; esac

  # Atomic write: new mode-600 file in the same directory, ownership handed over on the temp file,
  # then renamed over the target (never writes through a pre-existing path).
  tmp="$(mktemp -p "$(dirname "$env_file")" .env-qa-db.XXXXXX)"
  chmod 600 "$tmp"
  printf 'TEST_DATABASE_ADMIN_URL=postgres://postgres:%s@127.0.0.1:%s/postgres\n' "$pw" "$port" > "$tmp"
  if [ -n "$owner" ] && [ "$owner" != "$(id -un)" ]; then
    sudo -n chown -h -- "$owner" "$tmp"
  fi
  mv -f -- "$tmp" "$env_file"
  tmp=""
  ok=1
  echo "created $c on 127.0.0.1:$port; credentials in $env_file (mode 600)"
}

case "$cmd" in
  create)  [ -n "$name" ] || die "usage: create <name> --env-file PATH"; do_create "$name" ;;
  reset)   [ -n "$name" ] || die "usage: reset <name> --env-file PATH"; valid_name "$name"
           [ -z "$env_file" ] || check_env_target "$env_file"
           do_destroy "$name"; do_create "$name" ;;
  destroy) [ -n "$name" ] || die "usage: destroy <name>"; do_destroy "$name" ;;
  status)
    docker ps -a --filter "label=$LABEL" --format '{{.Names}}\t{{.Status}}\t{{.Ports}}' | sed "s/^${PREFIX}//" ;;
  *) die "usage: qa-db.sh create|reset|destroy|status <name> [--env-file PATH]" ;;
esac
