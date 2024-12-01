{
  pkgs ? import <nixpkgs> { },
}:
pkgs.mkShell {
  packages = [
    pkgs.cargo
    pkgs.clippy
    pkgs.rustc
    pkgs.rust-analyzer
    pkgs.rustfmt
  ];
}
