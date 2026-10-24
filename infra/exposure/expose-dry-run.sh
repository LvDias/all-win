#!/usr/bin/env bash
# Prints the firewall rules expose-open.sh would apply for a source IP. Changes NOTHING: no rule, no container, no marker.
#   expose-dry-run.sh <VPS_PUBLIC_IP>
set -euo pipefail
here="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
[ "$#" -eq 1 ] || { echo "usage: $0 <VPS_PUBLIC_IP>" >&2; exit 2; }
unset ALLWIN_FIREWALL_TEST_MODE
exec "$here/firewall-open.sh" "$1" --dry-run
