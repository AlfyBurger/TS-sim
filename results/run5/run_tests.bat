@echo off
cd /d "%~dp0"
title TS simulator worker - run 5
echo Twilight Struggle run 5. Starts at 23:00 and runs until the job list is finished.
echo Closing this window pauses the run; double-click again to resume where it stopped.
powershell -NoProfile -ExecutionPolicy Bypass -File "%~dp0wait_until_23.ps1"
echo Started %date% %time%
tssim.exe worker jobs.txt out
echo Finished %date% %time%
pause
