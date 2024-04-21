{
  description = "freecad-convert-shape-cli script for flake.";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixpkgs-unstable";
    flake-utils.url = "github:numtide/flake-utils";
  };

  outputs = { self, nixpkgs, flake-utils, ... }:
    flake-utils.lib.eachDefaultSystem (system:
      let
        lib = pkgs.lib;
        pkgs = import nixpkgs {
          inherit system;
          # overlays = [ nuenv.overlays.nuenv ];  
        };
        pkg = import ./pkg.nix {
          inherit pkgs lib;
        };
      in
      {
        devShell = pkgs.mkShell {
          buildInputs = [ pkg ];
        };
        packages = {
          default = pkg;
        };
        apps = let
          freecad-convert-shape = {
            type = "app";
            program = "${pkg}/bin/freecad-convert-shape";
          };

        in {
          inherit freecad-convert-shape;
          default = freecad-convert-shape;
          openscad-export = {
            type = "app";
            program = "${pkg}/bin/openscad-export";
          };
        };
        defaultApp = self.apps.${system}.default;
      }
    );
}

