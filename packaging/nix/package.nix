# SPDX-License-Identifier: GPL-3.0-or-later
# Nix package for Katna; flake.nix at the repository's top builds it.
# See packaging/README.md, "Nix".
{
  lib,
  rustPlatform,
  pkg-config,
  gettext,
  fontconfig,
  freetype,
  libxkbcommon,
  libxcb,
  libGL,
  vulkan-loader,
  wayland,
  src ? lib.cleanSource ../..,
  version ? "0.0.0",
  built ? 0,
}:

rustPlatform.buildRustPackage {
  pname = "katna";
  inherit src version;

  cargoLock.lockFile = ../../Cargo.lock;

  nativeBuildInputs = [
    pkg-config
    gettext
  ];
  # Linked: libxcb, libxkbcommon(-x11), fontconfig and freetype.
  buildInputs = [
    fontconfig
    freetype
    libxkbcommon
    libxcb
  ];

  # Katna Mail names this version in What's new, and the build's date in
  # the Update dialog. Nix updates it, so it offers no updates itself.
  env = {
    KATNA_VERSION = version;
    KATNA_BUILT = toString built;
  };

  # Two builds: in one, Cargo would give katna-daemon and katnactl the
  # features GPUI turns on in shared crates, making them bigger for nothing.
  buildPhase = ''
    runHook preBuild
    cargo build --frozen --release --package katna-mail
    cargo build --frozen --release --package katna-daemon --package katnactl
    runHook postBuild
  '';

  # The workspace's tests run in CI; they need a session bus and more.
  doCheck = false;

  installPhase = ''
    runHook preInstall
    sh packaging/linux/stage.sh target/release "" "$out"
    runHook postInstall
  '';

  # GPUI loads these at run time: Wayland, Vulkan and EGL (the OpenGL
  # fallback), plus xkbcommon's X11 half.
  postFixup = ''
    patchelf --add-rpath ${
      lib.makeLibraryPath [
        libGL
        libxkbcommon
        vulkan-loader
        wayland
      ]
    } $out/bin/katna-mail
  '';

  meta = {
    description = "Katna Mail and the Katna background sync service";
    homepage = "https://github.com/QuakeString/katna";
    license = lib.licenses.gpl3Plus;
    mainProgram = "katna-mail";
    platforms = [
      "x86_64-linux"
      "aarch64-linux"
    ];
  };
}
