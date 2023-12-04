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
    inherit (nixpkgs.lib) hasPrefix genAttrs;
    inherit (utils.lib) filterPackages eachSystem;
    days = filter (hasPrefix "day") (attrNames (readDir ./.));
  in
    eachSystem (import systems) (system: let
      pkgs = nixpkgs.legacyPackages.${system};
    in rec {
      packages = genAttrs days (day: pkgs.callPackage ./${day}/default.nix { });
      devShells = genAttrs days (day: pkgs.callPackage ./${day}/shell.nix { });
      hydraJobs = filterPackages system packages;
      formatter = pkgs.alejandra;
    });
}
