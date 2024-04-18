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
        demo-utility = pkgs.writeShellScriptBin "demo-utility" ''
          #!/usr/bin/env sh
          echo 'uwu'
        '';
      in
      {
        devShell = pkgs.mkShell {
          buildInputs = [ demo-utility ];
        };
        apps = {
          default = {
            type = "app";
            program = "${demo-utility}/bin/demo-utility";
          };
        };
        defaultApp = self.apps.${system}.default;
      }
    );
}

