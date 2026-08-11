[CmdletBinding()]
param([string]$AdminDatabaseUrl = $env:SEED_TEST_DATABASE_ADMIN_URL)
$ErrorActionPreference = "Stop"
Set-StrictMode -Version Latest
$Prefix = "seed_opencore_canonical_history_transfer_001_"
$Suffix = [guid]::NewGuid().ToString("N").Substring(0, 10)
$SourceName = $Prefix + "source_" + $Suffix
$TargetName = $Prefix + "target_" + $Suffix
$PreservedPattern = "seed_admission_p3_test_32944_%"
$RepoRoot = (Resolve-Path (Join-Path $PSScriptRoot "..")).Path
$BackendDir = Join-Path $RepoRoot "backend"
$RunRoot = Join-Path ([IO.Path]::GetTempPath()) ("opencore-canonical-history-transfer-" + $Suffix)
$ExportA = Join-Path $RunRoot "export-a"
$ExportB = Join-Path $RunRoot "export-b"
$ExportTarget = Join-Path $RunRoot "export-target"
$SourceArtifacts = Join-Path $RunRoot "source-artifacts"
$TargetArtifacts = Join-Path $RunRoot "target-artifacts"
$Created = New-Object System.Collections.Generic.List[string]
$env:Path = "$env:USERPROFILE\.cargo\bin;$env:Path"

