@echo off
rem All Win — fallback diagnostics collection. Writes AllWin-diag-collect-<UTC>.zip to the Desktop.
powershell -NoProfile -ExecutionPolicy Bypass -File "%~dp0collect-allwin.ps1"
pause
