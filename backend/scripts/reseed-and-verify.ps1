param(
    [string]$OwnerIdentityId,
    [string]$OwnerUsername,
    [string]$OwnerDisplayTitle,
    [switch]$SkipReplayEquality
)

$ErrorActionPreference = "Stop"
Set-StrictMode -Version Latest

$scriptDir = Split-Path -Parent $MyInvocation.MyCommand.Path
$backendDir = (Resolve-Path (Join-Path $scriptDir "..")).Path
$repoRoot = (Resolve-Path (Join-Path $backendDir "..")).Path
$seedPath = Join-Path $repoRoot "seed\seed-data-v0.json"
$migrationDir = Join-Path $backendDir "migrations\postgres"
$devBootstrapPath = Join-Path $scriptDir "dev-bootstrap.ps1"
$envPath = Join-Path $backendDir ".env"

$env:Path = "$env:USERPROFILE\.cargo\bin;$env:Path"

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
    Write-RedactedHost -Text "[reseed] $Message" -ForegroundColor "Cyan"
}

function Write-Pass {
    param([string]$Message)
    Write-RedactedHost -Text "[PASS] $Message" -ForegroundColor "Green"
}

function Fail {
    param([string]$Message)
    Write-RedactedHost -Text "[FAIL] $Message" -ForegroundColor "Red"
    exit 1
}

function Get-EnvValue {
    param([string[]]$Keys)
    foreach ($key in $Keys) {
        $item = Get-Item -Path "Env:$key" -ErrorAction SilentlyContinue
        if ($item -and -not [string]::IsNullOrWhiteSpace($item.Value)) {
            return $item.Value
        }
    }
    return $null
}

function Ensure-UpperEnv {
    param(
        [string]$UpperKey,
        [string]$LowerKey
    )
    $upperValue = Get-EnvValue @($UpperKey)
    if ($upperValue) {
        return
    }
    $lowerValue = Get-EnvValue @($LowerKey)
    if ($lowerValue) {
        Set-Item -Path "Env:$UpperKey" -Value $lowerValue
        Write-Step "Mapped $LowerKey -> $UpperKey"
    }
}

function Invoke-External {
    param(
        [string]$File,
        [string[]]$ArgumentList,
        [string]$WorkingDirectory
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
        Fail "Command failed with exit code ${LASTEXITCODE}: $rendered"
    }
}

function Invoke-PsqlScalar {
    param([string]$Sql)
    Write-RedactedHost -Text ('>> psql DATABASE_URL -At -v ON_ERROR_STOP=1 -c "{0}"' -f $Sql) -ForegroundColor "DarkGray"
    $output = & psql $env:DATABASE_URL -At -v ON_ERROR_STOP=1 -c $Sql
    if ($LASTEXITCODE -ne 0) {
        Fail "psql scalar query failed"
    }

    if ($output -is [array]) {
        $lines = @($output | ForEach-Object { $_.ToString().Trim() } | Where-Object { $_ -ne "" })
        if ($lines.Count -eq 0) {
            return ""
        }
        return $lines[-1]
    }

    return $output.ToString().Trim()
}

function Invoke-PsqlFile {
    param([string]$FilePath)
    Write-RedactedHost -Text ('>> psql DATABASE_URL -v ON_ERROR_STOP=1 -f "{0}"' -f $FilePath) -ForegroundColor "DarkGray"
    & psql $env:DATABASE_URL -v ON_ERROR_STOP=1 -f $FilePath
    if ($LASTEXITCODE -ne 0) {
        Fail "psql file execution failed: $FilePath"
    }
}

function Invoke-PsqlStatement {
    param([string]$Sql)
    Write-RedactedHost -Text ('>> psql DATABASE_URL -v ON_ERROR_STOP=1 -c "{0}"' -f $Sql) -ForegroundColor "DarkGray"
    & psql $env:DATABASE_URL -v ON_ERROR_STOP=1 -c $Sql
    if ($LASTEXITCODE -ne 0) {
        Fail "psql statement failed"
    }
}

function SqlQuote {
    param([string]$Value)
    return "'" + $Value.Replace("'", "''") + "'"
}

function Assert-ScalarEquals {
    param(
        [string]$Name,
        [string]$Sql,
        [string]$Expected
    )

    $actual = Invoke-PsqlScalar $Sql
    if ($actual -ne $Expected) {
        Fail "$Name expected=$Expected actual=$actual"
    }

    Write-Pass "$Name (value=$actual)"
}

