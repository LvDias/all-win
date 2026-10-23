#!/usr/bin/env bash
# Dev stack helper: up | down | logs | ps | config. Loopback-only; simulation/dev only.
set -euo pipefail
here="$(cd "$(dirname "${BASH_SOURCE[0]}")/../compose" && pwd)"
[ -f "$here/.env" ] || { echo "missing $here/.env (copy .env.example)" >&2; exit 1; }
dc() { docker compose -f "$here/docker-compose.dev.yml" --env-file "$here/.env" "$@"; }
case "${1:-}" in
  up)     shift; dc up -d "$@" ;;          # add --profile core once the Core image builds
  down)   dc down ;;
  logs)   shift; dc logs --no-color "$@" ;;
  ps)     dc ps ;;
  config) dc config -q && echo OK ;;
  *) echo "usage: $0 up|down|logs|ps|config" >&2; exit 2 ;;
esac
