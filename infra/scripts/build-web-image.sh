#!/usr/bin/env bash
# Builds the web image from a host-made Next.js standalone build. The install and the build go through allwin-heavy.
#   build-web-image.sh <repo worktree> <image tag>      e.g. allwin-web:sunday-dev
set -euo pipefail
here="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
wt="${1:-}"; tag="${2:-}"
[ -f "$wt/web/package.json" ] && [[ "$tag" =~ ^[a-z0-9][a-z0-9._/-]*:[A-Za-z0-9._-]+$ ]] || { echo "usage: $0 <repo worktree> <image:tag>" >&2; exit 2; }
dk() { if docker info >/dev/null 2>&1; then docker "$@"; else sg docker -c "$(printf '%q ' docker "$@")"; fi; }
wt="$(cd "$wt" && pwd -P)"
[ "$(git -C "$wt" rev-parse --show-toplevel)" = "$wt" ] || { echo "$wt is not the root of a git worktree" >&2; exit 2; }
if [ -n "$(git -C "$wt" status --porcelain)" ] && [ "${ALLWIN_ALLOW_DIRTY:-0}" != 1 ]; then
  echo "refusing: $wt has uncommitted changes, so the image would not match its HEAD (ALLWIN_ALLOW_DIRTY=1 to override)" >&2; exit 2
fi
cd "$wt/web"
# ALLWIN_SKIP_BUILD=1 reuses an existing .next/standalone (already built through allwin-heavy) and only assembles the image.
if [ "${ALLWIN_SKIP_BUILD:-0}" != 1 ]; then
  CI=true "$here/allwin-heavy" pnpm install --frozen-lockfile
  "$here/allwin-heavy" pnpm build
fi
[ -d .next/standalone ] || { echo "no .next/standalone: the build must use output: standalone" >&2; exit 1; }
stage="$(mktemp -d)"; trap 'rm -rf "$stage"' EXIT
cp -a .next/standalone "$stage/standalone"
# The server entry may sit under a sub-folder when the workspace root is above web/ (outputFileTracingRoot).
entry="$(cd "$stage/standalone" && find . -maxdepth 2 -name server.js -not -path '*/node_modules/*' | head -n 1)"
[ -n "$entry" ] || { echo "server.js not found in the standalone output" >&2; exit 1; }
echo "server entry: $entry"
appdir="$stage/standalone/$(dirname "$entry")"
mkdir -p "$appdir/.next"
cp -a .next/static "$appdir/.next/static"
if [ -d public ]; then cp -a public "$appdir/public"; else mkdir -p "$appdir/public"; fi
cp "$here/../docker/web.runtime.Dockerfile" "$stage/Dockerfile"
if [ "$entry" != "./server.js" ]; then
  sed -i "s|CMD \[\"node\", \"server.js\"\]|CMD [\"node\", \"${entry#./}\"]|" "$stage/Dockerfile"
  # the working directory stays /app; the entry path carries the sub-folder
fi
dk build -q -t "$tag" "$stage"
echo "image $tag built from $(git -C "$wt" rev-parse --short HEAD)"
