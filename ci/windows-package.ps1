# SPDX-License-Identifier: GPL-3.0-or-later
# Builds Katna Setup for Windows: Katna's programs, its session bus and icon
# packed into one KatnaSetup.exe in target\windows (docs/ARCHITECTURE.md
# §27.2). Needs a full git clone for the version, like packaging/arch/PKGBUILD.
$ErrorActionPreference = "Stop"
$root = Resolve-Path (Join-Path $PSScriptRoot "..")
Set-Location $root

# The same version the Arch package reports, so What's new and About match:
# packaging/linux/version.sh in PowerShell (docs/RELEASING.md).
function Describe-Tags([string[]] $options) {
    $described = git describe --long --tags --abbrev=7 @options 2>$null
    if ($LASTEXITCODE -eq 0) { $described } else { "" }
}
function Core([string] $described) { [version](($described -replace "^v", "") -replace "-.*", "") }
$stable = Describe-Tags @("--match", "v[0-9]*", "--exclude", "v*-*")
$pre = Describe-Tags @("--match", "v[0-9]*-alpha.[0-9]*", "--match", "v[0-9]*-beta.[0-9]*")
# An alpha or beta counts only while its release is not tagged yet.
$described = if ($pre -and (-not $stable -or (Core $pre) -gt (Core $stable))) { $pre } else { $stable }
if ($described) {
    $version = ((($described -replace "^v", "") -replace "-alpha\.", "alpha") -replace "-beta\.", "beta") -replace "-(\d+)-g", ".r`$1.g"
} else {
    $version = "0.0.0.r$(git rev-list --count HEAD).g$(git rev-parse --short=7 HEAD)"
}
$env:KATNA_VERSION = $version
Write-Host "Katna $version"
# Builds that update themselves from the windows-latest release
# (katna_core::update::Package::Windows).
$env:KATNA_PACKAGE = "windows"

# KATNA_PROFILE=quick links faster for test builds (Cargo.toml).
$cargoProfile = if ($env:KATNA_PROFILE) { $env:KATNA_PROFILE } else { "release" }
$out = "target\$cargoProfile"

# Two builds, as on Linux: the daemon and katnactl without GPUI's features.
cargo build --locked --profile $cargoProfile -p katna-mail
if ($LASTEXITCODE -ne 0) { exit $LASTEXITCODE }
cargo build --locked --profile $cargoProfile -p katna-daemon -p katnactl
if ($LASTEXITCODE -ne 0) { exit $LASTEXITCODE }
& "$PSScriptRoot\windows-dbus.ps1"
if ($LASTEXITCODE -ne 0) { exit $LASTEXITCODE }

$payload = Join-Path $root "target\windows\payload"
Remove-Item -Recurse -Force $payload -ErrorAction SilentlyContinue
New-Item -ItemType Directory -Force $payload | Out-Null
Copy-Item "$out\katna-mail.exe", "$out\katna-daemon.exe", "$out\katnactl.exe" $payload
Copy-Item target\dbus\* $payload
Copy-Item packaging\windows\katna.ico $payload
Copy-Item LICENSE "$payload\COPYING.txt"

$env:KATNA_SETUP_PAYLOAD = $payload
cargo build --locked --profile $cargoProfile -p katna-setup
if ($LASTEXITCODE -ne 0) { exit $LASTEXITCODE }
Copy-Item "$out\katna-setup.exe" target\windows\KatnaSetup.exe
Get-ChildItem target\windows\KatnaSetup.exe

# For the update manifest the publish job writes (katna-update.json): the
# version, and the newest What's new highlights as Katna Mail lists them.
# A Katna Mail that cannot print them (it is a windowed program) only
# leaves the Update dialog without highlights.
Set-Content -NoNewline -Encoding ascii target\windows\version.txt $version
$highlights = Join-Path $root "target\windows\highlights.json"
$printer = Start-Process -FilePath "$out\katna-mail.exe" -ArgumentList "--highlights" `
    -RedirectStandardOutput $highlights -NoNewWindow -PassThru
# Without this, PowerShell forgets the exit code.
$null = $printer.Handle
if (-not $printer.WaitForExit(60000) -or $printer.ExitCode -ne 0) {
    $printer | Stop-Process -Force -ErrorAction SilentlyContinue
    Write-Warning "katna-mail --highlights did not finish; no highlights"
    Set-Content -Encoding ascii $highlights "[]"
}
