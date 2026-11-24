<#
.SYNOPSIS
    Local pluginval test runner for Windows

.DESCRIPTION
    Downloads, caches, and runs pluginval (VST3 Plugin Validator) against the built VST3 bundle.
    Supports strictness levels 5 and 7.

.PARAMETER Vst3Path
    Path to the VST3 bundle (default: target/release/Periodic Instruments.vst3)

.PARAMETER StrictnessLevel
    Pluginval strictness level: 5 (host compatibility baseline) or 7 (extended testing) (default: 7)

.PARAMETER PluginvalVersion
    Pluginval version to download (default: v0.3.8)

.EXAMPLE
    ./scripts/run-pluginval.ps1

.EXAMPLE
    ./scripts/run-pluginval.ps1 -Vst3Path "target/release/Periodic Instruments.vst3" -StrictnessLevel 5

.EXAMPLE
    ./scripts/run-pluginval.ps1 -StrictnessLevel 7 -PluginvalVersion "v0.3.8"
#>

param(
    [string]$Vst3Path = "target/release/Periodic Instruments.vst3",
    [ValidateSet(5, 7)]
    [int]$StrictnessLevel = 7,
    [string]$PluginvalVersion = "v0.3.8"
)

$ErrorActionPreference = "Stop"

$scriptDir = Split-Path -Parent $MyInvocation.MyCommand.Definition
$projectRoot = Split-Path -Parent $scriptDir
$cacheDir = Join-Path $projectRoot ".cache\pluginval-$PluginvalVersion"
$pluginvalDir = Join-Path $cacheDir "windows"
$pluginvalZip = Join-Path $pluginvalDir "pluginval.zip"

Write-Host "=== pluginval Local Test Runner (Windows) ===" -ForegroundColor Cyan
Write-Host "VST3 Path: $Vst3Path"
Write-Host "Strictness Level: $StrictnessLevel"
Write-Host "Pluginval Version: $PluginvalVersion"
Write-Host "Cache Dir: $cacheDir"
Write-Host ""

# Check if VST3 bundle exists
if (-not (Test-Path $Vst3Path)) {
    Write-Error "VST3 bundle not found at '$Vst3Path'"
    Write-Host "Build the plugin first with: cargo build --release"
    exit 1
}

# Find pluginval.exe in cache
$pluginvalExe = Get-ChildItem -Path $pluginvalDir -Filter "pluginval.exe" -Recurse | Select-Object -First 1

if (-not $pluginvalExe) {
    Write-Host "Downloading pluginval $PluginvalVersion for Windows..." -ForegroundColor Yellow
    $url = "https://github.com/steinbergmedia/vst3sdk/releases/download/$PluginvalVersion/pluginval-win.zip"
    Write-Host "URL: $url"

    New-Item -ItemType Directory -Force -Path $pluginvalDir | Out-Null
    Invoke-WebRequest -Uri $url -OutFile $pluginvalZip
    Expand-Archive -Path $pluginvalZip -DestinationPath $pluginvalDir -Force

    $pluginvalExe = Get-ChildItem -Path $pluginvalDir -Filter "pluginval.exe" -Recurse | Select-Object -First 1
    if (-not $pluginvalExe) {
        Write-Error "pluginval.exe not found after extraction"
        exit 1
    }
    Write-Host "Pluginval installed to: $($pluginvalExe.FullName)" -ForegroundColor Green
} else {
    Write-Host "Using cached pluginval: $($pluginvalExe.FullName)" -ForegroundColor Green
}

# Run pluginval
Write-Host ""
Write-Host "=== Running pluginval (strictness level $StrictnessLevel) ===" -ForegroundColor Cyan
$exitCode = 0
try {
    & $pluginvalExe.FullName --strictness-level $StrictnessLevel $Vst3Path
    $exitCode = $LASTEXITCODE
} catch {
    $exitCode = $_.Exception.HResult
}

Write-Host ""
Write-Host "=== pluginval exited with code: $exitCode ===" -ForegroundColor Cyan
exit $exitCode