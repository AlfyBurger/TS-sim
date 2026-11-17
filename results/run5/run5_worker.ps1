# Twilight Struggle run 5 worker wrapper: keeps the PC awake, runs tssim worker,
# writes status.txt every minute, and ends the worker when every job has its result.
$ErrorActionPreference = "Continue"
Set-Location $PSScriptRoot
$Host.UI.RawUI.WindowTitle = "TS simulator - run 5"
Add-Type -Name Power -Namespace Win32 -MemberDefinition '[DllImport("kernel32.dll")] public static extern uint SetThreadExecutionState(uint f);'
[Win32.Power]::SetThreadExecutionState([uint32]"0x80000001") | Out-Null   # ES_CONTINUOUS | ES_SYSTEM_REQUIRED
if (Test-Path STOP) { Rename-Item STOP ("STOP_old_" + (Get-Date -Format yyyyMMddHHmmss)) }
$expected = Get-Content jobs.txt | Where-Object { $_.Trim() -ne "" } | ForEach-Object { $f = $_ -split "`t"; "out\" + $f[0] + "__" + $f[3] + ".csv" }
$start = Get-Date
"started $start" | Out-File -Encoding utf8 run5_started.txt
Write-Host "Run 5 started $start - $($expected.Count) jobs. Closing this window pauses the run (it resumes where it stopped)."
$p = Start-Process -FilePath ".\tssim.exe" -ArgumentList "worker","jobs.txt","out" -NoNewWindow -PassThru
while ($true) {
  Start-Sleep -Seconds 60
  $done = @($expected | Where-Object { Test-Path $_ }).Count
  $spsa = Get-ChildItem out\SPSA_T_log.txt -ErrorAction SilentlyContinue
  $spsaIt = if ($spsa) { (Get-Content $spsa.FullName | Measure-Object).Count } else { 0 }
  $el = (Get-Date) - $start
  "$(Get-Date -Format 'yyyy-MM-dd HH:mm') done $done / $($expected.Count) jobs; SPSA log lines $spsaIt; elapsed $([int]$el.TotalHours)h$($el.Minutes)m; worker alive $(-not $p.HasExited)" | Out-File -Encoding utf8 status.txt
  if ($done -ge $expected.Count) { break }
  if ($p.HasExited) { Write-Host "worker exited early"; "worker exited early $(Get-Date)" | Out-File -Encoding utf8 -Append status.txt; break }
}
New-Item -ItemType File STOP -Force | Out-Null
$p.WaitForExit(120000) | Out-Null
if ($done -ge $expected.Count) {
  "complete $(Get-Date)" | Out-File -Encoding utf8 RUN_COMPLETE.txt
  Write-Host ""; Write-Host "  ##########################################" -ForegroundColor Green
  Write-Host "  ##   RUN 5 COMPLETE  $(Get-Date -Format 'yyyy-MM-dd HH:mm')     ##" -ForegroundColor Green
  Write-Host "  ##########################################" -ForegroundColor Green
}
[Win32.Power]::SetThreadExecutionState([uint32]"0x80000000") | Out-Null
Read-Host "Press Enter to close"
