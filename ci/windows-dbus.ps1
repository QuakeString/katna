# SPDX-License-Identifier: GPL-3.0-or-later
# Builds the reference dbus-daemon for Windows with vcpkg and copies it, with
# the DLLs it needs, to target\dbus. Katna brings its own session bus on
# Windows (docs/ARCHITECTURE.md §27.1); Setup installs these files beside
# katna-daemon.exe. dbus is AFL-2.1 OR GPL-2.0-or-later, expat MIT.
$ErrorActionPreference = "Stop"
$out = Join-Path $PSScriptRoot "..\target\dbus"
if (Test-Path (Join-Path $out "dbus-daemon.exe")) {
    Write-Host "dbus-daemon.exe already built"
    exit 0
}
$vcpkg = $env:VCPKG_INSTALLATION_ROOT
& "$vcpkg\vcpkg.exe" install dbus:x64-windows-release --clean-after-build
if ($LASTEXITCODE -ne 0) { exit $LASTEXITCODE }
$tools = "$vcpkg\installed\x64-windows-release\tools\dbus"
New-Item -ItemType Directory -Force $out | Out-Null
Copy-Item "$tools\dbus-daemon.exe", "$tools\*.dll" $out
Copy-Item "$vcpkg\installed\x64-windows-release\share\dbus\copyright" "$out\COPYING-dbus.txt"
Get-ChildItem $out
