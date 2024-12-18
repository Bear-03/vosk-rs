use std::os::raw::c_int;

#[cfg(feature = "batch")]
mod batch;
mod errors;
mod results;
mod sequential;

#[cfg(feature = "batch")]
pub use batch::BatchRecognizer;
pub use errors::*;
pub use results::*;
pub use sequential::Recognizer;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
/// State of the decodification after processing a chunk of data.
pub enum DecodingState {
    /// Silence has occured and you can retrieve a new utterance with the [`Recognizer::result`].
    Finalized,
    /// Decoding still continues.
    Running,
    /// Decoding failed in some way.
    Failed,
}

impl DecodingState {
    /// Returns the variant that corresponds to `value` in C.
    pub(self) fn from_c_int(value: c_int) -> Self {
        match value {
            1 => Self::Finalized,
            0 => Self::Running,
            _ => Self::Failed,
        }
    }
}
