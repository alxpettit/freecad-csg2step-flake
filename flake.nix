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
        freecad-convert-shape = import ./pkg.nix {
          inherit pkgs lib;
        };
      in
      {
        devShell = pkgs.mkShell {
          buildInputs = [ freecad-convert-shape ];
        };
        packages = {
          default = freecad-convert-shape;
        };
        apps = {
          default = {
            type = "app";
            program = "${freecad-convert-shape}/bin/freecad-convert-shape-cli";
          };
        };
        defaultApp = self.apps.${system}.default;
      }
    );
}

