param(
    [string]$OwnerIdentityId,
    [string]$OwnerUsername,
    [string]$OwnerDisplayTitle,
    [switch]$SkipBaselineReseed
)

$ErrorActionPreference = "Stop"
Set-StrictMode -Version Latest

$scriptDir = Split-Path -Parent $MyInvocation.MyCommand.Path
$backendDir = (Resolve-Path (Join-Path $scriptDir "..")).Path
$repoRoot = (Resolve-Path (Join-Path $backendDir "..")).Path
$envPath = Join-Path $backendDir ".env"
$reseedScript = Join-Path $scriptDir "reseed-and-verify.ps1"
$auditRoot = Join-Path $repoRoot "audit_out"
$baselineDir = Join-Path $auditRoot "baseline"
$replayDir = Join-Path $auditRoot "replay"
$diffSummaryPath = Join-Path $auditRoot "diff-summary.txt"

$env:Path = "$env:USERPROFILE\.cargo\bin;$env:Path"

try {
    Add-Type -AssemblyName System.Net.Http -ErrorAction Stop
} catch {
}

function Write-Step {
    param([string]$Message)
    Write-Host "[replay-equality] $Message" -ForegroundColor Cyan
}

function Write-Pass {
    param([string]$Message)
    Write-Host "[PASS] $Message" -ForegroundColor Green
}

function Fail {
    param([string]$Message)
    Write-Host "[FAIL] $Message" -ForegroundColor Red
    exit 1
}

function Write-TextFile {
    param(
        [string]$Path,
        [string]$Content
    )
    $utf8NoBom = New-Object System.Text.UTF8Encoding($false)
    [System.IO.File]::WriteAllText($Path, $Content, $utf8NoBom)
}

function Ensure-UpperEnv {
    param(
        [string]$UpperKey,
        [string]$LowerKey
    )

    $upperValue = (Get-Item -Path "Env:$UpperKey" -ErrorAction SilentlyContinue).Value
    if (-not [string]::IsNullOrWhiteSpace($upperValue)) {
        return
    }

    $lowerValue = (Get-Item -Path "Env:$LowerKey" -ErrorAction SilentlyContinue).Value
    if (-not [string]::IsNullOrWhiteSpace($lowerValue)) {
        Set-Item -Path "Env:$UpperKey" -Value $lowerValue
    }
}