function Assert-ScalarGreaterThanZero {
    param(
        [string]$Name,
        [string]$Sql
    )

    $actualText = Invoke-PsqlScalar $Sql
    [int]$actualValue = 0
    if (-not [int]::TryParse($actualText, [ref]$actualValue)) {
        Fail "$Name expected integer > 0 actual=$actualText"
    }
    if ($actualValue -le 0) {
        Fail "$Name expected > 0 actual=$actualValue"
    }

    Write-Pass "$Name (value=$actualValue)"
}

function Assert-PsqlFails {
    param(
        [string]$Name,
        [string]$Sql
    )

    Write-RedactedHost -Text ('>> psql DATABASE_URL -v ON_ERROR_STOP=1 -c "{0}"' -f $Sql) -ForegroundColor "DarkGray"
    $failed = $false
    try {
        & psql $env:DATABASE_URL -v ON_ERROR_STOP=1 -c $Sql | Out-Null
        if ($LASTEXITCODE -ne 0) {
            $failed = $true
        }
    } catch {
        $failed = $true
    }

    if (-not $failed) {
        Fail "$Name expected SQL failure but command succeeded"
    }

    Write-Pass "$Name"
}

function Trigger-OwnerBootstrap {
    param(
        [string]$Username,
        [string]$IdentityId
    )

    $usernameSql = SqlQuote $Username
    $identitySql = "$(SqlQuote $IdentityId)::uuid"
    $accountCountSql = "SELECT COUNT(*) FROM accounts WHERE username = $usernameSql AND canonical_identity_id = $identitySql;"

    $existing = Invoke-PsqlScalar $accountCountSql
    if ($existing -eq "1") {
        Write-Pass "owner account already present before bootstrap trigger"
        return
    }

    if ((Get-EnvValue @("SEED_OWNER_BOOTSTRAP", "seed_owner_bootstrap")) -ne "1") {
        $env:SEED_OWNER_BOOTSTRAP = "1"
        Write-Step "Forced SEED_OWNER_BOOTSTRAP=1 for bootstrap run"
    }

    $runningApiServers = Get-Process -Name "api-server" -ErrorAction SilentlyContinue
    if ($runningApiServers) {
        Write-Step "Stopping existing api-server processes before bootstrap"
        foreach ($running in $runningApiServers) {
            Stop-Process -Id $running.Id -Force
        }
    }

    Write-Step "Building api-server binary for bootstrap trigger"
    Invoke-External "cargo" @("build", "-p", "api-server") $backendDir

    $apiExePath = Join-Path $backendDir "target\\debug\\api-server.exe"
    if (-not (Test-Path $apiExePath)) {
        Fail "api-server binary not found at $apiExePath"
    }

    Write-Step "Triggering owner bootstrap via api-server startup"
    $logBase = "seed-owner-bootstrap-" + [Guid]::NewGuid().ToString("N")
    $apiStdoutLogPath = Join-Path $env:TEMP ($logBase + ".stdout.log")
    $apiStderrLogPath = Join-Path $env:TEMP ($logBase + ".stderr.log")
    Write-RedactedHost -Text ">> $apiExePath" -ForegroundColor "DarkGray"

    $proc = Start-Process -FilePath $apiExePath -WorkingDirectory $backendDir -PassThru -RedirectStandardOutput $apiStdoutLogPath -RedirectStandardError $apiStderrLogPath

    try {
        $deadline = (Get-Date).AddMinutes(4)
        $bootstrapped = $false

        while ((Get-Date) -lt $deadline) {
            Start-Sleep -Milliseconds 750

            $count = Invoke-PsqlScalar $accountCountSql
            if ($count -eq "1") {
                $bootstrapped = $true
                break
            }

            if ($proc.HasExited) {
                break
            }
        }

        if (-not $bootstrapped) {
            $count = Invoke-PsqlScalar $accountCountSql
            if ($count -eq "1") {
                $bootstrapped = $true
            }
        }

        if (-not $bootstrapped) {
            if (Test-Path $apiStdoutLogPath) {
                Write-RedactedHost -Text "[reseed] api-server stdout tail:" -ForegroundColor "Yellow"
                Get-Content -Path $apiStdoutLogPath -Tail 80 | ForEach-Object {
                    Write-RedactedHost "  $_"
                }
            }
            if (Test-Path $apiStderrLogPath) {
                Write-RedactedHost -Text "[reseed] api-server stderr tail:" -ForegroundColor "Yellow"
                Get-Content -Path $apiStderrLogPath -Tail 80 | ForEach-Object {
                    Write-RedactedHost "  $_"
                }
            }

            if ($proc.HasExited) {
                Fail "owner bootstrap failed; api-server exited with code $($proc.ExitCode)"
            }

            Fail "owner bootstrap failed; expected account row was not created"
        }
    }
    finally {
        if ($proc -and -not $proc.HasExited) {
            Stop-Process -Id $proc.Id -Force
            Start-Sleep -Milliseconds 300
        }
    }

    Write-Pass "owner bootstrap created account binding"
}

