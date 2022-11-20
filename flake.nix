{
  description = "A devShell example";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
    rust-overlay.url = "github:oxalica/rust-overlay";
    flake-utils.url = "github:numtide/flake-utils";
  };

  outputs = { self, nixpkgs, rust-overlay, flake-utils, ... }:
    flake-utils.lib.eachDefaultSystem (system:
      let
        overlays = [ (import rust-overlay) ];
        pkgs = import nixpkgs {
          inherit system overlays;
        };
      in
      with pkgs;
      {
        devShells.default = mkShell {
          buildInputs = [
            openssl
            pkg-config

            # Rust part
            cargo-expand
            cargo-edit
            rust-analyzer
            (rust-bin.stable.latest.default.override {
              extensions = [ "rust-src" ];
            })
            rust-bin.stable.latest.rustfmt
            rust-bin.stable.latest.clippy
          ];
        };
      }
    );
}

