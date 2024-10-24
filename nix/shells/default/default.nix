{ pkgs, lib, ... }:
let
    # Libs and model files are needed to run the examples for testing purposes
    voskVersion = "0.3.45";
    arch = builtins.elemAt (lib.strings.splitString "-" pkgs.system) 0;
    voskLib = pkgs.fetchzip {
        url = "https://github.com/alphacep/vosk-api/releases/download/v${voskVersion}/vosk-linux-${arch}-${voskVersion}.zip";
        hash = "sha256-ToMDbD5ooFMHU0nNlfpLynF29kkfMknBluKO5PipLFY=";
    };
    model = pkgs.fetchzip {
        url = "https://alphacephei.com/vosk/models/vosk-model-small-en-us-0.15.zip";
        hash = "sha256-CIoPZ/krX+UW2w7c84W3oc1n4zc9BBS/fc8rVYUthuY=";
    };
    speakerModel = pkgs.fetchzip {
        url = "https://alphacephei.com/vosk/models/vosk-model-spk-0.4.zip";
        hash = "sha256-wpTfZnEL1sCfpLhp+l62d8GcOinR15XnSHaLVASH4RA=";
    };
in
pkgs.mkShell {
    buildInputs = with pkgs; [
        (rust-bin.stable.latest.default.override {
            extensions = [ "rust-src" ];
        })
        alsa-lib
    ];
    nativeBuildInputs = with pkgs; [ pkg-config ];

    RUSTFLAGS = "-L${voskLib}";
    LD_LIBRARY_PATH = lib.makeLibraryPath [
        pkgs.stdenv.cc.cc
        voskLib
    ];

    # Run the examples like "cargo run --example <example> $MODEL $SPEAKER_MODEL" etc.
    MODEL = model;
    SPEAKER_MODEL = speakerModel;
}