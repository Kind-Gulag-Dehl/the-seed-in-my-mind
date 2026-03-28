param(
    [string]$BaseUrl = "http://127.0.0.1:3000",
    [switch]$RunReplayEquality
)

$ErrorActionPreference = "Stop"
Set-StrictMode -Version Latest

$scriptDir = Split-Path -Parent $MyInvocation.MyCommand.Path
$backendDir = (Resolve-Path (Join-Path $scriptDir "..")).Path
$repoRoot = (Resolve-Path (Join-Path $backendDir "..")).Path

$script:Passed = 0
$script:Failed = 0
$script:Skipped = 0
$script:StartedServer = $false
$script:ApiProcess = $null
$script:ApiStdout = $null
$script:ApiStderr = $null
$script:HttpClient = $null

function Redact-SensitiveText {
    param(
        [AllowNull()]
        [string]$Text
    )

    if ($null -eq $Text) {
        return ""
    }

    $value = $Text
    $value = [regex]::Replace($value, '(?im)(authorization\s*:\s*bearer\s+)[^\s"''`r`n]+', '$1[REDACTED]')
    $value = [regex]::Replace($value, '(?im)(\bbearer\s+)[a-z0-9\-_\.=:+\/]{8,}', '$1[REDACTED]')
    $value = [regex]::Replace($value, '(?im)("token"\s*:\s*")[^"]+(")', '$1[REDACTED]$2')
    $value = [regex]::Replace($value, '(?im)(\b(token|secret|password|api[_-]?key)\b\s*[=:]\s*)[^\s,;]+', '$1[REDACTED]')
    $value = [regex]::Replace($value, '(?im)(database_url\s*=\s*)[^\s"''`r`n]+', '$1[REDACTED]')
    $value = [regex]::Replace($value, '(?im)(pgpassfile\s*=\s*)[^\s"''`r`n]+', '$1[REDACTED]')
    $value = [regex]::Replace($value, '(?im)\beyJ[a-zA-Z0-9_-]{8,}\.[a-zA-Z0-9_-]{8,}\.[a-zA-Z0-9_-]{8,}\b', '[REDACTED_JWT]')
    return $value
}

function Write-RedactedHost {
    param(
        [AllowNull()]
        [string]$Text,
        [string]$ForegroundColor
    )

    $safe = Redact-SensitiveText $Text
    if ([string]::IsNullOrWhiteSpace($ForegroundColor)) {
        Write-Host $safe
    } else {
        Write-Host $safe -ForegroundColor $ForegroundColor
    }
}

function Write-Step {
    param([string]$Message)
    Write-RedactedHost -Text "[stage1] $Message" -ForegroundColor "Cyan"
}

function Add-Pass {
    param([string]$Name)
    $script:Passed += 1
    Write-RedactedHost -Text "PASS $Name" -ForegroundColor "Green"
}

function Add-Skip {
    param(
        [string]$Name,
        [string]$Reason
    )
    $script:Skipped += 1
    Write-RedactedHost -Text "SKIP $Name ($Reason)" -ForegroundColor "Yellow"
}

function Fail-Required {
    param(
        [string]$Name,
        [string]$Reason
    )
    $script:Failed += 1
    Write-RedactedHost -Text "FAIL $Name ($Reason)" -ForegroundColor "Red"
    throw "REQUIRED_FAILURE: $Name"
}

function Show-Summary {
    Write-RedactedHost -Text "[stage1] summary" -ForegroundColor "Cyan"
    Write-RedactedHost -Text ("  passed={0} failed={1} skipped={2}" -f $script:Passed, $script:Failed, $script:Skipped)
    Write-RedactedHost -Text ("  base_url={0}" -f $BaseUrl)
    Write-RedactedHost -Text ("  api_server_started_by_script={0}" -f $script:StartedServer)
}

