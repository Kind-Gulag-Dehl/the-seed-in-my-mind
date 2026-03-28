param(
    [string]$SeedDataFile = "seed/reviewer-demo.seed-data-v0.json",
    [switch]$BuildReferenceFrontend,
    [switch]$KeepServerRunning
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

function Invoke-JsonRequest {
    param([string]$Uri)

    $response = Invoke-WebRequest -Uri $Uri -UseBasicParsing -TimeoutSec 5
    if ($response.StatusCode -ne 200) {
        throw "[open-core-demo] unexpected status for ${Uri}: $($response.StatusCode)"
    }

    return ($response.Content | ConvertFrom-Json)
}

$repoRoot = (Resolve-Path (Join-Path $PSScriptRoot "..")).Path
$backendDir = Join-Path $repoRoot "backend"
$frontendDir = Join-Path $repoRoot "frontend\\open-core-reference"
$seedPath = if ([System.IO.Path]::IsPathRooted($SeedDataFile)) {
    $SeedDataFile
} else {
    (Resolve-Path (Join-Path $repoRoot $SeedDataFile)).Path
}

if (-not (Test-Path $seedPath)) {
    throw "[open-core-demo] missing seed file: $seedPath"
}

$env:Path = "$env:USERPROFILE\.cargo\bin;$env:Path"
if (-not (Get-Command cargo -ErrorAction SilentlyContinue)) {
    throw "[open-core-demo] cargo not found on PATH"
}
if (-not (Get-Command psql -ErrorAction SilentlyContinue)) {
    throw "[open-core-demo] psql not found on PATH"
}

Import-EnvFile (Join-Path $backendDir ".env")
if ([string]::IsNullOrWhiteSpace($env:DATABASE_URL) -and -not [string]::IsNullOrWhiteSpace($env:database_url)) {
    $env:DATABASE_URL = $env:database_url
}
if ([string]::IsNullOrWhiteSpace($env:PGPASSFILE) -and -not [string]::IsNullOrWhiteSpace($env:pgpassfile)) {
    $env:PGPASSFILE = $env:pgpassfile
}
if ([string]::IsNullOrWhiteSpace($env:DATABASE_URL)) {
    throw "[open-core-demo] DATABASE_URL is required. Set it in the environment or copy backend/.env.example to backend/.env."
}

$cargoTargetDir = Join-Path $env:TEMP ("seed-open-core-demo-target-" + [guid]::NewGuid().ToString("N"))
$serverStdoutLogPath = Join-Path $env:TEMP ("seed-open-core-demo-api-out-" + [guid]::NewGuid().ToString("N") + ".log")
$serverStderrLogPath = Join-Path $env:TEMP ("seed-open-core-demo-api-err-" + [guid]::NewGuid().ToString("N") + ".log")
$serverProcess = $null
$builtReferenceFrontend = $false

$env:CARGO_TARGET_DIR = $cargoTargetDir
$env:CARGO_INCREMENTAL = "0"
$env:CARGO_PROFILE_DEV_DEBUG = "0"
$env:CARGO_PROFILE_TEST_DEBUG = "0"

try {
    Write-Host "[open-core-demo] reset database with reviewer demo seed" -ForegroundColor Cyan
    & (Join-Path $backendDir "scripts\\reset-dev-db.ps1") -SeedDataFile $seedPath
    if ($LASTEXITCODE -ne 0) {
        throw "[open-core-demo] reset-dev-db failed"
    }

    Write-Host "[open-core-demo] verify latest snapshot" -ForegroundColor Cyan
    Push-Location $backendDir
    try {
        cargo run -p snapshot-verify -- --latest --profile stage0
        if ($LASTEXITCODE -ne 0) {
            throw "[open-core-demo] snapshot verification failed"
        }
    } finally {
        Pop-Location
    }

    Write-Host "[open-core-demo] start api-server (open_core)" -ForegroundColor Cyan
    $serverProcess = Start-Process `
        -FilePath "cargo" `
        -ArgumentList @("run", "-p", "api-server", "--no-default-features", "--features", "open_core") `
        -WorkingDirectory $backendDir `
        -RedirectStandardOutput $serverStdoutLogPath `
        -RedirectStandardError $serverStderrLogPath `
        -PassThru

    $ready = $false
    for ($attempt = 0; $attempt -lt 60; $attempt++) {
        Start-Sleep -Milliseconds 500
        try {
            $health = Invoke-WebRequest -Uri "http://127.0.0.1:3000/api/v0/health" -UseBasicParsing -TimeoutSec 2
            if ($health.StatusCode -eq 200) {
                $ready = $true
                break
            }
        } catch {
            if ($serverProcess.HasExited) {
                break
            }
        }
    }

    if (-not $ready) {
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
        throw "[open-core-demo] api-server did not become ready.`n$logTail"
    }

    $snapshot = Invoke-JsonRequest -Uri "http://127.0.0.1:3000/api/v0/snapshot/latest?include_preview=true"
    $ideas = Invoke-JsonRequest -Uri "http://127.0.0.1:3000/api/v0/ideas/top?limit=10&offset=0&order=asc"
    $ideaDetail = Invoke-JsonRequest -Uri "http://127.0.0.1:3000/api/v0/idea/59427f80-5901-7128-990e-90b49f288bcc"

    if ($BuildReferenceFrontend) {
        if (-not (Get-Command npm -ErrorAction SilentlyContinue)) {
            throw "[open-core-demo] npm not found on PATH"
        }

        Write-Host "[open-core-demo] build reference frontend" -ForegroundColor Cyan
        Push-Location $frontendDir
        try {
            npm install
            if ($LASTEXITCODE -ne 0) {
                throw "[open-core-demo] npm install failed"
            }
            npm test
            if ($LASTEXITCODE -ne 0) {
                throw "[open-core-demo] frontend tests failed"
            }
            npm run build
            if ($LASTEXITCODE -ne 0) {
                throw "[open-core-demo] frontend build failed"
            }
            $builtReferenceFrontend = $true
        } finally {
            Pop-Location
        }
    }

    Write-Host ""
    Write-Host "Open-core demo report" -ForegroundColor Green
    Write-Host "  snapshot height        : $($snapshot.snapshot.height)"
    Write-Host "  snapshot hash          : $($snapshot.snapshot.snapshot_hash)"
    Write-Host "  shared map commitment  : $($snapshot.snapshot.shared_map_commitment)"
    Write-Host "  event count            : $($snapshot.snapshot.event_count)"
    Write-Host "  imported demo idea     : $($ideaDetail.idea.title)"
    Write-Host "  imported demo sentence : $($ideaDetail.idea.sentence)"
    Write-Host "  top idea titles        : $((@($ideas.ideas | Select-Object -First 3 | ForEach-Object { $_.title }) -join ' | '))"
    Write-Host "  api health             : http://127.0.0.1:3000/api/v0/health"
    if ($BuildReferenceFrontend) {
        Write-Host "  reference frontend     : frontend/open-core-reference built successfully"
    }
    Write-Host ""
    Write-Host "What this proves:" -ForegroundColor Green
    Write-Host "  1. Canonical events were ingested from a deterministic public seed file."
    Write-Host "  2. Replay materialized canonical state and snapshot commitments."
    Write-Host "  3. The read-only open-core API serves that verified state."
    Write-Host "  4. The reference viewer can be built against the same API surface."
    Write-Host ""
    Write-Host "PASS open-core-demo" -ForegroundColor Green

    if ($KeepServerRunning) {
        Write-Host "[open-core-demo] api-server left running at http://127.0.0.1:3000" -ForegroundColor Yellow
        $serverProcess = $null
    }
} finally {
    if ($null -ne $serverProcess) {
        try {
            if (-not $serverProcess.HasExited) {
                Stop-Process -Id $serverProcess.Id -Force -ErrorAction SilentlyContinue
                Wait-Process -Id $serverProcess.Id -Timeout 10 -ErrorAction SilentlyContinue
            }
        } catch {
            Write-Host "[open-core-demo] warning: failed to stop api-server process cleanly" -ForegroundColor Yellow
        }
    }

    Remove-Item -Path $cargoTargetDir -Recurse -Force -ErrorAction SilentlyContinue
    if ($builtReferenceFrontend) {
        Remove-Item -Path (Join-Path $frontendDir "dist") -Recurse -Force -ErrorAction SilentlyContinue
        Remove-Item -Path (Join-Path $frontendDir "node_modules") -Recurse -Force -ErrorAction SilentlyContinue
        Remove-Item -Path (Join-Path $frontendDir ".vite") -Recurse -Force -ErrorAction SilentlyContinue
        Remove-Item -Path (Join-Path $frontendDir "coverage") -Recurse -Force -ErrorAction SilentlyContinue
        Remove-Item -Path (Join-Path $frontendDir "package-lock.json") -Force -ErrorAction SilentlyContinue
        Remove-Item -Path (Join-Path $frontendDir "tsconfig.tsbuildinfo") -Recurse -Force -ErrorAction SilentlyContinue
    }
    Remove-Item -Path $serverStdoutLogPath -Force -ErrorAction SilentlyContinue
    Remove-Item -Path $serverStderrLogPath -Force -ErrorAction SilentlyContinue
}
