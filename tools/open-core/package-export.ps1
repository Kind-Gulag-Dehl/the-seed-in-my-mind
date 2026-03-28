param(
    [Parameter(Mandatory = $true)]
    [string]$ExportRoot,
    [Parameter(Mandatory = $true)]
    [string]$OutputZip,
    [Parameter(Mandatory = $true)]
    [string]$ExportTimestamp,
    [Parameter(Mandatory = $true)]
    [string]$GitCommit,
    [Parameter(Mandatory = $true)]
    [string]$ManifestVersion,
    [Parameter(Mandatory = $true)]
    [string]$ManifestSha256
)

$ErrorActionPreference = "Stop"

Add-Type -AssemblyName System.IO.Compression
Add-Type -AssemblyName System.IO.Compression.FileSystem

$resolvedExportRoot = (Resolve-Path $ExportRoot).Path
$outputPath = [System.IO.Path]::GetFullPath($OutputZip)
$outputDir = Split-Path -Parent $outputPath
$exportInfoPath = Join-Path $resolvedExportRoot "EXPORT_INFO.txt"
if (-not (Test-Path $outputDir)) {
    New-Item -ItemType Directory -Path $outputDir -Force | Out-Null
}

if (-not (Test-Path $exportInfoPath)) {
    throw "[open-core-package] missing EXPORT_INFO.txt in export root: $exportInfoPath"
}

if (Test-Path $outputPath) {
    Remove-Item -Path $outputPath -Force
}

$fixedTimestamp = [DateTimeOffset]::Parse("2000-01-01T00:00:00+00:00")
$zip = [System.IO.Compression.ZipFile]::Open($outputPath, [System.IO.Compression.ZipArchiveMode]::Create)

try {
    $files = Get-ChildItem -Path $resolvedExportRoot -Recurse -File | Sort-Object FullName
    foreach ($file in $files) {
        $relativePath = $file.FullName.Substring($resolvedExportRoot.Length).TrimStart('\', '/').Replace('\', '/')
        $entry = $zip.CreateEntry($relativePath, [System.IO.Compression.CompressionLevel]::Optimal)
        $entry.LastWriteTime = $fixedTimestamp
        $entryStream = $entry.Open()
        $fileStream = [System.IO.File]::OpenRead($file.FullName)
        try {
            $fileStream.CopyTo($entryStream)
        } finally {
            $fileStream.Dispose()
            $entryStream.Dispose()
        }
    }

} finally {
    $zip.Dispose()
}

Write-Host "[open-core-package] zip created: $outputPath"
