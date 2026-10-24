#!/usr/bin/env bash
# Creates the exchange folders on the dev server. Idempotent; no sudo; no network.
#   infra/nt8-exchange/setup-exchange.sh
# base, outbox/, clean/ are 2755: owned by the publishing and scanning account (dev2), group devs READ-ONLY, so no other account
# can replace a package in outbox/ or drop a file into clean/. Existing folders are corrected too.
# outbox/  team -> user: packages, test scripts                 (group devs: read, 2755)
# inbox/   user -> team: raw uploads                            (group devs: write+enter, NOT list/read, 2730)
# clean/   redacted, checked uploads; the only folder agents read (group devs: read, 2755)
# quarantine/  uploads that failed the scan                     (owner only, 700)
set -euo pipefail
base="${ALLWIN_NT8_EXCHANGE:-/srv/www/allwin.worktrees/.nt8-exchange}"
group="${ALLWIN_NT8_GROUP:-devs}"
mk() { # dir mode
  mkdir -p -- "$1"
  chgrp "$group" -- "$1" 2>/dev/null || echo "warning: could not chgrp $1 to $group" >&2
  chmod "$2" -- "$1"
}
mk "$base" 2755
mk "$base/outbox" 2755
mk "$base/inbox" 2730
mk "$base/clean" 2755
mkdir -p -- "$base/quarantine" "$base/.work"; chmod 700 -- "$base/quarantine" "$base/.work"
echo "ready: $base"
