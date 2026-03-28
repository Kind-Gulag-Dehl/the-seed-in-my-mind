param(
    [string]$ExportRoot = "_export/open-core",
    [string]$SeedDataFile = "seed/reviewer-demo.seed-data-v0.json"
)

$ErrorActionPreference = "Stop"

function Import-EnvFile {
    param([string]$FilePath)

    if (-not (Test-Path $FilePath)) {
        return
    }

    Get-Content $FilePath | ForEach-Object {
        $line = $_.Trim()
        if ($line.Length -eq 0 -or $line.StartsWith("#")) {
            return
        }

        $parts = $line.Split("=", 2)
        if ($parts.Length -ne 2) {
            return
        }

        $name = $parts[0].Trim()
        $value = $parts[1].Trim()
        if ($name.Length -eq 0) {
            return
        }

        $existing = (Get-Item -Path "Env:$name" -ErrorAction SilentlyContinue).Value
        if ([string]::IsNullOrWhiteSpace($existing)) {
            Set-Item -Path "Env:$name" -Value $value
        }
    }
}

function Remove-PathIfPresent {
    param([string]$Path)

    if (Test-Path $Path) {
        Remove-Item -Path $Path -Recurse -Force -ErrorAction SilentlyContinue
    }
}

function Cleanup-FrontendArtifacts {
    param([string]$FrontendDir)

    Remove-PathIfPresent (Join-Path $FrontendDir "dist")
    Remove-PathIfPresent (Join-Path $FrontendDir "node_modules")
    Remove-PathIfPresent (Join-Path $FrontendDir ".vite")
    Remove-PathIfPresent (Join-Path $FrontendDir "coverage")
    Remove-PathIfPresent (Join-Path $FrontendDir "package-lock.json")
    Remove-PathIfPresent (Join-Path $FrontendDir "tsconfig.tsbuildinfo")
}

function Invoke-JsonRequest {
    param(
        [string]$Uri
    )

    $response = Invoke-WebRequest -Uri $Uri -UseBasicParsing -TimeoutSec 5
    if ($response.StatusCode -ne 200) {
        throw "[smoke-export] unexpected status for ${Uri}: $($response.StatusCode)"
    }

    return ($response.Content | ConvertFrom-Json)
}

$resolvedExportRoot = (Resolve-Path $ExportRoot).Path
$scriptRepoRoot = (Resolve-Path (Join-Path $PSScriptRoot "..\\..")).Path
$backendDir = Join-Path $resolvedExportRoot "backend"
$frontendDir = Join-Path $resolvedExportRoot "frontend\\open-core-reference"
$seedPath = if ([System.IO.Path]::IsPathRooted($SeedDataFile)) {
    $SeedDataFile
} else {
    (Resolve-Path (Join-Path $resolvedExportRoot $SeedDataFile)).Path
}

if (-not (Test-Path $backendDir)) {
    throw "[smoke-export] backend directory missing: $backendDir"
}
if (-not (Test-Path $frontendDir)) {
    throw "[smoke-export] frontend reference directory missing: $frontendDir"
}
if (-not (Test-Path $seedPath)) {
    throw "[smoke-export] seed file missing: $seedPath"
}

$env:Path = "$env:USERPROFILE\.cargo\bin;$env:Path"
if (-not (Get-Command cargo -ErrorAction SilentlyContinue)) {
    throw "[smoke-export] cargo not found on PATH"
}
if (-not (Get-Command npm -ErrorAction SilentlyContinue)) {
    throw "[smoke-export] npm not found on PATH"
}
if (-not (Get-Command psql -ErrorAction SilentlyContinue)) {
    throw "[smoke-export] psql not found on PATH"
}

Import-EnvFile (Join-Path $backendDir ".env")
if ([string]::IsNullOrWhiteSpace($env:DATABASE_URL)) {
    Import-EnvFile (Join-Path $scriptRepoRoot "backend\\.env")
}
if ([string]::IsNullOrWhiteSpace($env:DATABASE_URL) -and -not [string]::IsNullOrWhiteSpace($env:database_url)) {
    $env:DATABASE_URL = $env:database_url
}
if ([string]::IsNullOrWhiteSpace($env:PGPASSFILE) -and -not [string]::IsNullOrWhiteSpace($env:pgpassfile)) {
    $env:PGPASSFILE = $env:pgpassfile
}
if ([string]::IsNullOrWhiteSpace($env:DATABASE_URL)) {
    throw "[smoke-export] DATABASE_URL is required for exported runtime smoke verification. Set it in the environment or create backend/.env from backend/.env.example."
}

$cargoTargetDir = Join-Path $env:TEMP ("seed-open-core-export-target-" + [guid]::NewGuid().ToString("N"))
$serverStdoutLogPath = Join-Path $env:TEMP ("seed-open-core-export-api-out-" + [guid]::NewGuid().ToString("N") + ".log")
$serverStderrLogPath = Join-Path $env:TEMP ("seed-open-core-export-api-err-" + [guid]::NewGuid().ToString("N") + ".log")
$serverProcess = $null

$env:CARGO_TARGET_DIR = $cargoTargetDir
$env:CARGO_INCREMENTAL = "0"
$env:CARGO_PROFILE_DEV_DEBUG = "0"
$env:CARGO_PROFILE_TEST_DEBUG = "0"

