{
    inputs = {
        nixpkgs.url = "github:NixOS/nixpkgs/nixpkgs-unstable";
        snowfall-lib = {
            url = "github:snowfallorg/lib";
            inputs.nixpkgs.follows = "nixpkgs";
        };
        rust-overlay = {
            url = "github:oxalica/rust-overlay";
            inputs.nixpkgs.follows = "nixpkgs";
        };
    };

    outputs = { snowfall-lib, rust-overlay, ... } @ inputs:
    snowfall-lib.mkFlake {
        inherit inputs;
        src = ./nix;

        overlays = [
            (import rust-overlay)
        ];
    };
}