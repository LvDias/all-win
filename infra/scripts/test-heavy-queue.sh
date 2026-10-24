#!/usr/bin/env bash
# Tests of the allwin-heavy queue: FIFO order, the sunday lane, dead tickets, the allowlist, --status and (when sudo is available) tickets made by
# another account. No root needed for the main tests. Run it as dev2: the lane file must be owned by dev2.   infra/scripts/test-heavy-queue.sh
set -uo pipefail
here="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"; H="$here/allwin-heavy"
T="$(mktemp -d /srv/www/allwin.worktrees/.test-heavy-queue.XXXXXX)"; chmod 2770 "$T"
trap 'pkill -P $$ 2>/dev/null; rm -rf "$T"' EXIT
export ALLWIN_HEAVY_LOCK_DIR="$T/lock" ALLWIN_HEAVY_SLOTS=1 ALLWIN_HEAVY_NO_SCCACHE=1 ALLWIN_HEAVY_TARGET_ROOT="$T/tr" ALLWIN_HEAVY_WAIT_S=120
[ "$(id -un)" = dev2 ] || echo "note: the lane tests expect to run as dev2 (the trusted owner of LANE.sunday); they will report failures otherwise"
fail=0; check() { if [ "$2" = "$3" ]; then echo "ok   : $1"; else echo "FAIL : $1 (got '$3', wanted '$2')"; fail=1; fi; }
reset() { rm -rf "$T/lock"; : > "$T/order"; }
hold() { "$H" sleep "$1" >/dev/null 2>&1 & sleep 1.5; }          # takes the only slot for N seconds
job()  { ( cd "${3:-$T}" && "$H" ${2:+--lane "$2"} bash -c "echo $1 >> $T/order" >/dev/null 2>&1 ) & sleep 1; }

echo "== FIFO among normal jobs"
reset; hold 8; job A; job B; job C; wait; check "A, B, C run in arrival order" "A B C" "$(tr '\n' ' ' < $T/order | sed 's/ $//')"

echo "== sunday lane goes first (allowed worktree)"
reset; mkdir -p "$T/lock" "$T/wt-sunday"; printf '%s\n' "$T/wt-sunday" > "$T/lock/LANE.sunday"; chmod 644 "$T/lock/LANE.sunday"
hold 8; job N1; job S sunday "$T/wt-sunday"; job N2; wait
check "S before N1 and N2, then N1, N2" "S N1 N2" "$(tr '\n' ' ' < $T/order | sed 's/ $//')"

echo "== lane sunday asked from a worktree that is NOT listed"
reset; mkdir -p "$T/lock" "$T/wt-other"; printf '%s\n' "$T/wt-sunday" > "$T/lock/LANE.sunday"; chmod 644 "$T/lock/LANE.sunday"
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
reset; mkdir -p "$T/lock" "$T/wt-sunday"; printf '%s\n' "$T/wt-sunday" > "$T/lock/LANE.sunday"; chmod 644 "$T/lock/LANE.sunday"
hold 6; job W1; job W2 sunday "$T/wt-sunday"
st="$("$H" --status 2>&1)"; wait
case "$st" in *RUNNING*) r=ok ;; *) r="$st" ;; esac; check "--status lists the running job" ok "$r"
case "$st" in *"queue (service order)"*"lane sunday"*"lane normal"*) r=ok ;; *) r="$st" ;; esac; check "--status lists the queue with lanes, sunday first" ok "$r"

echo "== M1: the sunday lane cannot be claimed by a look-alike directory, a wildcard or '..'"
reset; mkdir -p "$T/lock" "$T/elsewhere/wt-lookalike" "$T/wt-real"
printf '%s\n' '*/wt-lookalike' '../../etc' "$T/wt-*" > "$T/lock/LANE.sunday"; chmod 644 "$T/lock/LANE.sunday"
out="$( cd "$T/elsewhere/wt-lookalike" && "$H" --lane sunday echo hi 2>&1 )"
case "$out" in *"not allowed"*) r=refused ;; *) r="$out" ;; esac; check "'*/name' no longer matches a directory with that name" refused "$r"
out="$( cd "$T/wt-real" && "$H" --lane sunday echo hi 2>&1 )"
case "$out" in *"not allowed"*) r=refused ;; *) r="$out" ;; esac; check "a wildcard entry matches nothing" refused "$r"
printf '%s\n' "$T/wt-real" > "$T/lock/LANE.sunday"; chmod 644 "$T/lock/LANE.sunday"
out="$( cd "$T/wt-real" && "$H" --lane sunday echo hi 2>&1 )"
case "$out" in *"not allowed"*) r=refused ;; *) r=allowed ;; esac; check "an absolute prefix is accepted" allowed "$r"

