# https://github.com/oxalica/rust-overlay
{ pkgs ? import <nixpkgs> { overlays = [ (import <rust-overlay>) ]; } }:
  pkgs.mkShell rec {
    buildInputs = with pkgs; [
      rust-bin.nightly.latest.default
    ];
  }