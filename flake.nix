{
  description = "Flux flake";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-26.05";  
    rust-overlay.url = "github:oxalica/rust-overlay";
  };

  outputs = { self, nixpkgs, rust-overlay, ... }:
  let
    system = "x86_64-linux";
    overlays = [ (import rust-overlay) ];
    pkgs = import nixpkgs { inherit system overlays; };
  in {
    devShells.${system}.default = pkgs.mkShell {
      buildInputs = [
        # Rust backend
        pkgs.rust-bin.stable.latest.default
        pkgs.gcc
        pkgs.pkg-config
        pkgs.openssl
        
        # Node.js + pnpm for Next.js
        pkgs.nodejs
        pkgs.pnpm
        
        # Development tools
        pkgs.cargo-watch
        pkgs.cargo-edit
        pkgs.rust-analyzer
      ];

      shellHook = ''
        echo "🚀 Flux Development Environment"
        echo "📦 Rust: $(rustc --version)"
        echo "📦 Node: $(node --version)"
        echo "📦 pnpm: $(pnpm --version)"
      '';
    };
  };
}