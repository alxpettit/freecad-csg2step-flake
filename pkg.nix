{ pkgs, lib }:

let
  paths = [
    (pkgs.writeShellScriptBin "freecad-convert-shape-cli" ''
      FREECADPATH=${pkgs.freecad} ${pkgs.freecad}/bin/freecadcmd ${./freecad-convert-shape-cli.py} "$@"
    '')
  ];

in
  pkgs.symlinkJoin {
    inherit paths;
  }
  # freecad-convert-shape-cli =
  #   pkgs.writeShellScriptBin "freecad-convert-shape-cli" ''
  #     FREECADPATH=${pkgs.freecad} ${pkgs.freecad}/bin/freecadcmd ${./freecad-convert-shape-cli.py} "$@"
  # '';
