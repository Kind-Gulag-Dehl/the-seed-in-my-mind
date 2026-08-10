$ErrorActionPreference="Stop"
$repoRoot=(Resolve-Path(Join-Path $PSScriptRoot "..\..")).Path
$backendDir=Join-Path $repoRoot "backend"
$allowedPrefix="seed_opencore_m1_reviewer_repair_001_"
if(-not[string]::IsNullOrWhiteSpace($env:DATABASE_URL)){
 try{$dbName=([Uri]$env:DATABASE_URL).AbsolutePath.Trim("/")}catch{throw "[verify-backend] DATABASE_URL is invalid"}
 if(-not $dbName.StartsWith($allowedPrefix)){throw "[verify-backend] refuses ordinary or protected DATABASE_URL target"}
}
$env:Path="$env:USERPROFILE\.cargo\bin;$env:Path"
if(-not(Get-Command cargo -ErrorAction SilentlyContinue)){throw "[verify-backend] cargo not found"}
$ownedTarget=[string]::IsNullOrWhiteSpace($env:CARGO_TARGET_DIR)
if($ownedTarget){$env:CARGO_TARGET_DIR=Join-Path $env:TEMP ("seed-open-core-backend-verify-"+[guid]::NewGuid().ToString("N"))}
$env:CARGO_INCREMENTAL="0"
try{
 Push-Location $backendDir
 try{
  cargo build --workspace --locked
  if($LASTEXITCODE -ne 0){throw "[verify-backend] cargo build failed"}
  cargo test --workspace --all-targets --locked
  if($LASTEXITCODE -ne 0){throw "[verify-backend] cargo test failed"}
 }finally{Pop-Location}
 Write-Host "PASS verify-backend" -ForegroundColor Green
}finally{
 if($ownedTarget){Remove-Item -LiteralPath $env:CARGO_TARGET_DIR -Recurse -Force -ErrorAction SilentlyContinue}
}