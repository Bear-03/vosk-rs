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

### Compilation

The Vosk-API libraries have to be discoverable by the rust linker. Download the zip file containing the dynamic libraries for your platform [here](https://github.com/alphacep/vosk-api/releases). For iOS development you have to use static libraries. Get the static libraries from the [vosk-api][vosk-api-ios] team.

#### Using dynamic libraries
Do either of the following:

- **Recommended:** Create a [build script][build-script-explanation] and provide cargo with the path to the libraries
- Use the [`RUSTFLAGS` environment variable][rust-env-variables] to provide the path to the variables like so:
    `RUSTFLAGS=-L/path/to/the/libraries`
    with `cargo:rustc-link-search` or `cargo:rustc-link-lib`.
-   Make the vosk library accessible system or user-wide:
    - Windows: Move the libraries to a directory in your `PATH` environment variable.
    - Linux: Move them to `/usr/local/lib`, `/usr/lib` or set the `LIBRARY_PATH` environment variable to the directory containing the libraries.

Although the approaches are equivalent, using a build script is more convenient because it does not require
the developer to remember a terminal command or change anything outside the project scope.

#### Using static libraries (macOS-only, targeting iOS)

- [Extract](https://llvm.org/docs/CommandGuide/llvm-lipo.html) the correct non-fat file (also called thin file) from the static fat file (libvosk.a) for each architecture you would like to support.
- [Mark your crate type as](https://doc.rust-lang.org/cargo/reference/cargo-targets.html#the-crate-type-field) `staticlib`.
- Create a [build script][build-script-explanation] and provide cargo with the path to the libraries with `cargo:rustc-link-search=` and `cargo:rustc-link-lib=static=`.

##### Troubleshooting
In real-world scenarios, one will use Rust to cross compile a library (e.g. Android and iOS). Therefore, we need both `cdylib` as well as the `staticlib` as crate-type. If you compile as usual with cargo build (e.g.: `cargo build --target aarch64-apple-ios --release`) it will not work, because cargo tries to build the dylib as well. Fortunately, since rust 1.64. there is a new option for [rustc](https://github.com/rust-lang/cargo/issues/10083) in the stable channel. Because of this, the following will work: `cargo rustc --crate-type staticlib --lib --target aarch64-apple-ios --release` 

### Execution
Executables compiled with a dynamic lib must have access to the vosk library at runtime. Executables compiled with a statically compiled library do not.

#### Using dynamic libraries
Do either of the following:

-   **Recommended:** Copy the libraries to the root of the executable
    (`target/<cargo profile name>` by default). It is recommended that you use a tool such as
    [cargo-make](https://sagiegurari.github.io/cargo-make/) to automate moving the libraries
    from another, more practical, directory to the destination during build.
-   Make the vosk library accessible system or user-wide:
    - Windows: Move the libraries to a directory in your `PATH` environment variable.
    - Linux: Move them to `/usr/local/lib`, `/usr/lib` or set the `LD_LIBRARY_PATH` environment variable to the directory containing the libraries. Note: `LD_LIBRARY_PATH` is not the same as `LIBRARY_PATH` mentioned in the compilation step.


#### Using static libraries (iOS-only)

- Add the compiled .a library (or libraries if you would like to support more than one architecture) to your iOS project
- Set `Enable Bitcode` to **no** for your target
- Add the `Accelerate Framework` from the iOS SDK to your project
- Depending on your library and use case, you have to write some C -> Objective-C -> Swift glue code.

[build-script-explanation]: https://doc.rust-lang.org/cargo/reference/build-scripts.html
[rust-env-variables]: https://doc.rust-lang.org/cargo/reference/environment-variables.html
[vosk-api-ios]: https://alphacephei.com/vosk/install#ios-build