function Dump-ApiLogs {
    param(
        [string]$StdoutPath,
        [string]$StderrPath
    )
    if (Test-Path $StdoutPath) {
        Write-RedactedHost -Text "[reseed] api-server stdout tail:" -ForegroundColor "Yellow"
        Get-Content -Path $StdoutPath -Tail 80 | ForEach-Object {
            Write-RedactedHost "  $_"
        }
    }
    if (Test-Path $StderrPath) {
        Write-RedactedHost -Text "[reseed] api-server stderr tail:" -ForegroundColor "Yellow"
        Get-Content -Path $StderrPath -Tail 80 | ForEach-Object {
            Write-RedactedHost "  $_"
        }
    }
}

function Assert-IdentityEndpointsHealthy {
    param(
        [string]$IdentityId,
        [string]$ExpectedTitle,
        [string[]]$ExpectedOrganizerTitles
    )

    $runningApiServers = Get-Process -Name "api-server" -ErrorAction SilentlyContinue
    if ($runningApiServers) {
        foreach ($running in $runningApiServers) {
            Stop-Process -Id $running.Id -Force
        }
    }

    Write-Step "Building api-server binary for endpoint checks"
    Invoke-External "cargo" @("build", "-p", "api-server") $backendDir
    $apiExePath = Join-Path $backendDir "target\\debug\\api-server.exe"
    if (-not (Test-Path $apiExePath)) {
        Fail "api-server binary not found at $apiExePath"
    }

    $logBase = "seed-endpoint-check-" + [Guid]::NewGuid().ToString("N")
    $stdoutPath = Join-Path $env:TEMP ($logBase + ".stdout.log")
    $stderrPath = Join-Path $env:TEMP ($logBase + ".stderr.log")
    Write-RedactedHost -Text ">> $apiExePath" -ForegroundColor "DarkGray"
    $proc = Start-Process -FilePath $apiExePath -WorkingDirectory $backendDir -PassThru -RedirectStandardOutput $stdoutPath -RedirectStandardError $stderrPath

    try {
        $ready = $false
        for ($attempt = 0; $attempt -lt 40; $attempt++) {
            Start-Sleep -Milliseconds 250
            if ($proc.HasExited) {
                break
            }
            try {
                $health = Invoke-RestMethod -Method Get -Uri "http://127.0.0.1:3000/api/v0/health"
                if ($health.ok -eq $true) {
                    $ready = $true
                    break
                }
            }
            catch {
                continue
            }
        }

        if (-not $ready) {
            Dump-ApiLogs -StdoutPath $stdoutPath -StderrPath $stderrPath
            if ($proc.HasExited) {
                Fail "api-server exited before health check (code=$($proc.ExitCode))"
            }
            Fail "api-server did not become healthy"
        }

        $detailUrl = "http://127.0.0.1:3000/api/v0/idea/$IdentityId"
        $neighborhoodUrl = "http://127.0.0.1:3000/api/v0/idea/$IdentityId/neighborhood?depth=1&limit_per_hop=50"
        Write-RedactedHost -Text ">> GET $detailUrl" -ForegroundColor "DarkGray"
        $detail = Invoke-RestMethod -Method Get -Uri $detailUrl
        Write-RedactedHost -Text ">> GET $neighborhoodUrl" -ForegroundColor "DarkGray"
        $neighborhood = Invoke-RestMethod -Method Get -Uri $neighborhoodUrl

        if ($detail.idea.idea_type -ne "identity") {
            Fail "identity detail endpoint returned non-identity idea_type: $($detail.idea.idea_type)"
        }
        if ($detail.idea.title -ne $ExpectedTitle) {
            Fail "identity detail endpoint title mismatch: expected '$ExpectedTitle' actual '$($detail.idea.title)'"
        }

        if ($neighborhood.central_idea.idea_id -ne $IdentityId) {
            Fail "neighborhood central_idea does not match identity id"
        }
        if ($neighborhood.central_idea.title -ne $ExpectedTitle) {
            Fail "neighborhood central_idea title mismatch"
        }

        $adjacentTitles = @($neighborhood.adjacent_ideas | ForEach-Object { $_.title })
        foreach ($expectedOrganizerTitle in $ExpectedOrganizerTitles) {
            if (-not ($adjacentTitles -contains $expectedOrganizerTitle)) {
                Fail "neighborhood missing organizer idea '$expectedOrganizerTitle'"
            }
        }

        Write-Pass "GET /api/v0/idea/$IdentityId returned identity detail"
        Write-Pass "GET /api/v0/idea/$IdentityId/neighborhood returned organizer neighborhood"
    }
    catch {
        Dump-ApiLogs -StdoutPath $stdoutPath -StderrPath $stderrPath
        Fail "identity endpoint checks failed: $($_.Exception.Message)"
    }
    finally {
        if ($proc -and -not $proc.HasExited) {
            Stop-Process -Id $proc.Id -Force
            Start-Sleep -Milliseconds 300
        }
    }
}

