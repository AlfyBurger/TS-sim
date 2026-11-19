# Twilight Struggle run worker (generic, from run 7 on). Started by start_run.bat <Run> through Task Scheduler.
# Keeps the PC awake, runs `tssim<Run>.exe worker run<Run>_jobs.txt out<Run>`, writes status<Run>.txt every minute,
# and writes RUN<Run>_COMPLETE.txt (with a banner) when every job has its result.
param([Parameter(Mandatory = $true)][string]$Run, [string]$Exe = "")
$ErrorActionPreference = "Continue"
Set-Location $PSScriptRoot
if (-not $Exe) { $Exe = "tssim$Run.exe" }
$jobs = "run${Run}_jobs.txt"; $out = "out$Run"; $status = "status$Run.txt"; $done_file = "RUN${Run}_COMPLETE.txt"
# Task Scheduler starts tasks at BelowNormal priority and Windows then throttles them (19 Nov 2026): run at Normal.
try { (Get-Process -Id $PID).PriorityClass = 'Normal' } catch {}
$Host.UI.RawUI.WindowTitle = "TS simulator - run $Run"
Add-Type -Name Power -Namespace Win32 -MemberDefinition '[DllImport("kernel32.dll")] public static extern uint SetThreadExecutionState(uint f);'
[Win32.Power]::SetThreadExecutionState([uint32]"0x80000001") | Out-Null   # ES_CONTINUOUS | ES_SYSTEM_REQUIRED
if (Test-Path STOP) { Rename-Item STOP ("STOP_old_" + (Get-Date -Format yyyyMMddHHmmss)) }
New-Item -ItemType Directory -Force $out | Out-Null
$expected = Get-Content $jobs | Where-Object { $_.Trim() -ne "" } | ForEach-Object { $f = $_ -split "`t"; "$out\" + $f[0] + "__" + $f[3] + ".csv" }
$start = Get-Date
"started $start" | Out-File -Encoding utf8 "run${Run}_started.txt"
Write-Host "Run $Run started $start - $($expected.Count) jobs. Closing this window pauses the run (it resumes where it stopped)."
$p = Start-Process -FilePath ".\$Exe" -ArgumentList "worker", $jobs, $out -NoNewWindow -PassThru
try { $p.PriorityClass = 'Normal' } catch {}
while ($true) {
  Start-Sleep -Seconds 60
  $done = @($expected | Where-Object { Test-Path $_ }).Count
  $el = (Get-Date) - $start
  $prio = try { $p.PriorityClass } catch { "?" }
  "$(Get-Date -Format 'yyyy-MM-dd HH:mm') done $done / $($expected.Count) jobs; elapsed $([int][math]::Floor($el.TotalHours))h$($el.Minutes)m; worker alive $(-not $p.HasExited); priority $prio" | Out-File -Encoding utf8 $status
  if ($done -ge $expected.Count) { break }
  if ($p.HasExited) { Write-Host "worker exited early"; "worker exited early $(Get-Date)" | Out-File -Encoding utf8 -Append $status; break }
}
New-Item -ItemType File STOP -Force | Out-Null
$p.WaitForExit(120000) | Out-Null
if ($done -ge $expected.Count) {
  "complete $(Get-Date)" | Out-File -Encoding utf8 $done_file
  Write-Host ""; Write-Host "  ##########################################" -ForegroundColor Green
  Write-Host ("  ##   RUN {0} COMPLETE  {1}  ##" -f $Run, (Get-Date -Format 'yyyy-MM-dd HH:mm')) -ForegroundColor Green
  Write-Host "  ##########################################" -ForegroundColor Green
}
[Win32.Power]::SetThreadExecutionState([uint32]"0x80000000") | Out-Null
Read-Host "Press Enter to close"
