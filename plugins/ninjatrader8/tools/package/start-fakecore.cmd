@echo off
rem All Win — starts the local fake Core for the NT8 manual rounds (development only).
rem Listens on 127.0.0.1:8765 only. If Windows Firewall asks, click Cancel: nothing needs a firewall rule.
rem The test token is created once in %LOCALAPPDATA%\AllWin\fakecore-dev.token and is never printed (key id only).
setlocal
set "AW=%LOCALAPPDATA%\AllWin"
if not exist "%AW%\logs" mkdir "%AW%\logs"
for /f %%T in ('powershell -NoProfile -Command "(Get-Date).ToUniversalTime().ToString('yyyyMMddTHHmmssZ')"') do set "STAMP=%%T"
rem No arguments: the minimal round (scenarios\00-min-entrada.json). Otherwise the arguments are passed on.
set "ARGS=%*"
if "%~1"=="" set ARGS=-scenario "%~dp0scenarios\00-min-entrada.json"
"%~dp0AllWin.FakeCore.exe" -port 8765 -token-file "%AW%\fakecore-dev.token" -transcript "%AW%\logs\fakecore-%STAMP%.log" %ARGS%
echo.
echo Fake Core stopped. Transcript: %AW%\logs\fakecore-%STAMP%.log
pause