Write-Step "Repo root: $repoRoot"
Write-Step "Backend dir: $backendDir"

if (Test-Path $devBootstrapPath) {
    Write-Step "Using existing dev-bootstrap.ps1 (SkipMigrations)"
    & $devBootstrapPath -SkipMigrations
    if ($LASTEXITCODE -ne 0) {
        Fail "dev-bootstrap.ps1 failed"
    }
} else {
    Write-Step "dev-bootstrap.ps1 not found; loading .env minimally"
    if (-not (Test-Path $envPath)) {
        Fail "Missing .env at $envPath"
    }

    Get-Content $envPath | ForEach-Object {
        $line = $_.Trim()
        if ($line.Length -eq 0 -or $line.StartsWith("#")) {
            return
        }
        $parts = $line.Split("=", 2)
        if ($parts.Length -ne 2) {
            return
        }
        $key = $parts[0].Trim()
        $value = $parts[1].Trim()
        if ($key.Length -gt 0) {
            Set-Item -Path "Env:$key" -Value $value
        }
    }
}

if ([string]::IsNullOrWhiteSpace($env:DATABASE_URL)) {
    Fail "DATABASE_URL is not set"
}

try {
    $dbUri = [System.Uri]$env:DATABASE_URL
} catch {
    Fail "DATABASE_URL is not a valid URI"
}

if ($dbUri.UserInfo -match ":") {
    $userInfoParts = $dbUri.UserInfo.Split(":", 2)
    if ($userInfoParts.Length -eq 2 -and -not [string]::IsNullOrWhiteSpace($userInfoParts[1])) {
        Fail "DATABASE_URL contains a password. Use PGPASSFILE/pgpass.conf instead."
    }
}

Ensure-UpperEnv "SEED_OWNER_BOOTSTRAP" "seed_owner_bootstrap"
Ensure-UpperEnv "SEED_OWNER_USERNAME" "seed_owner_username"
Ensure-UpperEnv "SEED_OWNER_PASSWORD" "seed_owner_password"
Ensure-UpperEnv "SEED_OWNER_IDENTITY_ID" "seed_owner_identity_id"
Ensure-UpperEnv "SEED_OWNER_DISPLAY_TITLE" "seed_owner_display_title"

if (-not $OwnerIdentityId) {
    $OwnerIdentityId = Get-EnvValue @("SEED_OWNER_IDENTITY_ID", "seed_owner_identity_id")
}
if (-not $OwnerUsername) {
    $OwnerUsername = Get-EnvValue @("SEED_OWNER_USERNAME", "seed_owner_username")
}
if (-not $OwnerDisplayTitle) {
    $OwnerDisplayTitle = Get-EnvValue @("SEED_OWNER_DISPLAY_TITLE", "seed_owner_display_title")
}
if (-not $OwnerDisplayTitle) {
    if (-not [string]::IsNullOrWhiteSpace($OwnerUsername)) {
        $OwnerDisplayTitle = $OwnerUsername.Replace("-", " ")
    }
}
if (-not $OwnerDisplayTitle) {
    $OwnerDisplayTitle = "kind gulag dehl"
}

if ([string]::IsNullOrWhiteSpace($OwnerIdentityId)) {
    Fail "Missing owner identity id (SEED_OWNER_IDENTITY_ID or seed_owner_identity_id)"
}
if ([string]::IsNullOrWhiteSpace($OwnerUsername)) {
    Fail "Missing owner username (SEED_OWNER_USERNAME or seed_owner_username)"
}

