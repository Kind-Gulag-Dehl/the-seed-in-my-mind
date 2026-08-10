param(
 [string]$SeedDataFile="seed/reviewer-demo.seed-data-v0.json",
 [string]$AdminDatabaseUrl=$env:SEED_TEST_DATABASE_ADMIN_URL,
 [switch]$BuildReferenceFrontend,
 [switch]$KeepServerRunning
)
$ErrorActionPreference="Stop"
$prefix="seed_opencore_m1_reviewer_repair_001_"
$dbName=$prefix+([guid]::NewGuid().ToString("N").Substring(0,16))
$preservedPattern="seed_admission_p3_test_32944_%"
$dbCreated=$false
$server=$null
function AdminScalar([string]$Sql) {
 $o=& psql $AdminDatabaseUrl -X -q -t -A -v ON_ERROR_STOP=1 -c $Sql 2>&1
 if($LASTEXITCODE -ne 0){throw "[open-core-demo] administrator query failed"}
 return (($o|Out-String).Trim())
}
function JsonGet([string]$Uri) {
 $content=& curl.exe --fail --silent --show-error --max-time 5 $Uri
 if($LASTEXITCODE -ne 0){throw "[open-core-demo] GET failed for $Uri"}
 return (($content|Out-String)|ConvertFrom-Json)
}
if($KeepServerRunning){throw "[open-core-demo] KeepServerRunning is incompatible with disposable cleanup"}
if([string]::IsNullOrWhiteSpace($AdminDatabaseUrl)){throw "[open-core-demo] SEED_TEST_DATABASE_ADMIN_URL is required"}
if(-not[string]::IsNullOrWhiteSpace($env:DATABASE_URL)){
 try{$inheritedName=([Uri]$env:DATABASE_URL).AbsolutePath.Trim('/')}catch{throw '[open-core-demo] DATABASE_URL is invalid'}
 if(-not $inheritedName.StartsWith($prefix)){throw '[open-core-demo] refuses ordinary or protected DATABASE_URL target'}
}
if(-not(Get-Command cargo -ErrorAction SilentlyContinue)){throw "[open-core-demo] cargo not found"}
if(-not(Get-Command psql -ErrorAction SilentlyContinue)){throw "[open-core-demo] psql not found"}
if(-not(Get-Command curl.exe -ErrorAction SilentlyContinue)){throw "[open-core-demo] curl.exe not found"}
try{$adminUri=[System.Uri]$AdminDatabaseUrl}catch{throw "[open-core-demo] invalid administrator URL"}
if($adminUri.AbsolutePath.Trim("/") -ne "postgres"){throw "[open-core-demo] administrator URL must target postgres"}
if($dbName.Length -gt 63 -or -not $dbName.StartsWith($prefix)){throw "[open-core-demo] invalid generated database name"}
$repo=(Resolve-Path(Join-Path $PSScriptRoot "..")).Path
$backend=Join-Path $repo "backend"
$frontend=Join-Path $repo "frontend\open-core-reference"
$seed=if([IO.Path]::IsPathRooted($SeedDataFile)){(Resolve-Path $SeedDataFile).Path}else{(Resolve-Path(Join-Path $repo $SeedDataFile)).Path}
$run=Join-Path $env:TEMP ("seed-open-core-demo-"+[guid]::NewGuid().ToString("N"))
$target=Join-Path $run "cargo-target"
$artifacts=Join-Path $run "snapshot-artifacts"
$frontWork=Join-Path $run "frontend"
$outLog=Join-Path $run "api-out.log"
$errLog=Join-Path $run "api-err.log"
New-Item -ItemType Directory -Path $run -Force|Out-Null
$before=AdminScalar "SELECT count(*) FROM pg_database WHERE datname LIKE '$preservedPattern';"
$dbUrl=$adminUri.GetLeftPart([System.UriPartial]::Authority)+"/"+$dbName+$adminUri.Query
$env:Path="$env:USERPROFILE\.cargo\bin;$env:Path"
$env:DATABASE_URL=$dbUrl
$env:CARGO_TARGET_DIR=$target
$env:CARGO_INCREMENTAL="0"
$env:CARGO_PROFILE_DEV_DEBUG="0"
$env:CARGO_PROFILE_TEST_DEBUG="0"
$env:SEED_SNAPSHOT_ARTIFACT_BASE_DIR=$artifacts
try{
 $null=& psql $AdminDatabaseUrl -X -q -v ON_ERROR_STOP=1 -c "CREATE DATABASE $dbName;" 2>&1
 if($LASTEXITCODE -ne 0){throw "[open-core-demo] failed to create disposable database"}
 $dbCreated=$true
 Write-Host "ISOLATED_DB name=$dbName prefix_ok=true differs_from_seed_dev=true" -ForegroundColor Cyan
 &(Join-Path $backend "scripts\reset-dev-db.ps1") -SeedDataFile $seed
 if($LASTEXITCODE -ne 0){throw "[open-core-demo] reset/import failed"}
 Push-Location $backend
 try{cargo run -p snapshot-verify -- --latest --profile stage0;if($LASTEXITCODE -ne 0){throw "[open-core-demo] snapshot verification failed"}}finally{Pop-Location}
 Push-Location $backend
 try{cargo build -p api-server --no-default-features --features open_core --locked;if($LASTEXITCODE -ne 0){throw "[open-core-demo] api-server build failed"}}finally{Pop-Location}
 $apiExe=Join-Path $target "debug\api-server.exe"
 if(-not(Test-Path $apiExe)){throw "[open-core-demo] isolated api-server executable missing"}
 $processPath=$env:Path
 [Environment]::SetEnvironmentVariable("PATH",$null,"Process")
 [Environment]::SetEnvironmentVariable("Path",$processPath,"Process")
 $server=Start-Process -FilePath $apiExe -WorkingDirectory $backend -RedirectStandardOutput $outLog -RedirectStandardError $errLog -PassThru -WindowStyle Hidden
 $ready=$false
 for($i=0;$i -lt 60;$i++){
  Start-Sleep -Milliseconds 500
  if($server.HasExited){break}
  $status=& curl.exe --silent --output NUL --write-out "%{http_code}" --max-time 2 "http://127.0.0.1:3000/api/v0/health" 2>$null
  if($LASTEXITCODE -eq 0 -and $status -eq "200"){$ready=$true;break}
 }
 if(-not $ready){throw "[open-core-demo] owned api-server did not become ready; port 3000 may be occupied"}
 $capabilities=JsonGet "http://127.0.0.1:3000/api/v0/capabilities"
 if($capabilities.api_contract_version -ne "1.0.0" -or $capabilities.migration_head -ne "0025_seed_conformance_bindings"){throw "[open-core-demo] public API capability contract mismatch"}
 $snapshot=JsonGet "http://127.0.0.1:3000/api/v0/snapshot/latest?include_preview=true"
 $ideas=JsonGet "http://127.0.0.1:3000/api/v0/ideas/top?limit=10&offset=0&order=asc"
 $detail=JsonGet "http://127.0.0.1:3000/api/v0/idea/59427f80-5901-7128-990e-90b49f288bcc"
 if($BuildReferenceFrontend){
  if(-not(Get-Command npm -ErrorAction SilentlyContinue)){throw "[open-core-demo] npm not found"}
  New-Item -ItemType Directory -Path $frontWork -Force|Out-Null
  Get-ChildItem $frontend -Force|Where-Object{$_.Name -notin @("node_modules","dist",".vite","coverage","package-lock.json","tsconfig.tsbuildinfo")}|Copy-Item -Destination $frontWork -Recurse -Force
  Push-Location $frontWork
  try{npm install;if($LASTEXITCODE -ne 0){throw "frontend install failed"};npm test;if($LASTEXITCODE -ne 0){throw "frontend tests failed"};npm run build;if($LASTEXITCODE -ne 0){throw "frontend build failed"}}finally{Pop-Location}
 }
 Write-Host "Open-core demo report" -ForegroundColor Green
 Write-Host "  API contract version   : $($capabilities.api_contract_version)"
 Write-Host "  migration head         : $($capabilities.migration_head)"
 Write-Host "  snapshot height        : $($snapshot.snapshot.height)"
 Write-Host "  snapshot hash          : $($snapshot.snapshot.snapshot_hash)"
 Write-Host "  shared map commitment  : $($snapshot.snapshot.shared_map_commitment)"
 Write-Host "  event count            : $($snapshot.snapshot.event_count)"
 Write-Host "  imported demo idea     : $($detail.idea.title)"
 Write-Host "  top idea titles        : $((@($ideas.ideas|Select-Object -First 3|ForEach-Object{$_.title})-join ' | '))"
 Write-Host "PASS open-core-demo" -ForegroundColor Green
}finally{
 if($null -ne $server -and -not $server.HasExited){Stop-Process -Id $server.Id -Force -ErrorAction SilentlyContinue;Wait-Process -Id $server.Id -Timeout 10 -ErrorAction SilentlyContinue}
 if($dbCreated){
  $null=& psql $AdminDatabaseUrl -X -q -v ON_ERROR_STOP=1 -c "SELECT pg_terminate_backend(pid) FROM pg_stat_activity WHERE datname='$dbName' AND pid<>pg_backend_pid();" 2>&1
  $null=& psql $AdminDatabaseUrl -X -q -v ON_ERROR_STOP=1 -c "DROP DATABASE IF EXISTS $dbName;" 2>&1
  if((AdminScalar "SELECT count(*) FROM pg_database WHERE datname='$dbName';") -ne "0"){throw "[open-core-demo] database cleanup failed"}
  Write-Host "ISOLATED_DB_CLEANUP name=$dbName dropped=true" -ForegroundColor Cyan
 }
 $after=AdminScalar "SELECT count(*) FROM pg_database WHERE datname LIKE '$preservedPattern';"
 if($after -ne $before){throw "[open-core-demo] preserved admission database count changed"}
 Write-Host "PRESERVED_DATABASES pattern=$preservedPattern before=$before after=$after" -ForegroundColor Cyan
 Remove-Item -LiteralPath $run -Recurse -Force -ErrorAction SilentlyContinue
}