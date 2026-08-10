[CmdletBinding()]
param([string]$AdminDatabaseUrl = $env:SEED_TEST_DATABASE_ADMIN_URL)

$ErrorActionPreference = "Stop"
$DatabasePrefix = "seed_opencore_m1_reviewer_repair_001_"
$DatabaseName = $DatabasePrefix + ([guid]::NewGuid().ToString("N").Substring(0, 16))
$PreservedAdmissionPattern = "seed_admission_p3_test_32944_%"
$ProtectedDatabaseNames = @("seed_dev", "seed_open_core", "postgres", "template0", "template1")
$DatabaseUrl = $null
$DatabaseCreated = $false
$CaseCount = 0

function Invoke-AdminScalar {
    param([string]$Sql)
    $output = & psql $AdminDatabaseUrl -X -q -t -A -v ON_ERROR_STOP=1 -c $Sql 2>&1
    if ($LASTEXITCODE -ne 0) { throw "[seed-bindings-db] administrator query failed" }
    return (($output | Out-String).Trim())
}

function Invoke-Case {
    param([string]$Name, [bool]$ExpectSuccess, [string]$Sql)
    $priorErrorActionPreference = $ErrorActionPreference
    $ErrorActionPreference = "Continue"
    try {
        $null = & psql $DatabaseUrl -X -q -v ON_ERROR_STOP=1 -c $Sql 2>&1
        $exitCode = $LASTEXITCODE
    } finally {
        $ErrorActionPreference = $priorErrorActionPreference
    }
    $succeeded = $exitCode -eq 0
    if ($succeeded -ne $ExpectSuccess) {
        $expectation = if ($ExpectSuccess) { "success" } else { "rejection" }
        throw "[seed-bindings-db] case $Name expected $expectation"
    }
    $script:CaseCount += 1
    $result = if ($succeeded) { "accepted" } else { "rejected" }
    Write-Host "MATRIX_CASE name=$Name result=$result" -ForegroundColor Green
}

if ([string]::IsNullOrWhiteSpace($AdminDatabaseUrl)) {
    throw "[seed-bindings-db] SEED_TEST_DATABASE_ADMIN_URL is required"
}
if (-not (Get-Command psql -ErrorAction SilentlyContinue)) {
    throw "[seed-bindings-db] psql not found on PATH"
}
try { $adminUri = [System.Uri]$AdminDatabaseUrl } catch {
    throw "[seed-bindings-db] administrator URL is not a valid PostgreSQL URI"
}
if ($adminUri.AbsolutePath.Trim("/") -ne "postgres") {
    throw "[seed-bindings-db] administrator URL must target the postgres maintenance database"
}
if ($DatabaseName.Length -gt 63 -or -not $DatabaseName.StartsWith($DatabasePrefix)) {
    throw "[seed-bindings-db] generated database name violates the exact task prefix"
}
if ($ProtectedDatabaseNames -contains $DatabaseName) {
    throw "[seed-bindings-db] generated database name is protected"
}

$preservedBefore = Invoke-AdminScalar "SELECT count(*) FROM pg_database WHERE datname LIKE '$PreservedAdmissionPattern';"
$DatabaseUrl = $adminUri.GetLeftPart([System.UriPartial]::Authority) + "/" + $DatabaseName + $adminUri.Query

