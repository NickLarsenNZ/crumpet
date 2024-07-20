{
  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
    flake-utils.url = "github:numtide/flake-utils";
    rust-overlay = {
      url = "github:oxalica/rust-overlay";
      inputs = {
        nixpkgs.follows = "nixpkgs";
      };
    };
  };
  outputs = { self, nixpkgs, flake-utils, rust-overlay }:
    flake-utils.lib.eachDefaultSystem (system:
      let
        overlays = [ (import rust-overlay) ];
        pkgs = import nixpkgs {
          inherit system overlays;
        };
        rustToolchain = pkgs.pkgsBuildHost.rust-bin.fromRustupToolchainFile ./rust-toolchain.toml; # runtime inputs
        rustToolchainExtensions = (rustToolchain.rust.override {
          extensions = [ "rust-src" ];
        });
        RUST_SRC_PATH = "${rustToolchainExtensions}/lib/rustlib/src/rust/library";
        nativeBuildInputs = with pkgs; [ rustToolchain rustToolchainExtensions ]; # compile time inputs
        buildInputs = with pkgs; [ ];
      in
      with pkgs; {
        # packages.default = derivation {
        #     inherit name src system;
        #     builder = with pkgs; "${bash}/bin/bash";
        #     args = ["-c" "echo foo > $out"];
        # };
        devShells.default = mkShell {
          inherit buildInputs nativeBuildInputs;
        };
      }
    );
}
