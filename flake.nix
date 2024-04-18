{
  description = "A Nix flake with a custom shell script";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-23.05";
    flake-utils.url = "github:numtide/flake-utils";
  };

  outputs = { self, nixpkgs, flake-utils, ... }:
    flake-utils.lib.eachDefaultSystem (system:
      let
        pkgs = import nixpkgs { inherit system; };
        csg2step = pkgs.writeShellScriptBin "csg2step" ''
          #!/usr/bin/env sh
          echo 'uwu'
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

