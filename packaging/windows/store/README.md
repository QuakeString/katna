<!-- SPDX-License-Identifier: GPL-3.0-or-later -->

# Katna Mail in the Microsoft Store

`KatnaMail.msix` is Katna Mail for the Microsoft Store: the same programs
as `KatnaSetup.exe`, packaged as MSIX. The Store signs the packages it
accepts, so Store installs show no SmartScreen warning.
`KatnaSetup.exe` stays for testers and anyone who doesn't use the Store.

- `ci/windows-store-package.ps1` builds it after `ci/windows-package.ps1`,
  from Setup's payload, this folder's `AppxManifest.xml` and `Assets`, with
  `makeappx` from the Windows SDK. The Windows package workflow puts it on
  the `windows-latest` pre-release beside Setup.
- Its version is `major.minor.commits.0`: the newest `v*` tag's first two
  numbers, then the commit count, so each upload is higher than the last.
- `make-logos.py` writes `Assets` from the hicolor PNGs; run it again when
  the icon changes.

## Before the first upload

`AppxManifest.xml` has placeholder identity values. Replace them with the
ones Partner Center shows under Apps and games > Katna Mail > Product
identity, character for character:

| Partner Center | `AppxManifest.xml` |
|---|---|
| Package/Identity/Name | `Identity Name` |
| Package/Identity/Publisher | `Identity Publisher` |
| Package/Properties/PublisherDisplayName | `PublisherDisplayName` |

Until then the build warns, and the Store refuses the package.

The submission's own notes: the package asks for `runFullTrust` (a
desktop program), which certification asks a reason for ("Katna Mail is
a desktop mail client that runs its own background sync service"); and
Katna's AI summaries need the generative-AI disclosure (policy 11.16).

## How the Store package differs from Setup's install

Katna finds the package's `AppxManifest.xml` beside its programs
(`katna_core::update::Package::MsStore`) and then:

- never checks for or installs updates: the Store updates it (policy 10.2);
- starts at sign-in through the package's startup task instead of the
  `Run` key, which Windows keeps to the package (`katna_platform::store`).
  Settings > Apps > Startup shows it. The task cannot pass `--background`,
  so Katna Mail started by it starts only the service, unless the
  "Open the window too" choice left `open-window-at-sign-in` in Katna's
  config folder;
- shows toasts under the package's own app ID.

`mailto:` links come from the manifest's protocol, so Windows Settings >
Default apps lists Katna Mail. Explorer's "Send with Katna Mail" and Send to
entries are registry and shortcut changes Setup makes; a package's
registry writes stay inside it, so the Store package has no such menus
yet (they need a shell extension).
