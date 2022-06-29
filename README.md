# Vosk

[![Latest release on crates.io](https://img.shields.io/crates/v/vosk.svg)](https://crates.io/crates/vosk)
[![Documentation on docs.rs](https://docs.rs/vosk/badge.svg)](https://docs.rs/vosk)
[![GitHub](https://img.shields.io/github/license/Bear-03/vosk-rs)](https://github.com/Bear-03/vosk-rs)

Safe FFI bindings around the [Vosk API Speech Recognition](https://github.com/alphacep/vosk-api) library.

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

The Vosk-API dynamic libraries need to be discoverable by the rust linker. Download the zip for your platform
[here](https://github.com/alphacep/vosk-api/releases) and do **either** of the following:

#### Windows and Linux (Recommended)

-   Use the [`RUSTFLAGS` environment variable][rust-env-variables] to provide the path to the variables like so:
    `RURSTFLAGS=-L/path/to/the/libraries`
-   Create a [build script][build-script-explanation] and provide cargo with the path to the libraries
    with `cargo:rustc-link-search` or `cargo:rustc-link-lib`.

Although both approaches are equivalent, the latter is more practical as it does not
require the developer to remember a terminal command.

#### Windows-only

-   Move the libraries to a directory in your `PATH` environment variable.

#### Linux-only

-   Move them to `/usr/local/lib` or `/usr/lib`.
-   Set the `LIBRARY_PATH` environment variable to the folder where you saved the libraries, lik so

Static libraries are not available.

### Execution
The libraries also have to be discoverable by the executable at runtime.
You will have to follow one of the approaches in the same section you chose in [compilation](#compilation)

#### Windows and Linux (Recommended)
For both approaches, you will need to copy the libraries to the root of the executable
(`target/<cargo profile name>` by default). It is recommended that you use a tool such as 
[cargo-make](https://sagiegurari.github.io/cargo-make/) to automate moving the libraries
from another, more practical, directory to the destination during build.

#### Windows-only
No extra steps are needed as long as the target machine also has the libraries in a directory in its `PATH`.

#### Linux-only

-   **If you followed option 1 in the [compilation](#linux-only) section:** No extra steps are needed as long as the
    target machine also has the libraries in one of the mentioned directories.
-   **If you followed option 2:** You will need to add the directory containing the libraries to the
    `LD_LIBRARY_PATH` environment variable, like so: `LD_LIBRARY_PATH=$LD_LIBRARY_PATH:/path/to/the/libraries`.
    Note that this directory does not have to be the same added to `LIBRARY_PATH` in the compilation step.

[build-script-explanation]: https://doc.rust-lang.org/cargo/reference/build-scripts.html
[rust-env-variables]: https://doc.rust-lang.org/cargo/reference/environment-variables.html