function Admin-Scalar([string]$Sql) {
    $value = & psql $AdminDatabaseUrl -X -q -t -A -v ON_ERROR_STOP=1 -c $Sql 2>&1
    if ($LASTEXITCODE -ne 0) { throw "[canonical-history-db] administrator query failed" }
    return (($value | Out-String).Trim())
}
function Database-Url([string]$Name) {
    $uri = [System.Uri]$AdminDatabaseUrl
    return $uri.GetLeftPart([System.UriPartial]::Authority) + "/" + $Name + $uri.Query
}
function Create-Database([string]$Name) {
    if (-not $Name.StartsWith($Prefix) -or $Name.Length -gt 63) {
        throw "[canonical-history-db] generated name violates exact task prefix"
    }
    $null = & psql $AdminDatabaseUrl -X -q -v ON_ERROR_STOP=1 -c "CREATE DATABASE $Name;" 2>&1
    if ($LASTEXITCODE -ne 0) { throw "[canonical-history-db] create failed" }
    $Created.Add($Name)
    Write-Host "ISOLATED_DB name=$Name prefix_ok=true differs_from_seed_dev=true" -ForegroundColor Cyan
}
function Invoke-Database([string]$Url, [scriptblock]$Action) {
    $prior = $env:DATABASE_URL
    $env:DATABASE_URL = $Url
    try {
        & $Action
        if ($LASTEXITCODE -ne 0) { throw "[canonical-history-db] database command failed" }
    } finally {
        if ([string]::IsNullOrWhiteSpace($prior)) {
            Remove-Item Env:DATABASE_URL -ErrorAction SilentlyContinue
        } else { $env:DATABASE_URL = $prior }
    }
}
function Migrate([string]$Url) {
    Invoke-Database $Url { & (Join-Path $BackendDir "scripts\dev-bootstrap.ps1") }
}
function Compare-Package([string]$Left, [string]$Right) {
    foreach ($name in @("manifest.json", "blocks.ndjson", "events.ndjson")) {
        $leftHash = (Get-FileHash -Algorithm SHA256 (Join-Path $Left $name)).Hash
        $rightHash = (Get-FileHash -Algorithm SHA256 (Join-Path $Right $name)).Hash
        if ($leftHash -ne $rightHash) {
            throw "[canonical-history-db] deterministic package mismatch file=$name"
        }
        Write-Host "PACKAGE_REPEAT file=$name sha256=$leftHash" -ForegroundColor Green
    }
}
function Snapshot-Summary([string]$Url) {
    $sql = "SELECT concat_ws('|',block_height,snapshot_hash,state_root_hash,title_sentence_payload_root,shared_map_commitment,active_rulebook_set_hash,last_event_id,event_count) FROM snapshots ORDER BY block_height DESC LIMIT 1;"
    $value = & psql $Url -X -q -t -A -v ON_ERROR_STOP=1 -c $sql 2>&1
    if ($LASTEXITCODE -ne 0) { throw "[canonical-history-db] snapshot query failed" }
    return (($value | Out-String).Trim())
}
function Canonical-Counts([string]$Url) {
    $sql = "SELECT concat_ws('|',(SELECT COUNT(*) FROM events),(SELECT COUNT(*) FROM ideas),(SELECT COUNT(*) FROM connections),(SELECT COUNT(*) FROM representations),(SELECT COUNT(*) FROM orderings),(SELECT COUNT(*) FROM snapshots));"
    $value = & psql $Url -X -q -t -A -v ON_ERROR_STOP=1 -c $sql 2>&1
    if ($LASTEXITCODE -ne 0) { throw "[canonical-history-db] count query failed" }
    return (($value | Out-String).Trim())
}
function Normalize-ApiJson([string]$Json) {
    $value = $Json | ConvertFrom-Json
    function Remove-RebuildLocalFields($node) {
        if ($null -eq $node) { return }
        if ($node -is [System.Array]) {
            foreach ($item in $node) { Remove-RebuildLocalFields $item }
            return
        }
        if ($node -is [PSCustomObject]) {
            foreach ($name in @("snapshot_id", "artifact_path", "artifact_sha256", "created_at")) {
                $null = $node.PSObject.Properties.Remove($name)
            }
            foreach ($property in @($node.PSObject.Properties)) {
                Remove-RebuildLocalFields $property.Value
            }
        }
    }
    Remove-RebuildLocalFields $value
    return ($value | ConvertTo-Json -Depth 100 -Compress)
}
function Find-ApiDifference($Left, $Right, [string]$Path) {
    if ($null -eq $Left -or $null -eq $Right) {
        if ($null -eq $Left -and $null -eq $Right) { return $null }
        return "$Path null mismatch"
    }
    $leftArray = $Left -is [System.Array]
    $rightArray = $Right -is [System.Array]
    if ($leftArray -or $rightArray) {
        if (-not ($leftArray -and $rightArray)) { return "$Path type mismatch" }
        if ($Left.Count -ne $Right.Count) { return "$Path array length $($Left.Count) != $($Right.Count)" }
        for ($index = 0; $index -lt $Left.Count; $index++) {
            $difference = Find-ApiDifference $Left[$index] $Right[$index] "$Path[$index]"
            if ($difference) { return $difference }
        }
        return $null
    }
    $leftObject = $Left -is [PSCustomObject]
    $rightObject = $Right -is [PSCustomObject]
    if ($leftObject -or $rightObject) {
        if (-not ($leftObject -and $rightObject)) { return "$Path type mismatch" }
        $names = @($Left.PSObject.Properties.Name + $Right.PSObject.Properties.Name | Sort-Object -Unique)
        foreach ($name in $names) {
            $leftProperty = $Left.PSObject.Properties[$name]
            $rightProperty = $Right.PSObject.Properties[$name]
            if ($null -eq $leftProperty -or $null -eq $rightProperty) { return "$Path.$name property mismatch" }
            $difference = Find-ApiDifference $leftProperty.Value $rightProperty.Value "$Path.$name"
            if ($difference) { return $difference }
        }
        return $null
    }
    $leftScalar = $Left | ConvertTo-Json -Compress -Depth 20
    $rightScalar = $Right | ConvertTo-Json -Compress -Depth 20
    if ($leftScalar -ne $rightScalar) {
        return "$Path value $leftScalar != $rightScalar"
    }
    return $null
}

