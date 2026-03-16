$ErrorActionPreference = 'Stop'
Set-StrictMode -Version Latest

$repoRoot = Split-Path -Parent $PSScriptRoot
$frontendDir = Join-Path $repoRoot 'frontend'
$portableRoot = Join-Path $repoRoot 'dist\windows-portable\open-protocol-device-simulator'
$webRoot = Join-Path $portableRoot 'web'
$zipPath = Join-Path $repoRoot 'dist\windows-portable\open-protocol-device-simulator-windows-portable.zip'
$configSource = Join-Path $repoRoot 'config.toml'
$releaseExe = Join-Path $repoRoot 'target\release\open-protocol-device-simulator.exe'

function Resolve-CargoPath {
    $candidates = @(
        (Get-Command cargo -ErrorAction SilentlyContinue | Select-Object -ExpandProperty Source -ErrorAction SilentlyContinue),
        (Join-Path $HOME '.cargo\bin\cargo.exe'),
        'C:\Users\byron\.cargo\bin\cargo.exe'
    ) | Where-Object { $_ } | Select-Object -Unique

    foreach ($candidate in $candidates) {
        if (Test-Path $candidate) {
            return $candidate
        }
    }

    return $null
}

Write-Host 'Building frontend...'
Push-Location $frontendDir
try {
    npm run build
}
finally {
    Pop-Location
}

if ($cargoPath = Resolve-CargoPath) {
    Write-Host 'Building backend release executable...'
    Push-Location $repoRoot
    try {
        & $cargoPath build --release
    }
    finally {
        Pop-Location
    }
}
elseif (Test-Path $releaseExe) {
    Write-Host 'cargo was not found on PATH; using existing release executable.'
}
else {
    throw "cargo was not found on PATH and no prebuilt executable exists at '$releaseExe'. Build the backend once from a Rust-enabled shell, then rerun this script."
}

Write-Host 'Assembling portable package...'
if (Test-Path $portableRoot) {
    Remove-Item -Recurse -Force $portableRoot
}
New-Item -ItemType Directory -Path $webRoot -Force | Out-Null

Copy-Item $releaseExe $portableRoot
if (-not (Test-Path $configSource)) {
    $configSource = Join-Path $repoRoot 'config.example.toml'
}
Copy-Item $configSource (Join-Path $portableRoot 'config.toml')

$frontendBuildDir = Join-Path $frontendDir 'build'
Copy-Item (Join-Path $frontendBuildDir '*') $webRoot -Recurse -Force

$readme = @"
Open Protocol Device Simulator (Windows Portable)

Contents:
- open-protocol-device-simulator.exe
- config.toml
- web\

Run:
1. Extract this folder anywhere writable.
2. Double-click open-protocol-device-simulator.exe or run it from PowerShell.
3. Open http://localhost:8081 in a browser.

Default ports:
- TCP Open Protocol: 8080
- HTTP/UI: 8081

Notes:
- simulator.db will be created next to the executable on first run.
- Edit config.toml if you need different ports or bind address.
"@
Set-Content -Path (Join-Path $portableRoot 'README-PORTABLE.txt') -Value $readme

if (Test-Path $zipPath) {
    Remove-Item -Force $zipPath
}
Compress-Archive -Path (Join-Path $portableRoot '*') -DestinationPath $zipPath

Write-Host "Portable package created at $portableRoot"
Write-Host "Portable zip created at $zipPath"
