param(
    [string[]]$ArtifactTargets = @("backend/var", "audit_out"),
    [switch]$IncludeScriptChecks = $true
)

$ErrorActionPreference = "Stop"

function Resolve-ExistingTargets {
    param([string[]]$Candidates)
    $resolved = @()
    foreach ($candidate in $Candidates) {
        if (Test-Path $candidate) {
            $resolved += $candidate
        }
    }
    return $resolved
}

function Scan-Patterns {
    param(
        [string]$Scope,
        [hashtable]$Patterns,
        [string[]]$Targets
    )

    $scopeFindings = 0
    foreach ($name in $Patterns.Keys) {
        $pattern = $Patterns[$name]
        $files = @()
        if ($Targets.Count -gt 0) {
            $files = (& rg -l -S --no-messages -- "$pattern" @Targets | Sort-Object -Unique)
        }
        $count = @($files).Count
        Write-Host ("[scan] scope={0} pattern={1} hits={2}" -f $Scope, $name, $count)
        foreach ($file in $files) {
            Write-Host ("  {0}" -f $file)
        }
        $scopeFindings += $count
    }
    return $scopeFindings
}

$artifactPatterns = @{
    "bearer"       = "(?i)authorization\\s*:\\s*bearer\\s+|\\bbearer\\s+[a-z0-9\\-\\._=:+/]{20,}"
    "jwt"          = "(?i)\\beyJ[a-zA-Z0-9_-]{8,}\\.[a-zA-Z0-9_-]{8,}\\.[a-zA-Z0-9_-]{8,}\\b"
    "private_key"  = "(?i)BEGIN (RSA|EC|OPENSSH)? ?PRIVATE KEY"
    "password_set" = "(?i)\\bpassword\\s*[=:]\\s*\\S+"
    "secret_set"   = "(?i)\\bsecret\\s*[=:]\\s*\\S+"
}

$scriptLeakPatterns = @{
    "writehost_bearer"            = '(?i)Write-Host.*Bearer'
    "writehost_authorization"     = '(?i)Write-Host.*Authorization'
    "writehost_raw_token_var"     = '(?i)Write-Host.*\$token\b'
    "writehost_database_url_raw"  = '(?i)Write-Host.*DATABASE_URL=\$env:DATABASE_URL'
    "writehost_pgpassfile_raw"    = '(?i)Write-Host.*PGPASSFILE=\$env:PGPASSFILE'
}

$totalFindings = 0

$existingArtifactTargets = Resolve-ExistingTargets $ArtifactTargets
if ($existingArtifactTargets.Count -eq 0) {
    Write-Host "[scan] no artifact targets found"
} else {
    $totalFindings += Scan-Patterns -Scope "artifacts" -Patterns $artifactPatterns -Targets $existingArtifactTargets
}

if ($IncludeScriptChecks) {
    $scriptTargets = @(
        Get-ChildItem -Path "backend/scripts" -Filter "*.ps1" -File -ErrorAction SilentlyContinue |
            Where-Object { $_.Name -ne "scan-secrets.ps1" } |
            ForEach-Object { $_.FullName }
    )
    if ($scriptTargets.Count -gt 0) {
        $totalFindings += Scan-Patterns -Scope "scripts" -Patterns $scriptLeakPatterns -Targets $scriptTargets
    }
}

Write-Host ("[scan] total_findings={0}" -f $totalFindings)
if ($totalFindings -gt 0) {
    exit 1
}
exit 0
