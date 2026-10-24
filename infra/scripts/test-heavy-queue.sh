#!/usr/bin/env bash
# Tests of the allwin-heavy queue: FIFO order, the sunday lane, dead tickets, the allowlist, --status and (when sudo is available) tickets made by
# another account. No root needed for the main tests.   infra/scripts/test-heavy-queue.sh
set -uo pipefail
here="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"; H="$here/allwin-heavy"
T="$(mktemp -d /srv/www/allwin.worktrees/.test-heavy-queue.XXXXXX)"; chmod 2770 "$T"
trap 'pkill -P $$ 2>/dev/null; rm -rf "$T"' EXIT
export ALLWIN_HEAVY_LOCK_DIR="$T/lock" ALLWIN_HEAVY_SLOTS=1 ALLWIN_HEAVY_NO_SCCACHE=1 ALLWIN_HEAVY_TARGET_ROOT="$T/tr" ALLWIN_HEAVY_WAIT_S=120
fail=0; check() { if [ "$2" = "$3" ]; then echo "ok   : $1"; else echo "FAIL : $1 (got '$3', wanted '$2')"; fail=1; fi; }
reset() { rm -rf "$T/lock"; : > "$T/order"; }
hold() { "$H" sleep "$1" >/dev/null 2>&1 & sleep 1.5; }          # takes the only slot for N seconds
job()  { ( cd "${3:-$T}" && "$H" ${2:+--lane "$2"} bash -c "echo $1 >> $T/order" >/dev/null 2>&1 ) & sleep 1; }

echo "== FIFO among normal jobs"
reset; hold 8; job A; job B; job C; wait; check "A, B, C run in arrival order" "A B C" "$(tr '\n' ' ' < $T/order | sed 's/ $//')"

echo "== sunday lane goes first (allowed worktree)"
reset; mkdir -p "$T/lock" "$T/wt-sunday"; printf '%s\n' "$T/wt-sunday" > "$T/lock/LANE.sunday"
hold 8; job N1; job S sunday "$T/wt-sunday"; job N2; wait
check "S before N1 and N2, then N1, N2" "S N1 N2" "$(tr '\n' ' ' < $T/order | sed 's/ $//')"

echo "== lane sunday asked from a worktree that is NOT listed"
reset; mkdir -p "$T/lock" "$T/wt-other"; printf '%s\n' "$T/wt-sunday" > "$T/lock/LANE.sunday"
out="$( cd "$T/wt-other" && "$H" --lane sunday echo hi 2>&1 )"
case "$out" in *"not allowed"*"normal lane"*) r=notice ;; *) r="$out" ;; esac; check "notice printed and the job still runs" notice "$r"
case "$out" in *hi*) r=ran ;; *) r=no ;; esac; check "the command ran" ran "$r"
reset; out="$( "$H" --lane sunday echo hi 2>&1 )"; case "$out" in *"not allowed"*) r=notice ;; *) r="$out" ;; esac; check "no LANE.sunday file: nobody is promoted" notice "$r"

echo "== a ticket whose process was killed with -9 is skipped"
reset; hold 8
( cd "$T" && "$H" bash -c "echo DEAD >> $T/order" >/dev/null 2>&1 ) & sleep 1
tk="$(grep -l 'echo DEAD' "$T"/lock/queue/* 2>/dev/null | head -n 1)"; dead="${tk##*.}"   # the wrapper's own pid is the last part of the ticket name
job LIVE; kill -9 "$dead" 2>/dev/null; wait
check "only LIVE ran" "LIVE" "$(tr '\n' ' ' < $T/order | sed 's/ $//')"
check "no ticket is left behind" "0" "$(ls "$T/lock/queue" 2>/dev/null | wc -l)"

echo "== --status"
reset; mkdir -p "$T/lock" "$T/wt-sunday"; printf '%s\n' "$T/wt-sunday" > "$T/lock/LANE.sunday"
hold 6; job W1; job W2 sunday "$T/wt-sunday"
st="$("$H" --status 2>&1)"; wait
case "$st" in *RUNNING*) r=ok ;; *) r="$st" ;; esac; check "--status lists the running job" ok "$r"
case "$st" in *"queue (service order)"*"lane sunday"*"lane normal"*) r=ok ;; *) r="$st" ;; esac; check "--status lists the queue with lanes, sunday first" ok "$r"

echo "== pnpm install --offline --frozen-lockfile is not queued"
reset; hold 6; mkdir -p "$T/bin"; printf '#!/bin/sh\necho installed\n' > "$T/bin/pnpm"; chmod +x "$T/bin/pnpm"
out="$( PATH="$T/bin:$PATH" timeout 4 "$H" pnpm install --offline --frozen-lockfile 2>&1 )"; wait
case "$out" in *installed*) r=ran-at-once ;; *) r="$out" ;; esac; check "runs while the only slot is busy" ran-at-once "$r"

echo "== a ticket made by another account (dev3) is ordered like any other"
if sudo -n -u dev3 true 2>/dev/null; then
  reset; mkdir -p "$T/lock"; chmod 2770 "$T/lock"; hold 8
  ( sudo -n -u dev3 env ALLWIN_HEAVY_LOCK_DIR="$T/lock" ALLWIN_HEAVY_SLOTS=1 ALLWIN_HEAVY_NO_SCCACHE=1 ALLWIN_HEAVY_TARGET_ROOT="$T/tr3" \
      "$H" bash -c "echo DEV3 >> $T/order" >/dev/null 2>&1 ) & sleep 2
  job DEV2; wait
  check "dev3's earlier ticket runs first, then dev2's" "DEV3 DEV2" "$(tr '\n' ' ' < $T/order | sed 's/ $//')"
else echo "skip : no sudo for dev3"; fi
[ "$fail" = 0 ] && echo "queue tests OK" || { echo "queue tests FAILED"; exit 1; }