function Capture-Api([string]$Url) {
    $prior = $env:DATABASE_URL
    $env:DATABASE_URL = $Url
    $stdout = Join-Path $RunRoot ("api-" + [guid]::NewGuid().ToString("N") + ".out")
    $stderr = Join-Path $RunRoot ("api-" + [guid]::NewGuid().ToString("N") + ".err")
    $proc = $null
    $client = $null
    try {
        $proc = Start-Process -FilePath (Join-Path $BackendDir "target\debug\api-server.exe") -WorkingDirectory $BackendDir -WindowStyle Hidden -PassThru -RedirectStandardOutput $stdout -RedirectStandardError $stderr
        $client = New-Object System.Net.Http.HttpClient
        $client.Timeout = [TimeSpan]::FromSeconds(5)
        $ready = $false
        for ($attempt = 0; $attempt -lt 40; $attempt++) {
            if ($proc.HasExited) { break }
            try {
                $health = $client.GetAsync("http://127.0.0.1:3000/api/v0/health").GetAwaiter().GetResult()
                if ([int]$health.StatusCode -eq 200) { $ready = $true; break }
            } catch {}
            Start-Sleep -Milliseconds 250
        }
        if (-not $ready -or $proc.HasExited) {
            throw "[canonical-history-db] owned API server did not become ready"
        }
        $snapshotHeight = ((& psql $Url -X -q -t -A -v ON_ERROR_STOP=1 -c "SELECT block_height FROM snapshots ORDER BY block_height DESC LIMIT 1;" 2>&1) | Out-String).Trim()
        $capabilities = $client.GetStringAsync("http://127.0.0.1:3000/api/v0/capabilities").GetAwaiter().GetResult()
        $snapshot = $client.GetStringAsync("http://127.0.0.1:3000/api/v0/snapshot/$snapshotHeight").GetAwaiter().GetResult()
        $ideas = $client.GetStringAsync("http://127.0.0.1:3000/api/v0/ideas/top?snapshot_height=$snapshotHeight&limit=10&offset=0&order=asc").GetAwaiter().GetResult()
        return [PSCustomObject]@{
            capabilities = Normalize-ApiJson $capabilities
            snapshot_height = Normalize-ApiJson $snapshot
            ideas_top_snapshot_height = Normalize-ApiJson $ideas
        }
    } finally {
        if ($client) { $client.Dispose() }
        if ($proc -and -not $proc.HasExited) {
            Stop-Process -Id $proc.Id -Force
            $proc.WaitForExit()
        }
        if ([string]::IsNullOrWhiteSpace($prior)) {
            Remove-Item Env:DATABASE_URL -ErrorAction SilentlyContinue
        } else { $env:DATABASE_URL = $prior }
    }
}

if ([string]::IsNullOrWhiteSpace($AdminDatabaseUrl)) {
    throw "[canonical-history-db] SEED_TEST_DATABASE_ADMIN_URL is required"
}
if (-not (Get-Command psql -ErrorAction SilentlyContinue)) {
    throw "[canonical-history-db] psql not found"
}
try { $adminUri = [System.Uri]$AdminDatabaseUrl } catch {
    throw "[canonical-history-db] invalid administrator URL"
}
if ($adminUri.AbsolutePath.Trim("/") -ne "postgres") {
    throw "[canonical-history-db] administrator URL must target postgres"
}
$preservedBefore = Admin-Scalar "SELECT count(*) FROM pg_database WHERE datname LIKE '$PreservedPattern';"
if ($preservedBefore -ne "2") {
    throw "[canonical-history-db] expected exactly two preserved admission databases"
}

