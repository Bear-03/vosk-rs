# Vosk

[![Latest release](https://img.shields.io/crates/v/vosk.svg)](https://crates.io/crates/vosk)
[![Documentation](https://docs.rs/vosk/badge.svg)](https://docs.rs/vosk)
[![MIT](https://img.shields.io/github/license/Bear-03/vosk-rs)](https://github.com/Bear-03/vosk-rs)
[![Build Status](https://github.com/Bear-03/vosk-rs/workflows/CI/badge.svg)](https://github.com/Bear-03/vosk-rs/actions?workflow=CI)

Safe FFI bindings around the [Vosk API Speech Recognition Toolkit](https://github.com/alphacep/vosk-api).

## Usage
```rust
// Simplified version of examples/read_wav.rs

// Normally you would not want to hardcode the audio samples
let samples = vec![100, -2, 700, 30, 4, 5];
let model_path = "/path/to/model";

let model = Model::new(model_path).unwrap();
let mut recognizer = Recognizer::new(&model, 16000.0).unwrap();

recognizer.set_max_alternatives(10);
recognizer.set_words(true);
recognizer.set_partial_words(true);

for sample in samples.chunks(100) {
    recognizer.accept_waveform(sample);
    println!("{:#?}", recognizer.partial_result());
}

println!("{:#?}", recognizer.final_result().multiple().unwrap());
```

## Setup

### Compilation (dynamic libraries)

The Vosk-API dynamic libraries have to be discoverable by the rust linker (static libraries are not available, except for [iOS](#compilation-dynamic-libraries)).
Download the zip file for your platform [here](https://github.com/alphacep/vosk-api/releases) and:

#### Windows and Linux (Recommended)
Do either of the following:

-   Use the [`RUSTFLAGS` environment variable][rust-env-variables] to provide the path to the variables like so:
    `RUSTFLAGS=-L/path/to/the/libraries`
-   Create a [build script][build-script-explanation] and provide cargo with the path to the libraries
    with `cargo:rustc-link-search` or `cargo:rustc-link-lib`.

Although both approaches are equivalent, the latter is more practical as it does not
require the developer to remember a terminal command.

#### Windows-only

-   Move the libraries to a directory in your `PATH` environment variable.

#### Linux-only
Do either of the following:

-   Move them to `/usr/local/lib` or `/usr/lib`.
-   Set the `LIBRARY_PATH` environment variable to the directory containing the libraries.

### Compilation (static libraries)

For iOS development you have to use static libraries. Get the static libraries from the [vosk-api][vosk-api-ios] team.

#### macOS-only

- [Extract](https://llvm.org/docs/CommandGuide/llvm-lipo.html) the correct non-fat file from the static fat file (libvosk.a) for each architecture you would like to support.
- [Mark your crate type as](https://doc.rust-lang.org/cargo/reference/cargo-targets.html#the-crate-type-field) `staticlib`.
- Create a [build script][build-script-explanation] and provide cargo with the path to the libraries with `cargo:rustc-link-search=` and `cargo:rustc-link-lib=static=`.

##### Troubleshooting
In real-world scenarios, one will use Rust to cross compile a library (e.g. Android and iOS). Therefore, we need both `cdylib` as well as the `staticlib` as crate-type. If you compile as usual with cargo build (e.g.: `cargo build --target aarch64-apple-ios --release`) it will not work, because cargo tries to build the dylib as well. Fortunately, since rust 1.64. there is a new option for [rustc](https://github.com/rust-lang/cargo/issues/10083) in the stable channel. Because of this, the following will work: `cargo rustc --crate-type staticlib --lib --target aarch64-apple-ios --release` 

### Execution (dynamic libraries)
The libraries also have to be discoverable by the executable at runtime.
You will have to follow one of the approaches in the same section you chose in [compilation](<#compilation (dynamic libraries)>).

#### Windows and Linux (Recommended)
For both approaches, you will need to copy the libraries to the root of the executable
(`target/<cargo profile name>` by default). It is recommended that you use a tool such as 
[cargo-make](https://sagiegurari.github.io/cargo-make/) to automate moving the libraries
from another, more practical, directory to the destination during build.

#### Windows-only
If you added your libraries to a directory in your `PATH`, no extra steps are needed as long as that is also the case for the target machine.

#### Linux-only

-   **If you followed option 1 in the [compilation](#linux-only) section:** No extra steps are needed as long as the
    target machine also has the libraries in one of the mentioned directories.
-   **If you followed option 2:** You will need to add the directory containing the libraries to the
    `LD_LIBRARY_PATH` environment variable. Note that this directory does not have to be the same added to
    `LIBRARY_PATH` in the compilation step.

### Execution (static libraries)

#### iOS-only

- Add the compiled .a library (or libraries if you would like to support more than one architecture) to your iOS project
- Set `Enable Bitcode` to **no** for your target
- Add the `Accelerate Framework` from the iOS SDK to your project
- Depending on your library and use case, you have to write some C -> Objective-C -> Swift glue code.

[build-script-explanation]: https://doc.rust-lang.org/cargo/reference/build-scripts.html
[rust-env-variables]: https://doc.rust-lang.org/cargo/reference/environment-variables.html
[vosk-api-ios]: https://alphacephei.com/vosk/install#ios-build
