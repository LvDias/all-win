# NinjaTrader 8 file exchange (manual test rounds)

NinjaTrader 8 tests are run **by the user, by hand, over RDP** (Sim101 / Playback only, never a live account). The VPS is not
reachable from the dev server. Files travel through the user's Mac and an existing SSH login to the dev server; no port is
opened and no token is stored on the server.

```
dev server --scp--> Mac --RDP drive--> VPS / NT8           (package, checksum, test script)
dev server <--scp-- Mac <--RDP drive-- VPS / NT8           (redacted logs)
```

## Folders (dev server, created by `setup-exchange.sh`)

`/srv/www/allwin.worktrees/.nt8-exchange/`

| Folder | Direction | Access (group `devs`) |
|---|---|---|
| `outbox/` | team to user: `AllWin-NT8-<version>-<sha7>.zip` (NinjaScript import: Tools > Import > NinjaScript Add-On), its `.sha256`, the numbered test script `.md` | read (2775) |
| `inbox/` | user to team: raw uploads | write and enter, **no list, no read** (2730) |
| `clean/` | scanned, redacted uploads | read (2775). **The only folder agents read.** |
| `quarantine/` | uploads that failed the scan | owner only (700). Nobody reads it; DevOps tells the user. |

Agents never read `inbox/` or `quarantine/`.

## Commands for the user (on the Mac)

The account on the dev server is `dev2` (the Mac key already logs in as it); `<HOST>` is the SSH address already used for the server.

```bash
# 1. fetch the package, its checksum and the test script
scp "dev2@<HOST>:/srv/www/allwin.worktrees/.nt8-exchange/outbox/AllWin-NT8-<version>-<sha7>.*" ~/Downloads/
cd ~/Downloads && shasum -a 256 -c AllWin-NT8-<version>-<sha7>.zip.sha256

# 2. copy the .zip into the RDP session (drive redirection), import it in NinjaTrader 8, follow the numbered script

# 3. back on the Mac: redact the NT8 logs (Documents\NinjaTrader 8\log and \trace, copied out of the RDP session)
#    redact-nt8-logs.sh and redact.pl are in this folder; copy both to the Mac once
./redact-nt8-logs.sh ~/Downloads/nt8-logs          # creates nt8-logs-<date>.zip (+ .sha256) here
# skim the zip (REDACTION-REPORT.txt lists what was replaced and which files were skipped), then:
scp nt8-logs-<date>.zip "dev2@<HOST>:/srv/www/allwin.worktrees/.nt8-exchange/inbox/"
```

Tell the team (through the Maestro) that an upload is in; DevOps runs `check-inbox.sh`.

## Redaction (two barriers, same rules)

`redact.pl` replaces with `<redacted:...>`: All Win keys (`awk_`/`awd_`), Tailscale keys, `Bearer`/`Basic` credentials,
`password`/`token`/`secret`/`api key`/`license`/`serial` assignments, URL credentials, e-mails, tailnet host names, IP addresses
(except `127.0.0.1`), long hex or base64-like strings, the Windows user name in `C:\Users\<name>`, private-key lines, and any
account name that is not `Sim<n>` or `Playback<n>`. Counts per rule are reported, never the text.

1. **On the Mac**, `redact-nt8-logs.sh` redacts text files (`txt log json csv xml md`, up to 50 MB each) and skips everything
   else (listed in the report, not sent).
2. **On the server**, `check-inbox.sh` re-scans each upload with `redact.pl --check`. A clean upload is extracted to `clean/`
   and the raw file is deleted. An upload with a secret pattern left, a non-text file, a symlink, an unsafe zip path
   (`..` or absolute), more than 2000 entries or more than 200 MB goes to `quarantine/` with a reason (rule names only).

The regular expressions are a safety net, not a guarantee: skim the zip before sending, and never put real credentials or a
live account in a test round. Self-test with fake secrets: `./test-redact.sh`.

## Files

`setup-exchange.sh` (folders), `check-inbox.sh` (server scan), `redact-nt8-logs.sh` and `redact.pl` (Mac and server),
`test-redact.sh` (self-test).