function Invoke-External {
    param(
        [string]$File,
        [string[]]$ArgumentList,
        [string]$WorkingDirectory,
        [string]$FailureName
    )

    $rendered = if ($ArgumentList -and $ArgumentList.Length -gt 0) {
        "$File $($ArgumentList -join ' ')"
    } else {
        $File
    }
    Write-RedactedHost -Text ">> $rendered" -ForegroundColor "DarkGray"

    if ([string]::IsNullOrWhiteSpace($WorkingDirectory)) {
        & $File @ArgumentList
    } else {
        Push-Location $WorkingDirectory
        try {
            & $File @ArgumentList
        }
        finally {
            Pop-Location
        }
    }

    if ($LASTEXITCODE -ne 0) {
        Fail-Required -Name $FailureName -Reason ("command failed exit={0}" -f $LASTEXITCODE)
    }
}

function Get-EnvValue {
    param([string[]]$Keys)
    foreach ($key in $Keys) {
        $value = [Environment]::GetEnvironmentVariable($key)
        if (-not [string]::IsNullOrWhiteSpace($value)) {
            return $value
        }
    }
    return $null
}

function Parse-ErrorCode {
    param([string]$Body)

    if ([string]::IsNullOrWhiteSpace($Body)) {
        return ""
    }

    try {
        $json = $Body | ConvertFrom-Json -ErrorAction Stop
        if ($null -ne $json.error_code -and -not [string]::IsNullOrWhiteSpace([string]$json.error_code)) {
            return [string]$json.error_code
        }
        if ($null -ne $json.code -and -not [string]::IsNullOrWhiteSpace([string]$json.code)) {
            return [string]$json.code
        }
    } catch {
    }

    return ""
}

function New-ApiResult {
    param(
        [int]$StatusCode,
        [string]$ErrorCode,
        [bool]$Ok
    )

    return @{
        status = $StatusCode
        error_code = $ErrorCode
        ok = $Ok
    }
}

function Invoke-ApiJson {
    param(
        [string]$Method,
        [string]$Url,
        [AllowNull()]
        [object]$Body,
        [string]$Token
    )

    $request = New-Object System.Net.Http.HttpRequestMessage($Method, $Url)
    if ($null -ne $Body) {
        $jsonBody = $Body | ConvertTo-Json -Depth 8
        $request.Content = New-Object System.Net.Http.StringContent($jsonBody, [System.Text.Encoding]::UTF8, "application/json")
    }

    if (-not [string]::IsNullOrWhiteSpace($Token)) {
        $request.Headers.Authorization = New-Object System.Net.Http.Headers.AuthenticationHeaderValue("Bearer", $Token)
    }

    try {
        $resp = $script:HttpClient.SendAsync($request).GetAwaiter().GetResult()
        $status = [int]$resp.StatusCode
        $body = $resp.Content.ReadAsStringAsync().GetAwaiter().GetResult()
        $code = Parse-ErrorCode -Body $body
        return (New-ApiResult -StatusCode $status -ErrorCode $code -Ok $true)
    } catch {
        $ex = $_.Exception
        $status = -1
        $body = ""

        $responseProp = $null
        if ($null -ne $ex) {
            $responseProp = $ex.PSObject.Properties["Response"]
        }
        if ($null -ne $responseProp -and $null -ne $responseProp.Value) {
            $response = $responseProp.Value
            try {
                $status = [int]$response.StatusCode
            } catch {
                $status = -1
            }
            try {
                $stream = $response.GetResponseStream()
                if ($null -ne $stream) {
                    $reader = New-Object System.IO.StreamReader($stream)
                    $body = $reader.ReadToEnd()
                    $reader.Dispose()
                }
            } catch {
                $body = ""
            }
        }

        $code = Parse-ErrorCode -Body $body
        return (New-ApiResult -StatusCode $status -ErrorCode $code -Ok $false)
    }
}

