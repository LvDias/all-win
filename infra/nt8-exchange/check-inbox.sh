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
max_unzipped=${ALLWIN_NT8_MAX_UNZIPPED:-$((500 * 1024 * 1024))}; max_ratio=200   # zip bomb limits (uncompressed total, ratio)
stamp="$(date +%Y%m%d-%H%M%S)"
shopt -s nullglob

# Private work root (owner only): nothing another account can rename or replace is ever extracted into or moved from.
workroot="$base/.work"
[ ! -L "$base" ] || { echo "$base is a symlink" >&2; exit 1; }
[ -d "$workroot" ] || mkdir -m 700 -- "$workroot"
[ -d "$workroot" ] && [ ! -L "$workroot" ] && [ "$(stat -c %u "$workroot")" = "$(id -u)" ] && case "$(stat -c %a "$workroot")" in 700|2700) true ;; *) false ;; esac \
  || { echo "$workroot must be a real directory owned by $(id -un), mode 700" >&2; exit 1; }

# Text file: no NUL byte and valid UTF-8 (a binary with a .log name is refused).
is_utf8_text() { perl -e 'local $/; open(my $h, "<:raw", $ARGV[0]) or exit 1; my $s = <$h>; exit(($s =~ /\0/ || !utf8::decode($s)) ? 1 : 0)' "$1"; }

quarantine() { # file reason
  local f="$1" why="$2" dest="$base/quarantine/$stamp-$(basename "$1")"
  mv -- "$f" "$dest"; chmod 600 -- "$dest"; printf '%s\n' "$why" > "$dest.reason"; chmod 600 -- "$dest.reason"
  echo "quarantine: $(basename "$f") ($why)"
}

scan_dir() { # dir -> 0 clean; prints reasons on failure
  local d="$1" f reasons=""
  if find "$d" -type l | grep -q .; then echo "symlink inside"; return 1; fi
  while IFS= read -r -d '' f; do
    case "$(basename "$f" | tr 'A-Z' 'a-z')" in
      *token*|*consent*|*secret*|*password*|*credential*) echo "secret-like file name"; return 1 ;;
    esac
    case "$f" in
      *.txt|*.log|*.json|*.jsonl|*.ndjson|*.csv|*.xml|*.md) ;;
      *.snap) ;;
      *) echo "non-text file type: ${f##*.}"; return 1 ;;
    esac
    is_utf8_text "$f" || { echo "binary content or invalid UTF-8"; return 1; }
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
  work="$(mktemp -d "$workroot/u.XXXXXX")"
  case "$name" in
    *.zip)
      entries="$(unzip -Z1 "$f" 2>/dev/null || true)"
      if [ -z "$entries" ] || [ "$(printf '%s\n' "$entries" | wc -l)" -gt "$max_entries" ] \
         || printf '%s\n' "$entries" | grep -qE '(^/|(^|/)\.\.(/|$))'; then
        rm -rf "$work"; quarantine "$f" "unreadable zip, too many entries or unsafe entry path"; continue
      fi
      if unzip -Z "$f" 2>/dev/null | grep -qE '^l'; then rm -rf "$work"; quarantine "$f" "symlink entry in zip"; continue; fi
      total="$(unzip -Zt "$f" 2>/dev/null | sed -n 's/.* \([0-9]*\) bytes uncompressed, \([0-9]*\) bytes compressed.*/\1 \2/p' | head -n 1)"
      un="${total%% *}"; co="${total##* }"
      if [ -z "$total" ] || [ "$un" -gt "$max_unzipped" ] || [ "$un" -gt $(( (co + 1) * max_ratio )) ]; then
        rm -rf "$work"; quarantine "$f" "zip too large when uncompressed or too high a compression ratio"; continue
      fi
      # Extract counting the bytes really written (not the sizes the zip declares); stop at the total cap.
      python3 -I - "$f" "$work" "$max_unzipped" "$max_entries" <<'PY' || { rm -rf "$work"; quarantine "$f" "extraction failed or over the total size cap"; continue; }
import os, sys, zipfile
src, dest, cap, max_entries = sys.argv[1], sys.argv[2], int(sys.argv[3]), int(sys.argv[4])
total = 0
with zipfile.ZipFile(src) as z:
    infos = z.infolist()
    if len(infos) > max_entries:
        sys.exit(1)
    for zi in infos:
        name = zi.filename
        if name.startswith("/") or ".." in name.split("/"):
            sys.exit(1)
        target = os.path.realpath(os.path.join(dest, name))
        if not target.startswith(os.path.realpath(dest) + os.sep):
            sys.exit(1)
        if name.endswith("/"):
            os.makedirs(target, mode=0o700, exist_ok=True)
            continue
        os.makedirs(os.path.dirname(target), mode=0o700, exist_ok=True)
        with z.open(zi) as r, open(target, "xb") as w:
            while True:
                chunk = r.read(1 << 20)
                if not chunk:
                    break
                total += len(chunk)
                if total > cap:
                    sys.exit(1)
                w.write(chunk)
PY
      ;;
    *) cp -- "$f" "$work/$name" ;;
  esac
  [ -d "$work" ] && [ ! -L "$work" ] || { quarantine "$f" "work directory changed"; continue; }
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