Set-Item -Path "Env:SEED_OWNER_IDENTITY_ID" -Value $OwnerIdentityId
Set-Item -Path "Env:SEED_OWNER_USERNAME" -Value $OwnerUsername
Set-Item -Path "Env:SEED_OWNER_DISPLAY_TITLE" -Value $OwnerDisplayTitle

if (-not (Test-Path $seedPath)) {
    Fail "Seed file not found: $seedPath"
}
if (-not (Test-Path $migrationDir)) {
    Fail "Migration directory not found: $migrationDir"
}

Write-Step "Reset schema (destructive)"
Invoke-PsqlStatement "DROP SCHEMA public CASCADE; CREATE SCHEMA public;"

Write-Step "Apply migrations"
$migrationFiles = Get-ChildItem -Path $migrationDir -Filter "*.sql" | Sort-Object Name
if ($migrationFiles.Count -eq 0) {
    Fail "No migration files found in $migrationDir"
}

foreach ($migration in $migrationFiles) {
    Write-Step "Applying migration $($migration.Name)"
    Invoke-PsqlFile $migration.FullName
}

Write-Step "Import seed data"
Invoke-External "cargo" @("run", "-p", "seed-importer", "--", "--force", "--file", "..\\seed\\seed-data-v0.json") $backendDir

Write-Step "Build snapshot"
Invoke-External "cargo" @("run", "-p", "snapshot-builder") $backendDir

$ownerBootstrapEnabled = (Get-EnvValue @("SEED_OWNER_BOOTSTRAP", "seed_owner_bootstrap")) -eq "1"
$ownerPassword = Get-EnvValue @("SEED_OWNER_PASSWORD", "seed_owner_password")
$ownerPasswordPresent = -not [string]::IsNullOrWhiteSpace($ownerPassword)

if ($ownerBootstrapEnabled -and $ownerPasswordPresent) {
    Set-Item -Path "Env:SEED_OWNER_PASSWORD" -Value $ownerPassword
    Trigger-OwnerBootstrap -Username $OwnerUsername -IdentityId $OwnerIdentityId
} elseif ($ownerBootstrapEnabled) {
    Write-Step "SEED_OWNER_BOOTSTRAP=1 but SEED_OWNER_PASSWORD is empty; skipping account bootstrap."
} else {
    Write-Step "SEED_OWNER_BOOTSTRAP != 1; skipping account bootstrap."
}

Write-Step "Run verification queries"
$identitySql = "$(SqlQuote $OwnerIdentityId)::uuid"
$titleSql = SqlQuote $OwnerDisplayTitle
$usernameSql = SqlQuote $OwnerUsername
$organizerTitles = @(
    "$OwnerDisplayTitle's Mind Garden",
    "$OwnerDisplayTitle's Backyard of Ideas",
    "$OwnerDisplayTitle's Self Tree",
    "$OwnerDisplayTitle's Anthill",
    "$OwnerDisplayTitle's Saved Ideas"
)

Assert-ScalarEquals -Name "seed identity exists in identities_s0 with expected title" -Sql "SELECT COUNT(*) FROM identities_s0 WHERE identity_id = $identitySql AND title = $titleSql;" -Expected "1"

Assert-ScalarEquals -Name "identity idea row exists for seed identity" -Sql "SELECT COUNT(*) FROM ideas WHERE idea_id = $identitySql AND idea_type = 'identity' AND speaker_identity_id = $identitySql AND is_identity_idea = true AND underlying_identity_id = $identitySql;" -Expected "1"
Assert-ScalarEquals -Name "identity idea payload title matches expected title" -Sql "SELECT COUNT(*) FROM ideas i JOIN events e ON e.event_id = i.created_event_id WHERE i.idea_id = $identitySql AND e.payload_json->>'title' = $titleSql;" -Expected "1"
Assert-ScalarEquals -Name "identity idea payload_hash exists" -Sql "SELECT COUNT(*) FROM ideas i JOIN events e ON e.event_id = i.created_event_id WHERE i.idea_id = $identitySql AND COALESCE(e.payload_json->>'payload_hash','') <> '';" -Expected "1"

Assert-ScalarEquals -Name "exactly one identity idea for underlying_identity_id" -Sql "SELECT COUNT(*) FROM ideas WHERE underlying_identity_id = $identitySql AND is_identity_idea = true;" -Expected "1"

