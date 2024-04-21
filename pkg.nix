{ pkgs, lib }:
let
  
  freecad-convert-shape-cli = pkgs.writeShellScriptBin "raw-freecad-convert-shape-cli" ''
      FREECADPATH=${pkgs.freecad} ${pkgs.freecad}/bin/freecadcmd ${./freecad-convert-shape-cli.py} "$@"
    '';
  freecad-convert-shape-dynamic = pkgs.writeShellScriptBin "raw-freecad-convert-shape-dynamic" ''
      FREECADPATH=${pkgs.freecad} ${pkgs.freecad}/bin/freecadcmd ${./freecad-convert-shape.py} "$@"
    '';

  # openscad-convert-all = pkgs.writeShellScriptBin "openscad-export" (builtins.readFile
  #     ./openscad-export);

  rustPkg = pkgs.rustPlatform.buildRustPackage rec {
    pname = "openscad-export";
    version = "0.1.0";

    src = lib.cleanSource ./.; 
    buildInputs = [ pkgs.makeWrapper ];
    cargoBuildFlags = [ ];
    cargoLock.lockFile = ./Cargo.lock;
    meta = {
      description = "A wrapper to easily export multiple formats for OpenSCAD. Also takes care of seamlessly exporting STEP";
      homepage = "https://github.com/alxpettit/nixpak-flatpak-wrapper";
      license = pkgs.lib.licenses.lgpl3Only;
    };
  };
in
pkgs.symlinkJoin {
  name = "modeling-helpers";
  meta.mainProgram = "freecad-convert-shape-cli";
  paths = [
    freecad-convert-shape-cli
    freecad-convert-shape-dynamic
    # openscad-convert-all
    rustPkg
  ];
}