{
  description = "ISO 13346 Rust Library";

  inputs = {
    nixpkgs.url = "github:nixos/nixpkgs/nixos-unstable";
    utils.url = "github:numtide/flake-utils";
    naersk.url = "github:nix-community/naersk/master";
    rust-overlay.url = "github:oxalica/rust-overlay";
    flake-compat = {
      url = "github:edolstra/flake-compat";
      flake = false;
    };
  };

  outputs = { self, nixpkgs, utils, naersk, rust-overlay, ... }:
    let inherit (nixpkgs) lib;
    in utils.lib.eachDefaultSystem (system:
      let
        pkgs = import nixpkgs {
          inherit system;
          overlays = [ rust-overlay.overlays.default ];
        };
        rust-toolchain = pkgs.rust-bin.stable.latest.default;
      in rec {
        devShells.default = with pkgs;
          mkShell {
            buildInputs = [
              (rust-toolchain.override {
                extensions = [ "rust-src" "rust-analysis" "clippy" "rustfmt" ];
              })
            ];
          };
      });
}
