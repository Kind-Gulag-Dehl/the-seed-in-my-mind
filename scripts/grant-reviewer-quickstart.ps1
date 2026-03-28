param(
    [ValidateSet("15", "30")]
    [string]$Profile = "15"
)

$ErrorActionPreference = "Stop"

$repoRoot = (Resolve-Path (Join-Path $PSScriptRoot "..")).Path
Set-Location $repoRoot

function Test-DemoDatabaseConfig {
    $databaseUrl = [Environment]::GetEnvironmentVariable("DATABASE_URL")
    $backendEnvPath = Join-Path $repoRoot "backend\.env"

    if (-not [string]::IsNullOrWhiteSpace($databaseUrl)) {
        return $true
    }

    if (Test-Path $backendEnvPath) {
        return $true
    }

    Write-Host "[grant-review] DATABASE_URL is required to run the demo." -ForegroundColor Yellow
    Write-Host "[grant-review] See README.md -> 'Minimal local setup (quickstart)'" -ForegroundColor Yellow
    Write-Host "[grant-review] or docs/stage0-runtime-configuration.md -> 'Minimal local setup (quickstart)'." -ForegroundColor Yellow
    return $false
}

function Invoke-Step {
    param(
        [string]$Label,
        [scriptblock]$Action
    )

    Write-Host "[grant-review] $Label" -ForegroundColor Cyan
    & $Action
    if ($LASTEXITCODE -ne 0) {
        throw "[grant-review] step failed: $Label"
    }
}

Write-Host "Grant reviewer quickstart ($Profile-minute path)" -ForegroundColor Green
Write-Host "Repository: $repoRoot"
Write-Host ""

if (-not (Test-DemoDatabaseConfig)) {
    exit 1
}

Invoke-Step -Label "verify open-core boundaries" -Action { npm run verify:boundaries }
Invoke-Step -Label "verify canonical DTO surface" -Action { npm run verify:canonical-dto }

if ($Profile -eq "15") {
    Invoke-Step -Label "run deterministic open-core demo report" -Action {
        powershell -ExecutionPolicy Bypass -File scripts/open-core-demo.ps1
    }

    Write-Host ""
    Write-Host "15-minute path complete." -ForegroundColor Green
    Write-Host "Next:" -ForegroundColor Green
    Write-Host "  - Read docs/open-core-reviewer-guide.md"
    Write-Host "  - For the fuller path, rerun: powershell -ExecutionPolicy Bypass -File scripts/grant-reviewer-quickstart.ps1 -Profile 30"
    exit 0
}

Invoke-Step -Label "build and test the reference frontend" -Action {
    Push-Location (Join-Path $repoRoot "frontend\\open-core-reference")
    try {
        npm install
        if ($LASTEXITCODE -ne 0) { exit $LASTEXITCODE }
        npm test
        if ($LASTEXITCODE -ne 0) { exit $LASTEXITCODE }
        npm run build
    } finally {
        Pop-Location
    }
}

Invoke-Step -Label "run deterministic open-core demo report with reference frontend build" -Action {
    powershell -ExecutionPolicy Bypass -File scripts/open-core-demo.ps1 -BuildReferenceFrontend
}

Invoke-Step -Label "run open-core backend verification surface" -Action {
    powershell -ExecutionPolicy Bypass -File backend/scripts/verify-open-core.ps1
}

Write-Host ""
Write-Host "30-minute path complete." -ForegroundColor Green
Write-Host "Key follow-ups:" -ForegroundColor Green
Write-Host "  - Read docs/open-core-implementation-status.md for implemented vs planned scope."
Write-Host "  - Read docs/open-core-architecture-overview.md for boundary and data-flow context."
Write-Host "  - Use docs/open-core-demo-flow.md if you want to repeat the demo manually."
