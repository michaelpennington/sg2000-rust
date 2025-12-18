{
  description = "Bare metal ESP32-C3 dev environment";

  inputs = {
    nixpkgs.url = "github:nixos/nixpkgs/nixos-unstable";
    flake-utils.url = "github:numtide/flake-utils";
    rust-overlay = {
      url = "github:oxalica/rust-overlay";
      inputs.nixpkgs.follows = "nixpkgs";
    };
  };

  outputs = {
    self,
    nixpkgs,
    flake-utils,
    rust-overlay,
  }:
    flake-utils.lib.eachDefaultSystem (
      system: let
        pkgs = import nixpkgs {
          inherit system;
          overlays = [(import rust-overlay)];
        };
        crossRust = pkgs.rust-bin.fromRustupToolchainFile ./rust-toolchain.toml;
      in {
        devShells.default = pkgs.mkShell {
          nativeBuildInputs = with pkgs; [
            cargo-generate
            svd2rust
            svd2rust-form
            crossRust
            pkgs.pkgsCross.riscv64-embedded.gcc
          ];
          shellHook = ''
            export NPM_CONFIG_PREFIX=$(pwd)/.npm-packages
            export PATH=$NPM_CONFIG_PREFIX/bin:$PATH
            export LIBCLANG_PATH=${pkgs.libclang.lib}/lib/
            export CARGO_HOME=$(pwd)/.cargo
          '';
        };
      }
    );
}