try {
    New-Item -ItemType Directory -Path $RunRoot | Out-Null
    Create-Database $SourceName
    Create-Database $TargetName
    $SourceUrl = Database-Url $SourceName
    $TargetUrl = Database-Url $TargetName
    Migrate $SourceUrl
    Migrate $TargetUrl

    Push-Location $BackendDir
    try {
        cargo build -p canonical-history-transfer -p seed-importer -p snapshot-builder -p api-server
        if ($LASTEXITCODE -ne 0) { throw "[canonical-history-db] binary build failed" }
        cargo test -p canonical-history --all-targets
        if ($LASTEXITCODE -ne 0) { throw "[canonical-history-db] negative fixture tests failed" }

        Invoke-Database $SourceUrl {
            & (Join-Path $BackendDir "target\debug\seed-importer.exe") --file (Join-Path $RepoRoot "seed\seed-data-v0.json")
        }
        $env:SEED_SNAPSHOT_ARTIFACT_BASE_DIR = $SourceArtifacts
        Invoke-Database $SourceUrl { & (Join-Path $BackendDir "target\debug\snapshot-builder.exe") }
        Remove-Item Env:SEED_SNAPSHOT_ARTIFACT_BASE_DIR -ErrorAction SilentlyContinue

        $transfer = Join-Path $BackendDir "target\debug\canonical-history-transfer.exe"
        & $transfer export --database-url $SourceUrl --output $ExportA
        if ($LASTEXITCODE -ne 0) { throw "[canonical-history-db] first export failed" }
        & $transfer export --database-url $SourceUrl --output $ExportB
        if ($LASTEXITCODE -ne 0) { throw "[canonical-history-db] repeat export failed" }
        Compare-Package $ExportA $ExportB

        & $transfer validate --package $ExportA
        if ($LASTEXITCODE -ne 0) { throw "[canonical-history-db] validate-only failed" }
        & $transfer import --package $ExportA --validate-only
        if ($LASTEXITCODE -ne 0) { throw "[canonical-history-db] import validate-only failed" }

        & $transfer import --package $ExportA --database-url $TargetUrl --confirm-fresh-target $TargetName
        if ($LASTEXITCODE -ne 0) { throw "[canonical-history-db] import failed" }
        & $transfer import --package $ExportA --database-url $TargetUrl --confirm-fresh-target $TargetName
        if ($LASTEXITCODE -ne 0) { throw "[canonical-history-db] idempotent retry failed" }

        $env:SEED_SNAPSHOT_ARTIFACT_BASE_DIR = $TargetArtifacts
        Invoke-Database $TargetUrl { & (Join-Path $BackendDir "target\debug\snapshot-builder.exe") }
        Remove-Item Env:SEED_SNAPSHOT_ARTIFACT_BASE_DIR -ErrorAction SilentlyContinue

        & $transfer export --database-url $TargetUrl --output $ExportTarget
        if ($LASTEXITCODE -ne 0) { throw "[canonical-history-db] target export failed" }
        Compare-Package $ExportA $ExportTarget

        $sourceSnapshot = Snapshot-Summary $SourceUrl
        $targetSnapshot = Snapshot-Summary $TargetUrl
        if ($sourceSnapshot -ne $targetSnapshot) {
            throw "[canonical-history-db] replay/snapshot commitment summaries differ"
        }
        Write-Host "ROUNDTRIP_SNAPSHOT match=true value=$sourceSnapshot" -ForegroundColor Green
        $sourceCounts = Canonical-Counts $SourceUrl
        $targetCounts = Canonical-Counts $TargetUrl
        if ($sourceCounts -ne $targetCounts) {
            throw "[canonical-history-db] canonical/projection counts differ"
        }
        Write-Host "ROUNDTRIP_COUNTS match=true events_ideas_connections_representations_orderings_snapshots=$sourceCounts" -ForegroundColor Green


        Add-Type -AssemblyName System.Net.Http
        $sourceApi = Capture-Api $SourceUrl
        $targetApi = Capture-Api $TargetUrl
        $apiMismatch = $false
        foreach ($endpointName in @("capabilities", "snapshot_height", "ideas_top_snapshot_height")) {
            $sourceValue = [string]$sourceApi.$endpointName
            $targetValue = [string]$targetApi.$endpointName
            if ($sourceValue -cne $targetValue) {
                $difference = Find-ApiDifference ($sourceValue | ConvertFrom-Json) ($targetValue | ConvertFrom-Json) '$'
                Write-Host "API_DIFFERENCE endpoint=$endpointName first=$difference" -ForegroundColor Yellow
                $apiMismatch = $true
            }
        }
        if ($apiMismatch) {
            throw "[canonical-history-db] fixed-height API responses differ"
        }
        Write-Host "ROUNDTRIP_API match=true endpoints=capabilities,snapshot_height,ideas_top_snapshot_height" -ForegroundColor Green
    } finally { Pop-Location }
    Write-Host "PASS canonical-history-transfer two_database_roundtrip=true" -ForegroundColor Green
} finally {
    Remove-Item Env:SEED_SNAPSHOT_ARTIFACT_BASE_DIR -ErrorAction SilentlyContinue
    foreach ($name in $Created) {
        $null = & psql $AdminDatabaseUrl -X -q -v ON_ERROR_STOP=1 -c "SELECT pg_terminate_backend(pid) FROM pg_stat_activity WHERE datname = '$name' AND pid <> pg_backend_pid();" 2>&1
        $null = & psql $AdminDatabaseUrl -X -q -v ON_ERROR_STOP=1 -c "DROP DATABASE IF EXISTS $name;" 2>&1
        if ((Admin-Scalar "SELECT count(*) FROM pg_database WHERE datname = '$name';") -ne "0") {
            throw "[canonical-history-db] cleanup verification failed name=$name"
        }
        Write-Host "ISOLATED_DB_CLEANUP name=$name dropped=true" -ForegroundColor Cyan
    }
    if (Test-Path $RunRoot) { Remove-Item -LiteralPath $RunRoot -Recurse -Force }
    $remaining = Admin-Scalar "SELECT count(*) FROM pg_database WHERE datname LIKE '$Prefix%';"
    if ($remaining -ne "0") { throw "[canonical-history-db] task-prefixed databases remain" }
    $preservedAfter = Admin-Scalar "SELECT count(*) FROM pg_database WHERE datname LIKE '$PreservedPattern';"
    if ($preservedAfter -ne $preservedBefore) {
        throw "[canonical-history-db] preserved admission database count changed"
    }
    Write-Host "PRESERVED_DATABASES pattern=$PreservedPattern before=$preservedBefore after=$preservedAfter" -ForegroundColor Cyan
}
