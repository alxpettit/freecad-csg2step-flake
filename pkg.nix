{ pkgs, lib }:
let
  
  freecad-convert-shape-cli = pkgs.writeShellScriptBin "freecad-convert-shape-cli" ''
      FREECADPATH=${pkgs.freecad} ${pkgs.freecad}/bin/freecadcmd ${./freecad-convert-shape-cli.py} "$@"
    '';
  freecad-convert-shape-dynamic = pkgs.writeShellScriptBin "freecad-convert-shape-dynamic" ''
      FREECADPATH=${pkgs.freecad} ${pkgs.freecad}/bin/freecadcmd ${./freecad-convert-shape.py} "$@"
    '';

  openscad-convert-all = pkgs.writeShellScriptBin "openscad-export" (builtins.readFile
      ./openscad-export);

in
pkgs.symlinkJoin {
  name = "modeling-helpers";
  meta.mainProgram = "freecad-convert-shape-cli";
  paths = [
    freecad-convert-shape-cli
    freecad-convert-shape-dynamic
    openscad-convert-all
  ];
}