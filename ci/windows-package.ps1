# SPDX-License-Identifier: GPL-3.0-or-later
# Builds Katna Setup for Windows: Katna's programs, its session bus and icon
# packed into one KatnaSetup.exe in target\windows (docs/ARCHITECTURE.md
# §27.2). Needs a full git clone for the version, like packaging/arch/PKGBUILD.
$ErrorActionPreference = "Stop"
$root = Resolve-Path (Join-Path $PSScriptRoot "..")
Set-Location $root

# The same version the Arch package reports, so What's new and About match.
$described = git describe --long --tags --abbrev=7 --match "v[0-9]*" 2>$null
if ($LASTEXITCODE -eq 0 -and $described) {
    $version = ($described -replace "^v", "") -replace "-(\d+)-g", ".r`$1.g"
} else {
    $version = "0.0.0.r$(git rev-list --count HEAD).g$(git rev-parse --short=7 HEAD)"
}
$env:KATNA_VERSION = $version
Write-Host "Katna $version"

# Two builds, as on Linux: the daemon and katnactl without GPUI's features.
cargo build --locked --release -p katna-mail
if ($LASTEXITCODE -ne 0) { exit $LASTEXITCODE }
cargo build --locked --release -p katna-daemon -p katnactl
if ($LASTEXITCODE -ne 0) { exit $LASTEXITCODE }
& "$PSScriptRoot\windows-dbus.ps1"
if ($LASTEXITCODE -ne 0) { exit $LASTEXITCODE }

$payload = Join-Path $root "target\windows\payload"
Remove-Item -Recurse -Force $payload -ErrorAction SilentlyContinue
New-Item -ItemType Directory -Force $payload | Out-Null
Copy-Item target\release\katna-mail.exe, target\release\katna-daemon.exe, target\release\katnactl.exe $payload
Copy-Item target\dbus\* $payload
Copy-Item packaging\windows\katna.ico $payload
Copy-Item LICENSE "$payload\COPYING.txt"

$env:KATNA_SETUP_PAYLOAD = $payload
cargo build --locked --release -p katna-setup
if ($LASTEXITCODE -ne 0) { exit $LASTEXITCODE }
Copy-Item target\release\katna-setup.exe target\windows\KatnaSetup.exe
Get-ChildItem target\windows\KatnaSetup.exe
