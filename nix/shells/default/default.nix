{ pkgs, lib, ... }:
let
    # Libs are needed to run the examples
    voskVersion = "0.3.45";
    arch = builtins.elemAt (lib.strings.splitString "-" pkgs.system) 0;
    voskLib = pkgs.fetchzip {
        url = "https://github.com/alphacep/vosk-api/releases/download/v${voskVersion}/vosk-linux-${arch}-${voskVersion}.zip";
        hash = "sha256-ToMDbD5ooFMHU0nNlfpLynF29kkfMknBluKO5PipLFY=";
    };
in
pkgs.mkShell {
    buildInputs = with pkgs; [
        rust-bin.stable.latest.default
        alsa-lib
    ];
    nativeBuildInputs = with pkgs; [ pkg-config ];

    RUSTFLAGS = "-L${voskLib}";
    LD_LIBRARY_PATH = lib.makeLibraryPath [
        pkgs.stdenv.cc.cc
        voskLib
    ];
}