#!/usr/bin/env bash
# Runs on the user's Mac (bash 3.2, perl and zip ship with macOS). Redacts NinjaTrader/All Win logs and zips them for upload.
#   ./redact-nt8-logs.sh <log folder or files...>     (writes nt8-logs-<date>.zip + .sha256 in the current folder)
# Needs redact.pl next to this script. Only text files are included; anything else is listed as skipped, not sent.
set -euo pipefail
here="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
[ $# -gt 0 ] || { echo "usage: $0 <log folder or files...>" >&2; exit 2; }
[ -f "$here/redact.pl" ] || { echo "redact.pl must be next to this script" >&2; exit 1; }

stamp="$(date +%Y%m%d-%H%M%S)"
work="$(mktemp -d)"; trap 'rm -rf "$work"' EXIT
mkdir "$work/nt8-logs-$stamp"
out="$work/nt8-logs-$stamp"
: > "$work/skipped.txt"; : > "$work/counts.txt"
n=0

while IFS= read -r -d '' f; do
  base_name="$(basename "$f" | tr 'A-Z' 'a-z')"
  case "$base_name" in
    *token*|*consent*|*secret*|*password*|*credential*) echo "$f (secret-like name, never sent)" >> "$work/skipped.txt"; continue ;;
  esac
  case "$f" in
    *.txt|*.log|*.json|*.jsonl|*.ndjson|*.csv|*.xml|*.md) ;;
    *.snap) ;;   # plugin state snapshot: kept only if it is UTF-8 text (checked below)
    *) echo "$f" >> "$work/skipped.txt"; continue ;;
  esac
  if ! perl -e 'local $/; open(my $h, "<:raw", $ARGV[0]) or exit 1; my $s = <$h>; exit(($s =~ /\0/ || !utf8::decode($s)) ? 1 : 0)' "$f"; then
    echo "$f (not UTF-8 text; convert it to UTF-8 first)" >> "$work/skipped.txt"; continue
  fi
  size="$(wc -c < "$f" | tr -d ' ')"
  if [ "$size" -gt 52428800 ]; then echo "$f (over 50 MB)" >> "$work/skipped.txt"; continue; fi
  n=$((n + 1))
  flat="$(printf '%s' "$f" | tr '/\\ ' '___' | sed 's/^_*//')"
  perl "$here/redact.pl" "$f" > "$out/${n}_$(basename "$flat")" 2>> "$work/counts.txt"
done < <(find "$@" -type f -print0)

[ "$n" -gt 0 ] || { echo "no text log found in: $*" >&2; exit 1; }
{ echo "Redaction counts (rule x times, no content):"; sort "$work/counts.txt" | uniq -c; echo; echo "Skipped (not sent):"; cat "$work/skipped.txt"; } > "$out/REDACTION-REPORT.txt"

( cd "$work" && zip -qr "nt8-logs-$stamp.zip" "nt8-logs-$stamp" )
cp "$work/nt8-logs-$stamp.zip" "./nt8-logs-$stamp.zip"
shasum -a 256 "nt8-logs-$stamp.zip" > "nt8-logs-$stamp.zip.sha256"
echo "Created nt8-logs-$stamp.zip ($n files). Skipped files and redaction counts: see REDACTION-REPORT.txt inside."
echo "Open the zip and skim it before sending; the server scans it again."
