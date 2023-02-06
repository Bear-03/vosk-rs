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

The Vosk-API dynamic libraries have to be discoverable by the rust linker (static libraries are not available, except for [iOS](<#compilation (static libraries)>)).
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
The Rust linker is not able to extract the correct architecture from a static fat file itself. Therefore, for each architecture we need a non-fat file to link with our code.

Example for aarch64:
```shell
lipo -info libvosk.a
# Architectures in the fat file: libvosk.a are: armv7 armv7s arm64 
lipo libvosk.a -thin arm64 -output arm64/libvosk.a
lipo -info arm64/libvosk.a
# Non-fat file: arm64/libvosk.a is architecture: arm64
```
A typical use case is to create a lib using Rust to integrate with an iOS App. So the next step is to add `staticlib` as crate-type in the `Cargo.toml` file.
The static Vosk-API libraries must be discoverable and linked by the Rust linker. One very feasible approach is to create a [build script][build-script-explanation].

Example build script (build.rs file) for cross compiling aarch64 Android and iOS:
```rust
fn add_lib(location: impl AsRef<str>, name: impl AsRef<str>, static_link: bool) {
    println!("cargo:rustc-link-search={}", location.as_ref());
    println!(
        "cargo:rustc-link-lib={}{}",
        if static_link { "static=" } else { "" },
        name.as_ref()
    )
}

fn main() {
    let target =  env::var("TARGET").unwrap();
    match target.as_str() {
        "aarch64-apple-ios" => add_lib("./ios-libs/ios-device/arm64", "vosk", true),
        "aarch64-linux-android" => add_lib("./android-libs/arm64-v8a", "vosk", false),
        _ => {}
    }
}
```
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

#### macOS-only (target iOS)

- Add the compiled .a library (or libraries if you would like to support more than one architecture) to your iOS project
- Set `Enable Bitcode` to **no** for your target
- Add the `Accelerate Framework` from the iOS SDK to your project
- Depending on your library and use case, you have to write some C -> Objective-C -> Swift glue code.

[build-script-explanation]: https://doc.rust-lang.org/cargo/reference/build-scripts.html
[rust-env-variables]: https://doc.rust-lang.org/cargo/reference/environment-variables.html
[vosk-api-ios]: https://alphacephei.com/vosk/install#ios-build
