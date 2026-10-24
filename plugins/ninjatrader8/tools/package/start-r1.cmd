@echo off
rem All Win — round R1 (Pêndulo): runs the NT8-compatible scenarios in the order of the script, against the plugin.
rem Same port, token and transcript folder as start-fakecore.cmd. 06 runs last: it blocks the account, then unblocks it.
call "%~dp0start-fakecore.cmd" -scenario "%~dp0scenarios\01-entry-bracket.json" -scenario "%~dp0scenarios\03n-reject-nt8.json" -scenario "%~dp0scenarios\04-disconnect-reconnect.json" -scenario "%~dp0scenarios\05-expiry.json" -scenario "%~dp0scenarios\09-duplicates.json" -scenario "%~dp0scenarios\10-stale.json" -scenario "%~dp0scenarios\11-partial-exit.json" -scenario "%~dp0scenarios\12-reduce-position-changed.json" -scenario "%~dp0scenarios\06-flatten-block.json"
