param(
 [Parameter(Mandatory=$true)][string]$ExportRoot,
 [string]$SeedDataFile="seed/reviewer-demo.seed-data-v0.json"
)
$ErrorActionPreference="Stop"
$resolved=(Resolve-Path $ExportRoot).Path
if([string]::IsNullOrWhiteSpace($env:SEED_TEST_DATABASE_ADMIN_URL)){throw "[smoke-export] SEED_TEST_DATABASE_ADMIN_URL is required"}
if(-not[string]::IsNullOrWhiteSpace($env:DATABASE_URL)){
 try{$name=([Uri]$env:DATABASE_URL).AbsolutePath.Trim("/")}catch{throw "[smoke-export] DATABASE_URL is invalid"}
 if(-not $name.StartsWith("seed_opencore_m1_reviewer_repair_001_")){throw "[smoke-export] refuses ordinary or protected DATABASE_URL target"}
}
$demo=Join-Path $resolved "scripts\open-core-demo.ps1"
if(-not(Test-Path $demo)){throw "[smoke-export] exported demo script missing"}
& $demo -SeedDataFile $SeedDataFile -BuildReferenceFrontend
if($LASTEXITCODE -ne 0){throw "[smoke-export] isolated exported demo failed"}
Write-Host "PASS smoke-export" -ForegroundColor Green