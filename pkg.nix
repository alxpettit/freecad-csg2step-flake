{ pkgs, lib }:
let
  paths = [
    (pkgs.writeShellScriptBin "freecad-convert-shape-cli" ''
      FREECADPATH=${pkgs.freecad} ${pkgs.freecad}/bin/freecadcmd ${./freecad-convert-shape-cli.py} "$@"
    '')
    (pkgs.writeShellScriptBin "freecad-convert-shape" ''
      FREECADPATH=${pkgs.freecad} ${pkgs.freecad}/bin/freecadcmd ${./freecad-convert-shape.py} "$@"
    '')
  ];

in
pkgs.symlinkJoin {
  name = "modeling-helpers";
  meta.mainProgram = "freecad-convert-shape-cli";
  inherit paths;
}