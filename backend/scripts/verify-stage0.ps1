. "$PSScriptRoot\dev-bootstrap.ps1"

$backendDir = (Join-Path $PSScriptRoot "..")
$databaseUrl = $env:DATABASE_URL
$pgpassfile = $env:PGPASSFILE
$rustPathPrefix = "$env:USERPROFILE\.cargo\bin;"

Set-Location $backendDir

$ErrorActionPreference = "Stop"

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

Write-Host "[verify] stop any running api-server.exe" -ForegroundColor Cyan
try { taskkill /IM api-server.exe /F 2>$null | Out-Null } catch { }

Write-Host "[verify] build api-server" -ForegroundColor Cyan
cargo build -p api-server
if ($LASTEXITCODE -ne 0) { exit 1 }

[int64]$snapshotIntervalBlocks = 100
if (-not [string]::IsNullOrWhiteSpace($env:SNAPSHOT_INTERVAL_BLOCKS)) {
    $parsedInterval = 0L
    if ([int64]::TryParse($env:SNAPSHOT_INTERVAL_BLOCKS, [ref]$parsedInterval) -and $parsedInterval -gt 0) {
        $snapshotIntervalBlocks = $parsedInterval
    }
}
[int64]$currentMaxBlockHeight = 0
$currentMaxBlockHeightKnown = $false
$currentMaxBlockHeightRaw = $null
try {
    $currentMaxBlockHeightRaw = & psql $databaseUrl -At -v ON_ERROR_STOP=1 -c "SELECT COALESCE(MAX(block_height), 0)::bigint FROM blocks;"
    if ($currentMaxBlockHeightRaw -is [array]) {
        $currentMaxBlockHeightRaw = $currentMaxBlockHeightRaw[-1]
    }
    $parsedHeight = 0L
    if ([int64]::TryParse(([string]$currentMaxBlockHeightRaw).Trim(), [ref]$parsedHeight)) {
        $currentMaxBlockHeight = $parsedHeight
        $currentMaxBlockHeightKnown = $true
    }
} catch {
    $currentMaxBlockHeightRaw = $null
}

$snapshotBoundaryAligned = $currentMaxBlockHeightKnown -and ($snapshotIntervalBlocks -gt 0) -and (($currentMaxBlockHeight % $snapshotIntervalBlocks) -eq 0)
if ($snapshotBoundaryAligned) {
    $env:REQUIRE_SNAPSHOT_COMMIT = "1"
    Write-Host "[verify] snapshot-builder will require snapshot_commit at height=$currentMaxBlockHeight interval=$snapshotIntervalBlocks" -ForegroundColor DarkGray
} else {
    Remove-Item -Path Env:REQUIRE_SNAPSHOT_COMMIT -ErrorAction SilentlyContinue
    Write-Host "[verify] snapshot-builder running in non-boundary mode at height=$currentMaxBlockHeight interval=$snapshotIntervalBlocks" -ForegroundColor DarkGray
}

Write-Host "[verify] run snapshot-builder" -ForegroundColor Cyan
cargo run -p snapshot-builder
if ($LASTEXITCODE -ne 0) { exit 1 }

Write-Host "[verify] run snapshot-verify" -ForegroundColor Cyan
cargo run -p snapshot-verify -- --latest --profile stage0
if ($LASTEXITCODE -ne 0) { exit 1 }

$exe = Join-Path $backendDir "target\debug\api-server.exe"
Write-Host "[verify] start api-server (background)" -ForegroundColor Cyan
$job = Start-Job -ScriptBlock {
    param($backendDir, $exe, $databaseUrl, $pgpassfile, $rustPathPrefix)
    Set-Location $backendDir
    $env:Path = "$rustPathPrefix$env:Path"
    $env:DATABASE_URL = $databaseUrl
    if (-not [string]::IsNullOrWhiteSpace($pgpassfile)) {
        $env:PGPASSFILE = $pgpassfile
    }
    & $exe
} -ArgumentList $backendDir, $exe, $databaseUrl, $pgpassfile, $rustPathPrefix

Add-Type -AssemblyName System.Net.Http
$httpClient = New-Object System.Net.Http.HttpClient

Write-Host "[verify] waiting for api-server readiness" -ForegroundColor Cyan
$ready = $false
for ($i = 0; $i -lt 20; $i++) {
    if ($job.State -ne "Running") {
        Write-Host "[verify] api-server exited before ready" -ForegroundColor Red
        try { Receive-Job $job -Keep | ForEach-Object { Write-RedactedHost "$_" } } catch { }
        exit 1
    }
    try {
        $resp = $httpClient.GetAsync("http://127.0.0.1:3000/api/v0/snapshot/latest").GetAwaiter().GetResult()
        if ([int]$resp.StatusCode -eq 200) {
            $ready = $true
            break
        }
    } catch { }
    Start-Sleep -Milliseconds 250
}

if (-not $ready) {
    Write-Host "[verify] api-server not ready after timeout" -ForegroundColor Red
    try {
        $logs = Receive-Job $job -Keep -ErrorAction SilentlyContinue
        if ($logs) {
            Write-Host "[verify] api-server logs:" -ForegroundColor Yellow
            $logs | ForEach-Object { Write-RedactedHost "$_" }
        }
    } catch { }
    exit 1
}

function Invoke-Check {
    param(
        [string]$Method,
        [string]$Url,
        [int]$ExpectedStatus,
        [bool]$ExpectJsonError = $false
    )

    $request = New-Object System.Net.Http.HttpRequestMessage($Method, $Url)
    try {
        $response = $httpClient.SendAsync($request).GetAwaiter().GetResult()
        $status = [int]$response.StatusCode
        $body = $response.Content.ReadAsStringAsync().GetAwaiter().GetResult()
    } catch {
        $status = -1
        $body = $_.Exception.Message
    }

    Write-Host "[check] $Method $Url -> $status" -ForegroundColor Cyan
    Write-RedactedHost $body

    if ($status -ne $ExpectedStatus) {
        return $false
    }
    if ($ExpectedStatus -ge 400 -and $ExpectJsonError) {
        try {
            $json = $body | ConvertFrom-Json -ErrorAction Stop
        } catch {
            Write-Host "[check] expected JSON error body, got:" -ForegroundColor Red
            Write-RedactedHost $body
            return $false
        }
        if ($null -eq $json) { return $false }
        if (-not $json.PSObject.Properties.Name.Contains("error_code")) { return $false }
        if (-not $json.PSObject.Properties.Name.Contains("message")) { return $false }
        if ([string]::IsNullOrWhiteSpace([string]$json.error_code)) { return $false }
        if ([string]::IsNullOrWhiteSpace([string]$json.message)) { return $false }
    }
    return $true
}

function Invoke-Json {
    param(
        [string]$Method,
        [string]$Url,
        [object]$Body,
        [int]$ExpectedStatus
    )

    $jsonBody = $Body | ConvertTo-Json -Depth 5
    $content = New-Object System.Net.Http.StringContent($jsonBody, [System.Text.Encoding]::UTF8, "application/json")
    $request = New-Object System.Net.Http.HttpRequestMessage($Method, $Url)
    $request.Content = $content

    try {
        $response = $httpClient.SendAsync($request).GetAwaiter().GetResult()
        $status = [int]$response.StatusCode
        $body = $response.Content.ReadAsStringAsync().GetAwaiter().GetResult()
    } catch {
        $status = -1
        $body = $_.Exception.Message
    }

    Write-Host "[check] $Method $Url -> $status" -ForegroundColor Cyan
    Write-RedactedHost $body

    if ($status -ne $ExpectedStatus) {
        return @($false, $null)
    }
    try {
        $json = $body | ConvertFrom-Json -ErrorAction Stop
        return @($true, $json)
    } catch {
        return @($false, $null)
    }
}

