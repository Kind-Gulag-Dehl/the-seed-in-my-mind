param(
    [switch]$SkipMigrations
)

# Stage 0 Windows bootstrap for Rust + Postgres env
$env:Path = "$env:USERPROFILE\.cargo\bin;$env:Path"

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
    $value = [regex]::Replace($value, '(?im)(database_url\s*=\s*)[^\s"''`r`n]+', '$1[REDACTED]')
    $value = [regex]::Replace($value, '(?im)(pgpassfile\s*=\s*)[^\s"''`r`n]+', '$1[REDACTED]')
    $value = [regex]::Replace($value, '(?im)(\b(token|secret|password|api[_-]?key)\b\s*[=:]\s*)[^\s,;]+', '$1[REDACTED]')
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

Write-Host "[bootstrap] cargo location:" -ForegroundColor Cyan
where.exe cargo | ForEach-Object { Write-Host "  $_" }
Write-Host "[bootstrap] cargo version:" -ForegroundColor Cyan
cargo --version

$envPath = Join-Path $PSScriptRoot "..\.env"
if (Test-Path $envPath) {
    Write-Host "[bootstrap] loading env from $envPath" -ForegroundColor Cyan
    Get-Content $envPath | ForEach-Object {
        $line = $_.Trim()
        if ($line.Length -eq 0) { return }
        if ($line.StartsWith("#")) { return }
        $parts = $line.Split("=", 2)
        if ($parts.Length -ne 2) { return }
        $key = $parts[0].Trim()
        $value = $parts[1].Trim()
        if ($key.Length -eq 0) { return }
        Set-Item -Path "Env:$key" -Value $value
    }
} else {
    Write-Host "[bootstrap] no .env found at $envPath" -ForegroundColor Yellow
}

function Copy-EnvIfMissing {
    param(
        [string]$SourceKey,
        [string]$TargetKey
    )
    $targetValue = [Environment]::GetEnvironmentVariable($TargetKey, "Process")
    if (-not [string]::IsNullOrWhiteSpace($targetValue)) {
        return
    }
    $sourceValue = [Environment]::GetEnvironmentVariable($SourceKey, "Process")
    if (-not [string]::IsNullOrWhiteSpace($sourceValue)) {
        [Environment]::SetEnvironmentVariable($TargetKey, $sourceValue, "Process")
        Write-Host "[bootstrap] mapped $SourceKey -> $TargetKey" -ForegroundColor DarkGray
    }
}

Copy-EnvIfMissing "database_url" "DATABASE_URL"
Copy-EnvIfMissing "pgpassfile" "PGPASSFILE"
Copy-EnvIfMissing "seed_owner_bootstrap" "SEED_OWNER_BOOTSTRAP"
Copy-EnvIfMissing "seed_owner_username" "SEED_OWNER_USERNAME"
Copy-EnvIfMissing "seed_owner_password" "SEED_OWNER_PASSWORD"
Copy-EnvIfMissing "seed_owner_identity_id" "SEED_OWNER_IDENTITY_ID"
Copy-EnvIfMissing "seed_owner_display_title" "SEED_OWNER_DISPLAY_TITLE"

if (-not [string]::IsNullOrWhiteSpace($env:PGOPTIONS) -and $env:PGOPTIONS -like "*seed.allow_canonical_mutation*") {
    Write-Host "[bootstrap] removing PGOPTIONS seed.allow_canonical_mutation for non-privileged app/runtime paths" -ForegroundColor Yellow
    Remove-Item -Path Env:PGOPTIONS -ErrorAction SilentlyContinue
}

if ([string]::IsNullOrWhiteSpace($env:DATABASE_URL)) {
    Write-Host "[bootstrap] DATABASE_URL is not set." -ForegroundColor Red
    Write-Host "[bootstrap] Create backend/.env from backend/.env.example and set DATABASE_URL." -ForegroundColor Red
    exit 1
}

