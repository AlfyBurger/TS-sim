# Watchdog for a TS run (every 10 min via Task Scheduler "TS run <Run> watchdog"). Restarts the run if it died.
# Leaves it alone when complete, or when a PAUSE file exists (an intentional pause by Claude for Alfy).
param([Parameter(Mandatory = $true)][string]$Run, [string]$Exe = "")
Set-Location $PSScriptRoot
if (-not $Exe) { $Exe = "tssim$Run.exe" }
$proc = [IO.Path]::GetFileNameWithoutExtension($Exe); $log = "watchdog$Run.txt"; $done_file = "RUN${Run}_COMPLETE.txt"
if ((Test-Path $done_file) -or (Test-Path PAUSE)) { exit }
$w = Get-Process $proc -ErrorAction SilentlyContinue
if ($w) { foreach ($x in $w) { if ($x.PriorityClass -ne 'Normal') { try { $x.PriorityClass = 'Normal' } catch {} } }; exit }
Start-Sleep -Seconds 90   # a block boundary or a starting worker: look again before acting
if (Get-Process $proc -ErrorAction SilentlyContinue) { exit }
if ((Test-Path $done_file) -or (Test-Path PAUSE)) { exit }
$n = @(Get-Content $log -ErrorAction SilentlyContinue | Select-String "restarted").Count
if ($n -ge 6) { if (-not (Select-String -Path $log -Pattern "not restarting" -Quiet)) { "$(Get-Date -Format 'yyyy-MM-dd HH:mm') not restarting: 6 restarts already" | Add-Content $log }; exit }
# end any leftover wrapper of the dead run (e.g. waiting at 'Press Enter'), so the task can start again
Get-CimInstance Win32_Process | Where-Object { $_.CommandLine -match "run_worker.ps1.*-Run $Run\b|start_run.bat $Run\b" } | ForEach-Object { Stop-Process -Id $_.ProcessId -Force -ErrorAction SilentlyContinue }
Start-Sleep -Seconds 5
if (Test-Path STOP) { Rename-Item STOP ("STOP_old_" + (Get-Date -Format yyyyMMddHHmmss)) }
Start-ScheduledTask -TaskName "TS run $Run"
"$(Get-Date -Format 'yyyy-MM-dd HH:mm') restarted (status was: $(Get-Content "status$Run.txt" -ErrorAction SilentlyContinue))" | Add-Content $log
