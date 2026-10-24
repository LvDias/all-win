#!/usr/bin/env bash
# Creates the exchange folders on the dev server. Idempotent; no sudo; no network.
#   infra/nt8-exchange/setup-exchange.sh
# outbox/  team -> user: packages, checksums, test scripts      (group devs: read, setgid 2775)
# inbox/   user -> team: raw uploads                            (group devs: write+enter, NOT list/read, 2730)
# clean/   redacted, checked uploads; the only folder agents read (group devs: read, 2775)
# quarantine/  uploads that failed the scan                     (owner only, 700)
set -euo pipefail
base="${ALLWIN_NT8_EXCHANGE:-/srv/www/allwin.worktrees/.nt8-exchange}"
group="${ALLWIN_NT8_GROUP:-devs}"
mk() { # dir mode
  mkdir -p -- "$1"
  chgrp "$group" -- "$1" 2>/dev/null || echo "warning: could not chgrp $1 to $group" >&2
  chmod "$2" -- "$1"
}
mk "$base" 2775
mk "$base/outbox" 2775
mk "$base/inbox" 2730
mk "$base/clean" 2775
mkdir -p -- "$base/quarantine"; chmod 700 -- "$base/quarantine"
echo "ready: $base"
