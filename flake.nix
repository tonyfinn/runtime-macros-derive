{
  description = "Book Club Calendar";

  inputs = {
    nixpkgs.url = "github:nixos/nixpkgs?ref=nixos-unstable";
    rust-overlay = {
      url = "github:oxalica/rust-overlay";
      inputs.nixpkgs.follows = "nixpkgs";
    };
    flake-utils.url = "github:numtide/flake-utils";
  };

  outputs = {
    self,
    nixpkgs,
    rust-overlay,
    flake-utils,
  }:
    flake-utils.lib.eachDefaultSystem (
      system: let
        overlays = [(import rust-overlay)];
        pkgs = import nixpkgs {
          inherit system overlays;
        };
      in {
        devShells.default = pkgs.mkShell {
          buildInputs = [
            (pkgs.rust-bin.stable."1.81.0".default.override {
              extensions = ["rust-src" "rust-analyzer"];
            })
            pkgs.pkg-config
            pkgs.alejandra
            # tarpaulin as CLI tool
            pkgs.cargo-tarpaulin
            # tarpaulin as dep relies on libgit2 which has a bunch of deps
            pkgs.curl
            pkgs.openssl
            pkgs.libgit2
            pkgs.libssh2
            pkgs.openssh
            pkgs.zstd
          ];
        };
        formatter = pkgs.alejandra;
      }
    );
}
