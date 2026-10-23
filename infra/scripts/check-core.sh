#!/usr/bin/env bash
# Core gate: fmt, clippy, sqlx offline data against a freshly migrated database, tests with database.
# Starts a throwaway PostgreSQL 16 on 127.0.0.1 (random free port, random passwords) and removes it on exit.
# Order matters: the sqlx check runs BEFORE the tests, because the test harness alters the allwin_* roles.
set -euo pipefail
ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
# Shared dev host keeps the toolchain in /opt/rust; elsewhere (CI runners) use the default locations.
if [ -d /opt/rust/rustup ]; then export RUSTUP_HOME="${RUSTUP_HOME:-/opt/rust/rustup}" CARGO_HOME="${CARGO_HOME:-/opt/rust/cargo}"; fi
command -v docker >/dev/null && docker info >/dev/null 2>&1 || { echo "docker is not usable by this user (group docker?)" >&2; exit 1; }

rnd() { python3 -c 'import secrets;print(secrets.token_urlsafe(18).replace("-","a").replace("_","b"))'; }
port="$(python3 -c 'import socket;s=socket.socket();s.bind(("127.0.0.1",0));print(s.getsockname()[1]);s.close()')"
su="$(rnd)"; ow="$(rnd)"; ap="$(rnd)"; wo="$(rnd)"; ma="$(rnd)"
name="allwin-check-pg-$$"
trap 'docker rm -f "$name" >/dev/null 2>&1 || true' EXIT

docker run -d --name "$name" -e POSTGRES_USER=postgres -e POSTGRES_PASSWORD="$su" -e POSTGRES_DB=postgres \
  -e ALLWIN_OWNER_PASSWORD="$ow" -e ALLWIN_APP_PASSWORD="$ap" -e ALLWIN_WORKER_PASSWORD="$wo" -e ALLWIN_MAINT_PASSWORD="$ma" \
  -v "$ROOT/infra/compose/postgres-init:/docker-entrypoint-initdb.d:ro" \
  -p "127.0.0.1:$port:5432" --tmpfs /var/lib/postgresql/data postgres:16-alpine >/dev/null
owner_url="postgres://allwin_owner:$ow@127.0.0.1:$port/allwin"
for _ in $(seq 1 60); do psql "$owner_url" -tAc 'select 1' >/dev/null 2>&1 && break; sleep 1; done
psql "$owner_url" -tAc 'select 1' >/dev/null || { echo "test database did not come up" >&2; exit 1; }

cd "$ROOT/core"
echo "== core: fmt";    cargo fmt --check
echo "== core: clippy"; cargo clippy --workspace --all-targets --all-features --locked -- -D warnings
echo "== core: sqlx offline data (migrate, prepare, git diff)"
DATABASE_URL="$owner_url" sqlx migrate run --source migrations
DATABASE_URL="$owner_url" SQLX_OFFLINE=false cargo sqlx prepare --workspace -- --all-targets
if [ -n "$(git -C "$ROOT" status --porcelain -- core/.sqlx)" ]; then
  echo "core/.sqlx is out of date; run cargo sqlx prepare and commit" >&2; git -C "$ROOT" status --short -- core/.sqlx >&2; exit 1
fi
echo "== core: tests with database"
ALLWIN_REQUIRE_DB_TESTS=1 TEST_DATABASE_ADMIN_URL="postgres://postgres:$su@127.0.0.1:$port/postgres" cargo test -q --workspace --locked
