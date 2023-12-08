{
  description = "Advent of Code 2023";

  nixConfig = {
    extra-substituters = ["https://cache.m7.rs"];
    extra-trusted-public-keys = ["cache.m7.rs:kszZ/NSwE/TjhOcPPQ16IuUiuRSisdiIwhKZCxguaWg="];
  };

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-23.05";
    systems.url = "github:nix-systems/default-linux";
    utils.url = "github:numtide/flake-utils";
    utils.inputs.systems.follows = "systems";
  };

  outputs = {
    nixpkgs,
    utils,
    systems,
    ...
  }: let
    inherit (builtins) filter attrNames readDir;
    inherit (nixpkgs.lib) hasPrefix genAttrs hasSuffix any importTOML;
    inherit (utils.lib) filterPackages eachSystem;
    listDir = dir: attrNames (readDir dir);
    days = filter (hasPrefix "day") (listDir ./.);
    hasCabal = dir: any (hasSuffix ".cabal") (listDir dir);
    hasCargo = dir: any (n: n == "Cargo.toml") (listDir dir);
  in
    eachSystem (import systems) (system: let
      pkgs = nixpkgs.legacyPackages.${system};
      mkDay = src:
        if (hasCabal src)
        then mkHaskell src
        else if (hasCargo src)
        then mkRust src
        else throw "Couldn't find either cabal or cargo files at ${src}";
      mkHaskell = root: rec {
        package = pkgs.haskellPackages.developPackage {inherit root;};
        devShell = pkgs.mkShell {
          inputsFrom = [package];
          buildInputs = [
            pkgs.aoc-cli
            pkgs.ghc
            pkgs.cabal-install
            pkgs.haskell-language-server
          ];
        };
      };
      mkRust = src: rec {
        package = let
          manifest = importTOML "${src}/Cargo.toml";
        in
          pkgs.rustPlatform.buildRustPackage {
            inherit src;
            pname = manifest.package.name;
            version = manifest.package.version;
            cargoLock.lockFile = "${src}/Cargo.lock";
          };
        shell = pkgs.mkShell {
          inputsFrom = [package];
          buildInputs = [
            pkgs.aoc-cli
            pkgs.rustc
            pkgs.cargo
            pkgs.rust-analyzer
            pkgs.clippy
            pkgs.rustfmt
          ];
        };
      };
    in rec {
      packages = genAttrs days (day: (mkDay ./${day}).package);
      devShells = genAttrs days (day: (mkDay ./${day}).shell);
      hydraJobs = filterPackages system packages;
      formatter = pkgs.alejandra;
    });
}
