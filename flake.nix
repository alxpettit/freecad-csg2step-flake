{
  description = "csg2step script for flake.";

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
        csg2step = pkgs.nuenv.writeScriptBin "csg2step.nu" ''
          #!/usr/bin/env nu
          FREECADPATH=${pkgs.freecad} ${pkgs.freecad}/bin/freecadcmd ${./freecad-csg2step.py} $@
        '';
      in
      {
        devShell = pkgs.mkShell {
          buildInputs = [ csg2step ];
        };
        apps = {
          default = {
            type = "app";
            program = "${csg2step}/bin/csg2step";
          };
        };
        defaultApp = self.apps.${system}.default;
      }
    );
}

