#![deny(missing_docs)]
//! Safe FFI bindings around the [Vosk API Speech Recognition Toolkit](https://github.com/alphacep/vosk-api).
//!
//! **Basic usage:**
//! * Create a [`Model`]
//! * Create a [`Recognizer`] with that model
//! * Feel audio to the recognizer with [`Recognizer::accept_waveform`]
//! * Get the processed result with [`Recognizer::result`],
//! [`Recognizer::partial_result`] or [`Recognizer::final_result`]

#[cfg(feature = "batch")]
mod gpu;
mod log;
mod models;
mod recognition;

pub use crate::{log::*, models::*, recognition::{*, results::*}};
