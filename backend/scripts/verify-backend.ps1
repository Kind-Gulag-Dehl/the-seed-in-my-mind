$ErrorActionPreference = "Stop"

$repoRoot = (Resolve-Path (Join-Path $PSScriptRoot "..\..")).Path
$backendDir = Join-Path $repoRoot "backend"
Set-Location $backendDir

. "$PSScriptRoot\dev-bootstrap.ps1"

if ([string]::IsNullOrWhiteSpace($env:CARGO_TARGET_DIR)) {
    $env:CARGO_TARGET_DIR = Join-Path $env:TEMP "seed-backend-verify-target"
}

$env:CARGO_INCREMENTAL = "0"

Write-Host "[verify-backend] cargo build" -ForegroundColor Cyan
cargo build
if ($LASTEXITCODE -ne 0) {
    Write-Host "FAIL verify-backend" -ForegroundColor Red
    exit 1
}

Write-Host "[verify-backend] cargo test" -ForegroundColor Cyan
cargo test
if ($LASTEXITCODE -ne 0) {
    Write-Host "FAIL verify-backend" -ForegroundColor Red
    exit 1
}

Write-Host "PASS verify-backend" -ForegroundColor Green
exit 0
