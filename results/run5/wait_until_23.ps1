$n = Get-Date
if ($n.Hour -ge 23 -or $n.Hour -lt 7) {
  Write-Host "Starting now."
} else {
  $t = [datetime]::Today.AddHours(23)
  Write-Host ("Waiting until " + $t.ToString("HH:mm") + " ... (leave this window open)")
  Start-Sleep -Seconds ([int]($t - $n).TotalSeconds)
}
