{pkgs ? import <nixpkgs> {}}: let
  manifest = pkgs.lib.importTOML ./Cargo.toml;
in
  pkgs.rustPlatform.buildRustPackage {
    pname = manifest.package.name;
    version = manifest.package.version;
    src = ./.;
    cargoLock.lockFile = ./Cargo.lock;
  }
