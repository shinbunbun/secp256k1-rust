{ pkgs ? import <nixpkgs> {} }:

let
in pkgs.mkShell {
  buildInputs = with pkgs; [ 
    rustup
    gmp
    gnum4
  ];
  shellHook = ''
  rustup component add rustfmt clippy
  '';

#
  # Certain Rust tools won't work without this
  # This can also be fixed by using oxalica/rust-overlay and specifying the rust-src extension
  # See https://discourse.nixos.org/t/rust-src-not-found-and-other-misadventures-of-developing-rust-on-nixos/11570/3?u=samuela. for more details.
  RUST_SRC_PATH = "${pkgs.rust.packages.stable.rustPlatform.rustLibSrc}";
}
