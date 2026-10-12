# SPDX-License-Identifier: GPL-3.0-or-later
# Fedora package for Katna, built from a `git archive` of this repository.
#
#   git archive --prefix=katna/ -o ~/rpmbuild/SOURCES/katna.tar.gz HEAD
#   rpmbuild -bb packaging/fedora/katna.spec \
#     --define "katna_version $(packaging/linux/version.sh)" \
#     --define "katna_built $(git log -1 --format=%ct)"
#
# See packaging/README.md, "Fedora".

%{!?katna_version: %global katna_version 0.0.0}
%{!?katna_built: %global katna_built 0}

Name:           katna
Version:        %{katna_version}
Release:        1%{?dist}
Summary:        Katna Mail and the Katna background sync service
License:        GPL-3.0-or-later
URL:            https://github.com/QuakeString/katna
Source0:        katna.tar.gz
ExclusiveArch:  x86_64 aarch64

# Cargo's release profile already strips the binaries and uses LTO, and
# Fedora's LTO flags turn the C code that crates build (SQLite, ring, zstd)
# into GCC LTO objects that rustc's link step does not handle.
%global debug_package %{nil}
%global _lto_cflags %{nil}
# The build's date comes from its commit (katna_built), not a changelog.
%global source_date_epoch_from_changelog 0

BuildRequires:  cargo >= 1.98
BuildRequires:  rust >= 1.98
BuildRequires:  gcc
BuildRequires:  gettext
BuildRequires:  pkgconfig(fontconfig)
BuildRequires:  pkgconfig(freetype2)
BuildRequires:  pkgconfig(xcb)
BuildRequires:  pkgconfig(xkbcommon)
BuildRequires:  pkgconfig(xkbcommon-x11)

# Linked: libxcb, libxkbcommon(-x11), fontconfig, freetype. Loaded at run
# time: libwayland-client, libvulkan and libEGL (the OpenGL fallback).
Requires:       dbus
Requires:       libwayland-client
Requires:       libglvnd-egl
Requires:       vulkan-loader
Recommends:     mesa-vulkan-drivers
# Keeps account passwords (or KWallet, or KeePassXC).
Recommends:     gnome-keyring
Suggests:       gnupg2
Suggests:       hunspell-en-US
Suggests:       google-noto-sans-cjk-fonts
Suggests:       google-noto-color-emoji-fonts
Suggests:       nautilus-python

%description
Katna Mail reads, searches and writes mail from all your accounts in one
place. katna-daemon syncs them in the background, and katnactl drives it
from a terminal.

%prep
%autosetup -n katna

%build
# Fedora's Rust flags ask for full debug info, which with Cargo's fat LTO
# needs more memory than CI has and is stripped anyway; Cargo's release
# profile has what Katna is built with everywhere.
unset RUSTFLAGS
# Katna Mail names this version in What's new, and the build's date in
# the Update dialog. RPM's `~` (an alpha or beta sorts before its
# release) is not in the version Katna reports (docs/RELEASING.md).
export KATNA_VERSION=$(echo '%{version}' | tr -d '~')
# Katna offers the new build and the dnf command that installs it
# (katna_core::update::Package::Rpm).
export KATNA_PACKAGE=rpm
export KATNA_BUILT=%{katna_built}
# Two builds: in one, Cargo would give katna-daemon and katnactl the
# features GPUI turns on in shared crates, making them bigger for nothing.
cargo build --locked --release --package katna-mail
cargo build --locked --release --package katna-daemon --package katnactl

%install
packaging/linux/stage.sh "${CARGO_TARGET_DIR:-target}/release" %{buildroot} %{_prefix}
# Fedora keeps licenses in %%license, below.
rm -r %{buildroot}%{_datadir}/licenses/katna

%check
%{buildroot}%{_bindir}/katnactl --help > /dev/null
test "$(%{buildroot}%{_bindir}/katna-mail --version)" = "katna-mail $(echo '%{version}' | tr -d '~')"

%files
%license LICENSE
%{_bindir}/katna-mail
%{_bindir}/katna-daemon
%{_bindir}/katnactl
%{_prefix}/lib/systemd/user/*.service
%{_datadir}/dbus-1/services/*.service
%{_datadir}/applications/*.desktop
%{_metainfodir}/*.metainfo.xml
%{_datadir}/krunner/dbusplugins/*.desktop
%{_datadir}/gnome-shell/search-providers/*.ini
%{_datadir}/gnome-shell/extensions/*/
%{_datadir}/kio/servicemenus/*.desktop
%{_datadir}/nautilus-python/extensions/*.py
%{_datadir}/plasma/plasmoids/*/
%{_datadir}/icons/hicolor/*/apps/*
%{_datadir}/locale/*/LC_MESSAGES/katna-clock.mo
