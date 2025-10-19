{
  description = "devshell flake for rust packages";

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
      in
      {
        packages = {
          default = pkgs.rustPlatform.buildRustPackage rec {
            pname = "kittylitters";
            version = "0.2.1";
            name = "${pname}-${version}";
            src = ./.;
            cargoLock = {
              lockFile = ./Cargo.lock;
            };
            nativeBuildInputs = [ rustToolchain ];
          };
        };

        devShells.default =
          with pkgs;
          mkShell {
            buildInputs = [
              bacon
              cargo-audit
              cargo-nextest
              cargo-release
              cargo-semver-checks
              release-plz
              pre-commit
              ra-multiplex
              rustToolchain
            ];
          };
      }
    );
}