Assert-ScalarGreaterThanZero -Name "seeded idea events map speaker identity to expected title" -Sql "SELECT COUNT(*) FROM events e JOIN identities_s0 ident ON ident.identity_id = e.speaker_identity_id WHERE e.event_type = 'idea_create' AND e.speaker_identity_id = $identitySql AND ident.title = $titleSql;"

Assert-ScalarEquals -Name "seeded idea events have no speaker/title mismatch" -Sql "SELECT COUNT(*) FROM events e LEFT JOIN identities_s0 ident ON ident.identity_id = e.speaker_identity_id WHERE e.event_type = 'idea_create' AND e.speaker_identity_id = $identitySql AND (ident.title IS NULL OR ident.title <> $titleSql);" -Expected "0"

Assert-PsqlFails -Name "canonical ideas UPDATE is blocked for default app session" -Sql "UPDATE ideas SET created_block_height = created_block_height WHERE idea_id = (SELECT idea_id FROM ideas ORDER BY created_block_height, created_event_index LIMIT 1);"
Assert-PsqlFails -Name "canonical ideas DELETE is blocked for default app session" -Sql "DELETE FROM ideas WHERE idea_id = (SELECT idea_id FROM ideas ORDER BY created_block_height, created_event_index LIMIT 1);"

foreach ($organizerTitle in $organizerTitles) {
    $organizerTitleSql = SqlQuote $organizerTitle
    Assert-ScalarEquals -Name "organizer idea exists: $organizerTitle" -Sql "SELECT COUNT(*) FROM ideas i JOIN events e ON e.event_id = i.created_event_id WHERE i.speaker_identity_id = $identitySql AND i.idea_type = 'conceptual_idea' AND e.payload_json->>'title' = $organizerTitleSql;" -Expected "1"
    Assert-ScalarEquals -Name "membership has_space exists: $organizerTitle" -Sql "SELECT COUNT(*) FROM connections c JOIN ideas i ON i.idea_id = c.to_idea_id JOIN events e ON e.event_id = i.created_event_id WHERE c.from_idea_id = $identitySql AND c.connection_type = 'membership' AND c.usage = 'has_space' AND e.payload_json->>'title' = $organizerTitleSql;" -Expected "1"
    Assert-ScalarEquals -Name "membership space_of exists: $organizerTitle" -Sql "SELECT COUNT(*) FROM connections c JOIN ideas i ON i.idea_id = c.from_idea_id JOIN events e ON e.event_id = i.created_event_id WHERE c.to_idea_id = $identitySql AND c.connection_type = 'membership' AND c.usage = 'space_of' AND e.payload_json->>'title' = $organizerTitleSql;" -Expected "1"
}

if ($ownerBootstrapEnabled -and $ownerPasswordPresent) {
    Assert-ScalarEquals -Name "owner account canonical_identity_id matches seed owner identity" -Sql "SELECT COUNT(*) FROM accounts WHERE username = $usernameSql AND canonical_identity_id = $identitySql;" -Expected "1"
} else {
    Assert-ScalarEquals -Name "owner account row absent when seed owner password is missing" -Sql "SELECT COUNT(*) FROM accounts WHERE username = $usernameSql;" -Expected "0"
}

Assert-IdentityEndpointsHealthy -IdentityId $OwnerIdentityId -ExpectedTitle $OwnerDisplayTitle -ExpectedOrganizerTitles $organizerTitles

Write-Pass "Stage-0 reseed + verify pipeline completed"

if (-not $SkipReplayEquality) {
    $replayEqualityScript = Join-Path $scriptDir "verify-replay-equality.ps1"
    if (-not (Test-Path $replayEqualityScript)) {
        Fail "Replay equality script not found: $replayEqualityScript"
    }

    Write-Step "Run replay equality verification"
    $verifyParams = @{
        SkipBaselineReseed = $true
    }
    if (-not [string]::IsNullOrWhiteSpace($OwnerIdentityId)) {
        $verifyParams["OwnerIdentityId"] = $OwnerIdentityId
    }
    if (-not [string]::IsNullOrWhiteSpace($OwnerUsername)) {
        $verifyParams["OwnerUsername"] = $OwnerUsername
    }
    if (-not [string]::IsNullOrWhiteSpace($OwnerDisplayTitle)) {
        $verifyParams["OwnerDisplayTitle"] = $OwnerDisplayTitle
    }

    & $replayEqualityScript @verifyParams
    if ($LASTEXITCODE -ne 0) {
        Fail "Replay equality verification failed"
    }
    Write-Pass "Replay equality verification passed"
}

exit 0
