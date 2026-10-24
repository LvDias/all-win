#!/usr/bin/env bash
# Runs on the dev server (DevOps). Scans uploads in inbox/ with the same redaction rules; a clean upload is extracted to
# clean/ (the only folder agents read), anything else goes to quarantine/ (owner only). Never prints file content.
#   infra/nt8-exchange/check-inbox.sh
set -euo pipefail
umask 027
here="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
base="${ALLWIN_NT8_EXCHANGE:-/srv/www/allwin.worktrees/.nt8-exchange}"
for d in inbox clean quarantine; do [ -d "$base/$d" ] || { echo "missing $base/$d (run setup-exchange.sh)" >&2; exit 1; }; done
max_bytes=$((200 * 1024 * 1024)); max_entries=2000
stamp="$(date +%Y%m%d-%H%M%S)"
shopt -s nullglob

quarantine() { # file reason
  local f="$1" why="$2" dest="$base/quarantine/$stamp-$(basename "$1")"
  mv -- "$f" "$dest"; chmod 600 -- "$dest"; printf '%s\n' "$why" > "$dest.reason"; chmod 600 -- "$dest.reason"
  echo "quarantine: $(basename "$f") ($why)"
}

scan_dir() { # dir -> 0 clean; prints reasons on failure
  local d="$1" f reasons=""
  if find "$d" -type l | grep -q .; then echo "symlink inside"; return 1; fi
  while IFS= read -r -d '' f; do
    case "$f" in
      *.txt|*.log|*.json|*.csv|*.xml|*.md) ;;
      *) echo "non-text file type: ${f##*.}"; return 1 ;;
    esac
    if ! out="$(perl "$here/redact.pl" --check "$f" 2>&1 >/dev/null)"; then reasons="$reasons ${out//$'\n'/;}"; fi
  done < <(find "$d" -type f -print0)
  [ -z "$reasons" ] || { echo "secret patterns left:$reasons"; return 1; }
}

found=0
for f in "$base"/inbox/*; do
  found=1
  name="$(basename "$f")"
  if [ -L "$f" ] || [ ! -f "$f" ]; then quarantine "$f" "not a regular file"; continue; fi
  [[ "$name" =~ ^[A-Za-z0-9._-]{1,120}$ ]] || { quarantine "$f" "unsafe file name"; continue; }
  size="$(stat -c %s "$f")"
  [ "$size" -le "$max_bytes" ] || { quarantine "$f" "over 200 MB"; continue; }
  work="$(mktemp -d "$base/.work.XXXXXX")"
  case "$name" in
    *.zip)
      entries="$(unzip -Z1 "$f" 2>/dev/null || true)"
      if [ -z "$entries" ] || [ "$(printf '%s\n' "$entries" | wc -l)" -gt "$max_entries" ] \
         || printf '%s\n' "$entries" | grep -qE '(^/|(^|/)\.\.(/|$))'; then
        rm -rf "$work"; quarantine "$f" "unreadable zip, too many entries or unsafe entry path"; continue
      fi
      unzip -qq -o -d "$work" "$f" >/dev/null 2>&1 || { rm -rf "$work"; quarantine "$f" "unzip failed"; continue; }
      ;;
    *) cp -- "$f" "$work/$name" ;;
  esac
  if reasons="$(scan_dir "$work")"; then
    dest="$base/clean/${name%.*}-$stamp"
    mv -- "$work" "$dest"; chmod -R g+rX,o-rwx "$dest"; chmod 2750 "$dest"
    rm -f -- "$f"
    echo "clean: $name -> clean/$(basename "$dest")"
  else
    rm -rf "$work"; quarantine "$f" "$reasons"
  fi
done
[ "$found" = 1 ] || echo "inbox empty"
