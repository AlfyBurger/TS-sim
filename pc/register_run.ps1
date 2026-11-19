# Register (and optionally start) a TS run and its watchdog in Task Scheduler, both at Normal priority.
#   powershell -ExecutionPolicy Bypass -File register_run.ps1 -Run 7 -At now        # start in 1 minute
#   powershell -ExecutionPolicy Bypass -File register_run.ps1 -Run 7 -At "23:00"    # start at 23:00 (today, or tomorrow if past)
# Needs in this folder: tssim<Run>.exe, run<Run>_jobs.txt, start_run.bat, run_worker.ps1, watchdog_run.ps1.
param([Parameter(Mandatory = $true)][string]$Run, [string]$At = "now", [int]$WatchdogDays = 3)
$d = $PSScriptRoot
foreach ($f in "tssim$Run.exe", "run${Run}_jobs.txt", "start_run.bat", "run_worker.ps1", "watchdog_run.ps1") {
  if (-not (Test-Path (Join-Path $d $f))) { throw "missing $f" } }
foreach ($f in "RUN${Run}_COMPLETE.txt", "watchdog$Run.txt", "PAUSE", "STOP") {
  if (Test-Path (Join-Path $d $f)) { throw "$f exists: rename it first (a finished run, an old watchdog log, or a pause)" } }
if ($At -eq "now") { $start = (Get-Date).AddMinutes(1) } else { $start = [datetime]::Today.Add([TimeSpan]::Parse($At)); if ($start -lt (Get-Date)) { $start = $start.AddDays(1) } }
$pr = New-ScheduledTaskPrincipal -UserId "$env:USERDOMAIN\$env:USERNAME" -LogonType Interactive
$s = New-ScheduledTaskSettingsSet -ExecutionTimeLimit ([TimeSpan]::Zero) -StartWhenAvailable -WakeToRun; $s.Priority = 4
Register-ScheduledTask -TaskName "TS run $Run" -Principal $pr -Settings $s -Force `
  -Action (New-ScheduledTaskAction -Execute "$d\start_run.bat" -Argument $Run -WorkingDirectory $d) `
  -Trigger (New-ScheduledTaskTrigger -Once -At $start) | Out-Null
$ws = New-ScheduledTaskSettingsSet -ExecutionTimeLimit (New-TimeSpan -Minutes 5) -StartWhenAvailable -MultipleInstances IgnoreNew; $ws.Priority = 4
$wt = New-ScheduledTaskTrigger -Once -At $start.AddMinutes(10) -RepetitionInterval (New-TimeSpan -Minutes 10) -RepetitionDuration (New-TimeSpan -Days $WatchdogDays)
Register-ScheduledTask -TaskName "TS run $Run watchdog" -Principal $pr -Settings $ws -Force `
  -Action (New-ScheduledTaskAction -Execute "powershell.exe" -Argument "-WindowStyle Hidden -NoProfile -ExecutionPolicy Bypass -File `"$d\watchdog_run.ps1`" -Run $Run" -WorkingDirectory $d) `
  -Trigger $wt | Out-Null
if ($At -eq "now") { Start-ScheduledTask -TaskName "TS run $Run" }
foreach ($t in "TS run $Run", "TS run $Run watchdog") { $x = Get-ScheduledTask -TaskName $t; "{0}: priority {1}, next start {2}" -f $t, $x.Settings.Priority, (Get-ScheduledTaskInfo -TaskName $t).NextRunTime }
