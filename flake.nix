{
  description = "Description for the project";

  inputs = {
   nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";  
  };

  outputs = inputs@{ flake-parts, ... }:
    flake-parts.lib.mkFlake { inherit inputs; } {
      systems = [ "x86_64-linux" "aarch64-linux" "aarch64-darwin" "x86_64-darwin" ];
      perSystem = { config, self', inputs', pkgs, system, ... }: {
        devShells = {
	        default = pkgs.mkShell {
	          buildInputs = [];
	          inputsFrom = [ self'.packages.default ];
	        };
	      };	
	      packages = {
	        default = pkgs.rustPlatform.buildRustPackage {
	          pname = "poly";
	          src = ./.;
	          version = "v2-alpha-stealth-mode";
            buildInputs = [];
            cargoHash =  "sha256-snKu3ya3iWeOlyMH1umwOBHdb9vgIdFg/1A9hCbUl7c=";
	        };
	      };
      };
    };
}