if (-not [string]::IsNullOrWhiteSpace($env:PGPASSFILE)) {
    if (-not (Test-Path $env:PGPASSFILE)) {
        Write-Host "[bootstrap] PGPASSFILE is set but missing." -ForegroundColor Red
        exit 1
    }
} else {
    Write-Host "[bootstrap] PGPASSFILE not set (optional)." -ForegroundColor Yellow
}

function Redact-DatabaseUrl([string]$url) {
    try {
        $uri = [System.Uri]$url
        $userInfo = $uri.UserInfo
        if ([string]::IsNullOrEmpty($userInfo)) { return $url }
        $user = $userInfo.Split(":")[0]
        $builder = New-Object System.UriBuilder $uri
        $builder.UserName = $user
        $builder.Password = "***"
        return $builder.Uri.AbsoluteUri
    } catch {
        return $url
    }
}

$redacted = Redact-DatabaseUrl $env:DATABASE_URL
Write-RedactedHost "[bootstrap] DATABASE_URL=$redacted" "Green"
if (-not [string]::IsNullOrWhiteSpace($env:PGPASSFILE)) {
    Write-RedactedHost "[bootstrap] PGPASSFILE=[SET]" "Green"
}

$backendDir = Join-Path $PSScriptRoot ".."
$migrationDir = Join-Path $backendDir "migrations\\postgres"

Write-Host "[bootstrap] locating psql" -ForegroundColor Cyan
try {
    where.exe psql | ForEach-Object { Write-Host "  $_" }
} catch {
    Write-Host "[bootstrap] psql not found on PATH." -ForegroundColor Red
    exit 1
}

$uri = [System.Uri]$env:DATABASE_URL
if ($uri.UserInfo -match ":") {
    $env:PGPASSWORD = $uri.UserInfo.Split(":", 2)[1]
}

function Invoke-Scalar {
    param([string]$Sql)
    $result = psql $env:DATABASE_URL -v ON_ERROR_STOP=1 -At -c $Sql
    return ($result | Select-Object -First 1).Trim()
}

if ($SkipMigrations) {
    Write-Host "[bootstrap] SkipMigrations=1 (environment/bootstrap checks only)." -ForegroundColor DarkGray
    return
}

Write-Host "[bootstrap] ensure schema_migrations table" -ForegroundColor Cyan
psql $env:DATABASE_URL -v ON_ERROR_STOP=1 -c "CREATE TABLE IF NOT EXISTS schema_migrations (filename text PRIMARY KEY, applied_at timestamptz NOT NULL DEFAULT now());" | Out-Null