function Invoke-JsonAuth {
    param(
        [string]$Method,
        [string]$Url,
        [object]$Body,
        [string]$Token,
        [int]$ExpectedStatus
    )

    $jsonBody = $Body | ConvertTo-Json -Depth 5
    $content = New-Object System.Net.Http.StringContent($jsonBody, [System.Text.Encoding]::UTF8, "application/json")
    $request = New-Object System.Net.Http.HttpRequestMessage($Method, $Url)
    $request.Content = $content
    $request.Headers.Authorization = New-Object System.Net.Http.Headers.AuthenticationHeaderValue("Bearer", $Token)

    try {
        $response = $httpClient.SendAsync($request).GetAwaiter().GetResult()
        $status = [int]$response.StatusCode
        $body = $response.Content.ReadAsStringAsync().GetAwaiter().GetResult()
    } catch {
        $status = -1
        $body = $_.Exception.Message
    }

    Write-Host "[check] $Method $Url -> $status" -ForegroundColor Cyan
    Write-RedactedHost $body

    if ($status -ne $ExpectedStatus) {
        return @($false, $null)
    }
    try {
        $json = $body | ConvertFrom-Json -ErrorAction Stop
        return @($true, $json)
    } catch {
        return @($false, $null)
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

function Resolve-SeedVerifierToken {
    param(
        [string]$OwnerUsername,
        [string]$OwnerPassword
    )

    if ([string]::IsNullOrWhiteSpace($OwnerUsername) -or [string]::IsNullOrWhiteSpace($OwnerPassword)) {
        return $null
    }
    $login = Invoke-Json -Method POST -Url "http://127.0.0.1:3000/api/v0/auth/login" -Body @{
        username = $OwnerUsername
        password = $OwnerPassword
    } -ExpectedStatus 200
    if (-not $login[0]) {
        return $null
    }
    return [string]$login[1].token
}

function New-CanonicalUuidV7 {
    $timestampMs = [DateTimeOffset]::UtcNow.ToUnixTimeMilliseconds()
    $timeHex = "{0:x12}" -f $timestampMs
    $randomBytes = New-Object byte[] 10
    $rng = [System.Security.Cryptography.RandomNumberGenerator]::Create()
    try {
        $rng.GetBytes($randomBytes)
    } finally {
        $rng.Dispose()
    }
    $randHex = ($randomBytes | ForEach-Object { $_.ToString("x2") }) -join ""
    $variantNibble = [Convert]::ToInt32($randHex.Substring(4, 1), 16)
    $variantNibble = ($variantNibble -band 0x3) -bor 0x8
    $hex = "{0}7{1}{2:x1}{3}" -f $timeHex, $randHex.Substring(1, 3), $variantNibble, $randHex.Substring(5)
    return "{0}-{1}-{2}-{3}-{4}" -f $hex.Substring(0, 8), $hex.Substring(8, 4), $hex.Substring(12, 4), $hex.Substring(16, 4), $hex.Substring(20, 12)
}

function Get-CanonicalCounts {
    param(
        [string]$DatabaseUrl
    )

    try {
        $raw = & psql $DatabaseUrl -At -v ON_ERROR_STOP=1 -c "SELECT (SELECT COUNT(*)::bigint FROM events), (SELECT COUNT(*)::bigint FROM ideas), (SELECT COUNT(*)::bigint FROM connections);"
        if ($LASTEXITCODE -ne 0) {
            return $null
        }

        if ($raw -is [array]) {
            $raw = $raw[-1]
        }
        $line = [string]$raw
        if ([string]::IsNullOrWhiteSpace($line)) {
            return $null
        }

        $parts = $line.Trim().Split("|")
        if ($parts.Count -ne 3) {
            return $null
        }

        return @{
            events = [int64]$parts[0]
            ideas = [int64]$parts[1]
            connections = [int64]$parts[2]
        }
    } catch {
        return $null
    }
}

function CanonicalCountsChanged {
    param(
        [hashtable]$Before,
        [hashtable]$After
    )

    if ($null -eq $Before -or $null -eq $After) {
        return $true
    }

    return ($Before.events -ne $After.events) -or
        ($Before.ideas -ne $After.ideas) -or
        ($Before.connections -ne $After.connections)
}

function CanonicalMutationBlocked {
    param(
        [string]$DatabaseUrl,
        [string]$Sql
    )

    try {
        & psql $DatabaseUrl -v ON_ERROR_STOP=1 -c $Sql | Out-Null
        if ($LASTEXITCODE -ne 0) {
            return $true
        }
        return $false
    } catch {
        return $true
    }
}

function Get-FirstRepresentationId {
    param(
        [string]$DatabaseUrl
    )

    try {
        $raw = & psql $DatabaseUrl -At -v ON_ERROR_STOP=1 -c "SELECT representation_id::text FROM representations ORDER BY created_block_height, created_event_index LIMIT 1;"
        if ($LASTEXITCODE -ne 0) {
            return $null
        }
        if ($raw -is [array]) {
            $raw = $raw[-1]
        }
        $value = [string]$raw
        if ([string]::IsNullOrWhiteSpace($value)) {
            return $null
        }
        return $value.Trim()
    } catch {
        return $null
    }
}

function Insert-ForcedCycleClose {
    param(
        [string]$DatabaseUrl,
        [string]$NoopEventId,
        [string]$CycleCloseEventId,
        [string]$NoopSpeakerIdentityId,
        [string]$ClosureKind
    )

    try {
        $normalizedClosureKind = ([string]$ClosureKind).Trim().ToLowerInvariant()
        if ($normalizedClosureKind -ne "forced" -and $normalizedClosureKind -ne "deliberative") {
            return $false
        }
        $forcedSealLiteral = if ($normalizedClosureKind -eq "forced") { "true" } else { "false" }
        $closureKindCode = if ($normalizedClosureKind -eq "forced") { 1 } else { 0 }

        $blockRaw = & psql $DatabaseUrl -At -v ON_ERROR_STOP=1 -c "SELECT COALESCE(MAX(block_height), 0)::bigint FROM blocks;"
        if ($blockRaw -is [array]) { $blockRaw = $blockRaw[-1] }
        [int64]$blockHeight = [int64]([string]$blockRaw).Trim()

        $indexRaw = & psql $DatabaseUrl -At -v ON_ERROR_STOP=1 -c "SELECT COALESCE(MAX(event_index), -1)::int + 1 FROM events WHERE block_height = $blockHeight;"
        if ($indexRaw -is [array]) { $indexRaw = $indexRaw[-1] }
        [int]$noopIndex = [int]([string]$indexRaw).Trim()
        $closeIndex = $noopIndex + 1

        $cycleRaw = & psql $DatabaseUrl -At -v ON_ERROR_STOP=1 -c "SELECT COALESCE(MAX(cycle_index), -1)::bigint + 1 FROM cycle_boundaries;"
        if ($cycleRaw -is [array]) { $cycleRaw = $cycleRaw[-1] }
        [int64]$cycleIndex = [int64]([string]$cycleRaw).Trim()

        $closureHeight = $blockHeight
        $nextBlockHeight = $closureHeight + 1

        & psql $DatabaseUrl -v ON_ERROR_STOP=1 -c @"
INSERT INTO events (block_height, event_index, event_id, event_type, speaker_identity_id, payload_json, signature)
VALUES ($blockHeight, $noopIndex, '$NoopEventId'::uuid, 'noop', '$NoopSpeakerIdentityId'::uuid, '{}'::jsonb, NULL);
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
    'closure_kind', '$normalizedClosureKind',
    'forced_seal', $forcedSealLiteral,
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
  $closureKindCode,
  $forcedSealLiteral,
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
"@ | Out-Null
        if ($LASTEXITCODE -ne 0) {
            return $false
        }

        # Fail fast if harness inserts diverge: payload and cycle_boundaries must encode
        # the same closure path/forced flag so replay's cycle_close invariant stays authoritative.
        $consistencyRaw = & psql $DatabaseUrl -At -F '|' -v ON_ERROR_STOP=1 -c @"
SELECT
  COALESCE(e.payload_json->>'closure_kind',''),
  COALESCE(e.payload_json->>'forced_seal',''),
  COALESCE(CASE cb.closure_kind WHEN 0 THEN 'deliberative' WHEN 1 THEN 'forced' ELSE '' END,''),
  COALESCE(CASE WHEN cb.forced_seal THEN 'true' ELSE 'false' END,'')
FROM events e
LEFT JOIN cycle_boundaries cb ON cb.source_event_id = e.event_id
WHERE e.event_id = '$CycleCloseEventId'::uuid;
"@
        if ($LASTEXITCODE -ne 0) {
            throw "cycle-close guard failed: unable to read inserted cycle_close event_id=$CycleCloseEventId"
        }
        if ($consistencyRaw -is [array]) { $consistencyRaw = $consistencyRaw[-1] }
        $consistencyLine = [string]$consistencyRaw
        if ([string]::IsNullOrWhiteSpace($consistencyLine)) {
            throw "cycle-close guard failed: inserted cycle_close event_id=$CycleCloseEventId was not found"
        }
        $parts = $consistencyLine.Split('|')
        if ($parts.Count -lt 4) {
            throw "cycle-close guard failed: malformed consistency row for event_id=$CycleCloseEventId row='$consistencyLine'"
        }
        $eventClosureKind = $parts[0].Trim().ToLowerInvariant()
        $eventForcedSeal = $parts[1].Trim().ToLowerInvariant()
        $boundaryClosureKind = $parts[2].Trim().ToLowerInvariant()
        $boundaryForcedSeal = $parts[3].Trim().ToLowerInvariant()
        $expectedForcedSeal = if ($normalizedClosureKind -eq "forced") { "true" } else { "false" }
        if (($eventClosureKind -ne "forced" -and $eventClosureKind -ne "deliberative") -or ($boundaryClosureKind -ne "forced" -and $boundaryClosureKind -ne "deliberative")) {
            throw "cycle-close guard failed: invalid closure enum event='$eventClosureKind' boundary='$boundaryClosureKind' event_id=$CycleCloseEventId"
        }
        if (($eventClosureKind -ne $normalizedClosureKind) -or ($boundaryClosureKind -ne $normalizedClosureKind) -or ($eventForcedSeal -ne $expectedForcedSeal) -or ($boundaryForcedSeal -ne $expectedForcedSeal)) {
            throw "cycle-close guard mismatch event_id=$CycleCloseEventId expected(kind=$normalizedClosureKind,forced=$expectedForcedSeal) payload(kind=$eventClosureKind,forced=$eventForcedSeal) boundary(kind=$boundaryClosureKind,forced=$boundaryForcedSeal)"
        }
        return $true
    } catch {
        return $false
    }
}

function Resolve-CycleCloseKind {
    try {
        # Stage 0 may expose /api/v1/canonical/cycles/current for public read checks.
        # The harness must derive closure_kind from runtime cycle status (observed_work vs w_target)
        # and write matching closure_kind+forced_seal into both event payload and cycle_boundaries.
        # Replay invariant checks remain authoritative and unchanged.
        $resp = $httpClient.GetAsync("http://127.0.0.1:3000/api/v1/canonical/cycles/current").GetAwaiter().GetResult()
        $status = [int]$resp.StatusCode
        if ($status -ne 200) {
            return $null
        }

        $body = $resp.Content.ReadAsStringAsync().GetAwaiter().GetResult()
        $json = $body | ConvertFrom-Json -ErrorAction Stop
        if ($null -eq $json -or $null -eq $json.cycle) {
            return $null
        }

        $observedText = [string]$json.cycle.observed_work
        $targetText = [string]$json.cycle.w_target
        [int64]$observedWork = 0
        [int64]$wTarget = 0
        if (-not [int64]::TryParse($observedText, [ref]$observedWork)) {
            return $null
        }
        if (-not [int64]::TryParse($targetText, [ref]$wTarget)) {
            return $null
        }

        $derivedClosureKind = if ($observedWork -ge $wTarget) { "deliberative" } else { "forced" }
        if ($derivedClosureKind -ne "deliberative" -and $derivedClosureKind -ne "forced") {
            throw "cycle-close guard failed: invalid derived closure_kind '$derivedClosureKind' from observed_work=$observedWork w_target=$wTarget"
        }
        return $derivedClosureKind
    } catch {
        return $null
    }
}

$failures = @()

if (-not (Invoke-Check -Method GET -Url "http://127.0.0.1:3000/api/v0/snapshot/latest" -ExpectedStatus 200)) { $failures += "snapshot/latest" }
if (-not (Invoke-Check -Method GET -Url "http://127.0.0.1:3000/api/v0/ideas/top?limit=3&offset=0" -ExpectedStatus 200)) { $failures += "ideas/top" }
if (-not (Invoke-Check -Method GET -Url "http://127.0.0.1:3000/api/v0/idea/not-a-uuid" -ExpectedStatus 400 -ExpectJsonError $true)) { $failures += "idea/not-a-uuid" }
if (-not (Invoke-Check -Method GET -Url "http://127.0.0.1:3000/api/v0/idea/550e8400-e29b-41d4-a716-446655440000" -ExpectedStatus 400 -ExpectJsonError $true)) { $failures += "idea/uuidv4" }
if (-not (Invoke-Check -Method GET -Url "http://127.0.0.1:3000/does-not-exist" -ExpectedStatus 404 -ExpectJsonError $true)) { $failures += "404" }
if (-not (Invoke-Check -Method POST -Url "http://127.0.0.1:3000/api/v0/ideas/top" -ExpectedStatus 405 -ExpectJsonError $true)) { $failures += "405" }
if (-not (Invoke-Check -Method GET -Url "http://127.0.0.1:3000/api/v1/canonical/cycles/current" -ExpectedStatus 200)) { $failures += "stage1/public/cycles-current" }
if (-not (Invoke-Check -Method GET -Url "http://127.0.0.1:3000/api/v1/canonical/tempo/status" -ExpectedStatus 200)) { $failures += "stage1/public/tempo-status" }
if (-not (Invoke-Check -Method POST -Url "http://127.0.0.1:3000/api/v1/canonical/ideas" -ExpectedStatus 401 -ExpectJsonError $true)) { $failures += "stage1/write/auth-required" }

$ideaId = $null
try {
    $resp = $httpClient.GetAsync("http://127.0.0.1:3000/api/v0/ideas/top?limit=1&offset=0").GetAwaiter().GetResult()
    $status = [int]$resp.StatusCode
    $body = $resp.Content.ReadAsStringAsync().GetAwaiter().GetResult()
    Write-Host "[check] GET http://127.0.0.1:3000/api/v0/ideas/top?limit=1&offset=0 -> $status" -ForegroundColor Cyan
    Write-RedactedHost $body
    if ($status -ne 200) {
        $failures += "ideas/top/seed"
    } else {
        $json = $body | ConvertFrom-Json -ErrorAction Stop
        if ($null -eq $json.ideas -or $json.ideas.Count -lt 1) {
            $failures += "ideas/top/empty"
        } else {
            $ideaId = $json.ideas[0].idea_id
            if ([string]::IsNullOrWhiteSpace([string]$ideaId)) {
                $failures += "ideas/top/no-id"
                $ideaId = $null
            }
        }
    }
} catch {
    $failures += "ideas/top/seed"
}

if ($null -ne $ideaId) {
    try {
        $resp = $httpClient.GetAsync("http://127.0.0.1:3000/api/v0/idea/$ideaId").GetAwaiter().GetResult()
        $status = [int]$resp.StatusCode
        $body = $resp.Content.ReadAsStringAsync().GetAwaiter().GetResult()
        Write-Host "[check] GET http://127.0.0.1:3000/api/v0/idea/$ideaId -> $status" -ForegroundColor Cyan
        Write-RedactedHost $body
        if ($status -ne 200) {
            $failures += "idea/detail"
        } else {
            $json = $body | ConvertFrom-Json -ErrorAction Stop
            $payloadHash = $json.idea.payload_hash
            if ([string]::IsNullOrWhiteSpace([string]$payloadHash)) {
                $failures += "idea/detail/payload_hash"
            }
        }
    } catch {
        $failures += "idea/detail"
    }

    try {
        $resp = $httpClient.GetAsync("http://127.0.0.1:3000/api/v0/idea/$ideaId/neighborhood").GetAwaiter().GetResult()
        $status = [int]$resp.StatusCode
        $body = $resp.Content.ReadAsStringAsync().GetAwaiter().GetResult()
        Write-Host "[check] GET http://127.0.0.1:3000/api/v0/idea/$ideaId/neighborhood -> $status" -ForegroundColor Cyan
        Write-RedactedHost $body
        if ($status -ne 200) {
            $failures += "idea/neighborhood"
        } else {
            $json = $body | ConvertFrom-Json -ErrorAction Stop
            if ($null -eq $json.central_idea) { $failures += "idea/neighborhood/central_idea" }
            if ($null -eq $json.adjacent_ideas) { $failures += "idea/neighborhood/adjacent_ideas" }
            if ($null -eq $json.connections) { $failures += "idea/neighborhood/connections" }
            if ([string]::IsNullOrWhiteSpace([string]$json.depth_reached)) { $failures += "idea/neighborhood/depth_reached" }
        }
    } catch {
        $failures += "idea/neighborhood"
    }
}

$userSuffix = [DateTimeOffset]::UtcNow.ToUnixTimeMilliseconds()
$username = "stage0_$userSuffix"
$password = "stage0-test-pass-$userSuffix"
$token = $null
$canonicalBeforeAuth = Get-CanonicalCounts -DatabaseUrl $databaseUrl
if ($null -eq $canonicalBeforeAuth) {
    $failures += "canonical/counts/before-auth"
}

if (-not (CanonicalMutationBlocked -DatabaseUrl $databaseUrl -Sql "UPDATE ideas SET created_block_height = created_block_height WHERE idea_id = (SELECT idea_id FROM ideas ORDER BY created_block_height, created_event_index LIMIT 1);")) {
    $failures += "canonical/append-only/update-allowed"
}

if (-not (CanonicalMutationBlocked -DatabaseUrl $databaseUrl -Sql "DELETE FROM ideas WHERE idea_id = (SELECT idea_id FROM ideas ORDER BY created_block_height, created_event_index LIMIT 1);")) {
    $failures += "canonical/append-only/delete-allowed"
}

$reg = Invoke-Json -Method POST -Url "http://127.0.0.1:3000/api/v0/auth/register" -Body @{ username = $username; password = $password } -ExpectedStatus 200
if (-not $reg[0]) { $failures += "auth/register" } else { $token = $reg[1].token }

$canonicalAfterRegister = Get-CanonicalCounts -DatabaseUrl $databaseUrl
if (CanonicalCountsChanged -Before $canonicalBeforeAuth -After $canonicalAfterRegister) {
    $failures += "auth/register/canonical-write"
}

$login = Invoke-Json -Method POST -Url "http://127.0.0.1:3000/api/v0/auth/login" -Body @{ username = $username; password = $password } -ExpectedStatus 200
if (-not $login[0]) { $failures += "auth/login" } else { $token = $login[1].token }

if ([string]::IsNullOrWhiteSpace([string]$token)) { $failures += "auth/token" }

if ($null -ne $token) {
    $ideaPayload = @{
        title = "private draft $userSuffix"
        sentence = "private sentence $userSuffix"
        paragraph = "private paragraph $userSuffix"
        full = "private full $userSuffix"
    }
    $create = Invoke-JsonAuth -Method POST -Url "http://127.0.0.1:3000/api/v0/private/ideas" -Body $ideaPayload -Token $token -ExpectedStatus 200
    if (-not $create[0]) { $failures += "private/ideas/create" }

    $privateId = $null
    if ($create[1] -and $create[1].idea -and $create[1].idea.idea_id) {
        $privateId = $create[1].idea.idea_id
    } else {
        $failures += "private/ideas/create/shape"
    }

    $listRequest = New-Object System.Net.Http.HttpRequestMessage("GET", "http://127.0.0.1:3000/api/v0/private/ideas")
    $listRequest.Headers.Authorization = New-Object System.Net.Http.Headers.AuthenticationHeaderValue("Bearer", $token)
    try {
        $resp = $httpClient.SendAsync($listRequest).GetAwaiter().GetResult()
        $status = [int]$resp.StatusCode
        $body = $resp.Content.ReadAsStringAsync().GetAwaiter().GetResult()
    } catch {
        $status = -1
        $body = $_.Exception.Message
    }
    Write-Host "[check] GET http://127.0.0.1:3000/api/v0/private/ideas -> $status" -ForegroundColor Cyan
    Write-RedactedHost $body
    if ($status -ne 200) {
        $failures += "private/ideas/list"
    } else {
        try {
            $json = $body | ConvertFrom-Json -ErrorAction Stop
            $found = $false
            foreach ($idea in $json.ideas) {
                if ($idea.idea_id -eq $privateId) { $found = $true }
            }
            if (-not $found) { $failures += "private/ideas/list/missing" }
        } catch {
            $failures += "private/ideas/list/shape"
        }
    }

    if ($privateId) {
        $detailRequest = New-Object System.Net.Http.HttpRequestMessage("GET", "http://127.0.0.1:3000/api/v0/private/ideas/$privateId")
        $detailRequest.Headers.Authorization = New-Object System.Net.Http.Headers.AuthenticationHeaderValue("Bearer", $token)
        try {
            $resp = $httpClient.SendAsync($detailRequest).GetAwaiter().GetResult()
            $status = [int]$resp.StatusCode
            $body = $resp.Content.ReadAsStringAsync().GetAwaiter().GetResult()
        } catch {
            $status = -1
            $body = $_.Exception.Message
        }
        Write-Host "[check] GET http://127.0.0.1:3000/api/v0/private/ideas/$privateId -> $status" -ForegroundColor Cyan
        Write-RedactedHost $body
        if ($status -ne 200) {
            $failures += "private/ideas/detail"
        }

        $deleteRequest = New-Object System.Net.Http.HttpRequestMessage("DELETE", "http://127.0.0.1:3000/api/v0/private/ideas/$privateId")
        $deleteRequest.Headers.Authorization = New-Object System.Net.Http.Headers.AuthenticationHeaderValue("Bearer", $token)
        try {
            $resp = $httpClient.SendAsync($deleteRequest).GetAwaiter().GetResult()
            $status = [int]$resp.StatusCode
            $body = $resp.Content.ReadAsStringAsync().GetAwaiter().GetResult()
        } catch {
            $status = -1
            $body = $_.Exception.Message
        }
        Write-Host "[check] DELETE http://127.0.0.1:3000/api/v0/private/ideas/$privateId -> $status" -ForegroundColor Cyan
        Write-RedactedHost $body
        if ($status -ne 200) {
            $failures += "private/ideas/delete"
        }
    }

    $logoutRequest = New-Object System.Net.Http.HttpRequestMessage("POST", "http://127.0.0.1:3000/api/v0/auth/logout")
    $logoutRequest.Headers.Authorization = New-Object System.Net.Http.Headers.AuthenticationHeaderValue("Bearer", $token)
    try {
        $resp = $httpClient.SendAsync($logoutRequest).GetAwaiter().GetResult()
        $status = [int]$resp.StatusCode
        $body = $resp.Content.ReadAsStringAsync().GetAwaiter().GetResult()
    } catch {
        $status = -1
        $body = $_.Exception.Message
    }
    Write-Host "[check] POST http://127.0.0.1:3000/api/v0/auth/logout -> $status" -ForegroundColor Cyan
    Write-RedactedHost $body
    if ($status -ne 200) {
        $failures += "auth/logout"
    }

    $expiredToken = $token
    try {
        $escapedUsername = $username.Replace("'", "''")
        & psql $databaseUrl -v ON_ERROR_STOP=1 -c "UPDATE auth_sessions SET expires_at = NOW() - interval '1 day' WHERE account_id = (SELECT account_id FROM accounts WHERE username = '$escapedUsername');" | Out-Null
    } catch {
        $failures += "auth/expire-token"
    }

    $expiredRequest = New-Object System.Net.Http.HttpRequestMessage("GET", "http://127.0.0.1:3000/api/v0/private/ideas")
    $expiredRequest.Headers.Authorization = New-Object System.Net.Http.Headers.AuthenticationHeaderValue("Bearer", $expiredToken)
    try {
        $resp = $httpClient.SendAsync($expiredRequest).GetAwaiter().GetResult()
        $status = [int]$resp.StatusCode
        $body = $resp.Content.ReadAsStringAsync().GetAwaiter().GetResult()
    } catch {
        $status = -1
        $body = $_.Exception.Message
    }
    Write-Host "[check] GET http://127.0.0.1:3000/api/v0/private/ideas (expired token) -> $status" -ForegroundColor Cyan
    Write-RedactedHost $body
    if ($status -ne 401) {
        $failures += "auth/expired-token"
    }
}

$canonicalAfterPrivate = Get-CanonicalCounts -DatabaseUrl $databaseUrl
if (CanonicalCountsChanged -Before $canonicalBeforeAuth -After $canonicalAfterPrivate) {
    $failures += "private-lane/canonical-write"
}

$seedVerifierUsername = Get-EnvValue @("SEED_OWNER_USERNAME", "seed_owner_username")
$seedVerifierPassword = Get-EnvValue @("SEED_OWNER_PASSWORD", "seed_owner_password")
$seedVerifierToken = Resolve-SeedVerifierToken -OwnerUsername $seedVerifierUsername -OwnerPassword $seedVerifierPassword
if ([string]::IsNullOrWhiteSpace([string]$seedVerifierToken)) {
    $failures += "stage1/verifier-auth"
}

$writerUsername = "stage1_writer_$userSuffix"
$writerPassword = "stage1-writer-pass-$userSuffix"
$writerToken = $null
$writerAccountId = $null
$writerIdentityId = ("00000000-0000-7000-8000-{0}" -f (([int64]($userSuffix % 1000000000000)).ToString("x12")))
$writerTitle = "stage1-writer-$userSuffix"

$writerReg = Invoke-Json -Method POST -Url "http://127.0.0.1:3000/api/v0/auth/register" -Body @{ username = $writerUsername; password = $writerPassword } -ExpectedStatus 200
if (-not $writerReg[0]) {
    $failures += "stage1/register"
} else {
    $writerToken = $writerReg[1].token
    $writerAccountId = $writerReg[1].account_id
}

if (-not [string]::IsNullOrWhiteSpace([string]$writerAccountId)) {
    try {
        & psql $databaseUrl -v ON_ERROR_STOP=1 -c "UPDATE accounts SET canonical_identity_id = '$writerIdentityId'::uuid WHERE account_id = '$writerAccountId'::uuid;" | Out-Null
        & psql $databaseUrl -v ON_ERROR_STOP=1 -c "INSERT INTO identities_s0 (identity_id, title, created_event_id) VALUES ('$writerIdentityId'::uuid, '$writerTitle', '$writerIdentityId'::uuid) ON CONFLICT (identity_id) DO NOTHING;" | Out-Null
    } catch {
        $failures += "stage1/writer-bind"
    }
}

if (-not [string]::IsNullOrWhiteSpace([string]$writerToken)) {
    $unverifiedCreate = Invoke-JsonAuth -Method POST -Url "http://127.0.0.1:3000/api/v1/canonical/ideas" -Body @{
        idea_type = "conceptual_idea"
        title = "unverified write"
        sentence = "should be rejected"
    } -Token $writerToken -ExpectedStatus 403
    if (-not $unverifiedCreate[0]) {
        $failures += "stage1/unverified-reject"
    }

    $writerGrant = Invoke-JsonAuth -Method POST -Url "http://127.0.0.1:3000/api/v1/canonical/verifier/grants" -Body @{
        identity_id = $writerIdentityId
        canonical_writer_level = "1"
        email_verified = $true
    } -Token $seedVerifierToken -ExpectedStatus 200
    if (-not $writerGrant[0]) {
        $failures += "stage1/writer-verify"
    }

    $malformedCreate = Invoke-JsonAuth -Method POST -Url "http://127.0.0.1:3000/api/v1/canonical/ideas" -Body @{
        idea_type = "invalid_type"
        title = "bad"
        sentence = "bad"
    } -Token $writerToken -ExpectedStatus 400
    if (-not $malformedCreate[0]) {
        $failures += "stage1/malformed-reject"
    }

    $successfulWrites = 0
    for ($i = 0; $i -lt 20; $i++) {
        $write = Invoke-JsonAuth -Method POST -Url "http://127.0.0.1:3000/api/v1/canonical/ideas" -Body @{
            idea_type = "conceptual_idea"
            title = "stage1 idea $i $userSuffix"
            sentence = "stage1 deterministic sentence $i"
        } -Token $writerToken -ExpectedStatus 200
        if ($write[0]) {
            $successfulWrites++
        } else {
            $failures += "stage1/mana-seed/$i"
            break
        }
    }

    if ($successfulWrites -eq 20) {
        $insufficient = Invoke-JsonAuth -Method POST -Url "http://127.0.0.1:3000/api/v1/canonical/ideas" -Body @{
            idea_type = "conceptual_idea"
            title = "stage1 over cap"
            sentence = "must fail due to mana"
        } -Token $writerToken -ExpectedStatus 409
        if (-not $insufficient[0]) {
            $failures += "stage1/mana-reject"
        }
    }

    $challengeWriterUsername = "s1_ch_writer_$userSuffix"
    $challengeWriterPassword = "stage1-challenge-pass-$userSuffix"
    $challengeWriterToken = $null
    $challengeWriterAccountId = $null
    $challengeWriterIdentityId = ("00000000-0000-7000-8000-{0}" -f (([int64](($userSuffix + 11) % 1000000000000)).ToString("x12")))
    $challengeWriterTitle = "stage1-challenge-writer-$userSuffix"

    $challengeWriterReg = Invoke-Json -Method POST -Url "http://127.0.0.1:3000/api/v0/auth/register" -Body @{ username = $challengeWriterUsername; password = $challengeWriterPassword } -ExpectedStatus 200
    if (-not $challengeWriterReg[0]) {
        $failures += "stage1/challenge/register"
    } else {
        $challengeWriterToken = $challengeWriterReg[1].token
        $challengeWriterAccountId = $challengeWriterReg[1].account_id
    }

    if (-not [string]::IsNullOrWhiteSpace([string]$challengeWriterAccountId)) {
        try {
            & psql $databaseUrl -v ON_ERROR_STOP=1 -c "UPDATE accounts SET canonical_identity_id = '$challengeWriterIdentityId'::uuid WHERE account_id = '$challengeWriterAccountId'::uuid;" | Out-Null
            & psql $databaseUrl -v ON_ERROR_STOP=1 -c "INSERT INTO identities_s0 (identity_id, title, created_event_id) VALUES ('$challengeWriterIdentityId'::uuid, '$challengeWriterTitle', '$challengeWriterIdentityId'::uuid) ON CONFLICT (identity_id) DO NOTHING;" | Out-Null
        } catch {
            $failures += "stage1/challenge/writer-bind"
        }
        $challengeGrant = Invoke-JsonAuth -Method POST -Url "http://127.0.0.1:3000/api/v1/canonical/verifier/grants" -Body @{
            identity_id = $challengeWriterIdentityId
            canonical_writer_level = "1"
            email_verified = $true
        } -Token $seedVerifierToken -ExpectedStatus 200
        if (-not $challengeGrant[0]) {
            $failures += "stage1/challenge/writer-grant"
        }
    }

    if (-not [string]::IsNullOrWhiteSpace([string]$challengeWriterToken)) {
        $framingRepresentationId = Get-FirstRepresentationId -DatabaseUrl $databaseUrl
        $useFallbackFramingRepresentation = $false
        if ([string]::IsNullOrWhiteSpace([string]$framingRepresentationId)) {
            $framingRepresentationId = "00000000-0000-7000-8000-00000000fafe"
            $useFallbackFramingRepresentation = $true
            Write-Host "[check] no framing representation row found; using deterministic fallback id $framingRepresentationId" -ForegroundColor Yellow
        }
        $ideaA = Invoke-JsonAuth -Method POST -Url "http://127.0.0.1:3000/api/v1/canonical/ideas" -Body @{
            idea_type = "conceptual_idea"
            title = "stage1 challenge idea a $userSuffix"
            sentence = "stage1 challenge idea a sentence $userSuffix"
        } -Token $challengeWriterToken -ExpectedStatus 200
        $ideaB = Invoke-JsonAuth -Method POST -Url "http://127.0.0.1:3000/api/v1/canonical/ideas" -Body @{
            idea_type = "conceptual_idea"
            title = "stage1 challenge idea b $userSuffix"
            sentence = "stage1 challenge idea b sentence $userSuffix"
        } -Token $challengeWriterToken -ExpectedStatus 200
        $argumentIdea = Invoke-JsonAuth -Method POST -Url "http://127.0.0.1:3000/api/v1/canonical/ideas" -Body @{
            idea_type = "conceptual_idea"
            title = "stage1 challenge argument $userSuffix"
            sentence = "stage1 challenge argument sentence $userSuffix"
        } -Token $challengeWriterToken -ExpectedStatus 200
        if (-not $ideaA[0] -or -not $ideaB[0] -or -not $argumentIdea[0]) {
            $failures += "stage1/challenge/idea-seed"
        } else {
            $ideaAId = $ideaA[1].idea_id
            $ideaBId = $ideaB[1].idea_id
            $argumentIdeaId = $argumentIdea[1].idea_id
            if ([string]::IsNullOrWhiteSpace([string]$ideaAId) -or [string]::IsNullOrWhiteSpace([string]$ideaBId) -or [string]::IsNullOrWhiteSpace([string]$argumentIdeaId)) {
                $failures += "stage1/challenge/idea-seed-shape"
            } else {
                if ($useFallbackFramingRepresentation) {
                    try {
                        & psql $databaseUrl -v ON_ERROR_STOP=1 -c @"
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
  'stage1 verify fallback framing representation',
  '$challengeWriterIdentityId'::uuid,
  NULL,
  'verify-stage0-fallback',
  e.block_height,
  e.event_index,
  '$framingRepresentationId'::uuid
FROM events e
ORDER BY e.block_height, e.event_index
LIMIT 1
ON CONFLICT (representation_id) DO NOTHING;
"@ | Out-Null
                    } catch {
                        $failures += "stage1/challenge/fallback-framing-insert"
                    }
                }
                $challengeCreate = Invoke-JsonAuth -Method POST -Url "http://127.0.0.1:3000/api/v1/canonical/challenges/importance" -Body @{
                    framing_representation_ref = $framingRepresentationId
                    context_key = "universal:default"
                    axis = "important_to_humanity"
                    timeframe = "medium_term"
                    scope = "universal"
                    target_left_idea_id = $ideaAId
                    target_right_idea_id = $ideaBId
                } -Token $challengeWriterToken -ExpectedStatus 200
                if (-not $challengeCreate[0]) {
                    $failures += "stage1/challenge/create"
                } else {
                    $challengeId = $challengeCreate[1].challenge_id
                    if ([string]::IsNullOrWhiteSpace([string]$challengeId)) {
                        $failures += "stage1/challenge/create-shape"
                    } else {
                        $argumentAttach = Invoke-JsonAuth -Method POST -Url "http://127.0.0.1:3000/api/v1/canonical/challenges/$challengeId/arguments" -Body @{
                            argument_idea_id = $argumentIdeaId
                            subject_idea_id = $ideaAId
                        } -Token $challengeWriterToken -ExpectedStatus 200
                        if (-not $argumentAttach[0]) {
                            $failures += "stage1/challenge/attach"
                        }

                        $challengeDetailRequest = New-Object System.Net.Http.HttpRequestMessage("GET", "http://127.0.0.1:3000/api/v1/canonical/challenges/$challengeId")
                        try {
                            $resp = $httpClient.SendAsync($challengeDetailRequest).GetAwaiter().GetResult()
                            $status = [int]$resp.StatusCode
                            $body = $resp.Content.ReadAsStringAsync().GetAwaiter().GetResult()
                        } catch {
                            $status = -1
                            $body = $_.Exception.Message
                        }
                        Write-Host "[check] GET http://127.0.0.1:3000/api/v1/canonical/challenges/$challengeId -> $status" -ForegroundColor Cyan
                        Write-RedactedHost $body
                        if ($status -ne 200) {
                            $failures += "stage1/challenge/detail"
                        } else {
                            try {
                                $json = $body | ConvertFrom-Json -ErrorAction Stop
                                if ($null -eq $json.challenge) { $failures += "stage1/challenge/detail-shape" }
                                elseif ($null -eq $json.challenge.arguments -or $json.challenge.arguments.Count -lt 1) {
                                    $failures += "stage1/challenge/detail-arguments"
                                }
                            } catch {
                                $failures += "stage1/challenge/detail-parse"
                            }
                        }

                        $challengeDuplicate = Invoke-JsonAuth -Method POST -Url "http://127.0.0.1:3000/api/v1/canonical/challenges/importance" -Body @{
                            framing_representation_ref = $framingRepresentationId
                            context_key = "universal:default"
                            axis = "important_to_humanity"
                            timeframe = "medium_term"
                            scope = "universal"
                            target_left_idea_id = $ideaAId
                            target_right_idea_id = $ideaBId
                        } -Token $challengeWriterToken -ExpectedStatus 409
                        if (-not $challengeDuplicate[0]) {
                            $failures += "stage1/challenge/duplicate-reject"
                        }

                        $cycleNoopEventId = New-CanonicalUuidV7
                        $cycleCloseEventId = New-CanonicalUuidV7
                        $cycleCloseKind = Resolve-CycleCloseKind
                        if ([string]::IsNullOrWhiteSpace([string]$cycleCloseKind)) {
                            $failures += "stage1/challenge/cycle-advance/resolve-kind"
                        } elseif (-not (Insert-ForcedCycleClose -DatabaseUrl $databaseUrl -NoopEventId $cycleNoopEventId -CycleCloseEventId $cycleCloseEventId -NoopSpeakerIdentityId $challengeWriterIdentityId -ClosureKind $cycleCloseKind)) {
                            $failures += "stage1/challenge/cycle-advance"
                        } else {
                            function Register-CanonicalVoter {
                                param(
                                    [string]$Username,
                                    [string]$Password,
                                    [string]$IdentityId,
                                    [string]$IdentityTitle
                                )
                                $reg = Invoke-Json -Method POST -Url "http://127.0.0.1:3000/api/v0/auth/register" -Body @{ username = $Username; password = $Password } -ExpectedStatus 200
                                if (-not $reg[0]) { return $null }
                                $accountId = [string]$reg[1].account_id
                                $tokenValue = [string]$reg[1].token
                                if ([string]::IsNullOrWhiteSpace($accountId) -or [string]::IsNullOrWhiteSpace($tokenValue)) { return $null }
                                try {
                                    & psql $databaseUrl -v ON_ERROR_STOP=1 -c "UPDATE accounts SET canonical_identity_id = '$IdentityId'::uuid WHERE account_id = '$accountId'::uuid;" | Out-Null
                                    & psql $databaseUrl -v ON_ERROR_STOP=1 -c "INSERT INTO identities_s0 (identity_id, title, created_event_id) VALUES ('$IdentityId'::uuid, '$IdentityTitle', '$IdentityId'::uuid) ON CONFLICT (identity_id) DO NOTHING;" | Out-Null
                                } catch {
                                    return $null
                                }
                                $grant = Invoke-JsonAuth -Method POST -Url "http://127.0.0.1:3000/api/v1/canonical/verifier/grants" -Body @{
                                    identity_id = $IdentityId
                                    canonical_writer_level = "1"
                                    email_verified = $true
                                } -Token $seedVerifierToken -ExpectedStatus 200
                                if (-not $grant[0]) {
                                    return $null
                                }
                                return $tokenValue
                            }

                            $voter1Token = Register-CanonicalVoter -Username ("s1_vote_u1_{0}" -f $userSuffix) -Password ("s1-vote-pass-1-{0}" -f $userSuffix) -IdentityId "00000000-0000-7000-8000-00000000d921" -IdentityTitle "stage1 vote user 1"
                            $voter2Token = Register-CanonicalVoter -Username ("s1_vote_u2_{0}" -f $userSuffix) -Password ("s1-vote-pass-2-{0}" -f $userSuffix) -IdentityId "00000000-0000-7000-8000-00000000d922" -IdentityTitle "stage1 vote user 2"
                            $voter3Token = Register-CanonicalVoter -Username ("s1_vote_u3_{0}" -f $userSuffix) -Password ("s1-vote-pass-3-{0}" -f $userSuffix) -IdentityId "00000000-0000-7000-8000-00000000d923" -IdentityTitle "stage1 vote user 3"
                            if ([string]::IsNullOrWhiteSpace([string]$voter1Token) -or [string]::IsNullOrWhiteSpace([string]$voter2Token) -or [string]::IsNullOrWhiteSpace([string]$voter3Token)) {
                                $failures += "stage1/voting/register-voters"
                            } else {
                                function Resolve-VoteSessionForChallenge {
                                    param(
                                        [object[]]$InitialPull,
                                        [string]$TargetChallengeId,
                                        [string]$Token,
                                        [int]$MaxAdditionalPulls
                                    )

                                    if ($InitialPull[0] -and [string]$InitialPull[1].challenge_id -eq $TargetChallengeId) {
                                        return $InitialPull
                                    }

                                    for ($attemptIdx = 0; $attemptIdx -lt $MaxAdditionalPulls; $attemptIdx++) {
                                        $nextPull = Invoke-JsonAuth -Method POST -Url "http://127.0.0.1:3000/api/v1/canonical/vote-sessions/pull" -Body @{} -Token $Token -ExpectedStatus 200
                                        if (-not $nextPull[0]) {
                                            return $null
                                        }
                                        if ([string]$nextPull[1].challenge_id -eq $TargetChallengeId) {
                                            return $nextPull
                                        }
                                    }

                                    return $null
                                }

                                $pull1 = Invoke-JsonAuth -Method POST -Url "http://127.0.0.1:3000/api/v1/canonical/vote-sessions/pull" -Body @{} -Token $voter1Token -ExpectedStatus 200
                                $pull2 = Invoke-JsonAuth -Method POST -Url "http://127.0.0.1:3000/api/v1/canonical/vote-sessions/pull" -Body @{} -Token $voter2Token -ExpectedStatus 200
                                $pull3 = Invoke-JsonAuth -Method POST -Url "http://127.0.0.1:3000/api/v1/canonical/vote-sessions/pull" -Body @{} -Token $voter3Token -ExpectedStatus 200
                                if (-not $pull1[0] -or -not $pull2[0] -or -not $pull3[0]) {
                                    $failures += "stage1/voting/pull"
                                } else {
                                    $assignedChallenge = [string]$pull1[1].challenge_id
                                    if ([string]::IsNullOrWhiteSpace($assignedChallenge)) {
                                        $failures += "stage1/voting/pull-shape"
                                    } else {
                                        $pull2Resolved = Resolve-VoteSessionForChallenge -InitialPull $pull2 -TargetChallengeId $assignedChallenge -Token $voter2Token -MaxAdditionalPulls 2
                                        $pull3Resolved = Resolve-VoteSessionForChallenge -InitialPull $pull3 -TargetChallengeId $assignedChallenge -Token $voter3Token -MaxAdditionalPulls 2
                                        if ($null -eq $pull2Resolved -or $null -eq $pull3Resolved) {
                                            $failures += "stage1/voting/pull-shape"
                                        } else {
                                            $vote1 = Invoke-JsonAuth -Method POST -Url "http://127.0.0.1:3000/api/v1/canonical/challenges/$assignedChallenge/votes" -Body @{
                                                vote_session_id = [string]$pull1[1].vote_session_id
                                                vote_choice = "left"
                                            } -Token $voter1Token -ExpectedStatus 200
                                            $vote2 = Invoke-JsonAuth -Method POST -Url "http://127.0.0.1:3000/api/v1/canonical/challenges/$assignedChallenge/votes" -Body @{
                                                vote_session_id = [string]$pull2Resolved[1].vote_session_id
                                                vote_choice = "left"
                                            } -Token $voter2Token -ExpectedStatus 200
                                            $vote3 = Invoke-JsonAuth -Method POST -Url "http://127.0.0.1:3000/api/v1/canonical/challenges/$assignedChallenge/votes" -Body @{
                                                vote_session_id = [string]$pull3Resolved[1].vote_session_id
                                                vote_choice = "right"
                                            } -Token $voter3Token -ExpectedStatus 200
                                            if (-not $vote1[0] -or -not $vote2[0] -or -not $vote3[0]) {
                                                $failures += "stage1/voting/cast"
                                            } else {
                                                if ([string]::IsNullOrWhiteSpace([string]$vote3[1].verdict_event_id)) {
                                                    $failures += "stage1/voting/verdict-missing"
                                                }
                                                $duplicateVote = Invoke-JsonAuth -Method POST -Url "http://127.0.0.1:3000/api/v1/canonical/challenges/$assignedChallenge/votes" -Body @{
                                                    vote_session_id = [string]$pull1[1].vote_session_id
                                                    vote_choice = "left"
                                                } -Token $voter1Token -ExpectedStatus 409
                                                if (-not $duplicateVote[0]) {
                                                    $failures += "stage1/voting/duplicate-reject"
                                                }

                                                $challengeDetailAfterVotes = New-Object System.Net.Http.HttpRequestMessage("GET", "http://127.0.0.1:3000/api/v1/canonical/challenges/$assignedChallenge")
                                                try {
                                                    $resp = $httpClient.SendAsync($challengeDetailAfterVotes).GetAwaiter().GetResult()
                                                    $status = [int]$resp.StatusCode
                                                    $body = $resp.Content.ReadAsStringAsync().GetAwaiter().GetResult()
                                                } catch {
                                                    $status = -1
                                                    $body = $_.Exception.Message
                                                }
                                                Write-Host "[check] GET http://127.0.0.1:3000/api/v1/canonical/challenges/$assignedChallenge -> $status" -ForegroundColor Cyan
                                                Write-RedactedHost $body
                                                if ($status -ne 200) {
                                                    $failures += "stage1/voting/detail"
                                                } else {
                                                    try {
                                                        $json = $body | ConvertFrom-Json -ErrorAction Stop
                                                        if ($null -eq $json.challenge.verdict) {
                                                            $failures += "stage1/voting/detail-verdict"
                                                        }
                                                        if ($null -eq $json.challenge.votes -or $json.challenge.votes.Count -lt 3) {
                                                            $failures += "stage1/voting/detail-votes"
                                                        }
                                                    } catch {
                                                        $failures += "stage1/voting/detail-parse"
                                                    }
                                                }

                                                for ($capIdx = 1; $capIdx -le 4; $capIdx++) {
                                                    $capCreate = Invoke-JsonAuth -Method POST -Url "http://127.0.0.1:3000/api/v1/canonical/challenges/importance" -Body @{
                                                        framing_representation_ref = $framingRepresentationId
                                                        context_key = ("universal:capacity:{0}:{1}" -f $capIdx, $userSuffix)
                                                        axis = "important_to_humanity"
                                                        timeframe = "medium_term"
                                                        scope = "universal"
                                                        target_left_idea_id = $ideaAId
                                                        target_right_idea_id = $ideaBId
                                                    } -Token $challengeWriterToken -ExpectedStatus 200
                                                    if (-not $capCreate[0]) {
                                                        $failures += "stage1/voting/capacity-challenge-$capIdx"
                                                        break
                                                    }
                                                }

                                                $cycleNoopEventId2 = New-CanonicalUuidV7
                                                $cycleCloseEventId2 = New-CanonicalUuidV7
                                                $cycleCloseKind2 = Resolve-CycleCloseKind
                                                if ([string]::IsNullOrWhiteSpace([string]$cycleCloseKind2)) {
                                                    $failures += "stage1/voting/cycle-advance-2/resolve-kind"
                                                } elseif (-not (Insert-ForcedCycleClose -DatabaseUrl $databaseUrl -NoopEventId $cycleNoopEventId2 -CycleCloseEventId $cycleCloseEventId2 -NoopSpeakerIdentityId $challengeWriterIdentityId -ClosureKind $cycleCloseKind2)) {
                                                    $failures += "stage1/voting/cycle-advance-2"
                                                } else {
                                                    $probeToken = Register-CanonicalVoter -Username ("s1_vote_probe_{0}" -f $userSuffix) -Password ("s1-vote-probe-pass-{0}" -f $userSuffix) -IdentityId "00000000-0000-7000-8000-00000000d924" -IdentityTitle "stage1 vote probe"
                                                    if ([string]::IsNullOrWhiteSpace([string]$probeToken)) {
                                                        $failures += "stage1/voting/probe-register"
                                                    } else {
                                                        for ($probeIdx = 1; $probeIdx -le 3; $probeIdx++) {
                                                            $probePull = Invoke-JsonAuth -Method POST -Url "http://127.0.0.1:3000/api/v1/canonical/vote-sessions/pull" -Body @{} -Token $probeToken -ExpectedStatus 200
                                                            if (-not $probePull[0]) {
                                                                $failures += "stage1/voting/probe-pull-$probeIdx"
                                                                break
                                                            }
                                                        }
                                                        $probeInsufficient = Invoke-JsonAuth -Method POST -Url "http://127.0.0.1:3000/api/v1/canonical/vote-sessions/pull" -Body @{} -Token $probeToken -ExpectedStatus 409
                                                        if (-not $probeInsufficient[0]) {
                                                            $failures += "stage1/voting/insufficient-mana-reject"
                                                        }
                                                    }
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    } else {
        $failures += "stage1/challenge/writer-token"
    }
} else {
    $failures += "stage1/writer-token"
}

Write-Host "[verify] stopping api-server job" -ForegroundColor Cyan
try { Stop-Job $job -Force | Out-Null } catch { }
try { Receive-Job $job -Keep | Out-Null } catch { }

if ($failures.Count -gt 0) {
    Write-Host "[verify] failures: $($failures -join ', ')" -ForegroundColor Red
    try {
        $logs = Receive-Job $job -Keep -ErrorAction SilentlyContinue
        if ($logs) {
            Write-Host "[verify] api-server logs:" -ForegroundColor Yellow
            $logs | ForEach-Object { Write-RedactedHost "$_" }
        }
    } catch { }
    Write-Host "FAIL verify-stage0" -ForegroundColor Red
    exit 1
}

Write-Host "[verify] all checks passed" -ForegroundColor Green
Write-Host "PASS verify-stage0" -ForegroundColor Green
