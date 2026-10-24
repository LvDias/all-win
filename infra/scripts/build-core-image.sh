#!/usr/bin/env bash
# Builds the Core image from a host-compiled release binary. The compile goes through allwin-heavy.
#   build-core-image.sh <repo worktree> <image tag> [cargo build arguments after --]
#   build-core-image.sh /srv/www/allwin.worktrees/<wt> allwin-core:sunday-dev
#   build-core-image.sh /srv/www/allwin.worktrees/<wt> allwin-core:sunday-dev-stubs -- --features <feature>
# Another binary of the workspace: ALLWIN_BUILD_PACKAGE=adapter-sim ALLWIN_BUILD_BIN=adapter-sim ALLWIN_RUNTIME_DOCKERFILE=sim.runtime.Dockerfile ... allwin-sim:sunday-dev
# An image built with a test feature (dev-stubs, ADR-0006 rule 8) MUST get a distinct tag and never goes to staging or a release.
set -euo pipefail
here="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
wt="${1:-}"; tag="${2:-}"; shift 2 || true
[ -d "$wt/core" ] && [[ "$tag" =~ ^[a-z0-9][a-z0-9._/-]*:[A-Za-z0-9._-]+$ ]] || { echo "usage: $0 <repo worktree> <image:tag> [-- <cargo args>]" >&2; exit 2; }
[ "${1:-}" = "--" ] && shift
pkg="${ALLWIN_BUILD_PACKAGE:-app}"; bin="${ALLWIN_BUILD_BIN:-allwin-core}"; dockerfile="${ALLWIN_RUNTIME_DOCKERFILE:-core.runtime.Dockerfile}"
[[ "$pkg" =~ ^[a-z0-9_-]+$ && "$bin" =~ ^[a-z0-9_-]+$ && "$dockerfile" =~ ^[a-z.]+\.Dockerfile$ ]] || { echo "invalid package, binary or Dockerfile name" >&2; exit 2; }
stage="$(mktemp -d)"; trap 'rm -rf "$stage"' EXIT
dk() { if docker info >/dev/null 2>&1; then docker "$@"; else sg docker -c "$(printf '%q ' docker "$@")"; fi; }

SQLX_OFFLINE=true "$here/allwin-heavy" bash -c '
  set -euo pipefail
  cd "$1/core"; stage="$2"; pkg="$3"; bin="$4"; shift 4
  cargo build --release --locked -p "$pkg" "$@"
  install -m 755 "$CARGO_TARGET_DIR/release/$bin" "$stage/$bin"
' _ "$wt" "$stage" "$pkg" "$bin" "$@"
cp "$here/../docker/$dockerfile" "$stage/Dockerfile"
echo "binary sha256: $(sha256sum "$stage/$bin" | cut -d' ' -f1)"
echo "marker 'allwin-dev-stub:routing-entry-gate' occurrences in the binary: $(grep -c -a -- 'allwin-dev-stub:routing-entry-gate' "$stage/$bin" || true)"
dk build -q -t "$tag" "$stage"
echo "image $tag built from $(git -C "$wt" rev-parse --short HEAD)"
