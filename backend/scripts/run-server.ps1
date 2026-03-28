$ErrorActionPreference = "Stop"

$env:Path = "$env:USERPROFILE\.cargo\bin;$env:Path"

. "$PSScriptRoot\dev-bootstrap.ps1"

$repoRoot = Resolve-Path (Join-Path $PSScriptRoot "..\..")
$backendDir = Join-Path $repoRoot "backend"
$seedPath = Join-Path $repoRoot "seed\seed-data-v0.json"
$resetScriptPath = Join-Path $backendDir "scripts\reset-dev-db.ps1"

if (-not (Test-Path -Path $seedPath)) {
  Write-Error "Missing seed file at $seedPath"
  exit 1
}

Write-Host "[run-server] building api-server (cargo build -p api-server) ..."
Push-Location $backendDir
cargo build -p api-server
Write-Host "[run-server] building seed-importer (cargo build -p seed-importer) ..."
cargo build -p seed-importer
Pop-Location
Write-Host "[run-server] build complete"

$seedImporterExe = Join-Path $backendDir "target\debug\seed-importer.exe"
if (-not (Test-Path -Path $seedImporterExe)) {
  Write-Error "Missing seed-importer binary at $seedImporterExe"
  exit 1
}

Write-Host "[run-server] seeding database (if empty) ..."
$currentEventCountRaw = psql $env:DATABASE_URL -t -A -v ON_ERROR_STOP=1 -c "SELECT COUNT(*) FROM events;"
if ($LASTEXITCODE -ne 0) {
  Write-Error "Failed to query current event count"
  exit $LASTEXITCODE
}

$currentEventCount = 0
[void][int]::TryParse(($currentEventCountRaw | Out-String).Trim(), [ref]$currentEventCount)

if ($currentEventCount -gt 0 -and $currentEventCount -lt 100) {
  Write-Host "[run-server] detected reviewer demo dataset in local dev DB; resetting to full development seed ..." -ForegroundColor Yellow
  & $resetScriptPath -SeedDataFile $seedPath
  if ($LASTEXITCODE -ne 0) {
    Write-Error "reset-dev-db failed with exit code $LASTEXITCODE"
    exit $LASTEXITCODE
  }
} else {
  & $seedImporterExe --file $seedPath
  if ($LASTEXITCODE -ne 0) {
    Write-Error "seed-importer failed with exit code $LASTEXITCODE"
    exit $LASTEXITCODE
  }
}

Write-Host "[run-server] building snapshot (snapshot-builder) ..."
Push-Location $backendDir
cargo run -p snapshot-builder
Pop-Location

$exePath = Join-Path $backendDir "target\debug\api-server.exe"
if (-not (Test-Path -Path $exePath)) {
  Write-Error "Missing backend binary at $exePath. Build it with backend\scripts\dev-bootstrap.ps1"
  exit 1
}

$redactedDbUrl = $env:DATABASE_URL -replace "^(.*://[^:/]+:)[^@]+(@.*)$", '$1***$2'
Write-Host "[run-server] DATABASE_URL=$redactedDbUrl"
Write-Host "[run-server] starting api-server.exe ..."

& $exePath
