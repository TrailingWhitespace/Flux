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
      buildInputs = with pkgs; [
        # Rust backend
     (rust-bin.stable.latest.default.override {
  extensions = [ "rust-src" ]; # for rust-analyzer to work
})
        gcc
        pkg-config
        openssl
        
        # Node.js + pnpm for Next.js
        nodejs
        pnpm
        
        # Development tools
        cargo-watch
        cargo-edit
        rust-analyzer
        systemfd
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