function Ensure-ApiReady {
    $healthUrl = "$BaseUrl/api/v0/health"
    $initial = Invoke-ApiJson -Method "GET" -Url $healthUrl -Body $null -Token ""
    if ($initial.status -eq 200) {
        Write-Step "api-server already running"
        return
    }

    Write-Step "api-server not healthy; starting local api-server"
    Invoke-External -File "cargo" -ArgumentList @("build", "-p", "api-server") -WorkingDirectory $backendDir -FailureName "build/api-server"

    $apiExe = Join-Path $backendDir "target\debug\api-server.exe"
    if (-not (Test-Path $apiExe)) {
        Fail-Required -Name "start/api-server" -Reason "api-server binary missing"
    }

    $logBase = "verify-stage1-" + [Guid]::NewGuid().ToString("N")
    $script:ApiStdout = Join-Path $env:TEMP ($logBase + ".stdout.log")
    $script:ApiStderr = Join-Path $env:TEMP ($logBase + ".stderr.log")

    $script:ApiProcess = Start-Process -FilePath $apiExe -WorkingDirectory $backendDir -PassThru -RedirectStandardOutput $script:ApiStdout -RedirectStandardError $script:ApiStderr
    $script:StartedServer = $true

    Invoke-External -File "powershell" -ArgumentList @("-ExecutionPolicy", "Bypass", "-File", (Join-Path $scriptDir "wait-health.ps1")) -WorkingDirectory $repoRoot -FailureName "wait-health"
}

function Stop-ApiIfStarted {
    if ($script:StartedServer -and $null -ne $script:ApiProcess) {
        try {
            if (-not $script:ApiProcess.HasExited) {
                Stop-Process -Id $script:ApiProcess.Id -Force
            }
        } catch {
        }
    }
}

function New-TestUsername {
    param([string]$Prefix)
    $stamp = [DateTimeOffset]::UtcNow.ToUnixTimeMilliseconds()
    return ("{0}_{1}" -f $Prefix, $stamp)
}

function Require-StatusIn {
    param(
        [hashtable]$Result,
        [int[]]$Allowed,
        [string]$Name
    )

    if ($Allowed -contains [int]$Result.status) {
        Add-Pass -Name $Name
        return
    }

    Fail-Required -Name $Name -Reason ("expected status in [{0}] actual={1} code={2}" -f ($Allowed -join ","), $Result.status, $Result.error_code)
}

function Require-StatusAndCode {
    param(
        [hashtable]$Result,
        [int]$Status,
        [string]$Code,
        [string]$Name
    )

    if ([int]$Result.status -ne $Status) {
        Fail-Required -Name $Name -Reason ("expected status={0} actual={1} code={2}" -f $Status, $Result.status, $Result.error_code)
    }

    if ([string]::IsNullOrWhiteSpace($Result.error_code)) {
        Fail-Required -Name $Name -Reason ("expected code={0} actual=(empty)" -f $Code)
    }

    if ([string]$Result.error_code -ne $Code) {
        Fail-Required -Name $Name -Reason ("expected code={0} actual={1}" -f $Code, $Result.error_code)
    }

    Add-Pass -Name $Name
}

