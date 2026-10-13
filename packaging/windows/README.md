<!-- SPDX-License-Identifier: GPL-3.0-or-later -->

# Katna on Windows

Katna Setup (`apps/katna-setup`) is one `KatnaSetup.exe` that carries
Katna's programs and installs them for the current user, without
administrator rights, on Windows 10 (version 1903 or later) and 11.

- `ci/windows-package.ps1` builds it: Katna Mail, katna-daemon, katnactl,
  the bundled `dbus-daemon.exe` (`ci/windows-dbus.ps1`) and `katna.ico`
  go into Setup's payload.
- `.github/workflows/windows-package.yml` runs that script and, on
  `main`, updates the `windows-latest` pre-release in place with the new Setup.
  Secondary (`secondary.yml`) runs it after each push to `main` once the
  Windows tests pass; it can also be run by hand (Actions > Windows package
  > Run workflow).
  It builds with the `quick` profile (thin LTO) unless **Full build** is
  ticked, which uses `release` as the Arch package does.
- `katna.ico` is made from the hicolor PNGs by `make-ico.py`; run it again
  when the icon changes.
- The same workflow builds `KatnaMail.msix`, the Microsoft Store package
  (`store/README.md`).

Setup installs for the current user into `%LOCALAPPDATA%\Programs\Katna`
(no administrator) or for everyone into `%ProgramFiles%\Katna`, into a
folder the user may change, and adds the shortcuts and start at sign-in
the user chose, the Settings > Apps entry and the `mailto:` handler.
Running a newer Setup updates Katna in place; the Settings > Apps entry
removes it and asks whether to keep mail and passwords.
`docs/ARCHITECTURE.md` §27 says how each Linux piece maps to Windows.
