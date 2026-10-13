# SPDX-License-Identifier: GPL-3.0-or-later
# Builds Katna Mail's Microsoft Store package, target\windows\KatnaMail.msix,
# from the programs ci/windows-package.ps1 just packed into Katna Setup
# (packaging/windows/store/README.md). Run it after that script. The
# package is not signed: the Store signs what it accepts.
$ErrorActionPreference = "Stop"
$root = Resolve-Path (Join-Path $PSScriptRoot "..")
Set-Location $root

# MSIX versions are four numbers under 65536, the last 0 for the Store,
# and each upload's must be higher: the tag's first two numbers, then the
# commit count, which only grows.
$described = git describe --tags --abbrev=0 --match "v[0-9]*" 2>$null
$major, $minor = 0, 0
if ($LASTEXITCODE -eq 0 -and $described -match "^v(\d+)\.(\d+)") {
    $major, $minor = $Matches[1], $Matches[2]
}
$count = git rev-list --count HEAD
$version = "$major.$minor.$count.0"
Write-Host "Store package $version"

$store = Join-Path $root "packaging\windows\store"
$stage = Join-Path $root "target\windows\store"
Remove-Item -Recurse -Force $stage -ErrorAction SilentlyContinue
New-Item -ItemType Directory -Force $stage | Out-Null
# The same programs as Setup's, which katna_core::update::Package tells
# apart by the package's AppxManifest.xml.
Copy-Item "target\windows\payload\*" $stage -Recurse
Copy-Item "$store\Assets" $stage -Recurse
$manifest = (Get-Content -Raw "$store\AppxManifest.xml").Replace("@VERSION@", $version)
Set-Content -NoNewline -Encoding utf8 "$stage\AppxManifest.xml" $manifest
if ($manifest.Contains("PLACEHOLDER")) {
    Write-Warning "AppxManifest.xml still has placeholder identity values; the Store will refuse this package"
}

$makeappx = Get-ChildItem "${env:ProgramFiles(x86)}\Windows Kits\10\bin\*\x64\makeappx.exe" |
    Sort-Object { [version]$_.Directory.Parent.Name } | Select-Object -Last 1
if (-not $makeappx) { throw "makeappx.exe not found: install the Windows SDK" }
$msix = Join-Path $root "target\windows\KatnaMail.msix"
& $makeappx.FullName pack /o /d $stage /p $msix
if ($LASTEXITCODE -ne 0) { exit $LASTEXITCODE }
Get-ChildItem $msix
