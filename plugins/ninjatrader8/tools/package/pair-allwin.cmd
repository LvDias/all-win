@echo off
rem All Win — pairs this machine with the real Core (development build, M1 round). The code is asked by a prompt.
powershell -NoProfile -ExecutionPolicy Bypass -File "%~dp0pair-allwin.ps1"
pause
