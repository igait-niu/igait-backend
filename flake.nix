{
  # Tremendous thanks to @oati for her help
  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixpkgs-unstable";
    flake-utils.url = "github:numtide/flake-utils";
  };
  outputs = { self, nixpkgs, flake-utils }: 
    flake-utils.lib.eachDefaultSystem (system:
      let
        pkgs = import nixpkgs {
          inherit system;
        };
        
        python-package-list = p: with p; [
          pip
          opencv4
        ];
        python = pkgs.python311.withPackages python-package-list;
      in
      {
        packages = {
          default = (pkgs.rustPlatform.buildRustPackage rec {
            pname = "igait-backend";
            version = "0.1.0";

            src = ./.;
            cargoLock = {
              lockFile = ./Cargo.lock;
            };
          });
        };
        devShell = pkgs.mkShell {
          nativeBuildInputs = with pkgs; [ openssl.dev pkg-config ];
          buildInputs = with pkgs; [ cargo rustc rustfmt rust-analyzer clippy python opencv ];
          shellHook = 
            ''
            '';
          PKG_CONFIG_PATH = "${pkgs.openssl.dev}/lib/pkgconfig";
          RUST_SRC_PATH = pkgs.rustPlatform.rustLibSrc;
          OPENSSL_LIB_DIR = pkgs.openssl.out + "/lib";
        };
    });
}
