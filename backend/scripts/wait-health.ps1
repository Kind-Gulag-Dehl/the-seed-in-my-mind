$ErrorActionPreference = "Stop"

$urls = @(
  "http://127.0.0.1:3000/health",
  "http://127.0.0.1:3000/api/v0/health"
)
$defaultTimeoutSeconds = 120
$defaultIntervalMs = 500
$progressEverySeconds = 2

function Get-EnvIntOrDefault {
  param(
    [string]$Name,
    [int]$DefaultValue
  )

  $raw = [Environment]::GetEnvironmentVariable($Name)
  if ([string]::IsNullOrWhiteSpace($raw)) {
    return $DefaultValue
  }

  $parsed = 0
  if ([int]::TryParse($raw, [ref]$parsed) -and $parsed -gt 0) {
    return $parsed
  }

  Write-Host "[wait-health] ignoring invalid $Name='$raw'; using default $DefaultValue"
  return $DefaultValue
}

$timeoutSeconds = Get-EnvIntOrDefault -Name "SEED_WAIT_HEALTH_TIMEOUT_SECONDS" -DefaultValue $defaultTimeoutSeconds
$intervalMs = Get-EnvIntOrDefault -Name "SEED_WAIT_HEALTH_INTERVAL_MS" -DefaultValue $defaultIntervalMs
$stopwatch = [System.Diagnostics.Stopwatch]::StartNew()
$lastProgressSeconds = -1.0
$lastFailure = "none yet"

Write-Host ("[wait-health] waiting for health (timeout={0}s interval={1}ms urls={2})" -f $timeoutSeconds, $intervalMs, ($urls -join ", "))

while ($stopwatch.Elapsed.TotalSeconds -lt $timeoutSeconds) {
  foreach ($url in $urls) {
    try {
      $resp = Invoke-WebRequest -Uri $url -UseBasicParsing -TimeoutSec 3
      if ($resp.StatusCode -eq 200) {
        $body = $resp.Content
        $ok = $false
        try {
          $json = $body | ConvertFrom-Json
          if ($json.ok -eq $true) {
            $ok = $true
          }
        } catch {
          if ($body -match '\"ok\"\\s*:\\s*true') {
            $ok = $true
          }
        }
        if ($ok) {
          Write-Host ("[wait-health] PASS {0} elapsed={1:N1}s" -f $url, $stopwatch.Elapsed.TotalSeconds)
          exit 0
        }
        $lastFailure = "url=$url status=200 but body did not confirm ok=true"
      } else {
        $lastFailure = "url=$url status=$($resp.StatusCode)"
      }
    } catch {
      $ex = $_.Exception
      if ($null -ne $ex -and -not [string]::IsNullOrWhiteSpace($ex.Message)) {
        $lastFailure = "url=$url error=$($ex.Message)"
      } else {
        $lastFailure = "url=$url request failed"
      }
    }
  }

  if (($stopwatch.Elapsed.TotalSeconds - $lastProgressSeconds) -ge $progressEverySeconds) {
    Write-Host ("[wait-health] waiting... elapsed={0:N1}s last_error={1}" -f $stopwatch.Elapsed.TotalSeconds, $lastFailure)
    $lastProgressSeconds = $stopwatch.Elapsed.TotalSeconds
  }

  Start-Sleep -Milliseconds $intervalMs
}

Write-Host ("[wait-health] FAIL elapsed={0:N1}s last_error={1}" -f $stopwatch.Elapsed.TotalSeconds, $lastFailure)
exit 1
