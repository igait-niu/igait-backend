{
  # Tremendous thanks to @oati for her help
  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixpkgs-unstable";
    flake-utils.url = "github:numtide/flake-utils";
    rust-overlay.url = "github:oxalica/rust-overlay";
  };
  outputs = { self, nixpkgs, rust-overlay, flake-utils }: 
    flake-utils.lib.eachDefaultSystem (system:
      let
        pkgs = import nixpkgs {
          inherit system;
          overlays = [ rust-overlay.overlays.default ];
        };
      in
      {
        devShell = pkgs.mkShell {
          buildInputs = with pkgs; [ 
            # Rust toolchain
            (rust-bin.stable.latest.default.override {
              extensions = [ "rust-src" ];
            })
            
            # Build dependencies
            pkg-config
            openssl
            openssl.dev
            
            # Runtime tools
            bun 
            nodejs
            
            # Useful for microservices development
            ffmpeg  # Stage 1: media conversion
            python312  # Stage 2, 5, 6: Python processing
          ];
          
          # Set up environment for OpenSSL
          PKG_CONFIG_PATH = "${pkgs.openssl.dev}/lib/pkgconfig";
          OPENSSL_DIR = "${pkgs.openssl.dev}";
          OPENSSL_LIB_DIR = "${pkgs.openssl.out}/lib";
          OPENSSL_INCLUDE_DIR = "${pkgs.openssl.dev}/include";
        };
    });
}