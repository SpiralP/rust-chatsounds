{
  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-25.11";
    flake-utils.url = "github:SpiralP/nix-flake-utils";
  };

  outputs = inputs@{ flake-utils, ... }:
    flake-utils.lib.makeOutputs inputs
      ({ lib, pkgs, makeRustPackage, ... }:
        let
          args = {
            src = ./.;

            buildInputs = with pkgs; [
              alsa-lib
              openssl
            ];

            nativeBuildInputs = with pkgs; [
              pkg-config
            ];
          };
        in
        {
          default = makeRustPackage pkgs (self: args);
        });
}
