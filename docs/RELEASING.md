# Releasing Katna

How a version of Katna is numbered, built and published. The design is
`ARCHITECTURE.md` §21.2 ("Update channels and safe updates"); the work is
the plan's release track (U.1 and U.8).

Tagging a beta, approving a stable release and the repository settings
below are the owner's. Nobody else, people or Claude, pushes a version tag
or approves a promotion.

## Versions

Versions follow [SemVer](https://semver.org). Tags:

| Tag | What it is |
|---|---|
| `vX.Y.Z-beta.N` | Beta `N` of version `X.Y.Z`, cut from `main` |
| `vX.Y.Z` | The release, made only by promoting a beta (below), never pushed by hand |

- Patch releases (`X.Y.Z+1`) are bug fixes only and need no schema
  migration; migrations land in minor or major releases.
- A fix found while a beta soaks makes a new beta (`-beta.N+1`) from `main`,
  which restarts the soak.

Every package reports the same version, worked out from the newest version
tag by `packaging/linux/version.sh` (the PKGBUILD runs it; the Windows
script does the same in PowerShell):

| Commit | Version |
|---|---|
| before the first tag | `0.0.0.rCOUNT.gHASH` (`COUNT` = every commit) |
| `N` commits after `vX.Y.Z-beta.B` | `X.Y.ZbetaB.rN.gHASH` |
| `N` commits after `vX.Y.Z` | `X.Y.Z.rN.gHASH` |

A beta counts only until its release is tagged: on the commit a release was
promoted from, and after it, the release's version wins. Each package
manager sorts a beta before its release: pacman and Katna's own updater
(`katna_core::update::newer`) as written, Debian and RPM with `~beta`.
Katna Mail's About and What's new show the plain form.

Once the first tag exists, nightly builds are numbered from it, so their
`rNNN` counts commits since that tag, not every commit.

## Channels

| Channel | Built from | Published as |
|---|---|---|
| Nightly | every push to `main` | `arch-latest`, `windows-latest`, `linux-latest` (pre-releases, replaced on every push) |
| Beta | a tag `vX.Y.Z-beta.N` | the release `vX.Y.Z-beta.N` and the rolling `beta-latest` |
| Stable | a beta, promoted unchanged | the release `vX.Y.Z` (GitHub's "Latest") and the rolling `stable-latest` |

Stable and beta are the same files: promotion copies a beta's files and
signatures and never rebuilds, so what was tested is what ships.

Each beta and stable release holds every format: the Arch package with its
pacman database (`katna.db`), `KatnaSetup.exe` and `KatnaMail.msix`, the
RPM, `.deb`, Snap, Flatpak, AppImage and tarball, the screenshots of each
running in CI, `SHA256SUMS` (signed as `SHA256SUMS.minisig` once the update
signing key exists, as the Arch package is) and one update manifest per
format: `katna-update-arch.json`, `katna-update-windows.json` and
`katna-update-linux.json`, with the same fields as the nightly
`katna-update.json` files. Nix builds from the repository, so it has no
file here.

What each format uses today, and what §21.2 plans:

| Format | Today | Planned (§21.2) |
|---|---|---|
| Arch | the same `[katna]` repository section, with `Server =` `…/releases/download/beta-latest` or `…/stable-latest` instead of `arch-latest`; the package is still `katna-git` | `[katna]`, `[katna-testing]`, `[katna-nightly]`; package `katna` on the AUR |
| Windows | `KatnaSetup.exe` from the release | channel picked in the app |
| .deb, .rpm | the files from the release | apt and dnf repositories with `stable`, `beta` and `nightly` components |
| AppImage, Flatpak, Snap | the files from the release | per-channel update feeds; Flathub and Flathub beta |

Katna's in-app updates read the manifest of the install's channel
(`update::Channel`, U.10): a nightly build (`….rN.g…`, N above 0) stays
on nightly, and a beta's or release's files (`r0`) take stable, the
default, because both carry the same files. Beta testers pick Beta in
the settings (`updates.channel` in `config.toml`). Moving to a safer
channel never installs an older build: the installed one stays until the
channel passes it.

## Cutting a beta

1. Check `main` is green: CI, Secondary (Ubuntu and Windows) and the
   nightly packages.
2. Tag the commit and push the tag:

   ```sh
   git tag -a v0.1.0-beta.1 -m "Katna 0.1.0 beta 1" <commit>
   git push origin v0.1.0-beta.1
   ```

3. **Release** (`.github/workflows/release.yml`) starts on the tag. It
   - checks the tag is `vX.Y.Z-beta.N`, its commit is on `main`, `vX.Y.Z` is
     not out yet and `version.sh` gives the beta's version;
   - runs CI as on `main` (Arch, cargo-deny, sizes) and the Ubuntu and
     Windows tests;
   - builds every format once from that commit (`arch-package.yml`,
     `windows-package.yml` as a full build, `linux-packages.yml`), each
     installed and tried on its platform as every night;
   - checks every format reports the tag's version, signs, and publishes
     the release `vX.Y.Z-beta.N` (a pre-release) and `beta-latest`.

   If any of it fails nothing is published. Fix it on `main` and tag the
   next beta; a tag is never moved.

## Promoting a beta to stable

1. Let the beta soak (see §21.2) with no serious bug left open.
2. Actions › **Promote to stable** › Run workflow, with the beta's tag
   (for example `v0.1.0-beta.2`).
3. The workflow (`.github/workflows/promote.yml`) checks the beta has a
   release, is the newest beta of its version and that `vX.Y.Z` is not out
   yet, then waits in the `stable` environment.
4. A required reviewer approves it (Actions › the run › Review
   deployments). It then downloads the beta's files, checks them against
   `SHA256SUMS` (and its signature), tags `vX.Y.Z` on the beta's commit and
   publishes the release `vX.Y.Z` and `stable-latest` with the same files.

## Staging or pulling an update

Installs that update themselves can take a new beta or release a few at
a time. Each install draws a number from 0 to 99 once and keeps it in its
state folder (`update-slot`; nothing is sent), and takes the build once
the manifest's `rollout` passes it. A build with `pulled` is offered to
nobody, and a download of it waiting to be installed is dropped. Checking
for updates by hand skips the rollout, never a pull.

Actions › **Stage or pull an update** › Run workflow, with the channel,
the percent (100 offers it to all) and Pulled; it waits in the `stable`
environment for a reviewer, then rewrites only that channel's manifests
(`.github/workflows/rollout.yml`). Package managers cannot stage, so the
beta soak is their safety net. A pulled build is followed by a new beta,
or by promoting a fixed one.

## One-time setup (owner)

- **The `stable` environment:** Settings › Environments › New environment
  `stable`, with Required reviewers (the owner) and, under Deployment
  branches and tags, `main` only. Promotion refuses to run until the
  environment has a required reviewer, since GitHub would otherwise run
  it at once.
- **Tag protection (recommended):** Settings › Rules › Rulesets, a tag
  ruleset for `v*` that blocks updates and deletions, so no version tag
  is ever moved. Leave creation open: promotion creates `vX.Y.Z` itself.
- **Secrets:** the ones the nightly packages use (OAuth client IDs and
  `KATNA_UPDATE_SIGNING_KEY`) apply to releases too; nothing new.
