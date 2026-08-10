param(
    [string]$SeedSqlPath,
    [string]$SeedDataFile,
    [switch]$SkipSnapshot
)

if (-not [string]::IsNullOrWhiteSpace($SeedSqlPath) -and -not [string]::IsNullOrWhiteSpace($SeedDataFile)) {
    Write-Host "[reset] choose only one of -SeedSqlPath or -SeedDataFile" -ForegroundColor Red
    exit 1
}

. "$PSScriptRoot\\dev-bootstrap.ps1" -SkipMigrations

$backendDir = Join-Path $PSScriptRoot ".."
Set-Location $backendDir

$ErrorActionPreference = "Stop"

Write-Host "[reset] locating psql" -ForegroundColor Cyan
where.exe psql | ForEach-Object { Write-Host "  $_" }

$uri = [System.Uri]$env:DATABASE_URL
if ($uri.UserInfo -match ":") {
    $env:PGPASSWORD = $uri.UserInfo.Split(":", 2)[1]
}

Write-Host "[reset] drop and recreate public schema" -ForegroundColor Cyan
psql $env:DATABASE_URL -v ON_ERROR_STOP=1 -c "DROP SCHEMA public CASCADE; CREATE SCHEMA public;"

$migrationDir = Join-Path $backendDir "migrations\\postgres"
Write-Host "[reset] apply migrations from $migrationDir" -ForegroundColor Cyan
psql $env:DATABASE_URL -v ON_ERROR_STOP=1 -c "CREATE TABLE schema_migrations (filename text PRIMARY KEY, applied_at timestamptz NOT NULL DEFAULT now());"
Get-ChildItem -Path $migrationDir -Filter "*.sql" | Sort-Object Name | ForEach-Object {
    Write-Host "[reset] apply $($_.Name)" -ForegroundColor Cyan
    psql $env:DATABASE_URL -v ON_ERROR_STOP=1 -f $_.FullName
    if ($LASTEXITCODE -ne 0) {
        throw "[reset] migration failed: $($_.Name)"
    }
    psql $env:DATABASE_URL -v ON_ERROR_STOP=1 -c "INSERT INTO schema_migrations (filename) VALUES ('$($_.Name)');" | Out-Null
    if ($LASTEXITCODE -ne 0) {
        throw "[reset] migration ledger update failed: $($_.Name)"
    }
}

if (-not [string]::IsNullOrWhiteSpace($SeedDataFile)) {
    $resolvedSeedDataFile = if ([System.IO.Path]::IsPathRooted($SeedDataFile)) {
        $SeedDataFile
    } else {
        [System.IO.Path]::GetFullPath((Join-Path $backendDir "..\\$SeedDataFile"))
    }

    if (-not (Test-Path $resolvedSeedDataFile)) {
        Write-Host "[reset] missing seed data file: $resolvedSeedDataFile" -ForegroundColor Red
        exit 1
    }

    Write-Host "[reset] import seed data via seed-importer: $resolvedSeedDataFile" -ForegroundColor Cyan
    cargo run -p seed-importer -- --file $resolvedSeedDataFile --force
    if ($LASTEXITCODE -ne 0) {
        Write-Host "[reset] seed-importer failed" -ForegroundColor Red
        exit 1
    }
} else {
    $seedData = if (-not [string]::IsNullOrWhiteSpace($SeedSqlPath)) {
        if ([System.IO.Path]::IsPathRooted($SeedSqlPath)) {
            $SeedSqlPath
        } else {
            [System.IO.Path]::GetFullPath((Join-Path $backendDir "..\\$SeedSqlPath"))
        }
    } else {
        Join-Path $backendDir "fixtures\\seed_dev_data.sql"
    }

    if (-not (Test-Path $seedData)) {
        Write-Host "[reset] missing seed data: $seedData" -ForegroundColor Red
        exit 1
    }

    Write-Host "[reset] apply seed data" -ForegroundColor Cyan
    psql $env:DATABASE_URL -v ON_ERROR_STOP=1 -f $seedData

    if ([string]::IsNullOrWhiteSpace($SeedSqlPath)) {
        $seedOverrides = Join-Path $backendDir "fixtures\\seed_dev.sql"
        if (Test-Path $seedOverrides) {
            Write-Host "[reset] apply seed overrides" -ForegroundColor Cyan
            psql $env:DATABASE_URL -v ON_ERROR_STOP=1 -f $seedOverrides
        }
    }
}

Write-Host "[reset] validate payload_hash for canonical ideas" -ForegroundColor Cyan
cargo run -p payload-hash-seeder -- --validate

if (-not $SkipSnapshot) {
    Write-Host "[reset] run snapshot-builder" -ForegroundColor Cyan
    cargo run -p snapshot-builder
}
