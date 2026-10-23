#!/usr/bin/env bash
# Installs the pre-push guard into the shared hooks directory (applies to every worktree).
set -euo pipefail
common="$(git rev-parse --git-common-dir)"
install -m 755 "$(dirname "${BASH_SOURCE[0]}")/pre-push" "$common/hooks/pre-push"
echo "installed $common/hooks/pre-push"
