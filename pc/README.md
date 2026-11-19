# TS runs on Alfy's PC: the run kit (from run 7 on)

Copy these four files into `C:\Users\Alfy\Desktop\Game Dev\TS` (already there since 19 Nov 2026), plus, for run N:
`tssimN.exe` (cross-built: `cargo zigbuild --release --target x86_64-pc-windows-gnu`; send it under a new staged name,
rename on the PC and check the MD5) and `runN_jobs.txt` (label, bot A, bot B, seed, deals; tab-separated).

Register and start (from Desktop Commander; Task Scheduler then owns the processes, so they survive Claude's shell):

    powershell -ExecutionPolicy Bypass -File register_run.ps1 -Run 7 -At now      # or -At "23:00"

This creates the tasks "TS run N" and "TS run N watchdog" (every 10 min for 3 days), both at **Normal priority**
(Task Scheduler's default, BelowNormal, gets throttled by Windows: about a third of a core per process, 19 Nov 2026).
The worker also sets its own process and `tssimN.exe` to Normal, and the watchdog corrects a worker found at a lower priority.

Files the run writes: `outN\` (one CSV per block), `statusN.txt` (every minute: done / total, alive, priority),
`runN_started.txt`, `RUNN_COMPLETE.txt` (with a banner window), `watchdogN.txt` (restarts only).

Pause for Alfy: create `PAUSE`, then `STOP` (the worker ends after its current block; the watchdog leaves it alone).
Resume: rename `PAUSE` and `STOP`, `Start-ScheduledTask -TaskName "TS run N"`, check a minute later that `tssimN.exe` is alive.
After the run: `Disable-ScheduledTask -TaskName "TS run N watchdog"`.

`register_run.ps1` refuses to start if `RUNN_COMPLETE.txt`, `watchdogN.txt`, `PAUSE` or `STOP` exist (rename them first).
Tested 19 Nov 2026 in `TS\kit_test` (run "test"): Normal priority throughout, watchdog restart after a killed worker, completion file written.
