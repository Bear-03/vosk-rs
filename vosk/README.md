# Vosk

[![Latest release on crates.io](https://img.shields.io/crates/v/vosk.svg)](https://crates.io/crates/vosk)
[![Documentation on docs.rs](https://docs.rs/vosk/badge.svg)](https://docs.rs/vosk)
[![GitHub](https://img.shields.io/github/license/Bear-03/vosk-rs)](https://github.com/Bear-03/vosk-rs)

Raw FFI bindings around the [Vosk API Speech Recognition](https://github.com/alphacep/vosk-api) library.

## Setup and usage

### Compilation

The Vosk-API dynamic libraries need to be discoverable by the rust linker. Download the zip for your platform
[here](https://github.com/alphacep/vosk-api/releases) and do either of the following:

#### On Windows

-   Move them to a folder in your PATH variable.
-   Create a [build script](https://doc.rust-lang.org/cargo/reference/build-scripts.html) and
    provide cargo with the path to the libraries with `cargo:rustc-link-search` or `cargo:rustc-link-lib`

### On Linux

-   Move them to `/usr/local/lib` or `/usr/lib`
-   Set the `LIBRARY_PATH` environment variable to the folder where you saved the libraries, like so

Static libraries are not available.

### Execution

The libraries also have to be discoverable by the executable at runtime. 

#### On Windows

-   **If you followed option 1 in the [compilation](#on-windows) section:** No extra steps are needed as long as
    the target machine also has the libraries in one of those directories
-   **If you followed option 2:** You will need to copy the libraries to the current working directory
    of the executable (`target/<profile name>` by default). It is recommended that you use a tool
    such as [cargo-make](https://sagiegurari.github.io/cargo-make/) to automate moving the libraries
    from another directory to the destination on build.

#### On Linux

-   **If you followed option 2 in the [compilation](#on-linux) section:** No extra steps are needed as long as the
    target machine also has the libraries in one of those directories
-   **If you followed option 2:** You will need to add the directory containing the libraries to the
    `LD_LIBRARY_PATH` environment variables, like so: `LD_LIBRARY_PATH=/path/to/the/libraries:$LD_LIBRARY_PATH`.
    Note that this environment does not have to be the same added to `LIBRARY_PATH` in the compilation step

