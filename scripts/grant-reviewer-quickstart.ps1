param([ValidateSet("15","30")][string]$Profile="15")
$ErrorActionPreference="Stop"
$repoRoot=(Resolve-Path(Join-Path $PSScriptRoot "..")).Path
Set-Location $repoRoot
if([string]::IsNullOrWhiteSpace($env:SEED_TEST_DATABASE_ADMIN_URL)){
 throw "[grant-review] SEED_TEST_DATABASE_ADMIN_URL for the postgres maintenance database is required"
}
function Step([string]$Label,[scriptblock]$Action){
 Write-Host "[grant-review] $Label" -ForegroundColor Cyan
 &$Action
 if($LASTEXITCODE -ne 0){throw "[grant-review] step failed: $Label"}
}
Write-Host "Grant reviewer quickstart ($Profile-minute path)" -ForegroundColor Green
Step "verify open-core boundaries" {npm run verify:boundaries}
Step "verify canonical DTO surface" {npm run verify:canonical-dto}
if($Profile -eq "15"){
 Step "run isolated deterministic open-core demo" {powershell -ExecutionPolicy Bypass -File scripts/open-core-demo.ps1}
}else{
 Step "run isolated demo and reference frontend verification" {powershell -ExecutionPolicy Bypass -File scripts/open-core-demo.ps1 -BuildReferenceFrontend}
 Step "run guarded migration-0025 semantic matrix" {npm run verify:seed-bindings-db}
 Step "run guarded canonical-history two-database round trip" {npm run verify:canonical-history-db}
}
Write-Host "$Profile-minute path complete." -ForegroundColor Green
Write-Host "Read docs/open-core-reviewer-guide.md and docs/open-core-implementation-status.md."