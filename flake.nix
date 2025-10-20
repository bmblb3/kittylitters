{
  description = "flake for rust development";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
    rust-overlay.url = "github:oxalica/rust-overlay";
    flake-utils.url = "github:numtide/flake-utils";
  };

  outputs =
    {
      self,
      nixpkgs,
      rust-overlay,
      flake-utils,
      ...
    }:
    flake-utils.lib.eachDefaultSystem (
      system:
      let
        overlays = [ (import rust-overlay) ];
        pkgs = import nixpkgs {
          inherit system overlays;
        };
        rustToolchain = pkgs.rust-bin.fromRustupToolchainFile ./rust-toolchain.toml;
        cargoToml = builtins.fromTOML (builtins.readFile ./Cargo.toml);
      in
      {
        packages = {
          default = pkgs.pkgsStatic.rustPlatform.buildRustPackage {
            pname = cargoToml.package.name;
            version = cargoToml.package.version;
            src = pkgs.lib.cleanSource ./.;
            cargoLock = {
              lockFile = ./Cargo.lock;
            };
          };
        };

        devShells.default =
          with pkgs;
          mkShell {
            buildInputs = [
              bacon
              cargo-nextest
              cargo-release
              git-cliff
              just
              pre-commit
              ra-multiplex
              rustToolchain
            ];
          };
      }
    );
}