function Ensure-Environment {
    if ([string]::IsNullOrWhiteSpace($env:DATABASE_URL) -and (Test-Path $envPath)) {
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
            if ($key.Length -eq 0) {
                return
            }

            Set-Item -Path "Env:$key" -Value $value
        }
    }

    Ensure-UpperEnv -UpperKey "DATABASE_URL" -LowerKey "database_url"
    Ensure-UpperEnv -UpperKey "PGPASSFILE" -LowerKey "pgpassfile"

    if ([string]::IsNullOrWhiteSpace($env:DATABASE_URL)) {
        Fail "DATABASE_URL is not set. Provide backend/.env or environment overrides."
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
    Write-Host ">> $rendered" -ForegroundColor DarkGray

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

function Get-ReseedArguments {
    $reseedArgs = @{
        SkipReplayEquality = $true
    }
    if (-not [string]::IsNullOrWhiteSpace($OwnerIdentityId)) {
        $reseedArgs["OwnerIdentityId"] = $OwnerIdentityId
    }
    if (-not [string]::IsNullOrWhiteSpace($OwnerUsername)) {
        $reseedArgs["OwnerUsername"] = $OwnerUsername
    }
    if (-not [string]::IsNullOrWhiteSpace($OwnerDisplayTitle)) {
        $reseedArgs["OwnerDisplayTitle"] = $OwnerDisplayTitle
    }
    return $reseedArgs
}

function Run-ReseedPipeline {
    Write-Step "Run reseed-and-verify baseline pipeline"
    $reseedArgs = Get-ReseedArguments
    & $reseedScript @reseedArgs
    if ($LASTEXITCODE -ne 0) {
        Fail "reseed-and-verify failed"
    }
}

function Start-ApiServer {
    param(
        [string]$BackendDir,
        [string]$DatabaseUrl,
        [string]$PgPassFile
    )

    try { taskkill /IM api-server.exe /F 2>$null | Out-Null } catch { }

    $apiExePath = Join-Path $BackendDir "target\\debug\\api-server.exe"
    if (-not (Test-Path $apiExePath)) {
        Fail "api-server binary not found at $apiExePath"
    }

    if (-not [string]::IsNullOrWhiteSpace($DatabaseUrl)) {
        Set-Item -Path "Env:DATABASE_URL" -Value $DatabaseUrl
    }
    if (-not [string]::IsNullOrWhiteSpace($PgPassFile)) {
        Set-Item -Path "Env:PGPASSFILE" -Value $PgPassFile
    }

    $logBase = "replay-equality-" + [Guid]::NewGuid().ToString("N")
    $stdoutPath = Join-Path $env:TEMP ($logBase + ".stdout.log")
    $stderrPath = Join-Path $env:TEMP ($logBase + ".stderr.log")

    $proc = Start-Process -FilePath $apiExePath -WorkingDirectory $BackendDir -PassThru -RedirectStandardOutput $stdoutPath -RedirectStandardError $stderrPath

    $ready = $false
    for ($attempt = 0; $attempt -lt 60; $attempt++) {
        if ($proc.HasExited) {
            break
        }
        try {
            $resp = $script:HttpClient.GetAsync("http://127.0.0.1:3000/api/v0/health").GetAwaiter().GetResult()
            if ([int]$resp.StatusCode -eq 200) {
                $ready = $true
                break
            }
        } catch {
        }
        Start-Sleep -Milliseconds 250
    }

    if (-not $ready) {
        if (Test-Path $stdoutPath) {
            Write-Host "[replay-equality] api-server stdout tail:" -ForegroundColor Yellow
            Get-Content -Path $stdoutPath -Tail 80 | ForEach-Object { Write-Host "  $_" }
        }
        if (Test-Path $stderrPath) {
            Write-Host "[replay-equality] api-server stderr tail:" -ForegroundColor Yellow
            Get-Content -Path $stderrPath -Tail 80 | ForEach-Object { Write-Host "  $_" }
        }
        if ($proc.HasExited) {
            Fail "api-server exited before readiness (code=$($proc.ExitCode))"
        }
        try { Stop-Process -Id $proc.Id -Force } catch { }
        Fail "api-server did not become ready"
    }

    return @{
        process = $proc
        stdout = $stdoutPath
        stderr = $stderrPath
    }
}

function Stop-ApiServer {
    param([hashtable]$Handle)

    if ($null -eq $Handle) {
        return
    }

    try {
        if ($Handle.process -and -not $Handle.process.HasExited) {
            Stop-Process -Id $Handle.process.Id -Force
            Start-Sleep -Milliseconds 300
        }
    } catch {
    }
}

function Invoke-HttpGetRaw {
    param(
        [string]$Url,
        [int]$ExpectedStatus = 200,
        [string]$BearerToken = ""
    )

    $request = New-Object System.Net.Http.HttpRequestMessage("GET", $Url)
    if (-not [string]::IsNullOrWhiteSpace($BearerToken)) {
        $request.Headers.Authorization = New-Object System.Net.Http.Headers.AuthenticationHeaderValue("Bearer", $BearerToken)
    }

    $response = $script:HttpClient.SendAsync($request).GetAwaiter().GetResult()
    $status = [int]$response.StatusCode
    $body = $response.Content.ReadAsStringAsync().GetAwaiter().GetResult()
    Write-Host "[capture] GET $Url -> $status" -ForegroundColor DarkGray
    if ($status -ne $ExpectedStatus) {
        Fail "Unexpected status for $Url expected=$ExpectedStatus actual=$status body=$body"
    }
    return $body
}

function Invoke-HttpJson {
    param(
        [string]$Method,
        [string]$Url,
        [object]$Body,
        [int]$ExpectedStatus,
        [string]$BearerToken
    )

    $jsonBody = $Body | ConvertTo-Json -Depth 8
    $request = New-Object System.Net.Http.HttpRequestMessage($Method, $Url)
    $request.Content = New-Object System.Net.Http.StringContent($jsonBody, [System.Text.Encoding]::UTF8, "application/json")
    if (-not [string]::IsNullOrWhiteSpace($BearerToken)) {
        $request.Headers.Authorization = New-Object System.Net.Http.Headers.AuthenticationHeaderValue("Bearer", $BearerToken)
    }

    $response = $script:HttpClient.SendAsync($request).GetAwaiter().GetResult()
    $status = [int]$response.StatusCode
    $bodyText = $response.Content.ReadAsStringAsync().GetAwaiter().GetResult()
    Write-Host "[capture] $Method $Url -> $status" -ForegroundColor DarkGray
    if ($status -ne $ExpectedStatus) {
        Fail "Unexpected status for $Method $Url expected=$ExpectedStatus actual=$status body=$bodyText"
    }
    return ($bodyText | ConvertFrom-Json -ErrorAction Stop)
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

function Resolve-SeedVerifierToken {
    $ownerUsername = Get-EnvValue @("SEED_OWNER_USERNAME", "seed_owner_username")
    $ownerPassword = Get-EnvValue @("SEED_OWNER_PASSWORD", "seed_owner_password")
    if ([string]::IsNullOrWhiteSpace($ownerUsername) -or [string]::IsNullOrWhiteSpace($ownerPassword)) {
        return $null
    }

    $login = Invoke-HttpJson -Method "POST" -Url "http://127.0.0.1:3000/api/v0/auth/login" -Body @{
        username = $ownerUsername
        password = $ownerPassword
    } -ExpectedStatus 200 -BearerToken ""
    return [string]$login.token
}

function Invoke-PsqlStatement {
    param(
        [string]$DatabaseUrl,
        [string]$Sql
    )

    & psql $DatabaseUrl -v ON_ERROR_STOP=1 -c $Sql | Out-Null
    if ($LASTEXITCODE -ne 0) {
        Fail "psql statement failed: $Sql"
    }
}

function Invoke-PsqlScalar {
    param(
        [string]$DatabaseUrl,
        [string]$Sql
    )

    $raw = & psql $DatabaseUrl -At -v ON_ERROR_STOP=1 -c $Sql
    if ($LASTEXITCODE -ne 0) {
        Fail "psql scalar failed: $Sql"
    }
    if ($raw -is [array]) {
        $raw = $raw[-1]
    }
    if ($null -eq $raw) {
        return ""
    }
    $value = [string]$raw
    if ($null -eq $value) {
        return ""
    }
    return $value.Trim()
}

function Insert-ForcedCycleClose {
    param(
        [string]$DatabaseUrl,
        [string]$NoopEventId,
        [string]$CycleCloseEventId,
        [string]$NoopSpeakerIdentityId
    )

    [int64]$blockHeight = 0
    [void][int64]::TryParse((Invoke-PsqlScalar -DatabaseUrl $DatabaseUrl -Sql "SELECT COALESCE(MAX(block_height), 0)::bigint FROM blocks;"), [ref]$blockHeight)
    [int]$noopIndex = 0
    [void][int]::TryParse((Invoke-PsqlScalar -DatabaseUrl $DatabaseUrl -Sql "SELECT COALESCE(MAX(event_index), -1)::int + 1 FROM events WHERE block_height = $blockHeight;"), [ref]$noopIndex)
    $closeIndex = $noopIndex + 1

    [int64]$cycleIndex = 0
    [void][int64]::TryParse((Invoke-PsqlScalar -DatabaseUrl $DatabaseUrl -Sql "SELECT COALESCE(MAX(cycle_index), -1)::bigint + 1 FROM cycle_boundaries;"), [ref]$cycleIndex)

    $closureHeight = $blockHeight
    $nextBlockHeight = $closureHeight + 1

    Invoke-PsqlStatement -DatabaseUrl $DatabaseUrl -Sql @"
INSERT INTO events (block_height, event_index, event_id, event_type, speaker_identity_id, payload_json, signature)
VALUES (
  $blockHeight,
  $noopIndex,
  '$NoopEventId'::uuid,
  'noop',
  '$NoopSpeakerIdentityId'::uuid,
  '{}'::jsonb,
  NULL
);
INSERT INTO tempo_predicates (block_height, event_index, cycle_age_ge_dmin, cycle_age_ge_dmax, constrained_mode, record_only_mode)
VALUES ($blockHeight, $noopIndex, false, false, false, false)
ON CONFLICT (block_height, event_index) DO UPDATE SET
  cycle_age_ge_dmin = EXCLUDED.cycle_age_ge_dmin,
  cycle_age_ge_dmax = EXCLUDED.cycle_age_ge_dmax,
  constrained_mode = EXCLUDED.constrained_mode,
  record_only_mode = EXCLUDED.record_only_mode;
INSERT INTO events (block_height, event_index, event_id, event_type, speaker_identity_id, payload_json, signature)
VALUES (
  $blockHeight,
  $closeIndex,
  '$CycleCloseEventId'::uuid,
  'cycle_close',
  'ffffffff-ffff-7fff-bfff-ffffffffffff'::uuid,
  jsonb_build_object(
    'cycle_index', $cycleIndex,
    'closure_kind', 'forced',
    'forced_seal', true,
    'closure_boundary_ref', jsonb_build_object('block_height', $closureHeight)
  ),
  NULL
);
INSERT INTO tempo_predicates (block_height, event_index, cycle_age_ge_dmin, cycle_age_ge_dmax, constrained_mode, record_only_mode)
VALUES ($blockHeight, $closeIndex, true, true, false, false)
ON CONFLICT (block_height, event_index) DO UPDATE SET
  cycle_age_ge_dmin = EXCLUDED.cycle_age_ge_dmin,
  cycle_age_ge_dmax = EXCLUDED.cycle_age_ge_dmax,
  constrained_mode = EXCLUDED.constrained_mode,
  record_only_mode = EXCLUDED.record_only_mode;
INSERT INTO cycle_boundaries (
  cycle_index,
  closure_kind,
  forced_seal,
  closure_block_height,
  source_block_height,
  source_event_index,
  source_event_id
) VALUES (
  $cycleIndex,
  1,
  true,
  $closureHeight,
  $blockHeight,
  $closeIndex,
  '$CycleCloseEventId'::uuid
);
INSERT INTO blocks (block_height, block_hash, prev_block_hash)
VALUES (
  $nextBlockHeight,
  to_hex($nextBlockHeight),
  CASE WHEN $nextBlockHeight > 0 THEN to_hex($nextBlockHeight - 1) ELSE NULL END
)
ON CONFLICT (block_height) DO NOTHING;
"@
}

function Run-CanonicalWriteScenario {
    param(
        [string]$DatabaseUrl
    )

    $writerUsername = "stage1_replay_writer"
    $writerPassword = "stage1-replay-writer-password"
    $writerIdentityId = "00000000-0000-7000-8000-00000000d901"
    $writerTitle = "stage1 replay writer"
    $writerGrantEventId = "00000000-0000-7000-8000-00000000f104"
    $seedVerifierToken = Resolve-SeedVerifierToken
    if ([string]::IsNullOrWhiteSpace($seedVerifierToken)) {
        Fail "seed verifier credentials are missing; cannot issue canonical writer grants"
    }

    $ideaAId = "00000000-0000-7000-8000-00000000e101"
    $ideaBId = "00000000-0000-7000-8000-00000000e102"
    $argumentIdeaId = "00000000-0000-7000-8000-00000000e103"
    $ideaAEventId = "00000000-0000-7000-8000-00000000f101"
    $ideaBEventId = "00000000-0000-7000-8000-00000000f102"
    $argumentIdeaEventId = "00000000-0000-7000-8000-00000000f103"
    $connectionId = "00000000-0000-7000-8000-00000000e201"
    $connectionEventId = "00000000-0000-7000-8000-00000000f201"
    $challengeId = "00000000-0000-7000-8000-00000000c901"
    $challengeEventId = "00000000-0000-7000-8000-00000000f301"
    $argumentConnectionId = "00000000-0000-7000-8000-00000000e301"
    $argumentConnectionEventId = "00000000-0000-7000-8000-00000000f302"

    $register = Invoke-HttpJson -Method "POST" -Url "http://127.0.0.1:3000/api/v0/auth/register" -Body @{
        username = $writerUsername
        password = $writerPassword
    } -ExpectedStatus 200 -BearerToken ""

    $writerAccountId = [string]$register.account_id
    $writerToken = [string]$register.token
    if ([string]::IsNullOrWhiteSpace($writerAccountId) -or [string]::IsNullOrWhiteSpace($writerToken)) {
        Fail "writer bootstrap account response missing account_id/token"
    }

    Invoke-HttpJson -Method "POST" -Url "http://127.0.0.1:3000/api/v1/canonical/ideas" -Body @{
        idea_type = "conceptual_idea"
        title = "unauthorized"
        sentence = "must fail"
    } -ExpectedStatus 401 -BearerToken "" | Out-Null

    Invoke-PsqlStatement -DatabaseUrl $DatabaseUrl -Sql "UPDATE accounts SET canonical_identity_id = '$writerIdentityId'::uuid WHERE account_id = '$writerAccountId'::uuid;"
    Invoke-PsqlStatement -DatabaseUrl $DatabaseUrl -Sql "INSERT INTO identities_s0 (identity_id, title, created_event_id) VALUES ('$writerIdentityId'::uuid, '$writerTitle', '$writerIdentityId'::uuid) ON CONFLICT (identity_id) DO NOTHING;"
    Invoke-HttpJson -Method "POST" -Url "http://127.0.0.1:3000/api/v1/canonical/verifier/grants" -Body @{
        identity_id = $writerIdentityId
        canonical_writer_level = "1"
        email_verified = $true
        event_id = $writerGrantEventId
    } -ExpectedStatus 200 -BearerToken $seedVerifierToken | Out-Null
    $framingRepresentationId = Invoke-PsqlScalar -DatabaseUrl $DatabaseUrl -Sql "SELECT representation_id::text FROM representations ORDER BY created_block_height, created_event_index LIMIT 1;"
    $useFallbackFramingRepresentation = $false
    if ([string]::IsNullOrWhiteSpace($framingRepresentationId)) {
        $framingRepresentationId = "00000000-0000-7000-8000-00000000fafe"
        $useFallbackFramingRepresentation = $true
        Write-Host "[replay-equality] no framing representation row found; using deterministic fallback id $framingRepresentationId" -ForegroundColor Yellow
    }

    Invoke-HttpJson -Method "POST" -Url "http://127.0.0.1:3000/api/v1/canonical/ideas" -Body @{
        idea_id = $ideaAId
        event_id = $ideaAEventId
        idea_type = "conceptual_idea"
        title = "stage1 replay idea a"
        sentence = "deterministic replay idea a"
        paragraph = $null
        full = $null
    } -ExpectedStatus 200 -BearerToken $writerToken | Out-Null

    Invoke-HttpJson -Method "POST" -Url "http://127.0.0.1:3000/api/v1/canonical/ideas" -Body @{
        idea_id = $ideaBId
        event_id = $ideaBEventId
        idea_type = "conceptual_idea"
        title = "stage1 replay idea b"
        sentence = "deterministic replay idea b"
        paragraph = $null
        full = $null
    } -ExpectedStatus 200 -BearerToken $writerToken | Out-Null

    Invoke-HttpJson -Method "POST" -Url "http://127.0.0.1:3000/api/v1/canonical/ideas" -Body @{
        idea_id = $argumentIdeaId
        event_id = $argumentIdeaEventId
        idea_type = "conceptual_idea"
        title = "stage1 replay challenge argument"
        sentence = "deterministic replay challenge argument"
        paragraph = $null
        full = $null
    } -ExpectedStatus 200 -BearerToken $writerToken | Out-Null

    Invoke-HttpJson -Method "POST" -Url "http://127.0.0.1:3000/api/v1/canonical/connections" -Body @{
        connection_id = $connectionId
        event_id = $connectionEventId
        from_idea_id = $ideaAId
        to_idea_id = $ideaBId
        connection_type = "same_as"
        usage = $null
        axis = $null
        timeframe = $null
        scope = $null
    } -ExpectedStatus 200 -BearerToken $writerToken | Out-Null

    if ($useFallbackFramingRepresentation) {
        Invoke-PsqlStatement -DatabaseUrl $DatabaseUrl -Sql @"
INSERT INTO representations (
  representation_id,
  target_kind,
  target_id,
  tier_enum,
  tier_complexity,
  payload_hash,
  payload_text,
  author_identity_id,
  language_locale,
  provenance,
  created_block_height,
  created_event_index,
  created_event_id
)
SELECT
  '$framingRepresentationId'::uuid,
  0,
  '$ideaAId'::uuid,
  1,
  1,
  '0000000000000000000000000000000000000000000000000000000000000000',
  'stage1 replay fallback framing representation',
  '$writerIdentityId'::uuid,
  NULL,
  'verify-replay-equality-fallback',
  e.block_height,
  e.event_index,
  '$framingRepresentationId'::uuid
FROM events e
ORDER BY e.block_height, e.event_index
LIMIT 1
ON CONFLICT (representation_id) DO NOTHING;
"@
    }

    Invoke-HttpJson -Method "POST" -Url "http://127.0.0.1:3000/api/v1/canonical/challenges/importance" -Body @{
        challenge_id = $challengeId
        event_id = $challengeEventId
        framing_representation_ref = $framingRepresentationId
        context_key = "universal:default"
        axis = "important_to_humanity"
        timeframe = "medium_term"
        scope = "universal"
        target_left_idea_id = $ideaAId
        target_right_idea_id = $ideaBId
    } -ExpectedStatus 200 -BearerToken $writerToken | Out-Null

    Invoke-HttpJson -Method "POST" -Url "http://127.0.0.1:3000/api/v1/canonical/challenges/$challengeId/arguments" -Body @{
        connection_id = $argumentConnectionId
        event_id = $argumentConnectionEventId
        argument_idea_id = $argumentIdeaId
        subject_idea_id = $ideaAId
    } -ExpectedStatus 200 -BearerToken $writerToken | Out-Null

    $cycleNoopEventId = "00000000-0000-7000-8000-00000000f380"
    $cycleCloseEventId = "00000000-0000-7000-8000-00000000f381"
    Insert-ForcedCycleClose -DatabaseUrl $DatabaseUrl -NoopEventId $cycleNoopEventId -CycleCloseEventId $cycleCloseEventId -NoopSpeakerIdentityId $writerIdentityId

    function Register-CanonicalVoter {
        param(
            [string]$Username,
            [string]$Password,
            [string]$IdentityId,
            [string]$IdentityTitle,
            [string]$GrantEventId
        )

        $reg = Invoke-HttpJson -Method "POST" -Url "http://127.0.0.1:3000/api/v0/auth/register" -Body @{
            username = $Username
            password = $Password
        } -ExpectedStatus 200 -BearerToken ""
        $accountId = [string]$reg.account_id
        $token = [string]$reg.token
        if ([string]::IsNullOrWhiteSpace($accountId) -or [string]::IsNullOrWhiteSpace($token)) {
            Fail "voter registration response missing account_id/token for $Username"
        }
        Invoke-PsqlStatement -DatabaseUrl $DatabaseUrl -Sql "UPDATE accounts SET canonical_identity_id = '$IdentityId'::uuid WHERE account_id = '$accountId'::uuid;"
        Invoke-PsqlStatement -DatabaseUrl $DatabaseUrl -Sql "INSERT INTO identities_s0 (identity_id, title, created_event_id) VALUES ('$IdentityId'::uuid, '$IdentityTitle', '$IdentityId'::uuid) ON CONFLICT (identity_id) DO NOTHING;"
        Invoke-HttpJson -Method "POST" -Url "http://127.0.0.1:3000/api/v1/canonical/verifier/grants" -Body @{
            identity_id = $IdentityId
            canonical_writer_level = "1"
            email_verified = $true
            event_id = $GrantEventId
        } -ExpectedStatus 200 -BearerToken $seedVerifierToken | Out-Null
        return $token
    }

    $voter1Token = Register-CanonicalVoter -Username "stage1_replay_voter_1" -Password "stage1-replay-voter-1-password" -IdentityId "00000000-0000-7000-8000-00000000d911" -IdentityTitle "stage1 replay voter 1" -GrantEventId "00000000-0000-7000-8000-00000000f105"
    $voter2Token = Register-CanonicalVoter -Username "stage1_replay_voter_2" -Password "stage1-replay-voter-2-password" -IdentityId "00000000-0000-7000-8000-00000000d912" -IdentityTitle "stage1 replay voter 2" -GrantEventId "00000000-0000-7000-8000-00000000f106"
    $voter3Token = Register-CanonicalVoter -Username "stage1_replay_voter_3" -Password "stage1-replay-voter-3-password" -IdentityId "00000000-0000-7000-8000-00000000d913" -IdentityTitle "stage1 replay voter 3" -GrantEventId "00000000-0000-7000-8000-00000000f107"

    function Resolve-VoteSessionForChallenge {
        param(
            [object]$InitialPull,
            [string]$TargetChallengeId,
            [string]$BearerToken,
            [object[]]$ExtraAttempts
        )

        if ([string]$InitialPull.challenge_id -eq $TargetChallengeId) {
            return $InitialPull
        }

        foreach ($attempt in $ExtraAttempts) {
            $nextPull = Invoke-HttpJson -Method "POST" -Url "http://127.0.0.1:3000/api/v1/canonical/vote-sessions/pull" -Body @{
                vote_session_id = [string]$attempt.vote_session_id
                event_id = [string]$attempt.event_id
            } -ExpectedStatus 200 -BearerToken $BearerToken
            if ([string]$nextPull.challenge_id -eq $TargetChallengeId) {
                return $nextPull
            }
        }

        return $null
    }

    $pull1 = Invoke-HttpJson -Method "POST" -Url "http://127.0.0.1:3000/api/v1/canonical/vote-sessions/pull" -Body @{
        vote_session_id = "00000000-0000-7000-8000-00000000e401"
        event_id = "00000000-0000-7000-8000-00000000f401"
    } -ExpectedStatus 200 -BearerToken $voter1Token

    $pull2 = Invoke-HttpJson -Method "POST" -Url "http://127.0.0.1:3000/api/v1/canonical/vote-sessions/pull" -Body @{
        vote_session_id = "00000000-0000-7000-8000-00000000e402"
        event_id = "00000000-0000-7000-8000-00000000f402"
    } -ExpectedStatus 200 -BearerToken $voter2Token

    $pull3 = Invoke-HttpJson -Method "POST" -Url "http://127.0.0.1:3000/api/v1/canonical/vote-sessions/pull" -Body @{
        vote_session_id = "00000000-0000-7000-8000-00000000e403"
        event_id = "00000000-0000-7000-8000-00000000f403"
    } -ExpectedStatus 200 -BearerToken $voter3Token

    $assignedChallenge = [string]$pull1.challenge_id
    if ([string]::IsNullOrWhiteSpace($assignedChallenge)) {
        Fail "vote-session pull did not return challenge_id"
    }
    $pull2Resolved = Resolve-VoteSessionForChallenge -InitialPull $pull2 -TargetChallengeId $assignedChallenge -BearerToken $voter2Token -ExtraAttempts @(
        @{ vote_session_id = "00000000-0000-7000-8000-00000000e453"; event_id = "00000000-0000-7000-8000-00000000f453" },
        @{ vote_session_id = "00000000-0000-7000-8000-00000000e454"; event_id = "00000000-0000-7000-8000-00000000f454" }
    )
    $pull3Resolved = Resolve-VoteSessionForChallenge -InitialPull $pull3 -TargetChallengeId $assignedChallenge -BearerToken $voter3Token -ExtraAttempts @(
        @{ vote_session_id = "00000000-0000-7000-8000-00000000e455"; event_id = "00000000-0000-7000-8000-00000000f455" },
        @{ vote_session_id = "00000000-0000-7000-8000-00000000e456"; event_id = "00000000-0000-7000-8000-00000000f456" }
    )
    if ($null -eq $pull2Resolved -or $null -eq $pull3Resolved) {
        Fail "vote-session pulls did not converge on the same challenge for deterministic 3-vote aggregation"
    }

    Invoke-HttpJson -Method "POST" -Url "http://127.0.0.1:3000/api/v1/canonical/challenges/$assignedChallenge/votes" -Body @{
        vote_session_id = [string]$pull1.vote_session_id
        event_id = "00000000-0000-7000-8000-00000000f411"
        vote_choice = "left"
    } -ExpectedStatus 200 -BearerToken $voter1Token | Out-Null

    Invoke-HttpJson -Method "POST" -Url "http://127.0.0.1:3000/api/v1/canonical/challenges/$assignedChallenge/votes" -Body @{
        vote_session_id = [string]$pull2Resolved.vote_session_id
        event_id = "00000000-0000-7000-8000-00000000f412"
        vote_choice = "left"
    } -ExpectedStatus 200 -BearerToken $voter2Token | Out-Null

    $thirdVote = Invoke-HttpJson -Method "POST" -Url "http://127.0.0.1:3000/api/v1/canonical/challenges/$assignedChallenge/votes" -Body @{
        vote_session_id = [string]$pull3Resolved.vote_session_id
        event_id = "00000000-0000-7000-8000-00000000f413"
        vote_choice = "right"
    } -ExpectedStatus 200 -BearerToken $voter3Token
    if ([string]::IsNullOrWhiteSpace([string]$thirdVote.verdict_event_id)) {
        Fail "third deterministic vote did not produce a verdict event"
    }

    return @{
        WriterToken = $writerToken
        ChallengeId = $assignedChallenge
    }
}

function Capture-ApiSet {
    param(
        [string]$OutputDir,
        [string]$SelectedIdeaId,
        [string[]]$RelativeImportanceIdeaIds,
        [string]$SearchQuery,
        [string]$ChallengeId = ""
    )

    if (-not (Test-Path $OutputDir)) {
        New-Item -ItemType Directory -Path $OutputDir | Out-Null
    }

    $snapshotLatestRaw = Invoke-HttpGetRaw "http://127.0.0.1:3000/api/v0/snapshot/latest"
    Write-TextFile -Path (Join-Path $OutputDir "snapshot_latest.json") -Content $snapshotLatestRaw

    $cycleStatusRaw = Invoke-HttpGetRaw "http://127.0.0.1:3000/api/v1/canonical/cycles/current"
    Write-TextFile -Path (Join-Path $OutputDir "canonical_cycles_current.json") -Content $cycleStatusRaw

    $tempoStatusRaw = Invoke-HttpGetRaw "http://127.0.0.1:3000/api/v1/canonical/tempo/status"
    Write-TextFile -Path (Join-Path $OutputDir "canonical_tempo_status.json") -Content $tempoStatusRaw

    $ideasTopRaw = Invoke-HttpGetRaw "http://127.0.0.1:3000/api/v0/ideas/top?limit=10&offset=0"
    Write-TextFile -Path (Join-Path $OutputDir "ideas_top.json") -Content $ideasTopRaw
    $ideasTop = $ideasTopRaw | ConvertFrom-Json -ErrorAction Stop
    if ($null -eq $ideasTop.ideas -or $ideasTop.ideas.Count -lt 2) {
        Fail "ideas_top did not return enough ideas for replay equality checks"
    }

    if ([string]::IsNullOrWhiteSpace($SelectedIdeaId)) {
        $SelectedIdeaId = [string]$ideasTop.ideas[0].idea_id
    }
    if ([string]::IsNullOrWhiteSpace($SelectedIdeaId)) {
        Fail "Unable to resolve selected idea id"
    }

    $selectedIdea = $ideasTop.ideas | Where-Object { $_.idea_id -eq $SelectedIdeaId } | Select-Object -First 1
    if ($null -eq $selectedIdea) {
        Fail "Selected idea id not found in ideas_top: $SelectedIdeaId"
    }

    $identityId = [string]$selectedIdea.speaker_identity_id
    if ([string]::IsNullOrWhiteSpace($identityId)) {
        Fail "Selected idea missing speaker_identity_id"
    }

    if ($null -eq $RelativeImportanceIdeaIds -or $RelativeImportanceIdeaIds.Count -lt 2) {
        $RelativeImportanceIdeaIds = @(
            [string]$ideasTop.ideas[0].idea_id,
            [string]$ideasTop.ideas[1].idea_id
        )
    }

    if ([string]::IsNullOrWhiteSpace($SearchQuery)) {
        $title = [string]$selectedIdea.title
        if ([string]::IsNullOrWhiteSpace($title)) {
            $SearchQuery = "idea"
        } else {
            $SearchQuery = ($title.Split(" ", [System.StringSplitOptions]::RemoveEmptyEntries) | Select-Object -First 1)
            if ([string]::IsNullOrWhiteSpace($SearchQuery)) {
                $SearchQuery = "idea"
            }
        }
    }

    $ideaDetailRaw = Invoke-HttpGetRaw "http://127.0.0.1:3000/api/v0/idea/$SelectedIdeaId"
    Write-TextFile -Path (Join-Path $OutputDir "idea_detail.json") -Content $ideaDetailRaw

    $neighborhoodRaw = Invoke-HttpGetRaw "http://127.0.0.1:3000/api/v0/idea/$SelectedIdeaId/neighborhood?depth=1&limit_per_hop=50"
    Write-TextFile -Path (Join-Path $OutputDir "idea_neighborhood.json") -Content $neighborhoodRaw

    $identityRaw = Invoke-HttpGetRaw "http://127.0.0.1:3000/api/v0/identity/$identityId"
    Write-TextFile -Path (Join-Path $OutputDir "identity_detail.json") -Content $identityRaw

    $ideaRailsRaw = Invoke-HttpGetRaw "http://127.0.0.1:3000/api/v0/idea/$SelectedIdeaId/rails"
    Write-TextFile -Path (Join-Path $OutputDir "idea_rails.json") -Content $ideaRailsRaw

    $ideaRails = $ideaRailsRaw | ConvertFrom-Json -ErrorAction Stop
    $railId = $null
    if ($ideaRails.rails -and $ideaRails.rails.Count -gt 0) {
        $railId = [string]$ideaRails.rails[0].rail_id
    }
    if (-not [string]::IsNullOrWhiteSpace($railId)) {
        $railDetailRaw = Invoke-HttpGetRaw "http://127.0.0.1:3000/api/v0/rail/$railId"
        Write-TextFile -Path (Join-Path $OutputDir "rail_detail.json") -Content $railDetailRaw
    } else {
        Write-TextFile -Path (Join-Path $OutputDir "rail_detail.json") -Content "{`"rail`":null}"
    }

    $riIdsCsv = [string]::Join(",", $RelativeImportanceIdeaIds)
    $relativeImportanceRaw = Invoke-HttpGetRaw "http://127.0.0.1:3000/api/v0/connections/relative-importance?idea_ids=$riIdsCsv"
    Write-TextFile -Path (Join-Path $OutputDir "relative_importance.json") -Content $relativeImportanceRaw

    $escapedSearch = [System.Uri]::EscapeDataString($SearchQuery)
    $searchRaw = Invoke-HttpGetRaw "http://127.0.0.1:3000/api/v0/search/ideas?q=$escapedSearch&limit=10&offset=0"
    Write-TextFile -Path (Join-Path $OutputDir "search_ideas.json") -Content $searchRaw

    if (-not [string]::IsNullOrWhiteSpace($ChallengeId)) {
        $challengeRaw = Invoke-HttpGetRaw -Url "http://127.0.0.1:3000/api/v1/canonical/challenges/$ChallengeId" -ExpectedStatus 200
        Write-TextFile -Path (Join-Path $OutputDir "challenge_detail.json") -Content $challengeRaw
        Write-TextFile -Path (Join-Path $OutputDir "challenge_detail_after_votes.json") -Content $challengeRaw
    } else {
        Write-TextFile -Path (Join-Path $OutputDir "challenge_detail.json") -Content "{`"challenge`":null}"
        Write-TextFile -Path (Join-Path $OutputDir "challenge_detail_after_votes.json") -Content "{`"challenge`":null}"
    }

    $state = @{
        selected_idea_id = $SelectedIdeaId
        relative_importance_idea_ids = $RelativeImportanceIdeaIds
        search_query = $SearchQuery
        identity_id = $identityId
        rail_id = $railId
        challenge_id = $ChallengeId
    } | ConvertTo-Json -Depth 5
    Write-TextFile -Path (Join-Path $OutputDir "capture_state.json") -Content $state

    return @{
        SelectedIdeaId = $SelectedIdeaId
        RelativeImportanceIdeaIds = $RelativeImportanceIdeaIds
        SearchQuery = $SearchQuery
        ChallengeId = $ChallengeId
    }
}

if (-not (Test-Path $reseedScript)) {
    Fail "Missing reseed script: $reseedScript"
}

Ensure-Environment

if (-not (Test-Path $auditRoot)) {
    New-Item -ItemType Directory -Path $auditRoot | Out-Null
}
if (Test-Path $baselineDir) { Remove-Item -Recurse -Force $baselineDir }
if (Test-Path $replayDir) { Remove-Item -Recurse -Force $replayDir }
New-Item -ItemType Directory -Path $baselineDir | Out-Null
New-Item -ItemType Directory -Path $replayDir | Out-Null

if (-not $SkipBaselineReseed) {
    Run-ReseedPipeline
}

Write-Step "Build api-server for capture"
Invoke-External "cargo" @("build", "-p", "api-server") $backendDir

$script:HttpClient = New-Object System.Net.Http.HttpClient
$script:HttpClient.Timeout = [TimeSpan]::FromSeconds(10)

$baselineServer = $null
$replayServer = $null

try {
    Write-Step "Apply deterministic canonical write scenario (baseline)"
    $baselineServer = Start-ApiServer -BackendDir $backendDir -DatabaseUrl $env:DATABASE_URL -PgPassFile $env:PGPASSFILE
    $baselineScenario = Run-CanonicalWriteScenario -DatabaseUrl $env:DATABASE_URL
    Stop-ApiServer -Handle $baselineServer
    $baselineServer = $null

    Write-Step "Build snapshot after baseline canonical writes"
    Invoke-External "cargo" @("run", "-p", "snapshot-builder") $backendDir

    Write-Step "Capture baseline API outputs"
    $baselineServer = Start-ApiServer -BackendDir $backendDir -DatabaseUrl $env:DATABASE_URL -PgPassFile $env:PGPASSFILE
    $captureState = Capture-ApiSet -OutputDir $baselineDir -SelectedIdeaId $null -RelativeImportanceIdeaIds @() -SearchQuery "" -ChallengeId $baselineScenario.ChallengeId
    Stop-ApiServer -Handle $baselineServer
    $baselineServer = $null

    Write-Step "Run replay reseed pipeline"
    Run-ReseedPipeline

    Write-Step "Apply deterministic canonical write scenario (replay)"
    $replayServer = Start-ApiServer -BackendDir $backendDir -DatabaseUrl $env:DATABASE_URL -PgPassFile $env:PGPASSFILE
    $replayScenario = Run-CanonicalWriteScenario -DatabaseUrl $env:DATABASE_URL
    Stop-ApiServer -Handle $replayServer
    $replayServer = $null

    Write-Step "Build snapshot after replay canonical writes"
    Invoke-External "cargo" @("run", "-p", "snapshot-builder") $backendDir

    Write-Step "Capture replay API outputs"
    $replayServer = Start-ApiServer -BackendDir $backendDir -DatabaseUrl $env:DATABASE_URL -PgPassFile $env:PGPASSFILE
    Capture-ApiSet -OutputDir $replayDir -SelectedIdeaId $captureState.SelectedIdeaId -RelativeImportanceIdeaIds $captureState.RelativeImportanceIdeaIds -SearchQuery $captureState.SearchQuery -ChallengeId $captureState.ChallengeId | Out-Null
    Stop-ApiServer -Handle $replayServer
    $replayServer = $null
}
finally {
    Stop-ApiServer -Handle $baselineServer
    Stop-ApiServer -Handle $replayServer
    if ($script:HttpClient) {
        $script:HttpClient.Dispose()
    }
}

Write-Step "Compare baseline vs replay output hashes"
$baselineFiles = Get-ChildItem -Path $baselineDir -Filter "*.json" | Sort-Object Name
$summaryLines = New-Object System.Collections.Generic.List[string]
$mismatchCount = 0

foreach ($baselineFile in $baselineFiles) {
    $replayPath = Join-Path $replayDir $baselineFile.Name
    if (-not (Test-Path $replayPath)) {
        $summaryLines.Add("mismatch:$($baselineFile.Name):missing_in_replay")
        $mismatchCount += 1
        continue
    }

    $baselineHash = (Get-FileHash -Algorithm SHA256 -Path $baselineFile.FullName).Hash
    $replayHash = (Get-FileHash -Algorithm SHA256 -Path $replayPath).Hash
    if ($baselineHash -eq $replayHash) {
        $summaryLines.Add("match:$($baselineFile.Name):$($baselineHash)")
    } else {
        $summaryLines.Add("mismatch:$($baselineFile.Name):$($baselineHash):$($replayHash)")
        $mismatchCount += 1
    }
}

Write-TextFile -Path $diffSummaryPath -Content ($summaryLines -join [Environment]::NewLine)

if ($mismatchCount -gt 0) {
    Fail "Replay equality mismatches detected; see $diffSummaryPath"
}

Write-Pass "Replay equality verified"
Write-Pass "Diff summary written to $diffSummaryPath"
exit 0
