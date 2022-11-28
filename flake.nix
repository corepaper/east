{
  inputs = {
    nixpkgs.url = "github:nixos/nixpkgs/nixos-unstable";
    rustOverlay.url = "github:oxalica/rust-overlay";
    rustOverlay.inputs.nixpkgs.follows = "nixpkgs";
  };

  outputs = { self, nixpkgs, rustOverlay }: let
    pkgs = nixpkgs.legacyPackages."x86_64-linux";
    devInputs = with pkgs; [
      wasm-bindgen-cli binaryen clang pkg-config
      openssl openssl.dev trunk nodePackages.sass
    ];
  in {
    devShell."x86_64-linux" = with pkgs; mkShell {
      buildInputs = devInputs ++ [ rustup ];
    };
  };
}
