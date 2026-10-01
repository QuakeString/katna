# SPDX-License-Identifier: GPL-3.0-or-later
# Katna for Nix and NixOS (packaging/nix/package.nix):
#
#   nix run github:QuakeString/katna
#   nix profile install github:QuakeString/katna
#
# See packaging/README.md, "Nix".
{
  description = "Katna Mail and the Katna background sync service";

  inputs.nixpkgs.url = "https://channels.nixos.org/nixos-unstable/nixexprs.tar.xz";

  outputs =
    { self, nixpkgs }:
    let
      systems = [
        "x86_64-linux"
        "aarch64-linux"
      ];
      forAll = f: nixpkgs.lib.genAttrs systems (system: f nixpkgs.legacyPackages.${system});
    in
    {
      packages = forAll (pkgs: rec {
        katna = pkgs.callPackage ./packaging/nix/package.nix {
          src = self;
          # As the Arch package names it, where Nix knows the commit count.
          version = "0.0.0.r${toString (self.revCount or 0)}.g${self.shortRev or self.dirtyShortRev or "dirty"}";
          built = self.lastModified or 0;
        };
        default = katna;
      });
    };
}
