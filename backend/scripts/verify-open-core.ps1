$ErrorActionPreference = "Stop"

$repoRoot = (Resolve-Path (Join-Path $PSScriptRoot "..\..")).Path
$backendDir = Join-Path $repoRoot "backend"
Set-Location $backendDir

. "$PSScriptRoot\dev-bootstrap.ps1"

Write-Host "[verify-open-core] stop any running api-server.exe" -ForegroundColor Cyan
try { taskkill /IM api-server.exe /F 2>$null | Out-Null } catch { }

Write-Host "[verify-open-core] build api-server (open_core)" -ForegroundColor Cyan
cargo build -p api-server --no-default-features --features open_core
if ($LASTEXITCODE -ne 0) {
    Write-Host "FAIL verify-open-core" -ForegroundColor Red
    exit 1
}

Write-Host "[verify-open-core] run snapshot-builder" -ForegroundColor Cyan
cargo run -p snapshot-builder
if ($LASTEXITCODE -ne 0) {
    Write-Host "FAIL verify-open-core" -ForegroundColor Red
    exit 1
}

Write-Host "[verify-open-core] run snapshot-verify" -ForegroundColor Cyan
cargo run -p snapshot-verify -- --latest --profile stage0
if ($LASTEXITCODE -ne 0) {
    Write-Host "FAIL verify-open-core" -ForegroundColor Red
    exit 1
}

Write-Host "[verify-open-core] start api-server open_core (background)" -ForegroundColor Cyan
$job = Start-Job -ScriptBlock {
    param($backendDir)
    Set-Location $backendDir
    cargo run -p api-server --no-default-features --features open_core
} -ArgumentList $backendDir

Add-Type -AssemblyName System.Net.Http
$httpHandler = New-Object System.Net.Http.HttpClientHandler
$httpClient = New-Object System.Net.Http.HttpClient($httpHandler)
$httpClient.Timeout = [TimeSpan]::FromSeconds(20)

function Read-ResponseBody {
    param(
        [System.Net.Http.HttpResponseMessage]$Response
    )
    if ($null -eq $Response) {
        return ""
    }
    try {
        return $Response.Content.ReadAsStringAsync().GetAwaiter().GetResult()
    } catch {
        return ""
    }
}

function Invoke-Check {
    param(
        [string]$Method,
        [string]$Url,
        [int]$ExpectedStatus
    )

    $request = New-Object System.Net.Http.HttpRequestMessage([System.Net.Http.HttpMethod]::$Method, $Url)
    $response = $null
    $status = -1
    $body = ""
    try {
        $response = $httpClient.SendAsync($request).GetAwaiter().GetResult()
        $status = [int]$response.StatusCode
        $body = Read-ResponseBody -Response $response
    } catch {
        if ($null -ne $_.Exception.Response) {
            $response = $_.Exception.Response
            $status = [int]$response.StatusCode
            $body = Read-ResponseBody -Response $response
        } else {
            $body = $_.Exception.Message
        }
    } finally {
        if ($null -ne $response) {
            $response.Dispose()
        }
        $request.Dispose()
    }

    Write-Host "[check] $Method $Url -> $status" -ForegroundColor Cyan
    if (-not [string]::IsNullOrWhiteSpace($body)) {
        Write-Host $body
    }

    return ($status -eq $ExpectedStatus)
}

Write-Host "[verify-open-core] waiting for api-server readiness" -ForegroundColor Cyan
$ready = $false
for ($i = 0; $i -lt 30; $i++) {
    if ($job.State -ne "Running") {
        break
    }
    if (Invoke-Check -Method GET -Url "http://127.0.0.1:3000/api/v0/health" -ExpectedStatus 200) {
        $ready = $true
        break
    }
    Start-Sleep -Milliseconds 500
}

$failures = @()
if (-not $ready) {
    $failures += "open-core/health/readiness"
} else {
    if (-not (Invoke-Check -Method GET -Url "http://127.0.0.1:3000/api/v0/health" -ExpectedStatus 200)) { $failures += "open-core/health" }
    if (-not (Invoke-Check -Method GET -Url "http://127.0.0.1:3000/api/v0/snapshot/latest" -ExpectedStatus 200)) { $failures += "open-core/snapshot/latest" }
    if (-not (Invoke-Check -Method GET -Url "http://127.0.0.1:3000/api/v0/ideas/top?limit=3&offset=0" -ExpectedStatus 200)) { $failures += "open-core/ideas/top" }
    if (-not (Invoke-Check -Method GET -Url "http://127.0.0.1:3000/api/v0/idea/not-a-uuid" -ExpectedStatus 400)) { $failures += "open-core/idea/invalid-id" }
    if (-not (Invoke-Check -Method GET -Url "http://127.0.0.1:3000/api/v1/canonical/cycles/current" -ExpectedStatus 200)) { $failures += "open-core/canonical/cycles-current" }
    if (-not (Invoke-Check -Method GET -Url "http://127.0.0.1:3000/api/v1/canonical/tempo/status" -ExpectedStatus 200)) { $failures += "open-core/canonical/tempo-status" }

    if (-not (Invoke-Check -Method POST -Url "http://127.0.0.1:3000/api/v0/auth/login" -ExpectedStatus 404)) { $failures += "open-core/excluded/auth-login" }
    if (-not (Invoke-Check -Method GET -Url "http://127.0.0.1:3000/api/v0/auth/me" -ExpectedStatus 404)) { $failures += "open-core/excluded/auth-me" }
    if (-not (Invoke-Check -Method GET -Url "http://127.0.0.1:3000/api/v0/private/ideas" -ExpectedStatus 404)) { $failures += "open-core/excluded/private-ideas" }
    if (-not (Invoke-Check -Method GET -Url "http://127.0.0.1:3000/api/v0/me/orderings" -ExpectedStatus 404)) { $failures += "open-core/excluded/me-vines" }
    if (-not (Invoke-Check -Method POST -Url "http://127.0.0.1:3000/api/v1/canonical/ideas" -ExpectedStatus 404)) { $failures += "open-core/excluded/canonical-write-ideas" }
}

Write-Host "[verify-open-core] stopping api-server job" -ForegroundColor Cyan
try {
    Stop-Job $job -ErrorAction SilentlyContinue | Out-Null
    Receive-Job $job -Keep -ErrorAction SilentlyContinue | Out-Null
    Remove-Job $job -Force -ErrorAction SilentlyContinue
} catch { }

$httpClient.Dispose()
$httpHandler.Dispose()

if ($failures.Count -gt 0) {
    Write-Host "[verify-open-core] failures: $($failures -join ', ')" -ForegroundColor Red
    Write-Host "FAIL verify-open-core" -ForegroundColor Red
    exit 1
}

Write-Host "[verify-open-core] all checks passed" -ForegroundColor Green
Write-Host "PASS verify-open-core" -ForegroundColor Green
exit 0
