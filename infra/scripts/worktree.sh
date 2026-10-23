#!/usr/bin/env bash
# Slice worktrees per ADR-0002 §5.
#   worktree.sh plan   <slice-branch> <owner>...   print what `add` would do
#   worktree.sh add    <slice-branch> <owner> [base]
#   worktree.sh remove <slice-branch> <owner>
# <slice-branch> is `s/<slice-id>-<short-name>` (e.g. s/v1-s1-webhook-ingest); <owner> is a
# short suffix (core, web, plugin, contracts, e2e, infra). The owner branch is
# `<slice-branch>--<owner>`, created from the slice branch, which is created from [base] (main).
set -euo pipefail
umask 002   # worktrees are shared by group devs

REPO=/srv/www/allwin
WT_ROOT=/srv/www/allwin.worktrees
git() { command git -C "$REPO" "$@"; }

usage() { sed -n '2,8p' "$0" >&2; exit 2; }
check_slice() { [[ $1 =~ ^s/[a-z0-9]+(-[a-z0-9]+)+$ ]] || { echo "bad slice branch: $1" >&2; exit 2; }; }
check_owner() { [[ $1 =~ ^[a-z0-9]+(-[a-z0-9]+)*$ ]] || { echo "bad owner: $1" >&2; exit 2; }; }
wt_path() { echo "$WT_ROOT/${1//\//-}"; }
has_branch() { git show-ref --verify --quiet "refs/heads/$1"; }

cmd=${1:-}; shift || true
case $cmd in
  plan)
    [[ $# -ge 2 ]] || usage; slice=$1; shift; check_slice "$slice"
    for owner in "$@"; do
      check_owner "$owner"; b="$slice--$owner"
      printf '%-40s %s\n' "$b" "$(wt_path "$b")"
    done ;;
  add)
    [[ $# -ge 2 ]] || usage; slice=$1 owner=$2 base=${3:-main}
    check_slice "$slice"; check_owner "$owner"; b="$slice--$owner"; p=$(wt_path "$b")
    [[ -e $p ]] && { echo "exists: $p" >&2; exit 1; }
    has_branch "$slice" || git branch "$slice" "$base"
    if has_branch "$b"; then git worktree add "$p" "$b"; else git worktree add -b "$b" "$p" "$slice"; fi
    echo "branch=$b path=$p base=$(git rev-parse --short "$slice")"
    # git 2.43 ignores "/srv/www/*"; other users need an explicit entry for this path.
    git config --system --get-all safe.directory | grep -qxF "$p" \
      || echo "other users need (root): git config --system --add safe.directory $p" ;;
  remove)
    [[ $# -eq 2 ]] || usage; slice=$1 owner=$2
    check_slice "$slice"; check_owner "$owner"; b="$slice--$owner"; p=$(wt_path "$b")
    # Refuses a dirty worktree; never deletes an unmerged branch.
    git worktree remove "$p"
    if git merge-base --is-ancestor "$b" "$slice"; then git branch -d "$b"
    else echo "kept branch $b: not merged into $slice" >&2; fi ;;
  *) usage ;;
esac
