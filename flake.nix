{
  description = "newton-trade-agent";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
    rust-overlay.url = "github:oxalica/rust-overlay";
    flake-utils.url  = "github:numtide/flake-utils";
  };

  outputs = { self, nixpkgs, rust-overlay, flake-utils }:
    flake-utils.lib.eachDefaultSystem (system:
      let
        overlays = [ (import rust-overlay) ];
        pkgs = import nixpkgs {
          inherit system overlays;
        };
      in {
        devShells.default = with pkgs; mkShell {
          buildInputs = [
            (rust-bin.stable."1.88.0".default.override {
              extensions = [ "rust-src" ];
              targets = [ "wasm32-wasip2" ];
            })
            wasmtime
            cargo-component
            just
          ];
          RUST_SRC_PATH="${pkgs.rust-bin.stable."1.88.0".default}/lib/rustlib/src/rust/library";
        };
      }
    );
}