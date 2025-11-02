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
        rustVersion = pkgs.rust-bin.stable.latest.default;
        rustPlatform = pkgs.makeRustPlatform {
          cargo = rustVersion;
          rustc = rustVersion;
        };
        localRustBuild = rustPlatform.buildRustPackage rec {
          pname = "igait-backend";
          version = "0.0.1";
          src = ./.;
          cargoBuildFlags = "-p igait-backend";
          cargoLock = {
            lockFile = ./Cargo.lock;
            outputHashes = {
              "async-openai-0.27.2" = "Y8FZWaceFpF7PcJ70UcKNQy0hY2/mNQMq3d41Qq03dM=";
            };
          };
          nativeBuildInputs = [ (rustVersion.override { extensions = ["rust-src"]; }) ] ++ (with pkgs; [ 
            pkg-config
            cargo
            gcc
            rustfmt
            clippy
            openssl.dev
            openssh
            curl
          ]);
          /*
          libPath = with pkgs; lib.makeLibraryPath [
            udev
            alsa-lib
            vulkan-loader
            libGL
            libxkbcommon
            gtk3-x11
            gtk3
            wayland
            xorg.libX11
            xorg.libXcursor
            xorg.libXi
            xorg.libXrandr
            xorg.libxcb
          ];
          */
          # Certain Rust tools won't work without this
          # This can also be fixed by using oxalica/rust-overlay and specifying the rust-src extension
          # See https://discourse.nixos.org/t/rust-src-not-found-and-other-misadventures-of-developing-rust-on-nixos/11570/3?u=samuela. for more details.
          RUST_SRC_PATH = "${pkgs.rust.packages.stable.rustPlatform.rustLibSrc}";
          PKG_CONFIG_PATH = "${pkgs.openssl.dev}/lib/pkgconfig";
          #LD_LIBRARY_PATH = libPath;
          OPENSSL_LIB_DIR = pkgs.openssl.out + "/lib";
        };
        # Package the runtime assets
        runtimeAssets = pkgs.stdenv.mkDerivation {
          name = "igait-backend-assets";
          src = ./igait-backend;
          installPhase = ''
            mkdir -p $out
            cp -r inputs $out/
            cp -r outputs $out/
            cp -r pdf_handling $out/
          '';
        };
        dockerImage = pkgs.dockerTools.buildLayeredImage {
          name = "igait-backend";
          tag = "latest";
          contents = [ 
            localRustBuild 
            pkgs.cacert 
            pkgs.openssh
            runtimeAssets
            # Add core utilities needed for proper user/environment setup
            pkgs.coreutils
            pkgs.bash
          ];
          fakeRootCommands = ''
            # Create proper /etc/passwd and /etc/group for root user
            mkdir -p etc
            echo "root:x:0:0:root:/root:/bin/bash" > etc/passwd
            echo "root:x:0:" > etc/group
            
            # Create /tmp directory with proper permissions
            mkdir -p tmp
            chmod 1777 tmp
          '';
          config = {
            Cmd = [ "${localRustBuild}/bin/igait-backend" ];
            WorkingDir = "${runtimeAssets}";
            User = "root";
            Env = [
              "HOME=/root"
              "USER=root"
            ];
            ExposedPorts = {
              "3000/tcp" = {};
            };
            Volumes = {
              "/data" = {};
              "/root/.ssh" = {};
            };
          };
        };
      in
      {
        packages = {
          igait-backend = localRustBuild;
          docker = dockerImage;
          default = localRustBuild;
        };
        devShell = pkgs.mkShell {
          buildInputs = with pkgs; [ cargo rustc ];
        };
    });
}