function Test-MigrationApplied {
    param([string]$Filename)
    switch ($Filename) {
        "0001_init.sql" { return ((Invoke-Scalar "SELECT to_regclass('public.blocks') IS NOT NULL;") -eq "t") }
        "0002_indexes.sql" { return ((Invoke-Scalar "SELECT to_regclass('public.events_payload_json_gin_idx') IS NOT NULL;") -eq "t") }
        "0003_constraints_and_indexes.sql" { return ((Invoke-Scalar "SELECT to_regclass('public.snapshots_block_height_desc_idx') IS NOT NULL;") -eq "t") }
        "0004_stage0_constraints.sql" { return ((Invoke-Scalar "SELECT EXISTS (SELECT 1 FROM pg_constraint WHERE conname = 'ideas_created_order_key');") -eq "t") }
        "0005_stage0_snapshot_required.sql" { return ((Invoke-Scalar "SELECT is_nullable = 'NO' FROM information_schema.columns WHERE table_name = 'snapshots' AND column_name = 'state_root_hash';") -eq "t") }
        "0006_auth_private.sql" { return ((Invoke-Scalar "SELECT to_regclass('public.accounts') IS NOT NULL;") -eq "t") }
        "0007_auth_session_expiry.sql" { return ((Invoke-Scalar "SELECT EXISTS (SELECT 1 FROM information_schema.columns WHERE table_name = 'auth_sessions' AND column_name = 'expires_at');") -eq "t") }
        "0008_snapshot_artifacts.sql" { return ((Invoke-Scalar "SELECT EXISTS (SELECT 1 FROM information_schema.columns WHERE table_name = 'snapshots' AND column_name = 'artifact_path');") -eq "t") }
        "0009_identities_s0.sql" { return ((Invoke-Scalar "SELECT to_regclass('public.identities_s0') IS NOT NULL;") -eq "t") }
        "0010_identity_idea_uniqueness.sql" { return ((Invoke-Scalar "SELECT to_regclass('public.ideas_identity_idea_unique_idx') IS NOT NULL;") -eq "t") }
        "0011_personal_space_organizer_flag.sql" { return ((Invoke-Scalar "SELECT EXISTS (SELECT 1 FROM information_schema.columns WHERE table_name = 'ideas' AND column_name = 'is_personal_space_organizer');") -eq "t") }
        "0012_rails_vines_stage0.sql" { return ((Invoke-Scalar "SELECT to_regclass('public.orderings') IS NOT NULL OR to_regclass('public.rails') IS NOT NULL;") -eq "t") }
        "0013_private_vines_stage1.sql" { return ((Invoke-Scalar "SELECT to_regclass('public.private_orderings') IS NOT NULL OR to_regclass('public.private_vines') IS NOT NULL;") -eq "t") }
        "0014_canonical_append_only.sql" { return ((Invoke-Scalar "SELECT EXISTS (SELECT 1 FROM pg_proc WHERE proname = 'seed_enforce_canonical_append_only');") -eq "t") }
        "0015_cycle_tempo_foundation.sql" { return ((Invoke-Scalar "SELECT to_regclass('public.tempo_predicates') IS NOT NULL;") -eq "t") }
        "0016_canonical_writer_verifications.sql" { return ((Invoke-Scalar "SELECT to_regclass('public.canonical_writer_verifications') IS NOT NULL;") -eq "t") }
        "0017_challenges_importance_stage1.sql" { return ((Invoke-Scalar "SELECT to_regclass('public.challenges') IS NOT NULL;") -eq "t") }
        "0018_stage1_importance_voting.sql" { return ((Invoke-Scalar "SELECT to_regclass('public.challenge_vote_sessions') IS NOT NULL;") -eq "t") }
        "0019_verifier_writer_grants.sql" { return ((Invoke-Scalar "SELECT to_regclass('public.canonical_writer_verification_states') IS NOT NULL;") -eq "t") }
        "0020_auth_session_token_hash.sql" { return ((Invoke-Scalar "SELECT EXISTS (SELECT 1 FROM information_schema.columns WHERE table_name = 'auth_sessions' AND column_name = 'token_hash');") -eq "t") }
        "0024_native_ordering_cutover.sql" { return ((Invoke-Scalar "SELECT to_regclass('public.orderings') IS NOT NULL AND to_regclass('public.rails') IS NULL;") -eq "t") }
        default { return $false }
    }
}

Write-Host "[bootstrap] apply pending migrations (if any)" -ForegroundColor Cyan
Get-ChildItem -Path $migrationDir -Filter "*.sql" | Sort-Object Name | ForEach-Object {
    $name = $_.Name
    $alreadyRecorded = Invoke-Scalar "SELECT EXISTS (SELECT 1 FROM schema_migrations WHERE filename = '$name');"
    if ($alreadyRecorded -eq "t") {
        return
    }

    if (Test-MigrationApplied $name) {
        psql $env:DATABASE_URL -v ON_ERROR_STOP=1 -c "INSERT INTO schema_migrations (filename) VALUES ('$name') ON CONFLICT DO NOTHING;" | Out-Null
        Write-Host "[bootstrap] mark applied $name" -ForegroundColor DarkGray
        return
    }

    Write-Host "[bootstrap] apply $name" -ForegroundColor Cyan
    psql $env:DATABASE_URL -v ON_ERROR_STOP=1 -f $_.FullName
    psql $env:DATABASE_URL -v ON_ERROR_STOP=1 -c "INSERT INTO schema_migrations (filename) VALUES ('$name') ON CONFLICT DO NOTHING;" | Out-Null
}
