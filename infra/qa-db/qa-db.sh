#!/usr/bin/env bash
# Disposable PostgreSQL 16 for QA runs (one container per QA agent/run). See README.md.
#   qa-db.sh create  <name> --env-file PATH [--owner USER]
#   qa-db.sh reset   <name> --env-file PATH [--owner USER]   (destroy + create: new password, new port)
#   qa-db.sh destroy <name>
#   qa-db.sh status  [<name>]
set -euo pipefail
umask 077

IMAGE="postgres:16-alpine"
PREFIX="allwin-qa-db-"
LABEL="allwin.qa-db=1"

# Docker works for this user only through the docker group when not already in it.
if ! docker info >/dev/null 2>&1; then
  if [ -z "${QA_DB_UNDER_SG:-}" ] && command -v sg >/dev/null 2>&1; then
    export QA_DB_UNDER_SG=1
    exec sg docker -c "$(printf '%q ' "$0" "$@")"
  fi
  echo "error: docker is not usable by this user" >&2
  exit 1
fi

die() { echo "error: $*" >&2; exit 1; }

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
  [ -n "$env_file" ] || die "--env-file PATH is required (a git-ignored path outside the repo tree or core/.env.test of the QA worktree)"
  exists "$1" && die "$(cname "$1") already exists (use reset or destroy)"
  docker inspect "$(cname "$1")" >/dev/null 2>&1 && die "$(cname "$1") exists but is not a qa-db container"

  local pw; pw="$(openssl rand -hex 24)"
  # Password goes in by environment variable name only: never on a command line or in output.
  POSTGRES_PASSWORD="$pw" docker run -d --name "$(cname "$1")" --label "$LABEL" \
    --memory 1g --pids-limit 512 --restart no \
    -p 127.0.0.1::5432 -e POSTGRES_PASSWORD \
    --tmpfs /var/lib/postgresql/data:rw,size=1g \
    "$IMAGE" -c fsync=off -c synchronous_commit=off -c full_page_writes=off \
    -c max_connections=200 -c listen_addresses='*' >/dev/null

  local port="" i
  for i in $(seq 1 60); do
    if docker exec "$(cname "$1")" pg_isready -U postgres -q 2>/dev/null; then
      port="$(docker port "$(cname "$1")" 5432/tcp | head -1 | sed 's/.*://')"
      break
    fi
    sleep 1
  done
  [ -n "$port" ] || { docker rm -f -v "$(cname "$1")" >/dev/null 2>&1 || true; die "PostgreSQL did not become ready"; }
  case "$(docker port "$(cname "$1")" 5432/tcp | head -1)" in
    127.0.0.1:*) ;;
    *) docker rm -f -v "$(cname "$1")" >/dev/null; die "port is not bound to 127.0.0.1" ;;
  esac

  mkdir -p "$(dirname "$env_file")"
  ( umask 077; printf 'TEST_DATABASE_ADMIN_URL=postgres://postgres:%s@127.0.0.1:%s/postgres\n' "$pw" "$port" > "$env_file" )
  chmod 600 "$env_file"
  # Hand the file to the QA agent's account when it differs from the caller (mode stays 600).
  [ -z "$owner" ] || [ "$owner" = "$(id -un)" ] || sudo -n chown "$owner" "$env_file"
  echo "created $(cname "$1") on 127.0.0.1:$port; credentials in $env_file (mode 600)"
}

case "$cmd" in
  create)  [ -n "$name" ] || die "usage: create <name> --env-file PATH"; do_create "$name" ;;
  reset)   [ -n "$name" ] || die "usage: reset <name> --env-file PATH"; do_destroy "$name"; do_create "$name" ;;
  destroy) [ -n "$name" ] || die "usage: destroy <name>"; do_destroy "$name" ;;
  status)
    docker ps -a --filter "label=$LABEL" --format '{{.Names}}\t{{.Status}}\t{{.Ports}}' | sed "s/^${PREFIX}//" ;;
  *) die "usage: qa-db.sh create|reset|destroy|status <name> [--env-file PATH]" ;;
esac