try {
    Write-Host "[smoke-export] reset database with reviewer demo seed" -ForegroundColor Cyan
    & (Join-Path $backendDir "scripts\\reset-dev-db.ps1") -SeedDataFile $seedPath
    if ($LASTEXITCODE -ne 0) {
        throw "[smoke-export] reset-dev-db failed"
    }

    Write-Host "[smoke-export] verify latest snapshot" -ForegroundColor Cyan
    Push-Location $backendDir
    try {
        cargo run -p snapshot-verify -- --latest --profile stage0
        if ($LASTEXITCODE -ne 0) {
            throw "[smoke-export] snapshot verification failed"
        }
    } finally {
        Pop-Location
    }

    Write-Host "[smoke-export] build exported backend (api-server open_core)" -ForegroundColor Cyan
    Push-Location $backendDir
    try {
        cargo build -p api-server --no-default-features --features open_core
        if ($LASTEXITCODE -ne 0) {
            throw "[smoke-export] backend build failed"
        }
    } finally {
        Pop-Location
    }

    Write-Host "[smoke-export] start exported api-server" -ForegroundColor Cyan
    $serverProcess = Start-Process `
        -FilePath "cargo" `
        -ArgumentList @("run", "-p", "api-server", "--no-default-features", "--features", "open_core") `
        -WorkingDirectory $backendDir `
        -RedirectStandardOutput $serverStdoutLogPath `
        -RedirectStandardError $serverStderrLogPath `
        -PassThru

    $healthReady = $false
    for ($attempt = 0; $attempt -lt 60; $attempt++) {
        Start-Sleep -Milliseconds 500
        if ($serverProcess.HasExited) {
            $logTail = ""
            if (Test-Path $serverStdoutLogPath) {
                $logTail += ((Get-Content $serverStdoutLogPath -Tail 20) -join "`n")
            }
            if (Test-Path $serverStderrLogPath) {
                if (-not [string]::IsNullOrWhiteSpace($logTail)) {
                    $logTail += "`n"
                }
                $logTail += ((Get-Content $serverStderrLogPath -Tail 20) -join "`n")
            }
            throw "[smoke-export] api-server exited early.`n$logTail"
        }

        try {
            $response = Invoke-WebRequest -Uri "http://127.0.0.1:3000/api/v0/health" -UseBasicParsing -TimeoutSec 2
            if ($response.StatusCode -eq 200) {
                $healthReady = $true
                break
            }
        } catch {
            # keep waiting
        }
    }

    if (-not $healthReady) {
        throw "[smoke-export] api-server health endpoint did not become ready"
    }

    Write-Host "[smoke-export] query canonical snapshot and top ideas" -ForegroundColor Cyan
    $snapshot = Invoke-JsonRequest -Uri "http://127.0.0.1:3000/api/v0/snapshot/latest?include_preview=true"
    $ideas = Invoke-JsonRequest -Uri "http://127.0.0.1:3000/api/v0/ideas/top?limit=10&offset=0&order=asc"
    $detail = Invoke-JsonRequest -Uri "http://127.0.0.1:3000/api/v0/idea/59427f80-5901-7128-990e-90b49f288bcc"

    if (-not $snapshot.snapshot.snapshot_hash) {
        throw "[smoke-export] snapshot response missing snapshot_hash"
    }
    if ($ideas.ideas.Count -lt 3) {
        throw "[smoke-export] expected at least three ideas from reviewer demo dataset"
    }
    if ($detail.idea.title -ne "abstraction over redaction") {
        throw "[smoke-export] expected reviewer demo idea title in API detail response"
    }

        Write-Host "[smoke-export] install and build exported reference frontend" -ForegroundColor Cyan
        Push-Location $frontendDir
        try {
            npm install
            if ($LASTEXITCODE -ne 0) {
                throw "[smoke-export] frontend dependency install failed"
            }

        npm run build
        if ($LASTEXITCODE -ne 0) {
            throw "[smoke-export] frontend build failed"
        }
    } finally {
        Pop-Location
    }

    Write-Host "[smoke-export] verified snapshot_height=$($snapshot.snapshot.height) shared_map_commitment=$($snapshot.snapshot.shared_map_commitment)" -ForegroundColor Green
    Write-Host "[smoke-export] verified demo idea title=$($detail.idea.title)" -ForegroundColor Green
    Write-Host "PASS smoke-export" -ForegroundColor Green
} finally {
    if ($null -ne $serverProcess) {
        try {
            if (-not $serverProcess.HasExited) {
                Stop-Process -Id $serverProcess.Id -Force -ErrorAction SilentlyContinue
                Wait-Process -Id $serverProcess.Id -Timeout 10 -ErrorAction SilentlyContinue
            }
        } catch {
            Write-Host "[smoke-export] warning: failed to stop api-server process cleanly" -ForegroundColor Yellow
        }
    }

    Cleanup-FrontendArtifacts -FrontendDir $frontendDir
    Remove-PathIfPresent (Join-Path $backendDir "var")
    Remove-PathIfPresent $cargoTargetDir
    Remove-PathIfPresent $serverStdoutLogPath
    Remove-PathIfPresent $serverStderrLogPath
}
