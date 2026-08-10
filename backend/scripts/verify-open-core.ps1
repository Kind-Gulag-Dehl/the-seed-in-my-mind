$ErrorActionPreference="Stop"
$repoRoot=(Resolve-Path(Join-Path $PSScriptRoot "..\..")).Path
$allowedPrefix="seed_opencore_m1_reviewer_repair_001_"
if(-not[string]::IsNullOrWhiteSpace($env:DATABASE_URL)){
 try{$dbName=([Uri]$env:DATABASE_URL).AbsolutePath.Trim("/")}catch{throw "[verify-open-core] DATABASE_URL is invalid"}
 if(-not $dbName.StartsWith($allowedPrefix)){throw "[verify-open-core] refuses ordinary or protected DATABASE_URL target"}
}
if([string]::IsNullOrWhiteSpace($env:SEED_TEST_DATABASE_ADMIN_URL)){throw "[verify-open-core] SEED_TEST_DATABASE_ADMIN_URL is required"}
$env:Path="$env:USERPROFILE\.cargo\bin;$env:Path"
if(-not(Get-Command cargo -ErrorAction SilentlyContinue)){throw "[verify-open-core] cargo not found"}
$target=Join-Path $env:TEMP ("seed-open-core-surface-verify-"+[guid]::NewGuid().ToString("N"))
$env:CARGO_TARGET_DIR=$target
$env:CARGO_INCREMENTAL="0"
try{
 Push-Location(Join-Path $repoRoot "backend")
 try{
  cargo build -p api-server --no-default-features --features open_core --locked
  if($LASTEXITCODE -ne 0){throw "[verify-open-core] api-server build failed"}
 }finally{Pop-Location}
 &(Join-Path $repoRoot "scripts\open-core-demo.ps1")
 if($LASTEXITCODE -ne 0){throw "[verify-open-core] isolated demo verification failed"}
 Write-Host "PASS verify-open-core" -ForegroundColor Green
}finally{
 Remove-Item -LiteralPath $target -Recurse -Force -ErrorAction SilentlyContinue
}