try {
    Set-Location $repoRoot
    $env:Path = "$env:USERPROFILE\.cargo\bin;$env:Path"
    . (Join-Path $scriptDir "dev-bootstrap.ps1") -SkipMigrations
    Add-Type -AssemblyName System.Net.Http
    $script:HttpClient = New-Object System.Net.Http.HttpClient
    $script:HttpClient.Timeout = [TimeSpan]::FromSeconds(15)

    Write-Step "Reset baseline with reseed-and-verify (sequential; no parallel DB scripts)"
    Invoke-External -File "powershell" -ArgumentList @("-ExecutionPolicy", "Bypass", "-File", (Join-Path $scriptDir "reseed-and-verify.ps1"), "-SkipReplayEquality") -WorkingDirectory $repoRoot -FailureName "baseline/reseed-and-verify"
    Add-Pass -Name "baseline/reseed-and-verify"

    Ensure-ApiReady
    Add-Pass -Name "api/health"

    # A) Auth enforcement tests
    $noAuthCanonical = Invoke-ApiJson -Method "POST" -Url "$BaseUrl/api/v1/canonical/ideas" -Body @{
        idea_type = "conceptual_idea"
        title = "stage1 unauth canonical"
        sentence = "stage1 unauth canonical"
    } -Token ""
    Require-StatusIn -Result $noAuthCanonical -Allowed @(401, 403) -Name "auth/canonical-write-requires-auth"

    $noAuthPrivate = Invoke-ApiJson -Method "POST" -Url "$BaseUrl/api/v0/private/ideas" -Body @{
        title = "stage1 unauth private"
        sentence = "stage1 unauth private"
        paragraph = $null
        full = $null
    } -Token ""
    Require-StatusIn -Result $noAuthPrivate -Allowed @(401, 403) -Name "auth/private-write-requires-auth"

    # B) Session/auth flow tests
    $userA = New-TestUsername -Prefix "stage1_usera"
    $userB = New-TestUsername -Prefix "stage1_userb"
    $passwordA = "stage1-pass-a"
    $passwordB = "stage1-pass-b"

    $registerA = Invoke-ApiJson -Method "POST" -Url "$BaseUrl/api/v0/auth/register" -Body @{ username = $userA; password = $passwordA } -Token ""
    Require-StatusIn -Result $registerA -Allowed @(200) -Name "auth/register-userA"
    $loginA = Invoke-ApiJson -Method "POST" -Url "$BaseUrl/api/v0/auth/login" -Body @{ username = $userA; password = $passwordA } -Token ""
    Require-StatusIn -Result $loginA -Allowed @(200) -Name "auth/login-userA"

    $registerB = Invoke-ApiJson -Method "POST" -Url "$BaseUrl/api/v0/auth/register" -Body @{ username = $userB; password = $passwordB } -Token ""
    Require-StatusIn -Result $registerB -Allowed @(200) -Name "auth/register-userB"
    $loginB = Invoke-ApiJson -Method "POST" -Url "$BaseUrl/api/v0/auth/login" -Body @{ username = $userB; password = $passwordB } -Token ""
    Require-StatusIn -Result $loginB -Allowed @(200) -Name "auth/login-userB"

    # Parse tokens from login bodies quietly
    $tokenA = ""
    $tokenB = ""
    try {
        $loginAText = $script:HttpClient.PostAsync("$BaseUrl/api/v0/auth/login", (New-Object System.Net.Http.StringContent((@{ username = $userA; password = $passwordA } | ConvertTo-Json), [System.Text.Encoding]::UTF8, "application/json"))).GetAwaiter().GetResult().Content.ReadAsStringAsync().GetAwaiter().GetResult()
        $tokenA = [string](($loginAText | ConvertFrom-Json -ErrorAction Stop).token)
    } catch {
        Fail-Required -Name "auth/token-userA" -Reason "failed to parse login token"
    }
    try {
        $loginBText = $script:HttpClient.PostAsync("$BaseUrl/api/v0/auth/login", (New-Object System.Net.Http.StringContent((@{ username = $userB; password = $passwordB } | ConvertTo-Json), [System.Text.Encoding]::UTF8, "application/json"))).GetAwaiter().GetResult().Content.ReadAsStringAsync().GetAwaiter().GetResult()
        $tokenB = [string](($loginBText | ConvertFrom-Json -ErrorAction Stop).token)
    } catch {
        Fail-Required -Name "auth/token-userB" -Reason "failed to parse login token"
    }
    if ([string]::IsNullOrWhiteSpace($tokenA) -or [string]::IsNullOrWhiteSpace($tokenB)) {
        Fail-Required -Name "auth/tokens" -Reason "missing token(s)"
    }
    Add-Pass -Name "auth/token-capture-in-memory"

    # C) Role / permission tests
    $userBCanonical = Invoke-ApiJson -Method "POST" -Url "$BaseUrl/api/v1/canonical/ideas" -Body @{
        idea_type = "conceptual_idea"
        title = "stage1 userb canonical"
        sentence = "stage1 userb canonical"
    } -Token $tokenB
    Require-StatusIn -Result $userBCanonical -Allowed @(403) -Name "auth/non-writer-canonical-forbidden"

    $ownerUser = Get-EnvValue @("SEED_OWNER_USERNAME", "seed_owner_username")
    $ownerPass = Get-EnvValue @("SEED_OWNER_PASSWORD", "seed_owner_password")
    $ownerToken = ""
    $ownerIdentityId = ""
    $writerToken = ""

    if (-not [string]::IsNullOrWhiteSpace($ownerUser) -and -not [string]::IsNullOrWhiteSpace($ownerPass)) {
        $ownerLoginResult = Invoke-ApiJson -Method "POST" -Url "$BaseUrl/api/v0/auth/login" -Body @{ username = $ownerUser; password = $ownerPass } -Token ""
        if ($ownerLoginResult.status -eq 200) {
            try {
                $ownerLoginRaw = $script:HttpClient.PostAsync("$BaseUrl/api/v0/auth/login", (New-Object System.Net.Http.StringContent((@{ username = $ownerUser; password = $ownerPass } | ConvertTo-Json), [System.Text.Encoding]::UTF8, "application/json"))).GetAwaiter().GetResult().Content.ReadAsStringAsync().GetAwaiter().GetResult()
                $ownerLoginJson = $ownerLoginRaw | ConvertFrom-Json -ErrorAction Stop
                $ownerToken = [string]$ownerLoginJson.token
                $ownerIdentityId = [string]$ownerLoginJson.identity_id
            } catch {
                $ownerToken = ""
                $ownerIdentityId = ""
            }
        }
    }

    if (-not [string]::IsNullOrWhiteSpace($ownerToken)) {
        $ownerCanonical = Invoke-ApiJson -Method "POST" -Url "$BaseUrl/api/v1/canonical/ideas" -Body @{
            idea_type = "conceptual_idea"
            title = "stage1 owner canonical"
            sentence = "stage1 owner canonical"
        } -Token $ownerToken

        if ($ownerCanonical.status -in @(200, 201, 409)) {
            $writerToken = $ownerToken
            Add-Pass -Name "auth/writer-canonical-success"
        } elseif ($ownerCanonical.status -in @(401, 403)) {
            Add-Skip -Name "auth/writer-canonical-success" -Reason "owner identity is not configured as canonical writer"
        } else {
            Fail-Required -Name "auth/writer-canonical-success" -Reason ("unexpected status={0} code={1}" -f $ownerCanonical.status, $ownerCanonical.error_code)
        }
    } else {
        Add-Skip -Name "auth/writer-canonical-success" -Reason "owner credentials not configured"
    }

    $grantTargetIdentity = if (-not [string]::IsNullOrWhiteSpace($ownerIdentityId)) { $ownerIdentityId } else { "00000000-0000-7000-8000-00000000d999" }
    $nonVerifierGrant = Invoke-ApiJson -Method "POST" -Url "$BaseUrl/api/v1/canonical/verifier/grants" -Body @{
        identity_id = $grantTargetIdentity
        canonical_writer_level = "1"
        email_verified = $true
    } -Token $tokenB
    if ($nonVerifierGrant.status -in @(401, 403)) {
        Add-Pass -Name "auth/non-verifier-grant-forbidden"
    } elseif ($nonVerifierGrant.status -eq 400 -and $nonVerifierGrant.error_code -eq "invalid_request") {
        Add-Skip -Name "auth/non-verifier-grant-forbidden" -Reason "grant route rejected identity payload before permission check"
    } else {
        Fail-Required -Name "auth/non-verifier-grant-forbidden" -Reason ("unexpected status={0} code={1}" -f $nonVerifierGrant.status, $nonVerifierGrant.error_code)
    }

    if (-not [string]::IsNullOrWhiteSpace($ownerToken) -and -not [string]::IsNullOrWhiteSpace($ownerIdentityId)) {
        $ownerGrant = Invoke-ApiJson -Method "POST" -Url "$BaseUrl/api/v1/canonical/verifier/grants" -Body @{
            identity_id = $ownerIdentityId
            canonical_writer_level = "1"
            email_verified = $true
        } -Token $ownerToken

        if ($ownerGrant.status -in @(200, 201)) {
            Add-Pass -Name "auth/verifier-grant-by-owner"
            if ([string]::IsNullOrWhiteSpace($writerToken)) {
                $ownerCanonicalAfterGrant = Invoke-ApiJson -Method "POST" -Url "$BaseUrl/api/v1/canonical/ideas" -Body @{
                    idea_type = "conceptual_idea"
                    title = "stage1 owner canonical after grant"
                    sentence = "stage1 owner canonical after grant"
                } -Token $ownerToken
                if ($ownerCanonicalAfterGrant.status -in @(200, 201, 409)) {
                    $writerToken = $ownerToken
                    Add-Pass -Name "auth/writer-canonical-success-after-grant"
                } else {
                    Add-Skip -Name "auth/writer-canonical-success-after-grant" -Reason ("owner still not writable status={0}" -f $ownerCanonicalAfterGrant.status)
                }
            }
        } elseif ($ownerGrant.status -eq 400 -and $ownerGrant.error_code -eq "invalid_request") {
            Add-Skip -Name "auth/verifier-grant-by-owner" -Reason "owner verifier grant payload rejected by current local policy"
        } elseif ($ownerGrant.status -in @(401, 403)) {
            Add-Skip -Name "auth/verifier-grant-by-owner" -Reason "no verifier identity configured"
        } else {
            Fail-Required -Name "auth/verifier-grant-by-owner" -Reason ("unexpected status={0} code={1}" -f $ownerGrant.status, $ownerGrant.error_code)
        }
    } elseif (-not [string]::IsNullOrWhiteSpace($ownerToken)) {
        Add-Skip -Name "auth/verifier-grant-by-owner" -Reason "owner login response did not expose identity_id"
    } else {
        Add-Skip -Name "auth/verifier-grant-by-owner" -Reason "owner credentials not configured"
    }

    if ([string]::IsNullOrWhiteSpace($writerToken)) {
        Fail-Required -Name "validation/writer-token" -Reason "writer token unavailable for required Stage-1 ingress checks"
    }

    # D) Ingress validation tests (deterministic required)
    $tooLongTitle = ("x" * 51)
    $tooLong = Invoke-ApiJson -Method "POST" -Url "$BaseUrl/api/v1/canonical/ideas" -Body @{
        idea_type = "conceptual_idea"
        title = $tooLongTitle
        sentence = "stage1 length guard"
    } -Token $writerToken
    Require-StatusAndCode -Result $tooLong -Status 400 -Code "invalid_field_length" -Name "validation/canonical-title-max-50"

    $secretPattern = "-----BEGIN PRIVATE KEY-----"
    $secretCheck = Invoke-ApiJson -Method "POST" -Url "$BaseUrl/api/v1/canonical/ideas" -Body @{
        idea_type = "conceptual_idea"
        title = ("synthetic-marker-" + $secretPattern)
        sentence = "stage1 secret guard"
    } -Token $writerToken
    Require-StatusAndCode -Result $secretCheck -Status 400 -Code "secret_detected" -Name "validation/canonical-secret-screening"

    # E) Optional replay equality
    $replayScript = Join-Path $scriptDir "verify-replay-equality.ps1"
    if ($RunReplayEquality -and (Test-Path $replayScript)) {
        Invoke-External -File "powershell" -ArgumentList @("-ExecutionPolicy", "Bypass", "-File", $replayScript, "-SkipBaselineReseed") -WorkingDirectory $repoRoot -FailureName "optional/replay-equality"
        Add-Pass -Name "optional/replay-equality"
    } else {
        Add-Skip -Name "optional/replay-equality" -Reason "not requested or script unavailable"
    }

    Show-Summary
    if ($script:Failed -gt 0) {
        Write-RedactedHost -Text "FAIL verify-stage1" -ForegroundColor "Red"
        exit 1
    }

    Write-RedactedHost -Text "PASS verify-stage1" -ForegroundColor "Green"
    exit 0
} catch {
    Show-Summary
    Write-RedactedHost -Text "FAIL verify-stage1" -ForegroundColor "Red"
    exit 1
} finally {
    if ($script:HttpClient) {
        $script:HttpClient.Dispose()
    }
    Stop-ApiIfStarted
}
