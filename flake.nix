{
  description = "Eternity terminal memory system";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixpkgs-unstable";
    flake-utils.url = "github:numtide/flake-utils";
  };

  outputs = { self, nixpkgs, flake-utils }:
    flake-utils.lib.eachDefaultSystem (system:
      let
        pkgs = import nixpkgs { inherit system; };
      in
      {
        packages.default = pkgs.callPackage ./nix/buildPackage.nix { };
        devShells.default = pkgs.callPackage ./nix/devShell.nix { };
        formatter = pkgs.callPackage ./nix/formatter.nix { };
      });
}
