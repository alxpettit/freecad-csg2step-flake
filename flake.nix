{
  description = "freecad-convert-shape-cli script for flake.";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-23.05";
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
        freecad-convert-shape-cli = ./pkg.nix {
          inherit pkgs lib;
        };
      in
      {
        devShell = pkgs.mkShell {
          buildInputs = [ freecad-convert-shape-cli ];
        };
        apps = {
          default = {
            type = "app";
            program = "${freecad-convert-shape-cli}/bin/freecad-convert-shape-cli";
          };
        };
        defaultApp = self.apps.${system}.default;
      }
    );
}