echo "== the lane file is trusted only when dev2 owns it with no group/other write; the entry '/' matches nothing"
reset; mkdir -p "$T/lock" "$T/wt-real"; printf '%s\n' "$T/wt-real" > "$T/lock/LANE.sunday"; chmod 664 "$T/lock/LANE.sunday"
out="$( cd "$T/wt-real" && "$H" --lane sunday echo hi 2>&1 )"
case "$out" in *"ignoring LANE.sunday"*"not allowed"*) r=ignored ;; *) r="$out" ;; esac; check "a group-writable LANE.sunday is ignored with a warning" ignored "$r"
chmod 644 "$T/lock/LANE.sunday"; printf '%s\n' '/' > "$T/lock/LANE.sunday"
out="$( cd "$T/wt-real" && "$H" --lane sunday echo hi 2>&1 )"
case "$out" in *"not allowed"*) r=refused ;; *) r="$out" ;; esac; check "the entry '/' matches nothing" refused "$r"
if sudo -n -u dev3 true 2>/dev/null; then
  rm -f "$T/lock/LANE.sunday"; chmod 3770 "$T/lock"
  sudo -n -u dev3 sh -c "printf '%s\n' '$T/wt-real' > '$T/lock/LANE.sunday'; chmod 644 '$T/lock/LANE.sunday'" 2>/dev/null
  out="$( cd "$T/wt-real" && ALLWIN_HEAVY_LANE_OWNER=dev3 "$H" --lane sunday echo hi 2>&1 )"   # the old override variable must change nothing
  case "$out" in *"ignoring LANE.sunday"*"owner dev3"*) r=ignored ;; *) r="$out" ;; esac; check "a LANE.sunday made by another account (dev3) is ignored, even with ALLWIN_HEAVY_LANE_OWNER=dev3" ignored "$r"
fi

echo "== M1: another account cannot edit, replace or delete LANE.sunday (mode 644 + sticky directory)"
if sudo -n -u dev3 true 2>/dev/null; then
  reset; mkdir -p "$T/lock"; printf '%s\n' "$T/wt-real" > "$T/lock/LANE.sunday"
  chmod 644 "$T/lock/LANE.sunday"; chmod 3770 "$T/lock"
  sudo -n -u dev3 sh -c "echo '$T/evil' >> '$T/lock/LANE.sunday'" 2>/dev/null; check "dev3 cannot append to it" "$T/wt-real" "$(grep -v '^#' "$T/lock/LANE.sunday" | tr '\n' ' ' | sed 's/ $//')"
  sudo -n -u dev3 rm -f "$T/lock/LANE.sunday" 2>/dev/null; check "dev3 cannot delete it" yes "$([ -e "$T/lock/LANE.sunday" ] && echo yes || echo no)"
  sudo -n -u dev3 sh -c "echo x > '$T/lock/other' && mv -f '$T/lock/other' '$T/lock/LANE.sunday'" 2>/dev/null; check "dev3 cannot replace it by renaming" "$T/wt-real" "$(grep -v '^#' "$T/lock/LANE.sunday" | tr '\n' ' ' | sed 's/ $//')"
else echo "skip : no sudo for dev3"; fi

echo "== L3: exit 75 means starved (no progress), and a better position restarts the clock"
reset; hold 20
( ALLWIN_HEAVY_WAIT_S=6 "$H" echo starved >/dev/null 2>&1; echo $? > "$T/rc75" ) & sleep 9
check "a lone waiter gives up with 75 after the limit" 75 "$(cat "$T/rc75" 2>/dev/null)"
kill %1 2>/dev/null; wait 2>/dev/null
reset; hold 6
( cd "$T" && "$H" bash -c "sleep 5; echo A >> $T/order" >/dev/null 2>&1 ) & sleep 1
( ALLWIN_HEAVY_WAIT_S=8 "$H" bash -c "echo B >> $T/order" >/dev/null 2>&1; echo $? > "$T/rcB" ) & wait
check "B waited 11 s with an 8 s limit but its position improved: not starved" "0" "$(cat "$T/rcB" 2>/dev/null)"
check "B ran after A" "A B" "$(tr '\n' ' ' < $T/order | sed 's/ $//')"

echo "== L3: a ticket under a PAUSE keeps its place and the job runs when the PAUSE ends"
reset; mkdir -p "$T/lock"; printf 'W3 test — until 23:59\n' > "$T/lock/PAUSE"
( cd "$T" && "$H" bash -c "echo PAUSED_JOB >> $T/order" >/dev/null 2>&1 ) & sleep 3
check "its ticket exists while the PAUSE holds it" 1 "$(ls "$T/lock/queue" 2>/dev/null | wc -l)"
check "it has not run" "" "$(cat $T/order)"
rm -f "$T/lock/PAUSE"; wait
check "it ran after the PAUSE was removed" "PAUSED_JOB" "$(tr '\n' ' ' < $T/order | sed 's/ $//')"

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
