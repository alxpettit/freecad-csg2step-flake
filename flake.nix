{
  description = "freecad-convert-shape script for flake.";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-23.05";
    flake-utils.url = "github:numtide/flake-utils";
    nuenv.url = "github:DeterminateSystems/nuenv";
  };

  outputs = { self, nixpkgs, flake-utils, nuenv, ... }:
    flake-utils.lib.eachDefaultSystem (system:
      let
        pkgs = import nixpkgs {
          inherit system;
          overlays = [ nuenv.overlays.nuenv ];  
        };
        freecad-convert-shape = pkgs.nuenv.writeScriptBin {
          name = "freecad-convert-shape.nu";
          script = ''
            def freecad-convert-shape (input: path, output: path) {
              env FREECADPATH=${pkgs.freecad} ${pkgs.freecad}/bin/freecadcmd ${./freecad_convert_shape.py} $input $output
            }
          '';
        };
      in
      {
        devShell = pkgs.mkShell {
          buildInputs = [ freecad-convert-shape ];
        };
        apps = {
          default = {
            type = "app";
            program = "${freecad-convert-shape}/bin/freecad-convert-shape.nu";
          };
        };
        defaultApp = self.apps.${system}.default;
      }
    );
}

