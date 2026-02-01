{
  description = "Development environment for React + Vite with Bun";
  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixpkgs-unstable";
  };
  outputs = { self, nixpkgs }: 
    let
      system = "x86_64-linux";
      
      pkgs = import nixpkgs { 
        inherit system;
      };
    in
    {
    devShells.x86_64-linux.default = pkgs.mkShell {
      buildInputs = with pkgs; [ bun nodejs ];
      shellHook = 
        ''
          echo "🚀 Bun development environment loaded!"
          echo "Run 'bun install' to install dependencies"
          echo "Run 'bun dev' to start the dev server"
        '';
      
    };
  };
}