try {
    $null = & psql $AdminDatabaseUrl -X -q -v ON_ERROR_STOP=1 -c "CREATE DATABASE $DatabaseName;" 2>&1
    if ($LASTEXITCODE -ne 0) { throw "[seed-bindings-db] failed to create disposable database" }
    $DatabaseCreated = $true
    Write-Host "ISOLATED_DB name=$DatabaseName prefix_ok=true differs_from_seed_dev=true" -ForegroundColor Cyan

    $priorDatabaseUrl = $env:DATABASE_URL
    $env:DATABASE_URL = $DatabaseUrl
    try {
        $backendDir = (Resolve-Path (Join-Path $PSScriptRoot "..\backend")).Path
        & (Join-Path $backendDir "scripts\dev-bootstrap.ps1")
        if ($LASTEXITCODE -ne 0) { throw "[seed-bindings-db] first migration application failed" }
        & (Join-Path $backendDir "scripts\dev-bootstrap.ps1")
        if ($LASTEXITCODE -ne 0) { throw "[seed-bindings-db] second migration application failed" }
    } finally {
        if ([string]::IsNullOrWhiteSpace($priorDatabaseUrl)) {
            Remove-Item Env:DATABASE_URL -ErrorAction SilentlyContinue
        } else {
            $env:DATABASE_URL = $priorDatabaseUrl
        }
    }

    $fixtureSql = @"
INSERT INTO blocks (block_height, block_hash) VALUES (1, repeat('a', 64));
INSERT INTO events (block_height,event_index,event_id,event_type,speaker_identity_id,payload_json) VALUES
(1,0,'00000000-0000-7000-8000-00000000e001','identity_create','00000000-0000-7000-8000-00000000a001','{}'),
(1,1,'00000000-0000-7000-8000-00000000e002','idea_create','00000000-0000-7000-8000-00000000a001','{}'),
(1,2,'00000000-0000-7000-8000-00000000e003','idea_create','00000000-0000-7000-8000-00000000a001','{}'),
(1,3,'00000000-0000-7000-8000-00000000e004','idea_create','00000000-0000-7000-8000-00000000a001','{}'),
(1,4,'00000000-0000-7000-8000-00000000e005','idea_create','00000000-0000-7000-8000-00000000a001','{}'),
(1,5,'00000000-0000-7000-8000-00000000e006','idea_create','00000000-0000-7000-8000-00000000a001','{}'),
(1,6,'00000000-0000-7000-8000-00000000e007','idea_create','00000000-0000-7000-8000-00000000a001','{}'),
(1,7,'00000000-0000-7000-8000-00000000e008','idea_create','00000000-0000-7000-8000-00000000a001','{}'),
(1,40,'00000000-0000-7000-8000-00000000e040','identity_create','00000000-0000-7000-8000-00000000a002','{}'),
(1,41,'00000000-0000-7000-8000-00000000e041','idea_create','00000000-0000-7000-8000-00000000a001','{}');
INSERT INTO identities_s0 (identity_id,title,created_event_id) VALUES
('00000000-0000-7000-8000-00000000a001','matrix author','00000000-0000-7000-8000-00000000e001'),
('00000000-0000-7000-8000-00000000a002','later author','00000000-0000-7000-8000-00000000e040');
INSERT INTO ideas (idea_id,idea_type,speaker_identity_id,created_block_height,created_event_index,created_event_id) VALUES
('00000000-0000-7000-8000-00000000b001','truth_claim','00000000-0000-7000-8000-00000000a001',1,1,'00000000-0000-7000-8000-00000000e002'),
('00000000-0000-7000-8000-00000000b002','actionable_idea','00000000-0000-7000-8000-00000000a001',1,2,'00000000-0000-7000-8000-00000000e003'),
('00000000-0000-7000-8000-00000000b003','conceptual_idea','00000000-0000-7000-8000-00000000a001',1,3,'00000000-0000-7000-8000-00000000e004'),
('00000000-0000-7000-8000-00000000b004','conceptual_idea','00000000-0000-7000-8000-00000000a001',1,4,'00000000-0000-7000-8000-00000000e005'),
('00000000-0000-7000-8000-00000000b005','conceptual_idea','00000000-0000-7000-8000-00000000a001',1,5,'00000000-0000-7000-8000-00000000e006'),
('00000000-0000-7000-8000-00000000b006','conceptual_idea','00000000-0000-7000-8000-00000000a001',1,6,'00000000-0000-7000-8000-00000000e007'),
('00000000-0000-7000-8000-00000000b007','actionable_idea','00000000-0000-7000-8000-00000000a001',1,7,'00000000-0000-7000-8000-00000000e008'),
('00000000-0000-7000-8000-00000000b041','conceptual_idea','00000000-0000-7000-8000-00000000a001',1,41,'00000000-0000-7000-8000-00000000e041');
"@
    $null = & psql $DatabaseUrl -X -q -v ON_ERROR_STOP=1 -c $fixtureSql 2>&1
    if ($LASTEXITCODE -ne 0) { throw "[seed-bindings-db] baseline fixture failed" }

    $repEvent = "INSERT INTO events(block_height,event_index,event_id,event_type,speaker_identity_id,payload_json) VALUES(1,10,'00000000-0000-7000-8000-00000000e010','representation_create','00000000-0000-7000-8000-00000000a001','{}');"
    $lateAuthorEvent = "INSERT INTO events(block_height,event_index,event_id,event_type,speaker_identity_id,payload_json) VALUES(1,10,'00000000-0000-7000-8000-00000000e010','representation_create','00000000-0000-7000-8000-00000000a002','{}');"
    $repCols = "representation_id,target_kind,target_id,tier_enum,tier_complexity,vocabulary_version_id,payload_hash,payload_text,author_identity_id,created_block_height,created_event_index,created_event_id"
    Invoke-Case "valid_title_slot" $true "BEGIN;$repEvent INSERT INTO representations($repCols) VALUES('00000000-0000-7000-8000-00000000c001',0,'00000000-0000-7000-8000-00000000b003',0,NULL,NULL,repeat('a',64),NULL,'00000000-0000-7000-8000-00000000a001',1,10,'00000000-0000-7000-8000-00000000e010');SET CONSTRAINTS ALL IMMEDIATE;ROLLBACK;"
    Invoke-Case "invalid_title_description_fields" $false "BEGIN;$repEvent INSERT INTO representations($repCols) VALUES('00000000-0000-7000-8000-00000000c002',0,'00000000-0000-7000-8000-00000000b003',0,1,NULL,repeat('a',64),NULL,'00000000-0000-7000-8000-00000000a001',1,10,'00000000-0000-7000-8000-00000000e010');SET CONSTRAINTS ALL IMMEDIATE;"
    Invoke-Case "valid_standard_description_slot" $true "BEGIN;$repEvent INSERT INTO representations($repCols) VALUES('00000000-0000-7000-8000-00000000c003',0,'00000000-0000-7000-8000-00000000b003',2,1,NULL,repeat('a',64),NULL,'00000000-0000-7000-8000-00000000a001',1,10,'00000000-0000-7000-8000-00000000e010');SET CONSTRAINTS ALL IMMEDIATE;ROLLBACK;"
    Invoke-Case "valid_canonical_description_slot" $true "BEGIN;$repEvent INSERT INTO representations($repCols) VALUES('00000000-0000-7000-8000-00000000c004',0,'00000000-0000-7000-8000-00000000b003',3,3,'00000000-0000-7000-8000-00000000b004',repeat('a',64),NULL,'00000000-0000-7000-8000-00000000a001',1,10,'00000000-0000-7000-8000-00000000e010');SET CONSTRAINTS ALL IMMEDIATE;ROLLBACK;"
    Invoke-Case "invalid_description_missing_complexity" $false "BEGIN;$repEvent INSERT INTO representations($repCols) VALUES('00000000-0000-7000-8000-00000000c005',0,'00000000-0000-7000-8000-00000000b003',1,NULL,NULL,repeat('a',64),NULL,'00000000-0000-7000-8000-00000000a001',1,10,'00000000-0000-7000-8000-00000000e010');SET CONSTRAINTS ALL IMMEDIATE;"
    Invoke-Case "invalid_canonical_description_missing_vocabulary" $false "BEGIN;$repEvent INSERT INTO representations($repCols) VALUES('00000000-0000-7000-8000-00000000c006',0,'00000000-0000-7000-8000-00000000b003',1,3,NULL,repeat('a',64),NULL,'00000000-0000-7000-8000-00000000a001',1,10,'00000000-0000-7000-8000-00000000e010');SET CONSTRAINTS ALL IMMEDIATE;"
    Invoke-Case "invalid_noncanonical_description_with_vocabulary" $false "BEGIN;$repEvent INSERT INTO representations($repCols) VALUES('00000000-0000-7000-8000-00000000c007',0,'00000000-0000-7000-8000-00000000b003',1,1,'00000000-0000-7000-8000-00000000b004',repeat('a',64),NULL,'00000000-0000-7000-8000-00000000a001',1,10,'00000000-0000-7000-8000-00000000e010');SET CONSTRAINTS ALL IMMEDIATE;"
    Invoke-Case "invalid_author_pre_use" $false "BEGIN;$lateAuthorEvent INSERT INTO representations($repCols) VALUES('00000000-0000-7000-8000-00000000c008',0,'00000000-0000-7000-8000-00000000b003',0,NULL,NULL,repeat('a',64),NULL,'00000000-0000-7000-8000-00000000a002',1,10,'00000000-0000-7000-8000-00000000e010');SET CONSTRAINTS ALL IMMEDIATE;"
    Invoke-Case "invalid_vocabulary_pre_use" $false "BEGIN;$repEvent INSERT INTO representations($repCols) VALUES('00000000-0000-7000-8000-00000000c009',0,'00000000-0000-7000-8000-00000000b003',1,3,'00000000-0000-7000-8000-00000000b041',repeat('a',64),NULL,'00000000-0000-7000-8000-00000000a001',1,10,'00000000-0000-7000-8000-00000000e010');SET CONSTRAINTS ALL IMMEDIATE;"

    $ordEvent = "INSERT INTO events(block_height,event_index,event_id,event_type,speaker_identity_id,payload_json) VALUES(1,10,'00000000-0000-7000-8000-00000000e010','ordering_create','00000000-0000-7000-8000-00000000a001','{}');"
    $forkEvent = "INSERT INTO events(block_height,event_index,event_id,event_type,speaker_identity_id,payload_json) VALUES(1,11,'00000000-0000-7000-8000-00000000e011','ordering_fork','00000000-0000-7000-8000-00000000a001','{}');"
    $ordCols = "ordering_id,ordering_profile,vine_type,subject_idea_id,speaker_identity_id,created_block_height,created_event_index,created_event_id,base_ordering_id"
    Invoke-Case "valid_typed_evidence_roles" $true "BEGIN;$ordEvent INSERT INTO orderings($ordCols) VALUES('00000000-0000-7000-8000-00000000d001',1,NULL,'00000000-0000-7000-8000-00000000b001','00000000-0000-7000-8000-00000000a001',1,10,'00000000-0000-7000-8000-00000000e010',NULL);INSERT INTO ordering_items(ordering_id,idx,idea_id,item_role) VALUES('00000000-0000-7000-8000-00000000d001',0,'00000000-0000-7000-8000-00000000b005',0),('00000000-0000-7000-8000-00000000d001',1,'00000000-0000-7000-8000-00000000b006',1);SET CONSTRAINTS ALL IMMEDIATE;ROLLBACK;"
    Invoke-Case "invalid_evidence_subject_type" $false "BEGIN;$ordEvent INSERT INTO orderings($ordCols) VALUES('00000000-0000-7000-8000-00000000d002',1,NULL,'00000000-0000-7000-8000-00000000b003','00000000-0000-7000-8000-00000000a001',1,10,'00000000-0000-7000-8000-00000000e010',NULL);INSERT INTO ordering_items(ordering_id,idx,idea_id,item_role) VALUES('00000000-0000-7000-8000-00000000d002',0,'00000000-0000-7000-8000-00000000b005',0);SET CONSTRAINTS ALL IMMEDIATE;"
    Invoke-Case "invalid_action_subject_type" $false "BEGIN;$ordEvent INSERT INTO orderings($ordCols) VALUES('00000000-0000-7000-8000-00000000d003',2,NULL,'00000000-0000-7000-8000-00000000b001','00000000-0000-7000-8000-00000000a001',1,10,'00000000-0000-7000-8000-00000000e010',NULL);INSERT INTO ordering_items(ordering_id,idx,idea_id,item_role) VALUES('00000000-0000-7000-8000-00000000d003',0,'00000000-0000-7000-8000-00000000b005',2);SET CONSTRAINTS ALL IMMEDIATE;"
    Invoke-Case "invalid_item_role" $false "BEGIN;$ordEvent INSERT INTO orderings($ordCols) VALUES('00000000-0000-7000-8000-00000000d004',1,NULL,'00000000-0000-7000-8000-00000000b001','00000000-0000-7000-8000-00000000a001',1,10,'00000000-0000-7000-8000-00000000e010',NULL);INSERT INTO ordering_items(ordering_id,idx,idea_id,item_role) VALUES('00000000-0000-7000-8000-00000000d004',0,'00000000-0000-7000-8000-00000000b005',2);SET CONSTRAINTS ALL IMMEDIATE;"
    Invoke-Case "invalid_duplicate_standardized_item" $false "BEGIN;$ordEvent INSERT INTO orderings($ordCols) VALUES('00000000-0000-7000-8000-00000000d005',1,NULL,'00000000-0000-7000-8000-00000000b001','00000000-0000-7000-8000-00000000a001',1,10,'00000000-0000-7000-8000-00000000e010',NULL);INSERT INTO ordering_items(ordering_id,idx,idea_id,item_role) VALUES('00000000-0000-7000-8000-00000000d005',0,'00000000-0000-7000-8000-00000000b005',0),('00000000-0000-7000-8000-00000000d005',1,'00000000-0000-7000-8000-00000000b005',1);SET CONSTRAINTS ALL IMMEDIATE;"
    Invoke-Case "invalid_action_lane_mixed" $false "BEGIN;$ordEvent INSERT INTO orderings($ordCols) VALUES('00000000-0000-7000-8000-00000000d006',2,NULL,'00000000-0000-7000-8000-00000000b002','00000000-0000-7000-8000-00000000a001',1,10,'00000000-0000-7000-8000-00000000e010',NULL);INSERT INTO ordering_items(ordering_id,idx,idea_id,item_role) VALUES('00000000-0000-7000-8000-00000000d006',0,'00000000-0000-7000-8000-00000000b005',2),('00000000-0000-7000-8000-00000000d006',1,'00000000-0000-7000-8000-00000000b006',3);SET CONSTRAINTS ALL IMMEDIATE;"

    $baseAction = "$ordEvent INSERT INTO orderings($ordCols) VALUES('00000000-0000-7000-8000-00000000d010',2,NULL,'00000000-0000-7000-8000-00000000b002','00000000-0000-7000-8000-00000000a001',1,10,'00000000-0000-7000-8000-00000000e010',NULL);INSERT INTO ordering_items(ordering_id,idx,idea_id,item_role) VALUES('00000000-0000-7000-8000-00000000d010',0,'00000000-0000-7000-8000-00000000b005',2);$forkEvent"
    Invoke-Case "valid_action_fork_preservation" $true "BEGIN;$baseAction INSERT INTO orderings($ordCols) VALUES('00000000-0000-7000-8000-00000000d011',2,NULL,'00000000-0000-7000-8000-00000000b002','00000000-0000-7000-8000-00000000a001',1,11,'00000000-0000-7000-8000-00000000e011','00000000-0000-7000-8000-00000000d010');INSERT INTO ordering_items(ordering_id,idx,idea_id,item_role) VALUES('00000000-0000-7000-8000-00000000d011',0,'00000000-0000-7000-8000-00000000b006',2);SET CONSTRAINTS ALL IMMEDIATE;ROLLBACK;"
    Invoke-Case "invalid_fork_subject_change" $false "BEGIN;$baseAction INSERT INTO orderings($ordCols) VALUES('00000000-0000-7000-8000-00000000d012',2,NULL,'00000000-0000-7000-8000-00000000b007','00000000-0000-7000-8000-00000000a001',1,11,'00000000-0000-7000-8000-00000000e011','00000000-0000-7000-8000-00000000d010');INSERT INTO ordering_items(ordering_id,idx,idea_id,item_role) VALUES('00000000-0000-7000-8000-00000000d012',0,'00000000-0000-7000-8000-00000000b006',2);SET CONSTRAINTS ALL IMMEDIATE;"
    Invoke-Case "invalid_action_fork_lane_change" $false "BEGIN;$baseAction INSERT INTO orderings($ordCols) VALUES('00000000-0000-7000-8000-00000000d013',2,NULL,'00000000-0000-7000-8000-00000000b002','00000000-0000-7000-8000-00000000a001',1,11,'00000000-0000-7000-8000-00000000e011','00000000-0000-7000-8000-00000000d010');INSERT INTO ordering_items(ordering_id,idx,idea_id,item_role) VALUES('00000000-0000-7000-8000-00000000d013',0,'00000000-0000-7000-8000-00000000b006',3);SET CONSTRAINTS ALL IMMEDIATE;"

    $baseEvidence = "$ordEvent INSERT INTO orderings($ordCols) VALUES('00000000-0000-7000-8000-00000000d020',1,NULL,'00000000-0000-7000-8000-00000000b001','00000000-0000-7000-8000-00000000a001',1,10,'00000000-0000-7000-8000-00000000e010',NULL);INSERT INTO ordering_items(ordering_id,idx,idea_id,item_role) VALUES('00000000-0000-7000-8000-00000000d020',0,'00000000-0000-7000-8000-00000000b005',0);$forkEvent"
    Invoke-Case "invalid_fork_retained_role_change" $false "BEGIN;$baseEvidence INSERT INTO orderings($ordCols) VALUES('00000000-0000-7000-8000-00000000d021',1,NULL,'00000000-0000-7000-8000-00000000b001','00000000-0000-7000-8000-00000000a001',1,11,'00000000-0000-7000-8000-00000000e011','00000000-0000-7000-8000-00000000d020');INSERT INTO ordering_items(ordering_id,idx,idea_id,item_role) VALUES('00000000-0000-7000-8000-00000000d021',0,'00000000-0000-7000-8000-00000000b005',1);SET CONSTRAINTS ALL IMMEDIATE;"

    if ($CaseCount -ne 19) { throw "[seed-bindings-db] expected 19 matrix cases, observed $CaseCount" }
    Write-Host "PASS seed-conformance-bindings-postgres cases=$CaseCount migrations=0025" -ForegroundColor Green
} finally {
    if ($DatabaseCreated) {
        $null = & psql $AdminDatabaseUrl -X -q -v ON_ERROR_STOP=1 -c "SELECT pg_terminate_backend(pid) FROM pg_stat_activity WHERE datname = '$DatabaseName' AND pid <> pg_backend_pid();" 2>&1
        $null = & psql $AdminDatabaseUrl -X -q -v ON_ERROR_STOP=1 -c "DROP DATABASE IF EXISTS $DatabaseName;" 2>&1
        if ((Invoke-AdminScalar "SELECT count(*) FROM pg_database WHERE datname = '$DatabaseName';") -ne "0") {
            throw "[seed-bindings-db] cleanup verification failed"
        }
        Write-Host "ISOLATED_DB_CLEANUP name=$DatabaseName dropped=true" -ForegroundColor Cyan
    }
    $preservedAfter = Invoke-AdminScalar "SELECT count(*) FROM pg_database WHERE datname LIKE '$PreservedAdmissionPattern';"
    if ($preservedAfter -ne $preservedBefore) { throw "[seed-bindings-db] preserved admission-test database count changed" }
    Write-Host "PRESERVED_DATABASES pattern=$PreservedAdmissionPattern before=$preservedBefore after=$preservedAfter" -ForegroundColor Cyan
}
